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
pub fn fn0(mut _1: i64,mut _2: u8,mut _3: u16,mut _4: i8,mut _5: i128,mut _6: u64) -> *const [bool; 4] {
mir! {
type RET = *const [bool; 4];
let _7: u64;
let _8: (i16, u64, bool, isize);
let _9: &'static [i16; 7];
let _10: isize;
let _11: f32;
let _12: (u64, [bool; 2]);
let _13: f32;
let _14: i16;
let _15: &'static [u64; 5];
let _16: i32;
let _17: [char; 6];
let _18: char;
let _19: Adt37;
let _20: [u32; 6];
let _21: f64;
let _22: [i32; 3];
let _23: i32;
let _24: [u64; 5];
let _25: [i32; 3];
let _26: ();
let _27: ();
{
_4 = -(-75_i8);
_5 = -160256755386838650021384094360762811526_i128;
_7 = !3441917420231060447_u64;
_1 = -(-1836489257760449044_i64);
_3 = 21653_u16;
_1 = 8104374825659017072_i64;
_2 = 3385227160_u32 as u8;
_7 = 13564464020459907131_u64 + 7220667418194157694_u64;
_3 = 23651_u16 >> _7;
_6 = 13870_i16 as u64;
_5 = !168088547705423789913437027724116917620_i128;
_6 = _7 | _7;
_2 = 202_u8;
_1 = _6 as i64;
_2 = !248_u8;
_6 = _7 - _7;
_3 = !48962_u16;
_4 = 2014516318_i32 as i8;
_6 = _7;
_7 = _6;
_1 = !(-3400443071065648579_i64);
_2 = 117_u8;
_7 = !_6;
_3 = !19330_u16;
Call(RET = fn1(_5, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = (-142474499187686278497218264975607166470_i128);
_8 = (26274_i16, _7, false, 9223372036854775807_isize);
_1 = !(-6501111278298558840_i64);
_7 = _6 >> _6;
_3 = !63090_u16;
_2 = 75_u8 ^ 179_u8;
_1 = _3 as i64;
_5 = -(-55225261907077651817965996414070683024_i128);
_1 = 21907757091599057_i64 * 1530048768499519060_i64;
_4 = -18_i8;
_8.1 = _7;
_8.2 = true | false;
_8.1 = 141744478898338762302108388620623716448_u128 as u64;
_3 = 39221_u16;
_1 = (-6982169073653562202_i64);
_2 = 132_u8;
_8.3 = 9223372036854775807_isize;
_4 = (-30_i8);
_8.3 = _2 as isize;
_8.0 = 27362_i16;
_8.2 = true;
_3 = !17999_u16;
_5 = -(-78402893726315074976184070338197252286_i128);
_8.0 = (-24850_i16) | 22209_i16;
match _1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463456392438358114649254 => bb7,
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
_1 = 37697270542664920081581884453556544978_u128 as i64;
_1 = (-8989581227618034466_i64);
_8.3 = 1277890236_u32 as isize;
_2 = 36_u8 << _8.0;
_10 = _7 as isize;
_8.2 = false;
_5 = -12670278204234283510780498356272778494_i128;
_12.0 = _7;
_11 = _5 as f32;
_8.1 = _7 >> _8.0;
_8.1 = _12.0;
_12.0 = _6 ^ _6;
_3 = 3740_u16 - 15443_u16;
_2 = 248_u8;
_4 = 677948385_i32 as i8;
_12.1 = [_8.2,_8.2];
Goto(bb8)
}
bb8 = {
_12.1 = [_8.2,_8.2];
_8.1 = 336103287426787048961483803113880626210_u128 as u64;
_12.1 = [_8.2,_8.2];
_8.3 = _2 as isize;
_2 = !8_u8;
_8.3 = _10;
match _1 {
0 => bb4,
1 => bb9,
2 => bb10,
3 => bb11,
340282366920938463454385026204150176990 => bb13,
_ => bb12
}
}
bb9 = {
_1 = 37697270542664920081581884453556544978_u128 as i64;
_1 = (-8989581227618034466_i64);
_8.3 = 1277890236_u32 as isize;
_2 = 36_u8 << _8.0;
_10 = _7 as isize;
_8.2 = false;
_5 = -12670278204234283510780498356272778494_i128;
_12.0 = _7;
_11 = _5 as f32;
_8.1 = _7 >> _8.0;
_8.1 = _12.0;
_12.0 = _6 ^ _6;
_3 = 3740_u16 - 15443_u16;
_2 = 248_u8;
_4 = 677948385_i32 as i8;
_12.1 = [_8.2,_8.2];
Goto(bb8)
}
bb10 = {
Return()
}
bb11 = {
_5 = (-142474499187686278497218264975607166470_i128);
_8 = (26274_i16, _7, false, 9223372036854775807_isize);
_1 = !(-6501111278298558840_i64);
_7 = _6 >> _6;
_3 = !63090_u16;
_2 = 75_u8 ^ 179_u8;
_1 = _3 as i64;
_5 = -(-55225261907077651817965996414070683024_i128);
_1 = 21907757091599057_i64 * 1530048768499519060_i64;
_4 = -18_i8;
_8.1 = _7;
_8.2 = true | false;
_8.1 = 141744478898338762302108388620623716448_u128 as u64;
_3 = 39221_u16;
_1 = (-6982169073653562202_i64);
_2 = 132_u8;
_8.3 = 9223372036854775807_isize;
_4 = (-30_i8);
_8.3 = _2 as isize;
_8.0 = 27362_i16;
_8.2 = true;
_3 = !17999_u16;
_5 = -(-78402893726315074976184070338197252286_i128);
_8.0 = (-24850_i16) | 22209_i16;
match _1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463456392438358114649254 => bb7,
_ => bb6
}
}
bb12 = {
Return()
}
bb13 = {
_6 = !_7;
_8 = (4973_i16, _6, true, _10);
_8.1 = _7;
_1 = 1861269488472384351_i64 * (-105136897555648270_i64);
_3 = 5920_u16 >> _8.0;
_1 = -(-5770114884943442213_i64);
_10 = _8.3;
_2 = 43_u8;
_8.3 = _8.2 as isize;
_16 = !(-602866343_i32);
_3 = _7 as u16;
_11 = 1_usize as f32;
_6 = _7 | _8.1;
_16 = 1691400887_i32 << _3;
_13 = _11 + _11;
_17 = ['\u{e6bc1}','\u{b8782}','\u{b4a00}','\u{63b7}','\u{8dc45}','\u{b5b91}'];
_8.3 = !_10;
_8.3 = _10;
_8.0 = 29841_i16 >> _16;
_2 = !21_u8;
_11 = _13;
_1 = 6_usize as i64;
_3 = 53613_u16;
_17 = ['\u{bf832}','\u{3a012}','\u{7542d}','\u{d1904}','\u{b5775}','\u{b0986}'];
_13 = _3 as f32;
_13 = -_11;
_1 = (-3230089869420983148_i64) & (-7923938084055568852_i64);
_12.0 = !_6;
Goto(bb14)
}
bb14 = {
_16 = (-686444845_i32) & 1543442046_i32;
_8.1 = _12.0 << _1;
_8.2 = false;
_8.1 = _12.0;
_12.1 = [_8.2,_8.2];
_5 = 149861814078450615504999879301852506413_i128;
_18 = '\u{43f90}';
_2 = 62_u8 | 47_u8;
_12.0 = !_7;
_21 = _8.1 as f64;
_21 = _8.3 as f64;
_20 = [2533796428_u32,3901754388_u32,1599221574_u32,381330725_u32,2861335305_u32,933895539_u32];
_1 = 8932032466288529033_i64;
_14 = _8.0 << _6;
_7 = _12.0;
_10 = _8.3;
_14 = _8.0;
_8.0 = _14;
_8 = (_14, _6, true, _10);
_17 = [_18,_18,_18,_18,_18,_18];
_4 = _21 as i8;
_22 = [_16,_16,_16];
_6 = _3 as u64;
_23 = _16;
_6 = _8.1 >> _8.1;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(0_usize, 17_usize, Move(_17), 7_usize, Move(_7), 16_usize, Move(_16), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(0_usize, 4_usize, Move(_4), 18_usize, Move(_18), 6_usize, Move(_6), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i128,mut _2: u16) -> *const [bool; 4] {
mir! {
type RET = *const [bool; 4];
let _3: i32;
let _4: *const ([bool; 2], u64, &'static i8, (u64, [bool; 2]));
let _5: u16;
let _6: i16;
let _7: char;
let _8: *const *mut [bool; 2];
let _9: [bool; 4];
let _10: Adt60;
let _11: u128;
let _12: &'static &'static i8;
let _13: char;
let _14: ([u64; 5], (u64, [bool; 2]), usize, i128);
let _15: &'static u16;
let _16: (i8, [u64; 3]);
let _17: f64;
let _18: (usize, [isize; 8], [i16; 7], (i128,));
let _19: (i16, i8);
let _20: u8;
let _21: *mut u16;
let _22: i8;
let _23: &'static ((f64, Adt37), i8, *const [usize; 4]);
let _24: &'static usize;
let _25: *const [usize; 4];
let _26: &'static (i128,);
let _27: (&'static [isize; 8], ([bool; 2], u64, &'static i8, (u64, [bool; 2])), (i128,));
let _28: [i32; 3];
let _29: ();
let _30: ();
{
_2 = 16925_u16 ^ 53214_u16;
_2 = !37729_u16;
_2 = false as u16;
_2 = 4669_u16 >> _1;
_2 = 18154_u16 ^ 28418_u16;
_2 = 44900_u16;
_3 = !13638291_i32;
_1 = 166145954123138229789713501242916472720_i128;
_2 = 58391_u16 >> _3;
_3 = 6021101212617464864_u64 as i32;
_1 = 3142773907_u32 as i128;
_2 = !470_u16;
Goto(bb1)
}
bb1 = {
_2 = !30348_u16;
_1 = 163374862531957772707313315842073273730_i128 + (-121027299472155895642727007756006296691_i128);
_3 = !527136006_i32;
_1 = 73884373438635510277634590104775423017_i128 * (-116972340862399273344686752745196514505_i128);
_3 = 3828990608258988423_u64 as i32;
_3 = '\u{263a0}' as i32;
_2 = 30361_u16;
_2 = !64362_u16;
_3 = 2091601094_i32;
_2 = false as u16;
_1 = 23408_i16 as i128;
_2 = 53565_u16;
_2 = !7770_u16;
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
2091601094 => bb9,
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
_3 = 73809846_i32 & 200583806_i32;
_6 = 19850_i16;
_5 = _2 << _3;
_1 = 72428127561041199379916571451699427797_i128;
_5 = 17942244657889727118_u64 as u16;
_6 = 31406_i16 >> _1;
_5 = _2;
_2 = 69572743771978574849208062051846740585_u128 as u16;
_2 = _5 ^ _5;
_3 = (-489312078_i32) << _5;
_1 = 48300387525494282484877214390600383834_i128 - (-81641322736730741693669121505771237595_i128);
_3 = 179699442589196824_i64 as i32;
_1 = 4210728083849157519_i64 as i128;
_2 = _5;
Goto(bb10)
}
bb10 = {
_7 = '\u{8028b}';
_1 = 86216323581247013819240617764234241806_i128 + (-27958849320265339861767400094865543100_i128);
_1 = (-161918852973046838884175656698844410440_i128);
RET = core::ptr::addr_of!(_9);
_2 = _5;
(*RET) = [true,true,false,false];
(*RET) = [true,false,true,false];
_7 = '\u{78c3}';
(*RET) = [false,true,true,true];
_11 = 7831278540294782581464740137302060058_u128;
_2 = _5 & _5;
RET = core::ptr::addr_of!(_9);
(*RET) = [true,true,true,false];
(*RET) = [true,false,false,true];
(*RET) = [false,true,false,false];
_2 = _5;
_1 = (-137748060169993918265713318112125122822_i128);
_6 = !10284_i16;
Call(RET = fn2((*RET), _1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_13 = _7;
_5 = _2;
_3 = (-1513233320_i32) - 1124626657_i32;
_1 = -(-47860585073171222298637783553550631359_i128);
_14.3 = !_1;
_13 = _7;
_14.0 = [17699974139132603362_u64,11203074410186349730_u64,17035481226932743857_u64,9521017007850764300_u64,2185273299332365367_u64];
_14.3 = _1 * _1;
_14.1.1 = [false,true];
_5 = false as u16;
_7 = _13;
_14.1.1 = [true,false];
_11 = 9125626708136891546_i64 as u128;
RET = core::ptr::addr_of!(_9);
Goto(bb12)
}
bb12 = {
_14.1.0 = 2369653919_u32 as u64;
_14.1.1 = [true,false];
_1 = _14.3;
_16.1 = [_14.1.0,_14.1.0,_14.1.0];
_9 = [false,false,false,false];
_14.2 = !6416133188524386757_usize;
(*RET) = [true,false,true,true];
_14.1.0 = 11759022580979975772_u64;
_14.1.1 = [true,true];
_14.3 = _14.2 as i128;
_14.3 = !_1;
_1 = !_14.3;
RET = core::ptr::addr_of!((*RET));
_5 = _2 + _2;
_17 = _1 as f64;
Goto(bb13)
}
bb13 = {
RET = core::ptr::addr_of!(_9);
_15 = &_2;
_5 = _6 as u16;
_14.3 = _7 as i128;
_17 = _6 as f64;
_14.2 = !3_usize;
_18.0 = (-113_i8) as usize;
_6 = _7 as i16;
(*RET) = [true,true,true,false];
_18.2 = [_6,_6,_6,_6,_6,_6,_6];
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!(_9);
_18.3 = (_1,);
_19 = (_6, (-70_i8));
_3 = (-1486704402_i32) << _18.3.0;
RET = core::ptr::addr_of!((*RET));
_14.3 = _1 - _18.3.0;
_16.1 = [_14.1.0,_14.1.0,_14.1.0];
RET = core::ptr::addr_of!((*RET));
_14.1.0 = 5437373824078680054_u64;
_14.2 = _18.0;
(*RET) = [false,false,false,true];
(*RET) = [false,true,true,true];
_21 = core::ptr::addr_of_mut!((*_15));
_14.2 = !_18.0;
RET = core::ptr::addr_of!(_9);
match _19.1 {
0 => bb9,
1 => bb2,
2 => bb11,
340282366920938463463374607431768211386 => bb15,
_ => bb14
}
}
bb14 = {
_14.1.0 = 2369653919_u32 as u64;
_14.1.1 = [true,false];
_1 = _14.3;
_16.1 = [_14.1.0,_14.1.0,_14.1.0];
_9 = [false,false,false,false];
_14.2 = !6416133188524386757_usize;
(*RET) = [true,false,true,true];
_14.1.0 = 11759022580979975772_u64;
_14.1.1 = [true,true];
_14.3 = _14.2 as i128;
_14.3 = !_1;
_1 = !_14.3;
RET = core::ptr::addr_of!((*RET));
_5 = _2 + _2;
_17 = _1 as f64;
Goto(bb13)
}
bb15 = {
(*RET) = [true,true,false,false];
_14.1.1 = [true,false];
_18.1 = [(-9223372036854775808_isize),(-29_isize),9223372036854775807_isize,(-9223372036854775808_isize),116_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_14.1.0 = !6288372644564045161_u64;
_22 = _18.0 as i8;
_18.2 = [_6,_6,_19.0,_19.0,_19.0,_6,_6];
_15 = &_5;
_14.3 = _1 - _18.3.0;
_22 = (-59_isize) as i8;
_14.0 = [_14.1.0,_14.1.0,_14.1.0,_14.1.0,_14.1.0];
_14.1.1 = [false,false];
_15 = &(*_15);
_14.1.0 = 119_u8 as u64;
_15 = &(*_21);
_19 = (_6, _22);
_15 = &_2;
_14.1.1 = [false,true];
_14.0 = [_14.1.0,_14.1.0,_14.1.0,_14.1.0,_14.1.0];
_12 = &_27.1.2;
_27.1.2 = &_22;
_16.1 = [_14.1.0,_14.1.0,_14.1.0];
_24 = &_14.2;
_18.0 = (*_24) << _14.3;
_27.1.1 = _14.1.0 * _14.1.0;
_3 = (-2018786572_i32) + (-1923703870_i32);
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(1_usize, 13_usize, Move(_13), 7_usize, Move(_7), 11_usize, Move(_11), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(1_usize, 6_usize, Move(_6), 5_usize, Move(_5), 30_usize, _30, 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: [bool; 4],mut _2: i128) -> *const [bool; 4] {
mir! {
type RET = *const [bool; 4];
let _3: f64;
let _4: f64;
let _5: isize;
let _6: u16;
let _7: (isize,);
let _8: isize;
let _9: &'static (i16, i8);
let _10: *const ([bool; 2], u64, &'static i8, (u64, [bool; 2]));
let _11: &'static [i16; 7];
let _12: isize;
let _13: f32;
let _14: f32;
let _15: isize;
let _16: Adt71;
let _17: f32;
let _18: isize;
let _19: [bool; 4];
let _20: i64;
let _21: u8;
let _22: &'static [char; 6];
let _23: (&'static [isize; 8], ([bool; 2], u64, &'static i8, (u64, [bool; 2])), (i128,));
let _24: (i16, i8);
let _25: (&'static [isize; 8], ([bool; 2], u64, &'static i8, (u64, [bool; 2])), (i128,));
let _26: &'static [u32; 6];
let _27: isize;
let _28: ();
let _29: ();
{
_2 = 203023745841749258106786810284857765016_u128 as i128;
RET = core::ptr::addr_of!(_1);
(*RET) = [false,true,true,true];
(*RET) = [false,false,false,false];
(*RET) = [true,false,false,true];
(*RET) = [false,false,false,true];
(*RET) = [true,true,true,true];
(*RET) = [true,true,true,false];
_2 = (-63985117619651855385972683842385770190_i128) * 146205510378144966098014209727016134998_i128;
(*RET) = [true,true,true,true];
(*RET) = [false,true,false,false];
(*RET) = [true,false,false,true];
(*RET) = [true,true,true,true];
RET = core::ptr::addr_of!(_1);
(*RET) = [true,true,false,true];
_3 = 257890275848809770455332698650891023774_u128 as f64;
(*RET) = [false,true,true,true];
(*RET) = [true,true,true,true];
_1 = [true,true,false,false];
_5 = (-9223372036854775808_isize) << _2;
Goto(bb1)
}
bb1 = {
_5 = 9541331574844420382_u64 as isize;
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!(_1);
_5 = (-9223372036854775808_isize);
RET = core::ptr::addr_of!((*RET));
_4 = _3;
_3 = _4 - _4;
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!((*RET));
(*RET) = [false,false,true,false];
_7 = (_5,);
_8 = !_5;
RET = core::ptr::addr_of!(_1);
_6 = 19056_u16;
(*RET) = [false,false,false,false];
_7.0 = -_8;
RET = core::ptr::addr_of!(_1);
RET = core::ptr::addr_of!((*RET));
_5 = !_7.0;
Call((*RET) = fn3(Move(RET), _8, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = core::ptr::addr_of!(_1);
_13 = _2 as f32;
_6 = 58796_u16 ^ 17395_u16;
_1 = [true,true,true,true];
RET = core::ptr::addr_of!(_1);
(*RET) = [true,true,false,false];
_12 = _3 as isize;
_14 = _13 + _13;
(*RET) = [false,false,false,true];
_3 = _12 as f64;
_7.0 = _12;
(*RET) = [true,true,true,false];
_5 = -_12;
_14 = _13 + _13;
(*RET) = [false,false,false,true];
_6 = 12510_u16;
match _6 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
12510 => bb9,
_ => bb8
}
}
bb3 = {
_5 = 9541331574844420382_u64 as isize;
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!(_1);
_5 = (-9223372036854775808_isize);
RET = core::ptr::addr_of!((*RET));
_4 = _3;
_3 = _4 - _4;
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!((*RET));
(*RET) = [false,false,true,false];
_7 = (_5,);
_8 = !_5;
RET = core::ptr::addr_of!(_1);
_6 = 19056_u16;
(*RET) = [false,false,false,false];
_7.0 = -_8;
RET = core::ptr::addr_of!(_1);
RET = core::ptr::addr_of!((*RET));
_5 = !_7.0;
Call((*RET) = fn3(Move(RET), _8, _2), ReturnTo(bb2), UnwindUnreachable())
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
_18 = 9129268284215199463_i64 as isize;
(*RET) = [false,true,true,false];
_17 = -_14;
_13 = _17 - _17;
_2 = (-158719504031335786334317199911279924683_i128) | (-21543365641769995539815406838024960495_i128);
_15 = 1350718987_i32 as isize;
RET = core::ptr::addr_of!((*RET));
_4 = _3 + _3;
_1 = [true,true,true,false];
_19 = (*RET);
_13 = _14;
(*RET) = [false,false,false,false];
_20 = 732246245361695608_i64;
_7.0 = _5;
(*RET) = _19;
(*RET) = [false,false,true,true];
_21 = 209_u8;
_3 = 288884730847312109569793186988759627886_u128 as f64;
_18 = _5 ^ _7.0;
_7 = (_12,);
_20 = _2 as i64;
_13 = _14 + _17;
_1 = [false,false,true,true];
_2 = 10681068641381784032_usize as i128;
_20 = 2174204755720022900_i64;
_2 = 598106336692068408_u64 as i128;
match _21 {
0 => bb2,
209 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
(*RET) = _19;
(*RET) = [false,true,true,true];
_18 = !_12;
_7.0 = _18;
_5 = _7.0;
_7 = (_5,);
Goto(bb12)
}
bb12 = {
_4 = _3 - _3;
_7 = (_5,);
(*RET) = [true,false,false,false];
_18 = _8 ^ _5;
_3 = -_4;
_6 = 50145_u16;
_8 = !_5;
_13 = 48_i8 as f32;
_18 = 1524325293_i32 as isize;
_12 = _18 ^ _5;
_13 = -_17;
(*RET) = [false,false,true,false];
RET = core::ptr::addr_of!((*RET));
_3 = _4 + _4;
(*RET) = [true,true,false,true];
_2 = (-132436629621116904729858452689487492370_i128) >> _12;
_21 = !178_u8;
_5 = _15;
(*RET) = [true,true,false,false];
RET = core::ptr::addr_of!(_19);
_23.1.0 = [false,true];
_5 = _12;
match _6 {
0 => bb2,
50145 => bb14,
_ => bb13
}
}
bb13 = {
_5 = 9541331574844420382_u64 as isize;
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!(_1);
_5 = (-9223372036854775808_isize);
RET = core::ptr::addr_of!((*RET));
_4 = _3;
_3 = _4 - _4;
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!((*RET));
(*RET) = [false,false,true,false];
_7 = (_5,);
_8 = !_5;
RET = core::ptr::addr_of!(_1);
_6 = 19056_u16;
(*RET) = [false,false,false,false];
_7.0 = -_8;
RET = core::ptr::addr_of!(_1);
RET = core::ptr::addr_of!((*RET));
_5 = !_7.0;
Call((*RET) = fn3(Move(RET), _8, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_23.1.0 = [true,false];
_21 = 246_u8;
_23.1.0 = [false,false];
_23.2.0 = _2 ^ _2;
_13 = 14566243855913307524_u64 as f32;
_20 = (-7006278101628362876_i64);
_23.1.2 = &_24.1;
_23.2 = (_2,);
_25.1.3.0 = 6121024935657562122_usize as u64;
_7.0 = false as isize;
_25.1.1 = true as u64;
_25.1.0 = [false,true];
_14 = -_13;
_8 = _25.1.3.0 as isize;
RET = core::ptr::addr_of!((*RET));
_3 = _4;
_15 = -_5;
_27 = _6 as isize;
_23.1.3.1 = _23.1.0;
_10 = core::ptr::addr_of!(_23.1);
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(2_usize, 15_usize, Move(_15), 27_usize, Move(_27), 2_usize, Move(_2), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(2_usize, 5_usize, Move(_5), 7_usize, Move(_7), 29_usize, _29, 29_usize, _29), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: *const [bool; 4],mut _2: isize,mut _3: i128) -> [bool; 4] {
mir! {
type RET = [bool; 4];
let _4: (i64,);
let _5: (isize,);
let _6: Adt49;
let _7: bool;
let _8: (i16, u64, bool, isize);
let _9: i16;
let _10: *mut u16;
let _11: &'static [i16; 7];
let _12: u64;
let _13: (u64, [bool; 2]);
let _14: ([u64; 5], (u64, [bool; 2]), usize, i128);
let _15: [usize; 4];
let _16: Adt68;
let _17: f32;
let _18: Adt68;
let _19: f32;
let _20: isize;
let _21: Adt68;
let _22: ([u64; 5], (u64, [bool; 2]), usize, i128);
let _23: bool;
let _24: [usize; 4];
let _25: bool;
let _26: (&'static usize,);
let _27: i64;
let _28: isize;
let _29: [u64; 3];
let _30: i64;
let _31: Adt62;
let _32: char;
let _33: (i16, i8);
let _34: u8;
let _35: char;
let _36: Adt68;
let _37: ();
let _38: ();
{
_3 = !(-137455151100526413270345910309938539784_i128);
_1 = core::ptr::addr_of!(RET);
_2 = (-1819055587_i32) as isize;
Goto(bb1)
}
bb1 = {
RET = [false,true,true,false];
_1 = core::ptr::addr_of!((*_1));
_4.0 = -3459172439614571407_i64;
_2 = _3 as isize;
RET = [false,false,false,false];
Call(_7 = fn4((*_1), Move(_1), _4, RET, (*_1), (*_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = [_7,_7,_7,_7];
_6.fld2 = [11611112884161466587_u64,13135246467250591488_u64,2946845946105992179_u64,6666402679148181756_u64,13636103186092226481_u64];
_6.fld0.0 = 2_usize as f64;
_8.2 = !_7;
RET = [_7,_8.2,_8.2,_7];
_8 = (17939_i16, 3585987226712802512_u64, _7, _2);
_6.fld4 = [_8.1,_8.1,_8.1];
_4.0 = (-2764868328585637190_i64) * 7603936106077989071_i64;
_5.0 = -_2;
_2 = _8.3;
_6.fld3 = 94_i8;
_8.0 = 4242_i16 + (-6948_i16);
_6.fld0.1 = Adt37::Variant1 { fld0: _6.fld2,fld1: 20850_u16,fld2: _2 };
place!(Field::<[u64; 5]>(Variant(_6.fld0.1, 1), 0)) = _6.fld2;
_1 = core::ptr::addr_of!(RET);
_8.0 = (-1316_i16) ^ (-13802_i16);
_5.0 = -_8.3;
_3 = (-22410882142535587880764850854634621422_i128) + (-25894511386735398375656524186866061724_i128);
_6.fld1 = [_4.0,_4.0,_4.0,_4.0];
_4 = (8521405691357731944_i64,);
_6.fld0.0 = _3 as f64;
_6.fld0.1 = Adt37::Variant1 { fld0: _6.fld2,fld1: 46936_u16,fld2: _2 };
place!(Field::<isize>(Variant(_6.fld0.1, 1), 2)) = _8.3;
_6.fld4 = [_8.1,_8.1,_8.1];
_8.0 = (-21815_i16) - (-6806_i16);
_6.fld0.0 = 195_u8 as f64;
_4.0 = (-3944265594943197865_i64);
_6.fld0.0 = _8.0 as f64;
RET = [_8.2,_8.2,_8.2,_7];
Call(_2 = core::intrinsics::transmute(Field::<isize>(Variant(_6.fld0.1, 1), 2)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = core::ptr::addr_of!(RET);
_8.3 = _2 & Field::<isize>(Variant(_6.fld0.1, 1), 2);
_7 = !_8.2;
Goto(bb4)
}
bb4 = {
RET = [_7,_7,_7,_8.2];
_15 = [16137094561556720414_usize,5_usize,2_usize,4_usize];
(*_1) = [_7,_7,_8.2,_8.2];
_7 = _8.2;
_6.fld1 = [_4.0,_4.0,_4.0,_4.0];
place!(Field::<u16>(Variant(_6.fld0.1, 1), 1)) = _3 as u16;
match _8.1 {
3585987226712802512 => bb6,
_ => bb5
}
}
bb5 = {
RET = [false,true,true,false];
_1 = core::ptr::addr_of!((*_1));
_4.0 = -3459172439614571407_i64;
_2 = _3 as isize;
RET = [false,false,false,false];
Call(_7 = fn4((*_1), Move(_1), _4, RET, (*_1), (*_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_5 = (_8.3,);
RET = [_8.2,_8.2,_7,_8.2];
_14.1.1 = [_8.2,_7];
_13.0 = _8.0 as u64;
_10 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_6.fld0.1, 1), 1)));
_1 = core::ptr::addr_of!((*_1));
_8.0 = 6291_i16;
_14.2 = 4_usize;
place!(Field::<[u64; 5]>(Variant(_6.fld0.1, 1), 0)) = [_8.1,_13.0,_8.1,_8.1,_13.0];
_13 = (_8.1, _14.1.1);
_6.fld4 = [_8.1,_13.0,_13.0];
_13.0 = !_8.1;
(*_10) = 14235_u16 & 24834_u16;
_8.2 = _7;
_4.0 = 2077559328506329656_i64;
_17 = _6.fld3 as f32;
_9 = _8.0 + _8.0;
_14.3 = !_3;
place!(Field::<isize>(Variant(_6.fld0.1, 1), 2)) = _5.0;
match _4.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
2077559328506329656 => bb9,
_ => bb8
}
}
bb7 = {
_1 = core::ptr::addr_of!(RET);
_8.3 = _2 & Field::<isize>(Variant(_6.fld0.1, 1), 2);
_7 = !_8.2;
Goto(bb4)
}
bb8 = {
RET = [_7,_7,_7,_7];
_6.fld2 = [11611112884161466587_u64,13135246467250591488_u64,2946845946105992179_u64,6666402679148181756_u64,13636103186092226481_u64];
_6.fld0.0 = 2_usize as f64;
_8.2 = !_7;
RET = [_7,_8.2,_8.2,_7];
_8 = (17939_i16, 3585987226712802512_u64, _7, _2);
_6.fld4 = [_8.1,_8.1,_8.1];
_4.0 = (-2764868328585637190_i64) * 7603936106077989071_i64;
_5.0 = -_2;
_2 = _8.3;
_6.fld3 = 94_i8;
_8.0 = 4242_i16 + (-6948_i16);
_6.fld0.1 = Adt37::Variant1 { fld0: _6.fld2,fld1: 20850_u16,fld2: _2 };
place!(Field::<[u64; 5]>(Variant(_6.fld0.1, 1), 0)) = _6.fld2;
_1 = core::ptr::addr_of!(RET);
_8.0 = (-1316_i16) ^ (-13802_i16);
_5.0 = -_8.3;
_3 = (-22410882142535587880764850854634621422_i128) + (-25894511386735398375656524186866061724_i128);
_6.fld1 = [_4.0,_4.0,_4.0,_4.0];
_4 = (8521405691357731944_i64,);
_6.fld0.0 = _3 as f64;
_6.fld0.1 = Adt37::Variant1 { fld0: _6.fld2,fld1: 46936_u16,fld2: _2 };
place!(Field::<isize>(Variant(_6.fld0.1, 1), 2)) = _8.3;
_6.fld4 = [_8.1,_8.1,_8.1];
_8.0 = (-21815_i16) - (-6806_i16);
_6.fld0.0 = 195_u8 as f64;
_4.0 = (-3944265594943197865_i64);
_6.fld0.0 = _8.0 as f64;
RET = [_8.2,_8.2,_8.2,_7];
Call(_2 = core::intrinsics::transmute(Field::<isize>(Variant(_6.fld0.1, 1), 2)), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_8.3 = _5.0 << _14.3;
_6.fld3 = 90_i8 | (-70_i8);
_6.fld2 = [_13.0,_8.1,_13.0,_13.0,_13.0];
_14.1.0 = _8.1 + _8.1;
_13.0 = '\u{952d5}' as u64;
_14 = (Field::<[u64; 5]>(Variant(_6.fld0.1, 1), 0), _13, 7_usize, _3);
_6.fld0.0 = 4209735470_u32 as f64;
_6.fld2 = [_14.1.0,_8.1,_14.1.0,_8.1,_8.1];
_10 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_6.fld0.1, 1), 1)));
_22.3 = _3 + _14.3;
(*_1) = [_7,_8.2,_8.2,_7];
_20 = Field::<u16>(Variant(_6.fld0.1, 1), 1) as isize;
place!(Field::<u16>(Variant(_6.fld0.1, 1), 1)) = 53098_u16 | 45848_u16;
_13.1 = _14.1.1;
_4.0 = (-6656263171121255919_i64);
_8.1 = _13.0 >> _8.0;
_22.1 = _14.1;
_8.1 = _22.1.0;
_9 = _8.0;
_6.fld0.1 = Adt37::Variant1 { fld0: _6.fld2,fld1: 6923_u16,fld2: _5.0 };
_7 = _14.2 != _14.2;
_8.0 = _9 - _9;
_14.2 = _17 as usize;
match _9 {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb4,
6291 => bb10,
_ => bb8
}
}
bb10 = {
_22.1.1 = _13.1;
_23 = !_7;
_25 = !_8.2;
_8.1 = 806519268_i32 as u64;
_18 = Adt68::Variant1 { fld0: _6.fld4,fld1: 1070472198_i32,fld2: _2,fld3: _6.fld1,fld4: _17 };
_27 = _14.1.0 as i64;
_22.2 = _14.2;
_14.1.1 = [_7,_23];
_14 = (Field::<[u64; 5]>(Variant(_6.fld0.1, 1), 0), _22.1, _22.2, _22.3);
_10 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_6.fld0.1, 1), 1)));
_6.fld0.1 = Adt37::Variant1 { fld0: _6.fld2,fld1: 11103_u16,fld2: _20 };
_24 = [_22.2,_22.2,_22.2,_14.2];
_13.0 = _8.1 & _22.1.0;
_15 = _24;
_24 = _15;
Call(_14.0 = fn17(Move(_10), Field::<[u64; 5]>(Variant(_6.fld0.1, 1), 0), _5.0, _8.3, _13.1, _23, _8.1, _15, _7), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_7 = _23 != _23;
place!(Field::<i32>(Variant(_18, 1), 1)) = _8.1 as i32;
_22.0 = Field::<[u64; 5]>(Variant(_6.fld0.1, 1), 0);
_30 = -_4.0;
_33 = (_9, _6.fld3);
_13.1 = [_7,_23];
match _4.0 {
0 => bb12,
340282366920938463456718344260646955537 => bb14,
_ => bb13
}
}
bb12 = {
RET = [_7,_7,_7,_7];
_6.fld2 = [11611112884161466587_u64,13135246467250591488_u64,2946845946105992179_u64,6666402679148181756_u64,13636103186092226481_u64];
_6.fld0.0 = 2_usize as f64;
_8.2 = !_7;
RET = [_7,_8.2,_8.2,_7];
_8 = (17939_i16, 3585987226712802512_u64, _7, _2);
_6.fld4 = [_8.1,_8.1,_8.1];
_4.0 = (-2764868328585637190_i64) * 7603936106077989071_i64;
_5.0 = -_2;
_2 = _8.3;
_6.fld3 = 94_i8;
_8.0 = 4242_i16 + (-6948_i16);
_6.fld0.1 = Adt37::Variant1 { fld0: _6.fld2,fld1: 20850_u16,fld2: _2 };
place!(Field::<[u64; 5]>(Variant(_6.fld0.1, 1), 0)) = _6.fld2;
_1 = core::ptr::addr_of!(RET);
_8.0 = (-1316_i16) ^ (-13802_i16);
_5.0 = -_8.3;
_3 = (-22410882142535587880764850854634621422_i128) + (-25894511386735398375656524186866061724_i128);
_6.fld1 = [_4.0,_4.0,_4.0,_4.0];
_4 = (8521405691357731944_i64,);
_6.fld0.0 = _3 as f64;
_6.fld0.1 = Adt37::Variant1 { fld0: _6.fld2,fld1: 46936_u16,fld2: _2 };
place!(Field::<isize>(Variant(_6.fld0.1, 1), 2)) = _8.3;
_6.fld4 = [_8.1,_8.1,_8.1];
_8.0 = (-21815_i16) - (-6806_i16);
_6.fld0.0 = 195_u8 as f64;
_4.0 = (-3944265594943197865_i64);
_6.fld0.0 = _8.0 as f64;
RET = [_8.2,_8.2,_8.2,_7];
Call(_2 = core::intrinsics::transmute(Field::<isize>(Variant(_6.fld0.1, 1), 2)), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_5 = (_8.3,);
RET = [_8.2,_8.2,_7,_8.2];
_14.1.1 = [_8.2,_7];
_13.0 = _8.0 as u64;
_10 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_6.fld0.1, 1), 1)));
_1 = core::ptr::addr_of!((*_1));
_8.0 = 6291_i16;
_14.2 = 4_usize;
place!(Field::<[u64; 5]>(Variant(_6.fld0.1, 1), 0)) = [_8.1,_13.0,_8.1,_8.1,_13.0];
_13 = (_8.1, _14.1.1);
_6.fld4 = [_8.1,_13.0,_13.0];
_13.0 = !_8.1;
(*_10) = 14235_u16 & 24834_u16;
_8.2 = _7;
_4.0 = 2077559328506329656_i64;
_17 = _6.fld3 as f32;
_9 = _8.0 + _8.0;
_14.3 = !_3;
place!(Field::<isize>(Variant(_6.fld0.1, 1), 2)) = _5.0;
match _4.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
2077559328506329656 => bb9,
_ => bb8
}
}
bb14 = {
_14.2 = _22.2 * _22.2;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(3_usize, 33_usize, Move(_33), 20_usize, Move(_20), 9_usize, Move(_9), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(3_usize, 4_usize, Move(_4), 8_usize, Move(_8), 7_usize, Move(_7), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(3_usize, 30_usize, Move(_30), 38_usize, _38, 38_usize, _38, 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: [bool; 4],mut _2: *const [bool; 4],mut _3: (i64,),mut _4: [bool; 4],mut _5: [bool; 4],mut _6: [bool; 4]) -> bool {
mir! {
type RET = bool;
let _7: (f64, Adt37);
let _8: isize;
let _9: (i16, u64, bool, isize);
let _10: i32;
let _11: Adt60;
let _12: isize;
let _13: ([bool; 2], u64, &'static i8, (u64, [bool; 2]));
let _14: [u64; 5];
let _15: isize;
let _16: Adt49;
let _17: (*const ([bool; 2], u64, &'static i8, (u64, [bool; 2])), &'static [i16; 7], isize, &'static usize);
let _18: u128;
let _19: isize;
let _20: char;
let _21: u8;
let _22: u16;
let _23: (f64, Adt37);
let _24: (u64, [bool; 2]);
let _25: f64;
let _26: char;
let _27: [usize; 5];
let _28: [bool; 2];
let _29: f64;
let _30: isize;
let _31: f64;
let _32: [u64; 5];
let _33: &'static u16;
let _34: ();
let _35: ();
{
_1 = _6;
_5 = _4;
_2 = core::ptr::addr_of!(_5);
(*_2) = [true,false,true,true];
_2 = core::ptr::addr_of!(_4);
_5 = [true,true,true,false];
(*_2) = [true,true,false,true];
(*_2) = [true,false,false,false];
_4 = _5;
(*_2) = _1;
(*_2) = [true,false,false,true];
_1 = [false,true,true,true];
RET = !true;
_4 = [RET,RET,RET,RET];
_1 = [RET,RET,RET,RET];
_3.0 = 4403254829392416997_i64;
RET = false;
_3 = (6164692187327029083_i64,);
RET = _3.0 != _3.0;
_1 = [RET,RET,RET,RET];
_1 = [RET,RET,RET,RET];
_6 = _1;
_5 = _1;
RET = !false;
_9 = (658_i16, 10123781416141314733_u64, RET, 100_isize);
_7.0 = _9.3 as f64;
_5 = _1;
_9.0 = -(-29894_i16);
match _3.0 {
0 => bb1,
6164692187327029083 => bb3,
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
_6 = [_9.2,_9.2,_9.2,_9.2];
_6 = [_9.2,RET,RET,RET];
_8 = 196_u8 as isize;
_9.3 = _8 & _8;
_9.2 = RET;
Goto(bb4)
}
bb4 = {
(*_2) = [RET,RET,_9.2,_9.2];
_9.3 = _8 & _8;
_7.0 = 1494001334_i32 as f64;
RET = _8 >= _8;
Call(_10 = fn5(Move(_2), _9.3, _9.1, _9.1, _9, _9, _9, _6, (*_2), _9.0, _9), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_2 = core::ptr::addr_of!(_5);
_9.3 = _9.0 as isize;
_6 = (*_2);
_13.1 = !_9.1;
_1 = _6;
_9 = ((-31739_i16), _13.1, RET, _8);
_9.0 = 28795_i16;
RET = _7.0 != _7.0;
_16.fld4 = [_13.1,_13.1,_9.1];
_13.3.1 = [_9.2,RET];
_19 = _9.1 as isize;
_13.0 = _13.3.1;
_13.3.0 = _9.1;
_9.0 = 12082_i16 + 15908_i16;
_13.3 = (_13.1, _13.0);
_10 = 2083733163_u32 as i32;
_2 = core::ptr::addr_of!((*_2));
_3.0 = !(-2831247271443043643_i64);
Goto(bb6)
}
bb6 = {
_3.0 = '\u{9f44b}' as i64;
_18 = 114_i8 as u128;
_3 = (4155058901401245013_i64,);
_9.1 = _13.1 | _13.3.0;
_16.fld3 = _9.0 as i8;
_6 = [RET,_9.2,RET,_9.2];
_18 = 297634098011426789218275783726683912663_u128;
_13.2 = &_16.fld3;
_12 = _19 - _19;
_17.2 = _19;
_8 = _12 | _12;
_19 = 3123515096_u32 as isize;
_18 = !197176016028101995567460737578963138551_u128;
_16.fld0.0 = _7.0 * _7.0;
_9.2 = !RET;
_14 = [_13.3.0,_9.1,_13.1,_9.1,_13.1];
RET = !_9.2;
_16.fld2 = [_9.1,_13.3.0,_9.1,_13.3.0,_9.1];
_20 = '\u{5ec3b}';
(*_2) = [_9.2,RET,_9.2,_9.2];
_16.fld1 = [_3.0,_3.0,_3.0,_3.0];
_9.2 = RET | RET;
_23.0 = _16.fld0.0 + _16.fld0.0;
_4 = [RET,_9.2,RET,_9.2];
(*_2) = _1;
_19 = _8;
Goto(bb7)
}
bb7 = {
_20 = '\u{63258}';
_19 = _8 >> _8;
_4 = [_9.2,_9.2,_9.2,RET];
_26 = _20;
match _3.0 {
0 => bb3,
1 => bb6,
2 => bb8,
3 => bb9,
4155058901401245013 => bb11,
_ => bb10
}
}
bb8 = {
_3.0 = '\u{9f44b}' as i64;
_18 = 114_i8 as u128;
_3 = (4155058901401245013_i64,);
_9.1 = _13.1 | _13.3.0;
_16.fld3 = _9.0 as i8;
_6 = [RET,_9.2,RET,_9.2];
_18 = 297634098011426789218275783726683912663_u128;
_13.2 = &_16.fld3;
_12 = _19 - _19;
_17.2 = _19;
_8 = _12 | _12;
_19 = 3123515096_u32 as isize;
_18 = !197176016028101995567460737578963138551_u128;
_16.fld0.0 = _7.0 * _7.0;
_9.2 = !RET;
_14 = [_13.3.0,_9.1,_13.1,_9.1,_13.1];
RET = !_9.2;
_16.fld2 = [_9.1,_13.3.0,_9.1,_13.3.0,_9.1];
_20 = '\u{5ec3b}';
(*_2) = [_9.2,RET,_9.2,_9.2];
_16.fld1 = [_3.0,_3.0,_3.0,_3.0];
_9.2 = RET | RET;
_23.0 = _16.fld0.0 + _16.fld0.0;
_4 = [RET,_9.2,RET,_9.2];
(*_2) = _1;
_19 = _8;
Goto(bb7)
}
bb9 = {
Return()
}
bb10 = {
_6 = [_9.2,_9.2,_9.2,_9.2];
_6 = [_9.2,RET,RET,RET];
_8 = 196_u8 as isize;
_9.3 = _8 & _8;
_9.2 = RET;
Goto(bb4)
}
bb11 = {
_25 = _16.fld0.0;
_10 = (-524109064_i32);
_16.fld3 = (-17_i8);
_8 = _9.3;
_16.fld4 = [_9.1,_13.3.0,_13.1];
_16.fld4 = [_13.1,_13.1,_13.3.0];
_20 = _26;
_27 = [15733225376008186251_usize,2_usize,3_usize,7387223700264236607_usize,14770945596300421932_usize];
(*_2) = _6;
_28 = _13.0;
_21 = 1_u8;
_13.0 = [_9.2,_9.2];
_16.fld0.1 = Adt37::Variant1 { fld0: _14,fld1: 57493_u16,fld2: _19 };
_6 = [RET,_9.2,_9.2,_9.2];
_16.fld0.0 = _10 as f64;
_17.2 = Field::<isize>(Variant(_16.fld0.1, 1), 2);
_16.fld1 = [_3.0,_3.0,_3.0,_3.0];
_9.3 = _12;
_24.0 = !_13.1;
Call(_19 = fn16(_13.3.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_1 = [_9.2,RET,RET,RET];
_26 = _20;
_28 = [_9.2,_9.2];
_8 = _13.1 as isize;
_13.3.0 = !_24.0;
place!(Field::<[u64; 5]>(Variant(_16.fld0.1, 1), 0)) = _14;
_29 = _21 as f64;
_16.fld0.0 = _25 - _23.0;
_30 = Field::<isize>(Variant(_16.fld0.1, 1), 2) * _9.3;
_13.3.1 = [_9.2,_9.2];
_17.0 = core::ptr::addr_of!(_13);
_25 = _16.fld0.0;
_12 = Field::<isize>(Variant(_16.fld0.1, 1), 2);
_22 = 26936_u16;
match _16.fld3 {
0 => bb7,
1 => bb2,
2 => bb11,
3 => bb4,
4 => bb9,
5 => bb6,
340282366920938463463374607431768211439 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
place!(Field::<isize>(Variant(_16.fld0.1, 1), 2)) = 10239816052497057152_usize as isize;
_3 = ((-1383152619655649608_i64),);
_19 = _21 as isize;
_24 = _13.3;
_8 = _9.0 as isize;
_15 = 7_usize as isize;
_24.1 = _13.0;
_13.3.0 = !_9.1;
_18 = _3.0 as u128;
_3 = (356917908580209035_i64,);
_2 = core::ptr::addr_of!(_4);
_29 = -_25;
(*_2) = [_9.2,RET,_9.2,RET];
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(4_usize, 6_usize, Move(_6), 15_usize, Move(_15), 1_usize, Move(_1), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(4_usize, 24_usize, Move(_24), 8_usize, Move(_8), 26_usize, Move(_26), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(4_usize, 3_usize, Move(_3), 12_usize, Move(_12), 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: *const [bool; 4],mut _2: isize,mut _3: u64,mut _4: u64,mut _5: (i16, u64, bool, isize),mut _6: (i16, u64, bool, isize),mut _7: (i16, u64, bool, isize),mut _8: [bool; 4],mut _9: [bool; 4],mut _10: i16,mut _11: (i16, u64, bool, isize)) -> i32 {
mir! {
type RET = i32;
let _12: [i64; 4];
let _13: Adt60;
let _14: [usize; 5];
let _15: [char; 6];
let _16: (*const [usize; 4], [u32; 6]);
let _17: isize;
let _18: *const f64;
let _19: Adt37;
let _20: *const f64;
let _21: *mut [bool; 2];
let _22: isize;
let _23: isize;
let _24: u8;
let _25: u16;
let _26: isize;
let _27: f64;
let _28: isize;
let _29: f64;
let _30: Adt71;
let _31: u8;
let _32: ();
let _33: ();
{
_6.1 = 1251251462_i32 as u64;
_8 = [_11.2,_7.2,_11.2,_5.2];
_6.0 = 938_u16 as i16;
_11.0 = _10;
_7.0 = -_6.0;
_7.0 = _10;
_11.0 = _10;
Call(_5.2 = fn6(_10, _4, _7, _2, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11.3 = _7.3;
_6.3 = 61_u8 as isize;
_7.1 = !_4;
_7.0 = _5.0;
_6.1 = 1_i8 as u64;
_6.2 = !_5.2;
_11.2 = _5.2;
_1 = core::ptr::addr_of!(_8);
Call(_6.3 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = _11;
_5.3 = _2;
Goto(bb3)
}
bb3 = {
_1 = core::ptr::addr_of!((*_1));
RET = 1775299234_i32 ^ 818751433_i32;
_11.0 = !_5.0;
_6.1 = 183223194290365768197302905640162298254_u128 as u64;
_16.1 = [3221677820_u32,2350829517_u32,3935062155_u32,2288842065_u32,1418701116_u32,1040047940_u32];
(*_1) = [_6.2,_6.2,_6.2,_5.2];
_11.3 = _2 ^ _7.3;
_5.1 = !_7.1;
_10 = _7.0;
_11.3 = _5.3;
_7.3 = _6.3 - _2;
RET = 27258255441243895210266017111987031814_i128 as i32;
_7.3 = _2 << _11.1;
_11 = _5;
_8 = [_6.2,_5.2,_11.2,_5.2];
_11 = (_6.0, _7.1, _6.2, _7.3);
_7.1 = _4 ^ _11.1;
_10 = 216268033304417847197022065299100394202_u128 as i16;
_7.1 = _3 - _5.1;
_11.2 = _6.2 | _5.2;
_14 = [0_usize,7341120365786513527_usize,3_usize,17093564792442117186_usize,0_usize];
RET = (-2001905917_i32) * (-95180630_i32);
_12 = [(-2497123207957804396_i64),(-4073933588666017764_i64),(-7705352406826274007_i64),5189773931763946429_i64];
match _4 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
10123781416141314733 => bb9,
_ => bb8
}
}
bb4 = {
_6 = _11;
_5.3 = _2;
Goto(bb3)
}
bb5 = {
_11.3 = _7.3;
_6.3 = 61_u8 as isize;
_7.1 = !_4;
_7.0 = _5.0;
_6.1 = 1_i8 as u64;
_6.2 = !_5.2;
_11.2 = _5.2;
_1 = core::ptr::addr_of!(_8);
Call(_6.3 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
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
_5.2 = _4 == _5.1;
_12 = [2187814685932024126_i64,5476299000942122803_i64,8114143351309971138_i64,(-797965955968384113_i64)];
_2 = 43_i8 as isize;
Call(_6.1 = core::intrinsics::transmute(_2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
RET = 120_i8 as i32;
_12 = [(-3309216872270883727_i64),8177865116223155245_i64,4016108695814340486_i64,(-925867930091532694_i64)];
(*_1) = _9;
_9 = [_6.2,_11.2,_6.2,_6.2];
_3 = _7.1;
_11.3 = RET as isize;
_15 = ['\u{42c64}','\u{2f0a0}','\u{abcdf}','\u{70c65}','\u{8fbf4}','\u{ad24}'];
_11.1 = _3;
_10 = -_7.0;
_17 = _6.3 + _5.3;
_6.3 = _7.3;
_6.2 = !_11.2;
_15 = ['\u{16c85}','\u{518e0}','\u{10ee98}','\u{faf9d}','\u{8b24d}','\u{107495}'];
_5.1 = _7.1;
(*_1) = [_6.2,_6.2,_11.2,_6.2];
_6.3 = 63640_u16 as isize;
_9 = [_6.2,_6.2,_11.2,_6.2];
_16.1 = [3610291297_u32,1543073985_u32,2886617217_u32,3591904435_u32,2883888331_u32,3555871979_u32];
_15 = ['\u{49465}','\u{21405}','\u{4d5c5}','\u{eda55}','\u{b85f8}','\u{91e0a}'];
_7.3 = _5.3;
_5 = (_11.0, _7.1, _11.2, _2);
_7.2 = !_11.2;
Goto(bb11)
}
bb11 = {
_16.1 = [2550479443_u32,356049281_u32,3474553703_u32,982204267_u32,863574041_u32,2594583384_u32];
_22 = _7.1 as isize;
_3 = _4 << _22;
_11.2 = _7.2;
_6 = _5;
_16.1 = [624848930_u32,3533664640_u32,1408036195_u32,3609078488_u32,1095169903_u32,2592916201_u32];
_6.2 = !_5.2;
_6.0 = _11.0 * _7.0;
(*_1) = [_7.2,_6.2,_6.2,_7.2];
_7.0 = _11.0 >> _3;
RET = 77447502_i32;
_23 = _22 + _22;
RET = 407384993_i32;
_11.0 = !_7.0;
RET = _11.0 as i32;
_11.3 = 236_u8 as isize;
RET = 50287_u16 as i32;
_5.3 = -_23;
_20 = core::ptr::addr_of!(_27);
_25 = 65400_u16 ^ 56099_u16;
_5.0 = _7.0;
_25 = 50625_u16;
_5.3 = _17 | _22;
_5.1 = 156346518896295062137280770282161611265_u128 as u64;
match _4 {
0 => bb10,
1 => bb7,
2 => bb12,
3 => bb13,
4 => bb14,
10123781416141314733 => bb16,
_ => bb15
}
}
bb12 = {
_11.3 = _7.3;
_6.3 = 61_u8 as isize;
_7.1 = !_4;
_7.0 = _5.0;
_6.1 = 1_i8 as u64;
_6.2 = !_5.2;
_11.2 = _5.2;
_1 = core::ptr::addr_of!(_8);
Call(_6.3 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_6 = _11;
_5.3 = _2;
Goto(bb3)
}
bb14 = {
_1 = core::ptr::addr_of!((*_1));
RET = 1775299234_i32 ^ 818751433_i32;
_11.0 = !_5.0;
_6.1 = 183223194290365768197302905640162298254_u128 as u64;
_16.1 = [3221677820_u32,2350829517_u32,3935062155_u32,2288842065_u32,1418701116_u32,1040047940_u32];
(*_1) = [_6.2,_6.2,_6.2,_5.2];
_11.3 = _2 ^ _7.3;
_5.1 = !_7.1;
_10 = _7.0;
_11.3 = _5.3;
_7.3 = _6.3 - _2;
RET = 27258255441243895210266017111987031814_i128 as i32;
_7.3 = _2 << _11.1;
_11 = _5;
_8 = [_6.2,_5.2,_11.2,_5.2];
_11 = (_6.0, _7.1, _6.2, _7.3);
_7.1 = _4 ^ _11.1;
_10 = 216268033304417847197022065299100394202_u128 as i16;
_7.1 = _3 - _5.1;
_11.2 = _6.2 | _5.2;
_14 = [0_usize,7341120365786513527_usize,3_usize,17093564792442117186_usize,0_usize];
RET = (-2001905917_i32) * (-95180630_i32);
_12 = [(-2497123207957804396_i64),(-4073933588666017764_i64),(-7705352406826274007_i64),5189773931763946429_i64];
match _4 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
10123781416141314733 => bb9,
_ => bb8
}
}
bb15 = {
Return()
}
bb16 = {
_8 = _9;
_17 = _5.3 + _22;
_11.2 = _5.2;
_3 = _6.1 * _4;
_6.1 = _7.1 * _11.1;
_6.2 = _5.2 != _5.2;
_14 = [14773412552457693071_usize,3_usize,6_usize,7_usize,4_usize];
_23 = _17;
_18 = core::ptr::addr_of!(_29);
_7 = _11;
Goto(bb17)
}
bb17 = {
Call(_32 = dump_var(5_usize, 11_usize, Move(_11), 9_usize, Move(_9), 12_usize, Move(_12), 23_usize, Move(_23)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_32 = dump_var(5_usize, 22_usize, Move(_22), 15_usize, Move(_15), 14_usize, Move(_14), 2_usize, Move(_2)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: i16,mut _2: u64,mut _3: (i16, u64, bool, isize),mut _4: isize,mut _5: u64) -> bool {
mir! {
type RET = bool;
let _6: bool;
let _7: &'static ((f64, Adt37), i8, *const [usize; 4]);
let _8: f32;
let _9: [i64; 4];
let _10: isize;
let _11: (i16, u64, bool, isize);
let _12: [u64; 5];
let _13: [i32; 3];
let _14: isize;
let _15: Adt71;
let _16: u8;
let _17: isize;
let _18: ();
let _19: ();
{
_5 = 66_u8 as u64;
RET = _3.2;
RET = !_3.2;
_5 = _2;
_5 = !_2;
_4 = _3.3;
Call(_3.3 = core::intrinsics::transmute(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = _3.1;
_1 = _3.0;
_3.2 = RET;
_6 = !_3.2;
_3.1 = !_2;
_4 = _3.3 + _3.3;
_2 = _5 << _4;
_8 = (-1058689219_i32) as f32;
_11 = (_1, _3.1, _3.2, _3.3);
_5 = 78619840477464046449346855295517005476_u128 as u64;
_11.0 = _1;
_5 = !_2;
Call(_3.2 = fn7(_2, _3.3, _3.3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9 = [4106668624587593710_i64,7800523322946442470_i64,7719431884417780511_i64,5895484518309582469_i64];
_1 = _8 as i16;
_11.3 = _4;
Goto(bb3)
}
bb3 = {
_9 = [(-996991126219062501_i64),(-2254014624210597846_i64),(-1354538074183710041_i64),(-5236598338292541352_i64)];
_11.2 = !_3.2;
_11 = (_3.0, _2, _3.2, _4);
_11.0 = 771187596_i32 as i16;
_3.1 = !_5;
_3.1 = _11.1 & _11.1;
_1 = -_11.0;
_11.2 = _3.2 < _3.2;
_9 = [435468108404738866_i64,(-7585823416523948907_i64),(-7339378151245849434_i64),(-8272312355378399460_i64)];
_11.2 = _3.2;
_12 = [_2,_2,_3.1,_3.1,_3.1];
_10 = _4;
_3.2 = _11.2;
_4 = _10 ^ _11.3;
_10 = -_4;
_11 = _3;
_14 = 74460552470428957238333957832778925598_i128 as isize;
_6 = _11.2;
_13 = [(-1724973558_i32),1610831260_i32,1799605224_i32];
_11 = (_1, _5, _6, _10);
RET = _3.2 < _3.2;
_12 = [_3.1,_3.1,_3.1,_5,_11.1];
Goto(bb4)
}
bb4 = {
Call(_18 = dump_var(6_usize, 5_usize, Move(_5), 13_usize, Move(_13), 2_usize, Move(_2), 14_usize, Move(_14)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_18 = dump_var(6_usize, 10_usize, Move(_10), 4_usize, Move(_4), 19_usize, _19, 19_usize, _19), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: u64,mut _2: isize,mut _3: isize) -> bool {
mir! {
type RET = bool;
let _4: isize;
let _5: (i8, [u64; 3]);
let _6: (usize, [isize; 8], [i16; 7], (i128,));
let _7: *const [usize; 4];
let _8: [isize; 8];
let _9: Adt58;
let _10: isize;
let _11: [usize; 5];
let _12: &'static i64;
let _13: &'static [u64; 5];
let _14: isize;
let _15: char;
let _16: isize;
let _17: u32;
let _18: *mut u16;
let _19: &'static [isize; 8];
let _20: f64;
let _21: (isize,);
let _22: &'static [char; 6];
let _23: &'static [u32; 6];
let _24: ();
let _25: ();
{
RET = true;
_1 = !11660140275610195937_u64;
_4 = !_2;
_3 = _4 * _4;
_6.0 = 3739000027211543586_usize ^ 4_usize;
RET = !false;
_5.1 = [_1,_1,_1];
Call(_6.0 = core::intrinsics::bswap(975191405594833931_usize), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = 44547_u16 as u64;
_5.1 = [_1,_1,_1];
_2 = _4 * _3;
Goto(bb2)
}
bb2 = {
RET = true;
_6.3 = (8791605566000292532468778938449905552_i128,);
_9.fld1 = '\u{71e17}';
_5.0 = -(-7_i8);
RET = !false;
_4 = 6618340302234232440_i64 as isize;
_5.0 = (-5_i8) & 112_i8;
_9.fld7 = _1;
_10 = _2;
_11 = [_6.0,_6.0,_6.0,_6.0,_6.0];
match _6.3.0 {
8791605566000292532468778938449905552 => bb4,
_ => bb3
}
}
bb3 = {
_1 = 44547_u16 as u64;
_5.1 = [_1,_1,_1];
_2 = _4 * _3;
Goto(bb2)
}
bb4 = {
_15 = _9.fld1;
_9.fld6 = (_6.3.0,);
RET = _2 > _3;
_7 = core::ptr::addr_of!(_9.fld2);
_15 = _9.fld1;
_2 = _9.fld6.0 as isize;
_9.fld6.0 = _6.3.0 << _10;
_1 = 3613235493965360060_i64 as u64;
_9.fld2 = [_6.0,_6.0,_6.0,_6.0];
RET = !false;
_2 = _3 * _10;
_11 = [_6.0,_6.0,_6.0,_6.0,_6.0];
_4 = -_2;
Goto(bb5)
}
bb5 = {
_6.0 = 1991081660736958196_usize;
_3 = -_10;
_6.3.0 = _6.0 as i128;
_9.fld6.0 = 18224_u16 as i128;
_8 = [_10,_3,_2,_2,_2,_10,_2,_3];
(*_7) = [_6.0,_6.0,_6.0,_6.0];
_9.fld7 = 170_u8 as u64;
_7 = core::ptr::addr_of!((*_7));
_17 = 3150310068_u32 * 779566004_u32;
_15 = _9.fld1;
RET = false & true;
_17 = _9.fld6.0 as u32;
_9.fld4 = _4 as f32;
_3 = _4 & _4;
_9.fld0 = RET;
Call(RET = fn8(_4, _2, _4, _8, _9.fld4, _3, _3), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_7 = core::ptr::addr_of!((*_7));
_6.1 = _8;
_9.fld3 = _5.0 >> _3;
_16 = -_4;
_9.fld1 = _15;
(*_7) = [_6.0,_6.0,_6.0,_6.0];
_9.fld5 = !35805_u16;
_5.0 = _9.fld3 & _9.fld3;
_8 = [_16,_3,_3,_3,_10,_16,_3,_3];
_9.fld1 = _15;
_15 = _9.fld1;
_6.3 = (_9.fld6.0,);
_9.fld6.0 = _6.3.0 & _6.3.0;
_18 = core::ptr::addr_of_mut!(_9.fld5);
_6.2 = [(-26606_i16),8367_i16,8049_i16,25651_i16,3208_i16,(-17286_i16),15631_i16];
_17 = !430494493_u32;
_18 = core::ptr::addr_of_mut!((*_18));
_21 = (_2,);
Goto(bb7)
}
bb7 = {
Call(_24 = dump_var(7_usize, 10_usize, Move(_10), 11_usize, Move(_11), 3_usize, Move(_3), 5_usize, Move(_5)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_24 = dump_var(7_usize, 4_usize, Move(_4), 21_usize, Move(_21), 25_usize, _25, 25_usize, _25), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: [isize; 8],mut _5: f32,mut _6: isize,mut _7: isize) -> bool {
mir! {
type RET = bool;
let _8: [i32; 3];
let _9: f32;
let _10: *mut ([bool; 2], u64, &'static i8, (u64, [bool; 2]));
let _11: Adt68;
let _12: *mut ([bool; 2], u64, &'static i8, (u64, [bool; 2]));
let _13: isize;
let _14: u16;
let _15: Adt60;
let _16: [char; 6];
let _17: &'static i64;
let _18: ();
let _19: ();
{
_1 = _7;
_2 = _6 * _1;
_5 = (-9956_i16) as f32;
_7 = _2;
RET = true ^ true;
RET = true & true;
_7 = !_6;
_5 = 30_u8 as f32;
_4 = [_1,_2,_1,_7,_6,_3,_6,_2];
_1 = 22172_i16 as isize;
RET = true & true;
_6 = 14731200875755069435_usize as isize;
RET = !false;
_8 = [(-1696802545_i32),1366520067_i32,409830366_i32];
RET = !false;
_7 = 8316_i16 as isize;
_3 = _2;
RET = true | true;
_6 = _2 << _3;
_9 = _5 * _5;
Goto(bb1)
}
bb1 = {
_1 = 1423198125_i32 as isize;
_6 = !_3;
_1 = -_6;
_8 = [(-399849624_i32),(-915122404_i32),(-2081945260_i32)];
_6 = 317202791691488145764132534777080405831_u128 as isize;
_2 = _3 | _1;
_7 = _3 ^ _2;
_8 = [(-684090805_i32),287573665_i32,936366773_i32];
Call(_12 = fn9(_7, _1, _1, _2, _4, _7, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = [_7,_3,_7,_3,_3,_7,_7,_3];
_9 = _5;
_10 = Move(_12);
_13 = _1 << _1;
_4 = [_7,_1,_2,_13,_7,_13,_7,_2];
_9 = _5 + _5;
_12 = Move(_10);
_8 = [606827058_i32,558173314_i32,(-1413347785_i32)];
_10 = Move(_12);
_12 = Move(_10);
_8 = [2067624799_i32,161351763_i32,(-822437349_i32)];
_7 = _2;
_7 = _13;
_14 = 163_u8 as u16;
_3 = _7 ^ _13;
_13 = _1 << _3;
_10 = Move(_12);
_12 = Move(_10);
RET = !false;
_5 = _9 - _9;
RET = _3 >= _13;
_9 = 2271048346_u32 as f32;
_2 = 3439123844_u32 as isize;
_1 = 7699382473686915068_i64 as isize;
_7 = _3;
Goto(bb3)
}
bb3 = {
Call(_18 = dump_var(8_usize, 8_usize, Move(_8), 2_usize, Move(_2), 1_usize, Move(_1), 14_usize, Move(_14)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: [isize; 8],mut _6: isize,mut _7: isize) -> *mut ([bool; 2], u64, &'static i8, (u64, [bool; 2])) {
mir! {
type RET = *mut ([bool; 2], u64, &'static i8, (u64, [bool; 2]));
let _8: u128;
let _9: *const [usize; 4];
let _10: u64;
let _11: Adt68;
let _12: u8;
let _13: Adt71;
let _14: f64;
let _15: f64;
let _16: usize;
let _17: (*const [usize; 4], [u32; 6]);
let _18: usize;
let _19: f32;
let _20: Adt71;
let _21: &'static (i128,);
let _22: i32;
let _23: *mut [bool; 2];
let _24: *const [bool; 4];
let _25: (i16, i8);
let _26: Adt37;
let _27: (isize, bool, ([bool; 2], u64, &'static i8, (u64, [bool; 2])), i128);
let _28: &'static [char; 6];
let _29: isize;
let _30: (i8, [u64; 3]);
let _31: &'static i64;
let _32: i64;
let _33: bool;
let _34: i16;
let _35: i128;
let _36: &'static [char; 6];
let _37: ();
let _38: ();
{
_7 = 3010433621_u32 as isize;
_7 = _2;
_8 = 264304561999327907231491979648565515389_u128 | 110189871376301156786381659219584121445_u128;
_8 = 26599277365511433991633114322906343696_u128 + 76349740393973904340366980043444441040_u128;
_1 = !_7;
_6 = !_2;
_2 = _4;
_1 = -_3;
_8 = 2148187699_u32 as u128;
_7 = _1 >> _2;
_6 = _2;
_1 = -_6;
_10 = 2532279223675763560_u64 + 17519526343495896494_u64;
_4 = _2 | _1;
_7 = _6 & _2;
_6 = 172_u8 as isize;
Goto(bb1)
}
bb1 = {
_1 = '\u{72622}' as isize;
_8 = 39747567866479327675745092110383052704_u128 & 121358778721095512342609578067799537257_u128;
_1 = _3;
_8 = 281594599964447335346672109197342345411_u128 >> _2;
Goto(bb2)
}
bb2 = {
_7 = 53_u8 as isize;
_1 = _8 as isize;
_5 = [_3,_4,_1,_3,_3,_3,_2,_3];
_3 = _4 * _4;
_7 = 58662_u16 as isize;
_2 = -_3;
_6 = !_1;
_2 = !_1;
_5 = [_2,_1,_3,_1,_6,_4,_4,_1];
_4 = _2;
Goto(bb3)
}
bb3 = {
_12 = 33313_u16 as u8;
_8 = !261347504594538645085380026504951784740_u128;
_1 = '\u{8452f}' as isize;
_7 = _12 as isize;
_8 = _10 as u128;
_1 = _3;
_10 = !1188344886537625940_u64;
_10 = 14132050132396614764_u64;
_8 = 190862103623349722110060059922648135208_u128;
_8 = '\u{7a0ad}' as u128;
_6 = _1;
_6 = _4;
_6 = _1;
_6 = _8 as isize;
_14 = 62_i8 as f64;
_5 = [_2,_1,_3,_3,_4,_3,_1,_4];
match _10 {
14132050132396614764 => bb5,
_ => bb4
}
}
bb4 = {
_1 = '\u{72622}' as isize;
_8 = 39747567866479327675745092110383052704_u128 & 121358778721095512342609578067799537257_u128;
_1 = _3;
_8 = 281594599964447335346672109197342345411_u128 >> _2;
Goto(bb2)
}
bb5 = {
_3 = !_2;
_5 = [_2,_1,_1,_1,_1,_4,_1,_4];
_6 = _3 + _2;
_4 = _3 >> _10;
_5 = [_6,_3,_1,_4,_2,_1,_6,_2];
_4 = _6;
_4 = _6;
_1 = !_3;
_12 = 196_u8;
_6 = !_2;
_16 = _10 as usize;
_10 = 7828050694760561949_u64 ^ 8263775966304257504_u64;
Goto(bb6)
}
bb6 = {
_3 = _2 | _1;
_12 = 124_u8;
_15 = 89749973971399981779386758338589482741_i128 as f64;
_6 = !_2;
_3 = _6;
Call(_16 = core::intrinsics::bswap(6_usize), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_3 = _6 | _2;
_12 = 151_u8 - 102_u8;
_16 = !3_usize;
_1 = _3;
_6 = (-115_i8) as isize;
_15 = _16 as f64;
_8 = 165582999498272789926719947624152808976_u128 | 213376824004054963826235904023820580832_u128;
_6 = !_2;
_10 = !633999188458110054_u64;
_19 = _10 as f32;
_17.1 = [1101149673_u32,2826504862_u32,2136265868_u32,1356712556_u32,2564531546_u32,2525173515_u32];
_2 = -_4;
_18 = !_16;
_6 = !_2;
_15 = _14;
_14 = _15;
_8 = _4 as u128;
_7 = true as isize;
_7 = '\u{35d6e}' as isize;
_14 = _15;
_7 = _1 >> _4;
Goto(bb8)
}
bb8 = {
_10 = _18 as u64;
_18 = _16 ^ _16;
_4 = _2 << _3;
_3 = 30026_u16 as isize;
_5 = [_7,_2,_4,_2,_2,_1,_7,_6];
_16 = '\u{92887}' as usize;
_14 = 4689_i16 as f64;
_19 = (-72524221667343456530708461346513568785_i128) as f32;
_12 = !44_u8;
_2 = 2707704973_u32 as isize;
_18 = _16;
_6 = _4;
_4 = _14 as isize;
_1 = _7 << _6;
_22 = false as i32;
_15 = 12540406884540787398839318311532140800_i128 as f64;
_27.0 = _6 ^ _1;
_6 = '\u{2bbd1}' as isize;
_12 = '\u{d7a35}' as u8;
_27.2.3.0 = _10 & _10;
Call(_17.1 = fn10(_5, _7, _5, _5, _5, _5, _1, _27.0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_10 = _27.2.3.0 * _27.2.3.0;
_8 = 101570857523309133166695996138429774953_u128 - 288504006172936966176788582788262718818_u128;
_25 = ((-25939_i16), 123_i8);
match _25.1 {
0 => bb10,
123 => bb12,
_ => bb11
}
}
bb10 = {
_10 = _18 as u64;
_18 = _16 ^ _16;
_4 = _2 << _3;
_3 = 30026_u16 as isize;
_5 = [_7,_2,_4,_2,_2,_1,_7,_6];
_16 = '\u{92887}' as usize;
_14 = 4689_i16 as f64;
_19 = (-72524221667343456530708461346513568785_i128) as f32;
_12 = !44_u8;
_2 = 2707704973_u32 as isize;
_18 = _16;
_6 = _4;
_4 = _14 as isize;
_1 = _7 << _6;
_22 = false as i32;
_15 = 12540406884540787398839318311532140800_i128 as f64;
_27.0 = _6 ^ _1;
_6 = '\u{2bbd1}' as isize;
_12 = '\u{d7a35}' as u8;
_27.2.3.0 = _10 & _10;
Call(_17.1 = fn10(_5, _7, _5, _5, _5, _5, _1, _27.0), ReturnTo(bb9), UnwindUnreachable())
}
bb11 = {
_1 = '\u{72622}' as isize;
_8 = 39747567866479327675745092110383052704_u128 & 121358778721095512342609578067799537257_u128;
_1 = _3;
_8 = 281594599964447335346672109197342345411_u128 >> _2;
Goto(bb2)
}
bb12 = {
_6 = _27.0;
_27.2.2 = &_25.1;
_29 = !_6;
_2 = _8 as isize;
RET = core::ptr::addr_of_mut!(_27.2);
(*RET).1 = (*RET).3.0 | _27.2.3.0;
(*RET).3.1 = [true,true];
(*RET).3.1 = [false,true];
(*RET).1 = !_27.2.3.0;
_6 = _25.1 as isize;
_23 = core::ptr::addr_of_mut!((*RET).0);
(*RET).1 = !_27.2.3.0;
_25 = (9781_i16, 47_i8);
_27.2.2 = &_25.1;
(*RET).0 = _27.2.3.1;
_16 = 1235799097_u32 as usize;
_30.1 = [(*RET).3.0,(*RET).3.0,_27.2.3.0];
_22 = 1525764875_i32 + (-1719080080_i32);
_14 = _22 as f64;
_14 = _15;
(*RET).3.1 = [false,false];
(*_23) = [true,true];
(*RET).3.0 = (*RET).1 << _25.0;
(*RET).0 = [true,false];
_23 = core::ptr::addr_of_mut!((*RET).0);
match _25.1 {
47 => bb14,
_ => bb13
}
}
bb13 = {
_1 = '\u{72622}' as isize;
_8 = 39747567866479327675745092110383052704_u128 & 121358778721095512342609578067799537257_u128;
_1 = _3;
_8 = 281594599964447335346672109197342345411_u128 >> _2;
Goto(bb2)
}
bb14 = {
_1 = _27.0;
RET = core::ptr::addr_of_mut!((*RET));
_27.2.1 = (*RET).3.0 ^ (*RET).3.0;
_32 = 682474238143832327_i64 >> _7;
(*RET).0 = (*RET).3.1;
(*RET).3.1 = [false,true];
(*RET).3 = (_10, (*_23));
_27.2.2 = &_25.1;
_6 = _25.0 as isize;
_30.0 = -_25.1;
_33 = _29 < _1;
_31 = &_32;
_34 = _22 as i16;
_35 = 46855154134363058305497585605175155521_i128 & (-134548964027258170072149080097654340276_i128);
(*RET).3 = ((*RET).1, (*RET).0);
(*RET).3.0 = (*RET).1;
(*RET).2 = &_30.0;
_6 = 18699_u16 as isize;
_25.0 = -_34;
_27.0 = _1;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(9_usize, 8_usize, Move(_8), 16_usize, Move(_16), 7_usize, Move(_7), 32_usize, Move(_32)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(9_usize, 34_usize, Move(_34), 22_usize, Move(_22), 18_usize, Move(_18), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(9_usize, 4_usize, Move(_4), 30_usize, Move(_30), 38_usize, _38, 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: [isize; 8],mut _2: isize,mut _3: [isize; 8],mut _4: [isize; 8],mut _5: [isize; 8],mut _6: [isize; 8],mut _7: isize,mut _8: isize) -> [u32; 6] {
mir! {
type RET = [u32; 6];
let _9: u32;
let _10: isize;
let _11: (isize, bool, ([bool; 2], u64, &'static i8, (u64, [bool; 2])), i128);
let _12: Adt71;
let _13: &'static &'static ((f64, Adt37), i8, *const [usize; 4]);
let _14: *mut u16;
let _15: isize;
let _16: usize;
let _17: Adt60;
let _18: f32;
let _19: &'static i64;
let _20: u16;
let _21: [u128; 5];
let _22: isize;
let _23: (isize,);
let _24: &'static [u32; 6];
let _25: *const f64;
let _26: (f64, Adt37);
let _27: *const f64;
let _28: u8;
let _29: [usize; 5];
let _30: i128;
let _31: u16;
let _32: *const [usize; 4];
let _33: bool;
let _34: u32;
let _35: isize;
let _36: u8;
let _37: [isize; 8];
let _38: ();
let _39: ();
{
_8 = _2;
RET = [659666135_u32,67827640_u32,2905950165_u32,2717067137_u32,1106958503_u32,1443940807_u32];
_7 = _2;
_7 = _2;
_7 = _8 & _8;
_5 = [_7,_7,_7,_7,_7,_8,_8,_8];
_4 = [_7,_8,_8,_2,_7,_2,_2,_8];
_9 = 3011193095_u32 + 221227695_u32;
_5 = _1;
_1 = [_7,_2,_8,_2,_8,_8,_7,_8];
_6 = [_2,_2,_7,_7,_2,_8,_8,_7];
_3 = [_8,_8,_8,_8,_2,_8,_8,_2];
_9 = !1680762080_u32;
_1 = [_8,_7,_2,_7,_2,_2,_7,_8];
_5 = [_2,_2,_8,_7,_8,_8,_2,_7];
_9 = 1059793681_u32;
_1 = _5;
_6 = [_7,_7,_8,_8,_2,_7,_2,_2];
_10 = false as isize;
_2 = !_8;
_11.1 = false;
_11.2.3.1 = [_11.1,_11.1];
Goto(bb1)
}
bb1 = {
RET = [_9,_9,_9,_9,_9,_9];
_11.3 = 125225743228726610783037920324465400740_i128;
match _9 {
0 => bb2,
1059793681 => bb4,
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
_11.2.1 = 14924944239324131556_usize as u64;
_10 = -_8;
_11.3 = 143944450310008030233154364397191532473_i128;
RET = [_9,_9,_9,_9,_9,_9];
_4 = [_2,_10,_7,_7,_2,_7,_2,_2];
_11.0 = (-14_i8) as isize;
_4 = [_10,_2,_2,_2,_2,_2,_7,_7];
RET = [_9,_9,_9,_9,_9,_9];
_8 = !_10;
_1 = _6;
_9 = !4006136221_u32;
_6 = [_2,_10,_2,_10,_10,_2,_8,_2];
_11.1 = _10 > _7;
_4 = [_8,_2,_7,_2,_10,_8,_8,_10];
_11.3 = (-1583212530818104817_i64) as i128;
_11.2.3.0 = _11.2.1;
_2 = 29616_i16 as isize;
_1 = [_7,_7,_7,_7,_8,_7,_7,_10];
RET = [_9,_9,_9,_9,_9,_9];
Goto(bb5)
}
bb5 = {
_8 = -_7;
_6 = [_10,_10,_8,_10,_10,_8,_10,_8];
_7 = _8 - _10;
RET = [_9,_9,_9,_9,_9,_9];
_3 = [_10,_8,_8,_8,_7,_7,_10,_8];
_11.2.3.1 = [_11.1,_11.1];
_11.2.3.0 = _11.3 as u64;
_7 = _11.1 as isize;
_6 = _3;
_11.1 = _10 <= _10;
_4 = _5;
_11.2.1 = _11.2.3.0;
_11.1 = _8 <= _10;
_1 = [_8,_10,_10,_7,_8,_8,_7,_8];
Goto(bb6)
}
bb6 = {
_15 = _8;
_10 = -_15;
_8 = _15 ^ _10;
_7 = _8;
_15 = !_7;
_6 = _3;
_7 = !_8;
_16 = _11.1 as usize;
_11.0 = _8 - _8;
_6 = [_11.0,_15,_11.0,_7,_11.0,_15,_10,_8];
_11.3 = '\u{76a95}' as i128;
Goto(bb7)
}
bb7 = {
_1 = _3;
_11.2.0 = [_11.1,_11.1];
_11.0 = '\u{4c0f2}' as isize;
_11.2.3.0 = _11.2.1;
_1 = _4;
_11.2.1 = !_11.2.3.0;
_7 = _15 - _10;
_9 = 2107375656_u32 + 1896579305_u32;
_11.3 = -114475398219887913983639579174225401883_i128;
RET = [_9,_9,_9,_9,_9,_9];
Call(_9 = fn11(), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_16 = 5_usize << _7;
_18 = _15 as f32;
RET = [_9,_9,_9,_9,_9,_9];
RET = [_9,_9,_9,_9,_9,_9];
_11.2.0 = [_11.1,_11.1];
_15 = _10 >> _7;
_9 = _11.3 as u32;
_15 = !_8;
_20 = 3096_u16;
_18 = _11.2.1 as f32;
_11.2.3.1 = [_11.1,_11.1];
_10 = _8 ^ _8;
_11.1 = false;
_21 = [12509139633617128434244090007644546822_u128,52946458982202362877177621796629006352_u128,25034632723855293238668428651632995220_u128,273458551509087425918493585134223389169_u128,67002676178338460450608461307604910782_u128];
_5 = _1;
_20 = 30507_u16;
match _20 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb7,
5 => bb9,
6 => bb10,
30507 => bb12,
_ => bb11
}
}
bb9 = {
_1 = _3;
_11.2.0 = [_11.1,_11.1];
_11.0 = '\u{4c0f2}' as isize;
_11.2.3.0 = _11.2.1;
_1 = _4;
_11.2.1 = !_11.2.3.0;
_7 = _15 - _10;
_9 = 2107375656_u32 + 1896579305_u32;
_11.3 = -114475398219887913983639579174225401883_i128;
RET = [_9,_9,_9,_9,_9,_9];
Call(_9 = fn11(), ReturnTo(bb8), UnwindUnreachable())
}
bb10 = {
RET = [_9,_9,_9,_9,_9,_9];
_11.3 = 125225743228726610783037920324465400740_i128;
match _9 {
0 => bb2,
1059793681 => bb4,
_ => bb3
}
}
bb11 = {
_8 = -_7;
_6 = [_10,_10,_8,_10,_10,_8,_10,_8];
_7 = _8 - _10;
RET = [_9,_9,_9,_9,_9,_9];
_3 = [_10,_8,_8,_8,_7,_7,_10,_8];
_11.2.3.1 = [_11.1,_11.1];
_11.2.3.0 = _11.3 as u64;
_7 = _11.1 as isize;
_6 = _3;
_11.1 = _10 <= _10;
_4 = _5;
_11.2.1 = _11.2.3.0;
_11.1 = _8 <= _10;
_1 = [_8,_10,_10,_7,_8,_8,_7,_8];
Goto(bb6)
}
bb12 = {
_27 = core::ptr::addr_of!(_26.0);
_23 = (_10,);
RET = [_9,_9,_9,_9,_9,_9];
_10 = 1246851374_i32 as isize;
_8 = _23.0;
_29 = [_16,_16,_16,_16,_16];
_16 = !3_usize;
_11.2.3.0 = _11.2.1;
_23 = (_7,);
Call((*_27) = core::intrinsics::transmute(_23.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_10 = _8 | _23.0;
_15 = 58379499103102749657104816988643344196_u128 as isize;
_18 = _20 as f32;
_33 = _11.1;
_4 = [_7,_8,_8,_23.0,_7,_7,_10,_23.0];
_26.0 = _16 as f64;
_5 = _1;
_21 = [317523188278497161818483181561641887793_u128,248442478677833488231108633025404914065_u128,76313886809083647888059962643694310363_u128,212271013416076469411490120139781672801_u128,227050178536481782833565157360865092914_u128];
_11.2.3.0 = _11.2.1;
_9 = !1217271175_u32;
_11.2.1 = !_11.2.3.0;
_11.1 = _8 > _8;
RET = [_9,_9,_9,_9,_9,_9];
_18 = 155929124574690249635341238846728962990_u128 as f32;
_11.2.1 = !_11.2.3.0;
RET = [_9,_9,_9,_9,_9,_9];
_11.2.3 = (_11.2.1, _11.2.0);
_29 = [_16,_16,_16,_16,_16];
_33 = _11.1;
_5 = [_10,_8,_8,_7,_7,_8,_23.0,_7];
_23 = (_10,);
_3 = [_23.0,_23.0,_23.0,_7,_7,_7,_7,_23.0];
_21 = [189384810997028328106095189122589406507_u128,192341700763821150690988896020075344756_u128,14593610820546550915366144765787633863_u128,336312923637405595719198503091998021713_u128,333874758198685726576742620603657185950_u128];
_24 = &RET;
_20 = 54256_u16;
_23 = (_8,);
Goto(bb14)
}
bb14 = {
_26.0 = _11.2.1 as f64;
RET = [_9,_9,_9,_9,_9,_9];
_15 = -_8;
_30 = !_11.3;
_3 = [_23.0,_7,_10,_7,_8,_10,_10,_23.0];
_5 = [_15,_15,_7,_8,_8,_23.0,_8,_15];
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(10_usize, 4_usize, Move(_4), 7_usize, Move(_7), 23_usize, Move(_23), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(10_usize, 30_usize, Move(_30), 5_usize, Move(_5), 29_usize, Move(_29), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(10_usize, 3_usize, Move(_3), 39_usize, _39, 39_usize, _39, 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11() -> u32 {
mir! {
type RET = u32;
let _1: f64;
let _2: (i128,);
let _3: f32;
let _4: (f64, Adt37);
let _5: &'static [char; 6];
let _6: [u32; 6];
let _7: &'static &'static i8;
let _8: isize;
let _9: f64;
let _10: &'static [usize; 5];
let _11: Adt37;
let _12: &'static u64;
let _13: ([u64; 5], (u64, [bool; 2]), usize, i128);
let _14: isize;
let _15: ();
let _16: ();
{
RET = 3102335811_u32 & 3155334438_u32;
Goto(bb1)
}
bb1 = {
RET = false as u32;
_1 = 50929_u16 as f64;
RET = !3896851523_u32;
_1 = 65837048473896559737099171266382936477_u128 as f64;
_2.0 = 89249469695941952134321279548435476226_u128 as i128;
RET = true as u32;
_2 = (131652625594039876928267223686207103708_i128,);
RET = 1694216660_u32 ^ 1345216021_u32;
_1 = (-148386871_i32) as f64;
_2.0 = (-88363485649885865230847984704045733282_i128) | (-22529031633461560347156574833405570186_i128);
_3 = 3_usize as f32;
_3 = 7095656576262754773_i64 as f32;
_3 = _1 as f32;
_2 = (34687032941707429341585540411404027355_i128,);
RET = 3530927041_u32;
_2 = ((-27982491973336427128448925534987683873_i128),);
match _2.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
312299874947602036334925681896780527583 => bb10,
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
_4.0 = 554314519_i32 as f64;
_4.0 = -_1;
RET = 6165_i16 as u32;
RET = !1277731028_u32;
_2 = ((-62192007035858488858772652250061445049_i128),);
_3 = 124_i8 as f32;
RET = 4249419908_u32;
_6 = [RET,RET,RET,RET,RET,RET];
_4.0 = _1;
_3 = (-1914_i16) as f32;
_2.0 = -(-117020336753052035576742082774971080858_i128);
_2 = (59968176131928337389725877700868445814_i128,);
_1 = -_4.0;
_6 = [RET,RET,RET,RET,RET,RET];
RET = 2525673562_u32;
_6 = [RET,RET,RET,RET,RET,RET];
_1 = _4.0;
_2 = (127858669358062086418587145508356363837_i128,);
_3 = RET as f32;
_8 = (-9223372036854775808_isize);
RET = !1750381716_u32;
RET = 302410406_u32;
_4.0 = -_1;
_9 = _2.0 as f64;
_4.0 = _9 - _1;
RET = 1397685484_u32 + 3841446641_u32;
Call(_1 = fn12(_8, _4.0, _9, _8, _2.0, _3, _8, _2.0, _8), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_2.0 = 66602196443836394743212716974949603832_i128 >> RET;
_4.0 = -_9;
_6 = [RET,RET,RET,RET,RET,RET];
_1 = -_4.0;
_2 = ((-6668935362801366772225205343588839241_i128),);
_9 = _1 * _4.0;
_1 = 120028833828603456557850522644001445883_u128 as f64;
_1 = 31405079954185800700639378623081662626_u128 as f64;
match _8 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
340282366920938463454151235394913435648 => bb17,
_ => bb16
}
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
Return()
}
bb16 = {
Return()
}
bb17 = {
_8 = (-33_isize) * (-67_isize);
_1 = -_4.0;
_6 = [RET,RET,RET,RET,RET,RET];
RET = 759825911307645444_i64 as u32;
_8 = 717750404376363555_u64 as isize;
_4.0 = 36_i8 as f64;
RET = 4138066708_u32 - 327188129_u32;
RET = 2496827640_u32 + 2733363467_u32;
RET = 923302996_u32;
_8 = !(-56_isize);
_4.0 = 9319677345651881994_u64 as f64;
_13.3 = _3 as i128;
_13.1.0 = 8229070611419630766_u64 ^ 10216547249092880470_u64;
_8 = 8389665178068509564_usize as isize;
_1 = 187150344991805461657777017039418468629_u128 as f64;
_9 = 3796172665494768251_i64 as f64;
_9 = _1;
_2.0 = !_13.3;
_3 = 29533811545654808028027575760516016794_u128 as f32;
_9 = 236921739997080398907398102930275763481_u128 as f64;
_13.1.1 = [true,false];
_6 = [RET,RET,RET,RET,RET,RET];
Goto(bb18)
}
bb18 = {
Call(_15 = dump_var(11_usize, 2_usize, Move(_2), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: isize,mut _2: f64,mut _3: f64,mut _4: isize,mut _5: i128,mut _6: f32,mut _7: isize,mut _8: i128,mut _9: isize) -> f64 {
mir! {
type RET = f64;
let _10: f32;
let _11: (&'static [isize; 8], ([bool; 2], u64, &'static i8, (u64, [bool; 2])), (i128,));
let _12: u32;
let _13: *const *mut [bool; 2];
let _14: &'static usize;
let _15: u8;
let _16: i8;
let _17: (isize, bool, ([bool; 2], u64, &'static i8, (u64, [bool; 2])), i128);
let _18: &'static [i16; 7];
let _19: *const [usize; 4];
let _20: usize;
let _21: ([bool; 2], u64, &'static i8, (u64, [bool; 2]));
let _22: [i32; 3];
let _23: (i64,);
let _24: [usize; 4];
let _25: i32;
let _26: Adt57;
let _27: i8;
let _28: i32;
let _29: i128;
let _30: &'static &'static ((f64, Adt37), i8, *const [usize; 4]);
let _31: [bool; 4];
let _32: bool;
let _33: *mut ([bool; 2], u64, &'static i8, (u64, [bool; 2]));
let _34: Adt62;
let _35: f32;
let _36: ();
let _37: ();
{
RET = _2;
_7 = _9 << _4;
_4 = _1;
_5 = _8;
_1 = _6 as isize;
_6 = (-713122522_i32) as f32;
_4 = _7;
_6 = _1 as f32;
_11.2.0 = -_5;
_3 = (-326117496_i32) as f64;
_7 = !_1;
_11.1.3.0 = !13803423326039319660_u64;
RET = -_2;
_11.2 = (_5,);
_10 = _6;
_4 = _9 >> _8;
_5 = !_11.2.0;
_11.2.0 = _8 >> _4;
_1 = 249126326683524981966045288814309192590_u128 as isize;
_10 = _6;
_3 = -_2;
_6 = _10 - _10;
_7 = _4;
_1 = _7 ^ _4;
_1 = _4;
Call(_10 = fn13(_3, RET, _5, _11.2, _9, _11.2.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = _7 - _7;
_11.2.0 = _5;
_11.2.0 = _8;
_11.1.3.1 = [true,true];
_11.1.3.1 = [true,false];
_11.1.3.0 = !10326550102618858351_u64;
_11.1.0 = [false,false];
_3 = RET;
_4 = 145885854277721073225567994836562711321_u128 as isize;
_11.1.3.0 = 16115431287720115046_u64 | 16852285556981965543_u64;
RET = 109_i8 as f64;
RET = -_3;
_12 = !2473518989_u32;
_11.1.0 = _11.1.3.1;
_1 = _9 | _4;
_11.1.3.1 = [true,false];
_2 = RET;
_12 = 738063625_u32;
_11.2 = (_8,);
_6 = _10 * _10;
_4 = _1 >> _1;
_8 = 214333450991515119022004454665732477827_u128 as i128;
Goto(bb2)
}
bb2 = {
_11.1.3 = (282063911461089046_u64, _11.1.0);
_7 = _9 * _4;
_6 = -_10;
RET = _3;
_11.1.3 = (3298910529703042119_u64, _11.1.0);
_7 = '\u{6b590}' as isize;
_11.1.0 = [true,false];
_11.1.3.0 = 10348353427806808093_u64;
Goto(bb3)
}
bb3 = {
_11.1.0 = [true,false];
_12 = _1 as u32;
_6 = _10;
_5 = _8;
_4 = 25525_u16 as isize;
_11.1.1 = _11.1.3.0 - _11.1.3.0;
_1 = _9 << _9;
_12 = !793116691_u32;
_12 = !2746287283_u32;
_11.2 = (_8,);
_4 = -_1;
_11.1.3.0 = 4_usize as u64;
_11.1.3.0 = RET as u64;
_15 = 77_i8 as u8;
_2 = RET - _3;
_12 = !770861268_u32;
_6 = _10;
Call(_10 = fn14(_5, _1, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_11.2 = (_8,);
_12 = !2308143838_u32;
_11.1.3.0 = false as u64;
_3 = RET;
_5 = true as i128;
_4 = -_1;
_12 = 2943724257_u32 - 3604499823_u32;
_17.2.0 = [false,false];
_10 = (-8346947610897482941_i64) as f32;
_11.1.1 = !_11.1.3.0;
_17.2.3.0 = _11.2.0 as u64;
_11.1.3.1 = [true,false];
_11.1.3 = (_17.2.3.0, _11.1.0);
_16 = 20050_i16 as i8;
_11.2 = (_5,);
_11.1.2 = &_16;
_6 = _2 as f32;
_4 = _15 as isize;
_17.2.3 = (_11.1.1, _17.2.0);
_7 = _9 | _1;
Goto(bb5)
}
bb5 = {
_17.0 = _1 ^ _9;
_17.2.2 = Move(_11.1.2);
_17.2.1 = !_17.2.3.0;
_11.1.2 = &_16;
_20 = !3_usize;
_24 = [_20,_20,_20,_20];
_11.1.3.1 = [false,true];
_20 = !1972931126066909379_usize;
_23.0 = !1223717623326112771_i64;
Goto(bb6)
}
bb6 = {
_21.3.0 = _11.1.1;
_21.0 = [true,false];
_11.2 = (_5,);
_17.3 = !_11.2.0;
_21.3.1 = [true,true];
RET = _3;
_21.3.0 = _17.2.3.0;
Call(_25 = fn15(Move(_11.1), _24, _17.0, _9, _7, _17.0, _7, _21.0, _11.2, _7, _6), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_8 = _5 >> _7;
_12 = 884558842_u32;
_12 = '\u{1bfcf}' as u32;
_12 = 2904460529_u32 - 3455815420_u32;
_19 = core::ptr::addr_of!(_24);
_11.1.1 = !_17.2.1;
(*_19) = [_20,_20,_20,_20];
_21.3.0 = _11.1.1 ^ _17.2.1;
_22 = [_25,_25,_25];
_21.1 = _17.2.1;
_21.0 = _17.2.0;
_31 = [false,true,false,true];
_11.1.3.1 = [false,true];
_7 = _6 as isize;
_17.2.3.1 = _11.1.3.1;
_2 = _3;
_1 = _17.0 | _17.0;
_11.1.0 = _17.2.3.1;
_23.0 = 5013295358483677153_i64;
_1 = '\u{5f86f}' as isize;
match _23.0 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5013295358483677153 => bb14,
_ => bb13
}
}
bb8 = {
_21.3.0 = _11.1.1;
_21.0 = [true,false];
_11.2 = (_5,);
_17.3 = !_11.2.0;
_21.3.1 = [true,true];
RET = _3;
_21.3.0 = _17.2.3.0;
Call(_25 = fn15(Move(_11.1), _24, _17.0, _9, _7, _17.0, _7, _21.0, _11.2, _7, _6), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
_17.0 = _1 ^ _9;
_17.2.2 = Move(_11.1.2);
_17.2.1 = !_17.2.3.0;
_11.1.2 = &_16;
_20 = !3_usize;
_24 = [_20,_20,_20,_20];
_11.1.3.1 = [false,true];
_20 = !1972931126066909379_usize;
_23.0 = !1223717623326112771_i64;
Goto(bb6)
}
bb10 = {
_11.2 = (_8,);
_12 = !2308143838_u32;
_11.1.3.0 = false as u64;
_3 = RET;
_5 = true as i128;
_4 = -_1;
_12 = 2943724257_u32 - 3604499823_u32;
_17.2.0 = [false,false];
_10 = (-8346947610897482941_i64) as f32;
_11.1.1 = !_11.1.3.0;
_17.2.3.0 = _11.2.0 as u64;
_11.1.3.1 = [true,false];
_11.1.3 = (_17.2.3.0, _11.1.0);
_16 = 20050_i16 as i8;
_11.2 = (_5,);
_11.1.2 = &_16;
_6 = _2 as f32;
_4 = _15 as isize;
_17.2.3 = (_11.1.1, _17.2.0);
_7 = _9 | _1;
Goto(bb5)
}
bb11 = {
_11.1.0 = [true,false];
_12 = _1 as u32;
_6 = _10;
_5 = _8;
_4 = 25525_u16 as isize;
_11.1.1 = _11.1.3.0 - _11.1.3.0;
_1 = _9 << _9;
_12 = !793116691_u32;
_12 = !2746287283_u32;
_11.2 = (_8,);
_4 = -_1;
_11.1.3.0 = 4_usize as u64;
_11.1.3.0 = RET as u64;
_15 = 77_i8 as u8;
_2 = RET - _3;
_12 = !770861268_u32;
_6 = _10;
Call(_10 = fn14(_5, _1, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_11.1.3 = (282063911461089046_u64, _11.1.0);
_7 = _9 * _4;
_6 = -_10;
RET = _3;
_11.1.3 = (3298910529703042119_u64, _11.1.0);
_7 = '\u{6b590}' as isize;
_11.1.0 = [true,false];
_11.1.3.0 = 10348353427806808093_u64;
Goto(bb3)
}
bb13 = {
_9 = _7 - _7;
_11.2.0 = _5;
_11.2.0 = _8;
_11.1.3.1 = [true,true];
_11.1.3.1 = [true,false];
_11.1.3.0 = !10326550102618858351_u64;
_11.1.0 = [false,false];
_3 = RET;
_4 = 145885854277721073225567994836562711321_u128 as isize;
_11.1.3.0 = 16115431287720115046_u64 | 16852285556981965543_u64;
RET = 109_i8 as f64;
RET = -_3;
_12 = !2473518989_u32;
_11.1.0 = _11.1.3.1;
_1 = _9 | _4;
_11.1.3.1 = [true,false];
_2 = RET;
_12 = 738063625_u32;
_11.2 = (_8,);
_6 = _10 * _10;
_4 = _1 >> _1;
_8 = 214333450991515119022004454665732477827_u128 as i128;
Goto(bb2)
}
bb14 = {
_27 = _16;
(*_19) = [_20,_20,_20,_20];
(*_19) = [_20,_20,_20,_20];
_17.2.3 = (_11.1.1, _17.2.0);
_17.2.2 = &_27;
_17.2.3.1 = _17.2.0;
_35 = _6;
_23 = ((-7680265227280871317_i64),);
_17.2.3.0 = !_21.3.0;
_33 = core::ptr::addr_of_mut!(_17.2);
_32 = !true;
(*_33).0 = (*_33).3.1;
(*_19) = [_20,_20,_20,_20];
_17.2.2 = &_16;
(*_33).0 = _17.2.3.1;
_15 = !75_u8;
_21.2 = &_27;
_17.2.0 = [_32,_32];
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(12_usize, 23_usize, Move(_23), 12_usize, Move(_12), 24_usize, Move(_24), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(12_usize, 8_usize, Move(_8), 32_usize, Move(_32), 15_usize, Move(_15), 31_usize, Move(_31)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: f64,mut _2: f64,mut _3: i128,mut _4: (i128,),mut _5: isize,mut _6: i128) -> f32 {
mir! {
type RET = f32;
let _7: isize;
let _8: bool;
let _9: bool;
let _10: (&'static [isize; 8], ([bool; 2], u64, &'static i8, (u64, [bool; 2])), (i128,));
let _11: i128;
let _12: *const ([bool; 2], u64, &'static i8, (u64, [bool; 2]));
let _13: isize;
let _14: &'static *mut u16;
let _15: f64;
let _16: (u64, [bool; 2]);
let _17: [u64; 3];
let _18: (f64, Adt37);
let _19: f32;
let _20: &'static i8;
let _21: *mut ([bool; 2], u64, &'static i8, (u64, [bool; 2]));
let _22: u8;
let _23: i64;
let _24: f32;
let _25: &'static *mut u16;
let _26: i16;
let _27: (&'static [isize; 8], ([bool; 2], u64, &'static i8, (u64, [bool; 2])), (i128,));
let _28: i128;
let _29: &'static u16;
let _30: u16;
let _31: ();
let _32: ();
{
_4 = (_3,);
Goto(bb1)
}
bb1 = {
_7 = _5 ^ _5;
_7 = 29277_u16 as isize;
_9 = true;
_10.1.3.1 = [_9,_9];
_3 = !_6;
_10.2 = (_4.0,);
_11 = 1576286466_u32 as i128;
_10.1.3.0 = !11705940166181022652_u64;
_11 = _6;
_2 = _1;
match _5 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463454151235394913435648 => bb8,
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
_1 = _2 + _2;
_12 = core::ptr::addr_of!(_10.1);
_12 = core::ptr::addr_of!(_10.1);
_8 = _9;
_10.1.1 = _10.1.3.0 | _10.1.3.0;
(*_12).1 = (*_12).3.0;
_10.1.3.0 = _4.0 as u64;
_10.1.3.1 = [_9,_8];
_10.2 = (_6,);
match _5 {
340282366920938463454151235394913435648 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
(*_12).0 = [_9,_9];
(*_12).3.0 = !(*_12).1;
(*_12).1 = !_10.1.3.0;
_10.1.3.1 = [_9,_9];
(*_12).3 = ((*_12).1, (*_12).0);
_6 = !_10.2.0;
_16 = _10.1.3;
_11 = _10.2.0 >> (*_12).1;
_15 = _7 as f64;
_3 = !_6;
_10.2.0 = -_6;
(*_12).1 = 100_u8 as u64;
_6 = _16.0 as i128;
_12 = core::ptr::addr_of!((*_12));
_10.1.3.0 = _10.1.1;
_12 = core::ptr::addr_of!(_10.1);
(*_12).3.1 = (*_12).0;
(*_12).1 = _16.0;
_13 = !_7;
(*_12).3 = (_16.0, (*_12).0);
_4 = (_11,);
_8 = !_9;
_9 = _1 != _1;
_10.2.0 = _3 & _3;
_16.0 = _10.1.3.0 | _10.1.1;
match _5 {
0 => bb8,
1 => bb7,
2 => bb3,
3 => bb5,
4 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_18.0 = (-8738844618591791421_i64) as f64;
_10.2.0 = -_3;
Goto(bb14)
}
bb14 = {
_22 = 114_u8;
_7 = !_5;
_10.1.1 = _16.0;
_21 = core::ptr::addr_of_mut!(_10.1);
(*_12).1 = !_16.0;
_17 = [_10.1.3.0,(*_21).3.0,_10.1.1];
(*_21).3 = _16;
(*_21).0 = (*_12).3.1;
_16.1 = [_9,_9];
(*_21).3.0 = _16.0;
_10.1.0 = _16.1;
_19 = 15608105161300401249_usize as f32;
_1 = -_18.0;
(*_12).1 = 3804389997_u32 as u64;
_7 = (-7324973636303003880_i64) as isize;
_10.1.1 = _10.1.3.0;
_10.1.3.0 = !(*_21).1;
_16 = ((*_12).1, (*_12).0);
_23 = 8661934405612206441_i64;
_13 = -_5;
_10.1.0 = [_9,_9];
_11 = _4.0 >> (*_21).1;
(*_12).1 = _16.0 * (*_21).3.0;
match _22 {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb11,
4 => bb12,
114 => bb16,
_ => bb15
}
}
bb15 = {
_7 = _5 ^ _5;
_7 = 29277_u16 as isize;
_9 = true;
_10.1.3.1 = [_9,_9];
_3 = !_6;
_10.2 = (_4.0,);
_11 = 1576286466_u32 as i128;
_10.1.3.0 = !11705940166181022652_u64;
_11 = _6;
_2 = _1;
match _5 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463454151235394913435648 => bb8,
_ => bb7
}
}
bb16 = {
_16.0 = (*_21).3.0 | (*_21).3.0;
(*_21).3.0 = _2 as u64;
_3 = _7 as i128;
(*_21).3.0 = !(*_21).1;
_10.2 = (_11,);
RET = -_19;
(*_12).1 = _16.0 - _16.0;
(*_21).3 = _16;
_16 = (_10.1.1, (*_21).0);
_16.0 = (*_21).1;
_4.0 = -_11;
(*_21).0 = [_8,_9];
_4 = _10.2;
RET = -_19;
_26 = (-1468_i16) ^ (-26725_i16);
_27.1.1 = !_10.1.3.0;
_16.0 = 31_i8 as u64;
_27.1.3.1 = (*_12).3.1;
_5 = _7 * _7;
(*_12).3.1 = [_9,_9];
_13 = _7;
Goto(bb17)
}
bb17 = {
Call(_31 = dump_var(13_usize, 8_usize, Move(_8), 7_usize, Move(_7), 16_usize, Move(_16), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_31 = dump_var(13_usize, 4_usize, Move(_4), 26_usize, Move(_26), 9_usize, Move(_9), 32_usize, _32), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: i128,mut _2: isize,mut _3: isize) -> f32 {
mir! {
type RET = f32;
let _4: *const *mut [bool; 2];
let _5: isize;
let _6: &'static [isize; 8];
let _7: u128;
let _8: Adt58;
let _9: [u128; 5];
let _10: *const [usize; 4];
let _11: Adt62;
let _12: &'static [char; 6];
let _13: i128;
let _14: [char; 6];
let _15: f64;
let _16: (i16, u64, bool, isize);
let _17: *const [usize; 4];
let _18: usize;
let _19: *mut u16;
let _20: i128;
let _21: f64;
let _22: Adt71;
let _23: u32;
let _24: f64;
let _25: Adt37;
let _26: bool;
let _27: ();
let _28: ();
{
_1 = -25922926954329787962726002575102008525_i128;
RET = 127_i8 as f32;
RET = 144813548235418605894092571898989651256_u128 as f32;
RET = 207_u8 as f32;
RET = _1 as f32;
Goto(bb1)
}
bb1 = {
_5 = _2 >> _3;
_8.fld4 = -RET;
_8.fld7 = 53061_u16 as u64;
_7 = 280240696_i32 as u128;
_8.fld2 = [4_usize,5_usize,7_usize,4_usize];
_8.fld3 = (-36_i8) << _2;
_7 = !309590651338913649861105885324377806774_u128;
_8.fld0 = false | false;
_2 = -_5;
_5 = -_2;
_8.fld6 = (_1,);
_8.fld6.0 = _1 >> _2;
_8.fld1 = '\u{45a0}';
_8.fld7 = 8976977517189015625_u64;
Goto(bb2)
}
bb2 = {
_8.fld5 = 54686_u16;
_8.fld7 = 7568848129265699734_u64;
_5 = _8.fld7 as isize;
_8.fld7 = _8.fld4 as u64;
_8.fld0 = !false;
_1 = _8.fld6.0 * _8.fld6.0;
RET = -_8.fld4;
_8.fld7 = 13640334715802544400_u64 + 9350516526931639113_u64;
_8.fld5 = 1_usize as u16;
_8.fld1 = '\u{c5f60}';
_8.fld7 = !5720754250612049774_u64;
_10 = core::ptr::addr_of!(_8.fld2);
_9 = [_7,_7,_7,_7,_7];
_10 = core::ptr::addr_of!((*_10));
_8.fld5 = 8225_u16;
_8.fld0 = !true;
_2 = _3 & _3;
_8.fld0 = _8.fld3 > _8.fld3;
_8.fld1 = '\u{5de57}';
RET = 477381629_u32 as f32;
_1 = _8.fld6.0;
_8.fld0 = !false;
_8.fld1 = '\u{8aa91}';
_8.fld6 = (_1,);
_8.fld6 = (_1,);
_8.fld2 = [7_usize,2978583089331401528_usize,13724269286408995350_usize,5957570209528211424_usize];
_5 = _3;
Goto(bb3)
}
bb3 = {
(*_10) = [4_usize,1619940403594573951_usize,18188456696492122128_usize,5_usize];
_8.fld0 = _8.fld6.0 < _8.fld6.0;
_8.fld6.0 = _1 >> _2;
_8.fld6 = (_1,);
_10 = core::ptr::addr_of!(_8.fld2);
_14 = [_8.fld1,_8.fld1,_8.fld1,_8.fld1,_8.fld1,_8.fld1];
_8.fld6.0 = _1;
_8.fld0 = false;
_13 = _7 as i128;
_2 = _8.fld3 as isize;
_12 = &_14;
_10 = core::ptr::addr_of!((*_10));
(*_10) = [1_usize,1_usize,4_usize,15392738294113917357_usize];
_8.fld5 = !7628_u16;
_17 = core::ptr::addr_of!((*_10));
_18 = 4193225044622084262_usize;
_16 = (20373_i16, _8.fld7, _8.fld0, _5);
(*_10) = [_18,_18,_18,_18];
_17 = Move(_10);
_5 = _16.3;
_19 = core::ptr::addr_of_mut!(_8.fld5);
_7 = 213280958662129984789051724114663907949_u128;
_1 = (-667447649_i32) as i128;
_8.fld2 = [_18,_18,_18,_18];
_14 = [_8.fld1,_8.fld1,_8.fld1,_8.fld1,_8.fld1,_8.fld1];
_8.fld6.0 = _1 << _5;
_8.fld2 = [_18,_18,_18,_18];
match _18 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
4193225044622084262 => bb11,
_ => bb10
}
}
bb4 = {
_8.fld5 = 54686_u16;
_8.fld7 = 7568848129265699734_u64;
_5 = _8.fld7 as isize;
_8.fld7 = _8.fld4 as u64;
_8.fld0 = !false;
_1 = _8.fld6.0 * _8.fld6.0;
RET = -_8.fld4;
_8.fld7 = 13640334715802544400_u64 + 9350516526931639113_u64;
_8.fld5 = 1_usize as u16;
_8.fld1 = '\u{c5f60}';
_8.fld7 = !5720754250612049774_u64;
_10 = core::ptr::addr_of!(_8.fld2);
_9 = [_7,_7,_7,_7,_7];
_10 = core::ptr::addr_of!((*_10));
_8.fld5 = 8225_u16;
_8.fld0 = !true;
_2 = _3 & _3;
_8.fld0 = _8.fld3 > _8.fld3;
_8.fld1 = '\u{5de57}';
RET = 477381629_u32 as f32;
_1 = _8.fld6.0;
_8.fld0 = !false;
_8.fld1 = '\u{8aa91}';
_8.fld6 = (_1,);
_8.fld6 = (_1,);
_8.fld2 = [7_usize,2978583089331401528_usize,13724269286408995350_usize,5957570209528211424_usize];
_5 = _3;
Goto(bb3)
}
bb5 = {
_5 = _2 >> _3;
_8.fld4 = -RET;
_8.fld7 = 53061_u16 as u64;
_7 = 280240696_i32 as u128;
_8.fld2 = [4_usize,5_usize,7_usize,4_usize];
_8.fld3 = (-36_i8) << _2;
_7 = !309590651338913649861105885324377806774_u128;
_8.fld0 = false | false;
_2 = -_5;
_5 = -_2;
_8.fld6 = (_1,);
_8.fld6.0 = _1 >> _2;
_8.fld1 = '\u{45a0}';
_8.fld7 = 8976977517189015625_u64;
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
_15 = RET as f64;
_3 = _5;
_2 = _3 << _16.3;
_21 = _15 + _15;
_15 = _21 + _21;
_12 = &_14;
_16.1 = _8.fld7;
_2 = _16.3 ^ _16.3;
_16.0 = _15 as i16;
_8.fld7 = _16.1 << _2;
_18 = !5_usize;
_1 = !_8.fld6.0;
_8.fld5 = 33065_u16 + 43415_u16;
_15 = _21;
_5 = _16.3 + _3;
match _7 {
0 => bb12,
1 => bb13,
213280958662129984789051724114663907949 => bb15,
_ => bb14
}
}
bb12 = {
(*_10) = [4_usize,1619940403594573951_usize,18188456696492122128_usize,5_usize];
_8.fld0 = _8.fld6.0 < _8.fld6.0;
_8.fld6.0 = _1 >> _2;
_8.fld6 = (_1,);
_10 = core::ptr::addr_of!(_8.fld2);
_14 = [_8.fld1,_8.fld1,_8.fld1,_8.fld1,_8.fld1,_8.fld1];
_8.fld6.0 = _1;
_8.fld0 = false;
_13 = _7 as i128;
_2 = _8.fld3 as isize;
_12 = &_14;
_10 = core::ptr::addr_of!((*_10));
(*_10) = [1_usize,1_usize,4_usize,15392738294113917357_usize];
_8.fld5 = !7628_u16;
_17 = core::ptr::addr_of!((*_10));
_18 = 4193225044622084262_usize;
_16 = (20373_i16, _8.fld7, _8.fld0, _5);
(*_10) = [_18,_18,_18,_18];
_17 = Move(_10);
_5 = _16.3;
_19 = core::ptr::addr_of_mut!(_8.fld5);
_7 = 213280958662129984789051724114663907949_u128;
_1 = (-667447649_i32) as i128;
_8.fld2 = [_18,_18,_18,_18];
_14 = [_8.fld1,_8.fld1,_8.fld1,_8.fld1,_8.fld1,_8.fld1];
_8.fld6.0 = _1 << _5;
_8.fld2 = [_18,_18,_18,_18];
match _18 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
4193225044622084262 => bb11,
_ => bb10
}
}
bb13 = {
_5 = _2 >> _3;
_8.fld4 = -RET;
_8.fld7 = 53061_u16 as u64;
_7 = 280240696_i32 as u128;
_8.fld2 = [4_usize,5_usize,7_usize,4_usize];
_8.fld3 = (-36_i8) << _2;
_7 = !309590651338913649861105885324377806774_u128;
_8.fld0 = false | false;
_2 = -_5;
_5 = -_2;
_8.fld6 = (_1,);
_8.fld6.0 = _1 >> _2;
_8.fld1 = '\u{45a0}';
_8.fld7 = 8976977517189015625_u64;
Goto(bb2)
}
bb14 = {
Return()
}
bb15 = {
_16 = ((-28991_i16), _8.fld7, _8.fld0, _2);
_15 = _18 as f64;
_16.3 = !_2;
RET = _8.fld3 as f32;
_8.fld0 = _16.2 | _16.2;
RET = _8.fld4;
_20 = -_8.fld6.0;
_16.3 = 221_u8 as isize;
Goto(bb16)
}
bb16 = {
Call(_27 = dump_var(14_usize, 5_usize, Move(_5), 20_usize, Move(_20), 1_usize, Move(_1), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_27 = dump_var(14_usize, 18_usize, Move(_18), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: ([bool; 2], u64, &'static i8, (u64, [bool; 2])),mut _2: [usize; 4],mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: [bool; 2],mut _9: (i128,),mut _10: isize,mut _11: f32) -> i32 {
mir! {
type RET = i32;
let _12: (isize,);
let _13: f32;
let _14: u64;
let _15: f64;
let _16: ();
let _17: ();
{
_8 = _1.0;
_12 = (_3,);
_12 = (_3,);
_1.3 = (_1.1, _1.0);
_13 = -_11;
_9 = (24084567466017314844634061179949386227_i128,);
_1.3 = (_1.1, _8);
_13 = _3 as f32;
_9 = ((-96434449905205306888697728632887026918_i128),);
RET = -2077117649_i32;
_7 = _10;
_13 = _11;
_8 = _1.3.1;
_14 = 18249_i16 as u64;
_1.3.1 = [false,false];
_4 = -_5;
_5 = 62770_u16 as isize;
_15 = _1.1 as f64;
_13 = -_11;
_1.1 = !_1.3.0;
_1.0 = [false,true];
RET = (-71938265_i32) >> _6;
_12.0 = _13 as isize;
_12 = (_7,);
_1.3.1 = [true,false];
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(15_usize, 5_usize, Move(_5), 8_usize, Move(_8), 12_usize, Move(_12), 2_usize, Move(_2)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_16 = dump_var(15_usize, 6_usize, Move(_6), 17_usize, _17, 17_usize, _17, 17_usize, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: u64) -> isize {
mir! {
type RET = isize;
let _2: isize;
let _3: i32;
let _4: [usize; 5];
let _5: [u128; 5];
let _6: [usize; 4];
let _7: f64;
let _8: [u64; 3];
let _9: i128;
let _10: i128;
let _11: [u32; 6];
let _12: &'static *mut u16;
let _13: f64;
let _14: (isize,);
let _15: char;
let _16: isize;
let _17: isize;
let _18: isize;
let _19: i32;
let _20: bool;
let _21: Adt68;
let _22: isize;
let _23: isize;
let _24: f32;
let _25: Adt58;
let _26: u8;
let _27: i32;
let _28: ();
let _29: ();
{
_2 = (-9223372036854775808_isize) - (-47_isize);
RET = !_2;
_3 = (-951701912_i32) | 296524279_i32;
RET = (-20521_i16) as isize;
RET = _2 + _2;
RET = -_2;
RET = '\u{9d566}' as isize;
RET = _2;
_2 = RET * RET;
_3 = -(-948568876_i32);
Goto(bb1)
}
bb1 = {
_4 = [1_usize,10015462424352327119_usize,8890042478815303286_usize,4_usize,5695709387737722489_usize];
_2 = RET;
_5 = [205948286644829961104460383794658964242_u128,205331074316280438847622651492556137194_u128,37183133549266703198926701001052702551_u128,229670472459844692377939675648462000470_u128,313825174447960799034619987020759446436_u128];
_7 = 2259209350600663190_usize as f64;
_2 = -RET;
_6 = [5_usize,1_usize,9895479581978751446_usize,8093656562992149366_usize];
_3 = _7 as i32;
_7 = RET as f64;
_6 = [13884576594208035234_usize,2_usize,0_usize,7_usize];
_5 = [295158524420230007271013731001994881579_u128,130551987556230802438462399904879897945_u128,18173195342277955973444560733827262441_u128,258482025963794454752135889791220364186_u128,55866028821783552684037512857465755960_u128];
_6 = [2_usize,0_usize,2_usize,6_usize];
_3 = (-27688_i16) as i32;
_3 = -(-1717575482_i32);
_3 = 313715323_i32 | (-45782235_i32);
RET = _2 - _2;
_5 = [133090777015773491987020696675559185208_u128,57412008115975712450218751346622714962_u128,297911274722082481503627171583831111971_u128,9547689988680344558186746026150140090_u128,326418160289234837694123953547425256767_u128];
_1 = 5419038792823120221_u64;
match _1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
5419038792823120221 => bb7,
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
_4 = [4_usize,3880667391872183691_usize,7_usize,7_usize,7_usize];
_4 = [13635396211664327069_usize,2_usize,5_usize,10096605953595398113_usize,3251522195403864805_usize];
_8 = [_1,_1,_1];
RET = _2;
_10 = (-8170_i16) as i128;
_4 = [7_usize,7_usize,2_usize,12010114209073057851_usize,16126551552582537468_usize];
_7 = (-35_i8) as f64;
_6 = [5_usize,13034382498052523059_usize,2_usize,13917559319877760970_usize];
_9 = 10028_i16 as i128;
_6 = [7_usize,17411815748267281719_usize,7_usize,9545555570202997430_usize];
_10 = _2 as i128;
RET = _2 & _2;
_7 = _2 as f64;
_6 = [4154841956541865131_usize,4001728929324197146_usize,18278741009773863159_usize,6_usize];
_8 = [_1,_1,_1];
_13 = _2 as f64;
_14 = (RET,);
_8 = [_1,_1,_1];
match _1 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb4,
4 => bb8,
5 => bb9,
6 => bb10,
5419038792823120221 => bb12,
_ => bb11
}
}
bb8 = {
_4 = [1_usize,10015462424352327119_usize,8890042478815303286_usize,4_usize,5695709387737722489_usize];
_2 = RET;
_5 = [205948286644829961104460383794658964242_u128,205331074316280438847622651492556137194_u128,37183133549266703198926701001052702551_u128,229670472459844692377939675648462000470_u128,313825174447960799034619987020759446436_u128];
_7 = 2259209350600663190_usize as f64;
_2 = -RET;
_6 = [5_usize,1_usize,9895479581978751446_usize,8093656562992149366_usize];
_3 = _7 as i32;
_7 = RET as f64;
_6 = [13884576594208035234_usize,2_usize,0_usize,7_usize];
_5 = [295158524420230007271013731001994881579_u128,130551987556230802438462399904879897945_u128,18173195342277955973444560733827262441_u128,258482025963794454752135889791220364186_u128,55866028821783552684037512857465755960_u128];
_6 = [2_usize,0_usize,2_usize,6_usize];
_3 = (-27688_i16) as i32;
_3 = -(-1717575482_i32);
_3 = 313715323_i32 | (-45782235_i32);
RET = _2 - _2;
_5 = [133090777015773491987020696675559185208_u128,57412008115975712450218751346622714962_u128,297911274722082481503627171583831111971_u128,9547689988680344558186746026150140090_u128,326418160289234837694123953547425256767_u128];
_1 = 5419038792823120221_u64;
match _1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
5419038792823120221 => bb7,
_ => bb6
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
_6 = [4_usize,15216727146654687002_usize,7863045794085039576_usize,7531383601900691128_usize];
_5 = [171676605932705147598523835013641595827_u128,64675538243643981772471577771567178921_u128,239249421360154237308099681735964706275_u128,225152512101609483211102739078143093577_u128,263664081444473265726161848647910156828_u128];
_14.0 = 15225952215789148567_usize as isize;
_14.0 = _2;
_1 = 6810959419231167841_i64 as u64;
_10 = _9;
RET = _14.0 | _2;
_13 = _7;
_10 = _9 >> _3;
_3 = 398983426_i32;
_16 = !_14.0;
_13 = -_7;
_8 = [_1,_1,_1];
_14.0 = !_16;
_18 = -RET;
_15 = '\u{173d5}';
_1 = 4736215909455194243_u64 + 2785768998431267274_u64;
_6 = [5412459088905692827_usize,13007076188454991227_usize,7_usize,17222605004265002395_usize];
_14.0 = RET * _18;
_14.0 = !RET;
_13 = _7;
_1 = 17010236562724879141_u64 & 1796045735631566406_u64;
_19 = _3 + _3;
_14 = (RET,);
Goto(bb13)
}
bb13 = {
_20 = false;
_18 = _14.0 >> _1;
_20 = !false;
_11 = [3508929449_u32,2412720267_u32,1566383291_u32,115029370_u32,2162814503_u32,879413989_u32];
_10 = _9;
_4 = [1900657625738182946_usize,15640133418133641645_usize,1_usize,17421097678144352271_usize,2243602110089116165_usize];
_13 = _7;
_2 = -RET;
_17 = _20 as isize;
_4 = [10078545900650723646_usize,3939100079435039700_usize,1445999465616621474_usize,14478417354974126621_usize,3507799160423271478_usize];
_5 = [334027451419020902477983071567639246266_u128,201745641780429825250537810371849517297_u128,298036157835671559207804271174939512756_u128,246849046890133323180073282079439122126_u128,218602931214749303033792371634270919563_u128];
match _3 {
0 => bb14,
398983426 => bb16,
_ => bb15
}
}
bb14 = {
_4 = [1_usize,10015462424352327119_usize,8890042478815303286_usize,4_usize,5695709387737722489_usize];
_2 = RET;
_5 = [205948286644829961104460383794658964242_u128,205331074316280438847622651492556137194_u128,37183133549266703198926701001052702551_u128,229670472459844692377939675648462000470_u128,313825174447960799034619987020759446436_u128];
_7 = 2259209350600663190_usize as f64;
_2 = -RET;
_6 = [5_usize,1_usize,9895479581978751446_usize,8093656562992149366_usize];
_3 = _7 as i32;
_7 = RET as f64;
_6 = [13884576594208035234_usize,2_usize,0_usize,7_usize];
_5 = [295158524420230007271013731001994881579_u128,130551987556230802438462399904879897945_u128,18173195342277955973444560733827262441_u128,258482025963794454752135889791220364186_u128,55866028821783552684037512857465755960_u128];
_6 = [2_usize,0_usize,2_usize,6_usize];
_3 = (-27688_i16) as i32;
_3 = -(-1717575482_i32);
_3 = 313715323_i32 | (-45782235_i32);
RET = _2 - _2;
_5 = [133090777015773491987020696675559185208_u128,57412008115975712450218751346622714962_u128,297911274722082481503627171583831111971_u128,9547689988680344558186746026150140090_u128,326418160289234837694123953547425256767_u128];
_1 = 5419038792823120221_u64;
match _1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
5419038792823120221 => bb7,
_ => bb6
}
}
bb15 = {
Return()
}
bb16 = {
_11 = [1487712086_u32,639150250_u32,2324430186_u32,2626090609_u32,137272547_u32,2686658427_u32];
_22 = RET >> _2;
_1 = 16986415992619961781_u64;
_14.0 = _18 | _22;
_19 = _10 as i32;
_17 = _16;
_15 = '\u{c400b}';
_2 = -_18;
_9 = (-122_i8) as i128;
_14.0 = RET;
_1 = 13843496005297758082_u64 - 2315775938841649064_u64;
_25.fld7 = _1 + _1;
_25.fld2 = [4_usize,7_usize,5_usize,176848595884870481_usize];
Goto(bb17)
}
bb17 = {
Call(_28 = dump_var(16_usize, 4_usize, Move(_4), 18_usize, Move(_18), 14_usize, Move(_14), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_28 = dump_var(16_usize, 22_usize, Move(_22), 2_usize, Move(_2), 1_usize, Move(_1), 16_usize, Move(_16)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_28 = dump_var(16_usize, 10_usize, Move(_10), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: *mut u16,mut _2: [u64; 5],mut _3: isize,mut _4: isize,mut _5: [bool; 2],mut _6: bool,mut _7: u64,mut _8: [usize; 4],mut _9: bool) -> [u64; 5] {
mir! {
type RET = [u64; 5];
let _10: (i16, u64, bool, isize);
let _11: &'static [i16; 7];
let _12: u8;
let _13: u8;
let _14: &'static u16;
let _15: bool;
let _16: [i64; 4];
let _17: i128;
let _18: i8;
let _19: bool;
let _20: char;
let _21: [u64; 5];
let _22: [u64; 5];
let _23: (isize,);
let _24: [usize; 5];
let _25: ();
let _26: ();
{
RET = [_7,_7,_7,_7,_7];
_8 = [2_usize,5346847775257397351_usize,2_usize,2310826062520978853_usize];
Goto(bb1)
}
bb1 = {
_6 = _9 <= _9;
_10 = ((-20620_i16), _7, _6, _4);
_10 = ((-21769_i16), _7, _9, _4);
_10.0 = (-9580_i16) >> _3;
_10 = ((-11866_i16), _7, _6, _4);
_10.0 = 14351_i16;
match _10.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
14351 => bb10,
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
_10.2 = _9 < _6;
Call(_10.2 = fn18(_9, _6, _9, _9, _6, _4, _6, _4, _6, _6), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_3 = !_4;
RET = _2;
_12 = 104_u8;
Goto(bb12)
}
bb12 = {
_10.1 = _7;
_10.3 = 63306_u16 as isize;
_5 = [_6,_10.2];
_4 = -_3;
_10.3 = _3 << _4;
_4 = _10.0 as isize;
_7 = _10.1 | _10.1;
_13 = 98538758339770023578937287425587688445_u128 as u8;
_2 = [_7,_7,_10.1,_7,_7];
_2 = [_10.1,_7,_7,_7,_10.1];
_4 = _10.3;
_10 = ((-9799_i16), _7, _6, _3);
_3 = '\u{cafb3}' as isize;
_7 = _10.1 << _10.0;
_5 = [_10.2,_10.2];
_8 = [2_usize,2_usize,7596145068508748034_usize,8265203453564526933_usize];
RET = _2;
_7 = !_10.1;
_10.1 = _4 as u64;
_10 = (23879_i16, _7, _9, _4);
_4 = _10.3;
_9 = !_10.2;
_10 = (12700_i16, _7, _9, _4);
_10.1 = 64225_u16 as u64;
Goto(bb13)
}
bb13 = {
_10 = (2393_i16, _7, _9, _4);
RET = [_10.1,_10.1,_7,_10.1,_7];
_4 = _10.3 * _10.3;
_10.2 = _6 == _6;
_13 = !_12;
_10.0 = 51617414663631005246357049861604537071_i128 as i16;
_10.1 = 5580502376780355386_usize as u64;
_16 = [(-5529493715296003515_i64),(-8948318039230015685_i64),(-4418002992482122324_i64),8554925948518659182_i64];
_10.2 = !_9;
_9 = _6 | _6;
RET = [_7,_7,_7,_10.1,_7];
_10.3 = 1133578230_i32 as isize;
_9 = !_10.2;
_17 = (-160925967168387379863948965367589708249_i128);
_3 = 14205_u16 as isize;
_5 = [_6,_6];
_2 = RET;
_10.1 = _7;
_13 = _12 % _12;
_16 = [8422198884282621355_i64,193445620925340252_i64,5293295630983048406_i64,(-7029997565291375386_i64)];
_10.1 = _7;
Goto(bb14)
}
bb14 = {
_15 = _10.1 >= _7;
_7 = _10.1;
_9 = _6 == _6;
_18 = _17 as i8;
_6 = _9 & _10.2;
_5 = [_10.2,_6];
_13 = _12;
_5 = [_6,_6];
_13 = _12 ^ _12;
_21 = _2;
_22 = [_7,_10.1,_10.1,_7,_10.1];
_19 = !_9;
_17 = -(-131064224010507160800647957097451941484_i128);
_13 = _12;
_9 = !_6;
_10.3 = -_4;
_5 = [_19,_6];
_10 = ((-1973_i16), _7, _9, _4);
_20 = '\u{e5b48}';
_13 = _12 >> _10.0;
_10.3 = !_4;
_8 = [6_usize,5250927664817596051_usize,7_usize,12067802210411284208_usize];
_7 = _10.1;
_10 = ((-9857_i16), _7, _19, _4);
_12 = _13;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(17_usize, 16_usize, Move(_16), 13_usize, Move(_13), 20_usize, Move(_20), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(17_usize, 21_usize, Move(_21), 5_usize, Move(_5), 19_usize, Move(_19), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_25 = dump_var(17_usize, 2_usize, Move(_2), 26_usize, _26, 26_usize, _26, 26_usize, _26), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}
}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: isize,mut _7: bool,mut _8: isize,mut _9: bool,mut _10: bool) -> bool {
mir! {
type RET = bool;
let _11: Adt60;
let _12: isize;
let _13: &'static [u64; 5];
let _14: ();
let _15: ();
{
_7 = !_10;
_4 = _9 < _7;
_5 = _3 == _10;
_6 = 59608_u16 as isize;
_8 = !_6;
_1 = !_5;
_9 = _5 < _3;
_5 = _9 ^ _2;
_4 = _1;
_9 = _10;
RET = _5;
_10 = !_5;
_9 = !_10;
_10 = _1;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(18_usize, 5_usize, Move(_5), 4_usize, Move(_4), 3_usize, Move(_3), 9_usize, Move(_9)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_14 = dump_var(18_usize, 8_usize, Move(_8), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box((-1982478700548269306_i64)), std::hint::black_box(36_u8), std::hint::black_box(48980_u16), std::hint::black_box((-95_i8)), std::hint::black_box(56668605387823715405091918113786810032_i128), std::hint::black_box(6307996482051718763_u64));
                
            }
impl PrintFDebug for Adt37{
	unsafe fn printf_debug(&self){unsafe{printf("Adt37::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld5:".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld6:".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld5:".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld6:".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt37 {
Variant0{
fld0: [bool; 2],
fld1: char,
fld2: (i16, i8),
fld3: i8,
fld4: u8,
fld5: [i32; 3],
fld6: [u64; 5],

},
Variant1{
fld0: [u64; 5],
fld1: u16,
fld2: isize,

},
Variant2{
fld0: bool,
fld1: usize,
fld2: u128,
fld3: i8,
fld4: *mut [bool; 2],
fld5: u64,
fld6: i64,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: (f64, Adt37),
fld1: [i64; 4],
fld2: [u64; 5],
fld3: i8,
fld4: [u64; 3],
}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld5:".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld5:".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld6:".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: ((f64, Adt37), i8, *const [usize; 4]),
fld1: char,
fld2: u64,
fld3: Adt49,
fld4: u32,
fld5: usize,

},
Variant1{
fld0: [u64; 3],
fld1: *mut [bool; 2],
fld2: (f64, Adt37),
fld3: i8,
fld4: (i128,),

},
Variant2{
fld0: [usize; 4],
fld1: u16,
fld2: u8,
fld3: (f64, Adt37),
fld4: (i16, i8),
fld5: [u64; 5],
fld6: [i64; 4],
fld7: *const f64,

},
Variant3{
fld0: u128,
fld1: [isize; 8],
fld2: isize,
fld3: [u32; 6],
fld4: (i128,),

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt58{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt58 {
fld0: bool,
fld1: char,
fld2: [usize; 4],
fld3: i8,
fld4: f32,
fld5: u16,
fld6: (i128,),
fld7: u64,
}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf("Adt60::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld5:".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld6:".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld5:".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld6:".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: *mut u16,
fld1: [u64; 3],
fld2: [u32; 6],
fld3: [i8; 6],
fld4: (i8, [u64; 3]),
fld5: (i128,),
fld6: (i16, i8),
fld7: i128,

},
Variant1{
fld0: [u32; 6],
fld1: (i8, [u64; 3]),

},
Variant2{
fld0: u32,
fld1: (usize, [isize; 8], [i16; 7], (i128,)),
fld2: Adt49,
fld3: Adt37,
fld4: Adt58,
fld5: *const f64,
fld6: *const [usize; 4],
fld7: (f64, Adt37),

}}
impl PrintFDebug for Adt62{
	unsafe fn printf_debug(&self){unsafe{printf("Adt62::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt62 {
Variant0{
fld0: *const f64,

},
Variant1{
fld0: [u128; 5],
fld1: Adt57,
fld2: (i16, i8),
fld3: usize,

}}
impl PrintFDebug for Adt68{
	unsafe fn printf_debug(&self){unsafe{printf("Adt68::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld5:".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld6:".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt68 {
Variant0{
fld0: Adt60,
fld1: *mut u16,
fld2: isize,
fld3: f64,
fld4: [bool; 2],
fld5: (i32, usize, i16, bool),
fld6: [i32; 3],
fld7: (i8, [u64; 3]),

},
Variant1{
fld0: [u64; 3],
fld1: i32,
fld2: isize,
fld3: [i64; 4],
fld4: f32,

},
Variant2{
fld0: i32,
fld1: u8,
fld2: [i16; 7],
fld3: Adt60,
fld4: f64,

},
Variant3{
fld0: [bool; 2],
fld1: i128,
fld2: (u64, [bool; 2]),
fld3: *const f64,

}}
impl PrintFDebug for Adt71{
	unsafe fn printf_debug(&self){unsafe{printf("Adt71::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld5:".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld6:".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld2:".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld3:".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld4:".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld5:".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld6:".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt71 {
Variant0{
fld0: Adt57,

},
Variant1{
fld0: [i32; 3],
fld1: (i16, i8),
fld2: (i128,),
fld3: [i16; 7],
fld4: (i8, [u64; 3]),
fld5: i32,
fld6: *const f64,

},
Variant2{
fld0: *const f64,
fld1: u8,
fld2: [u32; 6],
fld3: f32,
fld4: i16,
fld5: i32,
fld6: i64,
fld7: [u64; 3],

},
Variant3{
fld0: [usize; 4],
fld1: [bool; 4],

}}

