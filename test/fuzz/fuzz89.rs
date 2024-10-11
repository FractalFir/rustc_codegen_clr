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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: u16,mut _4: u32,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u128) -> f64 {
mir! {
type RET = f64;
let _11: ([u32; 4], char, u64);
let _12: char;
let _13: isize;
let _14: [i32; 4];
let _15: [u32; 4];
let _16: u64;
let _17: Adt51;
let _18: isize;
let _19: Adt54;
let _20: i16;
let _21: i32;
let _22: Adt53;
let _23: f32;
let _24: *mut (i32, [i32; 4], i64, u128);
let _25: ([i8; 3],);
let _26: Adt50;
let _27: bool;
let _28: isize;
let _29: u8;
let _30: bool;
let _31: i128;
let _32: (u32, u32, *const isize);
let _33: ();
let _34: ();
{
RET = 1402921480838474593_u64 as f64;
_8 = (-76171520216259033290281262806930767612_i128);
_4 = !3317528342_u32;
_4 = 2893094159_u32;
_1 = !false;
Call(_8 = fn1(RET, _1, _4, RET, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11.2 = 10176757111817320074_u64;
_12 = '\u{cf61a}';
_10 = !59841895501959367713277893728965938887_u128;
_6 = !(-464438401_i32);
_6 = (-280979006_i32) << _10;
_13 = (-9223372036854775808_isize) ^ 69_isize;
_11.1 = _12;
_6 = !(-1609499875_i32);
_11.1 = _12;
_9 = 14815996724881692800_usize * 7660505632754881471_usize;
Goto(bb2)
}
bb2 = {
_5 = _11.1 as i16;
_4 = !346566909_u32;
_7 = _12 as i64;
RET = 51955_u16 as f64;
_2 = _12;
_11.1 = _2;
_11.0 = [_4,_4,_4,_4];
_14 = [_6,_6,_6,_6];
_13 = 10_isize & 92_isize;
_3 = _5 as u16;
_9 = 6_usize | 13006610467760632604_usize;
_8 = (-62601701792934460947303109032639079011_i128) << _9;
_11.0 = [_4,_4,_4,_4];
_3 = 53939_u16;
_13 = 9223372036854775807_isize & (-89_isize);
match _11.2 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
10176757111817320074 => bb10,
_ => bb9
}
}
bb3 = {
_11.2 = 10176757111817320074_u64;
_12 = '\u{cf61a}';
_10 = !59841895501959367713277893728965938887_u128;
_6 = !(-464438401_i32);
_6 = (-280979006_i32) << _10;
_13 = (-9223372036854775808_isize) ^ 69_isize;
_11.1 = _12;
_6 = !(-1609499875_i32);
_11.1 = _12;
_9 = 14815996724881692800_usize * 7660505632754881471_usize;
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
_2 = _11.1;
_15 = [_4,_4,_4,_4];
_18 = _13;
_22.fld1.fld5.1.0 = [59_i8,110_i8,(-109_i8)];
match _3 {
0 => bb1,
1 => bb2,
2 => bb9,
3 => bb4,
4 => bb5,
5 => bb7,
53939 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_22.fld2.2 = _18;
_22.fld5 = core::ptr::addr_of!(_16);
RET = (-126_i8) as f64;
_22.fld1.fld5.0 = (_22.fld1.fld5.1.0,);
_25.0 = [122_i8,100_i8,(-5_i8)];
_10 = 116891077131968073346794450471606601994_u128 | 73302263487712151729376654740486379888_u128;
_22.fld1.fld5.0 = _22.fld1.fld5.1;
_25.0 = [39_i8,15_i8,(-6_i8)];
Goto(bb13)
}
bb13 = {
_14 = [_6,_6,_6,_6];
_21 = _10 as i32;
_2 = _12;
_22.fld1.fld7.2 = !_3;
_22.fld6 = [_4,_4,_4,_4];
_28 = _22.fld2.2;
_22.fld5 = core::ptr::addr_of!(_11.2);
_2 = _11.1;
match _11.2 {
0 => bb5,
1 => bb11,
10176757111817320074 => bb14,
_ => bb4
}
}
bb14 = {
_22.fld1.fld7.0 = [_21,_6,_6,_21];
_22.fld2.3 = (_11.0, _12, _11.2);
_22.fld2.3.2 = !_11.2;
_28 = _22.fld2.2;
_22.fld1.fld5.2 = _4 as f32;
_22.fld2.0 = _2;
_31 = _8 << _11.2;
_22.fld1.fld5.1 = _22.fld1.fld5.0;
_22.fld1.fld5.1 = (_25.0,);
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(0_usize, 12_usize, Move(_12), 21_usize, Move(_21), 18_usize, Move(_18), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(0_usize, 2_usize, Move(_2), 6_usize, Move(_6), 15_usize, Move(_15), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(0_usize, 5_usize, Move(_5), 13_usize, Move(_13), 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: f64,mut _2: bool,mut _3: u32,mut _4: f64,mut _5: bool) -> i128 {
mir! {
type RET = i128;
let _6: Adt41;
let _7: Adt53;
let _8: [u16; 2];
let _9: f32;
let _10: isize;
let _11: Adt43;
let _12: [i8; 3];
let _13: [i8; 3];
let _14: isize;
let _15: ();
let _16: ();
{
_7.fld2.1 = core::ptr::addr_of!(_7.fld1.fld7.1);
match _3 {
2893094159 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_4 = -_1;
_7.fld1.fld7.0 = [(-1522118655_i32),(-772544743_i32),1412895356_i32,1516428275_i32];
_7.fld1.fld7.0 = [(-23620497_i32),(-331514877_i32),1845587270_i32,1142170641_i32];
_3 = 1498786690_u32;
_1 = _4 * _4;
_7.fld1.fld7.3 = _5;
_7.fld2.2 = 5_usize as isize;
_7.fld2.3.2 = 16289599716553223280_u64 >> _7.fld2.2;
_7.fld2.0 = '\u{10aba7}';
_7.fld4 = 159889979915506987323657593564159237525_i128 as f32;
_7.fld3 = core::ptr::addr_of_mut!(_7.fld2.1);
_7.fld1.fld5.3 = 15053215546947013999_usize;
_7.fld1.fld5.3 = 9314407288190534807_usize;
_7.fld1.fld1 = _7.fld2.0;
Goto(bb3)
}
bb3 = {
_9 = (-147010774409871639830558423798554750926_i128) as f32;
_7.fld2.3.0 = [_3,_3,_3,_3];
_7.fld6 = [_3,_3,_3,_3];
_7.fld1.fld7.2 = !13118_u16;
_7.fld1.fld5.2 = -_9;
_7.fld2.3.0 = [_3,_3,_3,_3];
_7.fld5 = core::ptr::addr_of!(_7.fld0);
_5 = !_2;
_13 = [(-128_i8),(-54_i8),(-38_i8)];
_8 = [_7.fld1.fld7.2,_7.fld1.fld7.2];
_1 = _7.fld1.fld7.2 as f64;
_6 = Adt41::Variant0 { fld0: 917677827123237930_i64,fld1: _8 };
_10 = _7.fld2.2 ^ _7.fld2.2;
_10 = -_7.fld2.2;
_7.fld1.fld5.0 = (_13,);
_7.fld5 = core::ptr::addr_of!(_7.fld2.3.2);
_7.fld1.fld7.1 = _7.fld2.3.2 ^ _7.fld2.3.2;
_7.fld1.fld5.1 = (_13,);
_7.fld1.fld5.1.0 = [(-104_i8),22_i8,108_i8];
RET = _7.fld1.fld5.3 as i128;
_6 = Adt41::Variant0 { fld0: 389161959007084065_i64,fld1: _8 };
match _7.fld1.fld5.3 {
0 => bb4,
1 => bb5,
9314407288190534807 => bb7,
_ => bb6
}
}
bb4 = {
_4 = -_1;
_7.fld1.fld7.0 = [(-1522118655_i32),(-772544743_i32),1412895356_i32,1516428275_i32];
_7.fld1.fld7.0 = [(-23620497_i32),(-331514877_i32),1845587270_i32,1142170641_i32];
_3 = 1498786690_u32;
_1 = _4 * _4;
_7.fld1.fld7.3 = _5;
_7.fld2.2 = 5_usize as isize;
_7.fld2.3.2 = 16289599716553223280_u64 >> _7.fld2.2;
_7.fld2.0 = '\u{10aba7}';
_7.fld4 = 159889979915506987323657593564159237525_i128 as f32;
_7.fld3 = core::ptr::addr_of_mut!(_7.fld2.1);
_7.fld1.fld5.3 = 15053215546947013999_usize;
_7.fld1.fld5.3 = 9314407288190534807_usize;
_7.fld1.fld1 = _7.fld2.0;
Goto(bb3)
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
_10 = (-1301179864162996406_i64) as isize;
_7.fld1.fld5.2 = _9 - _9;
_5 = _7.fld1.fld5.2 >= _7.fld4;
_7.fld2.3 = (_7.fld6, _7.fld1.fld1, _7.fld1.fld7.1);
_7.fld1.fld0 = Adt40::Variant1 { fld0: _7.fld3,fld1: _7.fld2.1 };
_7.fld1.fld7.3 = _5 | _5;
_6 = Adt41::Variant0 { fld0: 5346999327712312645_i64,fld1: _8 };
place!(Field::<[u16; 2]>(Variant(_6, 0), 1)) = [_7.fld1.fld7.2,_7.fld1.fld7.2];
_7.fld1.fld5.0 = (_13,);
_7.fld1.fld5.3 = 7_usize * 3441867919540585354_usize;
_7.fld1.fld5.2 = _3 as f32;
Call(_6 = fn2(_7.fld1.fld7, Field::<*const u64>(Variant(_7.fld1.fld0, 1), 1), _7.fld5, _7.fld2, Field::<*const u64>(Variant(_7.fld1.fld0, 1), 1), _7.fld2.3.2, _7.fld3, _7.fld2, _7.fld1.fld7.3, _7.fld2, _7.fld2, _7.fld1.fld5.0.0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_7.fld0 = 199299710622219933579942819481594339340_u128 as u64;
_7.fld2.0 = _7.fld2.3.1;
place!(Field::<*mut *const u64>(Variant(_7.fld1.fld0, 1), 0)) = _7.fld3;
_7.fld1.fld7.1 = Field::<f64>(Variant(_6, 2), 0) as u64;
place!(Field::<f64>(Variant(_6, 2), 0)) = _1;
_13 = [95_i8,73_i8,120_i8];
_11 = Adt43::Variant0 { fld0: Field::<[i64; 8]>(Variant(_6, 2), 3),fld1: _7.fld1.fld1,fld2: Field::<*mut *const u64>(Variant(_7.fld1.fld0, 1), 0),fld3: Move(_6),fld4: Field::<(([i8; 3],), ([i8; 3],), f32, usize)>(Variant(_6, 2), 1) };
place!(Field::<i32>(Variant(place!(Field::<Adt41>(Variant(_11, 0), 3)), 2), 5)) = -1181815326_i32;
place!(Field::<(([i8; 3],), ([i8; 3],), f32, usize)>(Variant(place!(Field::<Adt41>(Variant(_11, 0), 3)), 2), 1)).2 = (-6152055300339377306_i64) as f32;
_12 = Field::<(([i8; 3],), ([i8; 3],), f32, usize)>(Variant(Field::<Adt41>(Variant(_11, 0), 3), 2), 1).0.0;
_7.fld1.fld7.0 = [Field::<i32>(Variant(Field::<Adt41>(Variant(_11, 0), 3), 2), 5),Field::<i32>(Variant(Field::<Adt41>(Variant(_11, 0), 3), 2), 5),Field::<i32>(Variant(Field::<Adt41>(Variant(_11, 0), 3), 2), 5),Field::<i32>(Variant(Field::<Adt41>(Variant(_11, 0), 3), 2), 5)];
_1 = Field::<f64>(Variant(Field::<Adt41>(Variant(_11, 0), 3), 2), 0);
match _3 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
1498786690 => bb14,
_ => bb13
}
}
bb9 = {
_4 = -_1;
_7.fld1.fld7.0 = [(-1522118655_i32),(-772544743_i32),1412895356_i32,1516428275_i32];
_7.fld1.fld7.0 = [(-23620497_i32),(-331514877_i32),1845587270_i32,1142170641_i32];
_3 = 1498786690_u32;
_1 = _4 * _4;
_7.fld1.fld7.3 = _5;
_7.fld2.2 = 5_usize as isize;
_7.fld2.3.2 = 16289599716553223280_u64 >> _7.fld2.2;
_7.fld2.0 = '\u{10aba7}';
_7.fld4 = 159889979915506987323657593564159237525_i128 as f32;
_7.fld3 = core::ptr::addr_of_mut!(_7.fld2.1);
_7.fld1.fld5.3 = 15053215546947013999_usize;
_7.fld1.fld5.3 = 9314407288190534807_usize;
_7.fld1.fld1 = _7.fld2.0;
Goto(bb3)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_4 = -_1;
_7.fld1.fld7.0 = [(-1522118655_i32),(-772544743_i32),1412895356_i32,1516428275_i32];
_7.fld1.fld7.0 = [(-23620497_i32),(-331514877_i32),1845587270_i32,1142170641_i32];
_3 = 1498786690_u32;
_1 = _4 * _4;
_7.fld1.fld7.3 = _5;
_7.fld2.2 = 5_usize as isize;
_7.fld2.3.2 = 16289599716553223280_u64 >> _7.fld2.2;
_7.fld2.0 = '\u{10aba7}';
_7.fld4 = 159889979915506987323657593564159237525_i128 as f32;
_7.fld3 = core::ptr::addr_of_mut!(_7.fld2.1);
_7.fld1.fld5.3 = 15053215546947013999_usize;
_7.fld1.fld5.3 = 9314407288190534807_usize;
_7.fld1.fld1 = _7.fld2.0;
Goto(bb3)
}
bb13 = {
_9 = (-147010774409871639830558423798554750926_i128) as f32;
_7.fld2.3.0 = [_3,_3,_3,_3];
_7.fld6 = [_3,_3,_3,_3];
_7.fld1.fld7.2 = !13118_u16;
_7.fld1.fld5.2 = -_9;
_7.fld2.3.0 = [_3,_3,_3,_3];
_7.fld5 = core::ptr::addr_of!(_7.fld0);
_5 = !_2;
_13 = [(-128_i8),(-54_i8),(-38_i8)];
_8 = [_7.fld1.fld7.2,_7.fld1.fld7.2];
_1 = _7.fld1.fld7.2 as f64;
_6 = Adt41::Variant0 { fld0: 917677827123237930_i64,fld1: _8 };
_10 = _7.fld2.2 ^ _7.fld2.2;
_10 = -_7.fld2.2;
_7.fld1.fld5.0 = (_13,);
_7.fld5 = core::ptr::addr_of!(_7.fld2.3.2);
_7.fld1.fld7.1 = _7.fld2.3.2 ^ _7.fld2.3.2;
_7.fld1.fld5.1 = (_13,);
_7.fld1.fld5.1.0 = [(-104_i8),22_i8,108_i8];
RET = _7.fld1.fld5.3 as i128;
_6 = Adt41::Variant0 { fld0: 389161959007084065_i64,fld1: _8 };
match _7.fld1.fld5.3 {
0 => bb4,
1 => bb5,
9314407288190534807 => bb7,
_ => bb6
}
}
bb14 = {
place!(Field::<[i64; 8]>(Variant(_11, 0), 0)) = [85565626087866682_i64,1499607001887049342_i64,(-1720340342281623373_i64),8569024076951026035_i64,(-2267678952541614085_i64),(-1112998276593913943_i64),(-8229056466497547303_i64),(-3087911967186804270_i64)];
_7.fld2.3 = (_7.fld6, _7.fld2.0, _7.fld1.fld7.1);
place!(Field::<(([i8; 3],), ([i8; 3],), f32, usize)>(Variant(place!(Field::<Adt41>(Variant(_11, 0), 3)), 2), 1)).0.0 = [(-34_i8),(-127_i8),5_i8];
_7.fld2.0 = _7.fld1.fld1;
Goto(bb15)
}
bb15 = {
Call(_15 = dump_var(1_usize, 8_usize, Move(_8), 13_usize, Move(_13), 2_usize, Move(_2), 16_usize, _16), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: ([i32; 4], u64, u16, bool),mut _2: *const u64,mut _3: *const u64,mut _4: (char, *const u64, isize, ([u32; 4], char, u64)),mut _5: *const u64,mut _6: u64,mut _7: *mut *const u64,mut _8: (char, *const u64, isize, ([u32; 4], char, u64)),mut _9: bool,mut _10: (char, *const u64, isize, ([u32; 4], char, u64)),mut _11: (char, *const u64, isize, ([u32; 4], char, u64)),mut _12: [i8; 3]) -> Adt41 {
mir! {
type RET = Adt41;
let _13: u32;
let _14: u16;
let _15: char;
let _16: f32;
let _17: Adt45;
let _18: Adt45;
let _19: f32;
let _20: *mut *const u64;
let _21: usize;
let _22: ([i32; 4], u64, u16, bool);
let _23: Adt52;
let _24: [char; 2];
let _25: [i32; 4];
let _26: u8;
let _27: bool;
let _28: (char, *const u64, isize, ([u32; 4], char, u64));
let _29: [u32; 4];
let _30: i64;
let _31: [i32; 4];
let _32: [char; 2];
let _33: Adt42;
let _34: f32;
let _35: i8;
let _36: bool;
let _37: bool;
let _38: i8;
let _39: i128;
let _40: (([i8; 3],), ([i8; 3],), f32, usize);
let _41: f64;
let _42: f64;
let _43: i128;
let _44: f32;
let _45: (([i8; 3],), ([i8; 3],), f32, usize);
let _46: isize;
let _47: char;
let _48: (i128, i128, u64, u32);
let _49: isize;
let _50: Adt43;
let _51: Adt53;
let _52: [i64; 8];
let _53: (u32, u32, *const isize);
let _54: [u64; 3];
let _55: isize;
let _56: *mut (i32, [i32; 4], i64, u128);
let _57: [char; 2];
let _58: isize;
let _59: ([i8; 3],);
let _60: [i64; 8];
let _61: [char; 2];
let _62: i64;
let _63: isize;
let _64: char;
let _65: isize;
let _66: i8;
let _67: (i32, [i32; 4], i64, u128);
let _68: (i128, i128, u64, u32);
let _69: Adt41;
let _70: u64;
let _71: i64;
let _72: (([i8; 3],), ([i8; 3],), f32, usize);
let _73: u32;
let _74: (i32, [i32; 4], i64, u128);
let _75: isize;
let _76: Adt43;
let _77: [i32; 4];
let _78: &'static u32;
let _79: isize;
let _80: Adt55;
let _81: u64;
let _82: ();
let _83: ();
{
_8.1 = core::ptr::addr_of!(_8.3.2);
_10.1 = core::ptr::addr_of!((*_2));
(*_2) = _1.1;
_10.3.2 = 131_u8 as u64;
_11.3.1 = _8.3.1;
(*_2) = !_1.1;
_11 = _8;
Call(_11.0 = fn3((*_2), _4.1, (*_5), _6, _11.3, _11.3.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4.1 = core::ptr::addr_of!(_4.3.2);
_7 = core::ptr::addr_of_mut!(_8.1);
Goto(bb2)
}
bb2 = {
_8.3.2 = !_1.1;
(*_3) = !(*_5);
(*_2) = (*_3) + _11.3.2;
(*_2) = (*_3);
(*_2) = !(*_3);
_4.1 = _2;
_6 = (*_3) << (*_3);
_10.1 = _2;
(*_3) = 85_u8 as u64;
(*_2) = 9631_i16 as u64;
(*_2) = _6 & _11.3.2;
_8.0 = _4.0;
_11.3.0 = _4.3.0;
_14 = _1.2 << (*_2);
_10.0 = _11.0;
_8.0 = _10.0;
_10.0 = _8.0;
_10.3.2 = (*_5) << (*_2);
Goto(bb3)
}
bb3 = {
(*_7) = core::ptr::addr_of!((*_2));
_11.3 = _10.3;
_4 = (_8.3.1, _8.1, _8.2, _10.3);
_16 = _14 as f32;
_6 = !_4.3.2;
_8.3.2 = _11.3.2;
_8.3.1 = _11.0;
_9 = !_1.3;
_10 = _8;
_8.3 = (_4.3.0, _11.3.1, _4.3.2);
_8 = _11;
_21 = !18003314940535287499_usize;
(*_7) = _11.1;
_14 = 1039476723_i32 as u16;
_1.1 = (*_5);
_8.0 = _8.3.1;
(*_2) = _8.3.2 | _6;
(*_3) = !_4.3.2;
_11 = _8;
(*_3) = _6;
_20 = core::ptr::addr_of_mut!(_10.1);
_11.3 = _10.3;
_1.2 = _14 + _14;
_10 = (_11.0, (*_7), _11.2, _8.3);
(*_3) = !_10.3.2;
_15 = _4.3.1;
_4.0 = _15;
Goto(bb4)
}
bb4 = {
_13 = !4056829010_u32;
_23.fld0 = !_21;
_23.fld1.1 = (*_7);
_23.fld1.3.1 = _11.3.1;
_8.3.2 = !(*_5);
Goto(bb5)
}
bb5 = {
(*_2) = _10.3.2;
_23 = Adt52 { fld0: _21,fld1: _4,fld2: 105_u8 };
_11.1 = _10.1;
_4.3.2 = _11.3.2;
_23.fld1.3 = _8.3;
_20 = core::ptr::addr_of_mut!(_3);
_23 = Adt52 { fld0: _21,fld1: _10,fld2: 110_u8 };
_10.2 = _8.2 + _8.2;
_8.3.0 = [_13,_13,_13,_13];
_8.1 = core::ptr::addr_of!(_23.fld1.3.2);
_10 = _11;
(*_5) = _1.1 | _11.3.2;
_4.3.1 = _15;
_11.3 = _23.fld1.3;
_10.0 = _15;
_11.3.0 = [_13,_13,_13,_13];
_23 = Adt52 { fld0: _21,fld1: _8,fld2: 11_u8 };
_11.1 = core::ptr::addr_of!((*_3));
_23.fld1.3.0 = _11.3.0;
(*_3) = _10.3.2 | (*_5);
_23.fld1.1 = (*_20);
_19 = _16;
(*_5) = (*_3);
_7 = core::ptr::addr_of_mut!(_5);
_8 = (_11.3.1, _23.fld1.1, _4.2, _4.3);
Goto(bb6)
}
bb6 = {
_23.fld1.3.1 = _11.0;
_22.1 = 119_i8 as u64;
_12 = [(-53_i8),105_i8,124_i8];
_11.3.0 = _23.fld1.3.0;
Goto(bb7)
}
bb7 = {
_10.3.1 = _11.0;
_28.3 = (_10.3.0, _23.fld1.3.1, (*_5));
(*_20) = core::ptr::addr_of!(_23.fld1.3.2);
(*_7) = core::ptr::addr_of!((*_3));
_8.0 = _8.3.1;
_13 = 860367601_u32 & 3951309836_u32;
_23.fld1.3 = _10.3;
_1.3 = !_9;
(*_7) = core::ptr::addr_of!((*_5));
_4.1 = _23.fld1.1;
_1.2 = !_14;
_31 = [1082286487_i32,(-993254815_i32),(-1640453663_i32),(-1079100637_i32)];
_4.0 = _4.3.1;
_8 = _4;
_22.0 = [1124963361_i32,372634729_i32,42405678_i32,(-1967770965_i32)];
_12 = [(-88_i8),64_i8,127_i8];
_6 = _9 as u64;
_23.fld1.1 = core::ptr::addr_of!(_22.1);
_11.2 = !_10.2;
_27 = _4.3.2 <= (*_3);
Goto(bb8)
}
bb8 = {
_15 = _28.3.1;
_29 = [_13,_13,_13,_13];
_21 = !_23.fld0;
_23.fld1.3 = (_8.3.0, _15, (*_2));
_4.2 = -_23.fld1.2;
_9 = !_27;
_22.2 = _14 * _1.2;
_21 = _23.fld0;
_28.0 = _11.3.1;
_33 = Adt42::Variant0 { fld0: _8.3,fld1: _28.0 };
_4.2 = _21 as isize;
_26 = _21 as u8;
_23 = Adt52 { fld0: _21,fld1: _11,fld2: _26 };
_22.2 = (-30_i8) as u16;
_16 = _19 * _19;
_23 = Adt52 { fld0: _21,fld1: _10,fld2: _26 };
Call(_28.3.2 = core::intrinsics::bswap(Field::<([u32; 4], char, u64)>(Variant(_33, 0), 0).2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_8.3.1 = _10.0;
_23.fld1.1 = core::ptr::addr_of!(_10.3.2);
_8.1 = _4.1;
_4.3.1 = _15;
_5 = core::ptr::addr_of!(_23.fld1.3.2);
_10.2 = _9 as isize;
_14 = (-6838247675106011507_i64) as u16;
_23.fld1 = (Field::<([u32; 4], char, u64)>(Variant(_33, 0), 0).1, (*_20), _10.2, _28.3);
_19 = _16;
_9 = _1.3;
_16 = 18452068909385447303909348807597401594_i128 as f32;
_4.3.1 = _10.0;
_25 = [923989660_i32,1636228641_i32,1111957641_i32,(-477589619_i32)];
SetDiscriminant(_33, 0);
(*_20) = core::ptr::addr_of!((*_3));
_8.2 = -_10.2;
_28.2 = _10.2 >> (*_5);
_30 = _19 as i64;
_23.fld1.2 = (*_2) as isize;
_11 = (_15, _8.1, _23.fld1.2, _28.3);
place!(Field::<char>(Variant(_33, 0), 1)) = _23.fld1.0;
_28.3.0 = [_13,_13,_13,_13];
_5 = (*_20);
Goto(bb10)
}
bb10 = {
_22 = (_31, _11.3.2, _14, _9);
_39 = 161874795408965945194470122709866196631_i128;
_22.1 = !(*_2);
_40.2 = _19 + _19;
_11.2 = _10.2;
_5 = _3;
_2 = _23.fld1.1;
_22 = _1;
_36 = (*_2) <= (*_3);
_10.2 = _28.2;
_28.1 = core::ptr::addr_of!(_10.3.2);
_11.3.1 = _28.0;
place!(Field::<([u32; 4], char, u64)>(Variant(_33, 0), 0)) = _11.3;
_28.0 = _11.0;
_24 = [_15,_23.fld1.0];
_10.3.1 = Field::<([u32; 4], char, u64)>(Variant(_33, 0), 0).1;
_38 = !(-125_i8);
(*_2) = _36 as u64;
_28.0 = _11.0;
_11.3 = (_10.3.0, Field::<([u32; 4], char, u64)>(Variant(_33, 0), 0).1, _28.3.2);
_10.3 = _28.3;
_9 = !_27;
_28.3.0 = _11.3.0;
Call(_45.3 = core::intrinsics::bswap(_23.fld0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_26 = _30 as u8;
_11.2 = -_28.2;
SetDiscriminant(_33, 0);
_1.0 = [(-1619535789_i32),(-1825219047_i32),937396632_i32,1897875349_i32];
_28.3.0 = _8.3.0;
_4.3.2 = _11.3.2;
_40.1.0 = [_38,_38,_38];
_40.1.0 = [_38,_38,_38];
_42 = _21 as f64;
_41 = _42 + _42;
place!(Field::<([u32; 4], char, u64)>(Variant(_33, 0), 0)) = (_23.fld1.3.0, _8.0, _10.3.2);
_45.2 = _30 as f32;
_37 = _1.3;
_40.1.0 = [_38,_38,_38];
_34 = _45.2 - _19;
_40.2 = _45.2 * _34;
_22.3 = _40.2 == _40.2;
_35 = _1.2 as i8;
_11.3.1 = _4.0;
_16 = _34;
_46 = (-27405_i16) as isize;
_1.0 = [(-1811394456_i32),(-996997834_i32),267221926_i32,(-1875853869_i32)];
_10.3 = (_28.3.0, _8.0, (*_5));
_1.0 = _25;
place!(Field::<([u32; 4], char, u64)>(Variant(_33, 0), 0)) = _23.fld1.3;
_13 = 4263332527_u32;
_23.fld1.3 = (_10.3.0, _11.3.1, _10.3.2);
Goto(bb12)
}
bb12 = {
_4.3.0 = _8.3.0;
(*_2) = _4.3.2;
_2 = core::ptr::addr_of!(_48.2);
_36 = _27;
_10.3.0 = _28.3.0;
place!(Field::<([u32; 4], char, u64)>(Variant(_33, 0), 0)) = (_29, _8.0, _28.3.2);
_46 = _13 as isize;
_23.fld1.2 = -_8.2;
place!(Field::<([u32; 4], char, u64)>(Variant(_33, 0), 0)).0 = _28.3.0;
_40.1 = (_12,);
_7 = _20;
_37 = _9;
(*_2) = (*_5) >> _1.1;
_44 = -_40.2;
_6 = !(*_2);
_15 = _23.fld1.3.1;
_28 = _4;
_51.fld5 = _11.1;
_22.1 = _6;
_51.fld1.fld5.0.0 = [_38,_35,_38];
_23 = Adt52 { fld0: _21,fld1: _4,fld2: _26 };
_23.fld1.3.2 = _35 as u64;
_46 = _11.2;
Goto(bb13)
}
bb13 = {
_10 = _4;
_40.3 = _21 - _23.fld0;
_51.fld2 = _8;
_38 = _35;
_40.1 = (_12,);
_51.fld3 = core::ptr::addr_of_mut!(_3);
_11.1 = core::ptr::addr_of!(_10.3.2);
_53.2 = core::ptr::addr_of!(_4.2);
(*_7) = _2;
_51.fld1.fld7.0 = _25;
_40 = (_51.fld1.fld5.0, _51.fld1.fld5.0, _45.2, _21);
_39 = _13 as i128;
_49 = _41 as isize;
_51.fld1.fld5 = (_40.0, _40.1, _16, _23.fld0);
_53.0 = !_13;
_58 = -_51.fld2.2;
_51.fld1.fld7 = _22;
(*_3) = !_6;
_51.fld6 = _10.3.0;
_59.0 = [_38,_38,_38];
_40.1 = _40.0;
_25 = [521266838_i32,2048928594_i32,488648687_i32,598183292_i32];
_1.0 = [877171140_i32,(-2018004110_i32),(-478764274_i32),(-837953417_i32)];
_11.1 = _3;
_10.2 = _51.fld2.2 & _58;
Goto(bb14)
}
bb14 = {
_57 = [Field::<([u32; 4], char, u64)>(Variant(_33, 0), 0).1,_8.0];
_10.3.1 = _23.fld1.0;
_11.3.0 = [_13,_53.0,_53.0,_53.0];
_28.2 = (-1934826135_i32) as isize;
_28.1 = core::ptr::addr_of!((*_5));
_26 = _41 as u8;
_51.fld1.fld7.2 = _14;
_11 = (_10.0, (*_7), _8.2, _10.3);
(*_5) = _10.3.2;
_8.3.0 = [_53.0,_13,_53.0,_13];
_28.3 = _23.fld1.3;
_51.fld2.3.2 = _36 as u64;
match _13 {
0 => bb1,
1 => bb11,
2 => bb3,
3 => bb10,
4 => bb5,
5 => bb7,
6 => bb15,
4263332527 => bb17,
_ => bb16
}
}
bb15 = {
_8.3.2 = !_1.1;
(*_3) = !(*_5);
(*_2) = (*_3) + _11.3.2;
(*_2) = (*_3);
(*_2) = !(*_3);
_4.1 = _2;
_6 = (*_3) << (*_3);
_10.1 = _2;
(*_3) = 85_u8 as u64;
(*_2) = 9631_i16 as u64;
(*_2) = _6 & _11.3.2;
_8.0 = _4.0;
_11.3.0 = _4.3.0;
_14 = _1.2 << (*_2);
_10.0 = _11.0;
_8.0 = _10.0;
_10.0 = _8.0;
_10.3.2 = (*_5) << (*_2);
Goto(bb3)
}
bb16 = {
_4.1 = core::ptr::addr_of!(_4.3.2);
_7 = core::ptr::addr_of_mut!(_8.1);
Goto(bb2)
}
bb17 = {
_8.2 = _28.0 as isize;
_45.3 = _23.fld0 * _23.fld0;
place!(Field::<([u32; 4], char, u64)>(Variant(_33, 0), 0)).0 = [_13,_53.0,_53.0,_53.0];
_45 = _51.fld1.fld5;
_48 = (_39, _39, _10.3.2, _53.0);
place!(Field::<([u32; 4], char, u64)>(Variant(_33, 0), 0)) = (_11.3.0, _23.fld1.0, _51.fld1.fld7.1);
_51.fld1.fld5.2 = -_19;
_51.fld1.fld5.1.0 = [_38,_35,_35];
_10.3.1 = _23.fld1.3.1;
_10.3.1 = _23.fld1.3.1;
Call(_40.3 = core::intrinsics::bswap(_45.3), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
_8 = (_15, _4.1, _58, _11.3);
(*_3) = _22.1;
_51.fld2.3.1 = _4.0;
_25 = [392865703_i32,1960427416_i32,353296427_i32,532323729_i32];
_45 = (_59, _59, _16, _51.fld1.fld5.3);
_41 = _42;
_23 = Adt52 { fld0: _51.fld1.fld5.3,fld1: _28,fld2: _26 };
_24 = [_8.3.1,Field::<([u32; 4], char, u64)>(Variant(_33, 0), 0).1];
_1 = (_51.fld1.fld7.0, _6, _14, _22.3);
_8.3.2 = (*_3) & _51.fld2.3.2;
_6 = !_51.fld1.fld7.1;
(*_3) = _10.3.2;
_11.3.1 = _11.0;
Goto(bb19)
}
bb19 = {
_23.fld1.2 = _23.fld1.3.1 as isize;
_51.fld1.fld3 = Adt46::Variant1 { fld0: _7,fld1: _51.fld1.fld5 };
_51.fld1.fld1 = _8.3.1;
_48.2 = !_51.fld1.fld7.1;
_61 = [_10.0,_15];
_11 = (_28.0, _5, _51.fld2.2, _4.3);
_23.fld1.2 = _38 as isize;
_51.fld0 = _51.fld1.fld7.1;
_38 = _35 - _35;
_22.1 = !_28.3.2;
_40.2 = -_16;
_22.0 = [(-632692095_i32),806512542_i32,(-185913861_i32),(-1234241282_i32)];
_23.fld1 = _10;
_48.3 = _53.0;
_53.1 = _48.1 as u32;
_13 = _11.2 as u32;
place!(Field::<([u32; 4], char, u64)>(Variant(_33, 0), 0)).1 = _51.fld2.0;
(*_2) = _11.3.1 as u64;
Goto(bb20)
}
bb20 = {
(*_20) = core::ptr::addr_of!(_6);
_55 = _10.2 + _51.fld2.2;
_15 = _28.0;
_11.3.0 = [_13,_13,_13,_13];
(*_7) = _4.1;
_42 = _35 as f64;
_53.1 = _13;
(*_3) = !_10.3.2;
_32 = [Field::<([u32; 4], char, u64)>(Variant(_33, 0), 0).1,_51.fld1.fld1];
_4.1 = core::ptr::addr_of!(_48.2);
Goto(bb21)
}
bb21 = {
_52 = [_30,_30,_30,_30,_30,_30,_30,_30];
_23.fld1 = (_4.3.1, _51.fld5, _51.fld2.2, Field::<([u32; 4], char, u64)>(Variant(_33, 0), 0));
_11 = _10;
_38 = _35 & _35;
_40.0 = (_45.0.0,);
_51.fld1.fld5.0.0 = _12;
_22 = (_1.0, (*_3), _1.2, _1.3);
_1.0 = [1971918770_i32,919443614_i32,1439576415_i32,137358276_i32];
_23.fld2 = _51.fld1.fld1 as u8;
(*_2) = (*_5) | _10.3.2;
_22 = (_31, _1.1, _14, _36);
_68.3 = !_13;
_49 = _58 >> _51.fld1.fld7.1;
_66 = _40.2 as i8;
_6 = _22.1 - _51.fld0;
_22.1 = (*_3);
_23.fld1.3.0 = [_13,_68.3,_68.3,_53.1];
_73 = _13;
_4.2 = _9 as isize;
place!(Field::<([u32; 4], char, u64)>(Variant(_33, 0), 0)).2 = _11.3.2 >> _11.2;
_11.3.2 = !_51.fld1.fld7.1;
Goto(bb22)
}
bb22 = {
SetDiscriminant(_51.fld1.fld3, 1);
(*_20) = core::ptr::addr_of!(_1.1);
_45.0.0 = [_66,_66,_66];
_11 = _23.fld1;
_74.3 = 31260107792406782641366911323001221613_u128 ^ 149286727819378669806300635376534792710_u128;
_64 = _10.3.1;
Goto(bb23)
}
bb23 = {
_60 = [_30,_30,_30,_30,_30,_30,_30,_30];
_45 = _51.fld1.fld5;
_29 = [_53.1,_68.3,_73,_13];
_6 = _23.fld1.3.2;
_62 = _26 as i64;
_3 = _51.fld2.1;
_33 = Adt42::Variant2 { fld0: _37,fld1: _4.0,fld2: _52,fld3: _40.3,fld4: _53.2,fld5: (-613979038_i32),fld6: _8.1 };
_11.3.0 = [_73,_13,_68.3,_13];
_74.2 = (*_2) as i64;
_68.0 = !_39;
_1.2 = _51.fld1.fld7.2;
_67.1 = _22.0;
Goto(bb24)
}
bb24 = {
_23.fld1.3.2 = _10.3.2;
place!(Field::<usize>(Variant(_33, 2), 3)) = _26 as usize;
_70 = !_22.1;
_71 = _30;
_5 = (*_20);
_22.0 = [(-1378897412_i32),(-395706875_i32),(-1911114599_i32),1869694193_i32];
_59.0 = _12;
_67.0 = (-2129105370_i32) | (-1158603265_i32);
_5 = core::ptr::addr_of!(_70);
_49 = _58;
_51.fld2.1 = (*_20);
RET = Adt41::Variant2 { fld0: _41,fld1: _40,fld2: _4.2,fld3: _60,fld4: Field::<usize>(Variant(_33, 2), 3),fld5: _67.0,fld6: _14 };
_29 = _11.3.0;
_11.3.0 = _29;
_60 = Field::<[i64; 8]>(Variant(RET, 2), 3);
_51.fld4 = _66 as f32;
_28.1 = core::ptr::addr_of!((*_2));
_72.0 = (_59.0,);
_32 = [_11.0,_51.fld2.3.1];
Goto(bb25)
}
bb25 = {
Call(_82 = dump_var(2_usize, 32_usize, Move(_32), 36_usize, Move(_36), 29_usize, Move(_29), 27_usize, Move(_27)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_82 = dump_var(2_usize, 64_usize, Move(_64), 25_usize, Move(_25), 59_usize, Move(_59), 35_usize, Move(_35)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Call(_82 = dump_var(2_usize, 6_usize, Move(_6), 62_usize, Move(_62), 52_usize, Move(_52), 70_usize, Move(_70)), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Call(_82 = dump_var(2_usize, 71_usize, Move(_71), 9_usize, Move(_9), 55_usize, Move(_55), 14_usize, Move(_14)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_82 = dump_var(2_usize, 26_usize, Move(_26), 58_usize, Move(_58), 73_usize, Move(_73), 83_usize, _83), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: u64,mut _2: *const u64,mut _3: u64,mut _4: u64,mut _5: ([u32; 4], char, u64),mut _6: [u32; 4]) -> char {
mir! {
type RET = char;
let _7: ([i32; 4], u64, u16, bool);
let _8: ([i32; 4], u64, u16, bool);
let _9: [u16; 2];
let _10: u8;
let _11: f64;
let _12: u8;
let _13: Adt41;
let _14: bool;
let _15: isize;
let _16: [i8; 3];
let _17: Adt51;
let _18: u16;
let _19: bool;
let _20: [u32; 4];
let _21: char;
let _22: f32;
let _23: [char; 2];
let _24: f64;
let _25: Adt56;
let _26: char;
let _27: u16;
let _28: Adt42;
let _29: i16;
let _30: isize;
let _31: [char; 2];
let _32: [i8; 3];
let _33: (([i8; 3],), ([i8; 3],), f32, usize);
let _34: (u32, u32, *const isize);
let _35: i32;
let _36: &'static u32;
let _37: ([i32; 4], u64, u16, bool);
let _38: Adt50;
let _39: usize;
let _40: bool;
let _41: (i32, [i32; 4], i64, u128);
let _42: bool;
let _43: isize;
let _44: ();
let _45: ();
{
_4 = 62942823200186812766066681608730505158_i128 as u64;
_3 = (-120515726_i32) as u64;
RET = _5.1;
_6 = [1477846356_u32,3593982580_u32,140472137_u32,738726606_u32];
RET = _5.1;
_5.0 = _6;
_6 = [2883475177_u32,368507638_u32,2919601322_u32,769043302_u32];
_4 = (*_2);
_5.2 = !_3;
_3 = (-5186321084825912052_i64) as u64;
_7.3 = false;
RET = _5.1;
_5.0 = [1491829114_u32,4263222747_u32,2032655915_u32,3376064098_u32];
_8.0 = [(-927958168_i32),(-738236020_i32),(-984456273_i32),22278731_i32];
_7.2 = 9146_u16 | 55211_u16;
_8.2 = _7.2 | _7.2;
_9 = [_7.2,_7.2];
RET = _5.1;
_8.0 = [(-1152393584_i32),1643368162_i32,(-908957842_i32),1663965769_i32];
_7 = (_8.0, _4, _8.2, true);
_3 = (*_2);
_3 = _7.1;
_8 = (_7.0, _7.1, _7.2, _7.3);
_12 = 154_u8 & 7_u8;
Goto(bb1)
}
bb1 = {
_12 = 114_u8;
_1 = _3;
_7.0 = _8.0;
(*_2) = _8.1;
match _12 {
0 => bb2,
1 => bb3,
2 => bb4,
114 => bb6,
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
_7.2 = _7.3 as u16;
_8.1 = !_7.1;
Call(_7.2 = fn4(_8.0, _8), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_20 = _5.0;
_3 = _4 + _4;
_5 = (_20, RET, _8.1);
_21 = RET;
_8.1 = (*_2);
_6 = [1141162852_u32,2391868411_u32,3512092777_u32,3645046705_u32];
_1 = _4;
_7.3 = _8.3;
(*_2) = _8.1;
_15 = _8.2 as isize;
_12 = 48_u8;
_7.0 = [1563186407_i32,(-1060841765_i32),(-83339826_i32),(-1114388137_i32)];
_10 = 5548784739065948143286053780559117926_i128 as u8;
_1 = (*_2);
_14 = _8.3;
_1 = _4 << _12;
_16 = [(-59_i8),105_i8,(-46_i8)];
_5.2 = !_8.1;
_14 = _7.3;
_20 = [4007824502_u32,2653594616_u32,191169291_u32,162544688_u32];
_22 = 17338594455439514796981552881784735816_u128 as f32;
_24 = 8883730297326280769_usize as f64;
_8.3 = !_14;
_19 = !_7.3;
match _12 {
0 => bb1,
1 => bb6,
48 => bb9,
_ => bb8
}
}
bb8 = {
Return()
}
bb9 = {
_26 = _5.1;
_20 = _6;
_8.1 = _7.1;
_4 = _7.1 - _1;
_7 = _8;
(*_2) = _8.1;
_27 = _7.2;
RET = _5.1;
_12 = _10;
_8.3 = _7.3;
_9 = [_27,_8.2];
_13 = Adt41::Variant0 { fld0: (-3951304034783346941_i64),fld1: _9 };
place!(Field::<[u16; 2]>(Variant(_13, 0), 1)) = [_27,_7.2];
_14 = _7.3;
place!(Field::<[u16; 2]>(Variant(_13, 0), 1)) = [_7.2,_27];
_1 = (*_2) + _8.1;
_6 = _5.0;
_7.2 = !_8.2;
_28 = Adt42::Variant0 { fld0: _5,fld1: _5.1 };
Call(_11 = fn6(_14, _5, Field::<([u32; 4], char, u64)>(Variant(_28, 0), 0).2, _21, Move(_28), _6, _7.3, _7, _1, _7.3, _20, _19, _15, _8.2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
(*_2) = !_1;
_27 = _5.1 as u16;
_24 = -_11;
_8.2 = !_27;
_20 = _5.0;
_22 = 1364486395_i32 as f32;
_7 = _8;
_3 = _8.1 ^ (*_2);
_18 = _27;
_21 = RET;
RET = _21;
_10 = _12;
place!(Field::<[u16; 2]>(Variant(_13, 0), 1)) = _9;
_13 = Adt41::Variant0 { fld0: 3219617684483421239_i64,fld1: _9 };
_19 = !_7.3;
_5.2 = _3;
_8.1 = (*_2) ^ _3;
_27 = _18 ^ _18;
_8.0 = [761117987_i32,426344948_i32,(-837195828_i32),(-775977907_i32)];
_12 = _10 - _10;
Goto(bb11)
}
bb11 = {
_34.2 = core::ptr::addr_of!(_15);
(*_2) = _1;
_26 = _21;
_34.0 = 896446631_i32 as u32;
_35 = -981302796_i32;
_23 = [_5.1,_26];
_34.2 = core::ptr::addr_of!(_15);
_32 = _16;
_15 = (-126_isize);
_33.3 = _4 as usize;
match _15 {
0 => bb7,
1 => bb2,
340282366920938463463374607431768211330 => bb12,
_ => bb4
}
}
bb12 = {
_23 = [_26,RET];
_8.0 = [_35,_35,_35,_35];
_37.3 = (*_2) >= _3;
_24 = _8.2 as f64;
place!(Field::<i64>(Variant(_13, 0), 0)) = -(-8241127499977897797_i64);
_32 = _16;
_34.2 = core::ptr::addr_of!(_30);
_26 = _5.1;
_8 = _7;
SetDiscriminant(_13, 0);
(*_2) = _34.0 as u64;
_33.0.0 = [25_i8,23_i8,(-104_i8)];
_5.2 = !_4;
_33.0.0 = [(-100_i8),(-73_i8),80_i8];
_20 = [_34.0,_34.0,_34.0,_34.0];
match _15 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb10,
4 => bb11,
5 => bb9,
340282366920938463463374607431768211330 => bb13,
_ => bb7
}
}
bb13 = {
RET = _21;
_31 = _23;
_33.1 = (_33.0.0,);
match _15 {
0 => bb3,
1 => bb14,
340282366920938463463374607431768211330 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
_7.2 = _7.3 as u16;
_8.1 = !_7.1;
Call(_7.2 = fn4(_8.0, _8), ReturnTo(bb7), UnwindUnreachable())
}
bb16 = {
place!(Field::<i64>(Variant(_13, 0), 0)) = _12 as i64;
_34.2 = core::ptr::addr_of!(_15);
_8.2 = !_7.2;
_30 = _15 << _8.1;
_5.1 = RET;
_9 = [_18,_18];
(*_2) = _34.0 as u64;
_1 = _4 >> _8.1;
_33.1 = _33.0;
_36 = &_34.0;
(*_2) = !_1;
_35 = 47_i8 as i32;
_37.1 = _21 as u64;
place!(Field::<[u16; 2]>(Variant(_13, 0), 1)) = _9;
_34.0 = !3814727889_u32;
_37.2 = (*_2) as u16;
_15 = _22 as isize;
_41 = (_35, _7.0, Field::<i64>(Variant(_13, 0), 0), 138553609575576877275988864083289559408_u128);
_7.0 = [_35,_35,_35,_35];
_41.1 = [_41.0,_35,_41.0,_35];
_41.3 = 256908948288985210844308054759199211386_u128;
_33.0.0 = [(-70_i8),56_i8,21_i8];
_12 = _10 * _10;
RET = _26;
_30 = _7.3 as isize;
Goto(bb17)
}
bb17 = {
Call(_44 = dump_var(3_usize, 1_usize, Move(_1), 16_usize, Move(_16), 20_usize, Move(_20), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(3_usize, 35_usize, Move(_35), 15_usize, Move(_15), 21_usize, Move(_21), 41_usize, Move(_41)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_44 = dump_var(3_usize, 27_usize, Move(_27), 30_usize, Move(_30), 9_usize, Move(_9), 12_usize, Move(_12)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: [i32; 4],mut _2: ([i32; 4], u64, u16, bool)) -> u16 {
mir! {
type RET = u16;
let _3: (([i8; 3],), ([i8; 3],), f32, usize);
let _4: f64;
let _5: u64;
let _6: Adt40;
let _7: f32;
let _8: usize;
let _9: [u16; 2];
let _10: f32;
let _11: ([i32; 4], u64, u16, bool);
let _12: Adt44;
let _13: u64;
let _14: (i128, i128, u64, u32);
let _15: [i8; 3];
let _16: ();
let _17: ();
{
_2.1 = _2.3 as u64;
_3.0.0 = [122_i8,(-54_i8),52_i8];
_3.2 = 28_i8 as f32;
_2.2 = 39840_u16;
_2.1 = 17321214456718669745_u64 | 5018702646848601628_u64;
_3.3 = !0_usize;
RET = (-9223372036854775808_isize) as u16;
_4 = (-9223372036854775808_isize) as f64;
_3.1.0 = [21_i8,(-65_i8),(-127_i8)];
_4 = (-94_isize) as f64;
_3.0 = (_3.1.0,);
RET = _2.2;
RET = _2.2;
_3.0.0 = [(-91_i8),12_i8,90_i8];
_3.1.0 = _3.0.0;
_3.1.0 = [(-121_i8),(-60_i8),(-102_i8)];
_2.1 = !10647949235788628856_u64;
match _2.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
39840 => bb9,
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
_5 = _2.1;
_2.2 = !RET;
_4 = _3.3 as f64;
_3.1 = (_3.0.0,);
_2.2 = RET % RET;
_2 = (_1, _5, RET, true);
_3.3 = 336118396052125378650084172018061309286_u128 as usize;
RET = _2.2 << _3.3;
_3.0 = (_3.1.0,);
_2.1 = _3.3 as u64;
_4 = 43_u8 as f64;
_3.1 = _3.0;
_2.2 = !RET;
_3.2 = (-1411235276097219220_i64) as f32;
_2.2 = RET;
Goto(bb10)
}
bb10 = {
_3.0.0 = [9_i8,(-53_i8),117_i8];
_4 = (-857667506632404419_i64) as f64;
_1 = [1228240877_i32,906026647_i32,615041576_i32,1863996749_i32];
_3.1.0 = [(-7_i8),99_i8,(-118_i8)];
_8 = 7097219528279320728_i64 as usize;
_3.1.0 = [(-8_i8),(-11_i8),21_i8];
_10 = -_3.2;
_3.1.0 = [61_i8,76_i8,(-31_i8)];
_2 = (_1, _5, RET, true);
_2.0 = [1383402643_i32,1356002194_i32,1561169573_i32,(-1835052274_i32)];
_11.1 = _5 + _5;
_11 = _2;
RET = !_11.2;
_9 = [RET,_2.2];
Call(_3.1.0 = fn5(_3.0, _3.0, _11.3, _11.3, _11.3, _1, _11.2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_3.1.0 = _3.0.0;
_8 = _3.3;
_13 = _5 & _5;
_1 = [(-289560372_i32),833948176_i32,(-970399378_i32),476406052_i32];
_10 = _3.2 + _3.2;
_3.1 = (_3.0.0,);
RET = !_11.2;
_10 = -_3.2;
_4 = (-7348533079789010621_i64) as f64;
_2.1 = '\u{115c2}' as u64;
_3.0.0 = [(-18_i8),(-123_i8),34_i8];
_8 = !_3.3;
_3.1.0 = _3.0.0;
_11.0 = [1734493043_i32,(-740259516_i32),1815351451_i32,496435002_i32];
_3.0.0 = [(-124_i8),29_i8,(-44_i8)];
_11.3 = !_2.3;
RET = _11.2 + _11.2;
RET = _11.2;
Goto(bb12)
}
bb12 = {
_1 = [(-1789144253_i32),(-1682474595_i32),1567055056_i32,(-276441050_i32)];
_3.3 = _8;
_11 = _2;
_2.2 = RET;
_14.0 = 104410757251834721998891116332401295883_i128;
RET = _2.2;
_2.3 = _11.3 & _11.3;
_8 = _3.3 & _3.3;
_11.3 = !_2.3;
_7 = _11.2 as f32;
match _14.0 {
0 => bb5,
104410757251834721998891116332401295883 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_15 = [(-37_i8),(-60_i8),65_i8];
_3.3 = _8;
_8 = _3.3;
Goto(bb15)
}
bb15 = {
Call(_16 = dump_var(4_usize, 8_usize, Move(_8), 5_usize, Move(_5), 2_usize, Move(_2), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: ([i8; 3],),mut _2: ([i8; 3],),mut _3: bool,mut _4: bool,mut _5: bool,mut _6: [i32; 4],mut _7: u16) -> [i8; 3] {
mir! {
type RET = [i8; 3];
let _8: i16;
let _9: (i128, i128, u64, u32);
let _10: Adt47;
let _11: Adt56;
let _12: [u16; 2];
let _13: (i128, i128, u64, u32);
let _14: *const [u32; 4];
let _15: u32;
let _16: (char, *const u64, isize, ([u32; 4], char, u64));
let _17: u16;
let _18: bool;
let _19: Adt48;
let _20: isize;
let _21: bool;
let _22: ();
let _23: ();
{
_3 = _4;
RET = [(-40_i8),19_i8,(-60_i8)];
_5 = _4;
_9.2 = 7018_i16 as u64;
_9.0 = 29512238581652269460054086703780434791_i128;
_9.1 = -_9.0;
RET = [120_i8,7_i8,(-83_i8)];
_6 = [(-1445296132_i32),(-1334216929_i32),(-326647001_i32),2128287219_i32];
_9.2 = 1888082878_i32 as u64;
_9.1 = _9.0;
_4 = _5;
_9.3 = 65_u8 as u32;
_2.0 = RET;
RET = [(-35_i8),84_i8,(-93_i8)];
_9.2 = !15094247955206904427_u64;
RET = _1.0;
_1.0 = [(-128_i8),(-67_i8),96_i8];
match _9.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
29512238581652269460054086703780434791 => bb8,
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
_4 = _3 | _3;
_2.0 = _1.0;
_12 = [_7,_7];
_8 = (-1385_i16) * (-16458_i16);
_8 = 27463_i16;
_5 = !_4;
_9 = (120947552668965714394705943233957023626_i128, 131427126477112266803910634846828850526_i128, 16152004879859725723_u64, 1225225272_u32);
_13.2 = _9.2;
_5 = _4;
Goto(bb9)
}
bb9 = {
_9.0 = _9.1;
_2.0 = [57_i8,(-14_i8),112_i8];
RET = [(-65_i8),(-12_i8),(-87_i8)];
_6 = [579276319_i32,(-1125055461_i32),1818147555_i32,169372514_i32];
_1 = _2;
_13 = (_9.0, _9.0, _9.2, _9.3);
_9.2 = _13.2;
RET = _1.0;
RET = _2.0;
_12 = [_7,_7];
_2.0 = RET;
RET = [(-87_i8),108_i8,(-59_i8)];
_16.0 = '\u{a2ff9}';
_9.3 = _13.2 as u32;
Goto(bb10)
}
bb10 = {
_1.0 = [(-111_i8),(-97_i8),77_i8];
_16.3.2 = 50_i8 as u64;
_4 = _5;
RET = [(-48_i8),77_i8,(-111_i8)];
_16.1 = core::ptr::addr_of!(_13.2);
_14 = core::ptr::addr_of!(_16.3.0);
_15 = _9.3;
_5 = !_4;
_13 = _9;
_9.0 = -_13.1;
_18 = !_4;
_13.1 = _9.0;
_16.2 = 9223372036854775807_isize;
_3 = !_18;
_14 = core::ptr::addr_of!(_16.3.0);
_9 = (_13.1, _13.0, _16.3.2, _13.3);
_13.0 = 72070995683072354719429371044585376867_u128 as i128;
_19.fld7 = (_6, _9.2, _7, _18);
match _9.1 {
0 => bb11,
1 => bb12,
2 => bb13,
131427126477112266803910634846828850526 => bb15,
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
Return()
}
bb15 = {
_9.2 = _13.2 + _13.2;
_19.fld7.3 = _5;
Goto(bb16)
}
bb16 = {
Call(_22 = dump_var(5_usize, 6_usize, Move(_6), 8_usize, Move(_8), 4_usize, Move(_4), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_22 = dump_var(5_usize, 18_usize, Move(_18), 1_usize, Move(_1), 23_usize, _23, 23_usize, _23), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: bool,mut _2: ([u32; 4], char, u64),mut _3: u64,mut _4: char,mut _5: Adt42,mut _6: [u32; 4],mut _7: bool,mut _8: ([i32; 4], u64, u16, bool),mut _9: u64,mut _10: bool,mut _11: [u32; 4],mut _12: bool,mut _13: isize,mut _14: u16) -> f64 {
mir! {
type RET = f64;
let _15: [u32; 4];
let _16: ([i32; 4], u64, u16, bool);
let _17: (i32, [i32; 4], i64, u128);
let _18: usize;
let _19: ([i32; 4], u64, u16, bool);
let _20: i32;
let _21: Adt46;
let _22: isize;
let _23: (char, *const u64, isize, ([u32; 4], char, u64));
let _24: i128;
let _25: u32;
let _26: Adt53;
let _27: i128;
let _28: bool;
let _29: Adt55;
let _30: Adt50;
let _31: Adt42;
let _32: i8;
let _33: ();
let _34: ();
{
RET = 48963421703493134611195200059513385242_u128 as f64;
_12 = _1;
SetDiscriminant(_5, 2);
Call(place!(Field::<i32>(Variant(_5, 2), 5)) = fn7(_9, _2, _8, _2.2, _6, _13, _8, _7, _8.3, _8, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _7 | _10;
_7 = _1;
RET = 86257285862394399464887919455681208465_u128 as f64;
_8.0 = [Field::<i32>(Variant(_5, 2), 5),Field::<i32>(Variant(_5, 2), 5),Field::<i32>(Variant(_5, 2), 5),Field::<i32>(Variant(_5, 2), 5)];
_8.3 = !_1;
_13 = 9223372036854775807_isize | (-51_isize);
place!(Field::<usize>(Variant(_5, 2), 3)) = 196245137_u32 as usize;
_2.2 = _9;
_6 = _2.0;
_17.2 = (-1965435232564156398_i64);
_16 = (_8.0, _8.1, _14, _7);
_17 = (Field::<i32>(Variant(_5, 2), 5), _8.0, 4170359853999050225_i64, 177625599723620369704025020602280516579_u128);
place!(Field::<*const u64>(Variant(_5, 2), 6)) = core::ptr::addr_of!(_9);
_8.0 = _17.1;
_16.2 = _14;
_17 = (Field::<i32>(Variant(_5, 2), 5), _16.0, 8945480320537961615_i64, 289449482132183910444759182622062701668_u128);
_16 = _8;
_19 = _8;
_11 = _6;
_8 = (_19.0, _3, _14, _19.3);
_1 = _8.3 > _7;
_19.2 = _17.3 as u16;
place!(Field::<[i64; 8]>(Variant(_5, 2), 2)) = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
match _17.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
8945480320537961615 => bb10,
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
_15 = [1908176186_u32,2614062445_u32,2997078603_u32,1599704447_u32];
_2.2 = 99599013240987430145219351182278409719_i128 as u64;
_13 = Field::<usize>(Variant(_5, 2), 3) as isize;
_19.2 = _8.2;
_8 = (_19.0, _9, _14, _10);
_18 = _17.3 as usize;
_11 = [2873523050_u32,3520161963_u32,3868183588_u32,2167707622_u32];
_8.2 = !_16.2;
_20 = _17.0;
_19.3 = _10;
Goto(bb11)
}
bb11 = {
place!(Field::<char>(Variant(_5, 2), 1)) = _4;
_2.0 = _11;
_23 = (Field::<char>(Variant(_5, 2), 1), Field::<*const u64>(Variant(_5, 2), 6), _13, _2);
_8.1 = !_3;
_8.1 = !_9;
_5 = Adt42::Variant0 { fld0: _23.3,fld1: _2.1 };
_8.1 = !_9;
_14 = _16.2;
place!(Field::<([u32; 4], char, u64)>(Variant(_5, 0), 0)) = (_23.3.0, _4, _3);
_16.2 = _19.2 * _14;
_1 = !_7;
_8.2 = _16.2 + _16.2;
_14 = _19.2 ^ _8.2;
_17.3 = !58872625659046700235408531536332554107_u128;
RET = 2293610565_u32 as f64;
_25 = _19.2 as u32;
_26.fld2.1 = _23.1;
_26.fld2.0 = _2.1;
_26.fld2.3 = Field::<([u32; 4], char, u64)>(Variant(_5, 0), 0);
match _17.2 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
6 => bb18,
8945480320537961615 => bb20,
_ => bb19
}
}
bb12 = {
_15 = [1908176186_u32,2614062445_u32,2997078603_u32,1599704447_u32];
_2.2 = 99599013240987430145219351182278409719_i128 as u64;
_13 = Field::<usize>(Variant(_5, 2), 3) as isize;
_19.2 = _8.2;
_8 = (_19.0, _9, _14, _10);
_18 = _17.3 as usize;
_11 = [2873523050_u32,3520161963_u32,3868183588_u32,2167707622_u32];
_8.2 = !_16.2;
_20 = _17.0;
_19.3 = _10;
Goto(bb11)
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_1 = _7 | _10;
_7 = _1;
RET = 86257285862394399464887919455681208465_u128 as f64;
_8.0 = [Field::<i32>(Variant(_5, 2), 5),Field::<i32>(Variant(_5, 2), 5),Field::<i32>(Variant(_5, 2), 5),Field::<i32>(Variant(_5, 2), 5)];
_8.3 = !_1;
_13 = 9223372036854775807_isize | (-51_isize);
place!(Field::<usize>(Variant(_5, 2), 3)) = 196245137_u32 as usize;
_2.2 = _9;
_6 = _2.0;
_17.2 = (-1965435232564156398_i64);
_16 = (_8.0, _8.1, _14, _7);
_17 = (Field::<i32>(Variant(_5, 2), 5), _8.0, 4170359853999050225_i64, 177625599723620369704025020602280516579_u128);
place!(Field::<*const u64>(Variant(_5, 2), 6)) = core::ptr::addr_of!(_9);
_8.0 = _17.1;
_16.2 = _14;
_17 = (Field::<i32>(Variant(_5, 2), 5), _16.0, 8945480320537961615_i64, 289449482132183910444759182622062701668_u128);
_16 = _8;
_19 = _8;
_11 = _6;
_8 = (_19.0, _3, _14, _19.3);
_1 = _8.3 > _7;
_19.2 = _17.3 as u16;
place!(Field::<[i64; 8]>(Variant(_5, 2), 2)) = [_17.2,_17.2,_17.2,_17.2,_17.2,_17.2,_17.2,_17.2];
match _17.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
8945480320537961615 => bb10,
_ => bb9
}
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
_2.0 = [_25,_25,_25,_25];
_17.3 = 33904289379762046190564307344493530527_u128 >> _16.2;
place!(Field::<([u32; 4], char, u64)>(Variant(_5, 0), 0)).1 = Field::<char>(Variant(_5, 0), 1);
_26.fld1.fld1 = _4;
_17.2 = 3890209884066901354_i64;
_23.1 = core::ptr::addr_of!(_9);
RET = _17.0 as f64;
_26.fld2 = (Field::<([u32; 4], char, u64)>(Variant(_5, 0), 0).1, _23.1, _23.2, Field::<([u32; 4], char, u64)>(Variant(_5, 0), 0));
_26.fld1.fld5.2 = _25 as f32;
_8 = (_19.0, Field::<([u32; 4], char, u64)>(Variant(_5, 0), 0).2, _14, _12);
_17 = (_20, _8.0, (-7658908133620240363_i64), 276203785511596506973866677869320983191_u128);
_8.0 = [_17.0,_17.0,_17.0,_20];
_26.fld1.fld7.0 = _19.0;
_20 = -_17.0;
_26.fld1.fld7.2 = !_8.2;
_23 = (_26.fld2.0, _26.fld2.1, _13, Field::<([u32; 4], char, u64)>(Variant(_5, 0), 0));
_26.fld5 = core::ptr::addr_of!(_2.2);
Goto(bb21)
}
bb21 = {
Call(_33 = dump_var(6_usize, 11_usize, Move(_11), 2_usize, Move(_2), 20_usize, Move(_20), 25_usize, Move(_25)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_33 = dump_var(6_usize, 19_usize, Move(_19), 6_usize, Move(_6), 17_usize, Move(_17), 15_usize, Move(_15)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_33 = dump_var(6_usize, 3_usize, Move(_3), 16_usize, Move(_16), 34_usize, _34, 34_usize, _34), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: u64,mut _2: ([u32; 4], char, u64),mut _3: ([i32; 4], u64, u16, bool),mut _4: u64,mut _5: [u32; 4],mut _6: isize,mut _7: ([i32; 4], u64, u16, bool),mut _8: bool,mut _9: bool,mut _10: ([i32; 4], u64, u16, bool),mut _11: u64) -> i32 {
mir! {
type RET = i32;
let _12: Adt46;
let _13: Adt46;
let _14: ([i8; 3],);
let _15: Adt49;
let _16: i32;
let _17: *const u64;
let _18: (i32, [i32; 4], i64, u128);
let _19: [u32; 4];
let _20: (i32, [i32; 4], i64, u128);
let _21: [char; 2];
let _22: (([i8; 3],), ([i8; 3],), f32, usize);
let _23: [char; 2];
let _24: isize;
let _25: (i32, [i32; 4], i64, u128);
let _26: isize;
let _27: (([i8; 3],), ([i8; 3],), f32, usize);
let _28: u8;
let _29: u16;
let _30: (i32, [i32; 4], i64, u128);
let _31: f32;
let _32: f64;
let _33: &'static u32;
let _34: char;
let _35: bool;
let _36: [u64; 3];
let _37: u16;
let _38: u8;
let _39: isize;
let _40: (u32, u32, *const isize);
let _41: ();
let _42: ();
{
_2.2 = 14999_i16 as u64;
_3.3 = !_8;
_3.0 = [(-1706390120_i32),(-482517079_i32),(-668044637_i32),1857825192_i32];
_7.1 = !_2.2;
RET = 1524865040_i32 << _3.1;
_2.1 = '\u{98329}';
_7.3 = _10.3 == _8;
_9 = _7.3 ^ _7.3;
_10.2 = _7.2;
_7.1 = 8187443519343790839_i64 as u64;
_11 = _10.1;
_7.0 = [RET,RET,RET,RET];
_10.0 = [RET,RET,RET,RET];
_5 = [1062640683_u32,1104480246_u32,3702581235_u32,3429009123_u32];
_7.1 = 6181_i16 as u64;
_10 = (_7.0, _1, _3.2, _9);
_10.1 = !_11;
_10.0 = _3.0;
Goto(bb1)
}
bb1 = {
_10.3 = _3.3;
_7.3 = _9;
_2.2 = (-91_i8) as u64;
_5 = _2.0;
_10.2 = !_3.2;
_12 = Adt46::Variant3 { fld0: _5,fld1: 8947682197615898853_i64 };
_2.2 = _1 >> _4;
_10.0 = [RET,RET,RET,RET];
_2.2 = !_3.1;
_2 = (Field::<[u32; 4]>(Variant(_12, 3), 0), '\u{bd00c}', _4);
_10.3 = _9;
_4 = 303348350473556769074203917303207767260_u128 as u64;
_1 = _2.2 + _4;
_4 = _3.1 & _10.1;
_6 = 104_isize | 9223372036854775807_isize;
_1 = _2.1 as u64;
_3 = (_10.0, _1, _10.2, _7.3);
_10.2 = !_3.2;
_7.3 = _3.3 > _10.3;
place!(Field::<[u32; 4]>(Variant(_12, 3), 0)) = _2.0;
_6 = !(-9223372036854775808_isize);
_4 = _1 >> _7.2;
Call(_8 = fn8(_7.3, _7, _7, _7.3, _7.3, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2.1 = '\u{ad91a}';
_4 = !_11;
_2 = (_5, '\u{d48fb}', _3.1);
_5 = _2.0;
_4 = 1861127995_u32 as u64;
_6 = 86_isize - 17_isize;
_5 = _2.0;
place!(Field::<[u32; 4]>(Variant(_12, 3), 0)) = _5;
_2.1 = '\u{11e67}';
_7.1 = RET as u64;
RET = 1_i8 as i32;
_10 = (_3.0, _1, _7.2, _9);
place!(Field::<i64>(Variant(_12, 3), 1)) = 6307124244365351326_i64 - 7521405202284272254_i64;
_14.0 = [39_i8,(-9_i8),6_i8];
_6 = _2.1 as isize;
_15.fld1.2 = !_6;
_15.fld1.0 = _2.1;
_15.fld1.3.2 = _3.3 as u64;
_2.2 = 44_u8 as u64;
_7.2 = _10.2 & _3.2;
_9 = _3.3;
Call(_15.fld1.3.1 = fn9(_8, _9, _3, _10, _7, _9, _3, _9, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_15.fld0 = !_10.1;
_17 = core::ptr::addr_of!(_10.1);
Goto(bb4)
}
bb4 = {
_7 = (_10.0, _1, _10.2, _8);
_2 = (Field::<[u32; 4]>(Variant(_12, 3), 0), _15.fld1.3.1, _15.fld1.3.2);
_15.fld1 = (_2.1, _17, _6, _2);
_15.fld2 = 12962846336827440164_usize as i8;
_10.0 = [RET,RET,RET,RET];
_18.3 = _15.fld2 as u128;
_7.2 = 3371387263_u32 as u16;
_15.fld1.3.2 = _2.2 * _2.2;
_20.1 = _3.0;
Goto(bb5)
}
bb5 = {
_1 = _15.fld1.3.2;
_18.0 = RET;
place!(Field::<i64>(Variant(_12, 3), 1)) = _15.fld1.3.2 as i64;
_1 = _15.fld1.3.2;
_15.fld1.2 = _9 as isize;
_3.2 = !_10.2;
_18.2 = -Field::<i64>(Variant(_12, 3), 1);
_1 = _7.3 as u64;
_3.3 = _8;
_16 = RET;
RET = 5306178475102980395_usize as i32;
_14.0 = [_15.fld2,_15.fld2,_15.fld2];
_18.1 = [_18.0,_16,_18.0,_18.0];
_15.fld1.3 = _2;
Call(_15.fld1.1 = core::intrinsics::arith_offset(_17, (-53_isize)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_7.3 = _10.3 & _9;
_20.0 = -_16;
_20.0 = !RET;
Goto(bb7)
}
bb7 = {
RET = 249_u8 as i32;
_7.3 = _8;
_22.0.0 = [_15.fld2,_15.fld2,_15.fld2];
_20.3 = !_18.3;
_15.fld2 = 13_i8;
_20 = _18;
_10.3 = !_3.3;
_20.1 = [RET,RET,_16,_18.0];
_8 = !_10.3;
_21 = [_15.fld1.3.1,_2.1];
_2 = (Field::<[u32; 4]>(Variant(_12, 3), 0), _15.fld1.0, _15.fld1.3.2);
match _15.fld2 {
0 => bb1,
1 => bb6,
2 => bb5,
13 => bb9,
_ => bb8
}
}
bb8 = {
_2.1 = '\u{ad91a}';
_4 = !_11;
_2 = (_5, '\u{d48fb}', _3.1);
_5 = _2.0;
_4 = 1861127995_u32 as u64;
_6 = 86_isize - 17_isize;
_5 = _2.0;
place!(Field::<[u32; 4]>(Variant(_12, 3), 0)) = _5;
_2.1 = '\u{11e67}';
_7.1 = RET as u64;
RET = 1_i8 as i32;
_10 = (_3.0, _1, _7.2, _9);
place!(Field::<i64>(Variant(_12, 3), 1)) = 6307124244365351326_i64 - 7521405202284272254_i64;
_14.0 = [39_i8,(-9_i8),6_i8];
_6 = _2.1 as isize;
_15.fld1.2 = !_6;
_15.fld1.0 = _2.1;
_15.fld1.3.2 = _3.3 as u64;
_2.2 = 44_u8 as u64;
_7.2 = _10.2 & _3.2;
_9 = _3.3;
Call(_15.fld1.3.1 = fn9(_8, _9, _3, _10, _7, _9, _3, _9, _7), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_21 = [_15.fld1.0,_2.1];
_22.0 = (_14.0,);
Goto(bb10)
}
bb10 = {
_13 = Move(_12);
_16 = _18.0 >> _18.0;
_10.2 = !_3.2;
_4 = _7.2 as u64;
_22.1.0 = _14.0;
_1 = _15.fld1.2 as u64;
_15.fld1.3.2 = _1;
_2.0 = [1077542288_u32,276098666_u32,1907529599_u32,725694215_u32];
_22.2 = _7.2 as f32;
_2.1 = _15.fld1.3.1;
_25.1 = [_18.0,_18.0,RET,_20.0];
_22.3 = 6378386232447004151_usize;
_14 = _22.1;
_26 = _15.fld1.2 << _18.2;
_22.3 = 11373943107543175713_usize + 1691457484665476620_usize;
_27.1 = (_14.0,);
_25.0 = -RET;
SetDiscriminant(_13, 3);
Goto(bb11)
}
bb11 = {
_1 = 83_u8 as u64;
_27.0 = (_27.1.0,);
_9 = _8;
_15.fld0 = _3.1;
(*_17) = _7.1;
_30.1 = [_20.0,RET,RET,_16];
_18.3 = _20.3 * _20.3;
_7.1 = (-1385_i16) as u64;
_30.2 = -_20.2;
_31 = _2.2 as f32;
_22.1 = (_22.0.0,);
_12 = Adt46::Variant0 { fld0: 862973902_u32,fld1: _15.fld1 };
(*_17) = _26 as u64;
match _15.fld2 {
0 => bb8,
1 => bb10,
2 => bb7,
13 => bb12,
_ => bb4
}
}
bb12 = {
_30 = (_16, _3.0, _18.2, _18.3);
_25 = _18;
_34 = Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_12, 0), 1).3.1;
_11 = !_10.1;
_28 = !126_u8;
_28 = _25.2 as u8;
_2.0 = [2976251212_u32,171327384_u32,1477183575_u32,96520236_u32];
_3.0 = [_16,_25.0,_18.0,_25.0];
_18 = (_30.0, _7.0, _30.2, _25.3);
_25.3 = _18.3;
place!(Field::<u32>(Variant(_12, 0), 0)) = !4092560632_u32;
_3 = _7;
_19 = [Field::<u32>(Variant(_12, 0), 0),Field::<u32>(Variant(_12, 0), 0),Field::<u32>(Variant(_12, 0), 0),Field::<u32>(Variant(_12, 0), 0)];
_25.2 = (-80953496419011279496668169222779944767_i128) as i64;
Goto(bb13)
}
bb13 = {
_25.2 = _20.2 - _18.2;
_19 = _15.fld1.3.0;
_37 = !_10.2;
Goto(bb14)
}
bb14 = {
_14.0 = [_15.fld2,_15.fld2,_15.fld2];
_22.1.0 = _27.0.0;
_2.2 = (*_17);
_15.fld1.3.2 = !_11;
_25 = (_16, _30.1, _18.2, _18.3);
_34 = Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_12, 0), 1).0;
_30 = _25;
_38 = (-59806102477011709993785758131554645412_i128) as u8;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(7_usize, 21_usize, Move(_21), 11_usize, Move(_11), 30_usize, Move(_30), 38_usize, Move(_38)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(7_usize, 16_usize, Move(_16), 9_usize, Move(_9), 10_usize, Move(_10), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(7_usize, 3_usize, Move(_3), 28_usize, Move(_28), 26_usize, Move(_26), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: bool,mut _2: ([i32; 4], u64, u16, bool),mut _3: ([i32; 4], u64, u16, bool),mut _4: bool,mut _5: bool,mut _6: ([i32; 4], u64, u16, bool)) -> bool {
mir! {
type RET = bool;
let _7: isize;
let _8: *const [u32; 4];
let _9: usize;
let _10: [u16; 2];
let _11: ();
let _12: ();
{
_6.3 = _5 > _3.3;
_4 = !_2.3;
_7 = 9223372036854775807_isize;
RET = !_5;
_3.2 = _2.2;
_6.1 = _2.1 + _2.1;
_7 = 1725865382_i32 as isize;
RET = _2.3 | _1;
_1 = !RET;
_2 = (_6.0, _6.1, _3.2, _3.3);
_3.1 = !_2.1;
_1 = RET;
_3.3 = _2.3 ^ RET;
_2 = (_3.0, _6.1, _6.2, RET);
RET = _3.3 != _5;
_6.0 = [(-1980959997_i32),646846670_i32,133363063_i32,(-61463380_i32)];
RET = !_6.3;
_6.2 = _5 as u16;
_3.0 = _2.0;
_2.1 = '\u{2a272}' as u64;
_4 = _6.3 & _1;
_2.3 = !_1;
_3 = (_2.0, _6.1, _6.2, RET);
_6.0 = [370995956_i32,296302282_i32,(-1012876849_i32),(-700167302_i32)];
_2.1 = _6.1;
_10 = [_3.2,_3.2];
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(8_usize, 5_usize, Move(_5), 1_usize, Move(_1), 10_usize, Move(_10), 2_usize, Move(_2)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: bool,mut _2: bool,mut _3: ([i32; 4], u64, u16, bool),mut _4: ([i32; 4], u64, u16, bool),mut _5: ([i32; 4], u64, u16, bool),mut _6: bool,mut _7: ([i32; 4], u64, u16, bool),mut _8: bool,mut _9: ([i32; 4], u64, u16, bool)) -> char {
mir! {
type RET = char;
let _10: [u32; 4];
let _11: Adt40;
let _12: [char; 2];
let _13: [i32; 4];
let _14: f64;
let _15: [char; 2];
let _16: char;
let _17: (([i8; 3],), ([i8; 3],), f32, usize);
let _18: Adt45;
let _19: Adt46;
let _20: ([i32; 4], u64, u16, bool);
let _21: [u64; 3];
let _22: Adt44;
let _23: char;
let _24: Adt42;
let _25: ();
let _26: ();
{
_9.0 = _4.0;
_2 = _4.3;
_7.3 = _5.3;
_9.0 = [1249199720_i32,1196789078_i32,(-155609851_i32),(-927776290_i32)];
_3.3 = !_8;
_5.0 = [311648973_i32,130945005_i32,1331220410_i32,(-1297998275_i32)];
_8 = !_5.3;
_7.0 = [(-1851346069_i32),(-880255799_i32),(-543030727_i32),(-93672883_i32)];
_5.1 = _7.1 & _7.1;
_5.0 = [(-1946483798_i32),1729258640_i32,2112423697_i32,1608033390_i32];
_6 = _7.3;
_10 = [1817846585_u32,3779362827_u32,2405922177_u32,4214112191_u32];
_10 = [3638125487_u32,2710047289_u32,251786090_u32,318410042_u32];
_7.3 = !_8;
_5.1 = 158_u8 as u64;
_9.3 = _7.3 | _8;
_5.1 = _3.1;
RET = '\u{af964}';
_7.0 = [150610940_i32,(-1131568979_i32),(-90477283_i32),1502095931_i32];
_4.2 = _7.2 & _3.2;
_5.2 = !_3.2;
Call(_9.2 = fn10(_7, _5, _3, _7.3, _2, _4.1, _3, _4, _7, _9.3, _10, _9.3, _8, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = _3;
_10 = [2074762142_u32,4082363453_u32,2364553106_u32,3166657182_u32];
_4.0 = _3.0;
_7.3 = _9.3 > _2;
_5.1 = !_9.1;
_3.0 = [(-341980969_i32),773993305_i32,1383272212_i32,(-1264278070_i32)];
RET = '\u{20e10}';
_4 = _7;
_4.0 = _5.0;
_3.3 = _6;
_13 = [868977365_i32,(-510624328_i32),940728903_i32,(-1051540552_i32)];
_14 = (-9223372036854775808_isize) as f64;
_4.0 = _7.0;
_3.3 = _5.3 != _9.3;
_3.3 = _1;
_7.0 = [1699228792_i32,(-1501557881_i32),(-147012531_i32),620076721_i32];
Call(_5.2 = fn13(_9, _4.3, _4.3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_16 = RET;
_12 = [_16,_16];
_5.0 = [2039083560_i32,(-612936928_i32),(-82051225_i32),585357718_i32];
_14 = (-4554225702996454441_i64) as f64;
_3.0 = [393566048_i32,(-259683776_i32),(-435196057_i32),1260278895_i32];
_12 = [_16,_16];
_3 = (_13, _5.1, _7.2, _4.3);
_5.3 = !_1;
_14 = 78_isize as f64;
_9.0 = [58741508_i32,980114799_i32,1646412015_i32,(-461724587_i32)];
_9 = (_7.0, _5.1, _7.2, _3.3);
_9.1 = !_4.1;
_5.2 = !_7.2;
_7.0 = [1593428297_i32,(-947603164_i32),794117720_i32,1483803214_i32];
_8 = _6;
_7.2 = _4.2 + _4.2;
_5.1 = _9.1;
_7.2 = _5.2;
Goto(bb3)
}
bb3 = {
_4.3 = _5.3;
_17.3 = 987619827057727243_usize;
_5 = _7;
_4.3 = _8;
_12 = [_16,RET];
_6 = !_4.3;
_1 = _9.3 != _8;
_4.1 = !_3.1;
_15 = [RET,RET];
match _17.3 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
987619827057727243 => bb12,
_ => bb11
}
}
bb4 = {
_16 = RET;
_12 = [_16,_16];
_5.0 = [2039083560_i32,(-612936928_i32),(-82051225_i32),585357718_i32];
_14 = (-4554225702996454441_i64) as f64;
_3.0 = [393566048_i32,(-259683776_i32),(-435196057_i32),1260278895_i32];
_12 = [_16,_16];
_3 = (_13, _5.1, _7.2, _4.3);
_5.3 = !_1;
_14 = 78_isize as f64;
_9.0 = [58741508_i32,980114799_i32,1646412015_i32,(-461724587_i32)];
_9 = (_7.0, _5.1, _7.2, _3.3);
_9.1 = !_4.1;
_5.2 = !_7.2;
_7.0 = [1593428297_i32,(-947603164_i32),794117720_i32,1483803214_i32];
_8 = _6;
_7.2 = _4.2 + _4.2;
_5.1 = _9.1;
_7.2 = _5.2;
Goto(bb3)
}
bb5 = {
_5 = _3;
_10 = [2074762142_u32,4082363453_u32,2364553106_u32,3166657182_u32];
_4.0 = _3.0;
_7.3 = _9.3 > _2;
_5.1 = !_9.1;
_3.0 = [(-341980969_i32),773993305_i32,1383272212_i32,(-1264278070_i32)];
RET = '\u{20e10}';
_4 = _7;
_4.0 = _5.0;
_3.3 = _6;
_13 = [868977365_i32,(-510624328_i32),940728903_i32,(-1051540552_i32)];
_14 = (-9223372036854775808_isize) as f64;
_4.0 = _7.0;
_3.3 = _5.3 != _9.3;
_3.3 = _1;
_7.0 = [1699228792_i32,(-1501557881_i32),(-147012531_i32),620076721_i32];
Call(_5.2 = fn13(_9, _4.3, _4.3), ReturnTo(bb2), UnwindUnreachable())
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
Return()
}
bb12 = {
_4.2 = _5.2;
_17.2 = 12287_i16 as f32;
_3.2 = _9.2;
_17.0.0 = [62_i8,83_i8,(-20_i8)];
_7.1 = !_4.1;
_2 = _8 <= _4.3;
_3.3 = !_5.3;
Goto(bb13)
}
bb13 = {
_3.1 = _5.1 + _9.1;
_17.3 = 3_usize;
_4.2 = !_7.2;
_7.2 = 142952989122810492349637248259070212033_u128 as u16;
_21 = [_7.1,_9.1,_9.1];
_20.1 = !_9.1;
_10 = [1114929942_u32,4238909932_u32,1982723232_u32,3394116235_u32];
_3.2 = _2 as u16;
_4.2 = !_3.2;
_4.2 = _3.2 - _3.2;
_15 = [_16,RET];
Goto(bb14)
}
bb14 = {
_9.3 = !_8;
_23 = _16;
_9.1 = _9.3 as u64;
_17.0.0 = [(-74_i8),(-94_i8),(-87_i8)];
_9.3 = _4.3;
_12 = [_23,_23];
RET = _23;
_3.0 = _9.0;
_8 = !_3.3;
_3.3 = !_1;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(9_usize, 2_usize, Move(_2), 13_usize, Move(_13), 10_usize, Move(_10), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(9_usize, 5_usize, Move(_5), 15_usize, Move(_15), 23_usize, Move(_23), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: ([i32; 4], u64, u16, bool),mut _2: ([i32; 4], u64, u16, bool),mut _3: ([i32; 4], u64, u16, bool),mut _4: bool,mut _5: bool,mut _6: u64,mut _7: ([i32; 4], u64, u16, bool),mut _8: ([i32; 4], u64, u16, bool),mut _9: ([i32; 4], u64, u16, bool),mut _10: bool,mut _11: [u32; 4],mut _12: bool,mut _13: bool,mut _14: ([i32; 4], u64, u16, bool)) -> u16 {
mir! {
type RET = u16;
let _15: Adt54;
let _16: &'static u32;
let _17: (i32, [i32; 4], i64, u128);
let _18: f64;
let _19: f64;
let _20: (i32, [i32; 4], i64, u128);
let _21: [u16; 2];
let _22: bool;
let _23: Adt52;
let _24: [i64; 8];
let _25: f32;
let _26: *mut (i32, [i32; 4], i64, u128);
let _27: u32;
let _28: Adt43;
let _29: ();
let _30: ();
{
_3.1 = _2.2 as u64;
_8.1 = !_2.1;
_2.2 = _14.2;
_7.3 = _1.3;
_8.1 = (-9223372036854775808_isize) as u64;
_1 = _2;
_10 = !_2.3;
_1 = _9;
RET = !_14.2;
_14.3 = !_1.3;
_13 = _2.3;
_3.0 = [660566750_i32,845480461_i32,292400376_i32,(-433830309_i32)];
_3.1 = !_7.1;
_1.2 = _2.2;
_2.1 = _1.1;
_1.2 = !_3.2;
_14.1 = (-64_i8) as u64;
_7.1 = 22643791158832290293964440488386560017_i128 as u64;
_2.3 = !_5;
_7.2 = _8.2;
_14 = (_1.0, _3.1, _8.2, _5);
_17.2 = 93_u8 as i64;
_5 = _10 != _13;
_9.2 = 9223372036854775807_isize as u16;
_17.0 = (-57938853061994247411772666224035917652_i128) as i32;
Call(_9.2 = fn11(_10, _13, _3.1, _12, _7.2, _13, _1, _5, _8.2, _2, _1.3, _2.3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_21 = [_7.2,_9.2];
_8.2 = RET;
_20.1 = [_17.0,_17.0,_17.0,_17.0];
_2 = _14;
_18 = 228485681_u32 as f64;
_1.0 = [_17.0,_17.0,_17.0,_17.0];
_7.0 = [_17.0,_17.0,_17.0,_17.0];
_12 = !_7.3;
_14.1 = _2.1 | _1.1;
_17 = (867991734_i32, _14.0, 4368849766358838744_i64, 331702553487816153407391198103633861055_u128);
_8 = (_17.1, _1.1, _14.2, _7.3);
_14.1 = _9.1 - _2.1;
_23.fld1.3 = (_11, '\u{5402}', _2.1);
_9.2 = _8.2;
_17.1 = [_17.0,_17.0,_17.0,_17.0];
_9.2 = _8.2 ^ _7.2;
_23.fld1.2 = -9223372036854775807_isize;
_23.fld1.3.1 = '\u{7962e}';
_19 = -_18;
_2.1 = _14.1;
Call(_2 = fn12(_3.3, _8.3, _13, _4, _7.3, _10, _10, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_14 = (_17.1, _1.1, _1.2, _12);
_2.0 = [_17.0,_17.0,_17.0,_17.0];
RET = !_9.2;
_14.2 = _7.2;
_17 = ((-633209401_i32), _14.0, (-6244230359222057459_i64), 278814918791793899200187345094719874537_u128);
RET = !_9.2;
_23.fld0 = 7897707253829862556_usize;
_23.fld1.3 = (_11, '\u{8e3e8}', _14.1);
_1.3 = _4;
_13 = !_10;
_1.3 = _4;
Goto(bb3)
}
bb3 = {
Call(_29 = dump_var(10_usize, 4_usize, Move(_4), 11_usize, Move(_11), 21_usize, Move(_21), 6_usize, Move(_6)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_29 = dump_var(10_usize, 14_usize, Move(_14), 7_usize, Move(_7), 2_usize, Move(_2), 10_usize, Move(_10)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: bool,mut _2: bool,mut _3: u64,mut _4: bool,mut _5: u16,mut _6: bool,mut _7: ([i32; 4], u64, u16, bool),mut _8: bool,mut _9: u16,mut _10: ([i32; 4], u64, u16, bool),mut _11: bool,mut _12: bool) -> u16 {
mir! {
type RET = u16;
let _13: f32;
let _14: isize;
let _15: ();
let _16: ();
{
_6 = _2 | _1;
Goto(bb1)
}
bb1 = {
_1 = _6;
RET = _5 | _7.2;
_10.0 = [(-1629653394_i32),(-1088478211_i32),(-1043974695_i32),352488254_i32];
_1 = !_11;
_9 = !RET;
_7.0 = _10.0;
_6 = _11 >= _12;
Goto(bb2)
}
bb2 = {
Call(_15 = dump_var(11_usize, 8_usize, Move(_8), 11_usize, Move(_11), 6_usize, Move(_6), 2_usize, Move(_2)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_15 = dump_var(11_usize, 7_usize, Move(_7), 1_usize, Move(_1), 16_usize, _16, 16_usize, _16), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: ([i32; 4], u64, u16, bool)) -> ([i32; 4], u64, u16, bool) {
mir! {
type RET = ([i32; 4], u64, u16, bool);
let _9: isize;
let _10: f32;
let _11: ();
let _12: ();
{
RET.0 = [(-1750901392_i32),(-874425452_i32),(-1715804473_i32),757850540_i32];
RET.2 = _8.2;
_8.0 = [(-1082213737_i32),(-1133590597_i32),2103838124_i32,(-744935068_i32)];
RET.2 = 240246209_u32 as u16;
RET.2 = _8.2;
_5 = !_2;
_4 = !_3;
RET.0 = [893202701_i32,(-1626997394_i32),739584596_i32,242710981_i32];
RET = _8;
RET.3 = _2;
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(12_usize, 6_usize, Move(_6), 3_usize, Move(_3), 2_usize, Move(_2), 1_usize, Move(_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: ([i32; 4], u64, u16, bool),mut _2: bool,mut _3: bool) -> u16 {
mir! {
type RET = u16;
let _4: f32;
let _5: i128;
let _6: Adt47;
let _7: [char; 2];
let _8: i128;
let _9: [u32; 4];
let _10: [u32; 4];
let _11: [i64; 8];
let _12: *mut &'static u32;
let _13: Adt44;
let _14: f64;
let _15: i64;
let _16: isize;
let _17: i32;
let _18: usize;
let _19: char;
let _20: i64;
let _21: f64;
let _22: [u64; 3];
let _23: f32;
let _24: f32;
let _25: f32;
let _26: isize;
let _27: [u32; 4];
let _28: Adt54;
let _29: f32;
let _30: (([i8; 3],), ([i8; 3],), f32, usize);
let _31: ();
let _32: ();
{
_1.0 = [339394037_i32,(-1569638942_i32),385113570_i32,(-1903872621_i32)];
_1.2 = !56067_u16;
_3 = _1.3 >= _1.3;
_1.0 = [542078513_i32,(-1803481075_i32),1720116811_i32,(-1598198131_i32)];
RET = 3185474469866868768_i64 as u16;
RET = _1.2;
_1.1 = 16341689925025137563_u64 + 9181337075804253973_u64;
_2 = _3 != _3;
RET = _1.2 | _1.2;
RET = 19_u8 as u16;
_4 = 13093_i16 as f32;
_2 = !_1.3;
_3 = _2;
_1.0 = [1510118822_i32,1254348877_i32,2060906952_i32,(-1878981172_i32)];
_1.1 = 12694536838906610705_u64 * 13527862675448867387_u64;
_3 = _2;
_3 = _2 == _1.3;
_4 = _1.2 as f32;
Goto(bb1)
}
bb1 = {
RET = _1.2;
_5 = (-137347825849840307949575799587197530155_i128);
_3 = _2 | _2;
_1.3 = !_3;
_5 = 69825080047191029621468155918724691433_u128 as i128;
Goto(bb2)
}
bb2 = {
RET = !_1.2;
_2 = _3;
_1.3 = _2;
_4 = _5 as f32;
_7 = ['\u{5bd52}','\u{ce86f}'];
_1.3 = !_3;
_4 = _5 as f32;
RET = (-9223372036854775808_isize) as u16;
_8 = -_5;
_1.2 = 77_i8 as u16;
_3 = _2;
RET = _5 as u16;
_4 = _1.2 as f32;
_7 = ['\u{5755b}','\u{10d535}'];
RET = _1.2;
_10 = [651309980_u32,886276250_u32,1369085811_u32,478059970_u32];
RET = _1.2 & _1.2;
_9 = [4213050797_u32,2152199148_u32,2917276247_u32,1013028449_u32];
_3 = _1.3 & _2;
_4 = 0_usize as f32;
_1.0 = [2095088134_i32,1991991293_i32,197179606_i32,661042592_i32];
_9 = [3931004852_u32,2555946859_u32,450563590_u32,2037198692_u32];
_10 = [2513882615_u32,725463939_u32,1945356037_u32,1337197741_u32];
Call(_9 = fn14(_3, _2, _1.3, _3, _3, _2, _1.3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = _1.2 + _1.2;
_1.1 = 15057887271443055326_usize as u64;
_11 = [4352334936744675754_i64,(-7335000338272296057_i64),7966206383608089915_i64,3348075688596635097_i64,(-2409940478572130511_i64),6257105060872420512_i64,5444647942697746734_i64,7453087766720437391_i64];
_2 = !_3;
_9 = [2522579339_u32,1739763509_u32,1833242297_u32,3264276317_u32];
RET = !_1.2;
_1.1 = 7805935550621439635_u64 * 2643776663456359419_u64;
_10 = [1707788350_u32,3643631057_u32,2590868105_u32,3598216827_u32];
_7 = ['\u{f0a9b}','\u{39905}'];
_1.0 = [63378992_i32,(-230541488_i32),(-893653602_i32),(-778451589_i32)];
_2 = _1.3 > _3;
RET = !_1.2;
RET = _1.2;
_11 = [8837761770945240626_i64,(-9097339373801756855_i64),(-7377250714647037397_i64),(-5558240671815989559_i64),(-3483761243381165963_i64),1665194851410557480_i64,6321895465453257025_i64,7643980180154870696_i64];
_4 = _1.1 as f32;
_1.1 = !4484591744304494756_u64;
_1.0 = [1475534761_i32,1998904647_i32,1525109092_i32,1179977088_i32];
Goto(bb4)
}
bb4 = {
RET = !_1.2;
_4 = 97_isize as f32;
_2 = _3;
Call(_9 = fn16(_2, _1.3, _3, _1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_5 = 2_usize as i128;
_14 = _8 as f64;
_1.3 = _2;
_14 = 400504980_u32 as f64;
_2 = !_3;
_10 = [368257436_u32,2803716158_u32,3763947808_u32,3261884829_u32];
_5 = (-123_i8) as i128;
RET = _1.2;
_16 = 9223372036854775807_isize;
Goto(bb6)
}
bb6 = {
_17 = 879226370_i32;
_9 = _10;
Call(_1.1 = core::intrinsics::transmute(_16), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_15 = _1.1 as i64;
_19 = '\u{10574}';
_1.2 = RET;
_2 = _1.3 > _1.3;
_15 = -4032974337295407930_i64;
RET = (-128_i8) as u16;
_7 = [_19,_19];
_2 = !_3;
_16 = 9223372036854775807_isize >> _1.2;
_1.1 = _1.2 as u64;
_1.2 = 30_u8 as u16;
_17 = (-1852365172_i32);
_5 = -_8;
_14 = 13074271834281471008_usize as f64;
_1.0 = [_17,_17,_17,_17];
_1.1 = !6808678426580408683_u64;
_19 = '\u{8f177}';
RET = _1.2 & _1.2;
match _17 {
0 => bb6,
1 => bb5,
340282366920938463463374607429915846284 => bb8,
_ => bb3
}
}
bb8 = {
_1.2 = RET;
_18 = _2 as usize;
_10 = [832663145_u32,1988758077_u32,2697499583_u32,3971407292_u32];
_16 = 7567_i16 as isize;
_10 = [1741031004_u32,4099074507_u32,696559285_u32,1651380534_u32];
_14 = _1.2 as f64;
_8 = 336955167924862430931837019408397372156_u128 as i128;
_20 = _4 as i64;
_10 = _9;
RET = _14 as u16;
_20 = 10_i8 as i64;
_14 = _4 as f64;
_19 = '\u{2e18a}';
_1.0 = [_17,_17,_17,_17];
_2 = _1.3;
_21 = _14 - _14;
_8 = !_5;
_9 = [150568400_u32,2075800210_u32,3293159495_u32,627165164_u32];
_18 = 5023142756379686274_usize;
RET = 6_u8 as u16;
_5 = _8 - _8;
_20 = 151303354252371938263212625410260208911_u128 as i64;
_1.3 = _3;
_14 = (-27038_i16) as f64;
_20 = !_15;
Goto(bb9)
}
bb9 = {
_1.2 = RET;
_11 = [_15,_15,_15,_15,_20,_15,_15,_20];
_15 = 2314934878_u32 as i64;
_5 = _8 + _8;
_19 = '\u{27c63}';
_5 = _8 - _8;
_1.1 = 13884778144407611666_u64;
_1.3 = _2;
_22 = [_1.1,_1.1,_1.1];
_22 = [_1.1,_1.1,_1.1];
_1.0 = [_17,_17,_17,_17];
_1.1 = 4551594602860145625_u64 >> _16;
_4 = RET as f32;
_19 = '\u{e4faf}';
_1.1 = 13603360982765159248_u64 << RET;
_2 = _1.3;
_20 = _15;
_10 = [182821405_u32,458431394_u32,583314973_u32,1330685119_u32];
Goto(bb10)
}
bb10 = {
_1.1 = !8795035464144978277_u64;
_14 = -_21;
RET = _18 as u16;
_1.0 = [_17,_17,_17,_17];
_3 = _2 ^ _1.3;
_23 = -_4;
_1.0 = [_17,_17,_17,_17];
_5 = -_8;
_24 = _23 - _4;
_11 = [_15,_15,_20,_15,_20,_15,_15,_20];
_8 = _5 | _5;
RET = _1.2;
_9 = [3429081869_u32,2859221089_u32,2597519063_u32,1647369545_u32];
_15 = 6_i8 as i64;
_5 = -_8;
_25 = -_24;
_19 = '\u{40200}';
RET = _5 as u16;
_17 = (-1894190860_i32);
Goto(bb11)
}
bb11 = {
_18 = _1.1 as usize;
_8 = -_5;
_24 = -_4;
_19 = '\u{e7db6}';
_3 = _1.3 > _2;
_7 = [_19,_19];
_2 = _1.3 <= _3;
_10 = _9;
_21 = -_14;
match _17 {
0 => bb1,
340282366920938463463374607429874020596 => bb13,
_ => bb12
}
}
bb12 = {
RET = !_1.2;
_2 = _3;
_1.3 = _2;
_4 = _5 as f32;
_7 = ['\u{5bd52}','\u{ce86f}'];
_1.3 = !_3;
_4 = _5 as f32;
RET = (-9223372036854775808_isize) as u16;
_8 = -_5;
_1.2 = 77_i8 as u16;
_3 = _2;
RET = _5 as u16;
_4 = _1.2 as f32;
_7 = ['\u{5755b}','\u{10d535}'];
RET = _1.2;
_10 = [651309980_u32,886276250_u32,1369085811_u32,478059970_u32];
RET = _1.2 & _1.2;
_9 = [4213050797_u32,2152199148_u32,2917276247_u32,1013028449_u32];
_3 = _1.3 & _2;
_4 = 0_usize as f32;
_1.0 = [2095088134_i32,1991991293_i32,197179606_i32,661042592_i32];
_9 = [3931004852_u32,2555946859_u32,450563590_u32,2037198692_u32];
_10 = [2513882615_u32,725463939_u32,1945356037_u32,1337197741_u32];
Call(_9 = fn14(_3, _2, _1.3, _3, _3, _2, _1.3), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_26 = _8 as isize;
_19 = '\u{fc55f}';
_23 = -_25;
_23 = _25 + _25;
match _17 {
0 => bb6,
1 => bb4,
340282366920938463463374607429874020596 => bb15,
_ => bb14
}
}
bb14 = {
_1.1 = !8795035464144978277_u64;
_14 = -_21;
RET = _18 as u16;
_1.0 = [_17,_17,_17,_17];
_3 = _2 ^ _1.3;
_23 = -_4;
_1.0 = [_17,_17,_17,_17];
_5 = -_8;
_24 = _23 - _4;
_11 = [_15,_15,_20,_15,_20,_15,_15,_20];
_8 = _5 | _5;
RET = _1.2;
_9 = [3429081869_u32,2859221089_u32,2597519063_u32,1647369545_u32];
_15 = 6_i8 as i64;
_5 = -_8;
_25 = -_24;
_19 = '\u{40200}';
RET = _5 as u16;
_17 = (-1894190860_i32);
Goto(bb11)
}
bb15 = {
_5 = _8 & _8;
_5 = -_8;
_18 = !2_usize;
_3 = !_2;
_4 = 114_u8 as f32;
_14 = -_21;
_22 = [_1.1,_1.1,_1.1];
_25 = 88466320706912839793963082000577047507_u128 as f32;
_16 = _26;
_25 = _24;
_17 = (-446086899_i32) ^ (-994997426_i32);
_30.0.0 = [79_i8,(-115_i8),42_i8];
_26 = -_16;
Goto(bb16)
}
bb16 = {
Call(_31 = dump_var(13_usize, 20_usize, Move(_20), 19_usize, Move(_19), 11_usize, Move(_11), 26_usize, Move(_26)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(13_usize, 22_usize, Move(_22), 18_usize, Move(_18), 3_usize, Move(_3), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool) -> [u32; 4] {
mir! {
type RET = [u32; 4];
let _8: bool;
let _9: [char; 2];
let _10: char;
let _11: ([i8; 3],);
let _12: (u32, u32, *const isize);
let _13: Adt42;
let _14: ([i8; 3],);
let _15: u128;
let _16: isize;
let _17: (char, *const u64, isize, ([u32; 4], char, u64));
let _18: *const u64;
let _19: isize;
let _20: ([i32; 4], u64, u16, bool);
let _21: Adt48;
let _22: (i128, i128, u64, u32);
let _23: f64;
let _24: Adt51;
let _25: [i32; 4];
let _26: u64;
let _27: *mut &'static u32;
let _28: f64;
let _29: ();
let _30: ();
{
_2 = _1 < _7;
RET = [1742396514_u32,516198384_u32,1601702938_u32,3332827875_u32];
RET = [2248095337_u32,2871658378_u32,566867058_u32,4267407753_u32];
_4 = _2 | _2;
RET = [300051749_u32,534813443_u32,1771737949_u32,3012678168_u32];
_5 = !_7;
_7 = _1 <= _1;
_7 = _3 | _4;
_5 = !_4;
_1 = _6;
_5 = _1;
_2 = !_5;
_5 = !_7;
_5 = _2;
_4 = _1 >= _2;
Goto(bb1)
}
bb1 = {
_9 = ['\u{fab8f}','\u{37c92}'];
RET = [3185968114_u32,3530991217_u32,2682370880_u32,1181180453_u32];
_7 = !_1;
_3 = !_2;
_6 = _5 != _2;
RET = [246924686_u32,1319009107_u32,543226017_u32,3608679436_u32];
_4 = _7;
Call(_3 = fn15(RET, _1, _7, _4, _6, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = !_2;
_2 = !_1;
_11.0 = [21_i8,(-24_i8),73_i8];
_9 = ['\u{f4acd}','\u{89c88}'];
_4 = _5;
_5 = _2;
RET = [4085572827_u32,3975387641_u32,2394575709_u32,2114440800_u32];
_7 = _1;
_8 = !_2;
_11.0 = [(-60_i8),(-71_i8),(-109_i8)];
_12.1 = 1594503976_u32 * 2895997681_u32;
_10 = '\u{51f37}';
RET = [_12.1,_12.1,_12.1,_12.1];
_16 = 9223372036854775807_isize ^ 9223372036854775807_isize;
Goto(bb3)
}
bb3 = {
_2 = _4;
RET = [_12.1,_12.1,_12.1,_12.1];
_9 = [_10,_10];
_12.0 = (-28375_i16) as u32;
_5 = _1;
_14 = (_11.0,);
_12.1 = _12.0;
_2 = _7;
_17.0 = _10;
_17.1 = core::ptr::addr_of!(_17.3.2);
_11.0 = [(-71_i8),25_i8,(-104_i8)];
_17.2 = 21858842224751311397997379606932765225_i128 as isize;
_17.3.1 = _17.0;
_12.0 = _12.1;
_8 = !_4;
_17.3 = (RET, _10, 17301355886396959705_u64);
_1 = _6;
_16 = -_17.2;
RET = [_12.0,_12.1,_12.1,_12.0];
_6 = !_4;
match _17.3.2 {
0 => bb1,
17301355886396959705 => bb4,
_ => bb2
}
}
bb4 = {
_2 = _8;
RET = _17.3.0;
_18 = core::ptr::addr_of!(_20.1);
_17.3.1 = _10;
_17.1 = core::ptr::addr_of!(_20.1);
_20.1 = !_17.3.2;
_6 = _3;
Goto(bb5)
}
bb5 = {
_14 = (_11.0,);
_18 = _17.1;
_15 = !117461835587031391627281187606549441506_u128;
_2 = !_3;
_12.1 = !_12.0;
_17.3.0 = [_12.1,_12.0,_12.1,_12.1];
_21.fld7.3 = _3;
_9 = [_17.3.1,_17.0];
_20.0 = [(-806987046_i32),(-1351134451_i32),(-662583935_i32),(-1183664269_i32)];
_20.3 = _3 | _2;
match _17.3.2 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
17301355886396959705 => bb14,
_ => bb13
}
}
bb6 = {
_2 = _8;
RET = _17.3.0;
_18 = core::ptr::addr_of!(_20.1);
_17.3.1 = _10;
_17.1 = core::ptr::addr_of!(_20.1);
_20.1 = !_17.3.2;
_6 = _3;
Goto(bb5)
}
bb7 = {
_2 = _4;
RET = [_12.1,_12.1,_12.1,_12.1];
_9 = [_10,_10];
_12.0 = (-28375_i16) as u32;
_5 = _1;
_14 = (_11.0,);
_12.1 = _12.0;
_2 = _7;
_17.0 = _10;
_17.1 = core::ptr::addr_of!(_17.3.2);
_11.0 = [(-71_i8),25_i8,(-104_i8)];
_17.2 = 21858842224751311397997379606932765225_i128 as isize;
_17.3.1 = _17.0;
_12.0 = _12.1;
_8 = !_4;
_17.3 = (RET, _10, 17301355886396959705_u64);
_1 = _6;
_16 = -_17.2;
RET = [_12.0,_12.1,_12.1,_12.0];
_6 = !_4;
match _17.3.2 {
0 => bb1,
17301355886396959705 => bb4,
_ => bb2
}
}
bb8 = {
_1 = !_2;
_2 = !_1;
_11.0 = [21_i8,(-24_i8),73_i8];
_9 = ['\u{f4acd}','\u{89c88}'];
_4 = _5;
_5 = _2;
RET = [4085572827_u32,3975387641_u32,2394575709_u32,2114440800_u32];
_7 = _1;
_8 = !_2;
_11.0 = [(-60_i8),(-71_i8),(-109_i8)];
_12.1 = 1594503976_u32 * 2895997681_u32;
_10 = '\u{51f37}';
RET = [_12.1,_12.1,_12.1,_12.1];
_16 = 9223372036854775807_isize ^ 9223372036854775807_isize;
Goto(bb3)
}
bb9 = {
_9 = ['\u{fab8f}','\u{37c92}'];
RET = [3185968114_u32,3530991217_u32,2682370880_u32,1181180453_u32];
_7 = !_1;
_3 = !_2;
_6 = _5 != _2;
RET = [246924686_u32,1319009107_u32,543226017_u32,3608679436_u32];
_4 = _7;
Call(_3 = fn15(RET, _1, _7, _4, _6, _6), ReturnTo(bb2), UnwindUnreachable())
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
_21.fld5.0.0 = _11.0;
_22.3 = _12.1;
_13 = Adt42::Variant0 { fld0: _17.3,fld1: _10 };
_17.3.0 = RET;
_17.0 = _10;
place!(Field::<char>(Variant(_13, 0), 1)) = _10;
_14.0 = _11.0;
_9 = [Field::<char>(Variant(_13, 0), 1),Field::<([u32; 4], char, u64)>(Variant(_13, 0), 0).1];
SetDiscriminant(_13, 0);
place!(Field::<([u32; 4], char, u64)>(Variant(_13, 0), 0)).1 = _17.0;
place!(Field::<([u32; 4], char, u64)>(Variant(_13, 0), 0)).2 = 115_i8 as u64;
_20.3 = _7 > _7;
_22.3 = _12.1 | _12.0;
_9 = [_10,_17.3.1];
_21.fld7.3 = _3 | _5;
_6 = !_4;
_12.0 = _22.3 | _12.1;
_19 = _17.0 as isize;
place!(Field::<([u32; 4], char, u64)>(Variant(_13, 0), 0)).1 = _17.3.1;
_21.fld5.2 = (-2187_i16) as f32;
_8 = !_3;
_2 = !_20.3;
place!(Field::<([u32; 4], char, u64)>(Variant(_13, 0), 0)) = (RET, _10, _20.1);
_17.3.1 = _17.0;
_8 = _21.fld7.3 < _3;
_12.1 = 66_i8 as u32;
place!(Field::<([u32; 4], char, u64)>(Variant(_13, 0), 0)).0 = [_22.3,_12.0,_12.0,_12.0];
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(14_usize, 16_usize, Move(_16), 3_usize, Move(_3), 4_usize, Move(_4), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(14_usize, 10_usize, Move(_10), 1_usize, Move(_1), 8_usize, Move(_8), 30_usize, _30), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: [u32; 4],mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool) -> bool {
mir! {
type RET = bool;
let _7: [i8; 3];
let _8: isize;
let _9: Adt44;
let _10: f32;
let _11: (([i8; 3],), ([i8; 3],), f32, usize);
let _12: Adt42;
let _13: ();
let _14: ();
{
_1 = [2510826090_u32,3905787941_u32,360467718_u32,1433450318_u32];
_2 = _5 > _3;
_4 = _3 == _3;
_6 = !_3;
_10 = 3697866798788045394_i64 as f32;
_5 = _2;
_7 = [73_i8,(-91_i8),115_i8];
_10 = (-17041_i16) as f32;
_11.3 = 5_usize + 6_usize;
_11.3 = !3_usize;
_8 = _3 as isize;
RET = _5 & _2;
_11.1 = (_7,);
RET = _5;
_8 = 119_isize;
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(15_usize, 6_usize, Move(_6), 4_usize, Move(_4), 8_usize, Move(_8), 1_usize, Move(_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: ([i32; 4], u64, u16, bool)) -> [u32; 4] {
mir! {
type RET = [u32; 4];
let _5: f64;
let _6: [u32; 4];
let _7: [char; 2];
let _8: [u64; 3];
let _9: ([u32; 4], char, u64);
let _10: [i8; 3];
let _11: Adt55;
let _12: usize;
let _13: Adt51;
let _14: f64;
let _15: f32;
let _16: Adt53;
let _17: i32;
let _18: Adt44;
let _19: Adt54;
let _20: i16;
let _21: Adt46;
let _22: bool;
let _23: char;
let _24: &'static u32;
let _25: f32;
let _26: bool;
let _27: isize;
let _28: f64;
let _29: f32;
let _30: bool;
let _31: f64;
let _32: *mut &'static u32;
let _33: u16;
let _34: ([i32; 4], u64, u16, bool);
let _35: char;
let _36: ([i8; 3],);
let _37: ();
let _38: ();
{
_3 = _4.3;
_2 = !_3;
_2 = _1 < _1;
_4.3 = _2 | _1;
_1 = !_3;
_4.1 = 3738114708372070539_u64;
RET = [2731554822_u32,3709663639_u32,2808231974_u32,2692380932_u32];
_3 = !_4.3;
_4.0 = [(-1549121214_i32),(-220110419_i32),1862243908_i32,(-1375430950_i32)];
_1 = !_2;
_5 = 95979213517351895588171351371690236594_i128 as f64;
_4.3 = !_3;
_1 = _4.3;
_4.2 = 17220_u16 & 60740_u16;
_6 = RET;
_7 = ['\u{3bdab}','\u{67a91}'];
_8 = [_4.1,_4.1,_4.1];
_4.1 = (-9223372036854775808_isize) as u64;
_4.0 = [(-1508539124_i32),278399880_i32,441157351_i32,(-577131244_i32)];
_4.1 = (-12943_i16) as u64;
RET = [436526138_u32,3437017835_u32,158547896_u32,2003415544_u32];
_1 = !_4.3;
_4.3 = _3;
_4.0 = [(-1326621095_i32),(-586797400_i32),(-884762267_i32),319739106_i32];
_8 = [_4.1,_4.1,_4.1];
_3 = _2;
_7 = ['\u{23e4e}','\u{8dbfa}'];
_2 = _4.3;
Call(_3 = fn17(_1, _2, _4, _2, _2, _4, _2, _1, _8, _2, _4.3, _1, _1, _4.3, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4.3 = _2;
_2 = _4.3;
RET = [418306640_u32,2239295073_u32,2547558352_u32,853577480_u32];
_9.0 = [86726607_u32,3307227190_u32,2545244104_u32,2854122929_u32];
_4.2 = !61239_u16;
_4.1 = 1_usize as u64;
Goto(bb2)
}
bb2 = {
_5 = 247042426687733004368490113147469194048_u128 as f64;
_8 = [_4.1,_4.1,_4.1];
_2 = _1;
_3 = _2 != _2;
_2 = _4.3;
_9 = (_6, '\u{67f8}', _4.1);
_4.3 = !_1;
_4.1 = 200332884135092772574281767419530208829_u128 as u64;
_9.1 = '\u{10ea5b}';
_4.0 = [1896100663_i32,567265758_i32,(-774680485_i32),332801884_i32];
_6 = RET;
_6 = [2517286183_u32,2940326210_u32,1150359453_u32,642945964_u32];
_1 = !_4.3;
_7 = [_9.1,_9.1];
_3 = !_4.3;
_12 = 5_usize ^ 1_usize;
_7 = [_9.1,_9.1];
_4.2 = 9849_u16 >> _4.1;
_10 = [(-2_i8),(-52_i8),(-76_i8)];
_2 = _4.3;
_12 = 11426919773611846088_usize * 2_usize;
RET = [1206889470_u32,1743089870_u32,800519694_u32,2702936871_u32];
_9.0 = [1656574837_u32,704305682_u32,1023789376_u32,2506138966_u32];
_9.1 = '\u{e86d9}';
_4.3 = _2 & _2;
_9 = (_6, '\u{a2476}', _4.1);
_7 = [_9.1,_9.1];
_4.2 = 50354_u16 & 40254_u16;
Goto(bb3)
}
bb3 = {
_4.3 = _2;
_3 = _2 >= _2;
_7 = [_9.1,_9.1];
_7 = [_9.1,_9.1];
_5 = (-8699_i16) as f64;
_9.2 = 9223372036854775807_isize as u64;
_1 = _2 & _3;
_9.0 = [3562493077_u32,1796556533_u32,4246504695_u32,2624150862_u32];
_10 = [(-51_i8),15_i8,82_i8];
_6 = [2304450496_u32,3944590679_u32,875771163_u32,3912597645_u32];
_16.fld4 = (-1023239526190854072_i64) as f32;
_2 = _4.3;
_16.fld1.fld7.0 = [647994752_i32,268685501_i32,1832930565_i32,(-779446137_i32)];
_12 = !5063119487550839555_usize;
_16.fld2.3.1 = _9.1;
_16.fld1.fld5.1.0 = _10;
_16.fld2.3.1 = _9.1;
Goto(bb4)
}
bb4 = {
_3 = _1;
_14 = -_5;
_3 = _1 <= _4.3;
_16.fld2.1 = core::ptr::addr_of!(_9.2);
_16.fld1.fld5.0 = (_16.fld1.fld5.1.0,);
_5 = _14;
_16.fld3 = core::ptr::addr_of_mut!(_16.fld2.1);
_16.fld2.0 = _16.fld2.3.1;
_16.fld1.fld7.2 = _4.2 | _4.2;
_15 = (-105671255970107343948279261111817879256_i128) as f32;
_16.fld1.fld5.0.0 = [(-18_i8),(-98_i8),(-119_i8)];
Call(_16.fld1.fld5.1.0 = fn19(_4, _1, _3, _3, _4.3, _4.3, _3, _16.fld2.0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_16.fld1.fld5.1 = (_10,);
_16.fld1.fld7 = _4;
_2 = _16.fld1.fld7.3;
_16.fld5 = _16.fld2.1;
_16.fld1.fld7.3 = _3 <= _3;
_3 = !_16.fld1.fld7.3;
_16.fld1.fld7.0 = _4.0;
_16.fld2.3.1 = _16.fld2.0;
_9.0 = [2125069322_u32,3401418001_u32,1367079137_u32,585839414_u32];
_16.fld1.fld7 = (_4.0, _4.1, _4.2, _1);
_16.fld1.fld5.2 = 319897240168194285_i64 as f32;
_16.fld1.fld7.2 = !_4.2;
_16.fld1.fld5.0 = _16.fld1.fld5.1;
_16.fld1.fld3 = Adt46::Variant3 { fld0: _6,fld1: 1352088362151365079_i64 };
_6 = [2892310891_u32,608282416_u32,3959496495_u32,2073703931_u32];
_20 = 72_u8 as i16;
_16.fld2 = (_9.1, _16.fld5, 9223372036854775807_isize, _9);
_4 = (_16.fld1.fld7.0, _16.fld1.fld7.1, _16.fld1.fld7.2, _3);
_16.fld5 = _16.fld2.1;
_16.fld2.2 = (-9223372036854775808_isize) + 67_isize;
_4.1 = _16.fld2.3.2;
_23 = _16.fld2.3.1;
_16.fld1.fld7.1 = _16.fld2.3.2;
_16.fld2 = (_23, _16.fld5, 9223372036854775807_isize, _9);
_12 = 311298053422669997860970348731221634309_u128 as usize;
_6 = _16.fld2.3.0;
_19 = Adt54::Variant0 { fld0: _16.fld1.fld5.1,fld1: 2638097516_u32 };
Goto(bb6)
}
bb6 = {
place!(Field::<([i8; 3],)>(Variant(_19, 0), 0)).0 = [89_i8,67_i8,(-63_i8)];
_16.fld1.fld0 = Adt40::Variant1 { fld0: _16.fld3,fld1: _16.fld2.1 };
_16.fld1.fld3 = Adt46::Variant3 { fld0: RET,fld1: (-4605462905574478483_i64) };
_22 = _16.fld1.fld7.3 ^ _2;
_16.fld1.fld7 = _4;
_16.fld1.fld7.3 = _4.3 >= _22;
Goto(bb7)
}
bb7 = {
_16.fld1.fld7.3 = _4.3;
_16.fld1.fld5.1 = (Field::<([i8; 3],)>(Variant(_19, 0), 0).0,);
_16.fld1.fld7.1 = (-149254767_i32) as u64;
SetDiscriminant(_16.fld1.fld0, 0);
Goto(bb8)
}
bb8 = {
_10 = [45_i8,(-50_i8),(-38_i8)];
place!(Field::<i64>(Variant(_16.fld1.fld3, 3), 1)) = !(-5566677206857404842_i64);
_25 = _15;
place!(Field::<([i8; 3],)>(Variant(_19, 0), 0)).0 = [8_i8,89_i8,(-41_i8)];
_16.fld1.fld5 = (Field::<([i8; 3],)>(Variant(_19, 0), 0), Field::<([i8; 3],)>(Variant(_19, 0), 0), _25, _12);
_16.fld1.fld1 = _16.fld2.0;
_16.fld1.fld5.0.0 = [56_i8,(-17_i8),68_i8];
_16.fld2.3 = (_6, _23, _4.1);
_16.fld2.3.1 = _23;
_26 = !_16.fld1.fld7.3;
_17 = 1766296219_i32;
_4.1 = _16.fld2.3.2;
place!(Field::<char>(Variant(_16.fld1.fld0, 0), 1)) = _16.fld2.3.1;
place!(Field::<*const [u32; 4]>(Variant(_16.fld1.fld0, 0), 0)) = core::ptr::addr_of!(RET);
_20 = -23972_i16;
SetDiscriminant(_16.fld1.fld3, 3);
place!(Field::<i64>(Variant(_16.fld1.fld3, 3), 1)) = 3679182662384525610_i64 ^ (-4140297567609562803_i64);
_20 = !(-18503_i16);
_27 = Field::<i64>(Variant(_16.fld1.fld3, 3), 1) as isize;
_12 = _16.fld1.fld5.3 + _16.fld1.fld5.3;
_16.fld1.fld5.2 = _15 * _15;
_16.fld1.fld5.2 = -_16.fld4;
_26 = !_1;
_27 = _16.fld2.2;
_16.fld1.fld5.1.0 = Field::<([i8; 3],)>(Variant(_19, 0), 0).0;
match _17 {
0 => bb6,
1766296219 => bb10,
_ => bb9
}
}
bb9 = {
_16.fld1.fld5.1 = (_10,);
_16.fld1.fld7 = _4;
_2 = _16.fld1.fld7.3;
_16.fld5 = _16.fld2.1;
_16.fld1.fld7.3 = _3 <= _3;
_3 = !_16.fld1.fld7.3;
_16.fld1.fld7.0 = _4.0;
_16.fld2.3.1 = _16.fld2.0;
_9.0 = [2125069322_u32,3401418001_u32,1367079137_u32,585839414_u32];
_16.fld1.fld7 = (_4.0, _4.1, _4.2, _1);
_16.fld1.fld5.2 = 319897240168194285_i64 as f32;
_16.fld1.fld7.2 = !_4.2;
_16.fld1.fld5.0 = _16.fld1.fld5.1;
_16.fld1.fld3 = Adt46::Variant3 { fld0: _6,fld1: 1352088362151365079_i64 };
_6 = [2892310891_u32,608282416_u32,3959496495_u32,2073703931_u32];
_20 = 72_u8 as i16;
_16.fld2 = (_9.1, _16.fld5, 9223372036854775807_isize, _9);
_4 = (_16.fld1.fld7.0, _16.fld1.fld7.1, _16.fld1.fld7.2, _3);
_16.fld5 = _16.fld2.1;
_16.fld2.2 = (-9223372036854775808_isize) + 67_isize;
_4.1 = _16.fld2.3.2;
_23 = _16.fld2.3.1;
_16.fld1.fld7.1 = _16.fld2.3.2;
_16.fld2 = (_23, _16.fld5, 9223372036854775807_isize, _9);
_12 = 311298053422669997860970348731221634309_u128 as usize;
_6 = _16.fld2.3.0;
_19 = Adt54::Variant0 { fld0: _16.fld1.fld5.1,fld1: 2638097516_u32 };
Goto(bb6)
}
bb10 = {
_8 = [_4.1,_16.fld2.3.2,_9.2];
_24 = &place!(Field::<u32>(Variant(_19, 0), 1));
Goto(bb11)
}
bb11 = {
_19 = Adt54::Variant0 { fld0: _16.fld1.fld5.0,fld1: 4174428225_u32 };
_16.fld1.fld5.1.0 = [(-52_i8),29_i8,(-41_i8)];
_16.fld1.fld7.3 = _1;
_21 = Adt46::Variant0 { fld0: 1451682663_u32,fld1: _16.fld2 };
_4 = _16.fld1.fld7;
_16.fld1.fld3 = Adt46::Variant1 { fld0: _16.fld3,fld1: _16.fld1.fld5 };
place!(Field::<f64>(Variant(_16.fld1.fld0, 0), 2)) = _14;
_16.fld1.fld7.1 = _3 as u64;
place!(Field::<*const [u32; 4]>(Variant(_16.fld1.fld0, 0), 0)) = core::ptr::addr_of!(_9.0);
_9.0 = [3384430296_u32,2087947591_u32,2933897585_u32,592764207_u32];
_16.fld2.3 = Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_21, 0), 1).3;
_9.2 = !_16.fld1.fld7.1;
_23 = Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_21, 0), 1).0;
_16.fld1.fld3 = Adt46::Variant0 { fld0: 36483839_u32,fld1: Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_21, 0), 1) };
_4.2 = !_16.fld1.fld7.2;
_4 = (_16.fld1.fld7.0, _9.2, _16.fld1.fld7.2, _26);
_16.fld6 = Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_16.fld1.fld3, 0), 1).3.0;
_16.fld2 = (_16.fld1.fld1, Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_16.fld1.fld3, 0), 1).1, _27, Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_16.fld1.fld3, 0), 1).3);
_32 = core::ptr::addr_of_mut!(_24);
place!(Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_16.fld1.fld3, 0), 1)).2 = -_16.fld2.2;
Call(_33 = core::intrinsics::bswap(_16.fld1.fld7.2), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_32 = core::ptr::addr_of_mut!((*_32));
_4.3 = !_1;
_29 = -_25;
place!(Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_21, 0), 1)).2 = 18_i8 as isize;
_20 = 4158330256_u32 as i16;
_6 = [2174121435_u32,1286679012_u32,2058047976_u32,1338311541_u32];
_8 = [_9.2,_4.1,_16.fld1.fld7.1];
_7 = [Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_21, 0), 1).3.1,Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_16.fld1.fld3, 0), 1).0];
place!(Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_21, 0), 1)).3.1 = _16.fld1.fld1;
place!(Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_16.fld1.fld3, 0), 1)) = _16.fld2;
Goto(bb13)
}
bb13 = {
_16.fld1.fld5.0 = (_10,);
_16.fld2.2 = _27 * Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_16.fld1.fld3, 0), 1).2;
_16.fld2 = (Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_21, 0), 1).3.1, _16.fld5, Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_16.fld1.fld3, 0), 1).2, _9);
_31 = _5;
place!(Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_21, 0), 1)).3.2 = _20 as u64;
_16.fld2.2 = Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_16.fld1.fld3, 0), 1).2 << _4.1;
_9 = (RET, Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_16.fld1.fld3, 0), 1).0, _16.fld1.fld7.1);
_30 = _9.2 < _4.1;
match _27 {
9223372036854775807 => bb14,
_ => bb9
}
}
bb14 = {
place!(Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_21, 0), 1)).2 = !_16.fld2.2;
_28 = 43301226338346050236348165315554081371_i128 as f64;
_16.fld1.fld5.1.0 = [116_i8,108_i8,80_i8];
_34.2 = 241_u8 as u16;
_16.fld2.2 = Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_21, 0), 1).2 + Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_21, 0), 1).2;
_7 = [Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_21, 0), 1).0,_16.fld2.0];
_21 = Adt46::Variant0 { fld0: 3800008012_u32,fld1: Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_16.fld1.fld3, 0), 1) };
place!(Field::<u32>(Variant(_21, 0), 0)) = 2313046393_u32;
place!(Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_21, 0), 1)).0 = Field::<(char, *const u64, isize, ([u32; 4], char, u64))>(Variant(_21, 0), 1).3.1;
_16.fld2.1 = _16.fld5;
_4 = (_16.fld1.fld7.0, _16.fld1.fld7.1, _34.2, _30);
_5 = _28;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(16_usize, 8_usize, Move(_8), 17_usize, Move(_17), 23_usize, Move(_23), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(16_usize, 12_usize, Move(_12), 6_usize, Move(_6), 20_usize, Move(_20), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(16_usize, 27_usize, Move(_27), 38_usize, _38, 38_usize, _38, 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: bool,mut _2: bool,mut _3: ([i32; 4], u64, u16, bool),mut _4: bool,mut _5: bool,mut _6: ([i32; 4], u64, u16, bool),mut _7: bool,mut _8: bool,mut _9: [u64; 3],mut _10: bool,mut _11: bool,mut _12: bool,mut _13: bool,mut _14: bool,mut _15: bool) -> bool {
mir! {
type RET = bool;
let _16: (([i8; 3],), ([i8; 3],), f32, usize);
let _17: *mut *const u64;
let _18: Adt52;
let _19: u64;
let _20: char;
let _21: (char, *const u64, isize, ([u32; 4], char, u64));
let _22: f64;
let _23: f32;
let _24: [i64; 8];
let _25: ([i32; 4], u64, u16, bool);
let _26: bool;
let _27: char;
let _28: u32;
let _29: [i8; 3];
let _30: bool;
let _31: u8;
let _32: Adt46;
let _33: [char; 2];
let _34: isize;
let _35: [i8; 3];
let _36: ();
let _37: ();
{
_16.2 = _6.2 as f32;
_16.0.0 = [30_i8,(-12_i8),80_i8];
_2 = !_3.3;
_3 = (_6.0, _6.1, _6.2, _1);
_6.2 = _1 as u16;
_18.fld0 = 6_usize;
_3.1 = !_6.1;
_6.2 = _3.2 >> _3.2;
Call(_18.fld1.1 = fn18(_14, _3, _7, _10, _10), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_16.1 = (_16.0.0,);
_16.1.0 = [(-46_i8),127_i8,3_i8];
_5 = !_15;
_2 = !_15;
_16.2 = (-112_i8) as f32;
_3.2 = _6.2 | _6.2;
_18.fld1.3.1 = '\u{c2a1f}';
_18.fld1.3.2 = !_3.1;
_18.fld1.3.0 = [3074465347_u32,1217462807_u32,1001277188_u32,2202575473_u32];
_18.fld0 = (-29_i8) as usize;
_20 = _18.fld1.3.1;
_18.fld1.0 = _20;
_16.1 = (_16.0.0,);
_18.fld1.3.2 = (-3167987822752694449_i64) as u64;
_3.0 = [1189948208_i32,(-346216648_i32),1043696640_i32,539940918_i32];
_3.3 = !_14;
_17 = core::ptr::addr_of_mut!(_18.fld1.1);
_11 = !_4;
_9 = [_18.fld1.3.2,_18.fld1.3.2,_3.1];
_18.fld1.2 = 42_isize;
_16.3 = _18.fld0 | _18.fld0;
_2 = _7;
_3 = _6;
_15 = !_14;
_6.2 = _3.2 ^ _3.2;
_3.3 = _13;
match _18.fld1.2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
42 => bb9,
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
(*_17) = core::ptr::addr_of!(_6.1);
_16.0 = (_16.1.0,);
_6.1 = _18.fld1.3.2;
_21.0 = _18.fld1.3.1;
_19 = _18.fld1.0 as u64;
_20 = _18.fld1.3.1;
_11 = _15 < _4;
_10 = _4 != _11;
_18.fld1.3.0 = [4123314144_u32,4086483193_u32,337717071_u32,4174388432_u32];
_3.0 = _6.0;
_11 = _7;
_16.2 = 11_u8 as f32;
_17 = core::ptr::addr_of_mut!((*_17));
_18.fld1.3.2 = !_19;
_21.2 = _18.fld1.2;
_21.3 = (_18.fld1.3.0, _20, _3.1);
_3.1 = _16.3 as u64;
_9 = [_18.fld1.3.2,_3.1,_3.1];
(*_17) = core::ptr::addr_of!(_18.fld1.3.2);
_16.1.0 = _16.0.0;
_3.1 = _19 ^ _19;
_18.fld1.2 = _21.2 * _21.2;
_18.fld0 = 166_u8 as usize;
_25.2 = _6.2 & _3.2;
_18.fld1.3.0 = _21.3.0;
match _21.2 {
0 => bb4,
42 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_25.1 = 123303319782945087543346412679983854937_i128 as u64;
RET = _6.3 >= _2;
_20 = _21.3.1;
(*_17) = core::ptr::addr_of!(_25.1);
_4 = !_10;
_21 = (_18.fld1.3.1, (*_17), _18.fld1.2, _18.fld1.3);
_21.3.0 = [4005345837_u32,852452739_u32,3833853652_u32,3315308358_u32];
_10 = _2 ^ _2;
_13 = _10;
_11 = _10;
_25 = (_6.0, _3.1, _6.2, _6.3);
_27 = _18.fld1.3.1;
_6.2 = _21.3.1 as u16;
_29 = [(-111_i8),(-26_i8),25_i8];
_18.fld0 = _16.3;
_16.0.0 = [44_i8,87_i8,(-27_i8)];
_19 = !_18.fld1.3.2;
_18.fld1.0 = _27;
_18.fld0 = _16.3 + _16.3;
_18.fld1.0 = _27;
_25 = (_6.0, _3.1, _6.2, _6.3);
_21.2 = _18.fld1.2;
_18.fld1 = (_21.3.1, _21.1, _21.2, _21.3);
_18 = Adt52 { fld0: _16.3,fld1: _21,fld2: 78_u8 };
Goto(bb12)
}
bb12 = {
Call(_36 = dump_var(17_usize, 7_usize, Move(_7), 13_usize, Move(_13), 2_usize, Move(_2), 27_usize, Move(_27)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_36 = dump_var(17_usize, 29_usize, Move(_29), 10_usize, Move(_10), 12_usize, Move(_12), 9_usize, Move(_9)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_36 = dump_var(17_usize, 6_usize, Move(_6), 4_usize, Move(_4), 37_usize, _37, 37_usize, _37), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: bool,mut _2: ([i32; 4], u64, u16, bool),mut _3: bool,mut _4: bool,mut _5: bool) -> *const u64 {
mir! {
type RET = *const u64;
let _6: [i8; 3];
let _7: char;
let _8: isize;
let _9: [u64; 3];
let _10: [i32; 4];
let _11: f32;
let _12: ();
let _13: ();
{
_5 = !_2.3;
_3 = _4;
RET = core::ptr::addr_of!(_2.1);
_6 = [26_i8,(-8_i8),12_i8];
_4 = _5;
_2.2 = !56581_u16;
_6 = [(-31_i8),16_i8,(-35_i8)];
RET = core::ptr::addr_of!(_2.1);
_2.2 = !37443_u16;
_5 = !_2.3;
RET = core::ptr::addr_of!(_2.1);
_6 = [(-76_i8),(-64_i8),52_i8];
(*RET) = 1120811066093931933_u64;
_2.3 = !_1;
Call(_8 = core::intrinsics::transmute((*RET)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2.0 = [1125074060_i32,2116223297_i32,(-1660327927_i32),(-652375884_i32)];
_6 = [(-2_i8),(-8_i8),(-10_i8)];
_7 = '\u{f801e}';
(*RET) = 14575566849662786375_u64 | 8358934250421898547_u64;
_4 = _5 < _5;
_7 = '\u{9828f}';
(*RET) = !3289611726634348021_u64;
(*RET) = !7960384757171025780_u64;
(*RET) = !11558330612700195883_u64;
_2.0 = [798103359_i32,(-479842593_i32),(-1116134266_i32),1829107621_i32];
RET = core::ptr::addr_of!((*RET));
_9 = [(*RET),_2.1,_2.1];
(*RET) = !3913015715686145992_u64;
(*RET) = 15283072931357047601_u64;
_10 = _2.0;
_2.1 = 15372179849571504593_u64;
_2.3 = !_3;
(*RET) = 2_usize as u64;
(*RET) = 265796616977806184_u64 >> _2.2;
_10 = [(-291465879_i32),(-784026325_i32),316348264_i32,(-329967680_i32)];
_2 = (_10, 2610688892191892967_u64, 59239_u16, _5);
RET = core::ptr::addr_of!((*RET));
Goto(bb2)
}
bb2 = {
Call(_12 = dump_var(18_usize, 7_usize, Move(_7), 10_usize, Move(_10), 9_usize, Move(_9), 6_usize, Move(_6)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_12 = dump_var(18_usize, 2_usize, Move(_2), 13_usize, _13, 13_usize, _13, 13_usize, _13), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: ([i32; 4], u64, u16, bool),mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: char) -> [i8; 3] {
mir! {
type RET = [i8; 3];
let _9: char;
let _10: ([u32; 4], char, u64);
let _11: Adt46;
let _12: u128;
let _13: *mut &'static u32;
let _14: Adt55;
let _15: i64;
let _16: Adt56;
let _17: ([u32; 4], char, u64);
let _18: &'static u32;
let _19: u64;
let _20: ([i32; 4], u64, u16, bool);
let _21: u64;
let _22: f32;
let _23: ([u32; 4], char, u64);
let _24: u128;
let _25: Adt45;
let _26: Adt44;
let _27: bool;
let _28: ();
let _29: ();
{
_6 = !_5;
_10.2 = _1.1 + _1.1;
_1.3 = _7;
RET = [123_i8,(-43_i8),(-106_i8)];
_9 = _8;
_5 = _2 | _6;
_1.3 = _3 >= _6;
_2 = !_4;
_8 = _9;
_1.2 = 55395_u16 * 39144_u16;
_10.2 = _1.1 - _1.1;
RET = [82_i8,(-67_i8),(-121_i8)];
_10.1 = _9;
_2 = _3;
_1.1 = !_10.2;
_6 = _3;
_7 = _6;
_2 = _6 == _3;
_4 = !_3;
_10.0 = [2946422806_u32,1744227211_u32,3571002353_u32,195406835_u32];
_10.1 = _9;
_7 = _4;
RET = [(-55_i8),(-81_i8),(-36_i8)];
_7 = _2 > _4;
_2 = _8 == _9;
_6 = _4;
_4 = !_3;
_9 = _10.1;
Goto(bb1)
}
bb1 = {
_11 = Adt46::Variant3 { fld0: _10.0,fld1: 8860397332304529124_i64 };
_7 = !_4;
_12 = 116905420334796858437961830692848063408_u128;
_12 = !215269141932954040634863183204522282207_u128;
_3 = _2;
_1.0 = [1900300374_i32,(-201064011_i32),(-235850784_i32),(-1783770211_i32)];
_7 = _1.3 | _6;
_10.0 = [66178664_u32,2243843990_u32,616610419_u32,2555274231_u32];
_12 = 293817958588420012390033068468094195504_u128;
_12 = (-1618548162_i32) as u128;
_10 = (Field::<[u32; 4]>(Variant(_11, 3), 0), _9, _1.1);
_1.2 = !26655_u16;
place!(Field::<i64>(Variant(_11, 3), 1)) = (-3090864426073539758_i64);
_10 = (Field::<[u32; 4]>(Variant(_11, 3), 0), _9, _1.1);
_6 = _7 <= _5;
_9 = _10.1;
_5 = _7 & _6;
RET = [45_i8,(-123_i8),0_i8];
match Field::<i64>(Variant(_11, 3), 1) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463460283743005694671698 => bb9,
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
RET = [(-32_i8),59_i8,91_i8];
_3 = !_5;
RET = [(-1_i8),(-91_i8),59_i8];
_8 = _10.1;
_10.0 = [1888789450_u32,3967165847_u32,3890405530_u32,3954737593_u32];
_2 = _3 > _4;
_17.1 = _10.1;
_2 = _3;
_10 = (Field::<[u32; 4]>(Variant(_11, 3), 0), _8, _1.1);
_10.1 = _8;
_1.3 = !_6;
_10 = (Field::<[u32; 4]>(Variant(_11, 3), 0), _17.1, _1.1);
_17 = (_10.0, _8, _10.2);
_4 = !_6;
RET = [94_i8,90_i8,(-15_i8)];
_1.1 = _10.2 ^ _17.2;
_6 = !_7;
_17 = (_10.0, _10.1, _1.1);
Goto(bb10)
}
bb10 = {
_12 = !334093202304137878813623220937226357686_u128;
_9 = _10.1;
_1.0 = [767500367_i32,(-1453784646_i32),(-339802364_i32),(-546339456_i32)];
_10.2 = (-960304912_i32) as u64;
_3 = _7 <= _6;
_13 = core::ptr::addr_of_mut!(_18);
_17.2 = !_1.1;
place!(Field::<i64>(Variant(_11, 3), 1)) = 205_u8 as i64;
_19 = _17.2 ^ _1.1;
_20 = (_1.0, _1.1, _1.2, _6);
_17 = (Field::<[u32; 4]>(Variant(_11, 3), 0), _9, _1.1);
_21 = _1.1 & _1.1;
SetDiscriminant(_11, 3);
Goto(bb11)
}
bb11 = {
_23 = _10;
_9 = _23.1;
_21 = _20.1 + _20.1;
place!(Field::<i64>(Variant(_11, 3), 1)) = 1417283570869204420_i64;
_6 = _2 < _4;
place!(Field::<[u32; 4]>(Variant(_11, 3), 0)) = [48119709_u32,3231371321_u32,3489335813_u32,3300881234_u32];
_12 = 166627777627377357048422175823638131507_u128;
_15 = Field::<i64>(Variant(_11, 3), 1);
_24 = _12;
_17 = _10;
SetDiscriminant(_11, 2);
_22 = _24 as f32;
_3 = _6;
_10.0 = [2153555139_u32,834111775_u32,931333823_u32,2081350332_u32];
place!(Field::<*const u64>(Variant(_11, 2), 3)) = core::ptr::addr_of!(_20.1);
_20 = (_1.0, _21, _1.2, _6);
_5 = _6 == _7;
_10 = (_17.0, _17.1, _20.1);
RET = [45_i8,(-31_i8),(-113_i8)];
_11 = Adt46::Variant3 { fld0: _10.0,fld1: _15 };
_23.0 = [4070315723_u32,2516108317_u32,852551407_u32,1526438463_u32];
place!(Field::<[u32; 4]>(Variant(_11, 3), 0)) = [1822104918_u32,1990820397_u32,2330680363_u32,1729657917_u32];
RET = [4_i8,56_i8,119_i8];
_9 = _10.1;
_1.2 = _20.2 & _20.2;
_3 = !_5;
RET = [(-29_i8),4_i8,19_i8];
match _15 {
0 => bb9,
1 => bb2,
2 => bb10,
3 => bb4,
4 => bb7,
1417283570869204420 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_21 = _10.2 + _19;
_20.3 = !_4;
_20.2 = _1.2 - _1.2;
place!(Field::<[u32; 4]>(Variant(_11, 3), 0)) = _17.0;
_23.1 = _8;
_2 = _10.1 < _8;
_17.1 = _23.1;
place!(Field::<[u32; 4]>(Variant(_11, 3), 0)) = [2842744769_u32,37412038_u32,1165821655_u32,2468264353_u32];
_13 = core::ptr::addr_of_mut!(_18);
SetDiscriminant(_11, 3);
Call(_24 = core::intrinsics::transmute(_23.0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_20.2 = !_1.2;
RET = [(-100_i8),(-112_i8),(-74_i8)];
_11 = Adt46::Variant3 { fld0: _10.0,fld1: _15 };
_10 = (_23.0, _9, _1.1);
_8 = _23.1;
_7 = _20.3;
_17.2 = _1.1;
_20.3 = !_6;
_2 = !_4;
_23.1 = _9;
_20.0 = [(-2079664191_i32),(-692722647_i32),197329161_i32,(-543860010_i32)];
_2 = !_6;
place!(Field::<[u32; 4]>(Variant(_11, 3), 0)) = [426731491_u32,1865635030_u32,3766648031_u32,2356421373_u32];
_1 = (_20.0, _21, _20.2, _5);
_3 = _2;
_13 = core::ptr::addr_of_mut!((*_13));
_12 = 2613234089_u32 as u128;
_23.2 = !_19;
_19 = _17.2;
_1.1 = _20.1 | _19;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(19_usize, 6_usize, Move(_6), 23_usize, Move(_23), 10_usize, Move(_10), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(19_usize, 5_usize, Move(_5), 4_usize, Move(_4), 24_usize, Move(_24), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(19_usize, 7_usize, Move(_7), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{90cf1}'), std::hint::black_box(31759_u16), std::hint::black_box(2233050966_u32), std::hint::black_box(23847_i16), std::hint::black_box(260038918_i32), std::hint::black_box(5523572876341002817_i64), std::hint::black_box(109291785196529409912601421977140294180_i128), std::hint::black_box(5_usize), std::hint::black_box(127028424164410556054588833820917442867_u128));
                
            }
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt40::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: *const [u32; 4],
fld1: char,
fld2: f64,

},
Variant1{
fld0: *mut *const u64,
fld1: *const u64,

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt41::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: i64,
fld1: [u16; 2],

},
Variant1{
fld0: f64,
fld1: *mut *const u64,
fld2: (i32, [i32; 4], i64, u128),
fld3: i8,
fld4: *mut i64,
fld5: ([i32; 4], u64, u16, bool),
fld6: i64,
fld7: [u32; 4],

},
Variant2{
fld0: f64,
fld1: (([i8; 3],), ([i8; 3],), f32, usize),
fld2: isize,
fld3: [i64; 8],
fld4: usize,
fld5: i32,
fld6: u16,

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: ([u32; 4], char, u64),
fld1: char,

},
Variant1{
fld0: [u64; 3],
fld1: char,
fld2: *const [u32; 4],
fld3: (i128, i128, u64, u32),
fld4: [u32; 4],
fld5: [char; 2],
fld6: i64,

},
Variant2{
fld0: bool,
fld1: char,
fld2: [i64; 8],
fld3: usize,
fld4: *const isize,
fld5: i32,
fld6: *const u64,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: [i64; 8],
fld1: char,
fld2: *mut *const u64,
fld3: Adt41,
fld4: (([i8; 3],), ([i8; 3],), f32, usize),

},
Variant1{
fld0: bool,
fld1: *const [u32; 4],
fld2: *const isize,

},
Variant2{
fld0: u128,
fld1: (([i8; 3],), ([i8; 3],), f32, usize),
fld2: (char, *const u64, isize, ([u32; 4], char, u64)),
fld3: (i32, [i32; 4], i64, u128),
fld4: ([i8; 3],),
fld5: ([u32; 4], char, u64),
fld6: (u32, u32, *const isize),
fld7: u16,

}}
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: bool,
fld1: u32,
fld2: i64,
fld3: *mut (i32, [i32; 4], i64, u128),

},
Variant1{
fld0: (i128, i128, u64, u32),
fld1: [char; 2],
fld2: isize,
fld3: u64,
fld4: u128,
fld5: *mut *const u64,
fld6: *const isize,

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
fld0: ([i32; 4], u64, u16, bool),
fld1: u128,
fld2: Adt40,
fld3: *const isize,
fld4: Adt43,
fld5: [i32; 4],
fld6: *const [u32; 4],
fld7: [u32; 4],

},
Variant1{
fld0: i16,
fld1: [i64; 8],
fld2: Adt44,
fld3: Adt43,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: u32,
fld1: (char, *const u64, isize, ([u32; 4], char, u64)),

},
Variant1{
fld0: *mut *const u64,
fld1: (([i8; 3],), ([i8; 3],), f32, usize),

},
Variant2{
fld0: Adt44,
fld1: Adt45,
fld2: u8,
fld3: *const u64,

},
Variant3{
fld0: [u32; 4],
fld1: i64,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: Adt42,
fld1: i128,
fld2: f32,
fld3: *mut (i32, [i32; 4], i64, u128),
fld4: u64,
fld5: Adt40,

},
Variant1{
fld0: [i8; 3],
fld1: [u16; 2],
fld2: [i32; 4],
fld3: Adt46,
fld4: Adt40,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: Adt40,
fld1: char,
fld2: Adt45,
fld3: Adt46,
fld4: Adt41,
fld5: (([i8; 3],), ([i8; 3],), f32, usize),
fld6: Adt44,
fld7: ([i32; 4], u64, u16, bool),
}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: u64,
fld1: (char, *const u64, isize, ([u32; 4], char, u64)),
fld2: i8,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: *const [u32; 4],
fld1: u8,
fld2: i32,
fld3: (char, *const u64, isize, ([u32; 4], char, u64)),
fld4: [i64; 8],

},
Variant1{
fld0: Adt48,
fld1: char,
fld2: ([i8; 3],),
fld3: [i8; 3],
fld4: [u32; 4],

},
Variant2{
fld0: Adt42,
fld1: *mut i64,
fld2: *mut (i32, [i32; 4], i64, u128),
fld3: i8,
fld4: ([i32; 4], u64, u16, bool),
fld5: i32,
fld6: (char, *const u64, isize, ([u32; 4], char, u64)),

},
Variant3{
fld0: Adt41,
fld1: Adt40,
fld2: (u32, u32, *const isize),
fld3: (([i8; 3],), ([i8; 3],), f32, usize),
fld4: [i32; 4],
fld5: *const [u32; 4],
fld6: ([i32; 4], u64, u16, bool),
fld7: u32,

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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: usize,
fld1: Adt50,

},
Variant1{
fld0: Adt44,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: usize,
fld1: (char, *const u64, isize, ([u32; 4], char, u64)),
fld2: u8,
}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: u64,
fld1: Adt48,
fld2: (char, *const u64, isize, ([u32; 4], char, u64)),
fld3: *mut *const u64,
fld4: f32,
fld5: *const u64,
fld6: [u32; 4],
}
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: ([i8; 3],),
fld1: u32,

},
Variant1{
fld0: Adt40,
fld1: [i64; 8],
fld2: Adt45,
fld3: (i128, i128, u64, u32),
fld4: ([i32; 4], u64, u16, bool),
fld5: i128,

},
Variant2{
fld0: Adt45,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
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
fld0: Adt45,
fld1: ([i32; 4], u64, u16, bool),
fld2: Adt53,
fld3: [u64; 3],
fld4: u32,
fld5: ([u32; 4], char, u64),
fld6: [i32; 4],
fld7: Adt51,

},
Variant1{
fld0: Adt47,
fld1: i64,
fld2: (char, *const u64, isize, ([u32; 4], char, u64)),

},
Variant2{
fld0: *mut (i32, [i32; 4], i64, u128),
fld1: Adt47,
fld2: isize,
fld3: usize,

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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: ([i32; 4], u64, u16, bool),
fld1: usize,
fld2: ([i8; 3],),
fld3: *const [u32; 4],
fld4: i16,
fld5: [i8; 3],
fld6: Adt43,

},
Variant1{
fld0: Adt45,
fld1: char,
fld2: Adt46,
fld3: (i32, [i32; 4], i64, u128),
fld4: [i32; 4],
fld5: ([i32; 4], u64, u16, bool),

},
Variant2{
fld0: u8,
fld1: Adt46,
fld2: isize,
fld3: i8,
fld4: Adt44,
fld5: f32,

}}

