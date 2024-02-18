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
pub fn fn0(mut _1: i8,mut _2: isize) -> f64 {
mir! {
type RET = f64;
let _3: &'static u16;
let _4: Adt44;
let _5: (u64,);
let _6: f64;
let _7: &'static (u32, isize, u8);
let _8: (&'static *mut u8, (u32, isize, u8));
let _9: f32;
let _10: isize;
let _11: &'static i8;
let _12: u16;
let _13: i32;
let _14: Adt43;
let _15: (u32, isize, u8);
let _16: (*mut i64,);
let _17: isize;
let _18: u128;
let _19: *const &'static Adt21;
let _20: ();
let _21: ();
{
RET = 137089085532541495758967140790702419462_i128 as f64;
_1 = (-44_i8) | (-32_i8);
RET = 48454536364832930006416788860889275435_u128 as f64;
_2 = (-9223372036854775808_isize);
_2 = (-77_isize) ^ 9223372036854775807_isize;
_2 = RET as isize;
RET = 47_u8 as f64;
_1 = 18799_i16 as i8;
_2 = 8654155651130429516_u64 as isize;
_2 = 47_isize ^ 73_isize;
RET = _1 as f64;
Call(RET = fn1(_1, _2, _2, _2, _2, _2, _2, _1, _2, _2, _2, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = 43_isize;
match _2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
43 => bb8,
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
_2 = 114_isize;
RET = 18_u8 as f64;
RET = 22021_u16 as f64;
RET = 3_usize as f64;
RET = _2 as f64;
_1 = (-32_i8) << _2;
RET = 14336_u16 as f64;
RET = 526308885_u32 as f64;
_2 = 84_isize << _1;
RET = 10082125116041712026_u64 as f64;
RET = 249_u8 as f64;
Goto(bb9)
}
bb9 = {
_1 = -87_i8;
_2 = 9223372036854775807_isize << _1;
_1 = !96_i8;
_1 = 28_i8 & (-101_i8);
_2 = 9223372036854775807_isize >> _1;
_4.fld0 = core::ptr::addr_of_mut!(_1);
_1 = !(-18_i8);
_4.fld2 = 31323_u16 as f32;
Goto(bb10)
}
bb10 = {
_5 = (4112630595537434933_u64,);
_4.fld4 = -(-699266180_i32);
_5.0 = 3018633806641908841_u64 & 17894988192667741546_u64;
_4.fld3 = _1;
RET = 0_usize as f64;
_5 = (10568144017302400513_u64,);
_6 = (-11340_i16) as f64;
_4.fld2 = 43006_u16 as f32;
_4.fld2 = (-38100131791346685417078063857212014642_i128) as f32;
_4.fld3 = -_1;
_4.fld2 = (-19889_i16) as f32;
_4.fld3 = _6 as i8;
RET = _6 - _6;
_1 = _4.fld3;
Goto(bb11)
}
bb11 = {
_5 = (15814351664197272373_u64,);
_4.fld4 = '\u{a9ae}' as i32;
_8.1.0 = 1782134313_u32;
_8.1.2 = !19_u8;
_8.1.0 = '\u{92867}' as u32;
Goto(bb12)
}
bb12 = {
_8.0 = &_4.fld1;
_1 = _4.fld3 << _4.fld3;
_4.fld1 = core::ptr::addr_of_mut!(_8.1.2);
_12 = 56043_u16;
_4.fld2 = _8.1.0 as f32;
_9 = _4.fld2;
_8.0 = &_4.fld1;
_6 = RET * RET;
_3 = &_12;
_2 = !101_isize;
_11 = &_1;
_8.1 = (3679943958_u32, _2, 132_u8);
_1 = _4.fld3 | _4.fld3;
_9 = _4.fld2 * _4.fld2;
match _8.1.0 {
0 => bb11,
3679943958 => bb13,
_ => bb8
}
}
bb13 = {
_15 = _8.1;
_14 = Adt43::Variant2 { fld0: _15.0,fld1: 16998793407223362154_usize,fld2: _8.1.1,fld3: 3716999315654414226_i64 };
place!(Field::<usize>(Variant(_14, 2), 1)) = 10355902821860767527_usize;
place!(Field::<i64>(Variant(_14, 2), 3)) = _9 as i64;
_8.1.0 = _4.fld3 as u32;
_7 = &_8.1;
match (*_7).2 {
132 => bb14,
_ => bb9
}
}
bb14 = {
_10 = Field::<isize>(Variant(_14, 2), 2) | _15.1;
_5.0 = 1551762295902229540_u64 * 9205243625711019474_u64;
_15.1 = _8.1.1 | _8.1.1;
_3 = &(*_3);
place!(Field::<i64>(Variant(_14, 2), 3)) = -(-1918824541328955894_i64);
_5.0 = 16938150472464539216_u64 ^ 10942287844419303002_u64;
RET = -_6;
_3 = &(*_3);
RET = (*_3) as f64;
_15.2 = !_8.1.2;
_17 = (*_7).1;
_6 = RET - RET;
_15.0 = (*_7).0;
place!(Field::<u32>(Variant(_14, 2), 0)) = _8.1.0 - _15.0;
_15.0 = Field::<usize>(Variant(_14, 2), 1) as u32;
_5 = (18437936007819071914_u64,);
_15.2 = (*_7).2 ^ (*_7).2;
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(0_usize, 15_usize, Move(_15), 5_usize, Move(_5), 1_usize, Move(_1), 21_usize, _21), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i8,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: i8,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: i8) -> f64 {
mir! {
type RET = f64;
let _13: (&'static f32, ([usize; 5], char, &'static *mut u8), [usize; 5], i32);
let _14: u16;
let _15: [i32; 3];
let _16: u128;
let _17: isize;
let _18: (u64, (u32, isize, u8), &'static u16);
let _19: f64;
let _20: u16;
let _21: isize;
let _22: [u32; 6];
let _23: ([usize; 5], char, &'static *mut u8);
let _24: [char; 5];
let _25: [char; 5];
let _26: &'static Adt44;
let _27: i8;
let _28: isize;
let _29: char;
let _30: u64;
let _31: *mut i64;
let _32: &'static *const ([usize; 5], char, &'static *mut u8);
let _33: &'static Adt21;
let _34: *const f64;
let _35: isize;
let _36: *mut *mut (&'static *mut u8, (u32, isize, u8));
let _37: ();
let _38: ();
{
RET = 8412106406205309813_u64 as f64;
_1 = _8;
_13.1.1 = '\u{6a118}';
_8 = !_12;
_6 = _2;
RET = (-153598249756011827090531458615266037362_i128) as f64;
_10 = -_4;
RET = 58658_u16 as f64;
_8 = 8166_i16 as i8;
_5 = _11 >> _8;
_13.3 = (-1408177339_i32);
RET = 1_usize as f64;
_14 = 18064016272161304864_u64 as u16;
_14 = _13.3 as u16;
_13.1.1 = '\u{749c4}';
_16 = 212086033181616684931078521504577866437_u128 & 317143889889157517898942770805666230035_u128;
RET = (-70754843666872667957957650988219413964_i128) as f64;
_7 = -_9;
_2 = _6;
_12 = _8;
Goto(bb1)
}
bb1 = {
_3 = !_11;
_13.3 = RET as i32;
_15 = [_13.3,_13.3,_13.3];
_11 = _9 << _7;
RET = 1935597031_u32 as f64;
_5 = _7 << _10;
_13.1.1 = '\u{cef79}';
_15 = [_13.3,_13.3,_13.3];
RET = 31_u8 as f64;
_4 = _9;
RET = _16 as f64;
_13.1.0 = [15363466250419761213_usize,7225411441865273506_usize,16658937805088479718_usize,3_usize,6_usize];
_15 = [_13.3,_13.3,_13.3];
RET = 120_u8 as f64;
_18.2 = &_14;
_13.1.0 = [4_usize,6_usize,16259368399205411998_usize,2_usize,6_usize];
_18.1.1 = _3 ^ _10;
_18.0 = 10608896728425459431_u64;
_10 = _4;
_18.1.0 = !2087231398_u32;
RET = 739_i16 as f64;
_4 = -_10;
_18.1.0 = 71447200_u32;
Goto(bb2)
}
bb2 = {
_13.1.1 = '\u{59e63}';
_11 = _4 - _6;
RET = _18.1.0 as f64;
match _18.1.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
71447200 => bb8,
_ => bb7
}
}
bb3 = {
_3 = !_11;
_13.3 = RET as i32;
_15 = [_13.3,_13.3,_13.3];
_11 = _9 << _7;
RET = 1935597031_u32 as f64;
_5 = _7 << _10;
_13.1.1 = '\u{cef79}';
_15 = [_13.3,_13.3,_13.3];
RET = 31_u8 as f64;
_4 = _9;
RET = _16 as f64;
_13.1.0 = [15363466250419761213_usize,7225411441865273506_usize,16658937805088479718_usize,3_usize,6_usize];
_15 = [_13.3,_13.3,_13.3];
RET = 120_u8 as f64;
_18.2 = &_14;
_13.1.0 = [4_usize,6_usize,16259368399205411998_usize,2_usize,6_usize];
_18.1.1 = _3 ^ _10;
_18.0 = 10608896728425459431_u64;
_10 = _4;
_18.1.0 = !2087231398_u32;
RET = 739_i16 as f64;
_4 = -_10;
_18.1.0 = 71447200_u32;
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
_17 = _11;
_19 = RET - RET;
_18.2 = &_14;
_15 = [_13.3,_13.3,_13.3];
_7 = _11;
_18.1.0 = !3385640271_u32;
_18.1 = (1389547697_u32, _3, 184_u8);
_7 = !_9;
RET = _19 * _19;
RET = _19;
_9 = _18.1.2 as isize;
RET = _13.3 as f64;
_3 = _9;
_16 = 214302310883463776601197001638477638_u128;
_4 = _3;
_18.2 = &_14;
_20 = _14;
_4 = -_5;
_9 = _17;
_13.2 = _13.1.0;
_18.2 = &_14;
_21 = !_9;
RET = _19 * _19;
_18.2 = &_20;
_8 = _1;
_16 = !186966155498890716247831542063867123470_u128;
_18.1.2 = 134_u8;
Goto(bb9)
}
bb9 = {
_15 = [_13.3,_13.3,_13.3];
_7 = _6;
_8 = !_1;
RET = _19;
_23.0 = [3_usize,15780433596388885602_usize,10643243947706392601_usize,3_usize,6_usize];
_22 = [_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0];
_18.2 = &_20;
_13.1.0 = [17503394894147267654_usize,1_usize,3303900988935139223_usize,4_usize,10105705261841418234_usize];
_3 = 13917367748017472434_usize as isize;
_18.2 = &_20;
_15 = [_13.3,_13.3,_13.3];
_13.3 = (-1273734814_i32);
_22 = [_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0];
_21 = _11 | _2;
_2 = _6 >> _17;
_6 = _18.1.1;
_3 = !_17;
_8 = _19 as i8;
_18.2 = &_14;
_13.1.1 = '\u{ebc6d}';
_10 = _4 | _5;
_13.1.0 = [2_usize,7_usize,13003809570553754975_usize,6_usize,7_usize];
_18.1.2 = 153_u8;
_4 = -_6;
_18.1.2 = 96_u8;
Call(_10 = core::intrinsics::transmute(_3), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_18.1.0 = _18.1.2 as u32;
_8 = 10700551448785983903451499553221999791_i128 as i8;
_2 = _4;
_13.1.1 = '\u{2bf1a}';
_25 = [_13.1.1,_13.1.1,_13.1.1,_13.1.1,_13.1.1];
Goto(bb11)
}
bb11 = {
_4 = !_3;
_13.3 = !10816112_i32;
RET = _19;
_28 = _16 as isize;
RET = _19;
_30 = !_18.0;
RET = _19 + _19;
_12 = _8;
_22 = [_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0];
_14 = _13.3 as u16;
_18.2 = &_20;
_16 = (-3726897096838495378_i64) as u128;
_18.0 = _10 as u64;
_24 = _25;
_5 = _3 | _21;
match _18.1.2 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb9,
96 => bb12,
_ => bb7
}
}
bb12 = {
_22 = [_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0];
_14 = _20;
_29 = _13.1.1;
_13.2 = [12150717839385723860_usize,0_usize,5_usize,3639217212294171663_usize,7_usize];
_11 = !_5;
_15 = [_13.3,_13.3,_13.3];
_29 = _13.1.1;
_11 = _9 & _3;
Goto(bb13)
}
bb13 = {
_7 = _5 - _3;
_6 = _20 as isize;
_13.3 = (-1195964594_i32);
_14 = _20 - _20;
_5 = _10;
_13.3 = 7_usize as i32;
_18.1.0 = !3030538995_u32;
_34 = core::ptr::addr_of!(RET);
_5 = -_21;
match _18.1.2 {
0 => bb14,
1 => bb15,
2 => bb16,
96 => bb18,
_ => bb17
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_13.1.1 = '\u{59e63}';
_11 = _4 - _6;
RET = _18.1.0 as f64;
match _18.1.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
71447200 => bb8,
_ => bb7
}
}
bb17 = {
_3 = !_11;
_13.3 = RET as i32;
_15 = [_13.3,_13.3,_13.3];
_11 = _9 << _7;
RET = 1935597031_u32 as f64;
_5 = _7 << _10;
_13.1.1 = '\u{cef79}';
_15 = [_13.3,_13.3,_13.3];
RET = 31_u8 as f64;
_4 = _9;
RET = _16 as f64;
_13.1.0 = [15363466250419761213_usize,7225411441865273506_usize,16658937805088479718_usize,3_usize,6_usize];
_15 = [_13.3,_13.3,_13.3];
RET = 120_u8 as f64;
_18.2 = &_14;
_13.1.0 = [4_usize,6_usize,16259368399205411998_usize,2_usize,6_usize];
_18.1.1 = _3 ^ _10;
_18.0 = 10608896728425459431_u64;
_10 = _4;
_18.1.0 = !2087231398_u32;
RET = 739_i16 as f64;
_4 = -_10;
_18.1.0 = 71447200_u32;
Goto(bb2)
}
bb18 = {
_22 = [_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0];
_22 = [_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0,_18.1.0];
_34 = core::ptr::addr_of!((*_34));
_13.1.0 = _13.2;
_21 = (-12827_i16) as isize;
_27 = !_8;
_15 = [_13.3,_13.3,_13.3];
Goto(bb19)
}
bb19 = {
Call(_37 = dump_var(1_usize, 4_usize, Move(_4), 30_usize, Move(_30), 16_usize, Move(_16), 2_usize, Move(_2)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_37 = dump_var(1_usize, 10_usize, Move(_10), 22_usize, Move(_22), 12_usize, Move(_12), 3_usize, Move(_3)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_37 = dump_var(1_usize, 1_usize, Move(_1), 6_usize, Move(_6), 20_usize, Move(_20), 9_usize, Move(_9)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box((-102_i8)), std::hint::black_box((-9223372036854775808_isize)));
                
            }
impl PrintFDebug for Adt21{
	unsafe fn printf_debug(&self){unsafe{printf("Adt21::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt21 {
Variant0{
fld0: i64,
fld1: i8,

},
Variant1{
fld0: f64,

}}
impl PrintFDebug for Adt36{
	unsafe fn printf_debug(&self){unsafe{printf("Adt36::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt36 {
Variant0{
fld0: [usize; 6],
fld1: (*mut u32,),
fld2: u8,
fld3: i128,
fld4: i16,

},
Variant1{
fld0: *mut u128,
fld1: *const i128,
fld2: u16,
fld3: (*mut u32,),
fld4: usize,
fld5: i32,
fld6: u8,
fld7: u32,

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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: bool,
fld1: f32,
fld2: *mut i16,
fld3: ([usize; 6],),
fld4: (u32, isize, u8),

},
Variant1{
fld0: f64,
fld1: char,
fld2: u8,
fld3: *mut u32,
fld4: f32,
fld5: (u64,),
fld6: *mut i64,
fld7: *mut u128,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: bool,
fld1: (u64,),
fld2: Adt36,

},
Variant1{
fld0: *const i128,
fld1: char,
fld2: Adt36,
fld3: *mut u8,
fld4: u16,
fld5: f64,
fld6: *mut u32,
fld7: *mut i16,

},
Variant2{
fld0: u32,
fld1: usize,
fld2: isize,
fld3: i64,

},
Variant3{
fld0: [usize; 6],
fld1: *mut u128,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: *mut i8,
fld1: *mut u8,
fld2: f32,
fld3: i8,
fld4: i32,
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: [usize; 3],
fld1: f64,
fld2: *const f64,
fld3: f32,
fld4: u8,
}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: [usize; 3],
fld1: char,
fld2: *const f64,
fld3: *mut i8,
fld4: *const i128,
fld5: Adt42,
fld6: ((u64,), usize),
fld7: u64,

},
Variant1{
fld0: u64,

},
Variant2{
fld0: (u64,),
fld1: char,
fld2: *mut u32,
fld3: *mut i16,

}}
impl PrintFDebug for Adt68{
	unsafe fn printf_debug(&self){unsafe{printf("Adt68::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt68 {
Variant0{
fld0: usize,
fld1: [usize; 3],
fld2: *mut u32,

},
Variant1{
fld0: [char; 5],
fld1: Adt43,
fld2: u32,
fld3: [u128; 5],
fld4: u8,

}}

