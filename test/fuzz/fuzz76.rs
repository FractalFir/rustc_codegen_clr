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
pub fn fn0(mut _1: u64,mut _2: u128,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16) -> ([isize; 7],) {
mir! {
type RET = ([isize; 7],);
let _12: (i128, u8, f64, i32);
let _13: &'static isize;
let _14: char;
let _15: isize;
let _16: *mut (bool, u128);
let _17: u128;
let _18: &'static i8;
let _19: bool;
let _20: Adt68;
let _21: u128;
let _22: ();
let _23: ();
{
_6 = -(-1311486726_i32);
RET.0 = [(-9223372036854775808_isize),(-9223372036854775808_isize),30_isize,(-29_isize),(-19_isize),9223372036854775807_isize,9223372036854775807_isize];
_12.0 = !55776030397086678027595345070339399036_i128;
_3 = true as isize;
_12.0 = 103433494166532308406031623132831231611_i128 >> _3;
_1 = 3061854677570534650_u64;
_5 = (-20204_i16);
_5 = false as i16;
_12.3 = _6 * _6;
_12.2 = _5 as f64;
_3 = 9223372036854775807_isize << _12.3;
_10 = 203_u8;
RET.0 = [_3,_3,_3,_3,_3,_3,_3];
_3 = 4251962640_u32 as isize;
_4 = _12.2 as i8;
_6 = _12.3;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
3061854677570534650 => bb8,
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
_12.0 = _5 as i128;
_7 = _5 as i64;
_4 = (-67_i8);
_8 = _12.0 | _12.0;
_12.0 = _8 & _8;
_2 = 184114915386000042656082514262033226659_u128 >> _12.0;
_9 = 11881604406434809197_usize;
RET.0 = [_3,_3,_3,_3,_3,_3,_3];
_2 = 165920694909207352554741536386226257477_u128 ^ 223273541650927002570376888263950190907_u128;
_18 = &_4;
RET.0 = [_3,_3,_3,_3,_3,_3,_3];
_12.0 = _8;
_8 = _12.0;
_17 = _2 ^ _2;
_12.1 = _10;
_10 = _12.1;
_11 = _1 as u16;
_11 = _3 as u16;
RET.0 = [_3,_3,_3,_3,_3,_3,_3];
_7 = -773413861241253799_i64;
Goto(bb9)
}
bb9 = {
_14 = '\u{6d7ca}';
_10 = _12.1;
_18 = &_4;
_1 = !394174090115959657_u64;
_7 = 2294600564602164848_i64 >> _2;
_13 = &_3;
_1 = 11787751275821333124_u64 & 3410494750641416325_u64;
_2 = !_17;
RET.0 = [(*_13),_3,(*_13),(*_13),(*_13),(*_13),(*_13)];
_6 = true as i32;
Call(_19 = fn1(Move(_13), RET.0, _8, _12.1, _2, _3, _12), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_10 = (*_18) as u8;
_18 = &(*_18);
_14 = '\u{1e8ef}';
_7 = 353836482914504996_i64;
_12.3 = _6 << _2;
_4 = !(-29_i8);
RET.0 = [_3,_3,_3,_3,_3,_3,_3];
_14 = '\u{10e8b6}';
_13 = &_15;
_17 = !_2;
_11 = !60774_u16;
match _7 {
0 => bb4,
1 => bb11,
2 => bb12,
3 => bb13,
353836482914504996 => bb15,
_ => bb14
}
}
bb11 = {
_14 = '\u{6d7ca}';
_10 = _12.1;
_18 = &_4;
_1 = !394174090115959657_u64;
_7 = 2294600564602164848_i64 >> _2;
_13 = &_3;
_1 = 11787751275821333124_u64 & 3410494750641416325_u64;
_2 = !_17;
RET.0 = [(*_13),_3,(*_13),(*_13),(*_13),(*_13),(*_13)];
_6 = true as i32;
Call(_19 = fn1(Move(_13), RET.0, _8, _12.1, _2, _3, _12), ReturnTo(bb10), UnwindUnreachable())
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
RET.0 = [_3,_3,_3,_3,_3,_3,_3];
_12.3 = -_6;
_15 = _3 + _3;
_5 = _10 as i16;
RET.0 = [_3,_15,_3,_15,_15,_3,_3];
_13 = &_3;
_12.3 = _6;
_21 = _2 | _17;
RET.0 = [_15,(*_13),_15,_3,_15,(*_13),_15];
_17 = _7 as u128;
_7 = 1162992142099927387_i64;
_13 = &_3;
_13 = &(*_13);
_1 = !5125782707400314117_u64;
_19 = true;
_7 = !1734331501381993055_i64;
Goto(bb16)
}
bb16 = {
Call(_22 = dump_var(0_usize, 7_usize, Move(_7), 11_usize, Move(_11), 15_usize, Move(_15), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_22 = dump_var(0_usize, 8_usize, Move(_8), 4_usize, Move(_4), 2_usize, Move(_2), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: &'static isize,mut _2: [isize; 7],mut _3: i128,mut _4: u8,mut _5: u128,mut _6: isize,mut _7: (i128, u8, f64, i32)) -> bool {
mir! {
type RET = bool;
let _8: *const bool;
let _9: Adt56;
let _10: &'static &'static i8;
let _11: u8;
let _12: *const bool;
let _13: [char; 7];
let _14: isize;
let _15: (bool, u128);
let _16: [i8; 5];
let _17: [u16; 3];
let _18: isize;
let _19: i64;
let _20: f64;
let _21: (u64, [u16; 3], f64, u8);
let _22: *mut usize;
let _23: ([isize; 7],);
let _24: isize;
let _25: isize;
let _26: u8;
let _27: char;
let _28: ([isize; 7],);
let _29: &'static u128;
let _30: bool;
let _31: bool;
let _32: (i128, Adt60);
let _33: isize;
let _34: *mut usize;
let _35: [char; 1];
let _36: i128;
let _37: &'static ([u8; 1], (u32,), i16, &'static (&'static u128,));
let _38: char;
let _39: [isize; 7];
let _40: *mut &'static u32;
let _41: f32;
let _42: *mut i16;
let _43: isize;
let _44: isize;
let _45: ();
let _46: ();
{
_7.3 = !1278533275_i32;
_7.2 = _7.1 as f64;
_5 = !149063186175638936627389757278543087783_u128;
_6 = -(-9223372036854775808_isize);
_7.0 = _3 - _3;
RET = _7.2 > _7.2;
_5 = 178078231837625089239865328272973463625_u128 ^ 113702723160471024284192020030833636203_u128;
Goto(bb1)
}
bb1 = {
_1 = &_6;
_7.0 = _3;
_7.1 = _7.2 as u8;
_4 = _7.1 & _7.1;
_4 = _7.1 | _7.1;
RET = _4 < _7.1;
_5 = 12881817327739116735_u64 as u128;
_7.0 = -_3;
RET = false;
_8 = core::ptr::addr_of!(RET);
_7.0 = !_3;
_3 = _7.0 >> _7.3;
Goto(bb2)
}
bb2 = {
_7.2 = 55932_u16 as f64;
RET = true;
RET = !true;
_3 = _7.0 - _7.0;
_2 = [_6,_6,(*_1),(*_1),(*_1),_6,(*_1)];
_14 = (*_1) & _6;
_7.3 = !71853546_i32;
Goto(bb3)
}
bb3 = {
(*_8) = true | true;
_12 = Move(_8);
_11 = !_4;
_5 = _7.2 as u128;
_2 = [_14,(*_1),(*_1),(*_1),(*_1),_6,(*_1)];
RET = false;
_7.0 = _3;
_16 = [17_i8,(-44_i8),(-108_i8),26_i8,43_i8];
Call(_13 = fn2(Move(_1), _14), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_16 = [59_i8,(-23_i8),34_i8,4_i8,118_i8];
_7.3 = -(-2073754865_i32);
_3 = _7.2 as i128;
Goto(bb5)
}
bb5 = {
_16 = [90_i8,126_i8,(-49_i8),(-48_i8),42_i8];
_4 = _11 - _11;
_8 = core::ptr::addr_of!(_15.0);
RET = _14 <= _14;
(*_8) = RET;
RET = _3 != _7.0;
_19 = 3479780032596318522_i64 * 4004641621784470226_i64;
_7.0 = !_3;
_3 = _15.0 as i128;
RET = _7.1 == _7.1;
_15.1 = !_5;
_21.1 = [49762_u16,45066_u16,28105_u16];
_7.2 = (-18028_i16) as f64;
_17 = [45179_u16,63883_u16,6037_u16];
_21.0 = 10273736201740489162_u64 | 13311964892639686051_u64;
_8 = core::ptr::addr_of!(_15.0);
_15 = (RET, _5);
RET = !(*_8);
Goto(bb6)
}
bb6 = {
_3 = _7.0;
_1 = &_18;
_23.0 = [_6,_14,_6,_6,_14,_14,_6];
Goto(bb7)
}
bb7 = {
_19 = (-2887782567773757537_i64);
_21.2 = _7.2 + _7.2;
_21.3 = _11;
_24 = _6;
_15.0 = !RET;
_23.0 = [_6,_14,_24,_14,_14,_6,_6];
_25 = _14 & _14;
(*_8) = RET;
_21.0 = 1101073535727945813_u64;
(*_8) = !RET;
_7.0 = _3;
_5 = _15.1 * _15.1;
Goto(bb8)
}
bb8 = {
_15.0 = RET;
_16 = [(-110_i8),11_i8,(-107_i8),(-87_i8),(-66_i8)];
_23 = (_2,);
_1 = &_24;
_17 = [37092_u16,30958_u16,64115_u16];
_6 = 24365_i16 as isize;
_16 = [(-86_i8),(-9_i8),(-53_i8),27_i8,(-17_i8)];
_8 = core::ptr::addr_of!(_15.0);
_21.0 = !5733174666677395664_u64;
_3 = _7.0 + _7.0;
_20 = _21.2 - _7.2;
_12 = core::ptr::addr_of!((*_8));
RET = (*_12);
_23 = (_2,);
_8 = core::ptr::addr_of!((*_12));
_4 = _21.3 - _11;
_12 = Move(_8);
_19 = _15.0 as i64;
_28.0 = [_25,_25,_24,_14,_25,(*_1),_25];
Goto(bb9)
}
bb9 = {
_2 = [_14,_25,_25,_25,(*_1),_24,_14];
_20 = -_21.2;
_13 = ['\u{d67d7}','\u{c41a}','\u{e8f85}','\u{45c72}','\u{e9e80}','\u{5795}','\u{6b5f9}'];
_15.1 = !_5;
_1 = &_6;
_23.0 = [_14,_25,_25,_25,_14,_24,_24];
_4 = _11;
_27 = '\u{10abb2}';
_16 = [(-127_i8),(-124_i8),83_i8,(-63_i8),(-15_i8)];
_18 = -_14;
_21.0 = 10657884482224629771_u64 + 14501193652245313048_u64;
_7.1 = _11 >> _25;
_6 = -_14;
_21.3 = !_11;
_2 = [_6,_25,_18,_14,_24,_14,_14];
_18 = _21.2 as isize;
RET = !_15.0;
_23.0 = [_24,_14,_25,_24,_25,_25,_18];
_3 = _7.0 | _7.0;
_4 = _7.1;
_32.1.fld2 = _25;
_32.1.fld5 = [_14,_6,_14,_18,_32.1.fld2,_24,_25];
_18 = 27203_u16 as isize;
Goto(bb10)
}
bb10 = {
_31 = RET;
_21 = (17737323948855053892_u64, _17, _7.2, _4);
_13 = [_27,_27,_27,_27,_27,_27,_27];
Goto(bb11)
}
bb11 = {
_21.0 = 11776936027832018271_u64;
_8 = Move(_12);
_12 = core::ptr::addr_of!(_30);
(*_12) = !RET;
_21.3 = 30909_u16 as u8;
_21.0 = 7031429452951618647_u64 + 16899471302747264314_u64;
(*_12) = !_15.0;
_14 = !_32.1.fld2;
_32.0 = _3;
_26 = _4;
_1 = &_32.1.fld2;
RET = _15.0;
_32.1.fld4 = _7.2 as i16;
Goto(bb12)
}
bb12 = {
_32.1.fld5 = [(*_1),_25,_24,_32.1.fld2,(*_1),(*_1),_6];
_7.3 = (-2135372950_i32);
_25 = -(*_1);
_4 = _7.3 as u8;
_21.2 = _7.2;
_30 = RET & RET;
match _7.3 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
340282366920938463463374607429632838506 => bb18,
_ => bb17
}
}
bb13 = {
_16 = [90_i8,126_i8,(-49_i8),(-48_i8),42_i8];
_4 = _11 - _11;
_8 = core::ptr::addr_of!(_15.0);
RET = _14 <= _14;
(*_8) = RET;
RET = _3 != _7.0;
_19 = 3479780032596318522_i64 * 4004641621784470226_i64;
_7.0 = !_3;
_3 = _15.0 as i128;
RET = _7.1 == _7.1;
_15.1 = !_5;
_21.1 = [49762_u16,45066_u16,28105_u16];
_7.2 = (-18028_i16) as f64;
_17 = [45179_u16,63883_u16,6037_u16];
_21.0 = 10273736201740489162_u64 | 13311964892639686051_u64;
_8 = core::ptr::addr_of!(_15.0);
_15 = (RET, _5);
RET = !(*_8);
Goto(bb6)
}
bb14 = {
_31 = RET;
_21 = (17737323948855053892_u64, _17, _7.2, _4);
_13 = [_27,_27,_27,_27,_27,_27,_27];
Goto(bb11)
}
bb15 = {
_2 = [_14,_25,_25,_25,(*_1),_24,_14];
_20 = -_21.2;
_13 = ['\u{d67d7}','\u{c41a}','\u{e8f85}','\u{45c72}','\u{e9e80}','\u{5795}','\u{6b5f9}'];
_15.1 = !_5;
_1 = &_6;
_23.0 = [_14,_25,_25,_25,_14,_24,_24];
_4 = _11;
_27 = '\u{10abb2}';
_16 = [(-127_i8),(-124_i8),83_i8,(-63_i8),(-15_i8)];
_18 = -_14;
_21.0 = 10657884482224629771_u64 + 14501193652245313048_u64;
_7.1 = _11 >> _25;
_6 = -_14;
_21.3 = !_11;
_2 = [_6,_25,_18,_14,_24,_14,_14];
_18 = _21.2 as isize;
RET = !_15.0;
_23.0 = [_24,_14,_25,_24,_25,_25,_18];
_3 = _7.0 | _7.0;
_4 = _7.1;
_32.1.fld2 = _25;
_32.1.fld5 = [_14,_6,_14,_18,_32.1.fld2,_24,_25];
_18 = 27203_u16 as isize;
Goto(bb10)
}
bb16 = {
_15.0 = RET;
_16 = [(-110_i8),11_i8,(-107_i8),(-87_i8),(-66_i8)];
_23 = (_2,);
_1 = &_24;
_17 = [37092_u16,30958_u16,64115_u16];
_6 = 24365_i16 as isize;
_16 = [(-86_i8),(-9_i8),(-53_i8),27_i8,(-17_i8)];
_8 = core::ptr::addr_of!(_15.0);
_21.0 = !5733174666677395664_u64;
_3 = _7.0 + _7.0;
_20 = _21.2 - _7.2;
_12 = core::ptr::addr_of!((*_8));
RET = (*_12);
_23 = (_2,);
_8 = core::ptr::addr_of!((*_12));
_4 = _21.3 - _11;
_12 = Move(_8);
_19 = _15.0 as i64;
_28.0 = [_25,_25,_24,_14,_25,(*_1),_25];
Goto(bb9)
}
bb17 = {
_1 = &_6;
_7.0 = _3;
_7.1 = _7.2 as u8;
_4 = _7.1 & _7.1;
_4 = _7.1 | _7.1;
RET = _4 < _7.1;
_5 = 12881817327739116735_u64 as u128;
_7.0 = -_3;
RET = false;
_8 = core::ptr::addr_of!(RET);
_7.0 = !_3;
_3 = _7.0 >> _7.3;
Goto(bb2)
}
bb18 = {
_32.1.fld0 = (*_12) as u32;
_21.2 = -_7.2;
_32.1.fld5 = [_32.1.fld2,_32.1.fld2,(*_1),_32.1.fld2,(*_1),(*_1),_14];
_15 = (_31, _5);
_25 = (*_1) | _18;
_20 = _15.1 as f64;
_29 = &_5;
_35 = [_27];
_15.1 = !(*_29);
_36 = _32.0;
_38 = _27;
Goto(bb19)
}
bb19 = {
Call(_45 = dump_var(1_usize, 14_usize, Move(_14), 28_usize, Move(_28), 2_usize, Move(_2), 19_usize, Move(_19)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_45 = dump_var(1_usize, 5_usize, Move(_5), 35_usize, Move(_35), 27_usize, Move(_27), 16_usize, Move(_16)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_45 = dump_var(1_usize, 4_usize, Move(_4), 6_usize, Move(_6), 13_usize, Move(_13), 24_usize, Move(_24)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: &'static isize,mut _2: isize) -> [char; 7] {
mir! {
type RET = [char; 7];
let _3: &'static (&'static u128,);
let _4: *const [u128; 3];
let _5: &'static (&'static u128,);
let _6: &'static ([u8; 1], (u32,), i16, &'static (&'static u128,));
let _7: u64;
let _8: (bool, u128);
let _9: isize;
let _10: f32;
let _11: char;
let _12: char;
let _13: [i16; 2];
let _14: *mut i16;
let _15: f32;
let _16: f64;
let _17: (*const bool, i8, &'static &'static u32, Adt42);
let _18: f64;
let _19: Adt56;
let _20: &'static i64;
let _21: char;
let _22: [u32; 8];
let _23: i8;
let _24: isize;
let _25: [u8; 1];
let _26: [u16; 3];
let _27: [u16; 3];
let _28: &'static (&'static u128,);
let _29: (*const f32,);
let _30: bool;
let _31: [i8; 5];
let _32: *mut Adt42;
let _33: i128;
let _34: [char; 1];
let _35: i128;
let _36: [u8; 1];
let _37: &'static i8;
let _38: [i8; 2];
let _39: [i16; 2];
let _40: *mut i16;
let _41: u16;
let _42: *const [u128; 3];
let _43: [char; 1];
let _44: char;
let _45: (&'static u32,);
let _46: (*const bool, i8, &'static &'static u32, Adt42);
let _47: ();
let _48: ();
{
RET = ['\u{14b7}','\u{4ebf8}','\u{e3ac4}','\u{9d15e}','\u{a460b}','\u{ad577}','\u{10ec8d}'];
RET = ['\u{93abd}','\u{2027c}','\u{13622}','\u{ee7e2}','\u{5c6bb}','\u{a05c3}','\u{b88c2}'];
_2 = '\u{10c80e}' as isize;
RET = ['\u{9bfcc}','\u{3e350}','\u{f8781}','\u{cacfc}','\u{eead8}','\u{12890}','\u{e9fcb}'];
RET = ['\u{653c1}','\u{9b981}','\u{1ff72}','\u{fbea1}','\u{10b89f}','\u{742f2}','\u{e16d1}'];
_1 = &_2;
RET = ['\u{47b45}','\u{39404}','\u{5c959}','\u{87bcb}','\u{8099f}','\u{37ff5}','\u{b2773}'];
RET = ['\u{2b078}','\u{a2b51}','\u{d785f}','\u{218fe}','\u{249eb}','\u{fdfc}','\u{fcf47}'];
RET = ['\u{10a936}','\u{f2b3c}','\u{285ad}','\u{4ced6}','\u{10e0eb}','\u{84f31}','\u{9f1d3}'];
RET = ['\u{31b1c}','\u{98738}','\u{65c9c}','\u{bb201}','\u{431d8}','\u{c1321}','\u{dd27c}'];
_2 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_1 = &_2;
RET = ['\u{78262}','\u{10f6c1}','\u{54e20}','\u{1066c8}','\u{e16e4}','\u{e0001}','\u{3f4e5}'];
RET = ['\u{d4aa}','\u{47c2c}','\u{556bc}','\u{291e0}','\u{1047df}','\u{f5d7f}','\u{1067ed}'];
RET = ['\u{54932}','\u{4f5ae}','\u{a9676}','\u{23af1}','\u{2bf54}','\u{100d01}','\u{109488}'];
RET = ['\u{1f1d}','\u{8de47}','\u{32e5d}','\u{bbdee}','\u{b9730}','\u{89989}','\u{fe623}'];
RET = ['\u{e645}','\u{1084bf}','\u{1fe2c}','\u{4aa17}','\u{31d2}','\u{58dc2}','\u{4f649}'];
RET = ['\u{ec2d8}','\u{1ee94}','\u{ec04a}','\u{107ec}','\u{310e7}','\u{8c6ad}','\u{63b35}'];
RET = ['\u{c3fc4}','\u{acb5a}','\u{da115}','\u{698b3}','\u{c4d5e}','\u{d1a7e}','\u{dedcf}'];
_2 = 55_isize | (-63_isize);
Goto(bb1)
}
bb1 = {
_1 = &_2;
RET = ['\u{514a0}','\u{d068c}','\u{c6152}','\u{a9fed}','\u{debb4}','\u{570a8}','\u{ea68e}'];
_8.1 = 129203091416344033972272246120904176961_u128 - 114862608323515840036965377604620786240_u128;
_7 = !288784450288964542_u64;
RET = ['\u{bb780}','\u{2db82}','\u{80e87}','\u{2c7e}','\u{c1134}','\u{81581}','\u{358a0}'];
_7 = 77_u8 as u64;
_8.0 = !false;
Call(_4 = fn3(Move(_1), _8, _8.1, (*_1), (*_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = (-79_isize);
_2 = !27_isize;
_1 = &_2;
_2 = (-106_i8) as isize;
_1 = &_2;
_7 = 6617899502921581854_u64;
_2 = !(-9223372036854775808_isize);
_1 = &_9;
RET = ['\u{18cb0}','\u{6b2af}','\u{10eec5}','\u{4d15a}','\u{d17d7}','\u{91a64}','\u{e2678}'];
_8.1 = 83480724159067092160883775178942204548_u128 & 273575018254469261578230773460653747306_u128;
_8.1 = !279721016956966681359532272408949428331_u128;
RET = ['\u{a83dd}','\u{17402}','\u{24faa}','\u{44005}','\u{71e4d}','\u{10f74a}','\u{8a0f1}'];
_1 = &(*_1);
_7 = 12040167421967584506_u64;
_8 = (true, 151097473212771058741345088620709531136_u128);
_1 = &(*_1);
_1 = &_2;
RET = ['\u{31092}','\u{d6692}','\u{438d9}','\u{69787}','\u{431a8}','\u{a3a30}','\u{a32c5}'];
_9 = (*_1);
_1 = &(*_1);
_8.1 = 262349430414110330434796861018694635950_u128;
_11 = '\u{f806f}';
_9 = !(*_1);
_8.1 = 332546808633095954910516319444745865515_u128;
_8.0 = false;
_9 = !(*_1);
RET = [_11,_11,_11,_11,_11,_11,_11];
Call(_8.1 = core::intrinsics::bswap(86672084553725139947730121977629952321_u128), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_12 = _11;
_8 = (false, 198346226183706893105617080280856899431_u128);
_8.1 = !212081025895167831459948893850716644276_u128;
RET = [_11,_11,_12,_11,_12,_11,_11];
_11 = _12;
_10 = _8.1 as f32;
_8 = (true, 80505859429268328175033351968015738152_u128);
Goto(bb4)
}
bb4 = {
_15 = _10;
_16 = 12327263299690606008_usize as f64;
_2 = _9 | _9;
_13 = [99_i16,(-25214_i16)];
_2 = -_9;
_7 = _16 as u64;
_17.1 = 7474288864272095281_i64 as i8;
_18 = -_16;
_17.3.fld1 = 6_usize as f64;
_17.1 = 50_i8;
RET = [_11,_11,_12,_12,_11,_11,_11];
_15 = -_10;
_16 = (-3969308027586472283_i64) as f64;
Call(_17.3.fld3.2 = core::intrinsics::fmaf64(_16, _16, _17.3.fld1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_17.3.fld3.1 = 133_u8;
_17.3.fld3.2 = _16 - _17.3.fld1;
_14 = core::ptr::addr_of_mut!(_17.3.fld4);
_11 = _12;
_15 = _17.1 as f32;
_17.3.fld3.2 = _18;
_17.3.fld4 = _9 as i16;
_1 = &_2;
_17.3.fld5 = _12 as i32;
_8.0 = false;
(*_14) = _11 as i16;
_17.3.fld0 = _7 | _7;
(*_14) = _8.0 as i16;
_2 = !_9;
_17.3.fld1 = _16;
_24 = !_2;
_23 = !_17.1;
Goto(bb6)
}
bb6 = {
_8.0 = false;
_17.3.fld3.2 = _16 - _17.3.fld1;
_18 = -_17.3.fld1;
_1 = &_9;
match _8.1 {
0 => bb7,
1 => bb8,
80505859429268328175033351968015738152 => bb10,
_ => bb9
}
}
bb7 = {
_2 = (-79_isize);
_2 = !27_isize;
_1 = &_2;
_2 = (-106_i8) as isize;
_1 = &_2;
_7 = 6617899502921581854_u64;
_2 = !(-9223372036854775808_isize);
_1 = &_9;
RET = ['\u{18cb0}','\u{6b2af}','\u{10eec5}','\u{4d15a}','\u{d17d7}','\u{91a64}','\u{e2678}'];
_8.1 = 83480724159067092160883775178942204548_u128 & 273575018254469261578230773460653747306_u128;
_8.1 = !279721016956966681359532272408949428331_u128;
RET = ['\u{a83dd}','\u{17402}','\u{24faa}','\u{44005}','\u{71e4d}','\u{10f74a}','\u{8a0f1}'];
_1 = &(*_1);
_7 = 12040167421967584506_u64;
_8 = (true, 151097473212771058741345088620709531136_u128);
_1 = &(*_1);
_1 = &_2;
RET = ['\u{31092}','\u{d6692}','\u{438d9}','\u{69787}','\u{431a8}','\u{a3a30}','\u{a32c5}'];
_9 = (*_1);
_1 = &(*_1);
_8.1 = 262349430414110330434796861018694635950_u128;
_11 = '\u{f806f}';
_9 = !(*_1);
_8.1 = 332546808633095954910516319444745865515_u128;
_8.0 = false;
_9 = !(*_1);
RET = [_11,_11,_11,_11,_11,_11,_11];
Call(_8.1 = core::intrinsics::bswap(86672084553725139947730121977629952321_u128), ReturnTo(bb3), UnwindUnreachable())
}
bb8 = {
_15 = _10;
_16 = 12327263299690606008_usize as f64;
_2 = _9 | _9;
_13 = [99_i16,(-25214_i16)];
_2 = -_9;
_7 = _16 as u64;
_17.1 = 7474288864272095281_i64 as i8;
_18 = -_16;
_17.3.fld1 = 6_usize as f64;
_17.1 = 50_i8;
RET = [_11,_11,_12,_12,_11,_11,_11];
_15 = -_10;
_16 = (-3969308027586472283_i64) as f64;
Call(_17.3.fld3.2 = core::intrinsics::fmaf64(_16, _16, _17.3.fld1), ReturnTo(bb5), UnwindUnreachable())
}
bb9 = {
_1 = &_2;
RET = ['\u{514a0}','\u{d068c}','\u{c6152}','\u{a9fed}','\u{debb4}','\u{570a8}','\u{ea68e}'];
_8.1 = 129203091416344033972272246120904176961_u128 - 114862608323515840036965377604620786240_u128;
_7 = !288784450288964542_u64;
RET = ['\u{bb780}','\u{2db82}','\u{80e87}','\u{2c7e}','\u{c1134}','\u{81581}','\u{358a0}'];
_7 = 77_u8 as u64;
_8.0 = !false;
Call(_4 = fn3(Move(_1), _8, _8.1, (*_1), (*_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_17.3.fld2 = -_9;
_26 = [62023_u16,40909_u16,34312_u16];
_22 = [1323999119_u32,1365171716_u32,363008978_u32,499201151_u32,4278894110_u32,826765289_u32,3181425989_u32,2360140641_u32];
_8.0 = !true;
_17.0 = core::ptr::addr_of!(_30);
_17.3.fld1 = (-6525035664428723161_i64) as f64;
_8.0 = true;
_21 = _11;
_17.0 = core::ptr::addr_of!(_30);
_17.3.fld3.2 = _18;
_1 = &(*_1);
_8.1 = 200988978564649360036544682546470851606_u128 ^ 338820992609212494966251684216200839502_u128;
_17.3.fld1 = -_18;
_17.3.fld3.0 = 124199483772498442078996193052020426915_i128;
_14 = core::ptr::addr_of_mut!((*_14));
_25 = [_17.3.fld3.1];
RET = [_11,_11,_11,_21,_11,_12,_21];
_29.0 = core::ptr::addr_of!(_15);
Goto(bb11)
}
bb11 = {
_31 = [_23,_23,_17.1,_23,_23];
_17.3.fld3.3 = !_17.3.fld5;
_2 = _24 - _9;
_17.3.fld5 = _17.3.fld3.3 >> _17.3.fld3.3;
_17.3.fld3.3 = _17.3.fld5 | _17.3.fld5;
_8.0 = !false;
_24 = -_17.3.fld2;
_33 = _17.3.fld3.0;
_37 = &_17.1;
_17.3.fld5 = !_17.3.fld3.3;
_21 = _12;
_17.3.fld3.2 = _7 as f64;
_34 = [_21];
_14 = core::ptr::addr_of_mut!(_17.3.fld4);
_17.3.fld3 = (_33, 137_u8, _16, _17.3.fld5);
_27 = [30492_u16,56384_u16,56167_u16];
_38 = [_17.1,(*_37)];
_33 = _17.3.fld3.0;
_15 = -_10;
_17.0 = core::ptr::addr_of!(_8.0);
match _17.3.fld3.0 {
0 => bb5,
1 => bb7,
2 => bb9,
3 => bb4,
4 => bb12,
124199483772498442078996193052020426915 => bb14,
_ => bb13
}
}
bb12 = {
_15 = _10;
_16 = 12327263299690606008_usize as f64;
_2 = _9 | _9;
_13 = [99_i16,(-25214_i16)];
_2 = -_9;
_7 = _16 as u64;
_17.1 = 7474288864272095281_i64 as i8;
_18 = -_16;
_17.3.fld1 = 6_usize as f64;
_17.1 = 50_i8;
RET = [_11,_11,_12,_12,_11,_11,_11];
_15 = -_10;
_16 = (-3969308027586472283_i64) as f64;
Call(_17.3.fld3.2 = core::intrinsics::fmaf64(_16, _16, _17.3.fld1), ReturnTo(bb5), UnwindUnreachable())
}
bb13 = {
_1 = &_2;
RET = ['\u{514a0}','\u{d068c}','\u{c6152}','\u{a9fed}','\u{debb4}','\u{570a8}','\u{ea68e}'];
_8.1 = 129203091416344033972272246120904176961_u128 - 114862608323515840036965377604620786240_u128;
_7 = !288784450288964542_u64;
RET = ['\u{bb780}','\u{2db82}','\u{80e87}','\u{2c7e}','\u{c1134}','\u{81581}','\u{358a0}'];
_7 = 77_u8 as u64;
_8.0 = !false;
Call(_4 = fn3(Move(_1), _8, _8.1, (*_1), (*_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_30 = _8.0;
_33 = _17.3.fld3.0;
RET = [_21,_11,_11,_21,_12,_12,_12];
_1 = &_9;
_7 = _17.3.fld0 | _17.3.fld0;
_17.3.fld4 = -4219_i16;
_35 = _30 as i128;
_44 = _12;
_24 = _23 as isize;
_38 = [(*_37),_23];
_17.3.fld3.0 = _35 << (*_1);
_19 = Adt56::Variant1 { fld0: _30,fld1: 2367518016_u32,fld2: Move(_29),fld3: _15 };
_32 = core::ptr::addr_of_mut!(_17.3);
_7 = (*_37) as u64;
(*_32).fld2 = -_2;
_13 = [(*_32).fld4,(*_32).fld4];
_17.3.fld3.3 = (*_32).fld5 & (*_32).fld5;
Goto(bb15)
}
bb15 = {
Call(_47 = dump_var(2_usize, 23_usize, Move(_23), 34_usize, Move(_34), 24_usize, Move(_24), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_47 = dump_var(2_usize, 25_usize, Move(_25), 8_usize, Move(_8), 12_usize, Move(_12), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(2_usize, 31_usize, Move(_31), 21_usize, Move(_21), 48_usize, _48, 48_usize, _48), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: &'static isize,mut _2: (bool, u128),mut _3: u128,mut _4: isize,mut _5: isize) -> *const [u128; 3] {
mir! {
type RET = *const [u128; 3];
let _6: (*const f32,);
let _7: [u32; 8];
let _8: u16;
let _9: f32;
let _10: char;
let _11: (char, u128, i32);
let _12: bool;
let _13: i64;
let _14: char;
let _15: (i128, u8, f64, i32);
let _16: f32;
let _17: char;
let _18: u16;
let _19: *mut (bool, u128);
let _20: (char, u128, i32);
let _21: &'static [u8; 1];
let _22: i8;
let _23: f32;
let _24: bool;
let _25: [u32; 8];
let _26: f32;
let _27: [u128; 3];
let _28: ();
let _29: ();
{
_5 = !_4;
Goto(bb1)
}
bb1 = {
_5 = 796146803_u32 as isize;
_1 = &_4;
_2 = (true, _3);
_2.0 = true | true;
_2.1 = !_3;
_2.1 = 76_u8 as u128;
_5 = (*_1) + (*_1);
_2.0 = true & true;
_5 = (*_1);
_5 = -_4;
_1 = &_4;
_5 = -_4;
_2.1 = 381961473_u32 as u128;
Goto(bb2)
}
bb2 = {
_2.1 = _3 | _3;
_8 = 31926_u16;
_2 = (false, _3);
_5 = _4 ^ _4;
_1 = &(*_1);
_8 = !38320_u16;
_1 = &(*_1);
_7 = [1627865325_u32,3686525879_u32,2396109630_u32,3897071770_u32,3155299232_u32,1038101242_u32,2035022336_u32,3390105707_u32];
_2.0 = !true;
_8 = !32468_u16;
_2.0 = _5 == _4;
_5 = 112_i8 as isize;
_2.0 = false;
Call(_4 = fn4(_7, _7, _7, _2.1, _2.1, _5, _2.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_2.1 = !_3;
_10 = '\u{b0091}';
_5 = 171_u8 as isize;
_7 = [2773194642_u32,3130766647_u32,20874435_u32,3086813186_u32,1522899401_u32,902553450_u32,2042231529_u32,2724785994_u32];
_1 = &_5;
_9 = 354272165931817379_i64 as f32;
_11.2 = 835584090_i32;
_11.0 = _10;
_3 = 31479_i16 as u128;
_2.1 = _8 as u128;
_11.2 = 59713448174378257301760959989027317478_i128 as i32;
_11.1 = _3 - _2.1;
_1 = &(*_1);
_14 = _10;
_13 = (-745295658683752149_i64) + (-7492199432717456008_i64);
_13 = !947216868965206020_i64;
_6.0 = core::ptr::addr_of!(_9);
_5 = _14 as isize;
_11.1 = !_3;
_9 = 17581702191752116127_usize as f32;
_3 = !_2.1;
_10 = _11.0;
_3 = !_2.1;
Goto(bb4)
}
bb4 = {
_11.2 = 831849815_i32 * 114882572_i32;
_15.3 = _11.2;
_17 = _10;
_9 = _4 as f32;
_15.0 = !(-66327376540019783748443034832128915939_i128);
_12 = !_2.0;
_11.0 = _14;
_2 = (_12, _11.1);
_15.1 = 250_u8 >> _4;
_17 = _14;
_17 = _14;
_14 = _11.0;
_15.3 = _11.2 | _11.2;
_11.1 = !_2.1;
_14 = _17;
_11.2 = (-7591_i16) as i32;
_15.2 = _4 as f64;
_7 = [1991233402_u32,4069310560_u32,4137285537_u32,3020883205_u32,3981659058_u32,2883971565_u32,3899224000_u32,1604100792_u32];
_20 = (_14, _2.1, _11.2);
_18 = _8;
_5 = -_4;
_11.0 = _17;
Goto(bb5)
}
bb5 = {
_16 = _9;
_2 = (_12, _11.1);
_4 = !_5;
_17 = _20.0;
_11.0 = _10;
_7 = [2934565768_u32,2781129597_u32,3765823751_u32,1793843567_u32,4064924478_u32,2250631387_u32,2325247099_u32,3851049232_u32];
_11.0 = _10;
_3 = (-84_i8) as u128;
_22 = -8_i8;
_8 = _18 | _18;
_11.0 = _17;
_22 = -(-126_i8);
_18 = _8 >> _4;
_11.1 = !_20.1;
_20.1 = _11.1;
_7 = [1598296177_u32,4018582230_u32,1629169683_u32,4153715406_u32,1967876072_u32,4067827763_u32,4186499108_u32,3332946946_u32];
_10 = _11.0;
_23 = -_16;
_15.0 = !(-125967171789264927011247880739881960255_i128);
Goto(bb6)
}
bb6 = {
_15.3 = _2.0 as i32;
_24 = _2.0 | _12;
_2.1 = _5 as u128;
_15.1 = !118_u8;
_1 = &_4;
_3 = 18329_i16 as u128;
_13 = -8267457864671600904_i64;
_15.3 = -_20.2;
_15.0 = !164303373938027144782595038358123989245_i128;
_7 = [927281324_u32,2027829260_u32,3281740849_u32,3672419619_u32,2436792465_u32,2182898043_u32,1882562362_u32,1180487274_u32];
_25 = _7;
_20 = _11;
_17 = _20.0;
_13 = (-4203240797610249793_i64);
_9 = _23 + _23;
_27 = [_2.1,_2.1,_2.1];
_15.2 = _15.1 as f64;
_5 = -(*_1);
_2 = (_12, _3);
_23 = _9;
_3 = _13 as u128;
_4 = _5;
_9 = _23;
_1 = &_5;
_13 = _9 as i64;
Goto(bb7)
}
bb7 = {
_19 = core::ptr::addr_of_mut!(_2);
Goto(bb8)
}
bb8 = {
_2.0 = _24;
_1 = &_5;
Goto(bb9)
}
bb9 = {
_22 = _15.2 as i8;
(*_19).0 = (*_1) > (*_1);
(*_19) = (_24, _3);
_2.0 = _24 ^ _24;
(*_19).0 = _24;
Goto(bb10)
}
bb10 = {
_11.0 = _20.0;
_11.2 = _13 as i32;
RET = core::ptr::addr_of!(_27);
_5 = _4;
_2 = (_12, _20.1);
_6.0 = core::ptr::addr_of!(_16);
_11.1 = _2.1;
Goto(bb11)
}
bb11 = {
Call(_28 = dump_var(3_usize, 17_usize, Move(_17), 22_usize, Move(_22), 13_usize, Move(_13), 20_usize, Move(_20)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_28 = dump_var(3_usize, 8_usize, Move(_8), 3_usize, Move(_3), 14_usize, Move(_14), 12_usize, Move(_12)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_28 = dump_var(3_usize, 25_usize, Move(_25), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: [u32; 8],mut _2: [u32; 8],mut _3: [u32; 8],mut _4: u128,mut _5: u128,mut _6: isize,mut _7: bool) -> isize {
mir! {
type RET = isize;
let _8: f64;
let _9: (bool, u128, &'static Adt34, i64);
let _10: f32;
let _11: &'static i8;
let _12: char;
let _13: isize;
let _14: f64;
let _15: [i8; 6];
let _16: bool;
let _17: bool;
let _18: u8;
let _19: [i8; 5];
let _20: i16;
let _21: isize;
let _22: isize;
let _23: isize;
let _24: &'static (&'static u128,);
let _25: [u16; 1];
let _26: [u16; 3];
let _27: isize;
let _28: char;
let _29: ();
let _30: ();
{
RET = 471969973_u32 as isize;
_2 = [3872792371_u32,3394545879_u32,310369601_u32,3461740708_u32,488062540_u32,3603464486_u32,1148659620_u32,1512582304_u32];
Call(_9.3 = fn5(_2, _2, _6, _1, _2, _6, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _6;
_8 = _4 as f64;
_3 = [1269918265_u32,1101314073_u32,4129362527_u32,4147737856_u32,2729758959_u32,1424903042_u32,3404750576_u32,3584288672_u32];
_9.0 = RET < _6;
_4 = _5;
RET = 89714206_i32 as isize;
_5 = !_4;
_4 = _5;
_6 = RET & RET;
_6 = RET + RET;
_1 = _3;
_12 = '\u{44673}';
_4 = _5 + _5;
_10 = _4 as f32;
_13 = _6 | RET;
_9.3 = (-1001251918082856167_i64);
_9.0 = _7;
_13 = RET;
_12 = '\u{9b097}';
RET = _6;
_7 = !_9.0;
_8 = 1783144475_i32 as f64;
_12 = '\u{e617a}';
Goto(bb2)
}
bb2 = {
_9.1 = _4 << _4;
_2 = _3;
_14 = _8 * _8;
_8 = _14 * _14;
_9.0 = _7;
_15 = [(-114_i8),108_i8,(-59_i8),81_i8,(-71_i8),91_i8];
_9.3 = (-8268525939138287230_i64) ^ 2893676008982402964_i64;
_13 = RET;
_9.0 = _9.1 >= _4;
_17 = _9.0;
_9.0 = !_17;
_16 = _17 | _9.0;
_9.0 = _10 < _10;
_16 = _9.0;
_9.0 = _16 > _7;
_16 = _9.0 != _9.0;
RET = _6 << _9.1;
_5 = _9.1;
Goto(bb3)
}
bb3 = {
_8 = -_14;
_1 = [4057762411_u32,1290314780_u32,3831014016_u32,3743314127_u32,3643535654_u32,1962250231_u32,1971702623_u32,4032548404_u32];
Goto(bb4)
}
bb4 = {
_19 = [55_i8,80_i8,(-58_i8),(-93_i8),(-63_i8)];
Goto(bb5)
}
bb5 = {
_13 = RET;
_9.3 = (-3216226440177271125_i64) + (-5266196602718172256_i64);
RET = -_13;
_20 = !21172_i16;
_18 = 104_u8;
_20 = _9.0 as i16;
_19 = [(-23_i8),72_i8,(-69_i8),(-34_i8),105_i8];
_20 = 2357869342_u32 as i16;
_19 = [(-57_i8),(-118_i8),(-89_i8),123_i8,71_i8];
_7 = _16;
_12 = '\u{f28bb}';
_9.0 = !_17;
_22 = _20 as isize;
_22 = _14 as isize;
RET = -_13;
_7 = _10 > _10;
_4 = _5 << _6;
_13 = RET - RET;
_1 = [1761372734_u32,1510626777_u32,2921449218_u32,2669236192_u32,2902689564_u32,1520434683_u32,2156506022_u32,1259985271_u32];
_21 = _13;
_5 = !_9.1;
_5 = _9.3 as u128;
_5 = _4 * _9.1;
_5 = 36983_u16 as u128;
_13 = RET;
_23 = _13;
Goto(bb6)
}
bb6 = {
_13 = RET;
_3 = [1583054794_u32,178543783_u32,3391993531_u32,3968724600_u32,3830675434_u32,203587665_u32,2138121352_u32,1205264094_u32];
_12 = '\u{47c29}';
_8 = _14;
_21 = !_23;
_5 = _4 >> _4;
_13 = _23 & _6;
match _18 {
0 => bb4,
1 => bb3,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
104 => bb13,
_ => bb12
}
}
bb7 = {
_13 = RET;
_9.3 = (-3216226440177271125_i64) + (-5266196602718172256_i64);
RET = -_13;
_20 = !21172_i16;
_18 = 104_u8;
_20 = _9.0 as i16;
_19 = [(-23_i8),72_i8,(-69_i8),(-34_i8),105_i8];
_20 = 2357869342_u32 as i16;
_19 = [(-57_i8),(-118_i8),(-89_i8),123_i8,71_i8];
_7 = _16;
_12 = '\u{f28bb}';
_9.0 = !_17;
_22 = _20 as isize;
_22 = _14 as isize;
RET = -_13;
_7 = _10 > _10;
_4 = _5 << _6;
_13 = RET - RET;
_1 = [1761372734_u32,1510626777_u32,2921449218_u32,2669236192_u32,2902689564_u32,1520434683_u32,2156506022_u32,1259985271_u32];
_21 = _13;
_5 = !_9.1;
_5 = _9.3 as u128;
_5 = _4 * _9.1;
_5 = 36983_u16 as u128;
_13 = RET;
_23 = _13;
Goto(bb6)
}
bb8 = {
_19 = [55_i8,80_i8,(-58_i8),(-93_i8),(-63_i8)];
Goto(bb5)
}
bb9 = {
_8 = -_14;
_1 = [4057762411_u32,1290314780_u32,3831014016_u32,3743314127_u32,3643535654_u32,1962250231_u32,1971702623_u32,4032548404_u32];
Goto(bb4)
}
bb10 = {
_9.1 = _4 << _4;
_2 = _3;
_14 = _8 * _8;
_8 = _14 * _14;
_9.0 = _7;
_15 = [(-114_i8),108_i8,(-59_i8),81_i8,(-71_i8),91_i8];
_9.3 = (-8268525939138287230_i64) ^ 2893676008982402964_i64;
_13 = RET;
_9.0 = _9.1 >= _4;
_17 = _9.0;
_9.0 = !_17;
_16 = _17 | _9.0;
_9.0 = _10 < _10;
_16 = _9.0;
_9.0 = _16 > _7;
_16 = _9.0 != _9.0;
RET = _6 << _9.1;
_5 = _9.1;
Goto(bb3)
}
bb11 = {
RET = _6;
_8 = _4 as f64;
_3 = [1269918265_u32,1101314073_u32,4129362527_u32,4147737856_u32,2729758959_u32,1424903042_u32,3404750576_u32,3584288672_u32];
_9.0 = RET < _6;
_4 = _5;
RET = 89714206_i32 as isize;
_5 = !_4;
_4 = _5;
_6 = RET & RET;
_6 = RET + RET;
_1 = _3;
_12 = '\u{44673}';
_4 = _5 + _5;
_10 = _4 as f32;
_13 = _6 | RET;
_9.3 = (-1001251918082856167_i64);
_9.0 = _7;
_13 = RET;
_12 = '\u{9b097}';
RET = _6;
_7 = !_9.0;
_8 = 1783144475_i32 as f64;
_12 = '\u{e617a}';
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
_22 = _13;
match _18 {
0 => bb8,
1 => bb2,
2 => bb11,
3 => bb9,
4 => bb5,
5 => bb6,
6 => bb10,
104 => bb15,
_ => bb14
}
}
bb14 = {
_8 = -_14;
_1 = [4057762411_u32,1290314780_u32,3831014016_u32,3743314127_u32,3643535654_u32,1962250231_u32,1971702623_u32,4032548404_u32];
Goto(bb4)
}
bb15 = {
_5 = !_4;
_21 = _13 + RET;
_7 = !_17;
_1 = _3;
_3 = [731023483_u32,1264020192_u32,284149802_u32,3966639263_u32,4278626665_u32,3846684400_u32,106545161_u32,2029632110_u32];
_1 = _2;
_5 = _9.1 * _9.1;
_1 = _2;
_16 = !_17;
_9.3 = 2005084314451045006_i64 - 2562703552042877426_i64;
_9.1 = !_5;
_13 = _21 + _23;
_22 = _21 - _13;
RET = _22;
RET = _13;
Goto(bb16)
}
bb16 = {
Call(_29 = dump_var(4_usize, 21_usize, Move(_21), 4_usize, Move(_4), 2_usize, Move(_2), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(4_usize, 17_usize, Move(_17), 20_usize, Move(_20), 1_usize, Move(_1), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(4_usize, 16_usize, Move(_16), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [u32; 8],mut _2: [u32; 8],mut _3: isize,mut _4: [u32; 8],mut _5: [u32; 8],mut _6: isize,mut _7: [u32; 8]) -> i64 {
mir! {
type RET = i64;
let _8: bool;
let _9: Adt68;
let _10: ([u8; 1], i32, (u64, [u16; 3], f64, u8));
let _11: *const bool;
let _12: usize;
let _13: f32;
let _14: (i128, u8, f64, i32);
let _15: &'static Adt34;
let _16: *const u32;
let _17: isize;
let _18: f64;
let _19: f64;
let _20: [char; 1];
let _21: (u64, [u16; 3], f64, u8);
let _22: isize;
let _23: [i8; 5];
let _24: char;
let _25: isize;
let _26: i128;
let _27: ();
let _28: ();
{
_4 = [254114333_u32,2060151338_u32,252242670_u32,375724902_u32,3777330374_u32,1286947862_u32,806078208_u32,3393845179_u32];
_6 = _3 - _3;
Goto(bb1)
}
bb1 = {
_5 = [2073943086_u32,1157847908_u32,952100478_u32,2913881261_u32,4135755111_u32,2489995170_u32,2405640423_u32,751653522_u32];
_4 = _2;
RET = 5131884804379274695_i64 ^ (-3101650716784681142_i64);
_3 = !_6;
_4 = _2;
_6 = _3 - _3;
RET = (-3157976654186824452_i64) + 3346519267376212965_i64;
_4 = [2554810826_u32,867268846_u32,1134012155_u32,1057057511_u32,463122530_u32,4087809321_u32,688504325_u32,1062698927_u32];
_10.2.1 = [5873_u16,43674_u16,38812_u16];
_3 = _6;
_3 = (-46_i8) as isize;
_3 = !_6;
_10.2.0 = 3599536327534173618_usize as u64;
_4 = [1451379930_u32,607730854_u32,3095059465_u32,48351197_u32,2113644185_u32,3895754230_u32,4033579403_u32,1358699283_u32];
_8 = !false;
_10.2.2 = 3605567020_u32 as f64;
_10.2.3 = 107_u8 & 232_u8;
_11 = core::ptr::addr_of!(_8);
_13 = _3 as f32;
_1 = _5;
_10.2.1 = [44555_u16,28919_u16,16016_u16];
_10.2.2 = (-19849_i16) as f64;
Goto(bb2)
}
bb2 = {
_8 = !false;
_14.0 = (-10650_i16) as i128;
_10.1 = 436001563_i32;
_11 = core::ptr::addr_of!((*_11));
_14.3 = 269017046084283035546365029174108544434_u128 as i32;
RET = 3524155779848605891_i64;
(*_11) = false;
_3 = 14735_u16 as isize;
match _10.1 {
0 => bb3,
1 => bb4,
2 => bb5,
436001563 => bb7,
_ => bb6
}
}
bb3 = {
_5 = [2073943086_u32,1157847908_u32,952100478_u32,2913881261_u32,4135755111_u32,2489995170_u32,2405640423_u32,751653522_u32];
_4 = _2;
RET = 5131884804379274695_i64 ^ (-3101650716784681142_i64);
_3 = !_6;
_4 = _2;
_6 = _3 - _3;
RET = (-3157976654186824452_i64) + 3346519267376212965_i64;
_4 = [2554810826_u32,867268846_u32,1134012155_u32,1057057511_u32,463122530_u32,4087809321_u32,688504325_u32,1062698927_u32];
_10.2.1 = [5873_u16,43674_u16,38812_u16];
_3 = _6;
_3 = (-46_i8) as isize;
_3 = !_6;
_10.2.0 = 3599536327534173618_usize as u64;
_4 = [1451379930_u32,607730854_u32,3095059465_u32,48351197_u32,2113644185_u32,3895754230_u32,4033579403_u32,1358699283_u32];
_8 = !false;
_10.2.2 = 3605567020_u32 as f64;
_10.2.3 = 107_u8 & 232_u8;
_11 = core::ptr::addr_of!(_8);
_13 = _3 as f32;
_1 = _5;
_10.2.1 = [44555_u16,28919_u16,16016_u16];
_10.2.2 = (-19849_i16) as f64;
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
_14 = (71136963018264424339396368129569951638_i128, _10.2.3, _10.2.2, _10.1);
_17 = -_6;
_10.1 = _10.2.0 as i32;
_13 = 2179892286_u32 as f32;
_11 = core::ptr::addr_of!((*_11));
_10.2.0 = 4833207771721479518_u64;
_13 = (-88_i8) as f32;
_10.2.2 = _14.2;
_10.0 = [_10.2.3];
RET = !(-4928077460082254902_i64);
_6 = 45124013560705148873204937246439116582_u128 as isize;
(*_11) = !true;
_1 = [194714815_u32,1166712440_u32,2152106460_u32,1603501591_u32,1708994882_u32,3636337870_u32,2686250228_u32,3580616053_u32];
_18 = 26092_i16 as f64;
RET = (-212745563109510106_i64);
_10.2.0 = (*_11) as u64;
_17 = _6 << _14.1;
_12 = 4_usize;
_14.2 = _17 as f64;
_2[_12] = _7[_12];
_12 = 14215258036974560783_usize * 3_usize;
RET = (-4668372204359162363_i64) >> _10.2.0;
_1 = _7;
match _14.3 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb9,
4 => bb10,
436001563 => bb12,
_ => bb11
}
}
bb8 = {
Return()
}
bb9 = {
_8 = !false;
_14.0 = (-10650_i16) as i128;
_10.1 = 436001563_i32;
_11 = core::ptr::addr_of!((*_11));
_14.3 = 269017046084283035546365029174108544434_u128 as i32;
RET = 3524155779848605891_i64;
(*_11) = false;
_3 = 14735_u16 as isize;
match _10.1 {
0 => bb3,
1 => bb4,
2 => bb5,
436001563 => bb7,
_ => bb6
}
}
bb10 = {
_5 = [2073943086_u32,1157847908_u32,952100478_u32,2913881261_u32,4135755111_u32,2489995170_u32,2405640423_u32,751653522_u32];
_4 = _2;
RET = 5131884804379274695_i64 ^ (-3101650716784681142_i64);
_3 = !_6;
_4 = _2;
_6 = _3 - _3;
RET = (-3157976654186824452_i64) + 3346519267376212965_i64;
_4 = [2554810826_u32,867268846_u32,1134012155_u32,1057057511_u32,463122530_u32,4087809321_u32,688504325_u32,1062698927_u32];
_10.2.1 = [5873_u16,43674_u16,38812_u16];
_3 = _6;
_3 = (-46_i8) as isize;
_3 = !_6;
_10.2.0 = 3599536327534173618_usize as u64;
_4 = [1451379930_u32,607730854_u32,3095059465_u32,48351197_u32,2113644185_u32,3895754230_u32,4033579403_u32,1358699283_u32];
_8 = !false;
_10.2.2 = 3605567020_u32 as f64;
_10.2.3 = 107_u8 & 232_u8;
_11 = core::ptr::addr_of!(_8);
_13 = _3 as f32;
_1 = _5;
_10.2.1 = [44555_u16,28919_u16,16016_u16];
_10.2.2 = (-19849_i16) as f64;
Goto(bb2)
}
bb11 = {
_5 = [2073943086_u32,1157847908_u32,952100478_u32,2913881261_u32,4135755111_u32,2489995170_u32,2405640423_u32,751653522_u32];
_4 = _2;
RET = 5131884804379274695_i64 ^ (-3101650716784681142_i64);
_3 = !_6;
_4 = _2;
_6 = _3 - _3;
RET = (-3157976654186824452_i64) + 3346519267376212965_i64;
_4 = [2554810826_u32,867268846_u32,1134012155_u32,1057057511_u32,463122530_u32,4087809321_u32,688504325_u32,1062698927_u32];
_10.2.1 = [5873_u16,43674_u16,38812_u16];
_3 = _6;
_3 = (-46_i8) as isize;
_3 = !_6;
_10.2.0 = 3599536327534173618_usize as u64;
_4 = [1451379930_u32,607730854_u32,3095059465_u32,48351197_u32,2113644185_u32,3895754230_u32,4033579403_u32,1358699283_u32];
_8 = !false;
_10.2.2 = 3605567020_u32 as f64;
_10.2.3 = 107_u8 & 232_u8;
_11 = core::ptr::addr_of!(_8);
_13 = _3 as f32;
_1 = _5;
_10.2.1 = [44555_u16,28919_u16,16016_u16];
_10.2.2 = (-19849_i16) as f64;
Goto(bb2)
}
bb12 = {
_5 = [3603059849_u32,2374787529_u32,3179620073_u32,298558850_u32,1829870616_u32,721442966_u32,662454974_u32,503908906_u32];
_17 = !_3;
_20 = ['\u{5bd38}'];
_19 = _14.2;
_10.0 = [_10.2.3];
_4 = [3469519343_u32,4052081596_u32,860014205_u32,1210054774_u32,1344271967_u32,1388365834_u32,1176476526_u32,3331548528_u32];
_12 = 12814756209936325518_usize >> _10.1;
_10.2.1 = [53650_u16,31682_u16,17071_u16];
_14 = (104321698689124870235882994159346929429_i128, _10.2.3, _19, _10.1);
_21.3 = _12 as u8;
_20 = ['\u{a5fc8}'];
_21 = (_10.2.0, _10.2.1, _18, _10.2.3);
_1 = _7;
_8 = false;
_2 = [710142765_u32,2326200632_u32,3523029651_u32,23476978_u32,3246543188_u32,3080015626_u32,3063815840_u32,3971688865_u32];
_8 = _14.1 != _10.2.3;
_3 = _17;
_2 = [3219971539_u32,2136466969_u32,903590066_u32,2471515381_u32,721289489_u32,3103624364_u32,2165582441_u32,2657339747_u32];
_17 = _3;
Call(_21.3 = fn6(_8, Move(_11), _19, _7, _7, _14.0, _8, _21.2, RET, _7), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_6 = !_17;
_6 = _3 << _10.2.0;
_4 = _5;
_3 = _17 * _6;
_14.2 = _21.3 as f64;
_10.2.2 = _19;
_14.3 = 45734_u16 as i32;
_19 = 28554_i16 as f64;
RET = 2260021007910099837_i64;
_11 = core::ptr::addr_of!(_8);
_5 = [1147604563_u32,472670838_u32,1180357184_u32,834724320_u32,2099799706_u32,3408718971_u32,3962309122_u32,3040398427_u32];
_2 = _1;
_14 = ((-90734401352542539023665656652695249768_i128), _10.2.3, _18, _10.1);
_20 = ['\u{bacea}'];
RET = !6450872091818039899_i64;
_10.2 = (_21.0, _21.1, _19, _14.1);
_13 = 2656014949_u32 as f32;
_17 = -_3;
(*_11) = _10.2.3 <= _10.2.3;
_2 = [187900814_u32,3566063474_u32,2372690753_u32,2062876133_u32,2653931634_u32,407983867_u32,2514175846_u32,3972268566_u32];
_10.2.2 = _18;
_26 = !_14.0;
_10.2.0 = _8 as u64;
Goto(bb14)
}
bb14 = {
(*_11) = !false;
_18 = _10.2.0 as f64;
RET = (-2782798808124387334_i64);
_14 = (_26, _21.3, _18, _10.1);
_4 = _5;
(*_11) = !false;
_14 = (_26, _21.3, _10.2.2, _10.1);
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(5_usize, 3_usize, Move(_3), 2_usize, Move(_2), 5_usize, Move(_5), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(5_usize, 7_usize, Move(_7), 26_usize, Move(_26), 28_usize, _28, 28_usize, _28), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: bool,mut _2: *const bool,mut _3: f64,mut _4: [u32; 8],mut _5: [u32; 8],mut _6: i128,mut _7: bool,mut _8: f64,mut _9: i64,mut _10: [u32; 8]) -> u8 {
mir! {
type RET = u8;
let _11: i32;
let _12: i8;
let _13: (u64, [u16; 3], f64, u8);
let _14: [u16; 1];
let _15: bool;
let _16: &'static isize;
let _17: bool;
let _18: Adt81;
let _19: Adt81;
let _20: bool;
let _21: [char; 1];
let _22: Adt60;
let _23: isize;
let _24: f64;
let _25: [u8; 1];
let _26: char;
let _27: (&'static (char, u128, i32),);
let _28: u8;
let _29: u8;
let _30: &'static &'static &'static u32;
let _31: *mut Adt42;
let _32: f32;
let _33: *const u32;
let _34: f32;
let _35: isize;
let _36: Adt34;
let _37: ();
let _38: ();
{
_7 = _1;
_2 = core::ptr::addr_of!(_1);
_3 = _8 - _8;
_3 = _8;
RET = !112_u8;
Goto(bb1)
}
bb1 = {
RET = 69_u8;
RET = _6 as u8;
_12 = -(-68_i8);
match _6 {
0 => bb2,
1 => bb3,
104321698689124870235882994159346929429 => bb5,
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
_5 = [3092714342_u32,816635622_u32,2905671629_u32,3564859269_u32,2858282813_u32,2471539981_u32,3470913380_u32,3164191252_u32];
_11 = 3626_u16 as i32;
(*_2) = !_7;
_13.2 = _3;
_13.1 = [6429_u16,55535_u16,6547_u16];
_10 = [2452339981_u32,60654736_u32,4259301525_u32,3416812100_u32,791674245_u32,493595513_u32,1534942977_u32,4050320695_u32];
_2 = core::ptr::addr_of!((*_2));
_8 = _13.2 - _13.2;
_13.0 = !8187625855692951006_u64;
_14 = [48836_u16];
_7 = (*_2);
_5 = [2259699628_u32,3067245475_u32,931406496_u32,1732047069_u32,3078891325_u32,3500154636_u32,3954667107_u32,4215305513_u32];
_6 = (-45405078006281714279675415657764259404_i128) >> RET;
_12 = !119_i8;
_2 = core::ptr::addr_of!(_1);
_14 = [237_u16];
(*_2) = !_7;
_15 = (*_2) & _1;
(*_2) = !_7;
_13.1 = [17288_u16,61331_u16,50537_u16];
RET = _11 as u8;
_13.0 = !16784553641575526595_u64;
Call(RET = core::intrinsics::transmute(_15), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_3 = _13.2 + _8;
_13.0 = !16076342532040313808_u64;
_14 = [28585_u16];
_13.1 = [60379_u16,36655_u16,33728_u16];
_17 = (*_2) >= _7;
_7 = _17 | _15;
RET = !76_u8;
_15 = _1;
_18 = Adt81::Variant1 { fld0: _14 };
_20 = !_7;
_15 = (*_2);
_4 = [3041639658_u32,1238605183_u32,1683313723_u32,4216489251_u32,1289549259_u32,940159837_u32,2931792815_u32,4215822069_u32];
RET = 5_usize as u8;
Call(_3 = fn7(_13.1, _17, _20, _10, _20, _6, (*_2), _1, _6, (*_2), _13.1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
(*_2) = _17 >= _20;
_7 = _1;
_22.fld4 = !5506_i16;
_1 = !_15;
_16 = &_23;
_3 = -_8;
_22.fld1 = _13.0 >> _13.0;
Goto(bb8)
}
bb8 = {
place!(Field::<[u16; 1]>(Variant(_18, 1), 0)) = _14;
_21 = ['\u{7649a}'];
_13.3 = RET;
_22.fld2 = (-9223372036854775808_isize);
_22.fld5 = [_22.fld2,_22.fld2,_22.fld2,_22.fld2,_22.fld2,_22.fld2,_22.fld2];
_6 = RET as i128;
_25 = [RET];
_13.3 = _22.fld4 as u8;
_21 = ['\u{c29a4}'];
_15 = _7;
_3 = _8;
_3 = _8 - _13.2;
_12 = 67_i8;
_13.2 = _3 * _3;
_3 = -_13.2;
_10 = _5;
_21 = ['\u{7b91a}'];
_22.fld2 = (-58_isize) - 9223372036854775807_isize;
place!(Field::<[u16; 1]>(Variant(_18, 1), 0)) = [9519_u16];
_22.fld1 = !_13.0;
_22.fld5 = [_22.fld2,_22.fld2,_22.fld2,_22.fld2,_22.fld2,_22.fld2,_22.fld2];
(*_2) = _3 != _3;
_10 = [1841662395_u32,2092267560_u32,402153257_u32,10265697_u32,1629400051_u32,1154413621_u32,4138892301_u32,3157304156_u32];
Goto(bb9)
}
bb9 = {
_22.fld4 = 6118504806699940861_usize as i16;
_11 = 836678973_i32 * 1185609560_i32;
_16 = &(*_16);
_26 = '\u{ea447}';
_2 = core::ptr::addr_of!(_1);
_10 = [2753519719_u32,976914659_u32,3564567391_u32,2669955638_u32,3180073671_u32,3707033796_u32,1166654672_u32,2311819295_u32];
_13.2 = -_8;
match _12 {
0 => bb6,
67 => bb11,
_ => bb10
}
}
bb10 = {
place!(Field::<[u16; 1]>(Variant(_18, 1), 0)) = _14;
_21 = ['\u{7649a}'];
_13.3 = RET;
_22.fld2 = (-9223372036854775808_isize);
_22.fld5 = [_22.fld2,_22.fld2,_22.fld2,_22.fld2,_22.fld2,_22.fld2,_22.fld2];
_6 = RET as i128;
_25 = [RET];
_13.3 = _22.fld4 as u8;
_21 = ['\u{c29a4}'];
_15 = _7;
_3 = _8;
_3 = _8 - _13.2;
_12 = 67_i8;
_13.2 = _3 * _3;
_3 = -_13.2;
_10 = _5;
_21 = ['\u{7b91a}'];
_22.fld2 = (-58_isize) - 9223372036854775807_isize;
place!(Field::<[u16; 1]>(Variant(_18, 1), 0)) = [9519_u16];
_22.fld1 = !_13.0;
_22.fld5 = [_22.fld2,_22.fld2,_22.fld2,_22.fld2,_22.fld2,_22.fld2,_22.fld2];
(*_2) = _3 != _3;
_10 = [1841662395_u32,2092267560_u32,402153257_u32,10265697_u32,1629400051_u32,1154413621_u32,4138892301_u32,3157304156_u32];
Goto(bb9)
}
bb11 = {
_22.fld1 = _13.0 + _13.0;
_1 = _15;
_22.fld5 = [_22.fld2,_22.fld2,_22.fld2,_22.fld2,_22.fld2,_22.fld2,_22.fld2];
_9 = (-476687296185240142_i64) & (-1390560666681689641_i64);
_3 = _8 + _13.2;
_13.0 = !_22.fld1;
_12 = (-80_i8);
_16 = &_23;
_20 = !_1;
_19 = Move(_18);
_29 = !RET;
_28 = _13.3;
_10 = _4;
_8 = _3 - _3;
_7 = _15 == _15;
_17 = (*_2) | _20;
_4 = [4207036340_u32,819021249_u32,2820937285_u32,3630746674_u32,3791773218_u32,3337532675_u32,3260002063_u32,2919807110_u32];
_22.fld4 = -22163_i16;
_22.fld1 = _13.0;
_13.3 = !RET;
Goto(bb12)
}
bb12 = {
RET = _28 + _28;
_17 = !_7;
_7 = _15;
_13.0 = _22.fld1 + _22.fld1;
_16 = &_23;
_17 = !_7;
_5 = [2735438020_u32,1320362601_u32,4088857009_u32,943389858_u32,1247386235_u32,3060212237_u32,3902782949_u32,509532621_u32];
_21 = [_26];
_25 = [_13.3];
_11 = (-391101431_i32) | (-568715232_i32);
_26 = '\u{95754}';
_14 = Field::<[u16; 1]>(Variant(_19, 1), 0);
_20 = _17 <= (*_2);
_22.fld3 = core::ptr::addr_of!(_32);
_28 = _29;
_17 = !_15;
_32 = RET as f32;
_26 = '\u{9a9a9}';
_22.fld3 = core::ptr::addr_of!(_32);
_8 = _3 + _13.2;
Call(_23 = core::intrinsics::bswap(_22.fld2), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
RET = 38619657644291341012737846882676720932_u128 as u8;
_13.3 = RET;
_11 = _22.fld4 as i32;
_3 = 166726757680141662572148931271041998067_u128 as f64;
_22.fld0 = _6 as u32;
_7 = _15 > _17;
_22.fld4 = (-13274_i16) << _22.fld1;
_22.fld3 = core::ptr::addr_of!(_34);
_6 = 153094896263404369625543298552026266296_i128;
_13.2 = -_3;
place!(Field::<[u16; 1]>(Variant(_19, 1), 0)) = [42065_u16];
_9 = _26 as i64;
_22.fld5 = [_22.fld2,_22.fld2,_22.fld2,_22.fld2,_22.fld2,_22.fld2,_22.fld2];
_28 = !_13.3;
_25 = [_13.3];
_5 = [_22.fld0,_22.fld0,_22.fld0,_22.fld0,_22.fld0,_22.fld0,_22.fld0,_22.fld0];
_18 = Adt81::Variant1 { fld0: _14 };
_29 = _22.fld2 as u8;
RET = _29 ^ _29;
_22.fld2 = _28 as isize;
_3 = _13.2;
SetDiscriminant(_19, 0);
place!(Field::<i64>(Variant(_19, 0), 6)) = _9 - _9;
place!(Field::<Adt42>(Variant(_19, 0), 3)).fld2 = 32723_u16 as isize;
place!(Field::<*mut isize>(Variant(_19, 0), 2)) = core::ptr::addr_of_mut!(_35);
_24 = _22.fld0 as f64;
match _6 {
153094896263404369625543298552026266296 => bb14,
_ => bb10
}
}
bb14 = {
_22.fld2 = Field::<Adt42>(Variant(_19, 0), 3).fld2 ^ Field::<Adt42>(Variant(_19, 0), 3).fld2;
place!(Field::<Adt42>(Variant(_19, 0), 3)).fld3 = (_6, _28, _3, _11);
_15 = _20;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(6_usize, 20_usize, Move(_20), 15_usize, Move(_15), 25_usize, Move(_25), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(6_usize, 5_usize, Move(_5), 12_usize, Move(_12), 21_usize, Move(_21), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(6_usize, 6_usize, Move(_6), 38_usize, _38, 38_usize, _38, 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: [u16; 3],mut _2: bool,mut _3: bool,mut _4: [u32; 8],mut _5: bool,mut _6: i128,mut _7: bool,mut _8: bool,mut _9: i128,mut _10: bool,mut _11: [u16; 3]) -> f64 {
mir! {
type RET = f64;
let _12: ([u8; 1], i32, (u64, [u16; 3], f64, u8));
let _13: isize;
let _14: [char; 1];
let _15: isize;
let _16: Adt42;
let _17: [u8; 1];
let _18: [u16; 1];
let _19: isize;
let _20: &'static [i8; 6];
let _21: char;
let _22: ([u8; 1], i32, (u64, [u16; 3], f64, u8));
let _23: &'static &'static u32;
let _24: usize;
let _25: bool;
let _26: &'static (char, u128, i32);
let _27: i32;
let _28: &'static isize;
let _29: Adt60;
let _30: [char; 1];
let _31: (char, u128, i32);
let _32: Adt42;
let _33: f64;
let _34: char;
let _35: i64;
let _36: &'static [i8; 6];
let _37: (bool, u128, &'static Adt34, i64);
let _38: &'static [i8; 6];
let _39: [i8; 6];
let _40: [u128; 3];
let _41: ();
let _42: ();
{
_4 = [4146165378_u32,1838398703_u32,733273095_u32,3384165094_u32,58035027_u32,2938084455_u32,3768340038_u32,1052219578_u32];
RET = 9223372036854775807_isize as f64;
RET = 9223372036854775807_isize as f64;
_11 = [60189_u16,665_u16,11823_u16];
_11 = [41777_u16,1170_u16,31309_u16];
_2 = !_3;
_12.2.0 = 18176_u16 as u64;
_7 = _8;
_2 = !_5;
Goto(bb1)
}
bb1 = {
_16.fld5 = (-9223372036854775808_isize) as i32;
_9 = _6 | _6;
_16.fld3.1 = !209_u8;
_6 = -_9;
_12.0 = [_16.fld3.1];
_2 = _8;
_6 = RET as i128;
_12.2.1 = [1558_u16,36710_u16,60909_u16];
_12.1 = -_16.fld5;
_16.fld3.2 = RET + RET;
_9 = _6;
_12.2.0 = 15231249864599195116_u64 | 10506241229288901375_u64;
_16.fld3.2 = -RET;
_17 = _12.0;
_16.fld3.1 = 83_u8;
_12.2.3 = 4_usize as u8;
_21 = '\u{6303d}';
RET = _16.fld3.2 - _16.fld3.2;
_22.2.3 = 21693_u16 as u8;
_16.fld0 = _12.2.0 * _12.2.0;
_13 = 9223372036854775807_isize;
Goto(bb2)
}
bb2 = {
RET = -_16.fld3.2;
_15 = -_13;
RET = -_16.fld3.2;
RET = -_16.fld3.2;
_22.2 = (_16.fld0, _1, _16.fld3.2, _16.fld3.1);
_16.fld2 = _13 | _13;
_11 = [10533_u16,35919_u16,18204_u16];
_4 = [3567633609_u32,1077232920_u32,73475019_u32,1931675955_u32,709133320_u32,4043121150_u32,1405853138_u32,1950062662_u32];
_17 = [_12.2.3];
match _16.fld3.1 {
0 => bb3,
1 => bb4,
2 => bb5,
83 => bb7,
_ => bb6
}
}
bb3 = {
_16.fld5 = (-9223372036854775808_isize) as i32;
_9 = _6 | _6;
_16.fld3.1 = !209_u8;
_6 = -_9;
_12.0 = [_16.fld3.1];
_2 = _8;
_6 = RET as i128;
_12.2.1 = [1558_u16,36710_u16,60909_u16];
_12.1 = -_16.fld5;
_16.fld3.2 = RET + RET;
_9 = _6;
_12.2.0 = 15231249864599195116_u64 | 10506241229288901375_u64;
_16.fld3.2 = -RET;
_17 = _12.0;
_16.fld3.1 = 83_u8;
_12.2.3 = 4_usize as u8;
_21 = '\u{6303d}';
RET = _16.fld3.2 - _16.fld3.2;
_22.2.3 = 21693_u16 as u8;
_16.fld0 = _12.2.0 * _12.2.0;
_13 = 9223372036854775807_isize;
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
_22.0 = [_22.2.3];
Call(_12 = fn8(_4, _7), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_22.1 = _16.fld5;
_12.2.3 = _22.2.3 >> _16.fld0;
_18 = [39173_u16];
_10 = _5 > _3;
RET = -_12.2.2;
_16.fld1 = -RET;
_19 = _12.2.3 as isize;
_22.2 = (_12.2.0, _1, _12.2.2, _16.fld3.1);
_9 = _6;
_21 = '\u{5c39e}';
_2 = _3;
_15 = _21 as isize;
_7 = _3;
_10 = _2;
_24 = 13608284738605285467_usize * 0_usize;
_6 = _9 << _12.2.3;
_16.fld3.0 = _6 + _9;
_14 = [_21];
_12.0 = [_22.2.3];
_5 = !_7;
_12.2.0 = _6 as u64;
_24 = _12.1 as usize;
_22.0 = _17;
Call(_12.2.3 = core::intrinsics::bswap(_22.2.3), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_12.2.1 = [26936_u16,21434_u16,34599_u16];
Goto(bb10)
}
bb10 = {
_2 = _3;
_16.fld3.2 = _22.2.0 as f64;
Call(RET = core::intrinsics::fmaf64(_16.fld3.2, _16.fld3.2, _16.fld3.2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_5 = _7;
_16.fld0 = !_22.2.0;
_16.fld3 = (_6, _22.2.3, _12.2.2, _16.fld5);
_14 = [_21];
_14 = [_21];
_16.fld3.2 = _12.2.0 as f64;
Goto(bb12)
}
bb12 = {
_17 = [_16.fld3.1];
_19 = _13;
_9 = _16.fld3.0;
_30 = _14;
_15 = 104_i8 as isize;
Goto(bb13)
}
bb13 = {
RET = _16.fld3.0 as f64;
_26 = &_31;
_7 = !_5;
_32 = Adt42 { fld0: _16.fld0,fld1: _16.fld3.2,fld2: _16.fld2,fld3: _16.fld3,fld4: (-14016_i16),fld5: _22.1 };
_6 = _9;
_16.fld2 = _32.fld5 as isize;
_13 = _7 as isize;
_29.fld5 = [_13,_13,_16.fld2,_13,_13,_13,_13];
_33 = -_16.fld3.2;
_29.fld2 = _13 | _13;
_28 = &_29.fld2;
_31 = (_21, 175394592011496431568848237478640413671_u128, _12.1);
_9 = !_32.fld3.0;
_21 = _31.0;
_32.fld3.1 = _12.2.3;
RET = _32.fld1;
_16.fld1 = _12.2.2 - _22.2.2;
_16.fld3.1 = _32.fld3.1 - _12.2.3;
match _32.fld4 {
0 => bb11,
1 => bb12,
2 => bb3,
3 => bb6,
4 => bb7,
5 => bb14,
6 => bb15,
340282366920938463463374607431768197440 => bb17,
_ => bb16
}
}
bb14 = {
_22.0 = [_22.2.3];
Call(_12 = fn8(_4, _7), ReturnTo(bb8), UnwindUnreachable())
}
bb15 = {
Return()
}
bb16 = {
_2 = _3;
_16.fld3.2 = _22.2.0 as f64;
Call(RET = core::intrinsics::fmaf64(_16.fld3.2, _16.fld3.2, _16.fld3.2), ReturnTo(bb11), UnwindUnreachable())
}
bb17 = {
_16.fld3.2 = _16.fld3.0 as f64;
_5 = !_7;
_32.fld2 = !_13;
_28 = &(*_28);
_28 = &_32.fld2;
_16.fld3.2 = (-5887707391387855255_i64) as f64;
_31 = (_21, 337338165926667430732606486250049936650_u128, _12.1);
_28 = &(*_28);
_5 = !_2;
_20 = &_39;
_26 = &_31;
_12.0 = [_32.fld3.1];
Goto(bb18)
}
bb18 = {
Call(_41 = dump_var(7_usize, 6_usize, Move(_6), 7_usize, Move(_7), 1_usize, Move(_1), 10_usize, Move(_10)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_41 = dump_var(7_usize, 15_usize, Move(_15), 2_usize, Move(_2), 14_usize, Move(_14), 13_usize, Move(_13)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_41 = dump_var(7_usize, 21_usize, Move(_21), 3_usize, Move(_3), 42_usize, _42, 42_usize, _42), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: [u32; 8],mut _2: bool) -> ([u8; 1], i32, (u64, [u16; 3], f64, u8)) {
mir! {
type RET = ([u8; 1], i32, (u64, [u16; 3], f64, u8));
let _3: char;
let _4: isize;
let _5: bool;
let _6: [u16; 3];
let _7: &'static (&'static u128,);
let _8: char;
let _9: (i128, u8, f64, i32);
let _10: [isize; 7];
let _11: isize;
let _12: u64;
let _13: &'static Adt34;
let _14: (&'static u32,);
let _15: &'static Adt34;
let _16: (Adt60,);
let _17: &'static &'static i8;
let _18: [u128; 3];
let _19: [u16; 1];
let _20: (&'static u32,);
let _21: ();
let _22: ();
{
RET.0 = [208_u8];
RET.2.3 = _2 as u8;
RET.2.1 = [62595_u16,30559_u16,53091_u16];
_3 = '\u{309d5}';
RET.2.2 = (-9223372036854775808_isize) as f64;
RET.1 = (-503449644_i32) + 1892049259_i32;
RET.1 = !(-1860848482_i32);
RET.2.0 = 6915444846776863237_u64;
_3 = '\u{5fcba}';
RET.1 = 302862100897341576_i64 as i32;
_2 = true;
RET.2.2 = 9491388218561850752_usize as f64;
RET.2.1 = [10713_u16,32623_u16,44817_u16];
Call(RET.2.2 = fn9(RET.0, _1, _1, RET.2.3, RET.2.3, RET.2.1, RET.1, _3, RET.2.3, RET.2.1, RET.2.3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = [2082995611_u32,873795651_u32,2057987470_u32,3726422768_u32,3925697580_u32,1430013729_u32,3614598015_u32,3795259807_u32];
RET.1 = (-139442346_i32);
RET.2.2 = 1854272969_u32 as f64;
RET.2.1 = [16809_u16,56385_u16,18473_u16];
RET.2.3 = !222_u8;
_4 = (-18_isize);
RET.1 = 1205062548_i32;
RET.2.0 = !2528534403949339129_u64;
RET.2.1 = [47337_u16,9497_u16,34592_u16];
RET.2.1 = [42684_u16,56393_u16,61501_u16];
RET.2.0 = !15536271153576358909_u64;
_2 = true;
RET.1 = (-669686804_i32);
RET.2.0 = !17886041881557212557_u64;
_3 = '\u{42a4d}';
Goto(bb2)
}
bb2 = {
_4 = 21_isize | 93_isize;
RET.2.0 = !17372569450273435617_u64;
RET.2.2 = 3356259845532773975_i64 as f64;
RET.1 = 816045984_i32 ^ 1596014609_i32;
_5 = _2 ^ _2;
RET.2.2 = 73963122926629263931670119872496814213_u128 as f64;
RET.2.3 = 6_u8;
RET.2.1 = [27994_u16,24343_u16,17760_u16];
RET.2.3 = 206_u8 | 206_u8;
RET.2.1 = [10321_u16,5953_u16,50284_u16];
Goto(bb3)
}
bb3 = {
_1 = [3539650980_u32,3436052871_u32,1147382333_u32,2124698190_u32,40929960_u32,3650458552_u32,1776188361_u32,4282659673_u32];
_4 = (-9223372036854775808_isize);
RET.2.3 = 62_u8 & 1_u8;
RET.2.0 = 6580745952423045950_u64;
_10 = [_4,_4,_4,_4,_4,_4,_4];
_9.1 = !RET.2.3;
_9.0 = 423778607_u32 as i128;
RET.2.0 = 10859869488351135041_u64 * 15315908554094392057_u64;
_9.0 = 88437705512246252177279094604713658713_i128 + (-151869670126491678365521501218210532637_i128);
match _4 {
0 => bb1,
340282366920938463454151235394913435648 => bb5,
_ => bb4
}
}
bb4 = {
_4 = 21_isize | 93_isize;
RET.2.0 = !17372569450273435617_u64;
RET.2.2 = 3356259845532773975_i64 as f64;
RET.1 = 816045984_i32 ^ 1596014609_i32;
_5 = _2 ^ _2;
RET.2.2 = 73963122926629263931670119872496814213_u128 as f64;
RET.2.3 = 6_u8;
RET.2.1 = [27994_u16,24343_u16,17760_u16];
RET.2.3 = 206_u8 | 206_u8;
RET.2.1 = [10321_u16,5953_u16,50284_u16];
Goto(bb3)
}
bb5 = {
RET.1 = _9.0 as i32;
_9.1 = RET.2.3;
_4 = 9223372036854775807_isize - (-9223372036854775808_isize);
_4 = 87_isize << RET.1;
_11 = _4 & _4;
_9.0 = (-22960_i16) as i128;
_9.1 = RET.2.3 + RET.2.3;
_2 = !_5;
RET.2.2 = _9.0 as f64;
RET.2.0 = 15679189101259089975_u64;
RET.2.2 = (-2400851558222768200_i64) as f64;
Goto(bb6)
}
bb6 = {
_10 = [_11,_4,_11,_11,_11,_4,_11];
RET.1 = (-87624415_i32) | (-678268505_i32);
_9.2 = RET.2.2 * RET.2.2;
_11 = _4 | _4;
RET.0 = [_9.1];
RET.2.0 = 14719711052155355651_u64;
RET.2.2 = -_9.2;
RET.2.2 = _9.1 as f64;
_8 = _3;
_9.3 = RET.1 | RET.1;
_6 = RET.2.1;
RET.2.3 = _9.1;
Goto(bb7)
}
bb7 = {
_5 = _2;
_9.2 = RET.2.2;
_16.0.fld2 = _11;
RET.2.3 = _9.1;
RET.2.0 = 14191282202673532863_u64 ^ 7515362243341309204_u64;
RET.2.3 = _9.1 >> RET.1;
_11 = _4 >> _9.3;
RET.2.0 = 17618028606638476077_u64;
_9 = (92876878075361942717719911552723103416_i128, RET.2.3, RET.2.2, RET.1);
RET.2.2 = _9.2;
_6 = RET.2.1;
_16.0.fld5 = [_4,_11,_11,_16.0.fld2,_16.0.fld2,_11,_4];
_16.0.fld0 = RET.2.3 as u32;
RET.2 = (2402837466242685215_u64, _6, _9.2, _9.1);
_9.2 = RET.2.3 as f64;
RET.0 = [_9.1];
_9.3 = RET.1;
_1 = [_16.0.fld0,_16.0.fld0,_16.0.fld0,_16.0.fld0,_16.0.fld0,_16.0.fld0,_16.0.fld0,_16.0.fld0];
_19 = [42130_u16];
_5 = _2 | _2;
_12 = RET.2.0;
_4 = _9.1 as isize;
Goto(bb8)
}
bb8 = {
Call(_21 = dump_var(8_usize, 11_usize, Move(_11), 5_usize, Move(_5), 8_usize, Move(_8), 4_usize, Move(_4)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_21 = dump_var(8_usize, 19_usize, Move(_19), 22_usize, _22, 22_usize, _22, 22_usize, _22), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: [u8; 1],mut _2: [u32; 8],mut _3: [u32; 8],mut _4: u8,mut _5: u8,mut _6: [u16; 3],mut _7: i32,mut _8: char,mut _9: u8,mut _10: [u16; 3],mut _11: u8) -> f64 {
mir! {
type RET = f64;
let _12: [i8; 5];
let _13: isize;
let _14: (&'static isize,);
let _15: [char; 1];
let _16: bool;
let _17: isize;
let _18: bool;
let _19: [u32; 8];
let _20: Adt42;
let _21: [i8; 6];
let _22: i128;
let _23: i16;
let _24: &'static i32;
let _25: [char; 7];
let _26: (i128, Adt60);
let _27: Adt78;
let _28: (u32,);
let _29: &'static u32;
let _30: [i8; 5];
let _31: f64;
let _32: [u128; 3];
let _33: *mut isize;
let _34: Adt81;
let _35: ();
let _36: ();
{
_9 = 2147_i16 as u8;
Call(_10 = fn10(_3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 3431136453155972918_u64 as f64;
RET = 26697_u16 as f64;
_6 = [40719_u16,10993_u16,1728_u16];
_5 = (-9223372036854775808_isize) as u8;
_5 = _4;
_12 = [(-10_i8),(-87_i8),103_i8,(-114_i8),56_i8];
_9 = 39382_u16 as u8;
RET = (-694_i16) as f64;
_9 = !_11;
RET = 309923015819085257717493024362455446731_u128 as f64;
_10 = _6;
_13 = 36190_u16 as isize;
_1 = [_9];
_14.0 = &_13;
_11 = _9;
_11 = _9;
_14.0 = &_13;
_13 = _7 as isize;
_14.0 = &_13;
_13 = 9223372036854775807_isize + 9223372036854775807_isize;
_14.0 = &_13;
_1 = [_11];
Goto(bb2)
}
bb2 = {
_7 = true as i32;
_12 = [(-112_i8),(-109_i8),14_i8,6_i8,17_i8];
_8 = '\u{9706c}';
_12 = [(-42_i8),(-118_i8),(-74_i8),(-15_i8),(-114_i8)];
_15 = [_8];
_2 = [2798471554_u32,1417920719_u32,497730833_u32,3704503468_u32,382269786_u32,962741834_u32,2268794588_u32,1285555205_u32];
Goto(bb3)
}
bb3 = {
_1 = [_4];
_2 = [323720386_u32,2290857500_u32,3581976259_u32,3593388363_u32,401673173_u32,3404758930_u32,2085478572_u32,1444849718_u32];
_11 = _9 * _4;
_9 = 2741977527620176772_i64 as u8;
_13 = !9223372036854775807_isize;
_20.fld3.1 = _7 as u8;
_20.fld2 = 18096503032553306593_u64 as isize;
_15 = [_8];
_20.fld5 = 2148266416_u32 as i32;
_14.0 = &_17;
Goto(bb4)
}
bb4 = {
_1 = [_11];
_16 = false | true;
_20.fld0 = _8 as u64;
RET = _11 as f64;
_11 = _4;
_6 = [61133_u16,56245_u16,50034_u16];
_18 = !_16;
_24 = &_20.fld3.3;
_23 = 23387_i16;
_10 = [64366_u16,46376_u16,30018_u16];
_6 = _10;
_21 = [(-86_i8),77_i8,21_i8,(-83_i8),(-109_i8),(-82_i8)];
_22 = (-156682399845200571635224492709066858697_i128);
_12 = [51_i8,77_i8,57_i8,(-6_i8),73_i8];
_20.fld3.2 = -RET;
_20.fld3.3 = !_7;
_20.fld3.0 = _22 << _11;
_13 = _20.fld2 & _20.fld2;
_12 = [110_i8,83_i8,88_i8,(-27_i8),(-27_i8)];
_20.fld3.1 = _11 >> _20.fld3.0;
_7 = _20.fld5 + _20.fld5;
_25 = [_8,_8,_8,_8,_8,_8,_8];
_21 = [52_i8,(-55_i8),(-83_i8),(-29_i8),(-110_i8),48_i8];
match _23 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
23387 => bb9,
_ => bb8
}
}
bb5 = {
_1 = [_4];
_2 = [323720386_u32,2290857500_u32,3581976259_u32,3593388363_u32,401673173_u32,3404758930_u32,2085478572_u32,1444849718_u32];
_11 = _9 * _4;
_9 = 2741977527620176772_i64 as u8;
_13 = !9223372036854775807_isize;
_20.fld3.1 = _7 as u8;
_20.fld2 = 18096503032553306593_u64 as isize;
_15 = [_8];
_20.fld5 = 2148266416_u32 as i32;
_14.0 = &_17;
Goto(bb4)
}
bb6 = {
_7 = true as i32;
_12 = [(-112_i8),(-109_i8),14_i8,6_i8,17_i8];
_8 = '\u{9706c}';
_12 = [(-42_i8),(-118_i8),(-74_i8),(-15_i8),(-114_i8)];
_15 = [_8];
_2 = [2798471554_u32,1417920719_u32,497730833_u32,3704503468_u32,382269786_u32,962741834_u32,2268794588_u32,1285555205_u32];
Goto(bb3)
}
bb7 = {
RET = 3431136453155972918_u64 as f64;
RET = 26697_u16 as f64;
_6 = [40719_u16,10993_u16,1728_u16];
_5 = (-9223372036854775808_isize) as u8;
_5 = _4;
_12 = [(-10_i8),(-87_i8),103_i8,(-114_i8),56_i8];
_9 = 39382_u16 as u8;
RET = (-694_i16) as f64;
_9 = !_11;
RET = 309923015819085257717493024362455446731_u128 as f64;
_10 = _6;
_13 = 36190_u16 as isize;
_1 = [_9];
_14.0 = &_13;
_11 = _9;
_11 = _9;
_14.0 = &_13;
_13 = _7 as isize;
_14.0 = &_13;
_13 = 9223372036854775807_isize + 9223372036854775807_isize;
_14.0 = &_13;
_1 = [_11];
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_9 = !_20.fld3.1;
_22 = _20.fld3.0;
_24 = &_20.fld5;
_20.fld4 = _8 as i16;
RET = _9 as f64;
match _23 {
0 => bb8,
1 => bb10,
2 => bb11,
23387 => bb13,
_ => bb12
}
}
bb10 = {
Return()
}
bb11 = {
_1 = [_4];
_2 = [323720386_u32,2290857500_u32,3581976259_u32,3593388363_u32,401673173_u32,3404758930_u32,2085478572_u32,1444849718_u32];
_11 = _9 * _4;
_9 = 2741977527620176772_i64 as u8;
_13 = !9223372036854775807_isize;
_20.fld3.1 = _7 as u8;
_20.fld2 = 18096503032553306593_u64 as isize;
_15 = [_8];
_20.fld5 = 2148266416_u32 as i32;
_14.0 = &_17;
Goto(bb4)
}
bb12 = {
_7 = true as i32;
_12 = [(-112_i8),(-109_i8),14_i8,6_i8,17_i8];
_8 = '\u{9706c}';
_12 = [(-42_i8),(-118_i8),(-74_i8),(-15_i8),(-114_i8)];
_15 = [_8];
_2 = [2798471554_u32,1417920719_u32,497730833_u32,3704503468_u32,382269786_u32,962741834_u32,2268794588_u32,1285555205_u32];
Goto(bb3)
}
bb13 = {
_13 = _20.fld2;
_3 = _2;
_20.fld3.0 = 12957_u16 as i128;
_26.0 = 4_usize as i128;
_11 = !_9;
_20.fld3.2 = 325632808325750360172940691336194261673_u128 as f64;
_27.fld4 = _10;
_26.1.fld0 = 3344791971_u32;
_27.fld3.2.0 = 143947705734366728837640630785591587433_u128 as u64;
Goto(bb14)
}
bb14 = {
_16 = !_18;
_26.1.fld4 = -_20.fld4;
_27.fld5 = (*_24);
_20.fld3 = (_22, _4, RET, _7);
_27.fld1 = core::ptr::addr_of_mut!(_20.fld4);
_20.fld4 = _23;
_26.1.fld4 = -_20.fld4;
_18 = _4 > _9;
_20.fld3.1 = (*_24) as u8;
_27.fld3.1 = _7;
_19 = _3;
_20.fld3 = (_22, _11, RET, _20.fld5);
_13 = _20.fld2;
_3 = _19;
_27.fld2 = _20.fld0;
_3 = [_26.1.fld0,_26.1.fld0,_26.1.fld0,_26.1.fld0,_26.1.fld0,_26.1.fld0,_26.1.fld0,_26.1.fld0];
_26.1.fld1 = _27.fld2 - _27.fld3.2.0;
_27.fld3.2.1 = _10;
_20.fld3.3 = _26.1.fld0 as i32;
_6 = [33663_u16,52719_u16,40218_u16];
_31 = RET;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(9_usize, 23_usize, Move(_23), 12_usize, Move(_12), 21_usize, Move(_21), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(9_usize, 18_usize, Move(_18), 8_usize, Move(_8), 22_usize, Move(_22), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(9_usize, 7_usize, Move(_7), 19_usize, Move(_19), 36_usize, _36, 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: [u32; 8]) -> [u16; 3] {
mir! {
type RET = [u16; 3];
let _2: (&'static (char, u128, i32),);
let _3: char;
let _4: (u64, [u16; 3], f64, u8);
let _5: *const bool;
let _6: *const f32;
let _7: [u8; 1];
let _8: f32;
let _9: usize;
let _10: Adt64;
let _11: bool;
let _12: &'static &'static u32;
let _13: f64;
let _14: &'static [i8; 6];
let _15: &'static Adt34;
let _16: u8;
let _17: isize;
let _18: [u16; 1];
let _19: u8;
let _20: isize;
let _21: u32;
let _22: (bool, u128);
let _23: (i128, u8, f64, i32);
let _24: u16;
let _25: f32;
let _26: ();
let _27: ();
{
RET = [44533_u16,56803_u16,24233_u16];
RET = [58944_u16,62665_u16,52758_u16];
RET = [17588_u16,47029_u16,32354_u16];
RET = [22716_u16,32681_u16,31412_u16];
_1 = [244062493_u32,1732287520_u32,1682625458_u32,465831338_u32,1110065360_u32,1882809788_u32,1113381958_u32,715386678_u32];
RET = [11498_u16,24481_u16,14748_u16];
_1 = [2381990945_u32,3886252190_u32,380149598_u32,605872439_u32,512385782_u32,3029296094_u32,4164726417_u32,3637139033_u32];
RET = [23870_u16,25635_u16,24197_u16];
_1 = [3307452373_u32,625072872_u32,1039917352_u32,3663354463_u32,870283494_u32,668430681_u32,1104096521_u32,1743162919_u32];
RET = [63142_u16,62262_u16,12046_u16];
RET = [21521_u16,47464_u16,49545_u16];
RET = [1425_u16,5068_u16,53550_u16];
RET = [34542_u16,19841_u16,26915_u16];
RET = [13291_u16,3951_u16,18529_u16];
_1 = [3462892275_u32,524487761_u32,3646057697_u32,4294603583_u32,178000079_u32,2130922430_u32,2574418604_u32,3647463109_u32];
_1 = [4071251084_u32,1677767847_u32,2905274614_u32,3065158149_u32,3674271049_u32,3582220685_u32,349682992_u32,3202209765_u32];
RET = [31201_u16,61528_u16,22837_u16];
Goto(bb1)
}
bb1 = {
RET = [61539_u16,25644_u16,60155_u16];
_4.3 = 28_u8;
_1 = [4012005540_u32,2226034312_u32,4230318772_u32,1811581472_u32,2717945911_u32,280719346_u32,751014173_u32,1302371581_u32];
_1 = [3817791129_u32,1524126460_u32,1855630329_u32,2576402525_u32,2085476084_u32,2920361107_u32,2017435462_u32,4087246518_u32];
_4.0 = 14810712737846826942_u64 << _4.3;
_4.3 = !78_u8;
_4.2 = _4.0 as f64;
_4.1 = [57837_u16,15483_u16,53119_u16];
_4.3 = 111_u8;
_4.2 = _4.0 as f64;
RET = _4.1;
_3 = '\u{b1813}';
RET = _4.1;
_3 = '\u{a2034}';
_3 = '\u{b303b}';
_4.0 = 15605871877115974743_u64 + 3464273828359946705_u64;
_4.3 = !206_u8;
Goto(bb2)
}
bb2 = {
_4.1 = [40700_u16,53363_u16,9636_u16];
_4.1 = RET;
_4.3 = 114_u8;
_1 = [2366499451_u32,3564757795_u32,2563732326_u32,3757950563_u32,3842596070_u32,1187614738_u32,2209454686_u32,1409840489_u32];
_4.0 = (-338728509_i32) as u64;
_4.3 = 126_u8;
_4.1 = RET;
_4.2 = 3863476938_u32 as f64;
_4.3 = !103_u8;
_4.0 = (-35_i8) as u64;
RET = [17291_u16,54374_u16,2168_u16];
_1 = [2709271197_u32,1165268271_u32,2347338903_u32,700738714_u32,3267079055_u32,3839057171_u32,2441563500_u32,2152422617_u32];
_4.1 = RET;
_7 = [_4.3];
_4.3 = 18_i8 as u8;
_1 = [3414849486_u32,2324763706_u32,3496994595_u32,3862370261_u32,2383159995_u32,3935713185_u32,2452992386_u32,4207787783_u32];
_3 = '\u{10b701}';
_7 = [_4.3];
_1 = [3784954635_u32,3864419518_u32,4207297861_u32,1947650916_u32,2195843147_u32,3668079841_u32,2478407235_u32,2978182817_u32];
_8 = 157995519275190327937164530445648562964_u128 as f32;
_6 = core::ptr::addr_of!(_8);
(*_6) = 1757128592_u32 as f32;
Call((*_6) = fn11(_4.1, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = [_4.3];
_6 = core::ptr::addr_of!((*_6));
_4.1 = RET;
Call(_9 = core::intrinsics::bswap(17308620678332476373_usize), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_6 = core::ptr::addr_of!(_8);
_3 = '\u{813ef}';
_4.2 = (-86629233_i32) as f64;
_5 = core::ptr::addr_of!(_11);
_7 = [_4.3];
_13 = 2208422454743949459_usize as f64;
(*_5) = true;
RET = [7331_u16,16415_u16,49919_u16];
_4.1 = [26876_u16,60546_u16,57411_u16];
_8 = _4.3 as f32;
_1 = [911528252_u32,1648747436_u32,842639355_u32,3696342666_u32,1338643371_u32,878334895_u32,1094358638_u32,1496575468_u32];
(*_5) = !true;
(*_5) = !true;
Goto(bb5)
}
bb5 = {
_5 = core::ptr::addr_of!(_11);
_4 = (12798255509646981844_u64, RET, _13, 81_u8);
_17 = 9223372036854775807_isize;
_6 = core::ptr::addr_of!((*_6));
_8 = _17 as f32;
match _4.0 {
12798255509646981844 => bb6,
_ => bb3
}
}
bb6 = {
_3 = '\u{306ba}';
_11 = false ^ false;
_4.2 = _13 * _13;
_4.3 = 11_u8 ^ 183_u8;
_5 = core::ptr::addr_of!((*_5));
_4.3 = 20105_i16 as u8;
_1 = [2304019300_u32,3129969405_u32,1797923593_u32,3171839684_u32,509603622_u32,615161791_u32,974524317_u32,2605891931_u32];
(*_6) = 4703328095287344634_i64 as f32;
_13 = _4.2 + _4.2;
_16 = !_4.3;
_4.2 = _13;
(*_6) = _4.0 as f32;
_4.2 = _13;
Goto(bb7)
}
bb7 = {
_4.3 = !_16;
_1 = [2355666399_u32,1088978875_u32,2369647283_u32,1157295339_u32,3577598322_u32,2039043846_u32,1873551398_u32,128187839_u32];
(*_6) = 2018899746_u32 as f32;
match _4.0 {
0 => bb4,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
12798255509646981844 => bb13,
_ => bb12
}
}
bb8 = {
_3 = '\u{306ba}';
_11 = false ^ false;
_4.2 = _13 * _13;
_4.3 = 11_u8 ^ 183_u8;
_5 = core::ptr::addr_of!((*_5));
_4.3 = 20105_i16 as u8;
_1 = [2304019300_u32,3129969405_u32,1797923593_u32,3171839684_u32,509603622_u32,615161791_u32,974524317_u32,2605891931_u32];
(*_6) = 4703328095287344634_i64 as f32;
_13 = _4.2 + _4.2;
_16 = !_4.3;
_4.2 = _13;
(*_6) = _4.0 as f32;
_4.2 = _13;
Goto(bb7)
}
bb9 = {
_5 = core::ptr::addr_of!(_11);
_4 = (12798255509646981844_u64, RET, _13, 81_u8);
_17 = 9223372036854775807_isize;
_6 = core::ptr::addr_of!((*_6));
_8 = _17 as f32;
match _4.0 {
12798255509646981844 => bb6,
_ => bb3
}
}
bb10 = {
_6 = core::ptr::addr_of!(_8);
_3 = '\u{813ef}';
_4.2 = (-86629233_i32) as f64;
_5 = core::ptr::addr_of!(_11);
_7 = [_4.3];
_13 = 2208422454743949459_usize as f64;
(*_5) = true;
RET = [7331_u16,16415_u16,49919_u16];
_4.1 = [26876_u16,60546_u16,57411_u16];
_8 = _4.3 as f32;
_1 = [911528252_u32,1648747436_u32,842639355_u32,3696342666_u32,1338643371_u32,878334895_u32,1094358638_u32,1496575468_u32];
(*_5) = !true;
(*_5) = !true;
Goto(bb5)
}
bb11 = {
_7 = [_4.3];
_6 = core::ptr::addr_of!((*_6));
_4.1 = RET;
Call(_9 = core::intrinsics::bswap(17308620678332476373_usize), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_4.1 = [40700_u16,53363_u16,9636_u16];
_4.1 = RET;
_4.3 = 114_u8;
_1 = [2366499451_u32,3564757795_u32,2563732326_u32,3757950563_u32,3842596070_u32,1187614738_u32,2209454686_u32,1409840489_u32];
_4.0 = (-338728509_i32) as u64;
_4.3 = 126_u8;
_4.1 = RET;
_4.2 = 3863476938_u32 as f64;
_4.3 = !103_u8;
_4.0 = (-35_i8) as u64;
RET = [17291_u16,54374_u16,2168_u16];
_1 = [2709271197_u32,1165268271_u32,2347338903_u32,700738714_u32,3267079055_u32,3839057171_u32,2441563500_u32,2152422617_u32];
_4.1 = RET;
_7 = [_4.3];
_4.3 = 18_i8 as u8;
_1 = [3414849486_u32,2324763706_u32,3496994595_u32,3862370261_u32,2383159995_u32,3935713185_u32,2452992386_u32,4207787783_u32];
_3 = '\u{10b701}';
_7 = [_4.3];
_1 = [3784954635_u32,3864419518_u32,4207297861_u32,1947650916_u32,2195843147_u32,3668079841_u32,2478407235_u32,2978182817_u32];
_8 = 157995519275190327937164530445648562964_u128 as f32;
_6 = core::ptr::addr_of!(_8);
(*_6) = 1757128592_u32 as f32;
Call((*_6) = fn11(_4.1, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
RET = _4.1;
_5 = core::ptr::addr_of!((*_5));
_18 = [19455_u16];
_17 = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
_1 = [3802808251_u32,2202534231_u32,3275902407_u32,29524624_u32,2090659493_u32,1237506143_u32,3591614806_u32,19645062_u32];
_1 = [832626893_u32,3515283441_u32,2988397359_u32,3389005836_u32,3739942805_u32,2575675853_u32,1052489731_u32,3356650330_u32];
_18 = [64863_u16];
_4.1 = [6708_u16,55437_u16,24598_u16];
(*_5) = _4.3 < _16;
_16 = (*_6) as u8;
_9 = 12343534550855533100_usize;
_7 = [_4.3];
(*_5) = _13 > _4.2;
_4.2 = -_13;
_18 = [52672_u16];
_19 = _16;
_9 = 1974550343181380202_usize + 5721938658040216157_usize;
_3 = '\u{48ba7}';
_4 = (14574672591825060977_u64, RET, _13, _19);
Goto(bb14)
}
bb14 = {
_22.1 = 306647853950127384679865056056413007314_u128;
RET = [44792_u16,5296_u16,5535_u16];
_22.0 = _11 <= _11;
RET = [25034_u16,31372_u16,57595_u16];
_8 = _16 as f32;
_23.3 = 313467189_i32 << _4.0;
_8 = _17 as f32;
_19 = _4.3 - _16;
_21 = !3659876829_u32;
(*_5) = !_22.0;
_5 = core::ptr::addr_of!(_11);
_23.1 = _4.3;
_23 = (53276170050003022136853919288780567468_i128, _4.3, _13, 91168308_i32);
_5 = core::ptr::addr_of!((*_5));
_23 = ((-90972959412259682504597410080781606074_i128), _4.3, _13, (-1967296407_i32));
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(10_usize, 7_usize, Move(_7), 1_usize, Move(_1), 19_usize, Move(_19), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(10_usize, 22_usize, Move(_22), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: [u16; 3],mut _2: [u32; 8]) -> f32 {
mir! {
type RET = f32;
let _3: i128;
let _4: u16;
let _5: (Adt60,);
let _6: (i128, Adt60);
let _7: u32;
let _8: &'static u32;
let _9: [isize; 7];
let _10: (&'static u32,);
let _11: bool;
let _12: char;
let _13: ([u8; 1], i32, (u64, [u16; 3], f64, u8));
let _14: char;
let _15: *mut &'static u32;
let _16: [u128; 3];
let _17: isize;
let _18: u8;
let _19: *const f32;
let _20: *mut usize;
let _21: f64;
let _22: usize;
let _23: f64;
let _24: *const f32;
let _25: u128;
let _26: u32;
let _27: u8;
let _28: f32;
let _29: [u8; 1];
let _30: ();
let _31: ();
{
RET = (-11_i8) as f32;
_1 = [43795_u16,21747_u16,27948_u16];
_2 = [693314052_u32,379813759_u32,3173985857_u32,701299628_u32,676847193_u32,3060705325_u32,105361918_u32,3516121430_u32];
_1 = [20150_u16,46466_u16,26575_u16];
RET = 142_u8 as f32;
_2 = [2258784920_u32,2758547554_u32,1581718334_u32,2317099586_u32,402844529_u32,3765004823_u32,272642011_u32,3579548441_u32];
_3 = 82016029552556778789505350835806805183_i128;
_1 = [49082_u16,37560_u16,41196_u16];
RET = 11354_u16 as f32;
_1 = [36706_u16,26826_u16,35496_u16];
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
82016029552556778789505350835806805183 => bb5,
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
_3 = -140901195545186983733801755215840416270_i128;
RET = 3080662398563740416_u64 as f32;
RET = 48281634028143901348331948385521761380_u128 as f32;
RET = 18_u8 as f32;
_3 = 153841426468109407756695411743485489970_i128;
_3 = 151557718482836551419314661023426771612_i128 + (-34377355428928839003172953577535864670_i128);
_5.0.fld0 = !2359525151_u32;
_2 = [_5.0.fld0,_5.0.fld0,_5.0.fld0,_5.0.fld0,_5.0.fld0,_5.0.fld0,_5.0.fld0,_5.0.fld0];
_6.1.fld1 = true as u64;
_5.0.fld2 = 9223372036854775807_isize & (-9223372036854775808_isize);
_7 = _5.0.fld0 - _5.0.fld0;
_6.1.fld3 = core::ptr::addr_of!(RET);
_7 = _5.0.fld0 * _5.0.fld0;
_6.1.fld5 = [_5.0.fld2,_5.0.fld2,_5.0.fld2,_5.0.fld2,_5.0.fld2,_5.0.fld2,_5.0.fld2];
_6.1.fld2 = _5.0.fld2;
_10.0 = &_7;
_5.0.fld5 = _6.1.fld5;
_5.0.fld1 = _6.1.fld1;
_5.0.fld1 = _6.1.fld1;
_6.1.fld0 = _5.0.fld0;
_6.1.fld4 = 0_usize as i16;
Call(_5.0.fld1 = core::intrinsics::bswap(_6.1.fld1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
RET = _3 as f32;
_6.0 = _3;
_4 = 24101_u16 << _6.1.fld2;
RET = _6.1.fld4 as f32;
_6.0 = _3;
_5.0.fld2 = _6.1.fld1 as isize;
_1 = [_4,_4,_4];
_5.0 = Move(_6.1);
_13.1 = _5.0.fld4 as i32;
_11 = false;
Goto(bb7)
}
bb7 = {
_7 = _5.0.fld0;
_2 = [_5.0.fld0,_7,_5.0.fld0,_7,_7,_5.0.fld0,_7,_7];
_13.0 = [129_u8];
_2 = [_5.0.fld0,_5.0.fld0,_7,_7,_5.0.fld0,_5.0.fld0,_7,_5.0.fld0];
_6.1.fld5 = [_5.0.fld2,_5.0.fld2,_5.0.fld2,_5.0.fld2,_5.0.fld2,_5.0.fld2,_5.0.fld2];
_3 = _6.0 * _6.0;
_13.2.2 = _5.0.fld4 as f64;
_6.1.fld1 = _5.0.fld1 - _5.0.fld1;
_13.0 = [131_u8];
_7 = _5.0.fld0;
_6 = (_3, Move(_5.0));
_5.0.fld3 = Move(_6.1.fld3);
_6.1.fld3 = core::ptr::addr_of!(RET);
_5.0 = Adt60 { fld0: _7,fld1: _6.1.fld1,fld2: _6.1.fld2,fld3: Move(_6.1.fld3),fld4: _6.1.fld4,fld5: _6.1.fld5 };
_13.1 = -114349813_i32;
_8 = &_5.0.fld0;
_14 = '\u{693c7}';
_7 = _6.1.fld0 ^ _6.1.fld0;
_6.1.fld1 = !_5.0.fld1;
_6 = (_3, Move(_5.0));
_14 = '\u{f8145}';
Call(_5.0.fld2 = fn12(Move(_6)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_6.0 = 217_u8 as i128;
RET = _6.0 as f32;
_5.0.fld1 = !14107255111052997247_u64;
RET = _13.2.2 as f32;
_6.1.fld3 = core::ptr::addr_of!(RET);
RET = 63_i8 as f32;
_1 = [_4,_4,_4];
_11 = _3 < _3;
_6.1.fld5 = [_5.0.fld2,_5.0.fld2,_5.0.fld2,_5.0.fld2,_5.0.fld2,_5.0.fld2,_5.0.fld2];
_1 = [_4,_4,_4];
_13.2.0 = !_5.0.fld1;
_6.1.fld0 = _7;
_1 = [_4,_4,_4];
_11 = false;
_15 = core::ptr::addr_of_mut!(_8);
_12 = _14;
_16 = [47260564927400224586563492489481311405_u128,260085171904940313833010791991747519865_u128,2068210036012593538600155143722911781_u128];
_13.2.3 = _13.2.2 as u8;
_5.0.fld4 = _13.1 as i16;
_5.0.fld0 = !_6.1.fld0;
_13.2.2 = _5.0.fld4 as f64;
_18 = !_13.2.3;
Goto(bb9)
}
bb9 = {
_17 = _14 as isize;
_13.2.1 = [_4,_4,_4];
_6.1.fld4 = _5.0.fld4;
_11 = !true;
_6.1.fld1 = _13.2.0 - _5.0.fld1;
_15 = core::ptr::addr_of_mut!((*_15));
_5.0.fld5 = [_5.0.fld2,_17,_5.0.fld2,_5.0.fld2,_17,_17,_5.0.fld2];
_5.0 = Adt60 { fld0: _6.1.fld0,fld1: _13.2.0,fld2: _17,fld3: Move(_6.1.fld3),fld4: _6.1.fld4,fld5: _6.1.fld5 };
(*_15) = &_5.0.fld0;
_13.2.0 = RET as u64;
_9 = [_17,_5.0.fld2,_5.0.fld2,_5.0.fld2,_17,_5.0.fld2,_17];
_8 = &(*_8);
_5.0.fld4 = !_6.1.fld4;
(*_15) = &(*_8);
_4 = 50755_u16 * 31602_u16;
_15 = core::ptr::addr_of_mut!(_8);
_13.2.1 = _1;
Goto(bb10)
}
bb10 = {
(*_15) = &_5.0.fld0;
_15 = core::ptr::addr_of_mut!(_10.0);
_13.2.3 = !_18;
_6.1.fld1 = !_5.0.fld1;
_6.0 = !_3;
_6.1.fld5 = [_17,_17,_5.0.fld2,_5.0.fld2,_17,_17,_5.0.fld2];
_22 = 0_usize;
_25 = _16[_22];
_5.0.fld0 = _2[_22] - _7;
_17 = _5.0.fld2 | _6.1.fld5[_22];
_6.1 = Adt60 { fld0: _5.0.fld0,fld1: _5.0.fld1,fld2: _17,fld3: Move(_5.0.fld3),fld4: _5.0.fld4,fld5: _9 };
_8 = &_5.0.fld0;
_9[_22] = _17 & _5.0.fld2;
(*_15) = &_7;
match _16[_22] {
47260564927400224586563492489481311405 => bb12,
_ => bb11
}
}
bb11 = {
_6.0 = 217_u8 as i128;
RET = _6.0 as f32;
_5.0.fld1 = !14107255111052997247_u64;
RET = _13.2.2 as f32;
_6.1.fld3 = core::ptr::addr_of!(RET);
RET = 63_i8 as f32;
_1 = [_4,_4,_4];
_11 = _3 < _3;
_6.1.fld5 = [_5.0.fld2,_5.0.fld2,_5.0.fld2,_5.0.fld2,_5.0.fld2,_5.0.fld2,_5.0.fld2];
_1 = [_4,_4,_4];
_13.2.0 = !_5.0.fld1;
_6.1.fld0 = _7;
_1 = [_4,_4,_4];
_11 = false;
_15 = core::ptr::addr_of_mut!(_8);
_12 = _14;
_16 = [47260564927400224586563492489481311405_u128,260085171904940313833010791991747519865_u128,2068210036012593538600155143722911781_u128];
_13.2.3 = _13.2.2 as u8;
_5.0.fld4 = _13.1 as i16;
_5.0.fld0 = !_6.1.fld0;
_13.2.2 = _5.0.fld4 as f64;
_18 = !_13.2.3;
Goto(bb9)
}
bb12 = {
_5.0.fld1 = _6.1.fld4 as u64;
_10.0 = &_2[_22];
_13.0[_22] = _13.2.3 * _13.2.3;
_13.2.1 = _1;
match _16[_22] {
0 => bb1,
1 => bb11,
2 => bb3,
3 => bb6,
4 => bb7,
47260564927400224586563492489481311405 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_1[_22] = _13.2.1[_22] + _4;
_2 = [_6.1.fld0,_6.1.fld0,_5.0.fld0,_7,_7,_6.1.fld0,_7,(*_8)];
_10.0 = Move(_8);
_24 = core::ptr::addr_of!(_28);
_29 = _13.0;
_16[_22] = _25;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(11_usize, 29_usize, Move(_29), 14_usize, Move(_14), 4_usize, Move(_4), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(11_usize, 7_usize, Move(_7), 9_usize, Move(_9), 18_usize, Move(_18), 31_usize, _31), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: (i128, Adt60)) -> isize {
mir! {
type RET = isize;
let _2: u16;
let _3: *const f32;
let _4: [i16; 2];
let _5: *const u32;
let _6: &'static i64;
let _7: &'static isize;
let _8: &'static isize;
let _9: i64;
let _10: &'static (&'static u128,);
let _11: [i16; 2];
let _12: (bool, u128, &'static Adt34, i64);
let _13: char;
let _14: *mut usize;
let _15: char;
let _16: ([u8; 1], (u32,), i16, &'static (&'static u128,));
let _17: Adt56;
let _18: &'static isize;
let _19: bool;
let _20: (char, u128, i32);
let _21: [i8; 6];
let _22: i128;
let _23: ();
let _24: ();
{
_1.1.fld0 = !693102071_u32;
RET = _1.1.fld2 << _1.1.fld2;
Goto(bb1)
}
bb1 = {
_1.1.fld0 = 1811699230_u32 >> _1.0;
_1.1.fld1 = 11680474863880028856_u64 >> _1.1.fld0;
RET = _1.1.fld2 * _1.1.fld2;
_2 = RET as u16;
RET = _1.1.fld2;
RET = _1.1.fld1 as isize;
_1.1.fld5 = [_1.1.fld2,RET,RET,_1.1.fld2,RET,RET,RET];
_1.1.fld5 = [RET,RET,_1.1.fld2,RET,RET,RET,_1.1.fld2];
_2 = 60556_u16 + 34999_u16;
_1.1.fld1 = !12087090151423755252_u64;
_1.0 = !125295580392128631465044556378513365259_i128;
RET = !_1.1.fld2;
_1.1.fld4 = _2 as i16;
RET = _1.1.fld2;
RET = _1.1.fld2;
_4 = [_1.1.fld4,_1.1.fld4];
Goto(bb2)
}
bb2 = {
_1.1.fld4 = (-3914575324158243944_i64) as i16;
_1.1.fld1 = 12818227862492880432_u64 | 10274559622779415864_u64;
_1.1.fld4 = (-15977_i16) << _1.1.fld0;
_1.1.fld0 = 3206495905_u32;
_1.0 = '\u{7cfbe}' as i128;
_1.1.fld4 = !(-19942_i16);
_5 = core::ptr::addr_of!(_1.1.fld0);
_5 = core::ptr::addr_of!(_1.1.fld0);
_4 = [_1.1.fld4,_1.1.fld4];
(*_5) = !2512043833_u32;
_5 = core::ptr::addr_of!(_1.1.fld0);
(*_5) = 2981647766_u32;
_7 = &RET;
_3 = Move(_1.1.fld3);
_2 = !48229_u16;
RET = -_1.1.fld2;
_1.0 = 157773690408982168638228996271169682198_i128;
match _1.0 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
157773690408982168638228996271169682198 => bb11,
_ => bb10
}
}
bb3 = {
_1.1.fld0 = 1811699230_u32 >> _1.0;
_1.1.fld1 = 11680474863880028856_u64 >> _1.1.fld0;
RET = _1.1.fld2 * _1.1.fld2;
_2 = RET as u16;
RET = _1.1.fld2;
RET = _1.1.fld1 as isize;
_1.1.fld5 = [_1.1.fld2,RET,RET,_1.1.fld2,RET,RET,RET];
_1.1.fld5 = [RET,RET,_1.1.fld2,RET,RET,RET,_1.1.fld2];
_2 = 60556_u16 + 34999_u16;
_1.1.fld1 = !12087090151423755252_u64;
_1.0 = !125295580392128631465044556378513365259_i128;
RET = !_1.1.fld2;
_1.1.fld4 = _2 as i16;
RET = _1.1.fld2;
RET = _1.1.fld2;
_4 = [_1.1.fld4,_1.1.fld4];
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
Return()
}
bb11 = {
_2 = 2440_u16;
(*_5) = _1.1.fld1 as u32;
(*_5) = 983446198_u32 + 1167891185_u32;
_1.0 = 70944740740610936414398154036675998528_i128;
_1.1.fld0 = 2027495399_i32 as u32;
_4 = [_1.1.fld4,_1.1.fld4];
_1.1.fld4 = 13751_i16;
_5 = core::ptr::addr_of!((*_5));
_1.1.fld5 = [RET,_1.1.fld2,RET,RET,RET,RET,RET];
_7 = &RET;
_1.1.fld4 = 519_i16 ^ 18759_i16;
_1.1.fld2 = (*_7);
(*_5) = !594965_u32;
_1.1.fld3 = Move(_3);
_8 = &(*_7);
RET = _1.0 as isize;
RET = _2 as isize;
_5 = core::ptr::addr_of!((*_5));
_4 = [_1.1.fld4,_1.1.fld4];
RET = !_1.1.fld2;
_7 = &RET;
_1.1.fld2 = 746950327_i32 as isize;
RET = _1.1.fld2;
Goto(bb12)
}
bb12 = {
_1.1.fld1 = 14989827590879854450_u64;
(*_5) = 2142631315_u32;
(*_5) = 3501422878_u32 << _2;
_8 = &RET;
RET = _1.0 as isize;
_1.1.fld5 = [_1.1.fld2,_1.1.fld2,_1.1.fld2,_1.1.fld2,_1.1.fld2,_1.1.fld2,RET];
_8 = &_1.1.fld2;
_7 = Move(_8);
_3 = Move(_1.1.fld3);
_12.1 = _1.1.fld4 as u128;
_1.1.fld3 = Move(_3);
_6 = &_9;
_1.1.fld0 = 3125008864_u32;
(*_5) = 1035261419_u32 * 855636580_u32;
_3 = Move(_1.1.fld3);
_11 = [_1.1.fld4,_1.1.fld4];
_2 = _12.1 as u16;
_5 = core::ptr::addr_of!(_1.1.fld0);
_11 = [_1.1.fld4,_1.1.fld4];
_1.1.fld3 = Move(_3);
_1.1.fld4 = (-11123_i16);
RET = _1.1.fld2;
_16.2 = 2357674883971000453_i64 as i16;
_1.0 = 150903039394951048429788523743622704591_i128;
RET = _1.1.fld2;
_13 = '\u{e17bb}';
Call(_3 = fn13(_1.1.fld2, _16.2, _2, Move(_5), _11, Move(_1), _12.1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_1.1.fld1 = 2800393973871321735_u64;
_12.3 = (-4697623117680331799_i64);
_8 = &RET;
_8 = &(*_8);
_7 = &(*_8);
_13 = '\u{be0fc}';
_16.0 = [245_u8];
_6 = &_12.3;
_9 = (*_6);
_16.1 = (555143061_u32,);
_1.1.fld0 = _1.1.fld1 as u32;
RET = _16.2 as isize;
_1.1.fld1 = !14827490773421874662_u64;
_7 = &_1.1.fld2;
_15 = _13;
RET = _12.3 as isize;
_12.0 = true;
Goto(bb14)
}
bb14 = {
_6 = &(*_6);
RET = !(-9223372036854775808_isize);
_5 = core::ptr::addr_of!(_1.1.fld0);
_18 = &(*_7);
_5 = core::ptr::addr_of!((*_5));
_16.2 = 6133_i16 & (-19313_i16);
_5 = core::ptr::addr_of!(_16.1.0);
_20 = (_13, _12.1, 335923982_i32);
_19 = _12.0;
_9 = (*_6) << (*_6);
RET = 89_i8 as isize;
_1.0 = !(-52024760415171233301182460139315382081_i128);
_20.0 = _13;
RET = _1.1.fld1 as isize;
_12.0 = _19;
_6 = &_12.3;
Goto(bb15)
}
bb15 = {
Call(_23 = dump_var(12_usize, 15_usize, Move(_15), 19_usize, Move(_19), 20_usize, Move(_20), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: isize,mut _2: i16,mut _3: u16,mut _4: *const u32,mut _5: [i16; 2],mut _6: (i128, Adt60),mut _7: u128) -> *const f32 {
mir! {
type RET = *const f32;
let _8: [char; 1];
let _9: isize;
let _10: (i128, u8, f64, i32);
let _11: (&'static u32,);
let _12: Adt42;
let _13: (&'static (char, u128, i32),);
let _14: i32;
let _15: i16;
let _16: bool;
let _17: i64;
let _18: (*const bool, i8, &'static &'static u32, Adt42);
let _19: ([u8; 1], i32, (u64, [u16; 3], f64, u8));
let _20: char;
let _21: (Adt60,);
let _22: (*const f32,);
let _23: &'static bool;
let _24: &'static u32;
let _25: isize;
let _26: [u16; 3];
let _27: isize;
let _28: char;
let _29: i128;
let _30: [i8; 5];
let _31: f64;
let _32: i32;
let _33: [u32; 8];
let _34: &'static i8;
let _35: &'static i8;
let _36: char;
let _37: *mut (bool, u128);
let _38: [isize; 7];
let _39: isize;
let _40: char;
let _41: f64;
let _42: [char; 1];
let _43: f32;
let _44: i128;
let _45: bool;
let _46: Adt60;
let _47: [char; 7];
let _48: i128;
let _49: (*mut Adt42, &'static u128, u16);
let _50: ();
let _51: ();
{
_2 = 4524091967830104826_usize as i16;
_5 = [_2,_2];
RET = Move(_6.1.fld3);
Call(_6.1 = fn14(_6.0, Move(RET), Move(_4), _3, _7, _1, _1, _6.0, _6.0, _3, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6.0 = 90586231015564198238899173382549236880_i128 | (-117779031185040643963947207952878556179_i128);
_2 = !_6.1.fld4;
_1 = _6.1.fld2;
_6.1.fld0 = 969767965_u32 & 3483955023_u32;
_8 = ['\u{5adec}'];
_7 = '\u{f3b1f}' as u128;
_6.0 = (-122439183682418649820425038687491122860_i128) << _6.1.fld0;
_7 = 112_u8 as u128;
_6.1.fld2 = _6.0 as isize;
RET = Move(_6.1.fld3);
_5 = [_2,_6.1.fld4];
Goto(bb2)
}
bb2 = {
_6.1.fld4 = '\u{77321}' as i16;
_5 = [_2,_2];
_7 = 5527050035357747931137691503318069001_u128 ^ 226883083184859627093802482572235283561_u128;
_6.1.fld0 = 69100967_u32 * 88489135_u32;
_6.0 = (-39782717407940722166807276091199886480_i128);
_7 = 186072117849908590993217460281697171513_u128;
_8 = ['\u{31a5b}'];
_6.1.fld1 = !13216091103657121733_u64;
_2 = _6.1.fld4 << _6.1.fld4;
_7 = 72773376708864086384305808255631859709_u128 ^ 248211562852629115276971738281036573761_u128;
match _6.0 {
0 => bb1,
1 => bb3,
300499649512997741296567331340568324976 => bb5,
_ => bb4
}
}
bb3 = {
_6.0 = 90586231015564198238899173382549236880_i128 | (-117779031185040643963947207952878556179_i128);
_2 = !_6.1.fld4;
_1 = _6.1.fld2;
_6.1.fld0 = 969767965_u32 & 3483955023_u32;
_8 = ['\u{5adec}'];
_7 = '\u{f3b1f}' as u128;
_6.0 = (-122439183682418649820425038687491122860_i128) << _6.1.fld0;
_7 = 112_u8 as u128;
_6.1.fld2 = _6.0 as isize;
RET = Move(_6.1.fld3);
_5 = [_2,_6.1.fld4];
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
_1 = -_6.1.fld2;
_8 = ['\u{726b1}'];
_6.1.fld3 = Move(RET);
_4 = core::ptr::addr_of!(_6.1.fld0);
RET = Move(_6.1.fld3);
_8 = ['\u{984e3}'];
_9 = _1 & _6.1.fld2;
_6.1.fld3 = Move(RET);
_3 = !6429_u16;
Goto(bb6)
}
bb6 = {
(*_4) = 1372215677_u32;
_9 = _6.1.fld2 - _6.1.fld2;
_8 = ['\u{4609b}'];
_10.0 = _6.0 >> _6.1.fld0;
_4 = core::ptr::addr_of!((*_4));
RET = Move(_6.1.fld3);
_12.fld0 = !_6.1.fld1;
_11.0 = &(*_4);
_8 = ['\u{926b5}'];
Goto(bb7)
}
bb7 = {
_3 = _6.1.fld2 as u16;
_6.1.fld3 = Move(RET);
_12.fld5 = '\u{100764}' as i32;
_10.2 = 2_usize as f64;
match _6.1.fld0 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb5,
4 => bb8,
1372215677 => bb10,
_ => bb9
}
}
bb8 = {
_6.0 = 90586231015564198238899173382549236880_i128 | (-117779031185040643963947207952878556179_i128);
_2 = !_6.1.fld4;
_1 = _6.1.fld2;
_6.1.fld0 = 969767965_u32 & 3483955023_u32;
_8 = ['\u{5adec}'];
_7 = '\u{f3b1f}' as u128;
_6.0 = (-122439183682418649820425038687491122860_i128) << _6.1.fld0;
_7 = 112_u8 as u128;
_6.1.fld2 = _6.0 as isize;
RET = Move(_6.1.fld3);
_5 = [_2,_6.1.fld4];
Goto(bb2)
}
bb9 = {
Return()
}
bb10 = {
(*_4) = 809982987_u32;
_9 = -_6.1.fld2;
_12.fld3.3 = _12.fld5;
_14 = '\u{f5055}' as i32;
_9 = 805036485489004264_i64 as isize;
_16 = true;
_18.3.fld3.1 = 76_u8;
_18.3.fld2 = _12.fld3.3 as isize;
_12.fld3.3 = _12.fld5;
_11.0 = &(*_4);
_18.3.fld3.2 = -_10.2;
_4 = core::ptr::addr_of!(_6.1.fld0);
_12.fld0 = !_6.1.fld1;
_18.3.fld4 = _2;
_19.0 = [_18.3.fld3.1];
_21 = (Move(_6.1),);
_20 = '\u{cdcc9}';
_19.0 = [_18.3.fld3.1];
_6.1 = Adt60 { fld0: _21.0.fld0,fld1: _12.fld0,fld2: _1,fld3: Move(_21.0.fld3),fld4: _2,fld5: _21.0.fld5 };
_19.1 = _12.fld5;
_19.2.3 = _18.3.fld3.1 >> _6.1.fld2;
_19.2.0 = _21.0.fld1 * _12.fld0;
_11.0 = &_6.1.fld0;
_21 = (Move(_6.1),);
Call(_21.0.fld2 = fn18(_18.3.fld3.1, _20, _21.0.fld0, _8, _21.0.fld1, _5, _19.0, Move(_21.0.fld3), _5, _21.0.fld0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_12.fld3.2 = _12.fld0 as f64;
_10.3 = _20 as i32;
_12.fld3.3 = _19.1 ^ _10.3;
Call(_18.3.fld1 = core::intrinsics::fmaf64(_12.fld3.2, _10.2, _12.fld3.2), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_6.1.fld5 = [_18.3.fld2,_18.3.fld2,_1,_1,_9,_1,_9];
_18.3.fld3.3 = _14 * _19.1;
_10.1 = _19.2.3 + _19.2.3;
_12.fld3.0 = -_6.0;
_12.fld3.1 = _10.1;
_8 = [_20];
_18.3.fld5 = _10.3 + _14;
_7 = 283047996649086844620456673407368364742_u128;
_12.fld4 = _2 + _2;
_10.0 = _6.0 >> _12.fld0;
_19.1 = _14;
_18.3 = Adt42 { fld0: _19.2.0,fld1: _10.2,fld2: _1,fld3: _10,fld4: _21.0.fld4,fld5: _12.fld5 };
_11.0 = &_6.1.fld0;
_18.3.fld0 = _21.0.fld1;
_23 = &_16;
_12.fld0 = _19.2.0;
_12.fld3.2 = 6_i8 as f64;
_24 = Move(_11.0);
_21.0.fld1 = _19.2.0 + _12.fld0;
_18.3 = Adt42 { fld0: _19.2.0,fld1: _10.2,fld2: _21.0.fld2,fld3: _12.fld3,fld4: _12.fld4,fld5: _14 };
_21.0.fld4 = _12.fld4 & _18.3.fld4;
_10.2 = -_18.3.fld3.2;
_18.3.fld2 = _12.fld3.0 as isize;
Call(_12.fld2 = core::intrinsics::transmute(_9), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_4 = core::ptr::addr_of!(_6.1.fld0);
_18.2 = &_11.0;
_25 = _21.0.fld4 as isize;
_7 = 186199214317207242499325827661813024597_u128;
_6.1.fld1 = 51_i8 as u64;
_9 = _25;
_12.fld3 = _10;
_18.3.fld3.2 = _18.3.fld1;
_18.3.fld3.0 = _10.1 as i128;
_12.fld5 = !_14;
_19.2.2 = -_18.3.fld3.2;
_18.0 = core::ptr::addr_of!(_16);
_12.fld1 = _25 as f64;
_18.3.fld3 = _10;
_18.3.fld1 = -_12.fld1;
_17 = (-6472767264704877056_i64) << _9;
_1 = !_25;
_15 = _1 as i16;
_26 = [_3,_3,_3];
_18.3.fld3 = (_12.fld3.0, _19.2.3, _12.fld1, _10.3);
_3 = 62455_u16;
_18.1 = 117_i8;
_19.2.0 = 5_usize as u64;
_28 = _20;
match _7 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
186199214317207242499325827661813024597 => bb19,
_ => bb18
}
}
bb14 = {
Return()
}
bb15 = {
_3 = _6.1.fld2 as u16;
_6.1.fld3 = Move(RET);
_12.fld5 = '\u{100764}' as i32;
_10.2 = 2_usize as f64;
match _6.1.fld0 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb5,
4 => bb8,
1372215677 => bb10,
_ => bb9
}
}
bb16 = {
(*_4) = 809982987_u32;
_9 = -_6.1.fld2;
_12.fld3.3 = _12.fld5;
_14 = '\u{f5055}' as i32;
_9 = 805036485489004264_i64 as isize;
_16 = true;
_18.3.fld3.1 = 76_u8;
_18.3.fld2 = _12.fld3.3 as isize;
_12.fld3.3 = _12.fld5;
_11.0 = &(*_4);
_18.3.fld3.2 = -_10.2;
_4 = core::ptr::addr_of!(_6.1.fld0);
_12.fld0 = !_6.1.fld1;
_18.3.fld4 = _2;
_19.0 = [_18.3.fld3.1];
_21 = (Move(_6.1),);
_20 = '\u{cdcc9}';
_19.0 = [_18.3.fld3.1];
_6.1 = Adt60 { fld0: _21.0.fld0,fld1: _12.fld0,fld2: _1,fld3: Move(_21.0.fld3),fld4: _2,fld5: _21.0.fld5 };
_19.1 = _12.fld5;
_19.2.3 = _18.3.fld3.1 >> _6.1.fld2;
_19.2.0 = _21.0.fld1 * _12.fld0;
_11.0 = &_6.1.fld0;
_21 = (Move(_6.1),);
Call(_21.0.fld2 = fn18(_18.3.fld3.1, _20, _21.0.fld0, _8, _21.0.fld1, _5, _19.0, Move(_21.0.fld3), _5, _21.0.fld0), ReturnTo(bb11), UnwindUnreachable())
}
bb17 = {
Return()
}
bb18 = {
_6.1.fld4 = '\u{77321}' as i16;
_5 = [_2,_2];
_7 = 5527050035357747931137691503318069001_u128 ^ 226883083184859627093802482572235283561_u128;
_6.1.fld0 = 69100967_u32 * 88489135_u32;
_6.0 = (-39782717407940722166807276091199886480_i128);
_7 = 186072117849908590993217460281697171513_u128;
_8 = ['\u{31a5b}'];
_6.1.fld1 = !13216091103657121733_u64;
_2 = _6.1.fld4 << _6.1.fld4;
_7 = 72773376708864086384305808255631859709_u128 ^ 248211562852629115276971738281036573761_u128;
match _6.0 {
0 => bb1,
1 => bb3,
300499649512997741296567331340568324976 => bb5,
_ => bb4
}
}
bb19 = {
_10.2 = _18.3.fld1;
_6.1.fld2 = -_1;
_21.0.fld2 = !_1;
_12.fld0 = !_19.2.0;
_18.3.fld3.3 = _14;
_18.3.fld3.1 = _19.2.3;
_21.0.fld0 = 1653728667_u32;
_19.0 = [_10.1];
_19.0 = [_12.fld3.1];
_5 = [_15,_18.3.fld4];
Call(_27 = core::intrinsics::transmute(_9), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
_18.3.fld4 = _3 as i16;
_19.2.1 = _26;
_11.0 = &(*_4);
_12.fld3.2 = -_18.3.fld1;
_5 = [_21.0.fld4,_21.0.fld4];
_26 = [_3,_3,_3];
_31 = 2_usize as f64;
_19.1 = _12.fld5 - _18.3.fld5;
_6.1.fld4 = !_15;
_12.fld0 = !_18.3.fld0;
_12.fld3.3 = _21.0.fld0 as i32;
_19.2.2 = -_12.fld3.2;
_21.0.fld4 = _18.1 as i16;
_23 = &(*_23);
_12.fld0 = _12.fld3.3 as u64;
_12.fld3.1 = _10.1 & _18.3.fld3.1;
_18.2 = &_11.0;
Goto(bb21)
}
bb21 = {
_34 = &_18.1;
_26 = _19.2.1;
_24 = Move(_11.0);
_18.1 = -(-101_i8);
_12.fld4 = _15 * _6.1.fld4;
_19.0 = [_12.fld3.1];
_19.2.0 = _21.0.fld1 | _18.3.fld0;
_26 = [_3,_3,_3];
_14 = !_10.3;
_11.0 = &_6.1.fld0;
_19.2.1 = [_3,_3,_3];
_18.3.fld2 = _27;
_21.0.fld1 = _19.2.0;
_10.2 = _10.0 as f64;
_26 = [_3,_3,_3];
_25 = _21.0.fld2 ^ _18.3.fld2;
_26 = [_3,_3,_3];
_6.1.fld0 = _21.0.fld0 & _21.0.fld0;
_18.1 = 27_i8 & 64_i8;
_18.3.fld5 = _12.fld3.3 * _10.3;
_21.0.fld4 = _12.fld3.2 as i16;
_11.0 = &_21.0.fld0;
_18.3.fld5 = _12.fld3.3 * _19.1;
Goto(bb22)
}
bb22 = {
_34 = &_18.1;
match _3 {
0 => bb23,
1 => bb24,
62455 => bb26,
_ => bb25
}
}
bb23 = {
_12.fld3.2 = _12.fld0 as f64;
_10.3 = _20 as i32;
_12.fld3.3 = _19.1 ^ _10.3;
Call(_18.3.fld1 = core::intrinsics::fmaf64(_12.fld3.2, _10.2, _12.fld3.2), ReturnTo(bb12), UnwindUnreachable())
}
bb24 = {
_6.0 = 90586231015564198238899173382549236880_i128 | (-117779031185040643963947207952878556179_i128);
_2 = !_6.1.fld4;
_1 = _6.1.fld2;
_6.1.fld0 = 969767965_u32 & 3483955023_u32;
_8 = ['\u{5adec}'];
_7 = '\u{f3b1f}' as u128;
_6.0 = (-122439183682418649820425038687491122860_i128) << _6.1.fld0;
_7 = 112_u8 as u128;
_6.1.fld2 = _6.0 as isize;
RET = Move(_6.1.fld3);
_5 = [_2,_6.1.fld4];
Goto(bb2)
}
bb25 = {
_1 = -_6.1.fld2;
_8 = ['\u{726b1}'];
_6.1.fld3 = Move(RET);
_4 = core::ptr::addr_of!(_6.1.fld0);
RET = Move(_6.1.fld3);
_8 = ['\u{984e3}'];
_9 = _1 & _6.1.fld2;
_6.1.fld3 = Move(RET);
_3 = !6429_u16;
Goto(bb6)
}
bb26 = {
_18.3.fld0 = 1_usize as u64;
_24 = Move(_11.0);
match _6.0 {
0 => bb6,
1 => bb3,
2 => bb27,
3 => bb28,
300499649512997741296567331340568324976 => bb30,
_ => bb29
}
}
bb27 = {
_34 = &_18.1;
match _3 {
0 => bb23,
1 => bb24,
62455 => bb26,
_ => bb25
}
}
bb28 = {
_6.0 = 90586231015564198238899173382549236880_i128 | (-117779031185040643963947207952878556179_i128);
_2 = !_6.1.fld4;
_1 = _6.1.fld2;
_6.1.fld0 = 969767965_u32 & 3483955023_u32;
_8 = ['\u{5adec}'];
_7 = '\u{f3b1f}' as u128;
_6.0 = (-122439183682418649820425038687491122860_i128) << _6.1.fld0;
_7 = 112_u8 as u128;
_6.1.fld2 = _6.0 as isize;
RET = Move(_6.1.fld3);
_5 = [_2,_6.1.fld4];
Goto(bb2)
}
bb29 = {
_12.fld3.2 = _12.fld0 as f64;
_10.3 = _20 as i32;
_12.fld3.3 = _19.1 ^ _10.3;
Call(_18.3.fld1 = core::intrinsics::fmaf64(_12.fld3.2, _10.2, _12.fld3.2), ReturnTo(bb12), UnwindUnreachable())
}
bb30 = {
_32 = _19.1 << _19.2.0;
_29 = _12.fld3.0;
_2 = _21.0.fld4;
Call(_12.fld3.3 = core::intrinsics::transmute(_18.3.fld5), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
_18.3.fld3 = (_12.fld3.0, _10.1, _31, _18.3.fld5);
_12.fld3.1 = _28 as u8;
_18.3.fld3 = (_29, _10.1, _12.fld1, _12.fld5);
_2 = _6.1.fld4;
_12.fld4 = _2 & _6.1.fld4;
_12 = Move(_18.3);
_27 = _1 - _6.1.fld2;
_18.3.fld3.3 = -_32;
_34 = &(*_34);
_6.0 = -_10.0;
_24 = &_6.1.fld0;
_3 = !5296_u16;
_11.0 = Move(_24);
_12 = Adt42 { fld0: _21.0.fld1,fld1: _19.2.2,fld2: _27,fld3: _10,fld4: _6.1.fld4,fld5: _18.3.fld3.3 };
Goto(bb32)
}
bb32 = {
_21.0.fld0 = (*_23) as u32;
match _7 {
0 => bb28,
1 => bb25,
2 => bb33,
186199214317207242499325827661813024597 => bb35,
_ => bb34
}
}
bb33 = {
_12.fld3.2 = _12.fld0 as f64;
_10.3 = _20 as i32;
_12.fld3.3 = _19.1 ^ _10.3;
Call(_18.3.fld1 = core::intrinsics::fmaf64(_12.fld3.2, _10.2, _12.fld3.2), ReturnTo(bb12), UnwindUnreachable())
}
bb34 = {
_32 = _19.1 << _19.2.0;
_29 = _12.fld3.0;
_2 = _21.0.fld4;
Call(_12.fld3.3 = core::intrinsics::transmute(_18.3.fld5), ReturnTo(bb31), UnwindUnreachable())
}
bb35 = {
_32 = !_12.fld5;
_12.fld3 = _10;
_18.3.fld3.0 = -_6.0;
_18.3.fld5 = !_12.fld5;
_18.2 = &_24;
_18.3.fld2 = _6.1.fld2 + _12.fld2;
_44 = _10.0 - _18.3.fld3.0;
_18.3.fld1 = 3_usize as f64;
_10.1 = _17 as u8;
match _7 {
0 => bb21,
1 => bb26,
2 => bb10,
3 => bb14,
186199214317207242499325827661813024597 => bb37,
_ => bb36
}
}
bb36 = {
_6.1.fld5 = [_18.3.fld2,_18.3.fld2,_1,_1,_9,_1,_9];
_18.3.fld3.3 = _14 * _19.1;
_10.1 = _19.2.3 + _19.2.3;
_12.fld3.0 = -_6.0;
_12.fld3.1 = _10.1;
_8 = [_20];
_18.3.fld5 = _10.3 + _14;
_7 = 283047996649086844620456673407368364742_u128;
_12.fld4 = _2 + _2;
_10.0 = _6.0 >> _12.fld0;
_19.1 = _14;
_18.3 = Adt42 { fld0: _19.2.0,fld1: _10.2,fld2: _1,fld3: _10,fld4: _21.0.fld4,fld5: _12.fld5 };
_11.0 = &_6.1.fld0;
_18.3.fld0 = _21.0.fld1;
_23 = &_16;
_12.fld0 = _19.2.0;
_12.fld3.2 = 6_i8 as f64;
_24 = Move(_11.0);
_21.0.fld1 = _19.2.0 + _12.fld0;
_18.3 = Adt42 { fld0: _19.2.0,fld1: _10.2,fld2: _21.0.fld2,fld3: _12.fld3,fld4: _12.fld4,fld5: _14 };
_21.0.fld4 = _12.fld4 & _18.3.fld4;
_10.2 = -_18.3.fld3.2;
_18.3.fld2 = _12.fld3.0 as isize;
Call(_12.fld2 = core::intrinsics::transmute(_9), ReturnTo(bb13), UnwindUnreachable())
}
bb37 = {
_43 = _10.2 as f32;
_25 = _10.1 as isize;
_18.3.fld3.2 = _18.1 as f64;
_6.1.fld0 = _21.0.fld0 + _21.0.fld0;
_46.fld2 = _25;
_25 = _3 as isize;
_6.1.fld4 = _12.fld4;
_6.1.fld5 = [_27,_1,_18.3.fld2,_21.0.fld2,_27,_21.0.fld2,_1];
_40 = _28;
_41 = _12.fld3.2 - _12.fld1;
_6.0 = _41 as i128;
_12.fld5 = -_18.3.fld3.3;
_12.fld5 = (*_23) as i32;
_6.1.fld4 = _6.0 as i16;
_21.0.fld3 = core::ptr::addr_of!(_43);
_46.fld5 = [_21.0.fld2,_21.0.fld2,_1,_6.1.fld2,_18.3.fld2,_46.fld2,_9];
_6.0 = _44;
_41 = _7 as f64;
_39 = _46.fld2 | _46.fld2;
RET = Move(_21.0.fld3);
_46.fld4 = _10.1 as i16;
_18.3.fld4 = _12.fld4;
_46 = Adt60 { fld0: _21.0.fld0,fld1: _21.0.fld1,fld2: _27,fld3: Move(RET),fld4: _18.3.fld4,fld5: _6.1.fld5 };
_18.3.fld5 = _18.3.fld3.3;
_45 = (*_23);
RET = core::ptr::addr_of!(_43);
_10 = (_29, _12.fld3.1, _12.fld1, _18.3.fld5);
Goto(bb38)
}
bb38 = {
Call(_50 = dump_var(13_usize, 14_usize, Move(_14), 29_usize, Move(_29), 40_usize, Move(_40), 8_usize, Move(_8)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_50 = dump_var(13_usize, 1_usize, Move(_1), 25_usize, Move(_25), 32_usize, Move(_32), 15_usize, Move(_15)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Call(_50 = dump_var(13_usize, 9_usize, Move(_9), 5_usize, Move(_5), 16_usize, Move(_16), 51_usize, _51), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: i128,mut _2: *const f32,mut _3: *const u32,mut _4: u16,mut _5: u128,mut _6: isize,mut _7: isize,mut _8: i128,mut _9: i128,mut _10: u16,mut _11: u128) -> Adt60 {
mir! {
type RET = Adt60;
let _12: *const bool;
let _13: Adt42;
let _14: i8;
let _15: u128;
let _16: (bool, u128, &'static Adt34, i64);
let _17: [u32; 8];
let _18: (*const bool, i8, &'static &'static u32, Adt42);
let _19: (&'static isize,);
let _20: bool;
let _21: i8;
let _22: isize;
let _23: (&'static (char, u128, i32),);
let _24: &'static u16;
let _25: Adt42;
let _26: u64;
let _27: [u16; 1];
let _28: isize;
let _29: [i8; 5];
let _30: f64;
let _31: f64;
let _32: Adt42;
let _33: f64;
let _34: i128;
let _35: ();
let _36: ();
{
RET.fld5 = [_6,_6,_7,_6,_7,_6,_6];
RET.fld4 = (-25524_i16) - 20634_i16;
RET.fld1 = !13983509555597902108_u64;
RET.fld3 = Move(_2);
RET.fld4 = -(-20123_i16);
RET.fld0 = 436126468_u32;
_3 = core::ptr::addr_of!(RET.fld0);
_4 = _10;
RET.fld2 = _11 as isize;
_10 = _4 >> _6;
_10 = 116_u8 as u16;
RET.fld0 = 9413131906853047453_usize as u32;
(*_3) = (-53_i8) as u32;
_2 = Move(RET.fld3);
RET.fld5 = [RET.fld2,_6,RET.fld2,_7,RET.fld2,_6,RET.fld2];
RET.fld1 = !9328406456501868574_u64;
_13.fld3.2 = _4 as f64;
_11 = !_5;
_9 = -_1;
(*_3) = (-1028542054895680767_i64) as u32;
_1 = _10 as i128;
_10 = _13.fld3.2 as u16;
_13.fld2 = !_6;
_13.fld3.0 = !_8;
_13.fld4 = RET.fld4 + RET.fld4;
match _8 {
150903039394951048429788523743622704591 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_13.fld4 = RET.fld4 >> _10;
RET.fld5 = [RET.fld2,RET.fld2,RET.fld2,_7,_13.fld2,_13.fld2,_6];
_16.0 = _11 > _5;
_13.fld1 = _13.fld3.2;
_5 = _11;
_8 = _1;
_11 = !_5;
Goto(bb3)
}
bb3 = {
_18.3.fld1 = _13.fld3.2 * _13.fld3.2;
_12 = core::ptr::addr_of!(_16.0);
(*_3) = 1596247354_u32;
_13.fld3.3 = 461065405006906745_i64 as i32;
_18.3.fld3.3 = _8 as i32;
_18.3.fld3.0 = _13.fld3.0;
_13.fld2 = _7;
_13.fld0 = !RET.fld1;
(*_3) = !1986443252_u32;
_13.fld3.0 = (-7907434431475690389_i64) as i128;
_13.fld3 = (_9, 51_u8, _13.fld1, _18.3.fld3.3);
_16.3 = _8 as i64;
RET.fld0 = !4000818455_u32;
RET.fld0 = !290479554_u32;
_15 = _5 << RET.fld1;
_3 = core::ptr::addr_of!((*_3));
_18.3.fld4 = _13.fld4;
RET.fld3 = Move(_2);
_13.fld3.0 = _13.fld3.1 as i128;
Goto(bb4)
}
bb4 = {
_13.fld5 = _13.fld3.3;
_2 = Move(RET.fld3);
_18.3.fld0 = !_13.fld0;
_16.0 = true;
_18.3.fld3.0 = (*_3) as i128;
RET.fld4 = _18.3.fld3.3 as i16;
Call(_13.fld3.2 = fn15(RET.fld2, _13.fld3.0, RET.fld0, _13.fld2, _18.3.fld4, (*_12), Move(_12), _13.fld4, _6, RET.fld5), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_7 = RET.fld2;
_18.1 = 46_i8;
match _13.fld3.1 {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
51 => bb9,
_ => bb8
}
}
bb6 = {
_13.fld5 = _13.fld3.3;
_2 = Move(RET.fld3);
_18.3.fld0 = !_13.fld0;
_16.0 = true;
_18.3.fld3.0 = (*_3) as i128;
RET.fld4 = _18.3.fld3.3 as i16;
Call(_13.fld3.2 = fn15(RET.fld2, _13.fld3.0, RET.fld0, _13.fld2, _18.3.fld4, (*_12), Move(_12), _13.fld4, _6, RET.fld5), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_18.3.fld1 = _13.fld3.2 * _13.fld3.2;
_12 = core::ptr::addr_of!(_16.0);
(*_3) = 1596247354_u32;
_13.fld3.3 = 461065405006906745_i64 as i32;
_18.3.fld3.3 = _8 as i32;
_18.3.fld3.0 = _13.fld3.0;
_13.fld2 = _7;
_13.fld0 = !RET.fld1;
(*_3) = !1986443252_u32;
_13.fld3.0 = (-7907434431475690389_i64) as i128;
_13.fld3 = (_9, 51_u8, _13.fld1, _18.3.fld3.3);
_16.3 = _8 as i64;
RET.fld0 = !4000818455_u32;
RET.fld0 = !290479554_u32;
_15 = _5 << RET.fld1;
_3 = core::ptr::addr_of!((*_3));
_18.3.fld4 = _13.fld4;
RET.fld3 = Move(_2);
_13.fld3.0 = _13.fld3.1 as i128;
Goto(bb4)
}
bb8 = {
Return()
}
bb9 = {
_18.3.fld3.1 = _4 as u8;
RET.fld0 = 2322298969_u32;
_13.fld2 = RET.fld2;
RET.fld3 = Move(_2);
_21 = -_18.1;
_18.3.fld5 = _13.fld5 * _13.fld3.3;
_16.3 = (-8451520443847035549_i64);
_13.fld3.2 = -_18.3.fld1;
_18.3.fld2 = _21 as isize;
_16.1 = _15 + _5;
_21 = _18.1 * _18.1;
_18.3.fld3 = (_13.fld3.0, _13.fld3.1, _13.fld3.2, _18.3.fld5);
RET.fld2 = _7;
RET.fld5 = [_7,_7,RET.fld2,_18.3.fld2,_6,_7,_7];
_21 = _18.1 << _18.3.fld4;
_18.0 = core::ptr::addr_of!(_20);
_13.fld3 = _18.3.fld3;
_17 = [(*_3),(*_3),RET.fld0,(*_3),(*_3),RET.fld0,(*_3),RET.fld0];
_24 = &_10;
_21 = _18.3.fld5 as i8;
_16.0 = false;
_8 = !_13.fld3.0;
Goto(bb10)
}
bb10 = {
_5 = !_11;
_9 = !_13.fld3.0;
_13.fld3.2 = (*_24) as f64;
_12 = core::ptr::addr_of!(_20);
(*_12) = _16.0 ^ _16.0;
RET.fld1 = !_13.fld0;
_18.3.fld1 = _18.1 as f64;
_18.3.fld4 = 9146068523140560724_usize as i16;
_25.fld3.2 = _18.3.fld1;
_18.3.fld5 = _13.fld3.1 as i32;
_18.3.fld5 = RET.fld1 as i32;
_25.fld1 = _18.3.fld3.2;
_25.fld2 = _7;
_13.fld4 = RET.fld4 << _13.fld3.0;
RET.fld5 = [_6,_13.fld2,_6,_18.3.fld2,_25.fld2,_6,_6];
_25.fld3 = _13.fld3;
_25 = Adt42 { fld0: RET.fld1,fld1: _13.fld1,fld2: RET.fld2,fld3: _13.fld3,fld4: _18.3.fld4,fld5: _18.3.fld5 };
_18.3.fld3.1 = !_13.fld3.1;
(*_3) = 3821704382_u32 ^ 1527138362_u32;
match _25.fld3.1 {
0 => bb1,
51 => bb12,
_ => bb11
}
}
bb11 = {
_13.fld4 = RET.fld4 >> _10;
RET.fld5 = [RET.fld2,RET.fld2,RET.fld2,_7,_13.fld2,_13.fld2,_6];
_16.0 = _11 > _5;
_13.fld1 = _13.fld3.2;
_5 = _11;
_8 = _1;
_11 = !_5;
Goto(bb3)
}
bb12 = {
_18.3.fld5 = _7 as i32;
_13.fld1 = (*_3) as f64;
_25.fld0 = _18.3.fld3.3 as u64;
_7 = _18.3.fld2;
_15 = _16.1 - _11;
_25.fld0 = !_18.3.fld0;
_18.3 = Move(_13);
_19.0 = &_7;
_4 = (*_24);
_13.fld0 = _25.fld0 << _18.3.fld3.1;
_10 = _4;
_13.fld3.0 = _9 + _25.fld3.0;
_24 = &_10;
_27 = [_4];
_13.fld5 = _18.3.fld5;
RET.fld0 = 1792650946_u32;
_9 = _13.fld3.0;
_25.fld3 = (_18.3.fld3.0, _18.3.fld3.1, _25.fld1, _18.3.fld5);
RET.fld2 = _25.fld2;
_16.1 = _15;
Goto(bb13)
}
bb13 = {
_25.fld0 = _13.fld0 << _13.fld0;
RET.fld0 = _16.1 as u32;
_9 = _25.fld1 as i128;
_4 = (*_24);
_16.3 = (-3462625927672411959_i64) | 188665660275258233_i64;
_26 = _16.1 as u64;
_13.fld3.3 = _18.3.fld3.3 * _13.fld5;
_16.1 = _20 as u128;
_16.3 = (-5025244498555014679_i64);
_18.3.fld1 = -_18.3.fld3.2;
_22 = -_18.3.fld2;
_25 = Adt42 { fld0: _13.fld0,fld1: _18.3.fld1,fld2: _22,fld3: _18.3.fld3,fld4: _18.3.fld4,fld5: _13.fld5 };
_32 = Move(_18.3);
RET.fld4 = _32.fld4 - _32.fld4;
_25.fld3.3 = _13.fld3.3 << RET.fld4;
_18.3.fld4 = _32.fld4 - RET.fld4;
_9 = _13.fld3.0;
Goto(bb14)
}
bb14 = {
Call(_35 = dump_var(14_usize, 7_usize, Move(_7), 10_usize, Move(_10), 20_usize, Move(_20), 26_usize, Move(_26)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_35 = dump_var(14_usize, 11_usize, Move(_11), 22_usize, Move(_22), 21_usize, Move(_21), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: isize,mut _2: i128,mut _3: u32,mut _4: isize,mut _5: i16,mut _6: bool,mut _7: *const bool,mut _8: i16,mut _9: isize,mut _10: [isize; 7]) -> f64 {
mir! {
type RET = f64;
let _11: (*const f32,);
let _12: [u64; 7];
let _13: (&'static isize,);
let _14: u8;
let _15: isize;
let _16: [u64; 7];
let _17: isize;
let _18: i32;
let _19: *const u32;
let _20: i8;
let _21: &'static (char, u128, i32);
let _22: i16;
let _23: *mut &'static u32;
let _24: [u8; 1];
let _25: [i16; 2];
let _26: f32;
let _27: isize;
let _28: i64;
let _29: [u128; 3];
let _30: u16;
let _31: u64;
let _32: usize;
let _33: ();
let _34: ();
{
_9 = _4 & _1;
_8 = _9 as i16;
_8 = _2 as i16;
_2 = (-165167217271588723062397639562079051071_i128);
RET = _3 as f64;
_1 = !_9;
_5 = _8 * _8;
_12 = [1640619514249919613_u64,17360120199575611869_u64,4352587256767762221_u64,2464488508705124253_u64,16003594553120909009_u64,14572466336684521330_u64,15881139026258930553_u64];
_10 = [_9,_9,_9,_9,_4,_9,_1];
_7 = core::ptr::addr_of!(_6);
(*_7) = false;
_9 = 5_usize as isize;
_13.0 = &_9;
_14 = 54_u8;
RET = 2075211249831410052_u64 as f64;
_14 = 186_u8;
_7 = core::ptr::addr_of!((*_7));
_12 = [15192441148533749214_u64,529441154200287518_u64,2389599140867651059_u64,5946750214647015778_u64,13585303910269935060_u64,3269522215038535159_u64,1748754559069909057_u64];
_5 = -_8;
_12 = [4582317311730664334_u64,4206427356280221340_u64,386405399086585705_u64,16837061474215439566_u64,16757119535044859924_u64,11173646675852761052_u64,15350779166221437703_u64];
_7 = core::ptr::addr_of!((*_7));
(*_7) = false | false;
RET = _14 as f64;
Goto(bb1)
}
bb1 = {
_16 = [4083030685703563212_u64,4541431845040213325_u64,5743855193651931055_u64,14684086548277524534_u64,14669031844302602830_u64,9240163690190872348_u64,14417179986932152681_u64];
_4 = _1 - _1;
_16 = [6927865932101460435_u64,9656191589346461328_u64,10683585998226519213_u64,18160599572321059520_u64,1746661037291097164_u64,1317077102110055629_u64,3535014757120295981_u64];
_16 = [1933017698847721827_u64,2687934526601993032_u64,4702786106378921034_u64,12248729090379706818_u64,3360035617124153154_u64,7106044343663032221_u64,17683773882572397943_u64];
RET = 46_i8 as f64;
(*_7) = false;
_10 = [_1,_4,_1,_4,_4,_1,_4];
_8 = RET as i16;
Goto(bb2)
}
bb2 = {
_15 = (-5744619321584188959_i64) as isize;
_15 = _4 & _4;
_1 = RET as isize;
_6 = !false;
_3 = !767095283_u32;
RET = _2 as f64;
_17 = _2 as isize;
_9 = -_15;
_18 = -(-560691921_i32);
_3 = 3490166640_u32;
RET = _3 as f64;
_4 = _15;
_2 = -154582590503695995594677140191671848124_i128;
_9 = -_4;
_13.0 = &_17;
_1 = -_15;
_3 = 807828443_u32;
_20 = -(-49_i8);
_4 = !_15;
_5 = -_8;
_3 = 2387837404_u32;
_10 = [_15,_1,_9,_1,_15,_4,_15];
_17 = _1;
Call(_1 = fn16(_9, _9, _10, _4, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5 = -_8;
_20 = -52_i8;
_14 = !65_u8;
_14 = !244_u8;
match _3 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
2387837404 => bb10,
_ => bb9
}
}
bb4 = {
_15 = (-5744619321584188959_i64) as isize;
_15 = _4 & _4;
_1 = RET as isize;
_6 = !false;
_3 = !767095283_u32;
RET = _2 as f64;
_17 = _2 as isize;
_9 = -_15;
_18 = -(-560691921_i32);
_3 = 3490166640_u32;
RET = _3 as f64;
_4 = _15;
_2 = -154582590503695995594677140191671848124_i128;
_9 = -_4;
_13.0 = &_17;
_1 = -_15;
_3 = 807828443_u32;
_20 = -(-49_i8);
_4 = !_15;
_5 = -_8;
_3 = 2387837404_u32;
_10 = [_15,_1,_9,_1,_15,_4,_15];
_17 = _1;
Call(_1 = fn16(_9, _9, _10, _4, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_16 = [4083030685703563212_u64,4541431845040213325_u64,5743855193651931055_u64,14684086548277524534_u64,14669031844302602830_u64,9240163690190872348_u64,14417179986932152681_u64];
_4 = _1 - _1;
_16 = [6927865932101460435_u64,9656191589346461328_u64,10683585998226519213_u64,18160599572321059520_u64,1746661037291097164_u64,1317077102110055629_u64,3535014757120295981_u64];
_16 = [1933017698847721827_u64,2687934526601993032_u64,4702786106378921034_u64,12248729090379706818_u64,3360035617124153154_u64,7106044343663032221_u64,17683773882572397943_u64];
RET = 46_i8 as f64;
(*_7) = false;
_10 = [_1,_4,_1,_4,_4,_1,_4];
_8 = RET as i16;
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
_16 = [6219257731248549684_u64,4733807179400082539_u64,6486437431572045781_u64,1284264099863421675_u64,12851386174779086466_u64,17407945321631694555_u64,11110183729733067031_u64];
RET = _5 as f64;
RET = 209125293733377820_u64 as f64;
_13.0 = &_1;
_10 = [_9,_15,_1,_17,_1,_17,_1];
_12 = _16;
_4 = (-8466841324163830495_i64) as isize;
_14 = 186_u8;
_12 = [6891994866652676254_u64,13155318716293388275_u64,5115734919635510361_u64,4491416785176787756_u64,17458842472511481679_u64,11233850389648869675_u64,6838399075977563630_u64];
RET = _18 as f64;
_12 = _16;
_7 = core::ptr::addr_of!((*_7));
_12 = [10892448609131421610_u64,4570872069784695640_u64,2992577067787392897_u64,7850859652098854736_u64,17405239805628010547_u64,10379589207974639077_u64,17011050194653840951_u64];
_10 = [_9,_1,_1,_1,_15,_9,_1];
_5 = _8 & _8;
_6 = _1 != _1;
_22 = _14 as i16;
_3 = 3571098939_u32;
_8 = _5;
_17 = 1_usize as isize;
_6 = false & true;
_7 = core::ptr::addr_of!(_6);
_19 = core::ptr::addr_of!(_3);
_2 = -7209788041614818351522374806207175860_i128;
_9 = -_1;
(*_19) = 3783862951_u32 << _9;
Call(_16 = fn17(Move(_13.0), (*_19), (*_19), Move(_19), _1, _10, _1, (*_19), _1, _1, _15, _10), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_2 = RET as i128;
Goto(bb12)
}
bb12 = {
(*_7) = false & false;
_2 = 140845346294651729751810896228940950585_i128 + (-55883546886262642171303127394417663386_i128);
_19 = core::ptr::addr_of!(_3);
_1 = _9 & _15;
_13.0 = &_4;
Goto(bb13)
}
bb13 = {
_27 = _1;
_26 = _27 as f32;
_28 = !(-3102350700357136627_i64);
_1 = _17 - _9;
_10 = [_9,_27,_9,_27,_27,_1,_15];
_18 = 149432680_i32;
_19 = core::ptr::addr_of!((*_19));
match _14 {
0 => bb12,
1 => bb5,
2 => bb3,
186 => bb14,
_ => bb11
}
}
bb14 = {
_26 = 198520207117621492028995055831163968531_u128 as f32;
_29 = [326598890119998060087918921178563748630_u128,77600223061940908551912799147540702703_u128,93121003991934829889310811263507316429_u128];
(*_19) = 683066327_u32;
RET = 61372_u16 as f64;
_11.0 = core::ptr::addr_of!(_26);
_31 = _20 as u64;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(15_usize, 4_usize, Move(_4), 14_usize, Move(_14), 9_usize, Move(_9), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(15_usize, 17_usize, Move(_17), 3_usize, Move(_3), 8_usize, Move(_8), 29_usize, Move(_29)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(15_usize, 31_usize, Move(_31), 10_usize, Move(_10), 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: isize,mut _2: isize,mut _3: [isize; 7],mut _4: isize,mut _5: isize) -> isize {
mir! {
type RET = isize;
let _6: *const u32;
let _7: ();
let _8: ();
{
_4 = -_5;
_1 = -_2;
_5 = _1;
_3 = [_4,_1,_1,_4,_5,_4,_5];
RET = _5;
_4 = _1 >> _5;
RET = (-243926982306524026_i64) as isize;
RET = _2 * _1;
_1 = RET * _4;
RET = (-51691194673215081606018839585493408981_i128) as isize;
_3 = [_1,_1,_4,_1,_5,_5,_4];
_1 = _4 << _5;
RET = _2;
RET = _1 >> _4;
RET = _1 - _2;
_2 = RET;
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(16_usize, 2_usize, Move(_2), 3_usize, Move(_3), 8_usize, _8, 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: &'static isize,mut _2: u32,mut _3: u32,mut _4: *const u32,mut _5: isize,mut _6: [isize; 7],mut _7: isize,mut _8: u32,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: [isize; 7]) -> [u64; 7] {
mir! {
type RET = [u64; 7];
let _13: (Adt60,);
let _14: [char; 7];
let _15: (&'static isize,);
let _16: (u64, [u16; 3], f64, u8);
let _17: [u8; 1];
let _18: (&'static isize,);
let _19: ([u8; 1], i32, (u64, [u16; 3], f64, u8));
let _20: f64;
let _21: (&'static (char, u128, i32),);
let _22: u16;
let _23: &'static [i8; 6];
let _24: isize;
let _25: [u64; 7];
let _26: ();
let _27: ();
{
_2 = _3;
Goto(bb1)
}
bb1 = {
_2 = _8 << _9;
_1 = &_11;
_1 = &_7;
_10 = '\u{ca9ea}' as isize;
_13.0.fld4 = 26091_i16;
_1 = &(*_1);
RET = [7378322303951218469_u64,7942691166080663388_u64,17151565860364234544_u64,8077846126973006501_u64,11244786935870022153_u64,12565345958527641462_u64,8557857989840604040_u64];
_7 = -_5;
_5 = _10 - _9;
_1 = &_10;
_13.0.fld2 = _7;
_13.0.fld4 = 12466_i16 * 18024_i16;
RET = [5391565405676529936_u64,10876843520964435042_u64,9181399466355370614_u64,15430308107542357211_u64,11234793836591540648_u64,13184557763361735380_u64,3750908828920928290_u64];
RET = [4018850357179511571_u64,3524596338041820831_u64,16671980805356958200_u64,6391135426172933463_u64,6021674189463601907_u64,2067958105985536102_u64,14947230726134339207_u64];
_11 = false as isize;
_13.0.fld0 = _8 * _3;
_13.0.fld1 = 5674714027102918043_u64 | 8718869015042624216_u64;
Goto(bb2)
}
bb2 = {
_7 = !_9;
_13.0.fld5 = _12;
_2 = _13.0.fld0 * _13.0.fld0;
_4 = core::ptr::addr_of!(_3);
_9 = -_13.0.fld2;
_6 = _13.0.fld5;
_14 = ['\u{394d5}','\u{c82d3}','\u{dd45d}','\u{44598}','\u{d5b93}','\u{9cd0e}','\u{9fddf}'];
_13.0.fld2 = _13.0.fld1 as isize;
_12 = [_5,_9,_5,_7,_5,_5,_7];
_4 = core::ptr::addr_of!(_2);
_16.2 = _13.0.fld0 as f64;
_4 = core::ptr::addr_of!(_2);
_14 = ['\u{404ae}','\u{46d9}','\u{5c264}','\u{10f364}','\u{10a230}','\u{72c54}','\u{10ba97}'];
Goto(bb3)
}
bb3 = {
_18 = (Move(_1),);
_11 = !_5;
_2 = _13.0.fld0 - _3;
_11 = _7;
_13.0.fld1 = 6211019169940983260_u64;
_15.0 = &_13.0.fld2;
_19.1 = -(-977306482_i32);
_19.2.3 = _19.1 as u8;
_16.0 = _13.0.fld1;
_8 = (-2455933412511654241_i64) as u32;
_13.0.fld0 = (*_4);
_18 = Move(_15);
_19.1 = -1877499696_i32;
_1 = &_7;
_13.0.fld0 = !(*_4);
_19.2.1 = [29874_u16,1738_u16,14668_u16];
_19.0 = [_19.2.3];
_18 = (Move(_1),);
Goto(bb4)
}
bb4 = {
_11 = _7;
_11 = 7_usize as isize;
_16.2 = _19.1 as f64;
_17 = _19.0;
_13.0.fld1 = (*_4) as u64;
Goto(bb5)
}
bb5 = {
_16.2 = (-1609010507254760808_i64) as f64;
_1 = &_9;
RET = [_13.0.fld1,_13.0.fld1,_13.0.fld1,_13.0.fld1,_13.0.fld1,_13.0.fld1,_13.0.fld1];
_16.0 = _19.2.3 as u64;
_19.2.0 = _13.0.fld1;
_13.0.fld0 = _2 - (*_4);
_11 = (*_1) | _7;
_8 = (*_4);
_10 = _5;
_18 = (Move(_1),);
_8 = (*_4);
_25 = [_13.0.fld1,_13.0.fld1,_13.0.fld1,_19.2.0,_13.0.fld1,_19.2.0,_19.2.0];
_20 = _16.2;
Goto(bb6)
}
bb6 = {
Call(_26 = dump_var(17_usize, 3_usize, Move(_3), 9_usize, Move(_9), 8_usize, Move(_8), 17_usize, Move(_17)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_26 = dump_var(17_usize, 6_usize, Move(_6), 14_usize, Move(_14), 27_usize, _27, 27_usize, _27), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: u8,mut _2: char,mut _3: u32,mut _4: [char; 1],mut _5: u64,mut _6: [i16; 2],mut _7: [u8; 1],mut _8: *const f32,mut _9: [i16; 2],mut _10: u32) -> isize {
mir! {
type RET = isize;
let _11: *mut (bool, u128);
let _12: char;
let _13: f32;
let _14: isize;
let _15: (bool, u128);
let _16: (char, u128, i32);
let _17: i8;
let _18: u8;
let _19: [u16; 1];
let _20: [u32; 8];
let _21: u128;
let _22: *const [u128; 3];
let _23: Adt42;
let _24: f32;
let _25: ();
let _26: ();
{
_7 = [_1];
_1 = 121_u8;
RET = _10 as isize;
RET = (-9223372036854775808_isize);
RET = 105_isize & (-109_isize);
RET = -63_isize;
_5 = !1281945791327341548_u64;
_3 = _10;
RET = 52_isize & 9223372036854775807_isize;
_4 = [_2];
_1 = 27_u8 + 96_u8;
_4 = [_2];
Call(_1 = fn19(_7, _6, _7, RET, _6, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = !178_u8;
_6 = [(-13396_i16),(-11434_i16)];
_4 = [_2];
RET = 5284496855299396993_usize as isize;
_2 = '\u{8b378}';
_6 = [(-21722_i16),959_i16];
_1 = 194_u8 << _3;
_3 = !_10;
RET = (-89_isize);
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463463374607431768211367 => bb10,
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
_13 = 311489410283643708_usize as f32;
_12 = _2;
_8 = core::ptr::addr_of!(_13);
_4 = [_12];
_9 = _6;
_2 = _12;
_4 = [_2];
_8 = core::ptr::addr_of!((*_8));
(*_8) = _3 as f32;
_1 = 13_u8 - 52_u8;
_5 = RET as u64;
_2 = _12;
_3 = !_10;
_2 = _12;
(*_8) = _1 as f32;
Goto(bb11)
}
bb11 = {
_13 = _10 as f32;
(*_8) = 263496145484549508217450434349484173595_u128 as f32;
_14 = RET;
_15 = (false, 111814204497027854527358003937497182814_u128);
_15.1 = (*_8) as u128;
RET = _14;
_7 = [_1];
_15.0 = _1 <= _1;
_17 = !(-109_i8);
(*_8) = _1 as f32;
_13 = _1 as f32;
_18 = _1 * _1;
_10 = _3;
_16.1 = _15.1;
_15.1 = RET as u128;
_11 = core::ptr::addr_of_mut!(_15);
(*_8) = _17 as f32;
(*_8) = (-3335959457337971442_i64) as f32;
_16.2 = -1915000546_i32;
(*_8) = _17 as f32;
Goto(bb12)
}
bb12 = {
_9 = _6;
_15.1 = _16.1 | _16.1;
_11 = core::ptr::addr_of_mut!((*_11));
_9 = _6;
_12 = _2;
_5 = 15332117156678635697_u64;
_9 = _6;
_3 = _10 << (*_11).1;
_15.0 = _10 <= _3;
_12 = _2;
_15.0 = false;
match _14 {
0 => bb10,
340282366920938463463374607431768211367 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_16.0 = _12;
_12 = _16.0;
_6 = [(-27267_i16),24758_i16];
_19 = [23963_u16];
_2 = _12;
_15.0 = false;
_19 = [38275_u16];
(*_8) = _16.2 as f32;
_23.fld3.0 = -(-53574906218807461502083678322662129991_i128);
_24 = (*_8) * _13;
_23.fld3.2 = RET as f64;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(18_usize, 1_usize, Move(_1), 19_usize, Move(_19), 9_usize, Move(_9), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(18_usize, 16_usize, Move(_16), 12_usize, Move(_12), 14_usize, Move(_14), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: [u8; 1],mut _2: [i16; 2],mut _3: [u8; 1],mut _4: isize,mut _5: [i16; 2],mut _6: [i16; 2]) -> u8 {
mir! {
type RET = u8;
let _7: [u8; 1];
let _8: [i8; 6];
let _9: *mut usize;
let _10: f64;
let _11: bool;
let _12: char;
let _13: &'static [i8; 6];
let _14: [i8; 2];
let _15: (*mut Adt42, &'static u128, u16);
let _16: f32;
let _17: &'static Adt34;
let _18: [char; 1];
let _19: usize;
let _20: [i8; 6];
let _21: char;
let _22: bool;
let _23: *const f32;
let _24: f64;
let _25: isize;
let _26: i8;
let _27: isize;
let _28: isize;
let _29: &'static u128;
let _30: u8;
let _31: ();
let _32: ();
{
_8 = [26_i8,89_i8,(-38_i8),103_i8,(-97_i8),(-120_i8)];
RET = 178_u8 * 132_u8;
_7 = [RET];
_4 = 119408795589477281094570484523782700896_i128 as isize;
_3 = [RET];
_4 = 19684_i16 as isize;
RET = 68_u8;
_6 = [(-29909_i16),12487_i16];
RET = 0_u8 - 38_u8;
_5 = [22501_i16,15959_i16];
_5 = _6;
_6 = [(-26916_i16),(-17462_i16)];
_10 = 6710406380803093547_u64 as f64;
RET = 2590380238345405505_i64 as u8;
_1 = [RET];
Goto(bb1)
}
bb1 = {
RET = 220_u8;
_6 = [(-7938_i16),6107_i16];
_1 = _7;
_2 = [(-30395_i16),23357_i16];
_2 = [(-28259_i16),28848_i16];
_11 = !false;
_8 = [122_i8,43_i8,(-83_i8),55_i8,(-118_i8),(-108_i8)];
_8 = [120_i8,(-83_i8),(-43_i8),(-110_i8),73_i8,117_i8];
_14 = [(-60_i8),(-21_i8)];
_8 = [60_i8,(-110_i8),(-94_i8),(-5_i8),(-87_i8),(-44_i8)];
_11 = true;
_11 = !true;
_1 = [RET];
_3 = [RET];
_13 = &_8;
_12 = '\u{6f18a}';
_1 = _7;
_8 = [(-77_i8),(-81_i8),72_i8,70_i8,62_i8,27_i8];
_15.2 = 44763_u16;
Goto(bb2)
}
bb2 = {
_5 = [22231_i16,(-27500_i16)];
_16 = _15.2 as f32;
_10 = (-103665120783419252171883755973653483915_i128) as f64;
_5 = [1555_i16,(-15023_i16)];
_14 = [78_i8,(-20_i8)];
RET = !91_u8;
_12 = '\u{cc723}';
_16 = RET as f32;
_4 = 9223372036854775807_isize;
_10 = (-3632469410984500867_i64) as f64;
_16 = RET as f32;
_3 = [RET];
_7 = _3;
_11 = false;
_8 = [4_i8,29_i8,112_i8,(-19_i8),(-29_i8),(-11_i8)];
_15.2 = _16 as u16;
_9 = core::ptr::addr_of_mut!(_19);
(*_9) = 9190707296208096381_usize;
_8 = [93_i8,(-10_i8),13_i8,104_i8,108_i8,(-67_i8)];
_12 = '\u{9d163}';
_10 = 12651008767045241050580222096855786533_i128 as f64;
_3 = _1;
_10 = (-73864493246134596468060679289627286906_i128) as f64;
_18 = [_12];
RET = !236_u8;
match _19 {
9190707296208096381 => bb3,
_ => bb1
}
}
bb3 = {
(*_9) = !6247201339043856807_usize;
_2 = [6108_i16,25155_i16];
_9 = core::ptr::addr_of_mut!(_19);
_13 = &_20;
_5 = [(-17711_i16),(-3597_i16)];
_9 = core::ptr::addr_of_mut!(_19);
_6 = [(-17717_i16),(-5359_i16)];
_3 = [RET];
_11 = false;
_4 = 103_isize;
_12 = '\u{c95bd}';
_7 = [RET];
_19 = 16739117790310342740_usize * 2_usize;
_2 = [13853_i16,(-5329_i16)];
_20 = [44_i8,44_i8,(-14_i8),113_i8,34_i8,(-100_i8)];
_19 = !4_usize;
_7 = [RET];
_11 = false;
_2 = _5;
_16 = _19 as f32;
_14 = [(-16_i8),(-36_i8)];
_6 = [21_i16,24537_i16];
_14 = [(-31_i8),59_i8];
_14 = [118_i8,12_i8];
RET = !21_u8;
_11 = true;
match _4 {
0 => bb4,
103 => bb6,
_ => bb5
}
}
bb4 = {
_5 = [22231_i16,(-27500_i16)];
_16 = _15.2 as f32;
_10 = (-103665120783419252171883755973653483915_i128) as f64;
_5 = [1555_i16,(-15023_i16)];
_14 = [78_i8,(-20_i8)];
RET = !91_u8;
_12 = '\u{cc723}';
_16 = RET as f32;
_4 = 9223372036854775807_isize;
_10 = (-3632469410984500867_i64) as f64;
_16 = RET as f32;
_3 = [RET];
_7 = _3;
_11 = false;
_8 = [4_i8,29_i8,112_i8,(-19_i8),(-29_i8),(-11_i8)];
_15.2 = _16 as u16;
_9 = core::ptr::addr_of_mut!(_19);
(*_9) = 9190707296208096381_usize;
_8 = [93_i8,(-10_i8),13_i8,104_i8,108_i8,(-67_i8)];
_12 = '\u{9d163}';
_10 = 12651008767045241050580222096855786533_i128 as f64;
_3 = _1;
_10 = (-73864493246134596468060679289627286906_i128) as f64;
_18 = [_12];
RET = !236_u8;
match _19 {
9190707296208096381 => bb3,
_ => bb1
}
}
bb5 = {
RET = 220_u8;
_6 = [(-7938_i16),6107_i16];
_1 = _7;
_2 = [(-30395_i16),23357_i16];
_2 = [(-28259_i16),28848_i16];
_11 = !false;
_8 = [122_i8,43_i8,(-83_i8),55_i8,(-118_i8),(-108_i8)];
_8 = [120_i8,(-83_i8),(-43_i8),(-110_i8),73_i8,117_i8];
_14 = [(-60_i8),(-21_i8)];
_8 = [60_i8,(-110_i8),(-94_i8),(-5_i8),(-87_i8),(-44_i8)];
_11 = true;
_11 = !true;
_1 = [RET];
_3 = [RET];
_13 = &_8;
_12 = '\u{6f18a}';
_1 = _7;
_8 = [(-77_i8),(-81_i8),72_i8,70_i8,62_i8,27_i8];
_15.2 = 44763_u16;
Goto(bb2)
}
bb6 = {
_18 = [_12];
_22 = _11;
_25 = -_4;
_14 = [127_i8,124_i8];
_2 = [23073_i16,25217_i16];
_21 = _12;
_5 = [1302_i16,24426_i16];
_25 = _4;
_20 = _8;
_25 = _4 << _19;
_14 = [87_i8,127_i8];
_11 = !_22;
_18 = [_12];
_13 = &_8;
_26 = (-12_i8);
RET = 205_u8;
_20 = (*_13);
_22 = !_11;
Goto(bb7)
}
bb7 = {
_28 = _4 & _25;
_25 = -_4;
_26 = _16 as i8;
_5 = _2;
_18 = [_12];
_23 = core::ptr::addr_of!(_16);
RET = !103_u8;
_22 = _11;
_12 = _21;
_10 = _19 as f64;
match _4 {
0 => bb8,
1 => bb9,
103 => bb11,
_ => bb10
}
}
bb8 = {
_18 = [_12];
_22 = _11;
_25 = -_4;
_14 = [127_i8,124_i8];
_2 = [23073_i16,25217_i16];
_21 = _12;
_5 = [1302_i16,24426_i16];
_25 = _4;
_20 = _8;
_25 = _4 << _19;
_14 = [87_i8,127_i8];
_11 = !_22;
_18 = [_12];
_13 = &_8;
_26 = (-12_i8);
RET = 205_u8;
_20 = (*_13);
_22 = !_11;
Goto(bb7)
}
bb9 = {
RET = 220_u8;
_6 = [(-7938_i16),6107_i16];
_1 = _7;
_2 = [(-30395_i16),23357_i16];
_2 = [(-28259_i16),28848_i16];
_11 = !false;
_8 = [122_i8,43_i8,(-83_i8),55_i8,(-118_i8),(-108_i8)];
_8 = [120_i8,(-83_i8),(-43_i8),(-110_i8),73_i8,117_i8];
_14 = [(-60_i8),(-21_i8)];
_8 = [60_i8,(-110_i8),(-94_i8),(-5_i8),(-87_i8),(-44_i8)];
_11 = true;
_11 = !true;
_1 = [RET];
_3 = [RET];
_13 = &_8;
_12 = '\u{6f18a}';
_1 = _7;
_8 = [(-77_i8),(-81_i8),72_i8,70_i8,62_i8,27_i8];
_15.2 = 44763_u16;
Goto(bb2)
}
bb10 = {
(*_9) = !6247201339043856807_usize;
_2 = [6108_i16,25155_i16];
_9 = core::ptr::addr_of_mut!(_19);
_13 = &_20;
_5 = [(-17711_i16),(-3597_i16)];
_9 = core::ptr::addr_of_mut!(_19);
_6 = [(-17717_i16),(-5359_i16)];
_3 = [RET];
_11 = false;
_4 = 103_isize;
_12 = '\u{c95bd}';
_7 = [RET];
_19 = 16739117790310342740_usize * 2_usize;
_2 = [13853_i16,(-5329_i16)];
_20 = [44_i8,44_i8,(-14_i8),113_i8,34_i8,(-100_i8)];
_19 = !4_usize;
_7 = [RET];
_11 = false;
_2 = _5;
_16 = _19 as f32;
_14 = [(-16_i8),(-36_i8)];
_6 = [21_i16,24537_i16];
_14 = [(-31_i8),59_i8];
_14 = [118_i8,12_i8];
RET = !21_u8;
_11 = true;
match _4 {
0 => bb4,
103 => bb6,
_ => bb5
}
}
bb11 = {
RET = 82_u8;
_21 = _12;
(*_23) = RET as f32;
_12 = _21;
_6 = [(-8668_i16),26104_i16];
_18 = [_12];
_3 = [RET];
_28 = _16 as isize;
_23 = core::ptr::addr_of!(_16);
_24 = _10 + _10;
_12 = _21;
_1 = [RET];
_22 = _11;
(*_9) = 300085777905694190508467497903437645154_u128 as usize;
_27 = _4 & _28;
_22 = _11 | _11;
_26 = -3_i8;
_4 = -_28;
_28 = _25;
_13 = &_20;
_7 = [RET];
(*_23) = (*_9) as f32;
_26 = 50_i8 & (-33_i8);
_13 = &(*_13);
_19 = !4609723897074018723_usize;
_10 = _24;
_26 = 92_i8 << _25;
_25 = -_27;
_27 = _25 ^ _4;
_26 = 108_i8 * (-104_i8);
match RET {
0 => bb8,
1 => bb2,
2 => bb12,
3 => bb13,
82 => bb15,
_ => bb14
}
}
bb12 = {
RET = 220_u8;
_6 = [(-7938_i16),6107_i16];
_1 = _7;
_2 = [(-30395_i16),23357_i16];
_2 = [(-28259_i16),28848_i16];
_11 = !false;
_8 = [122_i8,43_i8,(-83_i8),55_i8,(-118_i8),(-108_i8)];
_8 = [120_i8,(-83_i8),(-43_i8),(-110_i8),73_i8,117_i8];
_14 = [(-60_i8),(-21_i8)];
_8 = [60_i8,(-110_i8),(-94_i8),(-5_i8),(-87_i8),(-44_i8)];
_11 = true;
_11 = !true;
_1 = [RET];
_3 = [RET];
_13 = &_8;
_12 = '\u{6f18a}';
_1 = _7;
_8 = [(-77_i8),(-81_i8),72_i8,70_i8,62_i8,27_i8];
_15.2 = 44763_u16;
Goto(bb2)
}
bb13 = {
RET = 220_u8;
_6 = [(-7938_i16),6107_i16];
_1 = _7;
_2 = [(-30395_i16),23357_i16];
_2 = [(-28259_i16),28848_i16];
_11 = !false;
_8 = [122_i8,43_i8,(-83_i8),55_i8,(-118_i8),(-108_i8)];
_8 = [120_i8,(-83_i8),(-43_i8),(-110_i8),73_i8,117_i8];
_14 = [(-60_i8),(-21_i8)];
_8 = [60_i8,(-110_i8),(-94_i8),(-5_i8),(-87_i8),(-44_i8)];
_11 = true;
_11 = !true;
_1 = [RET];
_3 = [RET];
_13 = &_8;
_12 = '\u{6f18a}';
_1 = _7;
_8 = [(-77_i8),(-81_i8),72_i8,70_i8,62_i8,27_i8];
_15.2 = 44763_u16;
Goto(bb2)
}
bb14 = {
(*_9) = !6247201339043856807_usize;
_2 = [6108_i16,25155_i16];
_9 = core::ptr::addr_of_mut!(_19);
_13 = &_20;
_5 = [(-17711_i16),(-3597_i16)];
_9 = core::ptr::addr_of_mut!(_19);
_6 = [(-17717_i16),(-5359_i16)];
_3 = [RET];
_11 = false;
_4 = 103_isize;
_12 = '\u{c95bd}';
_7 = [RET];
_19 = 16739117790310342740_usize * 2_usize;
_2 = [13853_i16,(-5329_i16)];
_20 = [44_i8,44_i8,(-14_i8),113_i8,34_i8,(-100_i8)];
_19 = !4_usize;
_7 = [RET];
_11 = false;
_2 = _5;
_16 = _19 as f32;
_14 = [(-16_i8),(-36_i8)];
_6 = [21_i16,24537_i16];
_14 = [(-31_i8),59_i8];
_14 = [118_i8,12_i8];
RET = !21_u8;
_11 = true;
match _4 {
0 => bb4,
103 => bb6,
_ => bb5
}
}
bb15 = {
_13 = &(*_13);
(*_23) = 5235981210804417543_u64 as f32;
(*_9) = 5959915511582977427_usize;
_18 = [_21];
_3 = [RET];
_25 = 1272120565_i32 as isize;
RET = _4 as u8;
RET = !209_u8;
_11 = !_22;
_23 = core::ptr::addr_of!((*_23));
Goto(bb16)
}
bb16 = {
Call(_31 = dump_var(19_usize, 11_usize, Move(_11), 8_usize, Move(_8), 1_usize, Move(_1), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(19_usize, 18_usize, Move(_18), 26_usize, Move(_26), 25_usize, Move(_25), 19_usize, Move(_19)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_31 = dump_var(19_usize, 6_usize, Move(_6), 28_usize, Move(_28), 32_usize, _32, 32_usize, _32), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(8500605054936073504_u64), std::hint::black_box(1956133865273176784928886808025221065_u128), std::hint::black_box(9223372036854775807_isize), std::hint::black_box((-31_i8)), std::hint::black_box(13303_i16), std::hint::black_box(1631212940_i32), std::hint::black_box(2639201897431255454_i64), std::hint::black_box((-88789933652512400208139301259136380957_i128)), std::hint::black_box(6_usize), std::hint::black_box(157_u8), std::hint::black_box(40874_u16));
                
            }
impl PrintFDebug for Adt34{
	unsafe fn printf_debug(&self){unsafe{printf("Adt34::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt34 {
Variant0{
fld0: bool,
fld1: i32,
fld2: *mut i16,
fld3: i128,
fld4: (u32,),

},
Variant1{
fld0: (bool, u128),
fld1: *const bool,
fld2: u64,
fld3: u8,
fld4: u32,
fld5: *mut (bool, u128),
fld6: i128,

},
Variant2{
fld0: usize,

},
Variant3{
fld0: (bool, u128),
fld1: (i128, u8, f64, i32),
fld2: *const f32,
fld3: i128,
fld4: u64,

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt42{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt42 {
fld0: u64,
fld1: f64,
fld2: isize,
fld3: (i128, u8, f64, i32),
fld4: i16,
fld5: i32,
}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: u64,
fld1: *mut (bool, u128),
fld2: isize,
fld3: u16,

},
Variant1{
fld0: bool,
fld1: u32,
fld2: (*const f32,),
fld3: f32,

},
Variant2{
fld0: u128,
fld1: *const bool,
fld2: *mut i16,
fld3: i8,
fld4: [u16; 3],
fld5: (char, u128, i32),

},
Variant3{
fld0: *mut (bool, u128),
fld1: [isize; 7],
fld2: i128,

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt60{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt60 {
fld0: u32,
fld1: u64,
fld2: isize,
fld3: *const f32,
fld4: i16,
fld5: [isize; 7],
}
impl PrintFDebug for Adt64{
	unsafe fn printf_debug(&self){unsafe{printf("Adt64::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt64 {
Variant0{
fld0: Adt42,
fld1: [u8; 1],

},
Variant1{
fld0: u64,
fld1: usize,
fld2: f32,
fld3: *const bool,
fld4: (u64, [u16; 3], f64, u8),
fld5: *const u32,
fld6: i64,

},
Variant2{
fld0: (i128, u8, f64, i32),
fld1: (u32,),
fld2: u64,
fld3: *mut Adt42,
fld4: Adt56,

},
Variant3{
fld0: [isize; 7],
fld1: [u16; 3],
fld2: (u64, [u16; 3], f64, u8),
fld3: *const f32,
fld4: [u8; 1],

}}
impl PrintFDebug for Adt68{
	unsafe fn printf_debug(&self){unsafe{printf("Adt68::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt68 {
Variant0{
fld0: Adt60,
fld1: u32,

},
Variant1{
fld0: Adt42,
fld1: char,
fld2: *const f32,
fld3: (u32,),
fld4: [char; 1],

}}
impl PrintFDebug for Adt78{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt78{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt78 {
fld0: ([isize; 7],),
fld1: *mut i16,
fld2: u64,
fld3: ([u8; 1], i32, (u64, [u16; 3], f64, u8)),
fld4: [u16; 3],
fld5: i32,
fld6: *const f32,
}
impl PrintFDebug for Adt81{
	unsafe fn printf_debug(&self){unsafe{printf("Adt81::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt81 {
Variant0{
fld0: Adt64,
fld1: f64,
fld2: *mut isize,
fld3: Adt42,
fld4: [i8; 2],
fld5: *mut (bool, u128),
fld6: i64,
fld7: [u16; 3],

},
Variant1{
fld0: [u16; 1],

}}

