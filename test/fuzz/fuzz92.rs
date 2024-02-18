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
pub fn fn0(mut _1: i64) -> i32 {
mir! {
type RET = i32;
let _2: bool;
let _3: *mut *mut (&'static bool, Adt17, &'static &'static bool, &'static &'static i16);
let _4: [usize; 4];
let _5: f64;
let _6: u128;
let _7: [i8; 3];
let _8: f64;
let _9: char;
let _10: (Adt31, i16);
let _11: isize;
let _12: u8;
let _13: u64;
let _14: bool;
let _15: &'static (u32, f64, u8, i8);
let _16: *mut [u16; 1];
let _17: *mut *mut (&'static bool, Adt17, &'static &'static bool, &'static &'static i16);
let _18: *const *mut f32;
let _19: Adt60;
let _20: f64;
let _21: (i8, u64, f32);
let _22: isize;
let _23: &'static *const u32;
let _24: [usize; 4];
let _25: bool;
let _26: char;
let _27: f64;
let _28: *const *mut f32;
let _29: isize;
let _30: char;
let _31: f64;
let _32: char;
let _33: (i8, u64, f32);
let _34: Adt17;
let _35: (i32,);
let _36: u8;
let _37: ();
let _38: ();
{
RET = (-1128203690_i32);
_1 = (-4662910182166431318_i64) & (-51306766295687509_i64);
RET = true as i32;
RET = (-1497608487_i32);
RET = -(-1734512551_i32);
RET = 1618076022_i32 * 781937428_i32;
RET = (-465711787_i32);
RET = -(-1959441103_i32);
_2 = _1 <= _1;
_2 = false & true;
_2 = _1 > _1;
_1 = (-4561_i16) as i64;
_1 = !8617211055252291778_i64;
Call(RET = fn1(_2, _2, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = (-659148197_i32) * 1802117672_i32;
RET = !1558644847_i32;
RET = !(-1729930144_i32);
_1 = (-5698068655151200073_i64) ^ (-6087288507157397455_i64);
_5 = 17138_i16 as f64;
_4 = [14154487255126671779_usize,11165774447552271664_usize,0_usize,6_usize];
RET = -1429751994_i32;
_4 = [14462326008815930543_usize,1676605422844327733_usize,3182551789704331934_usize,1824267796946116233_usize];
_5 = 13528446540281001849_usize as f64;
Call(_6 = fn14(_4, _1, RET, _2, _1, _4, RET, _4, _2, _1, _2, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = [6_usize,4281083458402055262_usize,4_usize,0_usize];
_6 = 283716541325409618202077713363322885356_u128;
_6 = 136408265303011519300252400145559795556_u128;
_8 = 46634_u16 as f64;
_9 = '\u{cc6b9}';
_2 = !true;
RET = (-1732410960_i32);
_10.1 = !31303_i16;
RET = !1577004109_i32;
_4 = [8545045213081814247_usize,2700987699454339576_usize,0_usize,16707523934794961770_usize];
_7 = [(-39_i8),107_i8,(-12_i8)];
_11 = 62_isize;
_8 = _5 + _5;
_5 = _8 - _8;
_13 = _10.1 as u64;
RET = (-359465311_i32);
_8 = -_5;
RET = (-1734525286_i32) | (-1894135345_i32);
_11 = (-9223372036854775808_isize) >> _13;
_11 = 115_isize << _1;
_14 = !_2;
_11 = -9223372036854775807_isize;
_5 = _13 as f64;
Goto(bb3)
}
bb3 = {
_11 = -9223372036854775807_isize;
_10.1 = 28456_i16;
_11 = _9 as isize;
_7 = [77_i8,123_i8,71_i8];
_10.1 = (-425_i16);
_5 = _8 - _8;
_7 = [7_i8,(-85_i8),124_i8];
_11 = (-5731706258233244972802914302084802301_i128) as isize;
_4 = [7074994524955687165_usize,1038315815855416349_usize,2_usize,6_usize];
_14 = _2;
_6 = 118759084941930913545589261468138923147_u128 ^ 232626328249049164099828543236665762270_u128;
_10.1 = _1 as i16;
_11 = 14_isize;
RET = (-86316592_i32);
_12 = 12_u8 ^ 107_u8;
RET = 812579984_i32;
_7 = [(-110_i8),11_i8,(-15_i8)];
_12 = 293_u16 as u8;
_14 = _2 & _2;
RET = 1163139742_i32;
_5 = _8 * _8;
RET = 492129305_i32;
_10.1 = _6 as i16;
_9 = '\u{37cb1}';
Goto(bb4)
}
bb4 = {
_14 = !_2;
_4 = [7764580984816550049_usize,5_usize,6_usize,0_usize];
_13 = 3437325657375071334_u64;
RET = 679165320_i32;
_9 = '\u{14534}';
_7 = [(-6_i8),(-107_i8),(-116_i8)];
RET = (-31204564_i32);
_7 = [23_i8,(-124_i8),(-124_i8)];
RET = _10.1 as i32;
_8 = _5 - _5;
_1 = !(-4155745781420212550_i64);
_9 = '\u{bbc89}';
_13 = !14002298772533231211_u64;
_21.1 = _13;
_21.0 = (-114_i8) >> _10.1;
_10.1 = (-573_i16);
_12 = 4186798711247912197_usize as u8;
_20 = _8 * _8;
_5 = _20 - _20;
_24 = _4;
_5 = _10.1 as f64;
RET = 1541938495_i32 | 1439649851_i32;
_20 = _8;
_9 = '\u{6adfd}';
_11 = (-9223372036854775808_isize);
match _11 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
340282366920938463454151235394913435648 => bb9,
_ => bb8
}
}
bb5 = {
_11 = -9223372036854775807_isize;
_10.1 = 28456_i16;
_11 = _9 as isize;
_7 = [77_i8,123_i8,71_i8];
_10.1 = (-425_i16);
_5 = _8 - _8;
_7 = [7_i8,(-85_i8),124_i8];
_11 = (-5731706258233244972802914302084802301_i128) as isize;
_4 = [7074994524955687165_usize,1038315815855416349_usize,2_usize,6_usize];
_14 = _2;
_6 = 118759084941930913545589261468138923147_u128 ^ 232626328249049164099828543236665762270_u128;
_10.1 = _1 as i16;
_11 = 14_isize;
RET = (-86316592_i32);
_12 = 12_u8 ^ 107_u8;
RET = 812579984_i32;
_7 = [(-110_i8),11_i8,(-15_i8)];
_12 = 293_u16 as u8;
_14 = _2 & _2;
RET = 1163139742_i32;
_5 = _8 * _8;
RET = 492129305_i32;
_10.1 = _6 as i16;
_9 = '\u{37cb1}';
Goto(bb4)
}
bb6 = {
_4 = [6_usize,4281083458402055262_usize,4_usize,0_usize];
_6 = 283716541325409618202077713363322885356_u128;
_6 = 136408265303011519300252400145559795556_u128;
_8 = 46634_u16 as f64;
_9 = '\u{cc6b9}';
_2 = !true;
RET = (-1732410960_i32);
_10.1 = !31303_i16;
RET = !1577004109_i32;
_4 = [8545045213081814247_usize,2700987699454339576_usize,0_usize,16707523934794961770_usize];
_7 = [(-39_i8),107_i8,(-12_i8)];
_11 = 62_isize;
_8 = _5 + _5;
_5 = _8 - _8;
_13 = _10.1 as u64;
RET = (-359465311_i32);
_8 = -_5;
RET = (-1734525286_i32) | (-1894135345_i32);
_11 = (-9223372036854775808_isize) >> _13;
_11 = 115_isize << _1;
_14 = !_2;
_11 = -9223372036854775807_isize;
_5 = _13 as f64;
Goto(bb3)
}
bb7 = {
RET = (-659148197_i32) * 1802117672_i32;
RET = !1558644847_i32;
RET = !(-1729930144_i32);
_1 = (-5698068655151200073_i64) ^ (-6087288507157397455_i64);
_5 = 17138_i16 as f64;
_4 = [14154487255126671779_usize,11165774447552271664_usize,0_usize,6_usize];
RET = -1429751994_i32;
_4 = [14462326008815930543_usize,1676605422844327733_usize,3182551789704331934_usize,1824267796946116233_usize];
_5 = 13528446540281001849_usize as f64;
Call(_6 = fn14(_4, _1, RET, _2, _1, _4, RET, _4, _2, _1, _2, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
RET = !(-72493501_i32);
_12 = 144_u8;
_5 = _8 * _8;
_21.0 = (-84_i8);
_13 = !_21.1;
_6 = 323920294824502943900537350838066328563_u128;
_21.0 = (-81_i8);
_21.1 = _13;
_29 = _11 | _11;
_25 = _5 > _5;
_9 = '\u{c449}';
_2 = _5 == _8;
_7 = [_21.0,_21.0,_21.0];
_24 = [10019312964478439747_usize,13380109420544271073_usize,17953860788442510445_usize,4_usize];
_1 = -7379893828269548589_i64;
_22 = _11 * _29;
Goto(bb10)
}
bb10 = {
_27 = RET as f64;
_29 = -_11;
_33.0 = _25 as i8;
Goto(bb11)
}
bb11 = {
_8 = -_5;
_30 = _9;
_21.2 = _29 as f32;
_9 = _30;
_32 = _9;
_20 = _5;
RET = !1829917913_i32;
_21.2 = _20 as f32;
_10.1 = RET as i16;
match _6 {
0 => bb5,
1 => bb9,
2 => bb12,
3 => bb13,
4 => bb14,
323920294824502943900537350838066328563 => bb16,
_ => bb15
}
}
bb12 = {
_27 = RET as f64;
_29 = -_11;
_33.0 = _25 as i8;
Goto(bb11)
}
bb13 = {
RET = (-659148197_i32) * 1802117672_i32;
RET = !1558644847_i32;
RET = !(-1729930144_i32);
_1 = (-5698068655151200073_i64) ^ (-6087288507157397455_i64);
_5 = 17138_i16 as f64;
_4 = [14154487255126671779_usize,11165774447552271664_usize,0_usize,6_usize];
RET = -1429751994_i32;
_4 = [14462326008815930543_usize,1676605422844327733_usize,3182551789704331934_usize,1824267796946116233_usize];
_5 = 13528446540281001849_usize as f64;
Call(_6 = fn14(_4, _1, RET, _2, _1, _4, RET, _4, _2, _1, _2, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_14 = !_2;
_4 = [7764580984816550049_usize,5_usize,6_usize,0_usize];
_13 = 3437325657375071334_u64;
RET = 679165320_i32;
_9 = '\u{14534}';
_7 = [(-6_i8),(-107_i8),(-116_i8)];
RET = (-31204564_i32);
_7 = [23_i8,(-124_i8),(-124_i8)];
RET = _10.1 as i32;
_8 = _5 - _5;
_1 = !(-4155745781420212550_i64);
_9 = '\u{bbc89}';
_13 = !14002298772533231211_u64;
_21.1 = _13;
_21.0 = (-114_i8) >> _10.1;
_10.1 = (-573_i16);
_12 = 4186798711247912197_usize as u8;
_20 = _8 * _8;
_5 = _20 - _20;
_24 = _4;
_5 = _10.1 as f64;
RET = 1541938495_i32 | 1439649851_i32;
_20 = _8;
_9 = '\u{6adfd}';
_11 = (-9223372036854775808_isize);
match _11 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
340282366920938463454151235394913435648 => bb9,
_ => bb8
}
}
bb15 = {
_4 = [6_usize,4281083458402055262_usize,4_usize,0_usize];
_6 = 283716541325409618202077713363322885356_u128;
_6 = 136408265303011519300252400145559795556_u128;
_8 = 46634_u16 as f64;
_9 = '\u{cc6b9}';
_2 = !true;
RET = (-1732410960_i32);
_10.1 = !31303_i16;
RET = !1577004109_i32;
_4 = [8545045213081814247_usize,2700987699454339576_usize,0_usize,16707523934794961770_usize];
_7 = [(-39_i8),107_i8,(-12_i8)];
_11 = 62_isize;
_8 = _5 + _5;
_5 = _8 - _8;
_13 = _10.1 as u64;
RET = (-359465311_i32);
_8 = -_5;
RET = (-1734525286_i32) | (-1894135345_i32);
_11 = (-9223372036854775808_isize) >> _13;
_11 = 115_isize << _1;
_14 = !_2;
_11 = -9223372036854775807_isize;
_5 = _13 as f64;
Goto(bb3)
}
bb16 = {
_11 = _10.1 as isize;
_5 = _13 as f64;
_32 = _9;
_36 = !_12;
_13 = _21.1;
_33 = _21;
_21.2 = _33.2;
RET = -(-1076541452_i32);
_33.0 = _21.0;
_20 = _8 - _8;
_6 = !166865815711191656127548533188263895703_u128;
RET = _33.2 as i32;
_33.0 = _21.0;
_14 = _25;
_24 = [2_usize,2_usize,7119124697191836493_usize,4_usize];
_31 = _20 + _20;
_2 = _25;
Goto(bb17)
}
bb17 = {
Call(_37 = dump_var(0_usize, 11_usize, Move(_11), 25_usize, Move(_25), 13_usize, Move(_13), 36_usize, Move(_36)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(0_usize, 4_usize, Move(_4), 7_usize, Move(_7), 29_usize, Move(_29), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool) -> i32 {
mir! {
type RET = i32;
let _5: ((i128, &'static *mut f32), Adt17, *const &'static *mut f32, &'static *mut f32);
let _6: i32;
let _7: char;
let _8: Adt36;
let _9: bool;
let _10: usize;
let _11: f32;
let _12: (&'static &'static bool,);
let _13: (u32, f64, u8, i8);
let _14: bool;
let _15: (i128, u64);
let _16: [char; 1];
let _17: [u16; 1];
let _18: bool;
let _19: u8;
let _20: [i8; 3];
let _21: u16;
let _22: ();
let _23: ();
{
RET = -2007332394_i32;
_5.1 = Adt17::Variant0 { fld0: 11438_i16,fld1: 159_u8 };
place!(Field::<u8>(Variant(_5.1, 0), 1)) = (-25662_i16) as u8;
_1 = _3;
place!(Field::<i16>(Variant(_5.1, 0), 0)) = (-1574_i16);
SetDiscriminant(_5.1, 0);
_2 = _4 < _3;
_5.1 = Adt17::Variant0 { fld0: 525_i16,fld1: 226_u8 };
_5.1 = Adt17::Variant0 { fld0: 7432_i16,fld1: 235_u8 };
_1 = !_2;
RET = !(-396292248_i32);
_2 = _1;
_5.0.0 = !(-64360400866777492371953927006943077171_i128);
_1 = _2;
Goto(bb1)
}
bb1 = {
place!(Field::<i16>(Variant(_5.1, 0), 0)) = 24679_i16 - 16110_i16;
_5.1 = Adt17::Variant0 { fld0: 31166_i16,fld1: 205_u8 };
_1 = !_3;
RET = !(-1313738111_i32);
RET = (-512240735_i32);
_3 = !_4;
RET = -(-715452649_i32);
_5.0.0 = (-161693603800047413457664737620990567349_i128);
_5.2 = core::ptr::addr_of!(_5.0.1);
_5.1 = Adt17::Variant0 { fld0: 7597_i16,fld1: 72_u8 };
place!(Field::<i16>(Variant(_5.1, 0), 0)) = -(-5818_i16);
_5.2 = core::ptr::addr_of!(_5.0.1);
_3 = _1;
RET = 1311822218_i32;
match _5.0.0 {
0 => bb2,
1 => bb3,
178588763120891050005709869810777644107 => bb5,
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
_3 = _2;
place!(Field::<u8>(Variant(_5.1, 0), 1)) = 163_u8;
RET = 8460358347663053296_i64 as i32;
_10 = '\u{af616}' as usize;
_1 = _2 == _2;
_5.1 = Adt17::Variant1 { fld0: (-6869746359212855763_i64),fld1: 19577_u16,fld2: 9223372036854775807_isize,fld3: (-47_i8),fld4: _10 };
place!(Field::<i64>(Variant(_5.1, 1), 0)) = 2486717469577642122_i64 & 2504634302108739098_i64;
place!(Field::<usize>(Variant(_5.1, 1), 4)) = Field::<i64>(Variant(_5.1, 1), 0) as usize;
place!(Field::<isize>(Variant(_5.1, 1), 2)) = 9223372036854775807_isize;
_9 = _1 & _4;
_5.2 = core::ptr::addr_of!(_5.0.1);
_7 = '\u{d120c}';
place!(Field::<isize>(Variant(_5.1, 1), 2)) = 37298_u16 as isize;
_5.0.0 = 136241735458929333944293302128307041022_i128;
_8 = Adt36::Variant2 { fld0: 10666731805308238209_u64,fld1: _5.0.0 };
place!(Field::<usize>(Variant(_5.1, 1), 4)) = Field::<i64>(Variant(_5.1, 1), 0) as usize;
_14 = !_2;
_9 = _1;
place!(Field::<i8>(Variant(_5.1, 1), 3)) = -15_i8;
_2 = _9;
_9 = _1;
Call(place!(Field::<i64>(Variant(_5.1, 1), 0)) = fn2(Field::<i128>(Variant(_8, 2), 1), _9), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_15.0 = !Field::<i128>(Variant(_8, 2), 1);
_4 = _9 >= _3;
place!(Field::<usize>(Variant(_5.1, 1), 4)) = Field::<i128>(Variant(_8, 2), 1) as usize;
_5.2 = core::ptr::addr_of!(_5.0.1);
_13.3 = 234826351812290204991608715287816134499_u128 as i8;
_15.0 = 28754748953627268257591574999869313638_u128 as i128;
_7 = '\u{c8013}';
_6 = (-16963_i16) as i32;
_5.1 = Adt17::Variant1 { fld0: (-3897479112547763748_i64),fld1: 31493_u16,fld2: 9223372036854775807_isize,fld3: _13.3,fld4: _10 };
place!(Field::<isize>(Variant(_5.1, 1), 2)) = 95_isize;
place!(Field::<usize>(Variant(_5.1, 1), 4)) = 269256471795720376916871478698990412357_u128 as usize;
_15.1 = _7 as u64;
place!(Field::<i8>(Variant(_5.1, 1), 3)) = _13.3 >> _13.3;
_8 = Adt36::Variant2 { fld0: _15.1,fld1: _15.0 };
_17 = [50578_u16];
_6 = _10 as i32;
place!(Field::<u64>(Variant(_8, 2), 0)) = _15.1 + _15.1;
_7 = '\u{6da31}';
_14 = _2;
_9 = _4;
_5.2 = core::ptr::addr_of!(_5.0.1);
_5.2 = core::ptr::addr_of!(_5.3);
_15.0 = Field::<i128>(Variant(_8, 2), 1) | _5.0.0;
place!(Field::<u16>(Variant(_5.1, 1), 1)) = Field::<i128>(Variant(_8, 2), 1) as u16;
Goto(bb7)
}
bb7 = {
place!(Field::<i64>(Variant(_5.1, 1), 0)) = !8938663340097817248_i64;
_15.0 = Field::<i64>(Variant(_5.1, 1), 0) as i128;
_7 = '\u{e4658}';
_15.0 = Field::<i128>(Variant(_8, 2), 1) + Field::<i128>(Variant(_8, 2), 1);
_3 = _2;
_13.2 = !202_u8;
_5.0.0 = !Field::<i128>(Variant(_8, 2), 1);
_17 = [Field::<u16>(Variant(_5.1, 1), 1)];
Goto(bb8)
}
bb8 = {
_13.1 = Field::<u16>(Variant(_5.1, 1), 1) as f64;
_14 = _2 == _3;
_10 = Field::<usize>(Variant(_5.1, 1), 4);
match Field::<isize>(Variant(_5.1, 1), 2) {
95 => bb9,
_ => bb7
}
}
bb9 = {
_9 = _1;
place!(Field::<usize>(Variant(_5.1, 1), 4)) = Field::<u16>(Variant(_5.1, 1), 1) as usize;
_11 = _13.1 as f32;
match Field::<isize>(Variant(_5.1, 1), 2) {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
95 => bb15,
_ => bb14
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_15.0 = !Field::<i128>(Variant(_8, 2), 1);
_4 = _9 >= _3;
place!(Field::<usize>(Variant(_5.1, 1), 4)) = Field::<i128>(Variant(_8, 2), 1) as usize;
_5.2 = core::ptr::addr_of!(_5.0.1);
_13.3 = 234826351812290204991608715287816134499_u128 as i8;
_15.0 = 28754748953627268257591574999869313638_u128 as i128;
_7 = '\u{c8013}';
_6 = (-16963_i16) as i32;
_5.1 = Adt17::Variant1 { fld0: (-3897479112547763748_i64),fld1: 31493_u16,fld2: 9223372036854775807_isize,fld3: _13.3,fld4: _10 };
place!(Field::<isize>(Variant(_5.1, 1), 2)) = 95_isize;
place!(Field::<usize>(Variant(_5.1, 1), 4)) = 269256471795720376916871478698990412357_u128 as usize;
_15.1 = _7 as u64;
place!(Field::<i8>(Variant(_5.1, 1), 3)) = _13.3 >> _13.3;
_8 = Adt36::Variant2 { fld0: _15.1,fld1: _15.0 };
_17 = [50578_u16];
_6 = _10 as i32;
place!(Field::<u64>(Variant(_8, 2), 0)) = _15.1 + _15.1;
_7 = '\u{6da31}';
_14 = _2;
_9 = _4;
_5.2 = core::ptr::addr_of!(_5.0.1);
_5.2 = core::ptr::addr_of!(_5.3);
_15.0 = Field::<i128>(Variant(_8, 2), 1) | _5.0.0;
place!(Field::<u16>(Variant(_5.1, 1), 1)) = Field::<i128>(Variant(_8, 2), 1) as u16;
Goto(bb7)
}
bb13 = {
place!(Field::<i16>(Variant(_5.1, 0), 0)) = 24679_i16 - 16110_i16;
_5.1 = Adt17::Variant0 { fld0: 31166_i16,fld1: 205_u8 };
_1 = !_3;
RET = !(-1313738111_i32);
RET = (-512240735_i32);
_3 = !_4;
RET = -(-715452649_i32);
_5.0.0 = (-161693603800047413457664737620990567349_i128);
_5.2 = core::ptr::addr_of!(_5.0.1);
_5.1 = Adt17::Variant0 { fld0: 7597_i16,fld1: 72_u8 };
place!(Field::<i16>(Variant(_5.1, 0), 0)) = -(-5818_i16);
_5.2 = core::ptr::addr_of!(_5.0.1);
_3 = _1;
RET = 1311822218_i32;
match _5.0.0 {
0 => bb2,
1 => bb3,
178588763120891050005709869810777644107 => bb5,
_ => bb4
}
}
bb14 = {
Return()
}
bb15 = {
_5.2 = core::ptr::addr_of!(_5.3);
place!(Field::<usize>(Variant(_5.1, 1), 4)) = _10;
_16 = [_7];
_15 = (_5.0.0, Field::<u64>(Variant(_8, 2), 0));
_2 = !_3;
place!(Field::<u64>(Variant(_8, 2), 0)) = _15.1;
_10 = Field::<usize>(Variant(_5.1, 1), 4) | Field::<usize>(Variant(_5.1, 1), 4);
_20 = [Field::<i8>(Variant(_5.1, 1), 3),_13.3,Field::<i8>(Variant(_5.1, 1), 3)];
Goto(bb16)
}
bb16 = {
Call(_22 = dump_var(1_usize, 9_usize, Move(_9), 4_usize, Move(_4), 7_usize, Move(_7), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_22 = dump_var(1_usize, 16_usize, Move(_16), 3_usize, Move(_3), 23_usize, _23, 23_usize, _23), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: i128,mut _2: bool) -> i64 {
mir! {
type RET = i64;
let _3: u16;
let _4: f32;
let _5: (&'static bool, Adt17, &'static &'static bool, &'static &'static i16);
let _6: u128;
let _7: f32;
let _8: Adt63;
let _9: (i32,);
let _10: f32;
let _11: &'static &'static *const u32;
let _12: (i128, u64);
let _13: [i16; 1];
let _14: i64;
let _15: [u16; 1];
let _16: i32;
let _17: isize;
let _18: char;
let _19: &'static bool;
let _20: f32;
let _21: char;
let _22: [i16; 1];
let _23: f64;
let _24: u32;
let _25: i128;
let _26: isize;
let _27: *const &'static u8;
let _28: char;
let _29: u128;
let _30: f32;
let _31: Adt40;
let _32: f32;
let _33: ();
let _34: ();
{
RET = (-5041008877347473129_i64) | (-5362378880409298577_i64);
_2 = _1 < _1;
RET = 383244768441299067_usize as i64;
Goto(bb1)
}
bb1 = {
_1 = 149969513359208073872702410342507787634_i128 << RET;
_1 = 60416214854358867127938974478804200902_i128 << RET;
_5.1 = Adt17::Variant0 { fld0: 2726_i16,fld1: 132_u8 };
_6 = !151187310060819477683904713267925823544_u128;
_5.0 = &_2;
place!(Field::<i16>(Variant(_5.1, 0), 0)) = !3735_i16;
place!(Field::<u8>(Variant(_5.1, 0), 1)) = Field::<i16>(Variant(_5.1, 0), 0) as u8;
_9.0 = (-2069525921_i32);
place!(Field::<i16>(Variant(_5.1, 0), 0)) = 1407122221_u32 as i16;
_3 = 23998_u16 & 54475_u16;
Goto(bb2)
}
bb2 = {
_12.1 = 7764603844192061730_u64;
_12.0 = _1;
SetDiscriminant(_5.1, 1);
_5.0 = &_2;
_9.0 = 3_usize as i32;
_10 = 55_isize as f32;
_1 = 59_u8 as i128;
place!(Field::<u16>(Variant(_5.1, 1), 1)) = _3;
_5.2 = &_5.0;
_10 = _6 as f32;
_10 = 63_i8 as f32;
_4 = _10;
place!(Field::<usize>(Variant(_5.1, 1), 4)) = 16944429027814064513_usize << _12.0;
RET = 91_u8 as i64;
RET = (-4214460109474786156_i64) - 1622194504885926470_i64;
_7 = -_4;
_10 = -_4;
Goto(bb3)
}
bb3 = {
_15 = [_3];
place!(Field::<i8>(Variant(_5.1, 1), 3)) = (-78_i8);
place!(Field::<i64>(Variant(_5.1, 1), 0)) = RET + RET;
_7 = -_4;
_7 = _10 + _10;
_10 = _7 + _7;
place!(Field::<i64>(Variant(_5.1, 1), 0)) = _3 as i64;
_18 = '\u{1009b7}';
_15 = [Field::<u16>(Variant(_5.1, 1), 1)];
_5.1 = Adt17::Variant0 { fld0: (-27717_i16),fld1: 4_u8 };
Goto(bb4)
}
bb4 = {
_5.0 = &_2;
_10 = _4 - _7;
_14 = 139_u8 as i64;
_3 = 37116_u16 & 15605_u16;
place!(Field::<i16>(Variant(_5.1, 0), 0)) = (-17467_i16) - (-11270_i16);
RET = -_14;
RET = _14 * _14;
_5.2 = &_19;
_5.2 = &_19;
RET = !_14;
_2 = _18 != _18;
_3 = 67_u8 as u16;
_9 = ((-975058436_i32),);
_5.2 = &_19;
_5.0 = &_2;
_19 = &_2;
_5.2 = &_19;
place!(Field::<u8>(Variant(_5.1, 0), 1)) = _3 as u8;
_17 = !9223372036854775807_isize;
RET = _14 << _12.1;
_5.1 = Adt17::Variant0 { fld0: 27117_i16,fld1: 142_u8 };
_12.1 = !12832026623064646602_u64;
_5.2 = &_19;
_14 = RET;
Call(_4 = fn3(Move(_19), Move(_5.0), _12.1, _12.0, _12.0, _15, _14, RET, _18), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET = _14 >> _14;
_13 = [25031_i16];
_10 = _12.0 as f32;
_5.1 = Adt17::Variant0 { fld0: 20273_i16,fld1: 77_u8 };
RET = _14 + _14;
_24 = !2340933862_u32;
_23 = 7_usize as f64;
place!(Field::<u8>(Variant(_5.1, 0), 1)) = !49_u8;
RET = _3 as i64;
_5.0 = &_2;
Goto(bb6)
}
bb6 = {
_24 = _12.0 as u32;
_9.0 = 812628646_i32;
_10 = -_4;
_22 = [27395_i16];
_6 = RET as u128;
_5.2 = &_5.0;
_31.fld1 = [_17,_17];
_25 = _12.0 ^ _1;
_20 = _10 * _4;
_31.fld2 = [_24,_24,_24,_24,_24,_24];
_31.fld0 = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_5.1, 0), 0)));
_14 = RET;
_26 = -_17;
match _9.0 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
812628646 => bb14,
_ => bb13
}
}
bb7 = {
RET = _14 >> _14;
_13 = [25031_i16];
_10 = _12.0 as f32;
_5.1 = Adt17::Variant0 { fld0: 20273_i16,fld1: 77_u8 };
RET = _14 + _14;
_24 = !2340933862_u32;
_23 = 7_usize as f64;
place!(Field::<u8>(Variant(_5.1, 0), 1)) = !49_u8;
RET = _3 as i64;
_5.0 = &_2;
Goto(bb6)
}
bb8 = {
_5.0 = &_2;
_10 = _4 - _7;
_14 = 139_u8 as i64;
_3 = 37116_u16 & 15605_u16;
place!(Field::<i16>(Variant(_5.1, 0), 0)) = (-17467_i16) - (-11270_i16);
RET = -_14;
RET = _14 * _14;
_5.2 = &_19;
_5.2 = &_19;
RET = !_14;
_2 = _18 != _18;
_3 = 67_u8 as u16;
_9 = ((-975058436_i32),);
_5.2 = &_19;
_5.0 = &_2;
_19 = &_2;
_5.2 = &_19;
place!(Field::<u8>(Variant(_5.1, 0), 1)) = _3 as u8;
_17 = !9223372036854775807_isize;
RET = _14 << _12.1;
_5.1 = Adt17::Variant0 { fld0: 27117_i16,fld1: 142_u8 };
_12.1 = !12832026623064646602_u64;
_5.2 = &_19;
_14 = RET;
Call(_4 = fn3(Move(_19), Move(_5.0), _12.1, _12.0, _12.0, _15, _14, RET, _18), ReturnTo(bb5), UnwindUnreachable())
}
bb9 = {
_15 = [_3];
place!(Field::<i8>(Variant(_5.1, 1), 3)) = (-78_i8);
place!(Field::<i64>(Variant(_5.1, 1), 0)) = RET + RET;
_7 = -_4;
_7 = _10 + _10;
_10 = _7 + _7;
place!(Field::<i64>(Variant(_5.1, 1), 0)) = _3 as i64;
_18 = '\u{1009b7}';
_15 = [Field::<u16>(Variant(_5.1, 1), 1)];
_5.1 = Adt17::Variant0 { fld0: (-27717_i16),fld1: 4_u8 };
Goto(bb4)
}
bb10 = {
_12.1 = 7764603844192061730_u64;
_12.0 = _1;
SetDiscriminant(_5.1, 1);
_5.0 = &_2;
_9.0 = 3_usize as i32;
_10 = 55_isize as f32;
_1 = 59_u8 as i128;
place!(Field::<u16>(Variant(_5.1, 1), 1)) = _3;
_5.2 = &_5.0;
_10 = _6 as f32;
_10 = 63_i8 as f32;
_4 = _10;
place!(Field::<usize>(Variant(_5.1, 1), 4)) = 16944429027814064513_usize << _12.0;
RET = 91_u8 as i64;
RET = (-4214460109474786156_i64) - 1622194504885926470_i64;
_7 = -_4;
_10 = -_4;
Goto(bb3)
}
bb11 = {
_1 = 149969513359208073872702410342507787634_i128 << RET;
_1 = 60416214854358867127938974478804200902_i128 << RET;
_5.1 = Adt17::Variant0 { fld0: 2726_i16,fld1: 132_u8 };
_6 = !151187310060819477683904713267925823544_u128;
_5.0 = &_2;
place!(Field::<i16>(Variant(_5.1, 0), 0)) = !3735_i16;
place!(Field::<u8>(Variant(_5.1, 0), 1)) = Field::<i16>(Variant(_5.1, 0), 0) as u8;
_9.0 = (-2069525921_i32);
place!(Field::<i16>(Variant(_5.1, 0), 0)) = 1407122221_u32 as i16;
_3 = 23998_u16 & 54475_u16;
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_17 = -_26;
_14 = -RET;
_5.0 = &_2;
_15 = [_3];
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(2_usize, 2_usize, Move(_2), 9_usize, Move(_9), 22_usize, Move(_22), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(2_usize, 13_usize, Move(_13), 6_usize, Move(_6), 15_usize, Move(_15), 34_usize, _34), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: &'static bool,mut _2: &'static bool,mut _3: u64,mut _4: i128,mut _5: i128,mut _6: [u16; 1],mut _7: i64,mut _8: i64,mut _9: char) -> f32 {
mir! {
type RET = f32;
let _10: bool;
let _11: i128;
let _12: i128;
let _13: f64;
let _14: Adt60;
let _15: f32;
let _16: i128;
let _17: f32;
let _18: isize;
let _19: u64;
let _20: u8;
let _21: [usize; 4];
let _22: f32;
let _23: u128;
let _24: *mut (&'static bool, Adt17, &'static &'static bool, &'static &'static i16);
let _25: &'static i16;
let _26: *mut Adt39;
let _27: char;
let _28: *const i32;
let _29: u64;
let _30: f32;
let _31: ();
let _32: ();
{
_7 = !_8;
RET = 251438882516056421867096576477065866260_u128 as f32;
_6 = [20683_u16];
_4 = !_5;
_9 = '\u{742c}';
_3 = !17944477996924106288_u64;
_5 = -_4;
_8 = _7 | _7;
_3 = 6160846599372801787_u64 + 10573023205953417656_u64;
_4 = _5;
_9 = '\u{cb578}';
_4 = _5 >> _8;
_2 = &_10;
_6 = [62583_u16];
_6 = [3266_u16];
_9 = '\u{10638b}';
_8 = (-787301236_i32) as i64;
_10 = !true;
_4 = RET as i128;
_3 = !4823454023023650644_u64;
_5 = _4 + _4;
_7 = _8 >> _5;
_4 = -_5;
_13 = 114_i8 as f64;
_4 = 1065641124_i32 as i128;
Call(_6 = fn4(RET, _3, _7, _8, _5, _8, _7, _8, _9, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = 2431491270_u32 as i128;
_15 = -RET;
_8 = _3 as i64;
_8 = _7 + _7;
_4 = _10 as i128;
_1 = &_10;
_16 = _4 >> _7;
RET = -_15;
_12 = _16;
_11 = _16 & _16;
_1 = &(*_1);
RET = _15 - _15;
_15 = RET * RET;
_17 = RET;
_19 = (-9223372036854775808_isize) as u64;
_16 = !_12;
_16 = _12 | _11;
_6 = [47555_u16];
_18 = (-9223372036854775808_isize);
_15 = -RET;
_7 = -_8;
_10 = _12 != _16;
_19 = !_3;
_7 = -_8;
_7 = _13 as i64;
match _18 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463454151235394913435648 => bb7,
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
_2 = &_10;
_11 = 158638794_u32 as i128;
RET = -_17;
_1 = &_10;
_7 = _8;
RET = 14851993646239241367_usize as f32;
_6 = [37817_u16];
_12 = _16;
_6 = [9406_u16];
_7 = _8;
RET = _17 * _15;
_22 = _15 - _15;
_5 = _12 << _16;
_4 = _12;
_5 = _12 - _16;
_6 = [21559_u16];
_10 = !false;
_12 = !_4;
match _18 {
0 => bb6,
1 => bb8,
2 => bb9,
340282366920938463454151235394913435648 => bb11,
_ => bb10
}
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_11 = 2431491270_u32 as i128;
_15 = -RET;
_8 = _3 as i64;
_8 = _7 + _7;
_4 = _10 as i128;
_1 = &_10;
_16 = _4 >> _7;
RET = -_15;
_12 = _16;
_11 = _16 & _16;
_1 = &(*_1);
RET = _15 - _15;
_15 = RET * RET;
_17 = RET;
_19 = (-9223372036854775808_isize) as u64;
_16 = !_12;
_16 = _12 | _11;
_6 = [47555_u16];
_18 = (-9223372036854775808_isize);
_15 = -RET;
_7 = -_8;
_10 = _12 != _16;
_19 = !_3;
_7 = -_8;
_7 = _13 as i64;
match _18 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463454151235394913435648 => bb7,
_ => bb6
}
}
bb11 = {
_23 = 131010088853509856451352923059362684326_u128 + 336993675190863924330468734410932862977_u128;
_11 = !_5;
_23 = 113_i8 as u128;
_2 = &_10;
_20 = 3263002272_u32 as u8;
_9 = '\u{24d9f}';
RET = -_22;
_22 = _15 * _15;
_4 = _5;
_27 = _9;
_21 = [2_usize,10847998776318248310_usize,16073428130477542603_usize,7_usize];
_7 = -_8;
_1 = Move(_2);
_13 = 59_i8 as f64;
_1 = &_10;
Goto(bb12)
}
bb12 = {
_2 = &(*_1);
_2 = &(*_1);
_16 = _5 | _11;
_18 = !(-9223372036854775808_isize);
_10 = false;
RET = _17;
_19 = _3 * _3;
_22 = _15 + _17;
_9 = _27;
_1 = &_10;
Goto(bb13)
}
bb13 = {
_21 = [5_usize,0_usize,9591507649671343340_usize,3793575543296672135_usize];
_6 = [31846_u16];
_29 = !_19;
_21 = [6129986574305021171_usize,3_usize,16933552360655888193_usize,3647041883581396756_usize];
_18 = 937191525_u32 as isize;
_6 = [22976_u16];
_11 = _5;
_23 = !194486484818880101178321353934497878993_u128;
_3 = _29;
_6 = [13706_u16];
_30 = _22 * _22;
Goto(bb14)
}
bb14 = {
_4 = _15 as i128;
_18 = (-9223372036854775808_isize);
_21 = [1_usize,6_usize,2_usize,7_usize];
_15 = 1_usize as f32;
_11 = _5;
_8 = _5 as i64;
_3 = _29;
_6 = [7852_u16];
_18 = _5 as isize;
_16 = _11;
RET = -_30;
RET = _15;
_18 = 9223372036854775807_isize * 34_isize;
RET = _30 - _30;
_15 = RET;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(3_usize, 19_usize, Move(_19), 29_usize, Move(_29), 20_usize, Move(_20), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(3_usize, 21_usize, Move(_21), 3_usize, Move(_3), 18_usize, Move(_18), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(3_usize, 8_usize, Move(_8), 32_usize, _32, 32_usize, _32, 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: f32,mut _2: u64,mut _3: i64,mut _4: i64,mut _5: i128,mut _6: i64,mut _7: i64,mut _8: i64,mut _9: char,mut _10: i64) -> [u16; 1] {
mir! {
type RET = [u16; 1];
let _11: bool;
let _12: [i8; 2];
let _13: i32;
let _14: isize;
let _15: [u16; 1];
let _16: [isize; 2];
let _17: i32;
let _18: [usize; 7];
let _19: usize;
let _20: Adt17;
let _21: (u32, f64, u8, i8);
let _22: *mut *const i64;
let _23: isize;
let _24: u16;
let _25: f64;
let _26: u64;
let _27: i8;
let _28: char;
let _29: (&'static bool, Adt17, &'static &'static bool, &'static &'static i16);
let _30: i64;
let _31: [isize; 7];
let _32: [i16; 1];
let _33: (*const u32, u32);
let _34: f64;
let _35: [u8; 5];
let _36: bool;
let _37: i32;
let _38: [isize; 2];
let _39: *const *const [isize; 7];
let _40: bool;
let _41: [char; 1];
let _42: &'static (u32, f64, u8, i8);
let _43: *mut *mut (&'static bool, Adt17, &'static &'static bool, &'static &'static i16);
let _44: isize;
let _45: f64;
let _46: ();
let _47: ();
{
RET = [12705_u16];
_3 = _10 << _10;
_5 = _2 as i128;
Goto(bb1)
}
bb1 = {
_11 = _10 < _7;
_1 = 3270503820_u32 as f32;
_5 = 140831952144531192876002814743456782832_i128 & 119180077715965291936067343704190208275_i128;
RET = [49691_u16];
_6 = !_10;
_8 = _3 ^ _10;
_5 = 111918959036094739336513750750017950085_i128 | 152678677187477371928659366989647748088_i128;
_5 = (-50944058489347810810689254887935063050_i128);
RET = [1834_u16];
_10 = !_8;
_11 = !true;
_4 = 6_usize as i64;
_5 = _6 as i128;
_4 = _10 | _3;
_3 = !_10;
_13 = (-1539680033_i32) << _5;
Call(_4 = fn5(_3, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_12 = [(-11_i8),(-31_i8)];
_14 = _1 as isize;
_11 = true;
_12 = [69_i8,61_i8];
_13 = (-1750001151_i32) * (-1333545846_i32);
_6 = _5 as i64;
_2 = !2322423902960290757_u64;
_16 = [_14,_14];
RET = [36714_u16];
_11 = _3 <= _3;
_5 = _2 as i128;
_7 = _3 + _3;
_9 = '\u{1a2f4}';
Goto(bb3)
}
bb3 = {
_6 = _8 ^ _7;
RET = [14810_u16];
_16 = [_14,_14];
_7 = !_8;
_14 = -(-9223372036854775808_isize);
Goto(bb4)
}
bb4 = {
_13 = -(-1564562205_i32);
_6 = 9191_u16 as i64;
_18 = [4_usize,16044295010371564710_usize,13458084392215484798_usize,1_usize,7885109800249724191_usize,251952955573410051_usize,1_usize];
_7 = -_10;
_9 = '\u{dc639}';
_16 = [_14,_14];
_15 = [5174_u16];
Goto(bb5)
}
bb5 = {
_17 = _13 & _13;
_10 = !_8;
RET = [35699_u16];
_15 = [42100_u16];
Call(_7 = core::intrinsics::transmute(_10), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
RET = [18711_u16];
RET = _15;
_13 = _17;
_21.1 = 213338051293013325688557460235404760122_u128 as f64;
_6 = _7;
_4 = _10;
_21.0 = 200227244_u32 * 158849075_u32;
_4 = _5 as i64;
_17 = _13 | _13;
_15 = RET;
_21.3 = _3 as i8;
RET = [20190_u16];
_18 = [6_usize,6_usize,14122817368010365350_usize,3_usize,2_usize,15535618930735414765_usize,5_usize];
_15 = [49631_u16];
Goto(bb7)
}
bb7 = {
_17 = !_13;
_19 = !0_usize;
_8 = _6;
_25 = _21.3 as f64;
_21 = (4029286030_u32, _25, 233_u8, (-49_i8));
_27 = !_21.3;
_8 = _10 >> _27;
_10 = _8;
_12 = [_27,_21.3];
_27 = _21.3 | _21.3;
_27 = !_21.3;
_25 = _21.1 - _21.1;
_7 = _3 << _21.0;
_24 = _13 as u16;
_20 = Adt17::Variant0 { fld0: (-20042_i16),fld1: _21.2 };
_15 = [_24];
_33.0 = core::ptr::addr_of!(_21.0);
place!(Field::<u8>(Variant(_20, 0), 1)) = 205713039041869848049908614493413674892_u128 as u8;
_26 = _2 - _2;
_20 = Adt17::Variant1 { fld0: _10,fld1: _24,fld2: _14,fld3: _27,fld4: _19 };
match _21.0 {
0 => bb1,
4029286030 => bb9,
_ => bb8
}
}
bb8 = {
_11 = _10 < _7;
_1 = 3270503820_u32 as f32;
_5 = 140831952144531192876002814743456782832_i128 & 119180077715965291936067343704190208275_i128;
RET = [49691_u16];
_6 = !_10;
_8 = _3 ^ _10;
_5 = 111918959036094739336513750750017950085_i128 | 152678677187477371928659366989647748088_i128;
_5 = (-50944058489347810810689254887935063050_i128);
RET = [1834_u16];
_10 = !_8;
_11 = !true;
_4 = 6_usize as i64;
_5 = _6 as i128;
_4 = _10 | _3;
_3 = !_10;
_13 = (-1539680033_i32) << _5;
Call(_4 = fn5(_3, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb9 = {
_29.2 = &_29.0;
_18 = [Field::<usize>(Variant(_20, 1), 4),Field::<usize>(Variant(_20, 1), 4),_19,Field::<usize>(Variant(_20, 1), 4),_19,Field::<usize>(Variant(_20, 1), 4),_19];
_35 = [_21.2,_21.2,_21.2,_21.2,_21.2];
_21.3 = Field::<i8>(Variant(_20, 1), 3);
_23 = _14 * _14;
_5 = -159967915588212497577602637656409592595_i128;
place!(Field::<isize>(Variant(_20, 1), 2)) = -_23;
_33.1 = _21.0;
_17 = _21.3 as i32;
_8 = _2 as i64;
_6 = _23 as i64;
Call(place!(Field::<i64>(Variant(_20, 1), 0)) = core::intrinsics::transmute(_7), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_5 = (-162153856051857847289951719693994469808_i128);
_7 = _10;
_21.0 = _1 as u32;
_16 = [_14,_23];
_37 = !_17;
_3 = Field::<i64>(Variant(_20, 1), 0);
_1 = _3 as f32;
_21.1 = -_25;
_20 = Adt17::Variant0 { fld0: (-4673_i16),fld1: _21.2 };
_13 = _17 | _17;
_36 = _11;
_16 = [_23,_23];
match _33.1 {
0 => bb7,
1 => bb6,
2 => bb8,
3 => bb11,
4 => bb12,
4029286030 => bb14,
_ => bb13
}
}
bb11 = {
_29.2 = &_29.0;
_18 = [Field::<usize>(Variant(_20, 1), 4),Field::<usize>(Variant(_20, 1), 4),_19,Field::<usize>(Variant(_20, 1), 4),_19,Field::<usize>(Variant(_20, 1), 4),_19];
_35 = [_21.2,_21.2,_21.2,_21.2,_21.2];
_21.3 = Field::<i8>(Variant(_20, 1), 3);
_23 = _14 * _14;
_5 = -159967915588212497577602637656409592595_i128;
place!(Field::<isize>(Variant(_20, 1), 2)) = -_23;
_33.1 = _21.0;
_17 = _21.3 as i32;
_8 = _2 as i64;
_6 = _23 as i64;
Call(place!(Field::<i64>(Variant(_20, 1), 0)) = core::intrinsics::transmute(_7), ReturnTo(bb10), UnwindUnreachable())
}
bb12 = {
_12 = [(-11_i8),(-31_i8)];
_14 = _1 as isize;
_11 = true;
_12 = [69_i8,61_i8];
_13 = (-1750001151_i32) * (-1333545846_i32);
_6 = _5 as i64;
_2 = !2322423902960290757_u64;
_16 = [_14,_14];
RET = [36714_u16];
_11 = _3 <= _3;
_5 = _2 as i128;
_7 = _3 + _3;
_9 = '\u{1a2f4}';
Goto(bb3)
}
bb13 = {
_17 = !_13;
_19 = !0_usize;
_8 = _6;
_25 = _21.3 as f64;
_21 = (4029286030_u32, _25, 233_u8, (-49_i8));
_27 = !_21.3;
_8 = _10 >> _27;
_10 = _8;
_12 = [_27,_21.3];
_27 = _21.3 | _21.3;
_27 = !_21.3;
_25 = _21.1 - _21.1;
_7 = _3 << _21.0;
_24 = _13 as u16;
_20 = Adt17::Variant0 { fld0: (-20042_i16),fld1: _21.2 };
_15 = [_24];
_33.0 = core::ptr::addr_of!(_21.0);
place!(Field::<u8>(Variant(_20, 0), 1)) = 205713039041869848049908614493413674892_u128 as u8;
_26 = _2 - _2;
_20 = Adt17::Variant1 { fld0: _10,fld1: _24,fld2: _14,fld3: _27,fld4: _19 };
match _21.0 {
0 => bb1,
4029286030 => bb9,
_ => bb8
}
}
bb14 = {
_12 = [_21.3,_21.3];
_5 = _36 as i128;
_15 = [_24];
_33.1 = _21.0;
_21 = (_33.1, _25, Field::<u8>(Variant(_20, 0), 1), _27);
_41 = [_9];
_21.2 = Field::<u8>(Variant(_20, 0), 1);
_35 = [_21.2,_21.2,_21.2,_21.2,_21.2];
place!(Field::<i16>(Variant(_20, 0), 0)) = -(-14157_i16);
_34 = _21.1 + _21.1;
_21.2 = Field::<u8>(Variant(_20, 0), 1);
_33.0 = core::ptr::addr_of!(_21.0);
_11 = _36;
_21.1 = _33.1 as f64;
_17 = !_13;
_41 = [_9];
_10 = _8;
_11 = !_36;
_24 = !7457_u16;
Goto(bb15)
}
bb15 = {
Call(_46 = dump_var(4_usize, 17_usize, Move(_17), 3_usize, Move(_3), 7_usize, Move(_7), 37_usize, Move(_37)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_46 = dump_var(4_usize, 27_usize, Move(_27), 19_usize, Move(_19), 16_usize, Move(_16), 36_usize, Move(_36)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(4_usize, 13_usize, Move(_13), 15_usize, Move(_15), 24_usize, Move(_24), 41_usize, Move(_41)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_46 = dump_var(4_usize, 14_usize, Move(_14), 47_usize, _47, 47_usize, _47, 47_usize, _47), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: i64,mut _2: i128) -> i64 {
mir! {
type RET = i64;
let _3: f32;
let _4: i128;
let _5: *const i16;
let _6: *const &'static u8;
let _7: i16;
let _8: Adt36;
let _9: [i16; 1];
let _10: [i8; 3];
let _11: isize;
let _12: i128;
let _13: &'static &'static (u32, f64, u8, i8);
let _14: f64;
let _15: i32;
let _16: ((i128, &'static *mut f32), Adt17, *const &'static *mut f32, &'static *mut f32);
let _17: (Adt31, i16);
let _18: usize;
let _19: [char; 1];
let _20: u8;
let _21: u8;
let _22: u8;
let _23: u32;
let _24: f64;
let _25: bool;
let _26: *mut *mut f32;
let _27: ();
let _28: ();
{
_2 = (-52560353245209801463944021295487430098_i128);
RET = 4_usize as i64;
RET = _2 as i64;
RET = !_1;
RET = !_1;
RET = _1;
_2 = (-166597367323559607805310900124044492887_i128);
RET = _1;
_3 = 113_i8 as f32;
_4 = _2 - _2;
_3 = 8227429266981152306_u64 as f32;
_3 = 76_isize as f32;
_9 = [(-17238_i16)];
_3 = 11831003825447257004_u64 as f32;
_10 = [(-71_i8),(-44_i8),(-123_i8)];
match _2 {
173684999597378855658063707307723718569 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_9 = [(-6889_i16)];
RET = _1;
_9 = [17333_i16];
_12 = !_4;
_11 = -9223372036854775807_isize;
_3 = 3151_i16 as f32;
_9 = [(-4349_i16)];
RET = _1;
_7 = _3 as i16;
_5 = core::ptr::addr_of!(_7);
match _2 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
173684999597378855658063707307723718569 => bb10,
_ => bb9
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
Return()
}
bb10 = {
(*_5) = -619_i16;
(*_5) = (-1717_i16) * (-12597_i16);
_1 = -RET;
_4 = _12 + _12;
_9 = [(*_5)];
_3 = _1 as f32;
_10 = [(-78_i8),(-39_i8),(-115_i8)];
(*_5) = (-10038_i16) - (-29183_i16);
_1 = 1643467557_i32 as i64;
_10 = [101_i8,1_i8,63_i8];
Call(_3 = fn6(_10, _10, _4, RET, RET, _2, RET, _2, RET), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
(*_5) = -11545_i16;
_3 = 156_u8 as f32;
RET = _1;
_16.0.0 = -_2;
_1 = _11 as i64;
_2 = _4 | _12;
_4 = _11 as i128;
_9 = [(*_5)];
RET = -_1;
_16.1 = Adt17::Variant0 { fld0: (*_5),fld1: 2_u8 };
_16.2 = core::ptr::addr_of!(_16.3);
_17.1 = !(*_5);
(*_5) = _16.0.0 as i16;
(*_5) = Field::<i16>(Variant(_16.1, 0), 0);
_9 = [(*_5)];
Goto(bb12)
}
bb12 = {
_5 = core::ptr::addr_of!((*_5));
_10 = [76_i8,87_i8,(-56_i8)];
_5 = core::ptr::addr_of!((*_5));
_18 = 3_usize * 3_usize;
(*_5) = _17.1;
_14 = _11 as f64;
_16.1 = Adt17::Variant0 { fld0: (*_5),fld1: 11_u8 };
_17.1 = -(*_5);
place!(Field::<u8>(Variant(_16.1, 0), 1)) = 52100_u16 as u8;
RET = _1 | _1;
_12 = _16.0.0 >> Field::<u8>(Variant(_16.1, 0), 1);
place!(Field::<i16>(Variant(_16.1, 0), 0)) = 22011_u16 as i16;
_15 = true as i32;
_12 = _2;
_14 = 48240_u16 as f64;
_11 = (-120_isize);
_10 = [(-73_i8),0_i8,45_i8];
_14 = 57901_u16 as f64;
_19 = ['\u{77df5}'];
_5 = core::ptr::addr_of!(_17.1);
_2 = _12 ^ _12;
SetDiscriminant(_16.1, 1);
place!(Field::<usize>(Variant(_16.1, 1), 4)) = _18;
_21 = 993980988179473026_u64 as u8;
place!(Field::<isize>(Variant(_16.1, 1), 2)) = _11 << _2;
_16.0.0 = _12;
_14 = 60257225250130254860840645300092962704_u128 as f64;
place!(Field::<usize>(Variant(_16.1, 1), 4)) = 13310724746866746324_u64 as usize;
match _11 {
0 => bb13,
340282366920938463463374607431768211336 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
_9 = [(-6889_i16)];
RET = _1;
_9 = [17333_i16];
_12 = !_4;
_11 = -9223372036854775807_isize;
_3 = 3151_i16 as f32;
_9 = [(-4349_i16)];
RET = _1;
_7 = _3 as i16;
_5 = core::ptr::addr_of!(_7);
match _2 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
173684999597378855658063707307723718569 => bb10,
_ => bb9
}
}
bb15 = {
_14 = _1 as f64;
RET = _1 << _16.0.0;
RET = '\u{75071}' as i64;
_11 = 79_i8 as isize;
_11 = Field::<isize>(Variant(_16.1, 1), 2) + Field::<isize>(Variant(_16.1, 1), 2);
_23 = 3604350104_u32;
(*_5) = _7;
RET = _3 as i64;
_12 = _2;
_17.1 = _7;
_20 = _21 - _21;
_14 = _3 as f64;
_10 = [(-35_i8),117_i8,55_i8];
_21 = _20;
RET = -_1;
place!(Field::<i64>(Variant(_16.1, 1), 0)) = _1;
_18 = Field::<usize>(Variant(_16.1, 1), 4) & Field::<usize>(Variant(_16.1, 1), 4);
_10 = [(-105_i8),(-36_i8),(-71_i8)];
(*_5) = _7;
_2 = !_12;
Goto(bb16)
}
bb16 = {
Call(_27 = dump_var(5_usize, 9_usize, Move(_9), 23_usize, Move(_23), 18_usize, Move(_18), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_27 = dump_var(5_usize, 4_usize, Move(_4), 12_usize, Move(_12), 10_usize, Move(_10), 28_usize, _28), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: [i8; 3],mut _2: [i8; 3],mut _3: i128,mut _4: i64,mut _5: i64,mut _6: i128,mut _7: i64,mut _8: i128,mut _9: i64) -> f32 {
mir! {
type RET = f32;
let _10: Adt31;
let _11: *const &'static *mut f32;
let _12: [u32; 6];
let _13: isize;
let _14: bool;
let _15: isize;
let _16: *mut Adt39;
let _17: *const u32;
let _18: (u64, i64, &'static (u32, f64, u8, i8));
let _19: f32;
let _20: Adt63;
let _21: Adt17;
let _22: i64;
let _23: i32;
let _24: &'static &'static i16;
let _25: *mut *mut (&'static bool, Adt17, &'static &'static bool, &'static &'static i16);
let _26: *const *mut f32;
let _27: char;
let _28: &'static *mut f32;
let _29: ();
let _30: ();
{
_8 = _6 + _3;
_1 = [75_i8,48_i8,28_i8];
RET = _9 as f32;
_9 = -_5;
RET = 69_i8 as f32;
_4 = _5 + _9;
match _6 {
0 => bb1,
1 => bb2,
173684999597378855658063707307723718569 => bb4,
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
_3 = _8 ^ _8;
_5 = !_7;
_8 = _3 + _3;
_6 = _8;
Goto(bb5)
}
bb5 = {
_5 = _4 & _7;
_6 = _3 + _3;
_6 = _8;
_4 = -_5;
_7 = _4 - _4;
_1 = _2;
_3 = _8 >> _6;
RET = (-74_i8) as f32;
_1 = [(-41_i8),(-111_i8),119_i8];
_3 = _8;
RET = (-9_i8) as f32;
_8 = _3;
RET = 15536279298029385625_usize as f32;
_5 = _4 + _7;
_5 = _7 - _7;
_8 = _6 + _6;
_12 = [3882282546_u32,1197959008_u32,1055592086_u32,4054093217_u32,1070419941_u32,1353060373_u32];
_12 = [2104107128_u32,3126822767_u32,3734973026_u32,4106802579_u32,3669711718_u32,3162242028_u32];
Call(_7 = core::intrinsics::bswap(_4), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
RET = (-75_isize) as f32;
_5 = _9 + _4;
RET = 29801_u16 as f32;
_9 = !_7;
RET = (-100_i8) as f32;
_12 = [1368065937_u32,57889979_u32,2653956484_u32,2859280235_u32,4293686144_u32,1432929562_u32];
_8 = _6;
_5 = _4;
RET = 38_i8 as f32;
_8 = _3 << _9;
_12 = [2341196602_u32,3471272546_u32,3314762882_u32,1257416862_u32,3672930233_u32,2979563615_u32];
RET = 10052_u16 as f32;
_14 = !false;
_3 = _8 << _9;
_13 = 62_isize;
_5 = _4 & _9;
RET = _13 as f32;
_13 = !9223372036854775807_isize;
_7 = 1788789972_i32 as i64;
_8 = _3 - _3;
_3 = !_8;
Goto(bb7)
}
bb7 = {
_1 = [100_i8,(-63_i8),(-19_i8)];
_9 = _4;
_14 = true ^ true;
_7 = -_9;
_2 = [70_i8,(-3_i8),40_i8];
_3 = -_8;
_14 = !false;
_8 = !_3;
_2 = _1;
_18.0 = 465934783203863803_u64 * 13046439286145666541_u64;
Call(_8 = fn7(_3, _6, _5, _5, _3), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
RET = 43_u8 as f32;
_5 = -_4;
_5 = _9 >> _8;
_18.1 = _5 ^ _4;
_15 = !_13;
_19 = RET + RET;
_9 = 164_u8 as i64;
_12 = [3033164751_u32,2082969951_u32,2797315774_u32,3772614017_u32,4125294655_u32,1363391558_u32];
_4 = _5;
RET = _19;
_18.1 = _5 & _5;
_3 = _8;
Call(_6 = fn8(_4, _5, _4, _7, _4, _8, _15, _3), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_7 = _18.1 * _5;
_18.0 = 14667976097668287002_u64;
_8 = _3;
_1 = [(-115_i8),85_i8,(-27_i8)];
_19 = RET;
_15 = _13 >> _3;
_20 = Adt63::Variant1 { fld0: 1_usize };
_9 = _4;
_13 = _15 - _15;
_19 = RET;
_19 = RET - RET;
Call(_17 = fn9(_7, _13, _9, _3), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_22 = -_7;
Goto(bb11)
}
bb11 = {
_4 = _5 - _18.1;
_18.0 = (-24930_i16) as u64;
_14 = false;
_3 = _6;
_5 = _18.1 << _7;
place!(Field::<usize>(Variant(_20, 1), 0)) = '\u{7e9d4}' as usize;
_14 = _15 < _13;
Call(_20 = fn10(_18.1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_10 = Move(Field::<(Adt31, i16)>(Variant(_20, 0), 3).0);
_13 = Field::<(u32, f64, u8, i8)>(Variant(_20, 0), 0).0 as isize;
place!(Field::<(*const u32, u32)>(Variant(_20, 0), 4)).0 = Move(_17);
_18.2 = &place!(Field::<(u32, f64, u8, i8)>(Variant(_20, 0), 0));
_18.0 = !Field::<u64>(Variant(_20, 0), 5);
_13 = !_15;
place!(Field::<Adt40>(Variant(_20, 0), 2)).fld0 = core::ptr::addr_of_mut!(place!(Field::<(Adt31, i16)>(Variant(_20, 0), 3)).1);
place!(Field::<(Adt31, i16)>(Variant(_20, 0), 3)).0 = Move(_10);
place!(Field::<(u32, f64, u8, i8)>(Variant(_20, 0), 0)) = (Field::<(*const u32, u32)>(Variant(_20, 0), 4).1, Field::<f64>(Variant(Field::<(Adt31, i16)>(Variant(_20, 0), 3).0, 3), 0), Field::<u8>(Variant(Field::<Adt17>(Variant(Field::<(Adt31, i16)>(Variant(_20, 0), 3).0, 3), 1), 0), 1), 7_i8);
_23 = 692714773_i32 & 77237298_i32;
match Field::<(u32, f64, u8, i8)>(Variant(_20, 0), 0).3 {
0 => bb7,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
6 => bb18,
7 => bb20,
_ => bb19
}
}
bb13 = {
Return()
}
bb14 = {
_22 = -_7;
Goto(bb11)
}
bb15 = {
_7 = _18.1 * _5;
_18.0 = 14667976097668287002_u64;
_8 = _3;
_1 = [(-115_i8),85_i8,(-27_i8)];
_19 = RET;
_15 = _13 >> _3;
_20 = Adt63::Variant1 { fld0: 1_usize };
_9 = _4;
_13 = _15 - _15;
_19 = RET;
_19 = RET - RET;
Call(_17 = fn9(_7, _13, _9, _3), ReturnTo(bb10), UnwindUnreachable())
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
RET = (-75_isize) as f32;
_5 = _9 + _4;
RET = 29801_u16 as f32;
_9 = !_7;
RET = (-100_i8) as f32;
_12 = [1368065937_u32,57889979_u32,2653956484_u32,2859280235_u32,4293686144_u32,1432929562_u32];
_8 = _6;
_5 = _4;
RET = 38_i8 as f32;
_8 = _3 << _9;
_12 = [2341196602_u32,3471272546_u32,3314762882_u32,1257416862_u32,3672930233_u32,2979563615_u32];
RET = 10052_u16 as f32;
_14 = !false;
_3 = _8 << _9;
_13 = 62_isize;
_5 = _4 & _9;
RET = _13 as f32;
_13 = !9223372036854775807_isize;
_7 = 1788789972_i32 as i64;
_8 = _3 - _3;
_3 = !_8;
Goto(bb7)
}
bb19 = {
_3 = _8 ^ _8;
_5 = !_7;
_8 = _3 + _3;
_6 = _8;
Goto(bb5)
}
bb20 = {
place!(Field::<(u32, f64, u8, i8)>(Variant(_20, 0), 0)).3 = 4_usize as i8;
place!(Field::<(Adt31, i16)>(Variant(_20, 0), 3)).1 = Field::<i16>(Variant(Field::<Adt17>(Variant(Field::<(Adt31, i16)>(Variant(_20, 0), 3).0, 3), 1), 0), 0) | Field::<i16>(Variant(Field::<Adt17>(Variant(Field::<(Adt31, i16)>(Variant(_20, 0), 3).0, 3), 1), 0), 0);
_10 = Move(Field::<(Adt31, i16)>(Variant(_20, 0), 3).0);
_17 = Move(Field::<(*const u32, u32)>(Variant(_20, 0), 4).0);
place!(Field::<u8>(Variant(place!(Field::<Adt17>(Variant(_10, 3), 1)), 0), 1)) = !Field::<(u32, f64, u8, i8)>(Variant(_20, 0), 0).2;
_17 = core::ptr::addr_of!(place!(Field::<(u32, f64, u8, i8)>(Variant(_20, 0), 0)).0);
_3 = _6;
_22 = _4;
_22 = _18.1;
_5 = _18.1 >> _15;
_21 = Adt17::Variant1 { fld0: _5,fld1: Field::<Adt40>(Variant(_20, 0), 2).fld3,fld2: _15,fld3: Field::<(u32, f64, u8, i8)>(Variant(_20, 0), 0).3,fld4: 7_usize };
_9 = _18.0 as i64;
place!(Field::<(Adt31, i16)>(Variant(_20, 0), 3)).0 = Move(_10);
place!(Field::<usize>(Variant(_21, 1), 4)) = !169003245876109690_usize;
_15 = Field::<Adt40>(Variant(_20, 0), 2).fld3 as isize;
SetDiscriminant(Field::<(Adt31, i16)>(Variant(_20, 0), 3).0, 0);
Goto(bb21)
}
bb21 = {
Call(_29 = dump_var(6_usize, 6_usize, Move(_6), 9_usize, Move(_9), 3_usize, Move(_3), 4_usize, Move(_4)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_29 = dump_var(6_usize, 2_usize, Move(_2), 15_usize, Move(_15), 14_usize, Move(_14), 30_usize, _30), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: i128,mut _2: i128,mut _3: i64,mut _4: i64,mut _5: i128) -> i128 {
mir! {
type RET = i128;
let _6: [i16; 1];
let _7: (Adt39,);
let _8: (&'static bool, Adt17, &'static &'static bool, &'static &'static i16);
let _9: &'static &'static *const u32;
let _10: f32;
let _11: isize;
let _12: bool;
let _13: ();
let _14: ();
{
RET = _5 >> _3;
_1 = _5 ^ _5;
_4 = -_3;
_3 = -_4;
_6 = [8093_i16];
_3 = 6575407657351581856_u64 as i64;
RET = 68_u8 as i128;
_2 = _1;
_6 = [14830_i16];
_1 = _5 ^ _2;
_1 = -_5;
_8.1 = Adt17::Variant0 { fld0: 15949_i16,fld1: 168_u8 };
_4 = (-97_i8) as i64;
_8.2 = &_8.0;
_8.2 = &_8.0;
_5 = false as i128;
_8.2 = &_8.0;
_8.1 = Adt17::Variant1 { fld0: _3,fld1: 31731_u16,fld2: (-9223372036854775808_isize),fld3: (-125_i8),fld4: 17707335771723102949_usize };
_8.2 = &_8.0;
place!(Field::<u16>(Variant(_8.1, 1), 1)) = 1505_u16 << _2;
Call(_1 = core::intrinsics::bswap(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<i8>(Variant(_8.1, 1), 3)) = 1246144689057142062_u64 as i8;
_3 = _4;
_2 = _1 << _1;
RET = _1 ^ _1;
_1 = (-52_isize) as i128;
_12 = !true;
_8.1 = Adt17::Variant1 { fld0: _3,fld1: 3133_u16,fld2: 9223372036854775807_isize,fld3: (-32_i8),fld4: 4735356071253804911_usize };
_2 = RET | RET;
_8.1 = Adt17::Variant1 { fld0: _3,fld1: 8015_u16,fld2: 9223372036854775807_isize,fld3: (-21_i8),fld4: 2_usize };
_12 = false;
_8.0 = &_12;
Goto(bb2)
}
bb2 = {
Call(_13 = dump_var(7_usize, 6_usize, Move(_6), 12_usize, Move(_12), 5_usize, Move(_5), 14_usize, _14), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: i64,mut _2: i64,mut _3: i64,mut _4: i64,mut _5: i64,mut _6: i128,mut _7: isize,mut _8: i128) -> i128 {
mir! {
type RET = i128;
let _9: f32;
let _10: *mut i16;
let _11: ();
let _12: ();
{
RET = !_6;
_4 = !_3;
_5 = _3 & _3;
RET = _6;
_9 = _2 as f32;
_5 = _7 as i64;
_8 = !RET;
_7 = (-9223372036854775808_isize) & (-103_isize);
RET = _6;
_9 = 31443_i16 as f32;
_2 = _4;
RET = !_8;
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(8_usize, 8_usize, Move(_8), 1_usize, Move(_1), 5_usize, Move(_5), 6_usize, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: i64,mut _2: isize,mut _3: i64,mut _4: i128) -> *const u32 {
mir! {
type RET = *const u32;
let _5: &'static *mut f32;
let _6: &'static Adt17;
let _7: *mut Adt39;
let _8: (i8, u64, f32);
let _9: *const i16;
let _10: Adt63;
let _11: [i8; 2];
let _12: [u16; 1];
let _13: i128;
let _14: i16;
let _15: (i8, u64, f32);
let _16: f32;
let _17: *mut Adt39;
let _18: i64;
let _19: bool;
let _20: [u8; 5];
let _21: *mut u128;
let _22: &'static Adt17;
let _23: *mut u128;
let _24: i128;
let _25: [u8; 5];
let _26: Adt60;
let _27: Adt36;
let _28: i128;
let _29: isize;
let _30: isize;
let _31: u64;
let _32: isize;
let _33: *mut Adt39;
let _34: ();
let _35: ();
{
_2 = -(-9223372036854775808_isize);
_3 = _1;
_3 = !_1;
_2 = (-9223372036854775808_isize);
_2 = -(-9223372036854775808_isize);
_2 = 9223372036854775807_isize;
_1 = _3;
_3 = _4 as i64;
_1 = _3 >> _3;
_3 = 2221344353_u32 as i64;
_1 = -_3;
_3 = _1 << _4;
_1 = _3;
_3 = _1;
_4 = -(-120231935384660040465198974223522493662_i128);
_8.0 = !(-59_i8);
_8.1 = 17767283224602353309_u64;
_2 = (-9223372036854775808_isize);
_4 = (-152856021613312765388610372127643769154_i128) & (-65289869057353240431511718745987399241_i128);
_8.2 = (-881191801_i32) as f32;
_1 = _2 as i64;
_2 = 1_usize as isize;
_1 = _3 | _3;
_1 = _3 + _3;
match _8.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
17767283224602353309 => bb9,
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
_10 = Adt63::Variant1 { fld0: 6_usize };
_11 = [_8.0,_8.0];
_1 = 5166379624395266527_usize as i64;
_8.0 = (-51_i8);
_2 = true as isize;
_2 = -9223372036854775807_isize;
_12 = [18642_u16];
place!(Field::<usize>(Variant(_10, 1), 0)) = 11771856467698837817_usize;
place!(Field::<usize>(Variant(_10, 1), 0)) = true as usize;
_12 = [1662_u16];
_2 = (-1377704001_i32) as isize;
_2 = (-107_isize) & 9223372036854775807_isize;
_14 = 17732_i16;
_8.1 = _14 as u64;
_8.1 = 2170285972881338650_u64 + 14815699042721374604_u64;
_8.0 = (-44_i8) & 22_i8;
_13 = _4 & _4;
match _14 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb4,
17732 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_1 = _2 as i64;
_12 = [43828_u16];
_4 = _13;
_15.2 = -_8.2;
_15.0 = _8.0;
_1 = _3 - _3;
Goto(bb12)
}
bb12 = {
_8.0 = '\u{dd028}' as i8;
_15.1 = !_8.1;
_8.2 = _15.1 as f32;
_8.0 = !_15.0;
_15 = (_8.0, _8.1, _8.2);
_8.0 = 20309_u16 as i8;
_14 = (-11550_i16);
_2 = 66_isize * 56_isize;
_16 = _15.2;
SetDiscriminant(_10, 2);
_8.2 = _16;
_15.0 = _8.0;
_10 = Adt63::Variant1 { fld0: 7_usize };
_8 = (_15.0, _15.1, _16);
_3 = _1 & _1;
_9 = core::ptr::addr_of!(_14);
Goto(bb13)
}
bb13 = {
_2 = (-9223372036854775808_isize) >> _1;
_13 = _4;
_13 = _4;
_9 = core::ptr::addr_of!((*_9));
_19 = !false;
_10 = Adt63::Variant1 { fld0: 3_usize };
_16 = -_8.2;
place!(Field::<usize>(Variant(_10, 1), 0)) = 5_usize >> _2;
_3 = _1 ^ _1;
_8.1 = !_15.1;
_15.1 = !_8.1;
_15.1 = _8.1;
_3 = _1;
_13 = _4;
_1 = _3 & _3;
_9 = core::ptr::addr_of!((*_9));
_8 = (_15.0, _15.1, _15.2);
_20 = [100_u8,20_u8,170_u8,20_u8,107_u8];
_4 = _13 + _13;
match (*_9) {
0 => bb14,
1 => bb15,
2 => bb16,
340282366920938463463374607431768199906 => bb18,
_ => bb17
}
}
bb14 = {
Return()
}
bb15 = {
_1 = _2 as i64;
_12 = [43828_u16];
_4 = _13;
_15.2 = -_8.2;
_15.0 = _8.0;
_1 = _3 - _3;
Goto(bb12)
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
_18 = _15.0 as i64;
_8.2 = _8.1 as f32;
_8.0 = _19 as i8;
_12 = [62262_u16];
_8 = _15;
_13 = _4 + _4;
_3 = !_1;
SetDiscriminant(_10, 2);
_14 = -(-10048_i16);
_20 = [217_u8,223_u8,23_u8,164_u8,97_u8];
Goto(bb19)
}
bb19 = {
_11 = [_15.0,_15.0];
_11 = [_8.0,_15.0];
_9 = core::ptr::addr_of!((*_9));
place!(Field::<usize>(Variant(_10, 2), 0)) = !12966281723780999739_usize;
(*_9) = _8.0 as i16;
_3 = Field::<usize>(Variant(_10, 2), 0) as i64;
_1 = _18 - _3;
_20 = [33_u8,92_u8,148_u8,7_u8,171_u8];
Goto(bb20)
}
bb20 = {
_20 = [98_u8,190_u8,77_u8,250_u8,77_u8];
_15 = (_8.0, _8.1, _16);
_14 = (-22044_i16);
_2 = !(-1_isize);
_19 = true;
place!(Field::<usize>(Variant(_10, 2), 0)) = 11177720790346656723_usize;
(*_9) = (-27519_i16) >> _4;
_18 = _1 - _3;
_19 = !true;
_13 = _15.0 as i128;
(*_9) = -(-8768_i16);
_8 = _15;
place!(Field::<[i16; 1]>(Variant(_10, 2), 1)) = [(*_9)];
(*_9) = !31115_i16;
_25 = _20;
_4 = _13 >> _1;
_16 = _15.2 * _8.2;
match Field::<usize>(Variant(_10, 2), 0) {
0 => bb17,
1 => bb2,
2 => bb16,
3 => bb9,
4 => bb8,
11177720790346656723 => bb22,
_ => bb21
}
}
bb21 = {
Return()
}
bb22 = {
_2 = (-98_isize) | (-56_isize);
_16 = _8.2 * _8.2;
_18 = _3 & _3;
_19 = !true;
_20 = _25;
_19 = false;
_9 = core::ptr::addr_of!((*_9));
_24 = (*_9) as i128;
(*_9) = -3568_i16;
_2 = -90_isize;
_8 = (_15.0, _15.1, _15.2);
match Field::<usize>(Variant(_10, 2), 0) {
0 => bb19,
11177720790346656723 => bb23,
_ => bb20
}
}
bb23 = {
_25 = _20;
_15.2 = _14 as f32;
place!(Field::<usize>(Variant(_10, 2), 0)) = 6_usize - 1_usize;
_13 = _4;
(*_9) = _19 as i16;
_15.2 = _16;
_19 = !true;
_11 = [_8.0,_15.0];
_16 = (*_9) as f32;
_8.0 = _15.0 & _15.0;
_25 = [110_u8,239_u8,29_u8,2_u8,45_u8];
_14 = -18607_i16;
(*_9) = (-9362_i16) * (-11739_i16);
_15.2 = -_16;
_1 = !_18;
Goto(bb24)
}
bb24 = {
_12 = [30773_u16];
(*_9) = _19 as i16;
_3 = '\u{93a59}' as i64;
_8 = (_15.0, _15.1, _15.2);
Goto(bb25)
}
bb25 = {
_28 = 305562202076670906770837138096523325577_u128 as i128;
_29 = -_2;
_15 = _8;
Goto(bb26)
}
bb26 = {
_29 = !_2;
_16 = _8.2 - _15.2;
_12 = [27327_u16];
_30 = -_2;
(*_9) = 24345_i16;
match (*_9) {
0 => bb6,
1 => bb16,
2 => bb13,
3 => bb18,
4 => bb27,
5 => bb28,
6 => bb29,
24345 => bb31,
_ => bb30
}
}
bb27 = {
Return()
}
bb28 = {
Return()
}
bb29 = {
_25 = _20;
_15.2 = _14 as f32;
place!(Field::<usize>(Variant(_10, 2), 0)) = 6_usize - 1_usize;
_13 = _4;
(*_9) = _19 as i16;
_15.2 = _16;
_19 = !true;
_11 = [_8.0,_15.0];
_16 = (*_9) as f32;
_8.0 = _15.0 & _15.0;
_25 = [110_u8,239_u8,29_u8,2_u8,45_u8];
_14 = -18607_i16;
(*_9) = (-9362_i16) * (-11739_i16);
_15.2 = -_16;
_1 = !_18;
Goto(bb24)
}
bb30 = {
Return()
}
bb31 = {
_8 = _15;
_24 = Field::<usize>(Variant(_10, 2), 0) as i128;
_8.1 = Field::<usize>(Variant(_10, 2), 0) as u64;
_8.0 = !_15.0;
_28 = _4;
_8.2 = _16 * _16;
_15.0 = _8.0;
_28 = -_24;
place!(Field::<usize>(Variant(_10, 2), 0)) = 5_usize * 5_usize;
_29 = _30;
_18 = _8.2 as i64;
_1 = _18;
SetDiscriminant(_10, 0);
RET = core::ptr::addr_of!(place!(Field::<(*const u32, u32)>(Variant(_10, 0), 4)).1);
(*RET) = '\u{9e3b9}' as u32;
_8.2 = _16 - _16;
_15 = _8;
(*RET) = !3512018236_u32;
(*RET) = !399430725_u32;
place!(Field::<(u32, f64, u8, i8)>(Variant(_10, 0), 0)).0 = _8.2 as u32;
_30 = _29;
_8.1 = _15.1;
place!(Field::<(*const u32, u32)>(Variant(_10, 0), 4)).1 = !Field::<(u32, f64, u8, i8)>(Variant(_10, 0), 0).0;
(*RET) = Field::<(u32, f64, u8, i8)>(Variant(_10, 0), 0).0;
Goto(bb32)
}
bb32 = {
Call(_34 = dump_var(9_usize, 28_usize, Move(_28), 3_usize, Move(_3), 18_usize, Move(_18), 14_usize, Move(_14)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Call(_34 = dump_var(9_usize, 1_usize, Move(_1), 25_usize, Move(_25), 13_usize, Move(_13), 4_usize, Move(_4)), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: i64) -> Adt63 {
mir! {
type RET = Adt63;
let _2: bool;
let _3: i128;
let _4: isize;
let _5: i32;
let _6: (i8, u64, f32);
let _7: isize;
let _8: *mut (&'static bool, Adt17, &'static &'static bool, &'static &'static i16);
let _9: bool;
let _10: [usize; 7];
let _11: f64;
let _12: f64;
let _13: u8;
let _14: f64;
let _15: *const &'static *mut f32;
let _16: isize;
let _17: *const u32;
let _18: *mut [u32; 6];
let _19: i64;
let _20: isize;
let _21: [isize; 2];
let _22: i32;
let _23: [u32; 6];
let _24: i128;
let _25: bool;
let _26: Adt17;
let _27: (*const i16, *const i16, &'static &'static i16, *const i16);
let _28: isize;
let _29: [i8; 3];
let _30: (Adt39,);
let _31: (&'static bool, Adt17, &'static &'static bool, &'static &'static i16);
let _32: [u32; 6];
let _33: bool;
let _34: &'static Adt34;
let _35: i32;
let _36: isize;
let _37: ();
let _38: ();
{
_1 = 7869358860579510541_i64 & 5911993246971304998_i64;
RET = Adt63::Variant1 { fld0: 7_usize };
place!(Field::<usize>(Variant(RET, 1), 0)) = 5905473818144686161_usize;
SetDiscriminant(RET, 1);
place!(Field::<usize>(Variant(RET, 1), 0)) = 12324319408485455475_usize + 3_usize;
_1 = (-5838883565187712401_i64);
place!(Field::<usize>(Variant(RET, 1), 0)) = 17095935480660872504_usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 62205_u16 as usize;
_1 = -(-7901193316676715537_i64);
place!(Field::<usize>(Variant(RET, 1), 0)) = 6154037732560066152_usize;
_1 = -2029212585411035844_i64;
place!(Field::<usize>(Variant(RET, 1), 0)) = 1378027707402111230_usize | 4613111480950481823_usize;
_3 = !(-135061645459631231116287851416728229132_i128);
Call(place!(Field::<usize>(Variant(RET, 1), 0)) = core::intrinsics::bswap(4204861655098041027_usize), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = -123_isize;
_2 = Field::<usize>(Variant(RET, 1), 0) >= Field::<usize>(Variant(RET, 1), 0);
place!(Field::<usize>(Variant(RET, 1), 0)) = 7_usize;
_4 = 3152992869837738895_u64 as isize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 3_usize;
_2 = false | false;
_1 = 47035_u16 as i64;
_2 = !false;
place!(Field::<usize>(Variant(RET, 1), 0)) = 17747037424026146553_usize - 18180873972022384172_usize;
_2 = !false;
_1 = !5983832284560713808_i64;
SetDiscriminant(RET, 0);
place!(Field::<u64>(Variant(RET, 0), 5)) = !11098356350000000808_u64;
RET = Adt63::Variant1 { fld0: 2_usize };
place!(Field::<usize>(Variant(RET, 1), 0)) = 610778150_u32 as usize;
_6.1 = !14761931547271279569_u64;
_1 = -(-3089888194129684091_i64);
Goto(bb2)
}
bb2 = {
_6.2 = 3_i8 as f32;
place!(Field::<usize>(Variant(RET, 1), 0)) = 22539_i16 as usize;
_6.1 = 11680100321234998345_u64;
_5 = (-31881582_i32);
_3 = (-78330809347596462276925168294688110894_i128);
place!(Field::<usize>(Variant(RET, 1), 0)) = 7265719000384252725_usize;
_7 = !_4;
_1 = (-1442109085147300808_i64) - (-7643761732414384377_i64);
_1 = -1889730036635937576_i64;
_9 = !_2;
_6.0 = 993816409_u32 as i8;
_3 = -69892913482806344077057852898132450117_i128;
_3 = !(-158379038820544117390945148403272546479_i128);
_5 = !(-2000450085_i32);
Goto(bb3)
}
bb3 = {
SetDiscriminant(RET, 2);
_4 = !_7;
_2 = _9 | _9;
RET = Adt63::Variant1 { fld0: 15639442319014914234_usize };
_2 = !_9;
_5 = -1185210548_i32;
_3 = !97596809362188731925349392645906249835_i128;
_12 = _5 as f64;
_6.0 = 3_i8 & 121_i8;
place!(Field::<usize>(Variant(RET, 1), 0)) = _6.1 as usize;
_6.1 = 3391_u16 as u64;
_7 = _6.0 as isize;
_5 = 580792977_i32 | (-51251990_i32);
_13 = !249_u8;
_11 = _12 + _12;
_9 = _2;
_4 = _7 >> Field::<usize>(Variant(RET, 1), 0);
_6.0 = 28_i8 & (-103_i8);
_9 = !_2;
_13 = !232_u8;
place!(Field::<usize>(Variant(RET, 1), 0)) = 16476130540095335742_usize | 5_usize;
_2 = _9 | _9;
_6.1 = !3176574963556200857_u64;
_3 = _6.1 as i128;
_6.1 = 9964919962257797368_u64 & 16827083314407278770_u64;
Goto(bb4)
}
bb4 = {
_9 = _2;
_1 = (-8202354163358960394_i64) >> Field::<usize>(Variant(RET, 1), 0);
place!(Field::<usize>(Variant(RET, 1), 0)) = 6996557258270810605_usize;
_3 = 90329727439512304973207126625395599051_i128 * (-111395503178751366021795337868543315915_i128);
match Field::<usize>(Variant(RET, 1), 0) {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
6996557258270810605 => bb11,
_ => bb10
}
}
bb5 = {
SetDiscriminant(RET, 2);
_4 = !_7;
_2 = _9 | _9;
RET = Adt63::Variant1 { fld0: 15639442319014914234_usize };
_2 = !_9;
_5 = -1185210548_i32;
_3 = !97596809362188731925349392645906249835_i128;
_12 = _5 as f64;
_6.0 = 3_i8 & 121_i8;
place!(Field::<usize>(Variant(RET, 1), 0)) = _6.1 as usize;
_6.1 = 3391_u16 as u64;
_7 = _6.0 as isize;
_5 = 580792977_i32 | (-51251990_i32);
_13 = !249_u8;
_11 = _12 + _12;
_9 = _2;
_4 = _7 >> Field::<usize>(Variant(RET, 1), 0);
_6.0 = 28_i8 & (-103_i8);
_9 = !_2;
_13 = !232_u8;
place!(Field::<usize>(Variant(RET, 1), 0)) = 16476130540095335742_usize | 5_usize;
_2 = _9 | _9;
_6.1 = !3176574963556200857_u64;
_3 = _6.1 as i128;
_6.1 = 9964919962257797368_u64 & 16827083314407278770_u64;
Goto(bb4)
}
bb6 = {
_6.2 = 3_i8 as f32;
place!(Field::<usize>(Variant(RET, 1), 0)) = 22539_i16 as usize;
_6.1 = 11680100321234998345_u64;
_5 = (-31881582_i32);
_3 = (-78330809347596462276925168294688110894_i128);
place!(Field::<usize>(Variant(RET, 1), 0)) = 7265719000384252725_usize;
_7 = !_4;
_1 = (-1442109085147300808_i64) - (-7643761732414384377_i64);
_1 = -1889730036635937576_i64;
_9 = !_2;
_6.0 = 993816409_u32 as i8;
_3 = -69892913482806344077057852898132450117_i128;
_3 = !(-158379038820544117390945148403272546479_i128);
_5 = !(-2000450085_i32);
Goto(bb3)
}
bb7 = {
_4 = -123_isize;
_2 = Field::<usize>(Variant(RET, 1), 0) >= Field::<usize>(Variant(RET, 1), 0);
place!(Field::<usize>(Variant(RET, 1), 0)) = 7_usize;
_4 = 3152992869837738895_u64 as isize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 3_usize;
_2 = false | false;
_1 = 47035_u16 as i64;
_2 = !false;
place!(Field::<usize>(Variant(RET, 1), 0)) = 17747037424026146553_usize - 18180873972022384172_usize;
_2 = !false;
_1 = !5983832284560713808_i64;
SetDiscriminant(RET, 0);
place!(Field::<u64>(Variant(RET, 0), 5)) = !11098356350000000808_u64;
RET = Adt63::Variant1 { fld0: 2_usize };
place!(Field::<usize>(Variant(RET, 1), 0)) = 610778150_u32 as usize;
_6.1 = !14761931547271279569_u64;
_1 = -(-3089888194129684091_i64);
Goto(bb2)
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
_6.0 = -68_i8;
_4 = _7 * _7;
_16 = _7 | _7;
_1 = -(-5591000717818807381_i64);
_9 = _2;
_11 = -_12;
_12 = _11;
_6.2 = (-4113_i16) as f32;
_9 = !_2;
place!(Field::<usize>(Variant(RET, 1), 0)) = 17785006738542093009_usize * 6_usize;
_6.2 = _13 as f32;
Goto(bb12)
}
bb12 = {
_12 = -_11;
SetDiscriminant(RET, 1);
_9 = _3 >= _3;
_20 = 7_usize as isize;
_9 = _2;
_1 = !1637308853415732301_i64;
_6.2 = _6.0 as f32;
place!(Field::<usize>(Variant(RET, 1), 0)) = _6.1 as usize;
_14 = -_12;
SetDiscriminant(RET, 1);
_11 = _12;
place!(Field::<usize>(Variant(RET, 1), 0)) = 7_usize;
_6.1 = 6111895571190328517_u64 | 9359700111394899599_u64;
_6.2 = _12 as f32;
_13 = !220_u8;
Goto(bb13)
}
bb13 = {
_19 = _1;
_21 = [_20,_4];
_22 = !_5;
_19 = _3 as i64;
_3 = _11 as i128;
place!(Field::<usize>(Variant(RET, 1), 0)) = _6.1 as usize;
_10 = [Field::<usize>(Variant(RET, 1), 0),Field::<usize>(Variant(RET, 1), 0),Field::<usize>(Variant(RET, 1), 0),Field::<usize>(Variant(RET, 1), 0),Field::<usize>(Variant(RET, 1), 0),Field::<usize>(Variant(RET, 1), 0),Field::<usize>(Variant(RET, 1), 0)];
_23 = [2250278683_u32,402724600_u32,3941335077_u32,704971075_u32,1337356866_u32,907885263_u32];
_6.2 = _1 as f32;
_14 = _11 + _11;
_2 = !_9;
place!(Field::<usize>(Variant(RET, 1), 0)) = _6.0 as usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 1_usize;
_22 = _5;
_19 = -_1;
RET = Adt63::Variant1 { fld0: 4_usize };
place!(Field::<usize>(Variant(RET, 1), 0)) = 12697196468117828387_usize | 5_usize;
_1 = 1798_i16 as i64;
_24 = 176760935956926447129416734912970120031_u128 as i128;
_13 = 69_u8 + 204_u8;
_12 = _11 + _11;
_16 = _7 ^ _7;
_12 = -_11;
_11 = _12;
_14 = _11 - _12;
Goto(bb14)
}
bb14 = {
_7 = _4;
_1 = 4033397669_u32 as i64;
place!(Field::<usize>(Variant(RET, 1), 0)) = 12582161663015167869_usize;
_6.0 = _20 as i8;
_12 = -_14;
_9 = !_2;
_25 = !_2;
_29 = [_6.0,_6.0,_6.0];
_23 = [1239740405_u32,2656406322_u32,3086921292_u32,3244010226_u32,1350802767_u32,1432815977_u32];
SetDiscriminant(RET, 1);
_14 = _12 + _12;
_22 = _5 & _5;
_26 = Adt17::Variant0 { fld0: 8850_i16,fld1: _13 };
_18 = core::ptr::addr_of_mut!(_23);
_16 = _4 + _4;
Call(RET = fn11(), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
place!(Field::<Adt40>(Variant(RET, 0), 2)).fld0 = core::ptr::addr_of_mut!(place!(Field::<(Adt31, i16)>(Variant(RET, 0), 3)).1);
_13 = !Field::<u8>(Variant(_26, 0), 1);
_31.0 = &_2;
_26 = Adt17::Variant0 { fld0: Field::<(Adt31, i16)>(Variant(RET, 0), 3).1,fld1: Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).2 };
place!(Field::<*mut f32>(Variant(place!(Field::<(Adt31, i16)>(Variant(RET, 0), 3)).0, 1), 0)) = core::ptr::addr_of_mut!(_6.2);
place!(Field::<Adt40>(Variant(RET, 0), 2)).fld3 = _14 as u16;
place!(Field::<Adt40>(Variant(RET, 0), 2)).fld2 = [Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).0,Field::<(*const u32, u32)>(Variant(RET, 0), 4).1,Field::<(*const u32, u32)>(Variant(RET, 0), 4).1,Field::<(*const u32, u32)>(Variant(RET, 0), 4).1,Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).0,Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).0];
_3 = !_24;
_31.2 = &_31.0;
_30.0 = Adt39::Variant1 { fld0: (*_18),fld1: _19,fld2: _14 };
place!(Field::<f64>(Variant(_30.0, 1), 2)) = _11 - _14;
place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)).3 = _6.0 + _6.0;
_31.1 = Adt17::Variant1 { fld0: _1,fld1: Field::<Adt40>(Variant(RET, 0), 2).fld3,fld2: _7,fld3: _6.0,fld4: 14854949247142480467_usize };
place!(Field::<Adt40>(Variant(RET, 0), 2)).fld1 = [_16,_20];
place!(Field::<u64>(Variant(RET, 0), 5)) = Field::<i16>(Variant(_26, 0), 0) as u64;
place!(Field::<u8>(Variant(_26, 0), 1)) = Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).2 ^ _13;
SetDiscriminant(_30.0, 1);
_23 = [Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).0,Field::<(*const u32, u32)>(Variant(RET, 0), 4).1,Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).0,Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).0,Field::<(*const u32, u32)>(Variant(RET, 0), 4).1,Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).0];
_28 = _7 >> _13;
_27.3 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_26, 0), 0)));
place!(Field::<(Adt31, i16)>(Variant(RET, 0), 3)).1 = Field::<Adt40>(Variant(RET, 0), 2).fld3 as i16;
_30.0 = Adt39::Variant3 { fld0: Field::<u8>(Variant(_26, 0), 1),fld1: _5,fld2: Move(Field::<Adt40>(Variant(RET, 0), 2).fld0) };
_27.1 = core::ptr::addr_of!(place!(Field::<(Adt31, i16)>(Variant(RET, 0), 3)).1);
place!(Field::<(Adt31, i16)>(Variant(RET, 0), 3)).0 = Adt31::Variant3 { fld0: Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).1,fld1: Move(_26) };
place!(Field::<Adt40>(Variant(RET, 0), 2)).fld0 = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(place!(Field::<Adt17>(Variant(place!(Field::<(Adt31, i16)>(Variant(RET, 0), 3)).0, 3), 1)), 0), 0)));
Goto(bb16)
}
bb16 = {
Call(_37 = dump_var(10_usize, 2_usize, Move(_2), 16_usize, Move(_16), 21_usize, Move(_21), 28_usize, Move(_28)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(10_usize, 10_usize, Move(_10), 1_usize, Move(_1), 9_usize, Move(_9), 23_usize, Move(_23)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(10_usize, 24_usize, Move(_24), 38_usize, _38, 38_usize, _38, 38_usize, _38), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11() -> Adt63 {
mir! {
type RET = Adt63;
let _1: char;
let _2: *const &'static *mut f32;
let _3: &'static i16;
let _4: isize;
let _5: *const &'static u8;
let _6: u32;
let _7: char;
let _8: ((*const i16, *const i16, &'static &'static i16, *const i16), isize, char);
let _9: Adt17;
let _10: [isize; 2];
let _11: bool;
let _12: isize;
let _13: isize;
let _14: f64;
let _15: &'static &'static *mut f32;
let _16: &'static &'static i16;
let _17: f32;
let _18: char;
let _19: ((i8, u64, f32), *mut f32, &'static *mut f32);
let _20: i16;
let _21: (&'static &'static bool,);
let _22: &'static (u32, f64, u8, i8);
let _23: isize;
let _24: f64;
let _25: usize;
let _26: i8;
let _27: [i8; 2];
let _28: isize;
let _29: *mut [u32; 6];
let _30: (i32,);
let _31: u128;
let _32: isize;
let _33: f64;
let _34: *mut f32;
let _35: [u32; 6];
let _36: ();
let _37: ();
{
RET = Adt63::Variant1 { fld0: 1423895794496842666_usize };
place!(Field::<usize>(Variant(RET, 1), 0)) = 5_usize >> (-6950136308608283171_i64);
place!(Field::<usize>(Variant(RET, 1), 0)) = !75465326328220823_usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 6890144730492422300_usize * 4914525392083869045_usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 2_usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = !1286748063278725209_usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 1_usize - 0_usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 15506584410866752657_usize;
RET = Adt63::Variant1 { fld0: 15865167918963610332_usize };
place!(Field::<usize>(Variant(RET, 1), 0)) = false as usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = !16012804041372316987_usize;
SetDiscriminant(RET, 2);
place!(Field::<[i16; 1]>(Variant(RET, 2), 1)) = [(-29629_i16)];
_1 = '\u{93878}';
_1 = '\u{d5c2e}';
place!(Field::<usize>(Variant(RET, 2), 0)) = 16684881291800617906_usize >> (-67_i8);
Call(place!(Field::<[i16; 1]>(Variant(RET, 2), 1)) = fn12(Field::<usize>(Variant(RET, 2), 0), Field::<usize>(Variant(RET, 2), 0), _1, _1, _1, _1, Field::<usize>(Variant(RET, 2), 0), Field::<usize>(Variant(RET, 2), 0), _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<[i16; 1]>(Variant(RET, 2), 1)) = [7310_i16];
place!(Field::<usize>(Variant(RET, 2), 0)) = 18178412320551187753_usize;
match Field::<usize>(Variant(RET, 2), 0) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
18178412320551187753 => bb7,
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
SetDiscriminant(RET, 2);
RET = Adt63::Variant1 { fld0: 5_usize };
place!(Field::<usize>(Variant(RET, 1), 0)) = 7_usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 14480662179567201668_usize ^ 6_usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 55_u8 as usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 10892985754460844756_usize * 2_usize;
Goto(bb8)
}
bb8 = {
place!(Field::<usize>(Variant(RET, 1), 0)) = 1965615434548863494_usize & 7_usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = !6_usize;
_1 = '\u{1a885}';
place!(Field::<usize>(Variant(RET, 1), 0)) = !7_usize;
_1 = '\u{48d0b}';
_1 = '\u{77e15}';
_1 = '\u{e754e}';
place!(Field::<usize>(Variant(RET, 1), 0)) = 6_usize + 6_usize;
_1 = '\u{cd113}';
place!(Field::<usize>(Variant(RET, 1), 0)) = 78768666304004059855593399995386737190_u128 as usize;
RET = Adt63::Variant1 { fld0: 3_usize };
place!(Field::<usize>(Variant(RET, 1), 0)) = (-1246701964821184452_i64) as usize;
_1 = '\u{e867c}';
_4 = 660716035_u32 as isize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 6_usize << _4;
Goto(bb9)
}
bb9 = {
_6 = !1138129710_u32;
place!(Field::<usize>(Variant(RET, 1), 0)) = 2_usize;
RET = Adt63::Variant1 { fld0: 1_usize };
_6 = false as u32;
place!(Field::<usize>(Variant(RET, 1), 0)) = (-3205658822547010874_i64) as usize;
_6 = false as u32;
_6 = (-1737749183_i32) as u32;
_6 = 1643084938_u32;
place!(Field::<usize>(Variant(RET, 1), 0)) = 3329382033097237951_usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 15103652245189884657_usize - 7981002373529972333_usize;
_6 = 3837973032_u32 ^ 1919649731_u32;
_8.0.2 = &_3;
_8.1 = !_4;
_10 = [_8.1,_8.1];
RET = Adt63::Variant1 { fld0: 6_usize };
_7 = _1;
_8.2 = _7;
_6 = !359635895_u32;
RET = Adt63::Variant1 { fld0: 1_usize };
Goto(bb10)
}
bb10 = {
place!(Field::<usize>(Variant(RET, 1), 0)) = !6738484126692470333_usize;
_8.0.2 = &_3;
_1 = _8.2;
_4 = 195113365_i32 as isize;
_10 = [_4,_8.1];
_8.0.2 = &_3;
_6 = 2787635642_u32 >> _4;
_7 = _8.2;
_8.1 = _4;
_7 = _1;
_11 = false;
_8.2 = _7;
_8.1 = _4 | _4;
SetDiscriminant(RET, 0);
place!(Field::<u64>(Variant(RET, 0), 5)) = 15831706415664772026_u64 - 6426789040030897613_u64;
place!(Field::<(*const u32, u32)>(Variant(RET, 0), 4)).0 = core::ptr::addr_of!(place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)).0);
_4 = _11 as isize;
place!(Field::<Adt40>(Variant(RET, 0), 2)).fld0 = core::ptr::addr_of_mut!(place!(Field::<(Adt31, i16)>(Variant(RET, 0), 3)).1);
place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)).2 = 76_u8 - 185_u8;
Goto(bb11)
}
bb11 = {
_8.0.0 = core::ptr::addr_of!(place!(Field::<(Adt31, i16)>(Variant(RET, 0), 3)).1);
place!(Field::<Adt40>(Variant(RET, 0), 2)).fld3 = _11 as u16;
_1 = _8.2;
place!(Field::<Adt40>(Variant(RET, 0), 2)).fld1 = [_8.1,_8.1];
place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)).0 = _6;
place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)).3 = (-368895830604068810_i64) as i8;
_9 = Adt17::Variant0 { fld0: 31431_i16,fld1: Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).2 };
Goto(bb12)
}
bb12 = {
place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)).1 = Field::<Adt40>(Variant(RET, 0), 2).fld3 as f64;
place!(Field::<i16>(Variant(_9, 0), 0)) = 25127_i16;
place!(Field::<(*const u32, u32)>(Variant(RET, 0), 4)).1 = _6;
place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)).2 = Field::<u8>(Variant(_9, 0), 1);
_1 = _8.2;
place!(Field::<Adt40>(Variant(RET, 0), 2)).fld3 = Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).0 as u16;
place!(Field::<char>(Variant(RET, 0), 1)) = _7;
place!(Field::<Adt40>(Variant(RET, 0), 2)).fld2 = [Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).0,Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).0,Field::<(*const u32, u32)>(Variant(RET, 0), 4).1,_6,_6,Field::<(*const u32, u32)>(Variant(RET, 0), 4).1];
_16 = &_3;
_8.0.1 = core::ptr::addr_of!(place!(Field::<(Adt31, i16)>(Variant(RET, 0), 3)).1);
place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)).3 = 16_i8;
place!(Field::<Adt40>(Variant(RET, 0), 2)).fld1 = [_4,_8.1];
_9 = Adt17::Variant1 { fld0: (-6953667391964656217_i64),fld1: Field::<Adt40>(Variant(RET, 0), 2).fld3,fld2: _4,fld3: Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).3,fld4: 3_usize };
_6 = 4657_i16 as u32;
_16 = &(*_16);
place!(Field::<i64>(Variant(_9, 1), 0)) = -9019281197215246391_i64;
_1 = _8.2;
_10 = Field::<Adt40>(Variant(RET, 0), 2).fld1;
_19.0.2 = Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).3 as f32;
_7 = Field::<char>(Variant(RET, 0), 1);
_8.0.0 = Move(_8.0.1);
place!(Field::<Adt40>(Variant(RET, 0), 2)).fld3 = Field::<u16>(Variant(_9, 1), 1) >> Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).3;
_19.1 = core::ptr::addr_of_mut!(_17);
match Field::<i8>(Variant(_9, 1), 3) {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
16 => bb20,
_ => bb19
}
}
bb13 = {
_8.0.0 = core::ptr::addr_of!(place!(Field::<(Adt31, i16)>(Variant(RET, 0), 3)).1);
place!(Field::<Adt40>(Variant(RET, 0), 2)).fld3 = _11 as u16;
_1 = _8.2;
place!(Field::<Adt40>(Variant(RET, 0), 2)).fld1 = [_8.1,_8.1];
place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)).0 = _6;
place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)).3 = (-368895830604068810_i64) as i8;
_9 = Adt17::Variant0 { fld0: 31431_i16,fld1: Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).2 };
Goto(bb12)
}
bb14 = {
Return()
}
bb15 = {
place!(Field::<[i16; 1]>(Variant(RET, 2), 1)) = [7310_i16];
place!(Field::<usize>(Variant(RET, 2), 0)) = 18178412320551187753_usize;
match Field::<usize>(Variant(RET, 2), 0) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
18178412320551187753 => bb7,
_ => bb6
}
}
bb16 = {
place!(Field::<usize>(Variant(RET, 1), 0)) = 1965615434548863494_usize & 7_usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = !6_usize;
_1 = '\u{1a885}';
place!(Field::<usize>(Variant(RET, 1), 0)) = !7_usize;
_1 = '\u{48d0b}';
_1 = '\u{77e15}';
_1 = '\u{e754e}';
place!(Field::<usize>(Variant(RET, 1), 0)) = 6_usize + 6_usize;
_1 = '\u{cd113}';
place!(Field::<usize>(Variant(RET, 1), 0)) = 78768666304004059855593399995386737190_u128 as usize;
RET = Adt63::Variant1 { fld0: 3_usize };
place!(Field::<usize>(Variant(RET, 1), 0)) = (-1246701964821184452_i64) as usize;
_1 = '\u{e867c}';
_4 = 660716035_u32 as isize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 6_usize << _4;
Goto(bb9)
}
bb17 = {
SetDiscriminant(RET, 2);
RET = Adt63::Variant1 { fld0: 5_usize };
place!(Field::<usize>(Variant(RET, 1), 0)) = 7_usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 14480662179567201668_usize ^ 6_usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 55_u8 as usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 10892985754460844756_usize * 2_usize;
Goto(bb8)
}
bb18 = {
Return()
}
bb19 = {
Return()
}
bb20 = {
_12 = !_4;
place!(Field::<Adt40>(Variant(RET, 0), 2)).fld3 = Field::<u16>(Variant(_9, 1), 1) ^ Field::<u16>(Variant(_9, 1), 1);
place!(Field::<isize>(Variant(_9, 1), 2)) = _19.0.2 as isize;
place!(Field::<usize>(Variant(_9, 1), 4)) = Field::<Adt40>(Variant(RET, 0), 2).fld3 as usize;
_23 = Field::<isize>(Variant(_9, 1), 2);
place!(Field::<(Adt31, i16)>(Variant(RET, 0), 3)).1 = 18518_i16 >> Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).0;
_6 = !Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).0;
_15 = &_19.2;
_17 = _19.0.2 * _19.0.2;
match Field::<i8>(Variant(_9, 1), 3) {
0 => bb21,
16 => bb23,
_ => bb22
}
}
bb21 = {
Return()
}
bb22 = {
SetDiscriminant(RET, 2);
RET = Adt63::Variant1 { fld0: 5_usize };
place!(Field::<usize>(Variant(RET, 1), 0)) = 7_usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 14480662179567201668_usize ^ 6_usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 55_u8 as usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 10892985754460844756_usize * 2_usize;
Goto(bb8)
}
bb23 = {
_19.0.0 = Field::<(*const u32, u32)>(Variant(RET, 0), 4).1 as i8;
_2 = core::ptr::addr_of!((*_15));
place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)).0 = Field::<usize>(Variant(_9, 1), 4) as u32;
place!(Field::<(Adt31, i16)>(Variant(RET, 0), 3)).0 = Adt31::Variant2 { fld0: Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).0,fld1: Move(_8.0.0),fld2: _4,fld3: 29936009338241361147476377143441952568_i128,fld4: Field::<Adt40>(Variant(RET, 0), 2).fld2 };
_14 = Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).1 + Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).1;
_3 = &_20;
_16 = &_3;
place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)).1 = 114088578903688178381100460660786100369_i128 as f64;
_19.2 = &_19.1;
_22 = &place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0));
SetDiscriminant(_9, 0);
place!(Field::<(*const u32, u32)>(Variant(RET, 0), 4)).0 = core::ptr::addr_of!((*_22).0);
place!(Field::<Adt40>(Variant(RET, 0), 2)).fld0 = core::ptr::addr_of_mut!((*_3));
_24 = -(*_22).1;
_8.0.1 = core::ptr::addr_of!((*_3));
_19.0 = (Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).3, Field::<u64>(Variant(RET, 0), 5), _17);
_28 = Field::<isize>(Variant(Field::<(Adt31, i16)>(Variant(RET, 0), 3).0, 2), 2) + _8.1;
place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)).2 = 131_u8 << _28;
_18 = _1;
place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)) = (Field::<(*const u32, u32)>(Variant(RET, 0), 4).1, _14, 111_u8, _19.0.0);
place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)).2 = 72_u8 | 192_u8;
place!(Field::<i16>(Variant(_9, 0), 0)) = !Field::<(Adt31, i16)>(Variant(RET, 0), 3).1;
_6 = Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).3 as u32;
match _19.0.0 {
0 => bb2,
1 => bb24,
16 => bb26,
_ => bb25
}
}
bb24 = {
SetDiscriminant(RET, 2);
RET = Adt63::Variant1 { fld0: 5_usize };
place!(Field::<usize>(Variant(RET, 1), 0)) = 7_usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 14480662179567201668_usize ^ 6_usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 55_u8 as usize;
place!(Field::<usize>(Variant(RET, 1), 0)) = 10892985754460844756_usize * 2_usize;
Goto(bb8)
}
bb25 = {
Return()
}
bb26 = {
_19.2 = &_19.1;
_31 = _11 as u128;
_8.2 = _1;
place!(Field::<(Adt31, i16)>(Variant(RET, 0), 3)).0 = Adt31::Variant1 { fld0: Move(_19.1) };
place!(Field::<(*const u32, u32)>(Variant(RET, 0), 4)).0 = core::ptr::addr_of!(place!(Field::<(*const u32, u32)>(Variant(RET, 0), 4)).1);
_20 = Field::<(Adt31, i16)>(Variant(RET, 0), 3).1 | Field::<(Adt31, i16)>(Variant(RET, 0), 3).1;
place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)).3 = _19.0.0 >> _23;
place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)).3 = _19.0.0 + _19.0.0;
_8.0.1 = core::ptr::addr_of!(place!(Field::<(Adt31, i16)>(Variant(RET, 0), 3)).1);
place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)).1 = _24;
place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)).3 = _19.0.0;
_16 = &(*_16);
_3 = &_20;
_19.0 = (Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).3, Field::<u64>(Variant(RET, 0), 5), _17);
place!(Field::<(Adt31, i16)>(Variant(RET, 0), 3)).1 = (*_3);
place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0)).3 = _19.0.0;
_24 = Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).1;
_9 = Adt17::Variant1 { fld0: 4788872329140808889_i64,fld1: Field::<Adt40>(Variant(RET, 0), 2).fld3,fld2: _12,fld3: Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).3,fld4: 1_usize };
_6 = Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).0 ^ Field::<(*const u32, u32)>(Variant(RET, 0), 4).1;
place!(Field::<Adt40>(Variant(RET, 0), 2)).fld2 = [Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).0,_6,_6,Field::<(*const u32, u32)>(Variant(RET, 0), 4).1,Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0).0,_6];
_22 = &place!(Field::<(u32, f64, u8, i8)>(Variant(RET, 0), 0));
_12 = !_4;
place!(Field::<Adt40>(Variant(RET, 0), 2)).fld1 = _10;
Goto(bb27)
}
bb27 = {
Call(_36 = dump_var(11_usize, 10_usize, Move(_10), 4_usize, Move(_4), 6_usize, Move(_6), 23_usize, Move(_23)), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Call(_36 = dump_var(11_usize, 11_usize, Move(_11), 20_usize, Move(_20), 37_usize, _37, 37_usize, _37), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: usize,mut _2: usize,mut _3: char,mut _4: char,mut _5: char,mut _6: char,mut _7: usize,mut _8: usize,mut _9: char) -> [i16; 1] {
mir! {
type RET = [i16; 1];
let _10: Adt34;
let _11: [usize; 4];
let _12: i128;
let _13: [i8; 3];
let _14: f32;
let _15: f64;
let _16: isize;
let _17: (i8, u64, f32);
let _18: &'static &'static *mut f32;
let _19: f64;
let _20: bool;
let _21: i64;
let _22: *const *mut f32;
let _23: usize;
let _24: &'static i16;
let _25: u128;
let _26: (*const i16, *const i16, &'static &'static i16, *const i16);
let _27: char;
let _28: (u32, f64, u8, i8);
let _29: [i16; 1];
let _30: u8;
let _31: *mut u128;
let _32: isize;
let _33: ((i128, &'static *mut f32), Adt17, *const &'static *mut f32, &'static *mut f32);
let _34: [i16; 1];
let _35: Adt39;
let _36: f32;
let _37: char;
let _38: &'static i16;
let _39: isize;
let _40: i8;
let _41: ();
let _42: ();
{
_1 = _8 >> _7;
RET = [5358_i16];
_4 = _5;
_2 = _8;
RET = [16183_i16];
RET = [(-29126_i16)];
_7 = !_2;
_1 = (-4827_i16) as usize;
_8 = _2 >> _2;
_11 = [_2,_1,_1,_2];
_1 = 9223372036854775807_isize as usize;
_13 = [(-5_i8),(-107_i8),106_i8];
_2 = (-9223372036854775808_isize) as usize;
_7 = _8 & _1;
Goto(bb1)
}
bb1 = {
_4 = _3;
_4 = _5;
_4 = _9;
_9 = _3;
RET = [15056_i16];
_8 = _7;
_9 = _5;
_4 = _9;
_9 = _5;
_14 = (-9223372036854775808_isize) as f32;
RET = [(-28304_i16)];
_14 = 217733335596422244595748759368966858019_u128 as f32;
_3 = _6;
_15 = (-122_i8) as f64;
RET = [(-19374_i16)];
_6 = _5;
_4 = _6;
_13 = [(-46_i8),50_i8,(-51_i8)];
_7 = 30088_u16 as usize;
_8 = 75_u8 as usize;
_17.1 = !11446741057545031662_u64;
_6 = _5;
_8 = true as usize;
Goto(bb2)
}
bb2 = {
_4 = _5;
_17.2 = -_14;
_16 = _17.1 as isize;
_17.1 = 8060_u16 as u64;
_17.2 = -_14;
_9 = _5;
_17.0 = (-21_i8);
_19 = _15 - _15;
_15 = _19 + _19;
RET = [(-9953_i16)];
_12 = false as i128;
_7 = _2;
_20 = _6 <= _4;
RET = [13737_i16];
_8 = _7 * _7;
_11 = [_8,_8,_8,_7];
_4 = _5;
_3 = _4;
_8 = _2 >> _12;
_17 = ((-59_i8), 16693038093018138481_u64, _14);
_16 = 9223372036854775807_isize ^ (-100_isize);
_11 = [_8,_7,_1,_2];
Call(_5 = fn13(_17.0, _9, _3, _15, _15, _9, _17.0, _15, _1, _17.0, _16, _19), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = _8;
_19 = _15 * _15;
_21 = 7188272898663423187_i64;
_17.2 = -_14;
_4 = _6;
_17 = ((-77_i8), 5096309168480177003_u64, _14);
Goto(bb4)
}
bb4 = {
_17.1 = 11630703442496718780_u64 * 416768011890958292_u64;
RET = [13609_i16];
_11 = [_8,_2,_8,_8];
_3 = _6;
_4 = _9;
_21 = _17.1 as i64;
_2 = _8 | _8;
RET = [29613_i16];
_19 = _15 - _15;
_20 = _8 >= _2;
_16 = (-9223372036854775808_isize);
_2 = _8 >> _7;
_2 = !_8;
_8 = !_7;
RET = [(-16770_i16)];
_6 = _3;
RET = [7723_i16];
Goto(bb5)
}
bb5 = {
_17.0 = (-16408_i16) as i8;
_21 = _17.2 as i64;
RET = [27196_i16];
_21 = -(-8993704910654750338_i64);
_8 = _7 << _17.1;
_6 = _4;
RET = [19474_i16];
_27 = _4;
RET = [(-299_i16)];
_20 = false;
_11 = [_7,_8,_8,_1];
_28.1 = _19;
_15 = -_19;
_3 = _4;
_31 = core::ptr::addr_of_mut!(_25);
_5 = _9;
_13 = [_17.0,_17.0,_17.0];
_30 = 196_u8 * 226_u8;
_25 = !235931237485570360739267007603179678733_u128;
match _16 {
0 => bb3,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb6 = {
_17.1 = 11630703442496718780_u64 * 416768011890958292_u64;
RET = [13609_i16];
_11 = [_8,_2,_8,_8];
_3 = _6;
_4 = _9;
_21 = _17.1 as i64;
_2 = _8 | _8;
RET = [29613_i16];
_19 = _15 - _15;
_20 = _8 >= _2;
_16 = (-9223372036854775808_isize);
_2 = _8 >> _7;
_2 = !_8;
_8 = !_7;
RET = [(-16770_i16)];
_6 = _3;
RET = [7723_i16];
Goto(bb5)
}
bb7 = {
_1 = _8;
_19 = _15 * _15;
_21 = 7188272898663423187_i64;
_17.2 = -_14;
_4 = _6;
_17 = ((-77_i8), 5096309168480177003_u64, _14);
Goto(bb4)
}
bb8 = {
_4 = _5;
_17.2 = -_14;
_16 = _17.1 as isize;
_17.1 = 8060_u16 as u64;
_17.2 = -_14;
_9 = _5;
_17.0 = (-21_i8);
_19 = _15 - _15;
_15 = _19 + _19;
RET = [(-9953_i16)];
_12 = false as i128;
_7 = _2;
_20 = _6 <= _4;
RET = [13737_i16];
_8 = _7 * _7;
_11 = [_8,_8,_8,_7];
_4 = _5;
_3 = _4;
_8 = _2 >> _12;
_17 = ((-59_i8), 16693038093018138481_u64, _14);
_16 = 9223372036854775807_isize ^ (-100_isize);
_11 = [_8,_7,_1,_2];
Call(_5 = fn13(_17.0, _9, _3, _15, _15, _9, _17.0, _15, _1, _17.0, _16, _19), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_4 = _3;
_4 = _5;
_4 = _9;
_9 = _3;
RET = [15056_i16];
_8 = _7;
_9 = _5;
_4 = _9;
_9 = _5;
_14 = (-9223372036854775808_isize) as f32;
RET = [(-28304_i16)];
_14 = 217733335596422244595748759368966858019_u128 as f32;
_3 = _6;
_15 = (-122_i8) as f64;
RET = [(-19374_i16)];
_6 = _5;
_4 = _6;
_13 = [(-46_i8),50_i8,(-51_i8)];
_7 = 30088_u16 as usize;
_8 = 75_u8 as usize;
_17.1 = !11446741057545031662_u64;
_6 = _5;
_8 = true as usize;
Goto(bb2)
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
_13 = [_17.0,_17.0,_17.0];
_8 = !_7;
_29 = [(-23743_i16)];
RET = [(-5424_i16)];
(*_31) = 165973623507809685106979702743991809630_u128 ^ 67469078341444427740032164312587856214_u128;
_30 = !32_u8;
_28.1 = _19;
_33.2 = core::ptr::addr_of!(_33.0.1);
RET = [10394_i16];
_9 = _6;
_28.0 = !45217518_u32;
_12 = -126877790958191983855330122829650033661_i128;
_17 = (40_i8, 13767956001978186108_u64, _14);
_28.3 = _20 as i8;
RET = [23979_i16];
_19 = _15;
_8 = !_2;
_11 = [_8,_2,_1,_8];
_28.3 = _17.0 | _17.0;
_12 = -132273060259794222962835919976043301140_i128;
_28.2 = _17.2 as u8;
_16 = !(-77_isize);
_26.2 = &_24;
_20 = _15 <= _19;
(*_31) = !234168222768123270263649633455674674828_u128;
_19 = _15;
_30 = _17.2 as u8;
match _17.0 {
0 => bb3,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
40 => bb19,
_ => bb18
}
}
bb14 = {
Return()
}
bb15 = {
_17.1 = 11630703442496718780_u64 * 416768011890958292_u64;
RET = [13609_i16];
_11 = [_8,_2,_8,_8];
_3 = _6;
_4 = _9;
_21 = _17.1 as i64;
_2 = _8 | _8;
RET = [29613_i16];
_19 = _15 - _15;
_20 = _8 >= _2;
_16 = (-9223372036854775808_isize);
_2 = _8 >> _7;
_2 = !_8;
_8 = !_7;
RET = [(-16770_i16)];
_6 = _3;
RET = [7723_i16];
Goto(bb5)
}
bb16 = {
_1 = _8;
_19 = _15 * _15;
_21 = 7188272898663423187_i64;
_17.2 = -_14;
_4 = _6;
_17 = ((-77_i8), 5096309168480177003_u64, _14);
Goto(bb4)
}
bb17 = {
_17.0 = (-16408_i16) as i8;
_21 = _17.2 as i64;
RET = [27196_i16];
_21 = -(-8993704910654750338_i64);
_8 = _7 << _17.1;
_6 = _4;
RET = [19474_i16];
_27 = _4;
RET = [(-299_i16)];
_20 = false;
_11 = [_7,_8,_8,_1];
_28.1 = _19;
_15 = -_19;
_3 = _4;
_31 = core::ptr::addr_of_mut!(_25);
_5 = _9;
_13 = [_17.0,_17.0,_17.0];
_30 = 196_u8 * 226_u8;
_25 = !235931237485570360739267007603179678733_u128;
match _16 {
0 => bb3,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb18 = {
_4 = _5;
_17.2 = -_14;
_16 = _17.1 as isize;
_17.1 = 8060_u16 as u64;
_17.2 = -_14;
_9 = _5;
_17.0 = (-21_i8);
_19 = _15 - _15;
_15 = _19 + _19;
RET = [(-9953_i16)];
_12 = false as i128;
_7 = _2;
_20 = _6 <= _4;
RET = [13737_i16];
_8 = _7 * _7;
_11 = [_8,_8,_8,_7];
_4 = _5;
_3 = _4;
_8 = _2 >> _12;
_17 = ((-59_i8), 16693038093018138481_u64, _14);
_16 = 9223372036854775807_isize ^ (-100_isize);
_11 = [_8,_7,_1,_2];
Call(_5 = fn13(_17.0, _9, _3, _15, _15, _9, _17.0, _15, _1, _17.0, _16, _19), ReturnTo(bb3), UnwindUnreachable())
}
bb19 = {
_16 = (-34_isize);
_20 = false ^ false;
_33.0.0 = _12 * _12;
_33.1 = Adt17::Variant0 { fld0: 24191_i16,fld1: _28.2 };
_17.1 = 3901796748107107128_u64;
_26.3 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_33.1, 0), 0)));
_14 = _17.2 * _17.2;
_6 = _9;
RET = [(-3376_i16)];
_28.0 = !69823828_u32;
_32 = _16 * _16;
_15 = -_28.1;
_17.2 = -_14;
_21 = _17.1 as i64;
_17 = (_28.3, 3934958983866739888_u64, _14);
_26.2 = &_24;
_18 = &_33.3;
_17.0 = _17.1 as i8;
_34 = _29;
_33.0.0 = _12 | _12;
_36 = -_14;
_13 = [_28.3,_17.0,_28.3];
_18 = &(*_18);
Goto(bb20)
}
bb20 = {
Call(_41 = dump_var(12_usize, 7_usize, Move(_7), 6_usize, Move(_6), 9_usize, Move(_9), 20_usize, Move(_20)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_41 = dump_var(12_usize, 4_usize, Move(_4), 32_usize, Move(_32), 2_usize, Move(_2), 34_usize, Move(_34)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_41 = dump_var(12_usize, 29_usize, Move(_29), 30_usize, Move(_30), 42_usize, _42, 42_usize, _42), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: i8,mut _2: char,mut _3: char,mut _4: f64,mut _5: f64,mut _6: char,mut _7: i8,mut _8: f64,mut _9: usize,mut _10: i8,mut _11: isize,mut _12: f64) -> char {
mir! {
type RET = char;
let _13: &'static bool;
let _14: *mut Adt39;
let _15: (i128, u64);
let _16: i64;
let _17: i8;
let _18: i64;
let _19: (i32,);
let _20: *const u32;
let _21: &'static bool;
let _22: (i128, u64);
let _23: isize;
let _24: *mut *mut (&'static bool, Adt17, &'static &'static bool, &'static &'static i16);
let _25: u128;
let _26: isize;
let _27: &'static &'static *const u32;
let _28: i128;
let _29: ();
let _30: ();
{
_5 = 206_u8 as f64;
_9 = 6299_i16 as usize;
_6 = _3;
RET = _3;
_6 = _3;
_5 = _4 + _4;
_5 = _4 * _4;
RET = _2;
_1 = -_10;
_6 = _2;
_7 = 1934158583_i32 as i8;
_11 = 76_isize;
RET = _2;
_10 = _1 | _7;
RET = _6;
_10 = _1;
_4 = _8;
_1 = _10 | _7;
_11 = 9223372036854775807_isize;
_15.1 = _11 as u64;
_2 = RET;
_15.1 = 2440246371682336038_u64;
_11 = 227_u8 as isize;
_1 = !_10;
_11 = -2_isize;
_6 = _2;
_3 = _2;
_15 = (61575876958761035702402853747226166744_i128, 9996814075100202461_u64);
_9 = 1821019452_u32 as usize;
match _15.1 {
0 => bb1,
1 => bb2,
2 => bb3,
9996814075100202461 => bb5,
_ => bb4
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
_10 = _1 & _7;
_15.1 = 17911890225481429767_u64 << _10;
_2 = _6;
match _15.0 {
0 => bb6,
1 => bb7,
2 => bb8,
61575876958761035702402853747226166744 => bb10,
_ => bb9
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
Return()
}
bb10 = {
_2 = _3;
_3 = _2;
_6 = _2;
_17 = _4 as i8;
_1 = _10 ^ _10;
_3 = RET;
_9 = _5 as usize;
_10 = _17;
_15 = (8997255860346727005310322446992139252_i128, 13397264299981523355_u64);
_17 = _10 >> _9;
_15.0 = !(-81696472952684998435136572988856253197_i128);
_9 = 2893718313847517454_usize;
_16 = 2026063528944538072_i64 * (-5979385537997608765_i64);
_17 = 64414_u16 as i8;
_9 = 4_usize;
_22.1 = _15.1;
RET = _3;
_5 = 45368_u16 as f64;
_15.1 = !_22.1;
_11 = (-9223372036854775808_isize) - 9223372036854775807_isize;
_5 = _4;
_15.0 = !61652107159466901120335664856689693595_i128;
Goto(bb11)
}
bb11 = {
_12 = _8;
_22.1 = _15.1 | _15.1;
RET = _2;
RET = _6;
_19 = ((-1923941641_i32),);
_15.0 = true as i128;
_5 = -_4;
match _9 {
0 => bb8,
1 => bb7,
2 => bb6,
3 => bb4,
5 => bb12,
4 => bb14,
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
_15.1 = _22.1 - _22.1;
_23 = _11 & _11;
_18 = _16;
_8 = 195908256584063755248144500948663036740_u128 as f64;
_7 = _10;
_22 = (_15.0, _15.1);
_19 = (1259019194_i32,);
_19.0 = -1378824916_i32;
_6 = _3;
_12 = _5 * _4;
_8 = _12 - _12;
_3 = _6;
_19 = ((-1241212252_i32),);
_16 = -_18;
_12 = -_4;
_15.1 = _22.1;
_11 = _23 & _23;
_15 = (_22.0, _22.1);
_10 = _17;
_22.1 = _15.1 * _15.1;
_8 = _9 as f64;
_22.0 = -_15.0;
_26 = _11 * _23;
_9 = 2892203039110352113_usize << _26;
_1 = _17;
_2 = _6;
RET = _3;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(13_usize, 3_usize, Move(_3), 7_usize, Move(_7), 9_usize, Move(_9), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(13_usize, 19_usize, Move(_19), 16_usize, Move(_16), 26_usize, Move(_26), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: [usize; 4],mut _2: i64,mut _3: i32,mut _4: bool,mut _5: i64,mut _6: [usize; 4],mut _7: i32,mut _8: [usize; 4],mut _9: bool,mut _10: i64,mut _11: bool,mut _12: [usize; 4]) -> u128 {
mir! {
type RET = u128;
let _13: i128;
let _14: (i8, u64, f32);
let _15: bool;
let _16: isize;
let _17: [usize; 4];
let _18: u128;
let _19: [u32; 6];
let _20: &'static &'static *const u32;
let _21: bool;
let _22: (Adt31, i16);
let _23: (u64, i64, &'static (u32, f64, u8, i8));
let _24: (i8, u64, f32);
let _25: char;
let _26: Adt17;
let _27: *mut [u16; 1];
let _28: f32;
let _29: u64;
let _30: (u64, i64, &'static (u32, f64, u8, i8));
let _31: bool;
let _32: usize;
let _33: isize;
let _34: u128;
let _35: [isize; 7];
let _36: isize;
let _37: [i16; 1];
let _38: [u16; 1];
let _39: ();
let _40: ();
{
_2 = _5 & _5;
_10 = _7 as i64;
_3 = 166_u8 as i32;
_13 = 76665482626543858807548664344147354641_i128 - (-96134746485806029565796818952933977017_i128);
_9 = !_4;
RET = 221059724881805369799167237215467610165_u128 << _2;
_2 = _5;
_13 = !75887186051658961162289352383437111963_i128;
_7 = _9 as i32;
_13 = 161718962356963053405520246586595835464_i128;
_13 = _7 as i128;
_12 = [13071252666430892584_usize,9700838052042438492_usize,5_usize,782528247916902495_usize];
RET = 202620983817793799569356535691951293049_u128 | 30281286074021231192065238715460707036_u128;
_10 = _5;
_12 = _8;
Goto(bb1)
}
bb1 = {
_5 = _10;
_10 = RET as i64;
RET = 1409023201_u32 as u128;
_14.1 = !12157987910565766780_u64;
Goto(bb2)
}
bb2 = {
_13 = 31397482586367106016044222947318558255_i128;
_15 = _9;
_11 = !_15;
_8 = [5582958591118765095_usize,5842268698240231682_usize,5_usize,2_usize];
RET = _5 as u128;
_16 = '\u{d8ac7}' as isize;
_14.2 = 4267643967_u32 as f32;
RET = !321568276762897567593741493208257280714_u128;
_12 = [1348880445981899879_usize,8050940265563899040_usize,1_usize,7_usize];
_3 = 17_i8 as i32;
match _13 {
31397482586367106016044222947318558255 => bb4,
_ => bb3
}
}
bb3 = {
_5 = _10;
_10 = RET as i64;
RET = 1409023201_u32 as u128;
_14.1 = !12157987910565766780_u64;
Goto(bb2)
}
bb4 = {
_11 = !_4;
_14.0 = !38_i8;
_16 = (-9223372036854775808_isize) | 119_isize;
_3 = _7;
_5 = _15 as i64;
RET = !256285497195955376774117156563339239230_u128;
_14.0 = (-93_i8) - 62_i8;
_11 = _4;
match _13 {
0 => bb1,
31397482586367106016044222947318558255 => bb5,
_ => bb2
}
}
bb5 = {
_12 = [3_usize,4_usize,5811115312326350687_usize,7909738911189445099_usize];
_11 = _4;
_15 = _11;
_4 = !_15;
_8 = [5_usize,3677599081431408855_usize,0_usize,1_usize];
_17 = [1_usize,162155365814875157_usize,8383049795825519685_usize,11429719726217828488_usize];
_14.0 = !82_i8;
_9 = _11;
Call(_7 = fn15(_10, _4, _8, _9, _11), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_16 = 9223372036854775807_isize;
_1 = [8909792655162436805_usize,6_usize,766254252812944720_usize,0_usize];
_19 = [547345024_u32,2857369998_u32,3101063761_u32,1672299521_u32,4110123750_u32,2669186718_u32];
_12 = _17;
_7 = _14.2 as i32;
_2 = _5 * _5;
_6 = [16698393801195748434_usize,7_usize,5092470643574016582_usize,2_usize];
_11 = !_9;
_7 = _3 - _3;
Call(_7 = core::intrinsics::transmute(_3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_3 = _7;
_15 = _3 <= _7;
_8 = [17159552061294380084_usize,3_usize,6_usize,0_usize];
_8 = [3_usize,2_usize,5_usize,4_usize];
RET = 180044547231730630262438224170074678581_u128;
RET = 156944784357249594689204255321696194837_u128 | 5764882732310173771268882739640276135_u128;
_18 = !RET;
_14.1 = 9384435181505276296_u64;
RET = _18;
_22.1 = -21595_i16;
_14.2 = _14.0 as f32;
_7 = _14.1 as i32;
_8 = _17;
_14.2 = 141_u8 as f32;
_11 = !_15;
_9 = !_11;
_14.1 = _16 as u64;
match _16 {
9223372036854775807 => bb9,
_ => bb8
}
}
bb8 = {
_11 = !_4;
_14.0 = !38_i8;
_16 = (-9223372036854775808_isize) | 119_isize;
_3 = _7;
_5 = _15 as i64;
RET = !256285497195955376774117156563339239230_u128;
_14.0 = (-93_i8) - 62_i8;
_11 = _4;
match _13 {
0 => bb1,
31397482586367106016044222947318558255 => bb5,
_ => bb2
}
}
bb9 = {
_6 = [18400502730881535104_usize,6_usize,5_usize,7_usize];
_3 = _7;
_16 = 4118_u16 as isize;
_23.1 = _5;
_12 = [11613114508688430391_usize,2_usize,6_usize,16241688943535615696_usize];
_22.1 = -(-2838_i16);
_24.2 = -_14.2;
_14 = (2_i8, 11678058697606454376_u64, _24.2);
_13 = -94722301556700527263206491213443334584_i128;
RET = !_18;
_18 = RET;
Call(_14.0 = core::intrinsics::bswap(65_i8), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_2 = -_5;
_10 = _2 - _2;
_10 = _2;
_3 = _14.0 as i32;
RET = _7 as u128;
_15 = _13 >= _13;
_11 = !_15;
_1 = [15442841184396903737_usize,5_usize,6289604174985315817_usize,2_usize];
_6 = _17;
_19 = [3175512873_u32,596182914_u32,1036152228_u32,504613393_u32,4152878628_u32,2057141972_u32];
_18 = RET * RET;
_6 = _1;
_15 = _9 ^ _9;
_24 = (_14.0, _14.1, _14.2);
_21 = _15;
_1 = [13175994102436193097_usize,5_usize,16810718489730094638_usize,4_usize];
_22.1 = _24.2 as i16;
_3 = _7 << _24.1;
_24.2 = -_14.2;
_28 = _24.2 - _14.2;
RET = _18 & _18;
_14.1 = _24.1 << _24.1;
_11 = !_21;
_23.0 = _24.1 >> RET;
RET = _18 * _18;
_21 = _15 > _15;
_24.1 = _14.1 + _23.0;
_24 = (_14.0, _23.0, _28);
Goto(bb11)
}
bb11 = {
_23.1 = 184098991_u32 as i64;
_10 = _2;
_10 = _5 >> _23.0;
_29 = _14.1 - _14.1;
_8 = [13806186077831982793_usize,6_usize,5056826440065584691_usize,9168864029176889837_usize];
_9 = _21 | _21;
_15 = _14.0 == _24.0;
_5 = _24.2 as i64;
_25 = '\u{ab801}';
_18 = !RET;
_24.0 = _14.0;
_6 = [1_usize,4_usize,6_usize,3_usize];
_13 = !78462619412404530860309508419854361825_i128;
_3 = _25 as i32;
_26 = Adt17::Variant1 { fld0: _10,fld1: 18470_u16,fld2: _16,fld3: _24.0,fld4: 14968563934145059078_usize };
_9 = !_11;
Goto(bb12)
}
bb12 = {
_1 = [5_usize,2416390690140355362_usize,15897440234849788047_usize,5_usize];
_30.1 = -_2;
_24 = (Field::<i8>(Variant(_26, 1), 3), _29, _28);
_17 = _12;
_26 = Adt17::Variant0 { fld0: _22.1,fld1: 70_u8 };
_8 = [3_usize,5_usize,6_usize,4833093628430748776_usize];
_14.1 = _24.1;
_14 = (_24.0, _23.0, _24.2);
_9 = !_11;
_1 = [3392429425601326910_usize,0_usize,13635221624297116509_usize,0_usize];
_24.1 = _2 as u64;
_13 = _9 as i128;
_32 = 4_usize;
_8 = _17;
_6 = _1;
_22.1 = _32 as i16;
_29 = _14.1;
_19 = [1142408411_u32,1153581020_u32,2193960483_u32,2857417399_u32,1849157075_u32,256870216_u32];
_31 = !_21;
_8 = _6;
_24 = _14;
_9 = _4 <= _21;
_7 = !_3;
RET = _18 >> _14.0;
_14 = (_24.0, _24.1, _24.2);
Goto(bb13)
}
bb13 = {
_16 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_2 = _24.1 as i64;
_9 = _31 > _11;
_14.0 = -_24.0;
_8 = [_32,_32,_32,_32];
_13 = 27369713987432965321287075937795104520_i128;
_1 = _17;
_35 = [_16,_16,_16,_16,_16,_16,_16];
_30.0 = _18 as u64;
_15 = _9;
_14.0 = _24.0;
place!(Field::<u8>(Variant(_26, 0), 1)) = _3 as u8;
_29 = _24.1;
_35 = [_16,_16,_16,_16,_16,_16,_16];
_14 = (_24.0, _29, _28);
RET = _18 & _18;
_31 = _15 | _11;
_8 = [_32,_32,_32,_32];
_33 = -_16;
_23.1 = _10;
RET = _24.2 as u128;
_32 = 17878492438808038122_usize ^ 6966473507382229832_usize;
_16 = _33;
_8 = [_32,_32,_32,_32];
_14.0 = _24.0 | _24.0;
_32 = !3186713733955511870_usize;
match _13 {
0 => bb9,
1 => bb6,
2 => bb3,
27369713987432965321287075937795104520 => bb14,
_ => bb7
}
}
bb14 = {
_5 = _2;
_35 = [_16,_33,_33,_33,_33,_16,_16];
_24.0 = _14.0;
_38 = [13704_u16];
_14 = (_24.0, _30.0, _28);
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(14_usize, 11_usize, Move(_11), 18_usize, Move(_18), 25_usize, Move(_25), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(14_usize, 15_usize, Move(_15), 21_usize, Move(_21), 5_usize, Move(_5), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(14_usize, 38_usize, Move(_38), 7_usize, Move(_7), 12_usize, Move(_12), 13_usize, Move(_13)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_39 = dump_var(14_usize, 4_usize, Move(_4), 40_usize, _40, 40_usize, _40, 40_usize, _40), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: i64,mut _2: bool,mut _3: [usize; 4],mut _4: bool,mut _5: bool) -> i32 {
mir! {
type RET = i32;
let _6: isize;
let _7: [isize; 7];
let _8: *mut u128;
let _9: bool;
let _10: Adt39;
let _11: [u16; 1];
let _12: isize;
let _13: Adt63;
let _14: u128;
let _15: &'static *const u32;
let _16: u64;
let _17: isize;
let _18: [char; 1];
let _19: bool;
let _20: *mut [u32; 6];
let _21: ();
let _22: ();
{
RET = -(-57636695_i32);
_3 = [6263699209434623237_usize,11234195847700967217_usize,3637450828550894430_usize,2742042961312088360_usize];
RET = (-1621138946_i32) & 87061182_i32;
RET = (-1755619648_i32);
_3 = [2818286121504297457_usize,4488417082858686295_usize,5_usize,7462653716002835473_usize];
_3 = [0_usize,4009361037606905662_usize,3_usize,3_usize];
RET = (-961302906_i32);
_3 = [6_usize,4348975281301080589_usize,7_usize,4_usize];
RET = 1675632081_i32;
_5 = !_2;
_2 = _4 ^ _5;
RET = !1820458757_i32;
_5 = !_2;
_5 = _2 | _4;
_6 = -9223372036854775807_isize;
_3 = [0_usize,7_usize,3_usize,5122602634040446117_usize];
_6 = 209305890618097763836017163406638192673_u128 as isize;
_2 = _4 <= _4;
_7 = [_6,_6,_6,_6,_6,_6,_6];
_2 = _5;
Goto(bb1)
}
bb1 = {
_2 = _4;
RET = -2011854004_i32;
_2 = _5;
_4 = _5;
_9 = _5 ^ _2;
_6 = 26_isize;
_4 = _2;
RET = -(-1416927288_i32);
_7 = [_6,_6,_6,_6,_6,_6,_6];
_1 = 6897687291217568625_i64;
RET = 2319005684_u32 as i32;
RET = (-1639479867_i32) << _1;
RET = -(-1475009657_i32);
_7 = [_6,_6,_6,_6,_6,_6,_6];
_5 = _9 <= _2;
_9 = !_4;
_6 = (-9223372036854775808_isize);
RET = 6991688816553373720_u64 as i32;
_4 = !_2;
_4 = _5;
_5 = !_4;
match _6 {
0 => bb2,
340282366920938463454151235394913435648 => bb4,
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
_7 = [_6,_6,_6,_6,_6,_6,_6];
RET = !(-40355702_i32);
RET = -(-1873795984_i32);
_12 = _1 as isize;
match _1 {
0 => bb3,
1 => bb5,
6897687291217568625 => bb7,
_ => bb6
}
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
_4 = !_5;
_6 = -_12;
_7 = [_6,_12,_12,_12,_12,_12,_12];
RET = 14158_u16 as i32;
_6 = 165_u8 as isize;
_5 = _4 & _4;
_6 = _12;
_6 = _12 - _12;
RET = (-122_i8) as i32;
_4 = _2 & _5;
_8 = core::ptr::addr_of_mut!(_14);
_6 = '\u{b7b45}' as isize;
_2 = _4;
_17 = (-85_i8) as isize;
match _1 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
6897687291217568625 => bb16,
_ => bb15
}
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_7 = [_6,_6,_6,_6,_6,_6,_6];
RET = !(-40355702_i32);
RET = -(-1873795984_i32);
_12 = _1 as isize;
match _1 {
0 => bb3,
1 => bb5,
6897687291217568625 => bb7,
_ => bb6
}
}
bb11 = {
Return()
}
bb12 = {
Return()
}
bb13 = {
_2 = _4;
RET = -2011854004_i32;
_2 = _5;
_4 = _5;
_9 = _5 ^ _2;
_6 = 26_isize;
_4 = _2;
RET = -(-1416927288_i32);
_7 = [_6,_6,_6,_6,_6,_6,_6];
_1 = 6897687291217568625_i64;
RET = 2319005684_u32 as i32;
RET = (-1639479867_i32) << _1;
RET = -(-1475009657_i32);
_7 = [_6,_6,_6,_6,_6,_6,_6];
_5 = _9 <= _2;
_9 = !_4;
_6 = (-9223372036854775808_isize);
RET = 6991688816553373720_u64 as i32;
_4 = !_2;
_4 = _5;
_5 = !_4;
match _6 {
0 => bb2,
340282366920938463454151235394913435648 => bb4,
_ => bb3
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_14 = 163297084563882360920857771164409605216_u128;
_16 = 14475166289460186735_u64;
_18 = ['\u{49fc7}'];
_6 = 220_u8 as isize;
_11 = [48617_u16];
_1 = -(-7360435745653057096_i64);
_16 = !10624270910485814967_u64;
_1 = 31404_u16 as i64;
_18 = ['\u{73ead}'];
Goto(bb17)
}
bb17 = {
Call(_21 = dump_var(15_usize, 2_usize, Move(_2), 5_usize, Move(_5), 1_usize, Move(_1), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_21 = dump_var(15_usize, 14_usize, Move(_14), 11_usize, Move(_11), 16_usize, Move(_16), 22_usize, _22), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(1693750985368500602_i64));
                
            }
impl PrintFDebug for Adt17{
	unsafe fn printf_debug(&self){unsafe{printf("Adt17::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt17 {
Variant0{
fld0: i16,
fld1: u8,

},
Variant1{
fld0: i64,
fld1: u16,
fld2: isize,
fld3: i8,
fld4: usize,

}}
impl PrintFDebug for Adt31{
	unsafe fn printf_debug(&self){unsafe{printf("Adt31::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt31 {
Variant0{
fld0: u32,
fld1: f64,
fld2: u64,
fld3: *mut i16,
fld4: (u32, f64, u8, i8),
fld5: (i8, u64, f32),
fld6: i64,

},
Variant1{
fld0: *mut f32,

},
Variant2{
fld0: u32,
fld1: *const i16,
fld2: isize,
fld3: i128,
fld4: [u32; 6],

},
Variant3{
fld0: f64,
fld1: Adt17,

}}
impl PrintFDebug for Adt34{
	unsafe fn printf_debug(&self){unsafe{printf("Adt34::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt34 {
Variant0{
fld0: u16,
fld1: char,
fld2: *mut f32,
fld3: f64,
fld4: i16,
fld5: i32,
fld6: (u32, f64, u8, i8),
fld7: (i8, u64, f32),

},
Variant1{
fld0: bool,
fld1: u16,
fld2: [u32; 6],
fld3: *const i16,

}}
impl PrintFDebug for Adt36{
	unsafe fn printf_debug(&self){unsafe{printf("Adt36::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt36 {
Variant0{
fld0: Adt34,
fld1: [u32; 6],
fld2: isize,
fld3: *mut i16,
fld4: u128,
fld5: Adt17,
fld6: *mut f32,

},
Variant1{
fld0: usize,
fld1: [isize; 2],
fld2: f64,
fld3: i8,
fld4: *mut i16,
fld5: (u32, f64, u8, i8),

},
Variant2{
fld0: u64,
fld1: i128,

},
Variant3{
fld0: Adt31,
fld1: i16,
fld2: (i8, u64, f32),
fld3: Adt34,

}}
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf("Adt39::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
	Self::Variant3{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: bool,
fld1: Adt36,
fld2: *const i16,
fld3: i128,
fld4: u8,
fld5: u16,
fld6: Adt17,

},
Variant1{
fld0: [u32; 6],
fld1: i64,
fld2: f64,

},
Variant2{
fld0: Adt36,
fld1: u8,
fld2: f64,
fld3: i64,
fld4: i16,
fld5: [isize; 2],

},
Variant3{
fld0: u8,
fld1: i32,
fld2: *mut i16,

}}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt40{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt40 {
fld0: *mut i16,
fld1: [isize; 2],
fld2: [u32; 6],
fld3: u16,
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: (u32, f64, u8, i8),
fld1: char,
fld2: usize,
fld3: (i8, u64, f32),
fld4: i16,
fld5: [isize; 7],
fld6: *mut f32,
fld7: Adt31,

},
Variant1{
fld0: Adt39,
fld1: *mut i16,
fld2: [i16; 1],

}}
impl PrintFDebug for Adt63{
	unsafe fn printf_debug(&self){unsafe{printf("Adt63::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
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
#[derive(Copy,Clone)]pub enum Adt63 {
Variant0{
fld0: (u32, f64, u8, i8),
fld1: char,
fld2: Adt40,
fld3: (Adt31, i16),
fld4: (*const u32, u32),
fld5: u64,

},
Variant1{
fld0: usize,

},
Variant2{
fld0: usize,
fld1: [i16; 1],

}}

