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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: u128,mut _6: u64,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u32) -> *const [i32; 8] {
mir! {
type RET = *const [i32; 8];
let _12: (f32,);
let _13: [char; 2];
let _14: i16;
let _15: [usize; 6];
let _16: [i32; 5];
let _17: (i8, bool);
let _18: (f32,);
let _19: *const (bool, bool, *const u64, i32, u64);
let _20: *const [i32; 8];
let _21: i8;
let _22: f32;
let _23: Adt43;
let _24: ();
let _25: ();
{
_4 = 16_i8;
_2 = '\u{546e}';
_10 = 126_u8;
_11 = 1153761628_u32;
_5 = _4 as u128;
_3 = 9223372036854775807_isize;
_7 = (-13638_i16) as i64;
_8 = (-83379782023617906821142968889910314068_i128);
_3 = 9223372036854775807_isize & 105_isize;
_6 = 271399823531548779_u64;
_9 = 2_usize + 5_usize;
_11 = 3308781660_u32;
_10 = 213_u8 ^ 144_u8;
_9 = !17859459422196712571_usize;
_1 = false;
_12.0 = _11 as f32;
_8 = 34182418172565263349731762053492210779_i128 * 170081094171389691609348101640109035606_i128;
_12.0 = 704680859_i32 as f32;
_7 = -(-1568492023850442466_i64);
_13 = [_2,_2];
_4 = (-498669956_i32) as i8;
_11 = 1562860456_u32;
_13 = [_2,_2];
_1 = !false;
Call(RET = fn1(_13, _4, _2, _8, _10, _10, _2, _13, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = _8 as u64;
_10 = !84_u8;
_5 = _12.0 as u128;
_15 = [_9,_9,_9,_9,_9,_9];
_12.0 = 3205_u16 as f32;
_3 = (-70_isize);
_2 = '\u{2bb4d}';
_3 = !(-23_isize);
_5 = 78029753796717279185703253439560686848_u128;
_7 = (-3554137704098569723_i64);
_10 = _11 as u8;
_15 = [_9,_9,_9,_9,_9,_9];
match _5 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
78029753796717279185703253439560686848 => bb8,
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
_6 = !15566726440841019736_u64;
_7 = 1388874944_i32 as i64;
_12.0 = _6 as f32;
_15 = [_9,_9,_9,_9,_9,_9];
_18.0 = _12.0 + _12.0;
_17.0 = _18.0 as i8;
_15 = [_9,_9,_9,_9,_9,_9];
_4 = -_17.0;
_12 = (_18.0,);
match _5 {
0 => bb6,
1 => bb3,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
78029753796717279185703253439560686848 => bb14,
_ => bb13
}
}
bb9 = {
_6 = _8 as u64;
_10 = !84_u8;
_5 = _12.0 as u128;
_15 = [_9,_9,_9,_9,_9,_9];
_12.0 = 3205_u16 as f32;
_3 = (-70_isize);
_2 = '\u{2bb4d}';
_3 = !(-23_isize);
_5 = 78029753796717279185703253439560686848_u128;
_7 = (-3554137704098569723_i64);
_10 = _11 as u8;
_15 = [_9,_9,_9,_9,_9,_9];
match _5 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
78029753796717279185703253439560686848 => bb8,
_ => bb7
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
Return()
}
bb14 = {
_11 = _3 as u32;
_17 = Checked(_4 * _4);
_9 = _11 as usize;
_9 = 7_usize;
_12.0 = _18.0;
_14 = (-2636_i16) & (-4663_i16);
_21 = -_4;
_6 = !1085407123461695509_u64;
_23.fld0 = core::ptr::addr_of!(_5);
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(0_usize, 4_usize, Move(_4), 15_usize, Move(_15), 6_usize, Move(_6), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(0_usize, 10_usize, Move(_10), 11_usize, Move(_11), 14_usize, Move(_14), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: [char; 2],mut _2: i8,mut _3: char,mut _4: i128,mut _5: u8,mut _6: u8,mut _7: char,mut _8: [char; 2],mut _9: char) -> *const [i32; 8] {
mir! {
type RET = *const [i32; 8];
let _10: Adt51;
let _11: u64;
let _12: isize;
let _13: isize;
let _14: isize;
let _15: Adt49;
let _16: Adt48;
let _17: &'static *const u64;
let _18: [usize; 6];
let _19: (f32,);
let _20: char;
let _21: f32;
let _22: bool;
let _23: bool;
let _24: [u8; 3];
let _25: *mut u8;
let _26: usize;
let _27: i32;
let _28: [i32; 8];
let _29: [i32; 8];
let _30: [usize; 6];
let _31: i32;
let _32: Adt52;
let _33: ();
let _34: ();
{
_1 = _8;
_2 = -(-86_i8);
_7 = _3;
_8 = [_3,_3];
_4 = (-16293895151893862541868280203825343093_i128);
_6 = 15368838950081411164_u64 as u8;
_8 = [_3,_3];
_10.fld5 = Adt49 { fld0: 5_usize };
_6 = _5 & _5;
_10.fld4 = [347420497639814558_i64,(-3093247588139305066_i64),(-4559154031781174392_i64),(-7226243443558655804_i64),(-863557742359443436_i64),(-2748685990013001716_i64)];
_10.fld1 = [_6,_6,_5];
_12 = -9223372036854775807_isize;
Call(_10.fld5.fld0 = fn2(_10.fld4, _3, _10.fld4, _12), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10.fld5.fld0 = 7_usize & 9027760448920144517_usize;
_10.fld5.fld0 = 4099530444144966740_usize;
_10.fld4 = [1334999075362433554_i64,8785832030081128991_i64,6495138851372025336_i64,(-4932440222606515227_i64),(-4077630807021746041_i64),(-7629733165932672958_i64)];
_9 = _3;
_14 = _12 << _10.fld5.fld0;
_10.fld1 = [_6,_6,_6];
_10.fld5.fld0 = 7_usize;
_8 = _1;
_4 = (-854264491188889775147493147699336955_i128);
_1 = [_3,_7];
_15.fld0 = _10.fld5.fld0;
_10.fld4 = [(-1634738096005543658_i64),(-6075238819560064760_i64),1544525796326462233_i64,6905467949164601897_i64,(-347483461667023101_i64),(-6765290299517471720_i64)];
_10.fld3 = 206512338142638562496750456425368435666_u128;
_14 = _10.fld5.fld0 as isize;
_10.fld5.fld0 = _15.fld0;
_9 = _3;
_6 = !_5;
_5 = _6 * _6;
_1 = [_3,_3];
_10.fld5.fld0 = _15.fld0;
_4 = -(-59400530943724740926996585192820238954_i128);
match _10.fld5.fld0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
7 => bb10,
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
_10.fld0 = !true;
_15 = _10.fld5;
_6 = (-2881966876766497211_i64) as u8;
RET = core::ptr::addr_of!(_16.fld0.1);
_11 = !11851267081340447121_u64;
_10.fld1 = [_6,_5,_6];
_11 = (-1312516606_i32) as u64;
_18 = [_10.fld5.fld0,_15.fld0,_15.fld0,_15.fld0,_10.fld5.fld0,_10.fld5.fld0];
_10.fld4 = [5173177243434643390_i64,8724675906789822163_i64,(-5515062076302297183_i64),7085317291628077378_i64,4115172378919900688_i64,2651067129087272464_i64];
RET = core::ptr::addr_of!((*RET));
_2 = !112_i8;
(*RET) = [(-526236043_i32),(-1861907952_i32),(-1064675650_i32),770380885_i32,620183404_i32,1656673274_i32,(-2086926643_i32),52682436_i32];
RET = core::ptr::addr_of!((*RET));
_19.0 = _14 as f32;
_7 = _3;
_7 = _3;
_18 = [_10.fld5.fld0,_10.fld5.fld0,_10.fld5.fld0,_10.fld5.fld0,_10.fld5.fld0,_15.fld0];
Goto(bb11)
}
bb11 = {
_11 = _19.0 as u64;
_14 = -_12;
_15.fld0 = _10.fld5.fld0;
_20 = _9;
_3 = _20;
_16.fld0.0 = 8120_i16 as f64;
_10.fld0 = _19.0 >= _19.0;
_3 = _7;
_11 = 4552055336008249342_u64 + 673378222176970358_u64;
_16.fld0.1 = [(-1985574553_i32),240765422_i32,1716511586_i32,(-1629189662_i32),780874622_i32,131259878_i32,(-694644882_i32),1268723386_i32];
_10.fld4 = [1284314848481166663_i64,(-9172117416885974634_i64),7724640540369823007_i64,1403381291006384561_i64,5733598324204844023_i64,(-180666886467721793_i64)];
_9 = _20;
_13 = _14 | _12;
_10.fld4 = [1959771066063078388_i64,(-4201079040358949392_i64),5903288202615549062_i64,(-2500931241336076380_i64),8226846787282983447_i64,9151053426589809318_i64];
_10.fld0 = true;
_11 = 6310603029326443272_u64;
_10.fld5.fld0 = _15.fld0 << _5;
(*RET) = [(-743628849_i32),1094416137_i32,(-202567298_i32),1548608822_i32,668711756_i32,(-1682202215_i32),541878313_i32,(-1657452236_i32)];
_8 = _1;
_10.fld5.fld0 = _15.fld0;
_10.fld0 = true;
_1 = _8;
Call(_15.fld0 = fn6(_2, _10.fld4, _16.fld0, _16.fld0, _16, RET, _6, _16, _16.fld0.1, _13, _10.fld1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_20 = _7;
(*RET) = [1686033750_i32,(-2068167889_i32),109592449_i32,(-624701819_i32),(-973397032_i32),338677672_i32,1805148087_i32,(-388427901_i32)];
_10.fld3 = 320642616574567825091714622364852582085_u128;
_9 = _20;
(*RET) = [208079036_i32,1979722647_i32,(-2102316790_i32),(-2058093900_i32),314071947_i32,(-209730276_i32),(-1226808318_i32),814917739_i32];
_15.fld0 = _10.fld5.fld0 + _10.fld5.fld0;
_16.fld0.1 = [(-1209726240_i32),(-515877808_i32),(-1024872338_i32),1180917962_i32,(-907319664_i32),(-1026431837_i32),(-401678469_i32),1709338485_i32];
_13 = _14;
RET = core::ptr::addr_of!((*RET));
_4 = 24873406131972761604693159827420906786_i128 - 153076043705051766583454461197644911730_i128;
_26 = _15.fld0 / _10.fld5.fld0;
(*RET) = [1541670407_i32,(-374864849_i32),(-614076132_i32),138743042_i32,(-1972161850_i32),(-1670966250_i32),(-2036827036_i32),267262733_i32];
_27 = 403442984035828226_i64 as i32;
_20 = _3;
_10.fld5.fld0 = _15.fld0 + _26;
_15.fld0 = _10.fld5.fld0;
(*RET) = [_27,_27,_27,_27,_27,_27,_27,_27];
_29 = (*RET);
_10.fld5 = _15;
_10.fld5 = Adt49 { fld0: _15.fld0 };
_14 = -_12;
Goto(bb13)
}
bb13 = {
_21 = _19.0;
_15.fld0 = _5 as usize;
_10.fld0 = !true;
_8 = [_3,_20];
_32.fld6 = _27 as u8;
_10.fld0 = true;
_3 = _20;
_26 = _10.fld5.fld0 + _15.fld0;
_19 = (_21,);
(*RET) = [_27,_27,_27,_27,_27,_27,_27,_27];
_11 = !11316670490873304194_u64;
_28 = [_27,_27,_27,_27,_27,_27,_27,_27];
_1 = [_20,_7];
_3 = _20;
_16.fld0.1 = [_27,_27,_27,_27,_27,_27,_27,_27];
_27 = !(-1815687487_i32);
_32.fld1.fld2 = [2270745716113770361_i64,2096349627214907691_i64,(-7742663319641280945_i64),(-5637365984318341003_i64)];
_32.fld1.fld1.2.1 = _10.fld0;
_10.fld1 = [_5,_32.fld6,_5];
_32.fld2 = !_10.fld3;
(*RET) = [_27,_27,_27,_27,_27,_27,_27,_27];
Goto(bb14)
}
bb14 = {
_10.fld5 = _15;
_14 = _2 as isize;
(*RET) = [_27,_27,_27,_27,_27,_27,_27,_27];
_14 = !_12;
_19 = (_21,);
_32.fld1.fld1.0 = [_27,_27,_27,_27,_27,_27,_27,_27];
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(1_usize, 6_usize, Move(_6), 2_usize, Move(_2), 27_usize, Move(_27), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(1_usize, 26_usize, Move(_26), 11_usize, Move(_11), 29_usize, Move(_29), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(1_usize, 4_usize, Move(_4), 34_usize, _34, 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: [i64; 6],mut _2: char,mut _3: [i64; 6],mut _4: isize) -> usize {
mir! {
type RET = usize;
let _5: char;
let _6: (i8, bool);
let _7: isize;
let _8: [char; 2];
let _9: Adt49;
let _10: [i32; 5];
let _11: isize;
let _12: ([i32; 8], isize, (i8, bool));
let _13: *mut usize;
let _14: usize;
let _15: bool;
let _16: [i32; 8];
let _17: *mut [char; 2];
let _18: u32;
let _19: *const u16;
let _20: bool;
let _21: ([i32; 8], isize, (i8, bool));
let _22: [i64; 6];
let _23: Adt53;
let _24: *const u16;
let _25: [char; 2];
let _26: (bool, bool, *const u64, i32, u64);
let _27: isize;
let _28: ();
let _29: ();
{
RET = 0_usize;
RET = 0_usize;
_3[RET] = _1[RET];
_3[RET] = !_1[RET];
_1 = [_3[RET],_3[RET],_3[RET],_3[RET],_3[RET],_3[RET]];
_5 = _2;
_1 = _3;
_1 = _3;
RET = !7007772238493084406_usize;
_3 = [8690210996394311322_i64,2115740941542595837_i64,2891343192172655535_i64,8845418488426126695_i64,(-3614333022854592569_i64),(-3327404411738026823_i64)];
_2 = _5;
_1 = [(-8112264000684513510_i64),9145001948389701065_i64,4150011596097648135_i64,(-890195152647484645_i64),(-7548356476386162379_i64),(-1839343568494059089_i64)];
_3 = [8107293297218154169_i64,(-7851004152774634560_i64),8104119484339757235_i64,7635654898869153745_i64,6859341591074985240_i64,2746879458539009217_i64];
_6.0 = 9_i8;
_8 = [_2,_2];
_6.1 = !true;
_9 = Adt49 { fld0: RET };
_10 = [(-1691087147_i32),75981622_i32,(-1050060927_i32),937387175_i32,(-2025259379_i32)];
RET = !_9.fld0;
_6.1 = true & false;
_11 = -_4;
Call(_12.1 = fn3(_2, _11, _6.0, _6.1, _6.0, _10), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12.2.1 = _6.1;
_9 = Adt49 { fld0: RET };
RET = _9.fld0;
_7 = -_11;
_6.1 = _12.2.1;
_12.0 = [224143216_i32,138593248_i32,(-378054132_i32),1964548810_i32,2145120877_i32,1805515829_i32,(-1688327590_i32),759880908_i32];
_12.2.0 = 332715738003400418207359672682839804747_u128 as i8;
RET = 1128023640_i32 as usize;
_8 = [_5,_2];
_13 = core::ptr::addr_of_mut!(RET);
match _6.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
9 => bb7,
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
RET = _9.fld0 - _9.fld0;
_4 = 2152_i16 as isize;
(*_13) = !_9.fld0;
_2 = _5;
_12.2.0 = -_6.0;
_15 = _7 > _12.1;
_9 = Adt49 { fld0: RET };
match _6.0 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb8,
5 => bb9,
9 => bb11,
_ => bb10
}
}
bb8 = {
_12.2.1 = _6.1;
_9 = Adt49 { fld0: RET };
RET = _9.fld0;
_7 = -_11;
_6.1 = _12.2.1;
_12.0 = [224143216_i32,138593248_i32,(-378054132_i32),1964548810_i32,2145120877_i32,1805515829_i32,(-1688327590_i32),759880908_i32];
_12.2.0 = 332715738003400418207359672682839804747_u128 as i8;
RET = 1128023640_i32 as usize;
_8 = [_5,_2];
_13 = core::ptr::addr_of_mut!(RET);
match _6.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
9 => bb7,
_ => bb6
}
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_2 = _5;
_16 = [2045464050_i32,1254553560_i32,536523608_i32,(-1472714076_i32),(-1375201708_i32),2029239196_i32,(-73061614_i32),(-405525491_i32)];
_12 = (_16, _7, _6);
_9.fld0 = (*_13) >> _12.2.0;
_6.1 = !_15;
_4 = _12.1 ^ _12.1;
RET = _9.fld0;
_15 = _6.1;
_7 = _11;
_16 = [(-2116380359_i32),1223154033_i32,888465952_i32,1872359545_i32,(-804704967_i32),1888827833_i32,1809576403_i32,191602214_i32];
(*_13) = _9.fld0 ^ _9.fld0;
_11 = _4 ^ _12.1;
_2 = _5;
_3 = _1;
_12.1 = !_11;
RET = _9.fld0 + _9.fld0;
_1 = _3;
_11 = _4 >> _7;
_20 = _15 ^ _15;
_1 = [2232142747955091744_i64,(-8544348634970657046_i64),2763430886036938731_i64,4770500208517805261_i64,(-8361670383373763381_i64),(-1882202957923493714_i64)];
_5 = _2;
_12.2.1 = _15;
Goto(bb12)
}
bb12 = {
_4 = -_12.1;
_15 = !_20;
_14 = RET & (*_13);
_18 = (-872880717_i32) as u32;
_13 = core::ptr::addr_of_mut!(_14);
_21.1 = -_4;
_12.2.1 = _15;
_18 = 1355791225_u32;
_14 = _9.fld0;
_6.1 = _12.2.1;
_3 = [(-4501442570213660857_i64),(-1735644790523571433_i64),(-7114647456333776233_i64),7597385728759424845_i64,(-6956020057696620244_i64),(-1332587809886665120_i64)];
_5 = _2;
_3 = [(-239899398604716588_i64),(-3897306801434652768_i64),5289141756983459675_i64,4929092943295225511_i64,4995907225060363450_i64,(-4796850269023738182_i64)];
_21 = (_12.0, _4, _6);
_21 = (_12.0, _11, _12.2);
_6 = (_12.2.0, _21.2.1);
_21 = (_12.0, _7, _6);
_21.1 = _4 + _7;
_21.2.1 = _21.1 == _21.1;
RET = _9.fld0;
_8 = [_5,_2];
_12 = (_21.0, _11, _21.2);
match _21.2.0 {
0 => bb1,
1 => bb9,
9 => bb13,
_ => bb3
}
}
bb13 = {
_16 = _21.0;
_10 = [700349474_i32,(-217729325_i32),(-938904425_i32),(-85191983_i32),1163077501_i32];
RET = _9.fld0 >> _4;
_12.2.0 = 8943905501248825354_u64 as i8;
_21.0 = _12.0;
_13 = core::ptr::addr_of_mut!(_14);
Goto(bb14)
}
bb14 = {
_3 = [(-5508087250186095831_i64),(-3947140997600117437_i64),2720169301152969148_i64,9129931995247141047_i64,(-1675530474936561688_i64),2505174317898016899_i64];
_17 = core::ptr::addr_of_mut!(_25);
_21.2.0 = !_12.2.0;
(*_13) = RET;
_12.0 = [2065981461_i32,1446991725_i32,1616044915_i32,(-1224307655_i32),1645825408_i32,(-9671746_i32),1021227951_i32,(-1401591319_i32)];
_14 = RET | RET;
_3 = [6953988340424460239_i64,414265058351056487_i64,5459404696631046310_i64,(-1262155078490165523_i64),(-2343227016170965898_i64),3835215692800937419_i64];
_18 = 1728587568_u32 & 3664577492_u32;
_12.1 = 1186_u16 as isize;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(2_usize, 12_usize, Move(_12), 2_usize, Move(_2), 5_usize, Move(_5), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(2_usize, 16_usize, Move(_16), 7_usize, Move(_7), 3_usize, Move(_3), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: char,mut _2: isize,mut _3: i8,mut _4: bool,mut _5: i8,mut _6: [i32; 5]) -> isize {
mir! {
type RET = isize;
let _7: i128;
let _8: (f64, [i32; 8]);
let _9: (f64, [i32; 8]);
let _10: *const [i32; 8];
let _11: Adt43;
let _12: u16;
let _13: [char; 2];
let _14: u16;
let _15: [i64; 4];
let _16: Adt49;
let _17: f32;
let _18: f64;
let _19: [usize; 6];
let _20: i16;
let _21: [usize; 6];
let _22: [usize; 6];
let _23: f32;
let _24: *const (bool, bool, *const u64, i32, u64);
let _25: (f64, [i32; 8]);
let _26: isize;
let _27: ([i32; 8], isize, (i8, bool));
let _28: *mut usize;
let _29: *const u64;
let _30: (i8, bool);
let _31: i64;
let _32: ();
let _33: ();
{
_2 = -9223372036854775807_isize;
_6 = [1890476507_i32,(-1185985049_i32),(-1423406657_i32),(-142519284_i32),1747451008_i32];
Goto(bb1)
}
bb1 = {
_6 = [117504054_i32,(-2023561654_i32),869836774_i32,(-1597158269_i32),437153164_i32];
_4 = !true;
_4 = _1 > _1;
RET = _2 * _2;
_6 = [292184719_i32,413390038_i32,1609125973_i32,(-1319719877_i32),(-1214866604_i32)];
Call(_4 = fn4(RET, _5, _2, RET, RET, _6, RET, _5, _5, _5, _5, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8.1 = [107459639_i32,1484486802_i32,1147469611_i32,1203991324_i32,2055253571_i32,(-56997868_i32),(-1307953756_i32),1025333300_i32];
_8.0 = 64732_u16 as f64;
_8.0 = 171025344376636366_usize as f64;
_8.0 = 3346459956879251761_u64 as f64;
_7 = (-135599747624690493838358868263000297010_i128) ^ (-136166127579790032088822117068249245724_i128);
_9.0 = _8.0;
_6 = [859243737_i32,1122919261_i32,(-69747500_i32),801052944_i32,35837330_i32];
_8.1 = [172175171_i32,(-1229866334_i32),(-249146228_i32),(-807136530_i32),(-1761500726_i32),1697618810_i32,(-953063920_i32),495563546_i32];
_9 = (_8.0, _8.1);
_8.0 = (-7199539420416777611_i64) as f64;
_4 = RET == RET;
_2 = _3 as isize;
_5 = _3 >> _7;
RET = !_2;
RET = _2;
RET = -_2;
_1 = '\u{6e48c}';
_4 = true;
_4 = _8.0 < _9.0;
_6 = [(-1841795793_i32),525749991_i32,46373303_i32,1350861929_i32,(-475167522_i32)];
_8.1 = [1377518942_i32,(-1747057016_i32),281923668_i32,1845436231_i32,(-1894184095_i32),448049862_i32,(-1881512447_i32),(-854550805_i32)];
_6 = [1547116709_i32,1365442098_i32,2087996919_i32,519707200_i32,1348549641_i32];
_7 = 124875094169979487972404229418335955947_i128;
_8.0 = _9.0 - _9.0;
Goto(bb3)
}
bb3 = {
_2 = !RET;
_9.0 = _8.0 - _8.0;
_8.0 = 105353244277524230027422675502428951554_u128 as f64;
_4 = !false;
_8.0 = _9.0 - _9.0;
_7 = _2 as i128;
_10 = core::ptr::addr_of!(_8.1);
_9.0 = _8.0;
_3 = 12020499094536891436_u64 as i8;
RET = -_2;
_6 = [(-1539637535_i32),(-2060135966_i32),(-1800998730_i32),(-2101624687_i32),(-331481310_i32)];
_2 = !RET;
_9.1 = [(-1709633774_i32),485313726_i32,(-1126959996_i32),803784256_i32,1758505794_i32,1521093032_i32,1098120677_i32,(-903163748_i32)];
_1 = '\u{31de9}';
_9.0 = _2 as f64;
_9.0 = _8.0;
_9 = _8;
Goto(bb4)
}
bb4 = {
(*_10) = [(-1127032792_i32),(-1467187270_i32),(-1904757483_i32),1245549779_i32,1489440207_i32,1039248422_i32,(-805625051_i32),306238686_i32];
_5 = -_3;
_1 = '\u{cfa28}';
_13 = [_1,_1];
_9.0 = -_8.0;
_5 = _3 >> _7;
_9.0 = _8.0;
_9 = _8;
_8 = _9;
_15 = [2445442389943882839_i64,(-3020865418390000538_i64),7421342917394467703_i64,347927892125602452_i64];
_1 = '\u{97567}';
_12 = 57624_u16;
Call(_11.fld0 = fn5(_10, _8.1, _10, _10, _8.0, _12), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_2 = !RET;
_3 = _5;
(*_10) = [(-968168669_i32),800076751_i32,550711253_i32,(-595329995_i32),1592461499_i32,1319708567_i32,1398909335_i32,946256741_i32];
_1 = '\u{ba740}';
_4 = _9.0 >= _9.0;
_4 = _9.0 > _8.0;
_14 = 147809992382862812033418647587039048639_u128 as u16;
match _12 {
0 => bb1,
1 => bb2,
57624 => bb6,
_ => bb3
}
}
bb6 = {
(*_10) = [(-1406422063_i32),1630609237_i32,(-958901055_i32),1171249235_i32,(-2084851307_i32),372700613_i32,(-593513527_i32),1398019702_i32];
_17 = _12 as f32;
_8 = _9;
_21 = [17075029227267367928_usize,18177804631394051558_usize,11339989484131661891_usize,2_usize,3_usize,12832141146189721610_usize];
_8.1 = [1118996433_i32,962775855_i32,427613765_i32,739859572_i32,788159774_i32,(-1534061837_i32),1013047585_i32,(-353088413_i32)];
_13 = [_1,_1];
_13 = [_1,_1];
_8.0 = -_9.0;
_8 = (_9.0, _9.1);
_11.fld1 = _13;
_19 = [2_usize,490458943442797991_usize,0_usize,2_usize,16966586359402814364_usize,2_usize];
RET = !_2;
_14 = _12;
(*_10) = _9.1;
_22 = [11126374511596334358_usize,13436642714805920422_usize,7895726877958627759_usize,4515215744885213845_usize,13323765447078149249_usize,12144577626930769143_usize];
_6 = [(-1132459801_i32),(-1072007736_i32),1692495165_i32,(-1422856577_i32),382313343_i32];
match _14 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
57624 => bb9,
_ => bb8
}
}
bb7 = {
_6 = [117504054_i32,(-2023561654_i32),869836774_i32,(-1597158269_i32),437153164_i32];
_4 = !true;
_4 = _1 > _1;
RET = _2 * _2;
_6 = [292184719_i32,413390038_i32,1609125973_i32,(-1319719877_i32),(-1214866604_i32)];
Call(_4 = fn4(RET, _5, _2, RET, RET, _6, RET, _5, _5, _5, _5, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_2 = !RET;
_9.0 = _8.0 - _8.0;
_8.0 = 105353244277524230027422675502428951554_u128 as f64;
_4 = !false;
_8.0 = _9.0 - _9.0;
_7 = _2 as i128;
_10 = core::ptr::addr_of!(_8.1);
_9.0 = _8.0;
_3 = 12020499094536891436_u64 as i8;
RET = -_2;
_6 = [(-1539637535_i32),(-2060135966_i32),(-1800998730_i32),(-2101624687_i32),(-331481310_i32)];
_2 = !RET;
_9.1 = [(-1709633774_i32),485313726_i32,(-1126959996_i32),803784256_i32,1758505794_i32,1521093032_i32,1098120677_i32,(-903163748_i32)];
_1 = '\u{31de9}';
_9.0 = _2 as f64;
_9.0 = _8.0;
_9 = _8;
Goto(bb4)
}
bb9 = {
_3 = !_5;
_8 = _9;
_18 = -_9.0;
_9 = (_18, (*_10));
_9 = (_18, _8.1);
_19 = [2205088701989677689_usize,12869654011765464879_usize,6_usize,1_usize,1803410943908462890_usize,6_usize];
_21 = _22;
_5 = -_3;
_23 = _17 + _17;
(*_10) = [166532485_i32,(-613953978_i32),2120214126_i32,(-45135553_i32),(-521272887_i32),(-2113823503_i32),(-510826008_i32),(-1511277360_i32)];
_12 = _14;
_5 = -_3;
(*_10) = [(-190162582_i32),(-156591981_i32),(-712246871_i32),(-1894696912_i32),482351371_i32,1232594252_i32,1040318060_i32,1224522515_i32];
_8.0 = _14 as f64;
_21 = [3528407616373197083_usize,6_usize,7901364875385743944_usize,13649104122512316666_usize,13302193120391218649_usize,2_usize];
_23 = 230793110908172013716987864365236713792_u128 as f32;
_18 = -_9.0;
_26 = 1247476880094000416_i64 as isize;
_8.0 = _3 as f64;
_19 = [1_usize,6_usize,25037759553695846_usize,4_usize,0_usize,7_usize];
match _12 {
0 => bb4,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
57624 => bb16,
_ => bb15
}
}
bb10 = {
_2 = !RET;
_9.0 = _8.0 - _8.0;
_8.0 = 105353244277524230027422675502428951554_u128 as f64;
_4 = !false;
_8.0 = _9.0 - _9.0;
_7 = _2 as i128;
_10 = core::ptr::addr_of!(_8.1);
_9.0 = _8.0;
_3 = 12020499094536891436_u64 as i8;
RET = -_2;
_6 = [(-1539637535_i32),(-2060135966_i32),(-1800998730_i32),(-2101624687_i32),(-331481310_i32)];
_2 = !RET;
_9.1 = [(-1709633774_i32),485313726_i32,(-1126959996_i32),803784256_i32,1758505794_i32,1521093032_i32,1098120677_i32,(-903163748_i32)];
_1 = '\u{31de9}';
_9.0 = _2 as f64;
_9.0 = _8.0;
_9 = _8;
Goto(bb4)
}
bb11 = {
_8.1 = [107459639_i32,1484486802_i32,1147469611_i32,1203991324_i32,2055253571_i32,(-56997868_i32),(-1307953756_i32),1025333300_i32];
_8.0 = 64732_u16 as f64;
_8.0 = 171025344376636366_usize as f64;
_8.0 = 3346459956879251761_u64 as f64;
_7 = (-135599747624690493838358868263000297010_i128) ^ (-136166127579790032088822117068249245724_i128);
_9.0 = _8.0;
_6 = [859243737_i32,1122919261_i32,(-69747500_i32),801052944_i32,35837330_i32];
_8.1 = [172175171_i32,(-1229866334_i32),(-249146228_i32),(-807136530_i32),(-1761500726_i32),1697618810_i32,(-953063920_i32),495563546_i32];
_9 = (_8.0, _8.1);
_8.0 = (-7199539420416777611_i64) as f64;
_4 = RET == RET;
_2 = _3 as isize;
_5 = _3 >> _7;
RET = !_2;
RET = _2;
RET = -_2;
_1 = '\u{6e48c}';
_4 = true;
_4 = _8.0 < _9.0;
_6 = [(-1841795793_i32),525749991_i32,46373303_i32,1350861929_i32,(-475167522_i32)];
_8.1 = [1377518942_i32,(-1747057016_i32),281923668_i32,1845436231_i32,(-1894184095_i32),448049862_i32,(-1881512447_i32),(-854550805_i32)];
_6 = [1547116709_i32,1365442098_i32,2087996919_i32,519707200_i32,1348549641_i32];
_7 = 124875094169979487972404229418335955947_i128;
_8.0 = _9.0 - _9.0;
Goto(bb3)
}
bb12 = {
(*_10) = [(-1406422063_i32),1630609237_i32,(-958901055_i32),1171249235_i32,(-2084851307_i32),372700613_i32,(-593513527_i32),1398019702_i32];
_17 = _12 as f32;
_8 = _9;
_21 = [17075029227267367928_usize,18177804631394051558_usize,11339989484131661891_usize,2_usize,3_usize,12832141146189721610_usize];
_8.1 = [1118996433_i32,962775855_i32,427613765_i32,739859572_i32,788159774_i32,(-1534061837_i32),1013047585_i32,(-353088413_i32)];
_13 = [_1,_1];
_13 = [_1,_1];
_8.0 = -_9.0;
_8 = (_9.0, _9.1);
_11.fld1 = _13;
_19 = [2_usize,490458943442797991_usize,0_usize,2_usize,16966586359402814364_usize,2_usize];
RET = !_2;
_14 = _12;
(*_10) = _9.1;
_22 = [11126374511596334358_usize,13436642714805920422_usize,7895726877958627759_usize,4515215744885213845_usize,13323765447078149249_usize,12144577626930769143_usize];
_6 = [(-1132459801_i32),(-1072007736_i32),1692495165_i32,(-1422856577_i32),382313343_i32];
match _14 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
57624 => bb9,
_ => bb8
}
}
bb13 = {
_2 = !RET;
_3 = _5;
(*_10) = [(-968168669_i32),800076751_i32,550711253_i32,(-595329995_i32),1592461499_i32,1319708567_i32,1398909335_i32,946256741_i32];
_1 = '\u{ba740}';
_4 = _9.0 >= _9.0;
_4 = _9.0 > _8.0;
_14 = 147809992382862812033418647587039048639_u128 as u16;
match _12 {
0 => bb1,
1 => bb2,
57624 => bb6,
_ => bb3
}
}
bb14 = {
(*_10) = [(-1127032792_i32),(-1467187270_i32),(-1904757483_i32),1245549779_i32,1489440207_i32,1039248422_i32,(-805625051_i32),306238686_i32];
_5 = -_3;
_1 = '\u{cfa28}';
_13 = [_1,_1];
_9.0 = -_8.0;
_5 = _3 >> _7;
_9.0 = _8.0;
_9 = _8;
_8 = _9;
_15 = [2445442389943882839_i64,(-3020865418390000538_i64),7421342917394467703_i64,347927892125602452_i64];
_1 = '\u{97567}';
_12 = 57624_u16;
Call(_11.fld0 = fn5(_10, _8.1, _10, _10, _8.0, _12), ReturnTo(bb5), UnwindUnreachable())
}
bb15 = {
_6 = [117504054_i32,(-2023561654_i32),869836774_i32,(-1597158269_i32),437153164_i32];
_4 = !true;
_4 = _1 > _1;
RET = _2 * _2;
_6 = [292184719_i32,413390038_i32,1609125973_i32,(-1319719877_i32),(-1214866604_i32)];
Call(_4 = fn4(RET, _5, _2, RET, RET, _6, RET, _5, _5, _5, _5, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_23 = -_17;
_18 = _9.0;
_23 = _17 + _17;
_17 = (-8795814533054288709_i64) as f32;
_10 = core::ptr::addr_of!(_8.1);
_7 = (-1606208529399622003_i64) as i128;
_9 = (_8.0, (*_10));
_3 = 3466708958_u32 as i8;
_23 = _17 - _17;
_25 = (_18, (*_10));
RET = _26;
(*_10) = [(-415237803_i32),853657138_i32,1505623044_i32,2052907198_i32,1685875447_i32,1142589466_i32,409203544_i32,(-1334325154_i32)];
_9.0 = _25.0 - _18;
_5 = _17 as i8;
_23 = _7 as f32;
Goto(bb17)
}
bb17 = {
Call(_32 = dump_var(3_usize, 26_usize, Move(_26), 22_usize, Move(_22), 6_usize, Move(_6), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_32 = dump_var(3_usize, 14_usize, Move(_14), 12_usize, Move(_12), 3_usize, Move(_3), 33_usize, _33), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: isize,mut _2: i8,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: [i32; 5],mut _7: isize,mut _8: i8,mut _9: i8,mut _10: i8,mut _11: i8,mut _12: isize) -> bool {
mir! {
type RET = bool;
let _13: (i8, bool);
let _14: f32;
let _15: [usize; 6];
let _16: char;
let _17: Adt49;
let _18: u128;
let _19: (i8, bool);
let _20: f32;
let _21: ([i32; 8], isize, (i8, bool));
let _22: bool;
let _23: *const [i32; 8];
let _24: ();
let _25: ();
{
_5 = !_4;
RET = !false;
_2 = _8 << _10;
_3 = 13468_u16 as isize;
_6 = [(-515265827_i32),(-427757968_i32),(-15525028_i32),1031797086_i32,(-373143024_i32)];
_12 = -_1;
_8 = _2;
RET = !false;
_10 = _9;
_12 = '\u{ca62e}' as isize;
RET = !false;
_8 = RET as i8;
_8 = !_11;
_13.1 = !RET;
_6 = [903706527_i32,619893557_i32,303068334_i32,(-1527686186_i32),(-829259005_i32)];
RET = _13.1;
RET = !_13.1;
_1 = _7 << _4;
_13.0 = -_11;
_3 = _7;
_4 = -_1;
_14 = (-2021807767_i32) as f32;
RET = !_13.1;
_3 = _7 << _4;
_13 = (_2, RET);
match _11 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
9 => bb6,
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
_6 = [(-236635420_i32),1918003702_i32,(-646927320_i32),407697439_i32,352711944_i32];
RET = _13.1 & _13.1;
_14 = 224041041991526892403293054081662738276_u128 as f32;
Goto(bb7)
}
bb7 = {
_13 = (_8, RET);
_11 = _10;
_13.0 = _10 * _2;
_16 = '\u{a924d}';
_7 = -_5;
_13.0 = -_8;
RET = _13.1;
_14 = _2 as f32;
_16 = '\u{10ed91}';
_13 = (_2, RET);
_7 = _4 ^ _4;
_7 = 309977782446453151407655072946373148597_u128 as isize;
_10 = _13.0;
_2 = !_13.0;
RET = _13.1 & _13.1;
_7 = 301676994766894374622308207024210361606_u128 as isize;
_9 = -_8;
_9 = _13.0;
_14 = 38631876467111674470191413601510371549_u128 as f32;
_19.0 = _2 ^ _11;
_2 = _8 << _3;
_19 = _13;
_13.1 = !RET;
_10 = 247_u8 as i8;
_15 = [6748580500047396935_usize,0_usize,3_usize,3_usize,4041238827234848616_usize,0_usize];
_13.1 = _19.1;
match _11 {
0 => bb5,
1 => bb2,
2 => bb4,
3 => bb8,
4 => bb9,
5 => bb10,
9 => bb12,
_ => bb11
}
}
bb8 = {
_6 = [(-236635420_i32),1918003702_i32,(-646927320_i32),407697439_i32,352711944_i32];
RET = _13.1 & _13.1;
_14 = 224041041991526892403293054081662738276_u128 as f32;
Goto(bb7)
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
_7 = _2 as isize;
_6 = [802667419_i32,902089240_i32,(-187097838_i32),1009264800_i32,(-498709593_i32)];
_20 = _1 as f32;
RET = _19.1;
_21.2 = _13;
_15 = [12592798825996396524_usize,0_usize,11844525581525919935_usize,1640413392359561342_usize,11746230414210361992_usize,12593603246778350575_usize];
_15 = [4_usize,0_usize,14796460523023084401_usize,4172682757725788134_usize,4_usize,10435349869144918214_usize];
_2 = -_13.0;
_17.fld0 = 240035911_u32 as usize;
_3 = !_1;
_21.1 = !_7;
_18 = 324423065999161958072729522947281451851_u128;
_21.2.0 = _9 - _11;
_13.1 = !_19.1;
_11 = 101_u8 as i8;
_21.0 = [(-110700048_i32),(-1702163632_i32),(-103835112_i32),(-1508804037_i32),(-1049505286_i32),396318078_i32,(-950576403_i32),(-557928393_i32)];
_23 = core::ptr::addr_of!(_21.0);
_21.2.1 = RET;
match _18 {
0 => bb13,
1 => bb14,
2 => bb15,
324423065999161958072729522947281451851 => bb17,
_ => bb16
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_6 = [(-236635420_i32),1918003702_i32,(-646927320_i32),407697439_i32,352711944_i32];
RET = _13.1 & _13.1;
_14 = 224041041991526892403293054081662738276_u128 as f32;
Goto(bb7)
}
bb17 = {
_13.1 = _11 >= _19.0;
_17 = Adt49 { fld0: 13824520157417678933_usize };
Goto(bb18)
}
bb18 = {
Call(_24 = dump_var(4_usize, 4_usize, Move(_4), 12_usize, Move(_12), 2_usize, Move(_2), 9_usize, Move(_9)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_24 = dump_var(4_usize, 18_usize, Move(_18), 8_usize, Move(_8), 15_usize, Move(_15), 11_usize, Move(_11)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_24 = dump_var(4_usize, 21_usize, Move(_21), 25_usize, _25, 25_usize, _25, 25_usize, _25), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: *const [i32; 8],mut _2: [i32; 8],mut _3: *const [i32; 8],mut _4: *const [i32; 8],mut _5: f64,mut _6: u16) -> *const u128 {
mir! {
type RET = *const u128;
let _7: char;
let _8: Adt55;
let _9: *const u128;
let _10: char;
let _11: Adt44;
let _12: i8;
let _13: [char; 2];
let _14: *const (bool, bool, *const u64, i32, u64);
let _15: *const u128;
let _16: [i64; 4];
let _17: u32;
let _18: [char; 2];
let _19: (i8, bool);
let _20: char;
let _21: (i8, bool);
let _22: usize;
let _23: f64;
let _24: (f64, [i32; 8]);
let _25: ();
let _26: ();
{
_2 = [(-1810668201_i32),(-1355682932_i32),809375596_i32,1535690952_i32,(-1491677706_i32),473933964_i32,(-1884252812_i32),(-476178881_i32)];
(*_3) = [(-1153632966_i32),1717405439_i32,(-1790851404_i32),(-1950617886_i32),1649896651_i32,(-641725718_i32),(-600006752_i32),(-544055560_i32)];
(*_3) = _2;
_2 = [1272501150_i32,(-904968175_i32),(-1329392354_i32),861895811_i32,(-2002048295_i32),(-1360952968_i32),(-1638622402_i32),1247651838_i32];
_8.fld3 = [7350236544080256635_i64,(-2775874665340269229_i64),5231433874389372731_i64,(-1522678966107554063_i64),1396547572164068366_i64,1213683848268141055_i64];
_8.fld4 = [18239449204251819889_usize,4_usize,12151200085620857098_usize,17530302121380632967_usize,6_usize,9328255658746000542_usize];
(*_1) = [(-1038204447_i32),(-1159187530_i32),1848436115_i32,1013465711_i32,48044995_i32,157990850_i32,851147263_i32,275227622_i32];
(*_3) = [2011941340_i32,(-1163246216_i32),(-1482249325_i32),(-1068606892_i32),1590107805_i32,1237239873_i32,51657427_i32,(-713001930_i32)];
_1 = core::ptr::addr_of!(_2);
RET = core::ptr::addr_of!(_8.fld2);
_8.fld0.0 = !true;
_8.fld0.4 = (-38_i8) as u64;
(*_4) = (*_1);
_8.fld0.4 = 12482239122450752573_u64 - 7928801787122708741_u64;
(*RET) = 251532081418228675003492569528938822418_u128 * 21229626435629721372969896627392915883_u128;
(*_1) = [1126329304_i32,(-2006122903_i32),2125109278_i32,1837268262_i32,(-199500005_i32),11205192_i32,(-952571397_i32),2063597259_i32];
_5 = _8.fld2 as f64;
_3 = _4;
_8.fld0.4 = 9980047069000392160_u64;
_8.fld0.1 = !_8.fld0.0;
RET = core::ptr::addr_of!((*RET));
(*RET) = (-23514_i16) as u128;
_6 = 22709_u16;
(*RET) = 314705919511245784800162246185530354055_u128 * 122441226845461123271993845140253175904_u128;
match _6 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
22709 => bb7,
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
_4 = _3;
_8.fld0.0 = !_8.fld0.1;
(*_1) = [2044715554_i32,147163255_i32,252076042_i32,1002780218_i32,1264931166_i32,795933004_i32,1335091802_i32,(-1085671242_i32)];
_5 = (*RET) as f64;
(*_1) = [(-1888576332_i32),(-396999821_i32),400740234_i32,973102705_i32,1077669392_i32,1710361116_i32,(-1270886292_i32),1154681841_i32];
(*RET) = 108356358714802049726471703772856247931_u128 ^ 18161719125101214849487973501955529181_u128;
_9 = core::ptr::addr_of!((*RET));
_8.fld5 = 104_i8 as u64;
_8.fld0.4 = !_8.fld5;
(*_4) = [(-130134426_i32),(-1045817252_i32),534730782_i32,(-1027018359_i32),(-1191184432_i32),(-1108135197_i32),2001624206_i32,(-495431854_i32)];
_2 = [1545716043_i32,40765160_i32,(-1878078259_i32),(-1125400082_i32),1934694045_i32,(-923170984_i32),(-1679552892_i32),(-1107266895_i32)];
_9 = core::ptr::addr_of!((*_9));
(*RET) = 137531078932112153279997758591836133921_u128;
_8.fld3 = [(-8433251995556379165_i64),2651220267544059607_i64,8225528618069122315_i64,5895296706592608075_i64,(-6317892227841060296_i64),(-4795438419473214543_i64)];
_1 = core::ptr::addr_of!(_2);
_8.fld0.3 = 2054132751_i32 - (-1249711924_i32);
Goto(bb8)
}
bb8 = {
(*RET) = _8.fld0.4 as u128;
Call(_12 = core::intrinsics::transmute(_8.fld0.1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_5 = (-37600873924976649951365926067139976651_i128) as f64;
_3 = _4;
_8.fld0.3 = 1942443073_i32;
_8.fld0.2 = core::ptr::addr_of!(_8.fld5);
_4 = core::ptr::addr_of!((*_4));
_10 = '\u{8cc57}';
(*RET) = _12 as u128;
_10 = '\u{31f2c}';
_8.fld5 = _8.fld0.4;
RET = core::ptr::addr_of!((*RET));
_8.fld2 = 175_u8 as u128;
(*_3) = _2;
(*_3) = (*_1);
_13 = [_10,_10];
_8.fld4 = [1_usize,1_usize,395154681605478762_usize,2317453303458302914_usize,4_usize,7_usize];
(*_4) = [_8.fld0.3,_8.fld0.3,_8.fld0.3,_8.fld0.3,_8.fld0.3,_8.fld0.3,_8.fld0.3,_8.fld0.3];
_8.fld0.3 = 1494673491_i32 + 1104608364_i32;
_8.fld0.2 = core::ptr::addr_of!(_8.fld5);
(*_3) = (*_1);
(*_1) = [_8.fld0.3,_8.fld0.3,_8.fld0.3,_8.fld0.3,_8.fld0.3,_8.fld0.3,_8.fld0.3,_8.fld0.3];
match _6 {
0 => bb2,
22709 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_8.fld4 = [3_usize,0_usize,5_usize,11039180673830196175_usize,7_usize,2624782994984496271_usize];
_8.fld0.3 = (-47296600_i32);
(*RET) = 312972313805454133450470278320316276020_u128;
_8.fld5 = _8.fld0.4;
_8.fld5 = _8.fld0.4 + _8.fld0.4;
(*_9) = 262906775684317162261816149408033376627_u128 >> _12;
RET = _9;
_12 = 23_isize as i8;
_7 = _10;
(*_1) = (*_4);
(*_1) = (*_3);
_8.fld0.2 = core::ptr::addr_of!(_8.fld5);
_6 = 24010_u16 + 9755_u16;
_8.fld0.3 = 1584962016_i32;
_14 = core::ptr::addr_of!(_8.fld0);
_8.fld0.4 = _8.fld5 * _8.fld5;
RET = core::ptr::addr_of!((*_9));
_15 = _9;
_8.fld2 = 47312382605610478233749800623701829160_u128;
_8.fld0.0 = _8.fld0.1;
_8.fld2 = 82505020332273739352855605045953030884_u128;
(*_1) = [(*_14).3,(*_14).3,(*_14).3,(*_14).3,(*_14).3,_8.fld0.3,(*_14).3,_8.fld0.3];
_9 = core::ptr::addr_of!((*RET));
match (*_14).3 {
0 => bb8,
1 => bb2,
2 => bb7,
3 => bb10,
4 => bb12,
5 => bb13,
1584962016 => bb15,
_ => bb14
}
}
bb12 = {
Return()
}
bb13 = {
_4 = _3;
_8.fld0.0 = !_8.fld0.1;
(*_1) = [2044715554_i32,147163255_i32,252076042_i32,1002780218_i32,1264931166_i32,795933004_i32,1335091802_i32,(-1085671242_i32)];
_5 = (*RET) as f64;
(*_1) = [(-1888576332_i32),(-396999821_i32),400740234_i32,973102705_i32,1077669392_i32,1710361116_i32,(-1270886292_i32),1154681841_i32];
(*RET) = 108356358714802049726471703772856247931_u128 ^ 18161719125101214849487973501955529181_u128;
_9 = core::ptr::addr_of!((*RET));
_8.fld5 = 104_i8 as u64;
_8.fld0.4 = !_8.fld5;
(*_4) = [(-130134426_i32),(-1045817252_i32),534730782_i32,(-1027018359_i32),(-1191184432_i32),(-1108135197_i32),2001624206_i32,(-495431854_i32)];
_2 = [1545716043_i32,40765160_i32,(-1878078259_i32),(-1125400082_i32),1934694045_i32,(-923170984_i32),(-1679552892_i32),(-1107266895_i32)];
_9 = core::ptr::addr_of!((*_9));
(*RET) = 137531078932112153279997758591836133921_u128;
_8.fld3 = [(-8433251995556379165_i64),2651220267544059607_i64,8225528618069122315_i64,5895296706592608075_i64,(-6317892227841060296_i64),(-4795438419473214543_i64)];
_1 = core::ptr::addr_of!(_2);
_8.fld0.3 = 2054132751_i32 - (-1249711924_i32);
Goto(bb8)
}
bb14 = {
Return()
}
bb15 = {
_10 = _7;
_15 = core::ptr::addr_of!(_8.fld2);
_8.fld0.4 = !_8.fld5;
(*_3) = [(*_14).3,_8.fld0.3,(*_14).3,_8.fld0.3,(*_14).3,_8.fld0.3,(*_14).3,(*_14).3];
(*_14).0 = (*_14).1;
(*_15) = !70941202597394736691640583776689808869_u128;
_17 = !2734102614_u32;
_1 = core::ptr::addr_of!((*_3));
_16 = [(-3984159030362590140_i64),3655064075960396596_i64,3059495366076874113_i64,(-907016321016435581_i64)];
_16 = [(-4454689977661524401_i64),5591562023381615064_i64,3340938870023204365_i64,(-6314237635528164946_i64)];
_3 = core::ptr::addr_of!(_2);
(*_14).0 = _8.fld0.1;
(*_14).4 = _8.fld5;
_2 = [_8.fld0.3,(*_14).3,(*_14).3,_8.fld0.3,(*_14).3,(*_14).3,_8.fld0.3,(*_14).3];
(*_14).0 = !(*_14).1;
(*_14).2 = core::ptr::addr_of!(_8.fld5);
_8.fld1 = core::ptr::addr_of_mut!(_13);
_21.1 = (*_14).1;
(*RET) = !60347487636356274350813859259726756729_u128;
_8.fld4 = [4_usize,0_usize,3_usize,15311387447640773034_usize,7_usize,3_usize];
_19.0 = _12;
(*_14).3 = 1923797973_i32;
Goto(bb16)
}
bb16 = {
Call(_25 = dump_var(5_usize, 6_usize, Move(_6), 17_usize, Move(_17), 16_usize, Move(_16), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: i8,mut _2: [i64; 6],mut _3: (f64, [i32; 8]),mut _4: (f64, [i32; 8]),mut _5: Adt48,mut _6: *const [i32; 8],mut _7: u8,mut _8: Adt48,mut _9: [i32; 8],mut _10: isize,mut _11: [u8; 3]) -> usize {
mir! {
type RET = usize;
let _12: i16;
let _13: u16;
let _14: f32;
let _15: [i64; 6];
let _16: char;
let _17: bool;
let _18: isize;
let _19: (f64, [i32; 8]);
let _20: bool;
let _21: Adt48;
let _22: *const (bool, bool, *const u64, i32, u64);
let _23: *mut [char; 2];
let _24: (f64, [i32; 8]);
let _25: Adt54;
let _26: [i64; 6];
let _27: Adt46;
let _28: [i64; 6];
let _29: Adt51;
let _30: ([i32; 8], isize, (i8, bool));
let _31: [i32; 8];
let _32: Adt43;
let _33: char;
let _34: ();
let _35: ();
{
_11 = [_7,_7,_7];
RET = !16710714717402778634_usize;
_4.1 = [(-1050345254_i32),1166937213_i32,1796838488_i32,1562275711_i32,1058377886_i32,(-1649108162_i32),(-1799916843_i32),(-1754387297_i32)];
_9 = [(-320346234_i32),(-804392989_i32),309394454_i32,1359984071_i32,(-21233483_i32),(-1268244650_i32),1659471721_i32,(-1511167633_i32)];
_5.fld0 = (_3.0, _8.fld0.1);
_8.fld0.1 = [288975685_i32,(-938912147_i32),(-1796650636_i32),(-1396271798_i32),699522238_i32,(-1752186109_i32),1776074383_i32,856731892_i32];
_3 = (_8.fld0.0, (*_6));
RET = 3_usize & 6_usize;
_8.fld0 = _4;
_5.fld0.0 = _8.fld0.0 * _3.0;
_2 = [(-8793555039267009198_i64),5461202640729684210_i64,(-3081081376491825423_i64),(-8933384733255050540_i64),5248662629682491017_i64,(-6328511592685382462_i64)];
_4 = (_5.fld0.0, _9);
_11 = [_7,_7,_7];
_8.fld0.0 = _3.0 + _4.0;
_5.fld0 = (_3.0, _9);
RET = 3168791905982176301_usize >> _1;
_1 = 51_i8;
_1 = (-29_i8) ^ (-4_i8);
_2 = [3930356424511418063_i64,8860148333840666826_i64,2057913251354642317_i64,8383541350074833070_i64,8127159160790765868_i64,(-4103852027136521368_i64)];
_8 = _5;
_4.0 = _5.fld0.0 * _3.0;
_9 = [(-563974975_i32),(-483136767_i32),1442460206_i32,1774965663_i32,2133654665_i32,762396584_i32,(-1790725057_i32),846985261_i32];
_3.0 = _4.0;
_5.fld0.1 = _3.1;
RET = 16151028892477330787_usize + 7_usize;
_3.1 = (*_6);
Goto(bb1)
}
bb1 = {
_13 = 94937894133124503293580388668164013918_u128 as u16;
RET = 14235054924989049186_usize;
_2 = [(-5700590141845305775_i64),(-2123453246453174833_i64),4199977872566274353_i64,1101841167694665072_i64,(-1203888728876916598_i64),815613406154160676_i64];
_8.fld0 = _3;
_8.fld0.0 = (-4503262321618849338_i64) as f64;
(*_6) = [476975804_i32,(-1797987746_i32),(-922749844_i32),595369872_i32,876995591_i32,739652790_i32,282266210_i32,1205458799_i32];
_8.fld0 = (_5.fld0.0, _9);
_15 = _2;
_3 = (_4.0, _8.fld0.1);
_1 = !65_i8;
_4.1 = [1644942334_i32,(-89651706_i32),1657256354_i32,537291881_i32,(-1516786233_i32),1136472273_i32,(-1183796664_i32),(-268336944_i32)];
match RET {
0 => bb2,
14235054924989049186 => bb4,
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
(*_6) = [1065333745_i32,1892961489_i32,(-650243664_i32),(-54040746_i32),936611962_i32,(-339124403_i32),(-1041587217_i32),(-1830434959_i32)];
_13 = !4379_u16;
_4 = (_3.0, _8.fld0.1);
_12 = -(-28473_i16);
_5 = _8;
_5.fld0.0 = _4.0 - _4.0;
Call(_1 = fn7(_8, _9, _2, _8.fld0, _8.fld0.1, _3.1, _6), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_8.fld0 = (_3.0, (*_6));
RET = !0_usize;
_14 = _4.0 as f32;
_2 = [2228139784161055161_i64,(-8187851728898251142_i64),(-107271562536379083_i64),(-1986756522487605706_i64),2150991749155654522_i64,1693369152361912871_i64];
_4.0 = -_5.fld0.0;
(*_6) = [(-329428047_i32),1992707456_i32,964106542_i32,2020603677_i32,1580314896_i32,171675942_i32,(-973488151_i32),(-2051696105_i32)];
_15 = [(-6956409044924748202_i64),(-2440389745626739687_i64),497640919233672482_i64,(-5930476848556370438_i64),(-138657398568645109_i64),3791057827843567023_i64];
(*_6) = [(-1805761899_i32),1504025624_i32,1734478287_i32,597302899_i32,1917425750_i32,1155995825_i32,1304243969_i32,(-878617577_i32)];
_1 = 31_i8 ^ (-54_i8);
(*_6) = [(-354570795_i32),563898935_i32,1922887254_i32,1766855225_i32,(-1383476620_i32),1194357869_i32,(-1046196808_i32),(-979794544_i32)];
_10 = !92_isize;
(*_6) = [(-981281995_i32),580205051_i32,(-5036390_i32),733891941_i32,(-1143277127_i32),(-1082916179_i32),(-33232409_i32),1370831522_i32];
_9 = [(-277565915_i32),(-671973150_i32),(-1311573451_i32),1703035462_i32,(-582462090_i32),531102249_i32,1679801821_i32,482912760_i32];
_4 = _8.fld0;
(*_6) = [1434407719_i32,(-2098193984_i32),1042040994_i32,1059082104_i32,(-1974590675_i32),855417861_i32,1075199873_i32,238964085_i32];
_3.0 = -_4.0;
_3 = (_5.fld0.0, _9);
_8 = Adt48 { fld0: _3 };
_7 = _14 as u8;
_5.fld0 = _8.fld0;
_9 = [(-1699615292_i32),712451089_i32,(-1920663295_i32),(-1673417159_i32),757048129_i32,(-2026346798_i32),1324670987_i32,(-649788899_i32)];
_12 = 2853295269475713026_i64 as i16;
_3.1 = _4.1;
_8.fld0.1 = (*_6);
_2 = _15;
_14 = RET as f32;
_5.fld0.0 = _4.0 + _4.0;
_8.fld0 = _3;
_6 = core::ptr::addr_of!(_4.1);
Goto(bb6)
}
bb6 = {
_8.fld0 = (_5.fld0.0, (*_6));
_7 = 183_u8;
_4.0 = _8.fld0.0 - _5.fld0.0;
_5.fld0 = (_4.0, _3.1);
_10 = -(-86_isize);
_3.1 = [(-1175626451_i32),(-12493782_i32),1462988567_i32,99760545_i32,2062945016_i32,1269514410_i32,106540490_i32,(-2132278638_i32)];
_8.fld0 = _4;
_17 = _1 < _1;
_12 = 106151393503103081937442642495955841867_i128 as i16;
_21.fld0.0 = _3.0 - _5.fld0.0;
_18 = !_10;
_8.fld0.1 = [150935321_i32,(-990574372_i32),(-562348054_i32),1802282374_i32,393179468_i32,(-2014781650_i32),(-1407102737_i32),(-1027940805_i32)];
_21 = _8;
_5.fld0.1 = [754753452_i32,245705662_i32,(-411910586_i32),(-182971972_i32),13239716_i32,1799701668_i32,1064941355_i32,(-698619718_i32)];
_19 = _5.fld0;
(*_6) = [810692653_i32,708540792_i32,(-292217547_i32),(-564458822_i32),418797211_i32,(-602831927_i32),167613164_i32,428064429_i32];
_16 = '\u{4c659}';
match _7 {
0 => bb1,
1 => bb4,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
183 => bb12,
_ => bb11
}
}
bb7 = {
_8.fld0 = (_3.0, (*_6));
RET = !0_usize;
_14 = _4.0 as f32;
_2 = [2228139784161055161_i64,(-8187851728898251142_i64),(-107271562536379083_i64),(-1986756522487605706_i64),2150991749155654522_i64,1693369152361912871_i64];
_4.0 = -_5.fld0.0;
(*_6) = [(-329428047_i32),1992707456_i32,964106542_i32,2020603677_i32,1580314896_i32,171675942_i32,(-973488151_i32),(-2051696105_i32)];
_15 = [(-6956409044924748202_i64),(-2440389745626739687_i64),497640919233672482_i64,(-5930476848556370438_i64),(-138657398568645109_i64),3791057827843567023_i64];
(*_6) = [(-1805761899_i32),1504025624_i32,1734478287_i32,597302899_i32,1917425750_i32,1155995825_i32,1304243969_i32,(-878617577_i32)];
_1 = 31_i8 ^ (-54_i8);
(*_6) = [(-354570795_i32),563898935_i32,1922887254_i32,1766855225_i32,(-1383476620_i32),1194357869_i32,(-1046196808_i32),(-979794544_i32)];
_10 = !92_isize;
(*_6) = [(-981281995_i32),580205051_i32,(-5036390_i32),733891941_i32,(-1143277127_i32),(-1082916179_i32),(-33232409_i32),1370831522_i32];
_9 = [(-277565915_i32),(-671973150_i32),(-1311573451_i32),1703035462_i32,(-582462090_i32),531102249_i32,1679801821_i32,482912760_i32];
_4 = _8.fld0;
(*_6) = [1434407719_i32,(-2098193984_i32),1042040994_i32,1059082104_i32,(-1974590675_i32),855417861_i32,1075199873_i32,238964085_i32];
_3.0 = -_4.0;
_3 = (_5.fld0.0, _9);
_8 = Adt48 { fld0: _3 };
_7 = _14 as u8;
_5.fld0 = _8.fld0;
_9 = [(-1699615292_i32),712451089_i32,(-1920663295_i32),(-1673417159_i32),757048129_i32,(-2026346798_i32),1324670987_i32,(-649788899_i32)];
_12 = 2853295269475713026_i64 as i16;
_3.1 = _4.1;
_8.fld0.1 = (*_6);
_2 = _15;
_14 = RET as f32;
_5.fld0.0 = _4.0 + _4.0;
_8.fld0 = _3;
_6 = core::ptr::addr_of!(_4.1);
Goto(bb6)
}
bb8 = {
(*_6) = [1065333745_i32,1892961489_i32,(-650243664_i32),(-54040746_i32),936611962_i32,(-339124403_i32),(-1041587217_i32),(-1830434959_i32)];
_13 = !4379_u16;
_4 = (_3.0, _8.fld0.1);
_12 = -(-28473_i16);
_5 = _8;
_5.fld0.0 = _4.0 - _4.0;
Call(_1 = fn7(_8, _9, _2, _8.fld0, _8.fld0.1, _3.1, _6), ReturnTo(bb5), UnwindUnreachable())
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_13 = 94937894133124503293580388668164013918_u128 as u16;
RET = 14235054924989049186_usize;
_2 = [(-5700590141845305775_i64),(-2123453246453174833_i64),4199977872566274353_i64,1101841167694665072_i64,(-1203888728876916598_i64),815613406154160676_i64];
_8.fld0 = _3;
_8.fld0.0 = (-4503262321618849338_i64) as f64;
(*_6) = [476975804_i32,(-1797987746_i32),(-922749844_i32),595369872_i32,876995591_i32,739652790_i32,282266210_i32,1205458799_i32];
_8.fld0 = (_5.fld0.0, _9);
_15 = _2;
_3 = (_4.0, _8.fld0.1);
_1 = !65_i8;
_4.1 = [1644942334_i32,(-89651706_i32),1657256354_i32,537291881_i32,(-1516786233_i32),1136472273_i32,(-1183796664_i32),(-268336944_i32)];
match RET {
0 => bb2,
14235054924989049186 => bb4,
_ => bb3
}
}
bb12 = {
_6 = core::ptr::addr_of!(_19.1);
_8 = _5;
_5.fld0.1 = _9;
_21.fld0.0 = _4.0;
_17 = !false;
_8 = _21;
_2 = _15;
_1 = 105_i8;
(*_6) = [(-150043598_i32),(-860393043_i32),2025841781_i32,(-1620907631_i32),(-1181955314_i32),(-1968951881_i32),90711815_i32,1112031254_i32];
(*_6) = [(-1064613311_i32),2131434705_i32,373640934_i32,469038261_i32,455062249_i32,(-1965204441_i32),1785447487_i32,(-173368959_i32)];
_24.0 = _8.fld0.0 - _19.0;
_10 = 647420189852812030_i64 as isize;
_5 = Adt48 { fld0: _4 };
match _7 {
0 => bb1,
1 => bb10,
183 => bb14,
_ => bb13
}
}
bb13 = {
_8.fld0 = (_3.0, (*_6));
RET = !0_usize;
_14 = _4.0 as f32;
_2 = [2228139784161055161_i64,(-8187851728898251142_i64),(-107271562536379083_i64),(-1986756522487605706_i64),2150991749155654522_i64,1693369152361912871_i64];
_4.0 = -_5.fld0.0;
(*_6) = [(-329428047_i32),1992707456_i32,964106542_i32,2020603677_i32,1580314896_i32,171675942_i32,(-973488151_i32),(-2051696105_i32)];
_15 = [(-6956409044924748202_i64),(-2440389745626739687_i64),497640919233672482_i64,(-5930476848556370438_i64),(-138657398568645109_i64),3791057827843567023_i64];
(*_6) = [(-1805761899_i32),1504025624_i32,1734478287_i32,597302899_i32,1917425750_i32,1155995825_i32,1304243969_i32,(-878617577_i32)];
_1 = 31_i8 ^ (-54_i8);
(*_6) = [(-354570795_i32),563898935_i32,1922887254_i32,1766855225_i32,(-1383476620_i32),1194357869_i32,(-1046196808_i32),(-979794544_i32)];
_10 = !92_isize;
(*_6) = [(-981281995_i32),580205051_i32,(-5036390_i32),733891941_i32,(-1143277127_i32),(-1082916179_i32),(-33232409_i32),1370831522_i32];
_9 = [(-277565915_i32),(-671973150_i32),(-1311573451_i32),1703035462_i32,(-582462090_i32),531102249_i32,1679801821_i32,482912760_i32];
_4 = _8.fld0;
(*_6) = [1434407719_i32,(-2098193984_i32),1042040994_i32,1059082104_i32,(-1974590675_i32),855417861_i32,1075199873_i32,238964085_i32];
_3.0 = -_4.0;
_3 = (_5.fld0.0, _9);
_8 = Adt48 { fld0: _3 };
_7 = _14 as u8;
_5.fld0 = _8.fld0;
_9 = [(-1699615292_i32),712451089_i32,(-1920663295_i32),(-1673417159_i32),757048129_i32,(-2026346798_i32),1324670987_i32,(-649788899_i32)];
_12 = 2853295269475713026_i64 as i16;
_3.1 = _4.1;
_8.fld0.1 = (*_6);
_2 = _15;
_14 = RET as f32;
_5.fld0.0 = _4.0 + _4.0;
_8.fld0 = _3;
_6 = core::ptr::addr_of!(_4.1);
Goto(bb6)
}
bb14 = {
_9 = [(-1552541327_i32),(-1396723777_i32),(-646273190_i32),(-1554341217_i32),(-548520413_i32),(-38784698_i32),(-2084380151_i32),1095571925_i32];
_20 = !_17;
_19.0 = _21.fld0.0 - _4.0;
_5.fld0 = (_24.0, (*_6));
(*_6) = [821359700_i32,2055709725_i32,(-1888910914_i32),1161799156_i32,(-148994390_i32),(-518339637_i32),409622493_i32,(-892992703_i32)];
_18 = _10;
_5.fld0.0 = _12 as f64;
_5.fld0.0 = (-8989547935960173370_i64) as f64;
_5.fld0.0 = _19.0;
_29.fld2 = core::ptr::addr_of!(_23);
_13 = 4751_u16;
(*_6) = [88677425_i32,(-732157888_i32),(-1746679655_i32),(-1323926561_i32),(-1465927114_i32),332047296_i32,(-1292720147_i32),(-1181426128_i32)];
_28 = _15;
_29.fld0 = _17 | _20;
_17 = _29.fld0 | _29.fld0;
_17 = _21.fld0.0 >= _19.0;
_5.fld0.1 = [1641780103_i32,1975516346_i32,1502900588_i32,(-983959130_i32),(-2020041694_i32),(-1695752580_i32),(-352743374_i32),576735187_i32];
_29.fld2 = core::ptr::addr_of!(_23);
_29.fld5 = Adt49 { fld0: RET };
_3.1 = (*_6);
_16 = '\u{19e06}';
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(6_usize, 18_usize, Move(_18), 17_usize, Move(_17), 15_usize, Move(_15), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(6_usize, 28_usize, Move(_28), 9_usize, Move(_9), 11_usize, Move(_11), 35_usize, _35), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: Adt48,mut _2: [i32; 8],mut _3: [i64; 6],mut _4: (f64, [i32; 8]),mut _5: [i32; 8],mut _6: [i32; 8],mut _7: *const [i32; 8]) -> i8 {
mir! {
type RET = i8;
let _8: (*const u16, u32);
let _9: [i64; 6];
let _10: Adt48;
let _11: [i64; 6];
let _12: i128;
let _13: u64;
let _14: Adt48;
let _15: i16;
let _16: [u64; 5];
let _17: bool;
let _18: char;
let _19: Adt48;
let _20: i16;
let _21: Adt57;
let _22: Adt50;
let _23: Adt44;
let _24: isize;
let _25: u128;
let _26: *const u128;
let _27: Adt42;
let _28: u64;
let _29: f32;
let _30: [i32; 5];
let _31: [u8; 3];
let _32: [u8; 3];
let _33: f64;
let _34: isize;
let _35: bool;
let _36: ();
let _37: ();
{
_1.fld0 = (_4.0, (*_7));
_1.fld0 = _4;
_1 = Adt48 { fld0: _4 };
_3 = [(-6805865472790580738_i64),(-4415973848235041058_i64),(-7919592566656812071_i64),(-6445971074781612451_i64),1373583212901887428_i64,(-8235431362949966296_i64)];
_2 = (*_7);
_6 = _2;
_3 = [(-203596276665510039_i64),(-7941150612866456467_i64),3567711433257765130_i64,5695955058543089625_i64,4551576216675124668_i64,4769492612778274609_i64];
_8.1 = 3171875421_u32 & 46982175_u32;
RET = (-82_i8);
_9 = [(-4863774286765845321_i64),762367558045510903_i64,7966228874081846111_i64,6452921559122852198_i64,(-9058414664556983362_i64),8580129985797818255_i64];
_1 = Adt48 { fld0: _4 };
_5 = [(-1122455343_i32),(-2052162608_i32),1280156207_i32,(-1778996213_i32),(-2145086430_i32),2027020081_i32,(-1490189322_i32),(-44212000_i32)];
_1.fld0.1 = (*_7);
_5 = (*_7);
_1.fld0 = _4;
_1.fld0.1 = [939299243_i32,171330579_i32,1711955459_i32,830290126_i32,(-1754173432_i32),(-531165645_i32),(-1052683154_i32),354373163_i32];
_3 = _9;
_3 = [(-364532330022123073_i64),(-1169792941930685743_i64),(-2244356923475563760_i64),(-5686930792672282618_i64),2880626759914930107_i64,(-8700975038268225906_i64)];
Call(_4.1 = fn8((*_7), (*_7), _5, _1.fld0.1, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = _1.fld0;
_4.0 = -_1.fld0.0;
_4.0 = 58_u8 as f64;
(*_7) = [147380984_i32,(-648115876_i32),1957438483_i32,(-358344782_i32),(-322488623_i32),(-1775715746_i32),(-831861026_i32),1848924856_i32];
_10 = _1;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211374 => bb7,
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
_1.fld0.1 = [1629478506_i32,(-1608496336_i32),(-298312884_i32),(-517001140_i32),(-1006967720_i32),1743525087_i32,(-1405578887_i32),(-400450705_i32)];
_4.1 = [(-164141582_i32),(-567395865_i32),(-1725922279_i32),1112555963_i32,(-53813710_i32),(-768393264_i32),(-357654525_i32),(-42894781_i32)];
_7 = core::ptr::addr_of!(_5);
_1.fld0.1 = _2;
_6 = (*_7);
(*_7) = _4.1;
_3 = [(-7088146970252919491_i64),(-9138648496831804168_i64),(-7744775021589234379_i64),(-4017150423625121475_i64),9098570516653161499_i64,(-3197355647094096727_i64)];
_7 = core::ptr::addr_of!(_6);
_11 = _9;
(*_7) = [602364649_i32,2108110064_i32,(-1586478927_i32),1006119694_i32,(-12471057_i32),(-2129880329_i32),(-1573331607_i32),2084202606_i32];
_10.fld0 = (_4.0, _2);
_7 = core::ptr::addr_of!(_6);
_1.fld0 = (_4.0, (*_7));
_10.fld0 = _4;
_10.fld0 = (_4.0, (*_7));
_13 = true as u64;
_1.fld0.1 = [(-1426718590_i32),1555879319_i32,(-1671491270_i32),1738130945_i32,(-160681519_i32),1050008427_i32,2104227994_i32,(-355142945_i32)];
_2 = [(-1053253513_i32),(-1186757461_i32),(-2106242842_i32),(-496188019_i32),171890475_i32,(-1872872120_i32),(-281730769_i32),2074436492_i32];
_5 = _2;
Call(RET = core::intrinsics::bswap((-120_i8)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_8.1 = !1836817970_u32;
_4.1 = [375970570_i32,422763764_i32,958250701_i32,23341125_i32,(-224084758_i32),(-1150980108_i32),453280203_i32,(-1331407966_i32)];
_1.fld0 = (_4.0, _4.1);
_11 = [(-3840908058956204393_i64),2221297956234612335_i64,6052158917605766825_i64,(-8843847916959427670_i64),6209430618038555995_i64,(-518626390599948648_i64)];
_11 = [310239205410736405_i64,(-7881591862473899984_i64),699474612277640375_i64,3490361150736103314_i64,(-2636798207388562561_i64),(-1766144928183111392_i64)];
_4.0 = 4780249074895207412_usize as f64;
_1.fld0.0 = _4.0;
_2 = _1.fld0.1;
RET = 43_i8 | 7_i8;
_4.1 = [1681965069_i32,(-248832347_i32),(-904186977_i32),(-1117997586_i32),97021296_i32,(-1483566172_i32),1408464537_i32,(-1748254821_i32)];
_1.fld0 = (_10.fld0.0, (*_7));
_8.1 = RET as u32;
_15 = 5347_i16;
RET = 9223372036854775807_isize as i8;
_16 = [_13,_13,_13,_13,_13];
_14.fld0 = _1.fld0;
_12 = 26338262288764497229170209479263570336_i128 ^ 81615607524226798306286839606014803328_i128;
_12 = '\u{978e9}' as i128;
_2 = [831678884_i32,414167990_i32,1596763659_i32,(-1941209167_i32),(-270632147_i32),(-958075659_i32),459746040_i32,(-464125068_i32)];
(*_7) = [398564629_i32,2035152969_i32,1295556269_i32,934825811_i32,531962694_i32,(-1581100882_i32),(-1770981582_i32),(-995821972_i32)];
_4.0 = -_14.fld0.0;
Goto(bb9)
}
bb9 = {
(*_7) = [(-1026749919_i32),1053753152_i32,1485229964_i32,928426294_i32,(-1870889444_i32),799535391_i32,(-2126922350_i32),1082138496_i32];
_14.fld0 = _10.fld0;
RET = (-91_i8);
_18 = '\u{cc451}';
_18 = '\u{9425d}';
_1.fld0.0 = _10.fld0.0 * _10.fld0.0;
_4 = (_10.fld0.0, (*_7));
RET = _12 as i8;
_10.fld0 = (_4.0, _14.fld0.1);
_9 = [2034108161753641874_i64,(-5206993023334338644_i64),2742862050399197786_i64,(-6557487798006248239_i64),(-8499034268099355538_i64),(-295027884279819953_i64)];
_4.1 = [667164089_i32,(-2130820580_i32),1153226669_i32,(-695252980_i32),(-465141528_i32),1345103531_i32,(-1385021059_i32),379551872_i32];
_10.fld0.1 = [637537608_i32,(-1906081973_i32),1819784400_i32,812798793_i32,17016736_i32,(-1727196468_i32),(-76851121_i32),(-399349740_i32)];
_1.fld0 = (_10.fld0.0, _6);
_2 = [(-838665065_i32),(-659359076_i32),(-1357108819_i32),(-1497968381_i32),522455818_i32,2110059094_i32,510569989_i32,1338212545_i32];
_16 = [_13,_13,_13,_13,_13];
_4.0 = _1.fld0.0 - _1.fld0.0;
_10.fld0 = (_14.fld0.0, _6);
_19.fld0 = (_1.fld0.0, (*_7));
_14.fld0.0 = 5561_u16 as f64;
_19.fld0.0 = -_4.0;
_15 = _12 as i16;
_19 = Adt48 { fld0: _4 };
_9 = _11;
_3 = [2911200541039773629_i64,(-4718830181597327521_i64),317471911447876614_i64,(-5806180087327950769_i64),(-7415110158347376623_i64),(-765912508092223733_i64)];
Goto(bb10)
}
bb10 = {
_4.0 = -_19.fld0.0;
RET = 109_i8 + (-71_i8);
_17 = !true;
_6 = [(-1892492866_i32),1281162452_i32,2058411562_i32,(-1701634645_i32),(-1362966114_i32),(-282782540_i32),(-266967426_i32),1557709961_i32];
_14.fld0.0 = _4.0 + _19.fld0.0;
_17 = true;
_19.fld0.0 = 89_u8 as f64;
_14 = Adt48 { fld0: _4 };
_10 = _14;
_4 = _14.fld0;
_24 = (-104_isize) & 66_isize;
_10 = Adt48 { fld0: _4 };
_10 = Adt48 { fld0: _1.fld0 };
Goto(bb11)
}
bb11 = {
_14 = Adt48 { fld0: _4 };
_21 = Adt57::Variant3 { fld0: (-1742513495_i32) };
_14.fld0.0 = -_4.0;
_17 = _13 < _13;
_1.fld0.1 = [(-1656404101_i32),65279461_i32,(-127865876_i32),(-1319176172_i32),(-335148380_i32),(-1198990128_i32),(-917362897_i32),(-349638652_i32)];
place!(Field::<i32>(Variant(_21, 3), 0)) = !1812881633_i32;
_19.fld0.1 = [Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0)];
Goto(bb12)
}
bb12 = {
SetDiscriminant(_21, 3);
_24 = -9223372036854775807_isize;
_27.fld1.2.0 = !RET;
_6 = [1159953217_i32,(-6022857_i32),1895749516_i32,76002640_i32,(-1997115614_i32),(-1733986273_i32),1516246368_i32,1988903911_i32];
place!(Field::<i32>(Variant(_21, 3), 0)) = _15 as i32;
_27.fld1.2.0 = 11865920947454110658_usize as i8;
_27.fld1.2 = (RET, _17);
_14.fld0.0 = -_19.fld0.0;
_24 = 9223372036854775807_isize & (-9223372036854775808_isize);
place!(Field::<i32>(Variant(_21, 3), 0)) = (-1550494771_i32) ^ 1498288936_i32;
place!(Field::<i32>(Variant(_21, 3), 0)) = (-1323878715_i32) & 494032901_i32;
_2 = [Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0)];
_29 = _12 as f32;
_28 = !_13;
_3 = [1860660878845097649_i64,2731895592494493117_i64,2038816356094520952_i64,6750600596649175101_i64,(-5960293455369016731_i64),(-7838214102086682826_i64)];
_27.fld1.2 = Checked(RET - RET);
_27.fld0 = core::ptr::addr_of_mut!(_15);
_6 = [Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0),Field::<i32>(Variant(_21, 3), 0)];
SetDiscriminant(_21, 0);
_10.fld0.1 = [170106555_i32,2058899224_i32,454772171_i32,(-1248748971_i32),(-88306987_i32),1825881645_i32,254374007_i32,1745605294_i32];
_3 = _11;
_11 = [(-3669106196767311539_i64),(-4854626834255392091_i64),4603515332762063780_i64,(-6839215837634372584_i64),6522863586171898550_i64,2569254798637187997_i64];
place!(Field::<Adt51>(Variant(_21, 0), 4)).fld4 = _9;
_19.fld0.1 = [(-1567551835_i32),889276679_i32,1079397774_i32,(-998251232_i32),(-1786157042_i32),(-56490238_i32),(-101144092_i32),(-998837960_i32)];
Goto(bb13)
}
bb13 = {
_15 = 7_usize as i16;
_27.fld2 = [(-995897703270669165_i64),(-1324756043190279269_i64),6950390290608806993_i64,3469588626885617735_i64];
place!(Field::<i64>(Variant(_21, 0), 6)) = (-4601590417096445293_i64) | (-6190639121301597351_i64);
_14.fld0.0 = _10.fld0.0 * _4.0;
_14.fld0.1 = [(-490252241_i32),(-71703269_i32),(-2002626186_i32),54928086_i32,(-427177901_i32),(-1838770079_i32),(-169163608_i32),2015750079_i32];
(*_7) = _1.fld0.1;
_28 = !_13;
Goto(bb14)
}
bb14 = {
_24 = 105_isize >> _27.fld1.2.0;
place!(Field::<Adt51>(Variant(_21, 0), 4)).fld1 = [12_u8,149_u8,168_u8];
place!(Field::<f32>(Variant(_21, 0), 5)) = _29;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(7_usize, 24_usize, Move(_24), 12_usize, Move(_12), 3_usize, Move(_3), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(7_usize, 15_usize, Move(_15), 18_usize, Move(_18), 5_usize, Move(_5), 37_usize, _37), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [i32; 8],mut _2: [i32; 8],mut _3: [i32; 8],mut _4: [i32; 8],mut _5: [i64; 6]) -> [i32; 8] {
mir! {
type RET = [i32; 8];
let _6: isize;
let _7: f64;
let _8: [usize; 6];
let _9: char;
let _10: i64;
let _11: [i64; 4];
let _12: [i64; 6];
let _13: [i32; 8];
let _14: (i8, bool);
let _15: [i64; 4];
let _16: [i64; 4];
let _17: (i8, bool);
let _18: bool;
let _19: u16;
let _20: ();
let _21: ();
{
_5 = [(-8268157629007474839_i64),3702049054835463899_i64,(-7836644204762713837_i64),(-3799632902455409601_i64),3906215301988558363_i64,(-4781216199658611827_i64)];
_4 = [(-782365049_i32),(-1611545756_i32),(-1864570304_i32),(-1978035063_i32),(-2147226010_i32),(-808812577_i32),(-2076452765_i32),(-1467981476_i32)];
RET = [(-752006054_i32),1066545518_i32,(-1098583614_i32),904697113_i32,170913171_i32,136291806_i32,(-71273780_i32),1588853757_i32];
RET = [(-1686734158_i32),1853896696_i32,(-1465029796_i32),2074756073_i32,(-653653430_i32),(-287205923_i32),(-571948908_i32),(-1841658539_i32)];
_5 = [(-1189264412641668070_i64),157762523975900631_i64,(-4946715972421750448_i64),409313430450586747_i64,(-2045654150685617644_i64),7059182001537535710_i64];
_3 = [(-1748282501_i32),1912965387_i32,(-1862016324_i32),(-1650095267_i32),(-130485106_i32),294355754_i32,1598884057_i32,710272087_i32];
_4 = [(-786714418_i32),1816233871_i32,542370771_i32,2116941618_i32,425559292_i32,2040989953_i32,1034094468_i32,(-1178702684_i32)];
RET = [1421276273_i32,1799588180_i32,219926486_i32,(-495359688_i32),(-274092238_i32),(-1816629684_i32),1282703604_i32,157593166_i32];
_3 = [647893200_i32,949792130_i32,632772881_i32,(-1232488_i32),(-387958667_i32),912101633_i32,707039627_i32,(-1506437007_i32)];
_4 = _1;
_2 = _3;
_3 = [503908783_i32,(-620349780_i32),1649722243_i32,1596621650_i32,(-754317949_i32),(-904972593_i32),430076258_i32,1859184726_i32];
Goto(bb1)
}
bb1 = {
_4 = _3;
_2 = _1;
RET = _2;
_7 = 9223372036854775807_isize as f64;
_6 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_6 = (-9223372036854775808_isize);
_4 = RET;
_4 = [(-2089461796_i32),1676264770_i32,(-1765501620_i32),136035218_i32,(-193361663_i32),(-449973687_i32),1193087160_i32,(-385578984_i32)];
Call(_2 = fn9(RET, _1, _1, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = [1749538072_i32,(-1787181697_i32),1991547356_i32,1358248700_i32,770635491_i32,1528149192_i32,799027012_i32,(-1222481288_i32)];
RET = _1;
_8 = [16285693624158606350_usize,15329871766226132388_usize,1432223718619376645_usize,7970972450076391926_usize,6_usize,7912052600981318747_usize];
_1 = [86206781_i32,884283455_i32,224508707_i32,1221238867_i32,(-622312560_i32),(-1056676278_i32),(-2021779300_i32),179812186_i32];
_2 = [795756003_i32,(-1160540700_i32),(-634018352_i32),(-151397559_i32),(-986351394_i32),(-1908699130_i32),1558234456_i32,(-295182787_i32)];
_2 = [(-950579017_i32),879999164_i32,1609233092_i32,1371748611_i32,1237843974_i32,(-1142462086_i32),(-840800038_i32),1599496539_i32];
_7 = 243335272463261804_u64 as f64;
_10 = !2019751505376479102_i64;
_9 = '\u{6c731}';
_5 = [_10,_10,_10,_10,_10,_10];
_9 = '\u{de53a}';
RET = _4;
_3 = RET;
_1 = [1042472000_i32,(-838369350_i32),(-112156851_i32),(-81123846_i32),106057191_i32,(-1478962004_i32),438407750_i32,1549634210_i32];
_2 = _1;
_8 = [8761514597216079113_usize,3_usize,7_usize,4_usize,1_usize,3_usize];
RET = [266450299_i32,(-795854540_i32),17373785_i32,(-1079638539_i32),1958273237_i32,(-1191839754_i32),1574759167_i32,(-1550230428_i32)];
_5 = [_10,_10,_10,_10,_10,_10];
_1 = _2;
_5 = [_10,_10,_10,_10,_10,_10];
RET = [1445152063_i32,(-1153252513_i32),1460512883_i32,865704210_i32,414726200_i32,332656437_i32,(-1500593542_i32),811514466_i32];
RET = [(-1476078270_i32),553600357_i32,1528771631_i32,(-853019461_i32),1753662611_i32,(-216852969_i32),786755398_i32,(-342184474_i32)];
_6 = 85_isize << _10;
_4 = [(-282252700_i32),(-1237453107_i32),(-2019070826_i32),314649201_i32,(-1080428388_i32),1657153266_i32,2041996644_i32,1975111188_i32];
_10 = !(-780548255708301423_i64);
_4 = _1;
_6 = 9223372036854775807_isize;
match _6 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
9223372036854775807 => bb8,
_ => bb7
}
}
bb3 = {
_4 = _3;
_2 = _1;
RET = _2;
_7 = 9223372036854775807_isize as f64;
_6 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_6 = (-9223372036854775808_isize);
_4 = RET;
_4 = [(-2089461796_i32),1676264770_i32,(-1765501620_i32),136035218_i32,(-193361663_i32),(-449973687_i32),1193087160_i32,(-385578984_i32)];
Call(_2 = fn9(RET, _1, _1, _3), ReturnTo(bb2), UnwindUnreachable())
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
_3 = RET;
_1 = [649508124_i32,(-1520389796_i32),1117930112_i32,2044518337_i32,(-1583669164_i32),(-497676917_i32),1382334457_i32,(-196650942_i32)];
RET = [1585628068_i32,(-2101503486_i32),(-1628898085_i32),(-397848559_i32),414627924_i32,493812358_i32,(-701518349_i32),861663011_i32];
_1 = RET;
_7 = (-77_i8) as f64;
_2 = [1783806752_i32,(-2064886415_i32),(-355049530_i32),(-678353318_i32),(-329036766_i32),1324712053_i32,(-321307830_i32),1197979533_i32];
_2 = [(-1080446008_i32),92754318_i32,1167281098_i32,(-1705269130_i32),(-1570946245_i32),1420580781_i32,849837317_i32,1248482107_i32];
_7 = 8734_u16 as f64;
_10 = _6 as i64;
_2 = _1;
_10 = 54_u8 as i64;
Goto(bb9)
}
bb9 = {
_1 = [1319069537_i32,(-1268818624_i32),(-196696356_i32),(-1799298249_i32),673676055_i32,(-1801352013_i32),(-82911262_i32),1864881522_i32];
_11 = [_10,_10,_10,_10];
_9 = '\u{c03bc}';
_11 = [_10,_10,_10,_10];
_1 = [(-296329181_i32),702754805_i32,(-1234927497_i32),(-1357469783_i32),(-540123505_i32),1915168378_i32,1301931336_i32,(-719579384_i32)];
_8 = [6632604576982742060_usize,5_usize,0_usize,6075465824506228648_usize,2761123717580235175_usize,8011836220796948859_usize];
_2 = RET;
_2 = [844107409_i32,723404618_i32,(-798775727_i32),56544132_i32,(-1333214898_i32),348856481_i32,(-1821892103_i32),(-37200771_i32)];
RET = _2;
RET = [1272028403_i32,982442021_i32,1187441439_i32,1077101978_i32,724727776_i32,(-441954800_i32),(-2019620082_i32),1463400041_i32];
RET = _1;
_8 = [2767576422816929994_usize,6_usize,7579927974372347240_usize,12336694495473748795_usize,1085000297474797560_usize,17099036731428304227_usize];
_2 = [(-290528315_i32),(-2114612440_i32),1678014013_i32,1545187338_i32,(-614217409_i32),617429158_i32,1767058490_i32,791659135_i32];
_4 = [656040872_i32,(-1844237617_i32),690616628_i32,436816578_i32,(-2096059218_i32),1199842742_i32,(-212671499_i32),416363762_i32];
_11 = [_10,_10,_10,_10];
RET = [48147865_i32,974127549_i32,(-1059997478_i32),(-1104980275_i32),1081447178_i32,2136390293_i32,(-524868091_i32),546329577_i32];
match _6 {
0 => bb1,
1 => bb8,
9223372036854775807 => bb10,
_ => bb4
}
}
bb10 = {
_6 = 9223372036854775807_isize;
match _6 {
0 => bb1,
1 => bb6,
2 => bb11,
9223372036854775807 => bb13,
_ => bb12
}
}
bb11 = {
RET = [1749538072_i32,(-1787181697_i32),1991547356_i32,1358248700_i32,770635491_i32,1528149192_i32,799027012_i32,(-1222481288_i32)];
RET = _1;
_8 = [16285693624158606350_usize,15329871766226132388_usize,1432223718619376645_usize,7970972450076391926_usize,6_usize,7912052600981318747_usize];
_1 = [86206781_i32,884283455_i32,224508707_i32,1221238867_i32,(-622312560_i32),(-1056676278_i32),(-2021779300_i32),179812186_i32];
_2 = [795756003_i32,(-1160540700_i32),(-634018352_i32),(-151397559_i32),(-986351394_i32),(-1908699130_i32),1558234456_i32,(-295182787_i32)];
_2 = [(-950579017_i32),879999164_i32,1609233092_i32,1371748611_i32,1237843974_i32,(-1142462086_i32),(-840800038_i32),1599496539_i32];
_7 = 243335272463261804_u64 as f64;
_10 = !2019751505376479102_i64;
_9 = '\u{6c731}';
_5 = [_10,_10,_10,_10,_10,_10];
_9 = '\u{de53a}';
RET = _4;
_3 = RET;
_1 = [1042472000_i32,(-838369350_i32),(-112156851_i32),(-81123846_i32),106057191_i32,(-1478962004_i32),438407750_i32,1549634210_i32];
_2 = _1;
_8 = [8761514597216079113_usize,3_usize,7_usize,4_usize,1_usize,3_usize];
RET = [266450299_i32,(-795854540_i32),17373785_i32,(-1079638539_i32),1958273237_i32,(-1191839754_i32),1574759167_i32,(-1550230428_i32)];
_5 = [_10,_10,_10,_10,_10,_10];
_1 = _2;
_5 = [_10,_10,_10,_10,_10,_10];
RET = [1445152063_i32,(-1153252513_i32),1460512883_i32,865704210_i32,414726200_i32,332656437_i32,(-1500593542_i32),811514466_i32];
RET = [(-1476078270_i32),553600357_i32,1528771631_i32,(-853019461_i32),1753662611_i32,(-216852969_i32),786755398_i32,(-342184474_i32)];
_6 = 85_isize << _10;
_4 = [(-282252700_i32),(-1237453107_i32),(-2019070826_i32),314649201_i32,(-1080428388_i32),1657153266_i32,2041996644_i32,1975111188_i32];
_10 = !(-780548255708301423_i64);
_4 = _1;
_6 = 9223372036854775807_isize;
match _6 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
9223372036854775807 => bb8,
_ => bb7
}
}
bb12 = {
_4 = _3;
_2 = _1;
RET = _2;
_7 = 9223372036854775807_isize as f64;
_6 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_6 = (-9223372036854775808_isize);
_4 = RET;
_4 = [(-2089461796_i32),1676264770_i32,(-1765501620_i32),136035218_i32,(-193361663_i32),(-449973687_i32),1193087160_i32,(-385578984_i32)];
Call(_2 = fn9(RET, _1, _1, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_5 = [_10,_10,_10,_10,_10,_10];
_3 = _1;
_1 = [(-1989026281_i32),1643704568_i32,(-142231278_i32),(-1326958685_i32),(-655359526_i32),(-292623305_i32),(-1291307678_i32),150766038_i32];
_13 = _2;
match _6 {
9223372036854775807 => bb15,
_ => bb14
}
}
bb14 = {
RET = [1749538072_i32,(-1787181697_i32),1991547356_i32,1358248700_i32,770635491_i32,1528149192_i32,799027012_i32,(-1222481288_i32)];
RET = _1;
_8 = [16285693624158606350_usize,15329871766226132388_usize,1432223718619376645_usize,7970972450076391926_usize,6_usize,7912052600981318747_usize];
_1 = [86206781_i32,884283455_i32,224508707_i32,1221238867_i32,(-622312560_i32),(-1056676278_i32),(-2021779300_i32),179812186_i32];
_2 = [795756003_i32,(-1160540700_i32),(-634018352_i32),(-151397559_i32),(-986351394_i32),(-1908699130_i32),1558234456_i32,(-295182787_i32)];
_2 = [(-950579017_i32),879999164_i32,1609233092_i32,1371748611_i32,1237843974_i32,(-1142462086_i32),(-840800038_i32),1599496539_i32];
_7 = 243335272463261804_u64 as f64;
_10 = !2019751505376479102_i64;
_9 = '\u{6c731}';
_5 = [_10,_10,_10,_10,_10,_10];
_9 = '\u{de53a}';
RET = _4;
_3 = RET;
_1 = [1042472000_i32,(-838369350_i32),(-112156851_i32),(-81123846_i32),106057191_i32,(-1478962004_i32),438407750_i32,1549634210_i32];
_2 = _1;
_8 = [8761514597216079113_usize,3_usize,7_usize,4_usize,1_usize,3_usize];
RET = [266450299_i32,(-795854540_i32),17373785_i32,(-1079638539_i32),1958273237_i32,(-1191839754_i32),1574759167_i32,(-1550230428_i32)];
_5 = [_10,_10,_10,_10,_10,_10];
_1 = _2;
_5 = [_10,_10,_10,_10,_10,_10];
RET = [1445152063_i32,(-1153252513_i32),1460512883_i32,865704210_i32,414726200_i32,332656437_i32,(-1500593542_i32),811514466_i32];
RET = [(-1476078270_i32),553600357_i32,1528771631_i32,(-853019461_i32),1753662611_i32,(-216852969_i32),786755398_i32,(-342184474_i32)];
_6 = 85_isize << _10;
_4 = [(-282252700_i32),(-1237453107_i32),(-2019070826_i32),314649201_i32,(-1080428388_i32),1657153266_i32,2041996644_i32,1975111188_i32];
_10 = !(-780548255708301423_i64);
_4 = _1;
_6 = 9223372036854775807_isize;
match _6 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
9223372036854775807 => bb8,
_ => bb7
}
}
bb15 = {
_2 = _1;
RET = _4;
RET = _4;
_6 = (-82_i8) as isize;
_11 = [_10,_10,_10,_10];
_5 = [_10,_10,_10,_10,_10,_10];
_10 = 2624958369358491892_i64;
RET = _4;
_14 = (22_i8, false);
_3 = [1303734135_i32,1296174897_i32,509240701_i32,(-2074595327_i32),545448758_i32,(-271559219_i32),778285817_i32,1439961619_i32];
RET = [(-2040496493_i32),(-881654714_i32),(-473273635_i32),(-638522470_i32),1871708827_i32,(-1547685403_i32),(-1067992405_i32),1983313043_i32];
_1 = RET;
_10 = _9 as i64;
_11 = [_10,_10,_10,_10];
_5 = [_10,_10,_10,_10,_10,_10];
_5 = [_10,_10,_10,_10,_10,_10];
_5 = [_10,_10,_10,_10,_10,_10];
_17.1 = _14.0 > _14.0;
_17.0 = _7 as i8;
_15 = [_10,_10,_10,_10];
_12 = [_10,_10,_10,_10,_10,_10];
_18 = _17.1 == _17.1;
Goto(bb16)
}
bb16 = {
Call(_20 = dump_var(8_usize, 8_usize, Move(_8), 3_usize, Move(_3), 6_usize, Move(_6), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_20 = dump_var(8_usize, 13_usize, Move(_13), 2_usize, Move(_2), 9_usize, Move(_9), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [i32; 8],mut _2: [i32; 8],mut _3: [i32; 8],mut _4: [i32; 8]) -> [i32; 8] {
mir! {
type RET = [i32; 8];
let _5: char;
let _6: [i64; 6];
let _7: Adt43;
let _8: [u64; 5];
let _9: [u64; 5];
let _10: char;
let _11: bool;
let _12: [usize; 6];
let _13: i32;
let _14: [char; 2];
let _15: Adt49;
let _16: isize;
let _17: *const u16;
let _18: f32;
let _19: ();
let _20: ();
{
_4 = [(-422841327_i32),1252382422_i32,(-1140134870_i32),(-285554849_i32),210484194_i32,(-828605601_i32),1445259461_i32,910232571_i32];
RET = [(-560565843_i32),893798089_i32,(-700436865_i32),407716205_i32,(-2124825363_i32),1321506201_i32,1710145468_i32,1253841789_i32];
_5 = '\u{c7655}';
_5 = '\u{39a41}';
RET = _3;
_2 = [1029999740_i32,1411026288_i32,1314876598_i32,1183506511_i32,(-2101706517_i32),726456041_i32,1228555106_i32,(-1075321001_i32)];
_3 = RET;
_4 = _1;
_5 = '\u{5912b}';
RET = _4;
_6 = [2357043059666988921_i64,563353583025375806_i64,3796899585721804853_i64,7994841362675045153_i64,(-3840502207554938954_i64),(-3592165472510630040_i64)];
RET = [1190601401_i32,(-379065409_i32),718801187_i32,(-894819920_i32),923089192_i32,(-297764786_i32),(-1085027858_i32),(-1140297105_i32)];
_6 = [(-7262757134664970961_i64),577997257696813805_i64,5594249870802603089_i64,(-7312990295053444983_i64),(-8328054814821302109_i64),5195554856665524114_i64];
_3 = [(-492026359_i32),(-509970785_i32),(-29573837_i32),(-1935114565_i32),616317312_i32,200317350_i32,1255956745_i32,270338243_i32];
Goto(bb1)
}
bb1 = {
_6 = [5581725202711117898_i64,(-5027332551484615321_i64),(-7574010407886620542_i64),(-2347000276691559276_i64),(-2576419467351915040_i64),(-439113122427596069_i64)];
_4 = [783170896_i32,800559310_i32,(-1726240037_i32),261997157_i32,136821983_i32,(-66949814_i32),(-460901156_i32),(-960709904_i32)];
_3 = [73643163_i32,(-665337350_i32),(-1870856651_i32),1701407399_i32,(-86661851_i32),(-1149557648_i32),(-986733800_i32),1534227334_i32];
RET = [1663571251_i32,(-944651454_i32),1070558102_i32,(-27072420_i32),(-542316496_i32),(-1716680472_i32),1169106864_i32,541400417_i32];
_2 = [(-879628124_i32),2044790361_i32,(-1687973994_i32),(-442493329_i32),(-327646437_i32),(-794342533_i32),954841220_i32,991030397_i32];
RET = [1063403411_i32,(-14022987_i32),141527672_i32,(-58571467_i32),(-667385579_i32),1717248587_i32,294479140_i32,1653425999_i32];
RET = _2;
Call(RET = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = [797719670_i32,425694493_i32,(-1664890274_i32),(-537196858_i32),(-1871350653_i32),(-2109027431_i32),(-196777407_i32),(-768224045_i32)];
_5 = '\u{fd803}';
_2 = [(-1203761375_i32),(-939271247_i32),(-573888040_i32),1472286704_i32,56531179_i32,(-1313573760_i32),51435546_i32,(-57639086_i32)];
_2 = [(-300508912_i32),1503708551_i32,(-405864417_i32),1655138781_i32,1183580679_i32,1460923623_i32,(-65201442_i32),958436111_i32];
_5 = '\u{2feda}';
_3 = [2044359684_i32,(-2019556518_i32),(-1111527241_i32),(-1363277044_i32),1751830572_i32,1861807843_i32,(-415751828_i32),720973652_i32];
_3 = _4;
_5 = '\u{8fa77}';
_8 = [2731917393149838987_u64,6485687440764946622_u64,11720680063094069236_u64,10456210558465994631_u64,12790417856008124636_u64];
_6 = [8618569383361430004_i64,2523644823958403051_i64,(-8859969508714917541_i64),(-191756365342367842_i64),320300899372153699_i64,5421691698123512078_i64];
_7.fld1 = [_5,_5];
_5 = '\u{983c9}';
Goto(bb3)
}
bb3 = {
_9 = [7775281968254525456_u64,972259898735153859_u64,16267728783919392584_u64,12304966018158898708_u64,5191875991515921592_u64];
_2 = _1;
_8 = [9533301686207693851_u64,13142965136836340239_u64,1606982136477192030_u64,9431140886670179963_u64,14669234104838915268_u64];
_9 = _8;
Goto(bb4)
}
bb4 = {
_10 = _5;
RET = _2;
_9 = _8;
_1 = _4;
_4 = [1760243321_i32,127662973_i32,26677200_i32,1722529374_i32,(-745809070_i32),576737724_i32,(-619507524_i32),688847431_i32];
_11 = _5 <= _10;
_4 = [(-969701993_i32),718868470_i32,(-767942260_i32),1060955232_i32,(-388281070_i32),258728578_i32,(-838831093_i32),(-330424803_i32)];
_4 = _2;
_11 = true;
_8 = _9;
RET = [(-1984861258_i32),418408789_i32,(-1293348050_i32),1779620056_i32,(-1579859553_i32),(-1019969380_i32),132620285_i32,1996206134_i32];
_3 = [1987077211_i32,312106696_i32,(-285867792_i32),1844088720_i32,2125672552_i32,(-2083215149_i32),1239416733_i32,533399265_i32];
_9 = [13431688458201549245_u64,8802489626586787677_u64,10896360938621829_u64,12097826187551918485_u64,12680339982598320156_u64];
Goto(bb5)
}
bb5 = {
_2 = [796584759_i32,(-2037422492_i32),2132974737_i32,(-976295914_i32),2134861169_i32,1404600060_i32,2125077998_i32,84056523_i32];
Call(_13 = fn10(_2, RET, _2, _1, RET), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
RET = [_13,_13,_13,_13,_13,_13,_13,_13];
_3 = [_13,_13,_13,_13,_13,_13,_13,_13];
match _13 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
1824990569 => bb13,
_ => bb12
}
}
bb7 = {
_2 = [796584759_i32,(-2037422492_i32),2132974737_i32,(-976295914_i32),2134861169_i32,1404600060_i32,2125077998_i32,84056523_i32];
Call(_13 = fn10(_2, RET, _2, _1, RET), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_10 = _5;
RET = _2;
_9 = _8;
_1 = _4;
_4 = [1760243321_i32,127662973_i32,26677200_i32,1722529374_i32,(-745809070_i32),576737724_i32,(-619507524_i32),688847431_i32];
_11 = _5 <= _10;
_4 = [(-969701993_i32),718868470_i32,(-767942260_i32),1060955232_i32,(-388281070_i32),258728578_i32,(-838831093_i32),(-330424803_i32)];
_4 = _2;
_11 = true;
_8 = _9;
RET = [(-1984861258_i32),418408789_i32,(-1293348050_i32),1779620056_i32,(-1579859553_i32),(-1019969380_i32),132620285_i32,1996206134_i32];
_3 = [1987077211_i32,312106696_i32,(-285867792_i32),1844088720_i32,2125672552_i32,(-2083215149_i32),1239416733_i32,533399265_i32];
_9 = [13431688458201549245_u64,8802489626586787677_u64,10896360938621829_u64,12097826187551918485_u64,12680339982598320156_u64];
Goto(bb5)
}
bb9 = {
_9 = [7775281968254525456_u64,972259898735153859_u64,16267728783919392584_u64,12304966018158898708_u64,5191875991515921592_u64];
_2 = _1;
_8 = [9533301686207693851_u64,13142965136836340239_u64,1606982136477192030_u64,9431140886670179963_u64,14669234104838915268_u64];
_9 = _8;
Goto(bb4)
}
bb10 = {
_4 = [797719670_i32,425694493_i32,(-1664890274_i32),(-537196858_i32),(-1871350653_i32),(-2109027431_i32),(-196777407_i32),(-768224045_i32)];
_5 = '\u{fd803}';
_2 = [(-1203761375_i32),(-939271247_i32),(-573888040_i32),1472286704_i32,56531179_i32,(-1313573760_i32),51435546_i32,(-57639086_i32)];
_2 = [(-300508912_i32),1503708551_i32,(-405864417_i32),1655138781_i32,1183580679_i32,1460923623_i32,(-65201442_i32),958436111_i32];
_5 = '\u{2feda}';
_3 = [2044359684_i32,(-2019556518_i32),(-1111527241_i32),(-1363277044_i32),1751830572_i32,1861807843_i32,(-415751828_i32),720973652_i32];
_3 = _4;
_5 = '\u{8fa77}';
_8 = [2731917393149838987_u64,6485687440764946622_u64,11720680063094069236_u64,10456210558465994631_u64,12790417856008124636_u64];
_6 = [8618569383361430004_i64,2523644823958403051_i64,(-8859969508714917541_i64),(-191756365342367842_i64),320300899372153699_i64,5421691698123512078_i64];
_7.fld1 = [_5,_5];
_5 = '\u{983c9}';
Goto(bb3)
}
bb11 = {
_6 = [5581725202711117898_i64,(-5027332551484615321_i64),(-7574010407886620542_i64),(-2347000276691559276_i64),(-2576419467351915040_i64),(-439113122427596069_i64)];
_4 = [783170896_i32,800559310_i32,(-1726240037_i32),261997157_i32,136821983_i32,(-66949814_i32),(-460901156_i32),(-960709904_i32)];
_3 = [73643163_i32,(-665337350_i32),(-1870856651_i32),1701407399_i32,(-86661851_i32),(-1149557648_i32),(-986733800_i32),1534227334_i32];
RET = [1663571251_i32,(-944651454_i32),1070558102_i32,(-27072420_i32),(-542316496_i32),(-1716680472_i32),1169106864_i32,541400417_i32];
_2 = [(-879628124_i32),2044790361_i32,(-1687973994_i32),(-442493329_i32),(-327646437_i32),(-794342533_i32),954841220_i32,991030397_i32];
RET = [1063403411_i32,(-14022987_i32),141527672_i32,(-58571467_i32),(-667385579_i32),1717248587_i32,294479140_i32,1653425999_i32];
RET = _2;
Call(RET = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
RET = [_13,_13,_13,_13,_13,_13,_13,_13];
RET = [_13,_13,_13,_13,_13,_13,_13,_13];
_8 = _9;
_1 = [_13,_13,_13,_13,_13,_13,_13,_13];
_10 = _5;
_7.fld1 = [_5,_5];
_12 = [7309527899710785618_usize,13469138868233293720_usize,5046381363110872054_usize,7_usize,1_usize,1743143931138505158_usize];
RET = [_13,_13,_13,_13,_13,_13,_13,_13];
_6 = [858173575503586159_i64,(-6072790444916722081_i64),(-9051922362847429461_i64),(-5093080486790690839_i64),8297805359369756398_i64,(-5777102957903847994_i64)];
_6 = [(-7208261532798666647_i64),(-4452778183633403591_i64),1008076571609162099_i64,(-2700231686651422986_i64),(-6068693561855086920_i64),8221847922725309493_i64];
_3 = [_13,_13,_13,_13,_13,_13,_13,_13];
_15.fld0 = 12508257928203394015_u64 as usize;
_15.fld0 = (-21782_i16) as usize;
_3 = [_13,_13,_13,_13,_13,_13,_13,_13];
_5 = _10;
_14 = _7.fld1;
Goto(bb14)
}
bb14 = {
_11 = true ^ true;
_6 = [(-4546886588639308433_i64),3791240209956960116_i64,(-4456272378688104059_i64),(-8155099423595661853_i64),(-7953853709812243891_i64),7699397004612294051_i64];
_18 = 1216871125_u32 as f32;
_1 = [_13,_13,_13,_13,_13,_13,_13,_13];
RET = [_13,_13,_13,_13,_13,_13,_13,_13];
_15.fld0 = 4_usize;
_3 = [_13,_13,_13,_13,_13,_13,_13,_13];
_6 = [263190232255463114_i64,8091870620581418318_i64,(-1874732423057429905_i64),(-6220049492614612189_i64),7617144654530161744_i64,7979419462842485929_i64];
_1 = [_13,_13,_13,_13,_13,_13,_13,_13];
Goto(bb15)
}
bb15 = {
Call(_19 = dump_var(9_usize, 2_usize, Move(_2), 5_usize, Move(_5), 3_usize, Move(_3), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_19 = dump_var(9_usize, 8_usize, Move(_8), 6_usize, Move(_6), 20_usize, _20, 20_usize, _20), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: [i32; 8],mut _2: [i32; 8],mut _3: [i32; 8],mut _4: [i32; 8],mut _5: [i32; 8]) -> i32 {
mir! {
type RET = i32;
let _6: f32;
let _7: (f64, [i32; 8]);
let _8: *const (bool, bool, *const u64, i32, u64);
let _9: Adt44;
let _10: (f64, [i32; 8]);
let _11: u32;
let _12: isize;
let _13: Adt46;
let _14: [u64; 5];
let _15: u64;
let _16: [i64; 4];
let _17: u32;
let _18: [char; 2];
let _19: bool;
let _20: f64;
let _21: (i8, bool);
let _22: Adt47;
let _23: [i64; 4];
let _24: bool;
let _25: u64;
let _26: [i64; 4];
let _27: [u8; 3];
let _28: u64;
let _29: f64;
let _30: *mut [char; 2];
let _31: [i64; 6];
let _32: u16;
let _33: [u64; 5];
let _34: Adt55;
let _35: *const (bool, bool, *const u64, i32, u64);
let _36: char;
let _37: isize;
let _38: f64;
let _39: ();
let _40: ();
{
RET = !(-1785121514_i32);
_3 = _4;
_6 = (-22667_i16) as f32;
_6 = 11680930115615398505_u64 as f32;
_3 = _5;
RET = 1244121404_i32;
_3 = [RET,RET,RET,RET,RET,RET,RET,RET];
_2 = [RET,RET,RET,RET,RET,RET,RET,RET];
_7.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
RET = -(-498068598_i32);
_10.0 = _6 as f64;
RET = 842032855_i32 << 31184548481708250006575302802026413902_i128;
_10.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_6 = 4470305004991753135_u64 as f32;
_2 = [RET,RET,RET,RET,RET,RET,RET,RET];
_7.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
Call(_12 = fn11(_4, RET, _10.1, _4, _4, _4, _7.1, _5, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = 83298643846183232536787528354488870895_u128 as f32;
_7 = (_10.0, _4);
_7 = _10;
_2 = [RET,RET,RET,RET,RET,RET,RET,RET];
_7.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
RET = !(-1465114873_i32);
Goto(bb2)
}
bb2 = {
_6 = 26358_u16 as f32;
_5 = [RET,RET,RET,RET,RET,RET,RET,RET];
_3 = [RET,RET,RET,RET,RET,RET,RET,RET];
_11 = 682030735_u32;
RET = -(-1751353810_i32);
_2 = [RET,RET,RET,RET,RET,RET,RET,RET];
_7 = _10;
_10.0 = _7.0 + _7.0;
_1 = _4;
_7.0 = -_10.0;
_10.0 = _7.0 * _7.0;
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
_5 = [RET,RET,RET,RET,RET,RET,RET,RET];
_15 = 769359897239896211_u64 - 14016445066796221675_u64;
_12 = 66979713425161762777149052274931871225_u128 as isize;
_14 = [_15,_15,_15,_15,_15];
_1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_3 = _10.1;
_3 = [RET,RET,RET,RET,RET,RET,RET,RET];
_11 = !1689517896_u32;
_1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_10.0 = _7.0;
_3 = _1;
_10.1 = _3;
_17 = 142463035196925119020477946076148608850_i128 as u32;
Call(_6 = fn12(_11, _7.1, _10, _12, _10.0, _7, _3, _7, _17, _10.0, _10.0, _3, _10.1, _11, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = (_10.0, _2);
_10.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_16 = [(-4675822254605969144_i64),(-1394883612968459665_i64),440767072312743454_i64,8154741018219873291_i64];
_10.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_12 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_7.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_7.0 = _10.0 - _10.0;
_10 = (_7.0, _3);
_14 = [_15,_15,_15,_15,_15];
_10.0 = 176_u8 as f64;
_1 = _3;
_7 = (_10.0, _2);
_5 = _7.1;
_18 = ['\u{b0cc}','\u{96604}'];
Goto(bb4)
}
bb4 = {
_4 = _5;
_21.0 = (-112_i8);
_12 = (-112_isize) * 17_isize;
_10.1 = _2;
_19 = !true;
_22 = Adt47::Variant1 { fld0: _19,fld1: RET };
_16 = [(-6762489168302205510_i64),2876118162713712369_i64,2735730408883385582_i64,5943877781314931314_i64];
match _21.0 {
0 => bb1,
1 => bb2,
340282366920938463463374607431768211344 => bb5,
_ => bb3
}
}
bb5 = {
_12 = 9223372036854775807_isize & 9223372036854775807_isize;
_10.1 = [Field::<i32>(Variant(_22, 1), 1),RET,Field::<i32>(Variant(_22, 1), 1),RET,RET,Field::<i32>(Variant(_22, 1), 1),Field::<i32>(Variant(_22, 1), 1),Field::<i32>(Variant(_22, 1), 1)];
_7.1 = [RET,RET,RET,Field::<i32>(Variant(_22, 1), 1),Field::<i32>(Variant(_22, 1), 1),RET,Field::<i32>(Variant(_22, 1), 1),RET];
_4 = [Field::<i32>(Variant(_22, 1), 1),Field::<i32>(Variant(_22, 1), 1),Field::<i32>(Variant(_22, 1), 1),Field::<i32>(Variant(_22, 1), 1),RET,Field::<i32>(Variant(_22, 1), 1),RET,RET];
_21 = (2_i8, Field::<bool>(Variant(_22, 1), 0));
_10 = _7;
RET = Field::<i32>(Variant(_22, 1), 1) + Field::<i32>(Variant(_22, 1), 1);
_16 = [(-9160739504539574441_i64),(-3309225497893115901_i64),2114503978521664574_i64,7404132114223825727_i64];
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
_7.1 = _5;
RET = !Field::<i32>(Variant(_22, 1), 1);
_10.1 = _4;
_1 = [RET,Field::<i32>(Variant(_22, 1), 1),Field::<i32>(Variant(_22, 1), 1),RET,RET,Field::<i32>(Variant(_22, 1), 1),Field::<i32>(Variant(_22, 1), 1),RET];
_25 = _15;
_22 = Adt47::Variant1 { fld0: _19,fld1: RET };
_1 = [RET,Field::<i32>(Variant(_22, 1), 1),RET,RET,Field::<i32>(Variant(_22, 1), 1),Field::<i32>(Variant(_22, 1), 1),RET,RET];
_18 = ['\u{4cfa4}','\u{d866c}'];
_21 = (18_i8, _19);
_21 = (120_i8, _19);
_2 = [Field::<i32>(Variant(_22, 1), 1),RET,Field::<i32>(Variant(_22, 1), 1),RET,Field::<i32>(Variant(_22, 1), 1),Field::<i32>(Variant(_22, 1), 1),RET,RET];
_19 = _11 < _17;
_15 = !_25;
match _21.0 {
0 => bb6,
1 => bb7,
2 => bb8,
120 => bb10,
_ => bb9
}
}
bb6 = {
_4 = _5;
_21.0 = (-112_i8);
_12 = (-112_isize) * 17_isize;
_10.1 = _2;
_19 = !true;
_22 = Adt47::Variant1 { fld0: _19,fld1: RET };
_16 = [(-6762489168302205510_i64),2876118162713712369_i64,2735730408883385582_i64,5943877781314931314_i64];
match _21.0 {
0 => bb1,
1 => bb2,
340282366920938463463374607431768211344 => bb5,
_ => bb3
}
}
bb7 = {
_7 = (_10.0, _2);
_10.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_16 = [(-4675822254605969144_i64),(-1394883612968459665_i64),440767072312743454_i64,8154741018219873291_i64];
_10.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_12 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_7.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_7.0 = _10.0 - _10.0;
_10 = (_7.0, _3);
_14 = [_15,_15,_15,_15,_15];
_10.0 = 176_u8 as f64;
_1 = _3;
_7 = (_10.0, _2);
_5 = _7.1;
_18 = ['\u{b0cc}','\u{96604}'];
Goto(bb4)
}
bb8 = {
_6 = 26358_u16 as f32;
_5 = [RET,RET,RET,RET,RET,RET,RET,RET];
_3 = [RET,RET,RET,RET,RET,RET,RET,RET];
_11 = 682030735_u32;
RET = -(-1751353810_i32);
_2 = [RET,RET,RET,RET,RET,RET,RET,RET];
_7 = _10;
_10.0 = _7.0 + _7.0;
_1 = _4;
_7.0 = -_10.0;
_10.0 = _7.0 * _7.0;
_4 = [RET,RET,RET,RET,RET,RET,RET,RET];
_5 = [RET,RET,RET,RET,RET,RET,RET,RET];
_15 = 769359897239896211_u64 - 14016445066796221675_u64;
_12 = 66979713425161762777149052274931871225_u128 as isize;
_14 = [_15,_15,_15,_15,_15];
_1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_3 = _10.1;
_3 = [RET,RET,RET,RET,RET,RET,RET,RET];
_11 = !1689517896_u32;
_1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_10.0 = _7.0;
_3 = _1;
_10.1 = _3;
_17 = 142463035196925119020477946076148608850_i128 as u32;
Call(_6 = fn12(_11, _7.1, _10, _12, _10.0, _7, _3, _7, _17, _10.0, _10.0, _3, _10.1, _11, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_6 = 83298643846183232536787528354488870895_u128 as f32;
_7 = (_10.0, _4);
_7 = _10;
_2 = [RET,RET,RET,RET,RET,RET,RET,RET];
_7.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
RET = !(-1465114873_i32);
Goto(bb2)
}
bb10 = {
_10.1 = [Field::<i32>(Variant(_22, 1), 1),Field::<i32>(Variant(_22, 1), 1),Field::<i32>(Variant(_22, 1), 1),Field::<i32>(Variant(_22, 1), 1),Field::<i32>(Variant(_22, 1), 1),RET,Field::<i32>(Variant(_22, 1), 1),RET];
place!(Field::<i32>(Variant(_22, 1), 1)) = RET | RET;
_14 = [_25,_15,_25,_25,_15];
SetDiscriminant(_22, 0);
_21.1 = _19;
_28 = !_15;
_20 = (-1093918048893708032_i64) as f64;
_18 = ['\u{107677}','\u{40382}'];
place!(Field::<isize>(Variant(_22, 0), 1)) = _12 - _12;
_7.0 = _20;
_21.0 = (-71_i8);
_22 = Adt47::Variant1 { fld0: _21.1,fld1: RET };
_20 = _7.0 - _10.0;
_10 = (_20, _4);
_21 = ((-44_i8), Field::<bool>(Variant(_22, 1), 0));
_14 = [_15,_28,_28,_28,_15];
_14 = [_28,_28,_25,_15,_15];
_3 = [Field::<i32>(Variant(_22, 1), 1),Field::<i32>(Variant(_22, 1), 1),Field::<i32>(Variant(_22, 1), 1),Field::<i32>(Variant(_22, 1), 1),RET,Field::<i32>(Variant(_22, 1), 1),RET,RET];
_6 = 308956691964041841419308118381250294397_u128 as f32;
_26 = _16;
RET = Field::<i32>(Variant(_22, 1), 1) ^ Field::<i32>(Variant(_22, 1), 1);
RET = Field::<i32>(Variant(_22, 1), 1);
_10.1 = _3;
place!(Field::<i32>(Variant(_22, 1), 1)) = RET;
_10.0 = _7.0 + _20;
Goto(bb11)
}
bb11 = {
_25 = (-6738327450043063890_i64) as u64;
_6 = Field::<i32>(Variant(_22, 1), 1) as f32;
_20 = -_10.0;
_27 = [90_u8,173_u8,175_u8];
_4 = _7.1;
Call(_17 = core::intrinsics::transmute(RET), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_30 = core::ptr::addr_of_mut!(_18);
SetDiscriminant(_22, 3);
place!(Field::<isize>(Variant(_22, 3), 2)) = _12;
_34.fld0.1 = !_21.1;
_34.fld1 = core::ptr::addr_of_mut!((*_30));
_34.fld0.1 = _12 <= _12;
match _21.0 {
0 => bb5,
1 => bb10,
340282366920938463463374607431768211412 => bb13,
_ => bb3
}
}
bb13 = {
_3 = [RET,RET,RET,RET,RET,RET,RET,RET];
_34.fld3 = [3738256784233348469_i64,(-4800838548345987102_i64),(-956299025565602450_i64),3743710582033196034_i64,(-2419625550025064001_i64),3157035309074163807_i64];
RET = -(-1467465019_i32);
RET = 1824990569_i32;
_5 = [RET,RET,RET,RET,RET,RET,RET,RET];
_5 = [RET,RET,RET,RET,RET,RET,RET,RET];
_28 = _15 + _25;
_33 = [_25,_28,_28,_28,_15];
match _21.0 {
0 => bb1,
1 => bb10,
2 => bb11,
340282366920938463463374607431768211412 => bb15,
_ => bb14
}
}
bb14 = {
_7 = (_10.0, _2);
_10.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_16 = [(-4675822254605969144_i64),(-1394883612968459665_i64),440767072312743454_i64,8154741018219873291_i64];
_10.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_12 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_7.1 = [RET,RET,RET,RET,RET,RET,RET,RET];
_7.0 = _10.0 - _10.0;
_10 = (_7.0, _3);
_14 = [_15,_15,_15,_15,_15];
_10.0 = 176_u8 as f64;
_1 = _3;
_7 = (_10.0, _2);
_5 = _7.1;
_18 = ['\u{b0cc}','\u{96604}'];
Goto(bb4)
}
bb15 = {
_21.0 = 34_i8;
_20 = _10.0 + _7.0;
_32 = 15612_u16;
place!(Field::<*const u128>(Variant(_22, 3), 1)) = core::ptr::addr_of!(_34.fld2);
_23 = _26;
_34.fld2 = 3575054899387050848_i64 as u128;
_3 = _10.1;
_18 = ['\u{41813}','\u{3c75d}'];
(*_30) = ['\u{e8c45}','\u{7facc}'];
_22 = Adt47::Variant0 { fld0: _34.fld1,fld1: _12 };
_34.fld0.3 = RET;
_14 = [_28,_28,_15,_15,_15];
_34.fld0.4 = _25 ^ _25;
_10.1 = [_34.fld0.3,_34.fld0.3,_34.fld0.3,_34.fld0.3,_34.fld0.3,RET,RET,RET];
_34.fld3 = [(-1675902100548836134_i64),104105072687915955_i64,4740704826814866853_i64,2889739428022730639_i64,(-905953734754498644_i64),(-4595575598580207910_i64)];
Goto(bb16)
}
bb16 = {
Call(_39 = dump_var(10_usize, 1_usize, Move(_1), 33_usize, Move(_33), 19_usize, Move(_19), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(10_usize, 26_usize, Move(_26), 15_usize, Move(_15), 17_usize, Move(_17), 11_usize, Move(_11)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_39 = dump_var(10_usize, 21_usize, Move(_21), 2_usize, Move(_2), 40_usize, _40, 40_usize, _40), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [i32; 8],mut _2: i32,mut _3: [i32; 8],mut _4: [i32; 8],mut _5: [i32; 8],mut _6: [i32; 8],mut _7: [i32; 8],mut _8: [i32; 8],mut _9: i32) -> isize {
mir! {
type RET = isize;
let _10: char;
let _11: u64;
let _12: Adt44;
let _13: i32;
let _14: [i32; 5];
let _15: i8;
let _16: u128;
let _17: (i8, bool);
let _18: f32;
let _19: ([i32; 8], isize, (i8, bool));
let _20: (f32,);
let _21: Adt48;
let _22: i32;
let _23: Adt49;
let _24: u16;
let _25: *const u64;
let _26: char;
let _27: Adt49;
let _28: Adt53;
let _29: (f32,);
let _30: i128;
let _31: i64;
let _32: bool;
let _33: isize;
let _34: isize;
let _35: isize;
let _36: Adt53;
let _37: Adt43;
let _38: ();
let _39: ();
{
_2 = !_9;
_2 = !_9;
RET = 99647800325005630797884317835973753387_i128 as isize;
_10 = '\u{c9f75}';
_9 = _2;
_5 = _8;
_3 = _1;
_1 = [_9,_9,_9,_2,_9,_2,_2,_2];
_11 = !4796534899261211740_u64;
_5 = [_9,_9,_9,_9,_2,_9,_2,_9];
_3 = [_2,_9,_9,_9,_9,_2,_2,_9];
RET = (-9223372036854775808_isize);
RET = (-63_isize);
_2 = _9 >> _9;
RET = 9223372036854775807_isize - (-9223372036854775808_isize);
_8 = [_2,_2,_2,_2,_9,_2,_2,_2];
RET = -(-4_isize);
_2 = -_9;
_6 = _3;
_7 = [_9,_2,_9,_2,_9,_9,_2,_2];
Goto(bb1)
}
bb1 = {
_5 = _8;
_5 = [_2,_2,_2,_2,_2,_9,_2,_9];
_14 = [_2,_9,_2,_2,_2];
_7 = [_2,_2,_2,_9,_2,_9,_9,_9];
_1 = [_9,_9,_9,_2,_9,_9,_2,_9];
_13 = _9 & _2;
_5 = [_13,_13,_13,_13,_9,_2,_9,_2];
RET = (-9223372036854775808_isize) * 9223372036854775807_isize;
_1 = _8;
_1 = _8;
_6 = [_13,_13,_13,_13,_9,_13,_9,_2];
RET = !103_isize;
_5 = [_2,_13,_9,_2,_9,_13,_9,_9];
_11 = (-26622443901817521324829813094609324657_i128) as u64;
_1 = _5;
Goto(bb2)
}
bb2 = {
_6 = _8;
_15 = 119_i8 | 13_i8;
RET = 67_isize >> _13;
_4 = _5;
_10 = '\u{39f01}';
_13 = -_9;
_15 = (-107_i8);
_11 = !15989647424773124715_u64;
_10 = '\u{bac50}';
_6 = [_13,_9,_9,_2,_13,_2,_13,_13];
_6 = [_2,_9,_9,_9,_2,_2,_2,_13];
_9 = _2 | _13;
_4 = [_2,_13,_9,_13,_13,_13,_9,_9];
_13 = 4_usize as i32;
_17.1 = false;
_4 = [_2,_2,_2,_2,_9,_13,_9,_2];
_10 = '\u{ce06c}';
_17.0 = _15 * _15;
_16 = 24303_u16 as u128;
match _15 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768211349 => bb8,
_ => bb7
}
}
bb3 = {
_5 = _8;
_5 = [_2,_2,_2,_2,_2,_9,_2,_9];
_14 = [_2,_9,_2,_2,_2];
_7 = [_2,_2,_2,_9,_2,_9,_9,_9];
_1 = [_9,_9,_9,_2,_9,_9,_2,_9];
_13 = _9 & _2;
_5 = [_13,_13,_13,_13,_9,_2,_9,_2];
RET = (-9223372036854775808_isize) * 9223372036854775807_isize;
_1 = _8;
_1 = _8;
_6 = [_13,_13,_13,_13,_9,_13,_9,_2];
RET = !103_isize;
_5 = [_2,_13,_9,_2,_9,_13,_9,_9];
_11 = (-26622443901817521324829813094609324657_i128) as u64;
_1 = _5;
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
RET = 9223372036854775807_isize;
_9 = -_2;
_13 = _2;
_8 = [_2,_2,_2,_13,_2,_13,_9,_9];
RET = -(-5_isize);
_2 = _13;
_18 = _13 as f32;
_13 = _10 as i32;
_17 = Checked(_15 + _15);
_19.1 = RET;
_19.0 = [_2,_2,_9,_2,_9,_2,_9,_13];
_14 = [_9,_2,_13,_2,_13];
_5 = [_2,_2,_2,_2,_2,_13,_2,_13];
_4 = [_9,_9,_13,_2,_9,_2,_9,_13];
_7 = _1;
_14 = [_9,_13,_2,_2,_9];
_21.fld0.1 = _1;
_4 = [_13,_2,_2,_9,_2,_9,_2,_2];
_20 = (_18,);
_20.0 = _18;
_21.fld0.1 = _7;
_1 = [_13,_9,_13,_9,_13,_9,_13,_2];
_19.1 = 958846064257673370_usize as isize;
Call(_13 = core::intrinsics::transmute(_10), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_22 = 221_u8 as i32;
_10 = '\u{14585}';
match _15 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463463374607431768211349 => bb10,
_ => bb8
}
}
bb10 = {
_19 = (_4, RET, _17);
RET = (-4196505927573831309_i64) as isize;
RET = _11 as isize;
_3 = _1;
_19.1 = RET;
RET = _19.1;
_20.0 = _17.0 as f32;
_19 = (_5, RET, _17);
_21.fld0.0 = _11 as f64;
_11 = 12500603205294096566_u64 & 11998145095443007036_u64;
_25 = core::ptr::addr_of!(_11);
_26 = _10;
_19.1 = RET;
_23 = Adt49 { fld0: 7847415806298025048_usize };
_17.0 = _19.2.0 & _15;
_17.0 = _15;
_21.fld0.0 = _19.2.0 as f64;
_19.2.1 = !_17.1;
_22 = -_9;
_23 = Adt49 { fld0: 0_usize };
_22 = -_9;
_21.fld0.0 = 3351983986_u32 as f64;
_3 = [_2,_22,_9,_2,_9,_13,_2,_2];
_23 = Adt49 { fld0: 5_usize };
_10 = _26;
(*_25) = !10914771523475874698_u64;
_9 = !_2;
_18 = -_20.0;
_23 = Adt49 { fld0: 1_usize };
match _17.0 {
0 => bb2,
340282366920938463463374607431768211349 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_11 = 12434292052530347672_u64;
match _23.fld0 {
0 => bb7,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
6 => bb18,
1 => bb20,
_ => bb19
}
}
bb13 = {
Return()
}
bb14 = {
_19 = (_4, RET, _17);
RET = (-4196505927573831309_i64) as isize;
RET = _11 as isize;
_3 = _1;
_19.1 = RET;
RET = _19.1;
_20.0 = _17.0 as f32;
_19 = (_5, RET, _17);
_21.fld0.0 = _11 as f64;
_11 = 12500603205294096566_u64 & 11998145095443007036_u64;
_25 = core::ptr::addr_of!(_11);
_26 = _10;
_19.1 = RET;
_23 = Adt49 { fld0: 7847415806298025048_usize };
_17.0 = _19.2.0 & _15;
_17.0 = _15;
_21.fld0.0 = _19.2.0 as f64;
_19.2.1 = !_17.1;
_22 = -_9;
_23 = Adt49 { fld0: 0_usize };
_22 = -_9;
_21.fld0.0 = 3351983986_u32 as f64;
_3 = [_2,_22,_9,_2,_9,_13,_2,_2];
_23 = Adt49 { fld0: 5_usize };
_10 = _26;
(*_25) = !10914771523475874698_u64;
_9 = !_2;
_18 = -_20.0;
_23 = Adt49 { fld0: 1_usize };
match _17.0 {
0 => bb2,
340282366920938463463374607431768211349 => bb12,
_ => bb11
}
}
bb15 = {
_22 = 221_u8 as i32;
_10 = '\u{14585}';
match _15 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463463374607431768211349 => bb10,
_ => bb8
}
}
bb16 = {
RET = 9223372036854775807_isize;
_9 = -_2;
_13 = _2;
_8 = [_2,_2,_2,_13,_2,_13,_9,_9];
RET = -(-5_isize);
_2 = _13;
_18 = _13 as f32;
_13 = _10 as i32;
_17 = Checked(_15 + _15);
_19.1 = RET;
_19.0 = [_2,_2,_9,_2,_9,_2,_9,_13];
_14 = [_9,_2,_13,_2,_13];
_5 = [_2,_2,_2,_2,_2,_13,_2,_13];
_4 = [_9,_9,_13,_2,_9,_2,_9,_13];
_7 = _1;
_14 = [_9,_13,_2,_2,_9];
_21.fld0.1 = _1;
_4 = [_13,_2,_2,_9,_2,_9,_2,_2];
_20 = (_18,);
_20.0 = _18;
_21.fld0.1 = _7;
_1 = [_13,_9,_13,_9,_13,_9,_13,_2];
_19.1 = 958846064257673370_usize as isize;
Call(_13 = core::intrinsics::transmute(_10), ReturnTo(bb9), UnwindUnreachable())
}
bb17 = {
Return()
}
bb18 = {
_5 = _8;
_5 = [_2,_2,_2,_2,_2,_9,_2,_9];
_14 = [_2,_9,_2,_2,_2];
_7 = [_2,_2,_2,_9,_2,_9,_9,_9];
_1 = [_9,_9,_9,_2,_9,_9,_2,_9];
_13 = _9 & _2;
_5 = [_13,_13,_13,_13,_9,_2,_9,_2];
RET = (-9223372036854775808_isize) * 9223372036854775807_isize;
_1 = _8;
_1 = _8;
_6 = [_13,_13,_13,_13,_9,_13,_9,_2];
RET = !103_isize;
_5 = [_2,_13,_9,_2,_9,_13,_9,_9];
_11 = (-26622443901817521324829813094609324657_i128) as u64;
_1 = _5;
Goto(bb2)
}
bb19 = {
_5 = _8;
_5 = [_2,_2,_2,_2,_2,_9,_2,_9];
_14 = [_2,_9,_2,_2,_2];
_7 = [_2,_2,_2,_9,_2,_9,_9,_9];
_1 = [_9,_9,_9,_2,_9,_9,_2,_9];
_13 = _9 & _2;
_5 = [_13,_13,_13,_13,_9,_2,_9,_2];
RET = (-9223372036854775808_isize) * 9223372036854775807_isize;
_1 = _8;
_1 = _8;
_6 = [_13,_13,_13,_13,_9,_13,_9,_2];
RET = !103_isize;
_5 = [_2,_13,_9,_2,_9,_13,_9,_9];
_11 = (-26622443901817521324829813094609324657_i128) as u64;
_1 = _5;
Goto(bb2)
}
bb20 = {
_1 = [_22,_22,_2,_22,_2,_2,_9,_2];
_18 = _20.0 - _20.0;
_23.fld0 = 0_usize;
_17.0 = _19.2.0 - _19.2.0;
RET = _19.1 + _19.1;
_24 = 5213_u16;
_19.2.0 = !_17.0;
_17.1 = !_19.2.1;
_19.2.1 = !_17.1;
_32 = _23.fld0 > _23.fld0;
_11 = 12962161687297845519_u64;
_17 = (_19.2.0, _32);
_4 = [_22,_9,_22,_2,_2,_22,_22,_9];
_22 = 224_u8 as i32;
(*_25) = 12122378555123367566_u64;
_32 = _17.1 ^ _17.1;
_25 = core::ptr::addr_of!(_11);
_9 = _13 - _2;
_25 = core::ptr::addr_of!((*_25));
_19.2.1 = _2 != _2;
Goto(bb21)
}
bb21 = {
Call(_38 = dump_var(11_usize, 13_usize, Move(_13), 5_usize, Move(_5), 10_usize, Move(_10), 17_usize, Move(_17)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_38 = dump_var(11_usize, 7_usize, Move(_7), 22_usize, Move(_22), 26_usize, Move(_26), 14_usize, Move(_14)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_38 = dump_var(11_usize, 3_usize, Move(_3), 1_usize, Move(_1), 39_usize, _39, 39_usize, _39), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: u32,mut _2: [i32; 8],mut _3: (f64, [i32; 8]),mut _4: isize,mut _5: f64,mut _6: (f64, [i32; 8]),mut _7: [i32; 8],mut _8: (f64, [i32; 8]),mut _9: u32,mut _10: f64,mut _11: f64,mut _12: [i32; 8],mut _13: [i32; 8],mut _14: u32,mut _15: [i32; 8]) -> f32 {
mir! {
type RET = f32;
let _16: u8;
let _17: isize;
let _18: i32;
let _19: [u8; 3];
let _20: isize;
let _21: char;
let _22: Adt43;
let _23: Adt47;
let _24: [i32; 8];
let _25: Adt52;
let _26: [u8; 3];
let _27: f64;
let _28: [i32; 5];
let _29: usize;
let _30: i64;
let _31: [u64; 5];
let _32: [i64; 6];
let _33: f64;
let _34: isize;
let _35: isize;
let _36: bool;
let _37: f32;
let _38: *mut [char; 2];
let _39: [char; 2];
let _40: isize;
let _41: Adt49;
let _42: Adt58;
let _43: i32;
let _44: char;
let _45: Adt42;
let _46: *const u64;
let _47: bool;
let _48: (f64, [i32; 8]);
let _49: char;
let _50: i64;
let _51: [char; 2];
let _52: i8;
let _53: f64;
let _54: ();
let _55: ();
{
_3.0 = 3204282150589877243_usize as f64;
_5 = 2006000548_i32 as f64;
_6 = _8;
_17 = _4;
_6.1 = [(-951597113_i32),(-297191952_i32),(-1862958927_i32),(-515089221_i32),1360913205_i32,(-1529705632_i32),(-1616164552_i32),(-7686546_i32)];
_18 = !(-1928641151_i32);
RET = 22176_u16 as f32;
_12 = _8.1;
_13 = _6.1;
_8 = (_10, _13);
_17 = -_4;
_3 = (_11, _8.1);
_8.1 = [_18,_18,_18,_18,_18,_18,_18,_18];
_6.1 = _13;
_6.1 = [_18,_18,_18,_18,_18,_18,_18,_18];
_19 = [135_u8,185_u8,123_u8];
_14 = !_9;
RET = _4 as f32;
_16 = 235_u8;
_4 = !_17;
_14 = _9;
_16 = 238_u8;
_15 = _2;
_8.1 = [_18,_18,_18,_18,_18,_18,_18,_18];
Call(_4 = fn13(_7, _12, _6, _15, _13, _8.1, RET, _6.0, _19, _13, _3, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_18 = 18611_i16 as i32;
_20 = -_4;
_1 = _14 | _9;
_9 = 654466069734942185_i64 as u32;
_15 = [_18,_18,_18,_18,_18,_18,_18,_18];
_6 = _3;
_4 = _20 ^ _20;
_3 = _8;
_9 = !_1;
_12 = [_18,_18,_18,_18,_18,_18,_18,_18];
_18 = 60782686_i32;
_14 = _1;
_8.1 = [_18,_18,_18,_18,_18,_18,_18,_18];
_4 = -_20;
_19 = [_16,_16,_16];
_3 = (_6.0, _13);
RET = 18079_i16 as f32;
_1 = !_14;
_6.0 = _11 - _10;
_18 = _20 as i32;
_16 = _14 as u8;
_11 = (-3927130296508346864_i64) as f64;
_8.1 = [_18,_18,_18,_18,_18,_18,_18,_18];
_3 = (_6.0, _13);
Goto(bb2)
}
bb2 = {
_14 = false as u32;
_12 = _6.1;
_1 = 20032_i16 as u32;
_10 = _3.0;
_22.fld1 = ['\u{3f439}','\u{a2b76}'];
_21 = '\u{a1d51}';
_20 = -_4;
_8.1 = [_18,_18,_18,_18,_18,_18,_18,_18];
_25.fld2 = 154099332610389531321582585116966221840_u128;
RET = 8198633472071594365_u64 as f32;
_15 = _8.1;
_25.fld3 = !67_i8;
_25.fld6 = !_16;
_25.fld1.fld1.2 = Checked(_25.fld3 + _25.fld3);
_25.fld6 = _16 >> _20;
_24 = [_18,_18,_18,_18,_18,_18,_18,_18];
_25.fld6 = _16 ^ _16;
_5 = _3.0;
_26 = [_16,_16,_25.fld6];
_25.fld1.fld1.0 = [_18,_18,_18,_18,_18,_18,_18,_18];
_3 = (_5, _25.fld1.fld1.0);
_22.fld0 = core::ptr::addr_of!(_25.fld2);
_25.fld0 = _8.0 + _10;
Goto(bb3)
}
bb3 = {
_4 = _20;
_14 = _9;
_25.fld1.fld1.0 = [_18,_18,_18,_18,_18,_18,_18,_18];
_8.1 = _24;
_30 = 1506266702809949851_i64;
match _25.fld2 {
0 => bb1,
1 => bb4,
2 => bb5,
154099332610389531321582585116966221840 => bb7,
_ => bb6
}
}
bb4 = {
_14 = false as u32;
_12 = _6.1;
_1 = 20032_i16 as u32;
_10 = _3.0;
_22.fld1 = ['\u{3f439}','\u{a2b76}'];
_21 = '\u{a1d51}';
_20 = -_4;
_8.1 = [_18,_18,_18,_18,_18,_18,_18,_18];
_25.fld2 = 154099332610389531321582585116966221840_u128;
RET = 8198633472071594365_u64 as f32;
_15 = _8.1;
_25.fld3 = !67_i8;
_25.fld6 = !_16;
_25.fld1.fld1.2 = Checked(_25.fld3 + _25.fld3);
_25.fld6 = _16 >> _20;
_24 = [_18,_18,_18,_18,_18,_18,_18,_18];
_25.fld6 = _16 ^ _16;
_5 = _3.0;
_26 = [_16,_16,_25.fld6];
_25.fld1.fld1.0 = [_18,_18,_18,_18,_18,_18,_18,_18];
_3 = (_5, _25.fld1.fld1.0);
_22.fld0 = core::ptr::addr_of!(_25.fld2);
_25.fld0 = _8.0 + _10;
Goto(bb3)
}
bb5 = {
_18 = 18611_i16 as i32;
_20 = -_4;
_1 = _14 | _9;
_9 = 654466069734942185_i64 as u32;
_15 = [_18,_18,_18,_18,_18,_18,_18,_18];
_6 = _3;
_4 = _20 ^ _20;
_3 = _8;
_9 = !_1;
_12 = [_18,_18,_18,_18,_18,_18,_18,_18];
_18 = 60782686_i32;
_14 = _1;
_8.1 = [_18,_18,_18,_18,_18,_18,_18,_18];
_4 = -_20;
_19 = [_16,_16,_16];
_3 = (_6.0, _13);
RET = 18079_i16 as f32;
_1 = !_14;
_6.0 = _11 - _10;
_18 = _20 as i32;
_16 = _14 as u8;
_11 = (-3927130296508346864_i64) as f64;
_8.1 = [_18,_18,_18,_18,_18,_18,_18,_18];
_3 = (_6.0, _13);
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_29 = !5_usize;
_8.1 = [_18,_18,_18,_18,_18,_18,_18,_18];
_22.fld1 = [_21,_21];
_8 = (_5, _12);
_32 = [_30,_30,_30,_30,_30,_30];
_26 = [_25.fld6,_25.fld6,_16];
_6.0 = -_25.fld0;
_25.fld1.fld1.1 = !_4;
_25.fld1.fld1.2 = (_25.fld3, false);
_25.fld1.fld1.1 = _20 << _18;
_25.fld1.fld2 = [_30,_30,_30,_30];
_8.0 = _3.0;
_13 = _12;
_28 = [_18,_18,_18,_18,_18];
_3.0 = _5 + _8.0;
_3.0 = _25.fld0 - _8.0;
Goto(bb8)
}
bb8 = {
_5 = _10;
_33 = -_25.fld0;
_28 = [_18,_18,_18,_18,_18];
_25.fld1.fld2 = [_30,_30,_30,_30];
_36 = _25.fld0 > _10;
_6.0 = -_3.0;
_38 = core::ptr::addr_of_mut!(_22.fld1);
_3.1 = [_18,_18,_18,_18,_18,_18,_18,_18];
(*_38) = [_21,_21];
_3 = (_10, _8.1);
_33 = -_25.fld0;
_7 = [_18,_18,_18,_18,_18,_18,_18,_18];
_25.fld1.fld1.1 = -_20;
(*_38) = [_21,_21];
_30 = (-3626190876204578777_i64) >> _20;
_32 = [_30,_30,_30,_30,_30,_30];
_8.1 = _13;
_24 = [_18,_18,_18,_18,_18,_18,_18,_18];
_25.fld6 = !_16;
_21 = '\u{35aad}';
_37 = -RET;
_34 = -_20;
_31 = [5173307178266307942_u64,7339193820346409069_u64,13036385540711145774_u64,7575454410937908122_u64,3407851817222027728_u64];
_25.fld1.fld2 = [_30,_30,_30,_30];
Goto(bb9)
}
bb9 = {
_8.0 = -_25.fld0;
(*_38) = [_21,_21];
_20 = (-46846632273181992556092480665487421142_i128) as isize;
_24 = [_18,_18,_18,_18,_18,_18,_18,_18];
_29 = _20 as usize;
_17 = -_34;
_25.fld1.fld2 = [_30,_30,_30,_30];
_41.fld0 = _29 - _29;
_15 = [_18,_18,_18,_18,_18,_18,_18,_18];
_13 = [_18,_18,_18,_18,_18,_18,_18,_18];
_8.1 = [_18,_18,_18,_18,_18,_18,_18,_18];
match _25.fld2 {
0 => bb8,
1 => bb2,
154099332610389531321582585116966221840 => bb11,
_ => bb10
}
}
bb10 = {
_18 = 18611_i16 as i32;
_20 = -_4;
_1 = _14 | _9;
_9 = 654466069734942185_i64 as u32;
_15 = [_18,_18,_18,_18,_18,_18,_18,_18];
_6 = _3;
_4 = _20 ^ _20;
_3 = _8;
_9 = !_1;
_12 = [_18,_18,_18,_18,_18,_18,_18,_18];
_18 = 60782686_i32;
_14 = _1;
_8.1 = [_18,_18,_18,_18,_18,_18,_18,_18];
_4 = -_20;
_19 = [_16,_16,_16];
_3 = (_6.0, _13);
RET = 18079_i16 as f32;
_1 = !_14;
_6.0 = _11 - _10;
_18 = _20 as i32;
_16 = _14 as u8;
_11 = (-3927130296508346864_i64) as f64;
_8.1 = [_18,_18,_18,_18,_18,_18,_18,_18];
_3 = (_6.0, _13);
Goto(bb2)
}
bb11 = {
_17 = _4 >> _20;
_25.fld5 = Adt47::Variant1 { fld0: _36,fld1: _18 };
_13 = [Field::<i32>(Variant(_25.fld5, 1), 1),Field::<i32>(Variant(_25.fld5, 1), 1),Field::<i32>(Variant(_25.fld5, 1), 1),_18,Field::<i32>(Variant(_25.fld5, 1), 1),Field::<i32>(Variant(_25.fld5, 1), 1),_18,_18];
_22.fld0 = core::ptr::addr_of!(_25.fld2);
_24 = [Field::<i32>(Variant(_25.fld5, 1), 1),Field::<i32>(Variant(_25.fld5, 1), 1),_18,Field::<i32>(Variant(_25.fld5, 1), 1),_18,_18,_18,_18];
_1 = _9 * _9;
_16 = _25.fld6 ^ _25.fld6;
SetDiscriminant(_25.fld5, 1);
_38 = core::ptr::addr_of_mut!(_22.fld1);
_14 = _21 as u32;
_29 = _41.fld0 << _16;
_39 = [_21,_21];
_1 = _34 as u32;
_31 = [16114748318718852815_u64,14761860396176409436_u64,14753168258293231096_u64,3646383610268954105_u64,72924580407111671_u64];
_9 = _1 - _1;
_3 = _6;
_25.fld1.fld1.2.0 = _25.fld3 << _1;
_3 = (_6.0, _24);
_45.fld1.1 = !_17;
_30 = !(-3682263919268291831_i64);
_27 = _25.fld0 + _10;
place!(Field::<bool>(Variant(_25.fld5, 1), 0)) = _9 != _9;
_45.fld1.2.1 = _27 <= _8.0;
_45.fld1 = (_8.1, _34, _25.fld1.fld1.2);
Goto(bb12)
}
bb12 = {
_25.fld4.1 = _1 & _1;
_45.fld1.0 = [_18,_18,_18,_18,_18,_18,_18,_18];
_25.fld6 = _16 ^ _16;
_29 = _41.fld0;
_32 = [_30,_30,_30,_30,_30,_30];
_47 = Field::<bool>(Variant(_25.fld5, 1), 0) > _45.fld1.2.1;
match _25.fld2 {
154099332610389531321582585116966221840 => bb14,
_ => bb13
}
}
bb13 = {
_14 = false as u32;
_12 = _6.1;
_1 = 20032_i16 as u32;
_10 = _3.0;
_22.fld1 = ['\u{3f439}','\u{a2b76}'];
_21 = '\u{a1d51}';
_20 = -_4;
_8.1 = [_18,_18,_18,_18,_18,_18,_18,_18];
_25.fld2 = 154099332610389531321582585116966221840_u128;
RET = 8198633472071594365_u64 as f32;
_15 = _8.1;
_25.fld3 = !67_i8;
_25.fld6 = !_16;
_25.fld1.fld1.2 = Checked(_25.fld3 + _25.fld3);
_25.fld6 = _16 >> _20;
_24 = [_18,_18,_18,_18,_18,_18,_18,_18];
_25.fld6 = _16 ^ _16;
_5 = _3.0;
_26 = [_16,_16,_25.fld6];
_25.fld1.fld1.0 = [_18,_18,_18,_18,_18,_18,_18,_18];
_3 = (_5, _25.fld1.fld1.0);
_22.fld0 = core::ptr::addr_of!(_25.fld2);
_25.fld0 = _8.0 + _10;
Goto(bb3)
}
bb14 = {
_14 = _10 as u32;
_28 = [_18,_18,_18,_18,_18];
_25.fld1.fld1.1 = _25.fld6 as isize;
_48.0 = -_27;
_48 = (_3.0, _15);
_30 = 6479405318066918210_i64 ^ (-6934062433711951792_i64);
(*_38) = [_21,_21];
_43 = 34970418127154582916742812678283644304_i128 as i32;
_25.fld1.fld1 = (_15, _17, _45.fld1.2);
_51 = [_21,_21];
_8 = (_3.0, _15);
_3.1 = [_18,_18,_18,_43,_43,_18,_18,_18];
_45.fld1.2.1 = !_47;
_29 = _41.fld0;
Goto(bb15)
}
bb15 = {
Call(_54 = dump_var(12_usize, 24_usize, Move(_24), 51_usize, Move(_51), 32_usize, Move(_32), 29_usize, Move(_29)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_54 = dump_var(12_usize, 14_usize, Move(_14), 18_usize, Move(_18), 19_usize, Move(_19), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_54 = dump_var(12_usize, 7_usize, Move(_7), 12_usize, Move(_12), 15_usize, Move(_15), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_54 = dump_var(12_usize, 28_usize, Move(_28), 21_usize, Move(_21), 55_usize, _55, 55_usize, _55), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [i32; 8],mut _2: [i32; 8],mut _3: (f64, [i32; 8]),mut _4: [i32; 8],mut _5: [i32; 8],mut _6: [i32; 8],mut _7: f32,mut _8: f64,mut _9: [u8; 3],mut _10: [i32; 8],mut _11: (f64, [i32; 8]),mut _12: (f64, [i32; 8])) -> isize {
mir! {
type RET = isize;
let _13: f32;
let _14: i64;
let _15: Adt49;
let _16: i32;
let _17: (f64, [i32; 8]);
let _18: isize;
let _19: *const *mut [char; 2];
let _20: i8;
let _21: char;
let _22: (f64, [i32; 8]);
let _23: Adt46;
let _24: char;
let _25: f32;
let _26: ([i32; 8], isize, (i8, bool));
let _27: [char; 2];
let _28: bool;
let _29: u128;
let _30: i32;
let _31: u64;
let _32: isize;
let _33: isize;
let _34: bool;
let _35: ();
let _36: ();
{
_12 = _11;
_11 = (_3.0, _5);
_10 = _2;
_10 = [(-280530747_i32),(-1236471005_i32),1883516239_i32,(-164774039_i32),(-977838575_i32),1203358860_i32,(-796679185_i32),(-2129258161_i32)];
_2 = _11.1;
_11 = _12;
_1 = [155075890_i32,943639285_i32,(-12850547_i32),(-1795343924_i32),483594500_i32,200265584_i32,1864619254_i32,1888814072_i32];
_13 = _7;
_13 = _7 * _7;
_6 = [305848452_i32,(-1994233211_i32),508199920_i32,255809592_i32,1361393547_i32,(-1379455660_i32),(-1066193762_i32),(-1874560196_i32)];
_2 = _1;
_13 = _7 - _7;
_3 = (_8, _1);
_9 = [164_u8,115_u8,137_u8];
_4 = [1253891410_i32,(-619980630_i32),1839491726_i32,(-716586845_i32),(-1205491507_i32),14154818_i32,(-1945105404_i32),(-994048834_i32)];
_10 = _4;
_4 = [(-128004307_i32),(-1529734363_i32),2118573146_i32,(-1663999083_i32),(-857342691_i32),18209237_i32,1375202737_i32,289500478_i32];
_12.0 = _8;
_12 = (_11.0, _5);
Goto(bb1)
}
bb1 = {
_4 = _1;
_10 = _4;
_9 = [235_u8,248_u8,169_u8];
_11 = (_8, _5);
_12.1 = [1411693351_i32,1973159847_i32,1163304317_i32,203836461_i32,784930178_i32,46674439_i32,2040532518_i32,(-1052997101_i32)];
_15 = Adt49 { fld0: 0_usize };
Goto(bb2)
}
bb2 = {
_4 = _6;
_12.1 = [129664300_i32,2023138412_i32,(-1884507964_i32),(-287589132_i32),1203895050_i32,(-1571878090_i32),(-40829463_i32),1724016673_i32];
_6 = [(-160341284_i32),2075360495_i32,(-1478493540_i32),(-2067398268_i32),(-1926778013_i32),(-1638234174_i32),689513665_i32,(-1323078203_i32)];
_3.0 = -_12.0;
_14 = 5815900067807552375127618882747931007_i128 as i64;
_11 = (_8, _2);
RET = (-24595_i16) as isize;
_15 = Adt49 { fld0: 1_usize };
_11.0 = -_3.0;
_3.1 = [(-522037350_i32),(-1037981644_i32),1388854194_i32,38633379_i32,1349169772_i32,(-1927826246_i32),439658455_i32,(-1110550698_i32)];
_16 = (-1328919122_i32);
_13 = (-6786_i16) as f32;
_15 = Adt49 { fld0: 16024215519154764061_usize };
_17 = (_8, _4);
_7 = _13;
_14 = !(-2628700273130748173_i64);
RET = _14 as isize;
_5 = [_16,_16,_16,_16,_16,_16,_16,_16];
_8 = _11.0 + _11.0;
_5 = [_16,_16,_16,_16,_16,_16,_16,_16];
_8 = -_3.0;
_11.1 = [_16,_16,_16,_16,_16,_16,_16,_16];
_3 = (_12.0, _10);
Goto(bb3)
}
bb3 = {
_15.fld0 = 0_usize & 18155320076368864270_usize;
_12.0 = _8 - _8;
_21 = '\u{961af}';
_10 = [_16,_16,_16,_16,_16,_16,_16,_16];
_11 = _12;
_13 = _7;
_20 = 114_i8;
_5 = [_16,_16,_16,_16,_16,_16,_16,_16];
_3 = _17;
_22.1 = [_16,_16,_16,_16,_16,_16,_16,_16];
Goto(bb4)
}
bb4 = {
_20 = 70_i8;
_12 = (_11.0, _2);
_17.0 = _8 - _12.0;
_3.1 = [_16,_16,_16,_16,_16,_16,_16,_16];
_3.1 = [_16,_16,_16,_16,_16,_16,_16,_16];
_24 = _21;
_25 = _7 - _13;
match _20 {
0 => bb1,
70 => bb5,
_ => bb2
}
}
bb5 = {
_20 = 41_i8;
_16 = 467034503_u32 as i32;
_26.2 = (_20, false);
_5 = [_16,_16,_16,_16,_16,_16,_16,_16];
RET = 41_isize << _15.fld0;
_26.1 = RET ^ RET;
Goto(bb6)
}
bb6 = {
_22.0 = -_3.0;
_18 = _26.1;
_28 = _26.2.1;
_15 = Adt49 { fld0: 5_usize };
Call(_17 = fn14(_12, _2, _9, _26.2.0, _12.1, _21, _22, _6, _11, _1, _4, _18, _11.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_27 = [_21,_24];
_6 = [_16,_16,_16,_16,_16,_16,_16,_16];
_25 = _13;
_12.0 = -_17.0;
_26.2 = (_20, _28);
_3.1 = [_16,_16,_16,_16,_16,_16,_16,_16];
_18 = RET | _26.1;
_18 = -RET;
_20 = _26.2.0 >> _26.2.0;
Goto(bb8)
}
bb8 = {
_3 = (_17.0, _1);
_20 = _26.2.0;
_10 = _2;
RET = !_18;
_12.0 = _3.0 + _17.0;
match _20 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
41 => bb10,
_ => bb9
}
}
bb9 = {
_4 = _1;
_10 = _4;
_9 = [235_u8,248_u8,169_u8];
_11 = (_8, _5);
_12.1 = [1411693351_i32,1973159847_i32,1163304317_i32,203836461_i32,784930178_i32,46674439_i32,2040532518_i32,(-1052997101_i32)];
_15 = Adt49 { fld0: 0_usize };
Goto(bb2)
}
bb10 = {
_17 = (_12.0, _10);
_3.1 = [_16,_16,_16,_16,_16,_16,_16,_16];
RET = _18 << _14;
_12 = (_3.0, _4);
_26.0 = [_16,_16,_16,_16,_16,_16,_16,_16];
_10 = [_16,_16,_16,_16,_16,_16,_16,_16];
RET = _18 * _26.1;
_26.2.0 = _20;
_12 = (_17.0, _2);
match _26.2.0 {
0 => bb1,
1 => bb7,
2 => bb9,
3 => bb6,
41 => bb11,
_ => bb5
}
}
bb11 = {
_25 = _7;
_15 = Adt49 { fld0: 17952163747206694902_usize };
_26.1 = RET;
_1 = _17.1;
_11.1 = [_16,_16,_16,_16,_16,_16,_16,_16];
_27 = [_21,_24];
_26.2.0 = _20;
_12.0 = _17.0 * _17.0;
_24 = _21;
_6 = [_16,_16,_16,_16,_16,_16,_16,_16];
_1 = [_16,_16,_16,_16,_16,_16,_16,_16];
_24 = _21;
_22.0 = -_12.0;
Goto(bb12)
}
bb12 = {
_11.1 = _6;
_27 = [_24,_21];
_31 = 150342630493838031856338604646057865063_u128 as u64;
_26.2.1 = !_28;
_6 = _17.1;
_5 = [_16,_16,_16,_16,_16,_16,_16,_16];
_10 = [_16,_16,_16,_16,_16,_16,_16,_16];
_9 = [196_u8,43_u8,227_u8];
_1 = [_16,_16,_16,_16,_16,_16,_16,_16];
_3.1 = _6;
_18 = RET;
_12 = (_17.0, _6);
_11 = (_12.0, _17.1);
_32 = _22.0 as isize;
match _20 {
0 => bb13,
41 => bb15,
_ => bb14
}
}
bb13 = {
_15.fld0 = 0_usize & 18155320076368864270_usize;
_12.0 = _8 - _8;
_21 = '\u{961af}';
_10 = [_16,_16,_16,_16,_16,_16,_16,_16];
_11 = _12;
_13 = _7;
_20 = 114_i8;
_5 = [_16,_16,_16,_16,_16,_16,_16,_16];
_3 = _17;
_22.1 = [_16,_16,_16,_16,_16,_16,_16,_16];
Goto(bb4)
}
bb14 = {
_4 = _6;
_12.1 = [129664300_i32,2023138412_i32,(-1884507964_i32),(-287589132_i32),1203895050_i32,(-1571878090_i32),(-40829463_i32),1724016673_i32];
_6 = [(-160341284_i32),2075360495_i32,(-1478493540_i32),(-2067398268_i32),(-1926778013_i32),(-1638234174_i32),689513665_i32,(-1323078203_i32)];
_3.0 = -_12.0;
_14 = 5815900067807552375127618882747931007_i128 as i64;
_11 = (_8, _2);
RET = (-24595_i16) as isize;
_15 = Adt49 { fld0: 1_usize };
_11.0 = -_3.0;
_3.1 = [(-522037350_i32),(-1037981644_i32),1388854194_i32,38633379_i32,1349169772_i32,(-1927826246_i32),439658455_i32,(-1110550698_i32)];
_16 = (-1328919122_i32);
_13 = (-6786_i16) as f32;
_15 = Adt49 { fld0: 16024215519154764061_usize };
_17 = (_8, _4);
_7 = _13;
_14 = !(-2628700273130748173_i64);
RET = _14 as isize;
_5 = [_16,_16,_16,_16,_16,_16,_16,_16];
_8 = _11.0 + _11.0;
_5 = [_16,_16,_16,_16,_16,_16,_16,_16];
_8 = -_3.0;
_11.1 = [_16,_16,_16,_16,_16,_16,_16,_16];
_3 = (_12.0, _10);
Goto(bb3)
}
bb15 = {
_7 = _25 + _25;
_3.1 = [_16,_16,_16,_16,_16,_16,_16,_16];
_9 = [58_u8,75_u8,229_u8];
_3 = _22;
_11 = _3;
_25 = _15.fld0 as f32;
_17.0 = _11.0;
_26.0 = _6;
_11 = _22;
_26.2.0 = _20 * _20;
_14 = -657517247150872061_i64;
Goto(bb16)
}
bb16 = {
Call(_35 = dump_var(13_usize, 5_usize, Move(_5), 28_usize, Move(_28), 32_usize, Move(_32), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(13_usize, 20_usize, Move(_20), 24_usize, Move(_24), 4_usize, Move(_4), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_35 = dump_var(13_usize, 18_usize, Move(_18), 36_usize, _36, 36_usize, _36, 36_usize, _36), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: (f64, [i32; 8]),mut _2: [i32; 8],mut _3: [u8; 3],mut _4: i8,mut _5: [i32; 8],mut _6: char,mut _7: (f64, [i32; 8]),mut _8: [i32; 8],mut _9: (f64, [i32; 8]),mut _10: [i32; 8],mut _11: [i32; 8],mut _12: isize,mut _13: f64) -> (f64, [i32; 8]) {
mir! {
type RET = (f64, [i32; 8]);
let _14: (f32,);
let _15: (f64, [i32; 8]);
let _16: bool;
let _17: u64;
let _18: f64;
let _19: [char; 2];
let _20: Adt44;
let _21: Adt49;
let _22: f32;
let _23: i128;
let _24: isize;
let _25: char;
let _26: u128;
let _27: i8;
let _28: ();
let _29: ();
{
_1 = (_9.0, _10);
_9 = (_7.0, _1.1);
_1 = (_9.0, _2);
_9.0 = _7.0 + _1.0;
Call(_9.1 = fn15(_8, _1, _3, _3, _7, _5, _3, _2, _12), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _1;
_9.1 = [1312013202_i32,85508600_i32,(-424355553_i32),2121647309_i32,48452094_i32,(-190628511_i32),(-1539337746_i32),728427001_i32];
RET = (_1.0, _2);
_11 = [(-1212737862_i32),(-967140746_i32),501503747_i32,666591920_i32,167613149_i32,(-920474684_i32),(-1477564308_i32),1867804054_i32];
RET = (_1.0, _2);
_9.1 = _10;
_3 = [59_u8,91_u8,127_u8];
_7 = (_13, _10);
_11 = [(-167948962_i32),(-1299924756_i32),(-2117953096_i32),(-630446689_i32),1301325774_i32,2069336852_i32,1897191287_i32,(-283003680_i32)];
_5 = [(-1795610162_i32),413530945_i32,(-929291418_i32),(-1866141297_i32),(-1883715264_i32),24315851_i32,(-205448013_i32),1658216504_i32];
_13 = _1.0;
RET.1 = _8;
_12 = (-9223372036854775808_isize);
_15 = _1;
_10 = [2061782069_i32,643674029_i32,2098672908_i32,2003821055_i32,1090696191_i32,(-1693700870_i32),791729993_i32,1383173056_i32];
_4 = 5_usize as i8;
_12 = 81_isize | 9223372036854775807_isize;
_8 = [1160906211_i32,(-1918707302_i32),1702774109_i32,1566773493_i32,(-2119672510_i32),(-841661041_i32),490448026_i32,(-1873382420_i32)];
RET = (_13, _8);
RET.1 = [(-1042726911_i32),89687456_i32,(-115904143_i32),(-807944078_i32),(-1195058698_i32),649914321_i32,(-294209789_i32),1361883796_i32];
RET = (_7.0, _9.1);
Goto(bb2)
}
bb2 = {
_9 = _1;
Goto(bb3)
}
bb3 = {
_15.1 = [1151821575_i32,(-2100980248_i32),625146475_i32,(-911005559_i32),1047736559_i32,332894241_i32,168152191_i32,2139201644_i32];
_14.0 = 10227237059678470692_u64 as f32;
RET.0 = _15.0 + _15.0;
_4 = 80_i8 & (-41_i8);
_7 = (_1.0, _2);
_18 = RET.0;
_9 = (RET.0, _15.1);
RET.1 = _11;
_8 = [713552306_i32,(-203095368_i32),1651533151_i32,(-1799158546_i32),939641565_i32,859583024_i32,710467892_i32,(-365739610_i32)];
RET.1 = [1153758901_i32,(-429085_i32),451095831_i32,928987801_i32,1632035674_i32,(-1741360536_i32),(-1212857365_i32),(-1135599075_i32)];
_14.0 = 15149336242588644995_usize as f32;
_8 = [188345147_i32,1133693307_i32,1369295196_i32,(-1494732011_i32),(-1017269277_i32),117081338_i32,1177489750_i32,856730917_i32];
RET.1 = [1877798245_i32,622098413_i32,(-662986190_i32),398161353_i32,766419480_i32,(-1307125615_i32),1357182332_i32,557225596_i32];
_17 = 14424394119289378123_u64 | 8967810686573339743_u64;
_9 = _7;
_5 = [1697557736_i32,(-316157021_i32),(-1831213172_i32),419991200_i32,1440035025_i32,478275992_i32,1151891039_i32,1302261658_i32];
_17 = _13 as u64;
_16 = !true;
_4 = (-10_i8);
_7 = (_1.0, _10);
_5 = [(-1563282507_i32),1555247089_i32,(-310718935_i32),2145557719_i32,1765749565_i32,1560015392_i32,(-793530321_i32),478233391_i32];
_21.fld0 = 6_usize >> _17;
RET.0 = _13 * _1.0;
RET = _1;
Goto(bb4)
}
bb4 = {
_9.0 = _7.0 * _13;
RET.0 = _18;
_15 = _1;
_4 = 211_u8 as i8;
_22 = -_14.0;
_9 = (_18, _8);
_21.fld0 = 2477466375_u32 as usize;
_21.fld0 = 1692267061784398446_usize;
_15 = _1;
RET.1 = [1100989244_i32,82226522_i32,1601913483_i32,2010855077_i32,(-279724954_i32),(-498288801_i32),1046733965_i32,436352648_i32];
RET.1 = _1.1;
RET.1 = [1963134720_i32,(-1076200627_i32),1135890642_i32,1349739430_i32,(-596984527_i32),(-775651234_i32),(-439459238_i32),(-1190090707_i32)];
_9.0 = _18 + _15.0;
RET.0 = -_9.0;
_15.0 = _9.0 + _9.0;
_2 = [1909774927_i32,(-327412881_i32),(-751025074_i32),(-1024232914_i32),1134596869_i32,1518529036_i32,(-695374577_i32),2120777923_i32];
_9.1 = _8;
RET = (_7.0, _7.1);
RET = (_15.0, _8);
_15.1 = _9.1;
_24 = _14.0 as isize;
_1 = _15;
_15.0 = RET.0 * _1.0;
Goto(bb5)
}
bb5 = {
Call(_28 = dump_var(14_usize, 12_usize, Move(_12), 3_usize, Move(_3), 2_usize, Move(_2), 10_usize, Move(_10)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_28 = dump_var(14_usize, 6_usize, Move(_6), 16_usize, Move(_16), 29_usize, _29, 29_usize, _29), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: [i32; 8],mut _2: (f64, [i32; 8]),mut _3: [u8; 3],mut _4: [u8; 3],mut _5: (f64, [i32; 8]),mut _6: [i32; 8],mut _7: [u8; 3],mut _8: [i32; 8],mut _9: isize) -> [i32; 8] {
mir! {
type RET = [i32; 8];
let _10: Adt54;
let _11: isize;
let _12: char;
let _13: ([i32; 8], isize, (i8, bool));
let _14: *const u64;
let _15: u64;
let _16: (i8, bool);
let _17: f64;
let _18: [char; 2];
let _19: *const [i32; 8];
let _20: *const u128;
let _21: i32;
let _22: bool;
let _23: (i8, bool);
let _24: ();
let _25: ();
{
_8 = [(-1459448338_i32),(-500334092_i32),407949988_i32,37195489_i32,(-639392227_i32),(-309121430_i32),(-1587996444_i32),932334039_i32];
_2.0 = _5.0 * _5.0;
_10 = Adt54::Variant0 { fld0: 1447729115_i32 };
_2.0 = _5.0;
_2 = (_5.0, _6);
_6 = [(-2117418476_i32),(-2131614646_i32),(-2052020964_i32),382275825_i32,(-762079085_i32),(-695069699_i32),284755333_i32,1432984933_i32];
_4 = [48_u8,171_u8,61_u8];
_11 = _9;
_4 = [160_u8,167_u8,156_u8];
_1 = _8;
_10 = Adt54::Variant0 { fld0: 485173097_i32 };
_2 = (_5.0, _6);
RET = [(-755429452_i32),(-1884147294_i32),(-1332087032_i32),(-162978900_i32),441811431_i32,(-2010669395_i32),1825431131_i32,1291134160_i32];
_5.1 = RET;
RET = [17644768_i32,1486895819_i32,(-714714968_i32),441559427_i32,1006904925_i32,1880168943_i32,723610302_i32,(-1550429405_i32)];
_12 = '\u{8e68a}';
_7 = [242_u8,54_u8,101_u8];
_12 = '\u{57079}';
_5.1 = [1801147730_i32,1019147470_i32,(-1745267401_i32),136529369_i32,(-2020598304_i32),303706653_i32,1395116103_i32,707766735_i32];
Call(_13.1 = fn16(_6, _6, _11, _2.1, _11, _5, _11, _5.1, RET, _5.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_13.2.1 = true;
_8 = _5.1;
place!(Field::<i32>(Variant(_10, 0), 0)) = 608841817_i32 | 1473559881_i32;
_6 = [Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0)];
_13.2.1 = _5.0 <= _5.0;
_6 = [Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0)];
SetDiscriminant(_10, 2);
place!(Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1)).0 = !_13.2.1;
_13.2.0 = -101_i8;
place!(Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1)).1 = !_13.2.1;
_7 = [239_u8,57_u8,92_u8];
_5.0 = _2.0;
_13.2.0 = 47_i8;
_6 = [(-1712784934_i32),(-364534393_i32),1930024691_i32,2040632921_i32,(-922784516_i32),(-482081529_i32),1108636798_i32,(-1883364978_i32)];
place!(Field::<(f64, [i32; 8])>(Variant(_10, 2), 4)).0 = _2.0 - _2.0;
_13.1 = !_11;
match _13.2.0 {
47 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_3 = [134_u8,32_u8,1_u8];
_5.1 = [611411476_i32,(-836916971_i32),1949808099_i32,(-1544571162_i32),(-1501212179_i32),(-2012902850_i32),(-1653199050_i32),683297172_i32];
place!(Field::<u32>(Variant(_10, 2), 3)) = !140472127_u32;
place!(Field::<(f64, [i32; 8])>(Variant(_10, 2), 4)) = (_2.0, _2.1);
place!(Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1)).4 = 11787075409160952493_u64;
_9 = -_13.1;
place!(Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1)).3 = (-973482123_i32) & (-101619804_i32);
_1 = _5.1;
_15 = Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).4 / Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).4;
_9 = _11;
_4 = [230_u8,191_u8,49_u8];
match Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).4 {
11787075409160952493 => bb4,
_ => bb1
}
}
bb4 = {
RET = [Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3];
RET = _1;
_16.0 = (-30848662199785993505903040281166497260_i128) as i8;
_13.0 = [Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3];
_5 = (_2.0, _1);
_1 = [Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3];
_13.2.1 = Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).1;
place!(Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1)).4 = !_15;
RET = Field::<(f64, [i32; 8])>(Variant(_10, 2), 4).1;
_3 = [23_u8,46_u8,108_u8];
place!(Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1)).4 = (-4513_i16) as u64;
_5.0 = _13.1 as f64;
_12 = '\u{2177a}';
_9 = _11 >> _13.1;
place!(Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1)).3 = (-1826131906_i32);
place!(Field::<(f64, [i32; 8])>(Variant(_10, 2), 4)).0 = _5.0;
_19 = core::ptr::addr_of!(place!(Field::<(f64, [i32; 8])>(Variant(_10, 2), 4)).1);
match _13.2.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
47 => bb7,
_ => bb6
}
}
bb5 = {
_13.2.1 = true;
_8 = _5.1;
place!(Field::<i32>(Variant(_10, 0), 0)) = 608841817_i32 | 1473559881_i32;
_6 = [Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0)];
_13.2.1 = _5.0 <= _5.0;
_6 = [Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0)];
SetDiscriminant(_10, 2);
place!(Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1)).0 = !_13.2.1;
_13.2.0 = -101_i8;
place!(Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1)).1 = !_13.2.1;
_7 = [239_u8,57_u8,92_u8];
_5.0 = _2.0;
_13.2.0 = 47_i8;
_6 = [(-1712784934_i32),(-364534393_i32),1930024691_i32,2040632921_i32,(-922784516_i32),(-482081529_i32),1108636798_i32,(-1883364978_i32)];
place!(Field::<(f64, [i32; 8])>(Variant(_10, 2), 4)).0 = _2.0 - _2.0;
_13.1 = !_11;
match _13.2.0 {
47 => bb3,
_ => bb2
}
}
bb6 = {
Return()
}
bb7 = {
RET = [Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3];
_5.0 = -Field::<(f64, [i32; 8])>(Variant(_10, 2), 4).0;
_7 = _4;
_16.1 = Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).0;
place!(Field::<(f64, [i32; 8])>(Variant(_10, 2), 4)).1 = _13.0;
RET = [Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3];
match _13.2.0 {
0 => bb5,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
47 => bb15,
_ => bb14
}
}
bb8 = {
Return()
}
bb9 = {
_13.2.1 = true;
_8 = _5.1;
place!(Field::<i32>(Variant(_10, 0), 0)) = 608841817_i32 | 1473559881_i32;
_6 = [Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0)];
_13.2.1 = _5.0 <= _5.0;
_6 = [Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0)];
SetDiscriminant(_10, 2);
place!(Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1)).0 = !_13.2.1;
_13.2.0 = -101_i8;
place!(Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1)).1 = !_13.2.1;
_7 = [239_u8,57_u8,92_u8];
_5.0 = _2.0;
_13.2.0 = 47_i8;
_6 = [(-1712784934_i32),(-364534393_i32),1930024691_i32,2040632921_i32,(-922784516_i32),(-482081529_i32),1108636798_i32,(-1883364978_i32)];
place!(Field::<(f64, [i32; 8])>(Variant(_10, 2), 4)).0 = _2.0 - _2.0;
_13.1 = !_11;
match _13.2.0 {
47 => bb3,
_ => bb2
}
}
bb10 = {
RET = [Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3];
RET = _1;
_16.0 = (-30848662199785993505903040281166497260_i128) as i8;
_13.0 = [Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3];
_5 = (_2.0, _1);
_1 = [Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3];
_13.2.1 = Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).1;
place!(Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1)).4 = !_15;
RET = Field::<(f64, [i32; 8])>(Variant(_10, 2), 4).1;
_3 = [23_u8,46_u8,108_u8];
place!(Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1)).4 = (-4513_i16) as u64;
_5.0 = _13.1 as f64;
_12 = '\u{2177a}';
_9 = _11 >> _13.1;
place!(Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1)).3 = (-1826131906_i32);
place!(Field::<(f64, [i32; 8])>(Variant(_10, 2), 4)).0 = _5.0;
_19 = core::ptr::addr_of!(place!(Field::<(f64, [i32; 8])>(Variant(_10, 2), 4)).1);
match _13.2.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
47 => bb7,
_ => bb6
}
}
bb11 = {
_3 = [134_u8,32_u8,1_u8];
_5.1 = [611411476_i32,(-836916971_i32),1949808099_i32,(-1544571162_i32),(-1501212179_i32),(-2012902850_i32),(-1653199050_i32),683297172_i32];
place!(Field::<u32>(Variant(_10, 2), 3)) = !140472127_u32;
place!(Field::<(f64, [i32; 8])>(Variant(_10, 2), 4)) = (_2.0, _2.1);
place!(Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1)).4 = 11787075409160952493_u64;
_9 = -_13.1;
place!(Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1)).3 = (-973482123_i32) & (-101619804_i32);
_1 = _5.1;
_15 = Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).4 / Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).4;
_9 = _11;
_4 = [230_u8,191_u8,49_u8];
match Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).4 {
11787075409160952493 => bb4,
_ => bb1
}
}
bb12 = {
Return()
}
bb13 = {
_13.2.1 = true;
_8 = _5.1;
place!(Field::<i32>(Variant(_10, 0), 0)) = 608841817_i32 | 1473559881_i32;
_6 = [Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0)];
_13.2.1 = _5.0 <= _5.0;
_6 = [Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0),Field::<i32>(Variant(_10, 0), 0)];
SetDiscriminant(_10, 2);
place!(Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1)).0 = !_13.2.1;
_13.2.0 = -101_i8;
place!(Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1)).1 = !_13.2.1;
_7 = [239_u8,57_u8,92_u8];
_5.0 = _2.0;
_13.2.0 = 47_i8;
_6 = [(-1712784934_i32),(-364534393_i32),1930024691_i32,2040632921_i32,(-922784516_i32),(-482081529_i32),1108636798_i32,(-1883364978_i32)];
place!(Field::<(f64, [i32; 8])>(Variant(_10, 2), 4)).0 = _2.0 - _2.0;
_13.1 = !_11;
match _13.2.0 {
47 => bb3,
_ => bb2
}
}
bb14 = {
Return()
}
bb15 = {
_17 = _5.0 - Field::<(f64, [i32; 8])>(Variant(_10, 2), 4).0;
RET = _6;
_14 = core::ptr::addr_of!(_15);
_2.1 = [Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3,Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).3];
_22 = Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).0 ^ Field::<(bool, bool, *const u64, i32, u64)>(Variant(_10, 2), 1).1;
Goto(bb16)
}
bb16 = {
Call(_24 = dump_var(15_usize, 1_usize, Move(_1), 8_usize, Move(_8), 16_usize, Move(_16), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_24 = dump_var(15_usize, 4_usize, Move(_4), 11_usize, Move(_11), 25_usize, _25, 25_usize, _25), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: [i32; 8],mut _2: [i32; 8],mut _3: isize,mut _4: [i32; 8],mut _5: isize,mut _6: (f64, [i32; 8]),mut _7: isize,mut _8: [i32; 8],mut _9: [i32; 8],mut _10: [i32; 8]) -> isize {
mir! {
type RET = isize;
let _11: i32;
let _12: [i64; 4];
let _13: u8;
let _14: u32;
let _15: *mut i16;
let _16: [u64; 5];
let _17: [i32; 5];
let _18: [i32; 8];
let _19: isize;
let _20: [i32; 5];
let _21: bool;
let _22: f32;
let _23: Adt49;
let _24: Adt55;
let _25: bool;
let _26: [i32; 8];
let _27: [i64; 4];
let _28: (f64, [i32; 8]);
let _29: *const *mut [char; 2];
let _30: f64;
let _31: (i8, bool);
let _32: Adt48;
let _33: ();
let _34: ();
{
_6.0 = 2863049228_u32 as f64;
_2 = [222209072_i32,2017531244_i32,474911559_i32,676289659_i32,(-1223062809_i32),(-2035024287_i32),42999561_i32,(-767164215_i32)];
Goto(bb1)
}
bb1 = {
_10 = [1513882791_i32,(-1938085558_i32),1049973607_i32,(-1657033777_i32),(-382310145_i32),1430898995_i32,556223739_i32,390709990_i32];
_11 = false as i32;
_4 = [_11,_11,_11,_11,_11,_11,_11,_11];
_8 = [_11,_11,_11,_11,_11,_11,_11,_11];
_6.0 = 16528457735134089925_u64 as f64;
_1 = [_11,_11,_11,_11,_11,_11,_11,_11];
Call(_6.0 = fn17(_5, _9, _4, _2, _2, _3, _6.1, _7, _1, _5, _3, _3, _6.1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = [_11,_11,_11,_11,_11,_11,_11,_11];
_11 = 172_u8 as i32;
_7 = _5 + _5;
RET = '\u{3a3a9}' as isize;
_4 = [_11,_11,_11,_11,_11,_11,_11,_11];
_14 = 2902636568_u32;
_8 = [_11,_11,_11,_11,_11,_11,_11,_11];
_13 = 93_u8 + 206_u8;
_16 = [12996748336307462112_u64,8744515912035947120_u64,17742443376438587603_u64,7857797109792630555_u64,8405029519343539155_u64];
_20 = [_11,_11,_11,_11,_11];
_13 = 16_u8 + 212_u8;
_3 = -RET;
Goto(bb3)
}
bb3 = {
_12 = [(-3377976841967098932_i64),(-1274418566488461787_i64),(-8143820191036206330_i64),5828314763638554765_i64];
_20 = [_11,_11,_11,_11,_11];
_16 = [18173273010296572179_u64,2691140240541247551_u64,8703894695551065381_u64,7351511212179671576_u64,5621377349038501653_u64];
RET = 32523_i16 as isize;
_22 = 3648290709251031393_i64 as f32;
match _14 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
2902636568 => bb11,
_ => bb10
}
}
bb4 = {
_4 = [_11,_11,_11,_11,_11,_11,_11,_11];
_11 = 172_u8 as i32;
_7 = _5 + _5;
RET = '\u{3a3a9}' as isize;
_4 = [_11,_11,_11,_11,_11,_11,_11,_11];
_14 = 2902636568_u32;
_8 = [_11,_11,_11,_11,_11,_11,_11,_11];
_13 = 93_u8 + 206_u8;
_16 = [12996748336307462112_u64,8744515912035947120_u64,17742443376438587603_u64,7857797109792630555_u64,8405029519343539155_u64];
_20 = [_11,_11,_11,_11,_11];
_13 = 16_u8 + 212_u8;
_3 = -RET;
Goto(bb3)
}
bb5 = {
_10 = [1513882791_i32,(-1938085558_i32),1049973607_i32,(-1657033777_i32),(-382310145_i32),1430898995_i32,556223739_i32,390709990_i32];
_11 = false as i32;
_4 = [_11,_11,_11,_11,_11,_11,_11,_11];
_8 = [_11,_11,_11,_11,_11,_11,_11,_11];
_6.0 = 16528457735134089925_u64 as f64;
_1 = [_11,_11,_11,_11,_11,_11,_11,_11];
Call(_6.0 = fn17(_5, _9, _4, _2, _2, _3, _6.1, _7, _1, _5, _3, _3, _6.1), ReturnTo(bb2), UnwindUnreachable())
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
_24.fld0.1 = false;
_23 = Adt49 { fld0: 0_usize };
_24.fld0.3 = -_11;
_13 = _22 as u8;
_24.fld0.1 = _5 == _7;
_20 = [_11,_11,_24.fld0.3,_24.fld0.3,_11];
_24.fld0.4 = !18286390868352338310_u64;
_20 = [_11,_11,_24.fld0.3,_11,_11];
_24.fld0.3 = _11 ^ _11;
_24.fld0.0 = _24.fld0.1;
_5 = _7 ^ _7;
_17 = [_24.fld0.3,_11,_24.fld0.3,_24.fld0.3,_24.fld0.3];
_24.fld3 = [607641950693323579_i64,(-3325132408862836869_i64),4753679981280805972_i64,7436570823631285974_i64,6884938706806425624_i64,3791127996931367104_i64];
_24.fld0.2 = core::ptr::addr_of!(_24.fld0.4);
_8 = [_24.fld0.3,_24.fld0.3,_24.fld0.3,_24.fld0.3,_11,_24.fld0.3,_11,_24.fld0.3];
_16 = [_24.fld0.4,_24.fld0.4,_24.fld0.4,_24.fld0.4,_24.fld0.4];
_18 = [_11,_11,_11,_24.fld0.3,_24.fld0.3,_24.fld0.3,_11,_24.fld0.3];
_22 = 5050265434606262903_i64 as f32;
_20 = _17;
_14 = !1246117425_u32;
_14 = 2134579243_u32 >> _5;
_19 = _24.fld0.1 as isize;
_8 = [_24.fld0.3,_24.fld0.3,_24.fld0.3,_11,_24.fld0.3,_11,_11,_11];
_24.fld2 = 335922304905937897163103574349965562924_u128;
_13 = 189_u8 ^ 185_u8;
_24.fld5 = 63513_u16 as u64;
Call(_2 = fn18(_7, _24.fld0.3, _14, _8, _14, _6.0, _6.0, _24.fld0.1, _24.fld0.0, _7, _19, _19), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_6.0 = (-23_i8) as f64;
_21 = _24.fld0.0 != _24.fld0.1;
_12 = [(-1195651676117312292_i64),(-6408188925933990498_i64),3277603830026176286_i64,6356285814939763316_i64];
_28.0 = _6.0 * _6.0;
_6.0 = _28.0;
_9 = [_24.fld0.3,_24.fld0.3,_11,_24.fld0.3,_11,_24.fld0.3,_11,_24.fld0.3];
_9 = [_24.fld0.3,_24.fld0.3,_24.fld0.3,_24.fld0.3,_24.fld0.3,_11,_24.fld0.3,_24.fld0.3];
_27 = [(-196651304110251869_i64),(-4404903223649807595_i64),(-250027964636501880_i64),4397538446915229512_i64];
_19 = _14 as isize;
_21 = _24.fld0.0;
_4 = _1;
_25 = _24.fld0.1;
_26 = [_11,_24.fld0.3,_11,_24.fld0.3,_24.fld0.3,_24.fld0.3,_11,_11];
_11 = _24.fld0.3 + _24.fld0.3;
_3 = 2871617869053839767_i64 as isize;
_24.fld3 = [(-5359471475135285552_i64),(-5200786793561499_i64),(-2876754930192973063_i64),(-5936208045623324522_i64),(-7220294960317374622_i64),5094695102315545178_i64];
_21 = _24.fld0.1 & _24.fld0.1;
_28.1 = [_11,_24.fld0.3,_11,_24.fld0.3,_11,_11,_11,_11];
_8 = _6.1;
_24.fld4 = [_23.fld0,_23.fld0,_23.fld0,_23.fld0,_23.fld0,_23.fld0];
_11 = _24.fld0.3;
_14 = !1546563568_u32;
_11 = !_24.fld0.3;
Call(_26 = core::intrinsics::transmute(_28.1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_23 = Adt49 { fld0: 3_usize };
_32.fld0.0 = 5_i8 as f64;
_31.1 = _21 == _24.fld0.0;
_24.fld0.4 = _24.fld5;
_16 = [_24.fld5,_24.fld0.4,_24.fld0.4,_24.fld5,_24.fld0.4];
_27 = _12;
_27 = _12;
_3 = '\u{5fc2d}' as isize;
_29 = core::ptr::addr_of!(_24.fld1);
_24.fld4 = [_23.fld0,_23.fld0,_23.fld0,_23.fld0,_23.fld0,_23.fld0];
_14 = !1212161695_u32;
match _23.fld0 {
0 => bb1,
1 => bb11,
2 => bb14,
4 => bb16,
5 => bb17,
3 => bb19,
_ => bb18
}
}
bb14 = {
_6.0 = (-23_i8) as f64;
_21 = _24.fld0.0 != _24.fld0.1;
_12 = [(-1195651676117312292_i64),(-6408188925933990498_i64),3277603830026176286_i64,6356285814939763316_i64];
_28.0 = _6.0 * _6.0;
_6.0 = _28.0;
_9 = [_24.fld0.3,_24.fld0.3,_11,_24.fld0.3,_11,_24.fld0.3,_11,_24.fld0.3];
_9 = [_24.fld0.3,_24.fld0.3,_24.fld0.3,_24.fld0.3,_24.fld0.3,_11,_24.fld0.3,_24.fld0.3];
_27 = [(-196651304110251869_i64),(-4404903223649807595_i64),(-250027964636501880_i64),4397538446915229512_i64];
_19 = _14 as isize;
_21 = _24.fld0.0;
_4 = _1;
_25 = _24.fld0.1;
_26 = [_11,_24.fld0.3,_11,_24.fld0.3,_24.fld0.3,_24.fld0.3,_11,_11];
_11 = _24.fld0.3 + _24.fld0.3;
_3 = 2871617869053839767_i64 as isize;
_24.fld3 = [(-5359471475135285552_i64),(-5200786793561499_i64),(-2876754930192973063_i64),(-5936208045623324522_i64),(-7220294960317374622_i64),5094695102315545178_i64];
_21 = _24.fld0.1 & _24.fld0.1;
_28.1 = [_11,_24.fld0.3,_11,_24.fld0.3,_11,_11,_11,_11];
_8 = _6.1;
_24.fld4 = [_23.fld0,_23.fld0,_23.fld0,_23.fld0,_23.fld0,_23.fld0];
_11 = _24.fld0.3;
_14 = !1546563568_u32;
_11 = !_24.fld0.3;
Call(_26 = core::intrinsics::transmute(_28.1), ReturnTo(bb13), UnwindUnreachable())
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
Return()
}
bb19 = {
_24.fld0.1 = !_21;
_21 = !_31.1;
_28 = _6;
Goto(bb20)
}
bb20 = {
Call(_33 = dump_var(16_usize, 12_usize, Move(_12), 18_usize, Move(_18), 17_usize, Move(_17), 8_usize, Move(_8)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_33 = dump_var(16_usize, 9_usize, Move(_9), 14_usize, Move(_14), 21_usize, Move(_21), 19_usize, Move(_19)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_33 = dump_var(16_usize, 27_usize, Move(_27), 20_usize, Move(_20), 4_usize, Move(_4), 34_usize, _34), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: isize,mut _2: [i32; 8],mut _3: [i32; 8],mut _4: [i32; 8],mut _5: [i32; 8],mut _6: isize,mut _7: [i32; 8],mut _8: isize,mut _9: [i32; 8],mut _10: isize,mut _11: isize,mut _12: isize,mut _13: [i32; 8]) -> f64 {
mir! {
type RET = f64;
let _14: u128;
let _15: i128;
let _16: [char; 2];
let _17: *const [i32; 8];
let _18: u64;
let _19: i16;
let _20: char;
let _21: *mut [char; 2];
let _22: (i8, bool);
let _23: isize;
let _24: u16;
let _25: isize;
let _26: bool;
let _27: Adt48;
let _28: Adt48;
let _29: usize;
let _30: Adt55;
let _31: ();
let _32: ();
{
RET = 0_usize as f64;
_8 = _10 | _12;
_10 = 136895229303918014427203502163102248405_i128 as isize;
_12 = _8 << _11;
Goto(bb1)
}
bb1 = {
_4 = _2;
_2 = _4;
RET = 34645_u16 as f64;
_8 = _6 & _12;
RET = (-11021_i16) as f64;
_15 = (-130735295507993818337612087678583257947_i128);
RET = 682922960_u32 as f64;
_16 = ['\u{bc456}','\u{7cefe}'];
RET = (-840134972_i32) as f64;
_6 = _8;
_14 = !110633578618787410071854131413828211467_u128;
_12 = 18_i8 as isize;
match _15 {
0 => bb2,
1 => bb3,
209547071412944645125762519753184953509 => bb5,
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
_15 = (-13278463563766359375490252275822353027_i128);
_4 = [(-1163799813_i32),1047693340_i32,933218270_i32,(-1795152347_i32),(-803867254_i32),189783269_i32,(-742324147_i32),2144336601_i32];
_11 = _8 ^ _8;
_9 = [876641970_i32,(-1181689280_i32),956337064_i32,1911798654_i32,1026099709_i32,(-685185736_i32),81611646_i32,(-312269761_i32)];
_10 = _11 + _6;
_18 = 3415720846671598681_u64;
_5 = [(-1634318198_i32),(-1406941911_i32),(-1330864586_i32),(-697968092_i32),(-2048531324_i32),(-605509277_i32),(-1357321770_i32),168469240_i32];
_17 = core::ptr::addr_of!(_13);
_9 = [1997977114_i32,(-1944332046_i32),(-1852312392_i32),1657498642_i32,1087693255_i32,125120595_i32,(-1747618487_i32),(-1587567715_i32)];
_19 = (-2012353514_i32) as i16;
_11 = 129060875_u32 as isize;
_18 = 11881195932143404987_u64 - 8266745629738046199_u64;
_9 = [582910354_i32,692937407_i32,1780411531_i32,(-2118387390_i32),(-1022323906_i32),(-1498030038_i32),(-1149094760_i32),(-1350145876_i32)];
_20 = '\u{3c8bd}';
_14 = !187913700108189013473999336894774170288_u128;
_10 = _20 as isize;
_5 = (*_17);
Goto(bb6)
}
bb6 = {
_14 = 12160_u16 as u128;
_18 = !16647689283556906164_u64;
_12 = -_8;
match _15 {
0 => bb5,
327003903357172104087884355155945858429 => bb7,
_ => bb4
}
}
bb7 = {
_18 = _19 as u64;
_15 = -4259727413683942395673231741808122132_i128;
_5 = [(-444792269_i32),(-1893982468_i32),(-1931120134_i32),(-1096148818_i32),2128869662_i32,177158504_i32,(-1931997760_i32),425993695_i32];
_8 = 31354_u16 as isize;
_6 = _12;
_2 = [1325636981_i32,(-1370714946_i32),(-1103528720_i32),(-1140716339_i32),916374058_i32,34027761_i32,1189638968_i32,(-373424508_i32)];
_1 = 2171949247_u32 as isize;
(*_17) = [2147267310_i32,(-867724093_i32),2141378289_i32,717697415_i32,1998949626_i32,418587814_i32,(-186678566_i32),(-1011687444_i32)];
_8 = (-16_i8) as isize;
_22 = (107_i8, false);
_10 = !_6;
RET = _18 as f64;
_8 = !_6;
_21 = core::ptr::addr_of_mut!(_16);
_11 = _15 as isize;
_1 = _6;
_23 = _12;
_18 = 2718834483870758965_u64;
Goto(bb8)
}
bb8 = {
_18 = 16455959517346008073_u64 << _12;
_12 = _1;
_5 = _3;
_18 = 12475360489381442300_u64 - 13220561384170375566_u64;
_11 = _23 - _23;
_10 = 58574_u16 as isize;
_14 = 125558787157206303551022439203093462674_u128;
_13 = _2;
RET = _15 as f64;
_22 = ((-29_i8), false);
_12 = _15 as isize;
_6 = -_1;
_2 = _13;
_24 = 36815_u16 - 14231_u16;
_7 = _13;
_21 = core::ptr::addr_of_mut!((*_21));
Goto(bb9)
}
bb9 = {
_9 = (*_17);
_21 = core::ptr::addr_of_mut!(_16);
_5 = (*_17);
_11 = _6 * _23;
(*_21) = [_20,_20];
(*_21) = [_20,_20];
(*_17) = [369834837_i32,(-1117094346_i32),1193603730_i32,(-989133167_i32),1363526907_i32,(-194498422_i32),1231025581_i32,(-2016748517_i32)];
_5 = [(-1656006994_i32),(-1470377309_i32),(-545533675_i32),(-26742031_i32),418812690_i32,(-910544197_i32),(-2029104792_i32),(-500056096_i32)];
_12 = _19 as isize;
_23 = -_11;
_8 = -_23;
_27.fld0 = (RET, (*_17));
_16 = [_20,_20];
_10 = _1;
_28.fld0 = _27.fld0;
_20 = '\u{363ec}';
_27 = Adt48 { fld0: _28.fld0 };
RET = 4_usize as f64;
_26 = !_22.1;
_17 = core::ptr::addr_of!(_2);
match _14 {
125558787157206303551022439203093462674 => bb11,
_ => bb10
}
}
bb10 = {
_15 = (-13278463563766359375490252275822353027_i128);
_4 = [(-1163799813_i32),1047693340_i32,933218270_i32,(-1795152347_i32),(-803867254_i32),189783269_i32,(-742324147_i32),2144336601_i32];
_11 = _8 ^ _8;
_9 = [876641970_i32,(-1181689280_i32),956337064_i32,1911798654_i32,1026099709_i32,(-685185736_i32),81611646_i32,(-312269761_i32)];
_10 = _11 + _6;
_18 = 3415720846671598681_u64;
_5 = [(-1634318198_i32),(-1406941911_i32),(-1330864586_i32),(-697968092_i32),(-2048531324_i32),(-605509277_i32),(-1357321770_i32),168469240_i32];
_17 = core::ptr::addr_of!(_13);
_9 = [1997977114_i32,(-1944332046_i32),(-1852312392_i32),1657498642_i32,1087693255_i32,125120595_i32,(-1747618487_i32),(-1587567715_i32)];
_19 = (-2012353514_i32) as i16;
_11 = 129060875_u32 as isize;
_18 = 11881195932143404987_u64 - 8266745629738046199_u64;
_9 = [582910354_i32,692937407_i32,1780411531_i32,(-2118387390_i32),(-1022323906_i32),(-1498030038_i32),(-1149094760_i32),(-1350145876_i32)];
_20 = '\u{3c8bd}';
_14 = !187913700108189013473999336894774170288_u128;
_10 = _20 as isize;
_5 = (*_17);
Goto(bb6)
}
bb11 = {
_17 = core::ptr::addr_of!(_5);
(*_21) = [_20,_20];
_25 = _11 | _6;
_8 = _11;
_30.fld4 = [5_usize,4_usize,2_usize,4_usize,3_usize,0_usize];
_6 = _1 | _25;
RET = _10 as f64;
_11 = 2257419373_u32 as isize;
_19 = 22807_i16;
_30.fld0.2 = core::ptr::addr_of!(_18);
_30.fld1 = core::ptr::addr_of_mut!(_16);
_10 = _6 * _8;
_7 = [1031648960_i32,(-1719215844_i32),1228007010_i32,(-30790837_i32),(-1254011336_i32),2046907347_i32,(-231993535_i32),(-800644411_i32)];
Goto(bb12)
}
bb12 = {
Call(_31 = dump_var(17_usize, 2_usize, Move(_2), 16_usize, Move(_16), 15_usize, Move(_15), 26_usize, Move(_26)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_31 = dump_var(17_usize, 1_usize, Move(_1), 10_usize, Move(_10), 8_usize, Move(_8), 7_usize, Move(_7)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_31 = dump_var(17_usize, 3_usize, Move(_3), 25_usize, Move(_25), 23_usize, Move(_23), 14_usize, Move(_14)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: isize,mut _2: i32,mut _3: u32,mut _4: [i32; 8],mut _5: u32,mut _6: f64,mut _7: f64,mut _8: bool,mut _9: bool,mut _10: isize,mut _11: isize,mut _12: isize) -> [i32; 8] {
mir! {
type RET = [i32; 8];
let _13: isize;
let _14: Adt44;
let _15: (f32,);
let _16: u32;
let _17: [i64; 6];
let _18: Adt53;
let _19: u32;
let _20: Adt44;
let _21: bool;
let _22: f64;
let _23: f64;
let _24: (f32,);
let _25: isize;
let _26: bool;
let _27: isize;
let _28: *mut [char; 2];
let _29: (f64, [i32; 8]);
let _30: [u64; 5];
let _31: i8;
let _32: isize;
let _33: Adt55;
let _34: f32;
let _35: Adt45;
let _36: u8;
let _37: bool;
let _38: *const [i32; 8];
let _39: isize;
let _40: u32;
let _41: bool;
let _42: f64;
let _43: Adt45;
let _44: [char; 2];
let _45: char;
let _46: ();
let _47: ();
{
RET = [_2,_2,_2,_2,_2,_2,_2,_2];
_8 = !_9;
_7 = 33_i8 as f64;
_11 = _2 as isize;
_8 = _9;
_9 = _8 & _8;
Goto(bb1)
}
bb1 = {
_10 = _2 as isize;
_11 = _10;
_1 = _12 * _12;
_6 = -_7;
RET = [_2,_2,_2,_2,_2,_2,_2,_2];
_7 = _6 * _6;
Goto(bb2)
}
bb2 = {
_9 = !_8;
_2 = -1145372142_i32;
_8 = _9 | _9;
_11 = _1;
_12 = _1 * _1;
_7 = -_6;
_2 = '\u{f582d}' as i32;
_7 = _6;
_5 = _3 ^ _3;
Goto(bb3)
}
bb3 = {
_1 = _11 << _12;
_5 = 49061_u16 as u32;
RET = _4;
_13 = _1;
RET = [_2,_2,_2,_2,_2,_2,_2,_2];
RET = [_2,_2,_2,_2,_2,_2,_2,_2];
_11 = _12;
_10 = !_1;
RET = [_2,_2,_2,_2,_2,_2,_2,_2];
_7 = _6;
_2 = (-31746_i16) as i32;
_11 = !_1;
_12 = -_10;
_10 = !_1;
_8 = _3 >= _3;
_8 = _13 > _13;
RET = [_2,_2,_2,_2,_2,_2,_2,_2];
_15.0 = _2 as f32;
_12 = _2 as isize;
_9 = _8;
_15.0 = _11 as f32;
RET = _4;
Call(_16 = core::intrinsics::transmute(_3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_16 = _11 as u32;
_16 = _3 ^ _3;
_1 = _11;
_21 = _10 > _1;
Goto(bb5)
}
bb5 = {
_5 = !_16;
_21 = _9 & _9;
_19 = _5 >> _11;
_21 = _8;
_2 = -1938421374_i32;
_3 = !_19;
_11 = _10 >> _1;
_1 = _13 >> _19;
_16 = _21 as u32;
_9 = !_8;
_9 = _21 | _21;
_12 = -_11;
_12 = 111_u8 as isize;
_17 = [(-7732853171713252207_i64),(-8354635723297890445_i64),(-6418355012841398357_i64),(-3121353679748879867_i64),(-1648492758587262353_i64),(-7508645823386362903_i64)];
RET = _4;
_21 = !_9;
_17 = [5044170965505534035_i64,3389631143629187857_i64,(-8972017439672156164_i64),(-6173698056728211294_i64),5050421925860925601_i64,6336725104024866399_i64];
_22 = _11 as f64;
_24.0 = _15.0 * _15.0;
_24 = (_15.0,);
_23 = _22;
_21 = !_9;
_4 = [_2,_2,_2,_2,_2,_2,_2,_2];
_6 = (-3256433392293321417_i64) as f64;
RET = [_2,_2,_2,_2,_2,_2,_2,_2];
_15 = (_24.0,);
_5 = (-17040_i16) as u32;
Goto(bb6)
}
bb6 = {
_25 = '\u{2dea9}' as isize;
_9 = _23 != _22;
_1 = _11 & _11;
_1 = _10 ^ _13;
_11 = _1;
RET = _4;
RET = [_2,_2,_2,_2,_2,_2,_2,_2];
_6 = _22 + _22;
_15.0 = _24.0;
_13 = _1 >> _1;
_10 = -_11;
_7 = _15.0 as f64;
_9 = _6 != _22;
_15.0 = _24.0;
_3 = _16;
_12 = _15.0 as isize;
_17 = [(-6029545501751058683_i64),2290795079899724896_i64,5773819470885432290_i64,(-2823148454599408734_i64),2437444083514749490_i64,5229977713649102152_i64];
_1 = -_13;
_16 = _3;
_15.0 = _24.0;
_25 = 4465200024943733247_u64 as isize;
_29.0 = _22 - _22;
_13 = _11 >> _1;
_29.1 = RET;
_27 = -_12;
_6 = -_7;
_22 = 8467_i16 as f64;
Call(_29.1 = core::intrinsics::transmute(_4), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_26 = _21;
_1 = _13 - _12;
_15 = _24;
_19 = !_16;
_8 = _9;
_15 = _24;
_31 = 60_i8;
_5 = _16 * _3;
_31 = _2 as i8;
_33.fld3 = [2460930202977985482_i64,(-8572624147664731848_i64),3826218367157710090_i64,6318459681432480490_i64,(-192921972910292482_i64),(-7000738455827108663_i64)];
_33.fld0.2 = core::ptr::addr_of!(_33.fld0.4);
_33.fld0.2 = core::ptr::addr_of!(_33.fld5);
_32 = !_12;
_33.fld0.0 = !_9;
_21 = !_8;
_11 = -_27;
_8 = _9 & _33.fld0.0;
_9 = !_33.fld0.0;
Call(_33.fld4 = fn19(_32, _5, _15.0, _16, _29.0, _12, _11, _23), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_33.fld0.4 = !6757543562555973938_u64;
Goto(bb9)
}
bb9 = {
_6 = _29.0 - _29.0;
_33.fld0.4 = !11726267640848641939_u64;
_22 = _6 - _29.0;
_12 = _10 >> _5;
_33.fld4 = [5_usize,2052480103981260797_usize,7_usize,6003923687353830630_usize,999422403478798982_usize,16080721420040939363_usize];
_2 = -555496755_i32;
_16 = '\u{73f58}' as u32;
_25 = !_1;
_33.fld5 = _2 as u64;
_4 = [_2,_2,_2,_2,_2,_2,_2,_2];
_33.fld5 = _33.fld0.4;
_30 = [_33.fld5,_33.fld5,_33.fld0.4,_33.fld5,_33.fld0.4];
_11 = _31 as isize;
_33.fld0.1 = _13 == _13;
Call(_27 = core::intrinsics::bswap(_10), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_32 = _27 | _27;
Goto(bb11)
}
bb11 = {
_33.fld0.3 = 127603206588422186949415116191203682631_i128 as i32;
_34 = _15.0 - _15.0;
Goto(bb12)
}
bb12 = {
_29.1 = [_2,_33.fld0.3,_2,_33.fld0.3,_33.fld0.3,_2,_2,_33.fld0.3];
_33.fld0.4 = _33.fld5 & _33.fld5;
_31 = (-109_i8);
_15.0 = _24.0;
_3 = _19;
_16 = _19 ^ _3;
_22 = 41_u8 as f64;
_29 = (_6, _4);
_23 = _7;
_3 = 703923867009754751_i64 as u32;
_29.0 = _7 + _6;
_23 = _7 * _29.0;
_29 = (_7, _4);
_33.fld0.4 = _31 as u64;
_2 = _33.fld0.3;
_29.1 = RET;
_33.fld0.2 = core::ptr::addr_of!(_33.fld5);
_27 = _13;
_24 = (_15.0,);
_29.1 = [_2,_2,_2,_33.fld0.3,_2,_33.fld0.3,_2,_2];
match _31 {
0 => bb8,
340282366920938463463374607431768211347 => bb13,
_ => bb9
}
}
bb13 = {
_36 = 92_u8 << _16;
_6 = _29.0 - _23;
_25 = _1 >> _13;
_25 = !_12;
RET = _4;
_33.fld0.2 = core::ptr::addr_of!(_33.fld0.4);
_2 = _33.fld0.3;
_33.fld0.1 = _36 <= _36;
_33.fld2 = 324473328417464217619977669148747441124_u128 >> _16;
_8 = _9;
_41 = !_26;
_41 = _8;
_34 = _15.0;
_10 = _5 as isize;
_29.1 = [_2,_2,_33.fld0.3,_33.fld0.3,_33.fld0.3,_2,_2,_33.fld0.3];
_40 = _5 ^ _16;
_24 = _15;
_26 = _15.0 >= _34;
_40 = 6_usize as u32;
_40 = !_16;
_29.1 = [_33.fld0.3,_33.fld0.3,_2,_2,_2,_2,_33.fld0.3,_2];
Goto(bb14)
}
bb14 = {
_3 = _16 + _40;
_26 = _33.fld0.1;
_5 = !_19;
Goto(bb15)
}
bb15 = {
Call(_46 = dump_var(18_usize, 4_usize, Move(_4), 12_usize, Move(_12), 19_usize, Move(_19), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_46 = dump_var(18_usize, 1_usize, Move(_1), 36_usize, Move(_36), 10_usize, Move(_10), 40_usize, Move(_40)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(18_usize, 25_usize, Move(_25), 31_usize, Move(_31), 11_usize, Move(_11), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: isize,mut _2: u32,mut _3: f32,mut _4: u32,mut _5: f64,mut _6: isize,mut _7: isize,mut _8: f64) -> [usize; 6] {
mir! {
type RET = [usize; 6];
let _9: isize;
let _10: (f64, [i32; 8]);
let _11: [char; 2];
let _12: bool;
let _13: isize;
let _14: i16;
let _15: f32;
let _16: (f32,);
let _17: (f32,);
let _18: Adt47;
let _19: f32;
let _20: f32;
let _21: [i32; 8];
let _22: Adt55;
let _23: [i32; 5];
let _24: bool;
let _25: (f64, [i32; 8]);
let _26: (f64, [i32; 8]);
let _27: Adt49;
let _28: *const u16;
let _29: *const *mut [char; 2];
let _30: u16;
let _31: i32;
let _32: i32;
let _33: u32;
let _34: Adt51;
let _35: f64;
let _36: *const (bool, bool, *const u64, i32, u64);
let _37: bool;
let _38: (bool, bool, *const u64, i32, u64);
let _39: isize;
let _40: bool;
let _41: [i64; 4];
let _42: *mut u8;
let _43: Adt58;
let _44: Adt51;
let _45: (i8, bool);
let _46: char;
let _47: f64;
let _48: u128;
let _49: isize;
let _50: ();
let _51: ();
{
_10.0 = _5;
_2 = _4;
RET = [12697872896533230027_usize,5_usize,7_usize,5_usize,4587423223940406975_usize,4_usize];
_2 = 27577_i16 as u32;
_1 = -_6;
_11 = ['\u{28dc8}','\u{5c9d4}'];
_8 = _10.0 - _5;
_2 = !_4;
_8 = -_10.0;
_3 = _1 as f32;
_12 = _5 < _10.0;
Goto(bb1)
}
bb1 = {
_9 = _6 ^ _1;
_4 = 178514783_i32 as u32;
Goto(bb2)
}
bb2 = {
_2 = (-80_i8) as u32;
_10.0 = -_8;
_15 = 907745399_i32 as f32;
_10.1 = [1049859042_i32,(-1636105878_i32),2077951145_i32,1038524447_i32,(-1203057110_i32),(-1852184251_i32),(-314741113_i32),1777716407_i32];
_15 = -_3;
_14 = 16545314236991387692_u64 as i16;
_4 = 192_u8 as u32;
_10.1 = [(-1986470522_i32),813259328_i32,1808732346_i32,1458337831_i32,(-562010777_i32),(-1472065894_i32),(-1952350450_i32),1250690088_i32];
_15 = -_3;
_16 = (_15,);
_11 = ['\u{f59e9}','\u{330c5}'];
_6 = _1;
_7 = -_6;
_12 = !false;
_4 = _2;
_5 = _8 - _10.0;
_10.1 = [1832835327_i32,1441979370_i32,(-639122980_i32),371585768_i32,1916307460_i32,(-679030323_i32),1216570569_i32,(-128445016_i32)];
_17 = _16;
RET = [13550052270980097636_usize,6_usize,12110354276964189835_usize,12785537248640928802_usize,2396490925601666499_usize,3_usize];
_3 = _16.0 - _15;
RET = [3_usize,7909561114154293859_usize,5_usize,5887195196740422007_usize,16101437455760914860_usize,1171496378623028701_usize];
_10.0 = _8 * _8;
_10.0 = _5;
_10.1 = [1212928219_i32,(-1038866371_i32),(-1604580234_i32),(-1659747951_i32),427506369_i32,(-48846579_i32),(-1593445894_i32),280778000_i32];
_11 = ['\u{f50b3}','\u{3a2a}'];
_3 = 383514294_i32 as f32;
_11 = ['\u{76e01}','\u{3feda}'];
_16 = (_17.0,);
Goto(bb3)
}
bb3 = {
_7 = _6;
_10.1 = [1593403729_i32,(-684685629_i32),909934572_i32,(-1416844892_i32),898309674_i32,1006220203_i32,1832979890_i32,(-1197875085_i32)];
_14 = (-22990_i16);
_14 = 42289_u16 as i16;
RET = [6_usize,6519378154473602363_usize,5_usize,7_usize,4634502420369044837_usize,7975432866612797073_usize];
_5 = -_8;
_17.0 = 68_i8 as f32;
Call(_9 = core::intrinsics::transmute(_1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_4 = _2;
_10.1 = [1524540062_i32,1134744356_i32,693340943_i32,1215673837_i32,399329537_i32,584534841_i32,(-241923822_i32),1402427113_i32];
_12 = false;
_12 = _7 == _9;
_8 = _10.0 + _10.0;
_20 = 2297554692531610740_usize as f32;
_3 = _15;
_7 = '\u{69241}' as isize;
_21 = [(-205522564_i32),1803216030_i32,1165442290_i32,(-1463167257_i32),(-1903366426_i32),(-695714613_i32),(-926572585_i32),770902086_i32];
_6 = _9;
_10.0 = 17615846883831129346767158490311417790_i128 as f64;
_10.0 = -_5;
_16.0 = _3;
_22.fld0.1 = _12;
_23 = [(-1618337901_i32),(-1817901850_i32),1463086611_i32,(-1767064678_i32),1677272733_i32];
Goto(bb5)
}
bb5 = {
_22.fld0.3 = (-1845646923_i32) * 1688000532_i32;
_17 = _16;
_22.fld0.2 = core::ptr::addr_of!(_22.fld0.4);
_22.fld5 = 7264043041008162328_u64 * 7837873453477519543_u64;
_24 = !_12;
_25 = (_8, _21);
_22.fld3 = [(-1570216849622502211_i64),(-3839168662703914803_i64),2074764981668651429_i64,3121780237689717075_i64,(-3372069971831040729_i64),5211048399242771909_i64];
_6 = _1 >> _9;
_22.fld0.4 = !_22.fld5;
_2 = !_4;
_16.0 = _15 + _15;
_22.fld0.4 = _22.fld5;
_22.fld4 = RET;
_12 = _24;
_22.fld0.0 = _22.fld0.1 & _22.fld0.1;
_10 = (_8, _25.1);
_22.fld3 = [1358614952725041617_i64,(-9142904775867050649_i64),4332769209930420241_i64,7698174506063182581_i64,(-2792111297453661503_i64),(-4557455673746869926_i64)];
_6 = 6944405070611630194_i64 as isize;
_22.fld0.1 = _9 != _1;
_20 = _22.fld0.3 as f32;
_27 = Adt49 { fld0: 406050555518011299_usize };
_15 = _27.fld0 as f32;
_18 = Adt47::Variant1 { fld0: _22.fld0.1,fld1: _22.fld0.3 };
_16.0 = _17.0 * _17.0;
_10 = (_5, _25.1);
SetDiscriminant(_18, 0);
Goto(bb6)
}
bb6 = {
_25.1 = [_22.fld0.3,_22.fld0.3,_22.fld0.3,_22.fld0.3,_22.fld0.3,_22.fld0.3,_22.fld0.3,_22.fld0.3];
_22.fld1 = core::ptr::addr_of_mut!(_11);
_22.fld0.0 = _12;
_29 = core::ptr::addr_of!(place!(Field::<*mut [char; 2]>(Variant(_18, 0), 0)));
place!(Field::<*mut [char; 2]>(Variant(_18, 0), 0)) = core::ptr::addr_of_mut!(_11);
_10 = (_25.0, _21);
RET = _22.fld4;
_28 = core::ptr::addr_of!(_30);
_15 = -_16.0;
_10 = (_8, _21);
_16.0 = -_15;
_31 = 19_i8 as i32;
_26.1 = [_22.fld0.3,_31,_22.fld0.3,_22.fld0.3,_31,_22.fld0.3,_31,_22.fld0.3];
_10.0 = _25.0 + _8;
_25 = (_10.0, _21);
_32 = _22.fld0.3 << _1;
_22.fld4 = [_27.fld0,_27.fld0,_27.fld0,_27.fld0,_27.fld0,_27.fld0];
_23 = [_32,_32,_32,_32,_32];
_25.1 = [_32,_32,_32,_32,_32,_32,_32,_32];
place!(Field::<isize>(Variant(_18, 0), 1)) = _9;
_34.fld1 = [76_u8,150_u8,162_u8];
_4 = 64901_u16 as u32;
_28 = core::ptr::addr_of!((*_28));
place!(Field::<isize>(Variant(_18, 0), 1)) = _1;
match _27.fld0 {
0 => bb4,
406050555518011299 => bb7,
_ => bb5
}
}
bb7 = {
_27.fld0 = !2_usize;
_26 = _10;
_9 = _1;
_4 = _16.0 as u32;
_31 = _32;
_8 = _25.0 + _5;
_27 = Adt49 { fld0: 2486457784238699937_usize };
place!(Field::<isize>(Variant(_18, 0), 1)) = -_1;
place!(Field::<*mut [char; 2]>(Variant(_18, 0), 0)) = _22.fld1;
_13 = _1 - _1;
_36 = core::ptr::addr_of!(_22.fld0);
_22.fld2 = !138453072327915560306550363040862157214_u128;
_15 = 3193146376809722762_i64 as f32;
SetDiscriminant(_18, 1);
_33 = 29809798437297079504339006417690248934_i128 as u32;
_19 = 201_u8 as f32;
_37 = _12 == (*_36).1;
_33 = _4 >> _31;
_10.0 = _26.0 - _8;
_34.fld1 = [253_u8,40_u8,178_u8];
_38.3 = -_31;
_21 = [_31,_31,_31,_38.3,_32,_38.3,_32,_38.3];
_34.fld5.fld0 = (-4621424647309268154_i64) as usize;
match _27.fld0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
2486457784238699937 => bb8,
_ => bb6
}
}
bb8 = {
_22.fld0.4 = _22.fld5 << _1;
(*_28) = 41130_u16 << _32;
_22.fld0.4 = _22.fld5 ^ _22.fld5;
_34.fld5 = Adt49 { fld0: _27.fld0 };
_36 = core::ptr::addr_of!(_38);
_41 = [(-1318485951170392956_i64),(-5089086559340148672_i64),2584829647555149757_i64,(-6733506274743721722_i64)];
Call(_9 = core::intrinsics::transmute(_22.fld5), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_16 = (_3,);
_34.fld1 = [69_u8,196_u8,251_u8];
Goto(bb10)
}
bb10 = {
(*_36).4 = '\u{829d7}' as u64;
_44.fld5.fld0 = _34.fld5.fld0;
_38.0 = _24;
(*_36).2 = core::ptr::addr_of!((*_36).4);
_44.fld2 = core::ptr::addr_of!(_22.fld1);
_25.1 = [_31,_32,(*_36).3,_32,_38.3,_31,_32,_38.3];
_27 = Adt49 { fld0: _44.fld5.fld0 };
Goto(bb11)
}
bb11 = {
_40 = !_37;
_25.0 = _26.0;
_44.fld2 = core::ptr::addr_of!(_22.fld1);
_29 = _44.fld2;
_35 = -_8;
_19 = _34.fld5.fld0 as f32;
_22.fld0.0 = (*_36).3 > _38.3;
_4 = _33;
_15 = _3;
_44.fld0 = _22.fld0.1;
_20 = _15;
(*_36) = _22.fld0;
(*_36).0 = !_24;
(*_36).2 = _22.fld0.2;
match _44.fld5.fld0 {
0 => bb4,
1 => bb12,
2 => bb13,
2486457784238699937 => bb15,
_ => bb14
}
}
bb12 = {
(*_36).4 = '\u{829d7}' as u64;
_44.fld5.fld0 = _34.fld5.fld0;
_38.0 = _24;
(*_36).2 = core::ptr::addr_of!((*_36).4);
_44.fld2 = core::ptr::addr_of!(_22.fld1);
_25.1 = [_31,_32,(*_36).3,_32,_38.3,_31,_32,_38.3];
_27 = Adt49 { fld0: _44.fld5.fld0 };
Goto(bb11)
}
bb13 = {
_27.fld0 = !2_usize;
_26 = _10;
_9 = _1;
_4 = _16.0 as u32;
_31 = _32;
_8 = _25.0 + _5;
_27 = Adt49 { fld0: 2486457784238699937_usize };
place!(Field::<isize>(Variant(_18, 0), 1)) = -_1;
place!(Field::<*mut [char; 2]>(Variant(_18, 0), 0)) = _22.fld1;
_13 = _1 - _1;
_36 = core::ptr::addr_of!(_22.fld0);
_22.fld2 = !138453072327915560306550363040862157214_u128;
_15 = 3193146376809722762_i64 as f32;
SetDiscriminant(_18, 1);
_33 = 29809798437297079504339006417690248934_i128 as u32;
_19 = 201_u8 as f32;
_37 = _12 == (*_36).1;
_33 = _4 >> _31;
_10.0 = _26.0 - _8;
_34.fld1 = [253_u8,40_u8,178_u8];
_38.3 = -_31;
_21 = [_31,_31,_31,_38.3,_32,_38.3,_32,_38.3];
_34.fld5.fld0 = (-4621424647309268154_i64) as usize;
match _27.fld0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
2486457784238699937 => bb8,
_ => bb6
}
}
bb14 = {
_22.fld0.3 = (-1845646923_i32) * 1688000532_i32;
_17 = _16;
_22.fld0.2 = core::ptr::addr_of!(_22.fld0.4);
_22.fld5 = 7264043041008162328_u64 * 7837873453477519543_u64;
_24 = !_12;
_25 = (_8, _21);
_22.fld3 = [(-1570216849622502211_i64),(-3839168662703914803_i64),2074764981668651429_i64,3121780237689717075_i64,(-3372069971831040729_i64),5211048399242771909_i64];
_6 = _1 >> _9;
_22.fld0.4 = !_22.fld5;
_2 = !_4;
_16.0 = _15 + _15;
_22.fld0.4 = _22.fld5;
_22.fld4 = RET;
_12 = _24;
_22.fld0.0 = _22.fld0.1 & _22.fld0.1;
_10 = (_8, _25.1);
_22.fld3 = [1358614952725041617_i64,(-9142904775867050649_i64),4332769209930420241_i64,7698174506063182581_i64,(-2792111297453661503_i64),(-4557455673746869926_i64)];
_6 = 6944405070611630194_i64 as isize;
_22.fld0.1 = _9 != _1;
_20 = _22.fld0.3 as f32;
_27 = Adt49 { fld0: 406050555518011299_usize };
_15 = _27.fld0 as f32;
_18 = Adt47::Variant1 { fld0: _22.fld0.1,fld1: _22.fld0.3 };
_16.0 = _17.0 * _17.0;
_10 = (_5, _25.1);
SetDiscriminant(_18, 0);
Goto(bb6)
}
bb15 = {
_27.fld0 = _34.fld5.fld0 * _44.fld5.fld0;
_33 = !_4;
_22.fld3 = [8332082745975900126_i64,(-783974554808922439_i64),(-5071641026760302491_i64),(-7966009272175346504_i64),(-8001253589280354366_i64),5832160704646490375_i64];
_22.fld4 = RET;
_27.fld0 = _44.fld5.fld0;
_45.0 = 116_i8 | (-36_i8);
_18 = Adt47::Variant1 { fld0: (*_36).0,fld1: _32 };
(*_28) = !24429_u16;
_40 = _25.0 > _25.0;
_22.fld0.0 = _38.1;
_46 = '\u{28fbd}';
(*_28) = !12300_u16;
(*_36).2 = core::ptr::addr_of!((*_36).4);
_22.fld1 = core::ptr::addr_of_mut!(_11);
_36 = core::ptr::addr_of!(_38);
_44.fld3 = (-2853311270772412965_i64) as u128;
(*_28) = 64929_u16 << _32;
(*_36) = (_40, _24, _22.fld0.2, Field::<i32>(Variant(_18, 1), 1), _22.fld5);
(*_29) = core::ptr::addr_of_mut!(_11);
_44.fld0 = _38.0;
_26.0 = -_8;
_5 = -_10.0;
_14 = 9554_i16 << _1;
_38.4 = _22.fld5;
Goto(bb16)
}
bb16 = {
Call(_50 = dump_var(19_usize, 33_usize, Move(_33), 7_usize, Move(_7), 24_usize, Move(_24), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_50 = dump_var(19_usize, 4_usize, Move(_4), 21_usize, Move(_21), 30_usize, Move(_30), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_50 = dump_var(19_usize, 2_usize, Move(_2), 41_usize, Move(_41), 51_usize, _51, 51_usize, _51), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{bd715}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box(16_i8), std::hint::black_box(115490757380409163919490789108618841884_u128), std::hint::black_box(13502314572572357768_u64), std::hint::black_box((-4030557247597907357_i64)), std::hint::black_box((-99548933771731504604087439174027565382_i128)), std::hint::black_box(0_usize), std::hint::black_box(48_u8), std::hint::black_box(4126262209_u32));
                
            }
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt42{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt42 {
fld0: *mut i16,
fld1: ([i32; 8], isize, (i8, bool)),
fld2: [i64; 4],
}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt43{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt43 {
fld0: *const u128,
fld1: [char; 2],
}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
	Self::Variant1{fld0,fld1,fld2,}=>{
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
},
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: i64,
fld1: *mut usize,
fld2: u8,
fld3: (*const u16, u32),
fld4: i16,
fld5: [usize; 6],

},
Variant1{
fld0: *const *mut [char; 2],
fld1: *const u128,
fld2: [i64; 4],

},
Variant2{
fld0: *mut u8,

},
Variant3{
fld0: Adt42,
fld1: *const u128,
fld2: *const u64,
fld3: usize,
fld4: u64,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: bool,
fld1: u8,
fld2: *mut u8,
fld3: i128,
fld4: [char; 2],
fld5: usize,

},
Variant1{
fld0: [u64; 5],
fld1: [u8; 3],
fld2: [i32; 8],
fld3: (bool, bool, *const u64, i32, u64),

},
Variant2{
fld0: (f32,),
fld1: usize,
fld2: Adt43,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: *mut [char; 2],
fld1: f32,
fld2: isize,
fld3: u16,
fld4: [i64; 4],
fld5: i32,

},
Variant1{
fld0: [u8; 3],
fld1: char,
fld2: *mut usize,
fld3: (f64, [i32; 8]),
fld4: i16,
fld5: *mut u8,
fld6: [i64; 4],
fld7: [u64; 5],

},
Variant2{
fld0: bool,
fld1: usize,
fld2: (*const u16, u32),
fld3: u128,
fld4: i16,
fld5: [i32; 5],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: *mut [char; 2],
fld1: isize,

},
Variant1{
fld0: bool,
fld1: i32,

},
Variant2{
fld0: Adt44,
fld1: Adt46,
fld2: u32,
fld3: (f64, [i32; 8]),

},
Variant3{
fld0: [i32; 5],
fld1: *const u128,
fld2: isize,
fld3: *mut i16,
fld4: (i8, bool),
fld5: [i64; 4],
fld6: Adt44,
fld7: *const u16,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: (f64, [i32; 8]),
}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: usize,
}
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
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: bool,
fld1: Adt48,
fld2: u8,
fld3: *const u128,
fld4: Adt49,

},
Variant1{
fld0: *mut i16,
fld1: [i64; 6],
fld2: *mut [char; 2],
fld3: f32,
fld4: (bool, bool, *const u64, i32, u64),

},
Variant2{
fld0: i16,
fld1: i32,
fld2: [i64; 6],
fld3: *const (bool, bool, *const u64, i32, u64),

},
Variant3{
fld0: Adt47,
fld1: char,
fld2: Adt43,
fld3: [i32; 8],
fld4: i128,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: bool,
fld1: [u8; 3],
fld2: *const *mut [char; 2],
fld3: u128,
fld4: [i64; 6],
fld5: Adt49,
fld6: *mut i16,
}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: f64,
fld1: Adt42,
fld2: u128,
fld3: i8,
fld4: (*const u16, u32),
fld5: Adt47,
fld6: u8,
}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: (*const u16, u32),
fld1: *const u64,
fld2: isize,
fld3: Adt45,
fld4: *const *mut [char; 2],

},
Variant1{
fld0: *mut [char; 2],

},
Variant2{
fld0: *mut i16,
fld1: Adt51,
fld2: *const (bool, bool, *const u64, i32, u64),

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: i32,

},
Variant1{
fld0: [i32; 5],
fld1: *mut u8,
fld2: Adt42,
fld3: *const u128,
fld4: i16,
fld5: *mut i16,

},
Variant2{
fld0: i64,
fld1: (bool, bool, *const u64, i32, u64),
fld2: *const u64,
fld3: u32,
fld4: (f64, [i32; 8]),

},
Variant3{
fld0: f64,
fld1: *const u16,
fld2: Adt42,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: (bool, bool, *const u64, i32, u64),
fld1: *mut [char; 2],
fld2: u128,
fld3: [i64; 6],
fld4: [usize; 6],
fld5: u64,
}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
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
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: Adt47,
fld1: [u8; 3],
fld2: [i64; 6],
fld3: ([i32; 8], isize, (i8, bool)),

},
Variant1{
fld0: [i32; 8],

},
Variant2{
fld0: Adt43,
fld1: Adt44,
fld2: [i32; 8],
fld3: *const *mut [char; 2],
fld4: *mut u8,

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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: [u8; 3],
fld1: [i64; 4],
fld2: *mut u8,
fld3: Adt46,
fld4: Adt51,
fld5: f32,
fld6: i64,
fld7: i128,

},
Variant1{
fld0: [usize; 6],
fld1: Adt46,

},
Variant2{
fld0: (i8, bool),

},
Variant3{
fld0: i32,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: [char; 2],
fld1: Adt44,

},
Variant1{
fld0: f64,

},
Variant2{
fld0: usize,
fld1: u32,
fld2: *mut u8,
fld3: Adt44,
fld4: Adt57,
fld5: i128,
fld6: f64,

},
Variant3{
fld0: Adt47,

}}

