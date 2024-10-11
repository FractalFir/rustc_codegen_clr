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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: u8,mut _7: u16,mut _8: u128,mut _9: usize) -> isize {
mir! {
type RET = isize;
let _10: f32;
let _11: (i16, f32);
let _12: isize;
let _13: Adt47;
let _14: (char, i16);
let _15: [bool; 7];
let _16: u32;
let _17: isize;
let _18: Adt49;
let _19: isize;
let _20: [u8; 3];
let _21: (char, i16);
let _22: i8;
let _23: [bool; 7];
let _24: usize;
let _25: *const u8;
let _26: f64;
let _27: ();
let _28: ();
{
RET = !(-9223372036854775808_isize);
_1 = !true;
_1 = !false;
_6 = 100_u8 ^ 202_u8;
_2 = '\u{5980e}';
_4 = -94_i8;
_1 = false;
_7 = 33360_u16;
_8 = 203303172781779106070794996098365191606_u128;
_1 = _7 > _7;
_9 = !13370086335563706598_usize;
_10 = _6 as f32;
_9 = 0_usize;
_6 = 25121_i16 as u8;
_5 = 13203_i16 + (-19850_i16);
_8 = !97104406115355087502146918831936007214_u128;
_7 = 54524_u16 ^ 30827_u16;
_8 = 296409781519253728178229744634618553304_u128;
_1 = true;
_4 = _10 as i8;
_4 = (-98_i8);
_7 = _6 as u16;
_3 = RET;
_4 = (-37_i8) << _7;
match _9 {
1 => bb2,
2 => bb3,
3 => bb4,
0 => bb6,
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
RET = _2 as isize;
_6 = 5_u8 * 139_u8;
RET = _3 - _3;
_5 = -(-7849_i16);
_11.1 = _10;
_5 = !(-18383_i16);
_10 = -_11.1;
_3 = (-8697870006340403038_i64) as isize;
_7 = !53881_u16;
_2 = '\u{7b7ae}';
RET = !_3;
_8 = !263225486541301360115369263499137982314_u128;
_12 = (-148303931127457122773466205193040446705_i128) as isize;
_3 = RET ^ _12;
RET = (-5081044888786890435_i64) as isize;
RET = _3;
_4 = _1 as i8;
_8 = 4647096324972923568_i64 as u128;
_1 = false ^ false;
_11.0 = (-974150714_i32) as i16;
_4 = (-98_i8) - (-103_i8);
_8 = !258520958408180681593961208593105553866_u128;
RET = _12 - _3;
Call(RET = fn1(_6, _7, _10, _3, _2, _11.1, _6, _7, _7, _11.0, _7, _9, _11, _11.1, _1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_8 = 99386288149676463307844271673779922523_u128 & 334877285334539548433775804268113945591_u128;
_11 = (_5, _10);
_11 = (_5, _10);
_13.fld2.fld1.1 = !_5;
_4 = _6 as i8;
_3 = _8 as isize;
_14 = (_2, _13.fld2.fld1.1);
_10 = _9 as f32;
_13.fld1 = -_11.1;
_14.0 = _2;
_14.0 = _2;
_20 = [_6,_6,_6];
RET = !_3;
_13.fld2.fld1.2 = (-1995289182_i32);
_13.fld0 = (_7,);
Goto(bb8)
}
bb8 = {
match _9 {
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
0 => bb16,
_ => bb15
}
}
bb9 = {
_8 = 99386288149676463307844271673779922523_u128 & 334877285334539548433775804268113945591_u128;
_11 = (_5, _10);
_11 = (_5, _10);
_13.fld2.fld1.1 = !_5;
_4 = _6 as i8;
_3 = _8 as isize;
_14 = (_2, _13.fld2.fld1.1);
_10 = _9 as f32;
_13.fld1 = -_11.1;
_14.0 = _2;
_14.0 = _2;
_20 = [_6,_6,_6];
RET = !_3;
_13.fld2.fld1.2 = (-1995289182_i32);
_13.fld0 = (_7,);
Goto(bb8)
}
bb10 = {
RET = _2 as isize;
_6 = 5_u8 * 139_u8;
RET = _3 - _3;
_5 = -(-7849_i16);
_11.1 = _10;
_5 = !(-18383_i16);
_10 = -_11.1;
_3 = (-8697870006340403038_i64) as isize;
_7 = !53881_u16;
_2 = '\u{7b7ae}';
RET = !_3;
_8 = !263225486541301360115369263499137982314_u128;
_12 = (-148303931127457122773466205193040446705_i128) as isize;
_3 = RET ^ _12;
RET = (-5081044888786890435_i64) as isize;
RET = _3;
_4 = _1 as i8;
_8 = 4647096324972923568_i64 as u128;
_1 = false ^ false;
_11.0 = (-974150714_i32) as i16;
_4 = (-98_i8) - (-103_i8);
_8 = !258520958408180681593961208593105553866_u128;
RET = _12 - _3;
Call(RET = fn1(_6, _7, _10, _3, _2, _11.1, _6, _7, _7, _11.0, _7, _9, _11, _11.1, _1), ReturnTo(bb7), UnwindUnreachable())
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
Return()
}
bb15 = {
Return()
}
bb16 = {
_9 = _13.fld1 as usize;
_21.1 = _4 as i16;
RET = _3 | _3;
_13.fld2.fld1.3 = 121029682099323827082630598580492991086_i128 as f64;
Goto(bb17)
}
bb17 = {
Call(_27 = dump_var(0_usize, 3_usize, Move(_3), 8_usize, Move(_8), 2_usize, Move(_2), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_27 = dump_var(0_usize, 4_usize, Move(_4), 7_usize, Move(_7), 28_usize, _28, 28_usize, _28), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: u8,mut _2: u16,mut _3: f32,mut _4: isize,mut _5: char,mut _6: f32,mut _7: u8,mut _8: u16,mut _9: u16,mut _10: i16,mut _11: u16,mut _12: usize,mut _13: (i16, f32),mut _14: f32,mut _15: bool) -> isize {
mir! {
type RET = isize;
let _16: i64;
let _17: bool;
let _18: bool;
let _19: u8;
let _20: isize;
let _21: u128;
let _22: [i8; 8];
let _23: i64;
let _24: Adt48;
let _25: isize;
let _26: Adt43;
let _27: u16;
let _28: isize;
let _29: (i16, f32);
let _30: Adt41;
let _31: [bool; 7];
let _32: *mut u64;
let _33: i32;
let _34: ();
let _35: ();
{
_8 = _9 >> _7;
_14 = 4699128672799149414_i64 as f32;
RET = -_4;
_9 = _8;
_9 = 3343547208_u32 as u16;
_6 = _14;
RET = _4 & _4;
_5 = '\u{64db1}';
_4 = RET >> _8;
_15 = false | false;
_19 = 1607894245_u32 as u8;
Call(_13.1 = fn2(_4, _4, _3, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_19 = _12 as u8;
_20 = 42406052447132558063170473584427424763_u128 as isize;
Goto(bb2)
}
bb2 = {
RET = _19 as isize;
_21 = !1167150146499440515403880139921326536_u128;
_8 = _9;
_14 = _9 as f32;
_17 = !_15;
RET = _19 as isize;
_9 = 3269256282_u32 as u16;
_20 = _4;
_3 = 641971893_u32 as f32;
_10 = _13.0;
_23 = (-4400540752143903906_i64) >> _2;
_14 = -_13.1;
_7 = 161542004335657423105471856260790261142_i128 as u8;
_20 = _4;
_15 = _17;
_22 = [(-98_i8),114_i8,122_i8,(-5_i8),95_i8,(-42_i8),76_i8,3_i8];
match _12 {
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
0 => bb8,
_ => bb7
}
}
bb3 = {
_19 = _12 as u8;
_20 = 42406052447132558063170473584427424763_u128 as isize;
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
_17 = _15;
_16 = -_23;
_18 = _15 & _15;
_8 = !_11;
_21 = 325481038876857798018522423870449646762_u128;
_10 = _13.0;
_17 = _18;
_22 = [59_i8,(-10_i8),34_i8,51_i8,15_i8,92_i8,(-64_i8),(-54_i8)];
_26.fld1.3 = _1 as f64;
_12 = !4346500425841317546_usize;
_26.fld1.3 = _13.0 as f64;
_26.fld1.2 = -1071092723_i32;
match _21 {
0 => bb5,
1 => bb2,
2 => bb9,
3 => bb10,
325481038876857798018522423870449646762 => bb12,
_ => bb11
}
}
bb9 = {
_19 = _12 as u8;
_20 = 42406052447132558063170473584427424763_u128 as isize;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_8 = RET as u16;
_6 = _13.0 as f32;
_17 = !_15;
_13 = (_10, _14);
_27 = !_2;
_19 = _1;
_16 = _23 | _23;
_26.fld2 = [(-89_i8),(-48_i8),68_i8,(-32_i8),69_i8,(-7_i8),(-102_i8),(-107_i8)];
_18 = _26.fld1.3 >= _26.fld1.3;
_14 = _13.1 - _13.1;
_26.fld1.0 = core::ptr::addr_of_mut!(_4);
_26.fld1.1 = _10 + _13.0;
_20 = !_4;
_20 = RET;
_14 = _26.fld1.1 as f32;
_25 = -RET;
_29.0 = -_13.0;
match _21 {
325481038876857798018522423870449646762 => bb13,
_ => bb8
}
}
bb13 = {
_7 = _19;
_29 = _13;
_26.fld1.2 = 9544469386995865198_u64 as i32;
RET = !_4;
_2 = _27;
_29.1 = -_3;
_16 = _23;
Goto(bb14)
}
bb14 = {
_4 = RET & _20;
_26.fld1.0 = core::ptr::addr_of_mut!(_25);
_25 = _13.1 as isize;
_28 = -_20;
_26.fld1.0 = core::ptr::addr_of_mut!(_25);
_7 = _26.fld1.2 as u8;
_8 = _11 & _9;
_12 = !13385268012101383668_usize;
_29.1 = _3 - _6;
_3 = -_13.1;
_2 = !_8;
_31 = [_18,_17,_18,_18,_15,_15,_15];
_30.fld1 = !_12;
_30.fld0 = _26.fld1.3 * _26.fld1.3;
_8 = _2;
_22 = [(-56_i8),(-77_i8),113_i8,68_i8,(-72_i8),52_i8,106_i8,75_i8];
_22 = [(-50_i8),(-99_i8),89_i8,(-123_i8),38_i8,69_i8,(-63_i8),78_i8];
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(1_usize, 7_usize, Move(_7), 2_usize, Move(_2), 25_usize, Move(_25), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(1_usize, 11_usize, Move(_11), 20_usize, Move(_20), 17_usize, Move(_17), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(1_usize, 28_usize, Move(_28), 21_usize, Move(_21), 22_usize, Move(_22), 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: isize,mut _2: isize,mut _3: f32,mut _4: isize) -> f32 {
mir! {
type RET = f32;
let _5: [u8; 3];
let _6: u128;
let _7: (i32, f32, (u16,));
let _8: f64;
let _9: [isize; 4];
let _10: Adt49;
let _11: u16;
let _12: i8;
let _13: [u8; 3];
let _14: (char, i16);
let _15: *const [isize; 4];
let _16: ();
let _17: ();
{
RET = -_3;
_1 = -_4;
_1 = 5_u8 as isize;
_4 = _2;
_4 = _2 & _2;
Call(_2 = fn3(_3, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _4 as f32;
_3 = -RET;
_2 = _4;
RET = 266819253975736416792930718344384130736_u128 as f32;
_5 = [119_u8,174_u8,141_u8];
RET = 147_u8 as f32;
_4 = 19278277690006422306483952544208952827_u128 as isize;
RET = (-3670696421235468658_i64) as f32;
_1 = _2 << _2;
_7.0 = 2119019760_i32;
match _7.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
2119019760 => bb9,
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
_7.2 = (62300_u16,);
_6 = !267112795098927466271812149147267993767_u128;
_7.1 = -_3;
_7.0 = (-751335062_i32);
_3 = _7.1 * _7.1;
Call(_3 = core::intrinsics::transmute(_7.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_6 = !107075904662447140245589027834427001716_u128;
_7.0 = 61_i8 as i32;
_7.1 = _7.0 as f32;
RET = _3;
RET = _3 - _3;
_7.2 = (36408_u16,);
_4 = _2;
_7.2 = (29957_u16,);
_5 = [14_u8,144_u8,249_u8];
_1 = _2;
_7.0 = -(-420409181_i32);
RET = -_7.1;
_7.0 = -439552027_i32;
_2 = _4 * _1;
_7.0 = !(-1384793574_i32);
_7.1 = RET - RET;
_7.2 = (13986_u16,);
_11 = _7.2.0 * _7.2.0;
_2 = -_1;
_3 = _7.1 + RET;
_7.2.0 = _11 >> _1;
_6 = (-5551_i16) as u128;
RET = (-107863803201984087966143754181849733528_i128) as f32;
_5 = [144_u8,41_u8,2_u8];
_11 = !_7.2.0;
Goto(bb11)
}
bb11 = {
_4 = !_2;
RET = _7.1;
_7.2.0 = _11 >> _1;
_11 = 15086763783878245382_u64 as u16;
_6 = 17832330560961315912_u64 as u128;
_7.1 = _3 + _3;
RET = -_7.1;
_8 = 9622557586000150126_u64 as f64;
_14 = ('\u{bdfdb}', 30513_i16);
_3 = 169074624428441696452157922046669113449_i128 as f32;
_11 = _7.2.0 | _7.2.0;
_14.0 = '\u{a59c3}';
_7.2 = (_11,);
_8 = _14.1 as f64;
_7.0 = (-438213037_i32);
_7.0 = 655871815_i32 >> _4;
_7.2 = (_11,);
_14 = ('\u{88945}', (-6015_i16));
_14 = ('\u{7ea59}', 25952_i16);
_6 = 15082539266106770673_usize as u128;
RET = _7.1 * _7.1;
_15 = core::ptr::addr_of!(_9);
Goto(bb12)
}
bb12 = {
Call(_16 = dump_var(2_usize, 5_usize, Move(_5), 6_usize, Move(_6), 11_usize, Move(_11), 17_usize, _17), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: f32,mut _2: isize) -> isize {
mir! {
type RET = isize;
let _3: f32;
let _4: *mut *const u8;
let _5: u32;
let _6: *mut u64;
let _7: i128;
let _8: i16;
let _9: isize;
let _10: (i64,);
let _11: f64;
let _12: isize;
let _13: &'static bool;
let _14: (i64,);
let _15: (u16,);
let _16: (i16, f32);
let _17: Adt43;
let _18: u32;
let _19: Adt36;
let _20: (*mut isize, i16, i32, f64);
let _21: isize;
let _22: bool;
let _23: ();
let _24: ();
{
RET = _2;
RET = -_2;
_2 = 5_usize as isize;
RET = -_2;
RET = _2;
RET = _2 & _2;
_1 = 105_u8 as f32;
_1 = (-166184209513577126377581259564336242416_i128) as f32;
_2 = !RET;
_2 = RET << RET;
RET = 3884425485_u32 as isize;
_3 = _1 + _1;
_3 = _1 + _1;
RET = 295839650558765278335784964619930643990_u128 as isize;
_2 = !RET;
RET = 2007591808597787939_i64 as isize;
RET = -_2;
_2 = RET;
_2 = 1480772230_u32 as isize;
_1 = _3 + _3;
RET = 161379402181083068427045946234746424594_u128 as isize;
_2 = '\u{26c4e}' as isize;
_1 = _3;
RET = -_2;
RET = -_2;
_2 = 105856124584974846262259566492472132777_u128 as isize;
_2 = 7868_i16 as isize;
Call(_1 = fn4(RET, _2, _3, _2, _3, _3, _2, RET, _3, RET, _2, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = RET | RET;
RET = _2;
_3 = _1 * _1;
RET = _2 + _2;
_1 = _3;
_2 = 14403338188261760441_u64 as isize;
RET = 112_u8 as isize;
_5 = !1346971646_u32;
_2 = RET >> _5;
_3 = _1;
_1 = 5997513611224952896_u64 as f32;
_2 = RET ^ RET;
_1 = _3;
_1 = -_3;
_3 = 79961903627202173055100481630753773705_u128 as f32;
RET = _2;
_7 = -163607541327864761135951738324206770862_i128;
_1 = _3 + _3;
_8 = 61719_u16 as i16;
RET = _2;
_5 = !143736192_u32;
RET = 45708_u16 as isize;
_7 = _8 as i128;
RET = _2;
_1 = (-150397559060159525_i64) as f32;
_2 = -RET;
Call(_4 = fn15(RET, _8, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = 160250804258230744791902646250762493679_i128;
_1 = -_3;
_3 = _1 - _1;
_10 = (8931024947377647906_i64,);
_1 = _3 - _3;
_9 = -RET;
_7 = -165635216978803902144610201107526562232_i128;
_7 = 74574280926178530225810898389227741378_i128 | 66104555753854233946325669208103772448_i128;
_10.0 = -(-1754770239564224867_i64);
_3 = 320186376513090301972588092588426465283_u128 as f32;
RET = _2;
_2 = _5 as isize;
_7 = !92293784836863630860430028253327429779_i128;
_11 = 13920084746146908270_usize as f64;
_12 = -_2;
Goto(bb3)
}
bb3 = {
_10.0 = (-6525741581127026570_i64);
_7 = 124126950311921344446918939774895355213_i128 & (-56612950802082932305229593578217198960_i128);
_12 = !_9;
_11 = 214_u8 as f64;
_1 = _3;
_12 = -_2;
_2 = !_9;
_5 = 481742849_u32;
_5 = 2380969599_u32 * 635742366_u32;
RET = _9;
_8 = 15089_i16;
Goto(bb4)
}
bb4 = {
_5 = !4143736006_u32;
_10 = (1678281053728702827_i64,);
_12 = 6622_u16 as isize;
_5 = _10.0 as u32;
_2 = _9 << _8;
_9 = RET;
_5 = 3951798729_u32;
_5 = (-117_i8) as u32;
_8 = (-18076_i16) * (-11500_i16);
_14.0 = _10.0 + _10.0;
_15 = (19771_u16,);
_16.1 = _3;
RET = -_9;
_16.0 = _8 | _8;
_3 = _1;
_2 = -_9;
RET = _9;
match _10.0 {
0 => bb5,
1 => bb6,
2 => bb7,
1678281053728702827 => bb9,
_ => bb8
}
}
bb5 = {
_10.0 = (-6525741581127026570_i64);
_7 = 124126950311921344446918939774895355213_i128 & (-56612950802082932305229593578217198960_i128);
_12 = !_9;
_11 = 214_u8 as f64;
_1 = _3;
_12 = -_2;
_2 = !_9;
_5 = 481742849_u32;
_5 = 2380969599_u32 * 635742366_u32;
RET = _9;
_8 = 15089_i16;
Goto(bb4)
}
bb6 = {
_7 = 160250804258230744791902646250762493679_i128;
_1 = -_3;
_3 = _1 - _1;
_10 = (8931024947377647906_i64,);
_1 = _3 - _3;
_9 = -RET;
_7 = -165635216978803902144610201107526562232_i128;
_7 = 74574280926178530225810898389227741378_i128 | 66104555753854233946325669208103772448_i128;
_10.0 = -(-1754770239564224867_i64);
_3 = 320186376513090301972588092588426465283_u128 as f32;
RET = _2;
_2 = _5 as isize;
_7 = !92293784836863630860430028253327429779_i128;
_11 = 13920084746146908270_usize as f64;
_12 = -_2;
Goto(bb3)
}
bb7 = {
_2 = RET | RET;
RET = _2;
_3 = _1 * _1;
RET = _2 + _2;
_1 = _3;
_2 = 14403338188261760441_u64 as isize;
RET = 112_u8 as isize;
_5 = !1346971646_u32;
_2 = RET >> _5;
_3 = _1;
_1 = 5997513611224952896_u64 as f32;
_2 = RET ^ RET;
_1 = _3;
_1 = -_3;
_3 = 79961903627202173055100481630753773705_u128 as f32;
RET = _2;
_7 = -163607541327864761135951738324206770862_i128;
_1 = _3 + _3;
_8 = 61719_u16 as i16;
RET = _2;
_5 = !143736192_u32;
RET = 45708_u16 as isize;
_7 = _8 as i128;
RET = _2;
_1 = (-150397559060159525_i64) as f32;
_2 = -RET;
Call(_4 = fn15(RET, _8, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
_16 = (_8, _1);
_17.fld2 = [(-7_i8),(-110_i8),70_i8,30_i8,94_i8,(-65_i8),76_i8,(-126_i8)];
RET = !_9;
_16 = (_8, _3);
_2 = !_9;
_17.fld3 = _4;
_16.0 = -_8;
_16.1 = 113_i8 as f32;
Goto(bb10)
}
bb10 = {
_2 = _9 + _12;
_2 = RET >> _16.0;
_4 = _17.fld3;
_17.fld1.3 = _11;
_15.0 = 51212_u16 & 50137_u16;
_15 = (55063_u16,);
_12 = !_2;
_17.fld2 = [(-17_i8),5_i8,(-14_i8),51_i8,(-99_i8),41_i8,57_i8,(-34_i8)];
_17.fld1.3 = -_11;
_14.0 = _10.0 - _10.0;
_18 = _5 ^ _5;
RET = _9;
_17.fld1.1 = _16.0 & _8;
_20.1 = _17.fld1.1 << RET;
match _10.0 {
0 => bb9,
1 => bb8,
2 => bb3,
3 => bb11,
1678281053728702827 => bb13,
_ => bb12
}
}
bb11 = {
_16 = (_8, _1);
_17.fld2 = [(-7_i8),(-110_i8),70_i8,30_i8,94_i8,(-65_i8),76_i8,(-126_i8)];
RET = !_9;
_16 = (_8, _3);
_2 = !_9;
_17.fld3 = _4;
_16.0 = -_8;
_16.1 = 113_i8 as f32;
Goto(bb10)
}
bb12 = {
_10.0 = (-6525741581127026570_i64);
_7 = 124126950311921344446918939774895355213_i128 & (-56612950802082932305229593578217198960_i128);
_12 = !_9;
_11 = 214_u8 as f64;
_1 = _3;
_12 = -_2;
_2 = !_9;
_5 = 481742849_u32;
_5 = 2380969599_u32 * 635742366_u32;
RET = _9;
_8 = 15089_i16;
Goto(bb4)
}
bb13 = {
_10.0 = 1_usize as i64;
_17.fld2 = [6_i8,49_i8,92_i8,(-81_i8),44_i8,117_i8,84_i8,35_i8];
_20.3 = _11 - _11;
_17.fld2 = [(-43_i8),(-64_i8),(-47_i8),43_i8,21_i8,(-104_i8),10_i8,42_i8];
_14 = (_10.0,);
_4 = _17.fld3;
_17.fld1.1 = _20.1 >> _18;
RET = _9 * _12;
_16.1 = _1;
_17.fld1.0 = core::ptr::addr_of_mut!(_21);
_9 = _7 as isize;
_15.0 = 32766_u16 << RET;
_15 = (28289_u16,);
_5 = !_18;
_11 = -_20.3;
_14.0 = _10.0 ^ _10.0;
_20.0 = core::ptr::addr_of_mut!(_2);
_14 = (_10.0,);
_19.fld1 = '\u{188e3}';
_17.fld1.1 = _20.1 + _20.1;
_16.0 = 22_u8 as i16;
RET = _17.fld1.1 as isize;
_5 = 2391847732623736604_u64 as u32;
_18 = _5 & _5;
_17.fld1 = (_20.0, _20.1, 50519169_i32, _11);
_17.fld2 = [(-79_i8),126_i8,(-31_i8),(-94_i8),54_i8,(-72_i8),29_i8,16_i8];
Goto(bb14)
}
bb14 = {
Call(_23 = dump_var(3_usize, 18_usize, Move(_18), 15_usize, Move(_15), 2_usize, Move(_2), 10_usize, Move(_10)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_23 = dump_var(3_usize, 8_usize, Move(_8), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: isize,mut _2: isize,mut _3: f32,mut _4: isize,mut _5: f32,mut _6: f32,mut _7: isize,mut _8: isize,mut _9: f32,mut _10: isize,mut _11: isize,mut _12: f32) -> f32 {
mir! {
type RET = f32;
let _13: u128;
let _14: (u16,);
let _15: f64;
let _16: *mut u64;
let _17: isize;
let _18: u16;
let _19: isize;
let _20: (i64,);
let _21: i64;
let _22: char;
let _23: u8;
let _24: f32;
let _25: [u8; 3];
let _26: *mut isize;
let _27: u32;
let _28: *mut u64;
let _29: Adt41;
let _30: u16;
let _31: i8;
let _32: (char, i16);
let _33: bool;
let _34: i16;
let _35: f64;
let _36: i64;
let _37: (u16,);
let _38: Adt38;
let _39: [bool; 7];
let _40: ();
let _41: ();
{
_10 = _4 >> _2;
_9 = _5;
_6 = -_9;
_9 = 32654_u16 as f32;
_10 = -_8;
RET = 82_u8 as f32;
RET = (-1972978435_i32) as f32;
_8 = (-85_i8) as isize;
_12 = -_3;
_11 = _8;
_13 = 1613623766893529257_i64 as u128;
RET = _12 - _9;
_13 = 10612045390275381935_usize as u128;
_10 = _1;
RET = _5 - _5;
_7 = _2 >> _11;
_9 = 1_usize as f32;
_9 = RET;
_14 = (55053_u16,);
_12 = 37_u8 as f32;
_14.0 = 39969_u16 << _8;
_2 = _11 * _4;
_2 = _11 - _10;
Goto(bb1)
}
bb1 = {
_14.0 = '\u{38fb0}' as u16;
_15 = _4 as f64;
_18 = !_14.0;
_6 = -_3;
_8 = _2;
_1 = (-4901978695654679920_i64) as isize;
_20.0 = (-2068257244270466156_i64) + (-8881023265517392106_i64);
_6 = -_9;
_8 = !_2;
_18 = _14.0;
_6 = _9;
_9 = _12 - _6;
RET = -_6;
_6 = -_9;
_14.0 = (-1137116906_i32) as u16;
RET = _5;
Goto(bb2)
}
bb2 = {
_9 = -_6;
Call(_14.0 = fn5(_3, _2, _7, _6, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_20 = ((-1979177873028389836_i64),);
_11 = !_8;
_26 = core::ptr::addr_of_mut!(_8);
_24 = -_9;
_6 = -_12;
RET = _24;
_13 = 87179170089768300565507503470858804105_u128 >> _4;
_13 = 106_u8 as u128;
_8 = !_2;
_12 = _9 + _5;
_6 = _12 * RET;
_21 = _20.0 << _13;
_13 = 319563326350883122486723554283252181959_u128 | 265550331917043980049914625997532867050_u128;
_29 = Adt41 { fld0: _15,fld1: 4_usize };
RET = 1473278643_i32 as f32;
_19 = _8 | _1;
_30 = !_18;
(*_26) = -_2;
_29.fld1 = 5_usize;
(*_26) = _2 * _7;
match _20.0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463461395429558739821620 => bb9,
_ => bb8
}
}
bb4 = {
_9 = -_6;
Call(_14.0 = fn5(_3, _2, _7, _6, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_14.0 = '\u{38fb0}' as u16;
_15 = _4 as f64;
_18 = !_14.0;
_6 = -_3;
_8 = _2;
_1 = (-4901978695654679920_i64) as isize;
_20.0 = (-2068257244270466156_i64) + (-8881023265517392106_i64);
_6 = -_9;
_8 = !_2;
_18 = _14.0;
_6 = _9;
_9 = _12 - _6;
RET = -_6;
_6 = -_9;
_14.0 = (-1137116906_i32) as u16;
RET = _5;
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
_19 = _2 | (*_26);
_11 = _4 >> _19;
(*_26) = 564777064_i32 as isize;
(*_26) = _2;
_4 = false as isize;
match _20.0 {
0 => bb10,
340282366920938463461395429558739821620 => bb12,
_ => bb11
}
}
bb10 = {
Return()
}
bb11 = {
_9 = -_6;
Call(_14.0 = fn5(_3, _2, _7, _6, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb12 = {
_30 = _14.0;
_14.0 = _30 << (*_26);
_7 = '\u{f0695}' as isize;
(*_26) = -_2;
_32.0 = '\u{ff7e9}';
_34 = 7542_i16;
_2 = _11;
_31 = -(-21_i8);
_4 = 2188724191_u32 as isize;
_23 = !235_u8;
_8 = _19;
_32.0 = '\u{10a704}';
_17 = 15190505633141541062_u64 as isize;
_8 = -_1;
_15 = _29.fld0 - _29.fld0;
_20 = (_21,);
_32 = ('\u{d2dbe}', _34);
Call(_6 = fn11(_26, _14, (*_26), _11, _23, _26, _2, _9, _2, _19, _32.1, _34, _3, _20), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_20.0 = _32.0 as i64;
_29 = Adt41 { fld0: _15,fld1: 4296549992106839980_usize };
_35 = -_29.fld0;
_34 = _32.1 ^ _32.1;
Call(_16 = fn12(_14.0, _11, _26), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_33 = !false;
_29.fld0 = _35;
_13 = 12413053337600265386708717254993022252_u128;
_26 = core::ptr::addr_of_mut!(_7);
_31 = _20.0 as i8;
_27 = !164896882_u32;
_2 = _11;
_13 = 38470845925805170496736647553750618809_u128;
RET = _29.fld1 as f32;
_37 = (_14.0,);
_35 = _29.fld0 - _29.fld0;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(4_usize, 19_usize, Move(_19), 11_usize, Move(_11), 20_usize, Move(_20), 33_usize, Move(_33)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(4_usize, 2_usize, Move(_2), 8_usize, Move(_8), 23_usize, Move(_23), 34_usize, Move(_34)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(4_usize, 27_usize, Move(_27), 37_usize, Move(_37), 1_usize, Move(_1), 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: f32,mut _2: isize,mut _3: isize,mut _4: f32,mut _5: f32) -> u16 {
mir! {
type RET = u16;
let _6: i64;
let _7: Adt41;
let _8: isize;
let _9: Adt46;
let _10: f64;
let _11: isize;
let _12: Adt41;
let _13: f32;
let _14: u64;
let _15: Adt49;
let _16: char;
let _17: [i8; 8];
let _18: Adt41;
let _19: Adt43;
let _20: f32;
let _21: i64;
let _22: u16;
let _23: (i16, f32);
let _24: (i64,);
let _25: usize;
let _26: usize;
let _27: [i8; 8];
let _28: [u8; 3];
let _29: [i8; 8];
let _30: Adt49;
let _31: (i32, f32, (u16,));
let _32: [bool; 7];
let _33: [i8; 8];
let _34: Adt41;
let _35: ();
let _36: ();
{
RET = !6986_u16;
RET = !11328_u16;
_4 = -_5;
RET = !51386_u16;
_6 = !(-9199193928880914512_i64);
RET = 12622_u16 | 30341_u16;
RET = 58749_u16 & 39274_u16;
_2 = !_3;
_4 = 157_u8 as f32;
_4 = -_5;
_7.fld1 = 6_usize | 2518802302348686342_usize;
_4 = _5 + _5;
Call(_5 = fn6(_1, RET, _1, _6, _3, _4, _4, RET, _4, _4, _3, _1, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = !5177_u16;
_8 = -_2;
_7.fld0 = (-19956_i16) as f64;
_4 = _5;
_2 = (-59971370747819959438941361868269324192_i128) as isize;
_1 = -_4;
_7.fld1 = 2_usize;
_2 = -_3;
_1 = _4 + _5;
_1 = _4 - _5;
_2 = _5 as isize;
_1 = (-150086654866715165207203135729988021265_i128) as f32;
_3 = !_2;
_4 = -_5;
_10 = _7.fld0;
_5 = _4 * _4;
RET = 34454_u16 >> _2;
RET = !42934_u16;
RET = 64309_u16;
_11 = _2;
_2 = _11 & _11;
_3 = -_2;
_11 = _3;
_11 = _3;
_3 = 41998896462455324041345799109750966541_i128 as isize;
_2 = _11;
Goto(bb2)
}
bb2 = {
_6 = !810802433589010917_i64;
_10 = _7.fld0 * _7.fld0;
_6 = (-779807299034364915_i64);
_12.fld0 = -_7.fld0;
_7.fld1 = !4279581588950361271_usize;
match RET {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
64309 => bb8,
_ => bb7
}
}
bb3 = {
RET = !5177_u16;
_8 = -_2;
_7.fld0 = (-19956_i16) as f64;
_4 = _5;
_2 = (-59971370747819959438941361868269324192_i128) as isize;
_1 = -_4;
_7.fld1 = 2_usize;
_2 = -_3;
_1 = _4 + _5;
_1 = _4 - _5;
_2 = _5 as isize;
_1 = (-150086654866715165207203135729988021265_i128) as f32;
_3 = !_2;
_4 = -_5;
_10 = _7.fld0;
_5 = _4 * _4;
RET = 34454_u16 >> _2;
RET = !42934_u16;
RET = 64309_u16;
_11 = _2;
_2 = _11 & _11;
_3 = -_2;
_11 = _3;
_11 = _3;
_3 = 41998896462455324041345799109750966541_i128 as isize;
_2 = _11;
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
_7 = Adt41 { fld0: _10,fld1: 5_usize };
_4 = _5;
_6 = 4770937421816119895_i64;
_12 = _7;
_4 = -_5;
Goto(bb9)
}
bb9 = {
_8 = !_2;
_13 = _5 + _4;
_11 = _8 << _8;
_11 = _2 * _2;
_7.fld1 = _12.fld1;
_6 = 230966107591123069_i64;
RET = !21016_u16;
_5 = _13 + _13;
_16 = '\u{1803d}';
_1 = _13;
_3 = _2;
_4 = _12.fld0 as f32;
_1 = _13;
_19.fld1.1 = 25926_i16;
_17 = [(-82_i8),80_i8,98_i8,(-106_i8),(-105_i8),(-68_i8),(-114_i8),87_i8];
_4 = _7.fld1 as f32;
_19.fld1.2 = (-1070744028_i32);
_1 = _5 + _13;
_12.fld1 = 691612382_u32 as usize;
_18.fld0 = _7.fld0 * _7.fld0;
_5 = _1;
_18.fld1 = _7.fld1 | _7.fld1;
match _19.fld1.2 {
0 => bb4,
1 => bb6,
340282366920938463463374607430697467428 => bb11,
_ => bb10
}
}
bb10 = {
RET = !5177_u16;
_8 = -_2;
_7.fld0 = (-19956_i16) as f64;
_4 = _5;
_2 = (-59971370747819959438941361868269324192_i128) as isize;
_1 = -_4;
_7.fld1 = 2_usize;
_2 = -_3;
_1 = _4 + _5;
_1 = _4 - _5;
_2 = _5 as isize;
_1 = (-150086654866715165207203135729988021265_i128) as f32;
_3 = !_2;
_4 = -_5;
_10 = _7.fld0;
_5 = _4 * _4;
RET = 34454_u16 >> _2;
RET = !42934_u16;
RET = 64309_u16;
_11 = _2;
_2 = _11 & _11;
_3 = -_2;
_11 = _3;
_11 = _3;
_3 = 41998896462455324041345799109750966541_i128 as isize;
_2 = _11;
Goto(bb2)
}
bb11 = {
_13 = _7.fld1 as f32;
_10 = _12.fld0 - _7.fld0;
_12.fld1 = _7.fld1;
_18.fld1 = _7.fld1;
_12.fld0 = _18.fld0 * _18.fld0;
_18.fld1 = _7.fld1;
_14 = 13407657662322517262_u64 | 2051294278024818245_u64;
_23.0 = _19.fld1.1;
_10 = RET as f64;
_20 = _5;
_17 = [(-48_i8),(-126_i8),116_i8,(-78_i8),14_i8,72_i8,91_i8,(-99_i8)];
_24 = (_6,);
_12.fld1 = _7.fld1;
_7 = _18;
_19.fld1.0 = core::ptr::addr_of_mut!(_2);
Goto(bb12)
}
bb12 = {
_2 = _3 + _3;
_16 = '\u{c2541}';
_19.fld2 = [(-72_i8),121_i8,17_i8,43_i8,31_i8,(-94_i8),(-31_i8),(-58_i8)];
_7 = Adt41 { fld0: _12.fld0,fld1: _12.fld1 };
_23.1 = -_20;
_7.fld0 = _10;
_25 = _12.fld1;
_7.fld0 = 257160277236248680286488611419931685930_u128 as f64;
_19.fld1.2 = _19.fld1.1 as i32;
Call(_5 = core::intrinsics::transmute(_16), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
RET = !60822_u16;
_27 = [_19.fld2[_25],_17[_25],_17[_25],_17[_25],_19.fld2[_25],_19.fld2[_25],_17[_25],_19.fld2[_25]];
_11 = _8;
_19.fld1.1 = _23.0;
_19.fld1.1 = -_23.0;
_19.fld1.0 = core::ptr::addr_of_mut!(_11);
_3 = !_2;
_2 = _1 as isize;
_14 = 15127498770049937674_u64 << _2;
_12 = Adt41 { fld0: _18.fld0,fld1: _18.fld1 };
_19.fld1.0 = core::ptr::addr_of_mut!(_11);
_26 = _3 as usize;
_19.fld1.1 = _27[_25] as i16;
_19.fld1.2 = (-563625963_i32);
_21 = _6 << _14;
_4 = RET as f32;
_12 = Adt41 { fld0: _18.fld0,fld1: _26 };
_12.fld0 = -_18.fld0;
_17[_25] = _27[_25] & _27[_25];
_4 = -_23.1;
RET = !20341_u16;
_19.fld2 = _17;
_28 = [126_u8,200_u8,95_u8];
match _18.fld1 {
0 => bb8,
5 => bb14,
_ => bb2
}
}
bb14 = {
_19.fld1.0 = core::ptr::addr_of_mut!(_3);
_29[_25] = _17[_25] >> _14;
_17 = [_29[_25],_29[_25],_29[_25],_29[_25],_27[_25],_29[_25],_29[_25],_29[_25]];
_27 = _17;
_13 = -_23.1;
_12 = Adt41 { fld0: _18.fld0,fld1: _26 };
_27 = [_17[_25],_17[_25],_19.fld2[_25],_17[_25],_17[_25],_29[_25],_19.fld2[_25],_29[_25]];
_22 = 269531349160221448073925706477435051766_u128 as u16;
_29 = [_17[_25],_27[_25],_17[_25],_27[_25],_27[_25],_27[_25],_27[_25],_17[_25]];
_32 = [false,true,true,false,true,true,false];
_24.0 = -_21;
_3 = _18.fld0 as isize;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(5_usize, 8_usize, Move(_8), 29_usize, Move(_29), 3_usize, Move(_3), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(5_usize, 32_usize, Move(_32), 21_usize, Move(_21), 25_usize, Move(_25), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: f32,mut _2: u16,mut _3: f32,mut _4: i64,mut _5: isize,mut _6: f32,mut _7: f32,mut _8: u16,mut _9: f32,mut _10: f32,mut _11: isize,mut _12: f32,mut _13: f32) -> f32 {
mir! {
type RET = f32;
let _14: [bool; 7];
let _15: Adt39;
let _16: f64;
let _17: (i64,);
let _18: isize;
let _19: f64;
let _20: (char, i16);
let _21: (u128, [isize; 4], (i16, f32));
let _22: isize;
let _23: Adt41;
let _24: i32;
let _25: isize;
let _26: bool;
let _27: Adt48;
let _28: isize;
let _29: (u128, [isize; 4], (i16, f32));
let _30: char;
let _31: Adt44;
let _32: (i16, f32);
let _33: (u128, [isize; 4], (i16, f32));
let _34: (char, i16);
let _35: Adt36;
let _36: bool;
let _37: (char, i16);
let _38: i16;
let _39: i8;
let _40: ();
let _41: ();
{
_9 = _6;
_10 = _4 as f32;
_1 = 1741773317_u32 as f32;
_5 = _11 & _11;
_2 = _8;
RET = _9 - _7;
RET = _7;
Call(_17 = fn7(RET, _8, RET, _9, _9, _12, _13, _7, _9, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_18 = (-134840335881737952422647234244817550613_i128) as isize;
_11 = -_18;
_5 = -_11;
_1 = RET;
_13 = _1 + _7;
_14 = [true,true,true,true,true,false,true];
_11 = !_18;
_16 = (-1583227853_i32) as f64;
RET = _8 as f32;
_12 = _1;
_20.1 = 26870_i16;
_2 = _8 * _8;
_20.0 = '\u{6c15e}';
_10 = _17.0 as f32;
_20.0 = '\u{53877}';
_7 = _16 as f32;
_9 = _3;
_19 = _16;
_4 = -_17.0;
_21.0 = 299578324289059921275964479798303271140_u128 >> _17.0;
_16 = _19 + _19;
RET = _1;
_21.2.1 = -_13;
_16 = _19;
_20.1 = _18 as i16;
Call(_21.2.1 = fn8(_8, _6, _6, _13, RET, _6, _17.0, _13, _13, _1, _17), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_21.2.0 = _20.1;
_20.0 = '\u{108f06}';
_21.2.1 = -_13;
_17 = (_4,);
_10 = _12;
_21.1 = [_11,_18,_5,_18];
_20.0 = '\u{65e0e}';
_22 = _19 as isize;
_20 = ('\u{604fc}', _21.2.0);
Call(_3 = fn10(_21, _18), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_26 = !true;
_16 = _19 - _19;
_8 = !_2;
_18 = _22;
_17 = (_4,);
_21.2.1 = RET - _6;
_25 = _18;
_18 = _22 << _20.1;
_9 = _10 + _21.2.1;
_21.2.0 = _20.1 >> _2;
_11 = _5;
_29.0 = !_21.0;
_17.0 = _16 as i64;
_13 = _9;
Call(_23.fld0 = core::intrinsics::fmaf64(_19, _19, _19), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_24 = !(-2054731886_i32);
_12 = 5_usize as f32;
_19 = -_16;
_29.1 = [_18,_18,_18,_5];
_29.2 = (_21.2.0, RET);
_28 = 6834933725420446330_usize as isize;
_19 = -_16;
_29.1 = [_11,_25,_11,_25];
_17 = (_4,);
_29.2 = (_21.2.0, _9);
_21.1 = [_22,_5,_22,_18];
_29.0 = _21.0 << _29.2.0;
_20 = ('\u{3f577}', _21.2.0);
_23 = Adt41 { fld0: _16,fld1: 3417149938276289583_usize };
_20.0 = '\u{10feca}';
_24 = _29.2.0 as i32;
_23 = Adt41 { fld0: _19,fld1: 4_usize };
_19 = -_23.fld0;
match _23.fld1 {
0 => bb1,
1 => bb2,
4 => bb5,
_ => bb3
}
}
bb5 = {
_17 = (_4,);
_8 = _2 & _2;
_23 = Adt41 { fld0: _16,fld1: 2_usize };
_24 = 1100078361_i32 * 768699840_i32;
_9 = -_10;
match _23.fld1 {
0 => bb3,
1 => bb2,
2 => bb7,
_ => bb6
}
}
bb6 = {
_24 = !(-2054731886_i32);
_12 = 5_usize as f32;
_19 = -_16;
_29.1 = [_18,_18,_18,_5];
_29.2 = (_21.2.0, RET);
_28 = 6834933725420446330_usize as isize;
_19 = -_16;
_29.1 = [_11,_25,_11,_25];
_17 = (_4,);
_29.2 = (_21.2.0, _9);
_21.1 = [_22,_5,_22,_18];
_29.0 = _21.0 << _29.2.0;
_20 = ('\u{3f577}', _21.2.0);
_23 = Adt41 { fld0: _16,fld1: 3417149938276289583_usize };
_20.0 = '\u{10feca}';
_24 = _29.2.0 as i32;
_23 = Adt41 { fld0: _19,fld1: 4_usize };
_19 = -_23.fld0;
match _23.fld1 {
0 => bb1,
1 => bb2,
4 => bb5,
_ => bb3
}
}
bb7 = {
_29.2 = (_21.2.0, _10);
_29.1 = [_18,_18,_5,_11];
_17.0 = _9 as i64;
_30 = _20.0;
_23 = Adt41 { fld0: _16,fld1: 5_usize };
_32.1 = _29.2.1 - _13;
_17.0 = _4;
_8 = 1038015301150323538_u64 as u16;
_29 = (_21.0, _21.1, _21.2);
Goto(bb8)
}
bb8 = {
_20.1 = _21.2.0;
_10 = -_13;
_21.1 = [_28,_5,_25,_28];
_20 = (_30, _21.2.0);
_29.2.0 = _20.1;
_25 = -_18;
_21.0 = 41801061292031278696903829814338921930_i128 as u128;
_16 = -_23.fld0;
_33.2 = (_21.2.0, _32.1);
Goto(bb9)
}
bb9 = {
RET = -_21.2.1;
_21.2.1 = _33.2.1 + _13;
_18 = _25;
_33.2.0 = _20.1 << _22;
_29.0 = _20.1 as u128;
_33.1 = [_5,_11,_11,_11];
_25 = _18;
_26 = !true;
_32 = (_33.2.0, _13);
_2 = _8;
_33.2 = (_20.1, _21.2.1);
_2 = _8;
_33.1 = [_25,_25,_25,_5];
_33 = (_29.0, _29.1, _21.2);
_34.0 = _20.0;
_21.2.0 = -_29.2.0;
_33 = (_29.0, _21.1, _21.2);
_32.0 = -_20.1;
_35.fld1 = _30;
_28 = _22 + _25;
_23.fld0 = _16 + _19;
_17 = (_4,);
_29.2.1 = _3 * _10;
_33.2.0 = _20.1;
_34 = _20;
Goto(bb10)
}
bb10 = {
Call(_40 = dump_var(6_usize, 17_usize, Move(_17), 14_usize, Move(_14), 24_usize, Move(_24), 5_usize, Move(_5)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_40 = dump_var(6_usize, 4_usize, Move(_4), 30_usize, Move(_30), 11_usize, Move(_11), 25_usize, Move(_25)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: f32,mut _2: u16,mut _3: f32,mut _4: f32,mut _5: f32,mut _6: f32,mut _7: f32,mut _8: f32,mut _9: f32,mut _10: f32) -> (i64,) {
mir! {
type RET = (i64,);
let _11: (*mut isize, i16, i32, f64);
let _12: f64;
let _13: u8;
let _14: (i16, f32);
let _15: i16;
let _16: u16;
let _17: (*const u8, [bool; 7]);
let _18: isize;
let _19: ();
let _20: ();
{
RET = ((-3849066912967854510_i64),);
_9 = _8;
_8 = _3;
_1 = -_3;
_4 = _1;
_5 = _9 - _8;
_2 = !3673_u16;
_11.1 = (-21934_i16) & 3220_i16;
_11.3 = 4163204196_u32 as f64;
_5 = _4;
_8 = -_10;
RET = ((-3242787858241675108_i64),);
_10 = 148568716237111593376305363111582452882_i128 as f32;
_3 = _8 + _9;
_5 = -_4;
_8 = _5;
_3 = _1;
_12 = _3 as f64;
_6 = _8;
_11.1 = -20815_i16;
_13 = 124127330790490594241257278869574307375_i128 as u8;
match RET.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463460131819573526536348 => bb6,
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
_7 = 2_usize as f32;
_3 = _8;
_5 = 2664709004_u32 as f32;
RET = ((-5230879736907342595_i64),);
_11.3 = _8 as f64;
_6 = _1 + _8;
_13 = true as u8;
_2 = 15780_u16;
RET = (681938446606134098_i64,);
_14.0 = _6 as i16;
RET = (9079219230671071973_i64,);
_15 = _14.0;
_10 = _1;
_11.2 = !1075323150_i32;
_2 = 28066_u16 * 42720_u16;
Goto(bb7)
}
bb7 = {
_12 = _11.3;
_13 = !222_u8;
_14.1 = _1 - _6;
_14 = (_15, _3);
_11.3 = RET.0 as f64;
_1 = (-9223372036854775808_isize) as f32;
_8 = _6;
_15 = _14.0 * _14.0;
_2 = 38274_u16;
_12 = _11.3 * _11.3;
RET = (4409289311945472645_i64,);
_14.0 = _15 - _15;
RET = ((-3859998496595529388_i64),);
_16 = _2 ^ _2;
_9 = _10;
match RET.0 {
0 => bb8,
340282366920938463459514608935172682068 => bb10,
_ => bb9
}
}
bb8 = {
_7 = 2_usize as f32;
_3 = _8;
_5 = 2664709004_u32 as f32;
RET = ((-5230879736907342595_i64),);
_11.3 = _8 as f64;
_6 = _1 + _8;
_13 = true as u8;
_2 = 15780_u16;
RET = (681938446606134098_i64,);
_14.0 = _6 as i16;
RET = (9079219230671071973_i64,);
_15 = _14.0;
_10 = _1;
_11.2 = !1075323150_i32;
_2 = 28066_u16 * 42720_u16;
Goto(bb7)
}
bb9 = {
Return()
}
bb10 = {
_3 = -_8;
match RET.0 {
0 => bb6,
1 => bb5,
2 => bb11,
3 => bb12,
4 => bb13,
340282366920938463459514608935172682068 => bb15,
_ => bb14
}
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_12 = _11.3;
_13 = !222_u8;
_14.1 = _1 - _6;
_14 = (_15, _3);
_11.3 = RET.0 as f64;
_1 = (-9223372036854775808_isize) as f32;
_8 = _6;
_15 = _14.0 * _14.0;
_2 = 38274_u16;
_12 = _11.3 * _11.3;
RET = (4409289311945472645_i64,);
_14.0 = _15 - _15;
RET = ((-3859998496595529388_i64),);
_16 = _2 ^ _2;
_9 = _10;
match RET.0 {
0 => bb8,
340282366920938463459514608935172682068 => bb10,
_ => bb9
}
}
bb14 = {
_7 = 2_usize as f32;
_3 = _8;
_5 = 2664709004_u32 as f32;
RET = ((-5230879736907342595_i64),);
_11.3 = _8 as f64;
_6 = _1 + _8;
_13 = true as u8;
_2 = 15780_u16;
RET = (681938446606134098_i64,);
_14.0 = _6 as i16;
RET = (9079219230671071973_i64,);
_15 = _14.0;
_10 = _1;
_11.2 = !1075323150_i32;
_2 = 28066_u16 * 42720_u16;
Goto(bb7)
}
bb15 = {
Goto(bb16)
}
bb16 = {
Call(_19 = dump_var(7_usize, 2_usize, Move(_2), 13_usize, Move(_13), 20_usize, _20, 20_usize, _20), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: u16,mut _2: f32,mut _3: f32,mut _4: f32,mut _5: f32,mut _6: f32,mut _7: i64,mut _8: f32,mut _9: f32,mut _10: f32,mut _11: (i64,)) -> f32 {
mir! {
type RET = f32;
let _12: Adt51;
let _13: Adt40;
let _14: ();
let _15: ();
{
_4 = -_10;
RET = 172_u8 as f32;
_11 = (_7,);
_6 = -_8;
_8 = -_2;
RET = 109134164222265143740137866963335769766_u128 as f32;
_9 = -_6;
_10 = -_6;
Call(_1 = fn9(_6, _3, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = 2331564010_u32 as f32;
_9 = _4;
_2 = _4;
_10 = 95617224141622081686808410506057968587_u128 as f32;
RET = -_5;
_11.0 = (-157920015572941445373461458523965413624_i128) as i64;
_3 = _6;
RET = _7 as f32;
RET = _6 + _8;
_9 = -_5;
_5 = -RET;
_11.0 = 2401595298_u32 as i64;
RET = _6 * _2;
RET = _3;
RET = _5 - _4;
_7 = _11.0 + _11.0;
_9 = -_6;
_9 = -_5;
_10 = -RET;
Goto(bb2)
}
bb2 = {
Call(_14 = dump_var(8_usize, 11_usize, Move(_11), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: f32,mut _2: f32,mut _3: f32) -> u16 {
mir! {
type RET = u16;
let _4: Adt50;
let _5: [i8; 8];
let _6: i128;
let _7: *const u8;
let _8: bool;
let _9: [isize; 4];
let _10: u64;
let _11: bool;
let _12: [u8; 3];
let _13: u8;
let _14: u16;
let _15: Adt51;
let _16: Adt52;
let _17: f64;
let _18: (u16,);
let _19: (i16, f32);
let _20: f32;
let _21: char;
let _22: f32;
let _23: f64;
let _24: char;
let _25: char;
let _26: f32;
let _27: ();
let _28: ();
{
_1 = (-1443973942_i32) as f32;
RET = 50863_u16;
_2 = _3;
_1 = -_2;
RET = !49608_u16;
RET = (-5203096254202507378_i64) as u16;
_3 = 233_u8 as f32;
_2 = -_1;
_1 = 3347832258_u32 as f32;
_2 = _3;
_3 = _2 - _1;
RET = !11693_u16;
_1 = 184015232120245939675016971641310151503_u128 as f32;
_3 = -_2;
RET = !37236_u16;
_1 = 103243121767496253641431352602119039545_i128 as f32;
RET = 8840_u16;
_3 = _1 - _2;
_5 = [(-27_i8),(-75_i8),(-1_i8),(-19_i8),(-25_i8),(-47_i8),(-14_i8),62_i8];
_3 = 25182_i16 as f32;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
8840 => bb9,
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
_3 = -_1;
_6 = '\u{e20ed}' as i128;
_1 = -_3;
_2 = 2142242412476581605_u64 as f32;
_6 = 1200252431_u32 as i128;
_3 = -_2;
RET = !38147_u16;
RET = 56987_u16 + 26468_u16;
_2 = _1;
RET = _3 as u16;
_5 = [71_i8,109_i8,46_i8,49_i8,(-40_i8),(-124_i8),(-49_i8),3_i8];
_6 = 120_u8 as i128;
_5 = [47_i8,43_i8,72_i8,(-102_i8),(-10_i8),50_i8,(-65_i8),(-47_i8)];
_5 = [(-100_i8),(-5_i8),(-117_i8),6_i8,(-46_i8),(-29_i8),(-114_i8),118_i8];
_3 = (-7428073400261702457_i64) as f32;
Goto(bb10)
}
bb10 = {
_5 = [(-31_i8),96_i8,11_i8,(-119_i8),(-53_i8),(-38_i8),(-52_i8),(-111_i8)];
_8 = false;
_9 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_11 = !_8;
_6 = (-120_i8) as i128;
_6 = 140188983631125146848038550382710816668_i128 | (-29741151098049651524333989177170256012_i128);
RET = !16783_u16;
_2 = -_3;
_14 = !RET;
_11 = _8;
_1 = 2413118239_u32 as f32;
_13 = 149_u8;
_3 = _2;
RET = 6_usize as u16;
_13 = !56_u8;
RET = !_14;
_8 = _11;
_8 = !_11;
Goto(bb11)
}
bb11 = {
_2 = _1 * _1;
_5 = [111_i8,97_i8,(-93_i8),0_i8,(-15_i8),(-110_i8),75_i8,74_i8];
_10 = !3951669827778914938_u64;
_10 = 271130266065090318407478568682353339991_u128 as u64;
_7 = core::ptr::addr_of!(_13);
_13 = 18786_i16 as u8;
_6 = (-147049065660804511486656372803182755566_i128) | (-85924880740304902172634891011060648938_i128);
_6 = 1107886948707195584_usize as i128;
_1 = _2;
(*_7) = !154_u8;
RET = _14 | _14;
_9 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-122_isize)];
RET = _14 + _14;
Goto(bb12)
}
bb12 = {
_18 = (_14,);
_6 = 123511072914322534879502728851860561730_i128 | (-8010277837302144653640177900595464668_i128);
_12 = [_13,(*_7),(*_7)];
_19.0 = (-30181_i16);
_18 = (RET,);
_9 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_22 = (-9223372036854775808_isize) as f32;
_2 = _13 as f32;
(*_7) = 92_u8;
_7 = core::ptr::addr_of!((*_7));
_9 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_10 = (-1489875778013636940_i64) as u64;
_17 = (-108_i8) as f64;
Goto(bb13)
}
bb13 = {
_19.0 = 27627_i16 ^ (-746_i16);
(*_7) = 132_u8 + 160_u8;
_19.1 = _1;
_11 = _3 <= _22;
_11 = !_8;
_5 = [(-46_i8),51_i8,55_i8,(-75_i8),(-95_i8),(-124_i8),31_i8,(-119_i8)];
_19.1 = _18.0 as f32;
_21 = '\u{3f17a}';
_2 = 68156987081057377315523087675154822831_u128 as f32;
_19.1 = -_1;
RET = 200838481942897246417157109759380911042_u128 as u16;
_18.0 = RET;
(*_7) = 72_u8 >> _6;
(*_7) = 77_u8 + 135_u8;
_7 = core::ptr::addr_of!((*_7));
_7 = core::ptr::addr_of!((*_7));
_11 = _2 < _3;
_9 = [(-105_isize),9223372036854775807_isize,9223372036854775807_isize,(-114_isize)];
_7 = core::ptr::addr_of!((*_7));
_20 = _19.1;
_22 = _20 * _20;
_19 = (23929_i16, _22);
_18 = (_14,);
_6 = (-89411910027240858053883432071860525461_i128);
_2 = -_20;
Goto(bb14)
}
bb14 = {
RET = !_14;
_25 = _21;
(*_7) = 216_u8;
_1 = _22;
_12 = [_13,_13,_13];
_9 = [80_isize,13_isize,(-3_isize),(-9223372036854775808_isize)];
_22 = -_1;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(9_usize, 5_usize, Move(_5), 10_usize, Move(_10), 6_usize, Move(_6), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(9_usize, 18_usize, Move(_18), 25_usize, Move(_25), 28_usize, _28, 28_usize, _28), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: (u128, [isize; 4], (i16, f32)),mut _2: isize) -> f32 {
mir! {
type RET = f32;
let _3: bool;
let _4: ();
let _5: ();
{
_1.1 = [_2,_2,_2,_2];
Goto(bb1)
}
bb1 = {
RET = _1.2.1;
_1.2 = (6448_i16, RET);
_1.2 = ((-28141_i16), RET);
_2 = !(-9223372036854775808_isize);
_1.2 = (19510_i16, RET);
_1.2 = ((-13439_i16), RET);
_1.0 = 168632573295866114290876528327507920277_u128 | 205840642624916850163331524390405229423_u128;
RET = -_1.2.1;
RET = (-904410795931555473_i64) as f32;
RET = -_1.2.1;
_1.2 = ((-24500_i16), RET);
_1.2.0 = (-13121_i16);
_1.2.0 = (-20673_i16);
Goto(bb2)
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: *mut isize,mut _2: (u16,),mut _3: isize,mut _4: isize,mut _5: u8,mut _6: *mut isize,mut _7: isize,mut _8: f32,mut _9: isize,mut _10: isize,mut _11: i16,mut _12: i16,mut _13: f32,mut _14: (i64,)) -> f32 {
mir! {
type RET = f32;
let _15: [i8; 8];
let _16: &'static bool;
let _17: char;
let _18: usize;
let _19: (u128, [isize; 4], (i16, f32));
let _20: u64;
let _21: Adt39;
let _22: Adt48;
let _23: ();
let _24: ();
{
_6 = core::ptr::addr_of_mut!(_9);
_12 = _11 & _11;
_15 = [(-66_i8),(-71_i8),90_i8,94_i8,6_i8,5_i8,77_i8,(-10_i8)];
(*_6) = _7 - _4;
_2.0 = !33203_u16;
_14.0 = -(-7716814400223745759_i64);
_3 = _9;
(*_6) = _3;
_10 = !(*_6);
_9 = !_3;
_2 = (65525_u16,);
_14.0 = (-7013087422050311872_i64) + 2458490916029130237_i64;
_14.0 = !832365495398928944_i64;
_15 = [104_i8,(-3_i8),(-79_i8),(-10_i8),40_i8,104_i8,(-66_i8),36_i8];
_11 = _12 ^ _12;
Goto(bb1)
}
bb1 = {
_10 = !(*_6);
(*_6) = -_3;
_2 = (19205_u16,);
_14 = ((-3371095221253254054_i64),);
_4 = (*_6);
_11 = true as i16;
_4 = (*_6) << (*_6);
match _14.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463460003512210514957402 => bb10,
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
_1 = _6;
match _2.0 {
0 => bb1,
1 => bb9,
2 => bb5,
3 => bb4,
4 => bb11,
5 => bb12,
6 => bb13,
19205 => bb15,
_ => bb14
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
_10 = !(*_6);
(*_6) = -_3;
_2 = (19205_u16,);
_14 = ((-3371095221253254054_i64),);
_4 = (*_6);
_11 = true as i16;
_4 = (*_6) << (*_6);
match _14.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463460003512210514957402 => bb10,
_ => bb9
}
}
bb15 = {
RET = -_13;
(*_6) = -_7;
RET = _8;
_8 = -_13;
RET = _13;
_18 = 5_usize - 2_usize;
_8 = -_13;
_13 = _8 + _8;
_1 = _6;
(*_1) = _10 ^ _10;
_19.2 = (_12, _13);
_4 = (*_6);
_19.1 = [_9,_9,_10,(*_1)];
(*_6) = _5 as isize;
_10 = _7;
_19.2 = (_12, RET);
(*_1) = _4 | _3;
_17 = '\u{ac4a9}';
_7 = _4 + (*_1);
_5 = 236_u8 * 0_u8;
_15 = [24_i8,(-32_i8),123_i8,51_i8,5_i8,117_i8,(-125_i8),24_i8];
_8 = _13;
_6 = core::ptr::addr_of_mut!((*_6));
_19.0 = (-23_i8) as u128;
_14.0 = -(-1748787002308147961_i64);
Goto(bb16)
}
bb16 = {
Call(_23 = dump_var(11_usize, 18_usize, Move(_18), 11_usize, Move(_11), 4_usize, Move(_4), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_23 = dump_var(11_usize, 14_usize, Move(_14), 17_usize, Move(_17), 24_usize, _24, 24_usize, _24), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: u16,mut _2: isize,mut _3: *mut isize) -> *mut u64 {
mir! {
type RET = *mut u64;
let _4: u128;
let _5: (i64,);
let _6: i128;
let _7: i32;
let _8: f64;
let _9: Adt52;
let _10: Adt50;
let _11: u16;
let _12: isize;
let _13: f32;
let _14: (i64,);
let _15: Adt49;
let _16: [u8; 3];
let _17: isize;
let _18: f32;
let _19: u8;
let _20: [bool; 7];
let _21: Adt51;
let _22: u32;
let _23: [isize; 4];
let _24: Adt41;
let _25: (char, i16);
let _26: char;
let _27: *mut u64;
let _28: f64;
let _29: bool;
let _30: isize;
let _31: i16;
let _32: u32;
let _33: Adt50;
let _34: isize;
let _35: [bool; 7];
let _36: bool;
let _37: u64;
let _38: isize;
let _39: u128;
let _40: i8;
let _41: *mut u64;
let _42: isize;
let _43: [isize; 4];
let _44: f32;
let _45: *mut u64;
let _46: u64;
let _47: (u16,);
let _48: Adt41;
let _49: ();
let _50: ();
{
_2 = (*_3);
_6 = -(-129150860125213916327033386264693094537_i128);
_5.0 = !6410779552127299366_i64;
_6 = !(-154888573401241665542678780604469280188_i128);
_5 = ((-6683005187396147941_i64),);
(*_3) = _2;
(*_3) = !_2;
_2 = (-1984123859_i32) as isize;
_1 = 1928_i16 as u16;
_5.0 = 92_u8 as i64;
_5 = (8281320220545054777_i64,);
_5.0 = (-2363954646381841590_i64) + (-639004787095330686_i64);
_5 = ((-9101830531557646871_i64),);
Goto(bb1)
}
bb1 = {
_8 = 164_u8 as f64;
Goto(bb2)
}
bb2 = {
_7 = -(-342669859_i32);
(*_3) = _2;
_7 = 165291515854500480851424058195160471938_u128 as i32;
_1 = 125691239640522536599333347066961719880_u128 as u16;
_8 = (*_3) as f64;
_7 = (-934080941_i32);
_5 = (8506885196479251093_i64,);
_8 = 50_i8 as f64;
_1 = 22005_u16 ^ 48403_u16;
_2 = 169_u8 as isize;
_8 = 1755003510_u32 as f64;
_6 = 5356790367419590757_u64 as i128;
_5 = ((-5165630804364696649_i64),);
Call(_4 = fn13(_2, _1, (*_3), _7, _3, _2, (*_3), (*_3), _7, _1, _5, (*_3), _5.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_3) = !_2;
(*_3) = _2;
_5 = ((-6643994793825033476_i64),);
_5.0 = (-5552705120267690760_i64);
_2 = (*_3) ^ (*_3);
_4 = !5496919522864213441138775201894135963_u128;
_2 = (*_3);
_2 = (*_3) & (*_3);
_5.0 = false as i64;
_4 = (*_3) as u128;
_8 = 61_u8 as f64;
(*_3) = !_2;
_7 = (-1710246157_i32);
(*_3) = !_2;
_4 = 262629920921123485332285525224511738102_u128;
_1 = 15830_u16;
(*_3) = _2;
Goto(bb4)
}
bb4 = {
(*_3) = _2 >> _1;
_11 = _1;
_8 = 124_u8 as f64;
_2 = !(*_3);
(*_3) = 6761788415140308877_usize as isize;
_11 = _1 & _1;
_7 = (-472143636_i32) * (-1788865641_i32);
_13 = _5.0 as f32;
(*_3) = 16642263016666562265_u64 as isize;
_2 = (*_3) ^ (*_3);
_11 = _1 >> _5.0;
_14.0 = -_5.0;
_12 = _4 as isize;
_8 = _4 as f64;
match _1 {
0 => bb5,
1 => bb6,
15830 => bb8,
_ => bb7
}
}
bb5 = {
(*_3) = !_2;
(*_3) = _2;
_5 = ((-6643994793825033476_i64),);
_5.0 = (-5552705120267690760_i64);
_2 = (*_3) ^ (*_3);
_4 = !5496919522864213441138775201894135963_u128;
_2 = (*_3);
_2 = (*_3) & (*_3);
_5.0 = false as i64;
_4 = (*_3) as u128;
_8 = 61_u8 as f64;
(*_3) = !_2;
_7 = (-1710246157_i32);
(*_3) = !_2;
_4 = 262629920921123485332285525224511738102_u128;
_1 = 15830_u16;
(*_3) = _2;
Goto(bb4)
}
bb6 = {
_7 = -(-342669859_i32);
(*_3) = _2;
_7 = 165291515854500480851424058195160471938_u128 as i32;
_1 = 125691239640522536599333347066961719880_u128 as u16;
_8 = (*_3) as f64;
_7 = (-934080941_i32);
_5 = (8506885196479251093_i64,);
_8 = 50_i8 as f64;
_1 = 22005_u16 ^ 48403_u16;
_2 = 169_u8 as isize;
_8 = 1755003510_u32 as f64;
_6 = 5356790367419590757_u64 as i128;
_5 = ((-5165630804364696649_i64),);
Call(_4 = fn13(_2, _1, (*_3), _7, _3, _2, (*_3), (*_3), _7, _1, _5, (*_3), _5.0), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_8 = 164_u8 as f64;
Goto(bb2)
}
bb8 = {
_5 = (_14.0,);
_5.0 = _14.0 - _14.0;
_4 = !231598319167870522780757067958732853471_u128;
_5.0 = _14.0 ^ _14.0;
_12 = (*_3) << _11;
_16 = [230_u8,164_u8,141_u8];
_11 = _1 * _1;
_2 = (*_3);
_13 = _2 as f32;
_5.0 = _14.0 << _11;
_14.0 = _5.0 | _5.0;
_8 = 5_u8 as f64;
_14 = (_5.0,);
_12 = _8 as isize;
Goto(bb9)
}
bb9 = {
(*_3) = !_2;
_11 = _1;
(*_3) = _12 & _12;
_11 = _1;
_5 = (_14.0,);
_13 = 7606985654513872452_usize as f32;
_11 = !_1;
(*_3) = _2;
_16 = [195_u8,185_u8,207_u8];
(*_3) = _2 | _12;
_17 = _11 as isize;
_4 = 170500688022941136523994412346507377635_u128;
_19 = !92_u8;
_7 = !1777080377_i32;
_18 = _13 - _13;
_19 = !235_u8;
_4 = 159437505647161986807725126005957147785_u128 & 191046856023194754665047900617936622153_u128;
_8 = 342110090_u32 as f64;
_5.0 = -_14.0;
Goto(bb10)
}
bb10 = {
_7 = 1205287131_i32 * (-465304328_i32);
_13 = _18 + _18;
_13 = _18;
_11 = _1 + _1;
_7 = !(-1182415762_i32);
_16 = [_19,_19,_19];
_7 = _4 as i32;
_1 = _19 as u16;
_5 = (_14.0,);
_5.0 = _14.0;
_13 = -_18;
_20 = [true,true,false,true,false,false,false];
_5 = (_14.0,);
_22 = 2761288265_u32 ^ 2955248751_u32;
_4 = 202881095792824818241163256181420813952_u128 | 191921702249013278446508669711348550000_u128;
_2 = (-1245_i16) as isize;
_17 = _12;
_11 = !_1;
_19 = 10_u8;
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = !_12;
_13 = _6 as f32;
_3 = core::ptr::addr_of_mut!(_12);
_25 = ('\u{24164}', (-864_i16));
_20 = [true,true,true,false,false,false,false];
_24.fld0 = -_8;
Goto(bb11)
}
bb11 = {
_23 = [(*_3),(*_3),_12,(*_3)];
_19 = _14.0 as u8;
_1 = _11;
_3 = core::ptr::addr_of_mut!((*_3));
_24 = Adt41 { fld0: _8,fld1: 4_usize };
_6 = -34897038736724483467965586894331031422_i128;
match _24.fld1 {
0 => bb5,
4 => bb12,
_ => bb9
}
}
bb12 = {
match _24.fld1 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb4,
5 => bb6,
6 => bb10,
4 => bb14,
_ => bb13
}
}
bb13 = {
(*_3) = !_2;
(*_3) = _2;
_5 = ((-6643994793825033476_i64),);
_5.0 = (-5552705120267690760_i64);
_2 = (*_3) ^ (*_3);
_4 = !5496919522864213441138775201894135963_u128;
_2 = (*_3);
_2 = (*_3) & (*_3);
_5.0 = false as i64;
_4 = (*_3) as u128;
_8 = 61_u8 as f64;
(*_3) = !_2;
_7 = (-1710246157_i32);
(*_3) = !_2;
_4 = 262629920921123485332285525224511738102_u128;
_1 = 15830_u16;
(*_3) = _2;
Goto(bb4)
}
bb14 = {
(*_3) = -_17;
_2 = (*_3);
_2 = _12;
_26 = _25.0;
_20 = [false,true,true,false,true,false,false];
_17 = _2;
_14 = (_5.0,);
_31 = _25.1;
_31 = _25.1 >> _25.1;
_18 = _13;
_17 = _12 & _12;
_2 = _1 as isize;
_3 = core::ptr::addr_of_mut!(_12);
Goto(bb15)
}
bb15 = {
_29 = !true;
_25.1 = _31;
_20 = [_29,_29,_29,_29,_29,_29,_29];
_6 = !(-95046774280061669788397244422064578109_i128);
_31 = _4 as i16;
_11 = _1;
_16 = [_19,_19,_19];
_24 = Adt41 { fld0: _8,fld1: 9150873897175494994_usize };
_29 = false;
Call((*_3) = core::intrinsics::transmute(_5.0), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_6 = (-118197170408075002618793789629604762454_i128);
_14.0 = _5.0 - _5.0;
_5.0 = 1854839932735614545_u64 as i64;
_25.0 = _26;
_24.fld0 = _8;
_8 = -_24.fld0;
_14 = (_5.0,);
_12 = _17;
_6 = _29 as i128;
_8 = _22 as f64;
_8 = -_24.fld0;
_17 = (*_3) ^ (*_3);
_32 = _14.0 as u32;
_30 = (*_3) + (*_3);
_12 = _17 >> _1;
_2 = -_12;
_1 = _11;
_25.0 = _26;
match _24.fld1 {
0 => bb8,
1 => bb2,
2 => bb6,
3 => bb17,
9150873897175494994 => bb19,
_ => bb18
}
}
bb17 = {
_29 = !true;
_25.1 = _31;
_20 = [_29,_29,_29,_29,_29,_29,_29];
_6 = !(-95046774280061669788397244422064578109_i128);
_31 = _4 as i16;
_11 = _1;
_16 = [_19,_19,_19];
_24 = Adt41 { fld0: _8,fld1: 9150873897175494994_usize };
_29 = false;
Call((*_3) = core::intrinsics::transmute(_5.0), ReturnTo(bb16), UnwindUnreachable())
}
bb18 = {
(*_3) = !_2;
(*_3) = _2;
_5 = ((-6643994793825033476_i64),);
_5.0 = (-5552705120267690760_i64);
_2 = (*_3) ^ (*_3);
_4 = !5496919522864213441138775201894135963_u128;
_2 = (*_3);
_2 = (*_3) & (*_3);
_5.0 = false as i64;
_4 = (*_3) as u128;
_8 = 61_u8 as f64;
(*_3) = !_2;
_7 = (-1710246157_i32);
(*_3) = !_2;
_4 = 262629920921123485332285525224511738102_u128;
_1 = 15830_u16;
(*_3) = _2;
Goto(bb4)
}
bb19 = {
_22 = _32 * _32;
_2 = _4 as isize;
_2 = _30 ^ (*_3);
_19 = 68_u8 >> _2;
_19 = !120_u8;
_13 = _4 as f32;
_25.0 = _26;
_5.0 = !_14.0;
_25 = (_26, _31);
_32 = !_22;
_35 = [_29,_29,_29,_29,_29,_29,_29];
_17 = _32 as isize;
Goto(bb20)
}
bb20 = {
RET = core::ptr::addr_of_mut!(_37);
_36 = _25.1 == _25.1;
_24 = Adt41 { fld0: _8,fld1: 0_usize };
_28 = _24.fld0;
_5.0 = -_14.0;
(*RET) = _26 as u64;
_1 = _11 * _11;
_38 = _2 * (*_3);
_12 = !_30;
_40 = 27_i8 * (-9_i8);
_4 = !205798103732527208486863024140922564669_u128;
_18 = -_13;
_27 = RET;
_44 = -_13;
_31 = _4 as i16;
_36 = !_29;
_28 = (*RET) as f64;
_11 = _1 + _1;
(*_3) = _38 + _2;
_5 = (_14.0,);
_3 = core::ptr::addr_of_mut!(_30);
_25.0 = _26;
Goto(bb21)
}
bb21 = {
Call(_49 = dump_var(12_usize, 32_usize, Move(_32), 12_usize, Move(_12), 11_usize, Move(_11), 35_usize, Move(_35)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_49 = dump_var(12_usize, 26_usize, Move(_26), 5_usize, Move(_5), 16_usize, Move(_16), 36_usize, Move(_36)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_49 = dump_var(12_usize, 29_usize, Move(_29), 20_usize, Move(_20), 30_usize, Move(_30), 19_usize, Move(_19)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_49 = dump_var(12_usize, 31_usize, Move(_31), 50_usize, _50, 50_usize, _50, 50_usize, _50), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: isize,mut _2: u16,mut _3: isize,mut _4: i32,mut _5: *mut isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: i32,mut _10: u16,mut _11: (i64,),mut _12: isize,mut _13: i64) -> u128 {
mir! {
type RET = u128;
let _14: isize;
let _15: u32;
let _16: [bool; 7];
let _17: (u128, [isize; 4], (i16, f32));
let _18: Adt37;
let _19: Adt45;
let _20: [bool; 7];
let _21: i32;
let _22: (i16, f32);
let _23: (char, i16);
let _24: u8;
let _25: (i16, f32);
let _26: Adt47;
let _27: Adt36;
let _28: &'static bool;
let _29: (i64,);
let _30: [i8; 8];
let _31: u128;
let _32: Adt46;
let _33: isize;
let _34: (*const u8, [bool; 7]);
let _35: *mut [bool; 7];
let _36: [u8; 3];
let _37: i8;
let _38: isize;
let _39: isize;
let _40: f32;
let _41: [u8; 3];
let _42: ();
let _43: ();
{
RET = 139318096476608829419794410454493208066_u128 >> _11.0;
_6 = -_7;
(*_5) = _7;
_1 = !(*_5);
_7 = _8 - _1;
_3 = 800494605274251359_u64 as isize;
_11.0 = RET as i64;
_11 = (_13,);
(*_5) = _1;
_11.0 = _13;
(*_5) = _8 & _12;
(*_5) = _12 - _3;
_8 = _1 | _3;
(*_5) = _8;
_13 = !_11.0;
_15 = '\u{c0525}' as u32;
_6 = (*_5) + (*_5);
_5 = core::ptr::addr_of_mut!(_12);
_2 = (-31808_i16) as u16;
_16 = [true,true,true,false,false,false,true];
_8 = _9 as isize;
(*_5) = _6 >> _8;
_11 = (_13,);
_17.2.0 = (-13057_i16);
_3 = RET as isize;
(*_5) = _3;
_2 = _10 << _6;
Goto(bb1)
}
bb1 = {
_13 = _11.0;
_2 = _10;
_6 = 61850250452364350079205067529209624247_i128 as isize;
_15 = 2560547491_u32 | 3291520789_u32;
(*_5) = -_3;
_17.2.1 = RET as f32;
_3 = _7 - (*_5);
_1 = _3;
_13 = _11.0 - _11.0;
_14 = RET as isize;
_6 = (*_5) >> (*_5);
_17.1 = [_12,(*_5),_3,_7];
_7 = _6 + _1;
_12 = 9819950216610517272_u64 as isize;
_17.1 = [_7,_7,_6,_6];
_10 = (-46_i8) as u16;
(*_5) = 13_u8 as isize;
Call(_10 = core::intrinsics::transmute(_17.2.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_17.2.1 = (-33072127192372178531484243450495102551_i128) as f32;
Call(_17.2.1 = fn14((*_5), _17.1, _7, _4, _12, _17.1, _12, _2, _15, _6, _11.0, _14, _5, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_19.fld5 = (RET, _17.1, _17.2);
_6 = _17.2.0 as isize;
_1 = (-9940376585480058906294789900322998208_i128) as isize;
_4 = !_9;
_19.fld5 = (RET, _17.1, _17.2);
_17 = (_19.fld5.0, _19.fld5.1, _19.fld5.2);
(*_5) = _1 & _3;
RET = _19.fld5.0 + _19.fld5.0;
_8 = _11.0 as isize;
_9 = _4 + _4;
_20 = _16;
Goto(bb4)
}
bb4 = {
_9 = _15 as i32;
_19.fld2 = (-122981921706182180532511380448559939789_i128);
_11.0 = -_13;
_22.1 = _6 as f32;
_19.fld5.2 = (_17.2.0, _22.1);
_22 = _17.2;
_20 = [false,false,false,true,true,false,false];
_19.fld5.2 = (_17.2.0, _17.2.1);
_19.fld5.2.0 = 253_u8 as i16;
_23.0 = '\u{708c9}';
_19.fld0.1 = _16;
match _19.fld2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
217300445214756282930863226983208271667 => bb7,
_ => bb6
}
}
bb5 = {
_19.fld5 = (RET, _17.1, _17.2);
_6 = _17.2.0 as isize;
_1 = (-9940376585480058906294789900322998208_i128) as isize;
_4 = !_9;
_19.fld5 = (RET, _17.1, _17.2);
_17 = (_19.fld5.0, _19.fld5.1, _19.fld5.2);
(*_5) = _1 & _3;
RET = _19.fld5.0 + _19.fld5.0;
_8 = _11.0 as isize;
_9 = _4 + _4;
_20 = _16;
Goto(bb4)
}
bb6 = {
_13 = _11.0;
_2 = _10;
_6 = 61850250452364350079205067529209624247_i128 as isize;
_15 = 2560547491_u32 | 3291520789_u32;
(*_5) = -_3;
_17.2.1 = RET as f32;
_3 = _7 - (*_5);
_1 = _3;
_13 = _11.0 - _11.0;
_14 = RET as isize;
_6 = (*_5) >> (*_5);
_17.1 = [_12,(*_5),_3,_7];
_7 = _6 + _1;
_12 = 9819950216610517272_u64 as isize;
_17.1 = [_7,_7,_6,_6];
_10 = (-46_i8) as u16;
(*_5) = 13_u8 as isize;
Call(_10 = core::intrinsics::transmute(_17.2.0), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_13 = !_11.0;
_14 = _13 as isize;
(*_5) = !_3;
_19.fld5.0 = !_17.0;
RET = 2_usize as u128;
_20 = _16;
_8 = _7;
_17.0 = _19.fld5.0;
_26.fld2.fld1.0 = core::ptr::addr_of_mut!(_12);
_19.fld5.0 = !_17.0;
_17.2 = _19.fld5.2;
_26.fld2.fld1.2 = _9;
_12 = _14;
match _22.0 {
340282366920938463463374607431768198399 => bb9,
_ => bb8
}
}
bb8 = {
_9 = _15 as i32;
_19.fld2 = (-122981921706182180532511380448559939789_i128);
_11.0 = -_13;
_22.1 = _6 as f32;
_19.fld5.2 = (_17.2.0, _22.1);
_22 = _17.2;
_20 = [false,false,false,true,true,false,false];
_19.fld5.2 = (_17.2.0, _17.2.1);
_19.fld5.2.0 = 253_u8 as i16;
_23.0 = '\u{708c9}';
_19.fld0.1 = _16;
match _19.fld2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
217300445214756282930863226983208271667 => bb7,
_ => bb6
}
}
bb9 = {
_26.fld2.fld1.0 = core::ptr::addr_of_mut!(_6);
_9 = _4 << _12;
_14 = -_8;
(*_5) = 45_u8 as isize;
_19.fld2 = -(-93123302706992442572189356026903891775_i128);
_17.2.0 = _19.fld5.2.0 - _22.0;
_19.fld5.0 = _17.0 + _17.0;
_19.fld0.1 = _16;
_24 = 5965997068499587186_u64 as u8;
match _22.0 {
0 => bb8,
340282366920938463463374607431768198399 => bb11,
_ => bb10
}
}
bb10 = {
_19.fld5 = (RET, _17.1, _17.2);
_6 = _17.2.0 as isize;
_1 = (-9940376585480058906294789900322998208_i128) as isize;
_4 = !_9;
_19.fld5 = (RET, _17.1, _17.2);
_17 = (_19.fld5.0, _19.fld5.1, _19.fld5.2);
(*_5) = _1 & _3;
RET = _19.fld5.0 + _19.fld5.0;
_8 = _11.0 as isize;
_9 = _4 + _4;
_20 = _16;
Goto(bb4)
}
bb11 = {
_14 = _6 << _22.0;
_26.fld2.fld2 = [115_i8,104_i8,124_i8,68_i8,(-120_i8),(-32_i8),28_i8,(-96_i8)];
_13 = _11.0 + _11.0;
_9 = false as i32;
_30 = [(-35_i8),69_i8,31_i8,47_i8,88_i8,14_i8,18_i8,(-37_i8)];
_27.fld0 = _10 as i16;
_22 = (_17.2.0, _17.2.1);
_33 = _8 ^ _3;
_22.1 = _9 as f32;
_17 = _19.fld5;
_25.1 = -_19.fld5.2.1;
_12 = -_7;
_26.fld2.fld1.2 = _9 & _9;
_33 = (*_5);
_26.fld0 = (_2,);
_19.fld5.0 = !_17.0;
_19.fld5.2.0 = _24 as i16;
_26.fld2.fld1.2 = _4;
_19.fld5.0 = !_17.0;
_25 = (_22.0, _19.fld5.2.1);
Goto(bb12)
}
bb12 = {
_25.1 = _19.fld5.2.1 - _17.2.1;
RET = !_19.fld5.0;
_26.fld1 = -_25.1;
_23 = ('\u{c433e}', _27.fld0);
_21 = _9 + _9;
_24 = !210_u8;
_12 = -_7;
_27.fld1 = _23.0;
_26.fld2.fld3 = core::ptr::addr_of_mut!(_34.0);
_26.fld0 = (_2,);
_34.0 = core::ptr::addr_of!(_24);
_26.fld2.fld1.1 = !_27.fld0;
_12 = _22.1 as isize;
_19.fld0.0 = core::ptr::addr_of!(_24);
_35 = core::ptr::addr_of_mut!(_16);
_36 = [_24,_24,_24];
_26.fld0.0 = _2;
_34 = (_19.fld0.0, _19.fld0.1);
_19.fld0 = (_34.0, (*_35));
_19.fld4 = [88_i8,88_i8,(-21_i8),(-115_i8),(-84_i8),58_i8,41_i8,(-41_i8)];
Goto(bb13)
}
bb13 = {
_17.0 = !RET;
_31 = _17.0;
RET = _31;
_27.fld0 = _26.fld2.fld1.1 + _26.fld2.fld1.1;
_11.0 = _13;
_15 = _11.0 as u32;
_17 = _19.fld5;
_11.0 = _27.fld1 as i64;
_26.fld2.fld3 = core::ptr::addr_of_mut!(_19.fld0.0);
_18 = Adt37::Variant0 { fld0: _24,fld1: _19.fld0.0,fld2: _7 };
(*_35) = [false,false,true,true,false,false,true];
_11 = (_13,);
SetDiscriminant(_18, 1);
_19.fld1 = [_24,_24,_24];
(*_35) = [true,true,false,false,true,false,true];
_39 = _26.fld1 as isize;
place!(Field::<[isize; 4]>(Variant(_18, 1), 2)) = [_7,_8,_14,_33];
place!(Field::<*mut *const u8>(Variant(_18, 1), 3)) = core::ptr::addr_of_mut!(_19.fld0.0);
RET = !_17.0;
_19.fld5.1 = _17.1;
_9 = _26.fld2.fld1.2;
_39 = _14 & _7;
(*_35) = [true,false,false,true,false,true,true];
_17.2 = _25;
_35 = core::ptr::addr_of_mut!(_20);
_17.1 = [_7,_33,_39,_3];
Goto(bb14)
}
bb14 = {
_19.fld0.0 = core::ptr::addr_of!(_24);
_19.fld5.0 = RET;
_11.0 = !_13;
_2 = _26.fld0.0 + _10;
_25.1 = _17.2.1 * _26.fld1;
_19.fld5.2 = (_22.0, _22.1);
_38 = _14 * _3;
_20 = _19.fld0.1;
_26.fld2.fld3 = core::ptr::addr_of_mut!(_34.0);
_17 = (_31, _19.fld5.1, _25);
_11.0 = -_13;
_26.fld0 = (_2,);
_22 = (_27.fld0, _17.2.1);
_26.fld0 = (_2,);
_34 = _19.fld0;
_19.fld4 = [(-101_i8),41_i8,93_i8,99_i8,(-83_i8),(-41_i8),(-78_i8),(-71_i8)];
_26.fld2.fld1.3 = _22.0 as f64;
_11.0 = _13 | _13;
_12 = -_8;
_23 = (_27.fld1, _27.fld0);
_31 = _17.0 & RET;
_23.1 = -_25.0;
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(13_usize, 7_usize, Move(_7), 36_usize, Move(_36), 9_usize, Move(_9), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(13_usize, 8_usize, Move(_8), 20_usize, Move(_20), 39_usize, Move(_39), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(13_usize, 31_usize, Move(_31), 33_usize, Move(_33), 16_usize, Move(_16), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: isize,mut _2: [isize; 4],mut _3: isize,mut _4: i32,mut _5: isize,mut _6: [isize; 4],mut _7: isize,mut _8: u16,mut _9: u32,mut _10: isize,mut _11: i64,mut _12: isize,mut _13: *mut isize,mut _14: *mut isize) -> f32 {
mir! {
type RET = f32;
let _15: Adt39;
let _16: u128;
let _17: i8;
let _18: (i64,);
let _19: isize;
let _20: char;
let _21: Adt45;
let _22: u8;
let _23: Adt51;
let _24: u16;
let _25: f32;
let _26: (i16, f32);
let _27: (*mut isize, i16, i32, f64);
let _28: [isize; 4];
let _29: char;
let _30: *const u8;
let _31: Adt46;
let _32: isize;
let _33: (i64,);
let _34: char;
let _35: ();
let _36: ();
{
RET = 162281845812121846610021309316412762080_i128 as f32;
_1 = -(*_13);
(*_13) = _4 as isize;
_6 = [_3,(*_14),_3,_3];
(*_14) = _10;
RET = 87_i8 as f32;
_4 = (-595945305_i32);
match _4 {
0 => bb1,
340282366920938463463374607431172266151 => bb3,
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
_3 = !_10;
_10 = 200614559838786352049261559504140527354_u128 as isize;
_7 = -(*_13);
RET = 227_u8 as f32;
(*_14) = _7 >> _1;
_10 = -(*_14);
_14 = _13;
_3 = !_7;
_2 = _6;
_18 = (_11,);
(*_13) = 30_u8 as isize;
_2 = _6;
_16 = 69868979416641988680372871040296706508_u128 + 318528757687052677647234181632182025160_u128;
_3 = !_10;
RET = 6_usize as f32;
_12 = _3;
_20 = '\u{9d1ac}';
_12 = -(*_13);
_6 = _2;
match _4 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463463374607431172266151 => bb10,
_ => bb9
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
Return()
}
bb10 = {
RET = 64979407534301500772012364902010109086_i128 as f32;
_1 = -_7;
_1 = _9 as isize;
_21.fld5.2 = (26507_i16, RET);
_21.fld5.2 = (14247_i16, RET);
_21.fld4 = [(-120_i8),(-72_i8),16_i8,(-117_i8),(-99_i8),(-29_i8),(-111_i8),11_i8];
_14 = _13;
_18 = (_11,);
_21.fld2 = 89537533860670149125928096624750029658_i128 - 134894073116627181095028398823369859133_i128;
_20 = '\u{162b7}';
_4 = (-3700951_i32);
_21.fld0.1 = [false,false,true,true,false,false,true];
_21.fld1 = [205_u8,189_u8,230_u8];
_9 = !1301895346_u32;
(*_14) = _7 ^ _5;
_11 = !_18.0;
_26.0 = _21.fld5.2.0;
_21.fld4 = [(-8_i8),97_i8,10_i8,0_i8,(-68_i8),3_i8,114_i8,(-90_i8)];
_21.fld0.0 = core::ptr::addr_of!(_22);
_21.fld4 = [120_i8,45_i8,29_i8,(-14_i8),13_i8,(-80_i8),(-81_i8),83_i8];
Goto(bb11)
}
bb11 = {
_21.fld4 = [(-89_i8),88_i8,(-26_i8),14_i8,66_i8,19_i8,(-92_i8),(-125_i8)];
(*_13) = _7;
_21.fld5.2.1 = RET;
_21.fld4 = [53_i8,(-86_i8),(-92_i8),(-123_i8),(-107_i8),19_i8,(-44_i8),14_i8];
_17 = -99_i8;
_21.fld5.2.0 = _20 as i16;
_26.1 = _21.fld5.2.1 * RET;
_21.fld5.1 = [_1,(*_14),(*_14),_7];
_24 = _11 as u16;
_27.2 = _4 << _5;
_21.fld0.1 = [false,true,false,true,true,false,false];
_8 = _24;
_22 = 129_u8 + 80_u8;
(*_14) = RET as isize;
_20 = '\u{1d727}';
_6 = _21.fld5.1;
(*_13) = _3 - _3;
_21.fld2 = _9 as i128;
_16 = !6879129203144789617065406592035180905_u128;
_19 = !_10;
_26.0 = _21.fld5.2.0 & _21.fld5.2.0;
match _4 {
0 => bb5,
1 => bb6,
2 => bb12,
3 => bb13,
4 => bb14,
340282366920938463463374607431764510505 => bb16,
_ => bb15
}
}
bb12 = {
RET = 64979407534301500772012364902010109086_i128 as f32;
_1 = -_7;
_1 = _9 as isize;
_21.fld5.2 = (26507_i16, RET);
_21.fld5.2 = (14247_i16, RET);
_21.fld4 = [(-120_i8),(-72_i8),16_i8,(-117_i8),(-99_i8),(-29_i8),(-111_i8),11_i8];
_14 = _13;
_18 = (_11,);
_21.fld2 = 89537533860670149125928096624750029658_i128 - 134894073116627181095028398823369859133_i128;
_20 = '\u{162b7}';
_4 = (-3700951_i32);
_21.fld0.1 = [false,false,true,true,false,false,true];
_21.fld1 = [205_u8,189_u8,230_u8];
_9 = !1301895346_u32;
(*_14) = _7 ^ _5;
_11 = !_18.0;
_26.0 = _21.fld5.2.0;
_21.fld4 = [(-8_i8),97_i8,10_i8,0_i8,(-68_i8),3_i8,114_i8,(-90_i8)];
_21.fld0.0 = core::ptr::addr_of!(_22);
_21.fld4 = [120_i8,45_i8,29_i8,(-14_i8),13_i8,(-80_i8),(-81_i8),83_i8];
Goto(bb11)
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_3 = !_10;
_10 = 200614559838786352049261559504140527354_u128 as isize;
_7 = -(*_13);
RET = 227_u8 as f32;
(*_14) = _7 >> _1;
_10 = -(*_14);
_14 = _13;
_3 = !_7;
_2 = _6;
_18 = (_11,);
(*_13) = 30_u8 as isize;
_2 = _6;
_16 = 69868979416641988680372871040296706508_u128 + 318528757687052677647234181632182025160_u128;
_3 = !_10;
RET = 6_usize as f32;
_12 = _3;
_20 = '\u{9d1ac}';
_12 = -(*_13);
_6 = _2;
match _4 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463463374607431172266151 => bb10,
_ => bb9
}
}
bb16 = {
_14 = core::ptr::addr_of_mut!((*_14));
_9 = _24 as u32;
_21.fld2 = (-126830468890120215658068092980263359180_i128);
_22 = !69_u8;
_28 = [(*_14),(*_13),_7,(*_14)];
_10 = _3 + (*_14);
_4 = _20 as i32;
_21.fld5.2 = (_26.0, RET);
_16 = !306309362411233763410194192842783953550_u128;
_10 = (*_14) + _7;
_21.fld2 = !36233848693293894774287477203292483787_i128;
_19 = -_3;
_20 = '\u{855f4}';
_21.fld5.2.1 = RET;
(*_13) = !_3;
_18 = (_11,);
_18 = (_11,);
_25 = _26.1 + _26.1;
_3 = _10 | _10;
_21.fld2 = 141366270917845042975312897321574364498_i128 - (-74392785979333694668987730217547162157_i128);
(*_13) = !_3;
_10 = _12 | _3;
_24 = _8 >> _3;
_3 = -(*_14);
_28 = [(*_13),(*_13),(*_13),(*_13)];
Goto(bb17)
}
bb17 = {
Call(_35 = dump_var(14_usize, 28_usize, Move(_28), 9_usize, Move(_9), 7_usize, Move(_7), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_35 = dump_var(14_usize, 19_usize, Move(_19), 2_usize, Move(_2), 24_usize, Move(_24), 18_usize, Move(_18)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_35 = dump_var(14_usize, 6_usize, Move(_6), 12_usize, Move(_12), 36_usize, _36, 36_usize, _36), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: isize,mut _2: i16,mut _3: f32) -> *mut *const u8 {
mir! {
type RET = *mut *const u8;
let _4: i128;
let _5: [i8; 8];
let _6: (char, i16);
let _7: (i32, f32, (u16,));
let _8: i64;
let _9: (u16,);
let _10: Adt49;
let _11: (i64,);
let _12: f64;
let _13: Adt41;
let _14: bool;
let _15: (char, i16);
let _16: (char, i16);
let _17: *mut [bool; 7];
let _18: bool;
let _19: (i16, f32);
let _20: Adt43;
let _21: [bool; 7];
let _22: f32;
let _23: Adt49;
let _24: [bool; 7];
let _25: u32;
let _26: *const [isize; 4];
let _27: *mut isize;
let _28: i16;
let _29: [bool; 7];
let _30: [u8; 3];
let _31: (i64,);
let _32: char;
let _33: f32;
let _34: i32;
let _35: char;
let _36: u32;
let _37: f32;
let _38: u16;
let _39: (u128, [isize; 4], (i16, f32));
let _40: bool;
let _41: [isize; 4];
let _42: char;
let _43: Adt52;
let _44: (u128, [isize; 4], (i16, f32));
let _45: (char, i16);
let _46: [i8; 8];
let _47: u64;
let _48: Adt44;
let _49: (i32, f32, (u16,));
let _50: i16;
let _51: isize;
let _52: i8;
let _53: (i16, f32);
let _54: [isize; 4];
let _55: [i8; 8];
let _56: f32;
let _57: Adt41;
let _58: (i64,);
let _59: u32;
let _60: u64;
let _61: char;
let _62: *mut isize;
let _63: (i16, f32);
let _64: i16;
let _65: ();
let _66: ();
{
_2 = !(-17445_i16);
_2 = (-1161504345_i32) as i16;
Goto(bb1)
}
bb1 = {
_2 = (-248680195_i32) as i16;
_1 = 687476132_i32 as isize;
_4 = (-70683648760167594104468432079364374369_i128);
_5 = [(-11_i8),28_i8,37_i8,(-66_i8),75_i8,82_i8,(-24_i8),47_i8];
_5 = [21_i8,44_i8,35_i8,(-10_i8),87_i8,(-95_i8),(-47_i8),66_i8];
_1 = (-9223372036854775808_isize);
_3 = _2 as f32;
_6.1 = 3866919109145405604_usize as i16;
_4 = (-28734255508052371308389397469233665332_i128);
_2 = -_6.1;
Goto(bb2)
}
bb2 = {
_6.1 = _2;
_6.0 = '\u{363ca}';
_7.2.0 = 55499_u16 + 24352_u16;
_6.0 = '\u{d4bef}';
_7.0 = -(-2001940736_i32);
_4 = (-135181806313606643066289216360784152005_i128) & 39841884263680363095893282892509218366_i128;
_6 = ('\u{b0a80}', _2);
_7.2 = (45961_u16,);
_7.1 = _3 + _3;
_7.2 = (7522_u16,);
_6.0 = '\u{fdbf3}';
_6.1 = 10333946045135024731_u64 as i16;
_5 = [(-7_i8),(-115_i8),(-104_i8),(-49_i8),44_i8,5_i8,54_i8,(-88_i8)];
_6 = ('\u{1032c6}', _2);
_6.1 = 4140610495418082988_i64 as i16;
_6 = ('\u{1e3cf}', _2);
_4 = (-71396803253345833185531827722197159786_i128) - (-135178136753505685367849368068080302357_i128);
_2 = -_6.1;
_9 = _7.2;
_9.0 = _7.2.0;
_9.0 = !_7.2.0;
_7 = ((-1525307361_i32), _3, _9);
_2 = _6.1 >> _6.1;
_6.0 = '\u{b6fed}';
_3 = _7.1 * _7.1;
_7 = (1435113246_i32, _3, _9);
_9 = _7.2;
match _7.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
1435113246 => bb9,
_ => bb8
}
}
bb3 = {
_2 = (-248680195_i32) as i16;
_1 = 687476132_i32 as isize;
_4 = (-70683648760167594104468432079364374369_i128);
_5 = [(-11_i8),28_i8,37_i8,(-66_i8),75_i8,82_i8,(-24_i8),47_i8];
_5 = [21_i8,44_i8,35_i8,(-10_i8),87_i8,(-95_i8),(-47_i8),66_i8];
_1 = (-9223372036854775808_isize);
_3 = _2 as f32;
_6.1 = 3866919109145405604_usize as i16;
_4 = (-28734255508052371308389397469233665332_i128);
_2 = -_6.1;
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
_6.1 = !_2;
_6.0 = '\u{f758f}';
_5 = [(-1_i8),(-124_i8),(-27_i8),(-32_i8),(-123_i8),(-74_i8),(-28_i8),81_i8];
_6 = ('\u{d6d3e}', _2);
_3 = _7.1;
_7 = ((-909499314_i32), _3, _9);
_6 = ('\u{7196e}', _2);
_7.1 = -_3;
_1 = (-33_isize);
_4 = (-157534542996321710183854629357373462081_i128) | 90960610453967077402470322818801481561_i128;
_1 = 9223372036854775807_isize << _7.2.0;
_7.0 = (-1810740648_i32) + 1629366392_i32;
_11.0 = -1800482471975478392_i64;
_7.0 = _1 as i32;
Call(_1 = fn16(_6.0, _6.0, _5, _5, _6.0, _5, _5), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Goto(bb11)
}
bb11 = {
_2 = 84_u8 as i16;
_13.fld0 = 3002369798_u32 as f64;
_12 = 231_u8 as f64;
_4 = 141493630712352881783654620261096858890_u128 as i128;
_12 = _13.fld0 + _13.fld0;
_11 = ((-9144884338072294136_i64),);
_14 = false & false;
_2 = !_6.1;
_13.fld1 = !7344633124056846923_usize;
Goto(bb12)
}
bb12 = {
_9.0 = _7.2.0 - _7.2.0;
_15 = (_6.0, _2);
_6 = _15;
_14 = true;
_16 = (_6.0, _6.1);
_11.0 = (-146145331389905830_i64);
_6.1 = _14 as i16;
_8 = _11.0 ^ _11.0;
_6 = (_15.0, _15.1);
_13.fld0 = _12;
_14 = !true;
Goto(bb13)
}
bb13 = {
_11 = (_8,);
_16.0 = _6.0;
_1 = 9223372036854775807_isize + (-9223372036854775808_isize);
_11 = (_8,);
Goto(bb14)
}
bb14 = {
_5 = [(-95_i8),52_i8,37_i8,116_i8,67_i8,(-72_i8),119_i8,(-97_i8)];
_6.0 = _15.0;
_16.1 = _15.1 >> _9.0;
_6.0 = _15.0;
_6 = _16;
_8 = _11.0;
_3 = _7.1;
_7 = ((-1249577501_i32), _3, _9);
_21 = [_14,_14,_14,_14,_14,_14,_14];
match _7.0 {
340282366920938463463374607430518633955 => bb15,
_ => bb3
}
}
bb15 = {
_2 = _6.1 >> _16.1;
_20.fld1.2 = _16.0 as i32;
_11 = (_8,);
_7.2 = (_9.0,);
_3 = _7.1 * _7.1;
_20.fld1.0 = core::ptr::addr_of_mut!(_1);
_18 = _14;
_20.fld1.0 = core::ptr::addr_of_mut!(_1);
_3 = -_7.1;
_19.1 = 3939351688_u32 as f32;
_22 = _7.1 + _19.1;
_17 = core::ptr::addr_of_mut!(_21);
_19 = (_15.1, _7.1);
_11 = (_8,);
_25 = 3399746478_u32;
_28 = _15.0 as i16;
_20.fld1.1 = _2 | _28;
_11 = (_8,);
match _7.0 {
0 => bb11,
340282366920938463463374607430518633955 => bb17,
_ => bb16
}
}
bb16 = {
_2 = (-248680195_i32) as i16;
_1 = 687476132_i32 as isize;
_4 = (-70683648760167594104468432079364374369_i128);
_5 = [(-11_i8),28_i8,37_i8,(-66_i8),75_i8,82_i8,(-24_i8),47_i8];
_5 = [21_i8,44_i8,35_i8,(-10_i8),87_i8,(-95_i8),(-47_i8),66_i8];
_1 = (-9223372036854775808_isize);
_3 = _2 as f32;
_6.1 = 3866919109145405604_usize as i16;
_4 = (-28734255508052371308389397469233665332_i128);
_2 = -_6.1;
Goto(bb2)
}
bb17 = {
_29 = [_18,_14,_18,_14,_14,_14,_18];
_13.fld0 = _25 as f64;
_18 = _14;
_7.2 = _9;
_19.0 = (-71_i8) as i16;
_2 = !_15.1;
_3 = _9.0 as f32;
_20.fld2 = _5;
_3 = -_19.1;
_15.1 = _16.1 >> _16.1;
_6.1 = _20.fld1.1;
_16.0 = _15.0;
_2 = _15.1;
_24 = [_18,_14,_18,_18,_18,_14,_14];
(*_17) = [_14,_14,_14,_18,_14,_14,_18];
_16.1 = _15.1;
_6.1 = -_15.1;
_4 = (-11523909015081752876921254925934164739_i128);
_20.fld2 = _5;
_4 = (-131241304998217276819488818920070946203_i128) | (-122738471041515089421256784671124227586_i128);
_19 = (_16.1, _3);
_28 = 216_u8 as i16;
_12 = _13.fld1 as f64;
match _7.0 {
0 => bb16,
340282366920938463463374607430518633955 => bb18,
_ => bb2
}
}
bb18 = {
_4 = _7.0 as i128;
_7.0 = _1 as i32;
_35 = _6.0;
_32 = _35;
_15.0 = _6.0;
_27 = _20.fld1.0;
_14 = _18 & _18;
_20.fld1.2 = !_7.0;
_30 = [16_u8,33_u8,151_u8];
_25 = !2853907063_u32;
_15 = (_35, _6.1);
_3 = _19.1 + _22;
_11 = (_8,);
_13.fld0 = _12;
_33 = -_3;
_20.fld1.0 = core::ptr::addr_of_mut!(_1);
_31.0 = (-15_i8) as i64;
_7.2.0 = !_9.0;
_38 = _22 as u16;
_32 = _16.0;
_13 = Adt41 { fld0: _12,fld1: 3215222385885802617_usize };
_7.0 = _14 as i32;
_27 = core::ptr::addr_of_mut!(_1);
_4 = 43299879842833978789252719834324057859_i128;
match _13.fld1 {
3215222385885802617 => bb19,
_ => bb4
}
}
bb19 = {
_20.fld1.3 = 5874886269856988545_u64 as f64;
_37 = _8 as f32;
Call(_20.fld1.2 = fn18(_32, _6, _16.0, _30, _22, _21, _30, _15, _20.fld1.1, _13, (*_27), _8), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
_39.2 = (_2, _19.1);
_7.1 = _39.2.1;
_19 = (_20.fld1.1, _3);
_26 = core::ptr::addr_of!(_39.1);
_1 = !9223372036854775807_isize;
_20.fld1.3 = _13.fld0 - _12;
_11 = (_8,);
_13.fld1 = 12462228653882911978_usize;
_7.0 = 9932361330882903176_u64 as i32;
(*_26) = [(*_27),_1,_1,(*_27)];
_5 = [98_i8,102_i8,(-25_i8),(-3_i8),80_i8,72_i8,43_i8,(-49_i8)];
_11.0 = _31.0 & _31.0;
_18 = _14;
_31 = (_8,);
_20.fld1.3 = _20.fld1.2 as f64;
Goto(bb21)
}
bb21 = {
_39.0 = 191378079413495816087856339360498024471_u128;
_19 = _39.2;
_27 = _20.fld1.0;
_40 = !_14;
_42 = _32;
_39.2 = (_15.1, _3);
_35 = _15.0;
_14 = _19.0 >= _15.1;
_40 = _14;
_11.0 = _8;
_39.0 = 311870704616989681648894686673538026383_u128 & 332109951683822025255635696755716670241_u128;
_7.2.0 = !_38;
_11.0 = !_31.0;
_6 = (_15.0, _19.0);
_7.2 = (_38,);
(*_27) = 9223372036854775807_isize - 9223372036854775807_isize;
_12 = _39.0 as f64;
_41 = [(*_27),(*_27),_1,(*_27)];
_36 = _25;
_20.fld1.1 = _39.2.0 >> _39.2.0;
(*_17) = _29;
_15 = (_35, _2);
_13.fld0 = _12;
_31.0 = _40 as i64;
match _13.fld1 {
12462228653882911978 => bb23,
_ => bb22
}
}
bb22 = {
_2 = (-248680195_i32) as i16;
_1 = 687476132_i32 as isize;
_4 = (-70683648760167594104468432079364374369_i128);
_5 = [(-11_i8),28_i8,37_i8,(-66_i8),75_i8,82_i8,(-24_i8),47_i8];
_5 = [21_i8,44_i8,35_i8,(-10_i8),87_i8,(-95_i8),(-47_i8),66_i8];
_1 = (-9223372036854775808_isize);
_3 = _2 as f32;
_6.1 = 3866919109145405604_usize as i16;
_4 = (-28734255508052371308389397469233665332_i128);
_2 = -_6.1;
Goto(bb2)
}
bb23 = {
_44 = (_39.0, _39.1, _39.2);
_17 = core::ptr::addr_of_mut!(_29);
_29 = [_14,_40,_40,_40,_40,_14,_40];
_15.1 = _20.fld1.1 * _44.2.0;
_19.0 = _39.2.0 - _39.2.0;
_27 = core::ptr::addr_of_mut!((*_27));
_29 = [_18,_14,_14,_40,_40,_14,_14];
_19.0 = _1 as i16;
_39 = (_44.0, _41, _44.2);
_41 = [(*_27),(*_27),(*_27),(*_27)];
_18 = _40;
_46 = _20.fld2;
_18 = !_14;
_46 = [110_i8,(-112_i8),77_i8,23_i8,(-17_i8),95_i8,(-76_i8),(-106_i8)];
_27 = _20.fld1.0;
_13.fld1 = 1329108188694365544_usize;
(*_27) = (-9223372036854775808_isize) << _15.1;
match _13.fld1 {
1329108188694365544 => bb24,
_ => bb4
}
}
bb24 = {
_30 = [135_u8,204_u8,125_u8];
_46 = _5;
_39.2 = _44.2;
Goto(bb25)
}
bb25 = {
_44.0 = !_39.0;
_51 = (*_27) * (*_27);
_25 = _36;
Goto(bb26)
}
bb26 = {
_52 = 37_i8;
_45.1 = _13.fld1 as i16;
_27 = _20.fld1.0;
Call(_16 = fn19(_44.2.0, _51, _20.fld1.0, _6.1, _9, _27, _42, (*_27), _20.fld1.0, _51, _51), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
_41 = (*_26);
_49.2 = (_38,);
_27 = _20.fld1.0;
match _52 {
0 => bb28,
37 => bb30,
_ => bb29
}
}
bb28 = {
Return()
}
bb29 = {
_11 = (_8,);
_16.0 = _6.0;
_1 = 9223372036854775807_isize + (-9223372036854775808_isize);
_11 = (_8,);
Goto(bb14)
}
bb30 = {
_28 = _15.1;
(*_17) = [_14,_14,_40,_40,_14,_40,_40];
_16.1 = _28 & _2;
_20.fld1.3 = -_12;
_5 = [_52,_52,_52,_52,_52,_52,_52,_52];
_39.2 = (_15.1, _3);
_6.1 = !_39.2.0;
_12 = _20.fld1.3;
_9.0 = 10974053283866717363_u64 as u16;
_4 = 103851405954559330510760802619566831156_i128;
_31.0 = !_11.0;
_45 = _15;
_22 = -_7.1;
_6.1 = _16.1;
_15.0 = _6.0;
_51 = _1;
_44.2 = (_15.1, _39.2.1);
Goto(bb31)
}
bb31 = {
_13 = Adt41 { fld0: _20.fld1.3,fld1: 12104789231292881002_usize };
_12 = _20.fld1.3 + _13.fld0;
_47 = 12337031475265711108_u64;
_44 = (_39.0, (*_26), _39.2);
_44.0 = _39.0 & _39.0;
_16.1 = _20.fld1.1 - _44.2.0;
Goto(bb32)
}
bb32 = {
_7.1 = _3 * _44.2.1;
_51 = (*_27) ^ _1;
_23 = Adt49::Variant2 { fld0: _9,fld1: _26 };
_16.0 = _6.0;
_57.fld0 = _12 + _12;
_7 = (_20.fld1.2, _44.2.1, _49.2);
_34 = !_20.fld1.2;
_57.fld1 = _11.0 as usize;
_11.0 = !_31.0;
_18 = _14 & _40;
_58.0 = _8;
_42 = _15.0;
_50 = (*_27) as i16;
match _47 {
0 => bb2,
12337031475265711108 => bb34,
_ => bb33
}
}
bb33 = {
_13 = Adt41 { fld0: _20.fld1.3,fld1: 12104789231292881002_usize };
_12 = _20.fld1.3 + _13.fld0;
_47 = 12337031475265711108_u64;
_44 = (_39.0, (*_26), _39.2);
_44.0 = _39.0 & _39.0;
_16.1 = _20.fld1.1 - _44.2.0;
Goto(bb32)
}
bb34 = {
_63.1 = _7.1;
_6 = _16;
_39.2.1 = _7.0 as f32;
SetDiscriminant(_23, 0);
RET = core::ptr::addr_of_mut!(place!(Field::<*const u8>(Variant(_23, 0), 3)));
place!(Field::<[u8; 3]>(Variant(_23, 0), 0)) = [13_u8,211_u8,74_u8];
_20.fld1.3 = _57.fld0;
_21 = [_14,_18,_40,_18,_14,_18,_18];
_53.1 = -_44.2.1;
_11.0 = -_8;
_49 = _7;
place!(Field::<(u128, [isize; 4], (i16, f32))>(Variant(_23, 0), 1)).2 = (_44.2.0, _7.1);
place!(Field::<Adt45>(Variant(_23, 0), 4)).fld0.1 = _21;
(*_17) = [_40,_40,_40,_14,_14,_18,_40];
_64 = -_50;
Goto(bb35)
}
bb35 = {
Call(_65 = dump_var(15_usize, 52_usize, Move(_52), 8_usize, Move(_8), 25_usize, Move(_25), 58_usize, Move(_58)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Call(_65 = dump_var(15_usize, 36_usize, Move(_36), 41_usize, Move(_41), 1_usize, Move(_1), 28_usize, Move(_28)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Call(_65 = dump_var(15_usize, 64_usize, Move(_64), 51_usize, Move(_51), 34_usize, Move(_34), 30_usize, Move(_30)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Call(_65 = dump_var(15_usize, 16_usize, Move(_16), 6_usize, Move(_6), 5_usize, Move(_5), 40_usize, Move(_40)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_65 = dump_var(15_usize, 32_usize, Move(_32), 66_usize, _66, 66_usize, _66, 66_usize, _66), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: char,mut _2: char,mut _3: [i8; 8],mut _4: [i8; 8],mut _5: char,mut _6: [i8; 8],mut _7: [i8; 8]) -> isize {
mir! {
type RET = isize;
let _8: Adt52;
let _9: i16;
let _10: (i16, f32);
let _11: i64;
let _12: u16;
let _13: (i32, f32, (u16,));
let _14: bool;
let _15: Adt42;
let _16: (u128, [isize; 4], (i16, f32));
let _17: isize;
let _18: isize;
let _19: [i8; 8];
let _20: (i16, f32);
let _21: f64;
let _22: (i16, f32);
let _23: Adt36;
let _24: char;
let _25: [i8; 8];
let _26: (i16, f32);
let _27: isize;
let _28: i8;
let _29: bool;
let _30: char;
let _31: f64;
let _32: (i32, f32, (u16,));
let _33: i8;
let _34: u16;
let _35: char;
let _36: [u8; 3];
let _37: ();
let _38: ();
{
RET = (-9223372036854775808_isize) | (-16_isize);
_5 = _2;
_1 = _2;
_4 = _3;
_2 = _1;
_1 = _2;
_7 = [(-41_i8),62_i8,(-94_i8),(-29_i8),(-29_i8),103_i8,(-116_i8),20_i8];
_7 = _3;
_7 = [77_i8,82_i8,(-25_i8),40_i8,126_i8,48_i8,(-32_i8),(-100_i8)];
_6 = [38_i8,(-100_i8),(-76_i8),120_i8,102_i8,(-9_i8),10_i8,125_i8];
_2 = _5;
RET = 1033533127_u32 as isize;
_4 = [(-74_i8),91_i8,(-61_i8),(-26_i8),(-66_i8),(-14_i8),65_i8,120_i8];
_2 = _5;
_5 = _2;
_9 = !(-19927_i16);
_4 = [63_i8,(-67_i8),102_i8,(-52_i8),92_i8,(-102_i8),124_i8,57_i8];
_1 = _5;
Goto(bb1)
}
bb1 = {
_5 = _1;
_10.0 = _9;
_7 = [(-126_i8),(-40_i8),(-123_i8),(-94_i8),122_i8,(-74_i8),49_i8,114_i8];
_6 = _4;
_5 = _1;
Goto(bb2)
}
bb2 = {
RET = 9223372036854775807_isize * 9223372036854775807_isize;
RET = false as isize;
_10.1 = (-46_i8) as f32;
_5 = _1;
_13.2 = (27095_u16,);
_13.1 = -_10.1;
_12 = _13.2.0;
_13.2 = (_12,);
_1 = _2;
_13.0 = 121_u8 as i32;
_2 = _5;
RET = !37_isize;
_12 = !_13.2.0;
_6 = _4;
_13.0 = !(-1340914368_i32);
_2 = _5;
_13.0 = (-356153029_i32) + 819985174_i32;
_10.1 = _13.1;
_11 = 718014161766341962_i64 - 5813592672620861685_i64;
_13.0 = -44964268_i32;
_16.2 = (_10.0, _13.1);
_7 = [(-64_i8),94_i8,126_i8,54_i8,(-95_i8),(-55_i8),47_i8,24_i8];
_13.0 = false as i32;
match _13.2.0 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
27095 => bb8,
_ => bb7
}
}
bb3 = {
_5 = _1;
_10.0 = _9;
_7 = [(-126_i8),(-40_i8),(-123_i8),(-94_i8),122_i8,(-74_i8),49_i8,114_i8];
_6 = _4;
_5 = _1;
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
_17 = RET - RET;
_16.0 = 214753666967775734565944328480737108075_u128;
RET = _17 >> _13.0;
RET = !_17;
_11 = _9 as i64;
_1 = _2;
_20 = (_9, _16.2.1);
_16.2.0 = _16.0 as i16;
_21 = _16.0 as f64;
_16.1 = [RET,RET,_17,RET];
_2 = _5;
_16.0 = !246845599834533321825662992712288132388_u128;
_13.0 = -36641135_i32;
_19 = [44_i8,1_i8,71_i8,89_i8,118_i8,111_i8,114_i8,(-17_i8)];
_22.1 = _20.1;
_6 = _3;
_13.0 = _13.2.0 as i32;
match _13.2.0 {
27095 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_4 = _6;
_13.0 = !1016839399_i32;
_13.2 = (_12,);
_7 = [82_i8,77_i8,44_i8,(-40_i8),54_i8,58_i8,(-35_i8),73_i8];
_10 = (_20.0, _16.2.1);
_22.0 = 16640466001757062918_u64 as i16;
_27 = !_17;
_20.1 = -_22.1;
_20.0 = -_9;
_23.fld1 = _1;
_10.0 = _13.0 as i16;
_23.fld1 = _2;
_24 = _23.fld1;
_20.1 = -_16.2.1;
_22 = _10;
_16.2.0 = _20.0 ^ _20.0;
_20.0 = 95_u8 as i16;
Goto(bb11)
}
bb11 = {
_26 = (_16.2.0, _22.1);
_25 = [107_i8,112_i8,(-7_i8),(-38_i8),(-92_i8),122_i8,(-62_i8),(-113_i8)];
_7 = _4;
_25 = [92_i8,(-30_i8),(-107_i8),56_i8,(-72_i8),(-114_i8),(-113_i8),(-108_i8)];
_12 = _13.2.0;
_10.1 = -_16.2.1;
_23.fld0 = _16.2.0;
_26.0 = _16.2.0;
_26 = (_23.fld0, _13.1);
_18 = _24 as isize;
_16.2 = (_26.0, _10.1);
_17 = true as isize;
_26.0 = -_9;
_30 = _5;
Goto(bb12)
}
bb12 = {
_7 = [76_i8,(-69_i8),24_i8,51_i8,90_i8,109_i8,118_i8,10_i8];
_29 = !false;
_30 = _23.fld1;
_27 = _18 >> _10.0;
_20.1 = _13.1;
RET = _27;
_16.2.0 = !_20.0;
Goto(bb13)
}
bb13 = {
_32.0 = _13.0 | _13.0;
RET = _27;
_16.2 = (_22.0, _10.1);
_14 = RET < _27;
_23.fld1 = _24;
_28 = _21 as i8;
_6 = _4;
_30 = _24;
_19 = [_28,_28,_28,_28,_28,_28,_28,_28];
_32.2.0 = _32.0 as u16;
_21 = 4107557541281274112_u64 as f64;
_7 = [_28,_28,_28,_28,_28,_28,_28,_28];
_32 = _13;
_1 = _24;
_32.0 = _13.0 << _20.0;
_10 = (_22.0, _13.1);
_36 = [199_u8,225_u8,137_u8];
_35 = _5;
RET = _18 - _17;
_25 = _3;
_32.0 = !_13.0;
_14 = !_29;
Call(_4 = fn17(_5, _6, _18, _3, _23.fld1, _24, RET, _24, _10, _27, _16.2.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
RET = _11 as isize;
_34 = 7333435506259610572_usize as u16;
_3 = [_28,_28,_28,_28,_28,_28,_28,_28];
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(16_usize, 27_usize, Move(_27), 9_usize, Move(_9), 24_usize, Move(_24), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(16_usize, 17_usize, Move(_17), 34_usize, Move(_34), 30_usize, Move(_30), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(16_usize, 35_usize, Move(_35), 11_usize, Move(_11), 28_usize, Move(_28), 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: char,mut _2: [i8; 8],mut _3: isize,mut _4: [i8; 8],mut _5: char,mut _6: char,mut _7: isize,mut _8: char,mut _9: (i16, f32),mut _10: isize,mut _11: i16) -> [i8; 8] {
mir! {
type RET = [i8; 8];
let _12: i128;
let _13: u32;
let _14: [isize; 4];
let _15: u32;
let _16: [i8; 8];
let _17: (i16, f32);
let _18: char;
let _19: i128;
let _20: Adt47;
let _21: (u128, [isize; 4], (i16, f32));
let _22: &'static bool;
let _23: i128;
let _24: (u128, [isize; 4], (i16, f32));
let _25: [bool; 7];
let _26: Adt47;
let _27: (i16, f32);
let _28: usize;
let _29: isize;
let _30: Adt52;
let _31: (u16,);
let _32: (*mut isize, i16, i32, f64);
let _33: isize;
let _34: [i8; 8];
let _35: bool;
let _36: isize;
let _37: [bool; 7];
let _38: u16;
let _39: (u128, [isize; 4], (i16, f32));
let _40: u8;
let _41: (u16,);
let _42: Adt45;
let _43: *mut isize;
let _44: i32;
let _45: bool;
let _46: [bool; 7];
let _47: f64;
let _48: Adt37;
let _49: isize;
let _50: Adt52;
let _51: ();
let _52: ();
{
_6 = _8;
_2 = [(-123_i8),83_i8,1_i8,(-54_i8),(-108_i8),44_i8,(-79_i8),(-108_i8)];
_3 = 8_u8 as isize;
_6 = _5;
_9.0 = _11 << _10;
_9.1 = 140109101319932870670745620746674697924_u128 as f32;
_2 = _4;
_5 = _1;
_2 = [38_i8,(-3_i8),103_i8,63_i8,100_i8,18_i8,113_i8,81_i8];
_4 = [64_i8,(-16_i8),87_i8,(-69_i8),27_i8,46_i8,47_i8,(-113_i8)];
_11 = _9.0;
RET = _2;
_3 = _7 << _9.0;
RET = [54_i8,39_i8,84_i8,67_i8,(-76_i8),(-112_i8),(-35_i8),12_i8];
_4 = RET;
_1 = _5;
Call(_7 = core::intrinsics::bswap(_3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [5_i8,111_i8,(-30_i8),(-73_i8),3_i8,124_i8,79_i8,(-53_i8)];
_9.1 = _11 as f32;
_6 = _1;
RET = [(-72_i8),(-71_i8),116_i8,75_i8,59_i8,(-49_i8),(-90_i8),(-34_i8)];
_8 = _1;
RET = [90_i8,108_i8,1_i8,72_i8,37_i8,86_i8,4_i8,(-1_i8)];
_8 = _1;
_11 = 254559816_u32 as i16;
_11 = _9.0 - _9.0;
_9.1 = 44307_u16 as f32;
RET = _4;
RET = _2;
_8 = _1;
_12 = _6 as i128;
_13 = !187118636_u32;
_1 = _5;
_7 = _10 - _3;
_14 = [_3,_7,_7,_7];
Goto(bb2)
}
bb2 = {
_8 = _5;
_15 = 18144367772421432764_u64 as u32;
_4 = _2;
_2 = _4;
_14 = [_3,_3,_3,_3];
_1 = _5;
Goto(bb3)
}
bb3 = {
_13 = _15;
_7 = (-82_i8) as isize;
_14 = [_3,_3,_7,_3];
_8 = _5;
_8 = _5;
_5 = _6;
_2 = [113_i8,(-46_i8),127_i8,(-1_i8),(-66_i8),78_i8,(-127_i8),(-100_i8)];
RET = _4;
_17.0 = _9.0 << _11;
_9.1 = 13715006190291566850_u64 as f32;
RET = [3_i8,(-102_i8),96_i8,(-42_i8),(-108_i8),(-26_i8),(-47_i8),9_i8];
_15 = !_13;
_16 = [(-39_i8),(-100_i8),(-90_i8),51_i8,(-119_i8),116_i8,6_i8,(-77_i8)];
RET = _16;
_1 = _5;
RET = _16;
_17.1 = 46662_u16 as f32;
_17.1 = _9.1;
RET = [25_i8,38_i8,(-89_i8),(-7_i8),(-43_i8),32_i8,(-113_i8),35_i8];
_12 = !(-54778140748035512733257394365797456291_i128);
_9 = (_17.0, _17.1);
_7 = !_3;
_4 = [(-46_i8),(-17_i8),88_i8,(-46_i8),(-50_i8),126_i8,(-43_i8),63_i8];
Goto(bb4)
}
bb4 = {
_20.fld2.fld1.3 = (-7261482963721928958_i64) as f64;
_8 = _6;
_15 = _13;
_5 = _1;
_17.0 = _9.0;
_12 = (-60253705508303474714408643676601285961_i128) << _11;
RET = _2;
RET = _4;
_21 = (9191706221875196751222221480412970641_u128, _14, _17);
_17.1 = _13 as f32;
_8 = _6;
_21.1 = [_7,_3,_7,_3];
_9.0 = 126_u8 as i16;
_21.2 = _17;
_12 = _15 as i128;
_17.1 = -_9.1;
_17.0 = _13 as i16;
_20.fld2.fld1.0 = core::ptr::addr_of_mut!(_10);
_21.1 = [_3,_7,_7,_3];
_1 = _6;
_3 = !_7;
_20.fld2.fld1.2 = -127057883_i32;
_20.fld2.fld1.2 = _12 as i32;
_5 = _1;
_20.fld0 = (49276_u16,);
_17.1 = _9.1 - _9.1;
_21.2.0 = _11 >> _21.0;
_17 = _21.2;
Goto(bb5)
}
bb5 = {
_18 = _5;
RET = _2;
RET = _2;
_5 = _6;
_11 = !_17.0;
_19 = !_12;
_20.fld0 = (11756_u16,);
_25 = [true,true,false,true,true,false,false];
_7 = 979966402667909508_u64 as isize;
_10 = _3 | _3;
_12 = !_19;
_23 = _19;
_20.fld2.fld1.1 = _11 - _21.2.0;
_24.2 = _17;
_17.1 = _21.2.1 - _9.1;
match _21.0 {
0 => bb1,
9191706221875196751222221480412970641 => bb6,
_ => bb3
}
}
bb6 = {
_17 = (_11, _24.2.1);
_2 = [(-73_i8),15_i8,(-12_i8),57_i8,(-32_i8),94_i8,12_i8,101_i8];
_25 = [false,false,false,false,true,false,false];
RET = _2;
_20.fld2.fld1.0 = core::ptr::addr_of_mut!(_10);
_12 = !_19;
_18 = _6;
_21.2 = _24.2;
_20.fld0 = (9908_u16,);
_20.fld2.fld2 = _16;
_26.fld1 = _17.1;
_24.2 = _17;
_21 = (249154848464804197491726847462502668693_u128, _14, _24.2);
_20.fld2.fld1.2 = 16317133271205239047_u64 as i32;
RET = [(-55_i8),43_i8,(-98_i8),122_i8,(-93_i8),8_i8,122_i8,(-52_i8)];
_20.fld1 = (-3293191839513384180_i64) as f32;
_26.fld2.fld1.3 = _10 as f64;
RET = [27_i8,(-120_i8),(-67_i8),112_i8,122_i8,96_i8,72_i8,(-90_i8)];
_24.1 = [_10,_10,_10,_10];
_26.fld2.fld1.2 = _20.fld2.fld1.2 * _20.fld2.fld1.2;
_21.2.0 = -_17.0;
RET = _4;
_20.fld2.fld1.2 = _26.fld2.fld1.2 + _26.fld2.fld1.2;
_19 = _12 << _20.fld2.fld1.1;
_24.0 = !_21.0;
_21.2 = _17;
_24.2.1 = _20.fld1 - _17.1;
Goto(bb7)
}
bb7 = {
_21.2.0 = _20.fld2.fld1.1;
_11 = -_20.fld2.fld1.1;
_26.fld2.fld1.0 = _20.fld2.fld1.0;
match _21.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
249154848464804197491726847462502668693 => bb8,
_ => bb6
}
}
bb8 = {
_24.1 = [_10,_3,_10,_10];
_26.fld0 = _20.fld0;
_1 = _6;
_20.fld2.fld1 = (_26.fld2.fld1.0, _11, _26.fld2.fld1.2, _26.fld2.fld1.3);
RET = _20.fld2.fld2;
_2 = [(-38_i8),96_i8,(-11_i8),(-60_i8),118_i8,120_i8,98_i8,(-65_i8)];
_17.1 = _20.fld1 - _24.2.1;
_24.1 = _21.1;
_20.fld0 = _26.fld0;
_27.1 = _20.fld1;
_23 = -_12;
_18 = _6;
_9 = (_21.2.0, _21.2.1);
_21.1 = [_10,_10,_10,_10];
_27.1 = _20.fld1 + _20.fld1;
_26.fld2.fld2 = [(-24_i8),(-46_i8),(-87_i8),(-112_i8),(-27_i8),100_i8,52_i8,(-74_i8)];
_26.fld2.fld1.3 = _20.fld2.fld1.3;
RET = _4;
_20.fld2.fld2 = [31_i8,23_i8,94_i8,(-95_i8),(-69_i8),(-125_i8),(-102_i8),(-40_i8)];
_21.2.0 = _11;
_26.fld2.fld1.2 = _26.fld2.fld1.3 as i32;
match _21.0 {
0 => bb5,
249154848464804197491726847462502668693 => bb9,
_ => bb4
}
}
bb9 = {
_26.fld0 = _20.fld0;
_32.0 = _26.fld2.fld1.0;
_20.fld1 = _27.1 + _24.2.1;
_29 = _3;
_21 = (_24.0, _24.1, _24.2);
_24 = (_21.0, _21.1, _9);
_16 = [(-68_i8),(-92_i8),(-94_i8),(-112_i8),(-17_i8),(-16_i8),32_i8,(-104_i8)];
_13 = _15;
_15 = _13;
_32.1 = 72_i8 as i16;
_9.0 = _20.fld0.0 as i16;
_20.fld0.0 = _11 as u16;
_26.fld2.fld1.2 = _20.fld2.fld1.2;
_20.fld2.fld1.0 = core::ptr::addr_of_mut!(_7);
_34 = [93_i8,(-125_i8),105_i8,(-76_i8),88_i8,(-87_i8),99_i8,(-78_i8)];
_2 = [(-101_i8),21_i8,21_i8,58_i8,(-29_i8),(-128_i8),(-35_i8),(-9_i8)];
_31 = (_20.fld0.0,);
_26.fld0.0 = _20.fld0.0 << _31.0;
_9.0 = !_20.fld2.fld1.1;
_11 = _13 as i16;
_32 = _20.fld2.fld1;
_26.fld2.fld1 = (_32.0, _32.1, _32.2, _20.fld2.fld1.3);
_32.1 = !_20.fld2.fld1.1;
_4 = [(-97_i8),(-14_i8),64_i8,(-125_i8),17_i8,(-8_i8),54_i8,112_i8];
Goto(bb10)
}
bb10 = {
_9 = (_32.1, _17.1);
_26.fld2.fld1.1 = _21.2.0 - _17.0;
RET = _16;
Goto(bb11)
}
bb11 = {
_32.1 = _24.2.0;
_9.1 = -_20.fld1;
_24.1 = [_3,_10,_3,_3];
_34 = [62_i8,(-101_i8),124_i8,(-26_i8),111_i8,51_i8,(-14_i8),14_i8];
_7 = _29;
_28 = 2273168472468598996_usize + 10265643965329779858_usize;
_11 = 1492095337723650785_u64 as i16;
_5 = _8;
_31 = (_26.fld0.0,);
_39.1 = _24.1;
_10 = _26.fld2.fld1.3 as isize;
_31 = _20.fld0;
_17.0 = _24.2.0 * _21.2.0;
_9.1 = _17.1 - _27.1;
_27 = _17;
_36 = -_10;
_39 = (_21.0, _24.1, _24.2);
_5 = _6;
_35 = _9.0 < _9.0;
_20.fld2.fld1.3 = _10 as f64;
_42.fld5.1 = [_36,_10,_36,_7];
_36 = _9.0 as isize;
_33 = _36;
_8 = _1;
Goto(bb12)
}
bb12 = {
_41 = _20.fld0;
_23 = !_19;
_17.0 = -_20.fld2.fld1.1;
_33 = _36 << _27.0;
_31.0 = !_41.0;
_20.fld1 = _9.1 * _9.1;
_39.1 = [_33,_36,_36,_36];
_26.fld2.fld1 = (_20.fld2.fld1.0, _17.0, _20.fld2.fld1.2, _20.fld2.fld1.3);
_12 = _23;
_42.fld5 = _24;
_42.fld0.1 = [_35,_35,_35,_35,_35,_35,_35];
Goto(bb13)
}
bb13 = {
_26.fld0.0 = 2981658350374138027_i64 as u16;
_34 = [61_i8,(-15_i8),(-70_i8),78_i8,127_i8,87_i8,(-6_i8),23_i8];
_12 = _19 * _19;
_27.1 = _20.fld1;
_39.0 = _42.fld5.0 | _24.0;
_37 = [_35,_35,_35,_35,_35,_35,_35];
_32.1 = _42.fld5.2.0;
_17.0 = _12 as i16;
_21.2.1 = -_20.fld1;
_19 = 205_u8 as i128;
_17 = (_9.0, _21.2.1);
_32.1 = !_26.fld2.fld1.1;
_21.1 = _39.1;
_39.2.1 = -_27.1;
Goto(bb14)
}
bb14 = {
_20.fld2.fld3 = core::ptr::addr_of_mut!(_42.fld0.0);
_42.fld5.2.1 = _27.1;
_5 = _1;
_14 = [_33,_33,_36,_33];
_47 = _20.fld2.fld1.3 * _20.fld2.fld1.3;
_39.2 = (_26.fld2.fld1.1, _17.1);
_9 = _24.2;
_5 = _1;
_26.fld2.fld1 = (_32.0, _32.1, _20.fld2.fld1.2, _20.fld2.fld1.3);
_20.fld2.fld1.0 = _32.0;
_31.0 = 3_i8 as u16;
_49 = _33 ^ _33;
_20.fld2.fld1 = (_26.fld2.fld1.0, _27.0, _32.2, _47);
_9.1 = _39.2.1 - _42.fld5.2.1;
_16 = [39_i8,(-53_i8),(-118_i8),(-6_i8),(-127_i8),(-103_i8),(-96_i8),40_i8];
_21.2 = _17;
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(17_usize, 7_usize, Move(_7), 29_usize, Move(_29), 2_usize, Move(_2), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(17_usize, 12_usize, Move(_12), 31_usize, Move(_31), 8_usize, Move(_8), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(17_usize, 16_usize, Move(_16), 13_usize, Move(_13), 41_usize, Move(_41), 3_usize, Move(_3)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_51 = dump_var(17_usize, 1_usize, Move(_1), 37_usize, Move(_37), 52_usize, _52, 52_usize, _52), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: char,mut _2: (char, i16),mut _3: char,mut _4: [u8; 3],mut _5: f32,mut _6: [bool; 7],mut _7: [u8; 3],mut _8: (char, i16),mut _9: i16,mut _10: Adt41,mut _11: isize,mut _12: i64) -> i32 {
mir! {
type RET = i32;
let _13: u64;
let _14: [bool; 7];
let _15: f64;
let _16: bool;
let _17: Adt52;
let _18: i128;
let _19: [bool; 7];
let _20: char;
let _21: u16;
let _22: f64;
let _23: char;
let _24: Adt40;
let _25: Adt39;
let _26: i32;
let _27: ();
let _28: ();
{
RET = _8.0 as i32;
_8 = (_3, _9);
RET = !1639863000_i32;
_3 = _1;
_4 = [194_u8,114_u8,78_u8];
_3 = _8.0;
_12 = 1502842425363859473_i64 - 6880842176460259531_i64;
_1 = _3;
RET = 85_i8 as i32;
_10.fld0 = 81688802272408160119571050529696184020_i128 as f64;
_11 = 4_isize;
_8.1 = _9 * _2.1;
_4 = [234_u8,78_u8,137_u8];
_13 = 10534278420227958313_u64 << _8.1;
_14 = [true,true,false,false,false,true,false];
_13 = 67_u8 as u64;
_6 = [false,true,true,false,true,true,false];
Goto(bb1)
}
bb1 = {
_8 = _2;
_4 = [188_u8,247_u8,94_u8];
_8.1 = _9;
_2.0 = _1;
_4 = [95_u8,149_u8,0_u8];
Goto(bb2)
}
bb2 = {
_1 = _2.0;
_10.fld0 = RET as f64;
RET = -1723503354_i32;
_15 = 68_u8 as f64;
_7 = [210_u8,133_u8,203_u8];
_7 = [152_u8,203_u8,96_u8];
_8 = (_3, _2.1);
_10.fld1 = !6_usize;
_2.1 = _8.1;
_18 = 135570857514460587942512487904862746145_i128;
_15 = _10.fld0;
_2.0 = _3;
_2.1 = -_9;
_10 = Adt41 { fld0: _15,fld1: 0_usize };
_9 = _2.1;
Call(RET = core::intrinsics::bswap(1897466199_i32), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10 = Adt41 { fld0: _15,fld1: 0_usize };
match _10.fld1 {
1 => bb4,
2 => bb5,
3 => bb6,
0 => bb8,
_ => bb7
}
}
bb4 = {
_1 = _2.0;
_10.fld0 = RET as f64;
RET = -1723503354_i32;
_15 = 68_u8 as f64;
_7 = [210_u8,133_u8,203_u8];
_7 = [152_u8,203_u8,96_u8];
_8 = (_3, _2.1);
_10.fld1 = !6_usize;
_2.1 = _8.1;
_18 = 135570857514460587942512487904862746145_i128;
_15 = _10.fld0;
_2.0 = _3;
_2.1 = -_9;
_10 = Adt41 { fld0: _15,fld1: 0_usize };
_9 = _2.1;
Call(RET = core::intrinsics::bswap(1897466199_i32), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_8 = _2;
_4 = [188_u8,247_u8,94_u8];
_8.1 = _9;
_2.0 = _1;
_4 = [95_u8,149_u8,0_u8];
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_13 = !13784667500367158590_u64;
_19 = _6;
RET = -498357225_i32;
RET = (-1101848291_i32) * (-1297926422_i32);
_19 = [false,true,false,true,true,true,true];
_12 = (-2083048022846951022_i64);
RET = (-1393215069_i32);
_16 = false;
_10.fld1 = 2_usize >> _8.1;
_2.0 = _1;
Goto(bb9)
}
bb9 = {
_20 = _2.0;
_3 = _20;
_2 = _8;
_3 = _8.0;
_14 = [_16,_16,_16,_16,_16,_16,_16];
_16 = _2.0 > _3;
_13 = 14394317969456271568_u64;
_23 = _3;
_1 = _3;
Goto(bb10)
}
bb10 = {
_18 = 2147951619_u32 as i128;
_15 = -_10.fld0;
_1 = _2.0;
_8.0 = _20;
_20 = _23;
match _13 {
0 => bb6,
1 => bb2,
2 => bb8,
3 => bb11,
4 => bb12,
14394317969456271568 => bb14,
_ => bb13
}
}
bb11 = {
_8 = _2;
_4 = [188_u8,247_u8,94_u8];
_8.1 = _9;
_2.0 = _1;
_4 = [95_u8,149_u8,0_u8];
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_1 = _8.0;
_26 = RET;
_8.1 = _3 as i16;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(18_usize, 1_usize, Move(_1), 11_usize, Move(_11), 3_usize, Move(_3), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(18_usize, 9_usize, Move(_9), 20_usize, Move(_20), 12_usize, Move(_12), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_27 = dump_var(18_usize, 4_usize, Move(_4), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: i16,mut _2: isize,mut _3: *mut isize,mut _4: i16,mut _5: (u16,),mut _6: *mut isize,mut _7: char,mut _8: isize,mut _9: *mut isize,mut _10: isize,mut _11: isize) -> (char, i16) {
mir! {
type RET = (char, i16);
let _12: i128;
let _13: Adt39;
let _14: (i16, f32);
let _15: i128;
let _16: Adt41;
let _17: (i64,);
let _18: ();
let _19: ();
{
(*_6) = (-26_i8) as isize;
_6 = core::ptr::addr_of_mut!((*_6));
(*_6) = _5.0 as isize;
RET = (_7, _4);
_12 = -(-119183023316836369101123791829702802890_i128);
RET = (_7, _4);
_14.1 = RET.1 as f32;
_10 = _8 & _2;
(*_9) = -_10;
_8 = !(*_9);
(*_6) = _2 * _2;
_5.0 = 46124_u16 | 47894_u16;
RET.0 = _7;
_15 = _12;
_12 = 162_u8 as i128;
Goto(bb1)
}
bb1 = {
_5.0 = 6854_u16 + 64966_u16;
_12 = _15;
_7 = RET.0;
_12 = _15 ^ _15;
_12 = 1382694526630780156_u64 as i128;
_7 = RET.0;
_4 = 3576327586407344049_i64 as i16;
(*_9) = _7 as isize;
RET = (_7, _1);
(*_9) = (-267329526_i32) as isize;
_9 = _3;
_4 = _14.1 as i16;
_4 = _1 * _1;
_10 = _7 as isize;
(*_3) = _8;
_14.1 = 704600522_i32 as f32;
_15 = 167860263032183381662773627825282535361_u128 as i128;
RET = (_7, _1);
RET = (_7, _1);
_10 = (*_9);
_3 = _9;
(*_6) = _10 << _10;
_5.0 = 6153_u16;
_6 = core::ptr::addr_of_mut!((*_3));
_2 = -(*_9);
_1 = _4;
_10 = (*_3);
RET.1 = -_1;
_4 = _14.1 as i16;
_12 = (*_3) as i128;
(*_6) = _2 & _8;
Goto(bb2)
}
bb2 = {
Call(_18 = dump_var(19_usize, 7_usize, Move(_7), 10_usize, Move(_10), 5_usize, Move(_5), 15_usize, Move(_15)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_18 = dump_var(19_usize, 12_usize, Move(_12), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{29c90}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box((-3_i8)), std::hint::black_box((-21553_i16)), std::hint::black_box(209_u8), std::hint::black_box(52303_u16), std::hint::black_box(53979386078064673428402113269951053366_u128), std::hint::black_box(1894712473183998657_usize));
                
            }
impl PrintFDebug for Adt36{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt36{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt36 {
fld0: i16,
fld1: char,
fld2: *mut u64,
}
impl PrintFDebug for Adt37{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt37::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt37 {
Variant0{
fld0: u8,
fld1: *const u8,
fld2: isize,

},
Variant1{
fld0: f64,
fld1: i64,
fld2: [isize; 4],
fld3: *mut *const u8,

}}
impl PrintFDebug for Adt38{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt38{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt38 {
fld0: (i32, f32, (u16,)),
fld1: *const u8,
fld2: u16,
}
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
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: (char, i16),
fld1: u128,
fld2: *mut u64,
fld3: i8,
fld4: u8,
fld5: (i64,),
fld6: i64,
fld7: i128,

},
Variant1{
fld0: u16,
fld1: (char, i16),
fld2: isize,
fld3: f32,
fld4: [bool; 7],
fld5: Adt37,
fld6: u32,
fld7: i128,

}}
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
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: (char, i16),
fld1: *mut isize,

},
Variant1{
fld0: bool,
fld1: f64,
fld2: (*const u8, [bool; 7]),
fld3: u16,
fld4: (u128, [isize; 4], (i16, f32)),

},
Variant2{
fld0: i16,
fld1: (u16,),
fld2: isize,

},
Variant3{
fld0: bool,
fld1: usize,
fld2: (*mut isize, i16, i32, f64),
fld3: (*const u8, [bool; 7]),
fld4: Adt38,

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt41{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt41 {
fld0: f64,
fld1: usize,
}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: [bool; 7],
fld1: char,
fld2: *const [isize; 4],
fld3: u32,
fld4: Adt36,
fld5: (i64,),
fld6: i64,

},
Variant1{
fld0: Adt39,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt43{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt43 {
fld0: Adt37,
fld1: (*mut isize, i16, i32, f64),
fld2: [i8; 8],
fld3: *mut *const u8,
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: u8,
fld1: Adt40,
fld2: isize,
fld3: [u8; 3],
fld4: Adt41,
fld5: usize,
fld6: i64,

},
Variant1{
fld0: u16,
fld1: *mut *const u8,
fld2: Adt39,
fld3: Adt36,
fld4: i16,

},
Variant2{
fld0: bool,
fld1: char,
fld2: (char, i16),
fld3: *mut u64,
fld4: (i16, f32),
fld5: usize,
fld6: [u8; 3],
fld7: Adt39,

},
Variant3{
fld0: bool,
fld1: *mut u64,
fld2: f64,
fld3: [isize; 4],

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: (*const u8, [bool; 7]),
fld1: [u8; 3],
fld2: i128,
fld3: Adt44,
fld4: [i8; 8],
fld5: (u128, [isize; 4], (i16, f32)),
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: bool,
fld1: Adt38,
fld2: [u8; 3],
fld3: *mut [bool; 7],
fld4: (*mut isize, i16, i32, f64),
fld5: *mut isize,

},
Variant1{
fld0: Adt39,
fld1: [i8; 8],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt47{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt47 {
fld0: (u16,),
fld1: f32,
fld2: Adt43,
}
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: u32,
fld1: [isize; 4],
fld2: (i32, f32, (u16,)),

},
Variant1{
fld0: f32,
fld1: i128,
fld2: (u16,),
fld3: (*const u8, [bool; 7]),
fld4: usize,

},
Variant2{
fld0: Adt47,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: [u8; 3],
fld1: (u128, [isize; 4], (i16, f32)),
fld2: f32,
fld3: *const u8,
fld4: Adt45,

},
Variant1{
fld0: f32,
fld1: char,
fld2: Adt42,
fld3: (i16, f32),
fld4: i128,
fld5: *const u8,
fld6: Adt45,

},
Variant2{
fld0: (u16,),
fld1: *const [isize; 4],

},
Variant3{
fld0: Adt40,

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
fld0: [bool; 7],
fld1: *const [isize; 4],
fld2: Adt47,
fld3: i8,
fld4: u8,
fld5: i32,
fld6: u128,
fld7: (i16, f32),

},
Variant1{
fld0: Adt36,
fld1: char,
fld2: Adt47,

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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: Adt38,
fld1: f32,

},
Variant1{
fld0: [i8; 8],
fld1: f32,
fld2: isize,
fld3: Adt42,

},
Variant2{
fld0: Adt38,
fld1: Adt44,
fld2: (i16, f32),
fld3: usize,
fld4: i16,
fld5: f64,

},
Variant3{
fld0: *const [isize; 4],
fld1: Adt44,
fld2: [u8; 3],
fld3: *mut isize,

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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: (i64,),
fld1: Adt36,
fld2: Adt47,

},
Variant1{
fld0: Adt44,
fld1: (*const u8, [bool; 7]),
fld2: *mut *const u8,
fld3: i8,
fld4: Adt43,
fld5: i32,
fld6: u32,

},
Variant2{
fld0: bool,
fld1: Adt50,
fld2: [bool; 7],
fld3: *mut u64,
fld4: Adt38,

}}

