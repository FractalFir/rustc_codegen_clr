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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: usize,mut _4: i8,mut _5: i16,mut _6: u64,mut _7: u16,mut _8: i128) -> ([i32; 3], f32, (u16, i32, i16, f64), u128) {
mir! {
type RET = ([i32; 3], f32, (u16, i32, i16, f64), u128);
let _9: Adt40;
let _10: ([i32; 3], f32, (u16, i32, i16, f64), u128);
let _11: Adt50;
let _12: isize;
let _13: [u64; 3];
let _14: [i32; 3];
let _15: i16;
let _16: ([i32; 3], [i128; 8]);
let _17: *const isize;
let _18: (i64, u8, bool);
let _19: char;
let _20: u32;
let _21: f32;
let _22: f32;
let _23: Adt42;
let _24: Adt46;
let _25: usize;
let _26: [i128; 8];
let _27: isize;
let _28: f32;
let _29: isize;
let _30: char;
let _31: ();
let _32: ();
{
RET.0 = [(-375709057_i32),1248388971_i32,(-932694241_i32)];
_3 = 4_usize + 7_usize;
_1 = _3 != _3;
RET.2.0 = 63576_u16 & 59514_u16;
RET.1 = (-5598443449855529893_i64) as f32;
RET.3 = 333605239451244691218722556756803599033_u128 ^ 332673294857112932117164586192773957120_u128;
RET.2.2 = _1 as i16;
_8 = _3 as i128;
RET.2.1 = -(-1944409063_i32);
_7 = !RET.2.0;
RET.2.3 = (-9223372036854775808_isize) as f64;
_4 = (-14_i8) * 125_i8;
RET.2.2 = 29762_i16;
RET.3 = 10469681527950120802_u64 as u128;
_2 = '\u{75565}';
RET.1 = (-9223372036854775808_isize) as f32;
RET.2.3 = RET.2.0 as f64;
Goto(bb1)
}
bb1 = {
_4 = (-123_i8);
RET.2.1 = (-1621346556_i32) << _3;
RET.1 = 13664164991619473330_u64 as f32;
match _4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463463374607431768211333 => bb10,
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
_1 = !false;
_10.3 = RET.3;
RET.1 = RET.3 as f32;
RET.2.2 = 6208_i16 << RET.2.1;
_10.2.0 = RET.1 as u16;
RET.0 = [RET.2.1,RET.2.1,RET.2.1];
_11.fld1 = [6007210189048174945_u64,13836899715066700129_u64,130429611271743113_u64];
_12 = (-9223372036854775808_isize);
_10.2.1 = RET.2.1;
_6 = _4 as u64;
_10.2.2 = RET.3 as i16;
_10 = (RET.0, RET.1, RET.2, RET.3);
_7 = _10.2.0 ^ _10.2.0;
_3 = _8 as usize;
_5 = _10.2.2;
RET.2 = (_7, _10.2.1, _10.2.2, _10.2.3);
RET = (_10.0, _10.1, _10.2, _10.3);
_13 = _11.fld1;
_14 = RET.0;
_10.1 = _10.2.1 as f32;
_3 = 1_usize - 7273131663532409797_usize;
Call(RET.3 = fn1(RET.2, _10.2, _7, _8, _10.2.2, RET.1, _5, _1, _10.2, _3, _10.0, _13, _10.2.1, _10), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_3 = 166_u8 as usize;
RET = (_10.0, _10.1, _10.2, _10.3);
_12 = _10.2.3 as isize;
RET = (_14, _10.1, _10.2, _10.3);
RET.1 = -_10.1;
_17 = core::ptr::addr_of!(_12);
_17 = core::ptr::addr_of!(_12);
_15 = RET.2.2 ^ _10.2.2;
RET.2.2 = _5 * _15;
_10.2.2 = RET.1 as i16;
_10.1 = RET.1;
_16.0 = _10.0;
_20 = 389239745_u32;
_10.2.1 = !RET.2.1;
RET.3 = !_10.3;
_4 = _6 as i8;
RET.2.1 = _10.2.1;
_7 = !_10.2.0;
RET.2.2 = _20 as i16;
_13 = _11.fld1;
Goto(bb12)
}
bb12 = {
RET.0 = [RET.2.1,_10.2.1,_10.2.1];
_10.1 = RET.1;
_10.2.3 = RET.2.3 - RET.2.3;
_18.2 = _1 | _1;
_2 = '\u{bba2e}';
_25 = _3;
_21 = _10.2.0 as f32;
RET.3 = !_10.3;
(*_17) = (-9223372036854775808_isize) - (-20_isize);
RET.2.2 = !_10.2.2;
RET.3 = _10.3;
_22 = RET.1;
_10.2.1 = RET.2.1 * RET.2.1;
Call(_24.fld1 = core::intrinsics::transmute(_8), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
(*_17) = _10.2.1 as isize;
_16.1 = [_8,_8,_8,_8,_8,_8,_8,_8];
_7 = _10.2.0;
_12 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_18 = ((-2544539148569522766_i64), 249_u8, _1);
RET.2.1 = _10.2.1;
(*_17) = (-77_isize);
_7 = _10.2.0 << _6;
_24 = Adt46 { fld0: _25,fld1: RET.3 };
Goto(bb14)
}
bb14 = {
_24.fld1 = !RET.3;
_10.3 = _8 as u128;
_21 = _22;
_10.0 = [RET.2.1,_10.2.1,RET.2.1];
_26 = _16.1;
_16 = (_10.0, _26);
_27 = !(*_17);
RET.0 = [RET.2.1,_10.2.1,_10.2.1];
_26 = [_8,_8,_8,_8,_8,_8,_8,_8];
_18 = ((-1711327151093236917_i64), 6_u8, _1);
_18.1 = !133_u8;
_2 = '\u{4d6a2}';
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(0_usize, 18_usize, Move(_18), 8_usize, Move(_8), 5_usize, Move(_5), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(0_usize, 26_usize, Move(_26), 6_usize, Move(_6), 3_usize, Move(_3), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(0_usize, 15_usize, Move(_15), 32_usize, _32, 32_usize, _32, 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: (u16, i32, i16, f64),mut _2: (u16, i32, i16, f64),mut _3: u16,mut _4: i128,mut _5: i16,mut _6: f32,mut _7: i16,mut _8: bool,mut _9: (u16, i32, i16, f64),mut _10: usize,mut _11: [i32; 3],mut _12: [u64; 3],mut _13: i32,mut _14: ([i32; 3], f32, (u16, i32, i16, f64), u128)) -> u128 {
mir! {
type RET = u128;
let _15: [u64; 3];
let _16: [i32; 3];
let _17: i64;
let _18: usize;
let _19: Adt50;
let _20: isize;
let _21: u8;
let _22: i32;
let _23: [i128; 8];
let _24: f64;
let _25: [i32; 3];
let _26: *const [i32; 3];
let _27: i16;
let _28: f32;
let _29: f64;
let _30: bool;
let _31: (u16, i32, i16, f64);
let _32: isize;
let _33: isize;
let _34: u32;
let _35: *mut *const i16;
let _36: isize;
let _37: i16;
let _38: i128;
let _39: Adt47;
let _40: ();
let _41: ();
{
_2.3 = _1.3;
_9.3 = _1.1 as f64;
_1.1 = _2.1;
_10 = !0_usize;
RET = !_14.3;
_2.3 = -_9.3;
_16 = [_13,_1.1,_14.2.1];
_11 = _16;
_11 = _16;
_4 = 96898165939987800824653317517794379520_i128 ^ 115993550069734603934243761374710771860_i128;
_17 = 8000092867574883568_i64;
_16 = [_14.2.1,_9.1,_9.1];
_17 = 3404894910885722602_i64 + 88557337801780803_i64;
_19.fld0 = Adt44::Variant1 { fld0: _3,fld1: _14 };
_18 = !_10;
_1 = Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1).2;
_9.1 = 4797526703463283928_u64 as i32;
_14.2.3 = _9.3 + Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1).2.3;
RET = (-9223372036854775808_isize) as u128;
Goto(bb1)
}
bb1 = {
_1.3 = 46_i8 as f64;
_4 = _17 as i128;
_20 = (-80_isize);
_5 = -Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1).2.2;
place!(Field::<u16>(Variant(_19.fld0, 1), 0)) = _3 & _2.0;
_10 = _18 << Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1).2.0;
_14.0 = [Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1).2.1,_1.1,_13];
_9.3 = _14.2.3 - _14.2.3;
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1)).2 = (Field::<u16>(Variant(_19.fld0, 1), 0), _2.1, _14.2.2, _9.3);
_2.1 = _1.1 * Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1).2.1;
_9 = (_14.2.0, _13, _7, Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1).2.3);
_8 = !false;
_12 = [13166173078271919044_u64,14900042295202439427_u64,18019156815259317236_u64];
_9.2 = '\u{10aa45}' as i16;
_2.2 = _14.2.2 + _7;
_13 = _9.1;
_5 = _2.2;
_24 = Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1).1 as f64;
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1)) = (_14.0, _14.1, _9, _14.3);
_1.0 = 93_i8 as u16;
Goto(bb2)
}
bb2 = {
_25 = [_2.1,_2.1,Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1).2.1];
_14.2.1 = _1.1;
_25 = [_1.1,_9.1,_14.2.1];
_8 = !false;
Call(_22 = fn2(_9.1, Move(_19.fld0), _9.3, _14.2, _13, _1.1, _2, _13, _2.2, _20, _12, _9.3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_21 = 32_u8 & 121_u8;
_10 = _18;
_4 = !(-60154593107092908120040712861732803687_i128);
_13 = _21 as i32;
_15 = [15063291479194088538_u64,8041621600353005423_u64,12698445924395833076_u64];
_5 = _2.2 ^ _2.2;
_6 = _14.1;
match _20 {
0 => bb4,
1 => bb5,
2 => bb6,
340282366920938463463374607431768211376 => bb8,
_ => bb7
}
}
bb4 = {
_25 = [_2.1,_2.1,Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1).2.1];
_14.2.1 = _1.1;
_25 = [_1.1,_9.1,_14.2.1];
_8 = !false;
Call(_22 = fn2(_9.1, Move(_19.fld0), _9.3, _14.2, _13, _1.1, _2, _13, _2.2, _20, _12, _9.3), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_1.3 = 46_i8 as f64;
_4 = _17 as i128;
_20 = (-80_isize);
_5 = -Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1).2.2;
place!(Field::<u16>(Variant(_19.fld0, 1), 0)) = _3 & _2.0;
_10 = _18 << Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1).2.0;
_14.0 = [Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1).2.1,_1.1,_13];
_9.3 = _14.2.3 - _14.2.3;
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1)).2 = (Field::<u16>(Variant(_19.fld0, 1), 0), _2.1, _14.2.2, _9.3);
_2.1 = _1.1 * Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1).2.1;
_9 = (_14.2.0, _13, _7, Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1).2.3);
_8 = !false;
_12 = [13166173078271919044_u64,14900042295202439427_u64,18019156815259317236_u64];
_9.2 = '\u{10aa45}' as i16;
_2.2 = _14.2.2 + _7;
_13 = _9.1;
_5 = _2.2;
_24 = Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1).1 as f64;
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_19.fld0, 1), 1)) = (_14.0, _14.1, _9, _14.3);
_1.0 = 93_i8 as u16;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_14 = (_11, _6, _1, RET);
_23 = [_4,_4,_4,_4,_4,_4,_4,_4];
_9 = (_3, _22, _7, _24);
_27 = _5 << _22;
_18 = _10;
_9.2 = _1.2;
_28 = _4 as f32;
_28 = -_6;
_19.fld1 = _15;
_19.fld1 = [2626406397494846062_u64,6755157106824249587_u64,10034920738316363701_u64];
_14.2.3 = _24;
_25 = [_2.1,_1.1,_9.1];
_9.3 = _14.2.3 * _2.3;
_12 = _15;
_14.0 = [_1.1,_9.1,_1.1];
_19.fld1 = [15773572273854173392_u64,7943971159143274064_u64,4964071687281773928_u64];
_25 = _14.0;
_14.2.3 = -_2.3;
_29 = _1.3;
_15 = [8817856636258050955_u64,72165732317415665_u64,473747130848794499_u64];
_14.2.0 = _9.0 + _3;
_1.2 = _27 >> _2.1;
_14.2.1 = _22;
Goto(bb9)
}
bb9 = {
_4 = !(-47387948431615449395923075665275269357_i128);
_9.1 = _14.2.1;
_13 = RET as i32;
_14.2.3 = _2.3;
_24 = _17 as f64;
_2.3 = 3493122112_u32 as f64;
_20 = (-9223372036854775808_isize) - 9223372036854775807_isize;
_1.1 = _9.1;
Goto(bb10)
}
bb10 = {
_13 = _22 | _14.2.1;
_1.0 = _9.0;
_29 = _14.2.3;
_27 = RET as i16;
_16 = _11;
_32 = _28 as isize;
_33 = _32;
_31 = _9;
_30 = !_8;
_2.2 = _1.2 & _5;
Call(_1.0 = fn4(_1.1, _13, _1.1, _2, _31.1, _2.2, _2.2, _14.2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
RET = _28 as u128;
Call(_25 = core::intrinsics::transmute(_11), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_9 = (_31.0, _22, _1.2, _14.2.3);
_2.3 = -_31.3;
_2.0 = _1.0;
_2.3 = _13 as f64;
_26 = core::ptr::addr_of!(_16);
_7 = -_9.2;
_9.2 = _1.2 + _7;
_20 = (-119_i8) as isize;
_14.2 = (_2.0, _13, _2.2, _2.3);
_26 = core::ptr::addr_of!(_16);
_14.2.1 = _8 as i32;
_14.2.2 = !_2.2;
_14.2 = (_2.0, _2.1, _9.2, _2.3);
_1 = (_14.2.0, _31.1, _14.2.2, _14.2.3);
_14.2.1 = -_9.1;
Goto(bb13)
}
bb13 = {
_10 = RET as usize;
_14.2.3 = _1.3 - _1.3;
_31.2 = _14.2.2 ^ _9.2;
RET = 3551420429_u32 as u128;
_31.2 = !_7;
(*_26) = [_31.1,_1.1,_14.2.1];
_22 = _1.1;
_1 = _2;
_2.1 = _31.1;
_8 = _30;
_14 = (_16, _6, _2, RET);
_2.3 = _31.3;
Goto(bb14)
}
bb14 = {
_8 = _30;
_1.1 = _6 as i32;
_25 = _16;
_28 = _14.1;
_25 = [_31.1,_13,_2.1];
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(1_usize, 3_usize, Move(_3), 21_usize, Move(_21), 5_usize, Move(_5), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(1_usize, 15_usize, Move(_15), 23_usize, Move(_23), 20_usize, Move(_20), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(1_usize, 17_usize, Move(_17), 12_usize, Move(_12), 18_usize, Move(_18), 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: i32,mut _2: Adt44,mut _3: f64,mut _4: (u16, i32, i16, f64),mut _5: i32,mut _6: i32,mut _7: (u16, i32, i16, f64),mut _8: i32,mut _9: i16,mut _10: isize,mut _11: [u64; 3],mut _12: f64) -> i32 {
mir! {
type RET = i32;
let _13: bool;
let _14: (*const i16,);
let _15: [isize; 1];
let _16: bool;
let _17: [i128; 8];
let _18: f32;
let _19: ();
let _20: ();
{
RET = -Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 1), 1).2.1;
_4.1 = !Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 1), 1).2.1;
_5 = -_7.1;
_7.2 = _9;
RET = _1;
_12 = -Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 1), 1).2.3;
SetDiscriminant(_2, 0);
_4.0 = _10 as u16;
place!(Field::<(u16, i32, i16, f64)>(Variant(_2, 0), 0)).0 = 152_u8 as u16;
place!(Field::<u128>(Variant(_2, 0), 3)) = 195932055946599715593349770566178438717_u128;
_13 = !true;
_5 = !_8;
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 0), 4)).0 = [_4.1,_8,_7.1];
_1 = _7.1 + _5;
place!(Field::<(u16, i32, i16, f64)>(Variant(_2, 0), 0)).3 = _10 as f64;
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 0), 4)).2.1 = -_1;
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 0), 4)).1 = Field::<u128>(Variant(_2, 0), 3) as f32;
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 0), 4)).2.3 = _7.3 - _12;
_9 = -_7.2;
Call(_1 = fn3(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 0), 4).2.3, _9, Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 0), 4).2.3, Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 0), 4).2.3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7.3 = Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 0), 4).2.3 * _12;
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 0), 4)).1 = 3604597331_u32 as f32;
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 0), 4)).3 = Field::<u128>(Variant(_2, 0), 3);
_3 = _12;
_7.1 = !_4.1;
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 0), 4)).3 = !Field::<u128>(Variant(_2, 0), 3);
_7 = (_4.0, _1, _9, Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 0), 4).2.3);
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 0), 4)).2.2 = _13 as i16;
_15 = [_10];
_4.0 = 554732597_u32 as u16;
_7.3 = -Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 0), 4).2.3;
Call(place!(Field::<f64>(Variant(_2, 0), 6)) = core::intrinsics::fmaf64(_12, _3, Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 0), 4).2.3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 0), 4)).0 = [_7.1,RET,_7.1];
_12 = _4.3 * Field::<f64>(Variant(_2, 0), 6);
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 0), 4)).2.3 = -_12;
RET = _7.1 + _7.1;
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_2, 0), 4)).1 = 8941841223809285023_i64 as f32;
_15 = [_10];
place!(Field::<f64>(Variant(_2, 0), 6)) = Field::<u128>(Variant(_2, 0), 3) as f64;
place!(Field::<(u16, i32, i16, f64)>(Variant(_2, 0), 0)).1 = _1 << _7.2;
_14.0 = core::ptr::addr_of!(_9);
Goto(bb3)
}
bb3 = {
Call(_19 = dump_var(2_usize, 9_usize, Move(_9), 13_usize, Move(_13), 8_usize, Move(_8), 5_usize, Move(_5)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: f64,mut _2: i16,mut _3: f64,mut _4: f64) -> i32 {
mir! {
type RET = i32;
let _5: u128;
let _6: bool;
let _7: Adt46;
let _8: Adt40;
let _9: *mut *const i16;
let _10: i128;
let _11: [u128; 4];
let _12: char;
let _13: i128;
let _14: bool;
let _15: f64;
let _16: i16;
let _17: i32;
let _18: f32;
let _19: isize;
let _20: f64;
let _21: *const i16;
let _22: Adt44;
let _23: f64;
let _24: Adt46;
let _25: char;
let _26: (i64, u8, bool);
let _27: (i64, u8, bool);
let _28: bool;
let _29: (*const i16,);
let _30: [u8; 4];
let _31: (*const i8,);
let _32: ([i32; 3], [i128; 8]);
let _33: [u8; 2];
let _34: Adt45;
let _35: isize;
let _36: Adt48;
let _37: u8;
let _38: ([i32; 3], f32, (u16, i32, i16, f64), u128);
let _39: ();
let _40: ();
{
RET = -(-785652100_i32);
_2 = 26472_i16;
_3 = 16_u8 as f64;
_3 = 6155646401945015608_i64 as f64;
RET = -802608129_i32;
_5 = (-3852551442320038666_i64) as u128;
RET = -1823017501_i32;
_1 = -_4;
RET = 1750193556_i32;
_3 = _1;
_5 = RET as u128;
_3 = _2 as f64;
_2 = 924_i16;
_7 = Adt46 { fld0: 2_usize,fld1: _5 };
_6 = !false;
_3 = _1;
_7.fld1 = !_5;
_7.fld1 = _5;
_7.fld1 = _5 | _5;
_2 = 11966_i16;
_3 = -_1;
Call(_7.fld1 = core::intrinsics::transmute(_5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = false ^ false;
RET = (-1948940431_i32) >> _2;
RET = (-8318639647033339983_i64) as i32;
_10 = 56191902669839060730312567323344821364_i128;
_7 = Adt46 { fld0: 8724121792195185108_usize,fld1: _5 };
_3 = 25776_u16 as f64;
_2 = 5948_i16;
_7.fld0 = 0_usize - 12176993772894679129_usize;
_4 = -_1;
_7.fld1 = _5;
_2 = 4298880858395487901_i64 as i16;
Goto(bb2)
}
bb2 = {
_4 = _1 - _1;
_5 = _7.fld1;
_7.fld0 = _4 as usize;
_10 = -(-125777617618602196280394987236729527049_i128);
RET = (-927237815_i32) + 642899006_i32;
_1 = -_4;
_5 = _7.fld1 | _7.fld1;
_5 = _7.fld1 ^ _7.fld1;
RET = 1313675777_i32 & 1509688306_i32;
_5 = _7.fld1 >> _7.fld0;
RET = 990362559_i32 | 250457411_i32;
_5 = !_7.fld1;
_1 = _4 + _4;
_6 = false | false;
_13 = _10;
_3 = _7.fld0 as f64;
_5 = !_7.fld1;
RET = 1695207844_i32 * 191601433_i32;
_12 = '\u{27ed6}';
_6 = _1 == _3;
_11 = [_5,_5,_5,_5];
RET = 2113648614_i32 + 288187917_i32;
_7.fld0 = (-9223372036854775808_isize) as usize;
_14 = !_6;
_7.fld0 = !2104706978520840115_usize;
Goto(bb3)
}
bb3 = {
_1 = _4 * _3;
_1 = _3 - _4;
_2 = !(-4420_i16);
_5 = !_7.fld1;
_15 = _1 + _3;
_3 = 17_i8 as f64;
_1 = _4 + _15;
_7.fld0 = 9223372036854775807_isize as usize;
_5 = _7.fld1 ^ _7.fld1;
_7 = Adt46 { fld0: 1_usize,fld1: _5 };
_12 = '\u{34e9e}';
_16 = _2 - _2;
_18 = (-70_i8) as f32;
_18 = _7.fld0 as f32;
Goto(bb4)
}
bb4 = {
_7.fld1 = !_5;
_15 = -_1;
_19 = _12 as isize;
_13 = _1 as i128;
_14 = _6 ^ _6;
_7 = Adt46 { fld0: 18265853204707860028_usize,fld1: _5 };
_7 = Adt46 { fld0: 7_usize,fld1: _5 };
_17 = _19 as i32;
_17 = RET;
_7.fld0 = 7497827025806086593_usize + 5_usize;
_11 = [_5,_5,_5,_5];
_23 = -_4;
_13 = _10 >> _10;
_23 = -_4;
_18 = _16 as f32;
_7.fld0 = !4_usize;
_13 = _18 as i128;
_14 = _6;
Goto(bb5)
}
bb5 = {
_20 = -_15;
_11 = [_5,_7.fld1,_5,_7.fld1];
RET = !_17;
_2 = _16 - _16;
RET = _17 >> _2;
_20 = _13 as f64;
_21 = core::ptr::addr_of!(_16);
_25 = _12;
_24 = Adt46 { fld0: _7.fld0,fld1: _7.fld1 };
_12 = _25;
_27 = ((-2150148546895256156_i64), 163_u8, _14);
_1 = -_15;
_4 = -_1;
_21 = core::ptr::addr_of!(_2);
_27.2 = !_6;
_29.0 = _21;
_26.1 = !_27.1;
match _27.0 {
0 => bb1,
1 => bb4,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
340282366920938463461224458884872955300 => bb12,
_ => bb11
}
}
bb6 = {
_7.fld1 = !_5;
_15 = -_1;
_19 = _12 as isize;
_13 = _1 as i128;
_14 = _6 ^ _6;
_7 = Adt46 { fld0: 18265853204707860028_usize,fld1: _5 };
_7 = Adt46 { fld0: 7_usize,fld1: _5 };
_17 = _19 as i32;
_17 = RET;
_7.fld0 = 7497827025806086593_usize + 5_usize;
_11 = [_5,_5,_5,_5];
_23 = -_4;
_13 = _10 >> _10;
_23 = -_4;
_18 = _16 as f32;
_7.fld0 = !4_usize;
_13 = _18 as i128;
_14 = _6;
Goto(bb5)
}
bb7 = {
_1 = _4 * _3;
_1 = _3 - _4;
_2 = !(-4420_i16);
_5 = !_7.fld1;
_15 = _1 + _3;
_3 = 17_i8 as f64;
_1 = _4 + _15;
_7.fld0 = 9223372036854775807_isize as usize;
_5 = _7.fld1 ^ _7.fld1;
_7 = Adt46 { fld0: 1_usize,fld1: _5 };
_12 = '\u{34e9e}';
_16 = _2 - _2;
_18 = (-70_i8) as f32;
_18 = _7.fld0 as f32;
Goto(bb4)
}
bb8 = {
_4 = _1 - _1;
_5 = _7.fld1;
_7.fld0 = _4 as usize;
_10 = -(-125777617618602196280394987236729527049_i128);
RET = (-927237815_i32) + 642899006_i32;
_1 = -_4;
_5 = _7.fld1 | _7.fld1;
_5 = _7.fld1 ^ _7.fld1;
RET = 1313675777_i32 & 1509688306_i32;
_5 = _7.fld1 >> _7.fld0;
RET = 990362559_i32 | 250457411_i32;
_5 = !_7.fld1;
_1 = _4 + _4;
_6 = false | false;
_13 = _10;
_3 = _7.fld0 as f64;
_5 = !_7.fld1;
RET = 1695207844_i32 * 191601433_i32;
_12 = '\u{27ed6}';
_6 = _1 == _3;
_11 = [_5,_5,_5,_5];
RET = 2113648614_i32 + 288187917_i32;
_7.fld0 = (-9223372036854775808_isize) as usize;
_14 = !_6;
_7.fld0 = !2104706978520840115_usize;
Goto(bb3)
}
bb9 = {
_6 = false ^ false;
RET = (-1948940431_i32) >> _2;
RET = (-8318639647033339983_i64) as i32;
_10 = 56191902669839060730312567323344821364_i128;
_7 = Adt46 { fld0: 8724121792195185108_usize,fld1: _5 };
_3 = 25776_u16 as f64;
_2 = 5948_i16;
_7.fld0 = 0_usize - 12176993772894679129_usize;
_4 = -_1;
_7.fld1 = _5;
_2 = 4298880858395487901_i64 as i16;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_23 = _24.fld1 as f64;
_3 = _1;
_17 = _26.1 as i32;
_30 = [_27.1,_27.1,_27.1,_26.1];
_15 = -_3;
_13 = _10;
_18 = _2 as f32;
match _27.0 {
0 => bb10,
1 => bb5,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
340282366920938463461224458884872955300 => bb19,
_ => bb18
}
}
bb13 = {
_4 = _1 - _1;
_5 = _7.fld1;
_7.fld0 = _4 as usize;
_10 = -(-125777617618602196280394987236729527049_i128);
RET = (-927237815_i32) + 642899006_i32;
_1 = -_4;
_5 = _7.fld1 | _7.fld1;
_5 = _7.fld1 ^ _7.fld1;
RET = 1313675777_i32 & 1509688306_i32;
_5 = _7.fld1 >> _7.fld0;
RET = 990362559_i32 | 250457411_i32;
_5 = !_7.fld1;
_1 = _4 + _4;
_6 = false | false;
_13 = _10;
_3 = _7.fld0 as f64;
_5 = !_7.fld1;
RET = 1695207844_i32 * 191601433_i32;
_12 = '\u{27ed6}';
_6 = _1 == _3;
_11 = [_5,_5,_5,_5];
RET = 2113648614_i32 + 288187917_i32;
_7.fld0 = (-9223372036854775808_isize) as usize;
_14 = !_6;
_7.fld0 = !2104706978520840115_usize;
Goto(bb3)
}
bb14 = {
_6 = false ^ false;
RET = (-1948940431_i32) >> _2;
RET = (-8318639647033339983_i64) as i32;
_10 = 56191902669839060730312567323344821364_i128;
_7 = Adt46 { fld0: 8724121792195185108_usize,fld1: _5 };
_3 = 25776_u16 as f64;
_2 = 5948_i16;
_7.fld0 = 0_usize - 12176993772894679129_usize;
_4 = -_1;
_7.fld1 = _5;
_2 = 4298880858395487901_i64 as i16;
Goto(bb2)
}
bb15 = {
_6 = false ^ false;
RET = (-1948940431_i32) >> _2;
RET = (-8318639647033339983_i64) as i32;
_10 = 56191902669839060730312567323344821364_i128;
_7 = Adt46 { fld0: 8724121792195185108_usize,fld1: _5 };
_3 = 25776_u16 as f64;
_2 = 5948_i16;
_7.fld0 = 0_usize - 12176993772894679129_usize;
_4 = -_1;
_7.fld1 = _5;
_2 = 4298880858395487901_i64 as i16;
Goto(bb2)
}
bb16 = {
_4 = _1 - _1;
_5 = _7.fld1;
_7.fld0 = _4 as usize;
_10 = -(-125777617618602196280394987236729527049_i128);
RET = (-927237815_i32) + 642899006_i32;
_1 = -_4;
_5 = _7.fld1 | _7.fld1;
_5 = _7.fld1 ^ _7.fld1;
RET = 1313675777_i32 & 1509688306_i32;
_5 = _7.fld1 >> _7.fld0;
RET = 990362559_i32 | 250457411_i32;
_5 = !_7.fld1;
_1 = _4 + _4;
_6 = false | false;
_13 = _10;
_3 = _7.fld0 as f64;
_5 = !_7.fld1;
RET = 1695207844_i32 * 191601433_i32;
_12 = '\u{27ed6}';
_6 = _1 == _3;
_11 = [_5,_5,_5,_5];
RET = 2113648614_i32 + 288187917_i32;
_7.fld0 = (-9223372036854775808_isize) as usize;
_14 = !_6;
_7.fld0 = !2104706978520840115_usize;
Goto(bb3)
}
bb17 = {
_1 = _4 * _3;
_1 = _3 - _4;
_2 = !(-4420_i16);
_5 = !_7.fld1;
_15 = _1 + _3;
_3 = 17_i8 as f64;
_1 = _4 + _15;
_7.fld0 = 9223372036854775807_isize as usize;
_5 = _7.fld1 ^ _7.fld1;
_7 = Adt46 { fld0: 1_usize,fld1: _5 };
_12 = '\u{34e9e}';
_16 = _2 - _2;
_18 = (-70_i8) as f32;
_18 = _7.fld0 as f32;
Goto(bb4)
}
bb18 = {
_7.fld1 = !_5;
_15 = -_1;
_19 = _12 as isize;
_13 = _1 as i128;
_14 = _6 ^ _6;
_7 = Adt46 { fld0: 18265853204707860028_usize,fld1: _5 };
_7 = Adt46 { fld0: 7_usize,fld1: _5 };
_17 = _19 as i32;
_17 = RET;
_7.fld0 = 7497827025806086593_usize + 5_usize;
_11 = [_5,_5,_5,_5];
_23 = -_4;
_13 = _10 >> _10;
_23 = -_4;
_18 = _16 as f32;
_7.fld0 = !4_usize;
_13 = _18 as i128;
_14 = _6;
Goto(bb5)
}
bb19 = {
_23 = _4 + _4;
_9 = core::ptr::addr_of_mut!(_21);
_24 = _7;
_28 = _27.1 < _26.1;
_32.1 = [_13,_10,_13,_10,_13,_10,_10,_10];
(*_21) = _16 << _26.1;
_28 = !_27.2;
_35 = -_19;
_26.0 = _18 as i64;
_37 = _26.1;
(*_9) = core::ptr::addr_of!((*_21));
_27.0 = _26.0 * _26.0;
_15 = _3;
_21 = core::ptr::addr_of!((*_21));
_4 = 4583681784142544641_u64 as f64;
_29.0 = (*_9);
(*_21) = _1 as i16;
_25 = _12;
Goto(bb20)
}
bb20 = {
Call(_39 = dump_var(3_usize, 27_usize, Move(_27), 25_usize, Move(_25), 5_usize, Move(_5), 14_usize, Move(_14)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_39 = dump_var(3_usize, 30_usize, Move(_30), 12_usize, Move(_12), 13_usize, Move(_13), 16_usize, Move(_16)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: i32,mut _2: i32,mut _3: i32,mut _4: (u16, i32, i16, f64),mut _5: i32,mut _6: i16,mut _7: i16,mut _8: (u16, i32, i16, f64)) -> u16 {
mir! {
type RET = u16;
let _9: *const [i32; 3];
let _10: isize;
let _11: bool;
let _12: isize;
let _13: char;
let _14: ([i32; 3], f32, (u16, i32, i16, f64), u128);
let _15: [i32; 3];
let _16: Adt40;
let _17: [u64; 3];
let _18: char;
let _19: (u16, i32, i16, f64);
let _20: [i128; 8];
let _21: i128;
let _22: (u16, i32, i16, f64);
let _23: ();
let _24: ();
{
RET = (-5_isize) as u16;
_4 = _8;
_8.3 = 16149678819182201917_u64 as f64;
_8 = (_4.0, _1, _7, _4.3);
_1 = _4.1;
_6 = 84_i8 as i16;
_6 = _8.2 >> _7;
RET = !_4.0;
_4.2 = false as i16;
RET = _4.3 as u16;
_2 = _8.1;
_8.0 = !RET;
_4.1 = (-3093889229169717983_i64) as i32;
_8.0 = RET;
_1 = (-8_i8) as i32;
RET = 54_u8 as u16;
_4.1 = -_3;
_10 = 9223372036854775807_isize * (-9223372036854775808_isize);
_4.1 = 28631568260141368855152229082297396432_u128 as i32;
RET = _8.3 as u16;
_4.1 = -_8.1;
_7 = -_6;
_8.2 = _6;
_8.1 = _2;
_2 = _8.1;
_5 = _4.1 | _3;
_8.2 = _6 ^ _6;
RET = _4.0;
Call(_4 = fn5(_8, _6, _8.2, _8, _6, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = true ^ true;
_3 = -_2;
_11 = _7 <= _4.2;
Call(_8.0 = fn6(_2, _4.2, _11, _4.1, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8.0 = RET;
_8.3 = 206_u8 as f64;
_5 = -_3;
_8.3 = _4.3 + _4.3;
_4.3 = _8.3;
_8.2 = 8684787025647703283_i64 as i16;
_7 = !_6;
_8.3 = -_4.3;
Goto(bb3)
}
bb3 = {
RET = _8.0;
_12 = _11 as isize;
_8.1 = 1232830048_u32 as i32;
Goto(bb4)
}
bb4 = {
_4.1 = _2 & _2;
_8.0 = !RET;
_8 = _4;
_4.1 = _4.2 as i32;
_4.2 = -_8.2;
_13 = '\u{1907a}';
RET = _11 as u16;
RET = !_8.0;
_2 = _8.1 >> _4.2;
_4.1 = !_2;
Call(_4.3 = fn18(_2, _12), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_8.3 = _4.3 * _4.3;
_4.1 = _2 * _2;
_8.0 = _8.3 as u16;
_8 = _4;
_8.0 = !RET;
Goto(bb6)
}
bb6 = {
_8.0 = _4.1 as u16;
Goto(bb7)
}
bb7 = {
_14.2.1 = _2;
_14.0 = [_8.1,_14.2.1,_4.1];
_2 = 11966083977991681761_u64 as i32;
Goto(bb8)
}
bb8 = {
_4.3 = _8.3 - _8.3;
_11 = true | false;
_4.3 = 8451505382290004973_u64 as f64;
_7 = _4.2;
_14.3 = !90001637727820862697617490859079987191_u128;
_4.2 = -_7;
_8.0 = _4.0 >> _7;
_14.2.3 = _8.0 as f64;
_4.1 = _8.1 | _14.2.1;
_14.1 = 85_i8 as f32;
_8 = (RET, _3, _7, _14.2.3);
_8.1 = _14.2.1;
_17 = [6836143280152946275_u64,854182633794516100_u64,1791521920880793341_u64];
_14.3 = 85189130001195065630859408535626166196_u128;
_14.2.0 = _4.0;
_15 = [_8.1,_14.2.1,_4.1];
_15 = [_3,_14.2.1,_14.2.1];
_9 = core::ptr::addr_of!(_14.0);
_14.3 = 199378726143441131972837798944622743438_u128 & 49410313362492859406501731360180398103_u128;
_8.1 = !_4.1;
_14.2 = (_4.0, _4.1, _8.2, _8.3);
_9 = core::ptr::addr_of!(_15);
_11 = _3 <= _4.1;
_8 = (_14.2.0, _14.2.1, _7, _14.2.3);
Goto(bb9)
}
bb9 = {
Goto(bb10)
}
bb10 = {
_8.1 = !_4.1;
(*_9) = [_5,_14.2.1,_14.2.1];
(*_9) = _14.0;
_8.1 = _4.1 ^ _4.1;
Call(_16 = fn19(_6, _12, _8.2, (*_9), _14.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_8.0 = RET * RET;
_14.2.1 = _4.1;
_19 = (RET, _14.2.1, _7, _8.3);
_8.2 = -_6;
_4.0 = _14.2.0;
SetDiscriminant(_16, 1);
_22.2 = _8.0 as i16;
place!(Field::<(i64, u8, bool)>(Variant(_16, 1), 1)).0 = 1807932261406168569_i64 ^ 1621425426402542978_i64;
place!(Field::<(i64, u8, bool)>(Variant(_16, 1), 1)).2 = _22.2 < _4.2;
_8.0 = _4.0 | RET;
_22.0 = _19.0;
place!(Field::<(i64, u8, bool)>(Variant(_16, 1), 1)).0 = (-3882735919966067902_i64) << _7;
place!(Field::<(i64, u8, bool)>(Variant(_16, 1), 1)).2 = _11;
place!(Field::<[u64; 3]>(Variant(_16, 1), 3)) = [890092234716627107_u64,1109680682425656583_u64,9258669062832902111_u64];
_22.1 = -_3;
_8.3 = _19.3 + _14.2.3;
_4.2 = _22.2 ^ _19.2;
_7 = _6 << _12;
_14.2.0 = !_8.0;
_8 = _14.2;
place!(Field::<(i64, u8, bool)>(Variant(_16, 1), 1)) = (8374249622792509277_i64, 200_u8, _11);
match Field::<(i64, u8, bool)>(Variant(_16, 1), 1).1 {
0 => bb12,
1 => bb13,
200 => bb15,
_ => bb14
}
}
bb12 = {
RET = _8.0;
_12 = _11 as isize;
_8.1 = 1232830048_u32 as i32;
Goto(bb4)
}
bb13 = {
Goto(bb10)
}
bb14 = {
_8.0 = RET;
_8.3 = 206_u8 as f64;
_5 = -_3;
_8.3 = _4.3 + _4.3;
_4.3 = _8.3;
_8.2 = 8684787025647703283_i64 as i16;
_7 = !_6;
_8.3 = -_4.3;
Goto(bb3)
}
bb15 = {
_22.3 = _8.3;
RET = _19.0;
place!(Field::<[u64; 3]>(Variant(_16, 1), 3)) = _17;
_19 = (_14.2.0, _14.2.1, _4.2, _14.2.3);
_19.3 = _8.3;
_20 = [(-111943623795174960552962417879282423111_i128),127874517992697474646297726940949015606_i128,(-12415111938099481798132091626269885216_i128),155620774102978311798296401271701533183_i128,26140656298037683237173841853059146444_i128,(-167100348759214420056911789792819160732_i128),(-41471841020259670088889600651679030403_i128),(-12758648917005439581508148724779060524_i128)];
_8.2 = _4.2;
(*_9) = [_19.1,_19.1,_19.1];
_18 = _13;
_14.2.1 = _8.1;
_4.3 = _8.3;
_8.3 = _12 as f64;
_22.1 = 15810205294441974874_usize as i32;
_15 = [_14.2.1,_14.2.1,_3];
_19.3 = _8.3 - _8.3;
_19.1 = _8.1 | _4.1;
RET = _11 as u16;
Goto(bb16)
}
bb16 = {
Call(_23 = dump_var(4_usize, 7_usize, Move(_7), 20_usize, Move(_20), 11_usize, Move(_11), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_23 = dump_var(4_usize, 5_usize, Move(_5), 1_usize, Move(_1), 13_usize, Move(_13), 24_usize, _24), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: (u16, i32, i16, f64),mut _2: i16,mut _3: i16,mut _4: (u16, i32, i16, f64),mut _5: i16,mut _6: i16) -> (u16, i32, i16, f64) {
mir! {
type RET = (u16, i32, i16, f64);
let _7: char;
let _8: ();
let _9: ();
{
_1 = (_4.0, _4.1, _2, _4.3);
_4.2 = !_3;
_4 = _1;
_4.0 = _1.0 | _1.0;
RET.2 = _6;
RET = (_4.0, _4.1, _5, _4.3);
_4.0 = RET.0 | RET.0;
RET.1 = _4.1;
_4 = (_1.0, RET.1, _5, _1.3);
Goto(bb1)
}
bb1 = {
Call(_8 = dump_var(5_usize, 3_usize, Move(_3), 2_usize, Move(_2), 9_usize, _9, 9_usize, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: i32,mut _2: i16,mut _3: bool,mut _4: i32,mut _5: i16) -> u16 {
mir! {
type RET = u16;
let _6: [isize; 1];
let _7: isize;
let _8: f32;
let _9: Adt45;
let _10: u128;
let _11: Adt45;
let _12: i32;
let _13: u8;
let _14: *const i8;
let _15: ([i32; 3], [i128; 8]);
let _16: ([i32; 3], [i128; 8]);
let _17: isize;
let _18: Adt46;
let _19: bool;
let _20: (u16, i32, i16, f64);
let _21: i16;
let _22: Adt44;
let _23: ([i32; 3], f32, (u16, i32, i16, f64), u128);
let _24: Adt47;
let _25: isize;
let _26: i8;
let _27: [u64; 3];
let _28: [u64; 3];
let _29: isize;
let _30: ([i32; 3], f32, (u16, i32, i16, f64), u128);
let _31: [u8; 4];
let _32: [u128; 4];
let _33: [isize; 1];
let _34: usize;
let _35: i32;
let _36: isize;
let _37: (i64, u8, bool);
let _38: *const i8;
let _39: ();
let _40: ();
{
_3 = !true;
RET = 37166_u16;
RET = 56408_u16;
_1 = (-3476063936286332678_i64) as i32;
_5 = '\u{1dde7}' as i16;
RET = 2769_u16;
_5 = 157546727096254383873978218372155674689_i128 as i16;
_4 = _1;
_5 = !_2;
RET = !41761_u16;
_4 = _1;
_6 = [(-9223372036854775808_isize)];
_1 = _4 - _4;
_1 = (-5709121455317078620_i64) as i32;
RET = 39366_u16;
_3 = true;
_4 = _1;
match RET {
0 => bb1,
39366 => bb3,
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
_1 = _4 & _4;
_5 = _2;
_3 = _2 > _5;
_4 = _1;
_1 = (-47_i8) as i32;
_1 = 3585311084_u32 as i32;
_4 = _1 - _1;
_2 = '\u{b63e3}' as i16;
RET = !28066_u16;
RET = 59646_u16 - 12351_u16;
RET = !56304_u16;
RET = 49356_u16;
_6 = [98_isize];
_6 = [(-111_isize)];
_7 = !9223372036854775807_isize;
_7 = 38_u8 as isize;
RET = !36022_u16;
_4 = 1491531845_u32 as i32;
_4 = !_1;
_3 = false;
_9.fld0 = _1 as u16;
RET = _9.fld0 * _9.fld0;
_8 = (-6795765779297513919_i64) as f32;
_10 = '\u{ef772}' as u128;
RET = _9.fld0 - _9.fld0;
Goto(bb4)
}
bb4 = {
_2 = -_5;
_9.fld1 = [96130033964289597763874292405056208095_i128,35829061912669768841600863742041861995_i128,(-156714310088516561266867884120117694145_i128),129184566555009686947865119709112718900_i128,141597930094491109842810380942296917302_i128,136227558208436409728458548750690445489_i128,(-61610494465401047336383598768056447096_i128),83673393750063002241452640306768360908_i128];
_7 = _3 as isize;
Goto(bb5)
}
bb5 = {
_9.fld0 = !RET;
_9.fld0 = RET;
_10 = 157092519615699248874181557601951376705_u128;
_11.fld1 = [11898184832651204355441792091477611782_i128,17094275803140306772253475716383602676_i128,(-21743158658064971059462737787366005249_i128),31704344516075111525910946035683444198_i128,95758492508792501348080452188021988570_i128,(-43246437077285779005413961254940957713_i128),62801851969366457361876444459813044549_i128,(-57219666658809249831743133723671674017_i128)];
_6 = [_7];
_6 = [_7];
_11.fld0 = !RET;
RET = !_9.fld0;
_12 = _4;
_1 = _3 as i32;
_8 = 31_u8 as f32;
_9.fld1 = _11.fld1;
_12 = _4;
_9.fld1 = [78552952023703813216123247102018881058_i128,(-101294250392738959026167170210210588924_i128),47699357014216640411190795387988470167_i128,(-44624585253684149294020201443361832865_i128),(-127687082520216011448978186888306914491_i128),152608453324928009454843678914344003043_i128,67079766273335226008634183900068695108_i128,135184821670620559488570815161582470484_i128];
Goto(bb6)
}
bb6 = {
_15.0 = [_4,_4,_1];
_4 = -_1;
RET = _10 as u16;
_12 = 5295708597656144110_i64 as i32;
_9.fld0 = !_11.fld0;
_8 = (-4577033962704965116_i64) as f32;
_6 = [_7];
_2 = !_5;
_13 = 32_u8 & 41_u8;
_11.fld0 = !_9.fld0;
_13 = 130_u8;
_8 = _13 as f32;
_4 = 4107728391_u32 as i32;
_15.1 = [(-39520269028201054988309961326421703585_i128),(-891066015618579866746759321174568217_i128),(-26197007094028825516599246662799646679_i128),48778760711390342776394294883766755912_i128,82213351403739449905372536775559368157_i128,(-95491436001978967372771528903181194867_i128),19814701862990851931689601474443799531_i128,(-68237627627681096622450787819516560558_i128)];
Call(_9 = fn7(_5, _15), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_17 = !_7;
_5 = '\u{70116}' as i16;
_9.fld0 = 4362929554098507594_usize as u16;
_7 = _17;
_17 = _1 as isize;
_11 = Adt45 { fld0: RET,fld1: _9.fld1,fld2: _9.fld2 };
_9.fld0 = !RET;
SetDiscriminant(_11.fld2, 0);
_19 = _3;
_16 = (_15.0, _15.1);
_20.3 = _2 as f64;
_18.fld0 = !6991404133926923800_usize;
_16.1 = [(-53333240535967755904092390224131557220_i128),(-133653857438635785269002713806614199324_i128),(-116165798567576122057742116411575367126_i128),103940310367903102564749372535320640621_i128,22779721951748167529674661282355816127_i128,73384217614311242642817760046381667665_i128,(-83553766909216298140313653668588850441_i128),(-107591906359014583292298746858526225852_i128)];
_11.fld2 = _9.fld2;
_1 = _12;
place!(Field::<(*const i16,)>(Variant(_9.fld2, 0), 0)) = (Field::<(*const i16,)>(Variant(_11.fld2, 0), 0).0,);
_6 = [_7];
Goto(bb8)
}
bb8 = {
_1 = RET as i32;
place!(Field::<(*const i16,)>(Variant(_9.fld2, 0), 0)) = Field::<(*const i16,)>(Variant(_11.fld2, 0), 0);
_16 = _15;
match _13 {
0 => bb6,
130 => bb9,
_ => bb2
}
}
bb9 = {
_8 = _2 as f32;
_20.2 = _2;
_1 = _12 >> _2;
RET = _13 as u16;
_15.1 = [(-100992643529245333416528132662174072403_i128),70641560404182867353603444206030538185_i128,(-168243610885873620185434580578858018110_i128),(-83719438693682748738813597901416519592_i128),(-17754700449713538351034153358671827917_i128),(-29814628161824111416074913817283431238_i128),106670312206949129582840401307526204390_i128,(-102972084971420379115652536107567729474_i128)];
SetDiscriminant(_11.fld2, 0);
_11.fld1 = [(-125232251068385440394677425052129942075_i128),50330111370220833910236804888172953647_i128,86531147094870867507438319341174655442_i128,(-74845163304708036094292124215028110000_i128),(-75939852696108737862615758962667265480_i128),125280971852839065704433047897985332597_i128,88349168963473659475283067063346550119_i128,101321699013469826142490156691533728562_i128];
_20.2 = _2;
_11 = Move(_9);
SetDiscriminant(_11.fld2, 0);
_10 = !136903276209443840110785152904107020440_u128;
RET = _11.fld0;
_9.fld0 = _11.fld0;
_7 = _17 ^ _17;
_23.1 = _8;
_24 = Adt47::Variant1 { fld0: _20.3 };
_18 = Adt46 { fld0: 4_usize,fld1: _10 };
place!(Field::<(*const i16,)>(Variant(_11.fld2, 0), 0)).0 = core::ptr::addr_of!(_23.2.2);
_6 = [_7];
_23.2.0 = !_11.fld0;
Goto(bb10)
}
bb10 = {
_4 = _1;
_23.3 = !_10;
_11.fld1 = [20082526262173995380160438629743195575_i128,12998234586189823196474365105859181291_i128,64440633822708872368054078548420610852_i128,49250406077855253026134285133942382421_i128,164116761103233658073114030157159937112_i128,(-19054434450189384996310704080251130099_i128),(-48480880467174850137656124010094959321_i128),(-112054964249782498841904941770727757516_i128)];
SetDiscriminant(_11.fld2, 1);
place!(Field::<(i64, u8, bool)>(Variant(_11.fld2, 1), 1)).0 = (-8754402111697257754_i64);
_5 = _2;
_23.2.2 = !_2;
place!(Field::<([i32; 3], [i128; 8])>(Variant(_11.fld2, 1), 0)).0 = [_4,_4,_1];
place!(Field::<[u8; 2]>(Variant(_11.fld2, 1), 5)) = [_13,_13];
_20.0 = _11.fld0 + _23.2.0;
_20 = (RET, _1, _23.2.2, Field::<f64>(Variant(_24, 1), 0));
place!(Field::<[u8; 2]>(Variant(_11.fld2, 1), 5)) = [_13,_13];
_26 = -125_i8;
_23.3 = _23.1 as u128;
_20.2 = _5 & _23.2.2;
place!(Field::<([i32; 3], [i128; 8])>(Variant(_11.fld2, 1), 0)).0 = [_1,_4,_12];
_23.2.0 = _20.0 - _9.fld0;
_9.fld1 = _16.1;
place!(Field::<(i64, u8, bool)>(Variant(_11.fld2, 1), 1)).1 = _23.3 as u8;
SetDiscriminant(_24, 0);
match _18.fld0 {
4 => bb12,
_ => bb11
}
}
bb11 = {
_15.0 = [_4,_4,_1];
_4 = -_1;
RET = _10 as u16;
_12 = 5295708597656144110_i64 as i32;
_9.fld0 = !_11.fld0;
_8 = (-4577033962704965116_i64) as f32;
_6 = [_7];
_2 = !_5;
_13 = 32_u8 & 41_u8;
_11.fld0 = !_9.fld0;
_13 = 130_u8;
_8 = _13 as f32;
_4 = 4107728391_u32 as i32;
_15.1 = [(-39520269028201054988309961326421703585_i128),(-891066015618579866746759321174568217_i128),(-26197007094028825516599246662799646679_i128),48778760711390342776394294883766755912_i128,82213351403739449905372536775559368157_i128,(-95491436001978967372771528903181194867_i128),19814701862990851931689601474443799531_i128,(-68237627627681096622450787819516560558_i128)];
Call(_9 = fn7(_5, _15), ReturnTo(bb7), UnwindUnreachable())
}
bb12 = {
place!(Field::<[u8; 2]>(Variant(_11.fld2, 1), 5)) = [Field::<(i64, u8, bool)>(Variant(_11.fld2, 1), 1).1,Field::<(i64, u8, bool)>(Variant(_11.fld2, 1), 1).1];
_18.fld0 = 5929120476602963236_u64 as usize;
place!(Field::<*const isize>(Variant(_24, 0), 0)) = core::ptr::addr_of!(_25);
place!(Field::<(i64, u8, bool)>(Variant(_11.fld2, 1), 1)) = (7253779824185688174_i64, _13, _3);
_15 = (Field::<([i32; 3], [i128; 8])>(Variant(_11.fld2, 1), 0).0, _11.fld1);
_27 = [12587599523804596138_u64,15591229988454119933_u64,12141367555239888695_u64];
_11.fld0 = !_23.2.0;
_18 = Adt46 { fld0: 4_usize,fld1: _23.3 };
_1 = _20.1 >> _23.2.2;
_21 = Field::<(i64, u8, bool)>(Variant(_11.fld2, 1), 1).0 as i16;
place!(Field::<u32>(Variant(_11.fld2, 1), 2)) = _20.3 as u32;
_9.fld1 = _11.fld1;
place!(Field::<(i64, u8, bool)>(Variant(_11.fld2, 1), 1)).1 = _13;
_13 = Field::<(i64, u8, bool)>(Variant(_11.fld2, 1), 1).1 % Field::<(i64, u8, bool)>(Variant(_11.fld2, 1), 1).1;
_15.1 = [54835658663540989084376621822388821303_i128,(-6210574698833755760371293907099963993_i128),63677838828030533094208226873069666878_i128,25069762262262181185640254156680422044_i128,62145073372198376730753028307083255780_i128,158365928078316828451066200590208920290_i128,(-95087373224948483093603698199549618404_i128),(-107641219603743554288147694273925728750_i128)];
_31 = [_13,_13,_13,_13];
_5 = _7 as i16;
_30 = (Field::<([i32; 3], [i128; 8])>(Variant(_11.fld2, 1), 0).0, _23.1, _20, _18.fld1);
_30.1 = _26 as f32;
_18.fld0 = 2467174096797029411_usize;
_29 = _18.fld0 as isize;
_18.fld1 = !_23.3;
_20.1 = _4 | _4;
_20 = (_30.2.0, _4, _30.2.2, _30.2.3);
_23.2.1 = -_30.2.1;
Goto(bb13)
}
bb13 = {
_8 = _23.1;
_20.2 = _23.2.2 - _2;
_30.2 = (_23.2.0, _23.2.1, _2, _20.3);
_23.1 = _8;
_16.1 = _9.fld1;
_20.0 = _23.2.1 as u16;
place!(Field::<(i64, u8, bool)>(Variant(_11.fld2, 1), 1)) = (7396840301304918164_i64, _13, _3);
place!(Field::<u128>(Variant(_24, 0), 2)) = 1421203504496575566_u64 as u128;
_18 = Adt46 { fld0: 2397988857566390433_usize,fld1: _30.3 };
place!(Field::<([i32; 3], [i128; 8])>(Variant(_11.fld2, 1), 0)) = (_30.0, _15.1);
place!(Field::<char>(Variant(_24, 0), 1)) = '\u{107b73}';
place!(Field::<[u64; 3]>(Variant(_24, 0), 3)) = _27;
_16 = (_15.0, _9.fld1);
_34 = !_18.fld0;
place!(Field::<([i32; 3], [i128; 8])>(Variant(_11.fld2, 1), 0)).1 = _15.1;
place!(Field::<([i32; 3], [i128; 8])>(Variant(_11.fld2, 1), 0)).1 = _9.fld1;
place!(Field::<[u64; 3]>(Variant(_24, 0), 3)) = [655599823840533369_u64,13486312727549347400_u64,7163119450959319466_u64];
place!(Field::<[u8; 2]>(Variant(_11.fld2, 1), 5)) = [_13,Field::<(i64, u8, bool)>(Variant(_11.fld2, 1), 1).1];
_17 = Field::<(i64, u8, bool)>(Variant(_11.fld2, 1), 1).0 as isize;
_24 = Adt47::Variant1 { fld0: _20.3 };
_27 = [3795590453887657980_u64,15280895697066308831_u64,5587888137909398857_u64];
_23.2.2 = _18.fld1 as i16;
_30.2.0 = _20.0;
_6 = [_7];
Call(place!(Field::<[u64; 3]>(Variant(_11.fld2, 1), 3)) = core::intrinsics::transmute(_27), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
SetDiscriminant(_24, 1);
place!(Field::<u32>(Variant(_11.fld2, 1), 2)) = 1308461267_u32;
_14 = core::ptr::addr_of!(_26);
_37.1 = Field::<(i64, u8, bool)>(Variant(_11.fld2, 1), 1).1;
_21 = -_2;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(6_usize, 21_usize, Move(_21), 19_usize, Move(_19), 15_usize, Move(_15), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(6_usize, 26_usize, Move(_26), 2_usize, Move(_2), 17_usize, Move(_17), 31_usize, Move(_31)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(6_usize, 16_usize, Move(_16), 12_usize, Move(_12), 40_usize, _40, 40_usize, _40), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: i16,mut _2: ([i32; 3], [i128; 8])) -> Adt45 {
mir! {
type RET = Adt45;
let _3: *const i16;
let _4: [u128; 4];
let _5: [i128; 8];
let _6: f64;
let _7: Adt55;
let _8: [i128; 8];
let _9: Adt48;
let _10: [u64; 3];
let _11: i16;
let _12: [u128; 4];
let _13: u32;
let _14: isize;
let _15: (i64, u8, bool);
let _16: isize;
let _17: [i32; 3];
let _18: i16;
let _19: u64;
let _20: [i128; 8];
let _21: (*const i16,);
let _22: i8;
let _23: *const i16;
let _24: ([i32; 3], f32, (u16, i32, i16, f64), u128);
let _25: [u128; 4];
let _26: [i32; 3];
let _27: [isize; 1];
let _28: i16;
let _29: Adt54;
let _30: f32;
let _31: u128;
let _32: f64;
let _33: [u8; 4];
let _34: ([i32; 3], [i128; 8]);
let _35: *const i16;
let _36: isize;
let _37: i128;
let _38: u32;
let _39: (*const i8,);
let _40: f32;
let _41: u16;
let _42: isize;
let _43: [u128; 4];
let _44: bool;
let _45: *mut *const i16;
let _46: f64;
let _47: f32;
let _48: u16;
let _49: ([i32; 3], f32, (u16, i32, i16, f64), u128);
let _50: Adt54;
let _51: ();
let _52: ();
{
RET.fld0 = 46229_u16;
_2.1 = [69512679926268624672501641186375027833_i128,164657081175181727706411916525399109417_i128,(-146320034214430250707655466116200707290_i128),108241754134644202729288442765205344217_i128,(-115755277951323020901146899365620715689_i128),(-19068243891572871288392723428969237963_i128),(-41459812480422130667515884130940481219_i128),(-161556104302888984663269837042132647213_i128)];
RET.fld0 = !64341_u16;
_3 = core::ptr::addr_of!(_1);
Goto(bb1)
}
bb1 = {
_3 = core::ptr::addr_of!((*_3));
_5 = [(-45598398673199222604833867437161089907_i128),140625939064185295338596628148676822827_i128,69470264530006267868536433618065836818_i128,(-141651853691510487694098380991485268320_i128),137011125268470256302948868791315488713_i128,152075822196117507851701908621882473882_i128,(-92616698984328804839567813401853496635_i128),93269076696210286898157765633375635361_i128];
RET.fld0 = 14121014610338869663_usize as u16;
_5 = [(-164461844817391209683668729116791055013_i128),(-117346524167551439287975024952515439083_i128),34727905110156147412649553488468121472_i128,135921957515071557180695520326467960576_i128,(-78241930419597929086534675072780050586_i128),(-85695124954717046913079476049201609087_i128),156093557353371135507588441793404927734_i128,43741319854597712004704367831752499389_i128];
(*_3) = (-30794_i16);
RET.fld0 = !26823_u16;
_6 = (-6734004740640203080_i64) as f64;
(*_3) = 6359050784497738992_i64 as i16;
_4 = [58342789476709886922631174226038336817_u128,117070507134285349942401315606883920046_u128,32298255887702774796356922352797942255_u128,139838342780679267772500324243638904140_u128];
Call((*_3) = fn8(_3, RET.fld0, _2, _4, _2.1, _2.1, _2, _5, _5, _5, _2.1, _2.1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET.fld1 = [72816004259420815845936344792817724701_i128,(-110278206044584763954296706387068974896_i128),(-131142185800053308279685343811553583303_i128),(-56202886603803844788954885073371693883_i128),38278849383452261813415162328491272411_i128,(-145952792498104245943427067366681211285_i128),80619237743059726824843067936189533632_i128,(-7166728799819569789118619861609002995_i128)];
_2.0 = [46018076_i32,(-1155527162_i32),(-813587681_i32)];
_2.0 = [(-1870924554_i32),(-856573919_i32),(-1810886052_i32)];
_2.1 = [62408652230960422156667315391947989623_i128,104483204655844635936726311216987772569_i128,(-126331496740885034719108126138521853494_i128),7900843647292000923945874012692356924_i128,130985309593977786099718873085598626336_i128,(-100347348359185654259417648792613070356_i128),58810055217625809690520508091351036633_i128,21665904874022878100820115816608188039_i128];
_8 = [42748713617934650810384992101926769200_i128,(-72232621290508416470307106434670770955_i128),73459624228465519143218121312268594923_i128,51534266009997904844122289323887759737_i128,148700542885899305063834033089653831072_i128,(-65831251166997833777299967459973761389_i128),(-157017390835814590999629301606276998547_i128),157071724627177439620184926063605185244_i128];
RET.fld1 = [(-139325160338070100988944081615821712747_i128),153824714235898252543044300544814239487_i128,118266052806614097864048601501780650180_i128,146192140277585600069281752303626472539_i128,(-107603201157374022321493280391534725378_i128),112576795648635400389452195975581949140_i128,64300864445816774489927948736080117178_i128,(-65049321127552624441857028180733807878_i128)];
_4 = [235805858296307483120877601207257311157_u128,315037279736008946577596887582964491772_u128,223022539767330305890280969140131775068_u128,151300784459634286042879725578765767103_u128];
RET.fld1 = _2.1;
_8 = [98632501199436122438915469834651421743_i128,(-99121723053421816405876017299471411830_i128),159389089200449718214120398678337803289_i128,(-28472962106144095949699015686713278357_i128),130205450735833436780135885567410030988_i128,128930677685388671606980964364552688962_i128,(-165823056845962605441655162014726180450_i128),168095114918979962846835311478622488656_i128];
_11 = (-18_i8) as i16;
(*_3) = _11;
_3 = core::ptr::addr_of!(_1);
_4 = [25561491014199839438671923604504343197_u128,3944929306114541814076031474883651426_u128,165974839053380059541389201438544724379_u128,39281894201529899916225591516013488809_u128];
_12 = [72906018340296349603024088419341036606_u128,179006649645023371268759259138575822677_u128,38755720230165646865371829347597918176_u128,98681343290715271134117186363263922670_u128];
_12 = [240380893238652766337977316658682169439_u128,238068274897197352027018540796323722677_u128,181145169828952009979284703568394610507_u128,10347194651570782933651905797824776153_u128];
_2.0 = [(-1134627616_i32),1086210437_i32,496549963_i32];
_3 = core::ptr::addr_of!((*_3));
Call(_15 = fn16(_4, _12, _5, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = _12;
_17 = [(-1695231452_i32),(-494110082_i32),292233021_i32];
RET.fld1 = [(-150477570401697297128923657264990697623_i128),(-27659680734924991521669502785672826741_i128),7644090384578748617101881521453161496_i128,(-72870866815867793050758922434984049785_i128),(-169989457887133408328470262182013688269_i128),(-151439207206767463495475849782857849976_i128),147705069645735627417532772426338616349_i128,77146625939504104568040212425266470932_i128];
_15.2 = false;
_13 = !2164801694_u32;
_12 = [157490263847409865603379441088620997691_u128,40690553440118989394963989090565452559_u128,113738883348289723735242142964297212546_u128,182677119130316107410856179389146816109_u128];
_1 = _11;
_8 = [36317888905022303328076899728946377584_i128,55563413063802117589673529779000304014_i128,(-85927722994338319544181564972304250982_i128),(-137615685445847332863442563905967385118_i128),167971487859363456411777261534982448271_i128,37074054342805747650634453820533576397_i128,90668298949868859616586772190471411146_i128,(-96967207258914127611582376993384753469_i128)];
_15.0 = (-5963531934770251856_i64) >> _15.1;
_15 = (1545793942508795221_i64, 203_u8, true);
_2.1 = [(-132483264950480454116670148555147699016_i128),158119634640606699251692865599823519850_i128,76627763662119937796710881785957340715_i128,(-115096290041910528478969786723314844_i128),9910580527085609899471302304452135995_i128,(-122556515614828685161864758361275480424_i128),(-152897291681617936232278603554388729783_i128),(-84177983755834387438458285910172670222_i128)];
RET.fld0 = 47980_u16;
_19 = 37_i8 as u64;
(*_3) = _13 as i16;
_1 = _19 as i16;
_15 = ((-416502118024340747_i64), 31_u8, false);
_10 = [_19,_19,_19];
_15.1 = !43_u8;
_15.2 = true;
_14 = (-9223372036854775808_isize) | (-40_isize);
Goto(bb4)
}
bb4 = {
_5 = [(-152742708620898534710940977011435207199_i128),(-135422408002033011624394911539935634274_i128),(-51005681078738838183211427691916720541_i128),89574646665967698940386633722815402699_i128,(-33225024939191954431963949755579875548_i128),95672864562653528124901636031366202886_i128,(-136232928544503774467978066525623490143_i128),19407501672950383463222002988410431375_i128];
_15 = ((-4645354134154970001_i64), 196_u8, true);
_10 = [_19,_19,_19];
RET.fld0 = 31022_u16 - 19342_u16;
_17 = _2.0;
(*_3) = !_11;
_16 = !_14;
(*_3) = _11 - _11;
_19 = 16710440709837535245_u64 + 6366762866152688342_u64;
_15.2 = !true;
RET.fld0 = !27803_u16;
_1 = '\u{46f5e}' as i16;
_2.0 = _17;
_15 = (1711690716561091904_i64, 78_u8, true);
(*_3) = _6 as i16;
match _15.1 {
78 => bb6,
_ => bb5
}
}
bb5 = {
_4 = _12;
_17 = [(-1695231452_i32),(-494110082_i32),292233021_i32];
RET.fld1 = [(-150477570401697297128923657264990697623_i128),(-27659680734924991521669502785672826741_i128),7644090384578748617101881521453161496_i128,(-72870866815867793050758922434984049785_i128),(-169989457887133408328470262182013688269_i128),(-151439207206767463495475849782857849976_i128),147705069645735627417532772426338616349_i128,77146625939504104568040212425266470932_i128];
_15.2 = false;
_13 = !2164801694_u32;
_12 = [157490263847409865603379441088620997691_u128,40690553440118989394963989090565452559_u128,113738883348289723735242142964297212546_u128,182677119130316107410856179389146816109_u128];
_1 = _11;
_8 = [36317888905022303328076899728946377584_i128,55563413063802117589673529779000304014_i128,(-85927722994338319544181564972304250982_i128),(-137615685445847332863442563905967385118_i128),167971487859363456411777261534982448271_i128,37074054342805747650634453820533576397_i128,90668298949868859616586772190471411146_i128,(-96967207258914127611582376993384753469_i128)];
_15.0 = (-5963531934770251856_i64) >> _15.1;
_15 = (1545793942508795221_i64, 203_u8, true);
_2.1 = [(-132483264950480454116670148555147699016_i128),158119634640606699251692865599823519850_i128,76627763662119937796710881785957340715_i128,(-115096290041910528478969786723314844_i128),9910580527085609899471302304452135995_i128,(-122556515614828685161864758361275480424_i128),(-152897291681617936232278603554388729783_i128),(-84177983755834387438458285910172670222_i128)];
RET.fld0 = 47980_u16;
_19 = 37_i8 as u64;
(*_3) = _13 as i16;
_1 = _19 as i16;
_15 = ((-416502118024340747_i64), 31_u8, false);
_10 = [_19,_19,_19];
_15.1 = !43_u8;
_15.2 = true;
_14 = (-9223372036854775808_isize) | (-40_isize);
Goto(bb4)
}
bb6 = {
_10 = [_19,_19,_19];
match _15.1 {
0 => bb1,
1 => bb7,
2 => bb8,
78 => bb10,
_ => bb9
}
}
bb7 = {
_4 = _12;
_17 = [(-1695231452_i32),(-494110082_i32),292233021_i32];
RET.fld1 = [(-150477570401697297128923657264990697623_i128),(-27659680734924991521669502785672826741_i128),7644090384578748617101881521453161496_i128,(-72870866815867793050758922434984049785_i128),(-169989457887133408328470262182013688269_i128),(-151439207206767463495475849782857849976_i128),147705069645735627417532772426338616349_i128,77146625939504104568040212425266470932_i128];
_15.2 = false;
_13 = !2164801694_u32;
_12 = [157490263847409865603379441088620997691_u128,40690553440118989394963989090565452559_u128,113738883348289723735242142964297212546_u128,182677119130316107410856179389146816109_u128];
_1 = _11;
_8 = [36317888905022303328076899728946377584_i128,55563413063802117589673529779000304014_i128,(-85927722994338319544181564972304250982_i128),(-137615685445847332863442563905967385118_i128),167971487859363456411777261534982448271_i128,37074054342805747650634453820533576397_i128,90668298949868859616586772190471411146_i128,(-96967207258914127611582376993384753469_i128)];
_15.0 = (-5963531934770251856_i64) >> _15.1;
_15 = (1545793942508795221_i64, 203_u8, true);
_2.1 = [(-132483264950480454116670148555147699016_i128),158119634640606699251692865599823519850_i128,76627763662119937796710881785957340715_i128,(-115096290041910528478969786723314844_i128),9910580527085609899471302304452135995_i128,(-122556515614828685161864758361275480424_i128),(-152897291681617936232278603554388729783_i128),(-84177983755834387438458285910172670222_i128)];
RET.fld0 = 47980_u16;
_19 = 37_i8 as u64;
(*_3) = _13 as i16;
_1 = _19 as i16;
_15 = ((-416502118024340747_i64), 31_u8, false);
_10 = [_19,_19,_19];
_15.1 = !43_u8;
_15.2 = true;
_14 = (-9223372036854775808_isize) | (-40_isize);
Goto(bb4)
}
bb8 = {
_5 = [(-152742708620898534710940977011435207199_i128),(-135422408002033011624394911539935634274_i128),(-51005681078738838183211427691916720541_i128),89574646665967698940386633722815402699_i128,(-33225024939191954431963949755579875548_i128),95672864562653528124901636031366202886_i128,(-136232928544503774467978066525623490143_i128),19407501672950383463222002988410431375_i128];
_15 = ((-4645354134154970001_i64), 196_u8, true);
_10 = [_19,_19,_19];
RET.fld0 = 31022_u16 - 19342_u16;
_17 = _2.0;
(*_3) = !_11;
_16 = !_14;
(*_3) = _11 - _11;
_19 = 16710440709837535245_u64 + 6366762866152688342_u64;
_15.2 = !true;
RET.fld0 = !27803_u16;
_1 = '\u{46f5e}' as i16;
_2.0 = _17;
_15 = (1711690716561091904_i64, 78_u8, true);
(*_3) = _6 as i16;
match _15.1 {
78 => bb6,
_ => bb5
}
}
bb9 = {
_4 = _12;
_17 = [(-1695231452_i32),(-494110082_i32),292233021_i32];
RET.fld1 = [(-150477570401697297128923657264990697623_i128),(-27659680734924991521669502785672826741_i128),7644090384578748617101881521453161496_i128,(-72870866815867793050758922434984049785_i128),(-169989457887133408328470262182013688269_i128),(-151439207206767463495475849782857849976_i128),147705069645735627417532772426338616349_i128,77146625939504104568040212425266470932_i128];
_15.2 = false;
_13 = !2164801694_u32;
_12 = [157490263847409865603379441088620997691_u128,40690553440118989394963989090565452559_u128,113738883348289723735242142964297212546_u128,182677119130316107410856179389146816109_u128];
_1 = _11;
_8 = [36317888905022303328076899728946377584_i128,55563413063802117589673529779000304014_i128,(-85927722994338319544181564972304250982_i128),(-137615685445847332863442563905967385118_i128),167971487859363456411777261534982448271_i128,37074054342805747650634453820533576397_i128,90668298949868859616586772190471411146_i128,(-96967207258914127611582376993384753469_i128)];
_15.0 = (-5963531934770251856_i64) >> _15.1;
_15 = (1545793942508795221_i64, 203_u8, true);
_2.1 = [(-132483264950480454116670148555147699016_i128),158119634640606699251692865599823519850_i128,76627763662119937796710881785957340715_i128,(-115096290041910528478969786723314844_i128),9910580527085609899471302304452135995_i128,(-122556515614828685161864758361275480424_i128),(-152897291681617936232278603554388729783_i128),(-84177983755834387438458285910172670222_i128)];
RET.fld0 = 47980_u16;
_19 = 37_i8 as u64;
(*_3) = _13 as i16;
_1 = _19 as i16;
_15 = ((-416502118024340747_i64), 31_u8, false);
_10 = [_19,_19,_19];
_15.1 = !43_u8;
_15.2 = true;
_14 = (-9223372036854775808_isize) | (-40_isize);
Goto(bb4)
}
bb10 = {
_12 = [306422853132199432591652171222539437669_u128,138376209905113616141851284948426149884_u128,169955077978321587451513231066265259745_u128,216794663122340665710476121082965743711_u128];
_10 = [_19,_19,_19];
_19 = 15474981638895009192_u64;
RET.fld1 = [49585737521620558942148006112500509393_i128,(-97423614669425555375169036469486688787_i128),(-97327524480631939171756638354121281657_i128),(-113905710353617810408347630774942129467_i128),94743037549179442084934611007354083846_i128,38905234951053613679969449768692260268_i128,157317871322549503568033339991095930356_i128,(-101439682585843796365134720682471504558_i128)];
_4 = [245946264682645769454005165617181074227_u128,37232922688825194929296176077775750407_u128,254465392718084845101867685925019424573_u128,38290685022380498419401145638798316463_u128];
RET.fld0 = _13 as u16;
_5 = [(-99717366168386980679796792966005907516_i128),20806061436600842065801169960174731601_i128,122055756457497210289061852308952291112_i128,103163976055195727027013739069105241723_i128,(-134871539491656444409320404065945204910_i128),58288804273220188595327466296946411642_i128,37832027952925424866418655834155257735_i128,31752198305429648490583228073006212038_i128];
_15.2 = _16 < _16;
_15.0 = 154002131806533321346242658861929915702_i128 as i64;
_17 = _2.0;
_5 = [121548077140720331334268529750102829781_i128,(-95499155098198360322603162834112299361_i128),(-39973224700047906313481901212748000030_i128),(-52290495273414888176806908053284319487_i128),(-2022543176476546708473812535842819298_i128),(-155005622800988372092428851213464735857_i128),(-74509615474844673341965422546540391568_i128),166133846742827938573739981481883505241_i128];
RET.fld0 = !43125_u16;
_15.0 = '\u{9e28}' as i64;
match _15.1 {
0 => bb8,
1 => bb9,
78 => bb11,
_ => bb6
}
}
bb11 = {
_18 = '\u{108c11}' as i16;
_11 = (*_3);
_11 = (*_3);
_8 = [(-152349616212264412821111794568707977756_i128),83307565201817035178973871721187824635_i128,97024193357166229680515008832585163770_i128,56326504767472919171006736675974879239_i128,(-52676020924190244285746882521035022152_i128),(-81533877431528076390972498763192805524_i128),138814322998434202680386039073633120461_i128,46849689636537332081216935980875329352_i128];
_2.0 = _17;
_21 = (_3,);
match _19 {
0 => bb5,
15474981638895009192 => bb12,
_ => bb8
}
}
bb12 = {
_6 = (*_3) as f64;
_8 = _5;
_4 = [141706326495021108728785760010216554476_u128,203320990478271993986392078323536916418_u128,96126099089261673581727316994114031612_u128,301717412040664777551051070611668996338_u128];
_16 = '\u{c1d74}' as isize;
_11 = RET.fld0 as i16;
_8 = [(-100713644725161442109096132826923712054_i128),95681038454844194408577859802045623632_i128,101338333323425469901758061069862845983_i128,121692212707517553931575460962006739767_i128,(-162639506787941256484191751937457705501_i128),58269152737818719551442490248686886449_i128,136521455703339259040131676559545981655_i128,20448083074718564125424363436713137137_i128];
RET.fld0 = 6841_u16;
(*_3) = _18 ^ _11;
RET.fld1 = [(-92499465535920239646655586202936618046_i128),117730388621280932859934534962445937867_i128,35419476029453576471273665372607174200_i128,(-104418372681278689607735531163635139075_i128),(-164821785519979267372310657800671626282_i128),148549788748954390897908437639290375209_i128,84492274224226709041965508942959288650_i128,(-5412318264614506428921553578659731645_i128)];
(*_3) = -_11;
_4 = [177348258350494607999201590885383826583_u128,230303855910324773039067098638408847728_u128,137252549899878962896150040514453427046_u128,46964827002565809413432944964196750985_u128];
_13 = 1168720765_u32 * 1382191307_u32;
_10 = [_19,_19,_19];
_8 = [(-29936168372119959938084133284923496177_i128),31744649108800365939816980595644917738_i128,(-31516006224750673018019269072548611758_i128),40253296135896046124566471149875255661_i128,(-56480133562296967497213708509667100075_i128),(-67972001948790958982121684531180587299_i128),(-58077110032678155367714305675913706786_i128),(-87743447044478034270360738586242252994_i128)];
_8 = [49084696128842998442080575499803918171_i128,(-129413957974187378376768361948239652508_i128),(-134276822191520114724753049690321719162_i128),(-102111232506287902310663116690367562800_i128),48768708513729646720748567394492455552_i128,2833379204015743302974866586734748663_i128,154114291726733669123501444441492260613_i128,(-103321216690840132338089805704181288025_i128)];
_6 = 86_i8 as f64;
_24.2.0 = !RET.fld0;
_24.2 = (RET.fld0, (-1702187695_i32), (*_3), _6);
_16 = _14;
_2 = (_17, _8);
Goto(bb13)
}
bb13 = {
_18 = (*_3) << _13;
_24.2.2 = -_18;
RET.fld1 = [81096417514486436392406706533792805732_i128,(-9624832362654237942067817396876630678_i128),25316663708009934705831173601333185682_i128,14951408321954117278269084573552495414_i128,152639186034004177773445286498464387980_i128,(-39263694445860266588627633888510877249_i128),(-18272868460748698019138016528189634461_i128),(-133270462632125418981232739422117545566_i128)];
_26 = [_24.2.1,_24.2.1,_24.2.1];
(*_3) = _18;
_2.1 = [(-113184815042553164324775211317765722385_i128),(-3900389855135008302157631686313206870_i128),(-125988108685923569377241719512203078610_i128),(-128160462463804465600621250016424718916_i128),(-32463166724894539435963084418282238855_i128),72276444251118102090648881597691909847_i128,(-137586521481567511140412384081488905554_i128),(-48962371220377226315755400424663842260_i128)];
_23 = core::ptr::addr_of!(_11);
RET.fld0 = 5264586984907813570_usize as u16;
_21 = (_3,);
_23 = _21.0;
_24.0 = [_24.2.1,_24.2.1,_24.2.1];
_24.0 = [_24.2.1,_24.2.1,_24.2.1];
_25 = _4;
_16 = _15.1 as isize;
_24.2 = (RET.fld0, (-293658378_i32), (*_3), _6);
(*_3) = (-77_i8) as i16;
_20 = _5;
_12 = [68876430755593688698411407163573574736_u128,266871261772257461251900659293654194624_u128,281554698741291919105916588392737996024_u128,276962976844548687316793283363979095239_u128];
_24.2.2 = -(*_3);
_5 = _20;
_18 = '\u{6ba3a}' as i16;
_24.2.0 = RET.fld0;
_2.1 = [83488347453862420948560549978802685938_i128,51923522663106944384042402545134867985_i128,(-160945942198914179632101041921075568393_i128),103108681899864769939762449953646126919_i128,(-23214317357013538704930687949786528522_i128),145106416685440454737351169524029503531_i128,62841646798588051859763834381364134774_i128,160523743188159242052779309618489926734_i128];
_6 = _14 as f64;
_16 = !_14;
match _15.1 {
0 => bb14,
1 => bb15,
78 => bb17,
_ => bb16
}
}
bb14 = {
_4 = _12;
_17 = [(-1695231452_i32),(-494110082_i32),292233021_i32];
RET.fld1 = [(-150477570401697297128923657264990697623_i128),(-27659680734924991521669502785672826741_i128),7644090384578748617101881521453161496_i128,(-72870866815867793050758922434984049785_i128),(-169989457887133408328470262182013688269_i128),(-151439207206767463495475849782857849976_i128),147705069645735627417532772426338616349_i128,77146625939504104568040212425266470932_i128];
_15.2 = false;
_13 = !2164801694_u32;
_12 = [157490263847409865603379441088620997691_u128,40690553440118989394963989090565452559_u128,113738883348289723735242142964297212546_u128,182677119130316107410856179389146816109_u128];
_1 = _11;
_8 = [36317888905022303328076899728946377584_i128,55563413063802117589673529779000304014_i128,(-85927722994338319544181564972304250982_i128),(-137615685445847332863442563905967385118_i128),167971487859363456411777261534982448271_i128,37074054342805747650634453820533576397_i128,90668298949868859616586772190471411146_i128,(-96967207258914127611582376993384753469_i128)];
_15.0 = (-5963531934770251856_i64) >> _15.1;
_15 = (1545793942508795221_i64, 203_u8, true);
_2.1 = [(-132483264950480454116670148555147699016_i128),158119634640606699251692865599823519850_i128,76627763662119937796710881785957340715_i128,(-115096290041910528478969786723314844_i128),9910580527085609899471302304452135995_i128,(-122556515614828685161864758361275480424_i128),(-152897291681617936232278603554388729783_i128),(-84177983755834387438458285910172670222_i128)];
RET.fld0 = 47980_u16;
_19 = 37_i8 as u64;
(*_3) = _13 as i16;
_1 = _19 as i16;
_15 = ((-416502118024340747_i64), 31_u8, false);
_10 = [_19,_19,_19];
_15.1 = !43_u8;
_15.2 = true;
_14 = (-9223372036854775808_isize) | (-40_isize);
Goto(bb4)
}
bb15 = {
_4 = _12;
_17 = [(-1695231452_i32),(-494110082_i32),292233021_i32];
RET.fld1 = [(-150477570401697297128923657264990697623_i128),(-27659680734924991521669502785672826741_i128),7644090384578748617101881521453161496_i128,(-72870866815867793050758922434984049785_i128),(-169989457887133408328470262182013688269_i128),(-151439207206767463495475849782857849976_i128),147705069645735627417532772426338616349_i128,77146625939504104568040212425266470932_i128];
_15.2 = false;
_13 = !2164801694_u32;
_12 = [157490263847409865603379441088620997691_u128,40690553440118989394963989090565452559_u128,113738883348289723735242142964297212546_u128,182677119130316107410856179389146816109_u128];
_1 = _11;
_8 = [36317888905022303328076899728946377584_i128,55563413063802117589673529779000304014_i128,(-85927722994338319544181564972304250982_i128),(-137615685445847332863442563905967385118_i128),167971487859363456411777261534982448271_i128,37074054342805747650634453820533576397_i128,90668298949868859616586772190471411146_i128,(-96967207258914127611582376993384753469_i128)];
_15.0 = (-5963531934770251856_i64) >> _15.1;
_15 = (1545793942508795221_i64, 203_u8, true);
_2.1 = [(-132483264950480454116670148555147699016_i128),158119634640606699251692865599823519850_i128,76627763662119937796710881785957340715_i128,(-115096290041910528478969786723314844_i128),9910580527085609899471302304452135995_i128,(-122556515614828685161864758361275480424_i128),(-152897291681617936232278603554388729783_i128),(-84177983755834387438458285910172670222_i128)];
RET.fld0 = 47980_u16;
_19 = 37_i8 as u64;
(*_3) = _13 as i16;
_1 = _19 as i16;
_15 = ((-416502118024340747_i64), 31_u8, false);
_10 = [_19,_19,_19];
_15.1 = !43_u8;
_15.2 = true;
_14 = (-9223372036854775808_isize) | (-40_isize);
Goto(bb4)
}
bb16 = {
_3 = core::ptr::addr_of!((*_3));
_5 = [(-45598398673199222604833867437161089907_i128),140625939064185295338596628148676822827_i128,69470264530006267868536433618065836818_i128,(-141651853691510487694098380991485268320_i128),137011125268470256302948868791315488713_i128,152075822196117507851701908621882473882_i128,(-92616698984328804839567813401853496635_i128),93269076696210286898157765633375635361_i128];
RET.fld0 = 14121014610338869663_usize as u16;
_5 = [(-164461844817391209683668729116791055013_i128),(-117346524167551439287975024952515439083_i128),34727905110156147412649553488468121472_i128,135921957515071557180695520326467960576_i128,(-78241930419597929086534675072780050586_i128),(-85695124954717046913079476049201609087_i128),156093557353371135507588441793404927734_i128,43741319854597712004704367831752499389_i128];
(*_3) = (-30794_i16);
RET.fld0 = !26823_u16;
_6 = (-6734004740640203080_i64) as f64;
(*_3) = 6359050784497738992_i64 as i16;
_4 = [58342789476709886922631174226038336817_u128,117070507134285349942401315606883920046_u128,32298255887702774796356922352797942255_u128,139838342780679267772500324243638904140_u128];
Call((*_3) = fn8(_3, RET.fld0, _2, _4, _2.1, _2.1, _2, _5, _5, _5, _2.1, _2.1), ReturnTo(bb2), UnwindUnreachable())
}
bb17 = {
_11 = -(*_3);
_18 = -_24.2.2;
_21.0 = _3;
_10 = [_19,_19,_19];
_24.2 = (RET.fld0, 80699738_i32, (*_3), _6);
_24.0 = _26;
_24.3 = 39772003715924862851886715318953112365_u128 - 204781573016797899000707668296127081197_u128;
_21 = (_23,);
_21.0 = core::ptr::addr_of!(_11);
_31 = _24.3;
_7 = Adt55::Variant1 { fld0: _15 };
(*_23) = -_18;
_21.0 = _23;
_3 = core::ptr::addr_of!((*_3));
_15.1 = _24.2.3 as u8;
_1 = _18;
_34.1 = [51987253085603757179988520877158143737_i128,61226249525856009277905222634616433526_i128,6884427393552664638073102636817708770_i128,91551252618636036746467977587006153051_i128,(-89678636981873936915046755354961593327_i128),130203753144502937121295120443024056622_i128,25839999464704339603890350575495806461_i128,(-3852334904109488714101513332748147962_i128)];
_2.0 = [_24.2.1,_24.2.1,_24.2.1];
place!(Field::<(i64, u8, bool)>(Variant(_7, 1), 0)).2 = !_15.2;
(*_3) = !_24.2.2;
(*_3) = _24.2.2 >> RET.fld0;
(*_3) = _24.2.2;
_27 = [_14];
_18 = _11;
Call(_28 = core::intrinsics::transmute((*_3)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
place!(Field::<(i64, u8, bool)>(Variant(_7, 1), 0)).0 = _15.0 & _15.0;
_30 = RET.fld0 as f32;
_31 = _24.3 << _24.2.1;
RET.fld1 = _2.1;
_33 = [Field::<(i64, u8, bool)>(Variant(_7, 1), 0).1,Field::<(i64, u8, bool)>(Variant(_7, 1), 0).1,Field::<(i64, u8, bool)>(Variant(_7, 1), 0).1,Field::<(i64, u8, bool)>(Variant(_7, 1), 0).1];
_24.2.0 = _24.2.3 as u16;
_35 = core::ptr::addr_of!((*_23));
_24.0 = _2.0;
_12 = [_31,_31,_31,_31];
Goto(bb19)
}
bb19 = {
_32 = -_24.2.3;
_14 = !_16;
SetDiscriminant(_7, 0);
RET.fld1 = [(-99373659230585372836874861091319595566_i128),(-112764560182688702967870636684389789976_i128),142023979891073173080019557950530820839_i128,68620984925493548728784486118957415554_i128,(-73672894135546022102611656054704504042_i128),160951515192703076490938173416751568112_i128,145184102887262785178924944109098886024_i128,(-23474081133274599311403608689360870558_i128)];
RET.fld0 = _24.2.0 - _24.2.0;
match _24.2.1 {
0 => bb7,
1 => bb14,
2 => bb3,
3 => bb17,
4 => bb12,
5 => bb16,
80699738 => bb21,
_ => bb20
}
}
bb20 = {
_6 = (*_3) as f64;
_8 = _5;
_4 = [141706326495021108728785760010216554476_u128,203320990478271993986392078323536916418_u128,96126099089261673581727316994114031612_u128,301717412040664777551051070611668996338_u128];
_16 = '\u{c1d74}' as isize;
_11 = RET.fld0 as i16;
_8 = [(-100713644725161442109096132826923712054_i128),95681038454844194408577859802045623632_i128,101338333323425469901758061069862845983_i128,121692212707517553931575460962006739767_i128,(-162639506787941256484191751937457705501_i128),58269152737818719551442490248686886449_i128,136521455703339259040131676559545981655_i128,20448083074718564125424363436713137137_i128];
RET.fld0 = 6841_u16;
(*_3) = _18 ^ _11;
RET.fld1 = [(-92499465535920239646655586202936618046_i128),117730388621280932859934534962445937867_i128,35419476029453576471273665372607174200_i128,(-104418372681278689607735531163635139075_i128),(-164821785519979267372310657800671626282_i128),148549788748954390897908437639290375209_i128,84492274224226709041965508942959288650_i128,(-5412318264614506428921553578659731645_i128)];
(*_3) = -_11;
_4 = [177348258350494607999201590885383826583_u128,230303855910324773039067098638408847728_u128,137252549899878962896150040514453427046_u128,46964827002565809413432944964196750985_u128];
_13 = 1168720765_u32 * 1382191307_u32;
_10 = [_19,_19,_19];
_8 = [(-29936168372119959938084133284923496177_i128),31744649108800365939816980595644917738_i128,(-31516006224750673018019269072548611758_i128),40253296135896046124566471149875255661_i128,(-56480133562296967497213708509667100075_i128),(-67972001948790958982121684531180587299_i128),(-58077110032678155367714305675913706786_i128),(-87743447044478034270360738586242252994_i128)];
_8 = [49084696128842998442080575499803918171_i128,(-129413957974187378376768361948239652508_i128),(-134276822191520114724753049690321719162_i128),(-102111232506287902310663116690367562800_i128),48768708513729646720748567394492455552_i128,2833379204015743302974866586734748663_i128,154114291726733669123501444441492260613_i128,(-103321216690840132338089805704181288025_i128)];
_6 = 86_i8 as f64;
_24.2.0 = !RET.fld0;
_24.2 = (RET.fld0, (-1702187695_i32), (*_3), _6);
_16 = _14;
_2 = (_17, _8);
Goto(bb13)
}
bb21 = {
_22 = _15.2 as i8;
_4 = [_31,_24.3,_31,_31];
place!(Field::<u16>(Variant(_7, 0), 4)) = RET.fld0;
_34.0 = _24.0;
_35 = _21.0;
_31 = _24.3;
_33 = [_15.1,_15.1,_15.1,_15.1];
(*_23) = _19 as i16;
_18 = (*_3) * (*_3);
place!(Field::<[u8; 4]>(Variant(_7, 0), 3)) = _33;
place!(Field::<Adt46>(Variant(_7, 0), 1)) = Adt46 { fld0: 0_usize,fld1: _31 };
_24.2.2 = -(*_3);
_23 = _3;
(*_23) = !_18;
_13 = 450686716_u32 << _14;
_17 = _24.0;
_2 = _34;
_36 = -_16;
place!(Field::<Adt46>(Variant(_7, 0), 1)).fld1 = _24.3;
_15.2 = !false;
_28 = (*_23);
(*_35) = -_28;
match Field::<Adt46>(Variant(_7, 0), 1).fld0 {
1 => bb17,
2 => bb15,
3 => bb9,
4 => bb5,
0 => bb23,
_ => bb22
}
}
bb22 = {
_12 = [306422853132199432591652171222539437669_u128,138376209905113616141851284948426149884_u128,169955077978321587451513231066265259745_u128,216794663122340665710476121082965743711_u128];
_10 = [_19,_19,_19];
_19 = 15474981638895009192_u64;
RET.fld1 = [49585737521620558942148006112500509393_i128,(-97423614669425555375169036469486688787_i128),(-97327524480631939171756638354121281657_i128),(-113905710353617810408347630774942129467_i128),94743037549179442084934611007354083846_i128,38905234951053613679969449768692260268_i128,157317871322549503568033339991095930356_i128,(-101439682585843796365134720682471504558_i128)];
_4 = [245946264682645769454005165617181074227_u128,37232922688825194929296176077775750407_u128,254465392718084845101867685925019424573_u128,38290685022380498419401145638798316463_u128];
RET.fld0 = _13 as u16;
_5 = [(-99717366168386980679796792966005907516_i128),20806061436600842065801169960174731601_i128,122055756457497210289061852308952291112_i128,103163976055195727027013739069105241723_i128,(-134871539491656444409320404065945204910_i128),58288804273220188595327466296946411642_i128,37832027952925424866418655834155257735_i128,31752198305429648490583228073006212038_i128];
_15.2 = _16 < _16;
_15.0 = 154002131806533321346242658861929915702_i128 as i64;
_17 = _2.0;
_5 = [121548077140720331334268529750102829781_i128,(-95499155098198360322603162834112299361_i128),(-39973224700047906313481901212748000030_i128),(-52290495273414888176806908053284319487_i128),(-2022543176476546708473812535842819298_i128),(-155005622800988372092428851213464735857_i128),(-74509615474844673341965422546540391568_i128),166133846742827938573739981481883505241_i128];
RET.fld0 = !43125_u16;
_15.0 = '\u{9e28}' as i64;
match _15.1 {
0 => bb8,
1 => bb9,
78 => bb11,
_ => bb6
}
}
bb23 = {
_24.1 = _30 * _30;
_3 = _23;
match _24.2.1 {
0 => bb10,
80699738 => bb24,
_ => bb11
}
}
bb24 = {
_7 = Adt55::Variant1 { fld0: _15 };
_19 = _13 as u64;
_39.0 = core::ptr::addr_of!(_22);
(*_35) = _11;
_24.2.2 = _1 ^ _18;
RET.fld0 = _24.2.0 & _24.2.0;
_18 = _24.2.1 as i16;
_19 = !1242499585054589455_u64;
(*_3) = _18 + _11;
_24.2.2 = _15.1 as i16;
_41 = !RET.fld0;
_2.1 = _5;
_24.3 = _31;
SetDiscriminant(_7, 2);
place!(Field::<u8>(Variant(_7, 2), 2)) = _15.1;
_38 = !_13;
Goto(bb25)
}
bb25 = {
_4 = [_31,_24.3,_31,_31];
_40 = _30;
_25 = _12;
(*_23) = _24.2.2 << RET.fld0;
_42 = _14;
_44 = _15.2;
place!(Field::<u8>(Variant(_7, 2), 2)) = _15.1 & _15.1;
_29 = Adt54::Variant2 { fld0: _24,fld1: _21 };
RET.fld1 = [(-87487992660733418590541511914804681499_i128),33086967215574576496868424039960752353_i128,(-50661762270101364352276759682071265598_i128),(-47268957508109147559600640690631654383_i128),140333105218209864772469289708805978415_i128,(-59717057296130719923788949016268796311_i128),(-73968718997215422335301217471059730986_i128),(-44105399934041635181131583919990806025_i128)];
_15.2 = _44;
_24 = Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_29, 2), 0);
_29 = Adt54::Variant0 { fld0: _19,fld1: _2 };
_10 = [_19,_19,_19];
Call((*_3) = fn17(Field::<([i32; 3], [i128; 8])>(Variant(_29, 0), 1), Field::<([i32; 3], [i128; 8])>(Variant(_29, 0), 1).1, _25, _24.2.1, _14, RET.fld1, _21.0, Move(_29), _24.0, _23), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
_30 = _40 * _40;
_33 = [Field::<u8>(Variant(_7, 2), 2),Field::<u8>(Variant(_7, 2), 2),_15.1,Field::<u8>(Variant(_7, 2), 2)];
_24.2.0 = _41 << _38;
_34.1 = RET.fld1;
place!(Field::<Adt42>(Variant(_7, 2), 1)) = Adt42::Variant3 { fld0: _44,fld1: _24.2 };
_49 = (_26, _30, Field::<(u16, i32, i16, f64)>(Variant(Field::<Adt42>(Variant(_7, 2), 1), 3), 1), _31);
_15.0 = Field::<(u16, i32, i16, f64)>(Variant(Field::<Adt42>(Variant(_7, 2), 1), 3), 1).0 as i64;
_26 = [Field::<(u16, i32, i16, f64)>(Variant(Field::<Adt42>(Variant(_7, 2), 1), 3), 1).1,_49.2.1,_49.2.1];
_47 = _24.1;
place!(Field::<Adt42>(Variant(_7, 2), 1)) = Adt42::Variant3 { fld0: _15.2,fld1: _24.2 };
_37 = -26358271862182775673856472800345811674_i128;
_5 = [_37,_37,_37,_37,_37,_37,_37,_37];
(*_35) = _28 ^ _18;
Call(place!(Field::<(u16, i32, i16, f64)>(Variant(place!(Field::<Adt42>(Variant(_7, 2), 1)), 3), 1)).0 = core::intrinsics::transmute(Field::<(u16, i32, i16, f64)>(Variant(Field::<Adt42>(Variant(_7, 2), 1), 3), 1).2), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
_45 = core::ptr::addr_of_mut!(_23);
SetDiscriminant(Field::<Adt42>(Variant(_7, 2), 1), 3);
_36 = -_14;
_49.2.2 = _37 as i16;
_13 = Field::<u8>(Variant(_7, 2), 2) as u32;
_15.2 = _41 <= _49.2.0;
_43 = _4;
_20 = [_37,_37,_37,_37,_37,_37,_37,_37];
RET.fld2 = Adt40::Variant0 { fld0: _21 };
_24.1 = _49.1 - _30;
_39.0 = core::ptr::addr_of!(_22);
place!(Field::<(*const i16,)>(Variant(RET.fld2, 0), 0)) = ((*_45),);
_49.2.2 = _15.1 as i16;
place!(Field::<(u16, i32, i16, f64)>(Variant(place!(Field::<Adt42>(Variant(_7, 2), 1)), 3), 1)).3 = _32;
place!(Field::<(u16, i32, i16, f64)>(Variant(place!(Field::<Adt42>(Variant(_7, 2), 1)), 3), 1)).3 = -_32;
Goto(bb28)
}
bb28 = {
Call(_51 = dump_var(7_usize, 25_usize, Move(_25), 37_usize, Move(_37), 34_usize, Move(_34), 26_usize, Move(_26)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_51 = dump_var(7_usize, 31_usize, Move(_31), 43_usize, Move(_43), 16_usize, Move(_16), 15_usize, Move(_15)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Call(_51 = dump_var(7_usize, 4_usize, Move(_4), 12_usize, Move(_12), 33_usize, Move(_33), 38_usize, Move(_38)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Call(_51 = dump_var(7_usize, 22_usize, Move(_22), 42_usize, Move(_42), 11_usize, Move(_11), 52_usize, _52), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: *const i16,mut _2: u16,mut _3: ([i32; 3], [i128; 8]),mut _4: [u128; 4],mut _5: [i128; 8],mut _6: [i128; 8],mut _7: ([i32; 3], [i128; 8]),mut _8: [i128; 8],mut _9: [i128; 8],mut _10: [i128; 8],mut _11: [i128; 8],mut _12: [i128; 8]) -> i16 {
mir! {
type RET = i16;
let _13: [i128; 8];
let _14: isize;
let _15: isize;
let _16: i128;
let _17: char;
let _18: i16;
let _19: isize;
let _20: char;
let _21: isize;
let _22: [u64; 3];
let _23: [u128; 4];
let _24: [u8; 2];
let _25: [u64; 3];
let _26: [i128; 8];
let _27: i64;
let _28: bool;
let _29: i8;
let _30: i128;
let _31: ([i32; 3], [i128; 8]);
let _32: bool;
let _33: [i32; 3];
let _34: (u16, i32, i16, f64);
let _35: [u8; 4];
let _36: [u64; 3];
let _37: (u16, i32, i16, f64);
let _38: Adt53;
let _39: *const (i64, u8, bool);
let _40: f32;
let _41: [u64; 3];
let _42: [u8; 4];
let _43: (i64, u8, bool);
let _44: [isize; 1];
let _45: Adt44;
let _46: Adt46;
let _47: *const [i32; 3];
let _48: Adt50;
let _49: isize;
let _50: *const [i32; 3];
let _51: [u128; 4];
let _52: u32;
let _53: i128;
let _54: Adt54;
let _55: Adt50;
let _56: Adt55;
let _57: u8;
let _58: isize;
let _59: bool;
let _60: u128;
let _61: [u8; 2];
let _62: i32;
let _63: i128;
let _64: ();
let _65: ();
{
_10 = [(-56529737985032383118489258920662622251_i128),91670516515764498489805484874701067932_i128,155936478495767831321765882715236354432_i128,(-22955164835747111385590185030202518744_i128),93618461023894619838528999689112011785_i128,145887470132199742749947297454410196985_i128,57724200541389248236240283892262236501_i128,(-31377347010986567317119306961397587600_i128)];
_7 = _3;
_7.0 = [(-1332773988_i32),242828611_i32,(-1140614643_i32)];
_3 = _7;
_1 = core::ptr::addr_of!(RET);
_4 = [38659506606354482725999674530252708100_u128,80759275315496697383772426290223426121_u128,154724974221208693443765166337497052445_u128,53095426990291568418531848065908963432_u128];
(*_1) = (-25439_i16) & 127_i16;
_10 = [(-47207401896776078507055124143144734703_i128),61993900055510297129148836585588820153_i128,(-6216259360215109418613194167098211650_i128),(-24953388598409158451932609566250095914_i128),13193660339371479061896611665427842800_i128,80308800263251183365539778282642351558_i128,(-112530378309916535561380484193572918905_i128),(-125382704849578835529575626198023836493_i128)];
_11 = [81382005892664530373213631593721432803_i128,(-5928101015940838271881995118079059690_i128),(-32192649200904203150826556285107829489_i128),(-168752429444983921753085426698491622450_i128),70553698235064560233742309418069492100_i128,153797823892620074975678094689557257447_i128,(-126901825965218509768710862924955136621_i128),(-91465023600046665312686885381558955511_i128)];
_13 = [7159539417556094756932173103152818896_i128,(-24678485507606955068069741239339492047_i128),(-17621351967252561380646522049946794991_i128),(-49231780362148843457254714098339974904_i128),(-20553527914225588604564103617751843173_i128),168793079445303790461627945086203739958_i128,33320670111597388773354308531778608685_i128,25369031896577626891926041314627399014_i128];
RET = -22746_i16;
_12 = [72300082309039205709937739118860184935_i128,44023140637554646062405907541394380267_i128,141955631911854405081397024733277937589_i128,(-36116379805208291711274603103012664801_i128),135167564736093063916203698701280738689_i128,(-3695880839454370159772946320800555403_i128),(-26913402494302284548824240974453579251_i128),43877335927116504476076390564504631928_i128];
_7.1 = [(-75709998831265951636138613641157126371_i128),(-88059094541785895458288935855331827396_i128),156692002398447568152472214947034916091_i128,164791024668324819823056992419634808334_i128,131288345609884660461688972384387678806_i128,56163094288508317713347525714641217884_i128,(-155692666675630337907868906804639320972_i128),(-21636433517735512261472608976360168098_i128)];
_2 = 42887_u16;
_12 = [(-156568704096735783349481012053272325111_i128),(-120840642157767797189119761617360102210_i128),(-169970185945045138196811878796969953061_i128),1848904996427115400524143312640061103_i128,(-134228309099055326817894801347703751270_i128),(-96363558873585197279008185703523654119_i128),(-18201202033607815003837705802497847503_i128),(-130191369392911250923651018620369851903_i128)];
_3 = (_7.0, _12);
_10 = _12;
(*_1) = 56_isize as i16;
_16 = 129541947239231090304168716552982698812_i128 - (-110580401106912397505012478029003742835_i128);
_4 = [16799201693451478285332232815858457106_u128,3264758546175835993316849030396440579_u128,111826203646493481333848531065305948330_u128,4156611483074442220011994625484120875_u128];
_15 = 78_isize | 9223372036854775807_isize;
_4 = [20178717368418247511494551869190906389_u128,268403542797926056914340882163504337656_u128,85721265362077507607533207373213462772_u128,134020402019970018467928497646425009301_u128];
_12 = _13;
RET = 28169_i16 + (-29585_i16);
(*_1) = -15187_i16;
Call(_3.0 = core::intrinsics::transmute(_7.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = [_16,_16,_16,_16,_16,_16,_16,_16];
_7.1 = [_16,_16,_16,_16,_16,_16,_16,_16];
_5 = _10;
_9 = _8;
_5 = _11;
_2 = 54_u8 as u16;
RET = 14372_i16 & 11234_i16;
_6 = _3.1;
_9 = _13;
(*_1) = -(-17202_i16);
_14 = !_15;
_13 = [_16,_16,_16,_16,_16,_16,_16,_16];
RET = 16794_i16 - 13849_i16;
RET = (-13583_i16) >> _14;
_7.1 = [_16,_16,_16,_16,_16,_16,_16,_16];
(*_1) = 26466_i16 - (-12120_i16);
Call(_17 = fn9(_11, _11, _5, _8, _3.0, _9, _3, _12, _4, _15, _14), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = [_16,_16,_16,_16,_16,_16,_16,_16];
_9 = [_16,_16,_16,_16,_16,_16,_16,_16];
Call(_3.0 = core::intrinsics::transmute(_7.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3 = _7;
_4 = [119673299270882835243338956003805957769_u128,188415742718857771944877766425735771170_u128,138915867579090481824083770256245177701_u128,23009706063909351474515173505092904084_u128];
_3.1 = _10;
_18 = (*_1) + RET;
_17 = '\u{ffe7c}';
_11 = _3.1;
_17 = '\u{67f61}';
_12 = _9;
Goto(bb4)
}
bb4 = {
_11 = _3.1;
_15 = !_14;
_4 = [80821685918216474907695342011838525871_u128,326795322467755246093383367351298114928_u128,108918828012630867733921828334672554455_u128,82275551049601972414364901658066579240_u128];
_10 = _6;
_1 = core::ptr::addr_of!((*_1));
(*_1) = _2 as i16;
_11 = [_16,_16,_16,_16,_16,_16,_16,_16];
_3.1 = [_16,_16,_16,_16,_16,_16,_16,_16];
_10 = [_16,_16,_16,_16,_16,_16,_16,_16];
_7 = _3;
_7.1 = [_16,_16,_16,_16,_16,_16,_16,_16];
_19 = _14 + _14;
_2 = 13189_u16;
RET = _18;
_8 = [_16,_16,_16,_16,_16,_16,_16,_16];
_2 = !45250_u16;
(*_1) = _2 as i16;
_16 = 205937132607833393308010669301330776148_u128 as i128;
_22 = [476177087048736493_u64,16886598805251708849_u64,5216160748312360504_u64];
(*_1) = _18 - _18;
_11 = _10;
_11 = [_16,_16,_16,_16,_16,_16,_16,_16];
_17 = '\u{3bcc}';
Goto(bb5)
}
bb5 = {
_25 = _22;
_3 = (_7.0, _11);
_5 = [_16,_16,_16,_16,_16,_16,_16,_16];
_17 = '\u{3a976}';
_24 = [247_u8,156_u8];
_17 = '\u{a35d8}';
_13 = [_16,_16,_16,_16,_16,_16,_16,_16];
_6 = _7.1;
RET = (-52_i8) as i16;
Call(_10 = core::intrinsics::transmute(_7.1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
(*_1) = _18 << _19;
_23 = _4;
(*_1) = !_18;
_20 = _17;
_12 = [_16,_16,_16,_16,_16,_16,_16,_16];
_9 = _11;
(*_1) = !_18;
RET = _18 + _18;
(*_1) = _18;
_16 = 7141722941896492658_u64 as i128;
_26 = [_16,_16,_16,_16,_16,_16,_16,_16];
_14 = _20 as isize;
_20 = _17;
_3.1 = _12;
_9 = [_16,_16,_16,_16,_16,_16,_16,_16];
_3 = (_7.0, _12);
_8 = [_16,_16,_16,_16,_16,_16,_16,_16];
_29 = 202011480891030788331564854643979379639_u128 as i8;
_29 = _15 as i8;
_23 = [304467304346307203358350616582861833285_u128,208326598894150341289723646604518642345_u128,249273751609732462615883220244306802686_u128,184755250927267522763938334749252585147_u128];
_23 = [234100232169519745196607281382895869135_u128,90222879482653365552974046030803746946_u128,242546821532411516746732533437942430473_u128,59687928519611543050284515339332019419_u128];
_12 = _8;
_31 = (_3.0, _3.1);
_21 = _15 ^ _19;
_27 = -8040470741426244285_i64;
Goto(bb7)
}
bb7 = {
_17 = _20;
_29 = (-10_i8) << _15;
_34.2 = (*_1) | (*_1);
_9 = [_16,_16,_16,_16,_16,_16,_16,_16];
_23 = _4;
RET = _34.2;
_22 = [17457583610064463165_u64,3142053220497920562_u64,14685404830881197070_u64];
_18 = -(*_1);
_3 = _7;
_25 = [14109075817751329564_u64,3811665637339686536_u64,7076850222769792654_u64];
_1 = core::ptr::addr_of!((*_1));
_31.0 = _7.0;
_3 = (_31.0, _6);
(*_1) = -_34.2;
_34.0 = _2;
_3 = (_31.0, _11);
_32 = true;
_30 = _16;
_31.0 = [1283593306_i32,290482237_i32,(-1292994163_i32)];
_3.1 = _6;
Goto(bb8)
}
bb8 = {
_33 = [(-1590904397_i32),805779842_i32,(-964208280_i32)];
_37.2 = !(*_1);
_19 = _34.0 as isize;
Call(_31.0 = fn14(_34.2, _25, _19, _24, _18, RET, _21, _19, _22, _7, _7.1, _1, (*_1), _34.2, _37.2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_29 = 45_i8 - (-24_i8);
_13 = [_30,_30,_30,_30,_16,_16,_30,_30];
_35 = [80_u8,186_u8,137_u8,71_u8];
(*_1) = _32 as i16;
_27 = (-723690630315693229_i64);
_20 = _17;
_37.0 = !_2;
_37.0 = _34.0;
_16 = _29 as i128;
_37.3 = 3_usize as f64;
_31.1 = _10;
_25 = _22;
_28 = _34.2 <= _34.2;
_34 = (_37.0, 1506251084_i32, _37.2, _37.3);
_24 = [95_u8,36_u8];
_31.0 = [_34.1,_34.1,_34.1];
_33 = _31.0;
_34.2 = _37.2 | _37.2;
_2 = _37.0;
_3.1 = [_16,_16,_30,_30,_16,_30,_16,_16];
Call(_13 = fn15(_34.1, _31.0, _3, _21, _28, _34, _34, _24, _34.1, _34, _34.2, _3, _31.0, _10, _7.0, _19), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_43 = (_27, 88_u8, _28);
_39 = core::ptr::addr_of!(_43);
_7.1 = _10;
_36 = [11361535021308743882_u64,3509409755710009741_u64,16559116922099145852_u64];
_3.0 = [_34.1,_34.1,_34.1];
_43.1 = 217_u8;
_10 = [_16,_16,_16,_30,_16,_16,_16,_16];
(*_39).0 = -_27;
_6 = [_16,_16,_16,_30,_16,_30,_16,_16];
_40 = 207471151264154057_u64 as f32;
_37.1 = _28 as i32;
_34 = (_2, _37.1, _18, _37.3);
(*_39) = (_27, 145_u8, _32);
match (*_39).1 {
145 => bb11,
_ => bb9
}
}
bb11 = {
(*_1) = _37.2;
_23 = [260590247137214513435681472221285960088_u128,208660862240201957176405674266136833787_u128,45843465737427848960074186310727891875_u128,47243355240504725809685593229166636523_u128];
(*_39) = (_27, 238_u8, _28);
_44 = [_21];
_44 = [_21];
_37.3 = _16 as f64;
_17 = _20;
_9 = _3.1;
_4 = _23;
_23 = [89279575521882545199841190495558532016_u128,88396492637366661027884478781321795327_u128,113793417333232679191037314262186844115_u128,68442292098937671617974791352808118957_u128];
_33 = [_37.1,_37.1,_34.1];
_37.0 = 4168368710574963294_u64 as u16;
_31.0 = [_37.1,_34.1,_37.1];
_47 = core::ptr::addr_of!(_7.0);
_47 = core::ptr::addr_of!((*_47));
_10 = [_16,_30,_30,_30,_16,_16,_16,_30];
_41 = [4452213093024503123_u64,9405855934485245940_u64,12691494712973267677_u64];
_7.1 = [_30,_16,_16,_30,_30,_30,_16,_16];
_28 = _34.1 < _34.1;
_13 = _9;
_15 = _21;
_36 = [9980426676309725979_u64,9138848180164065720_u64,5376033859089957838_u64];
match (*_39).1 {
238 => bb12,
_ => bb7
}
}
bb12 = {
_52 = 16702215880190607107_usize as u32;
_29 = 70_i8 & 56_i8;
_7.1 = [_16,_16,_16,_16,_16,_16,_16,_30];
_18 = -RET;
_51 = [292558900751351826723876512071829513325_u128,56397955329655905307964952423585967022_u128,149682665840280337154379407638164820005_u128,318890833787414043957705931055373990093_u128];
_5 = [_16,_16,_30,_30,_16,_16,_30,_16];
(*_1) = !_34.2;
_42 = [(*_39).1,(*_39).1,(*_39).1,(*_39).1];
_28 = !(*_39).2;
_6 = _12;
_42 = [(*_39).1,(*_39).1,(*_39).1,_43.1];
(*_39).1 = !251_u8;
_22 = _41;
_26 = [_16,_16,_16,_30,_16,_30,_30,_16];
(*_39) = (_27, 232_u8, _28);
_23 = [185904810585035948628942573561752289829_u128,329903432803784711726435623512498988226_u128,176637605820414802243016097450050619586_u128,70943838549083017660073707461722017455_u128];
_50 = core::ptr::addr_of!((*_47));
_21 = _15;
_32 = !_28;
_15 = _21 | _19;
_49 = _21;
_53 = !_16;
(*_47) = [_34.1,_37.1,_34.1];
_42 = [(*_39).1,(*_39).1,(*_39).1,(*_39).1];
(*_50) = _3.0;
Goto(bb13)
}
bb13 = {
_39 = core::ptr::addr_of!(_43);
_12 = [_30,_16,_16,_16,_30,_53,_30,_16];
_31 = ((*_50), _13);
_46.fld0 = 5_usize ^ 5_usize;
_21 = _15;
(*_39) = (_27, 222_u8, _28);
(*_39) = (_27, 39_u8, _32);
_37.2 = 112718121617375893963429264115592652478_u128 as i16;
RET = -_34.2;
_17 = _20;
_58 = _15 >> _37.1;
_29 = !(-5_i8);
_37.2 = _40 as i16;
_44 = [_58];
Goto(bb14)
}
bb14 = {
_48.fld1 = [9536448633192999452_u64,14747799384218377813_u64,15256100870756761375_u64];
_6 = _12;
(*_47) = [_34.1,_34.1,_37.1];
_31 = ((*_50), _3.1);
_1 = core::ptr::addr_of!((*_1));
_46 = Adt46 { fld0: 0_usize,fld1: 307082857951054835068074330477368333548_u128 };
_55.fld1 = [2494124795218033082_u64,12347041616019938413_u64,17358051019910994623_u64];
_61 = [_43.1,(*_39).1];
Goto(bb15)
}
bb15 = {
Call(_64 = dump_var(8_usize, 42_usize, Move(_42), 30_usize, Move(_30), 7_usize, Move(_7), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_64 = dump_var(8_usize, 28_usize, Move(_28), 44_usize, Move(_44), 49_usize, Move(_49), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_64 = dump_var(8_usize, 14_usize, Move(_14), 31_usize, Move(_31), 3_usize, Move(_3), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_64 = dump_var(8_usize, 25_usize, Move(_25), 18_usize, Move(_18), 41_usize, Move(_41), 19_usize, Move(_19)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_64 = dump_var(8_usize, 9_usize, Move(_9), 12_usize, Move(_12), 4_usize, Move(_4), 51_usize, Move(_51)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_64 = dump_var(8_usize, 10_usize, Move(_10), 6_usize, Move(_6), 65_usize, _65, 65_usize, _65), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: [i128; 8],mut _2: [i128; 8],mut _3: [i128; 8],mut _4: [i128; 8],mut _5: [i32; 3],mut _6: [i128; 8],mut _7: ([i32; 3], [i128; 8]),mut _8: [i128; 8],mut _9: [u128; 4],mut _10: isize,mut _11: isize) -> char {
mir! {
type RET = char;
let _12: Adt46;
let _13: (i64, u8, bool);
let _14: bool;
let _15: isize;
let _16: Adt44;
let _17: Adt41;
let _18: Adt42;
let _19: bool;
let _20: Adt46;
let _21: ([i32; 3], f32, (u16, i32, i16, f64), u128);
let _22: char;
let _23: f64;
let _24: Adt50;
let _25: [u8; 4];
let _26: f32;
let _27: bool;
let _28: Adt51;
let _29: [u8; 4];
let _30: (u16, i32, i16, f64);
let _31: char;
let _32: *const isize;
let _33: char;
let _34: ();
let _35: ();
{
RET = '\u{59a5c}';
RET = '\u{4c390}';
RET = '\u{3378b}';
RET = '\u{20473}';
_4 = [97233266643532825687342085901281209582_i128,(-16561530154933366130971587908540458222_i128),(-47991013013411939186337537431794653609_i128),(-132059703562239392243188461051483964042_i128),(-18547801516800289098140452844905489098_i128),21830419767329778567153302447340163857_i128,8561936225682343300467092077945646083_i128,(-16128104900977885680344947691190486_i128)];
_8 = _1;
_1 = _6;
_1 = [(-26001946818573517167797184354506860265_i128),78859528155367209133472092764741130757_i128,(-80731548859852216461617888978186359157_i128),94097147704867415955401660257557362923_i128,(-114692535202991081745623559690245586453_i128),(-70041826400200827332726603094381996000_i128),145690713927814038984751598202133875013_i128,4990043271278749327361147067261283639_i128];
_7.0 = [1848365717_i32,1216132702_i32,(-17114916_i32)];
RET = '\u{c0e8a}';
_7 = (_5, _1);
_4 = [(-131681057431971751214646150101396042883_i128),41915686005077241270079088131555501000_i128,79898395867106991965193487424157849809_i128,47443080263839301354959905858283276948_i128,31096611291484469262387816220628489559_i128,(-148927623199695834079509994737398667862_i128),93949038432086398116629704962929794526_i128,135922012207528803782697136328916502417_i128];
_7.1 = [(-66097988078651315409095729972128863163_i128),55530530028780995840156717830977420112_i128,(-83698555726914227456723669607276882938_i128),135494209104540744730842241871889602139_i128,(-109778637041012509961591254151179768021_i128),(-71799593258965203506454641182342079586_i128),(-104567369497128796372228120260949670080_i128),93149156522684167640146789128686560017_i128];
RET = '\u{a9f72}';
_9 = [284724481072477873518277001774897714030_u128,241927743528086346009032716181093070342_u128,72171356366746309283046827305126872071_u128,330533292978295767593181423920244995149_u128];
_11 = _10;
_2 = [73187605846598836259284077072791193900_i128,91314268395438136103546811340207691307_i128,(-89175417259422042620846452097220483305_i128),150439682491428435767502548224778537168_i128,141341248595820189038103866558773072076_i128,22242991428510826123760638655698824842_i128,(-28729345766887177353584826025877078502_i128),43050513221336531898091406900440104471_i128];
_11 = RET as isize;
_5 = _7.0;
_2 = _7.1;
_12.fld1 = 277308540219617076970535748912674391472_u128;
_12 = Adt46 { fld0: 7_usize,fld1: 315459405741038368717222981270566950668_u128 };
_5 = _7.0;
_13.2 = true;
_2 = _4;
Call(_13 = fn10(_7.1, _7.1, _3, _11, _6, _1, _12.fld0, _12.fld1, _6, _10, _8, _8, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12.fld1 = 81540531344864299497285256610193055766_u128 ^ 176900017421098927518622776816008365449_u128;
_2 = [(-113033972067141587178000111257198240895_i128),1173719217091305084299126314183152325_i128,37752367187411565362984079455199248301_i128,(-29446557738983680815368273371339975808_i128),156848587446894359638747730883998420134_i128,152340992760619171910118246420072624099_i128,106620576024654113760776290772019035679_i128,2529905646677425671927397706173384324_i128];
_2 = _1;
_11 = _13.0 as isize;
_13.1 = (-71_i8) as u8;
_2 = _3;
Call(_7.0 = core::intrinsics::transmute(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9 = [_12.fld1,_12.fld1,_12.fld1,_12.fld1];
_13.2 = _12.fld1 < _12.fld1;
_4 = [4153546277953120180226682004387164325_i128,(-88121119983150146919696677736014757973_i128),(-43227202749863055411470691375385285371_i128),104450986933169195781934123580003972918_i128,76833431896244792818458790097777065702_i128,(-70850994140450829089884393213055780852_i128),(-160622705865084653471479374551207928356_i128),(-29909507473574774590188638414467782940_i128)];
_9 = [_12.fld1,_12.fld1,_12.fld1,_12.fld1];
_13.2 = false;
_14 = !_13.2;
_8 = [88352939578788367790604377011472929957_i128,(-120545153237131651415534313974969738742_i128),(-115394752798070158356398229719781058576_i128),(-165504917976023688034426377938298124821_i128),3510362457182575409108493522760610715_i128,25230008895652132774702007486026052299_i128,77614280713369308723088711672258468364_i128,(-53735463234305595588860348413847811895_i128)];
_13.1 = _10 as u8;
_14 = _13.0 == _13.0;
RET = '\u{b399e}';
RET = '\u{20dcf}';
_7 = (_5, _1);
_17 = Adt41::Variant1 { fld0: (-127_i8) };
place!(Field::<i8>(Variant(_17, 1), 0)) = 51_i8;
_13.1 = !103_u8;
_12.fld0 = 0_usize + 7_usize;
_12.fld0 = _13.0 as usize;
_6 = [151016339383449026024112160745586056453_i128,(-16000962898380614777738415111338476884_i128),33231361276343337388717947381085767492_i128,149603476107218554064541589367431667138_i128,(-144721267745333617494409740182308517407_i128),62857800742230166359957779839575196786_i128,(-78462361421195375603346713197011211754_i128),124619298446093011509701019858081111174_i128];
Call(_12 = fn13(_11, _3, _1, _11, _2, _13.2, _9, _8, _7.1, _7.1, Field::<i8>(Variant(_17, 1), 0)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_15 = Field::<i8>(Variant(_17, 1), 0) as isize;
_6 = [(-72467039014516525930300162324801999370_i128),93319425619046621608057933704009404724_i128,(-107760458033511162821920930558660668816_i128),112075674998874829880345766414423846463_i128,(-109936192268966276117796291952623397390_i128),88420161149012378451727667683877915605_i128,(-77255095159929992816219309118779913262_i128),10312035715406533364763600928876314345_i128];
_6 = [72446219630185678855530884218859372448_i128,(-96763810026773475707453885939190207692_i128),31774567123179441467040776606291124225_i128,(-136884375741743938427463380232718903820_i128),150909322355803346628646853594449131574_i128,94232044248001271654613801855145753419_i128,(-166829604343705953492174490027002038656_i128),84329864979616613952403580083846961908_i128];
_13 = (1775902709455964017_i64, 69_u8, _14);
_20 = Adt46 { fld0: _12.fld0,fld1: _12.fld1 };
_13.2 = _14 | _14;
_13 = (8326481878581983774_i64, 239_u8, _14);
_21.0 = [(-1049977413_i32),107323372_i32,1089888184_i32];
_19 = _13.2;
_21.2.0 = 28379_u16 ^ 51297_u16;
_4 = [(-91421313566646674263366704599827673642_i128),(-36158063176997264726350204343724850800_i128),(-129538114889835011685622250599518928927_i128),(-62228382096698782533326086719002701867_i128),(-63472756695427228785249769385970764044_i128),(-90389297971353024942299565623806955263_i128),(-163792609797598163530031549871928339703_i128),(-69830385850509647558523428345155156360_i128)];
_23 = _13.1 as f64;
_7 = (_21.0, _1);
_13.0 = 1913514145_i32 as i64;
_12.fld1 = _20.fld1;
match _20.fld0 {
0 => bb1,
1 => bb5,
_ => bb4
}
}
bb4 = {
_12.fld1 = 81540531344864299497285256610193055766_u128 ^ 176900017421098927518622776816008365449_u128;
_2 = [(-113033972067141587178000111257198240895_i128),1173719217091305084299126314183152325_i128,37752367187411565362984079455199248301_i128,(-29446557738983680815368273371339975808_i128),156848587446894359638747730883998420134_i128,152340992760619171910118246420072624099_i128,106620576024654113760776290772019035679_i128,2529905646677425671927397706173384324_i128];
_2 = _1;
_11 = _13.0 as isize;
_13.1 = (-71_i8) as u8;
_2 = _3;
Call(_7.0 = core::intrinsics::transmute(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb5 = {
_21.1 = (-33696699988738845334384453153048844011_i128) as f32;
_20 = Adt46 { fld0: _12.fld0,fld1: _12.fld1 };
_21.2.3 = _23;
_15 = -_10;
_21.2 = (24589_u16, (-566538806_i32), 23586_i16, _23);
_21.1 = _12.fld1 as f32;
RET = '\u{33ad1}';
_21.3 = Field::<i8>(Variant(_17, 1), 0) as u128;
match _21.2.1 {
0 => bb3,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
340282366920938463463374607431201672650 => bb11,
_ => bb10
}
}
bb6 = {
_12.fld1 = 81540531344864299497285256610193055766_u128 ^ 176900017421098927518622776816008365449_u128;
_2 = [(-113033972067141587178000111257198240895_i128),1173719217091305084299126314183152325_i128,37752367187411565362984079455199248301_i128,(-29446557738983680815368273371339975808_i128),156848587446894359638747730883998420134_i128,152340992760619171910118246420072624099_i128,106620576024654113760776290772019035679_i128,2529905646677425671927397706173384324_i128];
_2 = _1;
_11 = _13.0 as isize;
_13.1 = (-71_i8) as u8;
_2 = _3;
Call(_7.0 = core::intrinsics::transmute(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_15 = Field::<i8>(Variant(_17, 1), 0) as isize;
_6 = [(-72467039014516525930300162324801999370_i128),93319425619046621608057933704009404724_i128,(-107760458033511162821920930558660668816_i128),112075674998874829880345766414423846463_i128,(-109936192268966276117796291952623397390_i128),88420161149012378451727667683877915605_i128,(-77255095159929992816219309118779913262_i128),10312035715406533364763600928876314345_i128];
_6 = [72446219630185678855530884218859372448_i128,(-96763810026773475707453885939190207692_i128),31774567123179441467040776606291124225_i128,(-136884375741743938427463380232718903820_i128),150909322355803346628646853594449131574_i128,94232044248001271654613801855145753419_i128,(-166829604343705953492174490027002038656_i128),84329864979616613952403580083846961908_i128];
_13 = (1775902709455964017_i64, 69_u8, _14);
_20 = Adt46 { fld0: _12.fld0,fld1: _12.fld1 };
_13.2 = _14 | _14;
_13 = (8326481878581983774_i64, 239_u8, _14);
_21.0 = [(-1049977413_i32),107323372_i32,1089888184_i32];
_19 = _13.2;
_21.2.0 = 28379_u16 ^ 51297_u16;
_4 = [(-91421313566646674263366704599827673642_i128),(-36158063176997264726350204343724850800_i128),(-129538114889835011685622250599518928927_i128),(-62228382096698782533326086719002701867_i128),(-63472756695427228785249769385970764044_i128),(-90389297971353024942299565623806955263_i128),(-163792609797598163530031549871928339703_i128),(-69830385850509647558523428345155156360_i128)];
_23 = _13.1 as f64;
_7 = (_21.0, _1);
_13.0 = 1913514145_i32 as i64;
_12.fld1 = _20.fld1;
match _20.fld0 {
0 => bb1,
1 => bb5,
_ => bb4
}
}
bb8 = {
_9 = [_12.fld1,_12.fld1,_12.fld1,_12.fld1];
_13.2 = _12.fld1 < _12.fld1;
_4 = [4153546277953120180226682004387164325_i128,(-88121119983150146919696677736014757973_i128),(-43227202749863055411470691375385285371_i128),104450986933169195781934123580003972918_i128,76833431896244792818458790097777065702_i128,(-70850994140450829089884393213055780852_i128),(-160622705865084653471479374551207928356_i128),(-29909507473574774590188638414467782940_i128)];
_9 = [_12.fld1,_12.fld1,_12.fld1,_12.fld1];
_13.2 = false;
_14 = !_13.2;
_8 = [88352939578788367790604377011472929957_i128,(-120545153237131651415534313974969738742_i128),(-115394752798070158356398229719781058576_i128),(-165504917976023688034426377938298124821_i128),3510362457182575409108493522760610715_i128,25230008895652132774702007486026052299_i128,77614280713369308723088711672258468364_i128,(-53735463234305595588860348413847811895_i128)];
_13.1 = _10 as u8;
_14 = _13.0 == _13.0;
RET = '\u{b399e}';
RET = '\u{20dcf}';
_7 = (_5, _1);
_17 = Adt41::Variant1 { fld0: (-127_i8) };
place!(Field::<i8>(Variant(_17, 1), 0)) = 51_i8;
_13.1 = !103_u8;
_12.fld0 = 0_usize + 7_usize;
_12.fld0 = _13.0 as usize;
_6 = [151016339383449026024112160745586056453_i128,(-16000962898380614777738415111338476884_i128),33231361276343337388717947381085767492_i128,149603476107218554064541589367431667138_i128,(-144721267745333617494409740182308517407_i128),62857800742230166359957779839575196786_i128,(-78462361421195375603346713197011211754_i128),124619298446093011509701019858081111174_i128];
Call(_12 = fn13(_11, _3, _1, _11, _2, _13.2, _9, _8, _7.1, _7.1, Field::<i8>(Variant(_17, 1), 0)), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_12.fld1 = 81540531344864299497285256610193055766_u128 ^ 176900017421098927518622776816008365449_u128;
_2 = [(-113033972067141587178000111257198240895_i128),1173719217091305084299126314183152325_i128,37752367187411565362984079455199248301_i128,(-29446557738983680815368273371339975808_i128),156848587446894359638747730883998420134_i128,152340992760619171910118246420072624099_i128,106620576024654113760776290772019035679_i128,2529905646677425671927397706173384324_i128];
_2 = _1;
_11 = _13.0 as isize;
_13.1 = (-71_i8) as u8;
_2 = _3;
Call(_7.0 = core::intrinsics::transmute(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
RET = '\u{88c14}';
_23 = -_21.2.3;
Goto(bb12)
}
bb12 = {
_13 = (643598442719050809_i64, 102_u8, _14);
_14 = _21.2.0 >= _21.2.0;
_13.1 = 176_u8 - 149_u8;
_22 = RET;
_19 = !_14;
place!(Field::<i8>(Variant(_17, 1), 0)) = _21.1 as i8;
_13.0 = _21.2.0 as i64;
_21.2.2 = (-22484_i16);
_21.2 = (45575_u16, 1900744915_i32, (-29365_i16), _23);
_12 = _20;
_24.fld1 = [6063465816615448163_u64,14305116338856229222_u64,8933019768194660508_u64];
_13.1 = 351507589_u32 as u8;
_9 = [_21.3,_12.fld1,_12.fld1,_20.fld1];
Goto(bb13)
}
bb13 = {
_13.1 = !36_u8;
_27 = _19;
_13.2 = !_27;
_24.fld1 = [13155075358289801204_u64,137464556822153385_u64,13859137522964832815_u64];
_25 = [_13.1,_13.1,_13.1,_13.1];
SetDiscriminant(_17, 0);
_1 = [27522116653195490930540902759027021010_i128,(-41728294627584909459885636398922002629_i128),(-15066971324982171592556112690692523145_i128),(-107175178840649703899522721278925575404_i128),144855897752890805112519428130424745545_i128,135670839469665492136333761330679058946_i128,(-74145318392208077471143623230597072149_i128),(-124755007474203868534070748184000740202_i128)];
_13.2 = _27;
_30.2 = -_21.2.2;
_29 = [_13.1,_13.1,_13.1,_13.1];
_13 = (8431387431992141575_i64, 166_u8, _14);
_9 = [_21.3,_20.fld1,_21.3,_20.fld1];
_6 = [(-57232138787335498840558891537826216_i128),142595613083537612665177469809211145245_i128,(-138639018249903684136311860988112405282_i128),87719666927508391431801050967896041329_i128,(-160860703597819080224945595295449910242_i128),(-52814474743797963458158464816089835542_i128),97380681878580257602459914096109848061_i128,139813078164683791415348693591074141268_i128];
_16 = Adt44::Variant1 { fld0: _21.2.0,fld1: _21 };
_9 = [_12.fld1,_21.3,_20.fld1,Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_16, 1), 1).3];
_29 = _25;
_30 = (Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_16, 1), 1).2.0, Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_16, 1), 1).2.1, Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_16, 1), 1).2.2, _23);
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_16, 1), 1)).2.2 = (-18545895727477153757609437653024822812_i128) as i16;
place!(Field::<[i32; 3]>(Variant(_17, 0), 2)) = _21.0;
_27 = _30.0 >= Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_16, 1), 1).2.0;
_12.fld0 = !_20.fld0;
Goto(bb14)
}
bb14 = {
_27 = _13.2;
_16 = Adt44::Variant1 { fld0: _30.0,fld1: _21 };
_31 = RET;
place!(Field::<u16>(Variant(_16, 1), 0)) = _21.2.0 ^ _21.2.0;
_12.fld0 = !_20.fld0;
_3 = _8;
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_16, 1), 1)).3 = !_20.fld1;
_21.2 = (Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_16, 1), 1).2.0, Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_16, 1), 1).2.1, _30.2, _30.3);
_30 = (_21.2.0, _21.2.1, _21.2.2, _21.2.3);
_2 = [102823739214707299844800430177620730097_i128,(-48998876611315841917335963217548169976_i128),136772469831506682480096647283381573458_i128,(-62384593745223047023613757251356197102_i128),(-120875548518415806408275348501878399128_i128),(-157365286676077786346091566008915646136_i128),26563987497412295738869336261349705153_i128,(-70156990200329067205404291122431558265_i128)];
_31 = _22;
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_16, 1), 1)) = (_7.0, _21.1, _21.2, _12.fld1);
_31 = _22;
_27 = _13.2;
_22 = RET;
SetDiscriminant(_16, 0);
_6 = [(-9402128640562069707097047738763403828_i128),65325448313107430987725348989955943672_i128,122815146106671912201160291002592357543_i128,59278578032518670826951017963986398341_i128,(-160477240858095084278762077205296314354_i128),117889978787470703839275602462004742451_i128,147437510144264965380897704843277731427_i128,(-31469467034744829926722297043195809056_i128)];
_12 = _20;
_17 = Adt41::Variant1 { fld0: 112_i8 };
place!(Field::<(u16, i32, i16, f64)>(Variant(_16, 0), 0)).0 = _21.2.0 | _21.2.0;
place!(Field::<(u16, i32, i16, f64)>(Variant(_16, 0), 0)).2 = _30.2 & _30.2;
place!(Field::<(u16, i32, i16, f64)>(Variant(_16, 0), 0)).0 = !_30.0;
_26 = _21.1;
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_16, 0), 4)).2.1 = !_21.2.1;
_5 = [_30.1,_21.2.1,_30.1];
place!(Field::<([i32; 3], f32, (u16, i32, i16, f64), u128)>(Variant(_16, 0), 4)).2.0 = !Field::<(u16, i32, i16, f64)>(Variant(_16, 0), 0).0;
_30.3 = -_23;
_32 = core::ptr::addr_of!(_11);
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(9_usize, 27_usize, Move(_27), 9_usize, Move(_9), 5_usize, Move(_5), 31_usize, Move(_31)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(9_usize, 7_usize, Move(_7), 13_usize, Move(_13), 14_usize, Move(_14), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(9_usize, 11_usize, Move(_11), 2_usize, Move(_2), 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: [i128; 8],mut _2: [i128; 8],mut _3: [i128; 8],mut _4: isize,mut _5: [i128; 8],mut _6: [i128; 8],mut _7: usize,mut _8: u128,mut _9: [i128; 8],mut _10: isize,mut _11: [i128; 8],mut _12: [i128; 8],mut _13: [i128; 8]) -> (i64, u8, bool) {
mir! {
type RET = (i64, u8, bool);
let _14: Adt46;
let _15: (*const i8,);
let _16: bool;
let _17: bool;
let _18: *mut char;
let _19: [i32; 3];
let _20: ([i32; 3], [i128; 8]);
let _21: i128;
let _22: [i32; 3];
let _23: Adt46;
let _24: i8;
let _25: Adt43;
let _26: [i32; 3];
let _27: [i32; 3];
let _28: u64;
let _29: char;
let _30: i32;
let _31: Adt41;
let _32: ();
let _33: ();
{
RET = (7411976496998642765_i64, 19_u8, true);
_12 = [_9[_7],_6[_7],_2[_7],_2[_7],_3[_7],_5[_7],_11[_7],_11[_7]];
_3 = [_13[_7],_13[_7],_9[_7],_13[_7],_9[_7],_6[_7],_5[_7],_6[_7]];
RET.0 = -7737368684685522046_i64;
_2 = _11;
_1[_7] = !_6[_7];
RET = ((-6277900234055304709_i64), 2_u8, false);
_14 = Adt46 { fld0: _7,fld1: _8 };
_2[_7] = -_13[_7];
Call(RET.1 = fn11(_5[_7], _11, _3, _1, _11[_7], _2[_7], _6, _12[_7], _12, _13, _3, _9[_7], _12[_7], _7, _12[_7], _3[_7]), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = _14.fld1 * _14.fld1;
_13 = [(-111556176959821987400102284061668995580_i128),(-112203566039713859644554994392846717837_i128),(-70569027952761721595688504606338561969_i128),68332676347091027428862949535284913663_i128,135160097225897038974604001063508909254_i128,(-41679487078879007573865816037267108611_i128),(-75302589756582111288472021944659295471_i128),161927714883642503633881688859994135122_i128];
_14 = Adt46 { fld0: _7,fld1: _8 };
RET.2 = false;
_14.fld0 = _7 % _7;
_14.fld1 = _8 * _8;
_7 = !_14.fld0;
RET.2 = !false;
_10 = _4 >> _14.fld1;
_1 = _6;
_20.0 = [(-2096999117_i32),(-3575207_i32),(-1054026137_i32)];
RET.0 = (-3300788080907333970_i64) + 8014970874896958387_i64;
_12 = [137539091092776046206732198176996311016_i128,(-72555261069900758134361098871162630253_i128),(-33658200598142738770476065815429087073_i128),(-34296705958629952241253897307211999827_i128),(-27329352639007184624595612462097609371_i128),(-61851996038673321562233574919146772155_i128),(-108450643349964795246477378803561794456_i128),17279646287024495209973590748249807849_i128];
_20.0 = [1625814216_i32,423652971_i32,(-1209492610_i32)];
RET.2 = true;
Goto(bb2)
}
bb2 = {
_17 = RET.2 | RET.2;
_1 = [98342685348473256124230306457979732174_i128,(-103688746554818465867119101434250960530_i128),110765453015874832902550833232871192968_i128,(-85473829868461631130207283745701114557_i128),(-70702522943920107302102167525638047428_i128),(-96860610377118604819145915725070242145_i128),56108439752931275970141302867743459546_i128,(-94215868007076827268844942101634164112_i128)];
_17 = _10 > _10;
_5 = [(-29790528833827910098081156369117304131_i128),(-134487159404851661915442772066726022994_i128),39756620708104012026156181244483680904_i128,(-13521763559394271810817409765717862721_i128),(-138476605418951637177681671811410910340_i128),(-84559013199696004185084360864644279866_i128),(-78025168430852368846486444710798872688_i128),(-159740941838809027286729981322969247405_i128)];
_21 = !(-102708133074779124628626529697056374316_i128);
Goto(bb3)
}
bb3 = {
_14 = Adt46 { fld0: _7,fld1: _8 };
_2 = _6;
_11 = [_21,_21,_21,_21,_21,_21,_21,_21];
_21 = !17996803634747045694929439037498459331_i128;
_21 = !151468499325914421846155636341235050429_i128;
_23 = _14;
_19 = [141533132_i32,(-931614460_i32),(-938395884_i32)];
RET.1 = 252_u8 & 149_u8;
RET.0 = (-326531198_i32) as i64;
_12 = [_21,_21,_21,_21,_21,_21,_21,_21];
_10 = !_4;
_2 = _1;
_9 = [_21,_21,_21,_21,_21,_21,_21,_21];
_20.0 = [800157647_i32,153811234_i32,(-1080208448_i32)];
_6 = _13;
_15.0 = core::ptr::addr_of!(_24);
_21 = -41148008970437870751522110789641252652_i128;
RET.1 = 211_u8;
_20.0 = [(-847842483_i32),(-1736192243_i32),1958708124_i32];
_20 = (_19, _13);
_8 = _14.fld1;
_7 = _23.fld0;
_24 = _17 as i8;
_7 = _23.fld0;
_16 = _17;
_16 = _17;
_6 = [_21,_21,_21,_21,_21,_21,_21,_21];
Goto(bb4)
}
bb4 = {
_23 = _14;
RET = ((-636088599445845518_i64), 88_u8, _17);
RET.0 = _4 as i64;
_22 = [(-963437950_i32),(-1599900460_i32),1527526304_i32];
_20.0 = [1712350021_i32,1470836787_i32,1261038086_i32];
_27 = [(-1859945113_i32),780242680_i32,(-1607469633_i32)];
Goto(bb5)
}
bb5 = {
Call(_32 = dump_var(10_usize, 1_usize, Move(_1), 8_usize, Move(_8), 5_usize, Move(_5), 27_usize, Move(_27)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_32 = dump_var(10_usize, 2_usize, Move(_2), 20_usize, Move(_20), 4_usize, Move(_4), 22_usize, Move(_22)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_32 = dump_var(10_usize, 13_usize, Move(_13), 6_usize, Move(_6), 33_usize, _33, 33_usize, _33), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: i128,mut _2: [i128; 8],mut _3: [i128; 8],mut _4: [i128; 8],mut _5: i128,mut _6: i128,mut _7: [i128; 8],mut _8: i128,mut _9: [i128; 8],mut _10: [i128; 8],mut _11: [i128; 8],mut _12: i128,mut _13: i128,mut _14: usize,mut _15: i128,mut _16: i128) -> u8 {
mir! {
type RET = u8;
let _17: bool;
let _18: [u128; 4];
let _19: f32;
let _20: (u16, i32, i16, f64);
let _21: (u16, i32, i16, f64);
let _22: [u64; 3];
let _23: isize;
let _24: u64;
let _25: bool;
let _26: *mut char;
let _27: bool;
let _28: Adt43;
let _29: char;
let _30: (u16, i32, i16, f64);
let _31: u64;
let _32: ();
let _33: ();
{
RET = 151_u8 ^ 12_u8;
_14 = 9536120872004689187_usize & 2_usize;
_13 = _16 - _12;
_8 = _15 + _6;
_6 = _13 - _16;
_4 = [_6,_12,_15,_5,_12,_5,_6,_13];
RET = 169_u8;
RET = 188_u8;
_1 = _8 + _15;
_12 = _5;
RET = !38_u8;
_5 = _6 >> _14;
_1 = _6 ^ _16;
_4 = _11;
_1 = false as i128;
_5 = RET as i128;
RET = 130_u8;
_10 = [_6,_8,_8,_8,_6,_8,_8,_12];
_14 = 10961192049917422932_usize ^ 5_usize;
_14 = 4_usize * 9226315375545923117_usize;
_7 = [_13,_8,_13,_8,_12,_6,_8,_6];
_14 = !1480308085703378656_usize;
_7 = [_6,_5,_6,_16,_15,_13,_6,_6];
_3 = [_13,_13,_6,_15,_6,_15,_12,_6];
_5 = _12;
_11 = [_6,_13,_16,_15,_5,_15,_12,_13];
_17 = false;
_13 = -_6;
RET = !64_u8;
_12 = _8;
Call(_12 = fn12(_6, _4, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _12 * _13;
RET = !225_u8;
_17 = !false;
_6 = _13;
_7 = [_12,_12,_1,_8,_1,_1,_6,_1];
RET = !123_u8;
_18 = [208422469244844662125865697781325579717_u128,103041891803410138392344983311378845006_u128,140816235784768745099658130297432684477_u128,312775481259467050975702879652075867094_u128];
match _5 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
248817343320891798150687722050209255945 => bb9,
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
_11 = [_1,_12,_1,_12,_12,_1,_12,_1];
_14 = 6192375824991652060_usize >> _8;
RET = 187297393762422513255446283803959783817_u128 as u8;
_16 = 105879356910451031751177568651144623393_u128 as i128;
_19 = (-2720966569185624995_i64) as f32;
_20.3 = 356213074_i32 as f64;
match _15 {
0 => bb10,
248817343320891798150687722050209255945 => bb12,
_ => bb11
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
RET = (-1765943901_i32) as u8;
_7 = _3;
_8 = RET as i128;
_5 = _12 & _1;
_20.0 = 62839_u16;
_15 = !_13;
_8 = _1;
_8 = _12 ^ _5;
_20.1 = -(-1732097975_i32);
_10 = [_8,_8,_8,_8,_5,_5,_6,_6];
_5 = _1;
_21 = (_20.0, _20.1, (-8532_i16), _20.3);
_1 = _21.2 as i128;
_8 = -_6;
_15 = 4066285558_u32 as i128;
_21.1 = _20.1;
Goto(bb13)
}
bb13 = {
_20.1 = !_21.1;
_13 = _5;
_20.3 = _21.3;
_5 = _13;
RET = 101_u8;
_21.0 = _20.0;
_22 = [8581864091990107069_u64,4245184429217892547_u64,3860907833749320684_u64];
_14 = 9805512974161971045_usize & 6_usize;
_14 = 6114164638562419344_i64 as usize;
_27 = _17;
_21.2 = !(-7185_i16);
_10 = [_13,_12,_13,_5,_6,_12,_5,_5];
_30 = _21;
RET = 1175411342026163453_i64 as u8;
_30 = (_21.0, _21.1, _21.2, _21.3);
_29 = '\u{2751e}';
_2 = _4;
_30.0 = !_21.0;
Goto(bb14)
}
bb14 = {
_24 = 107476958251819153684189207666862332267_u128 as u64;
_30.1 = _20.1;
_26 = core::ptr::addr_of_mut!(_29);
_21.0 = !_30.0;
_19 = 2164824683129082717470866522732542581_u128 as f32;
_24 = !931929844826839520_u64;
_21.0 = !_20.0;
_5 = _13 * _13;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(11_usize, 27_usize, Move(_27), 3_usize, Move(_3), 10_usize, Move(_10), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(11_usize, 9_usize, Move(_9), 18_usize, Move(_18), 14_usize, Move(_14), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(11_usize, 8_usize, Move(_8), 11_usize, Move(_11), 24_usize, Move(_24), 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: i128,mut _2: [i128; 8],mut _3: i128) -> i128 {
mir! {
type RET = i128;
let _4: [u64; 3];
let _5: bool;
let _6: ();
let _7: ();
{
_3 = true as i128;
_3 = _1;
RET = _1 * _3;
_5 = RET < RET;
_3 = _1;
_3 = 1223049683_u32 as i128;
Goto(bb1)
}
bb1 = {
Call(_6 = dump_var(12_usize, 3_usize, Move(_3), 1_usize, Move(_1), 7_usize, _7, 7_usize, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: isize,mut _2: [i128; 8],mut _3: [i128; 8],mut _4: isize,mut _5: [i128; 8],mut _6: bool,mut _7: [u128; 4],mut _8: [i128; 8],mut _9: [i128; 8],mut _10: [i128; 8],mut _11: i8) -> Adt46 {
mir! {
type RET = Adt46;
let _12: Adt46;
let _13: [u8; 4];
let _14: i32;
let _15: bool;
let _16: Adt55;
let _17: [i128; 8];
let _18: ([i32; 3], f32, (u16, i32, i16, f64), u128);
let _19: char;
let _20: Adt46;
let _21: bool;
let _22: [isize; 1];
let _23: Adt46;
let _24: Adt39;
let _25: f64;
let _26: Adt44;
let _27: ();
let _28: ();
{
_7 = [334506426418714323418579597551364032270_u128,44320885896428662072881898478564149452_u128,122792624084749361263139852265836979273_u128,222822467384913793283744892824216221006_u128];
_4 = _1;
_8 = _10;
match _11 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
51 => bb8,
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
RET = Adt46 { fld0: 15910424006100672544_usize,fld1: 120538530354468878243395325691298360259_u128 };
_6 = true;
_8 = [(-49947221327389449155320933899771711078_i128),(-20719905802456516829391080465071890884_i128),131610125177563994197606849787050958684_i128,(-117134055178859217798744834933951924096_i128),(-130894696720708047485785049306844626611_i128),(-167813303208227410230486807765412874814_i128),100676600996465538122687710863252153040_i128,(-159793526833360998035417867023899135780_i128)];
_5 = [(-54001359026502804541297501249096198955_i128),82396261353295884758818846129195146284_i128,(-79959894893421164508838544038479950626_i128),(-32195488307906060386035277448571049529_i128),(-121655873255693514345660227347323612612_i128),92701899897629802329718318363526398482_i128,145896253324634102592440664286577320590_i128,(-122755181841892565579427317106756379217_i128)];
RET.fld1 = 26329_i16 as u128;
_14 = 1561377625_i32 & (-373097731_i32);
_3 = [(-6308072895459567685604752334373635725_i128),144755117366467149185417147976258805037_i128,94338648293204983840670303405304068782_i128,143756883540455035561036992755381727605_i128,17469278433462316866340847087680134853_i128,120670193896161797793615631450375814297_i128,(-144679914632128468211845899046894798247_i128),(-46857647173021532801803623687740324506_i128)];
_15 = !_6;
match RET.fld0 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb9,
15910424006100672544 => bb11,
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
RET.fld0 = _4 as usize;
_12.fld1 = RET.fld1 - RET.fld1;
_18.2.0 = 60747_u16 >> RET.fld1;
_18.2.1 = _14;
RET = Adt46 { fld0: 1_usize,fld1: _12.fld1 };
_12.fld0 = RET.fld0;
_18.3 = 55_u8 as u128;
RET.fld1 = _18.3 ^ _12.fld1;
_19 = '\u{e34ba}';
_9 = [(-59798199478378127880427662614354367373_i128),(-104272858443132575256665826607220313_i128),29228662472762515251589812842735599549_i128,(-130503335866294330918243004011918864438_i128),135487725448380420150419802965470998077_i128,(-147951914814815362747204299551610433531_i128),88884070256099138510910559570705337412_i128,87158682020536549146413900580103746159_i128];
_20.fld0 = !_12.fld0;
_20 = Adt46 { fld0: _12.fld0,fld1: _12.fld1 };
RET.fld1 = RET.fld0 as u128;
_21 = _15;
Goto(bb12)
}
bb12 = {
_17 = [(-106402179200989922107970757096615644622_i128),(-165213771132320356614473195177150516979_i128),(-5760678607910770877926063558872251967_i128),83974325399668064412691581732366734074_i128,(-63902522112755214434530080784388840858_i128),132551186279010759853251336098672611254_i128,(-27333843669317663250084816551592613076_i128),(-75286434278246315393591440593607185028_i128)];
_20.fld1 = _15 as u128;
_13 = [138_u8,238_u8,77_u8,149_u8];
Goto(bb13)
}
bb13 = {
RET = _20;
_23 = Adt46 { fld0: RET.fld0,fld1: _12.fld1 };
_2 = [54489810775452988296947725792675788799_i128,(-141470417092149397673114289659103847610_i128),(-163209959603956847325161436434658391305_i128),(-142551738804187475062101041552404326358_i128),60880559096785069249981532624573317612_i128,128153272023414928461426752822921772883_i128,34171211143950304314704716315909438478_i128,(-49925956632387467317995971865759284466_i128)];
_9 = [(-22743945243553834442741943088730678193_i128),160631016208536548265287362179744049167_i128,84549872348240224761008311177019006037_i128,101393711169629476448623748771359440116_i128,94490120482003084289254327153681211478_i128,123542319560143800864364377481786638902_i128,(-164732087229430528708704759015418698547_i128),(-148597764849371013701008140259442219593_i128)];
_18.1 = (-144833951610293412746684918711126855438_i128) as f32;
_12.fld1 = _23.fld1;
_4 = !_1;
_8 = _5;
_17 = _9;
_18.2.1 = _11 as i32;
RET.fld1 = (-8663952058261694014_i64) as u128;
RET.fld1 = !_18.3;
_4 = -_1;
_18.3 = _23.fld1 + _12.fld1;
_2 = [(-166931211137526498684288540826307471326_i128),62169757676171083130065425392403919984_i128,(-83276924483630356557156765680733349419_i128),(-60734560139112208483781450807072223873_i128),146053034399950992791199499430713251699_i128,(-159500909016805326609031644399864296458_i128),(-51127679505663875666672498129844005474_i128),(-164792473535723482175448115697828604676_i128)];
match _12.fld0 {
0 => bb14,
2 => bb16,
3 => bb17,
4 => bb18,
1 => bb20,
_ => bb19
}
}
bb14 = {
_17 = [(-106402179200989922107970757096615644622_i128),(-165213771132320356614473195177150516979_i128),(-5760678607910770877926063558872251967_i128),83974325399668064412691581732366734074_i128,(-63902522112755214434530080784388840858_i128),132551186279010759853251336098672611254_i128,(-27333843669317663250084816551592613076_i128),(-75286434278246315393591440593607185028_i128)];
_20.fld1 = _15 as u128;
_13 = [138_u8,238_u8,77_u8,149_u8];
Goto(bb13)
}
bb15 = {
RET.fld0 = _4 as usize;
_12.fld1 = RET.fld1 - RET.fld1;
_18.2.0 = 60747_u16 >> RET.fld1;
_18.2.1 = _14;
RET = Adt46 { fld0: 1_usize,fld1: _12.fld1 };
_12.fld0 = RET.fld0;
_18.3 = 55_u8 as u128;
RET.fld1 = _18.3 ^ _12.fld1;
_19 = '\u{e34ba}';
_9 = [(-59798199478378127880427662614354367373_i128),(-104272858443132575256665826607220313_i128),29228662472762515251589812842735599549_i128,(-130503335866294330918243004011918864438_i128),135487725448380420150419802965470998077_i128,(-147951914814815362747204299551610433531_i128),88884070256099138510910559570705337412_i128,87158682020536549146413900580103746159_i128];
_20.fld0 = !_12.fld0;
_20 = Adt46 { fld0: _12.fld0,fld1: _12.fld1 };
RET.fld1 = RET.fld0 as u128;
_21 = _15;
Goto(bb12)
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
Return()
}
bb20 = {
_22 = [_1];
_3 = [148684965253698309161790691507775068516_i128,(-13548213514442864172794787545247975694_i128),129692022531722511533990660930187418906_i128,68898103570573472176225548542000701227_i128,(-119435712825243283963175743037008665517_i128),(-130579857981399386537724729856603599603_i128),74618785003164754020880853226978981675_i128,(-37850514647086843635393771141416504953_i128)];
_12.fld1 = 3459756356_u32 as u128;
_18.3 = RET.fld1 + _12.fld1;
_22 = [_1];
_18.2.3 = 5918774202446827788_u64 as f64;
_18.2.3 = 963067157810845019_i64 as f64;
_3 = [33031223412959909162391083016426786770_i128,(-52243972690258203235833696331595679352_i128),28397541733844938141979199697037656155_i128,(-160919004417959498849934437478555819302_i128),142462886096905356874096326374389976942_i128,(-154361174965604427629342320955822566037_i128),(-167677133487405148880273835512412169048_i128),39715242756727401674888603217018898853_i128];
_1 = (-1961038009206564422_i64) as isize;
_5 = [73229446676119342895031308944194792607_i128,27245348731386008031782664545575321255_i128,119706350995731600560181558079604496569_i128,(-31033573943719735784038385311079469431_i128),(-36043566188779268636351775449843734805_i128),(-164892294928755906342500005322871477514_i128),(-74127508819191949279101265492786207179_i128),(-72437763477587526054255004490747620385_i128)];
RET.fld1 = 16833892043814516207_u64 as u128;
RET = Adt46 { fld0: _20.fld0,fld1: _23.fld1 };
_18.2.1 = -_14;
_25 = _11 as f64;
_18.1 = 3636486267511050983_i64 as f32;
RET = _20;
_18.2 = (61721_u16, _14, (-15503_i16), _25);
_25 = _18.2.2 as f64;
RET.fld1 = _18.3;
_11 = -113_i8;
_12.fld0 = RET.fld0 / _20.fld0;
_15 = _21;
_17 = [87201475223701711227465874827822462692_i128,126777773835081287927974539583453798600_i128,57584531249092074497179203517640787676_i128,(-140360665615506615382660673586360744594_i128),27448639962789106406911672530958030026_i128,(-14073092894819760389119754316456191244_i128),(-114075719466394386503033649116733080615_i128),(-64232167184109341433733414427181190360_i128)];
_8 = [138245664595741993149854390756179920506_i128,13537962921267002479277302279188339684_i128,(-116987146481014811275792642457057258448_i128),(-92191534342134544565060355494633127968_i128),150161276696234725578430205611223447298_i128,(-56763056734033288155956666069406659688_i128),(-113552183263870809415788530672703838788_i128),(-73984319932943181323361085164522401094_i128)];
Goto(bb21)
}
bb21 = {
Call(_27 = dump_var(13_usize, 2_usize, Move(_2), 13_usize, Move(_13), 4_usize, Move(_4), 22_usize, Move(_22)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_27 = dump_var(13_usize, 1_usize, Move(_1), 8_usize, Move(_8), 19_usize, Move(_19), 9_usize, Move(_9)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_27 = dump_var(13_usize, 17_usize, Move(_17), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: i16,mut _2: [u64; 3],mut _3: isize,mut _4: [u8; 2],mut _5: i16,mut _6: i16,mut _7: isize,mut _8: isize,mut _9: [u64; 3],mut _10: ([i32; 3], [i128; 8]),mut _11: [i128; 8],mut _12: *const i16,mut _13: i16,mut _14: i16,mut _15: i16) -> [i32; 3] {
mir! {
type RET = [i32; 3];
let _16: (*const i16,);
let _17: (*const i16,);
let _18: isize;
let _19: isize;
let _20: u128;
let _21: bool;
let _22: f32;
let _23: usize;
let _24: [u8; 2];
let _25: ([i32; 3], [i128; 8]);
let _26: (i64, u8, bool);
let _27: &'static f32;
let _28: u16;
let _29: isize;
let _30: Adt39;
let _31: Adt41;
let _32: char;
let _33: i16;
let _34: *const i8;
let _35: Adt46;
let _36: i32;
let _37: i64;
let _38: isize;
let _39: u128;
let _40: [u64; 3];
let _41: i64;
let _42: isize;
let _43: ();
let _44: ();
{
_9 = [6647628640901551408_u64,14952135853884906473_u64,8455628001102392050_u64];
RET = [(-881361216_i32),(-806912430_i32),(-462591126_i32)];
_14 = _1 ^ (*_12);
_3 = _7;
_4 = [214_u8,40_u8];
_8 = _7;
_9 = [1892436771396773597_u64,202195052543228563_u64,4756925901677074690_u64];
_16 = (_12,);
_13 = _5 >> (*_12);
_15 = _13 ^ _6;
_16 = (_12,);
_10 = (RET, _11);
_16.0 = core::ptr::addr_of!(_14);
Goto(bb1)
}
bb1 = {
_19 = _7 * _3;
(*_12) = !_6;
_17 = _16;
_8 = 8053170330837765637_i64 as isize;
_17 = (_16.0,);
_1 = !_13;
_10.0 = [559937645_i32,587651244_i32,(-2138561497_i32)];
_22 = 108_i8 as f32;
_19 = _7 * _3;
_10.1 = [67655366515723944684841281603185923805_i128,(-139495786157948216791279746932338493777_i128),(-54599595358664504807757787694055749841_i128),(-70019767753456741504040513664647552346_i128),75655312027984813151644159639139664620_i128,115441749906517725518648327557907411896_i128,66764980824449131509429970598810683191_i128,112843138771034663920272763687004167490_i128];
_6 = _15 >> _13;
(*_12) = true as i16;
Goto(bb2)
}
bb2 = {
_16.0 = core::ptr::addr_of!((*_12));
(*_12) = _14;
_20 = 31613962982943566269527967340997181567_u128;
_1 = !(*_12);
_7 = _19 >> (*_12);
_25.0 = [(-1220142144_i32),(-1847815700_i32),(-755243367_i32)];
_21 = true & true;
_8 = _3 * _7;
_12 = core::ptr::addr_of!(_5);
_7 = _19 << _6;
_26 = (5961796060923885959_i64, 105_u8, _21);
_18 = -_7;
_17 = (_16.0,);
_10.1 = [(-146893857082233677145423518039783109106_i128),50833560548656646717705363190114615791_i128,42871748617971495176436420578419864579_i128,83105229561474352771519247147768412805_i128,(-142501389410443142623918069644096380567_i128),51990671310706894039149120464612353986_i128,(-112607753564587824778752445666890360512_i128),(-44218150574079629882548255061873268816_i128)];
RET = [912803131_i32,1156257012_i32,(-252091384_i32)];
_25 = _10;
_29 = !_8;
match _26.1 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
105 => bb11,
_ => bb10
}
}
bb3 = {
_19 = _7 * _3;
(*_12) = !_6;
_17 = _16;
_8 = 8053170330837765637_i64 as isize;
_17 = (_16.0,);
_1 = !_13;
_10.0 = [559937645_i32,587651244_i32,(-2138561497_i32)];
_22 = 108_i8 as f32;
_19 = _7 * _3;
_10.1 = [67655366515723944684841281603185923805_i128,(-139495786157948216791279746932338493777_i128),(-54599595358664504807757787694055749841_i128),(-70019767753456741504040513664647552346_i128),75655312027984813151644159639139664620_i128,115441749906517725518648327557907411896_i128,66764980824449131509429970598810683191_i128,112843138771034663920272763687004167490_i128];
_6 = _15 >> _13;
(*_12) = true as i16;
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
Return()
}
bb11 = {
(*_12) = _22 as i16;
_6 = _5 + _15;
_25 = _10;
_19 = -_7;
_19 = _18;
_3 = 65_i8 as isize;
_25.1 = _10.1;
_10.0 = [(-116631994_i32),(-718415587_i32),(-1777812439_i32)];
_7 = _19;
_3 = _8;
_25.1 = [(-110263599484966342389201550325957015006_i128),(-117192572715846969555889453696125216624_i128),(-93718682419765087138216456678312826281_i128),63554899516004249823637052314053557926_i128,13952552575118329707117137859707422682_i128,37723342490629003295409897953120181989_i128,(-25599333492512549613411937049573260508_i128),27023713479506106923461410395498850184_i128];
_10 = (_25.0, _25.1);
_17 = (_16.0,);
_9 = _2;
_31 = Adt41::Variant1 { fld0: 59_i8 };
_12 = core::ptr::addr_of!(_15);
_17 = (_12,);
_4 = [_26.1,_26.1];
_25.1 = [(-169879783293869320615154623471687152376_i128),(-102815312655630691452423200809752419778_i128),37412709153283247601874488614598267759_i128,(-23406726660878251209402317357117732862_i128),(-108565870429368484028550407874127221524_i128),(-134010989380585256781990544796272887871_i128),(-39496592719493434823010394474760954676_i128),81666522613256187034680424847423999590_i128];
_8 = _18;
_3 = _18 | _29;
_35.fld1 = _20 << (*_12);
Goto(bb12)
}
bb12 = {
_11 = [(-163350229483973017131376713340123526854_i128),74235252806165082531269625536307434576_i128,85411724772954857755093124964786893526_i128,(-44722931962396349695707326236855962686_i128),91378140512381879679816426897886522789_i128,147976332917709659734879459581401235559_i128,90808322276766209914232458505437400289_i128,(-119009317021755968861552324625991392993_i128)];
_24 = [_26.1,_26.1];
_29 = _8 - _7;
_3 = _8 >> _35.fld1;
_2 = [10434860388321983948_u64,6605268800156507048_u64,2771313828429519618_u64];
_17 = _16;
_35 = Adt46 { fld0: 11429115037991725008_usize,fld1: _20 };
_24 = [_26.1,_26.1];
Goto(bb13)
}
bb13 = {
_26.0 = 6563003780188068067_i64 - (-5054968969645380737_i64);
_29 = _19 | _18;
_12 = core::ptr::addr_of!(_14);
_25.1 = [87972405933115995977873049940912297632_i128,67071251569066436250996109740401574766_i128,22933537040579697289422969707759455455_i128,63270069019733637478620125378915203219_i128,(-71365500923425924097330845514678879589_i128),98832535887923719705203241638446616443_i128,(-127186144875296329542006982137991205864_i128),(-146037967988742953618203740395494099033_i128)];
_36 = _21 as i32;
_9 = _2;
Goto(bb14)
}
bb14 = {
place!(Field::<i8>(Variant(_31, 1), 0)) = (-21_i8) | (-111_i8);
_32 = '\u{f9168}';
_35 = Adt46 { fld0: 12010095526031402823_usize,fld1: _20 };
_12 = _17.0;
_32 = '\u{a3144}';
_20 = _21 as u128;
_2 = _9;
place!(Field::<i8>(Variant(_31, 1), 0)) = 62_i8;
RET = _10.0;
SetDiscriminant(_31, 3);
_26.0 = (-5618902505724714305_i64);
_26.2 = !_21;
_17 = (_12,);
place!(Field::<(*const i16,)>(Variant(_31, 3), 0)) = (_16.0,);
RET = [_36,_36,_36];
_10 = (_25.0, _11);
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(14_usize, 10_usize, Move(_10), 4_usize, Move(_4), 25_usize, Move(_25), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(14_usize, 24_usize, Move(_24), 7_usize, Move(_7), 13_usize, Move(_13), 26_usize, Move(_26)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(14_usize, 14_usize, Move(_14), 18_usize, Move(_18), 3_usize, Move(_3), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: i32,mut _2: [i32; 3],mut _3: ([i32; 3], [i128; 8]),mut _4: isize,mut _5: bool,mut _6: (u16, i32, i16, f64),mut _7: (u16, i32, i16, f64),mut _8: [u8; 2],mut _9: i32,mut _10: (u16, i32, i16, f64),mut _11: i16,mut _12: ([i32; 3], [i128; 8]),mut _13: [i32; 3],mut _14: [i128; 8],mut _15: [i32; 3],mut _16: isize) -> [i128; 8] {
mir! {
type RET = [i128; 8];
let _17: Adt44;
let _18: [u8; 4];
let _19: i8;
let _20: [u64; 3];
let _21: (*const i8,);
let _22: i128;
let _23: *mut i8;
let _24: Adt46;
let _25: f32;
let _26: [isize; 1];
let _27: [u8; 4];
let _28: char;
let _29: Adt45;
let _30: ();
let _31: ();
{
RET = [150881197830087902023752312943953113087_i128,(-125438456590998497829317809597600998237_i128),84510426099985685946380678523171551178_i128,21210313292602987415709190239821211885_i128,151941644947519344074319424140757779969_i128,(-127135307358257981513588438846268146815_i128),66692038042965297226925854655058881320_i128,(-128179315737384512549003935118432468807_i128)];
_13 = [_10.1,_1,_1];
_7.0 = _10.0;
_10 = (_6.0, _6.1, _6.2, _6.3);
_7.3 = _10.3 - _6.3;
_9 = !_7.1;
_7.1 = _9 * _1;
_6.2 = _7.2;
_9 = _4 as i32;
_4 = _16;
Goto(bb1)
}
bb1 = {
_15 = _12.0;
_7.1 = _1 | _10.1;
_3 = (_13, _14);
_2 = _13;
RET = _12.1;
_6.1 = _10.1;
_7.0 = 3479109610930519962_u64 as u16;
_7.1 = _10.1;
_21.0 = core::ptr::addr_of!(_19);
_2 = [_7.1,_10.1,_10.1];
_12.0 = [_6.1,_6.1,_1];
_6.3 = _7.3;
_6.2 = -_11;
_2 = [_1,_10.1,_10.1];
_7 = _10;
_10 = (_7.0, _6.1, _6.2, _6.3);
_7.0 = _6.0 - _10.0;
match _10.1 {
1506251084 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_6 = (_7.0, _1, _11, _7.3);
_23 = core::ptr::addr_of_mut!(_19);
Goto(bb4)
}
bb4 = {
(*_23) = -(-111_i8);
_10 = (_7.0, _6.1, _7.2, _6.3);
_12 = (_2, _3.1);
_6.1 = _4 as i32;
_10.2 = _6.2 | _11;
_6.2 = _7.2 * _7.2;
_3.0 = _2;
_18 = [31_u8,144_u8,220_u8,36_u8];
_7 = _10;
_15 = [_1,_7.1,_7.1];
_11 = -_7.2;
_21.0 = core::ptr::addr_of!((*_23));
Goto(bb5)
}
bb5 = {
_6.3 = -_7.3;
_3.1 = [(-64493400470004409885025185613673428076_i128),54074520755814131572983321526044884481_i128,90098899350960862537132862037686542553_i128,(-162017373917949099981771095547132369856_i128),97327693373279520553372873264396299115_i128,123718362220487381618478619926857253461_i128,132752029649921822400808890663247881921_i128,112953147046694664330399747408148786315_i128];
_7.1 = !_9;
_14 = [141777304930495021324910230456299452671_i128,91706573800888461570955143298729617266_i128,115152876144882162707380396909407211116_i128,47718483311819487524134757421388665224_i128,98581409830679407769851874182102389125_i128,(-126111477598405077575111374386652874094_i128),(-50467663708485824585424066672983918963_i128),(-144769411348587367100721342414554572349_i128)];
_3.1 = _14;
_6.2 = !_10.2;
_6.3 = (-30441106008128006488401318191824822135_i128) as f64;
_6.1 = _11 as i32;
_7.0 = !_10.0;
_12.0 = [_6.1,_1,_9];
_3.1 = RET;
_12.1 = _14;
_7.0 = _10.0;
RET = _12.1;
match _10.1 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
1506251084 => bb11,
_ => bb10
}
}
bb6 = {
(*_23) = -(-111_i8);
_10 = (_7.0, _6.1, _7.2, _6.3);
_12 = (_2, _3.1);
_6.1 = _4 as i32;
_10.2 = _6.2 | _11;
_6.2 = _7.2 * _7.2;
_3.0 = _2;
_18 = [31_u8,144_u8,220_u8,36_u8];
_7 = _10;
_15 = [_1,_7.1,_7.1];
_11 = -_7.2;
_21.0 = core::ptr::addr_of!((*_23));
Goto(bb5)
}
bb7 = {
_6 = (_7.0, _1, _11, _7.3);
_23 = core::ptr::addr_of_mut!(_19);
Goto(bb4)
}
bb8 = {
Return()
}
bb9 = {
_15 = _12.0;
_7.1 = _1 | _10.1;
_3 = (_13, _14);
_2 = _13;
RET = _12.1;
_6.1 = _10.1;
_7.0 = 3479109610930519962_u64 as u16;
_7.1 = _10.1;
_21.0 = core::ptr::addr_of!(_19);
_2 = [_7.1,_10.1,_10.1];
_12.0 = [_6.1,_6.1,_1];
_6.3 = _7.3;
_6.2 = -_11;
_2 = [_1,_10.1,_10.1];
_7 = _10;
_10 = (_7.0, _6.1, _6.2, _6.3);
_7.0 = _6.0 - _10.0;
match _10.1 {
1506251084 => bb3,
_ => bb2
}
}
bb10 = {
Return()
}
bb11 = {
_6.2 = !_7.2;
_3.0 = _2;
_18 = [24_u8,64_u8,118_u8,53_u8];
_22 = _5 as i128;
RET = [_22,_22,_22,_22,_22,_22,_22,_22];
_20 = [8989588689516104641_u64,6822773386301392096_u64,11912638154620088420_u64];
_25 = 7682287692855286574_u64 as f32;
_7.2 = _6.2 ^ _11;
_3 = (_12.0, _12.1);
RET = [_22,_22,_22,_22,_22,_22,_22,_22];
_27 = [241_u8,236_u8,104_u8,90_u8];
_24 = Adt46 { fld0: 14855199321021594238_usize,fld1: 219073937609366585526526938078088358687_u128 };
_13 = _12.0;
_25 = 4472124663160873368_i64 as f32;
_29.fld1 = _12.1;
_2 = [_6.1,_1,_6.1];
_19 = 80_i8 >> _1;
_7.2 = _6.2;
_10.3 = _6.3 + _6.3;
_9 = _1;
_7.3 = -_6.3;
Goto(bb12)
}
bb12 = {
Call(_30 = dump_var(15_usize, 1_usize, Move(_1), 27_usize, Move(_27), 5_usize, Move(_5), 19_usize, Move(_19)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_30 = dump_var(15_usize, 18_usize, Move(_18), 12_usize, Move(_12), 15_usize, Move(_15), 22_usize, Move(_22)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_30 = dump_var(15_usize, 2_usize, Move(_2), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: [u128; 4],mut _2: [u128; 4],mut _3: [i128; 8],mut _4: [i128; 8]) -> (i64, u8, bool) {
mir! {
type RET = (i64, u8, bool);
let _5: char;
let _6: isize;
let _7: (u16, i32, i16, f64);
let _8: [i128; 8];
let _9: [i32; 3];
let _10: Adt51;
let _11: [u64; 3];
let _12: u8;
let _13: *mut char;
let _14: i8;
let _15: i8;
let _16: *mut *const i16;
let _17: u32;
let _18: ([i32; 3], [i128; 8]);
let _19: bool;
let _20: *const isize;
let _21: Adt46;
let _22: f64;
let _23: ();
let _24: ();
{
RET = ((-817646702044761884_i64), 8_u8, true);
_1 = [215107960237145454915557048232296151371_u128,43568349077532009985335383551265405257_u128,94443427900172621268986146158866188075_u128,333398824111932439415111209916950538190_u128];
_2 = _1;
RET.2 = true;
_3 = _4;
_5 = '\u{f9830}';
RET.1 = 142_u8 & 24_u8;
RET = ((-284041608857218553_i64), 79_u8, false);
RET.0 = 8593611625541380949_i64 >> RET.1;
_2 = [300607363788107909952874149529051030438_u128,252876804567227380416334656501144792190_u128,294379319386752008040291618133124872889_u128,38951199772768883998317180248654723002_u128];
match RET.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
79 => bb9,
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
_6 = 62_i8 as isize;
RET = (3286028030561287589_i64, 197_u8, false);
_2 = _1;
_1 = _2;
RET.0 = (-8598496777192582179_i64);
RET = ((-4647602950768155313_i64), 171_u8, true);
RET.1 = 36_u8 ^ 109_u8;
_4 = _3;
_6 = -(-22_isize);
_6 = RET.1 as isize;
RET.0 = 8206619318506363310_i64;
_7.0 = 17885_u16;
_7.3 = 71131739402088610712869574370752963175_i128 as f64;
RET.0 = (-2527448963805991676_i64) - (-6871567464754834418_i64);
RET = ((-2939663968933446998_i64), 59_u8, true);
_8 = _4;
_5 = '\u{e7967}';
_7.1 = 927412708_u32 as i32;
_5 = '\u{91f93}';
_4 = [(-147069845433058945862362287498222050098_i128),(-111488210832666247554603573918563050477_i128),121686145499811368859631473015704134051_i128,22966797339137882076912125849322780919_i128,(-24087309910667593708065237087290346343_i128),126362069407258720459054727915182278309_i128,(-167802293165096084759750999989141825199_i128),37924420789360821078602579118225034630_i128];
Goto(bb10)
}
bb10 = {
_5 = '\u{b02dd}';
RET.0 = !8346450750273125360_i64;
_7.0 = 29598_u16;
_9 = [_7.1,_7.1,_7.1];
_8 = [(-49125011566417525622439589935012423415_i128),(-116004067163821574446818475862192670747_i128),(-149398424214169618609014743794549712531_i128),(-28648056742107597275429129864529783915_i128),(-34442671716826911702751299637634170853_i128),(-30374419257174843020657252293795182079_i128),153146518588657538699801805665248122380_i128,(-95651584903209409824176791298251867949_i128)];
_1 = _2;
RET.1 = _7.1 as u8;
_7.2 = !6596_i16;
_7.2 = 6064142776544645611_u64 as i16;
RET = ((-3150680901431323933_i64), 162_u8, true);
RET.1 = !171_u8;
_6 = (-9223372036854775808_isize);
_7.1 = -(-247471628_i32);
_9 = [_7.1,_7.1,_7.1];
RET.0 = 1316275814750703428_i64 + (-4460283599246374736_i64);
_7.0 = 25122_u16;
_6 = _5 as isize;
RET.0 = (-5842638888147792610_i64);
_4 = [15520996264306428217659994233601337966_i128,(-123630796074514028628550352987375726792_i128),93404886229636394464793193840320474765_i128,77601394755505642817137931736618587096_i128,92066164975654007323139585437490068171_i128,154288708158736898023402912159641443355_i128,87547172402664561749654300519541486705_i128,92084748545381661413913108048142230950_i128];
Call(_6 = core::intrinsics::transmute(RET.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_1 = _2;
RET.1 = _7.0 as u8;
RET.2 = _7.3 < _7.3;
RET.0 = (-19_i8) as i64;
_8 = [(-9258621493504107844654712679381460841_i128),(-66449201023234204254868047401790257825_i128),127175941746443673810140749885334084406_i128,(-98078839672727030064210635503511810902_i128),22115338525281934602956784914427580215_i128,25669130097535373746900350526290018259_i128,61405673840327861490453518361617014178_i128,136048188475041026923211640772738292259_i128];
_8 = [(-88742156989494331802080311035018177509_i128),7696274553069602167632328458505115656_i128,105593467638115910374971109021107562190_i128,163213738555371187478523791061057891512_i128,32619653138218788857015668206590092239_i128,13306596076781282963688975632433120348_i128,(-147630650866599344845382232273332482661_i128),88932918796300962108952176919269140083_i128];
RET.2 = !true;
_3 = [155795319598383906009565569000260143501_i128,(-30571186191804302373024374131714954080_i128),(-131487122625277193983076784537605071775_i128),(-116009378338744514586678670322502484680_i128),88379385619510639044673077176519877702_i128,(-141976760223706126498102289900410805284_i128),40406582317113876476174961617432500715_i128,23872399884826915835910182219835674421_i128];
RET = ((-1905991284346128489_i64), 98_u8, false);
RET = (5928733943586383268_i64, 246_u8, false);
_3 = [137616450502537828954337801940697298342_i128,(-9459365606535034865141987329922365059_i128),(-81075427418891766040016126778212163762_i128),(-153757375129129737639202904131840934345_i128),(-168951081760302967258883922205789579279_i128),156820607944314190013862978301900346213_i128,(-79592280177498937080320358741751513387_i128),64018628650965173022229874175738008270_i128];
_11 = [13861505338276142204_u64,10530715766187034739_u64,17758351645795022295_u64];
_7.1 = 986130733_i32;
RET.1 = 11698695924184221185_usize as u8;
RET = ((-5020823747148062303_i64), 103_u8, false);
_7.3 = RET.1 as f64;
_12 = RET.1;
_1 = [207370445179736689783257421504528669366_u128,333575579535714779439999337767068706100_u128,339150008970026846191261148770560934272_u128,269705995820798696986919752543023079236_u128];
RET.2 = true;
Goto(bb12)
}
bb12 = {
RET.0 = 3518917151618736802_i64;
RET.0 = _7.2 as i64;
RET.2 = RET.1 != _12;
_7.2 = (-21023_i16) - (-20628_i16);
Goto(bb13)
}
bb13 = {
_13 = core::ptr::addr_of_mut!(_5);
_13 = core::ptr::addr_of_mut!((*_13));
_5 = '\u{c4f9f}';
RET.2 = false;
_3 = [56684751362058314793767898923062891273_i128,74874987391825337531604645694024725952_i128,(-170106448158681381202483089090119571879_i128),44103029430369116256988414100902052121_i128,85809788695020908337951247098661644682_i128,(-79280425813137563475783991715441066303_i128),159215029547165880059627615012946955661_i128,(-39358887575599751511399526056335269792_i128)];
_3 = [(-17378496829822942007828780374623719647_i128),(-52873353749733946777051539913674976665_i128),134950784087207556525509570574399767342_i128,125100195304748944243490338985754804514_i128,9383137329469011194120739665536339119_i128,30002913570487297975367547532415746989_i128,107491264695974561661374236704681116365_i128,41223549455465315941850709680420564340_i128];
RET.0 = (-4558342018017452210_i64) ^ (-791050916857244775_i64);
RET = (3906387877169557889_i64, _12, false);
RET.0 = -(-5269167551397018027_i64);
(*_13) = '\u{a1c8b}';
_14 = 54_i8 | 37_i8;
_4 = [39127585544164862754066754482618113192_i128,9161694993897321192289374300893469536_i128,151511223988426179205481690109835177350_i128,(-120343013921541512390732372441009904957_i128),(-51360226998806876813648219165739721666_i128),(-66400586192045443814241059000270377878_i128),(-5859009927094821276075835125867714351_i128),138419111604056876084525585322810036795_i128];
_18.0 = [_7.1,_7.1,_7.1];
_14 = (-59_i8) | 33_i8;
RET = ((-8518258638745328725_i64), _12, true);
_18 = (_9, _4);
_3 = [(-89215152676089969039475721348410608398_i128),28096410258177707602866229480309959997_i128,24455979419685439951363409641184849773_i128,157790479777108970651356853600589130802_i128,110758211943050597300896216375274175742_i128,41660964635020819287663498425379271082_i128,68003397082209774153807416467100248780_i128,158965043847354845728903237158980275508_i128];
(*_13) = '\u{9fce1}';
_15 = 53227614494943928016503732871512089061_u128 as i8;
(*_13) = '\u{107a47}';
_7.0 = 51766_u16 >> _12;
match _12 {
0 => bb11,
103 => bb14,
_ => bb9
}
}
bb14 = {
_7.2 = _7.0 as i16;
_7.3 = 1718703376_u32 as f64;
RET.2 = true;
_19 = RET.2;
_17 = !636465821_u32;
_13 = core::ptr::addr_of_mut!((*_13));
(*_13) = '\u{ca0e1}';
_17 = !4163265582_u32;
RET = ((-246474789299374931_i64), _12, _19);
_20 = core::ptr::addr_of!(_6);
_20 = core::ptr::addr_of!((*_20));
_22 = -_7.3;
RET.1 = _6 as u8;
Goto(bb15)
}
bb15 = {
Call(_23 = dump_var(16_usize, 14_usize, Move(_14), 9_usize, Move(_9), 15_usize, Move(_15), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_23 = dump_var(16_usize, 5_usize, Move(_5), 3_usize, Move(_3), 19_usize, Move(_19), 24_usize, _24), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: ([i32; 3], [i128; 8]),mut _2: [i128; 8],mut _3: [u128; 4],mut _4: i32,mut _5: isize,mut _6: [i128; 8],mut _7: *const i16,mut _8: Adt54,mut _9: [i32; 3],mut _10: *const i16) -> i16 {
mir! {
type RET = i16;
let _11: u64;
let _12: i64;
let _13: [u8; 2];
let _14: Adt39;
let _15: (i64, u8, bool);
let _16: [isize; 1];
let _17: [u128; 4];
let _18: u8;
let _19: *mut *const i16;
let _20: Adt46;
let _21: *const i8;
let _22: bool;
let _23: bool;
let _24: [u8; 2];
let _25: ([i32; 3], [i128; 8]);
let _26: u64;
let _27: Adt52;
let _28: [u8; 2];
let _29: isize;
let _30: [u8; 4];
let _31: ();
let _32: ();
{
_10 = core::ptr::addr_of!(RET);
RET = 3118004046_u32 as i16;
RET = (-5999286694925468968_i64) as i16;
(*_10) = (-9745_i16) & (-26107_i16);
_9 = [_4,_4,_4];
_3 = [244250816156921363122957613338032062093_u128,311599517023863384597856109959172714390_u128,324293700247902035388404903843545965607_u128,327403187407508389021647448129045756328_u128];
place!(Field::<([i32; 3], [i128; 8])>(Variant(_8, 0), 1)) = (_9, _2);
_3 = [258515955232142116665563134202545562258_u128,332174236150576055366553163714703931147_u128,189110655310246098574481513387738143789_u128,235125760661525683702995965274266334502_u128];
_10 = core::ptr::addr_of!(RET);
_11 = Field::<u64>(Variant(_8, 0), 0);
(*_10) = !(-15234_i16);
_10 = _7;
_10 = core::ptr::addr_of!(RET);
_3 = [177753060946385972726840520119469382263_u128,137164432758885402561380322543622602111_u128,199839757942682607162436668933419418106_u128,55771576086399853388931443883305239721_u128];
_10 = _7;
_1 = (Field::<([i32; 3], [i128; 8])>(Variant(_8, 0), 1).0, _6);
_1.1 = [(-20407495644813957638790508725277216512_i128),(-48389168909135801940260293724882934118_i128),(-117311421438936101602824456862048167983_i128),121822135877144102066928925805792035034_i128,(-110499393179759763899227615046595070880_i128),(-144951027879567538000595068297633900607_i128),(-124308291894789180835335895656576040106_i128),(-165453663079403340767875771124869695908_i128)];
match _4 {
80699738 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
place!(Field::<u64>(Variant(_8, 0), 0)) = !_11;
_15.1 = !63_u8;
RET = 1413_i16;
_15.1 = !79_u8;
_15.1 = true as u8;
match _4 {
80699738 => bb3,
_ => bb1
}
}
bb3 = {
_6 = _2;
_15 = ((-2439178920752108913_i64), 39_u8, false);
_2 = [(-71666980436510184430838053538453893111_i128),(-115896437707964008359503010827782524431_i128),120411713460952306048066004473370958960_i128,(-5529617640683832454385620304185800965_i128),(-139101239233589624018346161891945066530_i128),(-120109483299250398497155910493272839248_i128),(-76873975071185427316290945273825064963_i128),(-15317790670095813443439433661257081889_i128)];
_16 = [_5];
_14 = Adt39::Variant1 { fld0: _9 };
SetDiscriminant(_8, 1);
place!(Field::<(u16, i32, i16, f64)>(Variant(_8, 1), 7)).3 = _4 as f64;
place!(Field::<Adt46>(Variant(_8, 1), 3)).fld1 = 314815275191127490605336629863798397723_u128;
_17 = [Field::<Adt46>(Variant(_8, 1), 3).fld1,Field::<Adt46>(Variant(_8, 1), 3).fld1,Field::<Adt46>(Variant(_8, 1), 3).fld1,Field::<Adt46>(Variant(_8, 1), 3).fld1];
place!(Field::<u64>(Variant(_8, 1), 6)) = !_11;
_19 = core::ptr::addr_of_mut!(_10);
_16 = [_5];
place!(Field::<[i32; 3]>(Variant(_14, 1), 0)) = [_4,_4,_4];
place!(Field::<(u16, i32, i16, f64)>(Variant(_8, 1), 7)).3 = _5 as f64;
_7 = _10;
SetDiscriminant(_14, 1);
place!(Field::<i32>(Variant(_8, 1), 5)) = _4 | _4;
(*_19) = _7;
_12 = 68016182903654461686430875135254126754_i128 as i64;
_18 = _15.1 ^ _15.1;
Goto(bb4)
}
bb4 = {
_5 = (-88_i8) as isize;
_8 = Adt54::Variant0 { fld0: _11,fld1: _1 };
_5 = !9223372036854775807_isize;
_20 = Adt46 { fld0: 1_usize,fld1: 11185025745202143609799656985282349246_u128 };
_1.0 = _9;
place!(Field::<[i32; 3]>(Variant(_14, 1), 0)) = [_4,_4,_4];
_18 = !_15.1;
place!(Field::<([i32; 3], [i128; 8])>(Variant(_8, 0), 1)).0 = _9;
_2 = _6;
RET = 6063_i16;
SetDiscriminant(_14, 0);
Call(_20.fld0 = core::intrinsics::bswap(4_usize), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_20 = Adt46 { fld0: 8634507877382778110_usize,fld1: 237817827264725648191309060792547172710_u128 };
place!(Field::<f32>(Variant(_14, 0), 1)) = _4 as f32;
place!(Field::<f32>(Variant(_14, 0), 1)) = _4 as f32;
(*_19) = _7;
place!(Field::<([i32; 3], [i128; 8])>(Variant(_8, 0), 1)).1 = [(-58754596437998726983421751506866710254_i128),(-101948446061446832849942801752746705768_i128),(-59120248028440295138060045360490213133_i128),169217279353736828744743910575290292625_i128,(-150279203943483942336819169580307661_i128),(-116599657263071043437449157443853986115_i128),(-152446251129371937147161535089168078467_i128),15976363626184327688038943777592399558_i128];
_20.fld1 = 280699156141907852848424965200829926052_u128;
place!(Field::<isize>(Variant(_14, 0), 2)) = _4 as isize;
_1.0 = [_4,_4,_4];
_10 = core::ptr::addr_of!(RET);
_5 = Field::<isize>(Variant(_14, 0), 2);
_1 = (Field::<([i32; 3], [i128; 8])>(Variant(_8, 0), 1).0, Field::<([i32; 3], [i128; 8])>(Variant(_8, 0), 1).1);
_18 = !_15.1;
_22 = _15.2 == _15.2;
place!(Field::<([i32; 3], [i128; 8])>(Variant(_8, 0), 1)) = (_9, _2);
_20.fld0 = !5158452563355854533_usize;
_11 = Field::<u64>(Variant(_8, 0), 0) | Field::<u64>(Variant(_8, 0), 0);
_11 = Field::<u64>(Variant(_8, 0), 0);
_14 = Adt39::Variant1 { fld0: _9 };
Goto(bb6)
}
bb6 = {
place!(Field::<([i32; 3], [i128; 8])>(Variant(_8, 0), 1)) = (_9, _2);
_7 = (*_19);
_1.0 = Field::<([i32; 3], [i128; 8])>(Variant(_8, 0), 1).0;
_15.1 = _5 as u8;
_15.2 = _22 & _22;
SetDiscriminant(_14, 2);
(*_7) = 2100_i16 ^ (-26446_i16);
match _4 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb7,
5 => bb8,
6 => bb9,
80699738 => bb11,
_ => bb10
}
}
bb7 = {
_20 = Adt46 { fld0: 8634507877382778110_usize,fld1: 237817827264725648191309060792547172710_u128 };
place!(Field::<f32>(Variant(_14, 0), 1)) = _4 as f32;
place!(Field::<f32>(Variant(_14, 0), 1)) = _4 as f32;
(*_19) = _7;
place!(Field::<([i32; 3], [i128; 8])>(Variant(_8, 0), 1)).1 = [(-58754596437998726983421751506866710254_i128),(-101948446061446832849942801752746705768_i128),(-59120248028440295138060045360490213133_i128),169217279353736828744743910575290292625_i128,(-150279203943483942336819169580307661_i128),(-116599657263071043437449157443853986115_i128),(-152446251129371937147161535089168078467_i128),15976363626184327688038943777592399558_i128];
_20.fld1 = 280699156141907852848424965200829926052_u128;
place!(Field::<isize>(Variant(_14, 0), 2)) = _4 as isize;
_1.0 = [_4,_4,_4];
_10 = core::ptr::addr_of!(RET);
_5 = Field::<isize>(Variant(_14, 0), 2);
_1 = (Field::<([i32; 3], [i128; 8])>(Variant(_8, 0), 1).0, Field::<([i32; 3], [i128; 8])>(Variant(_8, 0), 1).1);
_18 = !_15.1;
_22 = _15.2 == _15.2;
place!(Field::<([i32; 3], [i128; 8])>(Variant(_8, 0), 1)) = (_9, _2);
_20.fld0 = !5158452563355854533_usize;
_11 = Field::<u64>(Variant(_8, 0), 0) | Field::<u64>(Variant(_8, 0), 0);
_11 = Field::<u64>(Variant(_8, 0), 0);
_14 = Adt39::Variant1 { fld0: _9 };
Goto(bb6)
}
bb8 = {
_5 = (-88_i8) as isize;
_8 = Adt54::Variant0 { fld0: _11,fld1: _1 };
_5 = !9223372036854775807_isize;
_20 = Adt46 { fld0: 1_usize,fld1: 11185025745202143609799656985282349246_u128 };
_1.0 = _9;
place!(Field::<[i32; 3]>(Variant(_14, 1), 0)) = [_4,_4,_4];
_18 = !_15.1;
place!(Field::<([i32; 3], [i128; 8])>(Variant(_8, 0), 1)).0 = _9;
_2 = _6;
RET = 6063_i16;
SetDiscriminant(_14, 0);
Call(_20.fld0 = core::intrinsics::bswap(4_usize), ReturnTo(bb5), UnwindUnreachable())
}
bb9 = {
_6 = _2;
_15 = ((-2439178920752108913_i64), 39_u8, false);
_2 = [(-71666980436510184430838053538453893111_i128),(-115896437707964008359503010827782524431_i128),120411713460952306048066004473370958960_i128,(-5529617640683832454385620304185800965_i128),(-139101239233589624018346161891945066530_i128),(-120109483299250398497155910493272839248_i128),(-76873975071185427316290945273825064963_i128),(-15317790670095813443439433661257081889_i128)];
_16 = [_5];
_14 = Adt39::Variant1 { fld0: _9 };
SetDiscriminant(_8, 1);
place!(Field::<(u16, i32, i16, f64)>(Variant(_8, 1), 7)).3 = _4 as f64;
place!(Field::<Adt46>(Variant(_8, 1), 3)).fld1 = 314815275191127490605336629863798397723_u128;
_17 = [Field::<Adt46>(Variant(_8, 1), 3).fld1,Field::<Adt46>(Variant(_8, 1), 3).fld1,Field::<Adt46>(Variant(_8, 1), 3).fld1,Field::<Adt46>(Variant(_8, 1), 3).fld1];
place!(Field::<u64>(Variant(_8, 1), 6)) = !_11;
_19 = core::ptr::addr_of_mut!(_10);
_16 = [_5];
place!(Field::<[i32; 3]>(Variant(_14, 1), 0)) = [_4,_4,_4];
place!(Field::<(u16, i32, i16, f64)>(Variant(_8, 1), 7)).3 = _5 as f64;
_7 = _10;
SetDiscriminant(_14, 1);
place!(Field::<i32>(Variant(_8, 1), 5)) = _4 | _4;
(*_19) = _7;
_12 = 68016182903654461686430875135254126754_i128 as i64;
_18 = _15.1 ^ _15.1;
Goto(bb4)
}
bb10 = {
place!(Field::<u64>(Variant(_8, 0), 0)) = !_11;
_15.1 = !63_u8;
RET = 1413_i16;
_15.1 = !79_u8;
_15.1 = true as u8;
match _4 {
80699738 => bb3,
_ => bb1
}
}
bb11 = {
(*_19) = core::ptr::addr_of!(RET);
RET = _15.0 as i16;
Goto(bb12)
}
bb12 = {
_22 = !_15.2;
_7 = core::ptr::addr_of!((*_7));
_17 = [_20.fld1,_20.fld1,_20.fld1,_20.fld1];
_13 = [_18,_18];
_15 = (_12, _18, _22);
match _4 {
0 => bb13,
1 => bb14,
2 => bb15,
80699738 => bb17,
_ => bb16
}
}
bb13 = {
place!(Field::<u64>(Variant(_8, 0), 0)) = !_11;
_15.1 = !63_u8;
RET = 1413_i16;
_15.1 = !79_u8;
_15.1 = true as u8;
match _4 {
80699738 => bb3,
_ => bb1
}
}
bb14 = {
place!(Field::<u64>(Variant(_8, 0), 0)) = !_11;
_15.1 = !63_u8;
RET = 1413_i16;
_15.1 = !79_u8;
_15.1 = true as u8;
match _4 {
80699738 => bb3,
_ => bb1
}
}
bb15 = {
_6 = _2;
_15 = ((-2439178920752108913_i64), 39_u8, false);
_2 = [(-71666980436510184430838053538453893111_i128),(-115896437707964008359503010827782524431_i128),120411713460952306048066004473370958960_i128,(-5529617640683832454385620304185800965_i128),(-139101239233589624018346161891945066530_i128),(-120109483299250398497155910493272839248_i128),(-76873975071185427316290945273825064963_i128),(-15317790670095813443439433661257081889_i128)];
_16 = [_5];
_14 = Adt39::Variant1 { fld0: _9 };
SetDiscriminant(_8, 1);
place!(Field::<(u16, i32, i16, f64)>(Variant(_8, 1), 7)).3 = _4 as f64;
place!(Field::<Adt46>(Variant(_8, 1), 3)).fld1 = 314815275191127490605336629863798397723_u128;
_17 = [Field::<Adt46>(Variant(_8, 1), 3).fld1,Field::<Adt46>(Variant(_8, 1), 3).fld1,Field::<Adt46>(Variant(_8, 1), 3).fld1,Field::<Adt46>(Variant(_8, 1), 3).fld1];
place!(Field::<u64>(Variant(_8, 1), 6)) = !_11;
_19 = core::ptr::addr_of_mut!(_10);
_16 = [_5];
place!(Field::<[i32; 3]>(Variant(_14, 1), 0)) = [_4,_4,_4];
place!(Field::<(u16, i32, i16, f64)>(Variant(_8, 1), 7)).3 = _5 as f64;
_7 = _10;
SetDiscriminant(_14, 1);
place!(Field::<i32>(Variant(_8, 1), 5)) = _4 | _4;
(*_19) = _7;
_12 = 68016182903654461686430875135254126754_i128 as i64;
_18 = _15.1 ^ _15.1;
Goto(bb4)
}
bb16 = {
_6 = _2;
_15 = ((-2439178920752108913_i64), 39_u8, false);
_2 = [(-71666980436510184430838053538453893111_i128),(-115896437707964008359503010827782524431_i128),120411713460952306048066004473370958960_i128,(-5529617640683832454385620304185800965_i128),(-139101239233589624018346161891945066530_i128),(-120109483299250398497155910493272839248_i128),(-76873975071185427316290945273825064963_i128),(-15317790670095813443439433661257081889_i128)];
_16 = [_5];
_14 = Adt39::Variant1 { fld0: _9 };
SetDiscriminant(_8, 1);
place!(Field::<(u16, i32, i16, f64)>(Variant(_8, 1), 7)).3 = _4 as f64;
place!(Field::<Adt46>(Variant(_8, 1), 3)).fld1 = 314815275191127490605336629863798397723_u128;
_17 = [Field::<Adt46>(Variant(_8, 1), 3).fld1,Field::<Adt46>(Variant(_8, 1), 3).fld1,Field::<Adt46>(Variant(_8, 1), 3).fld1,Field::<Adt46>(Variant(_8, 1), 3).fld1];
place!(Field::<u64>(Variant(_8, 1), 6)) = !_11;
_19 = core::ptr::addr_of_mut!(_10);
_16 = [_5];
place!(Field::<[i32; 3]>(Variant(_14, 1), 0)) = [_4,_4,_4];
place!(Field::<(u16, i32, i16, f64)>(Variant(_8, 1), 7)).3 = _5 as f64;
_7 = _10;
SetDiscriminant(_14, 1);
place!(Field::<i32>(Variant(_8, 1), 5)) = _4 | _4;
(*_19) = _7;
_12 = 68016182903654461686430875135254126754_i128 as i64;
_18 = _15.1 ^ _15.1;
Goto(bb4)
}
bb17 = {
_25 = Field::<([i32; 3], [i128; 8])>(Variant(_8, 0), 1);
place!(Field::<u64>(Variant(_8, 0), 0)) = _22 as u64;
_19 = core::ptr::addr_of_mut!(_10);
_19 = core::ptr::addr_of_mut!((*_19));
place!(Field::<u32>(Variant(_14, 2), 2)) = _4 as u32;
Goto(bb18)
}
bb18 = {
Call(_31 = dump_var(17_usize, 9_usize, Move(_9), 1_usize, Move(_1), 12_usize, Move(_12), 15_usize, Move(_15)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_31 = dump_var(17_usize, 22_usize, Move(_22), 3_usize, Move(_3), 13_usize, Move(_13), 5_usize, Move(_5)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: i32,mut _2: isize) -> f64 {
mir! {
type RET = f64;
let _3: (*const i8,);
let _4: isize;
let _5: f32;
let _6: i16;
let _7: *const i8;
let _8: Adt48;
let _9: i128;
let _10: u8;
let _11: ([i32; 3], [i128; 8]);
let _12: isize;
let _13: [u128; 4];
let _14: u16;
let _15: ([i32; 3], f32, (u16, i32, i16, f64), u128);
let _16: bool;
let _17: Adt45;
let _18: ();
let _19: ();
{
RET = 28075_u16 as f64;
RET = 134947197474292394956898142946200498427_u128 as f64;
RET = 225244272942235628071218777111699073550_u128 as f64;
_1 = (-596668604_i32);
_2 = 9223372036854775807_isize * 9223372036854775807_isize;
_1 = (-839051484_i32) << _2;
_2 = (-9223372036854775808_isize) - 26_isize;
RET = 189668197267765294936195958957248037899_u128 as f64;
RET = (-6709473030647446301_i64) as f64;
RET = (-64186771986583123339180153173239543935_i128) as f64;
_2 = 56217968216508926453798120549693248701_u128 as isize;
RET = (-100_i8) as f64;
_4 = _2 >> _1;
RET = 232_u8 as f64;
_5 = 31455468599409224831269465421304087566_i128 as f32;
_4 = _2 >> _2;
RET = _1 as f64;
RET = _5 as f64;
_1 = (-1075705250_i32) & 267029341_i32;
RET = 7431336927342556982_i64 as f64;
_5 = (-10153_i16) as f32;
Goto(bb1)
}
bb1 = {
_2 = _4 ^ _4;
_1 = '\u{8e704}' as i32;
_1 = (-59162393_i32);
_1 = 1627210724_i32;
_6 = (-2694_i16);
_4 = _2;
_2 = _4;
_4 = _2 | _2;
_6 = _5 as i16;
_4 = _6 as isize;
_2 = _4;
_5 = RET as f32;
_6 = !(-22587_i16);
RET = 33676_u16 as f64;
_4 = _2;
Goto(bb2)
}
bb2 = {
_6 = (-26672_i16);
_5 = (-34833271010277450145339399198751923119_i128) as f32;
RET = 0_usize as f64;
RET = 154947519688059524316836226654168469101_i128 as f64;
_6 = -(-29217_i16);
_6 = 21_i8 as i16;
RET = _4 as f64;
_4 = !_2;
RET = 581717978505762970047620744871970174_i128 as f64;
RET = 166519294634436574746096520494998556262_u128 as f64;
RET = 174_u8 as f64;
_9 = -50955599699055039273207905574722551735_i128;
RET = _6 as f64;
_6 = (-12528_i16) + (-31628_i16);
_9 = 6872_u16 as i128;
RET = 7886299662269197947_i64 as f64;
RET = _2 as f64;
_1 = (-1594068334_i32) ^ 1597633066_i32;
_10 = !88_u8;
RET = (-24_i8) as f64;
RET = 3626143586746090045_u64 as f64;
_2 = _4 * _4;
_6 = -22400_i16;
Goto(bb3)
}
bb3 = {
_9 = (-155815082100603061024047955047067019587_i128) * 160093990725770335055729909863887736547_i128;
_10 = 225_u8;
_9 = !(-68399120638489159779469039452520498299_i128);
RET = _2 as f64;
RET = _1 as f64;
_10 = 17284086429594897789_u64 as u8;
_10 = 117_u8 + 116_u8;
_9 = 15318879690099228305_u64 as i128;
_5 = 318612896758800232206651326490728594360_u128 as f32;
_9 = _1 as i128;
_11.0 = [_1,_1,_1];
_9 = !(-26414531448897406718626698987709257955_i128);
RET = _5 as f64;
RET = _2 as f64;
_10 = 4_u8 * 39_u8;
RET = _4 as f64;
_11.1 = [_9,_9,_9,_9,_9,_9,_9,_9];
_4 = !_2;
Goto(bb4)
}
bb4 = {
_6 = -(-30744_i16);
_1 = (-1490241553_i32);
_5 = 157824368188083827385895606912348792249_u128 as f32;
Call(_4 = core::intrinsics::transmute(_2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_6 = -(-5981_i16);
_6 = -11947_i16;
_6 = (-25170_i16);
_5 = 6550732940621503563_i64 as f32;
_2 = -_4;
_9 = '\u{74445}' as i128;
_11.0 = [_1,_1,_1];
RET = _6 as f64;
RET = (-6632138139302470300_i64) as f64;
_11.1 = [_9,_9,_9,_9,_9,_9,_9,_9];
_9 = _2 as i128;
_11.1 = [_9,_9,_9,_9,_9,_9,_9,_9];
_1 = 10300286912904063865_u64 as i32;
Call(_12 = core::intrinsics::transmute(_2), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_9 = _1 as i128;
_6 = 14546_i16;
_12 = _4;
_4 = _12 & _2;
RET = _6 as f64;
_6 = 8150_i16;
_13 = [211963702412620873271930808264438542574_u128,218332889498296677150543908754901750466_u128,230509290189950680586357652950913831206_u128,263452342066825916174653947264851879310_u128];
_4 = 2526888755_u32 as isize;
_11.0 = [_1,_1,_1];
match _6 {
0 => bb1,
1 => bb5,
2 => bb7,
3 => bb8,
4 => bb9,
8150 => bb11,
_ => bb10
}
}
bb7 = {
_6 = -(-5981_i16);
_6 = -11947_i16;
_6 = (-25170_i16);
_5 = 6550732940621503563_i64 as f32;
_2 = -_4;
_9 = '\u{74445}' as i128;
_11.0 = [_1,_1,_1];
RET = _6 as f64;
RET = (-6632138139302470300_i64) as f64;
_11.1 = [_9,_9,_9,_9,_9,_9,_9,_9];
_9 = _2 as i128;
_11.1 = [_9,_9,_9,_9,_9,_9,_9,_9];
_1 = 10300286912904063865_u64 as i32;
Call(_12 = core::intrinsics::transmute(_2), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_6 = -(-30744_i16);
_1 = (-1490241553_i32);
_5 = 157824368188083827385895606912348792249_u128 as f32;
Call(_4 = core::intrinsics::transmute(_2), ReturnTo(bb5), UnwindUnreachable())
}
bb9 = {
_9 = (-155815082100603061024047955047067019587_i128) * 160093990725770335055729909863887736547_i128;
_10 = 225_u8;
_9 = !(-68399120638489159779469039452520498299_i128);
RET = _2 as f64;
RET = _1 as f64;
_10 = 17284086429594897789_u64 as u8;
_10 = 117_u8 + 116_u8;
_9 = 15318879690099228305_u64 as i128;
_5 = 318612896758800232206651326490728594360_u128 as f32;
_9 = _1 as i128;
_11.0 = [_1,_1,_1];
_9 = !(-26414531448897406718626698987709257955_i128);
RET = _5 as f64;
RET = _2 as f64;
_10 = 4_u8 * 39_u8;
RET = _4 as f64;
_11.1 = [_9,_9,_9,_9,_9,_9,_9,_9];
_4 = !_2;
Goto(bb4)
}
bb10 = {
_2 = _4 ^ _4;
_1 = '\u{8e704}' as i32;
_1 = (-59162393_i32);
_1 = 1627210724_i32;
_6 = (-2694_i16);
_4 = _2;
_2 = _4;
_4 = _2 | _2;
_6 = _5 as i16;
_4 = _6 as isize;
_2 = _4;
_5 = RET as f32;
_6 = !(-22587_i16);
RET = 33676_u16 as f64;
_4 = _2;
Goto(bb2)
}
bb11 = {
_4 = _12 ^ _2;
_1 = 3607201101_u32 as i32;
_12 = !_4;
RET = _1 as f64;
_13 = [96464246372682830171548432412949754042_u128,108669513744058571255012070164576787242_u128,202842392916296445985150605620453677049_u128,93124464148108791114483842129441415474_u128];
RET = 4292358327_u32 as f64;
_11.1 = [_9,_9,_9,_9,_9,_9,_9,_9];
_11.1 = [_9,_9,_9,_9,_9,_9,_9,_9];
Goto(bb12)
}
bb12 = {
_2 = _12;
_9 = (-16246157961144254480019776234349259846_i128) >> _4;
_10 = 58_u8 - 14_u8;
_10 = 0_usize as u8;
_11.0 = [_1,_1,_1];
_14 = 9078342159365036320_i64 as u16;
_11.1 = [_9,_9,_9,_9,_9,_9,_9,_9];
_11.1 = [_9,_9,_9,_9,_9,_9,_9,_9];
_9 = -(-75468881791461625131638933450419129838_i128);
_13 = [90850391035108343497805559343341719102_u128,236965111979850384264874318544371571572_u128,159480335383419754214828163578208819117_u128,102193257096812546151243247257229241759_u128];
_4 = _12;
RET = 4113326939_u32 as f64;
_13 = [28703987610605999463268742460327843582_u128,313161956939336745896881635071889646004_u128,194398345187646805363286635126389543542_u128,243756309641869824263960777857085284029_u128];
_12 = _4;
_9 = !(-163389533779207051025212748689655242352_i128);
RET = 331741778654993153505692602597136433897_u128 as f64;
_14 = 52268_u16;
Goto(bb13)
}
bb13 = {
_11.0 = [_1,_1,_1];
_11.0 = [_1,_1,_1];
_5 = _14 as f32;
_13 = [6601076998334079565501919538302841965_u128,223909041166490714277198583005785499034_u128,261790150924536665730047472622088357189_u128,324067766324789826077053925750961490868_u128];
_12 = _1 as isize;
RET = _14 as f64;
_12 = _4;
_2 = -_12;
_12 = '\u{32b9}' as isize;
_9 = (-30567488005416073917136202232009112154_i128) | 130746110690841706633017413001453829571_i128;
_2 = _14 as isize;
_13 = [51874614483686288198441437509849954758_u128,258420854604139752499888226377437233247_u128,333286987976681440028046272446148766134_u128,107393140911939235382036016472015713887_u128];
_10 = 210_u8 & 66_u8;
_9 = _10 as i128;
_14 = 31780_u16 << _12;
RET = _12 as f64;
_10 = 208_u8;
_10 = 40_u8;
_15.1 = _5 + _5;
_15.0 = [_1,_1,_1];
match _6 {
0 => bb10,
1 => bb12,
2 => bb11,
3 => bb4,
4 => bb6,
8150 => bb15,
_ => bb14
}
}
bb14 = {
_9 = (-155815082100603061024047955047067019587_i128) * 160093990725770335055729909863887736547_i128;
_10 = 225_u8;
_9 = !(-68399120638489159779469039452520498299_i128);
RET = _2 as f64;
RET = _1 as f64;
_10 = 17284086429594897789_u64 as u8;
_10 = 117_u8 + 116_u8;
_9 = 15318879690099228305_u64 as i128;
_5 = 318612896758800232206651326490728594360_u128 as f32;
_9 = _1 as i128;
_11.0 = [_1,_1,_1];
_9 = !(-26414531448897406718626698987709257955_i128);
RET = _5 as f64;
RET = _2 as f64;
_10 = 4_u8 * 39_u8;
RET = _4 as f64;
_11.1 = [_9,_9,_9,_9,_9,_9,_9,_9];
_4 = !_2;
Goto(bb4)
}
bb15 = {
_15.2.1 = -_1;
_15.2 = (_14, _1, _6, RET);
_16 = !false;
_1 = -_15.2.1;
RET = -_15.2.3;
_14 = _15.2.0 | _15.2.0;
_13 = [180653357823290918314994311520758257212_u128,64050553980386433745713679565064493632_u128,143505095120315782852707185793385829502_u128,16057792154739787241621981885835331106_u128];
_15.2.1 = _4 as i32;
_9 = 48513245542348910002050353595851640474_i128;
Goto(bb16)
}
bb16 = {
Call(_18 = dump_var(18_usize, 2_usize, Move(_2), 16_usize, Move(_16), 9_usize, Move(_9), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_18 = dump_var(18_usize, 6_usize, Move(_6), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: i16,mut _2: isize,mut _3: i16,mut _4: [i32; 3],mut _5: [i32; 3]) -> Adt40 {
mir! {
type RET = Adt40;
let _6: ([i32; 3], [i128; 8]);
let _7: char;
let _8: [u8; 2];
let _9: &'static f32;
let _10: (i64, u8, bool);
let _11: i128;
let _12: bool;
let _13: &'static f32;
let _14: *const i16;
let _15: i16;
let _16: f64;
let _17: Adt55;
let _18: ([i32; 3], f32, (u16, i32, i16, f64), u128);
let _19: isize;
let _20: i8;
let _21: (*const i16,);
let _22: isize;
let _23: isize;
let _24: [u8; 4];
let _25: Adt42;
let _26: [i128; 8];
let _27: char;
let _28: Adt46;
let _29: [i32; 3];
let _30: [i32; 3];
let _31: Adt55;
let _32: ();
let _33: ();
{
_3 = _1 - _1;
_3 = 3538510436600090762_i64 as i16;
_4 = _5;
_2 = !(-9223372036854775808_isize);
_6.1 = [99214654347519649626385884135478290543_i128,19460148418802895155389292719319661888_i128,45468191203811031486507571518032617702_i128,31705619900357588261507772973020067751_i128,(-6369564166067732319885763529528772475_i128),75985131953623636328014940790017990131_i128,35969706790088361208164771600841148434_i128,124031215317404554379753203017001700097_i128];
_4 = [(-1148663015_i32),1626828257_i32,(-1239211335_i32)];
_4 = _5;
_5 = [(-395335714_i32),1899777455_i32,104641671_i32];
_6.0 = [1241875610_i32,1839297897_i32,(-1688033637_i32)];
_3 = _1 - _1;
_7 = '\u{ab97}';
_6.0 = [143612367_i32,1551344010_i32,(-60285408_i32)];
Goto(bb1)
}
bb1 = {
_2 = 5498682659578134773115520995354612473_u128 as isize;
_8 = [103_u8,174_u8];
_7 = '\u{7a0c5}';
_2 = (-32_isize);
_1 = -_3;
_1 = _3 * _3;
_1 = _3 + _3;
_5 = _4;
_2 = -(-9223372036854775808_isize);
_8 = [144_u8,167_u8];
_6.0 = [(-27424737_i32),(-2084622543_i32),2125266030_i32];
_3 = 113_i8 as i16;
_8 = [124_u8,140_u8];
_1 = _3;
_3 = !_1;
_6.1 = [91315334273950129769823826468568869166_i128,66959651741916111011685613770167304482_i128,(-154391261769877610069403440523718923825_i128),53298786553577649969073563630541713758_i128,(-9365472311446126272513571802041655658_i128),(-124784383031319749219242469082145978853_i128),(-41019732539500023452950598656581237493_i128),(-78685949040222980788970100120726280677_i128)];
_8 = [224_u8,130_u8];
_4 = [1242073356_i32,(-1340104872_i32),1296469212_i32];
_6.1 = [65471789306517193536683017557845741699_i128,133899667084029278408949618494652817037_i128,(-83530097904343611029448890522198221571_i128),19421667389552976444500704449670139763_i128,(-92948713311736144139140808307675890049_i128),(-131734632615403769685496488034965659237_i128),(-89938884592273739080465605068935183598_i128),29639944488670791307882699093981605338_i128];
Goto(bb2)
}
bb2 = {
_7 = '\u{f3d25}';
_10.2 = false & false;
_10.1 = !202_u8;
_10.0 = 5993704562763869082_i64;
_4 = [(-367953993_i32),443115064_i32,(-1781375193_i32)];
_6.0 = [(-1386348340_i32),(-1795431389_i32),1127469416_i32];
_10.0 = !8399198918763117925_i64;
_7 = '\u{e1d2e}';
_7 = '\u{f5350}';
_10.0 = 3384_u16 as i64;
_1 = -_3;
_5 = _6.0;
_1 = _3 & _3;
_6.0 = [829721702_i32,480935825_i32,(-2086657662_i32)];
_4 = [(-1795707932_i32),(-816952053_i32),(-1225834672_i32)];
_7 = '\u{c544e}';
_5 = [1709939895_i32,496608211_i32,1939551077_i32];
_12 = _10.0 < _10.0;
_3 = (-165666505995480197800852358270900543952_i128) as i16;
_2 = 9223372036854775807_isize;
_10.2 = _2 >= _2;
_10.0 = (-4633283233390530623_i64);
_11 = 1994442628_i32 as i128;
_4 = _5;
_10 = (1549263871310293478_i64, 117_u8, _12);
match _10.1 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
117 => bb9,
_ => bb8
}
}
bb3 = {
_2 = 5498682659578134773115520995354612473_u128 as isize;
_8 = [103_u8,174_u8];
_7 = '\u{7a0c5}';
_2 = (-32_isize);
_1 = -_3;
_1 = _3 * _3;
_1 = _3 + _3;
_5 = _4;
_2 = -(-9223372036854775808_isize);
_8 = [144_u8,167_u8];
_6.0 = [(-27424737_i32),(-2084622543_i32),2125266030_i32];
_3 = 113_i8 as i16;
_8 = [124_u8,140_u8];
_1 = _3;
_3 = !_1;
_6.1 = [91315334273950129769823826468568869166_i128,66959651741916111011685613770167304482_i128,(-154391261769877610069403440523718923825_i128),53298786553577649969073563630541713758_i128,(-9365472311446126272513571802041655658_i128),(-124784383031319749219242469082145978853_i128),(-41019732539500023452950598656581237493_i128),(-78685949040222980788970100120726280677_i128)];
_8 = [224_u8,130_u8];
_4 = [1242073356_i32,(-1340104872_i32),1296469212_i32];
_6.1 = [65471789306517193536683017557845741699_i128,133899667084029278408949618494652817037_i128,(-83530097904343611029448890522198221571_i128),19421667389552976444500704449670139763_i128,(-92948713311736144139140808307675890049_i128),(-131734632615403769685496488034965659237_i128),(-89938884592273739080465605068935183598_i128),29639944488670791307882699093981605338_i128];
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
_10.1 = 190_u8;
_10.1 = 16_u8 ^ 164_u8;
_10 = ((-3325207506453626716_i64), 14_u8, _12);
_7 = '\u{aa58e}';
_4 = [(-1047521079_i32),696544695_i32,1288450380_i32];
_7 = '\u{3a1c1}';
_15 = 1_usize as i16;
_11 = 17704472286784353092261025899763728422_i128 * 46020553068088154349086400332073211159_i128;
_12 = _3 != _3;
_14 = core::ptr::addr_of!(_3);
_2 = !9223372036854775807_isize;
(*_14) = _1 & _1;
_10 = (3161620989357863181_i64, 128_u8, _12);
_15 = 3569724647_u32 as i16;
_5 = _4;
_8 = [_10.1,_10.1];
_4 = _6.0;
_3 = _1;
_10.2 = _12 != _12;
_15 = (*_14);
_6.0 = [1837772754_i32,(-426615229_i32),(-593413662_i32)];
_3 = 2009167123_u32 as i16;
_6.0 = [35077464_i32,(-507468266_i32),(-2033196386_i32)];
_16 = 10298182335617553639_u64 as f64;
Goto(bb10)
}
bb10 = {
_15 = (*_14);
_7 = '\u{56c6f}';
_5 = _6.0;
_15 = _3 * (*_14);
_15 = (*_14);
_3 = _1;
_4 = [(-830629966_i32),(-210941821_i32),664616756_i32];
_10 = (2737893188097614970_i64, 4_u8, _12);
_18.1 = 308807863101781065867516927697868147120_u128 as f32;
_4 = _6.0;
_18.2.2 = !_1;
_8 = [_10.1,_10.1];
_1 = 9708559950353795211_u64 as i16;
_15 = (*_14);
_13 = &_18.1;
_6.1 = [_11,_11,_11,_11,_11,_11,_11,_11];
_14 = core::ptr::addr_of!(_15);
_9 = Move(_13);
_14 = core::ptr::addr_of!(_3);
_18.3 = 122983689304140208445954922859400125147_u128 | 193587835206770717801553433815585744375_u128;
_19 = _2;
_5 = _6.0;
_5 = _4;
_3 = _10.1 as i16;
_20 = !(-40_i8);
_2 = _19 + _19;
Goto(bb11)
}
bb11 = {
_13 = &(*_9);
_10.0 = 7097703583313647701_i64;
_18.2.3 = _16 * _16;
_18.2 = (65220_u16, (-1860135672_i32), _1, _16);
Goto(bb12)
}
bb12 = {
_20 = 2_i8;
_18.2.1 = (-1899352995_i32) >> (*_14);
_21.0 = core::ptr::addr_of!(_3);
_9 = Move(_13);
_6.0 = _5;
_7 = '\u{1f859}';
_14 = _21.0;
_10 = (7736270042880409429_i64, 161_u8, _12);
_4 = [_18.2.1,_18.2.1,_18.2.1];
_19 = _2 ^ _2;
_18.2 = (9648_u16, (-343287609_i32), (*_14), _16);
_7 = '\u{10f3c8}';
_6.0 = [_18.2.1,_18.2.1,_18.2.1];
_6.1 = [_11,_11,_11,_11,_11,_11,_11,_11];
_9 = &_18.1;
_18.1 = _18.2.3 as f32;
_10.2 = _12;
_1 = _18.2.2 + (*_14);
_18.2.2 = _1 - (*_14);
_12 = _10.2;
_6.1 = [_11,_11,_11,_11,_11,_11,_11,_11];
Goto(bb13)
}
bb13 = {
_18.3 = !247688709705167960675026693070774083460_u128;
_17 = Adt55::Variant1 { fld0: _10 };
_18.2 = (15938_u16, 1819854801_i32, _1, _16);
_24 = [_10.1,Field::<(i64, u8, bool)>(Variant(_17, 1), 0).1,_10.1,_10.1];
_26 = _6.1;
_10.1 = !Field::<(i64, u8, bool)>(Variant(_17, 1), 0).1;
_4 = [_18.2.1,_18.2.1,_18.2.1];
_21.0 = _14;
_10.2 = !_12;
SetDiscriminant(_17, 2);
_10 = (3407539562672022149_i64, 117_u8, _12);
_28.fld0 = 7_usize & 6198089755580807943_usize;
match _10.0 {
0 => bb12,
1 => bb5,
2 => bb14,
3407539562672022149 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
_2 = 5498682659578134773115520995354612473_u128 as isize;
_8 = [103_u8,174_u8];
_7 = '\u{7a0c5}';
_2 = (-32_isize);
_1 = -_3;
_1 = _3 * _3;
_1 = _3 + _3;
_5 = _4;
_2 = -(-9223372036854775808_isize);
_8 = [144_u8,167_u8];
_6.0 = [(-27424737_i32),(-2084622543_i32),2125266030_i32];
_3 = 113_i8 as i16;
_8 = [124_u8,140_u8];
_1 = _3;
_3 = !_1;
_6.1 = [91315334273950129769823826468568869166_i128,66959651741916111011685613770167304482_i128,(-154391261769877610069403440523718923825_i128),53298786553577649969073563630541713758_i128,(-9365472311446126272513571802041655658_i128),(-124784383031319749219242469082145978853_i128),(-41019732539500023452950598656581237493_i128),(-78685949040222980788970100120726280677_i128)];
_8 = [224_u8,130_u8];
_4 = [1242073356_i32,(-1340104872_i32),1296469212_i32];
_6.1 = [65471789306517193536683017557845741699_i128,133899667084029278408949618494652817037_i128,(-83530097904343611029448890522198221571_i128),19421667389552976444500704449670139763_i128,(-92948713311736144139140808307675890049_i128),(-131734632615403769685496488034965659237_i128),(-89938884592273739080465605068935183598_i128),29639944488670791307882699093981605338_i128];
Goto(bb2)
}
bb16 = {
_11 = -65004983431511236455127608644583262951_i128;
_16 = _18.2.3 + _18.2.3;
_6.1 = _26;
_6 = (_4, _26);
(*_14) = _10.2 as i16;
_18.2.2 = _15 | _1;
_10.0 = (-4094565558217593813_i64);
place!(Field::<*const [i32; 3]>(Variant(_17, 2), 0)) = core::ptr::addr_of!(_5);
RET = Adt40::Variant0 { fld0: _21 };
_13 = &_18.1;
_8 = [_10.1,_10.1];
_10.2 = !_12;
_29 = [_18.2.1,_18.2.1,_18.2.1];
place!(Field::<(*const i16,)>(Variant(RET, 0), 0)).0 = core::ptr::addr_of!(_18.2.2);
_19 = !_2;
_10.1 = 90_u8;
_28.fld1 = _18.3;
_6 = (_4, _26);
place!(Field::<(*const i16,)>(Variant(RET, 0), 0)) = _21;
_6.0 = _4;
Goto(bb17)
}
bb17 = {
Call(_32 = dump_var(19_usize, 6_usize, Move(_6), 7_usize, Move(_7), 3_usize, Move(_3), 29_usize, Move(_29)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_32 = dump_var(19_usize, 10_usize, Move(_10), 20_usize, Move(_20), 2_usize, Move(_2), 26_usize, Move(_26)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{71e22}'), std::hint::black_box(5509164495176456686_usize), std::hint::black_box(81_i8), std::hint::black_box((-2004_i16)), std::hint::black_box(6615621877460429093_u64), std::hint::black_box(49307_u16), std::hint::black_box(78720668123829900935746088001184262555_i128));
                
            }
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt39::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: *const isize,
fld1: f32,
fld2: isize,
fld3: [isize; 1],

},
Variant1{
fld0: [i32; 3],

},
Variant2{
fld0: [i128; 8],
fld1: *const [i32; 3],
fld2: u32,
fld3: *const isize,

},
Variant3{
fld0: (*const i8,),
fld1: *mut char,
fld2: *const i8,
fld3: [u128; 4],

}}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt40::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: (*const i16,),

},
Variant1{
fld0: ([i32; 3], [i128; 8]),
fld1: (i64, u8, bool),
fld2: u32,
fld3: [u64; 3],
fld4: *const isize,
fld5: [u8; 2],

},
Variant2{
fld0: i32,
fld1: *const (i64, u8, bool),
fld2: [i128; 8],
fld3: [u8; 4],
fld4: u8,

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
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: Adt40,
fld1: *mut *const i16,
fld2: [i32; 3],

},
Variant1{
fld0: i8,

},
Variant2{
fld0: [isize; 1],
fld1: char,
fld2: Adt40,
fld3: *mut i8,
fld4: i16,
fld5: u16,
fld6: Adt39,

},
Variant3{
fld0: (*const i16,),
fld1: Adt39,
fld2: [u8; 4],
fld3: f32,
fld4: i64,
fld5: i32,

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: i128,
fld1: (*const i16,),
fld2: [u64; 3],
fld3: Adt40,

},
Variant1{
fld0: (i64, u8, bool),
fld1: *mut *const i16,
fld2: isize,
fld3: Adt39,
fld4: [isize; 1],
fld5: [u64; 3],
fld6: *const [i32; 3],
fld7: Adt40,

},
Variant2{
fld0: *const i8,
fld1: *mut *const i16,
fld2: (*const i8,),
fld3: [u64; 3],
fld4: i32,

},
Variant3{
fld0: bool,
fld1: (u16, i32, i16, f64),

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: [u64; 3],
fld1: char,
fld2: i64,
fld3: ([i32; 3], [i128; 8]),
fld4: Adt39,
fld5: u32,

},
Variant1{
fld0: *const i16,
fld1: char,
fld2: i128,
fld3: *const i8,

},
Variant2{
fld0: Adt40,
fld1: *mut i8,

},
Variant3{
fld0: [u8; 2],
fld1: [i128; 8],
fld2: u8,
fld3: ([i32; 3], f32, (u16, i32, i16, f64), u128),

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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
fld0: (u16, i32, i16, f64),
fld1: *const (i64, u8, bool),
fld2: *const i16,
fld3: u128,
fld4: ([i32; 3], f32, (u16, i32, i16, f64), u128),
fld5: [i32; 3],
fld6: f64,

},
Variant1{
fld0: u16,
fld1: ([i32; 3], f32, (u16, i32, i16, f64), u128),

},
Variant2{
fld0: bool,
fld1: Adt41,
fld2: u32,
fld3: i8,
fld4: [i32; 3],
fld5: [i128; 8],
fld6: i64,
fld7: *const i8,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: u16,
fld1: [i128; 8],
fld2: Adt40,
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: usize,
fld1: u128,
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
fld0: *const isize,
fld1: char,
fld2: u128,
fld3: [u64; 3],
fld4: usize,
fld5: i128,
fld6: *mut i8,

},
Variant1{
fld0: f64,

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
fld0: *const (i64, u8, bool),
fld1: (u16, i32, i16, f64),
fld2: usize,
fld3: u32,
fld4: *mut i8,

},
Variant1{
fld0: *const i16,
fld1: usize,
fld2: i32,
fld3: [u8; 2],

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: (*const i8,),

},
Variant1{
fld0: u8,

},
Variant2{
fld0: i64,
fld1: u128,
fld2: [u64; 3],
fld3: *mut char,
fld4: (*const i16,),
fld5: Adt45,

},
Variant3{
fld0: Adt39,
fld1: (*const i8,),
fld2: [u128; 4],
fld3: u64,
fld4: Adt43,
fld5: [u64; 3],

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: Adt44,
fld1: [u64; 3],
}
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: *const isize,
fld1: Adt42,
fld2: usize,
fld3: (*const i8,),
fld4: i16,
fld5: *const [i32; 3],
fld6: Adt47,
fld7: [u8; 2],

},
Variant1{
fld0: u8,
fld1: *mut *const i16,

},
Variant2{
fld0: *mut char,
fld1: char,

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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: Adt48,
fld1: [i128; 8],
fld2: f32,
fld3: ([i32; 3], f32, (u16, i32, i16, f64), u128),
fld4: i16,
fld5: [u64; 3],
fld6: u64,

},
Variant1{
fld0: Adt46,
fld1: (*const i8,),
fld2: Adt42,
fld3: i8,
fld4: *const isize,
fld5: [u128; 4],

},
Variant2{
fld0: i16,
fld1: [isize; 1],
fld2: (*const i16,),
fld3: *mut *const i16,

},
Variant3{
fld0: Adt39,
fld1: char,
fld2: i16,
fld3: Adt42,

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
fld0: usize,
fld1: u16,
fld2: Adt49,
fld3: *mut char,
fld4: *mut i8,
fld5: ([i32; 3], f32, (u16, i32, i16, f64), u128),
fld6: (*const i8,),

},
Variant1{
fld0: *mut char,
fld1: Adt47,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: u64,
fld1: ([i32; 3], [i128; 8]),

},
Variant1{
fld0: Adt44,
fld1: char,
fld2: isize,
fld3: Adt46,
fld4: [u128; 4],
fld5: i32,
fld6: u64,
fld7: (u16, i32, i16, f64),

},
Variant2{
fld0: ([i32; 3], f32, (u16, i32, i16, f64), u128),
fld1: (*const i16,),

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: i64,
fld1: Adt46,
fld2: isize,
fld3: [u8; 4],
fld4: u16,

},
Variant1{
fld0: (i64, u8, bool),

},
Variant2{
fld0: *const [i32; 3],
fld1: Adt42,
fld2: u8,
fld3: *mut *const i16,

},
Variant3{
fld0: Adt51,
fld1: Adt46,
fld2: Adt48,
fld3: u128,
fld4: *const [i32; 3],
fld5: u8,
fld6: i64,
fld7: (*const i16,),

}}

