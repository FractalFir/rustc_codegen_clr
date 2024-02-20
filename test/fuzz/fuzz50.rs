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
pub fn fn0(mut _1: usize,mut _2: u32,mut _3: i16) -> char {
mir! {
type RET = char;
let _4: isize;
let _5: (u32,);
let _6: [usize; 5];
let _7: &'static (bool, u128, ([i128; 1], u16, (i16,)), (i16,));
let _8: f64;
let _9: (Adt29, &'static u128, u64, u8);
let _10: f64;
let _11: bool;
let _12: f32;
let _13: [u128; 1];
let _14: char;
let _15: Adt71;
let _16: &'static [i32; 3];
let _17: f32;
let _18: i128;
let _19: isize;
let _20: u32;
let _21: (*mut (i16,), isize, f64, f64);
let _22: f64;
let _23: Adt41;
let _24: [i128; 1];
let _25: isize;
let _26: u8;
let _27: char;
let _28: f64;
let _29: (Adt51, [i16; 5], &'static i128);
let _30: isize;
let _31: u64;
let _32: i16;
let _33: f32;
let _34: isize;
let _35: &'static i128;
let _36: [i32; 5];
let _37: [u8; 8];
let _38: ();
let _39: ();
{
RET = '\u{ef74f}';
_3 = !16505_i16;
_3 = (-24240_i16) & (-24777_i16);
RET = '\u{119e7}';
_1 = !5_usize;
_1 = 8456674464759233635_usize;
_2 = (-4003243568714478502_i64) as u32;
_2 = _3 as u32;
_4 = 2352326584840781362_u64 as isize;
_3 = 10141_i16 - (-24154_i16);
RET = '\u{d4ca6}';
_5 = (_2,);
RET = '\u{5197b}';
RET = '\u{c0e46}';
_5 = (_2,);
_4 = 7742069975681997420_i64 as isize;
_1 = 7750577052534052175_usize * 7_usize;
Goto(bb1)
}
bb1 = {
_3 = (-32251_i16) & (-27206_i16);
_5 = (_2,);
RET = '\u{81e67}';
_3 = 21010_i16 | 4112_i16;
Call(RET = fn1(_5.0, _3, _5.0, _5, _3, _4, _3, _5.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = '\u{b06e1}';
_5.0 = !_2;
_5 = (_2,);
RET = '\u{2762a}';
_5.0 = _2;
RET = '\u{9abad}';
_1 = 13637871354545171101_usize * 8766030639596914031_usize;
_5 = (_2,);
RET = '\u{f587a}';
Call(_5.0 = core::intrinsics::bswap(_2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Goto(bb4)
}
bb4 = {
_5.0 = !_2;
_5.0 = 6038999670673049624_u64 as u32;
_1 = 0_usize & 1_usize;
_4 = (-54_i8) as isize;
_8 = _2 as f64;
_9.3 = 186_u8 & 70_u8;
_4 = (-1_isize);
_4 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
_1 = 18_i8 as usize;
_2 = _5.0 & _5.0;
_6 = [_1,_1,_1,_1,_1];
_11 = false;
_8 = (-44_i8) as f64;
_5 = (_2,);
_2 = _5.0 * _5.0;
_5.0 = _2;
_1 = 5_usize;
_13 = [99205059270673251329469080097727271341_u128];
_12 = 12188112099345648409_u64 as f32;
_12 = _8 as f32;
RET = '\u{71064}';
_6 = [_1,_1,_1,_1,_1];
_8 = 10532642979457103706_u64 as f64;
_5.0 = (-146704578384334565279661697244508632476_i128) as u32;
_3 = (-19718_i16);
Goto(bb5)
}
bb5 = {
_13 = [193706545289267738100620071746112861045_u128];
_11 = !true;
RET = '\u{1626d}';
_9.2 = !15586615628843688842_u64;
_5 = (_2,);
_14 = RET;
RET = _14;
_6 = [_1,_1,_1,_1,_1];
_13 = [64402524299133646857979605082068634827_u128];
RET = _14;
_8 = _12 as f64;
_6 = [_1,_1,_1,_1,_1];
_10 = _8 + _8;
_19 = -_4;
_3 = _10 as i16;
RET = _14;
match _1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb6,
5 => bb8,
_ => bb7
}
}
bb6 = {
_5.0 = !_2;
_5.0 = 6038999670673049624_u64 as u32;
_1 = 0_usize & 1_usize;
_4 = (-54_i8) as isize;
_8 = _2 as f64;
_9.3 = 186_u8 & 70_u8;
_4 = (-1_isize);
_4 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
_1 = 18_i8 as usize;
_2 = _5.0 & _5.0;
_6 = [_1,_1,_1,_1,_1];
_11 = false;
_8 = (-44_i8) as f64;
_5 = (_2,);
_2 = _5.0 * _5.0;
_5.0 = _2;
_1 = 5_usize;
_13 = [99205059270673251329469080097727271341_u128];
_12 = 12188112099345648409_u64 as f32;
_12 = _8 as f32;
RET = '\u{71064}';
_6 = [_1,_1,_1,_1,_1];
_8 = 10532642979457103706_u64 as f64;
_5.0 = (-146704578384334565279661697244508632476_i128) as u32;
_3 = (-19718_i16);
Goto(bb5)
}
bb7 = {
_3 = (-32251_i16) & (-27206_i16);
_5 = (_2,);
RET = '\u{81e67}';
_3 = 21010_i16 | 4112_i16;
Call(RET = fn1(_5.0, _3, _5.0, _5, _3, _4, _3, _5.0), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_3 = !2842_i16;
_2 = 144480206747657794180928261872749806048_u128 as u32;
_17 = (-37_i8) as f32;
_1 = 3909562545155585619_usize ^ 0_usize;
_8 = -_10;
_19 = _4 >> _5.0;
RET = _14;
_2 = !_5.0;
_3 = _2 as i16;
_2 = _5.0;
_8 = _10;
_5 = (_2,);
_20 = _5.0 - _2;
_4 = _8 as isize;
_12 = _17;
_18 = -114454237510624271731952084429246373383_i128;
_20 = _2;
_11 = !false;
_2 = _5.0;
_2 = !_5.0;
_17 = _18 as f32;
_1 = 1_usize * 7_usize;
_8 = _10 * _10;
RET = _14;
_2 = !_20;
Call(_21.3 = core::intrinsics::fmaf64(_8, _8, _8), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_11 = true & false;
_9.2 = 8349345388365043288_u64;
_13 = [149797699858840725803528914051327172733_u128];
_19 = _4;
_21.1 = _4;
_2 = _20 & _5.0;
Call(_38 = dump_var(0_usize, 2_usize, Move(_2), 20_usize, Move(_20), 5_usize, Move(_5), 2_usize, Move(_2)), ReturnTo(t0), UnwindUnreachable())
}
t0 = {
_23.fld5 = 1708492193_i32;
_22 = _8;
_23.fld6.2.0 = _8 as u32;
_23.fld2 = _4;
_23.fld6.0 = !269872210000144760371080075330709049953_u128;
_12 = _17 * _17;
_4 = _23.fld6.0 as isize;
_7 = &_23.fld3;
_23.fld1 = [_19,_23.fld2,_23.fld2,_4,_4,_19];
match _23.fld5 {
1708492193 => bb11,
_ => bb10
}
}
bb10 = {
_3 = (-32251_i16) & (-27206_i16);
_5 = (_2,);
RET = '\u{81e67}';
_3 = 21010_i16 | 4112_i16;
Call(RET = fn1(_5.0, _3, _5.0, _5, _3, _4, _3, _5.0), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_23.fld3.2.1 = 39286_u16;
_2 = _9.2 as u32;
Call(_38 = dump_var(0_usize, 2_usize, Move(_2), 2_usize, Move(_2), 9_usize, Move(_9.2), 9_usize, Move(_9.2)), ReturnTo(t1), UnwindUnreachable())
}
t1 = {
_23.fld3.1 = _9.3 as u128;
_21.1 = _19;
_3 = (-5737_i16);
_23.fld6.1 = core::ptr::addr_of!((*_7).0);
_5.0 = _20 & _23.fld6.2.0;
_23.fld3.2.2 = (_3,);
_23.fld1 = [_23.fld2,_21.1,_21.1,_4,_21.1,_23.fld2];
// WRONG!
_23.fld3.2.2.0 = _3 ^ _3;
_23.fld3.2.0 = [_18];
_23.fld2 = _9.2 as isize;
_7 = &_23.fld3;
_23.fld0 = (*_7).2.1;
_5.0 = _23.fld6.2.0 >> _23.fld3.2.2.0;
Call(_38 = dump_var(0_usize, 3_usize, Move(_3), 23_usize, Move(_23.fld6.2.0), 5_usize, Move(_5.0), 23_usize, Move(_23.fld3.2.2.0)), ReturnTo(t3), UnwindUnreachable())
}
t3 = {
_23.fld6.0 = (*_7).1 >> _5.0;
_21.0 = core::ptr::addr_of_mut!(_23.fld3.3);
_23.fld7.0 = _2;
_29.2 = &_18;
match (*_7).2.1 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
39286 => bb18,
_ => bb17
}
}
bb12 = {
_3 = (-32251_i16) & (-27206_i16);
_5 = (_2,);
RET = '\u{81e67}';
_3 = 21010_i16 | 4112_i16;
Call(RET = fn1(_5.0, _3, _5.0, _5, _3, _4, _3, _5.0), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
Goto(bb4)
}
bb14 = {
RET = '\u{b06e1}';
_5.0 = !_2;
_5 = (_2,);
RET = '\u{2762a}';
_5.0 = _2;
RET = '\u{9abad}';
_1 = 13637871354545171101_usize * 8766030639596914031_usize;
_5 = (_2,);
RET = '\u{f587a}';
Call(_5.0 = core::intrinsics::bswap(_2), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_3 = (-32251_i16) & (-27206_i16);
_5 = (_2,);

RET = '\u{81e67}';
_3 = 21010_i16 | 4112_i16;
Call(RET = fn1(_5.0, _3, _5.0, _5, _3, _4, _3, _5.0), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_5.0 = !_2;
_5.0 = 6038999670673049624_u64 as u32;
_1 = 0_usize & 1_usize;
_4 = (-54_i8) as isize;
_8 = _2 as f64;
_9.3 = 186_u8 & 70_u8;
_4 = (-1_isize);
_4 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
_1 = 18_i8 as usize;
_2 = _5.0 & _5.0;
_6 = [_1,_1,_1,_1,_1];
_11 = false;
_8 = (-44_i8) as f64;
_5 = (_2,);
_2 = _5.0 * _5.0;

_5.0 = _2;
_1 = 5_usize;
_13 = [99205059270673251329469080097727271341_u128];
_12 = 12188112099345648409_u64 as f32;
_12 = _8 as f32;
RET = '\u{71064}';
_6 = [_1,_1,_1,_1,_1];
_8 = 10532642979457103706_u64 as f64;
_5.0 = (-146704578384334565279661697244508632476_i128) as u32;
_3 = (-19718_i16);
Goto(bb5)
}
bb17 = {
_5.0 = !_2;
_5.0 = 6038999670673049624_u64 as u32;
_1 = 0_usize & 1_usize;
_4 = (-54_i8) as isize;
_8 = _2 as f64;
_9.3 = 186_u8 & 70_u8;
_4 = (-1_isize);
_4 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
_1 = 18_i8 as usize;
_2 = _5.0 & _5.0;
_6 = [_1,_1,_1,_1,_1];
_11 = false;
_8 = (-44_i8) as f64;
_5 = (_2,);
_2 = _5.0 * _5.0;

_5.0 = _2;
_1 = 5_usize;
_13 = [99205059270673251329469080097727271341_u128];
_12 = 12188112099345648409_u64 as f32;
_12 = _8 as f32;
RET = '\u{71064}';
_6 = [_1,_1,_1,_1,_1];
_8 = 10532642979457103706_u64 as f64;
_5.0 = (-146704578384334565279661697244508632476_i128) as u32;

_3 = (-19718_i16);
Goto(bb5)
}
bb18 = {
_23.fld3.2.0 = [_18];
RET = _14;
_18 = _9.3 as i128;
_28 = -_8;
_20 = !_23.fld7.0;
_23.fld7.0 = !_23.fld6.2.0;
_23.fld3.3 = (_23.fld3.2.2.0,);
_29.2 = &_18;
_33 = _23.fld6.0 as f32;
_8 = _22 * _28;
_23.fld4 = core::ptr::addr_of_mut!(_29.1);
_4 = (*_7).2.1 as isize;
_2 = _5.0;
Call(_38 = dump_var(0_usize, 2_usize, Move(_2), 2_usize, Move(_2), 5_usize, Move(_5.0), 5_usize, Move(_5.0)), ReturnTo(t2), UnwindUnreachable())
}
t2 = {
_34 = -_19;
_23.fld6.3 = [_21.1,_21.1,_21.1,_21.1,_21.1,_19];
_6 = [_1,_1,_1,_1,_1];
_23.fld3.1 = !_23.fld6.0;
_7 = &_23.fld3;
_23.fld7.0 = !_23.fld6.2.0;
_23.fld6.2 = (_2,);
Goto(bb19)
}
bb19 = {
Call(_38 = dump_var(0_usize, 34_usize, Move(_34), 20_usize, Move(_20), 1_usize, Move(_1), 2_usize, Move(_2)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_38 = dump_var(0_usize, 5_usize, Move(_5), 6_usize, Move(_6), 39_usize, _39, 39_usize, _39), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: u32,mut _2: i16,mut _3: u32,mut _4: (u32,),mut _5: i16,mut _6: isize,mut _7: i16,mut _8: u32) -> char {
mir! {
type RET = char;
let _9: u32;
let _10: *const &'static usize;
let _11: (bool, u128, ([i128; 1], u16, (i16,)), (i16,));
let _12: usize;
let _13: [i16; 5];
let _14: [i64; 5];
let _15: bool;
let _16: &'static (bool, u128, ([i128; 1], u16, (i16,)), (i16,));
let _17: *const bool;
let _18: *const Adt47;
let _19: [u128; 1];
let _20: isize;
let _21: (([i128; 1], u16, (i16,)),);
let _22: u128;
let _23: i32;
let _24: u128;
let _25: u32;
let _26: bool;
let _27: ();
let _28: ();
{
_1 = _8;
_6 = !9223372036854775807_isize;
_8 = _3 * _1;
_4.0 = _1;
Goto(bb1)
}
bb1 = {
_4 = (_3,);
_6 = -(-9223372036854775808_isize);
RET = '\u{e14c8}';
_2 = _5;
Call(_7 = fn2(_1, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = true as u32;
_5 = !_2;
RET = '\u{189d2}';
_9 = _4.0 * _3;
_6 = 9223372036854775807_isize + (-9223372036854775808_isize);
_7 = -_2;
_2 = -_7;
Goto(bb3)
}
bb3 = {
_4 = (_3,);
RET = '\u{24faf}';
_1 = _9 ^ _3;
_1 = !_8;
_4 = (_3,);
_1 = _3 & _9;
Call(_4.0 = core::intrinsics::bswap(_1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = '\u{495c1}';
_1 = !_9;
_2 = (-1483349596_i32) as i16;
_7 = -_5;
_4 = (_9,);
RET = '\u{74ba0}';
Goto(bb5)
}
bb5 = {
_11.2.0 = [(-126540402466054483439674892676129484617_i128)];
_5 = (-1824907082_i32) as i16;
_11.3.0 = _5;
Call(_3 = core::intrinsics::bswap(_1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_11.2.2.0 = -_2;
_6 = (-9223372036854775808_isize) | 9223372036854775807_isize;
_11.0 = _1 < _4.0;
_4.0 = !_9;
_11.2.2 = _11.3;
Goto(bb7)
}
bb7 = {
_9 = _3;
_4.0 = _6 as u32;
Goto(bb8)
}
bb8 = {
RET = '\u{f3cd3}';
_11.1 = 185403488878197683370488151072029699138_u128;
_6 = (-4894919208515074016_i64) as isize;
_4.0 = 2_usize as u32;
_8 = _1 * _1;
_11.3.0 = _7 - _7;
_11.3 = (_5,);
RET = '\u{78be}';
_11.3.0 = _2;
_5 = _11.3.0;
_12 = _7 as usize;
_11.2.1 = 47431_u16 >> _7;
_14 = [(-4852534031989590946_i64),2577473500506429746_i64,6088038546521592818_i64,(-2379181763911630296_i64),8542879920744990108_i64];
_4 = (_8,);
_5 = _2;
_13 = [_7,_7,_11.2.2.0,_11.3.0,_7];
_11.2.2.0 = _11.0 as i16;
_14 = [(-7460177441053576419_i64),6032484980732501871_i64,(-6740050971940813821_i64),(-6479574125237989191_i64),8736484626513431764_i64];
_8 = _3 ^ _3;
match _11.1 {
0 => bb7,
1 => bb2,
2 => bb5,
3 => bb4,
4 => bb9,
5 => bb10,
185403488878197683370488151072029699138 => bb12,
_ => bb11
}
}
bb9 = {
_4 = (_3,);
RET = '\u{24faf}';
_1 = _9 ^ _3;
_1 = !_8;
_4 = (_3,);
_1 = _3 & _9;
Call(_4.0 = core::intrinsics::bswap(_1), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
_8 = true as u32;
_5 = !_2;
RET = '\u{189d2}';
_9 = _4.0 * _3;
_6 = 9223372036854775807_isize + (-9223372036854775808_isize);
_7 = -_2;
_2 = -_7;
Goto(bb3)
}
bb11 = {
_4 = (_3,);
_6 = -(-9223372036854775808_isize);
RET = '\u{e14c8}';
_2 = _5;
Call(_7 = fn2(_1, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_15 = _11.0 ^ _11.0;
_14 = [267154479572469791_i64,(-3006188538864911736_i64),3418536824299499112_i64,543034313652941938_i64,7992707349748276510_i64];
_16 = &_11;
_11.2.0 = [(-109684131809605105143869292914251155112_i128)];
_11.2.1 = 11654_u16;
_17 = core::ptr::addr_of!((*_16).0);
_11.3 = ((*_16).2.2.0,);
_11.0 = _15 & _15;
_4.0 = _1;
_12 = RET as usize;
RET = '\u{48a26}';
_7 = _11.2.2.0;
_17 = core::ptr::addr_of!(_15);
_2 = _11.2.2.0;
Goto(bb13)
}
bb13 = {
(*_17) = !_11.0;
_11.2.2 = _11.3;
_13 = [_7,_7,_11.3.0,_11.2.2.0,_11.2.2.0];
_17 = core::ptr::addr_of!((*_17));
_11.2.2 = _11.3;
_20 = -_6;
RET = '\u{1056ba}';
(*_17) = _11.3.0 > _11.3.0;
_2 = -_7;
_7 = _2 * _11.3.0;
match _11.1 {
185403488878197683370488151072029699138 => bb14,
_ => bb8
}
}
bb14 = {
(*_17) = _11.0;
RET = '\u{103d0a}';
RET = '\u{e4862}';
_21.0 = _11.2;
_1 = !_4.0;
_4 = (_9,);
_12 = (-1384584046_i32) as usize;
_11.2.2.0 = _2;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(1_usize, 3_usize, Move(_3), 4_usize, Move(_4), 6_usize, Move(_6), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(1_usize, 12_usize, Move(_12), 11_usize, Move(_11), 7_usize, Move(_7), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: u32,mut _2: (u32,)) -> i16 {
mir! {
type RET = i16;
let _3: (Adt51, [i16; 5], &'static i128);
let _4: (&'static [i32; 3],);
let _5: f32;
let _6: (*mut (i16,), isize, f64, f64);
let _7: [u64; 6];
let _8: i8;
let _9: &'static (&'static [i32; 3],);
let _10: *const *const bool;
let _11: (Adt51, [i16; 5], &'static i128);
let _12: f64;
let _13: Adt71;
let _14: [usize; 5];
let _15: f32;
let _16: &'static Adt47;
let _17: [i16; 3];
let _18: (u32,);
let _19: isize;
let _20: ([i128; 1], char, f64);
let _21: (i64,);
let _22: u32;
let _23: [i128; 1];
let _24: (u128, *const bool, (u32,), [isize; 6]);
let _25: &'static Adt47;
let _26: ();
let _27: ();
{
_2 = (_1,);
_1 = _2.0;
RET = -(-10616_i16);
_1 = 4538090891541848624_i64 as u32;
_2.0 = !_1;
RET = (-6338694813367847702_i64) as i16;
_2.0 = _1;
_1 = !_2.0;
RET = 11190_u16 as i16;
_1 = !_2.0;
RET = (-30850_i16);
RET = !(-17115_i16);
RET = !1840_i16;
_2.0 = _1;
Call(_6.2 = fn3(RET, _1, _1, _2.0, _2, _2, _2.0, RET, RET, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2.0 = !_1;
_6.3 = (-79327596881434506215690925938116013755_i128) as f64;
_6.1 = 9_isize * 9223372036854775807_isize;
RET = (-96617412_i32) as i16;
Goto(bb2)
}
bb2 = {
_5 = (-1347663589_i32) as f32;
_2 = (_1,);
_7 = [4616961774710129176_u64,17932693602190258241_u64,14792479313380398220_u64,9897176106891377185_u64,1041664939935040970_u64,15669516654799333070_u64];
_3.1 = [RET,RET,RET,RET,RET];
_3.1 = [RET,RET,RET,RET,RET];
_2 = (_1,);
_1 = !_2.0;
Goto(bb3)
}
bb3 = {
RET = 13618_i16;
_1 = _6.3 as u32;
Goto(bb4)
}
bb4 = {
_8 = 74_i8;
RET = 11535_i16;
_6.1 = 1901043704081990469_i64 as isize;
_9 = &_4;
_5 = 1449022344753210891_i64 as f32;
_5 = 14162768414545682424_usize as f32;
_9 = &_4;
_5 = (-1896836692_i32) as f32;
_2.0 = !_1;
_2 = (_1,);
_3.1 = [RET,RET,RET,RET,RET];
_5 = RET as f32;
_5 = 3_usize as f32;
_8 = _5 as i8;
_9 = &(*_9);
match RET {
0 => bb1,
1 => bb5,
11535 => bb7,
_ => bb6
}
}
bb5 = {
_2.0 = !_1;
_6.3 = (-79327596881434506215690925938116013755_i128) as f64;
_6.1 = 9_isize * 9223372036854775807_isize;
RET = (-96617412_i32) as i16;
Goto(bb2)
}
bb6 = {
_5 = (-1347663589_i32) as f32;
_2 = (_1,);
_7 = [4616961774710129176_u64,17932693602190258241_u64,14792479313380398220_u64,9897176106891377185_u64,1041664939935040970_u64,15669516654799333070_u64];
_3.1 = [RET,RET,RET,RET,RET];
_3.1 = [RET,RET,RET,RET,RET];
_2 = (_1,);
_1 = !_2.0;
Goto(bb3)
}
bb7 = {
RET = true as i16;
_11.1 = [RET,RET,RET,RET,RET];
_1 = 87200846500715593700376768219344865378_i128 as u32;
RET = !25268_i16;
_1 = _2.0 * _2.0;
_1 = !_2.0;
_6.2 = _6.3 * _6.3;
RET = (-1496_i16) & 24514_i16;
_9 = &(*_9);
_6.2 = _6.3 - _6.3;
_2.0 = _1;
_6.1 = (-9223372036854775808_isize);
_6.1 = 9223372036854775807_isize * 9223372036854775807_isize;
_6.3 = _6.2;
_12 = -_6.3;
_2 = (_1,);
_9 = &(*_9);
RET = (-105620495656879321292886591978972305445_i128) as i16;
_14 = [0_usize,16513315857799159302_usize,3_usize,17309940108906983602_usize,4576037080515246353_usize];
_15 = _5 - _5;
_15 = _5 + _5;
RET = _1 as i16;
_3.1 = [RET,RET,RET,RET,RET];
RET = 9521280164669373784_usize as i16;
Goto(bb8)
}
bb8 = {
_2 = (_1,);
_7 = [11875577677414225682_u64,13856048814906845125_u64,11596897429775199933_u64,18272968201806560253_u64,7270776817121295466_u64,18358777485484716494_u64];
_1 = _2.0;
_5 = -_15;
_2.0 = _1 >> RET;
_9 = &(*_9);
_6.2 = -_6.3;
Goto(bb9)
}
bb9 = {
RET = 16940_i16 << _2.0;
_6.2 = _12;
_5 = -_15;
_7 = [14189144501945691958_u64,2750066604139174537_u64,2216284864128539469_u64,9649060894754201638_u64,17551321361687830645_u64,17723815674488797806_u64];
_9 = &_4;
RET = !(-22223_i16);
_6.1 = 9223372036854775807_isize;
_6.1 = 9223372036854775807_isize << _8;
_14 = [358728050969749968_usize,11401261273534874906_usize,1_usize,2_usize,3_usize];
RET = false as i16;
_17 = [RET,RET,RET];
_11.1 = [RET,RET,RET,RET,RET];
_17 = [RET,RET,RET];
_18 = _2;
_2.0 = !_18.0;
_6.3 = -_6.2;
_8 = 11_i8 + (-49_i8);
_6.3 = _12 + _12;
_19 = _6.1 * _6.1;
Goto(bb10)
}
bb10 = {
_6.1 = -_19;
_7 = [8554089397887550383_u64,9755038330800927207_u64,12784485472578367532_u64,7467981501991977219_u64,5797521734345707630_u64,15701605714811312712_u64];
_11.1 = [RET,RET,RET,RET,RET];
_6.2 = _6.1 as f64;
_11.1 = [RET,RET,RET,RET,RET];
_19 = _6.1 & _6.1;
_19 = _6.1 << _18.0;
_12 = _6.3;
_8 = (-96_i8);
_18.0 = (-139517434416333417345949993300978686682_i128) as u32;
_5 = _15 + _15;
_14 = [10366535693321760758_usize,7545228575591903946_usize,8256300095342636706_usize,3_usize,12794354737777528852_usize];
_6.3 = _6.2 + _12;
_18.0 = _1 ^ _2.0;
RET = 18125_i16;
_9 = &(*_9);
_19 = !_6.1;
_17 = [RET,RET,RET];
_8 = (-91_i8);
_20.1 = '\u{6a194}';
_8 = -70_i8;
_18.0 = 644739464_i32 as u32;
_11.1 = _3.1;
_20.0 = [48898609618043235923687101476767769742_i128];
_8 = 22_i8;
_1 = !_2.0;
match _8 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb8,
4 => bb5,
5 => bb11,
6 => bb12,
22 => bb14,
_ => bb13
}
}
bb11 = {
_5 = (-1347663589_i32) as f32;
_2 = (_1,);
_7 = [4616961774710129176_u64,17932693602190258241_u64,14792479313380398220_u64,9897176106891377185_u64,1041664939935040970_u64,15669516654799333070_u64];
_3.1 = [RET,RET,RET,RET,RET];
_3.1 = [RET,RET,RET,RET,RET];
_2 = (_1,);
_1 = !_2.0;
Goto(bb3)
}
bb12 = {
_2.0 = !_1;
_6.3 = (-79327596881434506215690925938116013755_i128) as f64;
_6.1 = 9_isize * 9223372036854775807_isize;
RET = (-96617412_i32) as i16;
Goto(bb2)
}
bb13 = {
_8 = 74_i8;
RET = 11535_i16;
_6.1 = 1901043704081990469_i64 as isize;
_9 = &_4;
_5 = 1449022344753210891_i64 as f32;
_5 = 14162768414545682424_usize as f32;
_9 = &_4;
_5 = (-1896836692_i32) as f32;
_2.0 = !_1;
_2 = (_1,);
_3.1 = [RET,RET,RET,RET,RET];
_5 = RET as f32;
_5 = 3_usize as f32;
_8 = _5 as i8;
_9 = &(*_9);
match RET {
0 => bb1,
1 => bb5,
11535 => bb7,
_ => bb6
}
}
bb14 = {
_3.1 = [RET,RET,RET,RET,RET];
_6.3 = _6.2;
_20.1 = '\u{1d7eb}';
_15 = _5 - _5;
_17 = [RET,RET,RET];
_11.1 = [RET,RET,RET,RET,RET];
_24.3 = [_6.1,_6.1,_19,_6.1,_6.1,_19];
_24.2 = (_1,);
_21 = (4097837993559563679_i64,);
_11.1 = [RET,RET,RET,RET,RET];
_20.1 = '\u{9017a}';
_5 = -_15;
_22 = _2.0 - _24.2.0;
_18.0 = !_22;
_20.1 = '\u{c2401}';
_10 = core::ptr::addr_of!(_24.1);
_2 = (_22,);
_24.0 = !237934945630014034187594728597803707372_u128;
_24.3 = [_6.1,_6.1,_19,_19,_19,_19];
_10 = core::ptr::addr_of!(_24.1);
_6.1 = _12 as isize;
_23 = _20.0;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(2_usize, 21_usize, Move(_21), 2_usize, Move(_2), 23_usize, Move(_23), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(2_usize, 19_usize, Move(_19), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: i16,mut _2: u32,mut _3: u32,mut _4: u32,mut _5: (u32,),mut _6: (u32,),mut _7: u32,mut _8: i16,mut _9: i16,mut _10: i16) -> f64 {
mir! {
type RET = f64;
let _11: (i64,);
let _12: bool;
let _13: u16;
let _14: bool;
let _15: &'static f64;
let _16: ([i128; 1], u16, (i16,));
let _17: *const bool;
let _18: [u8; 8];
let _19: Adt24;
let _20: i64;
let _21: *mut u32;
let _22: [i128; 1];
let _23: [i128; 1];
let _24: u64;
let _25: isize;
let _26: bool;
let _27: i8;
let _28: char;
let _29: [i32; 5];
let _30: &'static f64;
let _31: &'static &'static [i32; 3];
let _32: i64;
let _33: f64;
let _34: u32;
let _35: (*mut (i16,), isize, f64, f64);
let _36: *const Adt47;
let _37: isize;
let _38: [isize; 4];
let _39: [isize; 4];
let _40: ();
let _41: ();
{
_1 = _8;
Call(_4 = fn4(_1, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6.0 = !_4;
_3 = _6.0;
_2 = !_4;
_9 = _1 * _1;
RET = 38_i8 as f64;
_2 = _4 + _3;
_5 = _6;
RET = (-577015502_i32) as f64;
_8 = _10 - _9;
_2 = 31172_u16 as u32;
RET = 22_isize as f64;
_9 = false as i16;
_12 = true ^ true;
_1 = _8;
_5.0 = _3;
_13 = 43080_u16 & 11649_u16;
Goto(bb2)
}
bb2 = {
_3 = !_4;
_11 = (3665123938142135398_i64,);
_11.0 = 558423698_i32 as i64;
RET = _13 as f64;
RET = _6.0 as f64;
_14 = _12 ^ _12;
_1 = _9 ^ _8;
_1 = _8;
_11 = ((-67574008524151026_i64),);
_14 = _7 != _3;
_15 = &RET;
_18 = [188_u8,60_u8,199_u8,69_u8,18_u8,66_u8,101_u8,235_u8];
_8 = -_9;
_9 = _1 | _1;
_5 = (_7,);
_16.2 = (_9,);
_16.1 = _13;
_17 = core::ptr::addr_of!(_12);
_17 = core::ptr::addr_of!(_12);
_5 = (_3,);
_16.1 = _13 * _13;
(*_17) = _14;
_6.0 = _4 & _2;
match _11.0 {
340282366920938463463307033423244060430 => bb3,
_ => bb1
}
}
bb3 = {
_16.1 = _13 >> _1;
_11.0 = !7500198285614144956_i64;
_5.0 = !_6.0;
_3 = _6.0 - _6.0;
_7 = !_3;
_15 = &(*_15);
_12 = _7 <= _3;
_16.0 = [(-154325710490561605215580822690443382849_i128)];
_16.0 = [28108222660364307180616792980297429769_i128];
_11 = (4059854018438631629_i64,);
_17 = core::ptr::addr_of!(_12);
_17 = core::ptr::addr_of!(_14);
_12 = (*_17) | (*_17);
_12 = (*_17);
_16.0 = [169586385950164616746849823483592257624_i128];
_16.2 = (_9,);
_11 = ((-8494946954764888367_i64),);
_21 = core::ptr::addr_of_mut!(_2);
_11.0 = !(-7754953771404494597_i64);
_10 = 19_i8 as i16;
_9 = _1 + _8;
Call(_17 = fn15(), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_5 = (_3,);
_14 = !_12;
_20 = _11.0;
_5.0 = _4 >> _6.0;
RET = 1_usize as f64;
_24 = 9967631915486333827_u64;
_24 = !14096803779161417235_u64;
_5.0 = _4 - _6.0;
_5.0 = _7 + _7;
_9 = _1;
Goto(bb5)
}
bb5 = {
_16.2.0 = -_8;
_26 = _5.0 < _3;
_20 = _11.0;
RET = 269594681519971964094423558696729779372_u128 as f64;
_26 = !_14;
_11.0 = RET as i64;
_12 = _26;
_17 = core::ptr::addr_of!(_14);
_14 = _5.0 != _5.0;
_2 = _5.0 ^ _5.0;
(*_21) = 38_i8 as u32;
RET = _13 as f64;
_6.0 = _3 << _7;
_11 = (_20,);
_14 = _1 > _9;
_18 = [24_u8,93_u8,197_u8,208_u8,170_u8,112_u8,239_u8,226_u8];
_5 = (_6.0,);
_6 = (_4,);
_18 = [155_u8,229_u8,171_u8,92_u8,35_u8,35_u8,199_u8,97_u8];
_19 = Adt24::Variant2 { fld0: (-556777029_i32),fld1: 40786631322220282729913591495939855082_u128 };
_9 = _13 as i16;
_16.0 = [(-83277830779481704580726581114859095386_i128)];
_11.0 = _20;
(*_17) = !_26;
place!(Field::<i32>(Variant(_19, 2), 0)) = 839489245_i32;
_25 = RET as isize;
_7 = _3 >> _6.0;
_5 = (_7,);
RET = 9_i8 as f64;
place!(Field::<u128>(Variant(_19, 2), 1)) = 317490644537513230883017468968715596529_u128 << _6.0;
Goto(bb6)
}
bb6 = {
place!(Field::<u128>(Variant(_19, 2), 1)) = 277497692854369706872190077840193198580_u128;
_6.0 = _3 + _3;
_14 = _3 >= _6.0;
place!(Field::<i32>(Variant(_19, 2), 0)) = !1491470773_i32;
_24 = _8 as u64;
_11.0 = _20 | _20;
(*_17) = _5.0 <= _6.0;
_17 = core::ptr::addr_of!((*_17));
_25 = 9223372036854775807_isize;
_25 = _9 as isize;
_27 = 91_i8;
_17 = core::ptr::addr_of!(_14);
_15 = &RET;
_14 = _6.0 > _6.0;
_12 = !(*_17);
_3 = 12838328196486480754_usize as u32;
_18 = [48_u8,243_u8,87_u8,176_u8,245_u8,206_u8,183_u8,11_u8];
_15 = &(*_15);
(*_21) = _25 as u32;
match _27 {
0 => bb5,
1 => bb2,
2 => bb3,
91 => bb7,
_ => bb4
}
}
bb7 = {
_23 = [87744796118189446235701967344368918433_i128];
_17 = core::ptr::addr_of!((*_17));
_30 = &RET;
_32 = RET as i64;
SetDiscriminant(_19, 1);
match _27 {
91 => bb9,
_ => bb8
}
}
bb8 = {
place!(Field::<u128>(Variant(_19, 2), 1)) = 277497692854369706872190077840193198580_u128;
_6.0 = _3 + _3;
_14 = _3 >= _6.0;
place!(Field::<i32>(Variant(_19, 2), 0)) = !1491470773_i32;
_24 = _8 as u64;
_11.0 = _20 | _20;
(*_17) = _5.0 <= _6.0;
_17 = core::ptr::addr_of!((*_17));
_25 = 9223372036854775807_isize;
_25 = _9 as isize;
_27 = 91_i8;
_17 = core::ptr::addr_of!(_14);
_15 = &RET;
_14 = _6.0 > _6.0;
_12 = !(*_17);
_3 = 12838328196486480754_usize as u32;
_18 = [48_u8,243_u8,87_u8,176_u8,245_u8,206_u8,183_u8,11_u8];
_15 = &(*_15);
(*_21) = _25 as u32;
match _27 {
0 => bb5,
1 => bb2,
2 => bb3,
91 => bb7,
_ => bb4
}
}
bb9 = {
_16.1 = _13;
(*_17) = !_12;
_16.0 = [44947561995902540097201339535106355017_i128];
_16.1 = _13;
_22 = [(-23409825404030678086139710113294042900_i128)];
(*_21) = _5.0;
RET = _8 as f64;
_27 = 26_i8 * 60_i8;
_10 = -_9;
place!(Field::<bool>(Variant(_19, 1), 0)) = !_12;
_28 = '\u{664a2}';
_4 = _6.0;
_19 = Adt24::Variant2 { fld0: 1805502758_i32,fld1: 268796754253343503767461800621155423126_u128 };
(*_17) = !_12;
_30 = &RET;
_16.2.0 = _27 as i16;
_21 = core::ptr::addr_of_mut!(_7);
_24 = !1173184700511430053_u64;
_9 = _1;
_14 = _12;
_32 = -_11.0;
Goto(bb10)
}
bb10 = {
_28 = '\u{43e5d}';
_35.2 = -RET;
Goto(bb11)
}
bb11 = {
_23 = [(-112011228863247908676185133799982632906_i128)];
_25 = 265870921851775944398028430436407342473_u128 as isize;
_30 = &_35.2;
Goto(bb12)
}
bb12 = {
_35.0 = core::ptr::addr_of_mut!(_16.2);
RET = -(*_30);
_16.0 = [24371950579192954971706004046806504259_i128];
_35.1 = (*_17) as isize;
_24 = 7978168467056564476_u64 * 13715044989295543543_u64;
_38 = [_35.1,_35.1,_35.1,_35.1];
_33 = _20 as f64;
_16.2.0 = _8;
RET = -(*_30);
_15 = &_33;
_35.2 = -(*_15);
_30 = Move(_15);
_34 = !_6.0;
Goto(bb13)
}
bb13 = {
place!(Field::<i32>(Variant(_19, 2), 0)) = 1814161190_i32 & 1103488681_i32;
_27 = (-79_i8) * (-38_i8);
_28 = '\u{27378}';
_15 = &_35.3;
_2 = !_6.0;
(*_17) = _12 & _26;
_1 = _10;
Goto(bb14)
}
bb14 = {
_29 = [Field::<i32>(Variant(_19, 2), 0),Field::<i32>(Variant(_19, 2), 0),Field::<i32>(Variant(_19, 2), 0),Field::<i32>(Variant(_19, 2), 0),Field::<i32>(Variant(_19, 2), 0)];
_9 = _10 + _10;
_30 = &_33;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(3_usize, 14_usize, Move(_14), 20_usize, Move(_20), 12_usize, Move(_12), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(3_usize, 28_usize, Move(_28), 32_usize, Move(_32), 16_usize, Move(_16), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(3_usize, 6_usize, Move(_6), 13_usize, Move(_13), 3_usize, Move(_3), 11_usize, Move(_11)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(3_usize, 34_usize, Move(_34), 38_usize, Move(_38), 41_usize, _41, 41_usize, _41), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: i16,mut _2: u32) -> u32 {
mir! {
type RET = u32;
let _3: f64;
let _4: f64;
let _5: ([i64; 5], &'static Adt47);
let _6: (u128, *const bool, (u32,), [isize; 6]);
let _7: &'static Adt47;
let _8: u32;
let _9: i8;
let _10: u128;
let _11: isize;
let _12: &'static [i32; 3];
let _13: usize;
let _14: i8;
let _15: (bool, u128, ([i128; 1], u16, (i16,)), (i16,));
let _16: isize;
let _17: u16;
let _18: u16;
let _19: bool;
let _20: isize;
let _21: f64;
let _22: ([i16; 5], (bool, u128, ([i128; 1], u16, (i16,)), (i16,)), i128, &'static usize);
let _23: f64;
let _24: ();
let _25: ();
{
RET = _2 + _2;
RET = !_2;
RET = _2;
RET = 1084294345_i32 as u32;
Goto(bb1)
}
bb1 = {
_3 = 42_u8 as f64;
RET = !_2;
_3 = 14918690709993727547_u64 as f64;
_2 = (-9223372036854775808_isize) as u32;
_3 = (-136017078245472449704393500570332038758_i128) as f64;
RET = _2;
_2 = !RET;
RET = (-164608095427523480997878634025265279192_i128) as u32;
_1 = (-8564_i16);
_3 = (-9223372036854775808_isize) as f64;
_3 = (-69053407510406870458309660865823090927_i128) as f64;
_4 = -_3;
RET = !_2;
_2 = RET;
_1 = 10071734943311847701_usize as i16;
_5.0 = [2764294889439630582_i64,7388104317325520116_i64,7326446011146079651_i64,(-3703785707720808223_i64),(-7911543775090030047_i64)];
_6.3 = [(-9223372036854775808_isize),9223372036854775807_isize,(-80_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_8 = RET * _2;
RET = _8;
RET = !_8;
_6.2.0 = RET;
_3 = _4;
_4 = _3 - _3;
_2 = _8;
_1 = (-13287_i16) << _8;
Goto(bb2)
}
bb2 = {
_6.2 = (RET,);
_6.2.0 = _8;
_3 = _4;
_6.0 = 32232_u16 as u128;
_6.3 = [9223372036854775807_isize,125_isize,(-9223372036854775808_isize),9_isize,(-64_isize),16_isize];
_6.2.0 = _2 - _8;
_6.2.0 = _2;
_6.2 = (_2,);
_6.2 = (RET,);
_4 = -_3;
_5.0 = [7919811009743185375_i64,(-157755307903303939_i64),8481151034016154094_i64,3371764080437764143_i64,2598419945612577690_i64];
_6.0 = 53649807802140068371809892037613652577_u128 >> _1;
_6.2 = (_8,);
_9 = 222_u8 as i8;
RET = _6.2.0;
_10 = _6.0 * _6.0;
_4 = (-9223372036854775808_isize) as f64;
_11 = (-9_isize) - 33_isize;
_11 = -9223372036854775807_isize;
_4 = _3 * _3;
_6.2.0 = RET & _8;
_2 = !_6.2.0;
_9 = (-83_i8);
_2 = !RET;
_10 = _6.0 + _6.0;
_2 = _8 & _6.2.0;
Call(_6.2.0 = fn5(_6.0, _4, _5.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11 = (-9223372036854775808_isize);
_5.0 = [(-3984664440266766054_i64),5987260524464785675_i64,(-101208020238129052_i64),(-6025628846681681179_i64),(-5080971679190017186_i64)];
_6.0 = false as u128;
_11 = !(-17_isize);
match _9 {
0 => bb4,
340282366920938463463374607431768211373 => bb6,
_ => bb5
}
}
bb4 = {
_6.2 = (RET,);
_6.2.0 = _8;
_3 = _4;
_6.0 = 32232_u16 as u128;
_6.3 = [9223372036854775807_isize,125_isize,(-9223372036854775808_isize),9_isize,(-64_isize),16_isize];
_6.2.0 = _2 - _8;
_6.2.0 = _2;
_6.2 = (_2,);
_6.2 = (RET,);
_4 = -_3;
_5.0 = [7919811009743185375_i64,(-157755307903303939_i64),8481151034016154094_i64,3371764080437764143_i64,2598419945612577690_i64];
_6.0 = 53649807802140068371809892037613652577_u128 >> _1;
_6.2 = (_8,);
_9 = 222_u8 as i8;
RET = _6.2.0;
_10 = _6.0 * _6.0;
_4 = (-9223372036854775808_isize) as f64;
_11 = (-9_isize) - 33_isize;
_11 = -9223372036854775807_isize;
_4 = _3 * _3;
_6.2.0 = RET & _8;
_2 = !_6.2.0;
_9 = (-83_i8);
_2 = !RET;
_10 = _6.0 + _6.0;
_2 = _8 & _6.2.0;
Call(_6.2.0 = fn5(_6.0, _4, _5.0), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_3 = 42_u8 as f64;
RET = !_2;
_3 = 14918690709993727547_u64 as f64;
_2 = (-9223372036854775808_isize) as u32;
_3 = (-136017078245472449704393500570332038758_i128) as f64;
RET = _2;
_2 = !RET;
RET = (-164608095427523480997878634025265279192_i128) as u32;
_1 = (-8564_i16);
_3 = (-9223372036854775808_isize) as f64;
_3 = (-69053407510406870458309660865823090927_i128) as f64;
_4 = -_3;
RET = !_2;
_2 = RET;
_1 = 10071734943311847701_usize as i16;
_5.0 = [2764294889439630582_i64,7388104317325520116_i64,7326446011146079651_i64,(-3703785707720808223_i64),(-7911543775090030047_i64)];
_6.3 = [(-9223372036854775808_isize),9223372036854775807_isize,(-80_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_8 = RET * _2;
RET = _8;
RET = !_8;
_6.2.0 = RET;
_3 = _4;
_4 = _3 - _3;
_2 = _8;
_1 = (-13287_i16) << _8;
Goto(bb2)
}
bb6 = {
_14 = 49952_u16 as i8;
_4 = _3;
_10 = !_6.0;
_11 = 9223372036854775807_isize >> _2;
_13 = !11963345709078417831_usize;
_10 = _2 as u128;
_2 = RET;
_11 = (-9_isize);
_4 = _3 - _3;
_10 = _6.0;
_4 = -_3;
_11 = 48927_u16 as isize;
_6.1 = core::ptr::addr_of!(_15.0);
_15.2.1 = 55998_u16;
_15.2.2 = (_1,);
_4 = 9129586812512644933_i64 as f64;
_15.1 = 5_u8 as u128;
_8 = _6.2.0 - _6.2.0;
_4 = _3 * _3;
_17 = _15.2.1 << _8;
_6.2.0 = !_2;
match _9 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
340282366920938463463374607431768211373 => bb15,
_ => bb14
}
}
bb7 = {
_3 = 42_u8 as f64;
RET = !_2;
_3 = 14918690709993727547_u64 as f64;
_2 = (-9223372036854775808_isize) as u32;
_3 = (-136017078245472449704393500570332038758_i128) as f64;
RET = _2;
_2 = !RET;
RET = (-164608095427523480997878634025265279192_i128) as u32;
_1 = (-8564_i16);
_3 = (-9223372036854775808_isize) as f64;
_3 = (-69053407510406870458309660865823090927_i128) as f64;
_4 = -_3;
RET = !_2;
_2 = RET;
_1 = 10071734943311847701_usize as i16;
_5.0 = [2764294889439630582_i64,7388104317325520116_i64,7326446011146079651_i64,(-3703785707720808223_i64),(-7911543775090030047_i64)];
_6.3 = [(-9223372036854775808_isize),9223372036854775807_isize,(-80_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_8 = RET * _2;
RET = _8;
RET = !_8;
_6.2.0 = RET;
_3 = _4;
_4 = _3 - _3;
_2 = _8;
_1 = (-13287_i16) << _8;
Goto(bb2)
}
bb8 = {
_6.2 = (RET,);
_6.2.0 = _8;
_3 = _4;
_6.0 = 32232_u16 as u128;
_6.3 = [9223372036854775807_isize,125_isize,(-9223372036854775808_isize),9_isize,(-64_isize),16_isize];
_6.2.0 = _2 - _8;
_6.2.0 = _2;
_6.2 = (_2,);
_6.2 = (RET,);
_4 = -_3;
_5.0 = [7919811009743185375_i64,(-157755307903303939_i64),8481151034016154094_i64,3371764080437764143_i64,2598419945612577690_i64];
_6.0 = 53649807802140068371809892037613652577_u128 >> _1;
_6.2 = (_8,);
_9 = 222_u8 as i8;
RET = _6.2.0;
_10 = _6.0 * _6.0;
_4 = (-9223372036854775808_isize) as f64;
_11 = (-9_isize) - 33_isize;
_11 = -9223372036854775807_isize;
_4 = _3 * _3;
_6.2.0 = RET & _8;
_2 = !_6.2.0;
_9 = (-83_i8);
_2 = !RET;
_10 = _6.0 + _6.0;
_2 = _8 & _6.2.0;
Call(_6.2.0 = fn5(_6.0, _4, _5.0), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_11 = (-9223372036854775808_isize);
_5.0 = [(-3984664440266766054_i64),5987260524464785675_i64,(-101208020238129052_i64),(-6025628846681681179_i64),(-5080971679190017186_i64)];
_6.0 = false as u128;
_11 = !(-17_isize);
match _9 {
0 => bb4,
340282366920938463463374607431768211373 => bb6,
_ => bb5
}
}
bb10 = {
_6.2 = (RET,);
_6.2.0 = _8;
_3 = _4;
_6.0 = 32232_u16 as u128;
_6.3 = [9223372036854775807_isize,125_isize,(-9223372036854775808_isize),9_isize,(-64_isize),16_isize];
_6.2.0 = _2 - _8;
_6.2.0 = _2;
_6.2 = (_2,);
_6.2 = (RET,);
_4 = -_3;
_5.0 = [7919811009743185375_i64,(-157755307903303939_i64),8481151034016154094_i64,3371764080437764143_i64,2598419945612577690_i64];
_6.0 = 53649807802140068371809892037613652577_u128 >> _1;
_6.2 = (_8,);
_9 = 222_u8 as i8;
RET = _6.2.0;
_10 = _6.0 * _6.0;
_4 = (-9223372036854775808_isize) as f64;
_11 = (-9_isize) - 33_isize;
_11 = -9223372036854775807_isize;
_4 = _3 * _3;
_6.2.0 = RET & _8;
_2 = !_6.2.0;
_9 = (-83_i8);
_2 = !RET;
_10 = _6.0 + _6.0;
_2 = _8 & _6.2.0;
Call(_6.2.0 = fn5(_6.0, _4, _5.0), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_3 = 42_u8 as f64;
RET = !_2;
_3 = 14918690709993727547_u64 as f64;
_2 = (-9223372036854775808_isize) as u32;
_3 = (-136017078245472449704393500570332038758_i128) as f64;
RET = _2;
_2 = !RET;
RET = (-164608095427523480997878634025265279192_i128) as u32;
_1 = (-8564_i16);
_3 = (-9223372036854775808_isize) as f64;
_3 = (-69053407510406870458309660865823090927_i128) as f64;
_4 = -_3;
RET = !_2;
_2 = RET;
_1 = 10071734943311847701_usize as i16;
_5.0 = [2764294889439630582_i64,7388104317325520116_i64,7326446011146079651_i64,(-3703785707720808223_i64),(-7911543775090030047_i64)];
_6.3 = [(-9223372036854775808_isize),9223372036854775807_isize,(-80_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_8 = RET * _2;
RET = _8;
RET = !_8;
_6.2.0 = RET;
_3 = _4;
_4 = _3 - _3;
_2 = _8;
_1 = (-13287_i16) << _8;
Goto(bb2)
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
_18 = _17 / _15.2.1;
_15.3 = (_15.2.2.0,);
RET = !_2;
_4 = -_3;
_16 = _6.0 as isize;
_1 = _15.2.2.0 << _18;
_15.1 = _10 - _10;
_15.2.2.0 = -_1;
_15.3 = (_15.2.2.0,);
_1 = _15.2.2.0;
_2 = 59_u8 as u32;
_15.1 = !_10;
_6.1 = core::ptr::addr_of!(_15.0);
_15.1 = _6.0;
_5.0 = [8766374560609902692_i64,(-5418873592667133813_i64),3020592996051101725_i64,(-6800060451017637214_i64),5594810403747627762_i64];
_15.0 = _15.2.2.0 > _15.2.2.0;
_20 = 498215399_i32 as isize;
_6.2 = (_2,);
_18 = !_15.2.1;
_8 = RET;
_10 = 6702558937946858247_i64 as u128;
_4 = _3 + _3;
_16 = _11;
_2 = _6.2.0 | _8;
_8 = _2 | _2;
_22.1.2.2.0 = !_15.2.2.0;
_6.3 = [_11,_20,_16,_11,_16,_11];
RET = _8 >> _2;
Goto(bb16)
}
bb16 = {
Call(_24 = dump_var(4_usize, 17_usize, Move(_17), 20_usize, Move(_20), 16_usize, Move(_16), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_24 = dump_var(4_usize, 13_usize, Move(_13), 8_usize, Move(_8), 25_usize, _25, 25_usize, _25), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: u128,mut _2: f64,mut _3: [i64; 5]) -> u32 {
mir! {
type RET = u32;
let _4: *const *const bool;
let _5: usize;
let _6: [u64; 6];
let _7: u64;
let _8: char;
let _9: u32;
let _10: f32;
let _11: i32;
let _12: isize;
let _13: f64;
let _14: isize;
let _15: isize;
let _16: isize;
let _17: char;
let _18: *const [i64; 5];
let _19: (i64, [u32; 2]);
let _20: f64;
let _21: [i16; 3];
let _22: [i32; 5];
let _23: i16;
let _24: isize;
let _25: i8;
let _26: ([i64; 5], &'static Adt47);
let _27: [i32; 5];
let _28: ();
let _29: ();
{
_1 = !111482861130223428638388049740748769434_u128;
RET = 394300152_u32;
_2 = 36968157504311752617029376731469520868_i128 as f64;
RET = (-6011_i16) as u32;
RET = 2424730906_u32;
RET = !3378454432_u32;
RET = !3574385324_u32;
RET = 1903695039_u32 >> _1;
_2 = 738968514070749739_i64 as f64;
Call(_2 = fn6(_1, _1, _3, _1, _3, RET, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 3887760398_u32 - 3180066059_u32;
RET = !867564711_u32;
RET = 1215713851_u32;
_1 = 88705218788948884944051365317831626360_u128;
RET = 9223372036854775807_isize as u32;
match _1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
88705218788948884944051365317831626360 => bb8,
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
RET = 331212659_u32 << _1;
RET = !2187429343_u32;
RET = 3001853596_u32 | 2270962844_u32;
_6 = [6025283073266861209_u64,2821733520791264402_u64,10094122681017498093_u64,13458145629216225546_u64,3200584262636768749_u64,6461164340453565100_u64];
_5 = !5707147356548314825_usize;
_3 = [9174201994366590640_i64,(-440804024572382424_i64),(-3868192352321812166_i64),5625948891668154838_i64,(-6566010676392503860_i64)];
_6 = [2669465069174530447_u64,8551390012211175041_u64,1016611695017654218_u64,15302504267017747303_u64,9048943003803118766_u64,8355908031871837825_u64];
_3 = [8388661454041078154_i64,(-1793764508408558585_i64),1191747936580938881_i64,(-4907714211965240996_i64),261729452950021720_i64];
_7 = !8325122276137676555_u64;
_7 = 4535406130779486922_u64;
_1 = !70056948699476609718650425406074630446_u128;
_5 = 2_usize | 6_usize;
_5 = 7_usize;
RET = !2337372337_u32;
_8 = '\u{96fbd}';
RET = 3113574613_u32;
_7 = 16368826997436971045_u64 - 616575456595210102_u64;
_6 = [_7,_7,_7,_7,_7,_7];
_9 = !RET;
_10 = 240_u8 as f32;
RET = _9 * _9;
Goto(bb9)
}
bb9 = {
_11 = !1998744794_i32;
_12 = -(-9223372036854775808_isize);
_7 = 9545740755619908016_u64;
_13 = (-17_i8) as f64;
_12 = 13_isize ^ 9223372036854775807_isize;
_3 = [6118627649014665940_i64,7211383808863223680_i64,(-1706598990987028517_i64),(-1058237697497341408_i64),8431153689760411354_i64];
_6 = [_7,_7,_7,_7,_7,_7];
_14 = _12;
match _7 {
9545740755619908016 => bb10,
_ => bb6
}
}
bb10 = {
_15 = _12 * _12;
_8 = '\u{c59b5}';
_6 = [_7,_7,_7,_7,_7,_7];
_10 = 10429_u16 as f32;
RET = _2 as u32;
Goto(bb11)
}
bb11 = {
_11 = -(-802488534_i32);
_3 = [(-709523246723568444_i64),(-3232482295633152059_i64),4925314730668224527_i64,(-5562248001686803795_i64),1306883124418959032_i64];
_10 = _5 as f32;
RET = _9 * _9;
_15 = _12 >> RET;
_15 = false as isize;
_15 = _14 << _12;
_9 = 150_u8 as u32;
_1 = (-5801087733587051473_i64) as u128;
RET = !_9;
_17 = _8;
_12 = 46521_u16 as isize;
_12 = _15;
_8 = _17;
_9 = RET - RET;
_11 = _10 as i32;
_11 = _1 as i32;
_18 = core::ptr::addr_of!(_3);
(*_18) = [7864036940958469827_i64,(-4768682639057416595_i64),(-8043319028023715350_i64),(-4763133273233763526_i64),(-5918170740270094211_i64)];
_18 = core::ptr::addr_of!((*_18));
_13 = _2 * _2;
_19.1 = [_9,_9];
match _5 {
0 => bb8,
1 => bb2,
2 => bb4,
3 => bb12,
4 => bb13,
7 => bb15,
_ => bb14
}
}
bb12 = {
Return()
}
bb13 = {
_11 = !1998744794_i32;
_12 = -(-9223372036854775808_isize);
_7 = 9545740755619908016_u64;
_13 = (-17_i8) as f64;
_12 = 13_isize ^ 9223372036854775807_isize;
_3 = [6118627649014665940_i64,7211383808863223680_i64,(-1706598990987028517_i64),(-1058237697497341408_i64),8431153689760411354_i64];
_6 = [_7,_7,_7,_7,_7,_7];
_14 = _12;
match _7 {
9545740755619908016 => bb10,
_ => bb6
}
}
bb14 = {
RET = 331212659_u32 << _1;
RET = !2187429343_u32;
RET = 3001853596_u32 | 2270962844_u32;
_6 = [6025283073266861209_u64,2821733520791264402_u64,10094122681017498093_u64,13458145629216225546_u64,3200584262636768749_u64,6461164340453565100_u64];
_5 = !5707147356548314825_usize;
_3 = [9174201994366590640_i64,(-440804024572382424_i64),(-3868192352321812166_i64),5625948891668154838_i64,(-6566010676392503860_i64)];
_6 = [2669465069174530447_u64,8551390012211175041_u64,1016611695017654218_u64,15302504267017747303_u64,9048943003803118766_u64,8355908031871837825_u64];
_3 = [8388661454041078154_i64,(-1793764508408558585_i64),1191747936580938881_i64,(-4907714211965240996_i64),261729452950021720_i64];
_7 = !8325122276137676555_u64;
_7 = 4535406130779486922_u64;
_1 = !70056948699476609718650425406074630446_u128;
_5 = 2_usize | 6_usize;
_5 = 7_usize;
RET = !2337372337_u32;
_8 = '\u{96fbd}';
RET = 3113574613_u32;
_7 = 16368826997436971045_u64 - 616575456595210102_u64;
_6 = [_7,_7,_7,_7,_7,_7];
_9 = !RET;
_10 = 240_u8 as f32;
RET = _9 * _9;
Goto(bb9)
}
bb15 = {
_7 = !2412429679091884475_u64;
_3 = [6652902888695003720_i64,(-2879172335854607406_i64),(-2149419422643822472_i64),3113869534489205801_i64,1703300255467118667_i64];
_1 = 277286178634560530796926457857686533422_u128;
_13 = -_2;
_19.0 = !(-1916150410677097850_i64);
_24 = _9 as isize;
_3 = [_19.0,_19.0,_19.0,_19.0,_19.0];
_23 = _24 as i16;
_18 = core::ptr::addr_of!(_3);
_15 = _12 & _12;
_19.0 = 88052272116341834_i64;
_14 = -_12;
_2 = _13 - _13;
_17 = _8;
_25 = -(-91_i8);
_26.0 = [_19.0,_19.0,_19.0,_19.0,_19.0];
_11 = (-1117630522_i32) | 2017459007_i32;
_9 = !RET;
_20 = _13;
_25 = _23 as i8;
_22 = [_11,_11,_11,_11,_11];
_9 = _8 as u32;
Goto(bb16)
}
bb16 = {
Call(_28 = dump_var(5_usize, 1_usize, Move(_1), 12_usize, Move(_12), 9_usize, Move(_9), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(5_usize, 19_usize, Move(_19), 7_usize, Move(_7), 22_usize, Move(_22), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: u128,mut _2: u128,mut _3: [i64; 5],mut _4: u128,mut _5: [i64; 5],mut _6: u32,mut _7: [i64; 5]) -> f64 {
mir! {
type RET = f64;
let _8: &'static Adt42;
let _9: char;
let _10: isize;
let _11: [i32; 5];
let _12: f32;
let _13: u32;
let _14: Adt72;
let _15: ([u128; 5], u128, &'static usize, u32);
let _16: u128;
let _17: isize;
let _18: *const bool;
let _19: f64;
let _20: i32;
let _21: u32;
let _22: char;
let _23: *const [i128; 1];
let _24: &'static &'static [i32; 3];
let _25: f64;
let _26: Adt42;
let _27: [i64; 3];
let _28: [i128; 7];
let _29: bool;
let _30: [i128; 7];
let _31: f32;
let _32: (Adt51, [i16; 5], &'static i128);
let _33: (u128, *const bool, (u32,), [isize; 6]);
let _34: isize;
let _35: f64;
let _36: u32;
let _37: isize;
let _38: f64;
let _39: [i32; 3];
let _40: isize;
let _41: [u8; 8];
let _42: f32;
let _43: u32;
let _44: i16;
let _45: [i16; 5];
let _46: ([i128; 1], u16, (i16,));
let _47: *const [i64; 5];
let _48: &'static i128;
let _49: u16;
let _50: usize;
let _51: ();
let _52: ();
{
RET = _6 as f64;
_3 = [(-4864003372336901425_i64),(-5726307820317771390_i64),(-1568501751742209480_i64),8798915655166779622_i64,(-8505893641936405197_i64)];
Goto(bb1)
}
bb1 = {
_4 = (-6389395773975922519_i64) as u128;
RET = 2_usize as f64;
_3 = _5;
_3 = [(-5723304525889870067_i64),(-2636743039078682348_i64),(-274760792049662599_i64),(-6744713610095144815_i64),(-8152557841667317104_i64)];
RET = 33_u8 as f64;
_3 = _5;
RET = 6212_i16 as f64;
RET = (-2044399645144466029579048435938797588_i128) as f64;
_3 = [(-4981644229118070965_i64),6428748897403007721_i64,(-329890400847851882_i64),7401458412588331434_i64,(-6243732742902777213_i64)];
Call(_6 = fn7(_1, _7, _7, _3, _3, _3, _3, _4, _3, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = _3;
_1 = _2;
_1 = 19_i8 as u128;
_5 = [7437301636798969632_i64,7304572407498722274_i64,(-1361132990839862662_i64),(-792139834534159685_i64),(-1688499175499297765_i64)];
_6 = !2766206953_u32;
_6 = 38353204_u32 | 672499213_u32;
_6 = 7_usize as u32;
_7 = [7065297207898427485_i64,(-1368888728283542808_i64),9106998087481881942_i64,(-5779734459370084383_i64),5840731545038914844_i64];
_9 = '\u{3c7ab}';
_5 = [(-1288966545990297053_i64),753074061023936534_i64,(-2223864038654807355_i64),5367200037111630362_i64,7131747136656944762_i64];
_4 = _1 << _6;
_1 = _4;
Goto(bb3)
}
bb3 = {
_7 = _3;
RET = 4450889561585805457_u64 as f64;
_10 = 16_isize * (-8_isize);
_4 = !_1;
_5 = [(-5215595009063990411_i64),(-6675687335717950784_i64),2970284511359497474_i64,(-3607068381483332199_i64),3158239843881472857_i64];
_10 = 65_isize | 120_isize;
_2 = _1;
_3 = [(-5137943403511322711_i64),4421338675930349082_i64,6718296155564345530_i64,8312160519029165622_i64,(-7224577003509070717_i64)];
RET = _2 as f64;
_7 = [8806281831777443842_i64,7085074000484049043_i64,4085654092787529519_i64,8245210707612124632_i64,(-4881659431969024359_i64)];
RET = 2_u8 as f64;
_10 = _6 as isize;
RET = 34517_u16 as f64;
_4 = (-1105983162_i32) as u128;
_10 = (-9223372036854775808_isize) & (-2_isize);
_6 = (-121_i8) as u32;
_1 = _4;
Call(_1 = fn8(_9, _7), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1 = _4 | _2;
_2 = !_4;
_2 = _4 & _1;
_7 = [2206229334081221261_i64,3710580158967023746_i64,(-4706388730172597880_i64),(-5601605523019375159_i64),(-3841736397831640121_i64)];
RET = (-80_i8) as f64;
_1 = !_2;
_3 = [(-2960059384233538973_i64),885307099298148669_i64,(-5434064580592646889_i64),1242507628915905497_i64,569463085118452955_i64];
_9 = '\u{aec5d}';
Call(RET = fn12(_3, _7, _10, _2, _9, _9, _7, _9, _1, _9), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_11 = [98492660_i32,1425588783_i32,1105833760_i32,1534935407_i32,1271453723_i32];
_3 = [3087944141451062983_i64,5001983534162578529_i64,(-3996301915246221134_i64),(-9186604012963873956_i64),(-7727137836010925247_i64)];
_2 = _1 << _1;
RET = (-60_i8) as f64;
RET = 8251_u16 as f64;
_5 = [5044744537604854658_i64,(-6913981431281765835_i64),2627156440652730851_i64,(-7016920579153241174_i64),(-8804791356418688052_i64)];
Goto(bb6)
}
bb6 = {
_3 = [(-2942199026934984245_i64),(-4162093801936854763_i64),(-6768046321152009012_i64),(-1828634137127611310_i64),9188073391198908290_i64];
_12 = 80_u8 as f32;
_9 = '\u{14547}';
_12 = 218_u8 as f32;
_10 = -(-9223372036854775808_isize);
_15.1 = _9 as u128;
_15.1 = 34016_u16 as u128;
_3 = [3033923950095646464_i64,3677316883103199272_i64,3049870968966313135_i64,6194860395043727417_i64,2324643151036803797_i64];
_3 = [(-884967103661353017_i64),4644257904218088261_i64,2319733518386869466_i64,4845026545799853932_i64,463505180917355492_i64];
Goto(bb7)
}
bb7 = {
_15.3 = !_6;
_20 = 1363042990_i32 << _2;
_17 = _10 ^ _10;
_21 = !_15.3;
_15.0 = [_2,_2,_1,_2,_2];
_13 = !_6;
_16 = _2 >> _17;
Goto(bb8)
}
bb8 = {
_15.1 = _16;
_4 = _2 >> _20;
_2 = _9 as u128;
_9 = '\u{3ba66}';
_10 = 24810_u16 as isize;
_16 = !_15.1;
_10 = _17;
_2 = _4;
_4 = _16 >> _2;
Goto(bb9)
}
bb9 = {
RET = 4104573678043803936_usize as f64;
_7 = [6084658937113195276_i64,(-7240605651236689120_i64),(-3678360656191381081_i64),5319371389555220216_i64,(-725492661679296808_i64)];
_16 = _4;
_4 = _15.1;
_15.0 = [_2,_16,_15.1,_4,_2];
_19 = RET * RET;
_6 = !_13;
_5 = [(-5462346580691022256_i64),(-7829521332518869612_i64),(-8039993114910279960_i64),8821212953105315396_i64,(-7112725704082655992_i64)];
_21 = _13;
_19 = -RET;
_4 = _16;
_15.0 = [_16,_16,_2,_16,_4];
_4 = _16;
_2 = _16;
_11 = [_20,_20,_20,_20,_20];
_15.3 = _6;
_7 = [(-7170326841166603123_i64),(-6684536797397739197_i64),6853417253104764180_i64,7819843915534806359_i64,5542938713120092877_i64];
Goto(bb10)
}
bb10 = {
_3 = [2034748056112490384_i64,(-4171204124775977953_i64),(-4356487066374280884_i64),6778873488860025906_i64,2986265311457344090_i64];
_7 = [6756036222461055626_i64,8222743171104465729_i64,2084622251075726984_i64,(-7161263415814014304_i64),6079341820720700832_i64];
_8 = &_26;
_10 = -_17;
_13 = _2 as u32;
_4 = !_2;
_17 = _10 | _10;
_11 = [_20,_20,_20,_20,_20];
_1 = _4;
RET = _19 - _19;
_7 = [6960254567787393372_i64,3649610893620052768_i64,(-2884070122083857612_i64),4226941018268448462_i64,(-1699304364858232168_i64)];
_19 = RET - RET;
_9 = '\u{2efbe}';
_15.0 = [_4,_2,_1,_1,_1];
_2 = _16;
_15.3 = !_13;
RET = -_19;
_18 = core::ptr::addr_of!(_29);
(*_18) = !true;
Goto(bb11)
}
bb11 = {
_31 = -_12;
_17 = _10 << _13;
_15.1 = 9123418179079029795_i64 as u128;
_28 = [(-145316278625973159408586531175839159391_i128),(-58656203843620270487894019123366842426_i128),(-56318875011031257039379523331224357186_i128),(-57341507240298382611655641073975572531_i128),113807744621210289701523557992417353_i128,22412511450715301274928353171427951034_i128,117456117900257965371959474713332378062_i128];
_22 = _9;
RET = _19;
_27 = [6574662390928102385_i64,(-8581016328885959357_i64),(-4231262308992243068_i64)];
_3 = [(-6440293536527099087_i64),1417439146577984617_i64,(-1306246662601817290_i64),(-2891375505524113087_i64),(-3167518234114636881_i64)];
_18 = core::ptr::addr_of!(_29);
_25 = RET + RET;
_25 = -RET;
_16 = _1 * _1;
(*_18) = _1 != _16;
_15.1 = !_2;
_30 = [(-51552903150535074397941212049881678964_i128),(-50016359093171409499943112472504566479_i128),(-102110941439949622975557309316629417829_i128),161441875163811419453985240859518935648_i128,(-52904988835617714553746956508196860589_i128),88322331541227876196527408923851198451_i128,87703441054132078128094347276741796748_i128];
_17 = _10 + _10;
_29 = false;
RET = _19;
_9 = _22;
_15.3 = _13;
_33.3 = [_17,_10,_10,_10,_10,_17];
Call(_17 = fn14(_15.3, _4, _33.3, Move(_18), _16), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_33.2.0 = _15.1 as u32;
_16 = _4;
_16 = 178_u8 as u128;
_21 = !_13;
_21 = _13 - _13;
_34 = _19 as isize;
_32.1 = [18981_i16,(-29285_i16),(-5897_i16),4578_i16,9402_i16];
_35 = _25;
_7 = [602300881301601904_i64,2859548763665452995_i64,(-1345928627758481349_i64),(-9123497222723185954_i64),978297141268092304_i64];
Goto(bb13)
}
bb13 = {
_13 = !_21;
_37 = (-61525592026698906563414931353623028533_i128) as isize;
_36 = 9158_u16 as u32;
_29 = !false;
_35 = -RET;
_32.1 = [(-2705_i16),27635_i16,14664_i16,(-22275_i16),12784_i16];
_36 = _21;
Goto(bb14)
}
bb14 = {
_38 = _35;
_41 = [225_u8,23_u8,156_u8,8_u8,242_u8,133_u8,237_u8,218_u8];
_30 = _28;
_40 = _17 - _17;
RET = _38;
_33.2.0 = _15.3;
_15.1 = _29 as u128;
_4 = _1 - _1;
_2 = !_4;
_23 = core::ptr::addr_of!(_46.0);
_37 = -_17;
_46.2.0 = 327_i16 - 2445_i16;
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(6_usize, 17_usize, Move(_17), 41_usize, Move(_41), 28_usize, Move(_28), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(6_usize, 2_usize, Move(_2), 29_usize, Move(_29), 7_usize, Move(_7), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(6_usize, 9_usize, Move(_9), 36_usize, Move(_36), 27_usize, Move(_27), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: u128,mut _2: [i64; 5],mut _3: [i64; 5],mut _4: [i64; 5],mut _5: [i64; 5],mut _6: [i64; 5],mut _7: [i64; 5],mut _8: u128,mut _9: [i64; 5],mut _10: u128) -> u32 {
mir! {
type RET = u32;
let _11: u128;
let _12: u8;
let _13: Adt72;
let _14: isize;
let _15: char;
let _16: usize;
let _17: usize;
let _18: bool;
let _19: u32;
let _20: &'static i128;
let _21: [i16; 5];
let _22: i16;
let _23: isize;
let _24: ([i16; 5], (bool, u128, ([i128; 1], u16, (i16,)), (i16,)), i128, &'static usize);
let _25: i32;
let _26: u8;
let _27: &'static (bool, u128, ([i128; 1], u16, (i16,)), (i16,));
let _28: ();
let _29: ();
{
_9 = [(-3324547076535621512_i64),9222285245539610745_i64,4192030130632652582_i64,(-6495607118730058306_i64),(-896131504553712425_i64)];
_5 = [(-7757310300490787680_i64),(-7534143427819315053_i64),5195457302116724243_i64,(-4197157512959091155_i64),6330583871432594514_i64];
_8 = _10 - _10;
RET = 694349084_u32;
_9 = _4;
_6 = [7164885464145402338_i64,(-7234636432966366182_i64),5997819602740707568_i64,101546678946687774_i64,(-7143833332858150971_i64)];
_1 = _8;
_9 = [6827720750919187192_i64,(-2835532648695394894_i64),(-2310267813132580694_i64),(-8293620834493140728_i64),(-1493878635192746124_i64)];
_10 = (-5718653124995106256_i64) as u128;
_8 = 111479382463944970826644130733420395577_i128 as u128;
_14 = (-9223372036854775808_isize) >> RET;
_3 = _5;
_4 = [3616392841722541074_i64,2588472874377784418_i64,8731671882665003240_i64,2172796408236173800_i64,6635556749712654947_i64];
_4 = [4244473608863658341_i64,(-2097177934537725371_i64),1741042687126749799_i64,(-185438199231620954_i64),(-3268093673547023360_i64)];
_7 = [3588463642833273495_i64,(-7671782015434542311_i64),(-2447721670353509797_i64),(-939507188050023297_i64),(-1144908979318506370_i64)];
_4 = [2708299584908424147_i64,(-3349841656010911751_i64),7197761715684573672_i64,8355103639448512618_i64,455029985474844529_i64];
_2 = [(-9164393323507812923_i64),3023865583082222324_i64,1703316432178578547_i64,(-2519426537242183918_i64),(-1232975965743909119_i64)];
_12 = 121_u8 - 125_u8;
Goto(bb1)
}
bb1 = {
_11 = !_1;
_11 = _1;
RET = !3630718838_u32;
_9 = [2461237216404982151_i64,(-7864194532692954082_i64),3709938855892749437_i64,(-6707470275487645610_i64),4181412925866506438_i64];
RET = true as u32;
_15 = '\u{27c2d}';
_11 = _10;
_2 = [(-6595835026619688105_i64),5618599197538051675_i64,6643555840915944475_i64,375904309129713193_i64,1017866008052391807_i64];
_9 = [(-2653868460754297108_i64),(-1070332696486811635_i64),(-7410699382847784202_i64),2005750974913521647_i64,891632078631004208_i64];
_3 = [(-5520791003518326427_i64),(-7152620326522340323_i64),(-4870972273524080654_i64),(-1048698332816278028_i64),7759843726719168437_i64];
RET = 4003824985_u32;
RET = !3804840182_u32;
_6 = [4832451727095180252_i64,(-4155601095439511578_i64),(-6320530229951750441_i64),8469797259318485791_i64,4208646623762073185_i64];
_6 = [(-8983268229218084295_i64),(-7378429908512461122_i64),(-4677386056177077192_i64),(-6444682147875065544_i64),2848384352817766577_i64];
_10 = _8 * _8;
RET = !2081106680_u32;
_12 = 111_u8 * 206_u8;
_8 = _10;
Goto(bb2)
}
bb2 = {
_7 = _2;
_12 = _10 as u8;
_8 = true as u128;
RET = !3400241779_u32;
_7 = [(-5466577389268527351_i64),(-5052694802377953679_i64),(-1431435856467331827_i64),(-6179362829126785989_i64),2445697629729691906_i64];
_6 = _2;
_15 = '\u{9c1e9}';
RET = 622329259_u32 >> _14;
_9 = [(-6215017920166622162_i64),(-1493823906940027210_i64),(-1803656469841230664_i64),4991769211822292602_i64,4657358758164908491_i64];
_6 = _7;
_7 = [798054402611895098_i64,(-4372381270095723828_i64),7833081905638208885_i64,3349463549530089774_i64,7232189017952606988_i64];
_15 = '\u{9454c}';
_1 = _12 as u128;
_12 = 11_u8 ^ 186_u8;
_10 = !_8;
_17 = !0_usize;
_18 = !false;
_11 = _10 >> _14;
_14 = !9223372036854775807_isize;
_8 = _11 & _11;
_4 = _6;
Goto(bb3)
}
bb3 = {
_18 = false;
_12 = (-205373749_i32) as u8;
_14 = 9223372036854775807_isize;
_10 = !_11;
_21 = [6714_i16,(-25971_i16),(-6882_i16),31451_i16,32638_i16];
_8 = !_1;
_23 = _14 - _14;
_5 = [8624682072811696168_i64,(-5402781368911001582_i64),2901254571169048659_i64,115947777391066102_i64,(-5666484164766500634_i64)];
_14 = _12 as isize;
_16 = !_17;
_24.1.2.2.0 = !(-18425_i16);
_24.1.2.0 = [(-28375512034918170974565779589881877327_i128)];
_6 = [(-1495840851205357795_i64),(-79390212951541928_i64),(-3929289163633314370_i64),(-1411608605582377422_i64),(-6431393274597974041_i64)];
_24.1.2.0 = [(-71884285328450122071881686115751032119_i128)];
_7 = [3911126722313730894_i64,(-1451331225143868306_i64),849523602770931696_i64,(-2323598145034161069_i64),4548649977390042757_i64];
_24.1.2.2 = (8533_i16,);
_19 = RET & RET;
_16 = _17;
RET = _19 ^ _19;
_6 = _4;
_24.0 = _21;
_10 = !_11;
_5 = [(-8924787502390008200_i64),(-5091276602807054184_i64),(-5288489305468257177_i64),(-3924127318143106128_i64),5535966037298796950_i64];
_24.3 = &_17;
Goto(bb4)
}
bb4 = {
Call(_28 = dump_var(7_usize, 19_usize, Move(_19), 15_usize, Move(_15), 5_usize, Move(_5), 14_usize, Move(_14)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_28 = dump_var(7_usize, 23_usize, Move(_23), 10_usize, Move(_10), 3_usize, Move(_3), 4_usize, Move(_4)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_28 = dump_var(7_usize, 9_usize, Move(_9), 6_usize, Move(_6), 29_usize, _29, 29_usize, _29), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: char,mut _2: [i64; 5]) -> u128 {
mir! {
type RET = u128;
let _3: ([i16; 5], (bool, u128, ([i128; 1], u16, (i16,)), (i16,)), i128, &'static usize);
let _4: &'static *const *const bool;
let _5: Adt51;
let _6: (&'static [i32; 3],);
let _7: (*mut (i16,), isize, f64, f64);
let _8: u128;
let _9: *const (bool, u128, ([i128; 1], u16, (i16,)), (i16,));
let _10: bool;
let _11: char;
let _12: f32;
let _13: i128;
let _14: (i64,);
let _15: [u128; 5];
let _16: [i32; 8];
let _17: f32;
let _18: ();
let _19: ();
{
_3.1.3.0 = (-26892_i16);
_3.1.2.1 = !47049_u16;
_3.1.2.1 = 44334_u16 + 14625_u16;
_3.1.2.2.0 = _3.1.3.0 + _3.1.3.0;
_3.1.3.0 = _3.1.2.1 as i16;
_3.1.2.2.0 = _3.1.3.0 << _3.1.3.0;
_3.0 = [_3.1.3.0,_3.1.3.0,_3.1.2.2.0,_3.1.2.2.0,_3.1.2.2.0];
_3.1.2.1 = 18215232624123789138_u64 as u16;
_3.1.2.0 = [151418701249741172665620495455587262165_i128];
_3.1.0 = false;
RET = _3.1.2.2.0 as u128;
_1 = '\u{cb104}';
_3.1.2.2 = (_3.1.3.0,);
_3.1.2.1 = !49642_u16;
_3.1.3 = (_3.1.2.2.0,);
_3.1.1 = RET & RET;
_3.1.0 = false ^ true;
Goto(bb1)
}
bb1 = {
_2 = [5079181735765123690_i64,(-8227362265004543732_i64),1422098312199597017_i64,9196876723378755622_i64,6164716593561571226_i64];
_3.1.3 = (_3.1.2.2.0,);
_1 = '\u{9e1be}';
Goto(bb2)
}
bb2 = {
_3.1.2.0 = [136369715839571429543146034084855843176_i128];
_7.2 = 108_i8 as f64;
_3.0 = [_3.1.2.2.0,_3.1.2.2.0,_3.1.3.0,_3.1.2.2.0,_3.1.3.0];
_8 = _3.1.2.1 as u128;
_3.2 = -(-48081654545605685188314535154075877427_i128);
_3.1.1 = _8 * RET;
_3.1.2.2.0 = _3.1.3.0;
_7.2 = 15729147803423315641_u64 as f64;
RET = !_3.1.1;
_3.1.3 = (_3.1.2.2.0,);
_3.1.0 = !false;
_3.1.2.1 = 42546_u16 - 15261_u16;
_3.1.2.1 = 58845_u16;
_7.1 = (-90_isize);
_3.1.1 = _1 as u128;
_7.0 = core::ptr::addr_of_mut!(_3.1.3);
_3.1.2.2.0 = _3.1.3.0 & _3.1.3.0;
_7.3 = -_7.2;
_10 = _3.1.0;
_3.1.2.0 = [_3.2];
match _7.1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768211366 => bb8,
_ => bb7
}
}
bb3 = {
_2 = [5079181735765123690_i64,(-8227362265004543732_i64),1422098312199597017_i64,9196876723378755622_i64,6164716593561571226_i64];
_3.1.3 = (_3.1.2.2.0,);
_1 = '\u{9e1be}';
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
_3.1.2.2 = _3.1.3;
_8 = !RET;
_2 = [6823561138345555145_i64,(-2042282052040359022_i64),8118843714337012090_i64,(-2047115878400398766_i64),(-4837175968220306616_i64)];
_1 = '\u{4b225}';
_8 = RET | RET;
RET = _8 & _8;
RET = !_8;
_3.1.1 = !RET;
Goto(bb9)
}
bb9 = {
_12 = _7.1 as f32;
_9 = core::ptr::addr_of!(_3.1);
_3.1.1 = RET << _8;
_10 = !_3.1.0;
_3.1.2.2.0 = (*_9).3.0 + _3.1.3.0;
_3.1.3 = ((*_9).2.2.0,);
_1 = '\u{6f9cf}';
Goto(bb10)
}
bb10 = {
(*_9).2.2 = (*_9).3;
_3.1.3.0 = _1 as i16;
_3.1.0 = !_10;
(*_9).2.2.0 = (*_9).3.0 >> (*_9).1;
_9 = core::ptr::addr_of!((*_9));
_13 = !_3.2;
(*_9).2.0 = [_13];
_3.1.1 = _1 as u128;
_3.2 = (*_9).2.2.0 as i128;
_7.0 = core::ptr::addr_of_mut!((*_9).3);
_12 = _3.1.2.1 as f32;
_14.0 = 3371608305349612907_i64;
(*_9).2.0 = [_3.2];
RET = _8;
_3.0 = [(*_9).2.2.0,(*_9).2.2.0,_3.1.2.2.0,_3.1.2.2.0,_3.1.2.2.0];
RET = !_8;
_10 = !(*_9).0;
Call((*_9).0 = fn9(), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_14.0 = 8664217460938541477_usize as i64;
_3.1.2.1 = 65422_u16;
_3.1.3.0 = _3.1.2.2.0 * (*_9).2.2.0;
_9 = core::ptr::addr_of!((*_9));
_7.3 = _7.1 as f64;
_3.1.3.0 = (*_9).2.2.0;
_13 = _7.2 as i128;
(*_9).2.1 = !8258_u16;
_9 = core::ptr::addr_of!((*_9));
_7.3 = _7.2;
(*_9).2.2 = (_3.1.3.0,);
_3.2 = _13 + _13;
(*_9).2.0 = [_3.2];
_14 = ((-5719220219917720520_i64),);
_7.2 = -_7.3;
_9 = core::ptr::addr_of!((*_9));
(*_9).3.0 = _3.1.2.1 as i16;
_10 = !_3.1.0;
_3.1.0 = _10 ^ _10;
match _14.0 {
0 => bb8,
1 => bb7,
2 => bb4,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
340282366920938463457655387211850490936 => bb17,
_ => bb16
}
}
bb12 = {
_2 = [5079181735765123690_i64,(-8227362265004543732_i64),1422098312199597017_i64,9196876723378755622_i64,6164716593561571226_i64];
_3.1.3 = (_3.1.2.2.0,);
_1 = '\u{9e1be}';
Goto(bb2)
}
bb13 = {
Return()
}
bb14 = {
_3.1.2.2 = _3.1.3;
_8 = !RET;
_2 = [6823561138345555145_i64,(-2042282052040359022_i64),8118843714337012090_i64,(-2047115878400398766_i64),(-4837175968220306616_i64)];
_1 = '\u{4b225}';
_8 = RET | RET;
RET = _8 & _8;
RET = !_8;
_3.1.1 = !RET;
Goto(bb9)
}
bb15 = {
Return()
}
bb16 = {
_3.1.2.0 = [136369715839571429543146034084855843176_i128];
_7.2 = 108_i8 as f64;
_3.0 = [_3.1.2.2.0,_3.1.2.2.0,_3.1.3.0,_3.1.2.2.0,_3.1.3.0];
_8 = _3.1.2.1 as u128;
_3.2 = -(-48081654545605685188314535154075877427_i128);
_3.1.1 = _8 * RET;
_3.1.2.2.0 = _3.1.3.0;
_7.2 = 15729147803423315641_u64 as f64;
RET = !_3.1.1;
_3.1.3 = (_3.1.2.2.0,);
_3.1.0 = !false;
_3.1.2.1 = 42546_u16 - 15261_u16;
_3.1.2.1 = 58845_u16;
_7.1 = (-90_isize);
_3.1.1 = _1 as u128;
_7.0 = core::ptr::addr_of_mut!(_3.1.3);
_3.1.2.2.0 = _3.1.3.0 & _3.1.3.0;
_7.3 = -_7.2;
_10 = _3.1.0;
_3.1.2.0 = [_3.2];
match _7.1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768211366 => bb8,
_ => bb7
}
}
bb17 = {
_3.1.1 = 1762365642_u32 as u128;
(*_9).3.0 = 17513120966949099764_u64 as i16;
_3.0 = [(*_9).2.2.0,_3.1.2.2.0,(*_9).2.2.0,_3.1.2.2.0,_3.1.2.2.0];
Goto(bb18)
}
bb18 = {
Call(_18 = dump_var(8_usize, 2_usize, Move(_2), 8_usize, Move(_8), 14_usize, Move(_14), 19_usize, _19), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9() -> bool {
mir! {
type RET = bool;
let _1: &'static (&'static [i32; 3],);
let _2: (u128, *const bool, (u32,), [isize; 6]);
let _3: usize;
let _4: char;
let _5: &'static (&'static [i32; 3],);
let _6: *mut (i16,);
let _7: &'static Adt51;
let _8: i8;
let _9: &'static Adt51;
let _10: [i128; 1];
let _11: char;
let _12: isize;
let _13: usize;
let _14: bool;
let _15: *const (bool, u128, ([i128; 1], u16, (i16,)), (i16,));
let _16: ();
let _17: ();
{
RET = !true;
RET = !false;
RET = !true;
RET = true;
Goto(bb1)
}
bb1 = {
RET = !true;
RET = 168_u8 != 37_u8;
RET = 335723518670747562115621939274020622418_u128 <= 294130653696749287642321723477615821815_u128;
RET = !false;
Call(RET = fn10(), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = 71383967547372605137754070368841906689_i128 <= 88188981415275659032196899293788402188_i128;
RET = false;
RET = !false;
RET = false;
RET = !true;
RET = 4_isize <= 9223372036854775807_isize;
RET = !false;
RET = 32337_u16 != 22850_u16;
RET = !true;
RET = true;
RET = true | true;
RET = !false;
RET = true;
_2.2.0 = 3667027387_u32 ^ 2079631997_u32;
_2.0 = 278609854162705551926852491393734724161_u128 << _2.2.0;
RET = true;
_2.1 = core::ptr::addr_of!(RET);
_2.3 = [9223372036854775807_isize,(-35_isize),9223372036854775807_isize,29_isize,(-106_isize),(-9223372036854775808_isize)];
_3 = 4_usize;
_2.2.0 = 2509755853_u32 >> _2.0;
match _2.3[_3] {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
340282366920938463463374607431768211350 => bb10,
_ => bb9
}
}
bb3 = {
RET = !true;
RET = 168_u8 != 37_u8;
RET = 335723518670747562115621939274020622418_u128 <= 294130653696749287642321723477615821815_u128;
RET = !false;
Call(RET = fn10(), ReturnTo(bb2), UnwindUnreachable())
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
_2.2.0 = 1584169246234225115_u64 as u32;
_2.3 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),126_isize,(-9223372036854775808_isize)];
_4 = '\u{ea791}';
_3 = _2.2.0 as usize;
_2.2.0 = 2870942559_u32;
_2.2.0 = !96982811_u32;
RET = false;
RET = !false;
RET = true;
RET = true ^ false;
_2.2.0 = 3059861503_u32 >> _2.0;
Goto(bb11)
}
bb11 = {
_3 = 14797842900611159371_usize - 3_usize;
_2.2 = (2310437088_u32,);
_3 = _2.2.0 as usize;
_2.3 = [(-9223372036854775808_isize),(-9_isize),(-9223372036854775808_isize),(-5_isize),9223372036854775807_isize,73_isize];
_2.0 = 190_u8 as u128;
_2.1 = core::ptr::addr_of!(RET);
_4 = '\u{f1033}';
_4 = '\u{ac6e9}';
_2.0 = 177443210842128932425320168506267174932_u128;
_2.2.0 = 9096033515330562515_i64 as u32;
_2.3 = [(-96_isize),(-9223372036854775808_isize),(-17_isize),74_isize,9223372036854775807_isize,(-17_isize)];
RET = true | true;
_2.3 = [(-9223372036854775808_isize),(-18_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_2.2.0 = 1979985926_u32;
_2.2 = (4030353958_u32,);
_8 = 3353705088911986763_u64 as i8;
RET = _2.2.0 <= _2.2.0;
_2.1 = core::ptr::addr_of!(RET);
_3 = _4 as usize;
_2.1 = core::ptr::addr_of!(RET);
match _2.0 {
177443210842128932425320168506267174932 => bb12,
_ => bb2
}
}
bb12 = {
_2.2.0 = 1636638514_u32 << _3;
_10 = [(-68235946835367291961855077290011956508_i128)];
match _2.0 {
0 => bb13,
177443210842128932425320168506267174932 => bb15,
_ => bb14
}
}
bb13 = {
RET = !true;
RET = 168_u8 != 37_u8;
RET = 335723518670747562115621939274020622418_u128 <= 294130653696749287642321723477615821815_u128;
RET = !false;
Call(RET = fn10(), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
Return()
}
bb15 = {
RET = false;
_8 = (-31985_i16) as i8;
_2.2.0 = 3329452354_u32;
_11 = _4;
_2.1 = core::ptr::addr_of!(RET);
_10 = [95789422526111617570260033916835445917_i128];
RET = false;
_2.3 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-60_isize)];
_2.2.0 = 46848_u16 as u32;
Goto(bb16)
}
bb16 = {
Call(_16 = dump_var(9_usize, 3_usize, Move(_3), 4_usize, Move(_4), 17_usize, _17, 17_usize, _17), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10() -> bool {
mir! {
type RET = bool;
let _1: *const *const bool;
let _2: isize;
let _3: (Adt29, &'static u128, u64, u8);
let _4: [i32; 3];
let _5: (*mut (i16,), isize, f64, f64);
let _6: [i32; 8];
let _7: char;
let _8: (i64, [u32; 2]);
let _9: [i32; 8];
let _10: (u128, *const bool, (u32,), [isize; 6]);
let _11: Adt72;
let _12: isize;
let _13: Adt47;
let _14: [i32; 3];
let _15: char;
let _16: &'static Adt51;
let _17: [i128; 1];
let _18: i64;
let _19: isize;
let _20: i16;
let _21: isize;
let _22: u32;
let _23: Adt29;
let _24: &'static f64;
let _25: (i16,);
let _26: ();
let _27: ();
{
RET = true;
RET = !true;
RET = true;
RET = 9223372036854775807_isize == 9223372036854775807_isize;
RET = true;
RET = (-2722565563928686485_i64) != 532663858303925417_i64;
RET = 9223372036854775807_isize == (-9223372036854775808_isize);
RET = !true;
RET = !true;
_2 = (-117_isize);
RET = _2 < _2;
RET = _2 <= _2;
RET = _2 == _2;
RET = true | false;
RET = _2 < _2;
_2 = 9223372036854775807_isize;
RET = !false;
RET = false;
_3.2 = 16665554573092693302_u64;
RET = !false;
RET = false;
_3.3 = 109_u8;
RET = !true;
match _2 {
9223372036854775807 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_3.3 = !213_u8;
_3.3 = 1547341293_i32 as u8;
RET = true;
_4 = [1420579954_i32,573123031_i32,547362658_i32];
_2 = 9223372036854775807_isize | 92_isize;
RET = _3.2 > _3.2;
_5.1 = -_2;
_3.3 = 143_u8;
RET = !false;
_5.3 = 17364468056976356627_usize as f64;
RET = !false;
match _3.2 {
0 => bb1,
1 => bb3,
16665554573092693302 => bb5,
_ => bb4
}
}
bb3 = {
Return()
}
bb4 = {
Return()
}
bb5 = {
RET = true;
_6 = [(-1390232992_i32),1683896636_i32,(-1397619085_i32),(-1220618606_i32),(-670856843_i32),1731570871_i32,(-762993880_i32),(-1946427890_i32)];
_3.3 = 22_u8 >> _3.2;
_5.1 = _2 - _2;
_3.3 = 195_u8 & 187_u8;
_2 = 44736_u16 as isize;
RET = false;
_3.2 = 4758_u16 as u64;
_4 = [(-1528610792_i32),456171438_i32,1807480245_i32];
_2 = _5.1 >> _5.1;
_5.3 = _3.3 as f64;
_6 = [(-1879139140_i32),151817557_i32,1008500706_i32,(-1468156070_i32),466066932_i32,(-1839030107_i32),1361343880_i32,1330428629_i32];
_4 = [1951579532_i32,1883575002_i32,(-1647311551_i32)];
_6 = [(-1747828179_i32),(-1216305777_i32),(-1580020543_i32),(-856288560_i32),81369869_i32,716934323_i32,1107355413_i32,(-1400515927_i32)];
_5.1 = _5.3 as isize;
_4 = [(-1669688055_i32),845816570_i32,1756477687_i32];
Goto(bb6)
}
bb6 = {
_3.2 = 15945122534967946254_u64;
_5.2 = -_5.3;
_2 = _5.1 >> _3.2;
_2 = !_5.1;
_3.2 = 14602458678846525385_u64;
_5.1 = !_2;
_5.1 = _2;
_2 = _5.1;
match _3.2 {
0 => bb1,
1 => bb5,
2 => bb3,
14602458678846525385 => bb7,
_ => bb4
}
}
bb7 = {
_3.3 = 216_u8;
_5.2 = _5.3;
_7 = '\u{100c8a}';
_7 = '\u{58ca1}';
_5.2 = -_5.3;
RET = false;
_4 = [(-633757733_i32),891892996_i32,1206807759_i32];
_7 = '\u{1019b5}';
_8.1 = [3402893167_u32,2541282646_u32];
_8.0 = -(-4524382429579979123_i64);
_5.2 = _5.3 * _5.3;
_5.2 = -_5.3;
_7 = '\u{7b3fb}';
_5.2 = 1730256580_u32 as f64;
_9 = _6;
_3.3 = !64_u8;
_9 = [(-1391115034_i32),(-92081605_i32),1088595782_i32,1653792374_i32,(-790218179_i32),(-1944035326_i32),1632773080_i32,1230686755_i32];
_10.1 = core::ptr::addr_of!(RET);
Goto(bb8)
}
bb8 = {
_1 = core::ptr::addr_of!(_10.1);
_10.3 = [_5.1,_2,_5.1,_2,_5.1,_5.1];
_10.2.0 = !2082453315_u32;
_3.2 = !6560600331610123830_u64;
_14 = [333688344_i32,45959293_i32,(-257429808_i32)];
_9 = _6;
_3.1 = &_10.0;
_5.1 = _2;
_14 = [(-853625258_i32),(-87265845_i32),1669751312_i32];
(*_1) = core::ptr::addr_of!(RET);
Goto(bb9)
}
bb9 = {
_10.1 = core::ptr::addr_of!(RET);
_6 = _9;
_14 = [799530965_i32,(-139923520_i32),(-1989789248_i32)];
_5.3 = -_5.2;
_10.0 = _2 as u128;
_14 = [(-897503115_i32),(-1586989303_i32),626032957_i32];
_14 = [(-1119513165_i32),1523019787_i32,225362200_i32];
_8.0 = -(-2528510058538913359_i64);
_10.3 = [_2,_5.1,_2,_5.1,_2,_2];
_3.3 = 169_u8;
Goto(bb10)
}
bb10 = {
_5.3 = -_5.2;
(*_1) = core::ptr::addr_of!(RET);
(*_1) = core::ptr::addr_of!(RET);
_5.1 = _2;
_3.3 = 212_u8;
_7 = '\u{b97fe}';
_14 = [(-535963717_i32),1127482292_i32,(-288652957_i32)];
_8.0 = -5687605507401734785_i64;
_15 = _7;
_8.0 = (-2281867690361329646_i64) ^ (-8095816876004055653_i64);
_3.2 = 1698188241692106273_u64;
_10.2.0 = (-6156469916512814588247909824983511095_i128) as u32;
_7 = _15;
_8.1 = [_10.2.0,_10.2.0];
(*_1) = core::ptr::addr_of!(RET);
_5.2 = -_5.3;
_10.2.0 = !2372788304_u32;
_3.1 = &_10.0;
RET = !false;
_15 = _7;
_10.2.0 = !2334815317_u32;
RET = !true;
Goto(bb11)
}
bb11 = {
(*_1) = core::ptr::addr_of!(RET);
_2 = _5.1 * _5.1;
_12 = _5.2 as isize;
_5.1 = !_2;
Goto(bb12)
}
bb12 = {
_8.1 = [_10.2.0,_10.2.0];
_3.2 = 10756711269133027507_u64;
_5.3 = 13288830171014942718_usize as f64;
_4 = [171453997_i32,(-2037921858_i32),664022719_i32];
_1 = core::ptr::addr_of!(_10.1);
RET = false;
_8.1 = [_10.2.0,_10.2.0];
_12 = _2;
_10.3 = [_12,_5.1,_2,_12,_2,_2];
_3.1 = &_10.0;
_2 = _5.2 as isize;
_10.1 = core::ptr::addr_of!(RET);
_14 = [1106592226_i32,1611172206_i32,(-2017381471_i32)];
_18 = -_8.0;
_9 = _6;
_8.0 = _18;
_17 = [(-50319563448898356592792537497505354950_i128)];
_1 = core::ptr::addr_of!(_10.1);
_10.2.0 = 3393202231_u32 | 3288220869_u32;
_17 = [(-44716561734478682418664166206028516662_i128)];
_15 = _7;
RET = true;
RET = !false;
_10.3 = [_12,_2,_5.1,_2,_5.1,_12];
_8.0 = _18;
Call(_19 = core::intrinsics::bswap(_12), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_3.3 = 88_u8;
_3.2 = 16133121233729332743_u64;
_21 = -_2;
_8.1 = [_10.2.0,_10.2.0];
_15 = _7;
_5.3 = _5.2 - _5.2;
RET = _5.2 > _5.3;
_9 = [(-647147932_i32),111787331_i32,(-1998430185_i32),(-1418451382_i32),683596545_i32,1421720673_i32,1171553447_i32,(-151769989_i32)];
_15 = _7;
RET = _18 >= _18;
_3.1 = &_10.0;
_6 = [(-1020836046_i32),2135813567_i32,1364268551_i32,1441369707_i32,(-589696328_i32),(-1652356507_i32),61905306_i32,1542034787_i32];
_17 = [(-48609454419416094411846982124512357514_i128)];
_18 = !_8.0;
_22 = 121_i8 as u32;
_5.3 = _5.2 * _5.2;
_22 = _10.2.0 & _10.2.0;
_10.2 = (_22,);
_2 = -_5.1;
_10.1 = core::ptr::addr_of!(RET);
RET = false;
_6 = [855993403_i32,205091517_i32,1573693359_i32,1408817683_i32,2076922790_i32,(-1482674852_i32),(-1724760452_i32),(-1564147972_i32)];
_3.3 = 5_u8 + 14_u8;
_18 = !_8.0;
_1 = core::ptr::addr_of!(_10.1);
_15 = _7;
_10.0 = 212129846439509266366195694452764356784_u128 & 322566137645693971587615258401405856235_u128;
_10.2 = (_22,);
_9 = _6;
Call(_12 = fn11(_5.3, Move(_10), Move(_1)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_3.3 = 140_u8;
RET = false ^ false;
_6 = [1211575759_i32,(-2113905830_i32),(-79746124_i32),1785123578_i32,(-1430373308_i32),6036262_i32,1764303196_i32,1553341455_i32];
_15 = _7;
_3.2 = !4677666016584625385_u64;
_10.2.0 = !_22;
_10.0 = 224795122354909764621260897403356576289_u128;
_8.0 = !_18;
RET = !true;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(10_usize, 19_usize, Move(_19), 2_usize, Move(_2), 7_usize, Move(_7), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(10_usize, 22_usize, Move(_22), 9_usize, Move(_9), 17_usize, Move(_17), 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: f64,mut _2: (u128, *const bool, (u32,), [isize; 6]),mut _3: *const *const bool) -> isize {
mir! {
type RET = isize;
let _4: isize;
let _5: u32;
let _6: [isize; 4];
let _7: &'static (bool, u128, ([i128; 1], u16, (i16,)), (i16,));
let _8: [i16; 3];
let _9: ([i64; 5], &'static Adt47);
let _10: isize;
let _11: i16;
let _12: isize;
let _13: (i16,);
let _14: &'static u128;
let _15: Adt24;
let _16: *const (bool, u128, ([i128; 1], u16, (i16,)), (i16,));
let _17: [i32; 3];
let _18: i16;
let _19: char;
let _20: i16;
let _21: isize;
let _22: [i128; 7];
let _23: [u128; 1];
let _24: i128;
let _25: ();
let _26: ();
{
_3 = core::ptr::addr_of!(_2.1);
RET = (-9223372036854775808_isize);
_2.0 = 286302080721811094102420960299421975719_u128;
_2.0 = 44400_u16 as u128;
_2.2.0 = _1 as u32;
_2.3 = [RET,RET,RET,RET,RET,RET];
_2.2 = (2147685969_u32,);
_3 = core::ptr::addr_of!(_2.1);
_2.0 = 153277600382236444756102436948661191534_u128;
_2.2.0 = (-108_i8) as u32;
RET = false as isize;
_2.0 = 103734070420871629919886598057453230662_u128 + 81785096184441311452728399502596462832_u128;
_6 = [RET,RET,RET,RET];
_4 = RET;
RET = _4 - _4;
_4 = RET;
_5 = !_2.2.0;
_2.3 = [_4,RET,RET,_4,RET,RET];
_4 = RET;
RET = !_4;
_2.2.0 = !_5;
RET = _4 >> _4;
RET = -_4;
RET = !_4;
Goto(bb1)
}
bb1 = {
_1 = 7057_u16 as f64;
_2.2 = (_5,);
_2.2.0 = 4_i8 as u32;
_1 = _2.0 as f64;
_6 = [_4,RET,_4,_4];
_3 = core::ptr::addr_of!((*_3));
_5 = true as u32;
_1 = _5 as f64;
_4 = false as isize;
_2.2.0 = _5 * _5;
_5 = !_2.2.0;
_4 = RET * RET;
_2.2.0 = !_5;
_10 = 6_usize as isize;
_8 = [(-21387_i16),(-12078_i16),27882_i16];
_5 = _2.2.0 | _2.2.0;
_2.2 = (_5,);
_1 = _2.0 as f64;
_2.2.0 = _5 + _5;
_9.0 = [9202164271663521819_i64,6044310398240229415_i64,(-7154175905848769581_i64),8917854621814201991_i64,2722169534328347955_i64];
_2.0 = 225159875096466792135477031970216383001_u128;
match _2.0 {
0 => bb2,
225159875096466792135477031970216383001 => bb4,
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
_1 = 4_u8 as f64;
_13.0 = 9739_i16 - 19503_i16;
_3 = core::ptr::addr_of!(_2.1);
_2.3 = [_4,_4,_10,_4,RET,RET];
_13.0 = _1 as i16;
_2.2.0 = !_5;
_5 = (-375936578_i32) as u32;
Goto(bb5)
}
bb5 = {
_10 = _4;
_17 = [(-1961030725_i32),(-1400219095_i32),(-1296713093_i32)];
_18 = _13.0 * _13.0;
RET = _10 | _4;
_9.0 = [(-8990124980376713522_i64),(-4471997909306560228_i64),1615633991923361753_i64,(-839614509445578925_i64),(-9208046597417806773_i64)];
_19 = '\u{a1d07}';
_17 = [(-201084682_i32),(-972226879_i32),2130586838_i32];
_19 = '\u{6c20b}';
_18 = !_13.0;
_1 = _2.2.0 as f64;
_12 = RET;
_10 = 2_u8 as isize;
_2.3 = [RET,RET,_12,_12,_12,_4];
_2.0 = 122615138000581584165260610640336306447_u128;
match _2.0 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
122615138000581584165260610640336306447 => bb11,
_ => bb10
}
}
bb6 = {
_1 = 4_u8 as f64;
_13.0 = 9739_i16 - 19503_i16;
_3 = core::ptr::addr_of!(_2.1);
_2.3 = [_4,_4,_10,_4,RET,RET];
_13.0 = _1 as i16;
_2.2.0 = !_5;
_5 = (-375936578_i32) as u32;
Goto(bb5)
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
_1 = 7057_u16 as f64;
_2.2 = (_5,);
_2.2.0 = 4_i8 as u32;
_1 = _2.0 as f64;
_6 = [_4,RET,_4,_4];
_3 = core::ptr::addr_of!((*_3));
_5 = true as u32;
_1 = _5 as f64;
_4 = false as isize;
_2.2.0 = _5 * _5;
_5 = !_2.2.0;
_4 = RET * RET;
_2.2.0 = !_5;
_10 = 6_usize as isize;
_8 = [(-21387_i16),(-12078_i16),27882_i16];
_5 = _2.2.0 | _2.2.0;
_2.2 = (_5,);
_1 = _2.0 as f64;
_2.2.0 = _5 + _5;
_9.0 = [9202164271663521819_i64,6044310398240229415_i64,(-7154175905848769581_i64),8917854621814201991_i64,2722169534328347955_i64];
_2.0 = 225159875096466792135477031970216383001_u128;
match _2.0 {
0 => bb2,
225159875096466792135477031970216383001 => bb4,
_ => bb3
}
}
bb10 = {
Return()
}
bb11 = {
_21 = RET & RET;
_3 = core::ptr::addr_of!((*_3));
_18 = _19 as i16;
_22 = [144782968786088687006596754617323969201_i128,102424265782257451613746695929696783413_i128,41449191341253763221053771758772519289_i128,129391383198462412611178592285387545956_i128,137380542502873254936262377053503927729_i128,(-14651625303943780280374479169253874469_i128),(-80195922255979909841513921071109963920_i128)];
_12 = 4_usize as isize;
_3 = core::ptr::addr_of!((*_3));
_21 = _4 - RET;
_19 = '\u{e0c7e}';
_18 = 233_u8 as i16;
RET = !_21;
_11 = 64_i8 as i16;
_14 = &_2.0;
_2.3 = [_21,RET,_21,_4,RET,RET];
_3 = core::ptr::addr_of!(_2.1);
_17 = [1745391840_i32,66863958_i32,1374267174_i32];
RET = !_21;
RET = _21 * _21;
_4 = (-971302160_i32) as isize;
_22 = [(-143364325108538280533378419588085100366_i128),45158304087595136811637546452199408031_i128,(-9652116102588095487602912470536143166_i128),(-5120769443987469217727554466810054961_i128),63327593476655743712068828407278327034_i128,(-5064535483355197570809408417567195917_i128),(-35094606841811035918299886634062444382_i128)];
Goto(bb12)
}
bb12 = {
Call(_25 = dump_var(11_usize, 22_usize, Move(_22), 13_usize, Move(_13), 12_usize, Move(_12), 6_usize, Move(_6)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_25 = dump_var(11_usize, 19_usize, Move(_19), 10_usize, Move(_10), 26_usize, _26, 26_usize, _26), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: [i64; 5],mut _2: [i64; 5],mut _3: isize,mut _4: u128,mut _5: char,mut _6: char,mut _7: [i64; 5],mut _8: char,mut _9: u128,mut _10: char) -> f64 {
mir! {
type RET = f64;
let _11: bool;
let _12: [u128; 1];
let _13: [i64; 3];
let _14: i8;
let _15: f64;
let _16: u8;
let _17: bool;
let _18: u64;
let _19: &'static Adt42;
let _20: [i128; 7];
let _21: u128;
let _22: isize;
let _23: [i16; 3];
let _24: ([i128; 1], char, f64);
let _25: bool;
let _26: u128;
let _27: &'static Adt47;
let _28: *const &'static usize;
let _29: i64;
let _30: f64;
let _31: [isize; 4];
let _32: &'static u8;
let _33: ([i128; 1], char, f64);
let _34: f32;
let _35: f64;
let _36: bool;
let _37: u64;
let _38: [isize; 4];
let _39: i32;
let _40: i64;
let _41: ();
let _42: ();
{
_7 = [7061914407144119637_i64,4128952043757666434_i64,(-5506643294544809583_i64),4443985588038871075_i64,5442549643788315906_i64];
_4 = 91_u8 as u128;
_5 = _8;
_1 = [2383691662951973032_i64,2609944141788755026_i64,6546373877472246379_i64,652338386497688515_i64,(-4404216302973589475_i64)];
_10 = _5;
_9 = !_4;
Goto(bb1)
}
bb1 = {
_5 = _6;
_2 = [(-6920145073424073552_i64),1191307876717685252_i64,(-9159411838817193043_i64),7190922386385160612_i64,(-1644146232991896499_i64)];
_11 = _5 == _10;
RET = 4213521673908649487_usize as f64;
_7 = [(-7296950847070535109_i64),579497000563469320_i64,7796237747399938542_i64,(-1833493690677717569_i64),(-7947875369539941521_i64)];
RET = _4 as f64;
_13 = [(-7057472940212269312_i64),(-2339236265003510905_i64),1937918599339717280_i64];
_12 = [_4];
_5 = _10;
RET = _3 as f64;
_1 = _7;
RET = _3 as f64;
_8 = _10;
_5 = _10;
_10 = _5;
_5 = _8;
_10 = _5;
_7 = [(-7850087164499876023_i64),1651536713642001367_i64,1736304919549609501_i64,5013798372938346395_i64,(-5241598687017457663_i64)];
_16 = 154_u8 | 10_u8;
_7 = _2;
_3 = -(-9223372036854775808_isize);
Goto(bb2)
}
bb2 = {
_4 = !_9;
_10 = _8;
_17 = _11;
_15 = -RET;
_9 = _4 ^ _4;
_16 = !230_u8;
_9 = _4;
_3 = (-9223372036854775808_isize);
_9 = 5241126767071821140_usize as u128;
_4 = _9 << _16;
_16 = 187_u8 | 211_u8;
_14 = (-77_i8) & (-123_i8);
_8 = _6;
_4 = !_9;
_17 = _11 & _11;
_7 = [3511423552966996994_i64,3579103227407390683_i64,(-2610464464368224407_i64),5797203721243695914_i64,(-6586309389208611726_i64)];
_2 = [(-3208230910870392628_i64),8715281255274613855_i64,6561398691483034737_i64,3352219333489265498_i64,5768746157428634863_i64];
_18 = 11124119396357681398_u64;
_8 = _6;
_11 = _5 > _5;
_20 = [(-66879397234941889444109501566608106557_i128),(-103563487299221426489469224058935849079_i128),(-73245636266429129639816415491110731816_i128),(-89631040195821490601267432593665438914_i128),108406000142400221362925947813941362318_i128,(-115797603311179153893881230752933090437_i128),107962976833583426616302672395711554823_i128];
_12 = [_4];
_16 = _3 as u8;
_1 = _7;
RET = _15;
match _18 {
0 => bb1,
1 => bb3,
11124119396357681398 => bb5,
_ => bb4
}
}
bb3 = {
_5 = _6;
_2 = [(-6920145073424073552_i64),1191307876717685252_i64,(-9159411838817193043_i64),7190922386385160612_i64,(-1644146232991896499_i64)];
_11 = _5 == _10;
RET = 4213521673908649487_usize as f64;
_7 = [(-7296950847070535109_i64),579497000563469320_i64,7796237747399938542_i64,(-1833493690677717569_i64),(-7947875369539941521_i64)];
RET = _4 as f64;
_13 = [(-7057472940212269312_i64),(-2339236265003510905_i64),1937918599339717280_i64];
_12 = [_4];
_5 = _10;
RET = _3 as f64;
_1 = _7;
RET = _3 as f64;
_8 = _10;
_5 = _10;
_10 = _5;
_5 = _8;
_10 = _5;
_7 = [(-7850087164499876023_i64),1651536713642001367_i64,1736304919549609501_i64,5013798372938346395_i64,(-5241598687017457663_i64)];
_16 = 154_u8 | 10_u8;
_7 = _2;
_3 = -(-9223372036854775808_isize);
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
_6 = _10;
_1 = _2;
_14 = !(-5_i8);
_15 = -RET;
_7 = [(-7561278076637031074_i64),6462651325057147753_i64,2367612055006537680_i64,2070607251811461779_i64,1539625151106370362_i64];
_2 = [6028781032299171944_i64,6590392620938823746_i64,427180684945686279_i64,5362159265361460062_i64,(-123291595237751523_i64)];
_16 = 64852_u16 as u8;
_7 = [4714214885011757550_i64,4486324594084052771_i64,5029345852124256229_i64,6887210060541719513_i64,1941580328849338829_i64];
_2 = [(-2293152050928300788_i64),(-4388229772451048275_i64),4299623531348374200_i64,2481746217208335264_i64,(-215991594209179867_i64)];
_12 = [_9];
_18 = 1_usize as u64;
RET = (-68089668332148254427492577341411124561_i128) as f64;
Call(_16 = core::intrinsics::transmute(_17), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_6 = _5;
_2 = [(-8752714503522350333_i64),(-4001565069158778803_i64),(-6569760059945847811_i64),3273558981563756736_i64,(-3963696368595610309_i64)];
_21 = _14 as u128;
_16 = !75_u8;
_12 = [_21];
_4 = !_9;
_18 = 1549186472943045950_u64 + 13377327416626601300_u64;
_11 = _17;
_8 = _5;
_16 = 203_u8 ^ 84_u8;
_2 = [818813731821790781_i64,(-9015167737339233203_i64),(-3933310843509490799_i64),(-8985918917624367158_i64),(-287981060184427877_i64)];
_21 = !_9;
_17 = _16 < _16;
_18 = 13937602339482524260_u64;
_10 = _8;
_1 = [3241235812185625700_i64,(-817037254800761804_i64),3048721261881806883_i64,1683568427060606414_i64,(-6780563137477079791_i64)];
_5 = _8;
_24.1 = _10;
_24.0 = [(-151339856950647215820041303802163818982_i128)];
match _3 {
0 => bb5,
1 => bb7,
2 => bb8,
340282366920938463454151235394913435648 => bb10,
_ => bb9
}
}
bb7 = {
_6 = _10;
_1 = _2;
_14 = !(-5_i8);
_15 = -RET;
_7 = [(-7561278076637031074_i64),6462651325057147753_i64,2367612055006537680_i64,2070607251811461779_i64,1539625151106370362_i64];
_2 = [6028781032299171944_i64,6590392620938823746_i64,427180684945686279_i64,5362159265361460062_i64,(-123291595237751523_i64)];
_16 = 64852_u16 as u8;
_7 = [4714214885011757550_i64,4486324594084052771_i64,5029345852124256229_i64,6887210060541719513_i64,1941580328849338829_i64];
_2 = [(-2293152050928300788_i64),(-4388229772451048275_i64),4299623531348374200_i64,2481746217208335264_i64,(-215991594209179867_i64)];
_12 = [_9];
_18 = 1_usize as u64;
RET = (-68089668332148254427492577341411124561_i128) as f64;
Call(_16 = core::intrinsics::transmute(_17), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
_4 = !_9;
_10 = _8;
_17 = _11;
_15 = -RET;
_9 = _4 ^ _4;
_16 = !230_u8;
_9 = _4;
_3 = (-9223372036854775808_isize);
_9 = 5241126767071821140_usize as u128;
_4 = _9 << _16;
_16 = 187_u8 | 211_u8;
_14 = (-77_i8) & (-123_i8);
_8 = _6;
_4 = !_9;
_17 = _11 & _11;
_7 = [3511423552966996994_i64,3579103227407390683_i64,(-2610464464368224407_i64),5797203721243695914_i64,(-6586309389208611726_i64)];
_2 = [(-3208230910870392628_i64),8715281255274613855_i64,6561398691483034737_i64,3352219333489265498_i64,5768746157428634863_i64];
_18 = 11124119396357681398_u64;
_8 = _6;
_11 = _5 > _5;
_20 = [(-66879397234941889444109501566608106557_i128),(-103563487299221426489469224058935849079_i128),(-73245636266429129639816415491110731816_i128),(-89631040195821490601267432593665438914_i128),108406000142400221362925947813941362318_i128,(-115797603311179153893881230752933090437_i128),107962976833583426616302672395711554823_i128];
_12 = [_4];
_16 = _3 as u8;
_1 = _7;
RET = _15;
match _18 {
0 => bb1,
1 => bb3,
11124119396357681398 => bb5,
_ => bb4
}
}
bb10 = {
_23 = [(-3368_i16),(-13312_i16),(-30435_i16)];
_5 = _8;
_15 = -RET;
_24.0 = [(-106925673811001658303187315362915146533_i128)];
_24.2 = 146745479_i32 as f64;
_18 = !10381777647166111076_u64;
_3 = (-107_isize) << _4;
RET = -_24.2;
_25 = !_11;
_24.2 = RET + _15;
_7 = _1;
_24.1 = _5;
_5 = _6;
_13 = [93439936909284257_i64,1635102934193516667_i64,7425363309162976368_i64];
_6 = _24.1;
_24.2 = _15;
_23 = [27264_i16,(-24529_i16),(-10901_i16)];
RET = -_15;
_4 = _15 as u128;
_22 = _3 & _3;
_3 = _22 >> _22;
_1 = [5812392276133395319_i64,(-5259780279428609499_i64),(-4440278879338630599_i64),8017091060499812014_i64,2302994355423098441_i64];
_16 = 126_u8 & 81_u8;
_29 = 831855175212256066_i64 + (-6146273811508860634_i64);
_1 = [_29,_29,_29,_29,_29];
Call(_18 = fn13(_25, _3, _20, _23, _17, _16, _23, _24.1, _23, _3), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_4 = _9 + _9;
_14 = (-103_i8);
_8 = _10;
Goto(bb12)
}
bb12 = {
_13 = [_29,_29,_29];
_14 = 32_i8 - 89_i8;
RET = 62020_u16 as f64;
_20 = [(-63763246965218545451579166510354504465_i128),(-80611824229128453088780733992044837763_i128),93114751761994642006461818360956469398_i128,72868336022368761122514626540030499780_i128,(-55877711278012810770609510118566531068_i128),1124181993881232008859698042244453223_i128,(-22506321931223509019456802253223635611_i128)];
Goto(bb13)
}
bb13 = {
_22 = _3;
_17 = _11;
_15 = _24.2;
_10 = _5;
_29 = -7282117236321381655_i64;
_8 = _6;
_7 = _1;
_30 = _24.2 * RET;
_24.2 = (-20592_i16) as f64;
_7 = [_29,_29,_29,_29,_29];
RET = -_30;
_3 = _22 & _22;
_34 = _3 as f32;
_10 = _5;
_30 = _15;
_16 = _21 as u8;
_23 = [(-2090_i16),(-20203_i16),(-22888_i16)];
_14 = -78_i8;
_25 = !_11;
_1 = [_29,_29,_29,_29,_29];
_34 = _22 as f32;
Goto(bb14)
}
bb14 = {
_33.2 = 426951463_u32 as f64;
_33 = (_24.0, _24.1, _24.2);
RET = 17144_i16 as f64;
_1 = [_29,_29,_29,_29,_29];
_1 = [_29,_29,_29,_29,_29];
_1 = [_29,_29,_29,_29,_29];
_33.2 = -RET;
_31 = [_3,_3,_3,_22];
_26 = !_4;
_35 = _22 as f64;
_17 = _11;
_9 = !_4;
_37 = !_18;
_35 = (-59734720499670311307888606219197267208_i128) as f64;
_5 = _10;
_6 = _33.1;
_17 = !_11;
_38 = [_3,_22,_3,_3];
_33.2 = RET * _35;
_5 = _24.1;
_4 = _14 as u128;
_40 = _29 - _29;
_36 = _5 > _33.1;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(12_usize, 14_usize, Move(_14), 22_usize, Move(_22), 40_usize, Move(_40), 38_usize, Move(_38)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(12_usize, 8_usize, Move(_8), 5_usize, Move(_5), 31_usize, Move(_31), 36_usize, Move(_36)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(12_usize, 21_usize, Move(_21), 23_usize, Move(_23), 9_usize, Move(_9), 20_usize, Move(_20)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(12_usize, 16_usize, Move(_16), 26_usize, Move(_26), 42_usize, _42, 42_usize, _42), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: bool,mut _2: isize,mut _3: [i128; 7],mut _4: [i16; 3],mut _5: bool,mut _6: u8,mut _7: [i16; 3],mut _8: char,mut _9: [i16; 3],mut _10: isize) -> u64 {
mir! {
type RET = u64;
let _11: Adt24;
let _12: i128;
let _13: (bool, u128, ([i128; 1], u16, (i16,)), (i16,));
let _14: isize;
let _15: &'static f64;
let _16: isize;
let _17: *mut (i16,);
let _18: (u128, *const bool, (u32,), [isize; 6]);
let _19: *mut (i16,);
let _20: f64;
let _21: u8;
let _22: u8;
let _23: bool;
let _24: ([u128; 5], u128, &'static usize, u32);
let _25: *const bool;
let _26: *const Adt47;
let _27: i128;
let _28: *mut [i16; 5];
let _29: ();
let _30: ();
{
_4 = [16077_i16,10559_i16,2370_i16];
_5 = !_1;
_2 = -_10;
RET = 2548067387560632996_u64;
_7 = _9;
RET = !5894021698132636111_u64;
RET = _5 as u64;
_3 = [67490782710765147294699124379708923199_i128,(-85259058637240767922036236664846637933_i128),(-87605262809385285221782778330953777888_i128),22970389032487747140476509781706749656_i128,(-71092445111113501793171807241505191606_i128),55897020511702802136306689110939905630_i128,76278431462769957882169228468622821584_i128];
_10 = _2 | _2;
_2 = -_10;
_10 = _8 as isize;
_7 = [8945_i16,18478_i16,(-15337_i16)];
_3 = [26988662409741346326462519089719209821_i128,(-13316670141448039519797353898535813484_i128),(-50614287182554961852523408551249889499_i128),33527012965275719362252258918287514954_i128,(-71140904297511990837521143287521621956_i128),85040974114776708141028874238236302292_i128,146919091916243367974625461416963317024_i128];
_4 = [25213_i16,(-26071_i16),29282_i16];
_1 = _5;
Goto(bb1)
}
bb1 = {
_6 = 10_u8;
_10 = 124589197221817391335701688010594655722_u128 as isize;
_9 = [(-10442_i16),(-1873_i16),31466_i16];
RET = _5 as u64;
RET = 15546313927223486054_u64 & 14118163712784035998_u64;
_2 = _10 << RET;
_10 = _2;
_8 = '\u{10a352}';
_9 = [(-5921_i16),18239_i16,12181_i16];
_11 = Adt24::Variant2 { fld0: 1391797259_i32,fld1: 151881636135280915010093505351068493000_u128 };
place!(Field::<u128>(Variant(_11, 2), 1)) = 268595055917684087349810395266595512815_u128 - 283993712125006077991576834621381909768_u128;
_5 = !_1;
_12 = 3956300555_u32 as i128;
_13.2.2 = ((-17694_i16),);
place!(Field::<i32>(Variant(_11, 2), 0)) = 1330630157_i32;
_13.2.2 = (19361_i16,);
_11 = Adt24::Variant2 { fld0: (-1690364094_i32),fld1: 311860105735733154656051420477864868900_u128 };
_13.0 = _5 | _5;
match _13.2.2.0 {
19361 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_9 = _4;
_10 = _2 >> _2;
_7 = _9;
_1 = _2 == _2;
_1 = !_5;
_1 = _2 == _2;
RET = 4047752601332359099_u64 << _6;
_3 = [_12,_12,_12,_12,_12,_12,_12];
_13.1 = 324465394201668304417964056164573752741_u128;
RET = _10 as u64;
_6 = !7_u8;
_1 = _10 <= _10;
_13.1 = 190526921534981536483476896612264033751_u128;
RET = 16848492797242850765_u64 & 15869731930272456447_u64;
_2 = _10;
_13.2.2 = (21341_i16,);
place!(Field::<u128>(Variant(_11, 2), 1)) = _13.1;
_10 = _12 as isize;
_13.3.0 = _13.2.2.0;
_17 = core::ptr::addr_of_mut!(_13.3);
match (*_17).0 {
0 => bb4,
1 => bb5,
21341 => bb7,
_ => bb6
}
}
bb4 = {
Return()
}
bb5 = {
_6 = 10_u8;
_10 = 124589197221817391335701688010594655722_u128 as isize;
_9 = [(-10442_i16),(-1873_i16),31466_i16];
RET = _5 as u64;
RET = 15546313927223486054_u64 & 14118163712784035998_u64;
_2 = _10 << RET;
_10 = _2;
_8 = '\u{10a352}';
_9 = [(-5921_i16),18239_i16,12181_i16];
_11 = Adt24::Variant2 { fld0: 1391797259_i32,fld1: 151881636135280915010093505351068493000_u128 };
place!(Field::<u128>(Variant(_11, 2), 1)) = 268595055917684087349810395266595512815_u128 - 283993712125006077991576834621381909768_u128;
_5 = !_1;
_12 = 3956300555_u32 as i128;
_13.2.2 = ((-17694_i16),);
place!(Field::<i32>(Variant(_11, 2), 0)) = 1330630157_i32;
_13.2.2 = (19361_i16,);
_11 = Adt24::Variant2 { fld0: (-1690364094_i32),fld1: 311860105735733154656051420477864868900_u128 };
_13.0 = _5 | _5;
match _13.2.2.0 {
19361 => bb3,
_ => bb2
}
}
bb6 = {
Return()
}
bb7 = {
_13.2.2.0 = _13.3.0;
_3 = [_12,_12,_12,_12,_12,_12,_12];
_14 = _6 as isize;
_18.2.0 = !2907829400_u32;
_18.0 = Field::<u128>(Variant(_11, 2), 1);
_17 = core::ptr::addr_of_mut!(_13.2.2);
_18.2 = (2917475682_u32,);
_10 = -_2;
_12 = (-82627637985749381581502667845007843048_i128);
_17 = core::ptr::addr_of_mut!(_13.2.2);
_22 = !_6;
_18.3 = [_10,_10,_2,_10,_2,_2];
_16 = -_2;
_13.2.0 = [_12];
place!(Field::<i32>(Variant(_11, 2), 0)) = 1792624658_i32 << _14;
match _18.2.0 {
0 => bb8,
1 => bb9,
2 => bb10,
2917475682 => bb12,
_ => bb11
}
}
bb8 = {
Return()
}
bb9 = {
_6 = 10_u8;
_10 = 124589197221817391335701688010594655722_u128 as isize;
_9 = [(-10442_i16),(-1873_i16),31466_i16];
RET = _5 as u64;
RET = 15546313927223486054_u64 & 14118163712784035998_u64;
_2 = _10 << RET;
_10 = _2;
_8 = '\u{10a352}';
_9 = [(-5921_i16),18239_i16,12181_i16];
_11 = Adt24::Variant2 { fld0: 1391797259_i32,fld1: 151881636135280915010093505351068493000_u128 };
place!(Field::<u128>(Variant(_11, 2), 1)) = 268595055917684087349810395266595512815_u128 - 283993712125006077991576834621381909768_u128;
_5 = !_1;
_12 = 3956300555_u32 as i128;
_13.2.2 = ((-17694_i16),);
place!(Field::<i32>(Variant(_11, 2), 0)) = 1330630157_i32;
_13.2.2 = (19361_i16,);
_11 = Adt24::Variant2 { fld0: (-1690364094_i32),fld1: 311860105735733154656051420477864868900_u128 };
_13.0 = _5 | _5;
match _13.2.2.0 {
19361 => bb3,
_ => bb2
}
}
bb10 = {
Return()
}
bb11 = {
_6 = 10_u8;
_10 = 124589197221817391335701688010594655722_u128 as isize;
_9 = [(-10442_i16),(-1873_i16),31466_i16];
RET = _5 as u64;
RET = 15546313927223486054_u64 & 14118163712784035998_u64;
_2 = _10 << RET;
_10 = _2;
_8 = '\u{10a352}';
_9 = [(-5921_i16),18239_i16,12181_i16];
_11 = Adt24::Variant2 { fld0: 1391797259_i32,fld1: 151881636135280915010093505351068493000_u128 };
place!(Field::<u128>(Variant(_11, 2), 1)) = 268595055917684087349810395266595512815_u128 - 283993712125006077991576834621381909768_u128;
_5 = !_1;
_12 = 3956300555_u32 as i128;
_13.2.2 = ((-17694_i16),);
place!(Field::<i32>(Variant(_11, 2), 0)) = 1330630157_i32;
_13.2.2 = (19361_i16,);
_11 = Adt24::Variant2 { fld0: (-1690364094_i32),fld1: 311860105735733154656051420477864868900_u128 };
_13.0 = _5 | _5;
match _13.2.2.0 {
19361 => bb3,
_ => bb2
}
}
bb12 = {
_15 = &_20;
match (*_17).0 {
0 => bb7,
1 => bb11,
2 => bb13,
3 => bb14,
21341 => bb16,
_ => bb15
}
}
bb13 = {
_9 = _4;
_10 = _2 >> _2;
_7 = _9;
_1 = _2 == _2;
_1 = !_5;
_1 = _2 == _2;
RET = 4047752601332359099_u64 << _6;
_3 = [_12,_12,_12,_12,_12,_12,_12];
_13.1 = 324465394201668304417964056164573752741_u128;
RET = _10 as u64;
_6 = !7_u8;
_1 = _10 <= _10;
_13.1 = 190526921534981536483476896612264033751_u128;
RET = 16848492797242850765_u64 & 15869731930272456447_u64;
_2 = _10;
_13.2.2 = (21341_i16,);
place!(Field::<u128>(Variant(_11, 2), 1)) = _13.1;
_10 = _12 as isize;
_13.3.0 = _13.2.2.0;
_17 = core::ptr::addr_of_mut!(_13.3);
match (*_17).0 {
0 => bb4,
1 => bb5,
21341 => bb7,
_ => bb6
}
}
bb14 = {
Return()
}
bb15 = {
_6 = 10_u8;
_10 = 124589197221817391335701688010594655722_u128 as isize;
_9 = [(-10442_i16),(-1873_i16),31466_i16];
RET = _5 as u64;
RET = 15546313927223486054_u64 & 14118163712784035998_u64;
_2 = _10 << RET;
_10 = _2;
_8 = '\u{10a352}';
_9 = [(-5921_i16),18239_i16,12181_i16];
_11 = Adt24::Variant2 { fld0: 1391797259_i32,fld1: 151881636135280915010093505351068493000_u128 };
place!(Field::<u128>(Variant(_11, 2), 1)) = 268595055917684087349810395266595512815_u128 - 283993712125006077991576834621381909768_u128;
_5 = !_1;
_12 = 3956300555_u32 as i128;
_13.2.2 = ((-17694_i16),);
place!(Field::<i32>(Variant(_11, 2), 0)) = 1330630157_i32;
_13.2.2 = (19361_i16,);
_11 = Adt24::Variant2 { fld0: (-1690364094_i32),fld1: 311860105735733154656051420477864868900_u128 };
_13.0 = _5 | _5;
match _13.2.2.0 {
19361 => bb3,
_ => bb2
}
}
bb16 = {
_13.2.0 = [_12];
_13.3 = (*_17);
_13.2.1 = _12 as u16;
_24.1 = _13.1 | Field::<u128>(Variant(_11, 2), 1);
_20 = _13.2.1 as f64;
_2 = !_16;
_10 = _2 ^ _2;
_20 = _22 as f64;
_18.2.0 = 2522721906_u32 - 605129853_u32;
_13.2.1 = !34438_u16;
(*_17).0 = _12 as i16;
(*_17) = (_13.3.0,);
_22 = _6;
_11 = Adt24::Variant2 { fld0: 1256523907_i32,fld1: _24.1 };
_13.0 = _1;
_15 = &_20;
_24.1 = !_13.1;
_14 = _2 * _10;
_18.1 = core::ptr::addr_of!(_1);
(*_17).0 = _2 as i16;
Goto(bb17)
}
bb17 = {
Call(_29 = dump_var(13_usize, 1_usize, Move(_1), 2_usize, Move(_2), 7_usize, Move(_7), 12_usize, Move(_12)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(13_usize, 4_usize, Move(_4), 14_usize, Move(_14), 5_usize, Move(_5), 30_usize, _30), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: u32,mut _2: u128,mut _3: [isize; 6],mut _4: *const bool,mut _5: u128) -> isize {
mir! {
type RET = isize;
let _6: isize;
let _7: &'static Adt42;
let _8: [isize; 4];
let _9: (bool, u128, ([i128; 1], u16, (i16,)), (i16,));
let _10: ();
let _11: ();
{
_5 = !_2;
_1 = 100934566_u32;
RET = -(-9223372036854775808_isize);
RET = 100100461195187724370516820573129543999_i128 as isize;
RET = (-30_isize) * 9223372036854775807_isize;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
100934566 => bb5,
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
RET = (-73_isize);
RET = 9223372036854775807_isize;
_3 = [RET,RET,RET,RET,RET,RET];
_6 = RET << _5;
RET = -_6;
RET = _6;
_1 = 931813374_u32;
_3 = [RET,RET,_6,RET,_6,_6];
RET = -_6;
_2 = !_5;
_5 = (-6859396858125917322_i64) as u128;
_5 = !_2;
RET = _6;
_3 = [_6,RET,_6,_6,RET,_6];
_6 = _5 as isize;
_6 = RET ^ RET;
_2 = 13902_u16 as u128;
_2 = _5;
RET = -_6;
_3 = [RET,RET,RET,RET,RET,RET];
_2 = _5 * _5;
_3 = [RET,_6,_6,_6,RET,_6];
_6 = -RET;
_9.3.0 = !20694_i16;
_6 = RET;
_9.0 = !true;
Goto(bb6)
}
bb6 = {
Call(_10 = dump_var(14_usize, 6_usize, Move(_6), 1_usize, Move(_1), 11_usize, _11, 11_usize, _11), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15() -> *const bool {
mir! {
type RET = *const bool;
let _1: *const *const bool;
let _2: bool;
let _3: [i64; 3];
let _4: isize;
let _5: bool;
let _6: i8;
let _7: [i32; 8];
let _8: u8;
let _9: &'static &'static [i32; 3];
let _10: (bool, u128, ([i128; 1], u16, (i16,)), (i16,));
let _11: f64;
let _12: [u32; 2];
let _13: f64;
let _14: char;
let _15: *const [i64; 5];
let _16: [u32; 2];
let _17: (i64,);
let _18: f64;
let _19: u64;
let _20: Adt47;
let _21: *const *const bool;
let _22: isize;
let _23: f32;
let _24: bool;
let _25: i32;
let _26: i64;
let _27: &'static Adt41;
let _28: isize;
let _29: u8;
let _30: char;
let _31: ([i16; 5], (bool, u128, ([i128; 1], u16, (i16,)), (i16,)), i128, &'static usize);
let _32: [u128; 5];
let _33: i8;
let _34: ();
let _35: ();
{
_1 = core::ptr::addr_of!(RET);
_1 = core::ptr::addr_of!((*_1));
_1 = core::ptr::addr_of!((*_1));
_1 = core::ptr::addr_of!((*_1));
(*_1) = core::ptr::addr_of!(_2);
(*RET) = !false;
(*_1) = core::ptr::addr_of!((*RET));
_4 = 19747_i16 as isize;
(*_1) = core::ptr::addr_of!((*RET));
(*RET) = false ^ true;
Goto(bb1)
}
bb1 = {
(*RET) = !false;
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!(_5);
RET = core::ptr::addr_of!((*RET));
(*_1) = core::ptr::addr_of!(_5);
(*RET) = _2;
_6 = 84_i8 & 6_i8;
_1 = core::ptr::addr_of!(RET);
_5 = _2 | _2;
_3 = [5180862992345420501_i64,3049009637075584399_i64,6130154399865543654_i64];
_3 = [48790048042658681_i64,5536147891931459637_i64,(-3521915138339061973_i64)];
(*RET) = !_2;
(*RET) = _2;
RET = core::ptr::addr_of!(_2);
(*_1) = core::ptr::addr_of!(_2);
(*RET) = _5 ^ _5;
_3 = [(-7644134700272621592_i64),2670027836836551963_i64,(-3064264849467945427_i64)];
_6 = !126_i8;
RET = core::ptr::addr_of!(_2);
_3 = [(-4483410630234237840_i64),(-7029593587738314300_i64),3068502666370464260_i64];
RET = core::ptr::addr_of!((*RET));
Call(_4 = fn16(_2, Move((*_1)), (*RET), (*RET), _5, _2, Move(_1), (*RET), _3, _3, _2, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = 111_i8;
_5 = _2;
_1 = core::ptr::addr_of!(RET);
_2 = _5 <= _5;
_7 = [(-430422026_i32),989612294_i32,2001066459_i32,497247320_i32,1054138507_i32,90485384_i32,1878911138_i32,1859158407_i32];
(*_1) = core::ptr::addr_of!(_2);
_6 = 20383_u16 as i8;
_1 = core::ptr::addr_of!((*_1));
RET = core::ptr::addr_of!((*RET));
(*RET) = !_5;
(*RET) = _5;
(*_1) = core::ptr::addr_of!((*RET));
(*RET) = !_5;
RET = core::ptr::addr_of!(_5);
(*RET) = _2 ^ _2;
RET = core::ptr::addr_of!((*RET));
_8 = !159_u8;
RET = core::ptr::addr_of!((*RET));
_10.2.2 = ((-11404_i16),);
_10.2.1 = 48085_u16;
_10.3 = (_10.2.2.0,);
_10.3 = _10.2.2;
_2 = (*RET) ^ (*RET);
Goto(bb3)
}
bb3 = {
_11 = _8 as f64;
_10.2.2.0 = !_10.3.0;
_11 = _6 as f64;
_10.0 = !_2;
_13 = _4 as f64;
_1 = core::ptr::addr_of!(RET);
_10.2.0 = [100542947533424685171541438081561907137_i128];
_13 = (-790557562_i32) as f64;
_10.3 = (_10.2.2.0,);
_13 = -_11;
_10.3 = _10.2.2;
_6 = !0_i8;
_10.2.2.0 = _10.3.0;
(*RET) = !_10.0;
(*_1) = core::ptr::addr_of!(_5);
_16 = [3513707015_u32,3835957923_u32];
_14 = '\u{30166}';
_6 = (-50_i8) - (-113_i8);
Goto(bb4)
}
bb4 = {
_10.3 = (_10.2.2.0,);
_8 = 185_u8;
_11 = _13 + _13;
_10.2.1 = (-62520654271241387012959212136377811397_i128) as u16;
_14 = '\u{951e6}';
_10.2.1 = 13378_u16 >> _10.2.2.0;
_18 = _11 - _11;
(*RET) = _10.0;
_11 = _18;
_10.0 = (*RET);
_14 = '\u{a837f}';
_18 = _13 - _13;
_10.2.2.0 = _10.3.0;
match _8 {
0 => bb1,
1 => bb3,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
185 => bb11,
_ => bb10
}
}
bb5 = {
_11 = _8 as f64;
_10.2.2.0 = !_10.3.0;
_11 = _6 as f64;
_10.0 = !_2;
_13 = _4 as f64;
_1 = core::ptr::addr_of!(RET);
_10.2.0 = [100542947533424685171541438081561907137_i128];
_13 = (-790557562_i32) as f64;
_10.3 = (_10.2.2.0,);
_13 = -_11;
_10.3 = _10.2.2;
_6 = !0_i8;
_10.2.2.0 = _10.3.0;
(*RET) = !_10.0;
(*_1) = core::ptr::addr_of!(_5);
_16 = [3513707015_u32,3835957923_u32];
_14 = '\u{30166}';
_6 = (-50_i8) - (-113_i8);
Goto(bb4)
}
bb6 = {
_6 = 111_i8;
_5 = _2;
_1 = core::ptr::addr_of!(RET);
_2 = _5 <= _5;
_7 = [(-430422026_i32),989612294_i32,2001066459_i32,497247320_i32,1054138507_i32,90485384_i32,1878911138_i32,1859158407_i32];
(*_1) = core::ptr::addr_of!(_2);
_6 = 20383_u16 as i8;
_1 = core::ptr::addr_of!((*_1));
RET = core::ptr::addr_of!((*RET));
(*RET) = !_5;
(*RET) = _5;
(*_1) = core::ptr::addr_of!((*RET));
(*RET) = !_5;
RET = core::ptr::addr_of!(_5);
(*RET) = _2 ^ _2;
RET = core::ptr::addr_of!((*RET));
_8 = !159_u8;
RET = core::ptr::addr_of!((*RET));
_10.2.2 = ((-11404_i16),);
_10.2.1 = 48085_u16;
_10.3 = (_10.2.2.0,);
_10.3 = _10.2.2;
_2 = (*RET) ^ (*RET);
Goto(bb3)
}
bb7 = {
(*RET) = !false;
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!(_5);
RET = core::ptr::addr_of!((*RET));
(*_1) = core::ptr::addr_of!(_5);
(*RET) = _2;
_6 = 84_i8 & 6_i8;
_1 = core::ptr::addr_of!(RET);
_5 = _2 | _2;
_3 = [5180862992345420501_i64,3049009637075584399_i64,6130154399865543654_i64];
_3 = [48790048042658681_i64,5536147891931459637_i64,(-3521915138339061973_i64)];
(*RET) = !_2;
(*RET) = _2;
RET = core::ptr::addr_of!(_2);
(*_1) = core::ptr::addr_of!(_2);
(*RET) = _5 ^ _5;
_3 = [(-7644134700272621592_i64),2670027836836551963_i64,(-3064264849467945427_i64)];
_6 = !126_i8;
RET = core::ptr::addr_of!(_2);
_3 = [(-4483410630234237840_i64),(-7029593587738314300_i64),3068502666370464260_i64];
RET = core::ptr::addr_of!((*RET));
Call(_4 = fn16(_2, Move((*_1)), (*RET), (*RET), _5, _2, Move(_1), (*RET), _3, _3, _2, _3), ReturnTo(bb2), UnwindUnreachable())
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
_19 = 15564231821195960488_u64 ^ 13566708444308898342_u64;
(*RET) = !_10.0;
_10.2.2 = (_10.3.0,);
(*_1) = core::ptr::addr_of!(_2);
_10.1 = !278341887814332938081374971199751077565_u128;
_10.0 = !(*RET);
_10.0 = !(*RET);
_8 = _14 as u8;
_10.3.0 = _10.2.2.0 | _10.2.2.0;
_10.2.1 = 60931_u16;
_10.1 = 280384067429469838300639508624993958679_u128;
_19 = !10263802097742587935_u64;
_17 = (2228611798363109247_i64,);
_11 = _18 + _18;
_1 = core::ptr::addr_of!((*_1));
_4 = _17.0 as isize;
_18 = _11;
_6 = 15_i8;
_5 = _2 != _10.0;
_1 = core::ptr::addr_of!(RET);
(*RET) = _10.0;
_8 = _10.1 as u8;
_3 = [_17.0,_17.0,_17.0];
_12 = [1568474157_u32,3017779710_u32];
_23 = _10.1 as f32;
_10.3 = (_10.2.2.0,);
Call(_10.2 = fn18(Move(RET), (*RET), (*RET), _4, _10.0, Move(_1)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_22 = !_4;
_1 = core::ptr::addr_of!(RET);
_11 = _13 * _13;
_24 = _10.0 >= _10.0;
_6 = !54_i8;
_10.2.2.0 = 3659139801_u32 as i16;
_26 = _17.0;
RET = core::ptr::addr_of!(_10.0);
(*RET) = _5;
_10.1 = 153216106415891785409889558343023531933_u128 | 159500084931824630801558098379814435051_u128;
_12 = [3279071232_u32,2298502277_u32];
_10.2.0 = [(-69603179241531092472761466835673291287_i128)];
_10.1 = 236086201001834987531698405930864733695_u128 | 184059898741733866854350522135709117054_u128;
_1 = core::ptr::addr_of!(RET);
_22 = _4 ^ _4;
_1 = core::ptr::addr_of!((*_1));
Goto(bb13)
}
bb13 = {
_19 = !11438142744233090152_u64;
Goto(bb14)
}
bb14 = {
_25 = 534900515_i32;
_18 = -_13;
_17 = (_26,);
(*_1) = core::ptr::addr_of!((*RET));
_17.0 = _26 - _26;
_5 = !(*RET);
_31.1.0 = (*RET);
_21 = Move(_1);
_14 = '\u{17624}';
_10.3 = _10.2.2;
_10.0 = _24 >= _24;
_10.2.2 = (_10.3.0,);
_24 = (*RET) < (*RET);
_29 = _8;
_28 = -_4;
_14 = '\u{44087}';
_31.1.2.2.0 = -_10.2.2.0;
_5 = _10.0;
_24 = !(*RET);
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(15_usize, 25_usize, Move(_25), 10_usize, Move(_10), 3_usize, Move(_3), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(15_usize, 4_usize, Move(_4), 14_usize, Move(_14), 24_usize, Move(_24), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(15_usize, 22_usize, Move(_22), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: bool,mut _2: *const bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: *const *const bool,mut _8: bool,mut _9: [i64; 3],mut _10: [i64; 3],mut _11: bool,mut _12: [i64; 3]) -> isize {
mir! {
type RET = isize;
let _13: i128;
let _14: f64;
let _15: *mut (i16,);
let _16: [u32; 2];
let _17: &'static Adt41;
let _18: (*mut (i16,), isize, f64, f64);
let _19: ([u128; 5], u128, &'static usize, u32);
let _20: f32;
let _21: [isize; 4];
let _22: u64;
let _23: i32;
let _24: ([u128; 5], u128, &'static usize, u32);
let _25: *const [i64; 5];
let _26: *const [i128; 1];
let _27: (*mut (i16,), isize, f64, f64);
let _28: &'static *const *const bool;
let _29: i128;
let _30: [i128; 1];
let _31: *const [i64; 5];
let _32: f32;
let _33: isize;
let _34: ();
let _35: ();
{
_1 = !_11;
_4 = !_1;
_6 = !_8;
_1 = _4 <= _4;
_12 = [7219348304900002328_i64,(-3844465810822607662_i64),(-9021458381884303602_i64)];
RET = 15779421693460713925_u64 as isize;
RET = !(-85_isize);
_6 = _11 == _1;
_7 = core::ptr::addr_of!(_2);
Goto(bb1)
}
bb1 = {
_6 = _8 > _8;
_11 = !_1;
Call(_7 = fn17(Move(_2), _11, _4, _4, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = core::ptr::addr_of!(_1);
_6 = _5;
_13 = (-130375305623694124976645694782159787298_i128);
_13 = 138_u8 as i128;
RET = 6070145556550908798_u64 as isize;
_9 = [(-3474853071364808770_i64),(-7803906104990112010_i64),3798034902015952690_i64];
_13 = -(-159830196002085684561217974227003285895_i128);
_9 = _12;
_5 = _8 < _1;
_1 = !_11;
_8 = !_11;
_4 = _1 != _5;
_13 = 904782388159529425327973136768860805_i128;
Goto(bb3)
}
bb3 = {
_5 = !_11;
_12 = [2725378573097424370_i64,87723729723057154_i64,5449200797290751528_i64];
_11 = (*_2) < _5;
_2 = core::ptr::addr_of!(_1);
_5 = _4;
_10 = [(-5558446252087649367_i64),9222343756299568425_i64,(-5271240069938781925_i64)];
_2 = core::ptr::addr_of!((*_2));
_7 = core::ptr::addr_of!(_2);
_8 = _11;
_9 = [(-4705765912421656334_i64),(-769898812157475357_i64),(-4304430497335934861_i64)];
(*_2) = !_4;
_6 = !_8;
match _13 {
0 => bb1,
1 => bb2,
2 => bb4,
904782388159529425327973136768860805 => bb6,
_ => bb5
}
}
bb4 = {
_2 = core::ptr::addr_of!(_1);
_6 = _5;
_13 = (-130375305623694124976645694782159787298_i128);
_13 = 138_u8 as i128;
RET = 6070145556550908798_u64 as isize;
_9 = [(-3474853071364808770_i64),(-7803906104990112010_i64),3798034902015952690_i64];
_13 = -(-159830196002085684561217974227003285895_i128);
_9 = _12;
_5 = _8 < _1;
_1 = !_11;
_8 = !_11;
_4 = _1 != _5;
_13 = 904782388159529425327973136768860805_i128;
Goto(bb3)
}
bb5 = {
_6 = _8 > _8;
_11 = !_1;
Call(_7 = fn17(Move(_2), _11, _4, _4, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
RET = !106_isize;
RET = _13 as isize;
_9 = [4172489535048135173_i64,(-8607471461233253604_i64),2049588431512509633_i64];
_5 = _4;
_12 = [(-464835362184767554_i64),(-3940808982897454501_i64),1350049472528450450_i64];
(*_7) = core::ptr::addr_of!(_6);
(*_2) = _11 == _5;
_11 = _6 & _1;
_19.1 = 3583563982529841481_u64 as u128;
(*_7) = core::ptr::addr_of!(_6);
Goto(bb7)
}
bb7 = {
RET = (*_2) as isize;
_14 = _19.1 as f64;
_18.3 = _14 - _14;
_20 = 24_i8 as f32;
_11 = (*_2) >= _1;
_10 = [5206676085306421120_i64,(-1471811537993233370_i64),5417868167252993805_i64];
_4 = !_11;
_16 = [3098343553_u32,2069059752_u32];
_19.3 = !3264881679_u32;
_19.3 = !2655593751_u32;
(*_2) = !_4;
_6 = _11;
_4 = (*_2) > _11;
RET = _19.3 as isize;
_8 = (*_2) < _6;
_1 = !_4;
_7 = core::ptr::addr_of!(_2);
_6 = !_8;
(*_7) = core::ptr::addr_of!((*_2));
_12 = _10;
_19.3 = !1150481270_u32;
match _13 {
904782388159529425327973136768860805 => bb9,
_ => bb8
}
}
bb8 = {
_2 = core::ptr::addr_of!(_1);
_6 = _5;
_13 = (-130375305623694124976645694782159787298_i128);
_13 = 138_u8 as i128;
RET = 6070145556550908798_u64 as isize;
_9 = [(-3474853071364808770_i64),(-7803906104990112010_i64),3798034902015952690_i64];
_13 = -(-159830196002085684561217974227003285895_i128);
_9 = _12;
_5 = _8 < _1;
_1 = !_11;
_8 = !_11;
_4 = _1 != _5;
_13 = 904782388159529425327973136768860805_i128;
Goto(bb3)
}
bb9 = {
_6 = _11 >= _1;
_16 = [_19.3,_19.3];
_21 = [RET,RET,RET,RET];
_13 = !44709942878420861966818602015190130475_i128;
(*_7) = core::ptr::addr_of!(_8);
_24.0 = [_19.1,_19.1,_19.1,_19.1,_19.1];
_1 = _4;
_18.1 = RET;
_3 = _6 & _4;
_24.0 = [_19.1,_19.1,_19.1,_19.1,_19.1];
_4 = _6;
_24.3 = !_19.3;
_18.3 = 6536558202060316110_i64 as f64;
_9 = [(-7065328091841290082_i64),(-5915997076847496428_i64),(-3047054000936173621_i64)];
_24.1 = _19.1;
_22 = 715890424245860385_u64;
_8 = !_3;
_5 = _6;
_22 = _18.1 as u64;
_19.1 = !_24.1;
_23 = _18.1 as i32;
_24.1 = _13 as u128;
(*_7) = core::ptr::addr_of!(_3);
_14 = _13 as f64;
_19.1 = (-5085337619703977667_i64) as u128;
Goto(bb10)
}
bb10 = {
_27.1 = 19_u8 as isize;
RET = _27.1 >> _13;
_22 = 14361415031645195414_u64 ^ 1093089632006321425_u64;
_24.3 = _19.3;
_21 = [RET,_18.1,_18.1,_27.1];
_23 = 134869734_i32 & 1325542457_i32;
_18.2 = _14 * _18.3;
_27.3 = _18.2;
_19.1 = _24.1;
_24.1 = '\u{41f8f}' as u128;
_19.0 = [_24.1,_19.1,_19.1,_24.1,_19.1];
_12 = [6307725010923755184_i64,2186402995377623320_i64,(-6262630841248174174_i64)];
_27.3 = _14;
_13 = 46899903343615149404624846035385554042_i128;
_18.1 = !_27.1;
_24.1 = !_19.1;
_28 = &_7;
RET = 31503_u16 as isize;
_10 = _9;
_19.0 = [_24.1,_24.1,_24.1,_24.1,_24.1];
match _13 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb5,
4 => bb11,
46899903343615149404624846035385554042 => bb13,
_ => bb12
}
}
bb11 = {
_6 = _11 >= _1;
_16 = [_19.3,_19.3];
_21 = [RET,RET,RET,RET];
_13 = !44709942878420861966818602015190130475_i128;
(*_7) = core::ptr::addr_of!(_8);
_24.0 = [_19.1,_19.1,_19.1,_19.1,_19.1];
_1 = _4;
_18.1 = RET;
_3 = _6 & _4;
_24.0 = [_19.1,_19.1,_19.1,_19.1,_19.1];
_4 = _6;
_24.3 = !_19.3;
_18.3 = 6536558202060316110_i64 as f64;
_9 = [(-7065328091841290082_i64),(-5915997076847496428_i64),(-3047054000936173621_i64)];
_24.1 = _19.1;
_22 = 715890424245860385_u64;
_8 = !_3;
_5 = _6;
_22 = _18.1 as u64;
_19.1 = !_24.1;
_23 = _18.1 as i32;
_24.1 = _13 as u128;
(*_7) = core::ptr::addr_of!(_3);
_14 = _13 as f64;
_19.1 = (-5085337619703977667_i64) as u128;
Goto(bb10)
}
bb12 = {
_6 = _8 > _8;
_11 = !_1;
Call(_7 = fn17(Move(_2), _11, _4, _4, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_19.3 = _20 as u32;
_3 = _4 <= _5;
_13 = 117225714478738623444970600595271590529_i128 - (-52131444241120538806952364801190548021_i128);
_16 = [_19.3,_19.3];
Goto(bb14)
}
bb14 = {
_13 = 96459703411892735919027144392733658734_i128;
_22 = _23 as u64;
_24.3 = _19.3 >> _22;
_19.0 = [_19.1,_19.1,_24.1,_19.1,_24.1];
RET = _18.1;
_12 = _10;
_29 = (-17684_i16) as i128;
_30 = [_29];
_2 = core::ptr::addr_of!((*_2));
_10 = _9;
_32 = _20 * _20;
_22 = !7703206828453020525_u64;
_19.1 = _18.2 as u128;
_29 = _13 - _13;
_12 = [3012606023381689310_i64,5294297727562396919_i64,(-7843965855081079760_i64)];
_13 = _29 | _29;
(*_2) = _4;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(16_usize, 29_usize, Move(_29), 13_usize, Move(_13), 5_usize, Move(_5), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(16_usize, 10_usize, Move(_10), 21_usize, Move(_21), 22_usize, Move(_22), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: *const bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool) -> *const *const bool {
mir! {
type RET = *const *const bool;
let _6: Adt71;
let _7: &'static (&'static [i32; 3],);
let _8: char;
let _9: ([i64; 5], &'static Adt47);
let _10: char;
let _11: bool;
let _12: bool;
let _13: ();
let _14: ();
{
RET = core::ptr::addr_of!(_1);
RET = core::ptr::addr_of!((*RET));
(*RET) = core::ptr::addr_of!(_3);
Goto(bb1)
}
bb1 = {
RET = core::ptr::addr_of!((*RET));
(*_1) = _4 & _2;
_1 = core::ptr::addr_of!(_4);
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!((*RET));
(*_1) = _3;
_4 = _5 ^ _3;
Goto(bb2)
}
bb2 = {
(*RET) = core::ptr::addr_of!((*_1));
(*RET) = core::ptr::addr_of!(_2);
_1 = core::ptr::addr_of!((*_1));
_5 = _3;
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!((*RET));
_5 = _3 ^ _4;
_3 = _5 == _2;
_5 = _3 == _3;
_2 = _3;
(*_1) = _5;
(*_1) = _4 | _5;
_1 = core::ptr::addr_of!(_5);
(*RET) = core::ptr::addr_of!(_4);
RET = core::ptr::addr_of!((*RET));
RET = core::ptr::addr_of!(_1);
(*RET) = core::ptr::addr_of!(_2);
(*RET) = core::ptr::addr_of!(_5);
_1 = core::ptr::addr_of!(_3);
_1 = core::ptr::addr_of!(_5);
_3 = !_2;
(*RET) = core::ptr::addr_of!((*_1));
Goto(bb3)
}
bb3 = {
_8 = '\u{eba7f}';
(*RET) = core::ptr::addr_of!(_2);
_5 = !_3;
_2 = _4 <= _3;
(*RET) = core::ptr::addr_of!((*_1));
(*RET) = core::ptr::addr_of!((*_1));
RET = core::ptr::addr_of!(_1);
(*RET) = core::ptr::addr_of!(_5);
_8 = '\u{4891a}';
_2 = !(*_1);
_3 = _2;
_8 = '\u{a1e70}';
_5 = _2;
_10 = _8;
_9.0 = [6757711580743293803_i64,(-2321956048847432017_i64),1178126539929977270_i64,4383980597916235648_i64,4738252542209834740_i64];
(*_1) = !_2;
Goto(bb4)
}
bb4 = {
Call(_13 = dump_var(17_usize, 5_usize, Move(_5), 3_usize, Move(_3), 8_usize, Move(_8), 14_usize, _14), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: *const bool,mut _2: bool,mut _3: bool,mut _4: isize,mut _5: bool,mut _6: *const *const bool) -> ([i128; 1], u16, (i16,)) {
mir! {
type RET = ([i128; 1], u16, (i16,));
let _7: (Adt29, &'static u128, u64, u8);
let _8: isize;
let _9: u128;
let _10: (&'static [i32; 3],);
let _11: ([i128; 1], u16, (i16,));
let _12: *const (bool, u128, ([i128; 1], u16, (i16,)), (i16,));
let _13: i16;
let _14: char;
let _15: char;
let _16: f32;
let _17: Adt24;
let _18: &'static *const *const bool;
let _19: isize;
let _20: ();
let _21: ();
{
_7.3 = 92_u8;
_2 = _5;
RET.2 = (13374_i16,);
RET.1 = 3729237416_u32 as u16;
RET.1 = 14112_u16 ^ 51789_u16;
RET.2 = ((-16640_i16),);
RET.0 = [(-63074589932476756821160492534872136273_i128)];
RET.0 = [(-42392584744003143670754259667825300836_i128)];
_6 = core::ptr::addr_of!(_1);
Goto(bb1)
}
bb1 = {
_8 = -_4;
RET.0 = [(-19251599024109606162940663167948302241_i128)];
RET.2 = ((-15663_i16),);
RET.1 = 18295_u16 << _4;
(*_6) = core::ptr::addr_of!(_5);
_1 = core::ptr::addr_of!((*_1));
Call((*_1) = fn19(_3, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = !_2;
(*_1) = !_3;
RET.2 = (5404_i16,);
_7.1 = &_9;
_6 = core::ptr::addr_of!(_1);
_3 = _5 < _2;
RET.1 = 29933_u16;
(*_6) = core::ptr::addr_of!(_3);
Goto(bb3)
}
bb3 = {
_5 = (*_1) > _2;
_8 = _4 & _4;
RET.1 = 37763_u16;
RET.0 = [148569237248345997546544576491796254686_i128];
(*_1) = !_5;
_2 = _3 <= _5;
match RET.2.0 {
0 => bb1,
5404 => bb4,
_ => bb2
}
}
bb4 = {
(*_1) = _5 < _2;
(*_6) = core::ptr::addr_of!(_2);
_9 = 69776798153275300601513362819883206056_u128 & 126334924132151404978014460956290837406_u128;
_1 = core::ptr::addr_of!((*_1));
_11.2 = (RET.2.0,);
RET.0 = [123317861159444901064640465391485398229_i128];
_11.2.0 = RET.2.0;
RET.0 = [(-43676944732294382024668120825855398487_i128)];
_11.1 = !RET.1;
_11.2 = (RET.2.0,);
_1 = core::ptr::addr_of!(_5);
RET.2.0 = _11.2.0;
RET.2.0 = _11.2.0 ^ _11.2.0;
RET.2 = (_11.2.0,);
_11.0 = [(-117531589878792599632381208881780096297_i128)];
_11.2.0 = -RET.2.0;
_11 = (RET.0, RET.1, RET.2);
RET = (_11.0, _11.1, _11.2);
_11.2.0 = RET.2.0;
_14 = '\u{21966}';
_7.1 = &_9;
RET.2.0 = _11.2.0 >> _4;
RET.0 = _11.0;
match RET.1 {
0 => bb3,
1 => bb2,
2 => bb5,
37763 => bb7,
_ => bb6
}
}
bb5 = {
_8 = -_4;
RET.0 = [(-19251599024109606162940663167948302241_i128)];
RET.2 = ((-15663_i16),);
RET.1 = 18295_u16 << _4;
(*_6) = core::ptr::addr_of!(_5);
_1 = core::ptr::addr_of!((*_1));
Call((*_1) = fn19(_3, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_5 = !_2;
(*_1) = !_3;
RET.2 = (5404_i16,);
_7.1 = &_9;
_6 = core::ptr::addr_of!(_1);
_3 = _5 < _2;
RET.1 = 29933_u16;
(*_6) = core::ptr::addr_of!(_3);
Goto(bb3)
}
bb7 = {
_7.0 = Adt29::Variant1 { fld0: _3,fld1: Move((*_6)),fld2: Move(_6),fld3: RET,fld4: _7.3,fld5: 2976917341_u32,fld6: (-1197032785204819751_i64),fld7: 86073965301278008174856289950722305976_i128 };
RET.0 = _11.0;
_13 = RET.2.0;
place!(Field::<([i128; 1], u16, (i16,))>(Variant(_7.0, 1), 3)).1 = 15592912563818232345_u64 as u16;
_7.2 = 2299266355400700536_usize as u64;
_7.3 = Field::<u8>(Variant(_7.0, 1), 4);
RET = (Field::<([i128; 1], u16, (i16,))>(Variant(_7.0, 1), 3).0, _11.1, Field::<([i128; 1], u16, (i16,))>(Variant(_7.0, 1), 3).2);
RET.1 = !Field::<([i128; 1], u16, (i16,))>(Variant(_7.0, 1), 3).1;
place!(Field::<bool>(Variant(_7.0, 1), 0)) = _2;
place!(Field::<*const bool>(Variant(_7.0, 1), 1)) = core::ptr::addr_of!(_2);
RET.2.0 = _13 ^ _13;
_16 = Field::<u8>(Variant(_7.0, 1), 4) as f32;
_4 = _8;
_11.2.0 = _13 << RET.2.0;
RET = (_11.0, Field::<([i128; 1], u16, (i16,))>(Variant(_7.0, 1), 3).1, _11.2);
match _7.3 {
92 => bb9,
_ => bb8
}
}
bb8 = {
_8 = -_4;
RET.0 = [(-19251599024109606162940663167948302241_i128)];
RET.2 = ((-15663_i16),);
RET.1 = 18295_u16 << _4;
(*_6) = core::ptr::addr_of!(_5);
_1 = core::ptr::addr_of!((*_1));
Call((*_1) = fn19(_3, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb9 = {
_1 = core::ptr::addr_of!(_5);
_15 = _14;
_9 = _7.3 as u128;
_2 = _3 | _3;
place!(Field::<i128>(Variant(_7.0, 1), 7)) = (-139334262926844864144081706218504968188_i128) >> _7.2;
_2 = !(*_1);
_8 = -_4;
place!(Field::<u32>(Variant(_7.0, 1), 5)) = 1142320208_u32;
place!(Field::<*const bool>(Variant(_7.0, 1), 1)) = Move(_1);
_11.2 = (Field::<([i128; 1], u16, (i16,))>(Variant(_7.0, 1), 3).2.0,);
_5 = _3 == _3;
RET.1 = Field::<u8>(Variant(_7.0, 1), 4) as u16;
place!(Field::<*const bool>(Variant(_7.0, 1), 1)) = core::ptr::addr_of!(_2);
_9 = !319632909205424051546779541609470879674_u128;
_5 = _3 <= Field::<bool>(Variant(_7.0, 1), 0);
_7.3 = Field::<u8>(Variant(_7.0, 1), 4) & Field::<u8>(Variant(_7.0, 1), 4);
_11.2.0 = _13;
_8 = _4;
_5 = _3;
RET.2.0 = -_11.2.0;
_18 = &place!(Field::<*const *const bool>(Variant(_7.0, 1), 2));
_11.0 = [Field::<i128>(Variant(_7.0, 1), 7)];
_2 = _5 > _5;
_11.2.0 = RET.2.0 & RET.2.0;
_5 = _3 == Field::<bool>(Variant(_7.0, 1), 0);
place!(Field::<u8>(Variant(_7.0, 1), 4)) = _7.3;
place!(Field::<i64>(Variant(_7.0, 1), 6)) = 5855784448557353351_i64;
match _11.1 {
0 => bb4,
1 => bb2,
37763 => bb11,
_ => bb10
}
}
bb10 = {
_5 = !_2;
(*_1) = !_3;
RET.2 = (5404_i16,);
_7.1 = &_9;
_6 = core::ptr::addr_of!(_1);
_3 = _5 < _2;
RET.1 = 29933_u16;
(*_6) = core::ptr::addr_of!(_3);
Goto(bb3)
}
bb11 = {
place!(Field::<([i128; 1], u16, (i16,))>(Variant(_7.0, 1), 3)).2.0 = _4 as i16;
_1 = Move(Field::<*const bool>(Variant(_7.0, 1), 1));
_11.2 = (RET.2.0,);
place!(Field::<i64>(Variant(_7.0, 1), 6)) = _11.1 as i64;
match _11.1 {
0 => bb12,
37763 => bb14,
_ => bb13
}
}
bb12 = {
_8 = -_4;
RET.0 = [(-19251599024109606162940663167948302241_i128)];
RET.2 = ((-15663_i16),);
RET.1 = 18295_u16 << _4;
(*_6) = core::ptr::addr_of!(_5);
_1 = core::ptr::addr_of!((*_1));
Call((*_1) = fn19(_3, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
(*_1) = _5 < _2;
(*_6) = core::ptr::addr_of!(_2);
_9 = 69776798153275300601513362819883206056_u128 & 126334924132151404978014460956290837406_u128;
_1 = core::ptr::addr_of!((*_1));
_11.2 = (RET.2.0,);
RET.0 = [123317861159444901064640465391485398229_i128];
_11.2.0 = RET.2.0;
RET.0 = [(-43676944732294382024668120825855398487_i128)];
_11.1 = !RET.1;
_11.2 = (RET.2.0,);
_1 = core::ptr::addr_of!(_5);
RET.2.0 = _11.2.0;
RET.2.0 = _11.2.0 ^ _11.2.0;
RET.2 = (_11.2.0,);
_11.0 = [(-117531589878792599632381208881780096297_i128)];
_11.2.0 = -RET.2.0;
_11 = (RET.0, RET.1, RET.2);
RET = (_11.0, _11.1, _11.2);
_11.2.0 = RET.2.0;
_14 = '\u{21966}';
_7.1 = &_9;
RET.2.0 = _11.2.0 >> _4;
RET.0 = _11.0;
match RET.1 {
0 => bb3,
1 => bb2,
2 => bb5,
37763 => bb7,
_ => bb6
}
}
bb14 = {
_17 = Adt24::Variant3 { fld0: _9,fld1: _16,fld2: Field::<([i128; 1], u16, (i16,))>(Variant(_7.0, 1), 3).1,fld3: Field::<u32>(Variant(_7.0, 1), 5),fld4: _11.2.0,fld5: Field::<i128>(Variant(_7.0, 1), 7),fld6: Field::<i64>(Variant(_7.0, 1), 6) };
_11.2.0 = _7.2 as i16;
place!(Field::<([i128; 1], u16, (i16,))>(Variant(_7.0, 1), 3)).2 = _11.2;
place!(Field::<bool>(Variant(_7.0, 1), 0)) = _3 & _2;
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(18_usize, 14_usize, Move(_14), 5_usize, Move(_5), 4_usize, Move(_4), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_20 = dump_var(18_usize, 9_usize, Move(_9), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: bool,mut _2: *const *const bool) -> bool {
mir! {
type RET = bool;
let _3: f64;
let _4: *const Adt47;
let _5: Adt72;
let _6: isize;
let _7: ();
let _8: ();
{
RET = !_1;
_1 = !RET;
RET = _1;
_1 = RET | RET;
_3 = 11474678805678305824528216505959414482_i128 as f64;
_1 = RET;
_1 = !RET;
RET = _1;
RET = _1;
_3 = 105580201655133283515293070173601447055_u128 as f64;
RET = _1;
_1 = RET;
_3 = 4813244611903786077_i64 as f64;
_1 = !RET;
_3 = 39_i8 as f64;
_3 = 28645_i16 as f64;
RET = !_1;
RET = _1;
RET = !_1;
_3 = 1611553174374419886_usize as f64;
_1 = _3 >= _3;
Goto(bb1)
}
bb1 = {
_1 = RET | RET;
RET = _1 & _1;
RET = _1 | _1;
_3 = (-9223372036854775808_isize) as f64;
RET = !_1;
_6 = (-9223372036854775808_isize) - 9_isize;
_1 = !RET;
RET = !_1;
_6 = -(-9223372036854775808_isize);
_1 = _3 > _3;
_6 = 27_isize ^ (-9223372036854775808_isize);
_3 = 191351234135469731542317723747818347840_u128 as f64;
_1 = RET & RET;
Goto(bb2)
}
bb2 = {
Call(_7 = dump_var(19_usize, 1_usize, Move(_1), 8_usize, _8, 8_usize, _8, 8_usize, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(7133010158589113007_usize), std::hint::black_box(2204011881_u32), std::hint::black_box((-29697_i16)));
                
            }
impl PrintFDebug for Adt24{
	unsafe fn printf_debug(&self){unsafe{printf("Adt24::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt24 {
Variant0{
fld0: i16,
fld1: (i16,),
fld2: i128,
fld3: f32,

},
Variant1{
fld0: bool,
fld1: char,
fld2: f64,
fld3: [u128; 5],
fld4: u16,

},
Variant2{
fld0: i32,
fld1: u128,

},
Variant3{
fld0: u128,
fld1: f32,
fld2: u16,
fld3: u32,
fld4: i16,
fld5: i128,
fld6: i64,

}}
impl PrintFDebug for Adt29{
	unsafe fn printf_debug(&self){unsafe{printf("Adt29::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt29 {
Variant0{
fld0: (i16,),
fld1: ([i128; 1], u16, (i16,)),
fld2: u8,
fld3: u128,
fld4: i16,
fld5: (i64,),

},
Variant1{
fld0: bool,
fld1: *const bool,
fld2: *const *const bool,
fld3: ([i128; 1], u16, (i16,)),
fld4: u8,
fld5: u32,
fld6: i64,
fld7: i128,

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt41{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt41 {
fld0: u16,
fld1: [isize; 6],
fld2: isize,
fld3: (bool, u128, ([i128; 1], u16, (i16,)), (i16,)),
fld4: *mut [i16; 5],
fld5: i32,
fld6: (u128, *const bool, (u32,), [isize; 6]),
fld7: (u32,),
}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: *const *const bool,
fld1: i8,
fld2: u16,

},
Variant1{
fld0: *const bool,
fld1: u16,
fld2: Adt41,
fld3: (u32,),
fld4: (i64,),
fld5: *const *const bool,
fld6: [u128; 5],
fld7: Adt24,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: (i16,),

},
Variant1{
fld0: (([i128; 1], u16, (i16,)),),
fld1: (u128, *const bool, (u32,), [isize; 6]),

},
Variant2{
fld0: i32,
fld1: (bool, u128, ([i128; 1], u16, (i16,)), (i16,)),

},
Variant3{
fld0: (([i128; 1], u16, (i16,)),),
fld1: u128,
fld2: [u128; 5],
fld3: [isize; 6],

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: bool,
fld1: *mut [i16; 5],
fld2: (u128, *const bool, (u32,), [isize; 6]),
fld3: [isize; 6],

},
Variant1{
fld0: *mut (i16,),
fld1: *const bool,
fld2: i64,
fld3: [isize; 6],
fld4: Adt41,
fld5: i32,

},
Variant2{
fld0: bool,
fld1: *mut (i16,),
fld2: [i128; 1],
fld3: [u64; 6],
fld4: u64,

},
Variant3{
fld0: i64,
fld1: [i32; 3],
fld2: usize,
fld3: [i128; 1],
fld4: i128,
fld5: *const *const bool,

}}
impl PrintFDebug for Adt71{
	unsafe fn printf_debug(&self){unsafe{printf("Adt71::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt71 {
Variant0{
fld0: [u32; 2],
fld1: [i16; 5],
fld2: isize,
fld3: (([i128; 1], u16, (i16,)),),

},
Variant1{
fld0: f64,
fld1: *const [i64; 5],

}}
impl PrintFDebug for Adt72{
	unsafe fn printf_debug(&self){unsafe{printf("Adt72::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt72 {
Variant0{
fld0: [i128; 7],
fld1: *const Adt47,
fld2: (u128, *const bool, (u32,), [isize; 6]),

},
Variant1{
fld0: [i16; 5],
fld1: [i128; 7],
fld2: (bool, u128, ([i128; 1], u16, (i16,)), (i16,)),
fld3: *mut (i16,),
fld4: [isize; 6],
fld5: ([i128; 1], char, f64),

},
Variant2{
fld0: bool,
fld1: [u32; 2],
fld2: *const Adt47,
fld3: (u32,),
fld4: u64,
fld5: [u128; 5],
fld6: *const [i64; 5],
fld7: u16,

}}

