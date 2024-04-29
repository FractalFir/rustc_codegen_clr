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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> u128 {
mir! {
type RET = u128;
let _15: isize;
let _16: [u8; 7];
let _17: char;
let _18: u8;
let _19: f32;
let _20: *mut isize;
let _21: i128;
let _22: i64;
let _23: [char; 5];
let _24: Adt46;
let _25: [i32; 6];
let _26: *const (bool, (*mut char, [usize; 7]), i64, i8, i16, i32, [u8; 4]);
let _27: char;
let _28: &'static isize;
let _29: usize;
let _30: *mut *mut isize;
let _31: Adt50;
let _32: (f32, u8, i32, u8);
let _33: (f64, (*mut char, [usize; 7]), i8, char);
let _34: [u8; 4];
let _35: i128;
let _36: [i32; 8];
let _37: isize;
let _38: [usize; 7];
let _39: [i32; 8];
let _40: char;
let _41: Adt55;
let _42: *const (bool, (*mut char, [usize; 7]), i64, i8, i16, i32, [u8; 4]);
let _43: Adt43;
let _44: f64;
let _45: Adt41;
let _46: *mut char;
let _47: (&'static isize, f32);
let _48: ();
let _49: ();
{
_5 = 160_u8 as i16;
_7 = 8313463607285387069_u64 as i64;
_15 = 9223372036854775807_isize << _5;
_6 = -874734388_i32;
_11 = 28613_u16 - 32557_u16;
_4 = 81_i8;
_4 = _11 as i8;
_3 = -_15;
_2 = '\u{8aed4}';
_1 = false;
_9 = _4 as usize;
_14 = !152079820292413738125983729353134355302_u128;
_7 = 4927944097558338088_u64 as i64;
_3 = _15;
_13 = !10757984928902286462_u64;
RET = _14;
_8 = 161178116473679210073835282658414566512_i128 * (-158948260491600494978750827281243846899_i128);
_13 = !497895007376216994_u64;
Call(_13 = core::intrinsics::bswap(14958212070030689826_u64), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _15 >> _11;
_16 = [122_u8,178_u8,89_u8,3_u8,91_u8,170_u8,33_u8];
_8 = _13 as i128;
_8 = _1 as i128;
_1 = false;
_12 = 4070901028_u32 ^ 2260552779_u32;
_15 = _3 >> _4;
_14 = RET & RET;
_19 = _4 as f32;
_17 = _2;
_21 = _8 - _8;
_20 = core::ptr::addr_of_mut!(_3);
_23 = [_2,_2,_17,_17,_17];
(*_20) = -_15;
(*_20) = _15;
_4 = _17 as i8;
RET = _14 - _14;
_16 = [99_u8,51_u8,129_u8,125_u8,0_u8,190_u8,171_u8];
_4 = 118_i8 >> (*_20);
_25 = [_6,_6,_6,_6,_6,_6];
Call((*_20) = core::intrinsics::transmute(_9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = -(-82_i8);
_18 = 117_u8;
(*_20) = _15 ^ _15;
_6 = -1854524533_i32;
_1 = (*_20) <= _3;
Goto(bb3)
}
bb3 = {
_19 = _11 as f32;
_27 = _2;
_6 = _5 as i32;
_28 = &_15;
_13 = !6022050913734755967_u64;
(*_20) = _15;
_13 = _21 as u64;
_19 = _13 as f32;
_33.0 = _19 as f64;
_15 = (*_20) + _3;
_11 = !44131_u16;
_24 = Adt46::Variant1 { fld0: _5,fld1: _33.0 };
_22 = _18 as i64;
_33.1.0 = core::ptr::addr_of_mut!(_2);
_32.2 = _6 & _6;
_32.1 = !_18;
_30 = core::ptr::addr_of_mut!(_20);
_10 = _32.1;
_33.2 = _4 + _4;
_27 = _17;
_20 = core::ptr::addr_of_mut!(_3);
_19 = _11 as f32;
_30 = core::ptr::addr_of_mut!(_20);
SetDiscriminant(_24, 2);
_4 = _33.2 >> _3;
_8 = !_21;
Goto(bb4)
}
bb4 = {
_5 = 14643_i16 - (-2812_i16);
RET = !_14;
RET = !_14;
_19 = _4 as f32;
_25 = [_32.2,_32.2,_6,_6,_6,_32.2];
_5 = (-7188_i16) + 16334_i16;
_24 = Adt46::Variant0 { fld0: _33.1.0 };
_37 = _18 as isize;
place!(Field::<*mut char>(Variant(_24, 0), 0)) = core::ptr::addr_of_mut!(_33.3);
_32.1 = _10;
_7 = _13 as i64;
_33.1.1 = [_9,_9,_9,_9,_9,_9,_9];
_15 = !_3;
_14 = !RET;
_35 = _8 + _21;
_30 = core::ptr::addr_of_mut!((*_30));
_9 = 17540002892706890280_usize;
_8 = _21 & _21;
SetDiscriminant(_24, 0);
place!(Field::<*mut char>(Variant(_24, 0), 0)) = _33.1.0;
_28 = &_37;
_33.2 = _33.0 as i8;
_39 = [_6,_32.2,_32.2,_32.2,_6,_6,_32.2,_32.2];
match _18 {
0 => bb1,
1 => bb5,
117 => bb7,
_ => bb6
}
}
bb5 = {
_19 = _11 as f32;
_27 = _2;
_6 = _5 as i32;
_28 = &_15;
_13 = !6022050913734755967_u64;
(*_20) = _15;
_13 = _21 as u64;
_19 = _13 as f32;
_33.0 = _19 as f64;
_15 = (*_20) + _3;
_11 = !44131_u16;
_24 = Adt46::Variant1 { fld0: _5,fld1: _33.0 };
_22 = _18 as i64;
_33.1.0 = core::ptr::addr_of_mut!(_2);
_32.2 = _6 & _6;
_32.1 = !_18;
_30 = core::ptr::addr_of_mut!(_20);
_10 = _32.1;
_33.2 = _4 + _4;
_27 = _17;
_20 = core::ptr::addr_of_mut!(_3);
_19 = _11 as f32;
_30 = core::ptr::addr_of_mut!(_20);
SetDiscriminant(_24, 2);
_4 = _33.2 >> _3;
_8 = !_21;
Goto(bb4)
}
bb6 = {
_3 = _15 >> _11;
_16 = [122_u8,178_u8,89_u8,3_u8,91_u8,170_u8,33_u8];
_8 = _13 as i128;
_8 = _1 as i128;
_1 = false;
_12 = 4070901028_u32 ^ 2260552779_u32;
_15 = _3 >> _4;
_14 = RET & RET;
_19 = _4 as f32;
_17 = _2;
_21 = _8 - _8;
_20 = core::ptr::addr_of_mut!(_3);
_23 = [_2,_2,_17,_17,_17];
(*_20) = -_15;
(*_20) = _15;
_4 = _17 as i8;
RET = _14 - _14;
_16 = [99_u8,51_u8,129_u8,125_u8,0_u8,190_u8,171_u8];
_4 = 118_i8 >> (*_20);
_25 = [_6,_6,_6,_6,_6,_6];
Call((*_20) = core::intrinsics::transmute(_9), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_33.1.0 = core::ptr::addr_of_mut!(_27);
_33.2 = _4 ^ _4;
_33.1.0 = Field::<*mut char>(Variant(_24, 0), 0);
SetDiscriminant(_24, 2);
Call(place!(Field::<[u16; 7]>(Variant(_24, 2), 1)) = fn1(_35, _1, _33.2, (*_20), (*_30), _11, (*_20), _3, _1, (*_30), _1, _17, _32.1, (*_30)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
place!(Field::<f32>(Variant(_24, 2), 4)) = _8 as f32;
place!(Field::<u32>(Variant(_24, 2), 2)) = _12;
_38 = _33.1.1;
_3 = !_15;
_33.3 = _27;
_17 = _33.3;
_19 = Field::<f32>(Variant(_24, 2), 4);
place!(Field::<*const usize>(Variant(_24, 2), 6)) = core::ptr::addr_of!(place!(Field::<usize>(Variant(_24, 2), 5)));
(*_30) = core::ptr::addr_of_mut!(_3);
_37 = _3;
RET = _11 as u128;
_23 = [_2,_2,_17,_17,_33.3];
_4 = _1 as i8;
RET = _9 as u128;
_33.0 = _4 as f64;
_40 = _27;
_32 = (Field::<f32>(Variant(_24, 2), 4), _10, _6, _10);
_15 = !_37;
_8 = !_21;
(*_30) = core::ptr::addr_of_mut!(_37);
_23 = [_2,_17,_17,_17,_2];
_32 = (_19, _10, _6, _10);
_28 = &_37;
_38 = [_9,_9,_9,_9,_9,_9,_9];
place!(Field::<*const usize>(Variant(_24, 2), 6)) = core::ptr::addr_of!(_9);
match _18 {
117 => bb9,
_ => bb4
}
}
bb9 = {
_12 = !Field::<u32>(Variant(_24, 2), 2);
place!(Field::<f32>(Variant(_24, 2), 4)) = _19;
_33.2 = _4;
_10 = !_32.3;
place!(Field::<usize>(Variant(_24, 2), 5)) = !_9;
place!(Field::<*mut char>(Variant(_24, 2), 0)) = core::ptr::addr_of_mut!(_2);
_24 = Adt46::Variant0 { fld0: _33.1.0 };
_33.3 = _17;
_23 = [_27,_33.3,_27,_33.3,_17];
_7 = -_22;
_32.0 = _10 as f32;
Goto(bb10)
}
bb10 = {
_30 = core::ptr::addr_of_mut!((*_30));
RET = _1 as u128;
_11 = _33.2 as u16;
_8 = !_21;
_1 = !false;
_15 = _3 >> _37;
_33.1.1 = [_9,_9,_9,_9,_9,_9,_9];
_3 = _37 | _15;
_32.2 = !_6;
_16 = [_10,_32.3,_18,_32.3,_10,_10,_10];
_29 = _9 / _9;
_7 = _10 as i64;
_33.0 = _12 as f64;
_44 = _12 as f64;
_47.1 = _32.2 as f32;
Goto(bb11)
}
bb11 = {
Call(_48 = dump_var(0_usize, 6_usize, Move(_6), 15_usize, Move(_15), 35_usize, Move(_35), 37_usize, Move(_37)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_48 = dump_var(0_usize, 11_usize, Move(_11), 27_usize, Move(_27), 18_usize, Move(_18), 14_usize, Move(_14)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_48 = dump_var(0_usize, 39_usize, Move(_39), 29_usize, Move(_29), 38_usize, Move(_38), 17_usize, Move(_17)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_48 = dump_var(0_usize, 25_usize, Move(_25), 12_usize, Move(_12), 49_usize, _49, 49_usize, _49), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i128,mut _2: bool,mut _3: i8,mut _4: isize,mut _5: *mut isize,mut _6: u16,mut _7: isize,mut _8: isize,mut _9: bool,mut _10: *mut isize,mut _11: bool,mut _12: char,mut _13: u8,mut _14: *mut isize) -> [u16; 7] {
mir! {
type RET = [u16; 7];
let _15: (*mut char, [usize; 7]);
let _16: isize;
let _17: Adt49;
let _18: Adt44;
let _19: bool;
let _20: i64;
let _21: (&'static isize, f32);
let _22: [i32; 6];
let _23: isize;
let _24: char;
let _25: i64;
let _26: Adt51;
let _27: i32;
let _28: char;
let _29: u128;
let _30: char;
let _31: i32;
let _32: i128;
let _33: *mut isize;
let _34: (*mut char, [usize; 7]);
let _35: Adt49;
let _36: bool;
let _37: [u16; 7];
let _38: *mut *mut isize;
let _39: isize;
let _40: Adt53;
let _41: [usize; 7];
let _42: Adt48;
let _43: u32;
let _44: (&'static isize, f32);
let _45: [u8; 7];
let _46: [u16; 7];
let _47: f32;
let _48: Adt42;
let _49: ();
let _50: ();
{
(*_14) = -_4;
_6 = 67_u16 - 5830_u16;
(*_10) = !_8;
(*_5) = -_8;
(*_14) = _8;
_11 = !_2;
_9 = (*_5) < _4;
_15.0 = core::ptr::addr_of_mut!(_12);
RET = [_6,_6,_6,_6,_6,_6,_6];
_7 = (*_10);
RET = [_6,_6,_6,_6,_6,_6,_6];
(*_5) = _2 as isize;
(*_5) = 198981784966463892402781919718678073981_u128 as isize;
_11 = _2 >= _2;
_13 = !136_u8;
_9 = _3 == _3;
_13 = 167_u8 * 25_u8;
_5 = _10;
(*_5) = _6 as isize;
_17.fld0 = [12859025606693740948_usize,10272549987994698338_usize,3850832341909951899_usize];
_17.fld1 = !_13;
_15.1 = [4_usize,7_usize,540254404404245634_usize,2_usize,440490917799526838_usize,1_usize,7_usize];
_15.0 = core::ptr::addr_of_mut!(_12);
Goto(bb1)
}
bb1 = {
_3 = -49_i8;
_18 = Adt44 { fld0: _3 };
_4 = (*_14) | (*_14);
_12 = '\u{1dfd6}';
_17.fld2 = [_17.fld1,_13,_17.fld1,_17.fld1,_13,_17.fld1,_13];
Goto(bb2)
}
bb2 = {
_17.fld2 = [_17.fld1,_13,_17.fld1,_17.fld1,_17.fld1,_13,_17.fld1];
_17.fld1 = !_13;
_18.fld0 = _3 + _3;
(*_14) = _4;
_19 = _11 >= _2;
(*_14) = _7 * _8;
_6 = 1576472758_i32 as u16;
(*_14) = _7;
_14 = _10;
_21.1 = _8 as f32;
_15.1 = [17090098771242750040_usize,1948223380997904445_usize,6_usize,7_usize,1_usize,0_usize,7_usize];
_21.0 = &(*_5);
_17.fld2 = [_13,_17.fld1,_13,_13,_13,_17.fld1,_17.fld1];
_21.0 = &_8;
Goto(bb3)
}
bb3 = {
_8 = 7429801013484990621_u64 as isize;
_11 = _19 & _19;
(*_14) = _4 | _4;
_20 = 3380996943177465681_i64 >> (*_14);
(*_5) = _7;
_19 = !_9;
RET = [_6,_6,_6,_6,_6,_6,_6];
(*_5) = _20 as isize;
_17.fld1 = _13 ^ _13;
_22 = [1753209746_i32,(-1709617044_i32),(-497841877_i32),(-282293191_i32),316049263_i32,1364011420_i32];
(*_10) = _4 - _4;
_8 = -(*_5);
_9 = (*_10) < _7;
_17.fld0 = [3_usize,1525992042112956631_usize,4536357214086320535_usize];
_11 = !_19;
(*_5) = _7;
_15.0 = core::ptr::addr_of_mut!(_12);
(*_10) = -_7;
_22 = [(-1825675435_i32),953965410_i32,1498493351_i32,1148257059_i32,123685040_i32,1342657469_i32];
_23 = -(*_5);
_23 = _9 as isize;
_16 = _21.1 as isize;
(*_10) = _23;
_11 = _2;
_21.1 = 17376528475542786564_u64 as f32;
_24 = _12;
_18 = Adt44 { fld0: _3 };
_3 = _1 as i8;
Goto(bb4)
}
bb4 = {
_21.0 = &_7;
_20 = 3450685203697470241_usize as i64;
_14 = _10;
_21.0 = &(*_10);
_15.1 = [0_usize,3005969557630766462_usize,16610677294625542368_usize,824361900356509635_usize,4_usize,3_usize,5_usize];
(*_5) = _23 * _23;
_17.fld2 = [_17.fld1,_17.fld1,_17.fld1,_13,_17.fld1,_17.fld1,_17.fld1];
_18 = Adt44 { fld0: _3 };
_10 = core::ptr::addr_of_mut!((*_10));
_17.fld2 = [_13,_17.fld1,_17.fld1,_13,_13,_17.fld1,_17.fld1];
_19 = !_11;
_21.0 = &_7;
(*_10) = !_23;
_7 = (*_5);
_6 = 6_usize as u16;
_31 = !1570107038_i32;
(*_14) = -_8;
Call((*_10) = fn2(_2, _5, _7, _4, _15.1, _5, _8, _9), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_9 = !_2;
_12 = _24;
_5 = core::ptr::addr_of_mut!(_4);
_9 = _2 ^ _2;
_5 = _14;
Goto(bb6)
}
bb6 = {
_3 = _18.fld0 | _18.fld0;
_27 = -_31;
_17.fld1 = _13 - _13;
_16 = !(*_10);
_15.0 = core::ptr::addr_of_mut!(_24);
_25 = _20;
_24 = _12;
RET = [_6,_6,_6,_6,_6,_6,_6];
_30 = _24;
_2 = _9;
_18 = Adt44 { fld0: _3 };
_35.fld0 = [5967382712595813798_usize,568070250192018571_usize,6_usize];
_18 = Adt44 { fld0: _3 };
_14 = _10;
_10 = core::ptr::addr_of_mut!((*_14));
_34 = (_15.0, _15.1);
RET = [_6,_6,_6,_6,_6,_6,_6];
_34.0 = core::ptr::addr_of_mut!(_24);
_28 = _12;
_30 = _28;
_13 = _17.fld1 << _23;
_2 = !_9;
Goto(bb7)
}
bb7 = {
_30 = _28;
_13 = _17.fld1;
_2 = (*_10) > _16;
_35.fld2 = [_17.fld1,_17.fld1,_13,_17.fld1,_13,_13,_17.fld1];
Call(_17.fld0 = fn3((*_5), Move(_18), _5), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_18.fld0 = _28 as i8;
_2 = _19;
_8 = (*_10) + (*_10);
_38 = core::ptr::addr_of_mut!(_10);
_12 = _30;
_39 = -(*_10);
_11 = _9;
_23 = -(*_10);
_34 = (_15.0, _15.1);
RET = [_6,_6,_6,_6,_6,_6,_6];
_1 = 102188801023503589675703338381196166459_i128;
_4 = (*_14) & _39;
_42.fld1 = _21.1 - _21.1;
_14 = core::ptr::addr_of_mut!(_8);
match _1 {
0 => bb7,
1 => bb6,
2 => bb3,
3 => bb4,
4 => bb5,
102188801023503589675703338381196166459 => bb10,
_ => bb9
}
}
bb9 = {
_21.0 = &_7;
_20 = 3450685203697470241_usize as i64;
_14 = _10;
_21.0 = &(*_10);
_15.1 = [0_usize,3005969557630766462_usize,16610677294625542368_usize,824361900356509635_usize,4_usize,3_usize,5_usize];
(*_5) = _23 * _23;
_17.fld2 = [_17.fld1,_17.fld1,_17.fld1,_13,_17.fld1,_17.fld1,_17.fld1];
_18 = Adt44 { fld0: _3 };
_10 = core::ptr::addr_of_mut!((*_10));
_17.fld2 = [_13,_17.fld1,_17.fld1,_13,_13,_17.fld1,_17.fld1];
_19 = !_11;
_21.0 = &_7;
(*_10) = !_23;
_7 = (*_5);
_6 = 6_usize as u16;
_31 = !1570107038_i32;
(*_14) = -_8;
Call((*_10) = fn2(_2, _5, _7, _4, _15.1, _5, _8, _9), ReturnTo(bb5), UnwindUnreachable())
}
bb10 = {
_42.fld1 = _21.1;
_35.fld2 = _17.fld2;
_1 = (-64502545747218480614855091721044429821_i128);
_4 = 1106656760_u32 as isize;
_21.0 = &(*_14);
_9 = (*_5) > (*_14);
_33 = _14;
_3 = _18.fld0 | _18.fld0;
_35 = Adt49 { fld0: _17.fld0,fld1: _13,fld2: _17.fld2 };
Call(_26 = fn17((*_14), (*_33), _33, Move(_21.0), _5), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_15 = _34;
_7 = !(*_5);
_42.fld0 = [5532119683766680419_usize,5498046182772815314_usize,8491348327339708017_usize,16807235166639230553_usize,3_usize,17099442230434860407_usize,2_usize];
SetDiscriminant(_26, 3);
_24 = _12;
_4 = (*_14);
_34 = (_15.0, _15.1);
_42.fld2 = [_24,_12,_28,_24,_28];
(*_38) = core::ptr::addr_of_mut!(_16);
_25 = _20;
RET = [_6,_6,_6,_6,_6,_6,_6];
RET = [_6,_6,_6,_6,_6,_6,_6];
_17 = Adt49 { fld0: _35.fld0,fld1: _13,fld2: _35.fld2 };
_29 = _1 as u128;
(*_33) = _4 * _4;
_42.fld0 = [8315644003567771787_usize,4_usize,4_usize,13785585286085425908_usize,17069189572810114973_usize,4_usize,6965689753799389046_usize];
_45 = [_17.fld1,_17.fld1,_17.fld1,_13,_17.fld1,_13,_35.fld1];
(*_5) = _12 as isize;
_34.0 = _15.0;
_43 = 1926404291_u32 - 4145909480_u32;
_44.1 = _18.fld0 as f32;
_35.fld0 = _17.fld0;
_21.1 = _29 as f32;
Goto(bb12)
}
bb12 = {
_44.1 = _42.fld1;
_8 = (*_10) ^ _4;
_27 = _31 * _31;
place!(Field::<i32>(Variant(_26, 3), 0)) = _31;
_43 = 2995425020_u32 * 3474881034_u32;
_18.fld0 = _3;
_25 = (*_10) as i64;
_46 = RET;
(*_33) = _4;
_8 = _4;
_32 = _1 ^ _1;
_45 = [_35.fld1,_17.fld1,_35.fld1,_17.fld1,_35.fld1,_35.fld1,_35.fld1];
_36 = _9 & _9;
_17.fld1 = !_13;
_30 = _24;
(*_38) = _33;
(*_5) = !_8;
_44.0 = &_39;
place!(Field::<isize>(Variant(_26, 3), 2)) = _18.fld0 as isize;
match _1 {
0 => bb1,
275779821173719982848519515710723781635 => bb14,
_ => bb13
}
}
bb13 = {
_15 = _34;
_7 = !(*_5);
_42.fld0 = [5532119683766680419_usize,5498046182772815314_usize,8491348327339708017_usize,16807235166639230553_usize,3_usize,17099442230434860407_usize,2_usize];
SetDiscriminant(_26, 3);
_24 = _12;
_4 = (*_14);
_34 = (_15.0, _15.1);
_42.fld2 = [_24,_12,_28,_24,_28];
(*_38) = core::ptr::addr_of_mut!(_16);
_25 = _20;
RET = [_6,_6,_6,_6,_6,_6,_6];
RET = [_6,_6,_6,_6,_6,_6,_6];
_17 = Adt49 { fld0: _35.fld0,fld1: _13,fld2: _35.fld2 };
_29 = _1 as u128;
(*_33) = _4 * _4;
_42.fld0 = [8315644003567771787_usize,4_usize,4_usize,13785585286085425908_usize,17069189572810114973_usize,4_usize,6965689753799389046_usize];
_45 = [_17.fld1,_17.fld1,_17.fld1,_13,_17.fld1,_13,_35.fld1];
(*_5) = _12 as isize;
_34.0 = _15.0;
_43 = 1926404291_u32 - 4145909480_u32;
_44.1 = _18.fld0 as f32;
_35.fld0 = _17.fld0;
_21.1 = _29 as f32;
Goto(bb12)
}
bb14 = {
_17.fld1 = _13 - _13;
_18 = Adt44 { fld0: _3 };
_17 = Adt49 { fld0: _35.fld0,fld1: _35.fld1,fld2: _45 };
_15.0 = _34.0;
_17.fld2 = _45;
_30 = _24;
_1 = _28 as i128;
_35.fld2 = _17.fld2;
_2 = _9 <= _19;
_11 = (*_10) > _8;
_21 = Move(_44);
_7 = -_4;
Goto(bb15)
}
bb15 = {
Call(_49 = dump_var(1_usize, 32_usize, Move(_32), 28_usize, Move(_28), 1_usize, Move(_1), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_49 = dump_var(1_usize, 11_usize, Move(_11), 22_usize, Move(_22), 46_usize, Move(_46), 36_usize, Move(_36)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = dump_var(1_usize, 23_usize, Move(_23), 19_usize, Move(_19), 7_usize, Move(_7), 43_usize, Move(_43)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_49 = dump_var(1_usize, 8_usize, Move(_8), 31_usize, Move(_31), 50_usize, _50, 50_usize, _50), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: bool,mut _2: *mut isize,mut _3: isize,mut _4: isize,mut _5: [usize; 7],mut _6: *mut isize,mut _7: isize,mut _8: bool) -> isize {
mir! {
type RET = isize;
let _9: isize;
let _10: [u8; 4];
let _11: ();
let _12: ();
{
RET = _3 & _3;
_1 = _8 ^ _8;
_6 = _2;
_1 = _8;
_7 = '\u{30f73}' as isize;
_10 = [182_u8,241_u8,223_u8,12_u8];
_8 = _1;
_8 = !_1;
_1 = _8 ^ _8;
_1 = _8;
_2 = core::ptr::addr_of_mut!(_3);
(*_2) = RET + RET;
_7 = !(*_2);
(*_2) = 3_i8 as isize;
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(2_usize, 7_usize, Move(_7), 4_usize, Move(_4), 3_usize, Move(_3), 12_usize, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: isize,mut _2: Adt44,mut _3: *mut isize) -> [usize; 3] {
mir! {
type RET = [usize; 3];
let _4: [u16; 7];
let _5: bool;
let _6: char;
let _7: (&'static isize, f32);
let _8: [u16; 7];
let _9: [u8; 4];
let _10: (f32, u8, i32, u8);
let _11: u32;
let _12: f64;
let _13: f64;
let _14: f32;
let _15: isize;
let _16: [u8; 7];
let _17: [i32; 6];
let _18: [i32; 6];
let _19: i64;
let _20: bool;
let _21: bool;
let _22: [i32; 8];
let _23: f64;
let _24: Adt41;
let _25: isize;
let _26: ();
let _27: ();
{
RET = [4_usize,18185082196667754875_usize,18137694696601340067_usize];
_1 = (*_3) | (*_3);
_4 = [54709_u16,41748_u16,65443_u16,55163_u16,27757_u16,26542_u16,2717_u16];
_2.fld0 = 14_i8 + 89_i8;
_4 = [7622_u16,34898_u16,23436_u16,24819_u16,4342_u16,52203_u16,7088_u16];
_2 = Adt44 { fld0: 22_i8 };
_2.fld0 = (-112_i8);
_3 = core::ptr::addr_of_mut!(_1);
_2 = Adt44 { fld0: 3_i8 };
_5 = !false;
(*_3) = 9223372036854775807_isize;
_4 = [2326_u16,62492_u16,28366_u16,58225_u16,48263_u16,56950_u16,35234_u16];
_1 = (-47_isize) >> _2.fld0;
RET = [1003291112864569587_usize,7_usize,4_usize];
_1 = (-36_isize) | (-9223372036854775808_isize);
(*_3) = 9223372036854775807_isize;
(*_3) = -9223372036854775807_isize;
RET = [2_usize,3959799750013043238_usize,5_usize];
RET = [2_usize,2_usize,873021239300629206_usize];
_3 = core::ptr::addr_of_mut!(_1);
Call(RET = fn4(_4, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_3) = -9223372036854775807_isize;
_6 = '\u{38d61}';
_7.0 = &(*_3);
_1 = !(-9223372036854775808_isize);
_7.1 = 2402267246198981435_i64 as f32;
_2.fld0 = !46_i8;
RET = [6_usize,5486384057363579252_usize,7_usize];
(*_3) = (-9223372036854775808_isize) | 97_isize;
_2.fld0 = -(-101_i8);
_2 = Adt44 { fld0: (-36_i8) };
match _2.fld0 {
0 => bb2,
1 => bb3,
2 => bb4,
340282366920938463463374607431768211420 => bb6,
_ => bb5
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
_1 = (-1488222566728089008_i64) as isize;
_2 = Adt44 { fld0: 111_i8 };
RET = [2_usize,0_usize,15441117632907095070_usize];
_7.0 = &(*_3);
_7.1 = 7067147231886777006_u64 as f32;
_2 = Adt44 { fld0: (-37_i8) };
_9 = [85_u8,195_u8,192_u8,117_u8];
RET = [4919775864038060589_usize,5_usize,6_usize];
(*_3) = -9223372036854775807_isize;
Call((*_3) = fn5(_9, _4, _4, _2.fld0, _3, RET, _4, _4, _4), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_9 = [253_u8,168_u8,199_u8,105_u8];
match _2.fld0 {
0 => bb5,
1 => bb2,
2 => bb6,
340282366920938463463374607431768211419 => bb8,
_ => bb4
}
}
bb8 = {
_7.0 = &(*_3);
_2.fld0 = 0_usize as i8;
(*_3) = 9223372036854775807_isize;
_4 = [49558_u16,8403_u16,56336_u16,2405_u16,41085_u16,11006_u16,11054_u16];
_8 = [34817_u16,50025_u16,16920_u16,32667_u16,20221_u16,38943_u16,28095_u16];
_4 = _8;
_7.0 = &(*_3);
(*_3) = 9223372036854775807_isize;
RET = [13784625579224625564_usize,11251952647415982980_usize,14634368118165214119_usize];
_5 = !false;
_8 = [42979_u16,36306_u16,2273_u16,36547_u16,34832_u16,62847_u16,64960_u16];
_10.1 = 211_u8;
_10.2 = (-416231900_i32);
_10.3 = !_10.1;
_10.1 = 15603056452882336335_usize as u8;
_10.2 = (-7290981368737545740_i64) as i32;
_9 = [_10.1,_10.3,_10.1,_10.1];
_2.fld0 = (*_3) as i8;
_10.2 = 598005282_i32 & (-973171153_i32);
_10.0 = _7.1 * _7.1;
RET = [3_usize,16400658291897077530_usize,17586751170565319009_usize];
_2 = Adt44 { fld0: 100_i8 };
(*_3) = !(-9223372036854775808_isize);
RET = [2243346460691445050_usize,2589036303639109749_usize,0_usize];
_7.0 = &(*_3);
_10 = (_7.1, 43_u8, (-505032330_i32), 46_u8);
Goto(bb9)
}
bb9 = {
_10.0 = _7.1;
_10.1 = !_10.3;
RET = [6181747737647048870_usize,11704264801749999254_usize,5_usize];
_10.2 = !723070023_i32;
_2.fld0 = !52_i8;
_10.0 = _7.1 - _7.1;
_1 = !(-11_isize);
RET = [3_usize,1_usize,2_usize];
(*_3) = _10.0 as isize;
_6 = '\u{d74e8}';
_9 = [_10.1,_10.3,_10.3,_10.1];
_2.fld0 = 261678624014693131880141754351645930805_u128 as i8;
_11 = 1412401242_u32 * 2106704710_u32;
_9 = [_10.3,_10.3,_10.1,_10.1];
_13 = (-25113_i16) as f64;
_15 = _1 ^ _1;
_14 = _7.1;
_7.1 = -_10.0;
_13 = 28683_i16 as f64;
match _10.3 {
0 => bb3,
46 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_14 = _7.1;
_7.1 = _14 * _14;
_10.2 = 688397986_i32;
_19 = 16710_i16 as i64;
_10.1 = !_10.3;
_15 = (*_3) << _10.3;
_10.1 = (-31364_i16) as u8;
_20 = !_5;
Goto(bb12)
}
bb12 = {
_9 = [_10.3,_10.3,_10.3,_10.1];
_2.fld0 = 14296748595254740970_u64 as i8;
_9 = [_10.1,_10.3,_10.3,_10.3];
_2.fld0 = (-40_i8) >> _15;
_17 = [_10.2,_10.2,_10.2,_10.2,_10.2,_10.2];
_18 = [_10.2,_10.2,_10.2,_10.2,_10.2,_10.2];
_16 = [_10.3,_10.3,_10.3,_10.3,_10.1,_10.3,_10.3];
_10.2 = (-334540594_i32);
_5 = _20;
(*_3) = _15;
_17 = [_10.2,_10.2,_10.2,_10.2,_10.2,_10.2];
_19 = 56476_u16 as i64;
_8 = [28886_u16,59257_u16,18280_u16,8652_u16,11196_u16,63771_u16,1404_u16];
_10.0 = 6278868586253211197_u64 as f32;
_10.0 = _14 + _14;
(*_3) = !_15;
_16 = [_10.1,_10.1,_10.1,_10.3,_10.3,_10.1,_10.3];
_5 = !_20;
match _10.2 {
0 => bb11,
1 => bb2,
2 => bb9,
340282366920938463463374607431433670862 => bb14,
_ => bb13
}
}
bb13 = {
_10.0 = _7.1;
_10.1 = !_10.3;
RET = [6181747737647048870_usize,11704264801749999254_usize,5_usize];
_10.2 = !723070023_i32;
_2.fld0 = !52_i8;
_10.0 = _7.1 - _7.1;
_1 = !(-11_isize);
RET = [3_usize,1_usize,2_usize];
(*_3) = _10.0 as isize;
_6 = '\u{d74e8}';
_9 = [_10.1,_10.3,_10.3,_10.1];
_2.fld0 = 261678624014693131880141754351645930805_u128 as i8;
_11 = 1412401242_u32 * 2106704710_u32;
_9 = [_10.3,_10.3,_10.1,_10.1];
_13 = (-25113_i16) as f64;
_15 = _1 ^ _1;
_14 = _7.1;
_7.1 = -_10.0;
_13 = 28683_i16 as f64;
match _10.3 {
0 => bb3,
46 => bb11,
_ => bb10
}
}
bb14 = {
_12 = _13;
_2.fld0 = -102_i8;
_9 = [_10.3,_10.1,_10.3,_10.3];
_14 = _7.1;
_7.0 = &_15;
_4 = [62795_u16,42821_u16,58992_u16,10130_u16,6421_u16,31101_u16,35505_u16];
_8 = _4;
_10 = (_14, 204_u8, (-719597240_i32), 166_u8);
(*_3) = _15;
_10 = (_7.1, 98_u8, (-1600279696_i32), 249_u8);
(*_3) = !_15;
_5 = (*_3) != (*_3);
_22 = [_10.2,_10.2,_10.2,_10.2,_10.2,_10.2,_10.2,_10.2];
_17 = [_10.2,_10.2,_10.2,_10.2,_10.2,_10.2];
_25 = _1 ^ _1;
(*_3) = 11725_i16 as isize;
_24.fld0 = _19;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(3_usize, 19_usize, Move(_19), 18_usize, Move(_18), 25_usize, Move(_25), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(3_usize, 17_usize, Move(_17), 20_usize, Move(_20), 8_usize, Move(_8), 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: [u16; 7],mut _2: [u16; 7]) -> [usize; 3] {
mir! {
type RET = [usize; 3];
let _3: Adt48;
let _4: Adt47;
let _5: Adt44;
let _6: [i32; 6];
let _7: [u8; 4];
let _8: char;
let _9: u8;
let _10: u128;
let _11: [u8; 7];
let _12: [i32; 8];
let _13: isize;
let _14: u128;
let _15: char;
let _16: bool;
let _17: f64;
let _18: (f64, (*mut char, [usize; 7]), i8, char);
let _19: Adt53;
let _20: isize;
let _21: ();
let _22: ();
{
RET = [3_usize,6_usize,1_usize];
_1 = [16115_u16,52777_u16,27571_u16,11593_u16,18634_u16,10310_u16,11560_u16];
RET = [5_usize,14740381312371946564_usize,5916718793651245053_usize];
_3.fld2 = ['\u{1739c}','\u{a6d9e}','\u{3f78a}','\u{29de0}','\u{ff3b9}'];
_2 = [37508_u16,29292_u16,16053_u16,61283_u16,44585_u16,7654_u16,7143_u16];
_3.fld0 = [4_usize,6_usize,4_usize,2_usize,4_usize,6279014259250345940_usize,14122315669802253132_usize];
_3.fld0 = [2_usize,218104128940504780_usize,5_usize,2_usize,5581170232108993085_usize,0_usize,6_usize];
_3.fld2 = ['\u{44413}','\u{298d9}','\u{d8b26}','\u{4aebc}','\u{11e82}'];
_3.fld2 = ['\u{ab669}','\u{10eda1}','\u{a6103}','\u{53017}','\u{5c783}'];
_3.fld0 = [0_usize,0_usize,3_usize,5_usize,3582054295989190991_usize,8790703703388730700_usize,8783974930112250633_usize];
_3.fld0 = [2_usize,17164191850687175310_usize,8828604800250559432_usize,1_usize,16025030408379399697_usize,9639139112774455395_usize,329145428715554238_usize];
_3.fld1 = 80393295216677522732510345585359934915_u128 as f32;
_2 = [42728_u16,35245_u16,62736_u16,713_u16,13007_u16,33541_u16,62160_u16];
_2 = [27268_u16,51183_u16,8075_u16,40939_u16,30635_u16,25255_u16,42388_u16];
_5 = Adt44 { fld0: 15_i8 };
RET = [1_usize,10419966682500782844_usize,14381862769323351181_usize];
_2 = [11925_u16,19737_u16,45869_u16,64365_u16,50479_u16,18406_u16,27547_u16];
RET = [6612900144846225844_usize,6_usize,17839545351226589624_usize];
_3.fld0 = [5_usize,11747910130808500676_usize,6_usize,4_usize,8386801291198572607_usize,1801722474781192635_usize,5_usize];
_5.fld0 = 125_i8 - 12_i8;
_3.fld0 = [11270000061453176184_usize,11128788258627403433_usize,0_usize,6234434378099799624_usize,6426138129445600815_usize,0_usize,3326121773224916494_usize];
_6 = [820020365_i32,(-291824279_i32),1606205956_i32,(-1179355424_i32),1094259851_i32,(-245522332_i32)];
_4 = Adt47::Variant1 { fld0: _1,fld1: '\u{dce35}',fld2: Move(_5),fld3: _3.fld2 };
_1 = [5088_u16,59216_u16,19206_u16,17242_u16,12340_u16,57862_u16,55035_u16];
_3.fld1 = 9133_u16 as f32;
_2 = _1;
_6 = [1611739925_i32,1478175194_i32,(-760859453_i32),(-2074023593_i32),(-817085176_i32),430776309_i32];
RET = [15284037007917784672_usize,11190979585851791151_usize,7_usize];
Goto(bb1)
}
bb1 = {
place!(Field::<Adt44>(Variant(_4, 1), 2)).fld0 = 12_i8 << (-58_i8);
RET = [6_usize,13055611418718389093_usize,249904052073933484_usize];
place!(Field::<Adt44>(Variant(_4, 1), 2)) = Adt44 { fld0: (-24_i8) };
RET = [15446395027530889413_usize,445070411028708626_usize,2052827489929009725_usize];
_6 = [(-123309927_i32),(-863823295_i32),(-234895934_i32),(-1140944332_i32),1229453332_i32,(-638359508_i32)];
_5 = Move(Field::<Adt44>(Variant(_4, 1), 2));
_5.fld0 = -106_i8;
place!(Field::<Adt44>(Variant(_4, 1), 2)) = Adt44 { fld0: _5.fld0 };
place!(Field::<[u16; 7]>(Variant(_4, 1), 0)) = [62291_u16,2941_u16,45599_u16,1231_u16,31318_u16,3957_u16,23203_u16];
_2 = [42856_u16,40750_u16,60018_u16,9943_u16,9925_u16,58826_u16,50694_u16];
place!(Field::<Adt44>(Variant(_4, 1), 2)) = Adt44 { fld0: _5.fld0 };
_6 = [(-1705954819_i32),1127088196_i32,(-1323802423_i32),(-1468505300_i32),(-1003312654_i32),1019265356_i32];
_9 = 127_u8;
_7 = [_9,_9,_9,_9];
_6 = [(-121457764_i32),(-1997196467_i32),(-1779021582_i32),(-2055737122_i32),(-1967362711_i32),1036172760_i32];
_3.fld2 = ['\u{1c461}','\u{56ffb}','\u{9eaba}','\u{8bd69}','\u{41b8d}'];
_2 = [11290_u16,54025_u16,8972_u16,34814_u16,34434_u16,49570_u16,48058_u16];
place!(Field::<[char; 5]>(Variant(_4, 1), 3)) = ['\u{4677c}','\u{268af}','\u{22565}','\u{e2267}','\u{2a6f5}'];
_7 = [_9,_9,_9,_9];
place!(Field::<Adt44>(Variant(_4, 1), 2)) = Move(_5);
_3.fld0 = [2332287376820872497_usize,16936860379709024645_usize,7_usize,7804138467907056870_usize,2_usize,5_usize,2_usize];
place!(Field::<[char; 5]>(Variant(_4, 1), 3)) = ['\u{11e47}','\u{f0ec1}','\u{53a78}','\u{1f279}','\u{24ba}'];
_5 = Move(Field::<Adt44>(Variant(_4, 1), 2));
RET = [5_usize,5_usize,2831921122636477206_usize];
Goto(bb2)
}
bb2 = {
_7 = [_9,_9,_9,_9];
_10 = _9 as u128;
_7 = [_9,_9,_9,_9];
_8 = '\u{6f01b}';
_5 = Adt44 { fld0: (-29_i8) };
Goto(bb3)
}
bb3 = {
_3.fld2 = [_8,_8,_8,_8,_8];
place!(Field::<Adt44>(Variant(_4, 1), 2)) = Adt44 { fld0: _5.fld0 };
_12 = [1992235640_i32,201258469_i32,449412462_i32,1116652911_i32,2087643544_i32,1980468247_i32,(-1817340975_i32),543304692_i32];
_13 = (-101_isize);
_7 = [_9,_9,_9,_9];
_1 = Field::<[u16; 7]>(Variant(_4, 1), 0);
_8 = '\u{715e9}';
RET = [8270018148555218477_usize,6532043356513869370_usize,13989474023838963795_usize];
place!(Field::<char>(Variant(_4, 1), 1)) = _8;
Goto(bb4)
}
bb4 = {
RET = [6_usize,1801845516994911331_usize,0_usize];
_11 = [_9,_9,_9,_9,_9,_9,_9];
RET = [17334489837409256580_usize,8205585868418405682_usize,1_usize];
_9 = 82_u8 & 206_u8;
_7 = [_9,_9,_9,_9];
match Field::<Adt44>(Variant(_4, 1), 2).fld0 {
0 => bb3,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
340282366920938463463374607431768211427 => bb12,
_ => bb11
}
}
bb5 = {
_3.fld2 = [_8,_8,_8,_8,_8];
place!(Field::<Adt44>(Variant(_4, 1), 2)) = Adt44 { fld0: _5.fld0 };
_12 = [1992235640_i32,201258469_i32,449412462_i32,1116652911_i32,2087643544_i32,1980468247_i32,(-1817340975_i32),543304692_i32];
_13 = (-101_isize);
_7 = [_9,_9,_9,_9];
_1 = Field::<[u16; 7]>(Variant(_4, 1), 0);
_8 = '\u{715e9}';
RET = [8270018148555218477_usize,6532043356513869370_usize,13989474023838963795_usize];
place!(Field::<char>(Variant(_4, 1), 1)) = _8;
Goto(bb4)
}
bb6 = {
_7 = [_9,_9,_9,_9];
_10 = _9 as u128;
_7 = [_9,_9,_9,_9];
_8 = '\u{6f01b}';
_5 = Adt44 { fld0: (-29_i8) };
Goto(bb3)
}
bb7 = {
place!(Field::<Adt44>(Variant(_4, 1), 2)).fld0 = 12_i8 << (-58_i8);
RET = [6_usize,13055611418718389093_usize,249904052073933484_usize];
place!(Field::<Adt44>(Variant(_4, 1), 2)) = Adt44 { fld0: (-24_i8) };
RET = [15446395027530889413_usize,445070411028708626_usize,2052827489929009725_usize];
_6 = [(-123309927_i32),(-863823295_i32),(-234895934_i32),(-1140944332_i32),1229453332_i32,(-638359508_i32)];
_5 = Move(Field::<Adt44>(Variant(_4, 1), 2));
_5.fld0 = -106_i8;
place!(Field::<Adt44>(Variant(_4, 1), 2)) = Adt44 { fld0: _5.fld0 };
place!(Field::<[u16; 7]>(Variant(_4, 1), 0)) = [62291_u16,2941_u16,45599_u16,1231_u16,31318_u16,3957_u16,23203_u16];
_2 = [42856_u16,40750_u16,60018_u16,9943_u16,9925_u16,58826_u16,50694_u16];
place!(Field::<Adt44>(Variant(_4, 1), 2)) = Adt44 { fld0: _5.fld0 };
_6 = [(-1705954819_i32),1127088196_i32,(-1323802423_i32),(-1468505300_i32),(-1003312654_i32),1019265356_i32];
_9 = 127_u8;
_7 = [_9,_9,_9,_9];
_6 = [(-121457764_i32),(-1997196467_i32),(-1779021582_i32),(-2055737122_i32),(-1967362711_i32),1036172760_i32];
_3.fld2 = ['\u{1c461}','\u{56ffb}','\u{9eaba}','\u{8bd69}','\u{41b8d}'];
_2 = [11290_u16,54025_u16,8972_u16,34814_u16,34434_u16,49570_u16,48058_u16];
place!(Field::<[char; 5]>(Variant(_4, 1), 3)) = ['\u{4677c}','\u{268af}','\u{22565}','\u{e2267}','\u{2a6f5}'];
_7 = [_9,_9,_9,_9];
place!(Field::<Adt44>(Variant(_4, 1), 2)) = Move(_5);
_3.fld0 = [2332287376820872497_usize,16936860379709024645_usize,7_usize,7804138467907056870_usize,2_usize,5_usize,2_usize];
place!(Field::<[char; 5]>(Variant(_4, 1), 3)) = ['\u{11e47}','\u{f0ec1}','\u{53a78}','\u{1f279}','\u{24ba}'];
_5 = Move(Field::<Adt44>(Variant(_4, 1), 2));
RET = [5_usize,5_usize,2831921122636477206_usize];
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
Return()
}
bb12 = {
RET = [3875785766206891965_usize,3958439949900154418_usize,3125503502817695706_usize];
_13 = -(-117_isize);
place!(Field::<Adt44>(Variant(_4, 1), 2)) = Adt44 { fld0: _5.fld0 };
_8 = Field::<char>(Variant(_4, 1), 1);
place!(Field::<[u16; 7]>(Variant(_4, 1), 0)) = _1;
_10 = 263106487600618798143295814413228205588_u128 | 266945002418364425858813193171889180223_u128;
_3.fld2 = [Field::<char>(Variant(_4, 1), 1),Field::<char>(Variant(_4, 1), 1),Field::<char>(Variant(_4, 1), 1),Field::<char>(Variant(_4, 1), 1),Field::<char>(Variant(_4, 1), 1)];
_13 = 59326_u16 as isize;
place!(Field::<[u16; 7]>(Variant(_4, 1), 0)) = [18686_u16,20660_u16,15203_u16,52325_u16,19321_u16,34190_u16,60290_u16];
place!(Field::<[char; 5]>(Variant(_4, 1), 3)) = [_8,_8,_8,Field::<char>(Variant(_4, 1), 1),_8];
RET = [7751061733626081660_usize,5_usize,5_usize];
_16 = true;
Goto(bb13)
}
bb13 = {
RET = [1668370127353726546_usize,3_usize,13627325872577347890_usize];
_1 = Field::<[u16; 7]>(Variant(_4, 1), 0);
_7 = [_9,_9,_9,_9];
_18.0 = 613621453302791012_i64 as f64;
_6 = [(-1906715813_i32),55275587_i32,(-2060487777_i32),247974062_i32,862342664_i32,304729673_i32];
place!(Field::<Adt44>(Variant(_4, 1), 2)) = Move(_5);
place!(Field::<Adt44>(Variant(_4, 1), 2)) = Adt44 { fld0: 110_i8 };
match Field::<Adt44>(Variant(_4, 1), 2).fld0 {
0 => bb7,
1 => bb10,
2 => bb8,
3 => bb6,
110 => bb14,
_ => bb5
}
}
bb14 = {
_5 = Adt44 { fld0: Field::<Adt44>(Variant(_4, 1), 2).fld0 };
_18.3 = Field::<char>(Variant(_4, 1), 1);
_18.1.1 = _3.fld0;
_4 = Adt47::Variant1 { fld0: _2,fld1: _18.3,fld2: Move(_5),fld3: _3.fld2 };
_5.fld0 = !Field::<Adt44>(Variant(_4, 1), 2).fld0;
SetDiscriminant(_4, 0);
_8 = _18.3;
_15 = _8;
place!(Field::<i128>(Variant(_4, 0), 2)) = 96902759913841974986032857911975163297_i128 | 74454373366639186708125161285699551513_i128;
_18.1.0 = core::ptr::addr_of_mut!(_8);
_3.fld1 = 15508_i16 as f32;
_6 = [275024096_i32,605371660_i32,535208607_i32,(-478394069_i32),(-749305902_i32),(-2027489865_i32)];
_7 = [_9,_9,_9,_9];
_19 = Adt53::Variant0 { fld0: _3.fld0 };
RET = [1_usize,2_usize,13198477268916929556_usize];
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(4_usize, 9_usize, Move(_9), 16_usize, Move(_16), 13_usize, Move(_13), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_21 = dump_var(4_usize, 15_usize, Move(_15), 10_usize, Move(_10), 22_usize, _22, 22_usize, _22), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [u8; 4],mut _2: [u16; 7],mut _3: [u16; 7],mut _4: i8,mut _5: *mut isize,mut _6: [usize; 3],mut _7: [u16; 7],mut _8: [u16; 7],mut _9: [u16; 7]) -> isize {
mir! {
type RET = isize;
let _10: i8;
let _11: [usize; 3];
let _12: isize;
let _13: f64;
let _14: bool;
let _15: f64;
let _16: bool;
let _17: [u16; 7];
let _18: (bool, (*mut char, [usize; 7]), i64, i8, i16, i32, [u8; 4]);
let _19: Adt51;
let _20: isize;
let _21: u32;
let _22: Adt44;
let _23: bool;
let _24: i64;
let _25: *mut char;
let _26: ();
let _27: ();
{
_4 = 1115166790_i32 as i8;
_3 = _8;
_1 = [40_u8,151_u8,82_u8,39_u8];
RET = 9223372036854775807_isize;
_1 = [11_u8,97_u8,84_u8,246_u8];
_3 = _9;
RET = (-9223372036854775808_isize) + 9223372036854775807_isize;
_6 = [3_usize,4_usize,2944314563487918008_usize];
_1 = [185_u8,217_u8,52_u8,102_u8];
_10 = _4;
_5 = core::ptr::addr_of_mut!(RET);
_4 = -_10;
(*_5) = 89752550505192724556298751167167775377_i128 as isize;
RET = 9223372036854775807_isize ^ (-9223372036854775808_isize);
_13 = 7186089898440765376_u64 as f64;
_7 = _2;
_12 = RET - (*_5);
_11 = [7_usize,1_usize,2_usize];
_10 = _4;
RET = _12;
_6 = _11;
_10 = _4;
_9 = [51697_u16,64367_u16,34089_u16,37239_u16,64773_u16,58919_u16,60158_u16];
RET = _12 | _12;
RET = !_12;
_3 = [20882_u16,16964_u16,5634_u16,58950_u16,40700_u16,48978_u16,19027_u16];
Call(_10 = core::intrinsics::transmute(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = [6_usize,12139558468228726829_usize,7_usize];
_2 = [37298_u16,54782_u16,54974_u16,40854_u16,21572_u16,21834_u16,50740_u16];
(*_5) = -_12;
_9 = [56701_u16,23006_u16,47199_u16,48043_u16,6614_u16,23466_u16,50401_u16];
_12 = RET;
_14 = true;
_14 = _12 >= (*_5);
_5 = core::ptr::addr_of_mut!(RET);
_16 = _14;
_15 = _13 + _13;
_5 = core::ptr::addr_of_mut!(RET);
_9 = [44215_u16,34542_u16,30585_u16,24384_u16,42458_u16,48857_u16,56900_u16];
(*_5) = _12 ^ _12;
_5 = core::ptr::addr_of_mut!((*_5));
_11 = [8170207157885590785_usize,17989753802710546017_usize,3319788406256538778_usize];
_14 = _16 & _16;
_12 = _4 as isize;
_2 = [2393_u16,22633_u16,48172_u16,30224_u16,18835_u16,45144_u16,16453_u16];
_13 = 2387738549229246042_usize as f64;
(*_5) = _12;
_9 = [60177_u16,55324_u16,30649_u16,36103_u16,50932_u16,59718_u16,34860_u16];
_2 = [18103_u16,30448_u16,34127_u16,33972_u16,64515_u16,53128_u16,8899_u16];
_16 = _14;
_10 = _4;
_10 = 15509130551595508808_u64 as i8;
_14 = _16 >= _16;
_17 = _2;
_13 = _15;
Call(_12 = fn6(_16, _6, _14, _7, _14), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Goto(bb3)
}
bb3 = {
_2 = [65236_u16,12049_u16,10514_u16,7488_u16,57430_u16,48292_u16,9848_u16];
_15 = _13;
_8 = [40565_u16,7436_u16,63412_u16,61096_u16,33739_u16,2135_u16,42532_u16];
_18.0 = _14 ^ _14;
_18.2 = (-8270436344109582977_i64);
_22.fld0 = 10102715641729827599_u64 as i8;
_18.1.1 = [15586049673189681901_usize,3_usize,16116370486593299511_usize,2_usize,7_usize,9969959536410589530_usize,17814050859406138739_usize];
_18.5 = (-1881555113_i32) ^ 650827795_i32;
_13 = -_15;
_10 = _22.fld0;
_20 = _12;
_21 = 133378172_u32 | 992296659_u32;
_20 = _12 | _12;
_18.3 = '\u{11b90}' as i8;
(*_5) = -_20;
_18.2 = 3663676381838636857_u64 as i64;
_8 = [50663_u16,62512_u16,10803_u16,14815_u16,61819_u16,53569_u16,21596_u16];
_17 = [681_u16,31653_u16,12942_u16,25066_u16,23632_u16,25664_u16,14781_u16];
Goto(bb4)
}
bb4 = {
Call(_26 = dump_var(5_usize, 14_usize, Move(_14), 11_usize, Move(_11), 1_usize, Move(_1), 8_usize, Move(_8)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_26 = dump_var(5_usize, 4_usize, Move(_4), 3_usize, Move(_3), 10_usize, Move(_10), 6_usize, Move(_6)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: bool,mut _2: [usize; 3],mut _3: bool,mut _4: [u16; 7],mut _5: bool) -> isize {
mir! {
type RET = isize;
let _6: bool;
let _7: isize;
let _8: isize;
let _9: bool;
let _10: *const (bool, (*mut char, [usize; 7]), i64, i8, i16, i32, [u8; 4]);
let _11: [u8; 7];
let _12: Adt41;
let _13: Adt48;
let _14: isize;
let _15: f64;
let _16: Adt48;
let _17: u64;
let _18: *mut *mut isize;
let _19: [u8; 7];
let _20: f32;
let _21: f64;
let _22: [usize; 7];
let _23: char;
let _24: u8;
let _25: bool;
let _26: i64;
let _27: *const (bool, (*mut char, [usize; 7]), i64, i8, i16, i32, [u8; 4]);
let _28: u128;
let _29: i16;
let _30: u8;
let _31: [u8; 7];
let _32: (f64, (*mut char, [usize; 7]), i8, char);
let _33: f64;
let _34: [i128; 5];
let _35: isize;
let _36: bool;
let _37: char;
let _38: (f64, (*mut char, [usize; 7]), i8, char);
let _39: ();
let _40: ();
{
RET = (-9223372036854775808_isize);
_1 = _3;
_1 = _3;
_1 = _3;
_5 = _3 >= _3;
Goto(bb1)
}
bb1 = {
_4 = [15716_u16,11784_u16,26152_u16,44724_u16,4765_u16,17779_u16,39696_u16];
_2 = [3259401699374224465_usize,10938889305992351109_usize,4_usize];
_8 = !RET;
_5 = _3 <= _1;
_7 = _8 << _8;
RET = _8 * _8;
RET = _7 & _7;
_4 = [18287_u16,4736_u16,47815_u16,11788_u16,24071_u16,5867_u16,13355_u16];
_2 = [1_usize,5522038591474472763_usize,17342192877951743040_usize];
_1 = _3 == _5;
Call(_2 = fn7(_1, _1, _3, _1, _1, _8, _5, _5, _5, _5, _5, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = _5 & _3;
_4 = [448_u16,31151_u16,64433_u16,33246_u16,37675_u16,27643_u16,28405_u16];
_7 = _8 & RET;
RET = _8;
_11 = [186_u8,188_u8,107_u8,194_u8,160_u8,159_u8,10_u8];
_8 = RET >> _7;
_5 = _1 | _6;
Call(_8 = core::intrinsics::bswap(_7), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9 = _5 > _6;
_5 = !_9;
_6 = _1;
_6 = _5;
_11 = [102_u8,229_u8,0_u8,80_u8,92_u8,182_u8,19_u8];
RET = _8;
_9 = !_3;
RET = _7;
_12.fld0 = (-3557304158926817921_i64) | (-5968850472726447371_i64);
_9 = _5 | _6;
_8 = (-380368098_i32) as isize;
_12.fld1 = [129630765978368757108541384174717921219_i128,76233165566493459749117045213038629114_i128,158344485993377779404696534783488332023_i128,(-161717162982602136575470834077957415169_i128),78215982213618353969615151363234445614_i128];
_7 = RET | RET;
_12.fld0 = (-6430653054000154933_i64) - (-6557821954116048742_i64);
_12.fld1 = [(-125226061207667522887059294553125073349_i128),107460439206591177613886021594971422183_i128,1534244722491734561208263981002829241_i128,74735287889349369200724775222900822794_i128,(-130374783321529264144359887843923294019_i128)];
_13.fld0 = [6_usize,13377980046471629639_usize,7653027581469840543_usize,6072394772816488882_usize,6_usize,3508844152822263663_usize,13355791023903460238_usize];
_4 = [31811_u16,2018_u16,46803_u16,41861_u16,58730_u16,50133_u16,64096_u16];
_13.fld1 = RET as f32;
RET = _7;
RET = _7;
_1 = !_9;
_1 = !_6;
_13.fld2 = ['\u{a168c}','\u{5d155}','\u{6b5fc}','\u{bda4e}','\u{475ca}'];
Goto(bb4)
}
bb4 = {
_14 = _7;
_16.fld1 = 3003812229217254197_u64 as f32;
RET = 11740897177921497745_usize as isize;
_15 = _12.fld0 as f64;
_15 = 55901_u16 as f64;
_13.fld1 = _15 as f32;
_13.fld2 = ['\u{c53d2}','\u{c3696}','\u{a06e2}','\u{60865}','\u{a57ec}'];
_17 = !6114614144845405936_u64;
_13.fld2 = ['\u{d61a9}','\u{f5a85}','\u{1eb9b}','\u{8407a}','\u{78387}'];
_12.fld0 = 1136044254435126940_i64 & 2220096954444228290_i64;
_7 = _14 ^ _14;
_16.fld0 = [2332579935327104931_usize,6_usize,4_usize,6758685754391175142_usize,1_usize,11107290858444506401_usize,7_usize];
Goto(bb5)
}
bb5 = {
_16.fld1 = -_13.fld1;
_1 = _9 == _5;
_3 = _5;
RET = _13.fld1 as isize;
_7 = !_14;
_16 = Adt48 { fld0: _13.fld0,fld1: _13.fld1,fld2: _13.fld2 };
Goto(bb6)
}
bb6 = {
_13.fld2 = _16.fld2;
_8 = (-1672166779_i32) as isize;
_9 = _6;
_16 = Adt48 { fld0: _13.fld0,fld1: _13.fld1,fld2: _13.fld2 };
_5 = _1 | _9;
_15 = 165069762424552984339486515233656710257_u128 as f64;
_1 = _5;
_11 = [237_u8,127_u8,20_u8,241_u8,147_u8,120_u8,129_u8];
_13.fld2 = ['\u{56d79}','\u{9aa74}','\u{4c89d}','\u{19fb3}','\u{3f268}'];
_5 = _3;
_16.fld1 = _13.fld1 - _13.fld1;
_12.fld0 = !5320456445773019054_i64;
_7 = _14 ^ _14;
_16 = Adt48 { fld0: _13.fld0,fld1: _13.fld1,fld2: _13.fld2 };
_16.fld2 = ['\u{9d6b4}','\u{109c57}','\u{3d544}','\u{aef2d}','\u{33c89}'];
_13.fld1 = _14 as f32;
RET = _14 & _8;
_8 = RET >> _14;
_12.fld0 = -(-6348706803342751680_i64);
_12.fld0 = 6714422234387476727_i64 & 4838534965843928616_i64;
_19 = [47_u8,51_u8,86_u8,220_u8,21_u8,63_u8,212_u8];
RET = _13.fld1 as isize;
_6 = _1 < _3;
_16.fld1 = _13.fld1 - _13.fld1;
Goto(bb7)
}
bb7 = {
_16 = _13;
_21 = _15;
_2 = [10121917491475288750_usize,499591557432357634_usize,7744641631592759794_usize];
_14 = !_8;
_3 = _9;
_16 = _13;
_9 = _3 != _1;
_2 = [15634336429258268512_usize,4_usize,4_usize];
_13.fld1 = -_16.fld1;
_13.fld0 = _16.fld0;
_22 = _13.fld0;
_9 = _6;
Call(_18 = fn15(_9, _5, _19, _3, _3), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_14 = _8 + _7;
_12.fld1 = [143588009032456989864884688505270382012_i128,(-34798826697850476165508529259388040118_i128),148109833389367205085068605773105935619_i128,94350745617292535442863099150520268302_i128,69465549933228650626563485809198578974_i128];
_2 = [4722760945999919101_usize,1651907653735724710_usize,7_usize];
_19 = _11;
_16.fld0 = [13846136410858153235_usize,2_usize,13070626562778071181_usize,17713647395318874877_usize,5396937192085704734_usize,4586730368656884016_usize,18291362087831553179_usize];
_22 = [6_usize,11427108030224692496_usize,2_usize,5_usize,6_usize,2163765072201476425_usize,17063545999906057027_usize];
_11 = [138_u8,252_u8,22_u8,200_u8,213_u8,46_u8,200_u8];
_16 = Adt48 { fld0: _22,fld1: _13.fld1,fld2: _13.fld2 };
_20 = _16.fld1;
Call(_20 = fn16(_5, _14, _22, _6, _5, _14, _9, _1, _3, _13.fld2, _3, _3, _3, _3, _11, _3), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_11 = _19;
_13 = _16;
_22 = [6674246355005987593_usize,5_usize,9770401944786743324_usize,3057199488713139518_usize,3981098030503517847_usize,6_usize,0_usize];
_12.fld0 = (-9018374722296209340_i64) * (-2723767777144344831_i64);
_22 = [1205621090638772577_usize,13329685198351745104_usize,0_usize,4_usize,5_usize,3119425501846783105_usize,17290731525289398248_usize];
_2 = [4_usize,217129326530405073_usize,11592428144916074633_usize];
_16 = _13;
_15 = -_21;
_12.fld0 = -7026257021301400415_i64;
_17 = 4773942264457534895_u64 - 11517147447091456127_u64;
_7 = _8 - _8;
_8 = 15089943331134222211_usize as isize;
_23 = '\u{39928}';
_13.fld2 = _16.fld2;
Call(_11 = core::intrinsics::transmute(_19), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_16.fld2 = _13.fld2;
_12.fld1 = [(-158394303011574206312705145743944990010_i128),80682651853014335828427030655053937015_i128,(-9795645966571446783773458839669200048_i128),152621703948199262654265819664845990145_i128,(-39145513455220496375519004367855208771_i128)];
_13 = _16;
_13.fld1 = -_16.fld1;
_13.fld1 = -_20;
_24 = 45_u8 << RET;
_25 = !_1;
_13 = Adt48 { fld0: _16.fld0,fld1: _20,fld2: _16.fld2 };
_14 = -_7;
_15 = _13.fld1 as f64;
_6 = _3;
_26 = -_12.fld0;
_12.fld0 = 45479_u16 as i64;
_4 = [24205_u16,18444_u16,58759_u16,23417_u16,14539_u16,57095_u16,57834_u16];
_24 = 99_u8 * 216_u8;
_25 = _9 | _9;
_28 = !43715502442662949055597112769775005162_u128;
_16.fld2 = [_23,_23,_23,_23,_23];
_9 = _5 & _3;
_32.2 = !86_i8;
_16 = _13;
_30 = _24 * _24;
Goto(bb11)
}
bb11 = {
_3 = _6 | _6;
_32.1.0 = core::ptr::addr_of_mut!(_32.3);
_1 = _25;
_16.fld0 = _13.fld0;
_12.fld1 = [28860530292961414462127241054841440681_i128,59850529729557278211490177229978601527_i128,66661329465228210569946198986259216093_i128,40797470789344278495726038503717211674_i128,(-17316319461857843909666863520197230740_i128)];
_30 = _24 & _24;
_16.fld0 = [3078904850640615886_usize,1_usize,13391651429296895028_usize,2_usize,0_usize,17889573253265832670_usize,5159575651009819621_usize];
_13.fld2 = [_23,_23,_23,_23,_23];
_32.0 = -_15;
_33 = _32.0;
_12.fld0 = _26 ^ _26;
_17 = !5570503124056109263_u64;
_11 = [_30,_24,_30,_24,_30,_24,_30];
_30 = _24 - _24;
_25 = _1;
_20 = _16.fld1 + _13.fld1;
_13.fld0 = [5214415668777155989_usize,3_usize,3_usize,16422368576074741988_usize,2_usize,5_usize,4128933884396629686_usize];
_32.1.1 = [14379054052400255502_usize,12261240933452610177_usize,16739902509044797578_usize,15858369780253250446_usize,13550798449219647444_usize,2_usize,4_usize];
_13 = _16;
_29 = 21685_i16 ^ 29758_i16;
_4 = [24344_u16,30725_u16,27233_u16,42654_u16,42324_u16,47457_u16,58954_u16];
_1 = !_6;
Goto(bb12)
}
bb12 = {
_19 = [_24,_30,_30,_30,_30,_30,_30];
_9 = _1;
Goto(bb13)
}
bb13 = {
_23 = '\u{3aae}';
_16.fld1 = _20 - _13.fld1;
_35 = _15 as isize;
Goto(bb14)
}
bb14 = {
_32.2 = _17 as i8;
_36 = _1;
_19 = _11;
_29 = (-16733_i16);
_22 = _13.fld0;
_9 = _25;
_31 = [_24,_30,_30,_24,_30,_30,_30];
_7 = _14;
_20 = -_16.fld1;
RET = _14 * _14;
_31 = [_30,_24,_24,_30,_30,_24,_24];
_32.1.1 = _16.fld0;
_16 = _13;
_28 = 112744620024347473658592423964693605810_u128 | 19469945823365633110779001794899947821_u128;
_25 = !_1;
_24 = _30;
_32.1.1 = [4_usize,6839077864687575198_usize,2_usize,15180675124815518734_usize,4_usize,7553206503852307272_usize,14752830309468568918_usize];
_5 = _1;
_26 = _12.fld0 & _12.fld0;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(6_usize, 3_usize, Move(_3), 30_usize, Move(_30), 6_usize, Move(_6), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(6_usize, 2_usize, Move(_2), 7_usize, Move(_7), 8_usize, Move(_8), 28_usize, Move(_28)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(6_usize, 14_usize, Move(_14), 22_usize, Move(_22), 25_usize, Move(_25), 23_usize, Move(_23)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: isize,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: bool,mut _12: bool,mut _13: bool) -> [usize; 3] {
mir! {
type RET = [usize; 3];
let _14: f32;
let _15: Adt48;
let _16: [usize; 3];
let _17: [char; 5];
let _18: *const usize;
let _19: u128;
let _20: bool;
let _21: char;
let _22: *mut i16;
let _23: f32;
let _24: f32;
let _25: f64;
let _26: char;
let _27: Adt54;
let _28: (*mut char, [usize; 7]);
let _29: usize;
let _30: [u8; 7];
let _31: [i32; 6];
let _32: [u8; 7];
let _33: *mut char;
let _34: f64;
let _35: isize;
let _36: u32;
let _37: u32;
let _38: f64;
let _39: Adt49;
let _40: ();
let _41: ();
{
RET = [11799687927860601766_usize,15695646508849771286_usize,7_usize];
_13 = _12;
_14 = 202_u8 as f32;
_2 = _4;
_4 = _10;
_2 = _8 <= _5;
_3 = !_7;
_12 = _11;
_9 = _4 & _11;
_2 = _5;
_11 = !_4;
_9 = _13 != _7;
_10 = !_2;
Goto(bb1)
}
bb1 = {
_1 = _13;
_15.fld1 = _14 * _14;
_4 = _2 | _2;
_15.fld2 = ['\u{45b2c}','\u{6991}','\u{ae25f}','\u{6efbb}','\u{1e90c}'];
Goto(bb2)
}
bb2 = {
_1 = !_2;
_15.fld1 = _14 + _14;
_15.fld0 = [10473036996321457095_usize,6_usize,8652040035704046355_usize,5_usize,16245219814904271121_usize,10625212580106495219_usize,4_usize];
_5 = _9 == _3;
_15.fld2 = ['\u{1e09c}','\u{91a9a}','\u{f41c4}','\u{1d543}','\u{c8e8a}'];
_10 = _12;
_13 = _8;
_9 = !_1;
_2 = _9;
_7 = !_13;
_15.fld1 = _14 * _14;
_12 = _3;
_5 = _1 & _2;
_7 = _4 != _13;
Call(_3 = fn8(_12, _12, _7, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_15.fld0 = [18166636665983801056_usize,6_usize,5_usize,6_usize,13588114604976311473_usize,12207297306667953186_usize,1532719959156215964_usize];
Call(_16 = fn9(_5, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_12 = _2;
_17 = ['\u{50acc}','\u{29f21}','\u{8382c}','\u{b810e}','\u{d4074}'];
_16 = RET;
_19 = 3659475377693349028_i64 as u128;
_13 = !_7;
_15.fld2 = _17;
_20 = _7 <= _2;
_1 = _5 | _4;
_20 = _2;
Goto(bb5)
}
bb5 = {
_10 = _15.fld1 > _15.fld1;
_7 = !_9;
RET = [4_usize,1_usize,2_usize];
_15.fld2 = _17;
_14 = _15.fld1 + _15.fld1;
_1 = _9 & _4;
RET = _16;
_4 = _20 ^ _3;
Goto(bb6)
}
bb6 = {
RET = [511872020122418727_usize,5_usize,2_usize];
RET = [4_usize,15257517976526310511_usize,2_usize];
_5 = _7;
_9 = _8;
Goto(bb7)
}
bb7 = {
_8 = _5 > _1;
_21 = '\u{faf56}';
_15.fld2 = [_21,_21,_21,_21,_21];
_12 = !_2;
_17 = _15.fld2;
_21 = '\u{1f3a3}';
_15.fld1 = 480918186_i32 as f32;
_9 = _2;
_16 = RET;
_11 = _8;
_6 = !9223372036854775807_isize;
_4 = _7;
_5 = _3 > _8;
Goto(bb8)
}
bb8 = {
_6 = !(-9223372036854775808_isize);
_17 = _15.fld2;
_24 = _14;
_26 = _21;
_15.fld0 = [7389412163322449744_usize,18373076916762246489_usize,0_usize,17623707281476868480_usize,1_usize,5114223998435715141_usize,1_usize];
_21 = _26;
_23 = _24;
_9 = _1;
_21 = _26;
_23 = _14;
_19 = 109734158088883651201539426910395805049_u128;
_2 = _10 | _9;
RET = [14109092661004778444_usize,6_usize,9278887288527374197_usize];
match _19 {
0 => bb4,
1 => bb6,
2 => bb7,
3 => bb9,
4 => bb10,
109734158088883651201539426910395805049 => bb12,
_ => bb11
}
}
bb9 = {
_8 = _5 > _1;
_21 = '\u{faf56}';
_15.fld2 = [_21,_21,_21,_21,_21];
_12 = !_2;
_17 = _15.fld2;
_21 = '\u{1f3a3}';
_15.fld1 = 480918186_i32 as f32;
_9 = _2;
_16 = RET;
_11 = _8;
_6 = !9223372036854775807_isize;
_4 = _7;
_5 = _3 > _8;
Goto(bb8)
}
bb10 = {
_12 = _2;
_17 = ['\u{50acc}','\u{29f21}','\u{8382c}','\u{b810e}','\u{d4074}'];
_16 = RET;
_19 = 3659475377693349028_i64 as u128;
_13 = !_7;
_15.fld2 = _17;
_20 = _7 <= _2;
_1 = _5 | _4;
_20 = _2;
Goto(bb5)
}
bb11 = {
_1 = _13;
_15.fld1 = _14 * _14;
_4 = _2 | _2;
_15.fld2 = ['\u{45b2c}','\u{6991}','\u{ae25f}','\u{6efbb}','\u{1e90c}'];
Goto(bb2)
}
bb12 = {
_29 = 12238804356564022695_usize | 6_usize;
RET = [_29,_29,_29];
_30 = [12_u8,225_u8,208_u8,96_u8,126_u8,174_u8,122_u8];
_28.0 = core::ptr::addr_of_mut!(_26);
_17 = [_26,_26,_26,_21,_21];
_9 = _2 == _7;
_4 = !_11;
_15.fld0 = [_29,_29,_29,_29,_29,_29,_29];
_31 = [(-714832891_i32),1429332166_i32,2010301960_i32,1090027443_i32,(-745233265_i32),(-1447963727_i32)];
_13 = _12 == _7;
_18 = core::ptr::addr_of!(_29);
_12 = !_1;
_25 = (-38_i8) as f64;
_2 = _5 != _1;
_20 = _4;
_10 = !_13;
_3 = _5;
_14 = (-79_i8) as f32;
_26 = _21;
_16 = RET;
_11 = _3;
Goto(bb13)
}
bb13 = {
_32 = [113_u8,22_u8,122_u8,225_u8,92_u8,149_u8,57_u8];
_16 = [(*_18),(*_18),(*_18)];
_9 = _2;
_15.fld0 = [_29,(*_18),(*_18),(*_18),(*_18),(*_18),(*_18)];
_28.1 = [(*_18),(*_18),_29,(*_18),(*_18),(*_18),_29];
_2 = _5;
_3 = !_10;
_11 = _4;
_20 = !_13;
RET = [(*_18),(*_18),(*_18)];
_4 = _8 ^ _5;
_15.fld0 = [(*_18),_29,_29,(*_18),(*_18),_29,(*_18)];
_6 = 101_isize;
_15 = Adt48 { fld0: _28.1,fld1: _24,fld2: _17 };
(*_18) = 17363320882036972322_usize;
_16 = [_29,(*_18),_29];
_20 = _1 | _9;
_33 = _28.0;
(*_18) = 13805393763373349332_usize;
_23 = _24 + _24;
_34 = _25;
_15.fld1 = _24;
Goto(bb14)
}
bb14 = {
RET = _16;
_28.0 = _33;
RET = _16;
_16 = [(*_18),(*_18),_29];
_28.1 = [(*_18),(*_18),(*_18),(*_18),_29,_29,(*_18)];
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(7_usize, 5_usize, Move(_5), 19_usize, Move(_19), 11_usize, Move(_11), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(7_usize, 3_usize, Move(_3), 31_usize, Move(_31), 2_usize, Move(_2), 26_usize, Move(_26)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(7_usize, 29_usize, Move(_29), 9_usize, Move(_9), 6_usize, Move(_6), 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool) -> bool {
mir! {
type RET = bool;
let _5: Adt49;
let _6: Adt46;
let _7: ();
let _8: ();
{
_4 = _1 ^ _3;
_4 = _3;
_1 = _4 <= _4;
_4 = !_3;
RET = _2 & _4;
RET = _1 != _4;
RET = !_3;
RET = !_3;
_3 = _1 == RET;
_5.fld1 = !40_u8;
_1 = _3;
RET = _4;
_5.fld0 = [4_usize,3_usize,6477367133340323817_usize];
_5.fld2 = [_5.fld1,_5.fld1,_5.fld1,_5.fld1,_5.fld1,_5.fld1,_5.fld1];
_5.fld2 = [_5.fld1,_5.fld1,_5.fld1,_5.fld1,_5.fld1,_5.fld1,_5.fld1];
_5.fld2 = [_5.fld1,_5.fld1,_5.fld1,_5.fld1,_5.fld1,_5.fld1,_5.fld1];
_5.fld2 = [_5.fld1,_5.fld1,_5.fld1,_5.fld1,_5.fld1,_5.fld1,_5.fld1];
_1 = !_4;
RET = !_3;
RET = !_3;
_2 = _1 > _4;
RET = _4;
_1 = !_2;
RET = _3 == _2;
RET = _1 < _4;
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(8_usize, 2_usize, Move(_2), 4_usize, Move(_4), 8_usize, _8, 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: bool,mut _2: bool) -> [usize; 3] {
mir! {
type RET = [usize; 3];
let _3: *mut bool;
let _4: bool;
let _5: isize;
let _6: isize;
let _7: i16;
let _8: Adt50;
let _9: Adt41;
let _10: isize;
let _11: Adt41;
let _12: [usize; 7];
let _13: [i32; 6];
let _14: [i32; 8];
let _15: Adt41;
let _16: [u16; 7];
let _17: i64;
let _18: char;
let _19: isize;
let _20: isize;
let _21: u32;
let _22: Adt49;
let _23: char;
let _24: [u8; 4];
let _25: Adt40;
let _26: ();
let _27: ();
{
RET = [4_usize,14210449654447764725_usize,5_usize];
_3 = core::ptr::addr_of_mut!(_1);
RET = [4_usize,4_usize,1_usize];
_2 = !_1;
_1 = !_2;
_2 = !(*_3);
_1 = _2;
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = !_2;
(*_3) = _2;
(*_3) = _2;
RET = [4_usize,3_usize,7021693162736183865_usize];
RET = [15752113104330480988_usize,9500009500911364335_usize,1918835810440202267_usize];
RET = [4_usize,6_usize,3063931205238845347_usize];
(*_3) = _2 != _2;
_2 = _1;
RET = [7_usize,5754147621189822717_usize,10647994786400316139_usize];
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = !_2;
_1 = !_2;
RET = [4_usize,17266830896246455258_usize,17208607332211339537_usize];
Goto(bb1)
}
bb1 = {
(*_3) = _2;
(*_3) = _2 == _2;
(*_3) = !_2;
_1 = _2;
_4 = _1 < (*_3);
RET = [16986850815534496385_usize,13520464023155093692_usize,14850803786133207304_usize];
RET = [18001025395712403139_usize,0_usize,2_usize];
(*_3) = !_4;
RET = [7_usize,6_usize,2809839568252915271_usize];
(*_3) = !_4;
RET = [4_usize,3_usize,1_usize];
_4 = !(*_3);
_3 = core::ptr::addr_of_mut!((*_3));
RET = [12378766035687318697_usize,14860973402346105189_usize,13585063727987213299_usize];
_5 = -9223372036854775807_isize;
_5 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_4 = _2;
_2 = !_4;
_1 = _2 & _2;
RET = [6674114334778533935_usize,5_usize,13799768357025915291_usize];
RET = [3624849447723487740_usize,11267843951782264178_usize,8178546271716170168_usize];
RET = [1_usize,4_usize,15590838571094734115_usize];
_2 = _1;
_5 = (-83_isize) - 9223372036854775807_isize;
(*_3) = _2;
_1 = !_2;
_6 = _5 * _5;
_7 = 16437635908585734733_u64 as i16;
Call(_7 = fn10(_2, _2, _4, _4, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
(*_3) = _2;
_4 = _2 & _1;
_5 = !_6;
RET = [14527566112096352301_usize,10854735721628741577_usize,5_usize];
(*_3) = _2 & _4;
_1 = !_2;
RET = [1_usize,9987612480028152683_usize,1_usize];
RET = [13262305659095468380_usize,7_usize,4083399795038727505_usize];
_1 = _4;
Goto(bb3)
}
bb3 = {
_2 = !(*_3);
_3 = core::ptr::addr_of_mut!(_2);
RET = [2_usize,5_usize,1_usize];
RET = [3839227705665115595_usize,7242704451003795875_usize,6_usize];
_4 = _2;
_3 = core::ptr::addr_of_mut!(_4);
_7 = !9498_i16;
_6 = '\u{4cc1e}' as isize;
(*_3) = !_1;
_1 = (*_3) | _2;
_1 = !(*_3);
RET = [7934656122094326074_usize,2_usize,3393002571787760840_usize];
_1 = _4 > _4;
RET = [6_usize,15811299778606020290_usize,18275250111319216941_usize];
_5 = _6 | _6;
RET = [3_usize,8195084258169598991_usize,2260215578783649193_usize];
Goto(bb4)
}
bb4 = {
_2 = _1 ^ _1;
_11.fld1 = [139176221517458859342912483197196983450_i128,(-19425363443440220166364264194065145541_i128),(-54413425920589715054159339028072583157_i128),55000317311413493908701877821183800726_i128,(-92082847394374571961287394010349208105_i128)];
_10 = _6;
_9 = Adt41 { fld0: (-7790172208095623204_i64),fld1: _11.fld1 };
_11 = Adt41 { fld0: _9.fld0,fld1: _9.fld1 };
Goto(bb5)
}
bb5 = {
_7 = (-7321_i16) - (-15718_i16);
_5 = !_10;
_5 = -_10;
_4 = !_1;
_1 = (*_3);
_11 = Adt41 { fld0: _9.fld0,fld1: _9.fld1 };
_11.fld1 = _9.fld1;
RET = [13592774026393221536_usize,0_usize,4_usize];
_6 = _5 << _11.fld0;
_9.fld1 = _11.fld1;
(*_3) = !_2;
match _9.fld0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb6,
4 => bb7,
340282366920938463455584435223672588252 => bb9,
_ => bb8
}
}
bb6 = {
(*_3) = _2;
(*_3) = _2 == _2;
(*_3) = !_2;
_1 = _2;
_4 = _1 < (*_3);
RET = [16986850815534496385_usize,13520464023155093692_usize,14850803786133207304_usize];
RET = [18001025395712403139_usize,0_usize,2_usize];
(*_3) = !_4;
RET = [7_usize,6_usize,2809839568252915271_usize];
(*_3) = !_4;
RET = [4_usize,3_usize,1_usize];
_4 = !(*_3);
_3 = core::ptr::addr_of_mut!((*_3));
RET = [12378766035687318697_usize,14860973402346105189_usize,13585063727987213299_usize];
_5 = -9223372036854775807_isize;
_5 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_4 = _2;
_2 = !_4;
_1 = _2 & _2;
RET = [6674114334778533935_usize,5_usize,13799768357025915291_usize];
RET = [3624849447723487740_usize,11267843951782264178_usize,8178546271716170168_usize];
RET = [1_usize,4_usize,15590838571094734115_usize];
_2 = _1;
_5 = (-83_isize) - 9223372036854775807_isize;
(*_3) = _2;
_1 = !_2;
_6 = _5 * _5;
_7 = 16437635908585734733_u64 as i16;
Call(_7 = fn10(_2, _2, _4, _4, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_2 = !(*_3);
_3 = core::ptr::addr_of_mut!(_2);
RET = [2_usize,5_usize,1_usize];
RET = [3839227705665115595_usize,7242704451003795875_usize,6_usize];
_4 = _2;
_3 = core::ptr::addr_of_mut!(_4);
_7 = !9498_i16;
_6 = '\u{4cc1e}' as isize;
(*_3) = !_1;
_1 = (*_3) | _2;
_1 = !(*_3);
RET = [7934656122094326074_usize,2_usize,3393002571787760840_usize];
_1 = _4 > _4;
RET = [6_usize,15811299778606020290_usize,18275250111319216941_usize];
_5 = _6 | _6;
RET = [3_usize,8195084258169598991_usize,2260215578783649193_usize];
Goto(bb4)
}
bb8 = {
(*_3) = _2;
_4 = _2 & _1;
_5 = !_6;
RET = [14527566112096352301_usize,10854735721628741577_usize,5_usize];
(*_3) = _2 & _4;
_1 = !_2;
RET = [1_usize,9987612480028152683_usize,1_usize];
RET = [13262305659095468380_usize,7_usize,4083399795038727505_usize];
_1 = _4;
Goto(bb3)
}
bb9 = {
_14 = [(-9387478_i32),109861342_i32,130768329_i32,1621470509_i32,71947574_i32,1320680130_i32,(-2014996442_i32),(-1135980006_i32)];
_15.fld0 = _9.fld0 << _9.fld0;
_15.fld1 = _9.fld1;
_3 = core::ptr::addr_of_mut!(_1);
_12 = [1_usize,15693241776488903387_usize,5_usize,6_usize,1_usize,4_usize,2_usize];
_13 = [1293818359_i32,(-286338038_i32),(-1614488241_i32),(-2028180576_i32),2073223537_i32,2140735064_i32];
_15 = Adt41 { fld0: _9.fld0,fld1: _9.fld1 };
Goto(bb10)
}
bb10 = {
_5 = _10;
_1 = _4;
_11 = Adt41 { fld0: _15.fld0,fld1: _15.fld1 };
_11.fld1 = [70023313314637421702816607149744335096_i128,40221203180232207469804764291822989280_i128,162894696635897143526812826038849061287_i128,(-77648648951626988402101020514456991459_i128),19907865725711531733387226864115472170_i128];
_11.fld1 = _15.fld1;
_2 = (*_3);
_10 = 2454732851042765094_u64 as isize;
_7 = '\u{4a892}' as i16;
_9.fld1 = [32388429291087937632793376921804198293_i128,154516006928196413432206705960219946473_i128,(-33582869917550640581041799363606800308_i128),(-82014300878587237990434964972731634801_i128),(-152958875762971794670202583643549360249_i128)];
_18 = '\u{a6d9c}';
Call(_9.fld1 = fn14(_2, (*_3), _1, _1, _1, _3, _4, _2, _1, _1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
(*_3) = !_4;
_6 = -_5;
_2 = _4;
_20 = _10 + _6;
_17 = _9.fld0;
_16 = [23104_u16,31156_u16,44583_u16,52590_u16,17158_u16,12861_u16,47550_u16];
_15.fld1 = _11.fld1;
_11.fld1 = [100136527125565472056531799065586482164_i128,(-50343675413279541270622209352196029421_i128),143174304330836450211111648487456450620_i128,146372100109644927920689814878727006558_i128,78125489672537823203193034574276578325_i128];
_12 = [4_usize,6_usize,5816506958242099175_usize,2_usize,9729936309717971417_usize,12518039447214648268_usize,6_usize];
_10 = !_20;
_9.fld0 = _11.fld0;
_6 = -_5;
_9.fld1 = [(-90496063104753724145623832001958503639_i128),(-96289895779793459911895307620699220772_i128),77465912270688705284128930280508325169_i128,141603872324273225247254187377056101040_i128,140458378150496963790430794141299225086_i128];
_12 = [4438827618944421031_usize,1_usize,13416628221870953495_usize,7_usize,2_usize,11774670121075557213_usize,4_usize];
_9 = Adt41 { fld0: _11.fld0,fld1: _11.fld1 };
match _9.fld0 {
0 => bb7,
340282366920938463455584435223672588252 => bb13,
_ => bb12
}
}
bb12 = {
_2 = !(*_3);
_3 = core::ptr::addr_of_mut!(_2);
RET = [2_usize,5_usize,1_usize];
RET = [3839227705665115595_usize,7242704451003795875_usize,6_usize];
_4 = _2;
_3 = core::ptr::addr_of_mut!(_4);
_7 = !9498_i16;
_6 = '\u{4cc1e}' as isize;
(*_3) = !_1;
_1 = (*_3) | _2;
_1 = !(*_3);
RET = [7934656122094326074_usize,2_usize,3393002571787760840_usize];
_1 = _4 > _4;
RET = [6_usize,15811299778606020290_usize,18275250111319216941_usize];
_5 = _6 | _6;
RET = [3_usize,8195084258169598991_usize,2260215578783649193_usize];
Goto(bb4)
}
bb13 = {
(*_3) = _2;
_21 = !1552547176_u32;
_13 = [1269264880_i32,2087426004_i32,454142504_i32,(-1324143875_i32),605512370_i32,1033682127_i32];
_21 = !1681479398_u32;
Goto(bb14)
}
bb14 = {
_9.fld1 = [81246310595313795773289042200622174666_i128,(-43756057843521442219514723810818791400_i128),(-68585106651599703236760321615119839049_i128),(-146444292024615557964572700723931710114_i128),51521662149224389124179687562376498132_i128];
(*_3) = _4;
(*_3) = !_2;
(*_3) = !_2;
_9.fld0 = (-158495674991999653707812792411518690419_i128) as i64;
_16 = [59750_u16,25323_u16,39010_u16,7102_u16,56955_u16,17408_u16,10926_u16];
_15.fld0 = -_11.fld0;
_17 = _18 as i64;
_2 = !_1;
_22.fld0 = [15863193834664263668_usize,3_usize,0_usize];
_19 = -_6;
_11.fld1 = _9.fld1;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(9_usize, 14_usize, Move(_14), 17_usize, Move(_17), 7_usize, Move(_7), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(9_usize, 5_usize, Move(_5), 16_usize, Move(_16), 21_usize, Move(_21), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool) -> i16 {
mir! {
type RET = i16;
let _6: char;
let _7: Adt48;
let _8: (f32, &'static isize, u16);
let _9: f32;
let _10: f64;
let _11: [usize; 7];
let _12: f32;
let _13: char;
let _14: [u16; 7];
let _15: Adt46;
let _16: Adt41;
let _17: bool;
let _18: isize;
let _19: Adt51;
let _20: f64;
let _21: (bool, (*mut char, [usize; 7]), i64, i8, i16, i32, [u8; 4]);
let _22: *mut i16;
let _23: char;
let _24: [u16; 7];
let _25: Adt48;
let _26: (f32, u8, i32, u8);
let _27: Adt54;
let _28: (bool, (*mut char, [usize; 7]), i64, i8, i16, i32, [u8; 4]);
let _29: [usize; 7];
let _30: i64;
let _31: *mut *mut isize;
let _32: (f32, &'static isize, u16);
let _33: usize;
let _34: bool;
let _35: u8;
let _36: Adt45;
let _37: [i128; 5];
let _38: Adt41;
let _39: ();
let _40: ();
{
_3 = _1 <= _4;
_5 = _4;
_3 = _2;
RET = 5111_i16 & 5719_i16;
_3 = !_4;
_5 = !_2;
RET = -18184_i16;
Goto(bb1)
}
bb1 = {
_3 = !_1;
_5 = _1;
_7.fld0 = [6_usize,2_usize,1701868083263047392_usize,17802415083708624398_usize,5_usize,10734885335923613097_usize,6_usize];
_2 = !_3;
_1 = _4 < _2;
_2 = !_1;
Goto(bb2)
}
bb2 = {
_8.2 = !10433_u16;
_5 = _3 | _1;
_8.0 = (-8566861113538804180_i64) as f32;
_2 = _1;
_2 = _3 != _4;
_8.2 = 27575_u16;
_3 = _4 != _1;
RET = _8.0 as i16;
_9 = -_8.0;
_7.fld2 = ['\u{abb33}','\u{76038}','\u{ae4e1}','\u{29240}','\u{ab34a}'];
_8.2 = 17859_u16;
_5 = _3;
_7.fld2 = ['\u{eec0d}','\u{b3a32}','\u{2a546}','\u{ef4e2}','\u{46854}'];
_6 = '\u{1f123}';
_7.fld0 = [8836102976916577831_usize,4936137317791776998_usize,92914048852814849_usize,2834113549551507369_usize,17921622519463841176_usize,3414155798117534414_usize,445397367147713486_usize];
Goto(bb3)
}
bb3 = {
_11 = _7.fld0;
_8.2 = (-10_i8) as u16;
RET = 15466_i16;
_13 = _6;
_7.fld2 = [_13,_13,_6,_6,_13];
_4 = _5;
_13 = _6;
_10 = 4_i8 as f64;
_7.fld1 = _8.0;
RET = 3727371081602737214_i64 as i16;
_16.fld1 = [(-4890115842340799377560898619713536080_i128),(-109662315066910953198860698788693139088_i128),(-93374912528100162207663877117101642263_i128),(-43401401944794747681305553507160606713_i128),(-129742815400398785105460859739634447441_i128)];
_16.fld0 = 7180423409770471264_i64 & 4667379487327532314_i64;
_7.fld2 = [_6,_13,_13,_6,_6];
_11 = [10797194154356889955_usize,2_usize,0_usize,5_usize,6730134238734683991_usize,17998323245296715452_usize,12082415870605107383_usize];
_17 = !_4;
_8.2 = (-50_isize) as u16;
_1 = !_3;
_14 = [_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2];
_11 = [14944275324817488338_usize,6_usize,0_usize,6811526571003760491_usize,4_usize,1_usize,2_usize];
_17 = _2 <= _4;
_4 = !_17;
_4 = _3;
Call(_4 = fn11(_17, Move(_16), _1, _1, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_7.fld2 = [_6,_13,_13,_13,_13];
_16.fld1 = [69710481642148917862323921375243767897_i128,132318358749572015514826459028439882714_i128,149281378144216022416800937563004254895_i128,(-125421670846013705425288974548821650812_i128),(-95985967542452943903958155046733687273_i128)];
_8.1 = &_18;
_16.fld0 = 6070533173204328225_i64 * 6654338708634963092_i64;
_16.fld1 = [(-97173778344584438655244217338246912434_i128),79266708183714516799487922587309071662_i128,(-22060581504887834495270265631970143354_i128),48212903558484657334600068132327939156_i128,91551479673764021125299810592229046023_i128];
_11 = [3_usize,12504992434243155864_usize,4_usize,6993891554285101054_usize,7_usize,16292986730138855051_usize,7_usize];
_12 = _8.0 - _7.fld1;
_7.fld2 = [_13,_6,_13,_6,_13];
_16.fld0 = -6996514399889559979_i64;
_18 = 150_u8 as isize;
_18 = (-9223372036854775808_isize);
_8.1 = &_18;
_18 = 9223372036854775807_isize >> RET;
_11 = [12554204199539017111_usize,2_usize,0_usize,2_usize,6_usize,1048832064883728623_usize,9639034842396076437_usize];
_2 = _17;
_13 = _6;
_1 = _4 > _3;
_13 = _6;
_14 = [_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2];
_1 = !_5;
_5 = _4 >= _17;
RET = (-7030_i16);
_21.2 = 16241528387666691699_u64 as i64;
_8.2 = 44744_u16;
_21.3 = 89_i8;
_2 = _1;
_21.6 = [29_u8,247_u8,73_u8,33_u8];
Goto(bb5)
}
bb5 = {
_18 = 9223372036854775807_isize & (-99_isize);
_21.1.0 = core::ptr::addr_of_mut!(_13);
Call(_20 = core::intrinsics::transmute(_16.fld0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_21.4 = !RET;
_15 = Adt46::Variant1 { fld0: _21.4,fld1: _10 };
_25 = Adt48 { fld0: _11,fld1: _8.0,fld2: _7.fld2 };
_14 = [_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2];
_8.0 = _12;
_21.0 = _2;
_9 = _7.fld1 + _8.0;
_21.1.1 = [2_usize,4_usize,7_usize,0_usize,6_usize,6432707940380220493_usize,194379688226705185_usize];
_14 = [_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2];
Call(_21.0 = fn12(_4, _16.fld1, _17, _5, _4, _14, _3, _5, _3, _4), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_9 = _8.0 + _8.0;
_7 = Adt48 { fld0: _11,fld1: _9,fld2: _25.fld2 };
_21.1.1 = _7.fld0;
_21.5 = 2045319756_i32 << Field::<i16>(Variant(_15, 1), 0);
_25 = Adt48 { fld0: _21.1.1,fld1: _8.0,fld2: _7.fld2 };
SetDiscriminant(_15, 0);
_12 = _7.fld1;
_7.fld1 = _12 + _9;
_13 = _6;
_16.fld0 = _12 as i64;
_21.5 = 105_u8 as i32;
Call(_26.3 = core::intrinsics::bswap(17_u8), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_18 = _8.2 as isize;
_21.1.1 = [15062638308442663970_usize,6_usize,9395307770803843900_usize,3_usize,2_usize,5_usize,13122353011305099040_usize];
_16.fld0 = -_21.2;
RET = 179_u8 as i16;
Goto(bb9)
}
bb9 = {
_26.0 = _9;
_8.1 = &_18;
_28.3 = _21.3 << _21.4;
_21.1.0 = core::ptr::addr_of_mut!(_6);
_14 = [_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2];
_26.2 = RET as i32;
_22 = core::ptr::addr_of_mut!(RET);
_7.fld1 = _8.2 as f32;
_23 = _13;
Call(_21.2 = fn13(_4, _21.0, _1, _16.fld1, _1, _21.0, _17, _3, _2, _3, _17, _4, _21.1, _17, _17, _2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_11 = _21.1.1;
place!(Field::<*mut char>(Variant(_15, 0), 0)) = core::ptr::addr_of_mut!(_6);
_10 = _20 - _20;
_29 = [4_usize,852024983618060929_usize,1_usize,9141544594770156700_usize,6841482630131867712_usize,7847852943233952232_usize,4401636903409329802_usize];
_29 = [0_usize,11495767160370062228_usize,17521544252234032327_usize,3_usize,3_usize,2874355737376036143_usize,12128449029668116858_usize];
_21.3 = _28.3 >> _26.2;
_17 = _3;
_21.1.1 = [2_usize,5_usize,12119373776857288108_usize,16881324015397537979_usize,7414193297042183671_usize,4_usize,12939281010354635924_usize];
(*_22) = _21.4 & _21.4;
_2 = _4;
_8.0 = _12;
_25.fld1 = 96_u8 as f32;
_21.3 = _26.0 as i8;
_15 = Adt46::Variant0 { fld0: _21.1.0 };
_28.1.1 = [5_usize,3538575778910227258_usize,1_usize,7027074285989856142_usize,6_usize,2_usize,0_usize];
match _21.2 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
8931420316422118816 => bb16,
_ => bb15
}
}
bb11 = {
_26.0 = _9;
_8.1 = &_18;
_28.3 = _21.3 << _21.4;
_21.1.0 = core::ptr::addr_of_mut!(_6);
_14 = [_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2];
_26.2 = RET as i32;
_22 = core::ptr::addr_of_mut!(RET);
_7.fld1 = _8.2 as f32;
_23 = _13;
Call(_21.2 = fn13(_4, _21.0, _1, _16.fld1, _1, _21.0, _17, _3, _2, _3, _17, _4, _21.1, _17, _17, _2), ReturnTo(bb10), UnwindUnreachable())
}
bb12 = {
_18 = _8.2 as isize;
_21.1.1 = [15062638308442663970_usize,6_usize,9395307770803843900_usize,3_usize,2_usize,5_usize,13122353011305099040_usize];
_16.fld0 = -_21.2;
RET = 179_u8 as i16;
Goto(bb9)
}
bb13 = {
_9 = _8.0 + _8.0;
_7 = Adt48 { fld0: _11,fld1: _9,fld2: _25.fld2 };
_21.1.1 = _7.fld0;
_21.5 = 2045319756_i32 << Field::<i16>(Variant(_15, 1), 0);
_25 = Adt48 { fld0: _21.1.1,fld1: _8.0,fld2: _7.fld2 };
SetDiscriminant(_15, 0);
_12 = _7.fld1;
_7.fld1 = _12 + _9;
_13 = _6;
_16.fld0 = _12 as i64;
_21.5 = 105_u8 as i32;
Call(_26.3 = core::intrinsics::bswap(17_u8), ReturnTo(bb8), UnwindUnreachable())
}
bb14 = {
_3 = !_1;
_5 = _1;
_7.fld0 = [6_usize,2_usize,1701868083263047392_usize,17802415083708624398_usize,5_usize,10734885335923613097_usize,6_usize];
_2 = !_3;
_1 = _4 < _2;
_2 = !_1;
Goto(bb2)
}
bb15 = {
_7.fld2 = [_6,_13,_13,_13,_13];
_16.fld1 = [69710481642148917862323921375243767897_i128,132318358749572015514826459028439882714_i128,149281378144216022416800937563004254895_i128,(-125421670846013705425288974548821650812_i128),(-95985967542452943903958155046733687273_i128)];
_8.1 = &_18;
_16.fld0 = 6070533173204328225_i64 * 6654338708634963092_i64;
_16.fld1 = [(-97173778344584438655244217338246912434_i128),79266708183714516799487922587309071662_i128,(-22060581504887834495270265631970143354_i128),48212903558484657334600068132327939156_i128,91551479673764021125299810592229046023_i128];
_11 = [3_usize,12504992434243155864_usize,4_usize,6993891554285101054_usize,7_usize,16292986730138855051_usize,7_usize];
_12 = _8.0 - _7.fld1;
_7.fld2 = [_13,_6,_13,_6,_13];
_16.fld0 = -6996514399889559979_i64;
_18 = 150_u8 as isize;
_18 = (-9223372036854775808_isize);
_8.1 = &_18;
_18 = 9223372036854775807_isize >> RET;
_11 = [12554204199539017111_usize,2_usize,0_usize,2_usize,6_usize,1048832064883728623_usize,9639034842396076437_usize];
_2 = _17;
_13 = _6;
_1 = _4 > _3;
_13 = _6;
_14 = [_8.2,_8.2,_8.2,_8.2,_8.2,_8.2,_8.2];
_1 = !_5;
_5 = _4 >= _17;
RET = (-7030_i16);
_21.2 = 16241528387666691699_u64 as i64;
_8.2 = 44744_u16;
_21.3 = 89_i8;
_2 = _1;
_21.6 = [29_u8,247_u8,73_u8,33_u8];
Goto(bb5)
}
bb16 = {
_25 = _7;
_17 = _2 ^ _5;
_25 = Adt48 { fld0: _7.fld0,fld1: _8.0,fld2: _7.fld2 };
_34 = _21.0 <= _1;
_28.4 = -(*_22);
(*_22) = _28.3 as i16;
_26.2 = 60810095796539551834685086540083346466_u128 as i32;
_28.3 = _21.3 ^ _21.3;
_26.3 = !93_u8;
Goto(bb17)
}
bb17 = {
Call(_39 = dump_var(10_usize, 5_usize, Move(_5), 34_usize, Move(_34), 29_usize, Move(_29), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_39 = dump_var(10_usize, 4_usize, Move(_4), 6_usize, Move(_6), 13_usize, Move(_13), 40_usize, _40), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: bool,mut _2: Adt41,mut _3: bool,mut _4: bool,mut _5: bool) -> bool {
mir! {
type RET = bool;
let _6: ();
let _7: ();
{
_3 = _1 | _1;
_2.fld0 = !(-8470425439083838171_i64);
RET = !_5;
_5 = RET;
RET = _3;
_5 = _4 < _4;
_2.fld1 = [(-128353343549845976001814582853312162485_i128),93356652817199721619455375948973853629_i128,(-19808523932340864421804722747577298704_i128),(-25582987732534917176691400901268777448_i128),100904670232976366137811474764762557439_i128];
RET = _4 >= _1;
_4 = RET & _5;
RET = _4 < _3;
_2.fld0 = (-6167266142348387234_i64) << 88_i8;
_1 = !_5;
_2.fld1 = [12214614100907696465165799187200745481_i128,24871687796011308400466011980169480979_i128,(-114070434379697196446125870253184369413_i128),(-52036019687012710531578381936136401835_i128),49773694603355894972617001406964999962_i128];
Goto(bb1)
}
bb1 = {
Call(_6 = dump_var(11_usize, 1_usize, Move(_1), 5_usize, Move(_5), 7_usize, _7, 7_usize, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: bool,mut _2: [i128; 5],mut _3: bool,mut _4: bool,mut _5: bool,mut _6: [u16; 7],mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool) -> bool {
mir! {
type RET = bool;
let _11: *mut char;
let _12: i16;
let _13: [char; 5];
let _14: *mut char;
let _15: ();
let _16: ();
{
RET = _10;
_3 = _10 & RET;
_2 = [102783211228547530571615596488458725972_i128,55982976882829293046756841650268562126_i128,109933365110556956268698315423689503574_i128,(-84117577703549850529439528960829665015_i128),(-46730771685670912389986135743867937031_i128)];
_9 = _4;
_9 = _3;
_5 = RET;
_3 = _8;
_7 = _8 & _1;
_8 = _5 < _9;
_1 = _4 | _9;
_1 = _3;
_6 = [16980_u16,38658_u16,47884_u16,17210_u16,61836_u16,35203_u16,60917_u16];
_4 = _3 >= _10;
RET = _1;
_7 = !_5;
RET = _4 <= _8;
_10 = _3;
_3 = _5;
_8 = _7;
_4 = !_9;
_10 = _3 <= _4;
_4 = _1 > _5;
_13 = ['\u{7d71c}','\u{e4bfc}','\u{b1ac6}','\u{53971}','\u{5d4}'];
_9 = _3 >= _8;
_1 = !RET;
Goto(bb1)
}
bb1 = {
Call(_15 = dump_var(12_usize, 6_usize, Move(_6), 2_usize, Move(_2), 13_usize, Move(_13), 10_usize, Move(_10)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_15 = dump_var(12_usize, 9_usize, Move(_9), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: [i128; 5],mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: bool,mut _12: bool,mut _13: (*mut char, [usize; 7]),mut _14: bool,mut _15: bool,mut _16: bool) -> i64 {
mir! {
type RET = i64;
let _17: f32;
let _18: Adt51;
let _19: isize;
let _20: bool;
let _21: Adt40;
let _22: Adt44;
let _23: i32;
let _24: f64;
let _25: f32;
let _26: [usize; 7];
let _27: isize;
let _28: i16;
let _29: f64;
let _30: f32;
let _31: Adt41;
let _32: ();
let _33: ();
{
_17 = 20174_i16 as f32;
_3 = _16;
Goto(bb1)
}
bb1 = {
_3 = _7 <= _1;
RET = -4569961460907398666_i64;
_4 = [153248498810061952280948071624363366380_i128,126364970732254895570218886682568799803_i128,(-38081798113375649421942638891918438643_i128),46772370762495417871250233647094452659_i128,7465638682690995708577226975786905974_i128];
_2 = _8 != _15;
_5 = _16;
_5 = _6;
_9 = !_14;
_3 = _16;
_17 = (-118_i8) as f32;
Goto(bb2)
}
bb2 = {
_11 = !_14;
_11 = _8 | _1;
_10 = !_7;
_19 = !(-9223372036854775808_isize);
_8 = _7;
_22 = Adt44 { fld0: 43_i8 };
_4 = [(-134534095064216571907420349205819796029_i128),73575796147076903866055877861415874861_i128,59786638115093548448826151153828862770_i128,24266015955832077761266612754720671638_i128,23100291302838325888114041677973500561_i128];
_4 = [106521477520076783489568942655881020104_i128,(-80549328117670287303652002421967695335_i128),(-156233468216241091520600076192825186818_i128),31031576674692441852515477340438594048_i128,(-138481764026582062616711082399293305918_i128)];
Goto(bb3)
}
bb3 = {
_17 = 1161098724_i32 as f32;
_6 = _14;
match _22.fld0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
43 => bb11,
_ => bb10
}
}
bb4 = {
_11 = !_14;
_11 = _8 | _1;
_10 = !_7;
_19 = !(-9223372036854775808_isize);
_8 = _7;
_22 = Adt44 { fld0: 43_i8 };
_4 = [(-134534095064216571907420349205819796029_i128),73575796147076903866055877861415874861_i128,59786638115093548448826151153828862770_i128,24266015955832077761266612754720671638_i128,23100291302838325888114041677973500561_i128];
_4 = [106521477520076783489568942655881020104_i128,(-80549328117670287303652002421967695335_i128),(-156233468216241091520600076192825186818_i128),31031576674692441852515477340438594048_i128,(-138481764026582062616711082399293305918_i128)];
Goto(bb3)
}
bb5 = {
_3 = _7 <= _1;
RET = -4569961460907398666_i64;
_4 = [153248498810061952280948071624363366380_i128,126364970732254895570218886682568799803_i128,(-38081798113375649421942638891918438643_i128),46772370762495417871250233647094452659_i128,7465638682690995708577226975786905974_i128];
_2 = _8 != _15;
_5 = _16;
_5 = _6;
_9 = !_14;
_3 = _16;
_17 = (-118_i8) as f32;
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
_14 = _5;
_11 = !_9;
match _22.fld0 {
0 => bb5,
1 => bb4,
2 => bb12,
3 => bb13,
43 => bb15,
_ => bb14
}
}
bb12 = {
_17 = 1161098724_i32 as f32;
_6 = _14;
match _22.fld0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
43 => bb11,
_ => bb10
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_8 = _1 | _9;
_10 = _1;
_7 = !_5;
RET = 8931420316422118816_i64;
_24 = _17 as f64;
_16 = _7;
_6 = _16;
_2 = !_3;
_23 = 737558209_i32 << _22.fld0;
_20 = !_11;
_24 = (-22034_i16) as f64;
_3 = _2;
_5 = !_3;
_9 = _8;
_16 = _3;
_9 = _8;
_22 = Adt44 { fld0: 11_i8 };
_22 = Adt44 { fld0: 27_i8 };
_12 = !_20;
_20 = !_10;
_4 = [(-125601496385546785099724763060475596648_i128),41596547403825750255996305400273772462_i128,(-96200380453467178736776823629144814453_i128),(-29873914420183458355403714385458616321_i128),102298414403694005094989036801487630625_i128];
_26 = [4382962734459704880_usize,3322728170433151616_usize,9254109370980242613_usize,10700861217251880246_usize,1_usize,6_usize,5_usize];
Goto(bb16)
}
bb16 = {
Call(_32 = dump_var(13_usize, 1_usize, Move(_1), 16_usize, Move(_16), 14_usize, Move(_14), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(13_usize, 19_usize, Move(_19), 23_usize, Move(_23), 3_usize, Move(_3), 20_usize, Move(_20)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_32 = dump_var(13_usize, 2_usize, Move(_2), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: *mut bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool) -> [i128; 5] {
mir! {
type RET = [i128; 5];
let _11: f64;
let _12: Adt41;
let _13: [usize; 3];
let _14: Adt45;
let _15: (f32, &'static isize, u16);
let _16: Adt41;
let _17: [i32; 6];
let _18: [u16; 7];
let _19: char;
let _20: f32;
let _21: isize;
let _22: Adt48;
let _23: i64;
let _24: u32;
let _25: bool;
let _26: (f32, u8, i32, u8);
let _27: Adt56;
let _28: (f32, &'static isize, u16);
let _29: (f64, (*mut char, [usize; 7]), i8, char);
let _30: [char; 5];
let _31: bool;
let _32: u32;
let _33: bool;
let _34: *mut char;
let _35: f64;
let _36: ();
let _37: ();
{
(*_6) = _7;
RET = [(-38278634923762907254153877155691345866_i128),141689022308778533489203369412340424520_i128,68165959975848049035911230254766760640_i128,(-80605704991959399724808785002009260771_i128),(-76748751467091116659261549714726267352_i128)];
Goto(bb1)
}
bb1 = {
_9 = !_7;
RET = [(-82393969154367303643329017812682701342_i128),78932217479657016977625964225307573779_i128,(-5346877497327537150396768342147573170_i128),37283064364450706536866011224390757030_i128,(-123497573015298296681623256577087351088_i128)];
_10 = !_5;
_10 = !_4;
_10 = _5;
(*_6) = !_8;
(*_6) = !_7;
_11 = 31582248837047092969115809941644630606_i128 as f64;
_9 = (*_6) & (*_6);
_3 = _9 >= (*_6);
(*_6) = _10;
Goto(bb2)
}
bb2 = {
RET = [36799357379470959158777845466760937279_i128,(-50408624723876969169510555380241155100_i128),(-94668900303241497730243639034165507570_i128),(-106003275197856592459281078919202705263_i128),88315093416319014574375837246083278362_i128];
_4 = (*_6) | _1;
_5 = _1 ^ _1;
RET = [(-24254797977352663179167241622800224300_i128),155396407560207644521345978154122883789_i128,(-101343289437717367961302600251920687037_i128),93345314912876276085480481316385984830_i128,105524745349203299499311319243988462107_i128];
_1 = _7;
_7 = !_9;
_5 = _2 ^ _10;
_1 = !_3;
_1 = _10;
(*_6) = !_9;
_9 = !_1;
_8 = _9;
_10 = !_9;
RET = [(-24101645588265081708812806549383620243_i128),7953822891947496387452851036282986731_i128,123584984007392001994065602710165683024_i128,68220546829582893595086411090031216949_i128,42808897239180255166794944483946144611_i128];
(*_6) = _5 < _8;
_13 = [3_usize,10017265905308362497_usize,3_usize];
Goto(bb3)
}
bb3 = {
_5 = _1;
_10 = !_1;
(*_6) = _3 == _3;
_11 = 2930830225_u32 as f64;
_4 = !_1;
Goto(bb4)
}
bb4 = {
_12 = Adt41 { fld0: 1127245017750743654_i64,fld1: RET };
_4 = _2;
(*_6) = _1;
_3 = _7;
RET = _12.fld1;
_5 = !_3;
(*_6) = _4 & _10;
_3 = (*_6) == (*_6);
_12.fld0 = 2_usize as i64;
_13 = [2_usize,18302014350011153705_usize,6_usize];
_2 = _10 > _7;
_16 = Adt41 { fld0: _12.fld0,fld1: _12.fld1 };
(*_6) = _10 <= _3;
_7 = !_2;
_8 = _1;
_4 = _3 == _3;
_18 = [32441_u16,62179_u16,7978_u16,24694_u16,11652_u16,6374_u16,60795_u16];
_4 = _9 | (*_6);
_5 = (*_6) ^ _3;
_12.fld0 = 71_u8 as i64;
(*_6) = _10;
_16.fld1 = _12.fld1;
Goto(bb5)
}
bb5 = {
_9 = _7;
_4 = !_3;
_2 = _8;
_18 = [42347_u16,35470_u16,39685_u16,59054_u16,13423_u16,214_u16,11977_u16];
_18 = [17242_u16,53015_u16,22676_u16,10815_u16,8529_u16,24168_u16,54716_u16];
_12.fld1 = RET;
_15.0 = 50_i8 as f32;
_18 = [21422_u16,4477_u16,29964_u16,25939_u16,805_u16,16303_u16,22745_u16];
_16.fld0 = -_12.fld0;
RET = _12.fld1;
_7 = !_8;
_5 = _2;
_16.fld1 = [(-80903786319601640300376448541399121839_i128),123058700636743263055845343713242693744_i128,(-19873411464437640333483045401930997749_i128),(-35865647113857316869178708044636428204_i128),85067230076873636937742669482747766633_i128];
_11 = _16.fld0 as f64;
_12.fld0 = _16.fld0 ^ _16.fld0;
_2 = (*_6) <= _8;
_16 = Adt41 { fld0: _12.fld0,fld1: _12.fld1 };
_15.2 = 2711928420_u32 as u16;
_24 = 321446142982137765616045253417414601588_u128 as u32;
(*_6) = _2 < _5;
_9 = _10;
_15.0 = 13324_i16 as f32;
_15.0 = 12698_i16 as f32;
_19 = '\u{792a4}';
_23 = _12.fld0 - _12.fld0;
_8 = !_1;
(*_6) = _8 & _9;
Goto(bb6)
}
bb6 = {
_7 = !_2;
_16.fld1 = [(-261570467228816728945221466694186913_i128),9366420888447414122271543283399567980_i128,128016273890037029386715068235375864438_i128,38918125225430669864969026919408906433_i128,(-82351912526818474357097881949934894255_i128)];
_15.2 = _16.fld0 as u16;
_15.1 = &_21;
_20 = _15.0;
_5 = _10 <= (*_6);
_15.2 = !49175_u16;
_22.fld1 = (-20343_i16) as f32;
_15.2 = 291820515534077728330759217043381978502_u128 as u16;
_15.1 = &_21;
_22.fld2 = [_19,_19,_19,_19,_19];
_20 = (-84_i8) as f32;
_24 = 1719837869_u32;
_19 = '\u{1fd47}';
match _24 {
0 => bb2,
1 => bb7,
2 => bb8,
3 => bb9,
1719837869 => bb11,
_ => bb10
}
}
bb7 = {
_9 = _7;
_4 = !_3;
_2 = _8;
_18 = [42347_u16,35470_u16,39685_u16,59054_u16,13423_u16,214_u16,11977_u16];
_18 = [17242_u16,53015_u16,22676_u16,10815_u16,8529_u16,24168_u16,54716_u16];
_12.fld1 = RET;
_15.0 = 50_i8 as f32;
_18 = [21422_u16,4477_u16,29964_u16,25939_u16,805_u16,16303_u16,22745_u16];
_16.fld0 = -_12.fld0;
RET = _12.fld1;
_7 = !_8;
_5 = _2;
_16.fld1 = [(-80903786319601640300376448541399121839_i128),123058700636743263055845343713242693744_i128,(-19873411464437640333483045401930997749_i128),(-35865647113857316869178708044636428204_i128),85067230076873636937742669482747766633_i128];
_11 = _16.fld0 as f64;
_12.fld0 = _16.fld0 ^ _16.fld0;
_2 = (*_6) <= _8;
_16 = Adt41 { fld0: _12.fld0,fld1: _12.fld1 };
_15.2 = 2711928420_u32 as u16;
_24 = 321446142982137765616045253417414601588_u128 as u32;
(*_6) = _2 < _5;
_9 = _10;
_15.0 = 13324_i16 as f32;
_15.0 = 12698_i16 as f32;
_19 = '\u{792a4}';
_23 = _12.fld0 - _12.fld0;
_8 = !_1;
(*_6) = _8 & _9;
Goto(bb6)
}
bb8 = {
_12 = Adt41 { fld0: 1127245017750743654_i64,fld1: RET };
_4 = _2;
(*_6) = _1;
_3 = _7;
RET = _12.fld1;
_5 = !_3;
(*_6) = _4 & _10;
_3 = (*_6) == (*_6);
_12.fld0 = 2_usize as i64;
_13 = [2_usize,18302014350011153705_usize,6_usize];
_2 = _10 > _7;
_16 = Adt41 { fld0: _12.fld0,fld1: _12.fld1 };
(*_6) = _10 <= _3;
_7 = !_2;
_8 = _1;
_4 = _3 == _3;
_18 = [32441_u16,62179_u16,7978_u16,24694_u16,11652_u16,6374_u16,60795_u16];
_4 = _9 | (*_6);
_5 = (*_6) ^ _3;
_12.fld0 = 71_u8 as i64;
(*_6) = _10;
_16.fld1 = _12.fld1;
Goto(bb5)
}
bb9 = {
_5 = _1;
_10 = !_1;
(*_6) = _3 == _3;
_11 = 2930830225_u32 as f64;
_4 = !_1;
Goto(bb4)
}
bb10 = {
RET = [36799357379470959158777845466760937279_i128,(-50408624723876969169510555380241155100_i128),(-94668900303241497730243639034165507570_i128),(-106003275197856592459281078919202705263_i128),88315093416319014574375837246083278362_i128];
_4 = (*_6) | _1;
_5 = _1 ^ _1;
RET = [(-24254797977352663179167241622800224300_i128),155396407560207644521345978154122883789_i128,(-101343289437717367961302600251920687037_i128),93345314912876276085480481316385984830_i128,105524745349203299499311319243988462107_i128];
_1 = _7;
_7 = !_9;
_5 = _2 ^ _10;
_1 = !_3;
_1 = _10;
(*_6) = !_9;
_9 = !_1;
_8 = _9;
_10 = !_9;
RET = [(-24101645588265081708812806549383620243_i128),7953822891947496387452851036282986731_i128,123584984007392001994065602710165683024_i128,68220546829582893595086411090031216949_i128,42808897239180255166794944483946144611_i128];
(*_6) = _5 < _8;
_13 = [3_usize,10017265905308362497_usize,3_usize];
Goto(bb3)
}
bb11 = {
_30 = _22.fld2;
_15.1 = &_21;
_21 = _24 as isize;
_29.2 = 59_i8 - (-71_i8);
_3 = _9 >= _1;
_13 = [14286289162162327840_usize,1_usize,9476985575902266843_usize];
(*_6) = _10 <= _7;
_1 = !_4;
_16 = Move(_12);
_22.fld0 = [5_usize,5_usize,2258177248596637929_usize,7_usize,3_usize,13850405100463536749_usize,2_usize];
match _24 {
0 => bb6,
1 => bb2,
2 => bb12,
3 => bb13,
4 => bb14,
1719837869 => bb16,
_ => bb15
}
}
bb12 = {
_12 = Adt41 { fld0: 1127245017750743654_i64,fld1: RET };
_4 = _2;
(*_6) = _1;
_3 = _7;
RET = _12.fld1;
_5 = !_3;
(*_6) = _4 & _10;
_3 = (*_6) == (*_6);
_12.fld0 = 2_usize as i64;
_13 = [2_usize,18302014350011153705_usize,6_usize];
_2 = _10 > _7;
_16 = Adt41 { fld0: _12.fld0,fld1: _12.fld1 };
(*_6) = _10 <= _3;
_7 = !_2;
_8 = _1;
_4 = _3 == _3;
_18 = [32441_u16,62179_u16,7978_u16,24694_u16,11652_u16,6374_u16,60795_u16];
_4 = _9 | (*_6);
_5 = (*_6) ^ _3;
_12.fld0 = 71_u8 as i64;
(*_6) = _10;
_16.fld1 = _12.fld1;
Goto(bb5)
}
bb13 = {
_5 = _1;
_10 = !_1;
(*_6) = _3 == _3;
_11 = 2930830225_u32 as f64;
_4 = !_1;
Goto(bb4)
}
bb14 = {
_5 = _1;
_10 = !_1;
(*_6) = _3 == _3;
_11 = 2930830225_u32 as f64;
_4 = !_1;
Goto(bb4)
}
bb15 = {
_9 = _7;
_4 = !_3;
_2 = _8;
_18 = [42347_u16,35470_u16,39685_u16,59054_u16,13423_u16,214_u16,11977_u16];
_18 = [17242_u16,53015_u16,22676_u16,10815_u16,8529_u16,24168_u16,54716_u16];
_12.fld1 = RET;
_15.0 = 50_i8 as f32;
_18 = [21422_u16,4477_u16,29964_u16,25939_u16,805_u16,16303_u16,22745_u16];
_16.fld0 = -_12.fld0;
RET = _12.fld1;
_7 = !_8;
_5 = _2;
_16.fld1 = [(-80903786319601640300376448541399121839_i128),123058700636743263055845343713242693744_i128,(-19873411464437640333483045401930997749_i128),(-35865647113857316869178708044636428204_i128),85067230076873636937742669482747766633_i128];
_11 = _16.fld0 as f64;
_12.fld0 = _16.fld0 ^ _16.fld0;
_2 = (*_6) <= _8;
_16 = Adt41 { fld0: _12.fld0,fld1: _12.fld1 };
_15.2 = 2711928420_u32 as u16;
_24 = 321446142982137765616045253417414601588_u128 as u32;
(*_6) = _2 < _5;
_9 = _10;
_15.0 = 13324_i16 as f32;
_15.0 = 12698_i16 as f32;
_19 = '\u{792a4}';
_23 = _12.fld0 - _12.fld0;
_8 = !_1;
(*_6) = _8 & _9;
Goto(bb6)
}
bb16 = {
_19 = '\u{d13f2}';
_1 = _9;
_26.2 = _22.fld1 as i32;
_29.1.1 = _22.fld0;
_16.fld0 = -_23;
_12.fld0 = -_16.fld0;
_26 = (_20, 162_u8, (-791728540_i32), 59_u8);
_17 = [_26.2,_26.2,_26.2,_26.2,_26.2,_26.2];
_1 = _2 >= _4;
_12.fld1 = [22018442029488727257207472748696395218_i128,110744138759562701582667125654382349733_i128,160317891942821141979287306639616773961_i128,(-54730099445642033792270810150033699000_i128),41893712953302360988548993942359836742_i128];
_32 = _24 | _24;
_1 = _9 & (*_6);
_28.0 = _15.0;
_28.2 = !_15.2;
_12 = Adt41 { fld0: _16.fld0,fld1: RET };
RET = [128181951784328887775211820336650965068_i128,(-149677197775889255519723322168146632157_i128),(-14484207268480128587718158170409258222_i128),(-24996137739939486510796651040043449387_i128),(-2025752774101233561004536072158000539_i128)];
_26.1 = _26.3;
_5 = _3 != _3;
_19 = '\u{ebc6e}';
_29.3 = _19;
_16.fld1 = _12.fld1;
_20 = _21 as f32;
Goto(bb17)
}
bb17 = {
Call(_36 = dump_var(14_usize, 9_usize, Move(_9), 4_usize, Move(_4), 13_usize, Move(_13), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_36 = dump_var(14_usize, 5_usize, Move(_5), 8_usize, Move(_8), 32_usize, Move(_32), 30_usize, Move(_30)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_36 = dump_var(14_usize, 23_usize, Move(_23), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: bool,mut _2: bool,mut _3: [u8; 7],mut _4: bool,mut _5: bool) -> *mut *mut isize {
mir! {
type RET = *mut *mut isize;
let _6: [i32; 8];
let _7: [u8; 7];
let _8: isize;
let _9: bool;
let _10: Adt44;
let _11: Adt46;
let _12: i16;
let _13: (f32, u8, i32, u8);
let _14: (f32, u8, i32, u8);
let _15: *const usize;
let _16: [char; 5];
let _17: [i128; 5];
let _18: [usize; 7];
let _19: [usize; 7];
let _20: Adt44;
let _21: f32;
let _22: Adt40;
let _23: (f32, u8, i32, u8);
let _24: usize;
let _25: char;
let _26: char;
let _27: [u8; 4];
let _28: i64;
let _29: [u8; 4];
let _30: bool;
let _31: [usize; 3];
let _32: isize;
let _33: (bool, (*mut char, [usize; 7]), i64, i8, i16, i32, [u8; 4]);
let _34: bool;
let _35: (f32, u8, i32, u8);
let _36: isize;
let _37: [i32; 8];
let _38: isize;
let _39: u8;
let _40: [i128; 5];
let _41: [usize; 7];
let _42: f32;
let _43: char;
let _44: [u16; 7];
let _45: Adt47;
let _46: *mut char;
let _47: Adt48;
let _48: f64;
let _49: [i128; 5];
let _50: (f32, u8, i32, u8);
let _51: u8;
let _52: Adt51;
let _53: [u16; 7];
let _54: i32;
let _55: f32;
let _56: (f32, u8, i32, u8);
let _57: isize;
let _58: u32;
let _59: [i128; 5];
let _60: f64;
let _61: char;
let _62: u64;
let _63: [i32; 8];
let _64: [u16; 7];
let _65: Adt41;
let _66: [char; 5];
let _67: i32;
let _68: [i32; 8];
let _69: Adt41;
let _70: [i128; 5];
let _71: (f32, u8, i32, u8);
let _72: isize;
let _73: [i32; 6];
let _74: Adt51;
let _75: *mut isize;
let _76: *mut i16;
let _77: ();
let _78: ();
{
_5 = !_2;
_3 = [159_u8,148_u8,218_u8,96_u8,64_u8,136_u8,84_u8];
_3 = [66_u8,175_u8,70_u8,25_u8,40_u8,206_u8,117_u8];
_3 = [42_u8,177_u8,176_u8,240_u8,12_u8,169_u8,125_u8];
_1 = _4 > _4;
_4 = _2;
_3 = [3_u8,18_u8,91_u8,193_u8,154_u8,45_u8,11_u8];
_5 = _1;
_2 = _4;
_2 = _1 != _1;
_3 = [166_u8,127_u8,180_u8,0_u8,143_u8,18_u8,106_u8];
_2 = _4;
_3 = [161_u8,154_u8,188_u8,163_u8,219_u8,149_u8,17_u8];
_2 = _5 | _4;
_4 = _1 | _2;
_7 = _3;
_5 = !_2;
_7 = [167_u8,228_u8,216_u8,128_u8,123_u8,163_u8,168_u8];
_6 = [612741993_i32,(-2011832631_i32),(-1243898770_i32),642512560_i32,10412983_i32,1502729390_i32,1102551423_i32,(-1243976551_i32)];
_5 = _2 > _2;
_3 = [184_u8,30_u8,156_u8,171_u8,180_u8,4_u8,102_u8];
_4 = !_2;
_3 = _7;
_5 = _4 <= _2;
_8 = -93_isize;
Goto(bb1)
}
bb1 = {
_5 = _2;
_3 = [113_u8,8_u8,239_u8,112_u8,211_u8,68_u8,33_u8];
_2 = _1;
Goto(bb2)
}
bb2 = {
_8 = !(-9223372036854775808_isize);
_6 = [706660843_i32,(-234152377_i32),390733047_i32,2092135120_i32,1865542331_i32,1788252148_i32,667591224_i32,(-516363620_i32)];
_7 = [116_u8,9_u8,221_u8,32_u8,220_u8,57_u8,9_u8];
_6 = [464573053_i32,(-869509335_i32),1154674772_i32,(-1142900752_i32),(-587657376_i32),1555752733_i32,(-1271461906_i32),(-31274668_i32)];
_5 = !_4;
_1 = _5 & _5;
Goto(bb3)
}
bb3 = {
_10 = Adt44 { fld0: (-101_i8) };
match _10.fld0 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
340282366920938463463374607431768211355 => bb10,
_ => bb9
}
}
bb4 = {
_8 = !(-9223372036854775808_isize);
_6 = [706660843_i32,(-234152377_i32),390733047_i32,2092135120_i32,1865542331_i32,1788252148_i32,667591224_i32,(-516363620_i32)];
_7 = [116_u8,9_u8,221_u8,32_u8,220_u8,57_u8,9_u8];
_6 = [464573053_i32,(-869509335_i32),1154674772_i32,(-1142900752_i32),(-587657376_i32),1555752733_i32,(-1271461906_i32),(-31274668_i32)];
_5 = !_4;
_1 = _5 & _5;
Goto(bb3)
}
bb5 = {
_5 = _2;
_3 = [113_u8,8_u8,239_u8,112_u8,211_u8,68_u8,33_u8];
_2 = _1;
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
_6 = [1325576395_i32,365667370_i32,(-1974154472_i32),826057201_i32,1531141774_i32,1763329510_i32,(-579908432_i32),(-444658117_i32)];
_12 = (-7366_i16) << _8;
_10 = Adt44 { fld0: 89_i8 };
_10.fld0 = _8 as i8;
_9 = !_5;
_12 = 30912_i16 * 5281_i16;
Goto(bb11)
}
bb11 = {
_5 = !_1;
_13.2 = 2119292749_i32;
_13.0 = _12 as f32;
_6 = [_13.2,_13.2,_13.2,_13.2,_13.2,_13.2,_13.2,_13.2];
_8 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_9 = _4;
_4 = _1;
_13.1 = 220_u8 << _12;
_4 = _1 < _9;
_9 = _2;
_13.3 = !_13.1;
Goto(bb12)
}
bb12 = {
_10.fld0 = (-72_i8) - 67_i8;
_2 = _4;
_13.1 = _13.3 + _13.3;
_14 = _13;
_2 = !_5;
_8 = (-9223372036854775808_isize);
_7 = [_13.1,_13.1,_13.1,_13.3,_14.3,_13.1,_14.1];
_6 = [_13.2,_14.2,_14.2,_14.2,_14.2,_13.2,_13.2,_14.2];
_4 = !_2;
_14.1 = _13.2 as u8;
_13.3 = _14.3;
Goto(bb13)
}
bb13 = {
_5 = _2 | _1;
_8 = 49499_u16 as isize;
_10.fld0 = 60_i8;
_2 = _14.3 != _13.1;
_2 = _9;
Goto(bb14)
}
bb14 = {
_9 = _5 > _2;
_9 = !_2;
_2 = _14.3 < _13.3;
_16 = ['\u{bd9ae}','\u{4f902}','\u{78373}','\u{37d29}','\u{bcfb4}'];
_14.2 = _13.2 << _13.1;
_12 = !13233_i16;
_13.3 = _14.3;
_13.2 = '\u{77a38}' as i32;
_17 = [(-55585839989821922117117695417022088900_i128),13054908321057541347914795625703040037_i128,168472149010446013678764317087783406013_i128,(-131887222616216250471842078498490610118_i128),131541395068067169467324207056976810041_i128];
_14.3 = _13.1;
_14.3 = _14.1;
_8 = (-9223372036854775808_isize) << _13.1;
Goto(bb15)
}
bb15 = {
_17 = [(-141788798120943425477662505093920285379_i128),32329157009636309387242456187028035406_i128,(-60076733143540487094215740141056031762_i128),(-14299246232512934097491090040333399454_i128),(-102842862404156611718987589329243801884_i128)];
_2 = !_1;
_8 = 3278501649_u32 as isize;
_5 = _9;
_9 = _5;
_20 = Adt44 { fld0: _10.fld0 };
_2 = !_1;
_13.0 = 2194476232816429517_i64 as f32;
_14 = (_13.0, _13.3, _13.2, _13.1);
_16 = ['\u{10a81b}','\u{6a7f2}','\u{594d2}','\u{f26a4}','\u{9c781}'];
_9 = !_1;
_20.fld0 = '\u{e0b89}' as i8;
_14.3 = _13.1;
_6 = [_13.2,_13.2,_13.2,_14.2,_14.2,_13.2,_13.2,_14.2];
_19 = [9938499576354247489_usize,13263406674614294196_usize,4833059887906747698_usize,9694818765552351144_usize,13766323443774197651_usize,13410152802017874880_usize,206018713949448228_usize];
_14.3 = !_13.1;
_21 = _10.fld0 as f32;
_20 = Adt44 { fld0: _10.fld0 };
_23.3 = _14.1;
_20.fld0 = _10.fld0;
_17 = [145858698581784009015647877316081146394_i128,69175737604440448774166073809814062812_i128,(-156564620036052617520170695174333744106_i128),16991305406849690742708330878611872462_i128,(-158076514546982342941901536032792745012_i128)];
_18 = [1_usize,4_usize,13355960726253737381_usize,10538702129597591536_usize,3843801096741071938_usize,6356553002661946998_usize,1_usize];
_23 = _13;
_14.2 = _12 as i32;
_23 = (_13.0, _14.3, _13.2, _13.3);
Call(_20.fld0 = core::intrinsics::transmute(_2), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_25 = '\u{e6e9f}';
_7 = [_23.1,_23.1,_13.1,_13.1,_14.3,_13.3,_23.3];
_13.3 = !_13.1;
_2 = _9 | _4;
_16 = [_25,_25,_25,_25,_25];
_25 = '\u{e776b}';
_21 = _20.fld0 as f32;
_16 = [_25,_25,_25,_25,_25];
_13.1 = _23.1;
_10.fld0 = 97864585556239440900776234379189238578_i128 as i8;
_7 = [_13.3,_13.3,_14.3,_13.3,_23.1,_14.3,_13.1];
_15 = core::ptr::addr_of!(_24);
_7 = [_23.1,_13.1,_14.3,_13.1,_14.1,_14.3,_13.3];
_7 = _3;
_10.fld0 = _25 as i8;
_27 = [_14.3,_14.1,_13.1,_14.3];
_14.2 = _23.2 >> _20.fld0;
_13.0 = _8 as f32;
_9 = _5;
_9 = _4 | _1;
_27 = [_14.3,_14.1,_23.1,_14.3];
_26 = _25;
_1 = _20.fld0 >= _20.fld0;
_4 = !_2;
_14.2 = !_13.2;
Call(_19 = core::intrinsics::transmute(_18), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
_26 = _25;
_23.2 = (-12176445924083374260833435897183150772_i128) as i32;
_4 = _1;
Goto(bb18)
}
bb18 = {
_3 = [_13.3,_23.1,_14.3,_23.3,_13.3,_23.1,_23.3];
_13.2 = !_23.2;
_7 = [_23.1,_13.3,_13.3,_14.1,_13.3,_13.3,_13.3];
_18 = [4_usize,11738207529425770344_usize,8751599077811875014_usize,1100141172231519697_usize,6714780840022049558_usize,5_usize,5_usize];
_18 = [2596964996500375767_usize,7_usize,17313407098715040858_usize,6_usize,5_usize,6_usize,11434890455447586458_usize];
_14.0 = _12 as f32;
_13.0 = 3043034391_u32 as f32;
_13.0 = (-2539839002776023838_i64) as f32;
_21 = _14.0 - _14.0;
_27 = [_14.3,_13.1,_14.3,_13.3];
Goto(bb19)
}
bb19 = {
_10 = Move(_20);
_15 = core::ptr::addr_of!(_24);
_24 = !5885102859165085780_usize;
_27 = [_13.3,_23.1,_23.1,_14.1];
_10 = Adt44 { fld0: 105_i8 };
_14.0 = -_13.0;
_20 = Adt44 { fld0: _10.fld0 };
_14.1 = _23.1;
_23.3 = (-77913374567699179189737565642835693460_i128) as u8;
_30 = !_1;
_6 = [_14.2,_13.2,_13.2,_23.2,_23.2,_23.2,_14.2,_14.2];
_14 = (_21, _13.1, _13.2, _13.1);
_25 = _26;
_15 = core::ptr::addr_of!(_24);
_5 = _1;
Goto(bb20)
}
bb20 = {
_17 = [(-134684726039313568684535648656466802565_i128),136490620953503469433234494635548734567_i128,10257298158403122833455250203886962363_i128,(-84157778627550055072624557889961278437_i128),19975471780902101726543219548380162743_i128];
_23.3 = _13.1 ^ _13.3;
_33.3 = _20.fld0 + _10.fld0;
_33.0 = _2 & _5;
_13.0 = 64081_u16 as f32;
_33.1.1 = [(*_15),(*_15),_24,(*_15),(*_15),(*_15),(*_15)];
_5 = _2;
_18 = _19;
_33.4 = !_12;
_13.3 = _14.3 * _23.3;
_26 = _25;
_15 = core::ptr::addr_of!(_24);
_1 = _30 < _2;
_16 = [_25,_26,_26,_26,_25];
_20 = Adt44 { fld0: _10.fld0 };
_13.2 = _8 as i32;
_32 = _21 as isize;
_6 = [_13.2,_13.2,_14.2,_14.2,_23.2,_23.2,_23.2,_23.2];
_10.fld0 = 14088671278940672492649879696613317999_i128 as i8;
_10.fld0 = _33.3;
_35.0 = _23.3 as f32;
_33.6 = [_14.1,_23.3,_13.3,_23.3];
_7 = [_14.3,_14.3,_14.1,_13.1,_14.3,_14.3,_14.3];
_31 = [_24,(*_15),(*_15)];
match _20.fld0 {
0 => bb13,
1 => bb15,
2 => bb18,
3 => bb10,
4 => bb5,
5 => bb16,
6 => bb21,
105 => bb23,
_ => bb22
}
}
bb21 = {
_10.fld0 = (-72_i8) - 67_i8;
_2 = _4;
_13.1 = _13.3 + _13.3;
_14 = _13;
_2 = !_5;
_8 = (-9223372036854775808_isize);
_7 = [_13.1,_13.1,_13.1,_13.3,_14.3,_13.1,_14.1];
_6 = [_13.2,_14.2,_14.2,_14.2,_14.2,_13.2,_13.2,_14.2];
_4 = !_2;
_14.1 = _13.2 as u8;
_13.3 = _14.3;
Goto(bb13)
}
bb22 = {
_26 = _25;
_23.2 = (-12176445924083374260833435897183150772_i128) as i32;
_4 = _1;
Goto(bb18)
}
bb23 = {
_34 = _30;
_27 = [_14.3,_13.3,_23.3,_13.3];
_24 = 15745978301353478379_usize - 961297293256300076_usize;
_12 = _33.4 >> _32;
_33.0 = _4 < _9;
(*_15) = 10297094525272013133_usize;
_8 = _32;
Goto(bb24)
}
bb24 = {
_35.1 = !_23.3;
_22 = Adt40::Variant3 { fld0: _14.3,fld1: _13,fld2: _15 };
_7 = _3;
SetDiscriminant(_22, 0);
_39 = _14.3 - _14.1;
_33.6 = [_39,_13.3,_13.3,_23.3];
place!(Field::<*mut i16>(Variant(_22, 0), 4)) = core::ptr::addr_of_mut!(_33.4);
_36 = _33.4 as isize;
_33.1.0 = core::ptr::addr_of_mut!(_25);
place!(Field::<[i32; 6]>(Variant(_22, 0), 5)) = [_14.2,_13.2,_23.2,_14.2,_23.2,_14.2];
_35.2 = _23.2 | _14.2;
_40 = [68625167539901314355968418598969125055_i128,131945313840716065092803074700582134192_i128,(-54834198667247317784830872873394692848_i128),(-32473560352057628579976848023008526705_i128),154286234841308452062938587079183584615_i128];
Goto(bb25)
}
bb25 = {
_23.1 = !_14.3;
_20.fld0 = _13.2 as i8;
place!(Field::<char>(Variant(_22, 0), 1)) = _25;
_10.fld0 = _20.fld0 | _33.3;
_13.3 = !_14.1;
_29 = [_39,_23.3,_23.3,_23.1];
_6 = [_13.2,_13.2,_13.2,_14.2,_13.2,_35.2,_13.2,_35.2];
_27 = _29;
_37 = _6;
_19 = [(*_15),(*_15),_24,(*_15),(*_15),(*_15),_24];
_35 = (_21, _39, _14.2, _23.1);
_3 = [_35.3,_39,_35.3,_39,_23.3,_39,_35.1];
place!(Field::<*mut bool>(Variant(_22, 0), 6)) = core::ptr::addr_of_mut!(_2);
_21 = _35.0 * _14.0;
_46 = core::ptr::addr_of_mut!(_26);
_33.5 = -_35.2;
place!(Field::<*mut i16>(Variant(_22, 0), 4)) = core::ptr::addr_of_mut!(_33.4);
_3 = _7;
_13 = (_14.0, _14.1, _14.2, _14.3);
_36 = _8;
_33.2 = 8983500185994281520_i64 >> _13.1;
_45 = Adt47::Variant0 { fld0: _29,fld1: _24,fld2: 78906537889593462504944358874101705828_i128,fld3: Field::<*mut bool>(Variant(_22, 0), 6),fld4: _12 };
Goto(bb26)
}
bb26 = {
_40 = [(-71024062368208463297216315868185799820_i128),107883630746996787309189567054539762332_i128,141567188620388177855997504064100057593_i128,14450184673078759968972365358602360616_i128,(-4133507645408905733665767573304713981_i128)];
_16 = [_26,Field::<char>(Variant(_22, 0), 1),_25,Field::<char>(Variant(_22, 0), 1),_26];
place!(Field::<i128>(Variant(_45, 0), 2)) = 169127303101117364590707875710066293410_i128;
_23 = (_35.0, _35.3, _13.2, _35.1);
_48 = _32 as f64;
_23.0 = _13.0 + _13.0;
_35.1 = _39 - _13.1;
_17 = [Field::<i128>(Variant(_45, 0), 2),Field::<i128>(Variant(_45, 0), 2),Field::<i128>(Variant(_45, 0), 2),Field::<i128>(Variant(_45, 0), 2),Field::<i128>(Variant(_45, 0), 2)];
place!(Field::<u64>(Variant(_22, 0), 3)) = _33.2 as u64;
_45 = Adt47::Variant0 { fld0: _29,fld1: (*_15),fld2: (-99020531584316317263696590837845009335_i128),fld3: Field::<*mut bool>(Variant(_22, 0), 6),fld4: _12 };
_33.1.1 = _18;
_22 = Adt40::Variant3 { fld0: _13.1,fld1: _35,fld2: _15 };
_14.1 = _12 as u8;
_41 = [(*_15),(*_15),(*_15),_24,(*_15),(*_15),_24];
_47 = Adt48 { fld0: _33.1.1,fld1: _23.0,fld2: _16 };
Goto(bb27)
}
bb27 = {
_17 = [(-47746554787758955562688585881724494120_i128),(-18474953044722861127105857634980746662_i128),154355391210432903269680636178647661483_i128,101267540000989751770895775629806567095_i128,(-35158801170640336966706389967045713925_i128)];
_46 = _33.1.0;
place!(Field::<i128>(Variant(_45, 0), 2)) = _12 as i128;
_29 = [_35.1,_35.1,_35.1,_35.1];
_33.6 = [Field::<(f32, u8, i32, u8)>(Variant(_22, 3), 1).1,_23.3,_13.3,_13.1];
_50.0 = _47.fld1 + _14.0;
_56.2 = _33.5;
_4 = _33.0 ^ _2;
_3 = [_23.3,_35.3,_35.1,_39,Field::<(f32, u8, i32, u8)>(Variant(_22, 3), 1).1,_35.3,Field::<(f32, u8, i32, u8)>(Variant(_22, 3), 1).1];
_42 = _33.3 as f32;
place!(Field::<usize>(Variant(_45, 0), 1)) = _24 & (*_15);
_25 = _26;
_44 = [33506_u16,6850_u16,37921_u16,30198_u16,15691_u16,3214_u16,40513_u16];
_33.1.1 = [(*_15),_24,(*_15),(*_15),(*_15),(*_15),(*_15)];
match (*_15) {
0 => bb28,
1 => bb29,
2 => bb30,
3 => bb31,
4 => bb32,
5 => bb33,
10297094525272013133 => bb35,
_ => bb34
}
}
bb28 = {
Return()
}
bb29 = {
Return()
}
bb30 = {
_5 = !_1;
_13.2 = 2119292749_i32;
_13.0 = _12 as f32;
_6 = [_13.2,_13.2,_13.2,_13.2,_13.2,_13.2,_13.2,_13.2];
_8 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_9 = _4;
_4 = _1;
_13.1 = 220_u8 << _12;
_4 = _1 < _9;
_9 = _2;
_13.3 = !_13.1;
Goto(bb12)
}
bb31 = {
_34 = _30;
_27 = [_14.3,_13.3,_23.3,_13.3];
_24 = 15745978301353478379_usize - 961297293256300076_usize;
_12 = _33.4 >> _32;
_33.0 = _4 < _9;
(*_15) = 10297094525272013133_usize;
_8 = _32;
Goto(bb24)
}
bb32 = {
_5 = _2;
_3 = [113_u8,8_u8,239_u8,112_u8,211_u8,68_u8,33_u8];
_2 = _1;
Goto(bb2)
}
bb33 = {
_9 = _5 > _2;
_9 = !_2;
_2 = _14.3 < _13.3;
_16 = ['\u{bd9ae}','\u{4f902}','\u{78373}','\u{37d29}','\u{bcfb4}'];
_14.2 = _13.2 << _13.1;
_12 = !13233_i16;
_13.3 = _14.3;
_13.2 = '\u{77a38}' as i32;
_17 = [(-55585839989821922117117695417022088900_i128),13054908321057541347914795625703040037_i128,168472149010446013678764317087783406013_i128,(-131887222616216250471842078498490610118_i128),131541395068067169467324207056976810041_i128];
_14.3 = _13.1;
_14.3 = _14.1;
_8 = (-9223372036854775808_isize) << _13.1;
Goto(bb15)
}
bb34 = {
_17 = [(-134684726039313568684535648656466802565_i128),136490620953503469433234494635548734567_i128,10257298158403122833455250203886962363_i128,(-84157778627550055072624557889961278437_i128),19975471780902101726543219548380162743_i128];
_23.3 = _13.1 ^ _13.3;
_33.3 = _20.fld0 + _10.fld0;
_33.0 = _2 & _5;
_13.0 = 64081_u16 as f32;
_33.1.1 = [(*_15),(*_15),_24,(*_15),(*_15),(*_15),(*_15)];
_5 = _2;
_18 = _19;
_33.4 = !_12;
_13.3 = _14.3 * _23.3;
_26 = _25;
_15 = core::ptr::addr_of!(_24);
_1 = _30 < _2;
_16 = [_25,_26,_26,_26,_25];
_20 = Adt44 { fld0: _10.fld0 };
_13.2 = _8 as i32;
_32 = _21 as isize;
_6 = [_13.2,_13.2,_14.2,_14.2,_23.2,_23.2,_23.2,_23.2];
_10.fld0 = 14088671278940672492649879696613317999_i128 as i8;
_10.fld0 = _33.3;
_35.0 = _23.3 as f32;
_33.6 = [_14.1,_23.3,_13.3,_23.3];
_7 = [_14.3,_14.3,_14.1,_13.1,_14.3,_14.3,_14.3];
_31 = [_24,(*_15),(*_15)];
match _20.fld0 {
0 => bb13,
1 => bb15,
2 => bb18,
3 => bb10,
4 => bb5,
5 => bb16,
6 => bb21,
105 => bb23,
_ => bb22
}
}
bb35 = {
_35.3 = !_13.1;
place!(Field::<[u8; 4]>(Variant(_45, 0), 0)) = _33.6;
_11 = Adt46::Variant0 { fld0: _46 };
place!(Field::<(f32, u8, i32, u8)>(Variant(_22, 3), 1)).1 = !_23.1;
_52 = Adt51::Variant2 { fld0: _48,fld1: _25,fld2: _14 };
_56.1 = !_13.3;
_38 = !_8;
place!(Field::<*const usize>(Variant(_22, 3), 2)) = _15;
_14 = (_35.0, _23.3, _56.2, _13.3);
place!(Field::<(f32, u8, i32, u8)>(Variant(_52, 2), 2)).0 = _33.2 as f32;
_27 = _29;
_61 = _26;
_56.3 = Field::<i128>(Variant(_45, 0), 2) as u8;
_41 = [(*_15),_24,_24,_24,(*_15),Field::<usize>(Variant(_45, 0), 1),(*_15)];
_50 = _23;
_50 = _14;
place!(Field::<(f32, u8, i32, u8)>(Variant(_52, 2), 2)).3 = _35.1 | _35.1;
_54 = _13.2 + Field::<(f32, u8, i32, u8)>(Variant(_52, 2), 2).2;
match (*_15) {
0 => bb8,
1 => bb29,
2 => bb27,
3 => bb34,
4 => bb5,
5 => bb36,
6 => bb37,
10297094525272013133 => bb39,
_ => bb38
}
}
bb36 = {
_34 = _30;
_27 = [_14.3,_13.3,_23.3,_13.3];
_24 = 15745978301353478379_usize - 961297293256300076_usize;
_12 = _33.4 >> _32;
_33.0 = _4 < _9;
(*_15) = 10297094525272013133_usize;
_8 = _32;
Goto(bb24)
}
bb37 = {
Return()
}
bb38 = {
_10.fld0 = (-72_i8) - 67_i8;
_2 = _4;
_13.1 = _13.3 + _13.3;
_14 = _13;
_2 = !_5;
_8 = (-9223372036854775808_isize);
_7 = [_13.1,_13.1,_13.1,_13.3,_14.3,_13.1,_14.1];
_6 = [_13.2,_14.2,_14.2,_14.2,_14.2,_13.2,_13.2,_14.2];
_4 = !_2;
_14.1 = _13.2 as u8;
_13.3 = _14.3;
Goto(bb13)
}
bb39 = {
_14.3 = _23.3;
place!(Field::<(f32, u8, i32, u8)>(Variant(_22, 3), 1)).1 = 2039233159_u32 as u8;
_42 = -Field::<(f32, u8, i32, u8)>(Variant(_22, 3), 1).0;
_47.fld1 = _50.0 - _50.0;
_34 = !_4;
_23.0 = _21 + _21;
_1 = !_30;
_26 = Field::<char>(Variant(_52, 2), 1);
_23.3 = Field::<i128>(Variant(_45, 0), 2) as u8;
_19 = [Field::<usize>(Variant(_45, 0), 1),(*_15),(*_15),(*_15),(*_15),(*_15),Field::<usize>(Variant(_45, 0), 1)];
_18 = _19;
_23 = (_47.fld1, Field::<(f32, u8, i32, u8)>(Variant(_52, 2), 2).3, _50.2, _35.3);
(*_15) = Field::<usize>(Variant(_45, 0), 1) - Field::<usize>(Variant(_45, 0), 1);
Goto(bb40)
}
bb40 = {
_50.0 = _23.0 + _42;
_33.1.1 = [(*_15),_24,_24,Field::<usize>(Variant(_45, 0), 1),(*_15),(*_15),(*_15)];
_59 = [Field::<i128>(Variant(_45, 0), 2),Field::<i128>(Variant(_45, 0), 2),Field::<i128>(Variant(_45, 0), 2),Field::<i128>(Variant(_45, 0), 2),Field::<i128>(Variant(_45, 0), 2)];
SetDiscriminant(_11, 1);
place!(Field::<u8>(Variant(_22, 3), 0)) = _23.1 | _23.3;
_58 = !1769484293_u32;
_10 = Adt44 { fld0: _33.3 };
Goto(bb41)
}
bb41 = {
_58 = _35.1 as u32;
_11 = Adt46::Variant0 { fld0: _33.1.0 };
_46 = _33.1.0;
_24 = !Field::<usize>(Variant(_45, 0), 1);
place!(Field::<i128>(Variant(_45, 0), 2)) = Field::<f64>(Variant(_52, 2), 0) as i128;
_56.3 = !_23.1;
_8 = -_32;
_56.2 = _32 as i32;
_49 = _17;
_63 = [_14.2,_23.2,_54,_33.5,_14.2,_54,Field::<(f32, u8, i32, u8)>(Variant(_22, 3), 1).2,_13.2];
SetDiscriminant(_52, 0);
_33.1.1 = _18;
_13.3 = !Field::<u8>(Variant(_22, 3), 0);
_56.0 = -_50.0;
_28 = !_33.2;
_65.fld1 = _59;
_23 = (_13.0, _56.3, _56.2, _56.3);
Goto(bb42)
}
bb42 = {
place!(Field::<*mut i16>(Variant(_52, 0), 0)) = core::ptr::addr_of_mut!(place!(Field::<i16>(Variant(_45, 0), 4)));
_71.0 = -_50.0;
(*_15) = _32 as usize;
_45 = Adt47::Variant1 { fld0: _44,fld1: (*_46),fld2: Move(_10),fld3: _47.fld2 };
_14.1 = _13.3 + Field::<u8>(Variant(_22, 3), 0);
_23.0 = _71.0 * _56.0;
_56.1 = _35.1;
_3 = _7;
_53 = [40455_u16,30388_u16,58727_u16,11751_u16,32605_u16,27157_u16,59878_u16];
_56.1 = _2 as u8;
_70 = [(-54143379342448669033096027550213602780_i128),(-107524714074125982198310637694045142417_i128),93464891509064425527788744520311046805_i128,(-159858245888241472685925264907411605785_i128),136488802183666837842335956891465226636_i128];
_35.1 = _23.3;
_50.3 = _23.3 * _23.3;
_69 = Adt41 { fld0: _33.2,fld1: _40 };
_59 = _40;
_64 = [60619_u16,37592_u16,16469_u16,40662_u16,32961_u16,56227_u16,269_u16];
_10.fld0 = _20.fld0 - _33.3;
place!(Field::<Adt43>(Variant(_52, 0), 1)) = Adt43::Variant0 { fld0: Field::<[u16; 7]>(Variant(_45, 1), 0),fld1: _69.fld0 };
Goto(bb43)
}
bb43 = {
_33.3 = _10.fld0;
_50 = (_23.0, _56.1, Field::<(f32, u8, i32, u8)>(Variant(_22, 3), 1).2, _23.1);
RET = core::ptr::addr_of_mut!(_75);
SetDiscriminant(Field::<Adt43>(Variant(_52, 0), 1), 2);
RET = core::ptr::addr_of_mut!((*RET));
_57 = _36 >> _12;
_14 = (_50.0, _50.1, _50.2, _50.1);
Goto(bb44)
}
bb44 = {
Call(_77 = dump_var(15_usize, 36_usize, Move(_36), 57_usize, Move(_57), 70_usize, Move(_70), 26_usize, Move(_26)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
Call(_77 = dump_var(15_usize, 49_usize, Move(_49), 1_usize, Move(_1), 3_usize, Move(_3), 5_usize, Move(_5)), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
Call(_77 = dump_var(15_usize, 54_usize, Move(_54), 4_usize, Move(_4), 2_usize, Move(_2), 34_usize, Move(_34)), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
Call(_77 = dump_var(15_usize, 41_usize, Move(_41), 31_usize, Move(_31), 16_usize, Move(_16), 44_usize, Move(_44)), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
Call(_77 = dump_var(15_usize, 58_usize, Move(_58), 63_usize, Move(_63), 27_usize, Move(_27), 53_usize, Move(_53)), ReturnTo(bb49), UnwindUnreachable())
}
bb49 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: bool,mut _2: isize,mut _3: [usize; 7],mut _4: bool,mut _5: bool,mut _6: isize,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: [char; 5],mut _11: bool,mut _12: bool,mut _13: bool,mut _14: bool,mut _15: [u8; 7],mut _16: bool) -> f32 {
mir! {
type RET = f32;
let _17: f64;
let _18: [u8; 4];
let _19: Adt55;
let _20: f32;
let _21: u32;
let _22: i64;
let _23: [i32; 6];
let _24: char;
let _25: Adt49;
let _26: [i32; 8];
let _27: [char; 5];
let _28: isize;
let _29: i64;
let _30: i16;
let _31: Adt55;
let _32: isize;
let _33: u128;
let _34: isize;
let _35: *mut i16;
let _36: *mut bool;
let _37: Adt53;
let _38: i32;
let _39: Adt56;
let _40: f64;
let _41: (f32, u8, i32, u8);
let _42: u64;
let _43: i16;
let _44: Adt44;
let _45: [u8; 4];
let _46: [i32; 8];
let _47: i32;
let _48: Adt44;
let _49: u16;
let _50: Adt49;
let _51: ();
let _52: ();
{
RET = _2 as f32;
RET = 16010801994277174514_u64 as f32;
_4 = !_16;
RET = 7156_u16 as f32;
_8 = _9;
_12 = _9 ^ _13;
_7 = !_4;
_13 = _4;
_3 = [5746067052691361824_usize,2_usize,17463442057766769084_usize,7_usize,4_usize,12267940601536868983_usize,13968587357659951357_usize];
_4 = !_11;
_8 = !_4;
_17 = 147938809015090261052373039618114470876_i128 as f64;
_2 = _6;
_4 = !_11;
_16 = _13 <= _11;
_7 = _1;
_21 = !94113696_u32;
_1 = !_8;
Goto(bb1)
}
bb1 = {
_10 = ['\u{ad048}','\u{8c621}','\u{2bb3d}','\u{64f27}','\u{ddade}'];
_25.fld1 = 15_u8 - 0_u8;
_12 = _1 != _16;
_21 = !3817990621_u32;
_25.fld1 = 84_u8 >> _6;
_25.fld2 = [_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1];
_25.fld0 = [4251788219294445304_usize,18101668436359192832_usize,4222297517819771830_usize];
_25.fld2 = [_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1];
_2 = _6;
_10 = ['\u{3974d}','\u{b6213}','\u{3d1b5}','\u{a53c8}','\u{10485f}'];
_1 = !_4;
_12 = _9 != _5;
_15 = [_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1];
RET = 18423923666962165310_u64 as f32;
_24 = '\u{b992f}';
_13 = _9;
_4 = _12 > _9;
RET = 14679554199411017874_u64 as f32;
_20 = RET * RET;
_6 = _20 as isize;
_3 = [2030176222736271102_usize,0_usize,3018190946473957869_usize,3_usize,0_usize,4_usize,14263274969259963860_usize];
_26 = [1881128595_i32,1091601806_i32,196728191_i32,(-489817353_i32),2007795345_i32,1946413132_i32,(-1320783109_i32),336909208_i32];
_9 = _14;
_22 = 7748316714258327993_i64;
Goto(bb2)
}
bb2 = {
_3 = [11736218320526723715_usize,4_usize,13815022152843410097_usize,1_usize,0_usize,4_usize,14473745740933543521_usize];
_1 = _13 != _9;
_25.fld2 = [_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1];
_18 = [_25.fld1,_25.fld1,_25.fld1,_25.fld1];
_20 = RET;
_33 = !10520697183369839759166864810454676442_u128;
_11 = !_12;
RET = _20 - _20;
_30 = 16887_i16;
_26 = [1010497981_i32,(-1684634564_i32),256125726_i32,7970258_i32,(-1673182745_i32),185725896_i32,1736856375_i32,541208339_i32];
_25.fld1 = _16 as u8;
_33 = !316671987687894842351971661167573752343_u128;
_25.fld2 = [_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1];
_10 = [_24,_24,_24,_24,_24];
_20 = RET;
_17 = _30 as f64;
_9 = _5 < _8;
_32 = _2;
_12 = _1 != _5;
_23 = [763913553_i32,933992888_i32,647071790_i32,525818493_i32,(-890893673_i32),2095361270_i32];
_11 = _14;
match _22 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
7748316714258327993 => bb9,
_ => bb8
}
}
bb3 = {
_10 = ['\u{ad048}','\u{8c621}','\u{2bb3d}','\u{64f27}','\u{ddade}'];
_25.fld1 = 15_u8 - 0_u8;
_12 = _1 != _16;
_21 = !3817990621_u32;
_25.fld1 = 84_u8 >> _6;
_25.fld2 = [_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1];
_25.fld0 = [4251788219294445304_usize,18101668436359192832_usize,4222297517819771830_usize];
_25.fld2 = [_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1];
_2 = _6;
_10 = ['\u{3974d}','\u{b6213}','\u{3d1b5}','\u{a53c8}','\u{10485f}'];
_1 = !_4;
_12 = _9 != _5;
_15 = [_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1];
RET = 18423923666962165310_u64 as f32;
_24 = '\u{b992f}';
_13 = _9;
_4 = _12 > _9;
RET = 14679554199411017874_u64 as f32;
_20 = RET * RET;
_6 = _20 as isize;
_3 = [2030176222736271102_usize,0_usize,3018190946473957869_usize,3_usize,0_usize,4_usize,14263274969259963860_usize];
_26 = [1881128595_i32,1091601806_i32,196728191_i32,(-489817353_i32),2007795345_i32,1946413132_i32,(-1320783109_i32),336909208_i32];
_9 = _14;
_22 = 7748316714258327993_i64;
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
RET = _20 - _20;
_7 = !_1;
_24 = '\u{a2027}';
_15 = _25.fld2;
_34 = _32 - _2;
_8 = !_9;
_33 = 86772822822607947758173936448940155991_u128;
match _22 {
7748316714258327993 => bb11,
_ => bb10
}
}
bb10 = {
_10 = ['\u{ad048}','\u{8c621}','\u{2bb3d}','\u{64f27}','\u{ddade}'];
_25.fld1 = 15_u8 - 0_u8;
_12 = _1 != _16;
_21 = !3817990621_u32;
_25.fld1 = 84_u8 >> _6;
_25.fld2 = [_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1];
_25.fld0 = [4251788219294445304_usize,18101668436359192832_usize,4222297517819771830_usize];
_25.fld2 = [_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1];
_2 = _6;
_10 = ['\u{3974d}','\u{b6213}','\u{3d1b5}','\u{a53c8}','\u{10485f}'];
_1 = !_4;
_12 = _9 != _5;
_15 = [_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1,_25.fld1];
RET = 18423923666962165310_u64 as f32;
_24 = '\u{b992f}';
_13 = _9;
_4 = _12 > _9;
RET = 14679554199411017874_u64 as f32;
_20 = RET * RET;
_6 = _20 as isize;
_3 = [2030176222736271102_usize,0_usize,3018190946473957869_usize,3_usize,0_usize,4_usize,14263274969259963860_usize];
_26 = [1881128595_i32,1091601806_i32,196728191_i32,(-489817353_i32),2007795345_i32,1946413132_i32,(-1320783109_i32),336909208_i32];
_9 = _14;
_22 = 7748316714258327993_i64;
Goto(bb2)
}
bb11 = {
_22 = (-1267202464604742240_i64);
_11 = !_8;
_33 = 39412533604415195255766752481308036594_u128 * 133074225603288580993835521566419222638_u128;
_2 = _34 >> _34;
_25.fld1 = 5_usize as u8;
_12 = _9 == _4;
_5 = !_12;
_14 = _12 & _13;
_32 = _22 as isize;
_27 = [_24,_24,_24,_24,_24];
_27 = [_24,_24,_24,_24,_24];
_21 = _22 as u32;
_1 = _13;
_1 = !_8;
_27 = [_24,_24,_24,_24,_24];
_11 = !_13;
_10 = [_24,_24,_24,_24,_24];
_14 = _11;
_30 = -31280_i16;
_35 = core::ptr::addr_of_mut!(_30);
_9 = _13 < _13;
_7 = _14 > _9;
_22 = 4928859063686071971_i64;
_36 = core::ptr::addr_of_mut!(_4);
_36 = core::ptr::addr_of_mut!(_12);
(*_35) = _21 as i16;
Goto(bb12)
}
bb12 = {
_23 = [(-1604989609_i32),(-1580362407_i32),(-244289012_i32),(-1290854821_i32),(-1453596993_i32),(-269689388_i32)];
(*_36) = _5;
_3 = [4_usize,6_usize,8884244204928994194_usize,3_usize,7_usize,7_usize,7_usize];
_26 = [1127517751_i32,(-1527940374_i32),(-418083243_i32),(-679389665_i32),(-1632142917_i32),(-1970027224_i32),(-1634265148_i32),1052146320_i32];
_5 = !_12;
_29 = _22;
RET = _20 * _20;
_41.2 = 945281290_i32;
_1 = _5;
_21 = _2 as u32;
_26 = [_41.2,_41.2,_41.2,_41.2,_41.2,_41.2,_41.2,_41.2];
_7 = _5;
Goto(bb13)
}
bb13 = {
_41 = (RET, _25.fld1, (-2061614319_i32), _25.fld1);
_4 = _11 != _16;
_41.2 = 31_i8 as i32;
RET = _20 + _41.0;
_34 = 9189089324558047353_u64 as isize;
_26 = [_41.2,_41.2,_41.2,_41.2,_41.2,_41.2,_41.2,_41.2];
_42 = 11281316666186881660_u64;
_24 = '\u{21ba1}';
_27 = [_24,_24,_24,_24,_24];
_23 = [_41.2,_41.2,_41.2,_41.2,_41.2,_41.2];
_41.2 = (-214968655_i32) << _6;
_42 = !1184821243279347245_u64;
Goto(bb14)
}
bb14 = {
_40 = _17 + _17;
_44 = Adt44 { fld0: 94_i8 };
_5 = _14 ^ _4;
_36 = core::ptr::addr_of_mut!((*_36));
_33 = 293583299587071203055758793658194732070_u128 >> _21;
_12 = _11 | _7;
_46 = [_41.2,_41.2,_41.2,_41.2,_41.2,_41.2,_41.2,_41.2];
_38 = _41.2;
_47 = _44.fld0 as i32;
_32 = _2;
_15 = _25.fld2;
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(16_usize, 30_usize, Move(_30), 7_usize, Move(_7), 12_usize, Move(_12), 29_usize, Move(_29)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(16_usize, 27_usize, Move(_27), 33_usize, Move(_33), 4_usize, Move(_4), 47_usize, Move(_47)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(16_usize, 1_usize, Move(_1), 38_usize, Move(_38), 13_usize, Move(_13), 42_usize, Move(_42)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_51 = dump_var(16_usize, 9_usize, Move(_9), 5_usize, Move(_5), 11_usize, Move(_11), 32_usize, Move(_32)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: isize,mut _2: isize,mut _3: *mut isize,mut _4: &'static isize,mut _5: *mut isize) -> Adt51 {
mir! {
type RET = Adt51;
let _6: f64;
let _7: Adt45;
let _8: char;
let _9: f32;
let _10: bool;
let _11: Adt52;
let _12: f32;
let _13: Adt41;
let _14: (f32, u8, i32, u8);
let _15: isize;
let _16: Adt42;
let _17: i8;
let _18: u8;
let _19: [usize; 7];
let _20: f64;
let _21: [i32; 8];
let _22: Adt48;
let _23: Adt41;
let _24: ();
let _25: ();
{
_1 = -_2;
_5 = _3;
(*_3) = _2;
(*_3) = 1408967347_i32 as isize;
_2 = _1;
_2 = -_1;
_4 = &(*_3);
_6 = 111040729607705892030212031337925088886_u128 as f64;
Goto(bb1)
}
bb1 = {
_2 = _1 << _1;
_5 = core::ptr::addr_of_mut!((*_4));
_4 = &(*_3);
(*_3) = _2;
_5 = _3;
(*_5) = 11645172962291165656_u64 as isize;
_3 = _5;
(*_3) = _2;
_2 = '\u{8df0e}' as isize;
_4 = &_2;
Goto(bb2)
}
bb2 = {
_6 = 147684090920781420266330764034107141468_i128 as f64;
_3 = core::ptr::addr_of_mut!(_2);
Goto(bb3)
}
bb3 = {
_6 = 4048413275_u32 as f64;
_4 = &(*_5);
_1 = (*_5);
_5 = core::ptr::addr_of_mut!((*_4));
_8 = '\u{f9538}';
_3 = core::ptr::addr_of_mut!((*_3));
_6 = (-35754564102427233807778225692693458700_i128) as f64;
(*_3) = -_1;
_4 = &(*_4);
_3 = core::ptr::addr_of_mut!(_2);
Goto(bb4)
}
bb4 = {
_10 = true;
(*_3) = 1873921637409331747_i64 as isize;
_5 = core::ptr::addr_of_mut!((*_4));
_1 = (-4152_i16) as isize;
_9 = (*_5) as f32;
_10 = !true;
Goto(bb5)
}
bb5 = {
_2 = _6 as isize;
_2 = (*_5) * (*_4);
(*_3) = (-85608561128445587366600746619245766408_i128) as isize;
_4 = &(*_5);
_5 = _3;
_13.fld0 = -(-5932222966668273739_i64);
(*_3) = 12513_i16 as isize;
_1 = (*_4);
_13.fld1 = [(-3838181519965213888774625404374399535_i128),(-29322980389469937529744493854659775886_i128),(-164604063873018561902371632258242305714_i128),167405429599467249395143241697394866531_i128,(-147947244682953675486282627029638932530_i128)];
_12 = (-116_i8) as f32;
_8 = '\u{3b02c}';
(*_3) = (*_4) & (*_4);
Goto(bb6)
}
bb6 = {
_14.2 = 852424578_i32 - 521191617_i32;
(*_5) = (*_4);
_3 = core::ptr::addr_of_mut!((*_5));
_2 = (*_4) - _1;
_4 = &(*_4);
(*_3) = 3196211325653237419_u64 as isize;
_13.fld1 = [(-131626005696390105010670625872367863314_i128),(-138560557739340781515170787876358290250_i128),25067878205507884360643192027633555735_i128,110702857154560825409608542140489939543_i128,(-20252285697645176169056158462813955349_i128)];
_13.fld0 = (-7497299482925442132_i64) & 7454572682251753840_i64;
(*_5) = (*_4) + (*_4);
(*_5) = 190760382_u32 as isize;
_14.1 = !223_u8;
_14.2 = _9 as i32;
_10 = !true;
_8 = '\u{104767}';
_14 = (_9, 62_u8, 1749790007_i32, 18_u8);
Goto(bb7)
}
bb7 = {
_9 = 2_usize as f32;
_13.fld1 = [24506889703574539168892977365870448048_i128,137431890161403680254759244063187193932_i128,(-147934361120971016733061871100200815936_i128),47662314917975666620964042015159858638_i128,133003763833975659243983433036227843258_i128];
_13.fld1 = [(-6332312549238191735240122672013115023_i128),(-86383334899490022746344298376024566517_i128),(-82223470211306905634261541535066100576_i128),(-33783324823687042904613358193495933819_i128),(-10512597063660658058674614123263705442_i128)];
(*_3) = (*_4);
_4 = &(*_4);
_3 = core::ptr::addr_of_mut!(_15);
(*_3) = (*_5);
(*_5) = _15;
_14.1 = !_14.3;
_6 = (-62_i8) as f64;
_13.fld0 = 58941_u16 as i64;
_17 = !(-33_i8);
(*_5) = (*_4) & (*_3);
_15 = 41178_u16 as isize;
_9 = _14.0 + _14.0;
_14.2 = (-1551168324_i32);
match _14.3 {
18 => bb8,
_ => bb4
}
}
bb8 = {
_14.1 = !_14.3;
_14.2 = -(-1652330567_i32);
_14.1 = !_14.3;
_13.fld1 = [77439192662629328863322193732138614339_i128,(-154455725303546296656872711275511160731_i128),97104238370399975735766240903332602367_i128,(-20737117680452480020421744090558271014_i128),65061805913729442461348200987227474083_i128];
_1 = !_2;
_14.3 = (-3793_i16) as u8;
_14 = (_9, 24_u8, 1612494241_i32, 73_u8);
_13.fld1 = [21660060321938845360325089847659079448_i128,49845065572210874992945083697636250574_i128,88308963895116293735380604457957486688_i128,(-132994832775895792801208370941808997139_i128),74916079080271795961457032662394604540_i128];
_12 = _14.0 * _9;
_17 = 250205665055743057707651428841141950293_u128 as i8;
_5 = _3;
match _14.1 {
24 => bb10,
_ => bb9
}
}
bb9 = {
_10 = true;
(*_3) = 1873921637409331747_i64 as isize;
_5 = core::ptr::addr_of_mut!((*_4));
_1 = (-4152_i16) as isize;
_9 = (*_5) as f32;
_10 = !true;
Goto(bb5)
}
bb10 = {
_10 = (*_4) == (*_4);
_14.3 = !_14.1;
_18 = _14.1;
_1 = -(*_4);
_9 = _17 as f32;
(*_5) = (*_4) ^ _1;
_10 = _15 < (*_5);
(*_5) = -(*_4);
_4 = &(*_3);
(*_5) = -_1;
_14.0 = _12 * _12;
(*_5) = !_2;
Goto(bb11)
}
bb11 = {
_19 = [4_usize,2_usize,7128260918150593260_usize,149210546715850598_usize,2_usize,3_usize,0_usize];
_8 = '\u{1ed3e}';
_10 = false;
_8 = '\u{a5f05}';
_18 = _17 as u8;
_9 = -_14.0;
_12 = _6 as f32;
_12 = _14.3 as f32;
RET = Adt51::Variant2 { fld0: _6,fld1: _8,fld2: _14 };
place!(Field::<(f32, u8, i32, u8)>(Variant(RET, 2), 2)).2 = _14.2;
_14.3 = !Field::<(f32, u8, i32, u8)>(Variant(RET, 2), 2).3;
_13.fld0 = -(-7480954490332545784_i64);
_22.fld1 = _14.0;
_14.1 = Field::<(f32, u8, i32, u8)>(Variant(RET, 2), 2).1;
_18 = _14.3;
(*_5) = -_2;
place!(Field::<(f32, u8, i32, u8)>(Variant(RET, 2), 2)).3 = Field::<(f32, u8, i32, u8)>(Variant(RET, 2), 2).1;
place!(Field::<f64>(Variant(RET, 2), 0)) = _6;
place!(Field::<(f32, u8, i32, u8)>(Variant(RET, 2), 2)).1 = _18 >> (*_5);
_17 = 110_i8;
_6 = Field::<f64>(Variant(RET, 2), 0) - Field::<f64>(Variant(RET, 2), 0);
Goto(bb12)
}
bb12 = {
Call(_24 = dump_var(17_usize, 10_usize, Move(_10), 8_usize, Move(_8), 18_usize, Move(_18), 1_usize, Move(_1)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{ebb7e}'), std::hint::black_box(6_isize), std::hint::black_box((-78_i8)), std::hint::black_box((-9993_i16)), std::hint::black_box(1038847157_i32), std::hint::black_box((-8568959269228263946_i64)), std::hint::black_box((-90057179701809140879919197192777072783_i128)), std::hint::black_box(1_usize), std::hint::black_box(76_u8), std::hint::black_box(54203_u16), std::hint::black_box(371012663_u32), std::hint::black_box(14743217809979520717_u64), std::hint::black_box(338301736192250003563406522196690898694_u128));
                
            }
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf("Adt40::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: *const usize,
fld1: char,
fld2: isize,
fld3: u64,
fld4: *mut i16,
fld5: [i32; 6],
fld6: *mut bool,

},
Variant1{
fld0: bool,
fld1: (bool, (*mut char, [usize; 7]), i64, i8, i16, i32, [u8; 4]),
fld2: *mut char,
fld3: i8,
fld4: [u16; 7],
fld5: f32,
fld6: *mut isize,
fld7: u16,

},
Variant2{
fld0: usize,
fld1: u32,
fld2: *mut isize,
fld3: [i128; 5],

},
Variant3{
fld0: u8,
fld1: (f32, u8, i32, u8),
fld2: *const usize,

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt41{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt41 {
fld0: i64,
fld1: [i128; 5],
}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: *mut *mut isize,
fld1: (*mut char, [usize; 7]),
fld2: *mut bool,
fld3: [u16; 7],
fld4: u32,

},
Variant1{
fld0: *mut char,
fld1: u128,
fld2: f32,
fld3: [usize; 3],
fld4: *mut i16,
fld5: Adt40,
fld6: (f32, u8, i32, u8),

},
Variant2{
fld0: bool,
fld1: *const usize,
fld2: usize,
fld3: i32,

},
Variant3{
fld0: Adt40,
fld1: *const usize,
fld2: i8,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: [u16; 7],
fld1: i64,

},
Variant1{
fld0: [char; 5],
fld1: [u8; 7],
fld2: (bool, (*mut char, [usize; 7]), i64, i8, i16, i32, [u8; 4]),
fld3: *const usize,
fld4: f32,

},
Variant2{
fld0: u128,
fld1: u16,
fld2: f32,
fld3: *mut isize,
fld4: *mut bool,

},
Variant3{
fld0: bool,
fld1: [i128; 5],
fld2: u8,
fld3: Adt40,
fld4: [u8; 7],
fld5: u128,
fld6: [usize; 7],
fld7: Adt42,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: i8,
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: bool,
fld1: Adt44,
fld2: u128,
fld3: (*mut char, [usize; 7]),
fld4: usize,
fld5: i32,
fld6: *mut char,

},
Variant1{
fld0: u64,
fld1: [i32; 8],
fld2: f32,
fld3: *mut *mut isize,
fld4: Adt42,
fld5: Adt43,
fld6: i64,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: *mut char,

},
Variant1{
fld0: i16,
fld1: f64,

},
Variant2{
fld0: *mut char,
fld1: [u16; 7],
fld2: u32,
fld3: [char; 5],
fld4: f32,
fld5: usize,
fld6: *const usize,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: [u8; 4],
fld1: usize,
fld2: i128,
fld3: *mut bool,
fld4: i16,

},
Variant1{
fld0: [u16; 7],
fld1: char,
fld2: Adt44,
fld3: [char; 5],

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: [usize; 7],
fld1: f32,
fld2: [char; 5],
}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: [usize; 3],
fld1: u8,
fld2: [u8; 7],
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: i16,
fld1: [u8; 7],
fld2: [i128; 5],

},
Variant1{
fld0: (*mut char, [usize; 7]),
fld1: u32,
fld2: [i32; 6],
fld3: Adt41,

},
Variant2{
fld0: u128,
fld1: *const (bool, (*mut char, [usize; 7]), i64, i8, i16, i32, [u8; 4]),
fld2: (bool, (*mut char, [usize; 7]), i64, i8, i16, i32, [u8; 4]),
fld3: [i32; 8],

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: *mut i16,
fld1: Adt43,

},
Variant1{
fld0: Adt41,
fld1: [i128; 5],
fld2: isize,
fld3: Adt44,
fld4: *const usize,

},
Variant2{
fld0: f64,
fld1: char,
fld2: (f32, u8, i32, u8),

},
Variant3{
fld0: i32,
fld1: *const usize,
fld2: isize,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: f64,
fld1: char,
fld2: [usize; 3],
fld3: f32,
fld4: *mut isize,
fld5: (f32, u8, i32, u8),
fld6: *mut bool,

},
Variant1{
fld0: u16,
fld1: Adt47,
fld2: f64,
fld3: [u16; 7],
fld4: Adt40,
fld5: i32,
fld6: [i32; 8],
fld7: Adt50,

},
Variant2{
fld0: [u8; 4],
fld1: f64,
fld2: Adt48,
fld3: *const (bool, (*mut char, [usize; 7]), i64, i8, i16, i32, [u8; 4]),
fld4: [u8; 7],
fld5: (f32, u8, i32, u8),
fld6: Adt46,

},
Variant3{
fld0: Adt44,
fld1: [i32; 6],
fld2: isize,
fld3: Adt51,
fld4: i16,
fld5: Adt47,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: [usize; 7],

},
Variant1{
fld0: *mut isize,
fld1: [char; 5],
fld2: *mut char,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: bool,
fld1: u16,
fld2: (f64, (*mut char, [usize; 7]), i8, char),
fld3: [i32; 8],
fld4: *mut isize,

},
Variant1{
fld0: Adt52,
fld1: u128,
fld2: isize,
fld3: (f64, (*mut char, [usize; 7]), i8, char),
fld4: [u8; 4],
fld5: *mut *mut isize,
fld6: Adt46,
fld7: u8,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: bool,
fld1: char,
fld2: *mut *mut isize,
fld3: Adt53,
fld4: i16,
fld5: (f32, u8, i32, u8),
fld6: [i128; 5],
fld7: Adt51,

},
Variant1{
fld0: Adt54,
fld1: i64,
fld2: (*mut char, [usize; 7]),
fld3: u64,
fld4: u16,

},
Variant2{
fld0: Adt49,
fld1: [usize; 3],
fld2: Adt40,

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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: bool,
fld1: Adt51,
fld2: Adt53,

},
Variant1{
fld0: usize,
fld1: Adt52,
fld2: [u16; 7],
fld3: u32,
fld4: [u8; 7],
fld5: *mut i16,

},
Variant2{
fld0: *const usize,

},
Variant3{
fld0: (*mut char, [usize; 7]),
fld1: i8,

}}

