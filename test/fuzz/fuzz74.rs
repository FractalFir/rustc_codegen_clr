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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> bool {
mir! {
type RET = bool;
let _15: Adt54;
let _16: isize;
let _17: i32;
let _18: i16;
let _19: [isize; 7];
let _20: [isize; 7];
let _21: [u8; 1];
let _22: [char; 7];
let _23: *mut i128;
let _24: i16;
let _25: Adt43;
let _26: isize;
let _27: f32;
let _28: [u8; 1];
let _29: [u64; 1];
let _30: char;
let _31: Adt45;
let _32: bool;
let _33: [isize; 7];
let _34: f32;
let _35: u8;
let _36: Adt54;
let _37: f32;
let _38: isize;
let _39: i32;
let _40: [isize; 7];
let _41: *mut usize;
let _42: f64;
let _43: bool;
let _44: Adt48;
let _45: isize;
let _46: *mut *mut i128;
let _47: u16;
let _48: char;
let _49: isize;
let _50: ();
let _51: ();
{
_9 = true as usize;
_3 = 9223372036854775807_isize + 66_isize;
_12 = 4128128210_u32 * 2014652566_u32;
_8 = !(-77006744708096522307712357881168430933_i128);
_9 = 5_usize - 3_usize;
RET = false;
_5 = 2290111192560579665_i64 as i16;
_10 = 36_u8 - 130_u8;
_11 = 8557_u16 >> _12;
_6 = -(-19474089_i32);
_4 = (-71_i8);
_14 = 191971341056087174271252621839271807366_u128;
_14 = !149889373636896756721107386638373321460_u128;
RET = !true;
_2 = '\u{f090a}';
_8 = 148647951954785724311200158571884763799_i128 & (-54978064089439907902317304605149192528_i128);
_7 = -2795682409498217951_i64;
_13 = !7813923415194921587_u64;
_12 = _8 as u32;
_13 = 13023376614396208032_u64;
_18 = _5 + _5;
_16 = _3;
Call(_8 = fn1(_4, _14, _3, _7, _16, _9, _12, _3, _9, _3, _11, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_14 = _8 as u128;
_20 = [_3,_3,_3,_16,_16,_3,_3];
_2 = '\u{9fc68}';
RET = true & false;
_5 = _18;
_17 = !_6;
_9 = _6 as usize;
_18 = !_5;
_1 = RET;
RET = !_1;
RET = !_1;
_23 = core::ptr::addr_of_mut!(_8);
_24 = !_18;
_8 = 3263919029604320456895446051675262725_i128 & 64495499966233000117560905520693238992_i128;
_4 = (-98_i8);
_2 = '\u{b143f}';
_13 = _11 as u64;
_6 = _17;
_13 = !15516657804728407337_u64;
match _4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211358 => bb7,
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
_21 = [_10];
_26 = _16;
_5 = _18;
_26 = _3;
_6 = _17;
_14 = _5 as u128;
_14 = _11 as u128;
_22 = [_2,_2,_2,_2,_2,_2,_2];
_29 = [_13];
_6 = !_17;
_7 = (-2924585233809491748_i64);
_30 = _2;
_1 = !RET;
_10 = !135_u8;
_19 = [_3,_3,_3,_16,_16,_3,_3];
(*_23) = -(-73497361696028456953555841303323759939_i128);
_28 = [_10];
_33 = [_16,_16,_26,_26,_3,_16,_26];
_13 = 6537753967445674461_u64 << _11;
Goto(bb8)
}
bb8 = {
(*_23) = _13 as i128;
match _7 {
0 => bb6,
340282366920938463460450022197958719708 => bb10,
_ => bb9
}
}
bb9 = {
_14 = _8 as u128;
_20 = [_3,_3,_3,_16,_16,_3,_3];
_2 = '\u{9fc68}';
RET = true & false;
_5 = _18;
_17 = !_6;
_9 = _6 as usize;
_18 = !_5;
_1 = RET;
RET = !_1;
RET = !_1;
_23 = core::ptr::addr_of_mut!(_8);
_24 = !_18;
_8 = 3263919029604320456895446051675262725_i128 & 64495499966233000117560905520693238992_i128;
_4 = (-98_i8);
_2 = '\u{b143f}';
_13 = _11 as u64;
_6 = _17;
_13 = !15516657804728407337_u64;
match _4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211358 => bb7,
_ => bb6
}
}
bb10 = {
_34 = _3 as f32;
_32 = _9 <= _9;
_24 = _18;
_16 = _9 as isize;
_19 = _20;
_35 = _10;
_28 = [_35];
(*_23) = -6956682086291382360488878711268173359_i128;
_27 = _34 - _34;
_28 = _21;
_39 = !_17;
_21 = [_10];
_5 = _13 as i16;
_22 = [_2,_30,_30,_2,_30,_2,_30];
Call((*_23) = fn2(_11, _16, _20, _6, _27, _33, _12, _18, _33, _7), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_9 = 7_usize;
_7 = 8600396721119001555_i64 ^ 4235791315098289817_i64;
_33 = _19;
(*_23) = (-59039555271121283177428868018406043075_i128) & (-137848009746772434278483307346689829464_i128);
_34 = _4 as f32;
_10 = _35 * _35;
_29 = [_13];
_2 = _30;
_5 = _18;
_30 = _2;
(*_23) = !(-92248430513069491135220933563843086269_i128);
match _9 {
7 => bb13,
_ => bb12
}
}
bb12 = {
(*_23) = _13 as i128;
match _7 {
0 => bb6,
340282366920938463460450022197958719708 => bb10,
_ => bb9
}
}
bb13 = {
(*_23) = 3554292641686185218293947001682016827_i128 + 117926112810628793399726468263008020286_i128;
_12 = _3 as u32;
_16 = _3;
_16 = _26;
_29 = [_13];
_40 = _20;
_35 = _10;
match _9 {
7 => bb15,
_ => bb14
}
}
bb14 = {
(*_23) = _13 as i128;
match _7 {
0 => bb6,
340282366920938463460450022197958719708 => bb10,
_ => bb9
}
}
bb15 = {
_26 = _16;
_39 = -_6;
_6 = _14 as i32;
_21 = [_35];
_33 = [_3,_3,_16,_3,_16,_26,_16];
_6 = -_17;
_42 = _12 as f64;
_44.fld0 = (_18,);
_3 = -_16;
_28 = [_35];
_27 = -_34;
Goto(bb16)
}
bb16 = {
Call(_50 = dump_var(0_usize, 26_usize, Move(_26), 8_usize, Move(_8), 30_usize, Move(_30), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_50 = dump_var(0_usize, 10_usize, Move(_10), 13_usize, Move(_13), 6_usize, Move(_6), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_50 = dump_var(0_usize, 2_usize, Move(_2), 21_usize, Move(_21), 11_usize, Move(_11), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_50 = dump_var(0_usize, 22_usize, Move(_22), 39_usize, Move(_39), 7_usize, Move(_7), 51_usize, _51), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i8,mut _2: u128,mut _3: isize,mut _4: i64,mut _5: isize,mut _6: usize,mut _7: u32,mut _8: isize,mut _9: usize,mut _10: isize,mut _11: u16,mut _12: char) -> i128 {
mir! {
type RET = i128;
let _13: i64;
let _14: f32;
let _15: *const char;
let _16: u8;
let _17: f32;
let _18: [u32; 1];
let _19: [u64; 1];
let _20: *mut usize;
let _21: (i16,);
let _22: isize;
let _23: char;
let _24: [i8; 5];
let _25: [u128; 6];
let _26: f32;
let _27: [char; 4];
let _28: [char; 4];
let _29: isize;
let _30: (i16,);
let _31: Adt46;
let _32: *const char;
let _33: f64;
let _34: bool;
let _35: f32;
let _36: [char; 4];
let _37: ();
let _38: ();
{
_5 = _3 >> _10;
_2 = !96894066098760293496233778440082689148_u128;
_11 = 22735_u16 << _4;
_10 = (-974706485_i32) as isize;
_1 = (-14_i8);
_11 = _7 as u16;
_11 = 38305_u16 | 24094_u16;
_6 = 122305811911305917081976090872483774514_i128 as usize;
_10 = -_5;
_8 = _5 - _10;
_6 = _9 >> _10;
RET = -(-120286131573047112152737698489382548008_i128);
_7 = 2544524694_u32 << _8;
_8 = _5 ^ _10;
_1 = _8 as i8;
_2 = 40494280102767507888851536124261762010_u128 & 25132244665419497489249149182474232078_u128;
_9 = !_6;
_4 = !1113371544559650414_i64;
_10 = _3 - _8;
_6 = _9 >> _1;
RET = -(-52141735301729986717399188340515104617_i128);
_11 = 60242_u16;
_12 = '\u{5b6a2}';
Call(_10 = core::intrinsics::bswap(_5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = !44667_u16;
_2 = !106682107597573412475098323570707538598_u128;
_8 = _3 >> _6;
_3 = _10;
_2 = !193911213891481430933076601122171534811_u128;
_12 = '\u{c38c3}';
_3 = _8 + _8;
_14 = (-1294903420_i32) as f32;
_8 = !_3;
Goto(bb2)
}
bb2 = {
_5 = _3;
_8 = _10 ^ _3;
_15 = core::ptr::addr_of!(_12);
RET = (-939_i16) as i128;
_13 = _4;
_12 = '\u{37a06}';
_8 = true as isize;
_10 = _3;
(*_15) = '\u{b1216}';
_16 = 2_u8 ^ 231_u8;
_8 = true as isize;
_12 = '\u{729cb}';
_9 = _6;
_2 = 180572093398196765532349035380360395316_u128;
RET = (-54774766030378270602243311251218502407_i128);
_15 = core::ptr::addr_of!((*_15));
_18 = [_7];
match _2 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
180572093398196765532349035380360395316 => bb8,
_ => bb7
}
}
bb3 = {
_11 = !44667_u16;
_2 = !106682107597573412475098323570707538598_u128;
_8 = _3 >> _6;
_3 = _10;
_2 = !193911213891481430933076601122171534811_u128;
_12 = '\u{c38c3}';
_3 = _8 + _8;
_14 = (-1294903420_i32) as f32;
_8 = !_3;
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
_11 = 63329_u16;
_19 = [6200635429766936589_u64];
RET = 19666470375618357416538802541904345619_i128;
_8 = _10;
_14 = _4 as f32;
_6 = !_9;
_15 = core::ptr::addr_of!((*_15));
_21.0 = 19412_i16 << _8;
(*_15) = '\u{e3a65}';
RET = _7 as i128;
_22 = _5 + _5;
_6 = !_9;
_16 = !208_u8;
_4 = _16 as i64;
_1 = (-56_i8);
RET = !(-98081557732632735898806062607587921568_i128);
RET = (-49088502279379377693962656726807136862_i128);
Goto(bb9)
}
bb9 = {
_17 = _8 as f32;
_2 = 266187785094817731537090663586253436289_u128 * 281848350795534108210277724159271150973_u128;
_3 = _8;
_22 = _8;
_6 = !_9;
_15 = core::ptr::addr_of!((*_15));
_17 = _14 * _14;
_4 = _13 + _13;
_25 = [_2,_2,_2,_2,_2,_2];
RET = !117215573170675279543839125013614885098_i128;
_7 = 10247537077047135612_u64 as u32;
_25 = [_2,_2,_2,_2,_2,_2];
_24 = [_1,_1,_1,_1,_1];
_7 = !2668641449_u32;
_17 = _2 as f32;
Goto(bb10)
}
bb10 = {
_22 = _5;
_16 = 238_u8 ^ 62_u8;
_9 = _6 & _6;
_3 = -_22;
_30.0 = _21.0 & _21.0;
_15 = core::ptr::addr_of!(_12);
_23 = (*_15);
_28 = [_12,_12,(*_15),(*_15)];
_11 = 36051_u16 & 53165_u16;
(*_15) = _23;
_30.0 = _7 as i16;
_10 = _8 * _22;
_27 = [(*_15),_23,(*_15),(*_15)];
_31.fld0.1 = core::ptr::addr_of!(_7);
_12 = _23;
_31.fld6 = core::ptr::addr_of_mut!(_31.fld1);
_9 = _6 * _6;
_31.fld3 = [_12,_12,(*_15),_12,_23,(*_15),_23];
_13 = _4;
Goto(bb11)
}
bb11 = {
_24 = [_1,_1,_1,_1,_1];
_31.fld6 = core::ptr::addr_of_mut!(_31.fld1);
_31.fld2.0 = !_21.0;
_32 = _15;
_26 = _17;
_31.fld0.1 = core::ptr::addr_of!(_31.fld0.0);
_16 = _3 as u8;
_20 = core::ptr::addr_of_mut!(_31.fld1);
RET = _11 as i128;
_20 = core::ptr::addr_of_mut!(_31.fld1);
_2 = 32136687805279422339334308359737386483_u128 - 270960164604967425778929142568565737198_u128;
(*_32) = _23;
(*_20) = !_6;
(*_15) = _23;
match _1 {
0 => bb1,
1 => bb12,
2 => bb13,
340282366920938463463374607431768211400 => bb15,
_ => bb14
}
}
bb12 = {
Return()
}
bb13 = {
_11 = !44667_u16;
_2 = !106682107597573412475098323570707538598_u128;
_8 = _3 >> _6;
_3 = _10;
_2 = !193911213891481430933076601122171534811_u128;
_12 = '\u{c38c3}';
_3 = _8 + _8;
_14 = (-1294903420_i32) as f32;
_8 = !_3;
Goto(bb2)
}
bb14 = {
Return()
}
bb15 = {
_31.fld0.3 = -_4;
_33 = _16 as f64;
(*_32) = _23;
_36 = [_23,(*_15),(*_32),(*_32)];
(*_32) = _23;
_7 = !3680059904_u32;
_31.fld5 = [_16];
_30 = (_21.0,);
_30 = _31.fld2;
_13 = _14 as i64;
_26 = _17 + _14;
_8 = _10;
_25 = [_2,_2,_2,_2,_2,_2];
_2 = _33 as u128;
_3 = _10;
_25 = [_2,_2,_2,_2,_2,_2];
_13 = _17 as i64;
Goto(bb16)
}
bb16 = {
Call(_37 = dump_var(1_usize, 3_usize, Move(_3), 6_usize, Move(_6), 28_usize, Move(_28), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(1_usize, 27_usize, Move(_27), 24_usize, Move(_24), 21_usize, Move(_21), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(1_usize, 18_usize, Move(_18), 23_usize, Move(_23), 22_usize, Move(_22), 7_usize, Move(_7)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: u16,mut _2: isize,mut _3: [isize; 7],mut _4: i32,mut _5: f32,mut _6: [isize; 7],mut _7: u32,mut _8: i16,mut _9: [isize; 7],mut _10: i64) -> i128 {
mir! {
type RET = i128;
let _11: bool;
let _12: (isize, [char; 4], u32);
let _13: char;
let _14: i32;
let _15: f32;
let _16: isize;
let _17: *const u32;
let _18: *mut [isize; 7];
let _19: Adt53;
let _20: isize;
let _21: [bool; 5];
let _22: bool;
let _23: [u128; 6];
let _24: i32;
let _25: [u128; 6];
let _26: isize;
let _27: i8;
let _28: isize;
let _29: Adt46;
let _30: [i16; 4];
let _31: f32;
let _32: f32;
let _33: [u8; 1];
let _34: [char; 4];
let _35: bool;
let _36: (u32, *const u32, u32, i64);
let _37: [char; 4];
let _38: Adt56;
let _39: u16;
let _40: Adt52;
let _41: f32;
let _42: Adt54;
let _43: char;
let _44: isize;
let _45: i128;
let _46: [i16; 4];
let _47: f64;
let _48: *const char;
let _49: u64;
let _50: ();
let _51: ();
{
RET = -39301978031652960212508854964347796543_i128;
_10 = !7042018613782985702_i64;
_3 = _9;
_1 = 51805_u16;
_12.1 = ['\u{36eb0}','\u{28a46}','\u{c1970}','\u{f81c7}'];
_1 = 9787_u16 + 50214_u16;
_3 = _6;
_2 = (-9223372036854775808_isize);
_3 = _9;
_7 = 3613610856_u32 * 2100504441_u32;
_13 = '\u{8bab9}';
_12.2 = _8 as u32;
_5 = 1420445833323605424_u64 as f32;
_12.1 = [_13,_13,_13,_13];
_5 = _12.2 as f32;
_12.0 = _2;
_11 = _1 > _1;
_12.2 = _7 | _7;
_11 = true;
_8 = !(-1397_i16);
_8 = 560933724621182730_u64 as i16;
_2 = !_12.0;
_13 = '\u{ab415}';
_12.1 = [_13,_13,_13,_13];
Call(_3 = fn3(_12, RET, _12, _2, _6, _13, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_14 = _4;
_6 = _3;
_9 = _3;
_5 = 6_usize as f32;
Goto(bb2)
}
bb2 = {
_13 = '\u{1900b}';
_15 = _5;
_8 = 6021_i16 | 21806_i16;
_12.0 = _2 + _2;
_14 = _4;
_12.2 = _7 ^ _7;
_16 = _2;
_5 = _8 as f32;
_16 = 240_u8 as isize;
_5 = -_15;
_12.1 = [_13,_13,_13,_13];
_18 = core::ptr::addr_of_mut!(_3);
_5 = _15;
_5 = _15 + _15;
_10 = _11 as i64;
_12.1 = [_13,_13,_13,_13];
_17 = core::ptr::addr_of!(_7);
(*_18) = _6;
RET = 164894492713364914385967770157421845384_i128 & 160408026922224901539203118905061288061_i128;
_3 = [_12.0,_16,_16,_2,_16,_16,_12.0];
_6 = [_12.0,_16,_2,_12.0,_16,_12.0,_2];
Goto(bb3)
}
bb3 = {
_22 = _11;
_7 = RET as u32;
_8 = _11 as i16;
Goto(bb4)
}
bb4 = {
_21 = [_22,_22,_11,_22,_11];
_4 = _14;
_5 = _15;
(*_17) = 3769923141178415525_usize as u32;
_24 = -_4;
_3 = [_2,_2,_16,_12.0,_12.0,_12.0,_12.0];
_5 = _15 - _15;
_20 = _12.0;
RET = _1 as i128;
_12.1 = [_13,_13,_13,_13];
_17 = core::ptr::addr_of!((*_17));
_12.1 = [_13,_13,_13,_13];
RET = 214173244440977750817175800961377615106_u128 as i128;
_14 = _24;
_1 = 20385_u16;
_12.2 = _1 as u32;
_9 = [_20,_12.0,_12.0,_12.0,_12.0,_12.0,_2];
_26 = -_16;
_25 = [296360073467920672855498197716093741395_u128,77443960116350114635341100475117298843_u128,186733417804576385789374964999255452518_u128,205078373500060017116898742870608480121_u128,289582084467468707991911044425617419282_u128,214865127021062326938720401515255678402_u128];
_10 = (-3841674321807216738_i64);
_8 = 108_u8 as i16;
Call(_12.2 = core::intrinsics::transmute((*_17)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_12.0 = _20 - _2;
_22 = !_11;
RET = (-111657506211666161537905311783253042464_i128);
_27 = -26_i8;
_20 = !_2;
_29.fld0.2 = _27 as u32;
_2 = _12.0 + _26;
_29.fld2.0 = 6495960087407031696_u64 as i16;
_7 = !_29.fld0.2;
_11 = _22;
_3 = _9;
_28 = 3461266111317401306_u64 as isize;
_16 = _7 as isize;
_3 = _6;
_30 = [_29.fld2.0,_8,_29.fld2.0,_8];
_16 = _12.0 >> (*_17);
_29.fld5 = [47_u8];
_10 = -(-4352297149461291008_i64);
_29.fld0 = ((*_17), _17, (*_17), _10);
_29.fld2.0 = _8;
(*_17) = _29.fld0.2;
_29.fld4 = [_2,_12.0,_12.0,_16,_2,_28,_16];
_29.fld0.1 = _17;
_16 = 17568967900835759235_usize as isize;
_29.fld6 = core::ptr::addr_of_mut!(_29.fld1);
_25 = [160933528031463085066633000185851807232_u128,197302899442246782272371897838173257389_u128,253701923035319651596264065402683372942_u128,282965523067024500860976179260399892057_u128,24005758424596356508173837666695440105_u128,97609330155079495689597026098565808896_u128];
Goto(bb6)
}
bb6 = {
_22 = _11 ^ _11;
_12.1 = [_13,_13,_13,_13];
_31 = _10 as f32;
_34 = _12.1;
(*_18) = [_12.0,_28,_2,_16,_12.0,_2,_2];
_6 = (*_18);
(*_18) = [_12.0,_26,_2,_12.0,_20,_2,_12.0];
_33 = _29.fld5;
_17 = core::ptr::addr_of!((*_17));
_5 = 4649412907162802166_u64 as f32;
_36 = (_7, _29.fld0.1, _7, _29.fld0.3);
_29.fld2.0 = _1 as i16;
_9 = [_20,_2,_2,_16,_2,_12.0,_12.0];
_22 = !_11;
_36.3 = _29.fld0.3 << _29.fld0.2;
_12.2 = _36.0 << _14;
Goto(bb7)
}
bb7 = {
_22 = _11;
_29.fld0.2 = (*_17) ^ (*_17);
_22 = _11;
_10 = _29.fld0.3 << _14;
_29.fld1 = RET as usize;
_29.fld2.0 = _13 as i16;
(*_18) = _29.fld4;
_28 = 243001708297213333722799839962237227941_u128 as isize;
_22 = !_11;
_29.fld4 = _9;
_25 = [68454052238544906144710336454340873243_u128,35408880036579594588673327031230943850_u128,234928742033727082762533280281740111033_u128,166950409146500178809999766769343086334_u128,44390039284261409671374471272658655567_u128,63091909132354300561765145089914829284_u128];
_29.fld0.1 = core::ptr::addr_of!((*_17));
_9 = [_2,_12.0,_2,_26,_2,_12.0,_2];
_29.fld3 = [_13,_13,_13,_13,_13,_13,_13];
_36.3 = _29.fld0.3;
_6 = (*_18);
_8 = !_29.fld2.0;
RET = (-148564808623009207481263127143047136263_i128) | (-20296886553970567005725096964761650473_i128);
Goto(bb8)
}
bb8 = {
_35 = !_11;
_7 = !_12.2;
_5 = _12.2 as f32;
Call(_17 = fn19(Move(_29), _36.1, _35, (*_18), _36.1, _28, _16, _36, _3, (*_17), _12.0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_29.fld3 = [_13,_13,_13,_13,_13,_13,_13];
match _1 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb8,
4 => bb5,
5 => bb10,
20385 => bb12,
_ => bb11
}
}
bb10 = {
_35 = !_11;
_7 = !_12.2;
_5 = _12.2 as f32;
Call(_17 = fn19(Move(_29), _36.1, _35, (*_18), _36.1, _28, _16, _36, _3, (*_17), _12.0), ReturnTo(bb9), UnwindUnreachable())
}
bb11 = {
_22 = _11 ^ _11;
_12.1 = [_13,_13,_13,_13];
_31 = _10 as f32;
_34 = _12.1;
(*_18) = [_12.0,_28,_2,_16,_12.0,_2,_2];
_6 = (*_18);
(*_18) = [_12.0,_26,_2,_12.0,_20,_2,_12.0];
_33 = _29.fld5;
_17 = core::ptr::addr_of!((*_17));
_5 = 4649412907162802166_u64 as f32;
_36 = (_7, _29.fld0.1, _7, _29.fld0.3);
_29.fld2.0 = _1 as i16;
_9 = [_20,_2,_2,_16,_2,_12.0,_12.0];
_22 = !_11;
_36.3 = _29.fld0.3 << _29.fld0.2;
_12.2 = _36.0 << _14;
Goto(bb7)
}
bb12 = {
_9 = [_28,_2,_2,_28,_2,_12.0,_2];
_7 = _36.2;
_33 = [248_u8];
_29.fld1 = 3_usize;
_40.fld0 = (_8,);
_29.fld0.2 = _36.0 >> _27;
_29.fld0.3 = _1 as i64;
_32 = _31;
_23 = [215150472414430194378589096449786965273_u128,313541308712320708434984463144525100466_u128,52428811743077356118622330999166838170_u128,134760546372367955681281980684484789646_u128,62731655992032214374195694086293088000_u128,1027419260517907908580637028703984349_u128];
_40.fld1 = [_27,_27,_27,_27,_27];
_14 = _12.2 as i32;
_12.2 = _14 as u32;
_12.1 = [_13,_13,_13,_13];
_10 = _36.3 ^ _29.fld0.3;
_29.fld0 = _36;
_27 = _11 as i8;
_29.fld0.3 = !_36.3;
_31 = -_5;
_7 = !_12.2;
_41 = 4282594708151746785_u64 as f32;
_5 = _29.fld0.2 as f32;
_29.fld5 = [110_u8];
_24 = 10353007289028092501_u64 as i32;
(*_18) = [_2,_28,_2,_12.0,_2,_26,_16];
_40.fld0.0 = _8;
Goto(bb13)
}
bb13 = {
_29.fld5 = [32_u8];
match _1 {
20385 => bb14,
_ => bb10
}
}
bb14 = {
_28 = !_16;
_30 = [_8,_40.fld0.0,_8,_8];
_36.2 = _7;
_36.3 = !_10;
_18 = core::ptr::addr_of_mut!(_3);
_37 = [_13,_13,_13,_13];
_16 = !_12.0;
_10 = _27 as i64;
_18 = core::ptr::addr_of_mut!(_6);
_45 = RET;
_10 = _36.3;
_36.0 = _12.2;
_11 = _12.0 >= _2;
_15 = 274796476348216186439579133741777728902_u128 as f32;
_43 = _13;
_35 = _20 >= _2;
_29.fld0.2 = _10 as u32;
_5 = _12.2 as f32;
_40.fld0.0 = _8 + _8;
_32 = _41;
RET = _31 as i128;
_36 = _29.fld0;
Goto(bb15)
}
bb15 = {
Call(_50 = dump_var(2_usize, 33_usize, Move(_33), 20_usize, Move(_20), 24_usize, Move(_24), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_50 = dump_var(2_usize, 45_usize, Move(_45), 16_usize, Move(_16), 9_usize, Move(_9), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_50 = dump_var(2_usize, 26_usize, Move(_26), 1_usize, Move(_1), 13_usize, Move(_13), 23_usize, Move(_23)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_50 = dump_var(2_usize, 12_usize, Move(_12), 10_usize, Move(_10), 34_usize, Move(_34), 51_usize, _51), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: (isize, [char; 4], u32),mut _2: i128,mut _3: (isize, [char; 4], u32),mut _4: isize,mut _5: [isize; 7],mut _6: char,mut _7: f32) -> [isize; 7] {
mir! {
type RET = [isize; 7];
let _8: Adt50;
let _9: [isize; 7];
let _10: Adt49;
let _11: Adt52;
let _12: [i32; 8];
let _13: [isize; 7];
let _14: (isize, [char; 4], u32);
let _15: Adt48;
let _16: bool;
let _17: [u64; 1];
let _18: i64;
let _19: f64;
let _20: Adt55;
let _21: ();
let _22: ();
{
_8.fld3 = (-87_i8) + 96_i8;
_8.fld4 = 3255846661314895545_u64 << _8.fld3;
_8.fld5 = _8.fld3 as i32;
_6 = '\u{75d94}';
_8.fld5 = true as i32;
RET = [_4,_4,_3.0,_1.0,_3.0,_3.0,_1.0];
RET = _5;
_8.fld4 = 15334029215533158200_u64 + 12274346078155629133_u64;
_1.0 = -_4;
_9 = [_1.0,_3.0,_1.0,_1.0,_4,_4,_3.0];
_8.fld0 = !18939519526500623317248361230572581077_u128;
_8.fld7 = core::ptr::addr_of!(_6);
_8.fld7 = core::ptr::addr_of!(_6);
_1.0 = _4 & _3.0;
_3 = (_1.0, _1.1, _1.2);
Call(_8.fld0 = fn4(_1, _3.0, _1.0, _3.2, _1, _3.2, _3.0, _1, _1, _1, RET, _3, _8.fld3, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11.fld0.0 = !7141_i16;
_11.fld1 = [_8.fld3,_8.fld3,_8.fld3,_8.fld3,_8.fld3];
_1 = (_3.0, _3.1, _3.2);
RET = [_3.0,_3.0,_3.0,_1.0,_4,_4,_4];
_1.0 = _3.0;
_8.fld7 = core::ptr::addr_of!(_6);
_9 = _5;
_14 = (_1.0, _3.1, _3.2);
_3 = _1;
_6 = '\u{b407a}';
_1 = (_14.0, _3.1, _14.2);
RET = [_14.0,_3.0,_1.0,_1.0,_4,_1.0,_3.0];
RET = [_14.0,_3.0,_3.0,_3.0,_1.0,_3.0,_4];
_13 = RET;
_9 = [_3.0,_3.0,_14.0,_1.0,_14.0,_3.0,_14.0];
_14.2 = _11.fld0.0 as u32;
Goto(bb2)
}
bb2 = {
_8.fld0 = 225484510723750281087357729768812908155_u128;
_8.fld6 = [_8.fld0,_8.fld0,_8.fld0,_8.fld0,_8.fld0,_8.fld0];
_4 = _1.0;
_1.2 = !_14.2;
_13 = [_1.0,_3.0,_14.0,_3.0,_4,_3.0,_1.0];
_14.2 = !_3.2;
_3.1 = [_6,_6,_6,_6];
_3.0 = _3.2 as isize;
_3.2 = _14.2 & _14.2;
RET = [_3.0,_14.0,_1.0,_1.0,_4,_4,_14.0];
_8.fld3 = (-111_i8);
_8.fld5 = (-1191321222_i32);
_8.fld7 = core::ptr::addr_of!(_6);
_1.0 = _3.0 | _4;
_12 = [_8.fld5,_8.fld5,_8.fld5,_8.fld5,_8.fld5,_8.fld5,_8.fld5,_8.fld5];
_3.2 = _14.2;
_1.2 = _8.fld0 as u32;
Goto(bb3)
}
bb3 = {
_2 = -(-55046850083417829065481123607998025746_i128);
_15 = Adt48 { fld0: _11.fld0 };
match _8.fld0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
225484510723750281087357729768812908155 => bb10,
_ => bb9
}
}
bb4 = {
_8.fld0 = 225484510723750281087357729768812908155_u128;
_8.fld6 = [_8.fld0,_8.fld0,_8.fld0,_8.fld0,_8.fld0,_8.fld0];
_4 = _1.0;
_1.2 = !_14.2;
_13 = [_1.0,_3.0,_14.0,_3.0,_4,_3.0,_1.0];
_14.2 = !_3.2;
_3.1 = [_6,_6,_6,_6];
_3.0 = _3.2 as isize;
_3.2 = _14.2 & _14.2;
RET = [_3.0,_14.0,_1.0,_1.0,_4,_4,_14.0];
_8.fld3 = (-111_i8);
_8.fld5 = (-1191321222_i32);
_8.fld7 = core::ptr::addr_of!(_6);
_1.0 = _3.0 | _4;
_12 = [_8.fld5,_8.fld5,_8.fld5,_8.fld5,_8.fld5,_8.fld5,_8.fld5,_8.fld5];
_3.2 = _14.2;
_1.2 = _8.fld0 as u32;
Goto(bb3)
}
bb5 = {
_11.fld0.0 = !7141_i16;
_11.fld1 = [_8.fld3,_8.fld3,_8.fld3,_8.fld3,_8.fld3];
_1 = (_3.0, _3.1, _3.2);
RET = [_3.0,_3.0,_3.0,_1.0,_4,_4,_4];
_1.0 = _3.0;
_8.fld7 = core::ptr::addr_of!(_6);
_9 = _5;
_14 = (_1.0, _3.1, _3.2);
_3 = _1;
_6 = '\u{b407a}';
_1 = (_14.0, _3.1, _14.2);
RET = [_14.0,_3.0,_1.0,_1.0,_4,_1.0,_3.0];
RET = [_14.0,_3.0,_3.0,_3.0,_1.0,_3.0,_4];
_13 = RET;
_9 = [_3.0,_3.0,_14.0,_1.0,_14.0,_3.0,_14.0];
_14.2 = _11.fld0.0 as u32;
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
Return()
}
bb10 = {
_5 = _13;
_15.fld0.0 = !_11.fld0.0;
_9 = [_1.0,_1.0,_1.0,_3.0,_14.0,_1.0,_1.0];
_3.2 = _14.2 | _14.2;
_8.fld5 = (-1702057273_i32) | (-160198650_i32);
_11.fld0.0 = _6 as i16;
_8.fld2 = [_15.fld0.0,_15.fld0.0,_15.fld0.0,_11.fld0.0];
match _8.fld0 {
0 => bb7,
1 => bb11,
2 => bb12,
225484510723750281087357729768812908155 => bb14,
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
_8.fld0 = 225484510723750281087357729768812908155_u128;
_8.fld6 = [_8.fld0,_8.fld0,_8.fld0,_8.fld0,_8.fld0,_8.fld0];
_4 = _1.0;
_1.2 = !_14.2;
_13 = [_1.0,_3.0,_14.0,_3.0,_4,_3.0,_1.0];
_14.2 = !_3.2;
_3.1 = [_6,_6,_6,_6];
_3.0 = _3.2 as isize;
_3.2 = _14.2 & _14.2;
RET = [_3.0,_14.0,_1.0,_1.0,_4,_4,_14.0];
_8.fld3 = (-111_i8);
_8.fld5 = (-1191321222_i32);
_8.fld7 = core::ptr::addr_of!(_6);
_1.0 = _3.0 | _4;
_12 = [_8.fld5,_8.fld5,_8.fld5,_8.fld5,_8.fld5,_8.fld5,_8.fld5,_8.fld5];
_3.2 = _14.2;
_1.2 = _8.fld0 as u32;
Goto(bb3)
}
bb14 = {
RET = _9;
RET = [_1.0,_1.0,_3.0,_3.0,_4,_1.0,_3.0];
_1.2 = _14.2;
_14 = (_1.0, _1.1, _3.2);
_3 = (_1.0, _1.1, _14.2);
_3.0 = _1.0 + _1.0;
_8.fld3 = -54_i8;
RET = [_3.0,_14.0,_14.0,_3.0,_3.0,_3.0,_1.0];
_18 = -(-1846514620916671780_i64);
_3.0 = -_14.0;
_17 = [_8.fld4];
_3.1 = [_6,_6,_6,_6];
_8.fld5 = 1510712251_i32;
_19 = _8.fld5 as f64;
_11.fld0 = (_15.fld0.0,);
_1.1 = _14.1;
_9 = [_3.0,_3.0,_14.0,_14.0,_14.0,_14.0,_4];
_8.fld5 = 386166530_i32;
_9 = _13;
_1.0 = _3.0;
_11.fld0 = (_15.fld0.0,);
_8.fld7 = core::ptr::addr_of!(_6);
_9 = RET;
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(3_usize, 2_usize, Move(_2), 12_usize, Move(_12), 3_usize, Move(_3), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_21 = dump_var(3_usize, 17_usize, Move(_17), 4_usize, Move(_4), 22_usize, _22, 22_usize, _22), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: (isize, [char; 4], u32),mut _2: isize,mut _3: isize,mut _4: u32,mut _5: (isize, [char; 4], u32),mut _6: u32,mut _7: isize,mut _8: (isize, [char; 4], u32),mut _9: (isize, [char; 4], u32),mut _10: (isize, [char; 4], u32),mut _11: [isize; 7],mut _12: (isize, [char; 4], u32),mut _13: i8,mut _14: f32) -> u128 {
mir! {
type RET = u128;
let _15: *const u32;
let _16: [char; 4];
let _17: Adt48;
let _18: u64;
let _19: [u32; 1];
let _20: f32;
let _21: Adt45;
let _22: u8;
let _23: [i8; 5];
let _24: [u8; 1];
let _25: isize;
let _26: i16;
let _27: [u8; 1];
let _28: u16;
let _29: Adt52;
let _30: [u128; 6];
let _31: bool;
let _32: *const (isize, [char; 4], u32);
let _33: [char; 4];
let _34: bool;
let _35: f64;
let _36: u16;
let _37: [i16; 4];
let _38: [bool; 5];
let _39: isize;
let _40: [u64; 1];
let _41: [bool; 5];
let _42: u32;
let _43: Adt48;
let _44: Adt46;
let _45: u32;
let _46: *const (u32, *const u32, u32, i64);
let _47: ();
let _48: ();
{
_1 = _9;
_12.2 = !_8.2;
_5.0 = _8.0 - _1.0;
_10 = (_5.0, _5.1, _4);
_1.1 = ['\u{b3bcb}','\u{5f920}','\u{f999a}','\u{b185c}'];
_12.2 = _4;
RET = 310272070053307220662729506460360407142_u128 & 311601281675681050546103423574976949166_u128;
_1 = _12;
_5.1 = ['\u{59798}','\u{c863c}','\u{fcc66}','\u{1f81d}'];
_10 = (_3, _1.1, _6);
_8 = _12;
_2 = !_12.0;
_16 = ['\u{101de1}','\u{9cd3a}','\u{a5b39}','\u{107faa}'];
Goto(bb1)
}
bb1 = {
_17.fld0.0 = 16598_i16;
_15 = core::ptr::addr_of!(_12.2);
_10.0 = _5.0;
_10.1 = ['\u{105a42}','\u{eeb33}','\u{9855e}','\u{8180b}'];
_18 = 16268328393516219135_u64 + 4000953505041252447_u64;
_9.2 = (*_15) * _6;
_8.1 = ['\u{d6c2f}','\u{243e6}','\u{67d14}','\u{bd1d1}'];
_1.0 = 11453_u16 as isize;
_20 = _14;
_8.2 = (*_15) * _9.2;
RET = _17.fld0.0 as u128;
_15 = core::ptr::addr_of!(_6);
_17.fld0 = ((-22354_i16),);
_20 = 42812_u16 as f32;
_1.1 = ['\u{bccfe}','\u{45015}','\u{a741}','\u{b17a3}'];
_8.1 = ['\u{214ea}','\u{616f8}','\u{b63c1}','\u{9e13a}'];
_24 = [225_u8];
match _17.fld0.0 {
0 => bb2,
1 => bb3,
340282366920938463463374607431768189102 => bb5,
_ => bb4
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
_1.0 = _2;
_8.1 = _5.1;
_14 = _20;
_17.fld0 = ((-14303_i16),);
_5.0 = _3;
_5.0 = _10.0;
_8.2 = !_9.2;
(*_15) = _9.2 ^ _1.2;
_27 = [254_u8];
_5 = _10;
_10.1 = _1.1;
_5.0 = _10.0;
_9.0 = _7 << _4;
_19 = [_6];
(*_15) = _20 as u32;
_9.1 = ['\u{75b06}','\u{2d092}','\u{108a6}','\u{10df1b}'];
_1 = _10;
_10.2 = !_8.2;
_29.fld1 = [_13,_13,_13,_13,_13];
Goto(bb6)
}
bb6 = {
_17.fld0.0 = 15400_i16 * 30677_i16;
_12.0 = _10.0 * _8.0;
RET = (-8858065668325920488559287389937318301_i128) as u128;
_12 = (_5.0, _9.1, _8.2);
_15 = core::ptr::addr_of!(_12.2);
_1.0 = _5.0 << _12.2;
_9.2 = false as u32;
_8.1 = _1.1;
Goto(bb7)
}
bb7 = {
_9.1 = _10.1;
_10.2 = (*_15);
_18 = _12.2 as u64;
_29.fld0 = _17.fld0;
_23 = [_13,_13,_13,_13,_13];
_7 = _14 as isize;
_17.fld0.0 = _29.fld0.0 << _9.0;
_29.fld1 = [_13,_13,_13,_13,_13];
_9.1 = ['\u{9f84f}','\u{1095b1}','\u{b86cf}','\u{a4ba2}'];
_5.2 = _10.2;
_17.fld0.0 = _29.fld0.0;
_10.2 = _17.fld0.0 as u32;
_28 = 21599_u16;
_9.0 = _1.0;
_14 = _20;
_30 = [RET,RET,RET,RET,RET,RET];
_25 = _13 as isize;
_29 = Adt52 { fld0: _17.fld0,fld1: _23 };
_8.1 = ['\u{efefc}','\u{571c2}','\u{56d05}','\u{c0d91}'];
_27 = _24;
_29.fld0 = _17.fld0;
match _28 {
21599 => bb9,
_ => bb8
}
}
bb8 = {
_1.0 = _2;
_8.1 = _5.1;
_14 = _20;
_17.fld0 = ((-14303_i16),);
_5.0 = _3;
_5.0 = _10.0;
_8.2 = !_9.2;
(*_15) = _9.2 ^ _1.2;
_27 = [254_u8];
_5 = _10;
_10.1 = _1.1;
_5.0 = _10.0;
_9.0 = _7 << _4;
_19 = [_6];
(*_15) = _20 as u32;
_9.1 = ['\u{75b06}','\u{2d092}','\u{108a6}','\u{10df1b}'];
_1 = _10;
_10.2 = !_8.2;
_29.fld1 = [_13,_13,_13,_13,_13];
Goto(bb6)
}
bb9 = {
_1.2 = !(*_15);
_4 = !_8.2;
_12 = (_9.0, _9.1, _10.2);
_10.1 = _5.1;
_14 = _20 * _20;
_31 = !true;
_1.2 = !_4;
_30 = [RET,RET,RET,RET,RET,RET];
_26 = !_17.fld0.0;
_29.fld0.0 = _17.fld0.0 << _26;
_29.fld0.0 = _26 | _17.fld0.0;
_12.2 = !_8.2;
_1.1 = ['\u{102876}','\u{54e19}','\u{b768e}','\u{4ef1e}'];
_23 = [_13,_13,_13,_13,_13];
_16 = ['\u{3dc7e}','\u{c6d38}','\u{c7a69}','\u{b73be}'];
_22 = _14 as u8;
_3 = -_12.0;
_20 = _14;
Goto(bb10)
}
bb10 = {
_24 = [_22];
_29.fld0.0 = _14 as i16;
_5.1 = _10.1;
_33 = ['\u{8dd2e}','\u{106d9b}','\u{a61c1}','\u{fb3f7}'];
_10.0 = _5.0;
_27 = [_22];
_5.2 = _12.2 << _4;
_12.0 = _1.0 ^ _3;
_8.0 = _9.0;
_1.0 = !_3;
_9 = (_1.0, _5.1, _12.2);
_23 = [_13,_13,_13,_13,_13];
_24 = [_22];
_14 = _20 - _20;
_23 = _29.fld1;
RET = !257486071326983420398364247258768575461_u128;
_10.1 = _5.1;
_10.2 = _31 as u32;
_37 = [_26,_26,_29.fld0.0,_29.fld0.0];
_36 = _28 ^ _28;
_37 = [_26,_29.fld0.0,_29.fld0.0,_26];
_26 = -_29.fld0.0;
_10.0 = !_7;
_24 = _27;
_25 = _1.0;
_29.fld1 = [_13,_13,_13,_13,_13];
_9 = (_8.0, _8.1, _12.2);
Goto(bb11)
}
bb11 = {
_8.2 = _5.2 * _5.2;
_28 = _36;
_29.fld1 = _23;
_16 = _10.1;
_22 = !186_u8;
_10.1 = ['\u{6e96d}','\u{84446}','\u{108e16}','\u{6e539}'];
_1.2 = _8.2 | _8.2;
_4 = _1.2 - _1.2;
_20 = -_14;
Call(_10.1 = fn5(_1, _9.0, _12, _9.0, _5, _1, _1.2, _33, _9.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_18 = 17402716212550581370_u64 << _4;
_36 = !_28;
Goto(bb13)
}
bb13 = {
_8 = (_9.0, _1.1, (*_15));
_22 = 53_u8;
_8 = _1;
_2 = _5.0 >> _1.2;
_3 = -_2;
_14 = 2438683280480590654_i64 as f32;
_35 = _22 as f64;
_10 = _8;
_17.fld0 = _29.fld0;
_10.0 = _8.0 >> _1.2;
RET = 57878493435441252568207652239407099310_u128 - 309391875043125713200938533731691094712_u128;
_8.0 = 355789978_i32 as isize;
_1.1 = ['\u{e160f}','\u{10b1a1}','\u{3b0f}','\u{16eae}'];
_23 = [_13,_13,_13,_13,_13];
_22 = (-39869962371284580741697470407959198403_i128) as u8;
(*_15) = (-8757674258357305181462058052866880445_i128) as u32;
_5.0 = _3 | _3;
RET = 3887083165323990373831100682705686613_u128;
_24 = [_22];
_4 = (-1561257668_i32) as u32;
_18 = !4261716906800067752_u64;
_5.1 = ['\u{9ca67}','\u{c8789}','\u{75c05}','\u{8172b}'];
_15 = core::ptr::addr_of!(_8.2);
_17.fld0.0 = 20819699565435292568557853702526704589_i128 as i16;
_18 = _20 as u64;
match RET {
0 => bb1,
1 => bb5,
2 => bb6,
3887083165323990373831100682705686613 => bb15,
_ => bb14
}
}
bb14 = {
_9.1 = _10.1;
_10.2 = (*_15);
_18 = _12.2 as u64;
_29.fld0 = _17.fld0;
_23 = [_13,_13,_13,_13,_13];
_7 = _14 as isize;
_17.fld0.0 = _29.fld0.0 << _9.0;
_29.fld1 = [_13,_13,_13,_13,_13];
_9.1 = ['\u{9f84f}','\u{1095b1}','\u{b86cf}','\u{a4ba2}'];
_5.2 = _10.2;
_17.fld0.0 = _29.fld0.0;
_10.2 = _17.fld0.0 as u32;
_28 = 21599_u16;
_9.0 = _1.0;
_14 = _20;
_30 = [RET,RET,RET,RET,RET,RET];
_25 = _13 as isize;
_29 = Adt52 { fld0: _17.fld0,fld1: _23 };
_8.1 = ['\u{efefc}','\u{571c2}','\u{56d05}','\u{c0d91}'];
_27 = _24;
_29.fld0 = _17.fld0;
match _28 {
21599 => bb9,
_ => bb8
}
}
bb15 = {
_8.2 = 9116641121917441683_i64 as u32;
_18 = !5739729129855811951_u64;
_32 = core::ptr::addr_of!(_5);
(*_32).0 = -_3;
(*_15) = (*_32).2 * _10.2;
_17 = Adt48 { fld0: _29.fld0 };
_44.fld0.1 = _15;
_10.0 = _18 as isize;
_7 = _3;
_29.fld1 = [_13,_13,_13,_13,_13];
_17 = Adt48 { fld0: _29.fld0 };
_43.fld0 = _29.fld0;
_8.2 = !_10.2;
_44.fld1 = 857327376210578738_usize + 7_usize;
_44.fld2.0 = 1149541708_i32 as i16;
_14 = (-95712376066206693381117383191588453607_i128) as f32;
_1 = (_2, _10.1, _8.2);
_33 = _9.1;
Goto(bb16)
}
bb16 = {
Call(_47 = dump_var(4_usize, 22_usize, Move(_22), 28_usize, Move(_28), 33_usize, Move(_33), 26_usize, Move(_26)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(4_usize, 7_usize, Move(_7), 13_usize, Move(_13), 24_usize, Move(_24), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_47 = dump_var(4_usize, 4_usize, Move(_4), 3_usize, Move(_3), 30_usize, Move(_30), 11_usize, Move(_11)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_47 = dump_var(4_usize, 5_usize, Move(_5), 10_usize, Move(_10), 48_usize, _48, 48_usize, _48), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: (isize, [char; 4], u32),mut _2: isize,mut _3: (isize, [char; 4], u32),mut _4: isize,mut _5: (isize, [char; 4], u32),mut _6: (isize, [char; 4], u32),mut _7: u32,mut _8: [char; 4],mut _9: isize) -> [char; 4] {
mir! {
type RET = [char; 4];
let _10: usize;
let _11: bool;
let _12: isize;
let _13: isize;
let _14: char;
let _15: [isize; 7];
let _16: f32;
let _17: char;
let _18: f64;
let _19: [u64; 1];
let _20: [u8; 1];
let _21: usize;
let _22: char;
let _23: char;
let _24: (isize, [char; 4], u32);
let _25: [i32; 8];
let _26: f64;
let _27: Adt45;
let _28: bool;
let _29: Adt52;
let _30: Adt52;
let _31: [isize; 7];
let _32: [i16; 4];
let _33: f64;
let _34: Adt48;
let _35: i8;
let _36: char;
let _37: u64;
let _38: Adt52;
let _39: *const u32;
let _40: usize;
let _41: (isize, [char; 4], u32);
let _42: ();
let _43: ();
{
_7 = _1.2;
_5.2 = !_6.2;
_3.1 = _8;
_5.0 = _9;
_5 = (_6.0, _3.1, _1.2);
_7 = _1.2;
RET = _8;
_10 = !16577010837264598893_usize;
_3.0 = '\u{af020}' as isize;
_3.2 = !_7;
_5.1 = ['\u{d1b3d}','\u{ca046}','\u{8ecc2}','\u{ec074}'];
_5.2 = _7 << _1.2;
RET = _6.1;
_5 = _6;
_6 = (_1.0, RET, _5.2);
_5.2 = !_3.2;
_6.2 = _7;
_6 = (_4, _8, _3.2);
RET = _6.1;
_9 = _3.0;
_9 = _6.0 | _4;
_3.1 = _5.1;
_5.1 = _8;
_3.2 = !_5.2;
Call(_6.2 = fn6(_3, _9, _1, _7, _2, _5.2, _5.2, _1, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6.0 = _4 * _1.0;
RET = ['\u{fc3a3}','\u{c8738}','\u{56a71}','\u{d4233}'];
RET = ['\u{276c2}','\u{6c077}','\u{67677}','\u{2c8ac}'];
_6.1 = ['\u{4b486}','\u{2a4bc}','\u{7bddf}','\u{822f2}'];
_5.1 = ['\u{c97ee}','\u{8880f}','\u{64dbb}','\u{91b85}'];
_7 = _5.2 - _3.2;
_1.1 = RET;
_6 = _1;
_2 = (-26828_i16) as isize;
_6 = (_4, _8, _1.2);
RET = _6.1;
_3 = (_5.0, _5.1, _7);
_6.2 = !_5.2;
_11 = !true;
_10 = (-8215785780243331849_i64) as usize;
RET = ['\u{1041ee}','\u{56117}','\u{4dbf6}','\u{108b41}'];
_10 = 9224665759983383490_usize >> _7;
RET = ['\u{1030f2}','\u{4a550}','\u{944de}','\u{852ae}'];
_6.0 = _1.0;
_11 = _3.2 > _3.2;
_12 = _3.0 - _1.0;
_13 = !_6.0;
_16 = 5908223939744372872_i64 as f32;
RET = ['\u{7a675}','\u{e2770}','\u{bdc0d}','\u{cad4d}'];
_13 = 4087208663720195249_i64 as isize;
_8 = _3.1;
Call(RET = fn18(_11, _3, _6.2, _7, _1, _3, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Goto(bb3)
}
bb3 = {
_6 = _1;
_6 = (_9, _3.1, _7);
_10 = 9590028654302228940_usize >> _1.2;
_6 = _1;
_17 = '\u{b8057}';
_6.0 = -_9;
_14 = _17;
Goto(bb4)
}
bb4 = {
_3.0 = _9;
_12 = 41_u8 as isize;
_18 = 166458491869845358818445048928549947006_i128 as f64;
_1 = (_9, _3.1, _7);
RET = [_17,_17,_14,_14];
_18 = (-756516821_i32) as f64;
_5.1 = RET;
_11 = _6.2 < _1.2;
_15 = [_3.0,_1.0,_1.0,_9,_1.0,_9,_1.0];
_14 = _17;
_7 = (-2714166629633508354_i64) as u32;
_3.0 = -_9;
_3 = _6;
_7 = !_6.2;
_21 = !_10;
Goto(bb5)
}
bb5 = {
_8 = _6.1;
RET = [_14,_14,_17,_17];
_19 = [1954346741915587273_u64];
_4 = 30421_i16 as isize;
_23 = _17;
_22 = _17;
_6.0 = !_3.0;
_12 = _6.0;
_9 = _6.0;
_19 = [7186974747214418117_u64];
Goto(bb6)
}
bb6 = {
_24.1 = [_23,_14,_14,_17];
_20 = [150_u8];
_24.0 = (-1870112939293148800_i64) as isize;
_24.1 = [_22,_23,_17,_23];
_9 = _3.0;
_19 = [17017837968338733278_u64];
_14 = _22;
_6.1 = [_23,_22,_17,_23];
_2 = 55618_u16 as isize;
_4 = (-315815059_i32) as isize;
_5.0 = -_12;
_5.1 = _8;
_15 = [_1.0,_6.0,_5.0,_5.0,_5.0,_1.0,_12];
_11 = false;
_23 = _14;
_10 = !_21;
_12 = 71_i8 as isize;
_24.2 = _1.2;
_3.0 = -_5.0;
Goto(bb7)
}
bb7 = {
_14 = _22;
_24.0 = -_3.0;
_5 = (_2, _1.1, _1.2);
_26 = _18;
_3 = (_1.0, _8, _24.2);
_10 = !_21;
_10 = 620705758016019370_u64 as usize;
_12 = _9;
_8 = [_22,_23,_23,_17];
_2 = _24.0 & _1.0;
_29.fld0 = ((-30600_i16),);
_9 = 6491782090715247531_u64 as isize;
_20 = [219_u8];
_5.1 = _6.1;
_20 = [208_u8];
_2 = _1.0 >> _1.2;
_10 = _21;
_10 = _21 ^ _21;
_24.2 = !_1.2;
_6 = (_1.0, _8, _5.2);
_3.1 = _24.1;
_10 = 14521037950822977950850912167104569299_u128 as usize;
_29.fld1 = [(-88_i8),(-64_i8),(-49_i8),59_i8,54_i8];
_29.fld0 = ((-2605_i16),);
_13 = _5.2 as isize;
_1.0 = _2 + _13;
_3.0 = !_24.0;
Goto(bb8)
}
bb8 = {
_5.2 = _1.2;
_5.0 = _24.0;
Goto(bb9)
}
bb9 = {
RET = [_17,_23,_23,_22];
_26 = (-5534257122570111969_i64) as f64;
_18 = 147_u8 as f64;
_1 = (_24.0, RET, _5.2);
_30.fld0.0 = -_29.fld0.0;
_26 = _18 + _18;
RET = [_14,_17,_14,_14];
RET = [_14,_22,_22,_17];
_26 = -_18;
_5.0 = _18 as isize;
RET = [_23,_14,_14,_17];
_30.fld1 = [(-7_i8),(-128_i8),97_i8,(-91_i8),(-110_i8)];
RET = [_14,_14,_17,_14];
_30.fld1 = [(-118_i8),92_i8,(-14_i8),50_i8,(-96_i8)];
_18 = _26;
_29 = Move(_30);
_32 = [_29.fld0.0,_29.fld0.0,_29.fld0.0,_29.fld0.0];
_5 = (_13, _1.1, _6.2);
_21 = _10;
_15 = [_5.0,_5.0,_5.0,_6.0,_5.0,_13,_13];
_21 = _10 ^ _10;
_1.0 = _2 + _13;
_29.fld1 = [55_i8,(-49_i8),115_i8,87_i8,(-90_i8)];
Goto(bb10)
}
bb10 = {
_21 = _10;
_3.1 = _1.1;
_28 = !_11;
_5 = _24;
_21 = 68_u8 as usize;
_28 = _11;
_24 = (_2, _3.1, _6.2);
_30 = Adt52 { fld0: _29.fld0,fld1: _29.fld1 };
_29.fld0 = (_30.fld0.0,);
_5 = _24;
_31 = [_1.0,_5.0,_1.0,_13,_5.0,_24.0,_13];
_6 = (_13, _1.1, _24.2);
_5 = (_12, _6.1, _1.2);
_1.2 = 82_i8 as u32;
_1.1 = [_14,_22,_23,_22];
Goto(bb11)
}
bb11 = {
_26 = _18;
_21 = !_10;
_16 = 14818356670566159574_u64 as f32;
_16 = 116036610235567111885347044322334916337_i128 as f32;
Goto(bb12)
}
bb12 = {
_1.1 = _8;
_30 = Adt52 { fld0: _29.fld0,fld1: _29.fld1 };
_7 = _24.2;
_1.0 = !_24.0;
_30.fld1 = _29.fld1;
_10 = _21 >> _24.2;
_5 = _6;
_15 = [_5.0,_24.0,_24.0,_5.0,_2,_2,_5.0];
_34.fld0 = (_29.fld0.0,);
_34.fld0.0 = _30.fld0.0 & _30.fld0.0;
_8 = [_17,_23,_22,_17];
_30 = Adt52 { fld0: _29.fld0,fld1: _29.fld1 };
_19 = [9250950114353940735_u64];
_17 = _23;
_13 = _6.0 | _3.0;
_38.fld1 = _29.fld1;
_23 = _14;
_5.0 = -_12;
_36 = _23;
_1.0 = _2 - _24.0;
_33 = _26;
Goto(bb13)
}
bb13 = {
_34.fld0.0 = _30.fld0.0;
_3.2 = !_24.2;
_34.fld0 = (_30.fld0.0,);
_38 = Adt52 { fld0: _29.fld0,fld1: _30.fld1 };
_37 = !14323132488048308524_u64;
_26 = 225_u8 as f64;
_1.2 = _24.2;
_10 = !_21;
_2 = _1.0;
_29.fld0.0 = _30.fld0.0;
_29 = Adt52 { fld0: _34.fld0,fld1: _38.fld1 };
_6 = (_3.0, _24.1, _1.2);
_38.fld0 = (_29.fld0.0,);
_1.2 = _3.2;
_29.fld0 = _34.fld0;
_31 = [_1.0,_13,_3.0,_5.0,_1.0,_1.0,_2];
_38.fld0.0 = _17 as i16;
_28 = _11;
_35 = 26_i8 << _12;
_35 = 142761398749227072730900951190510830526_u128 as i8;
_6.1 = [_23,_23,_17,_17];
_35 = _21 as i8;
_22 = _17;
_30.fld1 = [_35,_35,_35,_35,_35];
_34.fld0.0 = _38.fld0.0 - _29.fld0.0;
_30.fld0 = (_34.fld0.0,);
_6.2 = !_7;
_15 = [_24.0,_12,_24.0,_1.0,_13,_24.0,_12];
_6 = (_2, _24.1, _3.2);
_1.1 = [_17,_36,_36,_14];
_3.0 = _6.0;
Goto(bb14)
}
bb14 = {
_24.0 = _1.0;
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(5_usize, 24_usize, Move(_24), 32_usize, Move(_32), 13_usize, Move(_13), 28_usize, Move(_28)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(5_usize, 10_usize, Move(_10), 23_usize, Move(_23), 35_usize, Move(_35), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(5_usize, 5_usize, Move(_5), 12_usize, Move(_12), 7_usize, Move(_7), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(5_usize, 36_usize, Move(_36), 22_usize, Move(_22), 43_usize, _43, 43_usize, _43), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: (isize, [char; 4], u32),mut _2: isize,mut _3: (isize, [char; 4], u32),mut _4: u32,mut _5: isize,mut _6: u32,mut _7: u32,mut _8: (isize, [char; 4], u32),mut _9: u32) -> u32 {
mir! {
type RET = u32;
let _10: f64;
let _11: Adt57;
let _12: &'static u16;
let _13: [i8; 5];
let _14: f32;
let _15: f32;
let _16: Adt43;
let _17: f64;
let _18: ();
let _19: ();
{
_1.2 = !_9;
_2 = _3.0 & _5;
_7 = _4;
_8 = (_5, _1.1, _4);
_9 = _6;
RET = 162605476338409647500977200564501224700_u128 as u32;
_7 = 45768_u16 as u32;
_8.0 = _3.0 - _2;
RET = !_1.2;
_9 = (-8051254301586272281409508148089725631_i128) as u32;
_10 = 747236346244661808_u64 as f64;
_1.0 = -_8.0;
_1.2 = !_6;
_8.0 = -_2;
_8 = (_1.0, _3.1, _1.2);
RET = 7258_i16 as u32;
_7 = _3.2 ^ _3.2;
_8.2 = 220215890652199201044787620460535895530_u128 as u32;
_1.2 = !_4;
Goto(bb1)
}
bb1 = {
Call(_3.1 = fn7(_8.0, _8.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9 = true as u32;
_4 = 2078794782245052282_usize as u32;
_6 = _7 & _1.2;
_14 = 64474_u16 as f32;
_1.1 = ['\u{99aaa}','\u{a55c6}','\u{c2f81}','\u{35ea7}'];
_1.0 = _8.0 << _6;
_5 = _2 | _1.0;
_3.2 = 143702061628179709155785820276824240768_i128 as u32;
_3.1 = ['\u{86dcf}','\u{54e8d}','\u{357d9}','\u{ef6c2}'];
RET = !_7;
_3.0 = !_1.0;
_10 = (-111021740381328495141204815539948816620_i128) as f64;
Goto(bb3)
}
bb3 = {
Call(_18 = dump_var(6_usize, 6_usize, Move(_6), 2_usize, Move(_2), 4_usize, Move(_4), 3_usize, Move(_3)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: isize,mut _2: isize) -> [char; 4] {
mir! {
type RET = [char; 4];
let _3: i128;
let _4: isize;
let _5: u8;
let _6: (f32, *const (u32, *const u32, u32, i64), [char; 4]);
let _7: i16;
let _8: Adt55;
let _9: [char; 7];
let _10: bool;
let _11: (i16,);
let _12: bool;
let _13: [char; 4];
let _14: u16;
let _15: Adt45;
let _16: bool;
let _17: isize;
let _18: bool;
let _19: u64;
let _20: [u32; 1];
let _21: isize;
let _22: [i16; 4];
let _23: [i16; 4];
let _24: bool;
let _25: isize;
let _26: [isize; 7];
let _27: [i16; 4];
let _28: Adt46;
let _29: [u128; 6];
let _30: isize;
let _31: ();
let _32: ();
{
_1 = 165_u8 as isize;
_1 = _2 << _2;
RET = ['\u{6410c}','\u{23887}','\u{f0930}','\u{cd9ac}'];
RET = ['\u{f29ff}','\u{352fb}','\u{22667}','\u{10b16c}'];
RET = ['\u{45042}','\u{27345}','\u{e3f7c}','\u{afefa}'];
_2 = _1 ^ _1;
_2 = _1 * _1;
_2 = -_1;
RET = ['\u{c8218}','\u{1c76c}','\u{87536}','\u{c4a7f}'];
RET = ['\u{10d0a7}','\u{d210b}','\u{f9cab}','\u{6b2d9}'];
_3 = -132687013404257370684872763613170801749_i128;
_1 = _2 - _2;
_1 = _2;
_1 = _2;
_3 = 514671847_i32 as i128;
_3 = 68819364036015563097377816572296151948_i128 * (-66872592866336108636658963986133028614_i128);
_4 = _2 & _1;
RET = ['\u{84304}','\u{7b47b}','\u{449f2}','\u{b90b3}'];
Goto(bb1)
}
bb1 = {
RET = ['\u{3b335}','\u{2bc50}','\u{2682b}','\u{b0515}'];
_4 = 4250377889387576645_i64 as isize;
RET = ['\u{5dba9}','\u{543bc}','\u{1a5f4}','\u{5e31e}'];
_1 = _2 >> _2;
_1 = _2 ^ _2;
RET = ['\u{9b3bb}','\u{22b4d}','\u{c421e}','\u{ad2ef}'];
RET = ['\u{baf65}','\u{7e9b3}','\u{105030}','\u{ed476}'];
RET = ['\u{69d96}','\u{b91a3}','\u{8951d}','\u{68a2a}'];
_5 = 103_u8;
_3 = _5 as i128;
_4 = _1 ^ _1;
_1 = _2 | _2;
_1 = -_4;
match _5 {
0 => bb2,
103 => bb4,
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
_1 = -_4;
RET = ['\u{d9603}','\u{2ddd1}','\u{b7db2}','\u{d8877}'];
_6.0 = 3150475591_u32 as f32;
_3 = 262286752615871079255961933522894948957_u128 as i128;
_2 = -_4;
_5 = 30_u8 * 101_u8;
_1 = 2114187206_i32 as isize;
_6.0 = 99_i8 as f32;
_7 = 14441_i16;
_5 = 6_u8;
_4 = _2 ^ _2;
Call(RET = fn8(_4, _4, _2, _4, _4, _2, _4, _2, _4, _4, _2, _2, _4, _4, _4), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_6.2 = ['\u{10f087}','\u{60234}','\u{fd8a4}','\u{f6d5}'];
_6.0 = _2 as f32;
_1 = _4 | _2;
_2 = _4 << _1;
_11 = (_7,);
_5 = 248_u8 * 49_u8;
_6.0 = _7 as f32;
RET = ['\u{104477}','\u{87f15}','\u{2d812}','\u{d6157}'];
_4 = _1;
_10 = _4 < _1;
_6.2 = RET;
_9 = ['\u{d7742}','\u{848aa}','\u{37cca}','\u{8b102}','\u{fa998}','\u{6e05e}','\u{fda54}'];
_9 = ['\u{7476b}','\u{3f8a0}','\u{81870}','\u{5225c}','\u{d45a6}','\u{9ebb0}','\u{b0dea}'];
RET = ['\u{9202f}','\u{a1a28}','\u{7569c}','\u{48723}'];
RET = ['\u{7a2e4}','\u{ab1a5}','\u{88563}','\u{8ec26}'];
_13 = ['\u{440a2}','\u{b4108}','\u{a6f9d}','\u{48969}'];
_11 = (_7,);
_11 = (_7,);
_10 = false & true;
Goto(bb6)
}
bb6 = {
_12 = _10;
_6.0 = 2303916382_u32 as f32;
_11.0 = _7;
_3 = 85543619837763461264294917056807978228_i128 * 108621623832384306298862979812628119673_i128;
_3 = (-7616955204272073343699416507277300456_i128);
_5 = 184_u8 >> _1;
_10 = _12;
RET = _6.2;
_13 = ['\u{84521}','\u{b9054}','\u{9982a}','\u{960c}'];
_11 = (_7,);
_1 = _2;
_5 = 7_u8 * 150_u8;
_3 = 7_i8 as i128;
_16 = _10 & _12;
_12 = _2 > _2;
_11.0 = _7;
_11 = (_7,);
_3 = -27983907967454938162772388294020448986_i128;
_1 = -_2;
_13 = ['\u{ec24f}','\u{87257}','\u{10c3bf}','\u{9031e}'];
_16 = _1 != _1;
_14 = !45907_u16;
_14 = 40187_u16 * 37575_u16;
_11 = (_7,);
_17 = _1 + _2;
RET = ['\u{8cffa}','\u{d8e37}','\u{a4aeb}','\u{28963}'];
_5 = 142_u8 * 110_u8;
_16 = _2 == _2;
match _11.0 {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb7,
4 => bb8,
5 => bb9,
14441 => bb11,
_ => bb10
}
}
bb7 = {
_6.2 = ['\u{10f087}','\u{60234}','\u{fd8a4}','\u{f6d5}'];
_6.0 = _2 as f32;
_1 = _4 | _2;
_2 = _4 << _1;
_11 = (_7,);
_5 = 248_u8 * 49_u8;
_6.0 = _7 as f32;
RET = ['\u{104477}','\u{87f15}','\u{2d812}','\u{d6157}'];
_4 = _1;
_10 = _4 < _1;
_6.2 = RET;
_9 = ['\u{d7742}','\u{848aa}','\u{37cca}','\u{8b102}','\u{fa998}','\u{6e05e}','\u{fda54}'];
_9 = ['\u{7476b}','\u{3f8a0}','\u{81870}','\u{5225c}','\u{d45a6}','\u{9ebb0}','\u{b0dea}'];
RET = ['\u{9202f}','\u{a1a28}','\u{7569c}','\u{48723}'];
RET = ['\u{7a2e4}','\u{ab1a5}','\u{88563}','\u{8ec26}'];
_13 = ['\u{440a2}','\u{b4108}','\u{a6f9d}','\u{48969}'];
_11 = (_7,);
_11 = (_7,);
_10 = false & true;
Goto(bb6)
}
bb8 = {
RET = ['\u{3b335}','\u{2bc50}','\u{2682b}','\u{b0515}'];
_4 = 4250377889387576645_i64 as isize;
RET = ['\u{5dba9}','\u{543bc}','\u{1a5f4}','\u{5e31e}'];
_1 = _2 >> _2;
_1 = _2 ^ _2;
RET = ['\u{9b3bb}','\u{22b4d}','\u{c421e}','\u{ad2ef}'];
RET = ['\u{baf65}','\u{7e9b3}','\u{105030}','\u{ed476}'];
RET = ['\u{69d96}','\u{b91a3}','\u{8951d}','\u{68a2a}'];
_5 = 103_u8;
_3 = _5 as i128;
_4 = _1 ^ _1;
_1 = _2 | _2;
_1 = -_4;
match _5 {
0 => bb2,
103 => bb4,
_ => bb3
}
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_13 = ['\u{4886c}','\u{48465}','\u{d94ee}','\u{da829}'];
RET = _6.2;
_18 = _16;
_4 = _2;
RET = ['\u{9e2b7}','\u{52e1f}','\u{a5359}','\u{e44ff}'];
_4 = _17 >> _1;
_13 = ['\u{64f4}','\u{6b3ad}','\u{6f8a5}','\u{7010e}'];
_12 = _18;
_19 = !13704234318380181835_u64;
_11.0 = -_7;
_13 = ['\u{d2639}','\u{1064fd}','\u{a9e4d}','\u{40111}'];
_12 = _18 >= _16;
_13 = _6.2;
_14 = (-6479208886989285159_i64) as u16;
Goto(bb12)
}
bb12 = {
_6.2 = ['\u{d0cfd}','\u{bf40b}','\u{bc935}','\u{23558}'];
_16 = !_12;
_2 = _17 - _1;
_6.0 = 2918398217_u32 as f32;
_17 = !_1;
_2 = _4 ^ _17;
_6.0 = 4_usize as f32;
RET = ['\u{129ee}','\u{15674}','\u{3c10a}','\u{243ac}'];
_4 = _17;
_18 = !_12;
_21 = _4 * _4;
_18 = _2 == _1;
_7 = (-646626113_i32) as i16;
_18 = _16;
_7 = !_11.0;
_2 = _4;
_11 = (_7,);
_14 = !22629_u16;
_17 = _1;
_13 = ['\u{7e702}','\u{25baa}','\u{958a9}','\u{7fe42}'];
_11 = (_7,);
_12 = !_18;
_11.0 = _7;
_13 = RET;
_12 = _18;
_20 = [1290519282_u32];
Goto(bb13)
}
bb13 = {
_6.0 = (-6_i8) as f32;
_24 = _12 | _18;
RET = _6.2;
_27 = [_7,_11.0,_11.0,_11.0];
_23 = _27;
_18 = _16;
_4 = _17 >> _17;
_17 = _11.0 as isize;
_11.0 = _14 as i16;
_28.fld0.3 = (-876715801_i32) as i64;
_28.fld2 = (_7,);
_28.fld0.0 = 16530602736224192763_usize as u32;
_21 = _4 ^ _1;
_9 = ['\u{fd1b}','\u{4b960}','\u{72d5c}','\u{ee40f}','\u{20b88}','\u{307}','\u{fddce}'];
_21 = -_2;
_28.fld5 = [_5];
_7 = _11.0;
_6.1 = core::ptr::addr_of!(_28.fld0);
Goto(bb14)
}
bb14 = {
_22 = [_11.0,_7,_7,_7];
_28.fld2 = (_11.0,);
_22 = [_7,_7,_11.0,_7];
_6.2 = ['\u{dec87}','\u{9e40c}','\u{c756f}','\u{82629}'];
_28.fld6 = core::ptr::addr_of_mut!(_28.fld1);
_28.fld5 = [_5];
_24 = _16 != _18;
RET = ['\u{488dd}','\u{108c0b}','\u{7e96f}','\u{4f8d1}'];
_28.fld4 = [_21,_21,_1,_4,_1,_4,_1];
_24 = !_18;
_11 = _28.fld2;
_18 = _24;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(7_usize, 23_usize, Move(_23), 16_usize, Move(_16), 19_usize, Move(_19), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(7_usize, 5_usize, Move(_5), 27_usize, Move(_27), 12_usize, Move(_12), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(7_usize, 11_usize, Move(_11), 2_usize, Move(_2), 17_usize, Move(_17), 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize,mut _15: isize) -> [char; 4] {
mir! {
type RET = [char; 4];
let _16: f32;
let _17: [u128; 6];
let _18: isize;
let _19: (isize, [char; 4], u32);
let _20: *mut i128;
let _21: Adt57;
let _22: f32;
let _23: [i8; 5];
let _24: i64;
let _25: Adt49;
let _26: usize;
let _27: (isize, [char; 4], u32);
let _28: bool;
let _29: isize;
let _30: Adt56;
let _31: (i16,);
let _32: (isize, [char; 4], u32);
let _33: i16;
let _34: bool;
let _35: u64;
let _36: f32;
let _37: isize;
let _38: i32;
let _39: [bool; 5];
let _40: &'static u16;
let _41: *const (isize, [char; 4], u32);
let _42: u16;
let _43: isize;
let _44: isize;
let _45: ();
let _46: ();
{
_6 = 100737575345894211478653773166593297819_i128 as isize;
RET = ['\u{44285}','\u{256dd}','\u{702f}','\u{99c7}'];
_4 = false as isize;
_5 = _8;
_3 = _14 ^ _15;
_2 = _10 & _13;
_3 = 2017324451_u32 as isize;
_5 = _2;
_15 = _13 * _2;
_4 = -_7;
_15 = !_10;
_2 = 66120944693161111768187787738479984066_u128 as isize;
_13 = -_15;
_5 = -_15;
_6 = -_1;
_16 = _8 as f32;
_16 = (-17911361712650405081633023075486951193_i128) as f32;
RET = ['\u{9d893}','\u{68f2}','\u{a7c60}','\u{baa1}'];
_18 = _4 - _6;
_13 = _7 ^ _8;
Call(_18 = fn9(_15, _10, _12, _7, _10), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = _11;
_13 = _16 as isize;
_4 = -_9;
_13 = -_12;
_17 = [229772678585103574176304514173172431006_u128,339716523734071556897055648804524812666_u128,296953025651301932316922119002004178268_u128,256769906986138078540677713303791868329_u128,96751284168931803879495097108996751022_u128,47918984991475000010395415495878713387_u128];
_19 = (_10, RET, 2262653934_u32);
_15 = !_8;
_17 = [221000989258171837435137171987556584668_u128,192425851640748569913577235456128851804_u128,74343129264663638690464020284605628100_u128,125205717338433692664305923241269384208_u128,287086346026324453191402910221612851952_u128,272155542506778669667667468621757386448_u128];
_19.0 = _13 * _12;
Goto(bb2)
}
bb2 = {
RET = ['\u{c495f}','\u{f5cac}','\u{2de0f}','\u{f03c4}'];
_8 = _7 * _7;
_11 = _4 * _14;
_17 = [247737374151853491991847582592137425616_u128,298010551642411721403577085304161924321_u128,277842020732062712386638210291967861589_u128,151608876388692716012910228214499236190_u128,151667229249270594743967039523612706660_u128,263326682613241380089305045785413825501_u128];
_19.0 = _6 * _13;
_19.0 = _1 << _4;
_11 = -_7;
_22 = -_16;
_1 = _5;
_7 = _9;
_8 = 61498_u16 as isize;
_17 = [134290354614750989933096063793795679587_u128,20750047588605366936554293410053147945_u128,293802585041703696117090845563694864551_u128,48072454393356672408050965123701523988_u128,86639444505504271887518722571919312997_u128,282631532422768814027471927597803437145_u128];
_11 = _10;
_6 = 4686624695488346165_i64 as isize;
_22 = _16 - _16;
_15 = (-143100115_i32) as isize;
_22 = 42984_u16 as f32;
_6 = !_9;
_4 = 6180_i16 as isize;
_5 = 5846046302305517235_u64 as isize;
_16 = _22;
_2 = -_7;
_15 = -_12;
_6 = !_4;
_24 = (-56_i8) as i64;
Call(_19.2 = fn10(_13, _7, _1, _19.1, _14, _2, _9, _12, _9, _11, _10), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_12 = _10 + _1;
_14 = _19.0 >> _12;
_19.1 = ['\u{10ffa9}','\u{8cf8d}','\u{842f2}','\u{1077eb}'];
_10 = -_11;
_15 = _24 as isize;
Call(_19.1 = fn11(_13, _13, _2, _9, _19.0, _2, _14, _19.0, _7, _9, _7, _11, _11, _2, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_8 = 7368072633236441085_u64 as isize;
_19.0 = 95_i8 as isize;
_3 = _1;
_9 = -_12;
_23 = [25_i8,(-13_i8),27_i8,64_i8,(-81_i8)];
RET = ['\u{3250a}','\u{8bf2f}','\u{859bd}','\u{3c9b2}'];
_24 = (-1527287520567629091_i64);
_3 = -_14;
_7 = -_10;
_19.1 = RET;
_13 = _10 >> _9;
_10 = _12 & _14;
_23 = [(-59_i8),29_i8,16_i8,66_i8,(-43_i8)];
_5 = -_2;
_26 = 3_usize << _3;
_19.2 = 58608_u16 as u32;
_19.1 = RET;
_7 = '\u{f76d4}' as isize;
_27.0 = _3 >> _5;
_9 = _12;
_24 = 18_i8 as i64;
_10 = 3090_i16 as isize;
_8 = -_13;
_19.1 = RET;
_27.2 = false as u32;
Call(_7 = core::intrinsics::bswap(_13), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_13 = _27.0;
_3 = 150856711003283220520188166875854085305_i128 as isize;
_22 = -_16;
_8 = '\u{f1a52}' as isize;
_27 = _19;
_27.2 = _19.2 + _19.2;
_2 = _27.2 as isize;
_17 = [68698485416295799061663629900898164087_u128,248373313964461898249703386811025049280_u128,181694751391215285957653141012401508351_u128,44905126662804033285295531185651844279_u128,83056152787194434962144237333755747612_u128,8780259892978439539263074827196372021_u128];
_4 = _11 & _12;
_13 = _4;
_27.2 = _24 as u32;
_10 = _26 as isize;
_6 = _11 * _18;
_19.0 = _6 + _10;
_7 = -_6;
_29 = !_13;
_27.0 = _16 as isize;
_31.0 = -12320_i16;
_15 = !_11;
_11 = _29;
_11 = _12 >> _19.0;
_28 = true | true;
Call(_7 = fn17(_1, _13), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_4 = 1125566160_i32 as isize;
_3 = _26 as isize;
RET = _19.1;
RET = ['\u{e2191}','\u{e0f6f}','\u{1c006}','\u{e5aba}'];
_11 = _14 >> _13;
_9 = -_29;
_3 = _10 - _1;
_32.1 = ['\u{1acce}','\u{bc41b}','\u{6c420}','\u{91861}'];
_24 = -(-115473914814970244_i64);
_19.1 = RET;
_32.2 = _27.2 ^ _27.2;
_32.0 = !_7;
_27.0 = _18;
_19.2 = !_27.2;
_22 = _16 + _16;
_33 = _31.0;
_29 = _32.2 as isize;
_2 = -_13;
_32.1 = ['\u{3225e}','\u{d7b47}','\u{e2073}','\u{bce25}'];
_19.2 = _32.2;
Goto(bb7)
}
bb7 = {
_29 = _14 * _14;
_3 = _19.0;
_14 = !_6;
_31.0 = !_33;
_9 = _2 + _6;
_27 = (_10, _19.1, _32.2);
_13 = _15;
_11 = 2653816882226520587_u64 as isize;
_9 = _12 << _3;
_15 = !_18;
_27.2 = _19.2;
_5 = _14;
_31.0 = _33;
_33 = _31.0;
_24 = !(-6179993041467453655_i64);
_36 = _16;
_10 = _27.0;
_2 = (-136819488_i32) as isize;
_17 = [86822385300629028743987717834837910931_u128,206424024179473955181678515837082259737_u128,277847244032250090914234582145608543514_u128,111628782963015995697732597013013288867_u128,153189529234462552828815513883850950037_u128,160682247782248152267648665838630477509_u128];
_9 = _19.0;
_36 = 58568952_i32 as f32;
_34 = !_28;
_17 = [291254026430610494972168892338127781956_u128,246680821328549077904892338403475289993_u128,42258300378993145453935369462651150037_u128,292583999176683884262088505887277987695_u128,181454821450226896779009781636883182717_u128,152937348496481105907666166597695295395_u128];
Goto(bb8)
}
bb8 = {
_27 = (_12, RET, _19.2);
_27.1 = ['\u{472d5}','\u{a9a1e}','\u{ad243}','\u{587d5}'];
_2 = _13 | _19.0;
_5 = _12;
_32.0 = 80_i8 as isize;
_4 = -_6;
_29 = (-1564009744_i32) as isize;
_17 = [186324801692717259659549604966997081489_u128,143181935322654812413639163643309035767_u128,111258199198329505200413722082515670915_u128,275019366208010910233451526524206072919_u128,66311752064043006958299141550425621302_u128,117475295065053768503260000840252837345_u128];
_27 = _19;
_29 = _1 * _15;
_15 = _7;
_9 = _10;
_27.0 = -_18;
Goto(bb9)
}
bb9 = {
_10 = _19.0;
_19.1 = ['\u{65e45}','\u{fce56}','\u{b9504}','\u{257a7}'];
_33 = _31.0;
Goto(bb10)
}
bb10 = {
_12 = !_5;
_16 = -_36;
_37 = -_14;
_12 = -_18;
Goto(bb11)
}
bb11 = {
_38 = 30954242727841289403714206176402351643_u128 as i32;
_13 = -_37;
_5 = -_9;
_43 = _4 << _14;
_10 = _34 as isize;
_29 = _37;
_39 = [_28,_34,_28,_34,_28];
_36 = _22 - _16;
_15 = -_14;
_27.0 = _38 as isize;
_38 = 1537905803_i32;
_5 = !_43;
_31.0 = _7 as i16;
_12 = -_43;
_32.2 = _26 as u32;
_9 = _7;
_32.1 = ['\u{ed064}','\u{5fcbe}','\u{15b5f}','\u{9ea1d}'];
_9 = -_2;
_36 = _38 as f32;
_28 = _34 ^ _34;
_23 = [18_i8,104_i8,(-55_i8),(-24_i8),(-15_i8)];
match _38 {
0 => bb8,
1 => bb10,
2 => bb3,
3 => bb12,
4 => bb13,
5 => bb14,
1537905803 => bb16,
_ => bb15
}
}
bb12 = {
_12 = !_5;
_16 = -_36;
_37 = -_14;
_12 = -_18;
Goto(bb11)
}
bb13 = {
_13 = _27.0;
_3 = 150856711003283220520188166875854085305_i128 as isize;
_22 = -_16;
_8 = '\u{f1a52}' as isize;
_27 = _19;
_27.2 = _19.2 + _19.2;
_2 = _27.2 as isize;
_17 = [68698485416295799061663629900898164087_u128,248373313964461898249703386811025049280_u128,181694751391215285957653141012401508351_u128,44905126662804033285295531185651844279_u128,83056152787194434962144237333755747612_u128,8780259892978439539263074827196372021_u128];
_4 = _11 & _12;
_13 = _4;
_27.2 = _24 as u32;
_10 = _26 as isize;
_6 = _11 * _18;
_19.0 = _6 + _10;
_7 = -_6;
_29 = !_13;
_27.0 = _16 as isize;
_31.0 = -12320_i16;
_15 = !_11;
_11 = _29;
_11 = _12 >> _19.0;
_28 = true | true;
Call(_7 = fn17(_1, _13), ReturnTo(bb6), UnwindUnreachable())
}
bb14 = {
_8 = _11;
_13 = _16 as isize;
_4 = -_9;
_13 = -_12;
_17 = [229772678585103574176304514173172431006_u128,339716523734071556897055648804524812666_u128,296953025651301932316922119002004178268_u128,256769906986138078540677713303791868329_u128,96751284168931803879495097108996751022_u128,47918984991475000010395415495878713387_u128];
_19 = (_10, RET, 2262653934_u32);
_15 = !_8;
_17 = [221000989258171837435137171987556584668_u128,192425851640748569913577235456128851804_u128,74343129264663638690464020284605628100_u128,125205717338433692664305923241269384208_u128,287086346026324453191402910221612851952_u128,272155542506778669667667468621757386448_u128];
_19.0 = _13 * _12;
Goto(bb2)
}
bb15 = {
_8 = 7368072633236441085_u64 as isize;
_19.0 = 95_i8 as isize;
_3 = _1;
_9 = -_12;
_23 = [25_i8,(-13_i8),27_i8,64_i8,(-81_i8)];
RET = ['\u{3250a}','\u{8bf2f}','\u{859bd}','\u{3c9b2}'];
_24 = (-1527287520567629091_i64);
_3 = -_14;
_7 = -_10;
_19.1 = RET;
_13 = _10 >> _9;
_10 = _12 & _14;
_23 = [(-59_i8),29_i8,16_i8,66_i8,(-43_i8)];
_5 = -_2;
_26 = 3_usize << _3;
_19.2 = 58608_u16 as u32;
_19.1 = RET;
_7 = '\u{f76d4}' as isize;
_27.0 = _3 >> _5;
_9 = _12;
_24 = 18_i8 as i64;
_10 = 3090_i16 as isize;
_8 = -_13;
_19.1 = RET;
_27.2 = false as u32;
Call(_7 = core::intrinsics::bswap(_13), ReturnTo(bb5), UnwindUnreachable())
}
bb16 = {
_3 = _13;
_11 = -_7;
_27.0 = -_13;
_38 = '\u{6aa53}' as i32;
_3 = _19.0 - _6;
_32 = (_29, RET, _19.2);
_39 = [_34,_28,_28,_28,_28];
_15 = _10 ^ _18;
_15 = !_11;
Goto(bb17)
}
bb17 = {
Call(_45 = dump_var(8_usize, 27_usize, Move(_27), 10_usize, Move(_10), 32_usize, Move(_32), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(8_usize, 31_usize, Move(_31), 28_usize, Move(_28), 24_usize, Move(_24), 38_usize, Move(_38)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_45 = dump_var(8_usize, 7_usize, Move(_7), 19_usize, Move(_19), 18_usize, Move(_18), 2_usize, Move(_2)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_45 = dump_var(8_usize, 8_usize, Move(_8), 5_usize, Move(_5), 1_usize, Move(_1), 23_usize, Move(_23)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize) -> isize {
mir! {
type RET = isize;
let _6: [char; 7];
let _7: [u32; 1];
let _8: ();
let _9: ();
{
_5 = -_2;
RET = _4;
_2 = !_5;
_5 = -_2;
_7 = [4254558526_u32];
_7 = [1196656648_u32];
_2 = _1;
Goto(bb1)
}
bb1 = {
Call(_8 = dump_var(9_usize, 4_usize, Move(_4), 3_usize, Move(_3), 5_usize, Move(_5), 9_usize, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: [char; 4],mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize) -> u32 {
mir! {
type RET = u32;
let _12: Adt48;
let _13: Adt47;
let _14: Adt43;
let _15: *mut *mut i128;
let _16: isize;
let _17: f64;
let _18: Adt56;
let _19: Adt53;
let _20: [u64; 1];
let _21: [i8; 5];
let _22: i64;
let _23: u64;
let _24: [u8; 1];
let _25: *mut i128;
let _26: &'static u16;
let _27: Adt57;
let _28: i16;
let _29: bool;
let _30: Adt56;
let _31: char;
let _32: Adt54;
let _33: f64;
let _34: u16;
let _35: u64;
let _36: bool;
let _37: *mut *mut i128;
let _38: f64;
let _39: i128;
let _40: ();
let _41: ();
{
_11 = _1 | _7;
RET = 130_u8 as u32;
_9 = (-71_i8) as isize;
_1 = _7;
_7 = !_3;
_5 = '\u{829bb}' as isize;
_4 = ['\u{c6c57}','\u{e1e6b}','\u{fda1b}','\u{3f7f8}'];
_11 = _10;
_3 = _6;
_4 = ['\u{e4a29}','\u{1227e}','\u{e334d}','\u{2fe25}'];
_11 = _2 | _3;
_10 = '\u{44afa}' as isize;
RET = 295611787_u32;
_12.fld0 = (23269_i16,);
_8 = _6 << _6;
_3 = 191_u8 as isize;
_5 = 226890312952968229644449641968017966463_u128 as isize;
_7 = !_6;
_2 = !_6;
_11 = _6;
_8 = _11;
_6 = -_8;
_10 = !_8;
_3 = !_8;
_12.fld0.0 = (-15472_i16);
RET = 2019311336_u32;
Call(_2 = core::intrinsics::bswap(_10), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 542441347_u32 & 83327306_u32;
_4 = ['\u{4484c}','\u{6e988}','\u{29917}','\u{99327}'];
_2 = _7;
_11 = 853412984_i32 as isize;
_17 = RET as f64;
_12.fld0 = (15435_i16,);
_2 = (-67050977825018628606869045173678189004_i128) as isize;
_17 = 21_i8 as f64;
_8 = 4_usize as isize;
_6 = false as isize;
_4 = ['\u{3ef3a}','\u{f0f30}','\u{b6819}','\u{20e1c}'];
_7 = _1 & _3;
_11 = true as isize;
_1 = _7 ^ _7;
_8 = 324863783597435281324950758811552278768_u128 as isize;
_12.fld0 = ((-4879_i16),);
RET = 3823894773_u32 - 2072949788_u32;
_16 = _7;
_6 = RET as isize;
_11 = _7;
_1 = _16;
_12.fld0.0 = (-19951_i16);
Goto(bb2)
}
bb2 = {
_16 = _3;
_10 = _16 + _11;
_3 = !_11;
_7 = _16;
_20 = [8047213435680799227_u64];
RET = !3738871199_u32;
_6 = _11 - _16;
_3 = _7 & _7;
_4 = ['\u{eb604}','\u{9f27b}','\u{ba0df}','\u{cfec5}'];
_16 = 1853684013_i32 as isize;
RET = !1027675679_u32;
_23 = 13635605307811973043_u64;
_10 = 31029168113923514885140550321178120322_i128 as isize;
_7 = _11;
_17 = 50708299681409487458638139188087247916_u128 as f64;
_1 = _23 as isize;
_24 = [141_u8];
_17 = (-2658036590847403743_i64) as f64;
_16 = _6 | _3;
_22 = !(-6259309365611581347_i64);
_17 = _22 as f64;
_3 = -_6;
_6 = 7054135133255725932311503077339048212_i128 as isize;
_2 = 952449759_i32 as isize;
match _23 {
0 => bb1,
1 => bb3,
13635605307811973043 => bb5,
_ => bb4
}
}
bb3 = {
RET = 542441347_u32 & 83327306_u32;
_4 = ['\u{4484c}','\u{6e988}','\u{29917}','\u{99327}'];
_2 = _7;
_11 = 853412984_i32 as isize;
_17 = RET as f64;
_12.fld0 = (15435_i16,);
_2 = (-67050977825018628606869045173678189004_i128) as isize;
_17 = 21_i8 as f64;
_8 = 4_usize as isize;
_6 = false as isize;
_4 = ['\u{3ef3a}','\u{f0f30}','\u{b6819}','\u{20e1c}'];
_7 = _1 & _3;
_11 = true as isize;
_1 = _7 ^ _7;
_8 = 324863783597435281324950758811552278768_u128 as isize;
_12.fld0 = ((-4879_i16),);
RET = 3823894773_u32 - 2072949788_u32;
_16 = _7;
_6 = RET as isize;
_11 = _7;
_1 = _16;
_12.fld0.0 = (-19951_i16);
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
_15 = core::ptr::addr_of_mut!(_25);
_5 = false as isize;
_24 = [70_u8];
_17 = (-108_i8) as f64;
_24 = [205_u8];
_6 = _7;
_20 = [_23];
_12.fld0.0 = -(-32297_i16);
_8 = _16;
RET = 2997858978_u32 + 3694772961_u32;
RET = '\u{a300}' as u32;
_3 = 208_u8 as isize;
Goto(bb6)
}
bb6 = {
_28 = 111801569543810068269580491176858303426_i128 as i16;
_28 = RET as i16;
_2 = _6 << _16;
_20 = [_23];
match _23 {
0 => bb7,
1 => bb8,
2 => bb9,
13635605307811973043 => bb11,
_ => bb10
}
}
bb7 = {
_15 = core::ptr::addr_of_mut!(_25);
_5 = false as isize;
_24 = [70_u8];
_17 = (-108_i8) as f64;
_24 = [205_u8];
_6 = _7;
_20 = [_23];
_12.fld0.0 = -(-32297_i16);
_8 = _16;
RET = 2997858978_u32 + 3694772961_u32;
RET = '\u{a300}' as u32;
_3 = 208_u8 as isize;
Goto(bb6)
}
bb8 = {
Return()
}
bb9 = {
RET = 542441347_u32 & 83327306_u32;
_4 = ['\u{4484c}','\u{6e988}','\u{29917}','\u{99327}'];
_2 = _7;
_11 = 853412984_i32 as isize;
_17 = RET as f64;
_12.fld0 = (15435_i16,);
_2 = (-67050977825018628606869045173678189004_i128) as isize;
_17 = 21_i8 as f64;
_8 = 4_usize as isize;
_6 = false as isize;
_4 = ['\u{3ef3a}','\u{f0f30}','\u{b6819}','\u{20e1c}'];
_7 = _1 & _3;
_11 = true as isize;
_1 = _7 ^ _7;
_8 = 324863783597435281324950758811552278768_u128 as isize;
_12.fld0 = ((-4879_i16),);
RET = 3823894773_u32 - 2072949788_u32;
_16 = _7;
_6 = RET as isize;
_11 = _7;
_1 = _16;
_12.fld0.0 = (-19951_i16);
Goto(bb2)
}
bb10 = {
_16 = _3;
_10 = _16 + _11;
_3 = !_11;
_7 = _16;
_20 = [8047213435680799227_u64];
RET = !3738871199_u32;
_6 = _11 - _16;
_3 = _7 & _7;
_4 = ['\u{eb604}','\u{9f27b}','\u{ba0df}','\u{cfec5}'];
_16 = 1853684013_i32 as isize;
RET = !1027675679_u32;
_23 = 13635605307811973043_u64;
_10 = 31029168113923514885140550321178120322_i128 as isize;
_7 = _11;
_17 = 50708299681409487458638139188087247916_u128 as f64;
_1 = _23 as isize;
_24 = [141_u8];
_17 = (-2658036590847403743_i64) as f64;
_16 = _6 | _3;
_22 = !(-6259309365611581347_i64);
_17 = _22 as f64;
_3 = -_6;
_6 = 7054135133255725932311503077339048212_i128 as isize;
_2 = 952449759_i32 as isize;
match _23 {
0 => bb1,
1 => bb3,
13635605307811973043 => bb5,
_ => bb4
}
}
bb11 = {
_7 = (-1837599630_i32) as isize;
_24 = [89_u8];
_22 = -4154013835917094849_i64;
_20 = [_23];
_28 = _12.fld0.0 ^ _12.fld0.0;
_17 = RET as f64;
_1 = _16 << _16;
_11 = _8 << _2;
_3 = _2 + _16;
_23 = _17 as u64;
Goto(bb12)
}
bb12 = {
_8 = _3 ^ _11;
RET = (-54_i8) as u32;
_24 = [116_u8];
_29 = true;
_6 = _3 - _2;
_21 = [(-38_i8),5_i8,(-84_i8),(-43_i8),(-126_i8)];
_17 = _11 as f64;
_24 = [67_u8];
_9 = _17 as isize;
_29 = true;
_5 = !_9;
_1 = _2;
_21 = [67_i8,126_i8,75_i8,(-28_i8),(-56_i8)];
_9 = _1 | _16;
_31 = '\u{20502}';
RET = !2667596067_u32;
_20 = [_23];
_10 = _2 + _1;
_22 = !(-6422313171406412517_i64);
_22 = 3741310170406920736_i64;
_23 = 11061295766030574874_u64 | 11991428279209994814_u64;
_3 = _31 as isize;
_28 = !_12.fld0.0;
_24 = [210_u8];
_15 = core::ptr::addr_of_mut!(_25);
_11 = !_5;
_12.fld0.0 = _28;
_10 = _5 >> _11;
_24 = [187_u8];
match _22 {
0 => bb11,
1 => bb4,
2 => bb3,
3 => bb13,
4 => bb14,
3741310170406920736 => bb16,
_ => bb15
}
}
bb13 = {
_7 = (-1837599630_i32) as isize;
_24 = [89_u8];
_22 = -4154013835917094849_i64;
_20 = [_23];
_28 = _12.fld0.0 ^ _12.fld0.0;
_17 = RET as f64;
_1 = _16 << _16;
_11 = _8 << _2;
_3 = _2 + _16;
_23 = _17 as u64;
Goto(bb12)
}
bb14 = {
RET = 542441347_u32 & 83327306_u32;
_4 = ['\u{4484c}','\u{6e988}','\u{29917}','\u{99327}'];
_2 = _7;
_11 = 853412984_i32 as isize;
_17 = RET as f64;
_12.fld0 = (15435_i16,);
_2 = (-67050977825018628606869045173678189004_i128) as isize;
_17 = 21_i8 as f64;
_8 = 4_usize as isize;
_6 = false as isize;
_4 = ['\u{3ef3a}','\u{f0f30}','\u{b6819}','\u{20e1c}'];
_7 = _1 & _3;
_11 = true as isize;
_1 = _7 ^ _7;
_8 = 324863783597435281324950758811552278768_u128 as isize;
_12.fld0 = ((-4879_i16),);
RET = 3823894773_u32 - 2072949788_u32;
_16 = _7;
_6 = RET as isize;
_11 = _7;
_1 = _16;
_12.fld0.0 = (-19951_i16);
Goto(bb2)
}
bb15 = {
Return()
}
bb16 = {
_4 = [_31,_31,_31,_31];
_22 = !2532557974608237489_i64;
_2 = -_10;
_4 = [_31,_31,_31,_31];
_11 = !_10;
_1 = _10 | _2;
_33 = _17 + _17;
_29 = false;
_35 = _23 - _23;
RET = 58039_u16 as u32;
_15 = core::ptr::addr_of_mut!(_25);
_36 = _29 & _29;
_6 = RET as isize;
_24 = [124_u8];
_9 = !_16;
_8 = _2;
_26 = &_34;
RET = 1414922659_u32;
RET = 440576576_u32 * 410618993_u32;
_2 = !_5;
_23 = _35;
Goto(bb17)
}
bb17 = {
Call(_40 = dump_var(10_usize, 3_usize, Move(_3), 35_usize, Move(_35), 23_usize, Move(_23), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(10_usize, 2_usize, Move(_2), 21_usize, Move(_21), 4_usize, Move(_4), 5_usize, Move(_5)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_40 = dump_var(10_usize, 20_usize, Move(_20), 6_usize, Move(_6), 36_usize, Move(_36), 41_usize, _41), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize,mut _15: isize) -> [char; 4] {
mir! {
type RET = [char; 4];
let _16: char;
let _17: usize;
let _18: f64;
let _19: isize;
let _20: *const char;
let _21: Adt52;
let _22: bool;
let _23: u16;
let _24: Adt55;
let _25: char;
let _26: [u64; 1];
let _27: [u8; 1];
let _28: isize;
let _29: [i32; 8];
let _30: [u8; 1];
let _31: char;
let _32: bool;
let _33: Adt48;
let _34: usize;
let _35: [isize; 7];
let _36: (i16,);
let _37: [u128; 6];
let _38: isize;
let _39: ();
let _40: ();
{
RET = ['\u{9cb03}','\u{c775b}','\u{c7f02}','\u{640ed}'];
_4 = 3580973174_u32 as isize;
_5 = 379794178644900982_i64 as isize;
RET = ['\u{139b}','\u{744da}','\u{a8c62}','\u{c3fc2}'];
_7 = -_12;
_13 = !_2;
_8 = _10;
Call(_15 = fn12(_7, _1, _7, _14), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = _7 - _10;
_1 = (-696354404_i32) as isize;
Goto(bb2)
}
bb2 = {
_16 = '\u{25670}';
_10 = _7 & _14;
_18 = 2525864792653782847_i64 as f64;
_18 = (-84_i8) as f64;
_7 = -_12;
_1 = _14 & _13;
_10 = 225999053509140426806233276555022290628_u128 as isize;
_11 = _7;
_7 = !_11;
_13 = _9 - _12;
_13 = (-33553939274335395_i64) as isize;
_17 = 76_i8 as usize;
_15 = _11 ^ _5;
_5 = 295029749139360409607524721758663259312_u128 as isize;
_11 = (-28580030248472484150978869998805095600_i128) as isize;
_9 = 169569597584753335163049550203593658171_u128 as isize;
_16 = '\u{9e1}';
RET = [_16,_16,_16,_16];
_9 = _2;
_1 = _14;
RET = [_16,_16,_16,_16];
_9 = true as isize;
_8 = 1085589929135862434_u64 as isize;
_13 = _3 & _7;
Call(_21.fld1 = fn13(_14, _14, _7, _7, _1, _6, _14, _6, _7, _1, _12), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9 = _14 + _1;
_13 = _7;
_5 = _18 as isize;
_9 = _14;
_22 = !false;
_5 = _7;
_4 = !_1;
_23 = !61419_u16;
_11 = _9 & _7;
_10 = -_13;
_21.fld0.0 = (-28688_i16) - (-26277_i16);
_21.fld1 = [(-67_i8),32_i8,(-123_i8),(-3_i8),70_i8];
_26 = [9141830825322885850_u64];
_10 = _6 << _6;
_23 = 4464_u16;
_20 = core::ptr::addr_of!(_25);
_22 = !true;
_13 = _4;
_21.fld0 = ((-16031_i16),);
_23 = 6056_u16 ^ 29088_u16;
_9 = (-79_i8) as isize;
RET = [_16,_16,_16,_16];
_25 = _16;
match _21.fld0.0 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
340282366920938463463374607431768195425 => bb9,
_ => bb8
}
}
bb4 = {
_16 = '\u{25670}';
_10 = _7 & _14;
_18 = 2525864792653782847_i64 as f64;
_18 = (-84_i8) as f64;
_7 = -_12;
_1 = _14 & _13;
_10 = 225999053509140426806233276555022290628_u128 as isize;
_11 = _7;
_7 = !_11;
_13 = _9 - _12;
_13 = (-33553939274335395_i64) as isize;
_17 = 76_i8 as usize;
_15 = _11 ^ _5;
_5 = 295029749139360409607524721758663259312_u128 as isize;
_11 = (-28580030248472484150978869998805095600_i128) as isize;
_9 = 169569597584753335163049550203593658171_u128 as isize;
_16 = '\u{9e1}';
RET = [_16,_16,_16,_16];
_9 = _2;
_1 = _14;
RET = [_16,_16,_16,_16];
_9 = true as isize;
_8 = 1085589929135862434_u64 as isize;
_13 = _3 & _7;
Call(_21.fld1 = fn13(_14, _14, _7, _7, _1, _6, _14, _6, _7, _1, _12), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_5 = _7 - _10;
_1 = (-696354404_i32) as isize;
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
_22 = !true;
_8 = _13 >> _5;
_30 = [49_u8];
(*_20) = _16;
Goto(bb10)
}
bb10 = {
_27 = [8_u8];
_21.fld1 = [(-66_i8),(-104_i8),(-116_i8),(-95_i8),(-116_i8)];
_28 = -_3;
_21.fld1 = [36_i8,34_i8,92_i8,10_i8,(-47_i8)];
_7 = _14;
_33.fld0 = _21.fld0;
Goto(bb11)
}
bb11 = {
_11 = 2188799986_u32 as isize;
(*_20) = _16;
_14 = _6 & _1;
_28 = _15 << _14;
match _33.fld0.0 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb8,
340282366920938463463374607431768195425 => bb13,
_ => bb12
}
}
bb12 = {
_9 = _14 + _1;
_13 = _7;
_5 = _18 as isize;
_9 = _14;
_22 = !false;
_5 = _7;
_4 = !_1;
_23 = !61419_u16;
_11 = _9 & _7;
_10 = -_13;
_21.fld0.0 = (-28688_i16) - (-26277_i16);
_21.fld1 = [(-67_i8),32_i8,(-123_i8),(-3_i8),70_i8];
_26 = [9141830825322885850_u64];
_10 = _6 << _6;
_23 = 4464_u16;
_20 = core::ptr::addr_of!(_25);
_22 = !true;
_13 = _4;
_21.fld0 = ((-16031_i16),);
_23 = 6056_u16 ^ 29088_u16;
_9 = (-79_i8) as isize;
RET = [_16,_16,_16,_16];
_25 = _16;
match _21.fld0.0 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
340282366920938463463374607431768195425 => bb9,
_ => bb8
}
}
bb13 = {
_29 = [282182655_i32,628373938_i32,293305612_i32,1058558735_i32,(-1892370907_i32),1438253443_i32,(-1797420014_i32),764777092_i32];
_27 = _30;
_31 = _25;
_19 = -_12;
_21.fld0 = (_33.fld0.0,);
_36 = (_21.fld0.0,);
_22 = true & true;
_21.fld1 = [(-43_i8),(-96_i8),112_i8,(-55_i8),119_i8];
_37 = [43081987300881120064978422011410196035_u128,43538296498822408937658937078068911127_u128,191686744930344462546909895022596108197_u128,118570538738568096211221819559318864128_u128,223673717430633872620390963446883204737_u128,4892563080532257134022014111601949691_u128];
_15 = _17 as isize;
_17 = !15929431295388077433_usize;
_22 = _10 <= _6;
_28 = _14;
_35 = [_7,_1,_10,_13,_2,_1,_4];
match _21.fld0.0 {
0 => bb4,
1 => bb6,
340282366920938463463374607431768195425 => bb14,
_ => bb7
}
}
bb14 = {
_35 = [_28,_4,_2,_5,_8,_28,_1];
_31 = (*_20);
_8 = _19 + _12;
_29 = [(-1988791776_i32),(-731993920_i32),(-858309990_i32),1318652829_i32,1039946129_i32,1902054536_i32,(-2053125776_i32),(-1318470997_i32)];
RET = [_16,_31,_16,_31];
_22 = _1 <= _13;
_20 = core::ptr::addr_of!((*_20));
_35 = [_10,_7,_5,_19,_13,_3,_7];
_33.fld0.0 = _36.0;
_19 = _5;
_16 = _25;
(*_20) = _16;
_34 = !_17;
_23 = _17 as u16;
RET = [_25,_25,_16,_25];
_20 = core::ptr::addr_of!(_25);
_10 = _2 >> _7;
_8 = _10;
_10 = -_4;
_17 = _34 & _34;
_32 = _22 ^ _22;
_26 = [3091749681583348213_u64];
_4 = !_10;
_1 = 10822813158018093923_u64 as isize;
_23 = !43488_u16;
_13 = _2 ^ _19;
_17 = _16 as usize;
_6 = !_28;
_37 = [192353388401017201391909247071536075766_u128,47022083000371317304424283871075906445_u128,225391084032965070644790287367928807174_u128,42553537765324973693494752850070488484_u128,74118701802514200792180426295093098565_u128,72108877616959537824333920675226322239_u128];
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(11_usize, 13_usize, Move(_13), 32_usize, Move(_32), 19_usize, Move(_19), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(11_usize, 22_usize, Move(_22), 36_usize, Move(_36), 25_usize, Move(_25), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(11_usize, 27_usize, Move(_27), 31_usize, Move(_31), 5_usize, Move(_5), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_39 = dump_var(11_usize, 9_usize, Move(_9), 12_usize, Move(_12), 14_usize, Move(_14), 10_usize, Move(_10)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize) -> isize {
mir! {
type RET = isize;
let _5: ();
let _6: ();
{
RET = _4;
RET = _1 * _2;
_3 = _4 & _4;
Goto(bb1)
}
bb1 = {
Call(_5 = dump_var(12_usize, 1_usize, Move(_1), 2_usize, Move(_2), 6_usize, _6, 6_usize, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize) -> [i8; 5] {
mir! {
type RET = [i8; 5];
let _12: &'static u16;
let _13: *mut i128;
let _14: bool;
let _15: u16;
let _16: char;
let _17: [isize; 7];
let _18: [u128; 6];
let _19: [i16; 4];
let _20: [u128; 6];
let _21: Adt48;
let _22: isize;
let _23: f64;
let _24: [i16; 4];
let _25: *const (isize, [char; 4], u32);
let _26: Adt52;
let _27: u128;
let _28: bool;
let _29: i8;
let _30: u128;
let _31: u16;
let _32: Adt44;
let _33: (i16,);
let _34: isize;
let _35: bool;
let _36: [u8; 1];
let _37: f32;
let _38: Adt47;
let _39: isize;
let _40: [i8; 5];
let _41: f32;
let _42: Adt47;
let _43: u32;
let _44: Adt42;
let _45: ();
let _46: ();
{
_9 = 7_usize as isize;
_3 = _2;
_8 = (-24695_i16) as isize;
_9 = _6;
Goto(bb1)
}
bb1 = {
_2 = _7;
_1 = -_2;
RET = [29_i8,79_i8,(-90_i8),62_i8,112_i8];
_4 = _1 * _7;
_9 = _2 >> _6;
_4 = _10;
_10 = _11 - _7;
_12 = &_15;
RET = [(-89_i8),(-94_i8),80_i8,(-108_i8),(-106_i8)];
_1 = !_3;
RET = [(-72_i8),(-36_i8),0_i8,97_i8,85_i8];
_2 = -_11;
_2 = _11 << _10;
_7 = _6 >> _10;
_10 = _11;
_8 = _9 >> _9;
_2 = _3;
_4 = _8 | _7;
Goto(bb2)
}
bb2 = {
_3 = 113301216768010291682544264794724439015_u128 as isize;
_20 = [154401526030070544511753304365565390367_u128,179645449610501716213096373679945650110_u128,57285049388566196992511360782336779546_u128,165912296558160240128578083152477125316_u128,45379173327347906306326941294401950458_u128,351761879505699533053669921189247288_u128];
RET = [54_i8,(-122_i8),(-89_i8),(-114_i8),(-128_i8)];
_6 = false as isize;
_16 = '\u{a8fb5}';
_17 = [_1,_7,_9,_5,_4,_5,_5];
_7 = 1603491994_u32 as isize;
_4 = _5 >> _11;
_18 = _20;
_9 = _5 ^ _5;
_26.fld0 = (18010_i16,);
_23 = 205_u8 as f64;
_5 = _4 << _9;
_21 = Adt48 { fld0: _26.fld0 };
_26 = Adt52 { fld0: _21.fld0,fld1: RET };
_26.fld1 = [(-1_i8),85_i8,(-104_i8),11_i8,123_i8];
_9 = 175_u8 as isize;
_1 = (-6578776488028830555_i64) as isize;
_17 = [_11,_8,_2,_5,_8,_10,_4];
_15 = 23705_u16 - 6872_u16;
_21.fld0 = (_26.fld0.0,);
_26.fld1 = RET;
_6 = _10;
_4 = -_6;
Call(_6 = fn14(_5, _3, _5, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = [(-58_i8),28_i8,32_i8,86_i8,18_i8];
_20 = [138747336940085262920934827187394042228_u128,245746205740921660834797551359175890409_u128,110016920243683734039077387982412236604_u128,324672084221970896284921305197891212013_u128,32128379494535227841319615352287892536_u128,273416758322374473908572513667128854694_u128];
_10 = _11 ^ _5;
_3 = _4 - _11;
_26.fld1 = [(-6_i8),99_i8,58_i8,(-67_i8),(-50_i8)];
_28 = !false;
_21 = Adt48 { fld0: _26.fld0 };
_26.fld1 = [(-25_i8),104_i8,(-111_i8),(-123_i8),(-70_i8)];
_1 = -_3;
_22 = 2445762799487817767_i64 as isize;
_6 = (-8426953041658276236_i64) as isize;
_5 = _21.fld0.0 as isize;
_3 = !_10;
_16 = '\u{86323}';
_4 = 4038703814_u32 as isize;
_29 = 74_i8;
_27 = 80572001327321247407828990640509141449_u128;
_19 = [_21.fld0.0,_21.fld0.0,_26.fld0.0,_26.fld0.0];
_19 = [_26.fld0.0,_21.fld0.0,_21.fld0.0,_21.fld0.0];
_27 = 289705810560745338997324363128459576425_u128;
Call(_19 = fn15(_1, _17), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_17 = [_1,_10,_11,_1,_11,_10,_3];
_21 = Adt48 { fld0: _26.fld0 };
_24 = _19;
RET = [_29,_29,_29,_29,_29];
_29 = 84_i8 & 23_i8;
_8 = 120_u8 as isize;
match _27 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
289705810560745338997324363128459576425 => bb8,
_ => bb7
}
}
bb5 = {
RET = [(-58_i8),28_i8,32_i8,86_i8,18_i8];
_20 = [138747336940085262920934827187394042228_u128,245746205740921660834797551359175890409_u128,110016920243683734039077387982412236604_u128,324672084221970896284921305197891212013_u128,32128379494535227841319615352287892536_u128,273416758322374473908572513667128854694_u128];
_10 = _11 ^ _5;
_3 = _4 - _11;
_26.fld1 = [(-6_i8),99_i8,58_i8,(-67_i8),(-50_i8)];
_28 = !false;
_21 = Adt48 { fld0: _26.fld0 };
_26.fld1 = [(-25_i8),104_i8,(-111_i8),(-123_i8),(-70_i8)];
_1 = -_3;
_22 = 2445762799487817767_i64 as isize;
_6 = (-8426953041658276236_i64) as isize;
_5 = _21.fld0.0 as isize;
_3 = !_10;
_16 = '\u{86323}';
_4 = 4038703814_u32 as isize;
_29 = 74_i8;
_27 = 80572001327321247407828990640509141449_u128;
_19 = [_21.fld0.0,_21.fld0.0,_26.fld0.0,_26.fld0.0];
_19 = [_26.fld0.0,_21.fld0.0,_21.fld0.0,_21.fld0.0];
_27 = 289705810560745338997324363128459576425_u128;
Call(_19 = fn15(_1, _17), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_3 = 113301216768010291682544264794724439015_u128 as isize;
_20 = [154401526030070544511753304365565390367_u128,179645449610501716213096373679945650110_u128,57285049388566196992511360782336779546_u128,165912296558160240128578083152477125316_u128,45379173327347906306326941294401950458_u128,351761879505699533053669921189247288_u128];
RET = [54_i8,(-122_i8),(-89_i8),(-114_i8),(-128_i8)];
_6 = false as isize;
_16 = '\u{a8fb5}';
_17 = [_1,_7,_9,_5,_4,_5,_5];
_7 = 1603491994_u32 as isize;
_4 = _5 >> _11;
_18 = _20;
_9 = _5 ^ _5;
_26.fld0 = (18010_i16,);
_23 = 205_u8 as f64;
_5 = _4 << _9;
_21 = Adt48 { fld0: _26.fld0 };
_26 = Adt52 { fld0: _21.fld0,fld1: RET };
_26.fld1 = [(-1_i8),85_i8,(-104_i8),11_i8,123_i8];
_9 = 175_u8 as isize;
_1 = (-6578776488028830555_i64) as isize;
_17 = [_11,_8,_2,_5,_8,_10,_4];
_15 = 23705_u16 - 6872_u16;
_21.fld0 = (_26.fld0.0,);
_26.fld1 = RET;
_6 = _10;
_4 = -_6;
Call(_6 = fn14(_5, _3, _5, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_2 = _7;
_1 = -_2;
RET = [29_i8,79_i8,(-90_i8),62_i8,112_i8];
_4 = _1 * _7;
_9 = _2 >> _6;
_4 = _10;
_10 = _11 - _7;
_12 = &_15;
RET = [(-89_i8),(-94_i8),80_i8,(-108_i8),(-106_i8)];
_1 = !_3;
RET = [(-72_i8),(-36_i8),0_i8,97_i8,85_i8];
_2 = -_11;
_2 = _11 << _10;
_7 = _6 >> _10;
_10 = _11;
_8 = _9 >> _9;
_2 = _3;
_4 = _8 | _7;
Goto(bb2)
}
bb8 = {
_1 = _10 << _10;
_33.0 = _21.fld0.0 + _21.fld0.0;
_28 = true;
_14 = !_28;
_12 = &_31;
RET = [_29,_29,_29,_29,_29];
_2 = (-8531923288078680945_i64) as isize;
_15 = 4_usize as u16;
RET = [_29,_29,_29,_29,_29];
_31 = _15;
_12 = &_31;
_1 = _11 * _3;
_18 = [_27,_27,_27,_27,_27,_27];
_33 = (_26.fld0.0,);
_33 = (_26.fld0.0,);
_34 = _27 as isize;
_2 = _23 as isize;
_35 = !_14;
_21.fld0 = (_26.fld0.0,);
_17 = [_10,_10,_10,_10,_1,_10,_3];
_18 = _20;
RET = [_29,_29,_29,_29,_29];
match _27 {
0 => bb9,
289705810560745338997324363128459576425 => bb11,
_ => bb10
}
}
bb9 = {
_17 = [_1,_10,_11,_1,_11,_10,_3];
_21 = Adt48 { fld0: _26.fld0 };
_24 = _19;
RET = [_29,_29,_29,_29,_29];
_29 = 84_i8 & 23_i8;
_8 = 120_u8 as isize;
match _27 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
289705810560745338997324363128459576425 => bb8,
_ => bb7
}
}
bb10 = {
RET = [(-58_i8),28_i8,32_i8,86_i8,18_i8];
_20 = [138747336940085262920934827187394042228_u128,245746205740921660834797551359175890409_u128,110016920243683734039077387982412236604_u128,324672084221970896284921305197891212013_u128,32128379494535227841319615352287892536_u128,273416758322374473908572513667128854694_u128];
_10 = _11 ^ _5;
_3 = _4 - _11;
_26.fld1 = [(-6_i8),99_i8,58_i8,(-67_i8),(-50_i8)];
_28 = !false;
_21 = Adt48 { fld0: _26.fld0 };
_26.fld1 = [(-25_i8),104_i8,(-111_i8),(-123_i8),(-70_i8)];
_1 = -_3;
_22 = 2445762799487817767_i64 as isize;
_6 = (-8426953041658276236_i64) as isize;
_5 = _21.fld0.0 as isize;
_3 = !_10;
_16 = '\u{86323}';
_4 = 4038703814_u32 as isize;
_29 = 74_i8;
_27 = 80572001327321247407828990640509141449_u128;
_19 = [_21.fld0.0,_21.fld0.0,_26.fld0.0,_26.fld0.0];
_19 = [_26.fld0.0,_21.fld0.0,_21.fld0.0,_21.fld0.0];
_27 = 289705810560745338997324363128459576425_u128;
Call(_19 = fn15(_1, _17), ReturnTo(bb4), UnwindUnreachable())
}
bb11 = {
_14 = _35;
_4 = 8899904416333765315_i64 as isize;
_17 = [_10,_3,_11,_11,_1,_11,_3];
_20 = [_27,_27,_27,_27,_27,_27];
Call(_17 = fn16(_3, _1, _3, _35, _10, _3, _3, _11, _3, _18, _1, _21.fld0.0, _10, _3, _10, _10), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
RET = _26.fld1;
_28 = _10 > _3;
_17 = [_11,_3,_1,_1,_10,_1,_10];
_29 = -55_i8;
_29 = !(-38_i8);
_30 = _27 - _27;
_23 = _29 as f64;
_29 = -88_i8;
_34 = _11 << _1;
_30 = _27;
_30 = _27;
_40 = RET;
_20 = [_30,_30,_27,_30,_30,_30];
_37 = _21.fld0.0 as f32;
_8 = _11;
_27 = _8 as u128;
_26.fld0 = _33;
_39 = !_11;
_4 = -_39;
_33.0 = _26.fld0.0 - _26.fld0.0;
RET = [_29,_29,_29,_29,_29];
_19 = [_33.0,_33.0,_33.0,_21.fld0.0];
_37 = 13573551763814527070_u64 as f32;
match _26.fld0.0 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
18010 => bb19,
_ => bb18
}
}
bb13 = {
RET = [(-58_i8),28_i8,32_i8,86_i8,18_i8];
_20 = [138747336940085262920934827187394042228_u128,245746205740921660834797551359175890409_u128,110016920243683734039077387982412236604_u128,324672084221970896284921305197891212013_u128,32128379494535227841319615352287892536_u128,273416758322374473908572513667128854694_u128];
_10 = _11 ^ _5;
_3 = _4 - _11;
_26.fld1 = [(-6_i8),99_i8,58_i8,(-67_i8),(-50_i8)];
_28 = !false;
_21 = Adt48 { fld0: _26.fld0 };
_26.fld1 = [(-25_i8),104_i8,(-111_i8),(-123_i8),(-70_i8)];
_1 = -_3;
_22 = 2445762799487817767_i64 as isize;
_6 = (-8426953041658276236_i64) as isize;
_5 = _21.fld0.0 as isize;
_3 = !_10;
_16 = '\u{86323}';
_4 = 4038703814_u32 as isize;
_29 = 74_i8;
_27 = 80572001327321247407828990640509141449_u128;
_19 = [_21.fld0.0,_21.fld0.0,_26.fld0.0,_26.fld0.0];
_19 = [_26.fld0.0,_21.fld0.0,_21.fld0.0,_21.fld0.0];
_27 = 289705810560745338997324363128459576425_u128;
Call(_19 = fn15(_1, _17), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
RET = [(-58_i8),28_i8,32_i8,86_i8,18_i8];
_20 = [138747336940085262920934827187394042228_u128,245746205740921660834797551359175890409_u128,110016920243683734039077387982412236604_u128,324672084221970896284921305197891212013_u128,32128379494535227841319615352287892536_u128,273416758322374473908572513667128854694_u128];
_10 = _11 ^ _5;
_3 = _4 - _11;
_26.fld1 = [(-6_i8),99_i8,58_i8,(-67_i8),(-50_i8)];
_28 = !false;
_21 = Adt48 { fld0: _26.fld0 };
_26.fld1 = [(-25_i8),104_i8,(-111_i8),(-123_i8),(-70_i8)];
_1 = -_3;
_22 = 2445762799487817767_i64 as isize;
_6 = (-8426953041658276236_i64) as isize;
_5 = _21.fld0.0 as isize;
_3 = !_10;
_16 = '\u{86323}';
_4 = 4038703814_u32 as isize;
_29 = 74_i8;
_27 = 80572001327321247407828990640509141449_u128;
_19 = [_21.fld0.0,_21.fld0.0,_26.fld0.0,_26.fld0.0];
_19 = [_26.fld0.0,_21.fld0.0,_21.fld0.0,_21.fld0.0];
_27 = 289705810560745338997324363128459576425_u128;
Call(_19 = fn15(_1, _17), ReturnTo(bb4), UnwindUnreachable())
}
bb15 = {
_17 = [_1,_10,_11,_1,_11,_10,_3];
_21 = Adt48 { fld0: _26.fld0 };
_24 = _19;
RET = [_29,_29,_29,_29,_29];
_29 = 84_i8 & 23_i8;
_8 = 120_u8 as isize;
match _27 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
289705810560745338997324363128459576425 => bb8,
_ => bb7
}
}
bb16 = {
_1 = _10 << _10;
_33.0 = _21.fld0.0 + _21.fld0.0;
_28 = true;
_14 = !_28;
_12 = &_31;
RET = [_29,_29,_29,_29,_29];
_2 = (-8531923288078680945_i64) as isize;
_15 = 4_usize as u16;
RET = [_29,_29,_29,_29,_29];
_31 = _15;
_12 = &_31;
_1 = _11 * _3;
_18 = [_27,_27,_27,_27,_27,_27];
_33 = (_26.fld0.0,);
_33 = (_26.fld0.0,);
_34 = _27 as isize;
_2 = _23 as isize;
_35 = !_14;
_21.fld0 = (_26.fld0.0,);
_17 = [_10,_10,_10,_10,_1,_10,_3];
_18 = _20;
RET = [_29,_29,_29,_29,_29];
match _27 {
0 => bb9,
289705810560745338997324363128459576425 => bb11,
_ => bb10
}
}
bb17 = {
_17 = [_1,_10,_11,_1,_11,_10,_3];
_21 = Adt48 { fld0: _26.fld0 };
_24 = _19;
RET = [_29,_29,_29,_29,_29];
_29 = 84_i8 & 23_i8;
_8 = 120_u8 as isize;
match _27 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
289705810560745338997324363128459576425 => bb8,
_ => bb7
}
}
bb18 = {
RET = [(-58_i8),28_i8,32_i8,86_i8,18_i8];
_20 = [138747336940085262920934827187394042228_u128,245746205740921660834797551359175890409_u128,110016920243683734039077387982412236604_u128,324672084221970896284921305197891212013_u128,32128379494535227841319615352287892536_u128,273416758322374473908572513667128854694_u128];
_10 = _11 ^ _5;
_3 = _4 - _11;
_26.fld1 = [(-6_i8),99_i8,58_i8,(-67_i8),(-50_i8)];
_28 = !false;
_21 = Adt48 { fld0: _26.fld0 };
_26.fld1 = [(-25_i8),104_i8,(-111_i8),(-123_i8),(-70_i8)];
_1 = -_3;
_22 = 2445762799487817767_i64 as isize;
_6 = (-8426953041658276236_i64) as isize;
_5 = _21.fld0.0 as isize;
_3 = !_10;
_16 = '\u{86323}';
_4 = 4038703814_u32 as isize;
_29 = 74_i8;
_27 = 80572001327321247407828990640509141449_u128;
_19 = [_21.fld0.0,_21.fld0.0,_26.fld0.0,_26.fld0.0];
_19 = [_26.fld0.0,_21.fld0.0,_21.fld0.0,_21.fld0.0];
_27 = 289705810560745338997324363128459576425_u128;
Call(_19 = fn15(_1, _17), ReturnTo(bb4), UnwindUnreachable())
}
bb19 = {
_26.fld0.0 = _33.0 + _21.fld0.0;
RET = [_29,_29,_29,_29,_29];
_33.0 = _8 as i16;
_34 = 598066961_i32 as isize;
_21.fld0 = (_33.0,);
_24 = _19;
_5 = !_8;
_17 = [_5,_10,_8,_39,_10,_4,_4];
RET = _26.fld1;
_9 = -_8;
_21 = Adt48 { fld0: _33 };
_22 = (-20823551459125252119865199068850099328_i128) as isize;
_33.0 = _21.fld0.0;
_41 = -_37;
_1 = _11 - _39;
Goto(bb20)
}
bb20 = {
Call(_45 = dump_var(13_usize, 39_usize, Move(_39), 35_usize, Move(_35), 4_usize, Move(_4), 20_usize, Move(_20)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_45 = dump_var(13_usize, 9_usize, Move(_9), 29_usize, Move(_29), 24_usize, Move(_24), 14_usize, Move(_14)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_45 = dump_var(13_usize, 19_usize, Move(_19), 6_usize, Move(_6), 30_usize, Move(_30), 15_usize, Move(_15)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_45 = dump_var(13_usize, 33_usize, Move(_33), 3_usize, Move(_3), 7_usize, Move(_7), 46_usize, _46), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: [isize; 7]) -> isize {
mir! {
type RET = isize;
let _5: [i32; 8];
let _6: isize;
let _7: ();
let _8: ();
{
_1 = -_3;
RET = _3;
_3 = 9108752319368338565_u64 as isize;
_2 = _1 * RET;
_3 = !RET;
_2 = _1 | RET;
RET = _1 | _1;
RET = _3;
_1 = 107_u8 as isize;
_5 = [(-16966888_i32),1384477396_i32,1345242488_i32,1283585415_i32,(-266489475_i32),1669168320_i32,(-1774993176_i32),345867637_i32];
RET = _3 ^ _2;
_2 = 16616_i16 as isize;
RET = -_3;
_5 = [(-1518957648_i32),411408900_i32,1499608990_i32,1091090378_i32,149884725_i32,(-1311692876_i32),1855077349_i32,(-1845055019_i32)];
RET = _3 << _3;
_4 = [_3,RET,_2,RET,RET,_3,_3];
_6 = -RET;
_5 = [(-362549692_i32),654712399_i32,1984249187_i32,(-241670731_i32),148301046_i32,305720391_i32,(-767960425_i32),(-505907263_i32)];
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(14_usize, 2_usize, Move(_2), 1_usize, Move(_1), 5_usize, Move(_5), 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: isize,mut _2: [isize; 7]) -> [i16; 4] {
mir! {
type RET = [i16; 4];
let _3: [isize; 7];
let _4: [u128; 6];
let _5: [char; 7];
let _6: i128;
let _7: u32;
let _8: isize;
let _9: isize;
let _10: Adt44;
let _11: char;
let _12: u128;
let _13: *const char;
let _14: f64;
let _15: [i8; 5];
let _16: *const u32;
let _17: f64;
let _18: usize;
let _19: f32;
let _20: [i32; 8];
let _21: char;
let _22: [u32; 1];
let _23: [i8; 5];
let _24: Adt46;
let _25: isize;
let _26: Adt45;
let _27: Adt52;
let _28: [u64; 1];
let _29: bool;
let _30: u8;
let _31: Adt42;
let _32: Adt51;
let _33: [i8; 5];
let _34: char;
let _35: isize;
let _36: Adt43;
let _37: (i16,);
let _38: u16;
let _39: *const (u32, *const u32, u32, i64);
let _40: f32;
let _41: (isize, [char; 4], u32);
let _42: isize;
let _43: bool;
let _44: *const char;
let _45: [isize; 7];
let _46: f32;
let _47: Adt45;
let _48: ();
let _49: ();
{
RET = [27417_i16,(-27018_i16),(-26374_i16),(-14284_i16)];
_2 = [_1,_1,_1,_1,_1,_1,_1];
RET = [(-1467_i16),27505_i16,(-6355_i16),4908_i16];
_4 = [190401428605671327744050536385605993909_u128,169082152835210897841664450968350153981_u128,209541311330403524140888970530842254343_u128,255501616244041963060066079481124205229_u128,307231828340028557126705348433042718137_u128,335779749774416642557976646837224841335_u128];
_4 = [299054983484019079098455114157235124732_u128,165043231348379945214944766388492933094_u128,276409928460243380183013187148226183667_u128,283647879657590969204879275446981018388_u128,246643139742660844512447094277382090372_u128,134774664203379329162703410589321698915_u128];
_1 = -9223372036854775807_isize;
_1 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_3 = [_1,_1,_1,_1,_1,_1,_1];
_2 = [_1,_1,_1,_1,_1,_1,_1];
Goto(bb1)
}
bb1 = {
_5 = ['\u{256bc}','\u{10e80b}','\u{b9ed7}','\u{fa57e}','\u{c1cee}','\u{ab975}','\u{a6c64}'];
_1 = !(-1_isize);
_2 = _3;
_2 = [_1,_1,_1,_1,_1,_1,_1];
_1 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
RET = [(-13834_i16),(-10166_i16),1044_i16,(-151_i16)];
_8 = _1 & _1;
_9 = -_8;
_6 = !(-108214297878805940792431323532547067122_i128);
_3 = [_8,_8,_9,_8,_8,_8,_8];
_7 = !1440445776_u32;
_8 = false as isize;
RET = [(-10060_i16),8792_i16,27760_i16,2778_i16];
_6 = !(-42743622409335128235043159446529140364_i128);
_2 = _3;
Goto(bb2)
}
bb2 = {
RET = [(-5420_i16),(-11064_i16),(-22542_i16),(-11064_i16)];
RET = [20334_i16,(-8819_i16),25532_i16,2146_i16];
_5 = ['\u{104e21}','\u{3dc2e}','\u{90e0b}','\u{d32ac}','\u{b36be}','\u{e510f}','\u{10889e}'];
_6 = 76011898294386501314139474700025673071_i128;
_6 = 80571284532791700506495354454743507764_i128 & 53368976251043107189203549278521197695_i128;
RET = [(-21815_i16),(-4048_i16),16331_i16,23335_i16];
_3 = [_9,_1,_9,_9,_9,_1,_1];
RET = [(-25022_i16),(-28420_i16),(-22246_i16),22318_i16];
_3 = [_8,_1,_8,_1,_1,_9,_1];
RET = [(-23980_i16),12427_i16,(-26464_i16),(-26995_i16)];
_1 = !_9;
RET = [18018_i16,(-9923_i16),(-13761_i16),(-5493_i16)];
_2 = [_9,_1,_9,_1,_9,_8,_9];
_7 = !4042311315_u32;
Goto(bb3)
}
bb3 = {
_6 = 4_usize as i128;
RET = [3657_i16,(-24725_i16),(-16668_i16),22133_i16];
_5 = ['\u{c19b0}','\u{1b1ec}','\u{d4095}','\u{10c683}','\u{ebbf3}','\u{5ddf4}','\u{7b211}'];
_7 = !1653823872_u32;
_3 = _2;
_13 = core::ptr::addr_of!(_11);
_6 = (-805265370980597500758919627259174060_i128) - 165641242436353920769803526499588559962_i128;
(*_13) = '\u{7c4c9}';
(*_13) = '\u{18852}';
_11 = '\u{cc12}';
_15 = [55_i8,(-84_i8),(-116_i8),(-14_i8),119_i8];
_15 = [(-15_i8),87_i8,64_i8,4_i8,41_i8];
_14 = _1 as f64;
_14 = 949773200_i32 as f64;
RET = [19883_i16,4605_i16,(-13233_i16),(-18099_i16)];
_9 = -_1;
_16 = core::ptr::addr_of!(_7);
_1 = false as isize;
Goto(bb4)
}
bb4 = {
_16 = core::ptr::addr_of!(_7);
_9 = 37119_u16 as isize;
(*_13) = '\u{cd918}';
(*_16) = 2613985379_u32;
RET = [15437_i16,(-30203_i16),(-10374_i16),(-23971_i16)];
_4 = [27762075790766565131464178035979650076_u128,297604828805042051831106170864882734039_u128,82650824787794692015426935832955446241_u128,169444873810236200064638470402903481458_u128,278917839150279274282735670435131727190_u128,226808112087783203421732967547190298301_u128];
_11 = '\u{5fd7a}';
_15 = [(-104_i8),35_i8,(-35_i8),(-124_i8),93_i8];
(*_13) = '\u{2482}';
_4 = [165531931061561213658869434370622744593_u128,273056586336186711718292016946856115495_u128,132384397307595773367280981013475021715_u128,206690287503774001524183565196206914354_u128,233316318347673172135386934529693480823_u128,62240120848037579534043652589260376740_u128];
_12 = 218569977778170380985037679416284311614_u128;
_11 = '\u{bd018}';
RET = [10973_i16,2974_i16,8205_i16,22331_i16];
_2 = [_8,_9,_1,_8,_9,_8,_9];
_19 = (-30608_i16) as f32;
_17 = 14199_i16 as f64;
match (*_16) {
0 => bb2,
2613985379 => bb6,
_ => bb5
}
}
bb5 = {
RET = [(-5420_i16),(-11064_i16),(-22542_i16),(-11064_i16)];
RET = [20334_i16,(-8819_i16),25532_i16,2146_i16];
_5 = ['\u{104e21}','\u{3dc2e}','\u{90e0b}','\u{d32ac}','\u{b36be}','\u{e510f}','\u{10889e}'];
_6 = 76011898294386501314139474700025673071_i128;
_6 = 80571284532791700506495354454743507764_i128 & 53368976251043107189203549278521197695_i128;
RET = [(-21815_i16),(-4048_i16),16331_i16,23335_i16];
_3 = [_9,_1,_9,_9,_9,_1,_1];
RET = [(-25022_i16),(-28420_i16),(-22246_i16),22318_i16];
_3 = [_8,_1,_8,_1,_1,_9,_1];
RET = [(-23980_i16),12427_i16,(-26464_i16),(-26995_i16)];
_1 = !_9;
RET = [18018_i16,(-9923_i16),(-13761_i16),(-5493_i16)];
_2 = [_9,_1,_9,_1,_9,_8,_9];
_7 = !4042311315_u32;
Goto(bb3)
}
bb6 = {
_5 = [(*_13),(*_13),(*_13),(*_13),(*_13),(*_13),(*_13)];
_9 = _1;
_15 = [97_i8,(-29_i8),(-112_i8),(-123_i8),(-37_i8)];
_3 = [_8,_8,_9,_9,_8,_8,_8];
_12 = !91182314154589585278986798396278217103_u128;
(*_16) = 1488931175_u32 >> _6;
_18 = 0_usize * 16024444159191362000_usize;
_1 = _6 as isize;
_9 = !_1;
_2 = [_1,_9,_8,_9,_9,_1,_1];
_20 = [(-1161591253_i32),1673703270_i32,(-333130912_i32),(-50518601_i32),(-629033551_i32),(-594651242_i32),2100232863_i32,115067531_i32];
_4 = [_12,_12,_12,_12,_12,_12];
_14 = _17 - _17;
_14 = _12 as f64;
_1 = (-9_i8) as isize;
_6 = -(-165208378817323572957773902327933311818_i128);
(*_16) = 3762917185_u32;
RET = [(-29068_i16),(-1118_i16),(-1163_i16),28012_i16];
_6 = !123721162384289594951859349674747448614_i128;
_8 = (-3042419451958489272_i64) as isize;
_24.fld2 = (29096_i16,);
_12 = 43_i8 as u128;
(*_16) = 1716218291_u32 ^ 3574815534_u32;
_17 = _19 as f64;
_18 = 366947133229602330_usize;
Goto(bb7)
}
bb7 = {
_24.fld6 = core::ptr::addr_of_mut!(_24.fld1);
_24.fld5 = [67_u8];
_24.fld1 = (*_13) as usize;
_1 = _8;
_8 = _9;
_21 = _11;
_3 = [_8,_9,_9,_8,_8,_8,_9];
_24.fld0.3 = 39872_u16 as i64;
_24.fld0.1 = core::ptr::addr_of!((*_16));
_22 = [(*_16)];
_9 = !_8;
_12 = 111735994604538885771041033140286734542_u128 ^ 314883887513064836297462660792088420535_u128;
_27 = Adt52 { fld0: _24.fld2,fld1: _15 };
_8 = _1;
_18 = _24.fld1 * _24.fld1;
_27.fld0.0 = _24.fld2.0 ^ _24.fld2.0;
_24.fld2.0 = 11_u8 as i16;
_28 = [8579247430461980048_u64];
_21 = (*_13);
_14 = _27.fld0.0 as f64;
_8 = _1;
_24.fld3 = _5;
_25 = 16746732740990822492_u64 as isize;
_23 = [(-92_i8),104_i8,(-14_i8),(-91_i8),16_i8];
_19 = (*_16) as f32;
_15 = [46_i8,(-77_i8),(-30_i8),(-89_i8),22_i8];
_23 = _27.fld1;
_27 = Adt52 { fld0: _24.fld2,fld1: _23 };
Goto(bb8)
}
bb8 = {
_3 = _2;
_2 = _3;
_20 = [1702255710_i32,(-1177304318_i32),(-1268196052_i32),2011965606_i32,(-1231599320_i32),(-1298372642_i32),1465482692_i32,1355519723_i32];
RET = [_27.fld0.0,_24.fld2.0,_24.fld2.0,_24.fld2.0];
_9 = -_25;
_17 = _14 * _14;
_24.fld1 = !_18;
(*_16) = _24.fld0.3 as u32;
_6 = _19 as i128;
RET = [_27.fld0.0,_27.fld0.0,_24.fld2.0,_27.fld0.0];
(*_16) = 2190990815_u32;
_11 = _21;
_1 = _8;
_11 = _21;
_11 = _21;
_5 = [(*_13),_21,(*_13),_11,(*_13),_21,(*_13)];
_24.fld6 = core::ptr::addr_of_mut!(_24.fld1);
_24.fld6 = core::ptr::addr_of_mut!(_18);
_24.fld0.3 = false as i64;
match (*_16) {
0 => bb1,
1 => bb2,
2190990815 => bb9,
_ => bb4
}
}
bb9 = {
_29 = (*_16) > (*_16);
_22 = [(*_16)];
_3 = _2;
_25 = 30503_u16 as isize;
_17 = _14 * _14;
_23 = [69_i8,51_i8,19_i8,(-119_i8),(-75_i8)];
_25 = !_1;
_8 = _27.fld0.0 as isize;
_17 = _14 - _14;
_9 = _14 as isize;
_18 = _24.fld1 + _24.fld1;
_7 = !723813025_u32;
_18 = _24.fld1 + _24.fld1;
_24.fld0.1 = _16;
Goto(bb10)
}
bb10 = {
(*_13) = _21;
_12 = 336588125970590121836351363974889536569_u128;
_6 = (-100966119930526083997080045884688923975_i128) & 93452313809336002898362149092307536856_i128;
_27.fld1 = [110_i8,70_i8,(-15_i8),58_i8,43_i8];
_24.fld6 = core::ptr::addr_of_mut!(_18);
RET = [_24.fld2.0,_24.fld2.0,_24.fld2.0,_24.fld2.0];
Call(_2 = core::intrinsics::transmute(_3), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
(*_16) = 3781934440_u32 | 1111998724_u32;
_27.fld1 = [(-33_i8),(-116_i8),18_i8,(-61_i8),3_i8];
_16 = core::ptr::addr_of!((*_16));
_28 = [9350524777404096582_u64];
_41.2 = 17604076082538667664_u64 as u32;
_16 = _24.fld0.1;
_24.fld5 = [218_u8];
_31 = Adt42::Variant2 { fld0: 152_u8 };
_41.1 = [(*_13),(*_13),(*_13),(*_13)];
_15 = [(-73_i8),63_i8,109_i8,63_i8,58_i8];
_34 = (*_13);
_2 = [_8,_1,_9,_1,_25,_8,_9];
_24.fld0.3 = _19 as i64;
match _12 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
336588125970590121836351363974889536569 => bb18,
_ => bb17
}
}
bb12 = {
(*_13) = _21;
_12 = 336588125970590121836351363974889536569_u128;
_6 = (-100966119930526083997080045884688923975_i128) & 93452313809336002898362149092307536856_i128;
_27.fld1 = [110_i8,70_i8,(-15_i8),58_i8,43_i8];
_24.fld6 = core::ptr::addr_of_mut!(_18);
RET = [_24.fld2.0,_24.fld2.0,_24.fld2.0,_24.fld2.0];
Call(_2 = core::intrinsics::transmute(_3), ReturnTo(bb11), UnwindUnreachable())
}
bb13 = {
_29 = (*_16) > (*_16);
_22 = [(*_16)];
_3 = _2;
_25 = 30503_u16 as isize;
_17 = _14 * _14;
_23 = [69_i8,51_i8,19_i8,(-119_i8),(-75_i8)];
_25 = !_1;
_8 = _27.fld0.0 as isize;
_17 = _14 - _14;
_9 = _14 as isize;
_18 = _24.fld1 + _24.fld1;
_7 = !723813025_u32;
_18 = _24.fld1 + _24.fld1;
_24.fld0.1 = _16;
Goto(bb10)
}
bb14 = {
_3 = _2;
_2 = _3;
_20 = [1702255710_i32,(-1177304318_i32),(-1268196052_i32),2011965606_i32,(-1231599320_i32),(-1298372642_i32),1465482692_i32,1355519723_i32];
RET = [_27.fld0.0,_24.fld2.0,_24.fld2.0,_24.fld2.0];
_9 = -_25;
_17 = _14 * _14;
_24.fld1 = !_18;
(*_16) = _24.fld0.3 as u32;
_6 = _19 as i128;
RET = [_27.fld0.0,_27.fld0.0,_24.fld2.0,_27.fld0.0];
(*_16) = 2190990815_u32;
_11 = _21;
_1 = _8;
_11 = _21;
_11 = _21;
_5 = [(*_13),_21,(*_13),_11,(*_13),_21,(*_13)];
_24.fld6 = core::ptr::addr_of_mut!(_24.fld1);
_24.fld6 = core::ptr::addr_of_mut!(_18);
_24.fld0.3 = false as i64;
match (*_16) {
0 => bb1,
1 => bb2,
2190990815 => bb9,
_ => bb4
}
}
bb15 = {
_16 = core::ptr::addr_of!(_7);
_9 = 37119_u16 as isize;
(*_13) = '\u{cd918}';
(*_16) = 2613985379_u32;
RET = [15437_i16,(-30203_i16),(-10374_i16),(-23971_i16)];
_4 = [27762075790766565131464178035979650076_u128,297604828805042051831106170864882734039_u128,82650824787794692015426935832955446241_u128,169444873810236200064638470402903481458_u128,278917839150279274282735670435131727190_u128,226808112087783203421732967547190298301_u128];
_11 = '\u{5fd7a}';
_15 = [(-104_i8),35_i8,(-35_i8),(-124_i8),93_i8];
(*_13) = '\u{2482}';
_4 = [165531931061561213658869434370622744593_u128,273056586336186711718292016946856115495_u128,132384397307595773367280981013475021715_u128,206690287503774001524183565196206914354_u128,233316318347673172135386934529693480823_u128,62240120848037579534043652589260376740_u128];
_12 = 218569977778170380985037679416284311614_u128;
_11 = '\u{bd018}';
RET = [10973_i16,2974_i16,8205_i16,22331_i16];
_2 = [_8,_9,_1,_8,_9,_8,_9];
_19 = (-30608_i16) as f32;
_17 = 14199_i16 as f64;
match (*_16) {
0 => bb2,
2613985379 => bb6,
_ => bb5
}
}
bb16 = {
_5 = [(*_13),(*_13),(*_13),(*_13),(*_13),(*_13),(*_13)];
_9 = _1;
_15 = [97_i8,(-29_i8),(-112_i8),(-123_i8),(-37_i8)];
_3 = [_8,_8,_9,_9,_8,_8,_8];
_12 = !91182314154589585278986798396278217103_u128;
(*_16) = 1488931175_u32 >> _6;
_18 = 0_usize * 16024444159191362000_usize;
_1 = _6 as isize;
_9 = !_1;
_2 = [_1,_9,_8,_9,_9,_1,_1];
_20 = [(-1161591253_i32),1673703270_i32,(-333130912_i32),(-50518601_i32),(-629033551_i32),(-594651242_i32),2100232863_i32,115067531_i32];
_4 = [_12,_12,_12,_12,_12,_12];
_14 = _17 - _17;
_14 = _12 as f64;
_1 = (-9_i8) as isize;
_6 = -(-165208378817323572957773902327933311818_i128);
(*_16) = 3762917185_u32;
RET = [(-29068_i16),(-1118_i16),(-1163_i16),28012_i16];
_6 = !123721162384289594951859349674747448614_i128;
_8 = (-3042419451958489272_i64) as isize;
_24.fld2 = (29096_i16,);
_12 = 43_i8 as u128;
(*_16) = 1716218291_u32 ^ 3574815534_u32;
_17 = _19 as f64;
_18 = 366947133229602330_usize;
Goto(bb7)
}
bb17 = {
RET = [(-5420_i16),(-11064_i16),(-22542_i16),(-11064_i16)];
RET = [20334_i16,(-8819_i16),25532_i16,2146_i16];
_5 = ['\u{104e21}','\u{3dc2e}','\u{90e0b}','\u{d32ac}','\u{b36be}','\u{e510f}','\u{10889e}'];
_6 = 76011898294386501314139474700025673071_i128;
_6 = 80571284532791700506495354454743507764_i128 & 53368976251043107189203549278521197695_i128;
RET = [(-21815_i16),(-4048_i16),16331_i16,23335_i16];
_3 = [_9,_1,_9,_9,_9,_1,_1];
RET = [(-25022_i16),(-28420_i16),(-22246_i16),22318_i16];
_3 = [_8,_1,_8,_1,_1,_9,_1];
RET = [(-23980_i16),12427_i16,(-26464_i16),(-26995_i16)];
_1 = !_9;
RET = [18018_i16,(-9923_i16),(-13761_i16),(-5493_i16)];
_2 = [_9,_1,_9,_1,_9,_8,_9];
_7 = !4042311315_u32;
Goto(bb3)
}
bb18 = {
_35 = _25;
_25 = 165_u8 as isize;
_30 = 53_u8 & 202_u8;
_24.fld2.0 = !_27.fld0.0;
_24.fld0.2 = !(*_16);
_24.fld6 = core::ptr::addr_of_mut!(_24.fld1);
_2 = [_9,_35,_9,_8,_8,_9,_9];
_43 = _29;
_11 = _34;
_27.fld0 = (_24.fld2.0,);
_7 = 72_i8 as u32;
(*_13) = _21;
_33 = [12_i8,72_i8,51_i8,61_i8,119_i8];
_9 = _8;
_41.2 = _12 as u32;
_39 = core::ptr::addr_of!(_24.fld0);
_20 = [(-759974293_i32),988957370_i32,1560190253_i32,(-900208106_i32),(-596119211_i32),(-879873025_i32),(-1584166890_i32),1909101067_i32];
_20 = [(-1501378702_i32),(-1770608317_i32),(-1376658100_i32),(-1448819122_i32),226703340_i32,1207237330_i32,718815196_i32,1954582824_i32];
_41.1 = [_34,_21,(*_13),_34];
_18 = _24.fld1;
Goto(bb19)
}
bb19 = {
Call(_48 = dump_var(15_usize, 18_usize, Move(_18), 5_usize, Move(_5), 30_usize, Move(_30), 23_usize, Move(_23)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_48 = dump_var(15_usize, 11_usize, Move(_11), 34_usize, Move(_34), 22_usize, Move(_22), 1_usize, Move(_1)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_48 = dump_var(15_usize, 43_usize, Move(_43), 33_usize, Move(_33), 35_usize, Move(_35), 12_usize, Move(_12)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: bool,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: [u128; 6],mut _11: isize,mut _12: i16,mut _13: isize,mut _14: isize,mut _15: isize,mut _16: isize) -> [isize; 7] {
mir! {
type RET = [isize; 7];
let _17: isize;
let _18: bool;
let _19: ();
let _20: ();
{
_8 = 16742681544088473425_u64 as isize;
_13 = -_5;
_7 = 2011441029_u32 as isize;
Call(_11 = core::intrinsics::bswap(_13), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = _2 >> _5;
_2 = _4 as isize;
_2 = 5347579243466208364_u64 as isize;
_14 = _5;
RET = [_3,_16,_14,_11,_15,_11,_14];
_1 = _4 as isize;
_8 = 403699591908903847_i64 as isize;
_14 = _6 | _5;
_4 = !true;
_10 = [158018317322648949025739986784120517745_u128,280534169465544411595980124819020271920_u128,240138308038898497650763387661863097341_u128,211916451385019487484880200780468892163_u128,35957056355306143986837742455048609307_u128,150553799767006707468043250279814687619_u128];
_4 = false;
_4 = !true;
_4 = !true;
_6 = _11 ^ _16;
_14 = '\u{9c219}' as isize;
_1 = _6;
_6 = -_16;
_13 = -_15;
_3 = _9 >> _5;
_5 = !_15;
_11 = 1495405327_u32 as isize;
_6 = _3 * _16;
RET = [_15,_15,_5,_13,_1,_16,_9];
_14 = _6 - _5;
_5 = !_9;
RET = [_13,_9,_13,_15,_1,_1,_3];
_13 = _5;
_13 = _15 << _5;
_10 = [223881054134125051677743526012507844677_u128,51859849695979932854122408805724427587_u128,169937088053819833524775033561876458857_u128,71141759254359489270301969762396829041_u128,282603018489469681795873783292335816023_u128,256420093339755578629425038957163942323_u128];
Goto(bb2)
}
bb2 = {
Call(_19 = dump_var(16_usize, 6_usize, Move(_6), 2_usize, Move(_2), 1_usize, Move(_1), 11_usize, Move(_11)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_19 = dump_var(16_usize, 14_usize, Move(_14), 7_usize, Move(_7), 9_usize, Move(_9), 5_usize, Move(_5)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: isize,mut _2: isize) -> isize {
mir! {
type RET = isize;
let _3: ();
let _4: ();
{
RET = -_2;
RET = _2;
RET = !_1;
_2 = RET ^ _1;
_1 = !_2;
Goto(bb1)
}
bb1 = {
Call(_3 = dump_var(17_usize, 1_usize, Move(_1), 4_usize, _4, 4_usize, _4, 4_usize, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: bool,mut _2: (isize, [char; 4], u32),mut _3: u32,mut _4: u32,mut _5: (isize, [char; 4], u32),mut _6: (isize, [char; 4], u32),mut _7: usize) -> [char; 4] {
mir! {
type RET = [char; 4];
let _8: u32;
let _9: isize;
let _10: isize;
let _11: [char; 7];
let _12: [isize; 7];
let _13: isize;
let _14: [i16; 4];
let _15: i128;
let _16: f64;
let _17: f32;
let _18: [u64; 1];
let _19: *mut i128;
let _20: *mut usize;
let _21: i8;
let _22: Adt52;
let _23: i64;
let _24: (i16,);
let _25: isize;
let _26: Adt49;
let _27: u16;
let _28: [i8; 5];
let _29: u128;
let _30: ();
let _31: ();
{
_5 = (_6.0, _2.1, _6.2);
_2.0 = '\u{4c7e9}' as isize;
_6.2 = _4;
_2.1 = ['\u{e42f7}','\u{6b273}','\u{fc6f4}','\u{4b5}'];
_5.2 = !_2.2;
_2 = (_6.0, _6.1, _4);
_6.0 = _5.0;
_3 = _5.2;
_2 = _5;
_2.0 = _5.0;
_9 = -_5.0;
RET = ['\u{a5a33}','\u{10ff69}','\u{74323}','\u{efddf}'];
_5 = _6;
_1 = false ^ true;
_5.1 = ['\u{1016e4}','\u{64e01}','\u{fb6ea}','\u{f608a}'];
_10 = '\u{24134}' as isize;
_5 = (_6.0, _6.1, _4);
_5.1 = ['\u{fc24d}','\u{2e325}','\u{5fe79}','\u{105271}'];
_8 = _3 + _4;
_7 = 1_usize;
_12 = [_6.0,_9,_5.0,_2.0,_2.0,_2.0,_5.0];
_12[_7] = (-125_i8) as isize;
_3 = _8 & _6.2;
_1 = !false;
_2.2 = 5236463871828828681_u64 as u32;
Goto(bb1)
}
bb1 = {
_12[_7] = _2.0;
_12[_7] = 206_u8 as isize;
_4 = 224966083901366944855768281194500076971_u128 as u32;
_2.1[_7] = RET[_7];
match _7 {
0 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
1 => bb9,
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
_5.1 = [_6.1[_7],RET[_7],RET[_7],_2.1[_7]];
_13 = _7 as isize;
_1 = false;
match _7 {
1 => bb10,
_ => bb2
}
}
bb10 = {
_2.1[_7] = _5.1[_7];
Goto(bb11)
}
bb11 = {
_8 = !_5.2;
_11 = [_6.1[_7],RET[_7],_6.1[_7],_2.1[_7],_5.1[_7],_5.1[_7],RET[_7]];
_16 = 21_i8 as f64;
RET = [_5.1[_7],_2.1[_7],_11[_7],_5.1[_7]];
_5 = (_2.0, _2.1, _6.2);
_14 = [19936_i16,(-26758_i16),(-14405_i16),(-10630_i16)];
_4 = _6.2 ^ _8;
_12[_7] = (-8531731034635315516_i64) as isize;
_12[_7] = _2.0;
_2 = (_9, _6.1, _5.2);
_16 = 70084167389826122347638602695474752437_u128 as f64;
_6 = _5;
_5 = _2;
_2.1 = [_5.1[_7],_11[_7],_6.1[_7],_11[_7]];
_18 = [8184335832542034047_u64];
_8 = _3 >> _3;
_15 = (-94459886728849554372546989863099717977_i128);
_16 = 571967449512009492_u64 as f64;
_4 = _2.2;
_2.0 = _12[_7];
_11 = [_2.1[_7],_2.1[_7],_5.1[_7],RET[_7],RET[_7],RET[_7],_6.1[_7]];
_2.1 = [_11[_7],_5.1[_7],_11[_7],RET[_7]];
_12[_7] = _14[_7] as isize;
_20 = core::ptr::addr_of_mut!(_7);
_1 = !true;
_4 = !_2.2;
Goto(bb12)
}
bb12 = {
_7 = 3_usize | 4881282462413664324_usize;
RET = _2.1;
_6 = (_2.0, _5.1, _2.2);
(*_20) = (-4232668864332872173_i64) as usize;
_17 = 660_i16 as f32;
_22.fld0.0 = 29157_i16;
(*_20) = 17617722702888956701_u64 as usize;
_2 = (_5.0, _6.1, _4);
(*_20) = (-2282193455523778141_i64) as usize;
_21 = 2869258831042222983_u64 as i8;
RET = ['\u{6d168}','\u{5567a}','\u{8f050}','\u{3fc88}'];
_16 = 91_u8 as f64;
_2.2 = 143_u8 as u32;
_23 = !(-2061801551724918224_i64);
_12 = [_9,_2.0,_5.0,_9,_6.0,_9,_2.0];
_2.2 = _9 as u32;
_22.fld1 = [_21,_21,_21,_21,_21];
_6.1 = ['\u{80fa2}','\u{d51c6}','\u{bc3df}','\u{7e31f}'];
_4 = _2.2 - _8;
_19 = core::ptr::addr_of_mut!(_15);
_21 = _2.0 as i8;
_16 = _2.0 as f64;
_5.0 = _6.0;
_12 = [_9,_6.0,_6.0,_6.0,_9,_9,_5.0];
(*_19) = 46_u8 as i128;
Goto(bb13)
}
bb13 = {
_16 = 193_u8 as f64;
_2 = (_9, _5.1, _8);
_6 = (_5.0, _5.1, _8);
(*_20) = 1089070817_i32 as usize;
_5.0 = !_6.0;
_1 = true;
(*_20) = _21 as usize;
Call(_5.0 = core::intrinsics::bswap(_2.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_25 = _2.0;
_22.fld0.0 = (-14650_i16);
RET = ['\u{f251}','\u{10392b}','\u{81d7}','\u{e9769}'];
_5.2 = _6.2;
_19 = core::ptr::addr_of_mut!((*_19));
_4 = 1501480570_i32 as u32;
_22.fld1 = [_21,_21,_21,_21,_21];
_10 = -_6.0;
_14 = [_22.fld0.0,_22.fld0.0,_22.fld0.0,_22.fld0.0];
(*_20) = !4592034351460433007_usize;
_15 = 35117647404145507172332918211597988420_i128;
_22.fld0.0 = 17195_i16 + 30849_i16;
_28 = [_21,_21,_21,_21,_21];
RET = ['\u{340dd}','\u{9a3af}','\u{df372}','\u{3bbdf}'];
_5.0 = !_2.0;
RET = ['\u{10c7e2}','\u{90a4d}','\u{c43f7}','\u{936cc}'];
_4 = _8 | _2.2;
_5.1 = RET;
_22.fld0 = ((-29138_i16),);
_28 = [_21,_21,_21,_21,_21];
_25 = _16 as isize;
_20 = core::ptr::addr_of_mut!(_7);
_5 = (_6.0, _2.1, _3);
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(18_usize, 7_usize, Move(_7), 23_usize, Move(_23), 14_usize, Move(_14), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(18_usize, 11_usize, Move(_11), 2_usize, Move(_2), 18_usize, Move(_18), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(18_usize, 25_usize, Move(_25), 15_usize, Move(_15), 31_usize, _31, 31_usize, _31), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: Adt46,mut _2: *const u32,mut _3: bool,mut _4: [isize; 7],mut _5: *const u32,mut _6: isize,mut _7: isize,mut _8: (u32, *const u32, u32, i64),mut _9: [isize; 7],mut _10: u32,mut _11: isize) -> *const u32 {
mir! {
type RET = *const u32;
let _12: f32;
let _13: f64;
let _14: [i8; 5];
let _15: *mut [isize; 7];
let _16: isize;
let _17: u64;
let _18: isize;
let _19: u64;
let _20: isize;
let _21: [i16; 4];
let _22: [u32; 1];
let _23: [i8; 5];
let _24: f32;
let _25: [i32; 8];
let _26: bool;
let _27: [bool; 5];
let _28: (u32, *const u32, u32, i64);
let _29: isize;
let _30: isize;
let _31: [u8; 1];
let _32: Adt53;
let _33: (f32, *const (u32, *const u32, u32, i64), [char; 4]);
let _34: isize;
let _35: [u8; 1];
let _36: isize;
let _37: isize;
let _38: ();
let _39: ();
{
RET = core::ptr::addr_of!((*_2));
_1.fld0.1 = _8.1;
(*_5) = '\u{d143b}' as u32;
(*_2) = _1.fld0.0;
_4 = [_6,_7,_11,_7,_6,_11,_11];
_8.0 = _8.2 + (*_2);
_1.fld0.2 = (*_5) - (*_2);
RET = core::ptr::addr_of!((*_2));
_4 = _1.fld4;
_4 = _1.fld4;
_1.fld6 = core::ptr::addr_of_mut!(_1.fld1);
_6 = _1.fld2.0 as isize;
_1.fld4 = [_7,_11,_7,_7,_7,_11,_11];
(*RET) = _10 * _8.2;
_8.0 = !(*_5);
_12 = 113844742265820396980619130989238886530_i128 as f32;
_11 = _7;
Goto(bb1)
}
bb1 = {
_8.1 = core::ptr::addr_of!((*_5));
_1.fld0 = ((*_5), _2, _8.0, _8.3);
(*RET) = !_1.fld0.0;
(*RET) = !_1.fld0.0;
_10 = !_1.fld0.2;
_1.fld0.1 = _5;
_1.fld0 = _8;
(*_2) = !_8.2;
_13 = _12 as f64;
_3 = true;
Call(_8.3 = core::intrinsics::transmute(_7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_11 = _7;
RET = _2;
_4 = [_7,_6,_7,_6,_11,_7,_7];
_1.fld2 = ((-25081_i16),);
(*RET) = '\u{c1a9f}' as u32;
_3 = !true;
_12 = 54106_u16 as f32;
RET = core::ptr::addr_of!(_10);
_8.3 = 18308803302181851785_u64 as i64;
_4 = [_6,_11,_6,_11,_6,_7,_7];
_12 = _1.fld2.0 as f32;
_1.fld2 = ((-28727_i16),);
_1.fld0 = ((*RET), RET, _10, _8.3);
_5 = RET;
_16 = -_6;
_13 = 318840109725111688472913066397064140699_u128 as f64;
_15 = core::ptr::addr_of_mut!(_9);
_15 = core::ptr::addr_of_mut!(_1.fld4);
_19 = 153512051557850861642714246715870185119_u128 as u64;
_1.fld6 = core::ptr::addr_of_mut!(_1.fld1);
_20 = _7;
_15 = core::ptr::addr_of_mut!((*_15));
_15 = core::ptr::addr_of_mut!(_4);
(*RET) = _19 as u32;
_17 = _19;
match _1.fld2.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768182729 => bb7,
_ => bb6
}
}
bb3 = {
_8.1 = core::ptr::addr_of!((*_5));
_1.fld0 = ((*_5), _2, _8.0, _8.3);
(*RET) = !_1.fld0.0;
(*RET) = !_1.fld0.0;
_10 = !_1.fld0.2;
_1.fld0.1 = _5;
_1.fld0 = _8;
(*_2) = !_8.2;
_13 = _12 as f64;
_3 = true;
Call(_8.3 = core::intrinsics::transmute(_7), ReturnTo(bb2), UnwindUnreachable())
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
_18 = _20;
_12 = 108_i8 as f32;
_9 = (*_15);
(*RET) = _8.2 - _1.fld0.0;
_20 = _7 * _7;
RET = core::ptr::addr_of!(_8.2);
_1.fld0.3 = -_8.3;
_23 = [(-55_i8),32_i8,(-119_i8),(-66_i8),61_i8];
RET = _5;
_8.3 = _1.fld0.0 as i64;
(*_2) = _1.fld0.0;
Goto(bb8)
}
bb8 = {
RET = core::ptr::addr_of!(_8.2);
_24 = _12 * _12;
_6 = _13 as isize;
_10 = '\u{a06e7}' as u32;
_25 = [(-256141104_i32),(-1743322985_i32),(-521070428_i32),1762091244_i32,86709345_i32,1841394983_i32,700040828_i32,1651955703_i32];
(*RET) = (*_2) << _8.0;
_4 = _1.fld4;
_12 = _24;
_14 = [79_i8,(-52_i8),(-51_i8),(-47_i8),71_i8];
_14 = _23;
_16 = '\u{31608}' as isize;
_17 = _19;
_28 = (_8.2, RET, (*_2), _8.3);
_27 = [_3,_3,_3,_3,_3];
(*RET) = _1.fld0.3 as u32;
(*_15) = _1.fld4;
(*_15) = _9;
_1.fld4 = [_20,_20,_18,_11,_16,_16,_20];
Goto(bb9)
}
bb9 = {
_28.2 = !_28.0;
_1.fld4 = [_20,_16,_20,_7,_18,_11,_16];
_5 = _2;
_26 = _8.3 != _28.3;
_15 = core::ptr::addr_of_mut!(_9);
_21 = [_1.fld2.0,_1.fld2.0,_1.fld2.0,_1.fld2.0];
_28.1 = core::ptr::addr_of!(_10);
RET = _5;
_1.fld6 = core::ptr::addr_of_mut!(_1.fld1);
(*_15) = [_6,_11,_6,_16,_20,_18,_20];
match _1.fld2.0 {
0 => bb6,
1 => bb10,
2 => bb11,
3 => bb12,
340282366920938463463374607431768182729 => bb14,
_ => bb13
}
}
bb10 = {
_8.1 = core::ptr::addr_of!((*_5));
_1.fld0 = ((*_5), _2, _8.0, _8.3);
(*RET) = !_1.fld0.0;
(*RET) = !_1.fld0.0;
_10 = !_1.fld0.2;
_1.fld0.1 = _5;
_1.fld0 = _8;
(*_2) = !_8.2;
_13 = _12 as f64;
_3 = true;
Call(_8.3 = core::intrinsics::transmute(_7), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_18 = _20;
_12 = 108_i8 as f32;
_9 = (*_15);
(*RET) = _8.2 - _1.fld0.0;
_20 = _7 * _7;
RET = core::ptr::addr_of!(_8.2);
_1.fld0.3 = -_8.3;
_23 = [(-55_i8),32_i8,(-119_i8),(-66_i8),61_i8];
RET = _5;
_8.3 = _1.fld0.0 as i64;
(*_2) = _1.fld0.0;
Goto(bb8)
}
bb12 = {
Return()
}
bb13 = {
_11 = _7;
RET = _2;
_4 = [_7,_6,_7,_6,_11,_7,_7];
_1.fld2 = ((-25081_i16),);
(*RET) = '\u{c1a9f}' as u32;
_3 = !true;
_12 = 54106_u16 as f32;
RET = core::ptr::addr_of!(_10);
_8.3 = 18308803302181851785_u64 as i64;
_4 = [_6,_11,_6,_11,_6,_7,_7];
_12 = _1.fld2.0 as f32;
_1.fld2 = ((-28727_i16),);
_1.fld0 = ((*RET), RET, _10, _8.3);
_5 = RET;
_16 = -_6;
_13 = 318840109725111688472913066397064140699_u128 as f64;
_15 = core::ptr::addr_of_mut!(_9);
_15 = core::ptr::addr_of_mut!(_1.fld4);
_19 = 153512051557850861642714246715870185119_u128 as u64;
_1.fld6 = core::ptr::addr_of_mut!(_1.fld1);
_20 = _7;
_15 = core::ptr::addr_of_mut!((*_15));
_15 = core::ptr::addr_of_mut!(_4);
(*RET) = _19 as u32;
_17 = _19;
match _1.fld2.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768182729 => bb7,
_ => bb6
}
}
bb14 = {
_26 = _3;
_1.fld2.0 = -2862_i16;
_31 = [243_u8];
_22 = [_1.fld0.0];
_2 = core::ptr::addr_of!((*RET));
(*RET) = _28.0 - _8.0;
_1.fld3 = ['\u{d4860}','\u{5c22f}','\u{bb9c3}','\u{cea9e}','\u{dfa5b}','\u{58ef7}','\u{8d0e7}'];
_30 = _20 >> (*_5);
_24 = _12;
_8.1 = core::ptr::addr_of!(_1.fld0.0);
_34 = -_20;
_28 = _1.fld0;
_17 = _19;
(*_5) = 195353455354385775110002037804960392664_u128 as u32;
_8.3 = -_28.3;
_33.1 = core::ptr::addr_of!(_28);
_1.fld5 = [61_u8];
_36 = _30 ^ _30;
_15 = core::ptr::addr_of_mut!(_1.fld4);
_1.fld0.0 = _28.0;
_23 = [124_i8,(-56_i8),(-98_i8),(-112_i8),32_i8];
RET = core::ptr::addr_of!(_1.fld0.2);
_1.fld0.1 = _2;
(*RET) = !(*_2);
_33.0 = 114479168660095400489667546719101204002_i128 as f32;
_6 = _36;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(19_usize, 17_usize, Move(_17), 23_usize, Move(_23), 27_usize, Move(_27), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(19_usize, 11_usize, Move(_11), 20_usize, Move(_20), 30_usize, Move(_30), 36_usize, Move(_36)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(19_usize, 26_usize, Move(_26), 22_usize, Move(_22), 16_usize, Move(_16), 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{27fce}'), std::hint::black_box((-71_isize)), std::hint::black_box(33_i8), std::hint::black_box((-3088_i16)), std::hint::black_box((-591785545_i32)), std::hint::black_box((-540064175007105377_i64)), std::hint::black_box((-31105498285704830757615670539222397836_i128)), std::hint::black_box(8965937636604532271_usize), std::hint::black_box(63_u8), std::hint::black_box(23761_u16), std::hint::black_box(2136941839_u32), std::hint::black_box(4379959865186618872_u64), std::hint::black_box(203528205019140930835281029389048191452_u128));
                
            }
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: [u64; 1],
fld1: *mut usize,
fld2: isize,
fld3: (isize, [char; 4], u32),
fld4: (u32, *const u32, u32, i64),
fld5: *const char,
fld6: *mut *mut i128,
fld7: *const (u32, *const u32, u32, i64),

},
Variant1{
fld0: [i16; 4],
fld1: i128,

},
Variant2{
fld0: u8,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: [u64; 1],
fld1: f32,
fld2: u32,
fld3: [char; 4],
fld4: (u32, *const u32, u32, i64),
fld5: u16,
fld6: f64,
fld7: *const u32,

},
Variant1{
fld0: *mut usize,
fld1: Adt42,
fld2: isize,
fld3: [u128; 6],
fld4: i16,
fld5: u32,
fld6: usize,

},
Variant2{
fld0: [u32; 1],
fld1: [u128; 6],
fld2: u8,
fld3: usize,
fld4: i16,
fld5: [char; 7],

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: *mut *mut i128,
fld1: *mut i128,
fld2: *const (isize, [char; 4], u32),
fld3: [bool; 5],
fld4: (*mut usize, u16, i32),

},
Variant1{
fld0: bool,
fld1: usize,
fld2: (isize, [char; 4], u32),
fld3: i64,
fld4: *const char,
fld5: *mut *mut i128,

},
Variant2{
fld0: *const (u32, *const u32, u32, i64),
fld1: (f32, *const (u32, *const u32, u32, i64), [char; 4]),
fld2: [i16; 4],

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: [u128; 6],
fld1: [u32; 1],
fld2: [isize; 7],
fld3: *mut [isize; 7],

},
Variant1{
fld0: (i16,),
fld1: i16,
fld2: [bool; 5],
fld3: [char; 7],

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: (u32, *const u32, u32, i64),
fld1: usize,
fld2: (i16,),
fld3: [char; 7],
fld4: [isize; 7],
fld5: [u8; 1],
fld6: *mut usize,
}
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
fld0: i64,
fld1: Adt42,

},
Variant1{
fld0: [isize; 7],
fld1: Adt44,
fld2: Adt45,
fld3: Adt43,

},
Variant2{
fld0: (*mut usize, u16, i32),
fld1: *mut [isize; 7],
fld2: Adt45,
fld3: (f32, *const (u32, *const u32, u32, i64), [char; 4]),
fld4: [u32; 1],
fld5: u64,
fld6: u128,
fld7: u32,

},
Variant3{
fld0: bool,
fld1: *mut i128,
fld2: [i8; 5],
fld3: u8,
fld4: Adt44,
fld5: *const (isize, [char; 4], u32),

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: (i16,),
}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: *mut i128,

},
Variant1{
fld0: *mut i128,
fld1: (isize, [char; 4], u32),
fld2: [i8; 5],
fld3: (*mut usize, u16, i32),

},
Variant2{
fld0: i32,

},
Variant3{
fld0: *mut usize,
fld1: [u8; 1],

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: u128,
fld1: Adt47,
fld2: [i16; 4],
fld3: i8,
fld4: u64,
fld5: i32,
fld6: [u128; 6],
fld7: *const char,
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: Adt42,
fld1: u8,
fld2: isize,
fld3: Adt44,
fld4: f32,
fld5: [u32; 1],
fld6: usize,
fld7: *mut *mut i128,

},
Variant1{
fld0: Adt46,
fld1: [u64; 1],
fld2: u64,
fld3: [bool; 5],
fld4: *const (u32, *const u32, u32, i64),
fld5: [u8; 1],

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: (i16,),
fld1: [i8; 5],
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: Adt51,
fld1: (*mut usize, u16, i32),
fld2: Adt45,

},
Variant1{
fld0: *mut [isize; 7],
fld1: [i8; 5],
fld2: *mut i128,
fld3: *const (u32, *const u32, u32, i64),
fld4: Adt49,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: i128,
fld1: u64,
fld2: Adt43,

},
Variant1{
fld0: bool,
fld1: u128,
fld2: [bool; 5],
fld3: [i16; 4],
fld4: i16,
fld5: Adt46,
fld6: usize,
fld7: [isize; 7],

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
fld0: [i16; 4],
fld1: Adt42,

},
Variant1{
fld0: f64,
fld1: u16,
fld2: isize,
fld3: *mut [isize; 7],
fld4: i64,
fld5: *const char,

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
fld0: bool,
fld1: char,
fld2: isize,
fld3: [u64; 1],
fld4: (u32, *const u32, u32, i64),
fld5: i32,

},
Variant1{
fld0: *const (u32, *const u32, u32, i64),
fld1: (f32, *const (u32, *const u32, u32, i64), [char; 4]),
fld2: u64,
fld3: *mut [isize; 7],
fld4: i16,
fld5: (u32, *const u32, u32, i64),
fld6: usize,
fld7: [i32; 8],

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: f64,
fld1: Adt45,
fld2: Adt54,
fld3: (isize, [char; 4], u32),
fld4: i16,
fld5: Adt51,
fld6: [i8; 5],

},
Variant1{
fld0: Adt56,
fld1: *mut [isize; 7],
fld2: u16,
fld3: Adt45,
fld4: Adt51,

},
Variant2{
fld0: i128,
fld1: char,
fld2: i64,
fld3: *mut [isize; 7],
fld4: (f32, *const (u32, *const u32, u32, i64), [char; 4]),

},
Variant3{
fld0: *const u32,
fld1: (*mut usize, u16, i32),
fld2: u32,
fld3: u128,
fld4: (isize, [char; 4], u32),
fld5: *const (u32, *const u32, u32, i64),
fld6: Adt51,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt58::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: bool,
fld1: Adt49,
fld2: [u32; 1],
fld3: Adt52,
fld4: i16,
fld5: Adt48,
fld6: Adt57,

},
Variant1{
fld0: *mut usize,
fld1: Adt56,
fld2: [char; 4],
fld3: f32,
fld4: u128,

},
Variant2{
fld0: Adt56,
fld1: i64,
fld2: Adt51,
fld3: i8,
fld4: [u8; 1],

},
Variant3{
fld0: [char; 7],
fld1: char,

}}

