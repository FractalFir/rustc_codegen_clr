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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> [i64; 5] {
mir! {
type RET = [i64; 5];
let _15: u8;
let _16: i128;
let _17: u128;
let _18: [i32; 1];
let _19: char;
let _20: i32;
let _21: isize;
let _22: isize;
let _23: Adt43;
let _24: isize;
let _25: i128;
let _26: Adt56;
let _27: Adt56;
let _28: i64;
let _29: [isize; 3];
let _30: [u128; 5];
let _31: [u128; 2];
let _32: char;
let _33: u64;
let _34: u128;
let _35: [i32; 6];
let _36: char;
let _37: u32;
let _38: char;
let _39: (*mut u64,);
let _40: Adt52;
let _41: Adt44;
let _42: [isize; 4];
let _43: Adt55;
let _44: ();
let _45: ();
{
_7 = 7260971311124780319_i64;
_5 = !26354_i16;
_10 = false as u8;
_14 = 5086191639927346074487587023613382222_u128 & 45205982599555601785074826896405846075_u128;
RET = [_7,_7,_7,_7,_7];
_1 = !true;
_14 = 1597172878_u32 as u128;
_13 = 12836087198798304040_u64;
_12 = !1320837220_u32;
RET = [_7,_7,_7,_7,_7];
_9 = 4_usize & 3_usize;
RET = [_7,_7,_7,_7,_7];
_7 = !7903076735475606412_i64;
_1 = !true;
_16 = _10 as i128;
_5 = 16723_i16 ^ 20111_i16;
_1 = true;
_16 = (-74918944511742696114420735683357560896_i128);
_7 = (-2658798224167716719_i64);
_3 = !9223372036854775807_isize;
_4 = 71_i8;
_8 = _16 * _16;
_7 = (-1257921832846190274_i64);
_8 = _7 as i128;
Goto(bb1)
}
bb1 = {
_11 = _9 as u16;
_10 = 46_u8 & 45_u8;
_12 = 3102685594_u32 ^ 1075206791_u32;
_11 = 42547_u16;
_15 = _10 | _10;
_15 = _10;
_2 = '\u{b04cd}';
_2 = '\u{888e0}';
_14 = 58253627509529569504588732175326085466_u128;
_15 = _10;
_11 = 1585_u16;
_2 = '\u{104e95}';
_5 = -18331_i16;
_13 = 1820944895_i32 as u64;
RET = [_7,_7,_7,_7,_7];
_4 = -(-128_i8);
match _14 {
0 => bb2,
1 => bb3,
2 => bb4,
58253627509529569504588732175326085466 => bb6,
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
_20 = -(-1270751254_i32);
_22 = _3;
_20 = !(-1984612704_i32);
_17 = !_14;
RET = [_7,_7,_7,_7,_7];
_19 = _2;
_7 = (-3515569664249744998_i64) << _13;
_6 = _20 >> _11;
_4 = -105_i8;
_10 = _17 as u8;
_16 = _8 + _8;
_20 = _6 + _6;
_25 = _16 ^ _16;
_1 = _10 <= _10;
_18 = [_6];
_11 = !29355_u16;
match _14 {
58253627509529569504588732175326085466 => bb8,
_ => bb7
}
}
bb7 = {
Return()
}
bb8 = {
_21 = _7 as isize;
_24 = -_21;
_2 = _19;
_10 = _15 >> _12;
_21 = _19 as isize;
_22 = _17 as isize;
RET = [_7,_7,_7,_7,_7];
_24 = _21 >> _10;
_15 = !_10;
_1 = false;
_7 = -(-3932336670990565199_i64);
_14 = _8 as u128;
_16 = -_8;
_14 = _17;
_21 = !_24;
_28 = _7;
_21 = _28 as isize;
_12 = 1035989558_u32;
_12 = !752922367_u32;
_4 = -60_i8;
_9 = 0_usize;
_3 = _24;
RET = [_28,_28,_28,_7,_28];
_29[_9] = -_24;
Call(_32 = fn1(_24, _24, _29[_9], _4, _18), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
RET = [_7,_28,_28,_28,_28];
_30 = [_14,_17,_14,_14,_17];
_19 = _2;
_12 = 1809584591_u32;
_16 = _12 as i128;
_31 = [_17,_14];
RET = [_28,_28,_28,_7,_7];
_9 = _12 as usize;
_35 = [_20,_6,_20,_20,_20,_20];
_28 = _7 + _7;
_7 = _28 - _28;
_34 = _17;
_28 = _7;
_10 = _15 + _15;
_15 = _9 as u8;
_16 = _12 as i128;
match _12 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
1809584591 => bb15,
_ => bb14
}
}
bb10 = {
_21 = _7 as isize;
_24 = -_21;
_2 = _19;
_10 = _15 >> _12;
_21 = _19 as isize;
_22 = _17 as isize;
RET = [_7,_7,_7,_7,_7];
_24 = _21 >> _10;
_15 = !_10;
_1 = false;
_7 = -(-3932336670990565199_i64);
_14 = _8 as u128;
_16 = -_8;
_14 = _17;
_21 = !_24;
_28 = _7;
_21 = _28 as isize;
_12 = 1035989558_u32;
_12 = !752922367_u32;
_4 = -60_i8;
_9 = 0_usize;
_3 = _24;
RET = [_28,_28,_28,_7,_28];
_29[_9] = -_24;
Call(_32 = fn1(_24, _24, _29[_9], _4, _18), ReturnTo(bb9), UnwindUnreachable())
}
bb11 = {
Return()
}
bb12 = {
_11 = _9 as u16;
_10 = 46_u8 & 45_u8;
_12 = 3102685594_u32 ^ 1075206791_u32;
_11 = 42547_u16;
_15 = _10 | _10;
_15 = _10;
_2 = '\u{b04cd}';
_2 = '\u{888e0}';
_14 = 58253627509529569504588732175326085466_u128;
_15 = _10;
_11 = 1585_u16;
_2 = '\u{104e95}';
_5 = -18331_i16;
_13 = 1820944895_i32 as u64;
RET = [_7,_7,_7,_7,_7];
_4 = -(-128_i8);
match _14 {
0 => bb2,
1 => bb3,
2 => bb4,
58253627509529569504588732175326085466 => bb6,
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
_18 = [_20];
_15 = _10 ^ _10;
_2 = _19;
_25 = -_16;
_37 = _12;
_20 = -_6;
_22 = _3;
_31 = [_14,_14];
RET = [_28,_28,_28,_28,_28];
_5 = !(-30149_i16);
_35 = [_20,_20,_6,_20,_20,_20];
_33 = _13;
_37 = _32 as u32;
_24 = _22 >> _3;
_2 = _19;
_5 = _33 as i16;
_14 = _34;
_25 = _16 >> _3;
_38 = _19;
_32 = _19;
_19 = _32;
_42 = [_3,_24,_24,_22];
RET = [_7,_7,_7,_7,_7];
_2 = _32;
_39.0 = core::ptr::addr_of_mut!(_33);
Goto(bb16)
}
bb16 = {
Call(_44 = dump_var(0_usize, 28_usize, Move(_28), 2_usize, Move(_2), 9_usize, Move(_9), 33_usize, Move(_33)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(0_usize, 4_usize, Move(_4), 18_usize, Move(_18), 17_usize, Move(_17), 10_usize, Move(_10)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(0_usize, 8_usize, Move(_8), 24_usize, Move(_24), 3_usize, Move(_3), 14_usize, Move(_14)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_44 = dump_var(0_usize, 13_usize, Move(_13), 32_usize, Move(_32), 5_usize, Move(_5), 31_usize, Move(_31)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_44 = dump_var(0_usize, 22_usize, Move(_22), 45_usize, _45, 45_usize, _45, 45_usize, _45), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: i8,mut _5: [i32; 1]) -> char {
mir! {
type RET = char;
let _6: f64;
let _7: usize;
let _8: [u32; 3];
let _9: [i64; 4];
let _10: [i32; 6];
let _11: i8;
let _12: Adt56;
let _13: i8;
let _14: [i32; 1];
let _15: bool;
let _16: *mut f32;
let _17: f32;
let _18: [usize; 4];
let _19: [i32; 1];
let _20: [i32; 1];
let _21: bool;
let _22: f64;
let _23: [usize; 4];
let _24: (u128, i8, [i64; 5]);
let _25: f32;
let _26: isize;
let _27: i128;
let _28: char;
let _29: i8;
let _30: isize;
let _31: u128;
let _32: ([u128; 5], usize, *mut f32, bool, *mut [u128; 5]);
let _33: Adt47;
let _34: (u128, i8, [i64; 5]);
let _35: [usize; 6];
let _36: [i32; 1];
let _37: [char; 3];
let _38: [i32; 6];
let _39: u8;
let _40: i64;
let _41: i128;
let _42: &'static u32;
let _43: [i32; 1];
let _44: ();
let _45: ();
{
_1 = _3;
_3 = _2;
RET = '\u{52360}';
_2 = !_1;
_3 = !_2;
RET = '\u{b0273}';
_2 = !_1;
_6 = (-3266827998475870389_i64) as f64;
_6 = 132_u8 as f64;
_5 = [2114637825_i32];
_8 = [3672660997_u32,4021460734_u32,1016115730_u32];
_7 = 5357609500842885184_usize * 7_usize;
_8 = [3235947719_u32,2564449042_u32,442405659_u32];
Goto(bb1)
}
bb1 = {
RET = '\u{3487f}';
_4 = (-6_i8);
RET = '\u{4f4a1}';
_10 = [(-1835428498_i32),(-384819330_i32),(-519062045_i32),1111687429_i32,1142477452_i32,(-939756664_i32)];
_3 = _2 | _1;
RET = '\u{25649}';
_6 = _7 as f64;
_10 = [890744452_i32,672696248_i32,(-175177855_i32),1721194381_i32,(-181798417_i32),(-1242080294_i32)];
_8 = [960581613_u32,1965702515_u32,865380541_u32];
_9 = [6527241183379197547_i64,(-1949468236704398898_i64),6888884988802610108_i64,3470521537680906392_i64];
_2 = 1484618651_u32 as isize;
_4 = -73_i8;
RET = '\u{e24ab}';
_9 = [8833280412710037797_i64,(-2999106561691980548_i64),7338034341959426838_i64,(-1224832248449758394_i64)];
_5 = [(-151169189_i32)];
_5 = [(-743868560_i32)];
_1 = !_3;
_9 = [957145772788775858_i64,(-2165070262353561371_i64),(-6140084368014524950_i64),1150290735283110611_i64];
_1 = 467752777_i32 as isize;
_6 = _3 as f64;
_11 = true as i8;
_8 = [1529786797_u32,2780839158_u32,863553509_u32];
Goto(bb2)
}
bb2 = {
_10 = [(-180746305_i32),1322852766_i32,(-590403929_i32),1287603461_i32,1419137466_i32,(-418722273_i32)];
_14 = [(-1844886161_i32)];
_13 = _4;
_3 = _1;
RET = '\u{dd3e3}';
_9 = [9161976812581040616_i64,(-2175977602841125318_i64),(-6970440837214083012_i64),(-1377435274514759995_i64)];
Goto(bb3)
}
bb3 = {
_4 = 20552_i16 as i8;
_6 = 1094917305_i32 as f64;
RET = '\u{4623c}';
_14 = [1441494475_i32];
_9 = [6017627687623104100_i64,(-7694171634023265550_i64),(-2473456750960889085_i64),5769340233294248709_i64];
_5 = [(-35669385_i32)];
_16 = core::ptr::addr_of_mut!(_17);
_15 = true;
_14 = [(-988888414_i32)];
_9 = [(-3883004624429030390_i64),2035165224136902937_i64,4678145001565972423_i64,1405206735480579273_i64];
_9 = [5894363156204875349_i64,7640995208600966329_i64,(-4161023965228765563_i64),7475893866834634178_i64];
_17 = 18058_i16 as f32;
_19 = [436232405_i32];
_16 = core::ptr::addr_of_mut!(_17);
_17 = _6 as f32;
_1 = 203_u8 as isize;
_17 = 70018563191496392879605380310297894550_u128 as f32;
(*_16) = 3632653624_u32 as f32;
_4 = (-168390449710395264146976893703273171819_i128) as i8;
RET = '\u{bfff0}';
_2 = -_1;
_2 = _3;
_14 = _5;
_9 = [(-3605090978752953055_i64),(-6329371538647554396_i64),2584400812056882851_i64,6452889353565556750_i64];
_17 = 68663300081946422429851706781425934747_u128 as f32;
Call(_1 = fn2(_11, RET, _8, _2, _6, _17, _13, _17, _8, _8, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_11 = _13 ^ _13;
_14 = [(-1848028587_i32)];
_6 = (*_16) as f64;
_20 = [(-1551343874_i32)];
_15 = true | true;
_17 = _1 as f32;
_8 = [3674561161_u32,2370953206_u32,3985647636_u32];
_9 = [1504821674062757514_i64,(-7300065828899190527_i64),(-9049436325873758955_i64),(-2758508944463525162_i64)];
_17 = 231_u8 as f32;
_7 = !3_usize;
_17 = 17729_i16 as f32;
Goto(bb5)
}
bb5 = {
_21 = (*_16) == (*_16);
(*_16) = _7 as f32;
(*_16) = 16251861023818171686_u64 as f32;
RET = '\u{1c619}';
_2 = _3;
Goto(bb6)
}
bb6 = {
_4 = !_13;
(*_16) = 36518_u16 as f32;
_5 = [(-1153481925_i32)];
_10 = [456279250_i32,1434573820_i32,1684407685_i32,(-757555262_i32),2072610114_i32,1846599016_i32];
_24.1 = 142777098908676292918418717053894077735_i128 as i8;
RET = '\u{e7869}';
_10 = [1262260752_i32,1743702570_i32,(-343528906_i32),508101781_i32,597993603_i32,(-900364088_i32)];
(*_16) = 20115_i16 as f32;
_10 = [(-2132856194_i32),(-326598775_i32),714161502_i32,1223107299_i32,191334854_i32,1839357393_i32];
_15 = !_21;
Goto(bb7)
}
bb7 = {
_7 = !0_usize;
_6 = 57_u8 as f64;
_20 = _19;
_22 = -_6;
_22 = -_6;
_18 = [_7,_7,_7,_7];
_24.2 = [(-7829098241819903494_i64),1891500331461717916_i64,(-1676805873874044026_i64),(-4784608394004653253_i64),(-1442122616847312919_i64)];
_4 = _24.1;
Goto(bb8)
}
bb8 = {
_17 = 43946_u16 as f32;
_9 = [67177370210587064_i64,7319078261185418847_i64,5244416439226328135_i64,3907847426383292680_i64];
_15 = !_21;
(*_16) = 1501042112_u32 as f32;
_21 = _7 < _7;
_5 = _14;
_15 = _21;
_8 = [52559967_u32,1182594055_u32,3729768842_u32];
_11 = (-8174_i16) as i8;
RET = '\u{11204}';
_20 = [(-1287623101_i32)];
_21 = _15;
_19 = _5;
_9 = [(-3355019638948400829_i64),2580561727128086506_i64,(-1420367836711962620_i64),7936399743835463721_i64];
_26 = !_2;
(*_16) = 11238261633598017243222056413178154525_u128 as f32;
_27 = !97119477174592896415723611657395157041_i128;
_2 = -_1;
_26 = _1 & _1;
_24.0 = 194967340017452784680435878782033561580_u128 & 163463815217916664567079136317486824342_u128;
_1 = _3 * _26;
Goto(bb9)
}
bb9 = {
_23 = [_7,_7,_7,_7];
_28 = RET;
_3 = _27 as isize;
_23 = [_7,_7,_7,_7];
_4 = _24.1;
_11 = _13;
Goto(bb10)
}
bb10 = {
_16 = core::ptr::addr_of_mut!((*_16));
RET = _28;
_13 = 9032_i16 as i8;
_25 = -_17;
_10 = [845318447_i32,731479197_i32,860630192_i32,(-1048104413_i32),988604727_i32,(-1874708851_i32)];
(*_16) = _6 as f32;
RET = _28;
_1 = _3 * _26;
_17 = -_25;
_10 = [1088303641_i32,(-1165467547_i32),1423855595_i32,(-1471206956_i32),22267554_i32,156248215_i32];
_4 = _24.0 as i8;
Goto(bb11)
}
bb11 = {
_14 = [(-1229036513_i32)];
_16 = core::ptr::addr_of_mut!(_17);
_31 = _24.0;
_32.2 = _16;
_8 = [439612526_u32,1891538657_u32,532887075_u32];
_22 = -_6;
_22 = _6 + _6;
_32.3 = _26 != _2;
_9 = [1617220574995840170_i64,3563180051472046643_i64,(-6498021217335462180_i64),(-7331091867865456345_i64)];
_28 = RET;
_29 = -_13;
(*_16) = _25;
_30 = _2;
_34.1 = _7 as i8;
_31 = 24481_i16 as u128;
_16 = core::ptr::addr_of_mut!((*_16));
_30 = -_1;
_32.0 = [_24.0,_31,_31,_24.0,_24.0];
_9 = [(-742722845542025689_i64),(-899181296508784869_i64),4244299185742940153_i64,5312123878391533084_i64];
_34.2 = [(-8763721853795356060_i64),5303282628105091442_i64,(-8689024979952207062_i64),2225903551847475381_i64,(-5320004249208844397_i64)];
_34.0 = 4042360935_u32 as u128;
_7 = 658910746_i32 as usize;
_24.2 = [(-4407423915567464507_i64),2915146926318355569_i64,(-2383442361968509278_i64),7842809673419409269_i64,8930154899612362763_i64];
_23 = [_7,_7,_7,_7];
_19 = [1402716013_i32];
_26 = _30;
_24.0 = !_34.0;
_4 = _22 as i8;
Goto(bb12)
}
bb12 = {
_32.1 = _7 ^ _7;
RET = _28;
_1 = _30;
_15 = _32.3 ^ _32.3;
_22 = -_6;
_3 = _1 >> _1;
_9 = [7358621078496638300_i64,(-2622416693827269437_i64),9085340412326499229_i64,8574026432441153512_i64];
_11 = _29 * _29;
Goto(bb13)
}
bb13 = {
_28 = RET;
_37 = [RET,_28,RET];
_36 = [1220655839_i32];
_34.1 = _4;
_1 = -_26;
_29 = -_11;
_17 = -_25;
_20 = [(-158835536_i32)];
_32.3 = !_15;
_32.4 = core::ptr::addr_of_mut!(_32.0);
(*_16) = _25;
_6 = _27 as f64;
_8 = [4144731932_u32,1693506312_u32,2281164259_u32];
_32.1 = _7 * _7;
_33 = Adt47::Variant1 { fld0: _7,fld1: 1227792624_i32,fld2: _1,fld3: _25 };
_32.2 = core::ptr::addr_of_mut!((*_16));
RET = _28;
_21 = _15;
_34 = _24;
_6 = _22 * _22;
_32.1 = !_7;
_28 = RET;
_38 = [(-829608626_i32),642598545_i32,(-64001130_i32),1941058236_i32,(-140421613_i32),(-398754973_i32)];
_10 = [(-514802266_i32),608288993_i32,340578509_i32,(-1786570554_i32),(-1910373649_i32),380414499_i32];
_2 = _1;
place!(Field::<i32>(Variant(_33, 1), 1)) = _29 as i32;
Call(_24.2 = core::intrinsics::transmute(_34.2), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_23 = [_32.1,_7,Field::<usize>(Variant(_33, 1), 0),_32.1];
_38 = [Field::<i32>(Variant(_33, 1), 1),Field::<i32>(Variant(_33, 1), 1),Field::<i32>(Variant(_33, 1), 1),Field::<i32>(Variant(_33, 1), 1),Field::<i32>(Variant(_33, 1), 1),Field::<i32>(Variant(_33, 1), 1)];
(*_16) = _22 as f32;
_37 = [RET,RET,_28];
_18 = _23;
_32.4 = core::ptr::addr_of_mut!(_32.0);
_6 = -_22;
(*_16) = -Field::<f32>(Variant(_33, 1), 3);
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(1_usize, 21_usize, Move(_21), 9_usize, Move(_9), 8_usize, Move(_8), 36_usize, Move(_36)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(1_usize, 30_usize, Move(_30), 11_usize, Move(_11), 13_usize, Move(_13), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(1_usize, 5_usize, Move(_5), 14_usize, Move(_14), 15_usize, Move(_15), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(1_usize, 18_usize, Move(_18), 27_usize, Move(_27), 45_usize, _45, 45_usize, _45), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: i8,mut _2: char,mut _3: [u32; 3],mut _4: isize,mut _5: f64,mut _6: f32,mut _7: i8,mut _8: f32,mut _9: [u32; 3],mut _10: [u32; 3],mut _11: isize) -> isize {
mir! {
type RET = isize;
let _12: u32;
let _13: i32;
let _14: isize;
let _15: Adt51;
let _16: [i32; 6];
let _17: i32;
let _18: usize;
let _19: isize;
let _20: f64;
let _21: f64;
let _22: isize;
let _23: isize;
let _24: f32;
let _25: bool;
let _26: u16;
let _27: bool;
let _28: ();
let _29: ();
{
_8 = _6;
RET = !_11;
RET = -_11;
Call(RET = fn3(_10, _2, _2, _11), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = [1596753821_u32,3584971617_u32,851838747_u32];
_10 = [1540122778_u32,175511131_u32,1815660434_u32];
_10 = _9;
_6 = _8 * _8;
_9 = [3601326639_u32,3020183597_u32,2413633516_u32];
RET = _11 >> _7;
_1 = !_7;
_12 = 589504315_u32;
_1 = 51963588995367068131862592398741608325_i128 as i8;
_5 = (-29654_i16) as f64;
_13 = 513729192_i32 >> _7;
_5 = RET as f64;
_13 = _2 as i32;
_11 = _7 as isize;
Call(_12 = core::intrinsics::bswap(2588036464_u32), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = !_4;
_10 = _3;
_6 = _8 - _8;
_6 = _8;
_3 = [_12,_12,_12];
_7 = _1;
_7 = _1 + _1;
_9 = [_12,_12,_12];
RET = _4;
_11 = RET * RET;
_16 = [_13,_13,_13,_13,_13,_13];
_12 = _13 as u32;
Goto(bb3)
}
bb3 = {
_6 = _8 * _8;
_12 = 2770246999_u32;
_2 = '\u{5b943}';
match _12 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
2770246999 => bb10,
_ => bb9
}
}
bb4 = {
RET = !_4;
_10 = _3;
_6 = _8 - _8;
_6 = _8;
_3 = [_12,_12,_12];
_7 = _1;
_7 = _1 + _1;
_9 = [_12,_12,_12];
RET = _4;
_11 = RET * RET;
_16 = [_13,_13,_13,_13,_13,_13];
_12 = _13 as u32;
Goto(bb3)
}
bb5 = {
_9 = [1596753821_u32,3584971617_u32,851838747_u32];
_10 = [1540122778_u32,175511131_u32,1815660434_u32];
_10 = _9;
_6 = _8 * _8;
_9 = [3601326639_u32,3020183597_u32,2413633516_u32];
RET = _11 >> _7;
_1 = !_7;
_12 = 589504315_u32;
_1 = 51963588995367068131862592398741608325_i128 as i8;
_5 = (-29654_i16) as f64;
_13 = 513729192_i32 >> _7;
_5 = RET as f64;
_13 = _2 as i32;
_11 = _7 as isize;
Call(_12 = core::intrinsics::bswap(2588036464_u32), ReturnTo(bb2), UnwindUnreachable())
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
_14 = _12 as isize;
_11 = _5 as isize;
_2 = '\u{ce87e}';
_2 = '\u{25937}';
RET = _11 & _4;
_18 = _2 as usize;
_5 = 11894_u16 as f64;
_3 = [_12,_12,_12];
_12 = (-4490397672394099329_i64) as u32;
RET = _11 >> _11;
RET = _14 | _11;
_19 = -_14;
Goto(bb11)
}
bb11 = {
_16 = [_13,_13,_13,_13,_13,_13];
_18 = 12373758801132159268_usize;
_18 = !1_usize;
_3 = [_12,_12,_12];
_18 = !14051210064845769271_usize;
_14 = _11 - _4;
_2 = '\u{103c4b}';
Goto(bb12)
}
bb12 = {
_14 = RET;
_17 = _13 | _13;
_12 = 1679849132_u32 ^ 2518526753_u32;
_5 = _7 as f64;
_7 = _1 ^ _1;
_19 = RET * RET;
RET = _19 | _19;
_9 = [_12,_12,_12];
_21 = _5 + _5;
RET = _4;
_16 = [_13,_17,_17,_17,_17,_13];
_22 = 249_u8 as isize;
_19 = 13563785318463164469_u64 as isize;
_11 = RET;
_19 = 177985591700744166570371797531539874986_u128 as isize;
_5 = _21;
_16 = [_13,_17,_17,_13,_13,_13];
_6 = _8;
_18 = _2 as usize;
_11 = -_14;
_19 = _11 << _22;
_23 = -_11;
_1 = 16884862339561940985_u64 as i8;
_8 = -_6;
_14 = _11 + _22;
RET = false as isize;
Goto(bb13)
}
bb13 = {
_20 = _5;
_14 = _11 * _19;
_20 = _5 + _5;
_3 = [_12,_12,_12];
_16 = [_17,_17,_17,_13,_17,_13];
_1 = -_7;
_4 = (-113769350530604865843671715161739807660_i128) as isize;
Goto(bb14)
}
bb14 = {
_3 = [_12,_12,_12];
_12 = 3354757841_u32 ^ 521739971_u32;
_13 = _17 ^ _17;
_24 = _8 - _6;
_21 = (-33136489835551989828013776655412655548_i128) as f64;
_19 = _14 | _14;
_12 = 249651858123232428016925126566530163884_u128 as u32;
RET = -_23;
_22 = _23;
_2 = '\u{90c88}';
_6 = _24 + _24;
_27 = !true;
_3 = _10;
_18 = !3_usize;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(2_usize, 19_usize, Move(_19), 1_usize, Move(_1), 23_usize, Move(_23), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(2_usize, 12_usize, Move(_12), 3_usize, Move(_3), 14_usize, Move(_14), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(2_usize, 13_usize, Move(_13), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [u32; 3],mut _2: char,mut _3: char,mut _4: isize) -> isize {
mir! {
type RET = isize;
let _5: u64;
let _6: Adt43;
let _7: bool;
let _8: f64;
let _9: Adt42;
let _10: isize;
let _11: [i32; 1];
let _12: [char; 3];
let _13: [u128; 5];
let _14: f64;
let _15: *mut f32;
let _16: [isize; 3];
let _17: (u128, i8, [i64; 5]);
let _18: char;
let _19: Adt42;
let _20: ();
let _21: ();
{
_3 = _2;
_2 = _3;
RET = _4;
_2 = _3;
RET = _4;
RET = _4 & _4;
_5 = !4938300949139948614_u64;
RET = _4;
_1 = [1966502328_u32,1953820286_u32,1387801801_u32];
_2 = _3;
RET = _4 << _4;
_5 = !11171955096311189849_u64;
RET = _4;
_5 = RET as u64;
_5 = !7758174804447181556_u64;
RET = _4;
_1 = [916467891_u32,1291505040_u32,2730845695_u32];
_3 = _2;
_1 = [358947047_u32,2902311845_u32,1940018703_u32];
_5 = !18120039175218166593_u64;
_4 = RET * RET;
_3 = _2;
RET = -_4;
_5 = !6277547421851350891_u64;
RET = 124728532844170141631020021601640189174_i128 as isize;
Goto(bb1)
}
bb1 = {
RET = _4;
RET = -_4;
_5 = 56514348849250953_u64 ^ 2802026532902963722_u64;
_9.fld0 = [3_usize,0_usize,0_usize,351670691323698339_usize];
_4 = 4626554287623712210_i64 as isize;
_8 = _5 as f64;
_10 = 1473642488995912294_usize as isize;
_1 = [1017407596_u32,632389097_u32,2481002477_u32];
_3 = _2;
_7 = true | false;
_5 = 2134143207342811626_u64 - 16687519383540848643_u64;
RET = _10;
_8 = 197_u8 as f64;
Goto(bb2)
}
bb2 = {
_2 = _3;
_5 = 17019205358123822765_u64;
_10 = _4;
_9.fld0 = [2329025673201474054_usize,15257137505975130305_usize,4_usize,6_usize];
match _5 {
0 => bb3,
1 => bb4,
2 => bb5,
17019205358123822765 => bb7,
_ => bb6
}
}
bb3 = {
RET = _4;
RET = -_4;
_5 = 56514348849250953_u64 ^ 2802026532902963722_u64;
_9.fld0 = [3_usize,0_usize,0_usize,351670691323698339_usize];
_4 = 4626554287623712210_i64 as isize;
_8 = _5 as f64;
_10 = 1473642488995912294_usize as isize;
_1 = [1017407596_u32,632389097_u32,2481002477_u32];
_3 = _2;
_7 = true | false;
_5 = 2134143207342811626_u64 - 16687519383540848643_u64;
RET = _10;
_8 = 197_u8 as f64;
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
_7 = true | false;
_5 = !15872472578408657575_u64;
_7 = _10 > RET;
_11 = [(-116488030_i32)];
Call(_11 = fn4(_7, _2, RET, _5, Move(_9), _4, _1, _7), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_3 = _2;
_9.fld0 = [6_usize,8339566690711088265_usize,16248191773607981212_usize,13187421058155500243_usize];
_8 = 36548591488320918984351651118000957431_u128 as f64;
_13 = [197891116384647307584783348527135880115_u128,2720488814378115127282527335263216940_u128,286759832189092636166910223353203498112_u128,145618654847778256361667728645667414479_u128,108864717686999723456739883100597276380_u128];
_8 = (-118_i8) as f64;
RET = _3 as isize;
_8 = _4 as f64;
_2 = _3;
_9.fld0 = [2_usize,13547495632676803149_usize,15630444800023936641_usize,4_usize];
_11 = [730166939_i32];
_7 = false ^ true;
_5 = 330921369916411082498508505007262999535_u128 as u64;
RET = !_10;
_16 = [_10,_10,_4];
_2 = _3;
_12 = [_3,_2,_2];
_14 = 89055086479387203895186860048825728960_i128 as f64;
_9.fld0 = [8961156070824802093_usize,7_usize,7_usize,1_usize];
_10 = !_4;
_5 = 2253147096241556412_u64;
_17.2 = [6675321311613435161_i64,680462730043974501_i64,(-1988921256229574844_i64),6913407711858388472_i64,(-6459778194490645939_i64)];
match _5 {
0 => bb3,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
2253147096241556412 => bb15,
_ => bb14
}
}
bb9 = {
_7 = true | false;
_5 = !15872472578408657575_u64;
_7 = _10 > RET;
_11 = [(-116488030_i32)];
Call(_11 = fn4(_7, _2, RET, _5, Move(_9), _4, _1, _7), ReturnTo(bb8), UnwindUnreachable())
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
RET = _4;
RET = -_4;
_5 = 56514348849250953_u64 ^ 2802026532902963722_u64;
_9.fld0 = [3_usize,0_usize,0_usize,351670691323698339_usize];
_4 = 4626554287623712210_i64 as isize;
_8 = _5 as f64;
_10 = 1473642488995912294_usize as isize;
_1 = [1017407596_u32,632389097_u32,2481002477_u32];
_3 = _2;
_7 = true | false;
_5 = 2134143207342811626_u64 - 16687519383540848643_u64;
RET = _10;
_8 = 197_u8 as f64;
Goto(bb2)
}
bb14 = {
_2 = _3;
_5 = 17019205358123822765_u64;
_10 = _4;
_9.fld0 = [2329025673201474054_usize,15257137505975130305_usize,4_usize,6_usize];
match _5 {
0 => bb3,
1 => bb4,
2 => bb5,
17019205358123822765 => bb7,
_ => bb6
}
}
bb15 = {
_17.1 = 61_i8;
_11 = [(-904027539_i32)];
_16 = [_4,_10,_4];
_12 = [_2,_3,_3];
_1 = [1592695397_u32,2571956833_u32,913968447_u32];
RET = _10;
_4 = 799824395_u32 as isize;
Goto(bb16)
}
bb16 = {
Call(_20 = dump_var(3_usize, 2_usize, Move(_2), 16_usize, Move(_16), 10_usize, Move(_10), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_20 = dump_var(3_usize, 1_usize, Move(_1), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: bool,mut _2: char,mut _3: isize,mut _4: u64,mut _5: Adt42,mut _6: isize,mut _7: [u32; 3],mut _8: bool) -> [i32; 1] {
mir! {
type RET = [i32; 1];
let _9: (u128, i8, [i64; 5]);
let _10: [u32; 3];
let _11: i32;
let _12: i64;
let _13: u64;
let _14: i8;
let _15: [i32; 1];
let _16: isize;
let _17: bool;
let _18: Adt55;
let _19: i8;
let _20: i16;
let _21: u32;
let _22: [i64; 5];
let _23: isize;
let _24: f64;
let _25: Adt49;
let _26: [u32; 3];
let _27: Adt54;
let _28: i32;
let _29: char;
let _30: [char; 3];
let _31: char;
let _32: [i64; 4];
let _33: [i32; 6];
let _34: isize;
let _35: [u32; 3];
let _36: [i32; 1];
let _37: [isize; 3];
let _38: ();
let _39: ();
{
_4 = 5511082875591408114_u64 >> _6;
_6 = (-20917_i16) as isize;
_4 = 10463923252438028666_u64;
Goto(bb1)
}
bb1 = {
_6 = _3;
_7 = [2203372315_u32,947084441_u32,2028972080_u32];
RET = [(-1536652769_i32)];
_9.1 = 8753_u16 as i8;
_9.1 = 237102788963877948930698640337071298386_u128 as i8;
_2 = '\u{8fa0d}';
_10 = [3827391855_u32,453606707_u32,2363955002_u32];
_9.0 = 154703693458181015467521389587061149215_u128;
RET = [1489213305_i32];
_11 = 24113870_i32 ^ 1422865768_i32;
_2 = '\u{edbd6}';
_9.2 = [(-2763818698313932713_i64),(-3232163257197832532_i64),2931128119346585695_i64,8592221137550452549_i64,(-764833716935600167_i64)];
_1 = _8;
_9.0 = 46800061319402490741091631482777744786_u128 + 311315244070053537325193555463135691214_u128;
_6 = _3 - _3;
_9.2 = [(-2332925335879085809_i64),(-6635997183009808313_i64),(-346594031755094732_i64),(-6260985209463612809_i64),(-1805752465541085912_i64)];
_14 = !_9.1;
_7 = [3119219622_u32,2663136861_u32,1142895693_u32];
_14 = !_9.1;
_8 = _6 <= _6;
_12 = 798809764936951326_i64;
Call(_11 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_15 = RET;
_13 = _4 & _4;
_4 = _11 as u64;
_6 = (-1070_i16) as isize;
_9.1 = _14;
_15 = RET;
_5.fld0 = [11294516460416366298_usize,3_usize,14548121503764958245_usize,3_usize];
_9.2 = [_12,_12,_12,_12,_12];
_11 = (-185662807_i32) * 2105676325_i32;
_5.fld0 = [2_usize,4_usize,13624498144158076435_usize,8065570503104253432_usize];
_9.2 = [_12,_12,_12,_12,_12];
RET = _15;
_15 = RET;
_8 = _13 <= _13;
RET = _15;
_10 = [2945989073_u32,1404699871_u32,1546449411_u32];
_3 = -_6;
RET = [_11];
_2 = '\u{d3938}';
_2 = '\u{120d7}';
_14 = _9.1;
_14 = _9.1 - _9.1;
_10 = _7;
_12 = 766506146776800353_i64 | 4701801628447531847_i64;
_2 = '\u{89dbf}';
RET = [_11];
_4 = _13 >> _9.1;
Call(_8 = fn5(Move(_5), _2, _9, _9.0, _1, _1, RET, _9.1, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = _3 | _3;
_9.2 = [_12,_12,_12,_12,_12];
_9.1 = _9.0 as i8;
_8 = !_1;
_5.fld0 = [6_usize,9846187455704781378_usize,13633592967673772068_usize,4428957398677442707_usize];
_8 = _1 | _1;
_16 = -_6;
_2 = '\u{1f983}';
_11 = (-1061038740_i32);
_6 = _16 >> _4;
RET = _15;
_14 = _9.1 * _9.1;
_13 = !_4;
_14 = _9.1;
_19 = _14;
_7 = [592172625_u32,913579729_u32,3252576213_u32];
_9.0 = 55355992921076989333898153346384669678_u128;
_19 = !_9.1;
_12 = _9.0 as i64;
_4 = _13;
_3 = -_6;
_4 = (-46713697960608419455489187997605556717_i128) as u64;
Goto(bb4)
}
bb4 = {
_3 = 2418133558_u32 as isize;
RET = [_11];
_9.2 = [_12,_12,_12,_12,_12];
_20 = (-31366_i16);
_9.1 = _19;
_8 = _1;
_13 = _4 | _4;
_15 = [_11];
match _9.0 {
0 => bb1,
1 => bb3,
55355992921076989333898153346384669678 => bb6,
_ => bb5
}
}
bb5 = {
_6 = _3;
_7 = [2203372315_u32,947084441_u32,2028972080_u32];
RET = [(-1536652769_i32)];
_9.1 = 8753_u16 as i8;
_9.1 = 237102788963877948930698640337071298386_u128 as i8;
_2 = '\u{8fa0d}';
_10 = [3827391855_u32,453606707_u32,2363955002_u32];
_9.0 = 154703693458181015467521389587061149215_u128;
RET = [1489213305_i32];
_11 = 24113870_i32 ^ 1422865768_i32;
_2 = '\u{edbd6}';
_9.2 = [(-2763818698313932713_i64),(-3232163257197832532_i64),2931128119346585695_i64,8592221137550452549_i64,(-764833716935600167_i64)];
_1 = _8;
_9.0 = 46800061319402490741091631482777744786_u128 + 311315244070053537325193555463135691214_u128;
_6 = _3 - _3;
_9.2 = [(-2332925335879085809_i64),(-6635997183009808313_i64),(-346594031755094732_i64),(-6260985209463612809_i64),(-1805752465541085912_i64)];
_14 = !_9.1;
_7 = [3119219622_u32,2663136861_u32,1142895693_u32];
_14 = !_9.1;
_8 = _6 <= _6;
_12 = 798809764936951326_i64;
Call(_11 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_7 = [1658248951_u32,1285633133_u32,1727765078_u32];
_19 = _9.1 & _14;
_14 = !_9.1;
_6 = _1 as isize;
_14 = !_9.1;
_14 = _19 - _19;
_20 = 1155_i16;
_9.2 = [_12,_12,_12,_12,_12];
_15 = [_11];
_4 = _13 << _14;
_22 = [_12,_12,_12,_12,_12];
RET = [_11];
_5.fld0 = [1_usize,4605205618809783184_usize,14422842098701239950_usize,348325791861506585_usize];
_5.fld0 = [4_usize,13071963035223415347_usize,8526105046791054387_usize,5802483268231867155_usize];
_15 = RET;
Goto(bb7)
}
bb7 = {
_2 = '\u{ea7f9}';
_20 = 25794_i16 | (-26921_i16);
_12 = !8840247732138898768_i64;
RET = [_11];
_4 = _13;
_23 = -_16;
_17 = !_1;
_27.fld2.3 = !_8;
_27.fld6 = [_12,_12,_12,_12,_12];
_16 = -_3;
RET = [_11];
RET = _15;
_27.fld1 = _9.0 as usize;
_22 = [_12,_12,_12,_12,_12];
_28 = _11 - _11;
match _9.0 {
0 => bb4,
1 => bb2,
2 => bb8,
55355992921076989333898153346384669678 => bb10,
_ => bb9
}
}
bb8 = {
_3 = 2418133558_u32 as isize;
RET = [_11];
_9.2 = [_12,_12,_12,_12,_12];
_20 = (-31366_i16);
_9.1 = _19;
_8 = _1;
_13 = _4 | _4;
_15 = [_11];
match _9.0 {
0 => bb1,
1 => bb3,
55355992921076989333898153346384669678 => bb6,
_ => bb5
}
}
bb9 = {
_6 = _3;
_7 = [2203372315_u32,947084441_u32,2028972080_u32];
RET = [(-1536652769_i32)];
_9.1 = 8753_u16 as i8;
_9.1 = 237102788963877948930698640337071298386_u128 as i8;
_2 = '\u{8fa0d}';
_10 = [3827391855_u32,453606707_u32,2363955002_u32];
_9.0 = 154703693458181015467521389587061149215_u128;
RET = [1489213305_i32];
_11 = 24113870_i32 ^ 1422865768_i32;
_2 = '\u{edbd6}';
_9.2 = [(-2763818698313932713_i64),(-3232163257197832532_i64),2931128119346585695_i64,8592221137550452549_i64,(-764833716935600167_i64)];
_1 = _8;
_9.0 = 46800061319402490741091631482777744786_u128 + 311315244070053537325193555463135691214_u128;
_6 = _3 - _3;
_9.2 = [(-2332925335879085809_i64),(-6635997183009808313_i64),(-346594031755094732_i64),(-6260985209463612809_i64),(-1805752465541085912_i64)];
_14 = !_9.1;
_7 = [3119219622_u32,2663136861_u32,1142895693_u32];
_14 = !_9.1;
_8 = _6 <= _6;
_12 = 798809764936951326_i64;
Call(_11 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_24 = (-27336315914883259977371823381218531607_i128) as f64;
_27.fld2.0 = [_9.0,_9.0,_9.0,_9.0,_9.0];
_27.fld2.1 = !_27.fld1;
_23 = -_6;
_27.fld2.0 = [_9.0,_9.0,_9.0,_9.0,_9.0];
_21 = !4239730814_u32;
_27.fld6 = [_12,_12,_12,_12,_12];
_11 = _28 | _28;
_26 = _10;
_1 = _27.fld2.3;
_9.1 = _14 * _14;
_27.fld2.1 = _27.fld1;
_27.fld0 = _1;
_19 = _9.1 + _14;
_3 = _23;
_33 = [_11,_11,_28,_28,_28,_11];
_28 = _11 >> _19;
_6 = -_23;
RET = _15;
_27.fld5 = core::ptr::addr_of_mut!(_4);
RET = [_11];
_30 = [_2,_2,_2];
Goto(bb11)
}
bb11 = {
_12 = -(-7069030760642479828_i64);
_34 = _23 << _28;
_22 = [_12,_12,_12,_12,_12];
_11 = _2 as i32;
_5.fld0 = [_27.fld2.1,_27.fld2.1,_27.fld2.1,_27.fld2.1];
_9.2 = [_12,_12,_12,_12,_12];
_33 = [_28,_28,_28,_28,_28,_28];
_9 = (146153667965096906669998766260115769108_u128, _19, _27.fld6);
_26 = [_21,_21,_21];
_36 = [_28];
_27.fld5 = core::ptr::addr_of_mut!(_4);
_4 = _13;
_14 = _9.1;
_29 = _2;
_11 = -_28;
RET = [_11];
_4 = !_13;
_9.0 = 124399571906517449113368568943744215413_u128 * 220608629079611234394448882141905228756_u128;
_27.fld0 = !_17;
Goto(bb12)
}
bb12 = {
Call(_38 = dump_var(4_usize, 16_usize, Move(_16), 3_usize, Move(_3), 6_usize, Move(_6), 1_usize, Move(_1)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_38 = dump_var(4_usize, 33_usize, Move(_33), 28_usize, Move(_28), 11_usize, Move(_11), 15_usize, Move(_15)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_38 = dump_var(4_usize, 17_usize, Move(_17), 30_usize, Move(_30), 12_usize, Move(_12), 26_usize, Move(_26)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_38 = dump_var(4_usize, 22_usize, Move(_22), 20_usize, Move(_20), 39_usize, _39, 39_usize, _39), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: Adt42,mut _2: char,mut _3: (u128, i8, [i64; 5]),mut _4: u128,mut _5: bool,mut _6: bool,mut _7: [i32; 1],mut _8: i8,mut _9: [u32; 3]) -> bool {
mir! {
type RET = bool;
let _10: [i32; 6];
let _11: Adt42;
let _12: isize;
let _13: f32;
let _14: i16;
let _15: *mut ([i32; 1], u16, usize, u32, f32, *mut f32);
let _16: Adt54;
let _17: [isize; 4];
let _18: u64;
let _19: isize;
let _20: [usize; 4];
let _21: f32;
let _22: isize;
let _23: u8;
let _24: isize;
let _25: Adt56;
let _26: (*mut u64,);
let _27: [u128; 5];
let _28: u64;
let _29: f32;
let _30: u16;
let _31: Adt42;
let _32: *mut [u128; 5];
let _33: i8;
let _34: [usize; 4];
let _35: [i64; 5];
let _36: [u32; 3];
let _37: [u128; 5];
let _38: Adt42;
let _39: char;
let _40: [u128; 5];
let _41: u128;
let _42: [usize; 4];
let _43: usize;
let _44: [i32; 6];
let _45: ();
let _46: ();
{
RET = !_5;
RET = _6;
_3.0 = !_4;
_2 = '\u{c3b73}';
RET = !_5;
_6 = _5;
_3.1 = 748164849208199709_i64 as i8;
RET = _6;
RET = _6;
RET = _3.0 != _4;
_3.1 = !_8;
Goto(bb1)
}
bb1 = {
_4 = _3.0;
Call(_8 = core::intrinsics::transmute(_3.1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = RET;
_4 = _3.0;
RET = _5;
_7 = [66185131_i32];
_2 = '\u{6f684}';
RET = !_5;
RET = _5;
_3.1 = _8 - _8;
_5 = RET;
_7 = [(-2132920207_i32)];
_10 = [(-1260938770_i32),231541792_i32,831878229_i32,2016493737_i32,431110688_i32,(-940467097_i32)];
_3.2 = [(-6038925837222931459_i64),(-3036475131584975420_i64),7404686645058778377_i64,3954317598405309108_i64,8729201083722331628_i64];
Goto(bb3)
}
bb3 = {
_7 = [(-2146907388_i32)];
_4 = _3.0 * _3.0;
_10 = [(-1544280900_i32),(-1980443449_i32),164383872_i32,1154669631_i32,(-1761587843_i32),(-628826773_i32)];
_11 = Adt42 { fld0: _1.fld0 };
_11.fld0 = [14978743020803223872_usize,2_usize,10565355630191924707_usize,3_usize];
_1.fld0 = [907677436736052230_usize,7_usize,3_usize,3_usize];
_5 = _6;
_11.fld0 = _1.fld0;
_9 = [3881645231_u32,3925885880_u32,4289199000_u32];
_5 = !RET;
_14 = 9350_i16 & (-22479_i16);
_9 = [312708963_u32,1601340333_u32,2611696622_u32];
_3.2 = [(-7676566383939439274_i64),7608657669847316006_i64,8525434784005948269_i64,(-9075602520376900664_i64),1614848717845523112_i64];
_6 = !_5;
_13 = _4 as f32;
_1.fld0 = [1_usize,16640140678164526981_usize,5_usize,3_usize];
_10 = [1470932372_i32,(-1220708101_i32),819983086_i32,(-1191740436_i32),166747902_i32,601139256_i32];
_2 = '\u{170ab}';
_1.fld0 = _11.fld0;
_3.2 = [(-4894980388190340966_i64),4411664779362836492_i64,4688230321896621291_i64,6006111510175618957_i64,(-6611794151793256913_i64)];
_11.fld0 = [2_usize,5285610134247420988_usize,18152719054762990347_usize,1_usize];
_4 = 1271578002498329689_i64 as u128;
_16.fld2.4 = core::ptr::addr_of_mut!(_16.fld2.0);
_7 = [(-1267866261_i32)];
Goto(bb4)
}
bb4 = {
_9 = [2082133493_u32,2092922624_u32,3551770511_u32];
_3.1 = 5425296148887177014_usize as i8;
_3.0 = (-25542331746080474470376409433258489300_i128) as u128;
_16.fld1 = 5_usize ^ 3_usize;
_16.fld6 = [1908785896613361460_i64,6322127759675097990_i64,6423776921122019946_i64,(-3471956438293073568_i64),1827488856104621184_i64];
_17 = [(-9223372036854775808_isize),9223372036854775807_isize,(-28_isize),26_isize];
_1.fld0 = [_16.fld1,_16.fld1,_16.fld1,_16.fld1];
_16.fld2.3 = !RET;
_16.fld2.1 = _16.fld1;
_1 = Adt42 { fld0: _11.fld0 };
_8 = -_3.1;
_16.fld2.3 = !RET;
_16.fld2.1 = !_16.fld1;
_8 = _3.1 << _16.fld1;
_3.0 = !_4;
_3.2 = [(-7511871836155533562_i64),(-7332594218868062039_i64),6560236152169018263_i64,4776822272181127848_i64,(-7511190839226223786_i64)];
_17 = [(-127_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_16.fld2.0 = [_4,_4,_4,_4,_4];
_3.0 = _4;
_16.fld0 = !_6;
Goto(bb5)
}
bb5 = {
_16.fld3 = [_2,_2,_2];
_16.fld2.4 = core::ptr::addr_of_mut!(_16.fld2.0);
RET = _6;
_4 = !_3.0;
_5 = !_16.fld2.3;
_16.fld0 = !_5;
_16.fld1 = _16.fld2.1;
_6 = _16.fld0;
_19 = 63_u8 as isize;
_16.fld0 = !_6;
_11.fld0 = _1.fld0;
_16.fld2.3 = !_6;
_3.0 = _4 | _4;
RET = _5 | _16.fld0;
_16.fld2.2 = core::ptr::addr_of_mut!(_13);
_8 = _3.1 << _3.0;
_4 = _19 as u128;
_9 = [4027270069_u32,1726249300_u32,1932848264_u32];
Call(_16.fld2.2 = fn6(_16.fld0, Move(_11), _17, _9, _5), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
RET = _16.fld2.3;
_11 = Move(_1);
_18 = !469122571994884159_u64;
RET = !_16.fld2.3;
_16.fld6 = _3.2;
_16.fld2.1 = _2 as usize;
_8 = _3.1 - _3.1;
_16.fld5 = core::ptr::addr_of_mut!(_18);
Goto(bb7)
}
bb7 = {
_12 = _2 as isize;
_16.fld2.3 = _5;
_1.fld0 = [_16.fld2.1,_16.fld2.1,_16.fld1,_16.fld2.1];
_24 = 95_u8 as isize;
_2 = '\u{be6bf}';
_16.fld2.4 = core::ptr::addr_of_mut!(_27);
_24 = -_19;
_16.fld5 = core::ptr::addr_of_mut!(_28);
_24 = _19;
_27 = [_4,_4,_4,_3.0,_4];
Call(_3.1 = core::intrinsics::bswap(_8), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_1 = Adt42 { fld0: _11.fld0 };
_13 = (-2128383358517807700_i64) as f32;
_16.fld0 = _5 != _5;
_16.fld5 = core::ptr::addr_of_mut!(_28);
_2 = '\u{139ff}';
_9 = [3528002482_u32,475817035_u32,1848858816_u32];
_29 = _13;
_23 = 106_u8 & 182_u8;
_11 = Adt42 { fld0: _1.fld0 };
_14 = (-2553_i16);
_16.fld0 = _8 == _3.1;
_19 = _4 as isize;
RET = _23 <= _23;
_13 = 58955_u16 as f32;
_16.fld2.3 = _6 ^ _5;
_11.fld0 = [_16.fld2.1,_16.fld1,_16.fld1,_16.fld2.1];
_5 = _19 == _19;
_24 = _12 * _12;
_16.fld2.0 = [_3.0,_4,_3.0,_3.0,_3.0];
_8 = _3.1;
Goto(bb9)
}
bb9 = {
_14 = -(-12262_i16);
_16.fld2.1 = _16.fld1;
_27 = [_4,_3.0,_4,_3.0,_4];
_10 = [2081293344_i32,1217928911_i32,(-1248693235_i32),1117022916_i32,2019693140_i32,(-1889199316_i32)];
_34 = [_16.fld1,_16.fld1,_16.fld1,_16.fld2.1];
_9 = [1318139416_u32,2686100114_u32,1073900480_u32];
_11 = Move(_1);
_10 = [1993155474_i32,1608531493_i32,1014717859_i32,(-815582848_i32),747775858_i32,(-356670582_i32)];
_31.fld0 = [_16.fld1,_16.fld2.1,_16.fld2.1,_16.fld1];
_26.0 = core::ptr::addr_of_mut!(_28);
Goto(bb10)
}
bb10 = {
_35 = _3.2;
_28 = !_18;
_5 = _6;
_6 = !RET;
_29 = 906660564131070669_i64 as f32;
_28 = _18 - _18;
_33 = 10764_u16 as i8;
_16.fld0 = !_5;
_36 = _9;
_17 = [_24,_19,_12,_19];
_1 = Adt42 { fld0: _34 };
_32 = core::ptr::addr_of_mut!(_37);
_28 = _18;
(*_32) = [_4,_3.0,_4,_3.0,_4];
_16.fld3 = [_2,_2,_2];
_30 = 50458_u16;
_17 = [_12,_12,_19,_12];
_38.fld0 = _34;
_8 = _3.1;
_22 = _16.fld1 as isize;
_38.fld0 = _31.fld0;
_38.fld0 = [_16.fld2.1,_16.fld1,_16.fld2.1,_16.fld1];
_21 = _14 as f32;
_16.fld2.1 = _16.fld1;
_2 = '\u{9bf42}';
_20 = [_16.fld1,_16.fld2.1,_16.fld2.1,_16.fld1];
_23 = !88_u8;
_31.fld0 = [_16.fld1,_16.fld2.1,_16.fld1,_16.fld1];
match _30 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb11,
4 => bb12,
50458 => bb14,
_ => bb13
}
}
bb11 = {
_16.fld3 = [_2,_2,_2];
_16.fld2.4 = core::ptr::addr_of_mut!(_16.fld2.0);
RET = _6;
_4 = !_3.0;
_5 = !_16.fld2.3;
_16.fld0 = !_5;
_16.fld1 = _16.fld2.1;
_6 = _16.fld0;
_19 = 63_u8 as isize;
_16.fld0 = !_6;
_11.fld0 = _1.fld0;
_16.fld2.3 = !_6;
_3.0 = _4 | _4;
RET = _5 | _16.fld0;
_16.fld2.2 = core::ptr::addr_of_mut!(_13);
_8 = _3.1 << _3.0;
_4 = _19 as u128;
_9 = [4027270069_u32,1726249300_u32,1932848264_u32];
Call(_16.fld2.2 = fn6(_16.fld0, Move(_11), _17, _9, _5), ReturnTo(bb6), UnwindUnreachable())
}
bb12 = {
_5 = RET;
_4 = _3.0;
RET = _5;
_7 = [66185131_i32];
_2 = '\u{6f684}';
RET = !_5;
RET = _5;
_3.1 = _8 - _8;
_5 = RET;
_7 = [(-2132920207_i32)];
_10 = [(-1260938770_i32),231541792_i32,831878229_i32,2016493737_i32,431110688_i32,(-940467097_i32)];
_3.2 = [(-6038925837222931459_i64),(-3036475131584975420_i64),7404686645058778377_i64,3954317598405309108_i64,8729201083722331628_i64];
Goto(bb3)
}
bb13 = {
_12 = _2 as isize;
_16.fld2.3 = _5;
_1.fld0 = [_16.fld2.1,_16.fld2.1,_16.fld1,_16.fld2.1];
_24 = 95_u8 as isize;
_2 = '\u{be6bf}';
_16.fld2.4 = core::ptr::addr_of_mut!(_27);
_24 = -_19;
_16.fld5 = core::ptr::addr_of_mut!(_28);
_24 = _19;
_27 = [_4,_4,_4,_3.0,_4];
Call(_3.1 = core::intrinsics::bswap(_8), ReturnTo(bb8), UnwindUnreachable())
}
bb14 = {
_25 = Adt56::Variant0 { fld0: _16.fld2.2,fld1: _17 };
place!(Field::<[isize; 4]>(Variant(_25, 0), 1)) = [_24,_24,_12,_12];
_31.fld0 = [_16.fld2.1,_16.fld1,_16.fld2.1,_16.fld1];
_26.0 = core::ptr::addr_of_mut!(_18);
_43 = !_16.fld2.1;
_41 = _3.0;
_34 = _20;
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(5_usize, 12_usize, Move(_12), 5_usize, Move(_5), 35_usize, Move(_35), 36_usize, Move(_36)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(5_usize, 23_usize, Move(_23), 17_usize, Move(_17), 7_usize, Move(_7), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(5_usize, 20_usize, Move(_20), 4_usize, Move(_4), 9_usize, Move(_9), 33_usize, Move(_33)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(5_usize, 10_usize, Move(_10), 34_usize, Move(_34), 46_usize, _46, 46_usize, _46), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: bool,mut _2: Adt42,mut _3: [isize; 4],mut _4: [u32; 3],mut _5: bool) -> *mut f32 {
mir! {
type RET = *mut f32;
let _6: (u128, i8, [i64; 5]);
let _7: Adt46;
let _8: f64;
let _9: u32;
let _10: i8;
let _11: isize;
let _12: *mut ([i32; 1], u16, usize, u32, f32, *mut f32);
let _13: Adt42;
let _14: f64;
let _15: [isize; 3];
let _16: [u128; 2];
let _17: char;
let _18: isize;
let _19: [u32; 3];
let _20: Adt57;
let _21: *mut f32;
let _22: Adt43;
let _23: bool;
let _24: Adt55;
let _25: isize;
let _26: i8;
let _27: Adt56;
let _28: Adt56;
let _29: usize;
let _30: [u128; 2];
let _31: Adt54;
let _32: [i32; 6];
let _33: ();
let _34: ();
{
_2.fld0 = [0_usize,14715025577938699842_usize,3003822154090614784_usize,7912173361626817845_usize];
_2.fld0 = [0_usize,0_usize,835234790959034107_usize,0_usize];
_6.0 = 56459354029943665764182835476821219076_u128;
_1 = !_5;
_6.2 = [2568586848189060705_i64,8289683072355821386_i64,2189061298830278649_i64,(-8282307904340053241_i64),(-541921135650692762_i64)];
_6.1 = _6.0 as i8;
Goto(bb1)
}
bb1 = {
_2.fld0 = [7_usize,6_usize,5399665231141528955_usize,3_usize];
_1 = _5;
_6.2 = [8875614516424624043_i64,4749856979494166752_i64,(-3043126302230322238_i64),(-2360929908704420736_i64),(-614979473419383836_i64)];
_6.2 = [(-7409268236153419413_i64),(-9027357371686912938_i64),(-1008194445742089764_i64),7190139857781921551_i64,(-7245020162374345442_i64)];
_5 = _1;
_1 = _6.1 < _6.1;
_1 = _6.1 <= _6.1;
_6.0 = 119864815090317354544568290152005094937_u128;
_3 = [9223372036854775807_isize,(-9223372036854775808_isize),(-39_isize),9223372036854775807_isize];
_6.0 = !93995957337529511635964132211343121091_u128;
_4 = [2100242186_u32,3827486957_u32,4264354733_u32];
_5 = _1;
_2.fld0 = [11570136896392837038_usize,0_usize,4_usize,7_usize];
_4 = [1696922118_u32,1238010261_u32,853446708_u32];
_4 = [710449441_u32,2897009852_u32,3954561717_u32];
_9 = '\u{161e6}' as u32;
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_6.1 = 7335242187633105652_i64 as i8;
_6.1 = 169_u8 as i8;
_2.fld0 = [0_usize,5053914248506891807_usize,5959415284522060411_usize,7_usize];
_10 = _6.1 >> _6.0;
Goto(bb2)
}
bb2 = {
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-67_isize)];
_2.fld0 = [6_usize,16331568081430554547_usize,0_usize,6_usize];
_4 = [_9,_9,_9];
_11 = 22_isize | 9223372036854775807_isize;
_10 = 176_u8 as i8;
_5 = _1;
_6.1 = 57570455053974422307387207282765673589_i128 as i8;
_4 = [_9,_9,_9];
_13.fld0 = _2.fld0;
_10 = _6.1 ^ _6.1;
_11 = (-9223372036854775808_isize);
Goto(bb3)
}
bb3 = {
_11 = 9223372036854775807_isize >> _6.0;
_10 = _6.1;
_15 = [_11,_11,_11];
_3 = [_11,_11,_11,_11];
_6.0 = 97423433399407057682599910018889867651_u128 * 300203073607628145436313537094319699226_u128;
_15 = [_11,_11,_11];
_15 = [_11,_11,_11];
_8 = _11 as f64;
_3 = [_11,_11,_11,_11];
_10 = _6.1;
_6.2 = [(-5718936969118812819_i64),2607537953664282464_i64,(-6359330830331147477_i64),4250582968759339045_i64,(-876382454263802540_i64)];
_16 = [_6.0,_6.0];
_14 = _8 + _8;
_2.fld0 = [562167793527067096_usize,6_usize,8995605285081874351_usize,0_usize];
_9 = !3851206210_u32;
_6.2 = [(-1810442470664225623_i64),(-4517021848374379697_i64),6934073416431038950_i64,7839845431320719813_i64,(-3138423415227726646_i64)];
_17 = '\u{10872f}';
_6.2 = [(-1308350406940349761_i64),(-971304220421774660_i64),(-5961991419069569972_i64),6330722871968688728_i64,(-3052530257403201042_i64)];
_16 = [_6.0,_6.0];
_9 = 2911114518_u32;
_1 = _6.1 != _10;
_6.1 = _10 | _10;
_13.fld0 = [3_usize,4_usize,5649479243792395680_usize,12106726814253794909_usize];
_10 = _6.1;
_6.0 = !259126132225872965774233315080022760903_u128;
_2.fld0 = [17985224356905074180_usize,4537156258898849913_usize,5_usize,0_usize];
Call(RET = fn7(_8, _2.fld0, _4), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_13.fld0 = _2.fld0;
_5 = _1;
_1 = _5 <= _5;
_6.0 = 7492104948652170457667682650096825840_u128;
_15 = [_11,_11,_11];
Call(_2.fld0 = fn19(_11, _6.2, _6.2, _15, _3, _15, _3, _5), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_19 = [_9,_9,_9];
_1 = !_5;
_4 = [_9,_9,_9];
_2 = Move(_13);
_19 = [_9,_9,_9];
_6.2 = [5453231813789434179_i64,(-4068253368761289290_i64),(-4198701400488449534_i64),(-482309174405436362_i64),2703975518241472186_i64];
_6.0 = !168167141971498563987219165546232603487_u128;
_6.1 = -_10;
_19 = _4;
_6.0 = !112701826937283277499420142850159526479_u128;
_3 = [_11,_11,_11,_11];
_13.fld0 = [1_usize,0_usize,4288336030532088432_usize,13434595343691040618_usize];
_3 = [_11,_11,_11,_11];
_16 = [_6.0,_6.0];
_14 = _8;
_6.2 = [(-2444335849257283363_i64),8310964460631363301_i64,170881176745549293_i64,(-532800804284066891_i64),(-382885810434720689_i64)];
_16 = [_6.0,_6.0];
_15 = [_11,_11,_11];
_1 = _8 != _14;
match _9 {
0 => bb6,
1 => bb7,
2 => bb8,
2911114518 => bb10,
_ => bb9
}
}
bb6 = {
_13.fld0 = _2.fld0;
_5 = _1;
_1 = _5 <= _5;
_6.0 = 7492104948652170457667682650096825840_u128;
_15 = [_11,_11,_11];
Call(_2.fld0 = fn19(_11, _6.2, _6.2, _15, _3, _15, _3, _5), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_11 = 9223372036854775807_isize >> _6.0;
_10 = _6.1;
_15 = [_11,_11,_11];
_3 = [_11,_11,_11,_11];
_6.0 = 97423433399407057682599910018889867651_u128 * 300203073607628145436313537094319699226_u128;
_15 = [_11,_11,_11];
_15 = [_11,_11,_11];
_8 = _11 as f64;
_3 = [_11,_11,_11,_11];
_10 = _6.1;
_6.2 = [(-5718936969118812819_i64),2607537953664282464_i64,(-6359330830331147477_i64),4250582968759339045_i64,(-876382454263802540_i64)];
_16 = [_6.0,_6.0];
_14 = _8 + _8;
_2.fld0 = [562167793527067096_usize,6_usize,8995605285081874351_usize,0_usize];
_9 = !3851206210_u32;
_6.2 = [(-1810442470664225623_i64),(-4517021848374379697_i64),6934073416431038950_i64,7839845431320719813_i64,(-3138423415227726646_i64)];
_17 = '\u{10872f}';
_6.2 = [(-1308350406940349761_i64),(-971304220421774660_i64),(-5961991419069569972_i64),6330722871968688728_i64,(-3052530257403201042_i64)];
_16 = [_6.0,_6.0];
_9 = 2911114518_u32;
_1 = _6.1 != _10;
_6.1 = _10 | _10;
_13.fld0 = [3_usize,4_usize,5649479243792395680_usize,12106726814253794909_usize];
_10 = _6.1;
_6.0 = !259126132225872965774233315080022760903_u128;
_2.fld0 = [17985224356905074180_usize,4537156258898849913_usize,5_usize,0_usize];
Call(RET = fn7(_8, _2.fld0, _4), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-67_isize)];
_2.fld0 = [6_usize,16331568081430554547_usize,0_usize,6_usize];
_4 = [_9,_9,_9];
_11 = 22_isize | 9223372036854775807_isize;
_10 = 176_u8 as i8;
_5 = _1;
_6.1 = 57570455053974422307387207282765673589_i128 as i8;
_4 = [_9,_9,_9];
_13.fld0 = _2.fld0;
_10 = _6.1 ^ _6.1;
_11 = (-9223372036854775808_isize);
Goto(bb3)
}
bb9 = {
_2.fld0 = [7_usize,6_usize,5399665231141528955_usize,3_usize];
_1 = _5;
_6.2 = [8875614516424624043_i64,4749856979494166752_i64,(-3043126302230322238_i64),(-2360929908704420736_i64),(-614979473419383836_i64)];
_6.2 = [(-7409268236153419413_i64),(-9027357371686912938_i64),(-1008194445742089764_i64),7190139857781921551_i64,(-7245020162374345442_i64)];
_5 = _1;
_1 = _6.1 < _6.1;
_1 = _6.1 <= _6.1;
_6.0 = 119864815090317354544568290152005094937_u128;
_3 = [9223372036854775807_isize,(-9223372036854775808_isize),(-39_isize),9223372036854775807_isize];
_6.0 = !93995957337529511635964132211343121091_u128;
_4 = [2100242186_u32,3827486957_u32,4264354733_u32];
_5 = _1;
_2.fld0 = [11570136896392837038_usize,0_usize,4_usize,7_usize];
_4 = [1696922118_u32,1238010261_u32,853446708_u32];
_4 = [710449441_u32,2897009852_u32,3954561717_u32];
_9 = '\u{161e6}' as u32;
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_6.1 = 7335242187633105652_i64 as i8;
_6.1 = 169_u8 as i8;
_2.fld0 = [0_usize,5053914248506891807_usize,5959415284522060411_usize,7_usize];
_10 = _6.1 >> _6.0;
Goto(bb2)
}
bb10 = {
_6.0 = 223325013404765353080401576880869747954_u128 >> _9;
_13 = Move(_2);
_18 = !_11;
_4 = _19;
_10 = _6.1;
_2.fld0 = _13.fld0;
_19 = [_9,_9,_9];
_6.2 = [(-5134759861962862272_i64),(-7205358225820219013_i64),7339175681457137465_i64,(-7137097555822937732_i64),(-2512698943982827897_i64)];
_15 = [_18,_11,_11];
_14 = _8 - _8;
_8 = 24415_u16 as f64;
_17 = '\u{e699b}';
_11 = _18 | _18;
_14 = _8 - _8;
_21 = RET;
_4 = [_9,_9,_9];
_8 = 21299_i16 as f64;
Goto(bb11)
}
bb11 = {
_13 = Adt42 { fld0: _2.fld0 };
_25 = _11;
_1 = _5 | _5;
_6.0 = 63099594232744533693619421298018351646_u128 - 179654637057928404561168787032898929984_u128;
_10 = _6.1;
_26 = _6.1;
_13.fld0 = [4_usize,17791852500409254108_usize,0_usize,6_usize];
_3 = [_25,_25,_11,_11];
_18 = _25;
_6.0 = 119359058788346150090013959430246709599_u128;
_17 = '\u{8081f}';
_1 = !_5;
_13.fld0 = [6457772806558552899_usize,3_usize,5_usize,18126738320177685518_usize];
_4 = _19;
_19 = _4;
_23 = _18 != _18;
_6.0 = 225972161502355960574383689003890019651_u128;
_2.fld0 = [4_usize,2_usize,7_usize,12727826067995605301_usize];
_13.fld0 = _2.fld0;
_11 = _25 | _25;
_4 = _19;
_8 = _6.0 as f64;
_15 = [_25,_18,_25];
match _9 {
2911114518 => bb13,
_ => bb12
}
}
bb12 = {
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-67_isize)];
_2.fld0 = [6_usize,16331568081430554547_usize,0_usize,6_usize];
_4 = [_9,_9,_9];
_11 = 22_isize | 9223372036854775807_isize;
_10 = 176_u8 as i8;
_5 = _1;
_6.1 = 57570455053974422307387207282765673589_i128 as i8;
_4 = [_9,_9,_9];
_13.fld0 = _2.fld0;
_10 = _6.1 ^ _6.1;
_11 = (-9223372036854775808_isize);
Goto(bb3)
}
bb13 = {
_26 = _10;
_14 = _8 + _8;
_16 = [_6.0,_6.0];
_9 = 4085505610_u32;
_29 = 34303_u16 as usize;
RET = _21;
_8 = 50058_u16 as f64;
_28 = Adt56::Variant0 { fld0: RET,fld1: _3 };
_3 = Field::<[isize; 4]>(Variant(_28, 0), 1);
_2.fld0 = _13.fld0;
SetDiscriminant(_28, 1);
_30 = [_6.0,_6.0];
place!(Field::<[i32; 6]>(Variant(_28, 1), 5)) = [(-1211667870_i32),(-907271545_i32),(-694898079_i32),(-1459326016_i32),1037202572_i32,1895793703_i32];
_2 = Move(_13);
Goto(bb14)
}
bb14 = {
_8 = _14;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(6_usize, 10_usize, Move(_10), 25_usize, Move(_25), 4_usize, Move(_4), 30_usize, Move(_30)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(6_usize, 17_usize, Move(_17), 26_usize, Move(_26), 11_usize, Move(_11), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(6_usize, 5_usize, Move(_5), 34_usize, _34, 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: f64,mut _2: [usize; 4],mut _3: [u32; 3]) -> *mut f32 {
mir! {
type RET = *mut f32;
let _4: f32;
let _5: [u128; 2];
let _6: bool;
let _7: [i32; 6];
let _8: u128;
let _9: char;
let _10: (u128, i8, [i64; 5]);
let _11: [usize; 4];
let _12: bool;
let _13: f64;
let _14: [i64; 4];
let _15: Adt42;
let _16: (u128, i8, [i64; 5]);
let _17: [char; 3];
let _18: Adt53;
let _19: bool;
let _20: char;
let _21: (*mut u64,);
let _22: [u128; 5];
let _23: [usize; 6];
let _24: char;
let _25: u8;
let _26: [char; 3];
let _27: u8;
let _28: isize;
let _29: [i64; 5];
let _30: f32;
let _31: Adt47;
let _32: Adt45;
let _33: ();
let _34: ();
{
_1 = 106_isize as f64;
_1 = 61749_u16 as f64;
_2 = [5_usize,576506561487215131_usize,6299162276597616331_usize,4_usize];
_3 = [2289721125_u32,785288148_u32,3340878644_u32];
_3 = [2772051320_u32,4041003092_u32,1300410374_u32];
_1 = 27229_i16 as f64;
_1 = 5966965711503277592_u64 as f64;
RET = core::ptr::addr_of_mut!(_4);
(*RET) = 19914_u16 as f32;
(*RET) = 15800895413354312719_u64 as f32;
(*RET) = (-1219683468_i32) as f32;
_1 = 23063_u16 as f64;
_3 = [892120649_u32,2480197525_u32,3145461544_u32];
(*RET) = 4157766852_u32 as f32;
_3 = [4048474675_u32,1035962088_u32,3156018737_u32];
_4 = 18470197681376566411707034260442055516_i128 as f32;
_2 = [4_usize,10882522583805724050_usize,13979290941030910073_usize,9947661875606035773_usize];
_4 = 1489935650472424161_i64 as f32;
RET = core::ptr::addr_of_mut!(_4);
(*RET) = 4581728109474570367_u64 as f32;
_5 = [4618275074585142376192061763533161330_u128,191843975765692981388412904808328929509_u128];
Goto(bb1)
}
bb1 = {
RET = core::ptr::addr_of_mut!(_4);
(*RET) = 14250177474949330619_usize as f32;
_5 = [164688839592175071083036616118501680738_u128,169638401620097506753361201033941314836_u128];
_7 = [(-246289479_i32),(-1033617293_i32),282370312_i32,(-120325924_i32),245043593_i32,(-607643749_i32)];
_7 = [552267354_i32,(-980986420_i32),37340691_i32,405912650_i32,604438451_i32,1814274610_i32];
_4 = 15259_i16 as f32;
_5 = [108685374897263501170777223260720623335_u128,124755264092227967649798882095432102405_u128];
_6 = (*RET) == (*RET);
_4 = 136799531667793330954365138525193632548_i128 as f32;
_2 = [5_usize,1_usize,8184854422273937402_usize,10900929281585090265_usize];
_3 = [2591031248_u32,687804573_u32,3751243072_u32];
Goto(bb2)
}
bb2 = {
_7 = [466597618_i32,537882155_i32,341724933_i32,839611371_i32,(-1782658163_i32),1195394604_i32];
(*RET) = 26_i8 as f32;
RET = core::ptr::addr_of_mut!((*RET));
_4 = 3089352397_u32 as f32;
_6 = !true;
(*RET) = _1 as f32;
_8 = 245908290850783888937236766411218947906_u128 - 81307092604853274912377270943098733813_u128;
(*RET) = 165922427567718499181904175456928971356_i128 as f32;
_5 = [_8,_8];
_2 = [4_usize,7_usize,10851898347197774637_usize,573934381531020699_usize];
_9 = '\u{be03}';
_1 = 8374002260914476196_u64 as f64;
_10.2 = [379841491669020965_i64,2793918824045144150_i64,(-5498019453352671321_i64),(-1094046636931741962_i64),4393648628926869862_i64];
_5 = [_8,_8];
(*RET) = _1 as f32;
_2 = [2526323242139173849_usize,2_usize,14296843270845729125_usize,7421352950355636894_usize];
_2 = [15277055424922010332_usize,1689524621044322280_usize,0_usize,0_usize];
_9 = '\u{9f390}';
_10.0 = _8 + _8;
_4 = (-67_i8) as f32;
_7 = [(-264865444_i32),(-1505826124_i32),(-1935181118_i32),(-859984965_i32),(-185675782_i32),(-621534391_i32)];
RET = core::ptr::addr_of_mut!((*RET));
_11 = [4756724744713897230_usize,0_usize,4_usize,3_usize];
_2 = [10619271655132029513_usize,14257079540698204933_usize,12108960111574442835_usize,4470637821056179471_usize];
Goto(bb3)
}
bb3 = {
_1 = (-109846783_i32) as f64;
_6 = !false;
_1 = 7695969878610904003_u64 as f64;
(*RET) = 16_i8 as f32;
(*RET) = 1_usize as f32;
_10.1 = 26_i8 - 86_i8;
_5 = [_8,_10.0];
_8 = _10.0 ^ _10.0;
(*RET) = _1 as f32;
_10.0 = 143880556_u32 as u128;
Goto(bb4)
}
bb4 = {
(*RET) = 1884619138_i32 as f32;
_10.2 = [(-1184093492654611765_i64),896493364456080412_i64,(-1935201372497085254_i64),2030423940831131248_i64,(-1677793851649617172_i64)];
_10.2 = [(-6834840824273947145_i64),(-2777561779073973655_i64),4155470732271083299_i64,2594278434427569664_i64,180108436376295196_i64];
_5 = [_8,_8];
(*RET) = 3203464827488579018_u64 as f32;
_14 = [(-5034199174207678308_i64),6119201383213696889_i64,(-2890504659724918273_i64),(-7413418359250809068_i64)];
_4 = (-7293202782338051686_i64) as f32;
_12 = _6 & _6;
RET = core::ptr::addr_of_mut!((*RET));
_15 = Adt42 { fld0: _11 };
Call(RET = fn8(_14, _5, _10, _7, _7, _2, _10.2, _9, _14, _10, (*RET), _10, _5), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_11 = [2_usize,8413358538470076633_usize,3_usize,3_usize];
_10.1 = 67_i8;
_3 = [914814683_u32,4010123279_u32,2725789212_u32];
_15.fld0 = [0_usize,12020052869377015545_usize,10747778968979018745_usize,7_usize];
_9 = '\u{12ae6}';
_5 = [_8,_8];
_5 = [_8,_8];
_5 = [_10.0,_8];
_14 = [(-2859149958170386065_i64),515876869476909458_i64,1766636259512795561_i64,6583072567538309950_i64];
_8 = _1 as u128;
_1 = 217_u8 as f64;
_7 = [(-556092714_i32),(-1102420905_i32),273956750_i32,(-911656493_i32),1554378965_i32,(-1594360205_i32)];
_1 = 6052_i16 as f64;
_15.fld0 = [4_usize,4_usize,3_usize,8629533741052781052_usize];
RET = core::ptr::addr_of_mut!(_4);
_16.1 = _9 as i8;
match _10.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb6,
67 => bb8,
_ => bb7
}
}
bb6 = {
RET = core::ptr::addr_of_mut!(_4);
(*RET) = 14250177474949330619_usize as f32;
_5 = [164688839592175071083036616118501680738_u128,169638401620097506753361201033941314836_u128];
_7 = [(-246289479_i32),(-1033617293_i32),282370312_i32,(-120325924_i32),245043593_i32,(-607643749_i32)];
_7 = [552267354_i32,(-980986420_i32),37340691_i32,405912650_i32,604438451_i32,1814274610_i32];
_4 = 15259_i16 as f32;
_5 = [108685374897263501170777223260720623335_u128,124755264092227967649798882095432102405_u128];
_6 = (*RET) == (*RET);
_4 = 136799531667793330954365138525193632548_i128 as f32;
_2 = [5_usize,1_usize,8184854422273937402_usize,10900929281585090265_usize];
_3 = [2591031248_u32,687804573_u32,3751243072_u32];
Goto(bb2)
}
bb7 = {
_1 = (-109846783_i32) as f64;
_6 = !false;
_1 = 7695969878610904003_u64 as f64;
(*RET) = 16_i8 as f32;
(*RET) = 1_usize as f32;
_10.1 = 26_i8 - 86_i8;
_5 = [_8,_10.0];
_8 = _10.0 ^ _10.0;
(*RET) = _1 as f32;
_10.0 = 143880556_u32 as u128;
Goto(bb4)
}
bb8 = {
_16.0 = _9 as u128;
_3 = [3164171985_u32,1574067792_u32,3270527097_u32];
_2 = [9923052289328604384_usize,7_usize,1_usize,1741148081975462122_usize];
_10.1 = !_16.1;
_8 = _16.0;
_17 = [_9,_9,_9];
_17 = [_9,_9,_9];
_15.fld0 = [3101191023885799240_usize,0_usize,2_usize,7_usize];
_16 = (_8, _10.1, _10.2);
Call(_16.1 = fn18((*RET), _10.1, _14, _10, _5, _16.2, _8, RET, (*RET), _14, _7, _16.2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_8 = _10.0 & _16.0;
RET = core::ptr::addr_of_mut!((*RET));
_10.1 = 10829677193143781615_usize as i8;
(*RET) = _1 as f32;
_2 = [11446526505870745768_usize,984081550362176143_usize,3_usize,11291328902470814421_usize];
_13 = -_1;
Goto(bb10)
}
bb10 = {
_10 = (_8, _16.1, _16.2);
_16.1 = _10.1;
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = 4_usize as f32;
Goto(bb11)
}
bb11 = {
_16 = (_8, _10.1, _10.2);
_1 = 103106725616908807863557853885176039616_i128 as f64;
(*RET) = 9737_u16 as f32;
_15 = Adt42 { fld0: _11 };
_16 = _10;
_16.2 = [6927221805798663974_i64,97567489636966888_i64,(-1711504216935470369_i64),8562068611972095074_i64,5803204605816328923_i64];
_9 = '\u{c7df8}';
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = _16.0 as f32;
_1 = -_13;
_14 = [(-6532977825160418029_i64),(-7644655264307769004_i64),3641493496399758145_i64,(-1529966689789356182_i64)];
_10.0 = _16.0 | _8;
RET = core::ptr::addr_of_mut!((*RET));
_5 = [_10.0,_10.0];
_12 = !_6;
_6 = _12;
_10 = (_16.0, _16.1, _16.2);
Goto(bb12)
}
bb12 = {
_17 = [_9,_9,_9];
_6 = _12;
_10.1 = _16.1;
_12 = !_6;
_12 = !_6;
(*RET) = 3733567774_u32 as f32;
_24 = _9;
(*RET) = 2113050384904548223_usize as f32;
_23 = [8350643284716448804_usize,190738460521722114_usize,3_usize,10908417177976231371_usize,5_usize,6_usize];
_6 = _12 | _12;
_23 = [18408574798633011181_usize,5_usize,12833428175959731509_usize,9544663194891417206_usize,7_usize,5_usize];
_4 = (-30542_i16) as f32;
(*RET) = 43522_u16 as f32;
_16.0 = _10.1 as u128;
_11 = _2;
_1 = 9304739933978774908_u64 as f64;
_16.1 = _24 as i8;
Goto(bb13)
}
bb13 = {
_13 = -_1;
_26 = [_9,_9,_24];
_10 = _16;
_12 = _6 ^ _6;
_15 = Adt42 { fld0: _2 };
_16.1 = _10.1;
_26 = [_24,_24,_9];
_16 = (_8, _10.1, _10.2);
_16.2 = [(-8064101086137347565_i64),5769260701928578483_i64,(-785740558010891751_i64),(-1764421507124827507_i64),2684240508208511620_i64];
_10 = _16;
_20 = _24;
_14 = [8832775994289406807_i64,40715703800556065_i64,1378310405728631298_i64,(-6621050793156274586_i64)];
_27 = !75_u8;
_14 = [7630965008628719352_i64,7774454578055679058_i64,(-2811934626740878402_i64),(-182983911706255156_i64)];
_28 = !9223372036854775807_isize;
_25 = _27;
_19 = !_12;
_28 = _16.1 as isize;
_15.fld0 = [0_usize,16899354910339750139_usize,1_usize,14806808906422591081_usize];
_15.fld0 = _11;
_16.1 = (*RET) as i8;
_24 = _9;
_27 = _25 & _25;
(*RET) = (-4835385118290476127_i64) as f32;
_24 = _9;
_10 = _16;
Goto(bb14)
}
bb14 = {
_14 = [(-5207711188647619365_i64),5336917129186127541_i64,(-7051278279731931545_i64),(-7811780701317543645_i64)];
_3 = [2125217560_u32,1109612422_u32,2105676626_u32];
_1 = _13;
_27 = _25 | _25;
_11 = [6_usize,5026843243090915824_usize,10151815541586482721_usize,3_usize];
(*RET) = _13 as f32;
_14 = [(-4758253191200826815_i64),1724124648370986785_i64,(-3916342502708873144_i64),7789077914121091617_i64];
_22 = [_8,_16.0,_8,_16.0,_8];
_8 = _10.0 | _10.0;
(*RET) = 1484378278_i32 as f32;
_10 = _16;
(*RET) = 2866909730488437371_u64 as f32;
_16.2 = _10.2;
_6 = _12 >= _19;
_14 = [(-4353111754481363252_i64),(-300610504492594155_i64),(-3731831965208778211_i64),(-3651006759547369568_i64)];
_20 = _24;
_10.1 = !_16.1;
_16.2 = [5302762115739816943_i64,(-9197501326006416103_i64),1145855371679951482_i64,(-5566797552305343387_i64),(-3995878349960947544_i64)];
_11 = [3_usize,3_usize,17005115716117130241_usize,4307870744315507826_usize];
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(7_usize, 11_usize, Move(_11), 20_usize, Move(_20), 14_usize, Move(_14), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(7_usize, 27_usize, Move(_27), 26_usize, Move(_26), 8_usize, Move(_8), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(7_usize, 7_usize, Move(_7), 3_usize, Move(_3), 22_usize, Move(_22), 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [i64; 4],mut _2: [u128; 2],mut _3: (u128, i8, [i64; 5]),mut _4: [i32; 6],mut _5: [i32; 6],mut _6: [usize; 4],mut _7: [i64; 5],mut _8: char,mut _9: [i64; 4],mut _10: (u128, i8, [i64; 5]),mut _11: f32,mut _12: (u128, i8, [i64; 5]),mut _13: [u128; 2]) -> *mut f32 {
mir! {
type RET = *mut f32;
let _14: Adt49;
let _15: i16;
let _16: Adt49;
let _17: [i64; 4];
let _18: f64;
let _19: [isize; 4];
let _20: u16;
let _21: [u128; 2];
let _22: u8;
let _23: char;
let _24: u128;
let _25: Adt58;
let _26: f64;
let _27: [usize; 6];
let _28: usize;
let _29: bool;
let _30: char;
let _31: u32;
let _32: [u32; 3];
let _33: [i32; 1];
let _34: [char; 3];
let _35: (u128, i8, [i64; 5]);
let _36: u32;
let _37: isize;
let _38: [usize; 6];
let _39: ();
let _40: ();
{
_12 = (_10.0, _10.1, _3.2);
_10.2 = _7;
_12 = (_10.0, _3.1, _10.2);
_10 = (_3.0, _12.1, _12.2);
_12.0 = !_3.0;
Call(RET = fn9(_3.2, _12, _5, _2, _10.2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = (_12.0, _3.1, _7);
_4 = _5;
_10 = _3;
_2 = [_12.0,_10.0];
_10.0 = _11 as u128;
_11 = 11133_i16 as f32;
_10.0 = 123614208237597985_usize as u128;
RET = core::ptr::addr_of_mut!(_11);
_6 = [4_usize,6_usize,880549738687708039_usize,325212543240148821_usize];
_12 = (_3.0, _3.1, _10.2);
_10 = (_3.0, _3.1, _12.2);
Call(_8 = fn10(_3, _4, _12.2, _10.2, _7, _3, _4, _1, _1, _13, _3.1, _4, _3.1, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = [(-701716311_i32),(-1362142779_i32),(-1409991149_i32),1694284295_i32,(-236512209_i32),(-1246405309_i32)];
_12.2 = [(-7077462300527383338_i64),(-5928302011045887049_i64),2302404786954992_i64,8327564443712678327_i64,(-3521467243071750046_i64)];
_3.0 = (-3_isize) as u128;
_8 = '\u{f2d4a}';
RET = core::ptr::addr_of_mut!((*RET));
_12.1 = _3.1 << _10.1;
_3.1 = -_10.1;
_10 = (_3.0, _3.1, _7);
_3 = (_10.0, _12.1, _7);
_4 = _5;
_3.0 = _12.0 | _12.0;
_13 = _2;
Goto(bb3)
}
bb3 = {
_10.0 = false as u128;
RET = core::ptr::addr_of_mut!(_11);
_20 = false as u16;
Goto(bb4)
}
bb4 = {
_12.2 = [(-5720224181820067785_i64),5114588004583403513_i64,7000948442993833250_i64,5893467812930266523_i64,(-6254731798975074180_i64)];
Call(_12.1 = fn12(_9, _10.2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_17 = [(-2002926576805040505_i64),(-1909705160639044571_i64),(-3226332465302284623_i64),(-490787925001550573_i64)];
_6 = [6_usize,1_usize,4855252811797926867_usize,3_usize];
_12.1 = _3.1 << _3.0;
(*RET) = _12.1 as f32;
_4 = [(-1216353487_i32),77775170_i32,832814688_i32,2105842125_i32,820711075_i32,(-501159284_i32)];
_1 = [(-1359802519855981191_i64),2965938575413347073_i64,(-5131964987881504400_i64),(-1673379221853309309_i64)];
_9 = [4264073497956609297_i64,(-8530350008661833175_i64),(-8030476240430874560_i64),7032014275047520933_i64];
_17 = [(-2270558760166666613_i64),(-1677860031328643248_i64),7263236922272034236_i64,5861948623222257820_i64];
_18 = (-124153292343720266103700864358926445904_i128) as f64;
_5 = [2003044348_i32,599835845_i32,(-904545908_i32),(-932201485_i32),(-389542192_i32),(-771016729_i32)];
(*RET) = _12.1 as f32;
_12.1 = _10.1 | _3.1;
(*RET) = 5_usize as f32;
_23 = _8;
_22 = 7_u8;
_2 = [_10.0,_3.0];
_13 = _2;
_24 = _18 as u128;
_4 = _5;
_3.1 = 795002882_i32 as i8;
_20 = _22 as u16;
_12.0 = 14337_i16 as u128;
_24 = _3.0;
_12.2 = [2584908715275686191_i64,2749735418678473476_i64,(-6447298890834540447_i64),(-5197692043755182672_i64),(-2657494600227821609_i64)];
_6 = [5_usize,6771396028803961593_usize,12768222913312675216_usize,5543103965927826995_usize];
RET = core::ptr::addr_of_mut!((*RET));
_3.2 = _12.2;
_10.0 = !_12.0;
Call((*RET) = fn17(_12.1, _4, _4, _9, _10.2, _6, _8, _10.1, _1, _10.2, _6, _2, _17, _10.2), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_5 = [(-1516047837_i32),(-698796828_i32),(-1507758154_i32),(-3952278_i32),1860881062_i32,(-850476254_i32)];
_3 = (_24, _12.1, _7);
_3 = (_10.0, _12.1, _10.2);
_10 = (_24, _12.1, _7);
(*RET) = 12671_i16 as f32;
_25 = Adt58::Variant1 { fld0: 1093096095_i32,fld1: 17051129004559110086_usize };
_27 = [2_usize,16361016587070527447_usize,4315047966581352727_usize,5438316496142723767_usize,4_usize,4_usize];
_15 = 5096_i16;
_12.2 = [(-5629380051979946785_i64),7271467259007916689_i64,2515299408501874719_i64,(-2567237535204129816_i64),5487442478719881787_i64];
_24 = 1848476447_i32 as u128;
(*RET) = _20 as f32;
_12.0 = 3687358044_u32 as u128;
_23 = _8;
_19 = [(-98_isize),2_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_7 = _3.2;
_10 = (_24, _3.1, _7);
(*RET) = _18 as f32;
_10 = (_12.0, _12.1, _7);
_28 = 0_usize ^ 5_usize;
_3.0 = false as u128;
_13 = _2;
_10.0 = !_3.0;
_10.2 = [(-596393029637428170_i64),3548129314122663837_i64,(-3519743956332023488_i64),5949405896337270804_i64,(-5281915103420178564_i64)];
match _22 {
0 => bb7,
7 => bb9,
_ => bb8
}
}
bb7 = {
_10.0 = false as u128;
RET = core::ptr::addr_of_mut!(_11);
_20 = false as u16;
Goto(bb4)
}
bb8 = {
_5 = [(-701716311_i32),(-1362142779_i32),(-1409991149_i32),1694284295_i32,(-236512209_i32),(-1246405309_i32)];
_12.2 = [(-7077462300527383338_i64),(-5928302011045887049_i64),2302404786954992_i64,8327564443712678327_i64,(-3521467243071750046_i64)];
_3.0 = (-3_isize) as u128;
_8 = '\u{f2d4a}';
RET = core::ptr::addr_of_mut!((*RET));
_12.1 = _3.1 << _10.1;
_3.1 = -_10.1;
_10 = (_3.0, _3.1, _7);
_3 = (_10.0, _12.1, _7);
_4 = _5;
_3.0 = _12.0 | _12.0;
_13 = _2;
Goto(bb3)
}
bb9 = {
_2 = _13;
_12.0 = _3.0;
_12.2 = _3.2;
_9 = [5348450226093352337_i64,(-718649290267618514_i64),(-3426959465403767462_i64),6949587386060216028_i64];
place!(Field::<i32>(Variant(_25, 1), 0)) = -(-1665723255_i32);
_12.1 = _10.1 ^ _3.1;
_3 = _12;
_26 = _18;
Call(_7 = core::intrinsics::transmute(_12.2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_29 = !false;
_12.2 = _7;
(*RET) = 2951489980158380794_i64 as f32;
_7 = [(-8764524104459995311_i64),8620804034529946287_i64,(-6282269205202311334_i64),(-9069141270381292323_i64),390748595374216550_i64];
_29 = _28 >= _28;
_3.1 = !_12.1;
_20 = 1023_u16;
_13 = _2;
(*RET) = _3.0 as f32;
_30 = _23;
(*RET) = _22 as f32;
_12.0 = !_10.0;
_3.1 = _12.1 >> _10.1;
_27 = [_28,_28,_28,_28,_28,_28];
_32 = [1000535785_u32,2535399500_u32,3132736677_u32];
_6 = [_28,_28,_28,_28];
match _22 {
0 => bb7,
1 => bb4,
2 => bb11,
3 => bb12,
7 => bb14,
_ => bb13
}
}
bb11 = {
_10 = (_12.0, _3.1, _7);
_4 = _5;
_10 = _3;
_2 = [_12.0,_10.0];
_10.0 = _11 as u128;
_11 = 11133_i16 as f32;
_10.0 = 123614208237597985_usize as u128;
RET = core::ptr::addr_of_mut!(_11);
_6 = [4_usize,6_usize,880549738687708039_usize,325212543240148821_usize];
_12 = (_3.0, _3.1, _10.2);
_10 = (_3.0, _3.1, _12.2);
Call(_8 = fn10(_3, _4, _12.2, _10.2, _7, _3, _4, _1, _1, _13, _3.1, _4, _3.1, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_12.2 = [(-5720224181820067785_i64),5114588004583403513_i64,7000948442993833250_i64,5893467812930266523_i64,(-6254731798975074180_i64)];
Call(_12.1 = fn12(_9, _10.2), ReturnTo(bb5), UnwindUnreachable())
}
bb13 = {
_10.0 = false as u128;
RET = core::ptr::addr_of_mut!(_11);
_20 = false as u16;
Goto(bb4)
}
bb14 = {
_23 = _8;
_33 = [Field::<i32>(Variant(_25, 1), 0)];
_3.0 = _10.0 * _12.0;
_8 = _23;
_27 = [_28,_28,_28,_28,_28,_28];
_27 = [_28,_28,_28,_28,_28,_28];
_34 = [_23,_30,_23];
_31 = !2518778899_u32;
_19 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,43_isize];
_32 = [_31,_31,_31];
(*RET) = _31 as f32;
_1 = _9;
RET = core::ptr::addr_of_mut!(_11);
_17 = [(-7482295639279159130_i64),(-8537891174500986448_i64),4871252641294611355_i64,(-217242911873902450_i64)];
_13 = _2;
_30 = _23;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(8_usize, 13_usize, Move(_13), 34_usize, Move(_34), 24_usize, Move(_24), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(8_usize, 15_usize, Move(_15), 31_usize, Move(_31), 2_usize, Move(_2), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(8_usize, 3_usize, Move(_3), 17_usize, Move(_17), 20_usize, Move(_20), 29_usize, Move(_29)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_39 = dump_var(8_usize, 28_usize, Move(_28), 40_usize, _40, 40_usize, _40, 40_usize, _40), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: [i64; 5],mut _2: (u128, i8, [i64; 5]),mut _3: [i32; 6],mut _4: [u128; 2],mut _5: [i64; 5]) -> *mut f32 {
mir! {
type RET = *mut f32;
let _6: u64;
let _7: [usize; 6];
let _8: Adt55;
let _9: bool;
let _10: *const &'static u32;
let _11: i128;
let _12: [u128; 5];
let _13: Adt54;
let _14: u64;
let _15: Adt44;
let _16: f32;
let _17: u8;
let _18: Adt55;
let _19: [isize; 3];
let _20: isize;
let _21: Adt51;
let _22: ();
let _23: ();
{
_5 = [(-2966838020614677503_i64),(-6086954147450250586_i64),(-8338151501809605587_i64),(-5397583276107961621_i64),(-8382857722735566634_i64)];
_2 = (122091852908684439687645754790951764054_u128, (-22_i8), _1);
_5 = _2.2;
_2.2 = [7507543478757594181_i64,(-6109763891367633200_i64),7071620709422843827_i64,7692973559089882167_i64,7900641024478898341_i64];
_1 = [(-8885869638122422386_i64),5436482729322055442_i64,(-3898187695812464666_i64),2849159839141914211_i64,9005764560590845053_i64];
_2.2 = _5;
_4 = [_2.0,_2.0];
_5 = _1;
_2.0 = !141471359966928318858975567154536972741_u128;
match _2.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463463374607431768211434 => bb8,
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
_2 = (164783615604283684803524614320701884110_u128, 48_i8, _5);
_6 = 13878738240398962021_u64 * 13368401030849703165_u64;
_2.0 = 238282449403271710549516214380138998442_u128 ^ 98598448066398510852696763334928897188_u128;
_2 = (144187871880849696394539907311395213383_u128, 35_i8, _1);
_9 = _6 <= _6;
_4 = [_2.0,_2.0];
_2.2 = _1;
_4 = [_2.0,_2.0];
_5 = _2.2;
_5 = [(-4134347981133055481_i64),937156934353231960_i64,8703760414903340898_i64,(-7337094435855327980_i64),(-1537957600533122719_i64)];
_3 = [372690851_i32,1541014221_i32,640415998_i32,220704272_i32,(-1736659379_i32),(-715284899_i32)];
_9 = _2.0 >= _2.0;
_9 = true;
_2.1 = 37_i8;
_7 = [4_usize,5753115242787858502_usize,0_usize,1690462786524211335_usize,11652416692285667713_usize,2_usize];
_2.1 = 53_i8 | 115_i8;
_9 = true;
_5 = [(-7629544723748304721_i64),(-286397597620783153_i64),5247234809097096587_i64,(-5965095301087737221_i64),(-4147342703492223763_i64)];
_5 = _1;
_2.2 = [(-8736523227104868585_i64),7421168229426410071_i64,7366287549189510467_i64,(-2387417102526607423_i64),3675966741350581024_i64];
_2.1 = (-1632092015_i32) as i8;
_9 = !true;
match _2.0 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb9,
144187871880849696394539907311395213383 => bb11,
_ => bb10
}
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_7 = [5931339928416961126_usize,7433002516183483087_usize,16982513674041156373_usize,6_usize,5_usize,10345835623436202818_usize];
Goto(bb12)
}
bb12 = {
_2.1 = 6_i8 + 61_i8;
_5 = [254939172080120821_i64,2289149861592096570_i64,(-4744475875754379101_i64),4207892943555834633_i64,(-3038307865910602215_i64)];
_2.0 = !153507086388350458740156827046950249895_u128;
_4 = [_2.0,_2.0];
_2.1 = 85_i8 << _6;
_2.0 = !296432066778177636998977073608949676069_u128;
_6 = 240_u8 as u64;
_2.0 = !194661233769960297010469361853586604594_u128;
_2 = (118475687418776920518291530343935650413_u128, (-79_i8), _5);
_2 = (305162854161363229051280936644276128374_u128, (-71_i8), _5);
_9 = !false;
_13.fld2.3 = _9;
_11 = !114889737538441967465320890994220893330_i128;
_6 = _9 as u64;
_13.fld0 = _13.fld2.3;
_2.2 = _5;
_12 = [_2.0,_2.0,_2.0,_2.0,_2.0];
_13.fld2.1 = 2_usize;
match _2.1 {
0 => bb7,
1 => bb10,
2 => bb5,
340282366920938463463374607431768211385 => bb13,
_ => bb6
}
}
bb13 = {
_13.fld6 = [(-8390458286140324690_i64),(-4324682717609572843_i64),5536643273714339780_i64,(-6331979130726839436_i64),(-1475663154572277915_i64)];
_13.fld2.0 = [_2.0,_2.0,_2.0,_2.0,_2.0];
_14 = _13.fld2.1 as u64;
_2 = (309813060330935139173119096328337551241_u128, 54_i8, _13.fld6);
_13.fld2.0 = [_2.0,_2.0,_2.0,_2.0,_2.0];
_2.2 = [(-6258288478167668814_i64),2698208716984770776_i64,(-1826658403405700352_i64),(-7868035179552982770_i64),(-3206029343587133940_i64)];
_13.fld3 = ['\u{2b66a}','\u{25c0}','\u{f56cd}'];
_4 = [_2.0,_2.0];
_2 = (97441381504891035543349133869407307983_u128, 88_i8, _1);
_16 = _13.fld2.1 as f32;
_12 = [_2.0,_2.0,_2.0,_2.0,_2.0];
Goto(bb14)
}
bb14 = {
_2.0 = 52791989095000397785903286888529346030_u128;
_13.fld2.0 = _12;
_2.1 = (-2_i8) - 109_i8;
match _13.fld2.1 {
0 => bb8,
1 => bb15,
2 => bb18,
_ => bb17
}
}
bb15 = {
_13.fld6 = [(-8390458286140324690_i64),(-4324682717609572843_i64),5536643273714339780_i64,(-6331979130726839436_i64),(-1475663154572277915_i64)];
_13.fld2.0 = [_2.0,_2.0,_2.0,_2.0,_2.0];
_14 = _13.fld2.1 as u64;
_2 = (309813060330935139173119096328337551241_u128, 54_i8, _13.fld6);
_13.fld2.0 = [_2.0,_2.0,_2.0,_2.0,_2.0];
_2.2 = [(-6258288478167668814_i64),2698208716984770776_i64,(-1826658403405700352_i64),(-7868035179552982770_i64),(-3206029343587133940_i64)];
_13.fld3 = ['\u{2b66a}','\u{25c0}','\u{f56cd}'];
_4 = [_2.0,_2.0];
_2 = (97441381504891035543349133869407307983_u128, 88_i8, _1);
_16 = _13.fld2.1 as f32;
_12 = [_2.0,_2.0,_2.0,_2.0,_2.0];
Goto(bb14)
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
_13.fld2.2 = core::ptr::addr_of_mut!(_16);
_11 = 78558782259409011037759126744985352880_i128;
_16 = _2.1 as f32;
_3 = [1824393971_i32,1231606411_i32,1289049244_i32,(-872008042_i32),414507078_i32,(-487825827_i32)];
RET = core::ptr::addr_of_mut!(_16);
_13.fld0 = _13.fld2.3;
_3 = [782220919_i32,(-268902982_i32),(-1197735807_i32),106163703_i32,(-470428642_i32),1065604739_i32];
_13.fld2.4 = core::ptr::addr_of_mut!(_13.fld2.0);
_7 = [_13.fld2.1,_13.fld2.1,_13.fld2.1,_13.fld2.1,_13.fld2.1,_13.fld2.1];
_13.fld6 = [7122416015627221701_i64,(-3926132986106835204_i64),(-3798761528096797734_i64),(-1942667999544718954_i64),(-2239943972261236180_i64)];
(*RET) = 25635_u16 as f32;
_13.fld0 = !_9;
_13.fld6 = [355253365006554362_i64,2952932746841856838_i64,(-1678585968854757440_i64),(-8491449847694703048_i64),5332155214819123462_i64];
_13.fld3 = ['\u{77090}','\u{35fce}','\u{f16e9}'];
_13.fld0 = _9 ^ _13.fld2.3;
RET = _13.fld2.2;
RET = _13.fld2.2;
(*RET) = 2870285023_u32 as f32;
_2 = (235871171901312672515558157907334913489_u128, 29_i8, _1);
_2.0 = 274246952551805779755062930214659005772_u128 + 290335151983635118718334050809644856639_u128;
_13.fld1 = !_13.fld2.1;
_13.fld5 = core::ptr::addr_of_mut!(_14);
_19 = [(-32_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_1 = [1385309727422240226_i64,7352371658798501361_i64,3096960066198798969_i64,3176160253915997219_i64,(-6548949801297658168_i64)];
Goto(bb19)
}
bb19 = {
Call(_22 = dump_var(9_usize, 9_usize, Move(_9), 5_usize, Move(_5), 19_usize, Move(_19), 11_usize, Move(_11)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_22 = dump_var(9_usize, 3_usize, Move(_3), 14_usize, Move(_14), 23_usize, _23, 23_usize, _23), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: (u128, i8, [i64; 5]),mut _2: [i32; 6],mut _3: [i64; 5],mut _4: [i64; 5],mut _5: [i64; 5],mut _6: (u128, i8, [i64; 5]),mut _7: [i32; 6],mut _8: [i64; 4],mut _9: [i64; 4],mut _10: [u128; 2],mut _11: i8,mut _12: [i32; 6],mut _13: i8,mut _14: [i32; 6]) -> char {
mir! {
type RET = char;
let _15: [usize; 4];
let _16: Adt58;
let _17: Adt42;
let _18: isize;
let _19: isize;
let _20: i128;
let _21: [u128; 2];
let _22: Adt42;
let _23: [u32; 3];
let _24: i16;
let _25: [i32; 6];
let _26: u16;
let _27: [usize; 4];
let _28: bool;
let _29: [i64; 5];
let _30: bool;
let _31: Adt57;
let _32: Adt53;
let _33: [u128; 2];
let _34: [i64; 5];
let _35: char;
let _36: f64;
let _37: (&'static u32,);
let _38: [i64; 4];
let _39: [usize; 4];
let _40: [u128; 2];
let _41: u32;
let _42: *mut f32;
let _43: ();
let _44: ();
{
_1.1 = !_6.1;
RET = '\u{e73f5}';
_3 = _5;
_6.0 = _1.0;
_15 = [12081281392884533154_usize,3_usize,0_usize,2_usize];
_15 = [7636025303868579506_usize,17619225646753993931_usize,0_usize,0_usize];
_4 = [4893480871906544547_i64,7612700480216800296_i64,(-4392170434925994080_i64),(-242718165727483524_i64),5854081160770873488_i64];
_1.1 = 7_usize as i8;
_1.1 = false as i8;
RET = '\u{daf2d}';
_1.2 = _3;
_5 = [(-6262736488662521902_i64),6150455936820471071_i64,6215520422931984625_i64,(-215413813624887058_i64),(-8127372117709573042_i64)];
_1.0 = _6.0 | _6.0;
_6.1 = _1.1 << _11;
_13 = !_1.1;
_8 = _9;
Goto(bb1)
}
bb1 = {
_1.2 = [7846236580057317125_i64,5870076839094528297_i64,(-3471420907417817925_i64),37203865049590364_i64,6118884296668567658_i64];
_12 = _7;
_9 = [460930222964775639_i64,5992628232669343022_i64,2701612742661999357_i64,(-1261509394276188723_i64)];
_11 = _6.1 ^ _6.1;
_15 = [4_usize,7_usize,18344672297118183494_usize,0_usize];
_6.2 = [3825817462039927098_i64,(-4128997915418111681_i64),(-1903975371140339343_i64),(-8044395056551328400_i64),(-1444841109389214890_i64)];
_17 = Adt42 { fld0: _15 };
_1 = (_6.0, _6.1, _6.2);
_4 = [8623556244784907209_i64,6040416527225907460_i64,9214379336521173156_i64,(-533594413705983469_i64),(-2085674264304917433_i64)];
_8 = [(-4508304458114065983_i64),(-6172435144722821701_i64),(-2703634889770392615_i64),8971754188653816419_i64];
_1.2 = _5;
_3 = _5;
_5 = _6.2;
_15 = [4_usize,13508427899827078228_usize,4_usize,1_usize];
_7 = [2076954418_i32,693961772_i32,(-388430156_i32),(-791669979_i32),430404899_i32,(-917394_i32)];
_8 = [(-3905950203814087493_i64),(-7488068617632009844_i64),(-7915086077027365553_i64),(-8253887660471989443_i64)];
_9 = [9212248486454280354_i64,3992409105708731031_i64,(-4272199993244723540_i64),(-4773616932018272338_i64)];
_17.fld0 = [5810003917950892012_usize,4_usize,8507437380797481479_usize,1_usize];
_7 = [(-2107853929_i32),2011922570_i32,146082092_i32,(-632047624_i32),321266602_i32,(-1648898846_i32)];
_7 = [1229358199_i32,(-853719181_i32),171707773_i32,623227335_i32,(-236547168_i32),737215404_i32];
_1.2 = _6.2;
_1.1 = _11 << _11;
_2 = _12;
_12 = [1264976927_i32,(-646375302_i32),(-256532529_i32),672959762_i32,1425972071_i32,(-738603756_i32)];
_15 = [4_usize,3_usize,5_usize,3_usize];
_13 = _11 ^ _11;
_21 = [_6.0,_6.0];
RET = '\u{226e9}';
Goto(bb2)
}
bb2 = {
_10 = [_6.0,_1.0];
_18 = 71355798926263278253740215762073486329_i128 as isize;
_8 = [154375070260340435_i64,(-8934482624886180290_i64),(-632427542565363380_i64),(-1278735966667036057_i64)];
_1.2 = [4686539058410529546_i64,4037402460968161354_i64,4276387695093198038_i64,(-2903732112435311092_i64),3715336277159635341_i64];
_18 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_10 = _21;
_16 = Adt58::Variant1 { fld0: (-380415782_i32),fld1: 0_usize };
_23 = [2642392497_u32,1004905001_u32,3419627917_u32];
Goto(bb3)
}
bb3 = {
_23 = [4227940911_u32,2446857219_u32,998506847_u32];
_20 = RET as i128;
_25 = _7;
_18 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_21 = [_6.0,_1.0];
_22 = Adt42 { fld0: _15 };
_6 = (_1.0, _13, _4);
Goto(bb4)
}
bb4 = {
place!(Field::<i32>(Variant(_16, 1), 0)) = -(-931470650_i32);
_22 = Adt42 { fld0: _17.fld0 };
_8 = _9;
_17.fld0 = _22.fld0;
_6.0 = _1.0 & _1.0;
_1.0 = _6.0;
_20 = _6.1 as i128;
_6.0 = _1.0;
_19 = -_18;
place!(Field::<usize>(Variant(_16, 1), 1)) = 14164902364696550771_usize;
_23 = [14355996_u32,2760591458_u32,857227113_u32];
place!(Field::<i32>(Variant(_16, 1), 0)) = (-1904486905_i32);
_1.1 = _13 >> _13;
_22.fld0 = [Field::<usize>(Variant(_16, 1), 1),Field::<usize>(Variant(_16, 1), 1),Field::<usize>(Variant(_16, 1), 1),Field::<usize>(Variant(_16, 1), 1)];
_16 = Adt58::Variant1 { fld0: 787613680_i32,fld1: 5362280228002957126_usize };
_14 = _2;
_16 = Adt58::Variant1 { fld0: (-967683763_i32),fld1: 3887914060070710807_usize };
_5 = [3999101460488958869_i64,4682725784418490441_i64,(-7897868178774162042_i64),3231101313438643560_i64,(-3733531121500762262_i64)];
_28 = !true;
_21 = [_6.0,_1.0];
RET = '\u{9a4a4}';
_1.0 = _6.0 >> _18;
_3 = [3127248211293274352_i64,(-5924151161099191048_i64),7339732181002527637_i64,(-1158666556468062558_i64),3668994105583114124_i64];
Goto(bb5)
}
bb5 = {
_29 = [8099041708961476365_i64,(-3828622112073365798_i64),1443642194919582754_i64,6009713066726742747_i64,(-6722110360581947530_i64)];
_10 = _21;
_3 = _4;
_4 = [(-3016991767715139162_i64),(-5095393082292678511_i64),3092596558569256941_i64,(-1511096525560331819_i64),(-6804926756105751938_i64)];
_8 = [8095640539640183421_i64,(-2485401235740014210_i64),7424094457894747543_i64,634659491779211752_i64];
_11 = _13 - _1.1;
Call(_14 = core::intrinsics::transmute(_2), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_6.0 = _1.0 & _1.0;
_12 = _14;
_6.2 = _29;
_6 = _1;
_19 = !_18;
_25 = [1144833758_i32,333950973_i32,(-1514779821_i32),(-1585765799_i32),(-869059236_i32),(-1007931333_i32)];
_29 = [2885293085044567497_i64,(-5069279896854957908_i64),6777471637169287881_i64,45236867846874082_i64,(-5454629037704713902_i64)];
_29 = _4;
_27 = [14822676516240930325_usize,1_usize,13553816912210826923_usize,4_usize];
_2 = _14;
_12 = _2;
_17 = Move(_22);
_22 = Move(_17);
_28 = false ^ false;
_17 = Move(_22);
_6.0 = !_1.0;
_6.1 = !_1.1;
_1 = _6;
RET = '\u{936ab}';
_5 = _29;
_6.1 = -_1.1;
_33 = [_1.0,_1.0];
Call(_2 = fn11(_11, _1.1, _6.0, _4, _9, _20, _11, _4), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_1 = _6;
_29 = [8648131443846309662_i64,(-1733112984108131547_i64),(-1286835657254582133_i64),(-2444031660329962299_i64),(-8496225863530716934_i64)];
_12 = [383344970_i32,35717864_i32,751046858_i32,1945558751_i32,(-708192279_i32),(-1287296128_i32)];
_5 = [(-3940167474461264793_i64),(-32614352049031723_i64),7868723547660189668_i64,(-3439727740626760264_i64),1183122509370451776_i64];
_30 = _28;
RET = '\u{57365}';
_26 = _19 as u16;
_6 = (_1.0, _1.1, _29);
_6.2 = [(-4126527074448236681_i64),(-657896939176937898_i64),(-8132031213338435051_i64),(-4716952373107816098_i64),(-3720803461684912138_i64)];
_17 = Adt42 { fld0: _15 };
_10 = _33;
_9 = _8;
_6.1 = 12_u8 as i8;
place!(Field::<usize>(Variant(_16, 1), 1)) = !3_usize;
_15 = [Field::<usize>(Variant(_16, 1), 1),Field::<usize>(Variant(_16, 1), 1),Field::<usize>(Variant(_16, 1), 1),Field::<usize>(Variant(_16, 1), 1)];
_1.1 = _13 + _11;
_30 = !_28;
_26 = _30 as u16;
_26 = 54534_u16;
match _26 {
0 => bb6,
1 => bb3,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
54534 => bb14,
_ => bb13
}
}
bb8 = {
_6.0 = _1.0 & _1.0;
_12 = _14;
_6.2 = _29;
_6 = _1;
_19 = !_18;
_25 = [1144833758_i32,333950973_i32,(-1514779821_i32),(-1585765799_i32),(-869059236_i32),(-1007931333_i32)];
_29 = [2885293085044567497_i64,(-5069279896854957908_i64),6777471637169287881_i64,45236867846874082_i64,(-5454629037704713902_i64)];
_29 = _4;
_27 = [14822676516240930325_usize,1_usize,13553816912210826923_usize,4_usize];
_2 = _14;
_12 = _2;
_17 = Move(_22);
_22 = Move(_17);
_28 = false ^ false;
_17 = Move(_22);
_6.0 = !_1.0;
_6.1 = !_1.1;
_1 = _6;
RET = '\u{936ab}';
_5 = _29;
_6.1 = -_1.1;
_33 = [_1.0,_1.0];
Call(_2 = fn11(_11, _1.1, _6.0, _4, _9, _20, _11, _4), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
_29 = [8099041708961476365_i64,(-3828622112073365798_i64),1443642194919582754_i64,6009713066726742747_i64,(-6722110360581947530_i64)];
_10 = _21;
_3 = _4;
_4 = [(-3016991767715139162_i64),(-5095393082292678511_i64),3092596558569256941_i64,(-1511096525560331819_i64),(-6804926756105751938_i64)];
_8 = [8095640539640183421_i64,(-2485401235740014210_i64),7424094457894747543_i64,634659491779211752_i64];
_11 = _13 - _1.1;
Call(_14 = core::intrinsics::transmute(_2), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
place!(Field::<i32>(Variant(_16, 1), 0)) = -(-931470650_i32);
_22 = Adt42 { fld0: _17.fld0 };
_8 = _9;
_17.fld0 = _22.fld0;
_6.0 = _1.0 & _1.0;
_1.0 = _6.0;
_20 = _6.1 as i128;
_6.0 = _1.0;
_19 = -_18;
place!(Field::<usize>(Variant(_16, 1), 1)) = 14164902364696550771_usize;
_23 = [14355996_u32,2760591458_u32,857227113_u32];
place!(Field::<i32>(Variant(_16, 1), 0)) = (-1904486905_i32);
_1.1 = _13 >> _13;
_22.fld0 = [Field::<usize>(Variant(_16, 1), 1),Field::<usize>(Variant(_16, 1), 1),Field::<usize>(Variant(_16, 1), 1),Field::<usize>(Variant(_16, 1), 1)];
_16 = Adt58::Variant1 { fld0: 787613680_i32,fld1: 5362280228002957126_usize };
_14 = _2;
_16 = Adt58::Variant1 { fld0: (-967683763_i32),fld1: 3887914060070710807_usize };
_5 = [3999101460488958869_i64,4682725784418490441_i64,(-7897868178774162042_i64),3231101313438643560_i64,(-3733531121500762262_i64)];
_28 = !true;
_21 = [_6.0,_1.0];
RET = '\u{9a4a4}';
_1.0 = _6.0 >> _18;
_3 = [3127248211293274352_i64,(-5924151161099191048_i64),7339732181002527637_i64,(-1158666556468062558_i64),3668994105583114124_i64];
Goto(bb5)
}
bb11 = {
_23 = [4227940911_u32,2446857219_u32,998506847_u32];
_20 = RET as i128;
_25 = _7;
_18 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_21 = [_6.0,_1.0];
_22 = Adt42 { fld0: _15 };
_6 = (_1.0, _13, _4);
Goto(bb4)
}
bb12 = {
_10 = [_6.0,_1.0];
_18 = 71355798926263278253740215762073486329_i128 as isize;
_8 = [154375070260340435_i64,(-8934482624886180290_i64),(-632427542565363380_i64),(-1278735966667036057_i64)];
_1.2 = [4686539058410529546_i64,4037402460968161354_i64,4276387695093198038_i64,(-2903732112435311092_i64),3715336277159635341_i64];
_18 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_10 = _21;
_16 = Adt58::Variant1 { fld0: (-380415782_i32),fld1: 0_usize };
_23 = [2642392497_u32,1004905001_u32,3419627917_u32];
Goto(bb3)
}
bb13 = {
_1.2 = [7846236580057317125_i64,5870076839094528297_i64,(-3471420907417817925_i64),37203865049590364_i64,6118884296668567658_i64];
_12 = _7;
_9 = [460930222964775639_i64,5992628232669343022_i64,2701612742661999357_i64,(-1261509394276188723_i64)];
_11 = _6.1 ^ _6.1;
_15 = [4_usize,7_usize,18344672297118183494_usize,0_usize];
_6.2 = [3825817462039927098_i64,(-4128997915418111681_i64),(-1903975371140339343_i64),(-8044395056551328400_i64),(-1444841109389214890_i64)];
_17 = Adt42 { fld0: _15 };
_1 = (_6.0, _6.1, _6.2);
_4 = [8623556244784907209_i64,6040416527225907460_i64,9214379336521173156_i64,(-533594413705983469_i64),(-2085674264304917433_i64)];
_8 = [(-4508304458114065983_i64),(-6172435144722821701_i64),(-2703634889770392615_i64),8971754188653816419_i64];
_1.2 = _5;
_3 = _5;
_5 = _6.2;
_15 = [4_usize,13508427899827078228_usize,4_usize,1_usize];
_7 = [2076954418_i32,693961772_i32,(-388430156_i32),(-791669979_i32),430404899_i32,(-917394_i32)];
_8 = [(-3905950203814087493_i64),(-7488068617632009844_i64),(-7915086077027365553_i64),(-8253887660471989443_i64)];
_9 = [9212248486454280354_i64,3992409105708731031_i64,(-4272199993244723540_i64),(-4773616932018272338_i64)];
_17.fld0 = [5810003917950892012_usize,4_usize,8507437380797481479_usize,1_usize];
_7 = [(-2107853929_i32),2011922570_i32,146082092_i32,(-632047624_i32),321266602_i32,(-1648898846_i32)];
_7 = [1229358199_i32,(-853719181_i32),171707773_i32,623227335_i32,(-236547168_i32),737215404_i32];
_1.2 = _6.2;
_1.1 = _11 << _11;
_2 = _12;
_12 = [1264976927_i32,(-646375302_i32),(-256532529_i32),672959762_i32,1425972071_i32,(-738603756_i32)];
_15 = [4_usize,3_usize,5_usize,3_usize];
_13 = _11 ^ _11;
_21 = [_6.0,_6.0];
RET = '\u{226e9}';
Goto(bb2)
}
bb14 = {
_1.2 = [(-8216458452679404849_i64),8166348863654310351_i64,(-2567489021202415017_i64),2285384539712796115_i64,5557609561811836256_i64];
_24 = (-5213_i16) >> _1.1;
_5 = [4134739053853805496_i64,3956428085526385322_i64,4270440686524217894_i64,6967539743454925220_i64,(-1825940353259627338_i64)];
_9 = [6744283916277954064_i64,1570242557417390601_i64,4882062919413340956_i64,(-2519443841501192248_i64)];
_13 = _11 + _11;
_40 = _21;
_13 = _1.1 & _11;
_10 = _33;
_21 = [_6.0,_6.0];
_34 = [(-8755548370295909556_i64),(-8226538318580585055_i64),(-8719459424722986912_i64),6311700812334147172_i64,8435184805424769871_i64];
_16 = Adt58::Variant1 { fld0: (-314718343_i32),fld1: 2_usize };
_22 = Move(_17);
_14 = _7;
_16 = Adt58::Variant1 { fld0: (-756792606_i32),fld1: 2_usize };
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(10_usize, 27_usize, Move(_27), 29_usize, Move(_29), 24_usize, Move(_24), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(10_usize, 23_usize, Move(_23), 11_usize, Move(_11), 3_usize, Move(_3), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(10_usize, 18_usize, Move(_18), 10_usize, Move(_10), 5_usize, Move(_5), 25_usize, Move(_25)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_43 = dump_var(10_usize, 30_usize, Move(_30), 8_usize, Move(_8), 4_usize, Move(_4), 44_usize, _44), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: i8,mut _2: i8,mut _3: u128,mut _4: [i64; 5],mut _5: [i64; 4],mut _6: i128,mut _7: i8,mut _8: [i64; 5]) -> [i32; 6] {
mir! {
type RET = [i32; 6];
let _9: *mut [u128; 5];
let _10: Adt58;
let _11: *mut [u128; 5];
let _12: u64;
let _13: Adt43;
let _14: *const i16;
let _15: *const &'static u32;
let _16: *mut f32;
let _17: f64;
let _18: Adt47;
let _19: [usize; 6];
let _20: f64;
let _21: [usize; 6];
let _22: (u128, i8, [i64; 5]);
let _23: bool;
let _24: *const &'static u32;
let _25: bool;
let _26: (&'static u32,);
let _27: [i32; 1];
let _28: ();
let _29: ();
{
RET = [(-727105139_i32),1551201145_i32,2031187311_i32,(-789551101_i32),515934102_i32,1910828102_i32];
RET = [120925795_i32,723631784_i32,633548510_i32,905837208_i32,1283443608_i32,2067813157_i32];
RET = [(-47206923_i32),(-565130034_i32),762421053_i32,1637822533_i32,847432773_i32,1155357140_i32];
_3 = !264032784141439571152087464562203084081_u128;
_1 = -_7;
_5 = [(-8875899443120261971_i64),(-8847893165982879203_i64),(-5784508964974464884_i64),4151201406729150139_i64];
RET = [276289232_i32,(-1343674074_i32),(-609715992_i32),579350556_i32,(-619346331_i32),(-1326369167_i32)];
_2 = _7 | _1;
RET = [(-1466035909_i32),(-446742478_i32),554316124_i32,(-1626490044_i32),(-1346332525_i32),(-1223404512_i32)];
_8 = [2329870943995275983_i64,(-1416297088157371849_i64),1220214249978339560_i64,165294009991161812_i64,5039604528172189854_i64];
_10 = Adt58::Variant1 { fld0: (-1438866195_i32),fld1: 6291773912383176967_usize };
_1 = _2 + _7;
_5 = [1392526699618068545_i64,9092875659129078228_i64,419572671729594553_i64,(-7151958382068769632_i64)];
_4 = _8;
_2 = _1 - _7;
_2 = !_1;
_5 = [1986173657899079594_i64,3375787263270962598_i64,(-3761543367087174063_i64),(-7806336934560216598_i64)];
place!(Field::<i32>(Variant(_10, 1), 0)) = (-676639123_i32) & (-39786752_i32);
_2 = !_1;
_8 = [4559417504634065366_i64,7626787906357506380_i64,6585168346696922594_i64,(-8311189663316491706_i64),5613932992328580202_i64];
_5 = [4441573627738286485_i64,(-6274091312873344492_i64),6010274989858142864_i64,(-7053657078754928957_i64)];
place!(Field::<i32>(Variant(_10, 1), 0)) = '\u{30375}' as i32;
_4 = [(-960127590738508958_i64),(-7189240707717001652_i64),1343737583395751231_i64,4167652283135106404_i64,7765219163900355152_i64];
_8 = [6061998189771001598_i64,1183739611216805128_i64,(-3282909271219840747_i64),(-7993320482127780440_i64),608308425637029813_i64];
_3 = (-325658769286891081_i64) as u128;
Goto(bb1)
}
bb1 = {
_4 = [(-8796779515537401763_i64),(-6716277617659611625_i64),975104880865062456_i64,8156740299770254186_i64,1039507397266292573_i64];
_2 = _7;
RET = [Field::<i32>(Variant(_10, 1), 0),Field::<i32>(Variant(_10, 1), 0),Field::<i32>(Variant(_10, 1), 0),Field::<i32>(Variant(_10, 1), 0),Field::<i32>(Variant(_10, 1), 0),Field::<i32>(Variant(_10, 1), 0)];
_10 = Adt58::Variant1 { fld0: (-96178903_i32),fld1: 11862300255842057934_usize };
_3 = 280072676553481529679764757067377537466_u128 ^ 190098695467270447795797034548662646203_u128;
_12 = 3745353363769338480_u64;
_4 = [(-1807789702807192829_i64),254700701290223869_i64,7224710923605225727_i64,4995624268728307140_i64,(-411712100656512667_i64)];
_7 = _1;
_12 = 8783624532457820732_u64;
_10 = Adt58::Variant1 { fld0: 687218376_i32,fld1: 7_usize };
_2 = 9223372036854775807_isize as i8;
place!(Field::<i32>(Variant(_10, 1), 0)) = _12 as i32;
_3 = 26836441081229006060893681466499697477_u128;
_4 = [(-2493246993235687053_i64),8601361189468438078_i64,(-1851172065608161383_i64),(-2025755554871155022_i64),(-9100021757889571826_i64)];
_12 = !16217550024862245444_u64;
_10 = Adt58::Variant1 { fld0: 1860614055_i32,fld1: 4_usize };
RET = [(-867045594_i32),(-761794095_i32),(-1636322536_i32),(-1491955645_i32),(-266328831_i32),194471746_i32];
_8 = _4;
_6 = (-63815403882425702186997712597464902830_i128) | 18346894152169943062297724334767333603_i128;
_8 = [(-1316418857915689772_i64),(-3616363571033252618_i64),1405490447652171380_i64,3941322882027307892_i64,8895486388514644828_i64];
_2 = true as i8;
RET = [(-1316302874_i32),151850661_i32,1689768569_i32,1578287245_i32,122066018_i32,(-1984400447_i32)];
_2 = -_7;
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
26836441081229006060893681466499697477 => bb7,
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
_5 = [5542544412074178463_i64,(-2408123165407781015_i64),4822825470050139220_i64,3054103399622096985_i64];
_7 = 3440233638_u32 as i8;
_8 = [(-6979508674859681403_i64),3683378196024366710_i64,1438779268885723193_i64,(-965876345670880548_i64),(-3789784905269553252_i64)];
RET = [1395146897_i32,926466048_i32,(-602266452_i32),1261962935_i32,(-1562279902_i32),1270446734_i32];
_4 = _8;
_6 = 73346535924375243953879004658598001607_i128 - (-82528735770162172136572704517257527122_i128);
place!(Field::<i32>(Variant(_10, 1), 0)) = (-1986899346_i32);
_17 = 8964262749323548584_usize as f64;
_19 = [17065962295318538226_usize,13763877226080943730_usize,5_usize,18324950842429865198_usize,3883808223093400189_usize,5_usize];
_6 = 9223372036854775807_isize as i128;
_20 = Field::<i32>(Variant(_10, 1), 0) as f64;
place!(Field::<usize>(Variant(_10, 1), 1)) = 7_usize << _1;
_17 = -_20;
_12 = 11717070925805183191_u64;
SetDiscriminant(_10, 1);
_8 = _4;
_19 = [838677671536347159_usize,2_usize,11881357076100623257_usize,17539745672780388069_usize,5_usize,4_usize];
_20 = _17;
_2 = !_1;
_8 = [(-6702920548907804547_i64),2569411636894058665_i64,1433372632537846147_i64,8329099995358155601_i64,(-1448027471773416714_i64)];
place!(Field::<i32>(Variant(_10, 1), 0)) = 1183383236_i32 | 1409322795_i32;
_7 = _1 << _2;
place!(Field::<i32>(Variant(_10, 1), 0)) = -(-741225414_i32);
_22.1 = -_2;
_20 = _17 - _17;
_22.0 = true as u128;
match _3 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb8,
6 => bb9,
26836441081229006060893681466499697477 => bb11,
_ => bb10
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
_19 = [5_usize,11914393748098793002_usize,2_usize,4_usize,14593051104443781894_usize,0_usize];
_17 = -_20;
_23 = true;
place!(Field::<usize>(Variant(_10, 1), 1)) = 32095_i16 as usize;
match _12 {
0 => bb12,
11717070925805183191 => bb14,
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
_20 = _6 as f64;
_22.2 = _4;
place!(Field::<i32>(Variant(_10, 1), 0)) = (-485159467_i32);
_3 = !_22.0;
_25 = _22.1 == _22.1;
_5 = [6159795754071649221_i64,4014450500823980628_i64,7348786821488116049_i64,(-9213992359513186848_i64)];
_7 = _2;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(11_usize, 22_usize, Move(_22), 3_usize, Move(_3), 25_usize, Move(_25), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(11_usize, 12_usize, Move(_12), 19_usize, Move(_19), 29_usize, _29, 29_usize, _29), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: [i64; 4],mut _2: [i64; 5]) -> i8 {
mir! {
type RET = i8;
let _3: i8;
let _4: f32;
let _5: Adt48;
let _6: u128;
let _7: [u128; 5];
let _8: Adt48;
let _9: [u128; 5];
let _10: u8;
let _11: [i64; 5];
let _12: f64;
let _13: [u32; 3];
let _14: &'static u32;
let _15: isize;
let _16: (&'static u32,);
let _17: Adt51;
let _18: u64;
let _19: i128;
let _20: Adt45;
let _21: f32;
let _22: [u32; 3];
let _23: Adt43;
let _24: [i32; 1];
let _25: f64;
let _26: i8;
let _27: ();
let _28: ();
{
_2 = [5051707648210565194_i64,(-6681474142521211817_i64),5390758372437216644_i64,(-5114982727563290936_i64),1236098109211727047_i64];
_2 = [(-3823532143459195589_i64),(-3889274035300296426_i64),3360687616703114437_i64,(-7203146226592563857_i64),(-2000925969110383811_i64)];
_3 = 25537_u16 as i8;
_1 = [6911770901352994315_i64,7086831094854887071_i64,(-7311411427654401200_i64),6879235798732547491_i64];
Call(RET = core::intrinsics::transmute(_3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = 4359590141118885170_i64 as i8;
RET = _3 + _3;
_4 = 60_u8 as f32;
_4 = 48247023864179382384729945014001355350_u128 as f32;
_1 = [(-9054569130825190803_i64),(-5694439340375718659_i64),1804133905606202663_i64,561698945707639418_i64];
_4 = (-316590017_i32) as f32;
RET = _3;
Goto(bb2)
}
bb2 = {
_5 = Adt48::Variant2 { fld0: (-6228_i16) };
_1 = [(-4630036513891414197_i64),(-4711721833176961023_i64),(-7777056284180631754_i64),27969141247377981_i64];
place!(Field::<i16>(Variant(_5, 2), 0)) = 12976_i16 & (-20702_i16);
_2 = [3952681818632857221_i64,6946985828518064492_i64,5155437004768310071_i64,(-2195114686773814151_i64),6450670528031973245_i64];
_3 = RET;
_1 = [(-1737018590032535422_i64),(-8442669050018333229_i64),9186552225338607238_i64,(-682635599774770430_i64)];
_4 = 1543483149_i32 as f32;
place!(Field::<i16>(Variant(_5, 2), 0)) = 24045_i16;
SetDiscriminant(_5, 1);
place!(Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4)).4 = _4;
place!(Field::<[i32; 1]>(Variant(_5, 1), 5)) = [(-1002137541_i32)];
place!(Field::<([i64; 4], u128, *mut f32, f64)>(Variant(_5, 1), 3)).0 = _1;
place!(Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4)).1 = 3234_u16;
place!(Field::<([i64; 4], u128, *mut f32, f64)>(Variant(_5, 1), 3)).3 = 6942330439836718411980612284810204209_u128 as f64;
place!(Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4)).3 = 2307202777_u32;
RET = _3;
place!(Field::<([i64; 4], u128, *mut f32, f64)>(Variant(_5, 1), 3)).3 = (-2529_i16) as f64;
place!(Field::<[i32; 1]>(Variant(_5, 1), 5)) = [1419963366_i32];
place!(Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4)).5 = core::ptr::addr_of_mut!(_4);
place!(Field::<([i64; 4], u128, *mut f32, f64)>(Variant(_5, 1), 3)).1 = !79461514852342485340509268069013729401_u128;
place!(Field::<([i64; 4], u128, *mut f32, f64)>(Variant(_5, 1), 3)).2 = core::ptr::addr_of_mut!(_4);
place!(Field::<([i64; 4], u128, *mut f32, f64)>(Variant(_5, 1), 3)).1 = RET as u128;
Goto(bb3)
}
bb3 = {
place!(Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4)).4 = -_4;
place!(Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4)) = (Field::<[i32; 1]>(Variant(_5, 1), 5), 63917_u16, 3505231950049110719_usize, 3755014839_u32, _4, Field::<([i64; 4], u128, *mut f32, f64)>(Variant(_5, 1), 3).2);
Goto(bb4)
}
bb4 = {
place!(Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4)).0 = [1362154117_i32];
Goto(bb5)
}
bb5 = {
place!(Field::<*mut [u128; 5]>(Variant(_5, 1), 1)) = core::ptr::addr_of_mut!(_9);
_6 = Field::<([i64; 4], u128, *mut f32, f64)>(Variant(_5, 1), 3).1 & Field::<([i64; 4], u128, *mut f32, f64)>(Variant(_5, 1), 3).1;
_6 = 8565160675582261222377414222868927035_i128 as u128;
_9 = [_6,Field::<([i64; 4], u128, *mut f32, f64)>(Variant(_5, 1), 3).1,Field::<([i64; 4], u128, *mut f32, f64)>(Variant(_5, 1), 3).1,_6,_6];
place!(Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4)).4 = _4 * _4;
_3 = 5153513329774946072_u64 as i8;
place!(Field::<([i64; 4], u128, *mut f32, f64)>(Variant(_5, 1), 3)).1 = 7870_i16 as u128;
place!(Field::<[usize; 4]>(Variant(_5, 1), 6)) = [Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4).2,Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4).2,Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4).2,Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4).2];
_1 = Field::<([i64; 4], u128, *mut f32, f64)>(Variant(_5, 1), 3).0;
place!(Field::<[i32; 1]>(Variant(_5, 1), 5)) = [392317158_i32];
_8 = Adt48::Variant2 { fld0: 18725_i16 };
place!(Field::<i16>(Variant(_8, 2), 0)) = (-23541_i16);
_9 = [Field::<([i64; 4], u128, *mut f32, f64)>(Variant(_5, 1), 3).1,Field::<([i64; 4], u128, *mut f32, f64)>(Variant(_5, 1), 3).1,Field::<([i64; 4], u128, *mut f32, f64)>(Variant(_5, 1), 3).1,_6,_6];
place!(Field::<([i64; 4], u128, *mut f32, f64)>(Variant(_5, 1), 3)).2 = core::ptr::addr_of_mut!(place!(Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4)).4);
place!(Field::<([i64; 4], u128, *mut f32, f64)>(Variant(_5, 1), 3)).2 = Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4).5;
_13 = [Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4).3,Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4).3,Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4).3];
place!(Field::<([i64; 4], u128, *mut f32, f64)>(Variant(_5, 1), 3)).3 = Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4).1 as f64;
Goto(bb6)
}
bb6 = {
place!(Field::<[i32; 1]>(Variant(_5, 1), 5)) = [861687049_i32];
RET = Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4).1 as i8;
_5 = Move(_8);
RET = -_3;
place!(Field::<i16>(Variant(_5, 2), 0)) = 1661_u16 as i16;
Call(_3 = fn13(_2, _1, _2, _1, _13, _1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_10 = false as u8;
_12 = 23793_u16 as f64;
_12 = 1914260643_i32 as f64;
_1 = [(-7136114479342015813_i64),(-5217172372252321957_i64),(-4332530808392157912_i64),(-8634587366949239842_i64)];
_11 = [(-1208565121074782270_i64),8122892800631183185_i64,7823414516984264752_i64,2785732481797547997_i64,551016878489290684_i64];
_7 = [_6,_6,_6,_6,_6];
_15 = (-4_isize) | 9223372036854775807_isize;
_1 = [(-4789260044285642686_i64),1486089908709453357_i64,(-2604212200129681570_i64),(-1777759629706758609_i64)];
RET = _3;
RET = -_3;
_9 = [_6,_6,_6,_6,_6];
_11 = [(-2895332304991745553_i64),5622196143128538044_i64,(-4869560216733120364_i64),(-1492981914143394875_i64),4701991240164849278_i64];
_9 = _7;
_12 = _10 as f64;
_2 = [4884560618379664912_i64,5761246732347045275_i64,3144829369002476976_i64,7047100264312720139_i64,(-6267754974631892117_i64)];
match _3 {
0 => bb4,
1 => bb2,
340282366920938463463374607431768211365 => bb8,
_ => bb6
}
}
bb8 = {
_19 = _12 as i128;
_1 = [(-6251397440340159525_i64),(-4482835394139649098_i64),3684281874008297783_i64,771875852667374263_i64];
_18 = 2011758083125980684_u64;
_8 = Move(_5);
match _18 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb5,
5 => bb9,
6 => bb10,
2011758083125980684 => bb12,
_ => bb11
}
}
bb9 = {
_10 = false as u8;
_12 = 23793_u16 as f64;
_12 = 1914260643_i32 as f64;
_1 = [(-7136114479342015813_i64),(-5217172372252321957_i64),(-4332530808392157912_i64),(-8634587366949239842_i64)];
_11 = [(-1208565121074782270_i64),8122892800631183185_i64,7823414516984264752_i64,2785732481797547997_i64,551016878489290684_i64];
_7 = [_6,_6,_6,_6,_6];
_15 = (-4_isize) | 9223372036854775807_isize;
_1 = [(-4789260044285642686_i64),1486089908709453357_i64,(-2604212200129681570_i64),(-1777759629706758609_i64)];
RET = _3;
RET = -_3;
_9 = [_6,_6,_6,_6,_6];
_11 = [(-2895332304991745553_i64),5622196143128538044_i64,(-4869560216733120364_i64),(-1492981914143394875_i64),4701991240164849278_i64];
_9 = _7;
_12 = _10 as f64;
_2 = [4884560618379664912_i64,5761246732347045275_i64,3144829369002476976_i64,7047100264312720139_i64,(-6267754974631892117_i64)];
match _3 {
0 => bb4,
1 => bb2,
340282366920938463463374607431768211365 => bb8,
_ => bb6
}
}
bb10 = {
place!(Field::<[i32; 1]>(Variant(_5, 1), 5)) = [861687049_i32];
RET = Field::<([i32; 1], u16, usize, u32, f32, *mut f32)>(Variant(_5, 1), 4).1 as i8;
_5 = Move(_8);
RET = -_3;
place!(Field::<i16>(Variant(_5, 2), 0)) = 1661_u16 as i16;
Call(_3 = fn13(_2, _1, _2, _1, _13, _1), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
_3 = 4359590141118885170_i64 as i8;
RET = _3 + _3;
_4 = 60_u8 as f32;
_4 = 48247023864179382384729945014001355350_u128 as f32;
_1 = [(-9054569130825190803_i64),(-5694439340375718659_i64),1804133905606202663_i64,561698945707639418_i64];
_4 = (-316590017_i32) as f32;
RET = _3;
Goto(bb2)
}
bb12 = {
_18 = !4244906255778885765_u64;
_4 = _15 as f32;
_5 = Move(_8);
_3 = -RET;
RET = _3 << _6;
_8 = Move(_5);
SetDiscriminant(_8, 0);
_2 = [5937914583204315505_i64,(-7318361575379131357_i64),(-764172387760211875_i64),(-3117482032080834533_i64),(-1853977613349112165_i64)];
RET = 1700983260_u32 as i8;
_13 = [2639961985_u32,3517809018_u32,2218306830_u32];
RET = _10 as i8;
RET = _6 as i8;
RET = !_3;
_12 = 8074925510304240088_usize as f64;
_12 = (-1302_i16) as f64;
_11 = [7421169512034526394_i64,5523339666962891169_i64,(-5904169442894873871_i64),(-1741452551096204085_i64),(-454085585117773123_i64)];
_9 = _7;
place!(Field::<bool>(Variant(_8, 0), 0)) = false;
_10 = _18 as u8;
Goto(bb13)
}
bb13 = {
_4 = 15937676321478113916_usize as f32;
place!(Field::<bool>(Variant(_8, 0), 0)) = _12 > _12;
_4 = _18 as f32;
Goto(bb14)
}
bb14 = {
_19 = (-121466780753710420128606278887106416137_i128);
_5 = Adt48::Variant2 { fld0: 27013_i16 };
place!(Field::<*const i16>(Variant(_8, 0), 1)) = core::ptr::addr_of!(place!(Field::<i16>(Variant(_5, 2), 0)));
_5 = Adt48::Variant2 { fld0: (-22261_i16) };
_1 = [(-4876685173890083450_i64),(-3930088943792129038_i64),7611804568817756977_i64,(-601229473700911229_i64)];
_1 = [(-7927535536730183840_i64),(-363052744697527739_i64),1867617158081363588_i64,(-8907706947839690509_i64)];
_7 = [_6,_6,_6,_6,_6];
_24 = [(-1242325449_i32)];
_10 = !195_u8;
_12 = _10 as f64;
place!(Field::<bool>(Variant(_8, 0), 0)) = !true;
place!(Field::<i16>(Variant(_5, 2), 0)) = (-5222_i16) << _15;
_23 = Adt43::Variant0 { fld0: Field::<bool>(Variant(_8, 0), 0),fld1: _4 };
_21 = Field::<f32>(Variant(_23, 0), 1);
_3 = !RET;
_10 = 1073382316_i32 as u8;
_4 = Field::<f32>(Variant(_23, 0), 1) - Field::<f32>(Variant(_23, 0), 1);
_4 = 55436_u16 as f32;
_6 = _21 as u128;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(12_usize, 24_usize, Move(_24), 3_usize, Move(_3), 7_usize, Move(_7), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(12_usize, 15_usize, Move(_15), 18_usize, Move(_18), 28_usize, _28, 28_usize, _28), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: [i64; 5],mut _2: [i64; 4],mut _3: [i64; 5],mut _4: [i64; 4],mut _5: [u32; 3],mut _6: [i64; 4]) -> i8 {
mir! {
type RET = i8;
let _7: f32;
let _8: f32;
let _9: isize;
let _10: i64;
let _11: i64;
let _12: bool;
let _13: [usize; 6];
let _14: i32;
let _15: bool;
let _16: [usize; 4];
let _17: isize;
let _18: [i32; 1];
let _19: [i64; 5];
let _20: [u32; 3];
let _21: isize;
let _22: [u32; 3];
let _23: [i32; 1];
let _24: ([u128; 5], usize, *mut f32, bool, *mut [u128; 5]);
let _25: [isize; 4];
let _26: [i64; 4];
let _27: [u128; 2];
let _28: [isize; 3];
let _29: Adt52;
let _30: f32;
let _31: ([u128; 5], usize, *mut f32, bool, *mut [u128; 5]);
let _32: i16;
let _33: [i64; 4];
let _34: Adt48;
let _35: (*mut u64,);
let _36: f64;
let _37: i8;
let _38: char;
let _39: [i64; 5];
let _40: ([i32; 1], u16, usize, u32, f32, *mut f32);
let _41: bool;
let _42: f32;
let _43: i8;
let _44: Adt51;
let _45: i8;
let _46: bool;
let _47: char;
let _48: char;
let _49: ();
let _50: ();
{
_6 = _2;
_6 = [808768887032821498_i64,7030467772655726633_i64,4876986912022977568_i64,(-3141754006838138102_i64)];
RET = 432042619_i32 as i8;
_3 = [5839887108415536934_i64,(-7703898846538879005_i64),(-4452213096740095278_i64),7793329469795037231_i64,(-2627138993025260651_i64)];
RET = 120_i8;
RET = !61_i8;
_4 = [(-2850548313596273229_i64),4748381854661361004_i64,1345850468643418296_i64,(-3685417352600392748_i64)];
_1 = [(-4811307954120978551_i64),(-2868590630176750982_i64),4871867504225110389_i64,4421616530394063760_i64,5780436191786396947_i64];
_7 = 6967585665791339411_u64 as f32;
_1 = _3;
_4 = [4103932171175120112_i64,(-860333047518600177_i64),(-2609268050164653559_i64),3332670571338210114_i64];
_2 = [(-7270880176171588258_i64),(-7466905549351187271_i64),(-3173337106474484620_i64),(-3243554054538237434_i64)];
_2 = [(-416296321319772346_i64),(-2484995112343731962_i64),(-3904408906554625077_i64),(-2807899313439982714_i64)];
RET = 5_usize as i8;
_5 = [1911507355_u32,3124912642_u32,4112990567_u32];
_4 = [(-6065956714853886973_i64),(-4607449310060421539_i64),7340116339908803013_i64,(-1824579379231393527_i64)];
_7 = 2391205038551536374_i64 as f32;
_8 = 105_u8 as f32;
RET = (-16_i8) << 31_u8;
_6 = _4;
_3 = [1767653780947761267_i64,(-697180075455387336_i64),4118855902728380837_i64,6810891335662959254_i64,(-4980378290331114381_i64)];
_5 = [2768082610_u32,74767586_u32,4000402677_u32];
_8 = _7;
_8 = -_7;
_5 = [3232833389_u32,3017713146_u32,2927091449_u32];
_3 = [(-2194116731789553533_i64),(-7815131587888359971_i64),(-3263071111680976366_i64),9124177075840809594_i64,(-8728006222865597996_i64)];
RET = 1000811142_i32 as i8;
_3 = [(-8853213983878887794_i64),(-4318316609212008517_i64),7540234829320300646_i64,(-1839288024020503900_i64),(-6705819078952197974_i64)];
_7 = _8;
Goto(bb1)
}
bb1 = {
_6 = [(-5088189755309144588_i64),(-8117540877506932352_i64),(-5004384766498960711_i64),3522396838314184501_i64];
_2 = [1908430744496961137_i64,(-4857214433496735354_i64),(-8815188871956722140_i64),(-7733456005560404849_i64)];
_3 = _1;
_5 = [2599421722_u32,3216373013_u32,3097146405_u32];
_8 = -_7;
_1 = _3;
_6 = [8083437901516764870_i64,3163380611691217512_i64,2924874158773417921_i64,(-692949716185477872_i64)];
Goto(bb2)
}
bb2 = {
_3 = [(-555642976359262288_i64),2835189287636869722_i64,(-2130058457238781793_i64),8771408189648530392_i64,(-7550940395883000579_i64)];
_1 = [7531696965457165153_i64,(-376780807917735520_i64),8339338167029761126_i64,(-8497678037459208826_i64),8011197280088061167_i64];
_4 = [(-649060765234212556_i64),(-519313502767380410_i64),2900392496795968142_i64,(-6576560239765642295_i64)];
_2 = [5828457663271718676_i64,(-6977599468209857557_i64),(-8552136765839665998_i64),(-7524231215933506350_i64)];
RET = -(-39_i8);
_4 = [(-8868115124887555351_i64),2334122916070072920_i64,9105009574399664362_i64,9120913031856610470_i64];
_7 = _8;
_3 = _1;
_7 = -_8;
RET = 53_i8;
_7 = -_8;
_6 = [7905766862112765257_i64,3256877792052145381_i64,5148759743546602518_i64,(-2313023704061141590_i64)];
_7 = -_8;
RET = 111_i8;
_9 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_4 = _2;
_5 = [2083528977_u32,725486816_u32,2509425037_u32];
_11 = -1794254362395738092_i64;
Call(_2 = fn14(_6, _8, _1, _9, _4, _5, RET, _1, _3, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10 = 2698211566_u32 as i64;
RET = 2_i8 + 0_i8;
_11 = false as i64;
_11 = _10;
_12 = !false;
_1 = [_11,_10,_11,_11,_11];
_1 = [_10,_11,_10,_10,_11];
_6 = [_10,_11,_11,_11];
RET = 51_i8;
_14 = (-1898816697_i32);
RET = (-10_i8);
_15 = _9 != _9;
_8 = _7 * _7;
_1 = _3;
_8 = -_7;
RET = 296665954936997607887351933319112226946_u128 as i8;
RET = _10 as i8;
RET = -(-6_i8);
_4 = [_10,_10,_10,_10];
_5 = [2279936220_u32,4285756194_u32,985823604_u32];
_3 = _1;
_14 = 1799582953_i32 & (-881400477_i32);
RET = 14183_u16 as i8;
_7 = _8;
_13 = [0_usize,4405365743100360821_usize,1_usize,17302561401163737548_usize,5_usize,6472666344254761499_usize];
Goto(bb4)
}
bb4 = {
_21 = _9;
_19 = [_10,_10,_10,_10,_11];
RET = (-75_i8) + (-32_i8);
_4 = [_11,_11,_11,_10];
_19 = [_10,_11,_11,_10,_11];
_14 = -(-492404266_i32);
_11 = 5157392244895968136_u64 as i64;
_11 = _10 | _10;
RET = 61_i8 - (-35_i8);
_12 = _15;
_22 = [2528872476_u32,3525835265_u32,3193404884_u32];
_15 = !_12;
RET = _9 as i8;
_13 = [3435545982773434610_usize,4_usize,4_usize,3_usize,4_usize,18316958435023239797_usize];
_8 = _7 * _7;
_24.3 = _15 | _15;
_3 = [_10,_10,_10,_11,_10];
_17 = _9;
_11 = _10;
_24.4 = core::ptr::addr_of_mut!(_24.0);
Goto(bb5)
}
bb5 = {
_18 = [_14];
_20 = [7159656_u32,2967634872_u32,1268751990_u32];
_13 = [1_usize,8044327419425940398_usize,7_usize,2_usize,1_usize,2620377956138686631_usize];
_16 = [3_usize,4_usize,10171387253825596381_usize,0_usize];
_11 = _10;
_25 = [_21,_9,_21,_9];
_3 = _1;
_11 = 5321_i16 as i64;
Goto(bb6)
}
bb6 = {
_5 = _22;
_28 = [_17,_21,_21];
_21 = _9;
_26 = _2;
_24.0 = [30216062620135966640148494421912765742_u128,38149346970535475943614040675278321804_u128,142890878580055891587274118389464440357_u128,328913647983113515781409779022958921599_u128,28283283203994559906886610365420759941_u128];
_26 = [_10,_11,_10,_10];
_1 = _3;
_24.3 = _15;
_24.2 = core::ptr::addr_of_mut!(_7);
_24.2 = core::ptr::addr_of_mut!(_8);
_18 = [_14];
_24.1 = 21170_u16 as usize;
_15 = !_12;
Goto(bb7)
}
bb7 = {
_5 = [1686491358_u32,2776860214_u32,3448425139_u32];
_9 = !_17;
_4 = _2;
_14 = 884636213_i32;
RET = !(-10_i8);
_18 = [_14];
RET = (-82_i8) & 36_i8;
RET = _14 as i8;
_30 = _7 - _7;
_23 = _18;
_14 = (-191707448_i32) * (-1762128313_i32);
_4 = _6;
_10 = _11;
_3 = [_11,_11,_11,_11,_11];
_6 = [_10,_10,_11,_10];
_13 = [_24.1,_24.1,_24.1,_24.1,_24.1,_24.1];
_16 = [_24.1,_24.1,_24.1,_24.1];
_24.2 = core::ptr::addr_of_mut!(_8);
_4 = _2;
_3 = [_11,_11,_11,_11,_11];
_23 = [_14];
_4 = _2;
_24.3 = !_12;
_6 = [_10,_11,_10,_11];
_27 = [218956083104888007427545738576419594297_u128,167329016083079171748737858824377043886_u128];
_3 = [_10,_10,_11,_11,_11];
_9 = !_17;
_30 = (-122939976608890742054136205789256318827_i128) as f32;
_31.3 = !_12;
Goto(bb8)
}
bb8 = {
_31 = (_24.0, _24.1, _24.2, _15, _24.4);
_20 = [1754175806_u32,1239450188_u32,984535028_u32];
_24.4 = core::ptr::addr_of_mut!(_31.0);
_26 = _4;
_14 = _15 as i32;
_4 = _2;
_31 = _24;
_22 = [2885613837_u32,3752701377_u32,771841480_u32];
_24.4 = _31.4;
_27 = [177372164354683621530172246343060173510_u128,56804605824239572054739503228823119709_u128];
Goto(bb9)
}
bb9 = {
_26 = [_11,_11,_11,_11];
_32 = _11 as i16;
_31.2 = core::ptr::addr_of_mut!(_8);
_32 = (-9211_i16) >> _9;
_6 = _26;
_7 = _8;
_8 = -_30;
_31.1 = _24.1;
RET = 51_i8;
_31.3 = _15;
_25 = [_9,_9,_17,_17];
_31.3 = _15 ^ _12;
_34 = Adt48::Variant2 { fld0: _32 };
_31.2 = core::ptr::addr_of_mut!(_8);
_13 = [_31.1,_24.1,_24.1,_24.1,_24.1,_24.1];
_27 = [57217326716512229256983805980150986139_u128,251602734193798714658004670690183267364_u128];
_20 = [2586272995_u32,3978100370_u32,3318639417_u32];
_13 = [_24.1,_24.1,_31.1,_31.1,_31.1,_31.1];
RET = (-91_i8);
SetDiscriminant(_34, 0);
_31 = (_24.0, _24.1, _24.2, _12, _24.4);
_17 = _21;
_13 = [_24.1,_24.1,_31.1,_24.1,_31.1,_31.1];
_17 = !_21;
_7 = -_8;
_27 = [5031937416127129891525834336673482976_u128,97429240576880688431552471371777885032_u128];
match RET {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463463374607431768211365 => bb10,
_ => bb6
}
}
bb10 = {
_31.3 = _9 >= _21;
_31.3 = _24.3 ^ _15;
_1 = [_11,_11,_10,_11,_11];
_6 = [_11,_10,_11,_11];
_24 = (_31.0, _31.1, _31.2, _12, _31.4);
_16 = [_24.1,_24.1,_31.1,_31.1];
_30 = _8;
_9 = _21;
Goto(bb11)
}
bb11 = {
_38 = '\u{b36a7}';
place!(Field::<bool>(Variant(_34, 0), 0)) = !_31.3;
_10 = -_11;
_16 = [_31.1,_24.1,_24.1,_31.1];
_40.3 = 914531548_u32;
_40.0 = _23;
place!(Field::<*const i16>(Variant(_34, 0), 1)) = core::ptr::addr_of!(_32);
_32 = !3844_i16;
_26 = [_10,_11,_10,_10];
_18 = [_14];
_15 = _31.3 == Field::<bool>(Variant(_34, 0), 0);
_21 = _9 & _17;
_31.1 = 37599006086551028286335266961551639298_u128 as usize;
Goto(bb12)
}
bb12 = {
_31 = (_24.0, _24.1, _24.2, Field::<bool>(Variant(_34, 0), 0), _24.4);
_37 = RET;
_40.0 = [_14];
place!(Field::<*const i16>(Variant(_34, 0), 1)) = core::ptr::addr_of!(_32);
_24.4 = _31.4;
_5 = _22;
_40.1 = 29049_u16 - 30837_u16;
_22 = [_40.3,_40.3,_40.3];
_36 = (-95215284584373305806927107777941356099_i128) as f64;
RET = _14 as i8;
_24.1 = _31.1 + _31.1;
_31.1 = !_24.1;
_43 = !RET;
_24 = _31;
_31.4 = _24.4;
_36 = _17 as f64;
_31.3 = _15;
_41 = _40.1 < _40.1;
match _37 {
0 => bb13,
340282366920938463463374607431768211365 => bb15,
_ => bb14
}
}
bb13 = {
_6 = [(-5088189755309144588_i64),(-8117540877506932352_i64),(-5004384766498960711_i64),3522396838314184501_i64];
_2 = [1908430744496961137_i64,(-4857214433496735354_i64),(-8815188871956722140_i64),(-7733456005560404849_i64)];
_3 = _1;
_5 = [2599421722_u32,3216373013_u32,3097146405_u32];
_8 = -_7;
_1 = _3;
_6 = [8083437901516764870_i64,3163380611691217512_i64,2924874158773417921_i64,(-692949716185477872_i64)];
Goto(bb2)
}
bb14 = {
_31 = (_24.0, _24.1, _24.2, _15, _24.4);
_20 = [1754175806_u32,1239450188_u32,984535028_u32];
_24.4 = core::ptr::addr_of_mut!(_31.0);
_26 = _4;
_14 = _15 as i32;
_4 = _2;
_31 = _24;
_22 = [2885613837_u32,3752701377_u32,771841480_u32];
_24.4 = _31.4;
_27 = [177372164354683621530172246343060173510_u128,56804605824239572054739503228823119709_u128];
Goto(bb9)
}
bb15 = {
_43 = _32 as i8;
_40.4 = RET as f32;
_17 = !_9;
_40.1 = !43946_u16;
_24.0 = _31.0;
_23 = [_14];
_8 = _40.4;
_10 = _11;
_5 = [_40.3,_40.3,_40.3];
_31.0 = _24.0;
_43 = _37;
_31 = _24;
RET = _37;
_8 = -_40.4;
_4 = [_10,_11,_11,_10];
_16 = [_31.1,_24.1,_24.1,_31.1];
_14 = 321327387_i32;
_25 = [_21,_21,_21,_21];
_20 = [_40.3,_40.3,_40.3];
Goto(bb16)
}
bb16 = {
Call(_49 = dump_var(13_usize, 1_usize, Move(_1), 9_usize, Move(_9), 43_usize, Move(_43), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = dump_var(13_usize, 26_usize, Move(_26), 37_usize, Move(_37), 16_usize, Move(_16), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_49 = dump_var(13_usize, 20_usize, Move(_20), 18_usize, Move(_18), 23_usize, Move(_23), 4_usize, Move(_4)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_49 = dump_var(13_usize, 32_usize, Move(_32), 13_usize, Move(_13), 15_usize, Move(_15), 50_usize, _50), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: [i64; 4],mut _2: f32,mut _3: [i64; 5],mut _4: isize,mut _5: [i64; 4],mut _6: [u32; 3],mut _7: i8,mut _8: [i64; 5],mut _9: [i64; 5],mut _10: [i64; 4]) -> [i64; 4] {
mir! {
type RET = [i64; 4];
let _11: ([i64; 4], u128, *mut f32, f64);
let _12: f32;
let _13: u64;
let _14: isize;
let _15: char;
let _16: [u32; 3];
let _17: u32;
let _18: [u128; 2];
let _19: [isize; 3];
let _20: u8;
let _21: Adt53;
let _22: isize;
let _23: i8;
let _24: u16;
let _25: Adt49;
let _26: [isize; 3];
let _27: isize;
let _28: [i32; 6];
let _29: i128;
let _30: isize;
let _31: i128;
let _32: (u128, i8, [i64; 5]);
let _33: Adt42;
let _34: (*mut u64,);
let _35: [i32; 1];
let _36: [char; 3];
let _37: isize;
let _38: (u128, i8, [i64; 5]);
let _39: ([u128; 5], usize, *mut f32, bool, *mut [u128; 5]);
let _40: [usize; 4];
let _41: (*mut u64,);
let _42: char;
let _43: [isize; 4];
let _44: [i32; 1];
let _45: [isize; 4];
let _46: char;
let _47: f64;
let _48: [u32; 3];
let _49: i32;
let _50: *mut [u128; 5];
let _51: i16;
let _52: bool;
let _53: ();
let _54: ();
{
_6 = [3965573901_u32,2821072176_u32,2183812923_u32];
_4 = _2 as isize;
RET = [3014925996574041482_i64,(-1700741319956157785_i64),(-6675549353584182948_i64),(-1634191765403370135_i64)];
_11.3 = 26406_u16 as f64;
RET = [(-3661248022261833817_i64),(-935877002172265716_i64),3123059822701860902_i64,1200896574454926713_i64];
_12 = (-1128517828_i32) as f32;
Goto(bb1)
}
bb1 = {
_12 = _2;
_18 = [272458042056564461495446849459162308630_u128,244520347250991501807866241643659109325_u128];
_7 = _11.3 as i8;
_13 = !5661040431716281072_u64;
_8 = _3;
Call(_17 = core::intrinsics::bswap(769751193_u32), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_16 = [200482320_u32,2881910846_u32,644468767_u32];
_11.1 = (-2138825904572054976_i64) as u128;
_15 = '\u{101c71}';
_8 = [2408872822809827554_i64,6710594370838911293_i64,1623890656547255172_i64,8945567192755579255_i64,(-1053655696817879635_i64)];
_8 = [(-1633542445772999502_i64),(-1722515933610844633_i64),(-7353989820998872839_i64),(-5313701954265906535_i64),(-2686823828059039916_i64)];
_22 = _4 | _4;
_6 = [1046351376_u32,3307766500_u32,4261617079_u32];
RET = _1;
_4 = -_22;
_20 = _15 as u8;
_13 = !1764949103613267421_u64;
_19 = [_22,_4,_4];
_22 = !_4;
_3 = [5820875062771199011_i64,(-1157962327682117036_i64),6314669235804215082_i64,7174769359898297320_i64,(-8424522283063994213_i64)];
RET = [382525654731323800_i64,4843678209540106802_i64,(-5847741347064573012_i64),1487251563388607353_i64];
RET = [3084487475563974711_i64,7152693219547388152_i64,1725570628966954401_i64,(-942257240864265200_i64)];
_18 = [_11.1,_11.1];
_8 = [(-4951417843454594957_i64),2433866825358131958_i64,8629547674690294796_i64,3433324397105078936_i64,1412782420564691135_i64];
_18 = [_11.1,_11.1];
_23 = 3_usize as i8;
_16 = _6;
_16 = [3974758529_u32,2048551475_u32,1519518951_u32];
_16 = [1012059851_u32,1581959350_u32,1732889040_u32];
_17 = 3795644879_u32 | 3829607000_u32;
_5 = _1;
Goto(bb3)
}
bb3 = {
_23 = _7 << _4;
_8 = [9022271292813388777_i64,(-8120689238704386555_i64),2217376912795944874_i64,(-1947961810989515790_i64),(-6818495460766349073_i64)];
_11.3 = _2 as f64;
_18 = [_11.1,_11.1];
_15 = '\u{ec776}';
_16 = [_17,_17,_17];
_22 = (-1867_i16) as isize;
RET = [2401246370780790044_i64,2733055018584173106_i64,210621847504725604_i64,487975276191350625_i64];
Goto(bb4)
}
bb4 = {
_5 = [(-5643536751082424253_i64),8567744709137175319_i64,(-7395264655614010690_i64),(-6146685172696327576_i64)];
_14 = (-25558_i16) as isize;
_7 = _23;
_16 = [_17,_17,_17];
_8 = [4579168688552215298_i64,718313267467866502_i64,5031564025343557237_i64,6830156455364842324_i64,9178039857372938200_i64];
_11.0 = [1463715600085499693_i64,(-7161267375891882788_i64),(-3877461973544757816_i64),2685025471787883376_i64];
_2 = _12 * _12;
_22 = !_4;
_3 = [463833570484415262_i64,7355654005727935972_i64,(-2322484257525181937_i64),6295420716688493581_i64,2216466578370700036_i64];
_11.2 = core::ptr::addr_of_mut!(_12);
_5 = [2025976341727334203_i64,(-3234667913618508448_i64),(-7440262738645487831_i64),5932242162403785487_i64];
_19 = [_22,_22,_4];
Goto(bb5)
}
bb5 = {
_1 = [(-4408815862097077200_i64),4782805841437598443_i64,6587101186273752589_i64,4950603960163554262_i64];
_9 = [(-300015832963747237_i64),(-4140045074433238662_i64),(-8622177762925556063_i64),(-1821349274298699909_i64),(-7120876040601781307_i64)];
_2 = _23 as f32;
_17 = !3899894045_u32;
_2 = _12;
_16 = [_17,_17,_17];
Goto(bb6)
}
bb6 = {
_11.0 = _1;
Goto(bb7)
}
bb7 = {
_14 = _4;
_17 = !587250439_u32;
_14 = _4 >> _22;
_16 = _6;
_26 = _19;
_32.1 = -_23;
Call(_32.1 = core::intrinsics::bswap(_23), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_3 = _8;
_15 = '\u{106e3e}';
_31 = 4_usize as i128;
_32.0 = _11.1 * _11.1;
_33.fld0 = [15579771783527341741_usize,8775058678528897337_usize,7884410186929789984_usize,15931731719830400851_usize];
_28 = [(-623537200_i32),(-1019813455_i32),(-1348335687_i32),(-527293995_i32),(-49806982_i32),(-967917056_i32)];
_16 = [_17,_17,_17];
_31 = 5_usize as i128;
_27 = 18256445272157057904_usize as isize;
_32.2 = _3;
_22 = _17 as isize;
_34.0 = core::ptr::addr_of_mut!(_13);
_24 = 44943_u16 - 3546_u16;
Goto(bb9)
}
bb9 = {
_38.0 = _32.0;
Goto(bb10)
}
bb10 = {
_38 = (_32.0, _7, _32.2);
_39.2 = core::ptr::addr_of_mut!(_2);
_23 = _15 as i8;
_40 = [0_usize,16897503804591326222_usize,2_usize,4_usize];
_24 = 2233_u16 - 2949_u16;
_30 = _4 - _14;
_39.0 = [_11.1,_38.0,_38.0,_32.0,_38.0];
_28 = [1313080480_i32,(-1838668654_i32),(-1770180004_i32),(-65991614_i32),612185409_i32,839366775_i32];
_37 = _30 * _4;
_8 = _32.2;
_44 = [(-1343380212_i32)];
_32.1 = _7 - _7;
_13 = _2 as u64;
_39.4 = core::ptr::addr_of_mut!(_39.0);
RET = [(-4862132779113349063_i64),(-5249796591274749672_i64),3289527931788691036_i64,258867160782535931_i64];
_10 = [(-8181600867505981545_i64),7157204831608542335_i64,6000473435633636335_i64,(-5066656987423599832_i64)];
_41 = _34;
_2 = _12 * _12;
_38.1 = !_7;
_34.0 = core::ptr::addr_of_mut!(_13);
_29 = _31;
_5 = _1;
_16 = _6;
_11.1 = _38.0;
Goto(bb11)
}
bb11 = {
_22 = _4;
_1 = [150471084924693835_i64,2747348591001572863_i64,(-4516841793689245411_i64),5393530057726792887_i64];
_11.2 = _39.2;
_45 = [_30,_22,_37,_14];
Call(_31 = fn15(_11, _38, _28, _41.0, _11, _14, _39.2, _10), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_6 = _16;
_43 = _45;
_47 = -_11.3;
_22 = 6540985084273979031_i64 as isize;
_46 = _15;
_38.2 = _3;
_39.3 = false;
_23 = _24 as i8;
_39.0 = [_38.0,_11.1,_11.1,_11.1,_11.1];
Goto(bb13)
}
bb13 = {
_11.2 = _39.2;
_39.1 = 0_usize;
_46 = _15;
_30 = _27 | _14;
_48 = [_17,_17,_17];
RET = _10;
_49 = _2 as i32;
_29 = _31 << _49;
_14 = -_30;
_35 = [_49];
_48 = [_17,_17,_17];
Goto(bb14)
}
bb14 = {
_32 = _38;
_48 = _6;
_15 = _46;
_15 = _46;
_22 = -_37;
_10 = _1;
_40 = _33.fld0;
_13 = !10220886699959927_u64;
_11.2 = core::ptr::addr_of_mut!(_12);
_13 = 6455778956254539702_u64 >> _29;
_39.2 = core::ptr::addr_of_mut!(_2);
_33.fld0 = [_39.1,_39.1,_39.1,_39.1];
_1 = [(-4932438102464022487_i64),(-8974433072174572128_i64),7400125281590077638_i64,2107805760299888522_i64];
_32.2 = _3;
_42 = _46;
_49 = (-304908629_i32);
_11.0 = _10;
_39.3 = !false;
_32.2 = [5734154576039401214_i64,(-7351965216812093826_i64),3112293083419787870_i64,(-3267458155455741699_i64),(-3814509221022377964_i64)];
_41.0 = core::ptr::addr_of_mut!(_13);
_5 = _11.0;
Goto(bb15)
}
bb15 = {
Call(_53 = dump_var(14_usize, 8_usize, Move(_8), 17_usize, Move(_17), 10_usize, Move(_10), 37_usize, Move(_37)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_53 = dump_var(14_usize, 44_usize, Move(_44), 32_usize, Move(_32), 27_usize, Move(_27), 46_usize, Move(_46)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_53 = dump_var(14_usize, 43_usize, Move(_43), 20_usize, Move(_20), 5_usize, Move(_5), 48_usize, Move(_48)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_53 = dump_var(14_usize, 22_usize, Move(_22), 7_usize, Move(_7), 3_usize, Move(_3), 26_usize, Move(_26)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_53 = dump_var(14_usize, 49_usize, Move(_49), 19_usize, Move(_19), 30_usize, Move(_30), 54_usize, _54), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: ([i64; 4], u128, *mut f32, f64),mut _2: (u128, i8, [i64; 5]),mut _3: [i32; 6],mut _4: *mut u64,mut _5: ([i64; 4], u128, *mut f32, f64),mut _6: isize,mut _7: *mut f32,mut _8: [i64; 4]) -> i128 {
mir! {
type RET = i128;
let _9: Adt56;
let _10: i32;
let _11: isize;
let _12: [i32; 6];
let _13: bool;
let _14: u32;
let _15: Adt43;
let _16: (&'static u32,);
let _17: isize;
let _18: f32;
let _19: u64;
let _20: bool;
let _21: ([i64; 4], u128, *mut f32, f64);
let _22: (u128, i8, [i64; 5]);
let _23: [char; 3];
let _24: Adt42;
let _25: isize;
let _26: &'static u32;
let _27: f32;
let _28: bool;
let _29: *mut ([i32; 1], u16, usize, u32, f32, *mut f32);
let _30: u64;
let _31: Adt56;
let _32: u32;
let _33: u8;
let _34: &'static u32;
let _35: [char; 3];
let _36: u8;
let _37: [i32; 1];
let _38: ();
let _39: ();
{
_2.2 = [1016919256857228069_i64,458062555260017419_i64,(-9043982876305272954_i64),8242779459045359849_i64,(-2267155320056291517_i64)];
_2.0 = _1.1;
_1.1 = _5.1;
RET = (-113074063874272002121696526829203749889_i128) | 42481394802422353828606283095183389679_i128;
_11 = _6 + _6;
(*_4) = 5342510608420684945_u64 | 3928145578239059977_u64;
_4 = core::ptr::addr_of_mut!((*_4));
_7 = _5.2;
_4 = core::ptr::addr_of_mut!((*_4));
_8 = _1.0;
_5 = (_1.0, _1.1, _7, _1.3);
_11 = _6;
_7 = core::ptr::addr_of_mut!((*_7));
RET = 72419841266791955941582996366349480949_i128 | (-83914940068351746440984658585341125764_i128);
_4 = core::ptr::addr_of_mut!((*_4));
_1 = _5;
_1 = (_8, _5.1, _7, _5.3);
_8 = _5.0;
_1.1 = !_5.1;
_10 = (-985471564_i32);
(*_7) = 3440179754_u32 as f32;
_4 = core::ptr::addr_of_mut!((*_4));
_10 = !826048501_i32;
_2.1 = 63075_u16 as i8;
Goto(bb1)
}
bb1 = {
_11 = (*_4) as isize;
RET = 119_u8 as i128;
_6 = -_11;
_1.2 = core::ptr::addr_of_mut!((*_7));
(*_7) = _10 as f32;
Goto(bb2)
}
bb2 = {
Call(_5.1 = core::intrinsics::bswap(_2.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_4) = !3020152541771689082_u64;
_14 = 91818146_u32 * 3940035899_u32;
_5.3 = _1.3;
_1.2 = core::ptr::addr_of_mut!((*_7));
_13 = true ^ false;
_6 = _11 + _11;
_1.1 = _5.1 | _2.0;
_15 = Adt43::Variant0 { fld0: _13,fld1: (*_7) };
_5 = (_1.0, _1.1, _1.2, _1.3);
_4 = core::ptr::addr_of_mut!((*_4));
_10 = RET as i32;
_5 = (_8, _1.1, _1.2, _1.3);
(*_7) = -Field::<f32>(Variant(_15, 0), 1);
RET = (-39786662323125079974751579194857099835_i128);
place!(Field::<f32>(Variant(_15, 0), 1)) = (*_7) * (*_7);
(*_4) = (*_7) as u64;
(*_4) = !13279516392353808260_u64;
place!(Field::<bool>(Variant(_15, 0), 0)) = _13;
_16.0 = &_14;
_11 = !_6;
_7 = _1.2;
_4 = core::ptr::addr_of_mut!((*_4));
place!(Field::<bool>(Variant(_15, 0), 0)) = _13;
_5.2 = core::ptr::addr_of_mut!(place!(Field::<f32>(Variant(_15, 0), 1)));
_12 = [_10,_10,_10,_10,_10,_10];
_1.0 = [7227687530962383958_i64,(-7335708063069388354_i64),59887871410629305_i64,2394602468009736074_i64];
place!(Field::<f32>(Variant(_15, 0), 1)) = (*_7);
_5.2 = _7;
_15 = Adt43::Variant0 { fld0: _13,fld1: (*_7) };
match RET {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
300495704597813383488623028236911111621 => bb9,
_ => bb8
}
}
bb4 = {
Call(_5.1 = core::intrinsics::bswap(_2.0), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_11 = (*_4) as isize;
RET = 119_u8 as i128;
_6 = -_11;
_1.2 = core::ptr::addr_of_mut!((*_7));
(*_7) = _10 as f32;
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
_1.2 = _7;
_22 = _2;
_1.3 = _5.3 + _5.3;
_1.0 = _5.0;
_19 = (*_4) + (*_4);
(*_7) = Field::<f32>(Variant(_15, 0), 1) * Field::<f32>(Variant(_15, 0), 1);
SetDiscriminant(_15, 0);
Call(_17 = core::intrinsics::transmute(_11), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_5.1 = _2.0;
_4 = core::ptr::addr_of_mut!((*_4));
place!(Field::<f32>(Variant(_15, 0), 1)) = (*_7) * (*_7);
_22.0 = _1.1;
_21.1 = !_5.1;
_4 = core::ptr::addr_of_mut!(_19);
_25 = -_17;
_25 = _17 ^ _11;
(*_7) = Field::<f32>(Variant(_15, 0), 1) - Field::<f32>(Variant(_15, 0), 1);
_2.2 = _22.2;
Call(_2.0 = core::intrinsics::bswap(_21.1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_22.1 = _2.1;
_18 = (*_7);
_8 = [4395155248372709379_i64,(-5161291878839243270_i64),5743580614715050170_i64,5134396291107876074_i64];
_1.0 = [4245408852247783730_i64,(-8354295881304550193_i64),(-1964338518561739773_i64),1883354343691253421_i64];
_7 = _5.2;
_21.3 = _1.3 * _1.3;
_2.2 = [1331562031387365689_i64,8628710986117116953_i64,9185035071679803383_i64,(-119840836786094670_i64),(-5721852452175744801_i64)];
_2.0 = !_22.0;
_1.1 = 268_i16 as u128;
_3 = _12;
(*_4) = 16898705494962096939_u64;
_12 = [_10,_10,_10,_10,_10,_10];
_22.2 = [3142299140103224301_i64,170600685694704925_i64,(-3492937322863472958_i64),8709828846510252654_i64,6654232141743503327_i64];
Goto(bb12)
}
bb12 = {
_20 = _13;
_15 = Adt43::Variant0 { fld0: _20,fld1: (*_7) };
RET = -(-31947951630670151864978891988573500432_i128);
_1.1 = _21.1 & _5.1;
_1.2 = _5.2;
_3 = [_10,_10,_10,_10,_10,_10];
SetDiscriminant(_15, 0);
_15 = Adt43::Variant0 { fld0: _20,fld1: _18 };
_26 = Move(_16.0);
RET = (-59985655824838738946883470898583994062_i128);
_2.1 = _22.1;
_28 = _13;
_21.2 = core::ptr::addr_of_mut!(_27);
_20 = !_13;
_21 = (_8, _22.0, _5.2, _5.3);
Goto(bb13)
}
bb13 = {
_13 = _2.0 != _2.0;
_19 = 16118876563728652972_u64 ^ 12565903742033829197_u64;
_22.1 = _2.1;
(*_7) = -_18;
_22 = (_2.0, _2.1, _2.2);
_27 = _18 * (*_7);
_1.0 = _21.0;
_19 = 213_u8 as u64;
(*_7) = -Field::<f32>(Variant(_15, 0), 1);
_1.2 = _5.2;
_21.3 = _5.3;
RET = 29353846724369699097581767437419131638_i128 + (-131866340688892435899930037207606346837_i128);
SetDiscriminant(_15, 1);
_23 = ['\u{2355b}','\u{14098}','\u{a4ab3}'];
_30 = _28 as u64;
(*_4) = _30 | _30;
_28 = _13;
_22.2 = [1719644880786162508_i64,1605529290958900907_i64,2144668356263036670_i64,1578753491644779605_i64,5232120256103581472_i64];
_22.2 = [8941948909336550761_i64,90712976396032620_i64,449407253032239441_i64,(-7318641701121137699_i64),4000128303811872246_i64];
_5 = (_8, _21.1, _1.2, _1.3);
(*_7) = -_27;
Call(_1.2 = fn16(_1.0, _28, _21.2, _25, (*_7), _21.2, _17, _7, _5.2, _25, _27, _6, _10, _17, _25), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_20 = _1.3 < _5.3;
_5.0 = [(-1367916589105791854_i64),(-6123299197569906506_i64),1258314099934723517_i64,(-260315209421508691_i64)];
_1 = (_5.0, _22.0, _5.2, _21.3);
_28 = _20 ^ _13;
_5.2 = core::ptr::addr_of_mut!((*_7));
_17 = -_25;
_2.0 = _1.1 << (*_26);
_2.0 = !_5.1;
_16 = (Move(_26),);
_34 = Move(_16.0);
_32 = _17 as u32;
_27 = RET as f32;
_35 = ['\u{2ed14}','\u{94dd0}','\u{af307}'];
_16.0 = Move(_34);
_25 = -_11;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(15_usize, 20_usize, Move(_20), 13_usize, Move(_13), 17_usize, Move(_17), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(15_usize, 11_usize, Move(_11), 3_usize, Move(_3), 2_usize, Move(_2), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(15_usize, 22_usize, Move(_22), 39_usize, _39, 39_usize, _39, 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: [i64; 4],mut _2: bool,mut _3: *mut f32,mut _4: isize,mut _5: f32,mut _6: *mut f32,mut _7: isize,mut _8: *mut f32,mut _9: *mut f32,mut _10: isize,mut _11: f32,mut _12: isize,mut _13: i32,mut _14: isize,mut _15: isize) -> *mut f32 {
mir! {
type RET = *mut f32;
let _16: f32;
let _17: char;
let _18: ();
let _19: ();
{
RET = core::ptr::addr_of_mut!((*_6));
(*RET) = _11;
RET = _8;
(*_3) = _5;
_2 = false;
RET = _8;
_13 = 87254750547734904916090882323883748020_i128 as i32;
(*_9) = 73_u8 as f32;
(*_3) = _5 + _11;
_7 = -_15;
(*RET) = _11;
Goto(bb1)
}
bb1 = {
Call(_18 = dump_var(16_usize, 2_usize, Move(_2), 4_usize, Move(_4), 14_usize, Move(_14), 1_usize, Move(_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: i8,mut _2: [i32; 6],mut _3: [i32; 6],mut _4: [i64; 4],mut _5: [i64; 5],mut _6: [usize; 4],mut _7: char,mut _8: i8,mut _9: [i64; 4],mut _10: [i64; 5],mut _11: [usize; 4],mut _12: [u128; 2],mut _13: [i64; 4],mut _14: [i64; 5]) -> f32 {
mir! {
type RET = f32;
let _15: Adt53;
let _16: char;
let _17: Adt55;
let _18: (u128, i8, [i64; 5]);
let _19: bool;
let _20: isize;
let _21: char;
let _22: [i32; 6];
let _23: ([u128; 5], usize, *mut f32, bool, *mut [u128; 5]);
let _24: i32;
let _25: (u128, i8, [i64; 5]);
let _26: ([u128; 5], usize, *mut f32, bool, *mut [u128; 5]);
let _27: i64;
let _28: isize;
let _29: *mut [u128; 5];
let _30: i32;
let _31: [i64; 4];
let _32: u64;
let _33: u8;
let _34: [usize; 4];
let _35: u8;
let _36: usize;
let _37: [i64; 4];
let _38: char;
let _39: bool;
let _40: [u128; 2];
let _41: usize;
let _42: f64;
let _43: [i64; 4];
let _44: u16;
let _45: f32;
let _46: [i32; 6];
let _47: i32;
let _48: *mut f32;
let _49: isize;
let _50: isize;
let _51: u32;
let _52: [i64; 4];
let _53: ();
let _54: ();
{
_10 = [1313718569064019441_i64,(-5054069614776971008_i64),(-1757477587010125748_i64),(-3170040434297348710_i64),4102950364162972998_i64];
Goto(bb1)
}
bb1 = {
_9 = [8892588684179848480_i64,(-5695377240794275116_i64),(-2994392785376890563_i64),(-2224344854424347870_i64)];
RET = 1451834697_u32 as f32;
_13 = [(-4148977817431303645_i64),(-6080591199438095757_i64),1772058519834868372_i64,(-2175286829289498395_i64)];
_5 = [(-1642919560500304695_i64),782862608002311997_i64,(-6474555551011736297_i64),(-8782500635565649855_i64),1797318769550532185_i64];
_12 = [80961375105246070101920043068869568031_u128,79456386259984923541145323215700765296_u128];
_16 = _7;
_2 = [1270023099_i32,(-37438879_i32),(-1133059459_i32),(-1301366763_i32),1220413107_i32,1293722210_i32];
_14 = [5218816802626311657_i64,(-3676998954586567381_i64),5367906215012531140_i64,(-5282880847301160451_i64),8329997479512296671_i64];
RET = 8824_i16 as f32;
_7 = _16;
_7 = _16;
_4 = [3181793838556024848_i64,5989207233866104354_i64,(-3233818602123176688_i64),4547005315098669010_i64];
_5 = _10;
_10 = [563929130368742788_i64,5838212271913051932_i64,(-5987586071937277985_i64),(-6801367404138347639_i64),8589092051972008896_i64];
_8 = (-87_isize) as i8;
_1 = 166_u8 as i8;
_3 = [(-364351714_i32),(-1475199116_i32),(-498621464_i32),1980349233_i32,(-459503635_i32),819031069_i32];
RET = 8735131622992345811_usize as f32;
_16 = _7;
_7 = _16;
_14 = _10;
_18.0 = !212256251117610244654644418451789070941_u128;
_18.1 = !_8;
_14 = _5;
Goto(bb2)
}
bb2 = {
_18.2 = [(-4909276192731695608_i64),(-179262234773740087_i64),(-1675887137726694060_i64),(-8274258881467159627_i64),(-8933811254076900323_i64)];
_6 = _11;
_12 = [_18.0,_18.0];
_2 = _3;
_12 = [_18.0,_18.0];
_13 = [(-2813642135259280881_i64),7283488380906740814_i64,216379364954423109_i64,3416900220551781920_i64];
_11 = [5_usize,4_usize,7_usize,1_usize];
_3 = _2;
_8 = _1 - _18.1;
Goto(bb3)
}
bb3 = {
_3 = _2;
RET = 825_u16 as f32;
_21 = _7;
_12 = [_18.0,_18.0];
_7 = _16;
_21 = _7;
_10 = [9204379107184858161_i64,1494032614451697598_i64,(-1027626027445778419_i64),(-1658593384814981281_i64),5773772706864899052_i64];
_5 = _18.2;
_18.2 = [(-7949576832440098370_i64),(-3442862822764904253_i64),(-2806424782666154161_i64),(-6256357629172395586_i64),(-2212224227334122928_i64)];
_7 = _16;
_16 = _7;
RET = 1238388228_u32 as f32;
_12 = [_18.0,_18.0];
_23.1 = 80_isize as usize;
RET = 144281369148986042170498942471855824238_i128 as f32;
_18.1 = _8 - _8;
RET = 1671288626_i32 as f32;
_23.4 = core::ptr::addr_of_mut!(_23.0);
_23.1 = 3268325488831672152_usize * 14117592589370326239_usize;
_23.1 = !16473994259336574640_usize;
_23.1 = 2_usize;
_18.1 = _8;
_16 = _7;
match _23.1 {
2 => bb5,
_ => bb4
}
}
bb4 = {
_18.2 = [(-4909276192731695608_i64),(-179262234773740087_i64),(-1675887137726694060_i64),(-8274258881467159627_i64),(-8933811254076900323_i64)];
_6 = _11;
_12 = [_18.0,_18.0];
_2 = _3;
_12 = [_18.0,_18.0];
_13 = [(-2813642135259280881_i64),7283488380906740814_i64,216379364954423109_i64,3416900220551781920_i64];
_11 = [5_usize,4_usize,7_usize,1_usize];
_3 = _2;
_8 = _1 - _18.1;
Goto(bb3)
}
bb5 = {
_20 = _16 as isize;
_23.3 = false;
_25.0 = !_18.0;
_19 = _8 > _8;
_23.3 = _20 <= _20;
_26.2 = core::ptr::addr_of_mut!(RET);
_7 = _21;
_13 = [8352503414155085659_i64,8911490769221167270_i64,2512610634484151880_i64,7982720787897157900_i64];
_19 = _23.3 & _23.3;
_23.2 = _26.2;
_18.2 = [1696348697182540883_i64,(-4074316747488512313_i64),1337041230235670015_i64,2155279252486877397_i64,(-1753128537072418652_i64)];
RET = _20 as f32;
RET = (-6818978987954524075_i64) as f32;
_25 = _18;
_11 = [_23.1,_23.1,_23.1,_23.1];
_18.1 = _8 ^ _25.1;
_18.2 = [(-439634722824324595_i64),(-8595726507725548944_i64),(-4021509590716951955_i64),(-9104799868155924909_i64),(-108662440021575936_i64)];
_25 = _18;
_27 = 1537422128581666484_i64 + (-4340500385542728313_i64);
_26.1 = _23.1;
_19 = !_23.3;
_4 = [_27,_27,_27,_27];
_8 = -_18.1;
match _23.1 {
2 => bb6,
_ => bb4
}
}
bb6 = {
_20 = 9223372036854775807_isize - 9223372036854775807_isize;
_23.0 = [_18.0,_25.0,_18.0,_18.0,_25.0];
RET = _20 as f32;
_6 = [_26.1,_23.1,_26.1,_26.1];
_13 = _4;
_18 = _25;
_5 = [_27,_27,_27,_27,_27];
_23.2 = _26.2;
_7 = _16;
_22 = [(-1658152058_i32),196545175_i32,(-652542194_i32),(-2099738956_i32),746104065_i32,(-470032462_i32)];
_9 = [_27,_27,_27,_27];
_24 = _25.1 as i32;
_4 = [_27,_27,_27,_27];
_26.2 = core::ptr::addr_of_mut!(RET);
_14 = [_27,_27,_27,_27,_27];
_12 = [_25.0,_25.0];
Goto(bb7)
}
bb7 = {
RET = 14223402637817363550_u64 as f32;
Call(_25.0 = core::intrinsics::transmute(_18.0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Goto(bb9)
}
bb9 = {
_21 = _7;
_26.3 = _18.1 > _25.1;
_26.4 = _23.4;
_25.1 = _18.1 + _18.1;
_7 = _16;
_3 = [_24,_24,_24,_24,_24,_24];
_32 = _24 as u64;
_7 = _21;
_25.0 = _18.0 - _18.0;
_1 = _25.1 ^ _25.1;
_24 = (-1258297564_i32);
_25 = (_18.0, _1, _14);
_33 = 205_u8 * 40_u8;
_28 = 3066037727_u32 as isize;
_30 = _24 & _24;
_23.1 = !_26.1;
_10 = [_27,_27,_27,_27,_27];
_30 = _18.0 as i32;
_23.3 = RET <= RET;
_25.0 = _18.0;
_25.0 = _18.0 ^ _18.0;
_3 = [_30,_24,_30,_30,_24,_30];
_20 = !_28;
match _24 {
340282366920938463463374607430509913892 => bb11,
_ => bb10
}
}
bb10 = {
_18.2 = [(-4909276192731695608_i64),(-179262234773740087_i64),(-1675887137726694060_i64),(-8274258881467159627_i64),(-8933811254076900323_i64)];
_6 = _11;
_12 = [_18.0,_18.0];
_2 = _3;
_12 = [_18.0,_18.0];
_13 = [(-2813642135259280881_i64),7283488380906740814_i64,216379364954423109_i64,3416900220551781920_i64];
_11 = [5_usize,4_usize,7_usize,1_usize];
_3 = _2;
_8 = _1 - _18.1;
Goto(bb3)
}
bb11 = {
_25 = (_18.0, _8, _5);
_31 = _13;
_14 = [_27,_27,_27,_27,_27];
_26 = (_23.0, _23.1, _23.2, _19, _23.4);
_18.2 = [_27,_27,_27,_27,_27];
_6 = [_23.1,_26.1,_23.1,_23.1];
_36 = !_23.1;
_16 = _7;
_16 = _7;
Goto(bb12)
}
bb12 = {
_5 = [_27,_27,_27,_27,_27];
_37 = [_27,_27,_27,_27];
_18.0 = _25.0;
_9 = [_27,_27,_27,_27];
_33 = 179_u8;
_26.4 = core::ptr::addr_of_mut!(_23.0);
_23.2 = core::ptr::addr_of_mut!(RET);
_34 = [_23.1,_23.1,_26.1,_23.1];
_25.1 = _33 as i8;
_23.4 = _26.4;
_29 = _23.4;
_38 = _7;
_35 = _33;
_26.3 = _1 >= _8;
_18.1 = _8;
_18.0 = _20 as u128;
match _33 {
0 => bb8,
1 => bb10,
2 => bb3,
3 => bb4,
179 => bb13,
_ => bb6
}
}
bb13 = {
_37 = [_27,_27,_27,_27];
_2 = [_24,_30,_30,_30,_24,_30];
_4 = _37;
_31 = [_27,_27,_27,_27];
_25.0 = _18.0 + _18.0;
_18.2 = [_27,_27,_27,_27,_27];
_11 = [_36,_36,_23.1,_23.1];
_20 = _28;
_25.2 = _14;
_2 = _22;
_26.4 = core::ptr::addr_of_mut!(_23.0);
_22 = [_24,_24,_30,_30,_30,_24];
_25 = (_18.0, _18.1, _18.2);
_40 = _12;
_26.4 = core::ptr::addr_of_mut!((*_29));
_43 = [_27,_27,_27,_27];
_26.0 = _23.0;
_26.0 = (*_29);
Goto(bb14)
}
bb14 = {
_47 = _30;
_43 = [_27,_27,_27,_27];
_5 = _10;
_50 = _28 & _20;
_40 = _12;
_11 = [_26.1,_26.1,_23.1,_23.1];
_24 = !_30;
_23 = (_26.0, _36, _26.2, _26.3, _26.4);
_21 = _16;
_18.2 = _25.2;
_1 = _18.1 * _25.1;
_23.0 = [_25.0,_18.0,_25.0,_25.0,_18.0];
_45 = _36 as f32;
_40 = [_18.0,_18.0];
_25.2 = [_27,_27,_27,_27,_27];
Goto(bb15)
}
bb15 = {
Call(_53 = dump_var(17_usize, 5_usize, Move(_5), 16_usize, Move(_16), 14_usize, Move(_14), 40_usize, Move(_40)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_53 = dump_var(17_usize, 34_usize, Move(_34), 30_usize, Move(_30), 1_usize, Move(_1), 28_usize, Move(_28)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_53 = dump_var(17_usize, 12_usize, Move(_12), 8_usize, Move(_8), 27_usize, Move(_27), 11_usize, Move(_11)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_53 = dump_var(17_usize, 33_usize, Move(_33), 19_usize, Move(_19), 32_usize, Move(_32), 7_usize, Move(_7)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_53 = dump_var(17_usize, 24_usize, Move(_24), 3_usize, Move(_3), 54_usize, _54, 54_usize, _54), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: f32,mut _2: i8,mut _3: [i64; 4],mut _4: (u128, i8, [i64; 5]),mut _5: [u128; 2],mut _6: [i64; 5],mut _7: u128,mut _8: *mut f32,mut _9: f32,mut _10: [i64; 4],mut _11: [i32; 6],mut _12: [i64; 5]) -> i8 {
mir! {
type RET = i8;
let _13: [i32; 1];
let _14: [usize; 4];
let _15: i128;
let _16: [i64; 5];
let _17: Adt54;
let _18: u32;
let _19: isize;
let _20: i16;
let _21: u64;
let _22: isize;
let _23: [i32; 1];
let _24: char;
let _25: Adt42;
let _26: &'static u32;
let _27: isize;
let _28: f32;
let _29: f32;
let _30: char;
let _31: isize;
let _32: isize;
let _33: [usize; 4];
let _34: u64;
let _35: [u128; 5];
let _36: char;
let _37: [usize; 4];
let _38: isize;
let _39: [char; 3];
let _40: char;
let _41: Adt52;
let _42: [char; 3];
let _43: [i32; 1];
let _44: isize;
let _45: i16;
let _46: char;
let _47: [i32; 1];
let _48: u64;
let _49: [isize; 4];
let _50: usize;
let _51: [usize; 4];
let _52: [i32; 1];
let _53: ();
let _54: ();
{
_11 = [131189858_i32,251830407_i32,509994001_i32,398006468_i32,(-174251134_i32),(-138086767_i32)];
_6 = _12;
RET = 5_usize as i8;
_8 = core::ptr::addr_of_mut!((*_8));
_11 = [1027590783_i32,(-1121121578_i32),(-1300465226_i32),(-424958904_i32),(-1152651237_i32),(-467765827_i32)];
_12 = [1688351805978352709_i64,(-6777331522283375751_i64),(-5665352178319897489_i64),(-7858601884084480626_i64),(-3890743685619937641_i64)];
_4.1 = !RET;
_4.0 = _7;
Goto(bb1)
}
bb1 = {
_4.0 = !_7;
(*_8) = _1 - _9;
_13 = [527721221_i32];
_8 = core::ptr::addr_of_mut!((*_8));
_9 = 172_u8 as f32;
_12 = [5893494577517452054_i64,(-7937071232455443575_i64),(-5156466182992263640_i64),7234037952085534326_i64,3173583346116507920_i64];
Goto(bb2)
}
bb2 = {
_13 = [(-399541417_i32)];
(*_8) = -_9;
_14 = [1_usize,6_usize,4_usize,2639415139909121194_usize];
_12 = [(-6203276115567214366_i64),7278584373688812502_i64,6179836865241702400_i64,(-6941550812886298012_i64),(-8136639584722186983_i64)];
RET = _2 * _2;
_17.fld2.3 = !true;
_2 = !RET;
_17.fld3 = ['\u{3309}','\u{9d5a1}','\u{84f8f}'];
_4 = (_7, RET, _6);
_17.fld2.3 = false;
RET = 669164964041267284_usize as i8;
RET = _4.1 - _2;
_17.fld2.0 = [_4.0,_4.0,_7,_7,_4.0];
Goto(bb3)
}
bb3 = {
RET = (-45_isize) as i8;
_4.0 = (-9223372036854775808_isize) as u128;
_7 = _4.0;
_2 = !_4.1;
_17.fld0 = RET != _2;
_7 = (-24061_i16) as u128;
_4 = (_7, RET, _6);
_17.fld2.4 = core::ptr::addr_of_mut!(_17.fld2.0);
RET = _2 * _2;
_11 = [215514938_i32,(-765678750_i32),1723309913_i32,451651562_i32,(-688646050_i32),(-1316818945_i32)];
_12 = _6;
_17.fld2.2 = core::ptr::addr_of_mut!((*_8));
_18 = !2038744982_u32;
Goto(bb4)
}
bb4 = {
_4.2 = _6;
_17.fld2.3 = !_17.fld0;
(*_8) = _9;
_19 = !(-52_isize);
_6 = _4.2;
_17.fld3 = ['\u{31a2e}','\u{dba28}','\u{28d53}'];
_21 = 17781910259167483764_u64 * 10022964057207908283_u64;
(*_8) = _9 * _9;
_4.1 = -RET;
_4.2 = _12;
_17.fld1 = 11652906316098518450_usize | 13324489530914913972_usize;
Goto(bb5)
}
bb5 = {
_15 = _19 as i128;
_10 = _3;
_17.fld2.1 = !_17.fld1;
_17.fld2.4 = core::ptr::addr_of_mut!(_17.fld2.0);
RET = _2 & _4.1;
_22 = _19 >> _15;
_4.2 = _12;
_4 = (_7, _2, _6);
_4 = (_7, _2, _12);
_25 = Adt42 { fld0: _14 };
(*_8) = _1;
_20 = 24779_i16 ^ 16019_i16;
_20 = !25306_i16;
_7 = _17.fld0 as u128;
_5 = [_7,_7];
_25 = Adt42 { fld0: _14 };
_11 = [1622692392_i32,(-610512626_i32),2141107957_i32,664342646_i32,1436800626_i32,1184992297_i32];
Goto(bb6)
}
bb6 = {
_17.fld2.4 = core::ptr::addr_of_mut!(_17.fld2.0);
_17.fld5 = core::ptr::addr_of_mut!(_21);
_23 = _13;
_4.2 = [3197870503636162132_i64,3517157643979420835_i64,1745441967362594931_i64,(-3003920743843208619_i64),(-8939868134259383250_i64)];
_10 = [(-1098387588232944785_i64),(-2991661011283217504_i64),(-5912211590611277492_i64),5063147849854670608_i64];
_17.fld2.0 = [_4.0,_7,_7,_7,_7];
_4.1 = (-232052515_i32) as i8;
_17.fld3 = ['\u{a211f}','\u{30b16}','\u{e8760}'];
_20 = (-5478_i16);
_25.fld0 = [_17.fld2.1,_17.fld2.1,_17.fld2.1,_17.fld1];
_17.fld0 = _17.fld2.3 != _17.fld2.3;
_17.fld6 = _4.2;
(*_8) = _1 - _1;
_3 = [3669692668785508327_i64,6109765908785832827_i64,449787916706994693_i64,6366896269603698087_i64];
_3 = [(-5606570761162815145_i64),(-2144649459833215548_i64),(-6439651995834988759_i64),9185296691642901992_i64];
match _20 {
0 => bb1,
340282366920938463463374607431768205978 => bb7,
_ => bb5
}
}
bb7 = {
_17.fld2.0 = [_4.0,_7,_7,_7,_4.0];
_6 = _12;
_3 = [(-3457662208776075893_i64),(-88737814437033955_i64),(-6760112858765124388_i64),(-8601357253327820939_i64)];
_24 = '\u{42fea}';
_15 = _7 as i128;
_8 = core::ptr::addr_of_mut!(_28);
_12 = [(-2920157319730554995_i64),(-4581415690292179738_i64),(-7841264555697708179_i64),(-3416250055811975090_i64),(-3826322209815635972_i64)];
RET = _2;
_4.1 = RET + RET;
_28 = _20 as f32;
_30 = _24;
_2 = !_4.1;
_5 = [_4.0,_7];
_29 = -(*_8);
_6 = _17.fld6;
_25 = Adt42 { fld0: _14 };
_4.1 = _21 as i8;
_2 = 7925999655870834097_i64 as i8;
(*_8) = _9 * _29;
_4.2 = _17.fld6;
_26 = &_18;
(*_8) = -_29;
Goto(bb8)
}
bb8 = {
_1 = _15 as f32;
_10 = [(-2696952083043617580_i64),(-5299886497588812896_i64),(-2535585694180305129_i64),(-2728458401237521279_i64)];
_20 = !23524_i16;
(*_8) = _1;
_17.fld2.2 = core::ptr::addr_of_mut!(_29);
_31 = _19;
RET = _20 as i8;
_32 = !_19;
_24 = _30;
_32 = _17.fld0 as isize;
Goto(bb9)
}
bb9 = {
_16 = [(-4890355716538506874_i64),145420067460569088_i64,(-5897880580691819046_i64),(-3598765198754361662_i64),(-295247076993427443_i64)];
_34 = _21 - _21;
_32 = _19 >> _7;
_17.fld2.3 = _17.fld0;
Goto(bb10)
}
bb10 = {
_27 = _17.fld1 as isize;
_4.2 = [1893890466903571169_i64,6800555626311311379_i64,8496341389767151424_i64,8255777244452262754_i64,5053994137153618993_i64];
_21 = _34 | _34;
_1 = _28;
_26 = &(*_26);
_4.2 = [(-5137863413854496782_i64),(-1777896888227253887_i64),1535237066578975620_i64,(-4989306878907050509_i64),(-3168007710528211067_i64)];
_2 = -_4.1;
_18 = 2779154907_u32 ^ 199684042_u32;
_19 = (-677629657_i32) as isize;
_35 = [_7,_4.0,_7,_4.0,_4.0];
_37 = [_17.fld2.1,_17.fld2.1,_17.fld2.1,_17.fld2.1];
_24 = _30;
_27 = _19;
_4.2 = [(-1530624181591412363_i64),1283218054264628795_i64,7145313636406753448_i64,6339559782539686686_i64,(-7074628856354828232_i64)];
_17.fld2.3 = !_17.fld0;
_13 = _23;
_38 = _27 >> _21;
Goto(bb11)
}
bb11 = {
_36 = _30;
_2 = -RET;
_17.fld5 = core::ptr::addr_of_mut!(_34);
_30 = _24;
_38 = !_32;
_27 = _32;
_25 = Adt42 { fld0: _14 };
_4.0 = _7 ^ _7;
_33 = [_17.fld2.1,_17.fld1,_17.fld2.1,_17.fld1];
Goto(bb12)
}
bb12 = {
(*_8) = _32 as f32;
_33 = [_17.fld2.1,_17.fld1,_17.fld1,_17.fld1];
_3 = _10;
_4 = (_7, RET, _17.fld6);
_12 = [(-8924349941204249603_i64),(-1601945996972920242_i64),(-6608701577957634137_i64),(-2502387889505374942_i64),(-2535929731975808495_i64)];
_12 = [8026520829292001371_i64,1065759355263522788_i64,(-557357262070011542_i64),3510577852650733602_i64,1904931912446604219_i64];
_23 = [(-1136887232_i32)];
_29 = _18 as f32;
_21 = 500_u16 as u64;
_26 = &_18;
_31 = _32 & _27;
_16 = [263021645239088579_i64,(-4638718953058940255_i64),8789928493733502779_i64,4814632472550821541_i64,(-4926901001939474726_i64)];
_3 = _10;
RET = _2 & _4.1;
_23 = [(-884264537_i32)];
_39 = [_30,_30,_30];
_17.fld5 = core::ptr::addr_of_mut!(_34);
_25 = Adt42 { fld0: _14 };
RET = !_4.1;
_36 = _24;
Goto(bb13)
}
bb13 = {
_12 = _6;
_47 = _13;
_46 = _24;
_31 = _27;
_20 = (-27055_i16) - 15271_i16;
_30 = _24;
_23 = [(-949788006_i32)];
_4 = (_7, _2, _12);
_17.fld2.1 = _7 as usize;
_48 = _34;
_31 = -_32;
_17.fld3 = _39;
_2 = _17.fld2.1 as i8;
_17.fld2.4 = core::ptr::addr_of_mut!(_35);
_49 = [_38,_19,_31,_38];
_2 = _7 as i8;
_7 = 2386766597284933656_i64 as u128;
_17.fld2.4 = core::ptr::addr_of_mut!(_35);
_14 = [_17.fld2.1,_17.fld1,_17.fld1,_17.fld2.1];
_37 = _14;
Goto(bb14)
}
bb14 = {
_17.fld2.2 = _8;
_17.fld2.3 = _17.fld0;
_14 = [_17.fld1,_17.fld2.1,_17.fld1,_17.fld2.1];
RET = -_2;
_31 = _27;
_7 = _4.0;
_17.fld2.3 = _17.fld0;
_26 = &_18;
_40 = _46;
_44 = 145_u8 as isize;
_4.1 = RET;
_25 = Adt42 { fld0: _37 };
_37 = [_17.fld2.1,_17.fld2.1,_17.fld1,_17.fld2.1];
Goto(bb15)
}
bb15 = {
Call(_53 = dump_var(18_usize, 35_usize, Move(_35), 37_usize, Move(_37), 31_usize, Move(_31), 27_usize, Move(_27)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_53 = dump_var(18_usize, 18_usize, Move(_18), 32_usize, Move(_32), 44_usize, Move(_44), 40_usize, Move(_40)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_53 = dump_var(18_usize, 22_usize, Move(_22), 11_usize, Move(_11), 6_usize, Move(_6), 46_usize, Move(_46)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_53 = dump_var(18_usize, 19_usize, Move(_19), 33_usize, Move(_33), 4_usize, Move(_4), 13_usize, Move(_13)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_53 = dump_var(18_usize, 24_usize, Move(_24), 38_usize, Move(_38), 54_usize, _54, 54_usize, _54), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: isize,mut _2: [i64; 5],mut _3: [i64; 5],mut _4: [isize; 3],mut _5: [isize; 4],mut _6: [isize; 3],mut _7: [isize; 4],mut _8: bool) -> [usize; 4] {
mir! {
type RET = [usize; 4];
let _9: f64;
let _10: (&'static u32,);
let _11: f64;
let _12: f64;
let _13: isize;
let _14: f64;
let _15: [char; 3];
let _16: i32;
let _17: &'static u32;
let _18: f64;
let _19: bool;
let _20: Adt48;
let _21: usize;
let _22: i8;
let _23: [isize; 4];
let _24: Adt53;
let _25: [char; 3];
let _26: [i64; 5];
let _27: bool;
let _28: [u128; 5];
let _29: isize;
let _30: *const &'static u32;
let _31: u128;
let _32: Adt54;
let _33: [u32; 3];
let _34: isize;
let _35: isize;
let _36: bool;
let _37: isize;
let _38: u8;
let _39: *mut [u128; 5];
let _40: i32;
let _41: i8;
let _42: bool;
let _43: f32;
let _44: i128;
let _45: ();
let _46: ();
{
_3 = _2;
_1 = !9223372036854775807_isize;
RET = [18398940669780732423_usize,5706668144842442062_usize,12490658046958034335_usize,7_usize];
Goto(bb1)
}
bb1 = {
RET = [13751548961057281067_usize,10149395650476907854_usize,5010932266872947540_usize,5232574241688668808_usize];
RET = [1_usize,3_usize,8651019726583584475_usize,15556468749729627138_usize];
_4 = _6;
_7 = [_1,_1,_1,_1];
_6 = [_1,_1,_1];
_4 = [_1,_1,_1];
_7 = [_1,_1,_1,_1];
_9 = 24032_i16 as f64;
_11 = -_9;
_13 = 0_usize as isize;
_5 = _7;
_5 = _7;
_3 = [(-3517907963639700200_i64),(-7463478112205370081_i64),6325329156066483466_i64,8082839231209029921_i64,(-2917220263899206771_i64)];
_12 = 29295_u16 as f64;
_6 = _4;
_9 = _12 + _11;
_4 = [_13,_1,_13];
RET = [11708477452881570517_usize,4714101429161907679_usize,6950410432021440323_usize,0_usize];
_3 = [(-7223001852385381998_i64),(-7159935935647534251_i64),(-3297723097767077148_i64),1151610621140427650_i64,(-2765216159059507945_i64)];
_11 = _9;
_6 = [_1,_13,_1];
_5 = [_1,_13,_1,_13];
_8 = false;
RET = [6_usize,1_usize,6_usize,4_usize];
_7 = [_13,_1,_1,_13];
_5 = [_13,_1,_1,_1];
_12 = -_11;
Goto(bb2)
}
bb2 = {
_14 = _12 - _12;
_11 = _13 as f64;
RET = [2_usize,3_usize,1_usize,2_usize];
_15 = ['\u{cce72}','\u{1271f}','\u{130d1}'];
Goto(bb3)
}
bb3 = {
_12 = _14;
RET = [0_usize,12231182674043594998_usize,13636556052072236901_usize,1_usize];
_5 = [_13,_13,_1,_1];
_7 = [_13,_1,_13,_1];
_2 = [7410911635435970508_i64,6460077655367752655_i64,(-6869095451957907510_i64),6012021314968340687_i64,8581430648549422200_i64];
_16 = (-524345782_i32);
_12 = _11;
_3 = _2;
_19 = _9 > _14;
_5 = _7;
_4 = [_13,_13,_13];
RET = [3889141190966849400_usize,1860800266253356902_usize,11524812535948117713_usize,989624957391297021_usize];
_15 = ['\u{3bfbb}','\u{6b390}','\u{106a6a}'];
_18 = _14;
_9 = _14 * _14;
_8 = _19 | _19;
_16 = 1668052509_i32;
_18 = -_9;
_11 = (-4848794251347000751_i64) as f64;
_11 = _14 * _18;
_21 = !1838612799082842409_usize;
_4 = [_1,_13,_1];
_19 = _8 > _8;
_12 = _18;
Call(_13 = core::intrinsics::bswap(_1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_2 = [(-4110771765918123267_i64),7811923207307152784_i64,1149670353010553136_i64,(-169828517681663873_i64),(-9154790864544525555_i64)];
Goto(bb5)
}
bb5 = {
RET = [_21,_21,_21,_21];
_8 = _19;
_1 = _13 << _16;
_1 = _13;
RET = [_21,_21,_21,_21];
_3 = _2;
_16 = 382107803_i32;
_19 = !_8;
_19 = !_8;
_7 = [_1,_13,_1,_1];
_15 = ['\u{42d4b}','\u{6d5fc}','\u{e438e}'];
RET = [_21,_21,_21,_21];
_22 = (-60_i8) - (-44_i8);
_8 = _14 < _12;
RET = [_21,_21,_21,_21];
_18 = _14;
Goto(bb6)
}
bb6 = {
_19 = _14 <= _9;
_12 = 2160884451_u32 as f64;
_14 = _9 + _18;
RET = [_21,_21,_21,_21];
_11 = _14 + _18;
_6 = [_13,_13,_13];
_23 = _5;
_18 = -_11;
Goto(bb7)
}
bb7 = {
_32.fld1 = _21 | _21;
_31 = 113226707183791125749313218555212545922_u128 * 239673332652927837457997612622274042961_u128;
_28 = [_31,_31,_31,_31,_31];
_25 = ['\u{c3862}','\u{987fa}','\u{e90f}'];
_32.fld0 = _32.fld1 == _32.fld1;
_2 = [1775815493609440672_i64,9136383471692145553_i64,(-8767129931797871950_i64),(-4396979478510564233_i64),(-3254938190805951964_i64)];
_32.fld1 = _21;
_32.fld2.3 = _8;
RET = [_21,_21,_21,_32.fld1];
_32.fld2.0 = [_31,_31,_31,_31,_31];
_30 = core::ptr::addr_of!(_10.0);
_21 = !_32.fld1;
_32.fld1 = !_21;
match _16 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb8,
382107803 => bb10,
_ => bb9
}
}
bb8 = {
_19 = _14 <= _9;
_12 = 2160884451_u32 as f64;
_14 = _9 + _18;
RET = [_21,_21,_21,_21];
_11 = _14 + _18;
_6 = [_13,_13,_13];
_23 = _5;
_18 = -_11;
Goto(bb7)
}
bb9 = {
RET = [13751548961057281067_usize,10149395650476907854_usize,5010932266872947540_usize,5232574241688668808_usize];
RET = [1_usize,3_usize,8651019726583584475_usize,15556468749729627138_usize];
_4 = _6;
_7 = [_1,_1,_1,_1];
_6 = [_1,_1,_1];
_4 = [_1,_1,_1];
_7 = [_1,_1,_1,_1];
_9 = 24032_i16 as f64;
_11 = -_9;
_13 = 0_usize as isize;
_5 = _7;
_5 = _7;
_3 = [(-3517907963639700200_i64),(-7463478112205370081_i64),6325329156066483466_i64,8082839231209029921_i64,(-2917220263899206771_i64)];
_12 = 29295_u16 as f64;
_6 = _4;
_9 = _12 + _11;
_4 = [_13,_1,_13];
RET = [11708477452881570517_usize,4714101429161907679_usize,6950410432021440323_usize,0_usize];
_3 = [(-7223001852385381998_i64),(-7159935935647534251_i64),(-3297723097767077148_i64),1151610621140427650_i64,(-2765216159059507945_i64)];
_11 = _9;
_6 = [_1,_13,_1];
_5 = [_1,_13,_1,_13];
_8 = false;
RET = [6_usize,1_usize,6_usize,4_usize];
_7 = [_13,_1,_1,_13];
_5 = [_13,_1,_1,_1];
_12 = -_11;
Goto(bb2)
}
bb10 = {
_33 = [2198220127_u32,903498653_u32,1596760931_u32];
_37 = _31 as isize;
_36 = _32.fld0;
_36 = !_32.fld2.3;
_7 = _23;
_16 = 1546627247_i32;
_35 = _13 | _37;
_22 = 54_i8;
_25 = ['\u{cb62b}','\u{e404d}','\u{46fd1}'];
_31 = 209824306520301532179372852333888414417_u128;
_14 = -_9;
_29 = !_1;
_28 = _32.fld2.0;
RET = [_21,_32.fld1,_32.fld1,_32.fld1];
_12 = _9;
_26 = [(-4950694139681931978_i64),(-335127290259003782_i64),(-2693926707184641152_i64),(-2297744232750992234_i64),6475291916314898229_i64];
_20 = Adt48::Variant2 { fld0: 15526_i16 };
_5 = [_13,_29,_37,_35];
Goto(bb11)
}
bb11 = {
_6 = [_35,_35,_37];
_32.fld2.3 = _8;
_8 = _11 <= _18;
_34 = _29 | _35;
_15 = ['\u{32abc}','\u{da63e}','\u{3b8ac}'];
_16 = 749399867_i32 << _32.fld1;
_32.fld2.3 = !_8;
_40 = -_16;
_38 = 134_u8;
_32.fld6 = [884940925653493891_i64,(-4994218324498419635_i64),(-8486424887146523838_i64),5426557780787404603_i64,(-7708410819778149262_i64)];
_16 = _40;
_39 = core::ptr::addr_of_mut!(_32.fld2.0);
_22 = -20_i8;
match _31 {
0 => bb12,
1 => bb13,
209824306520301532179372852333888414417 => bb15,
_ => bb14
}
}
bb12 = {
_33 = [2198220127_u32,903498653_u32,1596760931_u32];
_37 = _31 as isize;
_36 = _32.fld0;
_36 = !_32.fld2.3;
_7 = _23;
_16 = 1546627247_i32;
_35 = _13 | _37;
_22 = 54_i8;
_25 = ['\u{cb62b}','\u{e404d}','\u{46fd1}'];
_31 = 209824306520301532179372852333888414417_u128;
_14 = -_9;
_29 = !_1;
_28 = _32.fld2.0;
RET = [_21,_32.fld1,_32.fld1,_32.fld1];
_12 = _9;
_26 = [(-4950694139681931978_i64),(-335127290259003782_i64),(-2693926707184641152_i64),(-2297744232750992234_i64),6475291916314898229_i64];
_20 = Adt48::Variant2 { fld0: 15526_i16 };
_5 = [_13,_29,_37,_35];
Goto(bb11)
}
bb13 = {
_19 = _14 <= _9;
_12 = 2160884451_u32 as f64;
_14 = _9 + _18;
RET = [_21,_21,_21,_21];
_11 = _14 + _18;
_6 = [_13,_13,_13];
_23 = _5;
_18 = -_11;
Goto(bb7)
}
bb14 = {
_14 = _12 - _12;
_11 = _13 as f64;
RET = [2_usize,3_usize,1_usize,2_usize];
_15 = ['\u{cce72}','\u{1271f}','\u{130d1}'];
Goto(bb3)
}
bb15 = {
RET = [_32.fld1,_32.fld1,_21,_32.fld1];
_32.fld2.1 = _32.fld1;
_32.fld6 = _2;
_42 = _32.fld2.3 <= _32.fld2.3;
_29 = _35 | _35;
_3 = [9060042171820152900_i64,8125624747144776561_i64,5904523311117963256_i64,79676843930633328_i64,858464526367202036_i64];
RET = [_32.fld1,_32.fld2.1,_32.fld2.1,_21];
Goto(bb16)
}
bb16 = {
Call(_45 = dump_var(19_usize, 29_usize, Move(_29), 28_usize, Move(_28), 1_usize, Move(_1), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(19_usize, 26_usize, Move(_26), 7_usize, Move(_7), 33_usize, Move(_33), 21_usize, Move(_21)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(19_usize, 25_usize, Move(_25), 35_usize, Move(_35), 23_usize, Move(_23), 15_usize, Move(_15)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_45 = dump_var(19_usize, 16_usize, Move(_16), 3_usize, Move(_3), 46_usize, _46, 46_usize, _46), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{710ce}'), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(78_i8), std::hint::black_box((-6534_i16)), std::hint::black_box((-881671702_i32)), std::hint::black_box(5643256146731522804_i64), std::hint::black_box((-147761117344972106887708432795472486456_i128)), std::hint::black_box(3_usize), std::hint::black_box(137_u8), std::hint::black_box(16774_u16), std::hint::black_box(3393882056_u32), std::hint::black_box(13202011406030394198_u64), std::hint::black_box(207088504886977152597137998329804268880_u128));
                
            }
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt42{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt42 {
fld0: [usize; 4],
}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
fld0: bool,
fld1: f32,

},
Variant1{
fld0: [u128; 5],
fld1: [i32; 6],
fld2: [i64; 4],

}}
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: f64,
fld1: char,
fld2: Adt42,
fld3: [usize; 4],
fld4: *mut u64,
fld5: u128,
fld6: ([i32; 1], u16, usize, u32, f32, *mut f32),

},
Variant1{
fld0: *const i16,
fld1: [i32; 6],
fld2: *mut [u128; 5],
fld3: *mut f32,
fld4: i16,
fld5: f32,
fld6: *mut ([i32; 1], u16, usize, u32, f32, *mut f32),
fld7: f64,

},
Variant2{
fld0: Adt42,
fld1: f32,
fld2: u64,
fld3: [i64; 5],
fld4: [isize; 3],

},
Variant3{
fld0: [i32; 1],
fld1: (*mut u64,),

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: i8,

},
Variant1{
fld0: Adt43,
fld1: i128,
fld2: [u128; 5],
fld3: *mut [u128; 5],

},
Variant2{
fld0: [i64; 4],
fld1: Adt42,
fld2: [u128; 5],
fld3: [char; 3],

},
Variant3{
fld0: bool,
fld1: u16,
fld2: u32,

}}
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: f32,
fld1: u32,
fld2: [isize; 4],
fld3: *const i16,
fld4: *mut f32,
fld5: (u128, i8, [i64; 5]),

},
Variant1{
fld0: [usize; 6],
fld1: Adt44,
fld2: f32,
fld3: (u128, i8, [i64; 5]),
fld4: Adt45,
fld5: ([u128; 5], usize, *mut f32, bool, *mut [u128; 5]),

},
Variant2{
fld0: [i32; 1],
fld1: [i64; 4],
fld2: u32,

},
Variant3{
fld0: (u128, i8, [i64; 5]),
fld1: f64,
fld2: isize,
fld3: u64,
fld4: i16,
fld5: u8,
fld6: [usize; 4],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: [u128; 2],

},
Variant1{
fld0: usize,
fld1: i32,
fld2: isize,
fld3: f32,

},
Variant2{
fld0: bool,
fld1: u16,
fld2: Adt44,
fld3: [isize; 4],

},
Variant3{
fld0: Adt46,
fld1: Adt45,
fld2: i8,

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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: bool,
fld1: *const i16,

},
Variant1{
fld0: Adt44,
fld1: *mut [u128; 5],
fld2: *mut f32,
fld3: ([i64; 4], u128, *mut f32, f64),
fld4: ([i32; 1], u16, usize, u32, f32, *mut f32),
fld5: [i32; 1],
fld6: [usize; 4],

},
Variant2{
fld0: i16,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: *const i16,
fld1: Adt48,
fld2: [usize; 6],
fld3: *mut u64,
fld4: *mut ([i32; 1], u16, usize, u32, f32, *mut f32),
fld5: [isize; 4],
fld6: f64,
fld7: u64,

},
Variant1{
fld0: ([i64; 4], u128, *mut f32, f64),
fld1: [u128; 2],
fld2: *mut ([i32; 1], u16, usize, u32, f32, *mut f32),
fld3: i8,

},
Variant2{
fld0: Adt45,
fld1: f64,
fld2: *mut u64,
fld3: i8,
fld4: [u32; 3],

},
Variant3{
fld0: bool,
fld1: [usize; 6],
fld2: *mut f32,
fld3: Adt47,
fld4: *mut [u128; 5],

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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: ([i64; 4], u128, *mut f32, f64),
fld1: f32,
fld2: (u128, i8, [i64; 5]),
fld3: i8,
fld4: Adt48,
fld5: i32,
fld6: i64,

},
Variant1{
fld0: Adt46,
fld1: [usize; 4],
fld2: [isize; 4],
fld3: [i32; 1],
fld4: [isize; 3],
fld5: (u128, i8, [i64; 5]),
fld6: *mut [u128; 5],
fld7: i128,

},
Variant2{
fld0: bool,
fld1: char,
fld2: Adt48,
fld3: [isize; 4],
fld4: *mut u64,
fld5: i128,
fld6: [u128; 2],

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: (u128, i8, [i64; 5]),
fld1: u32,
fld2: Adt42,
fld3: Adt50,
fld4: *mut f32,
fld5: [isize; 3],

},
Variant1{
fld0: bool,
fld1: *mut ([i32; 1], u16, usize, u32, f32, *mut f32),

},
Variant2{
fld0: [u128; 5],
fld1: [usize; 4],
fld2: [u128; 2],
fld3: [i32; 6],
fld4: ([u128; 5], usize, *mut f32, bool, *mut [u128; 5]),
fld5: Adt47,

},
Variant3{
fld0: i64,
fld1: *mut ([i32; 1], u16, usize, u32, f32, *mut f32),

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: [i64; 4],
fld1: Adt44,

},
Variant1{
fld0: ([i32; 1], u16, usize, u32, f32, *mut f32),
fld1: usize,
fld2: [i64; 4],
fld3: [usize; 6],

},
Variant2{
fld0: [i64; 4],
fld1: f64,
fld2: ([u128; 5], usize, *mut f32, bool, *mut [u128; 5]),
fld3: [isize; 4],
fld4: [u128; 5],
fld5: [u128; 2],
fld6: i128,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: [u128; 5],
fld1: Adt45,
fld2: Adt49,
fld3: Adt48,
fld4: Adt50,
fld5: i32,
fld6: [isize; 4],
fld7: (*mut u64,),

},
Variant1{
fld0: Adt45,
fld1: u64,
fld2: ([i64; 4], u128, *mut f32, f64),
fld3: [i64; 4],

},
Variant2{
fld0: Adt49,
fld1: Adt45,
fld2: [u128; 2],
fld3: Adt51,
fld4: *mut f32,

},
Variant3{
fld0: [usize; 6],
fld1: char,
fld2: [i32; 1],
fld3: usize,
fld4: Adt42,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt54{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt54 {
fld0: bool,
fld1: usize,
fld2: ([u128; 5], usize, *mut f32, bool, *mut [u128; 5]),
fld3: [char; 3],
fld4: Adt53,
fld5: *mut u64,
fld6: [i64; 5],
}
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
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: f32,
fld1: f64,
fld2: *mut u64,

},
Variant1{
fld0: *mut f32,
fld1: ([u128; 5], usize, *mut f32, bool, *mut [u128; 5]),
fld2: Adt48,
fld3: [i64; 4],
fld4: i16,
fld5: Adt42,

},
Variant2{
fld0: *mut ([i32; 1], u16, usize, u32, f32, *mut f32),

},
Variant3{
fld0: [isize; 3],
fld1: Adt48,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: *mut f32,
fld1: [isize; 4],

},
Variant1{
fld0: bool,
fld1: Adt42,
fld2: Adt51,
fld3: i8,
fld4: u8,
fld5: [i32; 6],
fld6: Adt47,
fld7: [isize; 4],

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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: Adt48,
fld1: char,
fld2: u128,
fld3: [isize; 3],
fld4: [u128; 2],

},
Variant1{
fld0: Adt48,
fld1: Adt49,
fld2: Adt52,
fld3: (*mut u64,),
fld4: i16,
fld5: usize,
fld6: Adt50,
fld7: [i64; 5],

},
Variant2{
fld0: [i32; 6],
fld1: Adt56,
fld2: usize,
fld3: i128,
fld4: ([i64; 4], u128, *mut f32, f64),
fld5: u16,
fld6: Adt49,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt58::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: Adt56,
fld1: ([i32; 1], u16, usize, u32, f32, *mut f32),
fld2: Adt45,
fld3: [i32; 6],

},
Variant1{
fld0: i32,
fld1: usize,

}}

