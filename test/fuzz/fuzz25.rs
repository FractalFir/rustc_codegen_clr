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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: u32,mut _5: u128,mut _6: u8,mut _7: i64,mut _8: i128) -> [u32; 2] {
mir! {
type RET = [u32; 2];
let _9: *mut &'static [u128; 1];
let _10: ([u32; 8], [u8; 4], f64);
let _11: u128;
let _12: isize;
let _13: u32;
let _14: [u8; 4];
let _15: ((f32, (u8, *mut u16)), u16, &'static u8, char);
let _16: Adt53;
let _17: u128;
let _18: &'static (f32, u128, u64, [u8; 4]);
let _19: &'static u8;
let _20: *const usize;
let _21: isize;
let _22: u32;
let _23: isize;
let _24: ();
let _25: ();
{
_8 = 11310266050312252020_u64 as i128;
_3 = (-9223372036854775808_isize) << _8;
_4 = 132020435_u32 ^ 3366070596_u32;
_6 = (-2592580102362761348_i64) as u8;
RET = [_4,_4];
_2 = '\u{1c1f7}';
Goto(bb1)
}
bb1 = {
_4 = (-1287500828808128868_i64) as u32;
_2 = '\u{a57c}';
RET = [_4,_4];
_1 = _2 < _2;
RET = [_4,_4];
_4 = (-16506_i16) as u32;
_8 = (-33604175663452773966753797787928852407_i128);
RET = [_4,_4];
_7 = (-6955146379936640755_i64);
_10.1 = [_6,_6,_6,_6];
_3 = 180134816203629591769131516730613180395_u128 as isize;
_10.0 = [_4,_4,_4,_4,_4,_4,_4,_4];
RET = [_4,_4];
_2 = '\u{51b06}';
_1 = false;
_10.2 = 24089_u16 as f64;
match _8 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
306678191257485689496620809643839359049 => bb7,
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
RET = [_4,_4];
RET = [_4,_4];
_2 = '\u{29e27}';
_5 = 49317803836589996748076832933175129785_u128 - 279784797620332770649866057570407799350_u128;
_7 = 5848802662088135913_i64 - (-3116937267763569556_i64);
RET = [_4,_4];
_11 = _5 + _5;
RET = [_4,_4];
_8 = (-160988169292497953528993855675383922376_i128) >> _11;
_5 = _11 - _11;
_10.2 = _7 as f64;
_1 = _2 != _2;
_3 = _2 as isize;
_1 = _6 != _6;
RET = [_4,_4];
_5 = _2 as u128;
_4 = 1698444553_u32 ^ 2225491311_u32;
_2 = '\u{c4e4b}';
_2 = '\u{475f6}';
_3 = !(-9223372036854775808_isize);
_10.0 = [_4,_4,_4,_4,_4,_4,_4,_4];
_10.0 = [_4,_4,_4,_4,_4,_4,_4,_4];
RET = [_4,_4];
RET = [_4,_4];
_5 = 4_usize as u128;
_5 = !_11;
Call(_12 = fn1(_3, _7, _1, _8, _1, _10.2), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_12 = _3 * _3;
Goto(bb9)
}
bb9 = {
_14 = [_6,_6,_6,_6];
_8 = _6 as i128;
_12 = !_3;
_1 = !false;
_10.1 = _14;
RET = [_4,_4];
_8 = (-83012051207424455737568459264856433544_i128);
_8 = -125934309251768738578122026731549314546_i128;
_6 = 13113618833768591563_usize as u8;
_13 = _4 + _4;
_13 = _7 as u32;
RET = [_4,_13];
Goto(bb10)
}
bb10 = {
RET = [_13,_4];
_15.1 = !47941_u16;
_15.2 = &_6;
RET = [_4,_4];
_11 = _5 ^ _5;
_15.0.1.1 = core::ptr::addr_of_mut!(_15.1);
_4 = !_13;
_2 = '\u{d89e1}';
_15.0.0 = _8 as f32;
_15.3 = _2;
RET = [_13,_13];
RET = [_13,_13];
_11 = _5;
_12 = -_3;
_3 = !_12;
_10.1 = _14;
_5 = _6 as u128;
_15.1 = 47035_u16 + 35977_u16;
_8 = _15.0.0 as i128;
_15.2 = &_15.0.1.0;
_12 = !_3;
_15.0.0 = 17920617632847860446_usize as f32;
RET = [_4,_13];
Call(_12 = core::intrinsics::bswap(_3), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_15.0.0 = _3 as f32;
_15.2 = &_6;
_14 = _10.1;
_6 = 16790590638859193477_u64 as u8;
_11 = _15.3 as u128;
Goto(bb12)
}
bb12 = {
_15.3 = _2;
_15.2 = &_6;
_3 = _12 * _12;
_11 = _5;
_15.0.1.0 = !_6;
RET = [_4,_4];
_5 = _15.3 as u128;
_8 = (-65984502411001509573692664828238943716_i128);
_10.2 = _8 as f64;
_15.0.0 = (-94_i8) as f32;
match _8 {
0 => bb1,
1 => bb5,
274297864509936953889681942603529267740 => bb14,
_ => bb13
}
}
bb13 = {
_12 = _3 * _3;
Goto(bb9)
}
bb14 = {
_10.1 = _14;
_2 = _15.3;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(0_usize, 12_usize, Move(_12), 11_usize, Move(_11), 14_usize, Move(_14), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(0_usize, 4_usize, Move(_4), 6_usize, Move(_6), 25_usize, _25, 25_usize, _25), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: isize,mut _2: i64,mut _3: bool,mut _4: i128,mut _5: bool,mut _6: f64) -> isize {
mir! {
type RET = isize;
let _7: bool;
let _8: (f64, isize, i8, i16);
let _9: ([u8; 4],);
let _10: Adt64;
let _11: bool;
let _12: [u32; 2];
let _13: ();
let _14: ();
{
_2 = 1074239416_i32 as i64;
RET = _1;
Call(_2 = core::intrinsics::bswap(6221113597603130746_i64), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = (-15829328828536542482720449123360916054_i128);
_5 = _3;
_7 = _4 == _4;
_7 = _3 ^ _3;
_8.1 = -_1;
RET = _8.1;
_8.2 = !(-64_i8);
_1 = RET;
_8.1 = RET;
_3 = _6 != _6;
Call(_8.2 = core::intrinsics::transmute(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = (_6, _1, 51_i8, 5691_i16);
_8.0 = _6 - _6;
Goto(bb3)
}
bb3 = {
_7 = !_3;
RET = _1 | _8.1;
_8.3 = 13590853115566450068_usize as i16;
RET = !_1;
_6 = -_8.0;
RET = _8.1;
_8 = (_6, _1, 55_i8, (-14042_i16));
_2 = _5 as i64;
_11 = _3;
_12 = [3282131571_u32,3392092270_u32];
RET = _8.1;
_8.2 = (-2_i8) >> _2;
_2 = 41996_u16 as i64;
_7 = !_11;
_8.0 = _8.2 as f64;
_9.0 = [123_u8,20_u8,178_u8,3_u8];
_8.1 = _8.3 as isize;
_1 = _4 as isize;
_7 = _3;
_11 = _7;
_6 = _8.0 + _8.0;
_2 = (-3552082498211489087_i64);
_1 = _8.1 << _8.3;
_8.1 = _4 as isize;
RET = _1 - _1;
_8.0 = _6;
Goto(bb4)
}
bb4 = {
Call(_13 = dump_var(1_usize, 3_usize, Move(_3), 5_usize, Move(_5), 11_usize, Move(_11), 7_usize, Move(_7)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{ae550}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box(741389611_u32), std::hint::black_box(103106683273972612946288854119180845429_u128), std::hint::black_box(229_u8), std::hint::black_box(2075199098764432360_i64), std::hint::black_box((-3945253161514724069614026974986882639_i128)));
                
            }
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: [isize; 1],
fld1: *mut *const usize,

},
Variant1{
fld0: bool,
fld1: [isize; 1],
fld2: f32,
fld3: [i32; 1],

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: [char; 8],

},
Variant1{
fld0: [i32; 8],
fld1: [i32; 1],
fld2: [char; 8],
fld3: i8,
fld4: u128,
fld5: (f64, isize, i8, i16),
fld6: i64,
fld7: [i8; 7],

},
Variant2{
fld0: [isize; 1],
fld1: u128,
fld2: [i128; 4],
fld3: [i32; 1],
fld4: *const usize,
fld5: u32,
fld6: *const *mut u16,
fld7: (i8,),

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
	Self::Variant2{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt63 {
Variant0{
fld0: [i32; 1],
fld1: *mut *const usize,
fld2: isize,
fld3: [i64; 4],
fld4: (f64, isize, i8, i16),
fld5: *mut [isize; 1],

},
Variant1{
fld0: *mut i32,

},
Variant2{
fld0: *mut u16,
fld1: [u128; 1],
fld2: isize,

}}
impl PrintFDebug for Adt64{
	unsafe fn printf_debug(&self){unsafe{printf("Adt64::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt64 {
Variant0{
fld0: *const bool,
fld1: *const isize,
fld2: [isize; 1],
fld3: u8,
fld4: [u32; 2],
fld5: [i64; 4],
fld6: ([u8; 4],),

},
Variant1{
fld0: (u8, *mut u16),
fld1: ([u8; 4],),
fld2: [isize; 1],
fld3: *mut bool,
fld4: [i32; 1],

}}
impl PrintFDebug for Adt66{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt66{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt66 {
fld0: [i128; 7],
fld1: *mut i32,
fld2: [char; 8],
fld3: Adt63,
}
impl PrintFDebug for Adt72{
	unsafe fn printf_debug(&self){unsafe{printf("Adt72::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt72 {
Variant0{
fld0: u128,

},
Variant1{
fld0: [i128; 4],
fld1: [isize; 1],
fld2: *mut i32,
fld3: [i32; 8],
fld4: *const isize,
fld5: [i64; 4],
fld6: Adt63,

}}
impl PrintFDebug for Adt76{
	unsafe fn printf_debug(&self){unsafe{printf("Adt76::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt76 {
Variant0{
fld0: *mut u16,
fld1: *mut bool,
fld2: [u128; 1],
fld3: Adt63,
fld4: u128,

},
Variant1{
fld0: u128,
fld1: [u8; 4],
fld2: (u8, *mut u16),
fld3: i8,

},
Variant2{
fld0: (u8, *mut u16),

}}
impl PrintFDebug for Adt82{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt82{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt82 {
fld0: [i8; 8],
fld1: u16,
fld2: [i32; 1],
}

