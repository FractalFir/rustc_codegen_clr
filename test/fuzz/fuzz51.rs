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
            printf("%i\0".as_ptr() as *const c_char,*self as i8 as c_int);
        }
    }
    impl PrintFDebug for u8{
        unsafe fn printf_debug(&self){
            printf("%u\0".as_ptr() as *const c_char,*self as u8 as c_int);
        }
    } 
    impl PrintFDebug for i16{
        unsafe fn printf_debug(&self){
            printf("%i\0".as_ptr() as *const c_char,*self as i16 as c_int);
        }
    }
    impl PrintFDebug for u16{
        unsafe fn printf_debug(&self){
            printf("%u\0".as_ptr() as *const c_char,*self as u16 as c_int);
        }
    } 
    impl PrintFDebug for i32{
        unsafe fn printf_debug(&self){
            printf("%i\0".as_ptr() as *const c_char,*self);
        }
    }
    impl PrintFDebug for f32{
        unsafe fn printf_debug(&self){
            printf("%f\0".as_ptr() as *const c_char,*self as core::ffi::c_double);
        }
    }
    impl PrintFDebug for f64{
        unsafe fn printf_debug(&self){
            printf("%f\0".as_ptr() as *const c_char,*self as core::ffi::c_double);
        }
    }
    impl<T:PrintFDebug,const N:usize> PrintFDebug for [T;N]{
        unsafe fn printf_debug(&self){
            printf("[\0".as_ptr() as *const c_char);
            for b in self{
                b.printf_debug();
                printf(",\0".as_ptr() as *const c_char);
            }
            printf("]\0".as_ptr() as *const c_char);
        }
    }
    impl PrintFDebug for u32{
        unsafe fn printf_debug(&self){
            printf("%u\0".as_ptr() as *const c_char,*self);
        }
    } 
    impl PrintFDebug for char{
        unsafe fn printf_debug(&self){
            printf("%u\0".as_ptr() as *const c_char,*self as u64);
        }
    } 
    impl PrintFDebug for i64{
        unsafe fn printf_debug(&self){
            printf("%li\0".as_ptr() as *const c_char,*self);
        }
    }
    impl PrintFDebug for u64{
        unsafe fn printf_debug(&self){
            printf("%lu\0".as_ptr() as *const c_char,*self);
        }
    } 
    impl PrintFDebug for i128{
        unsafe fn printf_debug(&self){
            u128::printf_debug(&(*self as u128));
        }
    } 
    impl PrintFDebug for u128{
        unsafe fn printf_debug(&self){
            printf("%lx%lx\0".as_ptr() as *const c_char, (*self >> 64) as u64,*self as u64);
        }
    } 
    impl PrintFDebug for isize{
        unsafe fn printf_debug(&self){
            printf("%li\0".as_ptr() as *const c_char,*self as isize);
        }
    }
    impl PrintFDebug for usize{
        unsafe fn printf_debug(&self){
            printf("%lu\0".as_ptr() as *const c_char,*self as usize);
        }
    } 
    impl PrintFDebug for bool{
        unsafe fn printf_debug(&self){
            if *self{
                printf("true\0".as_ptr() as *const c_char);
            }
            else{
                printf("false\0".as_ptr() as *const c_char);
            }
        }
    } 
    impl PrintFDebug for (){
        unsafe fn printf_debug(&self){
            printf("()\0".as_ptr() as *const c_char);
        }
    } 
    impl<A:PrintFDebug> PrintFDebug for (A,){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",)\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug> PrintFDebug for (A,B){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug> PrintFDebug for (A,B,C){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug> PrintFDebug for (A,B,C,D){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.3.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug> PrintFDebug for (A,B,C,D,E){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.3.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.4.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug> PrintFDebug for (A,B,C,D,E,F){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.3.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.4.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.5.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.3.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.4.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.5.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.6.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.3.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.4.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.5.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.6.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.7.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.3.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.4.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.5.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.6.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.7.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.8.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.3.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.4.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.5.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.6.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.7.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.8.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.9.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug,K:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J,K){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.3.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.4.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.5.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.6.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.7.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.8.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.9.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.10.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug,K:PrintFDebug,L:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J,K,L){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.3.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.4.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.5.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.6.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.7.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.8.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.9.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.10.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.11.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
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
            printf("fn%u:_%u = \0".as_ptr() as *const c_char,f,var0);
            val0.printf_debug();
            printf("\n_%u = \0".as_ptr() as *const c_char,var1);
            val1.printf_debug();
            printf("\n_%u = \0".as_ptr() as *const c_char,var2);
            val2.printf_debug();
            printf("\n_%u = \0".as_ptr() as *const c_char,var3);
            val3.printf_debug();
            printf("\n\0".as_ptr() as *const c_char);
        }
    }
    #[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn0(mut _1: u128,mut _2: u32,mut _3: i64,mut _4: i8,mut _5: i16) -> [i32; 5] {
