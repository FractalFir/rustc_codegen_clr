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
pub fn fn0(mut _1: i128,mut _2: char,mut _3: isize,mut _4: i32,mut _5: u32) -> *mut *mut [char; 8] {
mir! {
type RET = *mut *mut [char; 8];
let _6: (f64, char, usize);
let _7: [char; 8];
let _8: u8;
let _9: Adt43;
let _10: i32;
let _11: i16;
let _12: Adt44;
let _13: [i64; 3];
let _14: isize;
let _15: (f64, [i8; 5], usize);
let _16: isize;
let _17: i32;
let _18: Adt59;
let _19: f32;
let _20: [char; 8];
let _21: i128;
let _22: i128;
let _23: f64;
let _24: isize;
let _25: *mut [char; 8];
let _26: u64;
let _27: u16;
let _28: [u64; 4];
let _29: f32;
let _30: char;
let _31: (u16,);
let _32: i16;
let _33: [u16; 4];
let _34: (u16,);
let _35: [char; 8];
let _36: f64;
let _37: *mut i128;
let _38: isize;
let _39: f32;
let _40: f32;
let _41: Adt57;
let _42: (f64, char, usize);
let _43: [u16; 1];
let _44: ();
let _45: ();
{
_3 = -62_isize;
_3 = (-9223372036854775808_isize);
_5 = !1498478270_u32;
_7 = ['\u{d9360}','\u{79994}','\u{ede42}','\u{7d10}','\u{1ea74}','\u{b334}','\u{8a617}','\u{e38d9}'];
_5 = 2239719867_u32 ^ 537657042_u32;
_6.2 = 4383442797816622236_usize;
_1 = !8197533247397145115937139294400147198_i128;
_6.2 = 5_usize;
_2 = '\u{3f2c2}';
_6.1 = _2;
_7 = [_2,_2,_6.1,_2,_6.1,_2,_2,_2];
_4 = 964652119_i32 * (-183708544_i32);
_6.0 = 8532135094300731979_u64 as f64;
_2 = _6.1;
_6.2 = !1_usize;
Goto(bb1)
}
bb1 = {
_7 = [_2,_6.1,_6.1,_6.1,_6.1,_2,_6.1,_6.1];
_2 = _6.1;
_7 = [_6.1,_2,_6.1,_6.1,_6.1,_2,_2,_6.1];
_6.2 = 6_usize | 2_usize;
_4 = _6.0 as i32;
_3 = (-79_isize);
Goto(bb2)
}
bb2 = {
_6.1 = _2;
_12.fld2.2 = 5990_u16;
_11 = 29645_i16 >> _1;
_12.fld6 = (-5022141130918318950_i64) + 8198720902607344874_i64;
_1 = -(-36690640718069172843741753151565558046_i128);
_8 = 59_u8;
_7 = [_6.1,_2,_2,_6.1,_6.1,_6.1,_6.1,_6.1];
Call(_9 = fn1(_4, _12.fld2.2, _6.1, _6.2, _7, _6, _6.0, _7, _12.fld6, _7, _6.1, _6.0, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_12.fld6 = 6913960488420202338_i64 ^ 8191757286043019181_i64;
_12.fld2.0.0 = _5 >> _12.fld6;
_12.fld2.0.1 = core::ptr::addr_of_mut!(_12.fld6);
_12.fld1 = _6.1;
_12.fld2.0.2 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_9, 0), 1)));
_15 = (_6.0, Field::<[i8; 5]>(Variant(_9, 0), 0), _6.2);
_5 = _12.fld2.0.0;
_14 = !_3;
_12.fld5 = _11 as i32;
_6.2 = _15.0 as usize;
_12.fld1 = _2;
place!(Field::<u128>(Variant(_9, 0), 1)) = 248404393114457550219936726800757886820_u128 ^ 219607608271802206407946660648866546670_u128;
_12.fld7 = _12.fld2.0.2;
_15 = (_6.0, Field::<[i8; 5]>(Variant(_9, 0), 0), _6.2);
_12.fld7 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_9, 0), 1)));
_16 = _3 >> _5;
_12.fld0.0 = _1 as u16;
_12.fld4 = core::ptr::addr_of!(_2);
_2 = _6.1;
_14 = _1 as isize;
_16 = _3 - _14;
match _8 {
59 => bb4,
_ => bb1
}
}
bb4 = {
place!(Field::<[i8; 5]>(Variant(_9, 0), 0)) = [125_i8,75_i8,13_i8,(-92_i8),(-60_i8)];
_12.fld6 = _2 as i64;
SetDiscriminant(_9, 0);
_7 = [_2,_2,_2,_2,_6.1,_2,_2,_6.1];
_12.fld2.0.1 = core::ptr::addr_of_mut!(_12.fld6);
place!(Field::<[i8; 5]>(Variant(_9, 0), 0)) = [(-113_i8),114_i8,5_i8,84_i8,66_i8];
_12.fld0.0 = _12.fld2.2;
_17 = _12.fld5 & _12.fld5;
_15.0 = _6.0 - _6.0;
_19 = _14 as f32;
_15.2 = !_6.2;
_12.fld3 = core::ptr::addr_of_mut!(_12.fld6);
_12.fld1 = _6.1;
_12.fld5 = true as i32;
_15.0 = _6.0 * _6.0;
_6 = (_15.0, _12.fld1, _15.2);
_12.fld2.1 = -_11;
_12.fld7 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_9, 0), 1)));
Goto(bb5)
}
bb5 = {
_2 = _6.1;
_11 = _5 as i16;
_5 = _12.fld2.0.0;
_5 = _1 as u32;
_3 = 10443611327314634631_u64 as isize;
_17 = _12.fld2.0.0 as i32;
_13 = [_12.fld6,_12.fld6,_12.fld6];
_12.fld5 = !_17;
_12.fld7 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_9, 0), 1)));
_4 = !_17;
_6 = (_15.0, _12.fld1, _15.2);
_6.1 = _2;
_12.fld2.0.0 = _5 ^ _5;
_5 = _12.fld2.0.0;
match _8 {
0 => bb1,
1 => bb2,
2 => bb3,
59 => bb6,
_ => bb4
}
}
bb6 = {
_12.fld1 = _6.1;
_17 = _4;
place!(Field::<[i8; 5]>(Variant(_9, 0), 0)) = [106_i8,53_i8,12_i8,77_i8,91_i8];
_6.2 = _12.fld2.2 as usize;
_6.0 = _15.0;
_11 = _2 as i16;
_6.2 = 11472517594008600203_u64 as usize;
Call(place!(Field::<[i8; 5]>(Variant(_9, 0), 0)) = core::intrinsics::transmute(_15.1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_3 = _16;
place!(Field::<u128>(Variant(_9, 0), 1)) = 308812173241966419133756391775125108159_u128 * 53745735863789916870333137699395313466_u128;
_1 = 161017262980805031470105814905798244155_i128;
_12.fld2.0.2 = _12.fld7;
_15.0 = _6.0 - _6.0;
_15.2 = _12.fld2.2 as usize;
_12.fld0 = (_12.fld2.2,);
_12.fld2.0.0 = _12.fld1 as u32;
_21 = _1;
_2 = _6.1;
_22 = -_21;
_12.fld7 = _12.fld2.0.2;
_19 = 7957289444654853252_u64 as f32;
_12.fld2.2 = !_12.fld0.0;
_23 = _15.0;
_12.fld0.0 = true as u16;
_4 = _8 as i32;
_19 = 56_i8 as f32;
Call(_15.2 = core::intrinsics::transmute(_16), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_10 = _12.fld5 + _12.fld5;
_12.fld2.2 = _12.fld0.0 & _12.fld0.0;
_20 = _7;
_1 = _22;
_14 = _6.1 as isize;
_15.1 = [7_i8,46_i8,43_i8,109_i8,45_i8];
_12.fld1 = _6.1;
_12.fld2.0.2 = _12.fld7;
RET = core::ptr::addr_of_mut!(_25);
_15.1 = [80_i8,(-81_i8),(-109_i8),(-4_i8),101_i8];
_12.fld5 = _8 as i32;
_12.fld3 = core::ptr::addr_of_mut!(_12.fld6);
(*RET) = core::ptr::addr_of_mut!(_7);
_12.fld2.0.2 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_9, 0), 1)));
RET = core::ptr::addr_of_mut!((*RET));
_13 = [_12.fld6,_12.fld6,_12.fld6];
(*_25) = [_12.fld1,_2,_6.1,_6.1,_6.1,_2,_6.1,_12.fld1];
_12.fld4 = core::ptr::addr_of!(_12.fld1);
(*_25) = [_2,_12.fld1,_2,_12.fld1,_6.1,_6.1,_2,_12.fld1];
_21 = _1;
_12.fld4 = core::ptr::addr_of!(_2);
Goto(bb9)
}
bb9 = {
_11 = _5 as i16;
place!(Field::<[i8; 5]>(Variant(_9, 0), 0)) = [(-62_i8),(-26_i8),25_i8,18_i8,37_i8];
_31.0 = false as u16;
_15.1 = [(-49_i8),(-115_i8),(-20_i8),(-102_i8),2_i8];
_29 = -_19;
RET = core::ptr::addr_of_mut!(_25);
_12.fld6 = 8201856305953308765_i64;
SetDiscriminant(_9, 0);
_32 = 59_i8 as i16;
_27 = _12.fld2.2;
_12.fld2.0 = (_5, _12.fld3, _12.fld7);
_3 = _16;
_22 = _21;
_13 = [_12.fld6,_12.fld6,_12.fld6];
_2 = _6.1;
_17 = 5682938600923376300_u64 as i32;
_12.fld2.0.0 = !_5;
(*_25) = [_2,_2,_12.fld1,_2,_6.1,_2,_2,_12.fld1];
_31 = _12.fld0;
_30 = _12.fld1;
_12.fld2.1 = _8 as i16;
_26 = 12510992147937092268_u64;
_23 = _15.0;
Goto(bb10)
}
bb10 = {
_12.fld4 = core::ptr::addr_of!(_2);
_33 = [_12.fld2.2,_12.fld2.2,_27,_27];
_12.fld5 = -_10;
_28 = [_26,_26,_26,_26];
_36 = _6.0;
_12.fld2.2 = _31.0 | _27;
_26 = 2291756456586312884_u64 * 7438472347436533876_u64;
_8 = !68_u8;
_13 = [_12.fld6,_12.fld6,_12.fld6];
place!(Field::<u128>(Variant(_9, 0), 1)) = 62751494311120422471700104555466015487_u128;
_15.2 = _12.fld1 as usize;
_24 = _14 | _16;
_14 = _3;
RET = core::ptr::addr_of_mut!(_25);
_33 = [_12.fld2.2,_12.fld2.2,_12.fld2.2,_27];
_27 = !_12.fld2.2;
_14 = _3;
_17 = true as i32;
_7 = _20;
_34.0 = _10 as u16;
RET = core::ptr::addr_of_mut!((*RET));
_28 = [_26,_26,_26,_26];
(*RET) = core::ptr::addr_of_mut!(_35);
_24 = _16;
_12.fld2.0.1 = _12.fld3;
_26 = 12894569099303024764_u64;
(*_25) = [_30,_30,_12.fld1,_6.1,_6.1,_12.fld1,_12.fld1,_2];
_37 = core::ptr::addr_of_mut!(_21);
Call(place!(Field::<u128>(Variant(_9, 0), 1)) = core::intrinsics::bswap(171477116179433538675260483666044630970_u128), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_12.fld2.1 = _11;
_15.1 = [(-84_i8),66_i8,(-44_i8),(-61_i8),107_i8];
_6.1 = _12.fld1;
_6.1 = _30;
_32 = _11;
_8 = !159_u8;
_25 = core::ptr::addr_of_mut!(_35);
_23 = -_15.0;
place!(Field::<u128>(Variant(_9, 0), 1)) = 161428133956598991474284906899683192606_u128;
_38 = _3 >> _34.0;
_12.fld4 = core::ptr::addr_of!(_12.fld1);
(*RET) = core::ptr::addr_of_mut!((*_25));
Call(_5 = core::intrinsics::bswap(_12.fld2.0.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_5 = _12.fld2.0.0;
_17 = _24 as i32;
_6.0 = _12.fld5 as f64;
match _26 {
0 => bb6,
1 => bb11,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
12894569099303024764 => bb18,
_ => bb17
}
}
bb13 = {
_12.fld2.1 = _11;
_15.1 = [(-84_i8),66_i8,(-44_i8),(-61_i8),107_i8];
_6.1 = _12.fld1;
_6.1 = _30;
_32 = _11;
_8 = !159_u8;
_25 = core::ptr::addr_of_mut!(_35);
_23 = -_15.0;
place!(Field::<u128>(Variant(_9, 0), 1)) = 161428133956598991474284906899683192606_u128;
_38 = _3 >> _34.0;
_12.fld4 = core::ptr::addr_of!(_12.fld1);
(*RET) = core::ptr::addr_of_mut!((*_25));
Call(_5 = core::intrinsics::bswap(_12.fld2.0.0), ReturnTo(bb12), UnwindUnreachable())
}
bb14 = {
_12.fld6 = 6913960488420202338_i64 ^ 8191757286043019181_i64;
_12.fld2.0.0 = _5 >> _12.fld6;
_12.fld2.0.1 = core::ptr::addr_of_mut!(_12.fld6);
_12.fld1 = _6.1;
_12.fld2.0.2 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_9, 0), 1)));
_15 = (_6.0, Field::<[i8; 5]>(Variant(_9, 0), 0), _6.2);
_5 = _12.fld2.0.0;
_14 = !_3;
_12.fld5 = _11 as i32;
_6.2 = _15.0 as usize;
_12.fld1 = _2;
place!(Field::<u128>(Variant(_9, 0), 1)) = 248404393114457550219936726800757886820_u128 ^ 219607608271802206407946660648866546670_u128;
_12.fld7 = _12.fld2.0.2;
_15 = (_6.0, Field::<[i8; 5]>(Variant(_9, 0), 0), _6.2);
_12.fld7 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_9, 0), 1)));
_16 = _3 >> _5;
_12.fld0.0 = _1 as u16;
_12.fld4 = core::ptr::addr_of!(_2);
_2 = _6.1;
_14 = _1 as isize;
_16 = _3 - _14;
match _8 {
59 => bb4,
_ => bb1
}
}
bb15 = {
_11 = _5 as i16;
place!(Field::<[i8; 5]>(Variant(_9, 0), 0)) = [(-62_i8),(-26_i8),25_i8,18_i8,37_i8];
_31.0 = false as u16;
_15.1 = [(-49_i8),(-115_i8),(-20_i8),(-102_i8),2_i8];
_29 = -_19;
RET = core::ptr::addr_of_mut!(_25);
_12.fld6 = 8201856305953308765_i64;
SetDiscriminant(_9, 0);
_32 = 59_i8 as i16;
_27 = _12.fld2.2;
_12.fld2.0 = (_5, _12.fld3, _12.fld7);
_3 = _16;
_22 = _21;
_13 = [_12.fld6,_12.fld6,_12.fld6];
_2 = _6.1;
_17 = 5682938600923376300_u64 as i32;
_12.fld2.0.0 = !_5;
(*_25) = [_2,_2,_12.fld1,_2,_6.1,_2,_2,_12.fld1];
_31 = _12.fld0;
_30 = _12.fld1;
_12.fld2.1 = _8 as i16;
_26 = 12510992147937092268_u64;
_23 = _15.0;
Goto(bb10)
}
bb16 = {
_10 = _12.fld5 + _12.fld5;
_12.fld2.2 = _12.fld0.0 & _12.fld0.0;
_20 = _7;
_1 = _22;
_14 = _6.1 as isize;
_15.1 = [7_i8,46_i8,43_i8,109_i8,45_i8];
_12.fld1 = _6.1;
_12.fld2.0.2 = _12.fld7;
RET = core::ptr::addr_of_mut!(_25);
_15.1 = [80_i8,(-81_i8),(-109_i8),(-4_i8),101_i8];
_12.fld5 = _8 as i32;
_12.fld3 = core::ptr::addr_of_mut!(_12.fld6);
(*RET) = core::ptr::addr_of_mut!(_7);
_12.fld2.0.2 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_9, 0), 1)));
RET = core::ptr::addr_of_mut!((*RET));
_13 = [_12.fld6,_12.fld6,_12.fld6];
(*_25) = [_12.fld1,_2,_6.1,_6.1,_6.1,_2,_6.1,_12.fld1];
_12.fld4 = core::ptr::addr_of!(_12.fld1);
(*_25) = [_2,_12.fld1,_2,_12.fld1,_6.1,_6.1,_2,_12.fld1];
_21 = _1;
_12.fld4 = core::ptr::addr_of!(_2);
Goto(bb9)
}
bb17 = {
_3 = _16;
place!(Field::<u128>(Variant(_9, 0), 1)) = 308812173241966419133756391775125108159_u128 * 53745735863789916870333137699395313466_u128;
_1 = 161017262980805031470105814905798244155_i128;
_12.fld2.0.2 = _12.fld7;
_15.0 = _6.0 - _6.0;
_15.2 = _12.fld2.2 as usize;
_12.fld0 = (_12.fld2.2,);
_12.fld2.0.0 = _12.fld1 as u32;
_21 = _1;
_2 = _6.1;
_22 = -_21;
_12.fld7 = _12.fld2.0.2;
_19 = 7957289444654853252_u64 as f32;
_12.fld2.2 = !_12.fld0.0;
_23 = _15.0;
_12.fld0.0 = true as u16;
_4 = _8 as i32;
_19 = 56_i8 as f32;
Call(_15.2 = core::intrinsics::transmute(_16), ReturnTo(bb8), UnwindUnreachable())
}
bb18 = {
_29 = _19;
_24 = _21 as isize;
_6.1 = _30;
_12.fld2.0.1 = core::ptr::addr_of_mut!(_12.fld6);
_12.fld2.0.1 = core::ptr::addr_of_mut!(_12.fld6);
_10 = _12.fld5;
_15.2 = _5 as usize;
_10 = _12.fld5 + _12.fld5;
_6.2 = _15.2 << _10;
_16 = _38 << _6.2;
_42 = (_23, _12.fld1, _6.2);
_9 = Adt43::Variant0 { fld0: _15.1,fld1: 144584078756732331986261566664404073384_u128 };
_8 = _22 as u8;
Goto(bb19)
}
bb19 = {
Call(_44 = dump_var(0_usize, 32_usize, Move(_32), 1_usize, Move(_1), 28_usize, Move(_28), 21_usize, Move(_21)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_44 = dump_var(0_usize, 3_usize, Move(_3), 2_usize, Move(_2), 26_usize, Move(_26), 5_usize, Move(_5)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_44 = dump_var(0_usize, 31_usize, Move(_31), 20_usize, Move(_20), 17_usize, Move(_17), 33_usize, Move(_33)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_44 = dump_var(0_usize, 16_usize, Move(_16), 45_usize, _45, 45_usize, _45, 45_usize, _45), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i32,mut _2: u16,mut _3: char,mut _4: usize,mut _5: [char; 8],mut _6: (f64, char, usize),mut _7: f64,mut _8: [char; 8],mut _9: i64,mut _10: [char; 8],mut _11: char,mut _12: f64,mut _13: (f64, char, usize)) -> Adt43 {
mir! {
type RET = Adt43;
let _14: char;
let _15: u8;
let _16: *const *const u16;
let _17: (i16,);
let _18: (f64, [i8; 5], usize);
let _19: u64;
let _20: *mut i128;
let _21: i16;
let _22: *mut *mut [char; 8];
let _23: isize;
let _24: bool;
let _25: isize;
let _26: f64;
let _27: bool;
let _28: [u32; 3];
let _29: *const *const u16;
let _30: (f64, char, usize);
let _31: [u16; 1];
let _32: char;
let _33: [u8; 5];
let _34: isize;
let _35: i32;
let _36: [u32; 3];
let _37: ();
let _38: ();
{
_12 = _7;
_11 = _6.1;
_13.2 = 331925446_u32 as usize;
_13.0 = _7;
_5 = [_6.1,_3,_13.1,_11,_3,_6.1,_13.1,_11];
_2 = 11736_u16 << _9;
_13.2 = !_4;
_1 = 770432560_i32;
_5 = _10;
_13 = _6;
_14 = _3;
_6 = (_13.0, _13.1, _4);
_12 = _9 as f64;
_7 = _12 - _6.0;
_18.0 = _6.0 + _12;
_13 = (_12, _3, _4);
_18.1 = [(-118_i8),(-5_i8),102_i8,45_i8,76_i8];
_17.0 = !(-8083_i16);
_17.0 = _2 as i16;
_8 = [_13.1,_6.1,_13.1,_3,_3,_14,_13.1,_6.1];
_15 = _6.1 as u8;
_5 = _10;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
770432560 => bb7,
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
_11 = _14;
_13.0 = _7;
_18.1 = [92_i8,19_i8,54_i8,(-28_i8),(-113_i8)];
_19 = 5748274277772556756_u64 & 4572836926926801256_u64;
_13.2 = 279084278805397465675428787197537576569_u128 as usize;
_14 = _11;
_18.2 = _4 ^ _4;
_18.1 = [(-19_i8),(-48_i8),(-128_i8),117_i8,(-43_i8)];
_21 = _17.0;
_6.0 = -_7;
_2 = _7 as u16;
_8 = _10;
_15 = 232_u8 << _2;
Call(_4 = fn2(_6.0, _19, _13.0, _5, _8, _17), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_15 = _9 as u8;
_18.2 = !_4;
_13.1 = _14;
_9 = -(-7996973318678653693_i64);
_6 = _13;
_6.0 = _13.0;
_4 = _18.2;
_3 = _6.1;
_17.0 = _21 << _4;
_13 = (_12, _3, _4);
_23 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_25 = _23;
_17.0 = _13.2 as i16;
_25 = !_23;
_26 = -_7;
_3 = _6.1;
RET = Adt43::Variant0 { fld0: _18.1,fld1: 320002025641167301262721704361187136434_u128 };
_18.2 = !_13.2;
_24 = false;
_13.0 = -_26;
RET = Adt43::Variant0 { fld0: _18.1,fld1: 189748967473254761959434295962064219540_u128 };
_6.2 = _18.2 * _18.2;
_18.1 = [(-27_i8),6_i8,(-95_i8),(-78_i8),(-41_i8)];
_6 = (_13.0, _11, _13.2);
place!(Field::<[i8; 5]>(Variant(RET, 0), 0)) = _18.1;
_21 = _17.0 & _17.0;
Goto(bb9)
}
bb9 = {
_12 = -_26;
_6.2 = _21 as usize;
_4 = !_13.2;
_12 = -_26;
_17.0 = !_21;
_13.2 = _4;
_21 = _17.0;
_17 = (_21,);
_15 = 181_u8;
_19 = _1 as u64;
_17 = (_21,);
_13.1 = _14;
_27 = _24;
_19 = 18191754660972898129_u64 ^ 18397824115309362757_u64;
_18.2 = !_6.2;
_25 = _23 & _23;
_8 = [_6.1,_14,_6.1,_13.1,_14,_11,_14,_13.1];
_8 = _10;
_30.1 = _3;
_18.1 = [(-5_i8),126_i8,50_i8,(-68_i8),29_i8];
_8 = _5;
_10 = _8;
_26 = _6.0 - _6.0;
match _1 {
0 => bb10,
1 => bb11,
770432560 => bb13,
_ => bb12
}
}
bb10 = {
_15 = _9 as u8;
_18.2 = !_4;
_13.1 = _14;
_9 = -(-7996973318678653693_i64);
_6 = _13;
_6.0 = _13.0;
_4 = _18.2;
_3 = _6.1;
_17.0 = _21 << _4;
_13 = (_12, _3, _4);
_23 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_25 = _23;
_17.0 = _13.2 as i16;
_25 = !_23;
_26 = -_7;
_3 = _6.1;
RET = Adt43::Variant0 { fld0: _18.1,fld1: 320002025641167301262721704361187136434_u128 };
_18.2 = !_13.2;
_24 = false;
_13.0 = -_26;
RET = Adt43::Variant0 { fld0: _18.1,fld1: 189748967473254761959434295962064219540_u128 };
_6.2 = _18.2 * _18.2;
_18.1 = [(-27_i8),6_i8,(-95_i8),(-78_i8),(-41_i8)];
_6 = (_13.0, _11, _13.2);
place!(Field::<[i8; 5]>(Variant(RET, 0), 0)) = _18.1;
_21 = _17.0 & _17.0;
Goto(bb9)
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_12 = (-101_i8) as f64;
_13.1 = _11;
_25 = _23 * _23;
RET = Adt43::Variant0 { fld0: _18.1,fld1: 183449194954935117490860370416350555394_u128 };
_13.0 = _26;
Goto(bb14)
}
bb14 = {
place!(Field::<u128>(Variant(RET, 0), 1)) = 306945207816107976819211021286881631596_u128 - 197318116471684743806520165117004750369_u128;
_35 = _1;
_5 = [_6.1,_3,_3,_3,_13.1,_11,_14,_13.1];
_27 = !_24;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(1_usize, 17_usize, Move(_17), 1_usize, Move(_1), 25_usize, Move(_25), 27_usize, Move(_27)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(1_usize, 11_usize, Move(_11), 9_usize, Move(_9), 23_usize, Move(_23), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(1_usize, 5_usize, Move(_5), 38_usize, _38, 38_usize, _38, 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: f64,mut _2: u64,mut _3: f64,mut _4: [char; 8],mut _5: [char; 8],mut _6: (i16,)) -> usize {
mir! {
type RET = usize;
let _7: i128;
let _8: [u32; 3];
let _9: [i64; 3];
let _10: bool;
let _11: [u128; 5];
let _12: i16;
let _13: [char; 8];
let _14: (f64, char, usize);
let _15: ();
let _16: ();
{
_6 = ((-14627_i16),);
_3 = _6.0 as f64;
_6.0 = 246609932_u32 as i16;
_3 = -_1;
RET = !2_usize;
RET = 5173944350925943597_usize - 5127190965276928987_usize;
_1 = _3;
_7 = 2684826390_u32 as i128;
RET = 14830250348430548790_usize * 7_usize;
_7 = _6.0 as i128;
_7 = _1 as i128;
RET = !12545602721456687152_usize;
_2 = 13556242040285822720_u64 * 12485343642242662964_u64;
_4 = ['\u{e2d04}','\u{909b8}','\u{e4749}','\u{88424}','\u{e4ba}','\u{5e300}','\u{fd316}','\u{9e585}'];
_1 = _3 - _3;
Call(_6 = fn3(RET, _4, _4, _7, _1, _4, _4, _1, _4, _7, _1, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _7 as usize;
_4 = _5;
_5 = _4;
_4 = ['\u{b0443}','\u{385ae}','\u{78b5f}','\u{cafa3}','\u{8450c}','\u{cac4b}','\u{66d20}','\u{1e1f4}'];
_6 = (22375_i16,);
_7 = '\u{9b094}' as i128;
RET = 7_usize & 6_usize;
_6.0 = 3257823691_u32 as i16;
RET = !11197867145144398644_usize;
_7 = 168441553572195547510321476077934341681_i128;
_6.0 = !(-17202_i16);
RET = 5_usize;
_5 = [_4[RET],_4[RET],_4[RET],_4[RET],_4[RET],_4[RET],_4[RET],_4[RET]];
_2 = 7295318655408295817_u64 << _7;
_3 = _1 + _1;
_6.0 = 10251_i16 ^ (-25610_i16);
_8 = [4083025629_u32,1346945529_u32,1527098133_u32];
RET = 11504435001482809568_usize << _2;
_1 = _3;
_4 = _5;
_5 = ['\u{4d548}','\u{30aba}','\u{b4e82}','\u{1052d4}','\u{dcffd}','\u{47503}','\u{5c319}','\u{d8bb2}'];
RET = 16132734778835840431_usize;
_5 = ['\u{3fa99}','\u{986be}','\u{22ac9}','\u{713b2}','\u{2b4ea}','\u{5cd75}','\u{3e574}','\u{85bc6}'];
_6.0 = (-31937_i16) ^ (-4473_i16);
RET = 4213774909157993296_usize;
RET = 792_u16 as usize;
Goto(bb2)
}
bb2 = {
_8 = [3877450893_u32,3931826094_u32,3059529051_u32];
_6.0 = -29980_i16;
_7 = _3 as i128;
_7 = 162954748903100322791953984474521610480_i128;
_3 = _6.0 as f64;
_6.0 = 24915_i16 << _7;
_7 = 50411968737649911077385993967981734084_i128;
_6.0 = 17914_i16 ^ (-12080_i16);
_6 = ((-25106_i16),);
_9 = [(-3005120651981196533_i64),80726554172952221_i64,6347077943209469080_i64];
_6 = ((-32726_i16),);
_4 = ['\u{de2a2}','\u{56803}','\u{cd84e}','\u{bd024}','\u{dc5dd}','\u{ee3ea}','\u{fd1b9}','\u{5785b}'];
_9 = [4217755539244840398_i64,(-4609727430891689221_i64),7089128900155461763_i64];
RET = 6_usize >> _2;
_1 = 2085766701_i32 as f64;
_12 = _6.0 >> RET;
_1 = 58_u8 as f64;
_14.1 = '\u{20dcf}';
_4 = [_14.1,_14.1,_14.1,_14.1,_14.1,_14.1,_14.1,_14.1];
_14 = (_1, '\u{95055}', RET);
_13 = [_14.1,_14.1,_14.1,_14.1,_14.1,_14.1,_14.1,_14.1];
_6 = (_12,);
_14.2 = RET + RET;
match _7 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
50411968737649911077385993967981734084 => bb10,
_ => bb9
}
}
bb3 = {
RET = _7 as usize;
_4 = _5;
_5 = _4;
_4 = ['\u{b0443}','\u{385ae}','\u{78b5f}','\u{cafa3}','\u{8450c}','\u{cac4b}','\u{66d20}','\u{1e1f4}'];
_6 = (22375_i16,);
_7 = '\u{9b094}' as i128;
RET = 7_usize & 6_usize;
_6.0 = 3257823691_u32 as i16;
RET = !11197867145144398644_usize;
_7 = 168441553572195547510321476077934341681_i128;
_6.0 = !(-17202_i16);
RET = 5_usize;
_5 = [_4[RET],_4[RET],_4[RET],_4[RET],_4[RET],_4[RET],_4[RET],_4[RET]];
_2 = 7295318655408295817_u64 << _7;
_3 = _1 + _1;
_6.0 = 10251_i16 ^ (-25610_i16);
_8 = [4083025629_u32,1346945529_u32,1527098133_u32];
RET = 11504435001482809568_usize << _2;
_1 = _3;
_4 = _5;
_5 = ['\u{4d548}','\u{30aba}','\u{b4e82}','\u{1052d4}','\u{dcffd}','\u{47503}','\u{5c319}','\u{d8bb2}'];
RET = 16132734778835840431_usize;
_5 = ['\u{3fa99}','\u{986be}','\u{22ac9}','\u{713b2}','\u{2b4ea}','\u{5cd75}','\u{3e574}','\u{85bc6}'];
_6.0 = (-31937_i16) ^ (-4473_i16);
RET = 4213774909157993296_usize;
RET = 792_u16 as usize;
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
_10 = !false;
_12 = _1 as i16;
RET = _14.2;
_14.1 = '\u{4e70e}';
RET = _14.2 * _14.2;
_6.0 = _12 ^ _12;
Goto(bb11)
}
bb11 = {
Call(_15 = dump_var(2_usize, 9_usize, Move(_9), 10_usize, Move(_10), 6_usize, Move(_6), 4_usize, Move(_4)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_15 = dump_var(2_usize, 7_usize, Move(_7), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: usize,mut _2: [char; 8],mut _3: [char; 8],mut _4: i128,mut _5: f64,mut _6: [char; 8],mut _7: [char; 8],mut _8: f64,mut _9: [char; 8],mut _10: i128,mut _11: f64,mut _12: [char; 8]) -> (i16,) {
mir! {
type RET = (i16,);
let _13: i8;
let _14: u128;
let _15: (u32, *mut i64, *const u128);
let _16: i32;
let _17: &'static char;
let _18: char;
let _19: bool;
let _20: char;
let _21: ((u32, *mut i64, *const u128), i16, u16);
let _22: isize;
let _23: &'static char;
let _24: f64;
let _25: [char; 8];
let _26: ();
let _27: ();
{
_12 = ['\u{100e67}','\u{19a03}','\u{2a151}','\u{99f48}','\u{a55dc}','\u{acfaa}','\u{efcf9}','\u{324a}'];
RET.0 = 6594_i16 << _10;
_11 = _8;
_13 = (-31_i8);
RET = ((-25813_i16),);
_2 = ['\u{f4329}','\u{365ef}','\u{8fd23}','\u{d41a1}','\u{d5aa4}','\u{d36b3}','\u{2bab4}','\u{fb99b}'];
_9 = ['\u{3c0fd}','\u{33949}','\u{ae80f}','\u{b23f}','\u{b3413}','\u{229f4}','\u{f203f}','\u{8eac6}'];
_5 = -_8;
RET.0 = (-29216_i16);
RET.0 = 32253_i16 >> _10;
_5 = -_8;
_14 = !132367316406560486455117734114486553094_u128;
_15.2 = core::ptr::addr_of!(_14);
_5 = 3725152797398286272_u64 as f64;
_10 = -_4;
_15.0 = 1538043568_u32;
RET.0 = !(-863_i16);
RET.0 = _8 as i16;
_10 = -_4;
_16 = (-262853201_i32) >> _13;
_8 = _11 - _11;
_12 = ['\u{47869}','\u{4ed89}','\u{3e769}','\u{102f4b}','\u{6a078}','\u{30f68}','\u{330a}','\u{fd093}'];
Call(_2 = fn4(RET, RET.0, RET.0, _9, _8, _8, _15.2, _16), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = ((-11652_i16),);
_17 = &_18;
RET = (25053_i16,);
_15.2 = core::ptr::addr_of!(_14);
_3 = ['\u{9e233}','\u{34b14}','\u{10e0e9}','\u{58bf4}','\u{107b62}','\u{8c5b6}','\u{3d51}','\u{2845c}'];
_13 = 1045493507060833337_i64 as i8;
_6 = ['\u{9605d}','\u{ca8c6}','\u{4822b}','\u{d6bdf}','\u{103c9a}','\u{b72a6}','\u{b3163}','\u{3531b}'];
_6 = ['\u{b6920}','\u{4631d}','\u{108982}','\u{13809}','\u{94ce1}','\u{352a1}','\u{106254}','\u{c6c02}'];
_18 = '\u{9b140}';
RET.0 = !15406_i16;
RET.0 = -(-18917_i16);
_10 = _4;
_3 = _9;
_12 = _6;
_21.0.2 = _15.2;
_11 = _4 as f64;
_19 = _5 == _8;
_21.2 = 46188_u16;
_8 = _5;
_21.1 = RET.0;
Call(RET.0 = fn17(_21.0.2, _19, _7, _6, _7, _18, _19), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10 = _4;
_9 = _7;
_9 = [_18,_18,_18,_18,_18,_18,_18,_18];
_19 = !false;
_21.0.0 = !_15.0;
_19 = _11 > _11;
_3 = _12;
Goto(bb3)
}
bb3 = {
Call(_26 = dump_var(3_usize, 13_usize, Move(_13), 3_usize, Move(_3), 16_usize, Move(_16), 7_usize, Move(_7)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_26 = dump_var(3_usize, 1_usize, Move(_1), 9_usize, Move(_9), 10_usize, Move(_10), 27_usize, _27), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: (i16,),mut _2: i16,mut _3: i16,mut _4: [char; 8],mut _5: f64,mut _6: f64,mut _7: *const u128,mut _8: i32) -> [char; 8] {
mir! {
type RET = [char; 8];
let _9: f64;
let _10: *mut (*mut i128, &'static char, i64);
let _11: isize;
let _12: isize;
let _13: [u8; 5];
let _14: char;
let _15: (f64, [i8; 5], usize);
let _16: *mut i64;
let _17: [u64; 4];
let _18: i32;
let _19: bool;
let _20: (f64, [i8; 5], usize);
let _21: [char; 8];
let _22: [u128; 5];
let _23: [u32; 3];
let _24: [i64; 3];
let _25: [u32; 3];
let _26: char;
let _27: isize;
let _28: u8;
let _29: ();
let _30: ();
{
RET = ['\u{978af}','\u{d56bc}','\u{e8c0b}','\u{10f9a7}','\u{70263}','\u{34e54}','\u{950cd}','\u{3587c}'];
_1 = (_3,);
RET = ['\u{104cd7}','\u{4b6a8}','\u{7e58}','\u{104166}','\u{1ffef}','\u{35916}','\u{3e07a}','\u{1d09}'];
(*_7) = 191873576704013035655945744081889657005_u128 ^ 92606118710359034878648371789076444597_u128;
(*_7) = 20857837058656472425965162122429888338_u128;
_7 = core::ptr::addr_of!((*_7));
(*_7) = 63629342417174949840437665831488371375_u128;
_2 = !_3;
_3 = 35066_u16 as i16;
(*_7) = 106063099705909692163510871189601531939_u128;
_2 = _5 as i16;
_9 = _2 as f64;
_6 = -_5;
_11 = _2 as isize;
RET = _4;
_6 = -_5;
Call(_2 = fn5(_9, _6, _5, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = (-379343216_i32) | 668810786_i32;
_15.0 = -_6;
RET = _4;
Call(_15.2 = fn8(_15.0, (*_7), _11, _5, _6, _11, _11, _11, _3, _1, _15.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = _1.0;
_13 = [54_u8,61_u8,156_u8,218_u8,13_u8];
match (*_7) {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
106063099705909692163510871189601531939 => bb10,
_ => bb9
}
}
bb3 = {
_8 = (-379343216_i32) | 668810786_i32;
_15.0 = -_6;
RET = _4;
Call(_15.2 = fn8(_15.0, (*_7), _11, _5, _6, _11, _11, _11, _3, _1, _15.0), ReturnTo(bb2), UnwindUnreachable())
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
_14 = '\u{50c46}';
_3 = _2 >> _11;
RET = [_14,_14,_14,_14,_14,_14,_14,_14];
_2 = -_3;
_12 = -_11;
_6 = -_9;
_1.0 = _2;
_6 = _5 + _15.0;
_3 = 916326375365269430_i64 as i16;
_15.2 = _2 as usize;
(*_7) = _15.2 as u128;
_2 = _1.0;
_6 = _5 - _15.0;
_4 = [_14,_14,_14,_14,_14,_14,_14,_14];
_2 = _1.0;
Goto(bb11)
}
bb11 = {
RET = _4;
_20.0 = -_9;
_20.1 = [49_i8,(-102_i8),(-128_i8),20_i8,90_i8];
_4 = RET;
_17 = [5392692500484832079_u64,8345211387859583927_u64,4405679709774320745_u64,15278344025070568431_u64];
_20.1 = [30_i8,10_i8,77_i8,(-18_i8),(-31_i8)];
_11 = _12 + _12;
_18 = _8 + _8;
_15 = (_9, _20.1, 5197056959935268066_usize);
_19 = true | true;
_15.0 = -_20.0;
_18 = 92_i8 as i32;
_15.2 = !4_usize;
RET = [_14,_14,_14,_14,_14,_14,_14,_14];
_1 = (_2,);
Call(_14 = fn16(_20.0, _1.0, _7, _15.1, _7, RET), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_5 = 4068980412_u32 as f64;
_20.1 = [(-80_i8),15_i8,74_i8,99_i8,(-65_i8)];
_22 = [(*_7),(*_7),(*_7),(*_7),(*_7)];
_15.1 = _20.1;
_21 = [_14,_14,_14,_14,_14,_14,_14,_14];
_12 = _20.0 as isize;
_7 = core::ptr::addr_of!((*_7));
_21 = RET;
_4 = [_14,_14,_14,_14,_14,_14,_14,_14];
_7 = core::ptr::addr_of!((*_7));
_20 = (_15.0, _15.1, _15.2);
_15.2 = _20.2;
Goto(bb13)
}
bb13 = {
_20.2 = !_15.2;
_20 = _15;
_7 = core::ptr::addr_of!((*_7));
_20 = (_9, _15.1, _15.2);
_12 = _9 as isize;
_14 = '\u{ecb42}';
_8 = !_18;
(*_7) = 123023702292049818265519086202707172940_u128 ^ 143480673415773327145459692791301572521_u128;
_4 = [_14,_14,_14,_14,_14,_14,_14,_14];
_7 = core::ptr::addr_of!((*_7));
_5 = _15.0;
_15.0 = _20.0 - _9;
(*_7) = 296360291052713013884246056779237142924_u128 & 122806948257393621788571884534472811846_u128;
_11 = 3280671156_u32 as isize;
_25 = [2821449397_u32,2123251767_u32,106002546_u32];
_13 = [208_u8,45_u8,114_u8,134_u8,154_u8];
_17 = [1839018489298897224_u64,7868571986450377904_u64,3961367584857762860_u64,10476077008152739725_u64];
_15 = (_6, _20.1, _20.2);
_6 = _9 * _9;
_20 = (_9, _15.1, _15.2);
_15 = (_20.0, _20.1, _20.2);
Goto(bb14)
}
bb14 = {
_1 = (_2,);
_24 = [(-2794711690717801953_i64),(-4773045329661970763_i64),7330166129461906102_i64];
_22 = [(*_7),(*_7),(*_7),(*_7),(*_7)];
_7 = core::ptr::addr_of!((*_7));
_15.0 = _6 - _5;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(4_usize, 2_usize, Move(_2), 18_usize, Move(_18), 13_usize, Move(_13), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(4_usize, 12_usize, Move(_12), 4_usize, Move(_4), 1_usize, Move(_1), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: f64,mut _2: f64,mut _3: f64,mut _4: f64) -> i16 {
mir! {
type RET = i16;
let _5: Adt45;
let _6: Adt54;
let _7: f64;
let _8: [u64; 4];
let _9: [u32; 3];
let _10: [i64; 3];
let _11: Adt43;
let _12: u64;
let _13: Adt55;
let _14: Adt48;
let _15: i64;
let _16: f32;
let _17: Adt53;
let _18: [u128; 5];
let _19: [u16; 4];
let _20: (i16,);
let _21: *const u128;
let _22: [u16; 4];
let _23: bool;
let _24: isize;
let _25: [u64; 4];
let _26: (u32, *mut i64, *const u128);
let _27: [i64; 3];
let _28: f32;
let _29: i128;
let _30: *const *const u16;
let _31: bool;
let _32: u8;
let _33: (f64, [i8; 5], usize);
let _34: [i64; 3];
let _35: u64;
let _36: [u64; 4];
let _37: *mut *mut [char; 8];
let _38: u128;
let _39: [u128; 5];
let _40: i16;
let _41: isize;
let _42: char;
let _43: Adt53;
let _44: isize;
let _45: f32;
let _46: isize;
let _47: f32;
let _48: (f64, [i8; 5], usize);
let _49: isize;
let _50: isize;
let _51: [i64; 3];
let _52: [i64; 3];
let _53: u16;
let _54: u64;
let _55: Adt51;
let _56: Adt45;
let _57: [i8; 5];
let _58: ();
let _59: ();
{
RET = 22774_i16;
_3 = -_1;
RET = !30599_i16;
Goto(bb1)
}
bb1 = {
RET = (-13264_i16) + 1671_i16;
_2 = _1;
_2 = (-101_i8) as f64;
RET = !19183_i16;
_3 = 79148678606207252757073993477302150037_u128 as f64;
RET = 3897_i16 - (-18506_i16);
_3 = (-109485472_i32) as f64;
_2 = _4 + _1;
RET = -7174_i16;
_8 = [13850973975673288723_u64,6166991663438659666_u64,8771244083688026972_u64,12215237617483131828_u64];
_9 = [2396282861_u32,4017122084_u32,1519913446_u32];
_1 = _2;
_4 = 5230677107391863615_i64 as f64;
RET = -30153_i16;
_7 = RET as f64;
_8 = [13008100222805747136_u64,14627126923455761346_u64,15907844549471479098_u64,6721256436426569007_u64];
RET = !(-15700_i16);
RET = (-21950_i16);
_1 = 1499580234_u32 as f64;
_4 = _2;
Goto(bb2)
}
bb2 = {
RET = !(-16985_i16);
_9 = [2688807736_u32,3997375839_u32,262745569_u32];
RET = -(-16593_i16);
_10 = [(-3077031950609995272_i64),(-5633664008723840033_i64),(-6293115716650920341_i64)];
_10 = [8910581293111794575_i64,638835521517648501_i64,6747198667793516851_i64];
_3 = _2 + _2;
RET = 201_u8 as i16;
_10 = [1026399396518301049_i64,4474802092608864687_i64,1532620137465134166_i64];
_9 = [2979116547_u32,1223495476_u32,522746029_u32];
_2 = _3;
_10 = [1894128543465441560_i64,(-3163759478166254058_i64),6459646574665526087_i64];
_12 = !5553099981025198786_u64;
_14.fld1.fld3 = core::ptr::addr_of_mut!(_14.fld1.fld6);
_14.fld0 = !false;
_1 = _2 + _3;
_14.fld5.1 = (-85027377422241781378269527170064566892_i128) as i16;
_15 = -3487191131899573554_i64;
_14.fld4 = [53_u8,67_u8,201_u8,157_u8,236_u8];
RET = -_14.fld5.1;
_15 = -3379598483043722204_i64;
Call(_14.fld5.0 = fn6(_3, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = _14.fld5.1 ^ _14.fld5.1;
_14.fld1.fld2.0 = (_14.fld5.0.0, _14.fld5.0.1, _14.fld5.0.2);
_14.fld1.fld0.0 = RET as u16;
RET = (-78347368091906398126186410553491582581_i128) as i16;
_14.fld5 = (_14.fld1.fld2.0, RET, _14.fld1.fld0.0);
_14.fld1.fld2.0.1 = _14.fld5.0.1;
_14.fld1.fld2.2 = !_14.fld1.fld0.0;
_14.fld1.fld1 = '\u{bcb89}';
_14.fld1.fld0 = (_14.fld5.2,);
_16 = _14.fld1.fld2.0.0 as f32;
_23 = !_14.fld0;
_21 = _14.fld1.fld2.0.2;
_16 = (-495058836_i32) as f32;
_19 = [_14.fld1.fld2.2,_14.fld5.2,_14.fld5.2,_14.fld5.2];
Goto(bb4)
}
bb4 = {
_1 = -_3;
_21 = _14.fld5.0.2;
_9 = [_14.fld1.fld2.0.0,_14.fld5.0.0,_14.fld1.fld2.0.0];
_14.fld1.fld4 = core::ptr::addr_of!(_14.fld1.fld1);
_7 = _2 - _1;
_14.fld5.0.0 = _14.fld1.fld2.0.0;
_14.fld1.fld2.1 = -RET;
_20 = (RET,);
_14.fld3 = _9;
_10 = [_15,_15,_15];
_20 = (RET,);
_14.fld6.0 = _2;
_24 = (-9223372036854775808_isize);
Call(_14.fld6.1 = core::intrinsics::transmute(_14.fld4), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_14.fld5 = _14.fld1.fld2;
_14.fld1.fld2.0.2 = _21;
_22 = _19;
_14.fld1.fld2.0 = (_14.fld5.0.0, _14.fld5.0.1, _21);
_26.2 = _14.fld1.fld2.0.2;
_14.fld1.fld4 = core::ptr::addr_of!(_14.fld1.fld1);
_14.fld5.0.0 = _14.fld1.fld2.0.0 << _14.fld1.fld2.0.0;
_16 = _24 as f32;
_22 = _19;
_4 = _7;
_9 = _14.fld3;
_14.fld6.1 = [87_i8,121_i8,120_i8,(-69_i8),5_i8];
_14.fld5.1 = _14.fld1.fld2.1 | _20.0;
_18 = [205076332074709066152977743819890123660_u128,184125502090484216640839790282376773560_u128,176768533588228074928565994864666149264_u128,133262079885481868480315588223418710199_u128,89651416891415298767973377733965959385_u128];
_14.fld1.fld2.2 = _14.fld1.fld0.0 + _14.fld1.fld0.0;
_29 = 22518607778607272133411416858804698895_i128 - 166916667471934833473187517832402561439_i128;
_11 = Adt43::Variant0 { fld0: _14.fld6.1,fld1: 112524767073595071076618024269976071522_u128 };
_3 = _4 - _4;
_14.fld5.0 = _14.fld1.fld2.0;
_14.fld0 = _23;
Goto(bb6)
}
bb6 = {
_26.2 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_11, 0), 1)));
_20 = (_14.fld1.fld2.1,);
_19 = [_14.fld5.2,_14.fld1.fld2.2,_14.fld5.2,_14.fld1.fld2.2];
_28 = _16;
_14.fld1.fld7 = _14.fld5.0.2;
_14.fld4 = [59_u8,193_u8,201_u8,123_u8,106_u8];
_27 = _10;
RET = -_14.fld1.fld2.1;
_14.fld0 = !_23;
_32 = 56_u8;
_14.fld1.fld7 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_11, 0), 1)));
_26.0 = _14.fld5.0.0;
_20 = (_14.fld5.1,);
_14.fld5.0 = (_14.fld1.fld2.0.0, _14.fld1.fld2.0.1, _26.2);
_25 = [_12,_12,_12,_12];
_14.fld1.fld2 = (_14.fld5.0, RET, _14.fld5.2);
place!(Field::<[i8; 5]>(Variant(_11, 0), 0)) = [(-1_i8),(-79_i8),(-81_i8),(-73_i8),(-63_i8)];
_33 = (_14.fld6.0, _14.fld6.1, 12978071545909612278_usize);
_14.fld6 = (_7, _33.1, _33.2);
_14.fld2 = Adt46::Variant2 { fld0: _14.fld1.fld0 };
Goto(bb7)
}
bb7 = {
_35 = !_12;
_19 = _22;
place!(Field::<u128>(Variant(_11, 0), 1)) = 270193586778056722081280161502390999049_u128 * 18670323269475335588590175074122214145_u128;
_14.fld6.2 = _33.2;
place!(Field::<u128>(Variant(_11, 0), 1)) = 204012014447803094636630237326525922052_u128;
_14.fld1.fld2.1 = -_20.0;
place!(Field::<(u16,)>(Variant(_14.fld2, 2), 0)) = (_14.fld5.2,);
_21 = core::ptr::addr_of!(place!(Field::<u128>(Variant(_11, 0), 1)));
_21 = core::ptr::addr_of!((*_21));
_23 = _14.fld0 ^ _14.fld0;
_9 = [_26.0,_14.fld1.fld2.0.0,_14.fld5.0.0];
_3 = -_14.fld6.0;
_32 = 63_u8 * 214_u8;
SetDiscriminant(_14.fld2, 2);
place!(Field::<u128>(Variant(_11, 0), 1)) = (-55_i8) as u128;
_20 = (_14.fld5.1,);
SetDiscriminant(_11, 0);
_15 = 5364279602469956719_i64 | (-6189982415906448556_i64);
Goto(bb8)
}
bb8 = {
place!(Field::<(u16,)>(Variant(_14.fld2, 2), 0)) = _14.fld1.fld0;
place!(Field::<[i8; 5]>(Variant(_11, 0), 0)) = _33.1;
_14.fld4 = [_32,_32,_32,_32,_32];
_26.1 = core::ptr::addr_of_mut!(_15);
SetDiscriminant(_14.fld2, 1);
RET = -_14.fld1.fld2.1;
_26.1 = core::ptr::addr_of_mut!(_15);
_14.fld6.2 = _33.2;
place!(Field::<*mut i128>(Variant(_14.fld2, 1), 4)) = core::ptr::addr_of_mut!(_29);
_14.fld1.fld0.0 = _14.fld5.2;
_11 = Adt43::Variant0 { fld0: _14.fld6.1,fld1: 294724547607426433211286476287729273700_u128 };
_18 = [278711355791933877368675372284212288767_u128,160086752930829336507575127811979200884_u128,161213478555517587140423598160076300047_u128,337864845709399113704422760525201959932_u128,250770083721731704683282268386242964926_u128];
_29 = 68981733241941021065475082864641015394_i128;
_14.fld0 = !_23;
_14.fld1.fld2.0.0 = _33.0 as u32;
match _14.fld6.2 {
12978071545909612278 => bb10,
_ => bb9
}
}
bb9 = {
RET = _14.fld5.1 ^ _14.fld5.1;
_14.fld1.fld2.0 = (_14.fld5.0.0, _14.fld5.0.1, _14.fld5.0.2);
_14.fld1.fld0.0 = RET as u16;
RET = (-78347368091906398126186410553491582581_i128) as i16;
_14.fld5 = (_14.fld1.fld2.0, RET, _14.fld1.fld0.0);
_14.fld1.fld2.0.1 = _14.fld5.0.1;
_14.fld1.fld2.2 = !_14.fld1.fld0.0;
_14.fld1.fld1 = '\u{bcb89}';
_14.fld1.fld0 = (_14.fld5.2,);
_16 = _14.fld1.fld2.0.0 as f32;
_23 = !_14.fld0;
_21 = _14.fld1.fld2.0.2;
_16 = (-495058836_i32) as f32;
_19 = [_14.fld1.fld2.2,_14.fld5.2,_14.fld5.2,_14.fld5.2];
Goto(bb4)
}
bb10 = {
place!(Field::<u128>(Variant(_11, 0), 1)) = 208892074974653282561177039983491037957_u128 + 183230613077950730632881437857263727369_u128;
_14.fld6.1 = [11_i8,91_i8,(-108_i8),23_i8,41_i8];
_14.fld1.fld0 = (_14.fld5.2,);
RET = !_20.0;
_14.fld1.fld2 = (_26, RET, _14.fld1.fld0.0);
_14.fld1.fld2.2 = _14.fld5.2 | _14.fld1.fld0.0;
SetDiscriminant(_11, 1);
_14.fld1.fld5 = -211579325_i32;
_26.1 = _14.fld5.0.1;
_24 = -(-9223372036854775808_isize);
_22 = _19;
place!(Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_11, 1), 3)).0 = (_26.0, _26.1, _26.2);
place!(Field::<i32>(Variant(_14.fld2, 1), 5)) = _7 as i32;
_14.fld1.fld2.0.1 = _14.fld5.0.1;
_14.fld5.2 = !_14.fld1.fld0.0;
_12 = !_35;
_7 = _3;
_14.fld1.fld2.2 = _14.fld5.2 | _14.fld5.2;
place!(Field::<*mut i128>(Variant(_14.fld2, 1), 4)) = core::ptr::addr_of_mut!(_29);
_14.fld0 = !_23;
_14.fld5.0 = (Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_11, 1), 3).0.0, _14.fld1.fld2.0.1, Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_11, 1), 3).0.2);
_26.1 = core::ptr::addr_of_mut!(_15);
place!(Field::<char>(Variant(_14.fld2, 1), 1)) = _14.fld1.fld1;
_33 = _14.fld6;
place!(Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_11, 1), 3)).2 = !_14.fld1.fld0.0;
_14.fld6.1 = _33.1;
_25 = _8;
Goto(bb11)
}
bb11 = {
_1 = -_33.0;
_34 = [_15,_15,_15];
_14.fld5.0 = (_14.fld1.fld2.0.0, Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_11, 1), 3).0.1, _14.fld1.fld2.0.2);
_41 = _24 >> Field::<i32>(Variant(_14.fld2, 1), 5);
place!(Field::<[char; 8]>(Variant(_14.fld2, 1), 3)) = [_14.fld1.fld1,Field::<char>(Variant(_14.fld2, 1), 1),_14.fld1.fld1,_14.fld1.fld1,_14.fld1.fld1,Field::<char>(Variant(_14.fld2, 1), 1),_14.fld1.fld1,_14.fld1.fld1];
_14.fld6.1 = [36_i8,(-101_i8),(-43_i8),(-117_i8),(-3_i8)];
_10 = _27;
_38 = 167850369683971250795408346573440950378_u128 & 83964099883063916894306244552069125717_u128;
place!(Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_11, 1), 3)).0.1 = core::ptr::addr_of_mut!(_15);
_9 = [_14.fld5.0.0,_14.fld5.0.0,_14.fld1.fld2.0.0];
_14.fld1.fld5 = -Field::<i32>(Variant(_14.fld2, 1), 5);
_24 = -_41;
_26.0 = _14.fld5.0.0;
place!(Field::<*const char>(Variant(_14.fld2, 1), 2)) = core::ptr::addr_of!(_14.fld1.fld1);
_14.fld1.fld2.1 = -RET;
_19 = [_14.fld1.fld2.2,Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_11, 1), 3).2,_14.fld5.2,_14.fld1.fld0.0];
place!(Field::<*mut i128>(Variant(_14.fld2, 1), 4)) = core::ptr::addr_of_mut!(_29);
_8 = _25;
RET = _14.fld5.1;
_8 = [_12,_12,_12,_12];
_14.fld1.fld5 = !Field::<i32>(Variant(_14.fld2, 1), 5);
_14.fld6.2 = !_33.2;
_20 = (_14.fld5.1,);
_14.fld1.fld2.0.1 = core::ptr::addr_of_mut!(_14.fld1.fld6);
Call(place!(Field::<i32>(Variant(_14.fld2, 1), 5)) = fn7(_41, Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_11, 1), 3).0, _14.fld1.fld2, _41, _33.2, _24, _14.fld1.fld2.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_14.fld6.2 = 94_i8 as usize;
_14.fld6.0 = _4 + _7;
_14.fld1.fld2.0.2 = core::ptr::addr_of!(_38);
_39 = [_38,_38,_38,_38,_38];
Goto(bb13)
}
bb13 = {
place!(Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_11, 1), 3)).0 = _14.fld1.fld2.0;
place!(Field::<i32>(Variant(_14.fld2, 1), 5)) = _7 as i32;
_29 = 120009197976570435882045169411694390390_i128;
_26 = (_14.fld1.fld2.0.0, _14.fld5.0.1, _14.fld5.0.2);
place!(Field::<f64>(Variant(_11, 1), 1)) = _14.fld6.0;
_29 = _33.2 as i128;
_9 = _14.fld3;
_42 = _14.fld1.fld1;
place!(Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_11, 1), 3)).0.1 = _14.fld5.0.1;
match _33.2 {
12978071545909612278 => bb15,
_ => bb14
}
}
bb14 = {
_14.fld5 = _14.fld1.fld2;
_14.fld1.fld2.0.2 = _21;
_22 = _19;
_14.fld1.fld2.0 = (_14.fld5.0.0, _14.fld5.0.1, _21);
_26.2 = _14.fld1.fld2.0.2;
_14.fld1.fld4 = core::ptr::addr_of!(_14.fld1.fld1);
_14.fld5.0.0 = _14.fld1.fld2.0.0 << _14.fld1.fld2.0.0;
_16 = _24 as f32;
_22 = _19;
_4 = _7;
_9 = _14.fld3;
_14.fld6.1 = [87_i8,121_i8,120_i8,(-69_i8),5_i8];
_14.fld5.1 = _14.fld1.fld2.1 | _20.0;
_18 = [205076332074709066152977743819890123660_u128,184125502090484216640839790282376773560_u128,176768533588228074928565994864666149264_u128,133262079885481868480315588223418710199_u128,89651416891415298767973377733965959385_u128];
_14.fld1.fld2.2 = _14.fld1.fld0.0 + _14.fld1.fld0.0;
_29 = 22518607778607272133411416858804698895_i128 - 166916667471934833473187517832402561439_i128;
_11 = Adt43::Variant0 { fld0: _14.fld6.1,fld1: 112524767073595071076618024269976071522_u128 };
_3 = _4 - _4;
_14.fld5.0 = _14.fld1.fld2.0;
_14.fld0 = _23;
Goto(bb6)
}
bb15 = {
RET = _14.fld1.fld2.1 | _20.0;
_21 = _14.fld1.fld7;
_14.fld5.0 = (_26.0, _26.1, Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_11, 1), 3).0.2);
_14.fld4 = [_32,_32,_32,_32,_32];
_47 = -_16;
place!(Field::<[u128; 5]>(Variant(_11, 1), 2)) = [_38,_38,_38,_38,_38];
_33.1 = [(-75_i8),37_i8,(-119_i8),60_i8,20_i8];
_29 = (-73903304963784515274987961309623897490_i128) * 55099496935132532881422687734239951416_i128;
_53 = _14.fld1.fld2.2 >> _14.fld5.0.0;
_33.0 = -_1;
_50 = !_24;
_26.0 = !Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_11, 1), 3).0.0;
place!(Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_11, 1), 3)).0 = (_26.0, _14.fld5.0.1, _14.fld1.fld7);
_33.0 = _3 + _2;
_31 = _14.fld0;
place!(Field::<char>(Variant(_14.fld2, 1), 1)) = _42;
_14.fld1.fld2.0.2 = _14.fld1.fld7;
_51 = [_15,_15,_15];
place!(Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_11, 1), 3)).0.0 = _14.fld1.fld2.0.0 * _14.fld5.0.0;
_14.fld0 = _31;
place!(Field::<bool>(Variant(_14.fld2, 1), 0)) = _23;
_14.fld1.fld2.0 = (_14.fld5.0.0, Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_11, 1), 3).0.1, _14.fld5.0.2);
Goto(bb16)
}
bb16 = {
Call(_58 = dump_var(5_usize, 51_usize, Move(_51), 31_usize, Move(_31), 22_usize, Move(_22), 29_usize, Move(_29)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_58 = dump_var(5_usize, 19_usize, Move(_19), 15_usize, Move(_15), 34_usize, Move(_34), 53_usize, Move(_53)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_58 = dump_var(5_usize, 41_usize, Move(_41), 32_usize, Move(_32), 9_usize, Move(_9), 8_usize, Move(_8)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: f64,mut _2: f64) -> (u32, *mut i64, *const u128) {
mir! {
type RET = (u32, *mut i64, *const u128);
let _3: char;
let _4: Adt56;
let _5: Adt45;
let _6: [u8; 5];
let _7: [i64; 3];
let _8: (f64, char, usize);
let _9: isize;
let _10: char;
let _11: char;
let _12: char;
let _13: isize;
let _14: (*mut i128, &'static char, i64);
let _15: (f64, char, usize);
let _16: isize;
let _17: (f64, char, usize);
let _18: f32;
let _19: i128;
let _20: char;
let _21: f64;
let _22: *mut i128;
let _23: u128;
let _24: f32;
let _25: *mut *mut [char; 8];
let _26: (i16,);
let _27: bool;
let _28: [u8; 5];
let _29: isize;
let _30: isize;
let _31: *mut i128;
let _32: char;
let _33: u8;
let _34: bool;
let _35: u64;
let _36: [u16; 4];
let _37: [i8; 5];
let _38: ();
let _39: ();
{
RET.0 = 3179775045_u32;
_1 = -_2;
_1 = -_2;
RET.0 = !2791688682_u32;
RET.0 = (-94_i8) as u32;
_1 = _2 - _2;
Goto(bb1)
}
bb1 = {
RET.0 = !1947409853_u32;
RET.0 = 204120251_u32 + 2459662977_u32;
RET.0 = 2268627723_u32 | 73032270_u32;
RET.0 = 170215492_u32;
_3 = '\u{8a88c}';
_6 = [112_u8,10_u8,176_u8,93_u8,129_u8];
_6 = [222_u8,202_u8,184_u8,17_u8,15_u8];
_1 = _2 - _2;
_6 = [36_u8,152_u8,50_u8,100_u8,221_u8];
_1 = _2;
_3 = '\u{b8ad3}';
_6 = [113_u8,212_u8,232_u8,248_u8,42_u8];
_7 = [4454969601328666172_i64,(-6434595183020254261_i64),3838242632086720078_i64];
_1 = _2;
_4 = Adt56::Variant1 { fld0: (-9223372036854775808_isize) };
_3 = '\u{23d3e}';
_8.0 = _1;
_8.1 = _3;
Goto(bb2)
}
bb2 = {
_8.0 = _1;
_8.0 = _1 - _2;
_8.1 = _3;
_1 = _2 * _8.0;
_10 = _3;
place!(Field::<isize>(Variant(_4, 1), 0)) = 17759789560741546361_u64 as isize;
_10 = _8.1;
_12 = _10;
_11 = _8.1;
_2 = -_1;
_8.2 = 3_usize + 17313660621466734308_usize;
_4 = Adt56::Variant1 { fld0: 23_isize };
_8.1 = _11;
_3 = _12;
_13 = 9223372036854775807_isize;
_8.0 = -_2;
_8 = (_2, _3, 16901627207562394334_usize);
_3 = _11;
match _8.2 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
16901627207562394334 => bb10,
_ => bb9
}
}
bb3 = {
RET.0 = !1947409853_u32;
RET.0 = 204120251_u32 + 2459662977_u32;
RET.0 = 2268627723_u32 | 73032270_u32;
RET.0 = 170215492_u32;
_3 = '\u{8a88c}';
_6 = [112_u8,10_u8,176_u8,93_u8,129_u8];
_6 = [222_u8,202_u8,184_u8,17_u8,15_u8];
_1 = _2 - _2;
_6 = [36_u8,152_u8,50_u8,100_u8,221_u8];
_1 = _2;
_3 = '\u{b8ad3}';
_6 = [113_u8,212_u8,232_u8,248_u8,42_u8];
_7 = [4454969601328666172_i64,(-6434595183020254261_i64),3838242632086720078_i64];
_1 = _2;
_4 = Adt56::Variant1 { fld0: (-9223372036854775808_isize) };
_3 = '\u{23d3e}';
_8.0 = _1;
_8.1 = _3;
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
_7 = [6942389892254683984_i64,(-594558727659361313_i64),7284094101734788179_i64];
_11 = _3;
_8.0 = 6109_i16 as f64;
_8.0 = _1;
_10 = _8.1;
_13 = !(-15_isize);
_9 = !_13;
_8.2 = 1_usize >> RET.0;
_14.2 = (-55748047246558600_i64);
_14.1 = &_11;
place!(Field::<isize>(Variant(_4, 1), 0)) = _9;
_1 = 11053906819386701116_u64 as f64;
_15.2 = !_8.2;
_14.2 = (-8511748613291008947_i64);
_3 = _12;
Goto(bb11)
}
bb11 = {
_15.1 = _11;
_1 = _8.0 + _8.0;
_7 = [_14.2,_14.2,_14.2];
_8.2 = _15.2 * _15.2;
_7 = [_14.2,_14.2,_14.2];
_8 = (_2, _15.1, _15.2);
_15 = (_2, _8.1, _8.2);
_15.1 = _11;
_2 = _15.0 - _8.0;
_7 = [_14.2,_14.2,_14.2];
SetDiscriminant(_4, 0);
place!(Field::<(u32, *mut i64, *const u128)>(Variant(_4, 0), 5)).1 = core::ptr::addr_of_mut!(place!(Field::<Adt52>(Variant(_4, 0), 0)).fld1.fld6);
_15 = (_8.0, _12, _8.2);
_13 = _9 - _9;
_4 = Adt56::Variant1 { fld0: _13 };
_11 = _12;
_11 = _15.1;
place!(Field::<isize>(Variant(_4, 1), 0)) = _13 | _9;
_17.0 = _1;
_12 = _11;
match _14.2 {
0 => bb12,
1 => bb13,
340282366920938463454862858818477202509 => bb15,
_ => bb14
}
}
bb12 = {
RET.0 = !1947409853_u32;
RET.0 = 204120251_u32 + 2459662977_u32;
RET.0 = 2268627723_u32 | 73032270_u32;
RET.0 = 170215492_u32;
_3 = '\u{8a88c}';
_6 = [112_u8,10_u8,176_u8,93_u8,129_u8];
_6 = [222_u8,202_u8,184_u8,17_u8,15_u8];
_1 = _2 - _2;
_6 = [36_u8,152_u8,50_u8,100_u8,221_u8];
_1 = _2;
_3 = '\u{b8ad3}';
_6 = [113_u8,212_u8,232_u8,248_u8,42_u8];
_7 = [4454969601328666172_i64,(-6434595183020254261_i64),3838242632086720078_i64];
_1 = _2;
_4 = Adt56::Variant1 { fld0: (-9223372036854775808_isize) };
_3 = '\u{23d3e}';
_8.0 = _1;
_8.1 = _3;
Goto(bb2)
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_9 = -Field::<isize>(Variant(_4, 1), 0);
_8.2 = !_15.2;
_17.2 = _15.2 ^ _15.2;
_3 = _11;
_16 = _9;
_14.0 = core::ptr::addr_of_mut!(_19);
_8.1 = _15.1;
_4 = Adt56::Variant1 { fld0: _16 };
_2 = -_1;
_23 = !146670398462866333901370288123858081196_u128;
SetDiscriminant(_4, 2);
_6 = [216_u8,246_u8,173_u8,250_u8,81_u8];
_4 = Adt56::Variant1 { fld0: _13 };
_18 = 30487_u16 as f32;
Goto(bb16)
}
bb16 = {
_17.2 = 108590714607745782856696820804477824543_i128 as usize;
_14.0 = core::ptr::addr_of_mut!(_19);
_15.1 = _8.1;
SetDiscriminant(_4, 0);
place!(Field::<Adt52>(Variant(_4, 0), 0)).fld1.fld2.0.2 = core::ptr::addr_of!(_23);
_7 = [_14.2,_14.2,_14.2];
_15.1 = _11;
place!(Field::<Adt52>(Variant(_4, 0), 0)).fld1.fld2.0.1 = core::ptr::addr_of_mut!(_14.2);
place!(Field::<Adt52>(Variant(_4, 0), 0)).fld1.fld7 = Field::<Adt52>(Variant(_4, 0), 0).fld1.fld2.0.2;
place!(Field::<Adt52>(Variant(_4, 0), 0)).fld1.fld0 = (39848_u16,);
place!(Field::<(u32, *mut i64, *const u128)>(Variant(_4, 0), 5)) = (RET.0, Field::<Adt52>(Variant(_4, 0), 0).fld1.fld2.0.1, Field::<Adt52>(Variant(_4, 0), 0).fld1.fld2.0.2);
place!(Field::<Adt52>(Variant(_4, 0), 0)).fld1.fld2.0.0 = _1 as u32;
RET.1 = Field::<(u32, *mut i64, *const u128)>(Variant(_4, 0), 5).1;
place!(Field::<Adt52>(Variant(_4, 0), 0)).fld1.fld1 = _8.1;
_7 = [_14.2,_14.2,_14.2];
_24 = Field::<Adt52>(Variant(_4, 0), 0).fld1.fld0.0 as f32;
place!(Field::<Adt52>(Variant(_4, 0), 0)).fld1.fld6 = _14.2;
_8.2 = _15.2;
_15.2 = _17.2 - _17.2;
_7 = [_14.2,Field::<Adt52>(Variant(_4, 0), 0).fld1.fld6,Field::<Adt52>(Variant(_4, 0), 0).fld1.fld6];
_7 = [_14.2,_14.2,Field::<Adt52>(Variant(_4, 0), 0).fld1.fld6];
place!(Field::<Adt52>(Variant(_4, 0), 0)).fld1.fld3 = Field::<Adt52>(Variant(_4, 0), 0).fld1.fld2.0.1;
place!(Field::<Adt52>(Variant(_4, 0), 0)).fld1.fld2.0.2 = core::ptr::addr_of!(_23);
_18 = (-22018948995388764310564812588612154746_i128) as f32;
place!(Field::<*const u128>(Variant(_4, 0), 2)) = Field::<Adt52>(Variant(_4, 0), 0).fld1.fld2.0.2;
Goto(bb17)
}
bb17 = {
place!(Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_4, 0), 1)).0 = Field::<Adt52>(Variant(_4, 0), 0).fld1.fld2.0;
_1 = _17.0 - _8.0;
place!(Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_4, 0), 1)).0.1 = core::ptr::addr_of_mut!(_14.2);
place!(Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_4, 0), 1)).0.0 = !Field::<Adt52>(Variant(_4, 0), 0).fld1.fld2.0.0;
place!(Field::<Adt52>(Variant(_4, 0), 0)).fld1.fld6 = 9927892793058493666_u64 as i64;
_15.2 = 1018083866_i32 as usize;
_14.0 = core::ptr::addr_of_mut!(_19);
_26.0 = (-12078_i16) >> Field::<Adt52>(Variant(_4, 0), 0).fld1.fld2.0.0;
_3 = Field::<Adt52>(Variant(_4, 0), 0).fld1.fld1;
place!(Field::<Adt52>(Variant(_4, 0), 0)).fld1.fld7 = Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_4, 0), 1).0.2;
place!(Field::<Adt52>(Variant(_4, 0), 0)).fld1.fld4 = core::ptr::addr_of!(_17.1);
_13 = _9 - _16;
_8.2 = 56773291910521062115074930300107096567_i128 as usize;
Goto(bb18)
}
bb18 = {
place!(Field::<Adt52>(Variant(_4, 0), 0)).fld1.fld5 = -834823653_i32;
_8.0 = -_17.0;
_26 = ((-4325_i16),);
place!(Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_4, 0), 1)).0 = (Field::<Adt52>(Variant(_4, 0), 0).fld1.fld2.0.0, Field::<Adt52>(Variant(_4, 0), 0).fld1.fld2.0.1, Field::<Adt52>(Variant(_4, 0), 0).fld1.fld7);
_3 = Field::<Adt52>(Variant(_4, 0), 0).fld1.fld1;
_9 = !_16;
_8.0 = _1;
place!(Field::<Adt52>(Variant(_4, 0), 0)).fld1.fld1 = _12;
place!(Field::<Adt52>(Variant(_4, 0), 0)).fld1.fld5 = 1667129913_i32;
RET = Field::<Adt52>(Variant(_4, 0), 0).fld1.fld2.0;
_32 = _11;
place!(Field::<Adt52>(Variant(_4, 0), 0)).fld1.fld4 = core::ptr::addr_of!(_17.1);
_22 = core::ptr::addr_of_mut!(_19);
place!(Field::<Adt52>(Variant(_4, 0), 0)).fld2 = _9 & _13;
place!(Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_4, 0), 1)) = (RET, _26.0, Field::<Adt52>(Variant(_4, 0), 0).fld1.fld0.0);
_17.1 = _3;
place!(Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_4, 0), 1)).0.1 = core::ptr::addr_of_mut!(_14.2);
place!(Field::<Adt52>(Variant(_4, 0), 0)).fld1.fld4 = core::ptr::addr_of!(_20);
_8 = (_15.0, _15.1, _15.2);
RET = Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_4, 0), 1).0;
Goto(bb19)
}
bb19 = {
Call(_38 = dump_var(6_usize, 3_usize, Move(_3), 9_usize, Move(_9), 11_usize, Move(_11), 23_usize, Move(_23)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_38 = dump_var(6_usize, 26_usize, Move(_26), 16_usize, Move(_16), 39_usize, _39, 39_usize, _39), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: isize,mut _2: (u32, *mut i64, *const u128),mut _3: ((u32, *mut i64, *const u128), i16, u16),mut _4: isize,mut _5: usize,mut _6: isize,mut _7: (u32, *mut i64, *const u128)) -> i32 {
mir! {
type RET = i32;
let _8: *const u16;
let _9: (f64, [i8; 5], usize);
let _10: [char; 8];
let _11: u16;
let _12: *const char;
let _13: bool;
let _14: [i8; 5];
let _15: f32;
let _16: Adt51;
let _17: [u16; 4];
let _18: [u8; 5];
let _19: ();
let _20: ();
{
_3.0.1 = _2.1;
_7 = (_2.0, _2.1, _3.0.2);
_7.2 = _3.0.2;
_3.1 = (-4436_i16);
RET = (-424703288_i32) * (-1850889503_i32);
_4 = _6 ^ _1;
_3.2 = 31877_u16 & 2540_u16;
_2 = (_3.0.0, _7.1, _7.2);
_3.1 = 13163_i16 + 18064_i16;
_1 = !_4;
_8 = core::ptr::addr_of!(_3.2);
RET = 627357723_i32;
_3 = (_7, (-21382_i16), 19273_u16);
_5 = 139336531787798628985222632962490447608_i128 as usize;
_7 = (_2.0, _3.0.1, _2.2);
RET = (-1312868201_i32) - (-664778056_i32);
_3.0.2 = _2.2;
_7.1 = _3.0.1;
_3.0.1 = _7.1;
_6 = _4;
_2 = (_7.0, _7.1, _3.0.2);
RET = -935340257_i32;
_1 = -_6;
match (*_8) {
0 => bb1,
19273 => bb3,
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
(*_8) = 42389_u16 << _4;
_3.0.1 = _2.1;
_2.2 = _3.0.2;
_2.1 = _3.0.1;
_7.2 = _3.0.2;
match _3.1 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
340282366920938463463374607431768190074 => bb8,
_ => bb7
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
_3.0.2 = _7.2;
_1 = !_4;
_9.0 = 126611909502199217637083753771396324240_u128 as f64;
_9.1 = [34_i8,(-125_i8),62_i8,(-39_i8),(-121_i8)];
_11 = (*_8);
_2.2 = _3.0.2;
_9.2 = !_5;
_7.1 = _3.0.1;
_5 = !_9.2;
_3.0 = (_2.0, _7.1, _2.2);
_9.2 = _5 * _5;
_10 = ['\u{50abc}','\u{24b4d}','\u{1ca76}','\u{e2230}','\u{e0956}','\u{d2268}','\u{4a823}','\u{444f1}'];
_2.1 = _3.0.1;
_7.1 = _2.1;
_3.0.0 = _7.0 + _7.0;
_2.1 = _7.1;
RET = !(-830201610_i32);
_9.0 = _4 as f64;
_7.1 = _2.1;
match _3.1 {
0 => bb1,
340282366920938463463374607431768190074 => bb9,
_ => bb3
}
}
bb9 = {
_7.0 = _2.0;
_7 = _3.0;
_1 = _3.1 as isize;
_7 = _3.0;
_3.2 = 60271878068720178568567854207664983015_i128 as u16;
_14 = [47_i8,20_i8,(-107_i8),(-92_i8),33_i8];
_7 = (_2.0, _3.0.1, _3.0.2);
_14 = _9.1;
_3.0 = _2;
_3 = (_2, 28421_i16, _11);
(*_8) = _11;
_10 = ['\u{9f3d9}','\u{4bc92}','\u{18a5b}','\u{6d0a}','\u{6baf7}','\u{228d9}','\u{1004a9}','\u{9600b}'];
match _3.1 {
0 => bb6,
1 => bb8,
2 => bb10,
3 => bb11,
4 => bb12,
28421 => bb14,
_ => bb13
}
}
bb10 = {
_3.0.2 = _7.2;
_1 = !_4;
_9.0 = 126611909502199217637083753771396324240_u128 as f64;
_9.1 = [34_i8,(-125_i8),62_i8,(-39_i8),(-121_i8)];
_11 = (*_8);
_2.2 = _3.0.2;
_9.2 = !_5;
_7.1 = _3.0.1;
_5 = !_9.2;
_3.0 = (_2.0, _7.1, _2.2);
_9.2 = _5 * _5;
_10 = ['\u{50abc}','\u{24b4d}','\u{1ca76}','\u{e2230}','\u{e0956}','\u{d2268}','\u{4a823}','\u{444f1}'];
_2.1 = _3.0.1;
_7.1 = _2.1;
_3.0.0 = _7.0 + _7.0;
_2.1 = _7.1;
RET = !(-830201610_i32);
_9.0 = _4 as f64;
_7.1 = _2.1;
match _3.1 {
0 => bb1,
340282366920938463463374607431768190074 => bb9,
_ => bb3
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
_11 = 67_u8 as u16;
RET = 1519322882_i32 & 230307133_i32;
_9.0 = 149512076291775268103073008140447167372_u128 as f64;
_7.2 = _3.0.2;
_13 = false;
_2 = (_3.0.0, _3.0.1, _7.2);
_8 = core::ptr::addr_of!((*_8));
_3.0.1 = _7.1;
RET = _9.2 as i32;
_3.0 = _2;
_10 = ['\u{107cfd}','\u{3bdd2}','\u{c0784}','\u{9136d}','\u{107cf0}','\u{be3ac}','\u{b6b05}','\u{4fa31}'];
_3.1 = 8900730070539103054_u64 as i16;
_7.0 = RET as u32;
_17 = [_3.2,_11,(*_8),(*_8)];
_3.0.0 = _2.0;
_3.1 = 27216_i16 + (-24828_i16);
_1 = _6;
_16 = Adt51::Variant2 { fld0: 15679974598598500963_u64 };
place!(Field::<u64>(Variant(_16, 2), 0)) = 10099834620829115667_u64;
_2 = _3.0;
_18 = [34_u8,18_u8,30_u8,162_u8,221_u8];
RET = !(-961671553_i32);
_17 = [_3.2,(*_8),(*_8),(*_8)];
_14 = _9.1;
SetDiscriminant(_16, 2);
(*_8) = _11 & _11;
_15 = 93_u8 as f32;
Goto(bb15)
}
bb15 = {
Call(_19 = dump_var(7_usize, 1_usize, Move(_1), 13_usize, Move(_13), 11_usize, Move(_11), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_19 = dump_var(7_usize, 10_usize, Move(_10), 20_usize, _20, 20_usize, _20, 20_usize, _20), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: f64,mut _2: u128,mut _3: isize,mut _4: f64,mut _5: f64,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: i16,mut _10: (i16,),mut _11: f64) -> usize {
mir! {
type RET = usize;
let _12: (u32, *mut i64, *const u128);
let _13: *mut *mut [char; 8];
let _14: [char; 8];
let _15: u8;
let _16: [u8; 5];
let _17: [char; 8];
let _18: isize;
let _19: f32;
let _20: isize;
let _21: (f64, char, usize);
let _22: isize;
let _23: *const char;
let _24: isize;
let _25: Adt51;
let _26: (*mut i128, &'static char, i64);
let _27: usize;
let _28: f64;
let _29: Adt51;
let _30: (f64, [i8; 5], usize);
let _31: f64;
let _32: isize;
let _33: bool;
let _34: (f64, char, usize);
let _35: ();
let _36: ();
{
RET = !10464556558160222263_usize;
_2 = 103240175523924500034382584475984855423_u128 ^ 283220791266526506109741280075156097595_u128;
_1 = _5;
_12.0 = !1925213_u32;
_8 = !_3;
_12.2 = core::ptr::addr_of!(_2);
_12.2 = core::ptr::addr_of!(_2);
_4 = -_5;
_2 = !302967291166287986204488499302107601648_u128;
_6 = _8;
_10 = (_9,);
_6 = _8 << _8;
_10.0 = _2 as i16;
_10 = (_9,);
_10 = (_9,);
RET = 17183416053462933597_usize;
_7 = _6;
_6 = _7;
_6 = RET as isize;
_6 = _7;
Call(_12.1 = fn9(_4, _12.2, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12.0 = 330851795_u32 << _6;
_6 = -_7;
_1 = _5 + _11;
RET = 12169753526071861420_usize;
_12.0 = !1127774631_u32;
RET = !15647380970949009991_usize;
_1 = -_5;
_14 = ['\u{98009}','\u{8c3ba}','\u{da25c}','\u{b34b2}','\u{87935}','\u{7628c}','\u{4a9bf}','\u{bc3b1}'];
_4 = (-107_i8) as f64;
_11 = 12833_u16 as f64;
_8 = _7 ^ _7;
_1 = -_5;
_12.2 = core::ptr::addr_of!(_2);
_8 = _6 >> _6;
_5 = 36975_u16 as f64;
_11 = _12.0 as f64;
_6 = (-115994040569782558437356278692361753800_i128) as isize;
_12.2 = core::ptr::addr_of!(_2);
_5 = -_4;
_15 = 347934604_i32 as u8;
_12.2 = core::ptr::addr_of!(_2);
_5 = RET as f64;
RET = 11332978219419521442_usize;
_2 = !35619224376901366233436819970668576207_u128;
_2 = !217857120314578691119766536038079316062_u128;
_8 = _7;
_10 = (_9,);
_16 = [_15,_15,_15,_15,_15];
_7 = !_8;
Call(_5 = core::intrinsics::transmute(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = _7;
_15 = 54_u8 ^ 25_u8;
_10 = (_9,);
_12.2 = core::ptr::addr_of!(_2);
_16 = [_15,_15,_15,_15,_15];
_12.0 = 370508555_u32 << _6;
_4 = 234371474159863239_u64 as f64;
_15 = !71_u8;
_12.2 = core::ptr::addr_of!(_2);
_16 = [_15,_15,_15,_15,_15];
_15 = !42_u8;
_16 = [_15,_15,_15,_15,_15];
_3 = 26351_u16 as isize;
_11 = _5 + _1;
_12.2 = core::ptr::addr_of!(_2);
Call(_9 = core::intrinsics::transmute(_10.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9 = -_10.0;
_1 = 787648088176260959_u64 as f64;
_14 = ['\u{f1822}','\u{e5ea7}','\u{83d1c}','\u{6af30}','\u{e03db}','\u{fc9f0}','\u{cd0cb}','\u{25728}'];
_3 = _7;
_12.0 = 3591526681_u32;
_7 = !_8;
_1 = 94_i8 as f64;
RET = 7047030484699763445_usize;
_2 = 279212051124616938868083466405409305382_u128 >> _3;
_6 = _8 >> _3;
_10 = (_9,);
_19 = 27824_u16 as f32;
_4 = _11;
_18 = 6237058549668315531_i64 as isize;
_17 = _14;
_4 = _11;
_17 = _14;
_19 = 19_i8 as f32;
_14 = _17;
_3 = _15 as isize;
_10 = (_9,);
_16 = [_15,_15,_15,_15,_15];
_5 = _15 as f64;
_8 = -_7;
_11 = -_4;
_8 = !_6;
_14 = _17;
_20 = _6;
match _12.0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
3591526681 => bb9,
_ => bb8
}
}
bb4 = {
_6 = _7;
_15 = 54_u8 ^ 25_u8;
_10 = (_9,);
_12.2 = core::ptr::addr_of!(_2);
_16 = [_15,_15,_15,_15,_15];
_12.0 = 370508555_u32 << _6;
_4 = 234371474159863239_u64 as f64;
_15 = !71_u8;
_12.2 = core::ptr::addr_of!(_2);
_16 = [_15,_15,_15,_15,_15];
_15 = !42_u8;
_16 = [_15,_15,_15,_15,_15];
_3 = 26351_u16 as isize;
_11 = _5 + _1;
_12.2 = core::ptr::addr_of!(_2);
Call(_9 = core::intrinsics::transmute(_10.0), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_12.0 = 330851795_u32 << _6;
_6 = -_7;
_1 = _5 + _11;
RET = 12169753526071861420_usize;
_12.0 = !1127774631_u32;
RET = !15647380970949009991_usize;
_1 = -_5;
_14 = ['\u{98009}','\u{8c3ba}','\u{da25c}','\u{b34b2}','\u{87935}','\u{7628c}','\u{4a9bf}','\u{bc3b1}'];
_4 = (-107_i8) as f64;
_11 = 12833_u16 as f64;
_8 = _7 ^ _7;
_1 = -_5;
_12.2 = core::ptr::addr_of!(_2);
_8 = _6 >> _6;
_5 = 36975_u16 as f64;
_11 = _12.0 as f64;
_6 = (-115994040569782558437356278692361753800_i128) as isize;
_12.2 = core::ptr::addr_of!(_2);
_5 = -_4;
_15 = 347934604_i32 as u8;
_12.2 = core::ptr::addr_of!(_2);
_5 = RET as f64;
RET = 11332978219419521442_usize;
_2 = !35619224376901366233436819970668576207_u128;
_2 = !217857120314578691119766536038079316062_u128;
_8 = _7;
_10 = (_9,);
_16 = [_15,_15,_15,_15,_15];
_7 = !_8;
Call(_5 = core::intrinsics::transmute(_3), ReturnTo(bb2), UnwindUnreachable())
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
_9 = _10.0;
_3 = _10.0 as isize;
_6 = _20;
_6 = -_20;
_8 = _7 << _7;
_10.0 = (-136992957144644683314285190299585953784_i128) as i16;
_1 = -_11;
_8 = (-3465248993173404743_i64) as isize;
_3 = _6;
_3 = _20 ^ _20;
Goto(bb10)
}
bb10 = {
_14 = ['\u{f0470}','\u{aa021}','\u{a852c}','\u{928d7}','\u{c54db}','\u{54df4}','\u{7f1b8}','\u{10ffcd}'];
_21 = (_1, '\u{7709c}', RET);
_8 = 33641_u16 as isize;
_16 = [_15,_15,_15,_15,_15];
RET = 1823518921_i32 as usize;
_21 = (_11, '\u{f211}', RET);
_11 = (-82_i8) as f64;
_12.0 = !992302273_u32;
Goto(bb11)
}
bb11 = {
_2 = !329798720020191052330486563565464243374_u128;
_8 = !_3;
_19 = 1986205979792097691_u64 as f32;
_1 = -_4;
_12.2 = core::ptr::addr_of!(_2);
_21 = (_1, '\u{9f996}', RET);
_15 = 157_u8 ^ 175_u8;
_20 = (-6650926097616269983_i64) as isize;
_21.0 = -_1;
Goto(bb12)
}
bb12 = {
_6 = -_3;
_18 = _3;
_3 = !_6;
_24 = _21.1 as isize;
_12.0 = !965567125_u32;
_21.1 = '\u{f1879}';
RET = _21.2 * _21.2;
_23 = core::ptr::addr_of!(_21.1);
_2 = 288682726013354434289916704565565200726_u128 - 136601475779438315115302714083159856378_u128;
_27 = _19 as usize;
_25 = Adt51::Variant2 { fld0: 7795625537727812439_u64 };
_2 = !263254076137259781115331972233246377382_u128;
_7 = _1 as isize;
_21 = (_4, '\u{5013b}', RET);
_6 = _10.0 as isize;
place!(Field::<u64>(Variant(_25, 2), 0)) = 12978391591330286793_u64 + 15727465977978435955_u64;
_10.0 = _15 as i16;
_12.2 = core::ptr::addr_of!(_2);
_3 = _8 << _8;
_22 = _8;
_17 = _14;
Goto(bb13)
}
bb13 = {
_15 = 232_u8;
_2 = 120843561395213685919081201031148756040_u128 & 236559188471473433901203776108743084827_u128;
_10 = (_9,);
_8 = _15 as isize;
_4 = _1 - _21.0;
_26.2 = (-5848233865069862302_i64) * (-6586187114275596660_i64);
_1 = 35674622894216938677834971726234570325_i128 as f64;
_29 = Move(_25);
(*_23) = '\u{7e1cb}';
_23 = core::ptr::addr_of!((*_23));
_28 = _4;
_6 = _22;
_12.2 = core::ptr::addr_of!(_2);
_20 = -_18;
_16 = [_15,_15,_15,_15,_15];
_22 = -_3;
_22 = _18;
_8 = -_6;
_27 = RET;
SetDiscriminant(_29, 0);
place!(Field::<(f64, char, usize)>(Variant(_29, 0), 0)).2 = _27;
_3 = _22 >> _22;
_5 = _9 as f64;
place!(Field::<(f64, char, usize)>(Variant(_29, 0), 0)).0 = _28 - _28;
_8 = _20;
place!(Field::<[u64; 4]>(Variant(_29, 0), 7)) = [15137345513636682095_u64,8333443645609887410_u64,2073303742392989730_u64,4708381463085314813_u64];
match _15 {
0 => bb10,
1 => bb14,
2 => bb15,
3 => bb16,
232 => bb18,
_ => bb17
}
}
bb14 = {
_6 = -_3;
_18 = _3;
_3 = !_6;
_24 = _21.1 as isize;
_12.0 = !965567125_u32;
_21.1 = '\u{f1879}';
RET = _21.2 * _21.2;
_23 = core::ptr::addr_of!(_21.1);
_2 = 288682726013354434289916704565565200726_u128 - 136601475779438315115302714083159856378_u128;
_27 = _19 as usize;
_25 = Adt51::Variant2 { fld0: 7795625537727812439_u64 };
_2 = !263254076137259781115331972233246377382_u128;
_7 = _1 as isize;
_21 = (_4, '\u{5013b}', RET);
_6 = _10.0 as isize;
place!(Field::<u64>(Variant(_25, 2), 0)) = 12978391591330286793_u64 + 15727465977978435955_u64;
_10.0 = _15 as i16;
_12.2 = core::ptr::addr_of!(_2);
_3 = _8 << _8;
_22 = _8;
_17 = _14;
Goto(bb13)
}
bb15 = {
_2 = !329798720020191052330486563565464243374_u128;
_8 = !_3;
_19 = 1986205979792097691_u64 as f32;
_1 = -_4;
_12.2 = core::ptr::addr_of!(_2);
_21 = (_1, '\u{9f996}', RET);
_15 = 157_u8 ^ 175_u8;
_20 = (-6650926097616269983_i64) as isize;
_21.0 = -_1;
Goto(bb12)
}
bb16 = {
_6 = _7;
_15 = 54_u8 ^ 25_u8;
_10 = (_9,);
_12.2 = core::ptr::addr_of!(_2);
_16 = [_15,_15,_15,_15,_15];
_12.0 = 370508555_u32 << _6;
_4 = 234371474159863239_u64 as f64;
_15 = !71_u8;
_12.2 = core::ptr::addr_of!(_2);
_16 = [_15,_15,_15,_15,_15];
_15 = !42_u8;
_16 = [_15,_15,_15,_15,_15];
_3 = 26351_u16 as isize;
_11 = _5 + _1;
_12.2 = core::ptr::addr_of!(_2);
Call(_9 = core::intrinsics::transmute(_10.0), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_9 = -_10.0;
_1 = 787648088176260959_u64 as f64;
_14 = ['\u{f1822}','\u{e5ea7}','\u{83d1c}','\u{6af30}','\u{e03db}','\u{fc9f0}','\u{cd0cb}','\u{25728}'];
_3 = _7;
_12.0 = 3591526681_u32;
_7 = !_8;
_1 = 94_i8 as f64;
RET = 7047030484699763445_usize;
_2 = 279212051124616938868083466405409305382_u128 >> _3;
_6 = _8 >> _3;
_10 = (_9,);
_19 = 27824_u16 as f32;
_4 = _11;
_18 = 6237058549668315531_i64 as isize;
_17 = _14;
_4 = _11;
_17 = _14;
_19 = 19_i8 as f32;
_14 = _17;
_3 = _15 as isize;
_10 = (_9,);
_16 = [_15,_15,_15,_15,_15];
_5 = _15 as f64;
_8 = -_7;
_11 = -_4;
_8 = !_6;
_14 = _17;
_20 = _6;
match _12.0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
3591526681 => bb9,
_ => bb8
}
}
bb18 = {
place!(Field::<[u8; 5]>(Variant(_29, 0), 3)) = [_15,_15,_15,_15,_15];
_10.0 = _9;
_25 = Adt51::Variant2 { fld0: 1540918206214525298_u64 };
Goto(bb19)
}
bb19 = {
Call(_35 = dump_var(8_usize, 22_usize, Move(_22), 9_usize, Move(_9), 6_usize, Move(_6), 24_usize, Move(_24)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_35 = dump_var(8_usize, 16_usize, Move(_16), 17_usize, Move(_17), 18_usize, Move(_18), 14_usize, Move(_14)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: f64,mut _2: *const u128,mut _3: isize) -> *mut i64 {
mir! {
type RET = *mut i64;
let _4: [char; 8];
let _5: bool;
let _6: u64;
let _7: i128;
let _8: f64;
let _9: Adt49;
let _10: [u8; 5];
let _11: (f64, [i8; 5], usize);
let _12: [u32; 3];
let _13: i64;
let _14: f32;
let _15: char;
let _16: f64;
let _17: *const *const u16;
let _18: *const char;
let _19: [u128; 5];
let _20: u8;
let _21: Adt50;
let _22: f32;
let _23: &'static char;
let _24: (f64, [i8; 5], usize);
let _25: usize;
let _26: Adt56;
let _27: f64;
let _28: f64;
let _29: ();
let _30: ();
{
_3 = (-9223372036854775808_isize);
_2 = core::ptr::addr_of!((*_2));
(*_2) = 51927388574381198390966557630684575320_u128 * 153632202398011891315677780963624811864_u128;
_3 = _1 as isize;
_2 = core::ptr::addr_of!((*_2));
(*_2) = 162179618427120923751308956294950387527_u128 & 175883867429441700469973576302218485190_u128;
(*_2) = 89_i8 as u128;
_5 = !true;
_2 = core::ptr::addr_of!((*_2));
_4 = ['\u{908b8}','\u{aa0ed}','\u{bb0d4}','\u{5b608}','\u{b2e57}','\u{e5d38}','\u{718d5}','\u{f201f}'];
_6 = 13_u8 as u64;
(*_2) = 64008985456103701527515512324410512998_u128;
_6 = (*_2) as u64;
_1 = 13590342203723668679_usize as f64;
_3 = -(-13_isize);
_2 = core::ptr::addr_of!((*_2));
_5 = true;
_2 = core::ptr::addr_of!((*_2));
_1 = _3 as f64;
_3 = (-9223372036854775808_isize);
_7 = !(-34625197939758308235565781910794471973_i128);
(*_2) = 87199717037462424713911131843240404644_u128;
Goto(bb1)
}
bb1 = {
_6 = 999824850_i32 as u64;
_6 = !2901616073436174252_u64;
_5 = _1 != _1;
_3 = _5 as isize;
_4 = ['\u{76cb5}','\u{63622}','\u{55655}','\u{60fb8}','\u{10964}','\u{170d4}','\u{d8d60}','\u{e798}'];
_8 = _1;
_7 = !50406054386515752365911314441230646412_i128;
_8 = -_1;
_8 = _1 * _1;
_4 = ['\u{ffb39}','\u{b7711}','\u{8400b}','\u{a2e6a}','\u{9c5e0}','\u{25424}','\u{a6c0c}','\u{e6eb3}'];
_5 = !true;
_5 = !true;
_2 = core::ptr::addr_of!((*_2));
_3 = -9223372036854775807_isize;
(*_2) = !235767046113244504122680226123442324924_u128;
_10 = [188_u8,85_u8,100_u8,52_u8,207_u8];
_11.0 = _8 - _8;
_10 = [209_u8,89_u8,237_u8,187_u8,37_u8];
_3 = 25_isize & (-47_isize);
(*_2) = !314909255590064680231915205497144761027_u128;
_12 = [3956760860_u32,2159652750_u32,1649925443_u32];
_11.0 = _7 as f64;
(*_2) = 16880560998755387735500983635263150201_u128;
_5 = false;
_11.2 = (*_2) as usize;
_8 = _11.0 + _1;
Goto(bb2)
}
bb2 = {
_7 = _11.2 as i128;
_4 = ['\u{23f76}','\u{777b0}','\u{e8852}','\u{57923}','\u{e9cbc}','\u{4f75e}','\u{81257}','\u{d8dfa}'];
_11.1 = [61_i8,(-24_i8),78_i8,(-38_i8),(-119_i8)];
_1 = _8;
_12 = [1276924795_u32,2623080999_u32,2827587750_u32];
_3 = 27835_u16 as isize;
_11.1 = [1_i8,(-99_i8),(-85_i8),(-118_i8),7_i8];
_12 = [3915296512_u32,4264513372_u32,2157627390_u32];
(*_2) = !242978020939236075028742365470926934310_u128;
_1 = 3878002674_u32 as f64;
_3 = 34_isize + (-9223372036854775808_isize);
_4 = ['\u{a7099}','\u{b839}','\u{b5e17}','\u{8dc9d}','\u{dfb64}','\u{b1f71}','\u{b12cf}','\u{6d5f6}'];
_2 = core::ptr::addr_of!((*_2));
_11.2 = 6_usize;
RET = core::ptr::addr_of_mut!(_13);
_13 = (-1166579354559771726_i64);
_4 = ['\u{79c8a}','\u{fb9da}','\u{acb29}','\u{1082fe}','\u{579fc}','\u{14c23}','\u{8c798}','\u{4f008}'];
_11.0 = 19_i8 as f64;
_11.2 = 17669059068271780605_usize;
_10 = [26_u8,5_u8,247_u8,62_u8,202_u8];
_11.0 = _8;
Call(_11.2 = core::intrinsics::transmute(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = 7797927112330972479_u64;
_3 = 97_u8 as isize;
_13 = _7 as i64;
(*RET) = 3495588394860279106_i64;
_12 = [3898976037_u32,401930847_u32,1028883275_u32];
_11.2 = 17882793391693561991_usize;
_12 = [1150704134_u32,1184117931_u32,3696776528_u32];
_16 = _11.0;
_18 = core::ptr::addr_of!(_15);
_7 = (-86570022903623652128812787723317503328_i128);
_13 = !(-7713190561415545833_i64);
Goto(bb4)
}
bb4 = {
_7 = 145259108857569494512685943773399727872_i128;
_5 = false;
RET = core::ptr::addr_of_mut!(_13);
_8 = -_16;
_11.0 = _16 + _8;
_3 = 115_i8 as isize;
_10 = [225_u8,167_u8,185_u8,1_u8,208_u8];
_19 = [(*_2),(*_2),(*_2),(*_2),(*_2)];
_1 = -_11.0;
(*_18) = '\u{9accf}';
_11.1 = [109_i8,56_i8,121_i8,(-5_i8),(-33_i8)];
_19 = [(*_2),(*_2),(*_2),(*_2),(*_2)];
Goto(bb5)
}
bb5 = {
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = 8_u8 as i64;
(*RET) = (-8176767220076980469_i64);
_8 = _11.2 as f64;
_11.2 = _7 as usize;
_16 = 6894_i16 as f64;
_8 = _3 as f64;
Call(_11.0 = fn10(_11.2, _6, (*_18), _4, RET, (*RET), _15, (*RET), _11.1, _3, _13, _6, (*_18), _2, _11.1, _19), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_5 = false;
(*_2) = !263513602227067336578904219392329682052_u128;
_10 = [111_u8,58_u8,177_u8,18_u8,32_u8];
_13 = (-54_i8) as i64;
_4 = [(*_18),(*_18),(*_18),_15,_15,(*_18),(*_18),(*_18)];
match _7 {
0 => bb3,
145259108857569494512685943773399727872 => bb7,
_ => bb5
}
}
bb7 = {
(*_18) = '\u{d5988}';
Goto(bb8)
}
bb8 = {
_20 = !191_u8;
_16 = (-111_i8) as f64;
RET = core::ptr::addr_of_mut!((*RET));
_2 = core::ptr::addr_of!((*_2));
_12 = [4135172125_u32,1863811121_u32,2792440294_u32];
_14 = _20 as f32;
_22 = _14;
_7 = !(-62408920949949059062674611846626638633_i128);
(*_2) = !125070243023695093098809841112560454105_u128;
_18 = core::ptr::addr_of!((*_18));
_24 = (_1, _11.1, _11.2);
match _6 {
0 => bb5,
1 => bb9,
2 => bb10,
3 => bb11,
7797927112330972479 => bb13,
_ => bb12
}
}
bb9 = {
_6 = 999824850_i32 as u64;
_6 = !2901616073436174252_u64;
_5 = _1 != _1;
_3 = _5 as isize;
_4 = ['\u{76cb5}','\u{63622}','\u{55655}','\u{60fb8}','\u{10964}','\u{170d4}','\u{d8d60}','\u{e798}'];
_8 = _1;
_7 = !50406054386515752365911314441230646412_i128;
_8 = -_1;
_8 = _1 * _1;
_4 = ['\u{ffb39}','\u{b7711}','\u{8400b}','\u{a2e6a}','\u{9c5e0}','\u{25424}','\u{a6c0c}','\u{e6eb3}'];
_5 = !true;
_5 = !true;
_2 = core::ptr::addr_of!((*_2));
_3 = -9223372036854775807_isize;
(*_2) = !235767046113244504122680226123442324924_u128;
_10 = [188_u8,85_u8,100_u8,52_u8,207_u8];
_11.0 = _8 - _8;
_10 = [209_u8,89_u8,237_u8,187_u8,37_u8];
_3 = 25_isize & (-47_isize);
(*_2) = !314909255590064680231915205497144761027_u128;
_12 = [3956760860_u32,2159652750_u32,1649925443_u32];
_11.0 = _7 as f64;
(*_2) = 16880560998755387735500983635263150201_u128;
_5 = false;
_11.2 = (*_2) as usize;
_8 = _11.0 + _1;
Goto(bb2)
}
bb10 = {
_5 = false;
(*_2) = !263513602227067336578904219392329682052_u128;
_10 = [111_u8,58_u8,177_u8,18_u8,32_u8];
_13 = (-54_i8) as i64;
_4 = [(*_18),(*_18),(*_18),_15,_15,(*_18),(*_18),(*_18)];
match _7 {
0 => bb3,
145259108857569494512685943773399727872 => bb7,
_ => bb5
}
}
bb11 = {
_7 = _11.2 as i128;
_4 = ['\u{23f76}','\u{777b0}','\u{e8852}','\u{57923}','\u{e9cbc}','\u{4f75e}','\u{81257}','\u{d8dfa}'];
_11.1 = [61_i8,(-24_i8),78_i8,(-38_i8),(-119_i8)];
_1 = _8;
_12 = [1276924795_u32,2623080999_u32,2827587750_u32];
_3 = 27835_u16 as isize;
_11.1 = [1_i8,(-99_i8),(-85_i8),(-118_i8),7_i8];
_12 = [3915296512_u32,4264513372_u32,2157627390_u32];
(*_2) = !242978020939236075028742365470926934310_u128;
_1 = 3878002674_u32 as f64;
_3 = 34_isize + (-9223372036854775808_isize);
_4 = ['\u{a7099}','\u{b839}','\u{b5e17}','\u{8dc9d}','\u{dfb64}','\u{b1f71}','\u{b12cf}','\u{6d5f6}'];
_2 = core::ptr::addr_of!((*_2));
_11.2 = 6_usize;
RET = core::ptr::addr_of_mut!(_13);
_13 = (-1166579354559771726_i64);
_4 = ['\u{79c8a}','\u{fb9da}','\u{acb29}','\u{1082fe}','\u{579fc}','\u{14c23}','\u{8c798}','\u{4f008}'];
_11.0 = 19_i8 as f64;
_11.2 = 17669059068271780605_usize;
_10 = [26_u8,5_u8,247_u8,62_u8,202_u8];
_11.0 = _8;
Call(_11.2 = core::intrinsics::transmute(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb12 = {
_7 = 145259108857569494512685943773399727872_i128;
_5 = false;
RET = core::ptr::addr_of_mut!(_13);
_8 = -_16;
_11.0 = _16 + _8;
_3 = 115_i8 as isize;
_10 = [225_u8,167_u8,185_u8,1_u8,208_u8];
_19 = [(*_2),(*_2),(*_2),(*_2),(*_2)];
_1 = -_11.0;
(*_18) = '\u{9accf}';
_11.1 = [109_i8,56_i8,121_i8,(-5_i8),(-33_i8)];
_19 = [(*_2),(*_2),(*_2),(*_2),(*_2)];
Goto(bb5)
}
bb13 = {
_25 = !_11.2;
_4 = [_15,_15,(*_18),(*_18),(*_18),(*_18),_15,_15];
_23 = &(*_18);
(*_2) = 173198274124162361205644713434029192489_u128;
_1 = _24.0;
_19 = [(*_2),(*_2),(*_2),(*_2),(*_2)];
(*_2) = !114025508303608421970072424746160187883_u128;
Goto(bb14)
}
bb14 = {
_24 = (_8, _11.1, _25);
_11 = (_1, _24.1, _24.2);
_11 = _24;
RET = core::ptr::addr_of_mut!((*RET));
(*RET) = 5836454501569639037_i64;
_11 = (_24.0, _24.1, _25);
(*_2) = 209031430712668713588081222022093617737_u128;
(*RET) = (-5241269330401317727_i64) + 744286657043157535_i64;
(*RET) = 653330580582906443_i64 ^ 2665180416720590559_i64;
_13 = 3028275334656120015_i64 + (-7271664584267927594_i64);
(*_2) = _14 as u128;
(*RET) = !(-240100713565546593_i64);
_26 = Adt56::Variant1 { fld0: _3 };
(*RET) = (-7562234071977160181_i64) * (-8323427222414353838_i64);
_23 = &(*_18);
SetDiscriminant(_26, 0);
_24.1 = _11.1;
place!(Field::<Adt52>(Variant(_26, 0), 0)).fld1.fld2.0.0 = !868785761_u32;
place!(Field::<Adt52>(Variant(_26, 0), 0)).fld1.fld0.0 = !39546_u16;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(9_usize, 19_usize, Move(_19), 7_usize, Move(_7), 13_usize, Move(_13), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(9_usize, 6_usize, Move(_6), 15_usize, Move(_15), 30_usize, _30, 30_usize, _30), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: usize,mut _2: u64,mut _3: char,mut _4: [char; 8],mut _5: *mut i64,mut _6: i64,mut _7: char,mut _8: i64,mut _9: [i8; 5],mut _10: isize,mut _11: i64,mut _12: u64,mut _13: char,mut _14: *const u128,mut _15: [i8; 5],mut _16: [u128; 5]) -> f64 {
mir! {
type RET = f64;
let _17: i8;
let _18: [u64; 4];
let _19: u64;
let _20: (f64, [i8; 5], usize);
let _21: f64;
let _22: Adt52;
let _23: i32;
let _24: u32;
let _25: [i8; 5];
let _26: i8;
let _27: [u16; 4];
let _28: isize;
let _29: u16;
let _30: (u16,);
let _31: *const char;
let _32: char;
let _33: [u64; 4];
let _34: isize;
let _35: Adt56;
let _36: Adt45;
let _37: (f64, char, usize);
let _38: [u16; 4];
let _39: Adt45;
let _40: [char; 8];
let _41: Adt46;
let _42: bool;
let _43: [i64; 3];
let _44: i8;
let _45: [char; 8];
let _46: ();
let _47: ();
{
RET = 61094413_i32 as f64;
_17 = !18_i8;
(*_14) = 149914757813714486196312447410052213802_u128;
_3 = _7;
_5 = core::ptr::addr_of_mut!(_6);
_10 = !(-24_isize);
_13 = _7;
RET = _12 as f64;
_8 = (-23066_i16) as i64;
_1 = 709389406291675877_usize | 9873510071646637203_usize;
_9 = [_17,_17,_17,_17,_17];
_1 = (-6637_i16) as usize;
_17 = (-54_i8) & 55_i8;
(*_14) = RET as u128;
(*_5) = _11 - _11;
_18 = [_12,_2,_12,_2];
_11 = 40939_u16 as i64;
_11 = _8;
_1 = 5_usize * 4_usize;
_12 = _2 - _2;
_2 = _1 as u64;
_15 = [_17,_17,_17,_17,_17];
_16 = [(*_14),(*_14),(*_14),(*_14),(*_14)];
_2 = _12;
_5 = core::ptr::addr_of_mut!((*_5));
_12 = !_2;
RET = _1 as f64;
(*_5) = (-23347_i16) as i64;
_3 = _13;
Call((*_5) = fn11(_4, _9, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = core::ptr::addr_of_mut!(_6);
_5 = core::ptr::addr_of_mut!(_11);
_11 = !_6;
_13 = _7;
Call((*_5) = core::intrinsics::transmute(_12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = !_6;
_19 = !_12;
_22.fld1.fld2.0.2 = _14;
_17 = !3_i8;
_23 = 1024740369_i32 << _19;
_9 = [_17,_17,_17,_17,_17];
_20 = (RET, _15, _1);
_19 = _12 ^ _12;
Goto(bb3)
}
bb3 = {
_20 = (RET, _9, _1);
_22.fld1.fld6 = _8 ^ _6;
_8 = _7 as i64;
_20.2 = _1 - _1;
_20.2 = !_1;
_21 = _20.0 - RET;
_22.fld1.fld4 = core::ptr::addr_of!(_13);
RET = _21;
(*_14) = !15373631473075263994088337570058671332_u128;
_20.1 = [_17,_17,_17,_17,_17];
_22.fld2 = _1 as isize;
_22.fld0 = _20.2 * _1;
_22.fld1.fld0.0 = !28122_u16;
_22.fld1.fld3 = core::ptr::addr_of_mut!(_22.fld1.fld6);
Goto(bb4)
}
bb4 = {
_22.fld1.fld2.1 = -13451_i16;
_20.0 = -RET;
_8 = -_22.fld1.fld6;
_22.fld1.fld2.2 = _22.fld1.fld0.0;
_22.fld1.fld5 = !_23;
_22.fld1.fld5 = _22.fld2 as i32;
_13 = _7;
(*_5) = _22.fld1.fld6 * _22.fld1.fld6;
_24 = 1624572928_u32 & 3516973558_u32;
_22.fld1.fld0 = (_22.fld1.fld2.2,);
_17 = _23 as i8;
_24 = 3006501100_u32 * 1583215981_u32;
_22.fld1.fld2.0.0 = _24 | _24;
_13 = _7;
_22.fld1.fld2.0 = (_24, _22.fld1.fld3, _14);
_7 = _3;
_15 = _20.1;
_22.fld1.fld3 = _5;
_22.fld1.fld1 = _3;
_20.1 = _15;
_22.fld2 = _22.fld1.fld2.2 as isize;
_18 = [_12,_19,_19,_12];
_26 = _17;
(*_5) = !_6;
_22.fld1.fld0.0 = _22.fld1.fld2.2 & _22.fld1.fld2.2;
_9 = [_26,_26,_17,_17,_17];
Goto(bb5)
}
bb5 = {
_22.fld1.fld6 = _8 ^ _11;
Goto(bb6)
}
bb6 = {
_8 = !_11;
Goto(bb7)
}
bb7 = {
_22.fld1.fld5 = !_23;
_22.fld1.fld2.2 = _22.fld1.fld0.0 * _22.fld1.fld0.0;
_8 = false as i64;
_16 = [(*_14),(*_14),(*_14),(*_14),(*_14)];
_17 = _26 << (*_5);
_27 = [_22.fld1.fld2.2,_22.fld1.fld2.2,_22.fld1.fld2.2,_22.fld1.fld2.2];
_22.fld1.fld1 = _3;
_29 = (*_14) as u16;
_23 = _22.fld1.fld5 + _22.fld1.fld5;
_22.fld1.fld2.1 = (-22416_i16) | 7760_i16;
Goto(bb8)
}
bb8 = {
_15 = [_17,_17,_17,_17,_17];
_28 = (*_14) as isize;
_22.fld1.fld3 = _22.fld1.fld2.0.1;
_6 = !_22.fld1.fld6;
_18 = [_19,_12,_19,_2];
_22.fld1.fld3 = core::ptr::addr_of_mut!(_6);
_22.fld1.fld2.2 = !_29;
_16 = [(*_14),(*_14),(*_14),(*_14),(*_14)];
_13 = _3;
_5 = core::ptr::addr_of_mut!((*_5));
_22.fld1.fld4 = core::ptr::addr_of!(_3);
_20.0 = -_21;
Goto(bb9)
}
bb9 = {
_22.fld1.fld2.2 = _22.fld1.fld0.0;
_24 = !_22.fld1.fld2.0.0;
_9 = _15;
_15 = _9;
_18 = [_19,_12,_19,_19];
_22.fld2 = _17 as isize;
_22.fld1.fld2.2 = _22.fld1.fld0.0 & _22.fld1.fld0.0;
_30.0 = _22.fld1.fld2.2 ^ _22.fld1.fld0.0;
_32 = _13;
_31 = core::ptr::addr_of!(_22.fld1.fld1);
_22.fld1.fld2.1 = 9143_i16 >> _22.fld1.fld6;
_3 = _22.fld1.fld1;
_22.fld1.fld6 = _6 * (*_5);
_20.0 = -RET;
(*_14) = !232586752035441539266202236699480412825_u128;
_22.fld1.fld6 = -_6;
_18 = [_2,_12,_19,_2];
_3 = _13;
_13 = _7;
Goto(bb10)
}
bb10 = {
_22.fld1.fld1 = _32;
_22.fld1.fld7 = _14;
_6 = _22.fld2 as i64;
_37.1 = _13;
_22.fld1.fld2.2 = !_30.0;
(*_5) = _22.fld1.fld1 as i64;
_12 = _2 << _17;
(*_14) = 230892865083947875263185800894187708246_u128 >> _22.fld2;
_22.fld1.fld3 = core::ptr::addr_of_mut!(_8);
_23 = _22.fld1.fld5 + _22.fld1.fld5;
_34 = _1 as isize;
_22.fld1.fld2.2 = _30.0 ^ _22.fld1.fld0.0;
_21 = 170_u8 as f64;
_20.1 = [_17,_17,_17,_17,_17];
_33 = [_19,_12,_19,_12];
_37.0 = RET - RET;
_37.0 = _21;
_18 = _33;
_37.2 = _22.fld0 + _22.fld0;
_38 = [_22.fld1.fld2.2,_22.fld1.fld0.0,_30.0,_22.fld1.fld2.2];
_30.0 = _22.fld1.fld2.2;
_1 = _24 as usize;
_19 = _22.fld1.fld6 as u64;
_22.fld0 = (*_14) as usize;
_22.fld1.fld5 = _22.fld1.fld2.1 as i32;
Goto(bb11)
}
bb11 = {
_19 = !_12;
_26 = _22.fld1.fld1 as i8;
_20 = (_21, _15, _37.2);
_30 = (_22.fld1.fld2.2,);
(*_31) = _13;
_7 = (*_31);
_23 = _22.fld2 as i32;
Goto(bb12)
}
bb12 = {
_22.fld1.fld0 = (_22.fld1.fld2.2,);
_37.2 = _1;
_9 = _15;
_14 = core::ptr::addr_of!((*_14));
_30.0 = (*_14) as u16;
_20.2 = !_22.fld0;
Goto(bb13)
}
bb13 = {
_22.fld1.fld1 = _37.1;
_40 = [_3,_7,_37.1,_13,_32,_3,_32,(*_31)];
_7 = _37.1;
_1 = !_20.2;
_23 = _22.fld1.fld5;
_22.fld1.fld1 = _13;
_32 = _13;
_22.fld1.fld2.2 = _22.fld1.fld6 as u16;
_37.1 = _13;
_22.fld0 = _20.2;
(*_31) = _37.1;
_27 = _38;
_15 = _9;
_43 = [_22.fld1.fld6,_22.fld1.fld6,_22.fld1.fld6];
Goto(bb14)
}
bb14 = {
(*_31) = _7;
_30.0 = !_22.fld1.fld2.2;
_10 = _22.fld1.fld2.1 as isize;
_22.fld2 = _10;
_3 = _32;
_20.2 = _1;
_7 = _13;
_22.fld1.fld5 = _23;
_22.fld1.fld2.1 = (-21881_i16) | (-29471_i16);
_15 = [_17,_17,_17,_17,_17];
_6 = !_8;
_14 = core::ptr::addr_of!((*_14));
_15 = [_17,_17,_17,_17,_26];
_40 = _4;
_38 = [_30.0,_30.0,_30.0,_30.0];
_8 = _1 as i64;
_22.fld1.fld2.0.0 = _24;
_3 = _7;
_13 = (*_31);
(*_31) = _37.1;
_22.fld1.fld7 = _22.fld1.fld2.0.2;
_25 = _15;
_45 = [(*_31),_7,_22.fld1.fld1,_13,_13,_3,_13,_32];
_20.0 = RET + RET;
_37.1 = _7;
_12 = _19 >> _10;
Goto(bb15)
}
bb15 = {
Call(_46 = dump_var(10_usize, 23_usize, Move(_23), 13_usize, Move(_13), 27_usize, Move(_27), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_46 = dump_var(10_usize, 9_usize, Move(_9), 16_usize, Move(_16), 32_usize, Move(_32), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(10_usize, 28_usize, Move(_28), 8_usize, Move(_8), 10_usize, Move(_10), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_46 = dump_var(10_usize, 18_usize, Move(_18), 11_usize, Move(_11), 34_usize, Move(_34), 2_usize, Move(_2)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: [char; 8],mut _2: [i8; 5],mut _3: f64) -> i64 {
mir! {
type RET = i64;
let _4: Adt47;
let _5: Adt55;
let _6: char;
let _7: i64;
let _8: i64;
let _9: [u8; 5];
let _10: i64;
let _11: Adt47;
let _12: Adt51;
let _13: (u16,);
let _14: usize;
let _15: [char; 8];
let _16: (i16,);
let _17: [u16; 4];
let _18: Adt56;
let _19: bool;
let _20: (f64, [i8; 5], usize);
let _21: [u32; 3];
let _22: [u128; 5];
let _23: ();
let _24: ();
{
_1 = ['\u{e3c86}','\u{e4ea}','\u{19ec7}','\u{3ec4f}','\u{9be62}','\u{1b67a}','\u{92a0f}','\u{555c1}'];
RET = 6856144864010303480_i64 ^ 6521769603403781459_i64;
RET = false as i64;
_2 = [81_i8,77_i8,(-111_i8),88_i8,(-71_i8)];
_2 = [43_i8,(-98_i8),(-14_i8),80_i8,(-112_i8)];
Call(_3 = fn12(_1, _1, _1, RET, RET, _1, _1, _1, _1, RET, _1, _1, RET, _1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = ['\u{41555}','\u{ae072}','\u{2e50a}','\u{ba9ba}','\u{a0df7}','\u{1cb96}','\u{252e9}','\u{eccc9}'];
_2 = [(-67_i8),86_i8,(-109_i8),(-78_i8),76_i8];
_1 = ['\u{bb9c0}','\u{5a8ea}','\u{26317}','\u{64d52}','\u{105b8}','\u{106634}','\u{abd9f}','\u{ebdd4}'];
RET = 7599568041834306784_i64;
match RET {
0 => bb2,
7599568041834306784 => bb4,
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
_2 = [19_i8,14_i8,(-36_i8),14_i8,(-36_i8)];
RET = 8097574808910066292_i64;
RET = _3 as i64;
_2 = [(-66_i8),(-124_i8),(-82_i8),(-8_i8),(-20_i8)];
_3 = 6919614094478105900_u64 as f64;
_1 = ['\u{a5b0b}','\u{f8b7b}','\u{63378}','\u{32745}','\u{b0a92}','\u{99e27}','\u{7bce6}','\u{b9bd6}'];
_1 = ['\u{49e34}','\u{467f6}','\u{42b7c}','\u{ea3cc}','\u{a864d}','\u{c1954}','\u{d84f3}','\u{9d4d3}'];
_1 = ['\u{f05af}','\u{c65f1}','\u{cd697}','\u{c84c2}','\u{b7800}','\u{5b9f6}','\u{1faf4}','\u{d9472}'];
_3 = (-57_isize) as f64;
_2 = [(-87_i8),(-57_i8),(-54_i8),67_i8,3_i8];
_2 = [125_i8,40_i8,17_i8,(-128_i8),(-53_i8)];
_6 = '\u{8b66f}';
_3 = 150_u8 as f64;
_3 = 50580_u16 as f64;
_1 = [_6,_6,_6,_6,_6,_6,_6,_6];
_2 = [(-55_i8),(-89_i8),(-71_i8),(-24_i8),(-35_i8)];
RET = !(-4678700876934930465_i64);
_2 = [(-105_i8),118_i8,(-41_i8),68_i8,64_i8];
_7 = RET * RET;
_2 = [123_i8,101_i8,58_i8,(-118_i8),26_i8];
RET = _7;
RET = 2365118604_u32 as i64;
_1 = [_6,_6,_6,_6,_6,_6,_6,_6];
RET = 187_u8 as i64;
_6 = '\u{7be4a}';
_8 = RET;
Goto(bb5)
}
bb5 = {
_3 = (-70637074325466228742602990704298790723_i128) as f64;
RET = -_7;
_9 = [189_u8,251_u8,132_u8,97_u8,41_u8];
RET = 3837564224766970672_u64 as i64;
RET = _8 | _7;
RET = 286265831787716024797125356472650816175_u128 as i64;
_10 = 3692_u16 as i64;
Goto(bb6)
}
bb6 = {
_1 = [_6,_6,_6,_6,_6,_6,_6,_6];
_6 = '\u{2b18e}';
_9 = [219_u8,111_u8,244_u8,221_u8,241_u8];
_2 = [41_i8,(-78_i8),(-95_i8),(-30_i8),(-1_i8)];
RET = _7;
_9 = [125_u8,54_u8,239_u8,109_u8,252_u8];
_6 = '\u{22331}';
_1 = [_6,_6,_6,_6,_6,_6,_6,_6];
_10 = _7;
_1 = [_6,_6,_6,_6,_6,_6,_6,_6];
_9 = [213_u8,138_u8,205_u8,74_u8,182_u8];
RET = _10 & _10;
_7 = RET - RET;
_3 = _7 as f64;
_9 = [221_u8,54_u8,22_u8,161_u8,69_u8];
_2 = [54_i8,13_i8,(-15_i8),(-105_i8),(-104_i8)];
_9 = [16_u8,108_u8,209_u8,153_u8,70_u8];
_9 = [8_u8,148_u8,11_u8,250_u8,197_u8];
_7 = -RET;
_1 = [_6,_6,_6,_6,_6,_6,_6,_6];
_10 = _7 & _7;
_9 = [111_u8,229_u8,176_u8,111_u8,244_u8];
RET = _10;
_1 = [_6,_6,_6,_6,_6,_6,_6,_6];
Goto(bb7)
}
bb7 = {
_3 = 206495555451295861389150666376895124289_u128 as f64;
RET = _10;
RET = 7993567590307974412_usize as i64;
_12 = Adt51::Variant2 { fld0: 8075724833936819163_u64 };
RET = _7;
_2 = [(-110_i8),(-21_i8),(-104_i8),(-48_i8),126_i8];
_2 = [76_i8,(-118_i8),18_i8,(-16_i8),116_i8];
place!(Field::<u64>(Variant(_12, 2), 0)) = (-164771842575274180325615712405771635912_i128) as u64;
_7 = -_10;
_6 = '\u{ad750}';
_8 = RET >> _7;
_1 = [_6,_6,_6,_6,_6,_6,_6,_6];
place!(Field::<u64>(Variant(_12, 2), 0)) = 1122561385_u32 as u64;
_13.0 = 17146_u16 - 45461_u16;
_14 = 2459186105_u32 as usize;
_7 = (-118_i8) as i64;
_16.0 = -(-18767_i16);
_10 = !_8;
SetDiscriminant(_12, 0);
place!(Field::<(f64, char, usize)>(Variant(_12, 0), 0)).2 = _14 * _14;
_19 = true;
place!(Field::<[u16; 4]>(Variant(_12, 0), 2)) = [_13.0,_13.0,_13.0,_13.0];
_19 = false;
Goto(bb8)
}
bb8 = {
place!(Field::<(f64, char, usize)>(Variant(_12, 0), 0)) = (_3, _6, _14);
place!(Field::<(f64, char, usize)>(Variant(_12, 0), 0)).2 = _14;
_3 = Field::<(f64, char, usize)>(Variant(_12, 0), 0).0;
RET = _8;
place!(Field::<(f64, char, usize)>(Variant(_12, 0), 0)).1 = _6;
_13 = (33117_u16,);
_20 = (_3, _2, _14);
place!(Field::<(f64, char, usize)>(Variant(_12, 0), 0)).0 = -_20.0;
Goto(bb9)
}
bb9 = {
Call(_23 = dump_var(11_usize, 6_usize, Move(_6), 10_usize, Move(_10), 14_usize, Move(_14), 2_usize, Move(_2)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_23 = dump_var(11_usize, 13_usize, Move(_13), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: [char; 8],mut _2: [char; 8],mut _3: [char; 8],mut _4: i64,mut _5: i64,mut _6: [char; 8],mut _7: [char; 8],mut _8: [char; 8],mut _9: [char; 8],mut _10: i64,mut _11: [char; 8],mut _12: [char; 8],mut _13: i64,mut _14: [char; 8],mut _15: [char; 8]) -> f64 {
mir! {
type RET = f64;
let _16: u8;
let _17: Adt43;
let _18: usize;
let _19: *mut i64;
let _20: ((u32, *mut i64, *const u128), i16, u16);
let _21: f64;
let _22: [u16; 1];
let _23: [u16; 4];
let _24: [u8; 5];
let _25: bool;
let _26: [char; 8];
let _27: *mut *mut [char; 8];
let _28: Adt44;
let _29: bool;
let _30: isize;
let _31: [u32; 3];
let _32: f32;
let _33: isize;
let _34: char;
let _35: f64;
let _36: u64;
let _37: (u16,);
let _38: isize;
let _39: *mut i64;
let _40: *mut [char; 8];
let _41: i16;
let _42: Adt44;
let _43: [u8; 5];
let _44: [u16; 4];
let _45: (u32, *mut i64, *const u128);
let _46: u64;
let _47: Adt54;
let _48: isize;
let _49: u64;
let _50: Adt45;
let _51: i32;
let _52: bool;
let _53: bool;
let _54: u32;
let _55: Adt57;
let _56: [u64; 4];
let _57: [u16; 1];
let _58: f32;
let _59: u8;
let _60: i16;
let _61: f32;
let _62: f32;
let _63: ();
let _64: ();
{
_3 = ['\u{b6984}','\u{31cc5}','\u{c9183}','\u{20934}','\u{6e296}','\u{da573}','\u{3d62e}','\u{4848e}'];
_11 = _15;
_13 = -_5;
_10 = 28428_i16 as i64;
_5 = _13;
_4 = _10 - _10;
RET = 7_usize as f64;
_14 = ['\u{1fb58}','\u{dfcdd}','\u{1004dc}','\u{e3204}','\u{9c9c5}','\u{a7152}','\u{b8c2b}','\u{84f71}'];
_3 = ['\u{23e4f}','\u{2a519}','\u{3b2be}','\u{a2844}','\u{84d81}','\u{7a16e}','\u{1ee24}','\u{a61e3}'];
_11 = ['\u{ed2f9}','\u{46799}','\u{d659d}','\u{2964f}','\u{86e51}','\u{c6f53}','\u{2109a}','\u{40710}'];
_5 = !_10;
_8 = ['\u{60d36}','\u{d40c2}','\u{988c4}','\u{8c64d}','\u{a0bdd}','\u{c60df}','\u{aff49}','\u{5df65}'];
_7 = ['\u{8c147}','\u{f80fb}','\u{843ab}','\u{c9d88}','\u{6079b}','\u{b75e8}','\u{18bf3}','\u{8860b}'];
_12 = ['\u{5a556}','\u{6a1c9}','\u{13e91}','\u{173a5}','\u{1d489}','\u{f8bed}','\u{3a090}','\u{4eb1e}'];
_10 = _13;
_1 = _11;
_6 = _3;
Goto(bb1)
}
bb1 = {
_15 = ['\u{fbcf8}','\u{71a17}','\u{f36e1}','\u{d3ca3}','\u{43297}','\u{c8de7}','\u{8f8ca}','\u{dd816}'];
_13 = !_5;
_6 = ['\u{a69fe}','\u{58a07}','\u{7defa}','\u{b7b53}','\u{2926e}','\u{941be}','\u{61ce8}','\u{91e70}'];
_5 = !_4;
RET = 3406056426_u32 as f64;
RET = 118_u8 as f64;
_9 = _8;
_2 = ['\u{a2496}','\u{cc633}','\u{17307}','\u{ebdf9}','\u{3a665}','\u{4d096}','\u{6e548}','\u{e075c}'];
_2 = ['\u{d48f0}','\u{87c78}','\u{4dcae}','\u{10c73d}','\u{332cd}','\u{9c340}','\u{f6e73}','\u{56d2e}'];
_8 = ['\u{10c061}','\u{6cb74}','\u{1826}','\u{baa64}','\u{3ac93}','\u{460a6}','\u{4bc61}','\u{481fb}'];
_15 = ['\u{e18db}','\u{b1bff}','\u{16cd5}','\u{103b39}','\u{77049}','\u{2edff}','\u{c1152}','\u{87002}'];
RET = 204909332199028496898348412638197041673_u128 as f64;
_7 = ['\u{48e9}','\u{23296}','\u{53102}','\u{81911}','\u{aab84}','\u{d75ad}','\u{24ded}','\u{330f4}'];
_10 = _13;
_6 = ['\u{8e87b}','\u{bebc3}','\u{fc44f}','\u{c8d20}','\u{bd39e}','\u{105be}','\u{7a03}','\u{42ef8}'];
RET = 53_i8 as f64;
_9 = _15;
_1 = ['\u{d3e53}','\u{20499}','\u{a5e03}','\u{6b7b3}','\u{78350}','\u{9a70}','\u{d296e}','\u{10eed}'];
_3 = ['\u{a3bcf}','\u{2aa29}','\u{50f2c}','\u{17346}','\u{aca11}','\u{fed3b}','\u{bdcf0}','\u{a214b}'];
_9 = _12;
_2 = ['\u{342bc}','\u{2abbf}','\u{9abdc}','\u{79ee5}','\u{7bc5}','\u{107119}','\u{aea54}','\u{f6f36}'];
_9 = _3;
Goto(bb2)
}
bb2 = {
_5 = (-157378490258006986575672085853355821547_i128) as i64;
Goto(bb3)
}
bb3 = {
_2 = ['\u{33614}','\u{bd58a}','\u{75390}','\u{e891a}','\u{f7753}','\u{3d874}','\u{10d79a}','\u{d68eb}'];
_15 = ['\u{e5985}','\u{1562f}','\u{84820}','\u{774e}','\u{4c4a8}','\u{2cd3d}','\u{104d4d}','\u{10a4a5}'];
RET = (-2189_i16) as f64;
RET = (-21965_i16) as f64;
_20.1 = (-18920_i16);
_19 = core::ptr::addr_of_mut!(_10);
_21 = (*_19) as f64;
_20.2 = 7362_u16 | 27039_u16;
(*_19) = _13;
_6 = _9;
_20.0.0 = (-32489905205047141986548503910312013157_i128) as u32;
RET = _21 + _21;
_18 = _20.1 as usize;
_6 = ['\u{55c53}','\u{6f372}','\u{63481}','\u{1787b}','\u{bbc79}','\u{718f8}','\u{9f62d}','\u{d061a}'];
_4 = (*_19) * _10;
_6 = ['\u{aaedc}','\u{57ed3}','\u{eb545}','\u{62d49}','\u{17709}','\u{69038}','\u{b61ec}','\u{32057}'];
RET = _21 - _21;
_14 = _15;
_15 = _14;
_8 = _12;
_25 = true;
_6 = _3;
_24 = [221_u8,47_u8,128_u8,27_u8,161_u8];
_19 = core::ptr::addr_of_mut!(_5);
_24 = [192_u8,145_u8,42_u8,131_u8,191_u8];
Goto(bb4)
}
bb4 = {
_24 = [7_u8,232_u8,3_u8,128_u8,178_u8];
_15 = ['\u{b0e50}','\u{a7bda}','\u{95657}','\u{f71d2}','\u{23019}','\u{68d7c}','\u{ff1c3}','\u{5b2ef}'];
_24 = [233_u8,1_u8,198_u8,58_u8,203_u8];
(*_19) = _4;
_16 = 108_u8 & 235_u8;
_22 = [_20.2];
_20.1 = _20.0.0 as i16;
_4 = (-94191065470903329607885377177296634582_i128) as i64;
_20.1 = 122622967599383739400929552475444682899_i128 as i16;
_7 = ['\u{e6e21}','\u{3ff6b}','\u{8e6b4}','\u{ede1a}','\u{f7560}','\u{54103}','\u{109cbc}','\u{29e2f}'];
_2 = ['\u{c1ccd}','\u{a8f11}','\u{16882}','\u{e2dfd}','\u{b5e8c}','\u{ae147}','\u{dcdff}','\u{5422b}'];
_2 = _12;
_20.1 = 24605_i16;
Goto(bb5)
}
bb5 = {
_4 = (*_19) >> _18;
_26 = ['\u{cb622}','\u{aaeb6}','\u{99be5}','\u{b4845}','\u{7b037}','\u{9b7a}','\u{f33a9}','\u{d5eee}'];
_28.fld1 = '\u{443c4}';
_9 = [_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1];
_28.fld0.0 = _28.fld1 as u16;
_28.fld6 = !_10;
_5 = -_4;
_5 = _4 + _4;
_29 = !_25;
_29 = !_25;
_28.fld5 = (-1588844294_i32) ^ 2131933030_i32;
_20.0.0 = (*_19) as u32;
_21 = -RET;
_15 = _14;
_11 = [_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1];
_2 = [_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1];
_28.fld2.1 = _28.fld1 as i16;
RET = _21;
_28.fld4 = core::ptr::addr_of!(_28.fld1);
_25 = _20.1 > _28.fld2.1;
Goto(bb6)
}
bb6 = {
_20.0.0 = 3588314488_u32;
_30 = 9223372036854775807_isize;
(*_19) = _28.fld6;
_10 = !_4;
_20.0.1 = core::ptr::addr_of_mut!((*_19));
_18 = _30 as usize;
_1 = [_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1,_28.fld1];
_5 = _28.fld2.1 as i64;
_20.1 = -_28.fld2.1;
(*_19) = _4;
_28.fld2.0.0 = !_20.0.0;
_28.fld4 = core::ptr::addr_of!(_28.fld1);
_5 = _4 | _10;
_28.fld2.0.0 = _20.0.0 % _20.0.0;
_28.fld2.0.1 = core::ptr::addr_of_mut!(_28.fld6);
_10 = _30 as i64;
_20.0.0 = _28.fld2.0.0 * _28.fld2.0.0;
_26 = _8;
_28.fld0 = (_20.2,);
_20.1 = _18 as i16;
Goto(bb7)
}
bb7 = {
_28.fld2.2 = _20.2 | _20.2;
_21 = RET;
_28.fld2.2 = _21 as u16;
_34 = _28.fld1;
match _30 {
0 => bb1,
1 => bb6,
9223372036854775807 => bb8,
_ => bb3
}
}
bb8 = {
_28.fld1 = _34;
_33 = _30 - _30;
_10 = 339003728165223261268385778484731257438_u128 as i64;
_37.0 = !_28.fld2.2;
_12 = _8;
_24 = [_16,_16,_16,_16,_16];
Call(_28.fld7 = fn13((*_19), (*_19), _8, _6, _14, _15, _5, _24, _20.0.1, _28.fld0.0, _3, _8), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_8 = [_28.fld1,_34,_34,_28.fld1,_34,_34,_28.fld1,_28.fld1];
_20.0 = (_28.fld2.0.0, _19, _28.fld7);
_20.2 = _34 as u16;
_36 = _20.0.0 as u64;
_38 = _33;
_20.0.2 = _28.fld7;
_28.fld2.2 = _20.2 ^ _20.2;
_1 = [_28.fld1,_34,_34,_28.fld1,_34,_28.fld1,_28.fld1,_28.fld1];
_28.fld6 = (*_19);
_35 = _21 + _21;
_12 = _15;
_42.fld2.0.1 = _28.fld2.0.1;
_28.fld3 = core::ptr::addr_of_mut!(_28.fld6);
_28.fld7 = _20.0.2;
_26 = [_34,_34,_28.fld1,_34,_34,_28.fld1,_34,_28.fld1];
_45.2 = _28.fld7;
_42.fld1 = _28.fld1;
_41 = _28.fld2.1 | _20.1;
_42.fld2.2 = _28.fld2.2 * _37.0;
_37.0 = _28.fld2.2;
_22 = [_37.0];
_28.fld2 = _20;
_42.fld2.0 = (_28.fld2.0.0, _19, _20.0.2);
match _30 {
9223372036854775807 => bb10,
_ => bb2
}
}
bb10 = {
_42.fld3 = _19;
_35 = _21;
_4 = _5 + (*_19);
_42.fld2.0.0 = _28.fld2.0.0 - _20.0.0;
_28.fld2.0 = (_20.0.0, _42.fld2.0.1, _45.2);
_42.fld7 = _28.fld7;
_42.fld2.0 = (_20.0.0, _28.fld3, _45.2);
_32 = _20.2 as f32;
_20.1 = _41 << _28.fld2.0.0;
_1 = _6;
_42.fld2 = (_28.fld2.0, _20.1, _20.2);
_1 = _14;
_10 = !_4;
(*_19) = _4 >> _10;
_27 = core::ptr::addr_of_mut!(_40);
_18 = 253465130612446687613921300076059394387_u128 as usize;
_35 = -RET;
(*_27) = core::ptr::addr_of_mut!(_9);
_49 = !_36;
_16 = _37.0 as u8;
match _30 {
0 => bb3,
1 => bb8,
9223372036854775807 => bb12,
_ => bb11
}
}
bb11 = {
_28.fld2.2 = _20.2 | _20.2;
_21 = RET;
_28.fld2.2 = _21 as u16;
_34 = _28.fld1;
match _30 {
0 => bb1,
1 => bb6,
9223372036854775807 => bb8,
_ => bb3
}
}
bb12 = {
_18 = !3_usize;
_48 = -_38;
_44 = [_28.fld2.2,_28.fld0.0,_28.fld0.0,_20.2];
_42.fld0 = (_42.fld2.2,);
_28.fld2.0.0 = 95_i8 as u32;
_28.fld2.0.1 = _42.fld3;
_31 = [_20.0.0,_42.fld2.0.0,_20.0.0];
_42.fld2 = _28.fld2;
_25 = _29 ^ _29;
_32 = _18 as f32;
_45.0 = _20.0.0;
Goto(bb13)
}
bb13 = {
_28.fld5 = (-35_i8) as i32;
_3 = [_42.fld1,_28.fld1,_42.fld1,_34,_42.fld1,_28.fld1,_42.fld1,_42.fld1];
_55.fld0 = _30 as i32;
_31 = [_45.0,_45.0,_20.0.0];
_42.fld0 = (_28.fld0.0,);
_53 = !_25;
_25 = _53 & _53;
_28.fld6 = _16 as i64;
_28.fld0 = _42.fld0;
Goto(bb14)
}
bb14 = {
_55 = Adt57 { fld0: _28.fld5,fld1: _44,fld2: _20.0.2 };
_28.fld2.0.2 = _45.2;
_24 = [_16,_16,_16,_16,_16];
_14 = [_42.fld1,_42.fld1,_28.fld1,_34,_34,_28.fld1,_42.fld1,_42.fld1];
_54 = _45.0;
Goto(bb15)
}
bb15 = {
Call(_63 = dump_var(12_usize, 33_usize, Move(_33), 53_usize, Move(_53), 36_usize, Move(_36), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_63 = dump_var(12_usize, 3_usize, Move(_3), 5_usize, Move(_5), 14_usize, Move(_14), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_63 = dump_var(12_usize, 25_usize, Move(_25), 49_usize, Move(_49), 18_usize, Move(_18), 34_usize, Move(_34)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_63 = dump_var(12_usize, 7_usize, Move(_7), 8_usize, Move(_8), 22_usize, Move(_22), 10_usize, Move(_10)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_63 = dump_var(12_usize, 12_usize, Move(_12), 64_usize, _64, 64_usize, _64, 64_usize, _64), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: i64,mut _2: i64,mut _3: [char; 8],mut _4: [char; 8],mut _5: [char; 8],mut _6: [char; 8],mut _7: i64,mut _8: [u8; 5],mut _9: *mut i64,mut _10: u16,mut _11: [char; 8],mut _12: [char; 8]) -> *const u128 {
mir! {
type RET = *const u128;
let _13: [u32; 3];
let _14: bool;
let _15: [u16; 4];
let _16: isize;
let _17: bool;
let _18: (i16,);
let _19: (u16,);
let _20: [char; 8];
let _21: [u128; 5];
let _22: [i64; 3];
let _23: [i8; 5];
let _24: isize;
let _25: (f64, char, usize);
let _26: (f64, [i8; 5], usize);
let _27: isize;
let _28: [u16; 1];
let _29: char;
let _30: u64;
let _31: bool;
let _32: *mut [char; 8];
let _33: [u128; 5];
let _34: i64;
let _35: bool;
let _36: u16;
let _37: Adt45;
let _38: Adt52;
let _39: isize;
let _40: bool;
let _41: (i16,);
let _42: isize;
let _43: *mut (*mut i128, &'static char, i64);
let _44: (f64, char, usize);
let _45: i32;
let _46: bool;
let _47: f64;
let _48: [u128; 5];
let _49: bool;
let _50: f64;
let _51: usize;
let _52: i32;
let _53: [u128; 5];
let _54: [u64; 4];
let _55: f32;
let _56: (f64, char, usize);
let _57: char;
let _58: [u128; 5];
let _59: i16;
let _60: i32;
let _61: Adt51;
let _62: Adt50;
let _63: (u32, *mut i64, *const u128);
let _64: bool;
let _65: u8;
let _66: i128;
let _67: char;
let _68: [i8; 5];
let _69: [u16; 4];
let _70: (f64, char, usize);
let _71: [u32; 3];
let _72: i8;
let _73: bool;
let _74: char;
let _75: f32;
let _76: [u32; 3];
let _77: bool;
let _78: u8;
let _79: isize;
let _80: isize;
let _81: (u16,);
let _82: isize;
let _83: Adt43;
let _84: [u64; 4];
let _85: isize;
let _86: [i64; 3];
let _87: Adt56;
let _88: [i64; 3];
let _89: i8;
let _90: [u128; 5];
let _91: [u64; 4];
let _92: char;
let _93: isize;
let _94: bool;
let _95: i32;
let _96: ();
let _97: ();
{
_7 = !_1;
_11 = _4;
_12 = ['\u{fa48}','\u{b21f6}','\u{102700}','\u{663c5}','\u{2a611}','\u{576cc}','\u{5dca8}','\u{de669}'];
_12 = ['\u{c90a7}','\u{66c18}','\u{255e3}','\u{a4cc}','\u{5a28c}','\u{89177}','\u{d30a9}','\u{1fcb0}'];
_9 = core::ptr::addr_of_mut!((*_9));
_1 = (*_9);
_9 = core::ptr::addr_of_mut!((*_9));
_8 = [78_u8,25_u8,28_u8,240_u8,197_u8];
_12 = _3;
Goto(bb1)
}
bb1 = {
_4 = ['\u{bdeb6}','\u{106d0e}','\u{e07ef}','\u{c5a9c}','\u{41189}','\u{926a5}','\u{14ec4}','\u{ba2c9}'];
_14 = _7 < (*_9);
(*_9) = _7 & _2;
_14 = !false;
_2 = (*_9);
_11 = _12;
_9 = core::ptr::addr_of_mut!((*_9));
_1 = !(*_9);
_13 = [1798266791_u32,4204139068_u32,1104645360_u32];
_16 = !9223372036854775807_isize;
_12 = _6;
Goto(bb2)
}
bb2 = {
_9 = core::ptr::addr_of_mut!((*_9));
(*_9) = _1;
_19.0 = !_10;
_7 = _1 ^ _1;
_19 = (_10,);
_17 = !_14;
_16 = -(-9223372036854775808_isize);
_17 = _14;
(*_9) = _7 + _7;
_9 = core::ptr::addr_of_mut!(_7);
_6 = _11;
_8 = [76_u8,249_u8,188_u8,38_u8,232_u8];
_14 = (*_9) != (*_9);
Goto(bb3)
}
bb3 = {
_17 = !_14;
_20 = _3;
_8 = [188_u8,252_u8,234_u8,99_u8,50_u8];
_14 = _17 | _17;
_13 = [2512349797_u32,1102180174_u32,3335377861_u32];
_15 = [_19.0,_10,_10,_19.0];
_4 = ['\u{c9fa3}','\u{4da08}','\u{ad1d6}','\u{5636e}','\u{d95bd}','\u{509d}','\u{32f40}','\u{6c93c}'];
_4 = _5;
_19 = (_10,);
_13 = [3379619508_u32,151963086_u32,1173358328_u32];
_23 = [76_i8,(-50_i8),(-30_i8),(-115_i8),58_i8];
_15 = [_10,_19.0,_10,_10];
_26.0 = 25_u8 as f64;
_26.2 = _14 as usize;
_25.0 = _26.0 + _26.0;
_22 = [(*_9),(*_9),_1];
_18 = ((-14102_i16),);
_28 = [_19.0];
_21 = [304691199169927038126867928250569474432_u128,8082711478801692830959934392146593623_u128,136852765572325736951522962071503855518_u128,72106978929830735347309514098743466342_u128,297515299806464003075893018834603317656_u128];
_25.1 = '\u{68aa7}';
Goto(bb4)
}
bb4 = {
_26.1 = _23;
_26.2 = !1_usize;
_25.0 = 1825507542_u32 as f64;
_2 = !_7;
_7 = _2;
_7 = _2 & _2;
_19.0 = !_10;
_5 = [_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1];
_27 = _16 & _16;
_4 = [_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1];
_5 = [_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1];
_9 = core::ptr::addr_of_mut!((*_9));
match _18.0 {
0 => bb1,
340282366920938463463374607431768197354 => bb5,
_ => bb2
}
}
bb5 = {
_29 = _25.1;
_25.2 = _26.2 ^ _26.2;
_12 = [_29,_29,_29,_25.1,_25.1,_25.1,_29,_29];
_5 = [_29,_29,_29,_29,_29,_25.1,_25.1,_25.1];
_6 = _3;
_27 = !_16;
Goto(bb6)
}
bb6 = {
_10 = 290753722990666521375190612760731464235_u128 as u16;
_26.0 = -_25.0;
_8 = [105_u8,213_u8,128_u8,71_u8,71_u8];
_27 = _16 ^ _16;
_2 = !(*_9);
_20 = [_25.1,_29,_25.1,_29,_29,_29,_29,_25.1];
_31 = _16 == _16;
_12 = [_29,_29,_25.1,_25.1,_29,_25.1,_25.1,_25.1];
Goto(bb7)
}
bb7 = {
_14 = !_17;
_4 = [_25.1,_25.1,_25.1,_25.1,_29,_25.1,_25.1,_29];
_26 = (_25.0, _23, _25.2);
_2 = -_7;
_19 = (_10,);
_26.0 = _25.0 - _25.0;
_2 = _1 >> _1;
Goto(bb8)
}
bb8 = {
(*_9) = _2;
_30 = !1215966869920074673_u64;
_25.0 = _26.0 * _26.0;
_34 = (-34275185016889878047836170913243606119_i128) as i64;
_23 = [(-23_i8),96_i8,(-122_i8),116_i8,59_i8];
_15 = [_10,_19.0,_10,_19.0];
_7 = _2 + _34;
_4 = [_29,_29,_25.1,_25.1,_25.1,_29,_25.1,_29];
Goto(bb9)
}
bb9 = {
_18.0 = -(-30045_i16);
_25.1 = _29;
_32 = core::ptr::addr_of_mut!(_20);
_2 = (*_9);
_14 = _17 ^ _17;
_25 = (_26.0, _29, _26.2);
(*_9) = _2 >> _26.2;
_25.0 = (-1248061460_i32) as f64;
Goto(bb10)
}
bb10 = {
_10 = !_19.0;
_15 = [_10,_19.0,_10,_19.0];
_7 = _29 as i64;
_6 = [_25.1,_25.1,_29,_25.1,_25.1,_25.1,_29,_25.1];
_18 = (13962_i16,);
_24 = _30 as isize;
_18.0 = (-21106_i16);
_19 = (_10,);
_12 = [_29,_25.1,_29,_25.1,_29,_25.1,_25.1,_29];
_26.0 = _25.0;
(*_32) = [_29,_29,_25.1,_29,_29,_29,_29,_25.1];
_31 = !_14;
_35 = !_17;
_8 = [12_u8,197_u8,224_u8,61_u8,195_u8];
Goto(bb11)
}
bb11 = {
_38.fld0 = !_25.2;
(*_32) = [_29,_29,_25.1,_29,_25.1,_29,_25.1,_29];
_18.0 = (-4726_i16);
_14 = !_35;
_4 = [_25.1,_25.1,_29,_25.1,_25.1,_25.1,_25.1,_25.1];
_38.fld1.fld2.0.0 = _16 as u32;
_4 = [_29,_29,_29,_29,_29,_25.1,_25.1,_29];
_38.fld1.fld2.0.1 = core::ptr::addr_of_mut!(_34);
_27 = _16 - _16;
_38.fld2 = -_16;
_38.fld1.fld1 = _25.1;
_24 = -_38.fld2;
_8 = [115_u8,48_u8,63_u8,96_u8,90_u8];
_36 = _19.0;
_19 = (_36,);
_28 = [_10];
(*_9) = _17 as i64;
_42 = -_16;
(*_9) = _25.0 as i64;
_30 = !10057545173205159675_u64;
_38.fld1.fld1 = _25.1;
_9 = core::ptr::addr_of_mut!(_2);
_26.0 = -_25.0;
(*_9) = _1;
_31 = _14 | _14;
_42 = _24;
_7 = _2;
Goto(bb12)
}
bb12 = {
_26.0 = _25.0;
_38.fld1.fld2.0.0 = 4163048986_u32 + 1474064983_u32;
_32 = core::ptr::addr_of_mut!(_6);
_38.fld1.fld2.0.1 = core::ptr::addr_of_mut!((*_9));
_38.fld1.fld4 = core::ptr::addr_of!(_25.1);
_25.0 = _26.0;
_41 = (_18.0,);
_14 = !_35;
_29 = _38.fld1.fld1;
_25.2 = !_26.2;
(*_9) = _1 | _1;
_7 = (*_9) << _16;
_21 = [48653352626647631694187872586129173041_u128,160627293770263762295837171491990980503_u128,13751870537603600868720543613413566898_u128,68995350687159517321908418988317807196_u128,245847045406094787047508435447497130614_u128];
_25.1 = _29;
_25.2 = _24 as usize;
_36 = !_19.0;
_30 = 14375310863889469871_u64 * 562520010379238232_u64;
_40 = !_17;
_39 = _40 as isize;
_38.fld2 = _39;
_8 = [110_u8,2_u8,218_u8,59_u8,233_u8];
Goto(bb13)
}
bb13 = {
_18 = (_41.0,);
_44.0 = 95_u8 as f64;
_16 = _39 * _39;
_29 = _38.fld1.fld1;
_15 = [_10,_19.0,_36,_19.0];
_37 = Adt45::Variant2 { fld0: _26,fld1: _3 };
_38.fld1.fld2.2 = !_19.0;
_38.fld1.fld5 = 905592012_i32 ^ (-1457188848_i32);
match _18.0 {
0 => bb14,
1 => bb15,
2 => bb16,
340282366920938463463374607431768206730 => bb18,
_ => bb17
}
}
bb14 = {
_26.0 = _25.0;
_38.fld1.fld2.0.0 = 4163048986_u32 + 1474064983_u32;
_32 = core::ptr::addr_of_mut!(_6);
_38.fld1.fld2.0.1 = core::ptr::addr_of_mut!((*_9));
_38.fld1.fld4 = core::ptr::addr_of!(_25.1);
_25.0 = _26.0;
_41 = (_18.0,);
_14 = !_35;
_29 = _38.fld1.fld1;
_25.2 = !_26.2;
(*_9) = _1 | _1;
_7 = (*_9) << _16;
_21 = [48653352626647631694187872586129173041_u128,160627293770263762295837171491990980503_u128,13751870537603600868720543613413566898_u128,68995350687159517321908418988317807196_u128,245847045406094787047508435447497130614_u128];
_25.1 = _29;
_25.2 = _24 as usize;
_36 = !_19.0;
_30 = 14375310863889469871_u64 * 562520010379238232_u64;
_40 = !_17;
_39 = _40 as isize;
_38.fld2 = _39;
_8 = [110_u8,2_u8,218_u8,59_u8,233_u8];
Goto(bb13)
}
bb15 = {
_29 = _25.1;
_25.2 = _26.2 ^ _26.2;
_12 = [_29,_29,_29,_25.1,_25.1,_25.1,_29,_29];
_5 = [_29,_29,_29,_29,_29,_25.1,_25.1,_25.1];
_6 = _3;
_27 = !_16;
Goto(bb6)
}
bb16 = {
_14 = !_17;
_4 = [_25.1,_25.1,_25.1,_25.1,_29,_25.1,_25.1,_29];
_26 = (_25.0, _23, _25.2);
_2 = -_7;
_19 = (_10,);
_26.0 = _25.0 - _25.0;
_2 = _1 >> _1;
Goto(bb8)
}
bb17 = {
_26.1 = _23;
_26.2 = !1_usize;
_25.0 = 1825507542_u32 as f64;
_2 = !_7;
_7 = _2;
_7 = _2 & _2;
_19.0 = !_10;
_5 = [_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1];
_27 = _16 & _16;
_4 = [_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1];
_5 = [_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1];
_9 = core::ptr::addr_of_mut!((*_9));
match _18.0 {
0 => bb1,
340282366920938463463374607431768197354 => bb5,
_ => bb2
}
}
bb18 = {
_26.0 = -_44.0;
_35 = _31;
place!(Field::<(f64, [i8; 5], usize)>(Variant(_37, 2), 0)).1 = [(-123_i8),2_i8,52_i8,(-123_i8),(-72_i8)];
_26.2 = Field::<(f64, [i8; 5], usize)>(Variant(_37, 2), 0).2 * Field::<(f64, [i8; 5], usize)>(Variant(_37, 2), 0).2;
_2 = _7;
_21 = [338163620837845507312759032555783629307_u128,121927151069229168797553358188607019318_u128,44527970226513711101906474601771511464_u128,131761677677623618227107949108147377619_u128,16503158487711277449876602120960526870_u128];
_32 = core::ptr::addr_of_mut!(_6);
_20 = [_29,_25.1,_38.fld1.fld1,_38.fld1.fld1,_38.fld1.fld1,_29,_25.1,_38.fld1.fld1];
_8 = [91_u8,83_u8,138_u8,151_u8,23_u8];
_13 = [_38.fld1.fld2.0.0,_38.fld1.fld2.0.0,_38.fld1.fld2.0.0];
_47 = (-1_i8) as f64;
_26 = Field::<(f64, [i8; 5], usize)>(Variant(_37, 2), 0);
_44.1 = _29;
_38.fld2 = _16;
(*_32) = [_29,_25.1,_25.1,_25.1,_38.fld1.fld1,_29,_44.1,_29];
_13 = [_38.fld1.fld2.0.0,_38.fld1.fld2.0.0,_38.fld1.fld2.0.0];
_49 = _35;
place!(Field::<(f64, [i8; 5], usize)>(Variant(_37, 2), 0)) = (_25.0, _23, _26.2);
_38.fld1.fld3 = _9;
_33 = [71067062414808200785513427873188908820_u128,244741404170955551950533816836279432203_u128,220767195087183760720469112207496872858_u128,243268183871509691428349820721889305581_u128,327721902066529529748532089198868845382_u128];
_53 = _21;
match _41.0 {
0 => bb1,
1 => bb2,
2 => bb12,
3 => bb16,
4 => bb9,
5 => bb6,
6 => bb19,
340282366920938463463374607431768206730 => bb21,
_ => bb20
}
}
bb19 = {
_26.0 = _25.0;
_38.fld1.fld2.0.0 = 4163048986_u32 + 1474064983_u32;
_32 = core::ptr::addr_of_mut!(_6);
_38.fld1.fld2.0.1 = core::ptr::addr_of_mut!((*_9));
_38.fld1.fld4 = core::ptr::addr_of!(_25.1);
_25.0 = _26.0;
_41 = (_18.0,);
_14 = !_35;
_29 = _38.fld1.fld1;
_25.2 = !_26.2;
(*_9) = _1 | _1;
_7 = (*_9) << _16;
_21 = [48653352626647631694187872586129173041_u128,160627293770263762295837171491990980503_u128,13751870537603600868720543613413566898_u128,68995350687159517321908418988317807196_u128,245847045406094787047508435447497130614_u128];
_25.1 = _29;
_25.2 = _24 as usize;
_36 = !_19.0;
_30 = 14375310863889469871_u64 * 562520010379238232_u64;
_40 = !_17;
_39 = _40 as isize;
_38.fld2 = _39;
_8 = [110_u8,2_u8,218_u8,59_u8,233_u8];
Goto(bb13)
}
bb20 = {
_10 = 290753722990666521375190612760731464235_u128 as u16;
_26.0 = -_25.0;
_8 = [105_u8,213_u8,128_u8,71_u8,71_u8];
_27 = _16 ^ _16;
_2 = !(*_9);
_20 = [_25.1,_29,_25.1,_29,_29,_29,_29,_25.1];
_31 = _16 == _16;
_12 = [_29,_29,_25.1,_25.1,_29,_25.1,_25.1,_25.1];
Goto(bb7)
}
bb21 = {
_38.fld1.fld3 = _9;
_4 = _11;
_32 = core::ptr::addr_of_mut!(_5);
_12 = [_29,_38.fld1.fld1,_25.1,_25.1,_44.1,_25.1,_44.1,_29];
_38.fld1.fld0.0 = 71_u8 as u16;
(*_9) = -_7;
_52 = _38.fld1.fld5;
_7 = _38.fld2 as i64;
_44.1 = _29;
_26.1 = [112_i8,(-118_i8),(-1_i8),76_i8,(-73_i8)];
_25 = (_26.0, _29, _38.fld0);
_51 = !Field::<(f64, [i8; 5], usize)>(Variant(_37, 2), 0).2;
_46 = !_31;
_54 = [_30,_30,_30,_30];
_38.fld1.fld6 = _7;
_27 = _31 as isize;
_10 = (-31_i8) as u16;
_41 = (_18.0,);
_41.0 = _18.0;
_3 = [_25.1,_38.fld1.fld1,_38.fld1.fld1,_38.fld1.fld1,_44.1,_25.1,_29,_25.1];
_54 = [_30,_30,_30,_30];
_38.fld0 = _51 + Field::<(f64, [i8; 5], usize)>(Variant(_37, 2), 0).2;
_21 = _33;
_31 = _49;
match _41.0 {
0 => bb13,
340282366920938463463374607431768206730 => bb22,
_ => bb10
}
}
bb22 = {
_11 = [_29,_29,_25.1,_38.fld1.fld1,_25.1,_29,_44.1,_44.1];
_27 = _39 - _38.fld2;
_38.fld0 = !Field::<(f64, [i8; 5], usize)>(Variant(_37, 2), 0).2;
_29 = _25.1;
_8 = [155_u8,54_u8,182_u8,44_u8,56_u8];
_6 = (*_32);
_41 = _18;
_33 = _53;
_56.1 = _38.fld1.fld1;
_44.1 = _29;
_38.fld1.fld2.0.1 = _9;
_36 = _10;
_44.2 = _38.fld0;
_38.fld0 = _51;
_18 = _41;
_53 = _21;
_56.0 = _47 * Field::<(f64, [i8; 5], usize)>(Variant(_37, 2), 0).0;
_9 = _38.fld1.fld3;
place!(Field::<(f64, [i8; 5], usize)>(Variant(_37, 2), 0)) = (_56.0, _26.1, _26.2);
_56 = (_44.0, _44.1, _44.2);
SetDiscriminant(_37, 3);
place!(Field::<usize>(Variant(_37, 3), 6)) = _56.2;
_6 = [_25.1,_38.fld1.fld1,_29,_56.1,_38.fld1.fld1,_38.fld1.fld1,_29,_44.1];
_59 = !_18.0;
_25 = (_56.0, _29, _44.2);
Goto(bb23)
}
bb23 = {
_13 = [_38.fld1.fld2.0.0,_38.fld1.fld2.0.0,_38.fld1.fld2.0.0];
_33 = [52941396044793117388250256226459504414_u128,112306344779569060965701837780851555308_u128,67113982530068534152752045948231547287_u128,269637600713483575777160859421684821443_u128,6267155277098255293151899081153599048_u128];
_53 = [124714146961048297403829727987619125467_u128,9447542457317204909675800640467339186_u128,108150784102887027915781111318195855962_u128,110003356845593897594919372493235142537_u128,9414069557887559566323085016320302954_u128];
_19.0 = !_36;
_41 = (_18.0,);
_38.fld1.fld0 = _19;
_33 = _21;
(*_9) = _7;
_44.2 = _25.2;
_36 = 223_u8 as u16;
_18 = _41;
_12 = [_38.fld1.fld1,_25.1,_29,_56.1,_44.1,_25.1,_25.1,_38.fld1.fld1];
_18.0 = _56.1 as i16;
Call(_38.fld1.fld6 = core::intrinsics::bswap((*_9)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
_2 = (-78_i8) as i64;
_22 = [_1,_38.fld1.fld6,_7];
_61 = Adt51::Variant2 { fld0: _30 };
_25.2 = _38.fld1.fld6 as usize;
place!(Field::<bool>(Variant(_37, 3), 0)) = _27 >= _38.fld2;
_25 = _56;
Goto(bb25)
}
bb25 = {
_48 = [215465186652940539327886479761122612046_u128,180076373473156452305157146459382149528_u128,60766911668947621588342493487825676661_u128,293932191615976853220898979016787676516_u128,33288257545464933388144365738361841011_u128];
_38.fld0 = _25.2 * _25.2;
_51 = _38.fld0;
Goto(bb26)
}
bb26 = {
_38.fld1.fld2.1 = _18.0 << _38.fld2;
_38.fld1.fld0.0 = _36 + _10;
_34 = _38.fld1.fld6;
_2 = _27 as i64;
_45 = 214576367220360269700317900876749292261_u128 as i32;
_66 = _38.fld1.fld2.1 as i128;
_13 = [_38.fld1.fld2.0.0,_38.fld1.fld2.0.0,_38.fld1.fld2.0.0];
_65 = 248_u8;
_66 = (-1404492074776803179662007326109520921_i128);
place!(Field::<usize>(Variant(_37, 3), 6)) = !_25.2;
_25.0 = _26.0 - _56.0;
_13 = [_38.fld1.fld2.0.0,_38.fld1.fld2.0.0,_38.fld1.fld2.0.0];
_26.2 = _51 | _25.2;
_52 = _45;
_24 = _38.fld2 >> _7;
SetDiscriminant(_61, 1);
_18 = _41;
_32 = core::ptr::addr_of_mut!(_4);
_8 = [_65,_65,_65,_65,_65];
_6 = _4;
place!(Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_61, 1), 0)).0.0 = _38.fld1.fld2.0.0;
_57 = _25.1;
_34 = 56_i8 as i64;
_52 = _38.fld1.fld5;
_2 = _7;
_16 = _38.fld2 & _24;
Call(_63.0 = fn14(_26, _30), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
(*_9) = _38.fld1.fld6;
place!(Field::<(i16,)>(Variant(_61, 1), 1)).0 = _38.fld1.fld2.1;
_2 = !_7;
_33 = [267129223031149125381789609916402081171_u128,333557291833946251935214650958737501595_u128,229344877606502247445204682721032434777_u128,117790451628082277329880750506868438548_u128,85435250426880343740042717587836817409_u128];
_26.2 = _66 as usize;
place!(Field::<i16>(Variant(_37, 3), 4)) = Field::<(i16,)>(Variant(_61, 1), 1).0 ^ Field::<(i16,)>(Variant(_61, 1), 1).0;
_29 = _57;
_54 = [_30,_30,_30,_30];
_27 = _24;
_25.2 = _30 as usize;
(*_9) = _7 + _38.fld1.fld6;
_26.0 = -_47;
_19.0 = (*_9) as u16;
_19.0 = _38.fld1.fld0.0 | _38.fld1.fld2.2;
_65 = _51 as u8;
_13 = [_63.0,Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_61, 1), 0).0.0,_38.fld1.fld2.0.0];
place!(Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_61, 1), 0)).1 = -_38.fld1.fld2.1;
_24 = _49 as isize;
_70.0 = _27 as f64;
place!(Field::<*mut i128>(Variant(_37, 3), 2)) = core::ptr::addr_of_mut!(_66);
match _41.0 {
0 => bb15,
1 => bb2,
2 => bb28,
340282366920938463463374607431768206730 => bb30,
_ => bb29
}
}
bb28 = {
_10 = 290753722990666521375190612760731464235_u128 as u16;
_26.0 = -_25.0;
_8 = [105_u8,213_u8,128_u8,71_u8,71_u8];
_27 = _16 ^ _16;
_2 = !(*_9);
_20 = [_25.1,_29,_25.1,_29,_29,_29,_29,_25.1];
_31 = _16 == _16;
_12 = [_29,_29,_25.1,_25.1,_29,_25.1,_25.1,_25.1];
Goto(bb7)
}
bb29 = {
_26.1 = _23;
_26.2 = !1_usize;
_25.0 = 1825507542_u32 as f64;
_2 = !_7;
_7 = _2;
_7 = _2 & _2;
_19.0 = !_10;
_5 = [_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1];
_27 = _16 & _16;
_4 = [_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1];
_5 = [_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1,_25.1];
_9 = core::ptr::addr_of_mut!((*_9));
match _18.0 {
0 => bb1,
340282366920938463463374607431768197354 => bb5,
_ => bb2
}
}
bb30 = {
_2 = _38.fld1.fld6;
_58 = _21;
_20 = (*_32);
_1 = _7 >> _38.fld1.fld6;
_63.0 = !_38.fld1.fld2.0.0;
_38.fld1.fld2.0.0 = !Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_61, 1), 0).0.0;
place!(Field::<bool>(Variant(_37, 3), 0)) = _46;
_23 = [(-21_i8),(-114_i8),61_i8,109_i8,32_i8];
_51 = _38.fld0;
_77 = _49 <= _14;
_13 = [_38.fld1.fld2.0.0,Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_61, 1), 0).0.0,Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_61, 1), 0).0.0];
_38.fld2 = _31 as isize;
_34 = !_38.fld1.fld6;
(*_9) = _34;
(*_9) = _7 >> _38.fld2;
_38.fld1.fld2.0.1 = _38.fld1.fld3;
_70.2 = _56.2;
_76 = _13;
_1 = (-128_i8) as i64;
_58 = [184800207197018429333529979293378373901_u128,82196797533510761302891979724373655336_u128,322898123435297521497499761366448617620_u128,24493960948033559676691310580821586500_u128,32976138458156454260229141877836977014_u128];
_76 = _13;
_40 = Field::<bool>(Variant(_37, 3), 0);
_12 = _20;
_77 = _40 <= _40;
place!(Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_61, 1), 0)).0.1 = _38.fld1.fld2.0.1;
Call(_80 = core::intrinsics::transmute(_38.fld2), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
_74 = _25.1;
_26.1 = [(-62_i8),7_i8,126_i8,24_i8,(-101_i8)];
_61 = Adt51::Variant2 { fld0: _30 };
_70.1 = _38.fld1.fld1;
_38.fld1.fld6 = -(*_9);
_56.2 = !_51;
_4 = [_74,_44.1,_38.fld1.fld1,_57,_56.1,_25.1,_25.1,_57];
_18.0 = _38.fld1.fld2.1;
_75 = Field::<u64>(Variant(_61, 2), 0) as f32;
_50 = _70.0;
_84 = [Field::<u64>(Variant(_61, 2), 0),Field::<u64>(Variant(_61, 2), 0),_30,Field::<u64>(Variant(_61, 2), 0)];
_82 = -_16;
_38.fld1.fld6 = _66 as i64;
place!(Field::<i16>(Variant(_37, 3), 4)) = _66 as i16;
_36 = _70.0 as u16;
_45 = _65 as i32;
_64 = !_46;
_67 = _74;
_68 = [(-44_i8),(-39_i8),47_i8,(-8_i8),101_i8];
_81.0 = !_36;
_38.fld0 = _51;
match _41.0 {
0 => bb32,
1 => bb33,
2 => bb34,
3 => bb35,
340282366920938463463374607431768206730 => bb37,
_ => bb36
}
}
bb32 = {
_10 = 290753722990666521375190612760731464235_u128 as u16;
_26.0 = -_25.0;
_8 = [105_u8,213_u8,128_u8,71_u8,71_u8];
_27 = _16 ^ _16;
_2 = !(*_9);
_20 = [_25.1,_29,_25.1,_29,_29,_29,_29,_25.1];
_31 = _16 == _16;
_12 = [_29,_29,_25.1,_25.1,_29,_25.1,_25.1,_25.1];
Goto(bb7)
}
bb33 = {
_10 = !_19.0;
_15 = [_10,_19.0,_10,_19.0];
_7 = _29 as i64;
_6 = [_25.1,_25.1,_29,_25.1,_25.1,_25.1,_29,_25.1];
_18 = (13962_i16,);
_24 = _30 as isize;
_18.0 = (-21106_i16);
_19 = (_10,);
_12 = [_29,_25.1,_29,_25.1,_29,_25.1,_25.1,_29];
_26.0 = _25.0;
(*_32) = [_29,_29,_25.1,_29,_29,_29,_29,_25.1];
_31 = !_14;
_35 = !_17;
_8 = [12_u8,197_u8,224_u8,61_u8,195_u8];
Goto(bb11)
}
bb34 = {
_38.fld1.fld3 = _9;
_4 = _11;
_32 = core::ptr::addr_of_mut!(_5);
_12 = [_29,_38.fld1.fld1,_25.1,_25.1,_44.1,_25.1,_44.1,_29];
_38.fld1.fld0.0 = 71_u8 as u16;
(*_9) = -_7;
_52 = _38.fld1.fld5;
_7 = _38.fld2 as i64;
_44.1 = _29;
_26.1 = [112_i8,(-118_i8),(-1_i8),76_i8,(-73_i8)];
_25 = (_26.0, _29, _38.fld0);
_51 = !Field::<(f64, [i8; 5], usize)>(Variant(_37, 2), 0).2;
_46 = !_31;
_54 = [_30,_30,_30,_30];
_38.fld1.fld6 = _7;
_27 = _31 as isize;
_10 = (-31_i8) as u16;
_41 = (_18.0,);
_41.0 = _18.0;
_3 = [_25.1,_38.fld1.fld1,_38.fld1.fld1,_38.fld1.fld1,_44.1,_25.1,_29,_25.1];
_54 = [_30,_30,_30,_30];
_38.fld0 = _51 + Field::<(f64, [i8; 5], usize)>(Variant(_37, 2), 0).2;
_21 = _33;
_31 = _49;
match _41.0 {
0 => bb13,
340282366920938463463374607431768206730 => bb22,
_ => bb10
}
}
bb35 = {
(*_9) = _38.fld1.fld6;
place!(Field::<(i16,)>(Variant(_61, 1), 1)).0 = _38.fld1.fld2.1;
_2 = !_7;
_33 = [267129223031149125381789609916402081171_u128,333557291833946251935214650958737501595_u128,229344877606502247445204682721032434777_u128,117790451628082277329880750506868438548_u128,85435250426880343740042717587836817409_u128];
_26.2 = _66 as usize;
place!(Field::<i16>(Variant(_37, 3), 4)) = Field::<(i16,)>(Variant(_61, 1), 1).0 ^ Field::<(i16,)>(Variant(_61, 1), 1).0;
_29 = _57;
_54 = [_30,_30,_30,_30];
_27 = _24;
_25.2 = _30 as usize;
(*_9) = _7 + _38.fld1.fld6;
_26.0 = -_47;
_19.0 = (*_9) as u16;
_19.0 = _38.fld1.fld0.0 | _38.fld1.fld2.2;
_65 = _51 as u8;
_13 = [_63.0,Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_61, 1), 0).0.0,_38.fld1.fld2.0.0];
place!(Field::<((u32, *mut i64, *const u128), i16, u16)>(Variant(_61, 1), 0)).1 = -_38.fld1.fld2.1;
_24 = _49 as isize;
_70.0 = _27 as f64;
place!(Field::<*mut i128>(Variant(_37, 3), 2)) = core::ptr::addr_of_mut!(_66);
match _41.0 {
0 => bb15,
1 => bb2,
2 => bb28,
340282366920938463463374607431768206730 => bb30,
_ => bb29
}
}
bb36 = {
_13 = [_38.fld1.fld2.0.0,_38.fld1.fld2.0.0,_38.fld1.fld2.0.0];
_33 = [52941396044793117388250256226459504414_u128,112306344779569060965701837780851555308_u128,67113982530068534152752045948231547287_u128,269637600713483575777160859421684821443_u128,6267155277098255293151899081153599048_u128];
_53 = [124714146961048297403829727987619125467_u128,9447542457317204909675800640467339186_u128,108150784102887027915781111318195855962_u128,110003356845593897594919372493235142537_u128,9414069557887559566323085016320302954_u128];
_19.0 = !_36;
_41 = (_18.0,);
_38.fld1.fld0 = _19;
_33 = _21;
(*_9) = _7;
_44.2 = _25.2;
_36 = 223_u8 as u16;
_18 = _41;
_12 = [_38.fld1.fld1,_25.1,_29,_56.1,_44.1,_25.1,_25.1,_38.fld1.fld1];
_18.0 = _56.1 as i16;
Call(_38.fld1.fld6 = core::intrinsics::bswap((*_9)), ReturnTo(bb24), UnwindUnreachable())
}
bb37 = {
_52 = _63.0 as i32;
_70 = (_50, _74, _56.2);
_42 = _38.fld2;
_63.1 = _38.fld1.fld3;
_69 = _15;
_55 = _75 - _75;
_22 = [_7,(*_9),(*_9)];
_38.fld1.fld0.0 = _36;
_56.1 = _70.1;
_12 = [_74,_38.fld1.fld1,_57,_25.1,_67,_57,_70.1,_56.1];
_20 = (*_32);
_56.0 = _70.0;
Goto(bb38)
}
bb38 = {
_83 = Adt43::Variant0 { fld0: _26.1,fld1: 243672737338762886309421950062049239334_u128 };
(*_9) = _65 as i64;
_4 = [_56.1,_56.1,_74,_29,_74,_25.1,_67,_70.1];
Call(_39 = fn15(_56, _77, _81.0), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
_25 = _70;
_28 = [_38.fld1.fld0.0];
SetDiscriminant(_61, 2);
_63.1 = _38.fld1.fld3;
_81 = (_38.fld1.fld0.0,);
_38.fld1.fld3 = core::ptr::addr_of_mut!((*_9));
_72 = !(-55_i8);
_44 = (_50, _56.1, _38.fld0);
place!(Field::<bool>(Variant(_37, 3), 0)) = _46;
_60 = _45;
_72 = _34 as i8;
_38.fld1.fld2.0.0 = !_63.0;
place!(Field::<usize>(Variant(_37, 3), 6)) = _25.2;
_50 = _70.0;
_38.fld1.fld2.1 = _38.fld1.fld2.0.0 as i16;
_4 = [_74,_67,_38.fld1.fld1,_38.fld1.fld1,_56.1,_44.1,_44.1,_38.fld1.fld1];
place!(Field::<*mut [char; 8]>(Variant(_37, 3), 5)) = core::ptr::addr_of_mut!(_11);
place!(Field::<u128>(Variant(_83, 0), 1)) = 313577438993548211220105935819025767496_u128 << _42;
_41.0 = _18.0;
_82 = _27 - _24;
SetDiscriminant(_83, 0);
_58 = [59136696242822628988301026183858714091_u128,328893034901787949751836029289207823187_u128,151073543406329822526601083796028812086_u128,294929412557364495253718368731360315195_u128,33054378567694521164118165998402777478_u128];
_38.fld1.fld2.0.0 = _63.0;
Goto(bb40)
}
bb40 = {
_2 = _56.1 as i64;
_68 = _26.1;
_70.0 = _25.2 as f64;
_56.1 = _38.fld1.fld1;
_11 = [_29,_56.1,_29,_38.fld1.fld1,_44.1,_25.1,_70.1,_70.1];
Goto(bb41)
}
bb41 = {
_18 = (_41.0,);
_18 = _41;
_70 = (_50, _38.fld1.fld1, Field::<usize>(Variant(_37, 3), 6));
_50 = -_56.0;
_74 = _57;
_45 = -_60;
_33 = _53;
_86 = _22;
_38.fld1.fld6 = _7;
_12 = [_74,_67,_70.1,_25.1,_57,_67,_70.1,_57];
RET = core::ptr::addr_of!(place!(Field::<u128>(Variant(_83, 0), 1)));
Goto(bb42)
}
bb42 = {
Call(_96 = dump_var(13_usize, 86_usize, Move(_86), 11_usize, Move(_11), 16_usize, Move(_16), 5_usize, Move(_5)), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
Call(_96 = dump_var(13_usize, 77_usize, Move(_77), 57_usize, Move(_57), 65_usize, Move(_65), 18_usize, Move(_18)), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Call(_96 = dump_var(13_usize, 6_usize, Move(_6), 48_usize, Move(_48), 34_usize, Move(_34), 60_usize, Move(_60)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
Call(_96 = dump_var(13_usize, 82_usize, Move(_82), 28_usize, Move(_28), 58_usize, Move(_58), 31_usize, Move(_31)), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
Call(_96 = dump_var(13_usize, 42_usize, Move(_42), 49_usize, Move(_49), 27_usize, Move(_27), 68_usize, Move(_68)), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
Call(_96 = dump_var(13_usize, 45_usize, Move(_45), 39_usize, Move(_39), 59_usize, Move(_59), 66_usize, Move(_66)), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
Call(_96 = dump_var(13_usize, 81_usize, Move(_81), 1_usize, Move(_1), 19_usize, Move(_19), 52_usize, Move(_52)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Call(_96 = dump_var(13_usize, 41_usize, Move(_41), 2_usize, Move(_2), 24_usize, Move(_24), 97_usize, _97), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: (f64, [i8; 5], usize),mut _2: u64) -> u32 {
mir! {
type RET = u32;
let _3: bool;
let _4: f32;
let _5: Adt53;
let _6: [i64; 3];
let _7: usize;
let _8: [u16; 1];
let _9: f64;
let _10: Adt57;
let _11: f64;
let _12: char;
let _13: isize;
let _14: i16;
let _15: isize;
let _16: ();
let _17: ();
{
RET = !1622903921_u32;
RET = 109479858_u32;
_1.1 = [(-94_i8),8_i8,5_i8,87_i8,25_i8];
_2 = 11351528232630756608_u64;
_1.0 = 54_i8 as f64;
RET = 1251416122_u32;
RET = !3702266199_u32;
_2 = 86034877283958113_u64;
RET = 3791173803_u32;
_1.1 = [31_i8,(-126_i8),(-55_i8),49_i8,(-114_i8)];
RET = 316643153_u32;
_1.2 = 15141935998998626257_usize;
RET = 1600348029_u32;
_3 = _1.0 < _1.0;
match _1.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
15141935998998626257 => bb7,
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
_6 = [1479560501666340520_i64,(-8821330590422636556_i64),7982049398442720439_i64];
_1.0 = 60390_u16 as f64;
_2 = 77_u8 as u64;
_3 = _1.0 >= _1.0;
RET = 1699550545_u32;
_1.1 = [(-85_i8),25_i8,119_i8,(-11_i8),82_i8];
_1.2 = 8881563047567773794_usize;
_1.0 = (-6584769927898847332_i64) as f64;
_7 = _1.2;
match RET {
0 => bb6,
1 => bb5,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
1699550545 => bb13,
_ => bb12
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
Return()
}
bb13 = {
_10.fld0 = (-1004797916_i32);
RET = 3241810233_u32;
_9 = -_1.0;
_9 = _1.0;
_4 = _10.fld0 as f32;
RET = !3099723009_u32;
match _7 {
0 => bb1,
1 => bb4,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
8881563047567773794 => bb19,
_ => bb18
}
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
Return()
}
bb18 = {
Return()
}
bb19 = {
_11 = 193_u8 as f64;
RET = 2453029066_u32 << _2;
_10.fld1 = [49097_u16,62061_u16,5003_u16,7163_u16];
_6 = [8729915716910317883_i64,(-5131594042660992582_i64),6194698929024552567_i64];
_1.1 = [110_i8,(-124_i8),(-100_i8),58_i8,(-126_i8)];
_6 = [2886152581810972790_i64,(-84365447806853004_i64),3281196774209046453_i64];
_2 = RET as u64;
_1.2 = _10.fld0 as usize;
_4 = _2 as f32;
RET = !1496316397_u32;
_10.fld1 = [57303_u16,25732_u16,22232_u16,35306_u16];
_15 = 46752_u16 as isize;
_15 = _4 as isize;
_9 = _1.0 - _1.0;
RET = !34087364_u32;
_8 = [60841_u16];
_11 = _9 - _9;
_10.fld0 = 447575964_i32;
RET = !455377508_u32;
_15 = (-9223372036854775808_isize);
Goto(bb20)
}
bb20 = {
Call(_16 = dump_var(14_usize, 3_usize, Move(_3), 6_usize, Move(_6), 2_usize, Move(_2), 17_usize, _17), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: (f64, char, usize),mut _2: bool,mut _3: u16) -> isize {
mir! {
type RET = isize;
let _4: [u128; 5];
let _5: [u128; 5];
let _6: bool;
let _7: f64;
let _8: [u8; 5];
let _9: u16;
let _10: f64;
let _11: (f64, [i8; 5], usize);
let _12: [u128; 5];
let _13: i8;
let _14: (f64, char, usize);
let _15: (f64, char, usize);
let _16: *const *const u16;
let _17: [u128; 5];
let _18: Adt57;
let _19: i64;
let _20: isize;
let _21: &'static char;
let _22: char;
let _23: isize;
let _24: ();
let _25: ();
{
_2 = _3 > _3;
RET = _1.2 as isize;
_1.2 = (-4637824610725998411_i64) as usize;
_2 = RET > RET;
_3 = 22282_u16;
_2 = !true;
_1.2 = 0_usize ^ 2_usize;
_1.1 = '\u{57bfb}';
_6 = !_2;
_1.2 = 6_usize & 3_usize;
_1.2 = 11856489097544136694_usize >> _3;
_3 = 43865_u16;
_6 = _1.0 > _1.0;
_4 = [318311948891737929717857494287029736762_u128,52434202567746875924526655353672864252_u128,57613188420314095474590027562876988958_u128,275304835196756143256818871596139514773_u128,205706767254848138083190534345420140411_u128];
Call(_5 = core::intrinsics::transmute(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = [240686558710085789908275813765301600384_u128,10473858087342717349702596783257322602_u128,171241893054041013502641911326386409664_u128,321494334445653171857558955243022617276_u128,32563255745457911204971020909651910698_u128];
_4 = [220632424641863980975887021729267429378_u128,281255802975859218028204864091200566979_u128,298532752793353361686399726803224869179_u128,78838069651576776853025392015468581601_u128,182794248553503310930808274685083411954_u128];
_6 = !_2;
RET = (-112_isize) & 119_isize;
_1.1 = '\u{a1dca}';
_1.2 = 6135578590832444596_usize;
_1.0 = 2039179986_u32 as f64;
_6 = _2;
_7 = (-4144114353309784937_i64) as f64;
_7 = _1.0 - _1.0;
RET = -(-9223372036854775808_isize);
_3 = 30862_u16 & 59204_u16;
_4 = _5;
RET = 9223372036854775807_isize;
_8 = [251_u8,223_u8,188_u8,232_u8,247_u8];
_6 = !_2;
RET = _1.1 as isize;
RET = _1.1 as isize;
_3 = !14671_u16;
_1.0 = -_7;
_8 = [177_u8,165_u8,44_u8,180_u8,26_u8];
_1.0 = _7;
_6 = _2;
RET = (-9223372036854775808_isize);
_1.2 = 0_usize & 4_usize;
_7 = _1.0 * _1.0;
_6 = _2;
_1.2 = 65219090714985027074148817088815427296_u128 as usize;
_4 = _5;
RET = 121_isize;
Goto(bb2)
}
bb2 = {
RET = !104_isize;
_3 = _1.2 as u16;
_8 = [53_u8,193_u8,202_u8,169_u8,212_u8];
_2 = RET < RET;
RET = !(-9223372036854775808_isize);
_1.0 = -_7;
_8 = [78_u8,45_u8,110_u8,154_u8,195_u8];
RET = (-87_isize);
_3 = !38099_u16;
_3 = (-77597188328793634503665734364594671755_i128) as u16;
_4 = [298750347942581314991165060455171423918_u128,134349514634842079784493831611534199708_u128,253293186718646129693769964157586487399_u128,114379229018981977390257723920252895518_u128,245436422912325922433445359076153646796_u128];
_4 = _5;
_1.1 = '\u{77c72}';
_1 = (_7, '\u{d4da9}', 8378781806501672430_usize);
_7 = _1.0 * _1.0;
_1.2 = !7_usize;
_4 = [187370423992339075778532286173101766638_u128,150887931431948374037123913662903133749_u128,255256172024073741202339359062252820121_u128,162921992669848621686117850797217748606_u128,122521178465253940301436968654732228222_u128];
_3 = RET as u16;
_1.0 = _7 + _7;
_7 = _1.0 * _1.0;
_1 = (_7, '\u{49738}', 8130180133640739226_usize);
match _1.2 {
0 => bb3,
8130180133640739226 => bb5,
_ => bb4
}
}
bb3 = {
_5 = [240686558710085789908275813765301600384_u128,10473858087342717349702596783257322602_u128,171241893054041013502641911326386409664_u128,321494334445653171857558955243022617276_u128,32563255745457911204971020909651910698_u128];
_4 = [220632424641863980975887021729267429378_u128,281255802975859218028204864091200566979_u128,298532752793353361686399726803224869179_u128,78838069651576776853025392015468581601_u128,182794248553503310930808274685083411954_u128];
_6 = !_2;
RET = (-112_isize) & 119_isize;
_1.1 = '\u{a1dca}';
_1.2 = 6135578590832444596_usize;
_1.0 = 2039179986_u32 as f64;
_6 = _2;
_7 = (-4144114353309784937_i64) as f64;
_7 = _1.0 - _1.0;
RET = -(-9223372036854775808_isize);
_3 = 30862_u16 & 59204_u16;
_4 = _5;
RET = 9223372036854775807_isize;
_8 = [251_u8,223_u8,188_u8,232_u8,247_u8];
_6 = !_2;
RET = _1.1 as isize;
RET = _1.1 as isize;
_3 = !14671_u16;
_1.0 = -_7;
_8 = [177_u8,165_u8,44_u8,180_u8,26_u8];
_1.0 = _7;
_6 = _2;
RET = (-9223372036854775808_isize);
_1.2 = 0_usize & 4_usize;
_7 = _1.0 * _1.0;
_6 = _2;
_1.2 = 65219090714985027074148817088815427296_u128 as usize;
_4 = _5;
RET = 121_isize;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
_11.2 = !_1.2;
_10 = _7;
_1.0 = _10;
_8 = [159_u8,9_u8,73_u8,236_u8,251_u8];
_14.0 = 6008932523836144027_u64 as f64;
_11.2 = (-2095170668_i32) as usize;
match _1.2 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb7,
8130180133640739226 => bb9,
_ => bb8
}
}
bb6 = {
_5 = [240686558710085789908275813765301600384_u128,10473858087342717349702596783257322602_u128,171241893054041013502641911326386409664_u128,321494334445653171857558955243022617276_u128,32563255745457911204971020909651910698_u128];
_4 = [220632424641863980975887021729267429378_u128,281255802975859218028204864091200566979_u128,298532752793353361686399726803224869179_u128,78838069651576776853025392015468581601_u128,182794248553503310930808274685083411954_u128];
_6 = !_2;
RET = (-112_isize) & 119_isize;
_1.1 = '\u{a1dca}';
_1.2 = 6135578590832444596_usize;
_1.0 = 2039179986_u32 as f64;
_6 = _2;
_7 = (-4144114353309784937_i64) as f64;
_7 = _1.0 - _1.0;
RET = -(-9223372036854775808_isize);
_3 = 30862_u16 & 59204_u16;
_4 = _5;
RET = 9223372036854775807_isize;
_8 = [251_u8,223_u8,188_u8,232_u8,247_u8];
_6 = !_2;
RET = _1.1 as isize;
RET = _1.1 as isize;
_3 = !14671_u16;
_1.0 = -_7;
_8 = [177_u8,165_u8,44_u8,180_u8,26_u8];
_1.0 = _7;
_6 = _2;
RET = (-9223372036854775808_isize);
_1.2 = 0_usize & 4_usize;
_7 = _1.0 * _1.0;
_6 = _2;
_1.2 = 65219090714985027074148817088815427296_u128 as usize;
_4 = _5;
RET = 121_isize;
Goto(bb2)
}
bb7 = {
_5 = [240686558710085789908275813765301600384_u128,10473858087342717349702596783257322602_u128,171241893054041013502641911326386409664_u128,321494334445653171857558955243022617276_u128,32563255745457911204971020909651910698_u128];
_4 = [220632424641863980975887021729267429378_u128,281255802975859218028204864091200566979_u128,298532752793353361686399726803224869179_u128,78838069651576776853025392015468581601_u128,182794248553503310930808274685083411954_u128];
_6 = !_2;
RET = (-112_isize) & 119_isize;
_1.1 = '\u{a1dca}';
_1.2 = 6135578590832444596_usize;
_1.0 = 2039179986_u32 as f64;
_6 = _2;
_7 = (-4144114353309784937_i64) as f64;
_7 = _1.0 - _1.0;
RET = -(-9223372036854775808_isize);
_3 = 30862_u16 & 59204_u16;
_4 = _5;
RET = 9223372036854775807_isize;
_8 = [251_u8,223_u8,188_u8,232_u8,247_u8];
_6 = !_2;
RET = _1.1 as isize;
RET = _1.1 as isize;
_3 = !14671_u16;
_1.0 = -_7;
_8 = [177_u8,165_u8,44_u8,180_u8,26_u8];
_1.0 = _7;
_6 = _2;
RET = (-9223372036854775808_isize);
_1.2 = 0_usize & 4_usize;
_7 = _1.0 * _1.0;
_6 = _2;
_1.2 = 65219090714985027074148817088815427296_u128 as usize;
_4 = _5;
RET = 121_isize;
Goto(bb2)
}
bb8 = {
RET = !104_isize;
_3 = _1.2 as u16;
_8 = [53_u8,193_u8,202_u8,169_u8,212_u8];
_2 = RET < RET;
RET = !(-9223372036854775808_isize);
_1.0 = -_7;
_8 = [78_u8,45_u8,110_u8,154_u8,195_u8];
RET = (-87_isize);
_3 = !38099_u16;
_3 = (-77597188328793634503665734364594671755_i128) as u16;
_4 = [298750347942581314991165060455171423918_u128,134349514634842079784493831611534199708_u128,253293186718646129693769964157586487399_u128,114379229018981977390257723920252895518_u128,245436422912325922433445359076153646796_u128];
_4 = _5;
_1.1 = '\u{77c72}';
_1 = (_7, '\u{d4da9}', 8378781806501672430_usize);
_7 = _1.0 * _1.0;
_1.2 = !7_usize;
_4 = [187370423992339075778532286173101766638_u128,150887931431948374037123913662903133749_u128,255256172024073741202339359062252820121_u128,162921992669848621686117850797217748606_u128,122521178465253940301436968654732228222_u128];
_3 = RET as u16;
_1.0 = _7 + _7;
_7 = _1.0 * _1.0;
_1 = (_7, '\u{49738}', 8130180133640739226_usize);
match _1.2 {
0 => bb3,
8130180133640739226 => bb5,
_ => bb4
}
}
bb9 = {
_14.1 = _1.1;
_1.0 = (-35781019325797104435182460890971108083_i128) as f64;
_6 = _1.2 != _1.2;
_15.1 = _1.1;
match _1.2 {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
8130180133640739226 => bb15,
_ => bb14
}
}
bb10 = {
RET = !104_isize;
_3 = _1.2 as u16;
_8 = [53_u8,193_u8,202_u8,169_u8,212_u8];
_2 = RET < RET;
RET = !(-9223372036854775808_isize);
_1.0 = -_7;
_8 = [78_u8,45_u8,110_u8,154_u8,195_u8];
RET = (-87_isize);
_3 = !38099_u16;
_3 = (-77597188328793634503665734364594671755_i128) as u16;
_4 = [298750347942581314991165060455171423918_u128,134349514634842079784493831611534199708_u128,253293186718646129693769964157586487399_u128,114379229018981977390257723920252895518_u128,245436422912325922433445359076153646796_u128];
_4 = _5;
_1.1 = '\u{77c72}';
_1 = (_7, '\u{d4da9}', 8378781806501672430_usize);
_7 = _1.0 * _1.0;
_1.2 = !7_usize;
_4 = [187370423992339075778532286173101766638_u128,150887931431948374037123913662903133749_u128,255256172024073741202339359062252820121_u128,162921992669848621686117850797217748606_u128,122521178465253940301436968654732228222_u128];
_3 = RET as u16;
_1.0 = _7 + _7;
_7 = _1.0 * _1.0;
_1 = (_7, '\u{49738}', 8130180133640739226_usize);
match _1.2 {
0 => bb3,
8130180133640739226 => bb5,
_ => bb4
}
}
bb11 = {
_5 = [240686558710085789908275813765301600384_u128,10473858087342717349702596783257322602_u128,171241893054041013502641911326386409664_u128,321494334445653171857558955243022617276_u128,32563255745457911204971020909651910698_u128];
_4 = [220632424641863980975887021729267429378_u128,281255802975859218028204864091200566979_u128,298532752793353361686399726803224869179_u128,78838069651576776853025392015468581601_u128,182794248553503310930808274685083411954_u128];
_6 = !_2;
RET = (-112_isize) & 119_isize;
_1.1 = '\u{a1dca}';
_1.2 = 6135578590832444596_usize;
_1.0 = 2039179986_u32 as f64;
_6 = _2;
_7 = (-4144114353309784937_i64) as f64;
_7 = _1.0 - _1.0;
RET = -(-9223372036854775808_isize);
_3 = 30862_u16 & 59204_u16;
_4 = _5;
RET = 9223372036854775807_isize;
_8 = [251_u8,223_u8,188_u8,232_u8,247_u8];
_6 = !_2;
RET = _1.1 as isize;
RET = _1.1 as isize;
_3 = !14671_u16;
_1.0 = -_7;
_8 = [177_u8,165_u8,44_u8,180_u8,26_u8];
_1.0 = _7;
_6 = _2;
RET = (-9223372036854775808_isize);
_1.2 = 0_usize & 4_usize;
_7 = _1.0 * _1.0;
_6 = _2;
_1.2 = 65219090714985027074148817088815427296_u128 as usize;
_4 = _5;
RET = 121_isize;
Goto(bb2)
}
bb12 = {
_5 = [240686558710085789908275813765301600384_u128,10473858087342717349702596783257322602_u128,171241893054041013502641911326386409664_u128,321494334445653171857558955243022617276_u128,32563255745457911204971020909651910698_u128];
_4 = [220632424641863980975887021729267429378_u128,281255802975859218028204864091200566979_u128,298532752793353361686399726803224869179_u128,78838069651576776853025392015468581601_u128,182794248553503310930808274685083411954_u128];
_6 = !_2;
RET = (-112_isize) & 119_isize;
_1.1 = '\u{a1dca}';
_1.2 = 6135578590832444596_usize;
_1.0 = 2039179986_u32 as f64;
_6 = _2;
_7 = (-4144114353309784937_i64) as f64;
_7 = _1.0 - _1.0;
RET = -(-9223372036854775808_isize);
_3 = 30862_u16 & 59204_u16;
_4 = _5;
RET = 9223372036854775807_isize;
_8 = [251_u8,223_u8,188_u8,232_u8,247_u8];
_6 = !_2;
RET = _1.1 as isize;
RET = _1.1 as isize;
_3 = !14671_u16;
_1.0 = -_7;
_8 = [177_u8,165_u8,44_u8,180_u8,26_u8];
_1.0 = _7;
_6 = _2;
RET = (-9223372036854775808_isize);
_1.2 = 0_usize & 4_usize;
_7 = _1.0 * _1.0;
_6 = _2;
_1.2 = 65219090714985027074148817088815427296_u128 as usize;
_4 = _5;
RET = 121_isize;
Goto(bb2)
}
bb13 = {
RET = !104_isize;
_3 = _1.2 as u16;
_8 = [53_u8,193_u8,202_u8,169_u8,212_u8];
_2 = RET < RET;
RET = !(-9223372036854775808_isize);
_1.0 = -_7;
_8 = [78_u8,45_u8,110_u8,154_u8,195_u8];
RET = (-87_isize);
_3 = !38099_u16;
_3 = (-77597188328793634503665734364594671755_i128) as u16;
_4 = [298750347942581314991165060455171423918_u128,134349514634842079784493831611534199708_u128,253293186718646129693769964157586487399_u128,114379229018981977390257723920252895518_u128,245436422912325922433445359076153646796_u128];
_4 = _5;
_1.1 = '\u{77c72}';
_1 = (_7, '\u{d4da9}', 8378781806501672430_usize);
_7 = _1.0 * _1.0;
_1.2 = !7_usize;
_4 = [187370423992339075778532286173101766638_u128,150887931431948374037123913662903133749_u128,255256172024073741202339359062252820121_u128,162921992669848621686117850797217748606_u128,122521178465253940301436968654732228222_u128];
_3 = RET as u16;
_1.0 = _7 + _7;
_7 = _1.0 * _1.0;
_1 = (_7, '\u{49738}', 8130180133640739226_usize);
match _1.2 {
0 => bb3,
8130180133640739226 => bb5,
_ => bb4
}
}
bb14 = {
Return()
}
bb15 = {
_8 = [2_u8,199_u8,212_u8,121_u8,39_u8];
_17 = [202324904413330419143709197202094800155_u128,205533799009603848396443476271455928227_u128,184169212723540311209794120640722570481_u128,321456700406694866966855459720054284377_u128,85139666457898350989702397714935431604_u128];
RET = _1.2 as isize;
_1.2 = !_11.2;
_7 = _10 - _10;
_11.1 = [46_i8,(-116_i8),(-116_i8),102_i8,(-85_i8)];
_12 = [61369851018031714524583770305538135128_u128,323491572321473032786941986825218541266_u128,227977579793784483320209554041612309544_u128,166252915692009787251280179663685040706_u128,124730395059564586484676919704410093736_u128];
RET = _6 as isize;
_1.0 = 264376932_u32 as f64;
_2 = _6 ^ _6;
_10 = (-98_i8) as f64;
_1.0 = _7;
_1.2 = _11.2;
_11.1 = [(-1_i8),(-15_i8),2_i8,(-80_i8),33_i8];
_18.fld0 = 164181874760602980902624904138292854614_i128 as i32;
_18.fld0 = (-1134353882_i32);
_14.0 = _7;
_14 = (_7, _15.1, _1.2);
_15.1 = _1.1;
_4 = _12;
_22 = _14.1;
RET = (-9223372036854775808_isize);
_13 = (-46_i8);
_5 = [272623904176959673632236711963875472433_u128,117698987940231561022986208231042313580_u128,291474948056233110572841495958123761188_u128,173052446457589540998085884141468720104_u128,337067170227155477135814056608575504152_u128];
Goto(bb16)
}
bb16 = {
Call(_24 = dump_var(15_usize, 13_usize, Move(_13), 5_usize, Move(_5), 2_usize, Move(_2), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_24 = dump_var(15_usize, 8_usize, Move(_8), 25_usize, _25, 25_usize, _25, 25_usize, _25), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: f64,mut _2: i16,mut _3: *const u128,mut _4: [i8; 5],mut _5: *const u128,mut _6: [char; 8]) -> char {
mir! {
type RET = char;
let _7: isize;
let _8: u128;
let _9: *const char;
let _10: *const char;
let _11: isize;
let _12: [u16; 4];
let _13: u16;
let _14: u16;
let _15: [u32; 3];
let _16: u64;
let _17: Adt51;
let _18: isize;
let _19: bool;
let _20: Adt45;
let _21: f64;
let _22: char;
let _23: f64;
let _24: i16;
let _25: u8;
let _26: (f64, char, usize);
let _27: Adt51;
let _28: [u64; 4];
let _29: Adt47;
let _30: isize;
let _31: isize;
let _32: *mut (*mut i128, &'static char, i64);
let _33: i8;
let _34: ();
let _35: ();
{
(*_5) = !99597569417934771382195610811249017672_u128;
RET = '\u{27036}';
_7 = -(-9223372036854775808_isize);
(*_5) = 6110200750248605151_u64 as u128;
_5 = _3;
_1 = 3985507410_u32 as f64;
_1 = 18493369_i32 as f64;
_5 = core::ptr::addr_of!((*_5));
_5 = core::ptr::addr_of!((*_5));
_2 = (-6534_i16) - (-20356_i16);
_2 = -6011_i16;
RET = '\u{d7db8}';
Goto(bb1)
}
bb1 = {
(*_5) = !51759835973723670534537558438379168559_u128;
_2 = -1237_i16;
RET = '\u{3afed}';
_9 = core::ptr::addr_of!(RET);
(*_3) = 144389242141403802635075859499285353103_u128;
_10 = core::ptr::addr_of!((*_9));
(*_3) = _7 as u128;
_8 = (*_3);
(*_9) = '\u{1023b}';
_8 = (*_5);
(*_3) = _8;
_7 = 94_isize - 9223372036854775807_isize;
Goto(bb2)
}
bb2 = {
(*_5) = _8 >> _7;
_9 = _10;
_9 = core::ptr::addr_of!((*_9));
_5 = _3;
_6 = [(*_10),(*_10),(*_10),(*_9),(*_10),(*_10),(*_10),(*_9)];
_6 = [RET,RET,(*_10),RET,RET,RET,(*_10),(*_10)];
(*_3) = (-61031735300006145850291172796893899541_i128) as u128;
Goto(bb3)
}
bb3 = {
_8 = (*_3);
Goto(bb4)
}
bb4 = {
_4 = [102_i8,55_i8,(-26_i8),(-21_i8),(-117_i8)];
_3 = core::ptr::addr_of!(_8);
(*_10) = '\u{95ad0}';
(*_9) = '\u{f6e6}';
(*_10) = '\u{73dfd}';
_1 = _2 as f64;
(*_10) = '\u{7350b}';
_12 = [10281_u16,301_u16,24939_u16,51401_u16];
_1 = 203_u8 as f64;
(*_10) = '\u{c31a3}';
_3 = core::ptr::addr_of!(_8);
_9 = core::ptr::addr_of!(RET);
_11 = _7 << (*_3);
(*_3) = (*_5);
_10 = core::ptr::addr_of!(RET);
(*_5) = (*_3);
_13 = 6327552232768660716_usize as u16;
(*_9) = '\u{19c77}';
(*_9) = '\u{10fa0a}';
_12 = [_13,_13,_13,_13];
(*_5) = !_8;
RET = '\u{a8104}';
_6 = [RET,(*_10),RET,(*_9),RET,RET,(*_9),(*_9)];
Goto(bb5)
}
bb5 = {
(*_10) = '\u{d8a5f}';
(*_10) = '\u{405d4}';
(*_5) = 5724603768262804366_u64 as u128;
_8 = !(*_5);
(*_9) = '\u{a9f9d}';
(*_3) = (*_5);
_12 = [_13,_13,_13,_13];
_11 = 3223075264_u32 as isize;
Goto(bb6)
}
bb6 = {
_9 = core::ptr::addr_of!((*_9));
_3 = core::ptr::addr_of!((*_5));
Goto(bb7)
}
bb7 = {
_3 = core::ptr::addr_of!(_8);
(*_9) = '\u{5808f}';
_7 = _11;
_12 = [_13,_13,_13,_13];
_12 = [_13,_13,_13,_13];
RET = '\u{2051c}';
_19 = !true;
_7 = -_11;
_15 = [1565974167_u32,1302057449_u32,1986562589_u32];
_4 = [(-27_i8),90_i8,49_i8,113_i8,88_i8];
(*_10) = '\u{43b08}';
(*_9) = '\u{ef2f1}';
RET = '\u{d756b}';
_10 = core::ptr::addr_of!((*_9));
(*_3) = (*_5) + (*_5);
_14 = _13;
Goto(bb8)
}
bb8 = {
_3 = _5;
(*_9) = '\u{ef01b}';
_11 = _7 & _7;
(*_3) = !_8;
_11 = !_7;
RET = '\u{df42c}';
_4 = [73_i8,(-55_i8),(-120_i8),98_i8,97_i8];
_14 = !_13;
_7 = _11 | _11;
_7 = -_11;
Goto(bb9)
}
bb9 = {
_19 = _1 <= _1;
_15 = [1819120683_u32,1597346289_u32,1777020971_u32];
_8 = (*_3);
(*_3) = _8 & _8;
(*_5) = _8 * _8;
_14 = 82985562271050269_i64 as u16;
Goto(bb10)
}
bb10 = {
_13 = _2 as u16;
_21 = _1 * _1;
(*_3) = !_8;
_16 = !1730436406952771160_u64;
(*_10) = '\u{30746}';
(*_5) = _8;
_12 = [_13,_13,_14,_13];
_22 = (*_9);
RET = _22;
RET = _22;
(*_10) = _22;
_21 = _13 as f64;
_19 = (*_5) != (*_5);
(*_9) = _22;
Goto(bb11)
}
bb11 = {
_8 = !(*_3);
Goto(bb12)
}
bb12 = {
(*_10) = _22;
RET = _22;
_9 = _10;
_12 = [_14,_14,_14,_13];
(*_3) = 1708759041_u32 as u128;
_24 = -_2;
_16 = 30_i8 as u64;
(*_10) = _22;
(*_9) = _22;
Goto(bb13)
}
bb13 = {
_22 = (*_9);
_8 = (*_3);
_26.1 = (*_9);
_25 = 2_u8;
_14 = !_13;
_28 = [_16,_16,_16,_16];
_9 = core::ptr::addr_of!((*_9));
_3 = core::ptr::addr_of!((*_5));
_26 = (_1, RET, 8022877024391920079_usize);
(*_10) = _26.1;
_1 = -_26.0;
_25 = 185_u8;
_21 = _1;
_19 = false;
(*_3) = _8 + _8;
match _26.2 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
8022877024391920079 => bb19,
_ => bb18
}
}
bb14 = {
_4 = [102_i8,55_i8,(-26_i8),(-21_i8),(-117_i8)];
_3 = core::ptr::addr_of!(_8);
(*_10) = '\u{95ad0}';
(*_9) = '\u{f6e6}';
(*_10) = '\u{73dfd}';
_1 = _2 as f64;
(*_10) = '\u{7350b}';
_12 = [10281_u16,301_u16,24939_u16,51401_u16];
_1 = 203_u8 as f64;
(*_10) = '\u{c31a3}';
_3 = core::ptr::addr_of!(_8);
_9 = core::ptr::addr_of!(RET);
_11 = _7 << (*_3);
(*_3) = (*_5);
_10 = core::ptr::addr_of!(RET);
(*_5) = (*_3);
_13 = 6327552232768660716_usize as u16;
(*_9) = '\u{19c77}';
(*_9) = '\u{10fa0a}';
_12 = [_13,_13,_13,_13];
(*_5) = !_8;
RET = '\u{a8104}';
_6 = [RET,(*_10),RET,(*_9),RET,RET,(*_9),(*_9)];
Goto(bb5)
}
bb15 = {
(*_5) = !51759835973723670534537558438379168559_u128;
_2 = -1237_i16;
RET = '\u{3afed}';
_9 = core::ptr::addr_of!(RET);
(*_3) = 144389242141403802635075859499285353103_u128;
_10 = core::ptr::addr_of!((*_9));
(*_3) = _7 as u128;
_8 = (*_3);
(*_9) = '\u{1023b}';
_8 = (*_5);
(*_3) = _8;
_7 = 94_isize - 9223372036854775807_isize;
Goto(bb2)
}
bb16 = {
_9 = core::ptr::addr_of!((*_9));
_3 = core::ptr::addr_of!((*_5));
Goto(bb7)
}
bb17 = {
_8 = (*_3);
Goto(bb4)
}
bb18 = {
(*_5) = _8 >> _7;
_9 = _10;
_9 = core::ptr::addr_of!((*_9));
_5 = _3;
_6 = [(*_10),(*_10),(*_10),(*_9),(*_10),(*_10),(*_10),(*_9)];
_6 = [RET,RET,(*_10),RET,RET,RET,(*_10),(*_10)];
(*_3) = (-61031735300006145850291172796893899541_i128) as u128;
Goto(bb3)
}
bb19 = {
_14 = _13 & _13;
(*_5) = !_8;
RET = _22;
(*_10) = _26.1;
_17 = Adt51::Variant2 { fld0: _16 };
_27 = Move(_17);
_12 = [_14,_14,_13,_13];
_8 = (*_5);
_28 = [Field::<u64>(Variant(_27, 2), 0),Field::<u64>(Variant(_27, 2), 0),Field::<u64>(Variant(_27, 2), 0),_16];
_23 = 87_i8 as f64;
_8 = _7 as u128;
_33 = 2_i8;
(*_3) = _8;
_5 = core::ptr::addr_of!((*_5));
Goto(bb20)
}
bb20 = {
Call(_34 = dump_var(16_usize, 7_usize, Move(_7), 15_usize, Move(_15), 11_usize, Move(_11), 4_usize, Move(_4)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_34 = dump_var(16_usize, 6_usize, Move(_6), 16_usize, Move(_16), 8_usize, Move(_8), 25_usize, Move(_25)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: *const u128,mut _2: bool,mut _3: [char; 8],mut _4: [char; 8],mut _5: [char; 8],mut _6: char,mut _7: bool) -> i16 {
mir! {
type RET = i16;
let _8: u128;
let _9: i8;
let _10: bool;
let _11: [i64; 3];
let _12: ();
let _13: ();
{
_4 = [_6,_6,_6,_6,_6,_6,_6,_6];
_5 = [_6,_6,_6,_6,_6,_6,_6,_6];
RET = (-21253_i16);
_7 = !_2;
_6 = '\u{23df5}';
RET = 21603_i16;
_5 = [_6,_6,_6,_6,_6,_6,_6,_6];
(*_1) = 136813099649634988326178861414709451126_u128;
RET = _2 as i16;
_6 = '\u{2c9ef}';
_8 = (*_1);
_8 = !(*_1);
_9 = (-16_i8) >> RET;
_8 = !(*_1);
_3 = _5;
_7 = _2 & _2;
_5 = _3;
_5 = _3;
_3 = [_6,_6,_6,_6,_6,_6,_6,_6];
_10 = _7;
_10 = !_7;
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(17_usize, 3_usize, Move(_3), 4_usize, Move(_4), 9_usize, Move(_9), 6_usize, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box((-124751553385046240544034802523419565778_i128)), std::hint::black_box('\u{1071a4}'), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(501080219_i32), std::hint::black_box(954226671_u32));
                
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: [i8; 5],
fld1: u128,

},
Variant1{
fld0: *const *const u16,
fld1: f64,
fld2: [u128; 5],
fld3: ((u32, *mut i64, *const u128), i16, u16),

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: (u16,),
fld1: char,
fld2: ((u32, *mut i64, *const u128), i16, u16),
fld3: *mut i64,
fld4: *const char,
fld5: i32,
fld6: i64,
fld7: *const u128,
}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: Adt44,
fld1: ((u32, *mut i64, *const u128), i16, u16),
fld2: u16,
fld3: [i64; 3],
fld4: (u32, *mut i64, *const u128),
fld5: *mut *mut [char; 8],
fld6: *const u16,
fld7: *mut [char; 8],

},
Variant1{
fld0: bool,
fld1: Adt44,
fld2: [u64; 4],
fld3: f64,
fld4: Adt43,
fld5: (u32, *mut i64, *const u128),

},
Variant2{
fld0: (f64, [i8; 5], usize),
fld1: [char; 8],

},
Variant3{
fld0: bool,
fld1: char,
fld2: *mut i128,
fld3: i8,
fld4: i16,
fld5: *mut [char; 8],
fld6: usize,
fld7: u64,

}}
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: bool,
fld1: ((u32, *mut i64, *const u128), i16, u16),
fld2: [u128; 5],
fld3: (u16,),
fld4: u32,
fld5: *mut i128,
fld6: u8,

},
Variant1{
fld0: bool,
fld1: char,
fld2: *const char,
fld3: [char; 8],
fld4: *mut i128,
fld5: i32,

},
Variant2{
fld0: (u16,),

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: *mut i128,
fld1: Adt45,
fld2: ((u32, *mut i64, *const u128), i16, u16),
fld3: *mut [char; 8],
fld4: (f64, char, usize),

},
Variant1{
fld0: *mut i128,
fld1: (f64, char, usize),
fld2: Adt44,
fld3: i8,
fld4: u64,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: bool,
fld1: Adt44,
fld2: Adt46,
fld3: [u32; 3],
fld4: [u8; 5],
fld5: ((u32, *mut i64, *const u128), i16, u16),
fld6: (f64, [i8; 5], usize),
}
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
fld0: i64,
fld1: Adt47,
fld2: f64,
fld3: i128,
fld4: [i64; 3],
fld5: *mut *mut [char; 8],

},
Variant1{
fld0: *mut [char; 8],
fld1: [i64; 3],
fld2: *mut i128,
fld3: *const char,
fld4: *const *const u16,
fld5: Adt44,

},
Variant2{
fld0: [u64; 4],
fld1: *const char,
fld2: (i16,),
fld3: i8,
fld4: i16,
fld5: i64,

},
Variant3{
fld0: [i64; 3],
fld1: (u32, *mut i64, *const u128),
fld2: [u16; 1],
fld3: (u16,),
fld4: Adt46,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: Adt43,
fld1: *mut [char; 8],
fld2: (u16,),
fld3: (f64, [i8; 5], usize),
fld4: [u16; 1],
fld5: *const u16,

},
Variant1{
fld0: [i8; 5],
fld1: u16,
fld2: isize,
fld3: usize,
fld4: u32,
fld5: [i64; 3],
fld6: *const u128,

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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: (f64, char, usize),
fld1: Adt45,
fld2: [u16; 4],
fld3: [u8; 5],
fld4: *mut [char; 8],
fld5: [char; 8],
fld6: Adt47,
fld7: [u64; 4],

},
Variant1{
fld0: ((u32, *mut i64, *const u128), i16, u16),
fld1: (i16,),
fld2: Adt46,
fld3: *const u16,
fld4: u8,

},
Variant2{
fld0: u64,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: usize,
fld1: Adt44,
fld2: isize,
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: [i64; 3],
fld1: Adt52,
fld2: u16,

},
Variant1{
fld0: bool,
fld1: char,
fld2: [u16; 4],
fld3: i8,
fld4: (f64, [i8; 5], usize),
fld5: *mut i128,
fld6: Adt46,
fld7: f32,

},
Variant2{
fld0: bool,
fld1: [u16; 4],
fld2: [i8; 5],

},
Variant3{
fld0: [u128; 5],
fld1: [char; 8],
fld2: Adt43,
fld3: (u16,),
fld4: (u32, *mut i64, *const u128),
fld5: u32,
fld6: [u16; 4],
fld7: u8,

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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: bool,
fld1: (f64, char, usize),
fld2: *mut *mut [char; 8],
fld3: *const u128,
fld4: ((u32, *mut i64, *const u128), i16, u16),
fld5: f32,
fld6: Adt49,
fld7: [i64; 3],

},
Variant1{
fld0: *const *const u16,
fld1: i16,
fld2: [char; 8],

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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: u64,
fld1: *mut i128,
fld2: *mut *mut [char; 8],
fld3: [u64; 4],
fld4: f64,
fld5: *const char,
fld6: u32,

},
Variant1{
fld0: ((u32, *mut i64, *const u128), i16, u16),
fld1: char,
fld2: *const char,
fld3: Adt45,
fld4: [i64; 3],
fld5: i32,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: Adt52,
fld1: ((u32, *mut i64, *const u128), i16, u16),
fld2: *const u128,
fld3: i8,
fld4: *mut *mut [char; 8],
fld5: (u32, *mut i64, *const u128),
fld6: f64,

},
Variant1{
fld0: isize,

},
Variant2{
fld0: [u128; 5],
fld1: Adt51,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt57{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt57 {
fld0: i32,
fld1: [u16; 4],
fld2: *const u128,
}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt58::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: bool,
fld1: *const u16,
fld2: u32,
fld3: Adt52,
fld4: ((u32, *mut i64, *const u128), i16, u16),
fld5: i32,
fld6: (u16,),
fld7: Adt57,

},
Variant1{
fld0: [u8; 5],
fld1: Adt52,
fld2: (f64, char, usize),
fld3: f32,
fld4: Adt45,

},
Variant2{
fld0: Adt50,
fld1: *mut *mut [char; 8],
fld2: Adt56,
fld3: (f64, char, usize),
fld4: f32,
fld5: [u16; 4],

},
Variant3{
fld0: usize,

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt59::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: u8,
fld1: Adt50,
fld2: (u32, *mut i64, *const u128),

},
Variant1{
fld0: [char; 8],
fld1: (i16,),
fld2: u8,
fld3: Adt53,
fld4: [u16; 1],
fld5: u64,

},
Variant2{
fld0: *const u128,
fld1: *const u16,
fld2: isize,
fld3: Adt49,
fld4: i64,

},
Variant3{
fld0: [u8; 5],

}}

