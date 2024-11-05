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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: u128,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64) -> (usize, bool, f32) {
mir! {
type RET = (usize, bool, f32);
let _14: f32;
let _15: Adt39;
let _16: (isize, usize, u16);
let _17: f64;
let _18: f64;
let _19: Adt48;
let _20: [u8; 2];
let _21: isize;
let _22: Adt37;
let _23: bool;
let _24: (i8,);
let _25: ();
let _26: ();
{
_4 = (-69_i8) << 54567_u16;
Call(RET.1 = fn1(_4, _4, _4, _4, _4, _4, _4, _4, _4, _4, _4, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = -(-9223372036854775808_isize);
_12 = 3415790211_u32 << _4;
_14 = 17499964770500859982_u64 as f32;
RET = (4_usize, true, _14);
RET = (174884157076069140_usize, false, _14);
_1 = RET.1 | RET.1;
_15.fld6 = !1215331739332581102_i64;
_6 = (-1655695611_i32) * 337343348_i32;
_16 = (_3, RET.0, 25234_u16);
_8 = !168160616582524039266673524915594845447_i128;
_12 = 428025845_u32;
_10 = 6_u8 ^ 103_u8;
_10 = 183_u8 << _6;
_15.fld5.fld3.0 = RET.2 as usize;
_15.fld5.fld0 = _1 & _1;
_18 = _16.2 as f64;
_15.fld5.fld3 = RET;
_5 = 137950137297546878237021986620757057998_u128 * 103975500324463099002867479878463036384_u128;
_15.fld5.fld0 = _16.1 <= _16.1;
_15.fld3 = 10164773697103877158_u64 | 14186808778225592347_u64;
_15.fld5.fld5 = [(-9203_i16),(-14876_i16),(-7291_i16),(-26771_i16),31171_i16];
_13 = _8 as u64;
RET.1 = RET.0 <= RET.0;
_15.fld5.fld3.0 = RET.0 ^ RET.0;
Call(_15.fld5.fld4 = fn4(_1, _16, _15.fld5.fld5, _1, _18, _18, _15.fld5.fld3.2, _10, _16, RET.1, _15.fld5.fld3.1, _15.fld5.fld3, _18), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_15.fld7 = (_4,);
RET.2 = _14 + _14;
_15.fld3 = _13;
_14 = RET.2;
_16 = (_3, _15.fld5.fld3.0, 40845_u16);
_15.fld5.fld5 = [_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4];
_13 = !_15.fld3;
Call(_18 = core::intrinsics::transmute(_13), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_15.fld5.fld1 = '\u{84c79}';
_15.fld0 = core::ptr::addr_of!(_8);
Call(RET = fn8(_4, _16.2, _16, _16, _16.2, _10, _15.fld5.fld1, _16.2, _15.fld3, _18, _4), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_15.fld5.fld5 = [_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4,_15.fld5.fld4];
_15.fld4 = -RET.2;
_15.fld5.fld0 = _16.1 != _16.1;
_6 = (-85036483_i32);
_15.fld5.fld3.1 = _15.fld5.fld3.0 < RET.0;
_15.fld5.fld4 = !(-28546_i16);
_15.fld5.fld3.1 = !RET.1;
_9 = !RET.0;
_14 = RET.2;
_20 = [_10,_10];
_15.fld4 = _10 as f32;
_15.fld5.fld3.2 = _16.2 as f32;
_2 = _15.fld5.fld1;
_22.fld1 = _2;
_15.fld1 = _16.2;
_22.fld2 = _15.fld6;
Goto(bb5)
}
bb5 = {
Call(_25 = dump_var(0_usize, 8_usize, Move(_8), 20_usize, Move(_20), 13_usize, Move(_13), 12_usize, Move(_12)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_25 = dump_var(0_usize, 5_usize, Move(_5), 16_usize, Move(_16), 26_usize, _26, 26_usize, _26), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i8,mut _2: i8,mut _3: i8,mut _4: i8,mut _5: i8,mut _6: i8,mut _7: i8,mut _8: i8,mut _9: i8,mut _10: i8,mut _11: i8,mut _12: i8) -> bool {
mir! {
type RET = bool;
let _13: [u8; 2];
let _14: f64;
let _15: Adt47;
let _16: f64;
let _17: ();
let _18: ();
{
RET = false;
_9 = _6;
_1 = _7;
_5 = _2 * _4;
_8 = 37841_u16 as i8;
_10 = 14544173742990268987_u64 as i8;
RET = !false;
_3 = _6;
_8 = _5 - _4;
_10 = _2 << _8;
_1 = -_3;
_5 = _8;
_4 = _8;
_9 = !_4;
_7 = _5;
Call(_13 = fn2(_4, _4, _5, _2, _7, _4, _1, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = _4;
_13 = [59_u8,169_u8];
_14 = _6 as f64;
_11 = _5 | _9;
Goto(bb2)
}
bb2 = {
_6 = _4;
RET = !false;
_13 = [114_u8,24_u8];
_2 = 5404287518524580114_usize as i8;
_12 = _4;
_9 = _8 >> _12;
_5 = -_10;
_13 = [55_u8,151_u8];
_4 = !_12;
_3 = 119_u8 as i8;
_11 = _1;
_2 = _8;
RET = _10 == _5;
_7 = _8;
_13 = [222_u8,47_u8];
Goto(bb3)
}
bb3 = {
Call(_17 = dump_var(1_usize, 9_usize, Move(_9), 2_usize, Move(_2), 11_usize, Move(_11), 13_usize, Move(_13)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_17 = dump_var(1_usize, 10_usize, Move(_10), 1_usize, Move(_1), 18_usize, _18, 18_usize, _18), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i8,mut _2: i8,mut _3: i8,mut _4: i8,mut _5: i8,mut _6: i8,mut _7: i8,mut _8: i8) -> [u8; 2] {
mir! {
type RET = [u8; 2];
let _9: isize;
let _10: isize;
let _11: *mut f32;
let _12: (f32,);
let _13: isize;
let _14: [u32; 8];
let _15: isize;
let _16: char;
let _17: *mut i16;
let _18: Adt40;
let _19: *mut f32;
let _20: bool;
let _21: (isize, usize, u16);
let _22: Adt51;
let _23: i8;
let _24: *const i128;
let _25: [u128; 7];
let _26: u16;
let _27: Adt36;
let _28: Adt48;
let _29: Adt46;
let _30: isize;
let _31: [i16; 5];
let _32: Adt39;
let _33: [u32; 8];
let _34: isize;
let _35: Adt35;
let _36: ();
let _37: ();
{
_5 = _6 * _6;
RET = [159_u8,223_u8];
_7 = _3;
_2 = _5 | _6;
_1 = 7_usize as i8;
RET = [127_u8,233_u8];
_4 = _7;
_4 = -_6;
_5 = !_2;
RET = [15_u8,208_u8];
RET = [245_u8,115_u8];
_8 = !_2;
_3 = _8;
_10 = -36_isize;
RET = [148_u8,236_u8];
_10 = 943523785_u32 as isize;
_3 = _8 & _4;
_6 = true as i8;
_9 = _10;
_7 = _9 as i8;
_11 = core::ptr::addr_of_mut!(_12.0);
_2 = _8 | _8;
_8 = _5;
_2 = -_3;
_12.0 = 246_u8 as f32;
_12.0 = 3_usize as f32;
(*_11) = 29_u8 as f32;
_3 = _8;
Goto(bb1)
}
bb1 = {
(*_11) = (-38377006478357927245758702155368518898_i128) as f32;
Call(_11 = fn3(_3, _2, _2, _8, _8, _2, _8, _2, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_12.0 = 222_u8 as f32;
_8 = (-121340092395273783014458848603199150655_i128) as i8;
_11 = core::ptr::addr_of_mut!(_12.0);
_1 = (-542211003_i32) as i8;
_7 = (-53714435_i32) as i8;
_9 = _10 << _8;
_13 = 3088133015517491941_u64 as isize;
_12.0 = 6756561389838896574_i64 as f32;
_3 = _5;
RET = [34_u8,225_u8];
_3 = _6;
_1 = _2;
_5 = 0_usize as i8;
_14 = [2170232466_u32,4121129118_u32,38470411_u32,2882273726_u32,224417817_u32,4266572075_u32,2323658993_u32,2180579778_u32];
_5 = 8362_i16 as i8;
_11 = core::ptr::addr_of_mut!((*_11));
_9 = -_13;
_13 = _9;
Goto(bb3)
}
bb3 = {
_11 = core::ptr::addr_of_mut!((*_11));
_7 = _13 as i8;
_4 = _1 + _2;
_9 = -_13;
_16 = '\u{106e4b}';
_12.0 = 27370_i16 as f32;
_5 = _1;
_1 = _13 as i8;
_6 = _4;
_6 = -_4;
_21 = (_9, 6_usize, 50844_u16);
(*_11) = (-3510931687289327334_i64) as f32;
RET = [182_u8,122_u8];
(*_11) = _2 as f32;
_1 = !_6;
_1 = _2 & _6;
_12.0 = (-84310237192737116109039481687249415088_i128) as f32;
_22.fld2.fld3 = [false,true,true,false,false];
_9 = 3124263717_u32 as isize;
_22.fld6 = _21.1;
_22.fld2.fld1.0 = (_22.fld6, true, _12.0);
_21 = (_10, _22.fld2.fld1.0.0, 16331_u16);
Goto(bb4)
}
bb4 = {
_26 = _22.fld2.fld1.0.2 as u16;
RET = [60_u8,145_u8];
_22.fld2.fld0 = 1282096501184761059_u64 as f32;
match _21.2 {
0 => bb1,
1 => bb2,
16331 => bb5,
_ => bb3
}
}
bb5 = {
_22.fld2.fld3 = [_22.fld2.fld1.0.1,_22.fld2.fld1.0.1,_22.fld2.fld1.0.1,_22.fld2.fld1.0.1,_22.fld2.fld1.0.1];
_21.1 = !_22.fld2.fld1.0.0;
_27 = Adt36::Variant1 { fld0: _11,fld1: 1199771420_i32,fld2: _21.0 };
_22.fld5.1 = !_22.fld2.fld1.0.1;
_22.fld2.fld1.0.1 = !_22.fld5.1;
_9 = _10 & _10;
_14 = [1562157264_u32,484611122_u32,2214348661_u32,4113946375_u32,3139712181_u32,3200812609_u32,1964387653_u32,728597917_u32];
place!(Field::<*mut f32>(Variant(_27, 1), 0)) = _11;
match _22.fld6 {
0 => bb3,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb12,
_ => bb11
}
}
bb6 = {
_26 = _22.fld2.fld1.0.2 as u16;
RET = [60_u8,145_u8];
_22.fld2.fld0 = 1282096501184761059_u64 as f32;
match _21.2 {
0 => bb1,
1 => bb2,
16331 => bb5,
_ => bb3
}
}
bb7 = {
_11 = core::ptr::addr_of_mut!((*_11));
_7 = _13 as i8;
_4 = _1 + _2;
_9 = -_13;
_16 = '\u{106e4b}';
_12.0 = 27370_i16 as f32;
_5 = _1;
_1 = _13 as i8;
_6 = _4;
_6 = -_4;
_21 = (_9, 6_usize, 50844_u16);
(*_11) = (-3510931687289327334_i64) as f32;
RET = [182_u8,122_u8];
(*_11) = _2 as f32;
_1 = !_6;
_1 = _2 & _6;
_12.0 = (-84310237192737116109039481687249415088_i128) as f32;
_22.fld2.fld3 = [false,true,true,false,false];
_9 = 3124263717_u32 as isize;
_22.fld6 = _21.1;
_22.fld2.fld1.0 = (_22.fld6, true, _12.0);
_21 = (_10, _22.fld2.fld1.0.0, 16331_u16);
Goto(bb4)
}
bb8 = {
_12.0 = 222_u8 as f32;
_8 = (-121340092395273783014458848603199150655_i128) as i8;
_11 = core::ptr::addr_of_mut!(_12.0);
_1 = (-542211003_i32) as i8;
_7 = (-53714435_i32) as i8;
_9 = _10 << _8;
_13 = 3088133015517491941_u64 as isize;
_12.0 = 6756561389838896574_i64 as f32;
_3 = _5;
RET = [34_u8,225_u8];
_3 = _6;
_1 = _2;
_5 = 0_usize as i8;
_14 = [2170232466_u32,4121129118_u32,38470411_u32,2882273726_u32,224417817_u32,4266572075_u32,2323658993_u32,2180579778_u32];
_5 = 8362_i16 as i8;
_11 = core::ptr::addr_of_mut!((*_11));
_9 = -_13;
_13 = _9;
Goto(bb3)
}
bb9 = {
(*_11) = (-38377006478357927245758702155368518898_i128) as f32;
Call(_11 = fn3(_3, _2, _2, _8, _8, _2, _8, _2, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
place!(Field::<*mut f32>(Variant(_27, 1), 0)) = core::ptr::addr_of_mut!(_12.0);
_23 = -_2;
_1 = _5;
_12 = (_22.fld2.fld1.0.2,);
_22.fld5.2 = -(*_11);
_22.fld2.fld1.0.1 = _22.fld5.1;
Goto(bb13)
}
bb13 = {
_29.fld2.0 = (-6221092580352757395_i64) as isize;
_29.fld1.fld1 = (_22.fld2.fld1.0, 1520544366088270752_u64, 1945404508_i32, _13);
_15 = Field::<isize>(Variant(_27, 1), 2);
_29.fld1.fld0 = (*_11) - (*_11);
_32.fld1 = _21.2 >> _6;
_20 = _3 < _6;
_22.fld5 = (_21.1, _20, _29.fld1.fld1.0.2);
_29.fld2.2 = _32.fld1 * _26;
_27 = Adt36::Variant1 { fld0: _11,fld1: _29.fld1.fld1.2,fld2: _21.0 };
_32.fld5.fld4 = -19592_i16;
_32.fld6 = !7715857466242210172_i64;
_23 = -_4;
_10 = _9 * _15;
_22.fld6 = _22.fld2.fld1.0.0;
_22.fld2.fld1.0.2 = (*_11);
_32.fld7.0 = _21.1 as i8;
Goto(bb14)
}
bb14 = {
_22.fld3 = _4;
_21 = (_9, _22.fld5.0, _32.fld1);
_32.fld5.fld0 = !_20;
_32.fld5.fld3.0 = _22.fld6;
_32.fld5.fld3.2 = _22.fld2.fld0 - (*_11);
place!(Field::<i32>(Variant(_27, 1), 1)) = _29.fld1.fld1.2 - _29.fld1.fld1.2;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(2_usize, 7_usize, Move(_7), 14_usize, Move(_14), 6_usize, Move(_6), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(2_usize, 8_usize, Move(_8), 4_usize, Move(_4), 1_usize, Move(_1), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(2_usize, 3_usize, Move(_3), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: i8,mut _2: i8,mut _3: i8,mut _4: i8,mut _5: i8,mut _6: i8,mut _7: i8,mut _8: i8,mut _9: i8) -> *mut f32 {
mir! {
type RET = *mut f32;
let _10: [i16; 5];
let _11: Adt44;
let _12: u32;
let _13: i8;
let _14: (u8, [u128; 7]);
let _15: f32;
let _16: (isize, usize, u16);
let _17: Adt44;
let _18: ((usize, bool, f32), u64, i32, isize);
let _19: char;
let _20: char;
let _21: isize;
let _22: Adt35;
let _23: [u8; 2];
let _24: Adt44;
let _25: i32;
let _26: Adt40;
let _27: f32;
let _28: [u32; 8];
let _29: char;
let _30: isize;
let _31: [bool; 5];
let _32: Adt41;
let _33: *mut f32;
let _34: Adt35;
let _35: ();
let _36: ();
{
_11.fld1.0.0 = 11753372420694576457_usize >> _8;
RET = core::ptr::addr_of_mut!(_11.fld1.0.2);
_9 = _11.fld1.0.0 as i8;
_4 = -_9;
_2 = _3;
(*RET) = 15349_i16 as f32;
_2 = _3;
_11.fld1.1 = 1739309033943356037_u64;
_11.fld1.3 = -(-115_isize);
_11.fld1.2 = 163502230_u32 as i32;
_8 = _11.fld1.2 as i8;
_10 = [18845_i16,(-13974_i16),31978_i16,(-10491_i16),6421_i16];
_11.fld2 = !_11.fld1.1;
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = (-121297517439922731697320147428305289360_i128) as f32;
match _11.fld1.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
1739309033943356037 => bb8,
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
_14.0 = 253_u8;
_11.fld3 = [false,true,false,true,true];
_11.fld1.0.1 = _4 > _2;
_17.fld1.2 = !_11.fld1.2;
_1 = _6 | _7;
_17.fld3 = [_11.fld1.0.1,_11.fld1.0.1,_11.fld1.0.1,_11.fld1.0.1,_11.fld1.0.1];
_18.0.2 = -_11.fld1.0.2;
_10 = [4005_i16,4012_i16,32765_i16,25007_i16,(-21420_i16)];
_17 = Adt44 { fld0: _11.fld1.0.2,fld1: _11.fld1,fld2: _11.fld1.1,fld3: _11.fld3 };
_16.0 = _17.fld1.3;
_16.0 = _17.fld1.3;
Goto(bb9)
}
bb9 = {
Goto(bb10)
}
bb10 = {
_18.0.0 = 9654_u16 as usize;
_15 = 116476461816911199682384662176283202309_u128 as f32;
_18 = _11.fld1;
_2 = 359263062_u32 as i8;
_11.fld1.1 = _11.fld2;
_24.fld1.3 = _16.0 << _6;
_8 = _9;
_14.1 = [194416302385021827706826812317844537103_u128,88770030666987404133482098697168834753_u128,125549853837747564988494816152632747280_u128,20837224339205584122395615674747817381_u128,299199818695047163098137318105580333282_u128,2783208911949986544182351602809007829_u128,155564817364197437345476528761844224965_u128];
_17.fld3 = [_11.fld1.0.1,_11.fld1.0.1,_17.fld1.0.1,_11.fld1.0.1,_18.0.1];
_16 = (_24.fld1.3, _18.0.0, 300_u16);
_24 = Adt44 { fld0: (*RET),fld1: _18,fld2: _17.fld1.1,fld3: _17.fld3 };
_11.fld1.2 = !_17.fld1.2;
_9 = !_1;
Goto(bb11)
}
bb11 = {
_20 = '\u{bd6e5}';
_18.2 = _17.fld1.2;
_17 = Adt44 { fld0: _15,fld1: _18,fld2: _24.fld2,fld3: _24.fld3 };
_22 = Adt35::Variant0 { fld0: _18.0,fld1: _16.1,fld2: _18.1,fld3: 42212106108103442088853382009109034006_i128 };
_12 = 1498659570_u32;
_2 = _1;
_23 = [_14.0,_14.0];
_15 = _12 as f32;
_10 = [(-15651_i16),(-26179_i16),5839_i16,(-3643_i16),7786_i16];
_7 = _2 >> _6;
_7 = -_5;
_24.fld2 = _17.fld1.1;
_11.fld0 = _18.0.2 * _15;
place!(Field::<u64>(Variant(_22, 0), 2)) = _24.fld2 | _18.1;
_11.fld1 = _24.fld1;
_29 = _20;
place!(Field::<(usize, bool, f32)>(Variant(_22, 0), 0)) = (_24.fld1.0.0, _17.fld1.0.1, _17.fld1.0.2);
_21 = !_16.0;
_30 = _21;
_28 = [_12,_12,_12,_12,_12,_12,_12,_12];
_30 = _21 - _16.0;
_10 = [(-10821_i16),6121_i16,(-10227_i16),(-28706_i16),16598_i16];
_18 = _11.fld1;
match _16.2 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
300 => bb17,
_ => bb16
}
}
bb12 = {
_18.0.0 = 9654_u16 as usize;
_15 = 116476461816911199682384662176283202309_u128 as f32;
_18 = _11.fld1;
_2 = 359263062_u32 as i8;
_11.fld1.1 = _11.fld2;
_24.fld1.3 = _16.0 << _6;
_8 = _9;
_14.1 = [194416302385021827706826812317844537103_u128,88770030666987404133482098697168834753_u128,125549853837747564988494816152632747280_u128,20837224339205584122395615674747817381_u128,299199818695047163098137318105580333282_u128,2783208911949986544182351602809007829_u128,155564817364197437345476528761844224965_u128];
_17.fld3 = [_11.fld1.0.1,_11.fld1.0.1,_17.fld1.0.1,_11.fld1.0.1,_18.0.1];
_16 = (_24.fld1.3, _18.0.0, 300_u16);
_24 = Adt44 { fld0: (*RET),fld1: _18,fld2: _17.fld1.1,fld3: _17.fld3 };
_11.fld1.2 = !_17.fld1.2;
_9 = !_1;
Goto(bb11)
}
bb13 = {
Return()
}
bb14 = {
_14.0 = 253_u8;
_11.fld3 = [false,true,false,true,true];
_11.fld1.0.1 = _4 > _2;
_17.fld1.2 = !_11.fld1.2;
_1 = _6 | _7;
_17.fld3 = [_11.fld1.0.1,_11.fld1.0.1,_11.fld1.0.1,_11.fld1.0.1,_11.fld1.0.1];
_18.0.2 = -_11.fld1.0.2;
_10 = [4005_i16,4012_i16,32765_i16,25007_i16,(-21420_i16)];
_17 = Adt44 { fld0: _11.fld1.0.2,fld1: _11.fld1,fld2: _11.fld1.1,fld3: _11.fld3 };
_16.0 = _17.fld1.3;
_16.0 = _17.fld1.3;
Goto(bb9)
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
(*RET) = _8 as f32;
place!(Field::<(usize, bool, f32)>(Variant(_22, 0), 0)).1 = _11.fld1.0.2 >= (*RET);
_17.fld1.0 = (_11.fld1.0.0, _18.0.1, _11.fld1.0.2);
_16.0 = _30 * _21;
_17.fld1.1 = _30 as u64;
_21 = -_30;
_25 = _24.fld1.2 >> _30;
_30 = _11.fld0 as isize;
_11.fld1.3 = _16.0 * _21;
Goto(bb18)
}
bb18 = {
Call(_35 = dump_var(3_usize, 1_usize, Move(_1), 23_usize, Move(_23), 16_usize, Move(_16), 29_usize, Move(_29)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_35 = dump_var(3_usize, 3_usize, Move(_3), 20_usize, Move(_20), 21_usize, Move(_21), 10_usize, Move(_10)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_35 = dump_var(3_usize, 30_usize, Move(_30), 25_usize, Move(_25), 36_usize, _36, 36_usize, _36), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: bool,mut _2: (isize, usize, u16),mut _3: [i16; 5],mut _4: bool,mut _5: f64,mut _6: f64,mut _7: f32,mut _8: u8,mut _9: (isize, usize, u16),mut _10: bool,mut _11: bool,mut _12: (usize, bool, f32),mut _13: f64) -> i16 {
mir! {
type RET = i16;
let _14: f32;
let _15: Adt38;
let _16: [u8; 2];
let _17: (isize, u128, i128, u16, u16, (usize, bool, f32));
let _18: (usize, bool, f32);
let _19: f32;
let _20: Adt36;
let _21: i128;
let _22: char;
let _23: [u32; 8];
let _24: Adt46;
let _25: i128;
let _26: Adt37;
let _27: isize;
let _28: isize;
let _29: bool;
let _30: (&'static f32, i16, [u128; 7], f64, i16);
let _31: f32;
let _32: char;
let _33: Adt36;
let _34: ((usize, bool, f32), u64, i32, isize);
let _35: Adt45;
let _36: f64;
let _37: Adt37;
let _38: f32;
let _39: Adt47;
let _40: Adt46;
let _41: Adt44;
let _42: (isize, u128, i128, u16, u16, (usize, bool, f32));
let _43: (isize, u128, i128, u16, u16, (usize, bool, f32));
let _44: (&'static f32, i16, [u128; 7], f64, i16);
let _45: ();
let _46: ();
{
RET = -28070_i16;
_5 = -_6;
_3 = [RET,RET,RET,RET,RET];
_1 = _10;
_12.0 = _9.1;
_2.0 = !_9.0;
_2.0 = _9.0 - _9.0;
_9.1 = _2.1 % _12.0;
_2.0 = !_9.0;
_8 = !38_u8;
_5 = _13;
_2 = (_9.0, _9.1, _9.2);
_9 = (_2.0, _12.0, _2.2);
_1 = _12.1;
_6 = -_13;
_7 = _12.2 * _12.2;
_8 = !109_u8;
_2.0 = -_9.0;
_4 = _12.1;
Goto(bb1)
}
bb1 = {
_3 = [RET,RET,RET,RET,RET];
_12.1 = _10 > _10;
_4 = _12.1 >= _11;
_12 = (_2.1, _4, _7);
_9.0 = _2.0 - _2.0;
_16 = [_8,_8];
_17 = (_9.0, 259239507511415307297091163387162322582_u128, 24709444904961614138414695694778249627_i128, _2.2, _9.2, _12);
_11 = _4 | _17.5.1;
_18.2 = _7 + _17.5.2;
_12.0 = _17.5.0;
Call(_12.0 = fn5(_17.1, _17.5.2, _12.1, _4, _17.5, _17.5, _17.2, _17.5, _6, _9.0, _17.2, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_12.2 = -_7;
_11 = _12.0 > _9.1;
_18.2 = _8 as f32;
_2.2 = _17.3 - _17.3;
_4 = !_17.5.1;
_17.5.2 = _7 * _18.2;
_4 = _11;
_12.0 = _9.1 % _9.1;
RET = (-9334_i16) - 20802_i16;
_17.2 = _2.2 as i128;
_12 = _17.5;
_18.2 = _7;
_12.0 = _2.1;
_9 = (_2.0, _2.1, _17.3);
_2 = (_17.0, _17.5.0, _17.4);
_21 = _17.2 | _17.2;
_17 = (_2.0, 184227363585884377554130250313587889666_u128, _21, _2.2, _9.2, _12);
_10 = _17.5.1;
_14 = -_7;
_18.2 = -_17.5.2;
_24.fld1.fld1 = (_12, 13555080801959652482_u64, 1639174451_i32, _17.0);
_10 = !_12.1;
_18.0 = _9.1;
match _17.1 {
0 => bb3,
1 => bb4,
2 => bb5,
184227363585884377554130250313587889666 => bb7,
_ => bb6
}
}
bb3 = {
_3 = [RET,RET,RET,RET,RET];
_12.1 = _10 > _10;
_4 = _12.1 >= _11;
_12 = (_2.1, _4, _7);
_9.0 = _2.0 - _2.0;
_16 = [_8,_8];
_17 = (_9.0, 259239507511415307297091163387162322582_u128, 24709444904961614138414695694778249627_i128, _2.2, _9.2, _12);
_11 = _4 | _17.5.1;
_18.2 = _7 + _17.5.2;
_12.0 = _17.5.0;
Call(_12.0 = fn5(_17.1, _17.5.2, _12.1, _4, _17.5, _17.5, _17.2, _17.5, _6, _9.0, _17.2, _4), ReturnTo(bb2), UnwindUnreachable())
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
_2.0 = -_17.0;
_19 = 3421203400_u32 as f32;
_17.4 = _8 as u16;
_24.fld2 = (_9.0, _18.0, _9.2);
_14 = _24.fld1.fld1.1 as f32;
_18.1 = _11;
_17.4 = !_9.2;
_24.fld0.0 = !_8;
_26.fld0 = _24.fld1.fld1.0.1;
_7 = _12.2;
_17.5.0 = _12.0 ^ _2.1;
_24.fld1.fld2 = !_24.fld1.fld1.1;
_24.fld1.fld0 = _14;
_28 = _13 as isize;
_29 = !_17.5.1;
_4 = _2.2 >= _17.3;
_19 = -_24.fld1.fld0;
_24.fld1.fld1.1 = 460502907_u32 as u64;
_17.2 = _17.1 as i128;
_24.fld2 = _9;
Call(_17.0 = core::intrinsics::bswap(_2.0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_6 = _13;
_24.fld1.fld3 = [_18.1,_17.5.1,_18.1,_18.1,_26.fld0];
_12.0 = _17.1 as usize;
_30.0 = &_14;
_24.fld2.1 = _18.0;
_24.fld1.fld0 = _14 + _14;
_3 = [RET,RET,RET,RET,RET];
_25 = _17.2 | _17.2;
_9.2 = '\u{51d08}' as u16;
_1 = _18.1 & _12.1;
_11 = _1;
_26.fld3.1 = !_29;
_26.fld3.1 = !_1;
_17.0 = _17.5.0 as isize;
_9.1 = _12.0 << _17.1;
_34.0 = (_17.5.0, _26.fld3.1, _19);
_11 = !_12.1;
Goto(bb9)
}
bb9 = {
_26.fld5 = _3;
_34.3 = (-113_i8) as isize;
RET = -(-278_i16);
_26.fld4 = -RET;
_30.1 = _26.fld4 ^ _26.fld4;
_33 = Adt36::Variant0 { fld0: _24.fld1.fld1.2 };
_34.0.2 = _24.fld1.fld0;
_27 = _17.0;
_24.fld1.fld1.3 = _17.0;
match _17.3 {
0 => bb4,
1 => bb3,
25234 => bb11,
_ => bb10
}
}
bb10 = {
_3 = [RET,RET,RET,RET,RET];
_12.1 = _10 > _10;
_4 = _12.1 >= _11;
_12 = (_2.1, _4, _7);
_9.0 = _2.0 - _2.0;
_16 = [_8,_8];
_17 = (_9.0, 259239507511415307297091163387162322582_u128, 24709444904961614138414695694778249627_i128, _2.2, _9.2, _12);
_11 = _4 | _17.5.1;
_18.2 = _7 + _17.5.2;
_12.0 = _17.5.0;
Call(_12.0 = fn5(_17.1, _17.5.2, _12.1, _4, _17.5, _17.5, _17.2, _17.5, _6, _9.0, _17.2, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_19 = _24.fld0.0 as f32;
_26.fld5 = [_30.1,_30.1,_26.fld4,_30.1,_30.1];
_24.fld2 = (_24.fld1.fld1.3, _9.1, _17.3);
_28 = _27;
_26.fld3.0 = _12.0 * _9.1;
_17.2 = 3514974548304631191_i64 as i128;
_37.fld3.1 = _1;
_31 = -_34.0.2;
_26.fld3.2 = _24.fld1.fld0;
Goto(bb12)
}
bb12 = {
_17.3 = !_24.fld2.2;
_24.fld2 = _9;
_9.0 = _24.fld1.fld1.3 ^ _2.0;
_36 = -_6;
_34.2 = -_24.fld1.fld1.2;
_30.2 = [_17.1,_17.1,_17.1,_17.1,_17.1,_17.1,_17.1];
_30.3 = _6;
_12.2 = -_24.fld1.fld1.0.2;
_24.fld1.fld1.0.0 = !_26.fld3.0;
_30.0 = &_12.2;
_37.fld2 = !(-7902714238234863312_i64);
_20 = _33;
_28 = _9.0;
_36 = 66_i8 as f64;
_27 = _1 as isize;
Call(_17.4 = core::intrinsics::bswap(_2.2), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_37.fld3.1 = !_26.fld3.1;
_17.4 = _2.2;
_26.fld0 = _29 | _24.fld1.fld1.0.1;
_24.fld1.fld2 = _37.fld2 as u64;
_37.fld5 = _26.fld5;
_33 = _20;
_21 = -_25;
_22 = '\u{6f9c1}';
_30.2 = [_17.1,_17.1,_17.1,_17.1,_17.1,_17.1,_17.1];
match _17.1 {
0 => bb5,
1 => bb14,
2 => bb15,
3 => bb16,
184227363585884377554130250313587889666 => bb18,
_ => bb17
}
}
bb14 = {
_3 = [RET,RET,RET,RET,RET];
_12.1 = _10 > _10;
_4 = _12.1 >= _11;
_12 = (_2.1, _4, _7);
_9.0 = _2.0 - _2.0;
_16 = [_8,_8];
_17 = (_9.0, 259239507511415307297091163387162322582_u128, 24709444904961614138414695694778249627_i128, _2.2, _9.2, _12);
_11 = _4 | _17.5.1;
_18.2 = _7 + _17.5.2;
_12.0 = _17.5.0;
Call(_12.0 = fn5(_17.1, _17.5.2, _12.1, _4, _17.5, _17.5, _17.2, _17.5, _6, _9.0, _17.2, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
_19 = _24.fld0.0 as f32;
_26.fld5 = [_30.1,_30.1,_26.fld4,_30.1,_30.1];
_24.fld2 = (_24.fld1.fld1.3, _9.1, _17.3);
_28 = _27;
_26.fld3.0 = _12.0 * _9.1;
_17.2 = 3514974548304631191_i64 as i128;
_37.fld3.1 = _1;
_31 = -_34.0.2;
_26.fld3.2 = _24.fld1.fld0;
Goto(bb12)
}
bb16 = {
_2.0 = -_17.0;
_19 = 3421203400_u32 as f32;
_17.4 = _8 as u16;
_24.fld2 = (_9.0, _18.0, _9.2);
_14 = _24.fld1.fld1.1 as f32;
_18.1 = _11;
_17.4 = !_9.2;
_24.fld0.0 = !_8;
_26.fld0 = _24.fld1.fld1.0.1;
_7 = _12.2;
_17.5.0 = _12.0 ^ _2.1;
_24.fld1.fld2 = !_24.fld1.fld1.1;
_24.fld1.fld0 = _14;
_28 = _13 as isize;
_29 = !_17.5.1;
_4 = _2.2 >= _17.3;
_19 = -_24.fld1.fld0;
_24.fld1.fld1.1 = 460502907_u32 as u64;
_17.2 = _17.1 as i128;
_24.fld2 = _9;
Call(_17.0 = core::intrinsics::bswap(_2.0), ReturnTo(bb8), UnwindUnreachable())
}
bb17 = {
_3 = [RET,RET,RET,RET,RET];
_12.1 = _10 > _10;
_4 = _12.1 >= _11;
_12 = (_2.1, _4, _7);
_9.0 = _2.0 - _2.0;
_16 = [_8,_8];
_17 = (_9.0, 259239507511415307297091163387162322582_u128, 24709444904961614138414695694778249627_i128, _2.2, _9.2, _12);
_11 = _4 | _17.5.1;
_18.2 = _7 + _17.5.2;
_12.0 = _17.5.0;
Call(_12.0 = fn5(_17.1, _17.5.2, _12.1, _4, _17.5, _17.5, _17.2, _17.5, _6, _9.0, _17.2, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
SetDiscriminant(_33, 0);
_30.4 = _24.fld1.fld1.1 as i16;
SetDiscriminant(_20, 0);
_24.fld1.fld1.0.0 = !_24.fld2.1;
_32 = _22;
_40.fld0.0 = _30.3 as u8;
_40.fld2.1 = _24.fld2.1;
_40.fld1 = Adt44 { fld0: _31,fld1: _24.fld1.fld1,fld2: _24.fld1.fld1.1,fld3: _24.fld1.fld3 };
_40.fld0.0 = _8 - _8;
_24.fld0.0 = _37.fld2 as u8;
_37.fld3.2 = _14 + _40.fld1.fld0;
_37.fld3 = _40.fld1.fld1.0;
_42 = _17;
_34.1 = !_40.fld1.fld1.1;
_26 = Adt37 { fld0: _11,fld1: _32,fld2: _37.fld2,fld3: _40.fld1.fld1.0,fld4: RET,fld5: _3 };
_40.fld2.0 = _24.fld1.fld1.0.0 as isize;
_34.0 = (_9.1, _4, _7);
_42 = (_40.fld2.0, _17.1, _21, _17.3, _17.3, _26.fld3);
_24.fld0.1 = _30.2;
_39 = Adt47::Variant2 { fld0: _30.1 };
_42.5 = _40.fld1.fld1.0;
_17.0 = _42.0;
Goto(bb19)
}
bb19 = {
Call(_45 = dump_var(4_usize, 8_usize, Move(_8), 27_usize, Move(_27), 1_usize, Move(_1), 2_usize, Move(_2)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_45 = dump_var(4_usize, 11_usize, Move(_11), 9_usize, Move(_9), 32_usize, Move(_32), 28_usize, Move(_28)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: u128,mut _2: f32,mut _3: bool,mut _4: bool,mut _5: (usize, bool, f32),mut _6: (usize, bool, f32),mut _7: i128,mut _8: (usize, bool, f32),mut _9: f64,mut _10: isize,mut _11: i128,mut _12: bool) -> usize {
mir! {
type RET = usize;
let _13: (usize, bool, f32);
let _14: i8;
let _15: f64;
let _16: [u128; 7];
let _17: bool;
let _18: ();
let _19: ();
{
_6.0 = _9 as usize;
_6.1 = _8.2 > _2;
_10 = (-26616_i16) as isize;
_6.2 = -_5.2;
_5 = _8;
_13.2 = (-2_i8) as f32;
_10 = 9223372036854775807_isize;
_6.2 = _8.2;
_10 = (-2136076710_i32) as isize;
_8.1 = !_6.1;
RET = _5.0;
Call(_8.1 = fn6(RET, _11, _4, _1, _6.1, _7, _1, _7, _8.0, _12, _9, _11, _3, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = 662081188_u32 as f64;
_6.2 = (-28576_i16) as f32;
_13 = _8;
_10 = -9223372036854775807_isize;
_6.2 = -_13.2;
_14 = 106_i8;
_10 = (-9223372036854775808_isize);
_6.2 = _2;
_5.0 = !RET;
_2 = -_8.2;
_5.0 = _11 as usize;
_15 = -_9;
_5 = _6;
_6.1 = _13.1;
_16 = [_1,_1,_1,_1,_1,_1,_1];
_4 = _13.1;
_10 = !9223372036854775807_isize;
_1 = !187811918688128721932836757312238823494_u128;
_2 = -_6.2;
_4 = !_3;
_11 = _7 ^ _7;
_5.2 = 11_u8 as f32;
Call(_8.2 = fn7(_6.1, _5.0, _16, _13.1, _8.1, _7, _7, _6.1, _7, _7, _8.1, _13.1, _6, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_14 = (-1706202189_i32) as i8;
_13.0 = _5.0 + RET;
RET = _13.0;
_5 = (_6.0, _8.1, _8.2);
_5 = (_13.0, _4, _8.2);
_5.0 = !RET;
_8 = (_13.0, _4, _5.2);
_14 = (-11_i8);
_8.1 = _3;
_5.2 = _10 as f32;
_6 = _13;
_3 = _5.1 ^ _13.1;
_8.2 = _5.2;
_8.0 = !RET;
_5 = (RET, _6.1, _2);
_17 = !_3;
_5.0 = !RET;
_8.0 = 54670_u16 as usize;
_5.2 = _2;
_5.2 = _13.0 as f32;
_4 = _17;
_9 = 137_u8 as f64;
RET = _5.0 & _5.0;
RET = _13.0;
RET = _6.0;
_2 = _5.2 * _13.2;
_8.1 = _12 == _17;
_8.0 = _13.0;
Goto(bb3)
}
bb3 = {
Call(_18 = dump_var(5_usize, 3_usize, Move(_3), 16_usize, Move(_16), 14_usize, Move(_14), 17_usize, Move(_17)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_18 = dump_var(5_usize, 10_usize, Move(_10), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: usize,mut _2: i128,mut _3: bool,mut _4: u128,mut _5: bool,mut _6: i128,mut _7: u128,mut _8: i128,mut _9: usize,mut _10: bool,mut _11: f64,mut _12: i128,mut _13: bool,mut _14: i128) -> bool {
mir! {
type RET = bool;
let _15: (u8, [u128; 7]);
let _16: ();
let _17: ();
{
_15.1 = [_7,_4,_4,_4,_7,_4,_7];
_5 = _10;
RET = _5 | _13;
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(6_usize, 8_usize, Move(_8), 2_usize, Move(_2), 6_usize, Move(_6), 3_usize, Move(_3)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_16 = dump_var(6_usize, 7_usize, Move(_7), 10_usize, Move(_10), 17_usize, _17, 17_usize, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: bool,mut _2: usize,mut _3: [u128; 7],mut _4: bool,mut _5: bool,mut _6: i128,mut _7: i128,mut _8: bool,mut _9: i128,mut _10: i128,mut _11: bool,mut _12: bool,mut _13: (usize, bool, f32),mut _14: (usize, bool, f32)) -> f32 {
mir! {
type RET = f32;
let _15: Adt40;
let _16: Adt50;
let _17: isize;
let _18: Adt44;
let _19: u16;
let _20: Adt40;
let _21: [u128; 7];
let _22: i32;
let _23: Adt49;
let _24: f64;
let _25: *const i128;
let _26: f32;
let _27: i32;
let _28: f32;
let _29: isize;
let _30: [i16; 5];
let _31: Adt47;
let _32: isize;
let _33: isize;
let _34: ();
let _35: ();
{
_13.1 = _14.1 != _5;
_10 = -_9;
RET = _13.0 as f32;
RET = _14.2 - _13.2;
_1 = _13.1 == _13.1;
_3 = [95596507260420971335089189248293491532_u128,151674006436384363882509165701884173567_u128,306677709288329202834051584394489549139_u128,190338920320730367837101310229388395091_u128,304397786028546368446906122205913283674_u128,243033753221054858628156639845663832826_u128,16122694282923688650899800731339368420_u128];
_14.2 = _13.2;
_6 = 3116095708_u32 as i128;
_10 = _4 as i128;
_12 = _13.1;
_6 = 17021763571114349248_u64 as i128;
Goto(bb1)
}
bb1 = {
_18.fld1.3 = 22_isize << _9;
_18.fld1.0.1 = _11;
_18.fld1.0.0 = _18.fld1.3 as usize;
_7 = 3298_i16 as i128;
_18.fld1.0.2 = _9 as f32;
_18.fld3 = [_8,_1,_14.1,_1,_12];
_1 = _8 | _12;
_18.fld0 = _18.fld1.0.2;
_1 = !_18.fld1.0.1;
_13 = (_18.fld1.0.0, _12, _18.fld0);
_13.1 = _18.fld1.0.2 < _18.fld1.0.2;
_3 = [167385818217859800516454226826278542149_u128,13033424717788462305581957036933023721_u128,269323953731572333700571193671270067325_u128,5157200663619647833841746047003307238_u128,133189356403937041209734735076770431839_u128,32943539760334819262428636306929387046_u128,107811858725284720543574283214741821488_u128];
_18.fld2 = 2101442489_u32 as u64;
_8 = !_5;
_18.fld2 = 3856750870814831496_u64;
_17 = !_18.fld1.3;
_18.fld1.1 = _18.fld2 % _18.fld2;
_10 = _9;
_12 = _18.fld1.0.1 & _1;
_22 = !901100633_i32;
_18.fld1.2 = _22 ^ _22;
_13.0 = _18.fld1.0.0;
_13 = _14;
Goto(bb2)
}
bb2 = {
_18.fld1.2 = _22;
_11 = _8 ^ _4;
_4 = !_11;
_18.fld1.0.0 = !_2;
_18.fld3 = [_12,_4,_18.fld1.0.1,_5,_4];
_18.fld1.0.0 = _18.fld1.0.2 as usize;
_18.fld2 = _18.fld1.1 >> _18.fld1.3;
_4 = !_13.1;
_18.fld0 = _18.fld2 as f32;
_25 = core::ptr::addr_of!(_7);
_26 = _9 as f32;
_18.fld1.0 = (_13.0, _11, _26);
_14 = (_13.0, _11, _18.fld0);
_13 = _18.fld1.0;
_1 = _18.fld1.0.1;
_25 = core::ptr::addr_of!(_9);
_27 = !_18.fld1.2;
_18.fld0 = -_18.fld1.0.2;
_13 = (_18.fld1.0.0, _11, _26);
_13.1 = !_12;
_3 = [269552201003564315044298678333223038725_u128,70600254418190071626264969276062445355_u128,278493904622691446156162331746961197155_u128,113576105485031322412351936530136955976_u128,110674117774874563215009889969140051304_u128,39842087072124111084946115928249511972_u128,193666604978253744233425833001471164869_u128];
_18.fld1 = (_14, _18.fld2, _27, _17);
_18.fld3 = [_12,_11,_4,_13.1,_8];
_19 = !13568_u16;
_1 = !_18.fld1.0.1;
_27 = _18.fld1.2;
_24 = 78_i8 as f64;
_12 = _13.0 != _13.0;
match (*_25) {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
24709444904961614138414695694778249627 => bb8,
_ => bb7
}
}
bb3 = {
_18.fld1.3 = 22_isize << _9;
_18.fld1.0.1 = _11;
_18.fld1.0.0 = _18.fld1.3 as usize;
_7 = 3298_i16 as i128;
_18.fld1.0.2 = _9 as f32;
_18.fld3 = [_8,_1,_14.1,_1,_12];
_1 = _8 | _12;
_18.fld0 = _18.fld1.0.2;
_1 = !_18.fld1.0.1;
_13 = (_18.fld1.0.0, _12, _18.fld0);
_13.1 = _18.fld1.0.2 < _18.fld1.0.2;
_3 = [167385818217859800516454226826278542149_u128,13033424717788462305581957036933023721_u128,269323953731572333700571193671270067325_u128,5157200663619647833841746047003307238_u128,133189356403937041209734735076770431839_u128,32943539760334819262428636306929387046_u128,107811858725284720543574283214741821488_u128];
_18.fld2 = 2101442489_u32 as u64;
_8 = !_5;
_18.fld2 = 3856750870814831496_u64;
_17 = !_18.fld1.3;
_18.fld1.1 = _18.fld2 % _18.fld2;
_10 = _9;
_12 = _18.fld1.0.1 & _1;
_22 = !901100633_i32;
_18.fld1.2 = _22 ^ _22;
_13.0 = _18.fld1.0.0;
_13 = _14;
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
_18.fld1.0.1 = !_14.1;
_11 = !_5;
_12 = !_1;
RET = 5035190085628676084_i64 as f32;
_18.fld0 = _13.2 - _26;
_21 = _3;
_22 = _18.fld1.2;
RET = -_18.fld0;
_25 = core::ptr::addr_of!((*_25));
_22 = 3681168846_u32 as i32;
_13.2 = -RET;
_18.fld2 = _18.fld1.1 * _18.fld1.1;
_18.fld1.0 = (_14.0, _14.1, RET);
_11 = _5;
_24 = (-119_i8) as f64;
_18.fld2 = !_18.fld1.1;
_17 = -_18.fld1.3;
_13 = (_14.0, _11, _26);
_10 = (*_25);
_26 = _18.fld0 * _13.2;
_30 = [(-10768_i16),(-11970_i16),14973_i16,30652_i16,2203_i16];
Goto(bb9)
}
bb9 = {
Call(_34 = dump_var(7_usize, 1_usize, Move(_1), 30_usize, Move(_30), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_34 = dump_var(7_usize, 3_usize, Move(_3), 9_usize, Move(_9), 27_usize, Move(_27), 4_usize, Move(_4)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_34 = dump_var(7_usize, 6_usize, Move(_6), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}

#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: i8,mut _2: u16,mut _3: (isize, usize, u16),mut _4: (isize, usize, u16),mut _5: u16,mut _6: u8,mut _7: char,mut _8: u16,mut _9: u64,mut _10: f64,mut _11: i8) -> (usize, bool, f32) {
mir! {
type RET = (usize, bool, f32);
let _12: Adt39;
let _13: bool;
let _14: Adt47;
let _15: isize;
let _16: Adt45;
let _17: i128;
let _18: Adt44;
let _19: Adt46;
let _20: ();
let _21: ();
{
_4 = (_3.0, _3.1, _2);
_12.fld3 = _9 * _9;
_4.0 = _3.0;
_12.fld6 = !(-1137747338163569460_i64);
_12.fld5.fld5 = [22097_i16,(-22190_i16),2382_i16,(-22742_i16),11564_i16];
RET.0 = _3.1 ^ _3.1;
_12.fld5.fld4 = 18593_i16 >> _8;
_12.fld1 = _3.2;
Call(_12.fld6 = fn9(_3, _6, _12.fld5.fld4, _12.fld1, _6, _3, _4, _4, _12.fld1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12.fld5.fld2 = !_12.fld6;
_12.fld5.fld3.2 = _12.fld5.fld2 as f32;
Call(RET = fn10(_12.fld1, _8, _3.1, _7, _8, _4.2, _3.0, _5, _4.2, _3.2, _5, _4, _5, _3, _2, _3.2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = (_4.0, _4.1, _12.fld1);
RET.0 = 2097223310_u32 as usize;
_8 = _12.fld1 >> _4.2;
_3 = _4;
_12.fld5.fld1 = _7;
_12.fld7 = (_1,);
_17 = -88323737885260227797895620887910301794_i128;
_12.fld2 = _10;
_12.fld5.fld4 = -(-16029_i16);
_13 = RET.1 ^ RET.1;
RET = (_3.1, _13, _12.fld5.fld3.2);
_18.fld1.0.1 = _13;
_17 = (-44935317310053034386216315986674238345_i128) >> _3.2;
RET.0 = !_3.1;
RET = (_4.1, _18.fld1.0.1, _12.fld5.fld3.2);
_18.fld1.0.0 = _12.fld5.fld4 as usize;
_14 = Adt47::Variant2 { fld0: _12.fld5.fld4 };
_4.0 = _3.0 ^ _3.0;
_12.fld1 = _2;
_12.fld1 = !_5;
_12.fld5.fld3 = RET;
_18.fld1.2 = 951331591_i32;
_19.fld2.1 = _4.1;
SetDiscriminant(_14, 0);
RET.0 = _19.fld2.1 - _12.fld5.fld3.0;
_5 = _8 / _2;
place!(Field::<(isize, u128, i128, u16, u16, (usize, bool, f32))>(Variant(_14, 0), 1)) = (_4.0, 255391426879096882075748640666893526376_u128, _17, _8, _2, _12.fld5.fld3);
Goto(bb3)
}
bb3 = {
Call(_20 = dump_var(8_usize, 17_usize, Move(_17), 7_usize, Move(_7), 2_usize, Move(_2), 9_usize, Move(_9)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_20 = dump_var(8_usize, 8_usize, Move(_8), 6_usize, Move(_6), 21_usize, _21, 21_usize, _21), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: (isize, usize, u16),mut _2: u8,mut _3: i16,mut _4: u16,mut _5: u8,mut _6: (isize, usize, u16),mut _7: (isize, usize, u16),mut _8: (isize, usize, u16),mut _9: u16) -> i64 {
mir! {
type RET = i64;
let _10: bool;
let _11: i32;
let _12: ((usize, bool, f32), u64, i32, isize);
let _13: f32;
let _14: bool;
let _15: i64;
let _16: Adt44;
let _17: bool;
let _18: Adt35;
let _19: i16;
let _20: [u8; 2];
let _21: char;
let _22: Adt48;
let _23: f32;
let _24: ();
let _25: ();
{
_1.2 = _4;
_6.2 = _4;
_1.2 = 318049448128533525180159028672857223241_u128 as u16;
_1.1 = _6.1 >> _7.2;
_12.3 = _1.0 & _1.0;
_12.0.1 = _5 < _5;
_9 = _4 + _8.2;
_8.1 = (-50336679619387738554824418770396107640_i128) as usize;
_5 = _2 | _2;
_8.2 = !_9;
_8 = (_12.3, _1.1, _9);
_7 = _1;
_11 = !(-1606385554_i32);
_6.1 = !_8.1;
_12.0.2 = _5 as f32;
_7.0 = _8.0 & _1.0;
_12.2 = 7344633756276758560_u64 as i32;
_4 = _8.2 << _1.1;
_6.0 = _7.0;
_7.2 = _9;
RET = !341853909730954150_i64;
_6.1 = _12.0.1 as usize;
_14 = _12.2 <= _12.2;
_1.0 = (-39799412373719966586471903502723701777_i128) as isize;
match _6.2 {
0 => bb1,
40845 => bb3,
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
_7 = (_6.0, _1.1, _4);
_8.1 = _6.1 | _7.1;
_1.2 = _12.2 as u16;
_16.fld1.0.2 = RET as f32;
_12.2 = _1.1 as i32;
_17 = _8.2 <= _4;
match _6.2 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
40845 => bb9,
_ => bb8
}
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
_13 = _12.0.2 * _16.fld1.0.2;
_14 = !_17;
_12.0 = (_7.1, _17, _13);
_16.fld1.0.1 = _8.2 <= _6.2;
_8.2 = _4;
_12.1 = _9 as u64;
_16.fld0 = -_12.0.2;
_16.fld1.3 = _12.3 + _6.0;
_16.fld1.1 = _12.1;
_6.1 = RET as usize;
_15 = (-13035470760663531819988616100588335356_i128) as i64;
_6.0 = -_12.3;
match _6.2 {
0 => bb8,
1 => bb7,
2 => bb3,
3 => bb10,
4 => bb11,
5 => bb12,
40845 => bb14,
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
_8.1 = _1.1;
_2 = 56940104410278319059961478763689990868_i128 as u8;
_16.fld1 = _12;
_3 = 1650_i16;
_6.0 = !_16.fld1.3;
_1 = (_7.0, _7.1, _4);
_12.0.1 = _1.1 > _12.0.0;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(9_usize, 1_usize, Move(_1), 9_usize, Move(_9), 17_usize, Move(_17), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(9_usize, 11_usize, Move(_11), 7_usize, Move(_7), 25_usize, _25, 25_usize, _25), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: u16,mut _2: u16,mut _3: usize,mut _4: char,mut _5: u16,mut _6: u16,mut _7: isize,mut _8: u16,mut _9: u16,mut _10: u16,mut _11: u16,mut _12: (isize, usize, u16),mut _13: u16,mut _14: (isize, usize, u16),mut _15: u16,mut _16: u16) -> (usize, bool, f32) {
mir! {
type RET = (usize, bool, f32);
let _17: Adt43;
let _18: Adt41;
let _19: ((usize, bool, f32), u64, i32, isize);
let _20: isize;
let _21: Adt44;
let _22: Adt50;
let _23: isize;
let _24: [i16; 5];
let _25: usize;
let _26: ();
let _27: ();
{
_14.2 = _5;
_15 = _16 % _14.2;
RET.2 = 84_u8 as f32;
RET.0 = !_12.1;
match _5 {
0 => bb1,
1 => bb2,
2 => bb3,
40845 => bb5,
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
_12.0 = -_7;
_14.1 = 16792248854246703600_u64 as usize;
RET.1 = false;
_3 = _14.1 ^ _12.1;
_7 = !_14.0;
_9 = _15 ^ _2;
_12.2 = _9 + _5;
_2 = _8;
_5 = _12.2 * _6;
_14.2 = (-4929929072674797137_i64) as u16;
RET.1 = !true;
RET.1 = !true;
RET.1 = false;
_19 = (RET, 134194391241849159_u64, 1553381090_i32, _14.0);
_19.3 = _12.0 * _12.0;
_19.1 = 5405892880763052668_u64 << _5;
_6 = _15;
Call(_19.0 = fn11(_12, _5, _6, _19.1, _15, _6, _19.1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_14 = (_19.3, _12.1, _9);
RET.1 = _19.0.1 | _19.0.1;
RET.0 = _3 >> _19.1;
_19.1 = 3538048333379130098_u64;
_19.0.2 = RET.0 as f32;
_5 = _14.2;
_19 = (RET, 17706676904197732184_u64, (-992506701_i32), _7);
_10 = _6;
_21.fld1.0.0 = !_14.1;
_21.fld1.2 = _19.2;
RET.2 = _19.0.2;
_21.fld3 = [RET.1,RET.1,RET.1,RET.1,_19.0.1];
_10 = _15;
_19.1 = !7528082741990378261_u64;
_21.fld1.0 = (RET.0, _19.0.1, _19.0.2);
RET.2 = _21.fld1.0.2 * _21.fld1.0.2;
_19.0.0 = _21.fld1.0.0 & _14.1;
RET.2 = -_19.0.2;
_4 = '\u{610f}';
_21.fld0 = _19.0.2;
RET.2 = _21.fld0 + _21.fld0;
Goto(bb7)
}
bb7 = {
Call(_26 = dump_var(10_usize, 16_usize, Move(_16), 10_usize, Move(_10), 6_usize, Move(_6), 13_usize, Move(_13)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_26 = dump_var(10_usize, 9_usize, Move(_9), 12_usize, Move(_12), 4_usize, Move(_4), 8_usize, Move(_8)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: (isize, usize, u16),mut _2: u16,mut _3: u16,mut _4: u64,mut _5: u16,mut _6: u16,mut _7: u64) -> (usize, bool, f32) {
mir! {
type RET = (usize, bool, f32);
let _8: Adt37;
let _9: Adt44;
let _10: (i8,);
let _11: [u8; 2];
let _12: f64;
let _13: ();
let _14: ();
{
_3 = _1.2 * _1.2;
_1.1 = !5736268051817167018_usize;
_1.1 = !3_usize;
RET.1 = _2 <= _1.2;
Goto(bb1)
}
bb1 = {
RET.0 = _1.1;
_1.1 = 108930756649114802333287912106316509715_i128 as usize;
RET.0 = 1211374433724882878787692763390010012_u128 as usize;
_8.fld2 = 4105303640959193726_i64;
_7 = _4;
_1.0 = 9223372036854775807_isize;
_8.fld5 = [4978_i16,6992_i16,10481_i16,(-16234_i16),(-19172_i16)];
_1.2 = _8.fld2 as u16;
_1.1 = RET.0;
_6 = _1.0 as u16;
_8.fld1 = '\u{bdc78}';
Goto(bb2)
}
bb2 = {
_7 = _4;
_1.2 = !_3;
RET.2 = 19183_i16 as f32;
_5 = !_1.2;
_9.fld2 = _7;
_8.fld3.1 = RET.1;
_10 = ((-101_i8),);
_9.fld1 = (RET, _9.fld2, (-2049624165_i32), _1.0);
Goto(bb3)
}
bb3 = {
Call(_13 = dump_var(11_usize, 1_usize, Move(_1), 10_usize, Move(_10), 6_usize, Move(_6), 2_usize, Move(_2)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{ab022}'), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(56_i8), std::hint::black_box(233884399233095438431918802996657776710_u128), std::hint::black_box((-903989604_i32)), std::hint::black_box((-3475965308476451299_i64)), std::hint::black_box((-6624336047510016840090467753157755435_i128)), std::hint::black_box(2_usize), std::hint::black_box(99_u8), std::hint::black_box(65399_u16), std::hint::black_box(3747033783_u32), std::hint::black_box(8073546552725385384_u64));
                
            }
impl PrintFDebug for Adt35{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt35::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt35 {
Variant0{
fld0: (usize, bool, f32),
fld1: usize,
fld2: u64,
fld3: i128,

},
Variant1{
fld0: *const i128,

},
Variant2{
fld0: (isize, usize, u16),
fld1: (u8, [u128; 7]),
fld2: (i8,),

}}
impl PrintFDebug for Adt36{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt36::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt36 {
Variant0{
fld0: i32,

},
Variant1{
fld0: *mut f32,
fld1: i32,
fld2: isize,

}}
impl PrintFDebug for Adt37{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt37{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt37 {
fld0: bool,
fld1: char,
fld2: i64,
fld3: (usize, bool, f32),
fld4: i16,
fld5: [i16; 5],
}
impl PrintFDebug for Adt38{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt38::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt38 {
Variant0{
fld0: f64,
fld1: char,
fld2: u32,
fld3: i8,
fld4: [u128; 7],
fld5: [bool; 5],
fld6: [u8; 2],

},
Variant1{
fld0: bool,
fld1: Adt36,
fld2: (isize, u128, i128, u16, u16, (usize, bool, f32)),
fld3: Adt37,

},
Variant2{
fld0: i32,
fld1: (f32,),
fld2: [i16; 5],

},
Variant3{
fld0: [bool; 5],
fld1: char,
fld2: f64,
fld3: [u128; 7],
fld4: (isize, usize, u16),
fld5: *const i128,
fld6: u128,

}}
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt39{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt39 {
fld0: *const i128,
fld1: u16,
fld2: f64,
fld3: u64,
fld4: f32,
fld5: Adt37,
fld6: i64,
fld7: (i8,),
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
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: i16,
fld1: Adt36,

},
Variant1{
fld0: (isize, usize, u16),
fld1: char,
fld2: u32,
fld3: Adt35,
fld4: (i8,),
fld5: u128,
fld6: Adt36,
fld7: i128,

},
Variant2{
fld0: (isize, u128, i128, u16, u16, (usize, bool, f32)),
fld1: i16,

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt41::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: Adt35,
fld1: Adt40,
fld2: i128,

},
Variant1{
fld0: usize,
fld1: [u128; 7],
fld2: Adt39,
fld3: u8,

},
Variant2{
fld0: u16,
fld1: Adt39,
fld2: i32,
fld3: i8,

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
fld0: Adt39,
fld1: (u8, [u128; 7]),
fld2: isize,
fld3: i128,
fld4: Adt41,

},
Variant1{
fld0: (f32,),
fld1: char,
fld2: f64,
fld3: i8,
fld4: (isize, usize, u16),
fld5: Adt41,
fld6: Adt39,

},
Variant2{
fld0: u128,
fld1: (u8, [u128; 7]),
fld2: isize,

},
Variant3{
fld0: bool,
fld1: u64,
fld2: (i8,),

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
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
fld0: *mut i16,
fld1: Adt41,
fld2: Adt38,

},
Variant1{
fld0: Adt36,
fld1: i64,
fld2: *const i32,
fld3: (f32,),

},
Variant2{
fld0: Adt36,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: f32,
fld1: ((usize, bool, f32), u64, i32, isize),
fld2: u64,
fld3: [bool; 5],
}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: u8,

},
Variant1{
fld0: Adt42,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: (u8, [u128; 7]),
fld1: Adt44,
fld2: (isize, usize, u16),
}
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: (f32,),
fld1: (isize, u128, i128, u16, u16, (usize, bool, f32)),
fld2: isize,
fld3: *const i32,
fld4: i16,
fld5: Adt45,
fld6: [u32; 8],

},
Variant1{
fld0: f32,
fld1: (u8, [u128; 7]),
fld2: [u8; 2],
fld3: *const i32,
fld4: i16,

},
Variant2{
fld0: i16,

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
fld0: Adt35,
fld1: [i16; 5],

},
Variant1{
fld0: Adt43,
fld1: Adt38,
fld2: *const i32,
fld3: [u128; 7],
fld4: f32,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: *const i32,
fld1: [u32; 8],
fld2: u16,
fld3: Adt47,
fld4: Adt44,
fld5: (i8,),

},
Variant1{
fld0: i32,
fld1: f64,
fld2: [u8; 2],
fld3: (isize, usize, u16),
fld4: u32,

},
Variant2{
fld0: u128,
fld1: Adt41,
fld2: (f32,),
fld3: u64,
fld4: [u128; 7],
fld5: Adt37,

},
Variant3{
fld0: [u8; 2],
fld1: Adt39,
fld2: Adt44,
fld3: i8,
fld4: *const i128,
fld5: Adt37,
fld6: Adt48,

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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: [u8; 2],
fld1: [bool; 5],
fld2: Adt48,
fld3: Adt47,
fld4: Adt45,
fld5: Adt41,
fld6: i64,

},
Variant1{
fld0: Adt38,
fld1: *const i32,
fld2: Adt35,
fld3: Adt48,
fld4: i32,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: Adt36,
fld1: char,
fld2: Adt44,
fld3: i8,
fld4: Adt41,
fld5: (usize, bool, f32),
fld6: usize,
fld7: *mut i16,
}

