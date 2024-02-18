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
pub fn fn0(mut _1: u8,mut _2: char,mut _3: usize,mut _4: u64,mut _5: u16) -> [u8; 2] {
mir! {
type RET = [u8; 2];
let _6: *const ([i8; 7], [i32; 2]);
let _7: f32;
let _8: f32;
let _9: char;
let _10: [bool; 1];
let _11: *mut [i8; 7];
let _12: char;
let _13: (&'static i8,);
let _14: (u64, [u64; 7], (u64, u64, u64, i64));
let _15: Adt24;
let _16: [i64; 1];
let _17: [i8; 7];
let _18: [usize; 3];
let _19: [u16; 3];
let _20: (f32, &'static i64, [u8; 7], Adt38);
let _21: (u64, [u64; 7], (u64, u64, u64, i64));
let _22: isize;
let _23: ([i32; 2], u32);
let _24: u16;
let _25: isize;
let _26: u8;
let _27: i32;
let _28: i64;
let _29: ();
let _30: ();
{
RET = [224_u8,167_u8];
_2 = '\u{97656}';
_5 = 52171_u16;
_1 = 85_u8 + 165_u8;
RET = [_1,_1];
_5 = 15443_u16 << _1;
_1 = 223_u8 | 192_u8;
_4 = !6412042713382546384_u64;
_3 = !3609248213428040350_usize;
RET = [_1,_1];
RET = [_1,_1];
_3 = 6510583195359047882_usize;
_2 = '\u{777a6}';
RET = [_1,_1];
_1 = !15_u8;
_3 = 10759336549066142154_usize >> _1;
_2 = '\u{fb117}';
_7 = (-9223372036854775808_isize) as f32;
_5 = !27337_u16;
RET = [_1,_1];
_2 = '\u{e6bcb}';
_7 = 35_i8 as f32;
_8 = _7;
Call(RET = fn1(_8, _3, _2, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = 65_u8;
_3 = !4_usize;
_2 = '\u{acf92}';
Call(_7 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = _7;
_4 = 7908624192122827507_u64;
_7 = _8 + _8;
_2 = '\u{8d258}';
_7 = _8;
_4 = 3732802230689540526_u64;
_4 = 8061386262151560608_u64;
RET = [_1,_1];
_2 = '\u{999bc}';
_2 = '\u{cf6b8}';
_3 = !12420041084455647467_usize;
_1 = false as u8;
RET = [_1,_1];
_14.2.0 = _4;
_14.2.1 = _14.2.0;
_4 = true as u64;
_14.2.2 = 1114088525_i32 as u64;
_14.2.2 = _14.2.1;
_4 = _14.2.2;
_14.2 = (_4, _4, _4, 5688511511413265774_i64);
_14.1 = [_14.2.2,_4,_14.2.2,_14.2.1,_4,_14.2.0,_14.2.2];
_14.2 = (_4, _4, _4, 5903473688668000416_i64);
_2 = '\u{c6508}';
_1 = !175_u8;
Goto(bb3)
}
bb3 = {
_14.2.1 = !_14.2.2;
_10 = [false];
_9 = _2;
_10 = [true];
_2 = _9;
_16 = [_14.2.3];
_12 = _9;
_3 = (-92_i8) as usize;
_3 = !11678539148595669300_usize;
_14.2.3 = 6028945394075469183_i64;
RET = [_1,_1];
_16 = [_14.2.3];
_15 = Adt24::Variant0 { fld0: _1,fld1: _12,fld2: (-13126_i16) };
place!(Field::<u8>(Variant(_15, 0), 0)) = _1 << _14.2.3;
_14.2.0 = !_14.2.2;
_15 = Adt24::Variant1 { fld0: _5,fld1: _14.2.1 };
place!(Field::<u16>(Variant(_15, 1), 0)) = _5;
_16 = [_14.2.3];
_14.0 = _14.2.2;
_16 = [_14.2.3];
_3 = _1 as usize;
_3 = 18364163941646002894_usize << _5;
_2 = _9;
_16 = [_14.2.3];
_2 = _9;
_14.2.1 = 3021492802_u32 as u64;
Goto(bb4)
}
bb4 = {
_7 = _8 + _8;
_1 = !218_u8;
_15 = Adt24::Variant0 { fld0: _1,fld1: _12,fld2: (-17983_i16) };
_14.1 = [_4,_14.2.2,_14.2.2,_14.0,_14.0,_4,_14.0];
_14.2.1 = _14.2.0 % _4;
_14.0 = !_14.2.1;
_10 = [true];
_14.2 = (_4, _14.0, _14.0, 5446338518052661153_i64);
place!(Field::<char>(Variant(_15, 0), 1)) = _12;
_4 = _14.2.2;
_5 = 18718_i16 as u16;
_5 = !63041_u16;
_17 = [(-80_i8),(-27_i8),(-63_i8),(-49_i8),38_i8,(-60_i8),(-20_i8)];
_8 = _7 * _7;
_18 = [_3,_3,_3];
place!(Field::<char>(Variant(_15, 0), 1)) = _12;
_14.2 = (_14.0, _14.0, _14.0, (-2823834110760856290_i64));
_12 = _2;
_14.2.0 = !_14.0;
_20.3.fld0 = 83_i8 as f32;
_2 = _9;
_21.2.1 = _14.2.0;
_21.2 = (_14.2.2, _4, _14.0, _14.2.3);
_20.3.fld5.0 = [1640563093_i32,540959640_i32];
_20.3.fld3 = _14.2.2 as i8;
Goto(bb5)
}
bb5 = {
_14.2.0 = _14.2.2;
_21.1 = _14.1;
_21.1 = [_21.2.1,_21.2.0,_21.2.0,_4,_4,_21.2.0,_14.2.0];
_11 = core::ptr::addr_of_mut!(_17);
_20.3.fld4 = (-9223372036854775808_isize) as i16;
_9 = _12;
_21.2.0 = _21.2.2;
_20.1 = &_14.2.3;
_20.3.fld1 = Field::<char>(Variant(_15, 0), 1);
place!(Field::<u8>(Variant(_15, 0), 0)) = _1 - _1;
_14.2.1 = _14.0;
RET = [_1,Field::<u8>(Variant(_15, 0), 0)];
place!(Field::<i16>(Variant(_15, 0), 2)) = _20.3.fld4 ^ _20.3.fld4;
_20.3.fld4 = Field::<i16>(Variant(_15, 0), 2) >> _14.2.1;
(*_11) = [_20.3.fld3,_20.3.fld3,_20.3.fld3,_20.3.fld3,_20.3.fld3,_20.3.fld3,_20.3.fld3];
_20.3.fld3 = 23_i8 ^ 35_i8;
_21 = _14;
match _14.2.3 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
340282366920938463460550773321007355166 => bb13,
_ => bb12
}
}
bb6 = {
_7 = _8 + _8;
_1 = !218_u8;
_15 = Adt24::Variant0 { fld0: _1,fld1: _12,fld2: (-17983_i16) };
_14.1 = [_4,_14.2.2,_14.2.2,_14.0,_14.0,_4,_14.0];
_14.2.1 = _14.2.0 % _4;
_14.0 = !_14.2.1;
_10 = [true];
_14.2 = (_4, _14.0, _14.0, 5446338518052661153_i64);
place!(Field::<char>(Variant(_15, 0), 1)) = _12;
_4 = _14.2.2;
_5 = 18718_i16 as u16;
_5 = !63041_u16;
_17 = [(-80_i8),(-27_i8),(-63_i8),(-49_i8),38_i8,(-60_i8),(-20_i8)];
_8 = _7 * _7;
_18 = [_3,_3,_3];
place!(Field::<char>(Variant(_15, 0), 1)) = _12;
_14.2 = (_14.0, _14.0, _14.0, (-2823834110760856290_i64));
_12 = _2;
_14.2.0 = !_14.0;
_20.3.fld0 = 83_i8 as f32;
_2 = _9;
_21.2.1 = _14.2.0;
_21.2 = (_14.2.2, _4, _14.0, _14.2.3);
_20.3.fld5.0 = [1640563093_i32,540959640_i32];
_20.3.fld3 = _14.2.2 as i8;
Goto(bb5)
}
bb7 = {
_14.2.1 = !_14.2.2;
_10 = [false];
_9 = _2;
_10 = [true];
_2 = _9;
_16 = [_14.2.3];
_12 = _9;
_3 = (-92_i8) as usize;
_3 = !11678539148595669300_usize;
_14.2.3 = 6028945394075469183_i64;
RET = [_1,_1];
_16 = [_14.2.3];
_15 = Adt24::Variant0 { fld0: _1,fld1: _12,fld2: (-13126_i16) };
place!(Field::<u8>(Variant(_15, 0), 0)) = _1 << _14.2.3;
_14.2.0 = !_14.2.2;
_15 = Adt24::Variant1 { fld0: _5,fld1: _14.2.1 };
place!(Field::<u16>(Variant(_15, 1), 0)) = _5;
_16 = [_14.2.3];
_14.0 = _14.2.2;
_16 = [_14.2.3];
_3 = _1 as usize;
_3 = 18364163941646002894_usize << _5;
_2 = _9;
_16 = [_14.2.3];
_2 = _9;
_14.2.1 = 3021492802_u32 as u64;
Goto(bb4)
}
bb8 = {
_8 = _7;
_4 = 7908624192122827507_u64;
_7 = _8 + _8;
_2 = '\u{8d258}';
_7 = _8;
_4 = 3732802230689540526_u64;
_4 = 8061386262151560608_u64;
RET = [_1,_1];
_2 = '\u{999bc}';
_2 = '\u{cf6b8}';
_3 = !12420041084455647467_usize;
_1 = false as u8;
RET = [_1,_1];
_14.2.0 = _4;
_14.2.1 = _14.2.0;
_4 = true as u64;
_14.2.2 = 1114088525_i32 as u64;
_14.2.2 = _14.2.1;
_4 = _14.2.2;
_14.2 = (_4, _4, _4, 5688511511413265774_i64);
_14.1 = [_14.2.2,_4,_14.2.2,_14.2.1,_4,_14.2.0,_14.2.2];
_14.2 = (_4, _4, _4, 5903473688668000416_i64);
_2 = '\u{c6508}';
_1 = !175_u8;
Goto(bb3)
}
bb9 = {
_1 = 65_u8;
_3 = !4_usize;
_2 = '\u{acf92}';
Call(_7 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
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
SetDiscriminant(_15, 2);
_13.0 = &_20.3.fld3;
place!(Field::<u64>(Variant(_15, 2), 0)) = _14.2.1 * _14.2.2;
_20.3.fld5.1 = 43404936_u32;
RET = [_1,_1];
_20.3.fld1 = _9;
place!(Field::<usize>(Variant(_15, 2), 4)) = _3;
_14.0 = _21.0 >> _21.2.0;
_23.1 = _20.3.fld5.1;
(*_11) = [_20.3.fld3,_20.3.fld3,_20.3.fld3,_20.3.fld3,_20.3.fld3,_20.3.fld3,_20.3.fld3];
_14.2.3 = _21.2.3 - _21.2.3;
match _20.3.fld5.1 {
0 => bb1,
1 => bb2,
2 => bb12,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
43404936 => bb19,
_ => bb18
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_14.2.1 = !_14.2.2;
_10 = [false];
_9 = _2;
_10 = [true];
_2 = _9;
_16 = [_14.2.3];
_12 = _9;
_3 = (-92_i8) as usize;
_3 = !11678539148595669300_usize;
_14.2.3 = 6028945394075469183_i64;
RET = [_1,_1];
_16 = [_14.2.3];
_15 = Adt24::Variant0 { fld0: _1,fld1: _12,fld2: (-13126_i16) };
place!(Field::<u8>(Variant(_15, 0), 0)) = _1 << _14.2.3;
_14.2.0 = !_14.2.2;
_15 = Adt24::Variant1 { fld0: _5,fld1: _14.2.1 };
place!(Field::<u16>(Variant(_15, 1), 0)) = _5;
_16 = [_14.2.3];
_14.0 = _14.2.2;
_16 = [_14.2.3];
_3 = _1 as usize;
_3 = 18364163941646002894_usize << _5;
_2 = _9;
_16 = [_14.2.3];
_2 = _9;
_14.2.1 = 3021492802_u32 as u64;
Goto(bb4)
}
bb17 = {
_1 = 65_u8;
_3 = !4_usize;
_2 = '\u{acf92}';
Call(_7 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
_8 = _7;
_4 = 7908624192122827507_u64;
_7 = _8 + _8;
_2 = '\u{8d258}';
_7 = _8;
_4 = 3732802230689540526_u64;
_4 = 8061386262151560608_u64;
RET = [_1,_1];
_2 = '\u{999bc}';
_2 = '\u{cf6b8}';
_3 = !12420041084455647467_usize;
_1 = false as u8;
RET = [_1,_1];
_14.2.0 = _4;
_14.2.1 = _14.2.0;
_4 = true as u64;
_14.2.2 = 1114088525_i32 as u64;
_14.2.2 = _14.2.1;
_4 = _14.2.2;
_14.2 = (_4, _4, _4, 5688511511413265774_i64);
_14.1 = [_14.2.2,_4,_14.2.2,_14.2.1,_4,_14.2.0,_14.2.2];
_14.2 = (_4, _4, _4, 5903473688668000416_i64);
_2 = '\u{c6508}';
_1 = !175_u8;
Goto(bb3)
}
bb19 = {
_10 = [false];
_22 = -9223372036854775807_isize;
Goto(bb20)
}
bb20 = {
Call(_29 = dump_var(0_usize, 4_usize, Move(_4), 22_usize, Move(_22), 1_usize, Move(_1), 17_usize, Move(_17)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_29 = dump_var(0_usize, 2_usize, Move(_2), 14_usize, Move(_14), 21_usize, Move(_21), 30_usize, _30), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: f32,mut _2: usize,mut _3: char,mut _4: char,mut _5: char) -> [u8; 2] {
mir! {
type RET = [u8; 2];
let _6: f64;
let _7: isize;
let _8: (&'static char,);
let _9: i128;
let _10: ([i32; 2], u32);
let _11: u8;
let _12: [i16; 8];
let _13: [bool; 1];
let _14: (f32, &'static i64, [u8; 7], Adt38);
let _15: (*const u64,);
let _16: f32;
let _17: char;
let _18: &'static char;
let _19: [usize; 3];
let _20: f64;
let _21: ();
let _22: ();
{
RET = [198_u8,80_u8];
RET = [115_u8,176_u8];
RET = [36_u8,202_u8];
_6 = 11812513991581000285_u64 as f64;
_2 = (-8172276370567572573_i64) as usize;
_7 = -9223372036854775807_isize;
_6 = (-13_i8) as f64;
RET = [62_u8,111_u8];
_5 = _4;
_1 = _7 as f32;
_8.0 = &_4;
_1 = 155_u8 as f32;
_2 = !5_usize;
_9 = 5553552336156153563110481076156167836_i128;
match _9 {
5553552336156153563110481076156167836 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_1 = (-26_i8) as f32;
_6 = 28110_u16 as f64;
_7 = (-9223372036854775808_isize) * 99_isize;
_3 = _5;
_10.0 = [(-1202007350_i32),(-326947724_i32)];
_2 = 1_usize ^ 17695175970788916975_usize;
_8.0 = &_5;
_9 = 72834254209953774506109069155185066722_i128 & (-55980835305670913063226434057564608601_i128);
_10.1 = _4 as u32;
_5 = _3;
_6 = _1 as f64;
_3 = _4;
_8.0 = &_4;
Goto(bb3)
}
bb3 = {
_2 = 170_u8 as usize;
RET = [195_u8,2_u8];
_3 = _4;
RET = [6_u8,246_u8];
_6 = _9 as f64;
_5 = _3;
Goto(bb4)
}
bb4 = {
_10.1 = 1460523182_u32 + 1820455913_u32;
_12 = [(-24391_i16),(-2052_i16),28430_i16,(-32694_i16),(-11611_i16),(-31241_i16),(-13618_i16),(-31424_i16)];
_10.1 = 3973202065_u32;
_9 = (-6883402700396736610107142989258359216_i128) >> _7;
_12 = [3449_i16,(-10009_i16),(-28580_i16),7273_i16,9746_i16,(-21492_i16),(-32454_i16),24401_i16];
_9 = _4 as i128;
_10.1 = 1533631875_u32;
_7 = (-89_isize);
_7 = 39_isize;
_14.3.fld1 = _5;
Goto(bb5)
}
bb5 = {
_11 = 115_u8 ^ 204_u8;
_2 = 6_usize;
_9 = 99394327253627661010225464690333588557_i128;
_14.3.fld5 = (_10.0, _10.1);
_14.3.fld5 = _10;
_1 = _11 as f32;
_4 = _14.3.fld1;
_12[_2] = _7 as i16;
_13 = [false];
_16 = -_1;
_17 = _5;
_16 = _1 - _1;
_14.2[_2] = _7 as u8;
_14.3.fld0 = _16;
_1 = _16;
_13 = [false];
_14.3.fld5 = (_10.0, _10.1);
_14.3.fld2 = _9;
Goto(bb6)
}
bb6 = {
_8.0 = &_14.3.fld1;
match _7 {
0 => bb4,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
39 => bb12,
_ => bb11
}
}
bb7 = {
_11 = 115_u8 ^ 204_u8;
_2 = 6_usize;
_9 = 99394327253627661010225464690333588557_i128;
_14.3.fld5 = (_10.0, _10.1);
_14.3.fld5 = _10;
_1 = _11 as f32;
_4 = _14.3.fld1;
_12[_2] = _7 as i16;
_13 = [false];
_16 = -_1;
_17 = _5;
_16 = _1 - _1;
_14.2[_2] = _7 as u8;
_14.3.fld0 = _16;
_1 = _16;
_13 = [false];
_14.3.fld5 = (_10.0, _10.1);
_14.3.fld2 = _9;
Goto(bb6)
}
bb8 = {
_10.1 = 1460523182_u32 + 1820455913_u32;
_12 = [(-24391_i16),(-2052_i16),28430_i16,(-32694_i16),(-11611_i16),(-31241_i16),(-13618_i16),(-31424_i16)];
_10.1 = 3973202065_u32;
_9 = (-6883402700396736610107142989258359216_i128) >> _7;
_12 = [3449_i16,(-10009_i16),(-28580_i16),7273_i16,9746_i16,(-21492_i16),(-32454_i16),24401_i16];
_9 = _4 as i128;
_10.1 = 1533631875_u32;
_7 = (-89_isize);
_7 = 39_isize;
_14.3.fld1 = _5;
Goto(bb5)
}
bb9 = {
_2 = 170_u8 as usize;
RET = [195_u8,2_u8];
_3 = _4;
RET = [6_u8,246_u8];
_6 = _9 as f64;
_5 = _3;
Goto(bb4)
}
bb10 = {
_1 = (-26_i8) as f32;
_6 = 28110_u16 as f64;
_7 = (-9223372036854775808_isize) * 99_isize;
_3 = _5;
_10.0 = [(-1202007350_i32),(-326947724_i32)];
_2 = 1_usize ^ 17695175970788916975_usize;
_8.0 = &_5;
_9 = 72834254209953774506109069155185066722_i128 & (-55980835305670913063226434057564608601_i128);
_10.1 = _4 as u32;
_5 = _3;
_6 = _1 as f64;
_3 = _4;
_8.0 = &_4;
Goto(bb3)
}
bb11 = {
Return()
}
bb12 = {
RET = [_14.2[_2],_11];
_12 = [(-27146_i16),29518_i16,(-20900_i16),21277_i16,(-31447_i16),19492_i16,(-28345_i16),17890_i16];
_2 = 7_usize ^ 6_usize;
_16 = _1 - _14.3.fld0;
_14.0 = _1;
_10 = (_14.3.fld5.0, _14.3.fld5.1);
_7 = 9223372036854775807_isize;
_14.3 = Adt38 { fld0: _1,fld1: _3,fld2: _9,fld3: (-108_i8),fld4: 2511_i16,fld5: _10,fld6: (-7439926039779425715_i64) };
_6 = _10.1 as f64;
_14.0 = _14.3.fld0;
_14.1 = &_14.3.fld6;
_10.0 = [(-1807911437_i32),540922395_i32];
_10.1 = _14.3.fld5.1;
_7 = (-9223372036854775808_isize);
_12 = [_14.3.fld4,_14.3.fld4,_14.3.fld4,_14.3.fld4,_14.3.fld4,_14.3.fld4,_14.3.fld4,_14.3.fld4];
_13 = [true];
_14.2 = [_11,_11,_11,_11,_11,_11,_11];
Call(_3 = fn2(Move(_14.1), _14.3.fld4, _14.3.fld1, _14.3.fld3, _6, _14.3.fld3, _14.3.fld3, _14.0, _14.3, _14.3.fld0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_13 = [true];
_14.3 = Adt38 { fld0: _14.0,fld1: _3,fld2: _9,fld3: 79_i8,fld4: 17710_i16,fld5: _10,fld6: 5697565034988840276_i64 };
_17 = _14.3.fld1;
_14.0 = _1;
Call(_14.3.fld2 = fn3(_14.3.fld4, RET, _7, _12, _16, _14.3.fld6, _6, _14.3.fld4), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_10 = (_14.3.fld5.0, _14.3.fld5.1);
_14.1 = &_14.3.fld6;
_1 = _16 * _16;
_14.3.fld5.0 = _10.0;
_19 = [_2,_2,_2];
_20 = -_6;
RET = [_11,_11];
_14.3.fld5.0 = [(-623735220_i32),(-1787396918_i32)];
_14.3.fld0 = _14.3.fld4 as f32;
_17 = _5;
_14.2 = [_11,_11,_11,_11,_11,_11,_11];
_10 = _14.3.fld5;
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(1_usize, 5_usize, Move(_5), 9_usize, Move(_9), 17_usize, Move(_17), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_21 = dump_var(1_usize, 10_usize, Move(_10), 13_usize, Move(_13), 22_usize, _22, 22_usize, _22), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: &'static i64,mut _2: i16,mut _3: char,mut _4: i8,mut _5: f64,mut _6: i8,mut _7: i8,mut _8: f32,mut _9: Adt38,mut _10: f32) -> char {
mir! {
type RET = char;
let _11: f64;
let _12: usize;
let _13: u128;
let _14: isize;
let _15: &'static u16;
let _16: (u64, u64, u64, i64);
let _17: (u64, [u64; 7], (u64, u64, u64, i64));
let _18: bool;
let _19: [u8; 7];
let _20: char;
let _21: [i64; 1];
let _22: &'static [i64; 8];
let _23: [u8; 7];
let _24: ();
let _25: ();
{
_4 = 278852296578533175562356798582004677900_u128 as i8;
_9.fld4 = _2 - _2;
_9.fld1 = _3;
_9.fld2 = (-16326557893661394733517454757377235601_i128) ^ 76834282886719783936135655948283207400_i128;
_9.fld5.0 = [1881310899_i32,1532628169_i32];
_6 = _9.fld3 ^ _7;
_6 = _9.fld3;
_9.fld5.1 = 3803822000_u32 * 230780628_u32;
RET = _9.fld1;
_9.fld6 = _9.fld4 as i64;
_9.fld3 = _7;
RET = _9.fld1;
_1 = &_9.fld6;
match _9.fld3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463463374607431768211348 => bb6,
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
_3 = _9.fld1;
_12 = !5_usize;
_16.0 = 17610243677317577293_u64;
_14 = true as isize;
_16.2 = _16.0;
_5 = _16.0 as f64;
_9.fld5.1 = !2885219583_u32;
_9.fld1 = _3;
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb7,
340282366920938463463374607431768211348 => bb9,
_ => bb8
}
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
_10 = _9.fld0;
_10 = _5 as f32;
_16.3 = (*_1) - (*_1);
_13 = 158007273974714800240948790857315161689_u128;
_17.1 = [_16.0,_16.2,_16.2,_16.2,_16.0,_16.2,_16.2];
match _7 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb10,
5 => bb11,
340282366920938463463374607431768211348 => bb13,
_ => bb12
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_3 = _9.fld1;
_12 = !5_usize;
_16.0 = 17610243677317577293_u64;
_14 = true as isize;
_16.2 = _16.0;
_5 = _16.0 as f64;
_9.fld5.1 = !2885219583_u32;
_9.fld1 = _3;
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb7,
340282366920938463463374607431768211348 => bb9,
_ => bb8
}
}
bb13 = {
_17.2 = (_16.0, _16.0, _16.2, (*_1));
_16.0 = _13 as u64;
_16 = (_17.2.1, _17.2.0, _17.2.0, _17.2.3);
_16.0 = _12 as u64;
RET = _9.fld1;
_9.fld3 = _6;
_11 = _17.2.2 as f64;
_17.0 = !_17.2.0;
_17.2.0 = !_17.2.2;
_16.0 = _16.1;
_9.fld2 = 101678044631874073172778418717034274058_i128 + (-126524816642832304178411718728226991428_i128);
_18 = false ^ false;
RET = _3;
_17.2.3 = -_9.fld6;
_9.fld0 = -_8;
RET = _3;
_5 = -_11;
_18 = _16.3 == _9.fld6;
_16.2 = _16.1;
_2 = -_9.fld4;
match _7 {
0 => bb1,
1 => bb6,
2 => bb12,
3 => bb7,
4 => bb14,
340282366920938463463374607431768211348 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_19 = [170_u8,227_u8,201_u8,62_u8,107_u8,77_u8,35_u8];
_7 = _17.2.0 as i8;
_23 = [251_u8,220_u8,111_u8,155_u8,224_u8,221_u8,95_u8];
_8 = _9.fld0 * _9.fld0;
_5 = _9.fld5.1 as f64;
_16.0 = _18 as u64;
_2 = _9.fld4;
_17.2.2 = _16.0;
_16.3 = _9.fld6;
_17.2.1 = _13 as u64;
_17.2.0 = _9.fld2 as u64;
_9.fld6 = _17.2.3;
_21 = [_17.2.3];
_9.fld0 = _8;
_21 = [_9.fld6];
_16.3 = _17.2.3;
_7 = _9.fld3 ^ _9.fld3;
_5 = _11;
_20 = _9.fld1;
_16.1 = _16.0 * _17.2.2;
_17.2 = (_16.0, _16.1, _16.1, _16.3);
Goto(bb17)
}
bb17 = {
Call(_24 = dump_var(2_usize, 18_usize, Move(_18), 19_usize, Move(_19), 6_usize, Move(_6), 20_usize, Move(_20)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_24 = dump_var(2_usize, 4_usize, Move(_4), 3_usize, Move(_3), 12_usize, Move(_12), 25_usize, _25), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: i16,mut _2: [u8; 2],mut _3: isize,mut _4: [i16; 8],mut _5: f32,mut _6: i64,mut _7: f64,mut _8: i16) -> i128 {
mir! {
type RET = i128;
let _9: [i64; 8];
let _10: [i8; 5];
let _11: ([i32; 2], u32);
let _12: (f32, &'static i64, [u8; 7], Adt38);
let _13: char;
let _14: u16;
let _15: *mut *mut Adt19;
let _16: &'static i64;
let _17: f32;
let _18: f32;
let _19: u32;
let _20: &'static [u8; 7];
let _21: isize;
let _22: [u32; 3];
let _23: Adt24;
let _24: *mut u128;
let _25: i8;
let _26: [usize; 3];
let _27: i64;
let _28: f32;
let _29: *mut Adt19;
let _30: usize;
let _31: &'static [u8; 2];
let _32: u32;
let _33: char;
let _34: char;
let _35: f64;
let _36: bool;
let _37: [u32; 3];
let _38: (&'static i8,);
let _39: f32;
let _40: [i32; 2];
let _41: i16;
let _42: *const [u8; 2];
let _43: f64;
let _44: ();
let _45: ();
{
RET = -(-138263392045372036314276904347118401620_i128);
RET = 111777826066273382673082500288725574854_i128 | 13954319518915449250783499099043533577_i128;
_5 = 17035160410163172507_usize as f32;
_2 = [180_u8,236_u8];
_6 = !4480895334825395669_i64;
_1 = _8 - _8;
RET = (-28_i8) as i128;
_7 = (-120_i8) as f64;
_6 = 7818120629607836531_i64 - 1687065599318716012_i64;
_6 = (-2437327723397428247_i64);
_2 = [195_u8,112_u8];
_2 = [101_u8,191_u8];
RET = (-79247412747828486426445668682428254990_i128) ^ 101116180717020359403220647332683758400_i128;
_12.3.fld1 = '\u{e1b1a}';
_11.1 = 3885800099_u32;
_2 = [233_u8,188_u8];
_3 = 4235980360286958786_u64 as isize;
_12.3.fld5.0 = [934082878_i32,1538356519_i32];
_12.3.fld5.0 = [1124499266_i32,1463677701_i32];
_11.0 = _12.3.fld5.0;
Call(_11 = fn4(_7, _2, _4, _4, _8, _4, _1, _8, _4, _1, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_14 = _3 as u16;
_8 = false as i16;
_13 = _12.3.fld1;
_12.3.fld4 = _1 * _1;
_11 = (_12.3.fld5.0, 1066077369_u32);
_10 = [(-124_i8),(-114_i8),11_i8,105_i8,124_i8];
_12.3.fld5.1 = _11.1 / _11.1;
_9 = [_6,_6,_6,_6,_6,_6,_6,_6];
_16 = &_6;
Goto(bb2)
}
bb2 = {
_18 = (-52_i8) as f32;
_20 = &_12.2;
_12.1 = Move(_16);
_12.3.fld6 = _6 * _6;
_5 = _18 - _18;
_12.3.fld2 = RET;
RET = _12.3.fld2 + _12.3.fld2;
_2 = [198_u8,9_u8];
_13 = _12.3.fld1;
_10 = [(-77_i8),(-93_i8),120_i8,(-126_i8),(-45_i8)];
_6 = (-1145664839_i32) as i64;
_5 = _18;
_11 = (_12.3.fld5.0, _12.3.fld5.1);
RET = _12.3.fld2 | _12.3.fld2;
_19 = !_11.1;
_12.3.fld5.0 = _11.0;
_5 = -_18;
_10 = [49_i8,(-81_i8),(-107_i8),(-99_i8),(-88_i8)];
Goto(bb3)
}
bb3 = {
_12.3.fld3 = !51_i8;
_17 = -_5;
_4 = [_12.3.fld4,_1,_12.3.fld4,_12.3.fld4,_12.3.fld4,_1,_12.3.fld4,_12.3.fld4];
_12.3.fld5 = (_11.0, _19);
_9 = [_12.3.fld6,_12.3.fld6,_12.3.fld6,_12.3.fld6,_12.3.fld6,_6,_12.3.fld6,_6];
_12.3.fld5.1 = _11.1;
_17 = _18 * _18;
Goto(bb4)
}
bb4 = {
_19 = _12.3.fld5.1;
_12.3.fld0 = _17;
RET = -_12.3.fld2;
_21 = _3;
_11.1 = !_19;
_13 = _12.3.fld1;
_12.3.fld1 = _13;
_12.1 = &_12.3.fld6;
_4 = [_1,_1,_12.3.fld4,_12.3.fld4,_12.3.fld4,_1,_1,_12.3.fld4];
_5 = -_18;
_12.3.fld2 = RET;
_8 = _18 as i16;
_9 = [_12.3.fld6,_12.3.fld6,_12.3.fld6,_6,_12.3.fld6,_12.3.fld6,_12.3.fld6,_12.3.fld6];
_20 = &(*_20);
_13 = _12.3.fld1;
_12.3.fld6 = _12.3.fld3 as i64;
_11.1 = !_12.3.fld5.1;
_17 = -_12.3.fld0;
Goto(bb5)
}
bb5 = {
_13 = _12.3.fld1;
_2 = [51_u8,136_u8];
_16 = &_6;
_12.3.fld6 = 5208550947409843946_u64 as i64;
_12.0 = _12.3.fld4 as f32;
_20 = &(*_20);
_18 = _12.0;
_12.3 = Adt38 { fld0: _12.0,fld1: _13,fld2: RET,fld3: 32_i8,fld4: _1,fld5: _11,fld6: (*_16) };
_12.3.fld6 = _6 >> _12.3.fld4;
_20 = &(*_20);
_10 = [_12.3.fld3,_12.3.fld3,_12.3.fld3,_12.3.fld3,_12.3.fld3];
_3 = !_21;
_12.3 = Adt38 { fld0: _12.0,fld1: _13,fld2: RET,fld3: (-94_i8),fld4: _1,fld5: _11,fld6: _6 };
_12.3.fld1 = _13;
_16 = &(*_16);
_7 = _3 as f64;
_12.0 = _12.3.fld0;
_27 = !_6;
_8 = -_12.3.fld4;
_16 = &(*_16);
_12.1 = &(*_16);
Call(_12.3.fld3 = core::intrinsics::bswap((-36_i8)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_12.3.fld5.0 = [1376136888_i32,(-891956391_i32)];
RET = _12.3.fld2 & _12.3.fld2;
_1 = false as i16;
_16 = Move(_12.1);
_12.3.fld5.0 = [(-2133853125_i32),913127442_i32];
_12.2 = [202_u8,0_u8,148_u8,129_u8,183_u8,206_u8,97_u8];
_7 = _8 as f64;
_12.3.fld5.0 = [1311265135_i32,760946954_i32];
_14 = 12925_u16;
_19 = _11.1 & _12.3.fld5.1;
_22 = [_11.1,_19,_19];
_19 = 705034123_i32 as u32;
Call(_12.3.fld2 = core::intrinsics::bswap(RET), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_22 = [_12.3.fld5.1,_12.3.fld5.1,_12.3.fld5.1];
_20 = &_12.2;
_25 = -_12.3.fld3;
_12.3 = Adt38 { fld0: _18,fld1: _13,fld2: RET,fld3: _25,fld4: _8,fld5: _11,fld6: _27 };
_12.1 = &_6;
_16 = &_12.3.fld6;
_11.1 = !_12.3.fld5.1;
_12.3.fld4 = -_8;
_2 = [25_u8,77_u8];
_15 = core::ptr::addr_of_mut!(_29);
_23 = Adt24::Variant1 { fld0: _14,fld1: 6969248558402362683_u64 };
_27 = !_12.3.fld6;
_12.3.fld5.1 = !_11.1;
_17 = -_12.0;
_22 = [_12.3.fld5.1,_11.1,_11.1];
_11.0 = [(-661571430_i32),(-200999878_i32)];
_4 = [_8,_12.3.fld4,_12.3.fld4,_12.3.fld4,_8,_12.3.fld4,_12.3.fld4,_12.3.fld4];
_23 = Adt24::Variant0 { fld0: 83_u8,fld1: _12.3.fld1,fld2: _8 };
place!(Field::<i16>(Variant(_23, 0), 2)) = _8;
_12.2 = [246_u8,68_u8,234_u8,7_u8,26_u8,237_u8,136_u8];
_30 = !11269276323506801687_usize;
match _14 {
0 => bb1,
1 => bb3,
12925 => bb9,
_ => bb8
}
}
bb8 = {
_18 = (-52_i8) as f32;
_20 = &_12.2;
_12.1 = Move(_16);
_12.3.fld6 = _6 * _6;
_5 = _18 - _18;
_12.3.fld2 = RET;
RET = _12.3.fld2 + _12.3.fld2;
_2 = [198_u8,9_u8];
_13 = _12.3.fld1;
_10 = [(-77_i8),(-93_i8),120_i8,(-126_i8),(-45_i8)];
_6 = (-1145664839_i32) as i64;
_5 = _18;
_11 = (_12.3.fld5.0, _12.3.fld5.1);
RET = _12.3.fld2 | _12.3.fld2;
_19 = !_11.1;
_12.3.fld5.0 = _11.0;
_5 = -_18;
_10 = [49_i8,(-81_i8),(-107_i8),(-99_i8),(-88_i8)];
Goto(bb3)
}
bb9 = {
_21 = -_3;
_18 = _17;
_17 = _12.3.fld0;
place!(Field::<i16>(Variant(_23, 0), 2)) = _12.3.fld4 - _12.3.fld4;
_12.3.fld6 = _6;
place!(Field::<u8>(Variant(_23, 0), 0)) = !50_u8;
_12.3.fld5 = (_11.0, _11.1);
Goto(bb10)
}
bb10 = {
SetDiscriminant(_23, 2);
_16 = &place!(Field::<(u64, u64, u64, i64)>(Variant(_23, 2), 2)).3;
_12.3.fld0 = _18;
_21 = _11.1 as isize;
place!(Field::<i32>(Variant(_23, 2), 5)) = _12.3.fld3 as i32;
_12.3.fld3 = !_25;
_14 = 5419_u16;
_12.3.fld0 = _17 * _18;
_30 = _12.3.fld2 as usize;
_12.3.fld0 = _17 + _18;
RET = _21 as i128;
place!(Field::<(u64, u64, u64, i64)>(Variant(_23, 2), 2)) = (9613117387606381550_u64, 2908276304705357080_u64, 13445557261464110522_u64, _6);
_8 = !_12.3.fld4;
_21 = -_3;
_12.3.fld6 = -_27;
_12.3.fld0 = _18;
_16 = Move(_12.1);
Goto(bb11)
}
bb11 = {
_8 = !_12.3.fld4;
place!(Field::<char>(Variant(_23, 2), 1)) = _12.3.fld1;
_22 = [_12.3.fld5.1,_12.3.fld5.1,_11.1];
_5 = RET as f32;
_12.1 = &_6;
_12.2 = [223_u8,217_u8,163_u8,140_u8,213_u8,245_u8,113_u8];
_12.3.fld1 = Field::<char>(Variant(_23, 2), 1);
_23 = Adt24::Variant1 { fld0: _14,fld1: 8782063231924344038_u64 };
RET = _12.3.fld2;
_12.3.fld5.1 = _11.1;
_35 = _7 - _7;
_35 = _21 as f64;
_12.3.fld2 = _12.3.fld4 as i128;
_12.3.fld5 = _11;
_12.3.fld5 = _11;
_21 = 310121344880186379_u64 as isize;
_28 = -_17;
_12.3.fld0 = _18;
place!(Field::<u64>(Variant(_23, 1), 1)) = 4599148329688345281_u64 + 11855097847933047448_u64;
_33 = _12.3.fld1;
_12.3.fld0 = -_28;
_12.3.fld4 = _8 * _8;
_20 = &_12.2;
_16 = &_6;
_36 = false ^ false;
_12.3.fld3 = _25;
Goto(bb12)
}
bb12 = {
_35 = -_7;
place!(Field::<u64>(Variant(_23, 1), 1)) = !10954661682899792623_u64;
_32 = Field::<u16>(Variant(_23, 1), 0) as u32;
_27 = (*_16) & _6;
_40 = [(-1611086389_i32),1615715341_i32];
_26 = [_30,_30,_30];
_17 = 1013318203_i32 as f32;
_13 = _33;
Goto(bb13)
}
bb13 = {
_38.0 = &_12.3.fld3;
_36 = true & false;
_12.3.fld1 = _13;
_26 = [_30,_30,_30];
_8 = (-289635192_i32) as i16;
_12.3.fld4 = _8;
Goto(bb14)
}
bb14 = {
_28 = _30 as f32;
_25 = _14 as i8;
_27 = _12.3.fld6 << _12.3.fld3;
SetDiscriminant(_23, 3);
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(3_usize, 30_usize, Move(_30), 11_usize, Move(_11), 8_usize, Move(_8), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(3_usize, 32_usize, Move(_32), 25_usize, Move(_25), 10_usize, Move(_10), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(3_usize, 40_usize, Move(_40), 14_usize, Move(_14), 33_usize, Move(_33), 45_usize, _45), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: f64,mut _2: [u8; 2],mut _3: [i16; 8],mut _4: [i16; 8],mut _5: i16,mut _6: [i16; 8],mut _7: i16,mut _8: i16,mut _9: [i16; 8],mut _10: i16,mut _11: [i16; 8]) -> ([i32; 2], u32) {
mir! {
type RET = ([i32; 2], u32);
let _12: (&'static [u8; 2], &'static i64);
let _13: [u8; 7];
let _14: u16;
let _15: &'static &'static u16;
let _16: [u16; 3];
let _17: bool;
let _18: &'static &'static u16;
let _19: i32;
let _20: isize;
let _21: f64;
let _22: bool;
let _23: bool;
let _24: [usize; 4];
let _25: u8;
let _26: *mut u128;
let _27: [i8; 7];
let _28: Adt24;
let _29: (u64, i128);
let _30: isize;
let _31: u64;
let _32: i16;
let _33: *mut [i8; 7];
let _34: bool;
let _35: char;
let _36: *mut char;
let _37: isize;
let _38: i128;
let _39: isize;
let _40: ();
let _41: ();
{
_2 = [241_u8,50_u8];
RET.1 = !1299958520_u32;
_3 = [_5,_10,_7,_7,_10,_5,_10,_10];
RET.0 = [(-922262176_i32),1411219271_i32];
_11 = [_10,_7,_10,_10,_5,_7,_10,_10];
_8 = false as i16;
_8 = _10 | _7;
_4 = _6;
RET.0 = [(-507154080_i32),1301538484_i32];
RET.0 = [(-2099283771_i32),1541577111_i32];
_4 = [_8,_10,_7,_7,_8,_8,_5,_5];
_3 = [_10,_7,_5,_8,_5,_10,_8,_7];
_5 = _10 << _8;
_12.0 = &_2;
_6 = [_5,_8,_8,_7,_7,_7,_8,_8];
_10 = 10452430806164221369_u64 as i16;
RET.1 = 305665461_u32 & 4064814489_u32;
_10 = _8 + _5;
_13 = [253_u8,241_u8,187_u8,126_u8,122_u8,69_u8,35_u8];
_8 = _10 * _7;
_7 = _8 | _10;
_2 = [162_u8,211_u8];
_11 = [_5,_8,_7,_5,_10,_8,_7,_5];
_4 = [_8,_5,_10,_7,_7,_5,_10,_10];
_2 = [74_u8,140_u8];
_3 = _9;
Goto(bb1)
}
bb1 = {
_6 = [_10,_8,_10,_8,_7,_7,_8,_5];
_16 = [34325_u16,23966_u16,45283_u16];
_3 = [_10,_8,_8,_5,_7,_7,_5,_8];
_7 = !_8;
_13 = [86_u8,249_u8,149_u8,243_u8,53_u8,198_u8,84_u8];
_11 = [_5,_5,_5,_10,_8,_7,_10,_8];
_11 = _3;
_7 = _8;
_17 = false;
RET.0 = [(-200504063_i32),(-67181436_i32)];
_3 = [_5,_7,_10,_5,_8,_10,_8,_7];
_14 = 33985_u16;
_2 = [51_u8,162_u8];
RET.0 = [(-249714350_i32),473591810_i32];
_12.0 = &_2;
match _14 {
0 => bb2,
33985 => bb4,
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
_5 = !_10;
_14 = 33301_u16 ^ 7699_u16;
_8 = 142_u8 as i16;
_14 = 28323_u16 | 20114_u16;
Call(_19 = fn5(_6, Move(_12.0), _3, _7), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_10 = -_5;
RET.0 = [_19,_19];
_11 = _6;
_5 = !_10;
Goto(bb6)
}
bb6 = {
RET.0 = [_19,_19];
RET.0 = [_19,_19];
RET.0 = [_19,_19];
RET.0 = [_19,_19];
_2 = [50_u8,51_u8];
_4 = _6;
_7 = -_10;
_4 = _6;
RET.1 = !4073502195_u32;
_22 = _10 == _10;
_21 = _1;
RET.0 = [_19,_19];
_1 = -_21;
_17 = _22;
_23 = _10 < _5;
_22 = _23;
Goto(bb7)
}
bb7 = {
_23 = _22 != _22;
_9 = _11;
_27 = [(-83_i8),(-122_i8),(-42_i8),26_i8,15_i8,(-90_i8),(-52_i8)];
_2 = [187_u8,80_u8];
_11 = [_7,_10,_7,_10,_7,_5,_10,_5];
_1 = -_21;
_16 = [_14,_14,_14];
_29 = (16252456973979881729_u64, (-96254931837627487817102178542694167382_i128));
_2 = [111_u8,164_u8];
_20 = 9223372036854775807_isize + 9223372036854775807_isize;
_20 = 9223372036854775807_isize >> _5;
_1 = -_21;
_25 = 12162453573832089307_usize as u8;
_6 = [_7,_5,_10,_7,_10,_10,_5,_10];
_19 = !(-1948345170_i32);
_11 = [_10,_5,_5,_5,_7,_7,_7,_10];
Call(_26 = fn18(_4, _23, _6, _20, _23, _5, _23, _20), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_25 = 56_u8 | 104_u8;
_3 = [_10,_10,_7,_10,_7,_10,_7,_10];
_34 = _17;
_10 = !_5;
_11 = [_5,_5,_7,_10,_7,_10,_7,_5];
_23 = !_17;
_29.1 = (-10895968674884096292776729379648382604_i128) & 91319289277529529452848066850850055309_i128;
_30 = 295185560859438791137149633505960988544_u128 as isize;
_7 = _29.0 as i16;
_28 = Adt24::Variant1 { fld0: _14,fld1: _29.0 };
_30 = _20;
_12.0 = &_2;
SetDiscriminant(_28, 1);
_29 = (850963359033816083_u64, (-51663586168958173331909252176916052122_i128));
_32 = '\u{1b0cd}' as i16;
_37 = 158227358523566564071821373222929718718_u128 as isize;
_19 = 1715083429_i32;
_6 = _9;
RET.1 = 656063179_u32 << _5;
_4 = [_10,_5,_10,_5,_10,_5,_5,_5];
_39 = !_30;
_24 = [6_usize,1152888761831870947_usize,12185708380243917732_usize,0_usize];
place!(Field::<u16>(Variant(_28, 1), 0)) = !_14;
_25 = 4_usize as u8;
_19 = 895549111_i32;
Goto(bb9)
}
bb9 = {
Call(_40 = dump_var(4_usize, 29_usize, Move(_29), 34_usize, Move(_34), 9_usize, Move(_9), 8_usize, Move(_8)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_40 = dump_var(4_usize, 20_usize, Move(_20), 3_usize, Move(_3), 16_usize, Move(_16), 2_usize, Move(_2)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_40 = dump_var(4_usize, 10_usize, Move(_10), 39_usize, Move(_39), 27_usize, Move(_27), 14_usize, Move(_14)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_40 = dump_var(4_usize, 19_usize, Move(_19), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: [i16; 8],mut _2: &'static [u8; 2],mut _3: [i16; 8],mut _4: i16) -> i32 {
mir! {
type RET = i32;
let _5: *const [i32; 2];
let _6: i64;
let _7: (usize, u16, Adt19, bool);
let _8: (&'static i8,);
let _9: f32;
let _10: char;
let _11: [i32; 2];
let _12: i128;
let _13: *mut u128;
let _14: [isize; 8];
let _15: char;
let _16: *const ([bool; 1], *mut *mut Adt19);
let _17: *const [u8; 2];
let _18: u16;
let _19: i16;
let _20: &'static &'static u16;
let _21: isize;
let _22: [bool; 1];
let _23: *mut Adt19;
let _24: isize;
let _25: [u8; 2];
let _26: [u16; 6];
let _27: isize;
let _28: &'static u32;
let _29: u128;
let _30: *const [u32; 3];
let _31: i32;
let _32: bool;
let _33: [i32; 2];
let _34: i8;
let _35: [bool; 1];
let _36: ();
let _37: ();
{
_3 = [_4,_4,_4,_4,_4,_4,_4,_4];
_3 = [_4,_4,_4,_4,_4,_4,_4,_4];
RET = (-9223372036854775808_isize) as i32;
RET = -925983390_i32;
_4 = (-10701_i16);
_1 = [_4,_4,_4,_4,_4,_4,_4,_4];
RET = (-966269164_i32);
RET = '\u{87734}' as i32;
_3 = _1;
_4 = !(-9358_i16);
_3 = _1;
_4 = 28087_i16 - (-23313_i16);
_3 = [_4,_4,_4,_4,_4,_4,_4,_4];
RET = !(-979304285_i32);
_1 = _3;
_7.3 = !true;
_6 = 9647136536510652580_u64 as i64;
RET = 472296856_u32 as i32;
_7.1 = 94407006465648357865308390237815271364_i128 as u16;
Call(_7.2 = fn6(_3, _3, _3, _4, _6, RET, _4, _1, _3, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<bool>(Variant(_7.2, 0), 0)) = Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).1 != Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).2;
_7.3 = Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).0 != Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).2;
place!(Field::<bool>(Variant(_7.2, 0), 0)) = _7.3;
_6 = -Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).3;
_7.1 = 51716_u16;
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)).1 = Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).2;
SetDiscriminant(_7.2, 0);
_9 = (-31_i8) as f32;
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)).1 = !802920147295145203_u64;
_7.1 = 253_u8 as u16;
_10 = '\u{10ab67}';
_6 = _10 as i64;
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)).0 = !Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).1;
_5 = core::ptr::addr_of!(_11);
_11 = [RET,RET];
_7.2 = Adt19::Variant1 { fld0: 13946294218733143801834283924627499012_i128 };
_7.3 = _9 >= _9;
_1 = [_4,_4,_4,_4,_4,_4,_4,_4];
_3 = _1;
_7.0 = !1_usize;
_11 = [RET,RET];
RET = 816460158_i32 - 1176420549_i32;
_4 = 1919700680_u32 as i16;
_10 = '\u{18639}';
Call(_11 = fn8(_3, _4, _7.3, _3, _6, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7.0 = 14099773147802672591_usize;
_11 = [RET,RET];
_12 = 8100573471675850713_u64 as i128;
_7.2 = Adt19::Variant1 { fld0: _12 };
_1 = [_4,_4,_4,_4,_4,_4,_4,_4];
place!(Field::<i128>(Variant(_7.2, 1), 0)) = _12;
_9 = _4 as f32;
(*_5) = [RET,RET];
_5 = core::ptr::addr_of!((*_5));
(*_5) = [RET,RET];
_4 = (-24786_i16);
_12 = Field::<i128>(Variant(_7.2, 1), 0) - Field::<i128>(Variant(_7.2, 1), 0);
RET = 193927195_u32 as i32;
_6 = !(-1582571253210251373_i64);
_9 = RET as f32;
_7.0 = 7_usize * 6_usize;
_4 = !20891_i16;
_1 = _3;
_6 = _7.0 as i64;
_5 = core::ptr::addr_of!((*_5));
_1 = [_4,_4,_4,_4,_4,_4,_4,_4];
_7.3 = _12 == Field::<i128>(Variant(_7.2, 1), 0);
RET = !(-1039972020_i32);
_1 = _3;
_5 = core::ptr::addr_of!(_11);
(*_5) = [RET,RET];
_1 = [_4,_4,_4,_4,_4,_4,_4,_4];
Goto(bb3)
}
bb3 = {
_6 = 3922978065_u32 as i64;
SetDiscriminant(_7.2, 1);
_7.1 = 65242_u16;
_7.1 = 9223372036854775807_isize as u16;
_3 = [_4,_4,_4,_4,_4,_4,_4,_4];
_14 = [56_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-113_isize),(-9223372036854775808_isize),75_isize];
_1 = [_4,_4,_4,_4,_4,_4,_4,_4];
_3 = [_4,_4,_4,_4,_4,_4,_4,_4];
_7.3 = !false;
_11 = [RET,RET];
_11 = [RET,RET];
_15 = _10;
_7.3 = !true;
_3 = [_4,_4,_4,_4,_4,_4,_4,_4];
_12 = 33981575031710079107781252804234082844_i128;
_7.0 = _12 as usize;
_7.2 = Adt19::Variant1 { fld0: _12 };
(*_5) = [RET,RET];
_4 = -10957_i16;
_6 = 2145756692569623546_i64 * (-2520764639958749746_i64);
(*_5) = [RET,RET];
_11 = [RET,RET];
RET = (-470666629_i32);
(*_5) = [RET,RET];
_12 = -Field::<i128>(Variant(_7.2, 1), 0);
match RET {
0 => bb1,
340282366920938463463374607431297544827 => bb5,
_ => bb4
}
}
bb4 = {
place!(Field::<bool>(Variant(_7.2, 0), 0)) = Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).1 != Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).2;
_7.3 = Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).0 != Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).2;
place!(Field::<bool>(Variant(_7.2, 0), 0)) = _7.3;
_6 = -Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).3;
_7.1 = 51716_u16;
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)).1 = Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).2;
SetDiscriminant(_7.2, 0);
_9 = (-31_i8) as f32;
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)).1 = !802920147295145203_u64;
_7.1 = 253_u8 as u16;
_10 = '\u{10ab67}';
_6 = _10 as i64;
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)).0 = !Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).1;
_5 = core::ptr::addr_of!(_11);
_11 = [RET,RET];
_7.2 = Adt19::Variant1 { fld0: 13946294218733143801834283924627499012_i128 };
_7.3 = _9 >= _9;
_1 = [_4,_4,_4,_4,_4,_4,_4,_4];
_3 = _1;
_7.0 = !1_usize;
_11 = [RET,RET];
RET = 816460158_i32 - 1176420549_i32;
_4 = 1919700680_u32 as i16;
_10 = '\u{18639}';
Call(_11 = fn8(_3, _4, _7.3, _3, _6, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb5 = {
_6 = 5334748457909974686_i64 << _4;
_4 = (-3043_i16);
match _4 {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
340282366920938463463374607431768208413 => bb11,
_ => bb10
}
}
bb6 = {
place!(Field::<bool>(Variant(_7.2, 0), 0)) = Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).1 != Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).2;
_7.3 = Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).0 != Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).2;
place!(Field::<bool>(Variant(_7.2, 0), 0)) = _7.3;
_6 = -Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).3;
_7.1 = 51716_u16;
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)).1 = Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).2;
SetDiscriminant(_7.2, 0);
_9 = (-31_i8) as f32;
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)).1 = !802920147295145203_u64;
_7.1 = 253_u8 as u16;
_10 = '\u{10ab67}';
_6 = _10 as i64;
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)).0 = !Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).1;
_5 = core::ptr::addr_of!(_11);
_11 = [RET,RET];
_7.2 = Adt19::Variant1 { fld0: 13946294218733143801834283924627499012_i128 };
_7.3 = _9 >= _9;
_1 = [_4,_4,_4,_4,_4,_4,_4,_4];
_3 = _1;
_7.0 = !1_usize;
_11 = [RET,RET];
RET = 816460158_i32 - 1176420549_i32;
_4 = 1919700680_u32 as i16;
_10 = '\u{18639}';
Call(_11 = fn8(_3, _4, _7.3, _3, _6, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_6 = 3922978065_u32 as i64;
SetDiscriminant(_7.2, 1);
_7.1 = 65242_u16;
_7.1 = 9223372036854775807_isize as u16;
_3 = [_4,_4,_4,_4,_4,_4,_4,_4];
_14 = [56_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-113_isize),(-9223372036854775808_isize),75_isize];
_1 = [_4,_4,_4,_4,_4,_4,_4,_4];
_3 = [_4,_4,_4,_4,_4,_4,_4,_4];
_7.3 = !false;
_11 = [RET,RET];
_11 = [RET,RET];
_15 = _10;
_7.3 = !true;
_3 = [_4,_4,_4,_4,_4,_4,_4,_4];
_12 = 33981575031710079107781252804234082844_i128;
_7.0 = _12 as usize;
_7.2 = Adt19::Variant1 { fld0: _12 };
(*_5) = [RET,RET];
_4 = -10957_i16;
_6 = 2145756692569623546_i64 * (-2520764639958749746_i64);
(*_5) = [RET,RET];
_11 = [RET,RET];
RET = (-470666629_i32);
(*_5) = [RET,RET];
_12 = -Field::<i128>(Variant(_7.2, 1), 0);
match RET {
0 => bb1,
340282366920938463463374607431297544827 => bb5,
_ => bb4
}
}
bb8 = {
_7.0 = 14099773147802672591_usize;
_11 = [RET,RET];
_12 = 8100573471675850713_u64 as i128;
_7.2 = Adt19::Variant1 { fld0: _12 };
_1 = [_4,_4,_4,_4,_4,_4,_4,_4];
place!(Field::<i128>(Variant(_7.2, 1), 0)) = _12;
_9 = _4 as f32;
(*_5) = [RET,RET];
_5 = core::ptr::addr_of!((*_5));
(*_5) = [RET,RET];
_4 = (-24786_i16);
_12 = Field::<i128>(Variant(_7.2, 1), 0) - Field::<i128>(Variant(_7.2, 1), 0);
RET = 193927195_u32 as i32;
_6 = !(-1582571253210251373_i64);
_9 = RET as f32;
_7.0 = 7_usize * 6_usize;
_4 = !20891_i16;
_1 = _3;
_6 = _7.0 as i64;
_5 = core::ptr::addr_of!((*_5));
_1 = [_4,_4,_4,_4,_4,_4,_4,_4];
_7.3 = _12 == Field::<i128>(Variant(_7.2, 1), 0);
RET = !(-1039972020_i32);
_1 = _3;
_5 = core::ptr::addr_of!(_11);
(*_5) = [RET,RET];
_1 = [_4,_4,_4,_4,_4,_4,_4,_4];
Goto(bb3)
}
bb9 = {
place!(Field::<bool>(Variant(_7.2, 0), 0)) = Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).1 != Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).2;
_7.3 = Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).0 != Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).2;
place!(Field::<bool>(Variant(_7.2, 0), 0)) = _7.3;
_6 = -Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).3;
_7.1 = 51716_u16;
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)).1 = Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).2;
SetDiscriminant(_7.2, 0);
_9 = (-31_i8) as f32;
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)).1 = !802920147295145203_u64;
_7.1 = 253_u8 as u16;
_10 = '\u{10ab67}';
_6 = _10 as i64;
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)).0 = !Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).1;
_5 = core::ptr::addr_of!(_11);
_11 = [RET,RET];
_7.2 = Adt19::Variant1 { fld0: 13946294218733143801834283924627499012_i128 };
_7.3 = _9 >= _9;
_1 = [_4,_4,_4,_4,_4,_4,_4,_4];
_3 = _1;
_7.0 = !1_usize;
_11 = [RET,RET];
RET = 816460158_i32 - 1176420549_i32;
_4 = 1919700680_u32 as i16;
_10 = '\u{18639}';
Call(_11 = fn8(_3, _4, _7.3, _3, _6, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
_7.0 = 4_usize;
_9 = _7.1 as f32;
_6 = _7.1 as i64;
_7.0 = 9625695319835401511_usize;
RET = 750345985_i32 - 66123354_i32;
(*_5) = [RET,RET];
_5 = core::ptr::addr_of!((*_5));
_7.3 = false;
_1 = [_4,_4,_4,_4,_4,_4,_4,_4];
_7.0 = 5_usize;
_15 = _10;
SetDiscriminant(_7.2, 1);
(*_5) = [RET,RET];
Call(_7.2 = fn9(_14, _14), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_7.3 = !true;
SetDiscriminant(_7.2, 0);
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)) = (13199865736027032317_u64, 2695687039158501029_u64, 8170877402640432227_u64, _6);
place!(Field::<bool>(Variant(_7.2, 0), 0)) = _7.3;
_3 = _1;
_9 = _12 as f32;
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)).2 = Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).1;
_7.3 = _12 < _12;
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)).2 = _7.1 as u64;
_4 = _12 as i16;
_11 = [RET,RET];
_14 = [(-30_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,39_isize,59_isize];
place!(Field::<u32>(Variant(_7.2, 0), 2)) = !3667459378_u32;
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)) = (7917098019775762366_u64, 14119070613727113676_u64, 12078129233682066314_u64, _6);
_11 = [RET,RET];
_15 = _10;
_19 = _4;
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)).3 = !_6;
(*_5) = [RET,RET];
_19 = -_4;
RET = (-177912120_i32);
_11 = [RET,RET];
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)).1 = !Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1).0;
_10 = _15;
_11 = [RET,RET];
_5 = core::ptr::addr_of!((*_5));
Goto(bb13)
}
bb13 = {
_23 = core::ptr::addr_of_mut!(_7.2);
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)) = (3999356494528630387_u64, 17536216797260774995_u64, 8812880012112723100_u64, _6);
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)) = (15007182122949559211_u64, 4532233228866295440_u64, 10496038014313864790_u64, _6);
_17 = core::ptr::addr_of!(_25);
_24 = (-9223372036854775808_isize);
SetDiscriminant(_7.2, 1);
(*_17) = [217_u8,3_u8];
_10 = _15;
place!(Field::<i128>(Variant(_7.2, 1), 0)) = _12 >> _7.1;
_2 = &(*_17);
_4 = _19;
SetDiscriminant(_7.2, 0);
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)).2 = !13116184232846268077_u64;
_17 = core::ptr::addr_of!((*_2));
place!(Field::<bool>(Variant(_7.2, 0), 0)) = _7.3;
_26 = [_7.1,_7.1,_7.1,_7.1,_7.1,_7.1];
place!(Field::<(u64, u64, u64, i64)>(Variant(_7.2, 0), 1)).1 = Field::<(u64, u64, u64, i64)>(Variant((*_23), 0), 1).2 << RET;
place!(Field::<(u64, u64, u64, i64)>(Variant((*_23), 0), 1)).1 = Field::<(u64, u64, u64, i64)>(Variant((*_23), 0), 1).2;
(*_23) = Adt19::Variant1 { fld0: _12 };
(*_5) = [RET,RET];
_18 = _24 as u16;
Goto(bb14)
}
bb14 = {
_19 = _6 as i16;
_31 = RET;
_4 = -_19;
_32 = _7.3 | _7.3;
_29 = 224204493277256728513659303597203888573_u128 - 304437273291099743781390147325333005274_u128;
(*_23) = Adt19::Variant1 { fld0: _12 };
_17 = core::ptr::addr_of!((*_17));
_15 = _10;
RET = _31 >> _18;
_3 = [_19,_4,_19,_19,_19,_4,_4,_19];
_13 = core::ptr::addr_of_mut!(_29);
_9 = 4172067069_u32 as f32;
place!(Field::<i128>(Variant(_7.2, 1), 0)) = _12;
_2 = &_25;
_21 = _24;
_5 = core::ptr::addr_of!(_33);
_18 = _4 as u16;
_1 = [_4,_19,_19,_4,_19,_19,_19,_4];
_19 = !_4;
_26 = [_7.1,_18,_7.1,_18,_7.1,_18];
_13 = core::ptr::addr_of_mut!((*_13));
(*_5) = [RET,_31];
_14 = [_21,_21,_24,_21,_24,_24,_21,_24];
(*_23) = Adt19::Variant1 { fld0: _12 };
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(5_usize, 32_usize, Move(_32), 12_usize, Move(_12), 18_usize, Move(_18), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(5_usize, 10_usize, Move(_10), 1_usize, Move(_1), 31_usize, Move(_31), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(5_usize, 26_usize, Move(_26), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [i16; 8],mut _2: [i16; 8],mut _3: [i16; 8],mut _4: i16,mut _5: i64,mut _6: i32,mut _7: i16,mut _8: [i16; 8],mut _9: [i16; 8],mut _10: [i16; 8]) -> Adt19 {
mir! {
type RET = Adt19;
let _11: i32;
let _12: Adt59;
let _13: &'static [u8; 2];
let _14: &'static u128;
let _15: i64;
let _16: f64;
let _17: *mut Adt19;
let _18: *mut u8;
let _19: (&'static char,);
let _20: &'static &'static u16;
let _21: (u64, [u64; 7], (u64, u64, u64, i64));
let _22: &'static u16;
let _23: f64;
let _24: i64;
let _25: isize;
let _26: [bool; 1];
let _27: ();
let _28: ();
{
_1 = _3;
_1 = [_7,_4,_7,_4,_4,_4,_7,_4];
RET = Adt19::Variant1 { fld0: 151342954646519794454698516517174942334_i128 };
_2 = [_4,_4,_7,_7,_7,_4,_4,_4];
_3 = [_4,_7,_7,_4,_4,_7,_7,_4];
place!(Field::<i128>(Variant(RET, 1), 0)) = _6 as i128;
_6 = (-2146264480_i32);
RET = Adt19::Variant1 { fld0: (-55583739654948592198682207906511440065_i128) };
_8 = [_7,_7,_7,_7,_7,_7,_7,_4];
place!(Field::<i128>(Variant(RET, 1), 0)) = -(-134140975068506787033209899342054042176_i128);
_5 = (-7653625623835667414_i64);
_3 = [_4,_4,_7,_4,_4,_4,_7,_4];
place!(Field::<i128>(Variant(RET, 1), 0)) = (-94229645462122577797457092781095935086_i128) >> _4;
_6 = !826828945_i32;
_10 = _9;
_1 = _2;
_6 = (-553694354_i32);
_4 = _7 >> _6;
place!(Field::<i128>(Variant(RET, 1), 0)) = 63565678359435679690637127157418422586_u128 as i128;
_5 = '\u{4c117}' as i64;
place!(Field::<i128>(Variant(RET, 1), 0)) = _7 as i128;
_11 = _6 * _6;
_3 = [_7,_4,_7,_4,_7,_7,_4,_4];
Call(place!(Field::<i128>(Variant(RET, 1), 0)) = core::intrinsics::bswap((-6837638422813154841128265473728635659_i128)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = [_4,_4,_4,_7,_7,_4,_7,_4];
_2 = [_4,_4,_4,_4,_4,_4,_7,_4];
_11 = !_6;
RET = Adt19::Variant1 { fld0: 110961705374177513883322547739717244137_i128 };
_5 = 9587659973358841306_u64 as i64;
_5 = (-3529489003576117898_i64);
place!(Field::<i128>(Variant(RET, 1), 0)) = 118574088255504198534995537901518659091_i128;
_5 = 3367246727522320196_i64 >> _7;
_12.fld2 = (3419242344837469678_u64, 11682727268915089881_u64, 7661726746507256854_u64, _5);
_12.fld5 = [251_u8,194_u8,203_u8,248_u8,129_u8,53_u8,9_u8];
_12.fld4.0 = [(-4_i8),(-124_i8),60_i8,(-4_i8),9_i8,23_i8,17_i8];
_12.fld4.0 = [114_i8,75_i8,(-47_i8),(-75_i8),(-15_i8),(-55_i8),69_i8];
_10 = _1;
_12.fld2.2 = !_12.fld2.1;
_1 = [_7,_4,_7,_4,_7,_4,_7,_4];
_12.fld7 = 3663224360_u32 * 346922560_u32;
_12.fld7 = 1765023329_u32 >> _7;
_12.fld1 = (3929989483281653365_usize, 48418_u16, RET, false);
SetDiscriminant(_12.fld1.2, 0);
_2 = [_4,_7,_4,_4,_7,_4,_4,_4];
_5 = _12.fld2.3;
_12.fld6 = !_5;
_15 = _12.fld2.2 as i64;
_12.fld2 = (17097323911699730562_u64, 12166957173593139594_u64, 14836625248402045876_u64, _5);
Call(place!(Field::<(u64, u64, u64, i64)>(Variant(_12.fld1.2, 0), 1)) = fn7(_12.fld2.2, _15, _10, _9, _12.fld1.0, _15, _12.fld2.0, _12.fld5, _12.fld2.1, _12.fld1.1, _12.fld7, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
place!(Field::<bool>(Variant(_12.fld1.2, 0), 0)) = _4 >= _7;
_7 = _4 ^ _4;
_12.fld2 = (Field::<(u64, u64, u64, i64)>(Variant(_12.fld1.2, 0), 1).1, Field::<(u64, u64, u64, i64)>(Variant(_12.fld1.2, 0), 1).1, Field::<(u64, u64, u64, i64)>(Variant(_12.fld1.2, 0), 1).0, _15);
_12.fld1.2 = RET;
_2 = _10;
_5 = _12.fld6;
_12.fld6 = -_15;
_15 = _6 as i64;
_2 = _9;
_17 = core::ptr::addr_of_mut!(_12.fld1.2);
_12.fld2.1 = !_12.fld2.0;
place!(Field::<i128>(Variant((*_17), 1), 0)) = -Field::<i128>(Variant(RET, 1), 0);
SetDiscriminant((*_17), 0);
place!(Field::<(u64, u64, u64, i64)>(Variant((*_17), 0), 1)).1 = _12.fld2.0 << _12.fld1.0;
place!(Field::<(u64, u64, u64, i64)>(Variant((*_17), 0), 1)).0 = _12.fld2.1 + Field::<(u64, u64, u64, i64)>(Variant((*_17), 0), 1).1;
_12.fld1.1 = _12.fld7 as u16;
place!(Field::<(u64, u64, u64, i64)>(Variant(_12.fld1.2, 0), 1)).2 = _12.fld2.2 * _12.fld2.1;
_12.fld4.1 = [_6,_6];
Goto(bb3)
}
bb3 = {
place!(Field::<(u64, u64, u64, i64)>(Variant(_12.fld1.2, 0), 1)).3 = !_12.fld6;
place!(Field::<i128>(Variant(RET, 1), 0)) = -32353563369402381005106061042073595076_i128;
place!(Field::<(u64, u64, u64, i64)>(Variant((*_17), 0), 1)).2 = Field::<(u64, u64, u64, i64)>(Variant((*_17), 0), 1).1;
_12.fld2.2 = Field::<(u64, u64, u64, i64)>(Variant((*_17), 0), 1).0;
_12.fld1.2 = Adt19::Variant1 { fld0: Field::<i128>(Variant(RET, 1), 0) };
_12.fld6 = _12.fld2.3;
_21.2.0 = _12.fld2.1 - _12.fld2.2;
place!(Field::<i128>(Variant((*_17), 1), 0)) = _21.2.0 as i128;
SetDiscriminant((*_17), 0);
RET = Adt19::Variant0 { fld0: _12.fld1.3,fld1: _12.fld2,fld2: _12.fld7 };
place!(Field::<(u64, u64, u64, i64)>(Variant((*_17), 0), 1)) = (_21.2.0, _12.fld2.2, _12.fld2.2, _12.fld6);
_12.fld2.3 = !Field::<(u64, u64, u64, i64)>(Variant(RET, 0), 1).3;
place!(Field::<(u64, u64, u64, i64)>(Variant((*_17), 0), 1)).3 = -Field::<(u64, u64, u64, i64)>(Variant(RET, 0), 1).3;
_12.fld0 = [_12.fld1.1,_12.fld1.1,_12.fld1.1,_12.fld1.1,_12.fld1.1,_12.fld1.1];
_16 = _12.fld7 as f64;
Goto(bb4)
}
bb4 = {
Call(_27 = dump_var(6_usize, 9_usize, Move(_9), 7_usize, Move(_7), 10_usize, Move(_10), 6_usize, Move(_6)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_27 = dump_var(6_usize, 15_usize, Move(_15), 3_usize, Move(_3), 28_usize, _28, 28_usize, _28), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: u64,mut _2: i64,mut _3: [i16; 8],mut _4: [i16; 8],mut _5: usize,mut _6: i64,mut _7: u64,mut _8: [u8; 7],mut _9: u64,mut _10: u16,mut _11: u32,mut _12: [i16; 8]) -> (u64, u64, u64, i64) {
mir! {
type RET = (u64, u64, u64, i64);
let _13: u64;
let _14: [isize; 8];
let _15: [u64; 7];
let _16: *mut u128;
let _17: [usize; 3];
let _18: ();
let _19: ();
{
RET.0 = !_7;
_10 = 10035919186265612462671971542741685799_i128 as u16;
RET = (_1, _1, _9, _2);
_11 = 3300650467_u32;
_3 = [(-11315_i16),6820_i16,(-21434_i16),5200_i16,(-19327_i16),(-20730_i16),(-25841_i16),19775_i16];
_1 = RET.0 ^ RET.0;
_1 = 108_u8 as u64;
_1 = _7;
_14 = [(-9223372036854775808_isize),100_isize,9223372036854775807_isize,(-93_isize),40_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
RET.3 = !_6;
_7 = RET.1;
RET.1 = _10 as u64;
_3 = [26626_i16,(-29779_i16),12494_i16,15933_i16,5357_i16,18487_i16,(-16055_i16),(-25652_i16)];
RET = (_1, _7, _7, _2);
_13 = 1963720886_i32 as u64;
RET.1 = RET.2 & _7;
Goto(bb1)
}
bb1 = {
Call(_18 = dump_var(7_usize, 3_usize, Move(_3), 8_usize, Move(_8), 14_usize, Move(_14), 12_usize, Move(_12)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_18 = dump_var(7_usize, 2_usize, Move(_2), 1_usize, Move(_1), 6_usize, Move(_6), 19_usize, _19), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [i16; 8],mut _2: i16,mut _3: bool,mut _4: [i16; 8],mut _5: i64,mut _6: f32) -> [i32; 2] {
mir! {
type RET = [i32; 2];
let _7: &'static (u64, [u64; 7], (u64, u64, u64, i64));
let _8: *const [u8; 2];
let _9: [i32; 2];
let _10: isize;
let _11: u128;
let _12: u8;
let _13: (*const usize, &'static char);
let _14: ([u32; 3], u16, Adt28);
let _15: u64;
let _16: u64;
let _17: [bool; 1];
let _18: u128;
let _19: [isize; 8];
let _20: isize;
let _21: isize;
let _22: Adt50;
let _23: [u64; 7];
let _24: (*const u64,);
let _25: ();
let _26: ();
{
_4 = [_2,_2,_2,_2,_2,_2,_2,_2];
RET = [1540875414_i32,1947124126_i32];
_6 = 38383_u16 as f32;
Goto(bb1)
}
bb1 = {
_1 = [_2,_2,_2,_2,_2,_2,_2,_2];
_5 = !(-3646930051488526962_i64);
Call(_1 = core::intrinsics::transmute(_4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = [_2,_2,_2,_2,_2,_2,_2,_2];
RET = [(-762751427_i32),(-437225062_i32)];
RET = [346550610_i32,1874787682_i32];
_3 = false;
_9 = [1443770674_i32,(-715390163_i32)];
_9 = RET;
_2 = (-3790_i16) + 31030_i16;
RET = _9;
_1 = _4;
_4 = _1;
_2 = (-7898_i16) & 15938_i16;
Call(_10 = core::intrinsics::transmute(_9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_10 = 16_isize;
_2 = 17083_i16 * 1264_i16;
_12 = 151_u8;
_11 = 293924406167212441019482646841801031489_u128 & 135557144990155227563278379990161472683_u128;
_5 = _10 as i64;
_12 = _11 as u8;
_10 = 0_usize as isize;
_4 = _1;
_2 = -24490_i16;
_4 = _1;
_5 = '\u{e7cfc}' as i64;
RET = [482981127_i32,(-768346617_i32)];
_11 = 6_usize as u128;
_11 = !157716459730644643893924908464904449031_u128;
_9 = [1542215182_i32,(-121065137_i32)];
RET = _9;
_6 = 5656799987055686120_u64 as f32;
_3 = true;
_10 = _6 as isize;
RET = _9;
_6 = _5 as f32;
RET = [(-211550479_i32),392952481_i32];
RET = _9;
_10 = -9223372036854775807_isize;
_3 = !false;
Goto(bb4)
}
bb4 = {
_1 = _4;
_3 = !true;
_15 = 1408909771192146693_u64 & 5053798749757474597_u64;
_14.0 = [2591380151_u32,2066588529_u32,1693379162_u32];
_5 = (-6072037942810566821_i64);
_16 = _15;
_14.0 = [1663274083_u32,1017881399_u32,2026347679_u32];
_6 = (-224484483_i32) as f32;
_2 = 61_i8 as i16;
_14.0 = [837318119_u32,1982418065_u32,881094271_u32];
_15 = _16 ^ _16;
_17 = [_3];
_2 = '\u{2816f}' as i16;
_14.0 = [2064682377_u32,284769742_u32,2519188282_u32];
RET = _9;
_10 = (-9223372036854775808_isize);
_15 = _16;
Goto(bb5)
}
bb5 = {
_3 = true;
_10 = 9223372036854775807_isize & 9223372036854775807_isize;
_2 = 4982273031802282117_usize as i16;
_17 = [_3];
_18 = !_11;
_14.1 = _3 as u16;
_2 = _3 as i16;
_11 = !_18;
_2 = (-29145_i16) & (-14306_i16);
_4 = _1;
_9 = [1625856801_i32,27520576_i32];
_15 = !_16;
_6 = (-1537844532_i32) as f32;
_17 = [_3];
_1 = [_2,_2,_2,_2,_2,_2,_2,_2];
_19 = [_10,_10,_10,_10,_10,_10,_10,_10];
_14.0 = [841466214_u32,4281358300_u32,461930803_u32];
_1 = _4;
_14.1 = 15385_u16;
_9 = RET;
_18 = !_11;
_6 = _12 as f32;
_15 = !_16;
_6 = (-944175344_i32) as f32;
_14.0 = [971928859_u32,3459379193_u32,3385287479_u32];
RET = _9;
_15 = _16;
_12 = _14.1 as u8;
match _14.1 {
15385 => bb7,
_ => bb6
}
}
bb6 = {
_1 = [_2,_2,_2,_2,_2,_2,_2,_2];
_5 = !(-3646930051488526962_i64);
Call(_1 = core::intrinsics::transmute(_4), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_20 = _10 ^ _10;
_3 = !false;
_9 = [(-1968246926_i32),(-1094618034_i32)];
_6 = 2038410601_u32 as f32;
RET = [790940164_i32,(-1553852777_i32)];
_1 = [_2,_2,_2,_2,_2,_2,_2,_2];
_14.0 = [2879959637_u32,2474857118_u32,1475254481_u32];
_20 = _18 as isize;
_9 = [469720567_i32,356280012_i32];
_14.0 = [2938190595_u32,2538243735_u32,3962804472_u32];
_11 = _16 as u128;
match _5 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
340282366920938463457302569488957644635 => bb14,
_ => bb13
}
}
bb8 = {
_1 = [_2,_2,_2,_2,_2,_2,_2,_2];
_5 = !(-3646930051488526962_i64);
Call(_1 = core::intrinsics::transmute(_4), ReturnTo(bb2), UnwindUnreachable())
}
bb9 = {
_3 = true;
_10 = 9223372036854775807_isize & 9223372036854775807_isize;
_2 = 4982273031802282117_usize as i16;
_17 = [_3];
_18 = !_11;
_14.1 = _3 as u16;
_2 = _3 as i16;
_11 = !_18;
_2 = (-29145_i16) & (-14306_i16);
_4 = _1;
_9 = [1625856801_i32,27520576_i32];
_15 = !_16;
_6 = (-1537844532_i32) as f32;
_17 = [_3];
_1 = [_2,_2,_2,_2,_2,_2,_2,_2];
_19 = [_10,_10,_10,_10,_10,_10,_10,_10];
_14.0 = [841466214_u32,4281358300_u32,461930803_u32];
_1 = _4;
_14.1 = 15385_u16;
_9 = RET;
_18 = !_11;
_6 = _12 as f32;
_15 = !_16;
_6 = (-944175344_i32) as f32;
_14.0 = [971928859_u32,3459379193_u32,3385287479_u32];
RET = _9;
_15 = _16;
_12 = _14.1 as u8;
match _14.1 {
15385 => bb7,
_ => bb6
}
}
bb10 = {
_1 = _4;
_3 = !true;
_15 = 1408909771192146693_u64 & 5053798749757474597_u64;
_14.0 = [2591380151_u32,2066588529_u32,1693379162_u32];
_5 = (-6072037942810566821_i64);
_16 = _15;
_14.0 = [1663274083_u32,1017881399_u32,2026347679_u32];
_6 = (-224484483_i32) as f32;
_2 = 61_i8 as i16;
_14.0 = [837318119_u32,1982418065_u32,881094271_u32];
_15 = _16 ^ _16;
_17 = [_3];
_2 = '\u{2816f}' as i16;
_14.0 = [2064682377_u32,284769742_u32,2519188282_u32];
RET = _9;
_10 = (-9223372036854775808_isize);
_15 = _16;
Goto(bb5)
}
bb11 = {
_10 = 16_isize;
_2 = 17083_i16 * 1264_i16;
_12 = 151_u8;
_11 = 293924406167212441019482646841801031489_u128 & 135557144990155227563278379990161472683_u128;
_5 = _10 as i64;
_12 = _11 as u8;
_10 = 0_usize as isize;
_4 = _1;
_2 = -24490_i16;
_4 = _1;
_5 = '\u{e7cfc}' as i64;
RET = [482981127_i32,(-768346617_i32)];
_11 = 6_usize as u128;
_11 = !157716459730644643893924908464904449031_u128;
_9 = [1542215182_i32,(-121065137_i32)];
RET = _9;
_6 = 5656799987055686120_u64 as f32;
_3 = true;
_10 = _6 as isize;
RET = _9;
_6 = _5 as f32;
RET = [(-211550479_i32),392952481_i32];
RET = _9;
_10 = -9223372036854775807_isize;
_3 = !false;
Goto(bb4)
}
bb12 = {
_4 = [_2,_2,_2,_2,_2,_2,_2,_2];
RET = [(-762751427_i32),(-437225062_i32)];
RET = [346550610_i32,1874787682_i32];
_3 = false;
_9 = [1443770674_i32,(-715390163_i32)];
_9 = RET;
_2 = (-3790_i16) + 31030_i16;
RET = _9;
_1 = _4;
_4 = _1;
_2 = (-7898_i16) & 15938_i16;
Call(_10 = core::intrinsics::transmute(_9), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_1 = [_2,_2,_2,_2,_2,_2,_2,_2];
_5 = !(-3646930051488526962_i64);
Call(_1 = core::intrinsics::transmute(_4), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_5 = _16 as i64;
_11 = !_18;
_6 = (-12_i8) as f32;
_14.1 = '\u{a2fb7}' as u16;
_14.0 = [153014192_u32,1377283030_u32,10557194_u32];
RET = _9;
_23 = [_16,_16,_15,_15,_16,_15,_16];
_21 = _3 as isize;
_10 = _20;
_24.0 = core::ptr::addr_of!(_16);
_23 = [_15,_16,_16,_16,_15,_16,_16];
_17 = [_3];
_11 = _18 - _18;
_20 = _10 * _10;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(8_usize, 11_usize, Move(_11), 21_usize, Move(_21), 2_usize, Move(_2), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(8_usize, 17_usize, Move(_17), 19_usize, Move(_19), 12_usize, Move(_12), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [isize; 8],mut _2: [isize; 8]) -> Adt19 {
mir! {
type RET = Adt19;
let _3: *mut Adt19;
let _4: &'static [u8; 2];
let _5: char;
let _6: f64;
let _7: *mut u128;
let _8: u16;
let _9: f64;
let _10: bool;
let _11: bool;
let _12: &'static [i64; 8];
let _13: i16;
let _14: [u16; 3];
let _15: Adt59;
let _16: i8;
let _17: u8;
let _18: isize;
let _19: [i8; 7];
let _20: *const [u32; 3];
let _21: usize;
let _22: (u64, [u64; 7], (u64, u64, u64, i64));
let _23: ();
let _24: ();
{
_2 = [(-9223372036854775808_isize),(-9223372036854775808_isize),52_isize,9223372036854775807_isize,43_isize,9223372036854775807_isize,(-95_isize),22_isize];
RET = Adt19::Variant1 { fld0: 2335405257569571825872835478292055271_i128 };
place!(Field::<i128>(Variant(RET, 1), 0)) = -25654724796587799563775793976544299024_i128;
_2 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-115_isize),9223372036854775807_isize,(-9223372036854775808_isize),23_isize,65_isize];
place!(Field::<i128>(Variant(RET, 1), 0)) = 148537179247083457083998748587721204697_i128;
place!(Field::<i128>(Variant(RET, 1), 0)) = 15571551758463970836847809862567349523_i128 + (-166870521056641117673612117970183979643_i128);
_3 = core::ptr::addr_of_mut!(RET);
_3 = core::ptr::addr_of_mut!(RET);
_2 = [0_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,57_isize];
_3 = core::ptr::addr_of_mut!((*_3));
_3 = core::ptr::addr_of_mut!((*_3));
(*_3) = Adt19::Variant1 { fld0: (-36774668128035082206136946472472395956_i128) };
_3 = core::ptr::addr_of_mut!((*_3));
place!(Field::<i128>(Variant(RET, 1), 0)) = 156421113619140020428726757980742530579_i128 * (-110758984442146866532480718509606886335_i128);
_3 = core::ptr::addr_of_mut!((*_3));
SetDiscriminant((*_3), 0);
Goto(bb1)
}
bb1 = {
place!(Field::<(u64, u64, u64, i64)>(Variant(RET, 0), 1)) = (13889833736832345569_u64, 905815362858663000_u64, 2817182631882405821_u64, (-3687924576052790706_i64));
place!(Field::<u32>(Variant((*_3), 0), 2)) = 2468972002_u32;
place!(Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1)).0 = 144861277638041526569711826639181351898_u128 as u64;
place!(Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1)).0 = !Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1).2;
place!(Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1)) = (1680831609518165654_u64, 7729045282731597224_u64, 12648259803853619105_u64, 3054781913333760462_i64);
_5 = '\u{efba9}';
_2 = [39_isize,(-9223372036854775808_isize),58_isize,(-9223372036854775808_isize),18_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
place!(Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1)).3 = -(-911527611365113231_i64);
place!(Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1)).2 = Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1).1;
place!(Field::<u32>(Variant(RET, 0), 2)) = 2601151897_u32;
place!(Field::<bool>(Variant((*_3), 0), 0)) = false | false;
place!(Field::<(u64, u64, u64, i64)>(Variant(RET, 0), 1)).0 = !Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1).1;
place!(Field::<u32>(Variant(RET, 0), 2)) = 826978166_u32;
_6 = 7876137224619775725_usize as f64;
place!(Field::<bool>(Variant((*_3), 0), 0)) = !true;
place!(Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1)).0 = Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1).1 % Field::<(u64, u64, u64, i64)>(Variant(RET, 0), 1).1;
place!(Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1)).0 = !Field::<(u64, u64, u64, i64)>(Variant(RET, 0), 1).2;
SetDiscriminant(RET, 1);
RET = Adt19::Variant1 { fld0: (-77680958985028682802844269155339956418_i128) };
place!(Field::<i128>(Variant(RET, 1), 0)) = 15341294022667352476_u64 as i128;
RET = Adt19::Variant1 { fld0: (-153032746768369492894892535024254413318_i128) };
_2 = [(-9223372036854775808_isize),38_isize,108_isize,(-47_isize),9223372036854775807_isize,(-57_isize),(-9223372036854775808_isize),13_isize];
Goto(bb2)
}
bb2 = {
RET = Adt19::Variant1 { fld0: 125809565279724576212562751336770329876_i128 };
place!(Field::<i128>(Variant((*_3), 1), 0)) = 8436384891656131651450397492951331110_i128;
_3 = core::ptr::addr_of_mut!((*_3));
place!(Field::<i128>(Variant(RET, 1), 0)) = 103923364369957644504467702538544099252_i128 ^ (-17251641119388483753655479912383326579_i128);
Goto(bb3)
}
bb3 = {
place!(Field::<i128>(Variant(RET, 1), 0)) = 62214_u16 as i128;
_3 = core::ptr::addr_of_mut!((*_3));
_3 = core::ptr::addr_of_mut!((*_3));
Call(place!(Field::<i128>(Variant((*_3), 1), 0)) = fn10(_1, _1, _1, _2, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1 = [(-92_isize),46_isize,(-46_isize),(-9223372036854775808_isize),(-106_isize),118_isize,9223372036854775807_isize,9223372036854775807_isize];
SetDiscriminant((*_3), 1);
(*_3) = Adt19::Variant1 { fld0: (-113286110768424195324689876420172058577_i128) };
(*_3) = Adt19::Variant1 { fld0: (-147505058194298945814695882126488136417_i128) };
_6 = 80_u8 as f64;
place!(Field::<i128>(Variant(RET, 1), 0)) = 50048733538576168895048720522916005539_i128;
place!(Field::<i128>(Variant(RET, 1), 0)) = _6 as i128;
_2 = [(-83_isize),(-9223372036854775808_isize),(-56_isize),(-9223372036854775808_isize),(-22_isize),14_isize,(-82_isize),(-9223372036854775808_isize)];
RET = Adt19::Variant1 { fld0: 30494780797346061383065441997172533057_i128 };
place!(Field::<i128>(Variant(RET, 1), 0)) = -44115167534811957011715655591005220641_i128;
(*_3) = Adt19::Variant1 { fld0: 88814507383165671720764803036106326242_i128 };
_3 = core::ptr::addr_of_mut!((*_3));
_8 = 32527_u16;
RET = Adt19::Variant1 { fld0: 23004176460474505179726004711989746135_i128 };
place!(Field::<i128>(Variant(RET, 1), 0)) = -141389155206561483349087913442002422239_i128;
place!(Field::<i128>(Variant(RET, 1), 0)) = _8 as i128;
(*_3) = Adt19::Variant1 { fld0: 46922484880649758684758725701548889549_i128 };
_1 = [9223372036854775807_isize,16_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
place!(Field::<i128>(Variant((*_3), 1), 0)) = (-5644517536839831916_i64) as i128;
_9 = _8 as f64;
_9 = _6 - _6;
_5 = '\u{4742d}';
_10 = true;
_8 = 23741_u16 >> Field::<i128>(Variant((*_3), 1), 0);
_1 = [50_isize,9223372036854775807_isize,33_isize,85_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_9 = _6;
_5 = '\u{d4ead}';
Call(RET = fn14(_2, _2, _2, _1, _1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
place!(Field::<i128>(Variant(RET, 1), 0)) = 3769401301777595801_usize as i128;
_2 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),15_isize,9223372036854775807_isize,9223372036854775807_isize];
_1 = [116_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
(*_3) = Adt19::Variant1 { fld0: (-11303242249427273255795841457185128354_i128) };
place!(Field::<i128>(Variant((*_3), 1), 0)) = 163335491005910199635144189059954560702_i128;
_3 = core::ptr::addr_of_mut!((*_3));
_1 = [(-9223372036854775808_isize),49_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),76_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
SetDiscriminant((*_3), 0);
place!(Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1)).3 = 2664596320330446219_i64 << _8;
place!(Field::<(u64, u64, u64, i64)>(Variant(RET, 0), 1)).0 = 15287586508950186754_u64;
place!(Field::<bool>(Variant((*_3), 0), 0)) = Field::<(u64, u64, u64, i64)>(Variant(RET, 0), 1).3 != Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1).3;
RET = Adt19::Variant1 { fld0: 111204026213924986146326084289241203963_i128 };
place!(Field::<i128>(Variant((*_3), 1), 0)) = (-154545513127145312558865847945861265601_i128);
_10 = false;
_11 = !_10;
place!(Field::<i128>(Variant((*_3), 1), 0)) = -(-109126928115169917385213630801286102401_i128);
_11 = _8 == _8;
RET = Adt19::Variant1 { fld0: 66048760520052147062682702508449521756_i128 };
_1 = [(-120_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-43_isize),9223372036854775807_isize,9223372036854775807_isize,87_isize,98_isize];
Goto(bb6)
}
bb6 = {
_8 = _11 as u16;
place!(Field::<i128>(Variant((*_3), 1), 0)) = (-3_i8) as i128;
_8 = 188_u8 as u16;
_10 = !_11;
place!(Field::<i128>(Variant((*_3), 1), 0)) = !(-97596677043634050034923004830847012888_i128);
_13 = 8176_i16 << Field::<i128>(Variant((*_3), 1), 0);
place!(Field::<i128>(Variant((*_3), 1), 0)) = 17255612008707030292070878257486854713_i128;
place!(Field::<i128>(Variant((*_3), 1), 0)) = 110656633691125428810237983873818376258_i128;
_1 = _2;
_9 = -_6;
_11 = _10;
place!(Field::<i128>(Variant((*_3), 1), 0)) = -4501700638886656846341852549818523257_i128;
_13 = !14169_i16;
place!(Field::<i128>(Variant((*_3), 1), 0)) = (-125880855706959504851076373551216248498_i128) << _13;
_8 = 4802_u16;
_14 = [_8,_8,_8];
place!(Field::<i128>(Variant((*_3), 1), 0)) = (-8096643235703420433436985358833762103_i128);
Goto(bb7)
}
bb7 = {
_1 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,11_isize,45_isize,(-9223372036854775808_isize),63_isize,(-9223372036854775808_isize)];
_11 = _10;
_1 = [106_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-59_isize),(-104_isize),(-9223372036854775808_isize)];
SetDiscriminant((*_3), 0);
_9 = _6 * _6;
match _8 {
0 => bb5,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
4802 => bb14,
_ => bb13
}
}
bb8 = {
_8 = _11 as u16;
place!(Field::<i128>(Variant((*_3), 1), 0)) = (-3_i8) as i128;
_8 = 188_u8 as u16;
_10 = !_11;
place!(Field::<i128>(Variant((*_3), 1), 0)) = !(-97596677043634050034923004830847012888_i128);
_13 = 8176_i16 << Field::<i128>(Variant((*_3), 1), 0);
place!(Field::<i128>(Variant((*_3), 1), 0)) = 17255612008707030292070878257486854713_i128;
place!(Field::<i128>(Variant((*_3), 1), 0)) = 110656633691125428810237983873818376258_i128;
_1 = _2;
_9 = -_6;
_11 = _10;
place!(Field::<i128>(Variant((*_3), 1), 0)) = -4501700638886656846341852549818523257_i128;
_13 = !14169_i16;
place!(Field::<i128>(Variant((*_3), 1), 0)) = (-125880855706959504851076373551216248498_i128) << _13;
_8 = 4802_u16;
_14 = [_8,_8,_8];
place!(Field::<i128>(Variant((*_3), 1), 0)) = (-8096643235703420433436985358833762103_i128);
Goto(bb7)
}
bb9 = {
place!(Field::<i128>(Variant(RET, 1), 0)) = 3769401301777595801_usize as i128;
_2 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),15_isize,9223372036854775807_isize,9223372036854775807_isize];
_1 = [116_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
(*_3) = Adt19::Variant1 { fld0: (-11303242249427273255795841457185128354_i128) };
place!(Field::<i128>(Variant((*_3), 1), 0)) = 163335491005910199635144189059954560702_i128;
_3 = core::ptr::addr_of_mut!((*_3));
_1 = [(-9223372036854775808_isize),49_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),76_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
SetDiscriminant((*_3), 0);
place!(Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1)).3 = 2664596320330446219_i64 << _8;
place!(Field::<(u64, u64, u64, i64)>(Variant(RET, 0), 1)).0 = 15287586508950186754_u64;
place!(Field::<bool>(Variant((*_3), 0), 0)) = Field::<(u64, u64, u64, i64)>(Variant(RET, 0), 1).3 != Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1).3;
RET = Adt19::Variant1 { fld0: 111204026213924986146326084289241203963_i128 };
place!(Field::<i128>(Variant((*_3), 1), 0)) = (-154545513127145312558865847945861265601_i128);
_10 = false;
_11 = !_10;
place!(Field::<i128>(Variant((*_3), 1), 0)) = -(-109126928115169917385213630801286102401_i128);
_11 = _8 == _8;
RET = Adt19::Variant1 { fld0: 66048760520052147062682702508449521756_i128 };
_1 = [(-120_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-43_isize),9223372036854775807_isize,9223372036854775807_isize,87_isize,98_isize];
Goto(bb6)
}
bb10 = {
_1 = [(-92_isize),46_isize,(-46_isize),(-9223372036854775808_isize),(-106_isize),118_isize,9223372036854775807_isize,9223372036854775807_isize];
SetDiscriminant((*_3), 1);
(*_3) = Adt19::Variant1 { fld0: (-113286110768424195324689876420172058577_i128) };
(*_3) = Adt19::Variant1 { fld0: (-147505058194298945814695882126488136417_i128) };
_6 = 80_u8 as f64;
place!(Field::<i128>(Variant(RET, 1), 0)) = 50048733538576168895048720522916005539_i128;
place!(Field::<i128>(Variant(RET, 1), 0)) = _6 as i128;
_2 = [(-83_isize),(-9223372036854775808_isize),(-56_isize),(-9223372036854775808_isize),(-22_isize),14_isize,(-82_isize),(-9223372036854775808_isize)];
RET = Adt19::Variant1 { fld0: 30494780797346061383065441997172533057_i128 };
place!(Field::<i128>(Variant(RET, 1), 0)) = -44115167534811957011715655591005220641_i128;
(*_3) = Adt19::Variant1 { fld0: 88814507383165671720764803036106326242_i128 };
_3 = core::ptr::addr_of_mut!((*_3));
_8 = 32527_u16;
RET = Adt19::Variant1 { fld0: 23004176460474505179726004711989746135_i128 };
place!(Field::<i128>(Variant(RET, 1), 0)) = -141389155206561483349087913442002422239_i128;
place!(Field::<i128>(Variant(RET, 1), 0)) = _8 as i128;
(*_3) = Adt19::Variant1 { fld0: 46922484880649758684758725701548889549_i128 };
_1 = [9223372036854775807_isize,16_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
place!(Field::<i128>(Variant((*_3), 1), 0)) = (-5644517536839831916_i64) as i128;
_9 = _8 as f64;
_9 = _6 - _6;
_5 = '\u{4742d}';
_10 = true;
_8 = 23741_u16 >> Field::<i128>(Variant((*_3), 1), 0);
_1 = [50_isize,9223372036854775807_isize,33_isize,85_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_9 = _6;
_5 = '\u{d4ead}';
Call(RET = fn14(_2, _2, _2, _1, _1), ReturnTo(bb5), UnwindUnreachable())
}
bb11 = {
place!(Field::<i128>(Variant(RET, 1), 0)) = 62214_u16 as i128;
_3 = core::ptr::addr_of_mut!((*_3));
_3 = core::ptr::addr_of_mut!((*_3));
Call(place!(Field::<i128>(Variant((*_3), 1), 0)) = fn10(_1, _1, _1, _2, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
RET = Adt19::Variant1 { fld0: 125809565279724576212562751336770329876_i128 };
place!(Field::<i128>(Variant((*_3), 1), 0)) = 8436384891656131651450397492951331110_i128;
_3 = core::ptr::addr_of_mut!((*_3));
place!(Field::<i128>(Variant(RET, 1), 0)) = 103923364369957644504467702538544099252_i128 ^ (-17251641119388483753655479912383326579_i128);
Goto(bb3)
}
bb13 = {
place!(Field::<(u64, u64, u64, i64)>(Variant(RET, 0), 1)) = (13889833736832345569_u64, 905815362858663000_u64, 2817182631882405821_u64, (-3687924576052790706_i64));
place!(Field::<u32>(Variant((*_3), 0), 2)) = 2468972002_u32;
place!(Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1)).0 = 144861277638041526569711826639181351898_u128 as u64;
place!(Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1)).0 = !Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1).2;
place!(Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1)) = (1680831609518165654_u64, 7729045282731597224_u64, 12648259803853619105_u64, 3054781913333760462_i64);
_5 = '\u{efba9}';
_2 = [39_isize,(-9223372036854775808_isize),58_isize,(-9223372036854775808_isize),18_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
place!(Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1)).3 = -(-911527611365113231_i64);
place!(Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1)).2 = Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1).1;
place!(Field::<u32>(Variant(RET, 0), 2)) = 2601151897_u32;
place!(Field::<bool>(Variant((*_3), 0), 0)) = false | false;
place!(Field::<(u64, u64, u64, i64)>(Variant(RET, 0), 1)).0 = !Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1).1;
place!(Field::<u32>(Variant(RET, 0), 2)) = 826978166_u32;
_6 = 7876137224619775725_usize as f64;
place!(Field::<bool>(Variant((*_3), 0), 0)) = !true;
place!(Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1)).0 = Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1).1 % Field::<(u64, u64, u64, i64)>(Variant(RET, 0), 1).1;
place!(Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1)).0 = !Field::<(u64, u64, u64, i64)>(Variant(RET, 0), 1).2;
SetDiscriminant(RET, 1);
RET = Adt19::Variant1 { fld0: (-77680958985028682802844269155339956418_i128) };
place!(Field::<i128>(Variant(RET, 1), 0)) = 15341294022667352476_u64 as i128;
RET = Adt19::Variant1 { fld0: (-153032746768369492894892535024254413318_i128) };
_2 = [(-9223372036854775808_isize),38_isize,108_isize,(-47_isize),9223372036854775807_isize,(-57_isize),(-9223372036854775808_isize),13_isize];
Goto(bb2)
}
bb14 = {
place!(Field::<(u64, u64, u64, i64)>(Variant(RET, 0), 1)).0 = !10731295936005513789_u64;
place!(Field::<(u64, u64, u64, i64)>(Variant(RET, 0), 1)).2 = 5390184642400819248_i64 as u64;
_15.fld0 = [_8,_8,_8,_8,_8,_8];
place!(Field::<(u64, u64, u64, i64)>(Variant(RET, 0), 1)).2 = !Field::<(u64, u64, u64, i64)>(Variant(RET, 0), 1).0;
place!(Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1)).3 = 1108653692_u32 as i64;
place!(Field::<(u64, u64, u64, i64)>(Variant(RET, 0), 1)).2 = !Field::<(u64, u64, u64, i64)>(Variant((*_3), 0), 1).0;
_3 = core::ptr::addr_of_mut!(RET);
_15.fld6 = _5 as i64;
_15.fld1.3 = !_10;
_11 = _15.fld1.3;
RET = Adt19::Variant1 { fld0: (-137809582520333167327657310712924250165_i128) };
RET = Adt19::Variant1 { fld0: 65850264520166019008129984034217609216_i128 };
_15.fld2.3 = !_15.fld6;
_15.fld4.1 = [1428023242_i32,66293261_i32];
_15.fld5 = [185_u8,77_u8,234_u8,211_u8,49_u8,204_u8,101_u8];
Call(_11 = fn15(_14, _1, _8), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
(*_3) = Adt19::Variant1 { fld0: 126428570899283222664558581447131408742_i128 };
_11 = _5 < _5;
_15.fld7 = 2636455945_u32;
_9 = 9901323881221799739_u64 as f64;
_18 = 9223372036854775807_isize + (-52_isize);
_15.fld1.3 = !_10;
_15.fld1.0 = 7_usize;
Goto(bb16)
}
bb16 = {
_10 = _11 <= _11;
_15.fld2.2 = !11210367745528527971_u64;
place!(Field::<i128>(Variant(RET, 1), 0)) = 1829225209_i32 as i128;
_2 = [_18,_18,_18,_18,_18,_18,_18,_18];
_9 = -_6;
_15.fld1 = (7262918425328542223_usize, _8, (*_3), _10);
_15.fld2 = (17593528748367525042_u64, 17368621241672092596_u64, 17087067791686617216_u64, _15.fld6);
Goto(bb17)
}
bb17 = {
Call(_23 = dump_var(9_usize, 13_usize, Move(_13), 11_usize, Move(_11), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: [isize; 8],mut _2: [isize; 8],mut _3: [isize; 8],mut _4: [isize; 8],mut _5: [isize; 8]) -> i128 {
mir! {
type RET = i128;
let _6: (u64, u64, u64, i64);
let _7: [usize; 4];
let _8: i8;
let _9: u8;
let _10: [i16; 8];
let _11: char;
let _12: *const [u8; 2];
let _13: f64;
let _14: (*const usize, &'static char);
let _15: [u64; 7];
let _16: &'static isize;
let _17: [i32; 2];
let _18: u128;
let _19: *mut u8;
let _20: char;
let _21: f64;
let _22: ();
let _23: ();
{
_1 = [64_isize,9223372036854775807_isize,81_isize,(-119_isize),(-79_isize),123_isize,12_isize,(-9223372036854775808_isize)];
_6.1 = !14636091618112971580_u64;
_6.1 = true as u64;
RET = 137413383363269305176171060454465516944_u128 as i128;
_2 = [(-9223372036854775808_isize),(-53_isize),9223372036854775807_isize,9223372036854775807_isize,(-49_isize),9223372036854775807_isize,100_isize,(-114_isize)];
_6.3 = 498923688415278514_i64;
_6.0 = _6.1;
_7 = [3348295524219979672_usize,324772142537948448_usize,11612590320746803274_usize,8058728476321053444_usize];
_2 = _5;
_6.2 = _6.0;
_6.3 = RET as i64;
_6.0 = !_6.1;
_6.3 = -(-5720679391869959852_i64);
_3 = [(-67_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_6.1 = _6.0 + _6.2;
_3 = _2;
_6 = (8874193593088488993_u64, 16219442808901011067_u64, 4668087070254656570_u64, (-6993638805337359748_i64));
_6 = (17737631986659335433_u64, 17762109538353694741_u64, 10990609196737688422_u64, (-4429930442827527467_i64));
RET = 24982_i16 as i128;
_6.1 = _6.0;
Call(_4 = fn11(_3, _5, _3, _5, _6.1, _3, _5, _6, _2, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _5;
_5 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
RET = 2068791475_i32 as i128;
RET = 163191752765714070880423138036635450868_i128;
_5 = _1;
RET = 116431814827027400507263474639577950807_i128 - 126595451481357701056525612047356767079_i128;
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,101_isize,(-69_isize)];
match _6.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
17737631986659335433 => bb7,
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
_6.2 = _6.1;
_6.1 = false as u64;
RET = (-10266570313419018492346411162134530460_i128);
_9 = 64_u8 >> _6.2;
_6.1 = _6.0 | _6.2;
_2 = _4;
_2 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),100_isize,9223372036854775807_isize];
_5 = [9223372036854775807_isize,9223372036854775807_isize,(-51_isize),15_isize,(-38_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_3 = [(-19_isize),36_isize,(-104_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),96_isize];
_6 = (14027591875094182534_u64, 10482597432841788570_u64, 771114793674056034_u64, (-586781474431790195_i64));
_8 = (-99_i8) ^ 3_i8;
_11 = '\u{6c573}';
_3 = [(-9223372036854775808_isize),58_isize,41_isize,(-9223372036854775808_isize),53_isize,9223372036854775807_isize,(-104_isize),9223372036854775807_isize];
_11 = '\u{63631}';
RET = !(-75598273202130945879492529108361937941_i128);
_6.2 = _6.1;
match _6.0 {
0 => bb8,
1 => bb9,
2 => bb10,
14027591875094182534 => bb12,
_ => bb11
}
}
bb8 = {
Return()
}
bb9 = {
_3 = _5;
_5 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
RET = 2068791475_i32 as i128;
RET = 163191752765714070880423138036635450868_i128;
_5 = _1;
RET = 116431814827027400507263474639577950807_i128 - 126595451481357701056525612047356767079_i128;
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,101_isize,(-69_isize)];
match _6.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
17737631986659335433 => bb7,
_ => bb6
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_13 = _6.3 as f64;
_6.3 = !(-8870378525815608455_i64);
_2 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-109_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_11 = '\u{7ebf3}';
_6.1 = RET as u64;
_9 = 219_u8 - 34_u8;
_6.3 = 6059466640257612179_i64;
_6.0 = _6.2;
_8 = (-120_i8) << _6.2;
_15 = [_6.1,_6.2,_6.1,_6.1,_6.2,_6.2,_6.2];
_8 = 16_i8 >> _6.0;
_6.2 = (-127_isize) as u64;
_6 = (12881798419301166287_u64, 15948467221500197528_u64, 5564347726915345351_u64, 1856178036286444622_i64);
_18 = !322344400166777622240857496792023925546_u128;
Call(_6 = fn12(_11, _5), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_17 = [492707222_i32,(-940096979_i32)];
_2 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_2 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-12_isize),(-9223372036854775808_isize),(-32_isize),125_isize,9223372036854775807_isize];
RET = 6_usize as i128;
_6.3 = (-1011753925537696399_i64);
_18 = 312835527937699037657568550121651457668_u128 - 314823126966872966455086496416591978772_u128;
_4 = _5;
_10 = [3068_i16,32544_i16,11491_i16,4109_i16,(-11813_i16),8182_i16,(-9537_i16),17664_i16];
_1 = [30_isize,(-125_isize),93_isize,9223372036854775807_isize,101_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = -93386424105608293465576071317657618561_i128;
_19 = core::ptr::addr_of_mut!(_9);
Goto(bb14)
}
bb14 = {
_6 = (17781429691544990524_u64, 17056814053397040853_u64, 11602511873741767141_u64, (-3564220955202976495_i64));
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(10_usize, 15_usize, Move(_15), 18_usize, Move(_18), 11_usize, Move(_11), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_22 = dump_var(10_usize, 7_usize, Move(_7), 6_usize, Move(_6), 5_usize, Move(_5), 23_usize, _23), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [isize; 8],mut _2: [isize; 8],mut _3: [isize; 8],mut _4: [isize; 8],mut _5: u64,mut _6: [isize; 8],mut _7: [isize; 8],mut _8: (u64, u64, u64, i64),mut _9: [isize; 8],mut _10: [isize; 8]) -> [isize; 8] {
mir! {
type RET = [isize; 8];
let _11: Adt50;
let _12: isize;
let _13: &'static u32;
let _14: &'static *const ([bool; 1], *mut *mut Adt19);
let _15: [u16; 6];
let _16: char;
let _17: *const [bool; 1];
let _18: ();
let _19: ();
{
RET = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-73_isize),(-9223372036854775808_isize)];
_4 = _6;
_2 = _1;
_8 = (_5, _5, _5, 43175387015397645_i64);
_8.0 = 31482_i16 as u64;
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
Goto(bb1)
}
bb1 = {
_8.2 = _5 & _5;
_10 = [(-7_isize),9223372036854775807_isize,9223372036854775807_isize,(-103_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_1 = [(-94_isize),(-120_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_6 = [9223372036854775807_isize,(-9223372036854775808_isize),(-107_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_4 = _3;
_8.3 = 5897149419451612315_i64;
_9 = [4_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),114_isize,(-9223372036854775808_isize),43_isize,(-9223372036854775808_isize)];
_10 = [9223372036854775807_isize,9223372036854775807_isize,66_isize,(-9223372036854775808_isize),(-117_isize),51_isize,55_isize,(-9223372036854775808_isize)];
_3 = _2;
_10 = _3;
Goto(bb2)
}
bb2 = {
_3 = _10;
_8.2 = !_8.1;
_7 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),74_isize,9223372036854775807_isize];
_8.2 = _8.0;
_6 = _1;
_8.2 = !_8.1;
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize),30_isize,(-29_isize),(-9223372036854775808_isize),61_isize,(-60_isize),75_isize];
_8 = (_5, _5, _5, 3197412002395981100_i64);
_8.2 = _8.0;
RET = [(-9223372036854775808_isize),81_isize,115_isize,(-9223372036854775808_isize),46_isize,(-105_isize),(-7_isize),9223372036854775807_isize];
RET = [(-115_isize),9223372036854775807_isize,27_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-74_isize),(-85_isize)];
_4 = _3;
RET = [(-9223372036854775808_isize),122_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_15 = [63605_u16,42920_u16,54030_u16,18963_u16,54833_u16,14662_u16];
_5 = (-14078_i16) as u64;
RET = [(-93_isize),(-9223372036854775808_isize),(-85_isize),(-109_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_2 = [9223372036854775807_isize,110_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_2 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),92_isize,(-3_isize),9223372036854775807_isize];
_5 = _8.1 & _8.2;
Goto(bb3)
}
bb3 = {
_16 = '\u{35bb7}';
_15 = [27031_u16,51443_u16,29416_u16,11719_u16,48173_u16,8895_u16];
_10 = [9223372036854775807_isize,9223372036854775807_isize,113_isize,(-9223372036854775808_isize),8_isize,9223372036854775807_isize,(-79_isize),49_isize];
_1 = [(-9223372036854775808_isize),9223372036854775807_isize,(-30_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-100_isize),9223372036854775807_isize];
_9 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),100_isize];
_6 = _4;
_15 = [34633_u16,8083_u16,16812_u16,12111_u16,54961_u16,53647_u16];
_7 = [9223372036854775807_isize,9223372036854775807_isize,16_isize,9223372036854775807_isize,(-100_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_8.3 = 7877723555450865757_i64;
_8.3 = (-102_isize) as i64;
_10 = [(-9223372036854775808_isize),104_isize,(-63_isize),9223372036854775807_isize,(-9223372036854775808_isize),9_isize,9223372036854775807_isize,(-114_isize)];
RET = [125_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,61_isize,77_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = _10;
_12 = -(-9223372036854775808_isize);
match _8.1 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
17737631986659335433 => bb12,
_ => bb11
}
}
bb4 = {
_3 = _10;
_8.2 = !_8.1;
_7 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),74_isize,9223372036854775807_isize];
_8.2 = _8.0;
_6 = _1;
_8.2 = !_8.1;
RET = [(-9223372036854775808_isize),(-9223372036854775808_isize),30_isize,(-29_isize),(-9223372036854775808_isize),61_isize,(-60_isize),75_isize];
_8 = (_5, _5, _5, 3197412002395981100_i64);
_8.2 = _8.0;
RET = [(-9223372036854775808_isize),81_isize,115_isize,(-9223372036854775808_isize),46_isize,(-105_isize),(-7_isize),9223372036854775807_isize];
RET = [(-115_isize),9223372036854775807_isize,27_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-74_isize),(-85_isize)];
_4 = _3;
RET = [(-9223372036854775808_isize),122_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_15 = [63605_u16,42920_u16,54030_u16,18963_u16,54833_u16,14662_u16];
_5 = (-14078_i16) as u64;
RET = [(-93_isize),(-9223372036854775808_isize),(-85_isize),(-109_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_2 = [9223372036854775807_isize,110_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_2 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),92_isize,(-3_isize),9223372036854775807_isize];
_5 = _8.1 & _8.2;
Goto(bb3)
}
bb5 = {
_8.2 = _5 & _5;
_10 = [(-7_isize),9223372036854775807_isize,9223372036854775807_isize,(-103_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_1 = [(-94_isize),(-120_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_6 = [9223372036854775807_isize,(-9223372036854775808_isize),(-107_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_4 = _3;
_8.3 = 5897149419451612315_i64;
_9 = [4_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),114_isize,(-9223372036854775808_isize),43_isize,(-9223372036854775808_isize)];
_10 = [9223372036854775807_isize,9223372036854775807_isize,66_isize,(-9223372036854775808_isize),(-117_isize),51_isize,55_isize,(-9223372036854775808_isize)];
_3 = _2;
_10 = _3;
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
Return()
}
bb12 = {
RET = _1;
_8.3 = (-1750714373732845416_i64);
_2 = _10;
RET = _7;
_16 = '\u{ad606}';
RET = _4;
RET = [_12,_12,_12,_12,_12,_12,_12,_12];
match _8.1 {
0 => bb1,
1 => bb4,
2 => bb9,
3 => bb13,
17737631986659335433 => bb15,
_ => bb14
}
}
bb13 = {
_16 = '\u{35bb7}';
_15 = [27031_u16,51443_u16,29416_u16,11719_u16,48173_u16,8895_u16];
_10 = [9223372036854775807_isize,9223372036854775807_isize,113_isize,(-9223372036854775808_isize),8_isize,9223372036854775807_isize,(-79_isize),49_isize];
_1 = [(-9223372036854775808_isize),9223372036854775807_isize,(-30_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-100_isize),9223372036854775807_isize];
_9 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),100_isize];
_6 = _4;
_15 = [34633_u16,8083_u16,16812_u16,12111_u16,54961_u16,53647_u16];
_7 = [9223372036854775807_isize,9223372036854775807_isize,16_isize,9223372036854775807_isize,(-100_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_8.3 = 7877723555450865757_i64;
_8.3 = (-102_isize) as i64;
_10 = [(-9223372036854775808_isize),104_isize,(-63_isize),9223372036854775807_isize,(-9223372036854775808_isize),9_isize,9223372036854775807_isize,(-114_isize)];
RET = [125_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,61_isize,77_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = _10;
_12 = -(-9223372036854775808_isize);
match _8.1 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
17737631986659335433 => bb12,
_ => bb11
}
}
bb14 = {
Return()
}
bb15 = {
_8.1 = !_8.2;
Goto(bb16)
}
bb16 = {
Call(_18 = dump_var(11_usize, 1_usize, Move(_1), 9_usize, Move(_9), 4_usize, Move(_4), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_18 = dump_var(11_usize, 16_usize, Move(_16), 5_usize, Move(_5), 19_usize, _19, 19_usize, _19), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: char,mut _2: [isize; 8]) -> (u64, u64, u64, i64) {
mir! {
type RET = (u64, u64, u64, i64);
let _3: f32;
let _4: *const [u32; 3];
let _5: bool;
let _6: i64;
let _7: Adt24;
let _8: &'static &'static u16;
let _9: ();
let _10: ();
{
RET.3 = 7851607519098017127_i64;
RET = (3612119027010237676_u64, 10028365582338459144_u64, 12964590264690402604_u64, 332216582657604077_i64);
RET = (16496270770417371551_u64, 535057560956103817_u64, 11590195418294936460_u64, 5309557143604612033_i64);
RET.1 = !RET.2;
RET.3 = -2043024023135098100_i64;
_1 = '\u{9d180}';
RET = (1810750014318299646_u64, 14161258228562334606_u64, 5565195132986022556_u64, (-8843631138353412578_i64));
RET.1 = RET.2 % RET.2;
RET.2 = (-108_isize) as u64;
RET.3 = (-2618499172330012660_i64) >> RET.1;
RET.2 = 9223372036854775807_isize as u64;
RET.2 = RET.0 | RET.0;
_3 = 60299_u16 as f32;
_1 = '\u{5909c}';
RET.1 = !RET.2;
RET.1 = RET.3 as u64;
RET.0 = !RET.2;
_1 = '\u{c2834}';
RET = (5969344492523273517_u64, 14357470220989515496_u64, 7091742308535331022_u64, 7764338100152674724_i64);
RET.3 = 988599425073274162_i64;
RET.2 = RET.1;
RET = (17697944820570104099_u64, 12909696263823187079_u64, 1941909437246326713_u64, (-6687199652746980216_i64));
_3 = 14680_u16 as f32;
RET = (2572749995719194150_u64, 2198614604625171480_u64, 6538417470027094354_u64, (-8999104047971733443_i64));
RET.0 = RET.2 << RET.2;
RET.1 = RET.2;
RET.3 = (-5432240723166637083_i64);
match RET.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463457942366708601574373 => bb8,
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
RET.0 = !RET.1;
RET.2 = RET.1 & RET.0;
RET.1 = 29187522336387762032940360232190510147_i128 as u64;
RET.2 = !RET.0;
RET.1 = !RET.2;
_2 = [27_isize,117_isize,121_isize,6_isize,(-9223372036854775808_isize),36_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET.3 = -8868231346967067504_i64;
Call(RET.0 = fn13(_2, _2, _2, RET.2, _2, _2, _2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_1 = '\u{344dd}';
RET.0 = RET.2 >> RET.1;
_3 = 83_i8 as f32;
_3 = RET.0 as f32;
_7 = Adt24::Variant1 { fld0: 41190_u16,fld1: RET.0 };
RET.0 = RET.2 >> RET.1;
_6 = RET.3 | RET.3;
_3 = 4050808575_u32 as f32;
_1 = '\u{2580a}';
place!(Field::<u16>(Variant(_7, 1), 0)) = 40394_u16;
place!(Field::<u64>(Variant(_7, 1), 1)) = RET.1 - RET.2;
RET.1 = RET.2 << Field::<u64>(Variant(_7, 1), 1);
_7 = Adt24::Variant0 { fld0: 107_u8,fld1: _1,fld2: (-25899_i16) };
_1 = Field::<char>(Variant(_7, 0), 1);
place!(Field::<u8>(Variant(_7, 0), 0)) = Field::<char>(Variant(_7, 0), 1) as u8;
Goto(bb10)
}
bb10 = {
Call(_9 = dump_var(12_usize, 2_usize, Move(_2), 10_usize, _10, 10_usize, _10, 10_usize, _10), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: [isize; 8],mut _2: [isize; 8],mut _3: [isize; 8],mut _4: u64,mut _5: [isize; 8],mut _6: [isize; 8],mut _7: [isize; 8]) -> u64 {
mir! {
type RET = u64;
let _8: isize;
let _9: i128;
let _10: *mut *mut Adt19;
let _11: [u8; 7];
let _12: char;
let _13: char;
let _14: i64;
let _15: *const Adt24;
let _16: f32;
let _17: Adt38;
let _18: isize;
let _19: i32;
let _20: (&'static char,);
let _21: *const Adt24;
let _22: char;
let _23: i64;
let _24: *const usize;
let _25: (u64, [u64; 7], (u64, u64, u64, i64));
let _26: i32;
let _27: (*const u64,);
let _28: Adt38;
let _29: isize;
let _30: f32;
let _31: i32;
let _32: *const ([i8; 7], [i32; 2]);
let _33: char;
let _34: f64;
let _35: &'static i8;
let _36: f32;
let _37: u8;
let _38: [i32; 2];
let _39: ();
let _40: ();
{
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-92_isize),(-128_isize),(-9223372036854775808_isize),(-89_isize),(-31_isize),(-9223372036854775808_isize)];
_2 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-78_isize),(-82_isize),(-9223372036854775808_isize),(-44_isize),49_isize,9223372036854775807_isize];
_8 = -9223372036854775807_isize;
RET = _4;
_5 = [_8,_8,_8,_8,_8,_8,_8,_8];
_3 = [_8,_8,_8,_8,_8,_8,_8,_8];
RET = _4 * _4;
_5 = [_8,_8,_8,_8,_8,_8,_8,_8];
RET = !_4;
_7 = [_8,_8,_8,_8,_8,_8,_8,_8];
_4 = !RET;
_11 = [222_u8,230_u8,240_u8,243_u8,77_u8,67_u8,221_u8];
_9 = !(-71118369343843355525746305315401445857_i128);
_12 = '\u{48689}';
RET = !_4;
_4 = RET;
_6 = _2;
_2 = [_8,_8,_8,_8,_8,_8,_8,_8];
RET = _4;
RET = _4;
Goto(bb1)
}
bb1 = {
_8 = 9223372036854775807_isize;
_12 = '\u{40526}';
RET = _4;
_2 = [_8,_8,_8,_8,_8,_8,_8,_8];
_14 = !(-8772288085088965360_i64);
_11 = [223_u8,33_u8,155_u8,106_u8,85_u8,133_u8,48_u8];
_9 = -41400596788983868577881661351828353421_i128;
_3 = [_8,_8,_8,_8,_8,_8,_8,_8];
_12 = '\u{104441}';
RET = _4 | _4;
_3 = _6;
_11 = [93_u8,104_u8,88_u8,178_u8,23_u8,54_u8,200_u8];
_5 = [_8,_8,_8,_8,_8,_8,_8,_8];
_8 = 9223372036854775807_isize;
_1 = [_8,_8,_8,_8,_8,_8,_8,_8];
_7 = [_8,_8,_8,_8,_8,_8,_8,_8];
RET = _4 ^ _4;
_3 = _6;
_12 = '\u{e01b}';
_7 = [_8,_8,_8,_8,_8,_8,_8,_8];
RET = !_4;
RET = _4 - _4;
_7 = [_8,_8,_8,_8,_8,_8,_8,_8];
_6 = [_8,_8,_8,_8,_8,_8,_8,_8];
_8 = (-9223372036854775808_isize) | (-124_isize);
Goto(bb2)
}
bb2 = {
RET = _4;
_7 = _5;
_11 = [90_u8,161_u8,194_u8,1_u8,72_u8,141_u8,49_u8];
RET = !_4;
Call(_14 = core::intrinsics::transmute(_4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_17.fld2 = _9;
_17.fld5.0 = [1021720352_i32,(-1394707199_i32)];
_12 = '\u{186e4}';
_17.fld0 = _17.fld2 as f32;
_13 = _12;
_11 = [188_u8,153_u8,18_u8,89_u8,130_u8,98_u8,100_u8];
_19 = (-1034346308_i32) * (-685320349_i32);
_3 = [_8,_8,_8,_8,_8,_8,_8,_8];
_9 = !_17.fld2;
_8 = 9223372036854775807_isize;
_16 = _17.fld0;
_11 = [50_u8,98_u8,143_u8,77_u8,184_u8,203_u8,9_u8];
_18 = 86_u8 as isize;
_1 = [_18,_18,_18,_8,_18,_18,_8,_8];
_18 = -_8;
_1 = [_18,_18,_8,_8,_8,_18,_8,_18];
_17.fld5.1 = 1494052620_u32 & 3389000864_u32;
match _8 {
0 => bb1,
1 => bb2,
9223372036854775807 => bb5,
_ => bb4
}
}
bb4 = {
RET = _4;
_7 = _5;
_11 = [90_u8,161_u8,194_u8,1_u8,72_u8,141_u8,49_u8];
RET = !_4;
Call(_14 = core::intrinsics::transmute(_4), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_17.fld4 = 4094_i16 >> _19;
_16 = _18 as f32;
_17.fld6 = _14 << _19;
_14 = _17.fld6;
_16 = _17.fld0;
_5 = [_8,_8,_18,_8,_18,_8,_18,_18];
_17.fld6 = false as i64;
_22 = _12;
_8 = _18 - _18;
_17.fld3 = 8_i8 | 36_i8;
_17.fld3 = 121_i8 << _17.fld2;
_17.fld3 = -1_i8;
_20.0 = &_12;
_1 = [_18,_8,_8,_8,_8,_8,_8,_18];
_5 = [_8,_8,_8,_8,_8,_8,_18,_8];
_4 = !RET;
_18 = _8;
_18 = _8;
Goto(bb6)
}
bb6 = {
_11 = [37_u8,56_u8,185_u8,246_u8,105_u8,87_u8,71_u8];
_12 = _22;
_12 = _22;
_23 = _14 ^ _14;
_25.2.2 = !RET;
_25.2.2 = !RET;
_25.2.1 = RET;
_8 = -_18;
_17.fld4 = 6488_i16;
_20.0 = &_22;
_25.2.2 = _17.fld5.1 as u64;
_17.fld6 = _14 | _14;
_25.1 = [_25.2.2,_4,_25.2.1,_25.2.2,RET,_25.2.1,_25.2.1];
_25.2.0 = _25.2.1;
_25.2.1 = !_25.2.0;
_12 = _22;
_18 = _8 | _8;
_9 = _17.fld5.1 as i128;
RET = _25.2.0 ^ _4;
_17.fld3 = (-16_i8) >> _25.2.0;
_17.fld5.0 = [_19,_19];
_4 = 34896_u16 as u64;
_17.fld1 = _13;
RET = _25.2.2 | _25.2.2;
_9 = 3470186588637471460_usize as i128;
_8 = _18 << _17.fld6;
_17.fld5.0 = [_19,_19];
_25.0 = _17.fld3 as u64;
Call(_3 = core::intrinsics::transmute(_1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_14 = 118_u8 as i64;
_25.1 = [_25.0,_25.0,_25.2.0,RET,RET,_25.2.0,_25.0];
_25.2.0 = _4;
Goto(bb8)
}
bb8 = {
_28.fld5 = _17.fld5;
_28.fld3 = -_17.fld3;
_17.fld1 = _13;
Goto(bb9)
}
bb9 = {
_28.fld6 = !_17.fld6;
_18 = -_8;
_1 = [_8,_18,_18,_18,_8,_8,_18,_18];
RET = _25.2.0;
_26 = !_19;
_28.fld4 = -_17.fld4;
_25.2.0 = !_25.2.1;
_16 = _17.fld0 * _17.fld0;
_2 = _5;
_28.fld4 = _17.fld4 << _18;
_17.fld0 = -_16;
_28.fld2 = _17.fld2 ^ _17.fld2;
_27.0 = core::ptr::addr_of!(_25.0);
_25.2.0 = RET & _25.0;
_29 = _8 * _8;
_11 = [92_u8,15_u8,188_u8,102_u8,229_u8,1_u8,91_u8];
_25.2.0 = _28.fld2 as u64;
RET = _25.0 * _25.2.2;
_16 = _18 as f32;
_5 = [_18,_29,_18,_29,_29,_29,_29,_29];
_20.0 = &_28.fld1;
_28.fld0 = -_16;
_7 = [_18,_8,_29,_8,_8,_29,_29,_29];
_17.fld6 = _23;
_12 = _22;
match _17.fld4 {
0 => bb1,
1 => bb10,
6488 => bb12,
_ => bb11
}
}
bb10 = {
_28.fld5 = _17.fld5;
_28.fld3 = -_17.fld3;
_17.fld1 = _13;
Goto(bb9)
}
bb11 = {
_17.fld2 = _9;
_17.fld5.0 = [1021720352_i32,(-1394707199_i32)];
_12 = '\u{186e4}';
_17.fld0 = _17.fld2 as f32;
_13 = _12;
_11 = [188_u8,153_u8,18_u8,89_u8,130_u8,98_u8,100_u8];
_19 = (-1034346308_i32) * (-685320349_i32);
_3 = [_8,_8,_8,_8,_8,_8,_8,_8];
_9 = !_17.fld2;
_8 = 9223372036854775807_isize;
_16 = _17.fld0;
_11 = [50_u8,98_u8,143_u8,77_u8,184_u8,203_u8,9_u8];
_18 = 86_u8 as isize;
_1 = [_18,_18,_18,_8,_18,_18,_8,_8];
_18 = -_8;
_1 = [_18,_18,_8,_8,_8,_18,_8,_18];
_17.fld5.1 = 1494052620_u32 & 3389000864_u32;
match _8 {
0 => bb1,
1 => bb2,
9223372036854775807 => bb5,
_ => bb4
}
}
bb12 = {
RET = _28.fld0 as u64;
_17 = Adt38 { fld0: _28.fld0,fld1: _22,fld2: _9,fld3: _28.fld3,fld4: _28.fld4,fld5: _28.fld5,fld6: _28.fld6 };
_8 = _18;
_28.fld0 = _16;
_2 = [_29,_29,_18,_8,_18,_18,_18,_18];
_25.1 = [_4,_25.2.1,RET,RET,_25.2.0,_25.0,_25.0];
_28.fld6 = _17.fld6;
_31 = _18 as i32;
_28 = Adt38 { fld0: _16,fld1: _22,fld2: _17.fld2,fld3: _17.fld3,fld4: _17.fld4,fld5: _17.fld5,fld6: _23 };
_17.fld4 = !_28.fld4;
_27.0 = core::ptr::addr_of!(_25.2.1);
_25.1 = [_25.2.2,RET,RET,RET,_25.2.1,_25.2.1,RET];
_26 = _8 as i32;
_37 = 187_u8;
_4 = _31 as u64;
_25.2 = (_4, RET, RET, _28.fld6);
_6 = [_29,_29,_29,_29,_18,_29,_8,_29];
Goto(bb13)
}
bb13 = {
Call(_39 = dump_var(13_usize, 29_usize, Move(_29), 2_usize, Move(_2), 12_usize, Move(_12), 14_usize, Move(_14)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_39 = dump_var(13_usize, 22_usize, Move(_22), 3_usize, Move(_3), 5_usize, Move(_5), 4_usize, Move(_4)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_39 = dump_var(13_usize, 26_usize, Move(_26), 19_usize, Move(_19), 31_usize, Move(_31), 40_usize, _40), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: [isize; 8],mut _2: [isize; 8],mut _3: [isize; 8],mut _4: [isize; 8],mut _5: [isize; 8]) -> Adt19 {
mir! {
type RET = Adt19;
let _6: ([i32; 2], u32);
let _7: &'static i8;
let _8: u128;
let _9: (*const u64,);
let _10: (&'static char,);
let _11: u64;
let _12: &'static *const ([bool; 1], *mut *mut Adt19);
let _13: [usize; 3];
let _14: *mut Adt19;
let _15: f64;
let _16: isize;
let _17: u16;
let _18: [i16; 8];
let _19: char;
let _20: &'static i8;
let _21: &'static [i64; 8];
let _22: ();
let _23: ();
{
_5 = [(-56_isize),9223372036854775807_isize,27_isize,24_isize,(-9223372036854775808_isize),(-38_isize),9223372036854775807_isize,(-41_isize)];
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-33_isize)];
RET = Adt19::Variant1 { fld0: 156104186443393336239708223597151665647_i128 };
_2 = _3;
_6.0 = [(-1595927559_i32),(-1824215431_i32)];
_4 = [(-9223372036854775808_isize),90_isize,(-9223372036854775808_isize),86_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_6.1 = !265901110_u32;
_8 = !146252348651162999802324207656159757500_u128;
_6.0 = [459504797_i32,(-76445659_i32)];
_6.1 = !2592565357_u32;
_1 = [9223372036854775807_isize,(-9223372036854775808_isize),(-14_isize),(-9223372036854775808_isize),28_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),74_isize];
place!(Field::<i128>(Variant(RET, 1), 0)) = 180_u8 as i128;
_6.1 = !3333366582_u32;
place!(Field::<i128>(Variant(RET, 1), 0)) = 25819_u16 as i128;
_5 = [94_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
RET = Adt19::Variant1 { fld0: (-93994925917162626616252204812571721826_i128) };
place!(Field::<i128>(Variant(RET, 1), 0)) = 156030077132316788787692604072830709148_i128 | (-162824669283091405871443391867572806233_i128);
_6.1 = 879731437_u32;
_3 = [(-9223372036854775808_isize),9223372036854775807_isize,(-59_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,101_isize,(-9223372036854775808_isize)];
SetDiscriminant(RET, 1);
_4 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-91_isize),(-9223372036854775808_isize),9223372036854775807_isize];
Goto(bb1)
}
bb1 = {
_9.0 = core::ptr::addr_of!(_11);
_11 = 15141604908883300906_u64 & 9030269753312901999_u64;
_6.0 = [(-119617777_i32),1124475511_i32];
_6.1 = 1585141468_u32;
RET = Adt19::Variant1 { fld0: 157615037804816778253509874839416238453_i128 };
place!(Field::<i128>(Variant(RET, 1), 0)) = 123862723274962118042555967120050867318_i128;
_11 = !9906106171745171017_u64;
place!(Field::<i128>(Variant(RET, 1), 0)) = 153071251174971237034095579532743323915_i128;
_4 = _2;
RET = Adt19::Variant1 { fld0: (-95086339532020007958235513747075041678_i128) };
_6.0 = [(-734266162_i32),692462590_i32];
place!(Field::<i128>(Variant(RET, 1), 0)) = 16_u8 as i128;
place!(Field::<i128>(Variant(RET, 1), 0)) = 157463252278776667272956449613132125867_i128 & (-65179332706734756591798895379682790803_i128);
Goto(bb2)
}
bb2 = {
RET = Adt19::Variant1 { fld0: 18837692193446278012504744160794217847_i128 };
RET = Adt19::Variant1 { fld0: 39794421431838484182949386252371035916_i128 };
_5 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-32_isize),(-9223372036854775808_isize),55_isize,(-65_isize),9223372036854775807_isize];
Goto(bb3)
}
bb3 = {
_6.0 = [(-1121577212_i32),200034868_i32];
_5 = [9223372036854775807_isize,(-69_isize),9223372036854775807_isize,63_isize,(-80_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
place!(Field::<i128>(Variant(RET, 1), 0)) = 102069514999707149955068646794677531796_i128 | (-141063111935344094474968096814827567983_i128);
_13 = [0_usize,3334160994398022773_usize,8363025945233725039_usize];
_3 = [(-9223372036854775808_isize),88_isize,42_isize,9223372036854775807_isize,(-9223372036854775808_isize),117_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_5 = _2;
_3 = _2;
_5 = _1;
_1 = _2;
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),118_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-118_isize),9223372036854775807_isize];
_4 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,50_isize,9223372036854775807_isize];
_14 = core::ptr::addr_of_mut!(RET);
RET = Adt19::Variant1 { fld0: 1588256644214517322082922996138824813_i128 };
_14 = core::ptr::addr_of_mut!(RET);
place!(Field::<i128>(Variant((*_14), 1), 0)) = 160397540121184990987854135877137264476_i128 ^ 169244207661097223224097110504708653231_i128;
_5 = [9223372036854775807_isize,53_isize,9223372036854775807_isize,28_isize,(-9_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-121_isize)];
_3 = [(-9223372036854775808_isize),57_isize,(-51_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
place!(Field::<i128>(Variant((*_14), 1), 0)) = !114979298812139343334988307689195464320_i128;
_11 = !5245432574253388258_u64;
RET = Adt19::Variant1 { fld0: 131440983200198130611494642192295356775_i128 };
_16 = !(-9223372036854775808_isize);
Goto(bb4)
}
bb4 = {
RET = Adt19::Variant1 { fld0: (-59588454068442679449229292490236477766_i128) };
RET = Adt19::Variant1 { fld0: 61317979684892607397974166538013651238_i128 };
_3 = [_16,_16,_16,_16,_16,_16,_16,_16];
_1 = [_16,_16,_16,_16,_16,_16,_16,_16];
place!(Field::<i128>(Variant(RET, 1), 0)) = 59290_u16 as i128;
_6.1 = 3348397511_u32;
match _6.1 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
3348397511 => bb11,
_ => bb10
}
}
bb5 = {
_6.0 = [(-1121577212_i32),200034868_i32];
_5 = [9223372036854775807_isize,(-69_isize),9223372036854775807_isize,63_isize,(-80_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
place!(Field::<i128>(Variant(RET, 1), 0)) = 102069514999707149955068646794677531796_i128 | (-141063111935344094474968096814827567983_i128);
_13 = [0_usize,3334160994398022773_usize,8363025945233725039_usize];
_3 = [(-9223372036854775808_isize),88_isize,42_isize,9223372036854775807_isize,(-9223372036854775808_isize),117_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_5 = _2;
_3 = _2;
_5 = _1;
_1 = _2;
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),118_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-118_isize),9223372036854775807_isize];
_4 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,50_isize,9223372036854775807_isize];
_14 = core::ptr::addr_of_mut!(RET);
RET = Adt19::Variant1 { fld0: 1588256644214517322082922996138824813_i128 };
_14 = core::ptr::addr_of_mut!(RET);
place!(Field::<i128>(Variant((*_14), 1), 0)) = 160397540121184990987854135877137264476_i128 ^ 169244207661097223224097110504708653231_i128;
_5 = [9223372036854775807_isize,53_isize,9223372036854775807_isize,28_isize,(-9_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-121_isize)];
_3 = [(-9223372036854775808_isize),57_isize,(-51_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
place!(Field::<i128>(Variant((*_14), 1), 0)) = !114979298812139343334988307689195464320_i128;
_11 = !5245432574253388258_u64;
RET = Adt19::Variant1 { fld0: 131440983200198130611494642192295356775_i128 };
_16 = !(-9223372036854775808_isize);
Goto(bb4)
}
bb6 = {
RET = Adt19::Variant1 { fld0: 18837692193446278012504744160794217847_i128 };
RET = Adt19::Variant1 { fld0: 39794421431838484182949386252371035916_i128 };
_5 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-32_isize),(-9223372036854775808_isize),55_isize,(-65_isize),9223372036854775807_isize];
Goto(bb3)
}
bb7 = {
_9.0 = core::ptr::addr_of!(_11);
_11 = 15141604908883300906_u64 & 9030269753312901999_u64;
_6.0 = [(-119617777_i32),1124475511_i32];
_6.1 = 1585141468_u32;
RET = Adt19::Variant1 { fld0: 157615037804816778253509874839416238453_i128 };
place!(Field::<i128>(Variant(RET, 1), 0)) = 123862723274962118042555967120050867318_i128;
_11 = !9906106171745171017_u64;
place!(Field::<i128>(Variant(RET, 1), 0)) = 153071251174971237034095579532743323915_i128;
_4 = _2;
RET = Adt19::Variant1 { fld0: (-95086339532020007958235513747075041678_i128) };
_6.0 = [(-734266162_i32),692462590_i32];
place!(Field::<i128>(Variant(RET, 1), 0)) = 16_u8 as i128;
place!(Field::<i128>(Variant(RET, 1), 0)) = 157463252278776667272956449613132125867_i128 & (-65179332706734756591798895379682790803_i128);
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
RET = Adt19::Variant1 { fld0: 116966886788494714941667186885253846406_i128 };
_14 = core::ptr::addr_of_mut!((*_14));
_1 = [_16,_16,_16,_16,_16,_16,_16,_16];
_19 = '\u{bb0f6}';
match _6.1 {
0 => bb1,
1 => bb6,
2 => bb10,
3 => bb4,
4 => bb5,
5 => bb12,
6 => bb13,
3348397511 => bb15,
_ => bb14
}
}
bb12 = {
Return()
}
bb13 = {
RET = Adt19::Variant1 { fld0: (-59588454068442679449229292490236477766_i128) };
RET = Adt19::Variant1 { fld0: 61317979684892607397974166538013651238_i128 };
_3 = [_16,_16,_16,_16,_16,_16,_16,_16];
_1 = [_16,_16,_16,_16,_16,_16,_16,_16];
place!(Field::<i128>(Variant(RET, 1), 0)) = 59290_u16 as i128;
_6.1 = 3348397511_u32;
match _6.1 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
3348397511 => bb11,
_ => bb10
}
}
bb14 = {
_6.0 = [(-1121577212_i32),200034868_i32];
_5 = [9223372036854775807_isize,(-69_isize),9223372036854775807_isize,63_isize,(-80_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
place!(Field::<i128>(Variant(RET, 1), 0)) = 102069514999707149955068646794677531796_i128 | (-141063111935344094474968096814827567983_i128);
_13 = [0_usize,3334160994398022773_usize,8363025945233725039_usize];
_3 = [(-9223372036854775808_isize),88_isize,42_isize,9223372036854775807_isize,(-9223372036854775808_isize),117_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_5 = _2;
_3 = _2;
_5 = _1;
_1 = _2;
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),118_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-118_isize),9223372036854775807_isize];
_4 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,50_isize,9223372036854775807_isize];
_14 = core::ptr::addr_of_mut!(RET);
RET = Adt19::Variant1 { fld0: 1588256644214517322082922996138824813_i128 };
_14 = core::ptr::addr_of_mut!(RET);
place!(Field::<i128>(Variant((*_14), 1), 0)) = 160397540121184990987854135877137264476_i128 ^ 169244207661097223224097110504708653231_i128;
_5 = [9223372036854775807_isize,53_isize,9223372036854775807_isize,28_isize,(-9_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-121_isize)];
_3 = [(-9223372036854775808_isize),57_isize,(-51_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
place!(Field::<i128>(Variant((*_14), 1), 0)) = !114979298812139343334988307689195464320_i128;
_11 = !5245432574253388258_u64;
RET = Adt19::Variant1 { fld0: 131440983200198130611494642192295356775_i128 };
_16 = !(-9223372036854775808_isize);
Goto(bb4)
}
bb15 = {
_3 = [_16,_16,_16,_16,_16,_16,_16,_16];
Goto(bb16)
}
bb16 = {
_10.0 = &_19;
_14 = core::ptr::addr_of_mut!((*_14));
place!(Field::<i128>(Variant(RET, 1), 0)) = -39690592189738617385132763123462143794_i128;
Goto(bb17)
}
bb17 = {
Call(_22 = dump_var(14_usize, 8_usize, Move(_8), 11_usize, Move(_11), 2_usize, Move(_2), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_22 = dump_var(14_usize, 19_usize, Move(_19), 23_usize, _23, 23_usize, _23, 23_usize, _23), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: [u16; 3],mut _2: [isize; 8],mut _3: u16) -> bool {
mir! {
type RET = bool;
let _4: [bool; 1];
let _5: Adt38;
let _6: isize;
let _7: f64;
let _8: ();
let _9: ();
{
RET = !true;
RET = !true;
RET = false;
RET = _3 == _3;
RET = !false;
RET = !false;
_1 = [_3,_3,_3];
_3 = 37731_u16 & 16798_u16;
_1 = [_3,_3,_3];
RET = _3 >= _3;
RET = true;
RET = true & false;
_2 = [(-100_isize),9223372036854775807_isize,(-9223372036854775808_isize),115_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-56_isize),(-9223372036854775808_isize)];
RET = !true;
_1 = [_3,_3,_3];
_2 = [(-9223372036854775808_isize),(-54_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-117_isize),(-9223372036854775808_isize)];
_3 = !58338_u16;
_1 = [_3,_3,_3];
_2 = [42_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),69_isize,116_isize];
_3 = !31306_u16;
_1 = [_3,_3,_3];
RET = true;
Goto(bb1)
}
bb1 = {
_4 = [RET];
_4 = [RET];
Call(_3 = fn16(_2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = !false;
_4 = [RET];
_2 = [(-62_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-123_isize),(-61_isize),(-9223372036854775808_isize),(-69_isize)];
RET = _3 >= _3;
_5.fld5.1 = !2832909353_u32;
_5.fld0 = (-9066633923005274399_i64) as f32;
_5.fld5.0 = [(-932086835_i32),(-1974746836_i32)];
Goto(bb3)
}
bb3 = {
_5.fld3 = (-78_i8);
_5.fld1 = '\u{100e4a}';
_5.fld5.1 = !2886744604_u32;
_5.fld6 = 9223372036854775807_isize as i64;
_5.fld1 = '\u{5a2ef}';
_2 = [(-9223372036854775808_isize),98_isize,(-21_isize),(-115_isize),9223372036854775807_isize,27_isize,25_isize,9223372036854775807_isize];
_5.fld3 = 52_i8 << _5.fld5.1;
RET = _5.fld3 == _5.fld3;
_5.fld4 = 18888_i16;
_5.fld1 = '\u{15240}';
RET = true | true;
_5.fld5.0 = [(-476194461_i32),(-1714256598_i32)];
_5.fld2 = _3 as i128;
_5.fld0 = 9223372036854775807_isize as f32;
_5.fld0 = 60_u8 as f32;
match _5.fld4 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
18888 => bb8,
_ => bb7
}
}
bb4 = {
RET = !false;
_4 = [RET];
_2 = [(-62_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-123_isize),(-61_isize),(-9223372036854775808_isize),(-69_isize)];
RET = _3 >= _3;
_5.fld5.1 = !2832909353_u32;
_5.fld0 = (-9066633923005274399_i64) as f32;
_5.fld5.0 = [(-932086835_i32),(-1974746836_i32)];
Goto(bb3)
}
bb5 = {
_4 = [RET];
_4 = [RET];
Call(_3 = fn16(_2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_4 = [RET];
_5.fld1 = '\u{6dc28}';
_1 = [_3,_3,_3];
_5.fld2 = 146405650671492435586981527168528521749_i128;
RET = _3 <= _3;
_5.fld4 = _5.fld0 as i16;
_2 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),27_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_5.fld1 = '\u{f4a59}';
match _5.fld2 {
0 => bb4,
1 => bb6,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
146405650671492435586981527168528521749 => bb15,
_ => bb14
}
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_4 = [RET];
_4 = [RET];
Call(_3 = fn16(_2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
RET = !false;
_4 = [RET];
_2 = [(-62_isize),9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-123_isize),(-61_isize),(-9223372036854775808_isize),(-69_isize)];
RET = _3 >= _3;
_5.fld5.1 = !2832909353_u32;
_5.fld0 = (-9066633923005274399_i64) as f32;
_5.fld5.0 = [(-932086835_i32),(-1974746836_i32)];
Goto(bb3)
}
bb13 = {
_5.fld3 = (-78_i8);
_5.fld1 = '\u{100e4a}';
_5.fld5.1 = !2886744604_u32;
_5.fld6 = 9223372036854775807_isize as i64;
_5.fld1 = '\u{5a2ef}';
_2 = [(-9223372036854775808_isize),98_isize,(-21_isize),(-115_isize),9223372036854775807_isize,27_isize,25_isize,9223372036854775807_isize];
_5.fld3 = 52_i8 << _5.fld5.1;
RET = _5.fld3 == _5.fld3;
_5.fld4 = 18888_i16;
_5.fld1 = '\u{15240}';
RET = true | true;
_5.fld5.0 = [(-476194461_i32),(-1714256598_i32)];
_5.fld2 = _3 as i128;
_5.fld0 = 9223372036854775807_isize as f32;
_5.fld0 = 60_u8 as f32;
match _5.fld4 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
18888 => bb8,
_ => bb7
}
}
bb14 = {
_4 = [RET];
_4 = [RET];
Call(_3 = fn16(_2, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
_5.fld2 = 78694907226409113350142805805051110657_i128;
_5.fld5.1 = 1372324914_u32;
_2 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),94_isize,(-39_isize),(-9223372036854775808_isize)];
_5.fld3 = 8376523342447521528_u64 as i8;
_3 = _5.fld3 as u16;
_3 = 55511_u16 ^ 51914_u16;
_5.fld6 = 3107733924180704297_i64 + 3682018616167368106_i64;
_5.fld4 = -(-342_i16);
RET = false | false;
_6 = 9223372036854775807_isize;
_6 = (-125_isize);
_1 = [_3,_3,_3];
_5.fld2 = -46024938224286970893321612081707288760_i128;
_4 = [RET];
Goto(bb16)
}
bb16 = {
Call(_8 = dump_var(15_usize, 4_usize, Move(_4), 1_usize, Move(_1), 9_usize, _9, 9_usize, _9), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: [isize; 8],mut _2: [isize; 8]) -> u16 {
mir! {
type RET = u16;
let _3: isize;
let _4: &'static u32;
let _5: [i64; 8];
let _6: (u64, u64, u64, i64);
let _7: isize;
let _8: u64;
let _9: u8;
let _10: f32;
let _11: f32;
let _12: [usize; 4];
let _13: [u32; 3];
let _14: (f64, u8, &'static char, i8);
let _15: &'static char;
let _16: isize;
let _17: f64;
let _18: *mut u128;
let _19: (f32, &'static i64, [u8; 7], Adt38);
let _20: u32;
let _21: (&'static [u8; 2], &'static i64);
let _22: ([u32; 3], u16, Adt28);
let _23: i32;
let _24: Adt50;
let _25: ();
let _26: ();
{
_2 = _1;
RET = !16079_u16;
_1 = [(-106_isize),9223372036854775807_isize,9223372036854775807_isize,97_isize,110_isize,0_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = 18811_u16;
_1 = [(-11_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),2_isize,82_isize,63_isize,67_isize];
RET = 34823_u16 >> 49439_u16;
_6.3 = 21669_i16 as i64;
_2 = _1;
_3 = (-9223372036854775808_isize);
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_2 = [_3,_3,_3,_3,_3,_3,_3,_3];
_6.0 = 7472681633647757388_u64;
_3 = _6.0 as isize;
_7 = 136704927926639303267130058206608062654_i128 as isize;
match _6.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
7472681633647757388 => bb8,
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
_6.2 = !_6.0;
_3 = _7;
_2 = _1;
_7 = -_3;
_6.1 = _6.0 & _6.0;
_6.3 = !(-1101124467151426387_i64);
_8 = _6.1;
_6.2 = 46505291087147288392750919035388009399_u128 as u64;
_3 = _7 | _7;
_10 = RET as f32;
_6.1 = _6.0 >> _6.3;
_3 = -_7;
_9 = 1934014123_u32 as u8;
Goto(bb9)
}
bb9 = {
_9 = 64_u8 ^ 37_u8;
_11 = _10 - _10;
_8 = !_6.1;
_10 = (-87671245578213549537482942109213858338_i128) as f32;
match _6.0 {
0 => bb10,
7472681633647757388 => bb12,
_ => bb11
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_3 = !_7;
_5 = [_6.3,_6.3,_6.3,_6.3,_6.3,_6.3,_6.3,_6.3];
_13 = [2834884652_u32,3435801310_u32,1294520361_u32];
_14.3 = -(-59_i8);
_5 = [_6.3,_6.3,_6.3,_6.3,_6.3,_6.3,_6.3,_6.3];
RET = !44069_u16;
_8 = !_6.1;
_5 = [_6.3,_6.3,_6.3,_6.3,_6.3,_6.3,_6.3,_6.3];
_14.0 = _14.3 as f64;
_7 = 6_usize as isize;
_9 = !88_u8;
_1 = _2;
_1 = _2;
RET = !17715_u16;
_16 = -_7;
_9 = 122_u8;
_11 = _10;
_6.1 = !_6.0;
_14.3 = !104_i8;
_9 = 152_u8 + 251_u8;
_17 = -_14.0;
_16 = _7 >> _8;
_6.3 = RET as i64;
_8 = true as u64;
_6 = (_8, _8, _8, (-714392219662470893_i64));
_13 = [2075052960_u32,2828109298_u32,3091087255_u32];
_6.3 = !4091988011757130470_i64;
_12 = [1_usize,4_usize,2423992356331926119_usize,4824089869204878473_usize];
Call(_10 = fn17(_13, _16, _1, _14.3), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_12 = [0_usize,3512980757919348589_usize,7_usize,3695862125359265180_usize];
_14.1 = _9 * _9;
_6.3 = !2434739031044129291_i64;
RET = !59311_u16;
_10 = _11 * _11;
_7 = _16 << _14.1;
_9 = '\u{64690}' as u8;
_7 = -_16;
_7 = _17 as isize;
_16 = _7;
RET = !30463_u16;
_19.1 = &_6.3;
_6.2 = _6.3 as u64;
Goto(bb14)
}
bb14 = {
_6.0 = !_8;
_19.3.fld1 = '\u{583ed}';
_14.2 = &_19.3.fld1;
_19.3.fld4 = (-12046_i16) | (-27251_i16);
_7 = _19.3.fld4 as isize;
_16 = _3;
_19.0 = 76447421_i32 as f32;
_19.1 = &_6.3;
_19.2 = [_14.1,_14.1,_14.1,_14.1,_14.1,_14.1,_14.1];
_19.3.fld5.0 = [1573978342_i32,(-484689611_i32)];
_1 = [_16,_3,_3,_16,_7,_7,_7,_16];
_20 = 2309037040_u32;
_19.3.fld2 = !86363767648459663915363868131443667136_i128;
_11 = _6.1 as f32;
_14.0 = _17 * _17;
_19.3.fld2 = 10353058128746617634_usize as i128;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(16_usize, 20_usize, Move(_20), 9_usize, Move(_9), 5_usize, Move(_5), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(16_usize, 13_usize, Move(_13), 6_usize, Move(_6), 26_usize, _26, 26_usize, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: [u32; 3],mut _2: isize,mut _3: [isize; 8],mut _4: i8) -> f32 {
mir! {
type RET = f32;
let _5: [i8; 7];
let _6: (u64, [u64; 7], (u64, u64, u64, i64));
let _7: i32;
let _8: isize;
let _9: (u64, i128);
let _10: u32;
let _11: &'static isize;
let _12: (u64, i128);
let _13: &'static u32;
let _14: isize;
let _15: *mut u128;
let _16: i8;
let _17: [isize; 8];
let _18: (f32, &'static i64, [u8; 7], Adt38);
let _19: ();
let _20: ();
{
_4 = 15_i8 ^ 8_i8;
RET = 17639_i16 as f32;
_4 = 6106_i16 as i8;
_2 = 26_isize;
_4 = 10402136313339394603_u64 as i8;
_5 = [_4,_4,_4,_4,_4,_4,_4];
_1 = [68329294_u32,1558810997_u32,3034900379_u32];
RET = (-5049_i16) as f32;
RET = 3172891231_u32 as f32;
_6.2 = (9921208530730499243_u64, 17665325017565712696_u64, 2398931030896481506_u64, 3569135195441494252_i64);
Goto(bb1)
}
bb1 = {
RET = _4 as f32;
_3 = [_2,_2,_2,_2,_2,_2,_2,_2];
_6.0 = !_6.2.0;
_4 = 116_i8 & (-31_i8);
RET = 15826742827580543822755589191973013458_i128 as f32;
Goto(bb2)
}
bb2 = {
_6.1 = [_6.2.2,_6.2.0,_6.0,_6.0,_6.0,_6.2.2,_6.2.2];
_6.2.1 = _6.0 ^ _6.2.0;
match _6.2.3 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
3569135195441494252 => bb8,
_ => bb7
}
}
bb3 = {
RET = _4 as f32;
_3 = [_2,_2,_2,_2,_2,_2,_2,_2];
_6.0 = !_6.2.0;
_4 = 116_i8 & (-31_i8);
RET = 15826742827580543822755589191973013458_i128 as f32;
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
_3 = [_2,_2,_2,_2,_2,_2,_2,_2];
_8 = _2 ^ _2;
RET = 1240442431_i32 as f32;
RET = _6.0 as f32;
_3 = [_8,_8,_2,_2,_2,_8,_8,_2];
RET = _4 as f32;
_6.2.2 = _6.2.1;
_6.2.2 = (-868524871_i32) as u64;
_5 = [_4,_4,_4,_4,_4,_4,_4];
_3 = [_8,_8,_8,_2,_8,_8,_8,_8];
_6.0 = !_6.2.1;
RET = (-6025310019501602141916304685825981400_i128) as f32;
_8 = _2;
_6.2.2 = !_6.0;
_8 = -_2;
_4 = -(-102_i8);
RET = 39_u8 as f32;
Goto(bb9)
}
bb9 = {
_2 = !_8;
_10 = 2495203713_u32 << _6.0;
_9.0 = _6.2.2 | _6.2.1;
_6.2.2 = _6.0;
RET = 1493983543_i32 as f32;
_9 = (_6.0, (-14121934353801390002257932764191481317_i128));
_12.0 = (-1035272108_i32) as u64;
_11 = &_2;
_2 = _10 as isize;
_4 = 100_i8;
_7 = 24340_i16 as i32;
_5 = [_4,_4,_4,_4,_4,_4,_4];
_13 = &_10;
_10 = false as u32;
_6.2.0 = !_6.2.1;
match _9.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb7,
326160432567137073461116674667576730139 => bb10,
_ => bb6
}
}
bb10 = {
_5 = [_4,_4,_4,_4,_4,_4,_4];
_4 = -30_i8;
_4 = RET as i8;
_13 = &_10;
_9 = (_6.2.0, 143325369305740750364296527965510647704_i128);
_8 = !_2;
_3 = [_8,_2,_2,_2,_8,_8,_8,_8];
_12 = (_6.2.1, _9.1);
_6.2.1 = !_6.2.2;
_6.2.3 = 923648314658889220_i64 & (-8251683878775260876_i64);
_9.0 = 53_u8 as u64;
_6.2 = (_12.0, _12.0, _6.0, (-159944014488677731_i64));
_8 = !_2;
_12.0 = _4 as u64;
_5 = [_4,_4,_4,_4,_4,_4,_4];
_12.1 = -_9.1;
_8 = -_2;
_6.2.0 = !_6.2.1;
_6.0 = !_6.2.0;
_13 = &(*_13);
_13 = &(*_13);
_13 = &(*_13);
_13 = &(*_13);
_9.1 = !_12.1;
match _6.2.3 {
0 => bb1,
1 => bb11,
2 => bb12,
3 => bb13,
340282366920938463463214663417279533725 => bb15,
_ => bb14
}
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
_6.1 = [_6.2.2,_6.2.0,_6.0,_6.0,_6.0,_6.2.2,_6.2.2];
_6.2.1 = _6.0 ^ _6.2.0;
match _6.2.3 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
3569135195441494252 => bb8,
_ => bb7
}
}
bb15 = {
_16 = _4;
_18.3.fld3 = !_4;
_18.1 = &_18.3.fld6;
_18.1 = &_18.3.fld6;
_14 = -_8;
Goto(bb16)
}
bb16 = {
Call(_19 = dump_var(17_usize, 12_usize, Move(_12), 4_usize, Move(_4), 10_usize, Move(_10), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_19 = dump_var(17_usize, 8_usize, Move(_8), 9_usize, Move(_9), 20_usize, _20, 20_usize, _20), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: [i16; 8],mut _2: bool,mut _3: [i16; 8],mut _4: isize,mut _5: bool,mut _6: i16,mut _7: bool,mut _8: isize) -> *mut u128 {
mir! {
type RET = *mut u128;
let _9: Adt38;
let _10: [u8; 7];
let _11: [u8; 2];
let _12: (u64, u64, u64, i64);
let _13: (u64, u64, u64, i64);
let _14: f32;
let _15: (usize, u16, Adt19, bool);
let _16: &'static [u8; 2];
let _17: isize;
let _18: (&'static char,);
let _19: *const ([i8; 7], [i32; 2]);
let _20: u64;
let _21: f64;
let _22: Adt59;
let _23: f32;
let _24: &'static u32;
let _25: bool;
let _26: ([i8; 7], [i32; 2]);
let _27: f64;
let _28: usize;
let _29: &'static [u8; 7];
let _30: [u8; 2];
let _31: *const Adt24;
let _32: u128;
let _33: (&'static i8,);
let _34: u64;
let _35: i16;
let _36: ();
let _37: ();
{
_7 = _6 < _6;
_2 = _5 > _7;
_5 = _7;
_3 = _1;
_6 = !(-29571_i16);
_2 = _8 == _4;
_5 = !_7;
_6 = -(-22215_i16);
_7 = _8 <= _4;
_3 = [_6,_6,_6,_6,_6,_6,_6,_6];
_7 = _2;
_1 = _3;
_9.fld5.0 = [(-1929038994_i32),1225136020_i32];
_9.fld2 = _2 as i128;
_9.fld5.1 = 1610753322_u32;
_10 = [160_u8,220_u8,140_u8,18_u8,48_u8,198_u8,148_u8];
_9.fld5.1 = 4233234307_u32 ^ 2073229761_u32;
_9.fld3 = 77_i8 - 37_i8;
_9.fld6 = -6462992177296286850_i64;
_6 = !3744_i16;
_3 = _1;
_12 = (2683779084350789206_u64, 18160021435432045576_u64, 16719777185044106529_u64, _9.fld6);
_11 = [17_u8,191_u8];
_2 = _9.fld2 < _9.fld2;
_9.fld1 = '\u{6d1b9}';
_10 = [212_u8,52_u8,220_u8,155_u8,11_u8,74_u8,87_u8];
match _12.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
18160021435432045576 => bb8,
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
_2 = _5;
_9.fld4 = !_6;
_8 = _4 + _4;
_5 = !_2;
_9.fld5.0 = [(-1096558111_i32),1149773105_i32];
match _12.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
16719777185044106529 => bb9,
_ => bb7
}
}
bb9 = {
_6 = _7 as i16;
_9.fld2 = 105224815337655992566493727357888720690_i128 << _12.1;
_9.fld6 = _12.3;
_9.fld3 = (-72_i8) - 113_i8;
_10 = [78_u8,142_u8,137_u8,145_u8,115_u8,157_u8,238_u8];
_12.1 = _12.2;
_12.2 = !_12.1;
_9.fld5.1 = 2040931380_u32 << _6;
_9.fld5.1 = !2499455410_u32;
_12.2 = _2 as u64;
_3 = _1;
_9.fld6 = !_12.3;
Goto(bb10)
}
bb10 = {
_9.fld1 = '\u{3d8db}';
_13.2 = _12.2 | _12.2;
_12.3 = _9.fld6 ^ _9.fld6;
_13.3 = _12.3 + _9.fld6;
_13 = (_12.2, _12.1, _12.2, _9.fld6);
_12.0 = _13.0 - _13.0;
_12.1 = !_13.0;
_12 = (_13.0, _13.2, _13.0, _13.3);
_9.fld3 = !(-94_i8);
_1 = _3;
_9.fld1 = '\u{93afb}';
_9.fld0 = _8 as f32;
_9.fld1 = '\u{59974}';
_11 = [67_u8,30_u8];
_7 = !_5;
_5 = _7;
_15.3 = !_2;
_15.0 = !9394103409232386358_usize;
_15.1 = !40719_u16;
_9.fld4 = -_6;
match _13.1 {
0 => bb1,
1 => bb9,
2 => bb11,
3 => bb12,
4 => bb13,
16719777185044106529 => bb15,
_ => bb14
}
}
bb11 = {
_6 = _7 as i16;
_9.fld2 = 105224815337655992566493727357888720690_i128 << _12.1;
_9.fld6 = _12.3;
_9.fld3 = (-72_i8) - 113_i8;
_10 = [78_u8,142_u8,137_u8,145_u8,115_u8,157_u8,238_u8];
_12.1 = _12.2;
_12.2 = !_12.1;
_9.fld5.1 = 2040931380_u32 << _6;
_9.fld5.1 = !2499455410_u32;
_12.2 = _2 as u64;
_3 = _1;
_9.fld6 = !_12.3;
Goto(bb10)
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
_17 = -_8;
_1 = _3;
_9.fld4 = _6;
_9.fld5.1 = 3255411129_u32;
_15.3 = !_7;
_11 = [2_u8,100_u8];
_13.2 = _9.fld0 as u64;
_15.3 = _7 | _5;
_9.fld0 = 72_u8 as f32;
_15.2 = Adt19::Variant0 { fld0: _2,fld1: _12,fld2: _9.fld5.1 };
_10 = [249_u8,8_u8,197_u8,6_u8,226_u8,216_u8,249_u8];
_14 = _9.fld0 + _9.fld0;
_9.fld5.1 = !Field::<u32>(Variant(_15.2, 0), 2);
SetDiscriminant(_15.2, 1);
_11 = [214_u8,1_u8];
_9.fld1 = '\u{a8f37}';
_13.1 = !_12.2;
_8 = _2 as isize;
_18.0 = &_9.fld1;
_9.fld3 = (-66_i8) ^ (-66_i8);
_13.3 = _9.fld6;
_16 = &_11;
_5 = _8 <= _4;
Goto(bb16)
}
bb16 = {
place!(Field::<i128>(Variant(_15.2, 1), 0)) = _13.0 as i128;
_13.3 = _15.1 as i64;
_18.0 = &_9.fld1;
_12.1 = !_13.0;
_9.fld4 = -_6;
_5 = _2 != _7;
_15.0 = _12.3 as usize;
_12.0 = _12.2 ^ _13.0;
_12.0 = _8 as u64;
_15.0 = !10784949952851563276_usize;
_9.fld1 = '\u{9159c}';
_22.fld1.2 = Adt19::Variant1 { fld0: Field::<i128>(Variant(_15.2, 1), 0) };
_22.fld2.1 = !_13.0;
_22.fld2.0 = _12.1;
_23 = 43_u8 as f32;
_2 = _15.3 & _5;
_15.3 = _6 >= _9.fld4;
place!(Field::<i128>(Variant(_22.fld1.2, 1), 0)) = Field::<i128>(Variant(_15.2, 1), 0);
_22.fld2.1 = _22.fld2.0;
_16 = &(*_16);
_5 = _7;
SetDiscriminant(_15.2, 1);
_22.fld2.2 = _12.1 | _13.0;
_2 = _13.2 == _13.2;
Goto(bb17)
}
bb17 = {
_22.fld6 = _9.fld6 ^ _13.3;
_22.fld1.3 = _5;
_15.2 = Adt19::Variant0 { fld0: _15.3,fld1: _13,fld2: _9.fld5.1 };
_22.fld4.0 = [_9.fld3,_9.fld3,_9.fld3,_9.fld3,_9.fld3,_9.fld3,_9.fld3];
_26.1 = [527416494_i32,553413_i32];
_3 = [_9.fld4,_9.fld4,_6,_9.fld4,_9.fld4,_6,_6,_6];
place!(Field::<i128>(Variant(_22.fld1.2, 1), 0)) = _12.3 as i128;
_22.fld5 = _10;
_17 = _8;
_26 = (_22.fld4.0, _9.fld5.0);
Goto(bb18)
}
bb18 = {
_28 = _9.fld1 as usize;
SetDiscriminant(_15.2, 1);
_25 = _7 & _15.3;
place!(Field::<i128>(Variant(_15.2, 1), 0)) = _15.0 as i128;
_26.1 = [(-941433250_i32),1500246390_i32];
_9.fld6 = _22.fld6;
_9.fld5.0 = [(-1590658578_i32),1306106815_i32];
_5 = _12.1 < _13.1;
_9.fld3 = -63_i8;
_10 = [59_u8,157_u8,65_u8,95_u8,95_u8,245_u8,229_u8];
_13.1 = _22.fld2.2 - _12.1;
_22.fld3 = Adt28::Variant2 { fld0: _15.1,fld1: _14,fld2: _4,fld3: _9.fld3,fld4: _15.0,fld5: _22.fld1.2 };
_22.fld6 = _13.3 - _13.3;
_22.fld1.1 = !Field::<u16>(Variant(_22.fld3, 2), 0);
Goto(bb19)
}
bb19 = {
_25 = !_22.fld1.3;
Goto(bb20)
}
bb20 = {
_6 = _9.fld4 + _9.fld4;
_16 = &(*_16);
_10 = [202_u8,238_u8,94_u8,4_u8,217_u8,157_u8,138_u8];
_28 = Field::<usize>(Variant(_22.fld3, 2), 4);
_9.fld5.0 = _26.1;
SetDiscriminant(Field::<Adt19>(Variant(_22.fld3, 2), 5), 1);
_22.fld2.3 = -_12.3;
Goto(bb21)
}
bb21 = {
_30 = (*_16);
_27 = 35_u8 as f64;
_12 = (_13.2, _13.2, _22.fld2.2, _22.fld2.3);
_22.fld0 = [_22.fld1.1,_22.fld1.1,_15.1,_22.fld1.1,_22.fld1.1,_22.fld1.1];
place!(Field::<isize>(Variant(_22.fld3, 2), 2)) = _12.0 as isize;
place!(Field::<Adt19>(Variant(_22.fld3, 2), 5)) = _22.fld1.2;
_22.fld4 = (_26.0, _26.1);
_15 = (Field::<usize>(Variant(_22.fld3, 2), 4), _22.fld1.1, Field::<Adt19>(Variant(_22.fld3, 2), 5), _22.fld1.3);
_9.fld5.1 = 89_u8 as u32;
_9.fld2 = _9.fld1 as i128;
place!(Field::<i8>(Variant(_22.fld3, 2), 3)) = _13.3 as i8;
place!(Field::<i8>(Variant(_22.fld3, 2), 3)) = _9.fld3;
_19 = core::ptr::addr_of!(_22.fld4);
_12.2 = _22.fld2.1 ^ _22.fld2.1;
_12 = (_13.0, _13.0, _22.fld2.2, _22.fld2.3);
_4 = Field::<isize>(Variant(_22.fld3, 2), 2) * _8;
_22.fld7 = (-1955719119_i32) as u32;
_26.0 = [Field::<i8>(Variant(_22.fld3, 2), 3),_9.fld3,Field::<i8>(Variant(_22.fld3, 2), 3),_9.fld3,Field::<i8>(Variant(_22.fld3, 2), 3),_9.fld3,Field::<i8>(Variant(_22.fld3, 2), 3)];
_22.fld2.2 = _12.2;
Call(_12.3 = core::intrinsics::bswap(_13.3), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
_22.fld3 = Adt28::Variant2 { fld0: _15.1,fld1: _9.fld0,fld2: _4,fld3: _9.fld3,fld4: _15.0,fld5: _15.2 };
_6 = _9.fld4;
_22.fld2.2 = _12.1 | _13.1;
_22.fld6 = Field::<i128>(Variant(_15.2, 1), 0) as i64;
_30 = [198_u8,67_u8];
RET = core::ptr::addr_of_mut!(_32);
(*RET) = _15.1 as u128;
_29 = &_10;
_5 = _2;
(*_19).0 = _26.0;
_9.fld0 = _15.0 as f32;
_9.fld6 = 98_u8 as i64;
(*_19) = (_26.0, _9.fld5.0);
_19 = core::ptr::addr_of!(_22.fld4);
Goto(bb23)
}
bb23 = {
Call(_36 = dump_var(18_usize, 6_usize, Move(_6), 11_usize, Move(_11), 3_usize, Move(_3), 17_usize, Move(_17)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_36 = dump_var(18_usize, 25_usize, Move(_25), 13_usize, Move(_13), 2_usize, Move(_2), 4_usize, Move(_4)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_36 = dump_var(18_usize, 28_usize, Move(_28), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(25_u8), std::hint::black_box('\u{1030d7}'), std::hint::black_box(7_usize), std::hint::black_box(13938767988004472548_u64), std::hint::black_box(14061_u16));
                
            }
impl PrintFDebug for Adt19{
	unsafe fn printf_debug(&self){unsafe{printf("Adt19::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt19 {
Variant0{
fld0: bool,
fld1: (u64, u64, u64, i64),
fld2: u32,

},
Variant1{
fld0: i128,

}}
impl PrintFDebug for Adt24{
	unsafe fn printf_debug(&self){unsafe{printf("Adt24::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt24 {
Variant0{
fld0: u8,
fld1: char,
fld2: i16,

},
Variant1{
fld0: u16,
fld1: u64,

},
Variant2{
fld0: u64,
fld1: char,
fld2: (u64, u64, u64, i64),
fld3: u16,
fld4: usize,
fld5: i32,

},
Variant3{
fld0: ([i32; 2], u32),
fld1: *mut Adt19,
fld2: usize,

}}
impl PrintFDebug for Adt28{
	unsafe fn printf_debug(&self){unsafe{printf("Adt28::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt28 {
Variant0{
fld0: u64,
fld1: char,
fld2: [i32; 2],
fld3: *mut u8,
fld4: [u64; 7],
fld5: i32,
fld6: u16,

},
Variant1{
fld0: u8,
fld1: u16,
fld2: f64,
fld3: i64,
fld4: *mut u8,
fld5: [u64; 7],

},
Variant2{
fld0: u16,
fld1: f32,
fld2: isize,
fld3: i8,
fld4: usize,
fld5: Adt19,

}}
impl PrintFDebug for Adt38{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt38{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt38 {
fld0: f32,
fld1: char,
fld2: i128,
fld3: i8,
fld4: i16,
fld5: ([i32; 2], u32),
fld6: i64,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: f64,
fld1: i8,

},
Variant1{
fld0: [i8; 7],
fld1: Adt28,
fld2: (u64, u64, u64, i64),
fld3: [u16; 3],
fld4: *mut char,
fld5: i32,
fld6: *const [i32; 2],
fld7: i128,

},
Variant2{
fld0: [i32; 2],
fld1: *mut *mut Adt19,
fld2: *const [bool; 1],
fld3: [i8; 7],
fld4: [u8; 2],
fld5: [u16; 3],
fld6: [u64; 7],

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt59{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt59 {
fld0: [u16; 6],
fld1: (usize, u16, Adt19, bool),
fld2: (u64, u64, u64, i64),
fld3: Adt28,
fld4: ([i8; 7], [i32; 2]),
fld5: [u8; 7],
fld6: i64,
fld7: u32,
}
impl PrintFDebug for Adt74{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt74{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt74 {
fld0: *mut char,
}
impl PrintFDebug for Adt83{
	unsafe fn printf_debug(&self){unsafe{printf("Adt83::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt83 {
Variant0{
fld0: Adt59,
fld1: ([i32; 2], u32),
fld2: Adt74,

},
Variant1{
fld0: [bool; 1],
fld1: char,
fld2: u32,
fld3: (*const u64,),

},
Variant2{
fld0: *mut *mut Adt19,
fld1: char,
fld2: Adt24,

},
Variant3{
fld0: [i16; 8],
fld1: char,
fld2: isize,
fld3: [isize; 8],
fld4: i16,
fld5: u16,

}}

