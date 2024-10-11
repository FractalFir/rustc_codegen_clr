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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> Adt50 {
mir! {
type RET = Adt50;
let _15: Adt44;
let _16: Adt56;
let _17: u128;
let _18: [bool; 5];
let _19: Adt54;
let _20: (*const i8, f32, ([u16; 5], u64, u32, i64));
let _21: isize;
let _22: i64;
let _23: u128;
let _24: Adt57;
let _25: [i64; 6];
let _26: (u16, i64);
let _27: f64;
let _28: isize;
let _29: [isize; 3];
let _30: (i16,);
let _31: bool;
let _32: char;
let _33: isize;
let _34: (i16,);
let _35: u8;
let _36: (char, i16, [u16; 5], u64);
let _37: i32;
let _38: (i64, bool, bool, isize, (u16, i64), i64);
let _39: char;
let _40: f32;
let _41: *mut ([u16; 5], u64, u32, i64);
let _42: i32;
let _43: i8;
let _44: u64;
let _45: Adt50;
let _46: [bool; 5];
let _47: Adt51;
let _48: u64;
let _49: isize;
let _50: *const i64;
let _51: Adt48;
let _52: [u16; 5];
let _53: isize;
let _54: ([bool; 5],);
let _55: f32;
let _56: u8;
let _57: u64;
let _58: (char, i16, [u16; 5], u64);
let _59: isize;
let _60: (u16, i64);
let _61: bool;
let _62: isize;
let _63: isize;
let _64: u32;
let _65: (u8, f32, ((char, i16, [u16; 5], u64),), (i128,), [bool; 5], usize, f32);
let _66: [isize; 3];
let _67: [u8; 8];
let _68: (i16,);
let _69: (i64, bool, bool, isize, (u16, i64), i64);
let _70: isize;
let _71: (i128,);
let _72: ((char, i16, [u16; 5], u64),);
let _73: isize;
let _74: isize;
let _75: ();
let _76: ();
{
_6 = (-154192579_i32) << 29664_i16;
_1 = true;
_8 = 4514282106677528065581210816958744916_i128 >> _6;
_13 = !16837509956116291036_u64;
_11 = 49595_u16 ^ 42729_u16;
_9 = !11490736023536719592_usize;
_5 = 11904_i16;
_17 = 26594605852799406754901071085426424956_u128;
_7 = _9 as i64;
_4 = -85_i8;
_2 = '\u{811c1}';
_14 = (-37_isize) as u128;
_2 = '\u{2942b}';
_8 = 72303783621898926152722884626369958491_i128;
_12 = 3168989851_u32 >> _8;
_5 = _12 as i16;
_18 = [_1,_1,_1,_1,_1];
_11 = !9160_u16;
_5 = !26537_i16;
_20.2.0 = [_11,_11,_11,_11,_11];
Goto(bb1)
}
bb1 = {
_3 = (-31_isize) ^ 9223372036854775807_isize;
_20.2.0 = [_11,_11,_11,_11,_11];
_20.2.1 = _6 as u64;
_10 = _11 as u8;
_9 = _8 as usize;
_20.1 = _12 as f32;
_10 = 228_u8 - 112_u8;
_20.2.2 = _12 << _17;
_20.2.0 = [_11,_11,_11,_11,_11];
_22 = _7;
_18 = [_1,_1,_1,_1,_1];
_20.1 = _22 as f32;
_20.2.1 = !_13;
_21 = !_3;
_10 = _7 as u8;
_6 = _13 as i32;
_11 = !13472_u16;
_20.2.1 = _13 - _13;
_20.0 = core::ptr::addr_of!(_4);
_26.1 = _2 as i64;
match _8 {
0 => bb2,
1 => bb3,
2 => bb4,
72303783621898926152722884626369958491 => bb6,
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
_25 = [_7,_22,_26.1,_7,_7,_7];
_23 = _14;
_25 = [_26.1,_26.1,_26.1,_7,_7,_26.1];
_3 = -_21;
_22 = -_7;
_13 = !_20.2.1;
_13 = _20.2.1 + _20.2.1;
_12 = _3 as u32;
_14 = _17;
_10 = 209_u8 | 236_u8;
_13 = _9 as u64;
_22 = _7 * _7;
_20.0 = core::ptr::addr_of!(_4);
_9 = 0_usize;
_8 = 33521096763746549691971252367585931820_i128 - 72145097130509241408187458492980643398_i128;
_20.2.0[_9] = _11 | _11;
_28 = _20.2.1 as isize;
_26 = (_20.2.0[_9], _22);
_18[_9] = !_1;
_30.0 = _17 as i16;
_8 = 14936548144466016412239979181988688144_i128;
_7 = !_26.1;
_20.2.0[_9] = !_26.0;
_31 = _26.0 <= _26.0;
_20.0 = core::ptr::addr_of!(_4);
_8 = _21 as i128;
_1 = _31 ^ _31;
match _14 {
0 => bb1,
1 => bb5,
2 => bb7,
3 => bb8,
26594605852799406754901071085426424956 => bb10,
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
_22 = _26.1 >> _20.2.1;
_27 = _9 as f64;
_13 = _20.2.1;
_30.0 = _5 ^ _5;
_12 = _20.2.2;
_4 = -(-116_i8);
_20.2.0[_9] = !_11;
_29[_9] = _28;
_20.2.2 = !_12;
_13 = _21 as u64;
_7 = _31 as i64;
_20.2.2 = _12;
_30 = (_5,);
_29 = [_21,_3,_3];
_14 = !_17;
_6 = 1513107664_i32 - (-1010752494_i32);
_20.2.2 = _6 as u32;
_20.2.1 = _14 as u64;
_20.0 = core::ptr::addr_of!(_4);
_1 = _18[_9];
_25 = [_7,_26.1,_26.1,_7,_26.1,_7];
_11 = _20.2.0[_9] | _20.2.0[_9];
_13 = _25[_9] as u64;
_3 = -_28;
_1 = _31;
_8 = (-86871318335930362077447541795879409462_i128);
_14 = !_17;
_20.0 = core::ptr::addr_of!(_4);
match _8 {
0 => bb8,
1 => bb7,
2 => bb5,
253411048585008101385927065635888801994 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_18[_9] = !_1;
_5 = _30.0 - _30.0;
_20.1 = _22 as f32;
_20.0 = core::ptr::addr_of!(_4);
_20.2.0 = [_26.0,_26.0,_26.0,_11,_26.0];
_1 = _18[_9];
_25[_9] = _22;
_34.0 = _30.0 & _5;
_20.2.3 = _34.0 as i64;
_22 = _34.0 as i64;
_3 = _29[_9];
_2 = '\u{cd06c}';
_25 = [_20.2.3,_7,_26.1,_26.1,_7,_20.2.3];
_18[_9] = !_1;
_23 = !_17;
_22 = _26.1;
_33 = _3 ^ _28;
_36.2[_9] = _20.2.0[_9];
_20.1 = _17 as f32;
_9 = !3_usize;
_8 = (-65304765689160940239179902930238779119_i128);
Goto(bb13)
}
bb13 = {
_36.0 = _2;
_38.5 = _20.2.3;
_10 = 205_u8;
_38.0 = _7 * _26.1;
_20.2.3 = _6 as i64;
_29 = [_33,_33,_28];
_31 = _7 < _38.0;
_3 = _33;
_25 = [_38.0,_38.0,_38.0,_20.2.3,_7,_20.2.3];
_36.3 = _38.0 as u64;
_22 = _38.5 >> _7;
_37 = _6 | _6;
_36.3 = _13;
_38.4 = (_11, _20.2.3);
_26.0 = _38.4.0;
_38.1 = _31;
_40 = _20.1;
_20.0 = core::ptr::addr_of!(_43);
_23 = _4 as u128;
_38.4.1 = _7 | _20.2.3;
_37 = -_6;
_20.2.1 = !_36.3;
_28 = _33 + _21;
Goto(bb14)
}
bb14 = {
_38.4.0 = _11;
_39 = _36.0;
_26 = _38.4;
_18 = [_31,_31,_38.1,_31,_1];
_38.4.1 = _22 & _22;
_25 = [_38.4.1,_38.4.1,_26.1,_7,_38.4.1,_7];
_36.1 = _34.0;
_36 = (_2, _34.0, _20.2.0, _20.2.1);
_1 = _31;
_31 = !_38.1;
_27 = _17 as f64;
_20.1 = -_40;
_43 = _31 as i8;
_49 = _28;
_3 = _9 as isize;
_41 = core::ptr::addr_of_mut!(_20.2);
_6 = _37;
_39 = _2;
_36 = (_39, _34.0, (*_41).0, (*_41).1);
_9 = !1_usize;
Goto(bb15)
}
bb15 = {
_36 = (_2, _34.0, (*_41).0, _20.2.1);
match _17 {
0 => bb1,
26594605852799406754901071085426424956 => bb17,
_ => bb16
}
}
bb16 = {
Return()
}
bb17 = {
_42 = _9 as i32;
match _17 {
26594605852799406754901071085426424956 => bb18,
_ => bb4
}
}
bb18 = {
_38.3 = (*_41).3 as isize;
_8 = 92718064011545746642764036947166537818_i128;
(*_41).1 = _13 + _13;
(*_41).1 = _13 - _36.3;
_22 = (*_41).3 << (*_41).1;
_32 = _2;
_46 = [_1,_1,_1,_38.1,_38.1];
_38 = (_26.1, _1, _1, _49, _26, (*_41).3);
_48 = (*_41).1;
Call(_38.4.1 = core::intrinsics::bswap(_22), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
_22 = _26.1;
_35 = _10 | _10;
(*_41).2 = !_12;
_20.0 = core::ptr::addr_of!(_43);
_1 = !_38.1;
_38.3 = _49;
_39 = _36.0;
_21 = -_33;
_35 = _10 & _10;
_46 = [_1,_1,_38.1,_1,_1];
_6 = -_37;
_50 = core::ptr::addr_of!((*_41).3);
Goto(bb20)
}
bb20 = {
_50 = core::ptr::addr_of!((*_41).3);
_53 = !_33;
_38.3 = _38.1 as isize;
(*_41).1 = !_48;
_22 = _42 as i64;
_8 = (-21991759067657124011448488182739641958_i128);
_41 = core::ptr::addr_of_mut!((*_41));
_36.2 = [_11,_26.0,_11,_38.4.0,_26.0];
_40 = -_20.1;
_11 = _38.4.0;
_44 = !(*_41).1;
_52 = [_26.0,_11,_38.4.0,_26.0,_26.0];
_30.0 = _5;
_43 = -_4;
_37 = _6 & _6;
(*_41) = (_52, _48, _12, _7);
_38.2 = _1;
Call((*_41).3 = fn1(_36.2, _12, _38.4.0, _38.4.1, _25, _38, _25, _41, (*_41).1, _29, _44, _50, _29, _21, _4), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
(*_41).3 = _26.1;
_54.0 = [_38.2,_38.1,_1,_31,_1];
_8 = (-72264928383226567787919995171085830540_i128) >> _7;
_38.3 = !_33;
_26.0 = _38.4.0;
_38.5 = _38.0 ^ _38.0;
_53 = _21 ^ _49;
_38.4 = (_11, _7);
_1 = !_31;
_7 = (*_50) ^ (*_41).3;
_54.0 = _18;
_23 = _38.1 as u128;
_34.0 = _30.0;
_20.2 = (_36.2, _44, _12, _38.5);
(*_41).1 = !_44;
_5 = !_30.0;
_60.0 = _23 as u16;
_61 = !_1;
_52 = [_60.0,_60.0,_60.0,_60.0,_60.0];
_7 = (*_41).3;
_58.1 = _30.0;
_9 = !0_usize;
_26.1 = _38.5;
_62 = -_28;
Goto(bb22)
}
bb22 = {
_38.3 = _49;
_20.2.3 = -_38.5;
(*_41).2 = _36.1 as u32;
_62 = _53 << (*_50);
_25 = [(*_41).3,_7,(*_50),(*_50),_7,_7];
_48 = !_13;
_22 = !(*_50);
_20.2.2 = _12;
_23 = !_14;
_43 = _8 as i8;
_39 = _2;
Call(_38.4.0 = core::intrinsics::bswap(_60.0), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
_52 = [_60.0,_38.4.0,_26.0,_60.0,_60.0];
_65.0 = _10;
_20.0 = core::ptr::addr_of!(_43);
_58.1 = _28 as i16;
_28 = _49 << _53;
_58.3 = _27 as u64;
match _17 {
26594605852799406754901071085426424956 => bb25,
_ => bb24
}
}
bb24 = {
_25 = [_7,_22,_26.1,_7,_7,_7];
_23 = _14;
_25 = [_26.1,_26.1,_26.1,_7,_7,_26.1];
_3 = -_21;
_22 = -_7;
_13 = !_20.2.1;
_13 = _20.2.1 + _20.2.1;
_12 = _3 as u32;
_14 = _17;
_10 = 209_u8 | 236_u8;
_13 = _9 as u64;
_22 = _7 * _7;
_20.0 = core::ptr::addr_of!(_4);
_9 = 0_usize;
_8 = 33521096763746549691971252367585931820_i128 - 72145097130509241408187458492980643398_i128;
_20.2.0[_9] = _11 | _11;
_28 = _20.2.1 as isize;
_26 = (_20.2.0[_9], _22);
_18[_9] = !_1;
_30.0 = _17 as i16;
_8 = 14936548144466016412239979181988688144_i128;
_7 = !_26.1;
_20.2.0[_9] = !_26.0;
_31 = _26.0 <= _26.0;
_20.0 = core::ptr::addr_of!(_4);
_8 = _21 as i128;
_1 = _31 ^ _31;
match _14 {
0 => bb1,
1 => bb5,
2 => bb7,
3 => bb8,
26594605852799406754901071085426424956 => bb10,
_ => bb9
}
}
bb25 = {
_38.4 = (_60.0, (*_41).3);
_63 = _62;
_35 = !_10;
_58.2 = _52;
_65.1 = _40;
_60.1 = !_20.2.3;
_6 = _37 << _63;
_38.4.1 = _38.5;
_36 = (_39, _34.0, _52, _20.2.1);
_20.1 = _9 as f32;
_64 = _17 as u32;
_28 = _62;
_22 = _20.1 as i64;
_29 = [_53,_53,_28];
_65.0 = !_10;
_44 = (*_41).1;
match _17 {
0 => bb26,
1 => bb27,
2 => bb28,
3 => bb29,
4 => bb30,
5 => bb31,
26594605852799406754901071085426424956 => bb33,
_ => bb32
}
}
bb26 = {
_25 = [_7,_22,_26.1,_7,_7,_7];
_23 = _14;
_25 = [_26.1,_26.1,_26.1,_7,_7,_26.1];
_3 = -_21;
_22 = -_7;
_13 = !_20.2.1;
_13 = _20.2.1 + _20.2.1;
_12 = _3 as u32;
_14 = _17;
_10 = 209_u8 | 236_u8;
_13 = _9 as u64;
_22 = _7 * _7;
_20.0 = core::ptr::addr_of!(_4);
_9 = 0_usize;
_8 = 33521096763746549691971252367585931820_i128 - 72145097130509241408187458492980643398_i128;
_20.2.0[_9] = _11 | _11;
_28 = _20.2.1 as isize;
_26 = (_20.2.0[_9], _22);
_18[_9] = !_1;
_30.0 = _17 as i16;
_8 = 14936548144466016412239979181988688144_i128;
_7 = !_26.1;
_20.2.0[_9] = !_26.0;
_31 = _26.0 <= _26.0;
_20.0 = core::ptr::addr_of!(_4);
_8 = _21 as i128;
_1 = _31 ^ _31;
match _14 {
0 => bb1,
1 => bb5,
2 => bb7,
3 => bb8,
26594605852799406754901071085426424956 => bb10,
_ => bb9
}
}
bb27 = {
Return()
}
bb28 = {
Return()
}
bb29 = {
(*_41).3 = _26.1;
_54.0 = [_38.2,_38.1,_1,_31,_1];
_8 = (-72264928383226567787919995171085830540_i128) >> _7;
_38.3 = !_33;
_26.0 = _38.4.0;
_38.5 = _38.0 ^ _38.0;
_53 = _21 ^ _49;
_38.4 = (_11, _7);
_1 = !_31;
_7 = (*_50) ^ (*_41).3;
_54.0 = _18;
_23 = _38.1 as u128;
_34.0 = _30.0;
_20.2 = (_36.2, _44, _12, _38.5);
(*_41).1 = !_44;
_5 = !_30.0;
_60.0 = _23 as u16;
_61 = !_1;
_52 = [_60.0,_60.0,_60.0,_60.0,_60.0];
_7 = (*_41).3;
_58.1 = _30.0;
_9 = !0_usize;
_26.1 = _38.5;
_62 = -_28;
Goto(bb22)
}
bb30 = {
Return()
}
bb31 = {
_36 = (_2, _34.0, (*_41).0, _20.2.1);
match _17 {
0 => bb1,
26594605852799406754901071085426424956 => bb17,
_ => bb16
}
}
bb32 = {
_38.3 = (*_41).3 as isize;
_8 = 92718064011545746642764036947166537818_i128;
(*_41).1 = _13 + _13;
(*_41).1 = _13 - _36.3;
_22 = (*_41).3 << (*_41).1;
_32 = _2;
_46 = [_1,_1,_1,_38.1,_38.1];
_38 = (_26.1, _1, _1, _49, _26, (*_41).3);
_48 = (*_41).1;
Call(_38.4.1 = core::intrinsics::bswap(_22), ReturnTo(bb19), UnwindUnreachable())
}
bb33 = {
_65.2.0.1 = -_36.1;
(*_41) = (_58.2, _36.3, _12, _26.1);
_21 = !_53;
_65.4 = [_38.2,_38.1,_31,_61,_38.1];
_45 = Adt50::Variant2 { fld0: _38,fld1: _50,fld2: _41,fld3: (*_41),fld4: _29,fld5: _36 };
_38.4 = (_11, (*_41).3);
match _10 {
0 => bb32,
1 => bb5,
205 => bb35,
_ => bb34
}
}
bb34 = {
_36 = (_2, _34.0, (*_41).0, _20.2.1);
match _17 {
0 => bb1,
26594605852799406754901071085426424956 => bb17,
_ => bb16
}
}
bb35 = {
_45 = Adt50::Variant2 { fld0: _38,fld1: _50,fld2: _41,fld3: _20.2,fld4: _29,fld5: _36 };
_69.1 = (*_50) != _26.1;
_68 = (_30.0,);
_65.2.0.0 = _36.0;
_65.2.0.2 = [_38.4.0,_38.4.0,_60.0,_11,_60.0];
_12 = !(*_41).2;
_69 = _38;
Goto(bb36)
}
bb36 = {
_67 = [_65.0,_35,_35,_10,_10,_35,_65.0,_10];
_30 = (_65.2.0.1,);
_27 = _43 as f64;
_69.4 = _38.4;
_36.0 = _2;
_20.2 = (_52, Field::<([u16; 5], u64, u32, i64)>(Variant(_45, 2), 3).1, Field::<([u16; 5], u64, u32, i64)>(Variant(_45, 2), 3).2, _60.1);
_30.0 = _9 as i16;
Goto(bb37)
}
bb37 = {
_65.3 = (_8,);
_58 = (_2, _65.2.0.1, Field::<([u16; 5], u64, u32, i64)>(Variant(_45, 2), 3).0, (*_41).1);
match _10 {
0 => bb22,
1 => bb29,
2 => bb8,
3 => bb16,
4 => bb5,
205 => bb38,
_ => bb13
}
}
bb38 = {
RET = Move(_45);
_34.0 = _43 as i16;
_71.0 = -_8;
(*_41) = (_36.2, _36.3, Field::<([u16; 5], u64, u32, i64)>(Variant(RET, 2), 3).2, _7);
_72.0.0 = _58.0;
place!(Field::<(char, i16, [u16; 5], u64)>(Variant(RET, 2), 5)).2 = [_26.0,_60.0,Field::<(i64, bool, bool, isize, (u16, i64), i64)>(Variant(RET, 2), 0).4.0,_60.0,_60.0];
_17 = _23 >> _20.2.3;
place!(Field::<*const i64>(Variant(RET, 2), 1)) = core::ptr::addr_of!(place!(Field::<([u16; 5], u64, u32, i64)>(Variant(RET, 2), 3)).3);
_69 = (_26.1, Field::<(i64, bool, bool, isize, (u16, i64), i64)>(Variant(RET, 2), 0).2, _38.2, _28, _60, _60.1);
place!(Field::<([u16; 5], u64, u32, i64)>(Variant(RET, 2), 3)).0 = Field::<(char, i16, [u16; 5], u64)>(Variant(RET, 2), 5).2;
Goto(bb39)
}
bb39 = {
Call(_75 = dump_var(0_usize, 48_usize, Move(_48), 3_usize, Move(_3), 36_usize, Move(_36), 44_usize, Move(_44)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Call(_75 = dump_var(0_usize, 71_usize, Move(_71), 35_usize, Move(_35), 1_usize, Move(_1), 17_usize, Move(_17)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Call(_75 = dump_var(0_usize, 6_usize, Move(_6), 26_usize, Move(_26), 4_usize, Move(_4), 28_usize, Move(_28)), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Call(_75 = dump_var(0_usize, 60_usize, Move(_60), 67_usize, Move(_67), 21_usize, Move(_21), 61_usize, Move(_61)), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
Call(_75 = dump_var(0_usize, 18_usize, Move(_18), 49_usize, Move(_49), 22_usize, Move(_22), 12_usize, Move(_12)), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Call(_75 = dump_var(0_usize, 25_usize, Move(_25), 68_usize, Move(_68), 2_usize, Move(_2), 54_usize, Move(_54)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
Call(_75 = dump_var(0_usize, 31_usize, Move(_31), 34_usize, Move(_34), 76_usize, _76, 76_usize, _76), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: [u16; 5],mut _2: u32,mut _3: u16,mut _4: i64,mut _5: [i64; 6],mut _6: (i64, bool, bool, isize, (u16, i64), i64),mut _7: [i64; 6],mut _8: *mut ([u16; 5], u64, u32, i64),mut _9: u64,mut _10: [isize; 3],mut _11: u64,mut _12: *const i64,mut _13: [isize; 3],mut _14: isize,mut _15: i8) -> i64 {
mir! {
type RET = i64;
let _16: ((char, i16, [u16; 5], u64),);
let _17: f32;
let _18: isize;
let _19: i128;
let _20: char;
let _21: i32;
let _22: u16;
let _23: u32;
let _24: i64;
let _25: (u8, f32, ((char, i16, [u16; 5], u64),), (i128,), [bool; 5], usize, f32);
let _26: (u16, i64);
let _27: (u16, i64);
let _28: isize;
let _29: char;
let _30: isize;
let _31: (i16,);
let _32: ();
let _33: ();
{
_7 = [_6.0,_6.4.1,_6.0,_4,_4,_6.4.1];
_7 = [_4,_6.4.1,_6.4.1,_6.4.1,_4,_6.5];
_2 = 110_u8 as u32;
_1 = [_3,_6.4.0,_6.4.0,_3,_6.4.0];
Goto(bb1)
}
bb1 = {
_6.4 = (_3, _6.5);
_11 = !(*_8).1;
(*_8).2 = !_2;
_6.0 = _6.5;
_13 = [_6.3,_14,_14];
RET = -_4;
(*_8).1 = _9 & _11;
_12 = core::ptr::addr_of!(RET);
_16.0.0 = '\u{a493f}';
RET = _11 as i64;
_3 = _6.4.0 & _6.4.0;
_6.3 = !_14;
_11 = !(*_8).1;
(*_8).1 = !_11;
_18 = _14 & _14;
Call(_15 = fn2(_13, _18, (*_8).1, _7, _4, _18, (*_8).1, RET, _10, _6.3, (*_8).1, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = [_3,_3,_3,_3,_3];
_1 = [_6.4.0,_3,_3,_6.4.0,_3];
_16.0.3 = (*_8).1;
_14 = _6.3 << (*_8).1;
_14 = _18 >> (*_8).1;
_16.0.1 = (*_8).2 as i16;
RET = _4;
_5 = _7;
_17 = _9 as f32;
_5 = [(*_12),_6.5,(*_12),(*_12),(*_12),(*_12)];
RET = !_4;
_10 = [_14,_14,_14];
_4 = -(*_12);
_6.4 = (_3, (*_12));
_6.4.1 = _4 + (*_12);
Call(_19 = fn3(_5, _6.4.1, _6.0, _14, _13, (*_8).1, _9, _10, _10, _6.4.1, _6, (*_8).1, _14, _16.0.3, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6.5 = !_4;
_6.2 = _6.1;
_12 = core::ptr::addr_of!(_4);
Call(_2 = core::intrinsics::transmute(_16.0.0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_9 = !_16.0.3;
_20 = _16.0.0;
_16.0 = (_20, 9745_i16, _1, _11);
_3 = _2 as u16;
(*_12) = !_6.4.1;
_18 = _11 as isize;
_16.0.0 = _20;
_18 = _6.4.0 as isize;
_17 = _16.0.1 as f32;
_25.0 = !36_u8;
_21 = !1560932361_i32;
_6.1 = !_6.2;
(*_8).1 = _16.0.3 * _9;
(*_8).0 = _1;
_25.3.0 = _19;
_25.2.0.3 = (*_8).1 >> (*_8).1;
_25.2.0.2 = [_6.4.0,_6.4.0,_6.4.0,_6.4.0,_3];
_25.3.0 = _19;
_6.4.1 = _4 << RET;
_23 = _2 << _14;
Goto(bb5)
}
bb5 = {
_25.2 = (_16.0,);
_27.1 = -(*_12);
_25.2.0 = (_20, _16.0.1, _1, _9);
_13 = [_6.3,_14,_14];
_16.0.0 = _25.2.0.0;
_21 = (-68361653_i32);
_6.0 = (*_12);
(*_8).1 = !_16.0.3;
(*_8).1 = !_9;
_26.1 = _27.1;
_11 = _9 - (*_8).1;
_19 = _25.3.0;
_27 = _6.4;
_24 = -_4;
_16 = (_25.2.0,);
(*_8).0 = [_27.0,_6.4.0,_27.0,_6.4.0,_27.0];
_29 = _16.0.0;
_1 = [_6.4.0,_6.4.0,_6.4.0,_3,_27.0];
_16 = (_25.2.0,);
_25.3 = (_19,);
_17 = _25.3.0 as f32;
RET = !_6.0;
_25.1 = _17 + _17;
_31.0 = _16.0.1;
_25.4 = [_6.2,_6.1,_6.1,_6.1,_6.1];
_28 = _14;
_30 = _25.3.0 as isize;
RET = _24 ^ (*_12);
Goto(bb6)
}
bb6 = {
Call(_32 = dump_var(1_usize, 4_usize, Move(_4), 18_usize, Move(_18), 16_usize, Move(_16), 15_usize, Move(_15)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_32 = dump_var(1_usize, 28_usize, Move(_28), 7_usize, Move(_7), 19_usize, Move(_19), 20_usize, Move(_20)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_32 = dump_var(1_usize, 27_usize, Move(_27), 14_usize, Move(_14), 24_usize, Move(_24), 1_usize, Move(_1)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: [isize; 3],mut _2: isize,mut _3: u64,mut _4: [i64; 6],mut _5: i64,mut _6: isize,mut _7: u64,mut _8: i64,mut _9: [isize; 3],mut _10: isize,mut _11: u64,mut _12: [i64; 6]) -> i8 {
mir! {
type RET = i8;
let _13: f64;
let _14: [bool; 5];
let _15: Adt42;
let _16: Adt48;
let _17: Adt47;
let _18: i64;
let _19: u32;
let _20: f64;
let _21: i64;
let _22: f64;
let _23: i128;
let _24: [u16; 5];
let _25: ();
let _26: ();
{
_2 = _10 & _6;
RET = 65_i8;
_5 = _8 | _8;
_7 = 298956257315189296430846300672489036397_u128 as u64;
_2 = false as isize;
_4 = [_8,_8,_8,_5,_5,_5];
_2 = !_6;
_3 = !_11;
_4 = [_8,_5,_5,_5,_8,_8];
_12 = [_5,_5,_5,_8,_5,_5];
_4 = [_5,_5,_5,_5,_8,_8];
_10 = 6_usize as isize;
_11 = _3 & _3;
RET = (-11_i8);
RET = 68_i8;
_14 = [true,false,false,true,true];
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
68 => bb8,
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
_11 = !_3;
_6 = !_2;
_6 = _2;
_5 = _8 ^ _8;
_13 = 55532403592156901459217007115678216654_i128 as f64;
RET = (-65_i8);
Goto(bb9)
}
bb9 = {
RET = 81_i8 * (-71_i8);
_7 = !_11;
Goto(bb10)
}
bb10 = {
_9 = [_2,_6,_2];
_12 = _4;
_3 = !_7;
_13 = _5 as f64;
_11 = !_3;
_6 = _10;
RET = !10_i8;
_14 = [true,true,true,false,true];
_12 = [_8,_5,_5,_8,_5,_5];
_6 = _2 + _2;
RET = (-101_i8);
_13 = (-15122_i16) as f64;
_3 = _11;
_13 = (-166570797275548226059019495590653389222_i128) as f64;
_6 = _2 + _2;
_1 = [_2,_2,_2];
_13 = 543838982_u32 as f64;
_7 = _11;
_11 = true as u64;
_11 = _3 + _3;
_14 = [true,false,false,false,false];
_13 = (-540523516_i32) as f64;
match RET {
0 => bb4,
1 => bb3,
2 => bb11,
3 => bb12,
340282366920938463463374607431768211355 => bb14,
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
_1 = [_6,_6,_2];
_8 = -_5;
_7 = _3;
_16 = Adt48::Variant3 { fld0: _2 };
RET = _13 as i8;
SetDiscriminant(_16, 1);
place!(Field::<(u8, f32, ((char, i16, [u16; 5], u64),), (i128,), [bool; 5], usize, f32)>(Variant(_16, 1), 3)).2.0.3 = 300442408394768562119160490104540791354_u128 as u64;
place!(Field::<(u8, f32, ((char, i16, [u16; 5], u64),), (i128,), [bool; 5], usize, f32)>(Variant(_16, 1), 3)).2.0.0 = '\u{492be}';
place!(Field::<(u8, f32, ((char, i16, [u16; 5], u64),), (i128,), [bool; 5], usize, f32)>(Variant(_16, 1), 3)).0 = 67_u8;
place!(Field::<(u8, f32, ((char, i16, [u16; 5], u64),), (i128,), [bool; 5], usize, f32)>(Variant(_16, 1), 3)).1 = _8 as f32;
place!(Field::<(u8, f32, ((char, i16, [u16; 5], u64),), (i128,), [bool; 5], usize, f32)>(Variant(_16, 1), 3)).3 = (147528014939952705267760217077474737980_i128,);
place!(Field::<i128>(Variant(_16, 1), 4)) = !Field::<(u8, f32, ((char, i16, [u16; 5], u64),), (i128,), [bool; 5], usize, f32)>(Variant(_16, 1), 3).3.0;
place!(Field::<(u8, f32, ((char, i16, [u16; 5], u64),), (i128,), [bool; 5], usize, f32)>(Variant(_16, 1), 3)).2.0.1 = 24444_i16;
place!(Field::<(u8, f32, ((char, i16, [u16; 5], u64),), (i128,), [bool; 5], usize, f32)>(Variant(_16, 1), 3)).4 = [true,false,true,true,true];
place!(Field::<u32>(Variant(_16, 1), 1)) = Field::<(u8, f32, ((char, i16, [u16; 5], u64),), (i128,), [bool; 5], usize, f32)>(Variant(_16, 1), 3).1 as u32;
_13 = Field::<(u8, f32, ((char, i16, [u16; 5], u64),), (i128,), [bool; 5], usize, f32)>(Variant(_16, 1), 3).2.0.3 as f64;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(2_usize, 12_usize, Move(_12), 5_usize, Move(_5), 3_usize, Move(_3), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(2_usize, 4_usize, Move(_4), 1_usize, Move(_1), 26_usize, _26, 26_usize, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: [i64; 6],mut _2: i64,mut _3: i64,mut _4: isize,mut _5: [isize; 3],mut _6: u64,mut _7: u64,mut _8: [isize; 3],mut _9: [isize; 3],mut _10: i64,mut _11: (i64, bool, bool, isize, (u16, i64), i64),mut _12: u64,mut _13: isize,mut _14: u64,mut _15: [i64; 6]) -> i128 {
mir! {
type RET = i128;
let _16: (i128,);
let _17: (u16, i64);
let _18: u16;
let _19: Adt53;
let _20: u32;
let _21: ([bool; 5],);
let _22: *const (bool, i64, (i64, bool, bool, isize, (u16, i64), i64));
let _23: isize;
let _24: *const i8;
let _25: [i64; 6];
let _26: char;
let _27: ();
let _28: ();
{
_15 = [_11.4.1,_11.4.1,_2,_2,_11.4.1,_11.4.1];
Call(RET = core::intrinsics::bswap(63850028309045946933692776420580691603_i128), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_15 = _1;
_3 = _2 * _2;
_11.4.1 = _10 | _3;
_11.1 = !_11.2;
_2 = -_11.4.1;
_11.5 = !_2;
_10 = _3;
_11.5 = !_10;
_2 = _3 + _11.5;
_11.4 = (15980_u16, _3);
_4 = !_13;
_11.4.0 = 52980_u16;
_16.0 = (-38494136018209814595900539591930986667_i128) + 12984731195809778946571569288885153943_i128;
_11.4 = (5603_u16, _3);
_7 = _14;
_11.3 = !_4;
Call(_15 = fn4(_11, _7, _11.3, _8, _11.4.0, _7, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = 1157726413_u32 as i128;
_4 = -_13;
_10 = _11.1 as i64;
_16.0 = RET;
Goto(bb3)
}
bb3 = {
_7 = _6 & _14;
_1 = [_11.4.1,_10,_2,_11.5,_11.5,_2];
_18 = _11.4.0 | _11.4.0;
_11.4 = (_18, _2);
_15 = [_11.4.1,_11.4.1,_2,_2,_11.4.1,_2];
_11.2 = _11.1;
_11.4.1 = _10 << _4;
_17.0 = _11.1 as u16;
_18 = _11.4.0;
_11.1 = !_11.2;
_11.2 = _11.1;
Goto(bb4)
}
bb4 = {
_11.4.0 = _17.0;
_21.0 = [_11.2,_11.1,_11.2,_11.2,_11.2];
_5 = [_4,_11.3,_11.3];
_11.3 = _13;
_20 = !2062502934_u32;
_10 = -_2;
_3 = (-81_i8) as i64;
_10 = _11.5 - _2;
_2 = _10 >> _7;
_17.1 = -_10;
_11.4 = (_18, _2);
_9 = [_4,_4,_11.3];
_13 = _4;
_11.3 = _4;
_2 = -_11.5;
_4 = !_11.3;
_14 = _6;
RET = -_16.0;
_11.2 = !_11.1;
_18 = !_11.4.0;
RET = _11.3 as i128;
_11.4 = (_17.0, _17.1);
_1 = [_10,_11.4.1,_17.1,_11.4.1,_10,_11.4.1];
Goto(bb5)
}
bb5 = {
Call(_27 = dump_var(3_usize, 13_usize, Move(_13), 9_usize, Move(_9), 10_usize, Move(_10), 2_usize, Move(_2)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_27 = dump_var(3_usize, 12_usize, Move(_12), 5_usize, Move(_5), 17_usize, Move(_17), 4_usize, Move(_4)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_27 = dump_var(3_usize, 14_usize, Move(_14), 11_usize, Move(_11), 28_usize, _28, 28_usize, _28), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: (i64, bool, bool, isize, (u16, i64), i64),mut _2: u64,mut _3: isize,mut _4: [isize; 3],mut _5: u16,mut _6: u64,mut _7: isize) -> [i64; 6] {
mir! {
type RET = [i64; 6];
let _8: (u16, i64);
let _9: bool;
let _10: Adt49;
let _11: Adt55;
let _12: isize;
let _13: ();
let _14: ();
{
_1.1 = !_1.2;
_1.2 = _1.1;
_6 = _1.4.0 as u64;
_1.2 = _1.4.1 < _1.4.1;
_8.1 = _1.4.1 >> _6;
_1.1 = !_1.2;
_8.0 = !_5;
Call(_1.5 = fn5(_1.3, _1.4.0, _1.4.1, _1.2, _7, _1.4, _6, _7, _1.2, _8.0, _8, _8.1, _8.1, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = _1.3;
_1.1 = _1.4.1 >= _1.5;
_2 = 4_usize as u64;
_1.1 = _1.2 | _1.2;
_9 = _1.2;
_8.1 = _1.4.1;
_1.2 = _1.3 >= _3;
_8.0 = 165682586150753115673090091526457029278_u128 as u16;
_1.3 = _7;
_4 = [_7,_7,_7];
_2 = !_6;
_8.0 = 141236921033893446494572671142914194553_i128 as u16;
RET = [_1.5,_1.5,_1.4.1,_1.4.1,_1.5,_1.5];
_1.1 = _1.2;
_1.1 = _2 != _2;
_6 = 0_usize as u64;
_12 = -_1.3;
_8.0 = _1.4.0;
_1.5 = _1.4.1;
_1.0 = _12 as i64;
_8 = (_5, _1.4.1);
Goto(bb2)
}
bb2 = {
Call(_13 = dump_var(4_usize, 7_usize, Move(_7), 6_usize, Move(_6), 9_usize, Move(_9), 4_usize, Move(_4)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_13 = dump_var(4_usize, 1_usize, Move(_1), 14_usize, _14, 14_usize, _14, 14_usize, _14), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: isize,mut _2: u16,mut _3: i64,mut _4: bool,mut _5: isize,mut _6: (u16, i64),mut _7: u64,mut _8: isize,mut _9: bool,mut _10: u16,mut _11: (u16, i64),mut _12: i64,mut _13: i64,mut _14: [isize; 3]) -> i64 {
mir! {
type RET = i64;
let _15: ();
let _16: ();
{
_7 = 11891495386408359820_u64 << _10;
_10 = _2;
_6.0 = _2;
match _2 {
0 => bb1,
5603 => bb3,
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
RET = _11.1;
_8 = !_1;
_9 = _3 < _3;
_6.0 = _11.0 | _2;
_14 = [_5,_8,_5];
_8 = -_1;
_5 = 16030_i16 as isize;
_7 = !8008754405830457112_u64;
_11 = _6;
_11.1 = _11.0 as i64;
_10 = _9 as u16;
_12 = RET;
_8 = 3_usize as isize;
_4 = !_9;
RET = _6.0 as i64;
Goto(bb4)
}
bb4 = {
Call(_15 = dump_var(5_usize, 3_usize, Move(_3), 6_usize, Move(_6), 13_usize, Move(_13), 2_usize, Move(_2)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_15 = dump_var(5_usize, 7_usize, Move(_7), 12_usize, Move(_12), 11_usize, Move(_11), 16_usize, _16), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{ed6eb}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box(53_i8), std::hint::black_box(24912_i16), std::hint::black_box((-2082714130_i32)), std::hint::black_box((-2178653222030060178_i64)), std::hint::black_box((-60799118486795366788512254332068105336_i128)), std::hint::black_box(5357774173086875411_usize), std::hint::black_box(70_u8), std::hint::black_box(5_u16), std::hint::black_box(915936148_u32), std::hint::black_box(5391954362017035788_u64), std::hint::black_box(310808197739277742925050984305553302596_u128));
                
            }
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt41::".as_ptr()  as *const c_char)};match self{
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
fld0: (u16, i64),
fld1: *const (bool, i64, (i64, bool, bool, isize, (u16, i64), i64)),
fld2: [i64; 6],
fld3: f64,

},
Variant1{
fld0: (bool, i64, (i64, bool, bool, isize, (u16, i64), i64)),
fld1: u16,
fld2: isize,
fld3: (i128,),

},
Variant2{
fld0: bool,
fld1: ([u16; 5], u64, u32, i64),
fld2: ((i128,), *mut ([u16; 5], u64, u32, i64), [isize; 3]),
fld3: (bool, i64, (i64, bool, bool, isize, (u16, i64), i64)),

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: ((char, i16, [u16; 5], u64),),
fld1: char,
fld2: (i16,),
fld3: (*const i8, f32, ([u16; 5], u64, u32, i64)),
fld4: (char, i16, [u16; 5], u64),
fld5: (i128,),
fld6: (u8, f32, ((char, i16, [u16; 5], u64),), (i128,), [bool; 5], usize, f32),
fld7: ((i128,), *mut ([u16; 5], u64, u32, i64), [isize; 3]),

},
Variant1{
fld0: *mut ([u16; 5], u64, u32, i64),

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: [u16; 5],

},
Variant1{
fld0: [u16; 5],
fld1: (bool, i64, (i64, bool, bool, isize, (u16, i64), i64)),
fld2: [i64; 6],
fld3: i8,
fld4: *const i64,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
fld0: [u8; 8],
fld1: (char, i16, [u16; 5], u64),
fld2: usize,

},
Variant1{
fld0: ((char, i16, [u16; 5], u64),),
fld1: [isize; 3],
fld2: *mut ([u16; 5], u64, u32, i64),

},
Variant2{
fld0: *mut ([u16; 5], u64, u32, i64),
fld1: Adt41,
fld2: (i128,),
fld3: (*const i8, f32, ([u16; 5], u64, u32, i64)),
fld4: i16,
fld5: ((char, i16, [u16; 5], u64),),
fld6: *mut [u16; 5],
fld7: i128,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: bool,
fld1: u32,
fld2: Adt42,
fld3: i8,
fld4: i128,
fld5: Adt43,
fld6: ([bool; 5],),
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
fld0: *mut [u16; 5],
fld1: (i128,),
fld2: Adt43,
fld3: i8,
fld4: u64,
fld5: f64,

},
Variant1{
fld0: (u16, i64),
fld1: Adt45,
fld2: (i128,),

},
Variant2{
fld0: u32,
fld1: f64,
fld2: ((i128,), *mut ([u16; 5], u64, u32, i64), [isize; 3]),
fld3: Adt41,
fld4: (bool, i64, (i64, bool, bool, isize, (u16, i64), i64)),
fld5: u64,
fld6: (i16,),
fld7: i128,

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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: bool,
fld1: [u8; 8],
fld2: (i64, bool, bool, isize, (u16, i64), i64),
fld3: *const i8,
fld4: [u16; 5],
fld5: i32,
fld6: ((i128,), *mut ([u16; 5], u64, u32, i64), [isize; 3]),

},
Variant1{
fld0: (i64, bool, bool, isize, (u16, i64), i64),
fld1: char,
fld2: Adt43,
fld3: usize,

},
Variant2{
fld0: bool,
fld1: *mut ([u16; 5], u64, u32, i64),

},
Variant3{
fld0: i128,
fld1: (bool, i64, (i64, bool, bool, isize, (u16, i64), i64)),

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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: ((char, i16, [u16; 5], u64),),
fld1: *const i8,

},
Variant1{
fld0: [bool; 5],
fld1: u32,
fld2: [u8; 8],
fld3: (u8, f32, ((char, i16, [u16; 5], u64),), (i128,), [bool; 5], usize, f32),
fld4: i128,
fld5: Adt44,
fld6: [i64; 6],

},
Variant2{
fld0: (i64, bool, bool, isize, (u16, i64), i64),
fld1: u16,
fld2: *const (bool, i64, (i64, bool, bool, isize, (u16, i64), i64)),
fld3: i8,
fld4: *const i64,
fld5: Adt44,
fld6: (u16, i64),
fld7: usize,

},
Variant3{
fld0: isize,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: f64,
fld1: Adt43,
fld2: *mut [u16; 5],
fld3: i8,

},
Variant1{
fld0: [bool; 5],
fld1: (i64, bool, bool, isize, (u16, i64), i64),
fld2: u8,
fld3: *const i8,
fld4: Adt47,
fld5: Adt41,
fld6: f32,
fld7: usize,

},
Variant2{
fld0: [isize; 3],
fld1: Adt45,
fld2: *mut [u16; 5],
fld3: i16,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: ([bool; 5],),
fld1: (bool, i64, (i64, bool, bool, isize, (u16, i64), i64)),
fld2: [u16; 5],

},
Variant1{
fld0: ([bool; 5],),
fld1: [u16; 5],
fld2: isize,
fld3: (bool, i64, (i64, bool, bool, isize, (u16, i64), i64)),
fld4: (i64, bool, bool, isize, (u16, i64), i64),
fld5: Adt49,
fld6: u8,

},
Variant2{
fld0: (i64, bool, bool, isize, (u16, i64), i64),
fld1: *const i64,
fld2: *mut ([u16; 5], u64, u32, i64),
fld3: ([u16; 5], u64, u32, i64),
fld4: [isize; 3],
fld5: (char, i16, [u16; 5], u64),

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
fld0: usize,

},
Variant1{
fld0: bool,
fld1: (i64, bool, bool, isize, (u16, i64), i64),
fld2: [u16; 5],
fld3: u32,
fld4: i16,
fld5: (u8, f32, ((char, i16, [u16; 5], u64),), (i128,), [bool; 5], usize, f32),

},
Variant2{
fld0: *mut [u16; 5],
fld1: (bool, i64, (i64, bool, bool, isize, (u16, i64), i64)),
fld2: Adt45,
fld3: ([u16; 5], u64, u32, i64),
fld4: *const (bool, i64, (i64, bool, bool, isize, (u16, i64), i64)),
fld5: (u16, i64),
fld6: Adt42,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: Adt41,
fld1: ((i128,), *mut ([u16; 5], u64, u32, i64), [isize; 3]),
fld2: isize,
fld3: [u16; 5],

},
Variant1{
fld0: [u8; 8],
fld1: [isize; 3],
fld2: (i64, bool, bool, isize, (u16, i64), i64),
fld3: (char, i16, [u16; 5], u64),
fld4: i16,
fld5: *const i64,
fld6: [i64; 6],

},
Variant2{
fld0: (*const i8, f32, ([u16; 5], u64, u32, i64)),
fld1: *mut [u16; 5],
fld2: u32,
fld3: *const i64,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: Adt42,
fld1: char,
fld2: Adt47,
fld3: u16,
fld4: (u16, i64),
fld5: Adt51,

},
Variant1{
fld0: f64,
fld1: (i128,),
fld2: u64,
fld3: [bool; 5],

},
Variant2{
fld0: Adt46,
fld1: Adt45,
fld2: Adt47,

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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: bool,
fld1: [u8; 8],
fld2: Adt47,
fld3: (bool, i64, (i64, bool, bool, isize, (u16, i64), i64)),
fld4: Adt50,
fld5: Adt42,
fld6: Adt44,
fld7: i128,

},
Variant1{
fld0: u64,
fld1: char,
fld2: ([bool; 5],),
fld3: Adt46,
fld4: (i16,),
fld5: i32,
fld6: Adt50,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: ((i128,), *mut ([u16; 5], u64, u32, i64), [isize; 3]),

},
Variant1{
fld0: Adt51,
fld1: Adt50,
fld2: *const (bool, i64, (i64, bool, bool, isize, (u16, i64), i64)),
fld3: (*const i8, f32, ([u16; 5], u64, u32, i64)),
fld4: u128,
fld5: [u8; 8],
fld6: Adt53,

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
fld0: (u8, f32, ((char, i16, [u16; 5], u64),), (i128,), [bool; 5], usize, f32),
fld1: i32,
fld2: f64,

},
Variant1{
fld0: *const i8,
fld1: char,
fld2: u64,
fld3: Adt48,
fld4: (*const i8, f32, ([u16; 5], u64, u32, i64)),
fld5: f32,
fld6: i64,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: Adt46,
fld1: *const i64,
fld2: f64,
fld3: Adt53,
fld4: i64,
fld5: i32,

},
Variant1{
fld0: Adt56,
fld1: Adt41,
fld2: ((i128,), *mut ([u16; 5], u64, u32, i64), [isize; 3]),

},
Variant2{
fld0: ([u16; 5], u64, u32, i64),
fld1: *mut ([u16; 5], u64, u32, i64),
fld2: Adt46,

},
Variant3{
fld0: u16,
fld1: Adt49,
fld2: *mut ([u16; 5], u64, u32, i64),
fld3: [i64; 6],

}}

