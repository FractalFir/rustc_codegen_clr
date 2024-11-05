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
pub fn fn0(mut _1: u64,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: u32,mut _7: u128,mut _8: i128,mut _9: usize,mut _10: u8) -> (char,) {
mir! {
type RET = (char,);
let _11: bool;
let _12: Adt55;
let _13: isize;
let _14: [i8; 5];
let _15: isize;
let _16: [i128; 7];
let _17: bool;
let _18: u8;
let _19: u16;
let _20: char;
let _21: bool;
let _22: bool;
let _23: isize;
let _24: f64;
let _25: f64;
let _26: isize;
let _27: (u128,);
let _28: u8;
let _29: isize;
let _30: Adt48;
let _31: f32;
let _32: isize;
let _33: (char,);
let _34: Adt55;
let _35: f64;
let _36: f64;
let _37: *mut i32;
let _38: (u128,);
let _39: (char,);
let _40: i64;
let _41: isize;
let _42: [i128; 2];
let _43: [isize; 6];
let _44: *const bool;
let _45: [u64; 8];
let _46: isize;
let _47: f64;
let _48: [i16; 2];
let _49: [i128; 7];
let _50: ();
let _51: ();
{
_7 = !27551055650201590236892939253960645079_u128;
_2 = '\u{53fc7}';
RET = (_2,);
_4 = 81_i8;
Goto(bb1)
}
bb1 = {
_5 = (-30752_i16);
_6 = !1191485475_u32;
Call(RET.0 = fn1(_2, _2, _4, _7, _4, _4, _2, _2, _2, _5, _4, _2, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = 4130086523313861714_u64;
_7 = 281100556366894620146222598049687030500_u128 & 140991699729620302516112891743298663503_u128;
RET.0 = _2;
_4 = !(-58_i8);
_1 = 6917181300500069558_u64 * 2018923098560633568_u64;
_7 = 53742301474006487355410858478172852444_u128 * 192587023094004945071475089978157327584_u128;
_5 = -(-21391_i16);
_8 = !(-109876480137193108157939891140691803274_i128);
_1 = _4 as u64;
_3 = (-9223372036854775808_isize) & (-83_isize);
Call(_6 = core::intrinsics::transmute(RET.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10 = !183_u8;
RET = (_2,);
_11 = !false;
_11 = !true;
RET.0 = _2;
_7 = 326877368952741235897348307637707545807_u128;
_4 = !(-36_i8);
_14 = [_4,_4,_4,_4,_4];
RET = (_2,);
_7 = 207789268827180630693630491366415394552_u128 - 212445311704439763182463417820436477120_u128;
_1 = 4427135802233060133_u64;
_11 = _7 >= _7;
_6 = RET.0 as u32;
_5 = (-16412_i16) | (-26980_i16);
_11 = false ^ true;
_15 = -_3;
_7 = !322986575976194574588565291225148017936_u128;
_17 = !_11;
_16 = [_8,_8,_8,_8,_8,_8,_8];
RET = (_2,);
_3 = _15 * _15;
_17 = _11;
_11 = !_17;
_7 = _6 as u128;
RET.0 = _2;
_4 = !53_i8;
match _1 {
0 => bb4,
4427135802233060133 => bb6,
_ => bb5
}
}
bb4 = {
_1 = 4130086523313861714_u64;
_7 = 281100556366894620146222598049687030500_u128 & 140991699729620302516112891743298663503_u128;
RET.0 = _2;
_4 = !(-58_i8);
_1 = 6917181300500069558_u64 * 2018923098560633568_u64;
_7 = 53742301474006487355410858478172852444_u128 * 192587023094004945071475089978157327584_u128;
_5 = -(-21391_i16);
_8 = !(-109876480137193108157939891140691803274_i128);
_1 = _4 as u64;
_3 = (-9223372036854775808_isize) & (-83_isize);
Call(_6 = core::intrinsics::transmute(RET.0), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_5 = (-30752_i16);
_6 = !1191485475_u32;
Call(RET.0 = fn1(_2, _2, _4, _7, _4, _4, _2, _2, _2, _5, _4, _2, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_11 = _17 ^ _17;
_17 = _15 >= _3;
_15 = _3;
_8 = (-39516505264547803522189234842715237245_i128);
_18 = _10;
_1 = 6187884623984967830_u64 - 11016368263423803113_u64;
_2 = RET.0;
_18 = _10;
_17 = _11 == _11;
RET.0 = _2;
_19 = 61533_u16 + 42290_u16;
Goto(bb7)
}
bb7 = {
_2 = RET.0;
_7 = 16457029742875279753225863595303448625_u128;
_20 = RET.0;
_18 = !_10;
_5 = (-9753_i16);
_21 = _17 >= _11;
RET.0 = _2;
_1 = !3856106362130447361_u64;
_13 = _7 as isize;
_7 = 252298504328127133210668259729840084939_u128;
_13 = _3 - _15;
_25 = 1787983241_i32 as f64;
_16 = [_8,_8,_8,_8,_8,_8,_8];
RET.0 = _20;
Call(_24 = core::intrinsics::fmaf64(_25, _25, _25), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_9 = 16619743654493334729_usize;
_4 = 116_i8;
_2 = RET.0;
_23 = _25 as isize;
_24 = (-3428312200578925858_i64) as f64;
_5 = (-5770_i16);
_27 = (_7,);
_10 = !_18;
_22 = !_21;
_6 = _8 as u32;
RET = (_2,);
_11 = _21;
_27.0 = _7 >> _9;
_17 = !_22;
_22 = _11 <= _21;
_21 = _17 ^ _22;
_28 = !_10;
_32 = !_23;
_1 = !17565644706731744797_u64;
_16 = [_8,_8,_8,_8,_8,_8,_8];
Goto(bb9)
}
bb9 = {
_29 = _3 & _13;
_26 = _3;
_11 = _17 == _21;
_10 = _8 as u8;
_21 = _11;
_32 = !_29;
_18 = _28;
_1 = _8 as u64;
_11 = _21;
_2 = _20;
_17 = _11;
_21 = _17;
_11 = _17;
_32 = !_13;
RET = (_20,);
_5 = (-18142_i16) ^ (-17927_i16);
_21 = !_11;
match _9 {
16619743654493334729 => bb11,
_ => bb10
}
}
bb10 = {
_5 = (-30752_i16);
_6 = !1191485475_u32;
Call(RET.0 = fn1(_2, _2, _4, _7, _4, _4, _2, _2, _2, _5, _4, _2, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_33 = RET;
_24 = _4 as f64;
_15 = -_13;
_21 = _17 == _17;
_22 = !_21;
_21 = _22;
RET.0 = _2;
_33 = (_20,);
_9 = 6_usize;
match _9 {
6 => bb12,
_ => bb2
}
}
bb12 = {
_18 = _10 | _28;
RET = (_2,);
_24 = _25 + _25;
_9 = _17 as usize;
_18 = _10 & _10;
_36 = _25;
_35 = _36;
_13 = _3 | _15;
_10 = _19 as u8;
_5 = (-12726_i16) * (-20142_i16);
_17 = _11 >= _22;
_8 = _1 as i128;
_36 = _19 as f64;
RET.0 = _20;
_21 = !_11;
_26 = _15 ^ _13;
_5 = -14565_i16;
_16 = [_8,_8,_8,_8,_8,_8,_8];
_28 = _18 << _6;
_6 = !969357930_u32;
_24 = -_25;
_28 = _10;
_5 = !3219_i16;
Goto(bb13)
}
bb13 = {
_33.0 = _20;
_29 = -_15;
_38 = (_7,);
_33.0 = RET.0;
_32 = _13 * _26;
_10 = _18;
_39.0 = RET.0;
_11 = !_17;
_10 = !_28;
_33 = _39;
_14 = [_4,_4,_4,_4,_4];
_3 = !_32;
_16 = [_8,_8,_8,_8,_8,_8,_8];
_42 = [_8,_8];
_21 = _17;
_35 = _24;
RET.0 = _33.0;
_33.0 = _20;
_45 = [_1,_1,_1,_1,_1,_1,_1,_1];
_41 = _19 as isize;
_40 = _19 as i64;
_25 = _4 as f64;
_46 = _1 as isize;
_15 = _19 as isize;
Goto(bb14)
}
bb14 = {
_15 = _29 + _26;
_18 = !_10;
_36 = _40 as f64;
_10 = _1 as u8;
_45 = [_1,_1,_1,_1,_1,_1,_1,_1];
_27 = (_7,);
_41 = _26;
_18 = _11 as u8;
_9 = !2_usize;
_35 = _8 as f64;
Goto(bb15)
}
bb15 = {
Call(_50 = dump_var(0_usize, 32_usize, Move(_32), 23_usize, Move(_23), 20_usize, Move(_20), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_50 = dump_var(0_usize, 28_usize, Move(_28), 13_usize, Move(_13), 5_usize, Move(_5), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_50 = dump_var(0_usize, 45_usize, Move(_45), 6_usize, Move(_6), 40_usize, Move(_40), 27_usize, Move(_27)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_50 = dump_var(0_usize, 39_usize, Move(_39), 9_usize, Move(_9), 14_usize, Move(_14), 17_usize, Move(_17)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_50 = dump_var(0_usize, 1_usize, Move(_1), 51_usize, _51, 51_usize, _51, 51_usize, _51), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: char,mut _2: char,mut _3: i8,mut _4: u128,mut _5: i8,mut _6: i8,mut _7: char,mut _8: char,mut _9: char,mut _10: i16,mut _11: i8,mut _12: char,mut _13: i16) -> char {
mir! {
type RET = char;
let _14: f64;
let _15: usize;
let _16: Adt55;
let _17: i128;
let _18: isize;
let _19: f32;
let _20: isize;
let _21: usize;
let _22: f32;
let _23: isize;
let _24: f64;
let _25: [isize; 6];
let _26: i16;
let _27: (*const usize, i16);
let _28: i16;
let _29: Adt60;
let _30: ([u64; 8], i128, i128, i8, (*const usize, i16));
let _31: u64;
let _32: [usize; 4];
let _33: bool;
let _34: char;
let _35: [i128; 2];
let _36: [i128; 7];
let _37: usize;
let _38: i8;
let _39: bool;
let _40: [u64; 8];
let _41: Adt57;
let _42: isize;
let _43: (&'static char, char, *const usize);
let _44: *const u16;
let _45: i16;
let _46: (u8, isize, i8, i16, f32);
let _47: f32;
let _48: isize;
let _49: [u32; 3];
let _50: ();
let _51: ();
{
_9 = _2;
_5 = !_11;
_5 = _11 + _11;
_9 = _8;
RET = _1;
_5 = !_11;
match _3 {
0 => bb1,
81 => bb3,
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
_1 = RET;
RET = _9;
_10 = _13;
_4 = !215628951665435347709806915715091502744_u128;
_8 = _12;
_8 = _2;
_14 = 10_u8 as f64;
_13 = _10;
_4 = 99291180219089558747918452203670267426_u128;
RET = _9;
_1 = _12;
_11 = _5 >> _13;
_13 = _10 - _10;
_17 = (-3962536925065573206_i64) as i128;
_11 = _3 | _6;
_13 = _10 * _10;
_5 = _3 << _10;
_2 = _7;
_14 = _13 as f64;
_4 = !284186066052936007930511439046294766993_u128;
_8 = _1;
_2 = _7;
_5 = _3 << _11;
_18 = 9223372036854775807_isize + 9223372036854775807_isize;
Goto(bb4)
}
bb4 = {
_11 = _13 as i8;
_17 = (-57421784514981583236910126893645356777_i128) * 133513906556544489959906807683286434910_i128;
_19 = (-4199264399485183401_i64) as f32;
RET = _9;
_11 = false as i8;
_10 = -_13;
_17 = (-128044085454262772379364162991272674695_i128) >> _18;
_3 = _18 as i8;
_14 = _17 as f64;
_3 = _5 - _6;
_6 = _14 as i8;
RET = _2;
_8 = _1;
_8 = _9;
_10 = _13;
_5 = _3;
_6 = -_5;
_6 = -_3;
_7 = RET;
_10 = _13 & _13;
_20 = _18 + _18;
_23 = _20;
_6 = (-8237114751898513343_i64) as i8;
RET = _1;
_17 = -(-107665101384036890771598112174790673713_i128);
Call(_15 = core::intrinsics::bswap(5848033741991081086_usize), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_14 = _20 as f64;
Goto(bb6)
}
bb6 = {
_19 = 80_u8 as f32;
Call(_10 = core::intrinsics::transmute(_13), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_6 = !_11;
_12 = RET;
_21 = !206138328785281462_usize;
_13 = _10;
_22 = _19 * _19;
_5 = _3 * _3;
_6 = _4 as i8;
RET = _7;
_25 = [_23,_23,_23,_20,_20,_23];
_8 = _12;
_10 = _4 as i16;
_8 = _7;
_12 = _9;
RET = _7;
_12 = _2;
_25 = [_20,_20,_23,_20,_20,_20];
_26 = _13;
RET = _8;
_13 = _26 | _26;
_10 = !_13;
_8 = _2;
_26 = _10;
_11 = _5 | _3;
Goto(bb8)
}
bb8 = {
_13 = _21 as i16;
_26 = _5 as i16;
_27.1 = _26;
_18 = _23;
_10 = _26 | _26;
RET = _1;
_9 = _1;
_13 = _5 as i16;
_11 = -_5;
_14 = (-1355326109_i32) as f64;
_7 = RET;
_6 = _10 as i8;
_6 = !_11;
_24 = _14;
_24 = -_14;
_30.4.1 = !_10;
Call(_30.1 = core::intrinsics::bswap(_17), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_4 = 216_u8 as u128;
_19 = _22 + _22;
_30.3 = _3 | _3;
_20 = _23;
_17 = 157587220037199911022195295419044211093_i128;
_15 = !_21;
_6 = _7 as i8;
_17 = (-17139179031848447020951562172704206105_i128) << _13;
_10 = _30.4.1;
_18 = _20 - _20;
_27.1 = _5 as i16;
_32 = [_21,_21,_21,_21];
_32 = [_15,_15,_21,_21];
_27.0 = core::ptr::addr_of!(_21);
_30.3 = -_11;
_27.0 = core::ptr::addr_of!(_21);
_4 = 133374531570927751759990214842535255294_u128;
_25 = [_20,_18,_18,_20,_18,_23];
_13 = _10 >> _30.3;
_3 = _30.3;
_28 = _26 * _30.4.1;
_7 = _2;
Call(_24 = fn2(_30.4.1, _26, _3, _25, _23, _13, _20), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_17 = -(-52092175936751527908840088154443473766_i128);
_8 = _12;
_2 = _8;
_17 = 8490633787242328121464878929770650252_i128 | (-115893777004659643730736807963355978441_i128);
Goto(bb11)
}
bb11 = {
_10 = _15 as i16;
_10 = _30.4.1;
_20 = _18;
_17 = _4 as i128;
_34 = _8;
_27.1 = -_28;
_27.1 = _4 as i16;
_33 = true;
_30.4.1 = _13;
_23 = !_20;
_15 = 4482755832111805878_u64 as usize;
_30.0 = [11023501296842195669_u64,7851502133240560971_u64,14396395253197119835_u64,16632061470697790604_u64,2162785501247868534_u64,9315893368959728481_u64,3620190695289278639_u64,2162257133076528507_u64];
Call(_23 = core::intrinsics::bswap(_18), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_3 = _5;
_30.3 = _3 | _11;
_30.0 = [10103002676442710958_u64,12942015364409675289_u64,14501523497097372294_u64,11446024782605574780_u64,8665452226396228074_u64,15610679399303265476_u64,18372450295709495673_u64,6898421353958312650_u64];
_35 = [_17,_17];
_12 = _8;
_17 = (-132470251513343702849562016722481374562_i128) - (-2922528565513192663444604619733862750_i128);
_38 = _15 as i8;
_27.1 = !_13;
Call(_11 = core::intrinsics::bswap(_30.3), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_30.1 = _17 + _17;
_1 = _34;
_24 = _14;
_37 = _21;
_36 = [_30.1,_30.1,_17,_17,_17,_30.1,_17];
_31 = _30.1 as u64;
_43.1 = RET;
_18 = _20;
_30.2 = _30.4.1 as i128;
_8 = _2;
Call(_41 = fn15(_28, _30.3, _30.2, _25, _5, _13, _28, _28, _27.1, _3, _28, _1, _25, _23, _30.0, _13), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_43.2 = core::ptr::addr_of!(_15);
_31 = _11 as u64;
_22 = 2399693731_u32 as f32;
_30.2 = _30.1;
_34 = RET;
_15 = _37;
_17 = 168_u8 as i128;
_32 = [_37,_37,_37,_15];
_10 = -_26;
_5 = -_30.3;
_35 = [_17,_30.1];
_42 = _18 ^ _23;
_40 = _30.0;
_27.1 = _28 | _28;
_1 = _12;
_36 = [_30.1,_30.1,_30.2,_30.2,_30.2,_30.1,_30.1];
_43.0 = &_43.1;
_15 = _21;
_13 = !_27.1;
Goto(bb15)
}
bb15 = {
Call(_50 = dump_var(1_usize, 15_usize, Move(_15), 26_usize, Move(_26), 40_usize, Move(_40), 36_usize, Move(_36)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_50 = dump_var(1_usize, 7_usize, Move(_7), 20_usize, Move(_20), 2_usize, Move(_2), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_50 = dump_var(1_usize, 3_usize, Move(_3), 4_usize, Move(_4), 31_usize, Move(_31), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_50 = dump_var(1_usize, 8_usize, Move(_8), 25_usize, Move(_25), 32_usize, Move(_32), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i16,mut _2: i16,mut _3: i8,mut _4: [isize; 6],mut _5: isize,mut _6: i16,mut _7: isize) -> f64 {
mir! {
type RET = f64;
let _8: f64;
let _9: i32;
let _10: i128;
let _11: f64;
let _12: f64;
let _13: char;
let _14: isize;
let _15: i64;
let _16: [i64; 8];
let _17: usize;
let _18: Adt55;
let _19: i32;
let _20: [i64; 8];
let _21: (u128,);
let _22: [u32; 3];
let _23: Adt60;
let _24: f32;
let _25: bool;
let _26: *const *const bool;
let _27: (*const i32, i64);
let _28: ();
let _29: ();
{
_3 = 5875224084222512499_i64 as i8;
_2 = _6;
_3 = (-70_i8) * (-83_i8);
_4 = [_5,_5,_7,_5,_7,_5];
_1 = _6;
RET = _3 as f64;
RET = _7 as f64;
_2 = 294164831149978008852413026032979198679_u128 as i16;
_2 = (-1589865143_i32) as i16;
_5 = _7;
_3 = -(-105_i8);
_7 = _5;
_1 = _6 | _6;
_7 = !_5;
_6 = 33725_u16 as i16;
_4 = [_7,_5,_7,_5,_5,_5];
_7 = 29054_u16 as isize;
_8 = -RET;
Goto(bb1)
}
bb1 = {
_3 = 338714050908964848581791341665213945427_u128 as i8;
_3 = 60_i8 >> _1;
_5 = -_7;
_3 = !60_i8;
_6 = _1 + _1;
_6 = _1 ^ _1;
_8 = -RET;
_2 = 8_u8 as i16;
_2 = !_6;
_4 = [_5,_7,_5,_5,_5,_7];
_3 = 6564117196561889440_u64 as i8;
_4 = [_5,_7,_5,_7,_5,_7];
_6 = _3 as i16;
_8 = RET;
_1 = !_2;
_2 = '\u{c597f}' as i16;
RET = -_8;
_1 = -_2;
_4 = [_7,_7,_7,_7,_5,_7];
RET = 3414851062651166003_u64 as f64;
_3 = -(-42_i8);
_3 = (-113_i8);
_2 = 1602567096_u32 as i16;
_4 = [_5,_5,_5,_7,_7,_5];
RET = _8;
_5 = _7;
RET = _8 + _8;
_3 = 40_i8;
RET = 65458065505091671646523315639939012434_u128 as f64;
_5 = 3958913090_u32 as isize;
_8 = RET;
Goto(bb2)
}
bb2 = {
_8 = -RET;
_4 = [_7,_7,_7,_7,_7,_5];
_7 = _5 | _5;
_2 = -_1;
_11 = -_8;
_5 = !_7;
RET = _11;
_6 = _11 as i16;
_5 = -_7;
RET = -_8;
_10 = 84786555012480613409137912191275863663_i128;
_7 = -_5;
_9 = _5 as i32;
Call(_4 = fn3(_5, _5, _5, _6, _7, _5, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = _5;
_2 = _6 & _6;
_15 = 5128718801380519258_i64;
_1 = !_2;
_15 = _3 as i64;
_1 = _2 ^ _6;
_16 = [_15,_15,_15,_15,_15,_15,_15,_15];
_9 = (-1114586530_i32) - 1142125869_i32;
_17 = _3 as usize;
_9 = !1954499953_i32;
_12 = _17 as f64;
_14 = !_5;
_2 = _1;
RET = -_8;
_3 = 110_i8;
_10 = !(-888801576628141990266653826357974028_i128);
_11 = _8 * _8;
_12 = _15 as f64;
match _3 {
110 => bb5,
_ => bb4
}
}
bb4 = {
_8 = -RET;
_4 = [_7,_7,_7,_7,_7,_5];
_7 = _5 | _5;
_2 = -_1;
_11 = -_8;
_5 = !_7;
RET = _11;
_6 = _11 as i16;
_5 = -_7;
RET = -_8;
_10 = 84786555012480613409137912191275863663_i128;
_7 = -_5;
_9 = _5 as i32;
Call(_4 = fn3(_5, _5, _5, _6, _7, _5, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_7 = _5 ^ _14;
_9 = !991192508_i32;
_2 = _1;
_13 = '\u{56716}';
_3 = (-71_i8) ^ 80_i8;
_10 = true as i128;
_17 = _13 as usize;
_2 = _9 as i16;
_7 = _14;
_19 = 117_u8 as i32;
_6 = -_1;
_15 = (-1643168125433357398_i64);
_15 = !677445925710059226_i64;
RET = _11;
_6 = false as i16;
_17 = 6_usize - 12143642665352456228_usize;
RET = _8;
_5 = -_14;
_11 = _8 + _8;
_12 = _8 - _8;
_20 = [_15,_15,_15,_15,_15,_15,_15,_15];
_9 = _19;
_1 = !_2;
_6 = 69519992969820405104132822217858352779_u128 as i16;
_21 = (53438750681036385325194703773181894573_u128,);
match _21.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
53438750681036385325194703773181894573 => bb9,
_ => bb8
}
}
bb6 = {
_8 = -RET;
_4 = [_7,_7,_7,_7,_7,_5];
_7 = _5 | _5;
_2 = -_1;
_11 = -_8;
_5 = !_7;
RET = _11;
_6 = _11 as i16;
_5 = -_7;
RET = -_8;
_10 = 84786555012480613409137912191275863663_i128;
_7 = -_5;
_9 = _5 as i32;
Call(_4 = fn3(_5, _5, _5, _6, _7, _5, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_3 = 338714050908964848581791341665213945427_u128 as i8;
_3 = 60_i8 >> _1;
_5 = -_7;
_3 = !60_i8;
_6 = _1 + _1;
_6 = _1 ^ _1;
_8 = -RET;
_2 = 8_u8 as i16;
_2 = !_6;
_4 = [_5,_7,_5,_5,_5,_7];
_3 = 6564117196561889440_u64 as i8;
_4 = [_5,_7,_5,_7,_5,_7];
_6 = _3 as i16;
_8 = RET;
_1 = !_2;
_2 = '\u{c597f}' as i16;
RET = -_8;
_1 = -_2;
_4 = [_7,_7,_7,_7,_5,_7];
RET = 3414851062651166003_u64 as f64;
_3 = -(-42_i8);
_3 = (-113_i8);
_2 = 1602567096_u32 as i16;
_4 = [_5,_5,_5,_7,_7,_5];
RET = _8;
_5 = _7;
RET = _8 + _8;
_3 = 40_i8;
RET = 65458065505091671646523315639939012434_u128 as f64;
_5 = 3958913090_u32 as isize;
_8 = RET;
Goto(bb2)
}
bb8 = {
_8 = -RET;
_4 = [_7,_7,_7,_7,_7,_5];
_7 = _5 | _5;
_2 = -_1;
_11 = -_8;
_5 = !_7;
RET = _11;
_6 = _11 as i16;
_5 = -_7;
RET = -_8;
_10 = 84786555012480613409137912191275863663_i128;
_7 = -_5;
_9 = _5 as i32;
Call(_4 = fn3(_5, _5, _5, _6, _7, _5, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_8 = RET + _12;
_19 = _9;
RET = -_12;
_24 = _14 as f32;
_1 = -_2;
match _21.0 {
0 => bb6,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb10,
5 => bb11,
6 => bb12,
53438750681036385325194703773181894573 => bb14,
_ => bb13
}
}
bb10 = {
_3 = 338714050908964848581791341665213945427_u128 as i8;
_3 = 60_i8 >> _1;
_5 = -_7;
_3 = !60_i8;
_6 = _1 + _1;
_6 = _1 ^ _1;
_8 = -RET;
_2 = 8_u8 as i16;
_2 = !_6;
_4 = [_5,_7,_5,_5,_5,_7];
_3 = 6564117196561889440_u64 as i8;
_4 = [_5,_7,_5,_7,_5,_7];
_6 = _3 as i16;
_8 = RET;
_1 = !_2;
_2 = '\u{c597f}' as i16;
RET = -_8;
_1 = -_2;
_4 = [_7,_7,_7,_7,_5,_7];
RET = 3414851062651166003_u64 as f64;
_3 = -(-42_i8);
_3 = (-113_i8);
_2 = 1602567096_u32 as i16;
_4 = [_5,_5,_5,_7,_7,_5];
RET = _8;
_5 = _7;
RET = _8 + _8;
_3 = 40_i8;
RET = 65458065505091671646523315639939012434_u128 as f64;
_5 = 3958913090_u32 as isize;
_8 = RET;
Goto(bb2)
}
bb11 = {
_3 = 338714050908964848581791341665213945427_u128 as i8;
_3 = 60_i8 >> _1;
_5 = -_7;
_3 = !60_i8;
_6 = _1 + _1;
_6 = _1 ^ _1;
_8 = -RET;
_2 = 8_u8 as i16;
_2 = !_6;
_4 = [_5,_7,_5,_5,_5,_7];
_3 = 6564117196561889440_u64 as i8;
_4 = [_5,_7,_5,_7,_5,_7];
_6 = _3 as i16;
_8 = RET;
_1 = !_2;
_2 = '\u{c597f}' as i16;
RET = -_8;
_1 = -_2;
_4 = [_7,_7,_7,_7,_5,_7];
RET = 3414851062651166003_u64 as f64;
_3 = -(-42_i8);
_3 = (-113_i8);
_2 = 1602567096_u32 as i16;
_4 = [_5,_5,_5,_7,_7,_5];
RET = _8;
_5 = _7;
RET = _8 + _8;
_3 = 40_i8;
RET = 65458065505091671646523315639939012434_u128 as f64;
_5 = 3958913090_u32 as isize;
_8 = RET;
Goto(bb2)
}
bb12 = {
_8 = -RET;
_4 = [_7,_7,_7,_7,_7,_5];
_7 = _5 | _5;
_2 = -_1;
_11 = -_8;
_5 = !_7;
RET = _11;
_6 = _11 as i16;
_5 = -_7;
RET = -_8;
_10 = 84786555012480613409137912191275863663_i128;
_7 = -_5;
_9 = _5 as i32;
Call(_4 = fn3(_5, _5, _5, _6, _7, _5, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_7 = _5;
_2 = _6 & _6;
_15 = 5128718801380519258_i64;
_1 = !_2;
_15 = _3 as i64;
_1 = _2 ^ _6;
_16 = [_15,_15,_15,_15,_15,_15,_15,_15];
_9 = (-1114586530_i32) - 1142125869_i32;
_17 = _3 as usize;
_9 = !1954499953_i32;
_12 = _17 as f64;
_14 = !_5;
_2 = _1;
RET = -_8;
_3 = 110_i8;
_10 = !(-888801576628141990266653826357974028_i128);
_11 = _8 * _8;
_12 = _15 as f64;
match _3 {
110 => bb5,
_ => bb4
}
}
bb14 = {
_7 = _5 | _14;
_13 = '\u{91348}';
_2 = -_1;
_11 = _8 + RET;
_24 = _1 as f32;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(2_usize, 17_usize, Move(_17), 4_usize, Move(_4), 13_usize, Move(_13), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(2_usize, 1_usize, Move(_1), 20_usize, Move(_20), 21_usize, Move(_21), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: i16,mut _5: isize,mut _6: isize,mut _7: i8) -> [isize; 6] {
mir! {
type RET = [isize; 6];
let _8: Adt60;
let _9: (u128,);
let _10: u16;
let _11: ();
let _12: ();
{
Call(_7 = fn4(_3, _2, _5, _1, _3, _2, _1, _3, _4, _6, _5, _4, _6, _4, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = '\u{58242}' as isize;
RET = [_2,_3,_1,_5,_3,_3];
_4 = 14112_i16;
_9 = (309544196723396633276010389479181884171_u128,);
_9 = (141097592065236677869299463747318115646_u128,);
RET = [_1,_6,_3,_1,_6,_2];
_5 = 138_u8 as isize;
_9 = (277689685997167108176339797203083059554_u128,);
_9.0 = !291753513078069093965942130743035171513_u128;
_9 = (320635008867457275654986124632377110489_u128,);
_4 = 10314_i16 >> _7;
_9 = (80272246763006397941574582549659129535_u128,);
_2 = _6;
_6 = _5;
_10 = 36103_u16;
_2 = _5;
_5 = !_3;
_6 = _1;
_6 = '\u{10b322}' as isize;
_7 = (-82_i8);
_1 = _3 >> _4;
RET = [_1,_1,_1,_2,_1,_1];
RET = [_1,_1,_1,_1,_1,_1];
_10 = 28619_u16 ^ 41931_u16;
Goto(bb2)
}
bb2 = {
Call(_11 = dump_var(3_usize, 9_usize, Move(_9), 1_usize, Move(_1), 3_usize, Move(_3), 7_usize, Move(_7)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: i16,mut _10: isize,mut _11: isize,mut _12: i16,mut _13: isize,mut _14: i16,mut _15: isize) -> i8 {
mir! {
type RET = i8;
let _16: usize;
let _17: f64;
let _18: i128;
let _19: Adt53;
let _20: (char,);
let _21: u16;
let _22: Adt51;
let _23: usize;
let _24: *mut [i128; 2];
let _25: isize;
let _26: f64;
let _27: [u64; 8];
let _28: (u8, isize, i8, i16, f32);
let _29: (u8, isize, i8, i16, f32);
let _30: [i64; 8];
let _31: Adt54;
let _32: Adt58;
let _33: u128;
let _34: (u128,);
let _35: i8;
let _36: (char,);
let _37: *const *const bool;
let _38: [isize; 6];
let _39: [i16; 2];
let _40: bool;
let _41: isize;
let _42: f64;
let _43: ();
let _44: ();
{
_7 = (-89_i8) as isize;
RET = (-266970285_i32) as i8;
_1 = -_7;
_6 = (-182535686_i32) as isize;
_4 = (-64635266967994161658372562840582761780_i128) as isize;
_1 = _5;
RET = !45_i8;
_12 = _14;
_12 = 2702681448196747145_i64 as i16;
_17 = 19937_u16 as f64;
_11 = _15 + _13;
_13 = _10;
_6 = -_11;
RET = (-81_i8) >> _7;
_9 = _14;
Goto(bb1)
}
bb1 = {
_15 = _11 + _11;
_18 = 142040960922622310606990431389077764055_i128;
_14 = _9;
_1 = 6_u8 as isize;
_20.0 = '\u{fd0c1}';
_6 = _13;
_16 = 14527104933598997031_usize >> _15;
RET = !(-43_i8);
_9 = -_14;
RET = (-75_i8) ^ (-27_i8);
_12 = 3823694542765963549_i64 as i16;
_17 = _7 as f64;
_2 = RET as isize;
_16 = 11324059430693880266_u64 as usize;
_26 = -_17;
_27 = [15083725886796813247_u64,991361088336594891_u64,2366281964543336560_u64,7466137672548994922_u64,7333020946149575894_u64,7360074400415136276_u64,18078780316257444097_u64,306896369861428551_u64];
Call(_6 = fn5(_27, _4, _10, _4, _27), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_28.4 = 2126652230_i32 as f32;
_28.1 = -_6;
_1 = _6;
_8 = !_1;
_22 = Adt51::Variant2 { fld0: 30721241097414429792013115759183821487_u128 };
_28.2 = RET & RET;
_28.3 = _12 & _9;
_3 = _28.1 >> _8;
place!(Field::<u128>(Variant(_22, 2), 0)) = 44734648_u32 as u128;
RET = _28.2;
_11 = 47011_u16 as isize;
_27 = [3420978579291614952_u64,4358827124867630555_u64,3040090052913761144_u64,416711891507811047_u64,7322932021530817258_u64,2063135228338581839_u64,13755372482228102274_u64,5305023331703163029_u64];
_29.2 = !RET;
RET = _29.2;
Call(_16 = core::intrinsics::bswap(4_usize), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
SetDiscriminant(_22, 0);
_26 = _16 as f64;
_2 = _28.2 as isize;
_15 = _3 | _8;
_13 = _3 - _6;
_21 = 49_u8 as u16;
place!(Field::<isize>(Variant(_22, 0), 2)) = _15;
_16 = !7_usize;
_26 = _17;
_29.0 = _26 as u8;
place!(Field::<i16>(Variant(_22, 0), 4)) = Field::<isize>(Variant(_22, 0), 2) as i16;
_15 = _26 as isize;
_23 = _16 << _8;
_21 = Field::<isize>(Variant(_22, 0), 2) as u16;
_29 = (14_u8, _1, _28.2, Field::<i16>(Variant(_22, 0), 4), _28.4);
_1 = _3 - _8;
_21 = 28586_u16;
match _29.0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
14 => bb10,
_ => bb9
}
}
bb4 = {
_28.4 = 2126652230_i32 as f32;
_28.1 = -_6;
_1 = _6;
_8 = !_1;
_22 = Adt51::Variant2 { fld0: 30721241097414429792013115759183821487_u128 };
_28.2 = RET & RET;
_28.3 = _12 & _9;
_3 = _28.1 >> _8;
place!(Field::<u128>(Variant(_22, 2), 0)) = 44734648_u32 as u128;
RET = _28.2;
_11 = 47011_u16 as isize;
_27 = [3420978579291614952_u64,4358827124867630555_u64,3040090052913761144_u64,416711891507811047_u64,7322932021530817258_u64,2063135228338581839_u64,13755372482228102274_u64,5305023331703163029_u64];
_29.2 = !RET;
RET = _29.2;
Call(_16 = core::intrinsics::bswap(4_usize), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_15 = _11 + _11;
_18 = 142040960922622310606990431389077764055_i128;
_14 = _9;
_1 = 6_u8 as isize;
_20.0 = '\u{fd0c1}';
_6 = _13;
_16 = 14527104933598997031_usize >> _15;
RET = !(-43_i8);
_9 = -_14;
RET = (-75_i8) ^ (-27_i8);
_12 = 3823694542765963549_i64 as i16;
_17 = _7 as f64;
_2 = RET as isize;
_16 = 11324059430693880266_u64 as usize;
_26 = -_17;
_27 = [15083725886796813247_u64,991361088336594891_u64,2366281964543336560_u64,7466137672548994922_u64,7333020946149575894_u64,7360074400415136276_u64,18078780316257444097_u64,306896369861428551_u64];
Call(_6 = fn5(_27, _4, _10, _4, _27), ReturnTo(bb2), UnwindUnreachable())
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
_33 = !290722540227872355950465713768952271997_u128;
_18 = -(-135895652814669126427553379580465555829_i128);
place!(Field::<[i8; 5]>(Variant(_22, 0), 3)) = [_28.2,_28.2,RET,_29.2,_28.2];
_19 = Adt53::Variant2 { fld0: _23,fld1: _20.0 };
place!(Field::<char>(Variant(_22, 0), 1)) = Field::<char>(Variant(_19, 2), 1);
_14 = _29.3 + Field::<i16>(Variant(_22, 0), 4);
_28.1 = Field::<isize>(Variant(_22, 0), 2);
_1 = -_3;
_17 = -_26;
_30 = [(-2508523899672840715_i64),1856960866342349860_i64,944729291843609460_i64,(-2675719852094445497_i64),8668805251720735551_i64,9124302448800212197_i64,6643834867080320662_i64,654433215749468168_i64];
_9 = _29.3 << _3;
_28 = (_29.0, _3, RET, _14, _29.4);
_12 = Field::<i16>(Variant(_22, 0), 4);
_29.3 = -_9;
_27 = [18418977668027702691_u64,8032639002517316047_u64,8525184016216171646_u64,6609337428799614261_u64,13216654702978359632_u64,9987917180477380763_u64,4194489705096440727_u64,8126591949163316373_u64];
_34 = (_33,);
_22 = Adt51::Variant2 { fld0: _33 };
_11 = _13;
_34 = (Field::<u128>(Variant(_22, 2), 0),);
Goto(bb11)
}
bb11 = {
_10 = -_1;
_35 = -_28.2;
place!(Field::<usize>(Variant(_19, 2), 0)) = !_23;
_36.0 = _20.0;
_2 = _1;
_28.4 = Field::<usize>(Variant(_19, 2), 0) as f32;
_9 = _12;
_28.2 = _29.0 as i8;
_26 = _17 + _17;
_21 = 11452_u16 | 37687_u16;
_28.3 = _28.0 as i16;
_29.3 = -_14;
_30 = [2436126661399166955_i64,4799030366302954945_i64,(-8186964460854630235_i64),(-3091849561732923276_i64),(-2781982755211277258_i64),3241072880465350320_i64,(-4648887862436385213_i64),(-6128610708250568503_i64)];
SetDiscriminant(_22, 2);
_18 = 71016192370137392582097038089177999326_i128;
_1 = _28.1 - _2;
_22 = Adt51::Variant2 { fld0: _33 };
_20 = (_36.0,);
RET = _28.2;
_8 = _3;
Goto(bb12)
}
bb12 = {
Call(_43 = dump_var(4_usize, 15_usize, Move(_15), 36_usize, Move(_36), 4_usize, Move(_4), 18_usize, Move(_18)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_43 = dump_var(4_usize, 16_usize, Move(_16), 21_usize, Move(_21), 20_usize, Move(_20), 14_usize, Move(_14)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_43 = dump_var(4_usize, 5_usize, Move(_5), 8_usize, Move(_8), 7_usize, Move(_7), 9_usize, Move(_9)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_43 = dump_var(4_usize, 23_usize, Move(_23), 44_usize, _44, 44_usize, _44, 44_usize, _44), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: [u64; 8],mut _2: isize,mut _3: isize,mut _4: isize,mut _5: [u64; 8]) -> isize {
mir! {
type RET = isize;
let _6: Adt52;
let _7: ([u64; 8], i128, i128, i8, (*const usize, i16));
let _8: isize;
let _9: isize;
let _10: f64;
let _11: *mut [i128; 2];
let _12: Adt58;
let _13: (u8, isize, i8, i16, f32);
let _14: char;
let _15: isize;
let _16: f32;
let _17: (u64, [i8; 5], usize, i16, *const i32);
let _18: u128;
let _19: char;
let _20: i32;
let _21: [usize; 4];
let _22: i64;
let _23: Adt47;
let _24: f64;
let _25: char;
let _26: f32;
let _27: [u64; 8];
let _28: ();
let _29: ();
{
_5 = [14909592865193229478_u64,13636012318731870017_u64,7054852722834444347_u64,7589472944842621300_u64,5625941793328858855_u64,15364726739040926614_u64,11053041386881818056_u64,6501204009539250084_u64];
_1 = [6440357292985531848_u64,7611056225933665662_u64,16603986890970139827_u64,3378016516531702279_u64,12049850844821148718_u64,5577494499638899411_u64,6089271687449125379_u64,3877446795257278824_u64];
_2 = _3;
_5 = [8859219390246628876_u64,9797389217999757444_u64,16832667582223282040_u64,5869016761120052085_u64,816265219670587362_u64,4915894607949952849_u64,15682437874862590388_u64,11881512469125231952_u64];
RET = _3;
RET = !_3;
_2 = 275622453016528879603063954183258192862_u128 as isize;
_3 = RET * _4;
_2 = RET;
Goto(bb1)
}
bb1 = {
_2 = RET;
_1 = _5;
_2 = RET & _3;
RET = _4;
RET = 309583954260459221002918866927742609169_u128 as isize;
_7.2 = 12219_u16 as i128;
_7.0 = _5;
RET = _3;
RET = (-1968360160_i32) as isize;
RET = (-286819041_i32) as isize;
_1 = _7.0;
_8 = false as isize;
Goto(bb2)
}
bb2 = {
_7.4.1 = !14318_i16;
_7.3 = (-1_i8);
_7.1 = -_7.2;
RET = _7.3 as isize;
_9 = -_2;
_8 = _2 & _3;
Call(_7.0 = fn6(_9, _5, _5, _7.4.1, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = _3;
_7.2 = _7.1;
_5 = _1;
_8 = !_9;
_7.2 = _7.4.1 as i128;
_2 = _9;
_8 = 9155513995998315192_i64 as isize;
_2 = _7.4.1 as isize;
_7.0 = [2012988828389721335_u64,13318124576842279421_u64,7436588254319351717_u64,9362803386134513561_u64,3289915993553413403_u64,7930959159759899978_u64,11052992748831005226_u64,11119612552887054268_u64];
_3 = _9;
_7.0 = [4724873752339490035_u64,6485999258494655864_u64,12678909336664239137_u64,6628581096338200117_u64,7083957138261855110_u64,5882423738589478543_u64,13344280519227010398_u64,8062431004620905218_u64];
RET = (-1852632649_i32) as isize;
_7.2 = _2 as i128;
RET = _9;
_7.3 = (-39_i8) & 8_i8;
_10 = _7.4.1 as f64;
_7.3 = _10 as i8;
Call(_7 = fn8(_3, _3, RET, _9, _5, _3, _3, _9, _3, _2, _3, RET, _3, RET, _5, _2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_2 = _7.4.1 as isize;
_10 = 743806698_u32 as f64;
_13.0 = 113_u8 >> _7.2;
_13.1 = _3 - _3;
_7.3 = RET as i8;
_13.3 = _13.0 as i16;
_13.4 = 2559051561_u32 as f32;
_13.2 = _10 as i8;
_3 = _7.1 as isize;
_7.3 = _13.2 * _13.2;
_13.2 = _7.3 + _7.3;
_7.1 = 623865192901675661_i64 as i128;
_2 = _9;
_2 = -_3;
_5 = _7.0;
_14 = '\u{a6fdd}';
_7.0 = [10841016879793789594_u64,427506387026176885_u64,7400175852761180078_u64,18302779821872558845_u64,1868021822597467083_u64,9095940701592910588_u64,1168565755549031290_u64,12609942963761953153_u64];
_10 = _7.2 as f64;
_7.3 = -_13.2;
_8 = _2;
_7.3 = _13.2;
_4 = _2;
_8 = _2 ^ _4;
Call(_15 = fn14(_13.0, _13.0, _8, _5, _7, _4, _4, _13, _7), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_7.4.0 = core::ptr::addr_of!(_17.2);
_3 = _15 & _8;
_17.0 = 6220621951730715659_u64;
_17.1 = [_7.3,_7.3,_7.3,_13.2,_13.2];
_17.2 = 3442915037465546806_usize << _15;
_13.3 = _7.4.1;
_13.2 = -_7.3;
_7.1 = -_7.2;
_5 = [_17.0,_17.0,_17.0,_17.0,_17.0,_17.0,_17.0,_17.0];
_16 = -_13.4;
_7.4.0 = core::ptr::addr_of!(_17.2);
_17.2 = 623322489309664047_usize;
_14 = '\u{85e6a}';
_13.1 = _8 << _3;
_13.4 = -_16;
_7.2 = _7.1 << _13.1;
_7.0 = _1;
_18 = !235299876536603269657298991758582797473_u128;
_7.0 = _1;
Goto(bb6)
}
bb6 = {
_8 = _4;
_20 = (-707504622_i32) + 1817880848_i32;
_16 = _20 as f32;
_5 = [_17.0,_17.0,_17.0,_17.0,_17.0,_17.0,_17.0,_17.0];
Goto(bb7)
}
bb7 = {
_13.2 = _7.3;
_17.3 = _13.3;
_13 = (92_u8, _4, _7.3, _17.3, _16);
_7.0 = _1;
_7.1 = !_7.2;
RET = _4;
_14 = '\u{e9815}';
RET = _3;
_13.0 = _18 as u8;
_18 = 114941592942320860336662338518719223195_u128 - 330120507931628257498998945608071831506_u128;
_7.1 = _20 as i128;
_17.3 = _7.4.1 ^ _13.3;
_19 = _14;
_7.3 = _13.2;
_17.3 = !_13.3;
_13.2 = _7.3;
_17.2 = 4481296112535821110_i64 as usize;
_17.1 = [_13.2,_13.2,_13.2,_13.2,_7.3];
_10 = (-1353115721965475547_i64) as f64;
_3 = _2;
_7.4.1 = -_13.3;
_13.0 = 69_u8;
_7.3 = _13.2 ^ _13.2;
_17.0 = !4948281869086454362_u64;
Goto(bb8)
}
bb8 = {
Call(_28 = dump_var(5_usize, 19_usize, Move(_19), 9_usize, Move(_9), 8_usize, Move(_8), 20_usize, Move(_20)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_28 = dump_var(5_usize, 3_usize, Move(_3), 4_usize, Move(_4), 29_usize, _29, 29_usize, _29), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: isize,mut _2: [u64; 8],mut _3: [u64; 8],mut _4: i16,mut _5: isize) -> [u64; 8] {
mir! {
type RET = [u64; 8];
let _6: [usize; 4];
let _7: f32;
let _8: (u8, isize, i8, i16, f32);
let _9: bool;
let _10: f64;
let _11: bool;
let _12: [i64; 8];
let _13: char;
let _14: Adt51;
let _15: bool;
let _16: [i64; 8];
let _17: *mut i32;
let _18: [i16; 2];
let _19: [i128; 2];
let _20: f32;
let _21: [isize; 6];
let _22: [i8; 5];
let _23: f64;
let _24: (char,);
let _25: u128;
let _26: f32;
let _27: f64;
let _28: (u128,);
let _29: *const u16;
let _30: (u64, [i8; 5], usize, i16, *const i32);
let _31: char;
let _32: [i128; 7];
let _33: i16;
let _34: char;
let _35: [isize; 6];
let _36: (char,);
let _37: i64;
let _38: Adt57;
let _39: f32;
let _40: [isize; 6];
let _41: Adt57;
let _42: i128;
let _43: ();
let _44: ();
{
RET = _2;
_1 = _5 + _5;
_3 = _2;
Goto(bb1)
}
bb1 = {
_3 = [16598332905769136997_u64,14074708920688419233_u64,10255656895680631101_u64,14821833545475871588_u64,5865484070712438839_u64,2379750480313874590_u64,13191380193763503068_u64,5611614485298310380_u64];
_3 = [15346313443401570892_u64,6803771143793606959_u64,10299724450644828583_u64,3743484251022107193_u64,8447125080165197636_u64,13367669552484231172_u64,7672789286512500711_u64,15694480984757710231_u64];
_1 = _5;
RET = _2;
RET = [13008705035726338942_u64,5591271078746481491_u64,14473199176392836822_u64,8981453129975638210_u64,10214236870731038567_u64,17734614900547115511_u64,4728301060991296959_u64,1757994510601504164_u64];
_2 = [2268149025557741491_u64,10897040835631502963_u64,2530574215388724334_u64,14061422918596539420_u64,7016773344739854335_u64,14627018533553909696_u64,7019649363050737541_u64,13212329418532053372_u64];
_5 = _1 - _1;
_1 = _5;
_3 = [14283792242314437246_u64,4652708975881785891_u64,14233756486901795425_u64,9828488901139888721_u64,15772173456369303145_u64,1877493177615154530_u64,10198789066610893477_u64,17195057266645823533_u64];
_6 = [4_usize,1_usize,3_usize,3_usize];
_2 = [13764743828707151204_u64,1415391526614157628_u64,3261478485990008216_u64,751543098928474661_u64,7024739895163154132_u64,17959248038762629441_u64,5417739624909311455_u64,18099244555921670350_u64];
_6 = [7343093749561735110_usize,7_usize,1767539883030710028_usize,1_usize];
RET = [5707725680141031067_u64,8104521823548586157_u64,16145154562563486548_u64,11334271820240857696_u64,11538127584488656521_u64,9812376078866065995_u64,17649953324879706700_u64,11060131500505920238_u64];
_5 = 18147274863557486695_u64 as isize;
_2 = [17601003729337716434_u64,12293648716049334199_u64,9234002642397111167_u64,4471998180657029448_u64,13046095873331465792_u64,439180701377714044_u64,9222242704945449080_u64,3884730315169003540_u64];
Goto(bb2)
}
bb2 = {
_2 = [12828022898305537453_u64,8359087970113785099_u64,3843838041249615501_u64,17950008711495432258_u64,492648017352593006_u64,4173118265293226382_u64,5770775236280560880_u64,5274611457554398880_u64];
_8.3 = _4;
Goto(bb3)
}
bb3 = {
RET = [15253474944824276265_u64,15279837643685187408_u64,3552410897824905250_u64,14609475459395463812_u64,8379876910240055979_u64,14871638404880389252_u64,14527748221888100570_u64,12115259809259391984_u64];
_8.4 = _1 as f32;
_8.1 = !_1;
_8.2 = !(-97_i8);
Goto(bb4)
}
bb4 = {
_8.4 = (-118674866346429491481356445122902178403_i128) as f32;
_8.0 = 149_u8;
_7 = _8.4;
RET = _3;
_1 = _8.1 | _5;
_2 = [13288677730050466688_u64,10575013198790901374_u64,14214107285465422287_u64,17892936072942897092_u64,9984922808999270784_u64,15167403648695520705_u64,512728493980925520_u64,10112282902358653710_u64];
Call(_6 = fn7(_8, RET, _1, _1, _8.3, _1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_8.1 = _1;
_8.0 = 8308_u16 as u8;
_8.0 = 63_u8 ^ 25_u8;
_7 = _8.4;
_8.4 = _7 + _7;
_9 = false ^ false;
_4 = !_8.3;
_5 = -_8.1;
_1 = _8.3 as isize;
_1 = _8.4 as isize;
_7 = -_8.4;
_4 = _8.0 as i16;
_11 = _8.1 != _5;
RET = [9674493234726395277_u64,11514393274044296307_u64,2724087268887188666_u64,15902216181935205701_u64,9848132481858594829_u64,15331346176667953565_u64,16535706992614813215_u64,10972139436862477097_u64];
Goto(bb6)
}
bb6 = {
_8 = (18_u8, _5, 81_i8, _4, _7);
_12 = [(-7232976703597322765_i64),3414155318100150054_i64,6263268966846305287_i64,(-1671192476588041439_i64),9216712230665288778_i64,(-6547089700525609409_i64),(-6672578838907292439_i64),6208194145919069056_i64];
_13 = '\u{d6bc6}';
_8.4 = _7 * _7;
_2 = [12749814177379094178_u64,12091818521515456344_u64,15667449081395397470_u64,16940199204431297032_u64,928921145550795347_u64,6575461293499032136_u64,12981492936912521202_u64,17145219864511736956_u64];
_15 = _11 ^ _11;
_8 = (169_u8, _5, (-47_i8), _4, _7);
_8.2 = (-98_i8) - 28_i8;
_8.4 = _7;
_13 = '\u{102c0f}';
_2 = [1423040457471603121_u64,2847325455333335210_u64,2996648613115812152_u64,17080029745078402260_u64,2496069238645465904_u64,12535934786539019752_u64,6242118853245061791_u64,18231205234278506804_u64];
_10 = 16193206399226258981_u64 as f64;
_5 = _8.1 >> _8.0;
_8.0 = 9_u8;
_5 = !_8.1;
RET = _3;
_8.0 = !5_u8;
_16 = _12;
_8.2 = _8.3 as i8;
Goto(bb7)
}
bb7 = {
_14 = Adt51::Variant2 { fld0: 297891420780918920049899136846713992499_u128 };
RET = [12690667534358895403_u64,4904365978809689797_u64,6065236542462490253_u64,8667901336143194987_u64,141674891256530151_u64,13300879970242756290_u64,8710705691203568570_u64,11977407722934451074_u64];
_12 = _16;
_11 = !_15;
place!(Field::<u128>(Variant(_14, 2), 0)) = 68683316359597541087505719230092895799_u128;
_10 = _4 as f64;
_5 = -_8.1;
place!(Field::<u128>(Variant(_14, 2), 0)) = (-1495517985_i32) as u128;
_15 = _11 != _11;
_14 = Adt51::Variant2 { fld0: 163701233449719705233402906295488332501_u128 };
_8.1 = _1;
_12 = [(-7016667843807987041_i64),(-1663008269833064016_i64),(-3513913334953526542_i64),8224982805560615224_i64,6553198674737014277_i64,(-4866852737456337854_i64),4115136559491844756_i64,(-8510373241986024891_i64)];
_7 = _8.0 as f32;
_19 = [(-106702421790401262130136936212224653605_i128),169776715734958758529525238135219501505_i128];
_15 = _11;
Goto(bb8)
}
bb8 = {
_2 = [12249083901769823199_u64,9846717669260885915_u64,6102144690053672839_u64,10050498255580181884_u64,18032691091594313312_u64,5928770619061811387_u64,482255039218692777_u64,1682340650278811278_u64];
_9 = !_15;
_1 = 52230497272587921324033412379682570745_i128 as isize;
place!(Field::<u128>(Variant(_14, 2), 0)) = !104145151100784071518263673397890872097_u128;
_8 = (4_u8, _5, 15_i8, _4, _7);
_23 = _10;
place!(Field::<u128>(Variant(_14, 2), 0)) = !244168151046792730901151656345535123674_u128;
_6 = [3441200307174411285_usize,3_usize,543734083472654921_usize,2697349433638074337_usize];
RET = _2;
_5 = _8.1 ^ _8.1;
_24.0 = _13;
Goto(bb9)
}
bb9 = {
_21 = [_8.1,_5,_8.1,_5,_8.1,_5];
_8.4 = _7;
_25 = Field::<u128>(Variant(_14, 2), 0);
_8.1 = _5 >> _5;
_16 = _12;
_16 = [8227057868500180480_i64,5739238738360408028_i64,(-2718073565400911561_i64),(-6064440709470181759_i64),2569960215164126052_i64,7133250543193272432_i64,6283976007804168170_i64,(-23267982402401774_i64)];
_8.1 = 878447324_i32 as isize;
_21 = [_5,_5,_5,_5,_5,_5];
SetDiscriminant(_14, 0);
place!(Field::<isize>(Variant(_14, 0), 2)) = (-4482748819365288503_i64) as isize;
_2 = [8913556399885669537_u64,4482816229946744427_u64,14158372271840931645_u64,6306795111336071136_u64,11939691006067066071_u64,14082920470981624390_u64,17945247060973929672_u64,15332755489044796101_u64];
place!(Field::<[i8; 5]>(Variant(_14, 0), 3)) = [_8.2,_8.2,_8.2,_8.2,_8.2];
_8.1 = -_5;
_16 = _12;
place!(Field::<char>(Variant(_14, 0), 1)) = _24.0;
_8.2 = _8.1 as i8;
_9 = _11;
Call(_1 = core::intrinsics::transmute(_8.1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_3 = [9318888304260500682_u64,8271733573169249621_u64,9896385531160813093_u64,16179938509719716620_u64,14627168337584986614_u64,13199124512668214404_u64,16103712285198527691_u64,275338451303058445_u64];
_27 = 0_usize as f64;
Goto(bb11)
}
bb11 = {
_11 = !_9;
_26 = 3703852709_u32 as f32;
_30.3 = _8.3 & _8.3;
Goto(bb12)
}
bb12 = {
_16 = [(-1180166756119237144_i64),6354140945833975284_i64,4798706373827667501_i64,(-1557625289919477790_i64),(-7401894677495626846_i64),(-2195759191895399087_i64),(-8210688865105625431_i64),(-2149449346166243169_i64)];
_10 = _23 - _23;
_31 = _24.0;
_32 = [(-10401174019723893772764347108857154952_i128),30764055809313248475367816195345583129_i128,(-88063606283689006444049908663850592209_i128),36979976994169128808543720101138734787_i128,(-32350484751605136910311661447784207324_i128),32418654679161038606355243497153955680_i128,84953204279374237778214562987487360227_i128];
place!(Field::<char>(Variant(_14, 0), 1)) = _24.0;
_8.1 = _1 * _1;
_22 = [_8.2,_8.2,_8.2,_8.2,_8.2];
_24.0 = Field::<char>(Variant(_14, 0), 1);
Goto(bb13)
}
bb13 = {
_20 = _7;
_1 = _8.1;
_8.3 = !_30.3;
_20 = _8.0 as f32;
_18 = [_8.3,_30.3];
_23 = _10;
match _8.0 {
0 => bb1,
1 => bb3,
2 => bb14,
3 => bb15,
5 => bb17,
4 => bb19,
_ => bb18
}
}
bb14 = {
_3 = [16598332905769136997_u64,14074708920688419233_u64,10255656895680631101_u64,14821833545475871588_u64,5865484070712438839_u64,2379750480313874590_u64,13191380193763503068_u64,5611614485298310380_u64];
_3 = [15346313443401570892_u64,6803771143793606959_u64,10299724450644828583_u64,3743484251022107193_u64,8447125080165197636_u64,13367669552484231172_u64,7672789286512500711_u64,15694480984757710231_u64];
_1 = _5;
RET = _2;
RET = [13008705035726338942_u64,5591271078746481491_u64,14473199176392836822_u64,8981453129975638210_u64,10214236870731038567_u64,17734614900547115511_u64,4728301060991296959_u64,1757994510601504164_u64];
_2 = [2268149025557741491_u64,10897040835631502963_u64,2530574215388724334_u64,14061422918596539420_u64,7016773344739854335_u64,14627018533553909696_u64,7019649363050737541_u64,13212329418532053372_u64];
_5 = _1 - _1;
_1 = _5;
_3 = [14283792242314437246_u64,4652708975881785891_u64,14233756486901795425_u64,9828488901139888721_u64,15772173456369303145_u64,1877493177615154530_u64,10198789066610893477_u64,17195057266645823533_u64];
_6 = [4_usize,1_usize,3_usize,3_usize];
_2 = [13764743828707151204_u64,1415391526614157628_u64,3261478485990008216_u64,751543098928474661_u64,7024739895163154132_u64,17959248038762629441_u64,5417739624909311455_u64,18099244555921670350_u64];
_6 = [7343093749561735110_usize,7_usize,1767539883030710028_usize,1_usize];
RET = [5707725680141031067_u64,8104521823548586157_u64,16145154562563486548_u64,11334271820240857696_u64,11538127584488656521_u64,9812376078866065995_u64,17649953324879706700_u64,11060131500505920238_u64];
_5 = 18147274863557486695_u64 as isize;
_2 = [17601003729337716434_u64,12293648716049334199_u64,9234002642397111167_u64,4471998180657029448_u64,13046095873331465792_u64,439180701377714044_u64,9222242704945449080_u64,3884730315169003540_u64];
Goto(bb2)
}
bb15 = {
_8.4 = (-118674866346429491481356445122902178403_i128) as f32;
_8.0 = 149_u8;
_7 = _8.4;
RET = _3;
_1 = _8.1 | _5;
_2 = [13288677730050466688_u64,10575013198790901374_u64,14214107285465422287_u64,17892936072942897092_u64,9984922808999270784_u64,15167403648695520705_u64,512728493980925520_u64,10112282902358653710_u64];
Call(_6 = fn7(_8, RET, _1, _1, _8.3, _1), ReturnTo(bb5), UnwindUnreachable())
}
bb16 = {
_3 = [9318888304260500682_u64,8271733573169249621_u64,9896385531160813093_u64,16179938509719716620_u64,14627168337584986614_u64,13199124512668214404_u64,16103712285198527691_u64,275338451303058445_u64];
_27 = 0_usize as f64;
Goto(bb11)
}
bb17 = {
RET = [15253474944824276265_u64,15279837643685187408_u64,3552410897824905250_u64,14609475459395463812_u64,8379876910240055979_u64,14871638404880389252_u64,14527748221888100570_u64,12115259809259391984_u64];
_8.4 = _1 as f32;
_8.1 = !_1;
_8.2 = !(-97_i8);
Goto(bb4)
}
bb18 = {
_8.1 = _1;
_8.0 = 8308_u16 as u8;
_8.0 = 63_u8 ^ 25_u8;
_7 = _8.4;
_8.4 = _7 + _7;
_9 = false ^ false;
_4 = !_8.3;
_5 = -_8.1;
_1 = _8.3 as isize;
_1 = _8.4 as isize;
_7 = -_8.4;
_4 = _8.0 as i16;
_11 = _8.1 != _5;
RET = [9674493234726395277_u64,11514393274044296307_u64,2724087268887188666_u64,15902216181935205701_u64,9848132481858594829_u64,15331346176667953565_u64,16535706992614813215_u64,10972139436862477097_u64];
Goto(bb6)
}
bb19 = {
_13 = _31;
_15 = !_9;
_32 = [80917461837708399604299090565727042238_i128,(-160260349626868610370559898500535548360_i128),83332560886939177744464769738030001110_i128,37898448658094876388548576831008452702_i128,159326728993860781846830292639446116069_i128,(-138595199812492801615672725030550729090_i128),(-122801614377394477093650975542115853559_i128)];
_34 = Field::<char>(Variant(_14, 0), 1);
_12 = [7713119606156823565_i64,3715664201016899942_i64,(-302786131845085080_i64),5570378889677979153_i64,(-8102207099594920738_i64),(-3782993782556377691_i64),(-1606528487495118840_i64),4359867501233246404_i64];
_4 = _8.3 << _1;
_30.3 = _8.0 as i16;
_19 = [64827901734363499964880072163535668683_i128,146298149807062827688820185330596421412_i128];
_24.0 = Field::<char>(Variant(_14, 0), 1);
place!(Field::<char>(Variant(_14, 0), 1)) = _24.0;
_16 = [(-7959006727584439602_i64),4055767001370596151_i64,6982046754591611733_i64,6709110383536846087_i64,(-1471573530010631468_i64),65958223381266359_i64,(-3311460138701685429_i64),5021278463642895274_i64];
_28 = (_25,);
place!(Field::<i16>(Variant(_14, 0), 4)) = _10 as i16;
_33 = 3912265705_u32 as i16;
_10 = 1085755734_u32 as f64;
_20 = _25 as f32;
_8.3 = _4;
_36 = _24;
_35 = [_8.1,_5,_8.1,_1,_1,_1];
_30.2 = 1527136772_i32 as usize;
_30.0 = 27354_u16 as u64;
_39 = 3593924061861854559_i64 as f32;
_23 = _27;
RET = [_30.0,_30.0,_30.0,_30.0,_30.0,_30.0,_30.0,_30.0];
place!(Field::<isize>(Variant(_14, 0), 2)) = _1;
Goto(bb20)
}
bb20 = {
Call(_43 = dump_var(6_usize, 4_usize, Move(_4), 11_usize, Move(_11), 25_usize, Move(_25), 16_usize, Move(_16)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_43 = dump_var(6_usize, 33_usize, Move(_33), 15_usize, Move(_15), 9_usize, Move(_9), 21_usize, Move(_21)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_43 = dump_var(6_usize, 2_usize, Move(_2), 24_usize, Move(_24), 34_usize, Move(_34), 28_usize, Move(_28)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: (u8, isize, i8, i16, f32),mut _2: [u64; 8],mut _3: isize,mut _4: isize,mut _5: i16,mut _6: isize) -> [usize; 4] {
mir! {
type RET = [usize; 4];
let _7: f64;
let _8: [i128; 2];
let _9: (char,);
let _10: [i128; 2];
let _11: (char,);
let _12: [i8; 5];
let _13: f64;
let _14: Adt59;
let _15: Adt55;
let _16: u128;
let _17: isize;
let _18: char;
let _19: [u32; 3];
let _20: ((&'static char, char, *const usize), isize);
let _21: [u64; 8];
let _22: (u64, [i8; 5], usize, i16, *const i32);
let _23: Adt49;
let _24: [i8; 5];
let _25: bool;
let _26: Adt51;
let _27: i32;
let _28: f64;
let _29: Adt50;
let _30: bool;
let _31: bool;
let _32: [i16; 2];
let _33: isize;
let _34: isize;
let _35: ();
let _36: ();
{
RET = [5929964928393521792_usize,2_usize,4_usize,3_usize];
_1.0 = true as u8;
_3 = _1.1 - _6;
_1.2 = -(-30_i8);
RET = [2_usize,0_usize,9308849032923087422_usize,0_usize];
_3 = 2374220314919257297_i64 as isize;
RET = [1_usize,4_usize,616634932995945968_usize,5_usize];
_5 = _1.2 as i16;
_1.0 = !104_u8;
_2 = [16530723776049224425_u64,4498576541851203976_u64,5507591061136487430_u64,15343185252821248965_u64,13575843549173725620_u64,17974768360398856129_u64,8217291685840734050_u64,9560574714279305286_u64];
_5 = _1.3;
_2 = [8672544026070940022_u64,13315024406284556163_u64,14769933071754341079_u64,7678205222126952521_u64,1293928608561205183_u64,9991474129199708640_u64,12670228117988374246_u64,18444055229447225005_u64];
_5 = _1.3;
_6 = _1.1;
_1.1 = _6;
_3 = _1.3 as isize;
_5 = -_1.3;
_1.3 = _5 - _5;
_5 = 279093106589814486213083104645447746845_u128 as i16;
RET = [3_usize,2_usize,3_usize,4_usize];
RET = [9625981600799446517_usize,16302157298932032338_usize,3_usize,4_usize];
RET = [9867329550294708929_usize,1_usize,6_usize,6_usize];
Call(_1.0 = core::intrinsics::transmute(_1.2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1.2 = (-128_i8) ^ (-92_i8);
_1.3 = (-116075361557463417397933821172377557239_i128) as i16;
_2 = [16623036629458654502_u64,9076814082824314261_u64,13908696539526635350_u64,15904877082252665763_u64,10280136034115564699_u64,16956868548743395943_u64,12018676090218697607_u64,15249457317707419290_u64];
_1.4 = _4 as f32;
RET = [2_usize,6_usize,5_usize,9132354641007925111_usize];
Goto(bb2)
}
bb2 = {
_5 = _1.3 * _1.3;
_2 = [5475206070925377530_u64,700853663818632750_u64,18390648256752572610_u64,4066583297326061596_u64,5735838689491612777_u64,18019127258374218229_u64,1475179020172969874_u64,17218438609420036247_u64];
_5 = _1.3 >> _1.1;
_5 = _1.4 as i16;
_1.0 = 252_u8 - 22_u8;
RET = [12367751984269086625_usize,3_usize,1715400818680106864_usize,4_usize];
_1.1 = _1.2 as isize;
_9.0 = '\u{87999}';
_9 = ('\u{1b433}',);
_8 = [141169761064899972497863017884304170634_i128,(-124936720971614527870446549724018417545_i128)];
_1.2 = 13_i8;
_6 = _4;
_7 = (-87611889562885262794437478794462009801_i128) as f64;
_1.4 = 2820479744780919905_usize as f32;
_6 = 182129367471290818094932359337609003740_u128 as isize;
_2 = [13069456812189324464_u64,3982080571354913928_u64,17118732891933195975_u64,10011652832258317025_u64,1172744791769512045_u64,14238285967204134300_u64,4413179485019806934_u64,10896998338913878942_u64];
RET = [10654331875829555037_usize,2_usize,7_usize,5819059147442484749_usize];
_6 = _4;
RET = [677168570814465479_usize,13505804803813853197_usize,3_usize,3_usize];
_10 = [(-164474133474179047010142539896562215014_i128),(-67420007819024826334257858981943517625_i128)];
_3 = _4;
RET = [3_usize,4425718990255731234_usize,4495242609181149831_usize,5_usize];
match _1.2 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
13 => bb10,
_ => bb9
}
}
bb3 = {
_1.2 = (-128_i8) ^ (-92_i8);
_1.3 = (-116075361557463417397933821172377557239_i128) as i16;
_2 = [16623036629458654502_u64,9076814082824314261_u64,13908696539526635350_u64,15904877082252665763_u64,10280136034115564699_u64,16956868548743395943_u64,12018676090218697607_u64,15249457317707419290_u64];
_1.4 = _4 as f32;
RET = [2_usize,6_usize,5_usize,9132354641007925111_usize];
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
_1.3 = _7 as i16;
RET = [5_usize,6269307088309817422_usize,0_usize,11541442101053914658_usize];
_11 = _9;
_10 = _8;
_1.3 = _5 | _5;
_11.0 = _9.0;
_1.2 = !81_i8;
_11 = _9;
_6 = _4;
_9 = (_11.0,);
_6 = _4;
_1.1 = _6 | _3;
_7 = 9113551241607374008_i64 as f64;
_1.3 = _5 * _5;
_4 = true as isize;
_10 = [94649047959654955220077899264253654558_i128,(-4807046821014181421341167688821356434_i128)];
_1.0 = 126_u8 * 229_u8;
_10 = [30265573755986887694337526062300882129_i128,(-66250153165502288232519452923193163644_i128)];
_10 = [104211772001455051397147711835222088755_i128,(-119472666169588018896818877321517289304_i128)];
_1.4 = 4672541887012627177_usize as f32;
_2 = [16941610738200398620_u64,12165689003708336550_u64,8075797356109398569_u64,13598416405146882847_u64,11208999899826040514_u64,9759147851339186386_u64,7935316689829883533_u64,17702851523809257337_u64];
Goto(bb11)
}
bb11 = {
_1.4 = (-82227364396881043302953895512970276444_i128) as f32;
_10 = [(-151901751726673964951532384103884815806_i128),(-97816585695617080045493807279764267673_i128)];
_18 = _11.0;
_16 = _7 as u128;
_7 = _1.4 as f64;
_10 = _8;
_20.0.0 = &_11.0;
_17 = _1.1 | _3;
_11 = _9;
_3 = _1.1 ^ _17;
_20.0.0 = &_11.0;
_20.1 = _17 * _3;
_3 = _20.1 & _20.1;
RET = [3614869270159235879_usize,13361681585991294109_usize,8213821298910430921_usize,3_usize];
_1.0 = 191_u8;
_20.0.1 = _9.0;
_13 = _7 + _7;
RET = [6_usize,15076144441364265243_usize,12031667677817132285_usize,3_usize];
_17 = !_6;
_1.4 = _1.3 as f32;
_20.0.1 = _18;
_19 = [2448157239_u32,2851320349_u32,554912256_u32];
_1.1 = _3 & _20.1;
match _1.0 {
0 => bb6,
1 => bb7,
191 => bb13,
_ => bb12
}
}
bb12 = {
_1.2 = (-128_i8) ^ (-92_i8);
_1.3 = (-116075361557463417397933821172377557239_i128) as i16;
_2 = [16623036629458654502_u64,9076814082824314261_u64,13908696539526635350_u64,15904877082252665763_u64,10280136034115564699_u64,16956868548743395943_u64,12018676090218697607_u64,15249457317707419290_u64];
_1.4 = _4 as f32;
RET = [2_usize,6_usize,5_usize,9132354641007925111_usize];
Goto(bb2)
}
bb13 = {
_1.2 = (-101581144563680647792204353994398001822_i128) as i8;
_20.0.1 = _18;
_7 = _16 as f64;
RET = [5_usize,7_usize,0_usize,0_usize];
_19 = [361909349_u32,2261422331_u32,3745254560_u32];
_10 = _8;
_13 = -_7;
_1.4 = 1555490841_i32 as f32;
_7 = _13 - _13;
RET = [6951587197820050389_usize,12887004854497804257_usize,3_usize,553498113859908332_usize];
_8 = [(-132267124226529677367439062969848503583_i128),(-121545416853739002961671283395688540487_i128)];
_16 = !144654236410005486660585553296481117921_u128;
_4 = !_20.1;
_1.0 = _7 as u8;
_22.1 = [_1.2,_1.2,_1.2,_1.2,_1.2];
_21 = _2;
_10 = [110455123472084077415402229760450452575_i128,52146638235729739947092808963918827083_i128];
_22.0 = 9234698991588645646_u64;
_1.2 = _1.0 as i8;
_23.fld0 = _8;
_2 = [_22.0,_22.0,_22.0,_22.0,_22.0,_22.0,_22.0,_22.0];
_22.3 = _5 * _1.3;
_11 = _9;
Goto(bb14)
}
bb14 = {
_23.fld1 = _18;
_20.0.2 = core::ptr::addr_of!(_22.2);
RET = [6989704640170305523_usize,5_usize,3_usize,4_usize];
_12 = [_1.2,_1.2,_1.2,_1.2,_1.2];
_16 = 234732872995241449025462287305828013021_u128;
_6 = _3 << _17;
_1.2 = 58_i8 * (-59_i8);
_30 = false;
_20.0.1 = _11.0;
_22.3 = (-247273416_i32) as i16;
_8 = [(-636646904135175663958292142711555540_i128),41737544069991135634828120145520073552_i128];
_22.2 = !5_usize;
RET = [_22.2,_22.2,_22.2,_22.2];
_23.fld0 = [116533492842888651099256926048706773680_i128,57241472203954783834048729039012205329_i128];
_5 = _1.3;
_25 = _20.1 <= _1.1;
_17 = -_3;
_6 = _20.1 >> _4;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(7_usize, 19_usize, Move(_19), 17_usize, Move(_17), 30_usize, Move(_30), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(7_usize, 3_usize, Move(_3), 4_usize, Move(_4), 9_usize, Move(_9), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: [u64; 8],mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize,mut _15: [u64; 8],mut _16: isize) -> ([u64; 8], i128, i128, i8, (*const usize, i16)) {
mir! {
type RET = ([u64; 8], i128, i128, i8, (*const usize, i16));
let _17: f64;
let _18: [usize; 4];
let _19: Adt52;
let _20: isize;
let _21: u32;
let _22: *const usize;
let _23: isize;
let _24: (char,);
let _25: char;
let _26: Adt50;
let _27: Adt44;
let _28: f64;
let _29: *const isize;
let _30: f64;
let _31: [i8; 5];
let _32: [i64; 8];
let _33: char;
let _34: i32;
let _35: f64;
let _36: u32;
let _37: (u128,);
let _38: Adt44;
let _39: char;
let _40: [i64; 8];
let _41: f32;
let _42: char;
let _43: [usize; 4];
let _44: u16;
let _45: bool;
let _46: isize;
let _47: ([u64; 8], i128, i128, i8, (*const usize, i16));
let _48: Adt46;
let _49: (u128,);
let _50: [i16; 2];
let _51: i16;
let _52: isize;
let _53: bool;
let _54: bool;
let _55: char;
let _56: isize;
let _57: [i16; 2];
let _58: [usize; 4];
let _59: u64;
let _60: (char,);
let _61: [i16; 2];
let _62: f32;
let _63: [i128; 7];
let _64: isize;
let _65: f64;
let _66: usize;
let _67: [i128; 2];
let _68: *const *const bool;
let _69: ();
let _70: ();
{
_14 = 13056812154749207186_u64 as isize;
RET.1 = 205_u8 as i128;
_5 = [4537590085743661219_u64,9196112459260866033_u64,17108216339295914774_u64,17833402761086794614_u64,10394295374859508504_u64,8686599016775474519_u64,2898228820543486285_u64,9755444535485086929_u64];
RET.1 = -(-115403434033206248586605108562340020474_i128);
_15 = [5417539682220794358_u64,13893355389756850653_u64,6717691397151894582_u64,10636920403299512403_u64,12521817837734248577_u64,8117221778855532649_u64,7611900182339880974_u64,6098617418914693791_u64];
RET.3 = 119_i8;
_16 = _8 & _6;
RET.0 = _5;
RET.3 = (-82_i8) | 109_i8;
RET.2 = !RET.1;
_8 = !_12;
_9 = _6;
_1 = _4 | _3;
RET.3 = !104_i8;
_11 = 14249155300427780932_usize as isize;
RET.4.1 = -2231_i16;
_17 = (-4213122834132562698_i64) as f64;
RET.0 = _15;
_10 = !_7;
_18 = [0_usize,5_usize,15378332114556610827_usize,13436530115991482377_usize];
_16 = _2 ^ _12;
_15 = RET.0;
RET.3 = !46_i8;
Goto(bb1)
}
bb1 = {
_16 = !_4;
_20 = false as isize;
_21 = !3633608864_u32;
_2 = 158833333067659484695872754098620432807_u128 as isize;
_11 = _1;
RET.0 = _5;
RET.2 = RET.1;
_24 = ('\u{4f1ec}',);
Goto(bb2)
}
bb2 = {
_3 = 9351334411567107991_usize as isize;
_8 = 248643093561733927797610603210088798766_u128 as isize;
RET.3 = 95_i8 * (-123_i8);
_5 = [13850475715475481632_u64,9144806469105224940_u64,10672398459235225023_u64,13160254050639456182_u64,10364766044106260144_u64,12919823343148417166_u64,5727271761198450532_u64,14708472950531562652_u64];
_20 = _11;
_10 = _1 | _1;
_11 = !_12;
_10 = -_20;
_12 = !_20;
_25 = _24.0;
_16 = _12 & _4;
_14 = _20;
_12 = 991292725_i32 as isize;
_9 = _7 - _20;
RET.4.1 = 4943615379049670441_u64 as i16;
_15 = RET.0;
_1 = _20 - _14;
_1 = _20;
RET.0 = _5;
_10 = _9 >> _1;
RET.0 = _15;
_13 = !_9;
_11 = _20 | _14;
_21 = 1362230786_u32;
RET.2 = _21 as i128;
RET.0 = _15;
_20 = _1 | _10;
Goto(bb3)
}
bb3 = {
_16 = _10 * _11;
Goto(bb4)
}
bb4 = {
RET.0 = [11973941242717457271_u64,17162406492248670712_u64,17420295211951644492_u64,10882900965677411993_u64,15849881985272914128_u64,8708053215531645664_u64,11693880440862086702_u64,2679891548918921285_u64];
Call(_13 = core::intrinsics::bswap(_20), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_23 = _13;
RET.4.1 = (-22757_i16);
_1 = _17 as isize;
_4 = _20;
_24.0 = _25;
_18 = [6_usize,7266221880551499913_usize,8956170609308000287_usize,4739397488689601301_usize];
_2 = _13 * _10;
_5 = [14376742388916470055_u64,16583676418093852142_u64,8867408204878901061_u64,115302231143530140_u64,2105745514483278460_u64,15535233358850645600_u64,15743203610734200104_u64,14017542345454977760_u64];
_16 = false as isize;
Call(_3 = fn9(_11, _20, _2, _2, _2, _5, _2, _20, _20), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_20 = _3 ^ _13;
_14 = _20;
_17 = 2148387568920739933_i64 as f64;
_9 = 2085885992_i32 as isize;
RET.2 = RET.1 >> _3;
_1 = -_3;
_16 = _20;
_4 = RET.3 as isize;
_2 = _23 - _16;
RET.2 = RET.1 + RET.1;
_24 = (_25,);
RET.4.1 = 24134760103866497922607989431312556307_u128 as i16;
RET.4.1 = 42070_u16 as i16;
RET.2 = 236_u8 as i128;
_17 = 1206175725_i32 as f64;
RET.4.1 = (-29547_i16);
Goto(bb7)
}
bb7 = {
RET.4.1 = (-19236_i16);
RET.4.1 = 26811_i16 * 8878_i16;
_5 = [14108239010377696646_u64,2396962077889315434_u64,3290262754919792871_u64,6157450694499533073_u64,7421663119232683813_u64,15905338917243564936_u64,9884382771166477281_u64,17194803725703245874_u64];
_23 = _16;
RET.1 = !RET.2;
RET.4.1 = -(-7413_i16);
Call(RET.1 = fn10(_23, _16, _23, _16, _13), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
RET.4.1 = (-8366_i16);
_8 = _10 - _23;
RET.1 = RET.3 as i128;
RET.0 = [13145754508406786427_u64,17999564482942585865_u64,8325818981924036767_u64,12392900420992930207_u64,15993359656859903702_u64,16274187981326151665_u64,15468202850206626475_u64,16015825923163106648_u64];
_24 = (_25,);
match RET.4.1 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
340282366920938463463374607431768203090 => bb16,
_ => bb15
}
}
bb9 = {
RET.4.1 = (-19236_i16);
RET.4.1 = 26811_i16 * 8878_i16;
_5 = [14108239010377696646_u64,2396962077889315434_u64,3290262754919792871_u64,6157450694499533073_u64,7421663119232683813_u64,15905338917243564936_u64,9884382771166477281_u64,17194803725703245874_u64];
_23 = _16;
RET.1 = !RET.2;
RET.4.1 = -(-7413_i16);
Call(RET.1 = fn10(_23, _16, _23, _16, _13), ReturnTo(bb8), UnwindUnreachable())
}
bb10 = {
_20 = _3 ^ _13;
_14 = _20;
_17 = 2148387568920739933_i64 as f64;
_9 = 2085885992_i32 as isize;
RET.2 = RET.1 >> _3;
_1 = -_3;
_16 = _20;
_4 = RET.3 as isize;
_2 = _23 - _16;
RET.2 = RET.1 + RET.1;
_24 = (_25,);
RET.4.1 = 24134760103866497922607989431312556307_u128 as i16;
RET.4.1 = 42070_u16 as i16;
RET.2 = 236_u8 as i128;
_17 = 1206175725_i32 as f64;
RET.4.1 = (-29547_i16);
Goto(bb7)
}
bb11 = {
_23 = _13;
RET.4.1 = (-22757_i16);
_1 = _17 as isize;
_4 = _20;
_24.0 = _25;
_18 = [6_usize,7266221880551499913_usize,8956170609308000287_usize,4739397488689601301_usize];
_2 = _13 * _10;
_5 = [14376742388916470055_u64,16583676418093852142_u64,8867408204878901061_u64,115302231143530140_u64,2105745514483278460_u64,15535233358850645600_u64,15743203610734200104_u64,14017542345454977760_u64];
_16 = false as isize;
Call(_3 = fn9(_11, _20, _2, _2, _2, _5, _2, _20, _20), ReturnTo(bb6), UnwindUnreachable())
}
bb12 = {
RET.0 = [11973941242717457271_u64,17162406492248670712_u64,17420295211951644492_u64,10882900965677411993_u64,15849881985272914128_u64,8708053215531645664_u64,11693880440862086702_u64,2679891548918921285_u64];
Call(_13 = core::intrinsics::bswap(_20), ReturnTo(bb5), UnwindUnreachable())
}
bb13 = {
_16 = _10 * _11;
Goto(bb4)
}
bb14 = {
_3 = 9351334411567107991_usize as isize;
_8 = 248643093561733927797610603210088798766_u128 as isize;
RET.3 = 95_i8 * (-123_i8);
_5 = [13850475715475481632_u64,9144806469105224940_u64,10672398459235225023_u64,13160254050639456182_u64,10364766044106260144_u64,12919823343148417166_u64,5727271761198450532_u64,14708472950531562652_u64];
_20 = _11;
_10 = _1 | _1;
_11 = !_12;
_10 = -_20;
_12 = !_20;
_25 = _24.0;
_16 = _12 & _4;
_14 = _20;
_12 = 991292725_i32 as isize;
_9 = _7 - _20;
RET.4.1 = 4943615379049670441_u64 as i16;
_15 = RET.0;
_1 = _20 - _14;
_1 = _20;
RET.0 = _5;
_10 = _9 >> _1;
RET.0 = _15;
_13 = !_9;
_11 = _20 | _14;
_21 = 1362230786_u32;
RET.2 = _21 as i128;
RET.0 = _15;
_20 = _1 | _10;
Goto(bb3)
}
bb15 = {
_16 = !_4;
_20 = false as isize;
_21 = !3633608864_u32;
_2 = 158833333067659484695872754098620432807_u128 as isize;
_11 = _1;
RET.0 = _5;
RET.2 = RET.1;
_24 = ('\u{4f1ec}',);
Goto(bb2)
}
bb16 = {
RET.4.1 = 22902_i16 & (-26198_i16);
_24.0 = _25;
RET.2 = -RET.1;
_9 = _2 >> _2;
_33 = _24.0;
_30 = _21 as f64;
RET.1 = _21 as i128;
_24 = (_33,);
_8 = 59571_u16 as isize;
_6 = -_20;
_34 = false as i32;
RET.1 = RET.2;
_7 = _14 | _23;
_23 = _9 ^ _9;
_15 = [1529033294155920824_u64,8612858047581928388_u64,17604754486800421219_u64,9629763827078201809_u64,1843798403946424275_u64,5017477370731669111_u64,13795516686505674156_u64,7133158172954725092_u64];
_1 = -_2;
_33 = _24.0;
RET.4.1 = (-2696_i16) << _10;
_35 = 3189537409527310002_u64 as f64;
RET.4.1 = (-25744_i16);
_29 = core::ptr::addr_of!(_4);
_17 = 3_usize as f64;
_21 = 8100693324567593827_i64 as u32;
Goto(bb17)
}
bb17 = {
_7 = _3;
_9 = _23;
_31 = [RET.3,RET.3,RET.3,RET.3,RET.3];
_7 = !_23;
_11 = _7 & _10;
_20 = -_3;
_13 = _9 & _14;
_7 = _23 - _13;
_30 = _17;
_8 = 9659630254703716412_usize as isize;
_28 = _35;
Goto(bb18)
}
bb18 = {
_33 = _24.0;
_21 = 239467340_u32 | 1480921394_u32;
_36 = !_21;
_37 = (79357200816305249315669735882442130984_u128,);
_14 = _25 as isize;
RET.4.1 = -14823_i16;
_20 = !_9;
_16 = true as isize;
_20 = 9848275420773799819_u64 as isize;
_6 = _17 as isize;
_23 = _10 - _3;
_8 = !_23;
RET.4.1 = _8 as i16;
_6 = _9 ^ _9;
(*_29) = _8 | _6;
_24 = (_33,);
match _37.0 {
0 => bb19,
79357200816305249315669735882442130984 => bb21,
_ => bb20
}
}
bb19 = {
_20 = _3 ^ _13;
_14 = _20;
_17 = 2148387568920739933_i64 as f64;
_9 = 2085885992_i32 as isize;
RET.2 = RET.1 >> _3;
_1 = -_3;
_16 = _20;
_4 = RET.3 as isize;
_2 = _23 - _16;
RET.2 = RET.1 + RET.1;
_24 = (_25,);
RET.4.1 = 24134760103866497922607989431312556307_u128 as i16;
RET.4.1 = 42070_u16 as i16;
RET.2 = 236_u8 as i128;
_17 = 1206175725_i32 as f64;
RET.4.1 = (-29547_i16);
Goto(bb7)
}
bb20 = {
RET.4.1 = 22902_i16 & (-26198_i16);
_24.0 = _25;
RET.2 = -RET.1;
_9 = _2 >> _2;
_33 = _24.0;
_30 = _21 as f64;
RET.1 = _21 as i128;
_24 = (_33,);
_8 = 59571_u16 as isize;
_6 = -_20;
_34 = false as i32;
RET.1 = RET.2;
_7 = _14 | _23;
_23 = _9 ^ _9;
_15 = [1529033294155920824_u64,8612858047581928388_u64,17604754486800421219_u64,9629763827078201809_u64,1843798403946424275_u64,5017477370731669111_u64,13795516686505674156_u64,7133158172954725092_u64];
_1 = -_2;
_33 = _24.0;
RET.4.1 = (-2696_i16) << _10;
_35 = 3189537409527310002_u64 as f64;
RET.4.1 = (-25744_i16);
_29 = core::ptr::addr_of!(_4);
_17 = 3_usize as f64;
_21 = 8100693324567593827_i64 as u32;
Goto(bb17)
}
bb21 = {
_6 = 50434_u16 as isize;
_34 = !(-928446247_i32);
RET.3 = (-123_i8) + 14_i8;
Goto(bb22)
}
bb22 = {
_12 = _37.0 as isize;
RET.3 = true as i8;
_8 = _1 << _4;
_7 = _9;
_30 = _35 * _17;
RET.1 = RET.2;
_39 = _25;
_35 = -_30;
RET.0 = _5;
_39 = _33;
RET.4.1 = RET.2 as i16;
_3 = _4 ^ _11;
_21 = !_36;
_14 = _34 as isize;
_35 = _28;
_31 = [RET.3,RET.3,RET.3,RET.3,RET.3];
_26 = Adt50::Variant2 { fld0: 47968_u16,fld1: _31 };
_37.0 = !169044040837451980809745553350532786964_u128;
_12 = _34 as isize;
_13 = 139_u8 as isize;
(*_29) = RET.3 as isize;
_6 = _11 + _7;
Goto(bb23)
}
bb23 = {
RET.3 = _33 as i8;
_43 = [2_usize,3_usize,0_usize,2721810272508754977_usize];
(*_29) = !_7;
_33 = _25;
_42 = _39;
_43 = _18;
RET.3 = RET.4.1 as i8;
_36 = _21 - _21;
Goto(bb24)
}
bb24 = {
RET.3 = (-36_i8);
place!(Field::<u16>(Variant(_26, 2), 0)) = 42661_u16 ^ 37369_u16;
_35 = 249_u8 as f64;
_46 = 2113767943615989726_u64 as isize;
RET.4.1 = -6022_i16;
_44 = _36 as u16;
(*_29) = -_11;
RET.3 = !31_i8;
_32 = [(-8691323569162714385_i64),(-4190262205272398736_i64),7828465010661076513_i64,(-2694258293202715752_i64),(-8296292163271235781_i64),(-8764027782301902764_i64),7661642636287363859_i64,896261630701369414_i64];
_47.3 = RET.3 << (*_29);
_46 = !_9;
_24.0 = _39;
_29 = core::ptr::addr_of!((*_29));
_24.0 = _42;
_42 = _24.0;
RET.4.1 = 16767_i16;
SetDiscriminant(_26, 2);
_18 = [15860590054907307242_usize,1750134673951790128_usize,3089123704735475345_usize,575105035632958125_usize];
_37 = (243738377049147174251681279695767620911_u128,);
Goto(bb25)
}
bb25 = {
(*_29) = _8 | _11;
_46 = (*_29);
_6 = (*_29) << _47.3;
_5 = [18371989928834934099_u64,3068604658384903445_u64,11031792020261746344_u64,6959971096069097096_u64,4826332793761745598_u64,12845618419576711749_u64,7133072389844822815_u64,6051610662420759583_u64];
_1 = _8 ^ _2;
_47.0 = [13102820015574068411_u64,3581522459939623614_u64,17099537477973362956_u64,2787863600781708610_u64,5271297184538318489_u64,5102093322181963089_u64,15166672703698327667_u64,11284375311890788434_u64];
_15 = [10212772146715207545_u64,3532245228669142924_u64,1671769872295727222_u64,10392878227802299159_u64,1991847626609473410_u64,12883316846349368154_u64,12019084989350229677_u64,7411701925351265200_u64];
_1 = (*_29);
place!(Field::<[i8; 5]>(Variant(_26, 2), 1)) = [_47.3,_47.3,_47.3,_47.3,_47.3];
_50 = [RET.4.1,RET.4.1];
_47.1 = _47.3 as i128;
RET.2 = _47.1 & _47.1;
_48 = Adt46::Variant1 { fld0: _28 };
_40 = [(-522592200713712598_i64),521969765487806719_i64,(-2380016235675022601_i64),3568977233532566605_i64,6528700897266083197_i64,(-4442502457040005535_i64),7146027480622178231_i64,(-8257348112415542646_i64)];
RET.3 = _6 as i8;
_30 = -Field::<f64>(Variant(_48, 1), 0);
Call(_10 = core::intrinsics::transmute(_9), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
_34 = (-1964293855_i32);
_47.0 = _15;
_53 = !false;
_41 = _44 as f32;
_31 = [_47.3,RET.3,RET.3,RET.3,_47.3];
_2 = !_23;
_50 = [RET.4.1,RET.4.1];
RET.1 = _24.0 as i128;
RET.3 = _47.3 >> _7;
SetDiscriminant(_48, 0);
RET.1 = _47.1;
_48 = Adt46::Variant1 { fld0: _17 };
_11 = _46 << _46;
_26 = Adt50::Variant2 { fld0: _44,fld1: _31 };
_17 = _41 as f64;
RET.4.1 = _34 as i16;
_24 = (_42,);
SetDiscriminant(_48, 0);
_31 = Field::<[i8; 5]>(Variant(_26, 2), 1);
_52 = _47.3 as isize;
_35 = _17;
match _37.0 {
0 => bb11,
1 => bb25,
2 => bb14,
243738377049147174251681279695767620911 => bb28,
_ => bb27
}
}
bb27 = {
_6 = 50434_u16 as isize;
_34 = !(-928446247_i32);
RET.3 = (-123_i8) + 14_i8;
Goto(bb22)
}
bb28 = {
_3 = !(*_29);
_20 = 163_u8 as isize;
_56 = _8;
_49 = (_37.0,);
_25 = _39;
_39 = _42;
Goto(bb29)
}
bb29 = {
place!(Field::<(char,)>(Variant(_48, 0), 3)) = (_25,);
_9 = (*_29);
(*_29) = _7;
_45 = _53;
_32 = [(-5724600920033541084_i64),7017589307142377755_i64,(-1735978263232781463_i64),506694512673079254_i64,(-8419624683902089627_i64),8999329323584170269_i64,(-6063183643180298546_i64),7575206694350658860_i64];
_20 = (*_29);
_42 = _25;
_29 = core::ptr::addr_of!(_52);
_11 = _7 | _52;
_37 = (_49.0,);
_12 = _52 ^ _8;
(*_29) = _10;
_26 = Adt50::Variant2 { fld0: _44,fld1: _31 };
RET.4.1 = 27486_i16 >> _20;
_20 = _4;
match _49.0 {
0 => bb23,
1 => bb30,
243738377049147174251681279695767620911 => bb32,
_ => bb31
}
}
bb30 = {
_7 = _3;
_9 = _23;
_31 = [RET.3,RET.3,RET.3,RET.3,RET.3];
_7 = !_23;
_11 = _7 & _10;
_20 = -_3;
_13 = _9 & _14;
_7 = _23 - _13;
_30 = _17;
_8 = 9659630254703716412_usize as isize;
_28 = _35;
Goto(bb18)
}
bb31 = {
RET.3 = (-36_i8);
place!(Field::<u16>(Variant(_26, 2), 0)) = 42661_u16 ^ 37369_u16;
_35 = 249_u8 as f64;
_46 = 2113767943615989726_u64 as isize;
RET.4.1 = -6022_i16;
_44 = _36 as u16;
(*_29) = -_11;
RET.3 = !31_i8;
_32 = [(-8691323569162714385_i64),(-4190262205272398736_i64),7828465010661076513_i64,(-2694258293202715752_i64),(-8296292163271235781_i64),(-8764027782301902764_i64),7661642636287363859_i64,896261630701369414_i64];
_47.3 = RET.3 << (*_29);
_46 = !_9;
_24.0 = _39;
_29 = core::ptr::addr_of!((*_29));
_24.0 = _42;
_42 = _24.0;
RET.4.1 = 16767_i16;
SetDiscriminant(_26, 2);
_18 = [15860590054907307242_usize,1750134673951790128_usize,3089123704735475345_usize,575105035632958125_usize];
_37 = (243738377049147174251681279695767620911_u128,);
Goto(bb25)
}
bb32 = {
_47.3 = _36 as i8;
_7 = !_46;
_51 = RET.4.1;
_41 = 8683212032456866666_usize as f32;
_29 = core::ptr::addr_of!((*_29));
_47.0 = RET.0;
_24.0 = _39;
place!(Field::<u16>(Variant(_26, 2), 0)) = !_44;
SetDiscriminant(_26, 0);
place!(Field::<*const i32>(Variant(_26, 0), 4)) = core::ptr::addr_of!(_34);
place!(Field::<(*const usize, i16)>(Variant(_26, 0), 6)).1 = 149_u8 as i16;
_60.0 = Field::<(char,)>(Variant(_48, 0), 3).0;
_34 = (-1826527267_i32) + (-1700737202_i32);
_24.0 = _25;
_37 = _49;
_35 = _30 * _17;
place!(Field::<[u64; 8]>(Variant(_26, 0), 2)) = _15;
_31 = [RET.3,RET.3,RET.3,RET.3,RET.3];
_2 = _1 << _3;
_58 = [7_usize,2_usize,9586801221108711841_usize,7843716660402176162_usize];
_5 = [8478117832294591107_u64,5058665240341847167_u64,8470432622574968920_u64,13040663520008474918_u64,11540306892326229098_u64,3546062705756424673_u64,11532083207620560144_u64,7870282420017721166_u64];
Goto(bb33)
}
bb33 = {
place!(Field::<[i16; 2]>(Variant(_48, 0), 4)) = [_51,_51];
_34 = -850554354_i32;
_60 = _24;
RET.4.1 = !_51;
_15 = [1492588618421249557_u64,9557538668247866381_u64,12389020434189101529_u64,7712029998967058886_u64,3571020417686413783_u64,4138675590473051003_u64,10469590732755807085_u64,10215264371604723738_u64];
_36 = _21;
place!(Field::<*mut i32>(Variant(_26, 0), 5)) = core::ptr::addr_of_mut!(_34);
_12 = -_2;
_52 = _11;
_53 = _45;
_52 = _12 & _4;
_47.4.1 = RET.4.1;
_35 = 124_u8 as f64;
Call(_41 = core::intrinsics::transmute(Field::<[i16; 2]>(Variant(_48, 0), 4)), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
_29 = core::ptr::addr_of!(_10);
place!(Field::<(*const usize, i16)>(Variant(_26, 0), 6)).0 = core::ptr::addr_of!(_66);
_31 = [RET.3,RET.3,_47.3,RET.3,RET.3];
_58 = [12225116367532645209_usize,0_usize,12851435355118796364_usize,3_usize];
_59 = _34 as u64;
_47.0 = [_59,_59,_59,_59,_59,_59,_59,_59];
_46 = _8;
_23 = _36 as isize;
_54 = _45;
RET = (Field::<[u64; 8]>(Variant(_26, 0), 2), _47.1, _47.1, _47.3, Field::<(*const usize, i16)>(Variant(_26, 0), 6));
RET.2 = -RET.1;
_52 = (*_29) ^ _56;
_65 = -_17;
_43 = _18;
_21 = _36 << _2;
_11 = _6 << _56;
place!(Field::<[i16; 2]>(Variant(_26, 0), 3)) = [_51,_51];
_23 = _1;
_67 = [RET.1,RET.1];
Goto(bb35)
}
bb35 = {
Call(_69 = dump_var(8_usize, 54_usize, Move(_54), 50_usize, Move(_50), 14_usize, Move(_14), 59_usize, Move(_59)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Call(_69 = dump_var(8_usize, 25_usize, Move(_25), 53_usize, Move(_53), 60_usize, Move(_60), 21_usize, Move(_21)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Call(_69 = dump_var(8_usize, 23_usize, Move(_23), 16_usize, Move(_16), 1_usize, Move(_1), 9_usize, Move(_9)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Call(_69 = dump_var(8_usize, 2_usize, Move(_2), 36_usize, Move(_36), 44_usize, Move(_44), 49_usize, Move(_49)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_69 = dump_var(8_usize, 3_usize, Move(_3), 31_usize, Move(_31), 20_usize, Move(_20), 8_usize, Move(_8)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Call(_69 = dump_var(8_usize, 11_usize, Move(_11), 42_usize, Move(_42), 45_usize, Move(_45), 70_usize, _70), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: [u64; 8],mut _7: isize,mut _8: isize,mut _9: isize) -> isize {
mir! {
type RET = isize;
let _10: i64;
let _11: usize;
let _12: char;
let _13: (u8, isize, i8, i16, f32);
let _14: ();
let _15: ();
{
_6 = [17157703256894146289_u64,16746647860458088763_u64,17101568344019733166_u64,8452623439181142479_u64,9033287875813718017_u64,18267955612120784588_u64,3912859779143386885_u64,15770754276524465902_u64];
_7 = _2;
_10 = 8442098966983810409_i64;
_11 = 3745734013662847472_usize * 0_usize;
_10 = 7483383545134687012_i64;
_9 = _8 ^ _7;
RET = _3 - _3;
_4 = RET - _1;
_6 = [17186819374916858816_u64,2345082602237433914_u64,11407113318686927786_u64,2437980795849628166_u64,12340405127104354103_u64,4188666317481846153_u64,1653704182093991432_u64,2035317292544150835_u64];
_7 = _1 >> _8;
_4 = 51714_u16 as isize;
_6 = [8997492485328788296_u64,7921413864325852476_u64,18304045843003029554_u64,2973234705311571364_u64,11702728748354633876_u64,16338442319829491760_u64,17408229500375329380_u64,4313453320606669791_u64];
_12 = '\u{2b8cc}';
_2 = -RET;
_11 = !7_usize;
_11 = 170864852_i32 as usize;
RET = 843367779_u32 as isize;
_7 = _2;
RET = _5 - _9;
_7 = _1 * _9;
_11 = 7_usize * 3368579559884304348_usize;
_1 = 462103191_i32 as isize;
_13.1 = !_2;
_13.3 = -406_i16;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(9_usize, 7_usize, Move(_7), 12_usize, Move(_12), 5_usize, Move(_5), 9_usize, Move(_9)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_14 = dump_var(9_usize, 1_usize, Move(_1), 2_usize, Move(_2), 15_usize, _15, 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize) -> i128 {
mir! {
type RET = i128;
let _6: [i128; 7];
let _7: [isize; 6];
let _8: [i128; 7];
let _9: isize;
let _10: [i128; 2];
let _11: i32;
let _12: Adt46;
let _13: [i8; 5];
let _14: f32;
let _15: f32;
let _16: isize;
let _17: f64;
let _18: [i64; 8];
let _19: u8;
let _20: Adt50;
let _21: [i8; 5];
let _22: u8;
let _23: [u64; 8];
let _24: (u128,);
let _25: (u128,);
let _26: f32;
let _27: isize;
let _28: i8;
let _29: [i128; 7];
let _30: [u64; 8];
let _31: usize;
let _32: u16;
let _33: bool;
let _34: &'static char;
let _35: char;
let _36: u64;
let _37: bool;
let _38: [i128; 2];
let _39: (char,);
let _40: char;
let _41: Adt44;
let _42: [u64; 8];
let _43: [i64; 8];
let _44: [usize; 4];
let _45: (u128,);
let _46: ();
let _47: ();
{
RET = 1340966267_i32 as i128;
_2 = !_4;
RET = 970975617454917332399171949918024751_i128 * 119032054911546025316990099356747274865_i128;
_3 = _4;
_2 = _1 >> _3;
RET = 213_u8 as i128;
_7 = [_3,_2,_3,_2,_1,_4];
_7 = [_2,_1,_1,_4,_1,_1];
_8 = [RET,RET,RET,RET,RET,RET,RET];
_3 = _2;
_9 = _1 >> _3;
_3 = (-87_i8) as isize;
_7 = [_4,_9,_2,_2,_2,_1];
RET = 158679187970449835196873971194185948009_i128 * (-76541784230084945132115643659562683420_i128);
RET = (-31728033872380504394317159708088970600_i128) ^ (-110951149935432363050366175073415828093_i128);
_3 = _9 * _1;
_1 = 295539435857719580478988387758396754952_u128 as isize;
_6 = [RET,RET,RET,RET,RET,RET,RET];
Call(_7 = fn11(_2, _4, _2, _9, _3, _9, _2, _4, _2, _9, _4, _3, _3, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = _4;
_10 = [RET,RET];
_1 = -_9;
RET = (-38148325411931436308845806616288559776_i128);
_11 = 166876388_i32 << _2;
_8 = _6;
_7 = [_3,_9,_9,_9,_4,_1];
_10 = [RET,RET];
_5 = true as isize;
_8 = [RET,RET,RET,RET,RET,RET,RET];
_5 = _3 - _2;
_2 = _1 - _1;
_2 = _1 * _1;
_10 = [RET,RET];
Goto(bb2)
}
bb2 = {
_14 = 8031699626354964049_usize as f32;
_10 = [RET,RET];
_15 = 10508_i16 as f32;
_9 = _2;
_2 = !_3;
_9 = _2 >> _3;
_14 = _15;
_4 = _5;
_4 = 59754_u16 as isize;
_16 = (-115_i8) as isize;
_14 = _15;
_7 = [_2,_1,_2,_9,_5,_5];
RET = (-97205134745569508539148860728265785773_i128);
_13 = [31_i8,(-78_i8),29_i8,(-65_i8),(-112_i8)];
match RET {
243077232175368954924225746703502425683 => bb3,
_ => bb1
}
}
bb3 = {
_10 = [RET,RET];
_18 = [(-5199938606455867342_i64),4028061759453787959_i64,(-2149172936078683736_i64),(-5561149369117587460_i64),683588080179346146_i64,(-3997687968845799200_i64),6186934940989934126_i64,3214287013937744136_i64];
RET = 136545678083505443564629484746740674690_i128 * 84794837968630989830062101511840636229_i128;
_9 = -_2;
_9 = '\u{90959}' as isize;
_18 = [(-6429650641731514505_i64),(-8589935570975749624_i64),(-2701046239439030995_i64),(-4366747819418729268_i64),3746140849191174897_i64,(-3339590468307521235_i64),(-4617112446922176110_i64),(-351414017163246690_i64)];
_19 = '\u{ca16a}' as u8;
_13 = [30_i8,52_i8,(-10_i8),84_i8,40_i8];
_8 = [RET,RET,RET,RET,RET,RET,RET];
_6 = [RET,RET,RET,RET,RET,RET,RET];
_13 = [49_i8,64_i8,(-51_i8),72_i8,(-79_i8)];
RET = _11 as i128;
_7 = [_1,_1,_5,_1,_2,_1];
_18 = [8259577046266635914_i64,2596648116251660108_i64,1710875170545798361_i64,(-3960311252215915838_i64),1760516328805239149_i64,2810347370682223895_i64,3186552576866668792_i64,(-5399903900663545376_i64)];
RET = (-147364848727196717217619833026320578134_i128);
_18 = [8043498480452666350_i64,7537572009622533426_i64,(-3919929643547192192_i64),(-7622309978534118382_i64),8686718538918444668_i64,(-2331045135595749267_i64),(-3136045645757983426_i64),(-7139533301361759737_i64)];
_6 = [RET,RET,RET,RET,RET,RET,RET];
_6 = [RET,RET,RET,RET,RET,RET,RET];
_15 = _14 + _14;
Call(_2 = fn12(_3, _1, _1, _11, _7, _7, _7, _5, _7, _1, _3, _1, _3, _1, _11, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_15 = _14;
_9 = _1;
_4 = 160107270355243297599646536218530752901_u128 as isize;
_8 = [RET,RET,RET,RET,RET,RET,RET];
_20 = Adt50::Variant2 { fld0: 55671_u16,fld1: _13 };
place!(Field::<[i8; 5]>(Variant(_20, 2), 1)) = [(-113_i8),(-73_i8),4_i8,(-50_i8),(-54_i8)];
_7 = [_2,_5,_9,_5,_3,_2];
_13 = Field::<[i8; 5]>(Variant(_20, 2), 1);
_24 = (46184327436817789401989193855342452391_u128,);
_17 = _2 as f64;
_25.0 = !_24.0;
_7 = [_3,_2,_2,_3,_2,_5];
_24 = _25;
_4 = !_1;
_6 = [RET,RET,RET,RET,RET,RET,RET];
_9 = _4;
_26 = (-7557103779936153470_i64) as f32;
_29 = _6;
_2 = !_5;
Goto(bb5)
}
bb5 = {
_15 = _14 - _26;
_24 = (_25.0,);
_2 = _1 - _5;
_16 = _4;
_18 = [(-4975805464481426939_i64),(-4432637410533694782_i64),(-6511851036122163562_i64),1627995529535154254_i64,3758720394213471882_i64,(-5483231821475847871_i64),3396944400360837131_i64,3423502239658982232_i64];
_8 = _6;
_2 = _9 >> _1;
_30 = [6984349063465206745_u64,13790006805517717388_u64,8920353459751606437_u64,8185676581492696879_u64,15517533088777951775_u64,13397754455817512572_u64,17586612836036278149_u64,3862511970044218168_u64];
_30 = [11215189783851206571_u64,5876282031204715987_u64,4101395998285315179_u64,5397103434245387519_u64,7154797754219796890_u64,2930319929712186277_u64,6462519909033543189_u64,148458553312802405_u64];
_24.0 = (-1_i8) as u128;
place!(Field::<u16>(Variant(_20, 2), 0)) = !25112_u16;
Call(_3 = core::intrinsics::transmute(_4), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_5 = !_9;
Call(_1 = core::intrinsics::transmute(_5), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_23 = [9106200195866447136_u64,790341651320870708_u64,10684751678030472025_u64,7085546524963185151_u64,5925531585986854069_u64,8702809578996394030_u64,14775395082007938351_u64,9753395668991623701_u64];
_21 = [58_i8,(-9_i8),(-81_i8),94_i8,16_i8];
_16 = _1 >> _2;
_14 = -_26;
_16 = _24.0 as isize;
Goto(bb8)
}
bb8 = {
_10 = [RET,RET];
_23 = [7411863464934536396_u64,8648262307055340862_u64,7482006398439994565_u64,10253757978648698080_u64,5910572505926952008_u64,3716054540184599340_u64,12486172030762728880_u64,15798749078426951408_u64];
_4 = !_1;
_10 = [RET,RET];
_16 = _9 & _1;
_22 = _19;
_7 = [_9,_2,_9,_5,_1,_3];
_18 = [6221396605678831797_i64,167565344526383143_i64,(-5867114243343220733_i64),(-3992500661701774555_i64),(-3514421111822964029_i64),(-510842682101812174_i64),4191875262232792587_i64,(-8100660434703554516_i64)];
_14 = _15;
_4 = 0_usize as isize;
_26 = _14;
_12 = Adt46::Variant1 { fld0: _17 };
_26 = _15;
_13 = [31_i8,(-43_i8),(-98_i8),92_i8,35_i8];
_13 = [(-18_i8),77_i8,(-123_i8),112_i8,56_i8];
SetDiscriminant(_12, 1);
_1 = _2;
_6 = [RET,RET,RET,RET,RET,RET,RET];
_31 = 6_usize - 4_usize;
_12 = Adt46::Variant1 { fld0: _17 };
_3 = _9 - _2;
_28 = (-48_i8) - 66_i8;
_35 = '\u{b14d0}';
_32 = Field::<u16>(Variant(_20, 2), 0);
_25.0 = RET as u128;
_17 = _25.0 as f64;
Call(_36 = fn13(_7, _1, _11, _11, _9, _9, _7, _16, _7, _5, _11, Move(_20), _7, _2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_23 = _30;
_32 = 41736_u16 << _3;
place!(Field::<f64>(Variant(_12, 1), 0)) = _31 as f64;
Goto(bb10)
}
bb10 = {
_38 = [RET,RET];
SetDiscriminant(_12, 0);
place!(Field::<(char,)>(Variant(_12, 0), 3)).0 = _35;
RET = (-31477140785897901281370916744863029498_i128);
_5 = _1;
_5 = _1 + _2;
_7 = [_3,_1,_5,_1,_16,_1];
_16 = _17 as isize;
_31 = 17574754638268984076_usize;
_33 = _3 == _2;
_28 = -15_i8;
_38 = [RET,RET];
_28 = -62_i8;
_36 = 13356807244744160816_u64 | 4407020955456305084_u64;
place!(Field::<(char,)>(Variant(_12, 0), 3)).0 = _35;
match _31 {
0 => bb11,
17574754638268984076 => bb13,
_ => bb12
}
}
bb11 = {
_10 = [RET,RET];
_18 = [(-5199938606455867342_i64),4028061759453787959_i64,(-2149172936078683736_i64),(-5561149369117587460_i64),683588080179346146_i64,(-3997687968845799200_i64),6186934940989934126_i64,3214287013937744136_i64];
RET = 136545678083505443564629484746740674690_i128 * 84794837968630989830062101511840636229_i128;
_9 = -_2;
_9 = '\u{90959}' as isize;
_18 = [(-6429650641731514505_i64),(-8589935570975749624_i64),(-2701046239439030995_i64),(-4366747819418729268_i64),3746140849191174897_i64,(-3339590468307521235_i64),(-4617112446922176110_i64),(-351414017163246690_i64)];
_19 = '\u{ca16a}' as u8;
_13 = [30_i8,52_i8,(-10_i8),84_i8,40_i8];
_8 = [RET,RET,RET,RET,RET,RET,RET];
_6 = [RET,RET,RET,RET,RET,RET,RET];
_13 = [49_i8,64_i8,(-51_i8),72_i8,(-79_i8)];
RET = _11 as i128;
_7 = [_1,_1,_5,_1,_2,_1];
_18 = [8259577046266635914_i64,2596648116251660108_i64,1710875170545798361_i64,(-3960311252215915838_i64),1760516328805239149_i64,2810347370682223895_i64,3186552576866668792_i64,(-5399903900663545376_i64)];
RET = (-147364848727196717217619833026320578134_i128);
_18 = [8043498480452666350_i64,7537572009622533426_i64,(-3919929643547192192_i64),(-7622309978534118382_i64),8686718538918444668_i64,(-2331045135595749267_i64),(-3136045645757983426_i64),(-7139533301361759737_i64)];
_6 = [RET,RET,RET,RET,RET,RET,RET];
_6 = [RET,RET,RET,RET,RET,RET,RET];
_15 = _14 + _14;
Call(_2 = fn12(_3, _1, _1, _11, _7, _7, _7, _5, _7, _1, _3, _1, _3, _1, _11, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_10 = [RET,RET];
_23 = [7411863464934536396_u64,8648262307055340862_u64,7482006398439994565_u64,10253757978648698080_u64,5910572505926952008_u64,3716054540184599340_u64,12486172030762728880_u64,15798749078426951408_u64];
_4 = !_1;
_10 = [RET,RET];
_16 = _9 & _1;
_22 = _19;
_7 = [_9,_2,_9,_5,_1,_3];
_18 = [6221396605678831797_i64,167565344526383143_i64,(-5867114243343220733_i64),(-3992500661701774555_i64),(-3514421111822964029_i64),(-510842682101812174_i64),4191875262232792587_i64,(-8100660434703554516_i64)];
_14 = _15;
_4 = 0_usize as isize;
_26 = _14;
_12 = Adt46::Variant1 { fld0: _17 };
_26 = _15;
_13 = [31_i8,(-43_i8),(-98_i8),92_i8,35_i8];
_13 = [(-18_i8),77_i8,(-123_i8),112_i8,56_i8];
SetDiscriminant(_12, 1);
_1 = _2;
_6 = [RET,RET,RET,RET,RET,RET,RET];
_31 = 6_usize - 4_usize;
_12 = Adt46::Variant1 { fld0: _17 };
_3 = _9 - _2;
_28 = (-48_i8) - 66_i8;
_35 = '\u{b14d0}';
_32 = Field::<u16>(Variant(_20, 2), 0);
_25.0 = RET as u128;
_17 = _25.0 as f64;
Call(_36 = fn13(_7, _1, _11, _11, _9, _9, _7, _16, _7, _5, _11, Move(_20), _7, _2), ReturnTo(bb9), UnwindUnreachable())
}
bb13 = {
_2 = -_3;
_8 = [RET,RET,RET,RET,RET,RET,RET];
_33 = !true;
_34 = &_35;
_3 = _36 as isize;
_15 = -_14;
_36 = 12982368157300679489_u64;
place!(Field::<[i16; 2]>(Variant(_12, 0), 4)) = [(-26370_i16),14569_i16];
_5 = !_9;
_38 = [RET,RET];
place!(Field::<f64>(Variant(_12, 0), 1)) = -_17;
_27 = _2;
_10 = [RET,RET];
_19 = _36 as u8;
place!(Field::<i128>(Variant(_12, 0), 2)) = RET - RET;
_17 = _32 as f64;
_16 = (*_34) as isize;
_40 = Field::<(char,)>(Variant(_12, 0), 3).0;
_27 = _31 as isize;
match _36 {
12982368157300679489 => bb14,
_ => bb4
}
}
bb14 = {
_27 = _9;
_42 = [_36,_36,_36,_36,_36,_36,_36,_36];
place!(Field::<(char,)>(Variant(_12, 0), 3)) = (_35,);
_13 = [_28,_28,_28,_28,_28];
_19 = _33 as u8;
_42 = _23;
_33 = !true;
_14 = -_26;
_15 = _19 as f32;
_3 = _5;
_44 = [_31,_31,_31,_31];
_27 = _3;
_36 = 8938457195055392656_u64 | 14714382232697952333_u64;
_5 = _3;
_30 = _23;
_37 = _27 == _27;
_25 = (_24.0,);
_40 = (*_34);
_38 = [Field::<i128>(Variant(_12, 0), 2),Field::<i128>(Variant(_12, 0), 2)];
_2 = _14 as isize;
_27 = _1;
place!(Field::<f64>(Variant(_12, 0), 1)) = _36 as f64;
_33 = !_37;
place!(Field::<[i16; 2]>(Variant(_12, 0), 4)) = [(-27032_i16),(-4397_i16)];
_31 = 7698391354126082738_usize;
_8 = _6;
_37 = _17 != _17;
_1 = !_9;
Goto(bb15)
}
bb15 = {
Call(_46 = dump_var(10_usize, 11_usize, Move(_11), 10_usize, Move(_10), 7_usize, Move(_7), 29_usize, Move(_29)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_46 = dump_var(10_usize, 23_usize, Move(_23), 5_usize, Move(_5), 24_usize, Move(_24), 32_usize, Move(_32)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(10_usize, 19_usize, Move(_19), 3_usize, Move(_3), 40_usize, Move(_40), 21_usize, Move(_21)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_46 = dump_var(10_usize, 42_usize, Move(_42), 18_usize, Move(_18), 44_usize, Move(_44), 9_usize, Move(_9)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_46 = dump_var(10_usize, 38_usize, Move(_38), 47_usize, _47, 47_usize, _47, 47_usize, _47), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize) -> [isize; 6] {
mir! {
type RET = [isize; 6];
let _15: ((&'static char, char, *const usize), isize);
let _16: Adt53;
let _17: isize;
let _18: ();
let _19: ();
{
_4 = _7;
_15.1 = (-683162090_i32) as isize;
_13 = !_14;
RET = [_8,_14,_5,_10,_3,_5];
_13 = 49208_u16 as isize;
_6 = _1;
_10 = _11 >> _9;
_15.0.0 = &_15.0.1;
_9 = _8 & _11;
_6 = 1135620024_u32 as isize;
_2 = 60208_u16 as isize;
_6 = _5 << _4;
_2 = (-492602838_i32) as isize;
_15.0.0 = &_15.0.1;
_10 = !_4;
_13 = -_5;
_3 = !_13;
_8 = _4 >> _11;
Goto(bb1)
}
bb1 = {
Call(_18 = dump_var(11_usize, 8_usize, Move(_8), 7_usize, Move(_7), 6_usize, Move(_6), 4_usize, Move(_4)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_18 = dump_var(11_usize, 14_usize, Move(_14), 9_usize, Move(_9), 5_usize, Move(_5), 19_usize, _19), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: i32,mut _5: [isize; 6],mut _6: [isize; 6],mut _7: [isize; 6],mut _8: isize,mut _9: [isize; 6],mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize,mut _15: i32,mut _16: isize) -> isize {
mir! {
type RET = isize;
let _17: (u128,);
let _18: isize;
let _19: ();
let _20: ();
{
_11 = _16 + _10;
_2 = _1 - _12;
RET = !_12;
_16 = RET * _10;
_7 = _9;
_15 = _4;
_15 = _4 >> _13;
_9 = [_16,_16,_3,_2,_11,_8];
_8 = RET >> RET;
_10 = _8 + _3;
_5 = [_16,_12,_3,RET,_11,_10];
_15 = 36304_u16 as i32;
RET = _11;
_11 = 21435_i16 as isize;
_9 = _7;
_15 = _4;
_2 = _16 << _10;
_16 = _2;
_4 = _15;
_8 = _2;
_5 = [_14,_13,_2,RET,_13,_13];
_13 = _15 as isize;
_5 = [_2,_1,_12,_3,_16,_14];
Goto(bb1)
}
bb1 = {
Call(_19 = dump_var(12_usize, 6_usize, Move(_6), 15_usize, Move(_15), 2_usize, Move(_2), 16_usize, Move(_16)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_19 = dump_var(12_usize, 5_usize, Move(_5), 4_usize, Move(_4), 1_usize, Move(_1), 12_usize, Move(_12)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: [isize; 6],mut _2: isize,mut _3: i32,mut _4: i32,mut _5: isize,mut _6: isize,mut _7: [isize; 6],mut _8: isize,mut _9: [isize; 6],mut _10: isize,mut _11: i32,mut _12: Adt50,mut _13: [isize; 6],mut _14: isize) -> u64 {
mir! {
type RET = u64;
let _15: f64;
let _16: ();
let _17: ();
{
_8 = _6 * _2;
RET = !8981219836756468273_u64;
place!(Field::<[i8; 5]>(Variant(_12, 2), 1)) = [11_i8,(-57_i8),(-37_i8),31_i8,16_i8];
_9 = [_6,_14,_5,_5,_2,_5];
_5 = -_8;
_11 = !_3;
_8 = (-8168_i16) as isize;
place!(Field::<u16>(Variant(_12, 2), 0)) = (-8444_i16) as u16;
_4 = !_11;
RET = !3061637374799654772_u64;
_4 = !_3;
_5 = !_14;
Goto(bb1)
}
bb1 = {
_7 = [_14,_5,_2,_10,_10,_6];
_14 = _11 as isize;
_15 = (-102_i8) as f64;
place!(Field::<u16>(Variant(_12, 2), 0)) = RET as u16;
_3 = _11 - _11;
RET = !11273842717665030209_u64;
_14 = !_5;
RET = _11 as u64;
_7 = _1;
Goto(bb2)
}
bb2 = {
Call(_16 = dump_var(13_usize, 6_usize, Move(_6), 10_usize, Move(_10), 2_usize, Move(_2), 11_usize, Move(_11)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_16 = dump_var(13_usize, 9_usize, Move(_9), 4_usize, Move(_4), 17_usize, _17, 17_usize, _17), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: u8,mut _2: u8,mut _3: isize,mut _4: [u64; 8],mut _5: ([u64; 8], i128, i128, i8, (*const usize, i16)),mut _6: isize,mut _7: isize,mut _8: (u8, isize, i8, i16, f32),mut _9: ([u64; 8], i128, i128, i8, (*const usize, i16))) -> isize {
mir! {
type RET = isize;
let _10: [u32; 3];
let _11: [i16; 2];
let _12: [usize; 4];
let _13: isize;
let _14: ();
let _15: ();
{
_9.2 = _5.2 | _5.2;
_9.1 = _9.2;
_6 = !_7;
_4 = _5.0;
_5.2 = false as i128;
_8.3 = _5.4.1 & _9.4.1;
RET = _6;
_10 = [3981813953_u32,2369376821_u32,3744310874_u32];
_5.3 = _8.2;
_5.1 = '\u{13bde}' as i128;
_11 = [_8.3,_9.4.1];
_4 = _9.0;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(14_usize, 2_usize, Move(_2), 10_usize, Move(_10), 11_usize, Move(_11), 6_usize, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: i16,mut _2: i8,mut _3: i128,mut _4: [isize; 6],mut _5: i8,mut _6: i16,mut _7: i16,mut _8: i16,mut _9: i16,mut _10: i8,mut _11: i16,mut _12: char,mut _13: [isize; 6],mut _14: isize,mut _15: [u64; 8],mut _16: i16) -> Adt57 {
mir! {
type RET = Adt57;
let _17: char;
let _18: Adt50;
let _19: (u128,);
let _20: [i8; 5];
let _21: u8;
let _22: [u32; 3];
let _23: (&'static char, char, *const usize);
let _24: f32;
let _25: [i128; 2];
let _26: [i128; 7];
let _27: Adt53;
let _28: (char,);
let _29: (u8, isize, i8, i16, f32);
let _30: Adt51;
let _31: i128;
let _32: u64;
let _33: char;
let _34: bool;
let _35: [i8; 5];
let _36: (char,);
let _37: [i64; 8];
let _38: Adt44;
let _39: i32;
let _40: Adt46;
let _41: isize;
let _42: [i64; 8];
let _43: f64;
let _44: [i128; 2];
let _45: isize;
let _46: f64;
let _47: [usize; 4];
let _48: [i128; 2];
let _49: bool;
let _50: isize;
let _51: bool;
let _52: bool;
let _53: f32;
let _54: ();
let _55: ();
{
_4 = _13;
_6 = _11 + _16;
_6 = -_16;
_6 = _8 << _14;
_9 = !_1;
_2 = _5;
_1 = _9 >> _8;
_13 = [_14,_14,_14,_14,_14,_14];
_4 = [_14,_14,_14,_14,_14,_14];
Call(_9 = fn16(_7, _7, _8, _16, _16, _4, _11, _1, _3, _11, _7, _1, _3, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = _9 & _8;
_11 = _8;
_3 = !(-21194090397972747662315076876857171829_i128);
_14 = 1402128444_u32 as isize;
_11 = -_9;
_13 = [_14,_14,_14,_14,_14,_14];
_2 = _10;
_14 = _12 as isize;
_17 = _12;
_10 = -_2;
_17 = _12;
_3 = (-36144251814188584537657411067638053239_i128);
_8 = _6 & _11;
_7 = _14 as i16;
Goto(bb2)
}
bb2 = {
_15 = [5279462973070390479_u64,16902940570041873359_u64,9198029428994151995_u64,2596604443492712157_u64,15743280177698435726_u64,1997899479760983593_u64,5984486856316273002_u64,2937104397689773418_u64];
_5 = _10;
_12 = _17;
_13 = [_14,_14,_14,_14,_14,_14];
_10 = _2 << _6;
_13 = [_14,_14,_14,_14,_14,_14];
_8 = _1;
_12 = _17;
_3 = -81766757111982973417190984916691524249_i128;
_2 = _10;
_6 = 13817244838827305910_usize as i16;
_5 = _10 - _10;
_2 = 192348810516291847725219509921250620694_u128 as i8;
_2 = _10 - _10;
_16 = _9 * _8;
_15 = [14324115421391262029_u64,7980026326033096258_u64,3008784522025079857_u64,923672387090609500_u64,17150145184949495541_u64,17688024697546497107_u64,3710489066077064580_u64,6078260517991705429_u64];
_14 = -(-9223372036854775808_isize);
_21 = 116_u8;
Goto(bb3)
}
bb3 = {
_24 = 7289076461688063124_i64 as f32;
_9 = -_1;
_16 = !_11;
_23.1 = _12;
_20 = [_2,_2,_5,_2,_5];
_14 = 108_isize >> _9;
_12 = _17;
_25 = [_3,_3];
_19 = (229961139413951034204643226721544329953_u128,);
_23.0 = &_12;
_19 = (78506507961665023362256157423851766173_u128,);
_19 = (68466040753749106838620148449649245620_u128,);
_12 = _23.1;
_3 = _17 as i128;
_7 = _16 * _8;
match _21 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
116 => bb8,
_ => bb7
}
}
bb4 = {
_15 = [5279462973070390479_u64,16902940570041873359_u64,9198029428994151995_u64,2596604443492712157_u64,15743280177698435726_u64,1997899479760983593_u64,5984486856316273002_u64,2937104397689773418_u64];
_5 = _10;
_12 = _17;
_13 = [_14,_14,_14,_14,_14,_14];
_10 = _2 << _6;
_13 = [_14,_14,_14,_14,_14,_14];
_8 = _1;
_12 = _17;
_3 = -81766757111982973417190984916691524249_i128;
_2 = _10;
_6 = 13817244838827305910_usize as i16;
_5 = _10 - _10;
_2 = 192348810516291847725219509921250620694_u128 as i8;
_2 = _10 - _10;
_16 = _9 * _8;
_15 = [14324115421391262029_u64,7980026326033096258_u64,3008784522025079857_u64,923672387090609500_u64,17150145184949495541_u64,17688024697546497107_u64,3710489066077064580_u64,6078260517991705429_u64];
_14 = -(-9223372036854775808_isize);
_21 = 116_u8;
Goto(bb3)
}
bb5 = {
_6 = _9 & _8;
_11 = _8;
_3 = !(-21194090397972747662315076876857171829_i128);
_14 = 1402128444_u32 as isize;
_11 = -_9;
_13 = [_14,_14,_14,_14,_14,_14];
_2 = _10;
_14 = _12 as isize;
_17 = _12;
_10 = -_2;
_17 = _12;
_3 = (-36144251814188584537657411067638053239_i128);
_8 = _6 & _11;
_7 = _14 as i16;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_19 = (139840409120664743581234527712277444804_u128,);
_29.0 = _3 as u8;
_13 = [_14,_14,_14,_14,_14,_14];
_29 = (_21, _14, _10, _7, _24);
_22 = [1189747970_u32,1909246230_u32,3586818644_u32];
_29.2 = _5;
_29.0 = 3896817169_u32 as u8;
_21 = _24 as u8;
_15 = [14664128119568134521_u64,18276697079113512402_u64,14267108026937710582_u64,16680108154585608396_u64,12986575892442058363_u64,6384416179139219426_u64,2434583677865243706_u64,9048083186363004505_u64];
_5 = _29.2;
_28 = (_12,);
_12 = _17;
_20 = [_5,_5,_5,_10,_29.2];
_23.0 = &_28.0;
_26 = [_3,_3,_3,_3,_3,_3,_3];
Goto(bb9)
}
bb9 = {
_31 = !_3;
_33 = _12;
_29 = (_21, _14, _5, _7, _24);
_9 = _7;
_28.0 = _17;
_29.2 = _2 * _5;
_25 = [_3,_3];
_32 = !12809550529966418476_u64;
_6 = !_29.3;
_26 = [_31,_31,_31,_3,_3,_3,_3];
_7 = (-1379500959_i32) as i16;
_29.0 = !_21;
_24 = _29.4 + _29.4;
_26 = [_31,_3,_3,_31,_31,_31,_31];
_23.1 = _17;
_23.1 = _33;
_36 = (_28.0,);
_7 = _19.0 as i16;
RET = Adt57::Variant1 { fld0: true };
_4 = [_14,_14,_29.1,_14,_14,_14];
_35 = _20;
_29 = (_21, _14, _5, _16, _24);
_37 = [(-7959060741200238992_i64),(-5742875157040024593_i64),1109434178389194564_i64,2067859413592689495_i64,8783941766993598004_i64,6458984272777176052_i64,(-4611291146752936158_i64),(-7495795541107072438_i64)];
_28.0 = _23.1;
Goto(bb10)
}
bb10 = {
_29.1 = !_14;
_19.0 = 184239842864064154015951864480577528637_u128 * 11364157361892076936085193419272769690_u128;
_23.0 = &_23.1;
Goto(bb11)
}
bb11 = {
_19 = (206768829765890379789783921293311644540_u128,);
_3 = _31 * _31;
_2 = -_5;
_4 = [_14,_14,_14,_29.1,_14,_14];
RET = Adt57::Variant1 { fld0: true };
_21 = _17 as u8;
_26 = [_3,_31,_3,_3,_3,_3,_3];
place!(Field::<bool>(Variant(RET, 1), 0)) = !false;
_3 = 56559287_i32 as i128;
SetDiscriminant(RET, 0);
place!(Field::<Adt51>(Variant(RET, 0), 1)) = Adt51::Variant2 { fld0: _19.0 };
place!(Field::<(u64, [i8; 5], usize, i16, *const i32)>(Variant(RET, 0), 3)).4 = core::ptr::addr_of!(_39);
place!(Field::<(u64, [i8; 5], usize, i16, *const i32)>(Variant(RET, 0), 3)).0 = _23.1 as u64;
_30 = Move(Field::<Adt51>(Variant(RET, 0), 1));
_12 = _17;
RET = Adt57::Variant1 { fld0: true };
_6 = _8 + _9;
_6 = 1136525632_u32 as i16;
_31 = -_3;
_10 = -_5;
_22 = [2189886160_u32,3080891239_u32,1019543559_u32];
_23.0 = &_23.1;
_29.3 = _29.4 as i16;
_29.4 = _24;
Goto(bb12)
}
bb12 = {
_20 = [_29.2,_29.2,_5,_29.2,_10];
_37 = [(-6331241926444651371_i64),9179657144640803839_i64,3586571105062471814_i64,7283695529970248961_i64,(-8683794634349487919_i64),(-5706847713735520134_i64),6000148541283458366_i64,(-1664610598748714635_i64)];
_23.0 = &_33;
_4 = _13;
SetDiscriminant(_30, 2);
_16 = _31 as i16;
_23.0 = &_12;
_26 = [_31,_31,_3,_31,_3,_31,_31];
_28 = _36;
_2 = _29.2;
_30 = Adt51::Variant2 { fld0: _19.0 };
_20 = [_29.2,_29.2,_10,_5,_5];
_39 = !1005410498_i32;
_44 = _25;
_29 = (_21, _14, _10, _8, _24);
_22 = [3840309282_u32,2386942719_u32,365128044_u32];
_25 = [_31,_31];
Goto(bb13)
}
bb13 = {
_42 = [7771487014275150731_i64,(-1503046013293245677_i64),4783361270774212432_i64,8914807410487679915_i64,8896764144880451533_i64,(-4726522177787477483_i64),4523781041623639671_i64,5216751454692787559_i64];
_41 = _14 | _14;
_14 = -_41;
SetDiscriminant(_30, 0);
place!(Field::<i16>(Variant(_30, 0), 4)) = _11 & _9;
_29.2 = -_2;
place!(Field::<char>(Variant(_30, 0), 1)) = _17;
_45 = _29.2 as isize;
_41 = _45 * _45;
_4 = _13;
_25 = [_3,_31];
_19.0 = !142309116509473344904374088370415007665_u128;
_23.0 = &_28.0;
place!(Field::<u8>(Variant(_30, 0), 0)) = _21 + _21;
place!(Field::<isize>(Variant(_30, 0), 2)) = _45 >> _9;
_4 = [_14,Field::<isize>(Variant(_30, 0), 2),_14,Field::<isize>(Variant(_30, 0), 2),_41,_45];
place!(Field::<bool>(Variant(RET, 1), 0)) = false & true;
_13 = _4;
_5 = _2 << _29.2;
_23.0 = &_33;
Goto(bb14)
}
bb14 = {
_46 = _9 as f64;
SetDiscriminant(RET, 3);
place!(Field::<[i8; 5]>(Variant(_30, 0), 3)) = _20;
place!(Field::<[usize; 4]>(Variant(RET, 3), 3)) = [18364870855568051155_usize,551747051670606219_usize,15347822540598996022_usize,9789781344426932207_usize];
_3 = !_31;
place!(Field::<(char,)>(Variant(RET, 3), 4)) = (_17,);
place!(Field::<u64>(Variant(RET, 3), 1)) = _32 - _32;
SetDiscriminant(_30, 2);
place!(Field::<[usize; 4]>(Variant(RET, 3), 3)) = [6_usize,13948706164722694597_usize,12128607110394139594_usize,11465327835387621636_usize];
_22 = [3382060416_u32,167463464_u32,1487078170_u32];
place!(Field::<([u64; 8], i128, i128, i8, (*const usize, i16))>(Variant(RET, 3), 0)).0 = [_32,_32,Field::<u64>(Variant(RET, 3), 1),Field::<u64>(Variant(RET, 3), 1),Field::<u64>(Variant(RET, 3), 1),_32,Field::<u64>(Variant(RET, 3), 1),_32];
_34 = !false;
RET = Adt57::Variant1 { fld0: _34 };
_26 = [_3,_31,_3,_3,_3,_31,_3];
Goto(bb15)
}
bb15 = {
Call(_54 = dump_var(15_usize, 36_usize, Move(_36), 12_usize, Move(_12), 14_usize, Move(_14), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_54 = dump_var(15_usize, 35_usize, Move(_35), 5_usize, Move(_5), 41_usize, Move(_41), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_54 = dump_var(15_usize, 16_usize, Move(_16), 6_usize, Move(_6), 44_usize, Move(_44), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_54 = dump_var(15_usize, 3_usize, Move(_3), 17_usize, Move(_17), 8_usize, Move(_8), 39_usize, Move(_39)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_54 = dump_var(15_usize, 25_usize, Move(_25), 4_usize, Move(_4), 55_usize, _55, 55_usize, _55), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: i16,mut _2: i16,mut _3: i16,mut _4: i16,mut _5: i16,mut _6: [isize; 6],mut _7: i16,mut _8: i16,mut _9: i128,mut _10: i16,mut _11: i16,mut _12: i16,mut _13: i128,mut _14: i128) -> i16 {
mir! {
type RET = i16;
let _15: Adt57;
let _16: u16;
let _17: i8;
let _18: Adt46;
let _19: char;
let _20: [isize; 6];
let _21: f32;
let _22: i64;
let _23: isize;
let _24: f64;
let _25: *const usize;
let _26: ();
let _27: ();
{
_4 = 19206_u16 as i16;
_2 = _9 as i16;
_11 = '\u{1a511}' as i16;
_2 = !_10;
_11 = -_10;
_13 = 4277577253_u32 as i128;
_13 = _14;
_11 = !_2;
_4 = 56229_u16 as i16;
_6 = [9223372036854775807_isize,35_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-30_isize),(-82_isize)];
RET = '\u{10de18}' as i16;
_11 = _1;
_9 = -_13;
_6 = [(-75_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-55_isize),13_isize,(-119_isize)];
_11 = _5 | RET;
_13 = _9;
_1 = 1779907301_u32 as i16;
_10 = _7 & _5;
_17 = 55_i8 << _10;
_19 = '\u{653f3}';
_6 = [(-60_isize),87_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-89_isize)];
_16 = 37464_u16 + 3119_u16;
Call(_4 = core::intrinsics::bswap(_11), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = _5 - _2;
_13 = _9 << _9;
_6 = [(-29_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-109_isize),(-105_isize),9223372036854775807_isize];
_8 = -_5;
_1 = !_12;
_8 = !_10;
_12 = -_5;
_3 = _19 as i16;
_12 = _3;
_3 = -_8;
_14 = _13 << _2;
_16 = 3220_u16 - 56350_u16;
_16 = 4396372675598358917_i64 as u16;
_14 = !_13;
_23 = !(-9223372036854775808_isize);
_24 = _11 as f64;
_10 = !_3;
_21 = _13 as f32;
_7 = _10;
RET = !_10;
_24 = _21 as f64;
_13 = _17 as i128;
_17 = _23 as i8;
Goto(bb2)
}
bb2 = {
Call(_26 = dump_var(16_usize, 7_usize, Move(_7), 13_usize, Move(_13), 10_usize, Move(_10), 16_usize, Move(_16)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_26 = dump_var(16_usize, 19_usize, Move(_19), 5_usize, Move(_5), 6_usize, Move(_6), 12_usize, Move(_12)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_26 = dump_var(16_usize, 23_usize, Move(_23), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(13923133707259352858_u64), std::hint::black_box('\u{ce6e6}'), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(41_i8), std::hint::black_box(10411_i16), std::hint::black_box(3446804072_u32), std::hint::black_box(312403911664695712304489912191746970921_u128), std::hint::black_box((-98250575046389649443366927257852232035_i128)), std::hint::black_box(1700455952094579282_usize), std::hint::black_box(202_u8));
                
            }
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: [u64; 8],
fld1: f32,
fld2: *const bool,
fld3: [i8; 5],

},
Variant1{
fld0: [u32; 3],
fld1: *const bool,
fld2: (u8, isize, i8, i16, f32),
fld3: f32,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: [usize; 4],
fld1: (*const usize, i16),
fld2: f32,
fld3: u128,
fld4: u16,
fld5: u32,

},
Variant1{
fld0: bool,
fld1: i128,
fld2: usize,
fld3: f32,
fld4: i16,
fld5: (u64, [i8; 5], usize, i16, *const i32),

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: Adt45,
fld1: f64,
fld2: i128,
fld3: (char,),
fld4: [i16; 2],

},
Variant1{
fld0: f64,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: [u64; 8],
fld1: u8,
fld2: [i8; 5],
fld3: (u128,),
fld4: *const i32,
fld5: [u32; 3],
fld6: Adt45,
fld7: [i64; 8],

},
Variant1{
fld0: i16,

},
Variant2{
fld0: bool,
fld1: *mut [i128; 2],

},
Variant3{
fld0: ([u64; 8], i128, i128, i8, (*const usize, i16)),
fld1: u128,
fld2: f64,
fld3: [u64; 8],
fld4: usize,
fld5: i32,

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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: u8,
fld1: u32,
fld2: [isize; 6],
fld3: i8,
fld4: i16,
fld5: [i128; 2],
fld6: i64,

},
Variant1{
fld0: [u64; 8],
fld1: *const isize,
fld2: i16,

},
Variant2{
fld0: u32,
fld1: *mut [i128; 2],
fld2: ([u64; 8], i128, i128, i8, (*const usize, i16)),
fld3: *const isize,
fld4: i32,

},
Variant3{
fld0: u16,
fld1: f64,
fld2: isize,
fld3: usize,
fld4: *mut [i128; 2],
fld5: [isize; 6],

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: [i128; 2],
fld1: char,
fld2: Adt47,
fld3: u8,
fld4: [u32; 3],
}
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: *mut [i128; 2],
fld1: Adt47,
fld2: [u64; 8],
fld3: [i16; 2],
fld4: *const i32,
fld5: *mut i32,
fld6: (*const usize, i16),

},
Variant1{
fld0: (u128,),
fld1: Adt47,
fld2: [i128; 2],
fld3: *const u16,
fld4: *const usize,
fld5: i32,

},
Variant2{
fld0: u16,
fld1: [i8; 5],

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: u8,
fld1: char,
fld2: isize,
fld3: [i8; 5],
fld4: i16,

},
Variant1{
fld0: usize,
fld1: Adt47,
fld2: i64,
fld3: Adt44,

},
Variant2{
fld0: u128,

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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: *mut i32,
fld1: u16,
fld2: [isize; 6],
fld3: [i8; 5],
fld4: [usize; 4],
fld5: i32,
fld6: i64,

},
Variant1{
fld0: [i128; 2],
fld1: *const *const bool,
fld2: *const usize,

},
Variant2{
fld0: [i64; 8],
fld1: f32,
fld2: [i128; 7],
fld3: [usize; 4],
fld4: i32,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: Adt45,
fld1: *const bool,

},
Variant1{
fld0: [i128; 2],
fld1: *const usize,
fld2: Adt49,
fld3: (*const i32, i64),
fld4: (char,),
fld5: [i16; 2],
fld6: [i64; 8],
fld7: usize,

},
Variant2{
fld0: usize,
fld1: char,

},
Variant3{
fld0: i64,
fld1: (*const usize, i16),
fld2: f64,

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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: (u8, isize, i8, i16, f32),
fld1: [i64; 8],
fld2: usize,
fld3: [i16; 2],
fld4: *mut [i128; 2],
fld5: [u64; 8],
fld6: *const bool,
fld7: *const i32,

},
Variant1{
fld0: [usize; 4],
fld1: u8,
fld2: f32,

},
Variant2{
fld0: Adt46,
fld1: u32,
fld2: [i64; 8],
fld3: Adt49,
fld4: usize,
fld5: (*const i32, i64),

},
Variant3{
fld0: Adt49,
fld1: Adt47,
fld2: u32,
fld3: *const isize,

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
fld0: u64,
fld1: f64,
fld2: [i128; 7],
fld3: [u64; 8],
fld4: *const *const bool,
fld5: [i16; 2],
fld6: i64,

},
Variant1{
fld0: (*const usize, i16),
fld1: *mut i32,
fld2: *const bool,
fld3: Adt52,
fld4: f64,
fld5: (char,),

},
Variant2{
fld0: (*const i32, i64),

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: (*const usize, i16),
fld1: f32,
fld2: (*const i32, i64),
fld3: f64,
fld4: *mut i32,

},
Variant1{
fld0: Adt53,
fld1: *const *const bool,
fld2: i128,
fld3: i32,

},
Variant2{
fld0: [i64; 8],
fld1: usize,
fld2: Adt44,
fld3: Adt47,
fld4: [i16; 2],
fld5: Adt51,
fld6: i64,
fld7: [i128; 2],

},
Variant3{
fld0: (u128,),
fld1: [usize; 4],
fld2: Adt50,
fld3: Adt45,
fld4: u8,
fld5: i128,
fld6: Adt53,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: u64,
fld1: Adt51,
fld2: u8,
fld3: (u64, [i8; 5], usize, i16, *const i32),

},
Variant1{
fld0: bool,

},
Variant2{
fld0: ([u64; 8], i128, i128, i8, (*const usize, i16)),

},
Variant3{
fld0: ([u64; 8], i128, i128, i8, (*const usize, i16)),
fld1: u64,
fld2: *const isize,
fld3: [usize; 4],
fld4: (char,),
fld5: Adt48,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt58::".as_ptr()  as *const c_char)};match self{
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
fld0: usize,
fld1: [isize; 6],
fld2: [i128; 7],

},
Variant1{
fld0: (*const usize, i16),
fld1: Adt49,

},
Variant2{
fld0: Adt54,
fld1: char,
fld2: f64,
fld3: [u64; 8],
fld4: u8,
fld5: *const u16,

}}
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: Adt47,
fld1: char,
fld2: (char,),
fld3: *mut i32,
fld4: i16,
fld5: *const *const bool,
fld6: Adt46,
fld7: (*const i32, i64),

},
Variant1{
fld0: Adt44,
fld1: [u32; 3],
fld2: *mut [i128; 2],
fld3: *const usize,
fld4: [i128; 7],

},
Variant2{
fld0: Adt56,
fld1: char,
fld2: *mut [i128; 2],
fld3: i8,
fld4: Adt51,
fld5: [i8; 5],
fld6: Adt45,
fld7: [i16; 2],

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
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: u32,
fld1: Adt52,

},
Variant1{
fld0: [i128; 2],
fld1: [u32; 3],
fld2: Adt48,
fld3: [usize; 4],
fld4: Adt46,
fld5: [i8; 5],
fld6: (u8, isize, i8, i16, f32),
fld7: *const bool,

},
Variant2{
fld0: *mut i32,
fld1: i64,
fld2: [u64; 8],
fld3: Adt50,

}}

