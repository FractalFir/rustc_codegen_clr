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
pub fn fn0(mut _1: i64,mut _2: i8) -> [usize; 1] {
mir! {
type RET = [usize; 1];
let _3: isize;
let _4: *mut (*mut (&'static *const u32, i64, isize, *const u128, (u16, i16, u16, usize)), u8);
let _5: bool;
let _6: [isize; 6];
let _7: u64;
let _8: (u128,);
let _9: isize;
let _10: [i8; 7];
let _11: isize;
let _12: Adt57;
let _13: isize;
let _14: ((f64, u32, isize, u32),);
let _15: i32;
let _16: f64;
let _17: u8;
let _18: ();
let _19: ();
{
RET = [13977625834457384087_usize];
RET = [18390470854850644252_usize];
_1 = 1562919688577071213_i64 ^ (-187441368644798909_i64);
_2 = 51_i8;
_3 = -(-9223372036854775808_isize);
RET = [64010875771330048_usize];
_1 = _3 as i64;
_2 = 60_i8;
RET = [2_usize];
match _2 {
0 => bb1,
1 => bb2,
2 => bb3,
60 => bb5,
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
_3 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_3 = -9223372036854775807_isize;
Goto(bb6)
}
bb6 = {
RET = [9155410969613760325_usize];
_5 = true;
_1 = !(-800539879553547003_i64);
_3 = !9223372036854775807_isize;
_1 = (-3868488016487497371_i64) * (-7833009866612759058_i64);
_6 = [_3,_3,_3,_3,_3,_3];
_5 = true & true;
_6 = [_3,_3,_3,_3,_3,_3];
_5 = false;
_6 = [_3,_3,_3,_3,_3,_3];
RET = [7_usize];
Call(RET = fn1(_1, _6, _3, _1, _6, _6, _2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_7 = 12080884198636927813_u64 | 6586934907141793109_u64;
Goto(bb8)
}
bb8 = {
RET = [0_usize];
_3 = (-9223372036854775808_isize);
RET = [588558983313975439_usize];
_5 = true;
_8.0 = !139599789912807801070498483453273323566_u128;
_6 = [_3,_3,_3,_3,_3,_3];
match _3 {
0 => bb2,
1 => bb9,
2 => bb10,
340282366920938463454151235394913435648 => bb12,
_ => bb11
}
}
bb9 = {
Return()
}
bb10 = {
RET = [9155410969613760325_usize];
_5 = true;
_1 = !(-800539879553547003_i64);
_3 = !9223372036854775807_isize;
_1 = (-3868488016487497371_i64) * (-7833009866612759058_i64);
_6 = [_3,_3,_3,_3,_3,_3];
_5 = true & true;
_6 = [_3,_3,_3,_3,_3,_3];
_5 = false;
_6 = [_3,_3,_3,_3,_3,_3];
RET = [7_usize];
Call(RET = fn1(_1, _6, _3, _1, _6, _6, _2), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
_3 = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_3 = -9223372036854775807_isize;
Goto(bb6)
}
bb12 = {
_5 = true & false;
_9 = _3 - _3;
_3 = _9;
RET = [12795520239267335774_usize];
_3 = _9;
_5 = false;
_10 = [_2,_2,_2,_2,_2,_2,_2];
RET = [11205206975783479396_usize];
_2 = 83_i8;
_6 = [_3,_9,_3,_3,_3,_9];
_2 = (-18_i8);
_1 = -7453220729745391418_i64;
_11 = 62337_u16 as isize;
_2 = 194_u8 as i8;
RET = [7_usize];
_11 = _9 >> _9;
_3 = -_11;
_13 = _11;
RET = [668149500022544029_usize];
_8 = (94318156492449641615467855185024224230_u128,);
_8 = (57931429211580003043501502045110679419_u128,);
_14.0.3 = !4294360665_u32;
_5 = !true;
_7 = 24348_i16 as u64;
Goto(bb13)
}
bb13 = {
_14.0.2 = '\u{2d22c}' as isize;
RET = [6_usize];
_10 = [_2,_2,_2,_2,_2,_2,_2];
Call(RET = core::intrinsics::transmute(_1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_7 = 13458949242380835120_u64 ^ 5269550476964433814_u64;
_2 = (-112_i8);
_1 = -5593453272851506918_i64;
_13 = '\u{1b2bb}' as isize;
Goto(bb15)
}
bb15 = {
Call(_18 = dump_var(0_usize, 1_usize, Move(_1), 3_usize, Move(_3), 6_usize, Move(_6), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_18 = dump_var(0_usize, 7_usize, Move(_7), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i64,mut _2: [isize; 6],mut _3: isize,mut _4: i64,mut _5: [isize; 6],mut _6: [isize; 6],mut _7: i8) -> [usize; 1] {
mir! {
type RET = [usize; 1];
let _8: char;
let _9: u128;
let _10: u128;
let _11: (f64, u32, isize, u32);
let _12: [i32; 6];
let _13: [isize; 6];
let _14: f32;
let _15: (f64, u32, isize, u32);
let _16: Adt43;
let _17: f32;
let _18: i16;
let _19: [i128; 6];
let _20: u16;
let _21: (u128,);
let _22: [isize; 6];
let _23: Adt51;
let _24: [i8; 7];
let _25: i16;
let _26: char;
let _27: *const *mut i32;
let _28: char;
let _29: i32;
let _30: (u128,);
let _31: (u128,);
let _32: f64;
let _33: char;
let _34: ((f64, u32, isize, u32),);
let _35: bool;
let _36: [i32; 6];
let _37: [i128; 6];
let _38: u64;
let _39: f32;
let _40: (f64, u32, isize, u32);
let _41: ();
let _42: ();
{
_7 = -(-118_i8);
_1 = _4;
_3 = (-9223372036854775808_isize) - 9223372036854775807_isize;
_4 = 265874549_u32 as i64;
RET = [11795090543832811813_usize];
_5 = [_3,_3,_3,_3,_3,_3];
RET = [6_usize];
_3 = !(-9223372036854775808_isize);
_6 = [_3,_3,_3,_3,_3,_3];
_6 = _5;
RET = [7_usize];
_9 = 317210236081098727336199257727166788098_u128 ^ 325420921323126408494747534583379030763_u128;
_8 = '\u{e2ceb}';
_3 = 127_u8 as isize;
_4 = -_1;
_8 = '\u{b5a20}';
_6 = [_3,_3,_3,_3,_3,_3];
Goto(bb1)
}
bb1 = {
_7 = (-128204774_i32) as i8;
_1 = _4 + _4;
Goto(bb2)
}
bb2 = {
_8 = '\u{82ed4}';
Goto(bb3)
}
bb3 = {
_1 = _9 as i64;
_6 = [_3,_3,_3,_3,_3,_3];
_6 = [_3,_3,_3,_3,_3,_3];
_1 = 10153_i16 as i64;
_11.0 = (-18103838460791182737458930766237730383_i128) as f64;
_1 = !_4;
_7 = 30903_u16 as i8;
RET = [4_usize];
Goto(bb4)
}
bb4 = {
_11.2 = _3;
_5 = [_3,_11.2,_11.2,_11.2,_3,_3];
_10 = _9 >> _4;
_11.1 = !2813904799_u32;
_11.3 = !_11.1;
_6 = [_3,_11.2,_11.2,_3,_3,_11.2];
_13 = [_11.2,_3,_3,_11.2,_3,_11.2];
_10 = _9 + _9;
_2 = _13;
_11.2 = !_3;
_10 = _9 + _9;
_1 = -_4;
_7 = !65_i8;
_11.1 = !_11.3;
_2 = [_11.2,_11.2,_3,_3,_3,_3];
_4 = (-8734_i16) as i64;
_3 = !_11.2;
_2 = [_3,_11.2,_11.2,_3,_11.2,_3];
_6 = [_3,_3,_11.2,_11.2,_3,_11.2];
_12 = [(-182027597_i32),1234214827_i32,(-988405329_i32),1181109925_i32,(-1108254681_i32),1173684514_i32];
_1 = _4 | _4;
_1 = false as i64;
_11.2 = _3 & _3;
RET = [17603962047957472290_usize];
Call(_2 = fn2(_13, _12), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_8 = '\u{100236}';
_13 = [_11.2,_3,_11.2,_11.2,_3,_11.2];
RET = [16249072902566419232_usize];
_6 = _13;
_11.3 = _1 as u32;
_1 = -_4;
_8 = '\u{d2de4}';
_11.2 = _3;
_2 = _13;
Call(_7 = fn6(_9, _9, _3, _12, _13, _11.2, _2, _3, _13, _10, _1, _8), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_10 = _11.0 as u128;
_9 = _11.0 as u128;
_9 = _10 >> _10;
_2 = [_3,_11.2,_11.2,_11.2,_3,_11.2];
_14 = 9453_i16 as f32;
_5 = [_11.2,_11.2,_11.2,_11.2,_11.2,_3];
_13 = [_3,_3,_3,_3,_3,_3];
_11.0 = 6797064297891648220_usize as f64;
_10 = _9;
_11.1 = !_11.3;
_15.3 = _11.1;
_15 = (_11.0, _11.1, _3, _11.3);
Goto(bb7)
}
bb7 = {
Goto(bb8)
}
bb8 = {
_11.1 = _11.3;
_3 = _11.2;
_11.2 = -_15.2;
_13 = [_15.2,_3,_11.2,_3,_15.2,_3];
_11.1 = _15.1;
_6 = _2;
_15.3 = _11.1;
_8 = '\u{780a0}';
_11.0 = _15.0 - _15.0;
_18 = !18542_i16;
_11 = (_15.0, _15.1, _15.2, _15.1);
_14 = 6460951422917509115_usize as f32;
_9 = _14 as u128;
_15.1 = _18 as u32;
_8 = '\u{a1d00}';
_22 = _2;
_17 = _14;
_15 = (_11.0, _11.3, _3, _11.1);
_3 = _8 as isize;
RET = [4_usize];
_9 = _10;
_22 = [_15.2,_15.2,_3,_15.2,_11.2,_11.2];
Goto(bb9)
}
bb9 = {
_7 = (-20_i8);
_20 = 32745_u16;
_11.2 = -_3;
_9 = !_10;
_14 = 137_u8 as f32;
_13 = [_3,_15.2,_15.2,_15.2,_15.2,_15.2];
_21 = (_10,);
_22 = _5;
_19 = [(-153462227673225397228192175068900545564_i128),(-60976722563970287128188979445310986688_i128),(-119816860881064055793106483014131791980_i128),(-28993429177432000151154209999612377595_i128),(-130935492892533193694784389690406895770_i128),76850044555678704587123522982848289351_i128];
_13 = [_15.2,_3,_3,_3,_3,_3];
_11.2 = _15.2;
RET = [0_usize];
match _20 {
0 => bb7,
1 => bb2,
32745 => bb10,
_ => bb6
}
}
bb10 = {
_11.0 = (-39693663552202495472111747747553625239_i128) as f64;
_1 = false as i64;
_25 = _18;
_3 = !_11.2;
_11.0 = -_15.0;
_6 = [_15.2,_15.2,_15.2,_15.2,_3,_3];
RET = [15139285354644356569_usize];
RET = [574990771841019291_usize];
_11.1 = 81835695469951311731539821037835768042_i128 as u32;
_26 = _8;
RET = [16844181799583201432_usize];
_21.0 = _10;
_6 = _13;
_15.1 = !_15.3;
Goto(bb11)
}
bb11 = {
_11 = _15;
_10 = !_21.0;
_32 = (-63700583304534382464360000284655663763_i128) as f64;
_13 = [_11.2,_11.2,_15.2,_3,_15.2,_3];
_11.0 = 16115948807347561809_usize as f64;
_20 = _8 as u16;
_17 = _14 + _14;
_19 = [(-109441156775321364094077102086946864977_i128),327180300031575931875060495371868760_i128,39069925307118270263298798229433856995_i128,148579871836176421295509494036723471748_i128,149668707440867944112765059273118128283_i128,(-132550614602255648364434596387600332705_i128)];
_11.1 = !_11.3;
_16 = Adt43::Variant0 { fld0: true };
_30.0 = _10;
match _7 {
0 => bb1,
1 => bb10,
2 => bb6,
3 => bb4,
340282366920938463463374607431768211436 => bb13,
_ => bb12
}
}
bb12 = {
_11.0 = (-39693663552202495472111747747553625239_i128) as f64;
_1 = false as i64;
_25 = _18;
_3 = !_11.2;
_11.0 = -_15.0;
_6 = [_15.2,_15.2,_15.2,_15.2,_3,_3];
RET = [15139285354644356569_usize];
RET = [574990771841019291_usize];
_11.1 = 81835695469951311731539821037835768042_i128 as u32;
_26 = _8;
RET = [16844181799583201432_usize];
_21.0 = _10;
_6 = _13;
_15.1 = !_15.3;
Goto(bb11)
}
bb13 = {
_11.2 = -_15.2;
_26 = _8;
_35 = !false;
_6 = [_11.2,_3,_11.2,_15.2,_11.2,_11.2];
_34.0.0 = 130917438252513601850230984881377602672_i128 as f64;
place!(Field::<bool>(Variant(_16, 0), 0)) = _11.2 <= _15.2;
_11.0 = _32 + _15.0;
_12 = [(-384776930_i32),620055080_i32,(-506825454_i32),(-209549272_i32),723120725_i32,1142583028_i32];
_30.0 = _25 as u128;
_34.0.3 = _15.1 + _15.1;
_15.1 = _34.0.3 & _11.3;
_31 = _21;
_28 = _26;
_31.0 = _10 << _25;
Goto(bb14)
}
bb14 = {
_36 = _12;
_38 = 13859599879587604745_u64 >> _15.1;
_28 = _26;
_14 = _17;
_15.2 = _11.2 & _11.2;
RET = [3_usize];
_20 = _28 as u16;
_4 = _1 - _1;
SetDiscriminant(_16, 0);
_34.0.3 = _11.3;
_16 = Adt43::Variant0 { fld0: _35 };
_30 = (_9,);
_29 = 2042583426_i32 - (-884881381_i32);
_9 = _10;
_17 = -_14;
_4 = _1;
_8 = _28;
_26 = _8;
_30 = _31;
_34.0 = (_32, _15.1, _3, _15.1);
_14 = _17 - _17;
_29 = 56_u8 as i32;
_3 = _34.0.2;
_39 = -_14;
_15.1 = _15.3 - _11.3;
_2 = _22;
_9 = _1 as u128;
_19 = [24527975278824982901925559973546734313_i128,109722404420218407785934837356557934692_i128,(-81740225825048250669055360469996118809_i128),72887723231752080484421688133381504022_i128,(-128391724482902871859767235042650837014_i128),(-62523574183458751811236807280283421222_i128)];
RET = [2_usize];
_30 = (_9,);
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(1_usize, 18_usize, Move(_18), 25_usize, Move(_25), 8_usize, Move(_8), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(1_usize, 36_usize, Move(_36), 13_usize, Move(_13), 12_usize, Move(_12), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(1_usize, 31_usize, Move(_31), 6_usize, Move(_6), 7_usize, Move(_7), 22_usize, Move(_22)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(1_usize, 19_usize, Move(_19), 42_usize, _42, 42_usize, _42, 42_usize, _42), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: [isize; 6],mut _2: [i32; 6]) -> [isize; 6] {
mir! {
type RET = [isize; 6];
let _3: (u16, i16, u16, usize);
let _4: char;
let _5: bool;
let _6: (bool,);
let _7: isize;
let _8: u32;
let _9: f32;
let _10: *const *mut i32;
let _11: f32;
let _12: *const *mut i32;
let _13: (u128,);
let _14: isize;
let _15: ((f64, u32, isize, u32),);
let _16: u128;
let _17: (u128,);
let _18: Adt42;
let _19: isize;
let _20: Adt45;
let _21: Adt46;
let _22: ();
let _23: ();
{
_2 = [154124531_i32,107194984_i32,(-625654999_i32),324728149_i32,(-631782667_i32),(-154123105_i32)];
RET = _1;
Call(RET = core::intrinsics::transmute(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [(-54_isize),9223372036854775807_isize,9223372036854775807_isize,122_isize,9223372036854775807_isize,9223372036854775807_isize];
RET = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-109_isize)];
RET = _1;
_2 = [1394146589_i32,885623411_i32,738150301_i32,(-544104490_i32),304360688_i32,605712575_i32];
_2 = [2086220237_i32,1770962772_i32,(-647712247_i32),(-1275289633_i32),(-150149897_i32),63910572_i32];
_3.3 = 3729494345672707879_usize;
_3.0 = 36_i8 as u16;
_3 = (13611_u16, (-1155_i16), 33447_u16, 4_usize);
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,119_isize];
Goto(bb2)
}
bb2 = {
RET = [(-9223372036854775808_isize),8_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_3.2 = (-276282643_i32) as u16;
_3.1 = 13863_i16;
_5 = !true;
_6 = (_5,);
_3.3 = 31905525227118394888844523804252192504_u128 as usize;
RET = _1;
_3.1 = 585_i16 & (-8342_i16);
_3.1 = -32136_i16;
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,53_isize,(-88_isize),9223372036854775807_isize];
_5 = _3.2 >= _3.0;
_4 = '\u{cf60b}';
_4 = '\u{3b697}';
_6 = (_5,);
_7 = 9223372036854775807_isize;
_6 = (_5,);
_1 = RET;
_3 = (42905_u16, 23668_i16, 38151_u16, 5_usize);
_3.1 = 55226407664035134_u64 as i16;
_3.0 = 1646951501_i32 as u16;
_7 = -(-69_isize);
RET = [_7,_7,_7,_7,_7,_7];
match _3.2 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
38151 => bb8,
_ => bb7
}
}
bb3 = {
RET = [(-54_isize),9223372036854775807_isize,9223372036854775807_isize,122_isize,9223372036854775807_isize,9223372036854775807_isize];
RET = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-109_isize)];
RET = _1;
_2 = [1394146589_i32,885623411_i32,738150301_i32,(-544104490_i32),304360688_i32,605712575_i32];
_2 = [2086220237_i32,1770962772_i32,(-647712247_i32),(-1275289633_i32),(-150149897_i32),63910572_i32];
_3.3 = 3729494345672707879_usize;
_3.0 = 36_i8 as u16;
_3 = (13611_u16, (-1155_i16), 33447_u16, 4_usize);
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,119_isize];
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
_4 = '\u{8e5fb}';
_7 = !(-3_isize);
_3.0 = _3.2;
_6 = (_5,);
_6 = (_5,);
_3.1 = 2501_i16 - 15504_i16;
_4 = '\u{4626b}';
Goto(bb9)
}
bb9 = {
_9 = 292477020297312772125277258741039104449_u128 as f32;
_3.0 = _3.2;
_1 = [_7,_7,_7,_7,_7,_7];
_3 = (4850_u16, 32339_i16, 63665_u16, 6_usize);
_8 = !193321423_u32;
_6.0 = _8 == _8;
_11 = _9;
_11 = _9 * _9;
RET = [_7,_7,_7,_7,_7,_7];
_3.0 = _3.2 / _3.2;
_3.2 = _3.0;
_3.2 = !_3.0;
RET = _1;
_6 = (_5,);
_3.2 = (-148377835759700499214784208107179089286_i128) as u16;
_5 = _6.0 ^ _6.0;
_3.3 = _9 as usize;
_6.0 = !_5;
_3.0 = _3.2 ^ _3.2;
_7 = (-159818760877167082044306916809913638846_i128) as isize;
_11 = _9;
_3.0 = 808186136_i32 as u16;
_8 = !1947652805_u32;
_3 = (36548_u16, 13631_i16, 10711_u16, 1_usize);
RET = _1;
_11 = _9 - _9;
_3 = (24142_u16, (-2283_i16), 33336_u16, 16031116338740099805_usize);
_8 = 1490842552_u32 << _3.2;
_4 = '\u{250ef}';
RET = _1;
Goto(bb10)
}
bb10 = {
_2 = [1225353693_i32,829575566_i32,(-589359933_i32),469539730_i32,1957822494_i32,804105500_i32];
_11 = _9;
_4 = '\u{25aa3}';
_11 = _9 * _9;
_13.0 = 11748126141453918159747909023257093707_u128;
RET = [_7,_7,_7,_7,_7,_7];
RET = _1;
_8 = 3135574969_u32 & 1119362031_u32;
_7 = !(-9223372036854775808_isize);
_3.3 = 4312211582780328854_usize - 1_usize;
_7 = !(-109_isize);
_14 = !_7;
_13 = (331675935865744735673573980229328404455_u128,);
_5 = !_6.0;
_9 = -_11;
_6 = (_5,);
_7 = _14 + _14;
_13.0 = _8 as u128;
_3.2 = _3.0 & _3.0;
Call(_11 = fn3(_3.1, _5), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_3.0 = _3.2 - _3.2;
_13.0 = _5 as u128;
_7 = _14;
_13 = (15215401539005174958176824759998477905_u128,);
_3.2 = _3.0 & _3.0;
_5 = _6.0 >= _6.0;
_3.2 = _3.0;
RET = [_7,_14,_14,_7,_7,_14];
_3.3 = !4_usize;
_5 = _6.0;
_2 = [1434416181_i32,984399023_i32,(-1033076327_i32),661486402_i32,(-311855409_i32),1644390938_i32];
_15.0.3 = _9 as u32;
_8 = _3.1 as u32;
_9 = -_11;
_14 = -_7;
_7 = -_14;
match _3.1 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb10,
4 => bb12,
340282366920938463463374607431768209173 => bb14,
_ => bb13
}
}
bb12 = {
Return()
}
bb13 = {
RET = [(-9223372036854775808_isize),8_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_3.2 = (-276282643_i32) as u16;
_3.1 = 13863_i16;
_5 = !true;
_6 = (_5,);
_3.3 = 31905525227118394888844523804252192504_u128 as usize;
RET = _1;
_3.1 = 585_i16 & (-8342_i16);
_3.1 = -32136_i16;
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,53_isize,(-88_isize),9223372036854775807_isize];
_5 = _3.2 >= _3.0;
_4 = '\u{cf60b}';
_4 = '\u{3b697}';
_6 = (_5,);
_7 = 9223372036854775807_isize;
_6 = (_5,);
_1 = RET;
_3 = (42905_u16, 23668_i16, 38151_u16, 5_usize);
_3.1 = 55226407664035134_u64 as i16;
_3.0 = 1646951501_i32 as u16;
_7 = -(-69_isize);
RET = [_7,_7,_7,_7,_7,_7];
match _3.2 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
38151 => bb8,
_ => bb7
}
}
bb14 = {
_15.0.0 = _3.2 as f64;
_6.0 = _5;
_15.0.1 = _8 + _8;
_3 = (30980_u16, (-6498_i16), 35445_u16, 10340936569867670822_usize);
_4 = '\u{dbc6f}';
_16 = _13.0;
_18.fld1.3 = _3.1 as u32;
_3 = (37422_u16, (-955_i16), 28112_u16, 0_usize);
_3.0 = _3.2;
_15.0.0 = _11 as f64;
_16 = _3.1 as u128;
_17.0 = (-140154113984895444417594547135386082214_i128) as u128;
_19 = (-4518240266140139810_i64) as isize;
_18.fld1.2 = !_7;
_1 = [_18.fld1.2,_7,_14,_19,_7,_18.fld1.2];
_8 = _15.0.1 >> _7;
RET = [_7,_18.fld1.2,_7,_19,_19,_18.fld1.2];
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(2_usize, 8_usize, Move(_8), 6_usize, Move(_6), 3_usize, Move(_3), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_22 = dump_var(2_usize, 13_usize, Move(_13), 2_usize, Move(_2), 23_usize, _23, 23_usize, _23), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: i16,mut _2: bool) -> f32 {
mir! {
type RET = f32;
let _3: ((f64, u32, isize, u32),);
let _4: f32;
let _5: bool;
let _6: bool;
let _7: Adt45;
let _8: f64;
let _9: [i128; 6];
let _10: usize;
let _11: bool;
let _12: (f64, u32, isize, u32);
let _13: ((f64, u32, isize, u32),);
let _14: Adt56;
let _15: i64;
let _16: ();
let _17: ();
{
RET = 125_isize as f32;
RET = 19910_u16 as f32;
_2 = true & true;
RET = 11033196954156076372_u64 as f32;
_2 = true;
RET = 10822940785873878242_u64 as f32;
_2 = !false;
_2 = _1 == _1;
_2 = false | false;
_2 = true & true;
RET = 225613278136523120359383611844567034280_u128 as f32;
_1 = (-26666_i16) * (-27906_i16);
_1 = 1654_i16 * (-6859_i16);
RET = 250505690_u32 as f32;
RET = 13135_u16 as f32;
_3.0.3 = 664779970_u32;
_3.0.2 = 9223372036854775807_isize + (-9223372036854775808_isize);
_2 = RET >= RET;
_3.0.2 = 9223372036854775807_isize;
_2 = _3.0.2 > _3.0.2;
_3.0.1 = !_3.0.3;
_3.0.2 = !9223372036854775807_isize;
match _3.0.3 {
0 => bb1,
1 => bb2,
664779970 => bb4,
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
_4 = RET;
_3.0.2 = (-9223372036854775808_isize);
_2 = !true;
_3.0.1 = !_3.0.3;
_2 = true;
_3.0.3 = _3.0.1;
_3.0.3 = _3.0.1 & _3.0.1;
_6 = _2;
_8 = _3.0.2 as f64;
Goto(bb5)
}
bb5 = {
match _3.0.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
5 => bb8,
340282366920938463454151235394913435648 => bb10,
_ => bb9
}
}
bb6 = {
_4 = RET;
_3.0.2 = (-9223372036854775808_isize);
_2 = !true;
_3.0.1 = !_3.0.3;
_2 = true;
_3.0.3 = _3.0.1;
_3.0.3 = _3.0.1 & _3.0.1;
_6 = _2;
_8 = _3.0.2 as f64;
Goto(bb5)
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
_3.0.3 = !_3.0.1;
RET = _4 * _4;
_3.0.3 = _3.0.1;
_3.0.2 = 9223372036854775807_isize;
_3.0 = (_8, 718523830_u32, (-9223372036854775808_isize), 89571009_u32);
RET = _4 + _4;
Goto(bb11)
}
bb11 = {
_5 = !_6;
RET = _4 + _4;
_3.0.2 = -(-9223372036854775808_isize);
_3.0 = (_8, 2098401668_u32, 37_isize, 3170463843_u32);
_9 = [28252008951190617983576668866785615858_i128,46998512586970742984198324018180808755_i128,137860838040204251648091653423429746845_i128,(-6119956969457782363318276545695518635_i128),(-81592772229061594505304595246499999960_i128),137760374569796818887046412130152021920_i128];
_3.0.2 = !9223372036854775807_isize;
_3.0.2 = 43429_u16 as isize;
_10 = 3_usize * 15514913589892979254_usize;
_8 = _3.0.0 * _3.0.0;
_12.0 = _3.0.2 as f64;
_3.0.0 = _4 as f64;
_1 = (-2572599853261230460_i64) as i16;
_11 = _5;
_12.1 = !_3.0.3;
_10 = 0_usize - 0_usize;
_9 = [(-165712761446715251468968524409408552284_i128),(-82126654116987236269345461620232798900_i128),166324658319654755171225530943427497115_i128,(-25056796583284715770682471900577377071_i128),(-57632061845871254393948354675494259416_i128),(-11337064610048020511967342714995015623_i128)];
_3.0.1 = _12.1 % _3.0.3;
_5 = _6;
_10 = !0_usize;
match _3.0.3 {
0 => bb9,
1 => bb7,
3170463843 => bb12,
_ => bb5
}
}
bb12 = {
_12.3 = !_3.0.3;
_12.2 = 4229_u16 as isize;
_12.1 = 52315701612167732474003029027351043249_u128 as u32;
_3.0.1 = !_3.0.3;
_9 = [3005354570879771265818702038521411309_i128,131156600969841245707743776939316873739_i128,(-51046207869871773646498875762409627575_i128),(-141309198948810289108623869647578780302_i128),(-59658686719314412485983646104373663353_i128),(-124331670463809678380488054303763527560_i128)];
_3.0.3 = !_12.3;
_15 = (-117196003817442118596820586695060006157_i128) as i64;
_13 = _3;
_15 = (-1743386752677187181_i64);
RET = -_4;
_4 = -RET;
_9 = [132450529314404263757096094326947299621_i128,155915679484789719426952470690102991945_i128,62674886417299468478185464910437470109_i128,(-150204481411644674758287257096934623485_i128),(-21285068287235976921436877368535742172_i128),89457063172332615621213631253225266493_i128];
Call(_3.0.2 = fn4(_13.0.1, _12.3, _12, _13.0.1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_13.0.1 = _1 as u32;
_13.0.1 = _3.0.1;
_9 = [(-19352193673136402791901238169207312875_i128),(-139277869679557144457766136907205016698_i128),170136177529406917519302671450100822027_i128,(-89901792925213720542338089736581035965_i128),39024728176050097364743312348375580464_i128,(-143442943670333866945159157443412237206_i128)];
RET = -_4;
_11 = !_5;
_11 = !_6;
_13 = _3;
_12.2 = _3.0.2;
Goto(bb14)
}
bb14 = {
_3.0.2 = !_13.0.2;
_1 = (-12963_i16);
Goto(bb15)
}
bb15 = {
Call(_16 = dump_var(3_usize, 2_usize, Move(_2), 1_usize, Move(_1), 5_usize, Move(_5), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: u32,mut _2: u32,mut _3: (f64, u32, isize, u32),mut _4: u32) -> isize {
mir! {
type RET = isize;
let _5: u128;
let _6: (u128,);
let _7: f64;
let _8: i128;
let _9: char;
let _10: Adt55;
let _11: isize;
let _12: (*mut (&'static *const u32, i64, isize, *const u128, (u16, i16, u16, usize)), u8);
let _13: *const ((f64, u32, isize, u32),);
let _14: bool;
let _15: ();
let _16: ();
{
_3.0 = _3.2 as f64;
_3.2 = !(-96_isize);
RET = 16101125490404179399_u64 as isize;
RET = false as isize;
RET = _3.2 >> _4;
_3.2 = 22809_i16 as isize;
_3.3 = !_1;
_3.2 = RET + RET;
_5 = !44978897420450727431439886919318346964_u128;
_3.1 = (-35710220491169067272010176474605577366_i128) as u32;
_5 = 3_usize as u128;
Goto(bb1)
}
bb1 = {
_6 = (_5,);
Goto(bb2)
}
bb2 = {
_3.2 = -RET;
_8 = (-79293050567283495691456830831841889351_i128);
_5 = _6.0;
_3.3 = _1;
RET = !_3.2;
_1 = '\u{108b88}' as u32;
_3.3 = _4 >> RET;
match _8 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
260989316353654967771917776599926322105 => bb8,
_ => bb7
}
}
bb3 = {
_6 = (_5,);
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
_8 = (-91151886688032802570362670914497318964_i128);
_7 = _3.1 as f64;
_8 = (-167340892247791414658176993462441761125_i128);
_3.3 = _1 * _4;
Call(_3 = fn5(_2, _4, _1, _8, _7), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_9 = '\u{4ae76}';
RET = -_3.2;
_9 = '\u{100e8e}';
_1 = _8 as u32;
_4 = _3.3 >> _2;
_11 = _3.2 >> _1;
_3.0 = 5_usize as f64;
_9 = '\u{4e3fb}';
_1 = _5 as u32;
_5 = _6.0;
_5 = _6.0;
_3.2 = RET;
_5 = !_6.0;
_3.1 = (-126_i8) as u32;
_3.3 = _4;
RET = _3.0 as isize;
_10 = Adt55::Variant1 { fld0: _3.0,fld1: 93_i8 };
_3.2 = _4 as isize;
_3.0 = _7 + Field::<f64>(Variant(_10, 1), 0);
_6.0 = !_5;
_12.1 = 20_u8 | 230_u8;
_9 = '\u{46045}';
_11 = _3.2 | RET;
_5 = _6.0;
_4 = !_2;
RET = _11 + _11;
_14 = !false;
Goto(bb10)
}
bb10 = {
Call(_15 = dump_var(4_usize, 5_usize, Move(_5), 14_usize, Move(_14), 6_usize, Move(_6), 4_usize, Move(_4)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: u32,mut _2: u32,mut _3: u32,mut _4: i128,mut _5: f64) -> (f64, u32, isize, u32) {
mir! {
type RET = (f64, u32, isize, u32);
let _6: *const &'static *const u32;
let _7: [usize; 1];
let _8: f32;
let _9: u128;
let _10: (bool,);
let _11: ();
let _12: ();
{
RET.3 = (-9223372036854775808_isize) as u32;
_2 = (-7553598580929047236_i64) as u32;
RET.1 = _2 - _1;
RET = (_5, _1, (-9223372036854775808_isize), _1);
RET.1 = _1 | _1;
_1 = !RET.1;
_1 = !RET.1;
_4 = !(-65246002929701383804957130488805388301_i128);
RET.1 = !_1;
_8 = RET.2 as f32;
RET.0 = (-144123257_i32) as f64;
RET.3 = 206121307515780173261319776163565138310_u128 as u32;
RET = (_5, _2, 9223372036854775807_isize, _1);
RET.1 = _1;
RET = (_5, _3, 107_isize, _1);
RET.3 = RET.1;
RET.3 = _1 ^ _1;
_10.0 = false & true;
_1 = RET.1;
RET.1 = (-7636664833599422923_i64) as u32;
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(5_usize, 2_usize, Move(_2), 3_usize, Move(_3), 12_usize, _12, 12_usize, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: u128,mut _2: u128,mut _3: isize,mut _4: [i32; 6],mut _5: [isize; 6],mut _6: isize,mut _7: [isize; 6],mut _8: isize,mut _9: [isize; 6],mut _10: u128,mut _11: i64,mut _12: char) -> i8 {
mir! {
type RET = i8;
let _13: (u16, i16, u16, usize);
let _14: (u16, i16, u16, usize);
let _15: i16;
let _16: (f64, u32, isize, u32);
let _17: [isize; 6];
let _18: i64;
let _19: *const u32;
let _20: i8;
let _21: bool;
let _22: (u128,);
let _23: [i32; 6];
let _24: *const u32;
let _25: [isize; 6];
let _26: ();
let _27: ();
{
_5 = [_3,_8,_6,_3,_8,_8];
_7 = _9;
RET = (-96_i8) | 108_i8;
_13 = (39631_u16, (-11788_i16), 47806_u16, 7_usize);
match _13.3 {
0 => bb1,
1 => bb2,
7 => bb4,
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
_13 = (57246_u16, 30685_i16, 12942_u16, 5522729836663168979_usize);
_1 = _13.3 as u128;
_5 = _9;
_13.1 = (-9814_i16) - (-8155_i16);
_8 = _3;
_13.1 = 10317_i16;
_13 = (29761_u16, 32314_i16, 40762_u16, 5_usize);
RET = !(-74_i8);
_14.3 = 129838257_u32 as usize;
_3 = _6 ^ _6;
_11 = 7499928597746497466_i64;
_14.1 = !_13.1;
_14.2 = _13.2;
_14.0 = _13.0 / _13.2;
_16.2 = _6 - _6;
_14.0 = _13.2;
_16.3 = !4258313887_u32;
Call(_8 = fn7(_4, _2, _13.1, _12, _1, _13.3, _14, _13.1, _9, _13.0, _13.3, _14.2, _14.3, _14), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_13.0 = _11 as u16;
_14.1 = _6 as i16;
_12 = '\u{167fa}';
_19 = core::ptr::addr_of!(_16.3);
_6 = _3 >> _13.2;
_12 = '\u{c3064}';
_16.0 = 1473227590437103902_u64 as f64;
_13.1 = _14.1 ^ _14.1;
_5 = [_8,_8,_6,_8,_3,_6];
_16.3 = !3407875913_u32;
(*_19) = !1542918994_u32;
match _14.2 {
0 => bb6,
1 => bb7,
2 => bb8,
40762 => bb10,
_ => bb9
}
}
bb6 = {
_13 = (57246_u16, 30685_i16, 12942_u16, 5522729836663168979_usize);
_1 = _13.3 as u128;
_5 = _9;
_13.1 = (-9814_i16) - (-8155_i16);
_8 = _3;
_13.1 = 10317_i16;
_13 = (29761_u16, 32314_i16, 40762_u16, 5_usize);
RET = !(-74_i8);
_14.3 = 129838257_u32 as usize;
_3 = _6 ^ _6;
_11 = 7499928597746497466_i64;
_14.1 = !_13.1;
_14.2 = _13.2;
_14.0 = _13.0 / _13.2;
_16.2 = _6 - _6;
_14.0 = _13.2;
_16.3 = !4258313887_u32;
Call(_8 = fn7(_4, _2, _13.1, _12, _1, _13.3, _14, _13.1, _9, _13.0, _13.3, _14.2, _14.3, _14), ReturnTo(bb5), UnwindUnreachable())
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
_16.0 = 12815257046053992490_u64 as f64;
_10 = _1 ^ _1;
_20 = RET >> _13.1;
_11 = -8312549240019088948_i64;
(*_19) = !2203144139_u32;
_16.1 = !(*_19);
_15 = _13.3 as i16;
_14.1 = _13.0 as i16;
_20 = -RET;
_5 = _7;
_13.2 = _14.0 & _14.2;
RET = _20 & _20;
_13.0 = _13.2;
Goto(bb11)
}
bb11 = {
_14.0 = _13.0 + _13.2;
_24 = core::ptr::addr_of!(_16.1);
match _14.2 {
0 => bb3,
1 => bb12,
40762 => bb14,
_ => bb13
}
}
bb12 = {
_13 = (57246_u16, 30685_i16, 12942_u16, 5522729836663168979_usize);
_1 = _13.3 as u128;
_5 = _9;
_13.1 = (-9814_i16) - (-8155_i16);
_8 = _3;
_13.1 = 10317_i16;
_13 = (29761_u16, 32314_i16, 40762_u16, 5_usize);
RET = !(-74_i8);
_14.3 = 129838257_u32 as usize;
_3 = _6 ^ _6;
_11 = 7499928597746497466_i64;
_14.1 = !_13.1;
_14.2 = _13.2;
_14.0 = _13.0 / _13.2;
_16.2 = _6 - _6;
_14.0 = _13.2;
_16.3 = !4258313887_u32;
Call(_8 = fn7(_4, _2, _13.1, _12, _1, _13.3, _14, _13.1, _9, _13.0, _13.3, _14.2, _14.3, _14), ReturnTo(bb5), UnwindUnreachable())
}
bb13 = {
_13.0 = _11 as u16;
_14.1 = _6 as i16;
_12 = '\u{167fa}';
_19 = core::ptr::addr_of!(_16.3);
_6 = _3 >> _13.2;
_12 = '\u{c3064}';
_16.0 = 1473227590437103902_u64 as f64;
_13.1 = _14.1 ^ _14.1;
_5 = [_8,_8,_6,_8,_3,_6];
_16.3 = !3407875913_u32;
(*_19) = !1542918994_u32;
match _14.2 {
0 => bb6,
1 => bb7,
2 => bb8,
40762 => bb10,
_ => bb9
}
}
bb14 = {
_24 = _19;
_22.0 = !_10;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(6_usize, 4_usize, Move(_4), 11_usize, Move(_11), 9_usize, Move(_9), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(6_usize, 12_usize, Move(_12), 1_usize, Move(_1), 3_usize, Move(_3), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: [i32; 6],mut _2: u128,mut _3: i16,mut _4: char,mut _5: u128,mut _6: usize,mut _7: (u16, i16, u16, usize),mut _8: i16,mut _9: [isize; 6],mut _10: u16,mut _11: usize,mut _12: u16,mut _13: usize,mut _14: (u16, i16, u16, usize)) -> isize {
mir! {
type RET = isize;
let _15: [usize; 1];
let _16: Adt43;
let _17: f32;
let _18: i32;
let _19: Adt52;
let _20: f64;
let _21: ((f64, u32, isize, u32),);
let _22: u32;
let _23: *const *mut i32;
let _24: (u128,);
let _25: i128;
let _26: Adt53;
let _27: isize;
let _28: ();
let _29: ();
{
_7.0 = _6 as u16;
_9[_6] = !(-78_isize);
RET = -_9[_6];
_12 = _6 as u16;
_8 = _7.1 & _14.1;
_9 = [RET,RET,RET,RET,RET,RET];
_12 = _7.2;
_14.2 = !_10;
_6 = _14.3 + _14.3;
RET = 403265127_u32 as isize;
_6 = (-4081801516639132249_i64) as usize;
_15 = [_6];
_14.0 = _14.1 as u16;
_14.1 = _7.1;
_3 = _5 as i16;
_14 = (_7.0, _7.1, _7.0, _11);
_7 = _14;
_7.0 = !_14.0;
_14.1 = (-165657998979001265089735130537259854692_i128) as i16;
RET = _9[_11];
_9 = [RET,RET,RET,RET,RET,RET];
_15 = [_14.3];
_7.3 = _14.3;
_11 = _14.2 as usize;
_15 = [_7.3];
Goto(bb1)
}
bb1 = {
_3 = _7.1;
_14.2 = _14.0 + _10;
_7 = (_14.2, _3, _14.2, _14.3);
_14.0 = _14.2;
_7.3 = _11;
_16 = Adt43::Variant0 { fld0: true };
_3 = true as i16;
place!(Field::<bool>(Variant(_16, 0), 0)) = false | false;
_4 = '\u{29419}';
_14.2 = !_14.0;
_14.3 = _6;
_14.3 = _7.3;
_10 = !_12;
_7.1 = _8 & _3;
_13 = _14.3 - _11;
_15 = [_11];
_4 = '\u{1a95f}';
_10 = _14.0 + _14.2;
_6 = _7.2 as usize;
_19.fld5 = (Field::<bool>(Variant(_16, 0), 0),);
RET = 24_isize * (-9223372036854775808_isize);
_19.fld5.0 = Field::<bool>(Variant(_16, 0), 0) | Field::<bool>(Variant(_16, 0), 0);
_21.0.1 = 2404329253_u32 + 900157044_u32;
_14.2 = _10;
_19.fld6 = (-123_i8) as u16;
_3 = !_8;
match _12 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
40762 => bb10,
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
_18 = (-1360933103_i32);
_7.0 = _14.2;
_8 = _3 << _3;
Goto(bb11)
}
bb11 = {
_19.fld7 = core::ptr::addr_of!(_24.0);
_22 = _21.0.1 ^ _21.0.1;
_15 = [_7.3];
Call(_19.fld4 = fn8(_1, _15, _7.0, _14, _7.0, _7.0, _8, _7.0, _6, _14.0, _3, _10, Move(_16), _14.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_21.0.0 = RET as f64;
_15 = [_11];
_20 = _18 as f64;
_7.1 = _8;
_17 = (-7641039591767137182_i64) as f32;
_5 = !_2;
_10 = !_7.2;
_17 = _3 as f32;
_24.0 = !_5;
_21.0.2 = -RET;
_3 = _7.1;
_12 = 121_u8 as u16;
match _18 {
340282366920938463463374607430407278353 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_21.0 = (_20, _22, RET, _22);
_6 = _4 as usize;
_2 = !_24.0;
_14 = (_7.0, _8, _7.2, _13);
_12 = _7.0;
_6 = !_14.3;
_17 = _8 as f32;
_7.0 = _14.2;
_2 = _14.1 as u128;
_12 = !_7.2;
_14.1 = -_8;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(7_usize, 12_usize, Move(_12), 24_usize, Move(_24), 8_usize, Move(_8), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(7_usize, 2_usize, Move(_2), 3_usize, Move(_3), 6_usize, Move(_6), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(7_usize, 1_usize, Move(_1), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [i32; 6],mut _2: [usize; 1],mut _3: u16,mut _4: (u16, i16, u16, usize),mut _5: u16,mut _6: u16,mut _7: i16,mut _8: u16,mut _9: usize,mut _10: u16,mut _11: i16,mut _12: u16,mut _13: Adt43,mut _14: u16) -> *const u32 {
mir! {
type RET = *const u32;
let _15: (f64, u32, isize, u32);
let _16: i128;
let _17: (&'static *const u32, i64, isize, *const u128, (u16, i16, u16, usize));
let _18: u64;
let _19: [i128; 6];
let _20: [i8; 7];
let _21: f32;
let _22: (f64, u32, isize, u32);
let _23: i128;
let _24: [isize; 6];
let _25: *mut i32;
let _26: Adt55;
let _27: [i64; 3];
let _28: *mut i32;
let _29: Adt51;
let _30: i64;
let _31: ((f64, u32, isize, u32),);
let _32: [i128; 6];
let _33: (bool,);
let _34: ();
let _35: ();
{
_10 = _5;
_4.1 = -_11;
_4.1 = 4294118222900482570_i64 as i16;
SetDiscriminant(_13, 0);
_4.1 = _11 * _7;
_17.2 = -9223372036854775807_isize;
_16 = 1766557102_u32 as i128;
_12 = _10;
_17.4.0 = !_8;
_4.2 = _10;
_17.4.0 = !_4.2;
_15.0 = _17.2 as f64;
RET = core::ptr::addr_of!(_15.3);
_17.4.1 = -_4.1;
_17.4.3 = !_9;
_4.2 = _8 & _17.4.0;
_11 = _17.4.1 + _17.4.1;
Call((*RET) = fn9(_17.4.0, _5, _14, _10, _7, _5, _17.4.1, _17.4.3, _4, _17.4.3, _14, _12), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*RET) = 395223609_u32;
_4.3 = _17.4.3;
_14 = !_10;
_12 = !_8;
RET = core::ptr::addr_of!((*RET));
_3 = _17.2 as u16;
match _15.3 {
0 => bb2,
395223609 => bb4,
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
_20 = [11_i8,(-101_i8),(-20_i8),9_i8,90_i8,107_i8,(-68_i8)];
_4.2 = _8;
_17.4.2 = !_14;
_4.2 = !_14;
_15.2 = !_17.2;
RET = core::ptr::addr_of!((*RET));
_12 = _17.4.0 | _4.0;
_13 = Adt43::Variant0 { fld0: true };
_15.1 = (*RET);
(*RET) = _15.1 ^ _15.1;
place!(Field::<bool>(Variant(_13, 0), 0)) = true;
Call(_4.1 = core::intrinsics::transmute(_7), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_7 = !_4.1;
_22 = _15;
_4.3 = _11 as usize;
_3 = !_6;
_20 = [110_i8,58_i8,79_i8,(-51_i8),(-105_i8),48_i8,(-52_i8)];
RET = core::ptr::addr_of!(_15.1);
_23 = 45543402996872920175880886812001279275_u128 as i128;
_3 = _4.2 - _12;
_18 = !1106022018452848211_u64;
_1 = [(-382489066_i32),(-1748397764_i32),(-189301280_i32),(-1369101010_i32),(-1695755871_i32),1396244101_i32];
_24 = [_22.2,_22.2,_17.2,_15.2,_22.2,_22.2];
_26 = Adt55::Variant3 { fld0: _18 };
_22.1 = _23 as u32;
_4.0 = (-3208735975824912197_i64) as u16;
place!(Field::<u64>(Variant(_26, 3), 0)) = _18 & _18;
_17.4 = (_6, _4.1, _5, _4.3);
match (*RET) {
0 => bb4,
1 => bb6,
2 => bb7,
3 => bb8,
395223609 => bb10,
_ => bb9
}
}
bb6 = {
_20 = [11_i8,(-101_i8),(-20_i8),9_i8,90_i8,107_i8,(-68_i8)];
_4.2 = _8;
_17.4.2 = !_14;
_4.2 = !_14;
_15.2 = !_17.2;
RET = core::ptr::addr_of!((*RET));
_12 = _17.4.0 | _4.0;
_13 = Adt43::Variant0 { fld0: true };
_15.1 = (*RET);
(*RET) = _15.1 ^ _15.1;
place!(Field::<bool>(Variant(_13, 0), 0)) = true;
Call(_4.1 = core::intrinsics::transmute(_7), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
(*RET) = 395223609_u32;
_4.3 = _17.4.3;
_14 = !_10;
_12 = !_8;
RET = core::ptr::addr_of!((*RET));
_3 = _17.2 as u16;
match _15.3 {
0 => bb2,
395223609 => bb4,
_ => bb3
}
}
bb10 = {
_15.3 = _22.3;
_4.2 = _14;
_13 = Adt43::Variant0 { fld0: false };
Call(_29 = fn12(_11, _17.4.2, _6, _8, _17.4, _4.3), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_31.0 = (_15.0, _22.3, _15.2, _15.3);
_8 = Field::<u16>(Variant(_29, 3), 2) * _5;
_6 = _3 << Field::<u16>(Variant(_29, 3), 2);
_13 = Adt43::Variant0 { fld0: true };
_15.3 = !(*RET);
_9 = _17.4.3;
_28 = Field::<*mut i32>(Variant(_29, 3), 0);
_8 = Field::<u16>(Variant(_29, 3), 2);
_15.2 = _22.2;
_1 = [235664586_i32,(-859278145_i32),1203852795_i32,869549933_i32,(-1411430502_i32),(-411793122_i32)];
match _15.1 {
0 => bb5,
1 => bb2,
2 => bb12,
395223609 => bb14,
_ => bb13
}
}
bb12 = {
_7 = !_4.1;
_22 = _15;
_4.3 = _11 as usize;
_3 = !_6;
_20 = [110_i8,58_i8,79_i8,(-51_i8),(-105_i8),48_i8,(-52_i8)];
RET = core::ptr::addr_of!(_15.1);
_23 = 45543402996872920175880886812001279275_u128 as i128;
_3 = _4.2 - _12;
_18 = !1106022018452848211_u64;
_1 = [(-382489066_i32),(-1748397764_i32),(-189301280_i32),(-1369101010_i32),(-1695755871_i32),1396244101_i32];
_24 = [_22.2,_22.2,_17.2,_15.2,_22.2,_22.2];
_26 = Adt55::Variant3 { fld0: _18 };
_22.1 = _23 as u32;
_4.0 = (-3208735975824912197_i64) as u16;
place!(Field::<u64>(Variant(_26, 3), 0)) = _18 & _18;
_17.4 = (_6, _4.1, _5, _4.3);
match (*RET) {
0 => bb4,
1 => bb6,
2 => bb7,
3 => bb8,
395223609 => bb10,
_ => bb9
}
}
bb13 = {
(*RET) = 395223609_u32;
_4.3 = _17.4.3;
_14 = !_10;
_12 = !_8;
RET = core::ptr::addr_of!((*RET));
_3 = _17.2 as u16;
match _15.3 {
0 => bb2,
395223609 => bb4,
_ => bb3
}
}
bb14 = {
SetDiscriminant(_29, 0);
place!(Field::<bool>(Variant(_13, 0), 0)) = !true;
place!(Field::<((f64, u32, isize, u32),)>(Variant(_29, 0), 3)).0 = (_22.0, _22.1, _22.2, _31.0.1);
_4 = _17.4;
_30 = -2221168064129361543_i64;
_4.3 = _9 ^ _17.4.3;
_4.0 = !_8;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(8_usize, 30_usize, Move(_30), 12_usize, Move(_12), 2_usize, Move(_2), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(8_usize, 3_usize, Move(_3), 14_usize, Move(_14), 20_usize, Move(_20), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(8_usize, 23_usize, Move(_23), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: u16,mut _2: u16,mut _3: u16,mut _4: u16,mut _5: i16,mut _6: u16,mut _7: i16,mut _8: usize,mut _9: (u16, i16, u16, usize),mut _10: usize,mut _11: u16,mut _12: u16) -> u32 {
mir! {
type RET = u32;
let _13: isize;
let _14: [usize; 1];
let _15: i64;
let _16: (u128,);
let _17: i8;
let _18: i128;
let _19: (f64, u32, isize, u32);
let _20: f32;
let _21: isize;
let _22: [i32; 6];
let _23: isize;
let _24: isize;
let _25: isize;
let _26: ();
let _27: ();
{
_10 = _8 + _8;
_3 = _9.2;
_7 = 278045796651648709376699015413586166904_u128 as i16;
_9 = (_11, _5, _12, _10);
_12 = !_6;
_7 = _5 | _5;
_10 = !_9.3;
RET = 3222745049_u32 * 1006964030_u32;
RET = 4262676635_u32 + 2915328644_u32;
_10 = 76_u8 as usize;
RET = !2331310258_u32;
_2 = !_6;
_14 = [_8];
_13 = (-1250784518_i32) as isize;
_4 = _3 >> _2;
_9.1 = 3689682847601434524307127971879883751_i128 as i16;
_14 = [_9.3];
_15 = (-5400580118759025464_i64);
_10 = _9.3;
_9.0 = _4 << _6;
RET = 3145539935_u32 * 2731211139_u32;
Goto(bb1)
}
bb1 = {
_9.0 = 3615451049349919212_u64 as u16;
_4 = _9.2;
_14 = [_10];
_5 = -_7;
_3 = _6;
_9.0 = false as u16;
_9.1 = _7 | _7;
_13 = -(-9223372036854775808_isize);
_7 = !_9.1;
_16.0 = !102596163813521663904491646146943907438_u128;
Call(_18 = fn10(_7, _4, _9.1, _2, _10, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_19.2 = -_13;
match _15 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
340282366920938463457974027313009185992 => bb11,
_ => bb10
}
}
bb3 = {
_9.0 = 3615451049349919212_u64 as u16;
_4 = _9.2;
_14 = [_10];
_5 = -_7;
_3 = _6;
_9.0 = false as u16;
_9.1 = _7 | _7;
_13 = -(-9223372036854775808_isize);
_7 = !_9.1;
_16.0 = !102596163813521663904491646146943907438_u128;
Call(_18 = fn10(_7, _4, _9.1, _2, _10, _3), ReturnTo(bb2), UnwindUnreachable())
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
Return()
}
bb11 = {
_19.0 = _18 as f64;
_10 = _9.3 << _9.1;
_19.0 = RET as f64;
_2 = _18 as u16;
_19.2 = -_13;
_9.3 = _10;
RET = 4219325497_u32;
_6 = (-388227668_i32) as u16;
RET = _15 as u32;
match _15 {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463457974027313009185992 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_19.2 = _13 ^ _13;
_19.3 = !RET;
_19.2 = !_13;
_19.1 = !RET;
_9.0 = _11 | _2;
_4 = _2;
_17 = (-90_i8) << _9.2;
_5 = 66_u8 as i16;
_19.0 = _18 as f64;
_6 = _4;
RET = _19.1;
_19.1 = RET;
_20 = _15 as f32;
_8 = '\u{a189c}' as usize;
_16.0 = 126500640303612515836074630543664824448_u128;
_9.3 = _10 | _10;
RET = _15 as u32;
_8 = 10624844475955214404_u64 as usize;
_16 = (19344222587480144045533322748020478569_u128,);
_19.0 = _7 as f64;
Call(_11 = fn11(_18, _18, _6), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_14 = [_9.3];
_8 = 3933349258191460542_u64 as usize;
_11 = _2 * _4;
_22 = [1493912582_i32,(-1864781388_i32),198836848_i32,492067687_i32,(-73643083_i32),(-1630969575_i32)];
_1 = _11 >> _11;
RET = !_19.1;
_21 = _19.2;
RET = _19.1;
_7 = _9.1;
_19.1 = !_19.3;
_12 = _11 * _9.0;
_22 = [(-456357996_i32),(-2026413559_i32),613226049_i32,(-630213571_i32),1230230856_i32,792526495_i32];
_2 = !_9.0;
_24 = !_19.2;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(9_usize, 11_usize, Move(_11), 7_usize, Move(_7), 16_usize, Move(_16), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(9_usize, 15_usize, Move(_15), 2_usize, Move(_2), 14_usize, Move(_14), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_26 = dump_var(9_usize, 18_usize, Move(_18), 8_usize, Move(_8), 27_usize, _27, 27_usize, _27), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: i16,mut _2: u16,mut _3: i16,mut _4: u16,mut _5: usize,mut _6: u16) -> i128 {
mir! {
type RET = i128;
let _7: u32;
let _8: *const (u128,);
let _9: char;
let _10: u32;
let _11: [usize; 1];
let _12: i128;
let _13: u64;
let _14: [i8; 7];
let _15: (u16, i16, u16, usize);
let _16: *const (u128,);
let _17: [i8; 7];
let _18: ();
let _19: ();
{
_1 = -_3;
RET = 76906380539226860530347232971988337519_i128;
_7 = 3501005493_u32 * 3531688655_u32;
_1 = _3 ^ _3;
_9 = '\u{8ddcf}';
_2 = !_6;
RET = (-110611204287005683461277251841785486943_i128) + 29831717063853632606394937424636925330_i128;
RET = !73326586634718405679219902688609788836_i128;
_5 = !16407532042047857002_usize;
_9 = '\u{9326f}';
_4 = _6;
_10 = _7 ^ _7;
_4 = _6 & _2;
_9 = '\u{a19cc}';
_7 = !_10;
_2 = _4 + _4;
_4 = (-8455448650384673821_i64) as u16;
_5 = !1_usize;
_1 = 18010978872507187576_u64 as i16;
RET = 14775789123961450045_u64 as i128;
Call(_11 = core::intrinsics::transmute(_5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_14 = [45_i8,(-10_i8),(-87_i8),(-96_i8),(-108_i8),103_i8,63_i8];
Goto(bb2)
}
bb2 = {
RET = (-159706976897076575072393511229929515648_i128) >> _2;
_11 = [_5];
_14 = [16_i8,(-81_i8),107_i8,(-71_i8),95_i8,31_i8,(-58_i8)];
_12 = RET;
_5 = !1274987843964478115_usize;
RET = _12;
_10 = _7 & _7;
_13 = 3550585189506498644_i64 as u64;
_13 = 263859797781515234_u64 | 3623254250815580407_u64;
_15.3 = (-3341402252707778778_i64) as usize;
_13 = !2578453208329102128_u64;
_4 = false as u16;
_14 = [(-92_i8),68_i8,12_i8,(-56_i8),(-4_i8),4_i8,(-69_i8)];
_11 = [_5];
_12 = !RET;
_11 = [_5];
_6 = _12 as u16;
_14 = [(-88_i8),54_i8,109_i8,(-118_i8),38_i8,9_i8,55_i8];
_9 = '\u{4d0e9}';
_15.1 = _3 ^ _3;
_14 = [40_i8,1_i8,2_i8,(-6_i8),(-126_i8),(-6_i8),(-43_i8)];
_2 = _6;
_4 = _6 - _2;
_4 = !_2;
_15 = (_4, _3, _6, _5);
_15.0 = _4 & _6;
_3 = true as i16;
Goto(bb3)
}
bb3 = {
Call(_18 = dump_var(10_usize, 1_usize, Move(_1), 5_usize, Move(_5), 11_usize, Move(_11), 4_usize, Move(_4)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_18 = dump_var(10_usize, 12_usize, Move(_12), 13_usize, Move(_13), 10_usize, Move(_10), 19_usize, _19), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: i128,mut _2: i128,mut _3: u16) -> u16 {
mir! {
type RET = u16;
let _4: ((f64, u32, isize, u32),);
let _5: ();
let _6: ();
{
RET = _3 ^ _3;
_1 = 2539248709_u32 as i128;
_1 = -_2;
_1 = _2 & _2;
_2 = _1 | _1;
Goto(bb1)
}
bb1 = {
Call(_5 = dump_var(11_usize, 1_usize, Move(_1), 6_usize, _6, 6_usize, _6, 6_usize, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: i16,mut _2: u16,mut _3: u16,mut _4: u16,mut _5: (u16, i16, u16, usize),mut _6: usize) -> Adt51 {
mir! {
type RET = Adt51;
let _7: bool;
let _8: *const &'static *const u32;
let _9: bool;
let _10: usize;
let _11: (f64, u32, isize, u32);
let _12: isize;
let _13: u64;
let _14: (bool,);
let _15: isize;
let _16: [i64; 3];
let _17: f32;
let _18: ((f64, u32, isize, u32),);
let _19: (u16, i16, u16, usize);
let _20: [isize; 6];
let _21: [i8; 7];
let _22: isize;
let _23: usize;
let _24: [usize; 1];
let _25: ((f64, u32, isize, u32),);
let _26: usize;
let _27: Adt53;
let _28: (u128,);
let _29: isize;
let _30: Adt52;
let _31: *const *mut i32;
let _32: i64;
let _33: f32;
let _34: (f64, u32, isize, u32);
let _35: [i8; 7];
let _36: (f64, u32, isize, u32);
let _37: u8;
let _38: [isize; 6];
let _39: i128;
let _40: f32;
let _41: Adt56;
let _42: [i64; 3];
let _43: bool;
let _44: char;
let _45: i16;
let _46: Adt50;
let _47: (bool,);
let _48: (u16, i16, u16, usize);
let _49: usize;
let _50: bool;
let _51: isize;
let _52: Adt44;
let _53: char;
let _54: f64;
let _55: f64;
let _56: bool;
let _57: (u128,);
let _58: Adt57;
let _59: Adt48;
let _60: ((f64, u32, isize, u32),);
let _61: bool;
let _62: isize;
let _63: [i64; 3];
let _64: i16;
let _65: usize;
let _66: i128;
let _67: bool;
let _68: *const *const (u128,);
let _69: *mut (*mut (&'static *const u32, i64, isize, *const u128, (u16, i16, u16, usize)), u8);
let _70: f32;
let _71: (u16, i16, u16, usize);
let _72: [i64; 3];
let _73: bool;
let _74: (bool,);
let _75: isize;
let _76: f64;
let _77: i64;
let _78: [isize; 6];
let _79: ((f64, u32, isize, u32),);
let _80: Adt56;
let _81: [i128; 6];
let _82: f64;
let _83: [isize; 6];
let _84: [i32; 6];
let _85: ();
let _86: ();
{
_5 = (_3, _1, _4, _6);
_3 = !_2;
_3 = 1150544674_u32 as u16;
_5.0 = _4;
_7 = _1 == _5.1;
_5.2 = _2;
Goto(bb1)
}
bb1 = {
_5.1 = _1 + _1;
Call(_4 = fn13(_7, _2, _5.1, _7, _3, _5.2, _5.3, _5.3, _5.3, _5.2, _5.1, _5.0, _7, _5, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = (_2, _1, _4, _6);
_6 = _5.3;
_5.3 = _6;
_7 = _1 == _5.1;
_5.0 = 3888798708121916156936257438084711310_u128 as u16;
_9 = _2 < _5.2;
_2 = _1 as u16;
_11.0 = (-26_i8) as f64;
_10 = 2754382795_u32 as usize;
_11.3 = 90826102705246572116988088420883398989_i128 as u32;
_3 = !_4;
Call(_10 = core::intrinsics::bswap(_6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10 = _6;
Goto(bb4)
}
bb4 = {
_11.2 = 9223372036854775807_isize | 94_isize;
_6 = !_5.3;
_12 = !_11.2;
_5.2 = _4;
_6 = _5.3;
_5.2 = 4301156819534757720_u64 as u16;
_7 = _5.3 >= _6;
_11.1 = !_11.3;
_10 = 236900789952097475458347047324524112856_u128 as usize;
Goto(bb5)
}
bb5 = {
_5.1 = _9 as i16;
_7 = _9;
_11.3 = _5.3 as u32;
_1 = '\u{99365}' as i16;
_5.0 = _3 + _4;
Goto(bb6)
}
bb6 = {
_1 = _5.1;
_2 = _3 & _4;
_11.0 = 6509730309957031625_u64 as f64;
_11.2 = !_12;
_12 = 108_i8 as isize;
_6 = _5.3;
Goto(bb7)
}
bb7 = {
_7 = !_9;
_5 = (_2, _1, _4, _6);
_5 = (_3, _1, _2, _6);
_13 = _4 as u64;
_11.1 = _11.3 * _11.3;
Call(_5.3 = core::intrinsics::bswap(_10), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_15 = _11.2;
_14 = (_7,);
_5 = (_2, _1, _4, _6);
_11.1 = !_11.3;
_13 = 3346200749429913499_u64 * 10302557926472201142_u64;
_16 = [(-4953665591954084637_i64),(-7140092501053241051_i64),3647946534758881999_i64];
_16 = [6594233225857995380_i64,(-2151089175311302726_i64),(-7646394955087125519_i64)];
_9 = _14.0 ^ _7;
_11.0 = 274284842895716731433389808312740155821_u128 as f64;
_11.2 = _15;
_11.1 = _11.3;
_9 = _7;
_5.3 = !_6;
_7 = _14.0;
_11.0 = (-3488697067878281904_i64) as f64;
_7 = _5.2 == _5.0;
_14 = (_7,);
_18.0.0 = -_11.0;
_17 = (-824712168_i32) as f32;
_18.0.1 = _17 as u32;
_6 = !_10;
Goto(bb9)
}
bb9 = {
_3 = (-1387629047_i32) as u16;
_18.0.3 = !_11.3;
_14 = (_9,);
_18.0 = _11;
_7 = _5.2 > _4;
_14 = (_7,);
_18.0.0 = _11.0 - _11.0;
_19.2 = _4 - _5.0;
_13 = _5.1 as u64;
_21 = [125_i8,11_i8,(-22_i8),(-94_i8),61_i8,47_i8,(-103_i8)];
_11.0 = _18.0.0 - _18.0.0;
_19.2 = _11.1 as u16;
_18 = (_11,);
_19.1 = _1 - _1;
_18.0 = (_11.0, _11.1, _12, _11.1);
Goto(bb10)
}
bb10 = {
_5.2 = _5.0 >> _19.1;
_1 = _15 as i16;
_25.0.0 = _11.0;
_25.0.3 = _18.0.1;
_1 = _5.1 << _19.1;
_23 = _5.3 + _5.3;
_19.0 = _5.0 << _13;
_18.0.0 = _15 as f64;
_11.2 = _15;
_26 = _23;
_1 = _5.1 & _5.1;
_23 = _26 << _2;
_11.0 = -_18.0.0;
_25.0.2 = _19.1 as isize;
Call(_19 = fn14(_14, _1, _4, _5.2, _26), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_18.0.1 = 82_u8 as u32;
_18.0 = _11;
Call(_25.0 = fn15(_19.1, _7, _13, _5.1, _2), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_25.0.2 = _15;
_11.3 = _18.0.1;
_16 = [(-6927463288038413291_i64),1109745313066267172_i64,6747063896261189369_i64];
_25.0.0 = _18.0.0;
_6 = _19.3;
_1 = _19.1 >> _23;
_16 = [2982709827559010249_i64,(-625759881708720174_i64),(-1031818496655470026_i64)];
_16 = [2836193609396669685_i64,(-7003215512759148584_i64),711109632406132779_i64];
_21 = [(-92_i8),61_i8,(-63_i8),(-114_i8),(-103_i8),(-17_i8),(-95_i8)];
_18.0 = _11;
_1 = _19.1 - _5.1;
_25.0.1 = _25.0.0 as u32;
_24 = [_23];
_19.2 = '\u{6e4b5}' as u16;
_23 = !_26;
_22 = 42_u8 as isize;
_13 = (-7_i8) as u64;
_17 = _19.1 as f32;
_18.0 = _11;
_12 = _11.2;
_11.3 = (-819809869_i32) as u32;
_24 = [_19.3];
_5.0 = _18.0.1 as u16;
Call(_18.0.2 = core::intrinsics::bswap(_25.0.2), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_4 = _18.0.3 as u16;
_13 = !10195997491091869429_u64;
_12 = _15 - _11.2;
_3 = _18.0.2 as u16;
_10 = _23;
_12 = !_11.2;
_2 = _5.2 << _19.3;
_25.0.0 = -_18.0.0;
_15 = (-354805941_i32) as isize;
_30.fld6 = !_2;
_20 = [_15,_25.0.2,_15,_12,_11.2,_11.2];
_11.2 = _18.0.2;
_30.fld4 = core::ptr::addr_of!(_18.0.3);
_19.1 = _1 << _10;
_13 = !9166696393355575896_u64;
_30.fld0 = 105872571290824158958906162467941729895_u128 >> _10;
_21 = [51_i8,122_i8,2_i8,(-39_i8),64_i8,(-128_i8),71_i8];
_14.0 = _7;
_5.0 = !_2;
_19.3 = _11.0 as usize;
Goto(bb14)
}
bb14 = {
_19.3 = !_23;
_32 = (-1092507016_i32) as i64;
_23 = _26;
Goto(bb15)
}
bb15 = {
_25.0.2 = !_15;
_20 = [_25.0.2,_22,_12,_18.0.2,_18.0.2,_11.2];
_33 = _17 * _17;
_14 = (_7,);
_18.0.0 = _13 as f64;
_30.fld2 = [(-91_i8),(-36_i8),40_i8,1_i8,115_i8,120_i8,(-66_i8)];
_11.1 = !_18.0.3;
_11.1 = _13 as u32;
_25.0.3 = _18.0.3;
_30.fld5 = (_14.0,);
_30.fld7 = core::ptr::addr_of!(_30.fld0);
_30.fld5.0 = _5.0 <= _5.2;
_25.0.2 = _12 >> _5.1;
_11 = (_25.0.0, _18.0.3, _25.0.2, _18.0.1);
_2 = _5.2 | _30.fld6;
_29 = !_11.2;
_30.fld1 = Adt46::Variant0 { fld0: _17,fld1: 557645924_i32 };
_34.0 = _13 as f64;
_11 = (_34.0, _18.0.3, _29, _18.0.1);
_30.fld5.0 = _33 <= _33;
Call(_19 = fn16(Field::<f32>(Variant(_30.fld1, 0), 0), _9, _5, _30.fld5.0, _11, _10, _7, _18, _30.fld6, _29, _11.3, _25.0, _25.0), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_30.fld6 = !_19.0;
_21 = [(-48_i8),85_i8,101_i8,(-7_i8),76_i8,64_i8,(-26_i8)];
_34.2 = !_29;
_30.fld5 = (_7,);
_36.0 = _19.2 as f64;
Call(_10 = core::intrinsics::transmute(_34.2), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
_18.0.2 = _25.0.2 + _25.0.2;
_33 = -Field::<f32>(Variant(_30.fld1, 0), 0);
_28.0 = _30.fld0;
_11.1 = _18.0.1;
_36.2 = _11.2;
Goto(bb18)
}
bb18 = {
place!(Field::<i32>(Variant(_30.fld1, 0), 1)) = (-82507752_i32) >> _18.0.1;
_5.0 = !_4;
_24 = [_10];
_23 = _6;
_30.fld3 = Adt49::Variant0 { fld0: _29,fld1: Field::<f32>(Variant(_30.fld1, 0), 0) };
place!(Field::<i32>(Variant(_30.fld1, 0), 1)) = _5.1 as i32;
_11.1 = !_18.0.1;
_19.0 = _18.0.1 as u16;
_18.0.0 = -_36.0;
_18.0.1 = _25.0.3 - _25.0.3;
_30.fld6 = _19.0;
_18.0.1 = (-34534521272319269033264167018539182276_i128) as u32;
_5.2 = _2 + _30.fld6;
_33 = Field::<f32>(Variant(_30.fld1, 0), 0) - Field::<f32>(Variant(_30.fld3, 0), 1);
_5.0 = !_19.2;
_34 = (_36.0, _11.3, _36.2, _11.1);
_20 = [_34.2,_29,Field::<isize>(Variant(_30.fld3, 0), 0),_34.2,_36.2,_29];
_16 = [_32,_32,_32];
_22 = _25.0.2;
Call(_30.fld2 = fn17(_18.0, _18.0.3, _28.0, _34, _2, _30.fld3, _6, _25.0.2, _19.1, _22, _23, _29, _25.0, Field::<f32>(Variant(_30.fld3, 0), 1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
_30.fld6 = _2;
_7 = _9 ^ _9;
_11.2 = (-71_i8) as isize;
_19 = (_5.2, _1, _2, _23);
_2 = _4 >> _34.3;
Goto(bb20)
}
bb20 = {
_1 = _19.1;
_5.0 = _19.2 & _2;
_10 = _23 | _19.3;
_36.3 = _11.3 >> _22;
_30.fld1 = Adt46::Variant0 { fld0: _17,fld1: 551485438_i32 };
_21 = [(-100_i8),(-14_i8),(-48_i8),(-116_i8),12_i8,(-3_i8),(-67_i8)];
_36 = (_34.0, _34.3, _29, _25.0.3);
_35 = _30.fld2;
_33 = _17;
_9 = _14.0 & _30.fld5.0;
_43 = _9;
_25 = (_18.0,);
_38 = _20;
Goto(bb21)
}
bb21 = {
_34.1 = !_36.1;
place!(Field::<f32>(Variant(_30.fld3, 0), 1)) = -_33;
_34.2 = _36.2 & _36.2;
_48.1 = _5.1 | _1;
place!(Field::<isize>(Variant(_30.fld3, 0), 0)) = -_25.0.2;
_33 = 2_i8 as f32;
_48.2 = _5.0;
_25.0.2 = -_29;
_1 = _48.1;
_19.0 = _5.2;
_25.0.3 = _36.3 ^ _11.3;
_11.1 = !_25.0.3;
_28 = (_30.fld0,);
_34.1 = _25.0.3;
_47 = (_14.0,);
_23 = _10 | _6;
_35 = [(-1_i8),93_i8,(-110_i8),117_i8,(-95_i8),(-18_i8),(-93_i8)];
_36.2 = _29;
_36.1 = _32 as u32;
_23 = _5.3;
_25.0.0 = _34.0 * _36.0;
_30.fld0 = !_28.0;
_34.2 = _18.0.2;
_18 = _25;
_11.0 = -_18.0.0;
_18.0.3 = _34.1 ^ _11.1;
_30.fld5 = (_43,);
Call(_16 = fn18(_47, _25.0, _34, _19.0, _28, _18, _7), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
_28.0 = _30.fld0;
_19.1 = (-105144012048705966570563587574823694318_i128) as i16;
SetDiscriminant(_30.fld3, 0);
_36 = (_25.0.0, _11.1, _18.0.2, _34.1);
_5 = (_2, _1, _19.2, _23);
_22 = _29;
place!(Field::<isize>(Variant(_30.fld3, 0), 0)) = -_34.2;
_43 = _30.fld5.0;
_50 = _43 | _47.0;
Goto(bb23)
}
bb23 = {
_36 = (_34.0, _34.3, _29, _34.3);
_45 = _48.1;
_52.fld2 = Field::<isize>(Variant(_30.fld3, 0), 0) * _25.0.2;
_18 = (_36,);
Call(_51 = core::intrinsics::bswap(_25.0.2), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
place!(Field::<f32>(Variant(_30.fld3, 0), 1)) = Field::<f32>(Variant(_30.fld1, 0), 0) + Field::<f32>(Variant(_30.fld1, 0), 0);
_11.0 = _36.0 * _18.0.0;
_18.0.2 = Field::<isize>(Variant(_30.fld3, 0), 0);
_34.1 = _48.1 as u32;
_52.fld1.0 = core::ptr::addr_of!(_18);
_18 = (_36,);
_52.fld4 = !_13;
_29 = !Field::<isize>(Variant(_30.fld3, 0), 0);
_11.0 = -_25.0.0;
_1 = _45 & _48.1;
_54 = _25.0.0 * _18.0.0;
_39 = !(-72191854572187597372032194181029341018_i128);
_36.1 = _36.3 & _11.1;
_48.0 = _32 as u16;
_18.0.0 = -_34.0;
_57 = (_28.0,);
_44 = '\u{c8e15}';
_17 = _2 as f32;
_39 = 12996542957862934418994777998884079841_i128 >> _30.fld6;
Goto(bb25)
}
bb25 = {
_1 = _45;
_52.fld4 = _48.1 as u64;
_19.1 = !_1;
Goto(bb26)
}
bb26 = {
_48.0 = !_48.2;
_52.fld0 = _16;
_19.0 = _5.2;
_18.0.1 = _36.1 | _36.3;
_60.0.1 = _18.0.1;
_52.fld3 = !_39;
_57.0 = !_28.0;
Goto(bb27)
}
bb27 = {
SetDiscriminant(_30.fld3, 2);
place!(Field::<Adt42>(Variant(_30.fld3, 2), 2)).fld1.0 = _19.3 as f64;
_25.0 = (_36.0, _60.0.1, _34.2, _11.3);
place!(Field::<Adt42>(Variant(_30.fld3, 2), 2)).fld1.1 = _36.1 - _11.1;
_19.3 = _36.3 as usize;
place!(Field::<Adt42>(Variant(_30.fld3, 2), 2)).fld0 = _32;
_11.0 = -_54;
_48.3 = _39 as usize;
_60.0.2 = _29;
_53 = _44;
_63 = [Field::<Adt42>(Variant(_30.fld3, 2), 2).fld0,Field::<Adt42>(Variant(_30.fld3, 2), 2).fld0,_32];
_33 = _17 + Field::<f32>(Variant(_30.fld1, 0), 0);
_52.fld1.0 = core::ptr::addr_of!(_25);
_9 = _14.0;
_11.0 = _36.0 * Field::<Adt42>(Variant(_30.fld3, 2), 2).fld1.0;
_29 = _36.0 as isize;
place!(Field::<Adt42>(Variant(_30.fld3, 2), 2)).fld2 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_30.fld1, 0), 1)));
_67 = _47.0 | _47.0;
_60.0.3 = _30.fld5.0 as u32;
_11 = (_34.0, _25.0.1, _29, Field::<Adt42>(Variant(_30.fld3, 2), 2).fld1.1);
_30.fld1 = Adt46::Variant0 { fld0: _17,fld1: 1695080593_i32 };
Call(_11.1 = core::intrinsics::bswap(_60.0.1), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
_21 = [(-67_i8),(-106_i8),(-50_i8),(-33_i8),39_i8,62_i8,38_i8];
_60 = (_11,);
_34.1 = _36.3 >> _26;
_55 = _11.0;
_52.fld3 = _39 & _39;
_46 = Adt50::Variant1 { fld0: _25,fld1: _57.0,fld2: _52.fld0,fld3: (-1555587134_i32),fld4: _24 };
_62 = _34.2;
_36.1 = _25.0.3 >> _36.3;
place!(Field::<Adt42>(Variant(_30.fld3, 2), 2)).fld2 = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_30.fld1, 0), 1)));
_52.fld1.2 = core::ptr::addr_of!(place!(Field::<Adt42>(Variant(_30.fld3, 2), 2)).fld4);
_35 = _30.fld2;
_3 = _5.0 * _2;
_18.0.1 = _25.0.1;
_52.fld1.2 = core::ptr::addr_of!(place!(Field::<Adt42>(Variant(_30.fld3, 2), 2)).fld4);
Goto(bb29)
}
bb29 = {
_15 = _39 as isize;
_29 = _15 ^ _60.0.2;
_13 = _52.fld4 >> _52.fld4;
_18.0.3 = !Field::<((f64, u32, isize, u32),)>(Variant(_46, 1), 0).0.3;
place!(Field::<((f64, u32, isize, u32),)>(Variant(_46, 1), 0)).0.1 = _11.1;
_70 = _32 as f32;
_25 = (_36,);
_15 = _36.2 - _29;
_25.0.0 = -_34.0;
_46 = Adt50::Variant1 { fld0: _60,fld1: _30.fld0,fld2: _16,fld3: 167629703_i32,fld4: _24 };
_50 = !_30.fld5.0;
_41 = Adt56::Variant3 { fld0: Field::<f32>(Variant(_30.fld1, 0), 0),fld1: _19.2,fld2: _57.0 };
_30.fld7 = core::ptr::addr_of!(_57.0);
_55 = _33 as f64;
place!(Field::<u128>(Variant(_46, 1), 1)) = _57.0 + _28.0;
_68 = core::ptr::addr_of!(place!(Field::<Adt42>(Variant(_30.fld3, 2), 2)).fld4);
_18 = (_60.0,);
_5.2 = (-1448027572_i32) as u16;
_11 = (_18.0.0, _18.0.1, _22, _60.0.3);
_53 = _44;
_5.2 = _52.fld4 as u16;
_52.fld0 = Field::<[i64; 3]>(Variant(_46, 1), 2);
place!(Field::<((f64, u32, isize, u32),)>(Variant(_46, 1), 0)).0.1 = _60.0.1 * _34.3;
_14.0 = !_9;
place!(Field::<[usize; 1]>(Variant(_46, 1), 4)) = [_48.3];
_49 = !_6;
Goto(bb30)
}
bb30 = {
_60.0.1 = _36.1;
_40 = Field::<f32>(Variant(_30.fld1, 0), 0);
_66 = _52.fld4 as i128;
_34.2 = !_36.2;
SetDiscriminant(_41, 2);
place!(Field::<f32>(Variant(_30.fld3, 2), 4)) = -_17;
_74.0 = !_43;
_52.fld1.2 = core::ptr::addr_of!((*_68));
_18.0.0 = Field::<Adt42>(Variant(_30.fld3, 2), 2).fld1.0;
_3 = _5.2;
_56 = _14.0;
_71.1 = Field::<u128>(Variant(_46, 1), 1) as i16;
_60.0.1 = _52.fld3 as u32;
_61 = _9;
place!(Field::<u16>(Variant(_41, 2), 6)) = _19.0;
_30.fld4 = core::ptr::addr_of!(_34.3);
_52.fld4 = !_13;
place!(Field::<*mut i32>(Variant(_30.fld3, 2), 1)) = core::ptr::addr_of_mut!(place!(Field::<i32>(Variant(_46, 1), 3)));
Goto(bb31)
}
bb31 = {
_28 = _57;
place!(Field::<Adt42>(Variant(_30.fld3, 2), 2)).fld3 = [(-19_i8),(-128_i8),9_i8,75_i8,49_i8,48_i8,61_i8];
place!(Field::<Adt42>(Variant(_30.fld3, 2), 2)).fld1 = (_34.0, _34.1, _52.fld2, _18.0.1);
_33 = _40;
(*_68) = core::ptr::addr_of!(_57);
_30.fld6 = _62 as u16;
_21 = _30.fld2;
_36.0 = _13 as f64;
place!(Field::<(f64, u32, isize, u32)>(Variant(_41, 2), 2)).3 = Field::<((f64, u32, isize, u32),)>(Variant(_46, 1), 0).0.1 | Field::<Adt42>(Variant(_30.fld3, 2), 2).fld1.1;
place!(Field::<(f64, u32, isize, u32)>(Variant(_41, 2), 2)).2 = -Field::<((f64, u32, isize, u32),)>(Variant(_46, 1), 0).0.2;
place!(Field::<Adt42>(Variant(_30.fld3, 2), 2)).fld0 = -_32;
_10 = _49;
place!(Field::<u128>(Variant(_46, 1), 1)) = !_57.0;
_5 = (Field::<u16>(Variant(_41, 2), 6), _48.1, Field::<u16>(Variant(_41, 2), 6), _49);
_71.1 = _5.1;
_44 = _53;
_36 = _60.0;
_52.fld1.1 = [_52.fld3,_39,_52.fld3,_52.fld3,_39,_39];
_11.1 = _48.2 as u32;
Call(place!(Field::<i32>(Variant(_46, 1), 3)) = fn19(_48.2, _52.fld1.0, _18.0.1, (*_68), (*_68), _7, _20, _49, _22, Field::<((f64, u32, isize, u32),)>(Variant(_46, 1), 0).0.3), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
_18.0.2 = _60.0.2;
_19.3 = _49;
_4 = !_48.2;
Goto(bb33)
}
bb33 = {
place!(Field::<Adt42>(Variant(_30.fld3, 2), 2)).fld1.1 = _52.fld4 as u32;
SetDiscriminant(_46, 1);
_33 = _52.fld4 as f32;
_60.0 = (_25.0.0, _36.3, _29, _25.0.1);
_57 = (_30.fld0,);
_30.fld1 = Adt46::Variant2 { fld0: _16 };
RET = Adt51::Variant2 { fld0: _52.fld4,fld1: (*_68),fld2: 158_u8 };
_71.0 = _5.2 | _19.0;
place!(Field::<((f64, u32, isize, u32),)>(Variant(_46, 1), 0)).0.3 = Field::<Adt42>(Variant(_30.fld3, 2), 2).fld1.1;
_27 = Adt53::Variant3 { fld0: _57,fld1: _52.fld3 };
_71.2 = !_30.fld6;
Call(_15 = core::intrinsics::transmute(Field::<(f64, u32, isize, u32)>(Variant(_41, 2), 2).2), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
_18.0.0 = _34.0 - _54;
_5.1 = !_45;
RET = Adt51::Variant3 { fld0: Field::<Adt42>(Variant(_30.fld3, 2), 2).fld2,fld1: _52.fld1.1,fld2: _71.2,fld3: _52.fld1.0,fld4: 50_u8 };
_5.0 = _48.0 & _2;
_60.0.1 = _18.0.1;
_5.0 = Field::<u16>(Variant(_41, 2), 6) - _71.2;
_72 = [_32,_32,Field::<Adt42>(Variant(_30.fld3, 2), 2).fld0];
_26 = _52.fld4 as usize;
_79.0.3 = _18.0.1;
_60.0.3 = _11.3;
place!(Field::<((f64, u32, isize, u32),)>(Variant(_46, 1), 0)).0.0 = _55;
_34.0 = Field::<Adt42>(Variant(_30.fld3, 2), 2).fld1.0;
_38 = [_62,_22,_15,_18.0.2,_29,_22];
_52.fld1.1 = Field::<[i128; 6]>(Variant(RET, 3), 1);
SetDiscriminant(_30.fld1, 1);
Goto(bb35)
}
bb35 = {
_28 = Field::<(u128,)>(Variant(_27, 3), 0);
_79.0.2 = _47.0 as isize;
_60.0.2 = Field::<Adt42>(Variant(_30.fld3, 2), 2).fld1.2;
_44 = _53;
_75 = !Field::<Adt42>(Variant(_30.fld3, 2), 2).fld1.2;
_19 = (_3, _45, _30.fld6, _26);
_32 = Field::<Adt42>(Variant(_30.fld3, 2), 2).fld0 ^ Field::<Adt42>(Variant(_30.fld3, 2), 2).fld0;
_5.3 = _19.3;
_52.fld1 = (Field::<*const ((f64, u32, isize, u32),)>(Variant(RET, 3), 3), Field::<[i128; 6]>(Variant(RET, 3), 1), _68, Field::<[i128; 6]>(Variant(RET, 3), 1));
_79.0 = (_25.0.0, Field::<Adt42>(Variant(_30.fld3, 2), 2).fld1.1, _75, Field::<Adt42>(Variant(_30.fld3, 2), 2).fld1.3);
place!(Field::<Adt42>(Variant(_30.fld3, 2), 2)).fld0 = _32;
place!(Field::<i128>(Variant(_27, 3), 1)) = _19.1 as i128;
_66 = -_52.fld3;
_52.fld2 = -_34.2;
place!(Field::<u8>(Variant(RET, 3), 4)) = 101_u8 + 64_u8;
place!(Field::<usize>(Variant(_30.fld3, 2), 0)) = _19.3 << _3;
place!(Field::<char>(Variant(_41, 2), 1)) = _44;
_18.0.0 = Field::<Adt42>(Variant(_30.fld3, 2), 2).fld0 as f64;
_79.0.3 = _60.0.3;
_52.fld0 = _16;
_42 = [_32,Field::<Adt42>(Variant(_30.fld3, 2), 2).fld0,_32];
Goto(bb36)
}
bb36 = {
Call(_85 = dump_var(12_usize, 26_usize, Move(_26), 13_usize, Move(_13), 15_usize, Move(_15), 56_usize, Move(_56)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Call(_85 = dump_var(12_usize, 32_usize, Move(_32), 14_usize, Move(_14), 75_usize, Move(_75), 42_usize, Move(_42)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Call(_85 = dump_var(12_usize, 24_usize, Move(_24), 29_usize, Move(_29), 47_usize, Move(_47), 48_usize, Move(_48)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_85 = dump_var(12_usize, 2_usize, Move(_2), 9_usize, Move(_9), 67_usize, Move(_67), 12_usize, Move(_12)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Call(_85 = dump_var(12_usize, 28_usize, Move(_28), 51_usize, Move(_51), 16_usize, Move(_16), 35_usize, Move(_35)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Call(_85 = dump_var(12_usize, 6_usize, Move(_6), 62_usize, Move(_62), 45_usize, Move(_45), 86_usize, _86), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: bool,mut _2: u16,mut _3: i16,mut _4: bool,mut _5: u16,mut _6: u16,mut _7: usize,mut _8: usize,mut _9: usize,mut _10: u16,mut _11: i16,mut _12: u16,mut _13: bool,mut _14: (u16, i16, u16, usize),mut _15: bool) -> u16 {
mir! {
type RET = u16;
let _16: (f64, u32, isize, u32);
let _17: Adt52;
let _18: char;
let _19: [i128; 6];
let _20: u64;
let _21: f32;
let _22: ();
let _23: ();
{
_14.2 = !_10;
_17.fld6 = !_14.0;
_7 = 208230790279128268466708236500986992160_u128 as usize;
_17.fld0 = 123204205505255455494716107340715474216_u128;
_5 = !_6;
_13 = _1;
_16.1 = 1856055978_u32 * 2727409820_u32;
_17.fld7 = core::ptr::addr_of!(_17.fld0);
match _17.fld0 {
0 => bb1,
1 => bb2,
123204205505255455494716107340715474216 => bb4,
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
_14.2 = 1602250352_i32 as u16;
_2 = _11 as u16;
_11 = 5359522630918408970_i64 as i16;
_17.fld5.0 = _4 ^ _4;
_17.fld4 = core::ptr::addr_of!(_16.3);
_14 = (_2, _3, _10, _9);
_17.fld7 = core::ptr::addr_of!(_17.fld0);
_15 = _10 > _12;
_16.2 = 9223372036854775807_isize;
_16.2 = !(-9223372036854775808_isize);
_16.1 = _17.fld0 as u32;
_9 = _14.3 << _3;
match _17.fld0 {
0 => bb5,
1 => bb6,
2 => bb7,
123204205505255455494716107340715474216 => bb9,
_ => bb8
}
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
_8 = _9 | _9;
RET = !_14.0;
_17.fld5 = (_4,);
_17.fld6 = _14.0;
_10 = _16.1 as u16;
_17.fld5 = (_15,);
_14 = (_17.fld6, _3, _17.fld6, _8);
_16.0 = _16.1 as f64;
_17.fld7 = core::ptr::addr_of!(_17.fld0);
_16.3 = _16.1 + _16.1;
_11 = -_14.1;
_3 = _14.1 << _9;
_14.2 = _12;
_9 = _17.fld0 as usize;
_16.2 = 10262352450450487242_u64 as isize;
_17.fld4 = core::ptr::addr_of!(_16.1);
_14 = (_2, _3, _2, _8);
_11 = _14.1;
_16.2 = 118_isize & (-9223372036854775808_isize);
RET = 3440739905595887375_i64 as u16;
RET = !_14.2;
_4 = !_1;
_8 = !_14.3;
Goto(bb10)
}
bb10 = {
Call(_22 = dump_var(13_usize, 12_usize, Move(_12), 9_usize, Move(_9), 8_usize, Move(_8), 2_usize, Move(_2)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_22 = dump_var(13_usize, 5_usize, Move(_5), 11_usize, Move(_11), 3_usize, Move(_3), 23_usize, _23), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: (bool,),mut _2: i16,mut _3: u16,mut _4: u16,mut _5: usize) -> (u16, i16, u16, usize) {
mir! {
type RET = (u16, i16, u16, usize);
let _6: Adt44;
let _7: ((f64, u32, isize, u32),);
let _8: i8;
let _9: ();
let _10: ();
{
RET.0 = _2 as u16;
RET.2 = _3;
RET.1 = 34_i8 as i16;
RET.3 = _5;
_7.0.3 = 266409021_u32;
_3 = 3793599783031256699_i64 as u16;
_7.0.0 = 75_u8 as f64;
_6.fld0 = [130190786360096119_i64,(-8926497393064469044_i64),756607471740404920_i64];
_4 = RET.0 * RET.0;
_6.fld1.3 = [(-143320164394147947865547918557423468455_i128),(-8006712030835482244985204139013107119_i128),49303861300201220031206594299630055574_i128,(-43123165541550209270521052151797997503_i128),(-110048207172897694037847131672833649418_i128),25967136909551394796118139279163236644_i128];
_6.fld3 = !16996495995856590578790026741029625621_i128;
_7.0.0 = 4856915992503192186_i64 as f64;
_4 = 7656136688216923357_u64 as u16;
_7.0.1 = _7.0.3;
_3 = !RET.0;
RET.1 = _2;
RET.2 = RET.1 as u16;
_6.fld0 = [5625663210641266838_i64,(-8984278874719041595_i64),336801137001057902_i64];
_6.fld1.3 = [_6.fld3,_6.fld3,_6.fld3,_6.fld3,_6.fld3,_6.fld3];
_7.0.2 = 9223372036854775807_isize;
Goto(bb1)
}
bb1 = {
Call(_9 = dump_var(14_usize, 2_usize, Move(_2), 1_usize, Move(_1), 10_usize, _10, 10_usize, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: i16,mut _2: bool,mut _3: u64,mut _4: i16,mut _5: u16) -> (f64, u32, isize, u32) {
mir! {
type RET = (f64, u32, isize, u32);
let _6: isize;
let _7: char;
let _8: &'static *const u32;
let _9: *const (u128,);
let _10: i8;
let _11: i32;
let _12: ();
let _13: ();
{
RET.1 = 147_u8 as u32;
RET.3 = (-84704306568490058634610321002214551569_i128) as u32;
RET.0 = 106531223017574454898696438415295402638_u128 as f64;
RET.2 = RET.1 as isize;
RET.1 = RET.3;
_6 = 12753128896706871899_usize as isize;
RET.1 = !RET.3;
_7 = '\u{a4ede}';
_3 = RET.0 as u64;
_1 = -_4;
RET.0 = 46_i8 as f64;
Call(RET.3 = core::intrinsics::transmute(_7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET.3 = _4 as u32;
_6 = RET.2 - RET.2;
RET.0 = 6_usize as f64;
_6 = !RET.2;
_4 = _1 << _5;
RET.3 = RET.1;
RET.1 = !RET.3;
RET.3 = !RET.1;
RET.3 = RET.1 + RET.1;
RET.1 = !RET.3;
RET.1 = _5 as u32;
RET.1 = RET.3 | RET.3;
RET.3 = RET.1 + RET.1;
RET.0 = 892056390_i32 as f64;
_4 = _1 ^ _1;
RET.0 = _1 as f64;
RET.1 = !RET.3;
_6 = !RET.2;
Goto(bb2)
}
bb2 = {
Call(_12 = dump_var(15_usize, 4_usize, Move(_4), 3_usize, Move(_3), 1_usize, Move(_1), 13_usize, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: f32,mut _2: bool,mut _3: (u16, i16, u16, usize),mut _4: bool,mut _5: (f64, u32, isize, u32),mut _6: usize,mut _7: bool,mut _8: ((f64, u32, isize, u32),),mut _9: u16,mut _10: isize,mut _11: u32,mut _12: (f64, u32, isize, u32),mut _13: (f64, u32, isize, u32)) -> (u16, i16, u16, usize) {
mir! {
type RET = (u16, i16, u16, usize);
let _14: [i128; 6];
let _15: [i128; 6];
let _16: (f64, u32, isize, u32);
let _17: ();
let _18: ();
{
RET.1 = _3.1 - _3.1;
_13.1 = _5.1;
RET.3 = _3.3;
RET = _3;
_12.0 = _12.2 as f64;
_1 = _12.0 as f32;
_3.0 = !RET.2;
_3 = RET;
_5.0 = _12.0;
_3.3 = _6 << _10;
_12.1 = !_8.0.1;
RET = (_9, _3.1, _3.0, _6);
_5.1 = !_8.0.3;
_7 = !_4;
_9 = RET.0;
RET.1 = _3.1;
_14 = [140052625561622555663844094518599233577_i128,(-140167504552113397213936561076821790212_i128),(-23136283974560943924996634434630380525_i128),(-6802151288003190182732850945358572904_i128),162402995857499138123621596169427016370_i128,(-15627266051206113831150086802321073146_i128)];
_2 = _7;
RET.3 = _6;
_7 = _5.1 < _8.0.1;
RET.1 = _3.1 ^ _3.1;
RET = (_3.0, _3.1, _3.0, _3.3);
RET.1 = _3.1;
RET.3 = _6 + _3.3;
_5.0 = _12.0;
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(16_usize, 3_usize, Move(_3), 2_usize, Move(_2), 9_usize, Move(_9), 7_usize, Move(_7)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: (f64, u32, isize, u32),mut _2: u32,mut _3: u128,mut _4: (f64, u32, isize, u32),mut _5: u16,mut _6: Adt49,mut _7: usize,mut _8: isize,mut _9: i16,mut _10: isize,mut _11: usize,mut _12: isize,mut _13: (f64, u32, isize, u32),mut _14: f32) -> [i8; 7] {
mir! {
type RET = [i8; 7];
let _15: f64;
let _16: f64;
let _17: f32;
let _18: (u128,);
let _19: i32;
let _20: isize;
let _21: (u128,);
let _22: [i32; 6];
let _23: isize;
let _24: [i8; 7];
let _25: char;
let _26: isize;
let _27: Adt51;
let _28: (*const ((f64, u32, isize, u32),), [i128; 6], *const *const (u128,), [i128; 6]);
let _29: Adt42;
let _30: ();
let _31: ();
{
_13.1 = _2;
_5 = '\u{94379}' as u16;
RET = [0_i8,(-49_i8),122_i8,(-116_i8),(-27_i8),114_i8,(-104_i8)];
_7 = _11;
_1.3 = !_13.3;
_8 = !_13.2;
_1 = (_4.0, _13.1, _4.2, _4.3);
_9 = false as i16;
_7 = _11 - _11;
_1.1 = _13.1;
_4 = (_1.0, _1.3, _8, _1.3);
RET = [92_i8,(-6_i8),38_i8,(-113_i8),68_i8,(-48_i8),(-74_i8)];
place!(Field::<isize>(Variant(_6, 0), 0)) = _11 as isize;
_8 = -_1.2;
_10 = _8 & _4.2;
_11 = !_7;
_2 = !_4.3;
_4.0 = -_1.0;
_7 = _11 >> Field::<isize>(Variant(_6, 0), 0);
_1.2 = !Field::<isize>(Variant(_6, 0), 0);
_13.1 = _4.3;
_1.3 = !_13.1;
_2 = _13.1;
Call(_12 = core::intrinsics::transmute(Field::<isize>(Variant(_6, 0), 0)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = 29934_i16;
_15 = Field::<isize>(Variant(_6, 0), 0) as f64;
_7 = _5 as usize;
RET = [41_i8,111_i8,(-34_i8),13_i8,(-89_i8),(-62_i8),69_i8];
RET = [88_i8,36_i8,(-23_i8),13_i8,(-105_i8),(-33_i8),(-91_i8)];
_2 = !_1.3;
_7 = !_11;
_5 = 21427_u16;
_7 = !_11;
_8 = -_12;
_4.1 = _9 as u32;
Goto(bb2)
}
bb2 = {
SetDiscriminant(_6, 3);
_14 = (-163786890_i32) as f32;
_1.0 = _15 - _15;
place!(Field::<u128>(Variant(_6, 3), 0)) = !_3;
_1.0 = _4.0;
place!(Field::<[usize; 1]>(Variant(_6, 3), 4)) = [_7];
_1.0 = _7 as f64;
_1.1 = _9 as u32;
_3 = _11 as u128;
Call(_13.0 = core::intrinsics::transmute(_4.2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_16 = -_1.0;
_13.0 = -_15;
_13 = _1;
_14 = _1.3 as f32;
_2 = _5 as u32;
_7 = _10 as usize;
_1 = (_15, _4.3, _4.2, _4.3);
_17 = _14;
_11 = !_7;
place!(Field::<u128>(Variant(_6, 3), 0)) = !_3;
_19 = (-626753170_i32);
_18.0 = _3 >> _3;
match _9 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
29934 => bb11,
_ => bb10
}
}
bb4 = {
SetDiscriminant(_6, 3);
_14 = (-163786890_i32) as f32;
_1.0 = _15 - _15;
place!(Field::<u128>(Variant(_6, 3), 0)) = !_3;
_1.0 = _4.0;
place!(Field::<[usize; 1]>(Variant(_6, 3), 4)) = [_7];
_1.0 = _7 as f64;
_1.1 = _9 as u32;
_3 = _11 as u128;
Call(_13.0 = core::intrinsics::transmute(_4.2), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_9 = 29934_i16;
_15 = Field::<isize>(Variant(_6, 0), 0) as f64;
_7 = _5 as usize;
RET = [41_i8,111_i8,(-34_i8),13_i8,(-89_i8),(-62_i8),69_i8];
RET = [88_i8,36_i8,(-23_i8),13_i8,(-105_i8),(-33_i8),(-91_i8)];
_2 = !_1.3;
_7 = !_11;
_5 = 21427_u16;
_7 = !_11;
_8 = -_12;
_4.1 = _9 as u32;
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
_2 = _13.3 & _1.3;
place!(Field::<[isize; 6]>(Variant(_6, 3), 3)) = [_1.2,_13.2,_10,_8,_12,_13.2];
_21 = (_3,);
_20 = _13.2;
RET = [(-102_i8),16_i8,(-9_i8),(-52_i8),109_i8,37_i8,(-127_i8)];
place!(Field::<isize>(Variant(_6, 3), 2)) = _8;
_1.1 = _2 << _4.3;
Goto(bb12)
}
bb12 = {
_18.0 = _3 | _21.0;
_5 = _2 as u16;
_19 = _5 as i32;
_18 = (_21.0,);
place!(Field::<u64>(Variant(_6, 3), 1)) = 3200986217641379461_u64 - 15040832457407570453_u64;
_9 = -4385_i16;
_22 = [_19,_19,_19,_19,_19,_19];
_13.3 = _16 as u32;
place!(Field::<u64>(Variant(_6, 3), 1)) = !4859079944579887304_u64;
_13.0 = _11 as f64;
_13.2 = _1.1 as isize;
_23 = _1.2;
_13.2 = _10 ^ _4.2;
_11 = _7;
_15 = _1.0;
_23 = !_12;
_23 = Field::<isize>(Variant(_6, 3), 2);
_4 = _13;
Goto(bb13)
}
bb13 = {
_5 = _9 as u16;
_25 = '\u{a4fd7}';
place!(Field::<u64>(Variant(_6, 3), 1)) = 13357660907783494479_u64 - 14142807747872277724_u64;
_13 = (_4.0, _4.3, _23, _4.3);
_13 = (_4.0, _2, _8, _1.1);
_10 = _25 as isize;
_4.2 = Field::<u64>(Variant(_6, 3), 1) as isize;
_21 = (_3,);
_1.3 = _4.3 & _1.1;
RET = [5_i8,(-9_i8),(-95_i8),(-36_i8),(-89_i8),10_i8,(-80_i8)];
_11 = !_7;
Goto(bb14)
}
bb14 = {
_29.fld1.0 = -_16;
_13.1 = _2 ^ _4.3;
_29.fld3 = [(-68_i8),88_i8,86_i8,(-92_i8),20_i8,118_i8,96_i8];
_1.2 = _13.2;
_13.3 = (-5497135841953523360_i64) as u32;
_4.1 = _13.1 ^ _4.3;
_9 = _19 as i16;
_4 = (_1.0, _1.3, _23, _1.3);
place!(Field::<isize>(Variant(_6, 3), 2)) = -_1.2;
_5 = !46750_u16;
_22 = [_19,_19,_19,_19,_19,_19];
_29.fld1.1 = _13.1 ^ _4.3;
place!(Field::<[isize; 6]>(Variant(_6, 3), 3)) = [_12,_12,_4.2,_13.2,_4.2,_8];
_24 = RET;
place!(Field::<*const *mut i32>(Variant(_6, 3), 6)) = core::ptr::addr_of!(_29.fld2);
_21 = (_3,);
place!(Field::<*const *mut i32>(Variant(_6, 3), 6)) = core::ptr::addr_of!(_29.fld2);
_28.2 = core::ptr::addr_of!(_29.fld4);
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(17_usize, 20_usize, Move(_20), 19_usize, Move(_19), 12_usize, Move(_12), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(17_usize, 22_usize, Move(_22), 3_usize, Move(_3), 5_usize, Move(_5), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: (bool,),mut _2: (f64, u32, isize, u32),mut _3: (f64, u32, isize, u32),mut _4: u16,mut _5: (u128,),mut _6: ((f64, u32, isize, u32),),mut _7: bool) -> [i64; 3] {
mir! {
type RET = [i64; 3];
let _8: f32;
let _9: i16;
let _10: usize;
let _11: (bool,);
let _12: Adt51;
let _13: *const &'static *const u32;
let _14: (u16, i16, u16, usize);
let _15: (u16, i16, u16, usize);
let _16: char;
let _17: i8;
let _18: [isize; 6];
let _19: u64;
let _20: bool;
let _21: *const ((f64, u32, isize, u32),);
let _22: (bool,);
let _23: i16;
let _24: [i64; 3];
let _25: [i128; 6];
let _26: Adt57;
let _27: f32;
let _28: isize;
let _29: (u16, i16, u16, usize);
let _30: (&'static *const u32, i64, isize, *const u128, (u16, i16, u16, usize));
let _31: f32;
let _32: ();
let _33: ();
{
_8 = _2.0 as f32;
_9 = (-47162378273671182891940580255555881825_i128) as i16;
_11 = _1;
_7 = _11.0 ^ _1.0;
_6.0 = (_3.0, _3.1, _2.2, _3.1);
_3.3 = _11.0 as u32;
_6.0.2 = !_2.2;
Goto(bb1)
}
bb1 = {
RET = [(-2335281040100760803_i64),(-3953318139977603593_i64),3315300015243801236_i64];
_3.0 = _6.0.0;
_6.0 = _2;
_6.0.0 = 35458988980031163096051424602725547793_i128 as f64;
_4 = _7 as u16;
_2.3 = !_6.0.3;
_3.2 = _2.2;
_3.0 = _2.0 * _2.0;
_1 = _11;
_2.3 = _3.3 + _6.0.3;
_4 = 31169_u16;
_14 = (_4, _9, _4, 3743757335699351747_usize);
_9 = _14.1;
_3.3 = !_3.1;
_9 = _8 as i16;
_2.1 = _3.3 & _3.3;
RET = [774866456343411760_i64,3503719038136404119_i64,(-4815507744705492654_i64)];
_14.2 = !_4;
_6.0 = (_3.0, _3.3, _2.2, _3.1);
_6.0.3 = !_3.1;
_16 = '\u{161c3}';
match _14.3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
3743757335699351747 => bb8,
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
_15.0 = !_4;
_14.1 = _11.0 as i16;
_15.2 = _16 as u16;
RET = [(-2228277404866097194_i64),6523438664505918460_i64,8680559995346709953_i64];
_11 = _1;
RET = [6116045677187685941_i64,(-8467333500536961020_i64),1998224323563882650_i64];
_7 = !_11.0;
_16 = '\u{1091fd}';
_6.0.2 = _2.2;
_6.0.0 = -_2.0;
_15 = (_14.0, _14.1, _14.2, _14.3);
_3.3 = _6.0.3;
match _14.3 {
3743757335699351747 => bb9,
_ => bb6
}
}
bb9 = {
_11.0 = _3.2 != _2.2;
_11.0 = _7 & _7;
_15.0 = _16 as u16;
_21 = core::ptr::addr_of!(_6);
_15 = _14;
_22.0 = _7;
_15.1 = (-10_i8) as i16;
(*_21).0 = _3;
_1 = (_7,);
_22.0 = _7;
_9 = (-1555751270_i32) as i16;
_7 = !_1.0;
_3.1 = 1514005289_i32 as u32;
_14.3 = !_15.3;
_15.0 = _14.2 - _4;
_22 = (_7,);
_11 = (_7,);
_10 = !_14.3;
_2.0 = -_6.0.0;
_4 = !_15.0;
match _15.3 {
0 => bb4,
3743757335699351747 => bb10,
_ => bb2
}
}
bb10 = {
_15.0 = !_14.2;
_22.0 = _1.0 | _1.0;
(*_21) = (_2,);
_3.1 = !_3.3;
_6.0.1 = _6.0.3 | _3.3;
_14.1 = _9;
_24 = RET;
_17 = (-105_i8);
_15.2 = _17 as u16;
_3.1 = _6.0.3 ^ _6.0.1;
_1 = _22;
Goto(bb11)
}
bb11 = {
_19 = 18378506000313874003_u64;
_6.0 = (_2.0, _3.1, _3.2, _2.3);
_6.0.1 = (*_21).0.3 >> _6.0.2;
_6.0 = (_3.0, _2.1, _2.2, _2.3);
(*_21).0.1 = _6.0.3 ^ _3.3;
(*_21).0.3 = (*_21).0.1;
_2 = (_6.0.0, (*_21).0.3, _6.0.2, (*_21).0.1);
RET = [5109389001959676851_i64,(-8939129078965487126_i64),(-864059742403548682_i64)];
(*_21) = (_2,);
_25 = [(-142616858368163631024915919154366065333_i128),7003217388664626042073231671661048690_i128,64806272520979726532808753979417061458_i128,1741446097413992887419315929291327101_i128,118686225797222014958309367418850834210_i128,(-94114729326358438708192534375254614452_i128)];
_6.0.1 = !_2.3;
Call(_14.0 = core::intrinsics::transmute(_15.1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_19 = 25_u8 as u64;
_14 = (_4, _9, _4, _15.3);
_30.4.0 = _14.0;
_14.0 = _4;
_29.3 = _14.3 & _14.3;
_14.1 = _15.1;
_2.3 = !_3.1;
_2.0 = (*_21).0.0 - (*_21).0.0;
RET = [3582883079799689979_i64,(-3985274527628604246_i64),(-6368214763195851173_i64)];
_30.3 = core::ptr::addr_of!(_5.0);
_15.2 = !_30.4.0;
_5.0 = 234848222591188581456992532239715270775_u128;
_14.3 = _29.3;
_2.2 = _8 as isize;
_6.0.1 = _2.3 - _2.3;
(*_21).0.0 = _3.0 - _2.0;
_15.3 = _5.0 as usize;
match _5.0 {
0 => bb7,
234848222591188581456992532239715270775 => bb14,
_ => bb13
}
}
bb13 = {
_15.0 = !_4;
_14.1 = _11.0 as i16;
_15.2 = _16 as u16;
RET = [(-2228277404866097194_i64),6523438664505918460_i64,8680559995346709953_i64];
_11 = _1;
RET = [6116045677187685941_i64,(-8467333500536961020_i64),1998224323563882650_i64];
_7 = !_11.0;
_16 = '\u{1091fd}';
_6.0.2 = _2.2;
_6.0.0 = -_2.0;
_15 = (_14.0, _14.1, _14.2, _14.3);
_3.3 = _6.0.3;
match _14.3 {
3743757335699351747 => bb9,
_ => bb6
}
}
bb14 = {
(*_21).0 = (_2.0, _3.3, _3.2, _2.3);
_7 = _11.0 > _1.0;
_6 = (_2,);
_29 = _14;
_14.0 = !_15.0;
_22 = _1;
_30.4.3 = _14.3;
(*_21) = (_2,);
_3.2 = !_6.0.2;
_30.4.3 = _29.3 & _29.3;
_8 = _15.1 as f32;
_2.1 = _6.0.1 & (*_21).0.3;
_2.3 = _2.1 * _6.0.3;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(18_usize, 1_usize, Move(_1), 4_usize, Move(_4), 22_usize, Move(_22), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(18_usize, 10_usize, Move(_10), 14_usize, Move(_14), 24_usize, Move(_24), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: u16,mut _2: *const ((f64, u32, isize, u32),),mut _3: u32,mut _4: *const (u128,),mut _5: *const (u128,),mut _6: bool,mut _7: [isize; 6],mut _8: usize,mut _9: isize,mut _10: u32) -> i32 {
mir! {
type RET = i32;
let _11: Adt54;
let _12: f64;
let _13: [i32; 6];
let _14: i32;
let _15: *const u32;
let _16: [isize; 6];
let _17: ();
let _18: ();
{
(*_4).0 = 252398644522900326725006267777394088135_u128;
(*_2).0.2 = !_9;
_4 = _5;
_6 = !true;
(*_2).0.2 = _9 * _9;
RET = (*_2).0.0 as i32;
(*_4).0 = 208697798162056301407912678820858771389_u128;
(*_5).0 = !269063445995756223304076551939184785196_u128;
(*_2).0.2 = _9 * _9;
_10 = (*_2).0.0 as u32;
(*_4).0 = 95148873587950727244360944088525297363_u128;
(*_4) = (263259908093837581994411149303353948595_u128,);
_2 = core::ptr::addr_of!((*_2));
_12 = (*_2).0.0 * (*_2).0.0;
(*_2).0.2 = '\u{8fe57}' as isize;
(*_2).0.3 = _10;
_6 = false & true;
_9 = (*_2).0.2 >> (*_2).0.3;
(*_5) = (10358714120913188726565370983907793704_u128,);
(*_4).0 = !138235586183983768139797775202360401497_u128;
_13 = [RET,RET,RET,RET,RET,RET];
_15 = core::ptr::addr_of!((*_2).0.1);
_4 = core::ptr::addr_of!((*_5));
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(19_usize, 7_usize, Move(_7), 10_usize, Move(_10), 13_usize, Move(_13), 9_usize, Move(_9)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(9190082807512596744_i64), std::hint::black_box((-58_i8)));
                
            }
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt42{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt42 {
fld0: i64,
fld1: (f64, u32, isize, u32),
fld2: *mut i32,
fld3: [i8; 7],
fld4: *const (u128,),
}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: bool,

},
Variant1{
fld0: (u16, i16, u16, usize),
fld1: (u128,),
fld2: ((f64, u32, isize, u32),),
fld3: *mut i32,
fld4: i16,
fld5: *const (u128,),

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: [i64; 3],
fld1: (*const ((f64, u32, isize, u32),), [i128; 6], *const *const (u128,), [i128; 6]),
fld2: isize,
fld3: i128,
fld4: u64,
}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: (*const ((f64, u32, isize, u32),), [i128; 6], *const *const (u128,), [i128; 6]),
fld1: *const *mut i32,
fld2: u128,
fld3: i8,
fld4: u8,
fld5: *const *const (u128,),
fld6: i64,

},
Variant1{
fld0: i128,
fld1: (u128,),
fld2: (bool,),
fld3: *const *mut i32,
fld4: (f64, u32, isize, u32),

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: f32,
fld1: i32,

},
Variant1{
fld0: u8,
fld1: *const *mut i32,

},
Variant2{
fld0: [i64; 3],

},
Variant3{
fld0: f64,
fld1: *const *const (u128,),
fld2: (*const ((f64, u32, isize, u32),), [i128; 6], *const *const (u128,), [i128; 6]),
fld3: [i64; 3],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: i64,
fld1: *const (u128,),
fld2: u16,

},
Variant1{
fld0: *const u128,
fld1: u128,
fld2: (bool,),
fld3: i8,

},
Variant2{
fld0: u64,
fld1: Adt45,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: bool,
fld1: *const u32,
fld2: Adt45,
fld3: (f64, u32, isize, u32),
fld4: i16,

},
Variant1{
fld0: bool,
fld1: i128,
fld2: *const u32,
fld3: i8,
fld4: [isize; 6],
fld5: *const (u128,),

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: isize,
fld1: f32,

},
Variant1{
fld0: (u128,),
fld1: *const (u128,),

},
Variant2{
fld0: usize,
fld1: *mut i32,
fld2: Adt42,
fld3: i128,
fld4: f32,

},
Variant3{
fld0: u128,
fld1: u64,
fld2: isize,
fld3: [isize; 6],
fld4: [usize; 1],
fld5: *const *const (u128,),
fld6: *const *mut i32,

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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: [usize; 1],
fld1: [i128; 6],
fld2: Adt43,
fld3: Adt44,
fld4: Adt48,

},
Variant1{
fld0: ((f64, u32, isize, u32),),
fld1: u128,
fld2: [i64; 3],
fld3: i32,
fld4: [usize; 1],

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: *const u128,
fld1: Adt43,
fld2: (f64, u32, isize, u32),
fld3: ((f64, u32, isize, u32),),
fld4: i16,

},
Variant1{
fld0: (*const ((f64, u32, isize, u32),), [i128; 6], *const *const (u128,), [i128; 6]),
fld1: char,
fld2: Adt46,
fld3: u128,
fld4: i16,
fld5: [i8; 7],
fld6: i64,
fld7: *const u32,

},
Variant2{
fld0: u64,
fld1: *const (u128,),
fld2: u8,

},
Variant3{
fld0: *mut i32,
fld1: [i128; 6],
fld2: u16,
fld3: *const ((f64, u32, isize, u32),),
fld4: u8,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: u128,
fld1: Adt46,
fld2: [i8; 7],
fld3: Adt49,
fld4: *const u32,
fld5: (bool,),
fld6: u16,
fld7: *const u128,
}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
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
fld0: ((f64, u32, isize, u32),),
fld1: [usize; 1],
fld2: [isize; 6],
fld3: [i8; 7],
fld4: u32,
fld5: *const ((f64, u32, isize, u32),),
fld6: (u16, i16, u16, usize),

},
Variant1{
fld0: bool,
fld1: i32,
fld2: i64,
fld3: Adt48,
fld4: usize,

},
Variant2{
fld0: Adt48,
fld1: u64,
fld2: (u16, i16, u16, usize),
fld3: Adt43,
fld4: (bool,),

},
Variant3{
fld0: (u128,),
fld1: i128,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: u16,
fld1: i64,
fld2: (*const ((f64, u32, isize, u32),), [i128; 6], *const *const (u128,), [i128; 6]),

},
Variant1{
fld0: bool,
fld1: Adt44,
fld2: [i128; 6],
fld3: (u128,),

},
Variant2{
fld0: ((f64, u32, isize, u32),),
fld1: Adt52,
fld2: [usize; 1],
fld3: [i8; 7],
fld4: u32,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: ((f64, u32, isize, u32),),
fld1: u16,
fld2: (bool,),
fld3: (f64, u32, isize, u32),
fld4: Adt51,

},
Variant1{
fld0: f64,
fld1: i8,

},
Variant2{
fld0: (bool,),
fld1: Adt45,
fld2: u64,
fld3: [i8; 7],
fld4: i16,
fld5: i32,
fld6: (f64, u32, isize, u32),

},
Variant3{
fld0: u64,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: i128,
fld1: [i32; 6],
fld2: (u128,),

},
Variant1{
fld0: Adt50,
fld1: [i128; 6],
fld2: isize,
fld3: (u16, i16, u16, usize),
fld4: usize,

},
Variant2{
fld0: Adt53,
fld1: char,
fld2: (f64, u32, isize, u32),
fld3: Adt46,
fld4: u8,
fld5: u128,
fld6: u16,

},
Variant3{
fld0: f32,
fld1: u16,
fld2: u128,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: (u128,),
fld1: *const ((f64, u32, isize, u32),),
fld2: *const *mut i32,
fld3: (u16, i16, u16, usize),
fld4: [i32; 6],
fld5: [usize; 1],
fld6: Adt42,
fld7: Adt55,

},
Variant1{
fld0: u32,
fld1: Adt46,
fld2: *const ((f64, u32, isize, u32),),
fld3: Adt51,
fld4: *const (u128,),
fld5: *const u128,
fld6: [i32; 6],
fld7: Adt55,

},
Variant2{
fld0: u16,
fld1: (u16, i16, u16, usize),
fld2: isize,
fld3: i8,
fld4: Adt44,
fld5: Adt43,
fld6: *const ((f64, u32, isize, u32),),

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: [i64; 3],
fld1: u8,
fld2: [usize; 1],
fld3: Adt54,
fld4: ((f64, u32, isize, u32),),
fld5: i32,
fld6: *const ((f64, u32, isize, u32),),
fld7: i128,

},
Variant1{
fld0: bool,
fld1: char,
fld2: Adt54,
fld3: Adt55,
fld4: [i8; 7],
fld5: Adt50,
fld6: *const u128,

},
Variant2{
fld0: Adt52,
fld1: [usize; 1],
fld2: *const *mut i32,
fld3: [i32; 6],
fld4: f32,
fld5: i32,

},
Variant3{
fld0: bool,
fld1: i128,
fld2: (*const ((f64, u32, isize, u32),), [i128; 6], *const *const (u128,), [i128; 6]),
fld3: [usize; 1],
fld4: *const *const (u128,),
fld5: u32,
fld6: i64,

}}