mir! {
type RET = [i32; 5];
let _6: ([isize; 2], usize, i64, u8);
let _7: isize;
let _8: *const i16;
let _9: i32;
let _10: [bool; 8];
let _11: (i32, u128);
let _12: *mut [char; 7];
let _13: i16;
let _14: (*const f32, [char; 7], [u32; 3]);
let _15: u16;
let _16: f64;
let _17: [isize; 2];
let _18: f32;
let _19: u8;
let _20: char;
let _21: isize;
let _22: *const f32;
let _23: f32;
let _24: [char; 6];
let _25: i8;
let _26: ();
let _27: ();
{
RET = [(-767367187_i32),2031870719_i32,(-1049411190_i32),(-108082589_i32),530475844_i32];
_3 = !2222424682185184839_i64;
_6.2 = _3;
_6.1 = 5_usize + 5_usize;
RET = [1445902081_i32,127260356_i32,(-2017876855_i32),(-1856439734_i32),(-584121468_i32)];
_1 = 89053039803927050221583283967734682486_u128;
_4 = 16566_u16 as i8;
_2 = !3748551599_u32;
_3 = _6.2;
_6.0 = [(-9223372036854775808_isize),(-123_isize)];
RET = [(-444755248_i32),3811834_i32,436696068_i32,(-206020319_i32),(-1643229937_i32)];
RET = [1545406748_i32,907932014_i32,(-545026024_i32),(-356326305_i32),(-427720958_i32)];
RET = [(-1020756081_i32),1138207014_i32,1525217227_i32,(-1358276061_i32),1672127292_i32];
_6.3 = !176_u8;
_5 = !(-20440_i16);
_2 = 54028_u16 as u32;
_6.3 = !84_u8;
_6.0 = [(-115_isize),(-9223372036854775808_isize)];
_7 = _6.3 as isize;
RET = [1022373931_i32,1854554203_i32,1534900311_i32,1473629953_i32,1007068240_i32];
RET = [1928255299_i32,1333588662_i32,1977579992_i32,1784772542_i32,(-676395260_i32)];
Call(_6.2 = fn1(_7, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6.0 = [_7,_7];
RET = [(-461137817_i32),(-1844816893_i32),130857056_i32,(-685449379_i32),919498755_i32];
_5 = 9023_i16 ^ (-11776_i16);
_1 = !197335746033889893303454987150674774663_u128;
_2 = (-568539592_i32) as u32;
_3 = (-95169960331578936228190105914507355274_i128) as i64;
_7 = 9223372036854775807_isize;
_6.3 = 214_u8 & 252_u8;
_5 = 816_u16 as i16;
_2 = !314098375_u32;
_7 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_6.0 = [_7,_7];
_2 = _6.1 as u32;
_6.2 = _3;
_5 = -(-11645_i16);
_3 = _6.2;
Goto(bb2)
}
bb2 = {
_5 = (-28820_i16) + 30844_i16;
_5 = !27011_i16;
_6.0 = [_7,_7];
_6.3 = 92_u8 >> _4;
_5 = -20835_i16;
_1 = !266214105007118745735321298547664769081_u128;
_11.1 = _1;
_8 = core::ptr::addr_of!(_5);
_6.2 = _6.3 as i64;
(*_8) = (-10604_i16);
_9 = 760312223_i32;
_5 = (-17430_i16);
_11 = (_9, _1);
_6.3 = 2_u8 | 191_u8;
_1 = (*_8) as u128;
_2 = 3419814241_u32;
_6.1 = 2_usize >> (*_8);
_10 = [true,true,false,false,false,false,true,false];
_4 = (*_8) as i8;
_11.0 = !_9;
Goto(bb3)
}
bb3 = {
(*_8) = -3653_i16;
RET = [_11.0,_9,_11.0,_9,_11.0];
_7 = '\u{fdc0c}' as isize;
_13 = (*_8) << _2;
Call(_6.2 = core::intrinsics::bswap(_3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_3 = !_6.2;
_6.3 = false as u8;
_13 = _5 - (*_8);
(*_8) = !_13;
RET = [_9,_9,_11.0,_11.0,_9];
_7 = 9223372036854775807_isize;
_3 = '\u{99a36}' as i64;
_6.3 = !253_u8;
_3 = _6.2;
_14.2 = [_2,_2,_2];
_16 = _4 as f64;
Goto(bb5)
}
bb5 = {
_8 = core::ptr::addr_of!(_13);
_11.0 = _9 - _9;
_5 = (*_8) >> _1;
_4 = 46_i8;
_17 = [_7,_7];
_12 = core::ptr::addr_of_mut!(_14.1);
_14.1 = ['\u{b291c}','\u{6cf03}','\u{c84c5}','\u{60b4b}','\u{4f9c8}','\u{13a66}','\u{849a5}'];
(*_12) = ['\u{b1ffa}','\u{1021ef}','\u{9a030}','\u{ffa60}','\u{e8e1d}','\u{ae72b}','\u{b3c6b}'];
_16 = _7 as f64;
_11.1 = 6380118807577741109_u64 as u128;
_14.1 = ['\u{a35fc}','\u{30f76}','\u{be4aa}','\u{edf2}','\u{5ad87}','\u{d8dd3}','\u{9bed7}'];
_4 = _6.3 as i8;
(*_8) = _5;
_13 = _5;
_14.0 = core::ptr::addr_of!(_18);
RET = [_11.0,_11.0,_9,_9,_9];
_16 = _11.1 as f64;
_3 = _6.2 << _5;
RET = [_9,_11.0,_9,_11.0,_11.0];
(*_12) = ['\u{bdbfd}','\u{5f5ef}','\u{b947c}','\u{5bc42}','\u{f43b}','\u{10c5b2}','\u{ca5e2}'];
_11 = (_9, _1);
_11.0 = _9;
_13 = _5;
(*_12) = ['\u{f14ee}','\u{74ba0}','\u{6aef5}','\u{c9b78}','\u{10c06e}','\u{b9045}','\u{b7954}'];
_2 = _1 as u32;
_18 = _2 as f32;
match _7 {
0 => bb1,
1 => bb4,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
9223372036854775807 => bb11,
_ => bb10
}
}
bb6 = {
_3 = !_6.2;
_6.3 = false as u8;
_13 = _5 - (*_8);
(*_8) = !_13;
RET = [_9,_9,_11.0,_11.0,_9];
_7 = 9223372036854775807_isize;
_3 = '\u{99a36}' as i64;
_6.3 = !253_u8;
_3 = _6.2;
_14.2 = [_2,_2,_2];
_16 = _4 as f64;
Goto(bb5)
}
bb7 = {
(*_8) = -3653_i16;
RET = [_11.0,_9,_11.0,_9,_11.0];
_7 = '\u{fdc0c}' as isize;
_13 = (*_8) << _2;
Call(_6.2 = core::intrinsics::bswap(_3), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_5 = (-28820_i16) + 30844_i16;
_5 = !27011_i16;
_6.0 = [_7,_7];
_6.3 = 92_u8 >> _4;
_5 = -20835_i16;
_1 = !266214105007118745735321298547664769081_u128;
_11.1 = _1;
_8 = core::ptr::addr_of!(_5);
_6.2 = _6.3 as i64;
(*_8) = (-10604_i16);
_9 = 760312223_i32;
_5 = (-17430_i16);
_11 = (_9, _1);
_6.3 = 2_u8 | 191_u8;
_1 = (*_8) as u128;
_2 = 3419814241_u32;
_6.1 = 2_usize >> (*_8);
_10 = [true,true,false,false,false,false,true,false];
_4 = (*_8) as i8;
_11.0 = !_9;
Goto(bb3)
}
bb9 = {
_6.0 = [_7,_7];
RET = [(-461137817_i32),(-1844816893_i32),130857056_i32,(-685449379_i32),919498755_i32];
_5 = 9023_i16 ^ (-11776_i16);
_1 = !197335746033889893303454987150674774663_u128;
_2 = (-568539592_i32) as u32;
_3 = (-95169960331578936228190105914507355274_i128) as i64;
_7 = 9223372036854775807_isize;
_6.3 = 214_u8 & 252_u8;
_5 = 816_u16 as i16;
_2 = !314098375_u32;
_7 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_6.0 = [_7,_7];
_2 = _6.1 as u32;
_6.2 = _3;
_5 = -(-11645_i16);
_3 = _6.2;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
_7 = 80_isize;
_20 = '\u{25e7f}';
RET = [_11.0,_9,_11.0,_9,_9];
_15 = 8370_u16;
RET = [_11.0,_11.0,_9,_9,_11.0];
_13 = -_5;
(*_8) = _5 << _11.0;
(*_12) = [_20,_20,_20,_20,_20,_20,_20];
_10 = [true,false,false,false,true,false,false,true];
_6.1 = !10921919243050511243_usize;
match _11.0 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb12,
4 => bb13,
760312223 => bb15,
_ => bb14
}
}
bb12 = {
(*_8) = -3653_i16;
RET = [_11.0,_9,_11.0,_9,_11.0];
_7 = '\u{fdc0c}' as isize;
_13 = (*_8) << _2;
Call(_6.2 = core::intrinsics::bswap(_3), ReturnTo(bb4), UnwindUnreachable())
}
bb13 = {
_6.0 = [_7,_7];
RET = [(-461137817_i32),(-1844816893_i32),130857056_i32,(-685449379_i32),919498755_i32];
_5 = 9023_i16 ^ (-11776_i16);
_1 = !197335746033889893303454987150674774663_u128;
_2 = (-568539592_i32) as u32;
_3 = (-95169960331578936228190105914507355274_i128) as i64;
_7 = 9223372036854775807_isize;
_6.3 = 214_u8 & 252_u8;
_5 = 816_u16 as i16;
_2 = !314098375_u32;
_7 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
_6.0 = [_7,_7];
_2 = _6.1 as u32;
_6.2 = _3;
_5 = -(-11645_i16);
_3 = _6.2;
Goto(bb2)
}
bb14 = {
_3 = !_6.2;
_6.3 = false as u8;
_13 = _5 - (*_8);
(*_8) = !_13;
RET = [_9,_9,_11.0,_11.0,_9];
_7 = 9223372036854775807_isize;
_3 = '\u{99a36}' as i64;
_6.3 = !253_u8;
_3 = _6.2;
_14.2 = [_2,_2,_2];
_16 = _4 as f64;
Goto(bb5)
}
bb15 = {
_24 = [_20,_20,_20,_20,_20,_20];
_24 = [_20,_20,_20,_20,_20,_20];
_19 = _6.3 >> _11.0;
_8 = core::ptr::addr_of!(_5);
RET = [_9,_9,_9,_11.0,_11.0];
(*_12) = [_20,_20,_20,_20,_20,_20,_20];
_7 = _3 as isize;
Goto(bb16)
}
bb16 = {
Call(_26 = dump_var(0_usize, 11_usize, Move(_11), 24_usize, Move(_24), 20_usize, Move(_20), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_26 = dump_var(0_usize, 17_usize, Move(_17), 19_usize, Move(_19), 3_usize, Move(_3), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: isize,mut _2: [i32; 5]) -> i64 {
mir! {
type RET = i64;
let _3: u8;
let _4: Adt56;
let _5: Adt49;
let _6: Adt51;
let _7: char;
let _8: char;
let _9: (u128, i32);
let _10: isize;
let _11: f32;
let _12: i64;
let _13: [i16; 2];
let _14: isize;
let _15: bool;
let _16: isize;
let _17: Adt50;
let _18: (i32, u128);
let _19: Adt60;
let _20: isize;
let _21: (f64, u64, i32, f64);
let _22: u128;
let _23: [u32; 1];
let _24: [u32; 3];
let _25: [i16; 2];
let _26: f32;
let _27: [bool; 8];
let _28: bool;
let _29: Adt59;
let _30: Adt53;
let _31: ();
let _32: ();
{
RET = (-1486805297640899372_i64) * 4340077223319149505_i64;
_3 = 198_u8 | 66_u8;
_2 = [(-1705677277_i32),(-648469343_i32),591239129_i32,(-2015965661_i32),1243644235_i32];
RET = 7281353887267290735_i64;
RET = (-7316008278935510898_i64);
_1 = (-48_isize) * 9223372036854775807_isize;
RET = (-3574970869490011487_i64) * (-5193286197961169453_i64);
RET = !1668878338295427149_i64;
_2 = [(-750313069_i32),1214729055_i32,1945723697_i32,(-1700916445_i32),(-1078961652_i32)];
RET = 4456871083279782451_i64;
RET = (-8086271182378522363_i64);
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463455288336249389689093 => bb9,
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
RET = (-5818271213276874792_i64);
_1 = (-9223372036854775808_isize) + 46_isize;
_1 = -(-9223372036854775808_isize);
RET = (-22548_i16) as i64;
RET = 9210973908076200904_i64 & (-7659664551490443839_i64);
_1 = 9223372036854775807_isize;
RET = 8280920950679185214_i64;
_3 = !37_u8;
_3 = 148_u8 * 73_u8;
RET = 9744531878581210642_u64 as i64;
RET = (-4748589529299173214_i64);
_1 = !(-115_isize);
_1 = !(-9223372036854775808_isize);
RET = '\u{14538}' as i64;
_9.0 = (-123182165891504572844203426235976738779_i128) as u128;
_9.1 = (-704629199_i32) | (-1420892696_i32);
_8 = '\u{3b797}';
RET = (-6610924169918011217_i64) & 6471145133847316931_i64;
Call(RET = fn2(_2, _2, _8, _9.0, _2, _3, _8, _9.1, _9, _3, _3, _9.1, _9), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_9.1 = (-805882472_i32);
_3 = 18_u8;
_3 = !117_u8;
RET = (-1123793085329369381_i64) >> _3;
_7 = _8;
_11 = 3388299328_u32 as f32;
_2 = [_9.1,_9.1,_9.1,_9.1,_9.1];
_9 = (80228279984841137758890417166301759549_u128, (-657290581_i32));
_14 = !_1;
_10 = _1;
_5 = Adt49::Variant3 { fld0: 24701_u16 };
_14 = !_10;
_2 = [_9.1,_9.1,_9.1,_9.1,_9.1];
_10 = _1 ^ _14;
_9 = (222373715815698231779529817154884902183_u128, (-2041200362_i32));
_13 = [(-17127_i16),442_i16];
place!(Field::<u16>(Variant(_5, 3), 0)) = 28097_u16;
Goto(bb11)
}
bb11 = {
_14 = _10 | _10;
_14 = !_10;
_6 = Adt51::Variant2 { fld0: Move(_5),fld1: _9.1 };
_18 = (_9.1, _9.0);
_11 = RET as f32;
_3 = 137_u8 ^ 45_u8;
_11 = _18.0 as f32;
_5 = Move(Field::<Adt49>(Variant(_6, 2), 0));
_20 = _10;
place!(Field::<i32>(Variant(_6, 2), 1)) = _18.0;
_21.3 = 1081_i16 as f64;
_10 = 4_usize as isize;
_12 = _9.0 as i64;
_6 = Adt51::Variant2 { fld0: Move(_5),fld1: _18.0 };
_5 = Move(Field::<Adt49>(Variant(_6, 2), 0));
_7 = _8;
_21.0 = _21.3;
SetDiscriminant(_5, 1);
_21.2 = !_18.0;
_18 = (Field::<i32>(Variant(_6, 2), 1), _9.0);
_22 = _9.0 & _9.0;
Goto(bb12)
}
bb12 = {
_8 = _7;
_2 = [_21.2,_18.0,_21.2,_21.2,_9.1];
_15 = true | false;
_21.1 = !3977440714093649794_u64;
_1 = !_10;
RET = _12 * _12;
_21.1 = 6273681770180144714_u64;
_8 = _7;
_2 = [_18.0,_9.1,Field::<i32>(Variant(_6, 2), 1),Field::<i32>(Variant(_6, 2), 1),Field::<i32>(Variant(_6, 2), 1)];
_21.3 = 121205448578607601559404242196252277755_i128 as f64;
_18.1 = _15 as u128;
_26 = 21865_i16 as f32;
place!(Field::<u8>(Variant(_5, 1), 0)) = _3;
_13 = [(-10797_i16),(-30592_i16)];
place!(Field::<Adt49>(Variant(_6, 2), 0)) = Move(_5);
Goto(bb13)
}
bb13 = {
SetDiscriminant(Field::<Adt49>(Variant(_6, 2), 0), 0);
_26 = _11;
_12 = RET;
match Field::<i32>(Variant(_6, 2), 1) {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
5 => bb19,
340282366920938463463374607429727011094 => bb21,
_ => bb20
}
}
bb14 = {
_8 = _7;
_2 = [_21.2,_18.0,_21.2,_21.2,_9.1];
_15 = true | false;
_21.1 = !3977440714093649794_u64;
_1 = !_10;
RET = _12 * _12;
_21.1 = 6273681770180144714_u64;
_8 = _7;
_2 = [_18.0,_9.1,Field::<i32>(Variant(_6, 2), 1),Field::<i32>(Variant(_6, 2), 1),Field::<i32>(Variant(_6, 2), 1)];
_21.3 = 121205448578607601559404242196252277755_i128 as f64;
_18.1 = _15 as u128;
_26 = 21865_i16 as f32;
place!(Field::<u8>(Variant(_5, 1), 0)) = _3;
_13 = [(-10797_i16),(-30592_i16)];
place!(Field::<Adt49>(Variant(_6, 2), 0)) = Move(_5);
Goto(bb13)
}
bb15 = {
_14 = _10 | _10;
_14 = !_10;
_6 = Adt51::Variant2 { fld0: Move(_5),fld1: _9.1 };
_18 = (_9.1, _9.0);
_11 = RET as f32;
_3 = 137_u8 ^ 45_u8;
_11 = _18.0 as f32;
_5 = Move(Field::<Adt49>(Variant(_6, 2), 0));
_20 = _10;
place!(Field::<i32>(Variant(_6, 2), 1)) = _18.0;
_21.3 = 1081_i16 as f64;
_10 = 4_usize as isize;
_12 = _9.0 as i64;
_6 = Adt51::Variant2 { fld0: Move(_5),fld1: _18.0 };
_5 = Move(Field::<Adt49>(Variant(_6, 2), 0));
_7 = _8;
_21.0 = _21.3;
SetDiscriminant(_5, 1);
_21.2 = !_18.0;
_18 = (Field::<i32>(Variant(_6, 2), 1), _9.0);
_22 = _9.0 & _9.0;
Goto(bb12)
}
bb16 = {
Return()
}
bb17 = {
RET = (-5818271213276874792_i64);
_1 = (-9223372036854775808_isize) + 46_isize;
_1 = -(-9223372036854775808_isize);
RET = (-22548_i16) as i64;
RET = 9210973908076200904_i64 & (-7659664551490443839_i64);
_1 = 9223372036854775807_isize;
RET = 8280920950679185214_i64;
_3 = !37_u8;
_3 = 148_u8 * 73_u8;
RET = 9744531878581210642_u64 as i64;
RET = (-4748589529299173214_i64);
_1 = !(-115_isize);
_1 = !(-9223372036854775808_isize);
RET = '\u{14538}' as i64;
_9.0 = (-123182165891504572844203426235976738779_i128) as u128;
_9.1 = (-704629199_i32) | (-1420892696_i32);
_8 = '\u{3b797}';
RET = (-6610924169918011217_i64) & 6471145133847316931_i64;
Call(RET = fn2(_2, _2, _8, _9.0, _2, _3, _8, _9.1, _9, _3, _3, _9.1, _9), ReturnTo(bb10), UnwindUnreachable())
}
bb18 = {
Return()
}
bb19 = {
Return()
}
bb20 = {
Return()
}
bb21 = {
_18.1 = _15 as u128;
RET = _12 - _12;
_21.3 = _21.0;
_7 = _8;
_23 = [2682210645_u32];
place!(Field::<[u128; 4]>(Variant(place!(Field::<Adt49>(Variant(_6, 2), 0)), 0), 0)) = [_18.1,_18.1,_22,_22];
_28 = !_15;
_14 = -_10;
_9.1 = Field::<i32>(Variant(_6, 2), 1);
Goto(bb22)
}
bb22 = {
Call(_31 = dump_var(1_usize, 28_usize, Move(_28), 2_usize, Move(_2), 18_usize, Move(_18), 9_usize, Move(_9)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_31 = dump_var(1_usize, 8_usize, Move(_8), 20_usize, Move(_20), 12_usize, Move(_12), 7_usize, Move(_7)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: [i32; 5],mut _2: [i32; 5],mut _3: char,mut _4: u128,mut _5: [i32; 5],mut _6: u8,mut _7: char,mut _8: i32,mut _9: (u128, i32),mut _10: u8,mut _11: u8,mut _12: i32,mut _13: (u128, i32)) -> i64 {
mir! {
type RET = i64;
let _14: [u128; 4];
let _15: [i16; 2];
let _16: bool;
let _17: usize;
let _18: u32;
let _19: isize;
let _20: (u128, i32);
let _21: (*const f32, [char; 7], [u32; 3]);
let _22: [isize; 2];
let _23: isize;
let _24: Adt46;
let _25: bool;
let _26: [char; 6];
let _27: &'static u32;
let _28: Adt52;
let _29: Adt57;
let _30: f32;
let _31: Adt53;
let _32: i32;
let _33: [char; 6];
let _34: isize;
let _35: [i32; 5];
let _36: i64;
let _37: u64;
let _38: i8;
let _39: bool;
let _40: char;
let _41: u64;
let _42: ();
let _43: ();
{
_1 = [_13.1,_8,_9.1,_9.1,_13.1];
_13.0 = !_9.0;
_13.1 = 155727108_u32 as i32;
_12 = _9.1;
_3 = _7;
_7 = _3;
_13.0 = !_4;
RET = 8901001582006376583_i64;
_11 = _10;
_3 = _7;
RET = (-4504754066101773156_i64);
_9 = (_13.0, _12);
_10 = _6;
_3 = _7;
_6 = _11;
_1 = [_12,_13.1,_8,_12,_12];
_8 = _13.1 ^ _12;
_5 = _2;
Goto(bb1)
}
bb1 = {
_10 = _11 | _6;
_14 = [_9.0,_4,_13.0,_9.0];
_2 = [_9.1,_9.1,_12,_9.1,_8];
_8 = _12;
_10 = 6_usize as u8;
_9.0 = _13.0;
_4 = 9223372036854775807_isize as u128;
RET = (-2769223556741254578_i64) ^ 4193352377202174407_i64;
_3 = _7;
_11 = 4_usize as u8;
RET = 4495021603457220307_i64;
RET = (-5474146477641076314_i64);
_9 = _13;
_13.1 = _12 + _9.1;
Goto(bb2)
}
bb2 = {
RET = 5588436012122932390_i64;
_16 = !false;
_17 = RET as usize;
_15 = [(-14145_i16),(-19332_i16)];
_12 = -_13.1;
_13.0 = _9.0;
Goto(bb3)
}
bb3 = {
RET = (-8190378389022957287_i64);
_5 = [_13.1,_13.1,_8,_13.1,_12];
_7 = _3;
_5 = [_12,_9.1,_13.1,_13.1,_12];
Goto(bb4)
}
bb4 = {
_6 = _11 * _11;
_3 = _7;
_5 = [_13.1,_13.1,_12,_8,_13.1];
_13.0 = !_9.0;
_7 = _3;
RET = 976883990012345220_i64;
_6 = !_11;
_12 = _13.1 - _9.1;
_5 = _2;
_18 = 3982622670_u32;
_9.0 = !_13.0;
_9.1 = -_12;
_7 = _3;
_9 = (_13.0, _13.1);
_12 = _17 as i32;
_2 = [_9.1,_8,_9.1,_8,_8];
_17 = 9223372036854775807_isize as usize;
_20.1 = _9.1 | _9.1;
_9.1 = -_20.1;
_21.2 = [_18,_18,_18];
_16 = !false;
_22 = [58_isize,9223372036854775807_isize];
Call(_21.2 = fn3(_22, _9.1, _9, _8, _18, _9.1, _18, _20.1, _13.1, _6, _20.1, _20.1, _22, _9), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_22 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = 5911025244857803935_i64;
_21.2 = [_18,_18,_18];
_9.0 = !_13.0;
_18 = 3887859554_u32;
RET = _9.1 as i64;
_13.1 = _11 as i32;
_18 = 2519416511_u32 + 2959052370_u32;
_14 = [_13.0,_4,_13.0,_9.0];
_14 = [_4,_9.0,_13.0,_9.0];
_19 = (-9223372036854775808_isize) ^ 52_isize;
RET = -(-5476662746473561282_i64);
_19 = 57_isize + (-9223372036854775808_isize);
_5 = _2;
_2 = [_20.1,_8,_9.1,_9.1,_9.1];
_19 = RET as isize;
_2 = [_13.1,_20.1,_20.1,_9.1,_9.1];
_6 = _10;
_24.fld2 = _19;
Goto(bb6)
}
bb6 = {
_15 = [(-5461_i16),(-14687_i16)];
_3 = _7;
_9.1 = _20.1 + _20.1;
_17 = 416427654659912482_usize;
_15 = [(-16518_i16),17577_i16];
_24.fld6 = 307618141866657025_u64 | 12489825951406483072_u64;
_4 = _18 as u128;
_19 = -_24.fld2;
_12 = !_20.1;
_9.0 = _4 - _4;
_5 = _1;
_13.0 = !_9.0;
_24.fld1 = core::ptr::addr_of!(_22);
_24.fld2 = _19 & _19;
_14 = [_13.0,_13.0,_13.0,_4];
_24.fld0 = !_16;
Goto(bb7)
}
bb7 = {
_13 = _9;
RET = (-3435144051212316446_i64) - 1229500338221043304_i64;
_20 = (_9.0, _13.1);
_24.fld3 = [16468_i16,19094_i16];
_21.1 = [_7,_7,_3,_7,_3,_7,_7];
_30 = _17 as f32;
_27 = &_18;
_27 = &_18;
_5 = [_9.1,_13.1,_20.1,_13.1,_12];
Goto(bb8)
}
bb8 = {
_24.fld5 = core::ptr::addr_of!(_30);
_22 = [_24.fld2,_24.fld2];
_7 = _3;
_13 = _20;
_9.0 = _13.0;
_15 = [12022_i16,22073_i16];
_6 = !_10;
_8 = -_20.1;
_18 = 1185241016_u32;
_21.2 = [_18,_18,_18];
RET = !(-7446059696001174396_i64);
_20 = (_9.0, _12);
_6 = _10 ^ _10;
_19 = _24.fld2 + _24.fld2;
RET = _19 as i64;
_23 = _24.fld6 as isize;
Goto(bb9)
}
bb9 = {
_14 = [_13.0,_4,_9.0,_20.0];
_15 = [(-18595_i16),21519_i16];
_31 = Adt53::Variant1 { fld0: _18,fld1: _14 };
_10 = !_11;
_26 = [_7,_3,_3,_3,_3,_7];
_6 = !_10;
_17 = _9.1 as usize;
_21.1 = [_7,_3,_3,_3,_3,_7,_3];
_32 = 62699553001106429563140162932776778666_i128 as i32;
_36 = RET ^ RET;
Goto(bb10)
}
bb10 = {
SetDiscriminant(_31, 0);
_34 = _19 * _19;
_35 = _1;
_3 = _7;
_13 = _9;
_13.1 = -_20.1;
_24.fld6 = (-158449870244618925760152209908913998617_i128) as u64;
_24.fld7 = [_24.fld0,_24.fld0,_16,_24.fld0,_24.fld0,_16];
_28 = Adt52::Variant0 { fld0: _21.2 };
_30 = _17 as f32;
_32 = _10 as i32;
_38 = !8_i8;
place!(Field::<f64>(Variant(_31, 0), 4)) = (-18785_i16) as f64;
_39 = !_16;
_9 = _13;
place!(Field::<f64>(Variant(_31, 0), 4)) = _38 as f64;
SetDiscriminant(_28, 0);
_40 = _3;
_27 = &_18;
place!(Field::<[u32; 3]>(Variant(_28, 0), 0)) = _21.2;
_31 = Adt53::Variant1 { fld0: (*_27),fld1: _14 };
match (*_27) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb9,
4 => bb11,
5 => bb12,
6 => bb13,
1185241016 => bb15,
_ => bb14
}
}
bb11 = {
_22 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = 5911025244857803935_i64;
_21.2 = [_18,_18,_18];
_9.0 = !_13.0;
_18 = 3887859554_u32;
RET = _9.1 as i64;
_13.1 = _11 as i32;
_18 = 2519416511_u32 + 2959052370_u32;
_14 = [_13.0,_4,_13.0,_9.0];
_14 = [_4,_9.0,_13.0,_9.0];
_19 = (-9223372036854775808_isize) ^ 52_isize;
RET = -(-5476662746473561282_i64);
_19 = 57_isize + (-9223372036854775808_isize);
_5 = _2;
_2 = [_20.1,_8,_9.1,_9.1,_9.1];
_19 = RET as isize;
_2 = [_13.1,_20.1,_20.1,_9.1,_9.1];
_6 = _10;
_24.fld2 = _19;
Goto(bb6)
}
bb12 = {
_24.fld5 = core::ptr::addr_of!(_30);
_22 = [_24.fld2,_24.fld2];
_7 = _3;
_13 = _20;
_9.0 = _13.0;
_15 = [12022_i16,22073_i16];
_6 = !_10;
_8 = -_20.1;
_18 = 1185241016_u32;
_21.2 = [_18,_18,_18];
RET = !(-7446059696001174396_i64);
_20 = (_9.0, _12);
_6 = _10 ^ _10;
_19 = _24.fld2 + _24.fld2;
RET = _19 as i64;
_23 = _24.fld6 as isize;
Goto(bb9)
}
bb13 = {
_13 = _9;
RET = (-3435144051212316446_i64) - 1229500338221043304_i64;
_20 = (_9.0, _13.1);
_24.fld3 = [16468_i16,19094_i16];
_21.1 = [_7,_7,_3,_7,_3,_7,_7];
_30 = _17 as f32;
_27 = &_18;
_27 = &_18;
_5 = [_9.1,_13.1,_20.1,_13.1,_12];
Goto(bb8)
}
bb14 = {
RET = 5588436012122932390_i64;
_16 = !false;
_17 = RET as usize;
_15 = [(-14145_i16),(-19332_i16)];
_12 = -_13.1;
_13.0 = _9.0;
Goto(bb3)
}
bb15 = {
place!(Field::<u32>(Variant(_31, 1), 0)) = !_18;
_7 = _40;
_12 = _9.1;
_34 = !_23;
_30 = 23369_i16 as f32;
_20.0 = _9.0;
RET = _36 + _36;
_34 = _19 & _19;
_17 = 79067363733944297450310169027762663947_i128 as usize;
_30 = _24.fld6 as f32;
_5 = _1;
_21.0 = _24.fld5;
_24.fld0 = _8 >= _8;
_24.fld6 = _9.1 as u64;
_6 = _11;
RET = _36 + _36;
_9.1 = !_20.1;
_36 = (-24483_i16) as i64;
_27 = &place!(Field::<u32>(Variant(_31, 1), 0));
_13.0 = _34 as u128;
_24.fld3 = [28886_i16,12124_i16];
_9 = _13;
_19 = _34 << _12;
_20.1 = _12 | _13.1;
_26 = [_40,_7,_7,_7,_7,_3];
SetDiscriminant(_31, 0);
_25 = !_24.fld0;
_24.fld6 = 590272685400086835_u64;
_22 = [_34,_23];
_23 = _19 ^ _34;
Goto(bb16)
}
bb16 = {
Call(_42 = dump_var(2_usize, 35_usize, Move(_35), 15_usize, Move(_15), 18_usize, Move(_18), 34_usize, Move(_34)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(2_usize, 40_usize, Move(_40), 16_usize, Move(_16), 8_usize, Move(_8), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(2_usize, 13_usize, Move(_13), 2_usize, Move(_2), 1_usize, Move(_1), 5_usize, Move(_5)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_42 = dump_var(2_usize, 3_usize, Move(_3), 7_usize, Move(_7), 9_usize, Move(_9), 43_usize, _43), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [isize; 2],mut _2: i32,mut _3: (u128, i32),mut _4: i32,mut _5: u32,mut _6: i32,mut _7: u32,mut _8: i32,mut _9: i32,mut _10: u8,mut _11: i32,mut _12: i32,mut _13: [isize; 2],mut _14: (u128, i32)) -> [u32; 3] {
mir! {
type RET = [u32; 3];
let _15: *const i128;
let _16: [u64; 3];
let _17: f64;
let _18: [bool; 6];
let _19: [u128; 4];
let _20: isize;
let _21: isize;
let _22: Adt46;
let _23: isize;
let _24: Adt53;
let _25: (*const f32, [char; 7], [u32; 3]);
let _26: [isize; 2];
let _27: [bool; 6];
let _28: [char; 7];
let _29: [isize; 2];
let _30: Adt56;
let _31: isize;
let _32: u128;
let _33: ([usize; 2], i16);
let _34: Adt60;
let _35: isize;
let _36: ([isize; 2], usize, i64, u8);
let _37: Adt51;
let _38: ([isize; 2], usize, i64, u8);
let _39: [i32; 5];
let _40: char;
let _41: [bool; 6];
let _42: (u128, i32);
let _43: ();
let _44: ();
{
RET = [_5,_5,_5];
RET = [_5,_5,_7];
_9 = _4 & _6;
_3.0 = (-28713577521132222121235065942698988967_i128) as u128;
Call(_5 = fn4(_3.0, _2, _3.1, _12, _9, _14), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = [9223372036854775807_isize,0_isize];
_11 = '\u{d61e2}' as i32;
_3.0 = _14.0 - _14.0;
_14.1 = _4 >> _5;
_8 = 29468_u16 as i32;
Call(_10 = fn6(_11, _9, _14.1, _14, _13, _14, _9, _14.1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = _2;
_3 = (_14.0, _2);
_14.1 = 81_i8 as i32;
_2 = _3.1 | _3.1;
_10 = 7105522897088513810_i64 as u8;
_5 = _7;
_12 = !_2;
_3.0 = 5205_i16 as u128;
RET = [_7,_7,_5];
_17 = 5144885831003673195_u64 as f64;
match _5 {
3982622670 => bb3,
_ => bb1
}
}
bb3 = {
_14.1 = (-5053562240066238885_i64) as i32;
_6 = -_12;
RET = [_5,_7,_7];
_17 = (-9766_i16) as f64;
_12 = _3.1;
_1 = [(-9223372036854775808_isize),9223372036854775807_isize];
Goto(bb4)
}
bb4 = {
_10 = !190_u8;
_18 = [false,true,false,true,false,false];
_13 = [(-9223372036854775808_isize),26_isize];
_16 = [12879365223995424777_u64,4919736023215058096_u64,4501551608326886862_u64];
_2 = 159871851840758741201822944831979760521_i128 as i32;
_14 = (_3.0, _6);
_13 = [9223372036854775807_isize,9223372036854775807_isize];
_19 = [_14.0,_14.0,_14.0,_14.0];
_19 = [_14.0,_3.0,_14.0,_14.0];
_3.1 = _14.1 << _6;
_6 = 6_usize as i32;
_10 = 140_u8;
_9 = _14.1;
_5 = _7;
_4 = _14.1;
_20 = 120_isize;
_3.1 = _6;
_5 = _7 | _7;
_21 = '\u{5318e}' as isize;
RET = [_5,_7,_5];
_3.0 = !_14.0;
_17 = (-3110968572903563923_i64) as f64;
Call(_22.fld0 = fn7(_18, _14, _21, _14.1, _14.1, _4, _18, _20, _14.1, _9), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_3.1 = _8 + _9;
_16 = [7050820957870405550_u64,16458457347479808709_u64,7471697132016005237_u64];
_14.0 = _10 as u128;
_22.fld3 = [442_i16,(-15037_i16)];
_14 = (_3.0, _3.1);
_17 = (-1382012270890011259_i64) as f64;
_17 = _10 as f64;
_22.fld2 = _3.1 as isize;
_23 = _3.0 as isize;
_22.fld7 = [_22.fld0,_22.fld0,_22.fld0,_22.fld0,_22.fld0,_22.fld0];
_24 = Adt53::Variant1 { fld0: _5,fld1: _19 };
_22.fld6 = 10861317964375457677_u64 & 13964518659968592554_u64;
_10 = !20_u8;
place!(Field::<[u128; 4]>(Variant(_24, 1), 1)) = [_14.0,_14.0,_3.0,_14.0];
_22.fld1 = core::ptr::addr_of!(_1);
RET = [_5,Field::<u32>(Variant(_24, 1), 0),Field::<u32>(Variant(_24, 1), 0)];
_22.fld4 = core::ptr::addr_of!(_17);
RET = [Field::<u32>(Variant(_24, 1), 0),_5,_5];
_22.fld2 = _21;
_22.fld1 = core::ptr::addr_of!(_26);
_26 = _13;
_22.fld2 = -_21;
_25.1 = ['\u{6948b}','\u{581d2}','\u{a9feb}','\u{daa69}','\u{61000}','\u{53c4f}','\u{d56c6}'];
_11 = _4;
_25.2 = [Field::<u32>(Variant(_24, 1), 0),Field::<u32>(Variant(_24, 1), 0),_7];
RET = _25.2;
match _7 {
0 => bb4,
1 => bb6,
3982622670 => bb8,
_ => bb7
}
}
bb6 = {
_1 = [9223372036854775807_isize,0_isize];
_11 = '\u{d61e2}' as i32;
_3.0 = _14.0 - _14.0;
_14.1 = _4 >> _5;
_8 = 29468_u16 as i32;
Call(_10 = fn6(_11, _9, _14.1, _14, _13, _14, _9, _14.1), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_14.1 = (-5053562240066238885_i64) as i32;
_6 = -_12;
RET = [_5,_7,_7];
_17 = (-9766_i16) as f64;
_12 = _3.1;
_1 = [(-9223372036854775808_isize),9223372036854775807_isize];
Goto(bb4)
}
bb8 = {
_24 = Adt53::Variant1 { fld0: _7,fld1: _19 };
place!(Field::<[u128; 4]>(Variant(_24, 1), 1)) = [_3.0,_14.0,_3.0,_3.0];
_3 = (_14.0, _12);
_13 = [_22.fld2,_20];
_23 = _22.fld2 + _21;
_27 = _22.fld7;
_13 = [_22.fld2,_20];
_16 = [_22.fld6,_22.fld6,_22.fld6];
_9 = !_14.1;
SetDiscriminant(_24, 0);
place!(Field::<([usize; 2], i16)>(Variant(_24, 0), 3)).0 = [1_usize,7_usize];
place!(Field::<([usize; 2], i16)>(Variant(_24, 0), 3)).1 = 237_i16 + 22118_i16;
_17 = _4 as f64;
_3.1 = _22.fld6 as i32;
_3.1 = _14.1;
_22.fld7 = [_22.fld0,_22.fld0,_22.fld0,_22.fld0,_22.fld0,_22.fld0];
_19 = [_14.0,_14.0,_14.0,_3.0];
_1 = _13;
_22.fld3 = [Field::<([usize; 2], i16)>(Variant(_24, 0), 3).1,Field::<([usize; 2], i16)>(Variant(_24, 0), 3).1];
_3.1 = _4 - _4;
_16 = [_22.fld6,_22.fld6,_22.fld6];
_4 = _11;
_26 = [_21,_20];
_13 = [_23,_22.fld2];
_25.2 = [_5,_7,_7];
Call(_21 = fn8(_11, _22.fld7, _22.fld7, _17, _27, _27, _25.1, _27, _27, _22.fld0, _27, _22.fld0, _22.fld7, _22.fld7), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_22.fld0 = true;
Call(_8 = core::intrinsics::transmute(_3.1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_11 = -_8;
_14 = (_3.0, _8);
place!(Field::<f64>(Variant(_24, 0), 4)) = _17 + _17;
_27 = [_22.fld0,_22.fld0,_22.fld0,_22.fld0,_22.fld0,_22.fld0];
_33.0 = Field::<([usize; 2], i16)>(Variant(_24, 0), 3).0;
_33 = Field::<([usize; 2], i16)>(Variant(_24, 0), 3);
_27 = [_22.fld0,_22.fld0,_22.fld0,_22.fld0,_22.fld0,_22.fld0];
_31 = _20;
_33.1 = !Field::<([usize; 2], i16)>(Variant(_24, 0), 3).1;
Goto(bb11)
}
bb11 = {
_33 = (Field::<([usize; 2], i16)>(Variant(_24, 0), 3).0, Field::<([usize; 2], i16)>(Variant(_24, 0), 3).1);
place!(Field::<([usize; 2], i16)>(Variant(_24, 0), 3)).1 = !_33.1;
_3 = (_14.0, _11);
_36.3 = _10;
match _20 {
0 => bb1,
1 => bb2,
120 => bb12,
_ => bb9
}
}
bb12 = {
_3.0 = 35674_u16 as u128;
_36.0 = [_22.fld2,_21];
_26 = [_21,_23];
_29 = _36.0;
_4 = _3.1 >> _8;
_20 = _23 | _31;
_3.0 = _14.0;
_12 = !_4;
Goto(bb13)
}
bb13 = {
_36 = (_1, 9622281852089799350_usize, 7488222343027805034_i64, _10);
_42.0 = _14.0 * _14.0;
_14.0 = _42.0 * _42.0;
Goto(bb14)
}
bb14 = {
_32 = _42.0 * _3.0;
_16 = [_22.fld6,_22.fld6,_22.fld6];
_35 = _22.fld6 as isize;
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(3_usize, 21_usize, Move(_21), 7_usize, Move(_7), 16_usize, Move(_16), 27_usize, Move(_27)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(3_usize, 10_usize, Move(_10), 8_usize, Move(_8), 4_usize, Move(_4), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(3_usize, 20_usize, Move(_20), 33_usize, Move(_33), 3_usize, Move(_3), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_43 = dump_var(3_usize, 11_usize, Move(_11), 18_usize, Move(_18), 44_usize, _44, 44_usize, _44), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: u128,mut _2: i32,mut _3: i32,mut _4: i32,mut _5: i32,mut _6: (u128, i32)) -> u32 {
mir! {
type RET = u32;
let _7: isize;
let _8: Adt53;
let _9: [i16; 2];
let _10: [bool; 6];
let _11: isize;
let _12: i16;
let _13: f64;
let _14: u32;
let _15: ();
let _16: ();
{
RET = 24441_u16 as u32;
_2 = _5 * _3;
_3 = -_6.1;
_6.0 = _1 * _1;
_6 = (_1, _5);
RET = 1934826193_u32 | 4035083600_u32;
_2 = _3;
RET = 2533684823_u32;
_2 = '\u{a7903}' as i32;
_1 = _6.0 + _6.0;
_5 = _3;
RET = (-6_i8) as u32;
_1 = _6.0;
_7 = (-9223372036854775808_isize) | 46_isize;
_4 = _6.1;
Call(_4 = core::intrinsics::transmute(RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = [20376_i16,(-13546_i16)];
_6.0 = _7 as u128;
_4 = _5;
_4 = !_6.1;
_4 = !_5;
_11 = RET as isize;
Call(_5 = fn5(_6.1, _4, _7, _6, _6.1, _6.1, _6.1, _4, _3, _4, _4, _3, _6.1, _7, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = 112_i8 as i32;
_5 = true as i32;
_11 = _7 & _7;
_1 = _6.0 + _6.0;
RET = !810374545_u32;
_2 = 116_i8 as i32;
_11 = 319236548840931574_u64 as isize;
_10 = [false,false,false,false,true,true];
_2 = -_4;
_10 = [false,false,true,true,true,false];
_1 = 10944600081809564877_usize as u128;
Goto(bb3)
}
bb3 = {
_2 = -_4;
_6.1 = _4 << _6.0;
RET = 1475425788_u32 ^ 2900623858_u32;
_7 = (-6891762994229141470_i64) as isize;
_11 = !_7;
_12 = (-2744_i16) >> _6.1;
_5 = 3318649080005096801_usize as i32;
_6 = (_1, _2);
_2 = _3;
RET = 842858446_u32;
_3 = !_2;
_10 = [false,false,false,true,false,false];
_3 = _4 - _2;
_6 = (_1, _2);
RET = 825767148_u32;
_6.1 = _3;
RET = 40_i8 as u32;
_2 = _6.1 + _3;
_6.0 = (-3256171381708962616_i64) as u128;
_6.0 = _1;
RET = 52509426_u32 >> _3;
_4 = (-8276531927740947119_i64) as i32;
Goto(bb4)
}
bb4 = {
Call(_15 = dump_var(4_usize, 11_usize, Move(_11), 3_usize, Move(_3), 9_usize, Move(_9), 7_usize, Move(_7)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_15 = dump_var(4_usize, 1_usize, Move(_1), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: i32,mut _2: i32,mut _3: isize,mut _4: (u128, i32),mut _5: i32,mut _6: i32,mut _7: i32,mut _8: i32,mut _9: i32,mut _10: i32,mut _11: i32,mut _12: i32,mut _13: i32,mut _14: isize,mut _15: (u128, i32)) -> i32 {
mir! {
type RET = i32;
let _16: ([usize; 2], i16);
let _17: *const i16;
let _18: [i32; 5];
let _19: [u64; 3];
let _20: Adt57;
let _21: [bool; 8];
let _22: ();
let _23: ();
{
_4.1 = _13 - _10;
_6 = !_4.1;
_4 = (_15.0, _1);
RET = _15.1 & _12;
_5 = _9;
_12 = _6 * _11;
_2 = _6;
_14 = _3;
_2 = _12;
_8 = _10 << _12;
_15.1 = _2 - _13;
RET = _12;
_4 = _15;
_17 = core::ptr::addr_of!(_16.1);
_10 = _4.1;
Goto(bb1)
}
bb1 = {
Call(_22 = dump_var(5_usize, 7_usize, Move(_7), 13_usize, Move(_13), 3_usize, Move(_3), 2_usize, Move(_2)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_22 = dump_var(5_usize, 14_usize, Move(_14), 1_usize, Move(_1), 4_usize, Move(_4), 23_usize, _23), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: i32,mut _2: i32,mut _3: i32,mut _4: (u128, i32),mut _5: [isize; 2],mut _6: (u128, i32),mut _7: i32,mut _8: i32) -> u8 {
mir! {
type RET = u8;
let _9: (f64, u64, i32, f64);
let _10: Adt49;
let _11: f64;
let _12: char;
let _13: Adt60;
let _14: i8;
let _15: (i32, u128);
let _16: [i16; 2];
let _17: [char; 6];
let _18: isize;
let _19: [bool; 6];
let _20: f32;
let _21: f32;
let _22: ();
let _23: ();
{
_6.1 = false as i32;
_4 = _6;
RET = 108_u8;
_3 = _7;
_3 = !_8;
RET = '\u{53360}' as u8;
_9.2 = -_2;
_2 = -_3;
_9.3 = 42324_u16 as f64;
_9.1 = !14724059870342710527_u64;
_9.0 = _3 as f64;
_2 = _8;
_6 = (_4.0, _9.2);
_9.2 = _3 | _6.1;
_4.1 = !_6.1;
_7 = 9223372036854775807_isize as i32;
_1 = -_4.1;
RET = 180_u8;
_4.0 = 1634498611_u32 as u128;
_2 = _8 - _3;
match RET {
0 => bb1,
1 => bb2,
180 => bb4,
_ => bb3
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
_12 = '\u{8666c}';
_6 = _4;
_9.3 = _9.0;
_2 = _4.1 * _6.1;
_8 = (-20655_i16) as i32;
_10 = Adt49::Variant3 { fld0: 62693_u16 };
_10 = Adt49::Variant3 { fld0: 24979_u16 };
_7 = !_9.2;
_14 = 72_i8 << _4.1;
_9.2 = _2 >> _7;
_9.3 = 24180_i16 as f64;
_9.1 = 3396867514791801284_u64;
_15 = (_1, _4.0);
_14 = 18_i8;
_9.3 = _9.0;
_17 = [_12,_12,_12,_12,_12,_12];
_6.1 = _3;
_14 = -67_i8;
RET = 5_u8 - 239_u8;
Goto(bb5)
}
bb5 = {
_19 = [false,false,false,false,false,false];
_16 = [11903_i16,30771_i16];
_6.1 = 6640996430531169076_i64 as i32;
_21 = 43660_u16 as f32;
_19 = [true,true,false,true,true,true];
_5 = [(-9223372036854775808_isize),(-122_isize)];
_6.1 = _15.0;
RET = 72_u8 << _9.2;
_4.1 = _8;
_16 = [2375_i16,(-26362_i16)];
_5 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_18 = -9223372036854775807_isize;
_20 = -_21;
Goto(bb6)
}
bb6 = {
Call(_22 = dump_var(6_usize, 19_usize, Move(_19), 4_usize, Move(_4), 17_usize, Move(_17), 5_usize, Move(_5)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_22 = dump_var(6_usize, 12_usize, Move(_12), 14_usize, Move(_14), 16_usize, Move(_16), 23_usize, _23), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: [bool; 6],mut _2: (u128, i32),mut _3: isize,mut _4: i32,mut _5: i32,mut _6: i32,mut _7: [bool; 6],mut _8: isize,mut _9: i32,mut _10: i32) -> bool {
mir! {
type RET = bool;
let _11: char;
let _12: [u32; 1];
let _13: *const i128;
let _14: f32;
let _15: [i32; 5];
let _16: isize;
let _17: (f64, u64, i32, f64);
let _18: u8;
let _19: u64;
let _20: (f64, u64, i32, f64);
let _21: Adt49;
let _22: f32;
let _23: [i16; 2];
let _24: isize;
let _25: [i16; 2];
let _26: [bool; 6];
let _27: [char; 6];
let _28: (i32, u128);
let _29: bool;
let _30: i32;
let _31: [isize; 2];
let _32: [u32; 3];
let _33: ();
let _34: ();
{
RET = _5 != _2.1;
_5 = 7_usize as i32;
RET = false;
_2.1 = _5;
_6 = !_10;
_8 = _3 ^ _3;
_1 = [RET,RET,RET,RET,RET,RET];
_6 = _10 ^ _9;
_9 = _6;
_2.0 = 93_u8 as u128;
_2.1 = _4 + _9;
RET = !false;
_9 = 203_u8 as i32;
_11 = '\u{17063}';
_12 = [2732883505_u32];
_7 = [RET,RET,RET,RET,RET,RET];
_6 = _2.1 >> _4;
_2 = (191232138226804152576772174898549462820_u128, _6);
_2.1 = !_4;
_15 = [_4,_10,_6,_10,_6];
_1 = [RET,RET,RET,RET,RET,RET];
RET = false | true;
_2 = (132543596584967332210319619985290258099_u128, _4);
RET = _6 > _6;
RET = _4 > _6;
RET = !false;
match _2.0 {
132543596584967332210319619985290258099 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_10 = _2.1 << _6;
_12 = [4154636505_u32];
_17.1 = 8027601732879103712_u64 + 123026496999707561_u64;
_17.3 = 0_usize as f64;
_17.3 = 126660137110047702136637402534300885201_i128 as f64;
_8 = -_3;
_17.1 = 31_u8 as u64;
_17.0 = _17.3 + _17.3;
_2.1 = _10;
RET = true & false;
_2 = (229008384726207681959476268924250591248_u128, _6);
RET = true;
_14 = _3 as f32;
_2.1 = 3310584761997422031_i64 as i32;
_20.1 = _10 as u64;
match _2.0 {
0 => bb1,
1 => bb3,
2 => bb4,
229008384726207681959476268924250591248 => bb6,
_ => bb5
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
_8 = (-289_i16) as isize;
_20.3 = _17.0;
_17.0 = _20.3;
_1 = [RET,RET,RET,RET,RET,RET];
_7 = [RET,RET,RET,RET,RET,RET];
_20 = (_17.0, _17.1, _10, _17.3);
_16 = _3 ^ _8;
_19 = _20.1 >> _10;
_4 = -_20.2;
_21 = Adt49::Variant1 { fld0: 79_u8 };
_20.1 = _19 >> _10;
_6 = _20.2;
_7 = [RET,RET,RET,RET,RET,RET];
_3 = _16;
_7 = _1;
_9 = _20.2;
_17 = (_20.0, _20.1, _10, _20.0);
_17 = _20;
_16 = _3;
Goto(bb7)
}
bb7 = {
_18 = 148_u8;
_6 = _10 ^ _10;
_2.0 = 171017417834821663146205657577600354561_u128;
_16 = _3 << _20.2;
_17.3 = _17.0;
_23 = [(-10342_i16),4357_i16];
_24 = !_16;
_15 = [_9,_4,_17.2,_6,_20.2];
place!(Field::<u8>(Variant(_21, 1), 0)) = _18;
_20.0 = _17.3 * _20.3;
_24 = -_16;
_20 = _17;
Call(_8 = core::intrinsics::transmute(_16), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_20.1 = !_17.1;
_26 = _1;
Goto(bb9)
}
bb9 = {
_27 = [_11,_11,_11,_11,_11,_11];
_17.0 = -_20.0;
_20.3 = _20.0;
_7 = [RET,RET,RET,RET,RET,RET];
SetDiscriminant(_21, 0);
_22 = _14;
RET = _17.1 >= _20.1;
_16 = -_24;
_4 = _17.2 + _6;
_11 = '\u{7e09c}';
_27 = [_11,_11,_11,_11,_11,_11];
_18 = 188_u8 & 38_u8;
_17.3 = _14 as f64;
_20.2 = (-32448_i16) as i32;
Goto(bb10)
}
bb10 = {
Call(_33 = dump_var(7_usize, 19_usize, Move(_19), 3_usize, Move(_3), 27_usize, Move(_27), 11_usize, Move(_11)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_33 = dump_var(7_usize, 24_usize, Move(_24), 4_usize, Move(_4), 16_usize, Move(_16), 10_usize, Move(_10)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_33 = dump_var(7_usize, 18_usize, Move(_18), 7_usize, Move(_7), 34_usize, _34, 34_usize, _34), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: i32,mut _2: [bool; 6],mut _3: [bool; 6],mut _4: f64,mut _5: [bool; 6],mut _6: [bool; 6],mut _7: [char; 7],mut _8: [bool; 6],mut _9: [bool; 6],mut _10: bool,mut _11: [bool; 6],mut _12: bool,mut _13: [bool; 6],mut _14: [bool; 6]) -> isize {
mir! {
type RET = isize;
let _15: u16;
let _16: bool;
let _17: [i16; 2];
let _18: [u128; 4];
let _19: Adt52;
let _20: f64;
let _21: Adt48;
let _22: (u128, i32);
let _23: isize;
let _24: f32;
let _25: Adt57;
let _26: [i32; 5];
let _27: char;
let _28: bool;
let _29: isize;
let _30: [i16; 2];
let _31: f64;
let _32: f32;
let _33: Adt51;
let _34: (i32, u128);
let _35: ();
let _36: ();
{
_7 = ['\u{9ef5d}','\u{46a0d}','\u{f5b4e}','\u{1446e}','\u{6d38}','\u{322d}','\u{23710}'];
_6 = [_12,_12,_12,_10,_10,_12];
_6 = _11;
_8 = [_10,_10,_12,_10,_10,_12];
_9 = [_10,_12,_12,_10,_12,_12];
_7 = ['\u{c272f}','\u{245a3}','\u{a1917}','\u{ad3af}','\u{b4a78}','\u{b3fd7}','\u{cb1ee}'];
_11 = [_12,_12,_12,_12,_10,_10];
_5 = [_12,_12,_10,_12,_10,_10];
_1 = !1710812232_i32;
_11 = [_12,_12,_12,_12,_10,_10];
_3 = [_10,_10,_12,_10,_10,_10];
RET = (-15981_i16) as isize;
_8 = [_12,_10,_12,_12,_12,_10];
_13 = [_10,_10,_10,_12,_10,_10];
_18 = [207327980463787166444990358785860427612_u128,169550708750848068712823010276514564345_u128,268433521270516427629583285126252817113_u128,74141371154539916383868015405980659942_u128];
_14 = _8;
_8 = [_10,_10,_12,_10,_10,_12];
_6 = _13;
_14 = [_12,_12,_12,_12,_10,_12];
Goto(bb1)
}
bb1 = {
RET = 9223372036854775807_isize ^ 9223372036854775807_isize;
_22.0 = !302481131844631350628973243764596492246_u128;
_15 = 63944_u16;
match _15 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
63944 => bb7,
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
_20 = _15 as f64;
_22 = (243312653098116800889572049410351521808_u128, _1);
_21.fld1 = [1613735824_u32,3055534279_u32,886534029_u32];
_4 = 156830691618724825779040990620899884550_i128 as f64;
_19 = Adt52::Variant0 { fld0: _21.fld1 };
_21.fld2.2 = [3916733195_u32,1652002414_u32,3473208611_u32];
Call(_11 = fn9(_2, _5, _13, _5, _8), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_23 = -RET;
_10 = !_12;
_21.fld2.1 = ['\u{5e3be}','\u{f5339}','\u{fc088}','\u{3f08a}','\u{1089d1}','\u{9daa2}','\u{ff6bb}'];
SetDiscriminant(_19, 0);
_24 = (-9682_i16) as f32;
_29 = 116285022684209133929845784752029493683_i128 as isize;
_21.fld3 = [_1,_22.1,_1,_1,_22.1,_22.1,_1];
_31 = _4 + _4;
_6 = [_12,_10,_12,_12,_12,_12];
_11 = [_12,_12,_12,_10,_12,_10];
_6 = _9;
_21.fld1 = _21.fld2.2;
_32 = -_24;
match _15 {
0 => bb7,
1 => bb6,
2 => bb4,
3 => bb9,
4 => bb10,
63944 => bb12,
_ => bb11
}
}
bb9 = {
RET = 9223372036854775807_isize ^ 9223372036854775807_isize;
_22.0 = !302481131844631350628973243764596492246_u128;
_15 = 63944_u16;
match _15 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
63944 => bb7,
_ => bb6
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_23 = -RET;
_28 = _12 & _12;
match _22.0 {
0 => bb13,
243312653098116800889572049410351521808 => bb15,
_ => bb14
}
}
bb13 = {
RET = 9223372036854775807_isize ^ 9223372036854775807_isize;
_22.0 = !302481131844631350628973243764596492246_u128;
_15 = 63944_u16;
match _15 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
63944 => bb7,
_ => bb6
}
}
bb14 = {
Return()
}
bb15 = {
_15 = !30370_u16;
_22.1 = _1;
_21.fld1 = [618932939_u32,1060981177_u32,1584831642_u32];
_21.fld2.0 = core::ptr::addr_of!(_24);
_28 = !_12;
RET = -_29;
_1 = _22.1 | _22.1;
_17 = [16235_i16,8642_i16];
_24 = _23 as f32;
Goto(bb16)
}
bb16 = {
Call(_35 = dump_var(8_usize, 1_usize, Move(_1), 9_usize, Move(_9), 8_usize, Move(_8), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(8_usize, 17_usize, Move(_17), 18_usize, Move(_18), 15_usize, Move(_15), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_35 = dump_var(8_usize, 28_usize, Move(_28), 23_usize, Move(_23), 36_usize, _36, 36_usize, _36), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [bool; 6],mut _2: [bool; 6],mut _3: [bool; 6],mut _4: [bool; 6],mut _5: [bool; 6]) -> [bool; 6] {
mir! {
type RET = [bool; 6];
let _6: ([usize; 2], i16);
let _7: [i32; 5];
let _8: u32;
let _9: [u128; 4];
let _10: Adt48;
let _11: f32;
let _12: char;
let _13: *const f32;
let _14: [i32; 7];
let _15: [u128; 4];
let _16: f64;
let _17: bool;
let _18: [char; 7];
let _19: bool;
let _20: isize;
let _21: u32;
let _22: char;
let _23: Adt55;
let _24: (i32, u128);
let _25: [i32; 7];
let _26: [i16; 2];
let _27: [char; 6];
let _28: [u64; 3];
let _29: *const i16;
let _30: *mut [char; 7];
let _31: [u64; 3];
let _32: isize;
let _33: i128;
let _34: [i32; 5];
let _35: Adt53;
let _36: i32;
let _37: [usize; 2];
let _38: *mut [char; 7];
let _39: isize;
let _40: ();
let _41: ();
{
RET = [true,true,false,false,true,false];
_1 = _2;
_1 = [false,true,false,true,true,false];
_1 = [true,true,true,true,false,true];
_5 = [false,false,false,false,false,false];
_6.1 = !3484_i16;
_8 = !2489903784_u32;
_3 = [false,true,true,false,false,true];
_6.0 = [10942731203609220683_usize,8921042555989354860_usize];
_8 = !2315613765_u32;
_4 = _2;
RET = _2;
_5 = [false,false,false,true,false,true];
RET = [false,false,false,true,false,false];
RET = [false,false,true,false,true,true];
_3 = [false,false,false,true,true,false];
_2 = _4;
_4 = [true,false,false,false,true,false];
_6.0 = [6_usize,9581414429567184938_usize];
_5 = _4;
_4 = _2;
_9 = [28599577624052766958767646924282409097_u128,244769525100505337278947653731732604479_u128,71960838597298985635727078649635104902_u128,176391963956695310135442314933018089192_u128];
_10.fld0 = [true,true,false,false,false,false,true,true];
_10.fld2.2 = [_8,_8,_8];
_7 = [1324902682_i32,(-1153268526_i32),(-390352037_i32),1317895378_i32,1894611813_i32];
_2 = [true,false,false,true,true,false];
RET = [false,true,true,false,true,true];
Goto(bb1)
}
bb1 = {
RET = [true,true,true,false,false,false];
_1 = [true,true,true,false,false,false];
_7 = [482672385_i32,575458794_i32,(-1674509741_i32),(-1264353336_i32),2124387563_i32];
_10.fld3 = [1868518292_i32,(-1440937000_i32),179525157_i32,2013496023_i32,(-892044947_i32),(-1904368793_i32),1908964090_i32];
_4 = [true,true,false,false,true,false];
_10.fld1 = [_8,_8,_8];
_4 = [false,true,true,false,true,false];
_10.fld2.0 = core::ptr::addr_of!(_11);
_10.fld2.1 = ['\u{fbbbc}','\u{56dfc}','\u{2ec89}','\u{41913}','\u{956c8}','\u{b0281}','\u{70795}'];
_10.fld1 = _10.fld2.2;
_9 = [213697677581565939967633095752093342662_u128,5055360728148239359131090795583447462_u128,139643636731428484431416075136986243649_u128,273714386432648805682022364616542624681_u128];
_10.fld2.2 = [_8,_8,_8];
_11 = 260166944268698526884767470343603701910_u128 as f32;
_6.0 = [5901907445540912022_usize,13591747252651336987_usize];
RET = [true,true,true,false,false,false];
_2 = _4;
_12 = '\u{25c63}';
_13 = _10.fld2.0;
_1 = [false,true,false,false,false,false];
Goto(bb2)
}
bb2 = {
_4 = [false,true,false,true,true,false];
_12 = '\u{3ef94}';
_10.fld2.2 = [_8,_8,_8];
(*_13) = 8723263855955428278_u64 as f32;
_14 = _10.fld3;
_7 = [(-1425286038_i32),565166839_i32,(-1369379321_i32),(-1955676248_i32),(-1370491401_i32)];
_10.fld2.0 = core::ptr::addr_of!((*_13));
_10.fld3 = [(-1731957815_i32),(-2124289798_i32),(-282413062_i32),(-1859926736_i32),1273476649_i32,1335109549_i32,(-1427209854_i32)];
_10.fld1 = [_8,_8,_8];
RET = _2;
_8 = 3401039712_u32;
_2 = [false,false,true,true,false,true];
_10.fld2.2 = _10.fld1;
RET = _5;
RET = [false,false,true,false,false,true];
RET = [true,false,false,false,false,false];
_10.fld1 = _10.fld2.2;
_2 = [true,false,true,false,true,false];
_15 = [241641142619565328837293168396719508297_u128,180031259016686160542826083890314527550_u128,32487727578898503768091979786451991192_u128,129519519460687149160956251345081308752_u128];
_3 = RET;
RET = [true,true,true,true,false,false];
_1 = [true,true,false,false,false,true];
_6.0 = [7_usize,6144790764698898793_usize];
_15 = [266845248881816626999453008672929012977_u128,171199619245960922244138103274098237818_u128,82837642459414167334784182019050970605_u128,12546886612738123217287570615299902147_u128];
_16 = (-80_isize) as f64;
match _8 {
3401039712 => bb3,
_ => bb1
}
}
bb3 = {
_17 = false;
_14 = [(-1005227688_i32),588013779_i32,(-56322368_i32),(-581706233_i32),(-865915596_i32),649243469_i32,(-422754626_i32)];
_10.fld2.1 = [_12,_12,_12,_12,_12,_12,_12];
_19 = _17;
_6.1 = 8401_i16;
_21 = _8;
_5 = [_19,_19,_17,_19,_19,_17];
_13 = core::ptr::addr_of!((*_13));
Call(RET = fn10(_7, _3, _14, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_10.fld0 = [_17,_17,_19,_17,_17,_19,_19,_17];
_13 = _10.fld2.0;
_15 = [231737854295924033841834108728654941506_u128,170737316737744207293011067634285398385_u128,24358704326143723838571366013530694085_u128,241521720401193063143579429958830498032_u128];
_6.0 = [4_usize,6362321345727747827_usize];
_2 = _1;
_8 = _21;
_2 = [_19,_19,_17,_17,_17,_19];
_22 = _12;
_24 = (1872200682_i32, 36144572058226817433800752296532825976_u128);
Goto(bb5)
}
bb5 = {
_24.0 = (-1391227787_i32);
RET = [_17,_17,_17,_19,_17,_19];
RET = _1;
_25 = _10.fld3;
_4 = [_17,_17,_19,_17,_19,_19];
_8 = 56_i8 as u32;
_13 = core::ptr::addr_of!((*_13));
_10.fld2.1 = [_12,_12,_22,_22,_12,_22,_12];
_7 = [_24.0,_24.0,_24.0,_24.0,_24.0];
Goto(bb6)
}
bb6 = {
_28 = [8741196643393390641_u64,1453809375574831607_u64,11707243049209713651_u64];
_20 = 9223372036854775807_isize;
_22 = _12;
_13 = core::ptr::addr_of!((*_13));
_6.0 = [5_usize,10394741032678589231_usize];
_3 = RET;
_11 = _16 as f32;
match _24.0 {
0 => bb4,
1 => bb7,
340282366920938463463374607430376983669 => bb9,
_ => bb8
}
}
bb7 = {
_17 = false;
_14 = [(-1005227688_i32),588013779_i32,(-56322368_i32),(-581706233_i32),(-865915596_i32),649243469_i32,(-422754626_i32)];
_10.fld2.1 = [_12,_12,_12,_12,_12,_12,_12];
_19 = _17;
_6.1 = 8401_i16;
_21 = _8;
_5 = [_19,_19,_17,_19,_19,_17];
_13 = core::ptr::addr_of!((*_13));
Call(RET = fn10(_7, _3, _14, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_10.fld0 = [_17,_17,_19,_17,_17,_19,_19,_17];
_13 = _10.fld2.0;
_15 = [231737854295924033841834108728654941506_u128,170737316737744207293011067634285398385_u128,24358704326143723838571366013530694085_u128,241521720401193063143579429958830498032_u128];
_6.0 = [4_usize,6362321345727747827_usize];
_2 = _1;
_8 = _21;
_2 = [_19,_19,_17,_17,_17,_19];
_22 = _12;
_24 = (1872200682_i32, 36144572058226817433800752296532825976_u128);
Goto(bb5)
}
bb9 = {
(*_13) = 35660_u16 as f32;
_24 = ((-1206100089_i32), 59586526448553602192863698563313562977_u128);
_6.0 = [7_usize,4_usize];
_17 = _19;
_27 = [_22,_12,_22,_22,_12,_22];
_30 = core::ptr::addr_of_mut!(_10.fld2.1);
_4 = [_19,_19,_19,_17,_17,_19];
RET = [_17,_17,_19,_17,_17,_17];
_25 = _14;
_16 = 234_u8 as f64;
_3 = _2;
_6.1 = _22 as i16;
_26 = [_6.1,_6.1];
_6.0 = [3184709640648445200_usize,13608822967676952275_usize];
_18 = (*_30);
(*_13) = _16 as f32;
_15 = _9;
_34 = [_24.0,_24.0,_24.0,_24.0,_24.0];
_10.fld2.0 = core::ptr::addr_of!(_11);
match _24.1 {
59586526448553602192863698563313562977 => bb10,
_ => bb2
}
}
bb10 = {
_24.1 = 148306475112173942478234648799360224904_u128;
_3 = [_19,_19,_19,_17,_17,_17];
_31 = _28;
_10.fld1 = _10.fld2.2;
_10.fld0 = [_17,_17,_17,_19,_17,_19,_17,_19];
(*_13) = _20 as f32;
(*_13) = _24.1 as f32;
(*_30) = [_22,_22,_22,_12,_22,_22,_22];
_10.fld2.2 = _10.fld1;
_14 = [_24.0,_24.0,_24.0,_24.0,_24.0,_24.0,_24.0];
_21 = _8;
_27 = [_22,_12,_12,_22,_12,_22];
match _20 {
9223372036854775807 => bb11,
_ => bb5
}
}
bb11 = {
_8 = 0_usize as u32;
RET = _4;
_27 = [_22,_12,_22,_22,_22,_12];
_1 = [_19,_17,_19,_17,_19,_17];
_10.fld2.0 = _13;
_20 = -(-9223372036854775808_isize);
_16 = _8 as f64;
_2 = _3;
(*_13) = 3708051810869635045_i64 as f32;
(*_13) = _16 as f32;
_1 = [_17,_17,_17,_17,_17,_19];
_4 = _3;
_6.0 = [7_usize,2_usize];
match _24.0 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
6 => bb18,
340282366920938463463374607430562111367 => bb20,
_ => bb19
}
}
bb12 = {
_24.1 = 148306475112173942478234648799360224904_u128;
_3 = [_19,_19,_19,_17,_17,_17];
_31 = _28;
_10.fld1 = _10.fld2.2;
_10.fld0 = [_17,_17,_17,_19,_17,_19,_17,_19];
(*_13) = _20 as f32;
(*_13) = _24.1 as f32;
(*_30) = [_22,_22,_22,_12,_22,_22,_22];
_10.fld2.2 = _10.fld1;
_14 = [_24.0,_24.0,_24.0,_24.0,_24.0,_24.0,_24.0];
_21 = _8;
_27 = [_22,_12,_12,_22,_12,_22];
match _20 {
9223372036854775807 => bb11,
_ => bb5
}
}
bb13 = {
(*_13) = 35660_u16 as f32;
_24 = ((-1206100089_i32), 59586526448553602192863698563313562977_u128);
_6.0 = [7_usize,4_usize];
_17 = _19;
_27 = [_22,_12,_22,_22,_12,_22];
_30 = core::ptr::addr_of_mut!(_10.fld2.1);
_4 = [_19,_19,_19,_17,_17,_19];
RET = [_17,_17,_19,_17,_17,_17];
_25 = _14;
_16 = 234_u8 as f64;
_3 = _2;
_6.1 = _22 as i16;
_26 = [_6.1,_6.1];
_6.0 = [3184709640648445200_usize,13608822967676952275_usize];
_18 = (*_30);
(*_13) = _16 as f32;
_15 = _9;
_34 = [_24.0,_24.0,_24.0,_24.0,_24.0];
_10.fld2.0 = core::ptr::addr_of!(_11);
match _24.1 {
59586526448553602192863698563313562977 => bb10,
_ => bb2
}
}
bb14 = {
_10.fld0 = [_17,_17,_19,_17,_17,_19,_19,_17];
_13 = _10.fld2.0;
_15 = [231737854295924033841834108728654941506_u128,170737316737744207293011067634285398385_u128,24358704326143723838571366013530694085_u128,241521720401193063143579429958830498032_u128];
_6.0 = [4_usize,6362321345727747827_usize];
_2 = _1;
_8 = _21;
_2 = [_19,_19,_17,_17,_17,_19];
_22 = _12;
_24 = (1872200682_i32, 36144572058226817433800752296532825976_u128);
Goto(bb5)
}
bb15 = {
_17 = false;
_14 = [(-1005227688_i32),588013779_i32,(-56322368_i32),(-581706233_i32),(-865915596_i32),649243469_i32,(-422754626_i32)];
_10.fld2.1 = [_12,_12,_12,_12,_12,_12,_12];
_19 = _17;
_6.1 = 8401_i16;
_21 = _8;
_5 = [_19,_19,_17,_19,_19,_17];
_13 = core::ptr::addr_of!((*_13));
Call(RET = fn10(_7, _3, _14, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb16 = {
_28 = [8741196643393390641_u64,1453809375574831607_u64,11707243049209713651_u64];
_20 = 9223372036854775807_isize;
_22 = _12;
_13 = core::ptr::addr_of!((*_13));
_6.0 = [5_usize,10394741032678589231_usize];
_3 = RET;
_11 = _16 as f32;
match _24.0 {
0 => bb4,
1 => bb7,
340282366920938463463374607430376983669 => bb9,
_ => bb8
}
}
bb17 = {
_24.0 = (-1391227787_i32);
RET = [_17,_17,_17,_19,_17,_19];
RET = _1;
_25 = _10.fld3;
_4 = [_17,_17,_19,_17,_19,_19];
_8 = 56_i8 as u32;
_13 = core::ptr::addr_of!((*_13));
_10.fld2.1 = [_12,_12,_22,_22,_12,_22,_12];
_7 = [_24.0,_24.0,_24.0,_24.0,_24.0];
Goto(bb6)
}
bb18 = {
_10.fld0 = [_17,_17,_19,_17,_17,_19,_19,_17];
_13 = _10.fld2.0;
_15 = [231737854295924033841834108728654941506_u128,170737316737744207293011067634285398385_u128,24358704326143723838571366013530694085_u128,241521720401193063143579429958830498032_u128];
_6.0 = [4_usize,6362321345727747827_usize];
_2 = _1;
_8 = _21;
_2 = [_19,_19,_17,_17,_17,_19];
_22 = _12;
_24 = (1872200682_i32, 36144572058226817433800752296532825976_u128);
Goto(bb5)
}
bb19 = {
RET = [true,true,true,false,false,false];
_1 = [true,true,true,false,false,false];
_7 = [482672385_i32,575458794_i32,(-1674509741_i32),(-1264353336_i32),2124387563_i32];
_10.fld3 = [1868518292_i32,(-1440937000_i32),179525157_i32,2013496023_i32,(-892044947_i32),(-1904368793_i32),1908964090_i32];
_4 = [true,true,false,false,true,false];
_10.fld1 = [_8,_8,_8];
_4 = [false,true,true,false,true,false];
_10.fld2.0 = core::ptr::addr_of!(_11);
_10.fld2.1 = ['\u{fbbbc}','\u{56dfc}','\u{2ec89}','\u{41913}','\u{956c8}','\u{b0281}','\u{70795}'];
_10.fld1 = _10.fld2.2;
_9 = [213697677581565939967633095752093342662_u128,5055360728148239359131090795583447462_u128,139643636731428484431416075136986243649_u128,273714386432648805682022364616542624681_u128];
_10.fld2.2 = [_8,_8,_8];
_11 = 260166944268698526884767470343603701910_u128 as f32;
_6.0 = [5901907445540912022_usize,13591747252651336987_usize];
RET = [true,true,true,false,false,false];
_2 = _4;
_12 = '\u{25c63}';
_13 = _10.fld2.0;
_1 = [false,true,false,false,false,false];
Goto(bb2)
}
bb20 = {
Goto(bb21)
}
bb21 = {
Call(_40 = dump_var(9_usize, 6_usize, Move(_6), 17_usize, Move(_17), 22_usize, Move(_22), 14_usize, Move(_14)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_40 = dump_var(9_usize, 18_usize, Move(_18), 27_usize, Move(_27), 8_usize, Move(_8), 28_usize, Move(_28)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_40 = dump_var(9_usize, 3_usize, Move(_3), 15_usize, Move(_15), 26_usize, Move(_26), 25_usize, Move(_25)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: [i32; 5],mut _2: [bool; 6],mut _3: [i32; 7],mut _4: [bool; 6]) -> [bool; 6] {
mir! {
type RET = [bool; 6];
let _5: Adt52;
let _6: u8;
let _7: (u128, i32);
let _8: Adt53;
let _9: Adt61;
let _10: Adt58;
let _11: u32;
let _12: char;
let _13: char;
let _14: f32;
let _15: isize;
let _16: bool;
let _17: Adt60;
let _18: bool;
let _19: f32;
let _20: [usize; 2];
let _21: [usize; 2];
let _22: [usize; 2];
let _23: u64;
let _24: [bool; 8];
let _25: u32;
let _26: char;
let _27: [char; 7];
let _28: Adt45;
let _29: ();
let _30: ();
{
RET = [false,true,true,false,false,true];
_2 = _4;
_3 = [1903660476_i32,1834743888_i32,(-2028411196_i32),1116582838_i32,344452837_i32,(-1426382762_i32),2069397779_i32];
_1 = [537553377_i32,1537394672_i32,(-1509649445_i32),1455373133_i32,(-1891054625_i32)];
Call(RET = fn11(_3, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [false,true,true,true,true,false];
_3 = [(-1660849820_i32),(-1335680252_i32),(-694011630_i32),23509683_i32,(-796670843_i32),168994887_i32,(-332924172_i32)];
RET = [true,false,false,false,false,true];
RET = [true,true,true,true,false,false];
_7.0 = !169762688725876451584450143630138428678_u128;
_7.1 = (-906038310_i32);
_6 = 131_u8;
_4 = [false,false,false,true,false,false];
RET = _4;
RET = [false,true,true,true,true,false];
RET = [true,false,true,false,true,true];
_7 = (275743258673865537663510452169025444969_u128, (-653281642_i32));
RET = [true,false,true,false,false,true];
_9.fld4.fld2.1 = ['\u{7ee07}','\u{b698}','\u{eabe5}','\u{1447b}','\u{ad193}','\u{ea2d1}','\u{ca065}'];
_9.fld1 = (-5550265630538377524_i64) as u64;
Goto(bb2)
}
bb2 = {
_9.fld4.fld1 = [1501545416_u32,3158158245_u32,3723146585_u32];
_9.fld4.fld3 = [_7.1,_7.1,_7.1,_7.1,_7.1,_7.1,_7.1];
_1 = [_7.1,_7.1,_7.1,_7.1,_7.1];
_9.fld0 = !_6;
_7.0 = 146079991906760935966543445614905815565_u128 * 134624749874461094908299855342258448296_u128;
_3 = _9.fld4.fld3;
_7 = (102907497985597246705503496272835921143_u128, (-1156749103_i32));
_6 = _9.fld0;
_9.fld4.fld0 = [false,false,true,true,false,false,true,true];
_6 = !_9.fld0;
_1 = [_7.1,_7.1,_7.1,_7.1,_7.1];
_9.fld4.fld0 = [false,true,false,true,false,false,false,false];
_9.fld4.fld3 = [_7.1,_7.1,_7.1,_7.1,_7.1,_7.1,_7.1];
_9.fld3 = -61_i8;
_6 = _9.fld0 | _9.fld0;
_7.1 = (-2127956573_i32);
Goto(bb3)
}
bb3 = {
_1 = [_7.1,_7.1,_7.1,_7.1,_7.1];
RET = [false,true,false,false,false,false];
_9.fld1 = 13160761265960281465_u64;
_9.fld4.fld0 = [false,false,true,false,false,true,true,false];
_12 = '\u{e022}';
_9.fld1 = !9315419268473678009_u64;
_7.1 = !1510448018_i32;
_5 = Adt52::Variant0 { fld0: _9.fld4.fld1 };
_5 = Adt52::Variant0 { fld0: _9.fld4.fld1 };
_2 = [true,false,true,true,false,true];
_9.fld4.fld2.1 = [_12,_12,_12,_12,_12,_12,_12];
_9.fld4.fld2.0 = core::ptr::addr_of!(_14);
_13 = _12;
_9.fld4.fld0 = [false,true,false,false,true,false,true,false];
_9.fld0 = !_6;
_9.fld4.fld0 = [false,false,false,true,false,false,true,true];
_7 = (157298086592134950683874233780359356773_u128, (-208373351_i32));
_9.fld1 = !15297114553631784877_u64;
_11 = !588239068_u32;
_9.fld4.fld3 = [_7.1,_7.1,_7.1,_7.1,_7.1,_7.1,_7.1];
_9.fld4.fld3 = _3;
_1 = [_7.1,_7.1,_7.1,_7.1,_7.1];
_15 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_9.fld4.fld0 = [false,true,false,false,false,false,true,false];
match _7.1 {
0 => bb4,
1 => bb5,
2 => bb6,
340282366920938463463374607431559838105 => bb8,
_ => bb7
}
}
bb4 = {
_9.fld4.fld1 = [1501545416_u32,3158158245_u32,3723146585_u32];
_9.fld4.fld3 = [_7.1,_7.1,_7.1,_7.1,_7.1,_7.1,_7.1];
_1 = [_7.1,_7.1,_7.1,_7.1,_7.1];
_9.fld0 = !_6;
_7.0 = 146079991906760935966543445614905815565_u128 * 134624749874461094908299855342258448296_u128;
_3 = _9.fld4.fld3;
_7 = (102907497985597246705503496272835921143_u128, (-1156749103_i32));
_6 = _9.fld0;
_9.fld4.fld0 = [false,false,true,true,false,false,true,true];
_6 = !_9.fld0;
_1 = [_7.1,_7.1,_7.1,_7.1,_7.1];
_9.fld4.fld0 = [false,true,false,true,false,false,false,false];
_9.fld4.fld3 = [_7.1,_7.1,_7.1,_7.1,_7.1,_7.1,_7.1];
_9.fld3 = -61_i8;
_6 = _9.fld0 | _9.fld0;
_7.1 = (-2127956573_i32);
Goto(bb3)
}
bb5 = {
RET = [false,true,true,true,true,false];
_3 = [(-1660849820_i32),(-1335680252_i32),(-694011630_i32),23509683_i32,(-796670843_i32),168994887_i32,(-332924172_i32)];
RET = [true,false,false,false,false,true];
RET = [true,true,true,true,false,false];
_7.0 = !169762688725876451584450143630138428678_u128;
_7.1 = (-906038310_i32);
_6 = 131_u8;
_4 = [false,false,false,true,false,false];
RET = _4;
RET = [false,true,true,true,true,false];
RET = [true,false,true,false,true,true];
_7 = (275743258673865537663510452169025444969_u128, (-653281642_i32));
RET = [true,false,true,false,false,true];
_9.fld4.fld2.1 = ['\u{7ee07}','\u{b698}','\u{eabe5}','\u{1447b}','\u{ad193}','\u{ea2d1}','\u{ca065}'];
_9.fld1 = (-5550265630538377524_i64) as u64;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
RET = [false,false,false,true,true,false];
_14 = 31436_i16 as f32;
_14 = 5709_u16 as f32;
_9.fld0 = !_6;
_2 = [false,false,true,true,false,false];
_2 = [false,true,true,false,false,true];
RET = _4;
_9.fld4.fld2.2 = _9.fld4.fld1;
_4 = RET;
RET = [true,false,false,false,false,false];
_9.fld4.fld2.2 = [_11,_11,_11];
_9.fld4.fld2.2 = [_11,_11,_11];
SetDiscriminant(_5, 1);
_9.fld4.fld2.1 = [_13,_12,_12,_12,_12,_13,_12];
_9.fld1 = 12863601316296138146_u64;
_21 = [4_usize,7_usize];
_19 = _15 as f32;
_15 = -9223372036854775807_isize;
_9.fld4.fld2.0 = core::ptr::addr_of!(_14);
_20 = [11448364734167397345_usize,6_usize];
_9.fld4.fld3 = [_7.1,_7.1,_7.1,_7.1,_7.1,_7.1,_7.1];
_12 = _13;
_20 = [7263022767634258997_usize,7059521332005183770_usize];
_6 = 65435_u16 as u8;
RET = _2;
place!(Field::<(i32, u128)>(Variant(_5, 1), 0)).1 = _19 as u128;
_22 = [17036862412286236671_usize,12364019087101602362_usize];
Goto(bb9)
}
bb9 = {
_15 = -9223372036854775807_isize;
_9.fld1 = !1767646048692207433_u64;
_20 = [2_usize,9981956551005172407_usize];
_13 = _12;
_9.fld1 = 6466073480820114904_u64;
_9.fld4.fld2.1 = [_13,_12,_13,_13,_13,_12,_12];
_9.fld4.fld3 = [_7.1,_7.1,_7.1,_7.1,_7.1,_7.1,_7.1];
_18 = _7.1 > _7.1;
_12 = _13;
_5 = Adt52::Variant0 { fld0: _9.fld4.fld1 };
_9.fld4.fld0 = [_18,_18,_18,_18,_18,_18,_18,_18];
_9.fld4.fld3 = [_7.1,_7.1,_7.1,_7.1,_7.1,_7.1,_7.1];
_15 = 28583_i16 as isize;
_25 = _11;
_9.fld4.fld0 = [_18,_18,_18,_18,_18,_18,_18,_18];
Goto(bb10)
}
bb10 = {
_9.fld2 = core::ptr::addr_of_mut!(_15);
_2 = [_18,_18,_18,_18,_18,_18];
_9.fld4.fld2.2 = [_11,_11,_11];
_11 = !_25;
_11 = 29005_u16 as u32;
_22 = _21;
_18 = _19 > _14;
Goto(bb11)
}
bb11 = {
_25 = _11;
_3 = [_7.1,_7.1,_7.1,_7.1,_7.1,_7.1,_7.1];
_4 = [_18,_18,_18,_18,_18,_18];
_22 = [0_usize,10510574132353419566_usize];
_16 = !_18;
_9.fld3 = 91_i8;
_20 = [2_usize,6_usize];
_9.fld4.fld2.0 = core::ptr::addr_of!(_14);
_19 = _14;
_18 = _16 | _16;
SetDiscriminant(_5, 0);
match _7.1 {
0 => bb4,
1 => bb7,
2 => bb12,
340282366920938463463374607431559838105 => bb14,
_ => bb13
}
}
bb12 = {
_9.fld2 = core::ptr::addr_of_mut!(_15);
_2 = [_18,_18,_18,_18,_18,_18];
_9.fld4.fld2.2 = [_11,_11,_11];
_11 = !_25;
_11 = 29005_u16 as u32;
_22 = _21;
_18 = _19 > _14;
Goto(bb11)
}
bb13 = {
_1 = [_7.1,_7.1,_7.1,_7.1,_7.1];
RET = [false,true,false,false,false,false];
_9.fld1 = 13160761265960281465_u64;
_9.fld4.fld0 = [false,false,true,false,false,true,true,false];
_12 = '\u{e022}';
_9.fld1 = !9315419268473678009_u64;
_7.1 = !1510448018_i32;
_5 = Adt52::Variant0 { fld0: _9.fld4.fld1 };
_5 = Adt52::Variant0 { fld0: _9.fld4.fld1 };
_2 = [true,false,true,true,false,true];
_9.fld4.fld2.1 = [_12,_12,_12,_12,_12,_12,_12];
_9.fld4.fld2.0 = core::ptr::addr_of!(_14);
_13 = _12;
_9.fld4.fld0 = [false,true,false,false,true,false,true,false];
_9.fld0 = !_6;
_9.fld4.fld0 = [false,false,false,true,false,false,true,true];
_7 = (157298086592134950683874233780359356773_u128, (-208373351_i32));
_9.fld1 = !15297114553631784877_u64;
_11 = !588239068_u32;
_9.fld4.fld3 = [_7.1,_7.1,_7.1,_7.1,_7.1,_7.1,_7.1];
_9.fld4.fld3 = _3;
_1 = [_7.1,_7.1,_7.1,_7.1,_7.1];
_15 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_9.fld4.fld0 = [false,true,false,false,false,false,true,false];
match _7.1 {
0 => bb4,
1 => bb5,
2 => bb6,
340282366920938463463374607431559838105 => bb8,
_ => bb7
}
}
bb14 = {
_14 = _19;
_23 = _9.fld1 | _9.fld1;
_7.1 = 1327065317_i32;
_9.fld3 = (-24_i8);
_26 = _12;
_7 = (295430939962191484981966391482141499367_u128, 68061153_i32);
_9.fld4.fld3 = [_7.1,_7.1,_7.1,_7.1,_7.1,_7.1,_7.1];
_7.1 = -(-1024622205_i32);
_16 = _18;
_7 = (39110259554950504839589223612010065857_u128, 512347297_i32);
_9.fld0 = _6;
_20 = [0_usize,16579648138522024949_usize];
_23 = _9.fld1 % _9.fld1;
_1 = [_7.1,_7.1,_7.1,_7.1,_7.1];
_12 = _26;
_24 = [_16,_16,_18,_18,_18,_16,_16,_16];
_20 = [10744421668019318943_usize,10440201344567428233_usize];
_9.fld4.fld3 = [_7.1,_7.1,_7.1,_7.1,_7.1,_7.1,_7.1];
_9.fld4.fld1 = [_25,_11,_25];
_1 = [_7.1,_7.1,_7.1,_7.1,_7.1];
_13 = _12;
_23 = !_9.fld1;
_9.fld4.fld2.1 = [_13,_12,_12,_26,_26,_13,_12];
_9.fld0 = _6 | _6;
_16 = !_18;
_5 = Adt52::Variant0 { fld0: _9.fld4.fld1 };
_9.fld1 = _23;
SetDiscriminant(_5, 1);
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(10_usize, 25_usize, Move(_25), 15_usize, Move(_15), 20_usize, Move(_20), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(10_usize, 2_usize, Move(_2), 21_usize, Move(_21), 11_usize, Move(_11), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(10_usize, 18_usize, Move(_18), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [i32; 7],mut _2: [bool; 6]) -> [bool; 6] {
mir! {
type RET = [bool; 6];
let _3: [char; 6];
let _4: *const i128;
let _5: Adt50;
let _6: isize;
let _7: isize;
let _8: usize;
let _9: Adt50;
let _10: f64;
let _11: isize;
let _12: [u128; 4];
let _13: Adt46;
let _14: u128;
let _15: Adt51;
let _16: i64;
let _17: Adt51;
let _18: Adt52;
let _19: bool;
let _20: (f64, u64, i32, f64);
let _21: i8;
let _22: Adt59;
let _23: isize;
let _24: (u128, i32);
let _25: Adt51;
let _26: (i32, u128);
let _27: Adt49;
let _28: u64;
let _29: isize;
let _30: *const f64;
let _31: f32;
let _32: isize;
let _33: Adt61;
let _34: Adt48;
let _35: [char; 7];
let _36: ();
let _37: ();
{
RET = _2;
_1 = [555178701_i32,(-712943235_i32),1151229944_i32,(-607981635_i32),154700774_i32,(-408783273_i32),(-916216391_i32)];
RET = [false,true,true,true,false,false];
RET = _2;
_3 = ['\u{c3431}','\u{5644d}','\u{d0021}','\u{4cb7e}','\u{e7040}','\u{7ed25}'];
_1 = [1582126519_i32,668921132_i32,(-561402269_i32),369425232_i32,571461390_i32,1377341621_i32,(-203949277_i32)];
RET = [false,false,false,true,false,true];
_2 = [false,false,true,true,true,true];
RET = _2;
RET = [false,true,true,false,true,true];
_2 = [true,true,false,false,true,true];
RET = [true,false,true,true,true,false];
RET = [true,false,false,true,true,true];
_1 = [(-1652485512_i32),327722047_i32,460570625_i32,713254523_i32,1718011254_i32,(-1501078376_i32),1619880954_i32];
_1 = [308899795_i32,792841869_i32,1687213494_i32,249870425_i32,64503924_i32,838184664_i32,151066548_i32];
Goto(bb1)
}
bb1 = {
_7 = 80_u8 as isize;
_6 = '\u{10c8b9}' as isize;
RET = [true,true,true,true,true,true];
_2 = [false,false,false,true,true,true];
_2 = [true,true,true,true,true,true];
_6 = 4816265249400493396_u64 as isize;
Goto(bb2)
}
bb2 = {
RET = [false,true,false,true,false,true];
RET = [true,false,true,true,false,true];
RET = _2;
_1 = [1256124859_i32,1761134162_i32,(-2122813946_i32),(-1731514035_i32),(-1426167750_i32),(-915679302_i32),1727377512_i32];
RET = [false,true,false,false,true,false];
RET = _2;
_10 = (-10044_i16) as f64;
_12 = [202489317803711162861769728396469333441_u128,101534830169192532925328221041503497541_u128,208944818352397040880056991358231901601_u128,159012596029876433277719564940841122670_u128];
_11 = !_7;
_2 = [false,false,false,false,false,true];
_10 = 29482_u16 as f64;
Goto(bb3)
}
bb3 = {
_1 = [(-2087544873_i32),(-1210000277_i32),2009907357_i32,1000696344_i32,1969636848_i32,(-1219763486_i32),1896148507_i32];
_3 = ['\u{47ca7}','\u{49988}','\u{17d18}','\u{54419}','\u{5a4d}','\u{e0fd1}'];
_2 = [true,false,true,true,false,false];
_8 = 8164820023443338680_usize & 1_usize;
_2 = RET;
_7 = !_11;
_10 = 41561664782730700590320248582967080472_i128 as f64;
_8 = 94_u8 as usize;
_1 = [802030666_i32,(-1406750711_i32),(-1179401871_i32),237498711_i32,(-745552517_i32),699692422_i32,1663670807_i32];
_8 = (-91_i8) as usize;
_2 = [true,false,true,false,false,true];
Call(_4 = fn12(RET, _10, _8, _12), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = [false,true,true,false,false,false];
RET = [true,false,true,true,true,false];
_13.fld4 = core::ptr::addr_of!(_10);
_13.fld3 = [(-21117_i16),(-1542_i16)];
_13.fld6 = !6218625950611525843_u64;
_12 = [255830126493875551177119502201523465683_u128,232601136760564883052834272688121268175_u128,157975589315056793767719671402833020501_u128,287028910944223897267126250205493148338_u128];
_13.fld7 = _2;
_16 = 4473874571915804588_i64 - (-5283421801391644826_i64);
_8 = false as usize;
_13.fld7 = [false,true,false,false,true,true];
_13.fld6 = _16 as u64;
RET = [true,false,true,true,true,false];
RET = [true,false,false,true,true,true];
_14 = _10 as u128;
Goto(bb5)
}
bb5 = {
_13.fld7 = RET;
_7 = _6;
_11 = _6 >> _13.fld6;
_19 = true;
_8 = 17863195330725852967_usize;
_8 = 15142310924065863868_usize ^ 3_usize;
_16 = _13.fld6 as i64;
_13.fld7 = RET;
_16 = !(-8259591890676691203_i64);
_14 = '\u{15efa}' as u128;
_20.3 = -_10;
_20 = (_10, _13.fld6, (-1458545027_i32), _10);
_7 = _19 as isize;
_20.0 = -_10;
_13.fld3 = [(-10662_i16),(-13026_i16)];
_7 = _19 as isize;
_20.1 = !_13.fld6;
_16 = _8 as i64;
_13.fld0 = _19;
_20.1 = _13.fld6;
_11 = !_6;
_8 = !5778298741965196615_usize;
_6 = _11 * _11;
_16 = -2076546907055215414_i64;
_8 = _20.3 as usize;
match _20.2 {
0 => bb6,
1 => bb7,
2 => bb8,
340282366920938463463374607430309666429 => bb10,
_ => bb9
}
}
bb6 = {
RET = [false,true,true,false,false,false];
RET = [true,false,true,true,true,false];
_13.fld4 = core::ptr::addr_of!(_10);
_13.fld3 = [(-21117_i16),(-1542_i16)];
_13.fld6 = !6218625950611525843_u64;
_12 = [255830126493875551177119502201523465683_u128,232601136760564883052834272688121268175_u128,157975589315056793767719671402833020501_u128,287028910944223897267126250205493148338_u128];
_13.fld7 = _2;
_16 = 4473874571915804588_i64 - (-5283421801391644826_i64);
_8 = false as usize;
_13.fld7 = [false,true,false,false,true,true];
_13.fld6 = _16 as u64;
RET = [true,false,true,true,true,false];
RET = [true,false,false,true,true,true];
_14 = _10 as u128;
Goto(bb5)
}
bb7 = {
_1 = [(-2087544873_i32),(-1210000277_i32),2009907357_i32,1000696344_i32,1969636848_i32,(-1219763486_i32),1896148507_i32];
_3 = ['\u{47ca7}','\u{49988}','\u{17d18}','\u{54419}','\u{5a4d}','\u{e0fd1}'];
_2 = [true,false,true,true,false,false];
_8 = 8164820023443338680_usize & 1_usize;
_2 = RET;
_7 = !_11;
_10 = 41561664782730700590320248582967080472_i128 as f64;
_8 = 94_u8 as usize;
_1 = [802030666_i32,(-1406750711_i32),(-1179401871_i32),237498711_i32,(-745552517_i32),699692422_i32,1663670807_i32];
_8 = (-91_i8) as usize;
_2 = [true,false,true,false,false,true];
Call(_4 = fn12(RET, _10, _8, _12), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
RET = [false,true,false,true,false,true];
RET = [true,false,true,true,false,true];
RET = _2;
_1 = [1256124859_i32,1761134162_i32,(-2122813946_i32),(-1731514035_i32),(-1426167750_i32),(-915679302_i32),1727377512_i32];
RET = [false,true,false,false,true,false];
RET = _2;
_10 = (-10044_i16) as f64;
_12 = [202489317803711162861769728396469333441_u128,101534830169192532925328221041503497541_u128,208944818352397040880056991358231901601_u128,159012596029876433277719564940841122670_u128];
_11 = !_7;
_2 = [false,false,false,false,false,true];
_10 = 29482_u16 as f64;
Goto(bb3)
}
bb9 = {
_7 = 80_u8 as isize;
_6 = '\u{10c8b9}' as isize;
RET = [true,true,true,true,true,true];
_2 = [false,false,false,true,true,true];
_2 = [true,true,true,true,true,true];
_6 = 4816265249400493396_u64 as isize;
Goto(bb2)
}
bb10 = {
RET = [_19,_13.fld0,_19,_19,_13.fld0,_19];
_11 = _13.fld6 as isize;
_10 = _7 as f64;
_21 = !48_i8;
_13.fld4 = core::ptr::addr_of!(_20.0);
_1 = [_20.2,_20.2,_20.2,_20.2,_20.2,_20.2,_20.2];
_12 = [_14,_14,_14,_14];
_11 = _20.1 as isize;
_12 = [_14,_14,_14,_14];
RET = _13.fld7;
_10 = _20.0;
_13.fld3 = [4182_i16,20147_i16];
_3 = ['\u{3032b}','\u{83241}','\u{de14d}','\u{1afae}','\u{34182}','\u{79626}'];
_13.fld0 = _20.2 <= _20.2;
_7 = _13.fld0 as isize;
_20.1 = _13.fld6 | _13.fld6;
_20.3 = _20.0;
_24.1 = _20.2;
RET = [_13.fld0,_13.fld0,_13.fld0,_13.fld0,_13.fld0,_13.fld0];
Goto(bb11)
}
bb11 = {
_13.fld4 = core::ptr::addr_of!(_20.0);
_20.0 = _10 + _10;
_26 = (_20.2, _14);
_20.2 = -_26.0;
_13.fld2 = _8 as isize;
_26 = (_20.2, _14);
_13.fld2 = _6;
_12 = [_14,_14,_14,_14];
_24.0 = _24.1 as u128;
_20.0 = -_20.3;
match _24.1 {
0 => bb5,
1 => bb6,
340282366920938463463374607430309666429 => bb12,
_ => bb3
}
}
bb12 = {
_12 = [_24.0,_24.0,_24.0,_26.1];
_26 = (_20.2, _14);
_23 = _7;
Goto(bb13)
}
bb13 = {
_30 = core::ptr::addr_of!(_20.0);
RET = [_13.fld0,_13.fld0,_13.fld0,_13.fld0,_13.fld0,_13.fld0];
_12 = [_26.1,_24.0,_24.0,_24.0];
_26.0 = _24.1 ^ _24.1;
Goto(bb14)
}
bb14 = {
_13.fld6 = 27577_u16 as u64;
_8 = 48_u8 as usize;
_23 = 425074581_u32 as isize;
_10 = -_20.3;
_28 = _20.1 & _20.1;
_13.fld0 = _19;
_20.1 = !_28;
_20 = (_10, _28, _26.0, _10);
RET = _2;
_20.2 = _28 as i32;
_24 = (_26.1, _26.0);
_7 = _6;
_20.0 = _20.3 + _20.3;
_13.fld2 = -_23;
_21 = 108_i8 & (-122_i8);
_33.fld4.fld0 = [_13.fld0,_13.fld0,_19,_13.fld0,_19,_19,_13.fld0,_13.fld0];
_2 = RET;
_20.1 = _28 << _20.2;
_24.1 = _20.2 + _26.0;
_2 = _13.fld7;
_33.fld4.fld2.2 = [677257033_u32,3074061356_u32,3341495671_u32];
_34.fld2.0 = core::ptr::addr_of!(_31);
_34.fld2.0 = core::ptr::addr_of!(_31);
_34.fld0 = _33.fld4.fld0;
_13.fld4 = core::ptr::addr_of!(_20.0);
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(11_usize, 3_usize, Move(_3), 2_usize, Move(_2), 12_usize, Move(_12), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(11_usize, 16_usize, Move(_16), 8_usize, Move(_8), 11_usize, Move(_11), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: [bool; 6],mut _2: f64,mut _3: usize,mut _4: [u128; 4]) -> *const i128 {
mir! {
type RET = *const i128;
let _5: char;
let _6: [i32; 7];
let _7: i128;
let _8: char;
let _9: f64;
let _10: isize;
let _11: f64;
let _12: Adt51;
let _13: Adt56;
let _14: [u32; 1];
let _15: f64;
let _16: isize;
let _17: [u32; 1];
let _18: [u128; 4];
let _19: [usize; 2];
let _20: f64;
let _21: [char; 7];
let _22: u16;
let _23: (i32, u128);
let _24: f64;
let _25: [char; 7];
let _26: i64;
let _27: i32;
let _28: ([isize; 2], usize, i64, u8);
let _29: Adt51;
let _30: *const i16;
let _31: Adt57;
let _32: f64;
let _33: ([usize; 2], i16);
let _34: [u32; 3];
let _35: ([isize; 2], usize, i64, u8);
let _36: Adt49;
let _37: ([isize; 2], usize, i64, u8);
let _38: [char; 6];
let _39: u32;
let _40: f64;
let _41: [i16; 2];
let _42: char;
let _43: [u64; 3];
let _44: isize;
let _45: f32;
let _46: [bool; 8];
let _47: ();
let _48: ();
{
_3 = 3726951660856504525_usize;
_2 = 2503393148147542519_u64 as f64;
_3 = 17341236770791440977_usize >> 2627859898064732312_u64;
_1 = [false,true,false,true,false,false];
_4 = [4249330747299244663559811622821084237_u128,18511225213624709674098085615061826478_u128,102799699423437214357283145753409180270_u128,100774413763833277450547062427715510809_u128];
_5 = '\u{106e06}';
_1 = [true,false,true,false,true,true];
_2 = 553999520922759065_u64 as f64;
_1 = [false,true,true,true,false,false];
_4 = [139511534970130048844812469176676945935_u128,318753652254297041913802442808663086346_u128,7922374127582666089139652668987735042_u128,109987228191936981697912433647912440602_u128];
_1 = [true,true,false,false,true,true];
_1 = [false,true,false,false,false,false];
_2 = 51_i8 as f64;
_2 = 4451270884036403493_u64 as f64;
_5 = '\u{1ce73}';
_4 = [21280432517739128613788481319531540454_u128,69045723006009983918587475255962532144_u128,110098088467486684207710644450583695826_u128,3429066866105521978428148924095838123_u128];
_4 = [50584643379713521198147110287866399023_u128,42113008605622428424766617389589732679_u128,278123495718092533009384233931627854795_u128,283612503424537180476986978071501465394_u128];
_4 = [173362686540457720606962903971305825723_u128,227588580432558322589584803916160307611_u128,210847597740547989838963895973637335007_u128,271581494430788226584506263008952716804_u128];
Call(_3 = core::intrinsics::bswap(3_usize), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = [1369711278_i32,(-1831226940_i32),(-1485046952_i32),(-1779129467_i32),(-665714877_i32),882491737_i32,163314474_i32];
_2 = (-46980916689845101481351658921029059151_i128) as f64;
_4 = [295902149949009208297141955371858531661_u128,187472411425438513002425295958991855672_u128,93021654859151432817368471671064115642_u128,232980029108105146884588814425729973840_u128];
_3 = !3_usize;
RET = core::ptr::addr_of!(_7);
_8 = _5;
_1 = [true,true,true,false,false,true];
RET = core::ptr::addr_of!((*RET));
_4 = [335907593168342121298928838979473124676_u128,36648049211350419781294951648889620689_u128,47447017845564595768544862236048935457_u128,299509335905730356373057193077431879051_u128];
(*RET) = _2 as i128;
(*RET) = 141853688799089882652323706569530338855_i128 & (-45727972871603575719484870907419253797_i128);
(*RET) = (-48_i8) as i128;
RET = core::ptr::addr_of!((*RET));
(*RET) = false as i128;
_5 = _8;
Call(_1 = fn13(_4, _6, _4, _6, _6, _2, _6, RET, _6, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = [216339180868003826470694506266579775243_u128,166387235456321179744357152049501490961_u128,331958953189311890064345117995228593121_u128,243050012772046048745497581211760728320_u128];
_7 = 99_u8 as i128;
_8 = _5;
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!(_7);
_3 = 6_usize;
(*RET) = !(-10784791279722785670795026959096530046_i128);
Goto(bb3)
}
bb3 = {
_7 = -146141266566417604995724549001290437239_i128;
_4 = [234751859415454155386379990805435670113_u128,178439083870428829590311152675287861964_u128,115446057724499379904874332850024407618_u128,142498833584159994778951408920841554871_u128];
_1 = [true,true,true,false,true,true];
(*RET) = (-109538372522270740156019707540036530945_i128) << _6[_3];
_11 = _2;
_9 = (-6009_i16) as f64;
(*RET) = -18209351698235337285952081863182790007_i128;
_6[_3] = -(-557023772_i32);
_10 = (-51_isize) * (-9223372036854775808_isize);
_6 = [(-817182625_i32),47340108_i32,371069164_i32,975447060_i32,(-1236348617_i32),(-942492595_i32),96183645_i32];
(*RET) = 27560608614007739377585677371276932603_i128 << _6[_3];
_1 = [true,true,true,true,true,false];
_7 = 135819922866545052087376170946498106880_i128;
_8 = _5;
_1 = [true,true,false,true,false,true];
_3 = 13538455173911517845_usize + 7_usize;
_14 = [2197090875_u32];
_16 = -_10;
_7 = (-129650685683225230091510821408090692790_i128);
_8 = _5;
_15 = 17188017762534251192_u64 as f64;
_9 = -_11;
_17 = [2387205584_u32];
Goto(bb4)
}
bb4 = {
_9 = _2;
_8 = _5;
_3 = 6973271664602617995_usize;
_3 = 2_usize;
_10 = -_16;
_4[_3] = !272850443433860561082503325642362676196_u128;
_10 = !_16;
_16 = -_10;
_1[_3] = true;
RET = core::ptr::addr_of!(_7);
_5 = _8;
_9 = (*RET) as f64;
_16 = 98_u8 as isize;
_8 = _5;
_6 = [1954509869_i32,2066495493_i32,120578525_i32,(-757033022_i32),(-997261225_i32),(-560164872_i32),(-1552020035_i32)];
_20 = -_11;
_3 = (-23_i8) as usize;
_11 = -_9;
_8 = _5;
_19 = [_3,_3];
(*RET) = (-370243005599858089_i64) as i128;
(*RET) = 161101992873050057942334572915226630459_i128;
(*RET) = 8423466224623127559454344955479684283_i128;
_4 = [140004970319032728204762241173985495103_u128,41025816033473463558529270812261057578_u128,297570736771891165844976748379602892873_u128,72539127697147283578780651946722843908_u128];
_10 = _3 as isize;
Goto(bb5)
}
bb5 = {
RET = core::ptr::addr_of!((*RET));
_7 = !(-108297329974919001076716354185959066080_i128);
(*RET) = _3 as i128;
_18 = [217824923372311488238610157495791290407_u128,202643122864902847954804615447083509190_u128,96339505834706687060552809854156868512_u128,144574636107240607913627230733785366976_u128];
_21 = [_8,_5,_8,_5,_5,_5,_8];
RET = core::ptr::addr_of!(_7);
Goto(bb6)
}
bb6 = {
_1 = [true,true,false,true,true,true];
(*RET) = (-60821745550917984294833154633060083953_i128);
_16 = 26420_u16 as isize;
_20 = -_11;
_5 = _8;
_19 = [_3,_3];
_25 = _21;
_2 = _9;
_10 = (*RET) as isize;
_16 = !_10;
_5 = _8;
_21 = [_5,_8,_8,_8,_8,_5,_5];
RET = core::ptr::addr_of!((*RET));
match (*RET) {
0 => bb3,
279460621370020479168541452798708127503 => bb7,
_ => bb2
}
}
bb7 = {
_28.3 = 37_u8 << _16;
_23.1 = 118991623497780299357408094526154360541_u128;
(*RET) = -(-36982403714283671872286378713134952325_i128);
_16 = 927808032_i32 as isize;
_23 = ((-186044158_i32), 75753199094449029802393386826322728638_u128);
_23 = (1990209430_i32, 1317439763683744687649387032891235630_u128);
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!((*RET));
_28.1 = _7 as usize;
_23.0 = (-669084633_i32) ^ 10916374_i32;
_23 = ((-773440861_i32), 179421275600477685401664141526981019864_u128);
_28.2 = 6263540628868581635_i64;
_8 = _5;
_19 = [_28.1,_3];
_17 = [2718418954_u32];
_28.0 = [_16,_10];
_21 = [_8,_8,_5,_5,_5,_5,_5];
_6 = [_23.0,_23.0,_23.0,_23.0,_23.0,_23.0,_23.0];
_5 = _8;
_4 = [_23.1,_23.1,_23.1,_23.1];
_24 = _28.2 as f64;
_16 = _10 << _28.3;
_22 = 5762_u16;
match _22 {
0 => bb8,
5762 => bb10,
_ => bb9
}
}
bb8 = {
_7 = -146141266566417604995724549001290437239_i128;
_4 = [234751859415454155386379990805435670113_u128,178439083870428829590311152675287861964_u128,115446057724499379904874332850024407618_u128,142498833584159994778951408920841554871_u128];
_1 = [true,true,true,false,true,true];
(*RET) = (-109538372522270740156019707540036530945_i128) << _6[_3];
_11 = _2;
_9 = (-6009_i16) as f64;
(*RET) = -18209351698235337285952081863182790007_i128;
_6[_3] = -(-557023772_i32);
_10 = (-51_isize) * (-9223372036854775808_isize);
_6 = [(-817182625_i32),47340108_i32,371069164_i32,975447060_i32,(-1236348617_i32),(-942492595_i32),96183645_i32];
(*RET) = 27560608614007739377585677371276932603_i128 << _6[_3];
_1 = [true,true,true,true,true,false];
_7 = 135819922866545052087376170946498106880_i128;
_8 = _5;
_1 = [true,true,false,true,false,true];
_3 = 13538455173911517845_usize + 7_usize;
_14 = [2197090875_u32];
_16 = -_10;
_7 = (-129650685683225230091510821408090692790_i128);
_8 = _5;
_15 = 17188017762534251192_u64 as f64;
_9 = -_11;
_17 = [2387205584_u32];
Goto(bb4)
}
bb9 = {
_9 = _2;
_8 = _5;
_3 = 6973271664602617995_usize;
_3 = 2_usize;
_10 = -_16;
_4[_3] = !272850443433860561082503325642362676196_u128;
_10 = !_16;
_16 = -_10;
_1[_3] = true;
RET = core::ptr::addr_of!(_7);
_5 = _8;
_9 = (*RET) as f64;
_16 = 98_u8 as isize;
_8 = _5;
_6 = [1954509869_i32,2066495493_i32,120578525_i32,(-757033022_i32),(-997261225_i32),(-560164872_i32),(-1552020035_i32)];
_20 = -_11;
_3 = (-23_i8) as usize;
_11 = -_9;
_8 = _5;
_19 = [_3,_3];
(*RET) = (-370243005599858089_i64) as i128;
(*RET) = 161101992873050057942334572915226630459_i128;
(*RET) = 8423466224623127559454344955479684283_i128;
_4 = [140004970319032728204762241173985495103_u128,41025816033473463558529270812261057578_u128,297570736771891165844976748379602892873_u128,72539127697147283578780651946722843908_u128];
_10 = _3 as isize;
Goto(bb5)
}
bb10 = {
RET = core::ptr::addr_of!((*RET));
_21 = [_8,_8,_5,_5,_5,_5,_5];
_25 = _21;
_27 = (-80_i8) as i32;
_33 = (_19, (-23646_i16));
_24 = -_11;
_33.1 = (-15457_i16);
_33 = (_19, 14824_i16);
_9 = _22 as f64;
_32 = _11 + _15;
_27 = _23.0;
_23.0 = -_27;
_20 = _9;
_26 = _28.2 ^ _28.2;
_23.0 = _27;
(*RET) = -140656694973655550902592154498971288147_i128;
_21 = _25;
_33 = (_19, 25770_i16);
_6 = [_27,_27,_27,_27,_27,_27,_27];
_23.0 = _16 as i32;
RET = core::ptr::addr_of!(_7);
_1 = [false,true,true,true,true,true];
_33.1 = 15730_i16;
_28.1 = _3;
_3 = !_28.1;
_10 = !_16;
_7 = _3 as i128;
_28.3 = 215_u8 & 41_u8;
Goto(bb11)
}
bb11 = {
_34 = [1455642259_u32,2009842346_u32,1291822972_u32];
_1 = [true,false,true,false,false,false];
RET = core::ptr::addr_of!(_7);
_28.2 = _26 >> _23.1;
_17 = [2195268202_u32];
_28.0 = [_16,_16];
_16 = _10;
_9 = -_24;
_27 = _20 as i32;
_28.0 = [_10,_10];
_33.1 = (-10327_i16) & 31827_i16;
_1 = [false,false,false,true,false,true];
(*RET) = 12286387024321707453168150803632305799_i128;
_18 = [_23.1,_23.1,_23.1,_23.1];
_33.1 = -(-14952_i16);
Goto(bb12)
}
bb12 = {
_10 = _16 << _28.2;
_20 = _32 - _11;
_8 = _5;
_26 = _33.1 as i64;
_16 = 1159003557_u32 as isize;
_35.2 = _28.2;
Goto(bb13)
}
bb13 = {
_35.1 = _22 as usize;
_4 = _18;
_1 = [true,true,true,true,true,false];
_20 = _15;
RET = core::ptr::addr_of!((*RET));
_14 = [113669715_u32];
_35 = _28;
_28.1 = _35.1 << _35.2;
_3 = _28.1 | _28.1;
_28.3 = !_35.3;
_15 = _24;
_2 = _28.1 as f64;
_37 = (_35.0, _3, _35.2, _35.3);
RET = core::ptr::addr_of!(_7);
_35 = _28;
_23 = (_27, 296047983118072195220406226572826183254_u128);
(*RET) = !47051226500480460123059593142281704872_i128;
_17 = _14;
(*RET) = (-126417879654465082679469448568484335401_i128);
Goto(bb14)
}
bb14 = {
_33.1 = 14619586692912103240_u64 as i16;
_23.1 = _27 as u128;
_23 = (_27, 30081470358667778291244975756883489909_u128);
_30 = core::ptr::addr_of!(_33.1);
RET = core::ptr::addr_of!((*RET));
(*RET) = -(-119494515091521065255613273440224102630_i128);
_35.3 = _28.3 + _37.3;
(*RET) = _27 as i128;
_20 = _24;
_11 = _2;
_30 = core::ptr::addr_of!((*_30));
_37.1 = _3 >> _10;
_42 = _5;
_23 = (_27, 110141248272214039706570310677506708333_u128);
_21 = [_42,_8,_42,_8,_8,_5,_8];
(*RET) = -(-33693573969392446260209988027570016413_i128);
_43 = [16585792537136332124_u64,6706561934972066831_u64,17642610004598503861_u64];
_37.0 = [_10,_10];
_25 = [_5,_42,_5,_42,_8,_42,_42];
_33.0 = [_3,_37.1];
_28 = (_35.0, _37.1, _37.2, _35.3);
Goto(bb15)
}
bb15 = {
Call(_47 = dump_var(12_usize, 26_usize, Move(_26), 1_usize, Move(_1), 6_usize, Move(_6), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_47 = dump_var(12_usize, 22_usize, Move(_22), 33_usize, Move(_33), 16_usize, Move(_16), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(12_usize, 27_usize, Move(_27), 3_usize, Move(_3), 25_usize, Move(_25), 37_usize, Move(_37)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_47 = dump_var(12_usize, 21_usize, Move(_21), 48_usize, _48, 48_usize, _48, 48_usize, _48), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: [u128; 4],mut _2: [i32; 7],mut _3: [u128; 4],mut _4: [i32; 7],mut _5: [i32; 7],mut _6: f64,mut _7: [i32; 7],mut _8: *const i128,mut _9: [i32; 7],mut _10: [i32; 7]) -> [bool; 6] {
mir! {
type RET = [bool; 6];
let _11: Adt61;
let _12: Adt49;
let _13: f64;
let _14: char;
let _15: isize;
let _16: *mut isize;
let _17: isize;
let _18: i64;
let _19: Adt55;
let _20: isize;
let _21: Adt45;
let _22: isize;
let _23: f32;
let _24: [u32; 1];
let _25: u128;
let _26: i64;
let _27: Adt50;
let _28: ();
let _29: ();
{
_4 = [(-1639541725_i32),(-310104493_i32),1893511089_i32,(-1723390730_i32),(-791188845_i32),1782248183_i32,(-1093156162_i32)];
_5 = [83157928_i32,(-215682240_i32),1759015039_i32,536161942_i32,754251888_i32,672276793_i32,1555564797_i32];
_3 = [109355846095460641488936178887071150377_u128,332356278115427645642678207427800821279_u128,144845151422935890743222514558907934630_u128,308947513502624811076951278375826534440_u128];
_7 = [1994622801_i32,(-159842775_i32),403251481_i32,2095371153_i32,1885204772_i32,1709924762_i32,1107355965_i32];
RET = [true,false,false,true,false,true];
_7 = [1104279950_i32,(-1182834437_i32),20660741_i32,2052472319_i32,(-622886516_i32),950147623_i32,180319337_i32];
_1 = [31838571579788208670574790531073247047_u128,242994773942812708535108719241575073052_u128,148032375966220945118807355566181227945_u128,327743769712034063430424564270415771923_u128];
RET = [true,false,true,false,false,false];
RET = [false,true,false,true,false,true];
_11.fld1 = 3469088249074589982_u64 * 1729647999768550760_u64;
_9 = [(-574446430_i32),1715681852_i32,979944254_i32,(-1128556615_i32),1153975010_i32,(-1532547256_i32),149148998_i32];
_11.fld6 = core::ptr::addr_of!((*_8));
_11.fld4.fld0 = [true,false,false,false,false,true,true,true];
_11.fld4.fld1 = [819699982_u32,3325022313_u32,772471939_u32];
_3 = _1;
_11.fld4.fld2.2 = [1349444752_u32,364694541_u32,1836909122_u32];
RET = [true,false,false,false,false,true];
_11.fld4.fld2.2 = _11.fld4.fld1;
_11.fld3 = _6 as i8;
_7 = [(-1833392205_i32),(-1934637573_i32),1146547826_i32,898381741_i32,(-1509882187_i32),1284564785_i32,261554940_i32];
_12 = Adt49::Variant1 { fld0: 211_u8 };
_11.fld4.fld2.1 = ['\u{69ec6}','\u{3d985}','\u{b3105}','\u{9634b}','\u{5dfea}','\u{6f5e1}','\u{ffce4}'];
_12 = Adt49::Variant0 { fld0: _3 };
_1 = [329686631649338884255313267997579341920_u128,143682501719167014755939690711894148204_u128,221117548810350266952089303896563684915_u128,325428854380138953450688361877829642838_u128];
_11.fld4.fld1 = _11.fld4.fld2.2;
_7 = [597309660_i32,1827604400_i32,(-1066766384_i32),126607731_i32,(-288564337_i32),1396505240_i32,259020072_i32];
Goto(bb1)
}
bb1 = {
_6 = 13896_u16 as f64;
_2 = [846721598_i32,(-1831104215_i32),1154841941_i32,1460518923_i32,(-122599369_i32),1375220845_i32,(-1496026232_i32)];
_11.fld0 = true as u8;
RET = [false,true,true,true,false,true];
(*_8) = !55583954867498043317583494497413542924_i128;
_11.fld4.fld0 = [false,true,true,true,true,true,true,false];
place!(Field::<[u128; 4]>(Variant(_12, 0), 0)) = _3;
_11.fld6 = core::ptr::addr_of!((*_8));
_15 = (-1_isize);
_11.fld6 = core::ptr::addr_of!((*_8));
_9 = [(-1360961991_i32),(-1584772056_i32),1261040026_i32,(-1100028119_i32),(-861866395_i32),2064921326_i32,2123147942_i32];
SetDiscriminant(_12, 0);
_11.fld4.fld3 = _10;
(*_8) = _11.fld3 as i128;
match _15 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463463374607431768211455 => bb10,
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
_11.fld4.fld3 = [1825872584_i32,(-476847002_i32),(-1421901057_i32),981202782_i32,(-2023411047_i32),(-1410512908_i32),(-1365945830_i32)];
_11.fld1 = 14633555337138924313_u64 + 10997694211805138199_u64;
_11.fld3 = (-37_i8) | (-2_i8);
_14 = '\u{f551}';
_1 = [152617898156816972810935142707603961739_u128,237924628789545986059797847304542370893_u128,172865113972612431246621077007194739045_u128,232189516675234305369697655844460570923_u128];
place!(Field::<[u128; 4]>(Variant(_12, 0), 0)) = [22236824381182784812077445445604690958_u128,126527999020193736282732236664950506675_u128,239797081328860487543143296645517641900_u128,284821220395826990678624193718600702237_u128];
_11.fld6 = core::ptr::addr_of!((*_8));
_6 = _11.fld0 as f64;
_13 = (*_8) as f64;
_10 = [(-1636170136_i32),1333497360_i32,(-225717270_i32),834206636_i32,905653757_i32,(-588603923_i32),(-653019564_i32)];
_11.fld2 = core::ptr::addr_of_mut!(_15);
Call(_9 = fn14(_10, Field::<[u128; 4]>(Variant(_12, 0), 0), _11.fld4.fld0, _7, _8, _2, Move(_12), _11.fld4.fld3, RET, _1, _7, _11.fld2, _11.fld4.fld0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_17 = _15 - _15;
_11.fld6 = core::ptr::addr_of!((*_8));
_11.fld3 = !1_i8;
(*_8) = 229359362847454181239254638517689933295_u128 as i128;
_2 = [(-2007272387_i32),(-2113237646_i32),(-399696226_i32),(-687426939_i32),(-1354866261_i32),(-1693089301_i32),1121742604_i32];
_14 = '\u{673c4}';
_16 = _11.fld2;
(*_8) = !(-4916984044101305932555471910698448302_i128);
_7 = [836791974_i32,(-1374586780_i32),1927377731_i32,1978715702_i32,(-312377561_i32),(-744888822_i32),(-2054997558_i32)];
_3 = [22429685662155262554045948742946756022_u128,9095243397118107039759208969843297320_u128,227463826843669957636478078780366373749_u128,111997189420293338564907942140272062144_u128];
_11.fld4.fld2.1 = [_14,_14,_14,_14,_14,_14,_14];
_11.fld2 = _16;
_13 = _6 * _6;
_6 = _13 + _13;
_11.fld0 = 145_u8 >> _17;
_4 = [(-1232961373_i32),1204828663_i32,(-1984147283_i32),(-1491186603_i32),1444061409_i32,1658583390_i32,1144113699_i32];
_11.fld2 = core::ptr::addr_of_mut!(_17);
Call((*_16) = core::intrinsics::bswap(_17), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_11.fld6 = core::ptr::addr_of!((*_8));
_16 = core::ptr::addr_of_mut!(_17);
_13 = 5_usize as f64;
_14 = '\u{e65a8}';
(*_16) = _15 >> _15;
_2 = [100756451_i32,(-418604153_i32),238254667_i32,1949129969_i32,1204250075_i32,(-2131079840_i32),1542079550_i32];
_22 = (*_16);
_11.fld0 = !142_u8;
_13 = _6;
_11.fld4.fld3 = _10;
_11.fld4.fld2.0 = core::ptr::addr_of!(_23);
_20 = (*_8) as isize;
_14 = '\u{4bd2c}';
_12 = Adt49::Variant0 { fld0: _3 };
_11.fld4.fld2.0 = core::ptr::addr_of!(_23);
(*_8) = (-152903655587326830904123990372726886228_i128);
_23 = 1971585917_i32 as f32;
(*_8) = (-46156261781596907519115951050670991460_i128) << (*_16);
_11.fld6 = _8;
_11.fld6 = core::ptr::addr_of!((*_8));
_11.fld4.fld1 = [3171294357_u32,3280292247_u32,13390030_u32];
Goto(bb13)
}
bb13 = {
_26 = 3596844104678327668_i64 | (-6481194959905097088_i64);
_8 = _11.fld6;
_6 = _13;
_10 = [270814887_i32,665450576_i32,(-1151731975_i32),(-285688337_i32),(-963069027_i32),(-1070361583_i32),(-1655680203_i32)];
SetDiscriminant(_12, 3);
(*_16) = -_22;
(*_8) = (-90456524769109155828942445968173787737_i128) << (*_16);
Goto(bb14)
}
bb14 = {
_11.fld4.fld2.2 = _11.fld4.fld1;
_8 = _11.fld6;
_22 = _11.fld1 as isize;
_11.fld3 = 99_i8;
_11.fld4.fld0 = [true,true,false,true,true,false,true,true];
_24 = [2703044553_u32];
_3 = _1;
_17 = _20 + _20;
_18 = _26 * _26;
_14 = '\u{f92e}';
_5 = _2;
place!(Field::<u16>(Variant(_12, 3), 0)) = 28278_u16;
_13 = _6;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(13_usize, 2_usize, Move(_2), 22_usize, Move(_22), 5_usize, Move(_5), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(13_usize, 3_usize, Move(_3), 9_usize, Move(_9), 14_usize, Move(_14), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: [i32; 7],mut _2: [u128; 4],mut _3: [bool; 8],mut _4: [i32; 7],mut _5: *const i128,mut _6: [i32; 7],mut _7: Adt49,mut _8: [i32; 7],mut _9: [bool; 6],mut _10: [u128; 4],mut _11: [i32; 7],mut _12: *mut isize,mut _13: [bool; 8]) -> [i32; 7] {
mir! {
type RET = [i32; 7];
let _14: i16;
let _15: ([usize; 2], i16);
let _16: bool;
let _17: Adt60;
let _18: *mut [char; 7];
let _19: (*const f32, [char; 7], [u32; 3]);
let _20: [bool; 6];
let _21: u32;
let _22: i32;
let _23: isize;
let _24: *mut [char; 7];
let _25: (i32, u128);
let _26: Adt59;
let _27: Adt48;
let _28: u8;
let _29: i32;
let _30: (u128, i32);
let _31: u32;
let _32: [u32; 1];
let _33: u64;
let _34: *const f32;
let _35: u32;
let _36: [char; 7];
let _37: isize;
let _38: ();
let _39: ();
{
_3 = [true,true,false,true,false,true,true,false];
_3 = [false,true,true,false,false,false,false,false];
RET = [533288799_i32,(-1602467542_i32),1097253535_i32,1369362624_i32,1129418149_i32,1728885461_i32,(-200499195_i32)];
_10 = _2;
_8 = [1667522843_i32,1471551391_i32,501743433_i32,327711568_i32,2135428714_i32,799463623_i32,1041973775_i32];
_14 = (-21516_i16);
_8 = [(-212376164_i32),1577782762_i32,(-792397932_i32),(-613996730_i32),1310767287_i32,(-1538715916_i32),705129695_i32];
_15.1 = (*_12) as i16;
SetDiscriminant(_7, 0);
_7 = Adt49::Variant3 { fld0: 12374_u16 };
_7 = Adt49::Variant3 { fld0: 24194_u16 };
_4 = RET;
_10 = [208842466146771806670021102960240233163_u128,195159063480581503409196747887193725084_u128,86269912924626268907353140448828016580_u128,234319076837252676211841950220022915523_u128];
_15.1 = '\u{2c727}' as i16;
_15.1 = _14 >> _14;
_15.0 = [0_usize,1_usize];
(*_12) = _15.1 as isize;
_12 = core::ptr::addr_of_mut!((*_12));
_11 = [160673602_i32,(-188475576_i32),1367061780_i32,(-1510312640_i32),969409225_i32,1542226012_i32,(-92413416_i32)];
_16 = true;
(*_12) = 9223372036854775807_isize - (-35_isize);
_14 = -_15.1;
Goto(bb1)
}
bb1 = {
_5 = core::ptr::addr_of!((*_5));
_10 = _2;
_15.1 = '\u{f7042}' as i16;
_13 = [_16,_16,_16,_16,_16,_16,_16,_16];
_11 = [(-1506522588_i32),773296087_i32,(-165548040_i32),(-2075177423_i32),850746232_i32,1286129202_i32,1501001746_i32];
_15.1 = !_14;
_9 = [_16,_16,_16,_16,_16,_16];
_11 = _1;
place!(Field::<u16>(Variant(_7, 3), 0)) = !9758_u16;
_5 = core::ptr::addr_of!((*_5));
_14 = _15.1;
_6 = [1198031529_i32,(-1764799862_i32),(-181030158_i32),1944033624_i32,480403666_i32,(-1220663076_i32),1147091007_i32];
RET = [(-136969670_i32),179060702_i32,22356639_i32,(-406009220_i32),(-1690321109_i32),478697999_i32,(-351883581_i32)];
_1 = [518243798_i32,1946994484_i32,331959749_i32,887731421_i32,(-900888265_i32),959970577_i32,641502018_i32];
Call(_5 = fn15(_8, _11, _11, (*_5), (*_12), _10, _10, (*_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10 = [33053248108608591769703911460550230936_u128,171110047330058840994258919041561312016_u128,273363172461581205351399736809260483331_u128,325588151821477072216490995895496614389_u128];
_15.1 = _14;
_15.0 = [2_usize,3_usize];
_19.1 = ['\u{7693d}','\u{eef}','\u{11b82}','\u{8d8a5}','\u{c49a5}','\u{e1954}','\u{eb923}'];
_19.2 = [720973705_u32,163078951_u32,230614602_u32];
_19.1 = ['\u{d510a}','\u{102999}','\u{79a78}','\u{9b151}','\u{864f9}','\u{719db}','\u{21ac0}'];
_19.1 = ['\u{fbcfd}','\u{e0481}','\u{8ed20}','\u{63aa6}','\u{8ec1c}','\u{16480}','\u{6075c}'];
_18 = core::ptr::addr_of_mut!(_19.1);
(*_18) = ['\u{867ee}','\u{1f4db}','\u{bfc38}','\u{1ef2f}','\u{b0343}','\u{d7ad4}','\u{6fa21}'];
_18 = core::ptr::addr_of_mut!(_19.1);
_18 = core::ptr::addr_of_mut!((*_18));
_19.2 = [3801943794_u32,1235969077_u32,2100899312_u32];
_12 = core::ptr::addr_of_mut!((*_12));
_4 = RET;
_7 = Adt49::Variant3 { fld0: 30058_u16 };
(*_18) = ['\u{17e5}','\u{33b8e}','\u{67115}','\u{f109e}','\u{2bf0}','\u{454}','\u{ca14b}'];
_1 = [880167189_i32,1456339676_i32,1569425331_i32,(-17081668_i32),(-68245384_i32),(-985158925_i32),126150502_i32];
_14 = -_15.1;
_3 = _13;
_3 = [_16,_16,_16,_16,_16,_16,_16,_16];
Goto(bb3)
}
bb3 = {
RET = _4;
_15.1 = -_14;
_13 = [_16,_16,_16,_16,_16,_16,_16,_16];
(*_18) = ['\u{fd72e}','\u{f02e}','\u{f5e99}','\u{42390}','\u{4ecdc}','\u{7d5a}','\u{ca88}'];
_8 = [1708953055_i32,(-1463284975_i32),1360326860_i32,(-1039877116_i32),(-88844428_i32),(-1015657913_i32),820040453_i32];
_18 = core::ptr::addr_of_mut!((*_18));
_4 = _1;
Goto(bb4)
}
bb4 = {
_20 = [_16,_16,_16,_16,_16,_16];
_10 = [241269607138572566165265725707695574929_u128,71840059415038349095380112050632797746_u128,172131120554069357265631770014255747646_u128,184758647192193445800133029513618877897_u128];
_21 = 113580181_u32;
_22 = _16 as i32;
_24 = core::ptr::addr_of_mut!((*_18));
_11 = RET;
_2 = [130018682724199681796228107868336797903_u128,208872261437727280044195662540493252150_u128,6313024158982973584322406169191116853_u128,217612863512177411131628950355338259214_u128];
_18 = core::ptr::addr_of_mut!((*_18));
_9 = [_16,_16,_16,_16,_16,_16];
_8 = _4;
_18 = _24;
_7 = Adt49::Variant3 { fld0: 16423_u16 };
_9 = _20;
_15.1 = 46791_u16 as i16;
_16 = !true;
_15.1 = -_14;
(*_12) = 9223372036854775807_isize | (-9223372036854775808_isize);
match _21 {
0 => bb5,
113580181 => bb7,
_ => bb6
}
}
bb5 = {
RET = _4;
_15.1 = -_14;
_13 = [_16,_16,_16,_16,_16,_16,_16,_16];
(*_18) = ['\u{fd72e}','\u{f02e}','\u{f5e99}','\u{42390}','\u{4ecdc}','\u{7d5a}','\u{ca88}'];
_8 = [1708953055_i32,(-1463284975_i32),1360326860_i32,(-1039877116_i32),(-88844428_i32),(-1015657913_i32),820040453_i32];
_18 = core::ptr::addr_of_mut!((*_18));
_4 = _1;
Goto(bb4)
}
bb6 = {
_5 = core::ptr::addr_of!((*_5));
_10 = _2;
_15.1 = '\u{f7042}' as i16;
_13 = [_16,_16,_16,_16,_16,_16,_16,_16];
_11 = [(-1506522588_i32),773296087_i32,(-165548040_i32),(-2075177423_i32),850746232_i32,1286129202_i32,1501001746_i32];
_15.1 = !_14;
_9 = [_16,_16,_16,_16,_16,_16];
_11 = _1;
place!(Field::<u16>(Variant(_7, 3), 0)) = !9758_u16;
_5 = core::ptr::addr_of!((*_5));
_14 = _15.1;
_6 = [1198031529_i32,(-1764799862_i32),(-181030158_i32),1944033624_i32,480403666_i32,(-1220663076_i32),1147091007_i32];
RET = [(-136969670_i32),179060702_i32,22356639_i32,(-406009220_i32),(-1690321109_i32),478697999_i32,(-351883581_i32)];
_1 = [518243798_i32,1946994484_i32,331959749_i32,887731421_i32,(-900888265_i32),959970577_i32,641502018_i32];
Call(_5 = fn15(_8, _11, _11, (*_5), (*_12), _10, _10, (*_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_12 = core::ptr::addr_of_mut!((*_12));
_8 = [_22,_22,_22,_22,_22,_22,_22];
_29 = -_22;
_2 = _10;
RET = [_29,_29,_22,_29,_29,_29,_29];
place!(Field::<u16>(Variant(_7, 3), 0)) = 36887_u16;
RET = [_22,_22,_22,_22,_29,_29,_22];
_12 = core::ptr::addr_of_mut!((*_12));
match Field::<u16>(Variant(_7, 3), 0) {
0 => bb3,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
36887 => bb14,
_ => bb13
}
}
bb8 = {
_5 = core::ptr::addr_of!((*_5));
_10 = _2;
_15.1 = '\u{f7042}' as i16;
_13 = [_16,_16,_16,_16,_16,_16,_16,_16];
_11 = [(-1506522588_i32),773296087_i32,(-165548040_i32),(-2075177423_i32),850746232_i32,1286129202_i32,1501001746_i32];
_15.1 = !_14;
_9 = [_16,_16,_16,_16,_16,_16];
_11 = _1;
place!(Field::<u16>(Variant(_7, 3), 0)) = !9758_u16;
_5 = core::ptr::addr_of!((*_5));
_14 = _15.1;
_6 = [1198031529_i32,(-1764799862_i32),(-181030158_i32),1944033624_i32,480403666_i32,(-1220663076_i32),1147091007_i32];
RET = [(-136969670_i32),179060702_i32,22356639_i32,(-406009220_i32),(-1690321109_i32),478697999_i32,(-351883581_i32)];
_1 = [518243798_i32,1946994484_i32,331959749_i32,887731421_i32,(-900888265_i32),959970577_i32,641502018_i32];
Call(_5 = fn15(_8, _11, _11, (*_5), (*_12), _10, _10, (*_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb9 = {
RET = _4;
_15.1 = -_14;
_13 = [_16,_16,_16,_16,_16,_16,_16,_16];
(*_18) = ['\u{fd72e}','\u{f02e}','\u{f5e99}','\u{42390}','\u{4ecdc}','\u{7d5a}','\u{ca88}'];
_8 = [1708953055_i32,(-1463284975_i32),1360326860_i32,(-1039877116_i32),(-88844428_i32),(-1015657913_i32),820040453_i32];
_18 = core::ptr::addr_of_mut!((*_18));
_4 = _1;
Goto(bb4)
}
bb10 = {
_20 = [_16,_16,_16,_16,_16,_16];
_10 = [241269607138572566165265725707695574929_u128,71840059415038349095380112050632797746_u128,172131120554069357265631770014255747646_u128,184758647192193445800133029513618877897_u128];
_21 = 113580181_u32;
_22 = _16 as i32;
_24 = core::ptr::addr_of_mut!((*_18));
_11 = RET;
_2 = [130018682724199681796228107868336797903_u128,208872261437727280044195662540493252150_u128,6313024158982973584322406169191116853_u128,217612863512177411131628950355338259214_u128];
_18 = core::ptr::addr_of_mut!((*_18));
_9 = [_16,_16,_16,_16,_16,_16];
_8 = _4;
_18 = _24;
_7 = Adt49::Variant3 { fld0: 16423_u16 };
_9 = _20;
_15.1 = 46791_u16 as i16;
_16 = !true;
_15.1 = -_14;
(*_12) = 9223372036854775807_isize | (-9223372036854775808_isize);
match _21 {
0 => bb5,
113580181 => bb7,
_ => bb6
}
}
bb11 = {
RET = _4;
_15.1 = -_14;
_13 = [_16,_16,_16,_16,_16,_16,_16,_16];
(*_18) = ['\u{fd72e}','\u{f02e}','\u{f5e99}','\u{42390}','\u{4ecdc}','\u{7d5a}','\u{ca88}'];
_8 = [1708953055_i32,(-1463284975_i32),1360326860_i32,(-1039877116_i32),(-88844428_i32),(-1015657913_i32),820040453_i32];
_18 = core::ptr::addr_of_mut!((*_18));
_4 = _1;
Goto(bb4)
}
bb12 = {
_10 = [33053248108608591769703911460550230936_u128,171110047330058840994258919041561312016_u128,273363172461581205351399736809260483331_u128,325588151821477072216490995895496614389_u128];
_15.1 = _14;
_15.0 = [2_usize,3_usize];
_19.1 = ['\u{7693d}','\u{eef}','\u{11b82}','\u{8d8a5}','\u{c49a5}','\u{e1954}','\u{eb923}'];
_19.2 = [720973705_u32,163078951_u32,230614602_u32];
_19.1 = ['\u{d510a}','\u{102999}','\u{79a78}','\u{9b151}','\u{864f9}','\u{719db}','\u{21ac0}'];
_19.1 = ['\u{fbcfd}','\u{e0481}','\u{8ed20}','\u{63aa6}','\u{8ec1c}','\u{16480}','\u{6075c}'];
_18 = core::ptr::addr_of_mut!(_19.1);
(*_18) = ['\u{867ee}','\u{1f4db}','\u{bfc38}','\u{1ef2f}','\u{b0343}','\u{d7ad4}','\u{6fa21}'];
_18 = core::ptr::addr_of_mut!(_19.1);
_18 = core::ptr::addr_of_mut!((*_18));
_19.2 = [3801943794_u32,1235969077_u32,2100899312_u32];
_12 = core::ptr::addr_of_mut!((*_12));
_4 = RET;
_7 = Adt49::Variant3 { fld0: 30058_u16 };
(*_18) = ['\u{17e5}','\u{33b8e}','\u{67115}','\u{f109e}','\u{2bf0}','\u{454}','\u{ca14b}'];
_1 = [880167189_i32,1456339676_i32,1569425331_i32,(-17081668_i32),(-68245384_i32),(-985158925_i32),126150502_i32];
_14 = -_15.1;
_3 = _13;
_3 = [_16,_16,_16,_16,_16,_16,_16,_16];
Goto(bb3)
}
bb13 = {
_5 = core::ptr::addr_of!((*_5));
_10 = _2;
_15.1 = '\u{f7042}' as i16;
_13 = [_16,_16,_16,_16,_16,_16,_16,_16];
_11 = [(-1506522588_i32),773296087_i32,(-165548040_i32),(-2075177423_i32),850746232_i32,1286129202_i32,1501001746_i32];
_15.1 = !_14;
_9 = [_16,_16,_16,_16,_16,_16];
_11 = _1;
place!(Field::<u16>(Variant(_7, 3), 0)) = !9758_u16;
_5 = core::ptr::addr_of!((*_5));
_14 = _15.1;
_6 = [1198031529_i32,(-1764799862_i32),(-181030158_i32),1944033624_i32,480403666_i32,(-1220663076_i32),1147091007_i32];
RET = [(-136969670_i32),179060702_i32,22356639_i32,(-406009220_i32),(-1690321109_i32),478697999_i32,(-351883581_i32)];
_1 = [518243798_i32,1946994484_i32,331959749_i32,887731421_i32,(-900888265_i32),959970577_i32,641502018_i32];
Call(_5 = fn15(_8, _11, _11, (*_5), (*_12), _10, _10, (*_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_25.0 = _29 - _22;
_30.1 = _22 + _29;
_25 = (_22, 267512695421124403130222704403445459220_u128);
_28 = 106_u8 & 128_u8;
_20 = [_16,_16,_16,_16,_16,_16];
_25.0 = _22 * _22;
_11 = [_30.1,_30.1,_30.1,_30.1,_29,_30.1,_25.0];
_27.fld2.1 = ['\u{787d4}','\u{597cc}','\u{9be31}','\u{88fff}','\u{58247}','\u{3dc52}','\u{8ea30}'];
_31 = 3_usize as u32;
_3 = _13;
_30.1 = -_25.0;
_6 = [_25.0,_25.0,_25.0,_25.0,_30.1,_30.1,_30.1];
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(14_usize, 16_usize, Move(_16), 4_usize, Move(_4), 28_usize, Move(_28), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(14_usize, 10_usize, Move(_10), 3_usize, Move(_3), 14_usize, Move(_14), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(14_usize, 20_usize, Move(_20), 21_usize, Move(_21), 39_usize, _39, 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: [i32; 7],mut _2: [i32; 7],mut _3: [i32; 7],mut _4: i128,mut _5: isize,mut _6: [u128; 4],mut _7: [u128; 4],mut _8: i128) -> *const i128 {
mir! {
type RET = *const i128;
let _9: f64;
let _10: *mut isize;
let _11: i64;
let _12: [u128; 4];
let _13: usize;
let _14: isize;
let _15: Adt58;
let _16: u32;
let _17: usize;
let _18: char;
let _19: u16;
let _20: i128;
let _21: [u32; 3];
let _22: (i32, u128);
let _23: u128;
let _24: Adt54;
let _25: ();
let _26: ();
{
RET = core::ptr::addr_of!(_4);
RET = core::ptr::addr_of!(_8);
_8 = 21610_i16 as i128;
(*RET) = _4;
Call(_2 = core::intrinsics::transmute(_3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = (-28286_i16) as f64;
_8 = !_4;
RET = core::ptr::addr_of!((*RET));
_10 = core::ptr::addr_of_mut!(_5);
_6 = [90539871645174691956082642777570031148_u128,278145894006076245731384002683785410994_u128,236779552464988966283115411123158429220_u128,265787645553113900839983493598875239991_u128];
(*_10) = 9223372036854775807_isize;
(*RET) = _4;
_5 = 1889490147_i32 as isize;
_11 = (-6929491733902505089_i64);
RET = core::ptr::addr_of!(_8);
_8 = _4 - _4;
_1 = [163070033_i32,(-1879415016_i32),152453876_i32,(-1739140369_i32),(-509347676_i32),(-135605387_i32),(-445740159_i32)];
Call(_3 = fn16(_6, RET, _9, (*_10), _11, _1, _1, _5, (*_10), _6, _10, _10, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = 173_u8 as i128;
_4 = (*RET) >> (*RET);
(*RET) = _4 >> _4;
RET = core::ptr::addr_of!((*RET));
(*RET) = _4;
_6 = [115059559785919790549037046053709391543_u128,124207254003165839955069479989153665260_u128,176748285353516567122133322790189060039_u128,117560231135947483658476079803758303642_u128];
(*RET) = _4 & _4;
(*RET) = !_4;
_8 = _4;
(*RET) = _4;
_9 = 167300005781957463908902557846093677749_u128 as f64;
_4 = -_8;
(*RET) = _4 >> _5;
_11 = '\u{2007f}' as i64;
_9 = _11 as f64;
_10 = core::ptr::addr_of_mut!(_5);
(*_10) = -(-9223372036854775808_isize);
RET = core::ptr::addr_of!(_4);
(*_10) = 1974346934_u32 as isize;
_11 = 2488634847192584703_i64 << _8;
Call(_7 = fn18(_5, _11, (*_10), _6, _6, _11, (*RET), _11, _1, (*_10), _8, _10), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_13 = !7_usize;
RET = core::ptr::addr_of!((*RET));
_6 = [94449322733130510497840737404525161189_u128,214344255710984952121437336068149869766_u128,113177521099249984643572049626195582405_u128,249132091436497777252146715816075657649_u128];
RET = core::ptr::addr_of!((*RET));
_10 = core::ptr::addr_of_mut!((*_10));
_8 = _4 >> _13;
(*_10) = (-9223372036854775808_isize) + 9223372036854775807_isize;
(*RET) = -_8;
(*RET) = _8 >> _8;
_12 = _7;
_14 = (*_10);
_7 = [100257711718690341319644685029172511795_u128,306267839220420789373802321776308330324_u128,103244996009643164980682276137424848848_u128,265360164662246692560058733156196384842_u128];
(*RET) = _8;
(*RET) = _8 | _8;
_6 = _12;
(*_10) = -_14;
(*RET) = _8;
_2 = [1843307764_i32,727188038_i32,272697518_i32,579689506_i32,(-1077523013_i32),(-2082406810_i32),(-1596823719_i32)];
Call(_12 = fn19(_6, _10, _2, _13), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
(*_10) = -_14;
_3 = [1985421784_i32,(-628580589_i32),(-1704415507_i32),1923082332_i32,(-330650264_i32),(-574842969_i32),(-552410394_i32)];
(*RET) = !_8;
(*RET) = _8 + _8;
_14 = (*_10) ^ _5;
_16 = 296562934045318120638265497825670103385_u128 as u32;
_3 = [99128008_i32,(-1533628583_i32),(-1316356695_i32),410276143_i32,402482975_i32,(-1015930333_i32),969896127_i32];
RET = core::ptr::addr_of!(_8);
RET = core::ptr::addr_of!(_4);
_5 = _14;
(*RET) = -_8;
(*RET) = _8;
RET = core::ptr::addr_of!((*RET));
(*RET) = -_8;
_16 = 1325_i16 as u32;
(*_10) = 46406_u16 as isize;
_3 = _2;
_13 = 10402821477572034901_usize;
_10 = core::ptr::addr_of_mut!((*_10));
(*RET) = (*_10) as i128;
(*_10) = (-66_i8) as isize;
_1 = [(-2269580_i32),(-374288142_i32),(-1414373581_i32),(-914413859_i32),(-1161770986_i32),1684562080_i32,197701202_i32];
(*RET) = _8;
_8 = -(*RET);
_9 = 9_u8 as f64;
Goto(bb5)
}
bb5 = {
RET = core::ptr::addr_of!(_4);
(*RET) = 780_u16 as i128;
_8 = (*RET) - (*RET);
match _13 {
0 => bb6,
1 => bb7,
10402821477572034901 => bb9,
_ => bb8
}
}
bb6 = {
(*_10) = -_14;
_3 = [1985421784_i32,(-628580589_i32),(-1704415507_i32),1923082332_i32,(-330650264_i32),(-574842969_i32),(-552410394_i32)];
(*RET) = !_8;
(*RET) = _8 + _8;
_14 = (*_10) ^ _5;
_16 = 296562934045318120638265497825670103385_u128 as u32;
_3 = [99128008_i32,(-1533628583_i32),(-1316356695_i32),410276143_i32,402482975_i32,(-1015930333_i32),969896127_i32];
RET = core::ptr::addr_of!(_8);
RET = core::ptr::addr_of!(_4);
_5 = _14;
(*RET) = -_8;
(*RET) = _8;
RET = core::ptr::addr_of!((*RET));
(*RET) = -_8;
_16 = 1325_i16 as u32;
(*_10) = 46406_u16 as isize;
_3 = _2;
_13 = 10402821477572034901_usize;
_10 = core::ptr::addr_of_mut!((*_10));
(*RET) = (*_10) as i128;
(*_10) = (-66_i8) as isize;
_1 = [(-2269580_i32),(-374288142_i32),(-1414373581_i32),(-914413859_i32),(-1161770986_i32),1684562080_i32,197701202_i32];
(*RET) = _8;
_8 = -(*RET);
_9 = 9_u8 as f64;
Goto(bb5)
}
bb7 = {
_13 = !7_usize;
RET = core::ptr::addr_of!((*RET));
_6 = [94449322733130510497840737404525161189_u128,214344255710984952121437336068149869766_u128,113177521099249984643572049626195582405_u128,249132091436497777252146715816075657649_u128];
RET = core::ptr::addr_of!((*RET));
_10 = core::ptr::addr_of_mut!((*_10));
_8 = _4 >> _13;
(*_10) = (-9223372036854775808_isize) + 9223372036854775807_isize;
(*RET) = -_8;
(*RET) = _8 >> _8;
_12 = _7;
_14 = (*_10);
_7 = [100257711718690341319644685029172511795_u128,306267839220420789373802321776308330324_u128,103244996009643164980682276137424848848_u128,265360164662246692560058733156196384842_u128];
(*RET) = _8;
(*RET) = _8 | _8;
_6 = _12;
(*_10) = -_14;
(*RET) = _8;
_2 = [1843307764_i32,727188038_i32,272697518_i32,579689506_i32,(-1077523013_i32),(-2082406810_i32),(-1596823719_i32)];
Call(_12 = fn19(_6, _10, _2, _13), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_9 = (-28286_i16) as f64;
_8 = !_4;
RET = core::ptr::addr_of!((*RET));
_10 = core::ptr::addr_of_mut!(_5);
_6 = [90539871645174691956082642777570031148_u128,278145894006076245731384002683785410994_u128,236779552464988966283115411123158429220_u128,265787645553113900839983493598875239991_u128];
(*_10) = 9223372036854775807_isize;
(*RET) = _4;
_5 = 1889490147_i32 as isize;
_11 = (-6929491733902505089_i64);
RET = core::ptr::addr_of!(_8);
_8 = _4 - _4;
_1 = [163070033_i32,(-1879415016_i32),152453876_i32,(-1739140369_i32),(-509347676_i32),(-135605387_i32),(-445740159_i32)];
Call(_3 = fn16(_6, RET, _9, (*_10), _11, _1, _1, _5, (*_10), _6, _10, _10, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb9 = {
_14 = !_5;
_4 = _8;
_12 = [3124443380385995944119935199825420622_u128,223293651627927963959565242409746786843_u128,142971782942927116107895600962318026223_u128,328463838898237163407217509733975081476_u128];
(*_10) = _14 >> (*RET);
_5 = _16 as isize;
_1 = [(-1009351058_i32),1975289568_i32,(-1076957936_i32),971444041_i32,1937937354_i32,(-985641898_i32),(-589228632_i32)];
_16 = 22492076_u32 >> (*RET);
_17 = !_13;
_17 = _13 * _13;
(*RET) = 107_u8 as i128;
_10 = core::ptr::addr_of_mut!((*_10));
RET = core::ptr::addr_of!(_8);
_8 = _4;
_10 = core::ptr::addr_of_mut!(_14);
RET = core::ptr::addr_of!((*RET));
(*_10) = _5;
_6 = [113280462089149520612998658254569471822_u128,93834651876366338969170124284375955463_u128,152999210082154537846952451140314211267_u128,210430308701988907605821149452887340554_u128];
match _13 {
10402821477572034901 => bb10,
_ => bb7
}
}
bb10 = {
_4 = (*RET);
(*_10) = 260632428_i32 as isize;
_6 = [55766403600862689724958384607708349375_u128,104064559617334884647966336722200913836_u128,57691290670460141214059512551691238567_u128,9183776502478075677605140962497856526_u128];
_20 = 1730947387_i32 as i128;
_21 = [_16,_16,_16];
_22.1 = 291519852119126818317301784796468919432_u128;
_22 = ((-1171593208_i32), 176158337585255731043781515484183889312_u128);
_19 = 40315_u16;
_11 = !8731265389436656203_i64;
_17 = !_13;
_13 = _8 as usize;
_5 = (*_10);
_12 = [_22.1,_22.1,_22.1,_22.1];
(*RET) = _20 * _20;
_9 = _19 as f64;
_17 = _13 << (*_10);
RET = core::ptr::addr_of!((*RET));
_1 = [_22.0,_22.0,_22.0,_22.0,_22.0,_22.0,_22.0];
_22 = ((-725293582_i32), 77891087087439326740149646364046708711_u128);
_17 = 55_u8 as usize;
_8 = _4;
(*RET) = _20;
Goto(bb11)
}
bb11 = {
(*RET) = _20 >> _17;
_19 = 16003_u16;
_19 = 49978_u16;
_1 = [_22.0,_22.0,_22.0,_22.0,_22.0,_22.0,_22.0];
_4 = _19 as i128;
Goto(bb12)
}
bb12 = {
(*RET) = _20 * _20;
_22.0 = 610475925_i32;
_13 = _17 ^ _17;
_6 = [_22.1,_22.1,_22.1,_22.1];
_21 = [_16,_16,_16];
(*_10) = !_5;
_24.fld0.1 = ['\u{203a1}','\u{103b7f}','\u{7d39f}','\u{b0199}','\u{ce2de}','\u{9c03a}','\u{a61f1}'];
Goto(bb13)
}
bb13 = {
_24.fld0.2 = [_16,_16,_16];
(*RET) = _20;
(*_10) = -_5;
_24.fld4 = _19 as f32;
_19 = _24.fld4 as u16;
match _22.0 {
0 => bb8,
1 => bb7,
2 => bb12,
3 => bb14,
4 => bb15,
5 => bb16,
610475925 => bb18,
_ => bb17
}
}
bb14 = {
_13 = !7_usize;
RET = core::ptr::addr_of!((*RET));
_6 = [94449322733130510497840737404525161189_u128,214344255710984952121437336068149869766_u128,113177521099249984643572049626195582405_u128,249132091436497777252146715816075657649_u128];
RET = core::ptr::addr_of!((*RET));
_10 = core::ptr::addr_of_mut!((*_10));
_8 = _4 >> _13;
(*_10) = (-9223372036854775808_isize) + 9223372036854775807_isize;
(*RET) = -_8;
(*RET) = _8 >> _8;
_12 = _7;
_14 = (*_10);
_7 = [100257711718690341319644685029172511795_u128,306267839220420789373802321776308330324_u128,103244996009643164980682276137424848848_u128,265360164662246692560058733156196384842_u128];
(*RET) = _8;
(*RET) = _8 | _8;
_6 = _12;
(*_10) = -_14;
(*RET) = _8;
_2 = [1843307764_i32,727188038_i32,272697518_i32,579689506_i32,(-1077523013_i32),(-2082406810_i32),(-1596823719_i32)];
Call(_12 = fn19(_6, _10, _2, _13), ReturnTo(bb4), UnwindUnreachable())
}
bb15 = {
(*RET) = _20 >> _17;
_19 = 16003_u16;
_19 = 49978_u16;
_1 = [_22.0,_22.0,_22.0,_22.0,_22.0,_22.0,_22.0];
_4 = _19 as i128;
Goto(bb12)
}
bb16 = {
_4 = (*RET);
(*_10) = 260632428_i32 as isize;
_6 = [55766403600862689724958384607708349375_u128,104064559617334884647966336722200913836_u128,57691290670460141214059512551691238567_u128,9183776502478075677605140962497856526_u128];
_20 = 1730947387_i32 as i128;
_21 = [_16,_16,_16];
_22.1 = 291519852119126818317301784796468919432_u128;
_22 = ((-1171593208_i32), 176158337585255731043781515484183889312_u128);
_19 = 40315_u16;
_11 = !8731265389436656203_i64;
_17 = !_13;
_13 = _8 as usize;
_5 = (*_10);
_12 = [_22.1,_22.1,_22.1,_22.1];
(*RET) = _20 * _20;
_9 = _19 as f64;
_17 = _13 << (*_10);
RET = core::ptr::addr_of!((*RET));
_1 = [_22.0,_22.0,_22.0,_22.0,_22.0,_22.0,_22.0];
_22 = ((-725293582_i32), 77891087087439326740149646364046708711_u128);
_17 = 55_u8 as usize;
_8 = _4;
(*RET) = _20;
Goto(bb11)
}
bb17 = {
_14 = !_5;
_4 = _8;
_12 = [3124443380385995944119935199825420622_u128,223293651627927963959565242409746786843_u128,142971782942927116107895600962318026223_u128,328463838898237163407217509733975081476_u128];
(*_10) = _14 >> (*RET);
_5 = _16 as isize;
_1 = [(-1009351058_i32),1975289568_i32,(-1076957936_i32),971444041_i32,1937937354_i32,(-985641898_i32),(-589228632_i32)];
_16 = 22492076_u32 >> (*RET);
_17 = !_13;
_17 = _13 * _13;
(*RET) = 107_u8 as i128;
_10 = core::ptr::addr_of_mut!((*_10));
RET = core::ptr::addr_of!(_8);
_8 = _4;
_10 = core::ptr::addr_of_mut!(_14);
RET = core::ptr::addr_of!((*RET));
(*_10) = _5;
_6 = [113280462089149520612998658254569471822_u128,93834651876366338969170124284375955463_u128,152999210082154537846952451140314211267_u128,210430308701988907605821149452887340554_u128];
match _13 {
10402821477572034901 => bb10,
_ => bb7
}
}
bb18 = {
_22 = ((-386849529_i32), 303687183235337330981557958322117391828_u128);
_18 = '\u{73db4}';
_14 = _5;
Goto(bb19)
}
bb19 = {
Call(_25 = dump_var(15_usize, 13_usize, Move(_13), 4_usize, Move(_4), 1_usize, Move(_1), 3_usize, Move(_3)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_25 = dump_var(15_usize, 16_usize, Move(_16), 20_usize, Move(_20), 8_usize, Move(_8), 18_usize, Move(_18)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_25 = dump_var(15_usize, 11_usize, Move(_11), 26_usize, _26, 26_usize, _26, 26_usize, _26), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: [u128; 4],mut _2: *const i128,mut _3: f64,mut _4: isize,mut _5: i64,mut _6: [i32; 7],mut _7: [i32; 7],mut _8: isize,mut _9: isize,mut _10: [u128; 4],mut _11: *mut isize,mut _12: *mut isize,mut _13: *mut isize) -> [i32; 7] {
mir! {
type RET = [i32; 7];
let _14: f64;
let _15: (*const f32, [char; 7], [u32; 3]);
let _16: [u32; 1];
let _17: [char; 6];
let _18: Adt51;
let _19: u16;
let _20: (f64, u64, i32, f64);
let _21: Adt48;
let _22: (i32, u128);
let _23: *const i128;
let _24: bool;
let _25: (*const f32, [char; 7], [u32; 3]);
let _26: isize;
let _27: bool;
let _28: Adt58;
let _29: bool;
let _30: Adt46;
let _31: [usize; 2];
let _32: [char; 7];
let _33: f64;
let _34: Adt51;
let _35: u32;
let _36: [bool; 6];
let _37: isize;
let _38: [u32; 1];
let _39: ([usize; 2], i16);
let _40: u32;
let _41: [u64; 3];
let _42: bool;
let _43: ();
let _44: ();
{
(*_13) = '\u{3fbd3}' as isize;
_5 = 1213961116_i32 as i64;
(*_12) = (-1789605333_i32) as isize;
_10 = _1;
(*_13) = _9;
RET = [1468883324_i32,322268507_i32,(-1268279215_i32),902670066_i32,856864557_i32,(-1726941265_i32),1384526030_i32];
(*_13) = _4;
_13 = core::ptr::addr_of_mut!(_4);
_5 = 93_u8 as i64;
Call(_6 = fn17(_13, _5, RET, RET, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12 = _11;
_15.1 = ['\u{10329}','\u{10bded}','\u{103fce}','\u{47478}','\u{76f}','\u{ee13}','\u{2d7ef}'];
(*_12) = !(*_13);
(*_13) = (*_11) ^ (*_11);
(*_12) = _4;
(*_11) = (-6854_i16) as isize;
_5 = 3744958525_u32 as i64;
_6 = _7;
Goto(bb2)
}
bb2 = {
_14 = _3;
_12 = core::ptr::addr_of_mut!((*_12));
_6 = [655757735_i32,1931081433_i32,(-1818225608_i32),(-803401611_i32),1190096758_i32,(-1131214959_i32),(-1202385924_i32)];
(*_2) = 147041376096013371664381742334098493271_i128 >> (*_13);
_16 = [2776293102_u32];
(*_13) = _5 as isize;
_15.1 = ['\u{6eeb6}','\u{3f4f9}','\u{de99b}','\u{54dd7}','\u{b7a5d}','\u{6c6c6}','\u{ed963}'];
_10 = [225067273425708027710505511429137832939_u128,152175366671966391568464779919526434821_u128,80317259089795117602095531985567089823_u128,24031215874582311742035609782480501205_u128];
_3 = _14 + _14;
Goto(bb3)
}
bb3 = {
_4 = (*_11) | (*_12);
_13 = core::ptr::addr_of_mut!((*_11));
_22 = (1817044789_i32, 198570508405188397128818085425128073490_u128);
_14 = -_3;
_20.3 = (*_2) as f64;
_21.fld2.2 = [3261832529_u32,3252451448_u32,4071382057_u32];
_3 = (-19867_i16) as f64;
_3 = _20.3 + _14;
_15.1 = ['\u{87c8e}','\u{f40a9}','\u{3c2a0}','\u{84491}','\u{54822}','\u{9033c}','\u{c94e8}'];
_21.fld1 = _21.fld2.2;
_7 = [_22.0,_22.0,_22.0,_22.0,_22.0,_22.0,_22.0];
_20 = (_3, 14688009186237948598_u64, _22.0, _3);
match _20.1 {
14688009186237948598 => bb5,
_ => bb4
}
}
bb4 = {
_14 = _3;
_12 = core::ptr::addr_of_mut!((*_12));
_6 = [655757735_i32,1931081433_i32,(-1818225608_i32),(-803401611_i32),1190096758_i32,(-1131214959_i32),(-1202385924_i32)];
(*_2) = 147041376096013371664381742334098493271_i128 >> (*_13);
_16 = [2776293102_u32];
(*_13) = _5 as isize;
_15.1 = ['\u{6eeb6}','\u{3f4f9}','\u{de99b}','\u{54dd7}','\u{b7a5d}','\u{6c6c6}','\u{ed963}'];
_10 = [225067273425708027710505511429137832939_u128,152175366671966391568464779919526434821_u128,80317259089795117602095531985567089823_u128,24031215874582311742035609782480501205_u128];
_3 = _14 + _14;
Goto(bb3)
}
bb5 = {
_17 = ['\u{27c7f}','\u{cc3bd}','\u{4f0fc}','\u{f767c}','\u{b6ee4}','\u{574e5}'];
_22.0 = _20.2;
_12 = core::ptr::addr_of_mut!((*_13));
_4 = (*_12) + _8;
_21.fld3 = [_20.2,_20.2,_20.2,_22.0,_22.0,_22.0,_20.2];
_27 = _20.1 != _20.1;
_21.fld1 = [327536278_u32,1943211958_u32,457813263_u32];
_12 = core::ptr::addr_of_mut!((*_11));
(*_13) = _22.1 as isize;
(*_2) = 37889713193145912251069877374388240112_i128 ^ 168328785732807693869548022381668749146_i128;
_2 = core::ptr::addr_of!((*_2));
_12 = _13;
Goto(bb6)
}
bb6 = {
_4 = 57311_u16 as isize;
RET = _7;
_11 = _13;
_13 = _12;
_22.0 = (-9_i8) as i32;
_14 = (*_2) as f64;
_2 = core::ptr::addr_of!((*_2));
_24 = _27;
(*_13) = !_9;
_29 = !_24;
_1 = [_22.1,_22.1,_22.1,_22.1];
_15.2 = [2744392363_u32,3737373714_u32,1462289981_u32];
(*_12) = !_9;
_25.1 = ['\u{2b53e}','\u{4b384}','\u{6506b}','\u{c1107}','\u{806a8}','\u{1ce5e}','\u{5e9a5}'];
_1 = [_22.1,_22.1,_22.1,_22.1];
_16 = [1740233200_u32];
_12 = core::ptr::addr_of_mut!(_9);
Goto(bb7)
}
bb7 = {
(*_11) = (*_12);
_20.2 = 13_u8 as i32;
match _22.1 {
0 => bb6,
1 => bb8,
198570508405188397128818085425128073490 => bb10,
_ => bb9
}
}
bb8 = {
_12 = _11;
_15.1 = ['\u{10329}','\u{10bded}','\u{103fce}','\u{47478}','\u{76f}','\u{ee13}','\u{2d7ef}'];
(*_12) = !(*_13);
(*_13) = (*_11) ^ (*_11);
(*_12) = _4;
(*_11) = (-6854_i16) as isize;
_5 = 3744958525_u32 as i64;
_6 = _7;
Goto(bb2)
}
bb9 = {
_14 = _3;
_12 = core::ptr::addr_of_mut!((*_12));
_6 = [655757735_i32,1931081433_i32,(-1818225608_i32),(-803401611_i32),1190096758_i32,(-1131214959_i32),(-1202385924_i32)];
(*_2) = 147041376096013371664381742334098493271_i128 >> (*_13);
_16 = [2776293102_u32];
(*_13) = _5 as isize;
_15.1 = ['\u{6eeb6}','\u{3f4f9}','\u{de99b}','\u{54dd7}','\u{b7a5d}','\u{6c6c6}','\u{ed963}'];
_10 = [225067273425708027710505511429137832939_u128,152175366671966391568464779919526434821_u128,80317259089795117602095531985567089823_u128,24031215874582311742035609782480501205_u128];
_3 = _14 + _14;
Goto(bb3)
}
bb10 = {
_13 = _12;
_23 = _2;
_22.1 = 215166243539827252364831668395607500975_u128 + 102057534292394057870587650986245304858_u128;
RET = _6;
_4 = (*_11) & (*_11);
_15.2 = [2771933267_u32,4050673930_u32,409291138_u32];
_21.fld0 = [_29,_27,_27,_27,_24,_29,_27,_27];
_22.1 = !215948079305982016239068319265827522947_u128;
_30.fld3 = [22113_i16,(-11875_i16)];
_20.1 = 15206717016170909066_u64;
(*_11) = -_4;
_30.fld6 = !_20.1;
_14 = _20.0;
_27 = _24 | _24;
_30.fld2 = -_9;
_4 = (*_11) >> _20.1;
RET = [_20.2,_22.0,_20.2,_22.0,_20.2,_22.0,_20.2];
_30.fld4 = core::ptr::addr_of!(_20.3);
_30.fld3 = [29107_i16,5327_i16];
Goto(bb11)
}
bb11 = {
_20.0 = 29_i8 as f64;
(*_12) = (*_11);
_7 = [_20.2,_20.2,_22.0,_20.2,_20.2,_22.0,_20.2];
_7 = [_20.2,_22.0,_20.2,_22.0,_20.2,_20.2,_20.2];
_10 = [_22.1,_22.1,_22.1,_22.1];
_20 = (_14, _30.fld6, _22.0, _14);
_20.1 = (*_23) as u64;
_13 = _11;
_30.fld6 = !_20.1;
_30.fld3 = [10204_i16,25723_i16];
_22.1 = 12206_i16 as u128;
_13 = core::ptr::addr_of_mut!(_8);
(*_2) = 25368_i16 as i128;
_20.0 = 6_usize as f64;
_2 = _23;
_22.1 = 19874745746747380856996485545042820781_u128;
_30.fld0 = _4 != (*_12);
(*_12) = !_4;
RET = [_20.2,_22.0,_20.2,_22.0,_22.0,_20.2,_20.2];
Goto(bb12)
}
bb12 = {
_32 = ['\u{2b59d}','\u{b8cfb}','\u{316ac}','\u{dd010}','\u{56344}','\u{90058}','\u{92b85}'];
(*_23) = 25512147304654846324112944196360557613_i128;
_25.2 = [1913605806_u32,1568515092_u32,3842753200_u32];
_20 = (_3, _30.fld6, _22.0, _14);
_17 = ['\u{3410f}','\u{47778}','\u{aaf3f}','\u{30d3}','\u{4fd23}','\u{5cb2a}'];
match _22.1 {
0 => bb1,
1 => bb2,
2 => bb10,
3 => bb9,
4 => bb5,
5 => bb6,
6 => bb11,
19874745746747380856996485545042820781 => bb14,
_ => bb13
}
}
bb13 = {
_20.0 = 29_i8 as f64;
(*_12) = (*_11);
_7 = [_20.2,_20.2,_22.0,_20.2,_20.2,_22.0,_20.2];
_7 = [_20.2,_22.0,_20.2,_22.0,_20.2,_20.2,_20.2];
_10 = [_22.1,_22.1,_22.1,_22.1];
_20 = (_14, _30.fld6, _22.0, _14);
_20.1 = (*_23) as u64;
_13 = _11;
_30.fld6 = !_20.1;
_30.fld3 = [10204_i16,25723_i16];
_22.1 = 12206_i16 as u128;
_13 = core::ptr::addr_of_mut!(_8);
(*_2) = 25368_i16 as i128;
_20.0 = 6_usize as f64;
_2 = _23;
_22.1 = 19874745746747380856996485545042820781_u128;
_30.fld0 = _4 != (*_12);
(*_12) = !_4;
RET = [_20.2,_22.0,_20.2,_22.0,_22.0,_20.2,_20.2];
Goto(bb12)
}
bb14 = {
_20 = (_14, _30.fld6, _22.0, _14);
_37 = 3434324924_u32 as isize;
_2 = _23;
_31 = [7382231724697519591_usize,2_usize];
_30.fld4 = core::ptr::addr_of!(_14);
(*_12) = 6108070158743640911_usize as isize;
_30.fld3 = [1733_i16,(-32236_i16)];
(*_11) = '\u{ab76c}' as isize;
_39.1 = (-25713_i16);
_26 = _20.1 as isize;
RET = [_20.2,_22.0,_20.2,_20.2,_22.0,_22.0,_22.0];
_39 = (_31, (-258_i16));
_15.1 = ['\u{e9f3}','\u{919a5}','\u{8b154}','\u{108f4d}','\u{2e38b}','\u{75746}','\u{9e69d}'];
(*_2) = _5 as i128;
_21.fld3 = [_20.2,_20.2,_22.0,_22.0,_20.2,_22.0,_20.2];
_25.1 = ['\u{615fa}','\u{b79a5}','\u{9380c}','\u{f0f8}','\u{c3bd2}','\u{4b7b5}','\u{ea834}'];
_22.1 = 1669272055_u32 as u128;
_41 = [_20.1,_20.1,_30.fld6];
_38 = [1056098806_u32];
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(16_usize, 26_usize, Move(_26), 16_usize, Move(_16), 41_usize, Move(_41), 39_usize, Move(_39)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(16_usize, 8_usize, Move(_8), 38_usize, Move(_38), 1_usize, Move(_1), 31_usize, Move(_31)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(16_usize, 7_usize, Move(_7), 27_usize, Move(_27), 44_usize, _44, 44_usize, _44), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: *mut isize,mut _2: i64,mut _3: [i32; 7],mut _4: [i32; 7],mut _5: [u128; 4]) -> [i32; 7] {
mir! {
type RET = [i32; 7];
let _6: ([isize; 2], usize, i64, u8);
let _7: i16;
let _8: i32;
let _9: isize;
let _10: i32;
let _11: ([isize; 2], usize, i64, u8);
let _12: [char; 7];
let _13: [i16; 2];
let _14: [u128; 4];
let _15: ([usize; 2], i16);
let _16: [i32; 7];
let _17: usize;
let _18: isize;
let _19: i64;
let _20: i128;
let _21: char;
let _22: i16;
let _23: isize;
let _24: Adt50;
let _25: [u32; 1];
let _26: isize;
let _27: [isize; 2];
let _28: isize;
let _29: f64;
let _30: Adt52;
let _31: ();
let _32: ();
{
_2 = 6772179578838171369_u64 as i64;
_3 = [1443060248_i32,(-1327885558_i32),(-324594585_i32),1095687795_i32,(-2095114634_i32),1424134461_i32,(-830663650_i32)];
_5 = [172036351981017023756859356123875114357_u128,72139963938663449935171255846669307153_u128,279039823659841726007860268920719504436_u128,152853387369015001720741809288288859344_u128];
Call((*_1) = core::intrinsics::transmute(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = [290816472186631753305204156808012569874_u128,25702093916822593198952366996710763727_u128,45715272614363095354428421862006735483_u128,59721840809054325472414867816392761272_u128];
_8 = 245578664_i32;
(*_1) = !(-9223372036854775808_isize);
_6.0 = [(*_1),(*_1)];
_6.0 = [(*_1),(*_1)];
_3 = [_8,_8,_8,_8,_8,_8,_8];
_1 = core::ptr::addr_of_mut!((*_1));
_6.3 = 47_u8 & 126_u8;
_6.3 = !109_u8;
_9 = (*_1);
(*_1) = !_9;
_7 = -20232_i16;
Goto(bb2)
}
bb2 = {
(*_1) = !_9;
_10 = -_8;
(*_1) = 262899287154891286199892065959564653705_u128 as isize;
_12 = ['\u{ac1c4}','\u{f8c2b}','\u{bbd7b}','\u{6e3ae}','\u{d83d2}','\u{72be6}','\u{106860}'];
_7 = (-8981_i16) * 3078_i16;
_11.2 = -_2;
RET = _4;
_11 = (_6.0, 15420021648696482133_usize, _2, _6.3);
RET = [_10,_8,_10,_8,_8,_10,_10];
RET = [_10,_8,_10,_8,_8,_10,_10];
_12 = ['\u{6ff43}','\u{5646b}','\u{b4c35}','\u{2ef16}','\u{a8e65}','\u{35dee}','\u{1ad7}'];
RET = [_8,_8,_10,_10,_8,_10,_10];
_6.1 = !_11.1;
_6.2 = -_11.2;
_11 = (_6.0, _6.1, _6.2, _6.3);
_2 = -_6.2;
_14 = [62973964684463877432500196879277578839_u128,171350318760264535972132273892079231265_u128,104028411654317304910237656410147538189_u128,25223745185935932454105135928620791319_u128];
_13 = [_7,_7];
_15.0 = [_11.1,_6.1];
_11.2 = _2 - _6.2;
_15.0 = [_6.1,_6.1];
_6.1 = _11.1;
_4 = [_8,_10,_10,_8,_8,_8,_10];
(*_1) = true as isize;
_11 = _6;
match _8 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
245578664 => bb8,
_ => bb7
}
}
bb3 = {
_5 = [290816472186631753305204156808012569874_u128,25702093916822593198952366996710763727_u128,45715272614363095354428421862006735483_u128,59721840809054325472414867816392761272_u128];
_8 = 245578664_i32;
(*_1) = !(-9223372036854775808_isize);
_6.0 = [(*_1),(*_1)];
_6.0 = [(*_1),(*_1)];
_3 = [_8,_8,_8,_8,_8,_8,_8];
_1 = core::ptr::addr_of_mut!((*_1));
_6.3 = 47_u8 & 126_u8;
_6.3 = !109_u8;
_9 = (*_1);
(*_1) = !_9;
_7 = -20232_i16;
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
_16 = [_10,_10,_10,_8,_10,_8,_10];
_6.0 = [(*_1),_9];
_3 = [_10,_8,_8,_10,_8,_10,_8];
_15.1 = -_7;
_12 = ['\u{108e55}','\u{b9f04}','\u{3c21}','\u{2b765}','\u{721ba}','\u{477dc}','\u{d6a36}'];
_11 = (_6.0, _6.1, _2, _6.3);
_6.3 = !_11.3;
_6 = _11;
_6.2 = !_11.2;
_2 = _6.2;
_11.0 = _6.0;
_11.3 = !_6.3;
_13 = [_15.1,_15.1];
_20 = 164498400006041022212802672198960997186_i128 ^ 69279798534217337065721960072865980174_i128;
match _8 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb9,
245578664 => bb11,
_ => bb10
}
}
bb9 = {
_5 = [290816472186631753305204156808012569874_u128,25702093916822593198952366996710763727_u128,45715272614363095354428421862006735483_u128,59721840809054325472414867816392761272_u128];
_8 = 245578664_i32;
(*_1) = !(-9223372036854775808_isize);
_6.0 = [(*_1),(*_1)];
_6.0 = [(*_1),(*_1)];
_3 = [_8,_8,_8,_8,_8,_8,_8];
_1 = core::ptr::addr_of_mut!((*_1));
_6.3 = 47_u8 & 126_u8;
_6.3 = !109_u8;
_9 = (*_1);
(*_1) = !_9;
_7 = -20232_i16;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
_18 = (*_1);
_23 = 949078077_u32 as isize;
_4 = [_8,_8,_10,_10,_10,_10,_10];
Goto(bb12)
}
bb12 = {
_6.0 = [_23,(*_1)];
_13 = [_15.1,_7];
_12 = ['\u{d59c}','\u{475d1}','\u{b2e45}','\u{c9fc3}','\u{41b0a}','\u{82cc3}','\u{ca322}'];
(*_1) = _9;
_11 = _6;
_15.0 = [_11.1,_11.1];
_9 = 91125124438863580900695529915874597768_u128 as isize;
_15.0 = [_11.1,_6.1];
_18 = _23;
_22 = _15.1;
_19 = _2 & _2;
_17 = _6.1;
_7 = -_15.1;
_11.3 = _6.3;
(*_1) = 160618535275143938744361070573130313923_u128 as isize;
RET = [_8,_10,_10,_8,_10,_10,_8];
_15.1 = _10 as i16;
_11.2 = _6.2 + _19;
match _8 {
0 => bb9,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
6 => bb18,
245578664 => bb20,
_ => bb19
}
}
bb13 = {
_5 = [290816472186631753305204156808012569874_u128,25702093916822593198952366996710763727_u128,45715272614363095354428421862006735483_u128,59721840809054325472414867816392761272_u128];
_8 = 245578664_i32;
(*_1) = !(-9223372036854775808_isize);
_6.0 = [(*_1),(*_1)];
_6.0 = [(*_1),(*_1)];
_3 = [_8,_8,_8,_8,_8,_8,_8];
_1 = core::ptr::addr_of_mut!((*_1));
_6.3 = 47_u8 & 126_u8;
_6.3 = !109_u8;
_9 = (*_1);
(*_1) = !_9;
_7 = -20232_i16;
Goto(bb2)
}
bb14 = {
Return()
}
bb15 = {
_5 = [290816472186631753305204156808012569874_u128,25702093916822593198952366996710763727_u128,45715272614363095354428421862006735483_u128,59721840809054325472414867816392761272_u128];
_8 = 245578664_i32;
(*_1) = !(-9223372036854775808_isize);
_6.0 = [(*_1),(*_1)];
_6.0 = [(*_1),(*_1)];
_3 = [_8,_8,_8,_8,_8,_8,_8];
_1 = core::ptr::addr_of_mut!((*_1));
_6.3 = 47_u8 & 126_u8;
_6.3 = !109_u8;
_9 = (*_1);
(*_1) = !_9;
_7 = -20232_i16;
Goto(bb2)
}
bb16 = {
_16 = [_10,_10,_10,_8,_10,_8,_10];
_6.0 = [(*_1),_9];
_3 = [_10,_8,_8,_10,_8,_10,_8];
_15.1 = -_7;
_12 = ['\u{108e55}','\u{b9f04}','\u{3c21}','\u{2b765}','\u{721ba}','\u{477dc}','\u{d6a36}'];
_11 = (_6.0, _6.1, _2, _6.3);
_6.3 = !_11.3;
_6 = _11;
_6.2 = !_11.2;
_2 = _6.2;
_11.0 = _6.0;
_11.3 = !_6.3;
_13 = [_15.1,_15.1];
_20 = 164498400006041022212802672198960997186_i128 ^ 69279798534217337065721960072865980174_i128;
match _8 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb9,
245578664 => bb11,
_ => bb10
}
}
bb17 = {
Return()
}
bb18 = {
Return()
}
bb19 = {
(*_1) = !_9;
_10 = -_8;
(*_1) = 262899287154891286199892065959564653705_u128 as isize;
_12 = ['\u{ac1c4}','\u{f8c2b}','\u{bbd7b}','\u{6e3ae}','\u{d83d2}','\u{72be6}','\u{106860}'];
_7 = (-8981_i16) * 3078_i16;
_11.2 = -_2;
RET = _4;
_11 = (_6.0, 15420021648696482133_usize, _2, _6.3);
RET = [_10,_8,_10,_8,_8,_10,_10];
RET = [_10,_8,_10,_8,_8,_10,_10];
_12 = ['\u{6ff43}','\u{5646b}','\u{b4c35}','\u{2ef16}','\u{a8e65}','\u{35dee}','\u{1ad7}'];
RET = [_8,_8,_10,_10,_8,_10,_10];
_6.1 = !_11.1;
_6.2 = -_11.2;
_11 = (_6.0, _6.1, _6.2, _6.3);
_2 = -_6.2;
_14 = [62973964684463877432500196879277578839_u128,171350318760264535972132273892079231265_u128,104028411654317304910237656410147538189_u128,25223745185935932454105135928620791319_u128];
_13 = [_7,_7];
_15.0 = [_11.1,_6.1];
_11.2 = _2 - _6.2;
_15.0 = [_6.1,_6.1];
_6.1 = _11.1;
_4 = [_8,_10,_10,_8,_8,_8,_10];
(*_1) = true as isize;
_11 = _6;
match _8 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
245578664 => bb8,
_ => bb7
}
}
bb20 = {
(*_1) = _18 >> _10;
RET = _3;
_11.2 = _19;
_27 = _6.0;
_26 = (*_1) - (*_1);
_9 = (*_1) & (*_1);
_6 = (_11.0, _11.1, _2, _11.3);
_11.3 = _6.3;
_1 = core::ptr::addr_of_mut!(_23);
_7 = _22 + _22;
RET = [_8,_10,_8,_8,_10,_10,_10];
_6 = (_27, _17, _19, _11.3);
_21 = '\u{1039c6}';
_16 = _4;
_12 = [_21,_21,_21,_21,_21,_21,_21];
_25 = [2770663413_u32];
_23 = !_9;
_7 = _15.1 & _22;
_27 = [(*_1),_26];
_15.1 = _7;
Goto(bb21)
}
bb21 = {
Call(_31 = dump_var(17_usize, 13_usize, Move(_13), 5_usize, Move(_5), 11_usize, Move(_11), 17_usize, Move(_17)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_31 = dump_var(17_usize, 12_usize, Move(_12), 7_usize, Move(_7), 27_usize, Move(_27), 10_usize, Move(_10)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_31 = dump_var(17_usize, 14_usize, Move(_14), 16_usize, Move(_16), 18_usize, Move(_18), 6_usize, Move(_6)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: isize,mut _2: i64,mut _3: isize,mut _4: [u128; 4],mut _5: [u128; 4],mut _6: i64,mut _7: i128,mut _8: i64,mut _9: [i32; 7],mut _10: isize,mut _11: i128,mut _12: *mut isize) -> [u128; 4] {
mir! {
type RET = [u128; 4];
let _13: isize;
let _14: [i32; 7];
let _15: isize;
let _16: u64;
let _17: [i16; 2];
let _18: f64;
let _19: [u32; 3];
let _20: char;
let _21: [u64; 3];
let _22: ([isize; 2], usize, i64, u8);
let _23: f32;
let _24: Adt52;
let _25: Adt60;
let _26: [i16; 2];
let _27: [usize; 2];
let _28: isize;
let _29: char;
let _30: char;
let _31: *const i16;
let _32: ([usize; 2], i16);
let _33: [bool; 6];
let _34: char;
let _35: Adt58;
let _36: isize;
let _37: [bool; 6];
let _38: [u32; 3];
let _39: [isize; 2];
let _40: [isize; 2];
let _41: isize;
let _42: [u128; 4];
let _43: [usize; 2];
let _44: i128;
let _45: usize;
let _46: Adt50;
let _47: [u64; 3];
let _48: ();
let _49: ();
{
RET = [180920850024258659160976945232337951555_u128,14476604731818920019232914892593836200_u128,210000102535241906129631289652094157352_u128,114979711473241138581389610964112060931_u128];
_6 = !_2;
_2 = _6;
_6 = _8;
_7 = _11 ^ _11;
_11 = (-30714_i16) as i128;
_3 = _10 + (*_12);
_3 = (*_12) ^ _10;
_1 = _3;
_7 = 232_u8 as i128;
_11 = 61_i8 as i128;
(*_12) = _1 & _1;
_8 = _6;
_4 = [207296862595371581962641943770856686175_u128,115978447282991182910037985654256306089_u128,63964726408498723156867010711422397475_u128,195116122119858049331031682512493087235_u128];
_14 = [1420285329_i32,(-1951093619_i32),(-241509389_i32),453264265_i32,(-921402505_i32),1949296451_i32,(-1878194248_i32)];
_15 = _1;
Goto(bb1)
}
bb1 = {
_5 = RET;
_13 = !_1;
_10 = -_1;
_18 = 7145370490462429557_u64 as f64;
_12 = core::ptr::addr_of_mut!(_10);
RET = [192384749297296099675978803330104075968_u128,242481330224545064993781327632685867773_u128,171977242775842474406312273507640375516_u128,183268544901618632671143877698614909928_u128];
(*_12) = _1;
(*_12) = _1;
_14 = [(-1036623463_i32),(-1171510807_i32),(-958760459_i32),(-1345116851_i32),(-241247811_i32),(-335699156_i32),(-683940353_i32)];
_16 = _18 as u64;
_19 = [316924716_u32,4011335325_u32,2710382884_u32];
(*_12) = -_15;
RET = [107519482257842348799072842800833917629_u128,304390860835133928546241016759848814622_u128,197117815618613489845681563601361009142_u128,66079731952751491068089834897452067866_u128];
_7 = (-102_i8) as i128;
(*_12) = _13;
_20 = '\u{7a8ef}';
_17 = [(-27941_i16),7599_i16];
_7 = _11;
_12 = core::ptr::addr_of_mut!(_3);
RET = _4;
_9 = _14;
_3 = !_15;
_6 = 152_u8 as i64;
_22.0 = [_10,(*_12)];
Goto(bb2)
}
bb2 = {
_18 = 103_i8 as f64;
_16 = !6769242417758459947_u64;
_10 = _1;
_18 = 487178634_u32 as f64;
_9 = [1317374266_i32,(-803601822_i32),(-1074662693_i32),508807815_i32,(-1649663395_i32),(-1386673264_i32),(-432387264_i32)];
_18 = 887567568_i32 as f64;
_9 = [1237702466_i32,2051935361_i32,(-1326044539_i32),(-624186721_i32),1152578120_i32,(-1761514033_i32),(-1909611072_i32)];
_24 = Adt52::Variant0 { fld0: _19 };
_23 = 62351_u16 as f32;
_21 = [_16,_16,_16];
_22.2 = 221602642055804846268942340007310430982_u128 as i64;
_18 = 2673759010_u32 as f64;
SetDiscriminant(_24, 0);
_20 = '\u{c838a}';
_19 = [662351878_u32,3434171350_u32,798850204_u32];
_15 = 11817462932830086690_usize as isize;
_16 = 14403874577682066984_u64;
_18 = _13 as f64;
_10 = !_3;
_6 = _2 >> _2;
_17 = [23921_i16,(-12297_i16)];
_25 = Adt60::Variant1 { fld0: 1833_i16,fld1: _21 };
_21 = [_16,_16,_16];
_1 = _10;
_17 = [31646_i16,3427_i16];
(*_12) = !_13;
match _16 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
14403874577682066984 => bb8,
_ => bb7
}
}
bb3 = {
_5 = RET;
_13 = !_1;
_10 = -_1;
_18 = 7145370490462429557_u64 as f64;
_12 = core::ptr::addr_of_mut!(_10);
RET = [192384749297296099675978803330104075968_u128,242481330224545064993781327632685867773_u128,171977242775842474406312273507640375516_u128,183268544901618632671143877698614909928_u128];
(*_12) = _1;
(*_12) = _1;
_14 = [(-1036623463_i32),(-1171510807_i32),(-958760459_i32),(-1345116851_i32),(-241247811_i32),(-335699156_i32),(-683940353_i32)];
_16 = _18 as u64;
_19 = [316924716_u32,4011335325_u32,2710382884_u32];
(*_12) = -_15;
RET = [107519482257842348799072842800833917629_u128,304390860835133928546241016759848814622_u128,197117815618613489845681563601361009142_u128,66079731952751491068089834897452067866_u128];
_7 = (-102_i8) as i128;
(*_12) = _13;
_20 = '\u{7a8ef}';
_17 = [(-27941_i16),7599_i16];
_7 = _11;
_12 = core::ptr::addr_of_mut!(_3);
RET = _4;
_9 = _14;
_3 = !_15;
_6 = 152_u8 as i64;
_22.0 = [_10,(*_12)];
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
_9 = [(-1668063807_i32),1008611213_i32,1624268427_i32,870700309_i32,360996411_i32,(-626763319_i32),961172436_i32];
_6 = _23 as i64;
_16 = 3310757992274126701_u64 ^ 14697990725522640186_u64;
place!(Field::<i16>(Variant(_25, 1), 0)) = 3_usize as i16;
_22.3 = 12_u8;
_14 = [(-1161866374_i32),(-1571211695_i32),294898599_i32,1303461972_i32,1810669172_i32,854545211_i32,711524898_i32];
_22.0 = [_3,_10];
_27 = [16444714076378511146_usize,3_usize];
_22.2 = 102_i8 as i64;
_22.0 = [(*_12),_10];
_30 = _20;
_31 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_25, 1), 0)));
(*_12) = (*_31) as isize;
_13 = 2105234937_i32 as isize;
_31 = core::ptr::addr_of!((*_31));
_3 = _13 << _8;
_8 = _2 - _2;
(*_31) = 22446_i16;
_27 = [9863436829419432262_usize,18332022705602627711_usize];
_23 = _18 as f32;
RET = _5;
Goto(bb9)
}
bb9 = {
_26 = [(*_31),(*_31)];
_10 = _3;
_20 = _30;
_9 = _14;
match Field::<i16>(Variant(_25, 1), 0) {
0 => bb10,
1 => bb11,
22446 => bb13,
_ => bb12
}
}
bb10 = {
Return()
}
bb11 = {
_18 = 103_i8 as f64;
_16 = !6769242417758459947_u64;
_10 = _1;
_18 = 487178634_u32 as f64;
_9 = [1317374266_i32,(-803601822_i32),(-1074662693_i32),508807815_i32,(-1649663395_i32),(-1386673264_i32),(-432387264_i32)];
_18 = 887567568_i32 as f64;
_9 = [1237702466_i32,2051935361_i32,(-1326044539_i32),(-624186721_i32),1152578120_i32,(-1761514033_i32),(-1909611072_i32)];
_24 = Adt52::Variant0 { fld0: _19 };
_23 = 62351_u16 as f32;
_21 = [_16,_16,_16];
_22.2 = 221602642055804846268942340007310430982_u128 as i64;
_18 = 2673759010_u32 as f64;
SetDiscriminant(_24, 0);
_20 = '\u{c838a}';
_19 = [662351878_u32,3434171350_u32,798850204_u32];
_15 = 11817462932830086690_usize as isize;
_16 = 14403874577682066984_u64;
_18 = _13 as f64;
_10 = !_3;
_6 = _2 >> _2;
_17 = [23921_i16,(-12297_i16)];
_25 = Adt60::Variant1 { fld0: 1833_i16,fld1: _21 };
_21 = [_16,_16,_16];
_1 = _10;
_17 = [31646_i16,3427_i16];
(*_12) = !_13;
match _16 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
14403874577682066984 => bb8,
_ => bb7
}
}
bb12 = {
Return()
}
bb13 = {
_32.1 = Field::<i16>(Variant(_25, 1), 0);
place!(Field::<[u32; 3]>(Variant(_24, 0), 0)) = [1230888358_u32,3845292906_u32,1669622750_u32];
_37 = [true,true,false,false,true,false];
_12 = core::ptr::addr_of_mut!((*_12));
_28 = (*_12) + (*_12);
_42 = [257810434699621215980423574023183078445_u128,72152120515540701199892706801834930370_u128,81210223841588247009067756032025105277_u128,126456654842476909190231654047725795580_u128];
_38 = [1149497915_u32,1172637129_u32,1871384303_u32];
_40 = [_3,(*_12)];
_25 = Adt60::Variant1 { fld0: _32.1,fld1: _21 };
_16 = 17779313245529522045_u64;
_19 = Field::<[u32; 3]>(Variant(_24, 0), 0);
_38 = [3124743282_u32,338593411_u32,3109697090_u32];
SetDiscriminant(_25, 1);
_41 = (*_12) | _3;
_23 = _8 as f32;
_32.0 = _27;
_7 = -_11;
_17 = _26;
_9 = _14;
_44 = _32.1 as i128;
_34 = _30;
Goto(bb14)
}
bb14 = {
SetDiscriminant(_24, 0);
_7 = _44 << _2;
_22.3 = 78_u8;
_12 = core::ptr::addr_of_mut!((*_12));
_22.2 = _8 - _8;
_32 = (_27, (-14372_i16));
_14 = [(-1441776953_i32),(-117692265_i32),(-1796369052_i32),2073083780_i32,1907434369_i32,(-1462533499_i32),(-1018849771_i32)];
_20 = _34;
place!(Field::<[u64; 3]>(Variant(_25, 1), 1)) = [_16,_16,_16];
_20 = _34;
_16 = _18 as u64;
place!(Field::<i16>(Variant(_25, 1), 0)) = _32.1;
_1 = _3 << _32.1;
_22.1 = 3375827851_u32 as usize;
_45 = _22.1;
Goto(bb15)
}
bb15 = {
Call(_48 = dump_var(18_usize, 22_usize, Move(_22), 17_usize, Move(_17), 34_usize, Move(_34), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_48 = dump_var(18_usize, 13_usize, Move(_13), 11_usize, Move(_11), 42_usize, Move(_42), 37_usize, Move(_37)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_48 = dump_var(18_usize, 45_usize, Move(_45), 7_usize, Move(_7), 4_usize, Move(_4), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(18_usize, 28_usize, Move(_28), 30_usize, Move(_30), 2_usize, Move(_2), 26_usize, Move(_26)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: [u128; 4],mut _2: *mut isize,mut _3: [i32; 7],mut _4: usize) -> [u128; 4] {
mir! {
type RET = [u128; 4];
let _5: bool;
let _6: [isize; 2];
let _7: isize;
let _8: i16;
let _9: [u64; 3];
let _10: [u32; 1];
let _11: u8;
let _12: [usize; 2];
let _13: [char; 7];
let _14: [u64; 3];
let _15: (*const f32, [char; 7], [u32; 3]);
let _16: ([usize; 2], i16);
let _17: [i16; 2];
let _18: *const i16;
let _19: Adt59;
let _20: usize;
let _21: char;
let _22: bool;
let _23: f64;
let _24: u64;
let _25: [i32; 5];
let _26: f32;
let _27: f64;
let _28: *const i128;
let _29: *const f64;
let _30: i16;
let _31: [u32; 3];
let _32: ();
let _33: ();
{
_4 = 3_usize;
_4 = 16042585645901238606_usize >> (*_2);
_4 = 7502408917722632778_u64 as usize;
_3 = [(-78338973_i32),1515003035_i32,(-1155362029_i32),527152210_i32,(-559029452_i32),1196968986_i32,(-374944519_i32)];
(*_2) = !(-52_isize);
_1 = [209230946446812263985196633707906122287_u128,267918486232938409706978730774862174309_u128,300799410245925173680972560139365902531_u128,296752436677339739060653807034625192601_u128];
RET = [196340012160771968297206057160607951071_u128,50998662367386336134112862287056234634_u128,235442012594992620151425607369777312815_u128,184898083300656213561286038502731585189_u128];
RET = [23886885709795213157301288936723192338_u128,130446130522474665988232368107745434489_u128,49605635307485540635136737526936789895_u128,293391871814660980336865409780671327361_u128];
RET = _1;
RET = [150636427761293199605519276594490614418_u128,179958309832846767273445998411572625639_u128,151072103654028014721164413726652300288_u128,197771490828314351163848131909275143348_u128];
_4 = 5001923682546577772_i64 as usize;
(*_2) = true as isize;
_6 = [(*_2),(*_2)];
RET = [324412863297283336813243330156822673786_u128,275156721291152061309917113305331273670_u128,175550124619412973191958962195775498995_u128,70378289267771916212842959122274555727_u128];
_3 = [(-1305331912_i32),162383399_i32,(-1813135732_i32),(-249217921_i32),(-1945111926_i32),(-1385975677_i32),223416041_i32];
RET = _1;
_5 = (*_2) == (*_2);
_1 = [170599753995988098806747094144927688965_u128,83458587985598201089116861555170829373_u128,171654935727308691362788731960630804015_u128,78892198057802109527890296301782461670_u128];
RET = [82620611224716559872600542617967402783_u128,53385814468194210248939087757034340548_u128,285427878025687781994366786735117096749_u128,23100578294171202794921416988056559189_u128];
_7 = (*_2) | (*_2);
_8 = 308969120_i32 as i16;
_4 = 1_usize;
Call((*_2) = core::intrinsics::transmute(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9[_4] = 17208373669566806344_u64 << _3[_4];
_11 = _8 as u8;
match _3[_4] {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
162383399 => bb8,
_ => bb7
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
_7 = '\u{5a670}' as isize;
(*_2) = _11 as isize;
_6 = [_7,_7];
Goto(bb9)
}
bb9 = {
_6[_4] = (*_2) | (*_2);
_9 = [13500548916566288848_u64,14301926293181272334_u64,10430126712154642069_u64];
_4 = 1848598209_u32 as usize;
RET = [108287245221086276005394129437806999164_u128,70854162681944261926507840500340487030_u128,241191980834297911269588011687629408010_u128,101760279493251932554818194679712662149_u128];
_8 = 22549_i16 * 14205_i16;
_15.2 = [2232632445_u32,3384045148_u32,189134307_u32];
_13 = ['\u{c5ccc}','\u{48e9a}','\u{8553f}','\u{9830e}','\u{40dee}','\u{6b119}','\u{b66e1}'];
_12 = [_4,_4];
_6 = [_7,_7];
_6 = [_7,(*_2)];
_9 = [14046102559439651624_u64,10932415292530694170_u64,15136141029099656427_u64];
_9 = [12635028182675079872_u64,5176060544230469734_u64,3034988606305031115_u64];
_16 = (_12, _8);
_17 = [_16.1,_8];
_11 = 231_u8 * 218_u8;
RET = _1;
Goto(bb10)
}
bb10 = {
_15.1 = ['\u{ead36}','\u{d1abc}','\u{7951a}','\u{61199}','\u{268a}','\u{284c6}','\u{4b103}'];
Call((*_2) = core::intrinsics::transmute(_7), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Goto(bb12)
}
bb12 = {
_18 = core::ptr::addr_of!(_8);
_21 = '\u{f55a}';
_16.1 = (*_18) * (*_18);
RET = [62545595114483904824855399752119290251_u128,311925909293351696891836350539958406792_u128,288921224422665359571557790344832695604_u128,58745451349004829966491590726342103789_u128];
_7 = -(*_2);
_16.1 = 276145253108321090_i64 as i16;
_10 = [570544623_u32];
_10 = [1184007069_u32];
_5 = _11 < _11;
_15.2 = [237726152_u32,2371895518_u32,1944565005_u32];
_22 = _5;
_14 = _9;
_23 = 2401347909_u32 as f64;
_24 = _11 as u64;
_24 = 12794373613870108443_u64;
_11 = _8 as u8;
RET = [281054883009065358924881001194820975636_u128,129394818231973032393616781456755725521_u128,117657974801432403846067268189897354600_u128,131963412927254030832764522954097561267_u128];
_6 = [(*_2),(*_2)];
_24 = !8437918619939497541_u64;
_16 = (_12, (*_18));
_8 = _4 as i16;
_24 = 7000298047991252560_u64;
Goto(bb13)
}
bb13 = {
_13 = [_21,_21,_21,_21,_21,_21,_21];
_22 = !_5;
_4 = 17809465807249450324_usize;
_15.0 = core::ptr::addr_of!(_26);
_14 = [_24,_24,_24];
_16.0 = [_4,_4];
_2 = core::ptr::addr_of_mut!(_7);
_9 = [_24,_24,_24];
_13 = _15.1;
_11 = 4248086094_u32 as u8;
_11 = !87_u8;
_25 = [(-1473005285_i32),210949047_i32,628730215_i32,2044211610_i32,773963199_i32];
_27 = _23 * _23;
match _4 {
0 => bb6,
1 => bb4,
2 => bb11,
3 => bb14,
4 => bb15,
5 => bb16,
17809465807249450324 => bb18,
_ => bb17
}
}
bb14 = {
_18 = core::ptr::addr_of!(_8);
_21 = '\u{f55a}';
_16.1 = (*_18) * (*_18);
RET = [62545595114483904824855399752119290251_u128,311925909293351696891836350539958406792_u128,288921224422665359571557790344832695604_u128,58745451349004829966491590726342103789_u128];
_7 = -(*_2);
_16.1 = 276145253108321090_i64 as i16;
_10 = [570544623_u32];
_10 = [1184007069_u32];
_5 = _11 < _11;
_15.2 = [237726152_u32,2371895518_u32,1944565005_u32];
_22 = _5;
_14 = _9;
_23 = 2401347909_u32 as f64;
_24 = _11 as u64;
_24 = 12794373613870108443_u64;
_11 = _8 as u8;
RET = [281054883009065358924881001194820975636_u128,129394818231973032393616781456755725521_u128,117657974801432403846067268189897354600_u128,131963412927254030832764522954097561267_u128];
_6 = [(*_2),(*_2)];
_24 = !8437918619939497541_u64;
_16 = (_12, (*_18));
_8 = _4 as i16;
_24 = 7000298047991252560_u64;
Goto(bb13)
}
bb15 = {
Return()
}
bb16 = {
_9[_4] = 17208373669566806344_u64 << _3[_4];
_11 = _8 as u8;
match _3[_4] {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
162383399 => bb8,
_ => bb7
}
}
bb17 = {
Return()
}
bb18 = {
(*_18) = _21 as i16;
_24 = 13175778675374904516_u64 - 14063053261918982170_u64;
_15.1 = _13;
_6 = [(*_2),(*_2)];
_27 = -_23;
_13 = [_21,_21,_21,_21,_21,_21,_21];
_4 = 10264497937041929050_usize;
_7 = -9223372036854775807_isize;
RET = [208893534184837024319517390395311159099_u128,330135737762303916588439239197138295512_u128,94803453678207150022031548042126522402_u128,156817396435940827128502567124949212740_u128];
_22 = _5;
_17 = [_16.1,(*_18)];
Goto(bb19)
}
bb19 = {
Call(_32 = dump_var(19_usize, 10_usize, Move(_10), 11_usize, Move(_11), 16_usize, Move(_16), 24_usize, Move(_24)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_32 = dump_var(19_usize, 12_usize, Move(_12), 6_usize, Move(_6), 5_usize, Move(_5), 17_usize, Move(_17)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_32 = dump_var(19_usize, 25_usize, Move(_25), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(109916531082195466595211742953878817168_u128), std::hint::black_box(2717664540_u32), std::hint::black_box((-8717186444065371943_i64)), std::hint::black_box(68_i8), std::hint::black_box((-6806_i16)));
                
            }
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: [char; 6],
fld1: [bool; 8],
fld2: ([isize; 2], usize, i64, u8),
fld3: u16,
fld4: *mut isize,

},
Variant1{
fld0: bool,
fld1: char,
fld2: u64,
fld3: ([isize; 2], usize, i64, u8),

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: bool,
fld1: *const [isize; 2],
fld2: isize,
fld3: [i16; 2],
fld4: *const f64,
fld5: *const f32,
fld6: u64,
fld7: [bool; 6],
}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt47{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt47 {
fld0: f64,
fld1: *const f32,
fld2: i16,
}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: [bool; 8],
fld1: [u32; 3],
fld2: (*const f32, [char; 7], [u32; 3]),
fld3: [i32; 7],
}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: [u128; 4],

},
Variant1{
fld0: u8,

},
Variant2{
fld0: *const [isize; 2],
fld1: f64,
fld2: Adt48,
fld3: [isize; 2],
fld4: u128,

},
Variant3{
fld0: u16,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: u128,
fld1: *mut isize,
fld2: u64,
fld3: Adt48,
fld4: *const [isize; 2],

},
Variant1{
fld0: Adt48,
fld1: [bool; 6],
fld2: Adt46,
fld3: usize,
fld4: u128,
fld5: u64,

},
Variant2{
fld0: [char; 7],
fld1: *mut i64,
fld2: f64,
fld3: ([usize; 2], i16),
fld4: *const i128,

},
Variant3{
fld0: bool,
fld1: Adt45,
fld2: usize,
fld3: f32,
fld4: Adt49,
fld5: *mut [char; 7],
fld6: *const i128,
fld7: (f64, u64, i32, f64),

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: i8,
fld1: [bool; 8],
fld2: Adt50,

},
Variant1{
fld0: [usize; 2],
fld1: char,
fld2: u8,
fld3: i32,
fld4: [bool; 8],

},
Variant2{
fld0: Adt49,
fld1: i32,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: [u32; 3],

},
Variant1{
fld0: (i32, u128),

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: Adt45,
fld1: *const i128,
fld2: usize,
fld3: ([usize; 2], i16),
fld4: f64,

},
Variant1{
fld0: u32,
fld1: [u128; 4],

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt54{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt54 {
fld0: (*const f32, [char; 7], [u32; 3]),
fld1: [usize; 2],
fld2: [i32; 5],
fld3: i8,
fld4: f32,
fld5: u16,
fld6: [u64; 3],
}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: Adt50,
fld1: Adt49,

},
Variant1{
fld0: Adt52,
fld1: char,
fld2: [u64; 3],
fld3: (i32, u128),
fld4: [bool; 6],

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: Adt52,

},
Variant1{
fld0: Adt48,
fld1: char,
fld2: *const i16,
fld3: [bool; 8],
fld4: i16,
fld5: i32,
fld6: [char; 7],
fld7: [bool; 6],

},
Variant2{
fld0: u32,
fld1: f32,
fld2: *mut [char; 7],

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: [i16; 2],
fld1: i16,
fld2: [usize; 2],
fld3: usize,

},
Variant1{
fld0: Adt55,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: [bool; 6],
fld1: usize,
fld2: Adt55,
fld3: (*const f32, [char; 7], [u32; 3]),

},
Variant1{
fld0: Adt57,
fld1: *mut [char; 7],
fld2: f32,
fld3: [i32; 5],
fld4: Adt50,

},
Variant2{
fld0: u8,
fld1: [isize; 2],
fld2: Adt48,
fld3: Adt45,
fld4: *mut isize,
fld5: [u64; 3],
fld6: i64,
fld7: [bool; 6],

},
Variant3{
fld0: u64,
fld1: *const i16,
fld2: ([isize; 2], usize, i64, u8),

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf("Adt59::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: Adt49,
fld1: char,
fld2: *const f64,
fld3: i8,
fld4: Adt50,
fld5: f32,
fld6: *mut i64,

},
Variant1{
fld0: f32,
fld1: i64,
fld2: Adt49,
fld3: i8,

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf("Adt60::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: Adt55,

},
Variant1{
fld0: i16,
fld1: [u64; 3],

},
Variant2{
fld0: [u128; 4],
fld1: Adt45,
fld2: [i32; 5],
fld3: *const i16,
fld4: Adt47,
fld5: u64,

}}
impl PrintFDebug for Adt61{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt61{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt61 {
fld0: u8,
fld1: u64,
fld2: *mut isize,
fld3: i8,
fld4: Adt48,
fld5: Adt56,
fld6: *const i128,
}

