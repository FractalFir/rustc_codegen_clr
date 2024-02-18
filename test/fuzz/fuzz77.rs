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
pub fn fn0(mut _1: u16,mut _2: char,mut _3: i128) -> [u32; 3] {
mir! {
type RET = [u32; 3];
let _4: [u32; 3];
let _5: usize;
let _6: isize;
let _7: (&'static &'static i8, [u32; 3], [u32; 3], bool);
let _8: char;
let _9: (usize,);
let _10: *const (Adt27, *const u64, [u8; 5]);
let _11: u8;
let _12: usize;
let _13: (i16, bool);
let _14: (usize,);
let _15: (i8, f64);
let _16: *const Adt35;
let _17: [isize; 4];
let _18: isize;
let _19: f64;
let _20: u8;
let _21: isize;
let _22: &'static (i16, bool);
let _23: *mut (*mut &'static i8,);
let _24: (usize,);
let _25: Adt62;
let _26: (usize,);
let _27: bool;
let _28: ();
let _29: ();
{
RET = [1676413813_u32,3020075336_u32,1909035304_u32];
RET = [848856410_u32,1437186888_u32,1014628819_u32];
_4 = [961219165_u32,3824421886_u32,1105404880_u32];
_3 = -(-164240861148779203167190166634753364356_i128);
_1 = 3523_u16;
_2 = '\u{6e921}';
_2 = '\u{264ff}';
_1 = 1760127581228361518_u64 as u16;
_5 = 17097607944844434433_usize;
_3 = 102267735110685331520560546131473682903_i128;
RET = _4;
_1 = (-9223372036854775808_isize) as u16;
_2 = '\u{f19b}';
_2 = '\u{38cab}';
_3 = -78170908856583187000276390463594486743_i128;
_4 = [1472741018_u32,446763616_u32,425975618_u32];
_5 = 2_usize;
_7.2 = [RET[_5],_4[_5],_4[_5]];
RET = [_4[_5],_4[_5],_7.2[_5]];
_2 = '\u{75d04}';
_7.1 = _7.2;
_7.3 = RET[_5] != RET[_5];
_7.1 = _7.2;
_7.1 = [_4[_5],_7.2[_5],_4[_5]];
_7.1[_5] = _7.2[_5] - _4[_5];
_1 = 37314_u16 - 46182_u16;
_4 = [_7.1[_5],_7.1[_5],_7.2[_5]];
Goto(bb1)
}
bb1 = {
_4 = [_7.1[_5],_7.1[_5],_7.1[_5]];
_1 = 15142_i16 as u16;
_1 = _7.3 as u16;
_7.1[_5] = _7.2[_5] / RET[_5];
_6 = !(-9223372036854775808_isize);
_3 = -83888524153683035546606397982963662548_i128;
RET[_5] = _7.1[_5];
_7.2[_5] = 19_i8 as u32;
_5 = 3865754533686780828_usize;
_5 = 7591548627933846001_usize;
_7.3 = true;
RET = _7.2;
Goto(bb2)
}
bb2 = {
_9 = (_5,);
_1 = !50505_u16;
_7.3 = !true;
_8 = _2;
_6 = !(-9223372036854775808_isize);
Goto(bb3)
}
bb3 = {
RET = [3580537701_u32,2007135612_u32,1561647654_u32];
_3 = 137323624382009995240082975878290490093_i128;
_9.0 = _5 & _5;
_1 = 496_u16;
_7.1 = [1323478911_u32,161711095_u32,362253299_u32];
_9.0 = _5;
RET = [836983025_u32,779770404_u32,4248799730_u32];
_9.0 = !_5;
_3 = 66201359911762492956242044147182385509_i128 * 151265328030738783857735443581486598618_i128;
_1 = _7.3 as u16;
_6 = _9.0 as isize;
_9 = (_5,);
_8 = _2;
_5 = _9.0 | _9.0;
RET = [1992042669_u32,282689864_u32,982073464_u32];
_3 = !(-56288246258694411486578882631413258483_i128);
_5 = _9.0 & _9.0;
_7.3 = !false;
Call(_1 = fn1(_2, _7.3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1 = 504_i16 as u16;
_9.0 = !_5;
_11 = 66_u8;
_9 = (_5,);
_4 = RET;
RET = [906090104_u32,4128039935_u32,2939331851_u32];
RET = _4;
_1 = 37336_u16;
_1 = !50325_u16;
RET = [3190943345_u32,2563098399_u32,4123572750_u32];
RET = [1696339860_u32,2412691552_u32,364512320_u32];
_13.0 = (-17449_i16);
_3 = 5988192052559661701451854206907197704_i128;
_6 = -(-9223372036854775808_isize);
_7.3 = true;
_14 = (_5,);
_13.1 = _6 < _6;
_9.0 = !_5;
_13 = ((-3183_i16), _7.3);
_1 = 58359_u16;
_3 = !169306395835862033179776799041744590388_i128;
_5 = !_14.0;
_9 = _14;
_2 = _8;
_11 = 92_u8;
_1 = 17591_u16 << _14.0;
RET = [408706453_u32,1258465441_u32,405763948_u32];
_7.1 = [185646972_u32,3073805760_u32,1085947557_u32];
Call(_10 = fn4(_1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_8 = _2;
RET = [423423482_u32,1707139965_u32,947749988_u32];
RET = _7.2;
_2 = _8;
_7.3 = _1 > _1;
_1 = !60175_u16;
_9 = (_5,);
Call(_9.0 = core::intrinsics::bswap(_14.0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_4 = [1729323530_u32,650898670_u32,342605800_u32];
_14 = (_9.0,);
_13 = ((-27804_i16), _7.3);
_2 = _8;
_15.1 = 4314931530527904550_u64 as f64;
_6 = _2 as isize;
Goto(bb7)
}
bb7 = {
_4 = _7.1;
_13 = Checked((-21033_i16) + (-9041_i16));
_7.3 = _13.1;
_12 = _14.0;
_15.1 = 17703475211190636144_u64 as f64;
_17 = [_6,_6,_6,_6];
_12 = _9.0;
_15.1 = _14.0 as f64;
_7.1 = [2607848899_u32,2673121123_u32,298054731_u32];
_12 = _14.0;
_12 = !_9.0;
_15.0 = !46_i8;
_17 = [_6,_6,_6,_6];
_6 = !(-9223372036854775808_isize);
_5 = !_14.0;
_3 = 106322845747091703759708710815516007515_i128;
_15.1 = _15.0 as f64;
_15.0 = _3 as i8;
_19 = 15632743208381944013_u64 as f64;
_15 = (86_i8, _19);
_7.1 = _4;
RET = _7.2;
_9 = (_14.0,);
Goto(bb8)
}
bb8 = {
_8 = _2;
_7.2 = [257095025_u32,1794814524_u32,834328011_u32];
RET = _7.2;
_13.0 = !(-18898_i16);
_9.0 = _14.0;
_7.1 = _7.2;
_11 = 3135848813_u32 as u8;
_2 = _8;
_1 = 5090_u16;
_9 = (_12,);
_15 = (112_i8, _19);
_7.2 = _7.1;
_18 = (-6554860275894716641_i64) as isize;
_3 = 165946824131208724831510678430811745881_i128;
_17 = [_18,_6,_18,_18];
_8 = _2;
_17 = [_6,_6,_18,_6];
_4 = _7.2;
_7.1 = [702208836_u32,127313928_u32,333263856_u32];
_7.2 = _4;
_20 = _11 << _13.0;
RET = [1091288752_u32,3574231827_u32,3009015647_u32];
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb7,
5090 => bb9,
_ => bb6
}
}
bb9 = {
_8 = _2;
_21 = !_18;
RET = _7.1;
_14 = (_12,);
_18 = -_21;
_18 = -_21;
_13.1 = _7.3 | _7.3;
_15.0 = (-54_i8) << _14.0;
_21 = _13.0 as isize;
_7.2 = RET;
_7.1 = [1728855926_u32,3115890419_u32,1374155349_u32];
_24 = _9;
_18 = _2 as isize;
RET = [3123521951_u32,824792275_u32,1349999077_u32];
_2 = _8;
_6 = _18 & _21;
_7.2 = _4;
_7.3 = !_13.1;
_15.0 = -65_i8;
_9 = (_5,);
_12 = _9.0;
Goto(bb10)
}
bb10 = {
_15 = ((-92_i8), _19);
_24.0 = _15.0 as usize;
_17 = [_21,_6,_6,_21];
_26 = (_12,);
_11 = _20 << _14.0;
_11 = _7.3 as u8;
_9 = (_12,);
_1 = !39484_u16;
_26 = (_9.0,);
match _3 {
0 => bb1,
1 => bb5,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
165946824131208724831510678430811745881 => bb16,
_ => bb15
}
}
bb11 = {
_8 = _2;
_21 = !_18;
RET = _7.1;
_14 = (_12,);
_18 = -_21;
_18 = -_21;
_13.1 = _7.3 | _7.3;
_15.0 = (-54_i8) << _14.0;
_21 = _13.0 as isize;
_7.2 = RET;
_7.1 = [1728855926_u32,3115890419_u32,1374155349_u32];
_24 = _9;
_18 = _2 as isize;
RET = [3123521951_u32,824792275_u32,1349999077_u32];
_2 = _8;
_6 = _18 & _21;
_7.2 = _4;
_7.3 = !_13.1;
_15.0 = -65_i8;
_9 = (_5,);
_12 = _9.0;
Goto(bb10)
}
bb12 = {
_9 = (_5,);
_1 = !50505_u16;
_7.3 = !true;
_8 = _2;
_6 = !(-9223372036854775808_isize);
Goto(bb3)
}
bb13 = {
RET = [3580537701_u32,2007135612_u32,1561647654_u32];
_3 = 137323624382009995240082975878290490093_i128;
_9.0 = _5 & _5;
_1 = 496_u16;
_7.1 = [1323478911_u32,161711095_u32,362253299_u32];
_9.0 = _5;
RET = [836983025_u32,779770404_u32,4248799730_u32];
_9.0 = !_5;
_3 = 66201359911762492956242044147182385509_i128 * 151265328030738783857735443581486598618_i128;
_1 = _7.3 as u16;
_6 = _9.0 as isize;
_9 = (_5,);
_8 = _2;
_5 = _9.0 | _9.0;
RET = [1992042669_u32,282689864_u32,982073464_u32];
_3 = !(-56288246258694411486578882631413258483_i128);
_5 = _9.0 & _9.0;
_7.3 = !false;
Call(_1 = fn1(_2, _7.3), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
_4 = [1729323530_u32,650898670_u32,342605800_u32];
_14 = (_9.0,);
_13 = ((-27804_i16), _7.3);
_2 = _8;
_15.1 = 4314931530527904550_u64 as f64;
_6 = _2 as isize;
Goto(bb7)
}
bb15 = {
_1 = 504_i16 as u16;
_9.0 = !_5;
_11 = 66_u8;
_9 = (_5,);
_4 = RET;
RET = [906090104_u32,4128039935_u32,2939331851_u32];
RET = _4;
_1 = 37336_u16;
_1 = !50325_u16;
RET = [3190943345_u32,2563098399_u32,4123572750_u32];
RET = [1696339860_u32,2412691552_u32,364512320_u32];
_13.0 = (-17449_i16);
_3 = 5988192052559661701451854206907197704_i128;
_6 = -(-9223372036854775808_isize);
_7.3 = true;
_14 = (_5,);
_13.1 = _6 < _6;
_9.0 = !_5;
_13 = ((-3183_i16), _7.3);
_1 = 58359_u16;
_3 = !169306395835862033179776799041744590388_i128;
_5 = !_14.0;
_9 = _14;
_2 = _8;
_11 = 92_u8;
_1 = 17591_u16 << _14.0;
RET = [408706453_u32,1258465441_u32,405763948_u32];
_7.1 = [185646972_u32,3073805760_u32,1085947557_u32];
Call(_10 = fn4(_1), ReturnTo(bb5), UnwindUnreachable())
}
bb16 = {
_22 = &_13;
_15 = ((-114_i8), _19);
_20 = _11;
_27 = _7.3 & _13.1;
_24.0 = (-1223301272_i32) as usize;
RET = [2311670053_u32,1009641062_u32,2572176553_u32];
_15.1 = _19 * _19;
RET = [1373698804_u32,3248601587_u32,1977381101_u32];
_5 = !_12;
_15.1 = -_19;
_12 = _14.0 ^ _5;
_12 = _5 & _5;
_26 = (_24.0,);
_4 = _7.1;
_20 = _27 as u8;
_14.0 = !_12;
RET = [1101427128_u32,3049886553_u32,1263112205_u32];
Goto(bb17)
}
bb17 = {
Call(_28 = dump_var(0_usize, 26_usize, Move(_26), 27_usize, Move(_27), 5_usize, Move(_5), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_28 = dump_var(0_usize, 4_usize, Move(_4), 8_usize, Move(_8), 11_usize, Move(_11), 18_usize, Move(_18)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_28 = dump_var(0_usize, 24_usize, Move(_24), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: char,mut _2: bool) -> u16 {
mir! {
type RET = u16;
let _3: [usize; 8];
let _4: f64;
let _5: u8;
let _6: &'static (Adt35, *mut &'static i8);
let _7: [char; 3];
let _8: *mut *const (Adt27, *const u64, [u8; 5]);
let _9: bool;
let _10: [u8; 5];
let _11: bool;
let _12: u128;
let _13: *mut *const &'static u8;
let _14: (usize,);
let _15: f64;
let _16: isize;
let _17: &'static char;
let _18: bool;
let _19: bool;
let _20: isize;
let _21: f32;
let _22: [u8; 5];
let _23: ();
let _24: ();
{
RET = (-37_i8) as u16;
_3 = [2181130340223628064_usize,5_usize,146405405716273753_usize,3365223839782538167_usize,4_usize,17120387405679837187_usize,1_usize,9499807943261564764_usize];
_3 = [3916388884616269932_usize,9858927119619087764_usize,3_usize,10835415017998258872_usize,7_usize,8060786626081963875_usize,4_usize,6_usize];
_1 = '\u{4bb2a}';
RET = 11909704572230001453_usize as u16;
RET = 4030076177303919224_u64 as u16;
RET = 55832_u16;
_2 = true ^ true;
_1 = '\u{845f2}';
_1 = '\u{59301}';
_2 = !true;
_1 = '\u{66141}';
_1 = '\u{de80a}';
_4 = (-73627089099186812039192611970521982605_i128) as f64;
_3 = [3423020151656151836_usize,2_usize,18367680997077334190_usize,0_usize,17873596949942879053_usize,6_usize,11455898360272021319_usize,12145572194951930428_usize];
_5 = 213_u8 + 44_u8;
_5 = !152_u8;
match RET {
55832 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_5 = 11920827788456550363_u64 as u8;
RET = (-12_isize) as u16;
RET = 1855409595_u32 as u16;
_3 = [10194405584994168707_usize,6_usize,8939876556169755307_usize,16510560200903942699_usize,2041598764280780365_usize,2_usize,4_usize,4205599518330949210_usize];
_5 = !120_u8;
_4 = 219784152153263631826672474854408259595_u128 as f64;
RET = 7321_u16 * 12804_u16;
_3 = [4_usize,11474677767203524499_usize,3873008700895690111_usize,4_usize,2_usize,0_usize,5_usize,7_usize];
_2 = _5 < _5;
_5 = !118_u8;
_3 = [5_usize,9380166261385780936_usize,3445815715388231434_usize,3_usize,3_usize,6_usize,7_usize,17508215841704590324_usize];
_5 = 3901092772096001565_i64 as u8;
RET = (-132495812847844611571819120768955489848_i128) as u16;
_1 = '\u{8e48f}';
_2 = _5 >= _5;
_4 = 5903800548095023108229756836260929338_u128 as f64;
_2 = !false;
_4 = 65_i8 as f64;
RET = 41633_u16 << _5;
_7 = [_1,_1,_1];
_7 = [_1,_1,_1];
_7 = [_1,_1,_1];
_7 = [_1,_1,_1];
_7 = [_1,_1,_1];
_7 = [_1,_1,_1];
_4 = (-38_i8) as f64;
_3 = [4_usize,4_usize,5_usize,13052004283537308765_usize,7_usize,2_usize,3_usize,7_usize];
Call(_4 = fn2(), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9 = _5 == _5;
_5 = _4 as u8;
RET = 28707_u16;
RET = _4 as u16;
RET = 19790_u16;
RET = _9 as u16;
_4 = (-9223372036854775808_isize) as f64;
_4 = _5 as f64;
RET = 62689_u16 ^ 59149_u16;
_2 = !_9;
_10 = [_5,_5,_5,_5,_5];
_10 = [_5,_5,_5,_5,_5];
_9 = _2 & _2;
_4 = 8292652392792207434_u64 as f64;
_2 = !_9;
_10 = [_5,_5,_5,_5,_5];
_4 = (-1617074921_i32) as f64;
_7 = [_1,_1,_1];
Goto(bb4)
}
bb4 = {
_4 = (-37_isize) as f64;
_2 = _9;
_11 = _2;
_3 = [4773549376453824091_usize,4_usize,14196014436991562982_usize,7_usize,5_usize,1_usize,3_usize,7_usize];
_4 = 324741059125033619780507246570108972623_u128 as f64;
_1 = '\u{cf882}';
_5 = 118_u8 & 158_u8;
Goto(bb5)
}
bb5 = {
_12 = 290402351408550779638854115428500563110_u128;
_4 = (-716921417198482099_i64) as f64;
_5 = !5_u8;
_1 = '\u{c7e0b}';
RET = _4 as u16;
_11 = _2 & _2;
_14.0 = 14588067670084192230_usize;
_12 = 68876272175887873955436038330952383320_u128;
RET = 3645_u16 - 61540_u16;
_10 = [_5,_5,_5,_5,_5];
_5 = 175_u8;
RET = !9245_u16;
_1 = '\u{89566}';
RET = !20301_u16;
RET = !35203_u16;
_10 = [_5,_5,_5,_5,_5];
_15 = _14.0 as f64;
_15 = _4;
_5 = 163_u8 << RET;
RET = !58447_u16;
_7 = [_1,_1,_1];
_11 = _2;
_1 = '\u{ab6fc}';
_7 = [_1,_1,_1];
_15 = _4;
_3 = [_14.0,_14.0,_14.0,_14.0,_14.0,_14.0,_14.0,_14.0];
match _12 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
68876272175887873955436038330952383320 => bb13,
_ => bb12
}
}
bb6 = {
_4 = (-37_isize) as f64;
_2 = _9;
_11 = _2;
_3 = [4773549376453824091_usize,4_usize,14196014436991562982_usize,7_usize,5_usize,1_usize,3_usize,7_usize];
_4 = 324741059125033619780507246570108972623_u128 as f64;
_1 = '\u{cf882}';
_5 = 118_u8 & 158_u8;
Goto(bb5)
}
bb7 = {
_9 = _5 == _5;
_5 = _4 as u8;
RET = 28707_u16;
RET = _4 as u16;
RET = 19790_u16;
RET = _9 as u16;
_4 = (-9223372036854775808_isize) as f64;
_4 = _5 as f64;
RET = 62689_u16 ^ 59149_u16;
_2 = !_9;
_10 = [_5,_5,_5,_5,_5];
_10 = [_5,_5,_5,_5,_5];
_9 = _2 & _2;
_4 = 8292652392792207434_u64 as f64;
_2 = !_9;
_10 = [_5,_5,_5,_5,_5];
_4 = (-1617074921_i32) as f64;
_7 = [_1,_1,_1];
Goto(bb4)
}
bb8 = {
_5 = 11920827788456550363_u64 as u8;
RET = (-12_isize) as u16;
RET = 1855409595_u32 as u16;
_3 = [10194405584994168707_usize,6_usize,8939876556169755307_usize,16510560200903942699_usize,2041598764280780365_usize,2_usize,4_usize,4205599518330949210_usize];
_5 = !120_u8;
_4 = 219784152153263631826672474854408259595_u128 as f64;
RET = 7321_u16 * 12804_u16;
_3 = [4_usize,11474677767203524499_usize,3873008700895690111_usize,4_usize,2_usize,0_usize,5_usize,7_usize];
_2 = _5 < _5;
_5 = !118_u8;
_3 = [5_usize,9380166261385780936_usize,3445815715388231434_usize,3_usize,3_usize,6_usize,7_usize,17508215841704590324_usize];
_5 = 3901092772096001565_i64 as u8;
RET = (-132495812847844611571819120768955489848_i128) as u16;
_1 = '\u{8e48f}';
_2 = _5 >= _5;
_4 = 5903800548095023108229756836260929338_u128 as f64;
_2 = !false;
_4 = 65_i8 as f64;
RET = 41633_u16 << _5;
_7 = [_1,_1,_1];
_7 = [_1,_1,_1];
_7 = [_1,_1,_1];
_7 = [_1,_1,_1];
_7 = [_1,_1,_1];
_7 = [_1,_1,_1];
_4 = (-38_i8) as f64;
_3 = [4_usize,4_usize,5_usize,13052004283537308765_usize,7_usize,2_usize,3_usize,7_usize];
Call(_4 = fn2(), ReturnTo(bb3), UnwindUnreachable())
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
_3 = [_14.0,_14.0,_14.0,_14.0,_14.0,_14.0,_14.0,_14.0];
_12 = RET as u128;
_2 = !_9;
_17 = &_1;
_2 = !_11;
_1 = '\u{6000c}';
RET = 7931342573095898970_u64 as u16;
_18 = _2 & _9;
_7 = [_1,_1,_1];
_12 = 303505264541364822580973411262105898211_u128;
RET = 56365_u16;
_17 = &_1;
_4 = _15 - _15;
_4 = -_15;
_2 = _18 <= _11;
RET = 45242_u16 & 3883_u16;
_16 = 9223372036854775807_isize - 47_isize;
_12 = _16 as u128;
_14 = (3_usize,);
match _14.0 {
0 => bb1,
1 => bb6,
2 => bb14,
4 => bb16,
5 => bb17,
3 => bb19,
_ => bb18
}
}
bb14 = {
Return()
}
bb15 = {
_4 = (-37_isize) as f64;
_2 = _9;
_11 = _2;
_3 = [4773549376453824091_usize,4_usize,14196014436991562982_usize,7_usize,5_usize,1_usize,3_usize,7_usize];
_4 = 324741059125033619780507246570108972623_u128 as f64;
_1 = '\u{cf882}';
_5 = 118_u8 & 158_u8;
Goto(bb5)
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
_5 = 11920827788456550363_u64 as u8;
RET = (-12_isize) as u16;
RET = 1855409595_u32 as u16;
_3 = [10194405584994168707_usize,6_usize,8939876556169755307_usize,16510560200903942699_usize,2041598764280780365_usize,2_usize,4_usize,4205599518330949210_usize];
_5 = !120_u8;
_4 = 219784152153263631826672474854408259595_u128 as f64;
RET = 7321_u16 * 12804_u16;
_3 = [4_usize,11474677767203524499_usize,3873008700895690111_usize,4_usize,2_usize,0_usize,5_usize,7_usize];
_2 = _5 < _5;
_5 = !118_u8;
_3 = [5_usize,9380166261385780936_usize,3445815715388231434_usize,3_usize,3_usize,6_usize,7_usize,17508215841704590324_usize];
_5 = 3901092772096001565_i64 as u8;
RET = (-132495812847844611571819120768955489848_i128) as u16;
_1 = '\u{8e48f}';
_2 = _5 >= _5;
_4 = 5903800548095023108229756836260929338_u128 as f64;
_2 = !false;
_4 = 65_i8 as f64;
RET = 41633_u16 << _5;
_7 = [_1,_1,_1];
_7 = [_1,_1,_1];
_7 = [_1,_1,_1];
_7 = [_1,_1,_1];
_7 = [_1,_1,_1];
_7 = [_1,_1,_1];
_4 = (-38_i8) as f64;
_3 = [4_usize,4_usize,5_usize,13052004283537308765_usize,7_usize,2_usize,3_usize,7_usize];
Call(_4 = fn2(), ReturnTo(bb3), UnwindUnreachable())
}
bb19 = {
_1 = '\u{940b8}';
_16 = _5 as isize;
_16 = (-9223372036854775808_isize) - 126_isize;
_3 = [_14.0,_14.0,_14.0,_14.0,_14.0,_14.0,_14.0,_14.0];
_11 = _2 & _2;
_3 = [_14.0,_14.0,_14.0,_14.0,_14.0,_14.0,_14.0,_14.0];
_4 = _15 - _15;
_20 = -_16;
_20 = _16 | _16;
_2 = _11 > _11;
_21 = _5 as f32;
_19 = _2 <= _2;
_9 = _11 > _18;
Goto(bb20)
}
bb20 = {
Call(_23 = dump_var(1_usize, 10_usize, Move(_10), 18_usize, Move(_18), 14_usize, Move(_14), 12_usize, Move(_12)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_23 = dump_var(1_usize, 19_usize, Move(_19), 16_usize, Move(_16), 9_usize, Move(_9), 24_usize, _24), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2() -> f64 {
mir! {
type RET = f64;
let _1: u64;
let _2: f32;
let _3: (i8, f64);
let _4: &'static &'static i8;
let _5: i128;
let _6: &'static u8;
let _7: &'static i128;
let _8: ([u16; 7], &'static i128);
let _9: ((i32,), (i32,), u32, (&'static &'static i8, [u32; 3], [u32; 3], bool));
let _10: u64;
let _11: i16;
let _12: f32;
let _13: f32;
let _14: isize;
let _15: Adt42;
let _16: Adt27;
let _17: [usize; 8];
let _18: &'static (i16, bool);
let _19: u16;
let _20: ();
let _21: ();
{
RET = 108_u8 as f64;
RET = 9223372036854775807_isize as f64;
RET = (-94_i8) as f64;
RET = (-9223372036854775808_isize) as f64;
_2 = 326993852309180721_i64 as f32;
_1 = 5561136388557381695_u64 | 15720742179263749090_u64;
_1 = 25382479193129779685869783636529123120_i128 as u64;
_1 = 18181978752853981061_u64;
_2 = (-8194_i16) as f32;
_1 = 14408905046014207468_u64;
RET = 1915825776_u32 as f64;
_2 = RET as f32;
RET = _2 as f64;
RET = 29017_u16 as f64;
_1 = !16991838092461881019_u64;
_3 = ((-59_i8), RET);
_3 = ((-9_i8), RET);
RET = -_3.1;
match _3.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463463374607431768211447 => bb8,
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
_1 = 11365017413451419021_u64 ^ 14855629736362155311_u64;
_3.1 = RET - RET;
RET = -_3.1;
_3 = (52_i8, RET);
_3 = ((-37_i8), RET);
_1 = 8453452761714447455_u64 | 15351542706970343144_u64;
_3.1 = -RET;
_3 = ((-47_i8), RET);
_3.1 = RET - RET;
_3.0 = (-63_i8) * (-96_i8);
_3.0 = _3.1 as i8;
_3.1 = RET + RET;
_3.1 = 55564_u16 as f64;
RET = (-32189_i16) as f64;
_3 = (50_i8, RET);
RET = -_3.1;
RET = -_3.1;
_2 = 208_u8 as f32;
_5 = -(-157431719720330855266292740965564157660_i128);
_3.0 = (-881746381_i32) as i8;
RET = _3.0 as f64;
_3.1 = -RET;
_5 = (-75547370812845899398116682486824870226_i128) * (-51306064031872076560042730027681656889_i128);
Call(_1 = core::intrinsics::bswap(15508568672678064728_u64), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_3 = (103_i8, RET);
_8.1 = &_5;
_9.0.0 = !(-841324534_i32);
_3.1 = 161_u8 as f64;
_11 = !(-7513_i16);
_5 = (-9223372036854775808_isize) as i128;
_1 = !7602952911980182194_u64;
_9.1.0 = _5 as i32;
_10 = !_1;
_9.3.2 = [1789079022_u32,1323071182_u32,136993324_u32];
_9.2 = 623033463_u32 - 335384881_u32;
_8.0 = [3156_u16,8993_u16,42792_u16,43368_u16,41563_u16,54489_u16,57712_u16];
_8.1 = &_5;
_9.3.3 = _5 != _5;
_9.3.3 = !true;
RET = _3.1;
_7 = &_5;
_5 = 136004580660874289085792941466241542120_i128 >> _1;
_8.1 = &_5;
Call(_9.3.3 = fn3(Move(_8)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_9.0 = _9.1;
_9.3.3 = !false;
_9.3.2 = [_9.2,_9.2,_9.2];
match _3.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb9,
6 => bb7,
103 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
RET = _3.1 * _3.1;
RET = _3.1 * _3.1;
_14 = -(-81_isize);
_8.1 = &_5;
_2 = _1 as f32;
_9.3.2 = [_9.2,_9.2,_9.2];
_9.1.0 = _9.0.0;
_3.1 = RET;
_7 = &_5;
_9.0 = (_9.1.0,);
_9.3.1 = [_9.2,_9.2,_9.2];
_9.0 = (_9.1.0,);
_13 = -_2;
_3.0 = -(-1_i8);
_12 = _2;
_14 = 9223372036854775807_isize | 72_isize;
_9.0 = _9.1;
_9.3.2 = [_9.2,_9.2,_9.2];
_9.1.0 = _9.0.0 << _3.0;
_9.1.0 = _9.0.0;
_3 = ((-36_i8), RET);
_3.1 = _3.0 as f64;
_10 = _1 & _1;
_7 = &_5;
_8.1 = &(*_7);
_1 = _10 ^ _10;
match _3.0 {
0 => bb3,
340282366920938463463374607431768211420 => bb13,
_ => bb9
}
}
bb13 = {
_3 = (126_i8, RET);
_3.0 = !103_i8;
Goto(bb14)
}
bb14 = {
_9.3.3 = !true;
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(2_usize, 11_usize, Move(_11), 14_usize, Move(_14), 21_usize, _21, 21_usize, _21), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: ([u16; 7], &'static i128)) -> bool {
mir! {
type RET = bool;
let _2: &'static char;
let _3: isize;
let _4: f32;
let _5: f64;
let _6: u8;
let _7: f32;
let _8: f64;
let _9: i64;
let _10: Adt52;
let _11: ((*mut char,), &'static u8, *mut char, &'static &'static i8);
let _12: i8;
let _13: isize;
let _14: [u16; 7];
let _15: &'static i128;
let _16: [char; 3];
let _17: *const usize;
let _18: [u8; 5];
let _19: (f64, u8, (Adt35, *mut &'static i8), isize);
let _20: char;
let _21: bool;
let _22: *mut (*mut &'static i8,);
let _23: f32;
let _24: &'static i8;
let _25: [u16; 1];
let _26: *const &'static u8;
let _27: (*mut (*mut &'static i8,), u16);
let _28: (*const u64, (i8, f64), [u16; 7], bool);
let _29: isize;
let _30: u128;
let _31: &'static *mut [u16; 7];
let _32: i128;
let _33: i32;
let _34: f32;
let _35: f64;
let _36: u128;
let _37: isize;
let _38: ();
let _39: ();
{
RET = !false;
RET = 28348_i16 < 8336_i16;
RET = !true;
RET = false;
RET = 8716808897344064970_i64 <= (-4301028909674130323_i64);
_1.0 = [28181_u16,27933_u16,15395_u16,62086_u16,25314_u16,29936_u16,26783_u16];
RET = true;
_3 = (-9223372036854775808_isize);
RET = _3 != _3;
_4 = 3061329623166907980_usize as f32;
RET = true;
_5 = (-1430518207839269403_i64) as f64;
_3 = 2_isize & 9223372036854775807_isize;
RET = !false;
_3 = 9223372036854775807_isize | (-81_isize);
_4 = (-13_i8) as f32;
_5 = 68_u8 as f64;
_4 = 119827248485191958462218312897578342263_i128 as f32;
_4 = 7370958531503445165_i64 as f32;
_6 = 238_u8 & 194_u8;
RET = _4 < _4;
Goto(bb1)
}
bb1 = {
RET = !true;
RET = !true;
_3 = 9223372036854775807_isize >> _6;
_5 = 87637552101791291110526147495331126659_i128 as f64;
Goto(bb2)
}
bb2 = {
_6 = 94_u8 & 64_u8;
RET = false & false;
_7 = -_4;
RET = _4 == _4;
_1.0 = [50066_u16,7010_u16,59881_u16,53907_u16,34998_u16,55878_u16,19568_u16];
_8 = _5;
_1.0 = [2994_u16,25580_u16,49504_u16,32277_u16,39373_u16,10106_u16,28754_u16];
_6 = 174_u8 + 126_u8;
_7 = _4;
_8 = 266339040490929246543238694719193499295_u128 as f64;
_7 = -_4;
_5 = -_8;
_8 = _3 as f64;
_9 = -2916023146861435662_i64;
RET = !false;
RET = !false;
_3 = !(-11_isize);
_3 = (-9223372036854775808_isize) + 9223372036854775807_isize;
Goto(bb3)
}
bb3 = {
_8 = _6 as f64;
Goto(bb4)
}
bb4 = {
_8 = _5 + _5;
_7 = _4 - _4;
RET = !true;
_8 = _9 as f64;
_3 = 144190340464877770615569596099164844278_i128 as isize;
_8 = _5;
_5 = (-17654353343966432103154590192757087492_i128) as f64;
_1.0 = [46650_u16,42858_u16,55310_u16,12762_u16,58643_u16,34631_u16,27792_u16];
_7 = _4 - _4;
_10 = Adt52::Variant1 { fld0: RET };
_6 = !92_u8;
place!(Field::<bool>(Variant(_10, 1), 0)) = RET ^ RET;
_8 = _5;
_7 = (-1098457522_i32) as f32;
_7 = -_4;
_1.0 = [38304_u16,404_u16,52558_u16,61058_u16,65269_u16,59564_u16,42258_u16];
_9 = !(-6680966570960723624_i64);
_10 = Adt52::Variant1 { fld0: RET };
_1.0 = [33049_u16,40141_u16,49451_u16,923_u16,955_u16,52631_u16,34188_u16];
_3 = !9223372036854775807_isize;
_5 = -_8;
_6 = 24_u8;
_3 = (-35_isize);
SetDiscriminant(_10, 1);
place!(Field::<bool>(Variant(_10, 1), 0)) = RET;
RET = Field::<bool>(Variant(_10, 1), 0) | Field::<bool>(Variant(_10, 1), 0);
_1.0 = [35020_u16,12337_u16,9876_u16,54755_u16,7767_u16,11850_u16,35272_u16];
match _6 {
0 => bb5,
24 => bb7,
_ => bb6
}
}
bb5 = {
RET = !true;
RET = !true;
_3 = 9223372036854775807_isize >> _6;
_5 = 87637552101791291110526147495331126659_i128 as f64;
Goto(bb2)
}
bb6 = {
_6 = 94_u8 & 64_u8;
RET = false & false;
_7 = -_4;
RET = _4 == _4;
_1.0 = [50066_u16,7010_u16,59881_u16,53907_u16,34998_u16,55878_u16,19568_u16];
_8 = _5;
_1.0 = [2994_u16,25580_u16,49504_u16,32277_u16,39373_u16,10106_u16,28754_u16];
_6 = 174_u8 + 126_u8;
_7 = _4;
_8 = 266339040490929246543238694719193499295_u128 as f64;
_7 = -_4;
_5 = -_8;
_8 = _3 as f64;
_9 = -2916023146861435662_i64;
RET = !false;
RET = !false;
_3 = !(-11_isize);
_3 = (-9223372036854775808_isize) + 9223372036854775807_isize;
Goto(bb3)
}
bb7 = {
_9 = 1171381626861881595_i64 << _6;
_1.0 = [42091_u16,59877_u16,51283_u16,34877_u16,10309_u16,43470_u16,38655_u16];
_12 = -106_i8;
_7 = -_4;
place!(Field::<bool>(Variant(_10, 1), 0)) = !RET;
_7 = _5 as f32;
_6 = 3828_i16 as u8;
_14 = [55920_u16,10199_u16,64698_u16,15160_u16,56267_u16,55999_u16,1551_u16];
_11.1 = &_6;
Goto(bb8)
}
bb8 = {
_14 = [48653_u16,56463_u16,3080_u16,3121_u16,42293_u16,3433_u16,6229_u16];
_16 = ['\u{7da3d}','\u{fe2dc}','\u{9577c}'];
_9 = (-6221533848271036499_i64);
_19.3 = -_3;
_10 = Adt52::Variant1 { fld0: RET };
_11.2 = core::ptr::addr_of_mut!(_20);
_5 = -_8;
_1.0 = [2210_u16,14041_u16,59077_u16,22528_u16,22542_u16,18568_u16,62907_u16];
_1.0 = _14;
_21 = !RET;
_9 = (-17601_i16) as i64;
_8 = 7276_u16 as f64;
_7 = _4 - _4;
Goto(bb9)
}
bb9 = {
_14 = [29501_u16,41622_u16,26158_u16,13542_u16,7608_u16,58810_u16,18466_u16];
_6 = !242_u8;
_11.1 = &_6;
_11.0.0 = core::ptr::addr_of_mut!(_20);
_5 = _8;
_19.2.1 = core::ptr::addr_of_mut!(_24);
_19.1 = 216_i16 as u8;
_11.1 = &_19.1;
_24 = &_12;
RET = _21;
_13 = _19.3;
_19.3 = _3 + _3;
_11.3 = &_24;
_14 = [27231_u16,48016_u16,40038_u16,9141_u16,33992_u16,540_u16,15983_u16];
_3 = _19.3 & _19.3;
place!(Field::<bool>(Variant(_10, 1), 0)) = _13 > _3;
RET = _21;
_5 = _8 + _8;
_11.0.0 = core::ptr::addr_of_mut!(_20);
_19.2.1 = core::ptr::addr_of_mut!(_24);
Goto(bb10)
}
bb10 = {
_11.3 = &_24;
_4 = 8458_i16 as f32;
_11.1 = &_6;
_14 = [58615_u16,5620_u16,34598_u16,40664_u16,44105_u16,49622_u16,16878_u16];
_12 = !97_i8;
_13 = _3;
_5 = -_8;
_11.3 = &_24;
_6 = '\u{17fad}' as u8;
_5 = -_8;
_19.1 = (-5948415019141590123826761206001887713_i128) as u8;
_26 = core::ptr::addr_of!(_11.1);
Goto(bb11)
}
bb11 = {
(*_26) = &_19.1;
_13 = -_3;
_19.2.1 = core::ptr::addr_of_mut!(_24);
_11.3 = &_24;
_19.3 = -_13;
_7 = _4 + _4;
_11.1 = &_19.1;
_19.0 = 3_usize as f64;
_2 = &_20;
_13 = _19.0 as isize;
_19.3 = _4 as isize;
_25 = [19482_u16];
_25 = [55177_u16];
_21 = !Field::<bool>(Variant(_10, 1), 0);
_23 = _4 + _4;
_9 = _3 as i64;
_16 = ['\u{64e45}','\u{ff20b}','\u{a72a2}'];
_27.1 = !23423_u16;
_27.1 = 25798_u16 ^ 41884_u16;
_2 = &(*_2);
_8 = _19.0;
Goto(bb12)
}
bb12 = {
RET = _21 == _21;
_12 = 80_i8;
_1.0 = [_27.1,_27.1,_27.1,_27.1,_27.1,_27.1,_27.1];
_23 = -_7;
_1.0 = [_27.1,_27.1,_27.1,_27.1,_27.1,_27.1,_27.1];
_1.0 = _14;
_20 = '\u{5f0ce}';
_12 = 93_i8;
_19.3 = _3;
_27.1 = 20635_u16 << _9;
_19.0 = _7 as f64;
_27.1 = !53257_u16;
_3 = !_13;
(*_26) = &_6;
_8 = _19.0 + _19.0;
_28.1 = (_12, _8);
RET = Field::<bool>(Variant(_10, 1), 0);
_9 = _27.1 as i64;
Goto(bb13)
}
bb13 = {
_11.2 = Move(_11.0.0);
place!(Field::<bool>(Variant(_10, 1), 0)) = RET ^ RET;
_6 = !_19.1;
_28.1 = (_12, _19.0);
_5 = _28.1.1;
_29 = 289028486944872877850573949472724594436_u128 as isize;
_30 = !47150707787488121190240261427875573407_u128;
SetDiscriminant(_10, 0);
_19.1 = 21488_i16 as u8;
place!(Field::<*const Adt35>(Variant(_10, 0), 1)) = core::ptr::addr_of!(_19.2.0);
_11.3 = &_24;
_33 = (-180499086_i32) + (-1902667741_i32);
_11.0 = (Move(_11.2),);
_11.2 = Move(_11.0.0);
_32 = 64823299170701043554610748478189410322_i128;
_32 = _21 as i128;
_23 = -_7;
_33 = _30 as i32;
_28.3 = _30 < _30;
_15 = &_32;
_24 = &_28.1.0;
_32 = (-93070016056511578540641631222260399846_i128) >> _30;
_29 = !_13;
place!(Field::<(i16, bool)>(Variant(_10, 0), 5)) = ((-30617_i16), _21);
_2 = &_20;
_11.3 = &_24;
_11.0.0 = core::ptr::addr_of_mut!(_20);
Goto(bb14)
}
bb14 = {
_28.3 = Field::<(i16, bool)>(Variant(_10, 0), 5).1 < Field::<(i16, bool)>(Variant(_10, 0), 5).1;
_28.1.1 = _8;
place!(Field::<u16>(Variant(_10, 0), 3)) = _19.1 as u16;
_6 = !_19.1;
place!(Field::<(i16, bool)>(Variant(_10, 0), 5)) = (7155_i16, _28.3);
_11.1 = &_6;
_2 = &(*_2);
_28.2 = [_27.1,Field::<u16>(Variant(_10, 0), 3),Field::<u16>(Variant(_10, 0), 3),Field::<u16>(Variant(_10, 0), 3),_27.1,_27.1,Field::<u16>(Variant(_10, 0), 3)];
_21 = _28.3 ^ _28.3;
RET = _21 != _21;
_29 = _19.3 ^ _19.3;
_9 = -7557011505849686972_i64;
place!(Field::<(i16, bool)>(Variant(_10, 0), 5)).1 = RET ^ RET;
_26 = core::ptr::addr_of!((*_26));
_16 = [(*_2),_20,(*_2)];
_28.1.1 = _5 * _8;
(*_26) = &_6;
_28.1 = (_12, _8);
_1.1 = &_32;
_35 = _8;
place!(Field::<(i16, bool)>(Variant(_10, 0), 5)) = (5975_i16, RET);
_28.2 = [Field::<u16>(Variant(_10, 0), 3),_27.1,_27.1,Field::<u16>(Variant(_10, 0), 3),_27.1,Field::<u16>(Variant(_10, 0), 3),Field::<u16>(Variant(_10, 0), 3)];
_2 = &(*_2);
_5 = _19.0 * _8;
_15 = &_32;
_2 = &(*_2);
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(3_usize, 14_usize, Move(_14), 33_usize, Move(_33), 32_usize, Move(_32), 29_usize, Move(_29)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(3_usize, 25_usize, Move(_25), 3_usize, Move(_3), 13_usize, Move(_13), 39_usize, _39), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: u16) -> *const (Adt27, *const u64, [u8; 5]) {
mir! {
type RET = *const (Adt27, *const u64, [u8; 5]);
let _2: *const u64;
let _3: *mut *const &'static u8;
let _4: f32;
let _5: u16;
let _6: (f64, u8, (Adt35, *mut &'static i8), isize);
let _7: i8;
let _8: [usize; 8];
let _9: i16;
let _10: [u16; 1];
let _11: (isize, u32, (Adt44, [u32; 3]), usize);
let _12: [isize; 4];
let _13: char;
let _14: i32;
let _15: Adt59;
let _16: isize;
let _17: isize;
let _18: (i16, bool);
let _19: i64;
let _20: *mut char;
let _21: *const *mut [u16; 7];
let _22: u8;
let _23: ((*mut char,), &'static u8, *mut char, &'static &'static i8);
let _24: ((usize,), &'static i128, *mut bool, (Adt27, *const u64, [u8; 5]));
let _25: *mut char;
let _26: ();
let _27: ();
{
_1 = !1322_u16;
Goto(bb1)
}
bb1 = {
_1 = !64901_u16;
_1 = 39740_u16;
_1 = !31909_u16;
_1 = 11132_u16 * 22647_u16;
_1 = (-9223372036854775808_isize) as u16;
_1 = 27409_u16;
_1 = 48452_u16 | 44785_u16;
_1 = !25414_u16;
_1 = !52756_u16;
_1 = false as u16;
_4 = 115595639626109251406943138570724204946_u128 as f32;
_4 = 17893536855668337235_usize as f32;
Call(RET = fn5(_4, _4, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = _1;
_1 = _5 >> _5;
_6.1 = 177_u8;
_6.1 = !199_u8;
_6.1 = !18_u8;
_6.3 = (-9223372036854775808_isize);
_6.0 = _6.3 as f64;
_7 = !103_i8;
_6.1 = 84_u8;
_5 = _1;
_6.3 = (-9223372036854775808_isize) * 84_isize;
_6.0 = 169714569972969142566174097762582174117_u128 as f64;
Goto(bb3)
}
bb3 = {
_10 = [_5];
_8 = [1_usize,0_usize,7_usize,17638550761471911792_usize,5_usize,12358760344033397080_usize,9352140228483375812_usize,6_usize];
_5 = !_1;
_9 = -(-20227_i16);
_6.0 = _6.1 as f64;
_1 = 3_usize as u16;
_7 = (-27_i8);
_1 = _5 | _5;
_1 = _5 + _5;
_5 = _9 as u16;
_11.0 = _6.3 & _6.3;
match _6.1 {
0 => bb1,
84 => bb5,
_ => bb4
}
}
bb4 = {
_5 = _1;
_1 = _5 >> _5;
_6.1 = 177_u8;
_6.1 = !199_u8;
_6.1 = !18_u8;
_6.3 = (-9223372036854775808_isize);
_6.0 = _6.3 as f64;
_7 = !103_i8;
_6.1 = 84_u8;
_5 = _1;
_6.3 = (-9223372036854775808_isize) * 84_isize;
_6.0 = 169714569972969142566174097762582174117_u128 as f64;
Goto(bb3)
}
bb5 = {
_11.3 = 1_usize & 17622548432297043099_usize;
_5 = !_1;
_12 = [_11.0,_11.0,_11.0,_11.0];
_6.3 = _11.0 - _11.0;
_6.0 = _7 as f64;
match _7 {
0 => bb3,
1 => bb2,
2 => bb6,
3 => bb7,
340282366920938463463374607431768211429 => bb9,
_ => bb8
}
}
bb6 = {
_5 = _1;
_1 = _5 >> _5;
_6.1 = 177_u8;
_6.1 = !199_u8;
_6.1 = !18_u8;
_6.3 = (-9223372036854775808_isize);
_6.0 = _6.3 as f64;
_7 = !103_i8;
_6.1 = 84_u8;
_5 = _1;
_6.3 = (-9223372036854775808_isize) * 84_isize;
_6.0 = 169714569972969142566174097762582174117_u128 as f64;
Goto(bb3)
}
bb7 = {
_10 = [_5];
_8 = [1_usize,0_usize,7_usize,17638550761471911792_usize,5_usize,12358760344033397080_usize,9352140228483375812_usize,6_usize];
_5 = !_1;
_9 = -(-20227_i16);
_6.0 = _6.1 as f64;
_1 = 3_usize as u16;
_7 = (-27_i8);
_1 = _5 | _5;
_1 = _5 + _5;
_5 = _9 as u16;
_11.0 = _6.3 & _6.3;
match _6.1 {
0 => bb1,
84 => bb5,
_ => bb4
}
}
bb8 = {
_1 = !64901_u16;
_1 = 39740_u16;
_1 = !31909_u16;
_1 = 11132_u16 * 22647_u16;
_1 = (-9223372036854775808_isize) as u16;
_1 = 27409_u16;
_1 = 48452_u16 | 44785_u16;
_1 = !25414_u16;
_1 = !52756_u16;
_1 = false as u16;
_4 = 115595639626109251406943138570724204946_u128 as f32;
_4 = 17893536855668337235_usize as f32;
Call(RET = fn5(_4, _4, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb9 = {
_5 = _1 - _1;
_6.1 = !84_u8;
_8 = [_11.3,_11.3,_11.3,_11.3,_11.3,_11.3,_11.3,_11.3];
_11.2.1 = [3772370224_u32,340236869_u32,957818497_u32];
_6.3 = _11.0;
_7 = 76_i8;
_11.0 = !_6.3;
_11.2.1 = [2443020973_u32,1198671632_u32,3725380517_u32];
_5 = _1;
_4 = _9 as f32;
_10 = [_1];
_9 = true as i16;
_1 = _9 as u16;
_5 = _1;
_6.0 = _6.3 as f64;
_5 = _1;
_12 = [_6.3,_11.0,_11.0,_6.3];
_7 = true as i8;
Goto(bb10)
}
bb10 = {
_15.fld2 = _6.1 as u128;
_11.2.1 = [1316621239_u32,185603412_u32,3160374345_u32];
_6.3 = !_11.0;
_15.fld0 = _11.3;
_11.0 = _15.fld2 as isize;
_6.1 = 2416485692_u32 as u8;
_11.3 = _7 as usize;
_6.3 = !_11.0;
_11.1 = 4275568748_u32 - 882987341_u32;
_15.fld3 = _11.3 as f64;
_11.1 = !3055212693_u32;
_11.0 = _6.3;
_4 = 3669858824783882297_u64 as f32;
Goto(bb11)
}
bb11 = {
_12 = [_6.3,_6.3,_11.0,_11.0];
_16 = _15.fld0 as isize;
_16 = _6.3 ^ _11.0;
_6.0 = _15.fld3;
_18.0 = _9;
_9 = _18.0 & _18.0;
_15.fld3 = -_6.0;
_13 = '\u{18198}';
_11.3 = _15.fld0;
_17 = -_16;
_14 = -(-1731762846_i32);
_11.0 = _4 as isize;
_8 = [_11.3,_15.fld0,_15.fld0,_15.fld0,_11.3,_11.3,_11.3,_15.fld0];
_11.2.1 = [_11.1,_11.1,_11.1];
_18 = (_9, false);
_1 = _6.0 as u16;
_13 = '\u{e9dce}';
_14 = (-6097198882568348183_i64) as i32;
Goto(bb12)
}
bb12 = {
_18 = (_9, false);
_17 = _11.1 as isize;
_6.3 = _16;
_18.0 = !_9;
_18 = (_9, true);
_14 = !(-615583255_i32);
_6.3 = _16;
_5 = _1 | _1;
_19 = 8896956628024173185_i64 << _5;
_11.1 = !1423470_u32;
_14 = (-1523918376_i32) >> _1;
_15.fld1 = core::ptr::addr_of_mut!(_18.1);
_15.fld2 = !121901173463381377903017653430209522741_u128;
_16 = -_6.3;
_18 = (_9, true);
_20 = core::ptr::addr_of_mut!(_13);
Goto(bb13)
}
bb13 = {
_12 = [_16,_17,_16,_6.3];
_4 = _16 as f32;
_15.fld0 = _11.3;
_18.0 = 366451170758600815_u64 as i16;
_12 = [_6.3,_16,_6.3,_6.3];
_15.fld0 = _11.3;
Call(_12 = fn17(_17, _18.1, Move(_15), _18.1, _16, _17, _11.1, _19, _10, _4), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_22 = _6.1;
_11.0 = _9 as isize;
_1 = _5;
_11.2.1 = [_11.1,_11.1,_11.1];
_18 = (_9, true);
(*_20) = '\u{88f8a}';
_19 = -(-1754559919170146408_i64);
_23.1 = &_22;
_6.0 = _6.1 as f64;
_15.fld0 = !_11.3;
_24.3.2 = [_22,_6.1,_6.1,_22,_6.1];
_15.fld0 = _11.3;
_1 = _19 as u16;
_15.fld2 = !258376869811036997794102662030582582790_u128;
_4 = _1 as f32;
(*_20) = '\u{1739b}';
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(4_usize, 10_usize, Move(_10), 13_usize, Move(_13), 16_usize, Move(_16), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(4_usize, 9_usize, Move(_9), 1_usize, Move(_1), 22_usize, Move(_22), 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: f32,mut _2: f32,mut _3: u16) -> *const (Adt27, *const u64, [u8; 5]) {
mir! {
type RET = *const (Adt27, *const u64, [u8; 5]);
let _4: u16;
let _5: u128;
let _6: char;
let _7: [usize; 2];
let _8: [u8; 1];
let _9: *const (Adt27, *const u64, [u8; 5]);
let _10: [char; 3];
let _11: (i8, f64);
let _12: [u8; 1];
let _13: bool;
let _14: [u8; 1];
let _15: ([u16; 7], &'static i128);
let _16: i8;
let _17: [usize; 2];
let _18: [usize; 2];
let _19: isize;
let _20: i8;
let _21: [char; 3];
let _22: isize;
let _23: i64;
let _24: [u32; 3];
let _25: &'static (*mut char,);
let _26: *const u64;
let _27: &'static (f64, u8, (Adt35, *mut &'static i8), isize);
let _28: f64;
let _29: f64;
let _30: isize;
let _31: ((i32,), (i32,), u32, (&'static &'static i8, [u32; 3], [u32; 3], bool));
let _32: f64;
let _33: &'static i8;
let _34: ((Adt27, *const u64, [u8; 5]), &'static *const usize, (Adt35, *mut &'static i8), *const usize);
let _35: ();
let _36: ();
{
_1 = _2;
_1 = _2;
_1 = (-33_i8) as f32;
_3 = 9005106366805495288_u64 as u16;
_3 = 32401_u16;
_1 = _2;
_5 = 1501243327_u32 as u128;
_5 = 127284032589793653094692072658978646443_u128;
_7 = [1_usize,4_usize];
_6 = '\u{64bda}';
_4 = !_3;
_7 = [10158037915791881566_usize,3_usize];
_1 = -_2;
_5 = 611189437_i32 as u128;
_4 = !_3;
_1 = -_2;
_2 = -_1;
_2 = -_1;
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
32401 => bb9,
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
_8 = [38_u8];
_7 = [5_usize,0_usize];
_7 = [8100610056965360845_usize,0_usize];
_6 = '\u{27f22}';
_4 = _3 & _3;
_2 = _1;
_7 = [3656315817012154362_usize,7521264686506186067_usize];
_8 = [225_u8];
_10 = [_6,_6,_6];
_4 = !_3;
_8 = [230_u8];
_2 = 693001736_i32 as f32;
_7 = [5_usize,5_usize];
_10 = [_6,_6,_6];
_4 = _3;
_10 = [_6,_6,_6];
_4 = 10116057629508337755790418574284640578_i128 as u16;
_11.1 = 561542943_i32 as f64;
_11.1 = 4125_i16 as f64;
_3 = _4;
_3 = _4 + _4;
_11.1 = 145877770165604918998284787253569087080_i128 as f64;
_13 = !true;
_12 = [9_u8];
_7 = [2_usize,7_usize];
_11.0 = (-59_i8);
Call(_2 = core::intrinsics::transmute(_6), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_5 = !330762179816842462997959263196087466535_u128;
_16 = _11.0;
Goto(bb11)
}
bb11 = {
_1 = 27967_i16 as f32;
_17 = _7;
_18 = [2_usize,1617113044661022128_usize];
_14 = [179_u8];
_6 = '\u{d3e11}';
_5 = !212574002975041455820882492188264554285_u128;
_12 = [103_u8];
_10 = [_6,_6,_6];
_5 = 92030770633827454411984787781335468804_u128;
Call(_13 = fn6(), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_15.0 = [_3,_4,_4,_3,_4,_3,_4];
_6 = '\u{1720a}';
_21 = [_6,_6,_6];
_14 = [30_u8];
_20 = _16;
_8 = [254_u8];
_24 = [941221996_u32,3108401928_u32,510364357_u32];
_16 = _20 >> _3;
_19 = (-64_isize) | 9223372036854775807_isize;
_15.0 = [_3,_3,_4,_3,_3,_3,_3];
_1 = -_2;
_19 = _3 as isize;
_4 = _6 as u16;
_23 = -5997023698790428360_i64;
_21 = _10;
_22 = _19 ^ _19;
Goto(bb13)
}
bb13 = {
_13 = !false;
_17 = [17713540968366930564_usize,7_usize];
_20 = _2 as i8;
_16 = _11.0;
_20 = _16 ^ _16;
_20 = _16;
_11.1 = 6_usize as f64;
_8 = _12;
_23 = -(-8057540490777475906_i64);
_28 = _11.1;
_14 = [45_u8];
_2 = _1 + _1;
_21 = [_6,_6,_6];
_16 = _11.0 ^ _11.0;
_10 = _21;
_21 = [_6,_6,_6];
_22 = _19;
_10 = [_6,_6,_6];
_24 = [3544846226_u32,2167132186_u32,818248884_u32];
_21 = [_6,_6,_6];
_23 = (-3256398109544814112_i64) | (-8219242571837757349_i64);
_3 = _11.0 as u16;
match _20 {
0 => bb1,
1 => bb12,
2 => bb10,
3 => bb14,
340282366920938463463374607431768211397 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
_5 = !330762179816842462997959263196087466535_u128;
_16 = _11.0;
Goto(bb11)
}
bb16 = {
_28 = -_11.1;
_18 = _7;
_29 = (-54666479551804293103706540553470533352_i128) as f64;
_22 = -_19;
_31.2 = 3527655581_u32;
_14 = [169_u8];
_17 = _18;
_12 = [156_u8];
_30 = 12004840295397203943_u64 as isize;
_4 = !_3;
_31.3.1 = [_31.2,_31.2,_31.2];
_16 = -_11.0;
_31.3.1 = _24;
_29 = _28;
_11 = (_16, _28);
_12 = [50_u8];
_12 = [84_u8];
_13 = !true;
_31.0 = (704708230_i32,);
_23 = 5408776196783901000_i64 & (-6741914619586674035_i64);
_29 = -_28;
RET = core::ptr::addr_of!(_34.0);
_9 = core::ptr::addr_of!((*RET));
Goto(bb17)
}
bb17 = {
Call(_35 = dump_var(5_usize, 30_usize, Move(_30), 4_usize, Move(_4), 20_usize, Move(_20), 13_usize, Move(_13)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_35 = dump_var(5_usize, 21_usize, Move(_21), 14_usize, Move(_14), 23_usize, Move(_23), 19_usize, Move(_19)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_35 = dump_var(5_usize, 5_usize, Move(_5), 22_usize, Move(_22), 36_usize, _36, 36_usize, _36), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6() -> bool {
mir! {
type RET = bool;
let _1: [u16; 1];
let _2: (i16, bool);
let _3: Adt75;
let _4: f64;
let _5: u64;
let _6: [u8; 5];
let _7: (i32,);
let _8: f32;
let _9: ((Adt27, *const u64, [u8; 5]), &'static *const usize, (Adt35, *mut &'static i8), *const usize);
let _10: isize;
let _11: i64;
let _12: (f64, u8, (Adt35, *mut &'static i8), isize);
let _13: f32;
let _14: isize;
let _15: u64;
let _16: *mut f64;
let _17: &'static u8;
let _18: [u16; 7];
let _19: (Adt44, [u32; 3]);
let _20: i16;
let _21: bool;
let _22: *const [u16; 7];
let _23: *mut &'static i8;
let _24: *mut f64;
let _25: f64;
let _26: i32;
let _27: char;
let _28: (i32,);
let _29: &'static (*mut char,);
let _30: isize;
let _31: f32;
let _32: Adt42;
let _33: [usize; 2];
let _34: (*mut &'static i8,);
let _35: u16;
let _36: &'static u8;
let _37: *mut [u16; 7];
let _38: bool;
let _39: [usize; 2];
let _40: *mut *const &'static u8;
let _41: u16;
let _42: bool;
let _43: isize;
let _44: [usize; 2];
let _45: [usize; 8];
let _46: char;
let _47: (i16, bool);
let _48: ();
let _49: ();
{
RET = true | true;
RET = !false;
RET = false;
RET = true;
Call(RET = fn7(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = !true;
RET = true;
RET = !true;
RET = false;
RET = !true;
RET = (-9223372036854775808_isize) != 9223372036854775807_isize;
RET = !false;
RET = true;
RET = !true;
RET = true;
RET = 977564299378477582_i64 != 5258317857109913219_i64;
RET = false;
_1 = [4803_u16];
_1 = [1384_u16];
RET = false;
_1 = [48863_u16];
RET = (-4783_i16) <= (-28220_i16);
RET = !false;
_1 = [38166_u16];
RET = !false;
RET = false;
RET = !false;
RET = false;
RET = false;
RET = false;
RET = !false;
RET = true;
RET = false ^ true;
Call(RET = fn16(_1, _1, _1, _1, _1, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = false | false;
_2 = (14955_i16, RET);
_2 = (8639_i16, RET);
RET = _2.1;
RET = _2.1;
Goto(bb3)
}
bb3 = {
_2.0 = 18875_i16;
_2.1 = !RET;
_1 = [58315_u16];
_4 = 103_i8 as f64;
_5 = 18282428611888531136_u64;
_6 = [171_u8,157_u8,111_u8,88_u8,246_u8];
_1 = [55836_u16];
_7 = (1130107921_i32,);
_1 = [61102_u16];
_2 = Checked((-28311_i16) - (-30666_i16));
_7.0 = !(-1628625053_i32);
_6 = [86_u8,11_u8,41_u8,137_u8,117_u8];
_7 = (198127650_i32,);
_7 = ((-726602391_i32),);
_9.0.2 = _6;
_10 = '\u{8c85}' as isize;
Call(_5 = core::intrinsics::transmute(_10), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = _2.0 < _2.0;
_11 = '\u{14ed6}' as i64;
_7 = ((-1806991089_i32),);
_9.0.2 = [21_u8,162_u8,195_u8,71_u8,208_u8];
_8 = 49_i8 as f32;
_2.1 = RET;
_2 = Checked((-25827_i16) * 9308_i16);
_5 = !5262883238366325568_u64;
_5 = _11 as u64;
_2.1 = !RET;
_7 = ((-586206676_i32),);
_13 = _4 as f32;
_12.1 = 45_u8;
RET = !_2.1;
_9.0.2 = [_12.1,_12.1,_12.1,_12.1,_12.1];
_6 = [_12.1,_12.1,_12.1,_12.1,_12.1];
_7.0 = _12.1 as i32;
_15 = _13 as u64;
_12.3 = !_10;
_9.0.1 = core::ptr::addr_of!(_15);
match _12.1 {
0 => bb3,
45 => bb5,
_ => bb2
}
}
bb5 = {
_12.3 = _10;
_12.0 = _4 - _4;
_12.0 = _10 as f64;
_7 = (899581683_i32,);
_9.1 = &_9.3;
_6 = [_12.1,_12.1,_12.1,_12.1,_12.1];
_13 = 179334149965693568032566795697016333009_u128 as f32;
_9.1 = &_9.3;
_5 = _15;
_16 = core::ptr::addr_of_mut!(_12.0);
_16 = core::ptr::addr_of_mut!(_12.0);
_12.0 = _4 * _4;
_17 = &_12.1;
_15 = _2.0 as u64;
_10 = !_12.3;
_2.1 = _4 > _12.0;
_2.1 = RET;
RET = _2.1;
(*_16) = -_4;
_18 = [38559_u16,27888_u16,43566_u16,6047_u16,18674_u16,9776_u16,36550_u16];
_5 = _15;
_12.3 = _10 ^ _10;
_12.0 = _4 - _4;
_12.0 = _4;
Goto(bb6)
}
bb6 = {
_12.0 = 12301226750168837731_usize as f64;
RET = _13 < _8;
RET = !_2.1;
_4 = (*_16);
(*_16) = -_4;
_5 = _15;
_13 = _8;
_14 = _10;
RET = !_2.1;
(*_16) = _4;
(*_16) = _4;
_14 = _12.3 << _7.0;
_19.1 = [2550735159_u32,2139304350_u32,2069773315_u32];
RET = !_2.1;
_9.0.1 = core::ptr::addr_of!(_15);
RET = _5 > _15;
_9.0.2 = _6;
_5 = 0_usize as u64;
_19.1 = [2917474347_u32,634805838_u32,1222181928_u32];
_1 = [19967_u16];
RET = _2.0 <= _2.0;
_4 = (*_16);
(*_16) = -_4;
match _12.1 {
0 => bb1,
1 => bb2,
2 => bb5,
45 => bb7,
_ => bb4
}
}
bb7 = {
_7 = ((-1898871290_i32),);
_2.0 = !15636_i16;
_20 = _2.0 ^ _2.0;
_20 = _2.0;
_2.1 = !RET;
_5 = _20 as u64;
_15 = _5 << _14;
_22 = core::ptr::addr_of!(_18);
_22 = core::ptr::addr_of!((*_22));
_7 = (1801878117_i32,);
_18 = [14848_u16,41163_u16,64856_u16,44344_u16,21862_u16,52717_u16,16639_u16];
(*_16) = _4;
RET = _2.1;
_16 = core::ptr::addr_of_mut!(_4);
_12.0 = (*_16);
_5 = _15;
_22 = core::ptr::addr_of!((*_22));
_25 = (*_16);
_13 = _8 - _8;
match _7.0 {
0 => bb1,
1801878117 => bb8,
_ => bb5
}
}
bb8 = {
_1 = [29075_u16];
_8 = _13;
Goto(bb9)
}
bb9 = {
_1 = [11869_u16];
_25 = (*_16) - (*_16);
_12.3 = _10 * _14;
_6 = [(*_17),_12.1,_12.1,(*_17),_12.1];
_20 = _2.0;
_5 = _15 | _15;
_8 = _13 - _13;
(*_16) = _25;
_9.0.2 = [(*_17),_12.1,(*_17),(*_17),(*_17)];
_26 = _7.0;
_10 = _12.3;
_28.0 = _7.0 | _7.0;
_6 = _9.0.2;
_12.1 = '\u{10f051}' as u8;
_16 = core::ptr::addr_of_mut!(_4);
(*_16) = _25;
_14 = _12.3 - _10;
_12.3 = _12.1 as isize;
Goto(bb10)
}
bb10 = {
_2 = Checked(_20 - _20);
_9.0.2 = [_12.1,_12.1,_12.1,_12.1,_12.1];
_13 = -_8;
_18 = [2021_u16,63987_u16,28105_u16,55945_u16,45320_u16,55982_u16,52021_u16];
_11 = (-4301689225618440974_i64) - (-3236519576476701363_i64);
_2.0 = _20 + _20;
_6 = [_12.1,_12.1,_12.1,_12.1,_12.1];
_27 = '\u{fb823}';
Goto(bb11)
}
bb11 = {
(*_22) = [10651_u16,47688_u16,38623_u16,49092_u16,10222_u16,25213_u16,23350_u16];
_16 = core::ptr::addr_of_mut!((*_16));
_15 = _5 - _5;
_22 = core::ptr::addr_of!((*_22));
_16 = core::ptr::addr_of_mut!((*_16));
_4 = _12.0 * _12.0;
_14 = _10;
_9.0.1 = core::ptr::addr_of!(_5);
_31 = _12.1 as f32;
(*_22) = [63575_u16,37325_u16,47860_u16,54636_u16,29741_u16,35999_u16,64845_u16];
_21 = _15 != _5;
Goto(bb12)
}
bb12 = {
_5 = _15 << _15;
_31 = _12.1 as f32;
_33 = [14918154334088720030_usize,9394987549368063893_usize];
Goto(bb13)
}
bb13 = {
_33 = [2332538067361163195_usize,14349905586122517436_usize];
_37 = core::ptr::addr_of_mut!((*_22));
(*_16) = _20 as f64;
(*_16) = _12.0;
(*_37) = [28061_u16,52728_u16,62841_u16,32300_u16,7231_u16,37797_u16,22435_u16];
_24 = core::ptr::addr_of_mut!(_4);
_27 = '\u{1003b6}';
_2.0 = _20;
_30 = 1547_u16 as isize;
_10 = _30;
_15 = _5;
_2.0 = _12.0 as i16;
_20 = _2.0 >> _15;
_27 = '\u{4e56c}';
_43 = -_10;
_39 = _33;
_9.0.1 = core::ptr::addr_of!(_5);
_44 = _33;
_6 = _9.0.2;
_28.0 = _7.0 >> _5;
_41 = 24528_u16 - 63742_u16;
_16 = core::ptr::addr_of_mut!((*_16));
match _26 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
1801878117 => bb19,
_ => bb18
}
}
bb14 = {
RET = false | false;
_2 = (14955_i16, RET);
_2 = (8639_i16, RET);
RET = _2.1;
RET = _2.1;
Goto(bb3)
}
bb15 = {
_7 = ((-1898871290_i32),);
_2.0 = !15636_i16;
_20 = _2.0 ^ _2.0;
_20 = _2.0;
_2.1 = !RET;
_5 = _20 as u64;
_15 = _5 << _14;
_22 = core::ptr::addr_of!(_18);
_22 = core::ptr::addr_of!((*_22));
_7 = (1801878117_i32,);
_18 = [14848_u16,41163_u16,64856_u16,44344_u16,21862_u16,52717_u16,16639_u16];
(*_16) = _4;
RET = _2.1;
_16 = core::ptr::addr_of_mut!(_4);
_12.0 = (*_16);
_5 = _15;
_22 = core::ptr::addr_of!((*_22));
_25 = (*_16);
_13 = _8 - _8;
match _7.0 {
0 => bb1,
1801878117 => bb8,
_ => bb5
}
}
bb16 = {
RET = !true;
RET = true;
RET = !true;
RET = false;
RET = !true;
RET = (-9223372036854775808_isize) != 9223372036854775807_isize;
RET = !false;
RET = true;
RET = !true;
RET = true;
RET = 977564299378477582_i64 != 5258317857109913219_i64;
RET = false;
_1 = [4803_u16];
_1 = [1384_u16];
RET = false;
_1 = [48863_u16];
RET = (-4783_i16) <= (-28220_i16);
RET = !false;
_1 = [38166_u16];
RET = !false;
RET = false;
RET = !false;
RET = false;
RET = false;
RET = false;
RET = !false;
RET = true;
RET = false ^ true;
Call(RET = fn16(_1, _1, _1, _1, _1, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb17 = {
RET = _2.0 < _2.0;
_11 = '\u{14ed6}' as i64;
_7 = ((-1806991089_i32),);
_9.0.2 = [21_u8,162_u8,195_u8,71_u8,208_u8];
_8 = 49_i8 as f32;
_2.1 = RET;
_2 = Checked((-25827_i16) * 9308_i16);
_5 = !5262883238366325568_u64;
_5 = _11 as u64;
_2.1 = !RET;
_7 = ((-586206676_i32),);
_13 = _4 as f32;
_12.1 = 45_u8;
RET = !_2.1;
_9.0.2 = [_12.1,_12.1,_12.1,_12.1,_12.1];
_6 = [_12.1,_12.1,_12.1,_12.1,_12.1];
_7.0 = _12.1 as i32;
_15 = _13 as u64;
_12.3 = !_10;
_9.0.1 = core::ptr::addr_of!(_15);
match _12.1 {
0 => bb3,
45 => bb5,
_ => bb2
}
}
bb18 = {
_1 = [29075_u16];
_8 = _13;
Goto(bb9)
}
bb19 = {
(*_37) = [_41,_41,_41,_41,_41,_41,_41];
_28 = (_26,);
_16 = core::ptr::addr_of_mut!((*_24));
_9.1 = &_9.3;
Goto(bb20)
}
bb20 = {
Call(_48 = dump_var(6_usize, 43_usize, Move(_43), 10_usize, Move(_10), 15_usize, Move(_15), 39_usize, Move(_39)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_48 = dump_var(6_usize, 30_usize, Move(_30), 21_usize, Move(_21), 27_usize, Move(_27), 20_usize, Move(_20)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_48 = dump_var(6_usize, 33_usize, Move(_33), 44_usize, Move(_44), 49_usize, _49, 49_usize, _49), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7() -> bool {
mir! {
type RET = bool;
let _1: (*mut (*mut &'static i8,), u16);
let _2: *mut &'static i8;
let _3: *mut *const &'static u8;
let _4: u8;
let _5: f64;
let _6: [char; 3];
let _7: &'static (Adt35, *mut &'static i8);
let _8: char;
let _9: u8;
let _10: isize;
let _11: Adt59;
let _12: &'static [usize; 2];
let _13: bool;
let _14: (isize, u32, (Adt44, [u32; 3]), usize);
let _15: u16;
let _16: *mut &'static i8;
let _17: Adt44;
let _18: f32;
let _19: (usize,);
let _20: u32;
let _21: bool;
let _22: ((i32,), (i32,), u32, (&'static &'static i8, [u32; 3], [u32; 3], bool));
let _23: u8;
let _24: f64;
let _25: *mut f64;
let _26: isize;
let _27: f32;
let _28: char;
let _29: &'static (i16, bool);
let _30: &'static [u8; 1];
let _31: i128;
let _32: [u8; 5];
let _33: u128;
let _34: Adt52;
let _35: char;
let _36: Adt75;
let _37: i16;
let _38: f32;
let _39: u8;
let _40: ();
let _41: ();
{
RET = true & true;
RET = !false;
RET = true;
RET = false;
RET = true;
RET = (-50_i8) != 52_i8;
RET = false;
RET = false;
RET = 9223372036854775807_isize == (-9223372036854775808_isize);
_1.1 = 23553_u16;
RET = true;
RET = _1.1 < _1.1;
_1.1 = 3081198589_u32 as u16;
RET = !false;
RET = false & false;
_1.1 = 63498_u16;
_1.1 = 3515_u16 & 37773_u16;
_1.1 = !26781_u16;
RET = true;
Call(_1.1 = fn8(RET, RET, RET, RET, RET, RET, RET, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _1.1 <= _1.1;
RET = false ^ true;
RET = _1.1 >= _1.1;
_4 = 16367136999279191453_u64 as u8;
RET = _1.1 == _1.1;
_1.1 = 59353_u16 << _4;
RET = _1.1 != _1.1;
RET = !false;
_1.1 = (-88_isize) as u16;
RET = !false;
RET = _4 >= _4;
RET = !false;
_4 = _1.1 as u8;
_5 = (-46_isize) as f64;
_6 = ['\u{581f9}','\u{107011}','\u{ae2bc}'];
_1.1 = !61536_u16;
_1.1 = !8011_u16;
_1.1 = 55349_u16;
_6 = ['\u{c3160}','\u{11ff0}','\u{607ae}'];
_5 = 346294239_i32 as f64;
Goto(bb2)
}
bb2 = {
_5 = 2_usize as f64;
Call(_1 = fn9(_6, RET, _4, _6, _6, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = 99_u8 >> _1.1;
_1.1 = (-294361410692986561_i64) as u16;
_5 = 15221098789274851060_u64 as f64;
_5 = (-9223372036854775808_isize) as f64;
_9 = 1981121463156426184_u64 as u8;
Goto(bb4)
}
bb4 = {
_8 = '\u{793de}';
RET = !true;
_8 = '\u{afc12}';
_1.1 = 65482_u16 ^ 14387_u16;
_8 = '\u{102e4f}';
_1.1 = (-6762949709692521894_i64) as u16;
_8 = '\u{202fb}';
Goto(bb5)
}
bb5 = {
RET = _8 < _8;
_8 = '\u{b45ab}';
_11.fld3 = _1.1 as f64;
_11.fld3 = -_5;
_11.fld3 = _5 - _5;
RET = _1.1 > _1.1;
_1.1 = !45914_u16;
_8 = '\u{adacb}';
_11.fld2 = 115297019517082128898352760485208775183_u128;
_10 = 9223372036854775807_isize & 17_isize;
_8 = '\u{942b9}';
_11.fld1 = core::ptr::addr_of_mut!(RET);
_9 = _4;
_11.fld1 = core::ptr::addr_of_mut!(RET);
_11.fld1 = core::ptr::addr_of_mut!(RET);
RET = _10 < _10;
_6 = [_8,_8,_8];
_9 = !_4;
_4 = _9;
_10 = 19_isize + (-41_isize);
_1.1 = RET as u16;
Goto(bb6)
}
bb6 = {
_5 = _11.fld2 as f64;
_11.fld2 = 23196529305930131586428850745405454611_u128 * 269952761821264831838176881457439078486_u128;
RET = _4 <= _4;
_5 = -_11.fld3;
_14.3 = _11.fld2 as usize;
_14.2.1 = [1501153194_u32,3250562124_u32,2620887803_u32];
_8 = '\u{b4b0b}';
_14.1 = 43157839911923925716972699628849500295_i128 as u32;
_14.1 = !1577373418_u32;
_11.fld0 = !_14.3;
_5 = _10 as f64;
_18 = _10 as f32;
_19 = (_14.3,);
_11.fld1 = core::ptr::addr_of_mut!(_13);
_19 = (_14.3,);
_5 = _11.fld3;
_18 = _19.0 as f32;
_18 = (-1367925562_i32) as f32;
_10 = (-71_isize) << _19.0;
_4 = _11.fld2 as u8;
Goto(bb7)
}
bb7 = {
_22.3.1 = _14.2.1;
_22.1 = ((-110457602_i32),);
_14.1 = !596076482_u32;
_8 = '\u{a27c0}';
_15 = !_1.1;
_20 = _22.1.0 as u32;
_5 = (-163986461225006113445023441669414313896_i128) as f64;
_11.fld2 = 221990399417477995562299718891011444384_u128;
_22.3.2 = [_14.1,_14.1,_20];
_4 = _9 & _9;
_22.2 = _20;
_22.0 = _22.1;
_8 = '\u{5ebd9}';
_22.1 = (_22.0.0,);
_24 = _11.fld3 + _5;
_20 = _10 as u32;
_27 = _18 + _18;
_26 = _14.3 as isize;
_25 = core::ptr::addr_of_mut!(_24);
_25 = core::ptr::addr_of_mut!(_5);
_1.1 = _15 & _15;
_11.fld0 = _10 as usize;
Goto(bb8)
}
bb8 = {
_5 = _24 + _24;
_11.fld2 = 280630527219883771153180824319641485488_u128 & 192857220501960975064795825260263031205_u128;
_23 = RET as u8;
_23 = !_9;
_13 = RET;
_19.0 = !_11.fld0;
_9 = _13 as u8;
_21 = _13 & RET;
_22.3.3 = !RET;
_25 = core::ptr::addr_of_mut!(_11.fld3);
_31 = _22.2 as i128;
_22.3.3 = !_13;
match _22.1.0 {
0 => bb1,
1 => bb4,
2 => bb3,
340282366920938463463374607431657753854 => bb10,
_ => bb9
}
}
bb9 = {
RET = _8 < _8;
_8 = '\u{b45ab}';
_11.fld3 = _1.1 as f64;
_11.fld3 = -_5;
_11.fld3 = _5 - _5;
RET = _1.1 > _1.1;
_1.1 = !45914_u16;
_8 = '\u{adacb}';
_11.fld2 = 115297019517082128898352760485208775183_u128;
_10 = 9223372036854775807_isize & 17_isize;
_8 = '\u{942b9}';
_11.fld1 = core::ptr::addr_of_mut!(RET);
_9 = _4;
_11.fld1 = core::ptr::addr_of_mut!(RET);
_11.fld1 = core::ptr::addr_of_mut!(RET);
RET = _10 < _10;
_6 = [_8,_8,_8];
_9 = !_4;
_4 = _9;
_10 = 19_isize + (-41_isize);
_1.1 = RET as u16;
Goto(bb6)
}
bb10 = {
_31 = _15 as i128;
_27 = -_18;
Goto(bb11)
}
bb11 = {
_23 = _4 * _4;
_22.3.2 = _14.2.1;
_14.0 = -_26;
_20 = _22.2;
_11.fld0 = _19.0;
_32 = [_23,_9,_23,_23,_9];
_14.1 = _22.2 << _1.1;
_5 = _24;
_10 = _11.fld0 as isize;
_1.1 = _15 | _15;
_14.0 = 7326675117122278910_i64 as isize;
(*_25) = _5;
_4 = _9 << _14.1;
_19.0 = _21 as usize;
_22.1 = (_22.0.0,);
_21 = !_13;
_11.fld3 = _24;
_27 = -_18;
_10 = -_14.0;
RET = _21;
_19.0 = !_14.3;
RET = !_21;
_22.3.2 = [_14.1,_14.1,_14.1];
match _22.1.0 {
340282366920938463463374607431657753854 => bb13,
_ => bb12
}
}
bb12 = {
RET = _8 < _8;
_8 = '\u{b45ab}';
_11.fld3 = _1.1 as f64;
_11.fld3 = -_5;
_11.fld3 = _5 - _5;
RET = _1.1 > _1.1;
_1.1 = !45914_u16;
_8 = '\u{adacb}';
_11.fld2 = 115297019517082128898352760485208775183_u128;
_10 = 9223372036854775807_isize & 17_isize;
_8 = '\u{942b9}';
_11.fld1 = core::ptr::addr_of_mut!(RET);
_9 = _4;
_11.fld1 = core::ptr::addr_of_mut!(RET);
_11.fld1 = core::ptr::addr_of_mut!(RET);
RET = _10 < _10;
_6 = [_8,_8,_8];
_9 = !_4;
_4 = _9;
_10 = 19_isize + (-41_isize);
_1.1 = RET as u16;
Goto(bb6)
}
bb13 = {
_14.1 = !_20;
_32 = [_4,_4,_4,_4,_4];
_14.1 = !_22.2;
_23 = _4 + _9;
(*_25) = _5 - _24;
_18 = -_27;
match _22.1.0 {
0 => bb14,
1 => bb15,
2 => bb16,
340282366920938463463374607431657753854 => bb18,
_ => bb17
}
}
bb14 = {
_22.3.1 = _14.2.1;
_22.1 = ((-110457602_i32),);
_14.1 = !596076482_u32;
_8 = '\u{a27c0}';
_15 = !_1.1;
_20 = _22.1.0 as u32;
_5 = (-163986461225006113445023441669414313896_i128) as f64;
_11.fld2 = 221990399417477995562299718891011444384_u128;
_22.3.2 = [_14.1,_14.1,_20];
_4 = _9 & _9;
_22.2 = _20;
_22.0 = _22.1;
_8 = '\u{5ebd9}';
_22.1 = (_22.0.0,);
_24 = _11.fld3 + _5;
_20 = _10 as u32;
_27 = _18 + _18;
_26 = _14.3 as isize;
_25 = core::ptr::addr_of_mut!(_24);
_25 = core::ptr::addr_of_mut!(_5);
_1.1 = _15 & _15;
_11.fld0 = _10 as usize;
Goto(bb8)
}
bb15 = {
RET = _8 < _8;
_8 = '\u{b45ab}';
_11.fld3 = _1.1 as f64;
_11.fld3 = -_5;
_11.fld3 = _5 - _5;
RET = _1.1 > _1.1;
_1.1 = !45914_u16;
_8 = '\u{adacb}';
_11.fld2 = 115297019517082128898352760485208775183_u128;
_10 = 9223372036854775807_isize & 17_isize;
_8 = '\u{942b9}';
_11.fld1 = core::ptr::addr_of_mut!(RET);
_9 = _4;
_11.fld1 = core::ptr::addr_of_mut!(RET);
_11.fld1 = core::ptr::addr_of_mut!(RET);
RET = _10 < _10;
_6 = [_8,_8,_8];
_9 = !_4;
_4 = _9;
_10 = 19_isize + (-41_isize);
_1.1 = RET as u16;
Goto(bb6)
}
bb16 = {
_5 = _11.fld2 as f64;
_11.fld2 = 23196529305930131586428850745405454611_u128 * 269952761821264831838176881457439078486_u128;
RET = _4 <= _4;
_5 = -_11.fld3;
_14.3 = _11.fld2 as usize;
_14.2.1 = [1501153194_u32,3250562124_u32,2620887803_u32];
_8 = '\u{b4b0b}';
_14.1 = 43157839911923925716972699628849500295_i128 as u32;
_14.1 = !1577373418_u32;
_11.fld0 = !_14.3;
_5 = _10 as f64;
_18 = _10 as f32;
_19 = (_14.3,);
_11.fld1 = core::ptr::addr_of_mut!(_13);
_19 = (_14.3,);
_5 = _11.fld3;
_18 = _19.0 as f32;
_18 = (-1367925562_i32) as f32;
_10 = (-71_isize) << _19.0;
_4 = _11.fld2 as u8;
Goto(bb7)
}
bb17 = {
_4 = 99_u8 >> _1.1;
_1.1 = (-294361410692986561_i64) as u16;
_5 = 15221098789274851060_u64 as f64;
_5 = (-9223372036854775808_isize) as f64;
_9 = 1981121463156426184_u64 as u8;
Goto(bb4)
}
bb18 = {
_11.fld1 = core::ptr::addr_of_mut!(_13);
_1.1 = _23 as u16;
_32 = [_23,_23,_23,_4,_23];
_33 = _26 as u128;
_25 = core::ptr::addr_of_mut!(_5);
_9 = _15 as u8;
_34 = Adt52::Variant1 { fld0: _21 };
_35 = _8;
(*_25) = _11.fld3;
_18 = -_27;
_35 = _8;
SetDiscriminant(_34, 0);
_24 = (*_25);
place!(Field::<u64>(Variant(_34, 0), 0)) = !4243545830476933617_u64;
place!(Field::<(i16, bool)>(Variant(_34, 0), 5)) = Checked(17128_i16 + 863_i16);
_32 = [_23,_23,_23,_23,_23];
RET = !_21;
_8 = _35;
_12 = &place!(Field::<[usize; 2]>(Variant(_34, 0), 4));
_6 = [_35,_8,_35];
Goto(bb19)
}
bb19 = {
Call(_40 = dump_var(7_usize, 4_usize, Move(_4), 26_usize, Move(_26), 9_usize, Move(_9), 19_usize, Move(_19)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_40 = dump_var(7_usize, 15_usize, Move(_15), 21_usize, Move(_21), 6_usize, Move(_6), 33_usize, Move(_33)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool) -> u16 {
mir! {
type RET = u16;
let _9: [u8; 5];
let _10: char;
let _11: isize;
let _12: &'static (f64, u8, (Adt35, *mut &'static i8), isize);
let _13: (i8, f64);
let _14: &'static (Adt35, *mut &'static i8);
let _15: (usize,);
let _16: isize;
let _17: [u32; 3];
let _18: (usize,);
let _19: (*const u64, (i8, f64), [u16; 7], bool);
let _20: char;
let _21: [usize; 2];
let _22: isize;
let _23: [u16; 1];
let _24: [u8; 1];
let _25: u32;
let _26: char;
let _27: [u8; 1];
let _28: (i32,);
let _29: ();
let _30: ();
{
_3 = _1;
_1 = !_8;
_5 = _6 <= _8;
_2 = _7;
_8 = !_2;
_7 = !_5;
_1 = _3;
_9 = [32_u8,136_u8,238_u8,21_u8,217_u8];
RET = 55255_u16;
_8 = !_7;
_3 = _5;
RET = 25433_u16;
_7 = _6 | _4;
_8 = _2 ^ _4;
_4 = !_7;
_2 = !_6;
_10 = '\u{99542}';
_9 = [73_u8,208_u8,174_u8,183_u8,29_u8];
_11 = 119_i8 as isize;
_11 = !(-9223372036854775808_isize);
_2 = _5 ^ _3;
_5 = _7 <= _7;
_8 = _4;
_13.1 = (-89_i8) as f64;
_11 = (-862604544_i32) as isize;
match RET {
25433 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_2 = !_8;
_1 = _5 ^ _5;
_2 = _1;
_13.1 = 248861455258402061164551225296676570389_u128 as f64;
match RET {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
25433 => bb8,
_ => bb7
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
_1 = _5 <= _5;
_4 = !_2;
_9 = [207_u8,118_u8,32_u8,115_u8,99_u8];
_15.0 = !15007925289257749277_usize;
RET = !50409_u16;
_6 = _4 ^ _4;
_15.0 = 6026585662058796509_usize << _11;
_9 = [0_u8,214_u8,255_u8,214_u8,17_u8];
_2 = !_6;
_9 = [141_u8,60_u8,226_u8,111_u8,86_u8];
Goto(bb9)
}
bb9 = {
_15 = (15101078193531210874_usize,);
RET = !50199_u16;
_15 = (1_usize,);
_5 = !_2;
_6 = !_2;
_2 = _4 == _8;
_16 = _11;
_13.1 = 4975010718385837659_u64 as f64;
_10 = '\u{3b769}';
_13.1 = (-104048607897962718480520103932640873957_i128) as f64;
_18 = _15;
_1 = !_3;
_5 = _6;
_8 = !_6;
_15 = (_18.0,);
match _18.0 {
1 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_6 = !_5;
_8 = _5 ^ _6;
_19.3 = _8 & _2;
_20 = _10;
_17 = [1687505169_u32,3721792931_u32,1008108104_u32];
_19.1.1 = -_13.1;
_22 = -_11;
_19.1.0 = 1135912794_i32 as i8;
RET = 46243_u16;
_15.0 = _18.0;
_13 = (_19.1.0, _19.1.1);
_3 = !_19.3;
_16 = _22 & _22;
_19.1.0 = !_13.0;
_19.1 = (_13.0, _13.1);
_17 = [3748811656_u32,3737212190_u32,1033242383_u32];
_13 = (_19.1.0, _19.1.1);
_24 = [63_u8];
_13.0 = _19.1.0;
Goto(bb12)
}
bb12 = {
_19.1.1 = -_13.1;
_13 = _19.1;
RET = 29635_u16;
_19.2 = [RET,RET,RET,RET,RET,RET,RET];
match _18.0 {
0 => bb7,
1 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_1 = _19.3;
_13 = (_19.1.0, _19.1.1);
_3 = _4 != _5;
_20 = _10;
_6 = !_1;
_4 = _2;
_21 = [_18.0,_15.0];
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(8_usize, 15_usize, Move(_15), 24_usize, Move(_24), 18_usize, Move(_18), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(8_usize, 20_usize, Move(_20), 10_usize, Move(_10), 8_usize, Move(_8), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(8_usize, 16_usize, Move(_16), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [char; 3],mut _2: bool,mut _3: u8,mut _4: [char; 3],mut _5: [char; 3],mut _6: [char; 3]) -> (*mut (*mut &'static i8,), u16) {
mir! {
type RET = (*mut (*mut &'static i8,), u16);
let _7: (&'static &'static i8, [u32; 3], [u32; 3], bool);
let _8: [usize; 2];
let _9: bool;
let _10: *mut *const (Adt27, *const u64, [u8; 5]);
let _11: [char; 3];
let _12: Adt59;
let _13: i16;
let _14: f64;
let _15: [u8; 5];
let _16: [u16; 1];
let _17: &'static (f64, u8, (Adt35, *mut &'static i8), isize);
let _18: *mut u32;
let _19: isize;
let _20: *mut [u16; 7];
let _21: i32;
let _22: *const &'static u8;
let _23: i32;
let _24: usize;
let _25: *const Adt35;
let _26: [u8; 5];
let _27: isize;
let _28: ((*mut char,), &'static u8, *mut char, &'static &'static i8);
let _29: &'static (i16, bool);
let _30: ([u16; 7], &'static i128);
let _31: f64;
let _32: i16;
let _33: *mut f64;
let _34: *const Adt35;
let _35: [u32; 3];
let _36: i32;
let _37: [char; 3];
let _38: bool;
let _39: (i32,);
let _40: f64;
let _41: f64;
let _42: [u16; 7];
let _43: ();
let _44: ();
{
RET.1 = 44937_u16 >> _3;
_8 = [17765437717569827141_usize,0_usize];
RET.1 = 200908501419610089738009007354967763096_u128 as u16;
RET.1 = 59166_u16;
_7.3 = _2;
RET.1 = !32933_u16;
_7.1 = [162617047_u32,4079855645_u32,1689418894_u32];
_5 = _6;
_7.3 = _2 & _2;
_2 = !_7.3;
_2 = _7.3 & _7.3;
_8 = [4_usize,16113590166654223231_usize];
_7.2 = [1161314786_u32,751418173_u32,4084697372_u32];
_4 = _1;
_6 = ['\u{10116e}','\u{22cd8}','\u{e556b}'];
_2 = _7.3 > _7.3;
_6 = ['\u{51b3f}','\u{10c60c}','\u{fc448}'];
_4 = ['\u{ea5bc}','\u{57702}','\u{6492b}'];
Goto(bb1)
}
bb1 = {
_7.3 = !_2;
_12.fld1 = core::ptr::addr_of_mut!(_7.3);
_8 = [11969747369071768145_usize,7_usize];
_11 = ['\u{dadd8}','\u{5343a}','\u{25840}'];
_9 = _7.3;
Goto(bb2)
}
bb2 = {
_12.fld0 = 16393784760307542944_usize ^ 1_usize;
_13 = -(-22196_i16);
_14 = (-94_i8) as f64;
_8 = [_12.fld0,_12.fld0];
_12.fld3 = _14;
_14 = -_12.fld3;
_12.fld2 = !287788854946584218120526438231361146704_u128;
_12.fld2 = 123983668915787048724140172371743945136_u128;
_11 = _4;
_14 = _12.fld3;
_16 = [RET.1];
_12.fld3 = (-125_i8) as f64;
_12.fld0 = 2_usize + 1_usize;
_2 = !_7.3;
_7.2 = _7.1;
_16 = [RET.1];
_12.fld2 = 190592578004597170930205361693701661322_u128;
_12.fld0 = 2090814426425763219_usize;
_5 = _11;
_12.fld2 = 42353642356444799460725355024567665284_u128 | 304084470211351439757124143589271051176_u128;
_6 = _11;
_12.fld1 = core::ptr::addr_of_mut!(_7.3);
_3 = 79_u8;
RET.1 = 49783_u16 ^ 29730_u16;
_1 = ['\u{7fed6}','\u{44624}','\u{da969}'];
_11 = _6;
Goto(bb3)
}
bb3 = {
_9 = _2 | _2;
_7.1 = [1829396190_u32,1371368873_u32,1819546036_u32];
_14 = _12.fld3 * _12.fld3;
_7.2 = _7.1;
_2 = _7.3;
_12.fld0 = !6729220059057660329_usize;
_12.fld1 = core::ptr::addr_of_mut!(_7.3);
_12.fld2 = 151153657924035399333012759429307386926_u128 - 134434814541804972071194411738584940689_u128;
_14 = (-8_i8) as f64;
Goto(bb4)
}
bb4 = {
_12.fld3 = -_14;
_12.fld2 = 25462263474297020338703733332659364743_u128 >> _12.fld0;
_8 = [_12.fld0,_12.fld0];
_1 = ['\u{7a55}','\u{476d1}','\u{3bd3c}'];
_5 = _4;
_14 = -_12.fld3;
Call(RET.1 = core::intrinsics::transmute(_16), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_12.fld1 = core::ptr::addr_of_mut!(_2);
_12.fld0 = 15491874741209528184_usize;
_14 = _13 as f64;
_7.1 = [2928985280_u32,4069753894_u32,3539467070_u32];
_15 = [_3,_3,_3,_3,_3];
_21 = (-27_i8) as i32;
_2 = _9;
_11 = _1;
_16 = [RET.1];
_19 = (-9223372036854775808_isize) - 9223372036854775807_isize;
_9 = _2 & _7.3;
_24 = _12.fld0;
RET.1 = 21198_u16 * 56611_u16;
_9 = _24 <= _12.fld0;
_13 = _7.3 as i16;
_7.1 = _7.2;
_4 = _5;
_2 = !_9;
_8 = [_24,_24];
_1 = ['\u{1a366}','\u{6ed17}','\u{870ca}'];
_12.fld2 = 278533439940478016042495146786660984043_u128 >> _12.fld0;
_8 = [_24,_24];
_13 = (-19997_i16);
_19 = 9223372036854775807_isize;
_26 = [_3,_3,_3,_3,_3];
_12.fld1 = core::ptr::addr_of_mut!(_9);
_7.3 = _2 != _9;
_12.fld3 = -_14;
Goto(bb6)
}
bb6 = {
_23 = _21;
_16 = [RET.1];
_23 = _21;
Goto(bb7)
}
bb7 = {
_7.1 = [3677900297_u32,273147735_u32,1580235085_u32];
_16 = [RET.1];
_15 = [_3,_3,_3,_3,_3];
_9 = _2 ^ _7.3;
_26 = [_3,_3,_3,_3,_3];
_2 = _7.3;
_7.2 = _7.1;
_12.fld3 = _14 - _14;
_4 = ['\u{16974}','\u{106012}','\u{9fb9}'];
_3 = '\u{3a267}' as u8;
_4 = ['\u{33b0b}','\u{be67d}','\u{f196a}'];
_19 = 47_isize + 9223372036854775807_isize;
_14 = -_12.fld3;
_7.1 = [790617737_u32,1103254475_u32,1409874715_u32];
_19 = 9223372036854775807_isize * (-16_isize);
_6 = ['\u{d4820}','\u{d12c6}','\u{c8120}'];
_12.fld0 = RET.1 as usize;
_7.2 = [758394462_u32,1975096967_u32,3309528132_u32];
_28.1 = &_3;
RET.1 = !27002_u16;
_28.1 = &_3;
_3 = 237_u8 + 140_u8;
_2 = !_9;
Call(RET = fn10(_23, _1, _2, _2), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_7.2 = [1225768736_u32,3751543136_u32,2679025269_u32];
_7.2 = [2349649746_u32,595132464_u32,327187906_u32];
RET.1 = 21405_u16 - 19630_u16;
_28.1 = &_3;
_22 = core::ptr::addr_of!(_28.1);
_3 = 87_u8;
_3 = !202_u8;
_11 = ['\u{fb8c3}','\u{dc5f1}','\u{a1c2b}'];
_16 = [RET.1];
_6 = ['\u{bb842}','\u{14292}','\u{4977d}'];
_6 = ['\u{39508}','\u{ccf13}','\u{e6ac3}'];
(*_22) = &_3;
_26 = _15;
_9 = _7.3;
_22 = core::ptr::addr_of!((*_22));
_3 = _12.fld2 as u8;
_22 = core::ptr::addr_of!((*_22));
(*_22) = &_3;
_14 = _12.fld3;
_7.1 = [186749503_u32,2420502439_u32,876562460_u32];
_3 = 220_u8;
(*_22) = &_3;
(*_22) = &_3;
Goto(bb9)
}
bb9 = {
_7.3 = _2;
_12.fld0 = _24;
_11 = _5;
_22 = core::ptr::addr_of!(_28.1);
_14 = -_12.fld3;
_12.fld3 = _14 - _14;
_30.0 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
(*_22) = &_3;
_24 = (-109_i8) as usize;
_9 = !_7.3;
RET.1 = 63911_u16 + 50973_u16;
_7.2 = _7.1;
(*_22) = &_3;
_7.1 = [1703171295_u32,2450964738_u32,4269998658_u32];
_30.0 = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
(*_22) = &_3;
_32 = RET.1 as i16;
_16 = [RET.1];
_32 = _13 ^ _13;
Goto(bb10)
}
bb10 = {
(*_22) = &_3;
_31 = _12.fld2 as f64;
_23 = _21;
_6 = ['\u{3dc0}','\u{99523}','\u{7a2c2}'];
_19 = _32 as isize;
_12.fld2 = _14 as u128;
_20 = core::ptr::addr_of_mut!(_30.0);
_12.fld2 = _32 as u128;
RET.1 = _21 as u16;
_15 = [_3,_3,_3,_3,_3];
_33 = core::ptr::addr_of_mut!(_14);
_6 = _1;
_31 = -_14;
_4 = ['\u{f41f7}','\u{bfbe9}','\u{108dab}'];
_12.fld0 = _3 as usize;
_35 = [11460834_u32,3454529416_u32,2141234007_u32];
match _3 {
0 => bb11,
1 => bb12,
220 => bb14,
_ => bb13
}
}
bb11 = {
_12.fld0 = 16393784760307542944_usize ^ 1_usize;
_13 = -(-22196_i16);
_14 = (-94_i8) as f64;
_8 = [_12.fld0,_12.fld0];
_12.fld3 = _14;
_14 = -_12.fld3;
_12.fld2 = !287788854946584218120526438231361146704_u128;
_12.fld2 = 123983668915787048724140172371743945136_u128;
_11 = _4;
_14 = _12.fld3;
_16 = [RET.1];
_12.fld3 = (-125_i8) as f64;
_12.fld0 = 2_usize + 1_usize;
_2 = !_7.3;
_7.2 = _7.1;
_16 = [RET.1];
_12.fld2 = 190592578004597170930205361693701661322_u128;
_12.fld0 = 2090814426425763219_usize;
_5 = _11;
_12.fld2 = 42353642356444799460725355024567665284_u128 | 304084470211351439757124143589271051176_u128;
_6 = _11;
_12.fld1 = core::ptr::addr_of_mut!(_7.3);
_3 = 79_u8;
RET.1 = 49783_u16 ^ 29730_u16;
_1 = ['\u{7fed6}','\u{44624}','\u{da969}'];
_11 = _6;
Goto(bb3)
}
bb12 = {
_7.3 = !_2;
_12.fld1 = core::ptr::addr_of_mut!(_7.3);
_8 = [11969747369071768145_usize,7_usize];
_11 = ['\u{dadd8}','\u{5343a}','\u{25840}'];
_9 = _7.3;
Goto(bb2)
}
bb13 = {
_7.1 = [3677900297_u32,273147735_u32,1580235085_u32];
_16 = [RET.1];
_15 = [_3,_3,_3,_3,_3];
_9 = _2 ^ _7.3;
_26 = [_3,_3,_3,_3,_3];
_2 = _7.3;
_7.2 = _7.1;
_12.fld3 = _14 - _14;
_4 = ['\u{16974}','\u{106012}','\u{9fb9}'];
_3 = '\u{3a267}' as u8;
_4 = ['\u{33b0b}','\u{be67d}','\u{f196a}'];
_19 = 47_isize + 9223372036854775807_isize;
_14 = -_12.fld3;
_7.1 = [790617737_u32,1103254475_u32,1409874715_u32];
_19 = 9223372036854775807_isize * (-16_isize);
_6 = ['\u{d4820}','\u{d12c6}','\u{c8120}'];
_12.fld0 = RET.1 as usize;
_7.2 = [758394462_u32,1975096967_u32,3309528132_u32];
_28.1 = &_3;
RET.1 = !27002_u16;
_28.1 = &_3;
_3 = 237_u8 + 140_u8;
_2 = !_9;
Call(RET = fn10(_23, _1, _2, _2), ReturnTo(bb8), UnwindUnreachable())
}
bb14 = {
_32 = !_13;
_32 = _13 + _13;
_26 = _15;
_22 = core::ptr::addr_of!((*_22));
(*_20) = [RET.1,RET.1,RET.1,RET.1,RET.1,RET.1,RET.1];
_11 = ['\u{6cb4c}','\u{79646}','\u{b5708}'];
_31 = -_14;
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(9_usize, 26_usize, Move(_26), 6_usize, Move(_6), 8_usize, Move(_8), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(9_usize, 23_usize, Move(_23), 1_usize, Move(_1), 5_usize, Move(_5), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(9_usize, 24_usize, Move(_24), 44_usize, _44, 44_usize, _44, 44_usize, _44), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: i32,mut _2: [char; 3],mut _3: bool,mut _4: bool) -> (*mut (*mut &'static i8,), u16) {
mir! {
type RET = (*mut (*mut &'static i8,), u16);
let _5: f32;
let _6: (i16, bool);
let _7: &'static (i16, bool);
let _8: bool;
let _9: *mut f64;
let _10: *const *mut [u16; 7];
let _11: [char; 3];
let _12: (&'static &'static i8, [u32; 3], [u32; 3], bool);
let _13: char;
let _14: [char; 3];
let _15: [u16; 1];
let _16: u32;
let _17: char;
let _18: &'static i128;
let _19: u8;
let _20: (&'static &'static i8, [u32; 3], [u32; 3], bool);
let _21: [isize; 4];
let _22: *mut bool;
let _23: &'static [u8; 1];
let _24: f32;
let _25: f64;
let _26: *const Adt35;
let _27: Adt59;
let _28: &'static &'static i8;
let _29: &'static i128;
let _30: *const u64;
let _31: isize;
let _32: f64;
let _33: f32;
let _34: u8;
let _35: u32;
let _36: (i8, f64);
let _37: *mut char;
let _38: ((*mut char,), &'static u8, *mut char, &'static &'static i8);
let _39: &'static i8;
let _40: *const usize;
let _41: &'static *const usize;
let _42: i32;
let _43: char;
let _44: usize;
let _45: f32;
let _46: i16;
let _47: Adt44;
let _48: [usize; 2];
let _49: f64;
let _50: [usize; 8];
let _51: [u16; 1];
let _52: isize;
let _53: usize;
let _54: u32;
let _55: bool;
let _56: i32;
let _57: i64;
let _58: isize;
let _59: (*mut &'static i8,);
let _60: (f64, u8, (Adt35, *mut &'static i8), isize);
let _61: u16;
let _62: (*mut (*mut &'static i8,), u16);
let _63: f64;
let _64: [usize; 8];
let _65: f32;
let _66: i128;
let _67: *const (Adt27, *const u64, [u8; 5]);
let _68: (isize, u32, (Adt44, [u32; 3]), usize);
let _69: (i16, bool);
let _70: u8;
let _71: (Adt44, [u32; 3]);
let _72: bool;
let _73: isize;
let _74: *mut f64;
let _75: isize;
let _76: Adt59;
let _77: (i16, bool);
let _78: ();
let _79: ();
{
RET.1 = 18962_u16 * 26873_u16;
_6.1 = _4 >= _4;
_3 = _4;
_3 = !_6.1;
_6.1 = !_4;
_6 = Checked((-3319_i16) + (-6537_i16));
RET.1 = 52408_u16 - 43738_u16;
RET.1 = 181_u8 as u16;
RET.1 = 29027_u16;
_1 = -(-301597757_i32);
_4 = _3 | _3;
_3 = _6.1;
_5 = 6026857454030342759579495259195451576_u128 as f32;
_2 = ['\u{77a0f}','\u{9310d}','\u{8841b}'];
RET.1 = (-71_isize) as u16;
_3 = _4;
_6.1 = _3;
RET.1 = 14782_u16;
_4 = _6.1;
_6 = (23677_i16, _3);
_7 = &_6;
_4 = (*_7).1 == (*_7).1;
_6.0 = -(-23819_i16);
_1 = (-1021368845_i32);
Goto(bb1)
}
bb1 = {
_6.1 = !_3;
_8 = !_6.1;
RET.1 = 2250_u16 << _1;
RET.1 = 53596_u16;
_12.2 = [1261388205_u32,3171649869_u32,46325343_u32];
_1 = 183798610_i32 + 741076519_i32;
_12.3 = _4;
_13 = '\u{5e622}';
_1 = (-237252295_i32) - 774543090_i32;
_4 = !_8;
_11 = [_13,_13,_13];
Goto(bb2)
}
bb2 = {
_4 = _3 | _12.3;
RET.1 = 157_u8 as u16;
_3 = !_12.3;
_12.1 = [1576445555_u32,1367332617_u32,394352371_u32];
_8 = _3 >= _12.3;
_2 = _11;
_3 = !_8;
_12.3 = !_4;
_5 = 7_isize as f32;
_12.3 = _8 >= _3;
_12.1 = _12.2;
_5 = 112336924308125104255367718054667067322_u128 as f32;
_6 = Checked((-14603_i16) - 25682_i16);
_2 = _11;
_13 = '\u{e0966}';
_15 = [RET.1];
_16 = 1280194540_u32;
_12.1 = [_16,_16,_16];
_12.1 = [_16,_16,_16];
_19 = RET.1 as u8;
_4 = !_12.3;
_6.1 = _4;
Goto(bb3)
}
bb3 = {
_7 = &_6;
_12.3 = (*_7).1;
_20.2 = [_16,_16,_16];
_22 = core::ptr::addr_of_mut!(_4);
RET.1 = !48487_u16;
Call(_6.0 = fn11(Move(_22), (*_22), (*_22), _12.3, _3, (*_7).1, _3, (*_7).1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_20.2 = [_16,_16,_16];
_1 = 133178834_i32;
_20.3 = (*_7).1 | (*_7).1;
_6.0 = -(-592_i16);
RET.1 = 19469_u16 * 27140_u16;
_20.1 = _12.2;
_22 = core::ptr::addr_of_mut!((*_7).1);
_24 = _5;
_2 = [_13,_13,_13];
Goto(bb5)
}
bb5 = {
_21 = [(-117_isize),(-9223372036854775808_isize),9223372036854775807_isize,97_isize];
RET.1 = 165933123626123950325184929988285441044_u128 as u16;
_12.1 = [_16,_16,_16];
_17 = _13;
_14 = _2;
_9 = core::ptr::addr_of_mut!(_25);
_4 = !_8;
_12.1 = _12.2;
_12.2 = [_16,_16,_16];
_9 = core::ptr::addr_of_mut!((*_9));
(*_9) = 171894993659979755481471140538926435368_u128 as f64;
_16 = _13 as u32;
RET.1 = !5422_u16;
_2 = [_17,_13,_13];
_20.2 = _12.2;
_9 = core::ptr::addr_of_mut!(_25);
RET.1 = (*_22) as u16;
Goto(bb6)
}
bb6 = {
_11 = [_13,_17,_17];
_25 = 1962724600853902787_u64 as f64;
_27.fld2 = 276757417175593849333414037170141537385_u128 >> RET.1;
_8 = !_4;
_20.1 = [_16,_16,_16];
RET.1 = !40439_u16;
_27.fld1 = Move(_22);
_12.2 = [_16,_16,_16];
_19 = 57_u8 - 7_u8;
_14 = _2;
_9 = core::ptr::addr_of_mut!(_25);
_20.3 = (*_7).1 != (*_7).1;
_8 = (*_7).1 < _20.3;
_22 = core::ptr::addr_of_mut!(_4);
_27.fld1 = Move(_22);
_6 = Checked((-27245_i16) - (-20591_i16));
_13 = _17;
_27.fld3 = (*_9) * (*_9);
_11 = [_13,_17,_13];
_3 = !_20.3;
RET.1 = !16139_u16;
_21 = [68_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_27.fld3 = 15628979098929742929_u64 as f64;
(*_9) = -_27.fld3;
_8 = _20.3;
_5 = _24 + _24;
RET.1 = 34759_u16;
_3 = _12.3 == _12.3;
_9 = core::ptr::addr_of_mut!(_25);
match RET.1 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
34759 => bb15,
_ => bb14
}
}
bb7 = {
_21 = [(-117_isize),(-9223372036854775808_isize),9223372036854775807_isize,97_isize];
RET.1 = 165933123626123950325184929988285441044_u128 as u16;
_12.1 = [_16,_16,_16];
_17 = _13;
_14 = _2;
_9 = core::ptr::addr_of_mut!(_25);
_4 = !_8;
_12.1 = _12.2;
_12.2 = [_16,_16,_16];
_9 = core::ptr::addr_of_mut!((*_9));
(*_9) = 171894993659979755481471140538926435368_u128 as f64;
_16 = _13 as u32;
RET.1 = !5422_u16;
_2 = [_17,_13,_13];
_20.2 = _12.2;
_9 = core::ptr::addr_of_mut!(_25);
RET.1 = (*_22) as u16;
Goto(bb6)
}
bb8 = {
_20.2 = [_16,_16,_16];
_1 = 133178834_i32;
_20.3 = (*_7).1 | (*_7).1;
_6.0 = -(-592_i16);
RET.1 = 19469_u16 * 27140_u16;
_20.1 = _12.2;
_22 = core::ptr::addr_of_mut!((*_7).1);
_24 = _5;
_2 = [_13,_13,_13];
Goto(bb5)
}
bb9 = {
_7 = &_6;
_12.3 = (*_7).1;
_20.2 = [_16,_16,_16];
_22 = core::ptr::addr_of_mut!(_4);
RET.1 = !48487_u16;
Call(_6.0 = fn11(Move(_22), (*_22), (*_22), _12.3, _3, (*_7).1, _3, (*_7).1), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
_4 = _3 | _12.3;
RET.1 = 157_u8 as u16;
_3 = !_12.3;
_12.1 = [1576445555_u32,1367332617_u32,394352371_u32];
_8 = _3 >= _12.3;
_2 = _11;
_3 = !_8;
_12.3 = !_4;
_5 = 7_isize as f32;
_12.3 = _8 >= _3;
_12.1 = _12.2;
_5 = 112336924308125104255367718054667067322_u128 as f32;
_6 = Checked((-14603_i16) - 25682_i16);
_2 = _11;
_13 = '\u{e0966}';
_15 = [RET.1];
_16 = 1280194540_u32;
_12.1 = [_16,_16,_16];
_12.1 = [_16,_16,_16];
_19 = RET.1 as u8;
_4 = !_12.3;
_6.1 = _4;
Goto(bb3)
}
bb11 = {
_6.1 = !_3;
_8 = !_6.1;
RET.1 = 2250_u16 << _1;
RET.1 = 53596_u16;
_12.2 = [1261388205_u32,3171649869_u32,46325343_u32];
_1 = 183798610_i32 + 741076519_i32;
_12.3 = _4;
_13 = '\u{5e622}';
_1 = (-237252295_i32) - 774543090_i32;
_4 = !_8;
_11 = [_13,_13,_13];
Goto(bb2)
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
_7 = &_6;
_21 = [(-9223372036854775808_isize),19_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_21 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-107_isize),9223372036854775807_isize];
_2 = [_13,_17,_17];
_25 = _27.fld3 + _27.fld3;
_4 = !_20.3;
_20.3 = _4 | _4;
_15 = [RET.1];
_27.fld0 = 17801388028688080539_usize;
_6.1 = _8 < _3;
_6.1 = !_3;
_15 = [RET.1];
_20.2 = [_16,_16,_16];
_3 = _8;
_16 = RET.1 as u32;
_22 = core::ptr::addr_of_mut!(_12.3);
_27.fld2 = !241709580532825078392982134038515091703_u128;
_27.fld1 = core::ptr::addr_of_mut!(_8);
_7 = &_6;
Goto(bb16)
}
bb16 = {
_6.0 = (-8965464869614080843_i64) as i16;
_5 = _24 + _24;
_6.1 = (*_22) < (*_22);
_36.0 = (-123_i8) << _1;
_33 = _27.fld0 as f32;
_3 = _4 <= _4;
_33 = _5 * _24;
_2 = [_17,_17,_13];
_37 = core::ptr::addr_of_mut!(_13);
_31 = 9223372036854775807_isize * (-17_isize);
RET.1 = 44578_u16;
match RET.1 {
44578 => bb17,
_ => bb2
}
}
bb17 = {
_8 = !_6.1;
_8 = _4 | _6.1;
_6.0 = (-26332_i16);
_39 = &_36.0;
_38.1 = &_19;
_43 = _13;
_20.0 = &_39;
_20.0 = &_39;
_35 = _16;
_44 = !_27.fld0;
_1 = 148315713_i32;
match _6.0 {
340282366920938463463374607431768185124 => bb19,
_ => bb18
}
}
bb18 = {
Return()
}
bb19 = {
_35 = (-11133538541886534964016486426793802357_i128) as u32;
_4 = (*_22) >= _8;
_38.0 = (Move(_37),);
_20.3 = _8;
_36.0 = 72_i8;
_36 = (33_i8, _25);
_2 = _14;
_7 = &_6;
_42 = !_1;
_38.3 = &_39;
_31 = (-72_isize);
_41 = &_40;
_20 = (Move(_38.3), _12.1, _12.2, (*_7).1);
_40 = core::ptr::addr_of!(_27.fld0);
(*_9) = _42 as f64;
(*_9) = 566594588156869728_u64 as f64;
(*_9) = -_36.1;
_34 = !_19;
_20.1 = [_16,_35,_16];
_15 = [RET.1];
(*_22) = _6.1 ^ (*_7).1;
match (*_7).0 {
340282366920938463463374607431768185124 => bb21,
_ => bb20
}
}
bb20 = {
Return()
}
bb21 = {
_20.0 = &_39;
_12.2 = _20.1;
_3 = !(*_7).1;
_24 = _33 * _5;
match (*_7).0 {
0 => bb16,
1 => bb18,
2 => bb19,
3 => bb20,
4 => bb10,
5 => bb12,
6 => bb22,
340282366920938463463374607431768185124 => bb24,
_ => bb23
}
}
bb22 = {
_7 = &_6;
_21 = [(-9223372036854775808_isize),19_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_21 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-107_isize),9223372036854775807_isize];
_2 = [_13,_17,_17];
_25 = _27.fld3 + _27.fld3;
_4 = !_20.3;
_20.3 = _4 | _4;
_15 = [RET.1];
_27.fld0 = 17801388028688080539_usize;
_6.1 = _8 < _3;
_6.1 = !_3;
_15 = [RET.1];
_20.2 = [_16,_16,_16];
_3 = _8;
_16 = RET.1 as u32;
_22 = core::ptr::addr_of_mut!(_12.3);
_27.fld2 = !241709580532825078392982134038515091703_u128;
_27.fld1 = core::ptr::addr_of_mut!(_8);
_7 = &_6;
Goto(bb16)
}
bb23 = {
_35 = (-11133538541886534964016486426793802357_i128) as u32;
_4 = (*_22) >= _8;
_38.0 = (Move(_37),);
_20.3 = _8;
_36.0 = 72_i8;
_36 = (33_i8, _25);
_2 = _14;
_7 = &_6;
_42 = !_1;
_38.3 = &_39;
_31 = (-72_isize);
_41 = &_40;
_20 = (Move(_38.3), _12.1, _12.2, (*_7).1);
_40 = core::ptr::addr_of!(_27.fld0);
(*_9) = _42 as f64;
(*_9) = 566594588156869728_u64 as f64;
(*_9) = -_36.1;
_34 = !_19;
_20.1 = [_16,_35,_16];
_15 = [RET.1];
(*_22) = _6.1 ^ (*_7).1;
match (*_7).0 {
340282366920938463463374607431768185124 => bb21,
_ => bb20
}
}
bb24 = {
_15 = [RET.1];
_27.fld0 = _44;
_38.1 = &_34;
(*_9) = _36.1 - _36.1;
_3 = (*_22) <= (*_7).1;
_6 = ((-23773_i16), _4);
_3 = _8;
_32 = _6.0 as f64;
_13 = _43;
(*_40) = _44;
RET.1 = !6028_u16;
_27.fld3 = _5 as f64;
(*_22) = !_8;
_7 = &_6;
_35 = _16;
_19 = _34;
_39 = &_36.0;
_20.2 = [_35,_35,_16];
_45 = -_24;
match _6.0 {
0 => bb15,
1 => bb2,
2 => bb12,
3 => bb20,
4 => bb8,
5 => bb23,
6 => bb14,
340282366920938463463374607431768187683 => bb26,
_ => bb25
}
}
bb25 = {
_6.0 = (-8965464869614080843_i64) as i16;
_5 = _24 + _24;
_6.1 = (*_22) < (*_22);
_36.0 = (-123_i8) << _1;
_33 = _27.fld0 as f32;
_3 = _4 <= _4;
_33 = _5 * _24;
_2 = [_17,_17,_13];
_37 = core::ptr::addr_of_mut!(_13);
_31 = 9223372036854775807_isize * (-17_isize);
RET.1 = 44578_u16;
match RET.1 {
44578 => bb17,
_ => bb2
}
}
bb26 = {
_12.0 = &_39;
_12.0 = &_39;
_12.0 = &_39;
_37 = core::ptr::addr_of_mut!(_17);
_15 = [RET.1];
_12.2 = [_16,_35,_35];
_28 = &_39;
_14 = _11;
_38.0 = (Move(_37),);
Goto(bb27)
}
bb27 = {
_36.0 = (-94_i8) - (-5_i8);
_48 = [(*_40),(*_40)];
Call(_27 = fn14(Move(_7), Move(_28), (*_7).0, _6, Move(_22)), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
RET.1 = 2034_u16 & 60457_u16;
_38.0.0 = core::ptr::addr_of_mut!(_13);
Goto(bb29)
}
bb29 = {
_27.fld1 = core::ptr::addr_of_mut!(_3);
_20.0 = &_39;
_22 = Move(_27.fld1);
_27 = Adt59 { fld0: _44,fld1: Move(_22),fld2: 156160880252532882493304918394995695426_u128,fld3: _32 };
_7 = &_6;
(*_9) = _27.fld3;
_49 = _27.fld3;
_12.3 = !_6.1;
match _6.0 {
0 => bb30,
1 => bb31,
2 => bb32,
3 => bb33,
340282366920938463463374607431768187683 => bb35,
_ => bb34
}
}
bb30 = {
Return()
}
bb31 = {
_7 = &_6;
_21 = [(-9223372036854775808_isize),19_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_21 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-107_isize),9223372036854775807_isize];
_2 = [_13,_17,_17];
_25 = _27.fld3 + _27.fld3;
_4 = !_20.3;
_20.3 = _4 | _4;
_15 = [RET.1];
_27.fld0 = 17801388028688080539_usize;
_6.1 = _8 < _3;
_6.1 = !_3;
_15 = [RET.1];
_20.2 = [_16,_16,_16];
_3 = _8;
_16 = RET.1 as u32;
_22 = core::ptr::addr_of_mut!(_12.3);
_27.fld2 = !241709580532825078392982134038515091703_u128;
_27.fld1 = core::ptr::addr_of_mut!(_8);
_7 = &_6;
Goto(bb16)
}
bb32 = {
_21 = [(-117_isize),(-9223372036854775808_isize),9223372036854775807_isize,97_isize];
RET.1 = 165933123626123950325184929988285441044_u128 as u16;
_12.1 = [_16,_16,_16];
_17 = _13;
_14 = _2;
_9 = core::ptr::addr_of_mut!(_25);
_4 = !_8;
_12.1 = _12.2;
_12.2 = [_16,_16,_16];
_9 = core::ptr::addr_of_mut!((*_9));
(*_9) = 171894993659979755481471140538926435368_u128 as f64;
_16 = _13 as u32;
RET.1 = !5422_u16;
_2 = [_17,_13,_13];
_20.2 = _12.2;
_9 = core::ptr::addr_of_mut!(_25);
RET.1 = (*_22) as u16;
Goto(bb6)
}
bb33 = {
Return()
}
bb34 = {
_15 = [RET.1];
_27.fld0 = _44;
_38.1 = &_34;
(*_9) = _36.1 - _36.1;
_3 = (*_22) <= (*_7).1;
_6 = ((-23773_i16), _4);
_3 = _8;
_32 = _6.0 as f64;
_13 = _43;
(*_40) = _44;
RET.1 = !6028_u16;
_27.fld3 = _5 as f64;
(*_22) = !_8;
_7 = &_6;
_35 = _16;
_19 = _34;
_39 = &_36.0;
_20.2 = [_35,_35,_16];
_45 = -_24;
match _6.0 {
0 => bb15,
1 => bb2,
2 => bb12,
3 => bb20,
4 => bb8,
5 => bb23,
6 => bb14,
340282366920938463463374607431768187683 => bb26,
_ => bb25
}
}
bb35 = {
_45 = _24 * _24;
_2 = _11;
_12.1 = [_16,_35,_35];
_20.2 = [_16,_16,_16];
_20.3 = !_3;
_45 = _24;
(*_9) = _27.fld3 - _49;
_36.0 = (-54_i8);
_12 = (Move(_20.0), _20.2, _20.1, _8);
_24 = _27.fld2 as f32;
RET.1 = 43537_u16;
_38.1 = &_34;
_19 = (-5685931620343431168_i64) as u8;
_20.2 = _12.1;
_36.1 = (*_9) + (*_9);
_37 = core::ptr::addr_of_mut!(_17);
_17 = _43;
_20.2 = [_35,_35,_16];
_36.0 = 19_i8;
_38.2 = Move(_37);
_46 = (*_7).0 >> (*_7).0;
_27.fld1 = core::ptr::addr_of_mut!(_8);
(*_9) = RET.1 as f64;
_36.0 = !49_i8;
_12.0 = &_39;
Goto(bb36)
}
bb36 = {
_38.0.0 = core::ptr::addr_of_mut!(_43);
_20.2 = _12.2;
_21 = [_31,_31,_31,_31];
_42 = _1 >> (*_7).0;
_25 = _36.1 - _49;
_33 = (-3073632838868776398_i64) as f32;
_50 = [(*_40),_44,_27.fld0,(*_40),(*_40),(*_40),(*_40),_44];
_55 = !(*_7).1;
_38.1 = &_19;
_27.fld0 = !_44;
_48 = [_44,(*_40)];
_54 = !_35;
_35 = _16;
_38.1 = &_19;
RET.1 = 46866_u16;
_57 = 3121867199192087695_i64 * 3560035876552154115_i64;
_34 = !_19;
_42 = _1 * _1;
Goto(bb37)
}
bb37 = {
_31 = !10_isize;
_11 = _2;
_38.3 = &_39;
_14 = _2;
(*_9) = _32;
_42 = -_1;
_25 = -_49;
_31 = 117_isize - (-9223372036854775808_isize);
_12.0 = &_39;
_12 = (Move(_38.3), _20.1, _20.1, _20.3);
_27.fld1 = core::ptr::addr_of_mut!(_6.1);
_45 = -_24;
_27.fld0 = _44 | _44;
_55 = _4;
_38.1 = &_34;
_20.0 = &_39;
_38.1 = &_34;
_55 = _12.3 < _4;
_37 = Move(_38.2);
_20.1 = [_35,_54,_54];
_28 = &_39;
match (*_7).0 {
0 => bb38,
1 => bb39,
2 => bb40,
3 => bb41,
340282366920938463463374607431768187683 => bb43,
_ => bb42
}
}
bb38 = {
Return()
}
bb39 = {
_20.2 = [_16,_16,_16];
_1 = 133178834_i32;
_20.3 = (*_7).1 | (*_7).1;
_6.0 = -(-592_i16);
RET.1 = 19469_u16 * 27140_u16;
_20.1 = _12.2;
_22 = core::ptr::addr_of_mut!((*_7).1);
_24 = _5;
_2 = [_13,_13,_13];
Goto(bb5)
}
bb40 = {
_15 = [RET.1];
_27.fld0 = _44;
_38.1 = &_34;
(*_9) = _36.1 - _36.1;
_3 = (*_22) <= (*_7).1;
_6 = ((-23773_i16), _4);
_3 = _8;
_32 = _6.0 as f64;
_13 = _43;
(*_40) = _44;
RET.1 = !6028_u16;
_27.fld3 = _5 as f64;
(*_22) = !_8;
_7 = &_6;
_35 = _16;
_19 = _34;
_39 = &_36.0;
_20.2 = [_35,_35,_16];
_45 = -_24;
match _6.0 {
0 => bb15,
1 => bb2,
2 => bb12,
3 => bb20,
4 => bb8,
5 => bb23,
6 => bb14,
340282366920938463463374607431768187683 => bb26,
_ => bb25
}
}
bb41 = {
Return()
}
bb42 = {
_20.2 = [_16,_16,_16];
_1 = 133178834_i32;
_20.3 = (*_7).1 | (*_7).1;
_6.0 = -(-592_i16);
RET.1 = 19469_u16 * 27140_u16;
_20.1 = _12.2;
_22 = core::ptr::addr_of_mut!((*_7).1);
_24 = _5;
_2 = [_13,_13,_13];
Goto(bb5)
}
bb43 = {
_25 = _36.1;
_50 = [(*_40),_27.fld0,(*_40),(*_40),(*_40),_44,(*_40),(*_40)];
_17 = _43;
_6.0 = _24 as i16;
RET.1 = 58592_u16;
_2 = [_13,_43,_43];
_42 = !_1;
Goto(bb44)
}
bb44 = {
_59.0 = core::ptr::addr_of_mut!((*_28));
RET.1 = !8017_u16;
_60.2.0 = Adt35::Variant0 { fld0: (-125391657992457986424886464998400904411_i128),fld1: _27.fld2,fld2: _31,fld3: _45,fld4: _6.0,fld5: _50,fld6: _48 };
_62.0 = core::ptr::addr_of_mut!(_59);
_26 = core::ptr::addr_of!(_60.2.0);
_12.3 = !_20.3;
_60.2.1 = core::ptr::addr_of_mut!(_39);
(*_9) = _32 - _36.1;
_12.1 = _20.2;
_60.0 = -_25;
place!(Field::<[usize; 2]>(Variant((*_26), 0), 6)) = [_44,(*_40)];
place!(Field::<f32>(Variant((*_26), 0), 3)) = _24 + _45;
_62.1 = !RET.1;
place!(Field::<[usize; 8]>(Variant(_60.2.0, 0), 5)) = [(*_40),(*_40),(*_40),(*_40),_27.fld0,(*_40),(*_40),_27.fld0];
_64 = [_27.fld0,(*_40),(*_40),_27.fld0,(*_40),(*_40),_27.fld0,(*_40)];
_12.0 = &(*_28);
match Field::<u128>(Variant((*_26), 0), 1) {
0 => bb45,
156160880252532882493304918394995695426 => bb47,
_ => bb46
}
}
bb45 = {
_21 = [(-117_isize),(-9223372036854775808_isize),9223372036854775807_isize,97_isize];
RET.1 = 165933123626123950325184929988285441044_u128 as u16;
_12.1 = [_16,_16,_16];
_17 = _13;
_14 = _2;
_9 = core::ptr::addr_of_mut!(_25);
_4 = !_8;
_12.1 = _12.2;
_12.2 = [_16,_16,_16];
_9 = core::ptr::addr_of_mut!((*_9));
(*_9) = 171894993659979755481471140538926435368_u128 as f64;
_16 = _13 as u32;
RET.1 = !5422_u16;
_2 = [_17,_13,_13];
_20.2 = _12.2;
_9 = core::ptr::addr_of_mut!(_25);
RET.1 = (*_22) as u16;
Goto(bb6)
}
bb46 = {
Return()
}
bb47 = {
_26 = core::ptr::addr_of!(_60.2.0);
_26 = core::ptr::addr_of!((*_26));
_41 = &_40;
_53 = !(*_40);
place!(Field::<f32>(Variant((*_26), 0), 3)) = _45;
_53 = (*_40);
place!(Field::<u128>(Variant(_60.2.0, 0), 1)) = (*_40) as u128;
_27.fld0 = !_44;
_38.2 = Move(_37);
_33 = Field::<f32>(Variant((*_26), 0), 3);
place!(Field::<i16>(Variant((*_26), 0), 4)) = -_46;
_68.3 = _19 as usize;
_60.0 = (*_9);
_27.fld3 = _49;
_11 = [_17,_43,_13];
_38.0.0 = core::ptr::addr_of_mut!(_13);
_51 = _15;
Call(_52 = fn15(Move(_12), Move(_20)), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
_37 = core::ptr::addr_of_mut!(_17);
_66 = 119008949586527790268275348407218078060_i128;
RET.1 = _62.1;
RET.0 = core::ptr::addr_of_mut!(_59);
_20.1 = [_16,_35,_16];
place!(Field::<i128>(Variant((*_26), 0), 0)) = _66 >> _6.0;
place!(Field::<[usize; 8]>(Variant(_60.2.0, 0), 5)) = [_44,_68.3,_53,_68.3,_27.fld0,_68.3,(*_40),(*_40)];
place!(Field::<isize>(Variant((*_26), 0), 2)) = Field::<i128>(Variant((*_26), 0), 0) as isize;
_27.fld1 = core::ptr::addr_of_mut!(_55);
_12.1 = _20.1;
_27.fld2 = !Field::<u128>(Variant(_60.2.0, 0), 1);
_3 = _55 | (*_7).1;
_36 = ((-86_i8), _25);
_31 = Field::<isize>(Variant((*_26), 0), 2) * Field::<isize>(Variant((*_26), 0), 2);
(*_37) = _13;
_71.1 = [_16,_54,_54];
_60.2.1 = core::ptr::addr_of_mut!(_39);
SetDiscriminant((*_26), 0);
Goto(bb49)
}
bb49 = {
Call(_78 = dump_var(10_usize, 52_usize, Move(_52), 16_usize, Move(_16), 50_usize, Move(_50), 66_usize, Move(_66)), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
Call(_78 = dump_var(10_usize, 13_usize, Move(_13), 54_usize, Move(_54), 44_usize, Move(_44), 31_usize, Move(_31)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_78 = dump_var(10_usize, 15_usize, Move(_15), 21_usize, Move(_21), 48_usize, Move(_48), 4_usize, Move(_4)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_78 = dump_var(10_usize, 2_usize, Move(_2), 46_usize, Move(_46), 17_usize, Move(_17), 79_usize, _79), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: *mut bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool) -> i16 {
mir! {
type RET = i16;
let _9: [isize; 4];
let _10: &'static bool;
let _11: *const *mut [u16; 7];
let _12: &'static i8;
let _13: bool;
let _14: &'static *const usize;
let _15: isize;
let _16: isize;
let _17: &'static &'static i8;
let _18: *mut *const (Adt27, *const u64, [u8; 5]);
let _19: u16;
let _20: bool;
let _21: [u16; 1];
let _22: [u8; 5];
let _23: [isize; 4];
let _24: i8;
let _25: char;
let _26: *const [u16; 7];
let _27: i8;
let _28: isize;
let _29: isize;
let _30: Adt35;
let _31: u128;
let _32: char;
let _33: bool;
let _34: ();
let _35: ();
{
_3 = !_6;
RET = !(-4282_i16);
RET = 18752_i16;
_1 = core::ptr::addr_of_mut!(_8);
(*_1) = _6;
RET = 119_i16 * 8623_i16;
_8 = _4;
_3 = !_2;
_2 = _4;
_3 = _5 & _8;
_7 = _8;
_9 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
RET = (-31320_i16);
_2 = !_8;
_3 = _6;
_1 = core::ptr::addr_of_mut!((*_1));
RET = 53658077601430025657088652786738139436_i128 as i16;
_2 = !_5;
_10 = &_8;
RET = 82_u8 as i16;
Goto(bb1)
}
bb1 = {
(*_1) = _2;
_2 = _7;
_10 = &_6;
_1 = core::ptr::addr_of_mut!(_4);
_3 = _5;
_2 = (*_10) != _4;
_3 = (*_10) == (*_10);
_3 = (*_10) >= (*_10);
_3 = !(*_1);
_7 = _6 <= _3;
(*_1) = !(*_10);
_8 = _3 != _5;
_6 = _3;
_3 = !(*_1);
_6 = !(*_1);
_8 = !(*_1);
_3 = !_6;
_15 = 9223372036854775807_isize + 18_isize;
_1 = core::ptr::addr_of_mut!(_4);
RET = -15282_i16;
Goto(bb2)
}
bb2 = {
RET = -(-17803_i16);
_4 = _3;
_3 = (*_1);
Goto(bb3)
}
bb3 = {
_10 = &_8;
_3 = _4;
_6 = !(*_1);
_13 = _4 < _8;
RET = 17331_i16 ^ (-22454_i16);
Goto(bb4)
}
bb4 = {
_9 = [_15,_15,_15,_15];
_17 = &_12;
_1 = core::ptr::addr_of_mut!(_6);
_10 = &(*_10);
_13 = (*_10) > _3;
_1 = core::ptr::addr_of_mut!(_4);
_1 = core::ptr::addr_of_mut!((*_10));
_10 = &_7;
_6 = !_8;
_5 = _13 & _6;
_19 = 1413643654850517030_usize as u16;
_22 = [82_u8,54_u8,146_u8,124_u8,24_u8];
_1 = core::ptr::addr_of_mut!(_13);
_19 = 36699_u16 ^ 23697_u16;
_16 = _15;
_13 = !_7;
(*_1) = (*_10) ^ _6;
_10 = &(*_1);
_16 = -_15;
RET = 31840_i16 << _16;
_4 = (*_10);
Goto(bb5)
}
bb5 = {
RET = 22419_i16 & 6212_i16;
RET = 993961578_i32 as i16;
_7 = _8 ^ _6;
_21 = [_19];
_5 = _4;
_1 = core::ptr::addr_of_mut!(_8);
_24 = (-10_i8) ^ (-30_i8);
_6 = (*_1) != _13;
Goto(bb6)
}
bb6 = {
_2 = _4 >= _6;
_24 = 69_i8;
_25 = '\u{bb8e0}';
RET = 7247_i16 >> _15;
_3 = !_8;
_1 = core::ptr::addr_of_mut!(_5);
_2 = (*_10) <= _3;
(*_1) = _4;
_19 = 6633_u16;
_8 = (*_10) <= _2;
_1 = core::ptr::addr_of_mut!(_5);
_1 = core::ptr::addr_of_mut!(_13);
_1 = core::ptr::addr_of_mut!(_6);
_23 = [_15,_15,_16,_16];
_4 = _6 == _3;
RET = 8538_i16 - (-24271_i16);
_8 = !_3;
_19 = _25 as u16;
_25 = '\u{575bb}';
_12 = &_24;
_13 = !_3;
_12 = &_24;
_1 = core::ptr::addr_of_mut!(_4);
_7 = !_5;
_16 = _15 * _15;
_1 = core::ptr::addr_of_mut!(_6);
_15 = _16 * _16;
Call(_20 = fn12((*_1), _7, _4, _13, _2, _4, _2, _3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
(*_1) = !_20;
_2 = _6 ^ _6;
(*_1) = !_8;
_7 = _13;
_28 = 218_u8 as isize;
_20 = (*_1);
_27 = _25 as i8;
_19 = 9546_u16;
_3 = _6 > _20;
_20 = _4 >= (*_1);
_8 = !_13;
_21 = [_19];
_20 = !_4;
_2 = _13;
(*_1) = !_4;
_32 = _25;
_13 = _5;
Call(_29 = fn13(_20, _20, _3, _7, _2, _20), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_4 = _8 >= _20;
_8 = (*_1);
Goto(bb9)
}
bb9 = {
_28 = (-225743530_i32) as isize;
_19 = 15791_u16;
_7 = _15 > _15;
_27 = (*_12);
_28 = !_15;
_27 = -_24;
_3 = !_20;
RET = _20 as i16;
_1 = core::ptr::addr_of_mut!(_33);
_12 = &_27;
_21 = [_19];
_17 = &_12;
Goto(bb10)
}
bb10 = {
Call(_34 = dump_var(11_usize, 8_usize, Move(_8), 15_usize, Move(_15), 27_usize, Move(_27), 20_usize, Move(_20)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_34 = dump_var(11_usize, 21_usize, Move(_21), 28_usize, Move(_28), 4_usize, Move(_4), 24_usize, Move(_24)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_34 = dump_var(11_usize, 2_usize, Move(_2), 16_usize, Move(_16), 29_usize, Move(_29), 35_usize, _35), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool) -> bool {
mir! {
type RET = bool;
let _9: [u32; 3];
let _10: isize;
let _11: [isize; 4];
let _12: [u16; 7];
let _13: Adt59;
let _14: ();
let _15: ();
{
RET = _5;
_3 = _6;
_2 = _6 | _6;
_7 = !_2;
_10 = (-9223372036854775808_isize);
_11 = [_10,_10,_10,_10];
RET = _5 & _8;
_2 = !_6;
_2 = !_8;
_13.fld2 = !238108027604969388111779681589660646511_u128;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(12_usize, 11_usize, Move(_11), 5_usize, Move(_5), 10_usize, Move(_10), 1_usize, Move(_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_14 = dump_var(12_usize, 7_usize, Move(_7), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool) -> isize {
mir! {
type RET = isize;
let _7: *const u64;
let _8: &'static [usize; 2];
let _9: i64;
let _10: char;
let _11: &'static [u16; 1];
let _12: [u8; 1];
let _13: u128;
let _14: [u8; 1];
let _15: &'static *mut [u16; 7];
let _16: ((i32,), (i32,), u32, (&'static &'static i8, [u32; 3], [u32; 3], bool));
let _17: i128;
let _18: (f64, u8, (Adt35, *mut &'static i8), isize);
let _19: f64;
let _20: isize;
let _21: *const (Adt27, *const u64, [u8; 5]);
let _22: u128;
let _23: u64;
let _24: f64;
let _25: (i32,);
let _26: (*const u64, (i8, f64), [u16; 7], bool);
let _27: ();
let _28: ();
{
_2 = !_4;
_5 = _3 == _6;
_2 = !_6;
_4 = _5;
_4 = _1 == _3;
RET = 9223372036854775807_isize;
_5 = _6 != _4;
_5 = _2;
_5 = _4 | _2;
_5 = _3 | _3;
_5 = _1;
_5 = !_2;
_4 = _3;
_3 = !_1;
_9 = !2939253639917671682_i64;
_9 = (-15_i8) as i64;
_5 = !_2;
_10 = '\u{4873f}';
RET = -(-33_isize);
_4 = _3;
_2 = _4 >= _6;
_2 = _6 <= _3;
RET = (-44_isize);
_3 = _5 ^ _4;
Goto(bb1)
}
bb1 = {
_4 = !_1;
_9 = 5_usize as i64;
_12 = [42_u8];
_5 = _1 < _2;
_2 = !_5;
_6 = _5;
RET = -(-124_isize);
_6 = !_3;
_1 = !_5;
_6 = !_4;
_10 = '\u{107094}';
_12 = [88_u8];
_9 = -5090409302880460391_i64;
RET = (-111_isize);
_12 = [168_u8];
_13 = 1287454106_u32 as u128;
_9 = 1955306972927791343_i64;
_5 = !_6;
RET = !(-9223372036854775808_isize);
Goto(bb2)
}
bb2 = {
_4 = _5 < _3;
Goto(bb3)
}
bb3 = {
RET = 13_isize & (-46_isize);
_2 = !_3;
_9 = (-5994140433641815328_i64) << _13;
_1 = _4;
_1 = _4;
_4 = !_5;
Goto(bb4)
}
bb4 = {
_3 = !_4;
_2 = _5 <= _4;
_5 = _6 ^ _2;
Goto(bb5)
}
bb5 = {
_1 = _2;
_12 = [77_u8];
_10 = '\u{21d56}';
Goto(bb6)
}
bb6 = {
RET = 9223372036854775807_isize;
_10 = '\u{7dc57}';
_14 = [11_u8];
_2 = _5 == _3;
_16.0 = (1161674332_i32,);
_16.1.0 = _16.0.0;
_16.1 = (_16.0.0,);
_16.3.3 = !_4;
_17 = !2895941640502145805918007640212027537_i128;
_16.3.1 = [1009006070_u32,3147045752_u32,849347787_u32];
_2 = !_3;
_3 = _2 != _16.3.3;
_17 = 9591_i16 as i128;
_16.2 = 2050590344_u32;
_14 = [93_u8];
_12 = _14;
_16.0 = (_16.1.0,);
_16.3.2 = _16.3.1;
_18.0 = (-15532_i16) as f64;
_18.3 = !RET;
_19 = _18.0 + _18.0;
_16.1.0 = !_16.0.0;
Goto(bb7)
}
bb7 = {
_16.1 = _16.0;
_4 = _1 != _2;
_10 = '\u{35831}';
_16.0 = (_16.1.0,);
_16.1.0 = -_16.0.0;
RET = _18.3;
_16.0.0 = _10 as i32;
_18.1 = 19194_i16 as u8;
_18.3 = !RET;
_10 = '\u{7784f}';
_3 = !_5;
_4 = _3 | _1;
_16.0 = (_16.1.0,);
_18.3 = !RET;
_9 = 3484383659020319395_i64;
_6 = !_5;
_16.3.1 = _16.3.2;
_10 = '\u{109e14}';
_3 = _6;
_20 = RET | _18.3;
_18.0 = 60893076241549245_u64 as f64;
_6 = _16.3.3 <= _16.3.3;
_9 = (-3907764963364411487_i64) & (-9218232857237070926_i64);
match _16.2 {
0 => bb5,
1 => bb3,
2 => bb8,
2050590344 => bb10,
_ => bb9
}
}
bb8 = {
_4 = _5 < _3;
Goto(bb3)
}
bb9 = {
_4 = !_1;
_9 = 5_usize as i64;
_12 = [42_u8];
_5 = _1 < _2;
_2 = !_5;
_6 = _5;
RET = -(-124_isize);
_6 = !_3;
_1 = !_5;
_6 = !_4;
_10 = '\u{107094}';
_12 = [88_u8];
_9 = -5090409302880460391_i64;
RET = (-111_isize);
_12 = [168_u8];
_13 = 1287454106_u32 as u128;
_9 = 1955306972927791343_i64;
_5 = !_6;
RET = !(-9223372036854775808_isize);
Goto(bb2)
}
bb10 = {
_12 = _14;
_9 = _16.2 as i64;
_4 = !_6;
_16.0 = (_16.1.0,);
_16.0.0 = _16.1.0 & _16.1.0;
_22 = _13 | _13;
_16.1.0 = !_16.0.0;
_7 = core::ptr::addr_of!(_23);
_9 = (-8226339934004209805_i64);
match _9 {
340282366920938463455148267497764001651 => bb12,
_ => bb11
}
}
bb11 = {
RET = 13_isize & (-46_isize);
_2 = !_3;
_9 = (-5994140433641815328_i64) << _13;
_1 = _4;
_1 = _4;
_4 = !_5;
Goto(bb4)
}
bb12 = {
_18.3 = _4 as isize;
_16.3.1 = _16.3.2;
match _16.2 {
0 => bb11,
2050590344 => bb13,
_ => bb7
}
}
bb13 = {
_23 = _10 as u64;
_16.3.3 = _5;
_23 = 1757321976846371188_u64 << _18.3;
_6 = _4;
RET = _18.3;
_16.3.1 = [_16.2,_16.2,_16.2];
_16.1 = (_16.0.0,);
_24 = _23 as f64;
_23 = (-28157_i16) as u64;
_5 = _4;
_26.1.0 = _22 as i8;
_20 = RET;
_16.3.3 = _3;
_26.2 = [36656_u16,14984_u16,14843_u16,52852_u16,17171_u16,10868_u16,59895_u16];
RET = _18.1 as isize;
_6 = _3 >= _16.3.3;
_16.0 = _16.1;
match _9 {
0 => bb9,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb12,
6 => bb7,
340282366920938463455148267497764001651 => bb15,
_ => bb14
}
}
bb14 = {
_1 = _2;
_12 = [77_u8];
_10 = '\u{21d56}';
Goto(bb6)
}
bb15 = {
_22 = _13 | _13;
_25 = (_16.1.0,);
_26.1.1 = -_24;
_4 = _5;
_16.1.0 = !_16.0.0;
_16.2 = 62609_u16 as u32;
_26.1.0 = (-87_i8);
_7 = core::ptr::addr_of!((*_7));
_9 = !5611162950062897333_i64;
_16.0 = (_16.1.0,);
_14 = [_18.1];
_13 = !_22;
_18.3 = _20 << _20;
_17 = 114106008882943088816205556027926789315_i128 - (-36308022763479815173660824373281299943_i128);
_16.1.0 = _16.0.0 & _16.0.0;
_16.3.1 = [_16.2,_16.2,_16.2];
Goto(bb16)
}
bb16 = {
Call(_27 = dump_var(13_usize, 4_usize, Move(_4), 22_usize, Move(_22), 13_usize, Move(_13), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_27 = dump_var(13_usize, 10_usize, Move(_10), 12_usize, Move(_12), 6_usize, Move(_6), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: &'static (i16, bool),mut _2: &'static &'static i8,mut _3: i16,mut _4: (i16, bool),mut _5: *mut bool) -> Adt59 {
mir! {
type RET = Adt59;
let _6: ();
let _7: ();
{
RET.fld0 = 17869652959076320918_usize;
RET.fld3 = (-1568962869622777060_i64) as f64;
RET.fld1 = Move(_5);
_4 = Checked(_3 * _3);
RET.fld2 = 10016_u16 as u128;
_3 = _4.0 - _4.0;
_4.0 = !_3;
RET.fld2 = 103320205919349419708878694160834708741_u128 << _4.0;
_4.1 = !true;
Goto(bb1)
}
bb1 = {
Call(_6 = dump_var(14_usize, 3_usize, Move(_3), 7_usize, _7, 7_usize, _7, 7_usize, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: (&'static &'static i8, [u32; 3], [u32; 3], bool),mut _2: (&'static &'static i8, [u32; 3], [u32; 3], bool)) -> isize {
mir! {
type RET = isize;
let _3: isize;
let _4: isize;
let _5: char;
let _6: *mut *const &'static u8;
let _7: i128;
let _8: (*mut char,);
let _9: [usize; 8];
let _10: isize;
let _11: isize;
let _12: ();
let _13: ();
{
_1.1 = [2375438407_u32,2545018193_u32,28713645_u32];
RET = 1583_i16 as isize;
_4 = (-9211_i16) as isize;
_4 = -RET;
_3 = !_4;
RET = -_4;
_3 = _4;
_4 = RET ^ _3;
_1.2 = [4022759660_u32,2205524991_u32,1275595754_u32];
_1.3 = _2.3 ^ _2.3;
_4 = _3 ^ _3;
_1.3 = _2.3 == _2.3;
_3 = _4;
_2.2 = [1025519073_u32,2876919876_u32,3913623719_u32];
Call(RET = core::intrinsics::transmute(_3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2.2 = [2071465781_u32,1106188887_u32,1086702741_u32];
_2.2 = [515267685_u32,1693986687_u32,784799603_u32];
RET = _3;
RET = _3 & _4;
Goto(bb2)
}
bb2 = {
_1.1 = _1.2;
_4 = -_3;
_1.3 = _2.3 > _2.3;
_7 = (-2095354695549061435_i64) as i128;
_5 = '\u{5495}';
RET = _3 | _4;
_4 = RET + RET;
_2.3 = _1.3 >= _1.3;
_1.3 = _2.3;
_8.0 = core::ptr::addr_of_mut!(_5);
_1.1 = _1.2;
RET = -_4;
_2.3 = _1.3;
_1.3 = !_2.3;
RET = _4 + _4;
_1.1 = [4235367747_u32,535783044_u32,14423540_u32];
_9 = [11735091127225611886_usize,2987728798732297471_usize,6_usize,5_usize,11717986795011581965_usize,2_usize,4_usize,3_usize];
_3 = _1.3 as isize;
_5 = '\u{52ee4}';
_10 = (-2818187167827979831_i64) as isize;
RET = _7 as isize;
_1.3 = _2.3;
_1.2 = [2193187486_u32,2915944653_u32,3030995230_u32];
RET = _3 * _3;
_2.1 = _1.2;
_1.1 = _2.2;
Goto(bb3)
}
bb3 = {
Call(_12 = dump_var(15_usize, 10_usize, Move(_10), 7_usize, Move(_7), 5_usize, Move(_5), 13_usize, _13), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: [u16; 1],mut _2: [u16; 1],mut _3: [u16; 1],mut _4: [u16; 1],mut _5: [u16; 1],mut _6: [u16; 1],mut _7: [u16; 1]) -> bool {
mir! {
type RET = bool;
let _8: (*const u64, (i8, f64), [u16; 7], bool);
let _9: &'static &'static i8;
let _10: Adt44;
let _11: (usize,);
let _12: u16;
let _13: (*const u64, (i8, f64), [u16; 7], bool);
let _14: u128;
let _15: *mut bool;
let _16: [u32; 3];
let _17: i16;
let _18: &'static u8;
let _19: isize;
let _20: isize;
let _21: *mut *const (Adt27, *const u64, [u8; 5]);
let _22: [u32; 3];
let _23: (i32,);
let _24: i32;
let _25: (*const u64, (i8, f64), [u16; 7], bool);
let _26: i8;
let _27: *const u64;
let _28: [u32; 3];
let _29: char;
let _30: u128;
let _31: Adt44;
let _32: f32;
let _33: (Adt35, *mut &'static i8);
let _34: ();
let _35: ();
{
_3 = [414_u16];
RET = true;
_8.1.1 = (-69_i8) as f64;
_7 = [4039_u16];
RET = !true;
_11 = (2_usize,);
_8.3 = RET > RET;
_11 = (5744512073220868398_usize,);
_8.1.0 = 0_i8;
_8.1.1 = 896317360702099494_u64 as f64;
_2 = _4;
_8.1.0 = 43_i8 - 121_i8;
_6 = [44248_u16];
_8.1.1 = 8643019936961085776_u64 as f64;
_8.1.1 = 17786574069055229004_u64 as f64;
_8.2 = [34111_u16,35315_u16,53808_u16,4360_u16,17937_u16,13062_u16,10244_u16];
_8.3 = RET;
_3 = [33847_u16];
RET = !_8.3;
_7 = _3;
_4 = [14075_u16];
_7 = [40937_u16];
_6 = [3566_u16];
_12 = 50104_u16 | 38805_u16;
Goto(bb1)
}
bb1 = {
_8.2 = [_12,_12,_12,_12,_12,_12,_12];
_3 = [_12];
_13.1 = (_8.1.0, _8.1.1);
_6 = [_12];
_13.2 = [_12,_12,_12,_12,_12,_12,_12];
_7 = [_12];
_8.1 = _13.1;
_15 = core::ptr::addr_of_mut!(_13.3);
RET = !_8.3;
_8.2 = [_12,_12,_12,_12,_12,_12,_12];
_14 = 100460031747271690939234617381735527231_u128 ^ 80656678296182933042335420625343931601_u128;
_8.3 = RET;
RET = _8.3 | _8.3;
_8.1.1 = _13.1.1;
_8.1 = (_13.1.0, _13.1.1);
RET = !_8.3;
_16 = [2036700641_u32,1095209093_u32,2762237779_u32];
_17 = _8.1.0 as i16;
_6 = _5;
(*_15) = RET;
_8.1.0 = _13.1.0 | _13.1.0;
(*_15) = RET;
_19 = (-9223372036854775808_isize) - (-98_isize);
match _11.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
5744512073220868398 => bb10,
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
_6 = [_12];
match _11.0 {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
5744512073220868398 => bb11,
_ => bb7
}
}
bb11 = {
_12 = 61936_u16 | 28297_u16;
_12 = 63769_u16 >> _8.1.0;
(*_15) = RET;
_12 = !10648_u16;
_19 = !127_isize;
_14 = _8.1.0 as u128;
_17 = 162_u8 as i16;
RET = _13.3 | _8.3;
_20 = _19 + _19;
RET = _8.3;
(*_15) = _8.3 | RET;
_8.3 = !(*_15);
_13.2 = [_12,_12,_12,_12,_12,_12,_12];
_13.3 = _8.3;
_13.1.1 = _8.1.1;
_8.1.0 = -_13.1.0;
_23.0 = _13.1.0 as i32;
_13.2 = _8.2;
_23.0 = _11.0 as i32;
Goto(bb12)
}
bb12 = {
(*_15) = !_8.3;
_13.1.1 = _14 as f64;
_24 = _23.0;
_15 = core::ptr::addr_of_mut!(RET);
_1 = [_12];
_8.2 = [_12,_12,_12,_12,_12,_12,_12];
_23.0 = 1193933615_u32 as i32;
_17 = -5565_i16;
_25.1.0 = _13.1.0;
_25.1 = (_13.1.0, _8.1.1);
_13.3 = !RET;
_13.1.0 = _25.1.0 + _25.1.0;
Goto(bb13)
}
bb13 = {
_26 = _25.1.0 << _25.1.0;
_13.3 = _13.1.1 >= _13.1.1;
_25.3 = _13.3;
_12 = 12505_u16 << _17;
_25.2 = _13.2;
_3 = [_12];
RET = !_25.3;
_25.1.0 = 4266310095331630066_u64 as i8;
_16 = [809347963_u32,2696331193_u32,1997991008_u32];
_6 = [_12];
_17 = (-21429_i16) >> _14;
_24 = !_23.0;
_20 = _19;
_23 = (_24,);
_28 = _16;
_13.1.0 = !_8.1.0;
_3 = _2;
_25.1.0 = _8.1.0 * _26;
_20 = !_19;
_13.2 = [_12,_12,_12,_12,_12,_12,_12];
_1 = _2;
_13.1.1 = _25.1.1;
Goto(bb14)
}
bb14 = {
_6 = [_12];
_17 = 6137_i16;
_26 = _25.1.0;
RET = _13.3 ^ _25.3;
(*_15) = !_13.3;
_25.1.0 = _26 | _13.1.0;
_2 = _7;
_13.2 = _25.2;
RET = _26 <= _8.1.0;
_20 = -_19;
_2 = [_12];
_6 = [_12];
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(16_usize, 23_usize, Move(_23), 24_usize, Move(_24), 28_usize, Move(_28), 26_usize, Move(_26)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(16_usize, 7_usize, Move(_7), 1_usize, Move(_1), 3_usize, Move(_3), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(16_usize, 6_usize, Move(_6), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: isize,mut _2: bool,mut _3: Adt59,mut _4: bool,mut _5: isize,mut _6: isize,mut _7: u32,mut _8: i64,mut _9: [u16; 1],mut _10: f32) -> [isize; 4] {
mir! {
type RET = [isize; 4];
let _11: [u8; 5];
let _12: f32;
let _13: char;
let _14: &'static u8;
let _15: ((usize,), &'static i128, *mut bool, (Adt27, *const u64, [u8; 5]));
let _16: char;
let _17: isize;
let _18: *mut &'static i8;
let _19: u128;
let _20: &'static (Adt35, *mut &'static i8);
let _21: u16;
let _22: bool;
let _23: u16;
let _24: char;
let _25: isize;
let _26: usize;
let _27: i8;
let _28: Adt62;
let _29: (Adt44, [u32; 3]);
let _30: f64;
let _31: Adt44;
let _32: Adt42;
let _33: [u8; 5];
let _34: *mut (*mut &'static i8,);
let _35: u64;
let _36: isize;
let _37: [char; 3];
let _38: ();
let _39: ();
{
RET = [_5,_1,_1,_1];
_9 = [21201_u16];
_9 = [25190_u16];
_4 = _8 <= _8;
RET = [_1,_5,_5,_6];
_7 = !2477118603_u32;
_12 = _8 as f32;
_3.fld2 = (-43_i8) as u128;
_7 = _3.fld3 as u32;
_3.fld2 = _12 as u128;
_3.fld1 = core::ptr::addr_of_mut!(_2);
_3.fld2 = 212591186097768056885509465653470631936_u128 | 83125913778545349021887374928981896727_u128;
_13 = '\u{57c2d}';
_3.fld0 = 4_usize;
_7 = _4 as u32;
_11 = [16_u8,160_u8,78_u8,71_u8,77_u8];
_15.3.2 = _11;
_3.fld0 = _4 as usize;
_3.fld2 = (-781851903_i32) as u128;
_10 = _12 + _12;
_1 = _5 >> _3.fld0;
_15.0 = (_3.fld0,);
_2 = _4;
Goto(bb1)
}
bb1 = {
_10 = -_12;
_15.3.2 = [61_u8,163_u8,47_u8,48_u8,19_u8];
_15.0.0 = !_3.fld0;
_3.fld3 = 591436319_i32 as f64;
_3.fld1 = core::ptr::addr_of_mut!(_2);
_8 = !199972792256638494_i64;
RET = [_5,_1,_1,_1];
_13 = '\u{101c7b}';
_15.2 = Move(_3.fld1);
_7 = (-7_i8) as u32;
Call(RET = fn18(_4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = !_2;
_15.0 = (_3.fld0,);
_3.fld1 = core::ptr::addr_of_mut!(_2);
RET = [_1,_1,_6,_1];
_7 = _12 as u32;
_3.fld2 = !248514541093761993317762746834119129411_u128;
_13 = '\u{a2680}';
_3.fld3 = _10 as f64;
_7 = !3799880214_u32;
_17 = !_1;
_7 = 1098570288_u32;
_15.0 = (_3.fld0,);
_11 = [132_u8,48_u8,175_u8,201_u8,17_u8];
_11 = [67_u8,183_u8,248_u8,179_u8,111_u8];
_15.0 = (_3.fld0,);
_10 = 40563_u16 as f32;
_10 = _3.fld0 as f32;
_4 = _1 <= _17;
_6 = 39688397_i32 as isize;
_12 = _10 + _10;
_8 = 4258909738873762044_i64 ^ 5324524774537771763_i64;
_3.fld0 = _15.0.0;
_9 = [6613_u16];
_19 = _3.fld2;
_13 = '\u{12cbe}';
Goto(bb3)
}
bb3 = {
_19 = 35_u8 as u128;
_13 = '\u{56bd3}';
_15.2 = Move(_3.fld1);
_15.0 = (_3.fld0,);
_10 = _12;
_13 = '\u{1001e1}';
RET = [_17,_1,_17,_17];
_3.fld3 = _7 as f64;
_16 = _13;
Goto(bb4)
}
bb4 = {
_4 = _2;
_21 = !30611_u16;
_15.2 = core::ptr::addr_of_mut!(_4);
_23 = _21 & _21;
_3.fld2 = _19 ^ _19;
_21 = !_23;
_22 = _2;
_7 = !4201422804_u32;
_3.fld1 = Move(_15.2);
RET = [_5,_1,_17,_17];
_13 = _16;
_5 = -_17;
_15.2 = core::ptr::addr_of_mut!(_4);
_9 = [_21];
_13 = _16;
_3.fld0 = _15.0.0;
_3.fld1 = Move(_15.2);
_6 = _17;
RET = [_1,_6,_1,_1];
RET = [_17,_6,_5,_6];
_15.0.0 = _3.fld0 | _3.fld0;
_1 = 812711783_i32 as isize;
Goto(bb5)
}
bb5 = {
_22 = !_2;
_3.fld1 = core::ptr::addr_of_mut!(_4);
_29.1 = [_7,_7,_7];
_6 = !_5;
_15.0.0 = _3.fld0;
Goto(bb6)
}
bb6 = {
_15.3.2 = [194_u8,80_u8,172_u8,86_u8,198_u8];
_7 = 2366672952_u32;
_1 = 30649_i16 as isize;
_3.fld1 = core::ptr::addr_of_mut!(_2);
_27 = (-57_i8) & 92_i8;
_22 = !_4;
_15.0 = (_3.fld0,);
_29.1 = [_7,_7,_7];
_27 = 113_i8 << _3.fld0;
_16 = _13;
_4 = _10 > _10;
_3.fld3 = _21 as f64;
_1 = -_5;
_19 = _3.fld2 ^ _3.fld2;
_21 = !_23;
_21 = !_23;
match _7 {
2366672952 => bb8,
_ => bb7
}
}
bb7 = {
_10 = -_12;
_15.3.2 = [61_u8,163_u8,47_u8,48_u8,19_u8];
_15.0.0 = !_3.fld0;
_3.fld3 = 591436319_i32 as f64;
_3.fld1 = core::ptr::addr_of_mut!(_2);
_8 = !199972792256638494_i64;
RET = [_5,_1,_1,_1];
_13 = '\u{101c7b}';
_15.2 = Move(_3.fld1);
_7 = (-7_i8) as u32;
Call(RET = fn18(_4), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_26 = !_3.fld0;
_15.3.2 = _11;
_15.0.0 = _3.fld0;
_23 = _21 | _21;
_9 = [_23];
_12 = _10;
_11 = _15.3.2;
RET = [_1,_1,_1,_5];
_29.1 = [_7,_7,_7];
_23 = _21;
_25 = _1 - _1;
_22 = _25 < _25;
_8 = _27 as i64;
_29.1 = [_7,_7,_7];
_27 = _3.fld3 as i8;
_6 = _1 ^ _25;
_3.fld3 = 110_u8 as f64;
_23 = _21;
Goto(bb9)
}
bb9 = {
_25 = !_1;
_6 = _1 | _1;
_13 = _16;
_15.3.1 = core::ptr::addr_of!(_35);
_3.fld1 = core::ptr::addr_of_mut!(_2);
_15.2 = Move(_3.fld1);
_3.fld0 = _26;
_12 = -_10;
_12 = _10 - _10;
_19 = _3.fld2 >> _1;
_27 = 1_i8;
RET = [_25,_17,_17,_5];
_26 = _10 as usize;
RET = [_25,_17,_17,_25];
_33 = [193_u8,198_u8,124_u8,61_u8,187_u8];
_15.0.0 = (-143456914350715199220880725995505610014_i128) as usize;
match _27 {
0 => bb1,
2 => bb3,
3 => bb8,
4 => bb5,
1 => bb11,
_ => bb10
}
}
bb10 = {
_15.3.2 = [194_u8,80_u8,172_u8,86_u8,198_u8];
_7 = 2366672952_u32;
_1 = 30649_i16 as isize;
_3.fld1 = core::ptr::addr_of_mut!(_2);
_27 = (-57_i8) & 92_i8;
_22 = !_4;
_15.0 = (_3.fld0,);
_29.1 = [_7,_7,_7];
_27 = 113_i8 << _3.fld0;
_16 = _13;
_4 = _10 > _10;
_3.fld3 = _21 as f64;
_1 = -_5;
_19 = _3.fld2 ^ _3.fld2;
_21 = !_23;
_21 = !_23;
match _7 {
2366672952 => bb8,
_ => bb7
}
}
bb11 = {
_36 = _1 >> _26;
_35 = 5634171350788492954_u64 * 10934257626678204302_u64;
RET = [_5,_36,_1,_6];
_3.fld0 = _26;
_37 = [_16,_16,_13];
Goto(bb12)
}
bb12 = {
Call(_38 = dump_var(17_usize, 2_usize, Move(_2), 19_usize, Move(_19), 35_usize, Move(_35), 5_usize, Move(_5)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_38 = dump_var(17_usize, 17_usize, Move(_17), 21_usize, Move(_21), 9_usize, Move(_9), 25_usize, Move(_25)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_38 = dump_var(17_usize, 6_usize, Move(_6), 13_usize, Move(_13), 26_usize, Move(_26), 39_usize, _39), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: bool) -> [isize; 4] {
mir! {
type RET = [isize; 4];
let _2: i32;
let _3: (i16, bool);
let _4: (*mut &'static i8,);
let _5: f64;
let _6: f64;
let _7: [isize; 4];
let _8: [u16; 7];
let _9: f64;
let _10: bool;
let _11: isize;
let _12: (i32,);
let _13: isize;
let _14: isize;
let _15: &'static (Adt35, *mut &'static i8);
let _16: f32;
let _17: f64;
let _18: isize;
let _19: i128;
let _20: [u8; 5];
let _21: ();
let _22: ();
{
RET = [9223372036854775807_isize,105_isize,(-9223372036854775808_isize),59_isize];
_1 = (-15111_i16) < 16702_i16;
RET = [78_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_1 = true;
_2 = 210444255268355799404726211292451935276_u128 as i32;
Goto(bb1)
}
bb1 = {
RET = [(-13_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-101_isize)];
_2 = !1181071919_i32;
_2 = '\u{d582a}' as i32;
_1 = _2 != _2;
RET = [90_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
RET = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_2 = !(-1978227269_i32);
_3 = Checked(17667_i16 - 13478_i16);
_3 = (21136_i16, _1);
_1 = _3.1 < _3.1;
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,100_isize];
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-53_isize),9223372036854775807_isize];
_1 = _3.1;
_1 = _3.0 == _3.0;
RET = [(-9223372036854775808_isize),37_isize,(-9223372036854775808_isize),35_isize];
_2 = -(-1147040590_i32);
_3.1 = _1;
RET = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
match _3.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
21136 => bb10,
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
RET = [7_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_5 = (-18_i8) as f64;
_6 = _5;
_3.0 = (-32280_i16);
_1 = _3.1;
_7 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_3.1 = _5 <= _6;
_1 = !_3.1;
RET = [9223372036854775807_isize,(-56_isize),9223372036854775807_isize,90_isize];
_5 = _6;
RET = _7;
_3.0 = 25931926115456712005028652927457884577_u128 as i16;
_3 = ((-28818_i16), _1);
_3.0 = (-5790_i16) + (-17306_i16);
_7 = [103_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_7 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_8 = [15596_u16,29877_u16,5683_u16,4604_u16,42963_u16,44648_u16,65351_u16];
_5 = _6;
_8 = [11374_u16,56030_u16,35400_u16,61452_u16,48336_u16,44241_u16,39849_u16];
Goto(bb11)
}
bb11 = {
_7 = RET;
RET = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
RET = [110_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-27_isize)];
_6 = _2 as f64;
_9 = -_5;
_3 = ((-838_i16), _1);
_10 = _1;
_3.1 = !_1;
_3.0 = 3942_i16 + 3738_i16;
_5 = _9;
RET = [(-9223372036854775808_isize),(-84_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_12.0 = 163038552732446006053256954235266653795_i128 as i32;
_12.0 = 47711484252761086828677116315881659301_i128 as i32;
_12.0 = 222317444220205710365173786136195814707_u128 as i32;
_3 = ((-19897_i16), _10);
Goto(bb12)
}
bb12 = {
_12.0 = 552385024026019218_i64 as i32;
_11 = (-9165411695480570279_i64) as isize;
_11 = 9223372036854775807_isize;
_2 = _12.0 * _12.0;
_3 = Checked((-24474_i16) * (-16201_i16));
_3.0 = (-24845_i16);
_5 = _9;
_8 = [43295_u16,54528_u16,21105_u16,9822_u16,45650_u16,51304_u16,8774_u16];
_13 = _11 << _2;
_9 = -_6;
_9 = _5 + _5;
_8 = [12176_u16,63341_u16,7190_u16,48761_u16,50823_u16,20589_u16,29844_u16];
_5 = _9 + _9;
_5 = _9;
_3.1 = _1;
_8 = [38007_u16,30038_u16,60187_u16,51317_u16,13369_u16,16051_u16,28011_u16];
Goto(bb13)
}
bb13 = {
RET = [_11,_11,_13,_13];
_1 = _3.1 & _3.1;
_13 = -_11;
_12.0 = 61_u8 as i32;
_9 = -_5;
_11 = _13;
_10 = _1;
_2 = _12.0 - _12.0;
_2 = -_12.0;
_13 = 14_i8 as isize;
_8 = [63385_u16,13488_u16,27169_u16,10840_u16,18768_u16,48027_u16,50530_u16];
_14 = !_13;
match _3.0 {
0 => bb1,
340282366920938463463374607431768186611 => bb14,
_ => bb8
}
}
bb14 = {
_16 = _6 as f32;
_8 = [3002_u16,26618_u16,37368_u16,36069_u16,37835_u16,33611_u16,64590_u16];
_11 = !_14;
_17 = -_5;
_12.0 = _2 - _2;
_11 = -_14;
RET = [_13,_14,_11,_13];
_18 = _13;
_10 = !_3.1;
_11 = _14 + _18;
_7 = [_11,_18,_11,_11];
_5 = 3140699698_u32 as f64;
_17 = -_9;
_5 = _3.0 as f64;
_6 = -_9;
_3.1 = _10;
_19 = 117019996409546346367073643933682540132_i128 << _3.0;
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(18_usize, 13_usize, Move(_13), 3_usize, Move(_3), 18_usize, Move(_18), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_21 = dump_var(18_usize, 10_usize, Move(_10), 14_usize, Move(_14), 22_usize, _22, 22_usize, _22), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(18490_u16), std::hint::black_box('\u{8604a}'), std::hint::black_box(100620797647946826379777899042477153139_i128));
                
            }
impl PrintFDebug for Adt27{
	unsafe fn printf_debug(&self){unsafe{printf("Adt27::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt27 {
Variant0{
fld0: [usize; 8],

},
Variant1{
fld0: i16,
fld1: i64,
fld2: u128,
fld3: *const u64,

}}
impl PrintFDebug for Adt35{
	unsafe fn printf_debug(&self){unsafe{printf("Adt35::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt35 {
Variant0{
fld0: i128,
fld1: u128,
fld2: isize,
fld3: f32,
fld4: i16,
fld5: [usize; 8],
fld6: [usize; 2],

},
Variant1{
fld0: bool,
fld1: i128,
fld2: [u8; 5],
fld3: *mut char,
fld4: i16,

},
Variant2{
fld0: (i16, bool),
fld1: *const usize,
fld2: i32,
fld3: i8,
fld4: i16,

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: bool,
fld1: i32,
fld2: (i16, bool),
fld3: *const usize,
fld4: u32,

},
Variant1{
fld0: (Adt27, *const u64, [u8; 5]),
fld1: char,
fld2: f64,
fld3: u128,
fld4: (*const u64, (i8, f64), [u16; 7], bool),
fld5: u16,
fld6: Adt27,
fld7: [u8; 5],

},
Variant2{
fld0: (Adt27, *const u64, [u8; 5]),
fld1: u128,
fld2: usize,
fld3: i8,
fld4: (i32,),
fld5: *const u64,
fld6: i64,
fld7: u32,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: i16,
fld1: [usize; 8],
fld2: Adt42,

},
Variant1{
fld0: (i32,),
fld1: char,
fld2: isize,
fld3: Adt42,
fld4: i16,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: u64,
fld1: *const Adt35,
fld2: *const usize,
fld3: u16,
fld4: [usize; 2],
fld5: (i16, bool),
fld6: i64,

},
Variant1{
fld0: bool,

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt59{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt59 {
fld0: usize,
fld1: *mut bool,
fld2: u128,
fld3: f64,
}
impl PrintFDebug for Adt62{
	unsafe fn printf_debug(&self){unsafe{printf("Adt62::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
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
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
#[derive(Copy,Clone)]pub enum Adt62 {
Variant0{
fld0: [usize; 2],
fld1: *const usize,
fld2: (Adt27, *const u64, [u8; 5]),

},
Variant1{
fld0: i64,
fld1: [u16; 1],
fld2: i128,
fld3: (i8, f64),
fld4: *mut [u16; 7],
fld5: Adt59,

},
Variant2{
fld0: Adt35,
fld1: (i32,),
fld2: isize,
fld3: u128,
fld4: i16,
fld5: u16,
fld6: [u16; 7],
fld7: u32,

},
Variant3{
fld0: u64,
fld1: i128,
fld2: (usize,),
fld3: Adt27,
fld4: *const (Adt27, *const u64, [u8; 5]),
fld5: (i16, bool),
fld6: u16,

}}
impl PrintFDebug for Adt75{
	unsafe fn printf_debug(&self){unsafe{printf("Adt75::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt75 {
Variant0{
fld0: u128,
fld1: Adt42,
fld2: *mut bool,
fld3: Adt44,
fld4: (usize,),
fld5: Adt62,
fld6: (*mut char,),

},
Variant1{
fld0: bool,
fld1: char,
fld2: u128,
fld3: u64,
fld4: Adt59,
fld5: (Adt44, [u32; 3]),

}}

