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
pub fn fn0(mut _1: bool,mut _2: u16,mut _3: u128,mut _4: u32,mut _5: u64,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8) -> isize {
mir! {
type RET = isize;
let _11: i8;
let _12: i8;
let _13: bool;
let _14: isize;
let _15: i64;
let _16: [char; 8];
let _17: f32;
let _18: isize;
let _19: char;
let _20: *mut i16;
let _21: i8;
let _22: (&'static i64, (u64, i16));
let _23: *const i8;
let _24: ();
let _25: ();
{
_9 = 3_usize * 7_usize;
_7 = '\u{d441}' as i64;
_4 = _9 as u32;
RET = -9223372036854775807_isize;
RET = (-1864046718_i32) as isize;
_10 = _9 as u8;
_8 = !(-152523049787942712012563840296733242066_i128);
Goto(bb1)
}
bb1 = {
_2 = !17213_u16;
_1 = false;
_6 = (-1261309592_i32);
_1 = true;
_3 = !111442023311878725432734183875316710159_u128;
_5 = _3 as u64;
_5 = 3647387972869999161_u64 - 7714422305427609590_u64;
_4 = RET as u32;
_1 = _10 <= _10;
_8 = (-73677477487007551309345687531426226281_i128);
_9 = 3346_i16 as usize;
_4 = 3237846021_u32 ^ 3945502984_u32;
_10 = 125_u8;
_8 = !158993375975226075164029384739409573662_i128;
_9 = !5_usize;
_6 = (-855604608_i32) >> _9;
match _10 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
125 => bb9,
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
_11 = _6 as i8;
_7 = (-3979358722013228316_i64);
_5 = 11124545833233807357_u64;
_7 = _9 as i64;
_6 = 1487649467_i32;
RET = (-112_isize);
_3 = _11 as u128;
RET = 9223372036854775807_isize - 107_isize;
_7 = (-7227515308889638024_i64);
Call(_10 = fn1(_5, RET, _7, _5, _1, _9), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
RET = _9 as isize;
_9 = _1 as usize;
_5 = !903530499292982045_u64;
_4 = 3159640162_u32 >> _9;
_2 = _5 as u16;
_10 = 59_u8;
Call(_6 = core::intrinsics::bswap(1819926834_i32), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_14 = RET;
_13 = !_1;
RET = -_14;
_1 = _13;
RET = _14 | _14;
_10 = 247_u8 | 62_u8;
_5 = 3549630891095337594_u64;
_8 = (-25122773968907886534226067075466662551_i128) * (-37666766518257227575186679941755937213_i128);
_1 = _13;
_16 = ['\u{4ea3f}','\u{c609a}','\u{8bd}','\u{109686}','\u{846e6}','\u{585da}','\u{85fa9}','\u{27896}'];
RET = _14 * _14;
_10 = !191_u8;
_3 = 111634541553711347740730240705565341518_u128;
_9 = 28382_i16 as usize;
_2 = _7 as u16;
_7 = (-2949701664346693132_i64) >> RET;
_2 = 63362_u16 - 11879_u16;
_3 = 164439563065053816619052932553740130332_u128 ^ 272501309037049153618201445040752223837_u128;
_16 = ['\u{e8bfc}','\u{6956d}','\u{441e1}','\u{f8039}','\u{3f581}','\u{c4a49}','\u{5f9f2}','\u{5e7d4}'];
_3 = 148029217119595128078334696154832549489_u128 * 186532796160843009878894387476897694846_u128;
match _5 {
0 => bb1,
1 => bb4,
2 => bb12,
3 => bb13,
4 => bb14,
3549630891095337594 => bb16,
_ => bb15
}
}
bb12 = {
Return()
}
bb13 = {
_11 = _6 as i8;
_7 = (-3979358722013228316_i64);
_5 = 11124545833233807357_u64;
_7 = _9 as i64;
_6 = 1487649467_i32;
RET = (-112_isize);
_3 = _11 as u128;
RET = 9223372036854775807_isize - 107_isize;
_7 = (-7227515308889638024_i64);
Call(_10 = fn1(_5, RET, _7, _5, _1, _9), ReturnTo(bb10), UnwindUnreachable())
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_19 = '\u{76a1a}';
_3 = _14 as u128;
_13 = !_1;
_3 = _19 as u128;
_15 = _7 - _7;
_2 = _19 as u16;
_10 = _13 as u8;
_5 = _19 as u64;
_23 = core::ptr::addr_of!(_11);
_13 = !_1;
Goto(bb17)
}
bb17 = {
Call(_24 = dump_var(0_usize, 1_usize, Move(_1), 2_usize, Move(_2), 13_usize, Move(_13), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_24 = dump_var(0_usize, 7_usize, Move(_7), 8_usize, Move(_8), 6_usize, Move(_6), 14_usize, Move(_14)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: u64,mut _2: isize,mut _3: i64,mut _4: u64,mut _5: bool,mut _6: usize) -> u8 {
mir! {
type RET = u8;
let _7: &'static i128;
let _8: &'static i32;
let _9: i32;
let _10: isize;
let _11: [u16; 4];
let _12: [i32; 3];
let _13: (u64, i16);
let _14: (((u128,), [usize; 5]),);
let _15: u64;
let _16: f32;
let _17: (char, [usize; 7]);
let _18: f64;
let _19: [u8; 6];
let _20: i64;
let _21: i128;
let _22: isize;
let _23: f32;
let _24: (((u128,), [usize; 5]),);
let _25: *const i8;
let _26: i128;
let _27: ();
let _28: ();
{
_1 = _4 << _2;
RET = !109_u8;
_2 = 9223372036854775807_isize << _1;
_6 = 7167853286058168713_usize - 14002118562295181715_usize;
_1 = _4 + _4;
_3 = -1895837483004265791_i64;
_9 = (-1703419728_i32);
_9 = '\u{ec768}' as i32;
_4 = (-14624_i16) as u64;
_9 = _5 as i32;
_9 = -2017858720_i32;
_11 = [7474_u16,54173_u16,16729_u16,46054_u16];
_10 = _5 as isize;
RET = 132_u8 + 60_u8;
RET = 82_u8;
RET = !188_u8;
_11 = [43536_u16,27922_u16,21172_u16,27239_u16];
Goto(bb1)
}
bb1 = {
_4 = _1 & _1;
_3 = (-7470073157681856744_i64) - 3583772919731070375_i64;
_2 = RET as isize;
_5 = _6 <= _6;
_12 = [_9,_9,_9];
_1 = !_4;
RET = 167_u8;
_4 = _1;
_8 = &_9;
_9 = 90084334_i32 - (-499014700_i32);
_3 = _4 as i64;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
167 => bb8,
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
_2 = (-163876551792753288785450023577879123328_i128) as isize;
_13.1 = !18978_i16;
_12 = [_9,_9,_9];
_13 = (_1, 20945_i16);
_3 = _6 as i64;
_11 = [37707_u16,20358_u16,61069_u16,37925_u16];
_9 = 274052930_i32 + 1858226404_i32;
_3 = 8563539060452299901_i64;
_8 = &_9;
_9 = _13.0 as i32;
_11 = [35615_u16,35873_u16,34711_u16,33465_u16];
_13 = (_4, (-27383_i16));
match _13.1 {
0 => bb4,
1 => bb9,
2 => bb10,
340282366920938463463374607431768184073 => bb12,
_ => bb11
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
_4 = _1;
_11 = [34354_u16,64907_u16,65348_u16,12522_u16];
_14.0.1 = [_6,_6,_6,_6,_6];
_6 = !6044314124386727889_usize;
_13.1 = !(-2396_i16);
_14.0.0 = (50596600730488840643952034549760902347_u128,);
_12 = [_9,_9,_9];
_14.0.0 = (24894417815833431840261456332084685654_u128,);
_3 = 6843841702047966184_i64 & 8701278426351229750_i64;
_15 = !_13.0;
_8 = &_9;
_11 = [40588_u16,43936_u16,17024_u16,41957_u16];
RET = (-124_i8) as u8;
_4 = !_13.0;
_15 = 2190803220_u32 as u64;
_1 = !_4;
_2 = _10 | _10;
_3 = (-2464514041914887913_i64);
RET = !251_u8;
_13 = (_4, 16925_i16);
_2 = _10;
_12 = [(*_8),(*_8),(*_8)];
_17.1 = [_6,_6,_6,_6,_6,_6,_6];
_18 = _6 as f64;
_5 = true & true;
_13.1 = 592823994_u32 as i16;
Goto(bb13)
}
bb13 = {
_15 = _13.0;
_17.0 = '\u{ec164}';
_14.0.0.0 = 126379809203233780336123912043327981693_u128 ^ 246296713616150443538399770414758774116_u128;
_13 = (_1, 13496_i16);
_20 = _18 as i64;
_14.0.0.0 = 283461685888600886657391606226092544505_u128 & 80140962707576258817818184870521685022_u128;
_19 = [RET,RET,RET,RET,RET,RET];
_9 = !(-104249946_i32);
_23 = 61803_u16 as f32;
_7 = &_21;
_20 = _3;
_2 = _10;
_23 = _4 as f32;
_23 = _18 as f32;
_2 = _6 as isize;
_14.0.1 = [_6,_6,_6,_6,_6];
_5 = !true;
_16 = _23;
Call(_25 = fn2(_10, _14, _15, _9, _1, _4, _3, _20, _17.0, _13.1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_17.1 = [_6,_6,_6,_6,_6,_6,_6];
_13.0 = _17.0 as u64;
_14.0.1 = [_6,_6,_6,_6,_6];
_26 = -6349646715248908754451490912919212165_i128;
RET = !39_u8;
_8 = &_9;
_15 = _1 * _1;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(1_usize, 3_usize, Move(_3), 26_usize, Move(_26), 1_usize, Move(_1), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(1_usize, 2_usize, Move(_2), 13_usize, Move(_13), 10_usize, Move(_10), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: (((u128,), [usize; 5]),),mut _3: u64,mut _4: i32,mut _5: u64,mut _6: u64,mut _7: i64,mut _8: i64,mut _9: char,mut _10: i16) -> *const i8 {
mir! {
type RET = *const i8;
let _11: bool;
let _12: char;
let _13: [u16; 4];
let _14: *const [char; 8];
let _15: char;
let _16: isize;
let _17: &'static isize;
let _18: [i16; 4];
let _19: [i64; 1];
let _20: *const *mut u32;
let _21: f64;
let _22: (u128,);
let _23: [i32; 3];
let _24: char;
let _25: (((u128,), [usize; 5]),);
let _26: [i32; 1];
let _27: (u64, i16);
let _28: isize;
let _29: *mut f64;
let _30: (&'static i64, (u64, i16));
let _31: &'static [i16; 4];
let _32: ([i64; 8], *mut *const [char; 8], [u32; 6], [u16; 4]);
let _33: &'static *mut &'static u128;
let _34: f32;
let _35: ((isize,), &'static *mut u8, (&'static i64, (u64, i16)), (&'static i64, (u64, i16)));
let _36: i8;
let _37: char;
let _38: char;
let _39: u16;
let _40: i64;
let _41: [u32; 3];
let _42: *mut &'static i32;
let _43: [i8; 5];
let _44: i64;
let _45: isize;
let _46: [i64; 5];
let _47: &'static [u8; 6];
let _48: usize;
let _49: [u8; 6];
let _50: [i64; 1];
let _51: isize;
let _52: &'static *mut u8;
let _53: isize;
let _54: ();
let _55: ();
{
_9 = '\u{7c3b1}';
_2.0.0.0 = 108817100975688149684867046174726075211_u128;
_7 = true as i64;
_7 = _8;
_6 = 7868306031656898522_usize as u64;
_10 = 4459_i16 | (-15216_i16);
_11 = !false;
_9 = '\u{a8e06}';
_6 = !_3;
_4 = 691270428_i32 | 278509473_i32;
_9 = '\u{2387c}';
_1 = (-126_isize) | (-24_isize);
_9 = '\u{c220a}';
_10 = 11170_i16;
_8 = _9 as i64;
_4 = !(-1423258241_i32);
_3 = !_6;
_12 = _9;
_12 = _9;
_1 = 9223372036854775807_isize * (-53_isize);
_5 = _3;
_2.0.1 = [11168668124481555439_usize,4824328482510418929_usize,5_usize,3_usize,4_usize];
_8 = _7;
_3 = !_6;
_6 = _5;
_1 = 9223372036854775807_isize;
_12 = _9;
Call(_2 = fn3(_6, _5, _11), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2.0.0.0 = 96150677336512520489537093979862587915_u128 << _6;
_4 = !1229048757_i32;
_1 = 9223372036854775807_isize;
_1 = -(-9223372036854775808_isize);
_10 = 693767502_u32 as i16;
_13 = [36852_u16,8550_u16,317_u16,39517_u16];
_2.0.0.0 = 152495150607507903669172591855963274091_u128;
_3 = _5 * _5;
_16 = _1;
_12 = _9;
_1 = _16;
_3 = _2.0.0.0 as u64;
_15 = _9;
_17 = &_1;
_11 = false;
Goto(bb2)
}
bb2 = {
_5 = _11 as u64;
_5 = (*_17) as u64;
_12 = _15;
_2.0.1 = [4_usize,16025253548586395940_usize,4_usize,3_usize,3463955917099488749_usize];
_2.0.1 = [3544526530134723898_usize,2272444378485593431_usize,10482228012917945799_usize,1_usize,5_usize];
_9 = _12;
_19 = [_8];
_15 = _12;
_15 = _12;
_3 = 20814_u16 as u64;
_6 = _5;
_6 = 2683423437_u32 as u64;
_17 = &_16;
_16 = _15 as isize;
_15 = _12;
_18 = [_10,_10,_10,_10];
_9 = _12;
Goto(bb3)
}
bb3 = {
_3 = 234_u8 as u64;
_2.0.1 = [14804119771981388582_usize,18382292546366883587_usize,2_usize,3138538085938008569_usize,3_usize];
_12 = _15;
_3 = 6_u8 as u64;
_6 = _3 | _5;
_18 = [_10,_10,_10,_10];
_4 = _11 as i32;
_21 = 1298345630_u32 as f64;
_22 = (_2.0.0.0,);
_23 = [_4,_4,_4];
_21 = 994954445051102853_usize as f64;
_2.0.0.0 = !_22.0;
_13 = [57705_u16,52993_u16,30777_u16,14296_u16];
_6 = !_5;
_5 = !_3;
_2.0.0.0 = _22.0;
_24 = _15;
_21 = (-16_i8) as f64;
_2.0.0.0 = _22.0 + _22.0;
_15 = _24;
_3 = _5;
_1 = _7 as isize;
_9 = _12;
_10 = (-28660_i16);
_25.0.1 = [5_usize,7913121198716940313_usize,0_usize,3_usize,5_usize];
match _22.0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
152495150607507903669172591855963274091 => bb9,
_ => bb8
}
}
bb4 = {
_5 = _11 as u64;
_5 = (*_17) as u64;
_12 = _15;
_2.0.1 = [4_usize,16025253548586395940_usize,4_usize,3_usize,3463955917099488749_usize];
_2.0.1 = [3544526530134723898_usize,2272444378485593431_usize,10482228012917945799_usize,1_usize,5_usize];
_9 = _12;
_19 = [_8];
_15 = _12;
_15 = _12;
_3 = 20814_u16 as u64;
_6 = _5;
_6 = 2683423437_u32 as u64;
_17 = &_16;
_16 = _15 as isize;
_15 = _12;
_18 = [_10,_10,_10,_10];
_9 = _12;
Goto(bb3)
}
bb5 = {
_2.0.0.0 = 96150677336512520489537093979862587915_u128 << _6;
_4 = !1229048757_i32;
_1 = 9223372036854775807_isize;
_1 = -(-9223372036854775808_isize);
_10 = 693767502_u32 as i16;
_13 = [36852_u16,8550_u16,317_u16,39517_u16];
_2.0.0.0 = 152495150607507903669172591855963274091_u128;
_3 = _5 * _5;
_16 = _1;
_12 = _9;
_1 = _16;
_3 = _2.0.0.0 as u64;
_15 = _9;
_17 = &_1;
_11 = false;
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
_7 = _8;
_22.0 = !_2.0.0.0;
_1 = _16;
_26 = [_4];
_21 = 2104582569_u32 as f64;
_18 = [_10,_10,_10,_10];
_29 = core::ptr::addr_of_mut!(_21);
match _8 {
0 => bb8,
1 => bb5,
2 => bb10,
340282366920938463460910093389853323543 => bb12,
_ => bb11
}
}
bb10 = {
_2.0.0.0 = 96150677336512520489537093979862587915_u128 << _6;
_4 = !1229048757_i32;
_1 = 9223372036854775807_isize;
_1 = -(-9223372036854775808_isize);
_10 = 693767502_u32 as i16;
_13 = [36852_u16,8550_u16,317_u16,39517_u16];
_2.0.0.0 = 152495150607507903669172591855963274091_u128;
_3 = _5 * _5;
_16 = _1;
_12 = _9;
_1 = _16;
_3 = _2.0.0.0 as u64;
_15 = _9;
_17 = &_1;
_11 = false;
Goto(bb2)
}
bb11 = {
Return()
}
bb12 = {
_25.0.1 = _2.0.1;
_24 = _12;
_25 = (_2.0,);
_25 = (_2.0,);
_25.0.0 = (_2.0.0.0,);
_1 = !_16;
_4 = (-830553702_i32);
_4 = 599923579_i32;
_27.0 = _3;
Goto(bb13)
}
bb13 = {
_1 = -_16;
_25 = (_2.0,);
_27 = (_6, _10);
_31 = &_18;
_29 = core::ptr::addr_of_mut!(_21);
_9 = _15;
_32.2 = [3708793352_u32,3588321832_u32,1533410772_u32,745689553_u32,873363782_u32,3133629648_u32];
_25 = (_2.0,);
_16 = _1;
_31 = &(*_31);
_10 = _22.0 as i16;
_2.0.1 = _25.0.1;
_3 = _27.0;
_17 = &_35.0.0;
_35.2.0 = &_7;
_2.0.1 = _25.0.1;
_35.3.1.1 = _10;
_34 = _22.0 as f32;
_35.2.1.0 = _3 ^ _5;
_5 = _34 as u64;
_35.2.1 = _27;
_35.2.1.0 = _1 as u64;
_2 = _25;
Goto(bb14)
}
bb14 = {
_12 = _24;
_35.0.0 = (*_29) as isize;
_35.3.1.1 = !_35.2.1.1;
_25.0.1 = _2.0.1;
_2.0.1 = _25.0.1;
_35.3.0 = &_7;
_2.0.0 = (_25.0.0.0,);
_8 = 35049_u16 as i64;
_35.3.1.1 = -_35.2.1.1;
_35.2.1.1 = -_10;
_32.1 = core::ptr::addr_of_mut!(_14);
_35.2 = (Move(_35.3.0), _27);
_30.1 = (_35.2.1.0, _35.2.1.1);
_32.0 = [_7,_7,_8,_7,_8,_8,_7,_7];
_32.3 = [52377_u16,7640_u16,61263_u16,10390_u16];
_41 = [3577473720_u32,1726419644_u32,3628302797_u32];
_35.3.0 = &_40;
_36 = -(-17_i8);
_37 = _9;
Goto(bb15)
}
bb15 = {
_8 = _7;
_2.0.0 = _25.0.0;
_4 = 156859243_i32;
_5 = _3 * _27.0;
_22 = _25.0.0;
_37 = _9;
_38 = _37;
_29 = core::ptr::addr_of_mut!(_21);
_40 = _7 << _25.0.0.0;
_35.0.0 = _40 as isize;
_35.2.1 = (_6, _10);
_35.3.1.0 = _6;
_35.3.0 = &_8;
_30.0 = &_8;
_26 = [_4];
match _8 {
0 => bb16,
1 => bb17,
2 => bb18,
3 => bb19,
340282366920938463460910093389853323543 => bb21,
_ => bb20
}
}
bb16 = {
_12 = _24;
_35.0.0 = (*_29) as isize;
_35.3.1.1 = !_35.2.1.1;
_25.0.1 = _2.0.1;
_2.0.1 = _25.0.1;
_35.3.0 = &_7;
_2.0.0 = (_25.0.0.0,);
_8 = 35049_u16 as i64;
_35.3.1.1 = -_35.2.1.1;
_35.2.1.1 = -_10;
_32.1 = core::ptr::addr_of_mut!(_14);
_35.2 = (Move(_35.3.0), _27);
_30.1 = (_35.2.1.0, _35.2.1.1);
_32.0 = [_7,_7,_8,_7,_8,_8,_7,_7];
_32.3 = [52377_u16,7640_u16,61263_u16,10390_u16];
_41 = [3577473720_u32,1726419644_u32,3628302797_u32];
_35.3.0 = &_40;
_36 = -(-17_i8);
_37 = _9;
Goto(bb15)
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
_2.0.0.0 = 96150677336512520489537093979862587915_u128 << _6;
_4 = !1229048757_i32;
_1 = 9223372036854775807_isize;
_1 = -(-9223372036854775808_isize);
_10 = 693767502_u32 as i16;
_13 = [36852_u16,8550_u16,317_u16,39517_u16];
_2.0.0.0 = 152495150607507903669172591855963274091_u128;
_3 = _5 * _5;
_16 = _1;
_12 = _9;
_1 = _16;
_3 = _2.0.0.0 as u64;
_15 = _9;
_17 = &_1;
_11 = false;
Goto(bb2)
}
bb21 = {
_4 = (-1653475282_i32);
_2 = _25;
_2.0.0.0 = _4 as u128;
_40 = !_7;
(*_29) = _10 as f64;
_23 = [_4,_4,_4];
_25 = (_2.0,);
_36 = 6_usize as i8;
_32.1 = core::ptr::addr_of_mut!(_14);
_23 = [_4,_4,_4];
_2 = (_25.0,);
_25 = (_2.0,);
_32.0 = [_40,_8,_8,_7,_40,_40,_40,_40];
_35.3 = (Move(_30.0), _30.1);
Goto(bb22)
}
bb22 = {
_10 = _35.2.1.1 >> _35.3.1.1;
_16 = -_1;
_43 = [_36,_36,_36,_36,_36];
RET = core::ptr::addr_of!(_36);
_35.3.0 = &_44;
_35.2.1 = (_35.3.1.0, _10);
_31 = &(*_31);
_36 = 34_i8 >> _35.2.1.1;
_22 = (_25.0.0.0,);
(*_29) = _5 as f64;
_27.1 = _30.1.1 | _35.2.1.1;
_48 = _35.0.0 as usize;
_30.1.1 = -_27.1;
_17 = &_35.0.0;
_49 = [115_u8,243_u8,221_u8,229_u8,47_u8,129_u8];
_45 = _16 | _1;
_2.0.0 = (_25.0.0.0,);
_28 = _11 as isize;
_27.0 = _30.1.0 * _30.1.0;
_19 = [_7];
_46 = [_40,_7,_40,_8,_8];
_41 = [2483196021_u32,1326081386_u32,2072235099_u32];
Goto(bb23)
}
bb23 = {
Call(_54 = dump_var(2_usize, 16_usize, Move(_16), 15_usize, Move(_15), 13_usize, Move(_13), 7_usize, Move(_7)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_54 = dump_var(2_usize, 4_usize, Move(_4), 41_usize, Move(_41), 36_usize, Move(_36), 8_usize, Move(_8)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_54 = dump_var(2_usize, 22_usize, Move(_22), 26_usize, Move(_26), 25_usize, Move(_25), 45_usize, Move(_45)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_54 = dump_var(2_usize, 38_usize, Move(_38), 10_usize, Move(_10), 2_usize, Move(_2), 23_usize, Move(_23)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Call(_54 = dump_var(2_usize, 24_usize, Move(_24), 55_usize, _55, 55_usize, _55, 55_usize, _55), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: u64,mut _2: u64,mut _3: bool) -> (((u128,), [usize; 5]),) {
mir! {
type RET = (((u128,), [usize; 5]),);
let _4: [u32; 8];
let _5: i128;
let _6: isize;
let _7: Adt64;
let _8: [i32; 3];
let _9: i8;
let _10: (u128, i128);
let _11: &'static isize;
let _12: f32;
let _13: *mut isize;
let _14: *mut i16;
let _15: isize;
let _16: f32;
let _17: (u128, i128);
let _18: Adt31;
let _19: [u32; 3];
let _20: &'static [u8; 6];
let _21: &'static &'static &'static *mut u8;
let _22: (u64, i16);
let _23: [char; 8];
let _24: Adt86;
let _25: [char; 8];
let _26: ();
let _27: ();
{
RET.0.0 = (17399243323518253261987393135029979350_u128,);
RET.0.0.0 = !106817160770336844366103485936197394557_u128;
RET.0.0.0 = 218882437390944458899704552849504986214_u128;
_2 = _1 * _1;
RET.0.1 = [5_usize,4576227189020009604_usize,1_usize,6_usize,10981985592451300961_usize];
_2 = 68_isize as u64;
RET.0.0.0 = !261439964576700784878080131218625731502_u128;
RET.0.0.0 = 322259222038852124205981033428769961841_u128;
RET.0.0.0 = !298366514808779095791108730971337251650_u128;
_4 = [3695145108_u32,2382445512_u32,2108417178_u32,1203480735_u32,1093190629_u32,3891888745_u32,3011433418_u32,4131092159_u32];
Call(RET = fn4(_4, _4, _4, _2, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET.0.0 = (84744792057772740065715282990829432070_u128,);
RET.0.0 = (234622689545620888404399444882119942882_u128,);
_3 = _1 == _1;
RET.0.0 = (190615272780490222384710079496160273813_u128,);
_4 = [1810602520_u32,700275958_u32,1074185094_u32,3156236348_u32,607993115_u32,3126852814_u32,3315004725_u32,1369514632_u32];
RET.0.0.0 = !255444505408297230390762085299433892158_u128;
_5 = 9223372036854775807_isize as i128;
_5 = (-92556157474945530127096519337759099080_i128) | (-38274725833579779225887608811997506039_i128);
RET.0.1 = [3_usize,12865975735171781666_usize,379269754400014022_usize,9018226771283240665_usize,3_usize];
_6 = 16722_i16 as isize;
_4 = [4086960327_u32,332047267_u32,1276465225_u32,3326599546_u32,3612892613_u32,1898701793_u32,1558662762_u32,653904129_u32];
RET.0.0 = (187238044351744996317413542617743511571_u128,);
RET.0.1 = [1_usize,3722396723757445717_usize,10103260952317234304_usize,7_usize,4818492542314680181_usize];
RET.0.0.0 = !336968406714776473692916446776552711573_u128;
_6 = !9223372036854775807_isize;
_2 = _1;
RET.0.1 = [13923749466411379649_usize,3122126524594832325_usize,0_usize,2_usize,6306602606853794988_usize];
RET.0.0 = (206686045085959305472999443344659279786_u128,);
RET.0.0.0 = !46631063227595483010530989599351328904_u128;
Goto(bb2)
}
bb2 = {
RET.0.0 = (315205178707619715575339185851616098821_u128,);
Goto(bb3)
}
bb3 = {
RET.0.1 = [5_usize,7_usize,3_usize,3460882545642016600_usize,6_usize];
_8 = [(-2112969179_i32),822500470_i32,1864358160_i32];
RET.0.1 = [7_usize,0_usize,17660744636525668853_usize,4_usize,3_usize];
RET.0.0.0 = 290718621659755613570301467398615213891_u128;
_10.0 = RET.0.0.0 - RET.0.0.0;
_8 = [(-330643202_i32),2082037455_i32,1028049752_i32];
_6 = 9223372036854775807_isize | 9223372036854775807_isize;
_1 = !_2;
_11 = &_6;
_4 = [2221965527_u32,1763596614_u32,2788338077_u32,3174001838_u32,400766793_u32,3134903331_u32,2087765362_u32,1305480524_u32];
_4 = [2366917732_u32,2646212894_u32,445686235_u32,2356780329_u32,985806142_u32,3844360267_u32,3997508058_u32,1176555596_u32];
_12 = 17369_i16 as f32;
_9 = (-5_i8) - 122_i8;
_10.0 = 42418_u16 as u128;
_6 = 9223372036854775807_isize;
_3 = !true;
_13 = core::ptr::addr_of_mut!(_6);
_2 = _10.0 as u64;
_4 = [4213092234_u32,2271149449_u32,3150696049_u32,223156218_u32,1412837474_u32,2219119833_u32,1430407155_u32,328424721_u32];
_6 = (-43_isize);
_12 = 20314_i16 as f32;
RET.0.0 = (_10.0,);
_5 = 2322_u16 as i128;
match _6 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211413 => bb7,
_ => bb6
}
}
bb4 = {
RET.0.0 = (315205178707619715575339185851616098821_u128,);
Goto(bb3)
}
bb5 = {
RET.0.0 = (84744792057772740065715282990829432070_u128,);
RET.0.0 = (234622689545620888404399444882119942882_u128,);
_3 = _1 == _1;
RET.0.0 = (190615272780490222384710079496160273813_u128,);
_4 = [1810602520_u32,700275958_u32,1074185094_u32,3156236348_u32,607993115_u32,3126852814_u32,3315004725_u32,1369514632_u32];
RET.0.0.0 = !255444505408297230390762085299433892158_u128;
_5 = 9223372036854775807_isize as i128;
_5 = (-92556157474945530127096519337759099080_i128) | (-38274725833579779225887608811997506039_i128);
RET.0.1 = [3_usize,12865975735171781666_usize,379269754400014022_usize,9018226771283240665_usize,3_usize];
_6 = 16722_i16 as isize;
_4 = [4086960327_u32,332047267_u32,1276465225_u32,3326599546_u32,3612892613_u32,1898701793_u32,1558662762_u32,653904129_u32];
RET.0.0 = (187238044351744996317413542617743511571_u128,);
RET.0.1 = [1_usize,3722396723757445717_usize,10103260952317234304_usize,7_usize,4818492542314680181_usize];
RET.0.0.0 = !336968406714776473692916446776552711573_u128;
_6 = !9223372036854775807_isize;
_2 = _1;
RET.0.1 = [13923749466411379649_usize,3122126524594832325_usize,0_usize,2_usize,6306602606853794988_usize];
RET.0.0 = (206686045085959305472999443344659279786_u128,);
RET.0.0.0 = !46631063227595483010530989599351328904_u128;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_10.1 = _5;
Call(_2 = core::intrinsics::bswap(_1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_6 = 12_isize - (-9223372036854775808_isize);
_10.0 = RET.0.0.0;
_6 = (-9223372036854775808_isize);
RET.0.1 = [15315451343480819954_usize,3_usize,0_usize,7_usize,17105588909566839958_usize];
_11 = &(*_13);
_11 = &(*_13);
_15 = (*_13);
_6 = -_15;
_11 = &_15;
_11 = &(*_11);
_16 = -_12;
match (*_11) {
0 => bb6,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb9,
5 => bb10,
6 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb9 = {
_10.1 = _5;
Call(_2 = core::intrinsics::bswap(_1), ReturnTo(bb8), UnwindUnreachable())
}
bb10 = {
RET.0.0 = (315205178707619715575339185851616098821_u128,);
Goto(bb3)
}
bb11 = {
RET.0.0 = (84744792057772740065715282990829432070_u128,);
RET.0.0 = (234622689545620888404399444882119942882_u128,);
_3 = _1 == _1;
RET.0.0 = (190615272780490222384710079496160273813_u128,);
_4 = [1810602520_u32,700275958_u32,1074185094_u32,3156236348_u32,607993115_u32,3126852814_u32,3315004725_u32,1369514632_u32];
RET.0.0.0 = !255444505408297230390762085299433892158_u128;
_5 = 9223372036854775807_isize as i128;
_5 = (-92556157474945530127096519337759099080_i128) | (-38274725833579779225887608811997506039_i128);
RET.0.1 = [3_usize,12865975735171781666_usize,379269754400014022_usize,9018226771283240665_usize,3_usize];
_6 = 16722_i16 as isize;
_4 = [4086960327_u32,332047267_u32,1276465225_u32,3326599546_u32,3612892613_u32,1898701793_u32,1558662762_u32,653904129_u32];
RET.0.0 = (187238044351744996317413542617743511571_u128,);
RET.0.1 = [1_usize,3722396723757445717_usize,10103260952317234304_usize,7_usize,4818492542314680181_usize];
RET.0.0.0 = !336968406714776473692916446776552711573_u128;
_6 = !9223372036854775807_isize;
_2 = _1;
RET.0.1 = [13923749466411379649_usize,3122126524594832325_usize,0_usize,2_usize,6306602606853794988_usize];
RET.0.0 = (206686045085959305472999443344659279786_u128,);
RET.0.0.0 = !46631063227595483010530989599351328904_u128;
Goto(bb2)
}
bb12 = {
RET.0.1 = [5_usize,7_usize,3_usize,3460882545642016600_usize,6_usize];
_8 = [(-2112969179_i32),822500470_i32,1864358160_i32];
RET.0.1 = [7_usize,0_usize,17660744636525668853_usize,4_usize,3_usize];
RET.0.0.0 = 290718621659755613570301467398615213891_u128;
_10.0 = RET.0.0.0 - RET.0.0.0;
_8 = [(-330643202_i32),2082037455_i32,1028049752_i32];
_6 = 9223372036854775807_isize | 9223372036854775807_isize;
_1 = !_2;
_11 = &_6;
_4 = [2221965527_u32,1763596614_u32,2788338077_u32,3174001838_u32,400766793_u32,3134903331_u32,2087765362_u32,1305480524_u32];
_4 = [2366917732_u32,2646212894_u32,445686235_u32,2356780329_u32,985806142_u32,3844360267_u32,3997508058_u32,1176555596_u32];
_12 = 17369_i16 as f32;
_9 = (-5_i8) - 122_i8;
_10.0 = 42418_u16 as u128;
_6 = 9223372036854775807_isize;
_3 = !true;
_13 = core::ptr::addr_of_mut!(_6);
_2 = _10.0 as u64;
_4 = [4213092234_u32,2271149449_u32,3150696049_u32,223156218_u32,1412837474_u32,2219119833_u32,1430407155_u32,328424721_u32];
_6 = (-43_isize);
_12 = 20314_i16 as f32;
RET.0.0 = (_10.0,);
_5 = 2322_u16 as i128;
match _6 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211413 => bb7,
_ => bb6
}
}
bb13 = {
_16 = _12;
_6 = !_15;
_17 = _10;
_4 = [3340800698_u32,2738390554_u32,31395368_u32,933946534_u32,3289226966_u32,3928511948_u32,2152574346_u32,1136486062_u32];
RET.0.0 = (_10.0,);
_10.0 = 5_usize as u128;
_4 = [4071863225_u32,2442773007_u32,2616788022_u32,47160249_u32,2450574483_u32,2169203907_u32,3389333999_u32,3025387261_u32];
_17 = (_10.0, _10.1);
_10.0 = _17.0 - _17.0;
_11 = &(*_13);
_11 = &(*_11);
_17 = (RET.0.0.0, _10.1);
(*_13) = -_15;
_1 = _2 << _15;
_2 = !_1;
RET.0.0 = (_10.0,);
_2 = _1;
_4 = [3766138772_u32,3962474501_u32,2796439329_u32,2904587977_u32,1610013422_u32,1453131622_u32,432990131_u32,440126409_u32];
_1 = !_2;
_15 = _6 * _6;
Goto(bb14)
}
bb14 = {
_6 = (-5588_i16) as isize;
_11 = &(*_13);
_18 = Adt31::Variant1 { fld0: 5174161056324785495_i64,fld1: '\u{1294d}',fld2: 185_u8,fld3: _4,fld4: 35226_u16,fld5: (-2076221562_i32) };
_4 = [935393876_u32,1291728782_u32,885320097_u32,2294869893_u32,2601117252_u32,2807998021_u32,3346310802_u32,2539184413_u32];
_10 = _17;
place!(Field::<[u32; 8]>(Variant(_18, 1), 3)) = [214073924_u32,3992552544_u32,2805516135_u32,3141114231_u32,610083071_u32,162172980_u32,2330079236_u32,3431714499_u32];
place!(Field::<i32>(Variant(_18, 1), 5)) = (-629357387_i32);
_12 = _16 + _16;
_19 = [72527893_u32,3105460918_u32,729089786_u32];
_17 = (RET.0.0.0, _5);
_10 = (RET.0.0.0, _17.1);
_10.1 = _12 as i128;
_19 = [921575941_u32,2076273177_u32,689406692_u32];
_11 = &(*_13);
_6 = _15 & _15;
RET.0.0.0 = _17.0 << (*_13);
_2 = RET.0.0.0 as u64;
_22.1 = _10.0 as i16;
place!(Field::<u8>(Variant(_18, 1), 2)) = 138_u8 ^ 213_u8;
_10 = (RET.0.0.0, _5);
_2 = _1 << _17.0;
_10 = (_17.0, _17.1);
_22.0 = !_2;
_22.0 = _1;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(3_usize, 4_usize, Move(_4), 6_usize, Move(_6), 1_usize, Move(_1), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(3_usize, 9_usize, Move(_9), 22_usize, Move(_22), 27_usize, _27, 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: [u32; 8],mut _2: [u32; 8],mut _3: [u32; 8],mut _4: u64,mut _5: [u32; 8]) -> (((u128,), [usize; 5]),) {
mir! {
type RET = (((u128,), [usize; 5]),);
let _6: Adt86;
let _7: isize;
let _8: isize;
let _9: f64;
let _10: &'static i64;
let _11: &'static (isize,);
let _12: bool;
let _13: isize;
let _14: isize;
let _15: i8;
let _16: &'static *mut u8;
let _17: [u8; 7];
let _18: &'static *const [u32; 6];
let _19: f32;
let _20: u16;
let _21: Adt64;
let _22: i32;
let _23: usize;
let _24: (isize,);
let _25: [i32; 1];
let _26: ((u128,), [usize; 5]);
let _27: isize;
let _28: f64;
let _29: usize;
let _30: isize;
let _31: bool;
let _32: &'static [u8; 6];
let _33: usize;
let _34: *mut [i32; 1];
let _35: isize;
let _36: f64;
let _37: *mut u8;
let _38: i16;
let _39: char;
let _40: i128;
let _41: isize;
let _42: char;
let _43: char;
let _44: *mut char;
let _45: *mut *mut isize;
let _46: char;
let _47: [u8; 6];
let _48: *const [char; 8];
let _49: [u32; 8];
let _50: ();
let _51: ();
{
RET.0.1 = [5_usize,14719910777932416720_usize,3_usize,11243426950939942977_usize,13543328492635229606_usize];
_4 = 0_i8 as u64;
RET.0.0.0 = !238373188193680466447506675796138473672_u128;
RET.0.0.0 = '\u{3a9da}' as u128;
_7 = 5205517634771115464_i64 as isize;
_1 = [2264694367_u32,2289589051_u32,3934881167_u32,2678053480_u32,2793289849_u32,1819603619_u32,1745765404_u32,620685010_u32];
_8 = -_7;
RET.0.1 = [5_usize,3705570290991859102_usize,4_usize,0_usize,1_usize];
_8 = _7 + _7;
RET.0.0.0 = !254419987245116829340410069582572265538_u128;
RET.0.0 = (264320847999952310422238262365543831081_u128,);
_5 = _2;
RET.0.0.0 = 1205563918_i32 as u128;
RET.0.0 = (77821481135788035073335451445353458329_u128,);
_3 = _2;
RET.0.1 = [11934672812114428893_usize,9781326366635616354_usize,4_usize,2_usize,10772774343617106706_usize];
_9 = 122_u8 as f64;
_5 = [3723029918_u32,2252448523_u32,4172895265_u32,2477085654_u32,861057666_u32,2415639949_u32,822908602_u32,2858876725_u32];
_9 = 3296775163_u32 as f64;
Goto(bb1)
}
bb1 = {
_2 = [434785022_u32,2474123168_u32,566461052_u32,3447688905_u32,2013538125_u32,1509549778_u32,2854064393_u32,4195181150_u32];
RET.0.1 = [6519403771705462270_usize,11715217998165548919_usize,3093343881840292393_usize,15050082053427003428_usize,8166591881437513590_usize];
Goto(bb2)
}
bb2 = {
RET.0.1 = [2_usize,10253764748472812128_usize,6702048771532993239_usize,7708066863808428137_usize,2_usize];
_8 = _7;
_9 = _7 as f64;
RET.0.0 = (218087202046808561004140437167467676924_u128,);
Goto(bb3)
}
bb3 = {
_8 = _7;
_3 = _5;
RET.0.0 = (277789054901860216160001871395732812633_u128,);
_4 = 10540753107004491022_u64;
RET.0.0.0 = 60245616960570108072263157939136795958_u128 + 252811657801191314476498378931514406283_u128;
_9 = 124552946292873246822284956071393043857_i128 as f64;
RET.0.0 = (237028991610447665591935701943952891495_u128,);
_3 = [1664114125_u32,346871859_u32,388207355_u32,2371751255_u32,285176334_u32,243210537_u32,264525293_u32,2450499805_u32];
_2 = [3238111761_u32,2159699396_u32,3034130684_u32,2375117774_u32,2487451831_u32,1297096504_u32,2794231218_u32,3992790419_u32];
RET.0.0.0 = true as u128;
_2 = [3541443547_u32,1119746465_u32,1409636736_u32,1790848565_u32,780164724_u32,1400399422_u32,3897749765_u32,4163422553_u32];
_5 = [3669733064_u32,3525883768_u32,484536632_u32,4037164977_u32,513439332_u32,3563925790_u32,4259901184_u32,519993317_u32];
_2 = [2310353423_u32,2512532473_u32,2538562782_u32,2893456106_u32,3338755845_u32,2241163580_u32,3298954478_u32,1243648969_u32];
_12 = false;
RET.0.0 = (298589653615702875457639158499827450754_u128,);
_13 = _8 + _8;
RET.0.0.0 = 313571325597970990451724597551326132386_u128 ^ 65826222307342262230447242112981613451_u128;
_7 = !_13;
_1 = [2147661611_u32,4251151245_u32,3448473974_u32,1841478241_u32,529341218_u32,1617808480_u32,2053539613_u32,1794376036_u32];
Goto(bb4)
}
bb4 = {
_15 = !75_i8;
_4 = RET.0.0.0 as u64;
_14 = _13;
RET.0.0 = (270494465930611022197860513218598328655_u128,);
_4 = 2101879036270778184_u64;
_12 = false;
_2 = [3022376925_u32,1364753408_u32,857977970_u32,4185294245_u32,2019053475_u32,812886584_u32,3961279958_u32,2024793785_u32];
_17 = [59_u8,222_u8,233_u8,126_u8,126_u8,14_u8,191_u8];
_2 = [863615004_u32,7338862_u32,2457063090_u32,383115571_u32,3696402571_u32,3653277565_u32,1567648084_u32,1333014752_u32];
RET.0.0 = (137768020573964496111286291219378209846_u128,);
_4 = 1502567510747687551_u64;
RET.0.1 = [13225723282600954019_usize,7_usize,2_usize,2_usize,2_usize];
_20 = _9 as u16;
_4 = 4791843744212823238_u64 << RET.0.0.0;
Call(_22 = fn5(), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET.0.0.0 = 154488875408422348176382381816325410666_u128 * 243566323678629736353892702737004509834_u128;
_15 = 92_i8 * (-47_i8);
_7 = _13 | _13;
RET.0.0.0 = !156330995271424979252096625668082942380_u128;
RET.0.0.0 = 14_u8 as u128;
_7 = -_13;
RET.0.0 = (148230727283459052889920164976444725024_u128,);
RET.0.0.0 = !234724662187813967451684526558943596278_u128;
_1 = _5;
_8 = _4 as isize;
RET.0.1 = [7_usize,7897118546645849675_usize,5472550847960916499_usize,1_usize,14034343056540219819_usize];
_7 = _8 | _8;
_7 = -_13;
_12 = true;
_22 = (-322603408_i32);
_24 = (_14,);
Goto(bb6)
}
bb6 = {
_14 = 1632173079_u32 as isize;
_22 = 1048795224_i32 >> _4;
_19 = 6_usize as f32;
_15 = RET.0.0.0 as i8;
RET.0.1 = [7_usize,3_usize,5711401034087912496_usize,4003354971241274147_usize,9515773714898993597_usize];
RET.0.0.0 = 228161715224479167517549076903682297326_u128;
_5 = [2608447420_u32,922794251_u32,509027171_u32,814329296_u32,13829261_u32,3517644450_u32,3164112218_u32,1286231578_u32];
_9 = 1162174870_u32 as f64;
_24.0 = _13 + _13;
_14 = 2145801468749983841424074178354719172_i128 as isize;
_26.1 = [7_usize,6_usize,2_usize,3761172837684822601_usize,13757195442963816413_usize];
_17 = [48_u8,152_u8,182_u8,237_u8,54_u8,240_u8,233_u8];
_11 = &_24;
RET.0.0.0 = 173295838244272411038208801235014177770_u128;
_25 = [_22];
_22 = _20 as i32;
_15 = 11861620565544752277_usize as i8;
_20 = 10225_u16;
_20 = _9 as u16;
_14 = _19 as isize;
_26.0 = (RET.0.0.0,);
RET.0.1 = [2_usize,0_usize,6_usize,8613421372604669946_usize,0_usize];
_3 = [1243479417_u32,128789189_u32,1352518956_u32,2713823343_u32,3000489969_u32,1061784380_u32,1585007994_u32,1324075203_u32];
_15 = 2236458237_u32 as i8;
RET.0.0.0 = !_26.0.0;
_23 = 5_usize * 3_usize;
Goto(bb7)
}
bb7 = {
_30 = _24.0;
_25 = [_22];
_1 = [3898257068_u32,114945399_u32,2199896918_u32,800682138_u32,2090034943_u32,180390559_u32,754744947_u32,1206531984_u32];
_27 = _24.0 + (*_11).0;
_24 = (_30,);
_31 = _12;
_29 = _23;
_24 = (_27,);
_26.0.0 = !RET.0.0.0;
_27 = _24.0;
_33 = !_23;
_26.0 = (RET.0.0.0,);
_4 = 13051356720208751003_u64 ^ 9787640652150014399_u64;
_11 = &_24;
_28 = _9;
_28 = _9 * _9;
_5 = [2366363708_u32,3359737316_u32,703810239_u32,4070118692_u32,4236559928_u32,3979241137_u32,3830695288_u32,3719783104_u32];
_29 = _23;
_13 = 62_u8 as isize;
_4 = !9164233820961999987_u64;
RET = (_26,);
Goto(bb8)
}
bb8 = {
RET.0.1 = [_33,_29,_33,_33,_23];
_29 = '\u{7aa2}' as usize;
_31 = _12;
RET = (_26,);
_33 = 26137_i16 as usize;
_8 = (*_11).0;
_13 = _27;
RET.0.1 = _26.1;
_34 = core::ptr::addr_of_mut!(_25);
(*_34) = [_22];
Goto(bb9)
}
bb9 = {
RET.0 = (_26.0, _26.1);
RET.0.1 = [_29,_23,_33,_33,_23];
_29 = _33 + _23;
_33 = _20 as usize;
_24.0 = _27;
_13 = 128820669005900702959695193059862779879_i128 as isize;
_9 = -_28;
_11 = &_24;
_35 = -_13;
_8 = (*_11).0;
_3 = _2;
RET = (_26,);
_12 = !_31;
_26.0.0 = !RET.0.0.0;
(*_34) = [_22];
_23 = _29;
Call(_27 = core::intrinsics::bswap(_7), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_5 = _1;
_2 = _3;
_30 = (*_11).0;
_29 = _23 | _23;
RET.0 = (_26.0, _26.1);
_24 = (_8,);
_27 = _8;
_29 = _4 as usize;
_25 = [_22];
_20 = !45569_u16;
RET.0 = (_26.0, _26.1);
_38 = 1520_i16 << _14;
_9 = -_28;
RET.0 = _26;
_36 = _22 as f64;
RET.0.0.0 = 3196405147_u32 as u128;
_11 = &_24;
Call(_19 = fn11(Move(_11), _24.0, (*_11)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
RET = (_26,);
(*_34) = [_22];
_16 = &_37;
_28 = _36;
Goto(bb12)
}
bb12 = {
RET.0 = _26;
_24.0 = _27 & _8;
_9 = 118_u8 as f64;
_26 = (RET.0.0, RET.0.1);
_23 = !_29;
_26.0.0 = _20 as u128;
_26 = (RET.0.0, RET.0.1);
RET.0.1 = [_33,_29,_23,_33,_33];
_11 = &_24;
_28 = _36 * _9;
RET.0 = (_26.0, _26.1);
_31 = !_12;
_13 = !_27;
_24.0 = 363752693_u32 as isize;
_8 = _30;
_16 = &_37;
_35 = _27;
Call(_25 = fn12(_22), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
RET.0.1 = [_29,_23,_33,_33,_33];
_3 = [3735803189_u32,3638539039_u32,3616094176_u32,2901490470_u32,248910869_u32,1778404660_u32,3033108127_u32,204572340_u32];
_8 = _19 as isize;
_26.0.0 = !RET.0.0.0;
_14 = _4 as isize;
_40 = 22936643574251341606552176263232977549_i128 - 120559606811075813495948172355114479537_i128;
Goto(bb14)
}
bb14 = {
_28 = -_9;
_1 = _5;
_36 = _28 * _28;
_17 = [234_u8,182_u8,199_u8,129_u8,144_u8,250_u8,9_u8];
_2 = _5;
_16 = &(*_16);
_2 = [1259708023_u32,2411486716_u32,1322031239_u32,2958782410_u32,2125672018_u32,2427743300_u32,2900975902_u32,1176669129_u32];
RET = (_26,);
_28 = _26.0.0 as f64;
_44 = core::ptr::addr_of_mut!(_43);
_36 = -_9;
_16 = &(*_16);
RET.0 = (_26.0, _26.1);
_20 = 3742193710_u32 as u16;
_2 = _1;
_41 = !_35;
RET.0.0.0 = _26.0.0;
_34 = core::ptr::addr_of_mut!((*_34));
_49 = [532006661_u32,1675663651_u32,4136215075_u32,1256045270_u32,105744253_u32,3002294577_u32,3407827684_u32,639365696_u32];
_30 = _13 * _13;
_7 = _27;
_29 = _23;
_16 = &_37;
Goto(bb15)
}
bb15 = {
Call(_50 = dump_var(4_usize, 24_usize, Move(_24), 8_usize, Move(_8), 20_usize, Move(_20), 27_usize, Move(_27)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_50 = dump_var(4_usize, 26_usize, Move(_26), 41_usize, Move(_41), 13_usize, Move(_13), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_50 = dump_var(4_usize, 3_usize, Move(_3), 7_usize, Move(_7), 49_usize, Move(_49), 38_usize, Move(_38)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_50 = dump_var(4_usize, 17_usize, Move(_17), 25_usize, Move(_25), 51_usize, _51, 51_usize, _51), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5() -> i32 {
mir! {
type RET = i32;
let _1: [i8; 5];
let _2: i32;
let _3: u64;
let _4: Adt44;
let _5: ((isize,), &'static *mut u8, (&'static i64, (u64, i16)), (&'static i64, (u64, i16)));
let _6: i16;
let _7: u8;
let _8: *mut *const [char; 8];
let _9: &'static (isize,);
let _10: char;
let _11: isize;
let _12: &'static &'static *mut u8;
let _13: &'static [i16; 4];
let _14: f64;
let _15: usize;
let _16: ((isize,), &'static *mut u8, (&'static i64, (u64, i16)), (&'static i64, (u64, i16)));
let _17: i64;
let _18: ();
let _19: ();
{
RET = -(-1847573667_i32);
RET = 130_u8 as i32;
RET = (-1172642539_i32);
RET = (-2141007520_i32) * 1533497336_i32;
RET = !(-1960884972_i32);
_1 = [(-74_i8),90_i8,48_i8,17_i8,(-54_i8)];
RET = 504686755_i32;
match RET {
0 => bb1,
504686755 => bb3,
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
RET = 1052182810_i32;
_3 = !5264557902126046345_u64;
RET = (-1234160575_i32) ^ (-666648586_i32);
RET = -(-1666995445_i32);
_2 = RET | RET;
_3 = !11916079386874732865_u64;
_2 = -RET;
_4 = Adt44::Variant1 { fld0: RET,fld1: _3 };
RET = Field::<i32>(Variant(_4, 1), 0);
_1 = [118_i8,(-66_i8),(-62_i8),(-124_i8),(-87_i8)];
Call(place!(Field::<u64>(Variant(_4, 1), 1)) = core::intrinsics::transmute(_3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = 61723_u16 as i32;
_1 = [(-82_i8),5_i8,35_i8,54_i8,53_i8];
place!(Field::<u64>(Variant(_4, 1), 1)) = 4_usize as u64;
_1 = [62_i8,51_i8,33_i8,2_i8,(-125_i8)];
_3 = !Field::<u64>(Variant(_4, 1), 1);
_3 = !Field::<u64>(Variant(_4, 1), 1);
RET = 106_u8 as i32;
_1 = [(-120_i8),52_i8,(-20_i8),82_i8,75_i8];
_5.0 = ((-9223372036854775808_isize),);
_1 = [96_i8,(-27_i8),(-73_i8),84_i8,(-41_i8)];
_1 = [13_i8,(-50_i8),116_i8,30_i8,(-10_i8)];
_5.3.1.0 = Field::<u64>(Variant(_4, 1), 1);
_5.3.1.0 = _3 - _3;
_2 = !Field::<i32>(Variant(_4, 1), 0);
RET = !_2;
_5.3.1.1 = 23419_i16;
_3 = !Field::<u64>(Variant(_4, 1), 1);
_3 = _5.3.1.0 << _2;
RET = Field::<i32>(Variant(_4, 1), 0) * Field::<i32>(Variant(_4, 1), 0);
_5.3.1.0 = !_3;
_5.2.1 = (_5.3.1.0, _5.3.1.1);
_7 = 16_u8 & 112_u8;
_5.2.1 = (_3, _5.3.1.1);
match _5.0.0 {
0 => bb2,
1 => bb5,
2 => bb6,
3 => bb7,
340282366920938463454151235394913435648 => bb9,
_ => bb8
}
}
bb5 = {
RET = 1052182810_i32;
_3 = !5264557902126046345_u64;
RET = (-1234160575_i32) ^ (-666648586_i32);
RET = -(-1666995445_i32);
_2 = RET | RET;
_3 = !11916079386874732865_u64;
_2 = -RET;
_4 = Adt44::Variant1 { fld0: RET,fld1: _3 };
RET = Field::<i32>(Variant(_4, 1), 0);
_1 = [118_i8,(-66_i8),(-62_i8),(-124_i8),(-87_i8)];
Call(place!(Field::<u64>(Variant(_4, 1), 1)) = core::intrinsics::transmute(_3), ReturnTo(bb4), UnwindUnreachable())
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
_5.2.1 = (_5.3.1.0, _5.3.1.1);
_5.2.1 = (_5.3.1.0, _5.3.1.1);
Goto(bb10)
}
bb10 = {
_7 = 210_u8;
_7 = 102_u8 + 37_u8;
_2 = Field::<i32>(Variant(_4, 1), 0) - RET;
place!(Field::<u64>(Variant(_4, 1), 1)) = _5.3.1.0 ^ _3;
_5.0.0 = (-23_isize);
_5.0.0 = 38721_u16 as isize;
_5.0.0 = !(-77_isize);
RET = -_2;
_5.3.1 = _5.2.1;
RET = _2 + _2;
place!(Field::<u64>(Variant(_4, 1), 1)) = 56_i8 as u64;
_2 = RET << _5.3.1.1;
_5.0 = ((-9223372036854775808_isize),);
_2 = RET | RET;
_5.3.1.0 = _5.2.1.0 + _5.2.1.0;
_5.2.1.0 = Field::<u64>(Variant(_4, 1), 1);
_9 = &_5.0;
_2 = RET | Field::<i32>(Variant(_4, 1), 0);
RET = _2 ^ Field::<i32>(Variant(_4, 1), 0);
_5.2.1.0 = _5.3.1.0;
SetDiscriminant(_4, 3);
place!(Field::<usize>(Variant(_4, 3), 3)) = (-112_i8) as usize;
Call(_5.2.1 = fn6(Move(_9), RET, _2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
place!(Field::<usize>(Variant(_4, 3), 3)) = !13874165632654504597_usize;
place!(Field::<u8>(Variant(_4, 3), 2)) = !_7;
_10 = '\u{ed8c6}';
place!(Field::<i16>(Variant(_4, 3), 4)) = _5.2.1.1 * _5.2.1.1;
_6 = 54899_u16 as i16;
place!(Field::<usize>(Variant(_4, 3), 3)) = !7_usize;
_1 = [78_i8,(-39_i8),(-120_i8),49_i8,(-98_i8)];
place!(Field::<u16>(Variant(_4, 3), 5)) = 1937_u16;
_5.0.0 = !(-9223372036854775808_isize);
RET = _2 & _2;
place!(Field::<(u128, i128)>(Variant(_4, 3), 0)).1 = 76010686885141358704179934214415924341_i128;
place!(Field::<(u128, i128)>(Variant(_4, 3), 0)).1 = Field::<u16>(Variant(_4, 3), 5) as i128;
_11 = _5.0.0 >> _5.3.1.0;
place!(Field::<u16>(Variant(_4, 3), 5)) = 19821_u16 - 49204_u16;
_12 = &_5.1;
_9 = &_5.0;
_16.3.1.0 = _3;
_16.3.1 = (_5.3.1.0, _5.3.1.1);
_5.2.1.0 = !_3;
Goto(bb12)
}
bb12 = {
Call(_18 = dump_var(5_usize, 3_usize, Move(_3), 1_usize, Move(_1), 2_usize, Move(_2), 19_usize, _19), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: &'static (isize,),mut _2: i32,mut _3: i32) -> (u64, i16) {
mir! {
type RET = (u64, i16);
let _4: *mut *mut isize;
let _5: [usize; 7];
let _6: char;
let _7: Adt57;
let _8: Adt64;
let _9: u32;
let _10: [i16; 4];
let _11: ([i64; 8], *mut *const [char; 8], [u32; 6], [u16; 4]);
let _12: &'static &'static &'static *mut u8;
let _13: char;
let _14: [i8; 5];
let _15: [u8; 7];
let _16: u64;
let _17: f64;
let _18: [u32; 6];
let _19: ();
let _20: ();
{
_3 = !_2;
RET = (9578816666337584919_u64, 10651_i16);
_5 = [9718734882011769221_usize,6_usize,1_usize,1139092388530071483_usize,6634866652315798152_usize,15868579027609673943_usize,0_usize];
_2 = _3 ^ _3;
Call(RET.0 = fn7(RET.1, _3, _2, _3, _3, _2, _2, RET.1, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = (9474679524259040866_u64, 1882_i16);
RET = (6977091666750754844_u64, (-22346_i16));
_2 = _3 | _3;
RET.1 = 29684_i16 ^ 19080_i16;
RET.1 = 97_i16 - 18174_i16;
_5 = [6_usize,1_usize,12670532678491966532_usize,6378997457689819731_usize,0_usize,7413783094879736006_usize,13233988415731963510_usize];
_2 = _3 * _3;
_2 = _3 << _3;
_6 = '\u{8ed9b}';
_2 = !_3;
RET = (11340423313488919073_u64, 9851_i16);
RET = (15492558459221904495_u64, 7672_i16);
_6 = '\u{cd3d0}';
RET.1 = (-32259_i16);
Goto(bb2)
}
bb2 = {
_6 = '\u{9726e}';
RET = (1527761171527708769_u64, (-4557_i16));
_6 = '\u{bd96d}';
RET.0 = !11699782122079185958_u64;
RET.1 = !(-30161_i16);
RET = (14689535019827966039_u64, 987_i16);
RET.0 = 24889_u16 as u64;
RET.1 = -(-5111_i16);
_3 = 56_i8 as i32;
_5 = [14463963871901799987_usize,2_usize,4_usize,3_usize,17612351803494062503_usize,4990765228278514805_usize,5_usize];
_3 = 71512178678530301026221079913110807722_u128 as i32;
_6 = '\u{7c1f3}';
RET.1 = (-7361089183607158230_i64) as i16;
_2 = 9223372036854775807_isize as i32;
_5 = [2_usize,2972314172664067091_usize,191294150215712323_usize,2721351567162818370_usize,3_usize,6818368519046314071_usize,14791253706890356334_usize];
_6 = '\u{53da7}';
RET.0 = !9824528480565302894_u64;
_2 = _3 - _3;
RET = (9762293094670047002_u64, 10142_i16);
RET = (8025685564527049344_u64, 19957_i16);
RET.0 = 15709146386029249469_u64;
Goto(bb3)
}
bb3 = {
_3 = _2;
RET = (13067291165584664059_u64, 8999_i16);
RET.0 = 122_i8 as u64;
RET = (13957996955182756333_u64, 23256_i16);
_6 = '\u{86ee4}';
RET = (15139892001384257254_u64, (-30614_i16));
_6 = '\u{7c4d9}';
_3 = _2 - _2;
RET = (6584597426017988309_u64, (-18464_i16));
RET.1 = 37_u8 as i16;
_3 = -_2;
RET.1 = 9223372036854775807_isize as i16;
RET = (10186928191695213840_u64, 31717_i16);
_2 = _3 - _3;
_2 = _3;
_6 = '\u{dd52f}';
_5 = [18327687149655194536_usize,0_usize,10459492771509518482_usize,10530720363987524057_usize,1_usize,0_usize,11745766210844335610_usize];
_2 = _3;
RET.1 = -30858_i16;
_9 = 1271958043_u32 + 3634469582_u32;
_9 = 9181005253856498741_i64 as u32;
_10 = [RET.1,RET.1,RET.1,RET.1];
_6 = '\u{100c26}';
RET = (9391062418363094888_u64, 6815_i16);
_3 = -_2;
_6 = '\u{19921}';
RET = (4061583491319446029_u64, 26737_i16);
RET.0 = 119747296854533680212592408621009307312_i128 as u64;
_9 = 1307525297_u32 | 1120445291_u32;
RET = (8757306057250331270_u64, (-27106_i16));
match RET.1 {
340282366920938463463374607431768184350 => bb5,
_ => bb4
}
}
bb4 = {
_6 = '\u{9726e}';
RET = (1527761171527708769_u64, (-4557_i16));
_6 = '\u{bd96d}';
RET.0 = !11699782122079185958_u64;
RET.1 = !(-30161_i16);
RET = (14689535019827966039_u64, 987_i16);
RET.0 = 24889_u16 as u64;
RET.1 = -(-5111_i16);
_3 = 56_i8 as i32;
_5 = [14463963871901799987_usize,2_usize,4_usize,3_usize,17612351803494062503_usize,4990765228278514805_usize,5_usize];
_3 = 71512178678530301026221079913110807722_u128 as i32;
_6 = '\u{7c1f3}';
RET.1 = (-7361089183607158230_i64) as i16;
_2 = 9223372036854775807_isize as i32;
_5 = [2_usize,2972314172664067091_usize,191294150215712323_usize,2721351567162818370_usize,3_usize,6818368519046314071_usize,14791253706890356334_usize];
_6 = '\u{53da7}';
RET.0 = !9824528480565302894_u64;
_2 = _3 - _3;
RET = (9762293094670047002_u64, 10142_i16);
RET = (8025685564527049344_u64, 19957_i16);
RET.0 = 15709146386029249469_u64;
Goto(bb3)
}
bb5 = {
_5 = [7_usize,10154603987335182185_usize,11588639146319149465_usize,8710794956929374164_usize,1_usize,14344489724841783959_usize,10079756782346533170_usize];
_5 = [2_usize,14487700883342345507_usize,7139633893033873107_usize,5048695345624267270_usize,2264554305293368389_usize,2_usize,10700779959311287817_usize];
RET.0 = 23892_u16 as u64;
_10 = [RET.1,RET.1,RET.1,RET.1];
RET.1 = 5346_i16;
RET.0 = 232015789293722635_i64 as u64;
_2 = _3;
_5 = [7_usize,10013401218311851667_usize,0_usize,4135676063849597407_usize,14641307372090030374_usize,16246698004976521205_usize,4835025721663932745_usize];
_9 = 33584818_u32 | 2133223820_u32;
_11.0 = [(-6726480750779301896_i64),4529897600687646151_i64,4626763100852912066_i64,(-869361600210501857_i64),1600009548188684097_i64,8761391143839383199_i64,1241573105190160655_i64,(-7368796749013910605_i64)];
_9 = 1070875448_u32 + 2761006557_u32;
_11.3 = [7129_u16,50901_u16,17498_u16,31196_u16];
_11.2 = [_9,_9,_9,_9,_9,_9];
RET = (3828195425347570232_u64, (-24616_i16));
_6 = '\u{e745a}';
RET.0 = 289396629979422120_u64;
_3 = (-167676019592530547035161141430012362266_i128) as i32;
_11.0 = [4649977258747878418_i64,8304098654903973455_i64,(-8413100762155577240_i64),471210334529024492_i64,3287777276376788080_i64,(-380785247976153710_i64),(-9173310523095606243_i64),8719909039647880675_i64];
_3 = _2 * _2;
_11.2 = [_9,_9,_9,_9,_9,_9];
_14 = [(-4_i8),63_i8,46_i8,102_i8,(-31_i8)];
_11.2 = [_9,_9,_9,_9,_9,_9];
match RET.1 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
340282366920938463463374607431768186840 => bb14,
_ => bb13
}
}
bb6 = {
_6 = '\u{9726e}';
RET = (1527761171527708769_u64, (-4557_i16));
_6 = '\u{bd96d}';
RET.0 = !11699782122079185958_u64;
RET.1 = !(-30161_i16);
RET = (14689535019827966039_u64, 987_i16);
RET.0 = 24889_u16 as u64;
RET.1 = -(-5111_i16);
_3 = 56_i8 as i32;
_5 = [14463963871901799987_usize,2_usize,4_usize,3_usize,17612351803494062503_usize,4990765228278514805_usize,5_usize];
_3 = 71512178678530301026221079913110807722_u128 as i32;
_6 = '\u{7c1f3}';
RET.1 = (-7361089183607158230_i64) as i16;
_2 = 9223372036854775807_isize as i32;
_5 = [2_usize,2972314172664067091_usize,191294150215712323_usize,2721351567162818370_usize,3_usize,6818368519046314071_usize,14791253706890356334_usize];
_6 = '\u{53da7}';
RET.0 = !9824528480565302894_u64;
_2 = _3 - _3;
RET = (9762293094670047002_u64, 10142_i16);
RET = (8025685564527049344_u64, 19957_i16);
RET.0 = 15709146386029249469_u64;
Goto(bb3)
}
bb7 = {
_3 = _2;
RET = (13067291165584664059_u64, 8999_i16);
RET.0 = 122_i8 as u64;
RET = (13957996955182756333_u64, 23256_i16);
_6 = '\u{86ee4}';
RET = (15139892001384257254_u64, (-30614_i16));
_6 = '\u{7c4d9}';
_3 = _2 - _2;
RET = (6584597426017988309_u64, (-18464_i16));
RET.1 = 37_u8 as i16;
_3 = -_2;
RET.1 = 9223372036854775807_isize as i16;
RET = (10186928191695213840_u64, 31717_i16);
_2 = _3 - _3;
_2 = _3;
_6 = '\u{dd52f}';
_5 = [18327687149655194536_usize,0_usize,10459492771509518482_usize,10530720363987524057_usize,1_usize,0_usize,11745766210844335610_usize];
_2 = _3;
RET.1 = -30858_i16;
_9 = 1271958043_u32 + 3634469582_u32;
_9 = 9181005253856498741_i64 as u32;
_10 = [RET.1,RET.1,RET.1,RET.1];
_6 = '\u{100c26}';
RET = (9391062418363094888_u64, 6815_i16);
_3 = -_2;
_6 = '\u{19921}';
RET = (4061583491319446029_u64, 26737_i16);
RET.0 = 119747296854533680212592408621009307312_i128 as u64;
_9 = 1307525297_u32 | 1120445291_u32;
RET = (8757306057250331270_u64, (-27106_i16));
match RET.1 {
340282366920938463463374607431768184350 => bb5,
_ => bb4
}
}
bb8 = {
_6 = '\u{9726e}';
RET = (1527761171527708769_u64, (-4557_i16));
_6 = '\u{bd96d}';
RET.0 = !11699782122079185958_u64;
RET.1 = !(-30161_i16);
RET = (14689535019827966039_u64, 987_i16);
RET.0 = 24889_u16 as u64;
RET.1 = -(-5111_i16);
_3 = 56_i8 as i32;
_5 = [14463963871901799987_usize,2_usize,4_usize,3_usize,17612351803494062503_usize,4990765228278514805_usize,5_usize];
_3 = 71512178678530301026221079913110807722_u128 as i32;
_6 = '\u{7c1f3}';
RET.1 = (-7361089183607158230_i64) as i16;
_2 = 9223372036854775807_isize as i32;
_5 = [2_usize,2972314172664067091_usize,191294150215712323_usize,2721351567162818370_usize,3_usize,6818368519046314071_usize,14791253706890356334_usize];
_6 = '\u{53da7}';
RET.0 = !9824528480565302894_u64;
_2 = _3 - _3;
RET = (9762293094670047002_u64, 10142_i16);
RET = (8025685564527049344_u64, 19957_i16);
RET.0 = 15709146386029249469_u64;
Goto(bb3)
}
bb9 = {
RET = (9474679524259040866_u64, 1882_i16);
RET = (6977091666750754844_u64, (-22346_i16));
_2 = _3 | _3;
RET.1 = 29684_i16 ^ 19080_i16;
RET.1 = 97_i16 - 18174_i16;
_5 = [6_usize,1_usize,12670532678491966532_usize,6378997457689819731_usize,0_usize,7413783094879736006_usize,13233988415731963510_usize];
_2 = _3 * _3;
_2 = _3 << _3;
_6 = '\u{8ed9b}';
_2 = !_3;
RET = (11340423313488919073_u64, 9851_i16);
RET = (15492558459221904495_u64, 7672_i16);
_6 = '\u{cd3d0}';
RET.1 = (-32259_i16);
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
Return()
}
bb14 = {
_3 = !_2;
_14 = [(-30_i8),(-50_i8),59_i8,(-27_i8),83_i8];
RET = (11151628443488360010_u64, 22082_i16);
RET = (10491001854094304102_u64, (-25911_i16));
_16 = !RET.0;
_14 = [71_i8,(-102_i8),(-65_i8),56_i8,25_i8];
RET.1 = (-1900_i16);
_18 = _11.2;
_17 = 262456786455822560418388872723533946881_u128 as f64;
_15 = [59_u8,75_u8,79_u8,30_u8,222_u8,158_u8,92_u8];
_11.2 = _18;
_13 = _6;
RET = (_16, 29203_i16);
_14 = [58_i8,17_i8,111_i8,93_i8,26_i8];
_5 = [3_usize,13143125467673460528_usize,6_usize,16009836446206013043_usize,5336638433348229898_usize,7_usize,0_usize];
_11.3 = [31822_u16,25520_u16,25576_u16,43482_u16];
Goto(bb15)
}
bb15 = {
Call(_19 = dump_var(6_usize, 2_usize, Move(_2), 3_usize, Move(_3), 5_usize, Move(_5), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_19 = dump_var(6_usize, 14_usize, Move(_14), 20_usize, _20, 20_usize, _20, 20_usize, _20), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: i16,mut _2: i32,mut _3: i32,mut _4: i32,mut _5: i32,mut _6: i32,mut _7: i32,mut _8: i16,mut _9: i32) -> u64 {
mir! {
type RET = u64;
let _10: *const [char; 8];
let _11: *mut &'static i32;
let _12: isize;
let _13: [u32; 8];
let _14: isize;
let _15: *mut isize;
let _16: [u32; 8];
let _17: isize;
let _18: *mut *mut u8;
let _19: i16;
let _20: char;
let _21: *mut char;
let _22: i16;
let _23: bool;
let _24: *mut &'static u128;
let _25: usize;
let _26: *mut *mut u8;
let _27: f32;
let _28: Adt50;
let _29: char;
let _30: ();
let _31: ();
{
RET = 26207_u16 as u64;
_9 = _2;
_6 = _3 >> _3;
_9 = _7;
RET = 34688_u16 as u64;
_8 = '\u{6acb9}' as i16;
_9 = _7 ^ _5;
_8 = _1 ^ _1;
_5 = _6 + _6;
_5 = '\u{30d2b}' as i32;
_4 = 169_u8 as i32;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
10651 => bb6,
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
_9 = _3;
RET = (-9223372036854775808_isize) as u64;
RET = 15208787008360483333_u64 - 12333292838363263368_u64;
RET = 14761989058352738607_u64;
RET = !7207792976196461322_u64;
_9 = 75609875078356401606135047755995295029_u128 as i32;
_3 = _7;
_12 = (-77_isize);
_5 = _3;
RET = 15630572369445846255_u64 + 14694008544080625301_u64;
_13 = [1171451337_u32,1062734891_u32,2902676293_u32,3423047425_u32,273386556_u32,1459003256_u32,3401913516_u32,1165155978_u32];
_9 = _7;
_2 = _7;
_1 = _8 >> _6;
_14 = _1 as isize;
_3 = _14 as i32;
match _12 {
0 => bb1,
1 => bb5,
2 => bb3,
340282366920938463463374607431768211379 => bb8,
_ => bb7
}
}
bb7 = {
Return()
}
bb8 = {
RET = 701449800652872247_u64 | 7214075821889951475_u64;
_15 = core::ptr::addr_of_mut!(_17);
(*_15) = !_12;
_15 = core::ptr::addr_of_mut!(_14);
_2 = true as i32;
_14 = !_12;
_13 = [1398633707_u32,3187850151_u32,313957673_u32,3058321662_u32,1472581064_u32,201808298_u32,2254765730_u32,311920930_u32];
_17 = (-70_i8) as isize;
_3 = _6 * _5;
_6 = -_7;
_13 = [176773454_u32,2368977390_u32,202394222_u32,2524453469_u32,1580006504_u32,3430690583_u32,3462621638_u32,3019212794_u32];
_13 = [3671115419_u32,969046692_u32,2657224980_u32,1569293668_u32,177262287_u32,4050597039_u32,3358939479_u32,1646075330_u32];
_19 = (-7183002305359976754_i64) as i16;
_1 = _19;
_7 = !_3;
(*_15) = _17;
_20 = '\u{db9cf}';
_12 = !(*_15);
Goto(bb9)
}
bb9 = {
_4 = _3;
_2 = _3 - _4;
_15 = core::ptr::addr_of_mut!((*_15));
_3 = _5 ^ _7;
(*_15) = !_12;
_12 = 28968_u16 as isize;
_19 = _8;
RET = 1773365390838689973_u64;
Call(RET = core::intrinsics::bswap(11736548171603909703_u64), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
RET = 16117765636558488218_u64 + 1812328261814003225_u64;
_8 = _19 * _1;
_19 = true as i16;
_17 = !_12;
_16 = [2913850144_u32,745432256_u32,1546908773_u32,1355364957_u32,3714091114_u32,3463716053_u32,4028370973_u32,59433647_u32];
_12 = -(*_15);
Call(_6 = core::intrinsics::bswap(_2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
RET = !10373267002144930481_u64;
_6 = _3 + _2;
_16 = [1251187934_u32,2222323273_u32,1475583462_u32,2947754064_u32,734036672_u32,3594960989_u32,3649397478_u32,1060365667_u32];
_20 = '\u{30af6}';
_12 = -_17;
_21 = core::ptr::addr_of_mut!(_20);
_22 = _8;
_1 = 54767559779318090537504041512722873654_u128 as i16;
_21 = core::ptr::addr_of_mut!((*_21));
_23 = !true;
_3 = !_2;
_23 = !false;
_16 = _13;
_8 = _22;
_6 = -_2;
_19 = _8;
_4 = _7;
_12 = !(*_15);
_4 = _6 & _2;
(*_21) = '\u{10c63c}';
(*_15) = 4302514214453959744_i64 as isize;
(*_21) = '\u{41026}';
_23 = true ^ false;
_17 = _12;
Goto(bb12)
}
bb12 = {
_19 = -_8;
_1 = 3658054680_u32 as i16;
_23 = !true;
_2 = !_4;
(*_15) = -_12;
_23 = !true;
(*_21) = '\u{3e95b}';
_21 = core::ptr::addr_of_mut!(_20);
_9 = -_6;
_15 = core::ptr::addr_of_mut!(_17);
_25 = 2_usize - 6035032676942267535_usize;
_8 = _19 & _22;
_7 = (-71_i8) as i32;
RET = 15623402005789226802_u64;
_17 = 34537470612053397817158930438742515808_i128 as isize;
Call(_8 = fn8(), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_7 = _12 as i32;
_20 = '\u{2509e}';
Goto(bb14)
}
bb14 = {
_9 = _3;
_27 = _2 as f32;
_14 = !_17;
_8 = 1247326008562241271_i64 as i16;
_9 = !_2;
_4 = !_2;
_7 = !_6;
_19 = -_22;
_3 = _7;
_25 = 0_usize | 5_usize;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(7_usize, 16_usize, Move(_16), 22_usize, Move(_22), 7_usize, Move(_7), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(7_usize, 2_usize, Move(_2), 6_usize, Move(_6), 4_usize, Move(_4), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(7_usize, 14_usize, Move(_14), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8() -> i16 {
mir! {
type RET = i16;
let _1: [u32; 3];
let _2: isize;
let _3: u16;
let _4: &'static (isize,);
let _5: Adt82;
let _6: *const [u32; 6];
let _7: [i64; 8];
let _8: i128;
let _9: (((isize,), &'static *mut u8, (&'static i64, (u64, i16)), (&'static i64, (u64, i16))), *mut *mut u8, usize, [i32; 1]);
let _10: isize;
let _11: *mut char;
let _12: bool;
let _13: &'static isize;
let _14: u32;
let _15: f32;
let _16: isize;
let _17: *const i8;
let _18: f32;
let _19: bool;
let _20: [usize; 5];
let _21: *mut char;
let _22: isize;
let _23: *mut Adt76;
let _24: isize;
let _25: [i16; 4];
let _26: &'static i128;
let _27: i32;
let _28: [u32; 6];
let _29: u8;
let _30: u32;
let _31: [u32; 8];
let _32: [usize; 7];
let _33: ();
let _34: ();
{
RET = (-18426_i16) * (-14237_i16);
RET = 6832461869260177880_i64 as i16;
RET = 28875_i16 - 14262_i16;
Goto(bb1)
}
bb1 = {
RET = true as i16;
RET = !(-26405_i16);
RET = 18729_i16 + 958_i16;
RET = 9167_i16;
_1 = [1694779743_u32,1472088692_u32,2756232199_u32];
RET = true as i16;
RET = 4032330850602951206_i64 as i16;
RET = -(-6543_i16);
_2 = -(-9223372036854775808_isize);
_2 = 106_isize;
_3 = 38570_u16;
RET = !(-18763_i16);
Goto(bb2)
}
bb2 = {
_3 = 42131_u16 * 5831_u16;
_3 = !2472_u16;
RET = !13123_i16;
_5.fld2 = [RET,RET,RET,RET];
_5.fld1 = [822684185_u32,2420459127_u32,504313549_u32,1951255120_u32,866921328_u32,3439448753_u32];
_5.fld0 = [7_usize,7_usize,2_usize,13799671987853960029_usize,7_usize];
_6 = core::ptr::addr_of!(_5.fld1);
(*_6) = [2467655265_u32,2892581666_u32,949639069_u32,1128427697_u32,2317921272_u32,2338713685_u32];
(*_6) = [866216614_u32,1747553156_u32,2621230942_u32,3379324547_u32,1818470477_u32,1224403280_u32];
_5.fld1 = [1100278468_u32,1754033336_u32,2909643027_u32,1070121625_u32,2288472372_u32,3583521925_u32];
_5.fld2 = [RET,RET,RET,RET];
_3 = 4442_u16;
(*_6) = [1619477119_u32,594733750_u32,1556923323_u32,3507195205_u32,3633604673_u32,1702370771_u32];
_2 = (-56_isize);
_3 = 32651_u16 * 53453_u16;
_2 = (-9223372036854775808_isize) | (-41_isize);
Call(RET = core::intrinsics::transmute(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5.fld0 = [0_usize,4_usize,0_usize,2_usize,7_usize];
RET = 27550_i16;
(*_6) = [3133033981_u32,1952769425_u32,1958300049_u32,1847569420_u32,3132746350_u32,1990136110_u32];
_1 = [4116604790_u32,2001235560_u32,3408932575_u32];
_5.fld1 = [778737031_u32,3894341606_u32,641017302_u32,623373887_u32,3163391920_u32,1247019134_u32];
_3 = 55340_u16 * 3533_u16;
_3 = !36319_u16;
(*_6) = [3039167570_u32,3924655604_u32,2648415350_u32,4191787964_u32,4114751851_u32,1828659396_u32];
Call(_5.fld0 = fn9(Move(_6), (*_6)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_2 = 9223372036854775807_isize + (-9223372036854775808_isize);
_6 = core::ptr::addr_of!(_5.fld1);
(*_6) = [3409494155_u32,3530607463_u32,1074151036_u32,1964877407_u32,3020251103_u32,501543120_u32];
match RET {
0 => bb1,
27550 => bb5,
_ => bb2
}
}
bb5 = {
_6 = core::ptr::addr_of!(_5.fld1);
RET = 1627320747_i32 as i16;
RET = (-25064_i16) >> _2;
Goto(bb6)
}
bb6 = {
_2 = (-1412890813_i32) as isize;
RET = (-4789_i16);
_8 = (-70030525662663219347121618740020766931_i128) << _2;
RET = 0_usize as i16;
_7 = [(-5784741734837003159_i64),6695694237222716878_i64,3050408698851568289_i64,8610206932101228682_i64,(-775188918082719721_i64),8947246094024142918_i64,(-4054111417283040531_i64),(-3661099465510661327_i64)];
RET = 11587_i16;
_5.fld1 = [3544031105_u32,2358065241_u32,3416539703_u32,1512013194_u32,3007501541_u32,1949938079_u32];
_9.0.3.1.1 = -RET;
RET = _9.0.3.1.1;
_9.0.2.1 = (13924425835588710953_u64, _9.0.3.1.1);
Goto(bb7)
}
bb7 = {
_9.0.3.1.0 = true as u64;
Goto(bb8)
}
bb8 = {
_2 = 4_isize >> _3;
_9.0.3.1 = (_9.0.2.1.0, RET);
_6 = core::ptr::addr_of!((*_6));
_5.fld4 = core::ptr::addr_of_mut!(_9.3);
_9.0.3.1.1 = _9.0.2.1.1;
_9.0.0 = (_2,);
_9.0.2.1.1 = _9.0.3.1.0 as i16;
_5.fld2 = [_9.0.2.1.1,RET,_9.0.2.1.1,RET];
_12 = !false;
_10 = _9.0.2.1.0 as isize;
_10 = _9.0.0.0 - _9.0.0.0;
RET = _9.0.3.1.1;
(*_6) = [1380318648_u32,1831177508_u32,3715388600_u32,824099628_u32,1594203070_u32,1352724277_u32];
_9.0.0.0 = -_2;
_9.3 = [1350612447_i32];
_9.0.3.1.1 = -RET;
_9.0.2.1.1 = RET ^ _9.0.3.1.1;
(*_6) = [3738911796_u32,4094060026_u32,2991954334_u32,545681562_u32,3556254456_u32,850656796_u32];
_9.0.2.1.0 = _9.0.3.1.0;
_9.2 = 15385199704627927709_usize;
RET = _12 as i16;
_5.fld0 = [_9.2,_9.2,_9.2,_9.2,_9.2];
_4 = &_9.0.0;
match _9.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
15385199704627927709 => bb9,
_ => bb7
}
}
bb9 = {
_9.0.2.1.1 = !_9.0.3.1.1;
_5.fld1 = [3979000932_u32,519250102_u32,3391135226_u32,511618439_u32,1888254006_u32,2697693007_u32];
_15 = _9.0.3.1.1 as f32;
_15 = _9.0.3.1.1 as f32;
_2 = _9.0.0.0 + (*_4).0;
_9.0.0.0 = _15 as isize;
_13 = &_10;
_10 = _2 - _2;
_9.0.3.1 = _9.0.2.1;
_4 = &_9.0.0;
_9.2 = !6_usize;
_9.0.0.0 = _10 | _2;
_18 = _15 - _15;
_19 = !_12;
_15 = 180_u8 as f32;
_12 = _9.0.3.1.0 > _9.0.3.1.0;
_4 = &_9.0.0;
_19 = !_12;
_3 = !54247_u16;
_5.fld0 = [_9.2,_9.2,_9.2,_9.2,_9.2];
_16 = 210053987667038681143965845265226189842_u128 as isize;
_9.0.3.1.1 = _9.0.0.0 as i16;
_20 = [_9.2,_9.2,_9.2,_9.2,_9.2];
(*_6) = [2456944394_u32,692987840_u32,1224889868_u32,4006958764_u32,1261786426_u32,366710472_u32];
Goto(bb10)
}
bb10 = {
_18 = -_15;
_13 = &_10;
Goto(bb11)
}
bb11 = {
_9.0.3.1.0 = _9.0.2.1.0 * _9.0.2.1.0;
_9.0.3.1.0 = !_9.0.2.1.0;
_5.fld0 = _20;
_5.fld0 = [_9.2,_9.2,_9.2,_9.2,_9.2];
_22 = (*_13) | (*_13);
(*_6) = [239590188_u32,287505031_u32,1794376455_u32,2018725430_u32,882886558_u32,4222873092_u32];
_16 = -_10;
_3 = !60377_u16;
_9.0.3.1.0 = _9.0.2.1.0;
match _9.0.2.1.0 {
0 => bb12,
13924425835588710953 => bb14,
_ => bb13
}
}
bb12 = {
_3 = 42131_u16 * 5831_u16;
_3 = !2472_u16;
RET = !13123_i16;
_5.fld2 = [RET,RET,RET,RET];
_5.fld1 = [822684185_u32,2420459127_u32,504313549_u32,1951255120_u32,866921328_u32,3439448753_u32];
_5.fld0 = [7_usize,7_usize,2_usize,13799671987853960029_usize,7_usize];
_6 = core::ptr::addr_of!(_5.fld1);
(*_6) = [2467655265_u32,2892581666_u32,949639069_u32,1128427697_u32,2317921272_u32,2338713685_u32];
(*_6) = [866216614_u32,1747553156_u32,2621230942_u32,3379324547_u32,1818470477_u32,1224403280_u32];
_5.fld1 = [1100278468_u32,1754033336_u32,2909643027_u32,1070121625_u32,2288472372_u32,3583521925_u32];
_5.fld2 = [RET,RET,RET,RET];
_3 = 4442_u16;
(*_6) = [1619477119_u32,594733750_u32,1556923323_u32,3507195205_u32,3633604673_u32,1702370771_u32];
_2 = (-56_isize);
_3 = 32651_u16 * 53453_u16;
_2 = (-9223372036854775808_isize) | (-41_isize);
Call(RET = core::intrinsics::transmute(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_2 = 4_isize >> _3;
_9.0.3.1 = (_9.0.2.1.0, RET);
_6 = core::ptr::addr_of!((*_6));
_5.fld4 = core::ptr::addr_of_mut!(_9.3);
_9.0.3.1.1 = _9.0.2.1.1;
_9.0.0 = (_2,);
_9.0.2.1.1 = _9.0.3.1.0 as i16;
_5.fld2 = [_9.0.2.1.1,RET,_9.0.2.1.1,RET];
_12 = !false;
_10 = _9.0.2.1.0 as isize;
_10 = _9.0.0.0 - _9.0.0.0;
RET = _9.0.3.1.1;
(*_6) = [1380318648_u32,1831177508_u32,3715388600_u32,824099628_u32,1594203070_u32,1352724277_u32];
_9.0.0.0 = -_2;
_9.3 = [1350612447_i32];
_9.0.3.1.1 = -RET;
_9.0.2.1.1 = RET ^ _9.0.3.1.1;
(*_6) = [3738911796_u32,4094060026_u32,2991954334_u32,545681562_u32,3556254456_u32,850656796_u32];
_9.0.2.1.0 = _9.0.3.1.0;
_9.2 = 15385199704627927709_usize;
RET = _12 as i16;
_5.fld0 = [_9.2,_9.2,_9.2,_9.2,_9.2];
_4 = &_9.0.0;
match _9.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
15385199704627927709 => bb9,
_ => bb7
}
}
bb14 = {
_8 = (-15508174752528337146593372613305439735_i128);
(*_6) = [3332919154_u32,3989875227_u32,3538375198_u32,3854835834_u32,1807034195_u32,4292054536_u32];
_3 = 6458_u16;
_9.0.3.1.1 = RET;
_15 = _2 as f32;
_14 = !1810925482_u32;
_16 = (-115_i8) as isize;
_5.fld0 = [_9.2,_9.2,_9.2,_9.2,_9.2];
_9.0.2.1.1 = RET - _9.0.3.1.1;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(8_usize, 10_usize, Move(_10), 8_usize, Move(_8), 1_usize, Move(_1), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(8_usize, 16_usize, Move(_16), 20_usize, Move(_20), 34_usize, _34, 34_usize, _34), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: *const [u32; 6],mut _2: [u32; 6]) -> [usize; 5] {
mir! {
type RET = [usize; 5];
let _3: i8;
let _4: *mut &'static u128;
let _5: i8;
let _6: [i32; 1];
let _7: *const [char; 8];
let _8: isize;
let _9: usize;
let _10: *mut Adt76;
let _11: *const i8;
let _12: *const i8;
let _13: char;
let _14: [i8; 4];
let _15: *const *mut *mut isize;
let _16: f32;
let _17: &'static [u8; 6];
let _18: f64;
let _19: bool;
let _20: [i64; 8];
let _21: char;
let _22: i128;
let _23: f32;
let _24: Adt50;
let _25: f64;
let _26: isize;
let _27: isize;
let _28: isize;
let _29: f64;
let _30: *const i8;
let _31: i64;
let _32: bool;
let _33: usize;
let _34: usize;
let _35: i64;
let _36: &'static &'static *mut u8;
let _37: ();
let _38: ();
{
RET = [5_usize,0_usize,3_usize,6_usize,6_usize];
_2 = [2158240632_u32,4001576015_u32,536478270_u32,716997887_u32,1630550295_u32,806973168_u32];
_3 = -76_i8;
RET = [7_usize,3_usize,13444072115138835530_usize,4_usize,7480222473904637755_usize];
_3 = 22482_u16 as i8;
_3 = -(-47_i8);
_5 = !_3;
_6 = [1066707479_i32];
_5 = true as i8;
RET = [9848031778243538392_usize,7_usize,8988368529973141988_usize,2718653916289454332_usize,13003994306676248669_usize];
_8 = (-9223372036854775808_isize) + 20_isize;
RET = [11979772919339125536_usize,380294537382064064_usize,8984323835573193113_usize,4597241142195446359_usize,3_usize];
_6 = [820987678_i32];
_9 = 13130463140838083744_u64 as usize;
_5 = -_3;
_5 = false as i8;
_5 = _3 >> _8;
_8 = 99_isize ^ (-9223372036854775808_isize);
_5 = _3 << _9;
_8 = 9223372036854775807_isize * (-9223372036854775808_isize);
Goto(bb1)
}
bb1 = {
_9 = 2599141154_u32 as usize;
_2 = [3785199613_u32,488222468_u32,213435793_u32,1382174938_u32,2208980726_u32,2766328646_u32];
_5 = 19149705185540730798285902094136308350_u128 as i8;
_3 = _5;
_11 = core::ptr::addr_of!(_3);
_2 = [3240537582_u32,430290605_u32,3775427310_u32,1952446155_u32,3284284519_u32,1727970787_u32];
_6 = [1104808364_i32];
_12 = core::ptr::addr_of!((*_11));
RET = [_9,_9,_9,_9,_9];
_12 = Move(_11);
RET = [_9,_9,_9,_9,_9];
_2 = [2126387526_u32,2517767478_u32,2121429147_u32,1572908595_u32,606975852_u32,205673994_u32];
_13 = '\u{10caf7}';
_2 = [2432460000_u32,3125507403_u32,1096540024_u32,1505449112_u32,4115328317_u32,1425249096_u32];
RET = [_9,_9,_9,_9,_9];
_6 = [529197701_i32];
_14 = [_5,_5,_3,_5];
Call(_3 = fn10(RET, Move(_12), Move(_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
match _3 {
103 => bb3,
_ => bb1
}
}
bb3 = {
_6 = [(-1877444987_i32)];
_1 = core::ptr::addr_of!(_2);
_1 = core::ptr::addr_of!((*_1));
_11 = core::ptr::addr_of!(_5);
match _3 {
0 => bb1,
1 => bb2,
103 => bb5,
_ => bb4
}
}
bb4 = {
match _3 {
103 => bb3,
_ => bb1
}
}
bb5 = {
(*_11) = _3 << _8;
RET = [_9,_9,_9,_9,_9];
_8 = -(-9223372036854775808_isize);
_13 = '\u{2fcbd}';
_11 = core::ptr::addr_of!((*_11));
(*_11) = false as i8;
(*_1) = [1056046890_u32,3032702084_u32,1102206087_u32,3593609834_u32,1938643948_u32,3089032986_u32];
_18 = _9 as f64;
_1 = core::ptr::addr_of!((*_1));
_2 = [2914882583_u32,3880206124_u32,4202991454_u32,2007738939_u32,3071966595_u32,467255628_u32];
_5 = -_3;
Goto(bb6)
}
bb6 = {
_6 = [(-1772545563_i32)];
Goto(bb7)
}
bb7 = {
_19 = (*_11) < (*_11);
_14 = [(*_11),_3,(*_11),_5];
_20 = [(-723045318205316112_i64),9173023116021248503_i64,(-5907198132324528210_i64),5607646013376719356_i64,(-818547904018321219_i64),(-4023335936565564344_i64),(-6718647073889183664_i64),8703739360499988052_i64];
_13 = '\u{6626f}';
RET = [_9,_9,_9,_9,_9];
_22 = (-69290364214542726299136369209511788015_i128) | 80203815491166389913728292277649383044_i128;
_14 = [_3,(*_11),(*_11),_3];
_6 = [(-824827393_i32)];
_21 = _13;
_11 = core::ptr::addr_of!((*_11));
_13 = _21;
(*_1) = [1922662391_u32,1161949991_u32,943409548_u32,4002949436_u32,1611107034_u32,1921875017_u32];
_9 = 3_usize;
_2[_9] = !1237490591_u32;
_12 = core::ptr::addr_of!(_14[_9]);
_12 = Move(_11);
_8 = !(-9223372036854775808_isize);
_5 = 2945_u16 as i8;
_6 = [1608972543_i32];
RET[_9] = _9;
_20 = [(-5130201550637275282_i64),(-4541967243153169999_i64),(-63118780392313789_i64),4788658092519071698_i64,1789329591131350852_i64,(-5238176725660655632_i64),(-5105681036013608343_i64),2008837505591378013_i64];
_23 = _2[_9] as f32;
_27 = _8;
_19 = !false;
_2 = [3599090458_u32,696227744_u32,2967086098_u32,2531055793_u32,822572391_u32,18063275_u32];
_20 = [(-7023760684438303477_i64),(-1052201546293334109_i64),8346570881591245504_i64,8688484400913034084_i64,(-5537979651154566832_i64),(-6218642122172564910_i64),(-3958945959573311206_i64),9049621016583945159_i64];
match _20[_9] {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb6,
4 => bb8,
8688484400913034084 => bb10,
_ => bb9
}
}
bb8 = {
_6 = [(-1772545563_i32)];
Goto(bb7)
}
bb9 = {
match _3 {
103 => bb3,
_ => bb1
}
}
bb10 = {
_25 = -_18;
_18 = _25;
_26 = !_27;
_5 = -_3;
_16 = 250_u8 as f32;
_12 = core::ptr::addr_of!(_3);
_28 = _27;
_20 = [(-6359391364252887962_i64),7920136288708640425_i64,(-4629132951857773130_i64),7995881374719971935_i64,8418302579275627528_i64,(-213213604454647011_i64),(-8957821076555286533_i64),3889793374932429925_i64];
_20[_9] = (-8341_i16) as i64;
_22 = (-140330241916704125283481202912852498524_i128) - 66900264064272175619747206366612067226_i128;
_19 = true;
_26 = _8 ^ _8;
match _2[_9] {
0 => bb7,
1 => bb11,
2 => bb12,
3 => bb13,
2531055793 => bb15,
_ => bb14
}
}
bb11 = {
_9 = 2599141154_u32 as usize;
_2 = [3785199613_u32,488222468_u32,213435793_u32,1382174938_u32,2208980726_u32,2766328646_u32];
_5 = 19149705185540730798285902094136308350_u128 as i8;
_3 = _5;
_11 = core::ptr::addr_of!(_3);
_2 = [3240537582_u32,430290605_u32,3775427310_u32,1952446155_u32,3284284519_u32,1727970787_u32];
_6 = [1104808364_i32];
_12 = core::ptr::addr_of!((*_11));
RET = [_9,_9,_9,_9,_9];
_12 = Move(_11);
RET = [_9,_9,_9,_9,_9];
_2 = [2126387526_u32,2517767478_u32,2121429147_u32,1572908595_u32,606975852_u32,205673994_u32];
_13 = '\u{10caf7}';
_2 = [2432460000_u32,3125507403_u32,1096540024_u32,1505449112_u32,4115328317_u32,1425249096_u32];
RET = [_9,_9,_9,_9,_9];
_6 = [529197701_i32];
_14 = [_5,_5,_3,_5];
Call(_3 = fn10(RET, Move(_12), Move(_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_6 = [(-1772545563_i32)];
Goto(bb7)
}
bb13 = {
_19 = (*_11) < (*_11);
_14 = [(*_11),_3,(*_11),_5];
_20 = [(-723045318205316112_i64),9173023116021248503_i64,(-5907198132324528210_i64),5607646013376719356_i64,(-818547904018321219_i64),(-4023335936565564344_i64),(-6718647073889183664_i64),8703739360499988052_i64];
_13 = '\u{6626f}';
RET = [_9,_9,_9,_9,_9];
_22 = (-69290364214542726299136369209511788015_i128) | 80203815491166389913728292277649383044_i128;
_14 = [_3,(*_11),(*_11),_3];
_6 = [(-824827393_i32)];
_21 = _13;
_11 = core::ptr::addr_of!((*_11));
_13 = _21;
(*_1) = [1922662391_u32,1161949991_u32,943409548_u32,4002949436_u32,1611107034_u32,1921875017_u32];
_9 = 3_usize;
_2[_9] = !1237490591_u32;
_12 = core::ptr::addr_of!(_14[_9]);
_12 = Move(_11);
_8 = !(-9223372036854775808_isize);
_5 = 2945_u16 as i8;
_6 = [1608972543_i32];
RET[_9] = _9;
_20 = [(-5130201550637275282_i64),(-4541967243153169999_i64),(-63118780392313789_i64),4788658092519071698_i64,1789329591131350852_i64,(-5238176725660655632_i64),(-5105681036013608343_i64),2008837505591378013_i64];
_23 = _2[_9] as f32;
_27 = _8;
_19 = !false;
_2 = [3599090458_u32,696227744_u32,2967086098_u32,2531055793_u32,822572391_u32,18063275_u32];
_20 = [(-7023760684438303477_i64),(-1052201546293334109_i64),8346570881591245504_i64,8688484400913034084_i64,(-5537979651154566832_i64),(-6218642122172564910_i64),(-3958945959573311206_i64),9049621016583945159_i64];
match _20[_9] {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb6,
4 => bb8,
8688484400913034084 => bb10,
_ => bb9
}
}
bb14 = {
match _3 {
103 => bb3,
_ => bb1
}
}
bb15 = {
RET = [_9,_9,_9,_9,_9];
_28 = _26 * _27;
_29 = (-7320_i16) as f64;
_5 = _9 as i8;
_8 = _28 >> (*_1)[_9];
_12 = core::ptr::addr_of!((*_12));
RET[_9] = _9 & _9;
_12 = core::ptr::addr_of!(_14[_9]);
_18 = _25;
_31 = -_20[_9];
RET[_9] = _20[_9] as usize;
(*_1) = [3056747288_u32,976836897_u32,823544009_u32,83190361_u32,2051523372_u32,514274576_u32];
(*_1) = [3739944327_u32,3434772914_u32,1470046828_u32,3477223946_u32,3964186489_u32,2980102888_u32];
_12 = core::ptr::addr_of!((*_12));
_14 = [_5,_3,_3,_5];
_16 = _23;
_16 = _23;
_20 = [_31,_31,_31,_31,_31,_31,_31,_31];
(*_1) = [327964931_u32,2286776829_u32,791802875_u32,3448467897_u32,3121062450_u32,2991535110_u32];
Goto(bb16)
}
bb16 = {
Call(_37 = dump_var(9_usize, 27_usize, Move(_27), 20_usize, Move(_20), 22_usize, Move(_22), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(9_usize, 9_usize, Move(_9), 26_usize, Move(_26), 14_usize, Move(_14), 19_usize, Move(_19)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: [usize; 5],mut _2: *const i8,mut _3: *const [u32; 6]) -> i8 {
mir! {
type RET = i8;
let _4: f32;
let _5: *const *mut u32;
let _6: f32;
let _7: [u8; 6];
let _8: f64;
let _9: [char; 8];
let _10: *mut *mut isize;
let _11: u32;
let _12: bool;
let _13: Adt86;
let _14: i128;
let _15: isize;
let _16: u16;
let _17: *const i8;
let _18: f32;
let _19: *const *mut u32;
let _20: f64;
let _21: *mut isize;
let _22: [u32; 6];
let _23: isize;
let _24: bool;
let _25: char;
let _26: isize;
let _27: ();
let _28: ();
{
RET = 6_i8;
_1 = [1_usize,6_usize,4174066073090362473_usize,5273360280167389612_usize,4_usize];
RET = (-122_i8);
RET = 59_i8;
RET = (-9223372036854775808_isize) as i8;
_1 = [4_usize,9200646358513073469_usize,2969898140064563044_usize,0_usize,5655693072589782314_usize];
RET = (-103_i8) ^ 11_i8;
_1 = [4237300323599883600_usize,7_usize,10885032945864271829_usize,4501498475285702018_usize,1_usize];
_2 = core::ptr::addr_of!(RET);
(*_2) = 4_i8 >> 170929033475092148968621013723768761350_u128;
_1 = [0_usize,1_usize,7_usize,5_usize,12290464492740510184_usize];
(*_2) = 33434_u16 as i8;
_2 = core::ptr::addr_of!(RET);
_1 = [6_usize,5_usize,8031697048456035374_usize,10965431052334643128_usize,16648778007376507521_usize];
RET = (-75_i8);
RET = (-92_i8);
RET = (-6668_i16) as i8;
(*_2) = 31_i8 | 73_i8;
_4 = (-8996918006429800306_i64) as f32;
(*_2) = 14_i8;
match (*_2) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
14 => bb7,
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
_4 = 2962325304815599490_u64 as f32;
_6 = _4;
_4 = (-24789_i16) as f32;
_2 = core::ptr::addr_of!((*_2));
_7 = [118_u8,87_u8,42_u8,240_u8,210_u8,52_u8];
(*_2) = 6969353386364060985777296344539631008_u128 as i8;
_7 = [66_u8,27_u8,254_u8,93_u8,89_u8,208_u8];
_9 = ['\u{d8446}','\u{c2c50}','\u{aaa4f}','\u{2710f}','\u{ed2fc}','\u{f456}','\u{29b86}','\u{e7ba7}'];
RET = !(-73_i8);
_8 = 53128_u16 as f64;
(*_2) = !55_i8;
RET = -(-76_i8);
(*_2) = 103_i8;
RET = (-116_i8);
_11 = 314125166565866890005008126217079758233_u128 as u32;
(*_2) = 12_i8;
RET = !(-73_i8);
(*_2) = (-61_i8) >> _11;
RET = 22_i8;
_11 = 452755239_u32 * 2894157833_u32;
RET = (-96_i8);
RET = -56_i8;
_12 = false;
_2 = core::ptr::addr_of!((*_2));
_2 = core::ptr::addr_of!((*_2));
Goto(bb8)
}
bb8 = {
_12 = !true;
_11 = 2625985469_u32;
_2 = core::ptr::addr_of!((*_2));
_9 = ['\u{d0a32}','\u{33432}','\u{ea1b3}','\u{a7852}','\u{da0ef}','\u{d310f}','\u{f3260}','\u{ddef9}'];
RET = !(-14_i8);
_4 = -_6;
Goto(bb9)
}
bb9 = {
(*_2) = !51_i8;
_8 = _4 as f64;
_12 = true ^ true;
_6 = -_4;
_9 = ['\u{3ff29}','\u{5b6e4}','\u{b9ef6}','\u{b0418}','\u{d2ea9}','\u{c86e9}','\u{35e8a}','\u{5481c}'];
_6 = _4;
_8 = _11 as f64;
RET = !3_i8;
_9 = ['\u{1f2d0}','\u{104051}','\u{61f0c}','\u{5390c}','\u{7f7ef}','\u{f4d74}','\u{4119a}','\u{5ed4c}'];
_8 = (-615558736370783683_i64) as f64;
_14 = 2458938172301346612086012842587617453_i128;
RET = 1023444820378374962_i64 as i8;
_2 = core::ptr::addr_of!((*_2));
_14 = -75302205944174400349258499574797532516_i128;
(*_2) = (-127_i8) & 36_i8;
(*_2) = !104_i8;
_9 = ['\u{e1a43}','\u{ce7ba}','\u{221a0}','\u{c29b}','\u{80f01}','\u{28cd2}','\u{2c6fa}','\u{69ec2}'];
_9 = ['\u{b5e07}','\u{b12c7}','\u{1107e}','\u{e8daa}','\u{6bb52}','\u{b79d}','\u{cc6b4}','\u{525e0}'];
(*_2) = !13_i8;
(*_2) = (-122_i8);
_8 = _14 as f64;
_6 = _4 + _4;
_12 = !false;
_2 = core::ptr::addr_of!(RET);
_15 = 9223372036854775807_isize << _11;
_2 = core::ptr::addr_of!(RET);
RET = 41_i8;
_11 = 2859538047_u32;
Goto(bb10)
}
bb10 = {
_16 = !22735_u16;
_14 = 28286576439416544462673227180857657142_i128;
_11 = !2119048323_u32;
_17 = Move(_2);
_8 = 13054131058184485108_u64 as f64;
_7 = [47_u8,170_u8,3_u8,32_u8,48_u8,154_u8];
_11 = !3965643133_u32;
_17 = core::ptr::addr_of!(RET);
_1 = [0_usize,576673871121431532_usize,11763242968257986857_usize,1_usize,10278764619503673068_usize];
_15 = 7_usize as isize;
Goto(bb11)
}
bb11 = {
_1 = [13516319339110032869_usize,0_usize,10866581945755092955_usize,15162284325309942916_usize,7_usize];
_6 = -_4;
_4 = _6 - _6;
_1 = [11435117857857170178_usize,7_usize,446812579366953035_usize,2_usize,90453086550335071_usize];
_2 = core::ptr::addr_of!(RET);
_17 = core::ptr::addr_of!((*_17));
_14 = 2_usize as i128;
_15 = 9223372036854775807_isize;
_14 = 126461267263386161264372267257521309240_i128;
_4 = _6 * _6;
_12 = !false;
(*_2) = (-76_i8);
_21 = core::ptr::addr_of_mut!(_15);
_10 = core::ptr::addr_of_mut!(_21);
_20 = _8 - _8;
_14 = _20 as i128;
(*_17) = _16 as i8;
RET = _20 as i8;
_11 = (-5398279198813853640_i64) as u32;
_7 = [197_u8,60_u8,131_u8,30_u8,122_u8,225_u8];
_3 = core::ptr::addr_of!(_22);
match _15 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
9223372036854775807 => bb18,
_ => bb17
}
}
bb12 = {
_16 = !22735_u16;
_14 = 28286576439416544462673227180857657142_i128;
_11 = !2119048323_u32;
_17 = Move(_2);
_8 = 13054131058184485108_u64 as f64;
_7 = [47_u8,170_u8,3_u8,32_u8,48_u8,154_u8];
_11 = !3965643133_u32;
_17 = core::ptr::addr_of!(RET);
_1 = [0_usize,576673871121431532_usize,11763242968257986857_usize,1_usize,10278764619503673068_usize];
_15 = 7_usize as isize;
Goto(bb11)
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_4 = 2962325304815599490_u64 as f32;
_6 = _4;
_4 = (-24789_i16) as f32;
_2 = core::ptr::addr_of!((*_2));
_7 = [118_u8,87_u8,42_u8,240_u8,210_u8,52_u8];
(*_2) = 6969353386364060985777296344539631008_u128 as i8;
_7 = [66_u8,27_u8,254_u8,93_u8,89_u8,208_u8];
_9 = ['\u{d8446}','\u{c2c50}','\u{aaa4f}','\u{2710f}','\u{ed2fc}','\u{f456}','\u{29b86}','\u{e7ba7}'];
RET = !(-73_i8);
_8 = 53128_u16 as f64;
(*_2) = !55_i8;
RET = -(-76_i8);
(*_2) = 103_i8;
RET = (-116_i8);
_11 = 314125166565866890005008126217079758233_u128 as u32;
(*_2) = 12_i8;
RET = !(-73_i8);
(*_2) = (-61_i8) >> _11;
RET = 22_i8;
_11 = 452755239_u32 * 2894157833_u32;
RET = (-96_i8);
RET = -56_i8;
_12 = false;
_2 = core::ptr::addr_of!((*_2));
_2 = core::ptr::addr_of!((*_2));
Goto(bb8)
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
(*_2) = 103_i8;
_15 = 2270585795125250815_u64 as isize;
_1 = [11495410280532539441_usize,1891905623873415025_usize,11319997938331065936_usize,3486211254998723050_usize,12450430263084223434_usize];
(*_3) = [_11,_11,_11,_11,_11,_11];
_2 = Move(_17);
Goto(bb19)
}
bb19 = {
Call(_27 = dump_var(10_usize, 14_usize, Move(_14), 12_usize, Move(_12), 22_usize, Move(_22), 7_usize, Move(_7)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: &'static (isize,),mut _2: isize,mut _3: (isize,)) -> f32 {
mir! {
type RET = f32;
let _4: char;
let _5: *mut *const [char; 8];
let _6: *mut u8;
let _7: Adt31;
let _8: f64;
let _9: Adt64;
let _10: *mut [i32; 1];
let _11: [u8; 6];
let _12: i64;
let _13: [i64; 1];
let _14: char;
let _15: [u32; 8];
let _16: &'static isize;
let _17: &'static u128;
let _18: &'static [i16; 4];
let _19: isize;
let _20: isize;
let _21: usize;
let _22: isize;
let _23: [u8; 6];
let _24: Adt86;
let _25: (u128, i128);
let _26: i128;
let _27: f32;
let _28: i16;
let _29: u32;
let _30: *mut [i32; 1];
let _31: [i64; 5];
let _32: [u32; 6];
let _33: &'static *mut &'static u128;
let _34: *const *mut *mut isize;
let _35: u16;
let _36: ();
let _37: ();
{
_2 = _3.0 >> _3.0;
RET = (-45_i8) as f32;
RET = 157301328318799535_i64 as f32;
RET = 8395721703976759195_usize as f32;
_3.0 = !_2;
_4 = '\u{1531a}';
_1 = &_3;
_3.0 = 6654_u16 as isize;
_2 = _3.0;
_2 = _3.0 | _3.0;
_3 = (_2,);
_1 = &_3;
RET = 304479185095842550_usize as f32;
_3.0 = 13438_u16 as isize;
_2 = _3.0;
_3.0 = 681291319683744258_u64 as isize;
RET = 82477873717695666078022381445892041150_u128 as f32;
RET = 3093366581861150528085367873775791212_i128 as f32;
Goto(bb1)
}
bb1 = {
_3 = (_2,);
_1 = &_3;
_3 = (_2,);
_3 = (_2,);
_2 = _3.0;
_2 = !_3.0;
_4 = '\u{10978a}';
RET = (-1009908793_i32) as f32;
_1 = &_3;
RET = 1288835756440779309_u64 as f32;
_3 = (_2,);
_3.0 = 13792120111690949570_u64 as isize;
_2 = _3.0;
_3 = (_2,);
_3.0 = _2 >> _2;
RET = (-61429223732417125818913641245769523826_i128) as f32;
_1 = &_3;
_3.0 = _2;
RET = 6327814574900159730_i64 as f32;
RET = 179_u8 as f32;
Goto(bb2)
}
bb2 = {
RET = 157107084113794710207871255354566921528_i128 as f32;
_3 = (_2,);
_8 = _2 as f64;
_7 = Adt31::Variant3 { fld0: _8,fld1: _4,fld2: (-13060_i16),fld3: 51_u8 };
_12 = 886241472409259354_i64;
_6 = core::ptr::addr_of_mut!(place!(Field::<u8>(Variant(_7, 3), 3)));
RET = _12 as f32;
_4 = Field::<char>(Variant(_7, 3), 1);
_13 = [_12];
place!(Field::<u8>(Variant(_7, 3), 3)) = 255_u8;
place!(Field::<f64>(Variant(_7, 3), 0)) = _8 + _8;
(*_6) = 11853444133623298132_usize as u8;
RET = 14351492541523387563_u64 as f32;
_14 = _4;
_11 = [(*_6),(*_6),(*_6),(*_6),(*_6),Field::<u8>(Variant(_7, 3), 3)];
place!(Field::<char>(Variant(_7, 3), 1)) = _4;
place!(Field::<i16>(Variant(_7, 3), 2)) = -(-7833_i16);
_2 = !_3.0;
RET = 6320475189072465124_u64 as f32;
place!(Field::<char>(Variant(_7, 3), 1)) = _14;
_7 = Adt31::Variant3 { fld0: _8,fld1: _14,fld2: (-18719_i16),fld3: 46_u8 };
_11 = [166_u8,81_u8,198_u8,133_u8,227_u8,182_u8];
_4 = Field::<char>(Variant(_7, 3), 1);
Goto(bb3)
}
bb3 = {
place!(Field::<u8>(Variant(_7, 3), 3)) = 227_u8 - 102_u8;
_6 = core::ptr::addr_of_mut!(place!(Field::<u8>(Variant(_7, 3), 3)));
_15 = [1805952192_u32,2919726054_u32,3424769908_u32,2392147145_u32,3190122800_u32,3268889124_u32,68320318_u32,3769190429_u32];
_8 = Field::<f64>(Variant(_7, 3), 0);
place!(Field::<f64>(Variant(_7, 3), 0)) = _8 - _8;
_15 = [3159810055_u32,805347684_u32,2657578303_u32,1627553843_u32,3804213868_u32,2593525839_u32,1017223703_u32,1953476226_u32];
_1 = &_3;
_14 = _4;
_6 = core::ptr::addr_of_mut!(place!(Field::<u8>(Variant(_7, 3), 3)));
_1 = &_3;
_14 = Field::<char>(Variant(_7, 3), 1);
place!(Field::<f64>(Variant(_7, 3), 0)) = _8 - _8;
place!(Field::<char>(Variant(_7, 3), 1)) = _14;
_15 = [3631345348_u32,73166423_u32,599778168_u32,3721369651_u32,1069381152_u32,1482105525_u32,4241114568_u32,2556377358_u32];
(*_6) = 5_u8;
place!(Field::<u8>(Variant(_7, 3), 3)) = 217_u8 << _12;
Goto(bb4)
}
bb4 = {
_3.0 = -_2;
_6 = core::ptr::addr_of_mut!((*_6));
place!(Field::<char>(Variant(_7, 3), 1)) = _14;
_12 = 23243_u16 as i64;
place!(Field::<i16>(Variant(_7, 3), 2)) = !(-5365_i16);
place!(Field::<i16>(Variant(_7, 3), 2)) = 22408_i16;
_12 = 2898788467434094653_i64;
RET = 232853022187464958_u64 as f32;
RET = (-71_i8) as f32;
_3.0 = RET as isize;
_2 = RET as isize;
_16 = &_2;
_16 = &_3.0;
place!(Field::<char>(Variant(_7, 3), 1)) = _14;
_8 = Field::<f64>(Variant(_7, 3), 0) + Field::<f64>(Variant(_7, 3), 0);
_19 = !(*_16);
RET = (-19_i8) as f32;
_12 = !1684164130373685969_i64;
SetDiscriminant(_7, 2);
place!(Field::<i16>(Variant(_7, 2), 4)) = 11722_i16;
_1 = &_3;
place!(Field::<u8>(Variant(_7, 2), 2)) = 7418_u16 as u8;
_1 = &(*_1);
match Field::<i16>(Variant(_7, 2), 4) {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
11722 => bb11,
_ => bb10
}
}
bb5 = {
place!(Field::<u8>(Variant(_7, 3), 3)) = 227_u8 - 102_u8;
_6 = core::ptr::addr_of_mut!(place!(Field::<u8>(Variant(_7, 3), 3)));
_15 = [1805952192_u32,2919726054_u32,3424769908_u32,2392147145_u32,3190122800_u32,3268889124_u32,68320318_u32,3769190429_u32];
_8 = Field::<f64>(Variant(_7, 3), 0);
place!(Field::<f64>(Variant(_7, 3), 0)) = _8 - _8;
_15 = [3159810055_u32,805347684_u32,2657578303_u32,1627553843_u32,3804213868_u32,2593525839_u32,1017223703_u32,1953476226_u32];
_1 = &_3;
_14 = _4;
_6 = core::ptr::addr_of_mut!(place!(Field::<u8>(Variant(_7, 3), 3)));
_1 = &_3;
_14 = Field::<char>(Variant(_7, 3), 1);
place!(Field::<f64>(Variant(_7, 3), 0)) = _8 - _8;
place!(Field::<char>(Variant(_7, 3), 1)) = _14;
_15 = [3631345348_u32,73166423_u32,599778168_u32,3721369651_u32,1069381152_u32,1482105525_u32,4241114568_u32,2556377358_u32];
(*_6) = 5_u8;
place!(Field::<u8>(Variant(_7, 3), 3)) = 217_u8 << _12;
Goto(bb4)
}
bb6 = {
RET = 157107084113794710207871255354566921528_i128 as f32;
_3 = (_2,);
_8 = _2 as f64;
_7 = Adt31::Variant3 { fld0: _8,fld1: _4,fld2: (-13060_i16),fld3: 51_u8 };
_12 = 886241472409259354_i64;
_6 = core::ptr::addr_of_mut!(place!(Field::<u8>(Variant(_7, 3), 3)));
RET = _12 as f32;
_4 = Field::<char>(Variant(_7, 3), 1);
_13 = [_12];
place!(Field::<u8>(Variant(_7, 3), 3)) = 255_u8;
place!(Field::<f64>(Variant(_7, 3), 0)) = _8 + _8;
(*_6) = 11853444133623298132_usize as u8;
RET = 14351492541523387563_u64 as f32;
_14 = _4;
_11 = [(*_6),(*_6),(*_6),(*_6),(*_6),Field::<u8>(Variant(_7, 3), 3)];
place!(Field::<char>(Variant(_7, 3), 1)) = _4;
place!(Field::<i16>(Variant(_7, 3), 2)) = -(-7833_i16);
_2 = !_3.0;
RET = 6320475189072465124_u64 as f32;
place!(Field::<char>(Variant(_7, 3), 1)) = _14;
_7 = Adt31::Variant3 { fld0: _8,fld1: _14,fld2: (-18719_i16),fld3: 46_u8 };
_11 = [166_u8,81_u8,198_u8,133_u8,227_u8,182_u8];
_4 = Field::<char>(Variant(_7, 3), 1);
Goto(bb3)
}
bb7 = {
_3 = (_2,);
_1 = &_3;
_3 = (_2,);
_3 = (_2,);
_2 = _3.0;
_2 = !_3.0;
_4 = '\u{10978a}';
RET = (-1009908793_i32) as f32;
_1 = &_3;
RET = 1288835756440779309_u64 as f32;
_3 = (_2,);
_3.0 = 13792120111690949570_u64 as isize;
_2 = _3.0;
_3 = (_2,);
_3.0 = _2 >> _2;
RET = (-61429223732417125818913641245769523826_i128) as f32;
_1 = &_3;
_3.0 = _2;
RET = 6327814574900159730_i64 as f32;
RET = 179_u8 as f32;
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
_16 = &_20;
RET = 6405735935174687920_usize as f32;
place!(Field::<[i16; 4]>(Variant(_7, 2), 6)) = [Field::<i16>(Variant(_7, 2), 4),Field::<i16>(Variant(_7, 2), 4),Field::<i16>(Variant(_7, 2), 4),Field::<i16>(Variant(_7, 2), 4)];
_1 = &(*_1);
place!(Field::<[i16; 4]>(Variant(_7, 2), 6)) = [Field::<i16>(Variant(_7, 2), 4),Field::<i16>(Variant(_7, 2), 4),Field::<i16>(Variant(_7, 2), 4),Field::<i16>(Variant(_7, 2), 4)];
_23 = [Field::<u8>(Variant(_7, 2), 2),Field::<u8>(Variant(_7, 2), 2),Field::<u8>(Variant(_7, 2), 2),Field::<u8>(Variant(_7, 2), 2),Field::<u8>(Variant(_7, 2), 2),Field::<u8>(Variant(_7, 2), 2)];
_21 = !3_usize;
place!(Field::<[i16; 4]>(Variant(_7, 2), 6)) = [Field::<i16>(Variant(_7, 2), 4),Field::<i16>(Variant(_7, 2), 4),Field::<i16>(Variant(_7, 2), 4),Field::<i16>(Variant(_7, 2), 4)];
_7 = Adt31::Variant1 { fld0: _12,fld1: _4,fld2: 100_u8,fld3: _15,fld4: 64499_u16,fld5: 815570317_i32 };
_25.1 = (-31193704540811882411487193358057549344_i128) & (-127334877781355588278149499735659307078_i128);
place!(Field::<char>(Variant(_7, 1), 1)) = _14;
place!(Field::<i64>(Variant(_7, 1), 0)) = _12 * _12;
place!(Field::<char>(Variant(_7, 1), 1)) = _4;
_25 = (249169809214786731997246237544581820724_u128, 43362141274344046878125664725994932096_i128);
place!(Field::<u16>(Variant(_7, 1), 4)) = 58802_u16;
_1 = &(*_1);
_23 = [106_u8,92_u8,27_u8,77_u8,206_u8,184_u8];
place!(Field::<u8>(Variant(_7, 1), 2)) = 4221091226_u32 as u8;
_7 = Adt31::Variant3 { fld0: _8,fld1: _4,fld2: 14351_i16,fld3: 77_u8 };
place!(Field::<i16>(Variant(_7, 3), 2)) = _12 as i16;
Goto(bb12)
}
bb12 = {
_12 = 3967579450_u32 as i64;
_3 = (_2,);
_7 = Adt31::Variant1 { fld0: _12,fld1: _14,fld2: 75_u8,fld3: _15,fld4: 46469_u16,fld5: (-1075702902_i32) };
_17 = &_25.0;
_11 = [170_u8,70_u8,52_u8,232_u8,107_u8,233_u8];
_16 = &_20;
_20 = _21 as isize;
_11 = [200_u8,62_u8,180_u8,242_u8,85_u8,38_u8];
_27 = RET;
_25.0 = !275381167305140550729445668908346384123_u128;
_31 = [_12,Field::<i64>(Variant(_7, 1), 0),Field::<i64>(Variant(_7, 1), 0),_12,_12];
_15 = Field::<[u32; 8]>(Variant(_7, 1), 3);
_15 = [2571354233_u32,2421536552_u32,1352966536_u32,2730153892_u32,2956116586_u32,3197871877_u32,3673768624_u32,3978297413_u32];
RET = _27 + _27;
_29 = 572794572_u32;
RET = _27 + _27;
RET = -_27;
_23 = _11;
match _29 {
0 => bb5,
1 => bb13,
572794572 => bb15,
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
_31 = [Field::<i64>(Variant(_7, 1), 0),Field::<i64>(Variant(_7, 1), 0),_12,_12,_12];
place!(Field::<u8>(Variant(_7, 1), 2)) = 239_u8;
_21 = _8 as usize;
_1 = &_3;
place!(Field::<u16>(Variant(_7, 1), 4)) = (-114_i8) as u16;
Goto(bb16)
}
bb16 = {
Call(_36 = dump_var(11_usize, 25_usize, Move(_25), 15_usize, Move(_15), 29_usize, Move(_29), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(11_usize, 11_usize, Move(_11), 19_usize, Move(_19), 23_usize, Move(_23), 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: i32) -> [i32; 1] {
mir! {
type RET = [i32; 1];
let _2: f32;
let _3: Adt57;
let _4: *mut [i32; 1];
let _5: f32;
let _6: f64;
let _7: bool;
let _8: char;
let _9: u16;
let _10: Adt86;
let _11: Adt82;
let _12: isize;
let _13: (&'static i64, (u64, i16));
let _14: bool;
let _15: &'static &'static *mut u8;
let _16: *mut *mut u8;
let _17: isize;
let _18: (u128, i128);
let _19: char;
let _20: u16;
let _21: isize;
let _22: i16;
let _23: [usize; 7];
let _24: (&'static i64, (u64, i16));
let _25: [i32; 3];
let _26: bool;
let _27: [u16; 4];
let _28: *mut u32;
let _29: *mut Adt76;
let _30: char;
let _31: Adt86;
let _32: [u32; 3];
let _33: char;
let _34: char;
let _35: bool;
let _36: bool;
let _37: ();
let _38: ();
{
RET = [_1];
_1 = 9768363349626310903870283488121095135_i128 as i32;
RET = [_1];
RET = [_1];
_1 = (-1631069909_i32);
RET = [_1];
_1 = 578168773_i32 >> 1092881759405678116_i64;
_2 = 13522396459134865871_u64 as f32;
_2 = 3403263263397497237_i64 as f32;
RET = [_1];
RET = [_1];
RET = [_1];
_1 = 252054998346749007604790816718499405127_u128 as i32;
RET = [_1];
_1 = -(-572683578_i32);
_2 = 4_u8 as f32;
RET = [_1];
_4 = core::ptr::addr_of_mut!(RET);
(*_4) = [_1];
RET = [_1];
Goto(bb1)
}
bb1 = {
(*_4) = [_1];
_4 = core::ptr::addr_of_mut!(RET);
RET = [_1];
_4 = core::ptr::addr_of_mut!((*_4));
(*_4) = [_1];
(*_4) = [_1];
(*_4) = [_1];
_6 = 63859999964298949095022233543337277032_u128 as f64;
Goto(bb2)
}
bb2 = {
_5 = _2;
(*_4) = [_1];
RET = [_1];
_1 = (-473369333_i32);
_5 = -_2;
_4 = core::ptr::addr_of_mut!((*_4));
_1 = -130757176_i32;
RET = [_1];
(*_4) = [_1];
_7 = !false;
_5 = -_2;
_5 = _2;
RET = [_1];
_7 = false;
Goto(bb3)
}
bb3 = {
_8 = '\u{5905e}';
_7 = false;
_5 = -_2;
_7 = false;
RET = [_1];
(*_4) = [_1];
RET = [_1];
_4 = core::ptr::addr_of_mut!((*_4));
(*_4) = [_1];
_7 = !true;
_8 = '\u{7cb80}';
RET = [_1];
(*_4) = [_1];
(*_4) = [_1];
RET = [_1];
_1 = 591387144_i32 & (-612904603_i32);
_7 = false;
_8 = '\u{42533}';
RET = [_1];
_6 = 5_usize as f64;
(*_4) = [_1];
_11.fld1 = [3891258433_u32,1062384614_u32,2034404004_u32,926535789_u32,1477847550_u32,1020604226_u32];
_11.fld4 = Move(_4);
Goto(bb4)
}
bb4 = {
_6 = 12357066172060440787_usize as f64;
_4 = core::ptr::addr_of_mut!(RET);
_12 = 4009088983_u32 as isize;
_7 = _6 == _6;
_11.fld2 = [20254_i16,(-5168_i16),(-28977_i16),9151_i16];
_13.1.1 = 25929_i16 - 19396_i16;
_11.fld1 = [2878716095_u32,2618107315_u32,3552379135_u32,2445670043_u32,860810975_u32,3463868223_u32];
RET = [_1];
_13.1.1 = !11490_i16;
_13.1.0 = 13545544829512142454_u64 * 9438599462400878667_u64;
_13.1.0 = 103246831039000139018623565532636757068_u128 as u64;
_13.1.0 = 371784226997278970_u64 | 14703446939649758041_u64;
_14 = !_7;
_6 = _13.1.1 as f64;
_2 = _13.1.0 as f32;
_11.fld0 = [10606513516087839708_usize,1_usize,4_usize,989710914788050133_usize,5_usize];
_11.fld4 = core::ptr::addr_of_mut!(RET);
_8 = '\u{e1e8c}';
_14 = !_7;
(*_4) = [_1];
_11.fld4 = core::ptr::addr_of_mut!((*_4));
Goto(bb5)
}
bb5 = {
_13.1 = (6381102234513275502_u64, (-5248_i16));
_5 = 55_u8 as f32;
_11.fld0 = [13135319665550488174_usize,9159235060555546903_usize,14547390356270488629_usize,4890361653383219248_usize,6_usize];
_18.1 = _12 as i128;
_11.fld4 = core::ptr::addr_of_mut!((*_4));
_11.fld4 = core::ptr::addr_of_mut!((*_4));
Goto(bb6)
}
bb6 = {
_11.fld4 = Move(_4);
_11.fld4 = core::ptr::addr_of_mut!(RET);
_19 = _8;
_18.0 = !147919917466154924333288823484521848584_u128;
_6 = 5_usize as f64;
_11.fld1 = [3045493308_u32,387508894_u32,985972615_u32,2491842301_u32,1592429279_u32,3590692058_u32];
_11.fld4 = core::ptr::addr_of_mut!(RET);
_17 = _12;
_4 = core::ptr::addr_of_mut!(RET);
_13.1.0 = _6 as u64;
_13.1 = (17321384649244132993_u64, 16864_i16);
_11.fld2 = [_13.1.1,_13.1.1,_13.1.1,_13.1.1];
_13.1.0 = 1610278124761778030_u64 & 10915445702200531348_u64;
Goto(bb7)
}
bb7 = {
_18.0 = !176147778903220228350564281532762444562_u128;
_19 = _8;
_18.0 = _2 as u128;
_20 = 30763_u16;
_13.1.1 = 24525_i16 >> _18.1;
_8 = _19;
_23 = [6_usize,7_usize,3_usize,5688917369408847211_usize,16788411783191112659_usize,4558068940179977316_usize,15723612787257221586_usize];
_11.fld0 = [3847944502046087522_usize,2_usize,5_usize,15852146460266657462_usize,3_usize];
_7 = _14;
_20 = _18.0 as u16;
Call(_6 = fn13((*_4), _11.fld1, _23, _13.1.1, _18.0, _11.fld0, Move(_4), Move(_11.fld4), _12), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_5 = _2 * _2;
_11.fld1 = [3564319441_u32,2108211057_u32,1882188963_u32,2762294242_u32,1174967847_u32,343010691_u32];
_13.1 = (18323466414142317003_u64, 1849_i16);
_13.1.1 = (-7816_i16);
_24.1.0 = _18.0 as u64;
_23 = [14338918926675420219_usize,3284389693622863130_usize,4655570807096509038_usize,15901105888822681249_usize,14998551222475569110_usize,4_usize,4822768640912555886_usize];
_11.fld0 = [7380209925453573711_usize,15499900744915626748_usize,4_usize,7_usize,1_usize];
_14 = _7;
_6 = (-80_i8) as f64;
_24.1 = _13.1;
_11.fld1 = [3499667115_u32,285168087_u32,3020750875_u32,2028133487_u32,156889442_u32,2721041475_u32];
RET = [_1];
_24.1.0 = _13.1.0;
_26 = !_14;
_23 = [7_usize,15989342186073938171_usize,16376388649834800358_usize,4_usize,7_usize,5_usize,5_usize];
_18 = (8378715025223109965947089842913891178_u128, (-93867968784965214536276162658153424010_i128));
_20 = !59545_u16;
match _13.1.0 {
0 => bb9,
1 => bb10,
18323466414142317003 => bb12,
_ => bb11
}
}
bb9 = {
_5 = _2;
(*_4) = [_1];
RET = [_1];
_1 = (-473369333_i32);
_5 = -_2;
_4 = core::ptr::addr_of_mut!((*_4));
_1 = -130757176_i32;
RET = [_1];
(*_4) = [_1];
_7 = !false;
_5 = -_2;
_5 = _2;
RET = [_1];
_7 = false;
Goto(bb3)
}
bb10 = {
_11.fld4 = Move(_4);
_11.fld4 = core::ptr::addr_of_mut!(RET);
_19 = _8;
_18.0 = !147919917466154924333288823484521848584_u128;
_6 = 5_usize as f64;
_11.fld1 = [3045493308_u32,387508894_u32,985972615_u32,2491842301_u32,1592429279_u32,3590692058_u32];
_11.fld4 = core::ptr::addr_of_mut!(RET);
_17 = _12;
_4 = core::ptr::addr_of_mut!(RET);
_13.1.0 = _6 as u64;
_13.1 = (17321384649244132993_u64, 16864_i16);
_11.fld2 = [_13.1.1,_13.1.1,_13.1.1,_13.1.1];
_13.1.0 = 1610278124761778030_u64 & 10915445702200531348_u64;
Goto(bb7)
}
bb11 = {
_13.1 = (6381102234513275502_u64, (-5248_i16));
_5 = 55_u8 as f32;
_11.fld0 = [13135319665550488174_usize,9159235060555546903_usize,14547390356270488629_usize,4890361653383219248_usize,6_usize];
_18.1 = _12 as i128;
_11.fld4 = core::ptr::addr_of_mut!((*_4));
_11.fld4 = core::ptr::addr_of_mut!((*_4));
Goto(bb6)
}
bb12 = {
_26 = _7;
_9 = _20;
RET = [_1];
_22 = 3770758681_u32 as i16;
_9 = 1862061830796724626_i64 as u16;
_13.1 = (_24.1.0, _22);
_1 = _13.1.0 as i32;
_21 = -_17;
_26 = _6 > _6;
_23 = [11182104976019285309_usize,4_usize,0_usize,12004384010593894768_usize,1_usize,4_usize,7_usize];
RET = [_1];
_7 = _1 == _1;
match _18.1 {
0 => bb11,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb10,
5 => bb9,
6 => bb7,
246414398135973248927098444773614787446 => bb14,
_ => bb13
}
}
bb13 = {
_13.1 = (6381102234513275502_u64, (-5248_i16));
_5 = 55_u8 as f32;
_11.fld0 = [13135319665550488174_usize,9159235060555546903_usize,14547390356270488629_usize,4890361653383219248_usize,6_usize];
_18.1 = _12 as i128;
_11.fld4 = core::ptr::addr_of_mut!((*_4));
_11.fld4 = core::ptr::addr_of_mut!((*_4));
Goto(bb6)
}
bb14 = {
_25 = [_1,_1,_1];
_13.1.0 = _24.1.0;
RET = [_1];
_11.fld4 = core::ptr::addr_of_mut!(RET);
_5 = _18.0 as f32;
_17 = _21;
_18.1 = 15305584404332762383_usize as i128;
_21 = _12;
_27 = [_20,_20,_20,_9];
_11.fld4 = core::ptr::addr_of_mut!(RET);
_21 = _12;
_30 = _8;
_11.fld2 = [_24.1.1,_22,_24.1.1,_24.1.1];
_4 = Move(_11.fld4);
_7 = !_14;
_23 = [5473252709143754965_usize,4_usize,1_usize,7_usize,17608887647343086147_usize,14131935007530679428_usize,12271438170115357743_usize];
RET = [_1];
_18.0 = _13.1.1 as u128;
_2 = _5 * _5;
_6 = _20 as f64;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(12_usize, 18_usize, Move(_18), 22_usize, Move(_22), 8_usize, Move(_8), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(12_usize, 12_usize, Move(_12), 9_usize, Move(_9), 25_usize, Move(_25), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [i32; 1],mut _2: [u32; 6],mut _3: [usize; 7],mut _4: i16,mut _5: u128,mut _6: [usize; 5],mut _7: *mut [i32; 1],mut _8: *mut [i32; 1],mut _9: isize) -> f64 {
mir! {
type RET = f64;
let _10: isize;
let _11: &'static &'static &'static *mut u8;
let _12: char;
let _13: usize;
let _14: i16;
let _15: u32;
let _16: u64;
let _17: f64;
let _18: u8;
let _19: i16;
let _20: (u128,);
let _21: Adt31;
let _22: *const [char; 8];
let _23: i32;
let _24: i64;
let _25: ([i64; 8], *mut *const [char; 8], [u32; 6], [u16; 4]);
let _26: *mut &'static i32;
let _27: u64;
let _28: u32;
let _29: [usize; 5];
let _30: char;
let _31: &'static *mut &'static u128;
let _32: [u8; 6];
let _33: isize;
let _34: u128;
let _35: *const i8;
let _36: Adt44;
let _37: i64;
let _38: [char; 8];
let _39: f64;
let _40: [u8; 6];
let _41: ([i64; 8], *mut *const [char; 8], [u32; 6], [u16; 4]);
let _42: isize;
let _43: Adt44;
let _44: f64;
let _45: [u8; 6];
let _46: ();
let _47: ();
{
_6 = [5_usize,0_usize,10941298553452302158_usize,12142405197521950601_usize,0_usize];
_5 = !172360798847161854542996837206760534406_u128;
RET = (-4283148485142340698_i64) as f64;
_2 = [2187146849_u32,1368239000_u32,2857365083_u32,2109845804_u32,1212316386_u32,3728857368_u32];
_8 = core::ptr::addr_of_mut!(_1);
(*_8) = [1400345384_i32];
_1 = [630298030_i32];
_2 = [1723403889_u32,1793523109_u32,283987713_u32,2390592941_u32,3789625048_u32,3482002870_u32];
(*_8) = [(-220888643_i32)];
_10 = _9 | _9;
_5 = !299958362427072972528512880482405816446_u128;
_2 = [3690438399_u32,1299509036_u32,2636749929_u32,643115389_u32,2178304056_u32,2411070009_u32];
(*_8) = [(-1816844083_i32)];
_13 = 7097148004270495700_usize + 2_usize;
_10 = true as isize;
_13 = 10728019629113089312_usize >> _10;
_7 = core::ptr::addr_of_mut!(_1);
_13 = 11528152542366926997_usize;
_1 = [(-78276464_i32)];
(*_7) = [(-2121732659_i32)];
_13 = !1_usize;
(*_8) = [(-1111001664_i32)];
_4 = !(-10682_i16);
_12 = '\u{fb0da}';
Goto(bb1)
}
bb1 = {
_14 = _4 ^ _4;
_6 = [_13,_13,_13,_13,_13];
_3 = [_13,_13,_13,_13,_13,_13,_13];
_4 = _9 as i16;
_6 = [_13,_13,_13,_13,_13];
_1 = [(-2006647347_i32)];
_17 = RET;
(*_7) = [332314527_i32];
_4 = !_14;
_1 = [1123967116_i32];
_3 = [_13,_13,_13,_13,_13,_13,_13];
_5 = 235737624108767994663084742628975793980_u128 * 272800631455942415305706700079625138729_u128;
_12 = '\u{b4c3b}';
_4 = -_14;
RET = (-38162837121372852720568132754827735692_i128) as f64;
_18 = !245_u8;
Call(_9 = fn14(_12, _5, _12, _2, _10, _4, _2, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_15 = RET as u32;
(*_8) = [1409785087_i32];
(*_7) = [903838935_i32];
_20 = (_5,);
_16 = (-11293011774069540851324080908744829301_i128) as u64;
(*_7) = [(-1362136840_i32)];
_2 = [_15,_15,_15,_15,_15,_15];
_13 = _14 as usize;
Goto(bb3)
}
bb3 = {
_19 = _4 * _14;
(*_8) = [(-1689434281_i32)];
RET = -_17;
_9 = _10;
_7 = Move(_8);
_10 = _12 as isize;
_1 = [(-1287587613_i32)];
_5 = _20.0 >> _19;
_18 = (-29541875051296014882437259597418763501_i128) as u8;
_12 = '\u{1c255}';
Goto(bb4)
}
bb4 = {
_25.2 = [_15,_15,_15,_15,_15,_15];
_6 = [_13,_13,_13,_13,_13];
Goto(bb5)
}
bb5 = {
_20 = (_5,);
_23 = !(-965825055_i32);
_19 = !_4;
_4 = _14;
_14 = !_19;
_27 = _16;
_10 = _16 as isize;
_19 = _14;
_4 = _15 as i16;
_3 = [_13,_13,_13,_13,_13,_13,_13];
RET = _17;
RET = _16 as f64;
_10 = 1674268767640677437_i64 as isize;
_25.1 = core::ptr::addr_of_mut!(_22);
_24 = RET as i64;
_12 = '\u{e8f39}';
_7 = core::ptr::addr_of_mut!(_1);
_23 = (-668709941_i32);
Goto(bb6)
}
bb6 = {
_7 = core::ptr::addr_of_mut!((*_7));
_2 = [_15,_15,_15,_15,_15,_15];
_27 = 4_i8 as u64;
RET = _27 as f64;
RET = _17 * _17;
(*_7) = [_23];
_10 = _12 as isize;
_24 = (-4763327624849147797_i64);
_16 = !_27;
_29 = _6;
_27 = _13 as u64;
_21 = Adt31::Variant3 { fld0: RET,fld1: _12,fld2: _19,fld3: _18 };
SetDiscriminant(_21, 2);
_6 = _29;
_25.3 = [54074_u16,56985_u16,10120_u16,7843_u16];
_29 = _6;
place!(Field::<bool>(Variant(_21, 2), 0)) = true;
place!(Field::<u8>(Variant(_21, 2), 2)) = !_18;
_25.1 = core::ptr::addr_of_mut!(_22);
place!(Field::<bool>(Variant(_21, 2), 0)) = true;
_18 = _10 as u8;
place!(Field::<char>(Variant(_21, 2), 1)) = _12;
_18 = Field::<u8>(Variant(_21, 2), 2) & Field::<u8>(Variant(_21, 2), 2);
_28 = _15;
place!(Field::<usize>(Variant(_21, 2), 3)) = !_13;
_17 = -RET;
_25.0 = [_24,_24,_24,_24,_24,_24,_24,_24];
Goto(bb7)
}
bb7 = {
_10 = _9;
_4 = !_19;
_33 = -_9;
RET = _15 as f64;
place!(Field::<i16>(Variant(_21, 2), 4)) = _4 * _14;
_12 = Field::<char>(Variant(_21, 2), 1);
_23 = (-2000081160_i32);
_12 = Field::<char>(Variant(_21, 2), 1);
_28 = Field::<i16>(Variant(_21, 2), 4) as u32;
_25.2 = [_28,_28,_28,_28,_28,_28];
_30 = Field::<char>(Variant(_21, 2), 1);
_32 = [Field::<u8>(Variant(_21, 2), 2),Field::<u8>(Variant(_21, 2), 2),_18,_18,_18,Field::<u8>(Variant(_21, 2), 2)];
_12 = Field::<char>(Variant(_21, 2), 1);
(*_7) = [_23];
_25.1 = core::ptr::addr_of_mut!(_22);
_8 = Move(_7);
_25.3 = [25155_u16,12267_u16,10650_u16,2171_u16];
_7 = core::ptr::addr_of_mut!(_1);
Goto(bb8)
}
bb8 = {
_6 = [Field::<usize>(Variant(_21, 2), 3),_13,_13,_13,Field::<usize>(Variant(_21, 2), 3)];
_21 = Adt31::Variant3 { fld0: _17,fld1: _30,fld2: _4,fld3: _18 };
_32 = [Field::<u8>(Variant(_21, 3), 3),_18,Field::<u8>(Variant(_21, 3), 3),_18,_18,_18];
SetDiscriminant(_21, 3);
_27 = _16;
_16 = !_27;
_34 = _12 as u128;
_25.2 = [_28,_28,_28,_28,_28,_28];
_16 = 149094436045776939572902893892460992638_i128 as u64;
_25.0 = [_24,_24,_24,_24,_24,_24,_24,_24];
_33 = RET as isize;
(*_7) = [_23];
_33 = !_9;
place!(Field::<i16>(Variant(_21, 3), 2)) = _19;
_10 = -_33;
_34 = !_5;
_2 = [_28,_28,_28,_28,_28,_28];
RET = _27 as f64;
_25.1 = core::ptr::addr_of_mut!(_22);
_24 = 40408_u16 as i64;
_29 = _6;
_37 = -_24;
place!(Field::<u8>(Variant(_21, 3), 3)) = _24 as u8;
_34 = _20.0 >> _28;
_34 = _5;
_1 = [_23];
Goto(bb9)
}
bb9 = {
(*_7) = [_23];
_20 = (_34,);
_19 = Field::<i16>(Variant(_21, 3), 2);
_30 = _12;
_7 = core::ptr::addr_of_mut!((*_7));
Call(_7 = fn18(_9), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_3 = [_13,_13,_13,_13,_13,_13,_13];
_37 = _18 as i64;
_1 = [_23];
_25.3 = [27756_u16,20609_u16,2247_u16,26279_u16];
_28 = _15;
_7 = Move(_8);
place!(Field::<f64>(Variant(_21, 3), 0)) = _17;
_38 = [_12,_12,_30,_30,_12,_30,_30,_30];
place!(Field::<i16>(Variant(_21, 3), 2)) = _14 >> _14;
_39 = Field::<f64>(Variant(_21, 3), 0);
_22 = core::ptr::addr_of!(_38);
Goto(bb11)
}
bb11 = {
_25.0 = [_37,_24,_24,_37,_24,_24,_37,_37];
place!(Field::<f64>(Variant(_21, 3), 0)) = RET + _39;
_2 = [_28,_28,_15,_15,_15,_15];
RET = _39 * _39;
_36 = Adt44::Variant1 { fld0: _23,fld1: _27 };
place!(Field::<i16>(Variant(_21, 3), 2)) = -_14;
_25.0 = [_37,_37,_24,_24,_37,_37,_37,_37];
match _23 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb9,
4 => bb8,
5 => bb12,
6 => bb13,
340282366920938463463374607429768130296 => bb15,
_ => bb14
}
}
bb12 = {
_7 = core::ptr::addr_of_mut!((*_7));
_2 = [_15,_15,_15,_15,_15,_15];
_27 = 4_i8 as u64;
RET = _27 as f64;
RET = _17 * _17;
(*_7) = [_23];
_10 = _12 as isize;
_24 = (-4763327624849147797_i64);
_16 = !_27;
_29 = _6;
_27 = _13 as u64;
_21 = Adt31::Variant3 { fld0: RET,fld1: _12,fld2: _19,fld3: _18 };
SetDiscriminant(_21, 2);
_6 = _29;
_25.3 = [54074_u16,56985_u16,10120_u16,7843_u16];
_29 = _6;
place!(Field::<bool>(Variant(_21, 2), 0)) = true;
place!(Field::<u8>(Variant(_21, 2), 2)) = !_18;
_25.1 = core::ptr::addr_of_mut!(_22);
place!(Field::<bool>(Variant(_21, 2), 0)) = true;
_18 = _10 as u8;
place!(Field::<char>(Variant(_21, 2), 1)) = _12;
_18 = Field::<u8>(Variant(_21, 2), 2) & Field::<u8>(Variant(_21, 2), 2);
_28 = _15;
place!(Field::<usize>(Variant(_21, 2), 3)) = !_13;
_17 = -RET;
_25.0 = [_24,_24,_24,_24,_24,_24,_24,_24];
Goto(bb7)
}
bb13 = {
_14 = _4 ^ _4;
_6 = [_13,_13,_13,_13,_13];
_3 = [_13,_13,_13,_13,_13,_13,_13];
_4 = _9 as i16;
_6 = [_13,_13,_13,_13,_13];
_1 = [(-2006647347_i32)];
_17 = RET;
(*_7) = [332314527_i32];
_4 = !_14;
_1 = [1123967116_i32];
_3 = [_13,_13,_13,_13,_13,_13,_13];
_5 = 235737624108767994663084742628975793980_u128 * 272800631455942415305706700079625138729_u128;
_12 = '\u{b4c3b}';
_4 = -_14;
RET = (-38162837121372852720568132754827735692_i128) as f64;
_18 = !245_u8;
Call(_9 = fn14(_12, _5, _12, _2, _10, _4, _2, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_6 = [Field::<usize>(Variant(_21, 2), 3),_13,_13,_13,Field::<usize>(Variant(_21, 2), 3)];
_21 = Adt31::Variant3 { fld0: _17,fld1: _30,fld2: _4,fld3: _18 };
_32 = [Field::<u8>(Variant(_21, 3), 3),_18,Field::<u8>(Variant(_21, 3), 3),_18,_18,_18];
SetDiscriminant(_21, 3);
_27 = _16;
_16 = !_27;
_34 = _12 as u128;
_25.2 = [_28,_28,_28,_28,_28,_28];
_16 = 149094436045776939572902893892460992638_i128 as u64;
_25.0 = [_24,_24,_24,_24,_24,_24,_24,_24];
_33 = RET as isize;
(*_7) = [_23];
_33 = !_9;
place!(Field::<i16>(Variant(_21, 3), 2)) = _19;
_10 = -_33;
_34 = !_5;
_2 = [_28,_28,_28,_28,_28,_28];
RET = _27 as f64;
_25.1 = core::ptr::addr_of_mut!(_22);
_24 = 40408_u16 as i64;
_29 = _6;
_37 = -_24;
place!(Field::<u8>(Variant(_21, 3), 3)) = _24 as u8;
_34 = _20.0 >> _28;
_34 = _5;
_1 = [_23];
Goto(bb9)
}
bb15 = {
_9 = -_33;
_25.0 = [_37,_37,_24,_37,_37,_37,_37,_37];
place!(Field::<char>(Variant(_21, 3), 1)) = _30;
_25.3 = [8550_u16,38127_u16,42149_u16,62160_u16];
_32 = [_18,_18,_18,Field::<u8>(Variant(_21, 3), 3),_18,Field::<u8>(Variant(_21, 3), 3)];
SetDiscriminant(_21, 3);
_39 = RET - RET;
_8 = Move(_7);
_25.0 = [_37,_24,_37,_37,_37,_37,_37,_37];
RET = _39 * _39;
_25.3 = [46020_u16,35852_u16,38768_u16,51852_u16];
_42 = _24 as isize;
_27 = !_16;
_40 = [_18,_18,_18,_18,_18,_18];
_41.0 = [_24,_37,_24,_37,_37,_37,_24,_37];
_24 = 124_i8 as i64;
_43 = Adt44::Variant1 { fld0: Field::<i32>(Variant(_36, 1), 0),fld1: Field::<u64>(Variant(_36, 1), 1) };
_6 = [_13,_13,_13,_13,_13];
place!(Field::<i32>(Variant(_43, 1), 0)) = Field::<i32>(Variant(_36, 1), 0) ^ _23;
_16 = Field::<u64>(Variant(_43, 1), 1);
SetDiscriminant(_36, 0);
_12 = _30;
_36 = Move(_43);
place!(Field::<i16>(Variant(_21, 3), 2)) = _19;
Goto(bb16)
}
bb16 = {
Call(_46 = dump_var(13_usize, 32_usize, Move(_32), 28_usize, Move(_28), 30_usize, Move(_30), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(13_usize, 18_usize, Move(_18), 14_usize, Move(_14), 24_usize, Move(_24), 33_usize, Move(_33)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_46 = dump_var(13_usize, 29_usize, Move(_29), 4_usize, Move(_4), 38_usize, Move(_38), 6_usize, Move(_6)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_46 = dump_var(13_usize, 19_usize, Move(_19), 20_usize, Move(_20), 47_usize, _47, 47_usize, _47), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: char,mut _2: u128,mut _3: char,mut _4: [u32; 6],mut _5: isize,mut _6: i16,mut _7: [u32; 6],mut _8: [usize; 5]) -> isize {
mir! {
type RET = isize;
let _9: [i64; 5];
let _10: isize;
let _11: [u32; 6];
let _12: [i16; 4];
let _13: *mut &'static i32;
let _14: isize;
let _15: i8;
let _16: f64;
let _17: isize;
let _18: f64;
let _19: *const [char; 8];
let _20: char;
let _21: *const [char; 8];
let _22: [i64; 5];
let _23: ();
let _24: ();
{
_5 = 8180468874101366429_u64 as isize;
_1 = _3;
RET = _5;
_7 = _4;
_8 = [2956881094810911874_usize,3954018785871896487_usize,2919215598483366884_usize,13503892199079493151_usize,6_usize];
_3 = _1;
_8 = [17706455752253278026_usize,4655540793176337735_usize,1424430101997650516_usize,135722122388132333_usize,2_usize];
Call(_9 = fn15(_8, _2, _4, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = true as i16;
_4 = _7;
RET = _5 >> _2;
_5 = !RET;
_10 = 118087972173873496237366545213383379617_i128 as isize;
_3 = _1;
_2 = !241547005577773835185502207857894140984_u128;
_7 = [376596320_u32,3087218983_u32,2406316376_u32,2946501304_u32,2940584658_u32,3256652088_u32];
_12 = [_6,_6,_6,_6];
_8 = [17046878855483853057_usize,14420564061967524412_usize,785965603043193959_usize,3333962696400943820_usize,8869834117002494188_usize];
_5 = _10 + RET;
Goto(bb2)
}
bb2 = {
_5 = RET << _10;
_1 = _3;
_10 = !RET;
_10 = !RET;
_14 = 7127519361742558789_u64 as isize;
_10 = _5;
_5 = -_10;
_2 = 54582299305106605162099651650425400825_u128;
_11 = _4;
_11 = [3985435627_u32,3372356212_u32,2820951989_u32,255385578_u32,3569377338_u32,3286019424_u32];
_16 = (-2053663939_i32) as f64;
_14 = _16 as isize;
_1 = _3;
_8 = [5_usize,2582780247328946786_usize,0_usize,8261680384918408358_usize,6_usize];
_8 = [5_usize,5_usize,2677213569499282206_usize,6394291775488956522_usize,1_usize];
Goto(bb3)
}
bb3 = {
RET = true as isize;
_16 = 56041_u16 as f64;
_6 = _10 as i16;
_15 = 94_i8 - 114_i8;
Call(_14 = fn17(_5, _4, _10), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_6 = 7994_i16 | (-1160_i16);
_17 = _14 >> _5;
_4 = [4181889101_u32,1877379240_u32,2041779874_u32,3685564450_u32,3135574729_u32,2423656729_u32];
_2 = 113309169990579049824139435449239603196_u128 + 223914101708483251277662073359749223242_u128;
_17 = -_14;
_9 = [(-8333633880470775484_i64),(-6104373176027478936_i64),6311824239485813446_i64,7582932217224265374_i64,6850664569934009061_i64];
Goto(bb5)
}
bb5 = {
_6 = (-20815_i16);
_9 = [6097088747568504810_i64,(-1091946744990236590_i64),2946644524016249256_i64,(-8602482571912726731_i64),(-6660677208291923409_i64)];
_15 = -(-121_i8);
_7 = _4;
_8 = [936342637720118720_usize,18046243735950690482_usize,3_usize,3_usize,4_usize];
_18 = -_16;
_7 = [2294570584_u32,2088807686_u32,769114074_u32,347460682_u32,3506282506_u32,3244090023_u32];
_2 = _1 as u128;
RET = -_14;
_1 = _3;
_15 = 169_u8 as i8;
_8 = [13917534744223896102_usize,4_usize,15615048656946650615_usize,2_usize,5_usize];
_8 = [2_usize,6636668815887603314_usize,3_usize,0_usize,14273465319020208685_usize];
_7 = [867810512_u32,942439659_u32,2548598845_u32,1598733782_u32,897337636_u32,2217947313_u32];
_3 = _1;
_20 = _1;
_9 = [(-436070558342601544_i64),5434985993333191146_i64,(-8405466509754598702_i64),(-2512590472766858120_i64),(-6994075630946125230_i64)];
Goto(bb6)
}
bb6 = {
Call(_23 = dump_var(14_usize, 8_usize, Move(_8), 3_usize, Move(_3), 17_usize, Move(_17), 12_usize, Move(_12)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_23 = dump_var(14_usize, 20_usize, Move(_20), 1_usize, Move(_1), 14_usize, Move(_14), 11_usize, Move(_11)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: [usize; 5],mut _2: u128,mut _3: [u32; 6],mut _4: i16) -> [i64; 5] {
mir! {
type RET = [i64; 5];
let _5: &'static *const i8;
let _6: &'static i64;
let _7: i8;
let _8: i32;
let _9: isize;
let _10: i16;
let _11: *mut i16;
let _12: Adt86;
let _13: [i64; 1];
let _14: bool;
let _15: *const *mut *mut isize;
let _16: f32;
let _17: [i8; 4];
let _18: &'static isize;
let _19: isize;
let _20: i8;
let _21: u128;
let _22: isize;
let _23: i64;
let _24: isize;
let _25: f64;
let _26: (char, [usize; 7]);
let _27: char;
let _28: i64;
let _29: i8;
let _30: Adt50;
let _31: (isize,);
let _32: Adt50;
let _33: i32;
let _34: Adt82;
let _35: &'static i64;
let _36: ();
let _37: ();
{
RET = [7740355606879150937_i64,8282641204943172440_i64,5265325612170525026_i64,1667804030720171401_i64,(-4315679654929860172_i64)];
RET = [(-2947401179577359717_i64),(-5772652567417516206_i64),(-6771711518702405825_i64),(-4346774000627470862_i64),2268500121426103025_i64];
_1 = [6_usize,7820728887316360276_usize,1_usize,3442347705419075302_usize,7_usize];
RET = [(-7186508700529760480_i64),(-8855412558991478139_i64),1361797675439745436_i64,(-29631391807331260_i64),8091803360384345723_i64];
RET = [(-3555130947267809383_i64),(-7173343664092347716_i64),7159048219949777258_i64,6520568871927144138_i64,(-5596879521843704569_i64)];
_1 = [1_usize,2_usize,17544853168873586058_usize,622898238298855338_usize,6696558992047188742_usize];
_1 = [14672041004082165588_usize,2104212302117365651_usize,6_usize,4220724561094346726_usize,15694361277634980413_usize];
RET = [(-3646055280057622306_i64),(-8891436117290033832_i64),(-7459865121041066817_i64),(-7504824950820190918_i64),(-2476935562094991136_i64)];
_8 = (-663795928_i32);
_1 = [13044579472212271702_usize,5427055261285025725_usize,15830511330356134828_usize,2817606031113842920_usize,3699146288206751902_usize];
RET = [903253181374811926_i64,(-2106015494402024228_i64),1225802667763390123_i64,(-8523593640261180756_i64),3034934431058331535_i64];
_8 = -1340139938_i32;
_1 = [4744383816176539920_usize,4_usize,1_usize,2_usize,0_usize];
_10 = _4 >> _4;
_11 = core::ptr::addr_of_mut!(_10);
RET = [3004250347635311772_i64,3447296097952276228_i64,(-2992183898440669961_i64),(-6372106622013129122_i64),950077795674916612_i64];
Goto(bb1)
}
bb1 = {
_7 = (-53_i8);
_3 = [2883423838_u32,2991336751_u32,422229903_u32,3145825276_u32,242323533_u32,4209635424_u32];
_9 = 8598757021290730606_i64 as isize;
_1 = [5259942858758057668_usize,12618510600651464703_usize,3_usize,1_usize,6_usize];
RET = [(-2921416276143214150_i64),2447996918186468329_i64,6007000762521651844_i64,(-6650485624383842618_i64),(-7261456142749846869_i64)];
(*_11) = !_4;
(*_11) = _4;
_11 = core::ptr::addr_of_mut!((*_11));
(*_11) = _4;
_2 = 56375291029841306011565713027018633044_u128 | 325919801581236763868858530143636023912_u128;
_4 = -_10;
_13 = [(-7987138450294048671_i64)];
_2 = !282624208595903441242398034224983065994_u128;
_14 = true;
_14 = true;
RET = [1618506537257473498_i64,(-4612727740924996690_i64),2948350953740931995_i64,(-7637125091475993432_i64),8787165139096634694_i64];
_4 = !(*_11);
_16 = 5516821620737017831_usize as f32;
Call(_9 = fn16(RET, Move(_11), _4, RET, _4, RET, _1, _10, _3, _3, (*_11)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = [330156422333844632_i64,551850542043727868_i64,(-6370414872460668787_i64),4450177624894693367_i64,(-3959288061007143989_i64)];
_14 = !false;
RET = [(-79509289106831185_i64),(-2690383007058955851_i64),(-6838971797293868241_i64),3909736833429899796_i64,3323524045262492343_i64];
_3 = [622593277_u32,3335741708_u32,558677827_u32,3504355328_u32,2604606953_u32,2885836145_u32];
_4 = 1900376132_u32 as i16;
_10 = -_4;
_2 = 335834819009664206388095594943932230704_u128 + 139161211147606347633386156956250826251_u128;
_18 = &_9;
_11 = core::ptr::addr_of_mut!(_10);
_11 = core::ptr::addr_of_mut!((*_11));
Goto(bb3)
}
bb3 = {
_1 = [3474425997010467521_usize,5_usize,17354211172331347804_usize,7_usize,15628252707161404002_usize];
_9 = 71_isize + 9223372036854775807_isize;
(*_11) = (-6645638578341106861_i64) as i16;
_9 = 2306988380_u32 as isize;
_13 = [(-426573549190335495_i64)];
_14 = true;
_7 = -(-114_i8);
Goto(bb4)
}
bb4 = {
(*_11) = _4;
_17 = [_7,_7,_7,_7];
(*_11) = '\u{ecdc4}' as i16;
_4 = !(*_11);
Goto(bb5)
}
bb5 = {
_18 = &_9;
_17 = [_7,_7,_7,_7];
_10 = -_4;
(*_11) = -_4;
_1 = [3_usize,6806691959137984887_usize,8001741618842361984_usize,4_usize,13016618657669652640_usize];
_17 = [_7,_7,_7,_7];
Goto(bb6)
}
bb6 = {
_13 = [7554738641208052437_i64];
_18 = &_19;
_18 = &(*_18);
_10 = _4 & _4;
_2 = _8 as u128;
RET = [(-1376505295114603051_i64),(-8791085994468865718_i64),5472696350155512001_i64,(-664387339835626747_i64),(-3574586592593671451_i64)];
(*_11) = 52503491167370476245568649179538573258_i128 as i16;
_14 = true;
Goto(bb7)
}
bb7 = {
_18 = &(*_18);
_19 = _9;
_8 = 1396891619_i32;
_20 = _16 as i8;
_22 = 184_u8 as isize;
_22 = !_9;
_20 = _7 ^ _7;
_2 = 233631316577752759021847059288827769395_u128 | 329938055645206605906156720514110742302_u128;
_19 = 88_u8 as isize;
Goto(bb8)
}
bb8 = {
RET = [(-4021060581830853403_i64),(-8619740973541866150_i64),(-5417310773535349763_i64),(-3985688348531657499_i64),(-4646867186856083035_i64)];
_21 = _2 - _2;
_23 = '\u{c3f2d}' as i64;
_7 = _20 >> _21;
_14 = true ^ true;
(*_11) = _4 | _4;
_6 = &_23;
_19 = !_22;
_2 = _21;
_22 = _19 * _19;
_20 = -_7;
_23 = _7 as i64;
_18 = &_9;
_17 = [_7,_7,_7,_20];
(*_11) = _4;
_16 = _21 as f32;
_11 = core::ptr::addr_of_mut!((*_11));
_26.1 = [4541968453116953379_usize,12915016371935966889_usize,5_usize,6_usize,17146628108220673201_usize,1_usize,4_usize];
_26.0 = '\u{72f70}';
_23 = 8022642786024473431_i64 + (-6562591505484994258_i64);
_16 = 11792795454494333227_usize as f32;
_20 = !_7;
_3 = [418711626_u32,2942465555_u32,116159124_u32,3626050958_u32,405594686_u32,1748922497_u32];
match _8 {
0 => bb6,
1 => bb9,
2 => bb10,
1396891619 => bb12,
_ => bb11
}
}
bb9 = {
RET = [330156422333844632_i64,551850542043727868_i64,(-6370414872460668787_i64),4450177624894693367_i64,(-3959288061007143989_i64)];
_14 = !false;
RET = [(-79509289106831185_i64),(-2690383007058955851_i64),(-6838971797293868241_i64),3909736833429899796_i64,3323524045262492343_i64];
_3 = [622593277_u32,3335741708_u32,558677827_u32,3504355328_u32,2604606953_u32,2885836145_u32];
_4 = 1900376132_u32 as i16;
_10 = -_4;
_2 = 335834819009664206388095594943932230704_u128 + 139161211147606347633386156956250826251_u128;
_18 = &_9;
_11 = core::ptr::addr_of_mut!(_10);
_11 = core::ptr::addr_of_mut!((*_11));
Goto(bb3)
}
bb10 = {
_1 = [3474425997010467521_usize,5_usize,17354211172331347804_usize,7_usize,15628252707161404002_usize];
_9 = 71_isize + 9223372036854775807_isize;
(*_11) = (-6645638578341106861_i64) as i16;
_9 = 2306988380_u32 as isize;
_13 = [(-426573549190335495_i64)];
_14 = true;
_7 = -(-114_i8);
Goto(bb4)
}
bb11 = {
_18 = &_9;
_17 = [_7,_7,_7,_7];
_10 = -_4;
(*_11) = -_4;
_1 = [3_usize,6806691959137984887_usize,8001741618842361984_usize,4_usize,13016618657669652640_usize];
_17 = [_7,_7,_7,_7];
Goto(bb6)
}
bb12 = {
_1 = [5490130159025363455_usize,3_usize,4_usize,0_usize,5_usize];
_6 = &_23;
_22 = (*_18);
_24 = _16 as isize;
Goto(bb13)
}
bb13 = {
_14 = true;
_27 = _26.0;
_20 = _7;
_25 = (*_6) as f64;
(*_11) = !_4;
_3 = [3536726122_u32,2250282998_u32,881954299_u32,849878317_u32,447549098_u32,10340956_u32];
_4 = _26.0 as i16;
_11 = core::ptr::addr_of_mut!(_10);
_13 = [(*_6)];
_29 = !_20;
_16 = _8 as f32;
_22 = _27 as isize;
_20 = _29;
_6 = &_23;
(*_11) = _4;
_26.0 = _27;
_20 = _29;
match _8 {
0 => bb11,
1396891619 => bb14,
_ => bb2
}
}
bb14 = {
(*_11) = 38960_u16 as i16;
_21 = !_2;
_21 = _2 & _2;
_18 = &_9;
_31 = ((*_18),);
(*_11) = -_4;
_25 = _24 as f64;
_4 = !(*_11);
_26.0 = _27;
_34.fld0 = [4_usize,10464036523299662610_usize,7_usize,17250727982223314165_usize,18162176235959611530_usize];
_14 = _20 <= _20;
_27 = _26.0;
_26.0 = _27;
_18 = &_19;
_23 = 5132808090952873940_i64;
(*_11) = _4 | _4;
_25 = _2 as f64;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(15_usize, 13_usize, Move(_13), 29_usize, Move(_29), 9_usize, Move(_9), 31_usize, Move(_31)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(15_usize, 20_usize, Move(_20), 22_usize, Move(_22), 17_usize, Move(_17), 21_usize, Move(_21)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(15_usize, 14_usize, Move(_14), 3_usize, Move(_3), 37_usize, _37, 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: [i64; 5],mut _2: *mut i16,mut _3: i16,mut _4: [i64; 5],mut _5: i16,mut _6: [i64; 5],mut _7: [usize; 5],mut _8: i16,mut _9: [u32; 6],mut _10: [u32; 6],mut _11: i16) -> isize {
mir! {
type RET = isize;
let _12: isize;
let _13: usize;
let _14: char;
let _15: i8;
let _16: &'static &'static *mut u8;
let _17: [i64; 8];
let _18: Adt31;
let _19: *mut u8;
let _20: &'static isize;
let _21: *const [char; 8];
let _22: *const i64;
let _23: isize;
let _24: *mut [i32; 1];
let _25: Adt76;
let _26: u16;
let _27: usize;
let _28: &'static *const [u32; 6];
let _29: u16;
let _30: &'static i64;
let _31: bool;
let _32: [usize; 5];
let _33: [usize; 7];
let _34: ([i64; 8], *mut *const [char; 8], [u32; 6], [u16; 4]);
let _35: &'static i32;
let _36: usize;
let _37: u32;
let _38: *const &'static *mut u8;
let _39: isize;
let _40: ();
let _41: ();
{
_12 = 7_usize as isize;
_12 = -(-123_isize);
RET = _12 ^ _12;
_4 = [7989280467857339349_i64,(-652628039783062486_i64),(-2012074606393899565_i64),914421859981956943_i64,(-414790221519667072_i64)];
_5 = 2336506969_u32 as i16;
_9 = _10;
_4 = _1;
_3 = !_5;
RET = _12;
_13 = !5_usize;
_3 = 109856024061502511118429200450080285522_u128 as i16;
Goto(bb1)
}
bb1 = {
_9 = [3406247133_u32,1180439701_u32,3352099894_u32,1189515834_u32,127598106_u32,4035538546_u32];
_14 = '\u{ba9e1}';
_7 = [_13,_13,_13,_13,_13];
_8 = 192_u8 as i16;
_4 = _6;
RET = _12;
Call(_13 = core::intrinsics::bswap(16159138381781463992_usize), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_10 = [3512793870_u32,921688018_u32,2677497019_u32,4191533324_u32,3226631758_u32,2168401104_u32];
_10 = _9;
_15 = 40_i8 & 68_i8;
_12 = RET;
_12 = -RET;
_15 = _8 as i8;
_5 = 2097479745_i32 as i16;
RET = !_12;
_2 = core::ptr::addr_of_mut!(_11);
_8 = RET as i16;
(*_2) = _3;
RET = _12 * _12;
_1 = [6678464030242530534_i64,9118440893358826819_i64,819042925147131410_i64,3257139913852936579_i64,(-2094033159343083519_i64)];
_4 = [(-1130466492932821951_i64),(-1818943530269012666_i64),5683886310931889091_i64,1635620220275232806_i64,7571345956527561187_i64];
_6 = [6158846805419008798_i64,8285182657719010829_i64,463524420006919550_i64,(-2811291974811368898_i64),6756822125328390952_i64];
_17 = [1498234679971872752_i64,3001116891121258497_i64,(-4589680814572583769_i64),3186563643920058032_i64,6088582848548388131_i64,(-444710356236038536_i64),2547997523987466135_i64,110474608929183180_i64];
_3 = -(*_2);
Goto(bb3)
}
bb3 = {
_13 = 915923402648161875_usize;
_5 = _3;
_8 = (*_2);
_23 = RET;
(*_2) = _15 as i16;
RET = false as isize;
(*_2) = _23 as i16;
_7 = [_13,_13,_13,_13,_13];
_12 = _23 + RET;
_15 = 37_i8 & 47_i8;
_15 = 1970264572_u32 as i8;
_2 = core::ptr::addr_of_mut!((*_2));
_6 = _1;
_13 = 2585393600970675350_usize & 15884110071410699983_usize;
_14 = '\u{e70d6}';
RET = _23;
_4 = [5642520028225563715_i64,(-2422764746091583548_i64),8116506078753746672_i64,8714963541878006639_i64,6565644253841016087_i64];
RET = _23;
_14 = '\u{ef87}';
_1 = [(-340585002636382940_i64),6869950355448171829_i64,2720861627268547306_i64,(-2497821396263904231_i64),5444496427225380569_i64];
_9 = _10;
_10 = [3410149177_u32,1350455844_u32,2444582550_u32,3343277342_u32,1065221704_u32,3610400229_u32];
RET = _12 >> _23;
_20 = &_23;
_8 = false as i16;
_8 = _3;
_2 = core::ptr::addr_of_mut!(_11);
Goto(bb4)
}
bb4 = {
_3 = 4608240723050229130_i64 as i16;
_5 = (*_2) >> _23;
_6 = [3261042474562575502_i64,7883123515248120233_i64,(-5558617216486030810_i64),(-4170416869622048293_i64),5079700802215089508_i64];
_27 = _13 << RET;
_10 = [2233771574_u32,3086407441_u32,2640169616_u32,2214993962_u32,93975170_u32,4151276929_u32];
_10 = _9;
_2 = core::ptr::addr_of_mut!((*_2));
_29 = (*_2) as u16;
_26 = _29 & _29;
_1 = _6;
_9 = _10;
_3 = _11 & _8;
_17 = [2202996935868215115_i64,(-4189199603727349697_i64),5664261180495810841_i64,2614117021824898194_i64,(-8788990193073086527_i64),8677179966489892548_i64,3292860892281124956_i64,7913592348547368080_i64];
RET = (*_20) - (*_20);
_13 = !_27;
(*_2) = _3;
_4 = [(-3502609583044504347_i64),6311183175296879521_i64,3717307833081719596_i64,4747044054476837013_i64,20480083312875307_i64];
Call(_3 = core::intrinsics::bswap(_8), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET = _14 as isize;
_6 = [(-6192585939073749101_i64),(-6775726602238674128_i64),8791420284607714625_i64,5981984824086765758_i64,734073576755072459_i64];
_9 = _10;
_10 = [3717346334_u32,845481926_u32,1073111470_u32,929796092_u32,4102667174_u32,740934916_u32];
_1 = [6731054154973847783_i64,(-1824870159930644160_i64),3577152802963180889_i64,(-8113052605324509720_i64),1231085753723680654_i64];
(*_2) = _14 as i16;
_6 = [8836515806794444180_i64,6460365728072336856_i64,(-4375132801763981757_i64),8898788044890915386_i64,3796904915982077466_i64];
(*_2) = _26 as i16;
_31 = _27 > _13;
RET = _14 as isize;
_15 = 48_i8;
_15 = _12 as i8;
Goto(bb6)
}
bb6 = {
_1 = [(-298986808035964217_i64),(-5075199623454879696_i64),(-7413436651956811858_i64),6695866338558775511_i64,(-2658414157038014816_i64)];
RET = !(*_20);
Call(_29 = core::intrinsics::transmute((*_2)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_34.2 = _10;
RET = _23 << _27;
_34.0 = _17;
_15 = 115_i8;
RET = (*_20);
_32 = [_13,_27,_27,_27,_13];
_34.1 = core::ptr::addr_of_mut!(_21);
_17 = [(-7766156316378726398_i64),3634173454392052507_i64,(-7866503133856398456_i64),4567726100447140634_i64,(-1774348171261622065_i64),(-9040094470994950093_i64),(-4004186106229322007_i64),(-1134114683086428525_i64)];
_3 = (*_2);
_8 = _3 | _11;
_12 = (*_20);
match _15 {
0 => bb5,
1 => bb2,
2 => bb8,
115 => bb10,
_ => bb9
}
}
bb8 = {
_10 = [3512793870_u32,921688018_u32,2677497019_u32,4191533324_u32,3226631758_u32,2168401104_u32];
_10 = _9;
_15 = 40_i8 & 68_i8;
_12 = RET;
_12 = -RET;
_15 = _8 as i8;
_5 = 2097479745_i32 as i16;
RET = !_12;
_2 = core::ptr::addr_of_mut!(_11);
_8 = RET as i16;
(*_2) = _3;
RET = _12 * _12;
_1 = [6678464030242530534_i64,9118440893358826819_i64,819042925147131410_i64,3257139913852936579_i64,(-2094033159343083519_i64)];
_4 = [(-1130466492932821951_i64),(-1818943530269012666_i64),5683886310931889091_i64,1635620220275232806_i64,7571345956527561187_i64];
_6 = [6158846805419008798_i64,8285182657719010829_i64,463524420006919550_i64,(-2811291974811368898_i64),6756822125328390952_i64];
_17 = [1498234679971872752_i64,3001116891121258497_i64,(-4589680814572583769_i64),3186563643920058032_i64,6088582848548388131_i64,(-444710356236038536_i64),2547997523987466135_i64,110474608929183180_i64];
_3 = -(*_2);
Goto(bb3)
}
bb9 = {
RET = _14 as isize;
_6 = [(-6192585939073749101_i64),(-6775726602238674128_i64),8791420284607714625_i64,5981984824086765758_i64,734073576755072459_i64];
_9 = _10;
_10 = [3717346334_u32,845481926_u32,1073111470_u32,929796092_u32,4102667174_u32,740934916_u32];
_1 = [6731054154973847783_i64,(-1824870159930644160_i64),3577152802963180889_i64,(-8113052605324509720_i64),1231085753723680654_i64];
(*_2) = _14 as i16;
_6 = [8836515806794444180_i64,6460365728072336856_i64,(-4375132801763981757_i64),8898788044890915386_i64,3796904915982077466_i64];
(*_2) = _26 as i16;
_31 = _27 > _13;
RET = _14 as isize;
_15 = 48_i8;
_15 = _12 as i8;
Goto(bb6)
}
bb10 = {
_9 = [3125111796_u32,2997617416_u32,1663508139_u32,1735151063_u32,3421724347_u32,1084397456_u32];
_4 = [(-829699304064113977_i64),(-2080152304401675798_i64),4259964697667397663_i64,8995086004861562876_i64,1031935473791946318_i64];
RET = (*_20) << _27;
(*_2) = _8 & _8;
_4 = _6;
_6 = [(-8505795577953200986_i64),4423034822051422180_i64,(-4709179228469956053_i64),(-8218403006914190015_i64),2622190619136277476_i64];
_7 = [_27,_13,_27,_27,_13];
_23 = _26 as isize;
_6 = _4;
_27 = _14 as usize;
(*_2) = _15 as i16;
_15 = 3561698384_u32 as i8;
_23 = RET ^ RET;
_4 = [(-3318291660608387395_i64),(-6891600651746254943_i64),(-60994113249705256_i64),8715928936724980092_i64,3808167517458367987_i64];
_2 = core::ptr::addr_of_mut!((*_2));
_33 = [_27,_13,_27,_13,_13,_13,_13];
_7 = _32;
_27 = 120_u8 as usize;
_32 = _7;
RET = _23;
_17 = [7568257348943658873_i64,(-1793289285601113893_i64),(-6956293173435946916_i64),(-2614946739186932582_i64),3815727944699713310_i64,3805237845514418536_i64,4865486611230612738_i64,(-4639708920400847877_i64)];
(*_2) = 121236568898471679444058812827546529101_i128 as i16;
_17 = [(-6510487804303143804_i64),(-3917740933208084269_i64),2102122268853684317_i64,6944959312481585840_i64,(-2139233381497056069_i64),4665139211832305738_i64,6888556618556099450_i64,6986739287285442644_i64];
_34.3 = [_29,_26,_29,_29];
_8 = (*_2);
_15 = _14 as i8;
_36 = _13;
Goto(bb11)
}
bb11 = {
Call(_40 = dump_var(16_usize, 15_usize, Move(_15), 27_usize, Move(_27), 31_usize, Move(_31), 32_usize, Move(_32)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_40 = dump_var(16_usize, 17_usize, Move(_17), 36_usize, Move(_36), 6_usize, Move(_6), 1_usize, Move(_1)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_40 = dump_var(16_usize, 33_usize, Move(_33), 10_usize, Move(_10), 7_usize, Move(_7), 41_usize, _41), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: isize,mut _2: [u32; 6],mut _3: isize) -> isize {
mir! {
type RET = isize;
let _4: bool;
let _5: *const &'static *mut u8;
let _6: ();
let _7: ();
{
_2 = [3818477382_u32,2652056746_u32,4205842800_u32,2575660755_u32,636174755_u32,2408473556_u32];
_4 = _3 != _1;
_3 = _1 & _1;
RET = _3 + _3;
_1 = -RET;
_4 = true;
Goto(bb1)
}
bb1 = {
Call(_6 = dump_var(17_usize, 1_usize, Move(_1), 3_usize, Move(_3), 7_usize, _7, 7_usize, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: isize) -> *mut [i32; 1] {
mir! {
type RET = *mut [i32; 1];
let _2: [char; 8];
let _3: isize;
let _4: Adt86;
let _5: (u128, i128);
let _6: [i16; 4];
let _7: f64;
let _8: isize;
let _9: [usize; 5];
let _10: bool;
let _11: i16;
let _12: &'static [i16; 4];
let _13: *mut isize;
let _14: u64;
let _15: char;
let _16: u8;
let _17: char;
let _18: i16;
let _19: [u32; 6];
let _20: *const [u32; 6];
let _21: isize;
let _22: i128;
let _23: *const *mut u32;
let _24: isize;
let _25: u8;
let _26: i16;
let _27: i128;
let _28: &'static [u8; 6];
let _29: *mut *mut u8;
let _30: isize;
let _31: isize;
let _32: *const i64;
let _33: u64;
let _34: (&'static i64, (u64, i16));
let _35: *mut f64;
let _36: i64;
let _37: *const &'static *mut u8;
let _38: (isize,);
let _39: *mut u32;
let _40: f64;
let _41: [char; 8];
let _42: [i64; 1];
let _43: u64;
let _44: [u8; 6];
let _45: &'static *mut &'static u128;
let _46: f32;
let _47: u128;
let _48: &'static (isize,);
let _49: char;
let _50: [i8; 4];
let _51: &'static &'static &'static *mut u8;
let _52: i64;
let _53: i8;
let _54: [u32; 6];
let _55: [u32; 3];
let _56: (((isize,), &'static *mut u8, (&'static i64, (u64, i16)), (&'static i64, (u64, i16))), *mut *mut u8, usize, [i32; 1]);
let _57: Adt64;
let _58: ((isize,), &'static *mut u8, (&'static i64, (u64, i16)), (&'static i64, (u64, i16)));
let _59: *mut *mut isize;
let _60: isize;
let _61: *mut char;
let _62: [u32; 6];
let _63: [u8; 7];
let _64: [i32; 1];
let _65: (u128,);
let _66: &'static *mut u8;
let _67: (((isize,), &'static *mut u8, (&'static i64, (u64, i16)), (&'static i64, (u64, i16))), *mut *mut u8, usize, [i32; 1]);
let _68: ();
let _69: ();
{
_2 = ['\u{63a7f}','\u{1a8c1}','\u{f88f2}','\u{109594}','\u{de886}','\u{ed7e5}','\u{c3e1a}','\u{e0681}'];
Goto(bb1)
}
bb1 = {
_1 = 194_u8 as isize;
_2 = ['\u{a5258}','\u{79073}','\u{40654}','\u{c7dfd}','\u{503a0}','\u{1a3fd}','\u{d718e}','\u{59f2}'];
_1 = (-80_isize) | 9223372036854775807_isize;
_1 = (-9223372036854775808_isize);
_1 = (-9223372036854775808_isize);
match _1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463454151235394913435648 => bb9,
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
_1 = (-81_isize);
_1 = !(-35_isize);
_1 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_3 = _1 - _1;
_3 = _1;
_2 = ['\u{9c8db}','\u{b2c8}','\u{8e10d}','\u{23d01}','\u{91d78}','\u{7ef89}','\u{99b76}','\u{3b463}'];
_2 = ['\u{ed10f}','\u{41f8d}','\u{2f9dc}','\u{9b8e1}','\u{935e2}','\u{5dffa}','\u{102327}','\u{2d8b3}'];
_2 = ['\u{8db84}','\u{f5e9d}','\u{28335}','\u{bfc35}','\u{edb91}','\u{2cd70}','\u{16e9f}','\u{7104f}'];
_5.1 = !(-152276856348757447920140834983038971912_i128);
Goto(bb10)
}
bb10 = {
_5.0 = 189575225695809616390591683468670138785_u128 * 172615065145337037308833269897752308214_u128;
_5 = (248128860228433084311998347674596142165_u128, (-143896087547079605364785642559261024531_i128));
_5 = (141604411293505562759924592571346893822_u128, (-134722654092659497862851683626884743903_i128));
_6 = [4084_i16,(-21245_i16),13138_i16,(-29038_i16)];
_1 = -_3;
_7 = _5.1 as f64;
_6 = [(-9331_i16),(-2347_i16),(-22818_i16),(-18913_i16)];
_8 = _3;
_7 = _8 as f64;
_7 = 11115562385882282234_u64 as f64;
_6 = [11473_i16,(-7405_i16),(-22462_i16),30380_i16];
_5 = (135593687165320334368608497923567137219_u128, (-79046054938632785253931804673204603667_i128));
_1 = _7 as isize;
_5.0 = 333185824647524296525786060601119588001_u128;
match _5.0 {
0 => bb2,
333185824647524296525786060601119588001 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_3 = _8;
_13 = core::ptr::addr_of_mut!(_8);
_14 = !11414680663435766160_u64;
_5.1 = (-22461964766584051460754341068333326236_i128);
_5 = (225500552533128939149590404188432809298_u128, 83978911461146724929614851259426145421_i128);
_9 = [14112028550578133854_usize,6_usize,4999497863186163802_usize,15730802713899846251_usize,10475945296852728003_usize];
_2 = ['\u{98f6}','\u{431e6}','\u{6fc95}','\u{8be23}','\u{d41e2}','\u{797e4}','\u{6ac40}','\u{fa68a}'];
_8 = !_3;
(*_13) = _3 << _5.0;
_9 = [5_usize,4_usize,16225838804740380959_usize,4_usize,0_usize];
_14 = 7163489538916319314_u64;
(*_13) = _3 << _3;
match _14 {
0 => bb1,
1 => bb7,
2 => bb9,
7163489538916319314 => bb14,
_ => bb13
}
}
bb13 = {
_1 = 194_u8 as isize;
_2 = ['\u{a5258}','\u{79073}','\u{40654}','\u{c7dfd}','\u{503a0}','\u{1a3fd}','\u{d718e}','\u{59f2}'];
_1 = (-80_isize) | 9223372036854775807_isize;
_1 = (-9223372036854775808_isize);
_1 = (-9223372036854775808_isize);
match _1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463454151235394913435648 => bb9,
_ => bb8
}
}
bb14 = {
_10 = true | false;
_1 = (*_13);
_10 = false;
_12 = &_6;
_12 = &(*_12);
_15 = '\u{aa405}';
_2 = [_15,_15,_15,_15,_15,_15,_15,_15];
_3 = !(*_13);
_8 = _3;
_9 = [1263062850900341525_usize,5_usize,15565707680367805618_usize,6_usize,16136661641338725292_usize];
_6 = [(-8017_i16),(-9993_i16),(-20531_i16),9563_i16];
_6 = [(-30749_i16),(-3164_i16),16327_i16,21920_i16];
_16 = 165_u8;
_7 = 8933536459260207514_i64 as f64;
(*_13) = _1;
_5.1 = 82745114971412106962051172711274190010_i128;
_9 = [7_usize,5_usize,6999577373936067270_usize,3_usize,2_usize];
_3 = _8 | _8;
_18 = (-1335017460_i32) as i16;
_16 = !128_u8;
Goto(bb15)
}
bb15 = {
_3 = _8;
_12 = &_6;
_5.1 = !(-36685579666072277206753954130927325895_i128);
_3 = !(*_13);
_7 = _8 as f64;
_6 = [_18,_18,_18,_18];
_11 = 17036624180269982082_usize as i16;
_19 = [1312654549_u32,900012619_u32,4074808816_u32,1623501541_u32,1995716628_u32,717759581_u32];
_14 = 9709451313008888715_u64 - 14495002736091040223_u64;
_12 = &_6;
_6 = [_18,_11,_11,_11];
match _5.0 {
0 => bb11,
1 => bb2,
2 => bb16,
3 => bb17,
4 => bb18,
5 => bb19,
6 => bb20,
225500552533128939149590404188432809298 => bb22,
_ => bb21
}
}
bb16 = {
Return()
}
bb17 = {
_1 = 194_u8 as isize;
_2 = ['\u{a5258}','\u{79073}','\u{40654}','\u{c7dfd}','\u{503a0}','\u{1a3fd}','\u{d718e}','\u{59f2}'];
_1 = (-80_isize) | 9223372036854775807_isize;
_1 = (-9223372036854775808_isize);
_1 = (-9223372036854775808_isize);
match _1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463454151235394913435648 => bb9,
_ => bb8
}
}
bb18 = {
_3 = _8;
_13 = core::ptr::addr_of_mut!(_8);
_14 = !11414680663435766160_u64;
_5.1 = (-22461964766584051460754341068333326236_i128);
_5 = (225500552533128939149590404188432809298_u128, 83978911461146724929614851259426145421_i128);
_9 = [14112028550578133854_usize,6_usize,4999497863186163802_usize,15730802713899846251_usize,10475945296852728003_usize];
_2 = ['\u{98f6}','\u{431e6}','\u{6fc95}','\u{8be23}','\u{d41e2}','\u{797e4}','\u{6ac40}','\u{fa68a}'];
_8 = !_3;
(*_13) = _3 << _5.0;
_9 = [5_usize,4_usize,16225838804740380959_usize,4_usize,0_usize];
_14 = 7163489538916319314_u64;
(*_13) = _3 << _3;
match _14 {
0 => bb1,
1 => bb7,
2 => bb9,
7163489538916319314 => bb14,
_ => bb13
}
}
bb19 = {
Return()
}
bb20 = {
Return()
}
bb21 = {
_1 = (-81_isize);
_1 = !(-35_isize);
_1 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_3 = _1 - _1;
_3 = _1;
_2 = ['\u{9c8db}','\u{b2c8}','\u{8e10d}','\u{23d01}','\u{91d78}','\u{7ef89}','\u{99b76}','\u{3b463}'];
_2 = ['\u{ed10f}','\u{41f8d}','\u{2f9dc}','\u{9b8e1}','\u{935e2}','\u{5dffa}','\u{102327}','\u{2d8b3}'];
_2 = ['\u{8db84}','\u{f5e9d}','\u{28335}','\u{bfc35}','\u{edb91}','\u{2cd70}','\u{16e9f}','\u{7104f}'];
_5.1 = !(-152276856348757447920140834983038971912_i128);
Goto(bb10)
}
bb22 = {
_5 = (204494331353408954809796675809355476252_u128, (-140247369838953631776474032498514905692_i128));
_3 = !(*_13);
_11 = _5.0 as i16;
_18 = _11 ^ _11;
_11 = -_18;
_9 = [6_usize,12140241083714548110_usize,2_usize,3_usize,4_usize];
_19 = [2278228208_u32,4031093726_u32,2806591249_u32,1888565344_u32,1681215383_u32,1403643790_u32];
_5.1 = 117596205120773712099357750248990106534_i128;
_8 = _3 & _1;
_22 = !_5.1;
_13 = core::ptr::addr_of_mut!(_8);
(*_13) = _7 as isize;
_11 = _18 + _18;
Goto(bb23)
}
bb23 = {
_16 = !187_u8;
_24 = (*_13);
_22 = _5.1 * _5.1;
_8 = (-789520281055393216_i64) as isize;
_21 = !_3;
_12 = &_6;
_26 = _16 as i16;
_22 = -_5.1;
_25 = _16;
_1 = -_24;
_10 = _5.0 >= _5.0;
_14 = !819181195525642940_u64;
_20 = core::ptr::addr_of!(_19);
_9 = [8216435005110274369_usize,1_usize,10205171006229337383_usize,7_usize,5_usize];
_10 = _7 > _7;
_20 = core::ptr::addr_of!((*_20));
_21 = _24;
_16 = 629870452_u32 as u8;
_15 = '\u{2e39f}';
_8 = 3516385708_u32 as isize;
(*_13) = _1 * _21;
_21 = _8;
_5.0 = 94578476387846556537987926508879538987_u128 >> (*_13);
_7 = 23_i8 as f64;
Call(_5.0 = core::intrinsics::transmute(_5.1), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
_16 = _5.0 as u8;
_14 = !1587335787612752057_u64;
_15 = '\u{3f8d1}';
_13 = core::ptr::addr_of_mut!(_21);
_3 = (*_13) ^ _8;
_11 = _7 as i16;
(*_20) = [500261068_u32,913952506_u32,713985836_u32,1107458723_u32,2239332999_u32,1970154606_u32];
_10 = false;
_30 = 2985878505618844904_usize as isize;
_22 = _5.1;
_7 = _14 as f64;
_11 = 35_i8 as i16;
_12 = &(*_12);
_5.1 = _22 << _18;
_30 = _3;
_22 = -_5.1;
_27 = -_22;
_1 = _16 as isize;
_14 = 4611181616357532381_u64;
_14 = 13169330569954559339_u64 | 4965785187219143062_u64;
_20 = core::ptr::addr_of!(_19);
_24 = _21 ^ _30;
(*_13) = !_24;
_6 = [_18,_26,_18,_18];
_34.1 = (_14, _18);
Goto(bb25)
}
bb25 = {
_22 = _27 * _27;
_17 = _15;
_31 = 39989_u16 as isize;
_24 = (*_13) - _21;
_25 = _16;
_19 = [169261152_u32,2800316119_u32,770118906_u32,2530561997_u32,2080574918_u32,3764894630_u32];
_35 = core::ptr::addr_of_mut!(_7);
_10 = !false;
_3 = _24 * (*_13);
(*_35) = _3 as f64;
_32 = core::ptr::addr_of!(_36);
_36 = 12724107325571449630_usize as i64;
_15 = _17;
_24 = -(*_13);
_18 = 1026627795176391852_usize as i16;
_25 = (*_32) as u8;
_16 = _25;
_33 = !_14;
_32 = core::ptr::addr_of!((*_32));
(*_32) = _22 as i64;
_16 = (*_35) as u8;
Goto(bb26)
}
bb26 = {
(*_32) = !2242040115226362572_i64;
_19 = [795585239_u32,2402333531_u32,3360512384_u32,1892566629_u32,1041131496_u32,1850471735_u32];
_14 = _22 as u64;
_38 = ((*_13),);
_34.1.1 = _26;
_26 = _11;
_13 = core::ptr::addr_of_mut!(_31);
_20 = core::ptr::addr_of!((*_20));
_33 = !_14;
_40 = -(*_35);
_20 = core::ptr::addr_of!((*_20));
_14 = _33 + _33;
_43 = _33 ^ _33;
_40 = _7 - _7;
(*_35) = (*_32) as f64;
_11 = _36 as i16;
_19 = [1924720521_u32,2293576076_u32,2932257036_u32,561640316_u32,1669439168_u32,2817008175_u32];
_21 = _3;
_17 = _15;
_18 = !_34.1.1;
_34.0 = &(*_32);
_18 = !_26;
_5.0 = 127402535149567375002139226671192653294_u128 & 6511727631323088601848418465090513757_u128;
_28 = &_44;
_12 = &_6;
Call(_34.1.0 = core::intrinsics::transmute(_3), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
_10 = true;
_21 = -_24;
_3 = _24 >> _22;
_10 = !false;
_32 = core::ptr::addr_of!(_36);
_38.0 = 2590178215_u32 as isize;
_42 = [(*_32)];
_42 = [(*_32)];
(*_20) = [2662826582_u32,1767997116_u32,391235446_u32,4042169790_u32,2735134594_u32,1086011702_u32];
(*_20) = [1256160429_u32,3572332418_u32,370308703_u32,676231382_u32,112383064_u32,548397422_u32];
_34.1 = (_43, _26);
_23 = core::ptr::addr_of!(_39);
_15 = _17;
_13 = core::ptr::addr_of_mut!(_38.0);
_24 = _36 as isize;
_26 = _34.1.1 ^ _11;
_24 = _3;
_15 = _17;
(*_20) = [2919558880_u32,546311341_u32,771353509_u32,1061620610_u32,3318352392_u32,4055932395_u32];
_16 = _25 + _25;
_30 = (*_32) as isize;
_5 = (152542412841519781046234981525018148204_u128, _27);
_32 = core::ptr::addr_of!(_36);
Goto(bb28)
}
bb28 = {
_27 = _10 as i128;
_33 = _14 | _43;
_28 = &(*_28);
_44 = [_25,_16,_25,_16,_16,_16];
_40 = _26 as f64;
_10 = true;
_35 = core::ptr::addr_of_mut!(_7);
_7 = -_40;
_26 = _16 as i16;
_15 = _17;
_14 = _43 & _33;
_5 = (325778542949754251032371439368757638790_u128, _22);
_49 = _15;
_9 = [7_usize,1_usize,14946251394224794816_usize,6151313793750225809_usize,5_usize];
_38 = (_3,);
_20 = core::ptr::addr_of!((*_20));
_8 = !(*_13);
(*_20) = [4023328062_u32,3107503329_u32,3005985070_u32,71403745_u32,2516950128_u32,1664126420_u32];
_13 = core::ptr::addr_of_mut!(_30);
_30 = !_31;
Goto(bb29)
}
bb29 = {
_36 = (*_35) as i64;
_32 = core::ptr::addr_of!(_36);
_47 = _34.1.0 as u128;
_6 = [_34.1.1,_26,_11,_11];
_34.1 = (_33, _26);
_46 = (*_32) as f32;
_22 = !_5.1;
(*_32) = 8446028662326703117_i64 ^ 1358596091508805451_i64;
_19 = [3493184639_u32,83471520_u32,2050359136_u32,564816927_u32,2810670558_u32,1646920575_u32];
_12 = &_6;
_34.1.0 = !_43;
_44 = [_16,_16,_25,_16,_16,_16];
_34.1.0 = _15 as u64;
_13 = core::ptr::addr_of_mut!(_31);
_34.1.1 = -_18;
_50 = [109_i8,(-116_i8),(-28_i8),82_i8];
_52 = (*_32);
_28 = &_44;
_38 = (_21,);
_48 = &_38;
_48 = &(*_48);
_17 = _15;
_38.0 = _24;
_34.1.0 = _33 | _14;
_11 = _26;
_28 = &_44;
match _5.0 {
0 => bb25,
1 => bb20,
2 => bb30,
325778542949754251032371439368757638790 => bb32,
_ => bb31
}
}
bb30 = {
_5 = (204494331353408954809796675809355476252_u128, (-140247369838953631776474032498514905692_i128));
_3 = !(*_13);
_11 = _5.0 as i16;
_18 = _11 ^ _11;
_11 = -_18;
_9 = [6_usize,12140241083714548110_usize,2_usize,3_usize,4_usize];
_19 = [2278228208_u32,4031093726_u32,2806591249_u32,1888565344_u32,1681215383_u32,1403643790_u32];
_5.1 = 117596205120773712099357750248990106534_i128;
_8 = _3 & _1;
_22 = !_5.1;
_13 = core::ptr::addr_of_mut!(_8);
(*_13) = _7 as isize;
_11 = _18 + _18;
Goto(bb23)
}
bb31 = {
Return()
}
bb32 = {
_54 = [463461037_u32,2889792614_u32,1941806261_u32,444439673_u32,1532942274_u32,907347420_u32];
_8 = _3 * _3;
_32 = core::ptr::addr_of!((*_32));
_5.0 = !_47;
_17 = _15;
_18 = _49 as i16;
_53 = -(-80_i8);
_28 = &(*_28);
_14 = (*_35) as u64;
_47 = !_5.0;
_46 = 7_usize as f32;
_30 = _3;
(*_35) = 1673607947_i32 as f64;
_6 = [_11,_11,_11,_11];
_38.0 = (-225902998_i32) as isize;
_50 = [_53,_53,_53,_53];
_8 = !_3;
_34.1.0 = _43;
_19 = [3144167557_u32,4101211846_u32,141232981_u32,3535685344_u32,3904962252_u32,2157850323_u32];
(*_20) = _54;
_44 = [_25,_16,_16,_16,_16,_16];
_34.1.0 = _33;
_12 = &_6;
_28 = &_44;
(*_32) = _52;
(*_20) = [1652816860_u32,3540715173_u32,2373472518_u32,573496185_u32,3798487264_u32,2977854729_u32];
Goto(bb33)
}
bb33 = {
(*_35) = _40 - _40;
_20 = core::ptr::addr_of!((*_20));
_53 = 16499779519934708965_usize as i8;
_41 = _2;
_53 = _17 as i8;
_35 = core::ptr::addr_of_mut!(_40);
_55 = [3629947837_u32,1617691127_u32,330388377_u32];
_26 = !_34.1.1;
_11 = 51621_u16 as i16;
_40 = _7 * _7;
Goto(bb34)
}
bb34 = {
_3 = !_24;
_40 = _7;
_58.3.1.0 = _15 as u64;
_10 = !true;
_2 = _41;
_5.1 = _47 as i128;
_59 = core::ptr::addr_of_mut!(_13);
_48 = &_38;
(*_32) = -_52;
_55 = [2916522006_u32,330948689_u32,4126019043_u32];
Goto(bb35)
}
bb35 = {
_34.1.0 = _33 >> (*_48).0;
_26 = !_18;
(*_35) = _46 as f64;
_38 = (_8,);
_18 = _26;
_10 = _3 == _24;
_58.0.0 = _5.1 as isize;
_23 = core::ptr::addr_of!(_39);
_8 = !_58.0.0;
(*_59) = core::ptr::addr_of_mut!(_38.0);
_44 = [_16,_16,_16,_16,_25,_25];
_37 = core::ptr::addr_of!(_58.1);
_5.1 = _22 * _22;
_9 = [9690124755525324777_usize,5_usize,0_usize,4_usize,1_usize];
_44 = [_16,_25,_25,_16,_25,_16];
_40 = _7;
(*_20) = [3807751949_u32,2069935070_u32,2218932690_u32,1023759561_u32,493378410_u32,3261747646_u32];
_8 = (*_13);
_58.2.1.1 = !_34.1.1;
_58.2.1 = (_33, _11);
(*_59) = core::ptr::addr_of_mut!((*_13));
_3 = _46 as isize;
Call(_58.2.1 = fn19(Move(_12), _34.1, Move((*_59)), _21, (*_13)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
RET = core::ptr::addr_of_mut!(_64);
_25 = _16;
Goto(bb37)
}
bb37 = {
Call(_68 = dump_var(18_usize, 21_usize, Move(_21), 3_usize, Move(_3), 26_usize, Move(_26), 17_usize, Move(_17)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Call(_68 = dump_var(18_usize, 6_usize, Move(_6), 10_usize, Move(_10), 33_usize, Move(_33), 49_usize, Move(_49)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_68 = dump_var(18_usize, 38_usize, Move(_38), 31_usize, Move(_31), 2_usize, Move(_2), 18_usize, Move(_18)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Call(_68 = dump_var(18_usize, 25_usize, Move(_25), 43_usize, Move(_43), 52_usize, Move(_52), 14_usize, Move(_14)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Call(_68 = dump_var(18_usize, 16_usize, Move(_16), 55_usize, Move(_55), 69_usize, _69, 69_usize, _69), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: &'static [i16; 4],mut _2: (u64, i16),mut _3: *mut isize,mut _4: isize,mut _5: isize) -> (u64, i16) {
mir! {
type RET = (u64, i16);
let _6: isize;
let _7: Adt64;
let _8: (u128,);
let _9: f32;
let _10: u8;
let _11: i32;
let _12: &'static *const i8;
let _13: char;
let _14: &'static (isize,);
let _15: f32;
let _16: [i64; 1];
let _17: &'static i32;
let _18: usize;
let _19: f32;
let _20: (isize,);
let _21: char;
let _22: (char, [usize; 7]);
let _23: [i64; 1];
let _24: *mut [i32; 1];
let _25: [i8; 4];
let _26: [i64; 5];
let _27: ();
let _28: ();
{
RET = (_2.0, _2.1);
_6 = _5;
RET.0 = 238_u8 as u64;
_2 = (RET.0, RET.1);
_2.1 = RET.1 + RET.1;
RET = _2;
_2.1 = 1099382463_u32 as i16;
_3 = core::ptr::addr_of_mut!(_5);
_6 = _4;
RET.1 = 2813_u16 as i16;
Call(_2.0 = core::intrinsics::bswap(RET.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET.1 = -_2.1;
RET.1 = !_2.1;
RET = _2;
RET = (_2.0, _2.1);
_3 = core::ptr::addr_of_mut!(_4);
_4 = !_5;
RET = (_2.0, _2.1);
RET.1 = !_2.1;
_5 = -(*_3);
_2 = (RET.0, RET.1);
_2 = (RET.0, RET.1);
_2 = (RET.0, RET.1);
_5 = _4;
_3 = core::ptr::addr_of_mut!((*_3));
Goto(bb2)
}
bb2 = {
_2 = (RET.0, RET.1);
_8.0 = '\u{6cf06}' as u128;
RET = _2;
_8.0 = 129388251619533067659075459106584923113_u128 ^ 205659684782285694284436659524723936757_u128;
_6 = _4;
RET.0 = !_2.0;
(*_3) = '\u{10feb6}' as isize;
RET = (_2.0, _2.1);
_6 = _5;
_11 = (-776770324_i32) << _5;
_8 = (140466483088950924711934464176150897741_u128,);
_10 = 62_u8;
_2.1 = RET.1;
_8 = (97114050684741045384729746859787267950_u128,);
_2 = (RET.0, RET.1);
RET = _2;
(*_3) = !_5;
_2 = (RET.0, RET.1);
_10 = _11 as u8;
RET.1 = !_2.1;
_2.0 = RET.0 ^ RET.0;
_6 = (-79_i8) as isize;
_9 = _10 as f32;
match _8.0 {
97114050684741045384729746859787267950 => bb3,
_ => bb1
}
}
bb3 = {
_13 = '\u{10964f}';
RET.0 = RET.1 as u64;
_4 = _5 >> _10;
_8.0 = 339722900996653732049819331658356970595_u128;
Call(_2.0 = core::intrinsics::transmute(_4), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_3 = core::ptr::addr_of_mut!(_6);
_8 = (146812089521231939898418096547378435420_u128,);
_3 = core::ptr::addr_of_mut!((*_3));
_4 = _6;
_11 = _10 as i32;
RET.0 = 2346193919566837497_i64 as u64;
_8.0 = 93300120278242538258855469326124812011_u128;
(*_3) = -_5;
_5 = _6;
_2.0 = RET.0;
_15 = (*_3) as f32;
_2 = (RET.0, RET.1);
_2 = (RET.0, RET.1);
_11 = 1550199199_i32 * 1273257949_i32;
_16 = [6820702940114740969_i64];
_8 = (330765740249007200105171936933879252294_u128,);
RET.0 = !_2.0;
RET.1 = _2.1;
_10 = true as u8;
_3 = core::ptr::addr_of_mut!(_6);
_18 = !6319368382887012266_usize;
_17 = &_11;
match _8.0 {
0 => bb1,
1 => bb2,
330765740249007200105171936933879252294 => bb6,
_ => bb5
}
}
bb5 = {
_13 = '\u{10964f}';
RET.0 = RET.1 as u64;
_4 = _5 >> _10;
_8.0 = 339722900996653732049819331658356970595_u128;
Call(_2.0 = core::intrinsics::transmute(_4), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_8.0 = 232144628778816054213230718596401298517_u128;
match _8.0 {
0 => bb1,
1 => bb2,
2 => bb3,
232144628778816054213230718596401298517 => bb7,
_ => bb4
}
}
bb7 = {
RET = _2;
_6 = _5;
(*_3) = _5;
_2 = RET;
RET = (_2.0, _2.1);
_15 = _9 * _9;
_16 = [1571290053704544303_i64];
_8.0 = (*_3) as u128;
RET = (_2.0, _2.1);
Goto(bb8)
}
bb8 = {
_8 = (57314021149445479452269850502497289994_u128,);
_13 = '\u{dac05}';
_9 = _18 as f32;
Goto(bb9)
}
bb9 = {
_5 = (*_3) | _6;
_10 = _13 as u8;
(*_3) = _5 | _5;
_11 = -1667442038_i32;
_2 = RET;
_17 = &_11;
_9 = RET.0 as f32;
_20.0 = -_6;
_11 = 8681255683031559697_i64 as i32;
_11 = 1117734263_i32 << (*_3);
_13 = '\u{f01fa}';
_21 = _13;
RET.0 = !_2.0;
_23 = [405050897913446667_i64];
RET.0 = _2.0;
_22.1 = [_18,_18,_18,_18,_18,_18,_18];
_2 = (RET.0, RET.1);
_19 = _15;
_25 = [31_i8,19_i8,91_i8,4_i8];
_17 = &_11;
_2.1 = RET.1 << (*_3);
RET.1 = _2.1 + _2.1;
_10 = _18 as u8;
Goto(bb10)
}
bb10 = {
Call(_27 = dump_var(19_usize, 25_usize, Move(_25), 18_usize, Move(_18), 4_usize, Move(_4), 21_usize, Move(_21)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_27 = dump_var(19_usize, 11_usize, Move(_11), 23_usize, Move(_23), 16_usize, Move(_16), 28_usize, _28), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box(40651_u16), std::hint::black_box(7429345450976314916490673790267076299_u128), std::hint::black_box(1681241638_u32), std::hint::black_box(13809813880514368708_u64), std::hint::black_box((-1868857863_i32)), std::hint::black_box(2331905917830782289_i64), std::hint::black_box(46290880784777127579743807827881018145_i128), std::hint::black_box(6_usize), std::hint::black_box(120_u8));
                
            }
impl PrintFDebug for Adt31{
	unsafe fn printf_debug(&self){unsafe{printf("Adt31::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt31 {
Variant0{
fld0: usize,
fld1: (u128, i128),
fld2: i32,

},
Variant1{
fld0: i64,
fld1: char,
fld2: u8,
fld3: [u32; 8],
fld4: u16,
fld5: i32,

},
Variant2{
fld0: bool,
fld1: char,
fld2: u8,
fld3: usize,
fld4: i16,
fld5: u16,
fld6: [i16; 4],

},
Variant3{
fld0: f64,
fld1: char,
fld2: i16,
fld3: u8,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: [u32; 3],
fld1: f32,

},
Variant1{
fld0: i32,
fld1: u64,

},
Variant2{
fld0: [i16; 4],
fld1: char,
fld2: isize,
fld3: i128,

},
Variant3{
fld0: (u128, i128),
fld1: *mut *mut u8,
fld2: u8,
fld3: usize,
fld4: i16,
fld5: u16,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: (u128, i128),
fld1: [u32; 8],
fld2: *mut isize,
fld3: i128,
fld4: *mut *mut u8,
fld5: i32,

},
Variant1{
fld0: usize,
fld1: *mut u8,
fld2: [u16; 4],
fld3: [u32; 3],
fld4: [i8; 4],
fld5: (isize,),
fld6: (u128, i128),
fld7: [i64; 5],

},
Variant2{
fld0: bool,
fld1: char,
fld2: [u32; 8],
fld3: [i8; 4],
fld4: [u32; 3],
fld5: f64,
fld6: *mut *mut u8,
fld7: i128,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: [usize; 5],
fld1: (u128, i128),
fld2: u16,
fld3: [i16; 4],
fld4: (u64, i16),
fld5: [u8; 6],
fld6: i64,

},
Variant1{
fld0: [u8; 7],
fld1: i16,
fld2: [i32; 1],
fld3: (u128, i128),

},
Variant2{
fld0: *mut *mut u8,
fld1: Adt50,
fld2: isize,
fld3: *mut i16,
fld4: *mut [i32; 1],
fld5: [i64; 8],
fld6: i64,
fld7: i128,

},
Variant3{
fld0: Adt44,
fld1: *mut [i32; 1],
fld2: *mut i16,

}}
impl PrintFDebug for Adt64{
	unsafe fn printf_debug(&self){unsafe{printf("Adt64::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt64 {
Variant0{
fld0: [usize; 5],
fld1: *mut i16,
fld2: f32,

},
Variant1{
fld0: i64,
fld1: Adt57,

},
Variant2{
fld0: [i32; 1],
fld1: char,
fld2: (u128, i128),
fld3: i8,
fld4: i16,
fld5: [u32; 3],
fld6: *mut [i32; 1],
fld7: *const [u32; 6],

}}
impl PrintFDebug for Adt76{
	unsafe fn printf_debug(&self){unsafe{printf("Adt76::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt76 {
Variant0{
fld0: [u32; 3],
fld1: (isize,),
fld2: (char, [usize; 7]),
fld3: usize,
fld4: [i32; 1],
fld5: [i64; 1],

},
Variant1{
fld0: f32,
fld1: [u16; 4],
fld2: [i16; 4],
fld3: [i8; 4],
fld4: [u8; 6],

}}
impl PrintFDebug for Adt82{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt82{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt82 {
fld0: [usize; 5],
fld1: [u32; 6],
fld2: [i16; 4],
fld3: *mut *mut isize,
fld4: *mut [i32; 1],
}
impl PrintFDebug for Adt86{
	unsafe fn printf_debug(&self){unsafe{printf("Adt86::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt86 {
Variant0{
fld0: *const *mut u32,
fld1: [u32; 8],

},
Variant1{
fld0: *mut f64,
fld1: *mut [i32; 1],
fld2: *const i64,
fld3: Adt57,
fld4: [i32; 1],
fld5: [u32; 3],
fld6: (u128, i128),

},
Variant2{
fld0: [i64; 8],
fld1: char,
fld2: (u128,),
fld3: *mut *const [char; 8],

},
Variant3{
fld0: (u128,),
fld1: [char; 8],
fld2: [i64; 8],
fld3: [u32; 6],
fld4: [u16; 4],
fld5: u32,

}}

