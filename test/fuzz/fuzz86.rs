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
pub fn fn0(mut _1: i128,mut _2: u32,mut _3: i64,mut _4: u16,mut _5: i16,mut _6: u8) -> [i64; 7] {
mir! {
type RET = [i64; 7];
let _7: ((u8, (((usize, u64), usize, u8, u128), *const char, &'static u16, u32), u8, &'static i32), (u128, u16, i8), isize, (u8, (((usize, u64), usize, u8, u128), *const char, &'static u16, u32), u8, &'static i32));
let _8: bool;
let _9: u32;
let _10: isize;
let _11: isize;
let _12: (u128, u16, i8);
let _13: [u32; 5];
let _14: i64;
let _15: ((u8,), isize, [i8; 2], usize);
let _16: (*const &'static i32,);
let _17: ((char, u16),);
let _18: u128;
let _19: char;
let _20: [char; 5];
let _21: isize;
let _22: u64;
let _23: Adt52;
let _24: ((usize, u64), usize, u8, u128);
let _25: u64;
let _26: ((usize, u64), usize, u8, u128);
let _27: Adt86;
let _28: [isize; 6];
let _29: bool;
let _30: f32;
let _31: f32;
let _32: ([i16; 4],);
let _33: ();
let _34: ();
{
_2 = !2277518078_u32;
_7.1.1 = !31365_u16;
RET = [(-4381666078785367982_i64),1428955641586513632_i64,4570730811989963048_i64,6730233318468069126_i64,(-8933603763166176790_i64),1933537038333750838_i64,(-8576501490558832267_i64)];
_6 = 43_u8;
_7.0.1.0.0.1 = !3096059381770940368_u64;
_7.0.0 = _6;
RET = [2812824323555478998_i64,8459150910152686638_i64,1213748290083884532_i64,131513580854376895_i64,8232563202456262763_i64,(-8124860994445550627_i64),(-1605277883480155476_i64)];
_7.0.1.0.2 = !_6;
_7.0.1.0.1 = 62308849853917259313918064718173039509_u128 as usize;
_7.3.1.0.1 = _7.0.1.0.1 >> _6;
_7.0.1.0.1 = _7.3.1.0.1 ^ _7.3.1.0.1;
_4 = _7.1.1;
_5 = -32204_i16;
_7.0.1.3 = !_2;
_8 = true;
_7.1 = (26871498260000677746136405870484905099_u128, _4, 52_i8);
_7.3.1.0.0.0 = _7.3.1.0.1;
_4 = _7.1.1;
_7.3.0 = (-138227814170991043252122045810072576258_i128) as u8;
_7.1 = (115717120176598540566911497145241171270_u128, _4, 105_i8);
_7.0.1.0.0.1 = 8011097847524636756_u64 | 2093579870012776648_u64;
_7.1.0 = 111897110341018242842657187049094036384_u128;
_7.3.1.3 = _2;
_7.3.2 = _5 as u8;
match _6 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
43 => bb6,
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
_7.3.0 = _6 ^ _6;
_7.0.1.0.3 = _5 as u128;
_7.0.1.0.0.0 = _7.3.1.0.1 << _7.3.1.0.1;
_3 = (-476084281066450520_i64) + (-3441132342923256158_i64);
_7.1.2 = !59_i8;
_11 = 9223372036854775807_isize ^ (-49_isize);
_7.3.1.0.0 = (_7.0.1.0.1, _7.0.1.0.0.1);
_12.1 = !_7.1.1;
RET = [_3,_3,_3,_3,_3,_3,_3];
_2 = _7.0.1.3 - _7.0.1.3;
_7.0.1.0.0.1 = _7.3.1.0.0.0 as u64;
match _6 {
0 => bb4,
43 => bb8,
_ => bb7
}
}
bb7 = {
Return()
}
bb8 = {
_7.0.1.2 = &_4;
_7.3.2 = _6;
_17.0.1 = _5 as u16;
_9 = _7.1.0 as u32;
_7.0.1.3 = !_2;
_7.0.1.0 = (_7.3.1.0.0, _7.3.1.0.0.0, _7.3.0, _7.1.0);
_7.0.1.1 = core::ptr::addr_of!(_17.0.0);
_15.0 = (_7.0.1.0.2,);
_18 = _7.1.0 + _7.1.0;
_7.3.1.0.2 = !_7.0.1.0.2;
_7.3.1 = Move(_7.0.1);
_7.3.1.2 = &_4;
_10 = _11;
_12.2 = !_7.1.2;
_7.1 = (_18, _4, _12.2);
_7.1.2 = -_12.2;
_2 = '\u{3e481}' as u32;
_6 = !_7.3.1.0.2;
_13 = [_7.3.1.3,_2,_9,_7.3.1.3,_2];
_16.0 = core::ptr::addr_of!(_7.0.3);
_17.0 = ('\u{3fe07}', _12.1);
_7.0.2 = !_7.3.2;
_7.0.0 = _9 as u8;
_20 = [_17.0.0,_17.0.0,_17.0.0,_17.0.0,_17.0.0];
_7.0.1.0.0 = (_7.3.1.0.1, _7.3.1.0.0.1);
_15.3 = (-1719263855244062304974642391888527439_i128) as usize;
Call(_17.0.0 = fn1(Move(_7.3.1), _10, _7.0.1.0.0, _20, _7.3.0, _12.1, _6, _7.3.2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_7.3.1.0.0.1 = _5 as u64;
_1 = (-163255199709457591216647699295040695154_i128) * (-68605526815129383942647588703039301721_i128);
_7.1.0 = _18 + _18;
_7.0.1.0.3 = !_18;
_19 = _17.0.0;
_12.0 = _7.0.1.0.3;
_6 = _15.0.0;
_7.3.1.0.0 = (_7.0.1.0.0.0, _7.0.1.0.0.1);
_17.0.0 = _19;
_17.0 = (_19, _7.1.1);
_7.0.1.0.2 = _5 as u8;
_7.3.1.0.2 = _18 as u8;
match _7.3.2 {
0 => bb8,
43 => bb10,
_ => bb2
}
}
bb10 = {
match _7.3.2 {
0 => bb6,
43 => bb11,
_ => bb4
}
}
bb11 = {
_15.1 = -_10;
_24.0.1 = _17.0.1 as u64;
_7.3.1.0.3 = _18 << _12.1;
_24.3 = _7.3.1.0.3;
_12.1 = !_17.0.1;
_7.0.1.0 = (_7.3.1.0.0, _15.3, _15.0.0, _7.1.0);
_26.0 = (_7.3.1.0.0.0, _24.0.1);
_7.0.1.2 = &_12.1;
_7.3.1.0.0.1 = _26.0.1 << _7.3.0;
_12.0 = _7.0.1.0.3 << _18;
_7.3.1.0.0 = (_7.0.1.0.1, _7.0.1.0.0.1);
_7.0.1.0.0.0 = _26.0.0;
_24.3 = !_7.3.1.0.3;
_7.0.1.0.0 = _7.3.1.0.0;
_15.3 = !_26.0.0;
_6 = _7.3.0;
_7.0.1.0.0.0 = !_7.3.1.0.0.0;
_7.3.1.2 = &_17.0.1;
_7.3.1.0.0 = _7.0.1.0.0;
match _7.3.2 {
0 => bb1,
1 => bb7,
2 => bb10,
43 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_17.0.0 = _19;
_7.0.1.0.0 = (_26.0.0, _24.0.1);
_14 = _17.0.0 as i64;
_26.1 = _7.0.1.0.0.0;
_11 = _10;
_7.0.1.0.0.0 = _15.3;
_7.3.1.3 = _4 as u32;
_15.2 = [_12.2,_12.2];
_26.3 = _5 as u128;
_8 = true | true;
_7.3.1.2 = &_17.0.1;
_7.0.1.0.0.0 = _7.3.1.0.0.0;
_21 = _7.3.1.3 as isize;
_7.3.1.1 = core::ptr::addr_of!(_19);
RET = [_14,_3,_14,_3,_14,_14,_3];
_7.3.1.0 = (_26.0, _26.0.0, _7.3.2, _7.0.1.0.3);
_26.3 = _12.2 as u128;
_7.0.0 = _9 as u8;
RET = [_3,_3,_14,_14,_3,_3,_3];
_21 = -_10;
_12.1 = _17.0.1;
_26.0 = _7.3.1.0.0;
_7.1.2 = !_12.2;
_25 = _24.0.1 * _7.3.1.0.0.1;
_10 = _15.1;
_17.0.0 = _19;
Call(_14 = core::intrinsics::transmute(_7.3.1.0.1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_7.1 = _12;
_9 = _7.3.1.3;
RET = [_3,_14,_3,_14,_14,_14,_3];
_7.3.1.0.0.1 = _7.3.1.3 as u64;
_28 = [_21,_15.1,_15.1,_15.1,_21,_11];
_24.0.0 = _26.0.0;
_14 = _7.3.1.3 as i64;
_7.1.1 = _5 as u16;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(0_usize, 2_usize, Move(_2), 21_usize, Move(_21), 8_usize, Move(_8), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(0_usize, 17_usize, Move(_17), 19_usize, Move(_19), 18_usize, Move(_18), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(0_usize, 20_usize, Move(_20), 28_usize, Move(_28), 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: (((usize, u64), usize, u8, u128), *const char, &'static u16, u32),mut _2: isize,mut _3: (usize, u64),mut _4: [char; 5],mut _5: u8,mut _6: u16,mut _7: u8,mut _8: u8) -> char {
mir! {
type RET = char;
let _9: [i8; 2];
let _10: u8;
let _11: isize;
let _12: *const char;
let _13: ((usize, u64), (*const &'static i32,), &'static u128, (u128, u16, i8));
let _14: isize;
let _15: u128;
let _16: f64;
let _17: [isize; 2];
let _18: i8;
let _19: f32;
let _20: bool;
let _21: *const Adt37;
let _22: [i8; 2];
let _23: f64;
let _24: *mut u8;
let _25: Adt37;
let _26: ();
let _27: ();
{
RET = '\u{b527f}';
_1.1 = core::ptr::addr_of!(RET);
RET = '\u{3bb85}';
RET = '\u{17c08}';
RET = '\u{1a09d}';
_8 = !_1.0.2;
_2 = (-83_i8) as isize;
_5 = _8 | _8;
_4 = [RET,RET,RET,RET,RET];
_1.2 = &_6;
_5 = _8;
_8 = _1.0.2;
_1.0.0 = (_3.0, _3.1);
_1.2 = &_6;
_1.0.0 = _3;
_9 = [93_i8,16_i8];
_5 = _3.1 as u8;
_11 = _2;
_1.0.2 = 55_i8 as u8;
_12 = Move(_1.1);
_1.2 = &_6;
match _1.0.3 {
0 => bb1,
1 => bb2,
2 => bb3,
111897110341018242842657187049094036384 => bb5,
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
_1.0.1 = _1.0.0.0;
_1.0.0.1 = !_3.1;
_1.1 = core::ptr::addr_of!(RET);
_10 = _8;
_4 = [RET,RET,RET,RET,RET];
Call(_7 = core::intrinsics::bswap(_8), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_1.0.0.1 = _3.1;
_12 = Move(_1.1);
_8 = _10;
_13.0 = (_1.0.0.0, _1.0.0.1);
_1.2 = &_13.3.1;
_11 = _2;
_1.0.3 = 213566937889993286665472933643053033880_u128;
_13.3.1 = _6;
_1.0.1 = !_13.0.0;
_13.3 = (_1.0.3, _6, (-10_i8));
_15 = _10 as u128;
match _1.0.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
213566937889993286665472933643053033880 => bb7,
_ => bb5
}
}
bb7 = {
_14 = _2;
_13.3.2 = 53_i8;
_5 = !_7;
_13.2 = &_13.3.0;
Goto(bb8)
}
bb8 = {
_1.0.0 = _3;
_11 = _13.0.1 as isize;
_19 = (-67267451550650208799324632917106578264_i128) as f32;
_1.0.3 = 81743558010084722088920495481241306216_i128 as u128;
_1.2 = &_13.3.1;
_4 = [RET,RET,RET,RET,RET];
_17 = [_11,_2];
_13.3.1 = _6;
_1.0.3 = _15 + _13.3.0;
_11 = !_14;
_2 = _14 & _11;
_1.0.2 = _13.3.2 as u8;
_1.0 = (_13.0, _13.0.0, _8, _15);
_19 = _5 as f32;
_12 = core::ptr::addr_of!(RET);
_1.0.0.1 = _3.1 | _3.1;
_9 = [_13.3.2,_13.3.2];
_11 = _2;
_8 = _1.0.2 << _11;
RET = '\u{6ea59}';
_12 = core::ptr::addr_of!((*_12));
_13.3.0 = !_15;
_18 = _13.3.2;
_5 = true as u8;
_3 = (_1.0.1, _1.0.0.1);
Goto(bb9)
}
bb9 = {
_1.0.1 = !_1.0.0.0;
_9 = [_13.3.2,_13.3.2];
_20 = false ^ true;
_1.0.1 = _1.3 as usize;
_1.0.0.1 = _3.1;
_12 = core::ptr::addr_of!((*_12));
_13.3 = (_15, _6, _18);
_19 = _3.1 as f32;
_13.3 = (_1.0.3, _6, _18);
(*_12) = '\u{a5a83}';
_13.0.1 = _3.1;
match _13.3.2 {
0 => bb4,
1 => bb8,
2 => bb5,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
53 => bb15,
_ => bb14
}
}
bb10 = {
_1.0.0 = _3;
_11 = _13.0.1 as isize;
_19 = (-67267451550650208799324632917106578264_i128) as f32;
_1.0.3 = 81743558010084722088920495481241306216_i128 as u128;
_1.2 = &_13.3.1;
_4 = [RET,RET,RET,RET,RET];
_17 = [_11,_2];
_13.3.1 = _6;
_1.0.3 = _15 + _13.3.0;
_11 = !_14;
_2 = _14 & _11;
_1.0.2 = _13.3.2 as u8;
_1.0 = (_13.0, _13.0.0, _8, _15);
_19 = _5 as f32;
_12 = core::ptr::addr_of!(RET);
_1.0.0.1 = _3.1 | _3.1;
_9 = [_13.3.2,_13.3.2];
_11 = _2;
_8 = _1.0.2 << _11;
RET = '\u{6ea59}';
_12 = core::ptr::addr_of!((*_12));
_13.3.0 = !_15;
_18 = _13.3.2;
_5 = true as u8;
_3 = (_1.0.1, _1.0.0.1);
Goto(bb9)
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
_3 = _1.0.0;
_9 = [_18,_18];
_13.3 = (_1.0.3, _6, _18);
_22 = _9;
_14 = _11;
_5 = _8;
_13.0.0 = _8 as usize;
_14 = _13.3.1 as isize;
_3.0 = _13.0.0;
_1.3 = 4152143250_u32;
_23 = _3.1 as f64;
_13.2 = &_1.0.3;
_1.1 = core::ptr::addr_of!((*_12));
_3 = _1.0.0;
(*_12) = '\u{ab254}';
_1.0.0.0 = !_3.0;
_9 = [_13.3.2,_13.3.2];
_3 = (_1.0.0.0, _13.0.1);
_21 = core::ptr::addr_of!(_25);
Goto(bb16)
}
bb16 = {
Call(_26 = dump_var(1_usize, 8_usize, Move(_8), 5_usize, Move(_5), 18_usize, Move(_18), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_26 = dump_var(1_usize, 20_usize, Move(_20), 7_usize, Move(_7), 2_usize, Move(_2), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(142052401641293537411987179077466258765_i128), std::hint::black_box(2985848879_u32), std::hint::black_box(1056625057684780147_i64), std::hint::black_box(7478_u16), std::hint::black_box((-11335_i16)), std::hint::black_box(238_u8));
                
            }
impl PrintFDebug for Adt24{
	unsafe fn printf_debug(&self){unsafe{printf("Adt24::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt24 {
Variant0{
fld0: u64,
fld1: u16,
fld2: f64,
fld3: ((usize, u64), usize, u8, u128),
fld4: i16,
fld5: i32,

},
Variant1{
fld0: (char, u16),
fld1: ((usize, u64), usize, u8, u128),
fld2: i128,
fld3: i8,
fld4: u32,

},
Variant2{
fld0: u32,
fld1: u8,
fld2: u16,

},
Variant3{
fld0: u128,
fld1: (u8,),
fld2: u8,
fld3: i128,
fld4: i16,
fld5: usize,

}}
impl PrintFDebug for Adt37{
	unsafe fn printf_debug(&self){unsafe{printf("Adt37::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt37 {
Variant0{
fld0: (usize, u64),

},
Variant1{
fld0: u128,
fld1: i8,
fld2: f32,

},
Variant2{
fld0: bool,
fld1: usize,
fld2: u16,
fld3: i64,

}}
impl PrintFDebug for Adt38{
	unsafe fn printf_debug(&self){unsafe{printf("Adt38::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt38 {
Variant0{
fld0: i128,

},
Variant1{
fld0: i64,
fld1: (i8, isize, f32),
fld2: f64,
fld3: Adt37,
fld4: *const char,

},
Variant2{
fld0: *mut f64,
fld1: *mut u16,

},
Variant3{
fld0: bool,
fld1: i64,
fld2: (u8,),
fld3: i32,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
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
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: i16,
fld1: u64,
fld2: f64,
fld3: u128,

},
Variant1{
fld0: [i8; 2],

},
Variant2{
fld0: (usize, u64),
fld1: usize,
fld2: u32,
fld3: ((char, u16),),
fld4: i16,
fld5: Adt24,
fld6: [usize; 5],
fld7: i128,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: *const u16,
fld1: i32,
fld2: u16,
fld3: i8,
fld4: i16,

},
Variant1{
fld0: *mut u16,

}}
impl PrintFDebug for Adt65{
	unsafe fn printf_debug(&self){unsafe{printf("Adt65::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt65 {
Variant0{
fld0: u64,
fld1: (u128, u16, i8),
fld2: ([usize; 5],),

},
Variant1{
fld0: i8,
fld1: *mut ([usize; 5],),

}}
impl PrintFDebug for Adt77{
	unsafe fn printf_debug(&self){unsafe{printf("Adt77::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt77 {
Variant0{
fld0: [i32; 7],
fld1: [i8; 2],

},
Variant1{
fld0: [isize; 6],
fld1: Adt37,
fld2: (u8,),
fld3: *mut ([usize; 5],),
fld4: *const u16,
fld5: ((char, u16),),
fld6: (char, u16),

}}
impl PrintFDebug for Adt86{
	unsafe fn printf_debug(&self){unsafe{printf("Adt86::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt86 {
Variant0{
fld0: *const [u32; 5],

},
Variant1{
fld0: Adt52,
fld1: char,
fld2: [u32; 5],
fld3: [i32; 7],
fld4: Adt65,
fld5: *const Adt38,

},
Variant2{
fld0: [i16; 4],
fld1: [i64; 7],
fld2: [u32; 6],
fld3: *const bool,
fld4: ((char, u16),),
fld5: f64,

}}

