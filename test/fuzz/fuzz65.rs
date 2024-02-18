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
pub fn fn0(mut _1: bool,mut _2: u8,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: u16,mut _8: i128,mut _9: usize) -> isize {
mir! {
type RET = isize;
let _10: (char, *const i128, i32);
let _11: char;
let _12: u8;
let _13: f64;
let _14: (f64, Adt22, &'static i8, (i128, u64, *mut f32, f32));
let _15: f64;
let _16: *const &'static &'static isize;
let _17: *const Adt20;
let _18: Adt22;
let _19: u64;
let _20: (Adt22, i64, isize);
let _21: [i32; 8];
let _22: *const *mut f32;
let _23: isize;
let _24: bool;
let _25: (char, *const i128, i32);
let _26: bool;
let _27: (f32,);
let _28: [i16; 2];
let _29: *mut Adt21;
let _30: *const [i16; 3];
let _31: (f32,);
let _32: (u8, usize);
let _33: Adt21;
let _34: &'static i8;
let _35: bool;
let _36: [u64; 7];
let _37: &'static f64;
let _38: (Adt53, Adt50, *const *mut f32, &'static i16);
let _39: isize;
let _40: [bool; 3];
let _41: f64;
let _42: i128;
let _43: &'static i64;
let _44: ();
let _45: ();
{
_8 = 109329943376899645871930028081319480512_i128 - (-69212326342554520644518953653638625845_i128);
RET = 12661_i16 as isize;
_12 = 6658387841541075538_usize as u8;
_3 = 8717683718916306225_u64 as isize;
RET = _3;
_12 = 72_u8;
_10.1 = core::ptr::addr_of!(_8);
_2 = !_12;
_7 = 39808_u16 >> _2;
_10.2 = -1771093068_i32;
_10.0 = '\u{be197}';
_10.1 = core::ptr::addr_of!(_8);
RET = _3;
Goto(bb1)
}
bb1 = {
_14.3.3 = 1597501252_u32 as f32;
_14.3.1 = !7570557452744763137_u64;
_4 = -(-85_i8);
RET = _3 + _3;
_6 = _7 as i32;
_14.3.2 = core::ptr::addr_of_mut!(_14.3.3);
_9 = 6_usize;
_13 = _14.3.1 as f64;
_10.2 = _6 - _6;
_5 = 9577_i16 << _8;
_10.0 = '\u{8c31a}';
_13 = _9 as f64;
_19 = _8 as u64;
_14.0 = -_13;
_11 = _10.0;
_14.3.3 = (-1262840775043767414_i64) as f32;
_14.3.2 = core::ptr::addr_of_mut!(_14.3.3);
_14.3.0 = _8;
Call(_8 = core::intrinsics::bswap(_14.3.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_20.1 = -944675176959110843_i64;
_15 = -_13;
_8 = _14.3.0 * _14.3.0;
_2 = _12 | _12;
_14.2 = &_4;
_10.1 = core::ptr::addr_of!(_14.3.0);
_21[_9] = _10.2;
_7 = !4924_u16;
_21 = [_10.2,_10.2,_10.2,_10.2,_10.2,_10.2,_6,_6];
RET = _2 as isize;
_12 = _2 ^ _2;
_23 = RET - RET;
_6 = !_10.2;
_22 = core::ptr::addr_of!(_14.3.2);
(*_22) = core::ptr::addr_of_mut!(_14.3.3);
_12 = true as u8;
_15 = -_13;
Call(_5 = core::intrinsics::transmute(_7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_22) = core::ptr::addr_of_mut!(_14.3.3);
_18 = Adt22::Variant1 { fld0: _21[_9],fld1: _11 };
(*_22) = core::ptr::addr_of_mut!(_14.3.3);
_14.3.0 = _8 >> _14.3.1;
_14.3.3 = 1112288302_u32 as f32;
_4 = !28_i8;
SetDiscriminant(_18, 0);
(*_22) = core::ptr::addr_of_mut!(_14.3.3);
_24 = _23 <= _23;
_13 = 265416921533909096847921213447596435826_u128 as f64;
place!(Field::<bool>(Variant(_18, 0), 0)) = _2 > _2;
_11 = _10.0;
(*_22) = core::ptr::addr_of_mut!(_14.3.3);
RET = _23;
_10.0 = _11;
_10.0 = _11;
_15 = _14.0 + _13;
(*_22) = core::ptr::addr_of_mut!(_14.3.3);
place!(Field::<i8>(Variant(_18, 0), 3)) = _4 & _4;
_19 = _14.3.1 + _14.3.1;
_20.0 = Adt22::Variant1 { fld0: _21[_9],fld1: _11 };
_25.1 = core::ptr::addr_of!(_8);
RET = _23 | _23;
RET = -_23;
_11 = Field::<char>(Variant(_20.0, 1), 1);
_20.2 = RET;
match _9 {
0 => bb1,
6 => bb4,
_ => bb2
}
}
bb4 = {
_8 = _14.3.0 ^ _14.3.0;
_14.0 = _14.3.1 as f64;
place!(Field::<Adt21>(Variant(_18, 0), 2)) = Adt21::Variant1 { fld0: _14.3.3,fld1: Move(_14.3.2),fld2: _15,fld3: _4,fld4: _7,fld5: _6,fld6: _20.1 };
(*_22) = core::ptr::addr_of_mut!(_14.3.3);
_18 = Move(_20.0);
_10.1 = Move(_25.1);
_10.1 = core::ptr::addr_of!(_8);
_13 = _14.0;
Goto(bb5)
}
bb5 = {
place!(Field::<char>(Variant(_18, 1), 1)) = _10.0;
_14.3.1 = _19 >> _23;
RET = _6 as isize;
_25.2 = _10.2 | _6;
_25.1 = core::ptr::addr_of!(_14.3.0);
_15 = _5 as f64;
_20.0 = Adt22::Variant1 { fld0: _25.2,fld1: Field::<char>(Variant(_18, 1), 1) };
Goto(bb6)
}
bb6 = {
_7 = _24 as u16;
_4 = (-18_i8) + (-31_i8);
_20.0 = Adt22::Variant1 { fld0: _25.2,fld1: _11 };
_25.0 = Field::<char>(Variant(_20.0, 1), 1);
_1 = !_24;
Goto(bb7)
}
bb7 = {
_1 = _24;
_23 = _3;
_3 = RET;
_14.2 = &_4;
_21 = [_6,Field::<i32>(Variant(_20.0, 1), 0),Field::<i32>(Variant(_20.0, 1), 0),Field::<i32>(Variant(_18, 1), 0),_25.2,_10.2,_10.2,Field::<i32>(Variant(_20.0, 1), 0)];
_10 = (Field::<char>(Variant(_18, 1), 1), Move(_25.1), Field::<i32>(Variant(_20.0, 1), 0));
_14.3.0 = _8;
_2 = _12;
_4 = 120_i8 >> _25.2;
_15 = _14.0;
_25.2 = _21[_9];
_32.1 = _9 + _9;
(*_22) = core::ptr::addr_of_mut!(_31.0);
(*_22) = core::ptr::addr_of_mut!(_14.3.3);
_14.3.3 = _4 as f32;
_14.3.2 = core::ptr::addr_of_mut!(_27.0);
Goto(bb8)
}
bb8 = {
_15 = _13;
(*_22) = core::ptr::addr_of_mut!(_31.0);
_14.1 = Move(_20.0);
Call(_19 = fn1(Move(_14.1), _10.2, _14.3.0, _14.3.0, RET, _1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_32.1 = _11 as usize;
_4 = 1572213112_u32 as i8;
_14.3.3 = RET as f32;
match _9 {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb10,
_ => bb7
}
}
bb10 = {
_35 = _24;
_28 = [_5,_5];
_1 = !_24;
_20.1 = 53954729820482354_i64 << _7;
_26 = _24 | _1;
_23 = !_3;
_32.1 = _12 as usize;
match _9 {
0 => bb1,
1 => bb7,
6 => bb12,
_ => bb11
}
}
bb11 = {
(*_22) = core::ptr::addr_of_mut!(_14.3.3);
_18 = Adt22::Variant1 { fld0: _21[_9],fld1: _11 };
(*_22) = core::ptr::addr_of_mut!(_14.3.3);
_14.3.0 = _8 >> _14.3.1;
_14.3.3 = 1112288302_u32 as f32;
_4 = !28_i8;
SetDiscriminant(_18, 0);
(*_22) = core::ptr::addr_of_mut!(_14.3.3);
_24 = _23 <= _23;
_13 = 265416921533909096847921213447596435826_u128 as f64;
place!(Field::<bool>(Variant(_18, 0), 0)) = _2 > _2;
_11 = _10.0;
(*_22) = core::ptr::addr_of_mut!(_14.3.3);
RET = _23;
_10.0 = _11;
_10.0 = _11;
_15 = _14.0 + _13;
(*_22) = core::ptr::addr_of_mut!(_14.3.3);
place!(Field::<i8>(Variant(_18, 0), 3)) = _4 & _4;
_19 = _14.3.1 + _14.3.1;
_20.0 = Adt22::Variant1 { fld0: _21[_9],fld1: _11 };
_25.1 = core::ptr::addr_of!(_8);
RET = _23 | _23;
RET = -_23;
_11 = Field::<char>(Variant(_20.0, 1), 1);
_20.2 = RET;
match _9 {
0 => bb1,
6 => bb4,
_ => bb2
}
}
bb12 = {
_39 = _3 ^ _20.2;
_25.1 = Move(_10.1);
_14.0 = 3091138539_u32 as f64;
(*_22) = core::ptr::addr_of_mut!(_31.0);
_19 = _14.3.1 | _14.3.1;
_2 = 60747672_u32 as u8;
_27.0 = _14.3.3;
_3 = _19 as isize;
SetDiscriminant(_18, 0);
_14.3.3 = -_27.0;
_7 = _35 as u16;
_10.1 = Move(_25.1);
_25.0 = _10.0;
_22 = core::ptr::addr_of!((*_22));
place!(Field::<bool>(Variant(_18, 0), 0)) = !_24;
_31 = _27;
_24 = !Field::<bool>(Variant(_18, 0), 0);
_26 = Field::<bool>(Variant(_18, 0), 0);
place!(Field::<i8>(Variant(_18, 0), 3)) = _4 | _4;
_25 = (_10.0, Move(_10.1), _10.2);
_3 = _23;
_27 = (_31.0,);
_9 = 3822452873_u32 as usize;
_32 = (_12, _9);
_10.0 = _11;
_27 = (_31.0,);
Call(_2 = fn2(_14.3.0, _25.0, _39, Move(_14.3.2), _10.2, _4, Move(_25), _35, _26), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_31 = (_14.3.3,);
_9 = _32.1;
_14.3.0 = _32.1 as i128;
_10.1 = core::ptr::addr_of!(_14.3.0);
_20.1 = 5677297612759404598_i64 << _2;
(*_22) = core::ptr::addr_of_mut!(_14.3.3);
_38.2 = Move(_22);
_9 = _32.1 * _32.1;
_14.3.3 = _27.0;
Goto(bb14)
}
bb14 = {
_20.0 = Adt22::Variant1 { fld0: _6,fld1: _10.0 };
_34 = &_4;
_10.2 = _20.1 as i32;
_20.0 = Adt22::Variant1 { fld0: _10.2,fld1: _11 };
_25.0 = Field::<char>(Variant(_20.0, 1), 1);
_25.0 = _11;
_21 = [_10.2,Field::<i32>(Variant(_20.0, 1), 0),_10.2,Field::<i32>(Variant(_20.0, 1), 0),_10.2,Field::<i32>(Variant(_20.0, 1), 0),_10.2,_6];
_14.1 = Move(_20.0);
_29 = core::ptr::addr_of_mut!(_33);
_40 = [_24,_26,_35];
_37 = &_15;
_32.1 = _9;
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(0_usize, 12_usize, Move(_12), 1_usize, Move(_1), 4_usize, Move(_4), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(0_usize, 32_usize, Move(_32), 26_usize, Move(_26), 9_usize, Move(_9), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(0_usize, 3_usize, Move(_3), 23_usize, Move(_23), 45_usize, _45, 45_usize, _45), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: Adt22,mut _2: i32,mut _3: i128,mut _4: i128,mut _5: isize,mut _6: bool) -> u64 {
mir! {
type RET = u64;
let _7: f32;
let _8: isize;
let _9: isize;
let _10: isize;
let _11: Adt77;
let _12: u8;
let _13: *const Adt20;
let _14: isize;
let _15: char;
let _16: i128;
let _17: ([u64; 1], ((char, *const i128, i32),));
let _18: *const i16;
let _19: [i16; 7];
let _20: [i32; 8];
let _21: *const u32;
let _22: u16;
let _23: [bool; 3];
let _24: (&'static i8, *const *mut f32, usize, Adt45);
let _25: isize;
let _26: ([usize; 5], *const i16, (char, *const i128, i32), [u16; 3]);
let _27: f64;
let _28: [i16; 7];
let _29: ();
let _30: ();
{
_6 = true;
_2 = _4 as i32;
_2 = !Field::<i32>(Variant(_1, 1), 0);
_5 = !(-101_isize);
_5 = !9223372036854775807_isize;
RET = !4235178153562654949_u64;
_7 = RET as f32;
_6 = !false;
_4 = _3 << _3;
place!(Field::<i32>(Variant(_1, 1), 0)) = _2;
place!(Field::<char>(Variant(_1, 1), 1)) = '\u{6199f}';
RET = _4 as u64;
_4 = _3;
_1 = Adt22::Variant1 { fld0: _2,fld1: '\u{1a027}' };
RET = !14248906483867064644_u64;
RET = 420223557697820693_u64;
Call(_8 = core::intrinsics::bswap(_5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<char>(Variant(_1, 1), 1)) = '\u{dc023}';
_8 = -_5;
_2 = -Field::<i32>(Variant(_1, 1), 0);
_6 = false;
SetDiscriminant(_1, 0);
place!(Field::<f64>(Variant(_1, 0), 4)) = _7 as f64;
place!(Field::<bool>(Variant(_1, 0), 0)) = _6;
_9 = _2 as isize;
place!(Field::<i8>(Variant(_1, 0), 3)) = 46_i8;
_7 = 1912452947534115501_i64 as f32;
_7 = Field::<f64>(Variant(_1, 0), 4) as f32;
place!(Field::<bool>(Variant(_1, 0), 0)) = _6 ^ _6;
_2 = !2113553873_i32;
_6 = !Field::<bool>(Variant(_1, 0), 0);
Goto(bb2)
}
bb2 = {
place!(Field::<f64>(Variant(_1, 0), 4)) = 58802_u16 as f64;
_14 = _8;
place!(Field::<char>(Variant(_1, 0), 1)) = '\u{25eea}';
place!(Field::<f64>(Variant(_1, 0), 4)) = _7 as f64;
_1 = Adt22::Variant1 { fld0: _2,fld1: '\u{aba45}' };
place!(Field::<i32>(Variant(_1, 1), 0)) = -_2;
_12 = 14_u8;
_9 = _8;
_14 = _5 - _9;
RET = _12 as u64;
_16 = _4 ^ _4;
_10 = _8;
_14 = !_9;
_17.1.0.2 = _2;
_7 = 3397849230_u32 as f32;
_17.1.0.1 = core::ptr::addr_of!(_4);
_17.1.0.0 = '\u{2d8a}';
place!(Field::<i32>(Variant(_1, 1), 0)) = -_17.1.0.2;
_19 = [25208_i16,(-4747_i16),(-16368_i16),(-13319_i16),(-24709_i16),(-26766_i16),13155_i16];
_19 = [26752_i16,(-24201_i16),1983_i16,27159_i16,19_i16,31862_i16,20336_i16];
_17.0 = [RET];
match _12 {
0 => bb1,
1 => bb3,
2 => bb4,
14 => bb6,
_ => bb5
}
}
bb3 = {
place!(Field::<char>(Variant(_1, 1), 1)) = '\u{dc023}';
_8 = -_5;
_2 = -Field::<i32>(Variant(_1, 1), 0);
_6 = false;
SetDiscriminant(_1, 0);
place!(Field::<f64>(Variant(_1, 0), 4)) = _7 as f64;
place!(Field::<bool>(Variant(_1, 0), 0)) = _6;
_9 = _2 as isize;
place!(Field::<i8>(Variant(_1, 0), 3)) = 46_i8;
_7 = 1912452947534115501_i64 as f32;
_7 = Field::<f64>(Variant(_1, 0), 4) as f32;
place!(Field::<bool>(Variant(_1, 0), 0)) = _6 ^ _6;
_2 = !2113553873_i32;
_6 = !Field::<bool>(Variant(_1, 0), 0);
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
RET = !1405009086374103888_u64;
_17.1.0.2 = _2;
RET = _2 as u64;
_17.0 = [RET];
Goto(bb7)
}
bb7 = {
_10 = _5 | _8;
RET = 481833476696328660_u64;
match _12 {
0 => bb6,
1 => bb8,
2 => bb9,
3 => bb10,
14 => bb12,
_ => bb11
}
}
bb8 = {
RET = !1405009086374103888_u64;
_17.1.0.2 = _2;
RET = _2 as u64;
_17.0 = [RET];
Goto(bb7)
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
place!(Field::<f64>(Variant(_1, 0), 4)) = 58802_u16 as f64;
_14 = _8;
place!(Field::<char>(Variant(_1, 0), 1)) = '\u{25eea}';
place!(Field::<f64>(Variant(_1, 0), 4)) = _7 as f64;
_1 = Adt22::Variant1 { fld0: _2,fld1: '\u{aba45}' };
place!(Field::<i32>(Variant(_1, 1), 0)) = -_2;
_12 = 14_u8;
_9 = _8;
_14 = _5 - _9;
RET = _12 as u64;
_16 = _4 ^ _4;
_10 = _8;
_14 = !_9;
_17.1.0.2 = _2;
_7 = 3397849230_u32 as f32;
_17.1.0.1 = core::ptr::addr_of!(_4);
_17.1.0.0 = '\u{2d8a}';
place!(Field::<i32>(Variant(_1, 1), 0)) = -_17.1.0.2;
_19 = [25208_i16,(-4747_i16),(-16368_i16),(-13319_i16),(-24709_i16),(-26766_i16),13155_i16];
_19 = [26752_i16,(-24201_i16),1983_i16,27159_i16,19_i16,31862_i16,20336_i16];
_17.0 = [RET];
match _12 {
0 => bb1,
1 => bb3,
2 => bb4,
14 => bb6,
_ => bb5
}
}
bb12 = {
_2 = _17.1.0.2 << _16;
_17.1.0.1 = core::ptr::addr_of!(_16);
_14 = -_10;
_15 = _17.1.0.0;
_10 = !_14;
RET = 9760067925041543955_u64 & 8483581032973549534_u64;
place!(Field::<char>(Variant(_1, 1), 1)) = _15;
_17.1.0.2 = _2;
_1 = Adt22::Variant1 { fld0: _2,fld1: _15 };
_19 = [(-10348_i16),9688_i16,(-3229_i16),15938_i16,17015_i16,(-19256_i16),(-31614_i16)];
_20 = [Field::<i32>(Variant(_1, 1), 0),_17.1.0.2,Field::<i32>(Variant(_1, 1), 0),Field::<i32>(Variant(_1, 1), 0),Field::<i32>(Variant(_1, 1), 0),Field::<i32>(Variant(_1, 1), 0),_17.1.0.2,Field::<i32>(Variant(_1, 1), 0)];
_10 = _5 | _9;
_4 = !_16;
_14 = -_10;
_20 = [Field::<i32>(Variant(_1, 1), 0),Field::<i32>(Variant(_1, 1), 0),Field::<i32>(Variant(_1, 1), 0),_2,_2,Field::<i32>(Variant(_1, 1), 0),_17.1.0.2,Field::<i32>(Variant(_1, 1), 0)];
_10 = 37423870866727987343989454774812601006_u128 as isize;
_5 = _8 & _8;
RET = 7444572504034920315_u64;
_8 = 1540100655_u32 as isize;
_4 = _16 & _16;
_6 = false & true;
_19 = [31902_i16,3779_i16,(-24620_i16),31081_i16,30139_i16,(-6033_i16),6661_i16];
_12 = !222_u8;
_9 = _5;
_17.1.0.2 = !_2;
_17.1.0.2 = Field::<i32>(Variant(_1, 1), 0) - _2;
_14 = _9;
match RET {
0 => bb2,
1 => bb13,
2 => bb14,
7444572504034920315 => bb16,
_ => bb15
}
}
bb13 = {
RET = !1405009086374103888_u64;
_17.1.0.2 = _2;
RET = _2 as u64;
_17.0 = [RET];
Goto(bb7)
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_6 = true;
_25 = _7 as isize;
_25 = (-5123003468071125110_i64) as isize;
_24.2 = 9939776897851596271_usize;
_5 = _25 + _25;
_1 = Adt22::Variant1 { fld0: _2,fld1: _15 };
_9 = _14 >> _17.1.0.2;
_4 = _3;
_19 = [(-30823_i16),19395_i16,(-16842_i16),7187_i16,28131_i16,(-28067_i16),(-7817_i16)];
_17.0 = [RET];
_14 = _9;
RET = _17.1.0.2 as u64;
_9 = _6 as isize;
_23 = [_6,_6,_6];
_4 = -_16;
_23 = [_6,_6,_6];
_5 = _14 ^ _14;
_6 = !false;
_17.1.0.0 = Field::<char>(Variant(_1, 1), 1);
_17.1.0.2 = _7 as i32;
SetDiscriminant(_1, 1);
_9 = !_14;
_26.3 = [8018_u16,26266_u16,5907_u16];
_16 = -_3;
_26.2.1 = Move(_17.1.0.1);
Goto(bb17)
}
bb17 = {
Call(_29 = dump_var(1_usize, 20_usize, Move(_20), 14_usize, Move(_14), 9_usize, Move(_9), 10_usize, Move(_10)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(1_usize, 4_usize, Move(_4), 16_usize, Move(_16), 6_usize, Move(_6), 15_usize, Move(_15)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i128,mut _2: char,mut _3: isize,mut _4: *mut f32,mut _5: i32,mut _6: i8,mut _7: (char, *const i128, i32),mut _8: bool,mut _9: bool) -> u8 {
mir! {
type RET = u8;
let _10: u64;
let _11: [i8; 5];
let _12: ([i64; 7],);
let _13: [i8; 8];
let _14: ();
let _15: ();
{
RET = 102_u8;
_8 = _9;
RET = 3741488591671518022_u64 as u8;
_5 = 32901_u16 as i32;
_7.2 = _5;
RET = 249_u8;
_7.0 = _2;
_9 = _8;
_6 = 2_usize as i8;
_1 = (-13465858591816927248019477770182918639_i128) & 15767870529349499020555641287358810129_i128;
_8 = !_9;
_9 = _8;
_5 = _7.2;
_11 = [_6,_6,_6,_6,_6];
_6 = -(-17_i8);
Call(RET = fn3(_8, _7.0, _8, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = !_7.2;
_11 = [_6,_6,_6,_6,_6];
_6 = (-12_i8);
_3 = 296362303770031523851341023574006665832_u128 as isize;
Goto(bb2)
}
bb2 = {
Call(_14 = dump_var(2_usize, 3_usize, Move(_3), 9_usize, Move(_9), 5_usize, Move(_5), 2_usize, Move(_2)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: bool,mut _2: char,mut _3: bool,mut _4: isize) -> u8 {
mir! {
type RET = u8;
let _5: Adt31;
let _6: Adt31;
let _7: *const &'static &'static isize;
let _8: [u64; 7];
let _9: i128;
let _10: i128;
let _11: [usize; 5];
let _12: isize;
let _13: &'static Adt20;
let _14: (char, *const i128, i32);
let _15: *const [i16; 3];
let _16: &'static i16;
let _17: f64;
let _18: (*mut Adt21,);
let _19: [i16; 3];
let _20: (char, *const i128, i32);
let _21: (f32,);
let _22: [u16; 3];
let _23: (char, *const i128, i32);
let _24: ();
let _25: ();
{
RET = (-30663_i16) as u8;
_3 = _1;
_2 = '\u{ac071}';
RET = _4 as u8;
_1 = !_3;
_3 = RET < RET;
RET = !103_u8;
_4 = 9223372036854775807_isize & (-18_isize);
_1 = !_3;
_8 = [18143886633595134947_u64,11866036867722888890_u64,11004495526536960599_u64,7857615468417451498_u64,10140428073284645152_u64,11214980813860896600_u64,4635594163590272491_u64];
_2 = '\u{10093b}';
_1 = !_3;
_10 = 48160839377676334373522228538417568039_i128 * (-120414354545676722320121122778231387074_i128);
RET = (-7273792878975251261_i64) as u8;
Goto(bb1)
}
bb1 = {
_4 = 51_isize;
RET = 86_u8 >> _10;
RET = 739382037_u32 as u8;
_9 = _10;
match _4 {
51 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_3 = _1;
_4 = -(-9223372036854775808_isize);
_12 = 58753_u16 as isize;
_12 = !_4;
_9 = _10 & _10;
_2 = '\u{ea0b1}';
Call(_4 = fn4(), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_14.2 = 1554722509_i32 - 1357706923_i32;
_12 = _4;
_9 = -_10;
_14.2 = 1847193056_i32 << _12;
_11 = [9109818137213539873_usize,6887952867814987352_usize,14701976176195749043_usize,8697267173292236043_usize,7337141546909561511_usize];
RET = 75_u8;
_4 = _12;
_14.2 = !119833088_i32;
_11 = [14168470025876227077_usize,948373428127565094_usize,3_usize,13311794043116759999_usize,16354106486416475259_usize];
_11 = [0_usize,2_usize,3_usize,18139381243940973601_usize,9658101613556128395_usize];
_8 = [9567434388614626503_u64,7890300958416260434_u64,332969759909603995_u64,6628756584583686497_u64,9409713355274271472_u64,14643190186074843651_u64,14193330408677665198_u64];
Goto(bb5)
}
bb5 = {
_9 = _12 as i128;
_2 = '\u{102752}';
match RET {
0 => bb3,
1 => bb2,
2 => bb6,
75 => bb8,
_ => bb7
}
}
bb6 = {
_14.2 = 1554722509_i32 - 1357706923_i32;
_12 = _4;
_9 = -_10;
_14.2 = 1847193056_i32 << _12;
_11 = [9109818137213539873_usize,6887952867814987352_usize,14701976176195749043_usize,8697267173292236043_usize,7337141546909561511_usize];
RET = 75_u8;
_4 = _12;
_14.2 = !119833088_i32;
_11 = [14168470025876227077_usize,948373428127565094_usize,3_usize,13311794043116759999_usize,16354106486416475259_usize];
_11 = [0_usize,2_usize,3_usize,18139381243940973601_usize,9658101613556128395_usize];
_8 = [9567434388614626503_u64,7890300958416260434_u64,332969759909603995_u64,6628756584583686497_u64,9409713355274271472_u64,14643190186074843651_u64,14193330408677665198_u64];
Goto(bb5)
}
bb7 = {
_3 = _1;
_4 = -(-9223372036854775808_isize);
_12 = 58753_u16 as isize;
_12 = !_4;
_9 = _10 & _10;
_2 = '\u{ea0b1}';
Call(_4 = fn4(), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_14.0 = _2;
_20.0 = _2;
_12 = 13264079021790273623_u64 as isize;
RET = 162_u8;
RET = 49998_u16 as u8;
RET = !176_u8;
_21.0 = 209368173689909755148809726528977305417_u128 as f32;
_4 = _12;
_20.1 = core::ptr::addr_of!(_9);
_15 = core::ptr::addr_of!(_19);
Goto(bb9)
}
bb9 = {
_14.1 = Move(_20.1);
_14.0 = _2;
_2 = _20.0;
_8 = [10930764642797527499_u64,10231737425853946661_u64,11195075089383594837_u64,9214563039237588154_u64,6239918335354872944_u64,15855440752188613755_u64,5006827534697183965_u64];
_11 = [1_usize,3295619409191211130_usize,3_usize,4_usize,2_usize];
(*_15) = [(-182_i16),(-7482_i16),(-8248_i16)];
_15 = core::ptr::addr_of!((*_15));
_10 = _14.2 as i128;
_3 = _1;
_19 = [23317_i16,(-11265_i16),(-22620_i16)];
Call(RET = core::intrinsics::transmute(_3), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_21.0 = _9 as f32;
Goto(bb11)
}
bb11 = {
Call(_24 = dump_var(3_usize, 1_usize, Move(_1), 12_usize, Move(_12), 4_usize, Move(_4), 19_usize, Move(_19)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_24 = dump_var(3_usize, 9_usize, Move(_9), 25_usize, _25, 25_usize, _25, 25_usize, _25), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4() -> isize {
mir! {
type RET = isize;
let _1: (*mut Adt21,);
let _2: [i64; 5];
let _3: Adt22;
let _4: i8;
let _5: [i16; 7];
let _6: bool;
let _7: [u64; 1];
let _8: f64;
let _9: [u128; 4];
let _10: [i8; 5];
let _11: *const (&'static i8, *const *mut f32, usize, Adt45);
let _12: [i32; 8];
let _13: [u128; 4];
let _14: isize;
let _15: (*mut f32, u16, Adt22, i64);
let _16: [bool; 8];
let _17: [i16; 7];
let _18: char;
let _19: ((char, *const i128, i32),);
let _20: *mut *mut u32;
let _21: [i64; 5];
let _22: *mut &'static i8;
let _23: isize;
let _24: (f32,);
let _25: *const i128;
let _26: &'static i64;
let _27: *mut &'static i8;
let _28: *const &'static &'static isize;
let _29: &'static f64;
let _30: [u64; 7];
let _31: *mut u32;
let _32: ([i64; 7],);
let _33: isize;
let _34: &'static i8;
let _35: u8;
let _36: [i16; 2];
let _37: ();
let _38: ();
{
RET = (-2298_i16) as isize;
RET = (-6725460806449749698_i64) as isize;
RET = '\u{d79e0}' as isize;
RET = 9223372036854775807_isize;
RET = (-9223372036854775808_isize) - (-82_isize);
RET = -9223372036854775807_isize;
RET = 32_isize;
RET = (-9223372036854775808_isize) * 29_isize;
RET = (-64_isize);
RET = -(-9223372036854775808_isize);
RET = !9223372036854775807_isize;
_2 = [(-8462440397232859316_i64),(-1673449118253458590_i64),(-8154828053366411568_i64),3086769038001725602_i64,(-4234233871447467108_i64)];
_2 = [2697276722373531438_i64,(-7104459801478120531_i64),8951216429185821356_i64,(-8337994811817025990_i64),(-8159586117178715011_i64)];
_2 = [(-8252525714673661678_i64),5150880377021897201_i64,2631360792486819039_i64,(-6366202809026638583_i64),5289713714698791792_i64];
Goto(bb1)
}
bb1 = {
_2 = [(-2364142423852990675_i64),2761191063235087335_i64,4758083843972668715_i64,4345739962207022949_i64,(-4227246934313804200_i64)];
_4 = (-27_i8);
RET = 9223372036854775807_isize << _4;
Goto(bb2)
}
bb2 = {
_8 = _4 as f64;
_8 = 272842423076565067749253935039023852232_u128 as f64;
_3 = Adt22::Variant1 { fld0: (-997745288_i32),fld1: '\u{ba314}' };
_6 = RET > RET;
Goto(bb3)
}
bb3 = {
place!(Field::<char>(Variant(_3, 1), 1)) = '\u{b6e06}';
_5 = [28069_i16,26377_i16,(-27626_i16),(-22058_i16),7195_i16,(-21626_i16),3740_i16];
_5 = [5183_i16,14185_i16,(-457_i16),18207_i16,15369_i16,9468_i16,23919_i16];
_7 = [14235122717087575474_u64];
RET = (-94_isize);
_7 = [6230753420599647467_u64];
place!(Field::<i32>(Variant(_3, 1), 0)) = !1494835353_i32;
RET = (-32508_i16) as isize;
_3 = Adt22::Variant1 { fld0: (-1644485240_i32),fld1: '\u{1067ea}' };
_4 = (-70_i8);
_6 = RET != RET;
_3 = Adt22::Variant1 { fld0: 860542701_i32,fld1: '\u{4d614}' };
_9 = [181840781096682649559879295949305038819_u128,275588583671527204119685517196212118130_u128,288804185291659936202604719911244557431_u128,23835977733589728765147690110108948095_u128];
place!(Field::<char>(Variant(_3, 1), 1)) = '\u{23ae5}';
_6 = RET >= RET;
place!(Field::<i32>(Variant(_3, 1), 0)) = (-1187143036_i32) ^ 1237675272_i32;
_2 = [(-4104840562879639542_i64),145496693374364026_i64,1397601621882386939_i64,(-5450569856618400231_i64),(-4722417412909857309_i64)];
RET = (-9223372036854775808_isize);
_7 = [7852642173237719434_u64];
_5 = [22579_i16,21620_i16,23226_i16,(-12903_i16),29809_i16,16012_i16,(-15615_i16)];
RET = (-9223372036854775808_isize);
_9 = [261962390427871020327677755844966010022_u128,154455169637930948672433765960473329856_u128,252145818249723893959111926174690141596_u128,254916124597525306498091357650450894951_u128];
_10 = [_4,_4,_4,_4,_4];
RET = 9223372036854775807_isize | 40_isize;
_2 = [5396800175454977667_i64,(-7403830154210802044_i64),3409863864730882559_i64,1935159089254334696_i64,3588723133229916799_i64];
place!(Field::<i32>(Variant(_3, 1), 0)) = (-151358806553060784399161889168055033648_i128) as i32;
_6 = false ^ false;
_2 = [8869644553007315703_i64,(-1986964955482730754_i64),(-2193742701736128853_i64),3046207835855111786_i64,(-6734113734107540612_i64)];
match _4 {
0 => bb4,
340282366920938463463374607431768211386 => bb6,
_ => bb5
}
}
bb4 = {
_8 = _4 as f64;
_8 = 272842423076565067749253935039023852232_u128 as f64;
_3 = Adt22::Variant1 { fld0: (-997745288_i32),fld1: '\u{ba314}' };
_6 = RET > RET;
Goto(bb3)
}
bb5 = {
_2 = [(-2364142423852990675_i64),2761191063235087335_i64,4758083843972668715_i64,4345739962207022949_i64,(-4227246934313804200_i64)];
_4 = (-27_i8);
RET = 9223372036854775807_isize << _4;
Goto(bb2)
}
bb6 = {
RET = 9223372036854775807_isize;
_9 = [18362266733246120641707951336938085356_u128,65019874108531745717276881526059815313_u128,32116308707840437864632368372975567037_u128,214921569613454310280005051057805005158_u128];
_7 = [9131475871361935976_u64];
_2 = [8537869779708333701_i64,2295624599455859554_i64,(-458249005439517938_i64),5774492050149845197_i64,7139011357286766446_i64];
_9 = [63190643173074636879460912768269411914_u128,38069924335044693817707681084753434532_u128,211065638047779354213214541974075631597_u128,62230723075522500579180224064370317580_u128];
_4 = !(-96_i8);
RET = (-9223372036854775808_isize);
RET = !9223372036854775807_isize;
_12 = [Field::<i32>(Variant(_3, 1), 0),Field::<i32>(Variant(_3, 1), 0),Field::<i32>(Variant(_3, 1), 0),Field::<i32>(Variant(_3, 1), 0),Field::<i32>(Variant(_3, 1), 0),Field::<i32>(Variant(_3, 1), 0),Field::<i32>(Variant(_3, 1), 0),Field::<i32>(Variant(_3, 1), 0)];
_9 = [299407438736880367481212858464945550369_u128,37372381437267546551425767226556934915_u128,48291213837994489183811933141379602526_u128,224748944629868188374600461214641192042_u128];
_6 = !true;
_16 = [_6,_6,_6,_6,_6,_6,_6,_6];
_9 = [263316127894285779659659252200993499786_u128,192644712702903995388102658381273803509_u128,39994225325978716746817878100788781725_u128,197547795700139814557326767774666318829_u128];
RET = (-9223372036854775808_isize);
_15.3 = (-2176346621420948450_i64);
_6 = !false;
_7 = [12698108426431523717_u64];
_9 = [110567494195265253936201693149792545638_u128,97263457572217883348458730810276522629_u128,317392802485209084791525664174620668571_u128,319279121768317431181962969572598391325_u128];
_8 = _15.3 as f64;
Call(_9 = fn5(_5, _2, Move(_3), _5, _5, RET, _2), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_14 = _4 as isize;
_15.2 = Adt22::Variant1 { fld0: 661502694_i32,fld1: '\u{88885}' };
_18 = '\u{a4915}';
match _15.3 {
340282366920938463461198260810347263006 => bb8,
_ => bb3
}
}
bb8 = {
place!(Field::<char>(Variant(_15.2, 1), 1)) = _18;
_3 = Adt22::Variant1 { fld0: (-344468006_i32),fld1: Field::<char>(Variant(_15.2, 1), 1) };
place!(Field::<char>(Variant(_3, 1), 1)) = _18;
_9 = [79813625173846782997179973280641552345_u128,165933374397411082005553366447816560301_u128,224257165039700474217062911840456566436_u128,36453108844681060554564515450980599950_u128];
_15.2 = Adt22::Variant1 { fld0: (-1437943096_i32),fld1: Field::<char>(Variant(_3, 1), 1) };
place!(Field::<i32>(Variant(_3, 1), 0)) = (-827134561_i32) & 682350101_i32;
SetDiscriminant(_3, 0);
_13 = [66262613337071378167406915167435526281_u128,7303977799510206764757528729284337683_u128,238118719656951690095057736657491521864_u128,189541809210720916087376662579524451336_u128];
place!(Field::<char>(Variant(_3, 0), 1)) = Field::<char>(Variant(_15.2, 1), 1);
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463454151235394913435648 => bb9,
_ => bb6
}
}
bb9 = {
place!(Field::<f64>(Variant(_3, 0), 4)) = -_8;
_4 = (-105_i8);
_13 = _9;
_5 = [30190_i16,(-23317_i16),5599_i16,(-5627_i16),28396_i16,(-11584_i16),(-10485_i16)];
_12 = [(-1435939002_i32),465391777_i32,(-510221027_i32),(-2052171096_i32),(-363928392_i32),109326541_i32,(-1811697291_i32),(-69708899_i32)];
Goto(bb10)
}
bb10 = {
place!(Field::<char>(Variant(_15.2, 1), 1)) = Field::<char>(Variant(_3, 0), 1);
_15.1 = 47072_u16;
_12 = [(-1302596993_i32),(-862270095_i32),1286121992_i32,1947795570_i32,(-455978442_i32),1300570390_i32,(-1441513375_i32),(-1144342746_i32)];
RET = _14;
_19.0.2 = 883053806_i32 - (-682958275_i32);
_21 = [_15.3,_15.3,_15.3,_15.3,_15.3];
_19.0.0 = Field::<char>(Variant(_15.2, 1), 1);
place!(Field::<i32>(Variant(_15.2, 1), 0)) = -_19.0.2;
_2 = _21;
_7 = [6419346970710680654_u64];
_9 = [184634680137692868453211236829985925987_u128,257282633179428761187885137682648061697_u128,278920754925486254870481457894130737581_u128,120578405727292114462675495019783776034_u128];
_16 = [_6,_6,_6,_6,_6,_6,_6,_6];
_12 = [_19.0.2,Field::<i32>(Variant(_15.2, 1), 0),Field::<i32>(Variant(_15.2, 1), 0),Field::<i32>(Variant(_15.2, 1), 0),_19.0.2,_19.0.2,Field::<i32>(Variant(_15.2, 1), 0),_19.0.2];
_19.0.2 = Field::<i32>(Variant(_15.2, 1), 0);
place!(Field::<bool>(Variant(_3, 0), 0)) = !_6;
RET = _14 << Field::<i32>(Variant(_15.2, 1), 0);
_6 = Field::<bool>(Variant(_3, 0), 0) | Field::<bool>(Variant(_3, 0), 0);
_3 = Move(_15.2);
_23 = _14;
_14 = RET << _15.1;
_10 = [_4,_4,_4,_4,_4];
_23 = _14 | RET;
_15.2 = Move(_3);
_9 = [67379092581443684142606313559611983658_u128,4875512161417283443388646764058727285_u128,158965769195243428724763942676538436619_u128,106707130473089416053948183076144009793_u128];
_2 = [_15.3,_15.3,_15.3,_15.3,_15.3];
_19.0.2 = _15.3 as i32;
_14 = _6 as isize;
_15.0 = core::ptr::addr_of_mut!(_24.0);
_18 = Field::<char>(Variant(_15.2, 1), 1);
_24.0 = _23 as f32;
Goto(bb11)
}
bb11 = {
_13 = [233045515640503244277568190538968205243_u128,48532329786262881094375128303858628455_u128,155561601965148143548245815877947453190_u128,25464596135459834228032343497643900306_u128];
_12 = [Field::<i32>(Variant(_15.2, 1), 0),Field::<i32>(Variant(_15.2, 1), 0),_19.0.2,Field::<i32>(Variant(_15.2, 1), 0),_19.0.2,_19.0.2,Field::<i32>(Variant(_15.2, 1), 0),_19.0.2];
_10 = [_4,_4,_4,_4,_4];
_18 = _19.0.0;
_26 = &_15.3;
_6 = true ^ false;
_4 = (-22_i8) * (-17_i8);
_15.3 = (-8530731666184465967_i64) & 6618404243230191116_i64;
_12 = [_19.0.2,_19.0.2,_19.0.2,_19.0.2,Field::<i32>(Variant(_15.2, 1), 0),Field::<i32>(Variant(_15.2, 1), 0),Field::<i32>(Variant(_15.2, 1), 0),_19.0.2];
_2 = [_15.3,_15.3,_15.3,_15.3,_15.3];
_6 = !true;
_8 = 153216427376035097236858845680329819532_i128 as f64;
SetDiscriminant(_15.2, 0);
_17 = [(-14993_i16),(-25806_i16),20784_i16,(-16749_i16),(-8105_i16),25066_i16,18088_i16];
place!(Field::<f64>(Variant(_15.2, 0), 4)) = _24.0 as f64;
_15.1 = 60133_u16;
place!(Field::<char>(Variant(_15.2, 0), 1)) = _19.0.0;
_6 = true;
_15.2 = Adt22::Variant1 { fld0: _19.0.2,fld1: _18 };
_29 = &_8;
_7 = [204654743514259674_u64];
match _15.1 {
60133 => bb12,
_ => bb9
}
}
bb12 = {
SetDiscriminant(_15.2, 0);
_15.3 = 4_usize as i64;
_9 = [323215797967244073847191836377591262910_u128,283054449573366448397527243691744701559_u128,184658242547532559726704893474064740271_u128,340156992808243831905986869772545390426_u128];
_15.3 = (-2044567807698008625_i64) & (-7484649069476957818_i64);
place!(Field::<f64>(Variant(_15.2, 0), 4)) = (*_29) + (*_29);
_13 = [296953726444301646421811926034145719739_u128,149466459163851285940782178497116830977_u128,136946650532331462235271877528329065945_u128,119197142051475161715919724840016180722_u128];
place!(Field::<char>(Variant(_15.2, 0), 1)) = _19.0.0;
_19.0.0 = Field::<char>(Variant(_15.2, 0), 1);
_4 = _6 as i8;
_14 = 3_usize as isize;
match _15.1 {
0 => bb5,
1 => bb8,
2 => bb3,
3 => bb13,
4 => bb14,
60133 => bb16,
_ => bb15
}
}
bb13 = {
_8 = _4 as f64;
_8 = 272842423076565067749253935039023852232_u128 as f64;
_3 = Adt22::Variant1 { fld0: (-997745288_i32),fld1: '\u{ba314}' };
_6 = RET > RET;
Goto(bb3)
}
bb14 = {
_2 = [(-2364142423852990675_i64),2761191063235087335_i64,4758083843972668715_i64,4345739962207022949_i64,(-4227246934313804200_i64)];
_4 = (-27_i8);
RET = 9223372036854775807_isize << _4;
Goto(bb2)
}
bb15 = {
RET = 9223372036854775807_isize;
_9 = [18362266733246120641707951336938085356_u128,65019874108531745717276881526059815313_u128,32116308707840437864632368372975567037_u128,214921569613454310280005051057805005158_u128];
_7 = [9131475871361935976_u64];
_2 = [8537869779708333701_i64,2295624599455859554_i64,(-458249005439517938_i64),5774492050149845197_i64,7139011357286766446_i64];
_9 = [63190643173074636879460912768269411914_u128,38069924335044693817707681084753434532_u128,211065638047779354213214541974075631597_u128,62230723075522500579180224064370317580_u128];
_4 = !(-96_i8);
RET = (-9223372036854775808_isize);
RET = !9223372036854775807_isize;
_12 = [Field::<i32>(Variant(_3, 1), 0),Field::<i32>(Variant(_3, 1), 0),Field::<i32>(Variant(_3, 1), 0),Field::<i32>(Variant(_3, 1), 0),Field::<i32>(Variant(_3, 1), 0),Field::<i32>(Variant(_3, 1), 0),Field::<i32>(Variant(_3, 1), 0),Field::<i32>(Variant(_3, 1), 0)];
_9 = [299407438736880367481212858464945550369_u128,37372381437267546551425767226556934915_u128,48291213837994489183811933141379602526_u128,224748944629868188374600461214641192042_u128];
_6 = !true;
_16 = [_6,_6,_6,_6,_6,_6,_6,_6];
_9 = [263316127894285779659659252200993499786_u128,192644712702903995388102658381273803509_u128,39994225325978716746817878100788781725_u128,197547795700139814557326767774666318829_u128];
RET = (-9223372036854775808_isize);
_15.3 = (-2176346621420948450_i64);
_6 = !false;
_7 = [12698108426431523717_u64];
_9 = [110567494195265253936201693149792545638_u128,97263457572217883348458730810276522629_u128,317392802485209084791525664174620668571_u128,319279121768317431181962969572598391325_u128];
_8 = _15.3 as f64;
Call(_9 = fn5(_5, _2, Move(_3), _5, _5, RET, _2), ReturnTo(bb7), UnwindUnreachable())
}
bb16 = {
_21 = _2;
_7 = [12837362171352213781_u64];
_5 = [(-5804_i16),31528_i16,8043_i16,(-12075_i16),26413_i16,(-8561_i16),29632_i16];
_30 = [2997304021086633591_u64,7051770373265998999_u64,3165992641703246221_u64,7795416028615669421_u64,3153049405725444647_u64,15332988676586484451_u64,5300319218386239778_u64];
_15.0 = core::ptr::addr_of_mut!(_24.0);
_5 = _17;
_26 = &_15.3;
_19.0.2 = (-1986788485_i32) * (-2029410217_i32);
_6 = !false;
place!(Field::<bool>(Variant(_15.2, 0), 0)) = !_6;
place!(Field::<i8>(Variant(_15.2, 0), 3)) = _4;
_30 = [845004051838573212_u64,17077049383113366005_u64,14504992916719629896_u64,15218929391887403932_u64,16982076369382881990_u64,7986173685045872806_u64,17740053022762987064_u64];
_15.1 = !39464_u16;
_19.0.0 = _18;
_15.0 = core::ptr::addr_of_mut!(_24.0);
_22 = core::ptr::addr_of_mut!(_34);
(*_22) = &_4;
_20 = core::ptr::addr_of_mut!(_31);
RET = _23 - _23;
_24.0 = _15.3 as f32;
RET = _23 & _23;
_15.1 = 57760_u16;
_27 = core::ptr::addr_of_mut!((*_22));
_15.0 = core::ptr::addr_of_mut!(_24.0);
_21 = [_15.3,_15.3,(*_26),_15.3,(*_26)];
_36 = [18205_i16,31740_i16];
(*_22) = &(*_34);
Goto(bb17)
}
bb17 = {
Call(_37 = dump_var(4_usize, 9_usize, Move(_9), 6_usize, Move(_6), 10_usize, Move(_10), 30_usize, Move(_30)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(4_usize, 36_usize, Move(_36), 14_usize, Move(_14), 23_usize, Move(_23), 17_usize, Move(_17)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [i16; 7],mut _2: [i64; 5],mut _3: Adt22,mut _4: [i16; 7],mut _5: [i16; 7],mut _6: isize,mut _7: [i64; 5]) -> [u128; 4] {
mir! {
type RET = [u128; 4];
let _8: i64;
let _9: bool;
let _10: [usize; 5];
let _11: Adt50;
let _12: char;
let _13: isize;
let _14: [i8; 8];
let _15: isize;
let _16: bool;
let _17: usize;
let _18: isize;
let _19: [usize; 5];
let _20: char;
let _21: f32;
let _22: char;
let _23: Adt50;
let _24: *mut *mut u32;
let _25: u8;
let _26: *mut f32;
let _27: i32;
let _28: char;
let _29: [u32; 1];
let _30: u64;
let _31: f32;
let _32: ();
let _33: ();
{
_3 = Adt22::Variant1 { fld0: (-1757199621_i32),fld1: '\u{102d4}' };
_3 = Adt22::Variant1 { fld0: (-1220522634_i32),fld1: '\u{3e15e}' };
Goto(bb1)
}
bb1 = {
_1 = _5;
_7 = [7023443792500141146_i64,3162030860363504016_i64,6777053490539682628_i64,(-195439047300233727_i64),4264888753765778630_i64];
_7 = [(-7315184209663043144_i64),1387387894072857401_i64,8352172600258868516_i64,(-3522811855966384981_i64),(-6353121382170679163_i64)];
place!(Field::<i32>(Variant(_3, 1), 0)) = 1174461680_i32 & 1215078354_i32;
_7 = [3312648525127931923_i64,(-5042975577814127071_i64),1566338214034038573_i64,2095348917941583676_i64,(-5708008377162122159_i64)];
_9 = false & true;
place!(Field::<i32>(Variant(_3, 1), 0)) = !(-2104255043_i32);
Goto(bb2)
}
bb2 = {
place!(Field::<char>(Variant(_3, 1), 1)) = '\u{f7a2}';
_12 = Field::<char>(Variant(_3, 1), 1);
_10 = [1994693634027728229_usize,0_usize,1805302871194808064_usize,3639618463525251543_usize,18271239189470724291_usize];
SetDiscriminant(_3, 0);
place!(Field::<bool>(Variant(_3, 0), 0)) = _9 ^ _9;
RET = [23461433237201703166663179298678421315_u128,248769855546900831968673811306595768165_u128,21912123976544221344447097663941020033_u128,221338255472257131294513532723080522568_u128];
_4 = _5;
RET = [104396543245191700955206054937563436243_u128,327164866024976860208974182093772951277_u128,111255798249439468047837173535864448475_u128,115906387628838667873301989417949662083_u128];
_12 = '\u{4183d}';
RET = [338307663559598348541709573018354960671_u128,209609804165999689045349301705114684836_u128,227114175404291746211462954794225782951_u128,8747303967661581536821978508199781699_u128];
_7 = [(-1043403678481641827_i64),(-3897432827078225164_i64),775412514147858351_i64,(-1182711572488060005_i64),(-5891420646417067644_i64)];
place!(Field::<char>(Variant(_3, 0), 1)) = _12;
_16 = Field::<bool>(Variant(_3, 0), 0) | Field::<bool>(Variant(_3, 0), 0);
_14 = [79_i8,(-100_i8),104_i8,(-64_i8),88_i8,111_i8,61_i8,(-45_i8)];
_14 = [6_i8,(-99_i8),71_i8,(-4_i8),(-39_i8),6_i8,46_i8,(-53_i8)];
RET = [171083203445377200543693648476440925278_u128,217128893949383086084562783104953920737_u128,334211057576363789517984110670798321666_u128,167368587515549957864548608970518969808_u128];
place!(Field::<f64>(Variant(_3, 0), 4)) = 203_u8 as f64;
_13 = Field::<char>(Variant(_3, 0), 1) as isize;
Goto(bb3)
}
bb3 = {
_20 = Field::<char>(Variant(_3, 0), 1);
_19 = [9592515450961825322_usize,1_usize,2_usize,4_usize,12723767968482343313_usize];
match _6 {
0 => bb4,
1 => bb5,
2 => bb6,
340282366920938463454151235394913435648 => bb8,
_ => bb7
}
}
bb4 = {
place!(Field::<char>(Variant(_3, 1), 1)) = '\u{f7a2}';
_12 = Field::<char>(Variant(_3, 1), 1);
_10 = [1994693634027728229_usize,0_usize,1805302871194808064_usize,3639618463525251543_usize,18271239189470724291_usize];
SetDiscriminant(_3, 0);
place!(Field::<bool>(Variant(_3, 0), 0)) = _9 ^ _9;
RET = [23461433237201703166663179298678421315_u128,248769855546900831968673811306595768165_u128,21912123976544221344447097663941020033_u128,221338255472257131294513532723080522568_u128];
_4 = _5;
RET = [104396543245191700955206054937563436243_u128,327164866024976860208974182093772951277_u128,111255798249439468047837173535864448475_u128,115906387628838667873301989417949662083_u128];
_12 = '\u{4183d}';
RET = [338307663559598348541709573018354960671_u128,209609804165999689045349301705114684836_u128,227114175404291746211462954794225782951_u128,8747303967661581536821978508199781699_u128];
_7 = [(-1043403678481641827_i64),(-3897432827078225164_i64),775412514147858351_i64,(-1182711572488060005_i64),(-5891420646417067644_i64)];
place!(Field::<char>(Variant(_3, 0), 1)) = _12;
_16 = Field::<bool>(Variant(_3, 0), 0) | Field::<bool>(Variant(_3, 0), 0);
_14 = [79_i8,(-100_i8),104_i8,(-64_i8),88_i8,111_i8,61_i8,(-45_i8)];
_14 = [6_i8,(-99_i8),71_i8,(-4_i8),(-39_i8),6_i8,46_i8,(-53_i8)];
RET = [171083203445377200543693648476440925278_u128,217128893949383086084562783104953920737_u128,334211057576363789517984110670798321666_u128,167368587515549957864548608970518969808_u128];
place!(Field::<f64>(Variant(_3, 0), 4)) = 203_u8 as f64;
_13 = Field::<char>(Variant(_3, 0), 1) as isize;
Goto(bb3)
}
bb5 = {
_1 = _5;
_7 = [7023443792500141146_i64,3162030860363504016_i64,6777053490539682628_i64,(-195439047300233727_i64),4264888753765778630_i64];
_7 = [(-7315184209663043144_i64),1387387894072857401_i64,8352172600258868516_i64,(-3522811855966384981_i64),(-6353121382170679163_i64)];
place!(Field::<i32>(Variant(_3, 1), 0)) = 1174461680_i32 & 1215078354_i32;
_7 = [3312648525127931923_i64,(-5042975577814127071_i64),1566338214034038573_i64,2095348917941583676_i64,(-5708008377162122159_i64)];
_9 = false & true;
place!(Field::<i32>(Variant(_3, 1), 0)) = !(-2104255043_i32);
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
place!(Field::<char>(Variant(_3, 0), 1)) = _20;
place!(Field::<f64>(Variant(_3, 0), 4)) = 48855317_i32 as f64;
_6 = _13;
_9 = Field::<bool>(Variant(_3, 0), 0) & _16;
RET = [75901519220118269001424365904551701857_u128,219463975905095843087757640539799174366_u128,81045334107762728781420492699923358016_u128,93557659264290175593669176639386089467_u128];
_10 = [14612345657512431434_usize,12538718578599905270_usize,4_usize,14318883199846920278_usize,16016129302223465076_usize];
_19 = [4_usize,3742891600616745282_usize,15238647751041389487_usize,8683525822189818650_usize,3_usize];
_10 = [17726823215579909571_usize,3_usize,3181407611259276731_usize,6_usize,6_usize];
_16 = !_9;
_12 = _20;
RET = [280409669460738465723114893427013053116_u128,165828234116661651751131649660690864648_u128,14727616395841906701363885405368781267_u128,126715522989851937363612493329244556450_u128];
Call(place!(Field::<char>(Variant(_3, 0), 1)) = fn6(_5, _19, _14, _10, _9, _16, _9), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_3 = Adt22::Variant1 { fld0: 1818806801_i32,fld1: _20 };
_3 = Adt22::Variant1 { fld0: 1328023930_i32,fld1: _20 };
_17 = !2_usize;
_21 = 2172236381070392894_u64 as f32;
_17 = _12 as usize;
RET = [147289114061745372581023032684023897875_u128,280256821268688826776299934319075441272_u128,108114382207914572534405012523029811531_u128,216716656018570369887881319466620375375_u128];
_15 = _6;
_8 = 37858295344231752261375473359120933673_u128 as i64;
_7 = [_8,_8,_8,_8,_8];
place!(Field::<i32>(Variant(_3, 1), 0)) = -(-1832446956_i32);
Goto(bb10)
}
bb10 = {
_25 = 108_u8;
SetDiscriminant(_3, 1);
place!(Field::<i32>(Variant(_3, 1), 0)) = 408494591_i32;
match _25 {
0 => bb5,
1 => bb9,
2 => bb4,
3 => bb11,
4 => bb12,
108 => bb14,
_ => bb13
}
}
bb11 = {
_3 = Adt22::Variant1 { fld0: 1818806801_i32,fld1: _20 };
_3 = Adt22::Variant1 { fld0: 1328023930_i32,fld1: _20 };
_17 = !2_usize;
_21 = 2172236381070392894_u64 as f32;
_17 = _12 as usize;
RET = [147289114061745372581023032684023897875_u128,280256821268688826776299934319075441272_u128,108114382207914572534405012523029811531_u128,216716656018570369887881319466620375375_u128];
_15 = _6;
_8 = 37858295344231752261375473359120933673_u128 as i64;
_7 = [_8,_8,_8,_8,_8];
place!(Field::<i32>(Variant(_3, 1), 0)) = -(-1832446956_i32);
Goto(bb10)
}
bb12 = {
place!(Field::<char>(Variant(_3, 1), 1)) = '\u{f7a2}';
_12 = Field::<char>(Variant(_3, 1), 1);
_10 = [1994693634027728229_usize,0_usize,1805302871194808064_usize,3639618463525251543_usize,18271239189470724291_usize];
SetDiscriminant(_3, 0);
place!(Field::<bool>(Variant(_3, 0), 0)) = _9 ^ _9;
RET = [23461433237201703166663179298678421315_u128,248769855546900831968673811306595768165_u128,21912123976544221344447097663941020033_u128,221338255472257131294513532723080522568_u128];
_4 = _5;
RET = [104396543245191700955206054937563436243_u128,327164866024976860208974182093772951277_u128,111255798249439468047837173535864448475_u128,115906387628838667873301989417949662083_u128];
_12 = '\u{4183d}';
RET = [338307663559598348541709573018354960671_u128,209609804165999689045349301705114684836_u128,227114175404291746211462954794225782951_u128,8747303967661581536821978508199781699_u128];
_7 = [(-1043403678481641827_i64),(-3897432827078225164_i64),775412514147858351_i64,(-1182711572488060005_i64),(-5891420646417067644_i64)];
place!(Field::<char>(Variant(_3, 0), 1)) = _12;
_16 = Field::<bool>(Variant(_3, 0), 0) | Field::<bool>(Variant(_3, 0), 0);
_14 = [79_i8,(-100_i8),104_i8,(-64_i8),88_i8,111_i8,61_i8,(-45_i8)];
_14 = [6_i8,(-99_i8),71_i8,(-4_i8),(-39_i8),6_i8,46_i8,(-53_i8)];
RET = [171083203445377200543693648476440925278_u128,217128893949383086084562783104953920737_u128,334211057576363789517984110670798321666_u128,167368587515549957864548608970518969808_u128];
place!(Field::<f64>(Variant(_3, 0), 4)) = 203_u8 as f64;
_13 = Field::<char>(Variant(_3, 0), 1) as isize;
Goto(bb3)
}
bb13 = {
Return()
}
bb14 = {
_25 = 252_u8 + 249_u8;
_27 = !Field::<i32>(Variant(_3, 1), 0);
place!(Field::<i32>(Variant(_3, 1), 0)) = 46295_u16 as i32;
_5 = [(-17272_i16),(-15047_i16),28112_i16,4106_i16,26676_i16,(-28912_i16),(-6255_i16)];
RET = [47771874693211282034591437092718867280_u128,326635277769585048419749754798716409837_u128,215389600823099830353917709205335739169_u128,288717239754072709986153296954076551629_u128];
_15 = 9544344499473176738_u64 as isize;
place!(Field::<char>(Variant(_3, 1), 1)) = _12;
_25 = 211_u8;
_2 = [_8,_8,_8,_8,_8];
_26 = core::ptr::addr_of_mut!(_21);
_13 = _6 ^ _15;
_16 = _9;
RET = [51510869319022263889799274054882510166_u128,64792036687485195322483130055349274648_u128,198494547851060075621040945345529561812_u128,335993106426857500589522585350281903192_u128];
_16 = _9 <= _9;
_20 = _12;
_28 = Field::<char>(Variant(_3, 1), 1);
_4 = _5;
(*_26) = _8 as f32;
_6 = (-11579_i16) as isize;
(*_26) = 24200_i16 as f32;
_30 = !15111215330693908251_u64;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(5_usize, 14_usize, Move(_14), 9_usize, Move(_9), 12_usize, Move(_12), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(5_usize, 8_usize, Move(_8), 17_usize, Move(_17), 4_usize, Move(_4), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(5_usize, 15_usize, Move(_15), 27_usize, Move(_27), 33_usize, _33, 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: [i16; 7],mut _2: [usize; 5],mut _3: [i8; 8],mut _4: [usize; 5],mut _5: bool,mut _6: bool,mut _7: bool) -> char {
mir! {
type RET = char;
let _8: *mut *mut u32;
let _9: (f32,);
let _10: &'static isize;
let _11: (f64, Adt22, &'static i8, (i128, u64, *mut f32, f32));
let _12: Adt21;
let _13: *const i16;
let _14: ([u64; 1], ((char, *const i128, i32),));
let _15: isize;
let _16: (Adt22, i64, isize);
let _17: (char, *const i128, i32);
let _18: isize;
let _19: [i64; 7];
let _20: (*const i16,);
let _21: isize;
let _22: &'static Adt20;
let _23: Adt21;
let _24: [i64; 7];
let _25: isize;
let _26: isize;
let _27: *const i128;
let _28: [bool; 3];
let _29: isize;
let _30: *mut *mut f32;
let _31: f32;
let _32: ();
let _33: ();
{
_6 = !_7;
RET = '\u{40d13}';
_2 = [1579779775627395315_usize,2294276633829665732_usize,4_usize,15742042115392627830_usize,6826259602700877466_usize];
_3 = [27_i8,71_i8,125_i8,41_i8,(-51_i8),(-126_i8),106_i8,(-47_i8)];
RET = '\u{75912}';
_4 = _2;
RET = '\u{b2b39}';
_7 = _6;
_3 = [114_i8,36_i8,(-88_i8),8_i8,(-23_i8),(-14_i8),113_i8,60_i8];
_5 = _7 == _7;
_1 = [(-30030_i16),31009_i16,9441_i16,15666_i16,29916_i16,24279_i16,14893_i16];
_7 = !_5;
_1 = [(-18402_i16),(-26794_i16),31113_i16,(-10207_i16),(-19492_i16),32749_i16,(-23887_i16)];
_6 = _5 == _5;
RET = '\u{5949b}';
_2 = [0_usize,7_usize,6_usize,1_usize,1_usize];
_3 = [66_i8,(-99_i8),101_i8,101_i8,(-44_i8),(-67_i8),18_i8,(-28_i8)];
RET = '\u{1090a4}';
_2 = [3122783133929695562_usize,11419337234912329314_usize,8135475976447442403_usize,3_usize,3040136024962361627_usize];
_1 = [16447_i16,(-20214_i16),12808_i16,22091_i16,8437_i16,17032_i16,4211_i16];
_1 = [17321_i16,(-9234_i16),(-6299_i16),14806_i16,(-31954_i16),3589_i16,(-14704_i16)];
_3 = [102_i8,(-109_i8),(-67_i8),52_i8,(-23_i8),(-37_i8),(-40_i8),39_i8];
_11.3.3 = 123406972584841513845231851719560738647_i128 as f32;
_3 = [(-102_i8),(-100_i8),111_i8,119_i8,(-8_i8),(-2_i8),(-119_i8),(-21_i8)];
_7 = _6;
Goto(bb1)
}
bb1 = {
_11.3.0 = (-106465092656897833626049386957952319122_i128) * 144512052213084174604098475345901901797_i128;
Goto(bb2)
}
bb2 = {
_9 = (_11.3.3,);
_6 = !_7;
_11.3.1 = 2219507115253882077_u64 & 2178256663889568276_u64;
_11.3.0 = (-144693153745802459689715228475126271062_i128) | 127914797121351996686393047943102179817_i128;
_11.0 = (-137354957_i32) as f64;
Goto(bb3)
}
bb3 = {
_2 = [10896170836039602959_usize,2612785839546263246_usize,17144756898359757120_usize,3_usize,17961641958824056183_usize];
_11.3.1 = 17008962889765977889_u64 << _11.3.0;
_14.1.0.1 = core::ptr::addr_of!(_11.3.0);
_11.3.1 = _11.0 as u64;
_6 = _11.3.0 < _11.3.0;
_11.3.3 = -_9.0;
Goto(bb4)
}
bb4 = {
_4 = _2;
_14.1.0.2 = (-1200927993_i32) ^ (-1957588579_i32);
_15 = (-9223372036854775808_isize) | 124_isize;
_11.3.3 = _9.0 * _9.0;
_4 = [10414428477762970257_usize,0_usize,6_usize,1_usize,3763857246287323001_usize];
_3 = [(-15_i8),67_i8,(-73_i8),(-68_i8),(-67_i8),23_i8,(-76_i8),(-32_i8)];
_9.0 = _11.3.3;
_16.1 = (-557857732136332604_i64) ^ 5422403066540357879_i64;
Call(RET = fn7(_7), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_16.2 = _15 << _15;
_14.1.0.1 = core::ptr::addr_of!(_11.3.0);
_18 = 12131_u16 as isize;
_11.3.1 = 34_u8 as u64;
_17.2 = _14.1.0.2 << _15;
_17 = (RET, Move(_14.1.0.1), _14.1.0.2);
_11.3.1 = 9848864576053379851_u64 ^ 4986393168584133090_u64;
_10 = &_16.2;
_17.2 = _11.3.0 as i32;
_14.0 = [_11.3.1];
_6 = !_7;
_1 = [17412_i16,(-6781_i16),(-17635_i16),3884_i16,13941_i16,19091_i16,6553_i16];
_11.3.2 = core::ptr::addr_of_mut!(_11.3.3);
Goto(bb6)
}
bb6 = {
_11.3.3 = _9.0 * _9.0;
_11.0 = _11.3.0 as f64;
_14.1.0.2 = _17.2 ^ _17.2;
Goto(bb7)
}
bb7 = {
_14.1.0.1 = Move(_17.1);
_5 = !_6;
Goto(bb8)
}
bb8 = {
_14.1.0.1 = core::ptr::addr_of!(_11.3.0);
_17 = (RET, Move(_14.1.0.1), _14.1.0.2);
_16.0 = Adt22::Variant1 { fld0: _17.2,fld1: RET };
_16.0 = Adt22::Variant1 { fld0: _14.1.0.2,fld1: _17.0 };
_11.1 = Adt22::Variant1 { fld0: _14.1.0.2,fld1: _17.0 };
place!(Field::<char>(Variant(_16.0, 1), 1)) = Field::<char>(Variant(_11.1, 1), 1);
_14.1.0 = (Field::<char>(Variant(_16.0, 1), 1), Move(_17.1), _17.2);
_16.0 = Adt22::Variant1 { fld0: _17.2,fld1: _17.0 };
_17.1 = Move(_14.1.0.1);
Goto(bb9)
}
bb9 = {
_11.1 = Adt22::Variant1 { fld0: _14.1.0.2,fld1: RET };
_14.1.0.0 = RET;
_14.1.0 = (Field::<char>(Variant(_16.0, 1), 1), Move(_17.1), Field::<i32>(Variant(_16.0, 1), 0));
_25 = 5_usize as isize;
_9.0 = _11.3.3 * _11.3.3;
_21 = (*_10);
_14.1.0.1 = core::ptr::addr_of!(_11.3.0);
_15 = 20888_i16 as isize;
_2 = [12625997937694102800_usize,7004225072489516566_usize,5_usize,1_usize,11379654398768649917_usize];
place!(Field::<i32>(Variant(_11.1, 1), 0)) = Field::<i32>(Variant(_16.0, 1), 0) & _14.1.0.2;
_23 = Adt21::Variant1 { fld0: _9.0,fld1: Move(_11.3.2),fld2: _11.0,fld3: (-101_i8),fld4: 27244_u16,fld5: _17.2,fld6: _16.1 };
_9 = (Field::<f32>(Variant(_23, 1), 0),);
_17.1 = core::ptr::addr_of!(_11.3.0);
_11.3 = ((-18234238583320849141125922996680471374_i128), 9271038091500444861_u64, Move(Field::<*mut f32>(Variant(_23, 1), 1)), _9.0);
_26 = _21 & _16.2;
_16.2 = -_26;
place!(Field::<i64>(Variant(_23, 1), 6)) = _16.1 - _16.1;
_24 = [_16.1,Field::<i64>(Variant(_23, 1), 6),Field::<i64>(Variant(_23, 1), 6),Field::<i64>(Variant(_23, 1), 6),Field::<i64>(Variant(_23, 1), 6),_16.1,Field::<i64>(Variant(_23, 1), 6)];
place!(Field::<i32>(Variant(_16.0, 1), 0)) = Field::<i32>(Variant(_11.1, 1), 0) | _17.2;
_11.1 = Move(_16.0);
Goto(bb10)
}
bb10 = {
place!(Field::<i64>(Variant(_23, 1), 6)) = _16.1 + _16.1;
_1 = [28436_i16,1306_i16,(-23231_i16),(-20760_i16),32487_i16,27226_i16,(-16889_i16)];
_14.1.0.2 = _26 as i32;
_11.3.2 = core::ptr::addr_of_mut!(place!(Field::<f32>(Variant(_23, 1), 0)));
place!(Field::<i8>(Variant(_23, 1), 3)) = (-95_i8);
place!(Field::<char>(Variant(_11.1, 1), 1)) = RET;
_11.3.1 = 7862047558295360013_u64;
_21 = -_16.2;
_18 = -_26;
match _11.3.0 {
0 => bb9,
1 => bb6,
2 => bb5,
3 => bb4,
4 => bb11,
5 => bb12,
322048128337617614322248684435087740082 => bb14,
_ => bb13
}
}
bb11 = {
_11.3.0 = (-106465092656897833626049386957952319122_i128) * 144512052213084174604098475345901901797_i128;
Goto(bb2)
}
bb12 = {
_9 = (_11.3.3,);
_6 = !_7;
_11.3.1 = 2219507115253882077_u64 & 2178256663889568276_u64;
_11.3.0 = (-144693153745802459689715228475126271062_i128) | 127914797121351996686393047943102179817_i128;
_11.0 = (-137354957_i32) as f64;
Goto(bb3)
}
bb13 = {
_14.1.0.1 = Move(_17.1);
_5 = !_6;
Goto(bb8)
}
bb14 = {
_17.2 = Field::<i32>(Variant(_11.1, 1), 0) ^ Field::<i32>(Variant(_11.1, 1), 0);
_6 = _7 | _5;
_16 = (Move(_11.1), Field::<i64>(Variant(_23, 1), 6), _21);
RET = Field::<char>(Variant(_16.0, 1), 1);
_18 = !_26;
_27 = core::ptr::addr_of!(_11.3.0);
place!(Field::<i64>(Variant(_23, 1), 6)) = _16.1;
_1 = [(-12211_i16),12099_i16,(-15528_i16),(-5402_i16),18309_i16,28602_i16,(-5397_i16)];
_12 = Adt21::Variant1 { fld0: _9.0,fld1: Move(_11.3.2),fld2: _11.0,fld3: Field::<i8>(Variant(_23, 1), 3),fld4: 45722_u16,fld5: _17.2,fld6: Field::<i64>(Variant(_23, 1), 6) };
_11.2 = &place!(Field::<i8>(Variant(_23, 1), 3));
place!(Field::<i64>(Variant(_12, 1), 6)) = !Field::<i64>(Variant(_23, 1), 6);
_15 = _16.2;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(6_usize, 4_usize, Move(_4), 25_usize, Move(_25), 26_usize, Move(_26), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(6_usize, 18_usize, Move(_18), 21_usize, Move(_21), 33_usize, _33, 33_usize, _33), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: bool) -> char {
mir! {
type RET = char;
let _2: [i8; 8];
let _3: u8;
let _4: *mut *mut f32;
let _5: isize;
let _6: [i8; 5];
let _7: [bool; 3];
let _8: (f64, Adt22, &'static i8, (i128, u64, *mut f32, f32));
let _9: Adt21;
let _10: bool;
let _11: [i8; 8];
let _12: (*mut Adt21, (*const i16,));
let _13: u8;
let _14: &'static isize;
let _15: i8;
let _16: isize;
let _17: ([usize; 5],);
let _18: f32;
let _19: i32;
let _20: [u16; 3];
let _21: isize;
let _22: *mut &'static i8;
let _23: isize;
let _24: &'static *const i16;
let _25: [i16; 3];
let _26: (*const i16,);
let _27: ();
let _28: ();
{
_1 = false;
RET = '\u{3e3da}';
RET = '\u{d99ae}';
_1 = RET >= RET;
RET = '\u{38ea3}';
_2 = [(-121_i8),120_i8,116_i8,0_i8,(-88_i8),38_i8,(-115_i8),(-46_i8)];
RET = '\u{35f02}';
_2 = [42_i8,38_i8,(-75_i8),82_i8,(-32_i8),(-23_i8),(-77_i8),(-80_i8)];
RET = '\u{c04fc}';
_1 = false;
_1 = !false;
RET = '\u{2557c}';
_1 = !false;
RET = '\u{fd06e}';
_1 = true ^ false;
RET = '\u{cd99d}';
RET = '\u{fe558}';
_1 = !false;
RET = '\u{cb743}';
_3 = 1819110305_i32 as u8;
_1 = true;
RET = '\u{ce43a}';
RET = '\u{4599a}';
_2 = [(-23_i8),26_i8,84_i8,(-7_i8),28_i8,(-111_i8),29_i8,(-118_i8)];
_1 = !true;
_3 = 16316623139808737350_u64 as u8;
Call(_3 = fn8(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = '\u{6cf72}';
RET = '\u{bd56c}';
RET = '\u{89e37}';
RET = '\u{a0eaf}';
RET = '\u{67d45}';
RET = '\u{1ced5}';
_5 = !9223372036854775807_isize;
_6 = [(-52_i8),72_i8,91_i8,13_i8,(-71_i8)];
RET = '\u{111c8}';
_3 = (-1699556381_i32) as u8;
_3 = !15_u8;
RET = '\u{f36a2}';
RET = '\u{390e2}';
RET = '\u{d3c4d}';
RET = '\u{6d589}';
_8.3.3 = 2111256435_i32 as f32;
_5 = !(-71_isize);
_4 = core::ptr::addr_of_mut!(_8.3.2);
_8.3.1 = 18145266204958977768_u64 | 17396042321160139349_u64;
(*_4) = core::ptr::addr_of_mut!(_8.3.3);
_3 = 1449311222_i32 as u8;
_8.0 = (-1916278296_i32) as f64;
_8.3.2 = core::ptr::addr_of_mut!(_8.3.3);
_7 = [_1,_1,_1];
_8.0 = _5 as f64;
Call(_8.3 = fn10(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
(*_4) = core::ptr::addr_of_mut!(_8.3.3);
_9 = Adt21::Variant1 { fld0: _8.3.3,fld1: Move(_8.3.2),fld2: _8.0,fld3: (-121_i8),fld4: 17000_u16,fld5: (-1327184659_i32),fld6: 4027504253253393413_i64 };
(*_4) = Move(Field::<*mut f32>(Variant(_9, 1), 1));
Call(place!(Field::<i64>(Variant(_9, 1), 6)) = core::intrinsics::transmute(_2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = [_1,_1,_1];
place!(Field::<*mut f32>(Variant(_9, 1), 1)) = core::ptr::addr_of_mut!(_8.3.3);
(*_4) = core::ptr::addr_of_mut!(_8.3.3);
_11 = [11_i8,(-125_i8),7_i8,57_i8,30_i8,113_i8,93_i8,101_i8];
_8.2 = &place!(Field::<i8>(Variant(_9, 1), 3));
_9 = Adt21::Variant1 { fld0: _8.3.3,fld1: Move(_8.3.2),fld2: _8.0,fld3: 48_i8,fld4: 9461_u16,fld5: (-1659079655_i32),fld6: 8472919842193282546_i64 };
place!(Field::<i64>(Variant(_9, 1), 6)) = !(-4330313256164198829_i64);
place!(Field::<i32>(Variant(_9, 1), 5)) = -(-1342587285_i32);
_1 = false & true;
_8.2 = &place!(Field::<i8>(Variant(_9, 1), 3));
_6 = [114_i8,37_i8,(-61_i8),7_i8,102_i8];
RET = '\u{469a3}';
Goto(bb4)
}
bb4 = {
_5 = (-48_isize);
_14 = &_5;
(*_4) = core::ptr::addr_of_mut!(_8.3.3);
_8.3.2 = Move(Field::<*mut f32>(Variant(_9, 1), 1));
place!(Field::<i8>(Variant(_9, 1), 3)) = -(-40_i8);
RET = '\u{5f1a6}';
_1 = RET <= RET;
_6 = [Field::<i8>(Variant(_9, 1), 3),Field::<i8>(Variant(_9, 1), 3),Field::<i8>(Variant(_9, 1), 3),Field::<i8>(Variant(_9, 1), 3),Field::<i8>(Variant(_9, 1), 3)];
_8.2 = &place!(Field::<i8>(Variant(_9, 1), 3));
_17.0 = [0_usize,2_usize,12330699129343134714_usize,5_usize,17610557868285430395_usize];
_5 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_14 = &_5;
(*_4) = core::ptr::addr_of_mut!(_18);
_8.1 = Adt22::Variant1 { fld0: Field::<i32>(Variant(_9, 1), 5),fld1: RET };
_8.3.3 = _8.0 as f32;
_13 = !_3;
_19 = Field::<i32>(Variant(_9, 1), 5) | Field::<i32>(Variant(_8.1, 1), 0);
_14 = &(*_14);
place!(Field::<i32>(Variant(_8.1, 1), 0)) = -_19;
_18 = _8.0 as f32;
_9 = Adt21::Variant1 { fld0: _18,fld1: Move((*_4)),fld2: _8.0,fld3: (-70_i8),fld4: 11879_u16,fld5: _19,fld6: (-2445082699439729290_i64) };
_11 = _2;
place!(Field::<u16>(Variant(_9, 1), 4)) = _5 as u16;
_8.3.1 = 10963805208939313017_u64 | 13915886808496009291_u64;
_20 = [Field::<u16>(Variant(_9, 1), 4),Field::<u16>(Variant(_9, 1), 4),Field::<u16>(Variant(_9, 1), 4)];
Goto(bb5)
}
bb5 = {
_5 = !(-9223372036854775808_isize);
_8.3 = ((-41679166059558502674089302775853802423_i128), 14744025009762301527_u64, Move(Field::<*mut f32>(Variant(_9, 1), 1)), Field::<f32>(Variant(_9, 1), 0));
_17.0 = [2338581605902450541_usize,0_usize,0_usize,16043852556419767613_usize,5880882643317505077_usize];
SetDiscriminant(_8.1, 0);
_21 = !_5;
_11 = [(-30_i8),17_i8,86_i8,(-9_i8),(-61_i8),(-62_i8),29_i8,88_i8];
_7 = [_1,_1,_1];
match _8.3.0 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
298603200861379960789285304655914409033 => bb14,
_ => bb13
}
}
bb6 = {
_5 = (-48_isize);
_14 = &_5;
(*_4) = core::ptr::addr_of_mut!(_8.3.3);
_8.3.2 = Move(Field::<*mut f32>(Variant(_9, 1), 1));
place!(Field::<i8>(Variant(_9, 1), 3)) = -(-40_i8);
RET = '\u{5f1a6}';
_1 = RET <= RET;
_6 = [Field::<i8>(Variant(_9, 1), 3),Field::<i8>(Variant(_9, 1), 3),Field::<i8>(Variant(_9, 1), 3),Field::<i8>(Variant(_9, 1), 3),Field::<i8>(Variant(_9, 1), 3)];
_8.2 = &place!(Field::<i8>(Variant(_9, 1), 3));
_17.0 = [0_usize,2_usize,12330699129343134714_usize,5_usize,17610557868285430395_usize];
_5 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_14 = &_5;
(*_4) = core::ptr::addr_of_mut!(_18);
_8.1 = Adt22::Variant1 { fld0: Field::<i32>(Variant(_9, 1), 5),fld1: RET };
_8.3.3 = _8.0 as f32;
_13 = !_3;
_19 = Field::<i32>(Variant(_9, 1), 5) | Field::<i32>(Variant(_8.1, 1), 0);
_14 = &(*_14);
place!(Field::<i32>(Variant(_8.1, 1), 0)) = -_19;
_18 = _8.0 as f32;
_9 = Adt21::Variant1 { fld0: _18,fld1: Move((*_4)),fld2: _8.0,fld3: (-70_i8),fld4: 11879_u16,fld5: _19,fld6: (-2445082699439729290_i64) };
_11 = _2;
place!(Field::<u16>(Variant(_9, 1), 4)) = _5 as u16;
_8.3.1 = 10963805208939313017_u64 | 13915886808496009291_u64;
_20 = [Field::<u16>(Variant(_9, 1), 4),Field::<u16>(Variant(_9, 1), 4),Field::<u16>(Variant(_9, 1), 4)];
Goto(bb5)
}
bb7 = {
_7 = [_1,_1,_1];
place!(Field::<*mut f32>(Variant(_9, 1), 1)) = core::ptr::addr_of_mut!(_8.3.3);
(*_4) = core::ptr::addr_of_mut!(_8.3.3);
_11 = [11_i8,(-125_i8),7_i8,57_i8,30_i8,113_i8,93_i8,101_i8];
_8.2 = &place!(Field::<i8>(Variant(_9, 1), 3));
_9 = Adt21::Variant1 { fld0: _8.3.3,fld1: Move(_8.3.2),fld2: _8.0,fld3: 48_i8,fld4: 9461_u16,fld5: (-1659079655_i32),fld6: 8472919842193282546_i64 };
place!(Field::<i64>(Variant(_9, 1), 6)) = !(-4330313256164198829_i64);
place!(Field::<i32>(Variant(_9, 1), 5)) = -(-1342587285_i32);
_1 = false & true;
_8.2 = &place!(Field::<i8>(Variant(_9, 1), 3));
_6 = [114_i8,37_i8,(-61_i8),7_i8,102_i8];
RET = '\u{469a3}';
Goto(bb4)
}
bb8 = {
(*_4) = core::ptr::addr_of_mut!(_8.3.3);
_9 = Adt21::Variant1 { fld0: _8.3.3,fld1: Move(_8.3.2),fld2: _8.0,fld3: (-121_i8),fld4: 17000_u16,fld5: (-1327184659_i32),fld6: 4027504253253393413_i64 };
(*_4) = Move(Field::<*mut f32>(Variant(_9, 1), 1));
Call(place!(Field::<i64>(Variant(_9, 1), 6)) = core::intrinsics::transmute(_2), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
RET = '\u{6cf72}';
RET = '\u{bd56c}';
RET = '\u{89e37}';
RET = '\u{a0eaf}';
RET = '\u{67d45}';
RET = '\u{1ced5}';
_5 = !9223372036854775807_isize;
_6 = [(-52_i8),72_i8,91_i8,13_i8,(-71_i8)];
RET = '\u{111c8}';
_3 = (-1699556381_i32) as u8;
_3 = !15_u8;
RET = '\u{f36a2}';
RET = '\u{390e2}';
RET = '\u{d3c4d}';
RET = '\u{6d589}';
_8.3.3 = 2111256435_i32 as f32;
_5 = !(-71_isize);
_4 = core::ptr::addr_of_mut!(_8.3.2);
_8.3.1 = 18145266204958977768_u64 | 17396042321160139349_u64;
(*_4) = core::ptr::addr_of_mut!(_8.3.3);
_3 = 1449311222_i32 as u8;
_8.0 = (-1916278296_i32) as f64;
_8.3.2 = core::ptr::addr_of_mut!(_8.3.3);
_7 = [_1,_1,_1];
_8.0 = _5 as f64;
Call(_8.3 = fn10(_2), ReturnTo(bb2), UnwindUnreachable())
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
_18 = _8.3.0 as f32;
_1 = false;
_1 = true;
place!(Field::<char>(Variant(_8.1, 0), 1)) = RET;
_8.2 = &place!(Field::<i8>(Variant(_9, 1), 3));
_9 = Adt21::Variant1 { fld0: _18,fld1: Move((*_4)),fld2: _8.0,fld3: 85_i8,fld4: 29055_u16,fld5: _19,fld6: 4580962112935733853_i64 };
_20 = [63779_u16,26349_u16,32274_u16];
_18 = _8.3.3;
_8.3.3 = Field::<f32>(Variant(_9, 1), 0) * Field::<f32>(Variant(_9, 1), 0);
_25 = [(-21329_i16),(-29641_i16),19153_i16];
_8.2 = &place!(Field::<i8>(Variant(_9, 1), 3));
place!(Field::<i8>(Variant(_9, 1), 3)) = !4_i8;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(7_usize, 13_usize, Move(_13), 7_usize, Move(_7), 11_usize, Move(_11), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(7_usize, 20_usize, Move(_20), 17_usize, Move(_17), 28_usize, _28, 28_usize, _28), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8() -> u8 {
mir! {
type RET = u8;
let _1: bool;
let _2: [u128; 4];
let _3: &'static isize;
let _4: *mut *mut f32;
let _5: u64;
let _6: [i8; 8];
let _7: Adt45;
let _8: u64;
let _9: isize;
let _10: f32;
let _11: (u16,);
let _12: [i64; 7];
let _13: i8;
let _14: isize;
let _15: bool;
let _16: f64;
let _17: &'static i8;
let _18: usize;
let _19: i8;
let _20: u64;
let _21: [i8; 8];
let _22: isize;
let _23: (f64, Adt22, &'static i8, (i128, u64, *mut f32, f32));
let _24: &'static f64;
let _25: *mut *mut u32;
let _26: [i16; 7];
let _27: [bool; 3];
let _28: ();
let _29: ();
{
RET = 255_u8 + 241_u8;
RET = 52_u8 - 191_u8;
RET = 35_u8;
RET = !69_u8;
RET = 1756355079541948260_u64 as u8;
RET = 234_u8 - 115_u8;
RET = 5601117115314840575_u64 as u8;
RET = 37_u8;
RET = !160_u8;
RET = 229_u8 >> 59_u8;
RET = 118_u8 * 245_u8;
RET = 24_u8;
_1 = RET != RET;
RET = !53_u8;
RET = 58_u8 << 815449212_u32;
Goto(bb1)
}
bb1 = {
RET = 155_u8 ^ 13_u8;
RET = 14046011229425591346_u64 as u8;
RET = (-2075253824_i32) as u8;
RET = 38_u8;
RET = 4_usize as u8;
RET = !239_u8;
RET = _1 as u8;
RET = !16_u8;
RET = 155_u8 * 129_u8;
Call(RET = core::intrinsics::bswap(251_u8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = [249674981142933890543659587283970883932_u128,173128760495494364728347981495569519826_u128,10540299942751760147024757766935179325_u128,52467498551092725486852261347897868612_u128];
RET = 170_u8;
_1 = true ^ true;
RET = 149_u8 >> 4_usize;
_2 = [15457180460755995848223325404868502027_u128,146096440274691797295020630944009817133_u128,303073047674187522752819621853616909374_u128,84425374192474762833315172141752235626_u128];
RET = !240_u8;
RET = 60_u8;
_1 = false & true;
_1 = false;
_2 = [20337114285961429476642215215487631849_u128,73417910394978032863628336903026808227_u128,216095176112695615815948964293685729445_u128,235839982058443307151975467652229865716_u128];
_2 = [143380199945899032128131111222218817605_u128,111418628879091088956962053046832020661_u128,197872460680282997202830729550270239532_u128,132192423077572557641997945566202146049_u128];
_6 = [37_i8,(-44_i8),103_i8,(-112_i8),23_i8,(-98_i8),50_i8,(-104_i8)];
_2 = [32622369439352359168361268727696615161_u128,174238384870646250930869698841176283056_u128,142130679463473760027223355297612954955_u128,153328041656271694280266887202042798455_u128];
RET = 106_u8;
RET = 83_u8 ^ 2_u8;
_5 = !16032459184051345731_u64;
_5 = 7891035197506374217_u64;
_1 = RET != RET;
_6 = [(-16_i8),(-125_i8),(-103_i8),24_i8,85_i8,(-68_i8),(-127_i8),(-57_i8)];
_8 = _5 | _5;
_2 = [262242390911119563446113889360229967892_u128,277760719151101096175478464892506663770_u128,77034261048344468384736718433374990604_u128,27346776860660632669722625133033322066_u128];
_3 = &_9;
_1 = true | true;
_5 = _8;
_9 = 26585_u16 as isize;
Goto(bb3)
}
bb3 = {
_6 = [(-24_i8),90_i8,115_i8,0_i8,61_i8,43_i8,(-66_i8),(-112_i8)];
_11 = (55741_u16,);
_11.0 = !45949_u16;
_3 = &_9;
_11.0 = 26391_u16 << _9;
_2 = [272431510590912754617205108736930462885_u128,297952632546884144964299666681214084520_u128,309205862704795557681509441159451365847_u128,146715872621172397991709186404016948360_u128];
Goto(bb4)
}
bb4 = {
_1 = false;
_11 = (54597_u16,);
_8 = 62593408132954416106659431093022614296_i128 as u64;
_12 = [(-3808628193234796307_i64),4029536162007503285_i64,(-8475270520772355492_i64),(-6543375657307003932_i64),(-7557376703891487141_i64),(-7053549292527772819_i64),(-2218535753455027294_i64)];
_8 = !_5;
_8 = !_5;
RET = !79_u8;
_8 = !_5;
_8 = _5;
match _11.0 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
54597 => bb11,
_ => bb10
}
}
bb5 = {
_6 = [(-24_i8),90_i8,115_i8,0_i8,61_i8,43_i8,(-66_i8),(-112_i8)];
_11 = (55741_u16,);
_11.0 = !45949_u16;
_3 = &_9;
_11.0 = 26391_u16 << _9;
_2 = [272431510590912754617205108736930462885_u128,297952632546884144964299666681214084520_u128,309205862704795557681509441159451365847_u128,146715872621172397991709186404016948360_u128];
Goto(bb4)
}
bb6 = {
_2 = [249674981142933890543659587283970883932_u128,173128760495494364728347981495569519826_u128,10540299942751760147024757766935179325_u128,52467498551092725486852261347897868612_u128];
RET = 170_u8;
_1 = true ^ true;
RET = 149_u8 >> 4_usize;
_2 = [15457180460755995848223325404868502027_u128,146096440274691797295020630944009817133_u128,303073047674187522752819621853616909374_u128,84425374192474762833315172141752235626_u128];
RET = !240_u8;
RET = 60_u8;
_1 = false & true;
_1 = false;
_2 = [20337114285961429476642215215487631849_u128,73417910394978032863628336903026808227_u128,216095176112695615815948964293685729445_u128,235839982058443307151975467652229865716_u128];
_2 = [143380199945899032128131111222218817605_u128,111418628879091088956962053046832020661_u128,197872460680282997202830729550270239532_u128,132192423077572557641997945566202146049_u128];
_6 = [37_i8,(-44_i8),103_i8,(-112_i8),23_i8,(-98_i8),50_i8,(-104_i8)];
_2 = [32622369439352359168361268727696615161_u128,174238384870646250930869698841176283056_u128,142130679463473760027223355297612954955_u128,153328041656271694280266887202042798455_u128];
RET = 106_u8;
RET = 83_u8 ^ 2_u8;
_5 = !16032459184051345731_u64;
_5 = 7891035197506374217_u64;
_1 = RET != RET;
_6 = [(-16_i8),(-125_i8),(-103_i8),24_i8,85_i8,(-68_i8),(-127_i8),(-57_i8)];
_8 = _5 | _5;
_2 = [262242390911119563446113889360229967892_u128,277760719151101096175478464892506663770_u128,77034261048344468384736718433374990604_u128,27346776860660632669722625133033322066_u128];
_3 = &_9;
_1 = true | true;
_5 = _8;
_9 = 26585_u16 as isize;
Goto(bb3)
}
bb7 = {
RET = 155_u8 ^ 13_u8;
RET = 14046011229425591346_u64 as u8;
RET = (-2075253824_i32) as u8;
RET = 38_u8;
RET = 4_usize as u8;
RET = !239_u8;
RET = _1 as u8;
RET = !16_u8;
RET = 155_u8 * 129_u8;
Call(RET = core::intrinsics::bswap(251_u8), ReturnTo(bb2), UnwindUnreachable())
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
RET = 159_u8 & 163_u8;
_11.0 = 533643725_u32 as u16;
_13 = (-125_i8) >> RET;
_6 = [_13,_13,_13,_13,_13,_13,_13,_13];
_8 = !_5;
_5 = _8;
_10 = _13 as f32;
RET = (-121703107740557107884410374872435318472_i128) as u8;
_10 = 17097727856461547804027835922934167413_u128 as f32;
RET = 90_u8 << _5;
_13 = (-1256009555_i32) as i8;
_9 = RET as isize;
_11 = (11123_u16,);
_14 = !_9;
_11.0 = _1 as u16;
_11 = (51129_u16,);
_10 = _5 as f32;
_2 = [212611012877766663809214856934841885942_u128,90399721608069810490178449872050075162_u128,332827427700732887175465937484620842507_u128,37819986207872486975325284949969298468_u128];
_13 = (-43_i8) - (-97_i8);
_9 = 74410671356544967808358754365168501994_i128 as isize;
_10 = RET as f32;
_14 = -_9;
_16 = 1292581563822804101_usize as f64;
_14 = _9 - _9;
_2 = [107440418651754105407666941715939179148_u128,109184315396029278876171409980149262390_u128,236647512062445001198844368511265463032_u128,196015076961192578250342478022713847097_u128];
_11 = (26044_u16,);
_5 = _8 >> _8;
match _11.0 {
26044 => bb12,
_ => bb7
}
}
bb12 = {
_16 = 2_usize as f64;
_8 = _5;
_16 = 124291123074464231067452289912788444379_u128 as f64;
_18 = 2_usize;
_12 = [(-4594091146274451930_i64),8414931361606358109_i64,(-9028185829277381450_i64),3129620438210522136_i64,(-7077820401198620289_i64),(-837156637107521394_i64),(-5748688551701532090_i64)];
_10 = _6[_18] as f32;
RET = _11.0 as u8;
_12[_18] = 40918443698269270426006539315062221296_i128 as i64;
_8 = !_5;
_17 = &_19;
_3 = &_14;
_11.0 = _6[_18] as u16;
_9 = !_14;
_21[_18] = _18 as i8;
Call(_23.3 = fn9(Move(_3), (*_3), _14, _2[_18]), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_23.0 = -_16;
_24 = &_23.0;
_12 = [(-8559362971025768150_i64),8944198300556333708_i64,(-7346336275335214598_i64),(-2742630912758593335_i64),2547317314520316373_i64,6758865485016756160_i64,(-5791830267779556472_i64)];
_24 = &_16;
match _18 {
0 => bb9,
1 => bb14,
2 => bb16,
_ => bb15
}
}
bb14 = {
_6 = [(-24_i8),90_i8,115_i8,0_i8,61_i8,43_i8,(-66_i8),(-112_i8)];
_11 = (55741_u16,);
_11.0 = !45949_u16;
_3 = &_9;
_11.0 = 26391_u16 << _9;
_2 = [272431510590912754617205108736930462885_u128,297952632546884144964299666681214084520_u128,309205862704795557681509441159451365847_u128,146715872621172397991709186404016948360_u128];
Goto(bb4)
}
bb15 = {
Return()
}
bb16 = {
_19 = -_13;
_12 = [(-2580717416894577190_i64),(-5101066424337224895_i64),(-2665668142708045502_i64),3057426100452124685_i64,(-4075054954873202252_i64),7162995512710379879_i64,335324954624819256_i64];
_3 = &_14;
_23.3.3 = -_10;
_3 = &(*_3);
_23.3.2 = core::ptr::addr_of_mut!(_10);
_4 = core::ptr::addr_of_mut!(_23.3.2);
_23.3.1 = _5 << _8;
_23.3.3 = _10 * _10;
_23.3.2 = core::ptr::addr_of_mut!(_10);
_23.3.2 = core::ptr::addr_of_mut!(_23.3.3);
_3 = &_14;
_23.0 = _16;
_3 = &(*_3);
_20 = _23.3.1;
_4 = core::ptr::addr_of_mut!(_23.3.2);
_16 = _13 as f64;
_15 = _1;
Goto(bb17)
}
bb17 = {
Call(_28 = dump_var(8_usize, 13_usize, Move(_13), 6_usize, Move(_6), 18_usize, Move(_18), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_28 = dump_var(8_usize, 9_usize, Move(_9), 19_usize, Move(_19), 8_usize, Move(_8), 29_usize, _29), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: &'static isize,mut _2: isize,mut _3: isize,mut _4: u128) -> (i128, u64, *mut f32, f32) {
mir! {
type RET = (i128, u64, *mut f32, f32);
let _5: *const (&'static i8, *const *mut f32, usize, Adt45);
let _6: u128;
let _7: &'static i64;
let _8: [i16; 7];
let _9: char;
let _10: f64;
let _11: u16;
let _12: isize;
let _13: [usize; 5];
let _14: i128;
let _15: i8;
let _16: char;
let _17: (u8, usize);
let _18: f32;
let _19: f32;
let _20: &'static isize;
let _21: bool;
let _22: &'static f64;
let _23: [i8; 5];
let _24: f32;
let _25: (f32,);
let _26: i32;
let _27: *const *mut f32;
let _28: Adt50;
let _29: f64;
let _30: *mut u32;
let _31: Adt45;
let _32: ([usize; 5], *const i16, (char, *const i128, i32), [u16; 3]);
let _33: [usize; 5];
let _34: u16;
let _35: [i16; 6];
let _36: isize;
let _37: i64;
let _38: (*const i16,);
let _39: &'static (Adt22, i64, isize);
let _40: isize;
let _41: *const *mut f32;
let _42: &'static i64;
let _43: f64;
let _44: &'static ([usize; 5], *const i16, (char, *const i128, i32), [u16; 3]);
let _45: u16;
let _46: ();
let _47: ();
{
RET.0 = !(-98040950956370137774846790760099506809_i128);
Goto(bb1)
}
bb1 = {
_4 = 144031622536274048268258047538519128609_u128 >> RET.0;
RET.0 = !(-11716154713164942572836107993646848994_i128);
RET.0 = 87567857656426638360003725401474277122_i128 | (-151387081807207934369731871651900470344_i128);
RET.1 = 11585744973797437344_u64 - 6372011891383051502_u64;
RET.0 = _2 as i128;
_1 = &_3;
_3 = _2 & _2;
RET.1 = 2251949322636815093_u64 << RET.0;
_1 = &_2;
_4 = 241106846177576466116225889779854437424_u128 >> RET.0;
RET.1 = (-1771663591797826494_i64) as u64;
RET.2 = core::ptr::addr_of_mut!(RET.3);
_6 = 237_u8 as u128;
RET.2 = core::ptr::addr_of_mut!(RET.3);
Call(_2 = core::intrinsics::bswap(_3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = &_2;
RET.0 = !1192232060501653385438950630501373799_i128;
_1 = &(*_1);
_4 = !_6;
RET.0 = !(-98326328292909604335555579093767825715_i128);
RET.0 = -(-118659012611677961644351337687426756851_i128);
_1 = &_2;
_6 = 182_u8 as u128;
_4 = !_6;
_3 = (*_1);
RET.3 = (*_1) as f32;
RET.2 = core::ptr::addr_of_mut!(RET.3);
_1 = &(*_1);
RET.3 = 2043555145_i32 as f32;
_4 = 5_usize as u128;
RET.3 = RET.0 as f32;
RET.2 = core::ptr::addr_of_mut!(RET.3);
RET.0 = 16281098004411140559086745076205258961_i128 * (-76155309069656955870809268663884023834_i128);
RET.2 = core::ptr::addr_of_mut!(RET.3);
_8 = [17860_i16,(-25825_i16),(-26522_i16),(-16828_i16),9956_i16,24606_i16,(-5521_i16)];
Goto(bb3)
}
bb3 = {
RET.1 = 386652549937861214_u64 * 9186294102060868376_u64;
_1 = &(*_1);
_2 = _3 >> _4;
_8 = [16179_i16,(-16383_i16),32616_i16,2525_i16,29963_i16,10982_i16,4669_i16];
_10 = 2124740956_i32 as f64;
RET.3 = RET.0 as f32;
_1 = &_3;
_4 = _6 << _6;
_9 = '\u{10392f}';
_9 = '\u{4b77}';
RET.2 = core::ptr::addr_of_mut!(RET.3);
_1 = &(*_1);
_2 = -_3;
RET.2 = core::ptr::addr_of_mut!(RET.3);
RET.3 = (-6468734212880186545_i64) as f32;
_2 = _3 << (*_1);
_8 = [330_i16,1212_i16,(-21424_i16),21802_i16,3617_i16,6067_i16,17718_i16];
RET.3 = 8097002047133596674_i64 as f32;
RET.1 = RET.0 as u64;
RET.2 = core::ptr::addr_of_mut!(RET.3);
RET.3 = _6 as f32;
_11 = 3351718506740150434_i64 as u16;
RET.1 = 8281650604241216975_u64 + 3540421751123081352_u64;
_6 = 1165685010_i32 as u128;
Goto(bb4)
}
bb4 = {
RET.3 = RET.0 as f32;
_1 = &_2;
_6 = !_4;
RET.2 = core::ptr::addr_of_mut!(RET.3);
Goto(bb5)
}
bb5 = {
_1 = &_2;
RET.2 = core::ptr::addr_of_mut!(RET.3);
RET.1 = false as u64;
RET.0 = RET.3 as i128;
_4 = !_6;
_4 = 2099258297_u32 as u128;
RET.3 = (-9144285699787017477_i64) as f32;
RET.0 = (-36009664574616993937263939745268784860_i128) << _2;
Goto(bb6)
}
bb6 = {
RET.2 = core::ptr::addr_of_mut!(RET.3);
_11 = 29524_u16 | 10291_u16;
_12 = _2;
RET.2 = core::ptr::addr_of_mut!(RET.3);
_10 = (*_1) as f64;
RET.2 = core::ptr::addr_of_mut!(RET.3);
RET.2 = core::ptr::addr_of_mut!(RET.3);
_11 = 62234_u16 << (*_1);
_10 = RET.1 as f64;
RET.3 = (-7630840843074790791_i64) as f32;
_9 = '\u{6625f}';
RET.3 = 255_u8 as f32;
_11 = 59208_u16;
_1 = &_12;
Goto(bb7)
}
bb7 = {
RET.2 = core::ptr::addr_of_mut!(RET.3);
_15 = (-58_i8) * 24_i8;
_9 = '\u{fb0c8}';
_6 = _4 << _15;
_10 = (-8689230242128129463_i64) as f64;
RET.1 = !15386601982357031430_u64;
RET.1 = 13693642107476300314_u64 * 4779394959696244009_u64;
_13 = [6_usize,11995299021592479026_usize,18113384803960322871_usize,0_usize,4_usize];
RET.1 = !5067972067910665973_u64;
_17.0 = !131_u8;
_3 = _12 & _2;
_17 = (5_u8, 10368589525673384760_usize);
_12 = -_3;
RET.0 = (-109145478684758999253139460031211982398_i128) | 101348890941796195828159743051476453861_i128;
_17.0 = 99_u8 >> _2;
Goto(bb8)
}
bb8 = {
_12 = !_2;
_17.1 = 6687630206064200499_usize;
RET.3 = _11 as f32;
RET.2 = core::ptr::addr_of_mut!(_18);
RET.3 = 6679447305376132010_i64 as f32;
_13 = [_17.1,_17.1,_17.1,_17.1,_17.1];
_20 = &_3;
RET.2 = core::ptr::addr_of_mut!(_18);
RET.1 = !8629155010722025943_u64;
_1 = &_12;
_20 = &(*_20);
_23 = [_15,_15,_15,_15,_15];
_16 = _9;
RET.1 = _15 as u64;
_14 = _4 as i128;
_18 = -RET.3;
_19 = RET.3;
_4 = _6 + _6;
_22 = &_10;
_21 = _6 >= _4;
RET.0 = _14;
_20 = &_3;
_21 = !true;
_26 = (-1402551941_i32);
Call(_4 = core::intrinsics::transmute(_14), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_23 = [_15,_15,_15,_15,_15];
_19 = (*_20) as f32;
_21 = _6 != _6;
RET.3 = _19 + _18;
_25.0 = -_19;
_6 = _4 - _4;
_4 = !_6;
_10 = _26 as f64;
_15 = -(-72_i8);
_21 = !true;
_20 = Move(_1);
_6 = _21 as u128;
_12 = !_3;
_1 = &_12;
_17 = (72_u8, 2_usize);
_2 = RET.3 as isize;
_17 = (15_u8, 819856652020173599_usize);
_19 = -RET.3;
RET.1 = 14141147371858636887_u64;
RET.0 = _14;
RET.3 = _10 as f32;
_29 = _10 + _10;
_9 = _16;
_18 = _11 as f32;
_11 = _25.0 as u16;
Goto(bb10)
}
bb10 = {
_27 = core::ptr::addr_of!(RET.2);
_16 = _9;
_17.0 = 2511975288_u32 as u8;
_27 = core::ptr::addr_of!((*_27));
RET.1 = _26 as u64;
RET.1 = 2589408984356877177_u64;
_16 = _9;
_24 = _19;
_25 = (_19,);
_21 = !false;
_24 = _25.0 + _19;
_17.1 = 3121223353230262536_usize;
_14 = RET.0;
_24 = _19;
_11 = 53776_u16 >> _12;
_34 = _11 ^ _11;
_4 = _25.0 as u128;
_17.0 = 248_u8 - 216_u8;
Goto(bb11)
}
bb11 = {
_19 = _24 - _24;
RET.0 = _14;
_7 = &_37;
_34 = _11;
_34 = _9 as u16;
_36 = _9 as isize;
_32.2.2 = _9 as i32;
_20 = &_12;
_32.2.2 = -_26;
_15 = 29_i8 & 26_i8;
_22 = &_29;
_37 = _16 as i64;
_3 = (*_20) - (*_20);
_25.0 = -_24;
_32.2.2 = -_26;
_4 = _17.0 as u128;
_20 = Move(_1);
RET.0 = _14 << _15;
match _26 {
0 => bb6,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463463374607430365659515 => bb13,
_ => bb12
}
}
bb12 = {
_1 = &_2;
RET.2 = core::ptr::addr_of_mut!(RET.3);
RET.1 = false as u64;
RET.0 = RET.3 as i128;
_4 = !_6;
_4 = 2099258297_u32 as u128;
RET.3 = (-9144285699787017477_i64) as f32;
RET.0 = (-36009664574616993937263939745268784860_i128) << _2;
Goto(bb6)
}
bb13 = {
_24 = _15 as f32;
_26 = !_32.2.2;
_16 = _9;
_21 = !true;
_8 = [(-12594_i16),(-11534_i16),12707_i16,(-26997_i16),(-21226_i16),(-17508_i16),10469_i16];
_15 = !45_i8;
_9 = _16;
_27 = core::ptr::addr_of!((*_27));
_33 = [_17.1,_17.1,_17.1,_17.1,_17.1];
_22 = &(*_22);
_20 = &_36;
RET.1 = !16356171368318033471_u64;
_10 = 1725107020_u32 as f64;
_35 = [5398_i16,(-800_i16),(-4127_i16),32003_i16,11932_i16,(-29948_i16)];
_16 = _9;
_41 = Move(_27);
_41 = core::ptr::addr_of!(RET.2);
_40 = _3 + _3;
RET.3 = _19;
_32.3 = [_11,_11,_11];
_15 = (-35_i8);
RET.2 = core::ptr::addr_of_mut!(RET.3);
RET.0 = _14 + _14;
(*_41) = core::ptr::addr_of_mut!(_19);
_23 = [_15,_15,_15,_15,_15];
Goto(bb14)
}
bb14 = {
Call(_46 = dump_var(9_usize, 11_usize, Move(_11), 6_usize, Move(_6), 23_usize, Move(_23), 26_usize, Move(_26)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_46 = dump_var(9_usize, 4_usize, Move(_4), 8_usize, Move(_8), 3_usize, Move(_3), 40_usize, Move(_40)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_46 = dump_var(9_usize, 9_usize, Move(_9), 16_usize, Move(_16), 13_usize, Move(_13), 47_usize, _47), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: [i8; 8]) -> (i128, u64, *mut f32, f32) {
mir! {
type RET = (i128, u64, *mut f32, f32);
let _2: (f32,);
let _3: bool;
let _4: u16;
let _5: Adt22;
let _6: isize;
let _7: (&'static i8, *const *mut f32, usize, Adt45);
let _8: isize;
let _9: Adt77;
let _10: (f32,);
let _11: char;
let _12: char;
let _13: u32;
let _14: [i32; 8];
let _15: i128;
let _16: i128;
let _17: isize;
let _18: f32;
let _19: [u64; 7];
let _20: usize;
let _21: *mut u32;
let _22: (char, *const i128, i32);
let _23: isize;
let _24: isize;
let _25: [u128; 4];
let _26: [bool; 8];
let _27: [bool; 8];
let _28: f64;
let _29: f64;
let _30: ();
let _31: ();
{
RET.1 = !11732819163068678022_u64;
_2.0 = 18899_i16 as f32;
_1 = [88_i8,108_i8,(-16_i8),(-125_i8),(-112_i8),(-65_i8),(-38_i8),1_i8];
RET.2 = core::ptr::addr_of_mut!(RET.3);
_2.0 = (-9223372036854775808_isize) as f32;
RET.3 = -_2.0;
RET.2 = core::ptr::addr_of_mut!(_2.0);
RET.0 = -(-84205368602832724203463925413411212677_i128);
_1 = [(-31_i8),24_i8,58_i8,78_i8,2_i8,(-119_i8),(-79_i8),39_i8];
_4 = !44886_u16;
RET.1 = 16590794154016446486_u64;
_1 = [112_i8,(-115_i8),(-27_i8),80_i8,25_i8,(-51_i8),(-93_i8),(-11_i8)];
RET.2 = core::ptr::addr_of_mut!(_2.0);
RET.3 = -_2.0;
RET.0 = 42528464867767381020856666088774027949_i128;
_5 = Adt22::Variant1 { fld0: (-65211040_i32),fld1: '\u{a083d}' };
_2 = (RET.3,);
place!(Field::<char>(Variant(_5, 1), 1)) = '\u{26c41}';
RET.0 = -127416673760715933639569148370856961410_i128;
place!(Field::<char>(Variant(_5, 1), 1)) = '\u{d1ad5}';
place!(Field::<i32>(Variant(_5, 1), 0)) = 1065558958_i32 >> _4;
match RET.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
16590794154016446486 => bb9,
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
_11 = Field::<char>(Variant(_5, 1), 1);
_8 = (-9223372036854775808_isize);
SetDiscriminant(_5, 0);
_7.2 = 511716710_u32 as usize;
_8 = 9223372036854775807_isize - 9223372036854775807_isize;
RET.1 = !15488266575922306765_u64;
_10.0 = _2.0 * RET.3;
RET.3 = -_10.0;
place!(Field::<i8>(Variant(_5, 0), 3)) = (-5_i8);
_12 = _11;
_3 = !true;
RET.1 = 13495326612932962828_u64 & 14893318654243889162_u64;
Call(_7.1 = fn11(), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
RET.2 = core::ptr::addr_of_mut!(_10.0);
RET.2 = core::ptr::addr_of_mut!(RET.3);
_5 = Adt22::Variant1 { fld0: (-345035797_i32),fld1: _11 };
_3 = _2.0 != RET.3;
RET.1 = 11694164354472962332_u64 ^ 15223224565713696791_u64;
RET.3 = -_10.0;
_10 = _2;
place!(Field::<i32>(Variant(_5, 1), 0)) = (-182646512_i32) >> _7.2;
_7.1 = core::ptr::addr_of!(RET.2);
_2 = (RET.3,);
_13 = !1613974345_u32;
RET.3 = _7.2 as f32;
_14 = [Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0)];
RET.1 = !92132885082941434_u64;
_15 = !RET.0;
_3 = !true;
_16 = _15;
_14 = [Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0)];
RET.1 = 18002303336740074466_u64;
RET.2 = core::ptr::addr_of_mut!(_10.0);
_14 = [Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0)];
_2.0 = _10.0;
_2.0 = _10.0 * _10.0;
RET.3 = 35_i8 as f32;
Call(_17 = core::intrinsics::transmute(_1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
SetDiscriminant(_5, 1);
place!(Field::<i32>(Variant(_5, 1), 0)) = -1702831507_i32;
_3 = !true;
_10.0 = -RET.3;
_4 = _16 as u16;
place!(Field::<char>(Variant(_5, 1), 1)) = _11;
RET.0 = !_15;
RET.3 = -_2.0;
_7.2 = !1_usize;
place!(Field::<char>(Variant(_5, 1), 1)) = _12;
_3 = _17 != _17;
RET.0 = _13 as i128;
place!(Field::<char>(Variant(_5, 1), 1)) = _11;
_7.2 = 5510685457838935802_i64 as usize;
_8 = (-6143430541795461145_i64) as isize;
_15 = _16;
RET.3 = 22352077523690739935084783675286287318_u128 as f32;
_8 = _17;
_6 = _17 | _17;
_13 = 269441473_u32;
_18 = (-20987_i16) as f32;
Goto(bb12)
}
bb12 = {
_16 = _15;
_13 = _4 as u32;
RET.1 = !6041053803336770376_u64;
_22.0 = _12;
_11 = _12;
_16 = RET.0 ^ _15;
_23 = _13 as isize;
_7.2 = 39772210019000111561137675999883118182_u128 as usize;
_23 = _17;
_14 = [Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0),Field::<i32>(Variant(_5, 1), 0)];
_22.1 = core::ptr::addr_of!(RET.0);
_16 = _15;
RET.3 = _2.0 - _10.0;
_23 = !_6;
_6 = _8 * _23;
SetDiscriminant(_5, 1);
_3 = true;
_16 = RET.0 - _15;
_23 = -_8;
Goto(bb13)
}
bb13 = {
_23 = _8;
_21 = core::ptr::addr_of_mut!(_13);
_16 = !RET.0;
(*_21) = 3905000375_u32 + 3202805430_u32;
_22.2 = !1964492997_i32;
RET.1 = (*_21) as u64;
RET.2 = core::ptr::addr_of_mut!(_10.0);
_14 = [_22.2,_22.2,_22.2,_22.2,_22.2,_22.2,_22.2,_22.2];
Goto(bb14)
}
bb14 = {
_22.1 = core::ptr::addr_of!(_16);
_18 = -RET.3;
_10.0 = -_2.0;
_25 = [276731509048088801481025755201277588588_u128,266712127860026930694470409044453428509_u128,320634896577023444984065636774485665751_u128,195322835152844303701547481944596237923_u128];
RET.3 = -_2.0;
_22.0 = _12;
_13 = 1274667814_u32;
_27 = [_3,_3,_3,_3,_3,_3,_3,_3];
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(10_usize, 14_usize, Move(_14), 13_usize, Move(_13), 12_usize, Move(_12), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(10_usize, 8_usize, Move(_8), 6_usize, Move(_6), 27_usize, Move(_27), 31_usize, _31), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11() -> *const *mut f32 {
mir! {
type RET = *const *mut f32;
let _1: i64;
let _2: [i64; 5];
let _3: f32;
let _4: [u64; 7];
let _5: *const i16;
let _6: &'static Adt20;
let _7: Adt22;
let _8: [u64; 1];
let _9: ((char, *const i128, i32),);
let _10: f64;
let _11: i8;
let _12: [i16; 3];
let _13: &'static Adt20;
let _14: u32;
let _15: &'static i64;
let _16: i8;
let _17: char;
let _18: (*mut Adt21,);
let _19: bool;
let _20: *mut u32;
let _21: bool;
let _22: *const (&'static i8, *const *mut f32, usize, Adt45);
let _23: [u64; 7];
let _24: [bool; 8];
let _25: char;
let _26: (*mut Adt21,);
let _27: [i8; 8];
let _28: [u128; 4];
let _29: i128;
let _30: u128;
let _31: &'static i64;
let _32: i64;
let _33: &'static *const i16;
let _34: [u64; 1];
let _35: i64;
let _36: isize;
let _37: [usize; 5];
let _38: bool;
let _39: u16;
let _40: isize;
let _41: [u64; 7];
let _42: isize;
let _43: [u128; 4];
let _44: f32;
let _45: i8;
let _46: isize;
let _47: u64;
let _48: (Adt53, Adt50, *const *mut f32, &'static i16);
let _49: [u64; 1];
let _50: isize;
let _51: f32;
let _52: [i64; 5];
let _53: [bool; 8];
let _54: (*mut Adt21,);
let _55: isize;
let _56: [i32; 8];
let _57: (f32,);
let _58: [u64; 7];
let _59: isize;
let _60: Adt50;
let _61: u128;
let _62: bool;
let _63: (*mut Adt21, (*const i16,));
let _64: isize;
let _65: bool;
let _66: [bool; 3];
let _67: isize;
let _68: (*mut f32, u16, Adt22, i64);
let _69: i32;
let _70: f32;
let _71: (*mut Adt21, (*const i16,));
let _72: f32;
let _73: f64;
let _74: i32;
let _75: u128;
let _76: u32;
let _77: &'static f64;
let _78: Adt77;
let _79: u32;
let _80: u64;
let _81: bool;
let _82: (u16,);
let _83: i32;
let _84: isize;
let _85: isize;
let _86: [i16; 3];
let _87: ([usize; 5],);
let _88: [i16; 6];
let _89: (u16,);
let _90: f32;
let _91: ([i64; 7],);
let _92: ();
let _93: ();
{
_2 = [(-5677702866859558527_i64),(-3587319865694561265_i64),(-171864773501872482_i64),1040446658524821990_i64,(-7623937583387008677_i64)];
_2 = [(-7958249709343012743_i64),(-5074035790843871578_i64),7219946196804215972_i64,(-2440479416726115414_i64),3051223209146176059_i64];
_1 = -115493823770474925_i64;
_1 = false as i64;
_1 = (-2558845786889413964_i64);
_2 = [_1,_1,_1,_1,_1];
_1 = (-2120546020891408450_i64) | 7852620717902570841_i64;
_2 = [_1,_1,_1,_1,_1];
_1 = 7627217642103289086_i64 >> 68352353626050438856554074794300318300_i128;
Goto(bb1)
}
bb1 = {
_2 = [_1,_1,_1,_1,_1];
_1 = !(-1377575984686168353_i64);
Call(_2 = fn12(_1, _1, _1, _1, _1, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = 47_u8 as f32;
_1 = 162279088_i32 as i64;
_4 = [1046886675212182930_u64,17571611895223874772_u64,4096121503509233612_u64,18075908592731011503_u64,18076466320861983347_u64,17130481466828636269_u64,1972275613881251490_u64];
_3 = 204_u8 as f32;
_2 = [_1,_1,_1,_1,_1];
_4 = [10802314066537503577_u64,14353474621291127913_u64,16602717459139422187_u64,17942225522512613737_u64,5743042654362310228_u64,1001762269723518895_u64,17968829214676821951_u64];
_3 = 67_u8 as f32;
_1 = -2083357958078370226_i64;
_7 = Adt22::Variant1 { fld0: 1701418166_i32,fld1: '\u{4c5d8}' };
_7 = Adt22::Variant1 { fld0: 1062827858_i32,fld1: '\u{e762e}' };
place!(Field::<i32>(Variant(_7, 1), 0)) = 13797965_i32;
_4 = [607733686660127018_u64,2866220622002208129_u64,12182880725260128365_u64,13865647573698009461_u64,5484046704396413980_u64,11840605903548075304_u64,6111438655864716463_u64];
_8 = [17920843352852427544_u64];
match Field::<i32>(Variant(_7, 1), 0) {
13797965 => bb3,
_ => bb1
}
}
bb3 = {
_9.0.0 = '\u{fb235}';
_7 = Adt22::Variant1 { fld0: (-28178283_i32),fld1: _9.0.0 };
_3 = 14529109723605631031_usize as f32;
_4 = [18357626269231079701_u64,4029409565639790162_u64,9283899867889203682_u64,11638123026918386389_u64,1277758026772304788_u64,4708068196766083606_u64,3250846841828668115_u64];
_9.0.2 = -19809598_i32;
_2 = [_1,_1,_1,_1,_1];
_8 = [9698322442275310894_u64];
_9.0.2 = !1792086519_i32;
place!(Field::<i32>(Variant(_7, 1), 0)) = !_9.0.2;
_7 = Adt22::Variant1 { fld0: _9.0.2,fld1: _9.0.0 };
_9.0.2 = Field::<i32>(Variant(_7, 1), 0) * Field::<i32>(Variant(_7, 1), 0);
_15 = &_1;
place!(Field::<i32>(Variant(_7, 1), 0)) = _9.0.2 - _9.0.2;
place!(Field::<char>(Variant(_7, 1), 1)) = _9.0.0;
Goto(bb4)
}
bb4 = {
_9.0.0 = Field::<char>(Variant(_7, 1), 1);
_9.0.2 = !Field::<i32>(Variant(_7, 1), 0);
_2 = [(*_15),_1,(*_15),(*_15),_1];
_11 = !113_i8;
_8 = [16137907660765812755_u64];
_11 = 17221692854013527092_u64 as i8;
_1 = 13_u8 as i64;
Goto(bb5)
}
bb5 = {
_7 = Adt22::Variant1 { fld0: _9.0.2,fld1: _9.0.0 };
SetDiscriminant(_7, 1);
_9.0.2 = 206_u8 as i32;
place!(Field::<char>(Variant(_7, 1), 1)) = _9.0.0;
_16 = _11 - _11;
_4 = [16575673332475370959_u64,3072408664073107911_u64,7194310491577651986_u64,1587533407528004654_u64,10689505576847486761_u64,18248805328804701644_u64,15669235593774036782_u64];
_3 = 4_usize as f32;
_10 = _3 as f64;
Goto(bb6)
}
bb6 = {
_15 = &_1;
_17 = _9.0.0;
_20 = core::ptr::addr_of_mut!(_14);
_10 = _1 as f64;
_9.0.0 = Field::<char>(Variant(_7, 1), 1);
place!(Field::<i32>(Variant(_7, 1), 0)) = -_9.0.2;
_19 = Field::<i32>(Variant(_7, 1), 0) >= Field::<i32>(Variant(_7, 1), 0);
(*_20) = 3792846840_u32;
_9.0.0 = _17;
SetDiscriminant(_7, 1);
place!(Field::<char>(Variant(_7, 1), 1)) = _9.0.0;
_14 = _19 as u32;
_9.0.0 = _17;
place!(Field::<char>(Variant(_7, 1), 1)) = _17;
_16 = -_11;
_15 = &(*_15);
_8 = [9729266368459037715_u64];
_14 = !2318580455_u32;
(*_20) = !3035239993_u32;
place!(Field::<char>(Variant(_7, 1), 1)) = _9.0.0;
_15 = &_1;
_16 = _11 << _14;
_15 = &_1;
(*_20) = 573450031_u32 ^ 1417373783_u32;
_21 = _19;
_10 = (*_20) as f64;
_14 = _17 as u32;
_3 = 10021978011182633350_usize as f32;
_2 = [_1,(*_15),(*_15),(*_15),(*_15)];
Goto(bb7)
}
bb7 = {
(*_20) = !135717391_u32;
_19 = !_21;
_17 = _9.0.0;
Goto(bb8)
}
bb8 = {
_3 = 15605173048975670420_usize as f32;
_2 = [_1,(*_15),_1,(*_15),(*_15)];
_15 = &_1;
_23 = [17075783123485706088_u64,15537782468944278625_u64,9092992591489217241_u64,17914028220373202109_u64,5310439300309348019_u64,4591011610477400269_u64,15857977340495669592_u64];
_25 = _17;
_10 = 247_u8 as f64;
_20 = core::ptr::addr_of_mut!((*_20));
_17 = Field::<char>(Variant(_7, 1), 1);
Goto(bb9)
}
bb9 = {
_17 = _9.0.0;
_15 = &(*_15);
_11 = 3_usize as i8;
_2 = [_1,_1,(*_15),(*_15),(*_15)];
_21 = _19 ^ _19;
Goto(bb10)
}
bb10 = {
_28 = [302412746655081769065891996652917856999_u128,145651042250900826198400012984621465166_u128,337024388534694803223777231974720086734_u128,47504895300625458928879395353042076093_u128];
_24 = [_21,_21,_19,_21,_19,_19,_19,_21];
(*_20) = _16 as u32;
_27 = [_16,_16,_16,_16,_16,_16,_16,_16];
_11 = _16 << (*_20);
_8 = [8239735274156224921_u64];
_15 = &_1;
Goto(bb11)
}
bb11 = {
_9.0.1 = core::ptr::addr_of!(_29);
_15 = &(*_15);
_14 = (*_15) as u32;
_19 = _21;
_8 = [14915572440518729148_u64];
_17 = _9.0.0;
_1 = (-8406343486792391880_i64) * (-3449801622098478271_i64);
_21 = !_19;
_29 = 129857937061576574423104733775897526530_i128;
place!(Field::<i32>(Variant(_7, 1), 0)) = _9.0.2 ^ _9.0.2;
_20 = core::ptr::addr_of_mut!((*_20));
_21 = _11 >= _11;
place!(Field::<i32>(Variant(_7, 1), 0)) = _9.0.2 & _9.0.2;
_10 = _3 as f64;
_27 = [_11,_11,_11,_11,_11,_11,_11,_11];
_9.0.2 = -Field::<i32>(Variant(_7, 1), 0);
_15 = &_32;
_16 = _11;
_21 = _25 <= Field::<char>(Variant(_7, 1), 1);
_1 = -(-7244935981165889563_i64);
place!(Field::<i32>(Variant(_7, 1), 0)) = _9.0.2;
_31 = &(*_15);
_15 = &_1;
_12 = [15003_i16,(-28368_i16),(-2319_i16)];
Call((*_20) = core::intrinsics::bswap(1104423167_u32), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_27 = [_11,_11,_16,_16,_16,_11,_16,_11];
(*_20) = !71476846_u32;
_7 = Adt22::Variant1 { fld0: _9.0.2,fld1: _17 };
_24 = [_19,_21,_19,_19,_19,_19,_19,_19];
(*_20) = 24699_i16 as u32;
(*_20) = 1854075789_u32;
_33 = &_5;
_24 = [_19,_19,_21,_21,_21,_21,_19,_19];
_20 = core::ptr::addr_of_mut!((*_20));
_19 = !_21;
SetDiscriminant(_7, 1);
_21 = _19;
(*_20) = !210075297_u32;
_9.0.1 = core::ptr::addr_of!(_29);
_8 = [17741457645555836445_u64];
place!(Field::<char>(Variant(_7, 1), 1)) = _9.0.0;
_24 = [_19,_21,_19,_21,_19,_21,_21,_21];
_30 = _16 as u128;
_28 = [_30,_30,_30,_30];
_20 = core::ptr::addr_of_mut!((*_20));
_19 = !_21;
_2 = [(*_15),(*_15),_1,_1,(*_15)];
_35 = -(*_15);
(*_20) = _17 as u32;
_4 = [6180970336926537298_u64,5625152618294197738_u64,8346631595751222292_u64,13234832670848727791_u64,5042629601483220719_u64,12720374782942145074_u64,841575189337127084_u64];
_36 = 9223372036854775807_isize;
_24 = [_21,_19,_21,_19,_19,_21,_21,_21];
Goto(bb13)
}
bb13 = {
_15 = &_1;
_10 = _1 as f64;
_20 = core::ptr::addr_of_mut!(_14);
_33 = &(*_33);
_32 = (*_15);
_3 = _10 as f32;
_15 = &(*_15);
(*_20) = 811491998_u32 * 575505146_u32;
_32 = 5724919122656904021_u64 as i64;
_16 = _11 - _11;
_9.0.2 = (-856565083_i32);
_35 = -_1;
_34 = _8;
_7 = Adt22::Variant1 { fld0: _9.0.2,fld1: _17 };
(*_20) = 1801529352_u32 ^ 2917389434_u32;
_29 = 1257115730441917149338325154290704783_i128;
_16 = _11;
_38 = _21;
_10 = _36 as f64;
_38 = _19;
_17 = _9.0.0;
match _36 {
9223372036854775807 => bb14,
_ => bb3
}
}
bb14 = {
_9.0.2 = Field::<i32>(Variant(_7, 1), 0) << _14;
Goto(bb15)
}
bb15 = {
_31 = &_32;
_3 = 51016_u16 as f32;
_11 = -_16;
_14 = 2972720030_u32;
_4 = [14937290249983079579_u64,14057705053966818891_u64,6718040297204274944_u64,8377719360955588234_u64,17085435075440674156_u64,12260555547394442975_u64,10544950036041515956_u64];
_44 = _3 + _3;
_15 = &_32;
_28 = [_30,_30,_30,_30];
_44 = _3;
Goto(bb16)
}
bb16 = {
_9.0.1 = core::ptr::addr_of!(_29);
_20 = core::ptr::addr_of_mut!((*_20));
_42 = _36;
_1 = _32;
_47 = !8798989876529529241_u64;
_37 = [3_usize,3_usize,8669921738241325126_usize,4659694882864793892_usize,6831470397103977901_usize];
_10 = (*_15) as f64;
_32 = _1 << _14;
_15 = &_35;
Goto(bb17)
}
bb17 = {
_19 = _21 == _21;
_9.0.2 = _30 as i32;
_41 = [_47,_47,_47,_47,_47,_47,_47];
_1 = -(*_15);
_43 = _28;
_33 = &_5;
_21 = _38;
(*_20) = 3066526877_u32;
_48.1 = Adt50::Variant2 { fld0: _12,fld1: _42 };
place!(Field::<i32>(Variant(_7, 1), 0)) = _9.0.2;
_15 = &_32;
_38 = !_19;
_21 = _11 >= _11;
_49 = [_47];
_32 = _1;
_21 = !_19;
_53 = [_21,_19,_21,_19,_38,_21,_19,_19];
_37 = [12662251583885123942_usize,1_usize,4_usize,7_usize,4_usize];
SetDiscriminant(_7, 1);
_25 = _9.0.0;
_15 = &_1;
_33 = &_5;
match (*_20) {
0 => bb9,
1 => bb14,
2 => bb3,
3 => bb8,
4 => bb5,
5 => bb15,
6 => bb18,
3066526877 => bb20,
_ => bb19
}
}
bb18 = {
_9.0.1 = core::ptr::addr_of!(_29);
_20 = core::ptr::addr_of_mut!((*_20));
_42 = _36;
_1 = _32;
_47 = !8798989876529529241_u64;
_37 = [3_usize,3_usize,8669921738241325126_usize,4659694882864793892_usize,6831470397103977901_usize];
_10 = (*_15) as f64;
_32 = _1 << _14;
_15 = &_35;
Goto(bb17)
}
bb19 = {
_15 = &_1;
_17 = _9.0.0;
_20 = core::ptr::addr_of_mut!(_14);
_10 = _1 as f64;
_9.0.0 = Field::<char>(Variant(_7, 1), 1);
place!(Field::<i32>(Variant(_7, 1), 0)) = -_9.0.2;
_19 = Field::<i32>(Variant(_7, 1), 0) >= Field::<i32>(Variant(_7, 1), 0);
(*_20) = 3792846840_u32;
_9.0.0 = _17;
SetDiscriminant(_7, 1);
place!(Field::<char>(Variant(_7, 1), 1)) = _9.0.0;
_14 = _19 as u32;
_9.0.0 = _17;
place!(Field::<char>(Variant(_7, 1), 1)) = _17;
_16 = -_11;
_15 = &(*_15);
_8 = [9729266368459037715_u64];
_14 = !2318580455_u32;
(*_20) = !3035239993_u32;
place!(Field::<char>(Variant(_7, 1), 1)) = _9.0.0;
_15 = &_1;
_16 = _11 << _14;
_15 = &_1;
(*_20) = 573450031_u32 ^ 1417373783_u32;
_21 = _19;
_10 = (*_20) as f64;
_14 = _17 as u32;
_3 = 10021978011182633350_usize as f32;
_2 = [_1,(*_15),(*_15),(*_15),(*_15)];
Goto(bb7)
}
bb20 = {
place!(Field::<char>(Variant(_7, 1), 1)) = _9.0.0;
_33 = &_5;
_29 = _47 as i128;
_33 = &(*_33);
_30 = 117751490014279304247389173922281939275_u128 + 271299721508347933836069396290705306070_u128;
_21 = _14 < (*_20);
_52 = [(*_15),_32,_35,_32,(*_15)];
_57 = (_3,);
_46 = _10 as isize;
(*_20) = !2863673564_u32;
_33 = &_5;
match _36 {
0 => bb18,
1 => bb2,
2 => bb13,
3 => bb9,
4 => bb5,
5 => bb12,
6 => bb15,
9223372036854775807 => bb22,
_ => bb21
}
}
bb21 = {
_9.0.1 = core::ptr::addr_of!(_29);
_20 = core::ptr::addr_of_mut!((*_20));
_42 = _36;
_1 = _32;
_47 = !8798989876529529241_u64;
_37 = [3_usize,3_usize,8669921738241325126_usize,4659694882864793892_usize,6831470397103977901_usize];
_10 = (*_15) as f64;
_32 = _1 << _14;
_15 = &_35;
Goto(bb17)
}
bb22 = {
SetDiscriminant(_48.1, 1);
_10 = (-22484_i16) as f64;
_61 = _30 - _30;
_60 = Adt50::Variant2 { fld0: _12,fld1: _36 };
SetDiscriminant(_60, 0);
_4 = _23;
place!(Field::<i128>(Variant(_60, 0), 3)) = _1 as i128;
_41 = [_47,_47,_47,_47,_47,_47,_47];
_62 = !_21;
place!(Field::<u16>(Variant(_60, 0), 0)) = !61658_u16;
_10 = _46 as f64;
_20 = core::ptr::addr_of_mut!((*_20));
_38 = _61 < _61;
_50 = _42 + _42;
match _36 {
0 => bb9,
1 => bb23,
2 => bb24,
3 => bb25,
4 => bb26,
5 => bb27,
9223372036854775807 => bb29,
_ => bb28
}
}
bb23 = {
_17 = _9.0.0;
_15 = &(*_15);
_11 = 3_usize as i8;
_2 = [_1,_1,(*_15),(*_15),(*_15)];
_21 = _19 ^ _19;
Goto(bb10)
}
bb24 = {
_15 = &_1;
_10 = _1 as f64;
_20 = core::ptr::addr_of_mut!(_14);
_33 = &(*_33);
_32 = (*_15);
_3 = _10 as f32;
_15 = &(*_15);
(*_20) = 811491998_u32 * 575505146_u32;
_32 = 5724919122656904021_u64 as i64;
_16 = _11 - _11;
_9.0.2 = (-856565083_i32);
_35 = -_1;
_34 = _8;
_7 = Adt22::Variant1 { fld0: _9.0.2,fld1: _17 };
(*_20) = 1801529352_u32 ^ 2917389434_u32;
_29 = 1257115730441917149338325154290704783_i128;
_16 = _11;
_38 = _21;
_10 = _36 as f64;
_38 = _19;
_17 = _9.0.0;
match _36 {
9223372036854775807 => bb14,
_ => bb3
}
}
bb25 = {
_15 = &_1;
_17 = _9.0.0;
_20 = core::ptr::addr_of_mut!(_14);
_10 = _1 as f64;
_9.0.0 = Field::<char>(Variant(_7, 1), 1);
place!(Field::<i32>(Variant(_7, 1), 0)) = -_9.0.2;
_19 = Field::<i32>(Variant(_7, 1), 0) >= Field::<i32>(Variant(_7, 1), 0);
(*_20) = 3792846840_u32;
_9.0.0 = _17;
SetDiscriminant(_7, 1);
place!(Field::<char>(Variant(_7, 1), 1)) = _9.0.0;
_14 = _19 as u32;
_9.0.0 = _17;
place!(Field::<char>(Variant(_7, 1), 1)) = _17;
_16 = -_11;
_15 = &(*_15);
_8 = [9729266368459037715_u64];
_14 = !2318580455_u32;
(*_20) = !3035239993_u32;
place!(Field::<char>(Variant(_7, 1), 1)) = _9.0.0;
_15 = &_1;
_16 = _11 << _14;
_15 = &_1;
(*_20) = 573450031_u32 ^ 1417373783_u32;
_21 = _19;
_10 = (*_20) as f64;
_14 = _17 as u32;
_3 = 10021978011182633350_usize as f32;
_2 = [_1,(*_15),(*_15),(*_15),(*_15)];
Goto(bb7)
}
bb26 = {
_9.0.1 = core::ptr::addr_of!(_29);
_20 = core::ptr::addr_of_mut!((*_20));
_42 = _36;
_1 = _32;
_47 = !8798989876529529241_u64;
_37 = [3_usize,3_usize,8669921738241325126_usize,4659694882864793892_usize,6831470397103977901_usize];
_10 = (*_15) as f64;
_32 = _1 << _14;
_15 = &_35;
Goto(bb17)
}
bb27 = {
_19 = _21 == _21;
_9.0.2 = _30 as i32;
_41 = [_47,_47,_47,_47,_47,_47,_47];
_1 = -(*_15);
_43 = _28;
_33 = &_5;
_21 = _38;
(*_20) = 3066526877_u32;
_48.1 = Adt50::Variant2 { fld0: _12,fld1: _42 };
place!(Field::<i32>(Variant(_7, 1), 0)) = _9.0.2;
_15 = &_32;
_38 = !_19;
_21 = _11 >= _11;
_49 = [_47];
_32 = _1;
_21 = !_19;
_53 = [_21,_19,_21,_19,_38,_21,_19,_19];
_37 = [12662251583885123942_usize,1_usize,4_usize,7_usize,4_usize];
SetDiscriminant(_7, 1);
_25 = _9.0.0;
_15 = &_1;
_33 = &_5;
match (*_20) {
0 => bb9,
1 => bb14,
2 => bb3,
3 => bb8,
4 => bb5,
5 => bb15,
6 => bb18,
3066526877 => bb20,
_ => bb19
}
}
bb28 = {
_31 = &_32;
_3 = 51016_u16 as f32;
_11 = -_16;
_14 = 2972720030_u32;
_4 = [14937290249983079579_u64,14057705053966818891_u64,6718040297204274944_u64,8377719360955588234_u64,17085435075440674156_u64,12260555547394442975_u64,10544950036041515956_u64];
_44 = _3 + _3;
_15 = &_32;
_28 = [_30,_30,_30,_30];
_44 = _3;
Goto(bb16)
}
bb29 = {
_39 = !Field::<u16>(Variant(_60, 0), 0);
_24 = _53;
_43 = [_30,_61,_61,_61];
_58 = _4;
_48.3 = &place!(Field::<i16>(Variant(_48.1, 1), 4));
_55 = _36;
_15 = &(*_15);
_1 = _47 as i64;
_15 = &_32;
_55 = _25 as isize;
place!(Field::<[u64; 7]>(Variant(_48.1, 1), 1)) = [_47,_47,_47,_47,_47,_47,_47];
_24 = [_38,_62,_38,_19,_62,_38,_38,_38];
_42 = _36;
_56 = [_9.0.2,_9.0.2,_9.0.2,_9.0.2,_9.0.2,_9.0.2,_9.0.2,_9.0.2];
_33 = &(*_33);
_5 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_48.1, 1), 4)));
_40 = -_46;
_24 = _53;
_49 = _8;
_28 = _43;
_34 = _8;
place!(Field::<*const i16>(Variant(_60, 0), 6)) = Move(_5);
_3 = _44 * _44;
place!(Field::<u128>(Variant(_60, 0), 4)) = _30;
Goto(bb30)
}
bb30 = {
_45 = _16 ^ _11;
_20 = core::ptr::addr_of_mut!((*_20));
_63.1 = (Move(Field::<*const i16>(Variant(_60, 0), 6)),);
(*_20) = 920076577_u32;
_8 = [_47];
Goto(bb31)
}
bb31 = {
place!(Field::<isize>(Variant(_48.1, 1), 2)) = 86_u8 as isize;
_9.0.2 = (-612241830_i32);
_64 = _40 * _55;
_30 = !_61;
_45 = _16 | _11;
_29 = !Field::<i128>(Variant(_60, 0), 3);
_59 = _17 as isize;
_8 = [_47];
_58 = [_47,_47,_47,_47,_47,_47,_47];
_48.1 = Adt50::Variant2 { fld0: _12,fld1: _64 };
_50 = _59;
_55 = _36 ^ Field::<isize>(Variant(_48.1, 2), 1);
Goto(bb32)
}
bb32 = {
_9.0.1 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_60, 0), 3)));
_21 = _64 > _55;
place!(Field::<*const i16>(Variant(_60, 0), 6)) = Move(_63.1.0);
match _36 {
0 => bb33,
9223372036854775807 => bb35,
_ => bb34
}
}
bb33 = {
_15 = &_1;
_17 = _9.0.0;
_20 = core::ptr::addr_of_mut!(_14);
_10 = _1 as f64;
_9.0.0 = Field::<char>(Variant(_7, 1), 1);
place!(Field::<i32>(Variant(_7, 1), 0)) = -_9.0.2;
_19 = Field::<i32>(Variant(_7, 1), 0) >= Field::<i32>(Variant(_7, 1), 0);
(*_20) = 3792846840_u32;
_9.0.0 = _17;
SetDiscriminant(_7, 1);
place!(Field::<char>(Variant(_7, 1), 1)) = _9.0.0;
_14 = _19 as u32;
_9.0.0 = _17;
place!(Field::<char>(Variant(_7, 1), 1)) = _17;
_16 = -_11;
_15 = &(*_15);
_8 = [9729266368459037715_u64];
_14 = !2318580455_u32;
(*_20) = !3035239993_u32;
place!(Field::<char>(Variant(_7, 1), 1)) = _9.0.0;
_15 = &_1;
_16 = _11 << _14;
_15 = &_1;
(*_20) = 573450031_u32 ^ 1417373783_u32;
_21 = _19;
_10 = (*_20) as f64;
_14 = _17 as u32;
_3 = 10021978011182633350_usize as f32;
_2 = [_1,(*_15),(*_15),(*_15),(*_15)];
Goto(bb7)
}
bb34 = {
_7 = Adt22::Variant1 { fld0: _9.0.2,fld1: _9.0.0 };
SetDiscriminant(_7, 1);
_9.0.2 = 206_u8 as i32;
place!(Field::<char>(Variant(_7, 1), 1)) = _9.0.0;
_16 = _11 - _11;
_4 = [16575673332475370959_u64,3072408664073107911_u64,7194310491577651986_u64,1587533407528004654_u64,10689505576847486761_u64,18248805328804701644_u64,15669235593774036782_u64];
_3 = 4_usize as f32;
_10 = _3 as f64;
Goto(bb6)
}
bb35 = {
place!(Field::<[i64; 7]>(Variant(_60, 0), 5)) = [_1,_35,_1,(*_15),_1,_1,_32];
_11 = _45 ^ _45;
_63.1 = (Move(Field::<*const i16>(Variant(_60, 0), 6)),);
_30 = _61 - _61;
match (*_20) {
0 => bb36,
920076577 => bb38,
_ => bb37
}
}
bb36 = {
_9.0.1 = core::ptr::addr_of!(_29);
_20 = core::ptr::addr_of_mut!((*_20));
_42 = _36;
_1 = _32;
_47 = !8798989876529529241_u64;
_37 = [3_usize,3_usize,8669921738241325126_usize,4659694882864793892_usize,6831470397103977901_usize];
_10 = (*_15) as f64;
_32 = _1 << _14;
_15 = &_35;
Goto(bb17)
}
bb37 = {
_2 = [_1,_1,_1,_1,_1];
_1 = !(-1377575984686168353_i64);
Call(_2 = fn12(_1, _1, _1, _1, _1, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb38 = {
SetDiscriminant(_48.1, 0);
_52 = _2;
_5 = Move(_63.1.0);
_30 = !_61;
_28 = [_30,_30,Field::<u128>(Variant(_60, 0), 4),_30];
_57 = (_3,);
_24 = [_38,_19,_21,_62,_38,_19,_19,_38];
_53 = [_38,_62,_38,_38,_19,_38,_21,_38];
_10 = _3 as f64;
place!(Field::<u128>(Variant(_48.1, 0), 4)) = Field::<char>(Variant(_7, 1), 1) as u128;
_60 = Adt50::Variant2 { fld0: _12,fld1: _64 };
place!(Field::<u16>(Variant(_48.1, 0), 0)) = !_39;
_68.2 = Adt22::Variant1 { fld0: _9.0.2,fld1: Field::<char>(Variant(_7, 1), 1) };
Goto(bb39)
}
bb39 = {
place!(Field::<i32>(Variant(_7, 1), 0)) = Field::<i32>(Variant(_68.2, 1), 0);
_68.1 = !Field::<u16>(Variant(_48.1, 0), 0);
_38 = _19;
_14 = _57.0 as u32;
_68.0 = core::ptr::addr_of_mut!(_3);
_27 = [_11,_11,_45,_11,_11,_11,_11,_11];
_42 = (-1161_i16) as isize;
_69 = _29 as i32;
_64 = _3 as isize;
place!(Field::<[i16; 3]>(Variant(_60, 2), 0)) = _12;
_1 = (*_15);
_23 = [_47,_47,_47,_47,_47,_47,_47];
place!(Field::<char>(Variant(_68.2, 1), 1)) = _9.0.0;
_7 = Move(_68.2);
_71.1 = (Move(_5),);
_67 = _64 - _50;
_37 = [1306501532516256599_usize,4_usize,0_usize,6_usize,7_usize];
_56 = [_9.0.2,_9.0.2,_9.0.2,_69,_9.0.2,_69,Field::<i32>(Variant(_7, 1), 0),Field::<i32>(Variant(_7, 1), 0)];
_68.2 = Move(_7);
_28 = [_61,_61,_30,_61];
place!(Field::<[i64; 7]>(Variant(_48.1, 0), 5)) = [(*_15),_35,(*_15),_35,_1,_32,_32];
_45 = _11;
_14 = 361340414_u32;
_61 = !_30;
_25 = _17;
place!(Field::<[i16; 3]>(Variant(_60, 2), 0)) = _12;
_46 = _67;
_36 = _67;
_29 = 3_usize as i128;
match (*_20) {
0 => bb10,
1 => bb36,
2 => bb40,
361340414 => bb42,
_ => bb41
}
}
bb40 = {
_17 = _9.0.0;
_15 = &(*_15);
_11 = 3_usize as i8;
_2 = [_1,_1,(*_15),(*_15),(*_15)];
_21 = _19 ^ _19;
Goto(bb10)
}
bb41 = {
_7 = Adt22::Variant1 { fld0: _9.0.2,fld1: _9.0.0 };
SetDiscriminant(_7, 1);
_9.0.2 = 206_u8 as i32;
place!(Field::<char>(Variant(_7, 1), 1)) = _9.0.0;
_16 = _11 - _11;
_4 = [16575673332475370959_u64,3072408664073107911_u64,7194310491577651986_u64,1587533407528004654_u64,10689505576847486761_u64,18248805328804701644_u64,15669235593774036782_u64];
_3 = 4_usize as f32;
_10 = _3 as f64;
Goto(bb6)
}
bb42 = {
_11 = -_45;
_72 = -_3;
_48.2 = core::ptr::addr_of!(_68.0);
_33 = &place!(Field::<*const i16>(Variant(_48.1, 0), 6));
SetDiscriminant(_68.2, 0);
_19 = !_38;
_68.0 = core::ptr::addr_of_mut!(_44);
place!(Field::<[i64; 7]>(Variant(_48.1, 0), 5)) = [(*_15),(*_15),_1,_32,(*_15),_32,_35];
SetDiscriminant(_60, 1);
_74 = _69;
_63.0 = core::ptr::addr_of_mut!(place!(Field::<Adt21>(Variant(_68.2, 0), 2)));
_14 = !2828293428_u32;
_54.0 = Move(_63.0);
_57 = (_3,);
place!(Field::<i128>(Variant(_48.1, 0), 3)) = _29;
_18.0 = core::ptr::addr_of_mut!(place!(Field::<Adt21>(Variant(_68.2, 0), 2)));
_71.0 = Move(_18.0);
_26.0 = Move(_71.0);
_59 = _44 as isize;
match _9.0.2 {
340282366920938463463374607431155969626 => bb43,
_ => bb17
}
}
bb43 = {
_65 = _62;
_48.2 = core::ptr::addr_of!(_68.0);
_18 = (Move(_54.0),);
_47 = 17237553751799474704_u64 ^ 8579829839998418035_u64;
place!(Field::<*const i16>(Variant(_48.1, 0), 6)) = core::ptr::addr_of!(place!(Field::<i16>(Variant(_60, 1), 4)));
_43 = [_30,_61,_61,_30];
_67 = _55 + _46;
_53 = [_19,_19,_65,_38,_62,_62,_21,_65];
_16 = _45;
_11 = !_16;
_5 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_60, 1), 4)));
place!(Field::<f32>(Variant(_60, 1), 0)) = _61 as f32;
_21 = _38 & _38;
_51 = -Field::<f32>(Variant(_60, 1), 0);
_28 = [_61,_61,_61,_61];
_76 = !(*_20);
_5 = Move(_71.1.0);
_49 = [_47];
_38 = _62;
_4 = [_47,_47,_47,_47,_47,_47,_47];
place!(Field::<(*mut Adt21,)>(Variant(_48.1, 0), 1)) = (Move(_26.0),);
place!(Field::<*const *mut f32>(Variant(_48.1, 0), 2)) = core::ptr::addr_of!(_68.0);
_8 = [_47];
_32 = _35 - _35;
Goto(bb44)
}
bb44 = {
_68.3 = -_1;
_52 = [_32,_32,_32,_1,_35];
_42 = _9.0.0 as isize;
_28 = [_61,_30,_30,_61];
_32 = _68.3;
_30 = _69 as u128;
_30 = _61 + Field::<u128>(Variant(_48.1, 0), 4);
_18 = (Move(Field::<(*mut Adt21,)>(Variant(_48.1, 0), 1).0),);
_79 = !(*_20);
_55 = _51 as isize;
_34 = [_47];
_65 = _38;
Goto(bb45)
}
bb45 = {
_63.0 = Move(_18.0);
_81 = _65;
_63.1 = (Move(Field::<*const i16>(Variant(_48.1, 0), 6)),);
_10 = 7913270974526909047_usize as f64;
place!(Field::<u16>(Variant(_48.1, 0), 0)) = _39 & _39;
_60 = Adt50::Variant2 { fld0: _12,fld1: _67 };
_70 = _51;
_42 = _57.0 as isize;
_8 = [_47];
_71.1.0 = Move(_5);
place!(Field::<i128>(Variant(_48.1, 0), 3)) = _29 ^ _29;
_76 = _25 as u32;
SetDiscriminant(_60, 2);
_55 = _69 as isize;
match _9.0.2 {
0 => bb33,
340282366920938463463374607431155969626 => bb46,
_ => bb36
}
}
bb46 = {
_1 = _42 as i64;
_83 = _69;
place!(Field::<isize>(Variant(_60, 2), 1)) = _68.1 as isize;
_58 = [_47,_47,_47,_47,_47,_47,_47];
_61 = _47 as u128;
_54 = (Move(_63.0),);
place!(Field::<[i16; 3]>(Variant(_60, 2), 0)) = [3921_i16,9437_i16,28226_i16];
RET = core::ptr::addr_of!(_68.0);
place!(Field::<i8>(Variant(_68.2, 0), 3)) = _10 as i8;
_9.0.1 = core::ptr::addr_of!(_29);
_57 = (_70,);
_63.1.0 = Move(_71.1.0);
_65 = _19;
_71.0 = core::ptr::addr_of_mut!(place!(Field::<Adt21>(Variant(_68.2, 0), 2)));
_64 = _30 as isize;
_23 = [_47,_47,_47,_47,_47,_47,_47];
_51 = -_72;
_26.0 = core::ptr::addr_of_mut!(place!(Field::<Adt21>(Variant(_68.2, 0), 2)));
_82 = (Field::<u16>(Variant(_48.1, 0), 0),);
_25 = _17;
_88 = [(-30157_i16),(-8928_i16),(-22549_i16),(-24492_i16),(-1680_i16),(-7942_i16)];
Goto(bb47)
}
bb47 = {
Call(_92 = dump_var(11_usize, 27_usize, Move(_27), 11_usize, Move(_11), 17_usize, Move(_17), 21_usize, Move(_21)), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
Call(_92 = dump_var(11_usize, 53_usize, Move(_53), 28_usize, Move(_28), 46_usize, Move(_46), 42_usize, Move(_42)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Call(_92 = dump_var(11_usize, 14_usize, Move(_14), 50_usize, Move(_50), 2_usize, Move(_2), 1_usize, Move(_1)), ReturnTo(bb50), UnwindUnreachable())
}
bb50 = {
Call(_92 = dump_var(11_usize, 56_usize, Move(_56), 69_usize, Move(_69), 52_usize, Move(_52), 88_usize, Move(_88)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_92 = dump_var(11_usize, 62_usize, Move(_62), 34_usize, Move(_34), 25_usize, Move(_25), 35_usize, Move(_35)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_92 = dump_var(11_usize, 67_usize, Move(_67), 36_usize, Move(_36), 59_usize, Move(_59), 74_usize, Move(_74)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_92 = dump_var(11_usize, 19_usize, Move(_19), 4_usize, Move(_4), 93_usize, _93, 93_usize, _93), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: i64,mut _2: i64,mut _3: i64,mut _4: i64,mut _5: i64,mut _6: i64,mut _7: i64) -> [i64; 5] {
mir! {
type RET = [i64; 5];
let _8: isize;
let _9: [i16; 2];
let _10: [u128; 4];
let _11: isize;
let _12: ((char, *const i128, i32),);
let _13: [u32; 1];
let _14: char;
let _15: i32;
let _16: (&'static i8, *const *mut f32, usize, Adt45);
let _17: i32;
let _18: bool;
let _19: (i128, u64, *mut f32, f32);
let _20: (*mut Adt21, (*const i16,));
let _21: isize;
let _22: f64;
let _23: *mut f32;
let _24: ([i64; 7],);
let _25: bool;
let _26: Adt21;
let _27: (*mut Adt21, (*const i16,));
let _28: f32;
let _29: f64;
let _30: bool;
let _31: f32;
let _32: &'static (char, *const i128, i32);
let _33: bool;
let _34: u32;
let _35: bool;
let _36: ((char, *const i128, i32),);
let _37: [i16; 2];
let _38: (f32,);
let _39: (u16,);
let _40: [i64; 5];
let _41: &'static isize;
let _42: usize;
let _43: [bool; 3];
let _44: isize;
let _45: *const (&'static i8, *const *mut f32, usize, Adt45);
let _46: isize;
let _47: char;
let _48: u32;
let _49: f32;
let _50: i32;
let _51: (f32,);
let _52: *mut *mut f32;
let _53: f64;
let _54: Adt50;
let _55: ();
let _56: ();
{
_3 = _5;
RET = [_4,_1,_1,_4,_4];
RET = [_5,_7,_4,_5,_4];
RET = [_5,_3,_5,_5,_3];
RET = [_4,_2,_2,_4,_4];
_10 = [204071193230333161630094778206344386088_u128,32466008784839970497213892824185645093_u128,271963638881220272426579420271888820707_u128,45406441583183357869862195337990351622_u128];
_5 = _3;
_3 = 91178642898541359723233203838260668529_u128 as i64;
_5 = _1;
_5 = _1;
_8 = (-9223372036854775808_isize) + (-9223372036854775808_isize);
_10 = [264444046952334467173291274872240308252_u128,75769065861131686079022253103729758428_u128,52806362530878310233329802245991371556_u128,290354045146360044889558939065387700034_u128];
_10 = [330555743079712015347063484995368820228_u128,162940175950949384500574572107084555866_u128,212001425405501261721425823232399068047_u128,100627796393971313623408989731175632273_u128];
_9 = [2042_i16,6925_i16];
_9 = [15870_i16,(-25763_i16)];
_6 = _1;
_1 = _8 as i64;
_5 = _1 * _7;
RET = [_1,_6,_3,_1,_2];
_9 = [(-16359_i16),24615_i16];
_5 = 3_usize as i64;
_1 = 41937_u16 as i64;
_6 = -_3;
_4 = _7 - _1;
_10 = [332211550980535218569838777348486913788_u128,143817658350587491955960001846418205148_u128,76443651647239341840458446695888679574_u128,80956786032104849327840417335332966169_u128];
Goto(bb1)
}
bb1 = {
_14 = '\u{d7ba9}';
_1 = !_4;
_1 = 21855624_i32 as i64;
_1 = -_4;
_5 = 3723792207_u32 as i64;
_3 = (-19515000473048510346177798294329148614_i128) as i64;
_2 = _6;
_11 = _8 | _8;
_6 = _2;
_12.0.0 = _14;
_11 = _8;
_13 = [1491021251_u32];
_17 = -(-583919032_i32);
_6 = _1;
Goto(bb2)
}
bb2 = {
_8 = _11 | _11;
_7 = _4;
RET = [_1,_7,_1,_4,_5];
_19.3 = 30_i8 as f32;
_6 = _19.3 as i64;
_15 = (-126_i8) as i32;
_18 = true;
_16.2 = 7_usize >> _2;
_12.0.1 = core::ptr::addr_of!(_19.0);
_9 = [(-25926_i16),25991_i16];
_6 = _4;
_7 = _4;
RET = [_6,_1,_6,_3,_6];
Call(_11 = core::intrinsics::bswap(_8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_17 = _15;
_16.1 = core::ptr::addr_of!(_19.2);
_12.0.2 = _17;
_19.1 = !14350852317426539582_u64;
_8 = !_11;
Goto(bb4)
}
bb4 = {
_17 = !_12.0.2;
_4 = _6;
_19.1 = 2359071155306296140_u64;
_19.2 = core::ptr::addr_of_mut!(_19.3);
_13 = [4193977985_u32];
_12.0.2 = -_17;
_18 = !true;
_14 = _12.0.0;
_4 = _17 as i64;
RET = [_7,_1,_7,_7,_6];
_12.0.0 = _14;
_16.1 = core::ptr::addr_of!(_19.2);
_17 = 183082154132178054379843658877375718504_u128 as i32;
_16.1 = core::ptr::addr_of!(_19.2);
_18 = !false;
_19.3 = 3290150336_u32 as f32;
_12.0.0 = _14;
_8 = _11 - _11;
_10 = [8089903054991605891694805216507670814_u128,112922464997434960599479053448391923238_u128,331616013703349585733294327890387047114_u128,16636470869096598237597341569406083828_u128];
RET = [_5,_1,_1,_5,_7];
_7 = _4;
_2 = _5;
RET = [_2,_6,_1,_6,_1];
_3 = 164521664475142175845382733836485571279_u128 as i64;
Goto(bb5)
}
bb5 = {
_12.0.2 = -_15;
_2 = _4;
_16.2 = (-95_i8) as usize;
_13 = [594558652_u32];
_12.0.0 = _14;
_22 = _6 as f64;
_26 = Adt21::Variant1 { fld0: _19.3,fld1: Move(_19.2),fld2: _22,fld3: (-106_i8),fld4: 27220_u16,fld5: _17,fld6: _5 };
_16.1 = core::ptr::addr_of!(_19.2);
_19.2 = core::ptr::addr_of_mut!(_19.3);
_5 = _4;
_20.0 = core::ptr::addr_of_mut!(_26);
_19 = ((-106825957616846226205825610207373425850_i128), 13271061733684882622_u64, Move(Field::<*mut f32>(Variant(_26, 1), 1)), Field::<f32>(Variant(_26, 1), 0));
_20.0 = core::ptr::addr_of_mut!(_26);
_27.0 = Move(_20.0);
place!(Field::<*mut f32>(Variant(_26, 1), 1)) = core::ptr::addr_of_mut!(_19.3);
_12.0.2 = -_17;
RET = [_3,_1,_4,_3,_6];
Call(_16.2 = core::intrinsics::transmute(_11), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_8 = 146_u8 as isize;
_9 = [(-3702_i16),(-16907_i16)];
_5 = _12.0.0 as i64;
_3 = !_1;
_9 = [1067_i16,30097_i16];
_16.2 = 14294149473974744283_usize;
_16.0 = &place!(Field::<i8>(Variant(_26, 1), 3));
_9 = [(-14095_i16),6613_i16];
_25 = !_18;
_2 = _6 & _3;
_17 = 98_i8 as i32;
_25 = !_18;
_30 = _25;
_12.0.1 = core::ptr::addr_of!(_19.0);
_18 = !_25;
_29 = Field::<f64>(Variant(_26, 1), 2) + _22;
place!(Field::<i8>(Variant(_26, 1), 3)) = 38_i8 ^ 10_i8;
_19.3 = 80_u8 as f32;
_17 = -_12.0.2;
place!(Field::<i8>(Variant(_26, 1), 3)) = (-87_i8);
_3 = _19.0 as i64;
_15 = -Field::<i32>(Variant(_26, 1), 5);
_19.0 = 138932202417697258934683846195677307184_i128 | 88021603068443174569030196191814825898_i128;
_20.0 = Move(_27.0);
Call(_16.1 = fn13(Move(_20.0), _19.0, _6, _10, _29, _11, Field::<f64>(Variant(_26, 1), 2), Move(_12), _2, _19.1, _19.1, _15), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_11 = _8 << _19.1;
place!(Field::<f64>(Variant(_26, 1), 2)) = _19.1 as f64;
place!(Field::<*mut f32>(Variant(_26, 1), 1)) = core::ptr::addr_of_mut!(place!(Field::<f32>(Variant(_26, 1), 0)));
Goto(bb8)
}
bb8 = {
_24.0 = [_2,_3,_6,_1,_3,_2,_2];
_5 = _11 as i64;
_19 = (38768460170530506226061351912317985184_i128, 14875591661293492683_u64, Move(Field::<*mut f32>(Variant(_26, 1), 1)), Field::<f32>(Variant(_26, 1), 0));
_32 = &_12.0;
_31 = -Field::<f32>(Variant(_26, 1), 0);
_34 = 3664777120_u32 | 4233285917_u32;
place!(Field::<u16>(Variant(_26, 1), 4)) = 453_u16;
_36.0.0 = _14;
_31 = 8879_i16 as f32;
_24.0 = [_5,_3,_3,_4,_5,Field::<i64>(Variant(_26, 1), 6),_5];
_21 = _14 as isize;
_36.0.2 = _36.0.0 as i32;
_14 = _36.0.0;
_36.0.2 = -_17;
_23 = Move(_19.2);
_33 = !_25;
_35 = !_25;
_2 = _3;
_27.0 = core::ptr::addr_of_mut!(_26);
_19 = (100019082014862154454859111322763820790_i128, 12427448415250919594_u64, Move(_23), Field::<f32>(Variant(_26, 1), 0));
_19.3 = _19.0 as f32;
_5 = _6 + _1;
_20.0 = core::ptr::addr_of_mut!(_26);
_23 = core::ptr::addr_of_mut!(_28);
(*_23) = _19.3;
place!(Field::<i8>(Variant(_26, 1), 3)) = -95_i8;
match _19.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb5,
5 => bb6,
6 => bb9,
100019082014862154454859111322763820790 => bb11,
_ => bb10
}
}
bb9 = {
_17 = !_12.0.2;
_4 = _6;
_19.1 = 2359071155306296140_u64;
_19.2 = core::ptr::addr_of_mut!(_19.3);
_13 = [4193977985_u32];
_12.0.2 = -_17;
_18 = !true;
_14 = _12.0.0;
_4 = _17 as i64;
RET = [_7,_1,_7,_7,_6];
_12.0.0 = _14;
_16.1 = core::ptr::addr_of!(_19.2);
_17 = 183082154132178054379843658877375718504_u128 as i32;
_16.1 = core::ptr::addr_of!(_19.2);
_18 = !false;
_19.3 = 3290150336_u32 as f32;
_12.0.0 = _14;
_8 = _11 - _11;
_10 = [8089903054991605891694805216507670814_u128,112922464997434960599479053448391923238_u128,331616013703349585733294327890387047114_u128,16636470869096598237597341569406083828_u128];
RET = [_5,_1,_1,_5,_7];
_7 = _4;
_2 = _5;
RET = [_2,_6,_1,_6,_1];
_3 = 164521664475142175845382733836485571279_u128 as i64;
Goto(bb5)
}
bb10 = {
_12.0.2 = -_15;
_2 = _4;
_16.2 = (-95_i8) as usize;
_13 = [594558652_u32];
_12.0.0 = _14;
_22 = _6 as f64;
_26 = Adt21::Variant1 { fld0: _19.3,fld1: Move(_19.2),fld2: _22,fld3: (-106_i8),fld4: 27220_u16,fld5: _17,fld6: _5 };
_16.1 = core::ptr::addr_of!(_19.2);
_19.2 = core::ptr::addr_of_mut!(_19.3);
_5 = _4;
_20.0 = core::ptr::addr_of_mut!(_26);
_19 = ((-106825957616846226205825610207373425850_i128), 13271061733684882622_u64, Move(Field::<*mut f32>(Variant(_26, 1), 1)), Field::<f32>(Variant(_26, 1), 0));
_20.0 = core::ptr::addr_of_mut!(_26);
_27.0 = Move(_20.0);
place!(Field::<*mut f32>(Variant(_26, 1), 1)) = core::ptr::addr_of_mut!(_19.3);
_12.0.2 = -_17;
RET = [_3,_1,_4,_3,_6];
Call(_16.2 = core::intrinsics::transmute(_11), ReturnTo(bb6), UnwindUnreachable())
}
bb11 = {
_13 = [_34];
_9 = [(-5537_i16),17383_i16];
_12.0.2 = _17 & _17;
_36.0.1 = core::ptr::addr_of!(_19.0);
RET = [_3,_2,_1,_2,_5];
_7 = !_3;
_30 = !_33;
_36.0.0 = _14;
_1 = _14 as i64;
_1 = _29 as i64;
place!(Field::<*mut f32>(Variant(_26, 1), 1)) = Move(_23);
SetDiscriminant(_26, 1);
place!(Field::<u16>(Variant(_26, 1), 4)) = !61817_u16;
place!(Field::<i32>(Variant(_26, 1), 5)) = _16.2 as i32;
place!(Field::<i8>(Variant(_26, 1), 3)) = (-99_i8);
_12 = (Move(_36.0),);
_37 = [2236_i16,27120_i16];
_38.0 = -_19.3;
_9 = [(-23391_i16),(-15772_i16)];
_14 = _12.0.0;
place!(Field::<u16>(Variant(_26, 1), 4)) = _19.3 as u16;
_12.0.0 = _14;
_44 = -_11;
_19.0 = Field::<u16>(Variant(_26, 1), 4) as i128;
Goto(bb12)
}
bb12 = {
_37 = [10935_i16,(-22041_i16)];
_43 = [_35,_33,_25];
_38.0 = _19.1 as f32;
_5 = _1;
_28 = _19.3;
_40 = RET;
_45 = core::ptr::addr_of!(_16);
(*_45).0 = &place!(Field::<i8>(Variant(_26, 1), 3));
_32 = &_36.0;
_6 = !_1;
_38.0 = _19.1 as f32;
_15 = _17;
_47 = _14;
_36.0.0 = _14;
(*_45).1 = core::ptr::addr_of!(_23);
_36.0.1 = core::ptr::addr_of!(_19.0);
_16.1 = core::ptr::addr_of!(_19.2);
_16.1 = core::ptr::addr_of!(_23);
place!(Field::<i64>(Variant(_26, 1), 6)) = -_1;
(*_45).2 = _11 as usize;
RET = _40;
_51.0 = _19.3;
_16.1 = core::ptr::addr_of!(place!(Field::<*mut f32>(Variant(_26, 1), 1)));
Goto(bb13)
}
bb13 = {
_20.0 = core::ptr::addr_of_mut!(_26);
place!(Field::<f64>(Variant(_26, 1), 2)) = _22 + _29;
_12.0.2 = Field::<i8>(Variant(_26, 1), 3) as i32;
_17 = !_15;
_36.0.1 = Move(_12.0.1);
_16.1 = core::ptr::addr_of!(_23);
_13 = [_34];
_36.0.1 = core::ptr::addr_of!(_19.0);
_46 = _44 | _11;
_11 = _46 + _46;
_30 = !_35;
match Field::<i8>(Variant(_26, 1), 3) {
0 => bb14,
1 => bb15,
2 => bb16,
340282366920938463463374607431768211357 => bb18,
_ => bb17
}
}
bb14 = {
_12.0.2 = -_15;
_2 = _4;
_16.2 = (-95_i8) as usize;
_13 = [594558652_u32];
_12.0.0 = _14;
_22 = _6 as f64;
_26 = Adt21::Variant1 { fld0: _19.3,fld1: Move(_19.2),fld2: _22,fld3: (-106_i8),fld4: 27220_u16,fld5: _17,fld6: _5 };
_16.1 = core::ptr::addr_of!(_19.2);
_19.2 = core::ptr::addr_of_mut!(_19.3);
_5 = _4;
_20.0 = core::ptr::addr_of_mut!(_26);
_19 = ((-106825957616846226205825610207373425850_i128), 13271061733684882622_u64, Move(Field::<*mut f32>(Variant(_26, 1), 1)), Field::<f32>(Variant(_26, 1), 0));
_20.0 = core::ptr::addr_of_mut!(_26);
_27.0 = Move(_20.0);
place!(Field::<*mut f32>(Variant(_26, 1), 1)) = core::ptr::addr_of_mut!(_19.3);
_12.0.2 = -_17;
RET = [_3,_1,_4,_3,_6];
Call(_16.2 = core::intrinsics::transmute(_11), ReturnTo(bb6), UnwindUnreachable())
}
bb15 = {
_13 = [_34];
_9 = [(-5537_i16),17383_i16];
_12.0.2 = _17 & _17;
_36.0.1 = core::ptr::addr_of!(_19.0);
RET = [_3,_2,_1,_2,_5];
_7 = !_3;
_30 = !_33;
_36.0.0 = _14;
_1 = _14 as i64;
_1 = _29 as i64;
place!(Field::<*mut f32>(Variant(_26, 1), 1)) = Move(_23);
SetDiscriminant(_26, 1);
place!(Field::<u16>(Variant(_26, 1), 4)) = !61817_u16;
place!(Field::<i32>(Variant(_26, 1), 5)) = _16.2 as i32;
place!(Field::<i8>(Variant(_26, 1), 3)) = (-99_i8);
_12 = (Move(_36.0),);
_37 = [2236_i16,27120_i16];
_38.0 = -_19.3;
_9 = [(-23391_i16),(-15772_i16)];
_14 = _12.0.0;
place!(Field::<u16>(Variant(_26, 1), 4)) = _19.3 as u16;
_12.0.0 = _14;
_44 = -_11;
_19.0 = Field::<u16>(Variant(_26, 1), 4) as i128;
Goto(bb12)
}
bb16 = {
_17 = !_12.0.2;
_4 = _6;
_19.1 = 2359071155306296140_u64;
_19.2 = core::ptr::addr_of_mut!(_19.3);
_13 = [4193977985_u32];
_12.0.2 = -_17;
_18 = !true;
_14 = _12.0.0;
_4 = _17 as i64;
RET = [_7,_1,_7,_7,_6];
_12.0.0 = _14;
_16.1 = core::ptr::addr_of!(_19.2);
_17 = 183082154132178054379843658877375718504_u128 as i32;
_16.1 = core::ptr::addr_of!(_19.2);
_18 = !false;
_19.3 = 3290150336_u32 as f32;
_12.0.0 = _14;
_8 = _11 - _11;
_10 = [8089903054991605891694805216507670814_u128,112922464997434960599479053448391923238_u128,331616013703349585733294327890387047114_u128,16636470869096598237597341569406083828_u128];
RET = [_5,_1,_1,_5,_7];
_7 = _4;
_2 = _5;
RET = [_2,_6,_1,_6,_1];
_3 = 164521664475142175845382733836485571279_u128 as i64;
Goto(bb5)
}
bb17 = {
_17 = !_12.0.2;
_4 = _6;
_19.1 = 2359071155306296140_u64;
_19.2 = core::ptr::addr_of_mut!(_19.3);
_13 = [4193977985_u32];
_12.0.2 = -_17;
_18 = !true;
_14 = _12.0.0;
_4 = _17 as i64;
RET = [_7,_1,_7,_7,_6];
_12.0.0 = _14;
_16.1 = core::ptr::addr_of!(_19.2);
_17 = 183082154132178054379843658877375718504_u128 as i32;
_16.1 = core::ptr::addr_of!(_19.2);
_18 = !false;
_19.3 = 3290150336_u32 as f32;
_12.0.0 = _14;
_8 = _11 - _11;
_10 = [8089903054991605891694805216507670814_u128,112922464997434960599479053448391923238_u128,331616013703349585733294327890387047114_u128,16636470869096598237597341569406083828_u128];
RET = [_5,_1,_1,_5,_7];
_7 = _4;
_2 = _5;
RET = [_2,_6,_1,_6,_1];
_3 = 164521664475142175845382733836485571279_u128 as i64;
Goto(bb5)
}
bb18 = {
(*_45).2 = 7_usize - 11100375564543774718_usize;
place!(Field::<f64>(Variant(_26, 1), 2)) = 16_u8 as f64;
_5 = _7 ^ _7;
Goto(bb19)
}
bb19 = {
Call(_55 = dump_var(12_usize, 8_usize, Move(_8), 35_usize, Move(_35), 18_usize, Move(_18), 40_usize, Move(_40)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_55 = dump_var(12_usize, 9_usize, Move(_9), 4_usize, Move(_4), 1_usize, Move(_1), 44_usize, Move(_44)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_55 = dump_var(12_usize, 15_usize, Move(_15), 47_usize, Move(_47), 5_usize, Move(_5), 11_usize, Move(_11)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_55 = dump_var(12_usize, 7_usize, Move(_7), 21_usize, Move(_21), 56_usize, _56, 56_usize, _56), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: *mut Adt21,mut _2: i128,mut _3: i64,mut _4: [u128; 4],mut _5: f64,mut _6: isize,mut _7: f64,mut _8: ((char, *const i128, i32),),mut _9: i64,mut _10: u64,mut _11: u64,mut _12: i32) -> *const *mut f32 {
mir! {
type RET = *const *mut f32;
let _13: i8;
let _14: [i64; 5];
let _15: f64;
let _16: char;
let _17: i16;
let _18: f32;
let _19: [u128; 4];
let _20: ([usize; 5],);
let _21: *mut i64;
let _22: &'static i16;
let _23: [bool; 3];
let _24: [u32; 1];
let _25: (*mut Adt21,);
let _26: [u64; 1];
let _27: f64;
let _28: &'static f64;
let _29: f32;
let _30: ((char, *const i128, i32),);
let _31: [i64; 7];
let _32: (i128, u64, *mut f32, f32);
let _33: isize;
let _34: char;
let _35: ();
let _36: ();
{
_9 = -_3;
_5 = _7;
_12 = !_8.0.2;
_11 = _7 as u64;
_8.0.2 = _12;
_3 = _9 >> _6;
Goto(bb1)
}
bb1 = {
_6 = (-66_isize) >> _2;
_13 = _10 as i8;
_13 = 1755601464208539315_usize as i8;
_4 = [15740325529560120464448843466287840009_u128,33607258554422691022500037412498462161_u128,6586354742920886814019036102913983943_u128,72618242892207129842989412862872736594_u128];
_15 = _2 as f64;
_15 = _5 - _7;
_14 = [_9,_3,_3,_3,_9];
_12 = _13 as i32;
_2 = _3 as i128;
_6 = 9223372036854775807_isize;
_2 = _8.0.2 as i128;
_7 = _15;
_4 = [47903379595970636630348399111150417185_u128,102875110937212776963830010737325055832_u128,193963330126501302436644776406042066971_u128,157684898848022872358806614627878017388_u128];
_12 = _6 as i32;
_12 = _8.0.2 | _8.0.2;
_9 = 144305654716765046542284661997089991530_u128 as i64;
Call(_8.0.2 = fn14(_9, _2, _2, Move(_8.0.1), Move(_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_16 = _8.0.0;
_14 = [_3,_3,_9,_3,_3];
_13 = (-63_i8) | 103_i8;
_11 = _10;
_2 = _8.0.0 as i128;
_6 = (-88_isize) - (-9223372036854775808_isize);
_9 = _3;
Goto(bb3)
}
bb3 = {
_8.0.1 = core::ptr::addr_of!(_2);
_11 = _6 as u64;
_3 = 224_u8 as i64;
match _10 {
0 => bb2,
13271061733684882622 => bb5,
_ => bb4
}
}
bb4 = {
_16 = _8.0.0;
_14 = [_3,_3,_9,_3,_3];
_13 = (-63_i8) | 103_i8;
_11 = _10;
_2 = _8.0.0 as i128;
_6 = (-88_isize) - (-9223372036854775808_isize);
_9 = _3;
Goto(bb3)
}
bb5 = {
_6 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_8.0.1 = core::ptr::addr_of!(_2);
_7 = -_15;
_13 = (-97_i8);
_10 = _11;
_8.0.0 = _16;
_4 = [186877902127226351434481895498047569725_u128,270247258550349748525399745681587146736_u128,213807709633785052605358490925559818014_u128,604843030202092773617230777248006647_u128];
_11 = !_10;
_17 = (-26542_i16) >> _6;
_18 = _5 as f32;
_8.0.2 = _3 as i32;
_6 = -(-9223372036854775808_isize);
_2 = 74618739225112778109444181487228839354_i128;
_17 = (-30382_i16);
_16 = _8.0.0;
_8.0.0 = _16;
Goto(bb6)
}
bb6 = {
_18 = _10 as f32;
_15 = 2994231540_u32 as f64;
_8.0.1 = core::ptr::addr_of!(_2);
_8.0.2 = _5 as i32;
_19 = [335705131422068706092994537768914797833_u128,320105934368011140966136029121036408673_u128,122215070167782065697680444121585022164_u128,165575441123267917473316613287286008943_u128];
_15 = -_7;
_14 = [_9,_9,_9,_9,_9];
_15 = _13 as f64;
_24 = [1729887744_u32];
_8.0.0 = _16;
_14 = [_3,_9,_9,_9,_9];
_5 = -_7;
_13 = 8819511479899251616_usize as i8;
_22 = &_17;
_20.0 = [3165805074379615514_usize,7499348579305233254_usize,7368560696509404994_usize,13387970343279366645_usize,0_usize];
_2 = 35_u8 as i128;
_10 = _11;
_20.0 = [2_usize,5_usize,17798095690381883705_usize,7_usize,335987072670706416_usize];
_13 = -(-125_i8);
_14 = [_9,_3,_9,_9,_9];
match _17 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
340282366920938463463374607431768181074 => bb14,
_ => bb13
}
}
bb7 = {
_6 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_8.0.1 = core::ptr::addr_of!(_2);
_7 = -_15;
_13 = (-97_i8);
_10 = _11;
_8.0.0 = _16;
_4 = [186877902127226351434481895498047569725_u128,270247258550349748525399745681587146736_u128,213807709633785052605358490925559818014_u128,604843030202092773617230777248006647_u128];
_11 = !_10;
_17 = (-26542_i16) >> _6;
_18 = _5 as f32;
_8.0.2 = _3 as i32;
_6 = -(-9223372036854775808_isize);
_2 = 74618739225112778109444181487228839354_i128;
_17 = (-30382_i16);
_16 = _8.0.0;
_8.0.0 = _16;
Goto(bb6)
}
bb8 = {
_16 = _8.0.0;
_14 = [_3,_3,_9,_3,_3];
_13 = (-63_i8) | 103_i8;
_11 = _10;
_2 = _8.0.0 as i128;
_6 = (-88_isize) - (-9223372036854775808_isize);
_9 = _3;
Goto(bb3)
}
bb9 = {
_8.0.1 = core::ptr::addr_of!(_2);
_11 = _6 as u64;
_3 = 224_u8 as i64;
match _10 {
0 => bb2,
13271061733684882622 => bb5,
_ => bb4
}
}
bb10 = {
_16 = _8.0.0;
_14 = [_3,_3,_9,_3,_3];
_13 = (-63_i8) | 103_i8;
_11 = _10;
_2 = _8.0.0 as i128;
_6 = (-88_isize) - (-9223372036854775808_isize);
_9 = _3;
Goto(bb3)
}
bb11 = {
_6 = (-66_isize) >> _2;
_13 = _10 as i8;
_13 = 1755601464208539315_usize as i8;
_4 = [15740325529560120464448843466287840009_u128,33607258554422691022500037412498462161_u128,6586354742920886814019036102913983943_u128,72618242892207129842989412862872736594_u128];
_15 = _2 as f64;
_15 = _5 - _7;
_14 = [_9,_3,_3,_3,_9];
_12 = _13 as i32;
_2 = _3 as i128;
_6 = 9223372036854775807_isize;
_2 = _8.0.2 as i128;
_7 = _15;
_4 = [47903379595970636630348399111150417185_u128,102875110937212776963830010737325055832_u128,193963330126501302436644776406042066971_u128,157684898848022872358806614627878017388_u128];
_12 = _6 as i32;
_12 = _8.0.2 | _8.0.2;
_9 = 144305654716765046542284661997089991530_u128 as i64;
Call(_8.0.2 = fn14(_9, _2, _2, Move(_8.0.1), Move(_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_4 = [296886717241549672343884689187342423435_u128,83232241695923349663409144919287073521_u128,89387611155928945025834477991474358957_u128,323875752419611017217332889553640380491_u128];
_10 = !_11;
_24 = [1616471662_u32];
_21 = core::ptr::addr_of_mut!(_3);
_20.0 = [0_usize,3_usize,5_usize,1_usize,1_usize];
Goto(bb15)
}
bb15 = {
_14 = [_3,_3,_9,_9,_9];
_5 = _7 + _15;
_26 = [_11];
_6 = 4_isize;
_16 = _8.0.0;
_13 = 22_i8 << (*_22);
(*_21) = _9 ^ _9;
_2 = 54_u8 as i128;
_8.0.1 = core::ptr::addr_of!(_2);
_4 = [81459789628669222940339921200423312750_u128,251675439184206065270142041970547050601_u128,184105182163313179210119609636844416973_u128,231379840818987060090533399075975092719_u128];
_16 = _8.0.0;
(*_21) = -_9;
_28 = &_7;
_27 = 7375909196184253646_usize as f64;
_7 = _27 + _15;
_29 = _18;
_28 = &_27;
_24 = [4233783907_u32];
_8.0.2 = _12;
_23 = [true,true,false];
Goto(bb16)
}
bb16 = {
_23 = [true,true,false];
_28 = &_27;
_16 = _8.0.0;
_8.0.2 = 155_u8 as i32;
_4 = [17899653540349875540220953784020711301_u128,232376293090932566035879753198709913172_u128,203373943512374339732246161136055274774_u128,86892148690615852843504830198148177595_u128];
_8.0.2 = _12;
_30.0.0 = _16;
_2 = _30.0.0 as i128;
_30.0.1 = Move(_8.0.1);
_5 = _15;
_26 = [_11];
_2 = (-106489666451707022709507040885124230572_i128) & (-124837490486288009193763958926141021953_i128);
_21 = core::ptr::addr_of_mut!((*_21));
_13 = (-71_i8) << _3;
_23 = [false,true,false];
Goto(bb17)
}
bb17 = {
_16 = _8.0.0;
_27 = -_7;
_8.0.2 = _12;
_23 = [false,false,false];
_8.0.1 = core::ptr::addr_of!(_2);
_6 = 9223372036854775807_isize;
_31 = [_3,_3,(*_21),_9,(*_21),(*_21),(*_21)];
_32.1 = _11 + _11;
RET = core::ptr::addr_of!(_32.2);
_32.0 = _2 - _2;
_13 = 46_i8 << _32.0;
_18 = _29;
(*RET) = core::ptr::addr_of_mut!(_18);
_7 = _32.0 as f64;
_21 = core::ptr::addr_of_mut!((*_21));
_6 = 126_isize ^ 9223372036854775807_isize;
_30.0.2 = _8.0.2 << (*_21);
(*_21) = -_9;
_11 = 82153816212860767194665614480272247778_u128 as u64;
Goto(bb18)
}
bb18 = {
Call(_35 = dump_var(13_usize, 13_usize, Move(_13), 3_usize, Move(_3), 26_usize, Move(_26), 11_usize, Move(_11)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_35 = dump_var(13_usize, 17_usize, Move(_17), 31_usize, Move(_31), 14_usize, Move(_14), 10_usize, Move(_10)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_35 = dump_var(13_usize, 20_usize, Move(_20), 36_usize, _36, 36_usize, _36, 36_usize, _36), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: i64,mut _2: i128,mut _3: i128,mut _4: *const i128,mut _5: *mut Adt21) -> i32 {
mir! {
type RET = i32;
let _6: *const i128;
let _7: isize;
let _8: char;
let _9: isize;
let _10: f64;
let _11: *mut *mut f32;
let _12: char;
let _13: [i16; 3];
let _14: [u16; 3];
let _15: ([usize; 5],);
let _16: char;
let _17: isize;
let _18: *const *mut f32;
let _19: &'static i64;
let _20: u8;
let _21: isize;
let _22: u32;
let _23: *const i16;
let _24: [u64; 7];
let _25: *mut &'static i8;
let _26: Adt31;
let _27: f32;
let _28: ();
let _29: ();
{
_1 = (-9215673329647911944_i64) + (-4017661793587138445_i64);
RET = (-2008094997_i32) & 1479915744_i32;
_1 = !4677925229302137641_i64;
_4 = core::ptr::addr_of!(_3);
(*_4) = -_2;
(*_4) = _2 ^ _2;
_6 = core::ptr::addr_of!((*_4));
(*_4) = _2;
_4 = Move(_6);
_2 = _3;
RET = 1828480476_i32 >> _1;
_9 = (-78_isize);
_6 = Move(_4);
_8 = '\u{46afe}';
_2 = -_3;
_4 = core::ptr::addr_of!(_3);
_4 = core::ptr::addr_of!(_2);
_6 = core::ptr::addr_of!(_2);
_4 = core::ptr::addr_of!((*_6));
(*_6) = _3 + _3;
Call(RET = core::intrinsics::bswap((-1181535064_i32)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = 43164_u16 as f64;
_8 = '\u{c9301}';
_7 = RET as isize;
_8 = '\u{5db5}';
RET = true as i32;
_3 = _2 | (*_4);
_8 = '\u{ab8dc}';
(*_6) = !_3;
_4 = core::ptr::addr_of!((*_6));
RET = (-812491675_i32);
_7 = 43441_u16 as isize;
RET = _10 as i32;
_7 = !_9;
_2 = _3;
RET = !(-1589423533_i32);
_2 = 3944376097_u32 as i128;
_14 = [20770_u16,33957_u16,53485_u16];
_13 = [(-28662_i16),(-10088_i16),26006_i16];
_8 = '\u{10a3b9}';
RET = (-1706431971_i32) + 283970502_i32;
_4 = Move(_6);
_13 = [(-3503_i16),4580_i16,7025_i16];
_3 = !_2;
_13 = [27792_i16,(-32008_i16),1034_i16];
match _9 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768211378 => bb8,
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
_12 = _8;
_15.0 = [12368557482234209907_usize,1_usize,4_usize,17485315064657255570_usize,17742933228457601727_usize];
_3 = RET as i128;
_4 = core::ptr::addr_of!(_3);
_8 = _12;
_2 = _3;
_12 = _8;
_13 = [(-32377_i16),(-10811_i16),4765_i16];
_2 = _1 as i128;
RET = (-2030198607_i32) >> _2;
_1 = -(-490904382246412652_i64);
_6 = Move(_4);
_14 = [11382_u16,38752_u16,29756_u16];
_14 = [23968_u16,57576_u16,56603_u16];
_12 = _8;
_7 = _9;
RET = 7023633720865871943_usize as i32;
_12 = _8;
_17 = _9;
_2 = _3 * _3;
_15.0 = [0_usize,18172613029939359051_usize,4_usize,16390030560698247081_usize,5521207998620263642_usize];
Goto(bb9)
}
bb9 = {
_4 = Move(_6);
_8 = _12;
Goto(bb10)
}
bb10 = {
_6 = core::ptr::addr_of!(_2);
_17 = -_9;
_16 = _8;
_15.0 = [8134941524956389940_usize,5490033236923776213_usize,3_usize,4_usize,14253574868019882642_usize];
(*_6) = _3;
Goto(bb11)
}
bb11 = {
_7 = _9 * _17;
_7 = 6335074203734920164_u64 as isize;
_7 = -_17;
_1 = 7322179691083116870_i64;
_3 = _2;
_2 = !_3;
_19 = &_1;
RET = _3 as i32;
_19 = &_1;
_3 = -(*_6);
_13 = [(-5767_i16),(-24881_i16),24642_i16];
(*_6) = 8728237475470679492_u64 as i128;
_8 = _12;
_10 = 19551_i16 as f64;
_19 = &(*_19);
(*_6) = !_3;
_15.0 = [11574965556524887210_usize,3325245566933678218_usize,0_usize,2211437596227966615_usize,6_usize];
RET = (-258816451_i32) + 303843139_i32;
(*_6) = -_3;
_19 = &(*_19);
_4 = Move(_6);
_15.0 = [12350746343285259071_usize,276207706381392848_usize,5863817802162124841_usize,2974255295086401479_usize,2155713219713930639_usize];
_22 = _8 as u32;
_13 = [(-25766_i16),(-3210_i16),13459_i16];
_21 = _9;
_3 = 91_i8 as i128;
_16 = _12;
_12 = _8;
Call(_15.0 = fn15(Move(_5), _21), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_19 = &_1;
_13 = [(-10806_i16),11977_i16,4331_i16];
_12 = _8;
_20 = 3505756479614216267_u64 as u8;
match _21 {
0 => bb10,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
6 => bb18,
340282366920938463463374607431768211378 => bb20,
_ => bb19
}
}
bb13 = {
_7 = _9 * _17;
_7 = 6335074203734920164_u64 as isize;
_7 = -_17;
_1 = 7322179691083116870_i64;
_3 = _2;
_2 = !_3;
_19 = &_1;
RET = _3 as i32;
_19 = &_1;
_3 = -(*_6);
_13 = [(-5767_i16),(-24881_i16),24642_i16];
(*_6) = 8728237475470679492_u64 as i128;
_8 = _12;
_10 = 19551_i16 as f64;
_19 = &(*_19);
(*_6) = !_3;
_15.0 = [11574965556524887210_usize,3325245566933678218_usize,0_usize,2211437596227966615_usize,6_usize];
RET = (-258816451_i32) + 303843139_i32;
(*_6) = -_3;
_19 = &(*_19);
_4 = Move(_6);
_15.0 = [12350746343285259071_usize,276207706381392848_usize,5863817802162124841_usize,2974255295086401479_usize,2155713219713930639_usize];
_22 = _8 as u32;
_13 = [(-25766_i16),(-3210_i16),13459_i16];
_21 = _9;
_3 = 91_i8 as i128;
_16 = _12;
_12 = _8;
Call(_15.0 = fn15(Move(_5), _21), ReturnTo(bb12), UnwindUnreachable())
}
bb14 = {
Return()
}
bb15 = {
_4 = Move(_6);
_8 = _12;
Goto(bb10)
}
bb16 = {
_12 = _8;
_15.0 = [12368557482234209907_usize,1_usize,4_usize,17485315064657255570_usize,17742933228457601727_usize];
_3 = RET as i128;
_4 = core::ptr::addr_of!(_3);
_8 = _12;
_2 = _3;
_12 = _8;
_13 = [(-32377_i16),(-10811_i16),4765_i16];
_2 = _1 as i128;
RET = (-2030198607_i32) >> _2;
_1 = -(-490904382246412652_i64);
_6 = Move(_4);
_14 = [11382_u16,38752_u16,29756_u16];
_14 = [23968_u16,57576_u16,56603_u16];
_12 = _8;
_7 = _9;
RET = 7023633720865871943_usize as i32;
_12 = _8;
_17 = _9;
_2 = _3 * _3;
_15.0 = [0_usize,18172613029939359051_usize,4_usize,16390030560698247081_usize,5521207998620263642_usize];
Goto(bb9)
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
_20 = _2 as u8;
_13 = [31693_i16,(-517_i16),15872_i16];
_1 = (-7113368852254375283_i64) * (-5931737451912552206_i64);
_21 = _7 - _9;
_9 = !_17;
_2 = _3 ^ _3;
RET = !(-1835760151_i32);
_7 = 40046_u16 as isize;
_16 = _12;
_24 = [12116066307748534514_u64,15084377748177475282_u64,17665211092710047463_u64,3964208302131643423_u64,18400279704937817476_u64,16018638776967862001_u64,16840040856242752981_u64];
_7 = -_21;
_14 = [59502_u16,36927_u16,13570_u16];
_24 = [5737029939697625968_u64,1120128472244736739_u64,10669047777967456142_u64,13661834036026943103_u64,17908428348150694140_u64,14128861251522107633_u64,14837229424194334442_u64];
_20 = 77_u8 & 109_u8;
_6 = core::ptr::addr_of!(_2);
_12 = _8;
Goto(bb21)
}
bb21 = {
Call(_28 = dump_var(14_usize, 24_usize, Move(_24), 8_usize, Move(_8), 22_usize, Move(_22), 13_usize, Move(_13)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_28 = dump_var(14_usize, 9_usize, Move(_9), 1_usize, Move(_1), 17_usize, Move(_17), 20_usize, Move(_20)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: *mut Adt21,mut _2: isize) -> [usize; 5] {
mir! {
type RET = [usize; 5];
let _3: ((char, *const i128, i32),);
let _4: i64;
let _5: [i32; 8];
let _6: f64;
let _7: u128;
let _8: &'static (Adt22, i64, isize);
let _9: i64;
let _10: *mut *mut f32;
let _11: isize;
let _12: &'static i16;
let _13: &'static (char, *const i128, i32);
let _14: u128;
let _15: isize;
let _16: char;
let _17: *mut f32;
let _18: char;
let _19: (*const i16,);
let _20: [u16; 3];
let _21: char;
let _22: ([usize; 5], *const i16, (char, *const i128, i32), [u16; 3]);
let _23: f32;
let _24: isize;
let _25: f32;
let _26: i64;
let _27: &'static i16;
let _28: Adt77;
let _29: bool;
let _30: [bool; 8];
let _31: [i8; 5];
let _32: *const &'static &'static isize;
let _33: [u128; 4];
let _34: isize;
let _35: bool;
let _36: *const ([usize; 5],);
let _37: isize;
let _38: ();
let _39: ();
{
RET = [1381884643177771967_usize,2897915836627226004_usize,3303268612152018043_usize,6740886701898820845_usize,2590680820246791242_usize];
RET = [3533792140175253475_usize,341809803073795121_usize,3207523349639402978_usize,11544348798547644091_usize,6_usize];
RET = [16572894107240921692_usize,7_usize,7_usize,3_usize,3560000850493399092_usize];
_3.0.0 = '\u{c934a}';
RET = [4770340234542671691_usize,1_usize,801348707893668316_usize,6361041633612373895_usize,2_usize];
RET = [2451192034676289752_usize,2003297486949026593_usize,16776644255946345884_usize,15955142081173892708_usize,5460007034743427918_usize];
RET = [4_usize,18301640712352169802_usize,0_usize,5_usize,14587851324635365145_usize];
RET = [4668524523492807001_usize,0_usize,11688532763108004901_usize,4_usize,4049495758333434994_usize];
_2 = !(-9223372036854775808_isize);
RET = [3_usize,6_usize,16610791908475053705_usize,7_usize,6635549969025555437_usize];
RET = [0_usize,4_usize,4_usize,1789298490550583740_usize,5_usize];
_3.0.2 = -(-1185961667_i32);
RET = [6_usize,2_usize,7143149781416407270_usize,6656444446809723901_usize,14541346039780890129_usize];
RET = [6_usize,3191012767552407339_usize,1_usize,3_usize,4_usize];
_2 = 9223372036854775807_isize | 9223372036854775807_isize;
_3.0.0 = '\u{c6eec}';
_2 = (-9223372036854775808_isize) >> _3.0.2;
Call(_2 = fn16(_3.0.2, Move(_1), _3.0.0, RET, RET, RET, RET, RET, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [2313529724103314076_usize,7_usize,0_usize,8530199258035433314_usize,3_usize];
_3.0.2 = (-995416488_i32) << _2;
_4 = 8158025092304567894_i64;
RET = [1_usize,2_usize,7_usize,4_usize,13306641421872929814_usize];
_4 = !3848065966748486297_i64;
RET = [7_usize,16306589784672331346_usize,3_usize,3_usize,2_usize];
_4 = 3443533364483606840_i64;
_5 = [_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2];
_2 = !9223372036854775807_isize;
_6 = 56157_u16 as f64;
_5 = [_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2];
RET = [6_usize,3_usize,13583427564850397617_usize,2325882376462427938_usize,8767055696511954276_usize];
_6 = (-77660965069367765353821742547781914736_i128) as f64;
_6 = 113_u8 as f64;
_3.0.0 = '\u{ac12f}';
Goto(bb2)
}
bb2 = {
_5 = [_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2];
_5 = [_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2];
_3.0.2 = -(-1618589743_i32);
_3.0.0 = '\u{acd88}';
_3.0.2 = !(-200732518_i32);
_7 = !85996981064236122546149034509948079837_u128;
_4 = -4235862530325712957_i64;
_6 = _4 as f64;
_3.0.0 = '\u{7cd70}';
_9 = _3.0.0 as i64;
_5 = [_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2];
_5 = [_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2];
_5 = [_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2];
_11 = 4067600434_u32 as isize;
_11 = !_2;
RET = [6_usize,2_usize,1159138154573165054_usize,2_usize,6_usize];
_3.0.0 = '\u{e044d}';
_3.0.2 = 1969659391_i32 * 191433773_i32;
_3.0.0 = '\u{6df78}';
_11 = _2 + _2;
_6 = _3.0.2 as f64;
_6 = _3.0.2 as f64;
_6 = (-102742413293889004035256100087059086980_i128) as f64;
_6 = 27467736035946257817984766596313504977_i128 as f64;
_7 = 251730730619020655682238716536024287442_u128 | 283279698507298388925641288026368900952_u128;
_9 = -_4;
Goto(bb3)
}
bb3 = {
_5 = [_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2];
_3.0.0 = '\u{41513}';
Goto(bb4)
}
bb4 = {
RET = [6_usize,15727237562666505283_usize,7_usize,13222306694941334516_usize,2463662524313093929_usize];
_13 = &_3.0;
_4 = !_9;
_3.0.0 = '\u{b0234}';
_3.0.2 = (-1568090335_i32);
_14 = _7 >> _4;
_15 = _2;
_16 = _3.0.0;
_11 = _2 ^ _15;
match _3.0.2 {
0 => bb1,
1 => bb3,
340282366920938463463374607430200121121 => bb6,
_ => bb5
}
}
bb5 = {
_5 = [_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2];
_3.0.0 = '\u{41513}';
Goto(bb4)
}
bb6 = {
_6 = 92_i8 as f64;
_16 = _3.0.0;
_3.0.2 = 11957_u16 as i32;
_16 = _3.0.0;
_3.0.0 = _16;
_7 = _3.0.0 as u128;
_15 = 10697092822077919889_usize as isize;
_13 = &_3.0;
RET = [7_usize,6_usize,6035690254272921939_usize,0_usize,6261761842564212225_usize];
Goto(bb7)
}
bb7 = {
_6 = 14176391646646220934_u64 as f64;
_9 = _4;
_18 = (*_13).0;
Goto(bb8)
}
bb8 = {
_13 = &_3.0;
_2 = -_15;
_13 = &(*_13);
_7 = _14 + _14;
_6 = 3082310945039457787_u64 as f64;
_4 = _9 << _14;
_6 = 7998_u16 as f64;
_3.0.2 = (-311216051_i32) ^ (-990206766_i32);
_13 = &_3.0;
_9 = (-26427_i16) as i64;
RET = [4_usize,10342920528309683188_usize,1_usize,309996862087365333_usize,5_usize];
_6 = (-11015_i16) as f64;
_3.0.2 = 1043000050_i32;
_16 = (*_13).0;
_5 = [_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2];
_14 = !_7;
_22.3 = [17641_u16,31714_u16,15299_u16];
_21 = _18;
_22.2.2 = !_3.0.2;
_18 = _21;
match _3.0.2 {
0 => bb9,
1043000050 => bb11,
_ => bb10
}
}
bb9 = {
_6 = 14176391646646220934_u64 as f64;
_9 = _4;
_18 = (*_13).0;
Goto(bb8)
}
bb10 = {
_5 = [_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2,_3.0.2];
_3.0.0 = '\u{41513}';
Goto(bb4)
}
bb11 = {
_23 = 32330_i16 as f32;
_23 = 33_u8 as f32;
_2 = _11;
_7 = _14;
_26 = _9;
RET = [9826907353098169576_usize,1_usize,6_usize,17034567046862624660_usize,11306090931568449077_usize];
_3.0.0 = _16;
_22.0 = [0_usize,15602047192813873959_usize,11858671348478419919_usize,788944164289615567_usize,8293116535713135550_usize];
RET = [6_usize,6_usize,11478886073096057106_usize,1_usize,7_usize];
_11 = _2;
_9 = _4 * _4;
_7 = _14;
_3.0.2 = _22.2.2;
_3.0.2 = _6 as i32;
_7 = false as u128;
_2 = _11;
_5 = [_3.0.2,_3.0.2,_22.2.2,_22.2.2,_3.0.2,_22.2.2,_3.0.2,_22.2.2];
_17 = core::ptr::addr_of_mut!(_23);
_22.2.2 = 4151433617_u32 as i32;
_22.2.2 = -_3.0.2;
Goto(bb12)
}
bb12 = {
_25 = (*_17) + _23;
_9 = _4 & _4;
_30 = [true,false,true,false,false,true,true,true];
_4 = _9 << _9;
_22.0 = RET;
_10 = core::ptr::addr_of_mut!(_17);
_16 = _21;
_21 = _16;
_22.2.0 = _16;
_33 = [_14,_14,_14,_14];
_25 = (*_17);
(*_10) = core::ptr::addr_of_mut!((*_17));
Goto(bb13)
}
bb13 = {
_13 = &_22.2;
_13 = &(*_13);
_11 = (*_13).2 as isize;
_25 = -(*_17);
_30 = [true,true,true,true,false,true,false,true];
_29 = true;
_15 = !_2;
_21 = (*_13).0;
_23 = -_25;
_26 = _4 & _4;
_15 = _2;
_35 = _29;
_16 = _21;
_34 = !_2;
_14 = _7;
_7 = 2584450469_u32 as u128;
_9 = _35 as i64;
(*_10) = core::ptr::addr_of_mut!(_23);
_5 = [(*_13).2,_22.2.2,_22.2.2,_22.2.2,(*_13).2,_22.2.2,_22.2.2,(*_13).2];
_30 = [_35,_35,_29,_29,_35,_29,_35,_35];
_30 = [_35,_35,_29,_35,_29,_35,_29,_35];
_6 = 151_u8 as f64;
_21 = (*_13).0;
Goto(bb14)
}
bb14 = {
_23 = _25 * _25;
_24 = _2 * _11;
_35 = !_29;
_10 = core::ptr::addr_of_mut!((*_10));
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(15_usize, 24_usize, Move(_24), 2_usize, Move(_2), 33_usize, Move(_33), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(15_usize, 34_usize, Move(_34), 9_usize, Move(_9), 18_usize, Move(_18), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(15_usize, 4_usize, Move(_4), 39_usize, _39, 39_usize, _39, 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: i32,mut _2: *mut Adt21,mut _3: char,mut _4: [usize; 5],mut _5: [usize; 5],mut _6: [usize; 5],mut _7: [usize; 5],mut _8: [usize; 5],mut _9: [usize; 5]) -> isize {
mir! {
type RET = isize;
let _10: (i128, u64, *mut f32, f32);
let _11: isize;
let _12: [i8; 5];
let _13: bool;
let _14: (f32,);
let _15: i32;
let _16: f32;
let _17: f64;
let _18: bool;
let _19: u64;
let _20: *const i128;
let _21: ([usize; 5], *const i16, (char, *const i128, i32), [u16; 3]);
let _22: &'static i64;
let _23: char;
let _24: [i16; 6];
let _25: f32;
let _26: *const Adt20;
let _27: f64;
let _28: [i32; 8];
let _29: &'static f64;
let _30: *mut *mut u32;
let _31: [u64; 7];
let _32: f64;
let _33: [u128; 4];
let _34: [bool; 8];
let _35: bool;
let _36: (char, *const i128, i32);
let _37: isize;
let _38: ();
let _39: ();
{
_7 = [6_usize,4148948188641291548_usize,15648081920612208998_usize,9638270453563357925_usize,0_usize];
_9 = _4;
_5 = [5_usize,15502972995488827841_usize,0_usize,2_usize,3_usize];
RET = -(-9223372036854775808_isize);
RET = -48_isize;
RET = 48_isize + (-9223372036854775808_isize);
_10.0 = (-21694403252625349361591312942192089951_i128) << RET;
_10.3 = RET as f32;
_10.2 = core::ptr::addr_of_mut!(_10.3);
_3 = '\u{35da7}';
RET = (-9223372036854775808_isize);
_6 = _7;
_9 = [12679591906608637176_usize,4_usize,7_usize,3_usize,4_usize];
_6 = _8;
RET = (-9223372036854775808_isize) << _10.0;
_10.1 = !6319520252960610877_u64;
Call(_5 = fn17(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = [5_usize,13251227154418127769_usize,995362200588240195_usize,2_usize,3_usize];
_10.3 = 9660_u16 as f32;
_10.0 = 338846666008397287304060203153907888708_u128 as i128;
_6 = [3_usize,7807067642284857676_usize,2_usize,13476762410677084891_usize,1_usize];
_6 = [7_usize,11565827394779755276_usize,1_usize,1_usize,4_usize];
RET = 12595797114479004533_usize as isize;
_11 = RET;
Goto(bb2)
}
bb2 = {
_12 = [(-61_i8),(-30_i8),(-3_i8),44_i8,(-21_i8)];
_1 = _10.0 as i32;
_10.0 = _10.3 as i128;
_5 = _7;
RET = _11 >> _11;
_1 = 1148847314_i32 ^ (-1105714358_i32);
_10.0 = 78133680906028619727286074260116953323_i128;
_10.0 = -123018640358071325981329435502442522898_i128;
_10.1 = (-16560_i16) as u64;
_12 = [(-93_i8),(-104_i8),2_i8,8_i8,(-89_i8)];
_11 = 6_usize as isize;
_5 = [16221046733715063183_usize,8616995913335150537_usize,7_usize,14062275088779483425_usize,11667888711867255366_usize];
_11 = RET & RET;
_11 = RET * RET;
_10.1 = _3 as u64;
_10.2 = core::ptr::addr_of_mut!(_14.0);
_14 = (_10.3,);
_12 = [115_i8,(-78_i8),94_i8,62_i8,(-81_i8)];
_16 = -_14.0;
_9 = _4;
_10.3 = _14.0 * _14.0;
_4 = [7898990543419435213_usize,11403097162016522168_usize,2_usize,0_usize,12405659454985237187_usize];
_15 = (-6151785356114846967_i64) as i32;
_16 = -_10.3;
Call(_7 = fn18(_12), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9 = _4;
RET = !_11;
_10.2 = core::ptr::addr_of_mut!(_14.0);
_10.2 = core::ptr::addr_of_mut!(_14.0);
_9 = _4;
_6 = [3_usize,8057475698403947955_usize,8248694918359307933_usize,1_usize,3_usize];
_1 = _15;
_10.2 = core::ptr::addr_of_mut!(_10.3);
_10.1 = !14451838418673070450_u64;
_17 = 56173_u16 as f64;
_20 = core::ptr::addr_of!(_10.0);
_15 = _1 >> (*_20);
(*_20) = -128820391417874930074201182090660257877_i128;
RET = -_11;
_19 = _10.1;
_12 = [(-43_i8),(-82_i8),(-54_i8),(-78_i8),47_i8];
_10.2 = core::ptr::addr_of_mut!(_10.3);
_9 = [16253925668863892644_usize,15676646123171668993_usize,7209601421251078287_usize,3534281045698127364_usize,6_usize];
_17 = _15 as f64;
_1 = -_15;
(*_20) = (-110442917746862394612518659844495854217_i128);
_13 = (*_20) == (*_20);
_21.2.1 = core::ptr::addr_of!((*_20));
_10.1 = _19;
Goto(bb4)
}
bb4 = {
_21.2.1 = Move(_20);
_12 = [(-37_i8),94_i8,123_i8,(-45_i8),62_i8];
_18 = !_13;
_7 = [7_usize,0_usize,11800723231566968159_usize,12141001778105706019_usize,6_usize];
_9 = [6_usize,14476978884200210565_usize,16124523736112090872_usize,2999019446417250787_usize,4829620198912985895_usize];
_10.2 = core::ptr::addr_of_mut!(_10.3);
_21.2.0 = _3;
_21.3 = [52161_u16,30438_u16,12580_u16];
_17 = _10.1 as f64;
_21.2.2 = -_1;
_20 = core::ptr::addr_of!(_10.0);
_21.2.2 = 1863503423_u32 as i32;
_19 = _10.1 << RET;
(*_20) = !63444146492072037338460893008680817858_i128;
_10.2 = core::ptr::addr_of_mut!(_14.0);
_21.2 = (_3, Move(_20), _1);
_21.3 = [12338_u16,33146_u16,41406_u16];
_8 = _9;
_10.3 = _14.0 - _16;
_18 = _13;
_21.2.0 = _3;
RET = _11 << _21.2.2;
_15 = _1 >> RET;
_15 = !_21.2.2;
_21.2.1 = core::ptr::addr_of!(_10.0);
_9 = [7768443658461003195_usize,4_usize,18408510535074676227_usize,13450033357964288352_usize,12819922120638234994_usize];
Goto(bb5)
}
bb5 = {
_6 = _7;
RET = !_11;
_7 = [0_usize,4_usize,4531202924642393286_usize,4228576261864696641_usize,4_usize];
Goto(bb6)
}
bb6 = {
_21.0 = _5;
_16 = _10.3;
Goto(bb7)
}
bb7 = {
_20 = Move(_21.2.1);
_18 = _11 == RET;
_21.2 = (_3, Move(_20), _15);
Goto(bb8)
}
bb8 = {
_10.3 = 6097_u16 as f32;
_23 = _21.2.0;
_10.2 = core::ptr::addr_of_mut!(_25);
_25 = _14.0;
Goto(bb9)
}
bb9 = {
_3 = _21.2.0;
RET = _19 as isize;
_15 = _1 * _21.2.2;
_1 = -_15;
Goto(bb10)
}
bb10 = {
_12 = [119_i8,114_i8,101_i8,(-95_i8),32_i8];
_27 = _11 as f64;
_8 = [6_usize,13748811711775472733_usize,2_usize,4_usize,4198833181354827310_usize];
_18 = !_13;
_16 = _14.0 * _14.0;
_21.2.0 = _3;
_29 = &_17;
_24 = [(-6157_i16),(-24556_i16),27630_i16,29143_i16,(-1648_i16),26559_i16];
_9 = [5_usize,6401133480645444744_usize,13595728769640058269_usize,3384694086468876337_usize,9736612453246106842_usize];
_21.2.1 = core::ptr::addr_of!(_10.0);
Goto(bb11)
}
bb11 = {
_14.0 = 2029489987_u32 as f32;
_14 = (_16,);
_10.0 = 399393049562389322715724134008731276_i128 * 98132187470066462157190684337889782245_i128;
_15 = 8954338467880526424_usize as i32;
_28 = [_21.2.2,_15,_1,_21.2.2,_1,_1,_1,_21.2.2];
_21.2.0 = _23;
_21.3 = [40818_u16,56066_u16,57512_u16];
_10.1 = _19 + _19;
_21.0 = [6_usize,16358915302204933697_usize,1_usize,11196221686621104270_usize,18179810478986046001_usize];
_3 = _23;
_7 = [5_usize,2_usize,7_usize,4_usize,3957051580917830291_usize];
_19 = _10.1;
_3 = _23;
Goto(bb12)
}
bb12 = {
_20 = Move(_21.2.1);
_7 = _4;
_21.2.1 = Move(_20);
_24 = [23907_i16,9147_i16,30160_i16,(-23147_i16),8619_i16,(-27767_i16)];
_3 = _21.2.0;
Goto(bb13)
}
bb13 = {
_36.1 = Move(_21.2.1);
Goto(bb14)
}
bb14 = {
_10.1 = _19 * _19;
_23 = _21.2.0;
_36.0 = _23;
_21.2 = (_3, Move(_36.1), _1);
_31 = [_19,_19,_19,_19,_19,_19,_10.1];
_10.2 = core::ptr::addr_of_mut!(_14.0);
_5 = [2_usize,5_usize,4_usize,6669973446259747562_usize,0_usize];
_4 = [2_usize,15283851979004128747_usize,2_usize,7955725499337204248_usize,4_usize];
_28 = [_1,_1,_21.2.2,_1,_15,_21.2.2,_21.2.2,_1];
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(16_usize, 24_usize, Move(_24), 8_usize, Move(_8), 6_usize, Move(_6), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(16_usize, 18_usize, Move(_18), 5_usize, Move(_5), 3_usize, Move(_3), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(16_usize, 12_usize, Move(_12), 39_usize, _39, 39_usize, _39, 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17() -> [usize; 5] {
mir! {
type RET = [usize; 5];
let _1: char;
let _2: &'static f64;
let _3: u128;
let _4: (u8, usize);
let _5: &'static &'static isize;
let _6: isize;
let _7: (*mut Adt21,);
let _8: (f32,);
let _9: u16;
let _10: bool;
let _11: bool;
let _12: f32;
let _13: u32;
let _14: *const Adt20;
let _15: [bool; 3];
let _16: f64;
let _17: char;
let _18: [i64; 5];
let _19: ([u64; 1], ((char, *const i128, i32),));
let _20: (&'static i8, *const *mut f32, usize, Adt45);
let _21: ([i64; 7],);
let _22: bool;
let _23: [i16; 3];
let _24: *mut *mut f32;
let _25: f64;
let _26: i16;
let _27: *const &'static &'static isize;
let _28: ((char, *const i128, i32),);
let _29: [u32; 1];
let _30: bool;
let _31: [i32; 8];
let _32: &'static f64;
let _33: (*mut f32, u16, Adt22, i64);
let _34: i8;
let _35: Adt31;
let _36: u16;
let _37: bool;
let _38: ();
let _39: ();
{
RET = [7555755998037647321_usize,16466782779274056284_usize,1854044791513704861_usize,5_usize,7_usize];
RET = [6976718117730779124_usize,2_usize,4408441194087812522_usize,14646419644692199764_usize,3_usize];
RET = [5385165644145395320_usize,16965840196196236356_usize,6_usize,16330525310120441469_usize,12976336594640301958_usize];
_1 = '\u{2e7b}';
RET = [0_usize,13369135985625511702_usize,4_usize,14797027588331704955_usize,14663870474707952285_usize];
RET = [4_usize,4_usize,3507238364318660535_usize,16080052969066543293_usize,5_usize];
RET = [1_usize,0_usize,8544835920542944150_usize,1_usize,5_usize];
RET = [7522699190473259705_usize,5_usize,3_usize,2_usize,9954975704202506035_usize];
RET = [1_usize,5123992688295729631_usize,4_usize,14014190301223962630_usize,18008073414344897865_usize];
_1 = '\u{9fa3a}';
_1 = '\u{57474}';
_1 = '\u{e53db}';
RET = [9973895505228797124_usize,7_usize,0_usize,7_usize,17932959897606154522_usize];
RET = [6_usize,8862346895831854796_usize,1166115610379209138_usize,7_usize,13381174026671824471_usize];
Goto(bb1)
}
bb1 = {
RET = [7_usize,1_usize,3178903691518536804_usize,11387622358587117228_usize,6_usize];
_1 = '\u{717e4}';
_1 = '\u{3ce95}';
RET = [5_usize,14527275626923200638_usize,7_usize,12579168619815327324_usize,0_usize];
_1 = '\u{cd5dc}';
RET = [15687415040143083740_usize,4673361275198822941_usize,7_usize,2_usize,3188510999884155210_usize];
RET = [18165083752599132264_usize,18042635611499343654_usize,6_usize,6_usize,16740555115322438928_usize];
RET = [3626402605505630804_usize,3271777805614895063_usize,4_usize,4438488658704650277_usize,17587434529295240214_usize];
RET = [2836153649180816444_usize,3_usize,14108287567430988952_usize,10164998156119462412_usize,2_usize];
RET = [11989995603930376810_usize,13568996078177296728_usize,8020580856805945001_usize,3618763611852381221_usize,16717544918283225211_usize];
RET = [7_usize,2_usize,6268584286259233395_usize,16488851865344909722_usize,0_usize];
RET = [13095244444650089654_usize,2_usize,8914503157920386019_usize,0_usize,5188958357716381366_usize];
RET = [4_usize,3_usize,1_usize,7_usize,8941389735693148080_usize];
_1 = '\u{c0969}';
_1 = '\u{89c50}';
RET = [8593596440333187423_usize,5362288316239884667_usize,5_usize,14023319450850644958_usize,1997927845090808549_usize];
_1 = '\u{3b359}';
RET = [9457513750353562931_usize,7_usize,1328979536691763178_usize,2_usize,0_usize];
RET = [1_usize,7_usize,13749102543168654186_usize,8096387485287084389_usize,0_usize];
RET = [8703377178059617627_usize,13300388048858255851_usize,7_usize,15049160111238664984_usize,0_usize];
Goto(bb2)
}
bb2 = {
RET = [2_usize,2874527239479579061_usize,5378790614315825844_usize,18216349805525060763_usize,6_usize];
_4 = (201_u8, 9478557777671834706_usize);
_4.0 = _1 as u8;
_3 = 5400912261392746566_i64 as u128;
Call(_3 = core::intrinsics::bswap(50594491039473716864793377216232539118_u128), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = '\u{10bc2b}';
_6 = 94_isize ^ 9223372036854775807_isize;
_6 = !9223372036854775807_isize;
_4.0 = 33_u8 | 229_u8;
_4.0 = 161_u8 - 88_u8;
_8.0 = (-6435263849533832189372467244886628017_i128) as f32;
_3 = 5009_i16 as u128;
_4.1 = 1783363970_u32 as usize;
_8.0 = (-138060912974534206571986084421934709819_i128) as f32;
_1 = '\u{5c681}';
_4.0 = 77_u8;
_10 = false;
_1 = '\u{c916e}';
_10 = false;
_10 = _4.1 > _4.1;
_4.0 = !167_u8;
_4.1 = (-6935999670229393819_i64) as usize;
RET = [_4.1,_4.1,_4.1,_4.1,_4.1];
_10 = !true;
_11 = _10 | _10;
_1 = '\u{f49da}';
_3 = 160781066666909865261920816032248856487_u128;
_11 = _10;
match _3 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
160781066666909865261920816032248856487 => bb7,
_ => bb6
}
}
bb4 = {
RET = [2_usize,2874527239479579061_usize,5378790614315825844_usize,18216349805525060763_usize,6_usize];
_4 = (201_u8, 9478557777671834706_usize);
_4.0 = _1 as u8;
_3 = 5400912261392746566_i64 as u128;
Call(_3 = core::intrinsics::bswap(50594491039473716864793377216232539118_u128), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
RET = [7_usize,1_usize,3178903691518536804_usize,11387622358587117228_usize,6_usize];
_1 = '\u{717e4}';
_1 = '\u{3ce95}';
RET = [5_usize,14527275626923200638_usize,7_usize,12579168619815327324_usize,0_usize];
_1 = '\u{cd5dc}';
RET = [15687415040143083740_usize,4673361275198822941_usize,7_usize,2_usize,3188510999884155210_usize];
RET = [18165083752599132264_usize,18042635611499343654_usize,6_usize,6_usize,16740555115322438928_usize];
RET = [3626402605505630804_usize,3271777805614895063_usize,4_usize,4438488658704650277_usize,17587434529295240214_usize];
RET = [2836153649180816444_usize,3_usize,14108287567430988952_usize,10164998156119462412_usize,2_usize];
RET = [11989995603930376810_usize,13568996078177296728_usize,8020580856805945001_usize,3618763611852381221_usize,16717544918283225211_usize];
RET = [7_usize,2_usize,6268584286259233395_usize,16488851865344909722_usize,0_usize];
RET = [13095244444650089654_usize,2_usize,8914503157920386019_usize,0_usize,5188958357716381366_usize];
RET = [4_usize,3_usize,1_usize,7_usize,8941389735693148080_usize];
_1 = '\u{c0969}';
_1 = '\u{89c50}';
RET = [8593596440333187423_usize,5362288316239884667_usize,5_usize,14023319450850644958_usize,1997927845090808549_usize];
_1 = '\u{3b359}';
RET = [9457513750353562931_usize,7_usize,1328979536691763178_usize,2_usize,0_usize];
RET = [1_usize,7_usize,13749102543168654186_usize,8096387485287084389_usize,0_usize];
RET = [8703377178059617627_usize,13300388048858255851_usize,7_usize,15049160111238664984_usize,0_usize];
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_6 = (-9223372036854775808_isize) ^ (-9223372036854775808_isize);
_4 = (34_u8, 4_usize);
_1 = '\u{10821a}';
RET = [_4.1,_4.1,_4.1,_4.1,_4.1];
_11 = !_10;
_4.1 = 107678018142016415_usize;
_8.0 = _4.0 as f32;
_9 = 23814_u16;
_4.0 = 134_u8;
_10 = !_11;
_1 = '\u{70b0c}';
_10 = _11;
_10 = _4.1 > _4.1;
_4 = (118_u8, 0_usize);
_13 = _3 as u32;
_12 = 13357334280365398860_u64 as f32;
RET = [_4.1,_4.1,_4.1,_4.1,_4.1];
_4 = (53_u8, 7_usize);
_12 = -_8.0;
_1 = '\u{c11b7}';
_9 = 8150_u16 ^ 50783_u16;
_1 = '\u{42002}';
_12 = _8.0;
_9 = _11 as u16;
_8.0 = _12 - _12;
_13 = 3732379794_u32;
Call(_4.0 = core::intrinsics::transmute(_11), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_13 = !3762375907_u32;
_8 = (_12,);
_6 = _4.0 as isize;
_13 = 972054942_u32 >> _9;
_13 = 15389412501180613005_u64 as u32;
_9 = !53068_u16;
_10 = _3 >= _3;
RET = [_4.1,_4.1,_4.1,_4.1,_4.1];
_16 = _13 as f64;
_3 = !17349102920085896074812234771990152835_u128;
_15 = [_10,_10,_10];
_13 = !2844521582_u32;
RET = [_4.1,_4.1,_4.1,_4.1,_4.1];
_12 = -_8.0;
_9 = 53437_u16;
RET = [_4.1,_4.1,_4.1,_4.1,_4.1];
_13 = 1116444630747472211_i64 as u32;
_10 = !_11;
_4 = (138_u8, 1_usize);
_4.1 = 2_usize & 11220581070235603156_usize;
_19.1.0.0 = _1;
_19.0 = [16036498301727135547_u64];
_18 = [(-3442805794636085268_i64),(-259448914621432165_i64),(-557888095963562880_i64),4948472527510582617_i64,3755226486957096262_i64];
_12 = _8.0;
_8.0 = _12;
Goto(bb9)
}
bb9 = {
_4.1 = 4_usize;
_10 = _6 < _6;
_9 = !35452_u16;
_19.1.0.2 = _9 as i32;
_13 = !972131609_u32;
_1 = _19.1.0.0;
_21.0 = [2422786332333801370_i64,8953165331133186646_i64,(-7448322492675656461_i64),(-1073525531838291515_i64),6265229917393500056_i64,7976478567726431730_i64,832884145051169979_i64];
_19.0 = [16012511077362375016_u64];
_12 = _8.0 - _8.0;
_20.2 = _4.1 >> _19.1.0.2;
_4 = (17_u8, _20.2);
RET = [_4.1,_20.2,_4.1,_20.2,_4.1];
_20.2 = _3 as usize;
_1 = _19.1.0.0;
_12 = _8.0;
_4 = (89_u8, _20.2);
_13 = 810527510_u32 ^ 1851845448_u32;
_10 = _11 | _11;
_23 = [15803_i16,(-29291_i16),4132_i16];
_4.1 = (-30248_i16) as usize;
match _4.0 {
0 => bb8,
89 => bb10,
_ => bb4
}
}
bb10 = {
_19.1.0.2 = !999160480_i32;
_1 = _19.1.0.0;
_1 = _19.1.0.0;
_13 = !1587674892_u32;
_2 = &_25;
_25 = _16 - _16;
_11 = !_10;
_4 = (113_u8, _20.2);
_16 = _25;
_19.0 = [17535108326437971707_u64];
_18 = [4769311255804903247_i64,(-7857227510310987664_i64),5706036564667491600_i64,(-9077285461961430154_i64),(-6943562262810345263_i64)];
_8.0 = _12;
_26 = -(-3014_i16);
_17 = _1;
_16 = -_25;
_2 = &_25;
_18 = [(-567022373577790748_i64),3480628455699259871_i64,(-4826743611265567371_i64),(-5960761894479322492_i64),(-2864019257712815061_i64)];
_23 = [_26,_26,_26];
_8.0 = _12 + _12;
_18 = [(-5899446662130427164_i64),8980477604193823109_i64,3436199456943361166_i64,(-6123873986052446736_i64),(-7674035573592281507_i64)];
_2 = &(*_2);
_8.0 = _19.1.0.2 as f32;
_22 = !_11;
_18 = [3893542571010035993_i64,7461344194380710087_i64,(-4458138828627164001_i64),(-509603740221223599_i64),(-5571227094598711389_i64)];
_19.1.0.0 = _1;
Goto(bb11)
}
bb11 = {
_26 = (-28936_i16) * (-30302_i16);
_26 = -14620_i16;
_8 = (_12,);
_28.0.0 = _17;
_25 = _16 - _16;
_24 = core::ptr::addr_of_mut!(_33.0);
_23 = [_26,_26,_26];
_27 = core::ptr::addr_of!(_5);
_31 = [_19.1.0.2,_19.1.0.2,_19.1.0.2,_19.1.0.2,_19.1.0.2,_19.1.0.2,_19.1.0.2,_19.1.0.2];
_21.0 = [3181771939022113524_i64,4155103473379643211_i64,(-7901649736976065485_i64),(-8182217101623785674_i64),7529869490682008256_i64,2723300764627443334_i64,(-390681291426068898_i64)];
_12 = _8.0 + _8.0;
_19.1.0.2 = -1146720321_i32;
_33.0 = core::ptr::addr_of_mut!(_12);
_34 = !(-14_i8);
_19.0 = [18399390691658402645_u64];
_22 = _10;
_10 = _11;
_33.3 = 6195099815727161058_i64 | 4177053258333368313_i64;
_32 = &_25;
_30 = !_22;
_26 = _13 as i16;
_32 = &_25;
_36 = _19.1.0.2 as u16;
_19.0 = [5818321833494117547_u64];
match _4.0 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
113 => bb17,
_ => bb16
}
}
bb12 = {
RET = [2_usize,2874527239479579061_usize,5378790614315825844_usize,18216349805525060763_usize,6_usize];
_4 = (201_u8, 9478557777671834706_usize);
_4.0 = _1 as u8;
_3 = 5400912261392746566_i64 as u128;
Call(_3 = core::intrinsics::bswap(50594491039473716864793377216232539118_u128), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_4.1 = 4_usize;
_10 = _6 < _6;
_9 = !35452_u16;
_19.1.0.2 = _9 as i32;
_13 = !972131609_u32;
_1 = _19.1.0.0;
_21.0 = [2422786332333801370_i64,8953165331133186646_i64,(-7448322492675656461_i64),(-1073525531838291515_i64),6265229917393500056_i64,7976478567726431730_i64,832884145051169979_i64];
_19.0 = [16012511077362375016_u64];
_12 = _8.0 - _8.0;
_20.2 = _4.1 >> _19.1.0.2;
_4 = (17_u8, _20.2);
RET = [_4.1,_20.2,_4.1,_20.2,_4.1];
_20.2 = _3 as usize;
_1 = _19.1.0.0;
_12 = _8.0;
_4 = (89_u8, _20.2);
_13 = 810527510_u32 ^ 1851845448_u32;
_10 = _11 | _11;
_23 = [15803_i16,(-29291_i16),4132_i16];
_4.1 = (-30248_i16) as usize;
match _4.0 {
0 => bb8,
89 => bb10,
_ => bb4
}
}
bb14 = {
RET = [7_usize,1_usize,3178903691518536804_usize,11387622358587117228_usize,6_usize];
_1 = '\u{717e4}';
_1 = '\u{3ce95}';
RET = [5_usize,14527275626923200638_usize,7_usize,12579168619815327324_usize,0_usize];
_1 = '\u{cd5dc}';
RET = [15687415040143083740_usize,4673361275198822941_usize,7_usize,2_usize,3188510999884155210_usize];
RET = [18165083752599132264_usize,18042635611499343654_usize,6_usize,6_usize,16740555115322438928_usize];
RET = [3626402605505630804_usize,3271777805614895063_usize,4_usize,4438488658704650277_usize,17587434529295240214_usize];
RET = [2836153649180816444_usize,3_usize,14108287567430988952_usize,10164998156119462412_usize,2_usize];
RET = [11989995603930376810_usize,13568996078177296728_usize,8020580856805945001_usize,3618763611852381221_usize,16717544918283225211_usize];
RET = [7_usize,2_usize,6268584286259233395_usize,16488851865344909722_usize,0_usize];
RET = [13095244444650089654_usize,2_usize,8914503157920386019_usize,0_usize,5188958357716381366_usize];
RET = [4_usize,3_usize,1_usize,7_usize,8941389735693148080_usize];
_1 = '\u{c0969}';
_1 = '\u{89c50}';
RET = [8593596440333187423_usize,5362288316239884667_usize,5_usize,14023319450850644958_usize,1997927845090808549_usize];
_1 = '\u{3b359}';
RET = [9457513750353562931_usize,7_usize,1328979536691763178_usize,2_usize,0_usize];
RET = [1_usize,7_usize,13749102543168654186_usize,8096387485287084389_usize,0_usize];
RET = [8703377178059617627_usize,13300388048858255851_usize,7_usize,15049160111238664984_usize,0_usize];
Goto(bb2)
}
bb15 = {
RET = [7_usize,1_usize,3178903691518536804_usize,11387622358587117228_usize,6_usize];
_1 = '\u{717e4}';
_1 = '\u{3ce95}';
RET = [5_usize,14527275626923200638_usize,7_usize,12579168619815327324_usize,0_usize];
_1 = '\u{cd5dc}';
RET = [15687415040143083740_usize,4673361275198822941_usize,7_usize,2_usize,3188510999884155210_usize];
RET = [18165083752599132264_usize,18042635611499343654_usize,6_usize,6_usize,16740555115322438928_usize];
RET = [3626402605505630804_usize,3271777805614895063_usize,4_usize,4438488658704650277_usize,17587434529295240214_usize];
RET = [2836153649180816444_usize,3_usize,14108287567430988952_usize,10164998156119462412_usize,2_usize];
RET = [11989995603930376810_usize,13568996078177296728_usize,8020580856805945001_usize,3618763611852381221_usize,16717544918283225211_usize];
RET = [7_usize,2_usize,6268584286259233395_usize,16488851865344909722_usize,0_usize];
RET = [13095244444650089654_usize,2_usize,8914503157920386019_usize,0_usize,5188958357716381366_usize];
RET = [4_usize,3_usize,1_usize,7_usize,8941389735693148080_usize];
_1 = '\u{c0969}';
_1 = '\u{89c50}';
RET = [8593596440333187423_usize,5362288316239884667_usize,5_usize,14023319450850644958_usize,1997927845090808549_usize];
_1 = '\u{3b359}';
RET = [9457513750353562931_usize,7_usize,1328979536691763178_usize,2_usize,0_usize];
RET = [1_usize,7_usize,13749102543168654186_usize,8096387485287084389_usize,0_usize];
RET = [8703377178059617627_usize,13300388048858255851_usize,7_usize,15049160111238664984_usize,0_usize];
Goto(bb2)
}
bb16 = {
Return()
}
bb17 = {
_28.0.0 = _1;
_24 = core::ptr::addr_of_mut!((*_24));
(*_24) = core::ptr::addr_of_mut!(_12);
_34 = (*_32) as i8;
_17 = _28.0.0;
_21.0 = [_33.3,_33.3,_33.3,_33.3,_33.3,_33.3,_33.3];
_37 = _22 ^ _10;
Goto(bb18)
}
bb18 = {
Call(_38 = dump_var(17_usize, 15_usize, Move(_15), 9_usize, Move(_9), 10_usize, Move(_10), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_38 = dump_var(17_usize, 23_usize, Move(_23), 37_usize, Move(_37), 18_usize, Move(_18), 26_usize, Move(_26)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_38 = dump_var(17_usize, 31_usize, Move(_31), 34_usize, Move(_34), 39_usize, _39, 39_usize, _39), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: [i8; 5]) -> [usize; 5] {
mir! {
type RET = [usize; 5];
let _2: i64;
let _3: i32;
let _4: char;
let _5: [i16; 7];
let _6: *mut Adt21;
let _7: [u128; 4];
let _8: (Adt53, Adt50, *const *mut f32, &'static i16);
let _9: u128;
let _10: *mut f32;
let _11: isize;
let _12: bool;
let _13: f64;
let _14: isize;
let _15: *const i16;
let _16: [bool; 3];
let _17: ([u64; 1], ((char, *const i128, i32),));
let _18: u16;
let _19: i32;
let _20: i128;
let _21: (u8, usize);
let _22: *const [i16; 3];
let _23: isize;
let _24: f32;
let _25: isize;
let _26: u64;
let _27: [u16; 3];
let _28: char;
let _29: *mut &'static i8;
let _30: u8;
let _31: i128;
let _32: u64;
let _33: isize;
let _34: &'static (Adt22, i64, isize);
let _35: [usize; 5];
let _36: Adt20;
let _37: (Adt53, Adt50, *const *mut f32, &'static i16);
let _38: [bool; 3];
let _39: *const Adt20;
let _40: ();
let _41: ();
{
RET = [6057163791322226591_usize,16020058961675156881_usize,1_usize,14875278137538242423_usize,3336778857817009310_usize];
RET = [8331383284558616653_usize,5_usize,2_usize,7_usize,6_usize];
RET = [6_usize,11776949298833193653_usize,8922254654824584573_usize,3_usize,14795207792288176745_usize];
_1 = [(-115_i8),(-28_i8),95_i8,88_i8,(-24_i8)];
RET = [4_usize,3609797263911537142_usize,1163396194255310965_usize,6_usize,637524145345101287_usize];
RET = [1_usize,11170607186911396368_usize,3_usize,16399879781090472444_usize,10772816183584681203_usize];
_1 = [(-58_i8),(-127_i8),(-69_i8),(-17_i8),(-54_i8)];
RET = [8148457155602962310_usize,2_usize,2_usize,7_usize,6_usize];
RET = [2_usize,1_usize,2113975296026789424_usize,7541747419365039160_usize,3280873170867743377_usize];
_1 = [(-113_i8),(-6_i8),(-106_i8),64_i8,(-6_i8)];
RET = [5_usize,3248573599709382828_usize,3745828490258035476_usize,4_usize,5268709195157966725_usize];
_1 = [37_i8,(-50_i8),(-51_i8),102_i8,(-38_i8)];
RET = [0_usize,7_usize,8508649117233029237_usize,2100713124465075018_usize,5926608825772950122_usize];
RET = [6_usize,1888910500803889443_usize,463427215480754503_usize,6_usize,7318682844661251134_usize];
RET = [4_usize,2358386390992497866_usize,2195479015897691908_usize,18349033968862253464_usize,5770794199325927128_usize];
RET = [1_usize,4_usize,10107354618886784395_usize,15371581613340711895_usize,5_usize];
RET = [3_usize,0_usize,1_usize,2_usize,7_usize];
RET = [4296766288526522629_usize,15596952434671571386_usize,14572694019400151875_usize,4_usize,7_usize];
RET = [2_usize,14426973366659089748_usize,4_usize,3450440712631898297_usize,6_usize];
RET = [0_usize,5_usize,6636890017796225174_usize,4_usize,4_usize];
Call(RET = fn19(_1, _1, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [16435693862033462185_usize,7_usize,4_usize,17015038315557659169_usize,1389551037946588140_usize];
RET = [5282025136621399800_usize,6888102138299405180_usize,4_usize,10412518826037456045_usize,16840489923114499128_usize];
_2 = 8169215024584198044_i64;
Goto(bb2)
}
bb2 = {
_1 = [(-83_i8),76_i8,(-71_i8),32_i8,(-80_i8)];
_4 = '\u{ee2a2}';
_1 = [122_i8,111_i8,(-25_i8),56_i8,(-67_i8)];
RET = [3_usize,8783390215683340810_usize,9172859772100623028_usize,1_usize,5651319850738403767_usize];
_1 = [125_i8,(-69_i8),66_i8,(-81_i8),(-88_i8)];
_3 = (-781796131_i32);
_3 = 15228498478723845809_u64 as i32;
RET = [13825223564188200868_usize,9223506739746511986_usize,3810394765622923396_usize,0_usize,17225654393876177173_usize];
_1 = [(-31_i8),100_i8,(-40_i8),112_i8,(-77_i8)];
match _2 {
0 => bb1,
1 => bb3,
2 => bb4,
8169215024584198044 => bb6,
_ => bb5
}
}
bb3 = {
RET = [16435693862033462185_usize,7_usize,4_usize,17015038315557659169_usize,1389551037946588140_usize];
RET = [5282025136621399800_usize,6888102138299405180_usize,4_usize,10412518826037456045_usize,16840489923114499128_usize];
_2 = 8169215024584198044_i64;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_2 = _3 as i64;
RET = [3762347804431066159_usize,4759946948721691203_usize,0_usize,1_usize,15418348303611214950_usize];
_2 = (-1732152908515452471_i64);
_5 = [4919_i16,1445_i16,(-30728_i16),(-29926_i16),(-13987_i16),5026_i16,3073_i16];
_7 = [29993368227796179386413302027370080323_u128,293384874240410752132439964255610930633_u128,121962264480597790534810009442601420533_u128,179142979953580844849186544025408590787_u128];
RET = [6_usize,8867676550400697097_usize,16097696455351371355_usize,11355847592560833744_usize,9776467350887006667_usize];
_2 = -1465881334696138367_i64;
_1 = [(-110_i8),(-115_i8),66_i8,56_i8,(-101_i8)];
_2 = _3 as i64;
_4 = '\u{6dbd7}';
_5 = [4318_i16,27794_i16,15799_i16,(-6047_i16),11334_i16,(-12502_i16),(-19049_i16)];
_3 = !299112560_i32;
_3 = (-345350830_i32);
RET = [708568476033716219_usize,9313016158902156000_usize,17214630988137189631_usize,10799270166951479585_usize,7214461611783867810_usize];
RET = [8110809865253857699_usize,15463349304674585214_usize,1_usize,5_usize,5_usize];
_2 = 49288_u16 as i64;
Goto(bb7)
}
bb7 = {
_4 = '\u{a4e25}';
_7 = [311777933200204113026677987604768050889_u128,144233805906645070527615565155947494438_u128,192137579182824584969839896278490948346_u128,17886212526927713917925866870461781663_u128];
_2 = 4765324942547802871_i64 ^ (-2199326787613332451_i64);
_5 = [18917_i16,23153_i16,(-24984_i16),(-16273_i16),(-30426_i16),(-237_i16),(-29249_i16)];
_5 = [1644_i16,7226_i16,(-6785_i16),(-16806_i16),1030_i16,13682_i16,29204_i16];
_8.2 = core::ptr::addr_of!(_10);
RET = [9894303526277249962_usize,5_usize,4_usize,13537338951672785080_usize,4_usize];
_4 = '\u{e228a}';
_3 = !1971458974_i32;
_7 = [155073151641414155297323546365718420276_u128,190308664636137895288168941073064057252_u128,12185365748224934525104751935327277601_u128,137242660345018858926097213070500764463_u128];
_9 = false as u128;
RET = [4_usize,7845948401790767603_usize,9878778076474298644_usize,6_usize,7026514709140592903_usize];
_7 = [_9,_9,_9,_9];
_7 = [_9,_9,_9,_9];
_8.2 = core::ptr::addr_of!(_10);
_8.2 = core::ptr::addr_of!(_10);
_11 = 12692165814833827351_u64 as isize;
_7 = [_9,_9,_9,_9];
_4 = '\u{97b24}';
Goto(bb8)
}
bb8 = {
_1 = [(-123_i8),(-82_i8),120_i8,(-88_i8),72_i8];
_3 = (-534549722_i32) * (-1270445021_i32);
_12 = _4 != _4;
_12 = true;
_3 = (-545794938_i32) ^ (-1060070826_i32);
RET = [9632472602470348101_usize,1_usize,7098893677220307182_usize,1_usize,13823001593802766521_usize];
RET = [1_usize,2_usize,7074311663556683036_usize,5732098309754096341_usize,0_usize];
_5 = [5137_i16,(-29084_i16),(-12224_i16),(-70_i16),(-7539_i16),(-1581_i16),(-24831_i16)];
_12 = true & false;
_2 = (-6153_i16) as i64;
_11 = 15595777443644257518_u64 as isize;
_7 = [_9,_9,_9,_9];
_5 = [(-11492_i16),(-18780_i16),(-18533_i16),24212_i16,14146_i16,(-9361_i16),(-13850_i16)];
_14 = -_11;
_11 = _14 + _14;
RET = [15957965771674868316_usize,18425990641957804423_usize,8856143371395266754_usize,0_usize,1414647042810258879_usize];
_2 = _11 as i64;
_8.2 = core::ptr::addr_of!(_10);
_12 = false;
_12 = false;
Call(_11 = core::intrinsics::bswap(_14), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_7 = [_9,_9,_9,_9];
_12 = true;
_7 = [_9,_9,_9,_9];
_4 = '\u{9133e}';
Goto(bb10)
}
bb10 = {
_12 = false;
RET = [16765191868150347166_usize,9496292345507880871_usize,3_usize,13956971432143014970_usize,9244588678322457262_usize];
_3 = _4 as i32;
_16 = [_12,_12,_12];
_4 = '\u{c2cd1}';
_5 = [19915_i16,(-16518_i16),(-12690_i16),19961_i16,22485_i16,29133_i16,3635_i16];
_9 = _2 as u128;
_9 = 3620081756133972997874406064357450255_u128;
_17.1.0.0 = _4;
_16 = [_12,_12,_12];
_17.1.0.0 = _4;
_21.1 = 8913892805546064417_usize ^ 16185751732952829907_usize;
_9 = 31663934299783072934312103783606083448_u128;
_17.1.0.0 = _4;
_23 = _11;
_14 = !_23;
Goto(bb11)
}
bb11 = {
_17.1.0.2 = _3;
_19 = -_3;
_23 = _11;
_18 = !65285_u16;
_23 = -_11;
_19 = _17.1.0.2;
_20 = _2 as i128;
_23 = -_14;
_2 = 6314628597216210140_i64 + 5489853838392307361_i64;
_13 = _21.1 as f64;
_21.0 = !155_u8;
_20 = 156635957880663238843912168508132092548_i128;
_1 = [99_i8,(-124_i8),(-89_i8),83_i8,(-9_i8)];
RET = [_21.1,_21.1,_21.1,_21.1,_21.1];
_9 = !324003334093282729628467409206071669132_u128;
_8.2 = core::ptr::addr_of!(_10);
_12 = _14 != _11;
_18 = 35920_u16;
_14 = _23 >> _11;
_12 = true;
_17.0 = [2762621386231398139_u64];
_25 = _18 as isize;
_24 = _9 as f32;
Goto(bb12)
}
bb12 = {
_17.1.0.0 = _4;
_26 = 11838453333165639742_u64 * 334643304281562152_u64;
_18 = _14 as u16;
_21 = (245_u8, 0_usize);
_17.1.0.1 = core::ptr::addr_of!(_31);
_27 = [_18,_18,_18];
_10 = core::ptr::addr_of_mut!(_24);
_28 = _17.1.0.0;
_33 = _14;
_28 = _17.1.0.0;
_2 = !(-4657769052130482046_i64);
_12 = !false;
_21 = (46_u8, 4_usize);
_8.2 = core::ptr::addr_of!(_10);
_21.0 = 39_u8 * 144_u8;
_18 = _12 as u16;
_31 = _20 ^ _20;
Goto(bb13)
}
bb13 = {
_1 = [(-121_i8),(-103_i8),72_i8,(-80_i8),(-17_i8)];
RET = [_21.1,_21.1,_21.1,_21.1,_21.1];
_17.1.0.1 = core::ptr::addr_of!(_20);
_8.2 = core::ptr::addr_of!(_10);
_27 = [_18,_18,_18];
_12 = !true;
(*_10) = _21.0 as f32;
RET = [_21.1,_21.1,_21.1,_21.1,_21.1];
_12 = true;
_39 = core::ptr::addr_of!(_36);
Goto(bb14)
}
bb14 = {
_35 = [_21.1,_21.1,_21.1,_21.1,_21.1];
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(18_usize, 21_usize, Move(_21), 33_usize, Move(_33), 16_usize, Move(_16), 26_usize, Move(_26)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(18_usize, 12_usize, Move(_12), 31_usize, Move(_31), 9_usize, Move(_9), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(18_usize, 3_usize, Move(_3), 11_usize, Move(_11), 1_usize, Move(_1), 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: [i8; 5],mut _2: [i8; 5],mut _3: [i8; 5]) -> [usize; 5] {
mir! {
type RET = [usize; 5];
let _4: bool;
let _5: char;
let _6: [i8; 8];
let _7: u16;
let _8: isize;
let _9: u64;
let _10: u64;
let _11: Adt31;
let _12: [bool; 3];
let _13: f64;
let _14: f64;
let _15: usize;
let _16: (f32,);
let _17: [bool; 3];
let _18: [usize; 5];
let _19: usize;
let _20: f64;
let _21: Adt50;
let _22: &'static ([usize; 5], *const i16, (char, *const i128, i32), [u16; 3]);
let _23: *const [i16; 3];
let _24: &'static f64;
let _25: char;
let _26: &'static ([usize; 5], *const i16, (char, *const i128, i32), [u16; 3]);
let _27: *mut *mut u32;
let _28: isize;
let _29: f32;
let _30: *const i16;
let _31: ();
let _32: ();
{
_3 = [72_i8,87_i8,(-52_i8),(-98_i8),57_i8];
RET = [8358176360050638991_usize,5_usize,12410074599839261021_usize,2751802334301568242_usize,3_usize];
_3 = _1;
_4 = !true;
_5 = '\u{f29c}';
RET = [11667013442375865283_usize,11405550668878725313_usize,13362424562067730887_usize,5_usize,3618478055197914744_usize];
_1 = [12_i8,(-128_i8),(-87_i8),35_i8,(-44_i8)];
Goto(bb1)
}
bb1 = {
_1 = _2;
_3 = _1;
_5 = '\u{7e700}';
RET = [6_usize,10655046986461341325_usize,18247645947189835286_usize,14515284834825244640_usize,2_usize];
RET = [5_usize,0_usize,3264728406369005973_usize,5_usize,5_usize];
_2 = [76_i8,(-79_i8),65_i8,60_i8,(-104_i8)];
_4 = false;
RET = [4_usize,7_usize,5189758388931894984_usize,3_usize,825290458161693952_usize];
RET = [2_usize,3_usize,8329488220827545748_usize,2_usize,6025444865312601399_usize];
_5 = '\u{5118}';
RET = [2_usize,7_usize,4661130108558204973_usize,3_usize,0_usize];
_5 = '\u{5e68c}';
_3 = [85_i8,108_i8,106_i8,(-98_i8),122_i8];
_3 = _2;
_3 = [(-72_i8),(-11_i8),(-32_i8),2_i8,114_i8];
_3 = [114_i8,45_i8,122_i8,(-81_i8),111_i8];
_3 = [15_i8,(-87_i8),(-14_i8),72_i8,2_i8];
_4 = _5 <= _5;
_2 = [(-44_i8),69_i8,(-77_i8),(-31_i8),26_i8];
_2 = [(-84_i8),(-115_i8),33_i8,(-105_i8),(-22_i8)];
_6 = [(-9_i8),113_i8,(-20_i8),57_i8,(-63_i8),27_i8,0_i8,(-88_i8)];
_5 = '\u{e654e}';
_2 = [103_i8,126_i8,48_i8,(-112_i8),51_i8];
_5 = '\u{9b025}';
Goto(bb2)
}
bb2 = {
RET = [6_usize,1_usize,1_usize,11065564217549896971_usize,4_usize];
_7 = 168_u16;
_5 = '\u{6dac5}';
_1 = [(-33_i8),86_i8,(-52_i8),43_i8,108_i8];
_7 = 871675647_u32 as u16;
_4 = true;
_9 = 14165275098926058802_u64;
_7 = (-14392_i16) as u16;
_10 = 4276926558494340048_usize as u64;
RET = [10081603820602842927_usize,14778147905843736153_usize,1_usize,3_usize,4983887322291012386_usize];
_2 = [62_i8,126_i8,123_i8,13_i8,(-105_i8)];
_9 = !_10;
_3 = [(-5_i8),(-92_i8),73_i8,(-78_i8),(-83_i8)];
Call(_3 = core::intrinsics::transmute(_1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10 = _4 as u64;
_8 = 119_isize;
_8 = _9 as isize;
RET = [17202226433394768045_usize,1_usize,5572905714315455793_usize,0_usize,0_usize];
_10 = _9 & _9;
_3 = [(-120_i8),(-124_i8),(-49_i8),(-91_i8),(-77_i8)];
_1 = [109_i8,(-113_i8),(-9_i8),42_i8,(-59_i8)];
_2 = [(-25_i8),(-72_i8),80_i8,44_i8,44_i8];
_6 = [69_i8,(-91_i8),36_i8,(-10_i8),(-27_i8),80_i8,(-116_i8),(-106_i8)];
_12 = [_4,_4,_4];
_12 = [_4,_4,_4];
_12 = [_4,_4,_4];
_5 = '\u{3e5d0}';
_6 = [(-56_i8),(-43_i8),(-94_i8),40_i8,(-15_i8),(-114_i8),(-29_i8),(-29_i8)];
_13 = 231_u8 as f64;
_10 = !_9;
_1 = [112_i8,(-123_i8),(-26_i8),(-102_i8),(-123_i8)];
_2 = [127_i8,(-101_i8),35_i8,(-31_i8),60_i8];
_7 = 18201_u16 + 52667_u16;
_2 = [44_i8,(-122_i8),(-71_i8),(-37_i8),(-25_i8)];
_9 = _10 * _10;
RET = [5664935750474569449_usize,4777432902802326449_usize,4_usize,5_usize,12627400072043484277_usize];
_4 = true;
_2 = [83_i8,(-41_i8),19_i8,52_i8,(-54_i8)];
Goto(bb4)
}
bb4 = {
_8 = 121423616473630563915682538844615354097_i128 as isize;
_5 = '\u{981b7}';
_15 = 16011909242433598626_usize;
_14 = _13;
_12 = [_4,_4,_4];
Goto(bb5)
}
bb5 = {
_13 = _8 as f64;
_16.0 = 217_u8 as f32;
RET = [_15,_15,_15,_15,_15];
_2 = [33_i8,(-102_i8),43_i8,(-65_i8),127_i8];
RET = [_15,_15,_15,_15,_15];
_8 = 9223372036854775807_isize << _15;
_17 = [_4,_4,_4];
_8 = 9223372036854775807_isize;
_4 = !false;
_18 = [_15,_15,_15,_15,_15];
_15 = 7990696989630704013_usize & 6_usize;
RET = [_15,_15,_15,_15,_15];
match _8 {
0 => bb1,
1 => bb4,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
9223372036854775807 => bb12,
_ => bb11
}
}
bb6 = {
_8 = 121423616473630563915682538844615354097_i128 as isize;
_5 = '\u{981b7}';
_15 = 16011909242433598626_usize;
_14 = _13;
_12 = [_4,_4,_4];
Goto(bb5)
}
bb7 = {
_10 = _4 as u64;
_8 = 119_isize;
_8 = _9 as isize;
RET = [17202226433394768045_usize,1_usize,5572905714315455793_usize,0_usize,0_usize];
_10 = _9 & _9;
_3 = [(-120_i8),(-124_i8),(-49_i8),(-91_i8),(-77_i8)];
_1 = [109_i8,(-113_i8),(-9_i8),42_i8,(-59_i8)];
_2 = [(-25_i8),(-72_i8),80_i8,44_i8,44_i8];
_6 = [69_i8,(-91_i8),36_i8,(-10_i8),(-27_i8),80_i8,(-116_i8),(-106_i8)];
_12 = [_4,_4,_4];
_12 = [_4,_4,_4];
_12 = [_4,_4,_4];
_5 = '\u{3e5d0}';
_6 = [(-56_i8),(-43_i8),(-94_i8),40_i8,(-15_i8),(-114_i8),(-29_i8),(-29_i8)];
_13 = 231_u8 as f64;
_10 = !_9;
_1 = [112_i8,(-123_i8),(-26_i8),(-102_i8),(-123_i8)];
_2 = [127_i8,(-101_i8),35_i8,(-31_i8),60_i8];
_7 = 18201_u16 + 52667_u16;
_2 = [44_i8,(-122_i8),(-71_i8),(-37_i8),(-25_i8)];
_9 = _10 * _10;
RET = [5664935750474569449_usize,4777432902802326449_usize,4_usize,5_usize,12627400072043484277_usize];
_4 = true;
_2 = [83_i8,(-41_i8),19_i8,52_i8,(-54_i8)];
Goto(bb4)
}
bb8 = {
RET = [6_usize,1_usize,1_usize,11065564217549896971_usize,4_usize];
_7 = 168_u16;
_5 = '\u{6dac5}';
_1 = [(-33_i8),86_i8,(-52_i8),43_i8,108_i8];
_7 = 871675647_u32 as u16;
_4 = true;
_9 = 14165275098926058802_u64;
_7 = (-14392_i16) as u16;
_10 = 4276926558494340048_usize as u64;
RET = [10081603820602842927_usize,14778147905843736153_usize,1_usize,3_usize,4983887322291012386_usize];
_2 = [62_i8,126_i8,123_i8,13_i8,(-105_i8)];
_9 = !_10;
_3 = [(-5_i8),(-92_i8),73_i8,(-78_i8),(-83_i8)];
Call(_3 = core::intrinsics::transmute(_1), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_1 = _2;
_3 = _1;
_5 = '\u{7e700}';
RET = [6_usize,10655046986461341325_usize,18247645947189835286_usize,14515284834825244640_usize,2_usize];
RET = [5_usize,0_usize,3264728406369005973_usize,5_usize,5_usize];
_2 = [76_i8,(-79_i8),65_i8,60_i8,(-104_i8)];
_4 = false;
RET = [4_usize,7_usize,5189758388931894984_usize,3_usize,825290458161693952_usize];
RET = [2_usize,3_usize,8329488220827545748_usize,2_usize,6025444865312601399_usize];
_5 = '\u{5118}';
RET = [2_usize,7_usize,4661130108558204973_usize,3_usize,0_usize];
_5 = '\u{5e68c}';
_3 = [85_i8,108_i8,106_i8,(-98_i8),122_i8];
_3 = _2;
_3 = [(-72_i8),(-11_i8),(-32_i8),2_i8,114_i8];
_3 = [114_i8,45_i8,122_i8,(-81_i8),111_i8];
_3 = [15_i8,(-87_i8),(-14_i8),72_i8,2_i8];
_4 = _5 <= _5;
_2 = [(-44_i8),69_i8,(-77_i8),(-31_i8),26_i8];
_2 = [(-84_i8),(-115_i8),33_i8,(-105_i8),(-22_i8)];
_6 = [(-9_i8),113_i8,(-20_i8),57_i8,(-63_i8),27_i8,0_i8,(-88_i8)];
_5 = '\u{e654e}';
_2 = [103_i8,126_i8,48_i8,(-112_i8),51_i8];
_5 = '\u{9b025}';
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_7 = 36830_u16;
RET = _18;
_6 = [(-102_i8),(-128_i8),(-111_i8),(-5_i8),(-90_i8),(-114_i8),16_i8,102_i8];
_7 = 9365_u16 | 12536_u16;
_5 = '\u{6b25c}';
_5 = '\u{bb04f}';
_9 = !_10;
_12 = [_4,_4,_4];
_17 = [_4,_4,_4];
RET = _18;
_17 = [_4,_4,_4];
_7 = 14764_u16;
_18 = RET;
_14 = _13;
_16.0 = _10 as f32;
_1 = [(-99_i8),(-94_i8),91_i8,(-103_i8),(-120_i8)];
_13 = _15 as f64;
_3 = _2;
_14 = _13 - _13;
_14 = -_13;
_17 = [_4,_4,_4];
_3 = _1;
RET = _18;
_19 = !_15;
_2 = _1;
_20 = _13;
_20 = _13 - _13;
_17 = [_4,_4,_4];
Goto(bb13)
}
bb13 = {
_19 = _15 + _15;
_10 = 1734_i16 as u64;
_13 = -_20;
_5 = '\u{a53aa}';
_12 = _17;
_1 = [(-13_i8),(-36_i8),(-60_i8),100_i8,(-35_i8)];
_13 = _14 - _14;
_19 = _15 + _15;
_6 = [48_i8,(-113_i8),(-52_i8),23_i8,(-71_i8),(-68_i8),12_i8,89_i8];
_9 = 181_u8 as u64;
_14 = -_13;
_20 = _13;
_8 = (-9223372036854775808_isize) - (-9223372036854775808_isize);
match _7 {
0 => bb1,
1 => bb2,
14764 => bb14,
_ => bb4
}
}
bb14 = {
_2 = [(-49_i8),126_i8,71_i8,24_i8,(-105_i8)];
_19 = _15;
_15 = _19;
_7 = !37145_u16;
_10 = _16.0 as u64;
_2 = [(-109_i8),14_i8,(-18_i8),71_i8,(-78_i8)];
_2 = _1;
_5 = '\u{19f3a}';
_5 = '\u{2be7e}';
_24 = &_20;
_18 = [_19,_15,_19,_15,_15];
_24 = &(*_24);
_10 = _9 * _9;
_6 = [12_i8,(-78_i8),85_i8,(-98_i8),31_i8,(-10_i8),(-62_i8),127_i8];
_9 = _10;
_25 = _5;
_24 = &_14;
_14 = _13 + _20;
_6 = [59_i8,29_i8,(-47_i8),(-101_i8),101_i8,105_i8,121_i8,62_i8];
_13 = -_14;
_24 = &_13;
_19 = _15;
RET = [_15,_19,_19,_15,_19];
_12 = [_4,_4,_4];
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(19_usize, 17_usize, Move(_17), 6_usize, Move(_6), 8_usize, Move(_8), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(19_usize, 1_usize, Move(_1), 10_usize, Move(_10), 9_usize, Move(_9), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box(136_u8), std::hint::black_box(9223372036854775807_isize), std::hint::black_box((-89_i8)), std::hint::black_box(8213_i16), std::hint::black_box(1629470875_i32), std::hint::black_box(55315_u16), std::hint::black_box((-101860015221455325852099175766417748935_i128)), std::hint::black_box(6776925651980938760_usize));
                
            }
impl PrintFDebug for Adt20{
	unsafe fn printf_debug(&self){unsafe{printf("Adt20::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld7:".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt20 {
Variant0{
fld0: usize,
fld1: *const *mut f32,
fld2: u64,
fld3: i64,
fld4: u8,

},
Variant1{
fld0: bool,
fld1: usize,
fld2: u8,
fld3: i8,
fld4: u64,
fld5: i128,

},
Variant2{
fld0: *mut f32,
fld1: char,
fld2: isize,
fld3: usize,
fld4: f32,
fld5: i32,
fld6: f64,

},
Variant3{
fld0: *const *mut f32,
fld1: usize,
fld2: u16,
fld3: f64,
fld4: u64,
fld5: i32,
fld6: i64,
fld7: u8,

}}
impl PrintFDebug for Adt21{
	unsafe fn printf_debug(&self){unsafe{printf("Adt21::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt21 {
Variant0{
fld0: f64,
fld1: f32,
fld2: Adt20,

},
Variant1{
fld0: f32,
fld1: *mut f32,
fld2: f64,
fld3: i8,
fld4: u16,
fld5: i32,
fld6: i64,

}}
impl PrintFDebug for Adt22{
	unsafe fn printf_debug(&self){unsafe{printf("Adt22::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt22 {
Variant0{
fld0: bool,
fld1: char,
fld2: Adt21,
fld3: i8,
fld4: f64,

},
Variant1{
fld0: i32,
fld1: char,

}}
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
#[derive(Copy,Clone)]pub enum Adt31 {
Variant0{
fld0: *mut f32,
fld1: Adt21,
fld2: usize,

},
Variant1{
fld0: [u16; 3],

},
Variant2{
fld0: u64,
fld1: char,
fld2: *const *mut f32,
fld3: u128,
fld4: Adt22,

},
Variant3{
fld0: f64,
fld1: Adt21,
fld2: u16,
fld3: u64,
fld4: u128,
fld5: u8,
fld6: [u16; 3],

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: u32,
fld1: (*mut f32, u16, Adt22, i64),
fld2: (char, *const i128, i32),
fld3: i8,

},
Variant1{
fld0: *mut f32,
fld1: char,
fld2: isize,
fld3: i8,
fld4: usize,
fld5: (*mut Adt21,),
fld6: (Adt22, i64, isize),

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: u16,
fld1: (*mut Adt21,),
fld2: *const *mut f32,
fld3: i128,
fld4: u128,
fld5: [i64; 7],
fld6: *const i16,

},
Variant1{
fld0: f32,
fld1: [u64; 7],
fld2: isize,
fld3: *const ([usize; 5],),
fld4: i16,
fld5: u64,

},
Variant2{
fld0: [i16; 3],
fld1: isize,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: Adt50,
fld1: [u16; 3],
fld2: f32,
fld3: (*mut Adt21,),
fld4: ([usize; 5], *const i16, (char, *const i128, i32), [u16; 3]),

},
Variant1{
fld0: *const Adt20,
fld1: u64,
fld2: isize,
fld3: ([usize; 5],),
fld4: u128,
fld5: f32,

}}
impl PrintFDebug for Adt77{
	unsafe fn printf_debug(&self){unsafe{printf("Adt77::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt77 {
Variant0{
fld0: [i16; 7],
fld1: ((char, *const i128, i32),),
fld2: *const *mut f32,

},
Variant1{
fld0: *const ([usize; 5],),
fld1: *mut i64,
fld2: [u64; 7],
fld3: f32,
fld4: [i16; 7],

},
Variant2{
fld0: [i16; 7],
fld1: (*mut f32, u16, Adt22, i64),

}}

