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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: u128,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16) -> [u128; 1] {
mir! {
type RET = [u128; 1];
let _12: bool;
let _13: (usize,);
let _14: i8;
let _15: &'static Adt22;
let _16: [u128; 1];
let _17: f32;
let _18: &'static &'static i8;
let _19: *const f32;
let _20: f64;
let _21: [u16; 4];
let _22: isize;
let _23: &'static f64;
let _24: &'static *mut (u8, u16, i16, Adt22);
let _25: (((i16, [u32; 3], f64, (u8, u16, i16, Adt22)), [u16; 2], &'static *mut (u8, u16, i16, Adt22)), (bool,), &'static isize);
let _26: u8;
let _27: u16;
let _28: isize;
let _29: (usize, i128, char, &'static isize);
let _30: i32;
let _31: f32;
let _32: bool;
let _33: f64;
let _34: char;
let _35: f32;
let _36: i32;
let _37: (char, &'static isize);
let _38: *const [char; 4];
let _39: &'static *mut Adt32;
let _40: [i64; 3];
let _41: Adt27;
let _42: ();
let _43: ();
{
_1 = 100_i8 < 35_i8;
_2 = '\u{c9c89}';
_1 = _2 < _2;
_3 = 9223372036854775807_isize >> 2321979257_u32;
_10 = (-31293_i16) as u8;
_10 = 215_u8;
_6 = (-993781083_i32) * (-552169086_i32);
RET = [302296415381158402357738296816179692198_u128];
_5 = 852220628_u32 as u128;
_6 = 42468757_i32 ^ 876747460_i32;
_9 = 2818767012741892867_u64 as usize;
_12 = _1 | _1;
_1 = _12;
_9 = !4742911542057587942_usize;
_12 = _1;
_4 = _2 as i8;
RET = [_5];
RET = [_5];
_14 = !_4;
_13.0 = !_9;
_7 = _5 as i64;
RET = [_5];
RET = [_5];
RET = [_5];
_14 = _4;
Call(_13 = fn1(_4, _9, _9, _3, _2, _3, _12), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = 101_u8 >> _13.0;
_1 = _12 < _12;
_9 = _13.0 - _13.0;
_3 = (-9223372036854775808_isize) >> _6;
_13.0 = _7 as usize;
RET = [_5];
_14 = _5 as i8;
_16 = [_5];
_10 = 118_u8 & 170_u8;
Call(_6 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = !(-14319580414533986141678134023307938668_i128);
RET = [_5];
_11 = _10 as u16;
_9 = !_13.0;
_11 = !30375_u16;
_11 = 9182_u16;
_12 = _1;
_10 = _12 as u8;
_16 = [_5];
_5 = !81287149750983447298505315773676202803_u128;
Goto(bb3)
}
bb3 = {
_1 = !_12;
_1 = _12 < _12;
_4 = _14;
_19 = core::ptr::addr_of!(_17);
_7 = (-450235651857431302_i64) ^ (-1395434683848098650_i64);
_16 = RET;
(*_19) = _7 as f32;
match _11 {
9182 => bb5,
_ => bb4
}
}
bb4 = {
_8 = !(-14319580414533986141678134023307938668_i128);
RET = [_5];
_11 = _10 as u16;
_9 = !_13.0;
_11 = !30375_u16;
_11 = 9182_u16;
_12 = _1;
_10 = _12 as u8;
_16 = [_5];
_5 = !81287149750983447298505315773676202803_u128;
Goto(bb3)
}
bb5 = {
_19 = core::ptr::addr_of!((*_19));
(*_19) = _3 as f32;
_10 = !120_u8;
_13.0 = _10 as usize;
_13.0 = _1 as usize;
_1 = _12;
_13 = (_9,);
_10 = !111_u8;
_22 = _3;
_19 = core::ptr::addr_of!((*_19));
_19 = core::ptr::addr_of!((*_19));
(*_19) = _13.0 as f32;
(*_19) = _3 as f32;
_7 = -9166244241680639542_i64;
_17 = 1160660895_u32 as f32;
_6 = !(-708800278_i32);
_25.0.0.3.0 = !_10;
_25.1.0 = !_1;
_25.1 = (_1,);
_25.0.0.0 = 18351_i16 & (-16339_i16);
_8 = 141367625721434981818513124984018713270_i128;
_13 = (_9,);
_21 = [_11,_11,_11,_11];
Call(_25.0.0.3.1 = core::intrinsics::bswap(_11), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Goto(bb7)
}
bb7 = {
_5 = _6 as u128;
_25.0.1 = [_11,_11];
_2 = '\u{25e22}';
_25.1 = (_12,);
_15 = &_25.0.0.3.3;
_29.3 = &_3;
_7 = 359153884642578959_i64;
_25.0.1 = [_11,_11];
_7 = (-6151875236818230065_i64);
_29.2 = _2;
_26 = !_25.0.0.3.0;
_32 = _25.1.0 >= _12;
_33 = _25.0.0.0 as f64;
_9 = _11 as usize;
_21 = [_11,_11,_11,_11];
_29.0 = !_9;
_29.0 = _9 << _25.0.0.3.0;
_34 = _29.2;
match _8 {
0 => bb8,
1 => bb9,
2 => bb10,
141367625721434981818513124984018713270 => bb12,
_ => bb11
}
}
bb8 = {
Goto(bb7)
}
bb9 = {
_10 = 101_u8 >> _13.0;
_1 = _12 < _12;
_9 = _13.0 - _13.0;
_3 = (-9223372036854775808_isize) >> _6;
_13.0 = _7 as usize;
RET = [_5];
_14 = _5 as i8;
_16 = [_5];
_10 = 118_u8 & 170_u8;
Call(_6 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_8 = !(-14319580414533986141678134023307938668_i128);
RET = [_5];
_11 = _10 as u16;
_9 = !_13.0;
_11 = !30375_u16;
_11 = 9182_u16;
_12 = _1;
_10 = _12 as u8;
_16 = [_5];
_5 = !81287149750983447298505315773676202803_u128;
Goto(bb3)
}
bb11 = {
_1 = !_12;
_1 = _12 < _12;
_4 = _14;
_19 = core::ptr::addr_of!(_17);
_7 = (-450235651857431302_i64) ^ (-1395434683848098650_i64);
_16 = RET;
(*_19) = _7 as f32;
match _11 {
9182 => bb5,
_ => bb4
}
}
bb12 = {
_16 = RET;
_23 = &_20;
_35 = _17;
_3 = 13998806445946782002_u64 as isize;
_30 = _9 as i32;
_11 = _5 as u16;
_23 = &(*_23);
_31 = (*_19);
_28 = _29.2 as isize;
_4 = !_14;
RET = _16;
_21 = [_11,_11,_11,_11];
_37.1 = &_22;
_15 = &(*_15);
_27 = _32 as u16;
_13 = (_9,);
_29.3 = &_22;
_20 = -_33;
_25.0.1 = [_27,_27];
_23 = &_33;
_25.0.0.1 = [1041658504_u32,3335018755_u32,2617504003_u32];
match _8 {
141367625721434981818513124984018713270 => bb14,
_ => bb13
}
}
bb13 = {
Goto(bb7)
}
bb14 = {
_29.3 = &_22;
_15 = &_25.0.0.3.3;
_28 = _22 << _6;
_28 = _22;
(*_19) = _31;
_25.0.0.3.0 = _26 >> _27;
_25.0.0.3.3 = Adt22::Variant0 { fld0: _25.1.0 };
_30 = _17 as i32;
_25.0.0.2 = _20 + _20;
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(0_usize, 4_usize, Move(_4), 28_usize, Move(_28), 27_usize, Move(_27), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(0_usize, 10_usize, Move(_10), 1_usize, Move(_1), 26_usize, Move(_26), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(0_usize, 9_usize, Move(_9), 8_usize, Move(_8), 5_usize, Move(_5), 43_usize, _43), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i8,mut _2: usize,mut _3: usize,mut _4: isize,mut _5: char,mut _6: isize,mut _7: bool) -> (usize,) {
mir! {
type RET = (usize,);
let _8: i32;
let _9: u8;
let _10: (usize,);
let _11: i32;
let _12: Adt47;
let _13: i32;
let _14: &'static isize;
let _15: f64;
let _16: (isize, f32, u32);
let _17: char;
let _18: usize;
let _19: u32;
let _20: &'static Adt32;
let _21: i64;
let _22: char;
let _23: [usize; 4];
let _24: char;
let _25: isize;
let _26: u8;
let _27: (u8,);
let _28: i64;
let _29: char;
let _30: ();
let _31: ();
{
_1 = 51919141497691154179751226653111028841_i128 as i8;
_6 = -_4;
_3 = !_2;
RET.0 = !_2;
_1 = (-100_i8) * (-117_i8);
_5 = '\u{b9229}';
_3 = !RET.0;
_6 = -_4;
RET.0 = _3;
_3 = (-14208_i16) as usize;
_4 = -_6;
Call(_1 = fn2(_6, _4, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = (_3,);
RET = (_3,);
_1 = 14209_i16 as i8;
_5 = '\u{cc06f}';
Goto(bb2)
}
bb2 = {
_1 = 70_i8;
_1 = 9490227103414281009_u64 as i8;
_3 = !RET.0;
RET = (_3,);
RET = (_2,);
RET.0 = _2;
_5 = '\u{6bed}';
_3 = _4 as usize;
RET = (_3,);
_2 = !RET.0;
_2 = !_3;
_2 = 20360_u16 as usize;
_7 = _5 <= _5;
_5 = '\u{a304}';
_3 = _2 - RET.0;
_6 = 234_u8 as isize;
RET = (_2,);
Goto(bb3)
}
bb3 = {
_2 = !_3;
_8 = 328343082016843691562344013973885505465_u128 as i32;
_5 = '\u{9b892}';
_2 = RET.0 & _3;
_6 = _4;
_8 = 41010_u16 as i32;
_7 = _3 < _2;
Call(_4 = core::intrinsics::bswap(_6), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET.0 = 257949394401016355964756620533393820158_u128 as usize;
_9 = 211_u8 * 250_u8;
_6 = _9 as isize;
RET.0 = _3 - _2;
_9 = 37_u8 & 51_u8;
_5 = '\u{101072}';
_9 = 6204100551450759489_i64 as u8;
_6 = -_4;
RET = (_2,);
RET.0 = _8 as usize;
Call(_8 = fn3(_2, _7, _9, _2, _4, _7), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET.0 = _2 >> _2;
_3 = _2 * _2;
RET.0 = !_3;
_6 = _4;
Goto(bb6)
}
bb6 = {
_8 = 1131762769_i32;
RET = (_3,);
_8 = 1228225125_i32;
_6 = _4;
_13 = 9446383361715373045_u64 as i32;
_1 = (-87_i8);
_10 = (_3,);
_14 = &_6;
_9 = _5 as u8;
_13 = _8;
_15 = 4660_i16 as f64;
RET = (_2,);
_3 = _1 as usize;
_9 = !198_u8;
RET = (_10.0,);
_9 = _8 as u8;
_4 = 17939_u16 as isize;
_16.2 = 4154568068_u32 & 1442155252_u32;
_4 = !_6;
_16.2 = 903459131_u32 << _6;
RET.0 = _3 | _10.0;
RET = (_10.0,);
RET = (_3,);
_11 = !_13;
_10.0 = _2 << _1;
_16.1 = _16.2 as f32;
match _8 {
1228225125 => bb8,
_ => bb7
}
}
bb7 = {
_2 = !_3;
_8 = 328343082016843691562344013973885505465_u128 as i32;
_5 = '\u{9b892}';
_2 = RET.0 & _3;
_6 = _4;
_8 = 41010_u16 as i32;
_7 = _3 < _2;
Call(_4 = core::intrinsics::bswap(_6), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_16.2 = 2907629052_u32 | 2493227022_u32;
_15 = (-20195_i16) as f64;
_1 = (-38_i8);
_16.0 = (*_14) << _4;
_17 = _5;
_18 = !_2;
_9 = 254_u8;
RET = (_18,);
_6 = _7 as isize;
_10.0 = !_18;
_1 = (-109_i8);
match _8 {
0 => bb9,
1228225125 => bb11,
_ => bb10
}
}
bb9 = {
_1 = 70_i8;
_1 = 9490227103414281009_u64 as i8;
_3 = !RET.0;
RET = (_3,);
RET = (_2,);
RET.0 = _2;
_5 = '\u{6bed}';
_3 = _4 as usize;
RET = (_3,);
_2 = !RET.0;
_2 = !_3;
_2 = 20360_u16 as usize;
_7 = _5 <= _5;
_5 = '\u{a304}';
_3 = _2 - RET.0;
_6 = 234_u8 as isize;
RET = (_2,);
Goto(bb3)
}
bb10 = {
RET.0 = 257949394401016355964756620533393820158_u128 as usize;
_9 = 211_u8 * 250_u8;
_6 = _9 as isize;
RET.0 = _3 - _2;
_9 = 37_u8 & 51_u8;
_5 = '\u{101072}';
_9 = 6204100551450759489_i64 as u8;
_6 = -_4;
RET = (_2,);
RET.0 = _8 as usize;
Call(_8 = fn3(_2, _7, _9, _2, _4, _7), ReturnTo(bb5), UnwindUnreachable())
}
bb11 = {
_10 = RET;
_2 = _5 as usize;
Goto(bb12)
}
bb12 = {
_10 = (RET.0,);
_11 = 16225281302482446131_u64 as i32;
_14 = &_6;
_18 = RET.0;
_10 = (_18,);
_16.2 = 3229022003_u32 ^ 275192291_u32;
_16.1 = (-31535408454620862617675430160179804711_i128) as f32;
RET.0 = _18;
_5 = _17;
_10.0 = !RET.0;
_19 = (-8289177939060850248_i64) as u32;
match _8 {
0 => bb11,
1228225125 => bb13,
_ => bb5
}
}
bb13 = {
_7 = !false;
_4 = !(*_14);
_23 = [_18,_3,RET.0,_10.0];
_11 = 155916204610652236256208367599942339545_i128 as i32;
_21 = 3433112713856753784_i64;
_19 = _16.2;
_4 = -_6;
_5 = _17;
_16.2 = !_19;
_27.0 = _15 as u8;
match _21 {
0 => bb1,
1 => bb11,
2 => bb10,
3 => bb4,
4 => bb5,
5 => bb12,
6 => bb7,
3433112713856753784 => bb14,
_ => bb8
}
}
bb14 = {
_2 = RET.0 & _10.0;
_5 = _17;
_16.0 = _1 as isize;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(1_usize, 10_usize, Move(_10), 19_usize, Move(_19), 7_usize, Move(_7), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(1_usize, 23_usize, Move(_23), 13_usize, Move(_13), 3_usize, Move(_3), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(1_usize, 21_usize, Move(_21), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: isize,mut _2: isize,mut _3: usize) -> i8 {
mir! {
type RET = i8;
let _4: u64;
let _5: (u8,);
let _6: [u16; 4];
let _7: Adt54;
let _8: (Adt54, (u8,));
let _9: isize;
let _10: i8;
let _11: Adt27;
let _12: u64;
let _13: Adt22;
let _14: (isize, f32, u32);
let _15: &'static *mut Adt32;
let _16: bool;
let _17: [i128; 7];
let _18: (usize, i128, char, &'static isize);
let _19: usize;
let _20: ();
let _21: ();
{
RET = (-72_i8);
RET = (-61_i8);
_4 = 12706639687394117803_u64 >> _2;
_6 = [39508_u16,63812_u16,33240_u16,7835_u16];
_6 = [30956_u16,2002_u16,1197_u16,44693_u16];
RET = 81_i8 | 92_i8;
_5 = (81_u8,);
RET = 25_i8;
_3 = _4 as usize;
RET = 80_i8;
RET = 38_i8 << _1;
_3 = !2_usize;
_6 = [12949_u16,23281_u16,56687_u16,46967_u16];
RET = 126_i8 - (-11_i8);
match _5.0 {
0 => bb1,
1 => bb2,
81 => bb4,
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
_5 = (161_u8,);
_6 = [50998_u16,57303_u16,14018_u16,37600_u16];
_2 = !_1;
_8.1.0 = !_5.0;
_5 = (_8.1.0,);
_1 = _2 ^ _2;
RET = (-8_i8);
RET = -57_i8;
_8.1.0 = !_5.0;
_8.1 = (_5.0,);
RET = (-102_i8);
_5.0 = 6921744486737509269_i64 as u8;
_6 = [62619_u16,35344_u16,56267_u16,50376_u16];
RET = 95_i8 + 26_i8;
_3 = !9738210170678921839_usize;
_5 = (_8.1.0,);
_9 = _1;
_9 = (-131341233552310494022636312131876629004_i128) as isize;
RET = 113_i8 >> _3;
_3 = !2011945333798217231_usize;
_8.1 = (_5.0,);
_3 = (-14357_i16) as usize;
_1 = _2 - _2;
RET = (-90_i8) ^ (-1_i8);
Goto(bb5)
}
bb5 = {
_5 = (_8.1.0,);
_12 = _4 & _4;
_4 = 2873684157_u32 as u64;
_10 = -RET;
_8.1 = (_5.0,);
_1 = _2 << _2;
RET = !_10;
_9 = _1 | _1;
_4 = !_12;
_3 = 9040396465516034781_usize & 15491458119493255082_usize;
_3 = 6_usize;
_5.0 = !_8.1.0;
_5.0 = '\u{4eb8a}' as u8;
_9 = !_2;
_11 = Adt27::Variant2 { fld0: false };
match _3 {
0 => bb6,
6 => bb8,
_ => bb7
}
}
bb6 = {
_5 = (161_u8,);
_6 = [50998_u16,57303_u16,14018_u16,37600_u16];
_2 = !_1;
_8.1.0 = !_5.0;
_5 = (_8.1.0,);
_1 = _2 ^ _2;
RET = (-8_i8);
RET = -57_i8;
_8.1.0 = !_5.0;
_8.1 = (_5.0,);
RET = (-102_i8);
_5.0 = 6921744486737509269_i64 as u8;
_6 = [62619_u16,35344_u16,56267_u16,50376_u16];
RET = 95_i8 + 26_i8;
_3 = !9738210170678921839_usize;
_5 = (_8.1.0,);
_9 = _1;
_9 = (-131341233552310494022636312131876629004_i128) as isize;
RET = 113_i8 >> _3;
_3 = !2011945333798217231_usize;
_8.1 = (_5.0,);
_3 = (-14357_i16) as usize;
_1 = _2 - _2;
RET = (-90_i8) ^ (-1_i8);
Goto(bb5)
}
bb7 = {
Return()
}
bb8 = {
_5 = (_8.1.0,);
_8.1.0 = _10 as u8;
_12 = _4 ^ _4;
_1 = _2;
_17[_3] = 124719399035528791134914130563754493930_i128;
_1 = _9 & _9;
_14.2 = 151890663_u32 << _4;
_14.1 = 217878015409875949274174822000861596982_u128 as f32;
_2 = _9;
_2 = 195967989723602015077421478094132422877_u128 as isize;
_8.1.0 = _5.0;
_18.0 = !_3;
_18.2 = '\u{9bd72}';
place!(Field::<bool>(Variant(_11, 2), 0)) = _12 == _12;
_4 = (-3482_i16) as u64;
_14.0 = 12400_u16 as isize;
_16 = !Field::<bool>(Variant(_11, 2), 0);
_18.0 = _3 - _3;
_5 = (_8.1.0,);
Call(_14.2 = core::intrinsics::bswap(1297376250_u32), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_5 = (_8.1.0,);
_13 = Adt22::Variant2 { fld0: _14.1,fld1: 2298440838232510738_i64,fld2: 64811423_i32 };
_18.3 = &_1;
RET = 4330_i16 as i8;
_18.1 = -_17[_3];
place!(Field::<i32>(Variant(_13, 2), 2)) = -197589023_i32;
place!(Field::<bool>(Variant(_11, 2), 0)) = _16 != _16;
place!(Field::<i32>(Variant(_13, 2), 2)) = !(-685753949_i32);
_5.0 = 88230737618941273164988000200270909303_u128 as u8;
place!(Field::<i64>(Variant(_13, 2), 1)) = -(-9036756548236615037_i64);
match _3 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb6,
4 => bb10,
5 => bb11,
6 => bb14,
_ => bb13
}
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
_5 = (161_u8,);
_6 = [50998_u16,57303_u16,14018_u16,37600_u16];
_2 = !_1;
_8.1.0 = !_5.0;
_5 = (_8.1.0,);
_1 = _2 ^ _2;
RET = (-8_i8);
RET = -57_i8;
_8.1.0 = !_5.0;
_8.1 = (_5.0,);
RET = (-102_i8);
_5.0 = 6921744486737509269_i64 as u8;
_6 = [62619_u16,35344_u16,56267_u16,50376_u16];
RET = 95_i8 + 26_i8;
_3 = !9738210170678921839_usize;
_5 = (_8.1.0,);
_9 = _1;
_9 = (-131341233552310494022636312131876629004_i128) as isize;
RET = 113_i8 >> _3;
_3 = !2011945333798217231_usize;
_8.1 = (_5.0,);
_3 = (-14357_i16) as usize;
_1 = _2 - _2;
RET = (-90_i8) ^ (-1_i8);
Goto(bb5)
}
bb14 = {
_6 = [12567_u16,44352_u16,51420_u16,31567_u16];
place!(Field::<bool>(Variant(_11, 2), 0)) = _16;
_18.3 = &_14.0;
_5 = (_8.1.0,);
_18.0 = _3;
_12 = Field::<i64>(Variant(_13, 2), 1) as u64;
_14.2 = 2042167604_u32 * 1554634059_u32;
_16 = Field::<bool>(Variant(_11, 2), 0);
_16 = !Field::<bool>(Variant(_11, 2), 0);
RET = Field::<i64>(Variant(_13, 2), 1) as i8;
_10 = 9861_u16 as i8;
_4 = Field::<i64>(Variant(_13, 2), 1) as u64;
SetDiscriminant(_11, 0);
_17[_3] = _18.1;
_16 = false;
_11 = Adt27::Variant2 { fld0: _16 };
place!(Field::<bool>(Variant(_11, 2), 0)) = _16;
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(2_usize, 9_usize, Move(_9), 3_usize, Move(_3), 4_usize, Move(_4), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_20 = dump_var(2_usize, 12_usize, Move(_12), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: usize,mut _2: bool,mut _3: u8,mut _4: usize,mut _5: isize,mut _6: bool) -> i32 {
mir! {
type RET = i32;
let _7: isize;
let _8: u64;
let _9: f32;
let _10: Adt48;
let _11: [u16; 2];
let _12: isize;
let _13: i128;
let _14: &'static *mut Adt32;
let _15: [bool; 1];
let _16: u64;
let _17: Adt32;
let _18: *const f32;
let _19: isize;
let _20: &'static [usize; 4];
let _21: isize;
let _22: [u128; 1];
let _23: &'static [usize; 4];
let _24: char;
let _25: [char; 4];
let _26: isize;
let _27: i128;
let _28: ();
let _29: ();
{
RET = 1481325920_i32 >> _5;
_3 = !255_u8;
RET = -93119783_i32;
_2 = !_6;
RET = -(-827163554_i32);
_4 = 231885206505709299525429343869047121322_u128 as usize;
_5 = 9223372036854775807_isize;
_3 = !157_u8;
_8 = 89514343902424787535381676826516594879_u128 as u64;
_3 = _6 as u8;
Goto(bb1)
}
bb1 = {
_1 = _4 & _4;
Goto(bb2)
}
bb2 = {
_6 = _4 == _1;
_4 = !_1;
_3 = 214_u8 >> _4;
_6 = !_2;
_3 = 150_u8 * 183_u8;
_5 = '\u{c2790}' as isize;
_1 = _4;
_8 = 2139104118_u32 as u64;
_7 = _5;
_6 = _2;
_7 = _5;
_4 = _1;
_7 = _5;
_2 = _1 != _1;
_3 = !93_u8;
_1 = _4;
_11 = [23628_u16,44671_u16];
_8 = !6949657255970939326_u64;
_5 = -_7;
_2 = _6;
RET = _6 as i32;
_3 = 58_u8 >> _7;
_5 = -_7;
_5 = _7;
_1 = _7 as usize;
RET = (-1065485678_i32);
_12 = 15520_u16 as isize;
match RET {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
340282366920938463463374607430702725778 => bb10,
_ => bb9
}
}
bb3 = {
_1 = _4 & _4;
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
_2 = _3 != _3;
_13 = -(-37767171165132724561176795955966593835_i128);
_8 = 9306248278832236035_u64;
RET = '\u{282d5}' as i32;
RET = (-657813612_i32);
_3 = 46_u8;
Goto(bb11)
}
bb11 = {
_7 = !_5;
_9 = _8 as f32;
_7 = _5;
_13 = 18874626597263859993344320394694931584_u128 as i128;
_6 = _2 ^ _2;
_5 = '\u{f2bd7}' as isize;
_2 = _6;
_15 = [_6];
RET = 111610355_i32 - (-2034286252_i32);
_16 = 13331_i16 as u64;
_13 = !(-62227169955554058088092212065251055287_i128);
_6 = _2;
_11 = [36649_u16,47038_u16];
_11 = [44811_u16,21751_u16];
_12 = '\u{4d5f5}' as isize;
_5 = _7 - _12;
_12 = (-7250117968706302827_i64) as isize;
_2 = _8 > _8;
_9 = 191805031195382030785759801644768681059_u128 as f32;
_16 = !_8;
_12 = _7 >> _1;
Goto(bb12)
}
bb12 = {
_2 = !_6;
_11 = [23433_u16,43444_u16];
RET = (-1481078795_i32);
_4 = !_1;
_15 = [_2];
_6 = _2 ^ _2;
_2 = !_6;
_4 = _1;
_19 = _7;
_2 = !_6;
_2 = _6;
_15 = [_6];
_1 = _12 as usize;
_13 = _5 as i128;
_2 = !_6;
_4 = _1 << _19;
_3 = '\u{4b53d}' as u8;
_18 = core::ptr::addr_of!(_9);
_2 = _6;
_16 = _8;
_3 = !13_u8;
_6 = !_2;
_19 = _12 * _12;
_2 = !_6;
(*_18) = 1891301682_u32 as f32;
_21 = _7 & _19;
RET = (-864120139_i32) * 187908571_i32;
Call(_13 = fn4(_12, _2, _15, _5, _21, _2, _1, _4, _15), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_18 = core::ptr::addr_of!(_9);
_17 = Adt32::Variant3 { fld0: _6,fld1: 45053709373033213579481888925417756911_u128,fld2: _21,fld3: (-25_i8),fld4: RET };
_16 = 33677_u16 as u64;
_1 = _4 + _4;
(*_18) = _13 as f32;
_22 = [38606512571509090512328964011604005689_u128];
_12 = _19 + Field::<isize>(Variant(_17, 3), 2);
_11 = [23022_u16,50831_u16];
_17 = Adt32::Variant0 { fld0: 7770839182184191170_i64,fld1: RET };
_8 = _16 >> _13;
_27 = _13 ^ _13;
_22 = [327792169147699254031277503483597802072_u128];
_8 = !_16;
place!(Field::<i64>(Variant(_17, 0), 0)) = (-6125675943911769575_i64);
Goto(bb14)
}
bb14 = {
_25 = ['\u{afc88}','\u{ca43c}','\u{74e6a}','\u{93b70}'];
_3 = _21 as u8;
(*_18) = (-93_i8) as f32;
_7 = _21;
_6 = !_2;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(3_usize, 6_usize, Move(_6), 12_usize, Move(_12), 25_usize, Move(_25), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(3_usize, 19_usize, Move(_19), 21_usize, Move(_21), 2_usize, Move(_2), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(3_usize, 13_usize, Move(_13), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: isize,mut _2: bool,mut _3: [bool; 1],mut _4: isize,mut _5: isize,mut _6: bool,mut _7: usize,mut _8: usize,mut _9: [bool; 1]) -> i128 {
mir! {
type RET = i128;
let _10: Adt22;
let _11: char;
let _12: &'static Adt32;
let _13: Adt54;
let _14: [bool; 1];
let _15: usize;
let _16: *const isize;
let _17: [u32; 3];
let _18: *const Adt27;
let _19: isize;
let _20: char;
let _21: (((i16, [u32; 3], f64, (u8, u16, i16, Adt22)), [u16; 2], &'static *mut (u8, u16, i16, Adt22)), (bool,), &'static isize);
let _22: [i16; 6];
let _23: char;
let _24: &'static u128;
let _25: usize;
let _26: Adt80;
let _27: [i64; 3];
let _28: (i8, u128, i128, Adt27);
let _29: isize;
let _30: isize;
let _31: i32;
let _32: f32;
let _33: [u16; 2];
let _34: f64;
let _35: i128;
let _36: ();
let _37: ();
{
_7 = !_8;
RET = (-87010425033386952962757116879321375268_i128);
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
253271941887551510500617490552446836188 => bb6,
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
_7 = _8 - _8;
_2 = _1 >= _4;
_4 = _5 - _5;
RET = (-105728203220526582473707213114430851972_i128);
_2 = !_6;
_6 = _2;
_2 = _6;
_1 = RET as isize;
_5 = _4 + _4;
_4 = -_5;
_3 = [_2];
_1 = 44690_u16 as isize;
_11 = '\u{30bbe}';
_8 = 20443_i16 as usize;
_7 = _8;
_9 = _3;
_7 = (-128_i8) as usize;
_5 = _4;
_5 = _4 | _4;
_7 = _8 >> _4;
_6 = !_2;
_8 = !_7;
_6 = !_2;
_5 = 1804044238_i32 as isize;
_5 = _11 as isize;
_5 = -_4;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
234554163700411880989667394317337359484 => bb8,
_ => bb7
}
}
bb7 = {
Return()
}
bb8 = {
_9 = [_6];
_11 = '\u{bd220}';
_2 = !_6;
RET = (-55_i8) as i128;
_15 = _8 - _7;
_15 = (-34_i8) as usize;
_3 = [_2];
_7 = _8 | _8;
RET = _11 as i128;
Call(_9 = fn5(_6, _4, _3, _8), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_5 = _4 + _4;
_15 = !_7;
_17 = [1807355662_u32,1137759860_u32,2451576091_u32];
_16 = core::ptr::addr_of!(_5);
_5 = _4;
_14 = _3;
_1 = _4;
_14 = [_6];
(*_16) = !_4;
Goto(bb10)
}
bb10 = {
_9 = _3;
(*_16) = _4;
RET = 98812205626953331318611317696928666522_i128 * 106900272032647978659230589531500985391_i128;
Goto(bb11)
}
bb11 = {
_5 = _4;
_19 = (*_16);
_7 = _15 * _15;
RET = !132935085960265099850199211901907798472_i128;
_17 = [2674310489_u32,793072007_u32,1591858749_u32];
_7 = 18_u8 as usize;
_20 = _11;
_8 = _15 + _15;
_3 = [_6];
_14 = [_2];
_21.0.1 = [17626_u16,18559_u16];
_21.0.0.2 = RET as f64;
(*_16) = _19 << _15;
_21.0.1 = [38200_u16,60247_u16];
_22 = [4493_i16,25695_i16,(-4554_i16),(-21224_i16),18561_i16,(-28425_i16)];
_1 = 6607702885111129516_u64 as isize;
Goto(bb12)
}
bb12 = {
_21.0.0.3.2 = 3332_i16;
_21.0.0.2 = (-54_i8) as f64;
_1 = (*_16);
_14 = [_2];
_21.0.0.3.3 = Adt22::Variant0 { fld0: _6 };
_21.1 = (Field::<bool>(Variant(_21.0.0.3.3, 0), 0),);
(*_16) = _4;
_4 = _5;
_21.0.0.3.1 = 48972_u16 | 12628_u16;
_10 = Move(_21.0.0.3.3);
_21.0.0.0 = (-1766234686_i32) as i16;
_5 = _19 | _1;
_26.fld1.1.0 = 6690517388841491771_i64 as u8;
_26.fld1.1.0 = 218_u8 << _1;
_23 = _20;
_26.fld4 = [_21.0.0.3.1,_21.0.0.3.1,_21.0.0.3.1,_21.0.0.3.1,_21.0.0.3.1];
_30 = (-4523823897558983681_i64) as isize;
_24 = &_28.1;
Goto(bb13)
}
bb13 = {
_21.0.0.3 = (_26.fld1.1.0, 16458_u16, _21.0.0.0, Move(_10));
_26.fld2 = 3417075533186220196_i64;
place!(Field::<bool>(Variant(_21.0.0.3.3, 0), 0)) = _6;
_28.0 = 108_i8 | (-26_i8);
_28.2 = RET + RET;
_26.fld4 = [_21.0.0.3.1,_21.0.0.3.1,_21.0.0.3.1,_21.0.0.3.1,_21.0.0.3.1];
RET = _28.0 as i128;
_30 = _5;
_21.0.1 = [_21.0.0.3.1,_21.0.0.3.1];
SetDiscriminant(_21.0.0.3.3, 0);
_27 = [_26.fld2,_26.fld2,_26.fld2];
_27 = [_26.fld2,_26.fld2,_26.fld2];
_26.fld0 = !1635209039_i32;
_21.0.0.3.1 = 290190154594343030308548125197032844042_u128 as u16;
_2 = !_21.1.0;
_26.fld3 = [_26.fld2,_26.fld2,_26.fld2];
place!(Field::<bool>(Variant(_21.0.0.3.3, 0), 0)) = !_21.1.0;
_8 = !_15;
Goto(bb14)
}
bb14 = {
RET = _28.2;
RET = _28.0 as i128;
_21.0.0.2 = _8 as f64;
_14 = [_6];
_28.0 = (-44_i8);
_28.3 = Adt27::Variant1 { fld0: 4751755052528978182_u64,fld1: _26.fld4 };
_34 = _26.fld0 as f64;
_25 = _21.0.0.3.1 as usize;
_32 = _21.0.0.3.1 as f32;
_21.0.0.1 = [477246399_u32,2040510945_u32,862380571_u32];
_23 = _20;
_26.fld2 = (-5174583103581261953_i64);
_2 = _21.1.0;
_15 = _21.0.0.3.1 as usize;
_27 = _26.fld3;
_14 = [Field::<bool>(Variant(_21.0.0.3.3, 0), 0)];
_21.0.0.1 = _17;
(*_16) = _28.0 as isize;
_35 = -RET;
_21.0.1 = [_21.0.0.3.1,_21.0.0.3.1];
_26.fld2 = 3270852034979028065_i64 | 3594746830057297954_i64;
_26.fld1.1.0 = 4726513816415928119_u64 as u8;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(4_usize, 2_usize, Move(_2), 17_usize, Move(_17), 6_usize, Move(_6), 35_usize, Move(_35)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(4_usize, 19_usize, Move(_19), 11_usize, Move(_11), 25_usize, Move(_25), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(4_usize, 27_usize, Move(_27), 8_usize, Move(_8), 37_usize, _37, 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: bool,mut _2: isize,mut _3: [bool; 1],mut _4: usize) -> [bool; 1] {
mir! {
type RET = [bool; 1];
let _5: *const bool;
let _6: [u8; 3];
let _7: f32;
let _8: char;
let _9: *mut [bool; 1];
let _10: char;
let _11: isize;
let _12: [u16; 4];
let _13: f64;
let _14: *mut Adt32;
let _15: *const f32;
let _16: Adt48;
let _17: *const isize;
let _18: *mut Adt32;
let _19: i32;
let _20: isize;
let _21: isize;
let _22: bool;
let _23: &'static &'static i8;
let _24: isize;
let _25: [char; 8];
let _26: f32;
let _27: bool;
let _28: usize;
let _29: char;
let _30: [u32; 3];
let _31: (i16, [u32; 3], f64, (u8, u16, i16, Adt22));
let _32: [bool; 5];
let _33: Adt22;
let _34: (u128, (bool,), (u8,), (i8, u128, i128, Adt27));
let _35: char;
let _36: &'static i8;
let _37: ();
let _38: ();
{
_4 = 5_usize & 8306002165595053660_usize;
_5 = core::ptr::addr_of!(_1);
_4 = 1_usize + 1_usize;
_2 = (-1474974415_i32) as isize;
_2 = 93_isize + 9223372036854775807_isize;
RET = _3;
(*_5) = true;
_5 = core::ptr::addr_of!((*_5));
_3 = RET;
(*_5) = false;
(*_5) = false;
(*_5) = false ^ false;
_3 = [(*_5)];
_2 = 71_isize & (-9223372036854775808_isize);
_3 = [(*_5)];
RET = [(*_5)];
_4 = 3_usize;
_6 = [174_u8,100_u8,210_u8];
_4 = 165_u8 as usize;
RET = _3;
_6 = [130_u8,74_u8,126_u8];
_4 = 197943768986703442_usize * 4_usize;
RET = _3;
(*_5) = false;
(*_5) = false;
_6 = [69_u8,122_u8,1_u8];
_2 = (-9223372036854775808_isize);
_1 = !false;
_5 = core::ptr::addr_of!((*_5));
_5 = core::ptr::addr_of!((*_5));
match _2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463454151235394913435648 => bb6,
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
RET = _3;
_3 = [(*_5)];
_8 = '\u{22c9b}';
_9 = core::ptr::addr_of_mut!(RET);
_7 = _4 as f32;
_9 = core::ptr::addr_of_mut!(_3);
_2 = 1669318618_i32 as isize;
_6 = [175_u8,87_u8,117_u8];
_4 = 5_usize;
(*_5) = _8 == _8;
_3 = RET;
_10 = _8;
_5 = core::ptr::addr_of!(_1);
_3 = [(*_5)];
_1 = _4 <= _4;
_9 = core::ptr::addr_of_mut!((*_9));
(*_5) = false;
(*_5) = !true;
_11 = _2 + _2;
_6 = [98_u8,5_u8,185_u8];
_8 = _10;
_4 = 12431463696036561159_u64 as usize;
Goto(bb7)
}
bb7 = {
_6 = [151_u8,121_u8,105_u8];
_6 = [222_u8,65_u8,166_u8];
_11 = 78811497971273919371976407990658488964_u128 as isize;
_7 = (-1997110186_i32) as f32;
(*_5) = !true;
_3 = RET;
_12 = [44064_u16,45972_u16,33997_u16,3983_u16];
_15 = core::ptr::addr_of!(_7);
_11 = (-1386610678_i32) as isize;
_3 = [(*_5)];
(*_15) = 94_u8 as f32;
(*_15) = 4034933283_u32 as f32;
RET = [_1];
_5 = core::ptr::addr_of!((*_5));
(*_15) = (-618642533_i32) as f32;
(*_5) = _2 < _11;
_12 = [47506_u16,42066_u16,64300_u16,17850_u16];
_9 = core::ptr::addr_of_mut!((*_9));
_8 = _10;
(*_9) = [(*_5)];
RET = [(*_5)];
_2 = _11;
_5 = core::ptr::addr_of!((*_5));
_5 = core::ptr::addr_of!((*_5));
_3 = [_1];
(*_5) = !false;
Call((*_9) = fn6(_8, _8, _2, _11, Move(_9), _10), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_13 = 721145156_i32 as f64;
_4 = 6440_u16 as usize;
_5 = core::ptr::addr_of!((*_5));
_7 = 2276068264471305551_i64 as f32;
_17 = core::ptr::addr_of!(_2);
Goto(bb9)
}
bb9 = {
RET = _3;
_17 = core::ptr::addr_of!((*_17));
_6 = [106_u8,158_u8,226_u8];
_1 = false;
(*_17) = _4 as isize;
_1 = !false;
_20 = !_11;
_9 = core::ptr::addr_of_mut!(_3);
_11 = (*_17) ^ _2;
_2 = 2673648091763484753_u64 as isize;
_7 = (-90_i8) as f32;
_19 = (-756583599_i32);
_5 = core::ptr::addr_of!((*_5));
_20 = 14171129442282363109_u64 as isize;
Call(_19 = core::intrinsics::transmute(_8), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
(*_17) = _11;
_10 = _8;
_17 = core::ptr::addr_of!(_11);
_2 = 6294290974128725516_i64 as isize;
(*_15) = 99_u8 as f32;
_6 = [39_u8,127_u8,217_u8];
_20 = !(*_17);
(*_9) = [(*_5)];
(*_15) = (-137309267289172333032263372509405921990_i128) as f32;
_7 = 49_u8 as f32;
_24 = _11;
_22 = (*_5);
(*_17) = _20;
_22 = !_1;
_15 = core::ptr::addr_of!((*_15));
(*_15) = 119907251922458018488689082782669362178_i128 as f32;
(*_17) = 65403_u16 as isize;
(*_17) = _24;
_1 = !_22;
_2 = -_11;
Call((*_17) = core::intrinsics::transmute(_12), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
(*_15) = 1583203728_u32 as f32;
_25 = [_10,_8,_10,_10,_8,_8,_10,_10];
_20 = !_11;
_26 = (*_15);
_22 = !(*_5);
_6 = [139_u8,100_u8,242_u8];
_11 = _24;
(*_5) = _11 == _20;
_27 = !_1;
(*_9) = RET;
(*_5) = _27;
_22 = (*_15) >= _26;
_21 = _20 << _2;
(*_17) = _20;
_26 = (-19396_i16) as f32;
_7 = _26 - _26;
_13 = _4 as f64;
_3 = [(*_5)];
_25 = [_8,_10,_8,_10,_8,_8,_10,_10];
_17 = core::ptr::addr_of!(_2);
_13 = _19 as f64;
Goto(bb12)
}
bb12 = {
_29 = _10;
(*_15) = _26;
(*_5) = _22 | _27;
_13 = (-12156_i16) as f64;
_12 = [10715_u16,6631_u16,16711_u16,29479_u16];
_9 = core::ptr::addr_of_mut!(RET);
_28 = !_4;
_12 = [22162_u16,9637_u16,213_u16,10642_u16];
Goto(bb13)
}
bb13 = {
_12 = [33259_u16,2588_u16,28374_u16,35324_u16];
_25 = [_10,_8,_29,_10,_8,_29,_8,_10];
_26 = _7 + (*_15);
(*_5) = _22;
RET = _3;
(*_15) = _26;
_31.3.2 = (-13540_i16) & 9381_i16;
_31.0 = _29 as i16;
_31.1 = [2637316099_u32,2425886109_u32,440134485_u32];
_31.3.3 = Adt22::Variant0 { fld0: (*_5) };
_12 = [58897_u16,11072_u16,6380_u16,19246_u16];
_22 = Field::<bool>(Variant(_31.3.3, 0), 0);
_25 = [_29,_8,_8,_8,_8,_10,_29,_10];
_28 = _4;
_22 = _1 ^ (*_5);
_30 = _31.1;
_31.1 = _30;
_31.2 = _13;
_31.3.2 = _31.0;
_19 = _8 as i32;
(*_5) = Field::<bool>(Variant(_31.3.3, 0), 0);
_19 = (-1910962512_i32);
_30 = [1492694125_u32,2659629594_u32,811858146_u32];
match _19 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
340282366920938463463374607429857248944 => bb20,
_ => bb19
}
}
bb14 = {
_29 = _10;
(*_15) = _26;
(*_5) = _22 | _27;
_13 = (-12156_i16) as f64;
_12 = [10715_u16,6631_u16,16711_u16,29479_u16];
_9 = core::ptr::addr_of_mut!(RET);
_28 = !_4;
_12 = [22162_u16,9637_u16,213_u16,10642_u16];
Goto(bb13)
}
bb15 = {
(*_15) = 1583203728_u32 as f32;
_25 = [_10,_8,_10,_10,_8,_8,_10,_10];
_20 = !_11;
_26 = (*_15);
_22 = !(*_5);
_6 = [139_u8,100_u8,242_u8];
_11 = _24;
(*_5) = _11 == _20;
_27 = !_1;
(*_9) = RET;
(*_5) = _27;
_22 = (*_15) >= _26;
_21 = _20 << _2;
(*_17) = _20;
_26 = (-19396_i16) as f32;
_7 = _26 - _26;
_13 = _4 as f64;
_3 = [(*_5)];
_25 = [_8,_10,_8,_10,_8,_8,_10,_10];
_17 = core::ptr::addr_of!(_2);
_13 = _19 as f64;
Goto(bb12)
}
bb16 = {
Return()
}
bb17 = {
RET = _3;
_17 = core::ptr::addr_of!((*_17));
_6 = [106_u8,158_u8,226_u8];
_1 = false;
(*_17) = _4 as isize;
_1 = !false;
_20 = !_11;
_9 = core::ptr::addr_of_mut!(_3);
_11 = (*_17) ^ _2;
_2 = 2673648091763484753_u64 as isize;
_7 = (-90_i8) as f32;
_19 = (-756583599_i32);
_5 = core::ptr::addr_of!((*_5));
_20 = 14171129442282363109_u64 as isize;
Call(_19 = core::intrinsics::transmute(_8), ReturnTo(bb10), UnwindUnreachable())
}
bb18 = {
_13 = 721145156_i32 as f64;
_4 = 6440_u16 as usize;
_5 = core::ptr::addr_of!((*_5));
_7 = 2276068264471305551_i64 as f32;
_17 = core::ptr::addr_of!(_2);
Goto(bb9)
}
bb19 = {
RET = _3;
_3 = [(*_5)];
_8 = '\u{22c9b}';
_9 = core::ptr::addr_of_mut!(RET);
_7 = _4 as f32;
_9 = core::ptr::addr_of_mut!(_3);
_2 = 1669318618_i32 as isize;
_6 = [175_u8,87_u8,117_u8];
_4 = 5_usize;
(*_5) = _8 == _8;
_3 = RET;
_10 = _8;
_5 = core::ptr::addr_of!(_1);
_3 = [(*_5)];
_1 = _4 <= _4;
_9 = core::ptr::addr_of_mut!((*_9));
(*_5) = false;
(*_5) = !true;
_11 = _2 + _2;
_6 = [98_u8,5_u8,185_u8];
_8 = _10;
_4 = 12431463696036561159_u64 as usize;
Goto(bb7)
}
bb20 = {
_33 = Move(_31.3.3);
_20 = (*_17) - _2;
_34.0 = !148837619562556271400334232970416781640_u128;
_31.3.0 = !93_u8;
_31.3.1 = !18939_u16;
_34.2.0 = _31.3.0;
_34.3.0 = !(-38_i8);
_6 = [_31.3.0,_34.2.0,_31.3.0];
_19 = (-6386686431214536067_i64) as i32;
_29 = _10;
_34.2 = (_31.3.0,);
(*_17) = _24;
_30 = [3592485517_u32,2447558422_u32,2518425698_u32];
_5 = core::ptr::addr_of!(_1);
_6 = [_31.3.0,_34.2.0,_34.2.0];
_31.0 = _31.3.1 as i16;
_1 = !_22;
_19 = (-1451980307_i32) | (-1534487460_i32);
_35 = _29;
RET = [_27];
(*_17) = _21;
_9 = core::ptr::addr_of_mut!((*_9));
Goto(bb21)
}
bb21 = {
Call(_37 = dump_var(5_usize, 6_usize, Move(_6), 19_usize, Move(_19), 27_usize, Move(_27), 3_usize, Move(_3)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_37 = dump_var(5_usize, 35_usize, Move(_35), 24_usize, Move(_24), 20_usize, Move(_20), 28_usize, Move(_28)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_37 = dump_var(5_usize, 2_usize, Move(_2), 11_usize, Move(_11), 38_usize, _38, 38_usize, _38), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: char,mut _2: char,mut _3: isize,mut _4: isize,mut _5: *mut [bool; 1],mut _6: char) -> [bool; 1] {
mir! {
type RET = [bool; 1];
let _7: (char, &'static isize);
let _8: u16;
let _9: ((i16, [u32; 3], f64, (u8, u16, i16, Adt22)), [u16; 2], &'static *mut (u8, u16, i16, Adt22));
let _10: bool;
let _11: Adt47;
let _12: i128;
let _13: isize;
let _14: (usize, i128, char, &'static isize);
let _15: [usize; 4];
let _16: ((i16, [u32; 3], f64, (u8, u16, i16, Adt22)), [u16; 2], &'static *mut (u8, u16, i16, Adt22));
let _17: bool;
let _18: usize;
let _19: *const [char; 4];
let _20: (u8, u16, i16, Adt22);
let _21: [u16; 2];
let _22: i16;
let _23: u64;
let _24: u64;
let _25: *const &'static &'static i8;
let _26: i8;
let _27: Adt22;
let _28: &'static &'static f64;
let _29: ();
let _30: ();
{
_5 = core::ptr::addr_of_mut!(RET);
RET = [true];
RET = [true];
_1 = _6;
_2 = _6;
_5 = core::ptr::addr_of_mut!(RET);
RET = [false];
_5 = core::ptr::addr_of_mut!((*_5));
RET = [false];
RET = [true];
_4 = 1936726970_u32 as isize;
(*_5) = [false];
_7.0 = _2;
_2 = _6;
_7.1 = &_4;
_7.1 = &_3;
RET = [false];
_2 = _7.0;
_7.0 = _6;
RET = [false];
Call(_4 = fn7(RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7.0 = _6;
_7.0 = _2;
_2 = _6;
_8 = !6986_u16;
_4 = _3;
_8 = !37451_u16;
_8 = 59361358461458541782212594291292090618_i128 as u16;
_8 = !2889_u16;
_7.0 = _6;
_7.1 = &_4;
RET = [true];
_4 = (-1793913158726102581_i64) as isize;
_6 = _2;
_7.0 = _6;
(*_5) = [true];
_4 = _3 | _3;
RET = [true];
_2 = _1;
_7.1 = &_3;
_1 = _6;
_9.0.3.0 = 52_u8 & 77_u8;
_9.0.1 = [4024671228_u32,3069480498_u32,2711519826_u32];
_9.0.2 = (-26507_i16) as f64;
Call(_12 = core::intrinsics::bswap((-153356925121405291229712944333368603856_i128)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7.1 = &_3;
_5 = core::ptr::addr_of_mut!((*_5));
_12 = 1427327649_u32 as i128;
Call(_4 = core::intrinsics::transmute(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9.0.3.2 = 3964262598_u32 as i16;
_7.1 = &_13;
_1 = _2;
_3 = _4 - _4;
_7.0 = _6;
_9.1 = [_8,_8];
(*_5) = [false];
_14.2 = _1;
_9.1 = [_8,_8];
_11 = Adt47::Variant2 { fld0: false,fld1: 958326979_i32 };
_14.3 = &_13;
_7.0 = _14.2;
_10 = !false;
RET = [_10];
_13 = _3 & _3;
_16.1 = _9.1;
_16.0.3.2 = _9.0.3.2 * _9.0.3.2;
_9.1 = _16.1;
Goto(bb4)
}
bb4 = {
_14.0 = 16381144480002441145_usize;
_4 = -_13;
RET = [_10];
_13 = !_4;
Goto(bb5)
}
bb5 = {
_16.0.3.0 = _9.0.3.0;
place!(Field::<bool>(Variant(_11, 2), 0)) = _10 ^ _10;
_16.0.1 = _9.0.1;
_2 = _14.2;
_16.0.3.1 = _8;
(*_5) = [Field::<bool>(Variant(_11, 2), 0)];
place!(Field::<i32>(Variant(_11, 2), 1)) = !2094528567_i32;
_13 = -_3;
_5 = core::ptr::addr_of_mut!((*_5));
_10 = Field::<bool>(Variant(_11, 2), 0);
_14.1 = _12 | _12;
_8 = _16.0.3.1 ^ _16.0.3.1;
_2 = _7.0;
match _14.0 {
0 => bb3,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb8,
16381144480002441145 => bb10,
_ => bb9
}
}
bb6 = {
_14.0 = 16381144480002441145_usize;
_4 = -_13;
RET = [_10];
_13 = !_4;
Goto(bb5)
}
bb7 = {
_9.0.3.2 = 3964262598_u32 as i16;
_7.1 = &_13;
_1 = _2;
_3 = _4 - _4;
_7.0 = _6;
_9.1 = [_8,_8];
(*_5) = [false];
_14.2 = _1;
_9.1 = [_8,_8];
_11 = Adt47::Variant2 { fld0: false,fld1: 958326979_i32 };
_14.3 = &_13;
_7.0 = _14.2;
_10 = !false;
RET = [_10];
_13 = _3 & _3;
_16.1 = _9.1;
_16.0.3.2 = _9.0.3.2 * _9.0.3.2;
_9.1 = _16.1;
Goto(bb4)
}
bb8 = {
_7.1 = &_3;
_5 = core::ptr::addr_of_mut!((*_5));
_12 = 1427327649_u32 as i128;
Call(_4 = core::intrinsics::transmute(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_7.0 = _6;
_7.0 = _2;
_2 = _6;
_8 = !6986_u16;
_4 = _3;
_8 = !37451_u16;
_8 = 59361358461458541782212594291292090618_i128 as u16;
_8 = !2889_u16;
_7.0 = _6;
_7.1 = &_4;
RET = [true];
_4 = (-1793913158726102581_i64) as isize;
_6 = _2;
_7.0 = _6;
(*_5) = [true];
_4 = _3 | _3;
RET = [true];
_2 = _1;
_7.1 = &_3;
_1 = _6;
_9.0.3.0 = 52_u8 & 77_u8;
_9.0.1 = [4024671228_u32,3069480498_u32,2711519826_u32];
_9.0.2 = (-26507_i16) as f64;
Call(_12 = core::intrinsics::bswap((-153356925121405291229712944333368603856_i128)), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_20.1 = !_8;
(*_5) = [_10];
_16.0.3.0 = _9.0.3.2 as u8;
_2 = _14.2;
_14.3 = &_4;
_18 = _14.0;
_20.2 = _16.0.3.2 >> _18;
match _18 {
0 => bb3,
16381144480002441145 => bb12,
_ => bb11
}
}
bb11 = {
_16.0.3.0 = _9.0.3.0;
place!(Field::<bool>(Variant(_11, 2), 0)) = _10 ^ _10;
_16.0.1 = _9.0.1;
_2 = _14.2;
_16.0.3.1 = _8;
(*_5) = [Field::<bool>(Variant(_11, 2), 0)];
place!(Field::<i32>(Variant(_11, 2), 1)) = !2094528567_i32;
_13 = -_3;
_5 = core::ptr::addr_of_mut!((*_5));
_10 = Field::<bool>(Variant(_11, 2), 0);
_14.1 = _12 | _12;
_8 = _16.0.3.1 ^ _16.0.3.1;
_2 = _7.0;
match _14.0 {
0 => bb3,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb8,
16381144480002441145 => bb10,
_ => bb9
}
}
bb12 = {
_16.0.0 = (-7018878434150415897_i64) as i16;
_22 = -_16.0.3.2;
_24 = 5884251234925419742_u64 * 671102693212137992_u64;
_20.0 = _9.0.3.0 ^ _16.0.3.0;
_16.0.2 = _9.0.2;
_21 = [_8,_8];
_20.0 = !_16.0.3.0;
_5 = core::ptr::addr_of_mut!((*_5));
_16.0.1 = [1549289005_u32,1469621795_u32,1983591513_u32];
_9.0.3.1 = _20.1;
_14.1 = _12 | _12;
place!(Field::<bool>(Variant(_11, 2), 0)) = !_10;
_10 = !Field::<bool>(Variant(_11, 2), 0);
_9.0.3.2 = _9.0.3.0 as i16;
_15 = [_14.0,_18,_14.0,_18];
_17 = _10;
_7.1 = Move(_14.3);
_9.1 = [_9.0.3.1,_20.1];
_17 = Field::<bool>(Variant(_11, 2), 0);
_9.0.1 = _16.0.1;
_9.0.0 = !_16.0.0;
_13 = !_3;
_16.0.3.2 = Field::<i32>(Variant(_11, 2), 1) as i16;
_14.3 = &_13;
_16.0.1 = [3917451614_u32,4218779836_u32,3137546885_u32];
_6 = _14.2;
match _14.0 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
16381144480002441145 => bb20,
_ => bb19
}
}
bb13 = {
_16.0.3.0 = _9.0.3.0;
place!(Field::<bool>(Variant(_11, 2), 0)) = _10 ^ _10;
_16.0.1 = _9.0.1;
_2 = _14.2;
_16.0.3.1 = _8;
(*_5) = [Field::<bool>(Variant(_11, 2), 0)];
place!(Field::<i32>(Variant(_11, 2), 1)) = !2094528567_i32;
_13 = -_3;
_5 = core::ptr::addr_of_mut!((*_5));
_10 = Field::<bool>(Variant(_11, 2), 0);
_14.1 = _12 | _12;
_8 = _16.0.3.1 ^ _16.0.3.1;
_2 = _7.0;
match _14.0 {
0 => bb3,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb8,
16381144480002441145 => bb10,
_ => bb9
}
}
bb14 = {
_20.1 = !_8;
(*_5) = [_10];
_16.0.3.0 = _9.0.3.2 as u8;
_2 = _14.2;
_14.3 = &_4;
_18 = _14.0;
_20.2 = _16.0.3.2 >> _18;
match _18 {
0 => bb3,
16381144480002441145 => bb12,
_ => bb11
}
}
bb15 = {
_7.0 = _6;
_7.0 = _2;
_2 = _6;
_8 = !6986_u16;
_4 = _3;
_8 = !37451_u16;
_8 = 59361358461458541782212594291292090618_i128 as u16;
_8 = !2889_u16;
_7.0 = _6;
_7.1 = &_4;
RET = [true];
_4 = (-1793913158726102581_i64) as isize;
_6 = _2;
_7.0 = _6;
(*_5) = [true];
_4 = _3 | _3;
RET = [true];
_2 = _1;
_7.1 = &_3;
_1 = _6;
_9.0.3.0 = 52_u8 & 77_u8;
_9.0.1 = [4024671228_u32,3069480498_u32,2711519826_u32];
_9.0.2 = (-26507_i16) as f64;
Call(_12 = core::intrinsics::bswap((-153356925121405291229712944333368603856_i128)), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_7.1 = &_3;
_5 = core::ptr::addr_of_mut!((*_5));
_12 = 1427327649_u32 as i128;
Call(_4 = core::intrinsics::transmute(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_9.0.3.2 = 3964262598_u32 as i16;
_7.1 = &_13;
_1 = _2;
_3 = _4 - _4;
_7.0 = _6;
_9.1 = [_8,_8];
(*_5) = [false];
_14.2 = _1;
_9.1 = [_8,_8];
_11 = Adt47::Variant2 { fld0: false,fld1: 958326979_i32 };
_14.3 = &_13;
_7.0 = _14.2;
_10 = !false;
RET = [_10];
_13 = _3 & _3;
_16.1 = _9.1;
_16.0.3.2 = _9.0.3.2 * _9.0.3.2;
_9.1 = _16.1;
Goto(bb4)
}
bb18 = {
_7.1 = &_3;
_5 = core::ptr::addr_of_mut!((*_5));
_12 = 1427327649_u32 as i128;
Call(_4 = core::intrinsics::transmute(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb19 = {
_9.0.3.2 = 3964262598_u32 as i16;
_7.1 = &_13;
_1 = _2;
_3 = _4 - _4;
_7.0 = _6;
_9.1 = [_8,_8];
(*_5) = [false];
_14.2 = _1;
_9.1 = [_8,_8];
_11 = Adt47::Variant2 { fld0: false,fld1: 958326979_i32 };
_14.3 = &_13;
_7.0 = _14.2;
_10 = !false;
RET = [_10];
_13 = _3 & _3;
_16.1 = _9.1;
_16.0.3.2 = _9.0.3.2 * _9.0.3.2;
_9.1 = _16.1;
Goto(bb4)
}
bb20 = {
_16.0.3.2 = _9.0.0 | _16.0.0;
_7 = (_14.2, Move(_14.3));
(*_5) = [_10];
SetDiscriminant(_11, 3);
_16.0.3.2 = 42_i8 as i16;
_3 = !_4;
_14.1 = _12;
_6 = _14.2;
_5 = core::ptr::addr_of_mut!((*_5));
_16.0.0 = _9.0.3.2;
Goto(bb21)
}
bb21 = {
Call(_29 = dump_var(6_usize, 17_usize, Move(_17), 8_usize, Move(_8), 4_usize, Move(_4), 24_usize, Move(_24)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_29 = dump_var(6_usize, 18_usize, Move(_18), 3_usize, Move(_3), 13_usize, Move(_13), 30_usize, _30), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: [bool; 1]) -> isize {
mir! {
type RET = isize;
let _2: char;
let _3: *mut [bool; 1];
let _4: u32;
let _5: (char, &'static isize);
let _6: Adt74;
let _7: *const &'static &'static i8;
let _8: u128;
let _9: *const &'static &'static i8;
let _10: u32;
let _11: bool;
let _12: *const bool;
let _13: u64;
let _14: &'static &'static i8;
let _15: f64;
let _16: (i128, (u8, u16, i16, Adt22), u64, u8);
let _17: f64;
let _18: Adt80;
let _19: *const &'static &'static i8;
let _20: bool;
let _21: i32;
let _22: f64;
let _23: (Adt54, (u8,));
let _24: char;
let _25: f64;
let _26: char;
let _27: char;
let _28: (((i16, [u32; 3], f64, (u8, u16, i16, Adt22)), [u16; 2], &'static *mut (u8, u16, i16, Adt22)), (bool,), &'static isize);
let _29: f32;
let _30: &'static &'static i8;
let _31: &'static u64;
let _32: *const &'static &'static i8;
let _33: ();
let _34: ();
{
RET = -(-9223372036854775808_isize);
_1 = [true];
RET = '\u{644a2}' as isize;
RET = 26050186657058117817012472288806111154_i128 as isize;
RET = (-74_isize);
_2 = '\u{1a081}';
RET = !(-9223372036854775808_isize);
RET = !9223372036854775807_isize;
RET = _2 as isize;
_3 = core::ptr::addr_of_mut!(_1);
(*_3) = [true];
RET = 17_isize - 9223372036854775807_isize;
_2 = '\u{6736a}';
RET = 93573989549980015789557264956099901212_i128 as isize;
_3 = core::ptr::addr_of_mut!((*_3));
RET = 9223372036854775807_isize - 9223372036854775807_isize;
(*_3) = [true];
RET = 9223372036854775807_isize ^ (-9223372036854775808_isize);
(*_3) = [true];
_3 = core::ptr::addr_of_mut!((*_3));
_3 = core::ptr::addr_of_mut!((*_3));
Call((*_3) = fn8(RET, RET, Move(_3), RET, _2, RET, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = !(-89_isize);
_4 = 3663797877_u32 & 1187373357_u32;
_3 = core::ptr::addr_of_mut!(_1);
_2 = '\u{667f1}';
_4 = !875998639_u32;
Goto(bb2)
}
bb2 = {
(*_3) = [false];
_1 = [true];
RET = 9223372036854775807_isize >> _4;
_5.0 = _2;
_2 = _5.0;
(*_3) = [false];
(*_3) = [true];
_1 = [true];
_1 = [true];
_5.1 = &RET;
_8 = 21696_i16 as u128;
_5.0 = _2;
_5.1 = &RET;
_4 = 12747571085326247341_usize as u32;
_1 = [false];
_5.0 = _2;
RET = !32_isize;
_1 = [false];
_3 = core::ptr::addr_of_mut!((*_3));
Call(_10 = core::intrinsics::transmute(_2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = !_10;
_1 = [false];
_10 = _4 + _4;
_5.0 = _2;
_4 = _10 + _10;
_1 = [true];
(*_3) = [false];
_5.1 = &RET;
_1 = [false];
(*_3) = [false];
_11 = !true;
_5.0 = _2;
_2 = _5.0;
_1 = [_11];
RET = (-9223372036854775808_isize);
RET = 9223372036854775807_isize ^ 9223372036854775807_isize;
_13 = 46517_u16 as u64;
Goto(bb4)
}
bb4 = {
_4 = _10 | _10;
_5.0 = _2;
_16.1.1 = 22468_u16 - 43829_u16;
_10 = _8 as u32;
_17 = _4 as f64;
_16.3 = RET as u8;
_9 = core::ptr::addr_of!(_14);
RET = (-9223372036854775808_isize);
_4 = _10;
_2 = _5.0;
_18.fld4 = [_16.1.1,_16.1.1,_16.1.1,_16.1.1,_16.1.1];
_15 = _17;
_16.2 = _16.1.1 as u64;
_12 = core::ptr::addr_of!(_20);
_7 = core::ptr::addr_of!((*_9));
_20 = !_11;
_13 = !_16.2;
_16.1.2 = 23265_i16 & 4519_i16;
_18.fld2 = _10 as i64;
(*_3) = [(*_12)];
_18.fld0 = 443508962_i32;
_17 = _15 * _15;
_18.fld4 = [_16.1.1,_16.1.1,_16.1.1,_16.1.1,_16.1.1];
_16.1.0 = _18.fld0 as u8;
match _18.fld0 {
0 => bb1,
1 => bb5,
443508962 => bb7,
_ => bb6
}
}
bb5 = {
RET = !(-89_isize);
_4 = 3663797877_u32 & 1187373357_u32;
_3 = core::ptr::addr_of_mut!(_1);
_2 = '\u{667f1}';
_4 = !875998639_u32;
Goto(bb2)
}
bb6 = {
(*_3) = [false];
_1 = [true];
RET = 9223372036854775807_isize >> _4;
_5.0 = _2;
_2 = _5.0;
(*_3) = [false];
(*_3) = [true];
_1 = [true];
_1 = [true];
_5.1 = &RET;
_8 = 21696_i16 as u128;
_5.0 = _2;
_5.1 = &RET;
_4 = 12747571085326247341_usize as u32;
_1 = [false];
_5.0 = _2;
RET = !32_isize;
_1 = [false];
_3 = core::ptr::addr_of_mut!((*_3));
Call(_10 = core::intrinsics::transmute(_2), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_19 = core::ptr::addr_of!(_14);
_16.1.1 = _18.fld2 as u16;
_19 = core::ptr::addr_of!((*_9));
_5.1 = &RET;
_15 = -_17;
_3 = core::ptr::addr_of_mut!((*_3));
_16.2 = _13;
_16.0 = 42393690970828257367703875179200937768_i128 + 154911612228940603590388451508928230030_i128;
_20 = _16.0 == _16.0;
_18.fld2 = (-520945750324100635_i64) ^ (-2631243655372550145_i64);
_20 = _11;
(*_12) = !_11;
_8 = 127001545910963691735839272874705927043_u128;
(*_3) = [(*_12)];
_5.1 = &RET;
(*_3) = [(*_12)];
_22 = -_17;
_2 = _5.0;
_18.fld1.1 = (_16.3,);
_18.fld3 = [_18.fld2,_18.fld2,_18.fld2];
_18.fld2 = -(-3170483260123707226_i64);
Goto(bb8)
}
bb8 = {
_16.3 = _16.1.0 + _16.1.0;
_23.1 = (_18.fld1.1.0,);
_24 = _5.0;
_2 = _5.0;
_24 = _2;
_18.fld1.1.0 = !_16.3;
_7 = core::ptr::addr_of!((*_19));
(*_12) = !_11;
_16.1.0 = _23.1.0 - _16.3;
_18.fld0 = (-878299844_i32);
match _8 {
0 => bb6,
127001545910963691735839272874705927043 => bb10,
_ => bb9
}
}
bb9 = {
RET = !(-89_isize);
_4 = 3663797877_u32 & 1187373357_u32;
_3 = core::ptr::addr_of_mut!(_1);
_2 = '\u{667f1}';
_4 = !875998639_u32;
Goto(bb2)
}
bb10 = {
_18.fld1.1 = (_16.3,);
_22 = _8 as f64;
_4 = _10;
_21 = _18.fld0;
_5.0 = _2;
_8 = !274333474723107568471809644680641404872_u128;
_16.0 = !155067204937904216378980410631483687934_i128;
_23.1.0 = _10 as u8;
_16.2 = !_13;
_4 = !_10;
(*_12) = _16.1.0 == _16.3;
(*_3) = [(*_12)];
_18.fld1.1.0 = !_16.3;
match RET {
0 => bb8,
1 => bb9,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
340282366920938463454151235394913435648 => bb17,
_ => bb16
}
}
bb11 = {
RET = !(-89_isize);
_4 = 3663797877_u32 & 1187373357_u32;
_3 = core::ptr::addr_of_mut!(_1);
_2 = '\u{667f1}';
_4 = !875998639_u32;
Goto(bb2)
}
bb12 = {
_16.3 = _16.1.0 + _16.1.0;
_23.1 = (_18.fld1.1.0,);
_24 = _5.0;
_2 = _5.0;
_24 = _2;
_18.fld1.1.0 = !_16.3;
_7 = core::ptr::addr_of!((*_19));
(*_12) = !_11;
_16.1.0 = _23.1.0 - _16.3;
_18.fld0 = (-878299844_i32);
match _8 {
0 => bb6,
127001545910963691735839272874705927043 => bb10,
_ => bb9
}
}
bb13 = {
_19 = core::ptr::addr_of!(_14);
_16.1.1 = _18.fld2 as u16;
_19 = core::ptr::addr_of!((*_9));
_5.1 = &RET;
_15 = -_17;
_3 = core::ptr::addr_of_mut!((*_3));
_16.2 = _13;
_16.0 = 42393690970828257367703875179200937768_i128 + 154911612228940603590388451508928230030_i128;
_20 = _16.0 == _16.0;
_18.fld2 = (-520945750324100635_i64) ^ (-2631243655372550145_i64);
_20 = _11;
(*_12) = !_11;
_8 = 127001545910963691735839272874705927043_u128;
(*_3) = [(*_12)];
_5.1 = &RET;
(*_3) = [(*_12)];
_22 = -_17;
_2 = _5.0;
_18.fld1.1 = (_16.3,);
_18.fld3 = [_18.fld2,_18.fld2,_18.fld2];
_18.fld2 = -(-3170483260123707226_i64);
Goto(bb8)
}
bb14 = {
(*_3) = [false];
_1 = [true];
RET = 9223372036854775807_isize >> _4;
_5.0 = _2;
_2 = _5.0;
(*_3) = [false];
(*_3) = [true];
_1 = [true];
_1 = [true];
_5.1 = &RET;
_8 = 21696_i16 as u128;
_5.0 = _2;
_5.1 = &RET;
_4 = 12747571085326247341_usize as u32;
_1 = [false];
_5.0 = _2;
RET = !32_isize;
_1 = [false];
_3 = core::ptr::addr_of_mut!((*_3));
Call(_10 = core::intrinsics::transmute(_2), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
RET = !(-89_isize);
_4 = 3663797877_u32 & 1187373357_u32;
_3 = core::ptr::addr_of_mut!(_1);
_2 = '\u{667f1}';
_4 = !875998639_u32;
Goto(bb2)
}
bb16 = {
(*_3) = [false];
_1 = [true];
RET = 9223372036854775807_isize >> _4;
_5.0 = _2;
_2 = _5.0;
(*_3) = [false];
(*_3) = [true];
_1 = [true];
_1 = [true];
_5.1 = &RET;
_8 = 21696_i16 as u128;
_5.0 = _2;
_5.1 = &RET;
_4 = 12747571085326247341_usize as u32;
_1 = [false];
_5.0 = _2;
RET = !32_isize;
_1 = [false];
_3 = core::ptr::addr_of_mut!((*_3));
Call(_10 = core::intrinsics::transmute(_2), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_1 = [(*_12)];
(*_12) = !_11;
_5.0 = _24;
_29 = _10 as f32;
_18.fld1.1 = (_16.3,);
_29 = _16.0 as f32;
_18.fld1.1.0 = _16.1.0 ^ _16.1.0;
_8 = 327757920480410056889496725536899854879_u128;
_22 = -_15;
_28.1 = ((*_12),);
RET = (-38_isize);
_27 = _5.0;
_28.0.0.0 = -_16.1.2;
RET = 9223372036854775807_isize - 9223372036854775807_isize;
Goto(bb18)
}
bb18 = {
Call(_33 = dump_var(7_usize, 1_usize, Move(_1), 11_usize, Move(_11), 2_usize, Move(_2), 24_usize, Move(_24)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_33 = dump_var(7_usize, 13_usize, Move(_13), 34_usize, _34, 34_usize, _34, 34_usize, _34), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: isize,mut _2: isize,mut _3: *mut [bool; 1],mut _4: isize,mut _5: char,mut _6: isize,mut _7: char,mut _8: char) -> [bool; 1] {
mir! {
type RET = [bool; 1];
let _9: *const Adt27;
let _10: &'static (i8, u128, i128, Adt27);
let _11: &'static Adt32;
let _12: (((i16, [u32; 3], f64, (u8, u16, i16, Adt22)), [u16; 2], &'static *mut (u8, u16, i16, Adt22)), (bool,), &'static isize);
let _13: *const Adt27;
let _14: [bool; 2];
let _15: &'static [usize; 4];
let _16: isize;
let _17: isize;
let _18: f64;
let _19: Adt80;
let _20: *const f32;
let _21: isize;
let _22: i32;
let _23: u128;
let _24: i8;
let _25: i8;
let _26: ((i16, [u32; 3], f64, (u8, u16, i16, Adt22)), [u16; 2], &'static *mut (u8, u16, i16, Adt22));
let _27: bool;
let _28: bool;
let _29: isize;
let _30: u64;
let _31: i8;
let _32: &'static i8;
let _33: *const Adt27;
let _34: f32;
let _35: (u128, (bool,), (u8,), (i8, u128, i128, Adt27));
let _36: [i64; 3];
let _37: ();
let _38: ();
{
_7 = _8;
_6 = !_2;
RET = [false];
_2 = false as isize;
_6 = 16393707271432563729_u64 as isize;
_5 = _7;
RET = [false];
Goto(bb1)
}
bb1 = {
_4 = -_2;
_1 = _2;
_6 = _1;
_7 = _8;
_7 = _8;
_6 = !_4;
_4 = _2;
RET = [false];
_4 = 105_u8 as isize;
_12.0.0.2 = _1 as f64;
_12.0.0.0 = 1959385537_i32 as i16;
_12.0.0.3.2 = (-691239509_i32) as i16;
_12.0.0.3.3 = Adt22::Variant0 { fld0: true };
_5 = _8;
_12.0.0.1 = [3775954259_u32,1587607716_u32,3046638058_u32];
_12.2 = &_6;
_12.0.1 = [64434_u16,57886_u16];
_12.1 = (true,);
_1 = _2;
_3 = core::ptr::addr_of_mut!(RET);
Goto(bb2)
}
bb2 = {
_12.0.0.3.0 = (-1041423315_i32) as u8;
_12.0.0.3.2 = _12.0.0.0;
_4 = _1;
_1 = _12.0.0.3.0 as isize;
_12.0.0.3.1 = 51786_u16;
_7 = _8;
_8 = _5;
_12.0.0.0 = _12.0.0.3.2;
_12.0.0.3.1 = 11462_u16 << _6;
_14 = [_12.1.0,_12.1.0];
place!(Field::<bool>(Variant(_12.0.0.3.3, 0), 0)) = _12.1.0 & _12.1.0;
_12.1 = (Field::<bool>(Variant(_12.0.0.3.3, 0), 0),);
_18 = _12.0.0.3.1 as f64;
Call(_12.0.1 = fn9(Move(_12.2), Move(_3), (*_3), _12.0.0.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = -_1;
_12.0.0.2 = -_18;
_12.0.1 = [_12.0.0.3.1,_12.0.0.3.1];
_12.1 = (Field::<bool>(Variant(_12.0.0.3.3, 0), 0),);
_21 = _6 | _1;
_17 = 153726293628827248607491194590985020938_i128 as isize;
_12.0.0.3.0 = 88_u8;
_19.fld1.1 = (_12.0.0.3.0,);
_12.0.0.0 = -_12.0.0.3.2;
_19.fld4 = [_12.0.0.3.1,_12.0.0.3.1,_12.0.0.3.1,_12.0.0.3.1,_12.0.0.3.1];
RET = [_12.1.0];
_16 = !_21;
_8 = _5;
_12.2 = &_1;
_8 = _5;
place!(Field::<bool>(Variant(_12.0.0.3.3, 0), 0)) = _12.1.0;
_19.fld0 = !(-1794201691_i32);
_23 = 57740416299840810515712322286796752468_u128 & 101148531833115268462452302779003259292_u128;
_24 = !71_i8;
_19.fld2 = _24 as i64;
_23 = 61977334336159087590642461297373520295_u128 * 103266520180133942024266920373004120308_u128;
_12.0.1 = [_12.0.0.3.1,_12.0.0.3.1];
_22 = -_19.fld0;
_12.0.0.3.0 = !_19.fld1.1.0;
match _19.fld1.1.0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
88 => bb10,
_ => bb9
}
}
bb4 = {
_12.0.0.3.0 = (-1041423315_i32) as u8;
_12.0.0.3.2 = _12.0.0.0;
_4 = _1;
_1 = _12.0.0.3.0 as isize;
_12.0.0.3.1 = 51786_u16;
_7 = _8;
_8 = _5;
_12.0.0.0 = _12.0.0.3.2;
_12.0.0.3.1 = 11462_u16 << _6;
_14 = [_12.1.0,_12.1.0];
place!(Field::<bool>(Variant(_12.0.0.3.3, 0), 0)) = _12.1.0 & _12.1.0;
_12.1 = (Field::<bool>(Variant(_12.0.0.3.3, 0), 0),);
_18 = _12.0.0.3.1 as f64;
Call(_12.0.1 = fn9(Move(_12.2), Move(_3), (*_3), _12.0.0.1), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_4 = -_2;
_1 = _2;
_6 = _1;
_7 = _8;
_7 = _8;
_6 = !_4;
_4 = _2;
RET = [false];
_4 = 105_u8 as isize;
_12.0.0.2 = _1 as f64;
_12.0.0.0 = 1959385537_i32 as i16;
_12.0.0.3.2 = (-691239509_i32) as i16;
_12.0.0.3.3 = Adt22::Variant0 { fld0: true };
_5 = _8;
_12.0.0.1 = [3775954259_u32,1587607716_u32,3046638058_u32];
_12.2 = &_6;
_12.0.1 = [64434_u16,57886_u16];
_12.1 = (true,);
_1 = _2;
_3 = core::ptr::addr_of_mut!(RET);
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
_12.0.0.0 = _17 as i16;
_12.0.0.0 = -_12.0.0.3.2;
_19.fld4 = [_12.0.0.3.1,_12.0.0.3.1,_12.0.0.3.1,_12.0.0.3.1,_12.0.0.3.1];
_12.0.1 = [_12.0.0.3.1,_12.0.0.3.1];
_6 = _21;
_12.0.0.0 = _12.0.0.3.2;
_21 = Field::<bool>(Variant(_12.0.0.3.3, 0), 0) as isize;
_4 = _6;
Goto(bb11)
}
bb11 = {
_12.0.0.3.1 = !18969_u16;
_3 = core::ptr::addr_of_mut!(RET);
_25 = _24;
_26.0.2 = _12.0.0.2 + _18;
_23 = 97729655841027679014746794660492611033_u128;
_26.0.3.2 = _12.0.0.3.2;
_12.0.1 = [_12.0.0.3.1,_12.0.0.3.1];
_4 = _21;
_17 = 3364179389_u32 as isize;
_8 = _5;
SetDiscriminant(_12.0.0.3.3, 2);
_3 = core::ptr::addr_of_mut!(RET);
_26.0.3.2 = -_12.0.0.3.2;
_18 = _26.0.2;
place!(Field::<f32>(Variant(_12.0.0.3.3, 2), 0)) = 3055670066823048629_u64 as f32;
_19.fld0 = _16 as i32;
_16 = _4;
_12.0.0.1 = [3996517747_u32,850816794_u32,1321232496_u32];
_12.0.1 = [_12.0.0.3.1,_12.0.0.3.1];
_21 = _24 as isize;
_30 = 8063471799998456169_u64;
place!(Field::<i32>(Variant(_12.0.0.3.3, 2), 2)) = _19.fld0 << _6;
Call(_1 = core::intrinsics::bswap(_4), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_5 = _8;
(*_3) = [_12.1.0];
_12.0.0.3.2 = _12.0.0.0 << _6;
_31 = _25;
_28 = _12.1.0 | _12.1.0;
_12.0.0.0 = -_12.0.0.3.2;
_12.2 = &_2;
_31 = _24;
_1 = _16 - _17;
_26.0.3.0 = _12.0.0.3.0;
_26.0.3.1 = _12.1.0 as u16;
RET = [_28];
_12.0.0.3.2 = _12.0.0.0;
_23 = _19.fld1.1.0 as u128;
_19.fld0 = -Field::<i32>(Variant(_12.0.0.3.3, 2), 2);
place!(Field::<i64>(Variant(_12.0.0.3.3, 2), 1)) = _23 as i64;
_2 = _28 as isize;
RET = [_28];
_19.fld0 = _22 ^ _22;
(*_3) = [_12.1.0];
_12.1.0 = !_28;
match _30 {
0 => bb1,
1 => bb13,
8063471799998456169 => bb15,
_ => bb14
}
}
bb13 = {
_12.0.0.3.1 = !18969_u16;
_3 = core::ptr::addr_of_mut!(RET);
_25 = _24;
_26.0.2 = _12.0.0.2 + _18;
_23 = 97729655841027679014746794660492611033_u128;
_26.0.3.2 = _12.0.0.3.2;
_12.0.1 = [_12.0.0.3.1,_12.0.0.3.1];
_4 = _21;
_17 = 3364179389_u32 as isize;
_8 = _5;
SetDiscriminant(_12.0.0.3.3, 2);
_3 = core::ptr::addr_of_mut!(RET);
_26.0.3.2 = -_12.0.0.3.2;
_18 = _26.0.2;
place!(Field::<f32>(Variant(_12.0.0.3.3, 2), 0)) = 3055670066823048629_u64 as f32;
_19.fld0 = _16 as i32;
_16 = _4;
_12.0.0.1 = [3996517747_u32,850816794_u32,1321232496_u32];
_12.0.1 = [_12.0.0.3.1,_12.0.0.3.1];
_21 = _24 as isize;
_30 = 8063471799998456169_u64;
place!(Field::<i32>(Variant(_12.0.0.3.3, 2), 2)) = _19.fld0 << _6;
Call(_1 = core::intrinsics::bswap(_4), ReturnTo(bb12), UnwindUnreachable())
}
bb14 = {
_12.0.0.3.0 = (-1041423315_i32) as u8;
_12.0.0.3.2 = _12.0.0.0;
_4 = _1;
_1 = _12.0.0.3.0 as isize;
_12.0.0.3.1 = 51786_u16;
_7 = _8;
_8 = _5;
_12.0.0.0 = _12.0.0.3.2;
_12.0.0.3.1 = 11462_u16 << _6;
_14 = [_12.1.0,_12.1.0];
place!(Field::<bool>(Variant(_12.0.0.3.3, 0), 0)) = _12.1.0 & _12.1.0;
_12.1 = (Field::<bool>(Variant(_12.0.0.3.3, 0), 0),);
_18 = _12.0.0.3.1 as f64;
Call(_12.0.1 = fn9(Move(_12.2), Move(_3), (*_3), _12.0.0.1), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_24 = _12.0.0.3.0 as i8;
_28 = _12.1.0 ^ _12.1.0;
_12.1.0 = _28;
_35.3.1 = _26.0.3.1 as u128;
SetDiscriminant(_12.0.0.3.3, 0);
_35.3.2 = _12.0.0.0 as i128;
Goto(bb16)
}
bb16 = {
Call(_37 = dump_var(8_usize, 28_usize, Move(_28), 5_usize, Move(_5), 25_usize, Move(_25), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(8_usize, 6_usize, Move(_6), 24_usize, Move(_24), 17_usize, Move(_17), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(8_usize, 23_usize, Move(_23), 38_usize, _38, 38_usize, _38, 38_usize, _38), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: &'static isize,mut _2: *mut [bool; 1],mut _3: [bool; 1],mut _4: [u32; 3]) -> [u16; 2] {
mir! {
type RET = [u16; 2];
let _5: [bool; 5];
let _6: usize;
let _7: f64;
let _8: (usize,);
let _9: f64;
let _10: f64;
let _11: [u64; 1];
let _12: *mut [bool; 1];
let _13: (usize,);
let _14: *const i16;
let _15: [i64; 3];
let _16: [u16; 5];
let _17: bool;
let _18: f64;
let _19: &'static &'static f64;
let _20: f64;
let _21: *mut (u8, u16, i16, Adt22);
let _22: char;
let _23: u32;
let _24: *const [char; 4];
let _25: Adt80;
let _26: f32;
let _27: usize;
let _28: [u64; 1];
let _29: f32;
let _30: bool;
let _31: ();
let _32: ();
{
RET = [1912_u16,1040_u16];
RET = [38843_u16,4585_u16];
_5 = [false,false,false,false,true];
_3 = [false];
Goto(bb1)
}
bb1 = {
RET = [1071_u16,51449_u16];
RET = [54013_u16,54253_u16];
_2 = core::ptr::addr_of_mut!(_3);
(*_2) = [false];
_3 = [false];
(*_2) = [true];
_7 = (-17778_i16) as f64;
_6 = 1981107141664154251_usize - 4979822762457605080_usize;
_8.0 = _6;
_8.0 = !_6;
_10 = _7;
_9 = _7 * _7;
_6 = 2233810150_u32 as usize;
_11 = [16145949797330252086_u64];
_2 = core::ptr::addr_of_mut!(_3);
_10 = _7 - _9;
(*_2) = [false];
(*_2) = [false];
(*_2) = [true];
Call(_7 = fn10(), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = [true,true,false,true,false];
_9 = 85_u8 as f64;
_7 = -_10;
_3 = [false];
_11 = [12961740341840175136_u64];
_12 = Move(_2);
_4 = [987717056_u32,2261085941_u32,1554702302_u32];
_13 = (_8.0,);
_2 = core::ptr::addr_of_mut!(_3);
(*_2) = [false];
RET = [11755_u16,4073_u16];
_12 = core::ptr::addr_of_mut!((*_2));
_12 = Move(_2);
_3 = [false];
_5 = [true,false,true,true,false];
_4 = [1866320930_u32,1519874692_u32,3403810954_u32];
_12 = core::ptr::addr_of_mut!(_3);
_2 = Move(_12);
Goto(bb3)
}
bb3 = {
_12 = Move(_2);
_13 = _8;
RET = [27990_u16,52785_u16];
_10 = _7;
_15 = [(-8759594120328360515_i64),(-8237669018882187898_i64),2143741810729034545_i64];
_10 = 1484066172_i32 as f64;
_11 = [3646755245970588791_u64];
_15 = [4700293815829692923_i64,5397685516375016833_i64,2120735887034586447_i64];
_5 = [true,false,true,true,false];
_5 = [true,false,false,true,false];
Call(_7 = fn11(_3, _8, _4, Move(_12), _5, _5, _5, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_4 = [2805642709_u32,1426926793_u32,233512198_u32];
_17 = false;
_12 = core::ptr::addr_of_mut!(_3);
_2 = core::ptr::addr_of_mut!(_3);
_10 = _7 - _7;
_18 = 2490195851_u32 as f64;
_11 = [1316643531920897650_u64];
(*_12) = [_17];
_13.0 = _8.0 + _6;
_17 = _8.0 > _13.0;
_3 = [_17];
_23 = 2306800893_u32 & 2273221446_u32;
Goto(bb5)
}
bb5 = {
(*_12) = [_17];
_3 = [_17];
_8 = _13;
_16 = [42569_u16,12046_u16,6018_u16,21666_u16,47111_u16];
_23 = 1511461716_u32;
_16 = [57715_u16,28167_u16,36294_u16,26662_u16,10583_u16];
_12 = Move(_2);
_6 = _8.0;
_8.0 = _13.0;
_20 = -_10;
_4 = [_23,_23,_23];
_12 = core::ptr::addr_of_mut!(_3);
_5 = [_17,_17,_17,_17,_17];
_2 = core::ptr::addr_of_mut!((*_12));
_13.0 = 228_u8 as usize;
_5 = [_17,_17,_17,_17,_17];
_22 = '\u{7aee2}';
_5 = [_17,_17,_17,_17,_17];
_9 = _10;
_12 = Move(_2);
_6 = !_8.0;
match _23 {
0 => bb6,
1 => bb7,
1511461716 => bb9,
_ => bb8
}
}
bb6 = {
_4 = [2805642709_u32,1426926793_u32,233512198_u32];
_17 = false;
_12 = core::ptr::addr_of_mut!(_3);
_2 = core::ptr::addr_of_mut!(_3);
_10 = _7 - _7;
_18 = 2490195851_u32 as f64;
_11 = [1316643531920897650_u64];
(*_12) = [_17];
_13.0 = _8.0 + _6;
_17 = _8.0 > _13.0;
_3 = [_17];
_23 = 2306800893_u32 & 2273221446_u32;
Goto(bb5)
}
bb7 = {
_12 = Move(_2);
_13 = _8;
RET = [27990_u16,52785_u16];
_10 = _7;
_15 = [(-8759594120328360515_i64),(-8237669018882187898_i64),2143741810729034545_i64];
_10 = 1484066172_i32 as f64;
_11 = [3646755245970588791_u64];
_15 = [4700293815829692923_i64,5397685516375016833_i64,2120735887034586447_i64];
_5 = [true,false,true,true,false];
_5 = [true,false,false,true,false];
Call(_7 = fn11(_3, _8, _4, Move(_12), _5, _5, _5, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
RET = [1071_u16,51449_u16];
RET = [54013_u16,54253_u16];
_2 = core::ptr::addr_of_mut!(_3);
(*_2) = [false];
_3 = [false];
(*_2) = [true];
_7 = (-17778_i16) as f64;
_6 = 1981107141664154251_usize - 4979822762457605080_usize;
_8.0 = _6;
_8.0 = !_6;
_10 = _7;
_9 = _7 * _7;
_6 = 2233810150_u32 as usize;
_11 = [16145949797330252086_u64];
_2 = core::ptr::addr_of_mut!(_3);
_10 = _7 - _9;
(*_2) = [false];
(*_2) = [false];
(*_2) = [true];
Call(_7 = fn10(), ReturnTo(bb2), UnwindUnreachable())
}
bb9 = {
Goto(bb10)
}
bb10 = {
_25.fld3 = [6978804397303871567_i64,326020619387292486_i64,(-4941869885047784421_i64)];
_2 = core::ptr::addr_of_mut!(_3);
(*_2) = [_17];
_16 = [23456_u16,5899_u16,45982_u16,51187_u16,52496_u16];
_25.fld2 = 8806967803114032630_i64;
_8 = _13;
_25.fld3 = _15;
_27 = _6;
_5 = [_17,_17,_17,_17,_17];
_7 = -_9;
_3 = [_17];
_25.fld4 = _16;
match _23 {
0 => bb1,
1 => bb7,
2 => bb6,
3 => bb9,
4 => bb5,
5 => bb11,
6 => bb12,
1511461716 => bb14,
_ => bb13
}
}
bb11 = {
_12 = Move(_2);
_13 = _8;
RET = [27990_u16,52785_u16];
_10 = _7;
_15 = [(-8759594120328360515_i64),(-8237669018882187898_i64),2143741810729034545_i64];
_10 = 1484066172_i32 as f64;
_11 = [3646755245970588791_u64];
_15 = [4700293815829692923_i64,5397685516375016833_i64,2120735887034586447_i64];
_5 = [true,false,true,true,false];
_5 = [true,false,false,true,false];
Call(_7 = fn11(_3, _8, _4, Move(_12), _5, _5, _5, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
RET = [1071_u16,51449_u16];
RET = [54013_u16,54253_u16];
_2 = core::ptr::addr_of_mut!(_3);
(*_2) = [false];
_3 = [false];
(*_2) = [true];
_7 = (-17778_i16) as f64;
_6 = 1981107141664154251_usize - 4979822762457605080_usize;
_8.0 = _6;
_8.0 = !_6;
_10 = _7;
_9 = _7 * _7;
_6 = 2233810150_u32 as usize;
_11 = [16145949797330252086_u64];
_2 = core::ptr::addr_of_mut!(_3);
_10 = _7 - _9;
(*_2) = [false];
(*_2) = [false];
(*_2) = [true];
Call(_7 = fn10(), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_12 = Move(_2);
_13 = _8;
RET = [27990_u16,52785_u16];
_10 = _7;
_15 = [(-8759594120328360515_i64),(-8237669018882187898_i64),2143741810729034545_i64];
_10 = 1484066172_i32 as f64;
_11 = [3646755245970588791_u64];
_15 = [4700293815829692923_i64,5397685516375016833_i64,2120735887034586447_i64];
_5 = [true,false,true,true,false];
_5 = [true,false,false,true,false];
Call(_7 = fn11(_3, _8, _4, Move(_12), _5, _5, _5, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
_23 = 3515464880_u32;
(*_2) = [_17];
_15 = [_25.fld2,_25.fld2,_25.fld2];
_25.fld4 = [16860_u16,1840_u16,11635_u16,63001_u16,13322_u16];
_3 = [_17];
_3 = [_17];
_27 = 54027_u16 as usize;
_23 = 357_i16 as u32;
_25.fld1.1.0 = !4_u8;
_25.fld0 = _25.fld1.1.0 as i32;
_25.fld3 = [_25.fld2,_25.fld2,_25.fld2];
_22 = '\u{c0fc8}';
_15 = _25.fld3;
_23 = 494117773_u32 << _6;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(9_usize, 15_usize, Move(_15), 13_usize, Move(_13), 22_usize, Move(_22), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(9_usize, 23_usize, Move(_23), 17_usize, Move(_17), 32_usize, _32, 32_usize, _32), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10() -> f64 {
mir! {
type RET = f64;
let _1: (Adt54, (u8,));
let _2: usize;
let _3: (((i16, [u32; 3], f64, (u8, u16, i16, Adt22)), [u16; 2], &'static *mut (u8, u16, i16, Adt22)), (bool,), &'static isize);
let _4: isize;
let _5: f32;
let _6: [bool; 2];
let _7: (u8,);
let _8: [char; 8];
let _9: &'static i8;
let _10: f32;
let _11: ((i16, [u32; 3], f64, (u8, u16, i16, Adt22)), [u16; 2], &'static *mut (u8, u16, i16, Adt22));
let _12: Adt80;
let _13: isize;
let _14: (((i16, [u32; 3], f64, (u8, u16, i16, Adt22)), [u16; 2], &'static *mut (u8, u16, i16, Adt22)), (bool,), &'static isize);
let _15: i8;
let _16: u16;
let _17: &'static &'static f64;
let _18: *mut (char, &'static isize);
let _19: (((i16, [u32; 3], f64, (u8, u16, i16, Adt22)), [u16; 2], &'static *mut (u8, u16, i16, Adt22)), (bool,), &'static isize);
let _20: char;
let _21: (u128, (bool,), (u8,), (i8, u128, i128, Adt27));
let _22: (u8, u16, i16, Adt22);
let _23: [u32; 3];
let _24: f32;
let _25: &'static f32;
let _26: (bool,);
let _27: Adt27;
let _28: [u64; 2];
let _29: ();
let _30: ();
{
RET = (-124_i8) as f64;
RET = 3040892396_u32 as f64;
RET = 22_isize as f64;
RET = (-105_i8) as f64;
_2 = (-40223622207709131419513541214597779538_i128) as usize;
_3.0.0.3.0 = !17_u8;
RET = 43_i8 as f64;
_1.1.0 = !_3.0.0.3.0;
_3.0.0.0 = -(-30953_i16);
_3.1.0 = false;
_3.0.0.1 = [2826537874_u32,1268965692_u32,4008695939_u32];
_3.0.0.2 = RET;
_3.0.0.1 = [1478840230_u32,3864339723_u32,737328651_u32];
_3.0.0.0 = 928434518_u32 as i16;
_3.0.0.3.2 = 195569685_i32 as i16;
_3.0.0.3.2 = _3.0.0.0;
Goto(bb1)
}
bb1 = {
_1.1.0 = 9223372036854775807_isize as u8;
_3.1.0 = true | true;
_1.1.0 = _3.0.0.3.0 ^ _3.0.0.3.0;
_3.0.1 = [54854_u16,6873_u16];
_3.0.0.3.3 = Adt22::Variant0 { fld0: _3.1.0 };
_3.1 = (Field::<bool>(Variant(_3.0.0.3.3, 0), 0),);
_3.1.0 = _2 == _2;
place!(Field::<bool>(Variant(_3.0.0.3.3, 0), 0)) = _3.1.0 & _3.1.0;
_3.0.0.1 = [2106139263_u32,1508526126_u32,852644455_u32];
RET = _3.0.0.2 + _3.0.0.2;
_4 = RET as isize;
RET = _1.1.0 as f64;
_3.0.0.3.1 = !32804_u16;
_3.1.0 = !Field::<bool>(Variant(_3.0.0.3.3, 0), 0);
RET = 89_i8 as f64;
SetDiscriminant(_3.0.0.3.3, 0);
_5 = _4 as f32;
_3.0.0.0 = _3.0.0.3.2 - _3.0.0.3.2;
_3.2 = &_4;
_3.0.0.2 = RET + RET;
place!(Field::<bool>(Variant(_3.0.0.3.3, 0), 0)) = _4 < _4;
_3.2 = &_4;
_7 = (_1.1.0,);
Goto(bb2)
}
bb2 = {
_11.0.3.0 = _4 as u8;
_5 = (-24075182284865069481498833758097646306_i128) as f32;
_11.0.2 = 25_i8 as f64;
_11.0.3.1 = _5 as u16;
_1.1.0 = _7.0 + _11.0.3.0;
RET = _11.0.2;
Goto(bb3)
}
bb3 = {
_11.1 = [_11.0.3.1,_11.0.3.1];
_3.0.0.2 = (-75_i8) as f64;
_6 = [_3.1.0,_3.1.0];
_11.0.2 = RET;
_14.1.0 = _3.1.0;
Goto(bb4)
}
bb4 = {
RET = (-72_i8) as f64;
_12.fld3 = [(-6332912007464209103_i64),6043445114543134925_i64,(-4425299165621188462_i64)];
_11.0.3.0 = _1.1.0 + _7.0;
SetDiscriminant(_3.0.0.3.3, 0);
_12.fld0 = (-1558622664_i32);
place!(Field::<bool>(Variant(_3.0.0.3.3, 0), 0)) = !_3.1.0;
_3.1 = (_14.1.0,);
_12.fld1.1 = (_1.1.0,);
_12.fld4 = [_11.0.3.1,_3.0.0.3.1,_11.0.3.1,_11.0.3.1,_11.0.3.1];
_3.2 = &_13;
_9 = &_15;
_14.0.0.0 = _3.0.0.0 + _3.0.0.0;
_7.0 = _12.fld1.1.0;
_11.0.1 = [388348528_u32,3874655470_u32,3414467727_u32];
_11.0.0 = -_3.0.0.0;
match _12.fld0 {
0 => bb3,
1 => bb5,
2 => bb6,
340282366920938463463374607430209588792 => bb8,
_ => bb7
}
}
bb5 = {
_11.1 = [_11.0.3.1,_11.0.3.1];
_3.0.0.2 = (-75_i8) as f64;
_6 = [_3.1.0,_3.1.0];
_11.0.2 = RET;
_14.1.0 = _3.1.0;
Goto(bb4)
}
bb6 = {
_11.0.3.0 = _4 as u8;
_5 = (-24075182284865069481498833758097646306_i128) as f32;
_11.0.2 = 25_i8 as f64;
_11.0.3.1 = _5 as u16;
_1.1.0 = _7.0 + _11.0.3.0;
RET = _11.0.2;
Goto(bb3)
}
bb7 = {
_1.1.0 = 9223372036854775807_isize as u8;
_3.1.0 = true | true;
_1.1.0 = _3.0.0.3.0 ^ _3.0.0.3.0;
_3.0.1 = [54854_u16,6873_u16];
_3.0.0.3.3 = Adt22::Variant0 { fld0: _3.1.0 };
_3.1 = (Field::<bool>(Variant(_3.0.0.3.3, 0), 0),);
_3.1.0 = _2 == _2;
place!(Field::<bool>(Variant(_3.0.0.3.3, 0), 0)) = _3.1.0 & _3.1.0;
_3.0.0.1 = [2106139263_u32,1508526126_u32,852644455_u32];
RET = _3.0.0.2 + _3.0.0.2;
_4 = RET as isize;
RET = _1.1.0 as f64;
_3.0.0.3.1 = !32804_u16;
_3.1.0 = !Field::<bool>(Variant(_3.0.0.3.3, 0), 0);
RET = 89_i8 as f64;
SetDiscriminant(_3.0.0.3.3, 0);
_5 = _4 as f32;
_3.0.0.0 = _3.0.0.3.2 - _3.0.0.3.2;
_3.2 = &_4;
_3.0.0.2 = RET + RET;
place!(Field::<bool>(Variant(_3.0.0.3.3, 0), 0)) = _4 < _4;
_3.2 = &_4;
_7 = (_1.1.0,);
Goto(bb2)
}
bb8 = {
_14.0.0.1 = [2565033692_u32,1700765906_u32,2674003844_u32];
match _12.fld0 {
0 => bb9,
1 => bb10,
340282366920938463463374607430209588792 => bb12,
_ => bb11
}
}
bb9 = {
_1.1.0 = 9223372036854775807_isize as u8;
_3.1.0 = true | true;
_1.1.0 = _3.0.0.3.0 ^ _3.0.0.3.0;
_3.0.1 = [54854_u16,6873_u16];
_3.0.0.3.3 = Adt22::Variant0 { fld0: _3.1.0 };
_3.1 = (Field::<bool>(Variant(_3.0.0.3.3, 0), 0),);
_3.1.0 = _2 == _2;
place!(Field::<bool>(Variant(_3.0.0.3.3, 0), 0)) = _3.1.0 & _3.1.0;
_3.0.0.1 = [2106139263_u32,1508526126_u32,852644455_u32];
RET = _3.0.0.2 + _3.0.0.2;
_4 = RET as isize;
RET = _1.1.0 as f64;
_3.0.0.3.1 = !32804_u16;
_3.1.0 = !Field::<bool>(Variant(_3.0.0.3.3, 0), 0);
RET = 89_i8 as f64;
SetDiscriminant(_3.0.0.3.3, 0);
_5 = _4 as f32;
_3.0.0.0 = _3.0.0.3.2 - _3.0.0.3.2;
_3.2 = &_4;
_3.0.0.2 = RET + RET;
place!(Field::<bool>(Variant(_3.0.0.3.3, 0), 0)) = _4 < _4;
_3.2 = &_4;
_7 = (_1.1.0,);
Goto(bb2)
}
bb10 = {
_11.0.3.0 = _4 as u8;
_5 = (-24075182284865069481498833758097646306_i128) as f32;
_11.0.2 = 25_i8 as f64;
_11.0.3.1 = _5 as u16;
_1.1.0 = _7.0 + _11.0.3.0;
RET = _11.0.2;
Goto(bb3)
}
bb11 = {
_11.1 = [_11.0.3.1,_11.0.3.1];
_3.0.0.2 = (-75_i8) as f64;
_6 = [_3.1.0,_3.1.0];
_11.0.2 = RET;
_14.1.0 = _3.1.0;
Goto(bb4)
}
bb12 = {
_12.fld1.1.0 = _1.1.0;
SetDiscriminant(_3.0.0.3.3, 1);
_14.0.0.3.2 = _11.0.0 >> _4;
place!(Field::<u16>(Variant(_3.0.0.3.3, 1), 2)) = _3.1.0 as u16;
_1.1 = (_7.0,);
_7 = (_11.0.3.0,);
_21.1.0 = _3.1.0;
_21.2 = (_3.0.0.3.0,);
_13 = !_4;
place!(Field::<(isize, f32, u32)>(Variant(_3.0.0.3.3, 1), 3)).1 = _12.fld0 as f32;
_14.0.0.3.1 = (-33_i8) as u16;
Goto(bb13)
}
bb13 = {
place!(Field::<i128>(Variant(_3.0.0.3.3, 1), 7)) = 101826209786274078317068409066901590027_i128;
_21.1.0 = _14.1.0 | _3.1.0;
_3.2 = &_4;
_19.0.0.3.3 = Adt22::Variant0 { fld0: _21.1.0 };
_19.0.0.2 = -_11.0.2;
_14.0.0.3.0 = _1.1.0 | _7.0;
place!(Field::<(isize, f32, u32)>(Variant(_3.0.0.3.3, 1), 3)).1 = -_5;
match _12.fld0 {
0 => bb1,
340282366920938463463374607430209588792 => bb14,
_ => bb9
}
}
bb14 = {
_12.fld3 = [8449116295308759931_i64,(-1161756833077307964_i64),261540987174587764_i64];
_3.0.0.2 = -_11.0.2;
_11.0.3.2 = _3.0.0.0 * _14.0.0.0;
_21.3.0 = -(-58_i8);
_8 = ['\u{cc280}','\u{c4058}','\u{fcf62}','\u{8d981}','\u{b81cb}','\u{535e9}','\u{185e1}','\u{5fd20}'];
place!(Field::<(isize, f32, u32)>(Variant(_3.0.0.3.3, 1), 3)) = (_13, _5, 1162726065_u32);
_3.0.0.2 = RET * _11.0.2;
_14.0.1 = [Field::<u16>(Variant(_3.0.0.3.3, 1), 2),_14.0.0.3.1];
SetDiscriminant(_19.0.0.3.3, 0);
_3.1.0 = _21.1.0;
_21.2.0 = 1194112498341078397_i64 as u8;
_21.3.3 = Adt27::Variant0 { fld0: _12.fld0,fld1: 31471742046935702617310565951153534154_u128,fld2: Field::<(isize, f32, u32)>(Variant(_3.0.0.3.3, 1), 3).2,fld3: _3.0.0.2 };
place!(Field::<bool>(Variant(_19.0.0.3.3, 0), 0)) = !_21.1.0;
_19.0.0.2 = Field::<i32>(Variant(_21.3.3, 0), 0) as f64;
_3.0.0.0 = _5 as i16;
SetDiscriminant(_19.0.0.3.3, 1);
place!(Field::<u128>(Variant(_21.3.3, 0), 1)) = 24338455117747467344648856789124210325_u128;
_3.0.0.2 = (-2360937801361763773_i64) as f64;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(10_usize, 7_usize, Move(_7), 4_usize, Move(_4), 2_usize, Move(_2), 30_usize, _30), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [bool; 1],mut _2: (usize,),mut _3: [u32; 3],mut _4: *mut [bool; 1],mut _5: [bool; 5],mut _6: [bool; 5],mut _7: [bool; 5],mut _8: [bool; 5]) -> f64 {
mir! {
type RET = f64;
let _9: bool;
let _10: u16;
let _11: bool;
let _12: [u128; 1];
let _13: f64;
let _14: ((i16, [u32; 3], f64, (u8, u16, i16, Adt22)), [u16; 2], &'static *mut (u8, u16, i16, Adt22));
let _15: char;
let _16: Adt80;
let _17: *const i16;
let _18: bool;
let _19: u32;
let _20: (u8, u16, i16, Adt22);
let _21: char;
let _22: [u128; 1];
let _23: u64;
let _24: (((i16, [u32; 3], f64, (u8, u16, i16, Adt22)), [u16; 2], &'static *mut (u8, u16, i16, Adt22)), (bool,), &'static isize);
let _25: [bool; 5];
let _26: char;
let _27: *mut Adt32;
let _28: i128;
let _29: u128;
let _30: Adt27;
let _31: Adt74;
let _32: &'static Adt22;
let _33: u16;
let _34: ();
let _35: ();
{
RET = 752089776_i32 as f64;
_9 = false;
_7 = [_9,_9,_9,_9,_9];
_6 = [_9,_9,_9,_9,_9];
_7 = [_9,_9,_9,_9,_9];
RET = 260485334270311146084815831776781475582_u128 as f64;
_9 = false | true;
RET = 5093046361883387159_i64 as f64;
_10 = 52921_u16 | 43230_u16;
_8 = [_9,_9,_9,_9,_9];
_1 = [_9];
_1 = [_9];
_10 = 49860_u16 * 38840_u16;
_10 = 64235_u16;
_2 = (2_usize,);
_4 = core::ptr::addr_of_mut!(_1);
_1 = [_9];
_8 = [_9,_9,_9,_9,_9];
(*_4) = [_9];
(*_4) = [_9];
_7 = [_9,_9,_9,_9,_9];
Call(_10 = fn12(_5, _6, _7, _3, _7, _3, _6, Move(_4), _7, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12 = [87272440902286311285544191985057667747_u128];
_14.0.3.0 = (-165368349955061709735151853355606389681_i128) as u8;
RET = 10109_i16 as f64;
_14.1 = [_10,_10];
_14.0.3.0 = 97_u8 & 39_u8;
_2.0 = RET as usize;
_14.1 = [_10,_10];
_8 = _7;
_9 = false;
_9 = _10 < _10;
_14.1 = [_10,_10];
_1 = [_9];
_14.0.1 = [1455511819_u32,3105049096_u32,2665191885_u32];
_9 = !true;
_11 = !_9;
_8 = _6;
_12 = [100031299458784363842959565880903578555_u128];
_14.0.0 = 11955_i16;
_3 = [2835456354_u32,2949569478_u32,2739239292_u32];
_8 = [_9,_9,_9,_9,_9];
_7 = _6;
_14.0.3.3 = Adt22::Variant0 { fld0: _9 };
_8 = _5;
_14.0.3.1 = _10 * _10;
_9 = !Field::<bool>(Variant(_14.0.3.3, 0), 0);
SetDiscriminant(_14.0.3.3, 2);
_5 = [_9,_9,_9,_11,_11];
_13 = RET;
Call(_14.0.1 = fn13(_11, _14.0.3.1, _1, _12, _14.0.3.1, _10, _8, _14.0.3.1, _9, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
place!(Field::<i64>(Variant(_14.0.3.3, 2), 1)) = 3659719474836546446_i64 - 2575747437536237873_i64;
_1 = [_11];
_14.0.0 = !16886_i16;
_14.0.1 = _3;
_7 = _6;
_14.0.3.0 = 52_u8;
RET = _13;
_14.1 = [_14.0.3.1,_14.0.3.1];
_2 = (4_usize,);
_5 = [_9,_11,_11,_9,_11];
_5 = _8;
place!(Field::<i32>(Variant(_14.0.3.3, 2), 2)) = 1109425757_i32 ^ (-366016055_i32);
_16.fld3 = [Field::<i64>(Variant(_14.0.3.3, 2), 1),Field::<i64>(Variant(_14.0.3.3, 2), 1),Field::<i64>(Variant(_14.0.3.3, 2), 1)];
_4 = core::ptr::addr_of_mut!(_1);
match _14.0.3.0 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
52 => bb9,
_ => bb8
}
}
bb3 = {
_12 = [87272440902286311285544191985057667747_u128];
_14.0.3.0 = (-165368349955061709735151853355606389681_i128) as u8;
RET = 10109_i16 as f64;
_14.1 = [_10,_10];
_14.0.3.0 = 97_u8 & 39_u8;
_2.0 = RET as usize;
_14.1 = [_10,_10];
_8 = _7;
_9 = false;
_9 = _10 < _10;
_14.1 = [_10,_10];
_1 = [_9];
_14.0.1 = [1455511819_u32,3105049096_u32,2665191885_u32];
_9 = !true;
_11 = !_9;
_8 = _6;
_12 = [100031299458784363842959565880903578555_u128];
_14.0.0 = 11955_i16;
_3 = [2835456354_u32,2949569478_u32,2739239292_u32];
_8 = [_9,_9,_9,_9,_9];
_7 = _6;
_14.0.3.3 = Adt22::Variant0 { fld0: _9 };
_8 = _5;
_14.0.3.1 = _10 * _10;
_9 = !Field::<bool>(Variant(_14.0.3.3, 0), 0);
SetDiscriminant(_14.0.3.3, 2);
_5 = [_9,_9,_9,_11,_11];
_13 = RET;
Call(_14.0.1 = fn13(_11, _14.0.3.1, _1, _12, _14.0.3.1, _10, _8, _14.0.3.1, _9, _8), ReturnTo(bb2), UnwindUnreachable())
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
_14.1 = [_14.0.3.1,_10];
_12 = [78193544758007298863710362585298003563_u128];
_12 = [213608327853841487292500964859431719957_u128];
_5 = [_9,_11,_11,_9,_11];
_14.0.0 = !(-3727_i16);
_16.fld1.1 = (_14.0.3.0,);
_14.0.3.0 = _16.fld1.1.0 << _14.0.3.1;
place!(Field::<i32>(Variant(_14.0.3.3, 2), 2)) = 14895492_i32;
_15 = '\u{c513c}';
_14.0.2 = -_13;
_18 = _9;
_15 = '\u{cbf6d}';
_6 = [_11,_11,_11,_9,_9];
Goto(bb10)
}
bb10 = {
_10 = !_14.0.3.1;
_4 = core::ptr::addr_of_mut!((*_4));
_14.0.3.3 = Adt22::Variant0 { fld0: _11 };
place!(Field::<bool>(Variant(_14.0.3.3, 0), 0)) = _11;
_1 = [_18];
_19 = 3335770371_u32 + 2385871722_u32;
_16.fld0 = 1261966014_i32;
_16.fld1.1.0 = _14.0.3.0 + _14.0.3.0;
_1 = [Field::<bool>(Variant(_14.0.3.3, 0), 0)];
Goto(bb11)
}
bb11 = {
_14.0.3.1 = _10;
_14.0.3.2 = !_14.0.0;
_14.0.3.1 = _10;
_12 = [111527642301874596432173412603001287890_u128];
place!(Field::<bool>(Variant(_14.0.3.3, 0), 0)) = _11 & _11;
(*_4) = [Field::<bool>(Variant(_14.0.3.3, 0), 0)];
_14.0.3.2 = !_14.0.0;
_11 = Field::<bool>(Variant(_14.0.3.3, 0), 0);
_1 = [_11];
_16.fld0 = (-316464179_i32);
SetDiscriminant(_14.0.3.3, 1);
_20.2 = 27_i8 as i16;
_14.0.1 = [_19,_19,_19];
Goto(bb12)
}
bb12 = {
_20.2 = -_14.0.0;
RET = _13 * _13;
place!(Field::<u16>(Variant(_14.0.3.3, 1), 2)) = _11 as u16;
_14.1 = [_10,_14.0.3.1];
place!(Field::<f64>(Variant(_14.0.3.3, 1), 5)) = RET;
place!(Field::<bool>(Variant(_14.0.3.3, 1), 0)) = _16.fld1.1.0 > _14.0.3.0;
(*_4) = [Field::<bool>(Variant(_14.0.3.3, 1), 0)];
place!(Field::<(isize, f32, u32)>(Variant(_14.0.3.3, 1), 3)).0 = (-9223372036854775808_isize);
_20.2 = !_14.0.0;
_20.0 = _14.0.3.0 << _16.fld1.1.0;
_17 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_14.0.3.3, 1), 4)));
_14.0.3.3 = Adt22::Variant0 { fld0: _11 };
_2 = (1_usize,);
_2 = (2977839375441531487_usize,);
_14.0.1 = [_19,_19,_19];
_14.0.3.2 = Field::<bool>(Variant(_14.0.3.3, 0), 0) as i16;
_22 = _12;
_14.0.3.2 = -_20.2;
_14.0.3.3 = Adt22::Variant0 { fld0: _9 };
SetDiscriminant(_14.0.3.3, 0);
_16.fld2 = (-6237460758193177933_i64);
_14.1 = [_14.0.3.1,_10];
_19 = 2366291988_u32 >> _20.0;
_8 = [_18,_11,_11,_9,_9];
_16.fld0 = !(-266312518_i32);
Goto(bb13)
}
bb13 = {
_12 = [74581792212636402213585444818147247371_u128];
_4 = core::ptr::addr_of_mut!((*_4));
_2 = (2_usize,);
_14.1 = [_14.0.3.1,_14.0.3.1];
_20.2 = -_14.0.3.2;
_21 = _15;
_3 = [_19,_19,_19];
_14.0.1 = _3;
_24.0.0.3.1 = _14.0.3.1 >> _20.0;
_11 = !_9;
_24.0.0.1 = _14.0.1;
_9 = _18;
_14.0.0 = !_20.2;
place!(Field::<bool>(Variant(_14.0.3.3, 0), 0)) = _11;
_14.0.3.1 = !_24.0.0.3.1;
_14.0.0 = _14.0.3.2 * _20.2;
_14.0.3.2 = _2.0 as i16;
_25 = _8;
_16.fld2 = _14.0.3.2 as i64;
_17 = core::ptr::addr_of!(_20.2);
SetDiscriminant(_14.0.3.3, 2);
Goto(bb14)
}
bb14 = {
_16.fld1.1 = (_20.0,);
_28 = -103206953566441823944175800181796520800_i128;
_24.0.0.3.2 = !(*_17);
_16.fld2 = _11 as i64;
_23 = 16861534415974464141_u64;
_17 = core::ptr::addr_of!(_20.2);
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(11_usize, 7_usize, Move(_7), 11_usize, Move(_11), 28_usize, Move(_28), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(11_usize, 6_usize, Move(_6), 10_usize, Move(_10), 12_usize, Move(_12), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(11_usize, 8_usize, Move(_8), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: [bool; 5],mut _2: [bool; 5],mut _3: [bool; 5],mut _4: [u32; 3],mut _5: [bool; 5],mut _6: [u32; 3],mut _7: [bool; 5],mut _8: *mut [bool; 1],mut _9: [bool; 5],mut _10: (usize,)) -> u16 {
mir! {
type RET = u16;
let _11: *const &'static &'static i8;
let _12: *const Adt27;
let _13: u64;
let _14: isize;
let _15: f64;
let _16: &'static &'static i8;
let _17: isize;
let _18: [i128; 7];
let _19: i64;
let _20: i64;
let _21: &'static (i8, u128, i128, Adt27);
let _22: i16;
let _23: char;
let _24: char;
let _25: (((i16, [u32; 3], f64, (u8, u16, i16, Adt22)), [u16; 2], &'static *mut (u8, u16, i16, Adt22)), (bool,), &'static isize);
let _26: u128;
let _27: &'static i32;
let _28: bool;
let _29: (Adt54, (u8,));
let _30: i64;
let _31: i16;
let _32: Adt47;
let _33: [char; 8];
let _34: usize;
let _35: *mut (char, &'static isize);
let _36: ();
let _37: ();
{
RET = !47305_u16;
_9 = [true,true,false,true,true];
_4 = [1108160475_u32,3879395588_u32,1930772107_u32];
_7 = [false,false,true,true,true];
_10 = (11925762292517871373_usize,);
_9 = [true,false,true,false,true];
_10.0 = false as usize;
Goto(bb1)
}
bb1 = {
_1 = [true,true,false,false,true];
Goto(bb2)
}
bb2 = {
_6 = _4;
_3 = [false,true,true,true,true];
_9 = [true,false,false,true,false];
RET = !24124_u16;
Goto(bb3)
}
bb3 = {
_2 = [true,false,false,true,false];
_7 = _3;
_2 = [true,false,false,true,false];
_3 = [true,false,true,true,true];
_6 = [2729382890_u32,2219046938_u32,3888850766_u32];
RET = 23712_u16 ^ 50169_u16;
Goto(bb4)
}
bb4 = {
_6 = _4;
_10.0 = !1_usize;
_2 = _1;
_7 = _9;
_1 = [false,false,false,true,true];
RET = 20613_u16;
_14 = RET as isize;
_13 = 636026702919153232_u64 - 3126929289380480992_u64;
_3 = [false,true,true,false,true];
_14 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
RET = 39769_u16 + 30758_u16;
RET = !20250_u16;
Goto(bb5)
}
bb5 = {
_6 = [2461218417_u32,44961032_u32,1383065205_u32];
_6 = [2707524757_u32,1099959278_u32,1865055499_u32];
_11 = core::ptr::addr_of!(_16);
_5 = _9;
RET = _13 as u16;
_18 = [(-74952520265093424724085376636753342290_i128),5616455201581670984095273112180610680_i128,(-3938948585244561084350725281386346117_i128),132823692680582291938598610203166122370_i128,(-37896443748236339093878670117395383389_i128),(-78283798035194454455682921034198455835_i128),(-63858039656131549138540766493610128777_i128)];
RET = 31336_u16 << _14;
_7 = _2;
_19 = '\u{fab83}' as i64;
Goto(bb6)
}
bb6 = {
_3 = [true,true,false,true,true];
_17 = (-1526417603_i32) as isize;
_1 = [true,true,false,false,false];
_10 = (4544926356425741023_usize,);
_4 = [152060345_u32,1672252109_u32,2748093801_u32];
_19 = (-3559916072828150159_i64);
RET = _10.0 as u16;
_20 = -_19;
_3 = [true,false,true,false,false];
_15 = RET as f64;
_4 = [3384163592_u32,750880389_u32,3791610824_u32];
_2 = [false,true,false,true,true];
_14 = _17;
_18 = [45371154287767582167227625989864814022_i128,(-26832775140051451934597734331200461770_i128),(-81373196625695045832072340181817073575_i128),(-39778281395219465720884665538064946505_i128),141886041654793808080922679314848537998_i128,(-150646467385775144372642290677007511873_i128),(-45737301530375816759384095578570530382_i128)];
_1 = _9;
_1 = [true,true,false,false,false];
RET = !12443_u16;
_3 = [true,false,true,false,false];
_1 = [true,true,false,true,true];
_7 = _3;
_10.0 = 3815656387_u32 as usize;
_5 = [true,true,true,true,false];
_2 = [false,true,false,false,false];
_22 = -2723_i16;
_3 = [true,true,false,true,false];
_17 = _10.0 as isize;
_19 = -_20;
_2 = _9;
_2 = [false,false,true,true,true];
Goto(bb7)
}
bb7 = {
_6 = _4;
_2 = [false,true,true,true,true];
_15 = 84_u8 as f64;
_14 = _17;
_18 = [154693125011073729399946800030927829878_i128,108706295913023120188870632638027744508_i128,(-68979627495707098804370005211797876710_i128),118084515320190698697878655522214654727_i128,97438255821199233430265463457549389287_i128,(-52884152274886321549605674138842715889_i128),(-129539888961909544392873193671144192860_i128)];
_11 = core::ptr::addr_of!((*_11));
_10.0 = true as usize;
_14 = _17;
_13 = !7066922450784208680_u64;
_14 = _19 as isize;
RET = (-140329405401247095417425161172986879257_i128) as u16;
_13 = !1721008870807165581_u64;
Goto(bb8)
}
bb8 = {
_20 = -_19;
RET = 32009_u16 & 35231_u16;
_14 = _17 | _17;
_23 = '\u{62dde}';
_3 = [true,true,true,true,false];
Goto(bb9)
}
bb9 = {
_10.0 = 4_usize * 11477321140352053813_usize;
Call(RET = core::intrinsics::bswap(6486_u16), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_3 = [false,false,false,false,false];
_10.0 = _22 as usize;
_6 = _4;
_6 = [2264327550_u32,2281144398_u32,1388119374_u32];
_25.0.0.3.2 = _22 - _22;
_25.0.0.3.2 = 187052017398619650870375380270974959453_u128 as i16;
RET = _13 as u16;
_25.0.0.0 = _25.0.0.3.2 + _22;
_25.1 = (true,);
_25.0.1 = [RET,RET];
_2 = _5;
_10 = (6_usize,);
_6 = [1543227611_u32,1974483398_u32,3827178383_u32];
_24 = _23;
_5 = [_25.1.0,_25.1.0,_25.1.0,_25.1.0,_25.1.0];
RET = _19 as u16;
_25.2 = &_17;
_28 = _25.1.0;
_6 = [3376749013_u32,962399327_u32,3692510579_u32];
_25.2 = &_17;
_25.0.0.3.3 = Adt22::Variant0 { fld0: _25.1.0 };
_10.0 = !1327873593103802174_usize;
_5 = [_25.1.0,_28,Field::<bool>(Variant(_25.0.0.3.3, 0), 0),Field::<bool>(Variant(_25.0.0.3.3, 0), 0),Field::<bool>(Variant(_25.0.0.3.3, 0), 0)];
_7 = [_28,Field::<bool>(Variant(_25.0.0.3.3, 0), 0),_25.1.0,_28,Field::<bool>(Variant(_25.0.0.3.3, 0), 0)];
RET = !63056_u16;
_25.0.0.2 = _15 * _15;
_2 = [_28,Field::<bool>(Variant(_25.0.0.3.3, 0), 0),Field::<bool>(Variant(_25.0.0.3.3, 0), 0),_25.1.0,_28];
Goto(bb11)
}
bb11 = {
_1 = _5;
_26 = _25.0.0.0 as u128;
_3 = _9;
_25.0.0.3.2 = _25.0.0.0 ^ _25.0.0.0;
_4 = _6;
_5 = [_28,_28,_28,Field::<bool>(Variant(_25.0.0.3.3, 0), 0),Field::<bool>(Variant(_25.0.0.3.3, 0), 0)];
_9 = [Field::<bool>(Variant(_25.0.0.3.3, 0), 0),_28,_28,Field::<bool>(Variant(_25.0.0.3.3, 0), 0),Field::<bool>(Variant(_25.0.0.3.3, 0), 0)];
_25.0.0.1 = _4;
_25.0.0.3.1 = !RET;
_25.2 = &_14;
place!(Field::<bool>(Variant(_25.0.0.3.3, 0), 0)) = _28;
_29.1.0 = 21_u8 << _26;
_15 = -_25.0.0.2;
_10 = (114593129465601263_usize,);
_18 = [(-162256681175662822302180676413419081712_i128),(-87391877110938989919024772715841560643_i128),46300869134225639427643496198412856068_i128,82416873749408062966211292040257199247_i128,(-97775219529403549037424013375385547502_i128),(-156969187489971408148991825733396191132_i128),(-137644299040431354531920075592932695621_i128)];
SetDiscriminant(_25.0.0.3.3, 1);
_25.2 = &_17;
_10.0 = 5_usize;
_15 = _10.0 as f64;
_25.0.0.1 = _6;
place!(Field::<i128>(Variant(_25.0.0.3.3, 1), 7)) = (-109287167736437737713730237610787422196_i128) & 72828031958400101929035174766871308770_i128;
_2 = [_25.1.0,_25.1.0,_28,_25.1.0,_28];
_26 = _25.0.0.0 as u128;
_9 = [_28,_28,_28,_25.1.0,_28];
Goto(bb12)
}
bb12 = {
place!(Field::<(isize, f32, u32)>(Variant(_25.0.0.3.3, 1), 3)).2 = 2390835026_u32;
place!(Field::<u8>(Variant(_25.0.0.3.3, 1), 1)) = _29.1.0 & _29.1.0;
RET = _25.0.0.3.1 << Field::<u8>(Variant(_25.0.0.3.3, 1), 1);
_25.0.0.3.0 = !Field::<u8>(Variant(_25.0.0.3.3, 1), 1);
Goto(bb13)
}
bb13 = {
place!(Field::<(isize, f32, u32)>(Variant(_25.0.0.3.3, 1), 3)).1 = RET as f32;
_25.0.0.3.0 = Field::<u8>(Variant(_25.0.0.3.3, 1), 1) >> RET;
_25.0.0.0 = Field::<i128>(Variant(_25.0.0.3.3, 1), 7) as i16;
_19 = -_20;
_25.1 = (_28,);
_14 = _17;
place!(Field::<u16>(Variant(_25.0.0.3.3, 1), 2)) = RET;
_18 = [Field::<i128>(Variant(_25.0.0.3.3, 1), 7),Field::<i128>(Variant(_25.0.0.3.3, 1), 7),Field::<i128>(Variant(_25.0.0.3.3, 1), 7),Field::<i128>(Variant(_25.0.0.3.3, 1), 7),Field::<i128>(Variant(_25.0.0.3.3, 1), 7),Field::<i128>(Variant(_25.0.0.3.3, 1), 7),Field::<i128>(Variant(_25.0.0.3.3, 1), 7)];
_25.2 = &place!(Field::<(isize, f32, u32)>(Variant(_25.0.0.3.3, 1), 3)).0;
place!(Field::<bool>(Variant(_25.0.0.3.3, 1), 0)) = _25.1.0 & _25.1.0;
_6 = [Field::<(isize, f32, u32)>(Variant(_25.0.0.3.3, 1), 3).2,Field::<(isize, f32, u32)>(Variant(_25.0.0.3.3, 1), 3).2,Field::<(isize, f32, u32)>(Variant(_25.0.0.3.3, 1), 3).2];
RET = !Field::<u16>(Variant(_25.0.0.3.3, 1), 2);
_31 = _25.0.0.0;
_15 = _14 as f64;
_29.1 = (Field::<u8>(Variant(_25.0.0.3.3, 1), 1),);
place!(Field::<(isize, f32, u32)>(Variant(_25.0.0.3.3, 1), 3)).1 = Field::<i128>(Variant(_25.0.0.3.3, 1), 7) as f32;
_33 = [_23,_24,_24,_23,_24,_24,_23,_23];
place!(Field::<(isize, f32, u32)>(Variant(_25.0.0.3.3, 1), 3)).1 = 1057864462_i32 as f32;
Goto(bb14)
}
bb14 = {
_2 = _7;
_29.1.0 = _25.0.0.3.0 ^ _25.0.0.3.0;
_25.0.0.3.3 = Adt22::Variant0 { fld0: _25.1.0 };
_34 = _13 as usize;
_25.0.0.3.1 = RET & RET;
SetDiscriminant(_25.0.0.3.3, 0);
_28 = _25.0.0.3.0 == _25.0.0.3.0;
place!(Field::<bool>(Variant(_25.0.0.3.3, 0), 0)) = !_28;
_24 = _23;
_4 = [3964769594_u32,2875536611_u32,1187544578_u32];
_29.1 = (_25.0.0.3.0,);
_18 = [(-93908264579943219526955913926349878510_i128),81191089587080581877644272113117395604_i128,(-51054093194129123977450154710599888002_i128),(-65864699336727972841156719877228714906_i128),(-93531522607031590521940882169505029871_i128),115722662564673786462431960625245840804_i128,(-27768825113980529445168831906276795553_i128)];
_34 = !_10.0;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(12_usize, 24_usize, Move(_24), 18_usize, Move(_18), 7_usize, Move(_7), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(12_usize, 3_usize, Move(_3), 23_usize, Move(_23), 5_usize, Move(_5), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(12_usize, 9_usize, Move(_9), 14_usize, Move(_14), 33_usize, Move(_33), 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: bool,mut _2: u16,mut _3: [bool; 1],mut _4: [u128; 1],mut _5: u16,mut _6: u16,mut _7: [bool; 5],mut _8: u16,mut _9: bool,mut _10: [bool; 5]) -> [u32; 3] {
mir! {
type RET = [u32; 3];
let _11: i16;
let _12: f32;
let _13: bool;
let _14: char;
let _15: i64;
let _16: (i128, (u8, u16, i16, Adt22), u64, u8);
let _17: [char; 8];
let _18: isize;
let _19: &'static &'static i8;
let _20: &'static f64;
let _21: i8;
let _22: isize;
let _23: u8;
let _24: [i16; 6];
let _25: i128;
let _26: [usize; 4];
let _27: isize;
let _28: [char; 8];
let _29: i16;
let _30: char;
let _31: [u64; 2];
let _32: char;
let _33: usize;
let _34: ();
let _35: ();
{
_2 = 4194170520578555594_i64 as u16;
RET = [1033908101_u32,1013404046_u32,839644760_u32];
_1 = !_9;
_8 = !_5;
_11 = -6770_i16;
_10 = [_1,_1,_1,_9,_9];
_3 = [_1];
Goto(bb1)
}
bb1 = {
_1 = !_9;
_10 = [_9,_1,_1,_1,_9];
Goto(bb2)
}
bb2 = {
_12 = 2_usize as f32;
_10 = _7;
_9 = _1;
_7 = _10;
_13 = _6 > _8;
_2 = _5 << _8;
_1 = !_13;
_10 = _7;
_8 = _2;
_4 = [104995444508467109674030841866322941783_u128];
_12 = (-7620623271825942113_i64) as f32;
_8 = (-3006344775425758495_i64) as u16;
_13 = _1 & _1;
_14 = '\u{83deb}';
_10 = [_13,_13,_13,_13,_13];
_12 = _11 as f32;
_15 = (-31_i8) as i64;
_5 = !_2;
RET = [3775678416_u32,1141683769_u32,801821561_u32];
_11 = _15 as i16;
_14 = '\u{ed0cd}';
_3 = [_13];
_3 = [_13];
_9 = _13 != _1;
Call(_2 = core::intrinsics::transmute(_6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = [221410508640747974131609590477296535675_u128];
_12 = _5 as f32;
Goto(bb4)
}
bb4 = {
RET = [3649210543_u32,142864168_u32,4218567423_u32];
_13 = !_1;
_5 = !_2;
_6 = _9 as u16;
_13 = _9 & _1;
_16.1.0 = 99_u8;
_16.2 = _16.1.0 as u64;
_16.1.0 = !135_u8;
_6 = _2;
Call(_15 = fn14(_13, _5, _10), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_16.0 = 2589583620_u32 as i128;
RET = [373660281_u32,2428189924_u32,4145547736_u32];
_9 = !_1;
_3 = [_13];
_17 = [_14,_14,_14,_14,_14,_14,_14,_14];
_11 = (-2681_i16) << _6;
_16.1.1 = _6 & _2;
_16.3 = _16.1.0;
_16.1.2 = _11 & _11;
_4 = [37911042573965070000664779321157901941_u128];
_17 = [_14,_14,_14,_14,_14,_14,_14,_14];
_12 = _16.2 as f32;
_4 = [12803537242720742794546247893429576832_u128];
_16.1.3 = Adt22::Variant2 { fld0: _12,fld1: _15,fld2: 1251659038_i32 };
_16.1.1 = _5 & _8;
_4 = [180296647879807906962018825092813107365_u128];
_7 = _10;
_10 = [_13,_9,_13,_1,_13];
_4 = [174181775009711456423122466771796615052_u128];
_10 = [_9,_13,_13,_13,_13];
_16.0 = (-48067816612701991145140215357558582357_i128);
_8 = _2;
_16.1.2 = _12 as i16;
match _16.0 {
292214550308236472318234392074209629099 => bb6,
_ => bb1
}
}
bb6 = {
_9 = !_1;
place!(Field::<i32>(Variant(_16.1.3, 2), 2)) = 334096608_i32 & 1509746329_i32;
SetDiscriminant(_16.1.3, 2);
_8 = !_5;
place!(Field::<i64>(Variant(_16.1.3, 2), 1)) = _15;
RET = [120115249_u32,3416486116_u32,819400500_u32];
_7 = _10;
RET = [535059190_u32,3866650629_u32,3426728173_u32];
place!(Field::<i32>(Variant(_16.1.3, 2), 2)) = -610363615_i32;
_16.3 = _16.1.0 ^ _16.1.0;
Goto(bb7)
}
bb7 = {
_16.1.2 = -_11;
_17 = [_14,_14,_14,_14,_14,_14,_14,_14];
place!(Field::<f32>(Variant(_16.1.3, 2), 0)) = _12 + _12;
_14 = '\u{e36a0}';
_16.1.0 = !_16.3;
_12 = 2258008372889963263943924056730701741_u128 as f32;
_23 = _16.3;
RET = [2090931912_u32,1305422567_u32,1378559488_u32];
_11 = 70143981503644287614169691038928805899_u128 as i16;
_26 = [2_usize,10248441301328608306_usize,2_usize,4_usize];
_10 = _7;
_11 = 660002729_u32 as i16;
_21 = (-71_i8) | (-11_i8);
_15 = 3822273174_u32 as i64;
_7 = _10;
RET = [3216104272_u32,1686080877_u32,494528671_u32];
_26 = [15832206682067814940_usize,12321705263947603530_usize,5875862313244436071_usize,0_usize];
_12 = Field::<f32>(Variant(_16.1.3, 2), 0);
Goto(bb8)
}
bb8 = {
_6 = _8;
_27 = 9223372036854775807_isize * 64_isize;
_18 = !_27;
_13 = !_9;
_13 = _9 != _1;
_16.1.3 = Adt22::Variant2 { fld0: _12,fld1: _15,fld2: (-1284071804_i32) };
_26 = [15212014401775893895_usize,16103008692116815040_usize,18004666299088206234_usize,5_usize];
_2 = _8 - _8;
_16.3 = _23 >> _6;
place!(Field::<i32>(Variant(_16.1.3, 2), 2)) = 1159684653_i32 * 1954009074_i32;
_3 = [_13];
_9 = _1 ^ _13;
place!(Field::<i32>(Variant(_16.1.3, 2), 2)) = 568421405_i32 << _6;
SetDiscriminant(_16.1.3, 2);
Goto(bb9)
}
bb9 = {
_8 = (-1028806983_i32) as u16;
_14 = '\u{f6a14}';
_16.1.2 = (-871913205_i32) as i16;
_27 = _18;
_21 = 109_i8;
_30 = _14;
_2 = !_5;
_25 = _16.0 * _16.0;
_14 = _30;
_17 = [_30,_30,_30,_30,_14,_14,_30,_30];
_11 = _16.1.2;
_25 = _16.0;
Call(_7 = fn16(_13, _13, _3, _9, _10, _10, _10, _15, _3), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_16.1.2 = _11 << _23;
_23 = !_16.3;
_29 = _16.1.2;
place!(Field::<i32>(Variant(_16.1.3, 2), 2)) = !831019617_i32;
_16.1.1 = !_2;
_8 = _16.1.1 & _2;
_16.2 = 4702783281592602781_u64 << _8;
RET = [2970762264_u32,1202554419_u32,3085921729_u32];
place!(Field::<i64>(Variant(_16.1.3, 2), 1)) = _15;
place!(Field::<f32>(Variant(_16.1.3, 2), 0)) = _16.1.2 as f32;
_5 = _9 as u16;
_24 = [_16.1.2,_16.1.2,_29,_16.1.2,_16.1.2,_29];
_5 = _8;
_15 = -Field::<i64>(Variant(_16.1.3, 2), 1);
_5 = _11 as u16;
_1 = !_9;
match _25 {
0 => bb5,
1 => bb11,
2 => bb12,
292214550308236472318234392074209629099 => bb14,
_ => bb13
}
}
bb11 = {
_9 = !_1;
place!(Field::<i32>(Variant(_16.1.3, 2), 2)) = 334096608_i32 & 1509746329_i32;
SetDiscriminant(_16.1.3, 2);
_8 = !_5;
place!(Field::<i64>(Variant(_16.1.3, 2), 1)) = _15;
RET = [120115249_u32,3416486116_u32,819400500_u32];
_7 = _10;
RET = [535059190_u32,3866650629_u32,3426728173_u32];
place!(Field::<i32>(Variant(_16.1.3, 2), 2)) = -610363615_i32;
_16.3 = _16.1.0 ^ _16.1.0;
Goto(bb7)
}
bb12 = {
_1 = !_9;
_10 = [_9,_1,_1,_1,_9];
Goto(bb2)
}
bb13 = {
_16.1.2 = -_11;
_17 = [_14,_14,_14,_14,_14,_14,_14,_14];
place!(Field::<f32>(Variant(_16.1.3, 2), 0)) = _12 + _12;
_14 = '\u{e36a0}';
_16.1.0 = !_16.3;
_12 = 2258008372889963263943924056730701741_u128 as f32;
_23 = _16.3;
RET = [2090931912_u32,1305422567_u32,1378559488_u32];
_11 = 70143981503644287614169691038928805899_u128 as i16;
_26 = [2_usize,10248441301328608306_usize,2_usize,4_usize];
_10 = _7;
_11 = 660002729_u32 as i16;
_21 = (-71_i8) | (-11_i8);
_15 = 3822273174_u32 as i64;
_7 = _10;
RET = [3216104272_u32,1686080877_u32,494528671_u32];
_26 = [15832206682067814940_usize,12321705263947603530_usize,5875862313244436071_usize,0_usize];
_12 = Field::<f32>(Variant(_16.1.3, 2), 0);
Goto(bb8)
}
bb14 = {
_10 = [_13,_9,_9,_9,_1];
_21 = _25 as i8;
_1 = !_9;
_22 = _18;
_22 = Field::<i32>(Variant(_16.1.3, 2), 2) as isize;
_6 = _2 >> _16.2;
_2 = _16.3 as u16;
_16.1.1 = !_8;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(13_usize, 24_usize, Move(_24), 14_usize, Move(_14), 26_usize, Move(_26), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(13_usize, 30_usize, Move(_30), 3_usize, Move(_3), 21_usize, Move(_21), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(13_usize, 9_usize, Move(_9), 8_usize, Move(_8), 27_usize, Move(_27), 22_usize, Move(_22)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: bool,mut _2: u16,mut _3: [bool; 5]) -> i64 {
mir! {
type RET = i64;
let _4: (i128, (u8, u16, i16, Adt22), u64, u8);
let _5: [i64; 3];
let _6: f64;
let _7: [u16; 4];
let _8: f64;
let _9: f64;
let _10: u8;
let _11: bool;
let _12: [i16; 6];
let _13: char;
let _14: isize;
let _15: [u16; 5];
let _16: [u64; 1];
let _17: char;
let _18: bool;
let _19: [u16; 2];
let _20: f64;
let _21: f64;
let _22: Adt80;
let _23: f64;
let _24: isize;
let _25: char;
let _26: (u8,);
let _27: ();
let _28: ();
{
_3 = [_1,_1,_1,_1,_1];
_4.1.0 = !186_u8;
_2 = (-27587_i16) as u16;
_4.2 = 8363748092348653881_u64;
RET = 350573504304526588_i64;
_1 = _4.1.0 > _4.1.0;
_4.2 = 18106687934733796484_u64 * 12730447228671236286_u64;
_4.0 = (-83918651536868788480011586001371607811_i128);
_4.1.2 = -4155_i16;
_4.3 = '\u{7d6ec}' as u8;
RET = 4003830253454703097_i64 * (-2627684959170452125_i64);
_5 = [RET,RET,RET];
Goto(bb1)
}
bb1 = {
RET = (-5315071497598319165_i64) ^ 3619504639485207009_i64;
_3 = [_1,_1,_1,_1,_1];
_6 = (-1274644169_i32) as f64;
_4.1.3 = Adt22::Variant0 { fld0: _1 };
_4.1.1 = !_2;
_7 = [_2,_2,_4.1.1,_4.1.1];
_1 = Field::<bool>(Variant(_4.1.3, 0), 0);
_4.1.2 = (-9386_i16) + 12235_i16;
_6 = _4.0 as f64;
_1 = Field::<bool>(Variant(_4.1.3, 0), 0);
_4.2 = 14822236567459485679_u64 & 6068187129010844824_u64;
Call(_4.2 = core::intrinsics::bswap(16568950746259044885_u64), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = _4.1.1;
_4.1.1 = _1 as u16;
place!(Field::<bool>(Variant(_4.1.3, 0), 0)) = !_1;
_6 = _2 as f64;
_4.3 = _4.1.0 | _4.1.0;
_10 = 16992578594619545914_usize as u8;
_4.0 = (-16812290220201339735498840556832925728_i128) - (-135074488672581544654205248058465080783_i128);
RET = -(-3225380045460769547_i64);
_4.1.0 = 14341545180780432553_usize as u8;
_7 = [_4.1.1,_4.1.1,_4.1.1,_4.1.1];
_4.2 = 1143787000119565783_u64 & 10227516013647780219_u64;
_9 = 3232071016_u32 as f64;
RET = _4.1.1 as i64;
_10 = _4.3 << _2;
_1 = Field::<bool>(Variant(_4.1.3, 0), 0) | Field::<bool>(Variant(_4.1.3, 0), 0);
_4.0 = 17954296463756477265561812916793118590_i128 ^ (-12434721187153692280075037360331448718_i128);
_4.1.0 = !_10;
_3 = [_1,Field::<bool>(Variant(_4.1.3, 0), 0),_1,Field::<bool>(Variant(_4.1.3, 0), 0),_1];
_4.1.0 = _10;
_4.1.2 = -22783_i16;
_4.0 = 29091215634612355109851219843228984834_i128;
place!(Field::<bool>(Variant(_4.1.3, 0), 0)) = _4.2 == _4.2;
_4.3 = _9 as u8;
_8 = _9 - _9;
_9 = _8 + _8;
_4.3 = _10;
Call(_5 = fn15(RET, _4.1.0, Field::<bool>(Variant(_4.1.3, 0), 0), Move(_4), _3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11 = _10 < _10;
_7 = [_2,_2,_2,_2];
_8 = -_9;
_4.1.2 = (-11697_i16);
_4.2 = !2460740854478797107_u64;
_9 = _8;
_4.1.0 = _10;
_9 = _6 * _8;
_4.0 = -(-34748825791070381527266899219526924828_i128);
_1 = _4.0 > _4.0;
RET = (-8617906069417767597_i64);
RET = 3745361584579547181_i64;
RET = 5282702840649748018_i64 >> _10;
_4.2 = 222436317680761898114194695509202180748_u128 as u64;
_12 = [_4.1.2,_4.1.2,_4.1.2,_4.1.2,_4.1.2,_4.1.2];
_4.1.0 = !_10;
RET = _8 as i64;
_4.3 = !_10;
_6 = _9;
_8 = _9 + _6;
Goto(bb4)
}
bb4 = {
_11 = !_1;
_13 = '\u{3eaa9}';
_6 = _8;
RET = !2732384026625991825_i64;
_6 = (-748761722_i32) as f64;
_4.0 = 60652983834432800290007427686089290055_i128 - 151496366503929653344474587882136612746_i128;
_4.1.1 = 337750967_i32 as u16;
_14 = 48_i8 as isize;
_9 = _8;
_7 = [_2,_4.1.1,_4.1.1,_2];
_4.1.0 = 39_i8 as u8;
_11 = _1 | _1;
_7 = [_2,_2,_4.1.1,_4.1.1];
RET = _4.0 as i64;
_4.3 = _4.1.0;
_1 = _4.3 < _10;
match _4.1.2 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
340282366920938463463374607431768199759 => bb11,
_ => bb10
}
}
bb5 = {
_11 = _10 < _10;
_7 = [_2,_2,_2,_2];
_8 = -_9;
_4.1.2 = (-11697_i16);
_4.2 = !2460740854478797107_u64;
_9 = _8;
_4.1.0 = _10;
_9 = _6 * _8;
_4.0 = -(-34748825791070381527266899219526924828_i128);
_1 = _4.0 > _4.0;
RET = (-8617906069417767597_i64);
RET = 3745361584579547181_i64;
RET = 5282702840649748018_i64 >> _10;
_4.2 = 222436317680761898114194695509202180748_u128 as u64;
_12 = [_4.1.2,_4.1.2,_4.1.2,_4.1.2,_4.1.2,_4.1.2];
_4.1.0 = !_10;
RET = _8 as i64;
_4.3 = !_10;
_6 = _9;
_8 = _9 + _6;
Goto(bb4)
}
bb6 = {
_2 = _4.1.1;
_4.1.1 = _1 as u16;
place!(Field::<bool>(Variant(_4.1.3, 0), 0)) = !_1;
_6 = _2 as f64;
_4.3 = _4.1.0 | _4.1.0;
_10 = 16992578594619545914_usize as u8;
_4.0 = (-16812290220201339735498840556832925728_i128) - (-135074488672581544654205248058465080783_i128);
RET = -(-3225380045460769547_i64);
_4.1.0 = 14341545180780432553_usize as u8;
_7 = [_4.1.1,_4.1.1,_4.1.1,_4.1.1];
_4.2 = 1143787000119565783_u64 & 10227516013647780219_u64;
_9 = 3232071016_u32 as f64;
RET = _4.1.1 as i64;
_10 = _4.3 << _2;
_1 = Field::<bool>(Variant(_4.1.3, 0), 0) | Field::<bool>(Variant(_4.1.3, 0), 0);
_4.0 = 17954296463756477265561812916793118590_i128 ^ (-12434721187153692280075037360331448718_i128);
_4.1.0 = !_10;
_3 = [_1,Field::<bool>(Variant(_4.1.3, 0), 0),_1,Field::<bool>(Variant(_4.1.3, 0), 0),_1];
_4.1.0 = _10;
_4.1.2 = -22783_i16;
_4.0 = 29091215634612355109851219843228984834_i128;
place!(Field::<bool>(Variant(_4.1.3, 0), 0)) = _4.2 == _4.2;
_4.3 = _9 as u8;
_8 = _9 - _9;
_9 = _8 + _8;
_4.3 = _10;
Call(_5 = fn15(RET, _4.1.0, Field::<bool>(Variant(_4.1.3, 0), 0), Move(_4), _3), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
RET = (-5315071497598319165_i64) ^ 3619504639485207009_i64;
_3 = [_1,_1,_1,_1,_1];
_6 = (-1274644169_i32) as f64;
_4.1.3 = Adt22::Variant0 { fld0: _1 };
_4.1.1 = !_2;
_7 = [_2,_2,_4.1.1,_4.1.1];
_1 = Field::<bool>(Variant(_4.1.3, 0), 0);
_4.1.2 = (-9386_i16) + 12235_i16;
_6 = _4.0 as f64;
_1 = Field::<bool>(Variant(_4.1.3, 0), 0);
_4.2 = 14822236567459485679_u64 & 6068187129010844824_u64;
Call(_4.2 = core::intrinsics::bswap(16568950746259044885_u64), ReturnTo(bb2), UnwindUnreachable())
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
_4.1.0 = 16045813703348545054_usize as u8;
_15 = [_2,_4.1.1,_4.1.1,_4.1.1,_2];
_18 = !_11;
_11 = _10 != _10;
_3 = [_11,_1,_11,_1,_1];
_12 = [_4.1.2,_4.1.2,_4.1.2,_4.1.2,_4.1.2,_4.1.2];
_3 = [_1,_11,_11,_11,_18];
_2 = _4.3 as u16;
_4.1.3 = Adt22::Variant0 { fld0: _18 };
_12 = [_4.1.2,_4.1.2,_4.1.2,_4.1.2,_4.1.2,_4.1.2];
RET = (-2746655627705412449_i64) * (-4487809225250739399_i64);
_1 = _18 | Field::<bool>(Variant(_4.1.3, 0), 0);
_1 = _18;
SetDiscriminant(_4.1.3, 1);
place!(Field::<(isize, f32, u32)>(Variant(_4.1.3, 1), 3)).0 = _14;
place!(Field::<u16>(Variant(_4.1.3, 1), 2)) = _2 << _10;
_8 = _10 as f64;
_19 = [Field::<u16>(Variant(_4.1.3, 1), 2),_2];
_17 = _13;
place!(Field::<u16>(Variant(_4.1.3, 1), 2)) = !_4.1.1;
_20 = _8;
match _4.1.2 {
340282366920938463463374607431768199759 => bb12,
_ => bb3
}
}
bb12 = {
place!(Field::<(isize, f32, u32)>(Variant(_4.1.3, 1), 3)).1 = 3237068633_u32 as f32;
_22.fld3 = [RET,RET,RET];
_11 = _1 & _18;
place!(Field::<f64>(Variant(_4.1.3, 1), 5)) = _20;
RET = 3229324014_u32 as i64;
_11 = Field::<u16>(Variant(_4.1.3, 1), 2) <= Field::<u16>(Variant(_4.1.3, 1), 2);
match _4.1.2 {
340282366920938463463374607431768199759 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_22.fld1.1.0 = _10;
place!(Field::<i128>(Variant(_4.1.3, 1), 7)) = _4.2 as i128;
place!(Field::<u8>(Variant(_4.1.3, 1), 1)) = _9 as u8;
_4.1.2 = (-29388_i16);
RET = -(-9146733152334614612_i64);
_2 = !Field::<u16>(Variant(_4.1.3, 1), 2);
_11 = !_18;
_22.fld1.1.0 = !Field::<u8>(Variant(_4.1.3, 1), 1);
_5 = _22.fld3;
_15 = [_4.1.1,Field::<u16>(Variant(_4.1.3, 1), 2),Field::<u16>(Variant(_4.1.3, 1), 2),_2,_4.1.1];
_12 = [_4.1.2,_4.1.2,_4.1.2,_4.1.2,_4.1.2,_4.1.2];
place!(Field::<usize>(Variant(_4.1.3, 1), 6)) = 3562379263903592490_usize;
RET = _8 as i64;
_22.fld1.1.0 = Field::<u8>(Variant(_4.1.3, 1), 1) & _10;
_21 = _20;
_22.fld1.1 = (Field::<u8>(Variant(_4.1.3, 1), 1),);
_4.0 = Field::<i128>(Variant(_4.1.3, 1), 7);
_22.fld2 = (-121_i8) as i64;
_22.fld1.1.0 = Field::<u8>(Variant(_4.1.3, 1), 1);
_12 = [_4.1.2,_4.1.2,_4.1.2,_4.1.2,_4.1.2,_4.1.2];
_3 = [_18,_1,_1,_18,_1];
_2 = Field::<u16>(Variant(_4.1.3, 1), 2);
_26 = _22.fld1.1;
_26 = (_22.fld1.1.0,);
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(14_usize, 15_usize, Move(_15), 1_usize, Move(_1), 26_usize, Move(_26), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(14_usize, 12_usize, Move(_12), 7_usize, Move(_7), 14_usize, Move(_14), 28_usize, _28), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: i64,mut _2: u8,mut _3: bool,mut _4: (i128, (u8, u16, i16, Adt22), u64, u8),mut _5: [bool; 5]) -> [i64; 3] {
mir! {
type RET = [i64; 3];
let _6: (u128, (bool,), (u8,), (i8, u128, i128, Adt27));
let _7: usize;
let _8: isize;
let _9: isize;
let _10: bool;
let _11: f64;
let _12: i32;
let _13: &'static Adt22;
let _14: u8;
let _15: [i128; 7];
let _16: i128;
let _17: bool;
let _18: f64;
let _19: i16;
let _20: [bool; 2];
let _21: Adt74;
let _22: Adt22;
let _23: u128;
let _24: (char, &'static isize);
let _25: &'static &'static i8;
let _26: &'static u64;
let _27: f64;
let _28: ();
let _29: ();
{
_4.2 = 11358555268969169785_u64 >> _2;
_4.1.1 = !17227_u16;
RET = [_1,_1,_1];
_4.2 = 13515626992907509756_u64;
RET = [_1,_1,_1];
_6.3.1 = !18066183768945806614346880467014201295_u128;
_6.0 = _6.3.1;
place!(Field::<bool>(Variant(_4.1.3, 0), 0)) = _3;
SetDiscriminant(_4.1.3, 1);
_6.2.0 = _4.3;
_6.2 = (_2,);
_6.3.0 = 75_i8 << _2;
_2 = _4.3;
_6.2 = (_2,);
place!(Field::<u8>(Variant(_4.1.3, 1), 1)) = !_4.3;
_4.1.2 = 5892_i16 << _4.1.0;
match _4.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
13515626992907509756 => bb8,
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
RET = [_1,_1,_1];
_4.1.2 = '\u{10ad70}' as i16;
_2 = _4.3 << _4.0;
_4.0 = (-131812033215837225795212921376783112613_i128) & 97502383670915211655089244251538185623_i128;
_6.3.2 = !_4.0;
_6.0 = _3 as u128;
place!(Field::<usize>(Variant(_4.1.3, 1), 6)) = 6_usize;
match Field::<usize>(Variant(_4.1.3, 1), 6) {
0 => bb5,
6 => bb9,
_ => bb4
}
}
bb9 = {
place!(Field::<i16>(Variant(_4.1.3, 1), 4)) = '\u{7ab3d}' as i16;
_6.2 = (Field::<u8>(Variant(_4.1.3, 1), 1),);
place!(Field::<f64>(Variant(_4.1.3, 1), 5)) = _4.2 as f64;
_4.2 = 7873448459656450198_u64;
_6.2 = (_2,);
place!(Field::<u8>(Variant(_4.1.3, 1), 1)) = _6.2.0;
place!(Field::<i128>(Variant(_4.1.3, 1), 7)) = _4.0 + _6.3.2;
_6.3.3 = Adt27::Variant0 { fld0: (-1288517338_i32),fld1: _6.3.1,fld2: 2615812982_u32,fld3: Field::<f64>(Variant(_4.1.3, 1), 5) };
_6.3.0 = !(-112_i8);
_7 = '\u{16857}' as usize;
place!(Field::<(isize, f32, u32)>(Variant(_4.1.3, 1), 3)).0 = (-114_isize);
_4.1.1 = !27510_u16;
place!(Field::<u32>(Variant(_6.3.3, 0), 2)) = _6.3.0 as u32;
place!(Field::<usize>(Variant(_4.1.3, 1), 6)) = Field::<(isize, f32, u32)>(Variant(_4.1.3, 1), 3).0 as usize;
_11 = Field::<f64>(Variant(_6.3.3, 0), 3) - Field::<f64>(Variant(_4.1.3, 1), 5);
place!(Field::<(isize, f32, u32)>(Variant(_4.1.3, 1), 3)).1 = _4.1.0 as f32;
place!(Field::<(isize, f32, u32)>(Variant(_4.1.3, 1), 3)).2 = !Field::<u32>(Variant(_6.3.3, 0), 2);
place!(Field::<i32>(Variant(_6.3.3, 0), 0)) = 1590503988_i32;
_6.3.0 = !(-96_i8);
_4.0 = _3 as i128;
place!(Field::<(isize, f32, u32)>(Variant(_4.1.3, 1), 3)).0 = !54_isize;
_4.0 = !Field::<i128>(Variant(_4.1.3, 1), 7);
_1 = Field::<u32>(Variant(_6.3.3, 0), 2) as i64;
_4.1.1 = 57600_u16 | 49342_u16;
_6.3.0 = !(-8_i8);
place!(Field::<bool>(Variant(_4.1.3, 1), 0)) = _3;
_6.2 = (_4.1.0,);
Goto(bb10)
}
bb10 = {
_6.1 = (_3,);
_4.1.3 = Adt22::Variant0 { fld0: _3 };
SetDiscriminant(_6.3.3, 0);
_8 = 111_isize >> _6.2.0;
_2 = !_6.2.0;
_4.0 = _6.3.2 ^ _6.3.2;
RET = [_1,_1,_1];
_10 = _3;
_6.3.3 = Adt27::Variant2 { fld0: _10 };
_4.2 = 12934984400537345858_u64 + 9892478256737462762_u64;
_4.1.0 = _6.2.0;
_13 = &_4.1.3;
_1 = (-5369671750914145056_i64);
_6.3.0 = _4.2 as i8;
_15 = [_6.3.2,_4.0,_4.0,_4.0,_4.0,_6.3.2,_4.0];
_16 = _6.3.2 << _4.3;
_10 = _4.1.2 < _4.1.2;
SetDiscriminant(_4.1.3, 1);
_4.0 = _16 << _4.1.2;
_7 = 2_usize;
place!(Field::<i128>(Variant(_4.1.3, 1), 7)) = !_16;
place!(Field::<u16>(Variant(_4.1.3, 1), 2)) = _4.1.1 << _16;
place!(Field::<(isize, f32, u32)>(Variant(_4.1.3, 1), 3)).0 = _10 as isize;
_3 = _16 >= _4.0;
_11 = _4.1.2 as f64;
_14 = _4.3 + _4.1.0;
match _7 {
2 => bb11,
_ => bb1
}
}
bb11 = {
place!(Field::<usize>(Variant(_4.1.3, 1), 6)) = _7 - _7;
_6.3.3 = Adt27::Variant2 { fld0: _3 };
place!(Field::<(isize, f32, u32)>(Variant(_4.1.3, 1), 3)).1 = _4.1.2 as f32;
_4.1.0 = _4.3;
place!(Field::<u16>(Variant(_4.1.3, 1), 2)) = _1 as u16;
_13 = &_4.1.3;
_6.3.3 = Adt27::Variant2 { fld0: _10 };
place!(Field::<f64>(Variant(_4.1.3, 1), 5)) = _4.0 as f64;
_6.0 = _6.3.1;
_6.2 = (_2,);
place!(Field::<i16>(Variant(_4.1.3, 1), 4)) = _4.1.2;
_18 = _1 as f64;
SetDiscriminant(_6.3.3, 1);
_18 = Field::<i128>(Variant((*_13), 1), 7) as f64;
_9 = -Field::<(isize, f32, u32)>(Variant((*_13), 1), 3).0;
_15[_7] = _4.2 as i128;
_8 = _9 - Field::<(isize, f32, u32)>(Variant((*_13), 1), 3).0;
place!(Field::<[u16; 5]>(Variant(_6.3.3, 1), 1))[_7] = _4.1.1 * _4.1.1;
_14 = !_4.3;
_11 = _18 * Field::<f64>(Variant(_4.1.3, 1), 5);
RET[_7] = '\u{58b46}' as i64;
match _1 {
0 => bb9,
1 => bb2,
2 => bb7,
3 => bb4,
340282366920938463458004935680854066400 => bb13,
_ => bb12
}
}
bb12 = {
place!(Field::<i16>(Variant(_4.1.3, 1), 4)) = '\u{7ab3d}' as i16;
_6.2 = (Field::<u8>(Variant(_4.1.3, 1), 1),);
place!(Field::<f64>(Variant(_4.1.3, 1), 5)) = _4.2 as f64;
_4.2 = 7873448459656450198_u64;
_6.2 = (_2,);
place!(Field::<u8>(Variant(_4.1.3, 1), 1)) = _6.2.0;
place!(Field::<i128>(Variant(_4.1.3, 1), 7)) = _4.0 + _6.3.2;
_6.3.3 = Adt27::Variant0 { fld0: (-1288517338_i32),fld1: _6.3.1,fld2: 2615812982_u32,fld3: Field::<f64>(Variant(_4.1.3, 1), 5) };
_6.3.0 = !(-112_i8);
_7 = '\u{16857}' as usize;
place!(Field::<(isize, f32, u32)>(Variant(_4.1.3, 1), 3)).0 = (-114_isize);
_4.1.1 = !27510_u16;
place!(Field::<u32>(Variant(_6.3.3, 0), 2)) = _6.3.0 as u32;
place!(Field::<usize>(Variant(_4.1.3, 1), 6)) = Field::<(isize, f32, u32)>(Variant(_4.1.3, 1), 3).0 as usize;
_11 = Field::<f64>(Variant(_6.3.3, 0), 3) - Field::<f64>(Variant(_4.1.3, 1), 5);
place!(Field::<(isize, f32, u32)>(Variant(_4.1.3, 1), 3)).1 = _4.1.0 as f32;
place!(Field::<(isize, f32, u32)>(Variant(_4.1.3, 1), 3)).2 = !Field::<u32>(Variant(_6.3.3, 0), 2);
place!(Field::<i32>(Variant(_6.3.3, 0), 0)) = 1590503988_i32;
_6.3.0 = !(-96_i8);
_4.0 = _3 as i128;
place!(Field::<(isize, f32, u32)>(Variant(_4.1.3, 1), 3)).0 = !54_isize;
_4.0 = !Field::<i128>(Variant(_4.1.3, 1), 7);
_1 = Field::<u32>(Variant(_6.3.3, 0), 2) as i64;
_4.1.1 = 57600_u16 | 49342_u16;
_6.3.0 = !(-8_i8);
place!(Field::<bool>(Variant(_4.1.3, 1), 0)) = _3;
_6.2 = (_4.1.0,);
Goto(bb10)
}
bb13 = {
_6.3.3 = Adt27::Variant2 { fld0: _5[_7] };
RET[_7] = _1 >> _4.2;
RET = [_1,_1,_1];
place!(Field::<f64>(Variant(_4.1.3, 1), 5)) = _18;
_4.1.3 = Adt22::Variant0 { fld0: _10 };
_9 = _8 << _4.0;
SetDiscriminant(_4.1.3, 1);
place!(Field::<i16>(Variant(_4.1.3, 1), 4)) = _4.1.2 << _14;
RET = [_1,_1,_1];
RET[_7] = _1 & _1;
_7 = !3_usize;
_12 = -1813813882_i32;
_19 = Field::<i16>(Variant(_4.1.3, 1), 4) ^ Field::<i16>(Variant(_4.1.3, 1), 4);
_18 = _11 - _11;
_20 = [_3,_3];
place!(Field::<f64>(Variant(_4.1.3, 1), 5)) = _11 + _18;
SetDiscriminant(_6.3.3, 2);
_6.1 = (_3,);
_6.2 = (_2,);
place!(Field::<f64>(Variant(_4.1.3, 1), 5)) = _18 + _18;
RET = [_1,_1,_1];
place!(Field::<f64>(Variant(_4.1.3, 1), 5)) = _2 as f64;
_6.3.1 = _6.0 | _6.0;
_4.1.2 = -Field::<i16>(Variant(_4.1.3, 1), 4);
_5 = [_3,_3,_6.1.0,_3,_3];
_16 = _6.3.2 & _4.0;
_13 = &_4.1.3;
match _1 {
0 => bb1,
1 => bb11,
2 => bb7,
3 => bb10,
4 => bb5,
5 => bb12,
6 => bb14,
340282366920938463458004935680854066400 => bb16,
_ => bb15
}
}
bb14 = {
_6.1 = (_3,);
_4.1.3 = Adt22::Variant0 { fld0: _3 };
SetDiscriminant(_6.3.3, 0);
_8 = 111_isize >> _6.2.0;
_2 = !_6.2.0;
_4.0 = _6.3.2 ^ _6.3.2;
RET = [_1,_1,_1];
_10 = _3;
_6.3.3 = Adt27::Variant2 { fld0: _10 };
_4.2 = 12934984400537345858_u64 + 9892478256737462762_u64;
_4.1.0 = _6.2.0;
_13 = &_4.1.3;
_1 = (-5369671750914145056_i64);
_6.3.0 = _4.2 as i8;
_15 = [_6.3.2,_4.0,_4.0,_4.0,_4.0,_6.3.2,_4.0];
_16 = _6.3.2 << _4.3;
_10 = _4.1.2 < _4.1.2;
SetDiscriminant(_4.1.3, 1);
_4.0 = _16 << _4.1.2;
_7 = 2_usize;
place!(Field::<i128>(Variant(_4.1.3, 1), 7)) = !_16;
place!(Field::<u16>(Variant(_4.1.3, 1), 2)) = _4.1.1 << _16;
place!(Field::<(isize, f32, u32)>(Variant(_4.1.3, 1), 3)).0 = _10 as isize;
_3 = _16 >= _4.0;
_11 = _4.1.2 as f64;
_14 = _4.3 + _4.1.0;
match _7 {
2 => bb11,
_ => bb1
}
}
bb15 = {
Return()
}
bb16 = {
_1 = 5061207282490121213_i64 | 8243679886026684868_i64;
_12 = 76941454_i32;
_6.0 = _6.3.1 + _6.3.1;
_6.3.2 = _4.0;
_6.2 = (_4.1.0,);
_20 = [_3,_3];
place!(Field::<i128>(Variant(_4.1.3, 1), 7)) = !_4.0;
_18 = _7 as f64;
place!(Field::<u8>(Variant(_4.1.3, 1), 1)) = !_4.1.0;
_8 = _9 - _9;
_6.2.0 = Field::<u8>(Variant(_4.1.3, 1), 1);
place!(Field::<i128>(Variant(_4.1.3, 1), 7)) = _8 as i128;
_6.1.0 = _3 | _3;
_4.1.2 = Field::<i16>(Variant((*_13), 1), 4);
_4.1.1 = 48299_u16;
_5 = [_3,_6.1.0,_6.1.0,_3,_3];
place!(Field::<f64>(Variant(_4.1.3, 1), 5)) = -_18;
_4.1.2 = _7 as i16;
place!(Field::<u8>(Variant(_4.1.3, 1), 1)) = _9 as u8;
place!(Field::<u8>(Variant(_4.1.3, 1), 1)) = _4.3;
_6.3.3 = Adt27::Variant2 { fld0: _3 };
_24.1 = &_9;
RET = [_1,_1,_1];
_6.1.0 = _3 >= _10;
Goto(bb17)
}
bb17 = {
Call(_28 = dump_var(15_usize, 12_usize, Move(_12), 8_usize, Move(_8), 15_usize, Move(_15), 20_usize, Move(_20)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_28 = dump_var(15_usize, 1_usize, Move(_1), 2_usize, Move(_2), 16_usize, Move(_16), 29_usize, _29), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: bool,mut _2: bool,mut _3: [bool; 1],mut _4: bool,mut _5: [bool; 5],mut _6: [bool; 5],mut _7: [bool; 5],mut _8: i64,mut _9: [bool; 1]) -> [bool; 5] {
mir! {
type RET = [bool; 5];
let _10: f64;
let _11: (u8, u16, i16, Adt22);
let _12: Adt22;
let _13: &'static *mut (u8, u16, i16, Adt22);
let _14: bool;
let _15: ();
let _16: ();
{
_6 = [_1,_4,_4,_4,_4];
_9 = [_4];
_5 = _6;
_9 = _3;
_10 = 138_u8 as f64;
_5 = _6;
_1 = _4;
Goto(bb1)
}
bb1 = {
_10 = (-404621599_i32) as f64;
_11.1 = '\u{10710d}' as u16;
_1 = _4;
RET = [_1,_4,_4,_1,_1];
_11.0 = 175_u8;
_7 = [_1,_1,_1,_2,_2];
_14 = _1;
_11.1 = !15941_u16;
_11.3 = Adt22::Variant0 { fld0: _2 };
_6 = [Field::<bool>(Variant(_11.3, 0), 0),_4,_1,_1,_14];
_2 = Field::<bool>(Variant(_11.3, 0), 0);
SetDiscriminant(_11.3, 0);
_11.1 = !38627_u16;
_8 = (-1669580533_i32) as i64;
Goto(bb2)
}
bb2 = {
Call(_15 = dump_var(16_usize, 5_usize, Move(_5), 6_usize, Move(_6), 9_usize, Move(_9), 3_usize, Move(_3)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_15 = dump_var(16_usize, 2_usize, Move(_2), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{6ea49}'), std::hint::black_box((-53_isize)), std::hint::black_box((-55_i8)), std::hint::black_box(234857378322960298660999145088739016386_u128), std::hint::black_box((-397436988_i32)), std::hint::black_box(6994668152011357536_i64), std::hint::black_box((-135469162138384253704255956676576788163_i128)), std::hint::black_box(0_usize), std::hint::black_box(117_u8), std::hint::black_box(17988_u16));
                
            }
impl PrintFDebug for Adt22{
	unsafe fn printf_debug(&self){unsafe{printf("Adt22::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt22 {
Variant0{
fld0: bool,

},
Variant1{
fld0: bool,
fld1: u8,
fld2: u16,
fld3: (isize, f32, u32),
fld4: i16,
fld5: f64,
fld6: usize,
fld7: i128,

},
Variant2{
fld0: f32,
fld1: i64,
fld2: i32,

}}
impl PrintFDebug for Adt27{
	unsafe fn printf_debug(&self){unsafe{printf("Adt27::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt27 {
Variant0{
fld0: i32,
fld1: u128,
fld2: u32,
fld3: f64,

},
Variant1{
fld0: u64,
fld1: [u16; 5],

},
Variant2{
fld0: bool,

}}
impl PrintFDebug for Adt32{
	unsafe fn printf_debug(&self){unsafe{printf("Adt32::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt32 {
Variant0{
fld0: i64,
fld1: i32,

},
Variant1{
fld0: usize,
fld1: u16,
fld2: f64,
fld3: u8,
fld4: u128,

},
Variant2{
fld0: i16,
fld1: f32,
fld2: (bool,),
fld3: i8,

},
Variant3{
fld0: bool,
fld1: u128,
fld2: isize,
fld3: i8,
fld4: i32,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: u64,
fld1: char,
fld2: (u128, (bool,), (u8,), (i8, u128, i128, Adt27)),
fld3: [u16; 5],
fld4: i16,
fld5: i32,

},
Variant1{
fld0: bool,
fld1: (bool,),
fld2: [i16; 6],
fld3: i8,
fld4: *const Adt27,

},
Variant2{
fld0: bool,
fld1: i32,

},
Variant3{
fld0: *const Adt27,
fld1: Adt32,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: [u16; 5],
fld1: Adt47,
fld2: isize,

},
Variant1{
fld0: [i16; 6],
fld1: *const bool,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt48,
fld1: Adt27,
fld2: (i8, u128, i128, Adt27),

},
Variant1{
fld0: f32,
fld1: u128,
fld2: isize,
fld3: i8,
fld4: (u8,),
fld5: i32,
fld6: Adt47,
fld7: usize,

},
Variant2{
fld0: [bool; 5],
fld1: u128,
fld2: f32,
fld3: *const Adt27,
fld4: Adt47,
fld5: u16,
fld6: [i128; 7],

}}
impl PrintFDebug for Adt74{
	unsafe fn printf_debug(&self){unsafe{printf("Adt74::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt74 {
Variant0{
fld0: *const isize,
fld1: [u16; 5],
fld2: (i8, u128, i128, Adt27),
fld3: f64,
fld4: *mut (u8, u16, i16, Adt22),
fld5: [i64; 3],
fld6: (usize,),
fld7: *mut Adt32,

},
Variant1{
fld0: [u16; 4],
fld1: *mut Adt32,

},
Variant2{
fld0: *const Adt27,

}}
impl PrintFDebug for Adt80{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt80{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt80 {
fld0: i32,
fld1: (Adt54, (u8,)),
fld2: i64,
fld3: [i64; 3],
fld4: [u16; 5],
}

