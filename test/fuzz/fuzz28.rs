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
pub fn fn0(mut _1: u64,mut _2: char,mut _3: u32,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: u8,mut _8: i128,mut _9: u128) -> (i128, u128, char) {
mir! {
type RET = (i128, u128, char);
let _10: isize;
let _11: u32;
let _12: (i64,);
let _13: isize;
let _14: usize;
let _15: Adt46;
let _16: f32;
let _17: f32;
let _18: isize;
let _19: ();
let _20: ();
{
_9 = (-3452154634372895101_i64) as u128;
Call(RET.0 = fn1(_9, _9, _9, _9, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = _9 as i128;
_6 = 781318557_i32 ^ (-1189272192_i32);
_7 = 26249_i16 as u8;
RET = (_8, _9, '\u{4957d}');
_5 = _6 as i16;
_7 = _5 as u8;
RET.2 = '\u{1396c}';
_2 = RET.2;
_6 = (-1519851840_i32);
_3 = 3144590332_u32;
RET.2 = _2;
RET.2 = _2;
RET.0 = !_8;
_2 = RET.2;
_2 = RET.2;
_12.0 = !5575045545517811949_i64;
_5 = 6_usize as i16;
RET.2 = _2;
_13 = (-9223372036854775808_isize);
_9 = RET.1 | RET.1;
_5 = !(-23278_i16);
_1 = 1784012394446571995_u64 >> _7;
RET.2 = _2;
_12.0 = !7826648223154580201_i64;
RET.1 = _9;
_1 = 11235250772357745768_u64;
_1 = !15149330058906102646_u64;
RET = (_8, _9, _2);
match _3 {
0 => bb2,
1 => bb3,
3144590332 => bb5,
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
_12.0 = 1579502401092115181_i64 << _5;
RET.2 = _2;
_14 = !11467093324889783022_usize;
_6 = -281317870_i32;
_11 = _8 as u32;
_4 = -(-115_i8);
_6 = !(-307427450_i32);
_15.fld1 = _2;
RET = (_8, _9, _15.fld1);
_11 = _3;
_13 = (-9223372036854775808_isize) & 64_isize;
_11 = _7 as u32;
_12.0 = (-7143453315797140832_i64) ^ 7228231868140797085_i64;
RET.2 = _2;
match _3 {
0 => bb2,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
3144590332 => bb13,
_ => bb12
}
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
_8 = _9 as i128;
_6 = 781318557_i32 ^ (-1189272192_i32);
_7 = 26249_i16 as u8;
RET = (_8, _9, '\u{4957d}');
_5 = _6 as i16;
_7 = _5 as u8;
RET.2 = '\u{1396c}';
_2 = RET.2;
_6 = (-1519851840_i32);
_3 = 3144590332_u32;
RET.2 = _2;
RET.2 = _2;
RET.0 = !_8;
_2 = RET.2;
_2 = RET.2;
_12.0 = !5575045545517811949_i64;
_5 = 6_usize as i16;
RET.2 = _2;
_13 = (-9223372036854775808_isize);
_9 = RET.1 | RET.1;
_5 = !(-23278_i16);
_1 = 1784012394446571995_u64 >> _7;
RET.2 = _2;
_12.0 = !7826648223154580201_i64;
RET.1 = _9;
_1 = 11235250772357745768_u64;
_1 = !15149330058906102646_u64;
RET = (_8, _9, _2);
match _3 {
0 => bb2,
1 => bb3,
3144590332 => bb5,
_ => bb4
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
_1 = !9275969395507432413_u64;
RET.2 = _2;
_12 = ((-746353868585568514_i64),);
_13 = 55_isize | (-95_isize);
_12 = ((-1568213073834513635_i64),);
match _3 {
3144590332 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
_8 = _7 as i128;
_1 = 8790676687330182948_u64;
_12 = ((-1502714096050029119_i64),);
_15.fld0 = _6 as f64;
_3 = _11 | _11;
_4 = 43480_u16 as i8;
_10 = _13;
_4 = 9_i8 ^ 3_i8;
RET = (_8, _9, _15.fld1);
_14 = !13506947511796935506_usize;
_16 = _6 as f32;
_1 = 5369208173194568974_u64;
_12 = (9051975985480106164_i64,);
RET.0 = _8;
_12.0 = 5267176153948078953_i64;
_4 = (-80_i8);
_11 = _14 as u32;
RET.2 = _2;
RET.2 = _15.fld1;
_13 = _10 << _8;
_4 = !100_i8;
_15.fld1 = _2;
RET.1 = !_9;
Goto(bb16)
}
bb16 = {
Call(_19 = dump_var(0_usize, 10_usize, Move(_10), 12_usize, Move(_12), 4_usize, Move(_4), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_19 = dump_var(0_usize, 11_usize, Move(_11), 8_usize, Move(_8), 1_usize, Move(_1), 20_usize, _20), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: u128,mut _2: u128,mut _3: u128,mut _4: u128,mut _5: u128) -> i128 {
mir! {
type RET = i128;
let _6: (bool, i64);
let _7: Adt44;
let _8: Adt58;
let _9: f32;
let _10: f32;
let _11: (i64,);
let _12: [i16; 7];
let _13: (bool, i64);
let _14: f32;
let _15: isize;
let _16: [u8; 3];
let _17: Adt52;
let _18: (u64, char, i128, i128, char, u32, u8, u8);
let _19: f64;
let _20: [i32; 4];
let _21: i64;
let _22: Adt47;
let _23: ();
let _24: ();
{
_3 = _2 + _1;
_6.1 = (-817163514004864680_i64) & 8408205966472939878_i64;
_4 = _3;
_6 = (true, 2702753469083870054_i64);
RET = 566079239_i32 as i128;
RET = (-81632497204629471520574461568816587533_i128) * 74975302825073085395427990221795076090_i128;
_4 = 2897444053124550539_u64 as u128;
RET = 81349782049008864255781311837860241774_i128;
_2 = _3 | _3;
_4 = _2 * _3;
_6.0 = false & true;
RET = !(-93872372068539859403858626761017118758_i128);
_6 = (false, (-1217939362675069435_i64));
_6 = (true, 520038798176209574_i64);
_1 = '\u{647e}' as u128;
RET = -66917140936999375667639274933917760646_i128;
_1 = _4;
_1 = _4 * _2;
_8.fld3.fld3.0.0 = _3;
_8.fld3.fld2 = (11003960802435981578_u64, '\u{10e8e6}', RET, RET, '\u{e8192}', 2651439342_u32, 135_u8, 104_u8);
_8.fld3.fld2 = (4272437442565260737_u64, '\u{cf27}', RET, RET, '\u{29dd3}', 3328734506_u32, 239_u8, 35_u8);
_8.fld2.7 = !_8.fld3.fld2.6;
_8.fld3.fld2.7 = _8.fld2.7;
_4 = _8.fld3.fld3.0.0;
Call(_8.fld4.fld6 = fn2(_8.fld3.fld2.6, _8.fld3.fld2.0, _4, _8.fld3.fld2.6, _8.fld3.fld2.1, _8.fld3.fld2.0, _8.fld3.fld2.4, _8.fld3.fld2.4, _8.fld3.fld2.0, _8.fld3.fld2.5, _8.fld3.fld2.4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = _8.fld2.7 as f32;
_8.fld4.fld2.0 = !_8.fld3.fld2.0;
_8.fld3.fld3.0.2 = core::ptr::addr_of_mut!(_8.fld3.fld3.1);
_8.fld3.fld3.2 = _8.fld3.fld2.0;
_8.fld2.1 = _8.fld3.fld2.1;
_8.fld2.4 = _8.fld2.1;
_8.fld2.3 = _8.fld3.fld2.2 & RET;
_8.fld3.fld0 = !6864372763241431614_usize;
_8.fld2 = (_8.fld4.fld2.0, _8.fld3.fld2.1, _8.fld3.fld2.2, _8.fld3.fld2.2, _8.fld3.fld2.4, _8.fld3.fld2.5, _8.fld3.fld2.7, _8.fld3.fld2.7);
_11 = (_6.1,);
_8.fld4.fld2.1 = _8.fld3.fld2.4;
_8.fld3.fld3.3 = core::ptr::addr_of_mut!(_8.fld3.fld3.1);
_11 = (_6.1,);
_4 = _1 ^ _1;
_8.fld4.fld2.5 = !_8.fld3.fld2.5;
_4 = _2;
_8.fld2 = (_8.fld3.fld2.0, _8.fld3.fld2.1, _8.fld3.fld2.2, RET, _8.fld3.fld2.4, _8.fld4.fld2.5, _8.fld3.fld2.7, _8.fld3.fld2.6);
_8.fld2.4 = _8.fld3.fld2.1;
_8.fld3.fld1 = _8.fld2.4 as u8;
RET = _8.fld3.fld2.3;
_8.fld3.fld2.2 = !_8.fld2.2;
_8.fld2.1 = _8.fld2.4;
_8.fld2.4 = _8.fld3.fld2.4;
_8.fld2.7 = !_8.fld3.fld1;
_8.fld2.2 = RET + RET;
Goto(bb2)
}
bb2 = {
_10 = _1 as f32;
_8.fld4.fld2.7 = _8.fld2.7 >> _8.fld3.fld3.2;
_15 = (-9223372036854775808_isize) & (-15_isize);
_8.fld2.1 = _8.fld2.4;
_8.fld2.0 = _8.fld3.fld3.2;
_14 = -_10;
_8.fld3.fld3.2 = _8.fld3.fld2.0;
Call(_8.fld4.fld2.0 = fn9(_8.fld4.fld6, _8.fld3.fld3.2, _11.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_8.fld4.fld2.0 = !_8.fld3.fld2.0;
_8.fld4.fld2.3 = !_8.fld3.fld2.3;
_18.0 = _6.1 as u64;
_16 = [_8.fld4.fld2.7,_8.fld2.7,_8.fld3.fld2.6];
_2 = _1;
_12 = [27986_i16,(-17953_i16),(-27471_i16),(-27613_i16),(-14058_i16),9811_i16,(-21609_i16)];
_8.fld2.6 = _8.fld3.fld2.6;
_18.3 = !_8.fld2.2;
RET = _18.3;
_18 = _8.fld2;
_8.fld3.fld3.0.0 = _1;
_8.fld2.6 = _8.fld2.7;
_8.fld3.fld3.0.1 = -_14;
_11.0 = _15 as i64;
match _8.fld3.fld3.2 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
4272437442565260737 => bb10,
_ => bb9
}
}
bb4 = {
_10 = _1 as f32;
_8.fld4.fld2.7 = _8.fld2.7 >> _8.fld3.fld3.2;
_15 = (-9223372036854775808_isize) & (-15_isize);
_8.fld2.1 = _8.fld2.4;
_8.fld2.0 = _8.fld3.fld3.2;
_14 = -_10;
_8.fld3.fld3.2 = _8.fld3.fld2.0;
Call(_8.fld4.fld2.0 = fn9(_8.fld4.fld6, _8.fld3.fld3.2, _11.0), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_10 = _8.fld2.7 as f32;
_8.fld4.fld2.0 = !_8.fld3.fld2.0;
_8.fld3.fld3.0.2 = core::ptr::addr_of_mut!(_8.fld3.fld3.1);
_8.fld3.fld3.2 = _8.fld3.fld2.0;
_8.fld2.1 = _8.fld3.fld2.1;
_8.fld2.4 = _8.fld2.1;
_8.fld2.3 = _8.fld3.fld2.2 & RET;
_8.fld3.fld0 = !6864372763241431614_usize;
_8.fld2 = (_8.fld4.fld2.0, _8.fld3.fld2.1, _8.fld3.fld2.2, _8.fld3.fld2.2, _8.fld3.fld2.4, _8.fld3.fld2.5, _8.fld3.fld2.7, _8.fld3.fld2.7);
_11 = (_6.1,);
_8.fld4.fld2.1 = _8.fld3.fld2.4;
_8.fld3.fld3.3 = core::ptr::addr_of_mut!(_8.fld3.fld3.1);
_11 = (_6.1,);
_4 = _1 ^ _1;
_8.fld4.fld2.5 = !_8.fld3.fld2.5;
_4 = _2;
_8.fld2 = (_8.fld3.fld2.0, _8.fld3.fld2.1, _8.fld3.fld2.2, RET, _8.fld3.fld2.4, _8.fld4.fld2.5, _8.fld3.fld2.7, _8.fld3.fld2.6);
_8.fld2.4 = _8.fld3.fld2.1;
_8.fld3.fld1 = _8.fld2.4 as u8;
RET = _8.fld3.fld2.3;
_8.fld3.fld2.2 = !_8.fld2.2;
_8.fld2.1 = _8.fld2.4;
_8.fld2.4 = _8.fld3.fld2.4;
_8.fld2.7 = !_8.fld3.fld1;
_8.fld2.2 = RET + RET;
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
_8.fld4.fld2.3 = _8.fld2.0 as i128;
_18.6 = _8.fld3.fld2.7 - _8.fld3.fld2.6;
_18.3 = _8.fld4.fld2.3 ^ _8.fld4.fld2.3;
_8.fld2.4 = _8.fld2.1;
RET = _18.3 ^ _8.fld4.fld2.3;
_6.1 = _11.0 & _11.0;
_8.fld4.fld1 = !_18.5;
_18.5 = _8.fld3.fld2.5;
_12 = [(-25353_i16),8403_i16,20531_i16,(-20463_i16),(-17974_i16),(-26267_i16),(-26567_i16)];
_6 = (false, _11.0);
_11 = (_6.1,);
_6.1 = !_11.0;
_13.0 = !_6.0;
Goto(bb11)
}
bb11 = {
Call(_23 = dump_var(1_usize, 5_usize, Move(_5), 3_usize, Move(_3), 15_usize, Move(_15), 6_usize, Move(_6)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_23 = dump_var(1_usize, 16_usize, Move(_16), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: u8,mut _2: u64,mut _3: u128,mut _4: u8,mut _5: char,mut _6: u64,mut _7: char,mut _8: char,mut _9: u64,mut _10: u32,mut _11: char) -> [i128; 6] {
mir! {
type RET = [i128; 6];
let _12: [i32; 1];
let _13: ();
let _14: ();
{
_5 = _7;
_11 = _7;
_1 = !_4;
Call(RET = fn3(_2, _11, _4, _5, _7, _9, _7, _5, _10, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12 = [(-1591216753_i32)];
_6 = 15208828464876301_i64 as u64;
Goto(bb2)
}
bb2 = {
Call(_13 = dump_var(2_usize, 11_usize, Move(_11), 8_usize, Move(_8), 1_usize, Move(_1), 5_usize, Move(_5)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_13 = dump_var(2_usize, 12_usize, Move(_12), 9_usize, Move(_9), 14_usize, _14, 14_usize, _14), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: u64,mut _2: char,mut _3: u8,mut _4: char,mut _5: char,mut _6: u64,mut _7: char,mut _8: char,mut _9: u32,mut _10: u64) -> [i128; 6] {
mir! {
type RET = [i128; 6];
let _11: isize;
let _12: Adt55;
let _13: u16;
let _14: usize;
let _15: (i64,);
let _16: bool;
let _17: [i16; 7];
let _18: isize;
let _19: (i128, u128, char);
let _20: u128;
let _21: Adt57;
let _22: isize;
let _23: Adt59;
let _24: *const i128;
let _25: Adt58;
let _26: i16;
let _27: (u16, u128, bool, u64);
let _28: Adt47;
let _29: f32;
let _30: Adt54;
let _31: f32;
let _32: i16;
let _33: ();
let _34: ();
{
_5 = _7;
_5 = _4;
_3 = 73_i8 as u8;
_4 = _8;
_9 = 47248550_u32;
_2 = _7;
_6 = !_1;
_1 = _6 ^ _10;
_8 = _4;
RET = [137691817705659354853088057983916621191_i128,(-61605711647364693827207397817919102202_i128),64906354363077943966190693824754485018_i128,(-81268191366436733923316866946197375743_i128),76100210087497243940075154507317410447_i128,(-127625191373860251209689586147718405356_i128)];
_4 = _8;
_11 = 38_isize >> _6;
_10 = _1;
_12.fld0.0 = 7785689088903391582_usize as u64;
_3 = !199_u8;
_11 = _3 as isize;
RET = [150131649534661506218374833789934107327_i128,(-96395922024497364355908007027364523853_i128),(-165568474092831482921583019854587096842_i128),(-130506760775511013149674133387518552311_i128),141299275265867885898566234838862676772_i128,27314896201387179390098859094902730205_i128];
_12.fld0.0 = !_6;
_2 = _4;
_1 = _10;
_12.fld0.1 = _4 as i64;
_5 = _2;
match _9 {
0 => bb1,
47248550 => bb3,
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
_12.fld2 = [(-42216390188989456924295591122660228254_i128),132741127278681076522421027457266980840_i128,(-71645452939236418890952869794026614477_i128),(-103909863291861439091538483755695094555_i128),(-127512356931401377597711709050126607134_i128),(-45084950677426840779610309024655109235_i128)];
_3 = 231_u8 | 54_u8;
_4 = _2;
_8 = _7;
_1 = _5 as u64;
RET = [(-148874097244201917207982775797973147537_i128),(-85660314022098976380353420043480184309_i128),150475562275717319182140448575544640035_i128,103747834788280271051947976585369891589_i128,(-64822759782129267156795398211530875192_i128),(-84310053189545365627057816098395053097_i128)];
_10 = _1 | _1;
Goto(bb4)
}
bb4 = {
_13 = 45159_u16;
_12.fld0.1 = -(-4135505852139670634_i64);
_12.fld0.1 = 2856061988166085867_i64;
_12.fld2 = [146718913230979452406354801552205128536_i128,104609082666393307037570462537346282260_i128,(-1707699486486506691003570708381608078_i128),164148452731757948314751898310062885632_i128,(-67667908004396580054851580175780000000_i128),164411661369669325986374576393771049640_i128];
RET = _12.fld2;
_5 = _7;
_11 = !(-9223372036854775808_isize);
_13 = !34615_u16;
_9 = 919376444_u32;
_1 = _6 * _12.fld0.0;
_12.fld1 = core::ptr::addr_of_mut!(_13);
_12.fld0.1 = (-54_i8) as i64;
_12.fld0.3 = _1 as f32;
_4 = _8;
_7 = _4;
_12.fld0.3 = _3 as f32;
_8 = _2;
match _9 {
0 => bb1,
1 => bb2,
2 => bb3,
919376444 => bb6,
_ => bb5
}
}
bb5 = {
Return()
}
bb6 = {
RET = [107291635809119678651458805122110079415_i128,(-110284669830307261843600943042872785867_i128),86065623629673276765121185183875537235_i128,(-12519683180174061290399092030575547491_i128),(-135720613045040869172955839540455374528_i128),(-117941361448631491275628041650887141654_i128)];
_14 = 13391661978140831013_usize;
_9 = !3332708583_u32;
_10 = _1;
_5 = _7;
_14 = _10 as usize;
_15 = (_12.fld0.1,);
_12.fld0.2 = _12.fld1;
_2 = _5;
_12.fld2 = [112876999711387552141790153175046314568_i128,122801911733497074253529507309082731172_i128,(-12172234423823356744751268940990727791_i128),17106778604349701249032977798446638793_i128,71361884358692361012523408776499224147_i128,153150100008857177117523502820394423906_i128];
_1 = _12.fld0.0 & _12.fld0.0;
_15.0 = _12.fld0.1;
_19.0 = -85777711011565016982212725479970292555_i128;
_19.2 = _4;
Call(_12 = fn4(_4, _2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_15 = (_12.fld0.1,);
_18 = _8 as isize;
_12.fld0.3 = _3 as f32;
_17 = [(-14737_i16),28244_i16,10848_i16,10802_i16,27127_i16,6604_i16,(-6548_i16)];
_12.fld0.1 = _15.0 ^ _15.0;
_6 = _12.fld0.0 * _1;
_10 = _12.fld0.1 as u64;
_15.0 = -_12.fld0.1;
_19 = ((-41029895905947241347653931346274462824_i128), 34329102110811330801519835535809462766_u128, _8);
_12.fld0.0 = _6 >> _14;
_14 = 7_usize;
_9 = !4034940582_u32;
_10 = _14 as u64;
_2 = _4;
_19 = (26833248073145404190339792375962495119_i128, 327562680880457021088419171012701694936_u128, _2);
_9 = !900728527_u32;
_7 = _5;
_4 = _8;
_12.fld1 = core::ptr::addr_of_mut!(_13);
_7 = _5;
_10 = !_12.fld0.0;
_14 = 6691140504901906801_usize | 980781225324104008_usize;
_2 = _7;
_12.fld2 = [_19.0,_19.0,_19.0,_19.0,_19.0,_19.0];
_12.fld2 = [_19.0,_19.0,_19.0,_19.0,_19.0,_19.0];
Goto(bb8)
}
bb8 = {
_12.fld0.1 = _15.0;
_19.2 = _4;
_18 = _11 ^ _11;
_13 = _15.0 as u16;
_1 = !_12.fld0.0;
RET = [_19.0,_19.0,_19.0,_19.0,_19.0,_19.0];
_12.fld0.3 = (-71_i8) as f32;
_20 = _19.1 / _19.1;
_12.fld0.1 = _15.0 + _15.0;
_8 = _4;
_25.fld3.fld3.1 = !15_i8;
_25.fld2.7 = 4633_i16 as u8;
_25.fld3.fld3.0.2 = core::ptr::addr_of_mut!(_25.fld3.fld3.1);
_25.fld2.4 = _7;
_28.fld2.5 = _13 as u32;
_25.fld4.fld2.3 = _19.0 & _19.0;
Goto(bb9)
}
bb9 = {
_25.fld2.5 = !_28.fld2.5;
_28.fld3.0.0 = _25.fld2.5 as u128;
_6 = _1 ^ _12.fld0.0;
_28.fld2.4 = _5;
_29 = _12.fld0.3 - _12.fld0.3;
_12.fld0.0 = _6 & _6;
_27.3 = !_1;
_25.fld2.3 = (-4418_i16) as i128;
_12.fld0.0 = _25.fld3.fld3.1 as u64;
RET = [_25.fld4.fld2.3,_25.fld2.3,_25.fld4.fld2.3,_25.fld4.fld2.3,_25.fld4.fld2.3,_25.fld4.fld2.3];
_27.2 = true;
_25.fld3.fld2.4 = _2;
Goto(bb10)
}
bb10 = {
Call(_33 = dump_var(3_usize, 15_usize, Move(_15), 20_usize, Move(_20), 18_usize, Move(_18), 14_usize, Move(_14)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_33 = dump_var(3_usize, 5_usize, Move(_5), 1_usize, Move(_1), 17_usize, Move(_17), 13_usize, Move(_13)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_33 = dump_var(3_usize, 11_usize, Move(_11), 34_usize, _34, 34_usize, _34, 34_usize, _34), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: char,mut _2: char) -> Adt55 {
mir! {
type RET = Adt55;
let _3: *mut (i8, f32, i32, i32);
let _4: (u128,);
let _5: [i32; 4];
let _6: (u64, char, i128, i128, char, u32, u8, u8);
let _7: f64;
let _8: isize;
let _9: f32;
let _10: [i32; 4];
let _11: bool;
let _12: usize;
let _13: f64;
let _14: f32;
let _15: (bool, i64);
let _16: u16;
let _17: ();
let _18: ();
{
RET.fld2 = [(-117117693160626181829073724627097486573_i128),(-10667007676590373107424459085869167071_i128),39970119914538012088810074695082685314_i128,8407348474897476000559386865236111170_i128,77317071499939460059918607888210992455_i128,(-158296114996166076854543650692866235706_i128)];
RET.fld0.3 = 2249302020_u32 as f32;
RET.fld0.0 = 15655949887061419289_u64 << 874708978_u32;
_1 = _2;
RET.fld0.0 = !13312419488214047805_u64;
RET.fld0.0 = 2787491619979006887_u64;
RET.fld0.1 = 5624199703468739577_usize as i64;
RET.fld0.1 = (-114161559491315617908014455685790120067_i128) as i64;
RET.fld0.3 = RET.fld0.1 as f32;
RET.fld2 = [128559546569364972950798067493088228018_i128,4737298926736567588755866961649773448_i128,(-161292057566720348795600758904016961954_i128),141878596755185610963372553375335566042_i128,(-75698505467876816590639333345548071150_i128),147878696973009459073559341650909074340_i128];
RET.fld0.3 = (-12_i8) as f32;
Goto(bb1)
}
bb1 = {
_2 = _1;
RET.fld2 = [61434582885476797001816932866738029848_i128,28762650594832962344160789178776453896_i128,121996377930167923950151116171317454154_i128,21273596185264406057364886773763718184_i128,(-121196358484238460695887290042597386879_i128),13876582099880172943589507124164528776_i128];
_4 = (320628392146518293069054289413270146567_u128,);
RET.fld2 = [82973241136994173731282194858385919396_i128,13464738572343490779208162900415908570_i128,(-109241898922648893288517226250493743198_i128),105849278116574456968527586887042750390_i128,(-81631321986457590309384552270033200809_i128),(-19539791966821728432362437802002074196_i128)];
_6.7 = 57_u8 | 197_u8;
Goto(bb2)
}
bb2 = {
_7 = 25572_u16 as f64;
Call(_5 = fn5(RET.fld0.0, RET.fld2, _1, _2, _1, _2, _1, _4.0, _1, _2, _1, _6.7, _4.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6.5 = 12292084653757326534_usize as u32;
RET.fld0.0 = !4891629512129407892_u64;
_1 = _2;
_6 = (RET.fld0.0, _1, (-75450047710595129972245897098531366021_i128), 42173064387710160192938868076424118709_i128, _1, 616570053_u32, 213_u8, 109_u8);
RET.fld2 = [_6.3,_6.2,_6.2,_6.2,_6.2,_6.3];
_6.1 = _2;
match _6.7 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
109 => bb11,
_ => bb10
}
}
bb4 = {
_7 = 25572_u16 as f64;
Call(_5 = fn5(RET.fld0.0, RET.fld2, _1, _2, _1, _2, _1, _4.0, _1, _2, _1, _6.7, _4.0), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_2 = _1;
RET.fld2 = [61434582885476797001816932866738029848_i128,28762650594832962344160789178776453896_i128,121996377930167923950151116171317454154_i128,21273596185264406057364886773763718184_i128,(-121196358484238460695887290042597386879_i128),13876582099880172943589507124164528776_i128];
_4 = (320628392146518293069054289413270146567_u128,);
RET.fld2 = [82973241136994173731282194858385919396_i128,13464738572343490779208162900415908570_i128,(-109241898922648893288517226250493743198_i128),105849278116574456968527586887042750390_i128,(-81631321986457590309384552270033200809_i128),(-19539791966821728432362437802002074196_i128)];
_6.7 = 57_u8 | 197_u8;
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
Return()
}
bb11 = {
RET.fld0.3 = 65380_u16 as f32;
_7 = RET.fld0.1 as f64;
RET.fld0.3 = RET.fld0.0 as f32;
RET.fld0.0 = !_6.0;
_9 = -RET.fld0.3;
_10 = [(-1882423072_i32),1167853879_i32,1721179957_i32,(-1563975393_i32)];
RET.fld0.1 = _7 as i64;
RET.fld2 = [_6.3,_6.2,_6.3,_6.3,_6.2,_6.2];
RET.fld0.3 = -_9;
_11 = true ^ false;
RET.fld0.3 = _9 - _9;
RET.fld2 = [_6.2,_6.2,_6.3,_6.3,_6.3,_6.3];
_4 = (253524183204681252032874854303261599657_u128,);
RET.fld2 = [_6.2,_6.2,_6.2,_6.2,_6.3,_6.3];
_12 = 5398141879318268270_usize;
_8 = !(-9223372036854775808_isize);
RET.fld0.1 = (-8537818512391633736_i64);
_6.5 = !378340718_u32;
RET.fld0.0 = 1834088056_i32 as u64;
match _6.6 {
0 => bb3,
1 => bb8,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
213 => bb17,
_ => bb16
}
}
bb12 = {
Return()
}
bb13 = {
_2 = _1;
RET.fld2 = [61434582885476797001816932866738029848_i128,28762650594832962344160789178776453896_i128,121996377930167923950151116171317454154_i128,21273596185264406057364886773763718184_i128,(-121196358484238460695887290042597386879_i128),13876582099880172943589507124164528776_i128];
_4 = (320628392146518293069054289413270146567_u128,);
RET.fld2 = [82973241136994173731282194858385919396_i128,13464738572343490779208162900415908570_i128,(-109241898922648893288517226250493743198_i128),105849278116574456968527586887042750390_i128,(-81631321986457590309384552270033200809_i128),(-19539791966821728432362437802002074196_i128)];
_6.7 = 57_u8 | 197_u8;
Goto(bb2)
}
bb14 = {
Return()
}
bb15 = {
_7 = 25572_u16 as f64;
Call(_5 = fn5(RET.fld0.0, RET.fld2, _1, _2, _1, _2, _1, _4.0, _1, _2, _1, _6.7, _4.0), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
Return()
}
bb17 = {
RET.fld0.0 = !_6.0;
_7 = _12 as f64;
RET.fld2 = [_6.2,_6.3,_6.2,_6.2,_6.3,_6.3];
_6.0 = _6.5 as u64;
RET.fld0.0 = _6.0 >> _6.7;
_6.5 = !4144370307_u32;
_10 = [(-546044799_i32),708339907_i32,994167968_i32,51707808_i32];
_6.3 = _6.2 * _6.2;
_6.5 = !497877053_u32;
_7 = _8 as f64;
_7 = _4.0 as f64;
_6.1 = _1;
_4 = (129207430202946674046598538821082215150_u128,);
RET.fld0.2 = core::ptr::addr_of_mut!(_16);
RET.fld0.2 = core::ptr::addr_of_mut!(_16);
RET.fld0.2 = core::ptr::addr_of_mut!(_16);
RET.fld0.3 = _7 as f32;
RET.fld1 = core::ptr::addr_of_mut!(_16);
RET.fld2 = [_6.3,_6.3,_6.2,_6.3,_6.3,_6.2];
Goto(bb18)
}
bb18 = {
Call(_17 = dump_var(4_usize, 5_usize, Move(_5), 10_usize, Move(_10), 6_usize, Move(_6), 4_usize, Move(_4)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: u64,mut _2: [i128; 6],mut _3: char,mut _4: char,mut _5: char,mut _6: char,mut _7: char,mut _8: u128,mut _9: char,mut _10: char,mut _11: char,mut _12: u8,mut _13: u128) -> [i32; 4] {
mir! {
type RET = [i32; 4];
let _14: [i128; 6];
let _15: f32;
let _16: *mut u16;
let _17: (i8, f32, i32, i32);
let _18: Adt52;
let _19: i16;
let _20: i128;
let _21: [i32; 1];
let _22: (u128, f32, *mut i8);
let _23: isize;
let _24: [i16; 7];
let _25: i64;
let _26: [i16; 7];
let _27: (u128,);
let _28: u16;
let _29: [u8; 3];
let _30: [u8; 7];
let _31: Adt46;
let _32: [u8; 3];
let _33: bool;
let _34: isize;
let _35: [i32; 4];
let _36: Adt51;
let _37: f32;
let _38: ();
let _39: ();
{
_1 = !8578113426088790404_u64;
_1 = 1889777556623387034_u64;
_13 = _8 % _8;
RET = [1930295954_i32,(-1783622378_i32),774271418_i32,499471269_i32];
_3 = _10;
_12 = !114_u8;
_13 = _8;
_10 = _11;
_1 = 14718224421297060431_u64;
_12 = !141_u8;
_7 = _4;
_7 = _10;
_3 = _6;
_5 = _7;
_6 = _10;
_15 = 2911595717767132560_i64 as f32;
_11 = _6;
_9 = _5;
Goto(bb1)
}
bb1 = {
_6 = _3;
_10 = _11;
_6 = _7;
_10 = _7;
_5 = _9;
RET = [(-1497132769_i32),1884321116_i32,613982920_i32,(-790240550_i32)];
_8 = _13 & _13;
_11 = _5;
_8 = 38_i8 as u128;
_17.0 = (-25_i8);
match _1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
14718224421297060431 => bb9,
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
_5 = _10;
_17.2 = (-694661556_i32);
_13 = _8;
_17 = ((-20_i8), _15, 1128704466_i32, 509673468_i32);
_11 = _7;
_17.0 = !35_i8;
_7 = _10;
_7 = _5;
_7 = _11;
_6 = _3;
_6 = _10;
_17.0 = (-51_i8) >> _13;
_8 = _13;
_13 = !_8;
_17.1 = _12 as f32;
_17.0 = _8 as i8;
_17.3 = -_17.2;
_8 = _13;
_5 = _10;
_17.0 = 74_i8 & (-55_i8);
_14 = [(-89806475433702124510141421466862070129_i128),154969502054938860562492172195198364522_i128,27774414465873851623901510773362035641_i128,(-50863750519967946873364923909992667553_i128),(-50034723819845641543590885678590136497_i128),(-49777233508200040121318463421159656561_i128)];
_17 = ((-60_i8), _15, (-1749852020_i32), 1395060508_i32);
Call(_5 = fn6(_14, _3, _4, _10, _17.0, _6, _6, _9, _11, _7, _7, _14, _17.2, _17.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_6 = _4;
_19 = -(-8299_i16);
RET = [_17.2,_17.2,_17.2,_17.2];
_14 = [65351198950566723795848928398626793869_i128,(-85138692988366991426279786257772300906_i128),(-162473476467634064527687227393807202732_i128),94548878999998668219851943308701858231_i128,(-60179471224868272254079451813975608152_i128),(-41909533734680717507456238724248217387_i128)];
_22.2 = core::ptr::addr_of_mut!(_17.0);
_20 = (-27665804528883899714850629829589877693_i128);
_20 = !(-99998794990386132445570471581766524634_i128);
_12 = 166_u8;
_21 = [_17.3];
_17 = ((-72_i8), _15, (-1704973282_i32), 1961149715_i32);
_22.2 = core::ptr::addr_of_mut!(_17.0);
Call(_18 = fn7(_11, _3, _5, _17, _17.2, _4, _11, _6, _2, _6, _12, _14, _5, _11, _9), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_22.1 = _17.1;
_9 = _6;
_1 = Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).0 >> Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).3;
_22.1 = -_15;
_14 = [Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).3,Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).3,Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).2,Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).3,Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).2,Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).3];
_19 = _5 as i16;
_15 = _22.1;
_17 = (51_i8, _22.1, (-1352458250_i32), 193924138_i32);
_14 = [Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).3,Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).3,Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).2,Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).3,Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).3,Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).2];
_13 = _8;
_2 = [Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).3,Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).2,Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).3,Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).3,Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).2,Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).2];
place!(Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0)).6 = 7719934071730138051_i64 as u8;
_17 = (48_i8, _22.1, 1369810845_i32, 2139118343_i32);
place!(Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0)).2 = -Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).3;
_21 = [_17.2];
_20 = Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).2 | Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(_18, 1), 0).3;
SetDiscriminant(_18, 0);
_22.1 = _15;
RET = [_17.3,_17.3,_17.2,_17.2];
_13 = _17.3 as u128;
_25 = (-5756123104230584816_i64) ^ 486399973856057617_i64;
_24 = [_19,_19,_19,_19,_19,_19,_19];
RET = [_17.2,_17.3,_17.2,_17.2];
place!(Field::<(u16, u128, bool, u64)>(Variant(_18, 0), 2)).3 = _1 * _1;
_16 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_18, 0), 0)));
_23 = -2_isize;
_16 = core::ptr::addr_of_mut!((*_16));
(*_16) = !8498_u16;
place!(Field::<(u16, u128, bool, u64)>(Variant(_18, 0), 2)).1 = !_13;
match _17.0 {
0 => bb1,
1 => bb10,
48 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_8 = _19 as u128;
place!(Field::<(*mut u16,)>(Variant(_18, 0), 1)).0 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_18, 0), 0)));
place!(Field::<(u16, u128, bool, u64)>(Variant(_18, 0), 2)).0 = (*_16) >> Field::<(u16, u128, bool, u64)>(Variant(_18, 0), 2).3;
_15 = -_22.1;
match _17.0 {
0 => bb11,
1 => bb10,
48 => bb14,
_ => bb9
}
}
bb14 = {
_12 = !145_u8;
_2 = _14;
_1 = !Field::<(u16, u128, bool, u64)>(Variant(_18, 0), 2).3;
place!(Field::<(u16, u128, bool, u64)>(Variant(_18, 0), 2)).2 = _20 >= _20;
place!(Field::<(u16, u128, bool, u64)>(Variant(_18, 0), 2)).0 = !(*_16);
_3 = _6;
_24 = [_19,_19,_19,_19,_19,_19,_19];
_22.1 = _17.1 - _15;
_9 = _6;
_8 = Field::<(u16, u128, bool, u64)>(Variant(_18, 0), 2).1;
_7 = _5;
_22.0 = _23 as u128;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(5_usize, 4_usize, Move(_4), 14_usize, Move(_14), 7_usize, Move(_7), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(5_usize, 5_usize, Move(_5), 12_usize, Move(_12), 3_usize, Move(_3), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(5_usize, 24_usize, Move(_24), 19_usize, Move(_19), 39_usize, _39, 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [i128; 6],mut _2: char,mut _3: char,mut _4: char,mut _5: i8,mut _6: char,mut _7: char,mut _8: char,mut _9: char,mut _10: char,mut _11: char,mut _12: [i128; 6],mut _13: i32,mut _14: i8) -> char {
mir! {
type RET = char;
let _15: i128;
let _16: f32;
let _17: ((u128, f32, *mut i8), i8, u64, *mut i8);
let _18: (*mut u16,);
let _19: (bool, i64);
let _20: f32;
let _21: i8;
let _22: Adt53;
let _23: f64;
let _24: f64;
let _25: f64;
let _26: isize;
let _27: (i64,);
let _28: [u8; 3];
let _29: isize;
let _30: (i64,);
let _31: Adt44;
let _32: *mut i8;
let _33: char;
let _34: (u128, f32, *mut i8);
let _35: ();
let _36: ();
{
_4 = _3;
_13 = 1792314884510954279_usize as i32;
RET = _8;
_5 = -_14;
_15 = _14 as i128;
_3 = _4;
_15 = 169662933341511226449254075060368355944_i128 << _14;
_2 = _7;
_11 = _2;
_13 = !(-1998933379_i32);
_4 = _7;
RET = _4;
_13 = (-1340038514_i32);
_6 = _10;
_10 = _6;
_2 = _3;
Goto(bb1)
}
bb1 = {
_4 = _11;
_3 = _9;
RET = _6;
_3 = _2;
_17.0.2 = core::ptr::addr_of_mut!(_14);
_6 = _3;
_12 = [_15,_15,_15,_15,_15,_15];
_6 = _9;
_7 = _10;
_17.3 = core::ptr::addr_of_mut!(_5);
_17.2 = 10613923644700210218_u64 - 16143644298767032467_u64;
_17.1 = _5;
_9 = _6;
_17.3 = _17.0.2;
_2 = _7;
_17.0.0 = 301358367684569323850814960488412855461_u128 - 307720295970158614185349859202396493522_u128;
_12 = _1;
_5 = !_14;
_12 = _1;
_11 = _8;
_16 = 182_u8 as f32;
_13 = true as i32;
_17.1 = _14 >> _15;
_10 = _6;
_17.1 = 252_u8 as i8;
_12 = [_15,_15,_15,_15,_15,_15];
match _14 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463463374607431768211396 => bb10,
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
_11 = _8;
_10 = _7;
_12 = [_15,_15,_15,_15,_15,_15];
_3 = _11;
_7 = _2;
_15 = (-151968649176682175364707742255413546557_i128);
_12 = _1;
_11 = _9;
_10 = _11;
_17.3 = _17.0.2;
_11 = _4;
_19 = (true, (-750666914813729277_i64));
_16 = _17.0.0 as f32;
_17.3 = core::ptr::addr_of_mut!(_14);
_15 = (-119722972288560396390824576097800491487_i128) * (-134642389047457510045252517106537704325_i128);
RET = _3;
RET = _4;
_6 = RET;
_10 = _2;
_3 = _11;
_11 = _4;
_9 = _10;
_17.2 = 11479197380679667304_u64 | 5057169123889725935_u64;
_13 = (-1588150109_i32) - 1667415753_i32;
_12 = [_15,_15,_15,_15,_15,_15];
_17.0 = (190040505611297135346562673181557436943_u128, _16, _17.3);
_2 = _11;
_17.0 = (175491054503883062935272008957588522313_u128, _16, _17.3);
_15 = 26690_u16 as i128;
_5 = _19.1 as i8;
Goto(bb11)
}
bb11 = {
_19.1 = 1528317865068083235_i64 << _17.0.0;
_7 = _10;
_19.1 = 9223372036854775807_isize as i64;
_19.0 = _16 > _16;
_17.0.0 = 137764913028492818823048352303989757272_u128;
_12 = [_15,_15,_15,_15,_15,_15];
_11 = _6;
_21 = _14 & _17.1;
_21 = _5 * _14;
_20 = _19.1 as f32;
RET = _4;
_20 = -_17.0.1;
_15 = 121311051905251581225848098248780140553_i128;
_24 = 159_u8 as f64;
_24 = _13 as f64;
_14 = _17.0.1 as i8;
RET = _10;
_25 = _24;
_12 = [_15,_15,_15,_15,_15,_15];
_5 = _21 ^ _21;
_17.0.2 = core::ptr::addr_of_mut!(_21);
_23 = _15 as f64;
_9 = _4;
_17.2 = !16174153567376739124_u64;
Goto(bb12)
}
bb12 = {
_17.2 = _4 as u64;
_23 = _13 as f64;
_17.0.0 = 262246925328146175223739937850998863215_u128;
_17.0.2 = _17.3;
_28 = [65_u8,96_u8,189_u8];
Goto(bb13)
}
bb13 = {
_14 = _5 + _21;
_27 = (_19.1,);
_1 = [_15,_15,_15,_15,_15,_15];
_19.0 = !false;
_17.0 = (215130885651374354239649951530129645395_u128, _20, _17.3);
_1 = _12;
_30.0 = _19.1 + _19.1;
_19.1 = !_27.0;
_6 = _7;
_27.0 = _19.1;
Goto(bb14)
}
bb14 = {
_1 = [_15,_15,_15,_15,_15,_15];
_25 = _23;
_34.0 = !_17.0.0;
_5 = -_21;
_27.0 = _17.2 as i64;
RET = _10;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(6_usize, 13_usize, Move(_13), 3_usize, Move(_3), 2_usize, Move(_2), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(6_usize, 19_usize, Move(_19), 30_usize, Move(_30), 15_usize, Move(_15), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(6_usize, 21_usize, Move(_21), 28_usize, Move(_28), 36_usize, _36, 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: char,mut _2: char,mut _3: char,mut _4: (i8, f32, i32, i32),mut _5: i32,mut _6: char,mut _7: char,mut _8: char,mut _9: [i128; 6],mut _10: char,mut _11: u8,mut _12: [i128; 6],mut _13: char,mut _14: char,mut _15: char) -> Adt52 {
mir! {
type RET = Adt52;
let _16: (i64, u16, f64, (u128, f32, *mut i8), i8);
let _17: f64;
let _18: [u8; 7];
let _19: [u8; 3];
let _20: (&'static char, u64, i64);
let _21: (i128, u128, char);
let _22: (i64,);
let _23: u128;
let _24: u8;
let _25: (u16, u128, bool, u64);
let _26: *const i128;
let _27: u32;
let _28: u128;
let _29: [i32; 4];
let _30: f32;
let _31: f32;
let _32: isize;
let _33: i32;
let _34: Adt59;
let _35: f64;
let _36: [i32; 1];
let _37: [i32; 4];
let _38: isize;
let _39: (*mut u16,);
let _40: isize;
let _41: *mut u16;
let _42: Adt45;
let _43: [i32; 1];
let _44: ();
let _45: ();
{
_8 = _14;
_16.1 = 2025045689688704268248560816368463834_i128 as u16;
_13 = _1;
_16.3.2 = core::ptr::addr_of_mut!(_16.4);
_4.1 = 9223372036854775807_isize as f32;
_4.2 = _5 - _4.3;
_16.3.2 = core::ptr::addr_of_mut!(_16.4);
_14 = _1;
_7 = _14;
_16.0 = 4629959977586871108_i64 & 2844185613980577281_i64;
_2 = _6;
_16.0 = 2493729866_u32 as i64;
_8 = _3;
_18 = [_11,_11,_11,_11,_11,_11,_11];
_4.2 = !_5;
_18 = [_11,_11,_11,_11,_11,_11,_11];
_20.1 = _4.1 as u64;
_4.3 = _5;
_7 = _10;
Goto(bb1)
}
bb1 = {
_18 = [_11,_11,_11,_11,_11,_11,_11];
_11 = !34_u8;
_18 = [_11,_11,_11,_11,_11,_11,_11];
_13 = _15;
_20.2 = !_16.0;
_6 = _1;
_4.2 = _5;
_11 = _4.1 as u8;
_10 = _8;
_20.0 = &_7;
_21.0 = !134970602499581556416512514645779353647_i128;
_19 = [_11,_11,_11];
_16.3.1 = _4.1 * _4.1;
_16.3.0 = 52806130464973236586267411044375486003_u128 * 274742823917640418312475769491656265325_u128;
_22.0 = _20.2;
_21.1 = _16.3.0 - _16.3.0;
_2 = _10;
_16.2 = _22.0 as f64;
_24 = false as u8;
_4.0 = _24 as i8;
_16.1 = 577_i16 as u16;
_9 = _12;
match _5 {
0 => bb2,
1 => bb3,
340282366920938463463374607430063238174 => bb5,
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
_1 = _2;
_21.0 = !85367267178573323929346257665442157584_i128;
_16.4 = (-16309_i16) as i8;
_20.2 = 43_isize as i64;
_21.2 = _3;
_28 = _21.1;
_20.1 = !14106759092403293902_u64;
_22 = (_20.2,);
_22.0 = _16.0 ^ _16.0;
_13 = _3;
_4.3 = _4.0 as i32;
_23 = _21.1 << _16.0;
_9 = [_21.0,_21.0,_21.0,_21.0,_21.0,_21.0];
match _4.2 {
0 => bb2,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
340282366920938463463374607430063238174 => bb12,
_ => bb11
}
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
_18 = [_11,_11,_11,_11,_11,_11,_11];
_11 = !34_u8;
_18 = [_11,_11,_11,_11,_11,_11,_11];
_13 = _15;
_20.2 = !_16.0;
_6 = _1;
_4.2 = _5;
_11 = _4.1 as u8;
_10 = _8;
_20.0 = &_7;
_21.0 = !134970602499581556416512514645779353647_i128;
_19 = [_11,_11,_11];
_16.3.1 = _4.1 * _4.1;
_16.3.0 = 52806130464973236586267411044375486003_u128 * 274742823917640418312475769491656265325_u128;
_22.0 = _20.2;
_21.1 = _16.3.0 - _16.3.0;
_2 = _10;
_16.2 = _22.0 as f64;
_24 = false as u8;
_4.0 = _24 as i8;
_16.1 = 577_i16 as u16;
_9 = _12;
match _5 {
0 => bb2,
1 => bb3,
340282366920938463463374607430063238174 => bb5,
_ => bb4
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_26 = core::ptr::addr_of!(_21.0);
_18 = [_24,_24,_24,_11,_11,_24,_11];
_26 = core::ptr::addr_of!(_21.0);
_4.0 = true as i8;
_25.0 = !_16.1;
_29 = [_4.2,_4.2,_4.2,_5];
_14 = _13;
_20.0 = &_8;
_1 = _13;
_34 = Adt59::Variant1 { fld0: 1610399234_u32 };
_23 = !_28;
_19 = [_24,_11,_24];
_25.3 = _20.1 | _20.1;
_16.0 = _4.2 as i64;
_21.2 = _3;
_14 = _7;
_35 = -_16.2;
_9 = [(*_26),_21.0,(*_26),(*_26),(*_26),_21.0];
_29 = [_4.3,_5,_4.2,_4.2];
_25.0 = _16.1;
_16.4 = _4.0 * _4.0;
_9 = [(*_26),(*_26),_21.0,_21.0,(*_26),_21.0];
match _4.2 {
0 => bb10,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb7,
340282366920938463463374607430063238174 => bb14,
_ => bb13
}
}
bb13 = {
_18 = [_11,_11,_11,_11,_11,_11,_11];
_11 = !34_u8;
_18 = [_11,_11,_11,_11,_11,_11,_11];
_13 = _15;
_20.2 = !_16.0;
_6 = _1;
_4.2 = _5;
_11 = _4.1 as u8;
_10 = _8;
_20.0 = &_7;
_21.0 = !134970602499581556416512514645779353647_i128;
_19 = [_11,_11,_11];
_16.3.1 = _4.1 * _4.1;
_16.3.0 = 52806130464973236586267411044375486003_u128 * 274742823917640418312475769491656265325_u128;
_22.0 = _20.2;
_21.1 = _16.3.0 - _16.3.0;
_2 = _10;
_16.2 = _22.0 as f64;
_24 = false as u8;
_4.0 = _24 as i8;
_16.1 = 577_i16 as u16;
_9 = _12;
match _5 {
0 => bb2,
1 => bb3,
340282366920938463463374607430063238174 => bb5,
_ => bb4
}
}
bb14 = {
place!(Field::<u32>(Variant(_34, 1), 0)) = 1546419342_u32 | 94460138_u32;
place!(Field::<u32>(Variant(_34, 1), 0)) = !3376043170_u32;
_25.2 = !false;
_33 = _4.2 * _5;
_1 = _21.2;
_30 = -_16.3.1;
_24 = _11;
_8 = _7;
_13 = _14;
Call(RET = fn8(_13, _4, _2, _3, _15, _15, _2, _23, _14, _4.2), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_25.2 = true ^ false;
_39.0 = core::ptr::addr_of_mut!(_16.1);
_1 = _21.2;
place!(Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(RET, 1), 0)).3 = Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(RET, 1), 0).2 + Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(RET, 1), 0).2;
_16.1 = _25.2 as u16;
place!(Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(RET, 1), 0)).6 = (-64_isize) as u8;
place!(Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(RET, 1), 0)).7 = Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(RET, 1), 0).2 as u8;
_21.0 = !Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(RET, 1), 0).3;
place!(Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(RET, 1), 0)).0 = !_25.3;
_36 = [_4.2];
place!(Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(RET, 1), 0)).0 = _25.3;
place!(Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(RET, 1), 0)).5 = 9223372036854775807_isize as u32;
(*_26) = -Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(RET, 1), 0).2;
_38 = !9223372036854775807_isize;
place!(Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(RET, 1), 0)).6 = !Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(RET, 1), 0).7;
Goto(bb16)
}
bb16 = {
Call(_44 = dump_var(7_usize, 7_usize, Move(_7), 14_usize, Move(_14), 8_usize, Move(_8), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(7_usize, 19_usize, Move(_19), 2_usize, Move(_2), 12_usize, Move(_12), 21_usize, Move(_21)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(7_usize, 33_usize, Move(_33), 13_usize, Move(_13), 1_usize, Move(_1), 10_usize, Move(_10)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: char,mut _2: (i8, f32, i32, i32),mut _3: char,mut _4: char,mut _5: char,mut _6: char,mut _7: char,mut _8: u128,mut _9: char,mut _10: i32) -> Adt52 {
mir! {
type RET = Adt52;
let _11: [i8; 3];
let _12: isize;
let _13: bool;
let _14: Adt47;
let _15: isize;
let _16: i32;
let _17: [i16; 7];
let _18: *mut u16;
let _19: bool;
let _20: Adt50;
let _21: i16;
let _22: *const (u64, i64, *mut u16, f32);
let _23: ();
let _24: ();
{
_8 = 338054945594059051016530910906311182309_u128;
_12 = 2_isize - 9223372036854775807_isize;
_11 = [_2.0,_2.0,_2.0];
_9 = _1;
_2.2 = true as i32;
_2.0 = _8 as i8;
_1 = _3;
_2.3 = _10 ^ _10;
_4 = _9;
_12 = -9223372036854775807_isize;
_13 = true;
_10 = _2.3 * _2.3;
_2.3 = _10 | _10;
_13 = _2.3 <= _10;
_8 = 11966_u16 as u128;
_10 = _2.3 * _2.3;
_9 = _4;
_8 = !149123769489697616948927868372614269573_u128;
_11 = [_2.0,_2.0,_2.0];
_9 = _3;
_14.fld2.5 = 3051888004_u32;
_14.fld2.7 = 156_u8 - 186_u8;
_14.fld2.2 = 2782445556847223773_i64 as i128;
_4 = _1;
_14.fld2.1 = _6;
_14.fld3.1 = _14.fld2.7 as i8;
_3 = _6;
_2.3 = !_10;
Goto(bb1)
}
bb1 = {
_14.fld3.0.0 = 31131_i16 as u128;
_14.fld2.6 = _14.fld2.7;
_14.fld3.0.1 = _8 as f32;
_14.fld3.0.0 = _8 ^ _8;
_14.fld3.3 = core::ptr::addr_of_mut!(_14.fld3.1);
_3 = _4;
_14.fld2.0 = 16313230464021906454_u64 - 11251313719733435912_u64;
_3 = _1;
_14.fld1 = _14.fld2.6 + _14.fld2.6;
_14.fld3.0.1 = _2.1;
_1 = _9;
_17 = [(-16723_i16),10723_i16,6814_i16,9198_i16,(-4677_i16),(-28048_i16),13193_i16];
_14.fld3.0 = (_8, _2.1, _14.fld3.3);
_14.fld1 = _14.fld2.6 * _14.fld2.6;
_14.fld2 = (15514872431270583194_u64, _4, 155879058922373182223110875653329396468_i128, 3193611794997285951631741516271565082_i128, _5, 2537841144_u32, _14.fld1, _14.fld1);
_14.fld0 = 4_usize;
_5 = _14.fld2.1;
Goto(bb2)
}
bb2 = {
_1 = _6;
_2.0 = _13 as i8;
_14.fld3.3 = _14.fld3.0.2;
_15 = -_12;
_9 = _14.fld2.4;
_1 = _3;
Goto(bb3)
}
bb3 = {
_7 = _9;
_15 = _12;
_14.fld1 = _14.fld2.6 >> _14.fld2.2;
_17 = [22407_i16,(-21181_i16),(-15545_i16),(-26492_i16),(-16238_i16),32032_i16,12079_i16];
_14.fld3.0.0 = (-14457_i16) as u128;
_14.fld2 = (4926682557590617420_u64, _4, 55612624923199229207090134561291976672_i128, (-124549287640224722768349584850408526832_i128), _6, 2363629517_u32, _14.fld1, _14.fld1);
_2.0 = _14.fld3.1 | _14.fld3.1;
_2 = (_14.fld3.1, _14.fld3.0.1, _10, _10);
_11 = [_2.0,_14.fld3.1,_2.0];
_14.fld2.3 = _14.fld2.2 - _14.fld2.2;
_16 = _2.2 >> _10;
_14.fld2.4 = _3;
_15 = _8 as isize;
_5 = _14.fld2.1;
_21 = -8991_i16;
_2.3 = -_16;
_14.fld2.1 = _14.fld2.4;
RET = Adt52::Variant1 { fld0: _14.fld2 };
_14.fld3.3 = _14.fld3.0.2;
_14.fld0 = _14.fld3.1 as usize;
place!(Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(RET, 1), 0)).6 = _14.fld2.7 & _14.fld2.7;
_17 = [_21,_21,_21,_21,_21,_21,_21];
_14.fld3.2 = Field::<(u64, char, i128, i128, char, u32, u8, u8)>(Variant(RET, 1), 0).0;
Goto(bb4)
}
bb4 = {
Call(_23 = dump_var(8_usize, 5_usize, Move(_5), 16_usize, Move(_16), 21_usize, Move(_21), 3_usize, Move(_3)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_23 = dump_var(8_usize, 9_usize, Move(_9), 12_usize, Move(_12), 8_usize, Move(_8), 6_usize, Move(_6)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [i128; 6],mut _2: u64,mut _3: i64) -> u64 {
mir! {
type RET = u64;
let _4: (i64,);
let _5: u16;
let _6: Adt55;
let _7: char;
let _8: isize;
let _9: (i8, f32, i32, i32);
let _10: u64;
let _11: char;
let _12: [u8; 3];
let _13: u64;
let _14: &'static char;
let _15: [u8; 7];
let _16: isize;
let _17: f32;
let _18: [i32; 4];
let _19: (i64, u16, f64, (u128, f32, *mut i8), i8);
let _20: [i16; 7];
let _21: isize;
let _22: i8;
let _23: Adt58;
let _24: (bool, i64);
let _25: (bool, i64);
let _26: [u8; 3];
let _27: isize;
let _28: Adt46;
let _29: isize;
let _30: Adt46;
let _31: [i16; 7];
let _32: isize;
let _33: f64;
let _34: isize;
let _35: (*mut u16,);
let _36: f64;
let _37: [i128; 6];
let _38: char;
let _39: ();
let _40: ();
{
_1 = [(-138755163824749528191392908471130444349_i128),136971158429268473980626418345583785893_i128,78467499406922005905398536220838201787_i128,35824303720286402610152052747151124323_i128,(-115467355327135909343265790531248622553_i128),(-18391747103121475049091618660400737780_i128)];
_2 = 1788132233_u32 as u64;
_4.0 = _3;
RET = 51656_u16 as u64;
RET = !_2;
RET = _2 + _2;
_4.0 = 32536_u16 as i64;
_1 = [(-163670828372383006228733965324520305291_i128),(-103079357312475858113196417586385982753_i128),(-1417381784302060441727064268669767569_i128),(-36054432619867166146417068879563036682_i128),(-59501710806228807846619597769242964956_i128),47506781898732906752830786489427030850_i128];
_1 = [60008063893522835498635810538928443175_i128,7125191200942369665207361324273432417_i128,(-131954398433476018101916976497470818402_i128),128262650211365676528611507382269192726_i128,61254637803254851101129119062671323978_i128,127431785733349972020326102130206174261_i128];
_6.fld0.1 = -_3;
_6.fld0.2 = core::ptr::addr_of_mut!(_5);
_3 = -_6.fld0.1;
_6.fld2 = [(-58038711644113611998124273500757793845_i128),(-48247203154046557303376261089806051314_i128),12122507393190530393819464658443719747_i128,151525742480797634310303284234084454895_i128,(-113431638274211547886085047289371234413_i128),83405926206979150153117061270661475507_i128];
Goto(bb1)
}
bb1 = {
_6.fld1 = core::ptr::addr_of_mut!(_5);
_6.fld0.3 = 69_u8 as f32;
_6.fld0.2 = core::ptr::addr_of_mut!(_5);
_6.fld0.0 = 1143244296_i32 as u64;
Call(_4.0 = core::intrinsics::bswap(_6.fld0.1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = 21774_u16 + 34772_u16;
_8 = 9223372036854775807_isize * (-9223372036854775808_isize);
_3 = 2060776293_u32 as i64;
_9 = ((-79_i8), _6.fld0.3, (-1956695304_i32), (-183081492_i32));
_6.fld0 = (RET, _4.0, _6.fld1, _9.1);
_6.fld0.3 = _9.1 * _9.1;
_7 = '\u{c97e}';
_9.0 = 209_u8 as i8;
_10 = !_6.fld0.0;
_4 = (_3,);
_6.fld0.1 = _3;
_6.fld1 = core::ptr::addr_of_mut!(_5);
_6.fld1 = core::ptr::addr_of_mut!(_5);
_9.0 = !(-91_i8);
_12 = [161_u8,253_u8,23_u8];
_1 = _6.fld2;
RET = !_6.fld0.0;
_11 = _7;
_4 = (_6.fld0.1,);
_11 = _7;
_1 = [(-95906746411365275430521576981090369906_i128),120114452292099754315082205302338883757_i128,73350541059529655703617331172653699839_i128,91409132266142228836041496400827651719_i128,83812230570979111376742316845429268877_i128,127974184059154419338331562897023789409_i128];
_12 = [84_u8,192_u8,242_u8];
_9.3 = true as i32;
_8 = -(-110_isize);
Goto(bb3)
}
bb3 = {
_9.2 = !_9.3;
_14 = &_11;
_3 = !_6.fld0.1;
_12 = [101_u8,4_u8,253_u8];
RET = _6.fld0.0 - _10;
_13 = !_10;
_6.fld0.3 = 3795780284_u32 as f32;
_5 = _8 as u16;
_9.2 = 184_u8 as i32;
_2 = RET;
_9.1 = _6.fld0.3;
_9.2 = _9.3 + _9.3;
_3 = _6.fld0.1 & _6.fld0.1;
RET = !_2;
_7 = (*_14);
RET = !_2;
_8 = !9223372036854775807_isize;
_7 = _11;
_12 = [22_u8,250_u8,206_u8];
_7 = (*_14);
_13 = _10;
_9 = ((-47_i8), _6.fld0.3, 399079660_i32, (-602961966_i32));
_12 = [230_u8,69_u8,72_u8];
_6.fld0.1 = -_4.0;
_1 = [(-4615976302066684903052713198518222163_i128),135931718025157406242698164516722740368_i128,5670925119471636209396378797703620941_i128,(-7703012104429169132624131229705954843_i128),(-8298168095546524392394794711430418101_i128),(-99810919621217497624337139993452261784_i128)];
_9.1 = _6.fld0.3;
_9.1 = _6.fld0.3;
_5 = 12833_u16;
_9 = (56_i8, _6.fld0.3, (-1175178999_i32), 1151591251_i32);
Goto(bb4)
}
bb4 = {
_6.fld0.1 = !_3;
_16 = _8;
_9.3 = _9.2;
_6.fld2 = [(-117883520240779866228451215668190319817_i128),138398771959451244395269463694141332300_i128,24301889008938029039659512133985084927_i128,(-137770435171510834545614795212460429175_i128),(-130040031797698679964067560781754594162_i128),4251540496360409854554780214024087455_i128];
_1 = [(-146902999164398039355282429564699113080_i128),(-18607341480463243288162378082346849188_i128),6176689461434464829881234676517010858_i128,(-56995264974036288137636922261520107460_i128),104783286543147885934921647314573203985_i128,(-45744377801442636711247460146386453256_i128)];
_7 = _11;
_6.fld0 = (_13, _3, _6.fld1, _9.1);
_8 = _16;
_17 = _9.1 + _9.1;
_14 = &(*_14);
_6.fld0.1 = _11 as i64;
Call(_17 = core::intrinsics::transmute(_9.3), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_12 = [181_u8,253_u8,149_u8];
_9 = ((-66_i8), _17, 868265906_i32, (-882131444_i32));
_11 = _7;
_13 = _2;
_6.fld0.1 = _4.0;
_17 = _9.1 - _9.1;
_14 = &_7;
_7 = _11;
_7 = _11;
_19.3.1 = _9.2 as f32;
_7 = _11;
_19.3.0 = _17 as u128;
_16 = _8;
_8 = -_16;
_9.3 = _9.2 ^ _9.2;
_19.0 = !_4.0;
_7 = _11;
_9.1 = _19.3.1;
_19.3.1 = _19.3.0 as f32;
_4.0 = !_6.fld0.1;
_4.0 = -_3;
_18 = [_9.3,_9.3,_9.3,_9.3];
_20 = [5096_i16,24821_i16,(-6671_i16),26916_i16,(-797_i16),2865_i16,3011_i16];
RET = _6.fld0.0 >> _3;
_3 = _4.0;
Goto(bb6)
}
bb6 = {
_14 = &_7;
_23.fld3.fld2.6 = _9.3 as u8;
_23.fld4.fld6 = _6.fld2;
_23.fld3.fld2.2 = 1234640835_u32 as i128;
_8 = !_16;
_19.2 = _19.3.0 as f64;
_17 = _19.3.1;
_23.fld3.fld2.5 = 585051412_u32 - 754295219_u32;
_6.fld0.2 = _6.fld1;
_23.fld2 = (_2, (*_14), _23.fld3.fld2.2, _23.fld3.fld2.2, _11, _23.fld3.fld2.5, _23.fld3.fld2.6, _23.fld3.fld2.6);
_13 = _19.3.0 as u64;
_23.fld4.fld0 = core::ptr::addr_of_mut!(_19.1);
_23.fld4.fld2.6 = _23.fld2.7;
_23.fld4.fld1 = _23.fld4.fld2.6 as u32;
_19.4 = (*_14) as i8;
_23.fld4.fld2.2 = !_23.fld2.2;
_23.fld3.fld3.0.2 = core::ptr::addr_of_mut!(_19.4);
_23.fld4.fld2.6 = false as u8;
_23.fld4.fld2.4 = (*_14);
_6.fld0 = (RET, _3, _6.fld1, _19.3.1);
_19.3.1 = _19.3.0 as f32;
_23.fld2.3 = -_23.fld2.2;
match _9.2 {
868265906 => bb8,
_ => bb7
}
}
bb7 = {
_6.fld1 = core::ptr::addr_of_mut!(_5);
_6.fld0.3 = 69_u8 as f32;
_6.fld0.2 = core::ptr::addr_of_mut!(_5);
_6.fld0.0 = 1143244296_i32 as u64;
Call(_4.0 = core::intrinsics::bswap(_6.fld0.1), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_25.1 = _4.0 * _19.0;
_6.fld0.1 = _19.3.0 as i64;
_23.fld3.fld2 = (RET, _7, _23.fld2.2, _23.fld2.3, _23.fld2.1, _23.fld4.fld1, _23.fld2.6, _23.fld2.6);
_22 = false as i8;
Goto(bb9)
}
bb9 = {
_9.1 = _6.fld0.3 - _17;
_21 = _16 ^ _16;
_23.fld4.fld4 = core::ptr::addr_of!(_6.fld0);
match _9.0 {
340282366920938463463374607431768211390 => bb10,
_ => bb5
}
}
bb10 = {
_25.1 = !_4.0;
_11 = _23.fld2.4;
_19.2 = _22 as f64;
_23.fld4.fld2.2 = _23.fld3.fld2.2 | _23.fld3.fld2.2;
_6.fld0.0 = !_13;
_23.fld3.fld3.2 = !_13;
_23.fld4.fld2.4 = _23.fld3.fld2.4;
_23.fld2.3 = _23.fld3.fld2.2 ^ _23.fld4.fld2.2;
_4.0 = _6.fld0.1;
_23.fld3.fld3.0.1 = -_17;
_23.fld4.fld2.7 = _23.fld3.fld2.6;
_13 = _23.fld2.0 & _23.fld3.fld3.2;
_24 = (false, _19.0);
_3 = _23.fld2.6 as i64;
_23.fld3.fld3.0.2 = core::ptr::addr_of_mut!(_23.fld3.fld3.1);
_4.0 = _16 as i64;
RET = _23.fld3.fld3.2;
_25.1 = _3 & _6.fld0.1;
_23.fld3.fld0 = !4_usize;
_3 = _6.fld0.1 + _6.fld0.1;
_23.fld4.fld2.0 = _23.fld2.0 & RET;
_21 = _19.2 as isize;
_23.fld2.7 = _23.fld4.fld2.7;
Goto(bb11)
}
bb11 = {
_23.fld4.fld6 = _6.fld2;
_23.fld0 = core::ptr::addr_of!(_6.fld0);
_23.fld4.fld0 = core::ptr::addr_of_mut!(_5);
_23.fld3.fld3.0.0 = _23.fld3.fld2.0 as u128;
_30.fld0 = -_19.2;
_23.fld4.fld2.3 = _23.fld2.2;
_19.3.0 = _23.fld3.fld2.2 as u128;
_23.fld3.fld3.2 = _6.fld0.0;
_20 = [18978_i16,28997_i16,(-4930_i16),(-14345_i16),31337_i16,3436_i16,12651_i16];
_25 = (_24.0, _6.fld0.1);
_28.fld0 = _23.fld3.fld3.0.0 as f64;
_8 = _16 | _21;
_24 = _25;
_19.3.0 = _23.fld3.fld3.0.0 - _23.fld3.fld3.0.0;
_19.3.0 = !_23.fld3.fld3.0.0;
_7 = _23.fld3.fld2.4;
_6.fld0.1 = _28.fld0 as i64;
_23.fld2.1 = _7;
_25.1 = _21 as i64;
_4 = (_3,);
_4 = (_6.fld0.1,);
Goto(bb12)
}
bb12 = {
_19.4 = _9.0;
_4 = (_3,);
_3 = _4.0;
_31 = _20;
_23.fld3.fld3.3 = core::ptr::addr_of_mut!(_19.4);
_6.fld0.3 = RET as f32;
_25.1 = _8 as i64;
_19.3.2 = core::ptr::addr_of_mut!(_9.0);
_23.fld3.fld3.2 = RET * RET;
RET = !_23.fld3.fld3.2;
_19.1 = _23.fld3.fld2.6 as u16;
_9.2 = _9.3 - _9.3;
_20 = _31;
_1 = _6.fld2;
_32 = _8;
_23.fld4.fld2 = (_13, _23.fld2.4, _23.fld3.fld2.2, _23.fld2.3, _23.fld2.4, _23.fld3.fld2.5, _23.fld3.fld2.7, _23.fld3.fld2.7);
_28.fld1 = _11;
_38 = _23.fld3.fld2.1;
_30 = Adt46 { fld0: _19.2,fld1: _23.fld4.fld2.4 };
_2 = !_23.fld4.fld2.0;
_23.fld4.fld2.4 = _23.fld3.fld2.1;
_14 = &_30.fld1;
Goto(bb13)
}
bb13 = {
Call(_39 = dump_var(9_usize, 32_usize, Move(_32), 22_usize, Move(_22), 2_usize, Move(_2), 11_usize, Move(_11)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_39 = dump_var(9_usize, 38_usize, Move(_38), 31_usize, Move(_31), 16_usize, Move(_16), 21_usize, Move(_21)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_39 = dump_var(9_usize, 20_usize, Move(_20), 3_usize, Move(_3), 40_usize, _40, 40_usize, _40), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(2874279757405906710_u64), std::hint::black_box('\u{2d454}'), std::hint::black_box(1208298296_u32), std::hint::black_box(110_i8), std::hint::black_box((-14510_i16)), std::hint::black_box((-2059601537_i32)), std::hint::black_box(205_u8), std::hint::black_box((-35142025856832627784952173057011444408_i128)), std::hint::black_box(262493765177291644711884678533443584884_u128));
                
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: [i8; 3],
fld1: (i8, f32, i32, i32),
fld2: *mut u16,
fld3: *mut (i8, f32, i32, i32),
fld4: (u16, u128, bool, u64),
fld5: ((u128, f32, *mut i8), i8, u64, *mut i8),
fld6: u16,

},
Variant1{
fld0: (bool, i64),
fld1: (u16, u128, bool, u64),
fld2: u32,

},
Variant2{
fld0: isize,
fld1: [i32; 1],

},
Variant3{
fld0: f32,
fld1: (u128,),
fld2: (i64,),
fld3: u32,
fld4: [u8; 3],

}}
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
fld0: (u16, u128, bool, u64),
fld1: (u128,),
fld2: isize,
fld3: u64,
fld4: (*mut u16,),
fld5: *mut (i8, f32, i32, i32),
fld6: [i128; 6],
fld7: i128,

},
Variant1{
fld0: ((u128, f32, *mut i8), i8, u64, *mut i8),
fld1: [i128; 6],
fld2: [u8; 3],
fld3: *mut i8,
fld4: *mut (i8, f32, i32, i32),
fld5: *mut u16,
fld6: i64,

},
Variant2{
fld0: bool,
fld1: char,
fld2: [i32; 4],
fld3: [i32; 1],
fld4: i16,
fld5: [u8; 7],
fld6: (i8, f32, i32, i32),
fld7: (u16, u128, bool, u64),

},
Variant3{
fld0: *mut u16,
fld1: *mut i8,
fld2: [i32; 4],

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: f64,
fld1: char,
}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt47{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt47 {
fld0: usize,
fld1: u8,
fld2: (u64, char, i128, i128, char, u32, u8, u8),
fld3: ((u128, f32, *mut i8), i8, u64, *mut i8),
}
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: f64,
fld1: (u128, f32, *mut i8),
fld2: (i64, u16, f64, (u128, f32, *mut i8), i8),
fld3: i8,
fld4: (i64,),

},
Variant1{
fld0: *mut u16,
fld1: (f64, *mut u16, f64, u32),
fld2: [i8; 3],

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: Adt48,
fld1: i128,

},
Variant1{
fld0: (bool, i64),
fld1: *mut u16,
fld2: i128,
fld3: i16,

},
Variant2{
fld0: (i64,),

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: [i32; 4],
fld1: usize,

},
Variant1{
fld0: bool,
fld1: char,
fld2: isize,
fld3: [u8; 3],
fld4: usize,
fld5: Adt45,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: *mut u16,
fld1: u32,
fld2: (u64, char, i128, i128, char, u32, u8, u8),
fld3: [i8; 3],
fld4: *const (u64, i64, *mut u16, f32),
fld5: Adt45,
fld6: [i128; 6],
}
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
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: u16,
fld1: (*mut u16,),
fld2: (u16, u128, bool, u64),

},
Variant1{
fld0: (u64, char, i128, i128, char, u32, u8, u8),

},
Variant2{
fld0: [i32; 4],
fld1: u128,
fld2: [u8; 7],
fld3: Adt50,
fld4: i16,

},
Variant3{
fld0: ((u128, f32, *mut i8), i8, u64, *mut i8),
fld1: usize,
fld2: [i32; 1],
fld3: Adt50,
fld4: (u128, f32, *mut i8),
fld5: (*mut u16,),
fld6: (i8, f32, i32, i32),
fld7: (u128,),

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
fld0: (i64, u16, f64, (u128, f32, *mut i8), i8),
fld1: [i8; 3],
fld2: isize,
fld3: *mut (i8, f32, i32, i32),
fld4: [u8; 7],
fld5: Adt45,
fld6: [i32; 4],
fld7: i128,

},
Variant1{
fld0: bool,
fld1: [i8; 3],
fld2: Adt44,
fld3: i64,
fld4: f64,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: bool,
fld1: [u8; 7],
fld2: Adt49,
fld3: (u128,),
fld4: i128,

},
Variant1{
fld0: Adt48,
fld1: u8,
fld2: (u16, u128, bool, u64),
fld3: [i16; 7],
fld4: *mut u16,
fld5: Adt45,
fld6: ((u128, f32, *mut i8), i8, u64, *mut i8),

},
Variant2{
fld0: [u8; 3],
fld1: u64,
fld2: i32,

},
Variant3{
fld0: Adt47,
fld1: i32,
fld2: isize,
fld3: Adt52,
fld4: [i32; 1],

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: (u64, i64, *mut u16, f32),
fld1: *mut u16,
fld2: [i128; 6],
}
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: Adt54,
fld1: (u128, f32, *mut i8),
fld2: Adt49,

},
Variant1{
fld0: [i32; 1],

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: (*mut u16,),
fld1: *mut (i8, f32, i32, i32),
fld2: (i64, u16, f64, (u128, f32, *mut i8), i8),

},
Variant1{
fld0: Adt53,
fld1: [u8; 3],
fld2: u16,
fld3: *mut u16,
fld4: usize,
fld5: i128,

},
Variant2{
fld0: (u128, f32, *mut i8),
fld1: Adt56,
fld2: f64,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt58{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt58 {
fld0: *const (u64, i64, *mut u16, f32),
fld1: Adt45,
fld2: (u64, char, i128, i128, char, u32, u8, u8),
fld3: Adt47,
fld4: Adt51,
}
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
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: (f64, *mut u16, f64, u32),
fld1: [u8; 3],
fld2: (u128, f32, *mut i8),
fld3: i8,
fld4: [i16; 7],
fld5: i32,
fld6: usize,
fld7: Adt50,

},
Variant1{
fld0: u32,

},
Variant2{
fld0: (u16, u128, bool, u64),
fld1: (i64,),

},
Variant3{
fld0: f64,
fld1: Adt56,
fld2: [u8; 3],
fld3: *mut u16,
fld4: i16,
fld5: Adt45,
fld6: u64,
fld7: i128,

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt60::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: [i8; 3],
fld1: (u128,),
fld2: isize,
fld3: i64,
fld4: u32,
fld5: Adt54,

},
Variant1{
fld0: (u128,),
fld1: Adt50,
fld2: f64,
fld3: (u16, u128, bool, u64),
fld4: u64,
fld5: ((u128, f32, *mut i8), i8, u64, *mut i8),

},
Variant2{
fld0: (*mut u16,),
fld1: Adt56,
fld2: isize,

},
Variant3{
fld0: Adt47,
fld1: f64,
fld2: isize,
fld3: i8,
fld4: Adt44,

}}

