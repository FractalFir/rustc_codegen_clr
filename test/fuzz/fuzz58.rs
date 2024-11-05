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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u64) -> isize {
mir! {
type RET = isize;
let _13: u64;
let _14: [u64; 2];
let _15: [i16; 5];
let _16: i64;
let _17: u16;
let _18: [u16; 8];
let _19: *mut u16;
let _20: (f64,);
let _21: i8;
let _22: [u64; 2];
let _23: u8;
let _24: Adt46;
let _25: isize;
let _26: f64;
let _27: i64;
let _28: [i64; 8];
let _29: Adt47;
let _30: ();
let _31: ();
{
RET = (-9223372036854775808_isize) << 15484_u16;
RET = (-501534523_i32) as isize;
_7 = 6302857660498155724_i64 >> RET;
_2 = '\u{3b7c9}';
_6 = !(-729484435_i32);
_5 = (-18375_i16) & 20221_i16;
_4 = (-114_i8);
Call(_9 = fn1(_4, _4, _4, _5, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = !56_isize;
_11 = 45359_u16 | 59339_u16;
_6 = (-521628024_i32) - (-1154247750_i32);
_9 = 15397563501925485253_usize * 6_usize;
_10 = 127_u8 - 139_u8;
_14 = [6035590193416477817_u64,6737385497222630748_u64];
_1 = false;
_7 = (-6134818064372375338_i64) << _4;
_1 = false;
_5 = (-14450_i16) | 8251_i16;
RET = 9223372036854775807_isize & 9223372036854775807_isize;
_4 = 7271453888756879314_u64 as i8;
_16 = _7 & _7;
RET = !(-9223372036854775808_isize);
_3 = RET >> _16;
_12 = !4847518546788903537_u64;
_6 = _10 as i32;
_3 = -RET;
_15 = [_5,_5,_5,_5,_5];
_11 = !56745_u16;
_15 = [_5,_5,_5,_5,_5];
_2 = '\u{10b5e}';
_8 = 8764534141355516557299981642129351484_i128;
_17 = !_11;
_15 = [_5,_5,_5,_5,_5];
Goto(bb2)
}
bb2 = {
_13 = _12;
_18 = [_11,_17,_17,_11,_17,_11,_17,_11];
_18 = [_11,_11,_11,_11,_17,_17,_17,_11];
_15 = [_5,_5,_5,_5,_5];
_1 = false & false;
_7 = _16 | _16;
_13 = !_12;
_9 = _1 as usize;
_13 = _12;
_15 = [_5,_5,_5,_5,_5];
_10 = 89_u8 ^ 128_u8;
_17 = _13 as u16;
_2 = '\u{339ac}';
_19 = core::ptr::addr_of_mut!(_11);
_10 = 153_u8;
_19 = core::ptr::addr_of_mut!((*_19));
_6 = !738595593_i32;
_6 = _8 as i32;
_11 = _17 & _17;
_3 = RET;
_17 = 118781743109053209542195645126894269170_u128 as u16;
_8 = 135823273592162431677841473373959928452_i128 | (-121165693157324228019712285251866184328_i128);
RET = _4 as isize;
_14 = [_13,_12];
_11 = !_17;
_3 = !RET;
_5 = _10 as i16;
match _10 {
0 => bb1,
1 => bb3,
2 => bb4,
153 => bb6,
_ => bb5
}
}
bb3 = {
RET = !56_isize;
_11 = 45359_u16 | 59339_u16;
_6 = (-521628024_i32) - (-1154247750_i32);
_9 = 15397563501925485253_usize * 6_usize;
_10 = 127_u8 - 139_u8;
_14 = [6035590193416477817_u64,6737385497222630748_u64];
_1 = false;
_7 = (-6134818064372375338_i64) << _4;
_1 = false;
_5 = (-14450_i16) | 8251_i16;
RET = 9223372036854775807_isize & 9223372036854775807_isize;
_4 = 7271453888756879314_u64 as i8;
_16 = _7 & _7;
RET = !(-9223372036854775808_isize);
_3 = RET >> _16;
_12 = !4847518546788903537_u64;
_6 = _10 as i32;
_3 = -RET;
_15 = [_5,_5,_5,_5,_5];
_11 = !56745_u16;
_15 = [_5,_5,_5,_5,_5];
_2 = '\u{10b5e}';
_8 = 8764534141355516557299981642129351484_i128;
_17 = !_11;
_15 = [_5,_5,_5,_5,_5];
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
RET = _1 as isize;
_19 = core::ptr::addr_of_mut!((*_19));
_16 = !_7;
_9 = 14324766896876359629_usize;
_15 = [_5,_5,_5,_5,_5];
_19 = core::ptr::addr_of_mut!(_17);
_9 = 0_usize;
(*_19) = !_18[_9];
_21 = 3695653651_u32 as i8;
_17 = _11;
_13 = !_14[_9];
Call(_13 = core::intrinsics::bswap(_14[_9]), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_15 = [_5,_5,_5,_5,_5];
_12 = _13 & _13;
_10 = 310223341719616610179838111400754084861_u128 as u8;
_14[_9] = _13;
_21 = _9 as i8;
_19 = core::ptr::addr_of_mut!((*_19));
_14[_9] = _12 * _13;
_21 = _4 * _4;
_11 = !(*_19);
_19 = core::ptr::addr_of_mut!(_17);
_18 = [(*_19),_11,(*_19),_11,(*_19),(*_19),_17,(*_19)];
match _9 {
0 => bb8,
_ => bb5
}
}
bb8 = {
_2 = '\u{b1b44}';
_4 = !_21;
Goto(bb9)
}
bb9 = {
_23 = _10 + _10;
_7 = _16 + _16;
_26 = _15[_9] as f64;
_15[_9] = _5 >> _7;
_22 = _14;
_21 = _4;
_8 = (-44924035828222300614132258051355866153_i128) + (-25232731589009435251444373110919349946_i128);
_7 = _16 & _16;
RET = _3 ^ _3;
_2 = '\u{766fa}';
_15 = [_5,_5,_5,_5,_5];
_16 = -_7;
match _9 {
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
0 => bb16,
_ => bb15
}
}
bb10 = {
_2 = '\u{b1b44}';
_4 = !_21;
Goto(bb9)
}
bb11 = {
_15 = [_5,_5,_5,_5,_5];
_12 = _13 & _13;
_10 = 310223341719616610179838111400754084861_u128 as u8;
_14[_9] = _13;
_21 = _9 as i8;
_19 = core::ptr::addr_of_mut!((*_19));
_14[_9] = _12 * _13;
_21 = _4 * _4;
_11 = !(*_19);
_19 = core::ptr::addr_of_mut!(_17);
_18 = [(*_19),_11,(*_19),_11,(*_19),(*_19),_17,(*_19)];
match _9 {
0 => bb8,
_ => bb5
}
}
bb12 = {
RET = _1 as isize;
_19 = core::ptr::addr_of_mut!((*_19));
_16 = !_7;
_9 = 14324766896876359629_usize;
_15 = [_5,_5,_5,_5,_5];
_19 = core::ptr::addr_of_mut!(_17);
_9 = 0_usize;
(*_19) = !_18[_9];
_21 = 3695653651_u32 as i8;
_17 = _11;
_13 = !_14[_9];
Call(_13 = core::intrinsics::bswap(_14[_9]), ReturnTo(bb7), UnwindUnreachable())
}
bb13 = {
Return()
}
bb14 = {
_13 = _12;
_18 = [_11,_17,_17,_11,_17,_11,_17,_11];
_18 = [_11,_11,_11,_11,_17,_17,_17,_11];
_15 = [_5,_5,_5,_5,_5];
_1 = false & false;
_7 = _16 | _16;
_13 = !_12;
_9 = _1 as usize;
_13 = _12;
_15 = [_5,_5,_5,_5,_5];
_10 = 89_u8 ^ 128_u8;
_17 = _13 as u16;
_2 = '\u{339ac}';
_19 = core::ptr::addr_of_mut!(_11);
_10 = 153_u8;
_19 = core::ptr::addr_of_mut!((*_19));
_6 = !738595593_i32;
_6 = _8 as i32;
_11 = _17 & _17;
_3 = RET;
_17 = 118781743109053209542195645126894269170_u128 as u16;
_8 = 135823273592162431677841473373959928452_i128 | (-121165693157324228019712285251866184328_i128);
RET = _4 as isize;
_14 = [_13,_12];
_11 = !_17;
_3 = !RET;
_5 = _10 as i16;
match _10 {
0 => bb1,
1 => bb3,
2 => bb4,
153 => bb6,
_ => bb5
}
}
bb15 = {
RET = !56_isize;
_11 = 45359_u16 | 59339_u16;
_6 = (-521628024_i32) - (-1154247750_i32);
_9 = 15397563501925485253_usize * 6_usize;
_10 = 127_u8 - 139_u8;
_14 = [6035590193416477817_u64,6737385497222630748_u64];
_1 = false;
_7 = (-6134818064372375338_i64) << _4;
_1 = false;
_5 = (-14450_i16) | 8251_i16;
RET = 9223372036854775807_isize & 9223372036854775807_isize;
_4 = 7271453888756879314_u64 as i8;
_16 = _7 & _7;
RET = !(-9223372036854775808_isize);
_3 = RET >> _16;
_12 = !4847518546788903537_u64;
_6 = _10 as i32;
_3 = -RET;
_15 = [_5,_5,_5,_5,_5];
_11 = !56745_u16;
_15 = [_5,_5,_5,_5,_5];
_2 = '\u{10b5e}';
_8 = 8764534141355516557299981642129351484_i128;
_17 = !_11;
_15 = [_5,_5,_5,_5,_5];
Goto(bb2)
}
bb16 = {
_11 = _2 as u16;
_3 = !RET;
_21 = -_4;
RET = _3 >> _11;
_22[_9] = _14[_9];
_13 = 93144501002730252351437566355130642814_u128 as u64;
_20.0 = _26 + _26;
_11 = _18[_9] | (*_19);
_12 = _8 as u64;
_22 = [_14[_9],_12];
_15[_9] = 468054121_u32 as i16;
Goto(bb17)
}
bb17 = {
Call(_30 = dump_var(0_usize, 13_usize, Move(_13), 17_usize, Move(_17), 18_usize, Move(_18), 21_usize, Move(_21)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_30 = dump_var(0_usize, 4_usize, Move(_4), 14_usize, Move(_14), 9_usize, Move(_9), 12_usize, Move(_12)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_30 = dump_var(0_usize, 10_usize, Move(_10), 22_usize, Move(_22), 31_usize, _31, 31_usize, _31), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i8,mut _2: i8,mut _3: i8,mut _4: i16,mut _5: i16) -> usize {
mir! {
type RET = usize;
let _6: i16;
let _7: (i8, i32, f64, i128);
let _8: i64;
let _9: [i16; 5];
let _10: Adt55;
let _11: u8;
let _12: Adt49;
let _13: [u16; 8];
let _14: Adt45;
let _15: i16;
let _16: [i16; 7];
let _17: i64;
let _18: *const u32;
let _19: Adt45;
let _20: (f64,);
let _21: &'static f64;
let _22: f64;
let _23: Adt59;
let _24: Adt51;
let _25: Adt51;
let _26: [u32; 3];
let _27: isize;
let _28: Adt45;
let _29: bool;
let _30: *mut char;
let _31: Adt53;
let _32: Adt51;
let _33: ();
let _34: ();
{
RET = _3 as usize;
_1 = _2 & _2;
_1 = !_2;
_3 = 44811_u16 as i8;
_5 = _4 ^ _4;
_5 = 4959314494184129439_i64 as i16;
_6 = _5;
_6 = _4 ^ _4;
_7.2 = 38128_u16 as f64;
_6 = _4 ^ _5;
_6 = _5 ^ _5;
_3 = _7.2 as i8;
_8 = 9182812619017860714_i64 & 454662354479807860_i64;
_6 = -_4;
RET = 14804138295481972354_usize >> _6;
_7.3 = !33276037096024266809077099692350301059_i128;
_7.1 = (-1120229712_i32) * 1439311689_i32;
RET = 18139584755591979670_usize;
_9 = [_4,_5,_4,_6,_6];
_2 = _1 + _1;
RET = 9223372036854775807_isize as usize;
_1 = _2 << _5;
_7.1 = -2020674485_i32;
Goto(bb1)
}
bb1 = {
_6 = _4 & _4;
_7.3 = (-85428772673969308057034459097880003198_i128);
RET = 4728656220040990081_usize << _6;
_12 = Adt49::Variant0 { fld0: _7.1 };
_5 = -_6;
_7.1 = !Field::<i32>(Variant(_12, 0), 0);
_7.0 = -_2;
_7.2 = (-9223372036854775808_isize) as f64;
Goto(bb2)
}
bb2 = {
_7.3 = 42374910405895086401989835273003090462_i128 | (-75762747950247227395651885646096828140_i128);
_1 = '\u{32e4d}' as i8;
RET = 1831442320237910765_u64 as usize;
_7.3 = true as i128;
_15 = _7.2 as i16;
_3 = 140_u8 as i8;
_6 = _15 - _4;
_7.2 = 117221561032697785904134915425288913965_u128 as f64;
_15 = _1 as i16;
_7.0 = !_3;
RET = !7_usize;
_8 = 8174705823736361556_i64 & (-306136069690823597_i64);
_14.fld0 = _2;
_15 = !_6;
_17 = 15884888119651480826_u64 as i64;
_14.fld2 = -_15;
_14.fld1 = [1904447463194866770_u64,267291516200784610_u64];
_16 = [_15,_6,_15,_15,_14.fld2,_15,_4];
_7.2 = _5 as f64;
_11 = !226_u8;
_14.fld2 = _5;
_14.fld1 = [8103273589397022853_u64,11592288277829574469_u64];
_1 = _14.fld0 * _3;
_12 = Adt49::Variant0 { fld0: _7.1 };
Call(_3 = fn2(_14.fld0, _15, _7, _14.fld1, _11, _7.0, _7.2, _7, _14.fld1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11 = !18_u8;
RET = 9223372036854775807_isize as usize;
_14.fld0 = -_7.0;
_7.1 = Field::<i32>(Variant(_12, 0), 0);
place!(Field::<i32>(Variant(_12, 0), 0)) = 3321929363_u32 as i32;
Goto(bb4)
}
bb4 = {
RET = 6_usize;
_16 = [_6,_15,_14.fld2,_15,_4,_5,_6];
_3 = -_1;
_7.1 = Field::<i32>(Variant(_12, 0), 0) << _14.fld2;
place!(Field::<i32>(Variant(_12, 0), 0)) = -_7.1;
_8 = _17 * _17;
_11 = !117_u8;
_13[RET] = 4088563981_u32 as u16;
_11 = 79_u8 * 204_u8;
_19 = Adt45 { fld0: _14.fld0,fld1: _14.fld1,fld2: _6 };
_13[RET] = 23519_u16 + 58533_u16;
_16[RET] = _6;
match RET {
0 => bb1,
1 => bb2,
6 => bb5,
_ => bb3
}
}
bb5 = {
_17 = _8;
_15 = _5;
_16 = [_19.fld2,_6,_15,_14.fld2,_5,_15,_15];
_19.fld2 = _4 >> _19.fld0;
_13 = [12326_u16,54895_u16,28915_u16,50658_u16,26874_u16,14269_u16,21894_u16,53281_u16];
_2 = _11 as i8;
place!(Field::<i32>(Variant(_12, 0), 0)) = _7.3 as i32;
_9 = [_15,_4,_5,_4,_4];
_7.0 = !_2;
_6 = '\u{da16b}' as i16;
RET = 1_usize;
_19.fld0 = _1;
_3 = !_1;
_11 = !39_u8;
_14.fld2 = _6;
_1 = _19.fld0;
_1 = _7.0 << _13[RET];
_20.0 = _7.2;
_23.fld0.fld1 = [_14.fld1[RET],_19.fld1[RET]];
_23.fld0.fld1 = [_14.fld1[RET],_19.fld1[RET]];
_23.fld4.3 = [true,false,true,false,true,true];
_14.fld0 = _19.fld2 as i8;
RET = 3_usize;
_23.fld2.fld0 = _4 & _15;
_23.fld0.fld2 = _11 as i16;
match _13[RET] {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
50658 => bb9,
_ => bb8
}
}
bb6 = {
RET = 6_usize;
_16 = [_6,_15,_14.fld2,_15,_4,_5,_6];
_3 = -_1;
_7.1 = Field::<i32>(Variant(_12, 0), 0) << _14.fld2;
place!(Field::<i32>(Variant(_12, 0), 0)) = -_7.1;
_8 = _17 * _17;
_11 = !117_u8;
_13[RET] = 4088563981_u32 as u16;
_11 = 79_u8 * 204_u8;
_19 = Adt45 { fld0: _14.fld0,fld1: _14.fld1,fld2: _6 };
_13[RET] = 23519_u16 + 58533_u16;
_16[RET] = _6;
match RET {
0 => bb1,
1 => bb2,
6 => bb5,
_ => bb3
}
}
bb7 = {
_11 = !18_u8;
RET = 9223372036854775807_isize as usize;
_14.fld0 = -_7.0;
_7.1 = Field::<i32>(Variant(_12, 0), 0);
place!(Field::<i32>(Variant(_12, 0), 0)) = 3321929363_u32 as i32;
Goto(bb4)
}
bb8 = {
_6 = _4 & _4;
_7.3 = (-85428772673969308057034459097880003198_i128);
RET = 4728656220040990081_usize << _6;
_12 = Adt49::Variant0 { fld0: _7.1 };
_5 = -_6;
_7.1 = !Field::<i32>(Variant(_12, 0), 0);
_7.0 = -_2;
_7.2 = (-9223372036854775808_isize) as f64;
Goto(bb2)
}
bb9 = {
_5 = _3 as i16;
_23.fld4.0 = _7.1 == _7.1;
_23.fld2 = Adt50 { fld0: _15,fld1: _23.fld4.3 };
_23.fld2 = Adt50 { fld0: _19.fld2,fld1: _23.fld4.3 };
_9 = [_15,_23.fld2.fld0,_23.fld2.fld0,_16[RET],_19.fld2];
_19.fld0 = -_1;
_6 = _8 as i16;
match RET {
0 => bb4,
1 => bb2,
2 => bb10,
3 => bb12,
_ => bb11
}
}
bb10 = {
_11 = !18_u8;
RET = 9223372036854775807_isize as usize;
_14.fld0 = -_7.0;
_7.1 = Field::<i32>(Variant(_12, 0), 0);
place!(Field::<i32>(Variant(_12, 0), 0)) = 3321929363_u32 as i32;
Goto(bb4)
}
bb11 = {
_11 = !18_u8;
RET = 9223372036854775807_isize as usize;
_14.fld0 = -_7.0;
_7.1 = Field::<i32>(Variant(_12, 0), 0);
place!(Field::<i32>(Variant(_12, 0), 0)) = 3321929363_u32 as i32;
Goto(bb4)
}
bb12 = {
_23.fld2.fld1[RET] = _23.fld4.0;
_23.fld7 = _7.3 + _7.3;
_3 = -_19.fld0;
SetDiscriminant(_12, 0);
_15 = _23.fld0.fld2;
_23.fld7 = !_7.3;
place!(Field::<i32>(Variant(_12, 0), 0)) = _7.1 - _7.1;
Goto(bb13)
}
bb13 = {
_27 = (-9223372036854775808_isize);
_23.fld0.fld0 = 2903201780_u32 as i8;
_12 = Adt49::Variant0 { fld0: _7.1 };
_18 = core::ptr::addr_of!(_23.fld1);
SetDiscriminant(_12, 0);
_23.fld4.2 = -_17;
_2 = _1;
_14.fld0 = 1864649250_u32 as i8;
_13 = [31880_u16,200_u16,47045_u16,48251_u16,41663_u16,29633_u16,3105_u16,61683_u16];
_23.fld0.fld0 = !_3;
_2 = !_23.fld0.fld0;
place!(Field::<i32>(Variant(_12, 0), 0)) = -_7.1;
_7.0 = _3;
_2 = !_7.0;
_20.0 = -_7.2;
_22 = _13[RET] as f64;
SetDiscriminant(_12, 0);
_19 = _14;
_16 = [_23.fld0.fld2,_23.fld2.fld0,_9[RET],_5,_19.fld2,_9[RET],_23.fld2.fld0];
_7.3 = -_23.fld7;
_20 = (_22,);
RET = _22 as usize;
_23.fld4.1 = _1 as f32;
place!(Field::<i32>(Variant(_12, 0), 0)) = !_7.1;
_23.fld4.1 = _11 as f32;
_23.fld6 = Adt52::Variant0 { fld0: _7.1 };
_28.fld1 = _19.fld1;
match _27 {
340282366920938463454151235394913435648 => bb15,
_ => bb14
}
}
bb14 = {
_6 = _4 & _4;
_7.3 = (-85428772673969308057034459097880003198_i128);
RET = 4728656220040990081_usize << _6;
_12 = Adt49::Variant0 { fld0: _7.1 };
_5 = -_6;
_7.1 = !Field::<i32>(Variant(_12, 0), 0);
_7.0 = -_2;
_7.2 = (-9223372036854775808_isize) as f64;
Goto(bb2)
}
bb15 = {
_27 = 15803_u16 as isize;
Goto(bb16)
}
bb16 = {
Call(_33 = dump_var(1_usize, 17_usize, Move(_17), 2_usize, Move(_2), 15_usize, Move(_15), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(1_usize, 1_usize, Move(_1), 13_usize, Move(_13), 6_usize, Move(_6), 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: i8,mut _2: i16,mut _3: (i8, i32, f64, i128),mut _4: [u64; 2],mut _5: u8,mut _6: i8,mut _7: f64,mut _8: (i8, i32, f64, i128),mut _9: [u64; 2]) -> i8 {
mir! {
type RET = i8;
let _10: Adt47;
let _11: *const (i16, [bool; 6], &'static f64);
let _12: Adt47;
let _13: isize;
let _14: char;
let _15: Adt47;
let _16: [char; 6];
let _17: i8;
let _18: (bool, f32, i64, [bool; 6]);
let _19: Adt47;
let _20: bool;
let _21: i8;
let _22: bool;
let _23: [u32; 3];
let _24: [bool; 6];
let _25: i128;
let _26: [u32; 3];
let _27: f32;
let _28: [u64; 2];
let _29: ();
let _30: ();
{
_9 = _4;
_8.2 = _7;
_8 = (_6, _3.1, _3.2, _3.3);
_10.fld1 = '\u{d594}';
_8.3 = true as i128;
_3.3 = _8.3 << _1;
_10.fld1 = '\u{19457}';
_10.fld0 = _5 as f32;
_12 = _10;
_3.2 = _8.2 * _8.2;
_10.fld0 = _3.1 as f32;
RET = _3.0;
_13 = -(-9223372036854775808_isize);
_8.0 = _3.2 as i8;
_10.fld1 = _12.fld1;
_3.2 = _3.1 as f64;
_8.0 = RET;
_3.3 = !_8.3;
Call(_3.2 = fn3(_7, _2, _3.0, _8.2, RET, _5, _2, _4, _10.fld0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_15 = Adt47 { fld0: _10.fld0,fld1: _12.fld1 };
_12.fld0 = _3.1 as f32;
_3.3 = _8.3;
_3 = (_1, _8.1, _8.2, _8.3);
_6 = _3.0 << _8.3;
_3.0 = 14429332802662530090_usize as i8;
_3.1 = -_8.1;
_13 = 9223372036854775807_isize;
_6 = _10.fld1 as i8;
_14 = _15.fld1;
RET = _8.0 * _1;
_14 = _15.fld1;
_15.fld1 = _12.fld1;
RET = _3.0;
_2 = 24672_i16;
_16 = [_10.fld1,_10.fld1,_14,_15.fld1,_15.fld1,_10.fld1];
_3.2 = -_7;
_12.fld0 = -_10.fld0;
_9 = [1879457517875993091_u64,12095024620996547381_u64];
_8.0 = _5 as i8;
_1 = true as i8;
_14 = _12.fld1;
_19 = _15;
_2 = (-6593_i16);
match _13 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
9223372036854775807 => bb10,
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
_19.fld0 = _10.fld0 * _10.fld0;
RET = _1;
_12 = Adt47 { fld0: _10.fld0,fld1: _14 };
_15.fld0 = _10.fld0 - _19.fld0;
_10 = _19;
_14 = _15.fld1;
_13 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_3.3 = _8.3 << RET;
_12.fld1 = _15.fld1;
_15.fld0 = _10.fld0;
_8.1 = _15.fld0 as i32;
RET = _1;
_18.0 = false & false;
_2 = -1483_i16;
_3.0 = RET;
_19 = Adt47 { fld0: _15.fld0,fld1: _14 };
_15.fld1 = _12.fld1;
Goto(bb11)
}
bb11 = {
_15.fld0 = -_19.fld0;
_8.2 = _7;
_20 = !_18.0;
_18.1 = _15.fld0 + _15.fld0;
_19.fld0 = 3135108627_u32 as f32;
_14 = _10.fld1;
_22 = !_18.0;
_14 = _15.fld1;
_18.3 = [_20,_20,_18.0,_18.0,_22,_18.0];
_2 = 8172_i16;
_21 = _6;
_6 = _13 as i8;
_16 = [_14,_19.fld1,_19.fld1,_15.fld1,_15.fld1,_15.fld1];
_15.fld0 = _18.1;
_12.fld0 = (-3732382058938610696_i64) as f32;
_18.0 = _20;
_18.3 = [_22,_22,_22,_18.0,_22,_18.0];
_3.0 = _6;
_18.3 = [_18.0,_20,_20,_22,_18.0,_18.0];
_23 = [3373363160_u32,1356423307_u32,4235445684_u32];
_18.1 = _19.fld0;
_14 = _19.fld1;
_15.fld1 = _10.fld1;
_8.0 = _3.0 * _3.0;
_18.2 = (-7340183285011975314_i64) << _8.0;
_15.fld1 = _10.fld1;
match _2 {
0 => bb1,
1 => bb9,
2 => bb12,
8172 => bb14,
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
_25 = _8.3 >> _8.1;
_2 = (-5752_i16);
_10 = _15;
_9 = [13463446952219571262_u64,1824038019239735694_u64];
_3.2 = -_8.2;
_12.fld1 = _14;
_20 = _18.0;
_3.3 = _25 * _25;
_26 = [2895607935_u32,872164275_u32,946997445_u32];
_7 = 3911_u16 as f64;
_6 = _8.0;
_17 = _18.2 as i8;
_3.3 = _8.3 << RET;
_17 = _2 as i8;
_18.3 = [_20,_22,_22,_18.0,_18.0,_20];
_26 = [1963841760_u32,1540383227_u32,3608490151_u32];
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(2_usize, 20_usize, Move(_20), 25_usize, Move(_25), 2_usize, Move(_2), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(2_usize, 16_usize, Move(_16), 13_usize, Move(_13), 17_usize, Move(_17), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: f64,mut _2: i16,mut _3: i8,mut _4: f64,mut _5: i8,mut _6: u8,mut _7: i16,mut _8: [u64; 2],mut _9: f32) -> f64 {
mir! {
type RET = f64;
let _10: isize;
let _11: [u64; 7];
let _12: [u64; 7];
let _13: f64;
let _14: Adt45;
let _15: u128;
let _16: Adt44;
let _17: f64;
let _18: [bool; 6];
let _19: u128;
let _20: ();
let _21: ();
{
RET = _4 - _4;
_2 = -_7;
_1 = _6 as f64;
RET = _4 - _4;
RET = 297895969772092548336336167539946012143_u128 as f64;
_9 = _4 as f32;
_7 = _2;
RET = _1 - _4;
_4 = 1238872103_u32 as f64;
_2 = true as i16;
_6 = 302328837687260799239605368475540469692_u128 as u8;
_3 = _5;
RET = (-181871671_i32) as f64;
Call(_3 = core::intrinsics::bswap(_5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = -9223372036854775807_isize;
_8 = [44571734623169355_u64,10062494357683485351_u64];
_6 = _7 as u8;
_3 = '\u{273a6}' as i8;
_4 = RET + RET;
_5 = _10 as i8;
_7 = !_2;
_7 = !_2;
_1 = _4 - RET;
_6 = 58_u8 + 190_u8;
_11 = [6862272851935428438_u64,6333492599286517771_u64,16365767184609412167_u64,13739467340479454309_u64,4652047256509540153_u64,14839510526898972454_u64,17013040310550366388_u64];
RET = 7714895869166251316_i64 as f64;
_11 = [7952578888293488432_u64,6618151318466710362_u64,12519720759517769892_u64,11234685953281175650_u64,17381267098256552443_u64,1926239515979986036_u64,3988427967072432785_u64];
_7 = _2 | _2;
_2 = -_7;
_3 = _5 | _5;
RET = 44900209819265303806964476215570079863_u128 as f64;
RET = 34684644966078312140689946236939422183_u128 as f64;
_12 = [1665529421971586432_u64,15662661708276458415_u64,3322927284516700166_u64,11943874023647338545_u64,9214269749365338564_u64,17766132995415891359_u64,1512187447807954445_u64];
Call(_6 = fn4(_1, _12, _2, _1, _1, _10, RET, _1, _11, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = [17021479411292870469_u64,9732324089061214199_u64];
_3 = _5 << _6;
_12 = [13469032040099035007_u64,16190998180756210062_u64,447096584991801497_u64,7935238425144682969_u64,16268480040332342119_u64,13644071471659387653_u64,3025735356658696068_u64];
_6 = 191_u8 & 2_u8;
_10 = 9223372036854775807_isize - 9223372036854775807_isize;
_6 = 124_u8;
_6 = 150_u8 | 186_u8;
_10 = !30_isize;
_7 = _6 as i16;
_12 = _11;
Goto(bb3)
}
bb3 = {
_14.fld1 = [17713721826182630623_u64,17101425424454771718_u64];
Goto(bb4)
}
bb4 = {
_11 = [3721147052270432228_u64,17382187064535066570_u64,2177244550309882880_u64,8051181110987300550_u64,7447468503326087441_u64,16892606716594173937_u64,1068292355020118346_u64];
_15 = 6778579207808599659_usize as u128;
_9 = 10425_u16 as f32;
_13 = -_1;
_2 = _7;
_11 = _12;
_9 = (-85517982280064039603807762268826643338_i128) as f32;
_17 = RET + _1;
_6 = 108_u8 | 130_u8;
_6 = !188_u8;
_3 = (-28253433680105886075462870646943643060_i128) as i8;
_11 = [8974041556419661834_u64,561966826353143172_u64,2621745543173509639_u64,10418768924466169603_u64,11643246484465986933_u64,12170827438580986890_u64,7095603549613866194_u64];
_4 = _17 + _17;
_14 = Adt45 { fld0: _3,fld1: _8,fld2: _2 };
_2 = 3_usize as i16;
_12 = _11;
_11 = [12627325793196740032_u64,11932170172631424730_u64,3210883262789669767_u64,13251904821741462109_u64,3903471802112684872_u64,11580549208864439967_u64,10656178324642821366_u64];
RET = _4;
_1 = -RET;
_5 = -_3;
RET = _17;
_14.fld2 = _7;
_18 = [false,false,false,false,true,true];
Goto(bb5)
}
bb5 = {
_1 = _4 * RET;
_9 = _5 as f32;
_6 = 35_u8 + 99_u8;
_1 = -_4;
_14.fld0 = _3 - _5;
_18 = [false,false,true,true,true,false];
_8 = _14.fld1;
RET = _1;
_1 = _4;
_7 = _14.fld2 + _2;
_9 = 7414337180936853769_i64 as f32;
_13 = _4;
RET = _1 * _17;
_13 = -_1;
_5 = -_3;
_14 = Adt45 { fld0: _3,fld1: _8,fld2: _7 };
_11 = _12;
_14 = Adt45 { fld0: _3,fld1: _8,fld2: _7 };
_14.fld0 = _17 as i8;
Goto(bb6)
}
bb6 = {
Call(_20 = dump_var(3_usize, 8_usize, Move(_8), 5_usize, Move(_5), 18_usize, Move(_18), 6_usize, Move(_6)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_20 = dump_var(3_usize, 10_usize, Move(_10), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: f64,mut _2: [u64; 7],mut _3: i16,mut _4: f64,mut _5: f64,mut _6: isize,mut _7: f64,mut _8: f64,mut _9: [u64; 7],mut _10: i8) -> u8 {
mir! {
type RET = u8;
let _11: *const &'static f64;
let _12: bool;
let _13: i32;
let _14: u8;
let _15: (i8, i32, f64, i128);
let _16: [i16; 7];
let _17: Adt47;
let _18: bool;
let _19: usize;
let _20: Adt45;
let _21: [u64; 2];
let _22: *mut char;
let _23: [u16; 8];
let _24: bool;
let _25: u32;
let _26: ();
let _27: ();
{
_8 = _3 as f64;
_2 = [445725267952876372_u64,758467775175765174_u64,5505775699418359106_u64,15782725955688482170_u64,9608734208815831087_u64,12266567483171357779_u64,14758380627078285440_u64];
_3 = 48128_u16 as i16;
RET = !90_u8;
_5 = _3 as f64;
RET = 183_u8;
_12 = !false;
_10 = 3925175854_u32 as i8;
_12 = true;
_2 = _9;
_5 = _1 - _8;
_4 = -_5;
_4 = _7;
_10 = 11521347701600984221_usize as i8;
_13 = 88000487517677568621610403399038474519_i128 as i32;
RET = 238_u8 >> _3;
_13 = _3 as i32;
Goto(bb1)
}
bb1 = {
_13 = 904427603_i32;
_14 = RET;
RET = !_14;
_10 = (-53_i8);
_7 = -_1;
_13 = (-1352725812_i32);
_5 = -_1;
_7 = _8 * _1;
_15.3 = 9190129397236045328404754385965916948_i128;
_15.1 = !_13;
Call(_2 = fn5(_7, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = [10006484985078572366_u64,7818594943718069466_u64,13953741901619851460_u64,12893192242282140668_u64,2185535765652356277_u64,5210050794699220281_u64,1119797088029124115_u64];
_14 = RET & RET;
_12 = !true;
match _10 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
340282366920938463463374607431768211403 => bb9,
_ => bb8
}
}
bb3 = {
_13 = 904427603_i32;
_14 = RET;
RET = !_14;
_10 = (-53_i8);
_7 = -_1;
_13 = (-1352725812_i32);
_5 = -_1;
_7 = _8 * _1;
_15.3 = 9190129397236045328404754385965916948_i128;
_15.1 = !_13;
Call(_2 = fn5(_7, _3), ReturnTo(bb2), UnwindUnreachable())
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
_6 = (-50_isize);
_15.0 = _10 * _10;
_3 = 11759_i16;
_15.0 = _10 * _10;
_15.1 = _15.3 as i32;
_15.3 = !(-161337904336878053695978502857169225154_i128);
_14 = RET;
_3 = (-2873_i16) * (-28328_i16);
_6 = (-9223372036854775808_isize);
_17.fld0 = 6745480050129117970_u64 as f32;
_15.1 = _13;
_20.fld1 = [14272361672379903136_u64,7560444559729391642_u64];
_15.3 = 286985173826112718925881321282427108237_u128 as i128;
_13 = -_15.1;
_17.fld1 = '\u{f3f80}';
_8 = 13630612192300145707_u64 as f64;
_17.fld0 = 62294_u16 as f32;
_12 = !false;
_8 = _6 as f64;
match _15.1 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb5,
340282366920938463463374607430415485644 => bb11,
_ => bb10
}
}
bb10 = {
_2 = [10006484985078572366_u64,7818594943718069466_u64,13953741901619851460_u64,12893192242282140668_u64,2185535765652356277_u64,5210050794699220281_u64,1119797088029124115_u64];
_14 = RET & RET;
_12 = !true;
match _10 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
340282366920938463463374607431768211403 => bb9,
_ => bb8
}
}
bb11 = {
_3 = (-18767_i16) ^ 12588_i16;
_6 = (-9223372036854775808_isize);
_21 = _20.fld1;
_20.fld0 = _15.0 | _15.0;
_12 = !true;
_6 = (-9223372036854775808_isize);
_15 = (_20.fld0, _13, _1, (-29990895833319722414629880277136715075_i128));
_20.fld2 = 173269386347771463762175641089967636498_u128 as i16;
_17.fld1 = '\u{36a32}';
RET = !_14;
_15.1 = _13 ^ _13;
Goto(bb12)
}
bb12 = {
_6 = !(-53_isize);
_12 = !false;
RET = _17.fld1 as u8;
_17.fld0 = _14 as f32;
RET = _14 << _15.3;
_9 = _2;
_1 = _10 as f64;
Goto(bb13)
}
bb13 = {
Call(_26 = dump_var(4_usize, 12_usize, Move(_12), 9_usize, Move(_9), 10_usize, Move(_10), 6_usize, Move(_6)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: f64,mut _2: i16) -> [u64; 7] {
mir! {
type RET = [u64; 7];
let _3: f64;
let _4: Adt52;
let _5: Adt57;
let _6: char;
let _7: [u32; 3];
let _8: [i16; 7];
let _9: Adt44;
let _10: bool;
let _11: [i16; 7];
let _12: Adt51;
let _13: Adt51;
let _14: Adt55;
let _15: Adt45;
let _16: [u32; 8];
let _17: u8;
let _18: i16;
let _19: Adt47;
let _20: char;
let _21: *const u32;
let _22: Adt50;
let _23: isize;
let _24: [i16; 5];
let _25: [i16; 7];
let _26: isize;
let _27: u16;
let _28: isize;
let _29: char;
let _30: [i64; 8];
let _31: [i16; 5];
let _32: bool;
let _33: isize;
let _34: i8;
let _35: bool;
let _36: isize;
let _37: u128;
let _38: char;
let _39: [u32; 3];
let _40: u8;
let _41: i128;
let _42: f64;
let _43: [u64; 8];
let _44: ();
let _45: ();
{
_3 = -_1;
RET = [6641583381314499812_u64,12712055995070003579_u64,11400585791011319645_u64,1478605156438677288_u64,7981588323642999072_u64,4464192518095434423_u64,16754862454055258818_u64];
_5.fld1.1 = (-118_i8) as f32;
Goto(bb1)
}
bb1 = {
_5.fld4.fld1 = [523104568972893355_u64,8967446208762517465_u64];
_5.fld4.fld0 = !(-47_i8);
_5.fld4.fld2 = _2;
_5.fld2 = !52_isize;
Call(_5.fld1.2 = core::intrinsics::transmute(_5.fld2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5.fld6 = _5.fld4.fld2 as i64;
_5.fld1.0 = false;
_1 = 188636663867020120378532665321371962425_u128 as f64;
_6 = '\u{55f8}';
_10 = _5.fld1.0;
_5.fld4.fld0 = 189_u8 as i8;
_10 = !_5.fld1.0;
_10 = _5.fld1.0;
_5.fld4.fld0 = -(-121_i8);
_11 = [_2,_2,_5.fld4.fld2,_2,_5.fld4.fld2,_2,_2];
_5.fld2 = (-9223372036854775808_isize);
_8 = [_2,_2,_2,_5.fld4.fld2,_2,_5.fld4.fld2,_2];
_4 = Adt52::Variant0 { fld0: (-1036393845_i32) };
_4 = Adt52::Variant0 { fld0: 557502154_i32 };
_5.fld4.fld0 = -(-113_i8);
_10 = !_5.fld1.0;
_5.fld1.1 = (-2011346818_i32) as f32;
_5.fld1.3 = [_5.fld1.0,_10,_10,_10,_10,_10];
_7 = [883583629_u32,2910006274_u32,1089340553_u32];
place!(Field::<i32>(Variant(_4, 0), 0)) = -839501357_i32;
_6 = '\u{6deba}';
SetDiscriminant(_4, 1);
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld1 = _6;
place!(Field::<i64>(Variant(_4, 1), 4)) = _5.fld1.2;
Goto(bb3)
}
bb3 = {
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld5 = [14484_u16,23454_u16,43778_u16,9398_u16,64642_u16,36247_u16,62248_u16,1690_u16];
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld4.0 = !_5.fld1.0;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld4.0 = _5.fld1.1 <= _5.fld1.1;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld4.2 = Field::<i64>(Variant(_4, 1), 4);
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld4.3 = [Field::<Adt48>(Variant(_4, 1), 0).fld4.0,_5.fld1.0,_10,Field::<Adt48>(Variant(_4, 1), 0).fld4.0,_5.fld1.0,Field::<Adt48>(Variant(_4, 1), 0).fld4.0];
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld3 = -_5.fld4.fld0;
Goto(bb4)
}
bb4 = {
_5.fld1.3 = [Field::<Adt48>(Variant(_4, 1), 0).fld4.0,Field::<Adt48>(Variant(_4, 1), 0).fld4.0,_5.fld1.0,Field::<Adt48>(Variant(_4, 1), 0).fld4.0,Field::<Adt48>(Variant(_4, 1), 0).fld4.0,_5.fld1.0];
_5.fld1.0 = Field::<Adt48>(Variant(_4, 1), 0).fld4.0;
_8 = [_2,_5.fld4.fld2,_5.fld4.fld2,_5.fld4.fld2,_2,_5.fld4.fld2,_2];
_5.fld1.1 = 3055121437_u32 as f32;
place!(Field::<[u32; 8]>(Variant(_4, 1), 2)) = [3387464228_u32,2552055663_u32,1915274823_u32,1492464343_u32,544045485_u32,3669843032_u32,3845755257_u32,1425298730_u32];
_5.fld4.fld1 = [6112212932250054113_u64,15112739754094159303_u64];
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld2 = 21_u8 as u64;
_15.fld2 = _2 ^ _5.fld4.fld2;
_5.fld4.fld1 = [Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2];
_16 = Field::<[u32; 8]>(Variant(_4, 1), 2);
_5.fld4.fld2 = 2834735913_u32 as i16;
_5.fld0 = core::ptr::addr_of!(_5.fld2);
_1 = -_3;
_10 = Field::<Adt48>(Variant(_4, 1), 0).fld4.0 ^ _5.fld1.0;
_5.fld4.fld2 = _2;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld4.1 = _5.fld1.1 - _5.fld1.1;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld0.fld1 = [Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2];
_2 = _5.fld4.fld2;
_5.fld4.fld1 = [Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2];
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld6 = core::ptr::addr_of_mut!(RET);
place!(Field::<[u32; 8]>(Variant(_4, 1), 2)) = [3838752611_u32,2596161467_u32,4173330470_u32,4217447737_u32,2361666048_u32,370481878_u32,846854160_u32,3432653543_u32];
_5.fld1.1 = _3 as f32;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld0 = Adt45 { fld0: Field::<Adt48>(Variant(_4, 1), 0).fld3,fld1: _5.fld4.fld1,fld2: _15.fld2 };
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld4.2 = -_5.fld6;
_5.fld0 = core::ptr::addr_of!(_5.fld2);
_15 = Adt45 { fld0: Field::<Adt48>(Variant(_4, 1), 0).fld3,fld1: Field::<Adt48>(Variant(_4, 1), 0).fld0.fld1,fld2: _2 };
_17 = !247_u8;
_2 = -_15.fld2;
Goto(bb5)
}
bb5 = {
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld3 = Field::<Adt48>(Variant(_4, 1), 0).fld0.fld0 << _15.fld2;
_7 = [2015601630_u32,107996041_u32,52358622_u32];
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld4 = (_5.fld1.0, _5.fld1.1, _5.fld6, _5.fld1.3);
_16 = Field::<[u32; 8]>(Variant(_4, 1), 2);
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld0.fld1 = [Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2];
_5.fld4 = Adt45 { fld0: _15.fld0,fld1: _15.fld1,fld2: Field::<Adt48>(Variant(_4, 1), 0).fld0.fld2 };
_5.fld1 = Field::<Adt48>(Variant(_4, 1), 0).fld4;
_19 = Adt47 { fld0: _5.fld1.1,fld1: Field::<Adt48>(Variant(_4, 1), 0).fld1 };
_5.fld0 = core::ptr::addr_of!(_5.fld2);
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld4.1 = -_19.fld0;
_15.fld0 = -_5.fld4.fld0;
place!(Field::<i64>(Variant(_4, 1), 4)) = _5.fld1.2;
_5.fld0 = core::ptr::addr_of!(_5.fld2);
_17 = 161_u8 & 19_u8;
_15.fld0 = Field::<Adt48>(Variant(_4, 1), 0).fld3 + _5.fld4.fld0;
place!(Field::<Adt49>(Variant(_4, 1), 5)) = Adt49::Variant0 { fld0: (-297941210_i32) };
Goto(bb6)
}
bb6 = {
_6 = _19.fld1;
_20 = _6;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld4.0 = _5.fld1.1 > _19.fld0;
place!(Field::<char>(Variant(_4, 1), 1)) = _6;
_15.fld0 = !Field::<Adt48>(Variant(_4, 1), 0).fld0.fld0;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld4.2 = 35057_u16 as i64;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld4.0 = _10;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld0 = Adt45 { fld0: _15.fld0,fld1: _5.fld4.fld1,fld2: _15.fld2 };
_8 = _11;
RET = [Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2];
_3 = _1 - _1;
_5.fld2 = (-119_isize) & (-9223372036854775808_isize);
_19 = Adt47 { fld0: _5.fld1.1,fld1: _20 };
_7 = [3392250774_u32,785312013_u32,3711134081_u32];
_5.fld4.fld0 = Field::<Adt48>(Variant(_4, 1), 0).fld0.fld0;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld4.2 = _5.fld2 as i64;
_15 = _5.fld4;
_15.fld2 = 1213044623_u32 as i16;
_23 = -_5.fld2;
_16 = [4001782093_u32,3967175594_u32,1622506040_u32,1784398586_u32,2153236246_u32,2684819237_u32,3398419302_u32,3434790081_u32];
_1 = _3 - _3;
_19.fld1 = _20;
_5.fld1.1 = -_19.fld0;
Goto(bb7)
}
bb7 = {
place!(Field::<i64>(Variant(_4, 1), 4)) = Field::<Adt48>(Variant(_4, 1), 0).fld2 as i64;
_18 = _2;
_19 = Adt47 { fld0: Field::<Adt48>(Variant(_4, 1), 0).fld4.1,fld1: _20 };
_15 = Adt45 { fld0: Field::<Adt48>(Variant(_4, 1), 0).fld0.fld0,fld1: Field::<Adt48>(Variant(_4, 1), 0).fld0.fld1,fld2: _2 };
_22 = Adt50 { fld0: _18,fld1: _5.fld1.3 };
_15.fld0 = !Field::<Adt48>(Variant(_4, 1), 0).fld3;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld3 = _5.fld2 as i8;
_5.fld1.0 = _10;
_15.fld2 = _3 as i16;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld4.0 = !_5.fld1.0;
_11 = [Field::<Adt48>(Variant(_4, 1), 0).fld0.fld2,_15.fld2,_22.fld0,Field::<Adt48>(Variant(_4, 1), 0).fld0.fld2,_15.fld2,_15.fld2,_5.fld4.fld2];
_22 = Adt50 { fld0: _15.fld2,fld1: _5.fld1.3 };
Goto(bb8)
}
bb8 = {
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld4.0 = !_5.fld1.0;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld4 = (_10, _19.fld0, Field::<i64>(Variant(_4, 1), 4), _22.fld1);
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld4.0 = !_10;
RET = [Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2];
place!(Field::<[u32; 8]>(Variant(_4, 1), 2)) = [909541437_u32,2426300806_u32,3900814460_u32,3134261690_u32,1809990138_u32,940429411_u32,2886750948_u32,2785799662_u32];
Call(place!(Field::<i32>(Variant(place!(Field::<Adt49>(Variant(_4, 1), 5)), 0), 0)) = core::intrinsics::bswap((-727496501_i32)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_1 = _3 * _3;
Goto(bb10)
}
bb10 = {
_15 = Adt45 { fld0: _5.fld4.fld0,fld1: _5.fld4.fld1,fld2: _22.fld0 };
_27 = !26892_u16;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld3 = Field::<Adt48>(Variant(_4, 1), 0).fld0.fld0 & _15.fld0;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld4.3 = [_10,Field::<Adt48>(Variant(_4, 1), 0).fld4.0,_10,_5.fld1.0,Field::<Adt48>(Variant(_4, 1), 0).fld4.0,Field::<Adt48>(Variant(_4, 1), 0).fld4.0];
RET = [Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2];
_15.fld2 = !_22.fld0;
SetDiscriminant(Field::<Adt49>(Variant(_4, 1), 5), 1);
_22 = Adt50 { fld0: _15.fld2,fld1: Field::<Adt48>(Variant(_4, 1), 0).fld4.3 };
_5.fld4.fld0 = !Field::<Adt48>(Variant(_4, 1), 0).fld0.fld0;
_19 = Adt47 { fld0: _5.fld1.1,fld1: _20 };
_26 = !_23;
_29 = _20;
_31 = [_22.fld0,Field::<Adt48>(Variant(_4, 1), 0).fld0.fld2,_22.fld0,_15.fld2,_22.fld0];
place!(Field::<[u32; 8]>(Variant(place!(Field::<Adt49>(Variant(_4, 1), 5)), 1), 1)) = [2976299291_u32,1410035527_u32,1339187561_u32,1727746820_u32,2162954883_u32,3293939635_u32,2598516941_u32,1360826660_u32];
place!(Field::<*mut char>(Variant(place!(Field::<Adt49>(Variant(_4, 1), 5)), 1), 2)) = core::ptr::addr_of_mut!(_29);
_5.fld0 = core::ptr::addr_of!(_26);
_19.fld1 = _20;
place!(Field::<*mut char>(Variant(place!(Field::<Adt49>(Variant(_4, 1), 5)), 1), 2)) = core::ptr::addr_of_mut!(place!(Field::<Adt48>(Variant(_4, 1), 0)).fld1);
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld1 = Field::<char>(Variant(_4, 1), 1);
_27 = !54877_u16;
_28 = Field::<Adt48>(Variant(_4, 1), 0).fld2 as isize;
_17 = 30_u8;
_5.fld4.fld1 = [Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2];
_11 = _8;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld0.fld1 = [Field::<Adt48>(Variant(_4, 1), 0).fld2,Field::<Adt48>(Variant(_4, 1), 0).fld2];
Call(_15 = fn6(_17, Field::<Adt48>(Variant(_4, 1), 0).fld4.0, _7, Field::<char>(Variant(_4, 1), 1), Field::<Adt48>(Variant(_4, 1), 0).fld4.1, Field::<Adt48>(Variant(_4, 1), 0).fld5, _3, Field::<Adt48>(Variant(_4, 1), 0).fld5, Field::<Adt48>(Variant(_4, 1), 0).fld4.3, _1, _3, _22.fld0, Field::<[u32; 8]>(Variant(Field::<Adt49>(Variant(_4, 1), 5), 1), 1)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_26 = !_5.fld2;
place!(Field::<Adt47>(Variant(place!(Field::<Adt49>(Variant(_4, 1), 5)), 1), 3)) = Adt47 { fld0: _19.fld0,fld1: Field::<Adt48>(Variant(_4, 1), 0).fld1 };
_1 = _3 + _3;
_5.fld2 = Field::<Adt48>(Variant(_4, 1), 0).fld2 as isize;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld4.3 = [Field::<Adt48>(Variant(_4, 1), 0).fld4.0,_10,Field::<Adt48>(Variant(_4, 1), 0).fld4.0,_5.fld1.0,Field::<Adt48>(Variant(_4, 1), 0).fld4.0,_5.fld1.0];
place!(Field::<bool>(Variant(place!(Field::<Adt49>(Variant(_4, 1), 5)), 1), 0)) = _15.fld0 < _15.fld0;
place!(Field::<Adt49>(Variant(_4, 1), 5)) = Adt49::Variant0 { fld0: (-281721088_i32) };
_30 = [Field::<i64>(Variant(_4, 1), 4),_5.fld1.2,_5.fld1.2,_5.fld6,_5.fld6,Field::<i64>(Variant(_4, 1), 4),_5.fld6,Field::<i64>(Variant(_4, 1), 4)];
_6 = _20;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld0.fld1 = _5.fld4.fld1;
_22 = Adt50 { fld0: _15.fld2,fld1: _5.fld1.3 };
_5.fld4 = Adt45 { fld0: _15.fld0,fld1: Field::<Adt48>(Variant(_4, 1), 0).fld0.fld1,fld2: _15.fld2 };
match _17 {
30 => bb12,
_ => bb7
}
}
bb12 = {
_2 = !_15.fld2;
_22.fld0 = -_15.fld2;
_4 = Adt52::Variant0 { fld0: (-1931683243_i32) };
_19.fld0 = _5.fld6 as f32;
_32 = _10;
_22 = Adt50 { fld0: _5.fld4.fld2,fld1: _5.fld1.3 };
_5.fld4.fld0 = -_15.fld0;
_5.fld1.0 = !_32;
_26 = _17 as isize;
_19.fld0 = -_5.fld1.1;
_5.fld1.1 = 898595259_u32 as f32;
_5.fld4.fld1 = [182516406035919911_u64,11149882650168949516_u64];
_27 = 4938_u16 >> _17;
_37 = 164808610575085252753069548423263537838_u128;
_5.fld1 = (_10, _19.fld0, _5.fld6, _22.fld1);
_2 = (-1275182368_i32) as i16;
_19 = Adt47 { fld0: _5.fld1.1,fld1: _29 };
_27 = 53325_u16 & 55113_u16;
_10 = !_32;
_15.fld0 = _5.fld4.fld0;
match _17 {
0 => bb5,
30 => bb14,
_ => bb13
}
}
bb13 = {
_26 = !_5.fld2;
place!(Field::<Adt47>(Variant(place!(Field::<Adt49>(Variant(_4, 1), 5)), 1), 3)) = Adt47 { fld0: _19.fld0,fld1: Field::<Adt48>(Variant(_4, 1), 0).fld1 };
_1 = _3 + _3;
_5.fld2 = Field::<Adt48>(Variant(_4, 1), 0).fld2 as isize;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld4.3 = [Field::<Adt48>(Variant(_4, 1), 0).fld4.0,_10,Field::<Adt48>(Variant(_4, 1), 0).fld4.0,_5.fld1.0,Field::<Adt48>(Variant(_4, 1), 0).fld4.0,_5.fld1.0];
place!(Field::<bool>(Variant(place!(Field::<Adt49>(Variant(_4, 1), 5)), 1), 0)) = _15.fld0 < _15.fld0;
place!(Field::<Adt49>(Variant(_4, 1), 5)) = Adt49::Variant0 { fld0: (-281721088_i32) };
_30 = [Field::<i64>(Variant(_4, 1), 4),_5.fld1.2,_5.fld1.2,_5.fld6,_5.fld6,Field::<i64>(Variant(_4, 1), 4),_5.fld6,Field::<i64>(Variant(_4, 1), 4)];
_6 = _20;
place!(Field::<Adt48>(Variant(_4, 1), 0)).fld0.fld1 = _5.fld4.fld1;
_22 = Adt50 { fld0: _15.fld2,fld1: _5.fld1.3 };
_5.fld4 = Adt45 { fld0: _15.fld0,fld1: Field::<Adt48>(Variant(_4, 1), 0).fld0.fld1,fld2: _15.fld2 };
match _17 {
30 => bb12,
_ => bb7
}
}
bb14 = {
_27 = 36973_u16 - 54701_u16;
_34 = _5.fld4.fld0;
_11 = [_15.fld2,_15.fld2,_15.fld2,_15.fld2,_5.fld4.fld2,_15.fld2,_15.fld2];
_18 = !_15.fld2;
_30 = [_5.fld6,_5.fld1.2,_5.fld6,_5.fld1.2,_5.fld6,_5.fld6,_5.fld1.2,_5.fld1.2];
_5.fld1 = (_32, _19.fld0, _5.fld6, _22.fld1);
_5.fld4 = Adt45 { fld0: _15.fld0,fld1: _15.fld1,fld2: _18 };
_5.fld4.fld0 = _34 >> _34;
_35 = _34 <= _34;
_34 = -_5.fld4.fld0;
_41 = 61209686253729010771174434156397743053_i128 | 126680757853813285390898427179674986495_i128;
_17 = 235_u8 + 28_u8;
_5.fld3 = Adt54::Variant1 { fld0: _35,fld1: _30,fld2: _19 };
SetDiscriminant(_5.fld3, 1);
_15.fld1 = _5.fld4.fld1;
place!(Field::<i32>(Variant(_4, 0), 0)) = 1188766092_i32 ^ 1589692096_i32;
place!(Field::<Adt47>(Variant(_5.fld3, 1), 2)).fld1 = _6;
_22 = Adt50 { fld0: _5.fld4.fld2,fld1: _5.fld1.3 };
_20 = Field::<Adt47>(Variant(_5.fld3, 1), 2).fld1;
place!(Field::<Adt47>(Variant(_5.fld3, 1), 2)) = _19;
place!(Field::<[i64; 8]>(Variant(_5.fld3, 1), 1)) = [_5.fld6,_5.fld6,_5.fld1.2,_5.fld6,_5.fld6,_5.fld1.2,_5.fld6,_5.fld1.2];
_25 = [_15.fld2,_18,_18,_15.fld2,_5.fld4.fld2,_5.fld4.fld2,_5.fld4.fld2];
SetDiscriminant(_4, 2);
_28 = _29 as isize;
_32 = _35 >= _35;
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(5_usize, 11_usize, Move(_11), 8_usize, Move(_8), 27_usize, Move(_27), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(5_usize, 7_usize, Move(_7), 35_usize, Move(_35), 26_usize, Move(_26), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(5_usize, 41_usize, Move(_41), 20_usize, Move(_20), 32_usize, Move(_32), 45_usize, _45), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: u8,mut _2: bool,mut _3: [u32; 3],mut _4: char,mut _5: f32,mut _6: [u16; 8],mut _7: f64,mut _8: [u16; 8],mut _9: [bool; 6],mut _10: f64,mut _11: f64,mut _12: i16,mut _13: [u32; 8]) -> Adt45 {
mir! {
type RET = Adt45;
let _14: Adt47;
let _15: isize;
let _16: Adt54;
let _17: isize;
let _18: Adt59;
let _19: [i16; 5];
let _20: f64;
let _21: u8;
let _22: f32;
let _23: [char; 6];
let _24: [u64; 7];
let _25: [u64; 7];
let _26: [u32; 3];
let _27: [i16; 7];
let _28: *const &'static f64;
let _29: (bool, f32, i64, [bool; 6]);
let _30: [u32; 3];
let _31: u128;
let _32: [u64; 7];
let _33: ();
let _34: ();
{
_13 = [1004002572_u32,2319007240_u32,57249811_u32,2996099438_u32,2149017319_u32,2065863993_u32,3622752134_u32,3284115388_u32];
_12 = 10706_i16;
_11 = _10 - _7;
RET.fld0 = 6_i8 & (-87_i8);
RET.fld1 = [11695588860726567813_u64,764500178557848854_u64];
_9 = [_2,_2,_2,_2,_2,_2];
RET.fld1 = [7715954374147437041_u64,5148842984761379057_u64];
_13 = [3970130490_u32,448985017_u32,2629446716_u32,282198415_u32,1080692319_u32,2356965391_u32,3711789098_u32,4224547249_u32];
RET.fld1 = [1888458626593124370_u64,17166255249471478121_u64];
RET.fld2 = _12 ^ _12;
_14.fld1 = _4;
Call(_3 = fn7(_11, RET, _10, _10, _11, _10, _11, _10, _10, _10, _10, _11, _13, _11), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = 2894526081846126692050802376098887280_u128 as f64;
_14.fld1 = _4;
_7 = -_11;
_3 = [1933072627_u32,1436459892_u32,3202825900_u32];
_4 = _14.fld1;
_14.fld0 = _7 as f32;
RET.fld0 = 86_i8;
RET.fld2 = !_12;
_14.fld1 = _4;
_14.fld0 = _5;
RET.fld2 = RET.fld0 as i16;
RET.fld1 = [17095487407627240880_u64,13598251179232647090_u64];
_6 = [36736_u16,22050_u16,48279_u16,47787_u16,43737_u16,44028_u16,64804_u16,52873_u16];
_14.fld1 = _4;
_10 = 1642830450_u32 as f64;
_6 = _8;
_14.fld0 = 6_usize as f32;
RET.fld2 = _12 + _12;
_10 = _7;
_14 = Adt47 { fld0: _5,fld1: _4 };
_10 = -_11;
RET.fld0 = 14_i8 + (-29_i8);
RET.fld1 = [17764204268072738575_u64,6270020382140480741_u64];
Goto(bb2)
}
bb2 = {
_14.fld1 = _4;
_3 = [1488031972_u32,2449435301_u32,1765954053_u32];
_3 = [33549878_u32,723746876_u32,1072295687_u32];
_8 = [65460_u16,62801_u16,42533_u16,8739_u16,22652_u16,5986_u16,36973_u16,18515_u16];
_14 = Adt47 { fld0: _5,fld1: _4 };
_5 = _14.fld0 + _14.fld0;
_13 = [2407768958_u32,1568533960_u32,375762284_u32,2613162067_u32,1927703069_u32,821836609_u32,1832180039_u32,127068877_u32];
RET.fld2 = _12;
_5 = _14.fld0;
RET.fld1 = [1027386576103525495_u64,10606488735065489549_u64];
RET.fld0 = 20_i8;
_12 = RET.fld2 | RET.fld2;
RET.fld1 = [15144757436869573088_u64,6930582171037487587_u64];
RET.fld0 = 103_i8 << _12;
_14.fld0 = _5 + _5;
_14 = Adt47 { fld0: _5,fld1: _4 };
Goto(bb3)
}
bb3 = {
_7 = _10 - _11;
_5 = 40313_u16 as f32;
_14 = Adt47 { fld0: _5,fld1: _4 };
_14.fld1 = _4;
Goto(bb4)
}
bb4 = {
_9 = [_2,_2,_2,_2,_2,_2];
_9 = [_2,_2,_2,_2,_2,_2];
Goto(bb5)
}
bb5 = {
_12 = !RET.fld2;
_12 = -RET.fld2;
_9 = [_2,_2,_2,_2,_2,_2];
_18.fld2.fld1 = [_2,_2,_2,_2,_2,_2];
_17 = (-9223372036854775808_isize);
_18.fld0.fld0 = -RET.fld0;
_13 = [2578841894_u32,3081722317_u32,1640970180_u32,2224542604_u32,2711014253_u32,2530792660_u32,1697417643_u32,2370655301_u32];
_11 = _7 + _7;
_9 = [_2,_2,_2,_2,_2,_2];
_18.fld2.fld0 = _14.fld1 as i16;
RET.fld0 = !_18.fld0.fld0;
RET.fld0 = _18.fld0.fld0;
_18.fld1 = !222870929_u32;
_14.fld1 = _4;
_18.fld0 = Adt45 { fld0: RET.fld0,fld1: RET.fld1,fld2: _12 };
_18.fld4.1 = _14.fld0;
_14 = Adt47 { fld0: _18.fld4.1,fld1: _4 };
_7 = (-1429677162_i32) as f64;
_18.fld4.0 = _11 >= _10;
RET.fld2 = _18.fld2.fld0 >> RET.fld0;
_18.fld0.fld0 = RET.fld0;
Goto(bb6)
}
bb6 = {
_18.fld0.fld0 = RET.fld0;
_18.fld2.fld1 = _9;
_15 = _1 as isize;
_13 = [_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1];
_6 = [19495_u16,6585_u16,48346_u16,57276_u16,3134_u16,37863_u16,15339_u16,22882_u16];
_5 = -_18.fld4.1;
_15 = -_17;
_13 = [_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1];
_18.fld4.0 = _2;
_8 = [56913_u16,10811_u16,5900_u16,54727_u16,11765_u16,417_u16,43752_u16,26389_u16];
_18.fld4 = (_2, _5, 6051690098220674967_i64, _18.fld2.fld1);
_18.fld2 = Adt50 { fld0: RET.fld2,fld1: _18.fld4.3 };
_15 = _17;
_18.fld4 = (_2, _14.fld0, 6436995370770297961_i64, _9);
_5 = _14.fld0;
match _17 {
0 => bb3,
340282366920938463454151235394913435648 => bb8,
_ => bb7
}
}
bb7 = {
_12 = !RET.fld2;
_12 = -RET.fld2;
_9 = [_2,_2,_2,_2,_2,_2];
_18.fld2.fld1 = [_2,_2,_2,_2,_2,_2];
_17 = (-9223372036854775808_isize);
_18.fld0.fld0 = -RET.fld0;
_13 = [2578841894_u32,3081722317_u32,1640970180_u32,2224542604_u32,2711014253_u32,2530792660_u32,1697417643_u32,2370655301_u32];
_11 = _7 + _7;
_9 = [_2,_2,_2,_2,_2,_2];
_18.fld2.fld0 = _14.fld1 as i16;
RET.fld0 = !_18.fld0.fld0;
RET.fld0 = _18.fld0.fld0;
_18.fld1 = !222870929_u32;
_14.fld1 = _4;
_18.fld0 = Adt45 { fld0: RET.fld0,fld1: RET.fld1,fld2: _12 };
_18.fld4.1 = _14.fld0;
_14 = Adt47 { fld0: _18.fld4.1,fld1: _4 };
_7 = (-1429677162_i32) as f64;
_18.fld4.0 = _11 >= _10;
RET.fld2 = _18.fld2.fld0 >> RET.fld0;
_18.fld0.fld0 = RET.fld0;
Goto(bb6)
}
bb8 = {
_18.fld7 = (-86064621648687143630487838726469412070_i128) & 62514987052024459747917422826376994901_i128;
_18.fld4 = (_2, _5, 5629648891449658596_i64, _9);
_18.fld0 = Adt45 { fld0: RET.fld0,fld1: RET.fld1,fld2: RET.fld2 };
Goto(bb9)
}
bb9 = {
_22 = _18.fld0.fld0 as f32;
_18.fld0.fld2 = !_18.fld2.fld0;
_5 = 5297620989431505651_usize as f32;
_9 = [_18.fld4.0,_18.fld4.0,_18.fld4.0,_18.fld4.0,_18.fld4.0,_18.fld4.0];
_21 = !_1;
_18.fld0.fld1 = [5856811031157588039_u64,14570638478115681570_u64];
_19 = [RET.fld2,RET.fld2,_18.fld2.fld0,_18.fld0.fld2,RET.fld2];
_1 = _21;
_14.fld0 = _5;
_27 = [_18.fld0.fld2,_18.fld2.fld0,_18.fld0.fld2,_18.fld0.fld2,_18.fld2.fld0,RET.fld2,RET.fld2];
match _18.fld4.2 {
0 => bb10,
5629648891449658596 => bb12,
_ => bb11
}
}
bb10 = {
_7 = _10 - _11;
_5 = 40313_u16 as f32;
_14 = Adt47 { fld0: _5,fld1: _4 };
_14.fld1 = _4;
Goto(bb4)
}
bb11 = {
_12 = !RET.fld2;
_12 = -RET.fld2;
_9 = [_2,_2,_2,_2,_2,_2];
_18.fld2.fld1 = [_2,_2,_2,_2,_2,_2];
_17 = (-9223372036854775808_isize);
_18.fld0.fld0 = -RET.fld0;
_13 = [2578841894_u32,3081722317_u32,1640970180_u32,2224542604_u32,2711014253_u32,2530792660_u32,1697417643_u32,2370655301_u32];
_11 = _7 + _7;
_9 = [_2,_2,_2,_2,_2,_2];
_18.fld2.fld0 = _14.fld1 as i16;
RET.fld0 = !_18.fld0.fld0;
RET.fld0 = _18.fld0.fld0;
_18.fld1 = !222870929_u32;
_14.fld1 = _4;
_18.fld0 = Adt45 { fld0: RET.fld0,fld1: RET.fld1,fld2: _12 };
_18.fld4.1 = _14.fld0;
_14 = Adt47 { fld0: _18.fld4.1,fld1: _4 };
_7 = (-1429677162_i32) as f64;
_18.fld4.0 = _11 >= _10;
RET.fld2 = _18.fld2.fld0 >> RET.fld0;
_18.fld0.fld0 = RET.fld0;
Goto(bb6)
}
bb12 = {
_18.fld0.fld2 = _18.fld4.2 as i16;
_8 = _6;
_3 = [_18.fld1,_18.fld1,_18.fld1];
_24 = [10022620756887339548_u64,16674273373101234989_u64,1071320429593911643_u64,16240202451107374695_u64,6767784564731613901_u64,4304459082912385930_u64,9061170655421436968_u64];
_26 = _3;
_26 = [_18.fld1,_18.fld1,_18.fld1];
_18.fld1 = !3888941098_u32;
_7 = -_11;
_17 = 62468_u16 as isize;
RET = Adt45 { fld0: _18.fld0.fld0,fld1: _18.fld0.fld1,fld2: _18.fld0.fld2 };
_18.fld2.fld1 = [_2,_2,_2,_18.fld4.0,_2,_18.fld4.0];
_4 = _14.fld1;
_18.fld2.fld1 = [_2,_18.fld4.0,_18.fld4.0,_18.fld4.0,_2,_18.fld4.0];
_24 = [9362101995666794572_u64,17576549572478566090_u64,1141059257400930525_u64,7836743114156489807_u64,8930377808668269589_u64,8865201983206842442_u64,15404300862629131893_u64];
_14.fld0 = _17 as f32;
_18.fld1 = 2279535563_u32;
_18.fld0.fld2 = RET.fld2 * RET.fld2;
_13 = [_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1];
_14.fld0 = _5;
_18.fld0.fld0 = _7 as i8;
_18.fld1 = 582300220_u32 + 2909328331_u32;
Goto(bb13)
}
bb13 = {
_13 = [_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1];
RET.fld0 = _18.fld0.fld0;
_18.fld4.0 = !_2;
RET.fld1 = [4623135756437315956_u64,13925546868752315486_u64];
_25 = _24;
Goto(bb14)
}
bb14 = {
Call(_33 = dump_var(6_usize, 15_usize, Move(_15), 4_usize, Move(_4), 13_usize, Move(_13), 17_usize, Move(_17)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_33 = dump_var(6_usize, 27_usize, Move(_27), 2_usize, Move(_2), 6_usize, Move(_6), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: f64,mut _2: Adt45,mut _3: f64,mut _4: f64,mut _5: f64,mut _6: f64,mut _7: f64,mut _8: f64,mut _9: f64,mut _10: f64,mut _11: f64,mut _12: f64,mut _13: [u32; 8],mut _14: f64) -> [u32; 3] {
mir! {
type RET = [u32; 3];
let _15: Adt48;
let _16: f32;
let _17: Adt53;
let _18: Adt59;
let _19: Adt54;
let _20: u128;
let _21: Adt50;
let _22: [u64; 8];
let _23: i32;
let _24: isize;
let _25: [i64; 8];
let _26: isize;
let _27: [i16; 7];
let _28: [u32; 8];
let _29: Adt58;
let _30: f32;
let _31: f32;
let _32: (f64,);
let _33: [u64; 8];
let _34: Adt60;
let _35: isize;
let _36: (f64,);
let _37: bool;
let _38: i128;
let _39: [i16; 7];
let _40: (bool, f32, i64, [bool; 6]);
let _41: [u32; 3];
let _42: ();
let _43: ();
{
_2.fld1 = [7366142645014458302_u64,8639483575952192667_u64];
_9 = _12;
_15.fld4.0 = false ^ false;
_5 = 5649956130012786900_i64 as f64;
_3 = _6 + _1;
_15.fld4.2 = !(-296335337976257166_i64);
_15.fld4.2 = (-1669629593403780889_i64) >> _2.fld2;
_2.fld0 = (-72_i8);
_2.fld0 = 122_i8;
_15.fld5 = [65264_u16,18900_u16,6680_u16,55720_u16,58140_u16,31209_u16,59224_u16,58761_u16];
_13 = [663330689_u32,863475320_u32,2463174606_u32,317050562_u32,155673876_u32,2378170531_u32,559354162_u32,1713935387_u32];
_8 = _11 + _7;
_14 = _10;
_2.fld0 = -57_i8;
RET = [1013873649_u32,1346240211_u32,1183550452_u32];
_7 = 3941574849_u32 as f64;
_7 = _1 * _11;
_15.fld4.1 = _15.fld4.2 as f32;
RET = [3166424767_u32,2961559523_u32,3756256995_u32];
_18.fld2.fld1 = [_15.fld4.0,_15.fld4.0,_15.fld4.0,_15.fld4.0,_15.fld4.0,_15.fld4.0];
Goto(bb1)
}
bb1 = {
_18.fld4.1 = _15.fld4.1;
_6 = _15.fld4.2 as f64;
_15.fld4.0 = !false;
_18.fld2.fld1 = [_15.fld4.0,_15.fld4.0,_15.fld4.0,_15.fld4.0,_15.fld4.0,_15.fld4.0];
_9 = _12;
_15.fld4 = (true, _18.fld4.1, 3971947441307978312_i64, _18.fld2.fld1);
_18.fld1 = !2073038852_u32;
_7 = 174_u8 as f64;
_15.fld0 = _2;
_18.fld4 = _15.fld4;
_4 = -_12;
_18.fld2.fld0 = _2.fld2;
_15.fld0.fld0 = -_2.fld0;
_10 = _2.fld2 as f64;
_5 = -_8;
_8 = _14;
_20 = !110474261125523795481273025257175558878_u128;
_2.fld2 = _20 as i16;
_15.fld2 = 29452_u16 as u64;
_18.fld4.1 = _15.fld4.1;
_18.fld0.fld2 = 31187_u16 as i16;
match _18.fld4.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
3971947441307978312 => bb7,
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
_15.fld4.3 = _18.fld4.3;
_3 = -_5;
Call(_15.fld1 = fn8(_14, _15.fld0.fld0, Move(_18.fld2), _18.fld4.0, _13, _8, _5, _14, _12, _8, _3), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_7 = 109_u8 as f64;
_18.fld4.1 = _15.fld4.1 * _15.fld4.1;
_18.fld2.fld0 = _15.fld0.fld2 | _15.fld0.fld2;
_18.fld7 = !126899775662284533432853898927949312404_i128;
_9 = _18.fld1 as f64;
_18.fld2.fld1 = [_15.fld4.0,_15.fld4.0,_15.fld4.0,_18.fld4.0,_18.fld4.0,_15.fld4.0];
_15.fld4.2 = !_18.fld4.2;
_10 = -_3;
_2 = Adt45 { fld0: _15.fld0.fld0,fld1: _15.fld0.fld1,fld2: _18.fld2.fld0 };
_15.fld4 = _18.fld4;
_2.fld1 = _15.fld0.fld1;
_18.fld7 = _15.fld1 as i128;
_18.fld0.fld2 = _18.fld2.fld0;
_4 = _8;
_10 = -_8;
_21.fld1 = [_18.fld4.0,_15.fld4.0,_15.fld4.0,_15.fld4.0,_15.fld4.0,_15.fld4.0];
_18.fld0.fld1 = [_15.fld2,_15.fld2];
_22 = [_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2];
_21 = Move(_18.fld2);
_18.fld0.fld2 = _15.fld1 as i16;
_21.fld1 = [_18.fld4.0,_18.fld4.0,_18.fld4.0,_18.fld4.0,_18.fld4.0,_18.fld4.0];
match _18.fld4.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb7,
5 => bb6,
3971947441307978312 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_21.fld1 = [_15.fld4.0,_15.fld4.0,_15.fld4.0,_18.fld4.0,_18.fld4.0,_18.fld4.0];
_3 = _5;
_2.fld2 = _15.fld0.fld2 & _21.fld0;
_18.fld0.fld1 = _2.fld1;
_23 = 1504708252_i32 ^ (-1167082010_i32);
RET = [_18.fld1,_18.fld1,_18.fld1];
_21.fld0 = _2.fld2 >> _18.fld1;
_12 = _18.fld1 as f64;
RET = [_18.fld1,_18.fld1,_18.fld1];
_6 = _8 + _4;
_24 = !9223372036854775807_isize;
_15.fld0.fld1 = _18.fld0.fld1;
_15.fld4.0 = _18.fld4.0;
_15.fld4.3 = [_15.fld4.0,_18.fld4.0,_15.fld4.0,_15.fld4.0,_18.fld4.0,_18.fld4.0];
_2.fld1 = [_15.fld2,_15.fld2];
_2 = Adt45 { fld0: _15.fld0.fld0,fld1: _15.fld0.fld1,fld2: _21.fld0 };
_4 = _14 * _14;
_1 = _15.fld0.fld0 as f64;
_15.fld0 = Adt45 { fld0: _2.fld0,fld1: _18.fld0.fld1,fld2: _21.fld0 };
_24 = !(-9223372036854775808_isize);
Goto(bb11)
}
bb11 = {
_8 = 42724_u16 as f64;
_25 = [_15.fld4.2,_15.fld4.2,_15.fld4.2,_15.fld4.2,_15.fld4.2,_18.fld4.2,_18.fld4.2,_18.fld4.2];
_26 = _21.fld0 as isize;
_23 = 1083444359_i32 & (-74081417_i32);
_25 = [_18.fld4.2,_18.fld4.2,_15.fld4.2,_18.fld4.2,_15.fld4.2,_18.fld4.2,_18.fld4.2,_15.fld4.2];
_18.fld0 = _15.fld0;
_15.fld0 = _2;
Goto(bb12)
}
bb12 = {
_18.fld2 = Adt50 { fld0: _18.fld0.fld2,fld1: _15.fld4.3 };
_18.fld2.fld0 = -_21.fld0;
_18.fld0.fld1 = _15.fld0.fld1;
_14 = 0_usize as f64;
_21.fld0 = _18.fld2.fld0;
_15.fld5 = [24436_u16,62047_u16,45386_u16,23676_u16,34144_u16,9394_u16,13452_u16,49107_u16];
_15.fld5 = [11097_u16,32520_u16,16289_u16,30048_u16,2336_u16,18750_u16,61584_u16,31045_u16];
_32 = (_10,);
_18.fld0.fld0 = _15.fld0.fld0;
_28 = [_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1,_18.fld1];
Goto(bb13)
}
bb13 = {
_32 = (_3,);
_18.fld2 = Adt50 { fld0: _2.fld2,fld1: _15.fld4.3 };
_15.fld0.fld0 = _15.fld1 as i8;
_31 = -_18.fld4.1;
_18.fld0.fld0 = _18.fld1 as i8;
_18.fld0.fld0 = _18.fld4.0 as i8;
_18.fld2.fld1 = [_18.fld4.0,_15.fld4.0,_15.fld4.0,_15.fld4.0,_15.fld4.0,_15.fld4.0];
_33 = [_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2,_15.fld2];
_16 = _18.fld4.1 * _15.fld4.1;
_5 = _6;
RET = [_18.fld1,_18.fld1,_18.fld1];
_5 = -_32.0;
match _15.fld4.2 {
0 => bb9,
1 => bb2,
3971947441307978312 => bb14,
_ => bb3
}
}
bb14 = {
_15.fld4.1 = -_31;
_15.fld4.0 = !_18.fld4.0;
_31 = 147_u8 as f32;
_15.fld3 = _15.fld0.fld0;
_9 = _6;
RET = [_18.fld1,_18.fld1,_18.fld1];
_30 = _20 as f32;
_2.fld1 = [_15.fld2,_15.fld2];
_18.fld2 = Adt50 { fld0: _2.fld2,fld1: _21.fld1 };
_28 = _13;
_15.fld1 = '\u{19d50}';
_15.fld4.1 = _16;
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(7_usize, 22_usize, Move(_22), 25_usize, Move(_25), 23_usize, Move(_23), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: f64,mut _2: i8,mut _3: Adt50,mut _4: bool,mut _5: [u32; 8],mut _6: f64,mut _7: f64,mut _8: f64,mut _9: f64,mut _10: f64,mut _11: f64) -> char {
mir! {
type RET = char;
let _12: u128;
let _13: (i16, [bool; 6], &'static f64);
let _14: (f64,);
let _15: i128;
let _16: f32;
let _17: i8;
let _18: *const u32;
let _19: Adt48;
let _20: [u16; 8];
let _21: isize;
let _22: i128;
let _23: f32;
let _24: Adt47;
let _25: i32;
let _26: u16;
let _27: isize;
let _28: [i16; 7];
let _29: isize;
let _30: (bool, f32, i64, [bool; 6]);
let _31: Adt44;
let _32: f64;
let _33: [char; 6];
let _34: [u64; 7];
let _35: [u16; 8];
let _36: f32;
let _37: isize;
let _38: [u32; 8];
let _39: [i16; 7];
let _40: isize;
let _41: [u16; 8];
let _42: ();
let _43: ();
{
_1 = _9;
_11 = (-824347886_i32) as f64;
_10 = -_1;
_1 = 16844033613611673538_u64 as f64;
RET = '\u{933a8}';
_6 = _9;
_7 = _10;
_2 = (-18196105915984306982683164336050646865_i128) as i8;
_2 = -21_i8;
_5 = [590730021_u32,2251051008_u32,2128503774_u32,1898154932_u32,2477237424_u32,2300296703_u32,3257521011_u32,663993582_u32];
_4 = _7 > _9;
_6 = 250_u8 as f64;
_8 = -_7;
RET = '\u{70f3e}';
_7 = -_10;
_9 = -_7;
_3.fld1 = [_4,_4,_4,_4,_4,_4];
RET = '\u{bca92}';
_13.1 = _3.fld1;
RET = '\u{543a2}';
_12 = 213427390489169200486679755996331656456_u128 >> _3.fld0;
_3.fld0 = 22580_i16;
_13.1 = _3.fld1;
_13.2 = &_11;
_11 = -_10;
_5 = [4263411521_u32,2145392813_u32,1099809510_u32,831387407_u32,1963556477_u32,2470524618_u32,1711027290_u32,3713008196_u32];
_3.fld0 = 25863_i16;
_10 = _9 - _11;
_3.fld1 = [_4,_4,_4,_4,_4,_4];
Goto(bb1)
}
bb1 = {
_13.2 = &_11;
_10 = _8 + _9;
RET = '\u{106b4e}';
_1 = (-18489388984653973648417748371182476237_i128) as f64;
match _3.fld0 {
25863 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_13.2 = &_11;
_11 = -_10;
_6 = _7 + _10;
_1 = -_6;
_6 = _1 - _11;
_10 = 90617834678278962302999479181400195680_i128 as f64;
_13.0 = _3.fld0;
_11 = 449523854_u32 as f64;
_14 = (_9,);
_15 = 46138462997059788619543420611921563775_i128 & 108659818950946256464503120198129780878_i128;
_7 = _6 + _1;
_14.0 = -_8;
_7 = _9 * _6;
_7 = -_1;
Call(RET = fn9(_14, _4, _8, Move(_3), _4, _7, _13.1, _1, _6, _1, _7, _1, _14, _6, _6), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_3.fld0 = _13.0;
_13.0 = _3.fld0 + _3.fld0;
_1 = -_7;
_13.0 = (-1913069381_i32) as i16;
_13.0 = 57694_u16 as i16;
_13.2 = &_8;
_19.fld4.2 = (-6337971219058098499_i64) | 5867749933288213783_i64;
_15 = (-91276486327606518213970659508959453226_i128) + (-142050609885978095235342407946274875588_i128);
_19.fld0.fld2 = _3.fld0;
_1 = _6;
_15 = !48290805360151361855706095446852173419_i128;
_11 = _8 - _1;
_12 = RET as u128;
_13.2 = &_9;
_19.fld1 = RET;
_3.fld1 = [_4,_4,_4,_4,_4,_4];
match _3.fld0 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb7,
25863 => bb9,
_ => bb8
}
}
bb5 = {
_13.2 = &_11;
_11 = -_10;
_6 = _7 + _10;
_1 = -_6;
_6 = _1 - _11;
_10 = 90617834678278962302999479181400195680_i128 as f64;
_13.0 = _3.fld0;
_11 = 449523854_u32 as f64;
_14 = (_9,);
_15 = 46138462997059788619543420611921563775_i128 & 108659818950946256464503120198129780878_i128;
_7 = _6 + _1;
_14.0 = -_8;
_7 = _9 * _6;
_7 = -_1;
Call(RET = fn9(_14, _4, _8, Move(_3), _4, _7, _13.1, _1, _6, _1, _7, _1, _14, _6, _6), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
_13.2 = &_11;
_10 = _8 + _9;
RET = '\u{106b4e}';
_1 = (-18489388984653973648417748371182476237_i128) as f64;
match _3.fld0 {
25863 => bb3,
_ => bb2
}
}
bb8 = {
Return()
}
bb9 = {
_19.fld0.fld1 = [11904783857273770190_u64,9464905584289502391_u64];
_19.fld0.fld2 = _13.0 & _3.fld0;
_19.fld0.fld2 = !_13.0;
_22 = _15 | _15;
_19.fld5 = [24318_u16,8892_u16,52076_u16,56288_u16,16826_u16,32364_u16,20646_u16,6319_u16];
_19.fld0.fld0 = !_2;
_7 = _11;
_4 = _7 <= _6;
_19.fld3 = _19.fld0.fld0 & _2;
_25 = 21511_u16 as i32;
_19.fld0.fld2 = _19.fld3 as i16;
_10 = _1 + _11;
_3.fld1 = _13.1;
_23 = 46280_u16 as f32;
_19.fld0.fld0 = -_2;
_24.fld1 = _19.fld1;
_3.fld0 = _13.0 ^ _19.fld0.fld2;
_22 = 8302009924145938634_u64 as i128;
_19.fld4.3 = _3.fld1;
Call(_19.fld0.fld0 = core::intrinsics::bswap(_2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_21 = !9223372036854775807_isize;
_19.fld4.0 = !_4;
_12 = 36357622136635217661239957197254218458_u128 | 69568614703033564379215092156168233616_u128;
Goto(bb11)
}
bb11 = {
_24 = Adt47 { fld0: _23,fld1: RET };
_20 = [63171_u16,32901_u16,19845_u16,35828_u16,35630_u16,55252_u16,37816_u16,5711_u16];
_19.fld4.2 = -3951948924342401416_i64;
_13.1 = [_4,_4,_19.fld4.0,_19.fld4.0,_4,_4];
_19.fld4.1 = _23 * _24.fld0;
_2 = _19.fld0.fld0;
_19.fld4.1 = _24.fld0 + _23;
_6 = _7 - _10;
_17 = _19.fld3;
_26 = 48154_u16 + 4764_u16;
_19.fld4 = (_4, _24.fld0, 3432872999112148319_i64, _3.fld1);
_30.1 = _23;
_19.fld0.fld2 = !_3.fld0;
_13.2 = &_11;
_30.3 = [_19.fld4.0,_4,_19.fld4.0,_4,_4,_4];
_23 = 7166036273391680290_usize as f32;
_19.fld0.fld0 = _13.0 as i8;
_30.0 = !_4;
_17 = -_2;
_19.fld5 = _20;
_13.2 = &_6;
_27 = _21;
_19.fld4.3 = [_19.fld4.0,_19.fld4.0,_30.0,_30.0,_30.0,_30.0];
_30.1 = _23;
_1 = -_11;
_19.fld4.2 = 3386753929308147904_u64 as i64;
RET = _24.fld1;
_24 = Adt47 { fld0: _23,fld1: RET };
Goto(bb12)
}
bb12 = {
_23 = -_30.1;
_23 = -_19.fld4.1;
_19.fld1 = RET;
_19.fld4 = (_4, _23, 5703988416247066491_i64, _13.1);
_19.fld4.2 = -(-1033343263119402688_i64);
_16 = -_30.1;
_19.fld3 = _17;
_29 = !_21;
_12 = !143834708244919197530436833465956042511_u128;
_29 = !_21;
_29 = _27 + _27;
_15 = _26 as i128;
_19.fld4 = (_4, _24.fld0, (-6126803725426443252_i64), _13.1);
_7 = _25 as f64;
_12 = !43879830207078108802438954509805269395_u128;
_16 = _23;
_14.0 = -_1;
_10 = _19.fld3 as f64;
_34 = [8751702001910422060_u64,17183492614909258024_u64,19175889170714408_u64,1449133975231976745_u64,12113236977126766887_u64,15774226769309480933_u64,10250568759545196684_u64];
_26 = !49547_u16;
_19.fld4.3 = [_4,_4,_4,_19.fld4.0,_30.0,_19.fld4.0];
_2 = _19.fld0.fld0 + _17;
_36 = -_23;
Goto(bb13)
}
bb13 = {
_19.fld6 = core::ptr::addr_of_mut!(_34);
_15 = _22;
_34 = [8792914849607719548_u64,15883094052665923078_u64,8546259041603810555_u64,16157949054763366490_u64,15670139003709346140_u64,11307593325348030343_u64,12348633074526871645_u64];
_35 = [_26,_26,_26,_26,_26,_26,_26,_26];
_26 = 210_u8 as u16;
_32 = _14.0 * _8;
_35 = _20;
_13.1 = _30.3;
_16 = -_19.fld4.1;
_19.fld4.3 = _3.fld1;
_28 = [_3.fld0,_19.fld0.fld2,_19.fld0.fld2,_19.fld0.fld2,_19.fld0.fld2,_19.fld0.fld2,_3.fld0];
_19.fld0.fld0 = -_2;
_19.fld0.fld2 = _3.fld0 & _13.0;
_16 = _36;
_27 = !_21;
_19.fld0.fld1 = [4117839782930265277_u64,15841465806779134032_u64];
_34 = [11648051377287566194_u64,10898849544122893811_u64,10929089372233840054_u64,15047210125808100180_u64,10251496633697776160_u64,16440015239994373402_u64,13076018386091775818_u64];
_32 = -_1;
_19.fld4.2 = _19.fld4.0 as i64;
_30 = _19.fld4;
_15 = -_22;
_36 = _6 as f32;
_13.0 = _25 as i16;
_30.3 = [_4,_19.fld4.0,_30.0,_4,_30.0,_4];
_19.fld0.fld0 = 3647165593_u32 as i8;
Call(_2 = core::intrinsics::bswap(_19.fld0.fld0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_19.fld1 = RET;
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(8_usize, 29_usize, Move(_29), 27_usize, Move(_27), 2_usize, Move(_2), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(8_usize, 34_usize, Move(_34), 26_usize, Move(_26), 28_usize, Move(_28), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: (f64,),mut _2: bool,mut _3: f64,mut _4: Adt50,mut _5: bool,mut _6: f64,mut _7: [bool; 6],mut _8: f64,mut _9: f64,mut _10: f64,mut _11: f64,mut _12: f64,mut _13: (f64,),mut _14: f64,mut _15: f64) -> char {
mir! {
type RET = char;
let _16: f32;
let _17: (bool, f32, i64, [bool; 6]);
let _18: u8;
let _19: (f64,);
let _20: f32;
let _21: char;
let _22: bool;
let _23: (bool, f32, i64, [bool; 6]);
let _24: *mut u16;
let _25: isize;
let _26: [i64; 8];
let _27: *const i8;
let _28: isize;
let _29: (*const &'static f64, [u64; 7]);
let _30: isize;
let _31: isize;
let _32: bool;
let _33: char;
let _34: [bool; 6];
let _35: [u16; 8];
let _36: [u32; 8];
let _37: bool;
let _38: [char; 6];
let _39: isize;
let _40: u8;
let _41: Adt44;
let _42: i64;
let _43: u64;
let _44: f64;
let _45: ();
let _46: ();
{
RET = '\u{95717}';
_15 = -_1.0;
_9 = _3;
_11 = _10 + _3;
RET = '\u{440d4}';
_13 = _1;
_4.fld1 = [_2,_2,_2,_2,_5,_5];
_16 = 2315317397372898829_u64 as f32;
_2 = _5;
RET = '\u{33428}';
RET = '\u{71856}';
_1 = (_14,);
_17.2 = -(-4773879531549655854_i64);
_6 = -_10;
_13 = _1;
_17.3 = _7;
_8 = _14 - _11;
_17.3 = _7;
_4.fld1 = [_2,_5,_5,_2,_5,_5];
RET = '\u{911b6}';
match _4.fld0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
25863 => bb6,
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
_15 = -_3;
_17.3 = [_2,_5,_2,_2,_2,_2];
_11 = 2973596579003882592_usize as f64;
_17.2 = (-9223372036854775808_isize) as i64;
_4 = Adt50 { fld0: (-19172_i16),fld1: _7 };
_4 = Adt50 { fld0: 31769_i16,fld1: _17.3 };
_5 = !_2;
_13 = (_8,);
_17.1 = -_16;
_19.0 = -_14;
Call(_7 = fn10(_19, _14, _10, _3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_14 = -_13.0;
_4.fld0 = (-22836_i16);
_18 = !81_u8;
_4 = Adt50 { fld0: (-17553_i16),fld1: _17.3 };
_19.0 = -_8;
_17.3 = [_5,_2,_5,_2,_5,_5];
_19.0 = -_14;
_19 = _13;
_17 = (_5, _16, (-6304083781364501801_i64), _4.fld1);
_6 = (-1660617894_i32) as f64;
_13.0 = _18 as f64;
Goto(bb8)
}
bb8 = {
_15 = _1.0 + _10;
_6 = -_19.0;
_9 = -_15;
_20 = -_16;
_19 = (_10,);
_18 = 185_u8;
_2 = _8 > _8;
_22 = _5;
_17 = (_22, _20, (-4321189181993769368_i64), _4.fld1);
_4.fld1 = _17.3;
_12 = -_8;
_8 = -_6;
_5 = _10 != _10;
RET = '\u{8d79a}';
RET = '\u{162f}';
_1.0 = _15 * _9;
_4.fld0 = !(-18472_i16);
_16 = _18 as f32;
_9 = _3 * _3;
_10 = _15;
_13.0 = 18233451588316687431_u64 as f64;
_5 = _17.0;
Goto(bb9)
}
bb9 = {
RET = '\u{4d396}';
_3 = _16 as f64;
RET = '\u{dc6b5}';
_14 = _15 + _1.0;
_23.1 = -_20;
_26 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
_17 = (_5, _20, 3330318327549629479_i64, _4.fld1);
_23.1 = -_20;
_19.0 = _14 + _6;
_7 = _17.3;
_17.3 = [_2,_2,_17.0,_2,_22,_2];
_8 = _9;
RET = '\u{1638b}';
_12 = -_19.0;
_29.1 = [10497133509440797085_u64,6327359316360706274_u64,6079361424848378448_u64,3347880632474809655_u64,7686664182618721631_u64,4078877103499758219_u64,12638198122171886796_u64];
_30 = (-9223372036854775808_isize) - (-45_isize);
_30 = (-9223372036854775808_isize);
_8 = -_19.0;
_23.0 = !_17.0;
_23 = (_2, _16, _17.2, _17.3);
Goto(bb10)
}
bb10 = {
_19 = (_9,);
_26 = [_23.2,_23.2,_17.2,_17.2,_17.2,_23.2,_23.2,_17.2];
_4 = Adt50 { fld0: (-6144_i16),fld1: _7 };
_12 = -_10;
_15 = _9;
_16 = _23.1;
_15 = _6;
_17.0 = !_2;
_32 = _5;
_23.2 = _17.2 ^ _17.2;
_23.0 = _17.2 >= _23.2;
Call(_16 = fn19(_1.0, _23.0, _4.fld0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_5 = _17.0 & _17.0;
_17.3 = [_2,_22,_2,_17.0,_2,_32];
_13 = (_12,);
_4.fld1 = [_2,_5,_17.0,_23.0,_5,_2];
_18 = 214_u8;
_7 = _17.3;
_31 = _30 >> _17.2;
_4.fld0 = (-28330_i16) ^ 17542_i16;
_9 = _4.fld0 as f64;
_30 = _31 * _31;
_21 = RET;
_28 = _31;
_28 = _30 + _30;
_16 = _20 * _17.1;
_19 = (_1.0,);
_4.fld1 = _17.3;
_17.0 = _1.0 == _14;
_23.0 = _5 > _2;
Goto(bb12)
}
bb12 = {
_23.3 = [_22,_17.0,_5,_17.0,_23.0,_2];
_2 = _32;
_37 = !_2;
_35 = [44193_u16,49636_u16,13765_u16,25146_u16,24807_u16,1999_u16,33458_u16,47126_u16];
_36 = [910231432_u32,2509897470_u32,2083749786_u32,1519686074_u32,180257960_u32,1933050279_u32,428968000_u32,697108743_u32];
_13 = (_1.0,);
_25 = !_31;
_16 = _20;
Goto(bb13)
}
bb13 = {
_13 = (_8,);
_3 = _18 as f64;
_23.0 = _37 & _2;
_18 = 242_u8;
_20 = -_23.1;
_23.0 = _17.0;
_28 = -_25;
_33 = RET;
_29.1 = [4964312516736003229_u64,3211543827273779996_u64,18225629597889686829_u64,3807125659810592193_u64,7356899404446079702_u64,18340930577162223593_u64,3454373380222108620_u64];
_19 = (_14,);
_1.0 = _13.0 * _15;
_17.2 = _23.2 >> _25;
_6 = 64866_u16 as f64;
_32 = _5;
_5 = _23.0;
_1.0 = 101710563095639750386260321862731714806_i128 as f64;
match _18 {
0 => bb14,
1 => bb15,
242 => bb17,
_ => bb16
}
}
bb14 = {
_23.3 = [_22,_17.0,_5,_17.0,_23.0,_2];
_2 = _32;
_37 = !_2;
_35 = [44193_u16,49636_u16,13765_u16,25146_u16,24807_u16,1999_u16,33458_u16,47126_u16];
_36 = [910231432_u32,2509897470_u32,2083749786_u32,1519686074_u32,180257960_u32,1933050279_u32,428968000_u32,697108743_u32];
_13 = (_1.0,);
_25 = !_31;
_16 = _20;
Goto(bb13)
}
bb15 = {
Return()
}
bb16 = {
_19 = (_9,);
_26 = [_23.2,_23.2,_17.2,_17.2,_17.2,_23.2,_23.2,_17.2];
_4 = Adt50 { fld0: (-6144_i16),fld1: _7 };
_12 = -_10;
_15 = _9;
_16 = _23.1;
_15 = _6;
_17.0 = !_2;
_32 = _5;
_23.2 = _17.2 ^ _17.2;
_23.0 = _17.2 >= _23.2;
Call(_16 = fn19(_1.0, _23.0, _4.fld0), ReturnTo(bb11), UnwindUnreachable())
}
bb17 = {
_17.2 = _23.2;
_34 = [_22,_5,_5,_5,_5,_23.0];
_23.2 = -_17.2;
_40 = _18 ^ _18;
_16 = _17.1 - _17.1;
_18 = _40;
_7 = [_22,_5,_23.0,_32,_37,_22];
_43 = 1354886007_i32 as u64;
_38 = [_33,_21,RET,RET,_33,_21];
_16 = -_23.1;
_42 = _23.2;
_17 = _23;
_42 = !_23.2;
_20 = _17.1;
_37 = _2 ^ _22;
_39 = _31 * _31;
Goto(bb18)
}
bb18 = {
Call(_45 = dump_var(9_usize, 34_usize, Move(_34), 31_usize, Move(_31), 43_usize, Move(_43), 28_usize, Move(_28)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_45 = dump_var(9_usize, 22_usize, Move(_22), 40_usize, Move(_40), 25_usize, Move(_25), 26_usize, Move(_26)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_45 = dump_var(9_usize, 5_usize, Move(_5), 2_usize, Move(_2), 35_usize, Move(_35), 46_usize, _46), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: (f64,),mut _2: f64,mut _3: f64,mut _4: f64) -> [bool; 6] {
mir! {
type RET = [bool; 6];
let _5: (bool, f32, i64, [bool; 6]);
let _6: u32;
let _7: *mut [u64; 7];
let _8: char;
let _9: *const u32;
let _10: Adt51;
let _11: Adt56;
let _12: char;
let _13: bool;
let _14: *mut [u64; 7];
let _15: char;
let _16: (*const &'static f64, [u64; 7]);
let _17: Adt49;
let _18: (f64,);
let _19: ();
let _20: ();
{
RET = [false,false,true,false,true,true];
_3 = -_1.0;
_3 = _2 - _2;
_2 = 1438656679_u32 as f64;
_1 = (_3,);
RET = [false,true,false,false,false,true];
_1 = (_3,);
_1.0 = -_3;
_4 = 24348_u16 as f64;
_1.0 = _3;
_1 = (_3,);
RET = [false,true,false,false,true,true];
Goto(bb1)
}
bb1 = {
RET = [true,true,false,false,true,true];
_4 = _3 - _1.0;
_3 = -_1.0;
_1 = (_4,);
_1.0 = -_4;
_5.2 = !8523198435294169977_i64;
_6 = 4175928232_u32;
_1 = (_4,);
_5.1 = _6 as f32;
_1 = (_3,);
_5.3 = [true,true,false,true,false,false];
_4 = -_1.0;
_1.0 = _3 - _4;
_5.0 = !false;
RET = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_3 = _4 * _1.0;
Goto(bb2)
}
bb2 = {
RET = _5.3;
_5.1 = _1.0 as f32;
Call(_5.1 = fn11(_1, _4, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = _1.0;
_2 = _4 - _3;
_4 = -_3;
_3 = _2;
_3 = -_1.0;
_9 = core::ptr::addr_of!(_6);
_1 = (_4,);
_5.1 = (-2090996201_i32) as f32;
RET = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_5.3 = RET;
RET = _5.3;
Goto(bb4)
}
bb4 = {
_1.0 = _2;
_5.0 = true;
_3 = 6_usize as f64;
_1.0 = -_2;
_1.0 = _2 + _4;
RET = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_1 = (_2,);
_6 = 2760216343_u32;
(*_9) = 4137189542_u32 + 1167919328_u32;
_1.0 = _2;
Call(_9 = fn15(_1.0, _1.0, _2, _2, _2, _4, _2, _2, _1, _4, _1.0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_1 = (_2,);
_1.0 = 32907903805753568591528093980920736632_u128 as f64;
RET = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_5.2 = (-5115516510575692259_i64);
_8 = '\u{1d457}';
_9 = core::ptr::addr_of!(_6);
Goto(bb6)
}
bb6 = {
_2 = -_4;
_1.0 = _4;
RET = _5.3;
(*_9) = (-1560062229_i32) as u32;
_1 = (_4,);
_5.3 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
(*_9) = 101_u8 as u32;
RET = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
RET = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_5.0 = true;
_3 = _1.0 * _1.0;
_5.1 = 1623987486119788489_u64 as f32;
_2 = _6 as f64;
_1 = (_4,);
_5.3 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_13 = !_5.0;
_1.0 = -_4;
_15 = _8;
_9 = core::ptr::addr_of!((*_9));
Goto(bb7)
}
bb7 = {
_5.2 = (-3563281739057567740_i64);
Goto(bb8)
}
bb8 = {
_1.0 = 9223372036854775807_isize as f64;
_5.1 = 265434363281095382247061800477351128624_u128 as f32;
_9 = core::ptr::addr_of!(_6);
_1 = (_3,);
RET = _5.3;
_7 = core::ptr::addr_of_mut!(_16.1);
(*_7) = [12624276189854031438_u64,2384118264910911052_u64,4771691649866646004_u64,17982825748544561604_u64,4050473662011177116_u64,2160790087467507723_u64,12634121036755160138_u64];
_5.2 = (-6900808037509653940_i64) - (-1961508361166159607_i64);
(*_7) = [17352255513404506832_u64,17325070724371466189_u64,11953860544970499929_u64,11835615088089317139_u64,11347927545005270535_u64,2306689806832995089_u64,4070000078878792135_u64];
(*_9) = 3065603695_u32;
_5.1 = 25_i8 as f32;
(*_9) = 1482467477_u32 * 2287023261_u32;
_5.1 = 7_usize as f32;
_12 = _15;
_7 = core::ptr::addr_of_mut!(_16.1);
_5.2 = 7996493313108160034_i64;
_1 = (_3,);
match _5.2 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb9,
5 => bb10,
6 => bb11,
7996493313108160034 => bb13,
_ => bb12
}
}
bb9 = {
_5.2 = (-3563281739057567740_i64);
Goto(bb8)
}
bb10 = {
_2 = -_4;
_1.0 = _4;
RET = _5.3;
(*_9) = (-1560062229_i32) as u32;
_1 = (_4,);
_5.3 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
(*_9) = 101_u8 as u32;
RET = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
RET = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_5.0 = true;
_3 = _1.0 * _1.0;
_5.1 = 1623987486119788489_u64 as f32;
_2 = _6 as f64;
_1 = (_4,);
_5.3 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_13 = !_5.0;
_1.0 = -_4;
_15 = _8;
_9 = core::ptr::addr_of!((*_9));
Goto(bb7)
}
bb11 = {
_4 = _1.0;
_2 = _4 - _3;
_4 = -_3;
_3 = _2;
_3 = -_1.0;
_9 = core::ptr::addr_of!(_6);
_1 = (_4,);
_5.1 = (-2090996201_i32) as f32;
RET = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_5.3 = RET;
RET = _5.3;
Goto(bb4)
}
bb12 = {
_1.0 = _2;
_5.0 = true;
_3 = 6_usize as f64;
_1.0 = -_2;
_1.0 = _2 + _4;
RET = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_1 = (_2,);
_6 = 2760216343_u32;
(*_9) = 4137189542_u32 + 1167919328_u32;
_1.0 = _2;
Call(_9 = fn15(_1.0, _1.0, _2, _2, _2, _4, _2, _2, _1, _4, _1.0), ReturnTo(bb5), UnwindUnreachable())
}
bb13 = {
_7 = core::ptr::addr_of_mut!(_16.1);
_3 = _1.0 * _1.0;
_5.1 = _3 as f32;
_14 = _7;
_16.1 = [11716648348356057134_u64,8783630664362210642_u64,17894673482836469184_u64,3364008282801202948_u64,5223396071669415323_u64,3737622502408727169_u64,4433841957531425122_u64];
_8 = _15;
_16.1 = [13855446332560611504_u64,7194481812693461527_u64,16138389890909380226_u64,4440456140482372658_u64,10166179539305220131_u64,7764914240067549673_u64,12457210140640945852_u64];
(*_14) = [1374512138231192835_u64,16587748924687815369_u64,15352566372892895662_u64,10151614578950891116_u64,13702374524253547322_u64,12175398050205048927_u64,17469199775262571278_u64];
_14 = _7;
(*_7) = [10154105029915433961_u64,16234356209574713133_u64,8611243860278169368_u64,165337470144318982_u64,17418597699011852447_u64,8793974673520780351_u64,11330049979870426883_u64];
(*_9) = 96275732698145739295115203054414149229_i128 as u32;
_14 = _7;
(*_14) = [4144709018021837808_u64,6975039207515779729_u64,919527979788456144_u64,6596951064932005228_u64,11901995301504777085_u64,5893694307068237172_u64,15824736277047833126_u64];
_5.3 = RET;
_4 = _1.0 + _1.0;
_16.1 = [11738325723490864148_u64,13897620839078016324_u64,6906911704403829134_u64,5663262538609943628_u64,17515707577842585936_u64,6388888882977506810_u64,5827267196911528149_u64];
match _5.2 {
0 => bb14,
1 => bb15,
7996493313108160034 => bb17,
_ => bb16
}
}
bb14 = {
_4 = _1.0;
_2 = _4 - _3;
_4 = -_3;
_3 = _2;
_3 = -_1.0;
_9 = core::ptr::addr_of!(_6);
_1 = (_4,);
_5.1 = (-2090996201_i32) as f32;
RET = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_5.3 = RET;
RET = _5.3;
Goto(bb4)
}
bb15 = {
_1.0 = 9223372036854775807_isize as f64;
_5.1 = 265434363281095382247061800477351128624_u128 as f32;
_9 = core::ptr::addr_of!(_6);
_1 = (_3,);
RET = _5.3;
_7 = core::ptr::addr_of_mut!(_16.1);
(*_7) = [12624276189854031438_u64,2384118264910911052_u64,4771691649866646004_u64,17982825748544561604_u64,4050473662011177116_u64,2160790087467507723_u64,12634121036755160138_u64];
_5.2 = (-6900808037509653940_i64) - (-1961508361166159607_i64);
(*_7) = [17352255513404506832_u64,17325070724371466189_u64,11953860544970499929_u64,11835615088089317139_u64,11347927545005270535_u64,2306689806832995089_u64,4070000078878792135_u64];
(*_9) = 3065603695_u32;
_5.1 = 25_i8 as f32;
(*_9) = 1482467477_u32 * 2287023261_u32;
_5.1 = 7_usize as f32;
_12 = _15;
_7 = core::ptr::addr_of_mut!(_16.1);
_5.2 = 7996493313108160034_i64;
_1 = (_3,);
match _5.2 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb9,
5 => bb10,
6 => bb11,
7996493313108160034 => bb13,
_ => bb12
}
}
bb16 = {
_1 = (_2,);
_1.0 = 32907903805753568591528093980920736632_u128 as f64;
RET = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_5.2 = (-5115516510575692259_i64);
_8 = '\u{1d457}';
_9 = core::ptr::addr_of!(_6);
Goto(bb6)
}
bb17 = {
_5.2 = (-3216861208129877434_i64) ^ (-4069266980126564241_i64);
_3 = 9223372036854775807_isize as f64;
_2 = -_4;
_3 = _5.2 as f64;
_5.0 = _13 ^ _13;
Goto(bb18)
}
bb18 = {
Call(_19 = dump_var(10_usize, 15_usize, Move(_15), 12_usize, Move(_12), 20_usize, _20, 20_usize, _20), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: (f64,),mut _2: f64,mut _3: f64) -> f32 {
mir! {
type RET = f32;
let _4: f32;
let _5: [u64; 8];
let _6: f32;
let _7: Adt48;
let _8: char;
let _9: Adt47;
let _10: Adt53;
let _11: (f64,);
let _12: usize;
let _13: Adt50;
let _14: (bool, f32, i64, [bool; 6]);
let _15: Adt50;
let _16: Adt55;
let _17: bool;
let _18: [u16; 8];
let _19: Adt51;
let _20: isize;
let _21: f32;
let _22: Adt47;
let _23: [u32; 3];
let _24: i16;
let _25: Adt45;
let _26: i32;
let _27: isize;
let _28: Adt53;
let _29: Adt54;
let _30: [u32; 3];
let _31: Adt50;
let _32: f64;
let _33: u128;
let _34: bool;
let _35: f64;
let _36: Adt56;
let _37: [u32; 3];
let _38: i8;
let _39: isize;
let _40: *mut [u64; 7];
let _41: [bool; 6];
let _42: (bool, f32, i64, [bool; 6]);
let _43: i128;
let _44: Adt48;
let _45: ();
let _46: ();
{
_4 = 9223372036854775807_isize as f32;
_1 = (_2,);
_1.0 = _3 * _3;
_2 = _3;
_2 = -_3;
RET = -_4;
_3 = _2;
_2 = -_3;
RET = _4;
_4 = RET * RET;
RET = (-17_i8) as f32;
_1 = (_3,);
_3 = _1.0 + _1.0;
Call(_2 = fn12(_1, _1, _1.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = [7959299550915224991_u64,16105722120218566987_u64,13648296935641697800_u64,2680989217882757761_u64,10406745427785896555_u64,5390973348453627436_u64,2716047987891912635_u64,11603630210252662151_u64];
_2 = _1.0;
_3 = _1.0 - _1.0;
_7.fld3 = 124_i8 - (-47_i8);
_7.fld5 = [26748_u16,57352_u16,2738_u16,2665_u16,52165_u16,28615_u16,955_u16,24338_u16];
_7.fld0.fld1 = [6720562742377406291_u64,13126738934822525564_u64];
_7.fld2 = false as u64;
_7.fld0.fld1 = [_7.fld2,_7.fld2];
_6 = -RET;
_7.fld4.3 = [false,false,true,true,true,true];
_7.fld4.0 = true;
_7.fld0.fld1 = [_7.fld2,_7.fld2];
_7.fld5 = [41863_u16,27854_u16,44897_u16,36745_u16,14509_u16,55645_u16,33128_u16,25640_u16];
RET = _4 - _6;
_7.fld4.2 = !5805880781559107567_i64;
_9.fld1 = '\u{a9d7d}';
_7.fld0.fld2 = (-14953_i16);
_9 = Adt47 { fld0: RET,fld1: '\u{dacd6}' };
_7.fld5 = [34037_u16,63757_u16,19660_u16,42738_u16,50932_u16,37432_u16,44917_u16,33208_u16];
_2 = -_3;
_7.fld0.fld1 = [_7.fld2,_7.fld2];
_7.fld0.fld0 = -_7.fld3;
_5 = [_7.fld2,_7.fld2,_7.fld2,_7.fld2,_7.fld2,_7.fld2,_7.fld2,_7.fld2];
_11.0 = 13607157481440913177_usize as f64;
Goto(bb2)
}
bb2 = {
_7.fld2 = 293934740014412071931588731807354398595_u128 as u64;
_9.fld1 = '\u{15aa0}';
_9.fld0 = RET;
_7.fld4.1 = _6;
_9.fld1 = '\u{d7a43}';
RET = _6 * _4;
_7.fld1 = _9.fld1;
_7.fld4.1 = RET - _4;
_7.fld4.2 = (-6984688902574671129_i64);
_9.fld1 = _7.fld1;
Goto(bb3)
}
bb3 = {
_7.fld1 = _9.fld1;
_9.fld0 = -RET;
RET = _4 * _6;
_14 = (_7.fld4.0, _9.fld0, _7.fld4.2, _7.fld4.3);
_7.fld0.fld1 = [_7.fld2,_7.fld2];
_7.fld4 = (_14.0, _14.1, _14.2, _14.3);
_14.2 = _7.fld4.2 * _7.fld4.2;
_15.fld1 = [_14.0,_7.fld4.0,_14.0,_14.0,_7.fld4.0,_14.0];
_7.fld3 = 17639327339558574290394858524065505467_i128 as i8;
_7.fld4 = (_14.0, _4, _14.2, _14.3);
_7.fld3 = !_7.fld0.fld0;
_7.fld4.3 = [_14.0,_14.0,_7.fld4.0,_7.fld4.0,_7.fld4.0,_14.0];
RET = 9223372036854775807_isize as f32;
_15.fld0 = _7.fld0.fld2 * _7.fld0.fld2;
_13.fld0 = _15.fld0;
Goto(bb4)
}
bb4 = {
_15 = Adt50 { fld0: _13.fld0,fld1: _14.3 };
_14.1 = _7.fld0.fld0 as f32;
_17 = !_14.0;
_13 = Adt50 { fld0: _15.fld0,fld1: _14.3 };
_4 = _7.fld2 as f32;
_7.fld2 = 7592492835829724217_u64 + 1233461489246540536_u64;
RET = _14.1;
_9 = Adt47 { fld0: _7.fld4.1,fld1: _7.fld1 };
_7.fld2 = 776378149122511738_u64;
_22.fld1 = _9.fld1;
_7.fld4.2 = _14.2;
_7.fld3 = -_7.fld0.fld0;
_14.1 = _7.fld4.1;
_15 = Adt50 { fld0: _13.fld0,fld1: _13.fld1 };
_1 = _11;
RET = _14.1 * _9.fld0;
_9.fld1 = _22.fld1;
_15.fld1 = [_17,_17,_7.fld4.0,_7.fld4.0,_7.fld4.0,_14.0];
_7.fld4.1 = RET + _4;
_7.fld5 = [5462_u16,37855_u16,106_u16,38005_u16,58595_u16,56911_u16,16846_u16,48605_u16];
match _7.fld2 {
0 => bb1,
776378149122511738 => bb5,
_ => bb3
}
}
bb5 = {
_25.fld2 = _14.2 as i16;
_13.fld1 = [_7.fld4.0,_7.fld4.0,_7.fld4.0,_7.fld4.0,_17,_17];
_9 = Adt47 { fld0: _7.fld4.1,fld1: _22.fld1 };
_4 = -_9.fld0;
match _7.fld2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
776378149122511738 => bb7,
_ => bb6
}
}
bb6 = {
_7.fld1 = _9.fld1;
_9.fld0 = -RET;
RET = _4 * _6;
_14 = (_7.fld4.0, _9.fld0, _7.fld4.2, _7.fld4.3);
_7.fld0.fld1 = [_7.fld2,_7.fld2];
_7.fld4 = (_14.0, _14.1, _14.2, _14.3);
_14.2 = _7.fld4.2 * _7.fld4.2;
_15.fld1 = [_14.0,_7.fld4.0,_14.0,_14.0,_7.fld4.0,_14.0];
_7.fld3 = 17639327339558574290394858524065505467_i128 as i8;
_7.fld4 = (_14.0, _4, _14.2, _14.3);
_7.fld3 = !_7.fld0.fld0;
_7.fld4.3 = [_14.0,_14.0,_7.fld4.0,_7.fld4.0,_7.fld4.0,_14.0];
RET = 9223372036854775807_isize as f32;
_15.fld0 = _7.fld0.fld2 * _7.fld0.fld2;
_13.fld0 = _15.fld0;
Goto(bb4)
}
bb7 = {
_22.fld0 = _7.fld0.fld2 as f32;
_8 = _22.fld1;
_14.0 = _7.fld4.0;
RET = _6 - _6;
_15.fld1 = _14.3;
_13.fld0 = _25.fld2 ^ _25.fld2;
_7.fld0.fld2 = _7.fld4.1 as i16;
Call(RET = fn13(_2, _3, Move(_15), _2, _7.fld4), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_12 = 5_usize;
Goto(bb9)
}
bb9 = {
_7.fld1 = _9.fld1;
_18[_12] = (-1315417426_i32) as u16;
_14.3 = [_7.fld4.3[_12],_17,_14.0,_7.fld4.0,_17,_7.fld4.3[_12]];
_15.fld1 = [_17,_14.3[_12],_7.fld4.3[_12],_17,_7.fld4.3[_12],_7.fld4.3[_12]];
_25 = _7.fld0;
match _7.fld5[_12] {
0 => bb1,
1 => bb2,
2 => bb4,
56911 => bb11,
_ => bb10
}
}
bb10 = {
_15 = Adt50 { fld0: _13.fld0,fld1: _14.3 };
_14.1 = _7.fld0.fld0 as f32;
_17 = !_14.0;
_13 = Adt50 { fld0: _15.fld0,fld1: _14.3 };
_4 = _7.fld2 as f32;
_7.fld2 = 7592492835829724217_u64 + 1233461489246540536_u64;
RET = _14.1;
_9 = Adt47 { fld0: _7.fld4.1,fld1: _7.fld1 };
_7.fld2 = 776378149122511738_u64;
_22.fld1 = _9.fld1;
_7.fld4.2 = _14.2;
_7.fld3 = -_7.fld0.fld0;
_14.1 = _7.fld4.1;
_15 = Adt50 { fld0: _13.fld0,fld1: _13.fld1 };
_1 = _11;
RET = _14.1 * _9.fld0;
_9.fld1 = _22.fld1;
_15.fld1 = [_17,_17,_7.fld4.0,_7.fld4.0,_7.fld4.0,_14.0];
_7.fld4.1 = RET + _4;
_7.fld5 = [5462_u16,37855_u16,106_u16,38005_u16,58595_u16,56911_u16,16846_u16,48605_u16];
match _7.fld2 {
0 => bb1,
776378149122511738 => bb5,
_ => bb3
}
}
bb11 = {
_14.3[_12] = _17;
_4 = _12 as f32;
_15.fld0 = _25.fld2 >> _25.fld0;
_26 = !(-1387786751_i32);
_15 = Adt50 { fld0: _13.fld0,fld1: _7.fld4.3 };
_7.fld0 = _25;
_7.fld0.fld0 = _25.fld0 - _25.fld0;
_7.fld3 = _25.fld0;
_25 = _7.fld0;
_7.fld4.3[_12] = _3 <= _3;
_7.fld4 = _14;
_7.fld3 = _25.fld0 * _25.fld0;
_3 = _2 + _2;
_3 = _2;
_8 = _22.fld1;
_7.fld5[_12] = _18[_12];
_20 = _25.fld2 as isize;
_18[_12] = _7.fld5[_12] - _7.fld5[_12];
_14 = _7.fld4;
_7.fld5 = [_18[_12],_18[_12],_18[_12],_18[_12],_18[_12],_18[_12],_18[_12],_18[_12]];
Goto(bb12)
}
bb12 = {
_31 = Adt50 { fld0: _7.fld0.fld2,fld1: _7.fld4.3 };
_31.fld1 = [_15.fld1[_12],_15.fld1[_12],_14.0,_13.fld1[_12],_14.0,_15.fld1[_12]];
_25.fld0 = _12 as i8;
_21 = -_9.fld0;
_31.fld1 = [_14.3[_12],_17,_13.fld1[_12],_14.0,_7.fld4.0,_14.3[_12]];
_9.fld0 = RET * _7.fld4.1;
_11 = (_3,);
_27 = _20;
_7.fld4 = _14;
_20 = _27 | _27;
_14.3 = _31.fld1;
_7.fld2 = !_5[_12];
_7.fld1 = _22.fld1;
_22 = _9;
_15.fld1[_12] = _3 == _2;
_31 = Move(_15);
Call(_22.fld1 = fn14(_31.fld1[_12], _22.fld0, _3, _27), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_15 = Adt50 { fld0: _13.fld0,fld1: _31.fld1 };
_17 = _11.0 > _3;
_5 = [_7.fld2,_7.fld2,_7.fld2,_7.fld2,_7.fld2,_7.fld2,_7.fld2,_7.fld2];
_23 = [1447504141_u32,1294096822_u32,400424618_u32];
_14.2 = -_7.fld4.2;
_13 = Move(_31);
_7.fld4.1 = _9.fld0;
_31.fld0 = _22.fld0 as i16;
_21 = RET - _9.fld0;
_7.fld1 = _9.fld1;
_14.0 = _20 > _20;
_14 = (_17, _7.fld4.1, _7.fld4.2, _15.fld1);
_14.2 = _7.fld4.2;
_25.fld2 = _7.fld0.fld2;
_39 = _20;
_11 = _1;
_13.fld1 = _15.fld1;
_9.fld0 = 33574_u16 as f32;
_31.fld1 = [_17,_14.0,_14.0,_14.0,_14.0,_14.0];
_38 = _7.fld0.fld0 - _7.fld3;
_7.fld4.1 = RET + _14.1;
_2 = _3 * _3;
_38 = _12 as i8;
_13.fld0 = _7.fld0.fld2 & _31.fld0;
_41 = [_14.0,_17,_14.0,_14.0,_14.0,_17];
_24 = _31.fld0;
Goto(bb14)
}
bb14 = {
_7.fld4 = (_17, _22.fld0, _14.2, _14.3);
_31 = Adt50 { fld0: _24,fld1: _15.fld1 };
_22 = _9;
_21 = _14.1 - RET;
_13 = Move(_15);
_34 = _17 > _17;
_8 = _9.fld1;
_41 = [_17,_7.fld4.0,_7.fld4.0,_17,_17,_7.fld4.0];
_7.fld4 = (_17, _21, _14.2, _41);
_25.fld1 = [_7.fld2,_7.fld2];
_4 = -_7.fld4.1;
_15.fld1 = [_17,_7.fld4.0,_17,_17,_14.0,_14.0];
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(11_usize, 24_usize, Move(_24), 34_usize, Move(_34), 8_usize, Move(_8), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(11_usize, 41_usize, Move(_41), 27_usize, Move(_27), 46_usize, _46, 46_usize, _46), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: (f64,),mut _2: (f64,),mut _3: f64) -> f64 {
mir! {
type RET = f64;
let _4: ();
let _5: ();
{
RET = _1.0 + _1.0;
Goto(bb1)
}
bb1 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: f64,mut _2: f64,mut _3: Adt50,mut _4: f64,mut _5: (bool, f32, i64, [bool; 6])) -> f32 {
mir! {
type RET = f32;
let _6: bool;
let _7: Adt58;
let _8: [u64; 2];
let _9: Adt53;
let _10: Adt45;
let _11: Adt50;
let _12: [u64; 8];
let _13: [u32; 8];
let _14: Adt50;
let _15: char;
let _16: isize;
let _17: [u64; 2];
let _18: [u32; 3];
let _19: isize;
let _20: usize;
let _21: (f64,);
let _22: bool;
let _23: *mut char;
let _24: i16;
let _25: f64;
let _26: i8;
let _27: (bool, f32, i64, [bool; 6]);
let _28: Adt59;
let _29: (u64, i128, i8, &'static f64, bool);
let _30: isize;
let _31: Adt57;
let _32: ();
let _33: ();
{
_2 = -_1;
RET = _5.1;
_5.1 = 35816376672729300710775553515974791975_i128 as f32;
_2 = _1 - _4;
_5.0 = !true;
Goto(bb1)
}
bb1 = {
RET = _5.1;
_5.0 = !true;
_1 = _4;
_3.fld1 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_3.fld1 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_3.fld0 = 4193646309_u32 as i16;
_1 = _4;
_5 = (true, RET, (-6452497169621910920_i64), _3.fld1);
_8 = [8182818185873856713_u64,8615535897170656482_u64];
_3.fld1 = _5.3;
_4 = 254757919384377433109124327819109724873_u128 as f64;
_2 = _5.1 as f64;
_4 = _1;
_8 = [11825925554364063676_u64,7625164193415826079_u64];
_6 = !_5.0;
_8 = [11359820675618799076_u64,16596307288155563593_u64];
_5.3 = [_6,_6,_5.0,_6,_5.0,_5.0];
_10.fld1 = [102412667124524165_u64,200672706392025091_u64];
_10 = Adt45 { fld0: 40_i8,fld1: _8,fld2: _3.fld0 };
_5 = (_6, RET, (-4304021012744420133_i64), _3.fld1);
_6 = _5.2 <= _5.2;
_5.3 = [_6,_6,_6,_6,_6,_6];
_3.fld0 = 9223372036854775807_isize as i16;
Goto(bb2)
}
bb2 = {
_10.fld2 = _3.fld0;
_5.3 = _3.fld1;
_10.fld0 = (-17_i8);
_2 = -_4;
_3 = Adt50 { fld0: _10.fld2,fld1: _5.3 };
RET = _5.1 * _5.1;
_5.2 = (-4332324419882634368_i64) & 147536924318051350_i64;
_1 = 30398369884296887713646958063731229211_i128 as f64;
_3.fld1 = _5.3;
_5.3 = [_6,_5.0,_6,_6,_6,_6];
_11.fld0 = !_3.fld0;
RET = 8530766886023943528_u64 as f32;
_5.2 = (-5355944896786121101_i64) * (-4453182578743719790_i64);
_10.fld1 = [9651966739165396772_u64,18147874004231476473_u64];
_1 = _5.2 as f64;
_1 = (-1269181839_i32) as f64;
_8 = [15798790944599600101_u64,7047422078594451281_u64];
match _10.fld0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768211439 => bb8,
_ => bb7
}
}
bb3 = {
RET = _5.1;
_5.0 = !true;
_1 = _4;
_3.fld1 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_3.fld1 = [_5.0,_5.0,_5.0,_5.0,_5.0,_5.0];
_3.fld0 = 4193646309_u32 as i16;
_1 = _4;
_5 = (true, RET, (-6452497169621910920_i64), _3.fld1);
_8 = [8182818185873856713_u64,8615535897170656482_u64];
_3.fld1 = _5.3;
_4 = 254757919384377433109124327819109724873_u128 as f64;
_2 = _5.1 as f64;
_4 = _1;
_8 = [11825925554364063676_u64,7625164193415826079_u64];
_6 = !_5.0;
_8 = [11359820675618799076_u64,16596307288155563593_u64];
_5.3 = [_6,_6,_5.0,_6,_5.0,_5.0];
_10.fld1 = [102412667124524165_u64,200672706392025091_u64];
_10 = Adt45 { fld0: 40_i8,fld1: _8,fld2: _3.fld0 };
_5 = (_6, RET, (-4304021012744420133_i64), _3.fld1);
_6 = _5.2 <= _5.2;
_5.3 = [_6,_6,_6,_6,_6,_6];
_3.fld0 = 9223372036854775807_isize as i16;
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
_12 = [16763617699136238992_u64,434892037498222850_u64,16249361795076575344_u64,17597871268386956999_u64,16855215126570349619_u64,15064848223254630067_u64,17193954321032532906_u64,8860721726412683256_u64];
_8 = [2400524288357718566_u64,8319045844825071667_u64];
_5.2 = (-6503149961052482086_i64) >> _10.fld2;
_1 = _2;
_11.fld0 = -_3.fld0;
_5.2 = 542843258_u32 as i64;
Goto(bb9)
}
bb9 = {
_5.3 = [_6,_5.0,_5.0,_6,_6,_5.0];
match _10.fld0 {
0 => bb1,
340282366920938463463374607431768211439 => bb10,
_ => bb7
}
}
bb10 = {
_8 = [6733505064318677125_u64,14246672921978932983_u64];
Goto(bb11)
}
bb11 = {
_10.fld1 = _8;
_11.fld0 = _10.fld2 >> _5.2;
_11.fld1 = [_6,_6,_5.0,_6,_5.0,_6];
_14 = Move(_11);
RET = _5.1 * _5.1;
_5.1 = RET - RET;
_15 = '\u{51ed0}';
_16 = _14.fld0 as isize;
_18 = [2906648426_u32,3033509503_u32,402219272_u32];
_14 = Adt50 { fld0: _3.fld0,fld1: _5.3 };
_11 = Adt50 { fld0: _14.fld0,fld1: _14.fld1 };
_11.fld1 = [_6,_6,_6,_6,_6,_5.0];
_5.0 = _1 >= _4;
_11.fld0 = _10.fld0 as i16;
_14.fld0 = -_3.fld0;
_5.2 = 2535487711278950388_i64 ^ (-8461265133231608654_i64);
_5.1 = -RET;
_3 = Adt50 { fld0: _10.fld2,fld1: _14.fld1 };
Goto(bb12)
}
bb12 = {
_19 = !_16;
_3 = Adt50 { fld0: _14.fld0,fld1: _5.3 };
_20 = (-367200132_i32) as usize;
_10.fld1 = [12405732328823974870_u64,2287786996945517283_u64];
_3.fld1 = _14.fld1;
_3.fld0 = _11.fld0 * _14.fld0;
_20 = !2_usize;
_6 = _5.0;
_11.fld1 = [_5.0,_5.0,_5.0,_5.0,_6,_6];
_16 = _19;
_5.1 = RET - RET;
_21 = (_1,);
_21 = (_2,);
_22 = _5.0;
_5.2 = 3796826713495045445_i64 | 586761427718257026_i64;
_20 = 841859481536426708_usize & 6092960584116024608_usize;
_17 = [1653507689217275707_u64,10070964012726086941_u64];
_5.3 = [_22,_22,_6,_5.0,_6,_5.0];
_16 = _15 as isize;
RET = _5.1;
_5.0 = _6;
_3.fld1 = [_6,_5.0,_6,_22,_6,_5.0];
_14.fld1 = [_22,_22,_6,_22,_5.0,_22];
_17 = [4807681795081037025_u64,764693960973658643_u64];
Goto(bb13)
}
bb13 = {
_13 = [1892351116_u32,2037347034_u32,2155090832_u32,1873636463_u32,526132722_u32,1731340805_u32,2743402618_u32,2497158503_u32];
_22 = _6;
_5 = (_6, RET, 8897325786921802722_i64, _11.fld1);
_22 = _1 >= _21.0;
_21.0 = _1;
_5 = (_22, RET, 6989686114070058453_i64, _3.fld1);
_18 = [322169732_u32,463745372_u32,2306582543_u32];
_11.fld1 = _3.fld1;
_3 = Move(_14);
_24 = !_10.fld2;
_15 = '\u{12894}';
_14 = Adt50 { fld0: _3.fld0,fld1: _11.fld1 };
_15 = '\u{10126d}';
_22 = _6 ^ _6;
_11.fld1 = [_6,_5.0,_5.0,_22,_6,_6];
_21.0 = _4;
_4 = _1;
_10 = Adt45 { fld0: 16_i8,fld1: _8,fld2: _14.fld0 };
_14.fld1 = [_6,_6,_5.0,_5.0,_22,_22];
match _5.2 {
0 => bb5,
1 => bb11,
2 => bb12,
6989686114070058453 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
_14 = Adt50 { fld0: _24,fld1: _5.3 };
_21 = (_2,);
_25 = _21.0 - _1;
_28.fld0.fld2 = _5.2 as i16;
_12 = [7753360306782471425_u64,2206973496767617098_u64,3314385218142230075_u64,14455745932229176454_u64,4454365542445894419_u64,3450034492985859830_u64,6018615960833379493_u64,215814796974528580_u64];
RET = _5.1;
_5.0 = !_6;
_17 = [9346967471413376772_u64,6644525926095321290_u64];
_10 = Adt45 { fld0: 7_i8,fld1: _17,fld2: _28.fld0.fld2 };
RET = _5.1 * _5.1;
_5.3 = [_22,_5.0,_5.0,_22,_6,_22];
_26 = _10.fld0;
_5.2 = _26 as i64;
_27.3 = [_5.0,_22,_5.0,_5.0,_22,_5.0];
_14.fld1 = [_5.0,_5.0,_5.0,_22,_22,_5.0];
_17 = _10.fld1;
_10.fld1 = [1361472808070182981_u64,13404288768684385181_u64];
_17 = [13360060119456884971_u64,4716948469506038479_u64];
Goto(bb16)
}
bb16 = {
Call(_32 = dump_var(13_usize, 19_usize, Move(_19), 17_usize, Move(_17), 16_usize, Move(_16), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(13_usize, 6_usize, Move(_6), 20_usize, Move(_20), 33_usize, _33, 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: bool,mut _2: f32,mut _3: f64,mut _4: isize) -> char {
mir! {
type RET = char;
let _5: u8;
let _6: Adt60;
let _7: Adt59;
let _8: [i16; 7];
let _9: (i8, i32, f64, i128);
let _10: f32;
let _11: (i16, [bool; 6], &'static f64);
let _12: [bool; 6];
let _13: (bool, f32, i64, [bool; 6]);
let _14: isize;
let _15: isize;
let _16: f64;
let _17: isize;
let _18: *mut char;
let _19: i64;
let _20: Adt50;
let _21: ();
let _22: ();
{
_3 = (-25_i8) as f64;
RET = '\u{b7dad}';
_5 = 128_u8 - 89_u8;
_3 = (-92_i8) as f64;
_7.fld4.0 = _1;
_2 = 16705345199605345044_u64 as f32;
_7.fld0.fld2 = RET as i16;
_7.fld2.fld1 = [_7.fld4.0,_1,_7.fld4.0,_7.fld4.0,_1,_1];
_7.fld4.2 = (-7078865279202963842_i64);
match _7.fld4.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463456295742152565247614 => bb7,
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
_8 = [_7.fld0.fld2,_7.fld0.fld2,_7.fld0.fld2,_7.fld0.fld2,_7.fld0.fld2,_7.fld0.fld2,_7.fld0.fld2];
_2 = 11253640593246016231_usize as f32;
_5 = !56_u8;
RET = '\u{f3b59}';
_7.fld4 = (_1, _2, (-5342973788789317630_i64), _7.fld2.fld1);
Goto(bb8)
}
bb8 = {
Goto(bb9)
}
bb9 = {
_7.fld2.fld1 = _7.fld4.3;
_1 = _7.fld4.2 <= _7.fld4.2;
_7.fld4.0 = _1 ^ _1;
_7.fld0.fld1 = [4367974356826216025_u64,6328674766688298252_u64];
_1 = !_7.fld4.0;
_7.fld4.2 = !8459409177536890086_i64;
_5 = !252_u8;
_7.fld4.3 = [_7.fld4.0,_1,_1,_7.fld4.0,_7.fld4.0,_7.fld4.0];
_7.fld2.fld0 = _4 as i16;
_9.0 = 20_i8 << _7.fld4.2;
_7.fld2.fld1 = [_7.fld4.0,_7.fld4.0,_1,_1,_7.fld4.0,_7.fld4.0];
_7.fld4.0 = !_1;
_7.fld4.0 = !_1;
_11.1 = _7.fld4.3;
_1 = !_7.fld4.0;
_12 = [_7.fld4.0,_1,_7.fld4.0,_7.fld4.0,_1,_1];
_7.fld4.3 = _11.1;
_11.0 = _7.fld4.2 as i16;
_7.fld2.fld0 = _11.0;
_11.2 = &_3;
_3 = (-930524501_i32) as f64;
_7.fld1 = _7.fld0.fld2 as u32;
_7.fld2.fld1 = _12;
Goto(bb10)
}
bb10 = {
_5 = !98_u8;
_7.fld4 = (_1, _2, 3869625054969044843_i64, _12);
_9.2 = (-1902926640_i32) as f64;
Goto(bb11)
}
bb11 = {
_13 = _7.fld4;
_9.2 = _3 * _3;
_9.2 = -_3;
_7.fld7 = (-115911320956555265280562868978859007726_i128) * 158288857305192737088086661385353947313_i128;
_7.fld4.0 = !_13.0;
_7.fld4 = (_13.0, _2, _13.2, _7.fld2.fld1);
_7.fld0.fld1 = [1746883992403655393_u64,8379787748818749013_u64];
_4 = (-9223372036854775808_isize);
_14 = 21417_u16 as isize;
_7.fld0.fld0 = -_9.0;
RET = '\u{cf2d0}';
Goto(bb12)
}
bb12 = {
_7.fld4.2 = !_13.2;
_7.fld2 = Adt50 { fld0: _11.0,fld1: _13.3 };
_9.3 = !_7.fld7;
_2 = _13.1 + _7.fld4.1;
_9.1 = 778178935_i32 | (-2042586740_i32);
_9.1 = _5 as i32;
_13.1 = _2;
_9.3 = _7.fld7;
_12 = [_13.0,_13.0,_1,_7.fld4.0,_13.0,_1];
_7.fld0.fld1 = [5027915858008150756_u64,10549294757912005768_u64];
_7.fld1 = !1529876328_u32;
_2 = _13.2 as f32;
_7.fld4.0 = _1 | _13.0;
_13 = (_7.fld4.0, _2, _7.fld4.2, _12);
_7.fld0.fld2 = !_7.fld2.fld0;
_7.fld2 = Adt50 { fld0: _7.fld0.fld2,fld1: _11.1 };
_7.fld4.1 = _9.2 as f32;
Goto(bb13)
}
bb13 = {
_9.2 = -_3;
_4 = _14 - _14;
_7.fld2.fld1 = [_1,_1,_7.fld4.0,_7.fld4.0,_1,_13.0];
_13 = _7.fld4;
_9.3 = _13.0 as i128;
Goto(bb14)
}
bb14 = {
_7.fld1 = 639252457_u32;
_9.2 = _3 + _3;
_7.fld4.3 = [_7.fld4.0,_13.0,_13.0,_7.fld4.0,_13.0,_13.0];
_18 = core::ptr::addr_of_mut!(RET);
_13.2 = -_7.fld4.2;
_9.1 = 1714973974_i32 - (-1944490957_i32);
_13 = (_1, _2, _7.fld4.2, _12);
_13.0 = !_1;
_5 = 7_usize as u8;
_16 = _9.2;
_10 = _2;
_13.2 = _7.fld4.2;
_15 = _14 * _4;
_7.fld0.fld0 = _9.0;
_20 = Adt50 { fld0: _11.0,fld1: _7.fld2.fld1 };
_7.fld2.fld1 = [_1,_1,_7.fld4.0,_7.fld4.0,_1,_13.0];
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(14_usize, 12_usize, Move(_12), 8_usize, Move(_8), 14_usize, Move(_14), 22_usize, _22), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: f64,mut _2: f64,mut _3: f64,mut _4: f64,mut _5: f64,mut _6: f64,mut _7: f64,mut _8: f64,mut _9: (f64,),mut _10: f64,mut _11: f64) -> *const u32 {
mir! {
type RET = *const u32;
let _12: *const isize;
let _13: isize;
let _14: Adt44;
let _15: u128;
let _16: i64;
let _17: i64;
let _18: Adt49;
let _19: u32;
let _20: (i16, [bool; 6], &'static f64);
let _21: [u64; 8];
let _22: char;
let _23: i16;
let _24: Adt47;
let _25: isize;
let _26: *mut u16;
let _27: isize;
let _28: Adt53;
let _29: [i16; 7];
let _30: Adt56;
let _31: [char; 6];
let _32: Adt54;
let _33: i16;
let _34: isize;
let _35: u32;
let _36: [u64; 2];
let _37: [u32; 8];
let _38: f32;
let _39: i16;
let _40: bool;
let _41: Adt53;
let _42: ();
let _43: ();
{
_8 = 47_u8 as f64;
_9 = (_2,);
_9.0 = _3;
_11 = -_1;
_11 = _10;
_11 = _7;
_5 = (-18604_i16) as f64;
_5 = _7;
_9 = (_5,);
_9.0 = _1;
_9 = (_10,);
_4 = 6154933728595334988_u64 as f64;
_12 = core::ptr::addr_of!(_13);
(*_12) = (-9223372036854775808_isize);
_9 = (_10,);
Call((*_12) = core::intrinsics::bswap(9223372036854775807_isize), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = _7;
_13 = (-9223372036854775808_isize);
_11 = _5;
_4 = _10 + _10;
_10 = _1;
_8 = _1;
_2 = _6;
(*_12) = -(-9223372036854775808_isize);
(*_12) = !(-9223372036854775808_isize);
(*_12) = 29_i8 as isize;
(*_12) = -9223372036854775807_isize;
_7 = 3_i8 as f64;
_9 = (_11,);
_7 = _13 as f64;
_10 = -_8;
_4 = 7_usize as f64;
_7 = -_8;
_2 = _7;
_5 = -_6;
_4 = 7103714143757217953_u64 as f64;
_7 = _6;
Goto(bb2)
}
bb2 = {
_11 = (*_12) as f64;
_3 = 5_usize as f64;
_9.0 = _8;
_9.0 = _3;
_7 = _10 * _10;
_3 = -_7;
_8 = -_10;
_4 = -_3;
_8 = _1 + _4;
_16 = -2770778041637665960_i64;
_15 = !230070680608128791009064765753848728959_u128;
_5 = (-103818191429503035272389077467622306057_i128) as f64;
_17 = 2853676667_u32 as i64;
_6 = _15 as f64;
Goto(bb3)
}
bb3 = {
(*_12) = (-10_isize) * (-103_isize);
_6 = _7;
_8 = -_10;
(*_12) = 9223372036854775807_isize;
_7 = _4 - _9.0;
_2 = _1 * _6;
_11 = -_6;
RET = core::ptr::addr_of!(_19);
_1 = _8 - _3;
_13 = _17 as isize;
_10 = _2;
_10 = (-723065673_i32) as f64;
RET = core::ptr::addr_of!((*RET));
Call((*RET) = core::intrinsics::bswap(1772471792_u32), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
(*RET) = 4016825950_u32;
_4 = _6;
_16 = !_17;
_11 = _8 - _1;
(*RET) = 25_u8 as u32;
_2 = -_3;
(*_12) = _6 as isize;
_20.1 = [false,false,true,true,true,false];
(*RET) = 1183015024_u32 >> _13;
_8 = _2;
_1 = 3999_u16 as f64;
(*RET) = 2721022979_u32;
_4 = 55_u8 as f64;
(*RET) = 992858784_u32 << (*_12);
_3 = _8;
(*_12) = !(-9223372036854775808_isize);
_19 = 2154293650_u32 * 3592245407_u32;
(*RET) = 38081744629023519863604292533801137934_i128 as u32;
RET = core::ptr::addr_of!(_19);
_22 = '\u{314a7}';
RET = core::ptr::addr_of!((*RET));
Goto(bb5)
}
bb5 = {
(*RET) = 22425_i16 as u32;
_1 = (-28718618977688733958357097466780595662_i128) as f64;
_20.2 = &_9.0;
_24.fld0 = 31205_u16 as f32;
_4 = 88_i8 as f64;
(*RET) = 114_i8 as u32;
Goto(bb6)
}
bb6 = {
_20.1 = [true,true,false,true,false,true];
_17 = _16;
_7 = _6 + _8;
RET = core::ptr::addr_of!(_19);
_19 = !1359817820_u32;
(*RET) = _15 as u32;
_17 = -_16;
_1 = _6;
_20.2 = &_4;
(*_12) = 27_u8 as isize;
_9 = (_2,);
RET = core::ptr::addr_of!((*RET));
_24.fld0 = 9956620740249193007_u64 as f32;
(*_12) = -9223372036854775807_isize;
_23 = 28_i8 as i16;
_11 = -_6;
_21 = [5872790051334825520_u64,928448234448265717_u64,9733664191103425197_u64,3380405338528599214_u64,7269580172457060062_u64,6862734397290732044_u64,2720261028286502555_u64,6134015367558918993_u64];
_27 = _13 - _13;
_25 = _24.fld0 as isize;
(*RET) = 1674889364_u32;
_11 = -_9.0;
_20.2 = &_4;
match (*RET) {
0 => bb3,
1674889364 => bb8,
_ => bb7
}
}
bb7 = {
(*_12) = (-10_isize) * (-103_isize);
_6 = _7;
_8 = -_10;
(*_12) = 9223372036854775807_isize;
_7 = _4 - _9.0;
_2 = _1 * _6;
_11 = -_6;
RET = core::ptr::addr_of!(_19);
_1 = _8 - _3;
_13 = _17 as isize;
_10 = _2;
_10 = (-723065673_i32) as f64;
RET = core::ptr::addr_of!((*RET));
Call((*RET) = core::intrinsics::bswap(1772471792_u32), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_2 = _8 * _1;
_7 = _9.0 + _9.0;
_12 = core::ptr::addr_of!((*_12));
_13 = _27 ^ _25;
_24.fld1 = _22;
match _19 {
0 => bb6,
1674889364 => bb10,
_ => bb9
}
}
bb9 = {
(*RET) = 4016825950_u32;
_4 = _6;
_16 = !_17;
_11 = _8 - _1;
(*RET) = 25_u8 as u32;
_2 = -_3;
(*_12) = _6 as isize;
_20.1 = [false,false,true,true,true,false];
(*RET) = 1183015024_u32 >> _13;
_8 = _2;
_1 = 3999_u16 as f64;
(*RET) = 2721022979_u32;
_4 = 55_u8 as f64;
(*RET) = 992858784_u32 << (*_12);
_3 = _8;
(*_12) = !(-9223372036854775808_isize);
_19 = 2154293650_u32 * 3592245407_u32;
(*RET) = 38081744629023519863604292533801137934_i128 as u32;
RET = core::ptr::addr_of!(_19);
_22 = '\u{314a7}';
RET = core::ptr::addr_of!((*RET));
Goto(bb5)
}
bb10 = {
_20.2 = &_11;
(*RET) = 4225460321_u32 * 2500876153_u32;
(*RET) = 3578089480_u32 & 4138905262_u32;
_5 = _2 - _11;
_4 = _9.0;
_25 = !_13;
_11 = 45222_u16 as f64;
_10 = _23 as f64;
(*_12) = _27;
_13 = _25;
_30 = Adt56::Variant1 { fld0: _24,fld1: _24.fld1,fld2: (-36835088945784890426858330530542173778_i128),fld3: _3,fld4: _23 };
(*RET) = _16 as u32;
(*RET) = !1429290706_u32;
place!(Field::<i16>(Variant(_30, 1), 4)) = _23;
place!(Field::<i128>(Variant(_30, 1), 2)) = 120228993771488684517885740476287522605_i128;
_9 = (_2,);
_3 = 10826160707419410599_u64 as f64;
_8 = _5 * Field::<f64>(Variant(_30, 1), 3);
Goto(bb11)
}
bb11 = {
_20.0 = _23 | Field::<i16>(Variant(_30, 1), 4);
_6 = Field::<i128>(Variant(_30, 1), 2) as f64;
_9 = (_5,);
_29 = [_20.0,_23,_23,_20.0,_20.0,_20.0,_23];
_21 = [13735381697071825048_u64,1550968415283125639_u64,8092888604336079964_u64,5871782532649849471_u64,15421706675706788785_u64,4637191354491685499_u64,2555024334993479716_u64,13194654929735261240_u64];
place!(Field::<char>(Variant(_30, 1), 1)) = _22;
place!(Field::<i128>(Variant(_30, 1), 2)) = 96082217201601200795518488287827658236_i128 | 104370348681439370829765128708546182079_i128;
SetDiscriminant(_30, 2);
_20.2 = &_9.0;
_31 = [_24.fld1,_22,_22,_24.fld1,_24.fld1,_24.fld1];
_17 = _16;
_15 = _16 as u128;
_9 = (_8,);
_19 = 194812335_u32 - 3648938008_u32;
_24.fld0 = _2 as f32;
_20.2 = &place!(Field::<(f64,)>(Variant(_30, 2), 1)).0;
_25 = (*_12) - (*_12);
place!(Field::<[u16; 8]>(Variant(_30, 2), 6)) = [33860_u16,24674_u16,27179_u16,50541_u16,10962_u16,34694_u16,56312_u16,58479_u16];
(*RET) = 2959565613_u32;
place!(Field::<Adt45>(Variant(_30, 2), 0)).fld2 = 3757187246291764101_u64 as i16;
place!(Field::<[u32; 3]>(Variant(_30, 2), 2)) = [(*RET),_19,(*RET)];
_5 = _2 - _9.0;
_20.2 = &_3;
(*RET) = 3931183852_u32 >> _25;
_10 = -_4;
_11 = _4 - _10;
_27 = -_25;
RET = core::ptr::addr_of!(_19);
place!(Field::<[u16; 8]>(Variant(_30, 2), 6)) = [60448_u16,59232_u16,38478_u16,1553_u16,17145_u16,30926_u16,9864_u16,9115_u16];
Goto(bb12)
}
bb12 = {
place!(Field::<(f64,)>(Variant(_30, 2), 1)).0 = 44378493742243979339394009042503794227_i128 as f64;
(*_12) = -_25;
place!(Field::<Adt45>(Variant(_30, 2), 0)).fld0 = 6260802136407021716_u64 as i8;
place!(Field::<(f64,)>(Variant(_30, 2), 1)) = (_9.0,);
_8 = _2;
_15 = 93048794551275125418644799376758642822_u128 >> (*RET);
(*_12) = !_25;
_10 = Field::<(f64,)>(Variant(_30, 2), 1).0;
place!(Field::<Adt45>(Variant(_30, 2), 0)).fld2 = _20.0;
RET = core::ptr::addr_of!((*RET));
_36 = [14857186888323415514_u64,10385694515975941038_u64];
_20.0 = _23;
Call(place!(Field::<*mut [u64; 7]>(Variant(_30, 2), 4)) = fn16(_1, _8, _10, _20.0, Field::<(f64,)>(Variant(_30, 2), 1).0, _4, _24.fld0, _9.0, _22, _2, _11, Field::<(f64,)>(Variant(_30, 2), 1)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
place!(Field::<Adt45>(Variant(_30, 2), 0)).fld0 = (-49_i8);
_5 = _16 as f64;
_36 = [18091403918064570812_u64,7860682118944996805_u64];
_11 = _1 * _9.0;
RET = core::ptr::addr_of!((*RET));
match Field::<Adt45>(Variant(_30, 2), 0).fld0 {
0 => bb12,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb9,
5 => bb14,
6 => bb15,
340282366920938463463374607431768211407 => bb17,
_ => bb16
}
}
bb14 = {
_11 = (*_12) as f64;
_3 = 5_usize as f64;
_9.0 = _8;
_9.0 = _3;
_7 = _10 * _10;
_3 = -_7;
_8 = -_10;
_4 = -_3;
_8 = _1 + _4;
_16 = -2770778041637665960_i64;
_15 = !230070680608128791009064765753848728959_u128;
_5 = (-103818191429503035272389077467622306057_i128) as f64;
_17 = 2853676667_u32 as i64;
_6 = _15 as f64;
Goto(bb3)
}
bb15 = {
_20.1 = [true,true,false,true,false,true];
_17 = _16;
_7 = _6 + _8;
RET = core::ptr::addr_of!(_19);
_19 = !1359817820_u32;
(*RET) = _15 as u32;
_17 = -_16;
_1 = _6;
_20.2 = &_4;
(*_12) = 27_u8 as isize;
_9 = (_2,);
RET = core::ptr::addr_of!((*RET));
_24.fld0 = 9956620740249193007_u64 as f32;
(*_12) = -9223372036854775807_isize;
_23 = 28_i8 as i16;
_11 = -_6;
_21 = [5872790051334825520_u64,928448234448265717_u64,9733664191103425197_u64,3380405338528599214_u64,7269580172457060062_u64,6862734397290732044_u64,2720261028286502555_u64,6134015367558918993_u64];
_27 = _13 - _13;
_25 = _24.fld0 as isize;
(*RET) = 1674889364_u32;
_11 = -_9.0;
_20.2 = &_4;
match (*RET) {
0 => bb3,
1674889364 => bb8,
_ => bb7
}
}
bb16 = {
(*RET) = 4016825950_u32;
_4 = _6;
_16 = !_17;
_11 = _8 - _1;
(*RET) = 25_u8 as u32;
_2 = -_3;
(*_12) = _6 as isize;
_20.1 = [false,false,true,true,true,false];
(*RET) = 1183015024_u32 >> _13;
_8 = _2;
_1 = 3999_u16 as f64;
(*RET) = 2721022979_u32;
_4 = 55_u8 as f64;
(*RET) = 992858784_u32 << (*_12);
_3 = _8;
(*_12) = !(-9223372036854775808_isize);
_19 = 2154293650_u32 * 3592245407_u32;
(*RET) = 38081744629023519863604292533801137934_i128 as u32;
RET = core::ptr::addr_of!(_19);
_22 = '\u{314a7}';
RET = core::ptr::addr_of!((*RET));
Goto(bb5)
}
bb17 = {
_22 = _24.fld1;
place!(Field::<Adt45>(Variant(_30, 2), 0)).fld1 = _36;
_3 = Field::<Adt45>(Variant(_30, 2), 0).fld2 as f64;
RET = core::ptr::addr_of!(_19);
_2 = 10487207710348535293_u64 as f64;
place!(Field::<Adt49>(Variant(_30, 2), 5)) = Adt49::Variant0 { fld0: 1040772747_i32 };
_2 = -_9.0;
_24.fld1 = _22;
_21 = [872097760090793974_u64,7290752911752474743_u64,3998059627621477607_u64,8179790444698454286_u64,9931106937367596708_u64,15081129296091996909_u64,17182202495434180739_u64,18093567327335455627_u64];
_33 = !Field::<Adt45>(Variant(_30, 2), 0).fld2;
_22 = _24.fld1;
_38 = Field::<Adt45>(Variant(_30, 2), 0).fld0 as f32;
_20.0 = true as i16;
_22 = _24.fld1;
_9.0 = _1;
_6 = -_2;
place!(Field::<[u32; 3]>(Variant(_30, 2), 2)) = [_19,(*RET),(*RET)];
_4 = _9.0;
place!(Field::<Adt49>(Variant(_30, 2), 5)) = Adt49::Variant0 { fld0: 1171061383_i32 };
place!(Field::<[u32; 3]>(Variant(_30, 2), 2)) = [_19,_19,(*RET)];
(*RET) = !1720405342_u32;
Goto(bb18)
}
bb18 = {
Call(_42 = dump_var(15_usize, 33_usize, Move(_33), 15_usize, Move(_15), 31_usize, Move(_31), 29_usize, Move(_29)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_42 = dump_var(15_usize, 13_usize, Move(_13), 27_usize, Move(_27), 17_usize, Move(_17), 43_usize, _43), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: f64,mut _2: f64,mut _3: f64,mut _4: i16,mut _5: f64,mut _6: f64,mut _7: f32,mut _8: f64,mut _9: char,mut _10: f64,mut _11: f64,mut _12: (f64,)) -> *mut [u64; 7] {
mir! {
type RET = *mut [u64; 7];
let _13: u64;
let _14: [i16; 5];
let _15: i64;
let _16: [bool; 6];
let _17: [u64; 7];
let _18: *mut u16;
let _19: u128;
let _20: [i16; 5];
let _21: u16;
let _22: (i8, i32, f64, i128);
let _23: u8;
let _24: char;
let _25: [u16; 8];
let _26: isize;
let _27: (*const &'static f64, [u64; 7]);
let _28: (f64,);
let _29: Adt45;
let _30: isize;
let _31: &'static f64;
let _32: Adt48;
let _33: u16;
let _34: (*const &'static f64, [u64; 7]);
let _35: char;
let _36: (i8, i32, f64, i128);
let _37: f32;
let _38: f32;
let _39: bool;
let _40: ();
let _41: ();
{
_1 = -_10;
_7 = 2086332883_i32 as f32;
_6 = _10 + _8;
_6 = _3 * _1;
_7 = 6418195974460580760_u64 as f32;
Call(_6 = fn17(_12, _2, _5, _11, _11, _12.0, _11, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = 2416659378_u32 as f32;
_10 = -_6;
_12.0 = _2;
_2 = _8 + _12.0;
_9 = '\u{694da}';
_12 = (_2,);
_8 = -_10;
_10 = -_3;
_8 = 666067286703226895_i64 as f64;
_10 = 9223372036854775807_isize as f64;
_2 = 3_u8 as f64;
_3 = _5 + _11;
_4 = 11150198365154491711_u64 as i16;
_7 = 40441_u16 as f32;
_13 = !2474857209982775974_u64;
_8 = -_6;
_12.0 = -_1;
_12.0 = _4 as f64;
_17 = [_13,_13,_13,_13,_13,_13,_13];
_14 = [_4,_4,_4,_4,_4];
_8 = _13 as f64;
_16 = [false,false,true,true,true,false];
_11 = 191654859518389328793622786365336592079_u128 as f64;
RET = core::ptr::addr_of_mut!(_17);
Goto(bb2)
}
bb2 = {
_13 = !10898396401954060795_u64;
_19 = 293595651007029460508444232073804134334_u128 + 77041535177022417575367093584046960161_u128;
_5 = _7 as f64;
RET = core::ptr::addr_of_mut!(_17);
(*RET) = [_13,_13,_13,_13,_13,_13,_13];
_10 = _4 as f64;
_5 = (-565780560_i32) as f64;
_12 = (_1,);
_6 = _12.0;
_1 = (-43_i8) as f64;
(*RET) = [_13,_13,_13,_13,_13,_13,_13];
_4 = -(-26782_i16);
_9 = '\u{cba5c}';
Call((*RET) = fn18(_12.0, _3, _3, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_2 = -_3;
_17 = [_13,_13,_13,_13,_13,_13,_13];
_20 = _14;
_22.3 = 99440931651346012150972901574669997996_i128;
(*RET) = [_13,_13,_13,_13,_13,_13,_13];
_7 = _19 as f32;
_22 = (34_i8, (-563627703_i32), _3, 37955877760914534229633631914847382393_i128);
_19 = 1402359658_u32 as u128;
_12 = (_6,);
_8 = -_3;
_11 = _2 - _8;
_15 = _9 as i64;
_6 = _19 as f64;
_8 = (-85_isize) as f64;
RET = core::ptr::addr_of_mut!(_17);
_23 = !208_u8;
_22.3 = (-38624419456163598151767847216843229519_i128) * (-49077875585377427441598685489986883320_i128);
_22.3 = (-115829993046408520228735960890452661210_i128) - 133191000921835638570571483587647687546_i128;
_9 = '\u{1bf45}';
_24 = _9;
RET = core::ptr::addr_of_mut!((*RET));
match _22.0 {
0 => bb1,
34 => bb4,
_ => bb2
}
}
bb4 = {
(*RET) = [_13,_13,_13,_13,_13,_13,_13];
_18 = core::ptr::addr_of_mut!(_21);
(*_18) = _22.1 as u16;
_13 = 9576428354861082207_u64;
Goto(bb5)
}
bb5 = {
_3 = _11 * _12.0;
_16 = [false,true,false,false,true,true];
_12.0 = 9223372036854775807_isize as f64;
(*RET) = [_13,_13,_13,_13,_13,_13,_13];
(*RET) = [_13,_13,_13,_13,_13,_13,_13];
_5 = _22.2 + _22.2;
_25 = [(*_18),(*_18),_21,_21,(*_18),(*_18),(*_18),(*_18)];
_15 = _24 as i64;
(*RET) = [_13,_13,_13,_13,_13,_13,_13];
_2 = _22.2;
_22.3 = (-101059173274235153427884136648773159109_i128);
_8 = 3510137184_u32 as f64;
_8 = _4 as f64;
_27.1 = [_13,_13,_13,_13,_13,_13,_13];
_29.fld2 = !_4;
_16 = [false,true,true,true,true,false];
_28 = (_3,);
_24 = _9;
match _22.1 {
0 => bb1,
1 => bb4,
2 => bb6,
3 => bb7,
340282366920938463463374607431204583753 => bb9,
_ => bb8
}
}
bb6 = {
_7 = 2416659378_u32 as f32;
_10 = -_6;
_12.0 = _2;
_2 = _8 + _12.0;
_9 = '\u{694da}';
_12 = (_2,);
_8 = -_10;
_10 = -_3;
_8 = 666067286703226895_i64 as f64;
_10 = 9223372036854775807_isize as f64;
_2 = 3_u8 as f64;
_3 = _5 + _11;
_4 = 11150198365154491711_u64 as i16;
_7 = 40441_u16 as f32;
_13 = !2474857209982775974_u64;
_8 = -_6;
_12.0 = -_1;
_12.0 = _4 as f64;
_17 = [_13,_13,_13,_13,_13,_13,_13];
_14 = [_4,_4,_4,_4,_4];
_8 = _13 as f64;
_16 = [false,false,true,true,true,false];
_11 = 191654859518389328793622786365336592079_u128 as f64;
RET = core::ptr::addr_of_mut!(_17);
Goto(bb2)
}
bb7 = {
_2 = -_3;
_17 = [_13,_13,_13,_13,_13,_13,_13];
_20 = _14;
_22.3 = 99440931651346012150972901574669997996_i128;
(*RET) = [_13,_13,_13,_13,_13,_13,_13];
_7 = _19 as f32;
_22 = (34_i8, (-563627703_i32), _3, 37955877760914534229633631914847382393_i128);
_19 = 1402359658_u32 as u128;
_12 = (_6,);
_8 = -_3;
_11 = _2 - _8;
_15 = _9 as i64;
_6 = _19 as f64;
_8 = (-85_isize) as f64;
RET = core::ptr::addr_of_mut!(_17);
_23 = !208_u8;
_22.3 = (-38624419456163598151767847216843229519_i128) * (-49077875585377427441598685489986883320_i128);
_22.3 = (-115829993046408520228735960890452661210_i128) - 133191000921835638570571483587647687546_i128;
_9 = '\u{1bf45}';
_24 = _9;
RET = core::ptr::addr_of_mut!((*RET));
match _22.0 {
0 => bb1,
34 => bb4,
_ => bb2
}
}
bb8 = {
_13 = !10898396401954060795_u64;
_19 = 293595651007029460508444232073804134334_u128 + 77041535177022417575367093584046960161_u128;
_5 = _7 as f64;
RET = core::ptr::addr_of_mut!(_17);
(*RET) = [_13,_13,_13,_13,_13,_13,_13];
_10 = _4 as f64;
_5 = (-565780560_i32) as f64;
_12 = (_1,);
_6 = _12.0;
_1 = (-43_i8) as f64;
(*RET) = [_13,_13,_13,_13,_13,_13,_13];
_4 = -(-26782_i16);
_9 = '\u{cba5c}';
Call((*RET) = fn18(_12.0, _3, _3, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_28.0 = -_11;
_28.0 = _5;
RET = core::ptr::addr_of_mut!((*RET));
_15 = _9 as i64;
_1 = _15 as f64;
_9 = _24;
_25 = [(*_18),_21,_21,(*_18),(*_18),(*_18),_21,(*_18)];
_14 = _20;
_32.fld4 = (true, _7, _15, _16);
_31 = &_6;
_29.fld0 = -_22.0;
_34.1 = [_13,_13,_13,_13,_13,_13,_13];
match _22.1 {
0 => bb6,
1 => bb10,
2 => bb11,
340282366920938463463374607431204583753 => bb13,
_ => bb12
}
}
bb10 = {
_3 = _11 * _12.0;
_16 = [false,true,false,false,true,true];
_12.0 = 9223372036854775807_isize as f64;
(*RET) = [_13,_13,_13,_13,_13,_13,_13];
(*RET) = [_13,_13,_13,_13,_13,_13,_13];
_5 = _22.2 + _22.2;
_25 = [(*_18),(*_18),_21,_21,(*_18),(*_18),(*_18),(*_18)];
_15 = _24 as i64;
(*RET) = [_13,_13,_13,_13,_13,_13,_13];
_2 = _22.2;
_22.3 = (-101059173274235153427884136648773159109_i128);
_8 = 3510137184_u32 as f64;
_8 = _4 as f64;
_27.1 = [_13,_13,_13,_13,_13,_13,_13];
_29.fld2 = !_4;
_16 = [false,true,true,true,true,false];
_28 = (_3,);
_24 = _9;
match _22.1 {
0 => bb1,
1 => bb4,
2 => bb6,
3 => bb7,
340282366920938463463374607431204583753 => bb9,
_ => bb8
}
}
bb11 = {
_2 = -_3;
_17 = [_13,_13,_13,_13,_13,_13,_13];
_20 = _14;
_22.3 = 99440931651346012150972901574669997996_i128;
(*RET) = [_13,_13,_13,_13,_13,_13,_13];
_7 = _19 as f32;
_22 = (34_i8, (-563627703_i32), _3, 37955877760914534229633631914847382393_i128);
_19 = 1402359658_u32 as u128;
_12 = (_6,);
_8 = -_3;
_11 = _2 - _8;
_15 = _9 as i64;
_6 = _19 as f64;
_8 = (-85_isize) as f64;
RET = core::ptr::addr_of_mut!(_17);
_23 = !208_u8;
_22.3 = (-38624419456163598151767847216843229519_i128) * (-49077875585377427441598685489986883320_i128);
_22.3 = (-115829993046408520228735960890452661210_i128) - 133191000921835638570571483587647687546_i128;
_9 = '\u{1bf45}';
_24 = _9;
RET = core::ptr::addr_of_mut!((*RET));
match _22.0 {
0 => bb1,
34 => bb4,
_ => bb2
}
}
bb12 = {
_7 = 2416659378_u32 as f32;
_10 = -_6;
_12.0 = _2;
_2 = _8 + _12.0;
_9 = '\u{694da}';
_12 = (_2,);
_8 = -_10;
_10 = -_3;
_8 = 666067286703226895_i64 as f64;
_10 = 9223372036854775807_isize as f64;
_2 = 3_u8 as f64;
_3 = _5 + _11;
_4 = 11150198365154491711_u64 as i16;
_7 = 40441_u16 as f32;
_13 = !2474857209982775974_u64;
_8 = -_6;
_12.0 = -_1;
_12.0 = _4 as f64;
_17 = [_13,_13,_13,_13,_13,_13,_13];
_14 = [_4,_4,_4,_4,_4];
_8 = _13 as f64;
_16 = [false,false,true,true,true,false];
_11 = 191654859518389328793622786365336592079_u128 as f64;
RET = core::ptr::addr_of_mut!(_17);
Goto(bb2)
}
bb13 = {
_27.0 = core::ptr::addr_of!(_31);
(*RET) = _27.1;
_32.fld4.0 = false;
_14 = [_4,_29.fld2,_4,_4,_4];
_33 = _21 << _29.fld0;
(*_18) = !_33;
_32.fld0.fld0 = -_29.fld0;
_34.1 = _17;
_15 = _32.fld4.2;
RET = core::ptr::addr_of_mut!(_34.1);
_32.fld4.1 = _7 + _7;
_5 = -_28.0;
_32.fld0.fld2 = _29.fld2;
_26 = 1597561481_u32 as isize;
_9 = _24;
_14 = _20;
_22.3 = -(-166767080450158121197135405021624151001_i128);
_36.1 = _22.1 | _22.1;
_23 = !189_u8;
_36.2 = -_3;
_22.2 = _11 + _5;
_32.fld5 = [(*_18),_21,(*_18),(*_18),(*_18),(*_18),(*_18),(*_18)];
_32.fld4.3 = [_32.fld4.0,_32.fld4.0,_32.fld4.0,_32.fld4.0,_32.fld4.0,_32.fld4.0];
_32.fld0.fld1 = [_13,_13];
_32.fld0.fld0 = _29.fld0;
_31 = &_5;
_2 = _36.2 - _3;
_34.0 = core::ptr::addr_of!(_31);
_17 = [_13,_13,_13,_13,_13,_13,_13];
_36.0 = -_29.fld0;
match _22.0 {
0 => bb12,
1 => bb9,
2 => bb3,
3 => bb4,
34 => bb15,
_ => bb14
}
}
bb14 = {
_13 = !10898396401954060795_u64;
_19 = 293595651007029460508444232073804134334_u128 + 77041535177022417575367093584046960161_u128;
_5 = _7 as f64;
RET = core::ptr::addr_of_mut!(_17);
(*RET) = [_13,_13,_13,_13,_13,_13,_13];
_10 = _4 as f64;
_5 = (-565780560_i32) as f64;
_12 = (_1,);
_6 = _12.0;
_1 = (-43_i8) as f64;
(*RET) = [_13,_13,_13,_13,_13,_13,_13];
_4 = -(-26782_i16);
_9 = '\u{cba5c}';
Call((*RET) = fn18(_12.0, _3, _3, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_32.fld4 = (false, _7, _15, _16);
_35 = _9;
_28.0 = _19 as f64;
_34.0 = core::ptr::addr_of!(_31);
(*RET) = _27.1;
_10 = _3;
_23 = !82_u8;
_19 = !160508446397515567881881133374255185097_u128;
RET = core::ptr::addr_of_mut!((*RET));
_21 = _33;
_32.fld1 = _24;
_27.1 = [_13,_13,_13,_13,_13,_13,_13];
_14 = _20;
_6 = _36.2;
_27.1 = [_13,_13,_13,_13,_13,_13,_13];
_30 = _26 - _26;
(*RET) = [_13,_13,_13,_13,_13,_13,_13];
Goto(bb16)
}
bb16 = {
Call(_40 = dump_var(16_usize, 13_usize, Move(_13), 23_usize, Move(_23), 14_usize, Move(_14), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(16_usize, 35_usize, Move(_35), 33_usize, Move(_33), 19_usize, Move(_19), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: (f64,),mut _2: f64,mut _3: f64,mut _4: f64,mut _5: f64,mut _6: f64,mut _7: f64,mut _8: f64) -> f64 {
mir! {
type RET = f64;
let _9: isize;
let _10: ();
let _11: ();
{
_8 = -_7;
_9 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
RET = -_2;
_5 = RET;
_8 = _1.0 + _1.0;
_2 = _6 + RET;
_6 = _7;
_3 = _4 + _7;
_7 = 3837173095375323111_u64 as f64;
_1.0 = _2;
_7 = _2;
_4 = _2 - _5;
_4 = -_3;
_5 = -_1.0;
_4 = -_2;
RET = _6;
RET = (-74321246932486520330062595816140018524_i128) as f64;
_9 = (-82_isize);
RET = _1.0;
_2 = _1.0;
Goto(bb1)
}
bb1 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: f64,mut _2: f64,mut _3: f64,mut _4: f64) -> [u64; 7] {
mir! {
type RET = [u64; 7];
let _5: i16;
let _6: u64;
let _7: Adt50;
let _8: *const &'static f64;
let _9: f32;
let _10: [u32; 3];
let _11: (bool, f32, i64, [bool; 6]);
let _12: f64;
let _13: u16;
let _14: Adt55;
let _15: i128;
let _16: u128;
let _17: i32;
let _18: f32;
let _19: [i16; 7];
let _20: [u64; 7];
let _21: i8;
let _22: i32;
let _23: [u16; 8];
let _24: bool;
let _25: Adt60;
let _26: f64;
let _27: u128;
let _28: isize;
let _29: Adt50;
let _30: f64;
let _31: i32;
let _32: (f64,);
let _33: *const &'static f64;
let _34: isize;
let _35: Adt51;
let _36: Adt51;
let _37: [i64; 8];
let _38: Adt47;
let _39: char;
let _40: *mut char;
let _41: *mut u16;
let _42: char;
let _43: [char; 6];
let _44: [u32; 8];
let _45: char;
let _46: ();
let _47: ();
{
_1 = _2 + _3;
RET = [2162442306279635063_u64,10643218878598034105_u64,13330035982192945096_u64,17118414118432689353_u64,10994095938998887845_u64,6708164058658446562_u64,9346797087646490665_u64];
_1 = -_4;
RET = [16654936297905678156_u64,1994792620278967694_u64,18116250331509568236_u64,7423221116491136022_u64,10700225251423732947_u64,17554171287465488623_u64,14443489287918302693_u64];
RET = [7711598973108672693_u64,12624238922412829698_u64,3618593878087419390_u64,8473534934113493878_u64,4605322653654360435_u64,340059809587256429_u64,7812614100642832883_u64];
_7.fld0 = '\u{21dd9}' as i16;
_5 = _7.fld0;
_6 = 9538761602529997940_u64 - 5159108740214222202_u64;
RET = [_6,_6,_6,_6,_6,_6,_6];
_1 = 2289653816455051945_i64 as f64;
_7.fld1 = [false,false,true,false,true,false];
_7.fld0 = _5;
_6 = !15925192825141641430_u64;
_7.fld0 = _5 & _5;
RET = [_6,_6,_6,_6,_6,_6,_6];
_7.fld0 = !_5;
_3 = _4 + _2;
_7.fld0 = 2598154266_u32 as i16;
_10 = [2771843169_u32,874776559_u32,127982770_u32];
_4 = 56484087128852174091096697831163051586_u128 as f64;
_4 = _2;
_4 = _2 * _2;
_10 = [2418064583_u32,1104018881_u32,1660290090_u32];
_3 = 276008857269326661798234664039756761458_u128 as f64;
Goto(bb1)
}
bb1 = {
_10 = [220893150_u32,746514943_u32,1941908979_u32];
_4 = _2 - _2;
RET = [_6,_6,_6,_6,_6,_6,_6];
_9 = (-1405745570_i32) as f32;
_7.fld0 = !_5;
RET = [_6,_6,_6,_6,_6,_6,_6];
_9 = 5224099607674274973_i64 as f32;
_7.fld0 = _6 as i16;
_9 = _5 as f32;
_6 = 33_u8 as u64;
_6 = !10190488008935896326_u64;
_3 = -_2;
Goto(bb2)
}
bb2 = {
_11.1 = -_9;
Goto(bb3)
}
bb3 = {
_7.fld1 = [true,false,true,true,true,false];
_11.3 = _7.fld1;
RET = [_6,_6,_6,_6,_6,_6,_6];
_13 = !21324_u16;
_11.0 = !false;
_2 = _3;
_15 = (-102030107505587463387544746808170449369_i128) - 54454772518091253473916935913127559911_i128;
_15 = '\u{559b3}' as i128;
_10 = [3034150675_u32,1198829124_u32,530257424_u32];
_7 = Adt50 { fld0: _5,fld1: _11.3 };
_11.0 = _2 <= _2;
_11 = (true, _9, 1734754577345420485_i64, _7.fld1);
RET = [_6,_6,_6,_6,_6,_6,_6];
_12 = _2;
_16 = _2 as u128;
_7.fld0 = _5 >> _16;
_13 = 33831_u16;
_11.2 = (-2517480817622181784_i64);
_11 = (true, _9, 2461198934819571430_i64, _7.fld1);
_10 = [158147091_u32,3858886409_u32,2456165612_u32];
_15 = 26955240227598315060769275865589159370_i128 & 85277645922174913566865793724273020741_i128;
_11.1 = _9 - _9;
match _11.2 {
0 => bb1,
1 => bb2,
2 => bb4,
2461198934819571430 => bb6,
_ => bb5
}
}
bb4 = {
_11.1 = -_9;
Goto(bb3)
}
bb5 = {
_10 = [220893150_u32,746514943_u32,1941908979_u32];
_4 = _2 - _2;
RET = [_6,_6,_6,_6,_6,_6,_6];
_9 = (-1405745570_i32) as f32;
_7.fld0 = !_5;
RET = [_6,_6,_6,_6,_6,_6,_6];
_9 = 5224099607674274973_i64 as f32;
_7.fld0 = _6 as i16;
_9 = _5 as f32;
_6 = 33_u8 as u64;
_6 = !10190488008935896326_u64;
_3 = -_2;
Goto(bb2)
}
bb6 = {
_17 = 63859182_i32 | 2145805667_i32;
_1 = -_2;
_17 = 1744816685_i32 << _7.fld0;
_11 = (true, _9, (-7356625085864359613_i64), _7.fld1);
_19 = [_7.fld0,_7.fld0,_7.fld0,_7.fld0,_7.fld0,_7.fld0,_7.fld0];
RET = [_6,_6,_6,_6,_6,_6,_6];
_18 = -_11.1;
_18 = _7.fld0 as f32;
_13 = 12487_u16 | 53256_u16;
_21 = 34_i8;
_4 = _18 as f64;
_11.2 = 2616345566877479119_i64 + 3502927543196376568_i64;
_11.3 = [_11.0,_11.0,_11.0,_11.0,_11.0,_11.0];
RET = [_6,_6,_6,_6,_6,_6,_6];
_11.3 = _7.fld1;
_16 = 329505659407151239265244333017978437130_u128;
_20 = [_6,_6,_6,_6,_6,_6,_6];
_5 = _7.fld0 << _11.2;
_10 = [2249938547_u32,3550154985_u32,914716904_u32];
_19 = [_5,_5,_5,_7.fld0,_7.fld0,_5,_5];
_7.fld0 = _5;
_3 = _2;
_22 = !_17;
_19 = [_7.fld0,_7.fld0,_7.fld0,_5,_5,_7.fld0,_5];
_22 = _17 << _7.fld0;
_21 = _5 as i8;
_15 = (-148291690804520303308014298167991696928_i128) & 68677166151155150986162061493268809187_i128;
_7 = Adt50 { fld0: _5,fld1: _11.3 };
Call(_21 = core::intrinsics::bswap(10_i8), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_13 = 4339_u16 * 19865_u16;
_7 = Adt50 { fld0: _5,fld1: _11.3 };
_4 = _18 as f64;
_3 = _4 + _4;
_15 = _11.2 as i128;
_2 = _4 - _1;
_4 = _2 + _2;
_3 = -_4;
_11.3 = [_11.0,_11.0,_11.0,_11.0,_11.0,_11.0];
_11 = (false, _18, 5593218303719739113_i64, _7.fld1);
_9 = _11.1 * _18;
_13 = !36946_u16;
_21 = !(-70_i8);
_27 = _22 as u128;
_11.2 = 2164538476843852567_i64 + (-6128539627889421631_i64);
_1 = _4 - _2;
_15 = _6 as i128;
_19 = [_5,_7.fld0,_5,_7.fld0,_5,_7.fld0,_7.fld0];
_30 = -_3;
match _16 {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb4,
329505659407151239265244333017978437130 => bb9,
_ => bb8
}
}
bb8 = {
_11.1 = -_9;
Goto(bb3)
}
bb9 = {
_27 = _15 as u128;
_24 = !_11.0;
_11.3 = _7.fld1;
_24 = _11.0 & _11.0;
_16 = !_27;
_4 = _30;
_29.fld1 = _7.fld1;
_17 = _11.0 as i32;
_23 = [_13,_13,_13,_13,_13,_13,_13,_13];
_29.fld0 = -_5;
_7.fld0 = _5 ^ _5;
_6 = !11443360375595023284_u64;
_6 = 10636663031361234892_u64 ^ 3552902312392772226_u64;
_29.fld0 = _5 | _7.fld0;
Goto(bb10)
}
bb10 = {
_31 = -_22;
_24 = !_11.0;
_30 = _17 as f64;
_4 = _30 + _1;
_3 = -_4;
_15 = -(-14839657612761131883575978680953292283_i128);
_3 = _1 - _12;
_34 = 13_isize;
_18 = _9;
_16 = _27;
_11.0 = !_24;
_32 = (_2,);
_3 = _1 * _30;
_4 = _2;
_15 = (-146246732197937100315232722742908112151_i128) | 58893212603553459757031653420797138556_i128;
_10 = [821631437_u32,3179256770_u32,4279535778_u32];
_21 = (-30_i8) - (-49_i8);
_32.0 = _6 as f64;
RET = [_6,_6,_6,_6,_6,_6,_6];
match _34 {
13 => bb11,
_ => bb6
}
}
bb11 = {
_9 = _11.1;
_11.1 = _21 as f32;
_10 = [1506070105_u32,3302899702_u32,2475651655_u32];
_9 = -_18;
_11.0 = !_24;
_37 = [_11.2,_11.2,_11.2,_11.2,_11.2,_11.2,_11.2,_11.2];
_16 = _27;
_28 = !_34;
_32.0 = _1;
_32 = (_1,);
_26 = _13 as f64;
_34 = _28;
_31 = !_17;
Goto(bb12)
}
bb12 = {
_38.fld1 = '\u{321d6}';
_38.fld0 = _9 + _9;
_31 = !_22;
_13 = 62591_u16;
_29 = Adt50 { fld0: _7.fld0,fld1: _7.fld1 };
_3 = _6 as f64;
_2 = _6 as f64;
_23 = [_13,_13,_13,_13,_13,_13,_13,_13];
_27 = _16 >> _5;
_28 = _34;
_38.fld0 = _21 as f32;
_38.fld1 = '\u{68de1}';
_6 = _13 as u64;
match _13 {
0 => bb5,
1 => bb2,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
62591 => bb18,
_ => bb17
}
}
bb13 = {
_11.1 = -_9;
Goto(bb3)
}
bb14 = {
_31 = -_22;
_24 = !_11.0;
_30 = _17 as f64;
_4 = _30 + _1;
_3 = -_4;
_15 = -(-14839657612761131883575978680953292283_i128);
_3 = _1 - _12;
_34 = 13_isize;
_18 = _9;
_16 = _27;
_11.0 = !_24;
_32 = (_2,);
_3 = _1 * _30;
_4 = _2;
_15 = (-146246732197937100315232722742908112151_i128) | 58893212603553459757031653420797138556_i128;
_10 = [821631437_u32,3179256770_u32,4279535778_u32];
_21 = (-30_i8) - (-49_i8);
_32.0 = _6 as f64;
RET = [_6,_6,_6,_6,_6,_6,_6];
match _34 {
13 => bb11,
_ => bb6
}
}
bb15 = {
_27 = _15 as u128;
_24 = !_11.0;
_11.3 = _7.fld1;
_24 = _11.0 & _11.0;
_16 = !_27;
_4 = _30;
_29.fld1 = _7.fld1;
_17 = _11.0 as i32;
_23 = [_13,_13,_13,_13,_13,_13,_13,_13];
_29.fld0 = -_5;
_7.fld0 = _5 ^ _5;
_6 = !11443360375595023284_u64;
_6 = 10636663031361234892_u64 ^ 3552902312392772226_u64;
_29.fld0 = _5 | _7.fld0;
Goto(bb10)
}
bb16 = {
_11.1 = -_9;
Goto(bb3)
}
bb17 = {
_10 = [220893150_u32,746514943_u32,1941908979_u32];
_4 = _2 - _2;
RET = [_6,_6,_6,_6,_6,_6,_6];
_9 = (-1405745570_i32) as f32;
_7.fld0 = !_5;
RET = [_6,_6,_6,_6,_6,_6,_6];
_9 = 5224099607674274973_i64 as f32;
_7.fld0 = _6 as i16;
_9 = _5 as f32;
_6 = 33_u8 as u64;
_6 = !10190488008935896326_u64;
_3 = -_2;
Goto(bb2)
}
bb18 = {
_29 = Adt50 { fld0: _5,fld1: _11.3 };
_29 = Adt50 { fld0: _7.fld0,fld1: _7.fld1 };
_34 = !_28;
_41 = core::ptr::addr_of_mut!(_13);
_16 = !_27;
_23 = [(*_41),(*_41),(*_41),(*_41),_13,_13,(*_41),(*_41)];
_3 = _30;
_6 = 4460952129300793602_u64 + 11651239277652007869_u64;
_27 = !_16;
_31 = -_22;
(*_41) = 4451_u16 << _27;
_11.0 = !_24;
_30 = _1 - _32.0;
_7.fld1 = [_24,_11.0,_11.0,_11.0,_11.0,_11.0];
_10 = [3494181430_u32,2542816421_u32,1197086550_u32];
_2 = -_12;
_32.0 = -_12;
_37 = [_11.2,_11.2,_11.2,_11.2,_11.2,_11.2,_11.2,_11.2];
_24 = !_11.0;
(*_41) = !29561_u16;
_19 = [_29.fld0,_29.fld0,_29.fld0,_29.fld0,_5,_5,_5];
_29 = Adt50 { fld0: _5,fld1: _7.fld1 };
_38.fld1 = '\u{f11b7}';
_13 = 5061_u16 ^ 54485_u16;
Goto(bb19)
}
bb19 = {
Call(_46 = dump_var(18_usize, 5_usize, Move(_5), 23_usize, Move(_23), 19_usize, Move(_19), 24_usize, Move(_24)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_46 = dump_var(18_usize, 20_usize, Move(_20), 21_usize, Move(_21), 34_usize, Move(_34), 16_usize, Move(_16)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_46 = dump_var(18_usize, 13_usize, Move(_13), 47_usize, _47, 47_usize, _47, 47_usize, _47), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: f64,mut _2: bool,mut _3: i16) -> f32 {
mir! {
type RET = f32;
let _4: char;
let _5: [u64; 7];
let _6: Adt45;
let _7: char;
let _8: Adt47;
let _9: [char; 6];
let _10: [i64; 8];
let _11: *const u32;
let _12: f32;
let _13: [u32; 8];
let _14: Adt55;
let _15: u16;
let _16: Adt45;
let _17: [char; 6];
let _18: *const i8;
let _19: [i16; 7];
let _20: [u32; 3];
let _21: (u64, i128, i8, &'static f64, bool);
let _22: bool;
let _23: *mut u16;
let _24: Adt50;
let _25: Adt50;
let _26: (bool, f32, i64, [bool; 6]);
let _27: [char; 6];
let _28: [u64; 2];
let _29: f32;
let _30: f32;
let _31: Adt50;
let _32: [i16; 7];
let _33: (i16, [bool; 6], &'static f64);
let _34: Adt52;
let _35: u64;
let _36: u64;
let _37: u128;
let _38: (i16, [bool; 6], &'static f64);
let _39: f64;
let _40: Adt45;
let _41: (f64,);
let _42: [i16; 5];
let _43: isize;
let _44: ();
let _45: ();
{
RET = 22497_u16 as f32;
_3 = (-24530_i16) + (-7720_i16);
_2 = _1 >= _1;
_3 = (-26128_i16);
_3 = 28984_i16 | (-11257_i16);
_4 = '\u{101fa}';
_4 = '\u{59161}';
_3 = (-22516_i16);
_4 = '\u{47551}';
_3 = (-96_i8) as i16;
_1 = 12206620011588323527_usize as f64;
RET = 57963004036277480959694275868773035605_i128 as f32;
_4 = '\u{ddcfd}';
_1 = 258278099170863779130140965762830804180_u128 as f64;
_6.fld0 = !70_i8;
_1 = 1337563520_i32 as f64;
_6.fld1 = [6590825886629301422_u64,16701885405815091247_u64];
_5 = [7735350256192339233_u64,17902036792528345849_u64,10736662365935116620_u64,3269404306638345880_u64,5275762661044278191_u64,12659246783277228835_u64,14926223723760036539_u64];
_1 = 6_usize as f64;
_6.fld2 = !_3;
_3 = !_6.fld2;
_3 = _6.fld2;
RET = 17982047376012528836_u64 as f32;
_3 = -_6.fld2;
_2 = _3 <= _3;
_4 = '\u{2bcb2}';
RET = (-2745868085691468426_i64) as f32;
RET = 233_u8 as f32;
Goto(bb1)
}
bb1 = {
_9 = [_4,_4,_4,_4,_4,_4];
_8 = Adt47 { fld0: RET,fld1: _4 };
Goto(bb2)
}
bb2 = {
_6.fld2 = _3;
_3 = !_6.fld2;
RET = -_8.fld0;
_8.fld0 = -RET;
_6.fld1 = [9456460496264938209_u64,14423478030379278756_u64];
_1 = 9384611151125114323_usize as f64;
RET = 41109499981486138979583635672917725405_u128 as f32;
_7 = _8.fld1;
_2 = true;
_6.fld1 = [11696209031055666956_u64,5211099387323453773_u64];
RET = _6.fld0 as f32;
_8 = Adt47 { fld0: RET,fld1: _4 };
_12 = RET;
_6.fld0 = -(-79_i8);
_10 = [(-7027022836672203575_i64),7697699833448437411_i64,(-3569622190885127982_i64),(-5922971247913663039_i64),3178956556117202530_i64,5541462915681663760_i64,707046674811870704_i64,1705527089752982160_i64];
_4 = _8.fld1;
Goto(bb3)
}
bb3 = {
_2 = false ^ false;
_6.fld0 = -(-51_i8);
_6.fld0 = !72_i8;
_13 = [2453523565_u32,3207075096_u32,3371666133_u32,2750029727_u32,57451242_u32,3856524033_u32,565177323_u32,432145459_u32];
_3 = _6.fld2 * _6.fld2;
RET = -_8.fld0;
_8.fld0 = RET - _12;
_3 = _6.fld2 << _6.fld0;
_12 = -RET;
RET = -_8.fld0;
_7 = _8.fld1;
_7 = _4;
_3 = 61928686162267557270703784971655700770_i128 as i16;
_8.fld0 = -RET;
_3 = _6.fld2 >> _6.fld0;
_16.fld2 = _3 >> _3;
_13 = [4140115373_u32,3867922038_u32,3686255559_u32,633453641_u32,2164833287_u32,2566653806_u32,2617063482_u32,2156813562_u32];
_16 = Adt45 { fld0: _6.fld0,fld1: _6.fld1,fld2: _3 };
_4 = _8.fld1;
_13 = [526235570_u32,1528482465_u32,1393809307_u32,2303326149_u32,2118283248_u32,2825546459_u32,2329864210_u32,3516773362_u32];
_12 = _8.fld0;
_17 = _9;
_9 = [_4,_8.fld1,_4,_7,_7,_4];
_15 = !56541_u16;
Goto(bb4)
}
bb4 = {
_16 = _6;
_1 = 4237412274_u32 as f64;
_2 = false;
_2 = true | false;
_20 = [2788173967_u32,4239450479_u32,178804267_u32];
_4 = _7;
_13 = [992821143_u32,3723223124_u32,3747273942_u32,1089855285_u32,257469076_u32,1049825522_u32,2406145692_u32,2933937570_u32];
_16.fld1 = [9640047537563530021_u64,10450016603205723531_u64];
_4 = _7;
_15 = 1340175351_i32 as u16;
_16.fld2 = -_6.fld2;
_16.fld1 = [13822373907191454533_u64,5416299374509812685_u64];
_9 = [_4,_7,_4,_8.fld1,_8.fld1,_8.fld1];
_13 = [991279588_u32,2531890991_u32,812161910_u32,3874783731_u32,1006536928_u32,995234344_u32,2440568153_u32,3398846159_u32];
_21.4 = _2 | _2;
_6.fld1 = [13802591819684199952_u64,564924902232786327_u64];
_4 = _8.fld1;
_19 = [_16.fld2,_3,_3,_16.fld2,_16.fld2,_3,_6.fld2];
_19 = [_3,_16.fld2,_3,_3,_6.fld2,_3,_6.fld2];
_20 = [549037359_u32,3851665770_u32,2522411274_u32];
_8.fld0 = -_12;
_12 = -_8.fld0;
_21.2 = !_6.fld0;
Goto(bb5)
}
bb5 = {
_24.fld0 = _1 as i16;
_10 = [(-8231583528209159780_i64),8552868316491520549_i64,8914886690274289583_i64,(-3163222718976532850_i64),2609139423339405090_i64,(-444898386588479070_i64),(-4642445668471806222_i64),2482952596770111110_i64];
_8.fld1 = _4;
_21.4 = _2;
_23 = core::ptr::addr_of_mut!(_15);
_1 = 9014882312342878089_usize as f64;
_7 = _8.fld1;
_18 = core::ptr::addr_of!(_6.fld0);
_21.1 = (-88864463737148293917459511893279703305_i128) + (-40164090640360092943543240580175401274_i128);
_6 = Adt45 { fld0: _21.2,fld1: _16.fld1,fld2: _16.fld2 };
_21.3 = &_1;
_21.3 = &_1;
_21.3 = &_1;
_21.3 = &_1;
_23 = core::ptr::addr_of_mut!((*_23));
_24.fld0 = _16.fld2;
_1 = 325164012482321907183121518007089276717_u128 as f64;
_1 = (*_23) as f64;
_7 = _8.fld1;
_21.0 = 2447139570559944553_u64 & 18323992792241850805_u64;
_29 = _8.fld0;
_21.2 = _1 as i8;
_2 = _21.4 ^ _21.4;
Goto(bb6)
}
bb6 = {
_19 = [_6.fld2,_24.fld0,_3,_6.fld2,_3,_24.fld0,_6.fld2];
_30 = -_8.fld0;
_22 = _2 | _2;
_26.2 = 8510137830695050786_i64 | (-1816195634124065089_i64);
_24.fld1 = [_21.4,_22,_22,_21.4,_22,_22];
_2 = !_22;
_20 = [1404646678_u32,101339285_u32,3265104953_u32];
_28 = [_21.0,_21.0];
_8 = Adt47 { fld0: RET,fld1: _7 };
_31.fld1 = [_2,_22,_2,_22,_2,_22];
_3 = _16.fld2;
_24.fld1 = [_2,_21.4,_2,_22,_2,_22];
_26 = (_21.4, _29, 4034240952277207629_i64, _24.fld1);
_6.fld2 = _26.2 as i16;
_28 = [_21.0,_21.0];
_27 = [_7,_7,_4,_4,_4,_7];
_20 = [3853698411_u32,2629541875_u32,1986409544_u32];
_31 = Move(_24);
Goto(bb7)
}
bb7 = {
_30 = _29;
RET = _8.fld0;
_24 = Adt50 { fld0: _6.fld2,fld1: _31.fld1 };
_17 = _9;
_7 = _4;
_26 = (_2, _8.fld0, (-1302539764098767520_i64), _24.fld1);
_3 = 29_isize as i16;
(*_18) = !_21.2;
(*_23) = 61391_u16 >> _6.fld2;
_15 = 54142_u16 - 714_u16;
_5 = [_21.0,_21.0,_21.0,_21.0,_21.0,_21.0,_21.0];
_21.1 = 14292117570478999615800561620559348898_i128;
_19 = [_24.fld0,_6.fld2,_6.fld2,_24.fld0,_24.fld0,_24.fld0,_6.fld2];
_26.1 = -_12;
RET = -_12;
_4 = _8.fld1;
_22 = !_26.0;
_33.0 = _24.fld0;
_16 = Adt45 { fld0: (*_18),fld1: _6.fld1,fld2: _24.fld0 };
_34 = Adt52::Variant0 { fld0: 907256178_i32 };
_25.fld1 = _24.fld1;
_8 = Adt47 { fld0: _30,fld1: _4 };
_25 = Move(_24);
_25 = Adt50 { fld0: _16.fld2,fld1: _26.3 };
Goto(bb8)
}
bb8 = {
_33.2 = &_1;
_18 = core::ptr::addr_of!((*_18));
_30 = RET * _12;
_3 = _25.fld0 * _16.fld2;
_7 = _4;
_25.fld1 = [_2,_26.0,_22,_2,_26.0,_2];
_21.4 = _16.fld2 < _3;
_21.0 = 17522481616457432577_u64 + 9868141838098663643_u64;
_24.fld0 = _1 as i16;
_6.fld0 = !_16.fld0;
_16.fld0 = _6.fld0;
_8 = Adt47 { fld0: _26.1,fld1: _4 };
_21 = (16850762023432700361_u64, 4685799953454860280528944198675932614_i128, _16.fld0, Move(_33.2), _2);
place!(Field::<i32>(Variant(_34, 0), 0)) = _2 as i32;
_21.4 = _2;
_13 = [2496015645_u32,2593507539_u32,3845748651_u32,2739369498_u32,461880006_u32,3791749729_u32,4003266314_u32,3066420306_u32];
_31.fld1 = [_22,_2,_21.4,_2,_26.0,_2];
match _21.1 {
0 => bb4,
1 => bb7,
2 => bb3,
3 => bb9,
4 => bb10,
4685799953454860280528944198675932614 => bb12,
_ => bb11
}
}
bb9 = {
_16 = _6;
_1 = 4237412274_u32 as f64;
_2 = false;
_2 = true | false;
_20 = [2788173967_u32,4239450479_u32,178804267_u32];
_4 = _7;
_13 = [992821143_u32,3723223124_u32,3747273942_u32,1089855285_u32,257469076_u32,1049825522_u32,2406145692_u32,2933937570_u32];
_16.fld1 = [9640047537563530021_u64,10450016603205723531_u64];
_4 = _7;
_15 = 1340175351_i32 as u16;
_16.fld2 = -_6.fld2;
_16.fld1 = [13822373907191454533_u64,5416299374509812685_u64];
_9 = [_4,_7,_4,_8.fld1,_8.fld1,_8.fld1];
_13 = [991279588_u32,2531890991_u32,812161910_u32,3874783731_u32,1006536928_u32,995234344_u32,2440568153_u32,3398846159_u32];
_21.4 = _2 | _2;
_6.fld1 = [13802591819684199952_u64,564924902232786327_u64];
_4 = _8.fld1;
_19 = [_16.fld2,_3,_3,_16.fld2,_16.fld2,_3,_6.fld2];
_19 = [_3,_16.fld2,_3,_3,_6.fld2,_3,_6.fld2];
_20 = [549037359_u32,3851665770_u32,2522411274_u32];
_8.fld0 = -_12;
_12 = -_8.fld0;
_21.2 = !_6.fld0;
Goto(bb5)
}
bb10 = {
_19 = [_6.fld2,_24.fld0,_3,_6.fld2,_3,_24.fld0,_6.fld2];
_30 = -_8.fld0;
_22 = _2 | _2;
_26.2 = 8510137830695050786_i64 | (-1816195634124065089_i64);
_24.fld1 = [_21.4,_22,_22,_21.4,_22,_22];
_2 = !_22;
_20 = [1404646678_u32,101339285_u32,3265104953_u32];
_28 = [_21.0,_21.0];
_8 = Adt47 { fld0: RET,fld1: _7 };
_31.fld1 = [_2,_22,_2,_22,_2,_22];
_3 = _16.fld2;
_24.fld1 = [_2,_21.4,_2,_22,_2,_22];
_26 = (_21.4, _29, 4034240952277207629_i64, _24.fld1);
_6.fld2 = _26.2 as i16;
_28 = [_21.0,_21.0];
_27 = [_7,_7,_4,_4,_4,_7];
_20 = [3853698411_u32,2629541875_u32,1986409544_u32];
_31 = Move(_24);
Goto(bb7)
}
bb11 = {
_9 = [_4,_4,_4,_4,_4,_4];
_8 = Adt47 { fld0: RET,fld1: _4 };
Goto(bb2)
}
bb12 = {
_25 = Move(_31);
place!(Field::<i32>(Variant(_34, 0), 0)) = (-1091110382_i32) + 406381332_i32;
_25 = Adt50 { fld0: _3,fld1: _26.3 };
(*_18) = _16.fld0 & _21.2;
_5 = [_21.0,_21.0,_21.0,_21.0,_21.0,_21.0,_21.0];
_36 = _21.0;
_31 = Adt50 { fld0: _33.0,fld1: _25.fld1 };
_21.1 = Field::<i32>(Variant(_34, 0), 0) as i128;
_26 = (_2, _8.fld0, 5409053050271130776_i64, _25.fld1);
_12 = 3_usize as f32;
(*_18) = _21.2 & _21.2;
_3 = _16.fld2 | _16.fld2;
_29 = -_12;
Goto(bb13)
}
bb13 = {
_16.fld1 = _28;
_10 = [_26.2,_26.2,_26.2,_26.2,_26.2,_26.2,_26.2,_26.2];
_6 = _16;
_33.1 = _31.fld1;
_33.2 = &_1;
_37 = 333930058438264083251628920580282750325_u128;
_26.1 = _29;
_20 = [3207612789_u32,1674034515_u32,1995794153_u32];
_21.4 = _26.2 == _26.2;
_32 = _19;
_17 = [_8.fld1,_4,_4,_7,_7,_8.fld1];
_24.fld1 = [_2,_21.4,_26.0,_22,_21.4,_21.4];
_16.fld0 = _21.2;
_26.3 = _24.fld1;
_38.1 = _24.fld1;
_21.0 = !_36;
_38 = (_24.fld0, _24.fld1, Move(_33.2));
match _36 {
0 => bb6,
1 => bb12,
2 => bb7,
3 => bb4,
4 => bb11,
5 => bb14,
6 => bb15,
16850762023432700361 => bb17,
_ => bb16
}
}
bb14 = {
_6.fld2 = _3;
_3 = !_6.fld2;
RET = -_8.fld0;
_8.fld0 = -RET;
_6.fld1 = [9456460496264938209_u64,14423478030379278756_u64];
_1 = 9384611151125114323_usize as f64;
RET = 41109499981486138979583635672917725405_u128 as f32;
_7 = _8.fld1;
_2 = true;
_6.fld1 = [11696209031055666956_u64,5211099387323453773_u64];
RET = _6.fld0 as f32;
_8 = Adt47 { fld0: RET,fld1: _4 };
_12 = RET;
_6.fld0 = -(-79_i8);
_10 = [(-7027022836672203575_i64),7697699833448437411_i64,(-3569622190885127982_i64),(-5922971247913663039_i64),3178956556117202530_i64,5541462915681663760_i64,707046674811870704_i64,1705527089752982160_i64];
_4 = _8.fld1;
Goto(bb3)
}
bb15 = {
_9 = [_4,_4,_4,_4,_4,_4];
_8 = Adt47 { fld0: RET,fld1: _4 };
Goto(bb2)
}
bb16 = {
_19 = [_6.fld2,_24.fld0,_3,_6.fld2,_3,_24.fld0,_6.fld2];
_30 = -_8.fld0;
_22 = _2 | _2;
_26.2 = 8510137830695050786_i64 | (-1816195634124065089_i64);
_24.fld1 = [_21.4,_22,_22,_21.4,_22,_22];
_2 = !_22;
_20 = [1404646678_u32,101339285_u32,3265104953_u32];
_28 = [_21.0,_21.0];
_8 = Adt47 { fld0: RET,fld1: _7 };
_31.fld1 = [_2,_22,_2,_22,_2,_22];
_3 = _16.fld2;
_24.fld1 = [_2,_21.4,_2,_22,_2,_22];
_26 = (_21.4, _29, 4034240952277207629_i64, _24.fld1);
_6.fld2 = _26.2 as i16;
_28 = [_21.0,_21.0];
_27 = [_7,_7,_4,_4,_4,_7];
_20 = [3853698411_u32,2629541875_u32,1986409544_u32];
_31 = Move(_24);
Goto(bb7)
}
bb17 = {
_9 = [_7,_8.fld1,_4,_4,_7,_7];
(*_18) = -_21.2;
_26 = (_21.4, _29, 1316197677558157258_i64, _31.fld1);
_41 = (_1,);
_13 = [822117931_u32,1703664056_u32,3090744051_u32,1842827838_u32,2958378649_u32,3004937455_u32,3000613376_u32,107444398_u32];
place!(Field::<i32>(Variant(_34, 0), 0)) = (-1379004145_i32) - (-487071908_i32);
_40.fld0 = -(*_18);
_24.fld0 = _4 as i16;
_38.0 = _25.fld0;
_32 = _19;
_40 = _16;
_21.3 = &_1;
_26.1 = _29 + _30;
_40.fld1 = _28;
_21.1 = 145142312060986516119261263987423398502_i128 >> _21.0;
SetDiscriminant(_34, 2);
_40.fld0 = !(*_18);
RET = -_26.1;
_18 = core::ptr::addr_of!(_40.fld0);
_28 = [_36,_21.0];
Goto(bb18)
}
bb18 = {
Call(_44 = dump_var(19_usize, 37_usize, Move(_37), 9_usize, Move(_9), 7_usize, Move(_7), 17_usize, Move(_17)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_44 = dump_var(19_usize, 20_usize, Move(_20), 15_usize, Move(_15), 2_usize, Move(_2), 28_usize, Move(_28)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_44 = dump_var(19_usize, 13_usize, Move(_13), 45_usize, _45, 45_usize, _45, 45_usize, _45), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{ac1ee}'), std::hint::black_box((-101_isize)), std::hint::black_box(43_i8), std::hint::black_box((-14792_i16)), std::hint::black_box(905073184_i32), std::hint::black_box(7945332765921472375_i64), std::hint::black_box((-96511017219935855968371149233372201005_i128)), std::hint::black_box(13961532842512659328_usize), std::hint::black_box(63_u8), std::hint::black_box(27220_u16), std::hint::black_box(14327336348486769807_u64));
                
            }
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: bool,
fld1: (f64,),
fld2: u64,
fld3: i8,
fld4: [u32; 8],
fld5: (bool, f32, i64, [bool; 6]),
fld6: u128,
fld7: (i8, i32, f64, i128),

},
Variant1{
fld0: i32,
fld1: *const u32,
fld2: [u16; 8],

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: i8,
fld1: [u64; 2],
fld2: i16,
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: bool,
fld1: [u64; 2],
fld2: [bool; 6],
fld3: usize,
fld4: i16,
fld5: f32,

},
Variant1{
fld0: u128,
fld1: *mut [u64; 7],
fld2: *const i8,
fld3: [i16; 5],

},
Variant2{
fld0: [u32; 3],
fld1: char,
fld2: f32,
fld3: f64,
fld4: Adt44,
fld5: [u64; 7],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt47{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt47 {
fld0: f32,
fld1: char,
}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: Adt45,
fld1: char,
fld2: u64,
fld3: i8,
fld4: (bool, f32, i64, [bool; 6]),
fld5: [u16; 8],
fld6: *mut [u64; 7],
}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: i32,

},
Variant1{
fld0: bool,
fld1: [u32; 8],
fld2: *mut char,
fld3: Adt47,
fld4: [u16; 8],

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: i16,
fld1: [bool; 6],
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: *mut char,
fld1: (i8, i32, f64, i128),
fld2: Adt47,
fld3: *const isize,
fld4: [u64; 7],
fld5: *mut u16,
fld6: [i64; 8],

},
Variant1{
fld0: i64,
fld1: Adt46,
fld2: *const i8,
fld3: f32,
fld4: [i16; 7],

},
Variant2{
fld0: u16,
fld1: char,
fld2: isize,
fld3: u8,
fld4: [i16; 5],
fld5: [u64; 7],
fld6: usize,
fld7: [u16; 8],

},
Variant3{
fld0: Adt44,
fld1: [i16; 5],
fld2: Adt48,
fld3: u32,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
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
fld0: i32,

},
Variant1{
fld0: Adt48,
fld1: char,
fld2: [u32; 8],
fld3: Adt44,
fld4: i64,
fld5: Adt49,

},
Variant2{
fld0: *const u32,
fld1: u32,
fld2: Adt44,
fld3: Adt51,

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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: [i16; 7],
fld1: u64,

},
Variant1{
fld0: usize,
fld1: [u64; 7],
fld2: f32,
fld3: *const isize,
fld4: [u32; 3],
fld5: *mut char,
fld6: *mut [u64; 7],
fld7: (bool, f32, i64, [bool; 6]),

},
Variant2{
fld0: [u32; 3],
fld1: [char; 6],
fld2: u64,
fld3: i8,

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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: *const i8,
fld1: *mut [u64; 7],
fld2: isize,
fld3: Adt47,
fld4: i16,
fld5: [i64; 8],
fld6: f32,
fld7: Adt51,

},
Variant1{
fld0: bool,
fld1: [i64; 8],
fld2: Adt47,

},
Variant2{
fld0: Adt51,
fld1: char,

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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: [u64; 2],
fld1: char,
fld2: Adt54,
fld3: [u16; 8],
fld4: u16,
fld5: u128,
fld6: (i8, i32, f64, i128),

},
Variant1{
fld0: Adt49,
fld1: f64,
fld2: Adt48,
fld3: *const isize,
fld4: i16,
fld5: Adt45,

},
Variant2{
fld0: i8,
fld1: Adt53,

},
Variant3{
fld0: bool,
fld1: u32,
fld2: *mut u16,
fld3: Adt47,
fld4: Adt48,
fld5: [u16; 8],
fld6: *mut [u64; 7],
fld7: [u64; 7],

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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: Adt44,
fld1: [u32; 8],
fld2: isize,
fld3: u8,
fld4: Adt54,

},
Variant1{
fld0: Adt47,
fld1: char,
fld2: i128,
fld3: f64,
fld4: i16,

},
Variant2{
fld0: Adt45,
fld1: (f64,),
fld2: [u32; 3],
fld3: [char; 6],
fld4: *mut [u64; 7],
fld5: Adt49,
fld6: [u16; 8],
fld7: Adt52,

},
Variant3{
fld0: bool,
fld1: u32,
fld2: [char; 6],
fld3: Adt47,
fld4: Adt45,
fld5: f32,
fld6: [i16; 7],
fld7: [bool; 6],

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt57{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt57 {
fld0: *const isize,
fld1: (bool, f32, i64, [bool; 6]),
fld2: isize,
fld3: Adt54,
fld4: Adt45,
fld5: *mut u16,
fld6: i64,
fld7: *const u32,
}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt58::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: bool,
fld1: *mut char,
fld2: (f64,),
fld3: Adt56,
fld4: i16,
fld5: u32,

},
Variant1{
fld0: f32,
fld1: [i16; 5],
fld2: usize,
fld3: [u32; 3],
fld4: Adt49,

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt59{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt59 {
fld0: Adt45,
fld1: u32,
fld2: Adt50,
fld3: Adt55,
fld4: (bool, f32, i64, [bool; 6]),
fld5: Adt56,
fld6: Adt52,
fld7: i128,
}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt60::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: Adt45,
fld1: *const isize,
fld2: u128,
fld3: [i16; 7],

},
Variant1{
fld0: u128,
fld1: i128,
fld2: Adt52,
fld3: Adt54,
fld4: [bool; 6],

},
Variant2{
fld0: *mut char,
fld1: u16,
fld2: *const isize,
fld3: Adt47,
fld4: [u64; 8],
fld5: Adt54,
fld6: [u16; 8],

},
Variant3{
fld0: Adt48,
fld1: [u16; 8],
fld2: (bool, f32, i64, [bool; 6]),
fld3: i8,
fld4: Adt55,
fld5: [char; 6],

}}

