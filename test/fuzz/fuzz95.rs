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
pub fn fn0(mut _1: i128,mut _2: u8,mut _3: usize,mut _4: u128,mut _5: i16,mut _6: i32,mut _7: i64) -> [isize; 8] {
mir! {
type RET = [isize; 8];
let _8: i128;
let _9: [bool; 8];
let _10: (i64, [i128; 6], u16, [i128; 3]);
let _11: i16;
let _12: f32;
let _13: f32;
let _14: (u64,);
let _15: Adt47;
let _16: i128;
let _17: u16;
let _18: (char, *mut &'static i128, char);
let _19: u16;
let _20: isize;
let _21: (i8, isize);
let _22: (char, *mut &'static i128, char);
let _23: f32;
let _24: isize;
let _25: (f32, i8, i128);
let _26: isize;
let _27: [i128; 4];
let _28: ([bool; 8],);
let _29: ();
let _30: ();
{
_6 = 112_isize as i32;
_3 = 5442273302603579363_i64 as usize;
_4 = 330134240817074745690593983425394319708_u128;
RET = [95_isize,(-9223372036854775808_isize),26_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_1 = -49186610967712546818229279128174833820_i128;
RET = [(-9223372036854775808_isize),9223372036854775807_isize,(-21_isize),9223372036854775807_isize,100_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_6 = (-1835702619_i32);
_7 = (-4627013363568643506_i64);
_2 = 27_u8 << _3;
RET = [(-9223372036854775808_isize),47_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-85_isize),(-71_isize),11_isize];
_8 = !_1;
_6 = 1686978962_i32 - 1442401977_i32;
RET = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_4 = 243811595_u32 as u128;
_7 = (-8554312077968603711_i64);
_10.0 = _1 as i64;
_5 = (-28944_i16) ^ 2848_i16;
_10.3 = [_1,_8,_1];
_10.2 = 28144_u16 << _1;
_11 = _5 ^ _5;
_10.3 = [_8,_8,_1];
_6 = -(-449880598_i32);
_1 = _8 * _8;
_10.3 = [_1,_1,_8];
_7 = _10.0;
_11 = _5 + _5;
_9 = [true,false,true,true,true,true,true,false];
Call(RET = fn1(_5, _9, _9, _10.3, _9, _2, _5, _1, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12 = _6 as f32;
_1 = _12 as i128;
_6 = false as i32;
RET = [(-88_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-19_isize),(-9223372036854775808_isize)];
RET = [9223372036854775807_isize,(-87_isize),98_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-109_isize),82_isize];
_3 = 5_usize + 7703561186447118389_usize;
_1 = _8;
_10.1 = [_1,_1,_8,_8,_8,_8];
_10.0 = _7;
Goto(bb2)
}
bb2 = {
_6 = true as i32;
_7 = _10.0 - _10.0;
_4 = 85733612536882822881379424284099354036_u128;
_7 = _10.0 >> _3;
_10.3 = [_8,_8,_1];
RET = [(-9223372036854775808_isize),9223372036854775807_isize,48_isize,(-4_isize),9223372036854775807_isize,(-72_isize),70_isize,9223372036854775807_isize];
Goto(bb3)
}
bb3 = {
_4 = !109836781830104289036463791060253105359_u128;
_14 = (18367834647598276371_u64,);
_5 = _11 ^ _11;
_14.0 = _5 as u64;
Call(_10.1 = fn12(_11, _5, _5, _11, _7, _9, _5, _9, _7, _4), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_14.0 = false as u64;
_1 = _8 * _8;
Call(_10.0 = core::intrinsics::transmute(_7), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_10.1 = [_1,_1,_1,_1,_1,_1];
_5 = 665644149_u32 as i16;
_5 = _12 as i16;
_10.1 = [_1,_8,_8,_1,_8,_1];
_7 = !_10.0;
_10.2 = _14.0 as u16;
Goto(bb6)
}
bb6 = {
_11 = -_5;
_10.2 = '\u{c1df4}' as u16;
_4 = !180347129046930691660243522310914117698_u128;
_10.2 = !52462_u16;
_12 = 1201794614_u32 as f32;
_10.0 = -_7;
_5 = -_11;
_10.1 = [_1,_8,_8,_1,_1,_1];
_5 = _10.2 as i16;
_11 = _5 + _5;
Goto(bb7)
}
bb7 = {
_11 = _5;
_12 = _6 as f32;
_10.0 = !_7;
_14.0 = 18440232531731550466_u64 - 12135126187660215305_u64;
_13 = _12;
_8 = !_1;
_1 = !_8;
_10.0 = '\u{3afe}' as i64;
_14.0 = _1 as u64;
_14 = (14794563507309015167_u64,);
_14 = (16545172691292687041_u64,);
_10.0 = _7;
_7 = !_10.0;
_9 = [false,true,false,false,true,true,true,false];
_5 = -_11;
_10.1 = [_8,_1,_1,_1,_8,_1];
_10.1 = [_8,_8,_8,_8,_1,_8];
_13 = -_12;
_7 = _10.0;
_3 = 6_usize;
_1 = _8;
Goto(bb8)
}
bb8 = {
RET[_3] = 9223372036854775807_isize;
_14 = (3514861292650150216_u64,);
_12 = -_13;
_6 = (-81001956_i32) >> _10.0;
RET[_3] = (-9223372036854775808_isize) + 114_isize;
_11 = _5 | _5;
RET[_3] = (-9223372036854775808_isize) | 9223372036854775807_isize;
_2 = 254_u8;
_5 = 11_i8 as i16;
_6 = (-513562449_i32) - 454331365_i32;
RET = [(-9223372036854775808_isize),(-25_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_7 = _10.0 * _10.0;
_16 = !_8;
_10.1 = [_8,_16,_16,_1,_16,_8];
_10.1 = [_16,_1,_16,_16,_1,_16];
_13 = -_12;
_14.0 = !10977944581119983916_u64;
_14 = (1097560313597484860_u64,);
_6 = 581891655_i32;
_9[_3] = !true;
Call(_1 = core::intrinsics::bswap(_16), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_12 = -_13;
_10.0 = _7;
_9 = [true,false,false,true,false,false,false,true];
_10.1 = [_16,_1,_8,_1,_8,_1];
RET[_3] = (-17_isize) << _16;
_10.1 = [_8,_16,_16,_16,_1,_1];
_14 = (2848765016671525185_u64,);
_5 = _11 | _11;
_4 = 135345296260812315393806642013970687592_u128 - 84776674809778633921964074040596011780_u128;
_10.0 = _7 | _7;
_11 = _5 ^ _5;
_5 = !_11;
_13 = _7 as f32;
_10.0 = !_7;
_18.2 = '\u{62337}';
_10.2 = _5 as u16;
_3 = _2 as usize;
_18.2 = '\u{ca95c}';
_8 = !_16;
_6 = _3 as i32;
Goto(bb10)
}
bb10 = {
_10.2 = !37618_u16;
match _14.0 {
2848765016671525185 => bb11,
_ => bb4
}
}
bb11 = {
_18.0 = _18.2;
_18.2 = _18.0;
_17 = _14.0 as u16;
_6 = 1350492218_i32;
_19 = _17;
_6 = (-1667181515_i32) & (-647386114_i32);
_2 = !70_u8;
_7 = _10.0;
match _14.0 {
0 => bb12,
1 => bb13,
2848765016671525185 => bb15,
_ => bb14
}
}
bb12 = {
_10.2 = !37618_u16;
match _14.0 {
2848765016671525185 => bb11,
_ => bb4
}
}
bb13 = {
_11 = _5;
_12 = _6 as f32;
_10.0 = !_7;
_14.0 = 18440232531731550466_u64 - 12135126187660215305_u64;
_13 = _12;
_8 = !_1;
_1 = !_8;
_10.0 = '\u{3afe}' as i64;
_14.0 = _1 as u64;
_14 = (14794563507309015167_u64,);
_14 = (16545172691292687041_u64,);
_10.0 = _7;
_7 = !_10.0;
_9 = [false,true,false,false,true,true,true,false];
_5 = -_11;
_10.1 = [_8,_1,_1,_1,_8,_1];
_10.1 = [_8,_8,_8,_8,_1,_8];
_13 = -_12;
_7 = _10.0;
_3 = 6_usize;
_1 = _8;
Goto(bb8)
}
bb14 = {
RET[_3] = 9223372036854775807_isize;
_14 = (3514861292650150216_u64,);
_12 = -_13;
_6 = (-81001956_i32) >> _10.0;
RET[_3] = (-9223372036854775808_isize) + 114_isize;
_11 = _5 | _5;
RET[_3] = (-9223372036854775808_isize) | 9223372036854775807_isize;
_2 = 254_u8;
_5 = 11_i8 as i16;
_6 = (-513562449_i32) - 454331365_i32;
RET = [(-9223372036854775808_isize),(-25_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_7 = _10.0 * _10.0;
_16 = !_8;
_10.1 = [_8,_16,_16,_1,_16,_8];
_10.1 = [_16,_1,_16,_16,_1,_16];
_13 = -_12;
_14.0 = !10977944581119983916_u64;
_14 = (1097560313597484860_u64,);
_6 = 581891655_i32;
_9[_3] = !true;
Call(_1 = core::intrinsics::bswap(_16), ReturnTo(bb9), UnwindUnreachable())
}
bb15 = {
_14 = (14443705889180510090_u64,);
_4 = 308858940587259951670321118314804821003_u128 - 182218673918417109486204454678372013159_u128;
_7 = _10.0;
_6 = -(-565553365_i32);
_20 = (-106_isize);
_22.0 = _18.2;
_6 = 77_i8 as i32;
_2 = !115_u8;
_10.2 = !_19;
_14.0 = !16125936554929252293_u64;
_23 = _5 as f32;
_11 = _5;
_25.1 = 2_i8 - (-29_i8);
_22.2 = _18.2;
_3 = _25.1 as usize;
_25.0 = -_23;
_10.1 = [_8,_8,_1,_16,_1,_1];
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(0_usize, 20_usize, Move(_20), 2_usize, Move(_2), 16_usize, Move(_16), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(0_usize, 5_usize, Move(_5), 8_usize, Move(_8), 9_usize, Move(_9), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i16,mut _2: [bool; 8],mut _3: [bool; 8],mut _4: [i128; 3],mut _5: [bool; 8],mut _6: u8,mut _7: i16,mut _8: i128,mut _9: [bool; 8]) -> [isize; 8] {
mir! {
type RET = [isize; 8];
let _10: bool;
let _11: isize;
let _12: *mut (usize, (*mut &'static i128, i64, (char, *mut &'static i128, char), (char, *mut &'static i128, char), u128, bool), i32, [u16; 6], [i128; 6]);
let _13: Adt50;
let _14: Adt46;
let _15: (usize, (*mut &'static i128, i64, (char, *mut &'static i128, char), (char, *mut &'static i128, char), u128, bool), i32, [u16; 6], [i128; 6]);
let _16: (f32, i8, i128);
let _17: (i64, [i128; 6], u16, [i128; 3]);
let _18: [char; 2];
let _19: [bool; 8];
let _20: [i128; 3];
let _21: Adt53;
let _22: isize;
let _23: [isize; 8];
let _24: u32;
let _25: isize;
let _26: [u16; 6];
let _27: i32;
let _28: i16;
let _29: Adt56;
let _30: [i128; 3];
let _31: isize;
let _32: i128;
let _33: char;
let _34: bool;
let _35: f64;
let _36: usize;
let _37: [i128; 6];
let _38: *const *mut &'static i128;
let _39: Adt55;
let _40: f32;
let _41: isize;
let _42: [i128; 6];
let _43: char;
let _44: f32;
let _45: f32;
let _46: ();
let _47: ();
{
_10 = !true;
RET = [9223372036854775807_isize,(-9223372036854775808_isize),(-57_isize),(-59_isize),9223372036854775807_isize,(-33_isize),9223372036854775807_isize,9223372036854775807_isize];
_9 = [_10,_10,_10,_10,_10,_10,_10,_10];
RET = [(-127_isize),(-9223372036854775808_isize),(-25_isize),86_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,46_isize];
RET = [96_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),84_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = [(-9223372036854775808_isize),(-56_isize),9223372036854775807_isize,9223372036854775807_isize,113_isize,(-57_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_10 = true;
_2 = [_10,_10,_10,_10,_10,_10,_10,_10];
RET = [9223372036854775807_isize,(-3_isize),48_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_2 = [_10,_10,_10,_10,_10,_10,_10,_10];
RET = [(-9223372036854775808_isize),49_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),108_isize,5_isize];
_11 = (-9223372036854775808_isize);
_9 = [_10,_10,_10,_10,_10,_10,_10,_10];
_2 = [_10,_10,_10,_10,_10,_10,_10,_10];
_7 = !_1;
_10 = _7 < _7;
Call(_11 = fn2(_2, _3, _10, _5, _6, RET, RET, _3, _6, _8, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _7 * _7;
_14.fld1 = (-1593566396_i32) | (-1246175090_i32);
_14.fld4 = (_3,);
_14.fld4 = (_3,);
_17.0 = 4315839652097488552_i64 + 5376646150462121341_i64;
_15.0 = 462932632793055340_usize * 1_usize;
_15.1.2.2 = '\u{5fc28}';
_4 = [_8,_8,_8];
_17.2 = 42056_u16;
_15.1.4 = 11389730391392379803_u64 as u128;
_16.0 = _6 as f32;
match _17.2 {
0 => bb2,
42056 => bb4,
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
_19 = [_10,_10,_10,_10,_10,_10,_10,_10];
_15.1.2.2 = '\u{d430}';
_17.3 = [_8,_8,_8];
_16.1 = 0_i8 * (-43_i8);
_15.1.1 = _17.0;
_17.0 = _15.1.1;
_15.2 = -_14.fld1;
_21.fld2 = _16.1 as isize;
_1 = 1252286445_u32 as i16;
_15.4 = [_8,_8,_8,_8,_8,_8];
_22 = _1 as isize;
_21.fld2 = _11;
_14.fld2 = [_15.1.1,_15.1.1,_15.1.1,_15.1.1,_17.0,_15.1.1,_15.1.1,_17.0];
_10 = true;
_12 = core::ptr::addr_of_mut!(_15);
_21.fld6.fld1 = core::ptr::addr_of!(_15.1.5);
_17.1 = _15.4;
_14.fld2 = [(*_12).1.1,(*_12).1.1,_15.1.1,_17.0,_15.1.1,(*_12).1.1,_17.0,(*_12).1.1];
_1 = _7;
_21.fld5 = (*_12).1.4 as f64;
(*_12).1.1 = _17.0;
match _17.2 {
0 => bb1,
1 => bb2,
42056 => bb5,
_ => bb3
}
}
bb5 = {
_20 = [_8,_8,_8];
(*_12).3 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
Call(RET = fn3(_3, _14.fld1, _15.1.1, Move(_12), _6, _5, _14.fld4, _21.fld5, (*_12).1.4, _4, _14.fld2, _5), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_9 = _14.fld4.0;
_21.fld1 = [_8,_8,_8,_8,_8,_8];
_16.2 = _8;
_21.fld7 = _8;
RET = [_11,_22,_21.fld2,_11,_22,_11,_21.fld2,_11];
_19 = [_10,_10,_15.1.5,_15.1.5,_10,_15.1.5,_15.1.5,_15.1.5];
_15.1.1 = _17.0;
_2 = _14.fld4.0;
_21.fld3 = -_16.0;
_11 = _22 + _21.fld2;
_12 = core::ptr::addr_of_mut!(_15);
_18 = [(*_12).1.2.0,(*_12).1.3.2];
(*_12).4 = [_16.2,_8,_8,_21.fld7,_16.2,_21.fld7];
_17.3 = [_16.2,_21.fld7,_8];
_14.fld0 = !_15.1.5;
_4 = _17.3;
_5 = _2;
(*_12).0 = 1_usize;
(*_12).0 = 10519448762752123848_usize - 3332003693267693637_usize;
_23 = [_21.fld2,_11,_22,_11,_11,_22,_11,_22];
_21.fld6.fld0 = 1691722721_u32;
_21.fld5 = (*_12).2 as f64;
(*_12).1.3.0 = _15.1.2.2;
(*_12).1.4 = !152417865552543499809546171762332359307_u128;
_21.fld6.fld0 = 94333014_u32 - 2617797259_u32;
match _17.2 {
0 => bb1,
1 => bb3,
2 => bb7,
42056 => bb9,
_ => bb8
}
}
bb7 = {
_20 = [_8,_8,_8];
(*_12).3 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
Call(RET = fn3(_3, _14.fld1, _15.1.1, Move(_12), _6, _5, _14.fld4, _21.fld5, (*_12).1.4, _4, _14.fld2, _5), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_1 = _7 * _7;
_14.fld1 = (-1593566396_i32) | (-1246175090_i32);
_14.fld4 = (_3,);
_14.fld4 = (_3,);
_17.0 = 4315839652097488552_i64 + 5376646150462121341_i64;
_15.0 = 462932632793055340_usize * 1_usize;
_15.1.2.2 = '\u{5fc28}';
_4 = [_8,_8,_8];
_17.2 = 42056_u16;
_15.1.4 = 11389730391392379803_u64 as u128;
_16.0 = _6 as f32;
match _17.2 {
0 => bb2,
42056 => bb4,
_ => bb3
}
}
bb9 = {
_21.fld2 = (*_12).0 as isize;
_21.fld4 = -_1;
(*_12).0 = 15090777712377530679_usize >> _16.2;
_30 = [_21.fld7,_8,_8];
_15.3 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
(*_12).1.3.0 = _15.1.3.2;
(*_12).1.3.0 = _15.1.3.2;
_21.fld6.fld0 = 1462758134_u32 >> (*_12).1.1;
RET = [_11,_22,_11,_22,_21.fld2,_22,_22,_22];
_33 = (*_12).1.2.0;
_21.fld6.fld1 = core::ptr::addr_of!((*_12).1.5);
_27 = (*_12).2 & _15.2;
_12 = core::ptr::addr_of_mut!(_15);
_36 = _22 as usize;
_4 = [_16.2,_21.fld7,_21.fld7];
_25 = _21.fld2 ^ _11;
_14.fld3 = _8;
(*_12).1.1 = _17.0;
_18 = [_33,(*_12).1.2.2];
_15.1.5 = _10;
_11 = _6 as isize;
_4 = _30;
_6 = 249_u8;
Goto(bb10)
}
bb10 = {
(*_12).0 = !_36;
_21.fld6.fld0 = 3556332535_u32;
_15.4 = [_14.fld3,_14.fld3,_21.fld7,_8,_16.2,_16.2];
_26 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
_28 = _7;
RET = [_21.fld2,_22,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_25,_11];
_15.1.3.0 = (*_12).1.2.2;
_16.2 = _21.fld7;
_15.1.2.2 = (*_12).1.3.2;
_35 = _14.fld3 as f64;
_26 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
Goto(bb11)
}
bb11 = {
_32 = _14.fld3 - _16.2;
_2 = [_15.1.5,(*_12).1.5,_10,_15.1.5,_14.fld0,_10,_14.fld0,_14.fld0];
_31 = !_11;
(*_12).4 = [_14.fld3,_8,_14.fld3,_16.2,_32,_32];
_25 = (*_12).1.1 as isize;
_37 = [_14.fld3,_14.fld3,_14.fld3,_21.fld7,_32,_14.fld3];
_14.fld0 = _10;
_21.fld3 = -_16.0;
_21.fld2 = _25 & _31;
_23 = [_21.fld2,_11,_21.fld2,_21.fld2,_31,_21.fld2,_11,_31];
_15.1.1 = _17.0 & _17.0;
_23 = [_31,_21.fld2,_31,_25,_25,_21.fld2,_11,_31];
_14.fld2 = [_15.1.1,(*_12).1.1,(*_12).1.1,_15.1.1,_15.1.1,(*_12).1.1,(*_12).1.1,(*_12).1.1];
(*_12).1.3.2 = (*_12).1.2.0;
_9 = _5;
_21.fld5 = _35 * _35;
(*_12).1.3.2 = _15.1.2.2;
_38 = core::ptr::addr_of!((*_12).1.0);
match _17.2 {
0 => bb5,
1 => bb7,
2 => bb3,
3 => bb8,
4 => bb12,
5 => bb13,
42056 => bb15,
_ => bb14
}
}
bb12 = {
(*_12).0 = !_36;
_21.fld6.fld0 = 3556332535_u32;
_15.4 = [_14.fld3,_14.fld3,_21.fld7,_8,_16.2,_16.2];
_26 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
_28 = _7;
RET = [_21.fld2,_22,_21.fld2,_21.fld2,_21.fld2,_21.fld2,_25,_11];
_15.1.3.0 = (*_12).1.2.2;
_16.2 = _21.fld7;
_15.1.2.2 = (*_12).1.3.2;
_35 = _14.fld3 as f64;
_26 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
Goto(bb11)
}
bb13 = {
_21.fld2 = (*_12).0 as isize;
_21.fld4 = -_1;
(*_12).0 = 15090777712377530679_usize >> _16.2;
_30 = [_21.fld7,_8,_8];
_15.3 = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
(*_12).1.3.0 = _15.1.3.2;
(*_12).1.3.0 = _15.1.3.2;
_21.fld6.fld0 = 1462758134_u32 >> (*_12).1.1;
RET = [_11,_22,_11,_22,_21.fld2,_22,_22,_22];
_33 = (*_12).1.2.0;
_21.fld6.fld1 = core::ptr::addr_of!((*_12).1.5);
_27 = (*_12).2 & _15.2;
_12 = core::ptr::addr_of_mut!(_15);
_36 = _22 as usize;
_4 = [_16.2,_21.fld7,_21.fld7];
_25 = _21.fld2 ^ _11;
_14.fld3 = _8;
(*_12).1.1 = _17.0;
_18 = [_33,(*_12).1.2.2];
_15.1.5 = _10;
_11 = _6 as isize;
_4 = _30;
_6 = 249_u8;
Goto(bb10)
}
bb14 = {
_9 = _14.fld4.0;
_21.fld1 = [_8,_8,_8,_8,_8,_8];
_16.2 = _8;
_21.fld7 = _8;
RET = [_11,_22,_21.fld2,_11,_22,_11,_21.fld2,_11];
_19 = [_10,_10,_15.1.5,_15.1.5,_10,_15.1.5,_15.1.5,_15.1.5];
_15.1.1 = _17.0;
_2 = _14.fld4.0;
_21.fld3 = -_16.0;
_11 = _22 + _21.fld2;
_12 = core::ptr::addr_of_mut!(_15);
_18 = [(*_12).1.2.0,(*_12).1.3.2];
(*_12).4 = [_16.2,_8,_8,_21.fld7,_16.2,_21.fld7];
_17.3 = [_16.2,_21.fld7,_8];
_14.fld0 = !_15.1.5;
_4 = _17.3;
_5 = _2;
(*_12).0 = 1_usize;
(*_12).0 = 10519448762752123848_usize - 3332003693267693637_usize;
_23 = [_21.fld2,_11,_22,_11,_11,_22,_11,_22];
_21.fld6.fld0 = 1691722721_u32;
_21.fld5 = (*_12).2 as f64;
(*_12).1.3.0 = _15.1.2.2;
(*_12).1.4 = !152417865552543499809546171762332359307_u128;
_21.fld6.fld0 = 94333014_u32 - 2617797259_u32;
match _17.2 {
0 => bb1,
1 => bb3,
2 => bb7,
42056 => bb9,
_ => bb8
}
}
bb15 = {
(*_12).0 = _17.2 as usize;
(*_12).1.5 = !_14.fld0;
_44 = _16.0 - _16.0;
Goto(bb16)
}
bb16 = {
Call(_46 = dump_var(1_usize, 5_usize, Move(_5), 17_usize, Move(_17), 30_usize, Move(_30), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(1_usize, 4_usize, Move(_4), 36_usize, Move(_36), 19_usize, Move(_19), 23_usize, Move(_23)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_46 = dump_var(1_usize, 2_usize, Move(_2), 6_usize, Move(_6), 31_usize, Move(_31), 3_usize, Move(_3)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_46 = dump_var(1_usize, 33_usize, Move(_33), 47_usize, _47, 47_usize, _47, 47_usize, _47), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: [bool; 8],mut _2: [bool; 8],mut _3: bool,mut _4: [bool; 8],mut _5: u8,mut _6: [isize; 8],mut _7: [isize; 8],mut _8: [bool; 8],mut _9: u8,mut _10: i128,mut _11: [bool; 8]) -> isize {
mir! {
type RET = isize;
let _12: Adt52;
let _13: Adt59;
let _14: u16;
let _15: bool;
let _16: Adt45;
let _17: u128;
let _18: ();
let _19: ();
{
_7 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,112_isize,(-10_isize),71_isize,(-9223372036854775808_isize)];
_6 = _7;
_11 = [_3,_3,_3,_3,_3,_3,_3,_3];
_7 = _6;
RET = (-9223372036854775808_isize);
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463454151235394913435648 => bb8,
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
_5 = _9;
_3 = !true;
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_4 = [_3,_3,_3,_3,_3,_3,_3,_3];
_7 = [RET,RET,RET,RET,RET,RET,RET,RET];
_6 = _7;
_10 = !(-61021949417228614718337583033292425387_i128);
match RET {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
340282366920938463454151235394913435648 => bb14,
_ => bb13
}
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
Return()
}
bb14 = {
_4 = _11;
RET = 9223372036854775807_isize ^ 55_isize;
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_8 = [_3,_3,_3,_3,_3,_3,_3,_3];
_9 = 21596_u16 as u8;
_15 = !_3;
_11 = [_3,_15,_15,_3,_15,_15,_15,_15];
_1 = [_15,_3,_3,_3,_3,_15,_3,_3];
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_1 = [_15,_15,_3,_3,_3,_15,_3,_15];
_14 = 299561629167450330432470747720290765545_u128 as u16;
RET = (-9223372036854775808_isize);
_9 = 12904656812555767018_usize as u8;
_10 = _9 as i128;
_10 = 2622725170_u32 as i128;
_14 = !12191_u16;
_4 = _2;
_11 = [_3,_3,_3,_3,_15,_15,_3,_15];
_7 = _6;
RET = 737539122730549859_i64 as isize;
_5 = 1441933247_u32 as u8;
_1 = [_15,_3,_15,_15,_15,_15,_15,_15];
Goto(bb15)
}
bb15 = {
Call(_18 = dump_var(2_usize, 1_usize, Move(_1), 3_usize, Move(_3), 10_usize, Move(_10), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_18 = dump_var(2_usize, 6_usize, Move(_6), 14_usize, Move(_14), 19_usize, _19, 19_usize, _19), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: [bool; 8],mut _2: i32,mut _3: i64,mut _4: *mut (usize, (*mut &'static i128, i64, (char, *mut &'static i128, char), (char, *mut &'static i128, char), u128, bool), i32, [u16; 6], [i128; 6]),mut _5: u8,mut _6: [bool; 8],mut _7: ([bool; 8],),mut _8: f64,mut _9: u128,mut _10: [i128; 3],mut _11: [i64; 8],mut _12: [bool; 8]) -> [isize; 8] {
mir! {
type RET = [isize; 8];
let _13: isize;
let _14: isize;
let _15: i8;
let _16: [u64; 8];
let _17: (char, *mut &'static i128, char);
let _18: Adt51;
let _19: char;
let _20: isize;
let _21: isize;
let _22: f32;
let _23: [isize; 8];
let _24: f64;
let _25: isize;
let _26: f32;
let _27: ([bool; 8],);
let _28: char;
let _29: bool;
let _30: Adt57;
let _31: (i64, [i128; 6], u16, [i128; 3]);
let _32: ([bool; 8],);
let _33: i64;
let _34: ();
let _35: ();
{
(*_4).3 = [8270_u16,47111_u16,22724_u16,14814_u16,48047_u16,22987_u16];
(*_4).1.2.0 = (*_4).1.2.2;
(*_4).1.3.0 = (*_4).1.2.0;
_7.0 = [false,false,false,true,false,false,true,false];
(*_4).1.3.2 = (*_4).1.2.2;
(*_4).1.2.2 = (*_4).1.3.0;
(*_4).1.5 = !false;
_2 = -(*_4).2;
RET = [13_isize,65_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),0_isize,(-9223372036854775808_isize)];
_3 = !(*_4).1.1;
(*_4).1.2.0 = (*_4).1.3.0;
_3 = (*_4).1.1 >> (*_4).0;
RET = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),46_isize,(-9223372036854775808_isize),38_isize];
(*_4).1.2.0 = (*_4).1.3.2;
(*_4).1.4 = _9 & _9;
_15 = _3 as i8;
(*_4).1.2.2 = (*_4).1.3.2;
Call(_17.1 = fn4((*_4).0, RET, RET, _1, (*_4).1.3.2, _7, (*_4).3, (*_4).3, (*_4).1.4, RET, _1, _12, (*_4).3, _1, (*_4).3, (*_4).3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_4).1.2.2 = (*_4).1.3.2;
_9 = (*_4).1.4;
(*_4).1.1 = _3 * _3;
_7.0 = _6;
_14 = 9223372036854775807_isize;
_17.0 = (*_4).1.3.0;
(*_4).0 = (-120890698714534754516786364049026667790_i128) as usize;
(*_4).0 = (*_4).1.1 as usize;
_17.2 = (*_4).1.2.0;
(*_4).1.5 = _14 < _14;
(*_4).1.2.2 = (*_4).1.2.0;
_8 = (*_4).0 as f64;
(*_4).1.3.0 = (*_4).1.2.0;
_8 = _5 as f64;
(*_4).1.4 = _9;
(*_4).1.2.0 = _17.2;
(*_4).1.3.2 = (*_4).1.3.0;
_13 = _15 as isize;
(*_4).0 = 2_usize;
_7 = (_6,);
Call(_5 = core::intrinsics::transmute((*_4).1.5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_18.fld0 = 4149529176_u32;
(*_4).3 = [28847_u16,49170_u16,40673_u16,8719_u16,30552_u16,13049_u16];
(*_4).1.4 = _9 >> (*_4).2;
_1 = _6;
(*_4).0 = 1655263327558537876_usize;
_6 = [(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5];
_18.fld0 = 2106616697_u32;
(*_4).3 = [32716_u16,58353_u16,45759_u16,36396_u16,14960_u16,52191_u16];
_21 = _13;
Call(_19 = fn9(_18.fld0, _1, (*_4).3, _7, _12, _7, (*_4).1.1, RET, Move(_17), (*_4).1.3.0, RET, (*_4).1.4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = [_21,_13,_21,_21,_14,_21,_13,_21];
_8 = _21 as f64;
_11 = [(*_4).1.1,(*_4).1.1,(*_4).1.1,(*_4).1.1,(*_4).1.1,(*_4).1.1,(*_4).1.1,(*_4).1.1];
(*_4).1.2.0 = (*_4).1.3.2;
_20 = (-100594402801189795635426562717468620129_i128) as isize;
_22 = 913_u16 as f32;
_16 = [5118765013649509671_u64,7278674907509642944_u64,13743334884852005047_u64,5959658963744721926_u64,14742259426594922686_u64,1378840866401292507_u64,3400920313188496490_u64,15347605682476106596_u64];
(*_4).1.3.0 = (*_4).1.2.0;
(*_4).2 = _2 & _2;
(*_4).2 = -_2;
Call((*_4).1.5 = fn11((*_4).0, (*_4).1.1, (*_4).3, _7, (*_4).1.3.2, (*_4).3, (*_4).3, _3, (*_4).1.4, _15), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
(*_4).1.1 = _3 ^ _3;
_17.0 = (*_4).1.3.0;
(*_4).3 = [51859_u16,30127_u16,41619_u16,63088_u16,50569_u16,56241_u16];
_24 = -_8;
(*_4).2 = _2 << (*_4).0;
_21 = _20;
_14 = !_13;
(*_4).1.5 = false;
(*_4).4 = [(-70693040731404370884070853941544398907_i128),(-12474470958051389229182772257764089076_i128),(-68392384284501431725746781347892182195_i128),29563152956836623160367358309067693297_i128,(-74530996200524364882531792106669853848_i128),72994503787632427022419909360524003521_i128];
_18.fld1 = core::ptr::addr_of!((*_4).1.5);
(*_4).1.2.0 = (*_4).1.3.2;
_23 = [_13,_13,_14,_13,_14,_14,_20,_14];
_11 = [(*_4).1.1,_3,(*_4).1.1,(*_4).1.1,(*_4).1.1,_3,(*_4).1.1,(*_4).1.1];
_10 = [(-160499854801755460928024221839372940216_i128),(-89100665719080399269870427126860613421_i128),132820048047563782837475628501085981774_i128];
_22 = 28701_u16 as f32;
_11 = [_3,(*_4).1.1,(*_4).1.1,(*_4).1.1,(*_4).1.1,(*_4).1.1,(*_4).1.1,(*_4).1.1];
_13 = _14;
_16 = [14250281407680252232_u64,16891810391344240584_u64,5326769720355239438_u64,10528325548681293448_u64,10630168588986979807_u64,10317271535576889509_u64,142843114456913979_u64,785919994704550346_u64];
match (*_4).0 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
1655263327558537876 => bb11,
_ => bb10
}
}
bb5 = {
RET = [_21,_13,_21,_21,_14,_21,_13,_21];
_8 = _21 as f64;
_11 = [(*_4).1.1,(*_4).1.1,(*_4).1.1,(*_4).1.1,(*_4).1.1,(*_4).1.1,(*_4).1.1,(*_4).1.1];
(*_4).1.2.0 = (*_4).1.3.2;
_20 = (-100594402801189795635426562717468620129_i128) as isize;
_22 = 913_u16 as f32;
_16 = [5118765013649509671_u64,7278674907509642944_u64,13743334884852005047_u64,5959658963744721926_u64,14742259426594922686_u64,1378840866401292507_u64,3400920313188496490_u64,15347605682476106596_u64];
(*_4).1.3.0 = (*_4).1.2.0;
(*_4).2 = _2 & _2;
(*_4).2 = -_2;
Call((*_4).1.5 = fn11((*_4).0, (*_4).1.1, (*_4).3, _7, (*_4).1.3.2, (*_4).3, (*_4).3, _3, (*_4).1.4, _15), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_18.fld0 = 4149529176_u32;
(*_4).3 = [28847_u16,49170_u16,40673_u16,8719_u16,30552_u16,13049_u16];
(*_4).1.4 = _9 >> (*_4).2;
_1 = _6;
(*_4).0 = 1655263327558537876_usize;
_6 = [(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5];
_18.fld0 = 2106616697_u32;
(*_4).3 = [32716_u16,58353_u16,45759_u16,36396_u16,14960_u16,52191_u16];
_21 = _13;
Call(_19 = fn9(_18.fld0, _1, (*_4).3, _7, _12, _7, (*_4).1.1, RET, Move(_17), (*_4).1.3.0, RET, (*_4).1.4), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
(*_4).1.2.2 = (*_4).1.3.2;
_9 = (*_4).1.4;
(*_4).1.1 = _3 * _3;
_7.0 = _6;
_14 = 9223372036854775807_isize;
_17.0 = (*_4).1.3.0;
(*_4).0 = (-120890698714534754516786364049026667790_i128) as usize;
(*_4).0 = (*_4).1.1 as usize;
_17.2 = (*_4).1.2.0;
(*_4).1.5 = _14 < _14;
(*_4).1.2.2 = (*_4).1.2.0;
_8 = (*_4).0 as f64;
(*_4).1.3.0 = (*_4).1.2.0;
_8 = _5 as f64;
(*_4).1.4 = _9;
(*_4).1.2.0 = _17.2;
(*_4).1.3.2 = (*_4).1.3.0;
_13 = _15 as isize;
(*_4).0 = 2_usize;
_7 = (_6,);
Call(_5 = core::intrinsics::transmute((*_4).1.5), ReturnTo(bb2), UnwindUnreachable())
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
_22 = _8 as f32;
(*_4).0 = 6237513664217942717_usize;
_14 = 11690313057613611569_u64 as isize;
(*_4).2 = !_2;
(*_4).1.2.0 = (*_4).1.3.0;
_7 = (_1,);
(*_4).1.2.0 = (*_4).1.2.2;
(*_4).1.4 = !_9;
(*_4).1.5 = true & true;
_28 = (*_4).1.3.0;
_5 = (*_4).0 as u8;
_22 = (*_4).2 as f32;
_20 = !_13;
(*_4).1.3.2 = (*_4).1.3.0;
_19 = (*_4).1.3.2;
_24 = (*_4).0 as f64;
(*_4).1.4 = !_9;
(*_4).0 = _18.fld0 as usize;
Goto(bb12)
}
bb12 = {
_26 = _22;
(*_4).1.5 = false;
_31.3 = [168074786993137138440192399922623463102_i128,157109542322956826046760278835018497092_i128,(-164053268048300823255312890638451369929_i128)];
(*_4).1.5 = false;
_28 = (*_4).1.3.2;
_15 = 40210_u16 as i8;
_7 = (_1,);
(*_4).1.4 = !_9;
(*_4).1.2.2 = _19;
_20 = _13 << (*_4).2;
_31.1 = (*_4).4;
_32 = (_1,);
(*_4).1.4 = _9 + _9;
_5 = !128_u8;
_22 = _26 - _26;
match _18.fld0 {
0 => bb13,
1 => bb14,
2 => bb15,
2106616697 => bb17,
_ => bb16
}
}
bb13 = {
_22 = _8 as f32;
(*_4).0 = 6237513664217942717_usize;
_14 = 11690313057613611569_u64 as isize;
(*_4).2 = !_2;
(*_4).1.2.0 = (*_4).1.3.0;
_7 = (_1,);
(*_4).1.2.0 = (*_4).1.2.2;
(*_4).1.4 = !_9;
(*_4).1.5 = true & true;
_28 = (*_4).1.3.0;
_5 = (*_4).0 as u8;
_22 = (*_4).2 as f32;
_20 = !_13;
(*_4).1.3.2 = (*_4).1.3.0;
_19 = (*_4).1.3.2;
_24 = (*_4).0 as f64;
(*_4).1.4 = !_9;
(*_4).0 = _18.fld0 as usize;
Goto(bb12)
}
bb14 = {
(*_4).1.2.2 = (*_4).1.3.2;
_9 = (*_4).1.4;
(*_4).1.1 = _3 * _3;
_7.0 = _6;
_14 = 9223372036854775807_isize;
_17.0 = (*_4).1.3.0;
(*_4).0 = (-120890698714534754516786364049026667790_i128) as usize;
(*_4).0 = (*_4).1.1 as usize;
_17.2 = (*_4).1.2.0;
(*_4).1.5 = _14 < _14;
(*_4).1.2.2 = (*_4).1.2.0;
_8 = (*_4).0 as f64;
(*_4).1.3.0 = (*_4).1.2.0;
_8 = _5 as f64;
(*_4).1.4 = _9;
(*_4).1.2.0 = _17.2;
(*_4).1.3.2 = (*_4).1.3.0;
_13 = _15 as isize;
(*_4).0 = 2_usize;
_7 = (_6,);
Call(_5 = core::intrinsics::transmute((*_4).1.5), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
_18.fld0 = 4149529176_u32;
(*_4).3 = [28847_u16,49170_u16,40673_u16,8719_u16,30552_u16,13049_u16];
(*_4).1.4 = _9 >> (*_4).2;
_1 = _6;
(*_4).0 = 1655263327558537876_usize;
_6 = [(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5];
_18.fld0 = 2106616697_u32;
(*_4).3 = [32716_u16,58353_u16,45759_u16,36396_u16,14960_u16,52191_u16];
_21 = _13;
Call(_19 = fn9(_18.fld0, _1, (*_4).3, _7, _12, _7, (*_4).1.1, RET, Move(_17), (*_4).1.3.0, RET, (*_4).1.4), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
(*_4).1.2.2 = (*_4).1.3.2;
_9 = (*_4).1.4;
(*_4).1.1 = _3 * _3;
_7.0 = _6;
_14 = 9223372036854775807_isize;
_17.0 = (*_4).1.3.0;
(*_4).0 = (-120890698714534754516786364049026667790_i128) as usize;
(*_4).0 = (*_4).1.1 as usize;
_17.2 = (*_4).1.2.0;
(*_4).1.5 = _14 < _14;
(*_4).1.2.2 = (*_4).1.2.0;
_8 = (*_4).0 as f64;
(*_4).1.3.0 = (*_4).1.2.0;
_8 = _5 as f64;
(*_4).1.4 = _9;
(*_4).1.2.0 = _17.2;
(*_4).1.3.2 = (*_4).1.3.0;
_13 = _15 as isize;
(*_4).0 = 2_usize;
_7 = (_6,);
Call(_5 = core::intrinsics::transmute((*_4).1.5), ReturnTo(bb2), UnwindUnreachable())
}
bb17 = {
_27 = (_7.0,);
_6 = [(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5];
(*_4).1.3.2 = (*_4).1.2.0;
(*_4).1.2.0 = (*_4).1.3.2;
_31.0 = !(*_4).1.1;
(*_4).1.3.2 = (*_4).1.2.2;
_17.2 = (*_4).1.2.2;
_32 = _7;
(*_4).1.3.2 = (*_4).1.2.0;
_8 = _24 - _24;
_8 = -_24;
_23 = RET;
_1 = _27.0;
_32.0 = _12;
(*_4).2 = !_2;
RET = [_20,_20,_13,_13,_20,_21,_13,_20];
(*_4).2 = !_2;
_32.0 = [(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5,(*_4).1.5];
(*_4).4 = [9765391545684203595362077725954577462_i128,157049509392285575965683581039009641203_i128,(-25877766836967544433411754431952644128_i128),(-144028581372269594366707657754454625097_i128),106850036329049878495323735611618099444_i128,93918292231333654130012029822162566436_i128];
Goto(bb18)
}
bb18 = {
Call(_34 = dump_var(3_usize, 7_usize, Move(_7), 13_usize, Move(_13), 1_usize, Move(_1), 16_usize, Move(_16)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_34 = dump_var(3_usize, 10_usize, Move(_10), 32_usize, Move(_32), 14_usize, Move(_14), 21_usize, Move(_21)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_34 = dump_var(3_usize, 3_usize, Move(_3), 20_usize, Move(_20), 35_usize, _35, 35_usize, _35), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: usize,mut _2: [isize; 8],mut _3: [isize; 8],mut _4: [bool; 8],mut _5: char,mut _6: ([bool; 8],),mut _7: [u16; 6],mut _8: [u16; 6],mut _9: u128,mut _10: [isize; 8],mut _11: [bool; 8],mut _12: [bool; 8],mut _13: [u16; 6],mut _14: [bool; 8],mut _15: [u16; 6],mut _16: [u16; 6]) -> *mut &'static i128 {
mir! {
type RET = *mut &'static i128;
let _17: Adt56;
let _18: isize;
let _19: f64;
let _20: [isize; 8];
let _21: f64;
let _22: [i128; 6];
let _23: [i128; 6];
let _24: u32;
let _25: char;
let _26: isize;
let _27: f32;
let _28: i128;
let _29: [isize; 8];
let _30: i32;
let _31: &'static i128;
let _32: isize;
let _33: f32;
let _34: usize;
let _35: ();
let _36: ();
{
_8 = _15;
_9 = 201705668060585125027214426043621323729_u128 + 257131511790249576961035608732310407473_u128;
_3 = _2;
_2 = _10;
_16 = [3930_u16,60795_u16,10541_u16,55848_u16,48734_u16,22924_u16];
_2 = [104_isize,(-9223372036854775808_isize),(-104_isize),76_isize,43_isize,(-9223372036854775808_isize),99_isize,9223372036854775807_isize];
_6.0 = _14;
_9 = 9692841654682129061544850682936907599_u128 << _1;
_7 = [3896_u16,16270_u16,11164_u16,62636_u16,52411_u16,45160_u16];
_6 = (_12,);
_10 = _2;
_10 = [(-99_isize),7_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-21_isize)];
_13 = _16;
_14 = [true,true,true,true,false,true,false,true];
_12 = [true,false,true,true,false,false,false,true];
_11 = [false,true,true,false,false,false,true,true];
_11 = [true,false,false,false,false,false,true,false];
_18 = 9223372036854775807_isize;
_5 = '\u{5bf1c}';
Goto(bb1)
}
bb1 = {
_14 = [false,false,true,true,true,false,false,true];
_12 = _11;
_16 = _8;
_12 = _6.0;
_8 = [30094_u16,13931_u16,33854_u16,2693_u16,59205_u16,59846_u16];
_6 = (_14,);
_14 = _12;
_7 = [56015_u16,2453_u16,27110_u16,38477_u16,58516_u16,7172_u16];
_18 = !(-9223372036854775808_isize);
_6.0 = [false,false,false,true,true,true,false,false];
_4 = [false,true,false,true,false,true,false,false];
_1 = 6_usize;
_6 = (_14,);
_2 = _3;
match _3[_1] {
340282366920938463454151235394913435648 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_20[_1] = _10[_1] << _10[_1];
_18 = _20[_1] ^ _10[_1];
_18 = _4[_1] as isize;
_21 = 146685922117493270362965359978811610331_i128 as f64;
_10[_1] = 2906697769_u32 as isize;
_12 = _14;
_3[_1] = _2[_1];
_25 = _5;
_4 = [_11[_1],_6.0[_1],_12[_1],_11[_1],_6.0[_1],_12[_1],_12[_1],_14[_1]];
_13 = _7;
_20[_1] = _6.0[_1] as isize;
_6.0[_1] = _11[_1];
_3 = [_2[_1],_2[_1],_18,_20[_1],_18,_18,_2[_1],_20[_1]];
_13 = _16;
_16 = [45088_u16,51670_u16,55019_u16,14133_u16,54295_u16,43792_u16];
_11 = _14;
_1 = 9672704380953551346_usize ^ 3090244168424452259_usize;
_19 = _21 * _21;
_23 = [149766984652099442730505622850599361556_i128,143172906167525861975135682219473720414_i128,(-123742848095012053810206974573537734452_i128),(-86493221623146500604610391098056312692_i128),141737251865338483724248533647892958064_i128,154848570408753006267475073075324329001_i128];
_2 = _10;
_16 = [48497_u16,59918_u16,35601_u16,9095_u16,40702_u16,433_u16];
_16 = _13;
_13 = [13742_u16,27959_u16,17558_u16,24813_u16,44777_u16,65161_u16];
Call(_13 = fn5(_6.0, _4, _4, _4, _4, _21, _14, _2, _2, _18, _6.0, _15, _12), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_25 = _5;
_1 = 3_usize;
match _23[_1] {
0 => bb2,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
253789145297791962858764216333711898764 => bb10,
_ => bb9
}
}
bb5 = {
_20[_1] = _10[_1] << _10[_1];
_18 = _20[_1] ^ _10[_1];
_18 = _4[_1] as isize;
_21 = 146685922117493270362965359978811610331_i128 as f64;
_10[_1] = 2906697769_u32 as isize;
_12 = _14;
_3[_1] = _2[_1];
_25 = _5;
_4 = [_11[_1],_6.0[_1],_12[_1],_11[_1],_6.0[_1],_12[_1],_12[_1],_14[_1]];
_13 = _7;
_20[_1] = _6.0[_1] as isize;
_6.0[_1] = _11[_1];
_3 = [_2[_1],_2[_1],_18,_20[_1],_18,_18,_2[_1],_20[_1]];
_13 = _16;
_16 = [45088_u16,51670_u16,55019_u16,14133_u16,54295_u16,43792_u16];
_11 = _14;
_1 = 9672704380953551346_usize ^ 3090244168424452259_usize;
_19 = _21 * _21;
_23 = [149766984652099442730505622850599361556_i128,143172906167525861975135682219473720414_i128,(-123742848095012053810206974573537734452_i128),(-86493221623146500604610391098056312692_i128),141737251865338483724248533647892958064_i128,154848570408753006267475073075324329001_i128];
_2 = _10;
_16 = [48497_u16,59918_u16,35601_u16,9095_u16,40702_u16,433_u16];
_16 = _13;
_13 = [13742_u16,27959_u16,17558_u16,24813_u16,44777_u16,65161_u16];
Call(_13 = fn5(_6.0, _4, _4, _4, _4, _21, _14, _2, _2, _18, _6.0, _15, _12), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
_14 = [false,false,true,true,true,false,false,true];
_12 = _11;
_16 = _8;
_12 = _6.0;
_8 = [30094_u16,13931_u16,33854_u16,2693_u16,59205_u16,59846_u16];
_6 = (_14,);
_14 = _12;
_7 = [56015_u16,2453_u16,27110_u16,38477_u16,58516_u16,7172_u16];
_18 = !(-9223372036854775808_isize);
_6.0 = [false,false,false,true,true,true,false,false];
_4 = [false,true,false,true,false,true,false,false];
_1 = 6_usize;
_6 = (_14,);
_2 = _3;
match _3[_1] {
340282366920938463454151235394913435648 => bb3,
_ => bb2
}
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_7 = _8;
_19 = -_21;
_17 = Adt56::Variant0 { fld0: 8648505861592932896_u64,fld1: _25 };
_23 = [118682460449381564616546911316337021563_i128,(-169851553923439769192317488594881018658_i128),14261398473401083345313687642539649470_i128,(-70872576904382994690312891279348593966_i128),135028207651682568865843545748273077466_i128,61737788559654692053868676339453364279_i128];
_26 = _3[_1];
_2[_1] = _10[_1] >> _3[_1];
_21 = -_19;
_16 = [_8[_1],_15[_1],_13[_1],_7[_1],_8[_1],_15[_1]];
_8[_1] = _5 as u16;
_6.0[_1] = !_11[_1];
_3 = [_26,_26,_18,_2[_1],_2[_1],_26,_18,_2[_1]];
_7 = [_13[_1],_16[_1],_15[_1],_15[_1],_16[_1],_13[_1]];
_10[_1] = _2[_1];
place!(Field::<u64>(Variant(_17, 0), 0)) = _23[_1] as u64;
_26 = _2[_1];
_3 = _2;
_23[_1] = -32696437165397735779716841479961492368_i128;
_16[_1] = _7[_1] / _15[_1];
_4[_1] = _6.0[_1] & _12[_1];
_5 = _25;
match _7[_1] {
0 => bb2,
1 => bb11,
2 => bb12,
3 => bb13,
14814 => bb15,
_ => bb14
}
}
bb11 = {
Return()
}
bb12 = {
_20[_1] = _10[_1] << _10[_1];
_18 = _20[_1] ^ _10[_1];
_18 = _4[_1] as isize;
_21 = 146685922117493270362965359978811610331_i128 as f64;
_10[_1] = 2906697769_u32 as isize;
_12 = _14;
_3[_1] = _2[_1];
_25 = _5;
_4 = [_11[_1],_6.0[_1],_12[_1],_11[_1],_6.0[_1],_12[_1],_12[_1],_14[_1]];
_13 = _7;
_20[_1] = _6.0[_1] as isize;
_6.0[_1] = _11[_1];
_3 = [_2[_1],_2[_1],_18,_20[_1],_18,_18,_2[_1],_20[_1]];
_13 = _16;
_16 = [45088_u16,51670_u16,55019_u16,14133_u16,54295_u16,43792_u16];
_11 = _14;
_1 = 9672704380953551346_usize ^ 3090244168424452259_usize;
_19 = _21 * _21;
_23 = [149766984652099442730505622850599361556_i128,143172906167525861975135682219473720414_i128,(-123742848095012053810206974573537734452_i128),(-86493221623146500604610391098056312692_i128),141737251865338483724248533647892958064_i128,154848570408753006267475073075324329001_i128];
_2 = _10;
_16 = [48497_u16,59918_u16,35601_u16,9095_u16,40702_u16,433_u16];
_16 = _13;
_13 = [13742_u16,27959_u16,17558_u16,24813_u16,44777_u16,65161_u16];
Call(_13 = fn5(_6.0, _4, _4, _4, _4, _21, _14, _2, _2, _18, _6.0, _15, _12), ReturnTo(bb4), UnwindUnreachable())
}
bb13 = {
_14 = [false,false,true,true,true,false,false,true];
_12 = _11;
_16 = _8;
_12 = _6.0;
_8 = [30094_u16,13931_u16,33854_u16,2693_u16,59205_u16,59846_u16];
_6 = (_14,);
_14 = _12;
_7 = [56015_u16,2453_u16,27110_u16,38477_u16,58516_u16,7172_u16];
_18 = !(-9223372036854775808_isize);
_6.0 = [false,false,false,true,true,true,false,false];
_4 = [false,true,false,true,false,true,false,false];
_1 = 6_usize;
_6 = (_14,);
_2 = _3;
match _3[_1] {
340282366920938463454151235394913435648 => bb3,
_ => bb2
}
}
bb14 = {
Return()
}
bb15 = {
_14[_1] = !_12[_1];
_29[_1] = -_2[_1];
_11[_1] = !_14[_1];
_20 = [_29[_1],_2[_1],_3[_1],_29[_1],_10[_1],_2[_1],_18,_3[_1]];
_13 = _8;
_16 = _7;
_4[_1] = _15[_1] != _7[_1];
_29 = _20;
place!(Field::<char>(Variant(_17, 0), 1)) = _25;
_2 = _29;
_19 = _3[_1] as f64;
_25 = Field::<char>(Variant(_17, 0), 1);
SetDiscriminant(_17, 2);
_20[_1] = _3[_1];
_9 = 166015746761932608181583810499496518730_u128 | 85204601346195933195458656929153653963_u128;
_14 = [_12[_1],_12[_1],_12[_1],_12[_1],_6.0[_1],_4[_1],_6.0[_1],_11[_1]];
_2 = _20;
place!(Field::<[i64; 8]>(Variant(_17, 2), 0))[_1] = 11498_i16 as i64;
_14 = [_4[_1],_4[_1],_11[_1],_4[_1],_4[_1],_6.0[_1],_11[_1],_11[_1]];
_6.0[_1] = _12[_1] < _14[_1];
Goto(bb16)
}
bb16 = {
_26 = _9 as isize;
_16 = _15;
_30 = (-6548534_i32) << _16[_1];
_8[_1] = !_7[_1];
place!(Field::<usize>(Variant(_17, 2), 6)) = _1;
_2[_1] = _30 as isize;
_10 = [_3[_1],_2[_1],_29[_1],_3[_1],_20[_1],_2[_1],_3[_1],_29[_1]];
place!(Field::<usize>(Variant(_17, 2), 6)) = _1;
_12 = _4;
_15 = _7;
_21 = _19 * _19;
RET = core::ptr::addr_of_mut!(_31);
_5 = _25;
_13 = [_15[_1],_8[_1],_7[_1],_8[_1],_16[_1],_8[_1]];
_7 = [_8[_1],_13[_1],_8[_1],_13[_1],_16[_1],_16[_1]];
Goto(bb17)
}
bb17 = {
Call(_35 = dump_var(4_usize, 15_usize, Move(_15), 2_usize, Move(_2), 8_usize, Move(_8), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_35 = dump_var(4_usize, 1_usize, Move(_1), 4_usize, Move(_4), 20_usize, Move(_20), 9_usize, Move(_9)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_35 = dump_var(4_usize, 25_usize, Move(_25), 3_usize, Move(_3), 11_usize, Move(_11), 36_usize, _36), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [bool; 8],mut _2: [bool; 8],mut _3: [bool; 8],mut _4: [bool; 8],mut _5: [bool; 8],mut _6: f64,mut _7: [bool; 8],mut _8: [isize; 8],mut _9: [isize; 8],mut _10: isize,mut _11: [bool; 8],mut _12: [u16; 6],mut _13: [bool; 8]) -> [u16; 6] {
mir! {
type RET = [u16; 6];
let _14: f64;
let _15: *const *mut &'static i128;
let _16: i32;
let _17: *mut usize;
let _18: isize;
let _19: Adt47;
let _20: char;
let _21: char;
let _22: bool;
let _23: Adt61;
let _24: isize;
let _25: f64;
let _26: (i8, i32);
let _27: (f32, i8, i128);
let _28: *mut (usize, (*mut &'static i128, i64, (char, *mut &'static i128, char), (char, *mut &'static i128, char), u128, bool), i32, [u16; 6], [i128; 6]);
let _29: Adt49;
let _30: i64;
let _31: (f32, i8, i128);
let _32: Adt53;
let _33: isize;
let _34: u32;
let _35: (i8, i32);
let _36: [i64; 8];
let _37: isize;
let _38: u16;
let _39: isize;
let _40: [isize; 8];
let _41: (i8, isize);
let _42: u128;
let _43: usize;
let _44: isize;
let _45: usize;
let _46: ();
let _47: ();
{
_5 = [false,false,false,true,false,true,false,true];
_3 = [true,false,true,true,false,false,true,true];
_12 = [45215_u16,26589_u16,43103_u16,27444_u16,13206_u16,4017_u16];
_9 = [_10,_10,_10,_10,_10,_10,_10,_10];
_3 = [true,true,true,true,true,false,false,false];
_4 = [false,true,false,true,false,false,false,true];
_9 = _8;
_12 = [9472_u16,29064_u16,38583_u16,148_u16,29759_u16,59409_u16];
_2 = _4;
_10 = 79_isize * (-5_isize);
_6 = 2161107620_u32 as f64;
_9 = [_10,_10,_10,_10,_10,_10,_10,_10];
RET = [45441_u16,56971_u16,39130_u16,41717_u16,59425_u16,48154_u16];
_7 = [false,false,true,false,false,false,false,false];
_1 = [false,true,false,false,false,true,true,true];
_1 = [false,false,true,false,false,false,true,false];
RET = [51770_u16,21244_u16,64931_u16,42864_u16,25762_u16,574_u16];
_2 = [false,true,false,true,true,true,false,false];
_8 = _9;
_9 = [_10,_10,_10,_10,_10,_10,_10,_10];
_11 = _13;
_4 = [true,false,true,true,true,true,false,false];
_14 = _6 + _6;
_2 = _7;
_1 = [false,true,true,false,true,false,false,true];
_14 = _6 * _6;
_8 = [_10,_10,_10,_10,_10,_10,_10,_10];
_11 = [false,false,false,true,false,true,false,false];
Call(_12 = core::intrinsics::transmute(RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = [false,true,true,true,false,true,false,true];
_3 = _5;
_4 = [true,false,false,false,true,true,true,true];
_11 = [true,true,true,true,false,true,true,false];
_2 = [false,false,true,true,true,false,true,true];
_14 = _6;
_5 = _3;
_9 = [_10,_10,_10,_10,_10,_10,_10,_10];
_21 = '\u{739e3}';
_6 = -_14;
_22 = _14 < _6;
_16 = !(-1659884473_i32);
_6 = -_14;
_20 = _21;
Goto(bb2)
}
bb2 = {
_13 = [_22,_22,_22,_22,_22,_22,_22,_22];
_18 = !_10;
_9 = [_10,_10,_18,_10,_10,_18,_18,_18];
_2 = [_22,_22,_22,_22,_22,_22,_22,_22];
RET = [4813_u16,30504_u16,45056_u16,30974_u16,31201_u16,37191_u16];
_3 = [_22,_22,_22,_22,_22,_22,_22,_22];
_19 = Adt47::Variant1 { fld0: _22,fld1: _9 };
RET = _12;
RET = [43686_u16,22535_u16,198_u16,36879_u16,2443_u16,7995_u16];
Goto(bb3)
}
bb3 = {
_1 = [_22,Field::<bool>(Variant(_19, 1), 0),_22,Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),_22,_22,Field::<bool>(Variant(_19, 1), 0)];
RET = _12;
_11 = _4;
_18 = _10;
_5 = [_22,_22,Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),_22,_22,Field::<bool>(Variant(_19, 1), 0)];
_12 = RET;
SetDiscriminant(_19, 1);
_6 = _14 + _14;
_8 = _9;
_21 = _20;
_27.0 = 158220442814034458910381015452978956212_u128 as f32;
_18 = _10;
_3 = [_22,_22,_22,_22,_22,_22,_22,_22];
_26.1 = 16110360537836693551_u64 as i32;
Goto(bb4)
}
bb4 = {
_24 = _10;
place!(Field::<bool>(Variant(_19, 1), 0)) = _22 ^ _22;
_18 = _10 | _24;
_11 = [Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),_22,_22];
_16 = 0_usize as i32;
Goto(bb5)
}
bb5 = {
_13 = [_22,Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0)];
place!(Field::<[isize; 8]>(Variant(_19, 1), 1)) = [_18,_10,_18,_18,_10,_10,_18,_24];
_29.fld0 = (_27.0, 112_i8, 139446255506036634310526722264395766371_i128);
match _29.fld0.1 {
0 => bb3,
1 => bb6,
2 => bb7,
112 => bb9,
_ => bb8
}
}
bb6 = {
_24 = _10;
place!(Field::<bool>(Variant(_19, 1), 0)) = _22 ^ _22;
_18 = _10 | _24;
_11 = [Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),_22,_22];
_16 = 0_usize as i32;
Goto(bb5)
}
bb7 = {
_1 = [_22,Field::<bool>(Variant(_19, 1), 0),_22,Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),_22,_22,Field::<bool>(Variant(_19, 1), 0)];
RET = _12;
_11 = _4;
_18 = _10;
_5 = [_22,_22,Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),Field::<bool>(Variant(_19, 1), 0),_22,_22,Field::<bool>(Variant(_19, 1), 0)];
_12 = RET;
SetDiscriminant(_19, 1);
_6 = _14 + _14;
_8 = _9;
_21 = _20;
_27.0 = 158220442814034458910381015452978956212_u128 as f32;
_18 = _10;
_3 = [_22,_22,_22,_22,_22,_22,_22,_22];
_26.1 = 16110360537836693551_u64 as i32;
Goto(bb4)
}
bb8 = {
_13 = [_22,_22,_22,_22,_22,_22,_22,_22];
_18 = !_10;
_9 = [_10,_10,_18,_10,_10,_18,_18,_18];
_2 = [_22,_22,_22,_22,_22,_22,_22,_22];
RET = [4813_u16,30504_u16,45056_u16,30974_u16,31201_u16,37191_u16];
_3 = [_22,_22,_22,_22,_22,_22,_22,_22];
_19 = Adt47::Variant1 { fld0: _22,fld1: _9 };
RET = _12;
RET = [43686_u16,22535_u16,198_u16,36879_u16,2443_u16,7995_u16];
Goto(bb3)
}
bb9 = {
_27 = (_29.fld0.0, _29.fld0.1, _29.fld0.2);
_4 = _7;
SetDiscriminant(_19, 2);
_29.fld4 = [(-446342782042624470_i64),8242245018976698440_i64,7722946486237361753_i64,8536157099547386562_i64,(-1299299369972839402_i64),(-6883089135560191273_i64),9208327589371774209_i64,981688495831279007_i64];
_26 = (_27.1, _16);
_32.fld4 = 57406759940291120092834231031366395923_u128 as i16;
_29.fld2 = core::ptr::addr_of!(_22);
place!(Field::<(i64, [i128; 6], u16, [i128; 3])>(Variant(_19, 2), 0)).3 = [_29.fld0.2,_27.2,_29.fld0.2];
_35.1 = _16 ^ _26.1;
_5 = [_22,_22,_22,_22,_22,_22,_22,_22];
_30 = -5181295710725917208_i64;
_29.fld6 = _32.fld4 as i64;
Goto(bb10)
}
bb10 = {
_32.fld1 = [_27.2,_27.2,_27.2,_29.fld0.2,_27.2,_27.2];
_32.fld3 = _29.fld0.0;
_29.fld4 = [_30,_30,_29.fld6,_29.fld6,_29.fld6,_29.fld6,_29.fld6,_29.fld6];
_6 = _32.fld4 as f64;
_31.1 = _20 as i8;
_32.fld5 = -_14;
_27.0 = _29.fld0.0;
_36 = _29.fld4;
_6 = _18 as f64;
_31.2 = 121316359486363555160095922800955502977_u128 as i128;
_25 = _14 * _6;
_26.1 = -_35.1;
_36 = [_30,_30,_30,_29.fld6,_29.fld6,_29.fld6,_30,_29.fld6];
_38 = 55344_u16 << _29.fld0.1;
_32.fld2 = _24;
_32.fld6.fld1 = core::ptr::addr_of!(_22);
Call(_32.fld2 = core::intrinsics::bswap(_10), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_29.fld6 = _30;
_32.fld7 = _29.fld0.2 * _27.2;
_41.0 = _29.fld0.1;
_35.0 = _41.0;
_41 = (_27.1, _18);
_41.1 = _18 | _18;
_33 = _41.1 ^ _41.1;
_7 = _4;
_41 = (_35.0, _33);
place!(Field::<(i64, [i128; 6], u16, [i128; 3])>(Variant(_19, 2), 0)).0 = 10979456963467010951_usize as i64;
_40 = [_32.fld2,_32.fld2,_32.fld2,_41.1,_41.1,_18,_41.1,_33];
Call(_32.fld6 = fn6(_26, _33, _33, _41, _41, _9, _41.0, _29.fld0.0, _40, _40, _25), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_29.fld0.0 = _35.1 as f32;
_39 = _41.1;
_29.fld5 = [_39,_39,_33,_39,_33,_41.1,_39,_41.1];
_13 = _4;
_5 = [_22,_22,_22,_22,_22,_22,_22,_22];
Call(_27.0 = fn8(Move(_32.fld6), _29.fld5, _26.0, _41.0, _7, _41, _33), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_20 = _21;
place!(Field::<(u64,)>(Variant(_19, 2), 1)).0 = 14212592910756487899_u64;
Goto(bb14)
}
bb14 = {
_41.0 = _31.1;
_18 = _22 as isize;
_12 = [_38,_38,_38,_38,_38,_38];
_34 = 1182292125_u32;
_19 = Adt47::Variant1 { fld0: _22,fld1: _29.fld5 };
_39 = _41.1 - _33;
_26 = (_27.1, _35.1);
_27.1 = _35.0;
_19 = Adt47::Variant0 { fld0: _29.fld0.2 };
_26 = _35;
_31.0 = -_27.0;
_38 = !22528_u16;
_37 = -_39;
_25 = _34 as f64;
_42 = 309322058400121204907588607702309900416_u128 | 86077962434975758233487126994248227505_u128;
Goto(bb15)
}
bb15 = {
Call(_46 = dump_var(5_usize, 33_usize, Move(_33), 7_usize, Move(_7), 38_usize, Move(_38), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_46 = dump_var(5_usize, 40_usize, Move(_40), 4_usize, Move(_4), 1_usize, Move(_1), 34_usize, Move(_34)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(5_usize, 24_usize, Move(_24), 41_usize, Move(_41), 26_usize, Move(_26), 22_usize, Move(_22)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_46 = dump_var(5_usize, 5_usize, Move(_5), 11_usize, Move(_11), 8_usize, Move(_8), 47_usize, _47), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: (i8, i32),mut _2: isize,mut _3: isize,mut _4: (i8, isize),mut _5: (i8, isize),mut _6: [isize; 8],mut _7: i8,mut _8: f32,mut _9: [isize; 8],mut _10: [isize; 8],mut _11: f64) -> Adt51 {
mir! {
type RET = Adt51;
let _12: [i128; 3];
let _13: Adt47;
let _14: (i8, i32);
let _15: isize;
let _16: Adt54;
let _17: u128;
let _18: isize;
let _19: Adt61;
let _20: Adt53;
let _21: i16;
let _22: i8;
let _23: ();
let _24: ();
{
_7 = _5.0;
RET.fld0 = 3517262794_u32 ^ 2811958889_u32;
RET.fld0 = !3385583044_u32;
_1.0 = _5.0;
_13 = Adt47::Variant1 { fld0: true,fld1: _10 };
_4 = (_5.0, _2);
_3 = _2 * _2;
Goto(bb1)
}
bb1 = {
_4 = (_7, _5.1);
_1.1 = (-965573423_i32);
_8 = 2008125460276396999_i64 as f32;
_5.0 = _1.0 - _1.0;
_10 = [_3,_3,_2,_5.1,_3,_3,_2,_3];
_12 = [(-62111407541512226861427817845737372128_i128),142093163421075250607137240815530202897_i128,120855846003908088315887279855078674391_i128];
_7 = -_5.0;
_1.1 = (-129022387_i32) & 1811428905_i32;
_4.1 = -_2;
_8 = 35623_u16 as f32;
_14.1 = _11 as i32;
_5 = (_4.0, _4.1);
_15 = _3;
RET.fld0 = 79123044_u32;
_6 = _10;
_13 = Adt47::Variant1 { fld0: true,fld1: _6 };
_4.0 = _1.0;
Call(_4.1 = fn7(_5.0, _10, _6, Field::<[isize; 8]>(Variant(_13, 1), 1), _15, _3, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5.1 = _4.1 ^ _3;
_5.1 = _15 ^ _4.1;
_14.0 = RET.fld0 as i8;
place!(Field::<[isize; 8]>(Variant(_13, 1), 1)) = [_5.1,_3,_3,_5.1,_5.1,_5.1,_2,_5.1];
RET.fld0 = 1016738906_u32 ^ 877985717_u32;
Goto(bb3)
}
bb3 = {
RET.fld0 = 1600701090_u32 - 3750417109_u32;
_5 = (_14.0, _3);
_15 = _14.1 as isize;
_11 = 293075211043435167891229200378295914420_u128 as f64;
_8 = (-97359050833647374601209067504747357274_i128) as f32;
RET.fld1 = core::ptr::addr_of!(place!(Field::<bool>(Variant(_13, 1), 0)));
Goto(bb4)
}
bb4 = {
Call(_23 = dump_var(6_usize, 3_usize, Move(_3), 15_usize, Move(_15), 5_usize, Move(_5), 12_usize, Move(_12)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_23 = dump_var(6_usize, 14_usize, Move(_14), 2_usize, Move(_2), 24_usize, _24, 24_usize, _24), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: i8,mut _2: [isize; 8],mut _3: [isize; 8],mut _4: [isize; 8],mut _5: isize,mut _6: isize,mut _7: [isize; 8]) -> isize {
mir! {
type RET = isize;
let _8: [i128; 6];
let _9: Adt56;
let _10: isize;
let _11: f64;
let _12: (usize, (*mut &'static i128, i64, (char, *mut &'static i128, char), (char, *mut &'static i128, char), u128, bool), i32, [u16; 6], [i128; 6]);
let _13: ();
let _14: ();
{
_7 = [_6,_6,_5,_6,_6,_5,_5,_5];
RET = !_5;
RET = !_6;
_6 = 18369_i16 as isize;
_7 = _2;
_7 = [RET,_5,RET,_5,RET,_5,RET,_5];
_4 = _2;
_3 = [RET,RET,_5,_5,RET,_5,RET,_5];
RET = !_5;
_6 = RET >> RET;
_8 = [(-109658352371542413433670950542322947952_i128),(-21690686970423727890799873797021211964_i128),74412432625221191051540821204596334692_i128,54131845197936494761352943710195728705_i128,(-89528551651626269553246627139804766571_i128),(-33324464363035402292734678061269411187_i128)];
_2 = [RET,_6,_6,_6,RET,_6,RET,_5];
_8 = [(-77112323901294507326306130128387811316_i128),(-60504336837371764657568936380257946142_i128),(-57027405267076366436113369384174299246_i128),26300931710904121011125549031367261874_i128,58326543655866877968029819093869211553_i128,3380527736525492541804828702079904693_i128];
_12.1.5 = !true;
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(7_usize, 7_usize, Move(_7), 2_usize, Move(_2), 5_usize, Move(_5), 8_usize, Move(_8)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: Adt51,mut _2: [isize; 8],mut _3: i8,mut _4: i8,mut _5: [bool; 8],mut _6: (i8, isize),mut _7: isize) -> f32 {
mir! {
type RET = f32;
let _8: [char; 2];
let _9: (f32, i8, i128);
let _10: f64;
let _11: f32;
let _12: char;
let _13: Adt60;
let _14: u16;
let _15: f64;
let _16: (i64, [i128; 6], u16, [i128; 3]);
let _17: u16;
let _18: *const *mut &'static i128;
let _19: char;
let _20: f32;
let _21: (u64,);
let _22: Adt57;
let _23: [i128; 6];
let _24: [bool; 8];
let _25: ();
let _26: ();
{
RET = 77685122769486793593071078562872814770_u128 as f32;
_1.fld0 = (-7009181678838505331_i64) as u32;
_7 = _6.1;
_9.2 = -(-115957798066875415853956988405060417461_i128);
_9.2 = !(-130259270900644331251019763284046606228_i128);
_9.1 = (-1302007654_i32) as i8;
_12 = '\u{a6203}';
_6.0 = -_4;
_6.1 = 74730042325706433395069811798738756781_u128 as isize;
RET = 172_u8 as f32;
RET = _9.1 as f32;
_13.fld7 = _3 as i128;
_9.1 = _4 * _4;
_13.fld3 = [_7,_7,_7,_7,_7,_7,_7,_7];
_13.fld2 = RET - RET;
_10 = _9.1 as f64;
RET = _13.fld2;
_6 = (_3, _7);
_2 = [_6.1,_6.1,_7,_6.1,_7,_6.1,_7,_6.1];
_13.fld0 = [_13.fld7,_13.fld7,_13.fld7,_13.fld7];
_9.0 = _13.fld2;
_8 = [_12,_12];
match _4 {
112 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_14 = 48273_u16;
_15 = _13.fld7 as f64;
_15 = _10;
_13.fld0 = [_13.fld7,_9.2,_13.fld7,_9.2];
_11 = _13.fld2;
_9.0 = RET;
_9.1 = _6.0 << _7;
_6.0 = _9.1;
_4 = _9.1;
_16.1 = [_13.fld7,_13.fld7,_9.2,_13.fld7,_9.2,_13.fld7];
_9.1 = _4 >> _4;
RET = 1191132540_i32 as f32;
_13.fld3 = [_6.1,_6.1,_6.1,_6.1,_6.1,_6.1,_7,_7];
_9.1 = -_4;
_3 = _10 as i8;
_6 = (_3, _7);
_6 = (_4, _7);
_16.2 = !_14;
_17 = !_16.2;
_13.fld6 = !13724712171581192699_u64;
_16.2 = _14;
_8 = [_12,_12];
_12 = '\u{a01c2}';
_11 = -_9.0;
_14 = !_16.2;
match _16.2 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
48273 => bb9,
_ => bb8
}
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
_16.0 = 8886636760513998093_i64 & (-29498765664222413_i64);
_3 = _6.0;
_20 = _6.1 as f32;
_13.fld0 = [_13.fld7,_13.fld7,_13.fld7,_13.fld7];
_10 = _15 + _15;
_13.fld2 = _16.0 as f32;
_16.0 = _1.fld0 as i64;
_9.0 = _20 - _20;
RET = _9.0;
_8 = [_12,_12];
_21 = (_13.fld6,);
_22 = Adt57::Variant0 { fld0: _2,fld1: _6 };
_15 = _10 - _10;
place!(Field::<(i8, isize)>(Variant(_22, 0), 1)).1 = _6.1 & _7;
_13.fld3 = [Field::<(i8, isize)>(Variant(_22, 0), 1).1,Field::<(i8, isize)>(Variant(_22, 0), 1).1,_7,_6.1,_6.1,_6.1,Field::<(i8, isize)>(Variant(_22, 0), 1).1,Field::<(i8, isize)>(Variant(_22, 0), 1).1];
_13.fld6 = _21.0 << _6.0;
_9.0 = _20;
Goto(bb10)
}
bb10 = {
Call(_25 = dump_var(8_usize, 7_usize, Move(_7), 21_usize, Move(_21), 3_usize, Move(_3), 2_usize, Move(_2)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_25 = dump_var(8_usize, 5_usize, Move(_5), 26_usize, _26, 26_usize, _26, 26_usize, _26), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: u32,mut _2: [bool; 8],mut _3: [u16; 6],mut _4: ([bool; 8],),mut _5: [bool; 8],mut _6: ([bool; 8],),mut _7: i64,mut _8: [isize; 8],mut _9: (char, *mut &'static i128, char),mut _10: char,mut _11: [isize; 8],mut _12: u128) -> char {
mir! {
type RET = char;
let _13: Adt45;
let _14: &'static i128;
let _15: [i128; 6];
let _16: u32;
let _17: Adt45;
let _18: [i128; 4];
let _19: [i64; 8];
let _20: *mut usize;
let _21: (u64,);
let _22: char;
let _23: Adt60;
let _24: (u64,);
let _25: (i64, [i128; 6], u16, [i128; 3]);
let _26: [isize; 8];
let _27: (u64,);
let _28: f64;
let _29: usize;
let _30: i64;
let _31: Adt54;
let _32: bool;
let _33: [isize; 8];
let _34: (*mut &'static i128, i64, (char, *mut &'static i128, char), (char, *mut &'static i128, char), u128, bool);
let _35: f64;
let _36: ();
let _37: ();
{
RET = _9.0;
RET = _10;
_6.0 = [true,true,false,false,false,true,false,true];
_5 = [false,true,false,true,true,true,false,false];
_15 = [167757512916868896571261881329006512710_i128,(-150812691707654985485124859249002305402_i128),144169287545371013049320780318131195893_i128,125931970697468264695110301243447737926_i128,(-103634227625407971230194939766233519868_i128),(-65953554290664152388053100390734851692_i128)];
_7 = 20_u8 as i64;
_3 = [30164_u16,56706_u16,109_u16,1490_u16,25551_u16,35125_u16];
_9.1 = core::ptr::addr_of_mut!(_14);
RET = _10;
RET = _10;
_16 = _1 ^ _1;
_5 = [false,true,false,false,false,false,false,true];
_10 = _9.2;
_4 = (_2,);
_15 = [122054088543254917461596633372639779224_i128,(-81924735868353189629613346403965216991_i128),69079702479032395971238397666599410594_i128,138474808721525484439836779692006735634_i128,(-163651062253997488273337660336456121378_i128),(-142206032331861892323341847926205155012_i128)];
_9.2 = RET;
_16 = _12 as u32;
RET = _10;
_4 = (_5,);
_15 = [(-77867341424371572726680663572448360042_i128),106213108649030233401582992870960524324_i128,106774532030480886567128261450669256735_i128,(-9671101517657670401539227331225524182_i128),96886920909439995175741045258053481475_i128,(-130166936271119119635553643541284030668_i128)];
_1 = !_16;
_2 = _4.0;
Goto(bb1)
}
bb1 = {
Goto(bb2)
}
bb2 = {
_4 = _6;
_4.0 = [false,true,false,false,false,true,true,true];
RET = _9.2;
_18 = [(-74032746961285942005702495202059090451_i128),104758299151422904681485614979283657056_i128,7526244835274950159276713843917208961_i128,80000481046394139197035893484517015206_i128];
_4 = (_2,);
_1 = !_16;
_16 = _1 + _1;
_9.0 = RET;
_18 = [(-157575203538173874841302034201670018791_i128),68693507289830583705100246364251285778_i128,(-89246173232296693022612078717157824095_i128),(-35296014873674589305993620619303780371_i128)];
_9.0 = _10;
_9.2 = _10;
_1 = 11270591516875215382_u64 as u32;
_6.0 = _2;
Goto(bb3)
}
bb3 = {
_9.0 = RET;
_2 = [false,false,true,true,true,true,true,false];
Goto(bb4)
}
bb4 = {
_1 = 9223372036854775807_isize as u32;
_2 = [false,true,true,false,false,false,false,true];
_15 = [155002172271919888931652843163627061322_i128,163658122300124533557735268982166364895_i128,142759037855671065610916321345687565_i128,(-158712122472042640202441719927926235295_i128),(-42465206523308775176305135164049336013_i128),(-79687950543615972170302460677128193804_i128)];
_9.0 = RET;
_9.2 = _10;
_9.0 = _9.2;
_2 = _6.0;
_9.2 = _10;
_18 = [(-8203534232055907243957401251505327981_i128),(-31187888284532245926610257510848358357_i128),119864368673778063047562056764328712387_i128,125673531711376001351999995934555617048_i128];
_5 = [false,false,false,false,false,true,false,true];
RET = _10;
_15 = [100501979419696903999920505442153355310_i128,(-10245856224200376525758199952013882338_i128),148785599957488305949242108135400430305_i128,(-37281391120370898435472397010659833245_i128),(-42017423596972297676281337796457535215_i128),134234483895491376668684043756533391153_i128];
_1 = _16 + _16;
_15 = [44701584920757510512010094936784141799_i128,12200669975466105508802038853625902084_i128,(-169560879037718956209448992105828423227_i128),13129627759186624354647148098961222148_i128,35865424918492181273183874893646143279_i128,87086548204046202318522529673251289409_i128];
Goto(bb5)
}
bb5 = {
_19 = [_7,_7,_7,_7,_7,_7,_7,_7];
Goto(bb6)
}
bb6 = {
_4 = (_6.0,);
_7 = (-3769164972521101419_i64);
_12 = 268105740963910814721263467490876904283_u128;
_18 = [(-158785253611860302384348895348935636322_i128),6549302693886066145693400504530812402_i128,(-57613069947121781880132548505584200104_i128),(-63649097794008779936510610739760260300_i128)];
_5 = [false,false,false,false,false,true,false,true];
_21 = (5479335155331432450_u64,);
_19 = [_7,_7,_7,_7,_7,_7,_7,_7];
_18 = [166859670056972002183651541343018692121_i128,(-11262827700340265961935301176396890759_i128),66579162530153486395044306923981940952_i128,(-65988767225047874433016096069756329846_i128)];
_6 = (_2,);
RET = _9.0;
_15 = [146363614433488928386319159338989944469_i128,74692542828809050663551007768587747646_i128,(-166630503546119068863024621931430527123_i128),89127289012901247069390186003207254341_i128,(-58807132521453132068016340012345746183_i128),1021277832551478262891470009291604619_i128];
_18 = [(-137731719260653081537067247534931947111_i128),18945031562303217930074083462965751892_i128,(-70363947293980280117877646498849766750_i128),(-112633437793826898811471518572213044625_i128)];
_12 = 90143423148712735538038820314224650665_u128 + 21426106717313817700322507897948530068_u128;
_10 = RET;
_1 = !_16;
_12 = 202361384334271966992475093817023191912_u128 >> _1;
Goto(bb7)
}
bb7 = {
_14 = &_23.fld7;
_4 = (_2,);
_6.0 = _4.0;
_23.fld2 = 14_i8 as f32;
_23.fld1 = Adt52::Variant1 { fld0: _19,fld1: _1 };
_4 = (_6.0,);
_15 = [(-22513578943481696880300548551743178904_i128),(-6401451524207949595033144342235254752_i128),(-130544272010211531035561001762002783546_i128),14452991630487789557876617901752096060_i128,(-1374591914681094504256862242096722117_i128),(-93314916944656769424287421125573259301_i128)];
_9.0 = _10;
_9.0 = _9.2;
_15 = [11099312312689197934102709232642891337_i128,46079400430826797243674670940487378831_i128,(-92650612228575458188056161491775446142_i128),83244726017360650650623011790758302732_i128,(-121225325750044785187599163590853061541_i128),111120459396124551403654614181188775152_i128];
_23.fld7 = (-107079814300721262027656216992790471535_i128) << _1;
_11 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-25_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
match _21.0 {
0 => bb3,
1 => bb8,
2 => bb9,
5479335155331432450 => bb11,
_ => bb10
}
}
bb8 = {
_4 = (_6.0,);
_7 = (-3769164972521101419_i64);
_12 = 268105740963910814721263467490876904283_u128;
_18 = [(-158785253611860302384348895348935636322_i128),6549302693886066145693400504530812402_i128,(-57613069947121781880132548505584200104_i128),(-63649097794008779936510610739760260300_i128)];
_5 = [false,false,false,false,false,true,false,true];
_21 = (5479335155331432450_u64,);
_19 = [_7,_7,_7,_7,_7,_7,_7,_7];
_18 = [166859670056972002183651541343018692121_i128,(-11262827700340265961935301176396890759_i128),66579162530153486395044306923981940952_i128,(-65988767225047874433016096069756329846_i128)];
_6 = (_2,);
RET = _9.0;
_15 = [146363614433488928386319159338989944469_i128,74692542828809050663551007768587747646_i128,(-166630503546119068863024621931430527123_i128),89127289012901247069390186003207254341_i128,(-58807132521453132068016340012345746183_i128),1021277832551478262891470009291604619_i128];
_18 = [(-137731719260653081537067247534931947111_i128),18945031562303217930074083462965751892_i128,(-70363947293980280117877646498849766750_i128),(-112633437793826898811471518572213044625_i128)];
_12 = 90143423148712735538038820314224650665_u128 + 21426106717313817700322507897948530068_u128;
_10 = RET;
_1 = !_16;
_12 = 202361384334271966992475093817023191912_u128 >> _1;
Goto(bb7)
}
bb9 = {
_9.0 = RET;
_2 = [false,false,true,true,true,true,true,false];
Goto(bb4)
}
bb10 = {
_1 = 9223372036854775807_isize as u32;
_2 = [false,true,true,false,false,false,false,true];
_15 = [155002172271919888931652843163627061322_i128,163658122300124533557735268982166364895_i128,142759037855671065610916321345687565_i128,(-158712122472042640202441719927926235295_i128),(-42465206523308775176305135164049336013_i128),(-79687950543615972170302460677128193804_i128)];
_9.0 = RET;
_9.2 = _10;
_9.0 = _9.2;
_2 = _6.0;
_9.2 = _10;
_18 = [(-8203534232055907243957401251505327981_i128),(-31187888284532245926610257510848358357_i128),119864368673778063047562056764328712387_i128,125673531711376001351999995934555617048_i128];
_5 = [false,false,false,false,false,true,false,true];
RET = _10;
_15 = [100501979419696903999920505442153355310_i128,(-10245856224200376525758199952013882338_i128),148785599957488305949242108135400430305_i128,(-37281391120370898435472397010659833245_i128),(-42017423596972297676281337796457535215_i128),134234483895491376668684043756533391153_i128];
_1 = _16 + _16;
_15 = [44701584920757510512010094936784141799_i128,12200669975466105508802038853625902084_i128,(-169560879037718956209448992105828423227_i128),13129627759186624354647148098961222148_i128,35865424918492181273183874893646143279_i128,87086548204046202318522529673251289409_i128];
Goto(bb5)
}
bb11 = {
_8 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-101_isize),98_isize,126_isize,102_isize];
Call(_12 = fn10(_6, _16, _4, _23.fld7, _7, _23.fld7, _18, _15, Move(_23.fld1), _8, _16), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_23.fld0 = [_23.fld7,_23.fld7,_23.fld7,_23.fld7];
_12 = !267321874108228597098640250694933470124_u128;
_24 = (_21.0,);
_6.0 = [false,false,false,false,true,false,true,true];
_16 = !_1;
_18 = [_23.fld7,_23.fld7,_23.fld7,_23.fld7];
_22 = _9.2;
_21 = (_24.0,);
_16 = _1 - _1;
_25.1 = [_23.fld7,_23.fld7,_23.fld7,_23.fld7,_23.fld7,_23.fld7];
_4.0 = [true,false,false,true,true,false,false,true];
_21.0 = (-23_isize) as u64;
_4 = _6;
_25.2 = _12 as u16;
_27 = (_21.0,);
_5 = [false,false,true,false,true,true,true,true];
_5 = [true,true,true,true,false,false,false,false];
_25.3 = [_23.fld7,_23.fld7,_23.fld7];
RET = _10;
_25.2 = 27027_u16;
_26 = _8;
_24.0 = _21.0 << _16;
_14 = &_23.fld7;
_9.0 = _10;
_27 = _24;
Goto(bb13)
}
bb13 = {
_4 = (_5,);
_21 = (_24.0,);
_3 = [_25.2,_25.2,_25.2,_25.2,_25.2,_25.2];
_22 = _9.2;
_10 = _9.0;
Goto(bb14)
}
bb14 = {
_6 = (_2,);
_23.fld1 = Adt52::Variant1 { fld0: _19,fld1: _1 };
SetDiscriminant(_23.fld1, 1);
_23.fld2 = _7 as f32;
_5 = _4.0;
_25.0 = (-9223372036854775808_isize) as i64;
_26 = [(-9223372036854775808_isize),(-90_isize),(-99_isize),(-9223372036854775808_isize),4_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_23.fld5 = core::ptr::addr_of_mut!(_29);
_5 = _6.0;
_3 = [_25.2,_25.2,_25.2,_25.2,_25.2,_25.2];
_32 = !true;
_9.0 = _9.2;
_6 = (_4.0,);
_29 = 4_usize & 7432970364057752049_usize;
_6.0 = [_32,_32,_32,_32,_32,_32,_32,_32];
_25.1 = [_23.fld7,(*_14),_23.fld7,(*_14),(*_14),_23.fld7];
_9.0 = RET;
_32 = true;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(9_usize, 22_usize, Move(_22), 6_usize, Move(_6), 29_usize, Move(_29), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(9_usize, 3_usize, Move(_3), 15_usize, Move(_15), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(9_usize, 16_usize, Move(_16), 18_usize, Move(_18), 24_usize, Move(_24), 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: ([bool; 8],),mut _2: u32,mut _3: ([bool; 8],),mut _4: i128,mut _5: i64,mut _6: i128,mut _7: [i128; 4],mut _8: [i128; 6],mut _9: Adt52,mut _10: [isize; 8],mut _11: u32) -> u128 {
mir! {
type RET = u128;
let _12: [i128; 3];
let _13: [bool; 8];
let _14: bool;
let _15: (u64,);
let _16: *const u32;
let _17: (u64,);
let _18: i64;
let _19: u64;
let _20: isize;
let _21: *const *mut &'static i128;
let _22: (i64, [i128; 6], u16, [i128; 3]);
let _23: ();
let _24: ();
{
_5 = (-392566901853522348_i64) + (-995875860959877400_i64);
_6 = _4 | _4;
_7 = [_4,_6,_4,_4];
_5 = -6657650123668126825_i64;
RET = 258215124799306737157106586024051370932_u128;
_1 = _3;
_13 = _1.0;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
258215124799306737157106586024051370932 => bb9,
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
RET = !98707750105927463530419964671564228287_u128;
_7 = [_4,_6,_4,_4];
_3 = _1;
_2 = Field::<u32>(Variant(_9, 1), 1);
_15 = (11110410816062833193_u64,);
_12 = [_6,_6,_4];
_15 = (11634535572613112363_u64,);
_14 = _2 > Field::<u32>(Variant(_9, 1), 1);
_1 = (_13,);
_12 = [_6,_4,_6];
_16 = core::ptr::addr_of!(place!(Field::<u32>(Variant(_9, 1), 1)));
Goto(bb10)
}
bb10 = {
_5 = (-6198912750392835042_i64) + 6413438167150034621_i64;
SetDiscriminant(_9, 0);
match _15.0 {
0 => bb1,
1 => bb8,
2 => bb9,
11634535572613112363 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_5 = (-7379754884998045439_i64);
_10 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-45_isize),(-9223372036854775808_isize),(-58_isize),44_isize,(-9223372036854775808_isize)];
_11 = _2;
_10 = [(-26_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
place!(Field::<usize>(Variant(_9, 0), 3)) = !4_usize;
place!(Field::<u16>(Variant(_9, 0), 2)) = !18087_u16;
_11 = _2;
_13 = [_14,_14,_14,_14,_14,_14,_14,_14];
_17.0 = _15.0 << _11;
_5 = 3070390991110861733_i64;
_15 = (_17.0,);
place!(Field::<i32>(Variant(_9, 0), 0)) = 962505658_i32 * (-1881800611_i32);
RET = 9223372036854775807_isize as u128;
Call(_2 = core::intrinsics::bswap(_11), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_18 = _5 * _5;
RET = 67747748543442798233434203698288710215_u128 * 155269495161092429969568027721318709075_u128;
_7 = [_4,_6,_6,_6];
_6 = _4 ^ _4;
place!(Field::<*mut i32>(Variant(_9, 0), 4)) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_9, 0), 0)));
place!(Field::<usize>(Variant(_9, 0), 3)) = 10262033006470946497_usize - 16398974051619308445_usize;
_10 = [9223372036854775807_isize,(-96_isize),(-9223372036854775808_isize),35_isize,9223372036854775807_isize,9223372036854775807_isize,91_isize,(-50_isize)];
_3 = _1;
_15.0 = _17.0 + _17.0;
_16 = core::ptr::addr_of!(_2);
place!(Field::<char>(Variant(_9, 0), 1)) = '\u{d2e0f}';
_1 = (_13,);
SetDiscriminant(_9, 1);
match _5 {
0 => bb14,
3070390991110861733 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_15 = (_17.0,);
_16 = core::ptr::addr_of!(_2);
(*_16) = _11;
_15.0 = !_17.0;
_6 = 5924263157191835027_usize as i128;
_7 = [_4,_4,_4,_4];
place!(Field::<[i64; 8]>(Variant(_9, 1), 0)) = [_18,_18,_18,_5,_5,_18,_18,_18];
_17 = _15;
_3.0 = _13;
_3.0 = [_14,_14,_14,_14,_14,_14,_14,_14];
RET = 16863295170094471651471783527643093224_u128;
_4 = _15.0 as i128;
place!(Field::<u32>(Variant(_9, 1), 1)) = !(*_16);
_13 = _3.0;
_22.2 = 19633_u16 << Field::<u32>(Variant(_9, 1), 1);
RET = 333364607112477874516174257183088641719_u128;
_22.1 = _8;
_22.1 = [_4,_4,_4,_4,_4,_4];
_22.0 = !_18;
_12 = [_4,_4,_4];
_15 = _17;
_17 = _15;
Goto(bb17)
}
bb17 = {
Call(_23 = dump_var(10_usize, 2_usize, Move(_2), 12_usize, Move(_12), 5_usize, Move(_5), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_23 = dump_var(10_usize, 10_usize, Move(_10), 17_usize, Move(_17), 7_usize, Move(_7), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: usize,mut _2: i64,mut _3: [u16; 6],mut _4: ([bool; 8],),mut _5: char,mut _6: [u16; 6],mut _7: [u16; 6],mut _8: i64,mut _9: u128,mut _10: i8) -> bool {
mir! {
type RET = bool;
let _11: i32;
let _12: u8;
let _13: char;
let _14: ();
let _15: ();
{
_3 = [53455_u16,11924_u16,53749_u16,34484_u16,16503_u16,25415_u16];
_4.0 = [true,true,true,false,false,false,true,true];
_4.0 = [false,true,true,false,false,false,true,true];
_10 = !39_i8;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
1655263327558537876 => bb9,
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
RET = false;
_10 = (-151625214_i32) as i8;
_6 = [46415_u16,23516_u16,63054_u16,37122_u16,36214_u16,12067_u16];
RET = !false;
_10 = _2 as i8;
_12 = _9 as u8;
RET = true;
_13 = _5;
_13 = _5;
_9 = 13891373305494627619_u64 as u128;
_9 = !194671256127595280783278714724141420399_u128;
_2 = _8;
_6 = [2921_u16,19263_u16,8837_u16,23873_u16,59101_u16,19640_u16];
_11 = _2 as i32;
RET = !false;
_7 = [51917_u16,43580_u16,54061_u16,30201_u16,4245_u16,20166_u16];
_9 = 1410336317_u32 as u128;
_11 = 1894177788_i32 ^ 2109601715_i32;
_1 = 7_usize;
RET = _4.0[_1] != _4.0[_1];
_12 = 214_u8 | 210_u8;
_1 = 48094544018945390629848842774344837085_i128 as usize;
_12 = 46_u8 | 43_u8;
_3 = [62658_u16,8621_u16,40163_u16,54998_u16,41886_u16,16694_u16];
_12 = !80_u8;
Goto(bb10)
}
bb10 = {
Call(_14 = dump_var(11_usize, 13_usize, Move(_13), 1_usize, Move(_1), 5_usize, Move(_5), 8_usize, Move(_8)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_14 = dump_var(11_usize, 3_usize, Move(_3), 10_usize, Move(_10), 15_usize, _15, 15_usize, _15), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: i16,mut _2: i16,mut _3: i16,mut _4: i16,mut _5: i64,mut _6: [bool; 8],mut _7: i16,mut _8: [bool; 8],mut _9: i64,mut _10: u128) -> [i128; 6] {
mir! {
type RET = [i128; 6];
let _11: (char, *mut &'static i128, char);
let _12: (f32, i8, i128);
let _13: u8;
let _14: char;
let _15: f64;
let _16: f32;
let _17: Adt59;
let _18: isize;
let _19: Adt47;
let _20: Adt56;
let _21: (char, *mut &'static i128, char);
let _22: f64;
let _23: [i128; 3];
let _24: f32;
let _25: u32;
let _26: ([bool; 8],);
let _27: ();
let _28: ();
{
_1 = _2;
_8 = [true,false,false,false,true,true,true,false];
_6 = [true,true,false,true,true,true,false,true];
RET = [4115867373116958613925252052483572183_i128,13217337261219234180866993090526621394_i128,111899241527162748552274289540548619694_i128,145956670865181801348980373558406629587_i128,2476682041235249440236570566611587878_i128,(-29782131848144777205154592974979089873_i128)];
_10 = !122999665097645551083312929049883034915_u128;
_8 = _6;
_11.2 = '\u{f61c9}';
_11.2 = '\u{317b}';
_4 = _1;
_13 = 236_u8;
_12.2 = !(-27532727569646475351953150124158334554_i128);
_14 = _11.2;
_11.0 = _11.2;
_5 = _9;
_8 = [false,true,true,false,true,true,false,true];
_2 = !_1;
_1 = _7;
_12.1 = 36_i8;
_7 = _2;
_12.0 = 3785683952_u32 as f32;
_12.2 = (-94374566321975948848583929531854148468_i128);
match _13 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
236 => bb9,
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
RET = [_12.2,_12.2,_12.2,_12.2,_12.2,_12.2];
_4 = _7 & _3;
_12.0 = _5 as f32;
_7 = 10109332284049748195_usize as i16;
_14 = _11.2;
_12.1 = _13 as i8;
_8 = _6;
RET = [_12.2,_12.2,_12.2,_12.2,_12.2,_12.2];
_15 = _10 as f64;
_13 = _4 as u8;
_9 = _5;
Call(_11 = fn13(_8, _13, _3, _1, _12.1, _13, _13), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_10 = _2 as u128;
Goto(bb11)
}
bb11 = {
_13 = !208_u8;
_13 = 239_u8 ^ 228_u8;
_4 = !_1;
_9 = _5 - _5;
_16 = _12.0;
_19 = Adt47::Variant0 { fld0: _12.2 };
_18 = !(-9223372036854775808_isize);
_2 = !_4;
_18 = 32858_u16 as isize;
RET = [_12.2,_12.2,_12.2,Field::<i128>(Variant(_19, 0), 0),Field::<i128>(Variant(_19, 0), 0),Field::<i128>(Variant(_19, 0), 0)];
_19 = Adt47::Variant0 { fld0: _12.2 };
_14 = _11.0;
RET = [Field::<i128>(Variant(_19, 0), 0),_12.2,_12.2,Field::<i128>(Variant(_19, 0), 0),Field::<i128>(Variant(_19, 0), 0),Field::<i128>(Variant(_19, 0), 0)];
_7 = _1 + _2;
_11.0 = _14;
_7 = _3 ^ _2;
RET = [Field::<i128>(Variant(_19, 0), 0),Field::<i128>(Variant(_19, 0), 0),Field::<i128>(Variant(_19, 0), 0),_12.2,Field::<i128>(Variant(_19, 0), 0),Field::<i128>(Variant(_19, 0), 0)];
_21 = Move(_11);
_5 = _9;
_3 = _7;
RET = [Field::<i128>(Variant(_19, 0), 0),Field::<i128>(Variant(_19, 0), 0),_12.2,Field::<i128>(Variant(_19, 0), 0),Field::<i128>(Variant(_19, 0), 0),Field::<i128>(Variant(_19, 0), 0)];
_3 = _1 * _4;
_11.2 = _21.2;
_11 = Move(_21);
_12.1 = -(-15_i8);
_5 = _9 >> _3;
_7 = _3;
_12.0 = (-316203814_i32) as f32;
_21.0 = _11.0;
_11.0 = _21.0;
match _12.2 {
0 => bb7,
1 => bb3,
2 => bb12,
3 => bb13,
4 => bb14,
245907800598962514614790677899914062988 => bb16,
_ => bb15
}
}
bb12 = {
_10 = _2 as u128;
Goto(bb11)
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
_1 = _4 + _3;
_6 = [true,false,false,true,false,false,false,false];
_4 = _10 as i16;
_7 = _14 as i16;
_6 = [false,false,false,false,false,false,false,true];
_5 = _9;
_11.0 = _11.2;
SetDiscriminant(_19, 2);
_15 = 75142523_i32 as f64;
place!(Field::<(u64,)>(Variant(_19, 2), 1)).0 = 17704670278211066248_u64;
_20 = Adt56::Variant0 { fld0: Field::<(u64,)>(Variant(_19, 2), 1).0,fld1: _21.0 };
_11.2 = _14;
SetDiscriminant(_20, 0);
place!(Field::<char>(Variant(_20, 0), 1)) = _11.2;
_21.2 = Field::<char>(Variant(_20, 0), 1);
_11.0 = _11.2;
place!(Field::<u64>(Variant(_20, 0), 0)) = Field::<(u64,)>(Variant(_19, 2), 1).0;
SetDiscriminant(_20, 1);
_15 = _9 as f64;
place!(Field::<(i64, [i128; 6], u16, [i128; 3])>(Variant(_19, 2), 0)).1 = RET;
_12 = (_16, (-83_i8), 58720416147091927537618163126354309156_i128);
_13 = 158_u8;
_19 = Adt47::Variant0 { fld0: _12.2 };
_12.0 = -_16;
_11.2 = _14;
Goto(bb17)
}
bb17 = {
Call(_27 = dump_var(12_usize, 8_usize, Move(_8), 6_usize, Move(_6), 9_usize, Move(_9), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_27 = dump_var(12_usize, 5_usize, Move(_5), 13_usize, Move(_13), 28_usize, _28, 28_usize, _28), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [bool; 8],mut _2: u8,mut _3: i16,mut _4: i16,mut _5: i8,mut _6: u8,mut _7: u8) -> (char, *mut &'static i128, char) {
mir! {
type RET = (char, *mut &'static i128, char);
let _8: char;
let _9: i64;
let _10: i16;
let _11: u32;
let _12: &'static i128;
let _13: [u64; 8];
let _14: [i64; 8];
let _15: char;
let _16: [bool; 8];
let _17: Adt50;
let _18: i32;
let _19: Adt55;
let _20: char;
let _21: *mut (usize, (*mut &'static i128, i64, (char, *mut &'static i128, char), (char, *mut &'static i128, char), u128, bool), i32, [u16; 6], [i128; 6]);
let _22: (i64, [i128; 6], u16, [i128; 3]);
let _23: bool;
let _24: [char; 2];
let _25: f32;
let _26: [char; 2];
let _27: [i128; 4];
let _28: (char, *mut &'static i128, char);
let _29: ();
let _30: ();
{
RET.0 = '\u{dd52d}';
_5 = (-20_i8) & (-122_i8);
_6 = _7;
RET.2 = RET.0;
_1 = [false,true,true,false,false,true,true,false];
RET.2 = RET.0;
RET.2 = RET.0;
_2 = _7 + _7;
_7 = !_2;
RET.2 = RET.0;
RET.2 = RET.0;
_7 = _6 ^ _2;
_2 = _7 << _7;
_8 = RET.2;
_9 = -(-7529410421335367688_i64);
_11 = !2421052859_u32;
_7 = _6 | _2;
RET.1 = core::ptr::addr_of_mut!(_12);
_11 = 997717450_u32 * 13106527_u32;
RET.0 = RET.2;
RET.0 = RET.2;
_3 = _4 + _4;
Call(_4 = core::intrinsics::bswap(_3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = !_3;
_9 = 8470513870242131834_i64 + 1206638185359271594_i64;
_8 = RET.0;
RET.2 = _8;
_1 = [true,false,false,true,false,true,true,true];
_3 = 55159_u16 as i16;
_8 = RET.2;
_14 = [_9,_9,_9,_9,_9,_9,_9,_9];
_14 = [_9,_9,_9,_9,_9,_9,_9,_9];
_11 = 3235790776_u32;
RET.2 = _8;
_13 = [11469287878713364003_u64,11740496896844054063_u64,6586380957578600385_u64,12198408640155090042_u64,6708687610332576520_u64,10168052619483680124_u64,7590458393553182417_u64,14050863385800137753_u64];
Goto(bb2)
}
bb2 = {
_2 = !_7;
_7 = _6;
_9 = 2314163076918378428_i64;
RET.0 = _8;
_3 = _10 ^ _10;
_2 = (-103_isize) as u8;
_16 = [false,false,false,false,false,true,false,true];
RET.1 = core::ptr::addr_of_mut!(_12);
_1 = [true,true,false,false,true,false,false,true];
_9 = 6967432594852920658_i64 | 8156109865026132703_i64;
RET.0 = _8;
_4 = !_10;
RET.1 = core::ptr::addr_of_mut!(_12);
RET.1 = core::ptr::addr_of_mut!(_12);
_15 = RET.2;
_18 = 896981220_i32;
_10 = _3;
RET.0 = _8;
_7 = !_6;
_13 = [16149223925714098662_u64,4966943743012415452_u64,16183437716107596112_u64,14855208249696180103_u64,16427566738260511261_u64,12234825353057389380_u64,5783532701535038489_u64,6117054427347213184_u64];
_15 = RET.0;
_16 = _1;
_11 = 2609424688_u32 + 227185308_u32;
_11 = (-9223372036854775808_isize) as u32;
_13 = [9563810304832590649_u64,433580624447665563_u64,4739187167574737629_u64,14028807135677828671_u64,6695525899889066176_u64,15439868110958935051_u64,17148266378313479054_u64,2439315168355782267_u64];
_10 = _4;
_15 = RET.0;
match _18 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
896981220 => bb10,
_ => bb9
}
}
bb3 = {
_10 = !_3;
_9 = 8470513870242131834_i64 + 1206638185359271594_i64;
_8 = RET.0;
RET.2 = _8;
_1 = [true,false,false,true,false,true,true,true];
_3 = 55159_u16 as i16;
_8 = RET.2;
_14 = [_9,_9,_9,_9,_9,_9,_9,_9];
_14 = [_9,_9,_9,_9,_9,_9,_9,_9];
_11 = 3235790776_u32;
RET.2 = _8;
_13 = [11469287878713364003_u64,11740496896844054063_u64,6586380957578600385_u64,12198408640155090042_u64,6708687610332576520_u64,10168052619483680124_u64,7590458393553182417_u64,14050863385800137753_u64];
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
_20 = RET.2;
RET.2 = RET.0;
_15 = RET.0;
_4 = _10;
RET.2 = RET.0;
_11 = false as u32;
RET.0 = _15;
_5 = !106_i8;
_16 = [true,true,true,false,true,false,true,false];
_23 = true;
_6 = RET.2 as u8;
RET.1 = core::ptr::addr_of_mut!(_12);
RET.1 = core::ptr::addr_of_mut!(_12);
_7 = 37559_u16 as u8;
_10 = _3;
match _18 {
896981220 => bb12,
_ => bb11
}
}
bb11 = {
_10 = !_3;
_9 = 8470513870242131834_i64 + 1206638185359271594_i64;
_8 = RET.0;
RET.2 = _8;
_1 = [true,false,false,true,false,true,true,true];
_3 = 55159_u16 as i16;
_8 = RET.2;
_14 = [_9,_9,_9,_9,_9,_9,_9,_9];
_14 = [_9,_9,_9,_9,_9,_9,_9,_9];
_11 = 3235790776_u32;
RET.2 = _8;
_13 = [11469287878713364003_u64,11740496896844054063_u64,6586380957578600385_u64,12198408640155090042_u64,6708687610332576520_u64,10168052619483680124_u64,7590458393553182417_u64,14050863385800137753_u64];
Goto(bb2)
}
bb12 = {
_4 = _3;
_14 = [_9,_9,_9,_9,_9,_9,_9,_9];
_8 = _20;
_13 = [16459533086852635216_u64,16031011502252320906_u64,6900980882217659702_u64,5692057869377162912_u64,15820350701705766368_u64,5652292377027775973_u64,17821195055799636780_u64,15113632571688515935_u64];
_24 = [_15,_15];
_26 = _24;
_22.0 = -_9;
_22.1 = [(-44242535907039335202639222661104590434_i128),(-113580712808085153916896538110947618172_i128),150950613358820410225064197031941993069_i128,46834308424821901116749063764152745739_i128,30211476889771076215494957659443407392_i128,(-47759801378349177112052811265639759153_i128)];
match _18 {
0 => bb2,
1 => bb13,
896981220 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_9 = _22.0 + _22.0;
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(13_usize, 9_usize, Move(_9), 15_usize, Move(_15), 16_usize, Move(_16), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(13_usize, 11_usize, Move(_11), 10_usize, Move(_10), 4_usize, Move(_4), 24_usize, Move(_24)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(13_usize, 1_usize, Move(_1), 3_usize, Move(_3), 30_usize, _30, 30_usize, _30), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(61923319586377512447214935228972511174_i128), std::hint::black_box(247_u8), std::hint::black_box(7_usize), std::hint::black_box(62564646485200695748687733941197542446_u128), std::hint::black_box((-30409_i16)), std::hint::black_box(1606128716_i32), std::hint::black_box(7590037201815825185_i64));
                
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: u64,
fld1: u8,
fld2: [i128; 6],
fld3: i8,
fld4: [i128; 4],
fld5: (i8, isize),
fld6: f64,
fld7: u16,

},
Variant1{
fld0: bool,
fld1: [i128; 6],
fld2: (i8, i32),
fld3: f64,
fld4: [isize; 8],

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: bool,
fld1: i32,
fld2: [i64; 8],
fld3: i128,
fld4: ([bool; 8],),
}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: i128,

},
Variant1{
fld0: bool,
fld1: [isize; 8],

},
Variant2{
fld0: (i64, [i128; 6], u16, [i128; 3]),
fld1: (u64,),
fld2: [char; 2],
fld3: [i64; 8],
fld4: *const u32,

}}
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: u32,
fld1: i16,
fld2: [isize; 8],

},
Variant1{
fld0: Adt46,
fld1: u8,
fld2: *const bool,
fld3: u64,

},
Variant2{
fld0: *mut usize,
fld1: i64,
fld2: u128,
fld3: ([bool; 8],),

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: (f32, i8, i128),
fld1: char,
fld2: *const bool,
fld3: (u64,),
fld4: [i64; 8],
fld5: [isize; 8],
fld6: i64,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: (i8, i32),

},
Variant1{
fld0: usize,
fld1: (i8, isize),
fld2: Adt46,
fld3: *mut usize,
fld4: *mut i32,
fld5: *const bool,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: u32,
fld1: *const bool,
}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: i32,
fld1: char,
fld2: u16,
fld3: usize,
fld4: *mut i32,

},
Variant1{
fld0: [i64; 8],
fld1: u32,

},
Variant2{
fld0: Adt46,
fld1: ([bool; 8],),

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: [u16; 6],
fld1: [i128; 6],
fld2: isize,
fld3: f32,
fld4: i16,
fld5: f64,
fld6: Adt51,
fld7: i128,
}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt52,
fld1: [u16; 6],
fld2: isize,
fld3: [u64; 8],

},
Variant1{
fld0: u8,
fld1: usize,
fld2: u32,
fld3: i8,

},
Variant2{
fld0: i64,
fld1: [u16; 6],
fld2: *const bool,
fld3: f32,

},
Variant3{
fld0: [i128; 3],
fld1: Adt45,
fld2: Adt51,
fld3: ([bool; 8],),
fld4: i16,
fld5: Adt53,
fld6: *mut i32,
fld7: (i8, isize),

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: [i128; 4],
fld1: *mut usize,
fld2: [bool; 8],
fld3: [char; 2],
fld4: Adt47,
fld5: ([bool; 8],),

},
Variant1{
fld0: [i128; 3],
fld1: (i64, [i128; 6], u16, [i128; 3]),
fld2: Adt45,
fld3: i8,
fld4: Adt48,

},
Variant2{
fld0: bool,
fld1: u16,
fld2: isize,
fld3: *const char,

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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: u64,
fld1: char,

},
Variant1{
fld0: Adt52,
fld1: i8,
fld2: Adt50,

},
Variant2{
fld0: [i64; 8],
fld1: *mut i32,
fld2: isize,
fld3: [char; 2],
fld4: f64,
fld5: *mut usize,
fld6: usize,

},
Variant3{
fld0: bool,
fld1: Adt47,
fld2: [i128; 4],
fld3: *mut i32,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
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
fld0: [isize; 8],
fld1: (i8, isize),

},
Variant1{
fld0: Adt52,
fld1: Adt46,
fld2: [isize; 8],
fld3: i128,
fld4: [i128; 4],
fld5: [bool; 8],
fld6: i64,

},
Variant2{
fld0: bool,
fld1: u32,
fld2: f64,
fld3: *const char,
fld4: usize,
fld5: [i128; 3],
fld6: Adt48,

},
Variant3{
fld0: i32,
fld1: char,
fld2: ([bool; 8],),
fld3: (i64, [i128; 6], u16, [i128; 3]),

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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: *const char,
fld1: (f32, i8, i128),
fld2: isize,
fld3: [u16; 6],
fld4: i128,
fld5: [i128; 4],
fld6: [i128; 3],

},
Variant1{
fld0: *mut i32,
fld1: Adt47,
fld2: u8,

},
Variant2{
fld0: *const u32,
fld1: (i64, [i128; 6], u16, [i128; 3]),
fld2: Adt48,
fld3: u32,
fld4: i16,
fld5: [i128; 4],
fld6: Adt54,
fld7: [i128; 3],

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt59::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: *mut i32,

},
Variant1{
fld0: u16,
fld1: char,
fld2: isize,
fld3: Adt46,
fld4: (i64, [i128; 6], u16, [i128; 3]),
fld5: i32,
fld6: u8,
fld7: [bool; 8],

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt60{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt60 {
fld0: [i128; 4],
fld1: Adt52,
fld2: f32,
fld3: [isize; 8],
fld4: Adt50,
fld5: *mut usize,
fld6: u64,
fld7: i128,
}
impl PrintFDebug for Adt61{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt61::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt61 {
Variant0{
fld0: i64,
fld1: u128,
fld2: Adt56,
fld3: (i64, [i128; 6], u16, [i128; 3]),
fld4: Adt46,

},
Variant1{
fld0: [u64; 8],
fld1: i32,
fld2: Adt60,
fld3: f64,
fld4: Adt55,

}}

