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
pub fn fn0(mut _1: u128,mut _2: u32,mut _3: isize,mut _4: usize,mut _5: i128,mut _6: i32,mut _7: u64) -> i16 {
mir! {
type RET = i16;
let _8: char;
let _9: f32;
let _10: ((u32, f64, u64), [u128; 1], &'static i128, [i8; 7]);
let _11: isize;
let _12: isize;
let _13: &'static isize;
let _14: (&'static isize, u16);
let _15: usize;
let _16: ((u128, i64, f64),);
let _17: ();
let _18: ();
{
_6 = (-214369361_i32) & (-2002053087_i32);
_7 = !16342670237354944955_u64;
_5 = 151786301421841694528426806696126121638_i128;
_3 = !9223372036854775807_isize;
_2 = 9085563743165748917_usize as u32;
_8 = '\u{5b957}';
_6 = (-840784882_i32);
_9 = 7_usize as f32;
_9 = _3 as f32;
_10.1 = [104957818460889683352820888998166577832_u128];
_10.3 = [96_i8,(-36_i8),(-52_i8),(-63_i8),29_i8,115_i8,61_i8];
_7 = 549337407440209196_usize as u64;
_10.0.1 = _6 as f64;
_4 = 10185201348352832078_usize;
_1 = !200129113662703540800943844068407192907_u128;
RET = 4104_i16;
_11 = _2 as isize;
RET = _8 as i16;
_10.2 = &_5;
_3 = !_11;
_10.3 = [2_i8,99_i8,40_i8,50_i8,(-48_i8),(-15_i8),(-12_i8)];
_9 = 4802882914701795413_i64 as f32;
Goto(bb1)
}
bb1 = {
_10.0.1 = _3 as f64;
_10.0.1 = (-43_i8) as f64;
_7 = 1513081366435303149_u64 >> _5;
_7 = 15045179724423030859_u64 ^ 17248768909975947671_u64;
_4 = !13277084073787097118_usize;
_9 = _1 as f32;
_10.0.0 = _2;
_10.3 = [(-88_i8),42_i8,(-123_i8),(-1_i8),26_i8,(-93_i8),12_i8];
_4 = 1152081855646387504_usize | 6_usize;
_10.0.0 = _2;
_7 = 8241874365965285336_u64;
_6 = !1019142903_i32;
_10.0.1 = _11 as f64;
_6 = _4 as i32;
_11 = !_3;
_8 = '\u{e7823}';
_8 = '\u{5a81b}';
_4 = 1939187102251594611_i64 as usize;
_3 = !_11;
_10.3 = [86_i8,(-113_i8),(-7_i8),92_i8,(-73_i8),(-67_i8),68_i8];
_11 = !_3;
_5 = -(-15293822568291763639510782123214237270_i128);
_10.2 = &_5;
_2 = _10.0.0 ^ _10.0.0;
_10.0.2 = _7 << _1;
Goto(bb2)
}
bb2 = {
RET = -(-29416_i16);
_4 = 5069173992994995065_usize - 0_usize;
_10.0.2 = !_7;
Goto(bb3)
}
bb3 = {
_5 = _6 as i128;
_5 = -(-6863505616961893811201878910493036253_i128);
RET = _11 as i16;
_5 = (-33555549466450103993990704578921949998_i128) & (-97969312598987316436747464832160213216_i128);
_1 = 6816_u16 as u128;
_10.0.2 = _7 >> _3;
_10.1 = [_1];
match _7 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
8241874365965285336 => bb9,
_ => bb8
}
}
bb4 = {
RET = -(-29416_i16);
_4 = 5069173992994995065_usize - 0_usize;
_10.0.2 = !_7;
Goto(bb3)
}
bb5 = {
_10.0.1 = _3 as f64;
_10.0.1 = (-43_i8) as f64;
_7 = 1513081366435303149_u64 >> _5;
_7 = 15045179724423030859_u64 ^ 17248768909975947671_u64;
_4 = !13277084073787097118_usize;
_9 = _1 as f32;
_10.0.0 = _2;
_10.3 = [(-88_i8),42_i8,(-123_i8),(-1_i8),26_i8,(-93_i8),12_i8];
_4 = 1152081855646387504_usize | 6_usize;
_10.0.0 = _2;
_7 = 8241874365965285336_u64;
_6 = !1019142903_i32;
_10.0.1 = _11 as f64;
_6 = _4 as i32;
_11 = !_3;
_8 = '\u{e7823}';
_8 = '\u{5a81b}';
_4 = 1939187102251594611_i64 as usize;
_3 = !_11;
_10.3 = [86_i8,(-113_i8),(-7_i8),92_i8,(-73_i8),(-67_i8),68_i8];
_11 = !_3;
_5 = -(-15293822568291763639510782123214237270_i128);
_10.2 = &_5;
_2 = _10.0.0 ^ _10.0.0;
_10.0.2 = _7 << _1;
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
_4 = 5_usize;
_10.2 = &_5;
_2 = !_10.0.0;
_2 = _10.0.0;
_1 = 131427848299818207524342119704794959864_u128;
RET = 27087_i16 << _10.3[_4];
_8 = '\u{2047f}';
match _10.3[_4] {
0 => bb8,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
340282366920938463463374607431768211389 => bb16,
_ => bb15
}
}
bb10 = {
Return()
}
bb11 = {
_10.0.1 = _3 as f64;
_10.0.1 = (-43_i8) as f64;
_7 = 1513081366435303149_u64 >> _5;
_7 = 15045179724423030859_u64 ^ 17248768909975947671_u64;
_4 = !13277084073787097118_usize;
_9 = _1 as f32;
_10.0.0 = _2;
_10.3 = [(-88_i8),42_i8,(-123_i8),(-1_i8),26_i8,(-93_i8),12_i8];
_4 = 1152081855646387504_usize | 6_usize;
_10.0.0 = _2;
_7 = 8241874365965285336_u64;
_6 = !1019142903_i32;
_10.0.1 = _11 as f64;
_6 = _4 as i32;
_11 = !_3;
_8 = '\u{e7823}';
_8 = '\u{5a81b}';
_4 = 1939187102251594611_i64 as usize;
_3 = !_11;
_10.3 = [86_i8,(-113_i8),(-7_i8),92_i8,(-73_i8),(-67_i8),68_i8];
_11 = !_3;
_5 = -(-15293822568291763639510782123214237270_i128);
_10.2 = &_5;
_2 = _10.0.0 ^ _10.0.0;
_10.0.2 = _7 << _1;
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
_10.0.1 = _3 as f64;
_10.0.1 = (-43_i8) as f64;
_7 = 1513081366435303149_u64 >> _5;
_7 = 15045179724423030859_u64 ^ 17248768909975947671_u64;
_4 = !13277084073787097118_usize;
_9 = _1 as f32;
_10.0.0 = _2;
_10.3 = [(-88_i8),42_i8,(-123_i8),(-1_i8),26_i8,(-93_i8),12_i8];
_4 = 1152081855646387504_usize | 6_usize;
_10.0.0 = _2;
_7 = 8241874365965285336_u64;
_6 = !1019142903_i32;
_10.0.1 = _11 as f64;
_6 = _4 as i32;
_11 = !_3;
_8 = '\u{e7823}';
_8 = '\u{5a81b}';
_4 = 1939187102251594611_i64 as usize;
_3 = !_11;
_10.3 = [86_i8,(-113_i8),(-7_i8),92_i8,(-73_i8),(-67_i8),68_i8];
_11 = !_3;
_5 = -(-15293822568291763639510782123214237270_i128);
_10.2 = &_5;
_2 = _10.0.0 ^ _10.0.0;
_10.0.2 = _7 << _1;
Goto(bb2)
}
bb14 = {
RET = -(-29416_i16);
_4 = 5069173992994995065_usize - 0_usize;
_10.0.2 = !_7;
Goto(bb3)
}
bb15 = {
_5 = _6 as i128;
_5 = -(-6863505616961893811201878910493036253_i128);
RET = _11 as i16;
_5 = (-33555549466450103993990704578921949998_i128) & (-97969312598987316436747464832160213216_i128);
_1 = 6816_u16 as u128;
_10.0.2 = _7 >> _3;
_10.1 = [_1];
match _7 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
8241874365965285336 => bb9,
_ => bb8
}
}
bb16 = {
_15 = _4;
_3 = _11 - _11;
Goto(bb17)
}
bb17 = {
Call(_17 = dump_var(0_usize, 11_usize, Move(_11), 8_usize, Move(_8), 2_usize, Move(_2), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_17 = dump_var(0_usize, 15_usize, Move(_15), 18_usize, _18, 18_usize, _18, 18_usize, _18), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(223294319261571753953331484317037675405_u128), std::hint::black_box(1632011459_u32), std::hint::black_box((-14_isize)), std::hint::black_box(10010020611583412053_usize), std::hint::black_box((-132438473697784100053114866187540103966_i128)), std::hint::black_box((-159366380_i32)), std::hint::black_box(9486302833694526795_u64));
                
            }
impl PrintFDebug for Adt17{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt17{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt17 {
fld0: u16,
fld1: char,
fld2: isize,
fld3: usize,
fld4: i16,
fld5: u64,
fld6: i64,
}
impl PrintFDebug for Adt33{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt33{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt33 {
fld0: bool,
fld1: f64,
fld2: (u8, isize),
fld3: i8,
fld4: [u16; 1],
fld5: u16,
fld6: i64,
}
impl PrintFDebug for Adt37{
	unsafe fn printf_debug(&self){unsafe{printf("Adt37::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt37 {
Variant0{
fld0: [u16; 1],
fld1: u16,
fld2: (Adt17,),
fld3: (u128, f64),
fld4: [char; 5],
fld5: (u128, i64, f64),
fld6: f32,
fld7: Adt17,

},
Variant1{
fld0: [u64; 5],

},
Variant2{
fld0: (bool, i16),

}}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt40{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt40 {
fld0: *mut i64,
fld1: usize,
fld2: (u128, i64, f64),
fld3: Adt17,
fld4: i16,
fld5: i32,
}
impl PrintFDebug for Adt65{
	unsafe fn printf_debug(&self){unsafe{printf("Adt65::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt65 {
Variant0{
fld0: [i16; 4],
fld1: Adt33,

},
Variant1{
fld0: Adt17,

}}
impl PrintFDebug for Adt68{
	unsafe fn printf_debug(&self){unsafe{printf("Adt68::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt68 {
Variant0{
fld0: bool,
fld1: *mut i64,
fld2: isize,
fld3: usize,
fld4: [u64; 5],
fld5: Adt40,

},
Variant1{
fld0: i32,
fld1: (u16, [i8; 1], usize, u32),
fld2: [u64; 5],
fld3: *mut u16,

},
Variant2{
fld0: (bool, i16),

},
Variant3{
fld0: *mut [u16; 6],
fld1: i16,

}}
impl PrintFDebug for Adt73{
	unsafe fn printf_debug(&self){unsafe{printf("Adt73::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt73 {
Variant0{
fld0: f32,
fld1: char,
fld2: Adt37,
fld3: [u16; 1],
fld4: *mut u128,
fld5: (u8, isize),
fld6: [u64; 5],

},
Variant1{
fld0: [u16; 1],

},
Variant2{
fld0: Adt40,
fld1: *const u32,

}}
impl PrintFDebug for Adt81{
	unsafe fn printf_debug(&self){unsafe{printf("Adt81::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt81 {
Variant0{
fld0: *const i32,
fld1: char,
fld2: [u32; 6],
fld3: [u16; 6],
fld4: (u16, [i8; 1], usize, u32),

},
Variant1{
fld0: *mut i64,

}}

