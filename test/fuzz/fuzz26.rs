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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: u128,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u64,mut _12: u32) -> [i32; 3] {
mir! {
type RET = [i32; 3];
let _13: char;
let _14: char;
let _15: isize;
let _16: bool;
let _17: isize;
let _18: (&'static u32, &'static Adt34);
let _19: i128;
let _20: f32;
let _21: [bool; 4];
let _22: i128;
let _23: [u64; 4];
let _24: f64;
let _25: [isize; 6];
let _26: char;
let _27: *const i128;
let _28: (u8, u128, Adt31, *const *const isize);
let _29: ();
let _30: ();
{
_5 = -14922_i16;
_7 = (-3475578370232691243_i64) & 1386021166347983679_i64;
_6 = (-1761625626_i32) as u128;
_9 = 8288384736191650728_usize;
_7 = 148413589_i32 as i64;
_12 = !520235353_u32;
_4 = _5 as i8;
match _9 {
8288384736191650728 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_12 = '\u{ad7c9}' as u32;
_12 = 2019204763_u32 | 2070807828_u32;
_11 = 1661902296767970580_u64;
RET = [744671999_i32,1199325335_i32,(-737225444_i32)];
match _9 {
8288384736191650728 => bb3,
_ => bb1
}
}
bb3 = {
_8 = -146825490130951892963313947262475779162_i128;
_8 = (-28090243152228045546998376333255942297_i128) | 85897352438797336926853622240644275865_i128;
_9 = 12978048630392207125_usize | 5_usize;
_11 = !3675223207342128975_u64;
_13 = '\u{69ba9}';
_12 = 656879137_u32 + 4086143501_u32;
_2 = _13;
_10 = 29_u8;
_11 = 17800388559108588332_u64;
_1 = !true;
_9 = 7_usize ^ 2145719958822149592_usize;
RET = [(-1219079823_i32),1946594681_i32,(-1891691616_i32)];
_13 = _2;
_1 = false | false;
_8 = -143065059628893590236994013029882162882_i128;
_7 = (-8364249617401299191_i64);
_14 = _13;
_3 = _4 as isize;
_11 = 14186478736139652090_u64;
Call(_7 = fn1(_13, RET, _2, _1, RET, _3, _10, RET, _14, _9, _6), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_7 = !(-358328147639928814_i64);
_8 = 1200051260566542616375282912074460882_i128 >> _9;
_13 = _2;
_5 = _11 as i16;
_9 = 2_usize;
_16 = !_1;
_8 = (-125031616252168456444110440650972473114_i128) << _11;
_1 = _16;
_15 = _3;
_16 = _1;
_8 = _12 as i128;
RET[_9] = -331456811_i32;
RET[_9] = 48210450_i32;
_10 = RET[_9] as u8;
RET = [(-95703800_i32),889802338_i32,(-2013338232_i32)];
_17 = !_15;
RET[_9] = (-2079067760_i32);
_19 = _5 as i128;
_2 = _14;
RET[_9] = -953590964_i32;
match _11 {
0 => bb5,
1 => bb6,
2 => bb7,
14186478736139652090 => bb9,
_ => bb8
}
}
bb5 = {
_8 = -146825490130951892963313947262475779162_i128;
_8 = (-28090243152228045546998376333255942297_i128) | 85897352438797336926853622240644275865_i128;
_9 = 12978048630392207125_usize | 5_usize;
_11 = !3675223207342128975_u64;
_13 = '\u{69ba9}';
_12 = 656879137_u32 + 4086143501_u32;
_2 = _13;
_10 = 29_u8;
_11 = 17800388559108588332_u64;
_1 = !true;
_9 = 7_usize ^ 2145719958822149592_usize;
RET = [(-1219079823_i32),1946594681_i32,(-1891691616_i32)];
_13 = _2;
_1 = false | false;
_8 = -143065059628893590236994013029882162882_i128;
_7 = (-8364249617401299191_i64);
_14 = _13;
_3 = _4 as isize;
_11 = 14186478736139652090_u64;
Call(_7 = fn1(_13, RET, _2, _1, RET, _3, _10, RET, _14, _9, _6), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_12 = '\u{ad7c9}' as u32;
_12 = 2019204763_u32 | 2070807828_u32;
_11 = 1661902296767970580_u64;
RET = [744671999_i32,1199325335_i32,(-737225444_i32)];
match _9 {
8288384736191650728 => bb3,
_ => bb1
}
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
_7 = -6156759177029311698_i64;
_18.0 = &_12;
_17 = _3 >> _5;
_20 = RET[_9] as f32;
_16 = _1;
_21[_9] = !_1;
_6 = 311505527766137984766277607394638878961_u128 | 234417693720390661592796357809895424618_u128;
_14 = _2;
_21 = [_1,_16,_1,_16];
_8 = -_19;
_2 = _14;
match _11 {
14186478736139652090 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
RET[_9] = _6 as i32;
RET = [(-808756779_i32),(-113915078_i32),(-2009408175_i32)];
RET[_9] = (-873796581_i32);
_7 = !(-2478203334524552451_i64);
_13 = _14;
_3 = _17 + _17;
_9 = 2_usize;
_16 = _21[_9];
match _11 {
14186478736139652090 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_22 = _19;
_21[_9] = _3 >= _15;
Goto(bb14)
}
bb14 = {
_22 = !_19;
_23 = [_11,_11,_11,_11];
_3 = _11 as isize;
_25 = [_17,_17,_15,_15,_17,_17];
_11 = 33558_u16 as u64;
_15 = _17 | _25[_9];
RET = [558561254_i32,1345105353_i32,(-844505662_i32)];
_25 = [_3,_17,_15,_3,_15,_15];
_2 = _13;
RET[_9] = (-509436080_i32) - 1832886469_i32;
_26 = _14;
RET = [2022316729_i32,(-564509434_i32),71736510_i32];
_28.1 = 58282_u16 as u128;
RET[_9] = (-645282828_i32) + 2025880604_i32;
_27 = core::ptr::addr_of!(_8);
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(0_usize, 13_usize, Move(_13), 10_usize, Move(_10), 25_usize, Move(_25), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(0_usize, 5_usize, Move(_5), 4_usize, Move(_4), 2_usize, Move(_2), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(0_usize, 9_usize, Move(_9), 3_usize, Move(_3), 6_usize, Move(_6), 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: char,mut _2: [i32; 3],mut _3: char,mut _4: bool,mut _5: [i32; 3],mut _6: isize,mut _7: u8,mut _8: [i32; 3],mut _9: char,mut _10: usize,mut _11: u128) -> i64 {
mir! {
type RET = i64;
let _12: f64;
let _13: char;
let _14: isize;
let _15: char;
let _16: usize;
let _17: ([u128; 1],);
let _18: (*const u8,);
let _19: i128;
let _20: *const ([u128; 1],);
let _21: &'static usize;
let _22: f64;
let _23: i8;
let _24: *const *const isize;
let _25: bool;
let _26: *mut [u128; 1];
let _27: f32;
let _28: char;
let _29: i128;
let _30: [bool; 4];
let _31: *const *const isize;
let _32: isize;
let _33: char;
let _34: u8;
let _35: i8;
let _36: *const u32;
let _37: f32;
let _38: isize;
let _39: i128;
let _40: [u8; 4];
let _41: ();
let _42: ();
{
_1 = _9;
RET = 8796923932144829784_i64 + 8974588757730814927_i64;
_2 = [(-790165186_i32),(-1654929448_i32),(-1345857286_i32)];
_6 = (-9223372036854775808_isize);
_7 = !173_u8;
_3 = _1;
_3 = _1;
_3 = _9;
_12 = _6 as f64;
_4 = true;
RET = 70308918861621079654568600898827676787_i128 as i64;
_12 = 8_i8 as f64;
RET = !(-1262624580250090231_i64);
_4 = false | false;
_1 = _9;
_9 = _3;
_6 = !9223372036854775807_isize;
_9 = _1;
_13 = _9;
_7 = !148_u8;
_1 = _9;
_9 = _13;
RET = -(-7004819349944218656_i64);
_8 = [(-623753896_i32),(-2126357266_i32),1078687029_i32];
_7 = 206_u8 & 89_u8;
_11 = 166375385698649559502250596932966629533_u128;
_2 = [503844441_i32,(-1999561306_i32),2051141091_i32];
Call(_12 = core::intrinsics::transmute(_6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _1;
_9 = _13;
_6 = (-9223372036854775808_isize);
_7 = _12 as u8;
_7 = 238_u8;
_7 = _10 as u8;
_4 = true;
_6 = -9223372036854775807_isize;
RET = (-7725685436695941735_i64);
_11 = 68791431902323291829423149468411283622_u128;
_3 = _1;
_3 = _9;
_3 = _1;
_11 = 183950958562154441062696226970468628909_u128 - 279836433008895270151234684761708567379_u128;
Call(_8 = core::intrinsics::transmute(_5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9 = _3;
RET = (-6744423250092667270_i64) | (-24313180361779086_i64);
_1 = _3;
_12 = 1018_i16 as f64;
_8 = _5;
_6 = (-11308_i16) as isize;
_8 = [(-176165323_i32),814563809_i32,1946972975_i32];
_4 = _13 < _1;
_16 = _9 as usize;
_2 = [(-1338459719_i32),1267510034_i32,1355728021_i32];
_14 = _10 as isize;
_14 = _6;
RET = (-1182442794931095974_i64) >> _7;
_19 = 10719919885326231211359127545376099238_i128 + (-131394194627256073183743956754210555391_i128);
_6 = 16340891779243798504_u64 as isize;
_20 = core::ptr::addr_of!(_17);
Call((*_20) = fn2(_19, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_20).0 = [_11];
_18.0 = core::ptr::addr_of!(_7);
(*_20).0 = [_11];
(*_20).0 = [_11];
_6 = _16 as isize;
_21 = &_10;
_17.0 = [_11];
Goto(bb4)
}
bb4 = {
Goto(bb5)
}
bb5 = {
_6 = !_14;
_13 = _9;
_12 = _19 as f64;
(*_20).0 = [_11];
_12 = _19 as f64;
_4 = true;
_9 = _1;
_17.0 = [_11];
RET = 3918066329168091643_i64 ^ 8111933234722871954_i64;
_15 = _13;
_20 = core::ptr::addr_of!(_17);
_3 = _1;
_20 = core::ptr::addr_of!((*_20));
_12 = (-664117848_i32) as f64;
_18.0 = core::ptr::addr_of!(_7);
_16 = !_10;
(*_20).0 = [_11];
(*_20).0 = [_11];
_22 = _12 - _12;
_1 = _13;
_6 = (-100_i16) as isize;
Goto(bb6)
}
bb6 = {
_15 = _3;
_2 = _8;
_1 = _3;
_20 = core::ptr::addr_of!(_17);
_1 = _3;
_23 = (-39_i8) & 116_i8;
_12 = _22;
_11 = 333572863429295073890613119349714778394_u128 | 318086856238831264125396663631219740681_u128;
_21 = &(*_21);
_9 = _15;
_25 = _4;
RET = 98814520191287930_i64;
_5 = [1562932466_i32,(-1559138868_i32),(-2032961453_i32)];
_4 = !_25;
_4 = !_25;
_14 = _6 | _6;
_17.0 = [_11];
_16 = (*_21);
_26 = core::ptr::addr_of_mut!(_17.0);
(*_26) = [_11];
_7 = !12_u8;
_10 = !_16;
_19 = _4 as i128;
_18.0 = core::ptr::addr_of!(_7);
Goto(bb7)
}
bb7 = {
(*_20).0 = [_11];
_27 = _11 as f32;
_12 = (-16381_i16) as f64;
_3 = _13;
_6 = _14 & _14;
_12 = -_22;
_3 = _15;
_11 = 74276113109821152548296991863838257696_u128 << _10;
(*_20).0 = [_11];
_18.0 = core::ptr::addr_of!(_7);
_19 = (-99509483261615006430538518512146288494_i128);
_4 = _25 & _25;
_29 = _19;
(*_20).0 = [_11];
_10 = _22 as usize;
_22 = _12;
(*_20).0 = [_11];
Goto(bb8)
}
bb8 = {
RET = (-1096532532952883760_i64);
_10 = !_16;
_2 = [614449880_i32,1496103643_i32,(-442103412_i32)];
(*_26) = [_11];
(*_26) = [_11];
_20 = core::ptr::addr_of!((*_20));
_9 = _3;
_26 = core::ptr::addr_of_mut!((*_26));
_23 = (-59_i8);
_9 = _1;
_15 = _9;
_16 = !_10;
_12 = _22 * _22;
_30 = [_25,_4,_4,_25];
_18.0 = core::ptr::addr_of!(_7);
_18.0 = core::ptr::addr_of!(_7);
_3 = _9;
_12 = -_22;
_14 = 3814941173907244070_u64 as isize;
match _19 {
0 => bb1,
240772883659323457032836088919621922962 => bb10,
_ => bb9
}
}
bb9 = {
_15 = _3;
_2 = _8;
_1 = _3;
_20 = core::ptr::addr_of!(_17);
_1 = _3;
_23 = (-39_i8) & 116_i8;
_12 = _22;
_11 = 333572863429295073890613119349714778394_u128 | 318086856238831264125396663631219740681_u128;
_21 = &(*_21);
_9 = _15;
_25 = _4;
RET = 98814520191287930_i64;
_5 = [1562932466_i32,(-1559138868_i32),(-2032961453_i32)];
_4 = !_25;
_4 = !_25;
_14 = _6 | _6;
_17.0 = [_11];
_16 = (*_21);
_26 = core::ptr::addr_of_mut!(_17.0);
(*_26) = [_11];
_7 = !12_u8;
_10 = !_16;
_19 = _4 as i128;
_18.0 = core::ptr::addr_of!(_7);
Goto(bb7)
}
bb10 = {
(*_20).0 = [_11];
_25 = _4;
(*_26) = [_11];
RET = 6131074110398106971_i64;
RET = _12 as i64;
_17.0 = [_11];
_3 = _1;
_15 = _9;
(*_26) = [_11];
_28 = _9;
_11 = 120733377669970937414902525700600571862_u128 & 77778515363040620196579370044153753173_u128;
_11 = !140864415897420543192716177840587528941_u128;
(*_20).0 = [_11];
(*_20).0 = [_11];
RET = 3059774892465387201_i64;
(*_26) = [_11];
(*_20).0 = [_11];
(*_26) = [_11];
_29 = _19 | _19;
Goto(bb11)
}
bb11 = {
_6 = _14;
_26 = core::ptr::addr_of_mut!((*_20).0);
_12 = _22;
_34 = _7;
_7 = _19 as u8;
_23 = (-113_i8);
_17.0 = [_11];
_32 = _6 * _14;
_15 = _9;
_1 = _28;
(*_20).0 = [_11];
(*_26) = [_11];
_32 = RET as isize;
_2 = [(-1591905400_i32),1770087207_i32,(-1179096288_i32)];
_33 = _13;
RET = !6050598397771955570_i64;
_28 = _1;
_5 = [1980688728_i32,(-786644636_i32),182790606_i32];
_8 = _5;
_4 = _25;
_28 = _33;
Call(_29 = core::intrinsics::transmute((*_26)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
(*_26) = [_11];
_11 = 247576509523002160016557825869669271111_u128 ^ 190377286284780420859282018471337088978_u128;
_14 = !_6;
_35 = _23;
(*_26) = [_11];
_38 = _6 & _14;
_16 = 12833973495345681492_u64 as usize;
_29 = _19;
_1 = _13;
_17.0 = [_11];
_17.0 = [_11];
(*_26) = [_11];
_21 = &_16;
_14 = 13247_u16 as isize;
(*_26) = [_11];
_10 = !_16;
(*_26) = [_11];
_3 = _15;
_3 = _33;
_33 = _15;
_26 = core::ptr::addr_of_mut!((*_26));
_17.0 = [_11];
_23 = _35 | _35;
_21 = &(*_21);
_33 = _28;
_30 = [_4,_4,_25,_25];
_22 = _16 as f64;
(*_26) = [_11];
_5 = [673816776_i32,(-367930780_i32),364223355_i32];
RET = 8056215724151599528_i64;
match _29 {
0 => bb3,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
6 => bb18,
240772883659323457032836088919621922962 => bb20,
_ => bb19
}
}
bb13 = {
_6 = _14;
_26 = core::ptr::addr_of_mut!((*_20).0);
_12 = _22;
_34 = _7;
_7 = _19 as u8;
_23 = (-113_i8);
_17.0 = [_11];
_32 = _6 * _14;
_15 = _9;
_1 = _28;
(*_20).0 = [_11];
(*_26) = [_11];
_32 = RET as isize;
_2 = [(-1591905400_i32),1770087207_i32,(-1179096288_i32)];
_33 = _13;
RET = !6050598397771955570_i64;
_28 = _1;
_5 = [1980688728_i32,(-786644636_i32),182790606_i32];
_8 = _5;
_4 = _25;
_28 = _33;
Call(_29 = core::intrinsics::transmute((*_26)), ReturnTo(bb12), UnwindUnreachable())
}
bb14 = {
(*_20).0 = [_11];
_25 = _4;
(*_26) = [_11];
RET = 6131074110398106971_i64;
RET = _12 as i64;
_17.0 = [_11];
_3 = _1;
_15 = _9;
(*_26) = [_11];
_28 = _9;
_11 = 120733377669970937414902525700600571862_u128 & 77778515363040620196579370044153753173_u128;
_11 = !140864415897420543192716177840587528941_u128;
(*_20).0 = [_11];
(*_20).0 = [_11];
RET = 3059774892465387201_i64;
(*_26) = [_11];
(*_20).0 = [_11];
(*_26) = [_11];
_29 = _19 | _19;
Goto(bb11)
}
bb15 = {
_15 = _3;
_2 = _8;
_1 = _3;
_20 = core::ptr::addr_of!(_17);
_1 = _3;
_23 = (-39_i8) & 116_i8;
_12 = _22;
_11 = 333572863429295073890613119349714778394_u128 | 318086856238831264125396663631219740681_u128;
_21 = &(*_21);
_9 = _15;
_25 = _4;
RET = 98814520191287930_i64;
_5 = [1562932466_i32,(-1559138868_i32),(-2032961453_i32)];
_4 = !_25;
_4 = !_25;
_14 = _6 | _6;
_17.0 = [_11];
_16 = (*_21);
_26 = core::ptr::addr_of_mut!(_17.0);
(*_26) = [_11];
_7 = !12_u8;
_10 = !_16;
_19 = _4 as i128;
_18.0 = core::ptr::addr_of!(_7);
Goto(bb7)
}
bb16 = {
RET = (-1096532532952883760_i64);
_10 = !_16;
_2 = [614449880_i32,1496103643_i32,(-442103412_i32)];
(*_26) = [_11];
(*_26) = [_11];
_20 = core::ptr::addr_of!((*_20));
_9 = _3;
_26 = core::ptr::addr_of_mut!((*_26));
_23 = (-59_i8);
_9 = _1;
_15 = _9;
_16 = !_10;
_12 = _22 * _22;
_30 = [_25,_4,_4,_25];
_18.0 = core::ptr::addr_of!(_7);
_18.0 = core::ptr::addr_of!(_7);
_3 = _9;
_12 = -_22;
_14 = 3814941173907244070_u64 as isize;
match _19 {
0 => bb1,
240772883659323457032836088919621922962 => bb10,
_ => bb9
}
}
bb17 = {
Goto(bb5)
}
bb18 = {
(*_20).0 = [_11];
_18.0 = core::ptr::addr_of!(_7);
(*_20).0 = [_11];
(*_20).0 = [_11];
_6 = _16 as isize;
_21 = &_10;
_17.0 = [_11];
Goto(bb4)
}
bb19 = {
_9 = _3;
RET = (-6744423250092667270_i64) | (-24313180361779086_i64);
_1 = _3;
_12 = 1018_i16 as f64;
_8 = _5;
_6 = (-11308_i16) as isize;
_8 = [(-176165323_i32),814563809_i32,1946972975_i32];
_4 = _13 < _1;
_16 = _9 as usize;
_2 = [(-1338459719_i32),1267510034_i32,1355728021_i32];
_14 = _10 as isize;
_14 = _6;
RET = (-1182442794931095974_i64) >> _7;
_19 = 10719919885326231211359127545376099238_i128 + (-131394194627256073183743956754210555391_i128);
_6 = 16340891779243798504_u64 as isize;
_20 = core::ptr::addr_of!(_17);
Call((*_20) = fn2(_19, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb20 = {
_11 = 239267082019389293214515004388815382498_u128 - 40205429159526793952382387784362956113_u128;
_22 = _12 * _12;
_13 = _3;
_11 = !100158024788144257004046824043049684844_u128;
_20 = core::ptr::addr_of!((*_20));
_8 = _5;
_19 = _29 >> _38;
_10 = (*_21);
_8 = _2;
(*_20).0 = [_11];
Goto(bb21)
}
bb21 = {
Call(_41 = dump_var(1_usize, 17_usize, Move(_17), 30_usize, Move(_30), 28_usize, Move(_28), 4_usize, Move(_4)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_41 = dump_var(1_usize, 5_usize, Move(_5), 11_usize, Move(_11), 38_usize, Move(_38), 15_usize, Move(_15)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_41 = dump_var(1_usize, 29_usize, Move(_29), 1_usize, Move(_1), 19_usize, Move(_19), 16_usize, Move(_16)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_41 = dump_var(1_usize, 10_usize, Move(_10), 42_usize, _42, 42_usize, _42, 42_usize, _42), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i128,mut _2: isize) -> ([u128; 1],) {
mir! {
type RET = ([u128; 1],);
let _3: [i16; 1];
let _4: (*const isize, u128, Adt63);
let _5: &'static *const i128;
let _6: i128;
let _7: isize;
let _8: [u128; 1];
let _9: [u8; 4];
let _10: *const u32;
let _11: (f32, &'static (Adt24, *const *const isize, u16, *const u8));
let _12: (f32, &'static (Adt24, *const *const isize, u16, *const u8));
let _13: [char; 7];
let _14: f32;
let _15: [isize; 6];
let _16: (f64, i64, &'static *const i128);
let _17: bool;
let _18: i128;
let _19: isize;
let _20: f32;
let _21: [i32; 3];
let _22: bool;
let _23: f32;
let _24: [i16; 5];
let _25: bool;
let _26: *const &'static u8;
let _27: bool;
let _28: f32;
let _29: ();
let _30: ();
{
_1 = (-154420759393361137699704522092309630568_i128);
RET.0 = [75360177720771359957335699940911632876_u128];
RET.0 = [211792838397026445449727358498561434361_u128];
RET.0 = [198699107639360904554683550579504490005_u128];
_1 = 53794685148582474698493858711119144808_i128 * (-94215193015201017009637988297322533063_i128);
RET.0 = [67814931870994592617962000554594767452_u128];
_2 = (-9223372036854775808_isize);
RET.0 = [34638134773045407023900575169563983680_u128];
RET.0 = [153564943565658116923898445301122728145_u128];
_2 = 9223372036854775807_isize + (-9223372036854775808_isize);
_2 = !9223372036854775807_isize;
RET.0 = [78885027163818484819202777104471160391_u128];
RET.0 = [90417207776063730352761166477888706779_u128];
_1 = -160089891046777242809371481613864978372_i128;
RET.0 = [178113796839258325536813699150097180590_u128];
RET.0 = [68658825664273672973149878133602388090_u128];
_1 = 16710_i16 as i128;
_1 = (-32325887217141996444553391566633059216_i128);
RET.0 = [179796190739581280700838656148416799825_u128];
_1 = !8054791090695501038100947439481671625_i128;
_2 = 15542578918850936608_usize as isize;
Goto(bb1)
}
bb1 = {
_2 = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
_2 = 9223372036854775807_isize;
_1 = 298780707965661476387712207671055466871_u128 as i128;
RET.0 = [247022726769431853286585542225059615282_u128];
RET.0 = [175823267792869701279757715612463837820_u128];
RET.0 = [174264046766056496080147735974169094142_u128];
_1 = !46620173836193556787320959209525687232_i128;
_2 = 894701761_i32 as isize;
_2 = (-3039_i16) as isize;
_2 = _1 as isize;
RET.0 = [119024729207688320875282870068100007482_u128];
RET.0 = [297225342372664037844780424437045886715_u128];
_1 = (-86014324751507838215261534942935271682_i128) * (-130802279595776808931898073599071347356_i128);
_2 = 9223372036854775807_isize;
_1 = 81856398902159691085610753882973900807_i128 + (-148644709208319595922227883185737430546_i128);
RET.0 = [177535231955658223448954798674036626688_u128];
_1 = -(-67683665730385979181787382362790307506_i128);
_2 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
Goto(bb2)
}
bb2 = {
_4.1 = !12909531991868460469885124463479664886_u128;
_4.0 = core::ptr::addr_of!(_2);
_2 = 37_i8 as isize;
_3 = [2125_i16];
_4.1 = 196965233633658936733499450600949090521_u128 | 21181966058225393371039813582499317354_u128;
_1 = (-24385996400823429760459781641968967324_i128) & (-140429198078299498662141597919996667392_i128);
_4.1 = (-5622_i16) as u128;
_4.1 = 330637769580589591837454393936502218259_u128 << _1;
RET.0 = [_4.1];
_1 = (-95000961189206427910985110518200101945_i128) << _2;
_4.1 = !123036871249569187070338534123879297321_u128;
_1 = (-67692026611182885703325485223491990794_i128) & (-79237496650374945108814792205506097454_i128);
_6 = !_1;
RET.0 = [_4.1];
_4.0 = core::ptr::addr_of!(_2);
RET.0 = [_4.1];
_3 = [(-9203_i16)];
RET.0 = [_4.1];
_4.1 = 33005776254643145127459622864764946802_u128;
_6 = !_1;
_1 = _6 & _6;
_4.0 = core::ptr::addr_of!(_2);
_2 = (-9223372036854775808_isize);
RET.0 = [_4.1];
_2 = -(-9223372036854775808_isize);
Call(_4 = fn3(_2, _3, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
place!(Field::<[usize; 8]>(Variant(_4.2, 2), 2)) = [2_usize,7_usize,8447241431774829971_usize,4_usize,7_usize,2_usize,0_usize,18144030435450684573_usize];
place!(Field::<f64>(Variant(_4.2, 2), 0)) = 43090_u16 as f64;
_4.1 = 1591944105055197743_i64 as u128;
_4.1 = !89542787876819302416918090965103740580_u128;
SetDiscriminant(_4.2, 0);
_2 = 5468903439240512123_i64 as isize;
place!(Field::<usize>(Variant(_4.2, 0), 0)) = 17762026309297395871_usize >> _1;
_7 = _2;
place!(Field::<*const *const isize>(Variant(_4.2, 0), 3)) = core::ptr::addr_of!(_4.0);
place!(Field::<*const *const isize>(Variant(_4.2, 0), 3)) = core::ptr::addr_of!(_4.0);
RET.0 = [_4.1];
_3 = [30121_i16];
_6 = -_1;
_8 = [_4.1];
_2 = -_7;
place!(Field::<usize>(Variant(_4.2, 0), 0)) = 6_usize ^ 7_usize;
_7 = _2 ^ _2;
place!(Field::<usize>(Variant(_4.2, 0), 0)) = !7110357509753427267_usize;
_3 = [(-17794_i16)];
_6 = 300264013884289984_i64 as i128;
_4.0 = core::ptr::addr_of!(_7);
_9 = [13_u8,2_u8,122_u8,130_u8];
place!(Field::<[bool; 4]>(Variant(_4.2, 0), 4)) = [false,true,true,true];
place!(Field::<[u16; 8]>(Variant(_4.2, 0), 2)) = [10246_u16,9207_u16,38570_u16,11730_u16,63072_u16,23343_u16,37974_u16,3259_u16];
place!(Field::<*const *const isize>(Variant(_4.2, 0), 3)) = core::ptr::addr_of!(_4.0);
Goto(bb4)
}
bb4 = {
_4.0 = core::ptr::addr_of!(_7);
_4.0 = core::ptr::addr_of!(_7);
_4.1 = 1790062542_u32 as u128;
RET = (_8,);
_4.1 = 191717087805380817582678450737402462444_u128 >> _1;
_1 = Field::<usize>(Variant(_4.2, 0), 0) as i128;
_4.0 = core::ptr::addr_of!(_7);
place!(Field::<usize>(Variant(_4.2, 0), 0)) = 1_usize * 4045450188205085230_usize;
RET.0 = _8;
_4.1 = 139085332035810103048293402016526570306_u128 ^ 217698606957196030533438012260257453217_u128;
_9 = [89_u8,48_u8,237_u8,217_u8];
place!(Field::<*const *const isize>(Variant(_4.2, 0), 3)) = core::ptr::addr_of!(_4.0);
place!(Field::<[u16; 8]>(Variant(_4.2, 0), 2)) = [52480_u16,30810_u16,21924_u16,59245_u16,55072_u16,49632_u16,36738_u16,61475_u16];
RET = (_8,);
_9 = [92_u8,232_u8,164_u8,95_u8];
_7 = _2 | _2;
RET = (_8,);
place!(Field::<*const *const isize>(Variant(_4.2, 0), 3)) = core::ptr::addr_of!(_4.0);
_14 = 2096583029_u32 as f32;
_8 = [_4.1];
place!(Field::<usize>(Variant(_4.2, 0), 0)) = !13619542564550559591_usize;
_7 = 47561_u16 as isize;
Goto(bb5)
}
bb5 = {
_9 = [94_u8,250_u8,172_u8,47_u8];
_11.0 = _6 as f32;
_15 = [_7,_2,_2,_7,_2,_2];
_13 = ['\u{95f39}','\u{496dc}','\u{2a219}','\u{88306}','\u{9a383}','\u{107aa5}','\u{a82a9}'];
place!(Field::<*const *const isize>(Variant(_4.2, 0), 3)) = core::ptr::addr_of!(_4.0);
place!(Field::<[bool; 4]>(Variant(_4.2, 0), 4)) = [true,true,false,false];
_16.0 = 3757663875_u32 as f64;
_7 = _2 ^ _2;
RET = (_8,);
RET.0 = _8;
RET.0 = [_4.1];
RET = (_8,);
_15 = [_2,_7,_2,_7,_7,_7];
_9 = [244_u8,162_u8,208_u8,238_u8];
_17 = _4.1 >= _4.1;
Goto(bb6)
}
bb6 = {
place!(Field::<*const *const isize>(Variant(_4.2, 0), 3)) = core::ptr::addr_of!(_4.0);
_17 = !true;
_2 = !_7;
_4.1 = 111313042985146218324581030140039683089_u128 >> _2;
_9 = [85_u8,133_u8,213_u8,66_u8];
_15 = [_7,_2,_7,_7,_2,_7];
_4.0 = core::ptr::addr_of!(_7);
RET.0 = [_4.1];
_12.0 = -_11.0;
_11.0 = _14;
_6 = _1 | _1;
Goto(bb7)
}
bb7 = {
_8 = [_4.1];
_13 = ['\u{7ed04}','\u{10cc45}','\u{1dd3}','\u{c2e2e}','\u{c43d0}','\u{b7095}','\u{e0b0f}'];
_4.1 = 117937147431988913548365175866028439048_u128;
_20 = 1082385519_u32 as f32;
place!(Field::<[bool; 4]>(Variant(_4.2, 0), 4)) = [_17,_17,_17,_17];
_12.0 = (-16005725521825250_i64) as f32;
place!(Field::<usize>(Variant(_4.2, 0), 0)) = !10242726876469148545_usize;
Goto(bb8)
}
bb8 = {
place!(Field::<[bool; 4]>(Variant(_4.2, 0), 4)) = [_17,_17,_17,_17];
_21 = [(-1959132653_i32),(-560985815_i32),1859640219_i32];
_16.1 = !2439431141894766547_i64;
_15 = [_2,_7,_7,_2,_7,_2];
_4.0 = core::ptr::addr_of!(_19);
_22 = !_17;
_16.1 = 1886449825313924533_i64 << _6;
_19 = !_7;
place!(Field::<usize>(Variant(_4.2, 0), 0)) = 15922849512631926568_usize;
RET = (_8,);
_18 = _6;
_6 = _18 - _1;
place!(Field::<[bool; 4]>(Variant(_4.2, 0), 4)) = [_22,_17,_17,_22];
Call(_4.1 = core::intrinsics::transmute(Field::<[u16; 8]>(Variant(_4.2, 0), 2)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_16.0 = 2821513246_u32 as f64;
_13 = ['\u{d0d78}','\u{9c778}','\u{e4457}','\u{4c90f}','\u{9ca5}','\u{10c0c7}','\u{aa34e}'];
_14 = _12.0 * _12.0;
_13 = ['\u{f70d}','\u{19962}','\u{cbb73}','\u{817ae}','\u{daab5}','\u{10a66}','\u{758ea}'];
RET.0 = _8;
_1 = !_18;
_2 = -_19;
_11.0 = _20 - _20;
_15 = [_19,_2,_2,_2,_2,_7];
_23 = _16.1 as f32;
_13 = ['\u{7ef8f}','\u{6b1b1}','\u{17537}','\u{94833}','\u{b23f9}','\u{a39a6}','\u{2}'];
match Field::<usize>(Variant(_4.2, 0), 0) {
15922849512631926568 => bb11,
_ => bb10
}
}
bb10 = {
place!(Field::<[bool; 4]>(Variant(_4.2, 0), 4)) = [_17,_17,_17,_17];
_21 = [(-1959132653_i32),(-560985815_i32),1859640219_i32];
_16.1 = !2439431141894766547_i64;
_15 = [_2,_7,_7,_2,_7,_2];
_4.0 = core::ptr::addr_of!(_19);
_22 = !_17;
_16.1 = 1886449825313924533_i64 << _6;
_19 = !_7;
place!(Field::<usize>(Variant(_4.2, 0), 0)) = 15922849512631926568_usize;
RET = (_8,);
_18 = _6;
_6 = _18 - _1;
place!(Field::<[bool; 4]>(Variant(_4.2, 0), 4)) = [_22,_17,_17,_22];
Call(_4.1 = core::intrinsics::transmute(Field::<[u16; 8]>(Variant(_4.2, 0), 2)), ReturnTo(bb9), UnwindUnreachable())
}
bb11 = {
_24 = [(-14210_i16),(-26168_i16),(-368_i16),(-29381_i16),(-8082_i16)];
place!(Field::<*const *const isize>(Variant(_4.2, 0), 3)) = core::ptr::addr_of!(_4.0);
match Field::<usize>(Variant(_4.2, 0), 0) {
0 => bb10,
1 => bb2,
2 => bb7,
3 => bb12,
15922849512631926568 => bb14,
_ => bb13
}
}
bb12 = {
_4.0 = core::ptr::addr_of!(_7);
_4.0 = core::ptr::addr_of!(_7);
_4.1 = 1790062542_u32 as u128;
RET = (_8,);
_4.1 = 191717087805380817582678450737402462444_u128 >> _1;
_1 = Field::<usize>(Variant(_4.2, 0), 0) as i128;
_4.0 = core::ptr::addr_of!(_7);
place!(Field::<usize>(Variant(_4.2, 0), 0)) = 1_usize * 4045450188205085230_usize;
RET.0 = _8;
_4.1 = 139085332035810103048293402016526570306_u128 ^ 217698606957196030533438012260257453217_u128;
_9 = [89_u8,48_u8,237_u8,217_u8];
place!(Field::<*const *const isize>(Variant(_4.2, 0), 3)) = core::ptr::addr_of!(_4.0);
place!(Field::<[u16; 8]>(Variant(_4.2, 0), 2)) = [52480_u16,30810_u16,21924_u16,59245_u16,55072_u16,49632_u16,36738_u16,61475_u16];
RET = (_8,);
_9 = [92_u8,232_u8,164_u8,95_u8];
_7 = _2 | _2;
RET = (_8,);
place!(Field::<*const *const isize>(Variant(_4.2, 0), 3)) = core::ptr::addr_of!(_4.0);
_14 = 2096583029_u32 as f32;
_8 = [_4.1];
place!(Field::<usize>(Variant(_4.2, 0), 0)) = !13619542564550559591_usize;
_7 = 47561_u16 as isize;
Goto(bb5)
}
bb13 = {
_16.0 = 2821513246_u32 as f64;
_13 = ['\u{d0d78}','\u{9c778}','\u{e4457}','\u{4c90f}','\u{9ca5}','\u{10c0c7}','\u{aa34e}'];
_14 = _12.0 * _12.0;
_13 = ['\u{f70d}','\u{19962}','\u{cbb73}','\u{817ae}','\u{daab5}','\u{10a66}','\u{758ea}'];
RET.0 = _8;
_1 = !_18;
_2 = -_19;
_11.0 = _20 - _20;
_15 = [_19,_2,_2,_2,_2,_7];
_23 = _16.1 as f32;
_13 = ['\u{7ef8f}','\u{6b1b1}','\u{17537}','\u{94833}','\u{b23f9}','\u{a39a6}','\u{2}'];
match Field::<usize>(Variant(_4.2, 0), 0) {
15922849512631926568 => bb11,
_ => bb10
}
}
bb14 = {
_6 = _18 | _1;
place!(Field::<[bool; 4]>(Variant(_4.2, 0), 4)) = [_22,_22,_22,_17];
_23 = -_14;
_4.1 = !324351327889231100680871904553180787266_u128;
_22 = _17;
_12.0 = _23 + _14;
_24 = [(-16452_i16),(-21463_i16),31579_i16,(-451_i16),32007_i16];
_16.0 = 11_i8 as f64;
_18 = (-29753_i16) as i128;
RET = (_8,);
_9 = [44_u8,61_u8,217_u8,240_u8];
_8 = [_4.1];
_6 = _16.1 as i128;
_14 = _11.0;
RET = (_8,);
_11.0 = _23;
_24 = [13538_i16,23656_i16,8407_i16,23295_i16,(-11203_i16)];
_12.0 = _14 * _20;
RET.0 = _8;
_4.0 = core::ptr::addr_of!(_7);
RET = (_8,);
_8 = RET.0;
_13 = ['\u{107184}','\u{eccfa}','\u{fab91}','\u{dae12}','\u{d079f}','\u{34a88}','\u{864a9}'];
_16.0 = 561_u16 as f64;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(2_usize, 3_usize, Move(_3), 8_usize, Move(_8), 18_usize, Move(_18), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(2_usize, 22_usize, Move(_22), 17_usize, Move(_17), 6_usize, Move(_6), 30_usize, _30), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: isize,mut _2: [i16; 1],mut _3: ([u128; 1],)) -> (*const isize, u128, Adt63) {
mir! {
type RET = (*const isize, u128, Adt63);
let _4: u16;
let _5: &'static u32;
let _6: f64;
let _7: ([bool; 4], [usize; 8]);
let _8: isize;
let _9: &'static Adt34;
let _10: i8;
let _11: [i32; 1];
let _12: isize;
let _13: isize;
let _14: f64;
let _15: f64;
let _16: [i16; 1];
let _17: *mut *const u8;
let _18: [i8; 7];
let _19: isize;
let _20: (f64, i64, &'static *const i128);
let _21: bool;
let _22: usize;
let _23: Adt71;
let _24: [i32; 1];
let _25: [usize; 8];
let _26: f64;
let _27: (*const isize, u128, Adt63);
let _28: *const &'static u8;
let _29: usize;
let _30: &'static f32;
let _31: bool;
let _32: ([bool; 4], [usize; 8]);
let _33: [i8; 7];
let _34: [u128; 1];
let _35: &'static usize;
let _36: isize;
let _37: ([u128; 1],);
let _38: isize;
let _39: *const isize;
let _40: *const ([u128; 1],);
let _41: f64;
let _42: f64;
let _43: &'static (u32, u64, f32);
let _44: usize;
let _45: isize;
let _46: f32;
let _47: (*const u8,);
let _48: Adt24;
let _49: isize;
let _50: &'static usize;
let _51: isize;
let _52: char;
let _53: &'static (u32, u64, f32);
let _54: Adt72;
let _55: u32;
let _56: isize;
let _57: *const isize;
let _58: *const char;
let _59: f32;
let _60: ();
let _61: ();
{
RET.1 = 153335617347228617516937473549351163066_u128;
RET.1 = false as u128;
RET.0 = core::ptr::addr_of!(_1);
_3.0 = [RET.1];
_2 = [8451_i16];
RET.1 = 196049580205893415671034726797878341111_u128;
_4 = 26473_u16 - 43550_u16;
RET.1 = !262883432689382647170701580255961586664_u128;
RET.0 = core::ptr::addr_of!(_1);
RET.1 = !213179024142560141676730746702150345083_u128;
RET.0 = core::ptr::addr_of!(_1);
_2 = [3016_i16];
_3.0 = [RET.1];
RET.1 = 203684377323497233544138719337743933402_u128 | 202235724725667729760055174062695537640_u128;
_2 = [(-5685_i16)];
_4 = _1 as u16;
RET.1 = 47482013772828835444177419975490338925_u128;
_4 = '\u{7a46}' as u16;
_1 = !(-9223372036854775808_isize);
RET.0 = core::ptr::addr_of!(_1);
_2 = [12091_i16];
RET.0 = core::ptr::addr_of!(_1);
RET.0 = core::ptr::addr_of!(_1);
RET.0 = core::ptr::addr_of!(_1);
match RET.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
47482013772828835444177419975490338925 => bb7,
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
_2 = [1179_i16];
RET.0 = core::ptr::addr_of!(_1);
RET.1 = 116_i8 as u128;
_6 = (-70253479270429361299352728121618543807_i128) as f64;
RET.1 = !67220771628567177872538724176319898590_u128;
RET.0 = core::ptr::addr_of!(_1);
_6 = 6951124241680301473_usize as f64;
_1 = (-9223372036854775808_isize) >> RET.1;
_1 = (-9223372036854775808_isize);
RET.0 = core::ptr::addr_of!(_1);
_3.0 = [RET.1];
_3.0 = [RET.1];
_1 = true as isize;
_7.1 = [4_usize,6605803528035154073_usize,4_usize,17588201616136306707_usize,2604651155646131183_usize,3_usize,15472325588422403720_usize,4_usize];
_7.0 = [false,true,true,true];
_4 = 37763_u16;
_3.0 = [RET.1];
RET.1 = _1 as u128;
_7.0 = [false,false,true,true];
_7.0 = [false,false,false,false];
_7.1 = [12132659114242064995_usize,12341738898636489538_usize,4_usize,7_usize,1525988920476073120_usize,8687243801896237006_usize,4519696634264350358_usize,5_usize];
_6 = 6793_i16 as f64;
Goto(bb8)
}
bb8 = {
_7.0 = [true,false,true,true];
_8 = _1 | _1;
RET.0 = core::ptr::addr_of!(_12);
_3.0 = [RET.1];
RET.0 = core::ptr::addr_of!(_8);
_6 = (-1314715277779946695_i64) as f64;
_3.0 = [RET.1];
Goto(bb9)
}
bb9 = {
_1 = _8;
_10 = 67_i8;
_2 = [21424_i16];
_16 = [(-17464_i16)];
_15 = _6 + _6;
_14 = 2569770591_u32 as f64;
_1 = !_8;
_11 = [1264398038_i32];
_11 = [1308935012_i32];
_7.0 = [false,false,true,true];
_7.0 = [true,true,true,true];
RET.0 = core::ptr::addr_of!(_8);
Goto(bb10)
}
bb10 = {
_8 = RET.1 as isize;
_19 = _1 | _1;
_12 = _1;
RET.0 = core::ptr::addr_of!(_13);
_12 = !_19;
_15 = 13799951713171173674_u64 as f64;
_11 = [(-511362708_i32)];
_18 = [_10,_10,_10,_10,_10,_10,_10];
RET.0 = core::ptr::addr_of!(_8);
_3.0 = [RET.1];
Call(_13 = fn4(), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
RET.1 = 38587943892178631580187476592491254320_u128 << _19;
_1 = true as isize;
Goto(bb12)
}
bb12 = {
RET.0 = core::ptr::addr_of!(_1);
_4 = _12 as u16;
_6 = _14;
_11 = [(-993177698_i32)];
_20.1 = (-5104951840187997387_i64) << _13;
_2 = [9088_i16];
_16 = [(-8099_i16)];
RET.0 = core::ptr::addr_of!(_1);
match _10 {
0 => bb1,
1 => bb6,
2 => bb10,
3 => bb13,
67 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
_1 = _8;
_10 = 67_i8;
_2 = [21424_i16];
_16 = [(-17464_i16)];
_15 = _6 + _6;
_14 = 2569770591_u32 as f64;
_1 = !_8;
_11 = [1264398038_i32];
_11 = [1308935012_i32];
_7.0 = [false,false,true,true];
_7.0 = [true,true,true,true];
RET.0 = core::ptr::addr_of!(_8);
Goto(bb10)
}
bb15 = {
_4 = 25698_u16 & 64642_u16;
RET.0 = core::ptr::addr_of!(_19);
_20.0 = RET.1 as f64;
_13 = _1;
_6 = _20.0;
_19 = _4 as isize;
_19 = _20.1 as isize;
_27.0 = core::ptr::addr_of!(_1);
_1 = 14069358146952182408_usize as isize;
_22 = _1 as usize;
_3.0 = [RET.1];
_6 = -_14;
RET.1 = !21705230659142892416909936894825177576_u128;
_12 = 1537192080_i32 as isize;
RET.0 = core::ptr::addr_of!(_12);
_1 = !_19;
_19 = 38496797317950279346291649633694910820_i128 as isize;
match _10 {
0 => bb4,
1 => bb11,
2 => bb16,
3 => bb17,
4 => bb18,
5 => bb19,
67 => bb21,
_ => bb20
}
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
RET.1 = 38587943892178631580187476592491254320_u128 << _19;
_1 = true as isize;
Goto(bb12)
}
bb20 = {
_2 = [1179_i16];
RET.0 = core::ptr::addr_of!(_1);
RET.1 = 116_i8 as u128;
_6 = (-70253479270429361299352728121618543807_i128) as f64;
RET.1 = !67220771628567177872538724176319898590_u128;
RET.0 = core::ptr::addr_of!(_1);
_6 = 6951124241680301473_usize as f64;
_1 = (-9223372036854775808_isize) >> RET.1;
_1 = (-9223372036854775808_isize);
RET.0 = core::ptr::addr_of!(_1);
_3.0 = [RET.1];
_3.0 = [RET.1];
_1 = true as isize;
_7.1 = [4_usize,6605803528035154073_usize,4_usize,17588201616136306707_usize,2604651155646131183_usize,3_usize,15472325588422403720_usize,4_usize];
_7.0 = [false,true,true,true];
_4 = 37763_u16;
_3.0 = [RET.1];
RET.1 = _1 as u128;
_7.0 = [false,false,true,true];
_7.0 = [false,false,false,false];
_7.1 = [12132659114242064995_usize,12341738898636489538_usize,4_usize,7_usize,1525988920476073120_usize,8687243801896237006_usize,4519696634264350358_usize,5_usize];
_6 = 6793_i16 as f64;
Goto(bb8)
}
bb21 = {
_7.0 = [true,false,true,true];
_26 = -_20.0;
_24 = [(-988396614_i32)];
_1 = _12 ^ _12;
_24 = [(-707170714_i32)];
_12 = _15 as isize;
Goto(bb22)
}
bb22 = {
_14 = _6 + _20.0;
RET.0 = Move(_27.0);
_24 = [(-219363615_i32)];
_25 = [_22,_22,_22,_22,_22,_22,_22,_22];
_6 = _14;
RET.0 = core::ptr::addr_of!(_1);
_21 = _20.0 > _26;
_25 = _7.1;
_1 = _21 as isize;
_16 = _2;
RET.0 = core::ptr::addr_of!(_12);
_27.1 = _20.1 as u128;
_18 = [_10,_10,_10,_10,_10,_10,_10];
RET.0 = core::ptr::addr_of!(_12);
_24 = [551161672_i32];
_27.0 = Move(RET.0);
_29 = _22 | _22;
_7.0 = [_21,_21,_21,_21];
_15 = -_14;
_24 = [93857220_i32];
_32.0 = _7.0;
_1 = !_13;
_20.0 = _15 - _15;
_24 = [2121656831_i32];
match _10 {
0 => bb16,
1 => bb2,
2 => bb19,
3 => bb17,
4 => bb23,
5 => bb24,
67 => bb26,
_ => bb25
}
}
bb23 = {
Return()
}
bb24 = {
_1 = _8;
_10 = 67_i8;
_2 = [21424_i16];
_16 = [(-17464_i16)];
_15 = _6 + _6;
_14 = 2569770591_u32 as f64;
_1 = !_8;
_11 = [1264398038_i32];
_11 = [1308935012_i32];
_7.0 = [false,false,true,true];
_7.0 = [true,true,true,true];
RET.0 = core::ptr::addr_of!(_8);
Goto(bb10)
}
bb25 = {
RET.1 = 38587943892178631580187476592491254320_u128 << _19;
_1 = true as isize;
Goto(bb12)
}
bb26 = {
_4 = 6710_u16;
_35 = &_29;
_27.1 = RET.1 | RET.1;
RET.0 = core::ptr::addr_of!(_8);
_32 = _7;
RET.1 = !_27.1;
_38 = 15487463391849653596_u64 as isize;
_33 = _18;
_37.0 = _3.0;
_3 = (_37.0,);
RET.0 = Move(_27.0);
RET.0 = core::ptr::addr_of!(_8);
_7 = (_32.0, _32.1);
_15 = (-134718691433174340895172441426827104724_i128) as f64;
Goto(bb27)
}
bb27 = {
_16 = [12549_i16];
_3.0 = [_27.1];
_32 = (_7.0, _7.1);
_16 = [(-3842_i16)];
RET.0 = core::ptr::addr_of!(_13);
_38 = _19;
_14 = _20.0 + _26;
RET.1 = _27.1;
_22 = _27.1 as usize;
_39 = core::ptr::addr_of!(_12);
_27.1 = RET.1;
_19 = !_12;
_20.0 = _6;
match _4 {
0 => bb1,
1 => bb25,
2 => bb9,
3 => bb28,
6710 => bb30,
_ => bb29
}
}
bb28 = {
Return()
}
bb29 = {
RET.1 = 38587943892178631580187476592491254320_u128 << _19;
_1 = true as isize;
Goto(bb12)
}
bb30 = {
(*_39) = _8 & _38;
RET.0 = Move(_39);
_40 = core::ptr::addr_of!(_37);
RET.2 = Adt63::Variant2 { fld0: _20.0,fld1: '\u{5cc55}',fld2: _25,fld3: Move(_40) };
place!(Field::<[usize; 8]>(Variant(RET.2, 2), 2)) = _25;
_4 = 48589_u16;
_41 = 7505_i16 as f64;
RET.0 = core::ptr::addr_of!(_1);
_12 = _8 & _1;
_12 = _8 + _1;
_44 = _29;
_19 = _13 | _12;
_39 = Move(RET.0);
place!(Field::<[usize; 8]>(Variant(RET.2, 2), 2)) = _25;
place!(Field::<*const ([u128; 1],)>(Variant(RET.2, 2), 3)) = core::ptr::addr_of!(_37);
_44 = _22;
_11 = [704958268_i32];
_20.1 = (-6346668190462843287_i64) * 3374909494098275421_i64;
Call(_26 = core::intrinsics::fmaf64(_14, _14, _6), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
_45 = !_8;
_46 = _10 as f32;
place!(Field::<char>(Variant(RET.2, 2), 1)) = '\u{68df5}';
place!(Field::<char>(Variant(RET.2, 2), 1)) = '\u{5dcfe}';
_15 = _14 - _14;
_8 = _21 as isize;
_36 = _12 | _8;
SetDiscriminant(RET.2, 2);
RET.0 = core::ptr::addr_of!(_36);
_38 = _46 as isize;
place!(Field::<char>(Variant(RET.2, 2), 1)) = '\u{5f828}';
place!(Field::<[usize; 8]>(Variant(RET.2, 2), 2)) = _25;
_21 = !false;
match _4 {
0 => bb32,
1 => bb33,
2 => bb34,
3 => bb35,
48589 => bb37,
_ => bb36
}
}
bb32 = {
(*_39) = _8 & _38;
RET.0 = Move(_39);
_40 = core::ptr::addr_of!(_37);
RET.2 = Adt63::Variant2 { fld0: _20.0,fld1: '\u{5cc55}',fld2: _25,fld3: Move(_40) };
place!(Field::<[usize; 8]>(Variant(RET.2, 2), 2)) = _25;
_4 = 48589_u16;
_41 = 7505_i16 as f64;
RET.0 = core::ptr::addr_of!(_1);
_12 = _8 & _1;
_12 = _8 + _1;
_44 = _29;
_19 = _13 | _12;
_39 = Move(RET.0);
place!(Field::<[usize; 8]>(Variant(RET.2, 2), 2)) = _25;
place!(Field::<*const ([u128; 1],)>(Variant(RET.2, 2), 3)) = core::ptr::addr_of!(_37);
_44 = _22;
_11 = [704958268_i32];
_20.1 = (-6346668190462843287_i64) * 3374909494098275421_i64;
Call(_26 = core::intrinsics::fmaf64(_14, _14, _6), ReturnTo(bb31), UnwindUnreachable())
}
bb33 = {
Return()
}
bb34 = {
Return()
}
bb35 = {
Return()
}
bb36 = {
Return()
}
bb37 = {
_40 = core::ptr::addr_of!(_3);
_30 = &_46;
place!(Field::<[usize; 8]>(Variant(RET.2, 2), 2)) = [_44,(*_35),(*_35),_29,_29,_44,_22,(*_35)];
_33 = _18;
RET.2 = Adt63::Variant2 { fld0: _15,fld1: '\u{761b4}',fld2: _32.1,fld3: Move(_40) };
_42 = _20.0 - Field::<f64>(Variant(RET.2, 2), 0);
_4 = 11494_u16 | 4017_u16;
RET.0 = Move(_39);
_13 = _8 ^ _36;
_7 = _32;
_15 = (-37939324241061879730825106323176875289_i128) as f64;
_45 = _36 + _13;
_40 = core::ptr::addr_of!(_3);
_50 = Move(_35);
_39 = core::ptr::addr_of!(_8);
_32.1 = [_29,_44,_29,_22,_44,_44,_44,_44];
(*_40) = _37;
RET.0 = Move(_39);
Goto(bb38)
}
bb38 = {
_10 = -(-60_i8);
_36 = _45;
_51 = _13;
_20.1 = !311602113243149322_i64;
_45 = _36;
_17 = core::ptr::addr_of_mut!(_47.0);
_11 = [(-301344003_i32)];
place!(Field::<char>(Variant(RET.2, 2), 1)) = '\u{18151}';
_7.1 = _25;
_37 = (*_40);
_39 = core::ptr::addr_of!(_12);
place!(Field::<*const ([u128; 1],)>(Variant(RET.2, 2), 3)) = core::ptr::addr_of!(_37);
(*_40) = (_37.0,);
_27 = (Move(RET.0), RET.1, Move(RET.2));
RET.2 = Adt63::Variant2 { fld0: _42,fld1: Field::<char>(Variant(_27.2, 2), 1),fld2: Field::<[usize; 8]>(Variant(_27.2, 2), 2),fld3: Move(_40) };
Goto(bb39)
}
bb39 = {
_54.fld0 = [1016202959_i32,(-1837024925_i32),(-1919341367_i32)];
_37.0 = [_27.1];
_7.0 = [_21,_21,_21,_21];
_19 = !_36;
_27.2 = Move(RET.2);
_1 = _13;
_25 = [_29,_29,_29,_29,_22,_22,_22,_29];
_10 = (-16_i8) << _1;
_11 = _24;
place!(Field::<[usize; 8]>(Variant(_27.2, 2), 2)) = [_29,_44,_22,_22,_22,_22,_29,_44];
_57 = Move(_27.0);
_55 = 4042848630_u32;
(*_39) = _51 ^ _1;
_20.1 = (-8887946319957341694_i64) >> (*_39);
RET = (Move(_39), _27.1, Move(_27.2));
_2 = _16;
_29 = _46 as usize;
_17 = core::ptr::addr_of_mut!((*_17));
_19 = _13;
Goto(bb40)
}
bb40 = {
Call(_60 = dump_var(3_usize, 11_usize, Move(_11), 32_usize, Move(_32), 13_usize, Move(_13), 10_usize, Move(_10)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Call(_60 = dump_var(3_usize, 38_usize, Move(_38), 7_usize, Move(_7), 25_usize, Move(_25), 8_usize, Move(_8)), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Call(_60 = dump_var(3_usize, 2_usize, Move(_2), 18_usize, Move(_18), 19_usize, Move(_19), 55_usize, Move(_55)), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
Call(_60 = dump_var(3_usize, 29_usize, Move(_29), 61_usize, _61, 61_usize, _61, 61_usize, _61), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4() -> isize {
mir! {
type RET = isize;
let _1: isize;
let _2: [u32; 5];
let _3: u64;
let _4: [u16; 8];
let _5: [u128; 1];
let _6: [char; 7];
let _7: ([u64; 4], ([u128; 1],), *mut &'static u32, &'static Adt34);
let _8: f64;
let _9: bool;
let _10: ([i32; 3],);
let _11: (*mut *const u8, (*const *const isize, *mut *const u8, [u16; 8], i16), [u128; 1]);
let _12: u64;
let _13: i128;
let _14: [i16; 5];
let _15: isize;
let _16: &'static f32;
let _17: ([bool; 4], [usize; 8]);
let _18: ();
let _19: ();
{
RET = 9223372036854775807_isize;
RET = 9223372036854775807_isize;
_1 = RET & RET;
RET = !_1;
_2 = [1394436472_u32,1721166939_u32,2158372055_u32,3791852610_u32,1960621778_u32];
_3 = 16063638482660135457_u64 >> RET;
_3 = 16081612092788819505_u64 + 35795815486700367_u64;
RET = 211_u8 as isize;
_3 = 9275435970244522149_u64 & 15688600017081136204_u64;
_3 = 163926002291996828703297358577194568750_u128 as u64;
RET = '\u{f4d26}' as isize;
_2 = [2540185201_u32,2190123786_u32,1494211078_u32,880986264_u32,3952230325_u32];
_2 = [939549437_u32,1680234370_u32,2633578842_u32,135618483_u32,1163577977_u32];
_1 = 11645145660109512698_usize as isize;
RET = 1792144479_u32 as isize;
RET = (-100_i8) as isize;
Goto(bb1)
}
bb1 = {
_2 = [2444585695_u32,57718651_u32,2009748174_u32,1727928362_u32,2871340685_u32];
_2 = [3415689525_u32,4234597370_u32,3137470808_u32,3675658304_u32,475449303_u32];
_3 = 1857583187825476611_u64;
_1 = 1_usize as isize;
RET = _1;
_2 = [3474872282_u32,187681287_u32,3474844167_u32,588589045_u32,1139885742_u32];
RET = -_1;
_2 = [105026019_u32,469158507_u32,4005590339_u32,2805798376_u32,2684536161_u32];
_3 = 11820871512136778577_u64 & 15773894226452021303_u64;
_5 = [132487523947596303547466696055190442992_u128];
_2 = [2764297435_u32,666654786_u32,4210571333_u32,110406742_u32,1728533756_u32];
RET = -_1;
RET = !_1;
_6 = ['\u{390bf}','\u{bbf1e}','\u{51e0d}','\u{10b300}','\u{103f4f}','\u{cfab1}','\u{cc72c}'];
RET = _1;
_7.0 = [_3,_3,_3,_3];
_3 = !12496659315890370570_u64;
_7.1 = (_5,);
_7.1 = (_5,);
RET = _1 + _1;
Goto(bb2)
}
bb2 = {
_1 = RET >> RET;
RET = '\u{bba1e}' as isize;
_7.1.0 = [175246503053757725326913802247293974769_u128];
_7.1.0 = [216860814260888167634520690071001239681_u128];
_2 = [2023683388_u32,1762734724_u32,2139085980_u32,1958823983_u32,1777099763_u32];
_6 = ['\u{99610}','\u{89200}','\u{a74ef}','\u{da923}','\u{1b836}','\u{3b119}','\u{a245}'];
_5 = [217633527967719008758566848390074100701_u128];
_9 = _1 > _1;
_6 = ['\u{c3132}','\u{a94fc}','\u{ec9ce}','\u{5d18d}','\u{e97df}','\u{2822b}','\u{3931a}'];
_1 = RET;
_4 = [15034_u16,5765_u16,23658_u16,27983_u16,62495_u16,23422_u16,15421_u16,54671_u16];
_7.1 = (_5,);
_8 = (-78313108228049718041256945845743479766_i128) as f64;
RET = _1 ^ _1;
_8 = _3 as f64;
_6 = ['\u{8755c}','\u{bfe3d}','\u{95c9c}','\u{2d5bd}','\u{437d0}','\u{b832a}','\u{facc4}'];
_7.1.0 = [46346204702854826495863169876479819490_u128];
_7.0 = [_3,_3,_3,_3];
_5 = [23911062143356934164331166742989031799_u128];
_7.0 = [_3,_3,_3,_3];
_7.1.0 = [52846569438530622900281686446188530840_u128];
_3 = 2092124838281534057_u64 * 6733662572841460329_u64;
_3 = 3098863533438532346_u64;
_10.0 = [(-1493304593_i32),(-1258910394_i32),1067285837_i32];
match _3 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
3098863533438532346 => bb7,
_ => bb6
}
}
bb3 = {
_2 = [2444585695_u32,57718651_u32,2009748174_u32,1727928362_u32,2871340685_u32];
_2 = [3415689525_u32,4234597370_u32,3137470808_u32,3675658304_u32,475449303_u32];
_3 = 1857583187825476611_u64;
_1 = 1_usize as isize;
RET = _1;
_2 = [3474872282_u32,187681287_u32,3474844167_u32,588589045_u32,1139885742_u32];
RET = -_1;
_2 = [105026019_u32,469158507_u32,4005590339_u32,2805798376_u32,2684536161_u32];
_3 = 11820871512136778577_u64 & 15773894226452021303_u64;
_5 = [132487523947596303547466696055190442992_u128];
_2 = [2764297435_u32,666654786_u32,4210571333_u32,110406742_u32,1728533756_u32];
RET = -_1;
RET = !_1;
_6 = ['\u{390bf}','\u{bbf1e}','\u{51e0d}','\u{10b300}','\u{103f4f}','\u{cfab1}','\u{cc72c}'];
RET = _1;
_7.0 = [_3,_3,_3,_3];
_3 = !12496659315890370570_u64;
_7.1 = (_5,);
_7.1 = (_5,);
RET = _1 + _1;
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
_1 = 4107086741880773844_i64 as isize;
RET = _1;
_9 = true;
Call(_11.1.3 = fn5(), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_1 = RET | RET;
RET = 4509396409183967831_i64 as isize;
_11.1.2 = [23618_u16,9771_u16,21342_u16,11867_u16,19670_u16,30331_u16,24693_u16,28198_u16];
_10.0 = [(-790405440_i32),(-540397157_i32),177641682_i32];
RET = _1;
_1 = RET << _11.1.3;
match _3 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
3098863533438532346 => bb17,
_ => bb16
}
}
bb9 = {
_1 = 4107086741880773844_i64 as isize;
RET = _1;
_9 = true;
Call(_11.1.3 = fn5(), ReturnTo(bb8), UnwindUnreachable())
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
_2 = [2444585695_u32,57718651_u32,2009748174_u32,1727928362_u32,2871340685_u32];
_2 = [3415689525_u32,4234597370_u32,3137470808_u32,3675658304_u32,475449303_u32];
_3 = 1857583187825476611_u64;
_1 = 1_usize as isize;
RET = _1;
_2 = [3474872282_u32,187681287_u32,3474844167_u32,588589045_u32,1139885742_u32];
RET = -_1;
_2 = [105026019_u32,469158507_u32,4005590339_u32,2805798376_u32,2684536161_u32];
_3 = 11820871512136778577_u64 & 15773894226452021303_u64;
_5 = [132487523947596303547466696055190442992_u128];
_2 = [2764297435_u32,666654786_u32,4210571333_u32,110406742_u32,1728533756_u32];
RET = -_1;
RET = !_1;
_6 = ['\u{390bf}','\u{bbf1e}','\u{51e0d}','\u{10b300}','\u{103f4f}','\u{cfab1}','\u{cc72c}'];
RET = _1;
_7.0 = [_3,_3,_3,_3];
_3 = !12496659315890370570_u64;
_7.1 = (_5,);
_7.1 = (_5,);
RET = _1 + _1;
Goto(bb2)
}
bb14 = {
_1 = RET >> RET;
RET = '\u{bba1e}' as isize;
_7.1.0 = [175246503053757725326913802247293974769_u128];
_7.1.0 = [216860814260888167634520690071001239681_u128];
_2 = [2023683388_u32,1762734724_u32,2139085980_u32,1958823983_u32,1777099763_u32];
_6 = ['\u{99610}','\u{89200}','\u{a74ef}','\u{da923}','\u{1b836}','\u{3b119}','\u{a245}'];
_5 = [217633527967719008758566848390074100701_u128];
_9 = _1 > _1;
_6 = ['\u{c3132}','\u{a94fc}','\u{ec9ce}','\u{5d18d}','\u{e97df}','\u{2822b}','\u{3931a}'];
_1 = RET;
_4 = [15034_u16,5765_u16,23658_u16,27983_u16,62495_u16,23422_u16,15421_u16,54671_u16];
_7.1 = (_5,);
_8 = (-78313108228049718041256945845743479766_i128) as f64;
RET = _1 ^ _1;
_8 = _3 as f64;
_6 = ['\u{8755c}','\u{bfe3d}','\u{95c9c}','\u{2d5bd}','\u{437d0}','\u{b832a}','\u{facc4}'];
_7.1.0 = [46346204702854826495863169876479819490_u128];
_7.0 = [_3,_3,_3,_3];
_5 = [23911062143356934164331166742989031799_u128];
_7.0 = [_3,_3,_3,_3];
_7.1.0 = [52846569438530622900281686446188530840_u128];
_3 = 2092124838281534057_u64 * 6733662572841460329_u64;
_3 = 3098863533438532346_u64;
_10.0 = [(-1493304593_i32),(-1258910394_i32),1067285837_i32];
match _3 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
3098863533438532346 => bb7,
_ => bb6
}
}
bb15 = {
_2 = [2444585695_u32,57718651_u32,2009748174_u32,1727928362_u32,2871340685_u32];
_2 = [3415689525_u32,4234597370_u32,3137470808_u32,3675658304_u32,475449303_u32];
_3 = 1857583187825476611_u64;
_1 = 1_usize as isize;
RET = _1;
_2 = [3474872282_u32,187681287_u32,3474844167_u32,588589045_u32,1139885742_u32];
RET = -_1;
_2 = [105026019_u32,469158507_u32,4005590339_u32,2805798376_u32,2684536161_u32];
_3 = 11820871512136778577_u64 & 15773894226452021303_u64;
_5 = [132487523947596303547466696055190442992_u128];
_2 = [2764297435_u32,666654786_u32,4210571333_u32,110406742_u32,1728533756_u32];
RET = -_1;
RET = !_1;
_6 = ['\u{390bf}','\u{bbf1e}','\u{51e0d}','\u{10b300}','\u{103f4f}','\u{cfab1}','\u{cc72c}'];
RET = _1;
_7.0 = [_3,_3,_3,_3];
_3 = !12496659315890370570_u64;
_7.1 = (_5,);
_7.1 = (_5,);
RET = _1 + _1;
Goto(bb2)
}
bb16 = {
Return()
}
bb17 = {
_11.1.3 = 20782_i16 | 23951_i16;
_7.1.0 = _5;
_10.0 = [(-1995377_i32),(-609174028_i32),538431378_i32];
_11.2 = [221037811746762464032487304934621453303_u128];
_7.1.0 = _11.2;
_7.0 = [_3,_3,_3,_3];
_12 = _3;
_15 = RET & _1;
_6 = ['\u{86ea2}','\u{821fb}','\u{1de8}','\u{7dea1}','\u{e4dfa}','\u{5156b}','\u{747d9}'];
_11.1.3 = -(-507_i16);
_11.2 = [65281578220012125480356415917588568671_u128];
_17.0 = [_9,_9,_9,_9];
_7.1 = (_11.2,);
_4 = [18283_u16,63939_u16,4871_u16,37240_u16,29110_u16,47945_u16,64706_u16,20685_u16];
_6 = ['\u{b4492}','\u{bfd4c}','\u{85391}','\u{835f}','\u{10dbb0}','\u{737e3}','\u{83792}'];
Goto(bb18)
}
bb18 = {
Call(_18 = dump_var(4_usize, 12_usize, Move(_12), 5_usize, Move(_5), 6_usize, Move(_6), 10_usize, Move(_10)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_18 = dump_var(4_usize, 2_usize, Move(_2), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5() -> i16 {
mir! {
type RET = i16;
let _1: (f32, &'static (Adt24, *const *const isize, u16, *const u8));
let _2: ([u64; 4], ([u128; 1],), *mut &'static u32, &'static Adt34);
let _3: f32;
let _4: Adt50;
let _5: i16;
let _6: [i16; 5];
let _7: Adt72;
let _8: f32;
let _9: isize;
let _10: f64;
let _11: f32;
let _12: *const *const isize;
let _13: f64;
let _14: bool;
let _15: f32;
let _16: ();
let _17: ();
{
RET = -22804_i16;
RET = (-13992_i16);
RET = 19345_i16 | (-29991_i16);
RET = 31001_i16 << (-82_i8);
RET = (-4859_i16);
RET = true as i16;
RET = !14832_i16;
RET = 5541628774356980948_i64 as i16;
_1.0 = 3430126761_u32 as f32;
RET = (-6452_i16) - (-17394_i16);
RET = (-1277_i16);
_1.0 = 5318942923177245191_u64 as f32;
_1.0 = 5161374298804932173_u64 as f32;
_1.0 = 105_u8 as f32;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
340282366920938463463374607431768210179 => bb6,
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
RET = 20875_i16 * (-13963_i16);
RET = -(-9778_i16);
RET = 12695_i16 >> (-655711860_i32);
RET = 52041429450690103523721502332243762373_u128 as i16;
RET = (-25322_i16) + 27528_i16;
_1.0 = 274831449351734181133433571004327584007_u128 as f32;
RET = 19853_i16 + 20658_i16;
_1.0 = 2484435158322875698_i64 as f32;
_2.0 = [1715027941644536562_u64,11793854472846273421_u64,12406090600594572614_u64,12347458649346423906_u64];
_1.0 = (-9223372036854775808_isize) as f32;
RET = -2197_i16;
_2.1.0 = [191347318128473983468240072709713399286_u128];
_3 = -_1.0;
_2.0 = [14497055229099525120_u64,4007264909335582307_u64,16391208601926073874_u64,2572490110259574177_u64];
Goto(bb7)
}
bb7 = {
_3 = -_1.0;
_1.0 = _3 + _3;
_3 = 75314012_u32 as f32;
RET = 21214_i16;
_1.0 = (-9223372036854775808_isize) as f32;
RET = (-23177_i16) * 26693_i16;
_3 = _1.0;
_1.0 = _3;
_2.0 = [6652556943798685455_u64,18382567225112636764_u64,7929372281823393772_u64,11505860890560483434_u64];
_1.0 = _3;
_2.0 = [6114075452185811330_u64,16101032743759239423_u64,13087839133218680240_u64,12958228018315098080_u64];
_2.0 = [15951490673479910313_u64,5393707659804767832_u64,12098514076342275420_u64,14415776678504949326_u64];
RET = (-28298_i16) << 41_isize;
RET = (-16889_i16) << 9223372036854775807_isize;
Call(_2.1 = fn6(_1.0, _3, _1.0, _2.0, _3, RET), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_5 = RET;
_5 = RET + RET;
_5 = RET;
_2.0 = [7798508464781700940_u64,6893742371307387167_u64,17281337331268066076_u64,10099155361521796750_u64];
_2.0 = [11676778051094624556_u64,4438464912099543982_u64,5029578331119873011_u64,3962170149454707951_u64];
_2.1.0 = [300472419838900966970942730779438107031_u128];
_5 = RET >> RET;
RET = _5;
Goto(bb9)
}
bb9 = {
RET = 56135_u16 as i16;
RET = !_5;
_6 = [_5,RET,RET,RET,_5];
_2.0 = [7110064835770315045_u64,143957078107227818_u64,10126226914079346665_u64,3678695716349675914_u64];
_2.1.0 = [228614873328613946982449801880136689962_u128];
RET = _5 & _5;
_2.0 = [11002011231576654877_u64,3916205949287752546_u64,11342467413802623344_u64,5156418301037102209_u64];
_8 = -_1.0;
_7.fld0 = [1138130148_i32,1324964056_i32,1639242006_i32];
_1.0 = -_3;
_11 = -_8;
_7.fld0 = [(-1015932903_i32),(-792374670_i32),1268748479_i32];
_9 = 9223372036854775807_isize;
_5 = -RET;
_1.0 = _3 + _11;
_2.1.0 = [180418259802370883365449547707637416091_u128];
RET = !_5;
_7.fld0 = [266197902_i32,(-625527223_i32),(-1105745207_i32)];
_2.1.0 = [4079897839627729131135451583220616472_u128];
_13 = 222_u8 as f64;
match _9 {
0 => bb2,
9223372036854775807 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_10 = _13;
_5 = RET << _9;
_11 = _1.0 - _3;
_7.fld0 = [1172724659_i32,(-1059601052_i32),1322147438_i32];
_14 = !false;
RET = _5;
_11 = -_3;
RET = (-3420761924939527288_i64) as i16;
_7.fld0 = [1101635436_i32,(-979629970_i32),2002565206_i32];
_3 = _8 + _8;
_8 = _1.0 - _11;
_15 = _8;
Goto(bb12)
}
bb12 = {
_2.0 = [14446485552066014978_u64,7476319678783694599_u64,1088705978698667126_u64,11550900437808953380_u64];
RET = _5;
match _9 {
9223372036854775807 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_1.0 = _15 - _15;
_11 = _13 as f32;
Goto(bb15)
}
bb15 = {
Call(_16 = dump_var(5_usize, 6_usize, Move(_6), 9_usize, Move(_9), 17_usize, _17, 17_usize, _17), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: f32,mut _2: f32,mut _3: f32,mut _4: [u64; 4],mut _5: f32,mut _6: i16) -> ([u128; 1],) {
mir! {
type RET = ([u128; 1],);
let _7: u128;
let _8: &'static f32;
let _9: &'static f32;
let _10: isize;
let _11: bool;
let _12: *const *const isize;
let _13: i8;
let _14: char;
let _15: f32;
let _16: u16;
let _17: [u32; 5];
let _18: char;
let _19: f32;
let _20: u32;
let _21: [i8; 7];
let _22: i16;
let _23: Adt24;
let _24: char;
let _25: bool;
let _26: ([bool; 4], [usize; 8]);
let _27: i32;
let _28: isize;
let _29: u64;
let _30: [i16; 1];
let _31: char;
let _32: ([i32; 3],);
let _33: Adt71;
let _34: [isize; 6];
let _35: i64;
let _36: char;
let _37: f32;
let _38: i32;
let _39: *const ([u128; 1],);
let _40: i64;
let _41: ();
let _42: ();
{
_5 = 2520860729_u32 as f32;
_2 = 103_isize as f32;
_1 = _3 * _2;
_4 = [6732854755148457720_u64,7709757965329783238_u64,15095226561468611702_u64,4030143198649617007_u64];
_5 = _1 * _1;
Call(RET = fn7(_5, _2, _5, _6, _1, _4, _4, _5, _4, _6, _1, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _5;
RET.0 = [190723259613382784960784428760257975043_u128];
_3 = _1 - _1;
RET.0 = [113557277192986450502447905766255209047_u128];
_4 = [18087218429031361157_u64,2374132870082816591_u64,3316350438765366682_u64,14657032125434084276_u64];
_7 = !203874472789040067634520704688234204199_u128;
_2 = -_5;
_7 = !254563328613717162402425711647744488081_u128;
_2 = -_3;
_4 = [410745714379750655_u64,16271697412779411366_u64,12724906160285659484_u64,17808190217375101875_u64];
_7 = !156681140518869088767036962969336888670_u128;
_9 = &_2;
_4 = [15790575882755477097_u64,10501854815968215676_u64,15602537059912551466_u64,9461430887462105075_u64];
_3 = -(*_9);
_2 = _3;
_3 = -_2;
_5 = _1 * _3;
_2 = _5 * _3;
_4 = [10059313956663399727_u64,14874552470231855787_u64,18442703745963244439_u64,14726047978518421193_u64];
_8 = &_5;
_11 = true;
_11 = true ^ false;
_3 = -_2;
_10 = (-87_isize) + 9223372036854775807_isize;
_11 = !true;
Goto(bb2)
}
bb2 = {
_3 = 332638894_u32 as f32;
_1 = _2 + (*_8);
_1 = _5 - _5;
_6 = !(-2110_i16);
_1 = -_2;
_2 = -(*_8);
_2 = _1;
_6 = 190_u8 as i16;
_9 = Move(_8);
_5 = _2;
_9 = &_3;
_6 = !(-2652_i16);
_14 = '\u{b7874}';
_5 = _1 * _1;
_6 = -(-14546_i16);
_8 = &(*_9);
_3 = _2 * _5;
Goto(bb3)
}
bb3 = {
_13 = 30881_u16 as i8;
_4 = [2435646348108015881_u64,5107815969940525057_u64,17914080200938194005_u64,8819400606830908443_u64];
_4 = [15115913119545357592_u64,15260500695260598520_u64,5240780858255637710_u64,12911178948143783600_u64];
RET.0 = [_7];
_4 = [16923417388855057475_u64,18306590488887206677_u64,5558609613350408351_u64,7677303492529622153_u64];
_9 = &_5;
_15 = _2 - (*_9);
_7 = !300405937177457236373868888614507400070_u128;
_7 = 275863167690088595439129884971871212467_u128;
_16 = 53173_u16;
_11 = !false;
_13 = -57_i8;
_15 = -_1;
_16 = 57103_u16;
Goto(bb4)
}
bb4 = {
_16 = !62062_u16;
_16 = 9095_u16 + 22915_u16;
_14 = '\u{432ca}';
_5 = -_3;
_8 = &_2;
_17 = [1554415650_u32,3574920373_u32,2190730585_u32,4142238339_u32,2914762913_u32];
_14 = '\u{5119c}';
_1 = _2 + _5;
_15 = _5 * _1;
_10 = !(-9223372036854775808_isize);
_5 = _3;
_15 = (-571309903822666962_i64) as f32;
_4 = [4075735290108274260_u64,8071205516845647884_u64,7175841697712095447_u64,10255166599040300351_u64];
_13 = 11_i8;
_3 = _1 * _5;
RET.0 = [_7];
_16 = !27400_u16;
_3 = _1 * _2;
RET.0 = [_7];
_7 = _13 as u128;
_18 = _14;
_17 = [2024259283_u32,1155090161_u32,3925980863_u32,2323252932_u32,2034985523_u32];
_4 = [14570742490373909181_u64,14125343495447923745_u64,6059330802072939627_u64,11596329421795857838_u64];
_16 = !64868_u16;
match _13 {
0 => bb1,
1 => bb2,
11 => bb5,
_ => bb3
}
}
bb5 = {
RET.0 = [_7];
_9 = &_5;
_7 = 88579032116485354999858744895032515471_i128 as u128;
_2 = _13 as f32;
_10 = (-9223372036854775808_isize);
_19 = (*_9) + _1;
_8 = &_1;
_2 = _19 + _19;
_21 = [_13,_13,_13,_13,_13,_13,_13];
_11 = _3 != _5;
_16 = 46557_u16;
_1 = _3 * (*_9);
_18 = _14;
_19 = _1;
_14 = _18;
_8 = Move(_9);
_24 = _18;
_24 = _18;
Goto(bb6)
}
bb6 = {
_14 = _18;
RET.0 = [_7];
_9 = &_5;
_21 = [_13,_13,_13,_13,_13,_13,_13];
_8 = Move(_9);
_18 = _14;
_16 = 29027_u16 << _13;
_23 = Adt24::Variant0 { fld0: 731936938_i32 };
_20 = 3933916557_u32 >> _10;
RET.0 = [_7];
_9 = &_2;
_6 = _13 as i16;
_26.0 = [_11,_11,_11,_11];
_13 = !(-9_i8);
_22 = !_6;
_5 = _15 * _3;
_24 = _14;
RET.0 = [_7];
_26.0 = [_11,_11,_11,_11];
_27 = (*_9) as i32;
_21 = [_13,_13,_13,_13,_13,_13,_13];
RET.0 = [_7];
_21 = [_13,_13,_13,_13,_13,_13,_13];
_11 = true;
_23 = Adt24::Variant0 { fld0: _27 };
Goto(bb7)
}
bb7 = {
RET.0 = [_7];
_28 = _10;
_2 = -_19;
_28 = _10;
_4 = [4508704064265359221_u64,13044831939419540970_u64,12897505247841803518_u64,7465510345075960599_u64];
_11 = false;
SetDiscriminant(_23, 1);
_26.1 = [2_usize,4_usize,0_usize,4_usize,7_usize,7671768970187338266_usize,6723831701355045456_usize,0_usize];
place!(Field::<u32>(Variant(_23, 1), 3)) = _20;
_24 = _14;
match _10 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb8,
340282366920938463454151235394913435648 => bb10,
_ => bb9
}
}
bb8 = {
_1 = _5;
RET.0 = [190723259613382784960784428760257975043_u128];
_3 = _1 - _1;
RET.0 = [113557277192986450502447905766255209047_u128];
_4 = [18087218429031361157_u64,2374132870082816591_u64,3316350438765366682_u64,14657032125434084276_u64];
_7 = !203874472789040067634520704688234204199_u128;
_2 = -_5;
_7 = !254563328613717162402425711647744488081_u128;
_2 = -_3;
_4 = [410745714379750655_u64,16271697412779411366_u64,12724906160285659484_u64,17808190217375101875_u64];
_7 = !156681140518869088767036962969336888670_u128;
_9 = &_2;
_4 = [15790575882755477097_u64,10501854815968215676_u64,15602537059912551466_u64,9461430887462105075_u64];
_3 = -(*_9);
_2 = _3;
_3 = -_2;
_5 = _1 * _3;
_2 = _5 * _3;
_4 = [10059313956663399727_u64,14874552470231855787_u64,18442703745963244439_u64,14726047978518421193_u64];
_8 = &_5;
_11 = true;
_11 = true ^ false;
_3 = -_2;
_10 = (-87_isize) + 9223372036854775807_isize;
_11 = !true;
Goto(bb2)
}
bb9 = {
_13 = 30881_u16 as i8;
_4 = [2435646348108015881_u64,5107815969940525057_u64,17914080200938194005_u64,8819400606830908443_u64];
_4 = [15115913119545357592_u64,15260500695260598520_u64,5240780858255637710_u64,12911178948143783600_u64];
RET.0 = [_7];
_4 = [16923417388855057475_u64,18306590488887206677_u64,5558609613350408351_u64,7677303492529622153_u64];
_9 = &_5;
_15 = _2 - (*_9);
_7 = !300405937177457236373868888614507400070_u128;
_7 = 275863167690088595439129884971871212467_u128;
_16 = 53173_u16;
_11 = !false;
_13 = -57_i8;
_15 = -_1;
_16 = 57103_u16;
Goto(bb4)
}
bb10 = {
place!(Field::<u128>(Variant(_23, 1), 6)) = _7;
place!(Field::<u64>(Variant(_23, 1), 0)) = !313964962600632359_u64;
_5 = _3;
_26.0 = [_11,_11,_11,_11];
_31 = _18;
_23 = Adt24::Variant0 { fld0: _27 };
_35 = -(-378507540091236160_i64);
_6 = _7 as i16;
_29 = 8393251776419779131_u64 >> _27;
_19 = -_2;
_3 = _2;
_25 = !_11;
_37 = -_2;
Goto(bb11)
}
bb11 = {
_11 = _25;
_18 = _14;
_17 = [_20,_20,_20,_20,_20];
_7 = _11 as u128;
_16 = 48541_u16 >> Field::<i32>(Variant(_23, 0), 0);
place!(Field::<i32>(Variant(_23, 0), 0)) = _27;
_32.0 = [Field::<i32>(Variant(_23, 0), 0),Field::<i32>(Variant(_23, 0), 0),_27];
_36 = _14;
_34 = [_28,_10,_10,_10,_28,_10];
_28 = -_10;
_24 = _14;
SetDiscriminant(_23, 3);
_8 = &_15;
place!(Field::<i32>(Variant(_23, 3), 5)) = _27;
_16 = 38300_u16;
_23 = Adt24::Variant0 { fld0: _27 };
_3 = _2;
_20 = !3418716729_u32;
_26.0 = [_25,_11,_25,_11];
_21 = [_13,_13,_13,_13,_13,_13,_13];
_5 = _22 as f32;
_26.1 = [13406710345834407360_usize,1_usize,6_usize,13468577287359384343_usize,1145501448233779442_usize,6_usize,14333691517554442811_usize,3574809014690515020_usize];
_29 = _24 as u64;
_5 = _2 * _1;
_28 = !_10;
RET.0 = [_7];
_27 = _35 as i32;
RET.0 = [_7];
match _16 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb10,
4 => bb12,
38300 => bb14,
_ => bb13
}
}
bb12 = {
_1 = _5;
RET.0 = [190723259613382784960784428760257975043_u128];
_3 = _1 - _1;
RET.0 = [113557277192986450502447905766255209047_u128];
_4 = [18087218429031361157_u64,2374132870082816591_u64,3316350438765366682_u64,14657032125434084276_u64];
_7 = !203874472789040067634520704688234204199_u128;
_2 = -_5;
_7 = !254563328613717162402425711647744488081_u128;
_2 = -_3;
_4 = [410745714379750655_u64,16271697412779411366_u64,12724906160285659484_u64,17808190217375101875_u64];
_7 = !156681140518869088767036962969336888670_u128;
_9 = &_2;
_4 = [15790575882755477097_u64,10501854815968215676_u64,15602537059912551466_u64,9461430887462105075_u64];
_3 = -(*_9);
_2 = _3;
_3 = -_2;
_5 = _1 * _3;
_2 = _5 * _3;
_4 = [10059313956663399727_u64,14874552470231855787_u64,18442703745963244439_u64,14726047978518421193_u64];
_8 = &_5;
_11 = true;
_11 = true ^ false;
_3 = -_2;
_10 = (-87_isize) + 9223372036854775807_isize;
_11 = !true;
Goto(bb2)
}
bb13 = {
_14 = _18;
RET.0 = [_7];
_9 = &_5;
_21 = [_13,_13,_13,_13,_13,_13,_13];
_8 = Move(_9);
_18 = _14;
_16 = 29027_u16 << _13;
_23 = Adt24::Variant0 { fld0: 731936938_i32 };
_20 = 3933916557_u32 >> _10;
RET.0 = [_7];
_9 = &_2;
_6 = _13 as i16;
_26.0 = [_11,_11,_11,_11];
_13 = !(-9_i8);
_22 = !_6;
_5 = _15 * _3;
_24 = _14;
RET.0 = [_7];
_26.0 = [_11,_11,_11,_11];
_27 = (*_9) as i32;
_21 = [_13,_13,_13,_13,_13,_13,_13];
RET.0 = [_7];
_21 = [_13,_13,_13,_13,_13,_13,_13];
_11 = true;
_23 = Adt24::Variant0 { fld0: _27 };
Goto(bb7)
}
bb14 = {
_15 = _19;
_1 = _19;
_27 = !Field::<i32>(Variant(_23, 0), 0);
_30 = [_22];
_5 = 73_u8 as f32;
_15 = -_3;
SetDiscriminant(_23, 2);
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(6_usize, 27_usize, Move(_27), 28_usize, Move(_28), 22_usize, Move(_22), 35_usize, Move(_35)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(6_usize, 26_usize, Move(_26), 32_usize, Move(_32), 16_usize, Move(_16), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(6_usize, 11_usize, Move(_11), 21_usize, Move(_21), 6_usize, Move(_6), 34_usize, Move(_34)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: f32,mut _2: f32,mut _3: f32,mut _4: i16,mut _5: f32,mut _6: [u64; 4],mut _7: [u64; 4],mut _8: f32,mut _9: [u64; 4],mut _10: i16,mut _11: f32,mut _12: f32) -> ([u128; 1],) {
mir! {
type RET = ([u128; 1],);
let _13: isize;
let _14: *const [u64; 4];
let _15: [u16; 4];
let _16: f64;
let _17: [u8; 4];
let _18: [u64; 4];
let _19: [u16; 4];
let _20: *const ([u128; 1],);
let _21: [u128; 1];
let _22: char;
let _23: [usize; 8];
let _24: Adt66;
let _25: *mut *const u8;
let _26: [char; 7];
let _27: u64;
let _28: f32;
let _29: isize;
let _30: isize;
let _31: char;
let _32: isize;
let _33: u128;
let _34: usize;
let _35: [i32; 1];
let _36: Adt63;
let _37: isize;
let _38: isize;
let _39: [u8; 4];
let _40: bool;
let _41: u32;
let _42: [char; 7];
let _43: [usize; 8];
let _44: i64;
let _45: i64;
let _46: (Adt50, *const isize, &'static u8, *mut *const u8);
let _47: f64;
let _48: f32;
let _49: ();
let _50: ();
{
RET.0 = [305063765891101045225815610209108639952_u128];
_7 = [8294400196579445207_u64,8814021767885085156_u64,16597023647430440991_u64,4172574117009704246_u64];
_8 = _12 + _1;
RET.0 = [48507327250193978462873523762511253047_u128];
_9 = [14609937242929672913_u64,395810217589208248_u64,13767870187681672130_u64,4033819810359862128_u64];
_4 = _10;
_3 = -_1;
_12 = 87_isize as f32;
_4 = _10;
_2 = _1;
_12 = -_8;
RET.0 = [156949318305311443416600548874009667125_u128];
_12 = 127_isize as f32;
_9 = _6;
_13 = !9223372036854775807_isize;
_11 = _5;
_10 = !_4;
_8 = _5 + _1;
_8 = 296959266967854568017946886250065900925_u128 as f32;
_10 = _13 as i16;
_16 = 196937905769302596128666742623321692005_u128 as f64;
_9 = [6605965896449701907_u64,11369152591276518829_u64,15112055285326550735_u64,15504478360952080622_u64];
_12 = _2;
_7 = [8552946505015942034_u64,4453435788618758287_u64,8177206700341613819_u64,2496936799135209123_u64];
_2 = _13 as f32;
_16 = 7104815162865720093_u64 as f64;
_1 = _3 - _3;
_3 = (-6202703069695401055_i64) as f32;
Goto(bb1)
}
bb1 = {
_5 = 7678783450724658820_u64 as f32;
_6 = [527774932424151743_u64,2154461056175542993_u64,9332217635378989010_u64,11613028577125860344_u64];
_2 = _11;
_2 = _11;
_9 = [14720785502565878759_u64,10528405772066170325_u64,13139908472343244581_u64,15541158851772903566_u64];
RET.0 = [222673281822395068964331585549734992713_u128];
_6 = [2217795428514225430_u64,153313659672938611_u64,449725370515118983_u64,2800479762772035286_u64];
_6 = _9;
_19 = [33197_u16,46380_u16,36903_u16,18081_u16];
_4 = _10 + _10;
_9 = [6663181492722272810_u64,14231027763587806271_u64,15810104391499843847_u64,2293028881691020628_u64];
_6 = [17498357966731769755_u64,16334632409374521971_u64,414780248324081361_u64,1837808644501361871_u64];
_11 = -_1;
_14 = core::ptr::addr_of!(_6);
_9 = [4899353136878724762_u64,14403896392723557236_u64,10643464217127640543_u64,5585058201357013929_u64];
_1 = _11;
_4 = 1245811219_i32 as i16;
_20 = core::ptr::addr_of!(RET);
Goto(bb2)
}
bb2 = {
_18 = [10036386755352914547_u64,10262697411408947473_u64,7024155054619957770_u64,5453878641885565359_u64];
Call(_5 = fn8(_6, Move(_14), _16, _11, _9, _12, (*_14)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*_20).0 = [188067241331235838943673256159875479337_u128];
_22 = '\u{42917}';
_18 = [5296785596031002857_u64,12560978037722728331_u64,15257049511098993072_u64,4879484188245522575_u64];
RET.0 = [84152208890835937297139028269420551914_u128];
_9 = [2898293518660248778_u64,13197340321348485260_u64,392119489566454266_u64,6635459610940550911_u64];
_18 = _6;
_14 = core::ptr::addr_of!(_6);
(*_14) = [12084469194796233775_u64,12308721699621235638_u64,969693004861332086_u64,7360102510641935213_u64];
_5 = _8 - _11;
_18 = [13988249211178124839_u64,8223978295245657482_u64,5133896621367099407_u64,16783810283296448730_u64];
(*_20).0 = [318757026374993026246193591415297340119_u128];
_11 = _13 as f32;
_21 = [261477144022094967628933416708138803102_u128];
(*_14) = [3436613540262124744_u64,2821401150909870088_u64,9061337424303266752_u64,15804896907789057493_u64];
_11 = -_12;
(*_20).0 = _21;
(*_14) = _9;
_13 = 9223372036854775807_isize;
Goto(bb4)
}
bb4 = {
(*_20).0 = _21;
_7 = [9877419232988846125_u64,6094146866635031709_u64,14431334148070254486_u64,15117029384852279950_u64];
(*_14) = [13692339762037860988_u64,694004929579732027_u64,7121774158451982724_u64,14277431692395221417_u64];
_19 = [18506_u16,51006_u16,51600_u16,52444_u16];
_23 = [10224167141712981651_usize,4_usize,7_usize,9805766324777331533_usize,15345574238536744690_usize,1474781916021898783_usize,5_usize,8066896041451426246_usize];
_6 = [5981377273353735559_u64,10255424615774069620_u64,17272565612070000433_u64,15988983566193985027_u64];
(*_14) = [11688369294420578650_u64,9380340225296517502_u64,7286149179958126502_u64,14027427146026419712_u64];
_3 = 320913815_u32 as f32;
_18 = (*_14);
_3 = _12;
RET.0 = [219099207852943668813689290380665833976_u128];
Call(_25 = fn9(Move(_14), _1, _9, _12, _18, _23, _23, _5, _13), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_20 = core::ptr::addr_of!((*_20));
_17 = [156_u8,165_u8,203_u8,115_u8];
(*_20).0 = _21;
_5 = 301034228946866143223588678630758074374_u128 as f32;
(*_20).0 = [6129009436064474761587657922962887601_u128];
RET.0 = [217057684808573356455914757372982860671_u128];
_16 = 294606426498904260953165556067972806128_u128 as f64;
_17 = [198_u8,89_u8,83_u8,38_u8];
_3 = -_11;
_27 = 7136823763107810741_u64;
_10 = _4;
_5 = 238402070257958495370991940767132193710_u128 as f32;
_22 = '\u{319e7}';
(*_20) = (_21,);
_15 = [20783_u16,63687_u16,35922_u16,41572_u16];
_6 = [_27,_27,_27,_27];
RET.0 = [315715657139480804330175885576567008478_u128];
_27 = 2511092389936172907_u64;
RET.0 = _21;
_3 = 59844893903678135377777619928391483202_u128 as f32;
_14 = core::ptr::addr_of!(_7);
(*_20) = (_21,);
_13 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_16 = 65408110224938279418660978388678149148_u128 as f64;
_28 = _16 as f32;
_13 = -9223372036854775807_isize;
Goto(bb6)
}
bb6 = {
_16 = (-46_i8) as f64;
_11 = _2;
_32 = _13;
_11 = _1;
_31 = _22;
_17 = [219_u8,145_u8,108_u8,221_u8];
_23 = [1_usize,14701753486766966960_usize,9924187884323177380_usize,1208799385589878526_usize,1_usize,12481519957689510083_usize,9001030025472019505_usize,16895662219657042064_usize];
_16 = 2529_u16 as f64;
(*_20) = (_21,);
_9 = [_27,_27,_27,_27];
RET = (_21,);
RET = (_21,);
_23 = [0_usize,3_usize,1_usize,14283627498704545784_usize,2_usize,17426153692832160176_usize,3_usize,6977822798461909328_usize];
(*_20) = (_21,);
_8 = _12 - _11;
_33 = 13_i8 as u128;
_27 = 10005378246835656366_u64;
_20 = core::ptr::addr_of!((*_20));
_22 = _31;
Goto(bb7)
}
bb7 = {
(*_20) = (_21,);
_26 = [_31,_22,_31,_22,_22,_22,_31];
(*_20).0 = _21;
_19 = [9741_u16,18373_u16,55524_u16,40537_u16];
_8 = _1 - _5;
_4 = 9068_u16 as i16;
_12 = _2;
_10 = !_4;
_18 = [_27,_27,_27,_27];
RET.0 = _21;
Goto(bb8)
}
bb8 = {
RET.0 = _21;
_8 = _1;
_20 = core::ptr::addr_of!((*_20));
_36 = Adt63::Variant2 { fld0: _16,fld1: _22,fld2: _23,fld3: Move(_20) };
_30 = 516574867016446810_usize as isize;
_34 = 13_i8 as usize;
_2 = -_1;
_17 = [133_u8,173_u8,227_u8,242_u8];
_1 = _8 + _11;
_37 = _34 as isize;
_4 = _10;
_5 = _33 as f32;
_9 = (*_14);
place!(Field::<f64>(Variant(_36, 2), 0)) = -_16;
_37 = 66_u8 as isize;
RET = (_21,);
_15 = [176_u16,19811_u16,44664_u16,9614_u16];
_19 = _15;
_9 = [_27,_27,_27,_27];
_35 = [1962774004_i32];
match _27 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb7,
4 => bb5,
5 => bb6,
10005378246835656366 => bb10,
_ => bb9
}
}
bb9 = {
(*_20).0 = _21;
_7 = [9877419232988846125_u64,6094146866635031709_u64,14431334148070254486_u64,15117029384852279950_u64];
(*_14) = [13692339762037860988_u64,694004929579732027_u64,7121774158451982724_u64,14277431692395221417_u64];
_19 = [18506_u16,51006_u16,51600_u16,52444_u16];
_23 = [10224167141712981651_usize,4_usize,7_usize,9805766324777331533_usize,15345574238536744690_usize,1474781916021898783_usize,5_usize,8066896041451426246_usize];
_6 = [5981377273353735559_u64,10255424615774069620_u64,17272565612070000433_u64,15988983566193985027_u64];
(*_14) = [11688369294420578650_u64,9380340225296517502_u64,7286149179958126502_u64,14027427146026419712_u64];
_3 = 320913815_u32 as f32;
_18 = (*_14);
_3 = _12;
RET.0 = [219099207852943668813689290380665833976_u128];
Call(_25 = fn9(Move(_14), _1, _9, _12, _18, _23, _23, _5, _13), ReturnTo(bb5), UnwindUnreachable())
}
bb10 = {
_24 = Adt66::Variant2 { fld0: 131_u8 };
place!(Field::<f64>(Variant(_36, 2), 0)) = _16;
_3 = 27168_u16 as f32;
Goto(bb11)
}
bb11 = {
place!(Field::<u8>(Variant(_24, 2), 0)) = !71_u8;
match _27 {
0 => bb4,
1 => bb5,
10005378246835656366 => bb12,
_ => bb6
}
}
bb12 = {
_1 = -_2;
_40 = !false;
_32 = _13 - _13;
SetDiscriminant(_36, 1);
_10 = -_4;
_26 = [_31,_31,_31,_22,_22,_22,_22];
SetDiscriminant(_24, 2);
_2 = _1 * _12;
_38 = !_13;
_18 = [_27,_27,_27,_27];
_42 = [_22,_22,_31,_22,_22,_22,_31];
place!(Field::<u8>(Variant(_24, 2), 0)) = !38_u8;
(*_14) = [_27,_27,_27,_27];
SetDiscriminant(_24, 3);
_10 = _16 as i16;
Call(_41 = fn17(_27, _4, _35, _10, _11, _11, _16, _1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
place!(Field::<i8>(Variant(_24, 3), 3)) = 55_i8 + 31_i8;
_6 = [_27,_27,_27,_27];
place!(Field::<[u16; 4]>(Variant(_36, 1), 2)) = [13908_u16,55464_u16,44310_u16,17604_u16];
place!(Field::<[u8; 4]>(Variant(_24, 3), 4)) = _17;
_27 = 5239382937913106054_u64 - 8863166082831937372_u64;
_42 = [_22,_22,_31,_31,_22,_22,_22];
place!(Field::<bool>(Variant(_24, 3), 0)) = _40;
_7 = [_27,_27,_27,_27];
_11 = _2;
_22 = _31;
Goto(bb14)
}
bb14 = {
_35 = [1533607750_i32];
_5 = -_12;
_45 = _16 as i64;
_29 = !_30;
place!(Field::<u64>(Variant(_24, 3), 2)) = _11 as u64;
_45 = _11 as i64;
_32 = _37;
_20 = core::ptr::addr_of!(RET);
place!(Field::<([u128; 1],)>(Variant(_24, 3), 6)).0 = [_33];
_18 = [Field::<u64>(Variant(_24, 3), 2),_27,Field::<u64>(Variant(_24, 3), 2),Field::<u64>(Variant(_24, 3), 2)];
Goto(bb15)
}
bb15 = {
Call(_49 = dump_var(7_usize, 23_usize, Move(_23), 42_usize, Move(_42), 7_usize, Move(_7), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_49 = dump_var(7_usize, 33_usize, Move(_33), 45_usize, Move(_45), 10_usize, Move(_10), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = dump_var(7_usize, 38_usize, Move(_38), 21_usize, Move(_21), 19_usize, Move(_19), 41_usize, Move(_41)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_49 = dump_var(7_usize, 15_usize, Move(_15), 34_usize, Move(_34), 50_usize, _50, 50_usize, _50), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [u64; 4],mut _2: *const [u64; 4],mut _3: f64,mut _4: f32,mut _5: [u64; 4],mut _6: f32,mut _7: [u64; 4]) -> f32 {
mir! {
type RET = f32;
let _8: (u8, u128, Adt31, *const *const isize);
let _9: *const *const isize;
let _10: isize;
let _11: (&'static u32,);
let _12: i32;
let _13: f32;
let _14: ();
let _15: ();
{
RET = _6 * _4;
_2 = core::ptr::addr_of!(_1);
_1 = [5228858377577947884_u64,2626374343655294442_u64,13176733087240175189_u64,1777435932873210947_u64];
(*_2) = [5266873358261276900_u64,13090109906138299669_u64,5390356963604121036_u64,6436602134799294747_u64];
_3 = 7_i8 as f64;
_8.2.fld6 = !2196906328051775855_i64;
_8.2.fld2 = [47128_u16,13808_u16,5948_u16,20749_u16,34276_u16,44113_u16,57549_u16,5977_u16];
_8.2.fld2 = [58648_u16,6471_u16,60985_u16,34549_u16,14060_u16,29031_u16,45797_u16,54972_u16];
_8.2.fld2 = [34081_u16,16821_u16,805_u16,34151_u16,26040_u16,43245_u16,42390_u16,51513_u16];
_8.2.fld4 = 14253500736549262047_usize;
_8.2.fld3 = core::ptr::addr_of!(_10);
_8.2.fld1 = '\u{100309}';
_8.1 = 254488687919383487687550939985713519725_u128;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(8_usize, 5_usize, Move(_5), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: *const [u64; 4],mut _2: f32,mut _3: [u64; 4],mut _4: f32,mut _5: [u64; 4],mut _6: [usize; 8],mut _7: [usize; 8],mut _8: f32,mut _9: isize) -> *mut *const u8 {
mir! {
type RET = *mut *const u8;
let _10: [i16; 5];
let _11: isize;
let _12: bool;
let _13: isize;
let _14: i64;
let _15: isize;
let _16: (f32, &'static (Adt24, *const *const isize, u16, *const u8));
let _17: isize;
let _18: i8;
let _19: *mut [u128; 1];
let _20: [i32; 3];
let _21: u32;
let _22: i16;
let _23: u8;
let _24: ();
let _25: ();
{
_5 = [9362125721731908298_u64,7092693959613186428_u64,3453079015454970138_u64,9486539750924708226_u64];
_5 = [3859657625839358465_u64,7012020900952674179_u64,13080877334693917071_u64,1997504953897179141_u64];
_3 = [17087062303931291738_u64,14209945879057640256_u64,9586432072499770764_u64,2622325077414718429_u64];
_7 = [17799435173128402198_usize,3_usize,9184706589722927316_usize,13677061896056113509_usize,0_usize,0_usize,10106320226648940394_usize,7354444051509449966_usize];
_4 = _8 - _8;
_1 = core::ptr::addr_of!(_5);
_8 = 188630448982324103971973775632305324055_u128 as f32;
_1 = core::ptr::addr_of!((*_1));
_7 = [14419913538143196548_usize,14842996219904933606_usize,12014027293483557449_usize,17575515971626129269_usize,1_usize,7_usize,6_usize,1010340765321716018_usize];
_6 = _7;
Call((*_1) = core::intrinsics::transmute(_3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = [5232458680302078950_u64,10302002818611967955_u64,17793856738241290949_u64,641944449792244387_u64];
_5 = [959477529962256509_u64,9347304525552796016_u64,1461786845123909671_u64,4367043025381007066_u64];
_2 = _4 * _4;
_3 = [1419983383323079108_u64,6959702580249457006_u64,4372833130303844931_u64,13362424255804936857_u64];
_8 = -_2;
(*_1) = _3;
_3 = (*_1);
_10 = [13489_i16,16464_i16,28480_i16,(-15308_i16),(-24076_i16)];
Goto(bb2)
}
bb2 = {
_4 = 16149099559461160284049751244304430845_u128 as f32;
Goto(bb3)
}
bb3 = {
(*_1) = _3;
_7 = _6;
_5 = [17564519418139840128_u64,2018063004524229138_u64,310415854151886705_u64,1773492109557287631_u64];
_8 = 252177064973286565135027627884814402514_u128 as f32;
_1 = core::ptr::addr_of!((*_1));
_11 = _9;
_3 = [7097310537134249765_u64,15268822564979587192_u64,4940528094849545193_u64,10961739690669368545_u64];
_11 = _9 + _9;
match _9 {
0 => bb1,
1 => bb2,
2 => bb4,
9223372036854775807 => bb6,
_ => bb5
}
}
bb4 = {
_4 = 16149099559461160284049751244304430845_u128 as f32;
Goto(bb3)
}
bb5 = {
_5 = [5232458680302078950_u64,10302002818611967955_u64,17793856738241290949_u64,641944449792244387_u64];
_5 = [959477529962256509_u64,9347304525552796016_u64,1461786845123909671_u64,4367043025381007066_u64];
_2 = _4 * _4;
_3 = [1419983383323079108_u64,6959702580249457006_u64,4372833130303844931_u64,13362424255804936857_u64];
_8 = -_2;
(*_1) = _3;
_3 = (*_1);
_10 = [13489_i16,16464_i16,28480_i16,(-15308_i16),(-24076_i16)];
Goto(bb2)
}
bb6 = {
_3 = (*_1);
_10 = [15574_i16,(-31260_i16),20592_i16,6498_i16,10060_i16];
_4 = _2;
_5 = [16892821594886073686_u64,3055991615824876329_u64,1784621491184782869_u64,17261888486675113551_u64];
_6 = _7;
_1 = core::ptr::addr_of!((*_1));
_6 = [0_usize,11362579859259032590_usize,6_usize,7_usize,1438225276974183892_usize,11693389271897663522_usize,0_usize,4706475234582824220_usize];
_10 = [(-13579_i16),(-30108_i16),10719_i16,19121_i16,(-25437_i16)];
_10 = [(-5870_i16),25960_i16,(-15232_i16),28938_i16,27531_i16];
_8 = _2;
(*_1) = [11239644425914558458_u64,7183747310150809789_u64,17237686823175057857_u64,17145735381374479027_u64];
Call(RET = fn10(_2, Move(_1), _7, _4, _4, _4, (*_1), _6, _2, _2, _11), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_5 = [4893746583105541802_u64,2386951238526580710_u64,14335661759802703353_u64,14668559875639984516_u64];
_5 = [2747320920401225716_u64,3274409335477918366_u64,10918540644049991806_u64,16220426286287902980_u64];
_2 = _8 * _4;
_10 = [(-7457_i16),7072_i16,6929_i16,23936_i16,6455_i16];
_15 = !_11;
_7 = [1_usize,10313148180405229508_usize,2412130685483689928_usize,16030172988795819268_usize,1_usize,6_usize,9548538984732140243_usize,1_usize];
_12 = _11 <= _11;
_8 = -_4;
_11 = 3153114225_u32 as isize;
_13 = _9 << _9;
_6 = _7;
_7 = [16158807671201761691_usize,10234259434527291068_usize,1796228013737596702_usize,3381244221916633534_usize,15860305577820858327_usize,15390231825690393993_usize,16170658628363209689_usize,4_usize];
_2 = -_4;
_10 = [700_i16,(-28177_i16),30080_i16,(-24356_i16),(-7831_i16)];
_15 = _9 << _9;
_5 = [8185459148669545498_u64,7877306906284574712_u64,5731461763309699013_u64,13652552211485753663_u64];
_14 = 4190176950263030236_i64;
_13 = _9 | _11;
_16.0 = -_4;
Goto(bb8)
}
bb8 = {
_14 = (-27413_i16) as i64;
_5 = [8450566334124013943_u64,7447123034129115646_u64,7389124243713083282_u64,1985948278710686853_u64];
_3 = _5;
_13 = _11 | _15;
_8 = _14 as f32;
_14 = 2077663994620983146_i64;
_14 = 1197004517753148462_i64;
_1 = core::ptr::addr_of!(_3);
_13 = 55877_u16 as isize;
_14 = -5524704888148427236_i64;
_12 = false;
(*_1) = [5260463755047931169_u64,5108905891021183324_u64,13709489318726439470_u64,9114373181960259444_u64];
_4 = _2 + _16.0;
_7 = [5_usize,1_usize,10372274188807747831_usize,0_usize,7210564416318825121_usize,717624510875030935_usize,1_usize,5_usize];
_4 = -_2;
_13 = 1987354307_i32 as isize;
_4 = 43_u8 as f32;
_9 = _11;
_17 = _9 * _11;
(*_1) = _5;
_15 = _11;
_1 = core::ptr::addr_of!(_3);
_4 = _16.0 - _16.0;
_15 = _9;
Goto(bb9)
}
bb9 = {
_12 = !true;
_16.0 = _4 - _4;
_13 = _15 * _15;
Goto(bb10)
}
bb10 = {
_12 = false;
_8 = 168790826494700514896915784583572670737_u128 as f32;
_1 = core::ptr::addr_of!((*_1));
_1 = core::ptr::addr_of!((*_1));
_10 = [9398_i16,(-6598_i16),(-4519_i16),5951_i16,(-17143_i16)];
_12 = true;
_8 = -_16.0;
_5 = [55414571041147860_u64,4699415882800947855_u64,6668992154564612790_u64,15577438284031832112_u64];
_6 = [9081613346477545753_usize,5_usize,249208545778561313_usize,16376331864939332480_usize,15642007017874825561_usize,2_usize,5287617629687283946_usize,4203493477343561105_usize];
_13 = _15;
_11 = '\u{446f3}' as isize;
_5 = [15403804427687963010_u64,14465356502072317260_u64,5205105971011531970_u64,15436153112946965277_u64];
_3 = [11359385466615382618_u64,10105129529074751899_u64,6210172245312052238_u64,12392991647084636202_u64];
(*_1) = [7434717251792327657_u64,18221024293480969666_u64,16123578705395588815_u64,14768400480761781460_u64];
_4 = -_16.0;
_16.0 = _8 * _8;
_18 = 28_i8;
_14 = -2437051591455976883_i64;
_6 = [14869302197322120256_usize,7_usize,3647924794286455506_usize,1_usize,18262969785839742518_usize,12212491365675989412_usize,11941826514219964698_usize,4_usize];
_16.0 = _4 + _4;
_1 = core::ptr::addr_of!((*_1));
(*_1) = [7304291173734120844_u64,17996403692556161858_u64,15013300851703983327_u64,4126385672301277859_u64];
_8 = _14 as f32;
match _18 {
28 => bb11,
_ => bb7
}
}
bb11 = {
_7 = [0_usize,1_usize,6_usize,2_usize,47215422483624863_usize,8292487281308183018_usize,7920068889828393345_usize,12806891963918229611_usize];
_8 = _16.0 + _4;
_8 = -_16.0;
_22 = (-16823_i16) - 15575_i16;
_11 = 999741476_i32 as isize;
_3 = _5;
_9 = _13;
_21 = '\u{3212f}' as u32;
_16.0 = _4 + _2;
_9 = _17 - _11;
Goto(bb12)
}
bb12 = {
(*_1) = _5;
match _18 {
0 => bb3,
1 => bb13,
28 => bb15,
_ => bb14
}
}
bb13 = {
_14 = (-27413_i16) as i64;
_5 = [8450566334124013943_u64,7447123034129115646_u64,7389124243713083282_u64,1985948278710686853_u64];
_3 = _5;
_13 = _11 | _15;
_8 = _14 as f32;
_14 = 2077663994620983146_i64;
_14 = 1197004517753148462_i64;
_1 = core::ptr::addr_of!(_3);
_13 = 55877_u16 as isize;
_14 = -5524704888148427236_i64;
_12 = false;
(*_1) = [5260463755047931169_u64,5108905891021183324_u64,13709489318726439470_u64,9114373181960259444_u64];
_4 = _2 + _16.0;
_7 = [5_usize,1_usize,10372274188807747831_usize,0_usize,7210564416318825121_usize,717624510875030935_usize,1_usize,5_usize];
_4 = -_2;
_13 = 1987354307_i32 as isize;
_4 = 43_u8 as f32;
_9 = _11;
_17 = _9 * _11;
(*_1) = _5;
_15 = _11;
_1 = core::ptr::addr_of!(_3);
_4 = _16.0 - _16.0;
_15 = _9;
Goto(bb9)
}
bb14 = {
_4 = 16149099559461160284049751244304430845_u128 as f32;
Goto(bb3)
}
bb15 = {
(*_1) = [6618647118887827493_u64,5177534855397787448_u64,16148759594873003811_u64,14831082300938542657_u64];
_1 = core::ptr::addr_of!((*_1));
_11 = _17 << _9;
Goto(bb16)
}
bb16 = {
Call(_24 = dump_var(9_usize, 7_usize, Move(_7), 18_usize, Move(_18), 21_usize, Move(_21), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_24 = dump_var(9_usize, 14_usize, Move(_14), 22_usize, Move(_22), 5_usize, Move(_5), 25_usize, _25), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: f32,mut _2: *const [u64; 4],mut _3: [usize; 8],mut _4: f32,mut _5: f32,mut _6: f32,mut _7: [u64; 4],mut _8: [usize; 8],mut _9: f32,mut _10: f32,mut _11: isize) -> *mut *const u8 {
mir! {
type RET = *mut *const u8;
let _12: &'static u8;
let _13: [u32; 5];
let _14: bool;
let _15: u64;
let _16: *mut char;
let _17: Adt63;
let _18: &'static *const i128;
let _19: [char; 7];
let _20: [i16; 1];
let _21: *mut &'static u32;
let _22: *mut &'static u32;
let _23: isize;
let _24: (Adt24, *const *const isize, u16, *const u8);
let _25: &'static u8;
let _26: i128;
let _27: isize;
let _28: *const [u64; 4];
let _29: i32;
let _30: u16;
let _31: isize;
let _32: ();
let _33: ();
{
_6 = 4654899652322546069_u64 as f32;
Goto(bb1)
}
bb1 = {
_3 = [12689125411816954589_usize,1_usize,7_usize,3084645682177756439_usize,8697250236462034337_usize,2523781657856511500_usize,3_usize,5_usize];
_3 = [2_usize,6_usize,15062564340690010263_usize,3_usize,15310630864908244017_usize,2289690783726147464_usize,4_usize,1_usize];
_8 = [5_usize,0_usize,15124179920090992270_usize,4_usize,2_usize,11803722386070002062_usize,16193909035774656952_usize,7634666508126212184_usize];
_7 = [4753298284292365861_u64,7068704965945015257_u64,13137825018608441124_u64,1103119061885563375_u64];
Call(_13 = fn11(_1, _9, _1, _4, _10, _5, _9, _1, _1, _10, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = core::ptr::addr_of!(_7);
(*_2) = [12684070545917852234_u64,10317417535085711239_u64,6009028698480975065_u64,9050142108025344226_u64];
(*_2) = [5828105212586627197_u64,5502238860503868903_u64,9025497244929211975_u64,11560164436770011826_u64];
_13 = [3007801285_u32,374376432_u32,4052876593_u32,1011801075_u32,1145886619_u32];
(*_2) = [15366139170976016710_u64,15877889305419768470_u64,8230171507866207738_u64,1510293268025268245_u64];
_2 = core::ptr::addr_of!(_7);
_8 = [6358253673021719595_usize,4_usize,7507020022398352115_usize,2_usize,0_usize,0_usize,0_usize,4_usize];
_7 = [7882160773507947993_u64,6931926504313625385_u64,13912243558794902546_u64,3018879974582069855_u64];
_2 = core::ptr::addr_of!(_7);
_2 = core::ptr::addr_of!((*_2));
_6 = (-96_i8) as f32;
Goto(bb3)
}
bb3 = {
(*_2) = [2901360933704369207_u64,3503654162288361562_u64,7727492025341188368_u64,13621053730904492093_u64];
_14 = !true;
_10 = _5;
_3 = [1210141486263341245_usize,10839193939086657206_usize,11605992012881892410_usize,4_usize,13054854442918595930_usize,15881753569724640606_usize,6_usize,4_usize];
(*_2) = [5851543543474759267_u64,11700548654898247712_u64,12544554773318792751_u64,6881900195331369604_u64];
_4 = _1;
_1 = _9 - _9;
_14 = false;
_3 = [2_usize,10336403726184502057_usize,892451103887450675_usize,4_usize,1_usize,3554807178957174353_usize,8110661082821013827_usize,10116706155206735745_usize];
Goto(bb4)
}
bb4 = {
_15 = 3301695733574064712_u64 + 9338427874248071611_u64;
(*_2) = [_15,_15,_15,_15];
_4 = _10;
(*_2) = [_15,_15,_15,_15];
_3 = [7_usize,13643426991525987292_usize,1_usize,8842789747789850931_usize,15830337902804744147_usize,1_usize,15031365328317264322_usize,2378972110606032473_usize];
_10 = -_5;
(*_2) = [_15,_15,_15,_15];
_7 = [_15,_15,_15,_15];
_5 = _9 - _9;
_8 = _3;
_11 = '\u{b8bb0}' as isize;
_2 = core::ptr::addr_of!((*_2));
_1 = -_10;
_7 = [_15,_15,_15,_15];
_3 = [3787462253295793196_usize,5001128088666272063_usize,5742183809713481389_usize,5512075204076711980_usize,10053777597581104109_usize,8147692846535864352_usize,5_usize,4_usize];
_4 = _5;
_6 = _10 * _10;
_14 = !true;
_3 = [13273914092442919407_usize,6_usize,4_usize,344391868288203147_usize,16500758098543598263_usize,0_usize,2282684251258862584_usize,7_usize];
_3 = [6_usize,4_usize,13434995595881709149_usize,17261264018396623019_usize,3109765121078503273_usize,3_usize,14386892626469458585_usize,5_usize];
Goto(bb5)
}
bb5 = {
_19 = ['\u{e8814}','\u{2b817}','\u{796ce}','\u{103ef7}','\u{1be3d}','\u{11e1f}','\u{6f14f}'];
_9 = _5;
_7 = [_15,_15,_15,_15];
_3 = [2_usize,5_usize,7_usize,7_usize,4_usize,4895506338279999300_usize,1_usize,1_usize];
Goto(bb6)
}
bb6 = {
_19 = ['\u{106d6f}','\u{d7006}','\u{d4ba6}','\u{2a4b0}','\u{efd2c}','\u{d7829}','\u{d6620}'];
_15 = _14 as u64;
_10 = (-25_i8) as f32;
_3 = _8;
(*_2) = [_15,_15,_15,_15];
_4 = _1 - _9;
_13 = [1656194503_u32,1070784821_u32,417044307_u32,3169210365_u32,25200558_u32];
_8 = _3;
_13 = [1760427651_u32,4287487238_u32,1693170585_u32,2784778657_u32,348855107_u32];
_10 = _5 * _1;
_23 = _11 - _11;
_19 = ['\u{36817}','\u{ec7b0}','\u{ec35c}','\u{94016}','\u{44f4c}','\u{aa4ff}','\u{705d1}'];
_13 = [1912682713_u32,2831755512_u32,1398565202_u32,2019736553_u32,3609059600_u32];
_5 = _9;
_20 = [1453_i16];
_10 = _1;
RET = core::ptr::addr_of_mut!(_24.3);
(*_2) = [_15,_15,_15,_15];
_9 = _4 - _5;
_24.0 = Adt24::Variant0 { fld0: (-166024684_i32) };
_4 = 3_usize as f32;
_24.2 = 39319_u16;
_14 = true;
RET = core::ptr::addr_of_mut!((*RET));
_1 = -_5;
(*_2) = [_15,_15,_15,_15];
match _24.2 {
0 => bb4,
1 => bb7,
2 => bb8,
3 => bb9,
39319 => bb11,
_ => bb10
}
}
bb7 = {
_19 = ['\u{e8814}','\u{2b817}','\u{796ce}','\u{103ef7}','\u{1be3d}','\u{11e1f}','\u{6f14f}'];
_9 = _5;
_7 = [_15,_15,_15,_15];
_3 = [2_usize,5_usize,7_usize,7_usize,4_usize,4895506338279999300_usize,1_usize,1_usize];
Goto(bb6)
}
bb8 = {
_3 = [12689125411816954589_usize,1_usize,7_usize,3084645682177756439_usize,8697250236462034337_usize,2523781657856511500_usize,3_usize,5_usize];
_3 = [2_usize,6_usize,15062564340690010263_usize,3_usize,15310630864908244017_usize,2289690783726147464_usize,4_usize,1_usize];
_8 = [5_usize,0_usize,15124179920090992270_usize,4_usize,2_usize,11803722386070002062_usize,16193909035774656952_usize,7634666508126212184_usize];
_7 = [4753298284292365861_u64,7068704965945015257_u64,13137825018608441124_u64,1103119061885563375_u64];
Call(_13 = fn11(_1, _9, _1, _4, _10, _5, _9, _1, _1, _10, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb9 = {
(*_2) = [2901360933704369207_u64,3503654162288361562_u64,7727492025341188368_u64,13621053730904492093_u64];
_14 = !true;
_10 = _5;
_3 = [1210141486263341245_usize,10839193939086657206_usize,11605992012881892410_usize,4_usize,13054854442918595930_usize,15881753569724640606_usize,6_usize,4_usize];
(*_2) = [5851543543474759267_u64,11700548654898247712_u64,12544554773318792751_u64,6881900195331369604_u64];
_4 = _1;
_1 = _9 - _9;
_14 = false;
_3 = [2_usize,10336403726184502057_usize,892451103887450675_usize,4_usize,1_usize,3554807178957174353_usize,8110661082821013827_usize,10116706155206735745_usize];
Goto(bb4)
}
bb10 = {
_2 = core::ptr::addr_of!(_7);
(*_2) = [12684070545917852234_u64,10317417535085711239_u64,6009028698480975065_u64,9050142108025344226_u64];
(*_2) = [5828105212586627197_u64,5502238860503868903_u64,9025497244929211975_u64,11560164436770011826_u64];
_13 = [3007801285_u32,374376432_u32,4052876593_u32,1011801075_u32,1145886619_u32];
(*_2) = [15366139170976016710_u64,15877889305419768470_u64,8230171507866207738_u64,1510293268025268245_u64];
_2 = core::ptr::addr_of!(_7);
_8 = [6358253673021719595_usize,4_usize,7507020022398352115_usize,2_usize,0_usize,0_usize,0_usize,4_usize];
_7 = [7882160773507947993_u64,6931926504313625385_u64,13912243558794902546_u64,3018879974582069855_u64];
_2 = core::ptr::addr_of!(_7);
_2 = core::ptr::addr_of!((*_2));
_6 = (-96_i8) as f32;
Goto(bb3)
}
bb11 = {
_4 = _1;
_24.2 = 14194_u16 * 9004_u16;
Goto(bb12)
}
bb12 = {
_15 = '\u{1c5e}' as u64;
_15 = 3752715398026869986_u64 ^ 6444680444796535197_u64;
(*_2) = [_15,_15,_15,_15];
(*_2) = [_15,_15,_15,_15];
_1 = _6;
_15 = 12123332310109942700_u64 - 9714437186772491273_u64;
place!(Field::<i32>(Variant(_24.0, 0), 0)) = !2060215222_i32;
_1 = 6617402307917167313_i64 as f32;
_9 = 28502_i16 as f32;
_2 = core::ptr::addr_of!((*_2));
_5 = _6 + _10;
_11 = _23;
_10 = -_5;
place!(Field::<i32>(Variant(_24.0, 0), 0)) = _15 as i32;
_7 = [_15,_15,_15,_15];
_26 = 29478307369432784017706247383505701073_i128;
_9 = _23 as f32;
Goto(bb13)
}
bb13 = {
_1 = _4 + _6;
(*_2) = [_15,_15,_15,_15];
_13 = [69597805_u32,2021211279_u32,1469562553_u32,1909602792_u32,3486261927_u32];
match _26 {
0 => bb1,
1 => bb11,
2 => bb7,
3 => bb5,
29478307369432784017706247383505701073 => bb15,
_ => bb14
}
}
bb14 = {
_2 = core::ptr::addr_of!(_7);
(*_2) = [12684070545917852234_u64,10317417535085711239_u64,6009028698480975065_u64,9050142108025344226_u64];
(*_2) = [5828105212586627197_u64,5502238860503868903_u64,9025497244929211975_u64,11560164436770011826_u64];
_13 = [3007801285_u32,374376432_u32,4052876593_u32,1011801075_u32,1145886619_u32];
(*_2) = [15366139170976016710_u64,15877889305419768470_u64,8230171507866207738_u64,1510293268025268245_u64];
_2 = core::ptr::addr_of!(_7);
_8 = [6358253673021719595_usize,4_usize,7507020022398352115_usize,2_usize,0_usize,0_usize,0_usize,4_usize];
_7 = [7882160773507947993_u64,6931926504313625385_u64,13912243558794902546_u64,3018879974582069855_u64];
_2 = core::ptr::addr_of!(_7);
_2 = core::ptr::addr_of!((*_2));
_6 = (-96_i8) as f32;
Goto(bb3)
}
bb15 = {
_5 = _1;
_24.0 = Adt24::Variant1 { fld0: _15,fld1: '\u{c20b8}',fld2: _11,fld3: 1391913022_u32,fld4: 12434_i16,fld5: 7_usize,fld6: 165358453695761688324844542075321340145_u128,fld7: _26 };
_23 = _11;
_20 = [(-14802_i16)];
RET = core::ptr::addr_of_mut!((*RET));
_3 = _8;
_6 = -_10;
_20 = [20265_i16];
_11 = -Field::<isize>(Variant(_24.0, 1), 2);
(*_2) = [Field::<u64>(Variant(_24.0, 1), 0),_15,_15,Field::<u64>(Variant(_24.0, 1), 0)];
_14 = false ^ false;
_26 = _11 as i128;
_7 = [_15,_15,Field::<u64>(Variant(_24.0, 1), 0),Field::<u64>(Variant(_24.0, 1), 0)];
_16 = core::ptr::addr_of_mut!(place!(Field::<char>(Variant(_24.0, 1), 1)));
_10 = -_1;
_27 = Field::<isize>(Variant(_24.0, 1), 2);
_29 = 71029990821554201902222226720481733124_u128 as i32;
_30 = !_24.2;
_1 = (-5973376685790068915_i64) as f32;
place!(Field::<u64>(Variant(_24.0, 1), 0)) = !_15;
_29 = (-1815833111_i32);
Goto(bb16)
}
bb16 = {
Call(_32 = dump_var(10_usize, 26_usize, Move(_26), 23_usize, Move(_23), 20_usize, Move(_20), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(10_usize, 13_usize, Move(_13), 30_usize, Move(_30), 14_usize, Move(_14), 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: f32,mut _2: f32,mut _3: f32,mut _4: f32,mut _5: f32,mut _6: f32,mut _7: f32,mut _8: f32,mut _9: f32,mut _10: f32,mut _11: f32) -> [u32; 5] {
mir! {
type RET = [u32; 5];
let _12: i16;
let _13: char;
let _14: i8;
let _15: u64;
let _16: i8;
let _17: f64;
let _18: i128;
let _19: char;
let _20: [u16; 4];
let _21: [i32; 3];
let _22: [isize; 6];
let _23: &'static (u32, u64, f32);
let _24: i32;
let _25: f64;
let _26: isize;
let _27: ([u128; 1],);
let _28: u32;
let _29: f64;
let _30: bool;
let _31: *mut char;
let _32: i128;
let _33: isize;
let _34: *mut *const u8;
let _35: ();
let _36: ();
{
_11 = _1 * _1;
_11 = _4;
RET = [2070853721_u32,2696562876_u32,1595425100_u32,1290541688_u32,902713121_u32];
_2 = -_1;
_2 = -_7;
_12 = true as i16;
_5 = -_9;
_6 = _9;
_3 = _9 - _6;
_18 = 5677222891563772874_i64 as i128;
RET = [4055626268_u32,1328988748_u32,70354238_u32,1288970021_u32,2972976388_u32];
Goto(bb1)
}
bb1 = {
_17 = (-12_i8) as f64;
_14 = !(-115_i8);
_6 = _12 as f32;
_13 = '\u{7b861}';
_9 = -_3;
Goto(bb2)
}
bb2 = {
_19 = _13;
_21 = [1487137877_i32,1264573827_i32,(-736708745_i32)];
_18 = false as i128;
_9 = _10 + _4;
Call(_19 = fn12(_3, _1, _2, _5, _3, _8, _3, _5, _11), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3 = _11 * _7;
RET = [986195646_u32,1950583969_u32,2184690568_u32,959842178_u32,316639938_u32];
_15 = 7692255292435058357_u64 + 13107531129015428088_u64;
_11 = -_2;
_20 = [20864_u16,8899_u16,6875_u16,9582_u16];
_8 = 61557_u16 as f32;
_10 = _18 as f32;
_15 = 8506929412690168943_u64 + 17931134341621180249_u64;
_6 = 299418784094332607121600819857045935777_u128 as f32;
_15 = 9570163729949661013_u64 ^ 11698851563019285340_u64;
_19 = _13;
RET = [748597369_u32,4164440330_u32,1322449046_u32,3550727166_u32,1644963486_u32];
_13 = _19;
_8 = -_11;
_13 = _19;
_24 = 1410713718_i32 ^ (-307460982_i32);
_25 = _17 * _17;
_5 = _3;
Goto(bb4)
}
bb4 = {
_5 = _3 - _3;
_26 = (-9223372036854775808_isize);
_13 = _19;
_20 = [42982_u16,28654_u16,60185_u16,62240_u16];
_13 = _19;
_11 = _8;
_20 = [65268_u16,17354_u16,7340_u16,33324_u16];
Goto(bb5)
}
bb5 = {
RET = [2691823432_u32,4151758586_u32,376633939_u32,3539109598_u32,2473682935_u32];
_13 = _19;
_20 = [40798_u16,18389_u16,41652_u16,4539_u16];
RET = [2169392402_u32,1288750895_u32,622849569_u32,808428767_u32,898663234_u32];
RET = [3740609808_u32,3772129995_u32,3704776555_u32,4038539740_u32,4292767989_u32];
_24 = _26 as i32;
_21 = [_24,_24,_24];
_17 = -_25;
_10 = _1;
_22 = [_26,_26,_26,_26,_26,_26];
match _26 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb6 = {
_5 = _3 - _3;
_26 = (-9223372036854775808_isize);
_13 = _19;
_20 = [42982_u16,28654_u16,60185_u16,62240_u16];
_13 = _19;
_11 = _8;
_20 = [65268_u16,17354_u16,7340_u16,33324_u16];
Goto(bb5)
}
bb7 = {
_3 = _11 * _7;
RET = [986195646_u32,1950583969_u32,2184690568_u32,959842178_u32,316639938_u32];
_15 = 7692255292435058357_u64 + 13107531129015428088_u64;
_11 = -_2;
_20 = [20864_u16,8899_u16,6875_u16,9582_u16];
_8 = 61557_u16 as f32;
_10 = _18 as f32;
_15 = 8506929412690168943_u64 + 17931134341621180249_u64;
_6 = 299418784094332607121600819857045935777_u128 as f32;
_15 = 9570163729949661013_u64 ^ 11698851563019285340_u64;
_19 = _13;
RET = [748597369_u32,4164440330_u32,1322449046_u32,3550727166_u32,1644963486_u32];
_13 = _19;
_8 = -_11;
_13 = _19;
_24 = 1410713718_i32 ^ (-307460982_i32);
_25 = _17 * _17;
_5 = _3;
Goto(bb4)
}
bb8 = {
_19 = _13;
_21 = [1487137877_i32,1264573827_i32,(-736708745_i32)];
_18 = false as i128;
_9 = _10 + _4;
Call(_19 = fn12(_3, _1, _2, _5, _3, _8, _3, _5, _11), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_17 = (-12_i8) as f64;
_14 = !(-115_i8);
_6 = _12 as f32;
_13 = '\u{7b861}';
_9 = -_3;
Goto(bb2)
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
_12 = _24 as i16;
_16 = _14;
_24 = (-591376945_i32) + (-1248285936_i32);
_27.0 = [142805713894761281031685108557612533650_u128];
Call(_18 = core::intrinsics::bswap(75430828801716167765251814741230163462_i128), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_3 = -_7;
_4 = -_5;
_24 = !(-1968210089_i32);
_10 = _2;
_11 = _3;
_22 = [_26,_26,_26,_26,_26,_26];
_1 = _9;
_10 = -_4;
_1 = _3 * _9;
_17 = _2 as f64;
RET = [1764204425_u32,2586462160_u32,1387249943_u32,2097271015_u32,933143398_u32];
_11 = _10 * _10;
_29 = _17;
_17 = -_29;
_4 = _12 as f32;
_20 = [29183_u16,58311_u16,49441_u16,43566_u16];
_11 = _9 + _10;
_32 = _18;
_4 = -_8;
_19 = _13;
_10 = 14542793831532976920_usize as f32;
_15 = 10662136025130688219_u64 ^ 14995916069661585862_u64;
_16 = _14;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(11_usize, 16_usize, Move(_16), 21_usize, Move(_21), 20_usize, Move(_20), 26_usize, Move(_26)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(11_usize, 15_usize, Move(_15), 18_usize, Move(_18), 24_usize, Move(_24), 36_usize, _36), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: f32,mut _2: f32,mut _3: f32,mut _4: f32,mut _5: f32,mut _6: f32,mut _7: f32,mut _8: f32,mut _9: f32) -> char {
mir! {
type RET = char;
let _10: f32;
let _11: (f32, Adt50, Adt34);
let _12: ([i32; 3],);
let _13: *const i128;
let _14: isize;
let _15: isize;
let _16: (f32, &'static (Adt24, *const *const isize, u16, *const u8));
let _17: isize;
let _18: i16;
let _19: &'static *const i128;
let _20: usize;
let _21: *const Adt24;
let _22: isize;
let _23: ([u128; 1],);
let _24: [u32; 5];
let _25: [i32; 3];
let _26: isize;
let _27: f64;
let _28: isize;
let _29: f32;
let _30: *const char;
let _31: isize;
let _32: isize;
let _33: f64;
let _34: (*mut *const u8, (*const *const isize, *mut *const u8, [u16; 8], i16), [u128; 1]);
let _35: ();
let _36: ();
{
_8 = -_5;
_4 = 173114796890666583135491770701206359442_u128 as f32;
_2 = -_7;
_4 = _5 + _9;
_2 = -_9;
_5 = _4;
_10 = -_8;
_11.0 = 2978534136_u32 as f32;
_10 = _2 + _5;
_4 = -_10;
_6 = _5 + _2;
_4 = 1148259846_i32 as f32;
_6 = _10 - _1;
_10 = (-9223372036854775808_isize) as f32;
_12.0 = [(-1973093424_i32),(-1725764440_i32),1975288454_i32];
RET = '\u{22ce4}';
_4 = _1;
_4 = _6;
_6 = _8 + _5;
_4 = -_5;
_10 = _6;
_3 = -_8;
_6 = _4 - _10;
Goto(bb1)
}
bb1 = {
_8 = _5 - _4;
Call(_9 = fn13(), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = '\u{71602}';
_4 = _10 - _10;
_8 = -_1;
_8 = (-17_i8) as f32;
_12.0 = [(-2072971357_i32),(-475561184_i32),(-1460532853_i32)];
_16.0 = _10 - _4;
_16.0 = -_4;
_12.0 = [405610899_i32,258388577_i32,1704215128_i32];
_4 = _6;
_6 = _4 * _5;
_17 = 9223372036854775807_isize;
_3 = -_6;
_11.0 = 86_u8 as f32;
_3 = -_1;
_14 = !_17;
_3 = _10;
RET = '\u{e8fa6}';
_4 = -_5;
_15 = false as isize;
RET = '\u{10a77c}';
_7 = _16.0 * _3;
match _17 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
9223372036854775807 => bb8,
_ => bb7
}
}
bb3 = {
_8 = _5 - _4;
Call(_9 = fn13(), ReturnTo(bb2), UnwindUnreachable())
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
RET = '\u{21aaa}';
_18 = 3666_i16;
_19 = &_13;
_3 = 64_u8 as f32;
_22 = -_14;
_17 = -_15;
_23.0 = [202932513984208522637800531923548931227_u128];
_23.0 = [237779194129631023555230264754904052887_u128];
_19 = &_13;
_12.0 = [233516021_i32,(-1333523955_i32),1381079400_i32];
_20 = 1546809953936912137_u64 as usize;
_19 = &(*_19);
_25 = _12.0;
_25 = [(-1277986018_i32),(-776529640_i32),(-1318261651_i32)];
_22 = _15 & _17;
_24 = [2555388865_u32,3012574894_u32,3553312187_u32,4126224451_u32,4074710980_u32];
_19 = &(*_19);
_3 = _18 as f32;
Goto(bb9)
}
bb9 = {
_14 = -_15;
_15 = _14;
_9 = _10 + _7;
_22 = !_17;
RET = '\u{ac6dd}';
_23.0 = [79053912747033621678719157898956228248_u128];
_25 = [2060171312_i32,2133435274_i32,1899998397_i32];
_6 = -_10;
_2 = _16.0;
_9 = _10 - _4;
_28 = _22 << _17;
_26 = !_28;
_1 = _6 * _16.0;
RET = '\u{79247}';
match _18 {
0 => bb1,
1 => bb10,
2 => bb11,
3666 => bb13,
_ => bb12
}
}
bb10 = {
RET = '\u{21aaa}';
_18 = 3666_i16;
_19 = &_13;
_3 = 64_u8 as f32;
_22 = -_14;
_17 = -_15;
_23.0 = [202932513984208522637800531923548931227_u128];
_23.0 = [237779194129631023555230264754904052887_u128];
_19 = &_13;
_12.0 = [233516021_i32,(-1333523955_i32),1381079400_i32];
_20 = 1546809953936912137_u64 as usize;
_19 = &(*_19);
_25 = _12.0;
_25 = [(-1277986018_i32),(-776529640_i32),(-1318261651_i32)];
_22 = _15 & _17;
_24 = [2555388865_u32,3012574894_u32,3553312187_u32,4126224451_u32,4074710980_u32];
_19 = &(*_19);
_3 = _18 as f32;
Goto(bb9)
}
bb11 = {
RET = '\u{71602}';
_4 = _10 - _10;
_8 = -_1;
_8 = (-17_i8) as f32;
_12.0 = [(-2072971357_i32),(-475561184_i32),(-1460532853_i32)];
_16.0 = _10 - _4;
_16.0 = -_4;
_12.0 = [405610899_i32,258388577_i32,1704215128_i32];
_4 = _6;
_6 = _4 * _5;
_17 = 9223372036854775807_isize;
_3 = -_6;
_11.0 = 86_u8 as f32;
_3 = -_1;
_14 = !_17;
_3 = _10;
RET = '\u{e8fa6}';
_4 = -_5;
_15 = false as isize;
RET = '\u{10a77c}';
_7 = _16.0 * _3;
match _17 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
9223372036854775807 => bb8,
_ => bb7
}
}
bb12 = {
Return()
}
bb13 = {
_1 = _20 as f32;
_4 = -_7;
_30 = core::ptr::addr_of!(RET);
RET = '\u{c0e59}';
_12.0 = _25;
_23.0 = [171722850211919777783150236357967384669_u128];
_6 = _3;
(*_30) = '\u{b730b}';
_6 = _7;
_1 = _6 * _16.0;
_7 = -_1;
_31 = _22;
_27 = _26 as f64;
_22 = _28;
_4 = 104_u8 as f32;
match _18 {
0 => bb1,
1 => bb10,
2 => bb8,
3 => bb4,
4 => bb5,
5 => bb6,
3666 => bb14,
_ => bb7
}
}
bb14 = {
(*_30) = '\u{e946a}';
_6 = _7 + _1;
_11.0 = _1 - _7;
(*_30) = '\u{de577}';
_26 = _31 - _17;
_17 = -_22;
_19 = &(*_19);
_24 = [2435236075_u32,319012078_u32,3106548274_u32,3985076327_u32,3399765395_u32];
_10 = -_6;
_29 = _16.0 * _10;
_31 = _26 | _28;
_33 = 242685343876760645438114086764503944003_u128 as f64;
_3 = _5;
_9 = _11.0 * _1;
_20 = 12_i8 as usize;
_7 = _20 as f32;
_34.1.2 = [65394_u16,40116_u16,10413_u16,27204_u16,25477_u16,54482_u16,13624_u16,29963_u16];
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(12_usize, 14_usize, Move(_14), 23_usize, Move(_23), 31_usize, Move(_31), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(12_usize, 26_usize, Move(_26), 25_usize, Move(_25), 36_usize, _36, 36_usize, _36), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13() -> f32 {
mir! {
type RET = f32;
let _1: *const u32;
let _2: i128;
let _3: *mut u64;
let _4: (u8, u128, Adt31, *const *const isize);
let _5: char;
let _6: isize;
let _7: bool;
let _8: *const isize;
let _9: Adt71;
let _10: *mut u64;
let _11: i128;
let _12: i32;
let _13: f64;
let _14: [i16; 5];
let _15: f32;
let _16: isize;
let _17: isize;
let _18: [i32; 3];
let _19: u32;
let _20: char;
let _21: ([i32; 3],);
let _22: ([bool; 4], [usize; 8]);
let _23: isize;
let _24: [u16; 4];
let _25: isize;
let _26: [bool; 4];
let _27: (f32, &'static (Adt24, *const *const isize, u16, *const u8));
let _28: [u32; 5];
let _29: f64;
let _30: usize;
let _31: Adt66;
let _32: ();
let _33: ();
{
RET = (-86726059_i32) as f32;
RET = 15_i8 as f32;
RET = 26007_u16 as f32;
RET = (-8392695475344166228_i64) as f32;
_2 = (-65940414860295168047470360211748823713_i128);
_2 = !168713397486874503900248077928245893408_i128;
_2 = (-41889297248777129954856390481227297349_i128);
_4.2.fld0 = !false;
_4.2.fld6 = 231133987432212912_i64 + 6247363416855216297_i64;
_4.2.fld5 = Adt24::Variant0 { fld0: 858182526_i32 };
_5 = '\u{49bbe}';
_4.3 = core::ptr::addr_of!(_4.2.fld3);
_4.2.fld5 = Adt24::Variant0 { fld0: 184325814_i32 };
_4.2.fld4 = 14559037258320036790_usize;
_5 = '\u{6db1d}';
_4.2.fld2 = [21641_u16,26781_u16,40105_u16,62346_u16,51828_u16,24731_u16,7933_u16,38563_u16];
_4.0 = !194_u8;
_4.2.fld3 = core::ptr::addr_of!(_6);
_4.2.fld1 = _5;
Goto(bb1)
}
bb1 = {
RET = 9328_u16 as f32;
_5 = _4.2.fld1;
_4.2.fld1 = _5;
_4.2.fld0 = RET >= RET;
place!(Field::<i32>(Variant(_4.2.fld5, 0), 0)) = (-102_isize) as i32;
_4.2.fld3 = core::ptr::addr_of!(_6);
_4.2.fld4 = !0_usize;
_4.2.fld0 = !true;
_4.2.fld6 = !8015265037013111857_i64;
Goto(bb2)
}
bb2 = {
_4.2.fld6 = -(-3275500389285335296_i64);
_4.2.fld2 = [58387_u16,46172_u16,7726_u16,38703_u16,52014_u16,29357_u16,28205_u16,39735_u16];
_4.0 = !233_u8;
_4.2.fld6 = 7839582003342379312_i64 & 9134037482873540550_i64;
_8 = core::ptr::addr_of!(_6);
_4.2.fld2 = [51526_u16,10936_u16,62763_u16,41041_u16,30863_u16,1586_u16,42875_u16,44533_u16];
_7 = RET != RET;
SetDiscriminant(_4.2.fld5, 2);
place!(Field::<u128>(Variant(_4.2.fld5, 2), 1)) = !234976345403049594388923730346797102129_u128;
_4.0 = 255_u8;
place!(Field::<isize>(Variant(_4.2.fld5, 2), 2)) = !(-58_isize);
place!(Field::<i16>(Variant(_4.2.fld5, 2), 4)) = 3562129612_u32 as i16;
_4.2.fld3 = core::ptr::addr_of!(_6);
Goto(bb3)
}
bb3 = {
_4.2.fld1 = _5;
match _2 {
0 => bb1,
1 => bb2,
298393069672161333508518216950540914107 => bb5,
_ => bb4
}
}
bb4 = {
_4.2.fld6 = -(-3275500389285335296_i64);
_4.2.fld2 = [58387_u16,46172_u16,7726_u16,38703_u16,52014_u16,29357_u16,28205_u16,39735_u16];
_4.0 = !233_u8;
_4.2.fld6 = 7839582003342379312_i64 & 9134037482873540550_i64;
_8 = core::ptr::addr_of!(_6);
_4.2.fld2 = [51526_u16,10936_u16,62763_u16,41041_u16,30863_u16,1586_u16,42875_u16,44533_u16];
_7 = RET != RET;
SetDiscriminant(_4.2.fld5, 2);
place!(Field::<u128>(Variant(_4.2.fld5, 2), 1)) = !234976345403049594388923730346797102129_u128;
_4.0 = 255_u8;
place!(Field::<isize>(Variant(_4.2.fld5, 2), 2)) = !(-58_isize);
place!(Field::<i16>(Variant(_4.2.fld5, 2), 4)) = 3562129612_u32 as i16;
_4.2.fld3 = core::ptr::addr_of!(_6);
Goto(bb3)
}
bb5 = {
_5 = _4.2.fld1;
RET = Field::<isize>(Variant(_4.2.fld5, 2), 2) as f32;
(*_8) = !Field::<isize>(Variant(_4.2.fld5, 2), 2);
_4.2.fld2 = [56173_u16,64064_u16,41280_u16,49502_u16,24122_u16,9691_u16,20585_u16,57321_u16];
(*_8) = Field::<isize>(Variant(_4.2.fld5, 2), 2);
_4.2.fld0 = _7 | _7;
RET = (*_8) as f32;
place!(Field::<[u16; 8]>(Variant(_4.2.fld5, 2), 3)) = _4.2.fld2;
place!(Field::<u128>(Variant(_4.2.fld5, 2), 1)) = !221471107542669675236780348025219969696_u128;
_4.2.fld6 = 4855695485525135815_i64;
_2 = !(-136943508302576404226321284421086772495_i128);
place!(Field::<[u16; 8]>(Variant(_4.2.fld5, 2), 3)) = [65337_u16,37491_u16,52565_u16,32367_u16,3588_u16,7814_u16,59779_u16,16163_u16];
_4.3 = core::ptr::addr_of!(_8);
place!(Field::<isize>(Variant(_4.2.fld5, 2), 2)) = _6;
RET = 178903395_i32 as f32;
place!(Field::<u128>(Variant(_4.2.fld5, 2), 1)) = 103908498942485885427040087073808233662_u128 & 303823461701262444472288097258435492310_u128;
_7 = _4.2.fld0 < _4.2.fld0;
(*_8) = Field::<isize>(Variant(_4.2.fld5, 2), 2) ^ Field::<isize>(Variant(_4.2.fld5, 2), 2);
_4.2.fld0 = _7;
_4.2.fld1 = _5;
_7 = _4.2.fld0 & _4.2.fld0;
_8 = Move(_4.2.fld3);
_11 = _2 * _2;
_4.2.fld6 = 4836032975352031173_i64 << _6;
_13 = 1421528165_u32 as f64;
_7 = !_4.2.fld0;
_14 = [Field::<i16>(Variant(_4.2.fld5, 2), 4),Field::<i16>(Variant(_4.2.fld5, 2), 4),Field::<i16>(Variant(_4.2.fld5, 2), 4),Field::<i16>(Variant(_4.2.fld5, 2), 4),Field::<i16>(Variant(_4.2.fld5, 2), 4)];
Goto(bb6)
}
bb6 = {
match _4.0 {
0 => bb5,
1 => bb4,
2 => bb3,
3 => bb7,
4 => bb8,
255 => bb10,
_ => bb9
}
}
bb7 = {
RET = 9328_u16 as f32;
_5 = _4.2.fld1;
_4.2.fld1 = _5;
_4.2.fld0 = RET >= RET;
place!(Field::<i32>(Variant(_4.2.fld5, 0), 0)) = (-102_isize) as i32;
_4.2.fld3 = core::ptr::addr_of!(_6);
_4.2.fld4 = !0_usize;
_4.2.fld0 = !true;
_4.2.fld6 = !8015265037013111857_i64;
Goto(bb2)
}
bb8 = {
_4.2.fld6 = -(-3275500389285335296_i64);
_4.2.fld2 = [58387_u16,46172_u16,7726_u16,38703_u16,52014_u16,29357_u16,28205_u16,39735_u16];
_4.0 = !233_u8;
_4.2.fld6 = 7839582003342379312_i64 & 9134037482873540550_i64;
_8 = core::ptr::addr_of!(_6);
_4.2.fld2 = [51526_u16,10936_u16,62763_u16,41041_u16,30863_u16,1586_u16,42875_u16,44533_u16];
_7 = RET != RET;
SetDiscriminant(_4.2.fld5, 2);
place!(Field::<u128>(Variant(_4.2.fld5, 2), 1)) = !234976345403049594388923730346797102129_u128;
_4.0 = 255_u8;
place!(Field::<isize>(Variant(_4.2.fld5, 2), 2)) = !(-58_isize);
place!(Field::<i16>(Variant(_4.2.fld5, 2), 4)) = 3562129612_u32 as i16;
_4.2.fld3 = core::ptr::addr_of!(_6);
Goto(bb3)
}
bb9 = {
_4.2.fld1 = _5;
match _2 {
0 => bb1,
1 => bb2,
298393069672161333508518216950540914107 => bb5,
_ => bb4
}
}
bb10 = {
_15 = -RET;
_12 = Field::<u128>(Variant(_4.2.fld5, 2), 1) as i32;
place!(Field::<u128>(Variant(_4.2.fld5, 2), 1)) = _12 as u128;
_4.2.fld5 = Adt24::Variant0 { fld0: _12 };
_4.2.fld4 = !1836806555021428632_usize;
_4.2.fld3 = core::ptr::addr_of!(_17);
_12 = !Field::<i32>(Variant(_4.2.fld5, 0), 0);
_17 = _6;
_2 = _11;
_14 = [(-4526_i16),(-706_i16),(-23041_i16),(-5811_i16),(-10131_i16)];
_8 = core::ptr::addr_of!(_17);
_8 = Move(_4.2.fld3);
_6 = _17 * _17;
_4.2.fld3 = core::ptr::addr_of!(_17);
place!(Field::<i32>(Variant(_4.2.fld5, 0), 0)) = 975869383_u32 as i32;
_4.1 = 112_i8 as u128;
SetDiscriminant(_4.2.fld5, 2);
_5 = _4.2.fld1;
_17 = _6;
_8 = Move(_4.2.fld3);
_4.0 = _4.2.fld1 as u8;
Goto(bb11)
}
bb11 = {
_4.1 = 175030161355584088384817016685013785886_u128;
_5 = _4.2.fld1;
_4.0 = !43_u8;
_4.2.fld4 = 6_usize;
place!(Field::<[u16; 8]>(Variant(_4.2.fld5, 2), 3)) = [48843_u16,50841_u16,44562_u16,22649_u16,15341_u16,22738_u16,65107_u16,63826_u16];
_4.2.fld4 = _11 as usize;
Call(_21.0 = fn14(), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_13 = _4.1 as f64;
_4.2.fld6 = 4557365431463146141_i64 ^ (-6858913196408923394_i64);
_15 = -RET;
RET = _4.0 as f32;
_4.2.fld3 = core::ptr::addr_of!(place!(Field::<isize>(Variant(_4.2.fld5, 2), 2)));
_11 = _2 ^ _2;
_4.2.fld5 = Adt24::Variant1 { fld0: 1469764228774732150_u64,fld1: _5,fld2: _17,fld3: 3254644431_u32,fld4: 17531_i16,fld5: _4.2.fld4,fld6: _4.1,fld7: _11 };
_4.2.fld1 = Field::<char>(Variant(_4.2.fld5, 1), 1);
place!(Field::<u64>(Variant(_4.2.fld5, 1), 0)) = 6689926423436525635_u64 + 11086130268069137816_u64;
_1 = core::ptr::addr_of!(_19);
place!(Field::<char>(Variant(_4.2.fld5, 1), 1)) = _5;
_4.2.fld6 = -674543467984722784_i64;
place!(Field::<u32>(Variant(_4.2.fld5, 1), 3)) = 3777501531_u32 << _17;
_18 = [_12,_12,_12];
_19 = 10806_i16 as u32;
Call(_2 = core::intrinsics::transmute(_4.2.fld2), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_4.2.fld4 = !Field::<usize>(Variant(_4.2.fld5, 1), 5);
_22.0 = [_4.2.fld0,_4.2.fld0,_4.2.fld0,_7];
_2 = _13 as i128;
place!(Field::<i16>(Variant(_4.2.fld5, 1), 4)) = 20167_i16;
_18 = [_12,_12,_12];
place!(Field::<usize>(Variant(_4.2.fld5, 1), 5)) = _4.2.fld4;
_4.2.fld2 = [18582_u16,4852_u16,57846_u16,64178_u16,26313_u16,14886_u16,52281_u16,3467_u16];
_10 = core::ptr::addr_of_mut!(place!(Field::<u64>(Variant(_4.2.fld5, 1), 0)));
place!(Field::<u32>(Variant(_4.2.fld5, 1), 3)) = (-94_i8) as u32;
_3 = Move(_10);
_16 = Field::<isize>(Variant(_4.2.fld5, 1), 2);
_14 = [Field::<i16>(Variant(_4.2.fld5, 1), 4),Field::<i16>(Variant(_4.2.fld5, 1), 4),Field::<i16>(Variant(_4.2.fld5, 1), 4),Field::<i16>(Variant(_4.2.fld5, 1), 4),Field::<i16>(Variant(_4.2.fld5, 1), 4)];
_22.0 = [_7,_4.2.fld0,_4.2.fld0,_4.2.fld0];
SetDiscriminant(_4.2.fld5, 1);
_26 = [_7,_4.2.fld0,_7,_7];
_21 = (_18,);
_4.3 = core::ptr::addr_of!(_8);
RET = _15 + _15;
_4.1 = 310683851747908988330337446864130040029_u128;
_13 = _4.2.fld4 as f64;
_5 = _4.2.fld1;
_27.0 = -RET;
Goto(bb14)
}
bb14 = {
_4.3 = core::ptr::addr_of!(_4.2.fld3);
place!(Field::<u64>(Variant(_4.2.fld5, 1), 0)) = 4845326718889811030_u64;
place!(Field::<usize>(Variant(_4.2.fld5, 1), 5)) = _11 as usize;
_2 = _11 & _11;
_2 = _4.2.fld6 as i128;
RET = -_15;
RET = _4.2.fld6 as f32;
place!(Field::<i128>(Variant(_4.2.fld5, 1), 7)) = _12 as i128;
_4.2.fld0 = _7 ^ _7;
_15 = _27.0 * _27.0;
_29 = _11 as f64;
_4.3 = core::ptr::addr_of!(_4.2.fld3);
_28 = [(*_1),(*_1),_19,(*_1),(*_1)];
_29 = _13 - _13;
_22.0 = [_7,_4.2.fld0,_4.2.fld0,_4.2.fld0];
_10 = core::ptr::addr_of_mut!(place!(Field::<u64>(Variant(_4.2.fld5, 1), 0)));
place!(Field::<isize>(Variant(_4.2.fld5, 1), 2)) = _6;
_29 = _6 as f64;
_4.2.fld0 = _7;
(*_10) = 16971756570092966177_u64;
place!(Field::<usize>(Variant(_4.2.fld5, 1), 5)) = _7 as usize;
place!(Field::<usize>(Variant(_4.2.fld5, 1), 5)) = !_4.2.fld4;
(*_10) = _12 as u64;
place!(Field::<i16>(Variant(_4.2.fld5, 1), 4)) = -(-16207_i16);
_30 = !_4.2.fld4;
place!(Field::<char>(Variant(_4.2.fld5, 1), 1)) = _5;
_24 = [10430_u16,5425_u16,38618_u16,19158_u16];
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(13_usize, 18_usize, Move(_18), 6_usize, Move(_6), 17_usize, Move(_17), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(13_usize, 7_usize, Move(_7), 24_usize, Move(_24), 19_usize, Move(_19), 26_usize, Move(_26)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14() -> [i32; 3] {
mir! {
type RET = [i32; 3];
let _1: *const Adt24;
let _2: (f32, Adt50, Adt34);
let _3: i16;
let _4: i32;
let _5: *mut u64;
let _6: isize;
let _7: [char; 7];
let _8: u16;
let _9: usize;
let _10: ();
let _11: ();
{
RET = [822347937_i32,1320653959_i32,331045738_i32];
RET = [(-434628772_i32),(-1311336095_i32),(-1893464343_i32)];
Goto(bb1)
}
bb1 = {
RET = [703247007_i32,206794685_i32,678885824_i32];
RET = [719822523_i32,177938137_i32,242798653_i32];
RET = [566437550_i32,410788525_i32,(-597641081_i32)];
RET = [664211193_i32,1926853898_i32,(-820067244_i32)];
RET = [1756897912_i32,(-1205820181_i32),(-928829810_i32)];
RET = [830868887_i32,(-2112750355_i32),1640060065_i32];
RET = [1462752812_i32,(-2145381289_i32),969559951_i32];
RET = [790764575_i32,1816502077_i32,1340770483_i32];
_2.0 = (-100_isize) as f32;
Goto(bb2)
}
bb2 = {
RET = [1852157069_i32,(-1021819947_i32),(-372424497_i32)];
RET = [(-2045861545_i32),2006583423_i32,(-1773050542_i32)];
RET = [2146952338_i32,(-692985290_i32),1268704076_i32];
_2.0 = 49161_u16 as f32;
RET = [660511358_i32,(-1875950369_i32),(-1497251723_i32)];
RET = [(-937077350_i32),1260372472_i32,(-373116217_i32)];
Call(_3 = fn15(RET, RET, RET, RET, _2.0, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = (-746953909_i32) & (-840234048_i32);
_4 = (-1189458878_i32);
_6 = 9223372036854775807_isize << _3;
RET = [_4,_4,_4];
_3 = 28137_i16;
RET = [_4,_4,_4];
match _3 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
28137 => bb8,
_ => bb7
}
}
bb4 = {
RET = [1852157069_i32,(-1021819947_i32),(-372424497_i32)];
RET = [(-2045861545_i32),2006583423_i32,(-1773050542_i32)];
RET = [2146952338_i32,(-692985290_i32),1268704076_i32];
_2.0 = 49161_u16 as f32;
RET = [660511358_i32,(-1875950369_i32),(-1497251723_i32)];
RET = [(-937077350_i32),1260372472_i32,(-373116217_i32)];
Call(_3 = fn15(RET, RET, RET, RET, _2.0, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
RET = [703247007_i32,206794685_i32,678885824_i32];
RET = [719822523_i32,177938137_i32,242798653_i32];
RET = [566437550_i32,410788525_i32,(-597641081_i32)];
RET = [664211193_i32,1926853898_i32,(-820067244_i32)];
RET = [1756897912_i32,(-1205820181_i32),(-928829810_i32)];
RET = [830868887_i32,(-2112750355_i32),1640060065_i32];
RET = [1462752812_i32,(-2145381289_i32),969559951_i32];
RET = [790764575_i32,1816502077_i32,1340770483_i32];
_2.0 = (-100_isize) as f32;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_6 = 9223372036854775807_isize + 9223372036854775807_isize;
RET = [_4,_4,_4];
RET = [_4,_4,_4];
_7 = ['\u{103697}','\u{f6490}','\u{bd668}','\u{eba85}','\u{596f}','\u{48518}','\u{47489}'];
RET = [_4,_4,_4];
RET = [_4,_4,_4];
RET = [_4,_4,_4];
_3 = 31036_i16;
match _4 {
0 => bb3,
1 => bb2,
2 => bb9,
3 => bb10,
4 => bb11,
340282366920938463463374607430578752578 => bb13,
_ => bb12
}
}
bb9 = {
_4 = (-746953909_i32) & (-840234048_i32);
_4 = (-1189458878_i32);
_6 = 9223372036854775807_isize << _3;
RET = [_4,_4,_4];
_3 = 28137_i16;
RET = [_4,_4,_4];
match _3 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
28137 => bb8,
_ => bb7
}
}
bb10 = {
Return()
}
bb11 = {
RET = [703247007_i32,206794685_i32,678885824_i32];
RET = [719822523_i32,177938137_i32,242798653_i32];
RET = [566437550_i32,410788525_i32,(-597641081_i32)];
RET = [664211193_i32,1926853898_i32,(-820067244_i32)];
RET = [1756897912_i32,(-1205820181_i32),(-928829810_i32)];
RET = [830868887_i32,(-2112750355_i32),1640060065_i32];
RET = [1462752812_i32,(-2145381289_i32),969559951_i32];
RET = [790764575_i32,1816502077_i32,1340770483_i32];
_2.0 = (-100_isize) as f32;
Goto(bb2)
}
bb12 = {
RET = [1852157069_i32,(-1021819947_i32),(-372424497_i32)];
RET = [(-2045861545_i32),2006583423_i32,(-1773050542_i32)];
RET = [2146952338_i32,(-692985290_i32),1268704076_i32];
_2.0 = 49161_u16 as f32;
RET = [660511358_i32,(-1875950369_i32),(-1497251723_i32)];
RET = [(-937077350_i32),1260372472_i32,(-373116217_i32)];
Call(_3 = fn15(RET, RET, RET, RET, _2.0, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_9 = (-66_i8) as usize;
_7 = ['\u{dc87b}','\u{26a6}','\u{531f0}','\u{1090f1}','\u{5af81}','\u{86b15}','\u{77531}'];
_4 = _3 as i32;
match _3 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
31036 => bb19,
_ => bb18
}
}
bb14 = {
RET = [1852157069_i32,(-1021819947_i32),(-372424497_i32)];
RET = [(-2045861545_i32),2006583423_i32,(-1773050542_i32)];
RET = [2146952338_i32,(-692985290_i32),1268704076_i32];
_2.0 = 49161_u16 as f32;
RET = [660511358_i32,(-1875950369_i32),(-1497251723_i32)];
RET = [(-937077350_i32),1260372472_i32,(-373116217_i32)];
Call(_3 = fn15(RET, RET, RET, RET, _2.0, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
RET = [703247007_i32,206794685_i32,678885824_i32];
RET = [719822523_i32,177938137_i32,242798653_i32];
RET = [566437550_i32,410788525_i32,(-597641081_i32)];
RET = [664211193_i32,1926853898_i32,(-820067244_i32)];
RET = [1756897912_i32,(-1205820181_i32),(-928829810_i32)];
RET = [830868887_i32,(-2112750355_i32),1640060065_i32];
RET = [1462752812_i32,(-2145381289_i32),969559951_i32];
RET = [790764575_i32,1816502077_i32,1340770483_i32];
_2.0 = (-100_isize) as f32;
Goto(bb2)
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
_6 = 9223372036854775807_isize + 9223372036854775807_isize;
RET = [_4,_4,_4];
RET = [_4,_4,_4];
_7 = ['\u{103697}','\u{f6490}','\u{bd668}','\u{eba85}','\u{596f}','\u{48518}','\u{47489}'];
RET = [_4,_4,_4];
RET = [_4,_4,_4];
RET = [_4,_4,_4];
_3 = 31036_i16;
match _4 {
0 => bb3,
1 => bb2,
2 => bb9,
3 => bb10,
4 => bb11,
340282366920938463463374607430578752578 => bb13,
_ => bb12
}
}
bb19 = {
_7 = ['\u{cbf57}','\u{d2754}','\u{42fb0}','\u{7d3de}','\u{878aa}','\u{ed2f5}','\u{d5440}'];
_4 = 1759709188_i32 + 1793289206_i32;
RET = [_4,_4,_4];
_7 = ['\u{613f4}','\u{63c48}','\u{1e61a}','\u{beff1}','\u{57b6b}','\u{c8909}','\u{cf990}'];
_8 = (-41292091020182713990916774266314709158_i128) as u16;
Goto(bb20)
}
bb20 = {
Call(_10 = dump_var(14_usize, 3_usize, Move(_3), 4_usize, Move(_4), 9_usize, Move(_9), 11_usize, _11), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: [i32; 3],mut _2: [i32; 3],mut _3: [i32; 3],mut _4: [i32; 3],mut _5: f32,mut _6: [i32; 3]) -> i16 {
mir! {
type RET = i16;
let _7: Adt31;
let _8: u8;
let _9: bool;
let _10: f32;
let _11: f64;
let _12: bool;
let _13: [u8; 4];
let _14: &'static u8;
let _15: [usize; 8];
let _16: f64;
let _17: i64;
let _18: [i32; 1];
let _19: [i32; 1];
let _20: isize;
let _21: i32;
let _22: *mut u64;
let _23: Adt50;
let _24: bool;
let _25: isize;
let _26: isize;
let _27: ();
let _28: ();
{
RET = -(-9236_i16);
_6 = [2057777089_i32,(-1374074911_i32),(-1113195807_i32)];
_3 = [746509002_i32,(-1899731087_i32),(-318964442_i32)];
_4 = [52356748_i32,(-955719245_i32),(-616514381_i32)];
_2 = _6;
_1 = [1091003442_i32,(-515535419_i32),2078843463_i32];
RET = 19085_i16 >> 1846095503_u32;
_3 = [1558730714_i32,(-846379302_i32),(-1156081920_i32)];
_1 = [(-1343804156_i32),(-998655223_i32),407415667_i32];
_2 = _4;
RET = !18562_i16;
_3 = [440839786_i32,(-921551407_i32),(-260504848_i32)];
_7.fld5 = Adt24::Variant0 { fld0: (-498796466_i32) };
_4 = _2;
place!(Field::<i32>(Variant(_7.fld5, 0), 0)) = '\u{2c972}' as i32;
SetDiscriminant(_7.fld5, 3);
place!(Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3)).2 = _5 * _5;
Goto(bb1)
}
bb1 = {
place!(Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3)) = (215738932_u32, 8436157484517588467_u64, _5);
place!(Field::<i16>(Variant(_7.fld5, 3), 4)) = RET;
_7.fld1 = '\u{fd293}';
_3 = [(-912696642_i32),712350528_i32,(-1126182763_i32)];
_7.fld6 = (-290987573832998541_i64);
place!(Field::<i32>(Variant(_7.fld5, 3), 5)) = (-100788572132003351342444599316365307995_i128) as i32;
_7.fld4 = 8268574917817992240_usize + 11495898126898743346_usize;
place!(Field::<usize>(Variant(_7.fld5, 3), 2)) = _7.fld4 | _7.fld4;
_7.fld2 = [159_u16,12100_u16,56386_u16,43733_u16,61751_u16,1671_u16,44565_u16,59195_u16];
Call(_7.fld1 = fn16(Field::<usize>(Variant(_7.fld5, 3), 2), _7.fld6, _7.fld2, Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7.fld6 = 7560907826023019157_i64 >> Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).1;
place!(Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3)) = (113385595_u32, 13201935347575775442_u64, _5);
_7.fld0 = true;
Goto(bb3)
}
bb3 = {
_2 = _4;
place!(Field::<i16>(Variant(_7.fld5, 3), 4)) = RET ^ RET;
_5 = -Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).2;
_5 = Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).2 - Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).2;
_7.fld4 = Field::<usize>(Variant(_7.fld5, 3), 2);
_9 = _7.fld4 != Field::<usize>(Variant(_7.fld5, 3), 2);
place!(Field::<i32>(Variant(_7.fld5, 3), 5)) = !452437787_i32;
_8 = _7.fld1 as u8;
RET = !Field::<i16>(Variant(_7.fld5, 3), 4);
place!(Field::<i32>(Variant(_7.fld5, 3), 5)) = 1209210442_i32 | 580380944_i32;
_4 = _1;
Goto(bb4)
}
bb4 = {
place!(Field::<usize>(Variant(_7.fld5, 3), 2)) = _7.fld4;
_12 = _7.fld4 > _7.fld4;
place!(Field::<char>(Variant(_7.fld5, 3), 1)) = _7.fld1;
_13 = [_8,_8,_8,_8];
_13 = [_8,_8,_8,_8];
_11 = 332638535866867593651584580140629515870_u128 as f64;
_10 = Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).2;
_11 = RET as f64;
match Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).1 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
13201935347575775442 => bb11,
_ => bb10
}
}
bb5 = {
_2 = _4;
place!(Field::<i16>(Variant(_7.fld5, 3), 4)) = RET ^ RET;
_5 = -Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).2;
_5 = Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).2 - Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).2;
_7.fld4 = Field::<usize>(Variant(_7.fld5, 3), 2);
_9 = _7.fld4 != Field::<usize>(Variant(_7.fld5, 3), 2);
place!(Field::<i32>(Variant(_7.fld5, 3), 5)) = !452437787_i32;
_8 = _7.fld1 as u8;
RET = !Field::<i16>(Variant(_7.fld5, 3), 4);
place!(Field::<i32>(Variant(_7.fld5, 3), 5)) = 1209210442_i32 | 580380944_i32;
_4 = _1;
Goto(bb4)
}
bb6 = {
_7.fld6 = 7560907826023019157_i64 >> Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).1;
place!(Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3)) = (113385595_u32, 13201935347575775442_u64, _5);
_7.fld0 = true;
Goto(bb3)
}
bb7 = {
place!(Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3)) = (215738932_u32, 8436157484517588467_u64, _5);
place!(Field::<i16>(Variant(_7.fld5, 3), 4)) = RET;
_7.fld1 = '\u{fd293}';
_3 = [(-912696642_i32),712350528_i32,(-1126182763_i32)];
_7.fld6 = (-290987573832998541_i64);
place!(Field::<i32>(Variant(_7.fld5, 3), 5)) = (-100788572132003351342444599316365307995_i128) as i32;
_7.fld4 = 8268574917817992240_usize + 11495898126898743346_usize;
place!(Field::<usize>(Variant(_7.fld5, 3), 2)) = _7.fld4 | _7.fld4;
_7.fld2 = [159_u16,12100_u16,56386_u16,43733_u16,61751_u16,1671_u16,44565_u16,59195_u16];
Call(_7.fld1 = fn16(Field::<usize>(Variant(_7.fld5, 3), 2), _7.fld6, _7.fld2, Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).2), ReturnTo(bb2), UnwindUnreachable())
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
_7.fld6 = _8 as i64;
place!(Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3)).2 = _5 * _5;
_6 = _2;
_1 = [Field::<i32>(Variant(_7.fld5, 3), 5),Field::<i32>(Variant(_7.fld5, 3), 5),Field::<i32>(Variant(_7.fld5, 3), 5)];
_7.fld6 = 4040720398872897956_i64;
_4 = [Field::<i32>(Variant(_7.fld5, 3), 5),Field::<i32>(Variant(_7.fld5, 3), 5),Field::<i32>(Variant(_7.fld5, 3), 5)];
place!(Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3)).2 = 30079_u16 as f32;
_7.fld6 = 3304165095362513864_i64 << Field::<usize>(Variant(_7.fld5, 3), 2);
place!(Field::<i16>(Variant(_7.fld5, 3), 4)) = !RET;
place!(Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3)).0 = !483892552_u32;
_5 = (-123_i8) as f32;
place!(Field::<i16>(Variant(_7.fld5, 3), 4)) = _10 as i16;
_17 = _7.fld6 & _7.fld6;
_16 = -_11;
_7.fld4 = 9223372036854775807_isize as usize;
_17 = 39759601974265886145244465796912687172_i128 as i64;
RET = Field::<i16>(Variant(_7.fld5, 3), 4);
_18 = [Field::<i32>(Variant(_7.fld5, 3), 5)];
_11 = _16;
_4 = _1;
_15 = [Field::<usize>(Variant(_7.fld5, 3), 2),Field::<usize>(Variant(_7.fld5, 3), 2),Field::<usize>(Variant(_7.fld5, 3), 2),Field::<usize>(Variant(_7.fld5, 3), 2),_7.fld4,Field::<usize>(Variant(_7.fld5, 3), 2),Field::<usize>(Variant(_7.fld5, 3), 2),Field::<usize>(Variant(_7.fld5, 3), 2)];
_6 = [Field::<i32>(Variant(_7.fld5, 3), 5),Field::<i32>(Variant(_7.fld5, 3), 5),Field::<i32>(Variant(_7.fld5, 3), 5)];
Goto(bb12)
}
bb12 = {
place!(Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3)) = (1569921090_u32, 13048619933388555596_u64, _5);
_13 = [_8,_8,_8,_8];
place!(Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3)).0 = 1061243240_u32 - 1516968388_u32;
_7.fld1 = Field::<char>(Variant(_7.fld5, 3), 1);
_18 = [Field::<i32>(Variant(_7.fld5, 3), 5)];
_14 = &_8;
place!(Field::<i16>(Variant(_7.fld5, 3), 4)) = -RET;
_2 = _3;
_20 = 31_isize ^ 9223372036854775807_isize;
_7.fld4 = Field::<usize>(Variant(_7.fld5, 3), 2);
_5 = Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).2 * _10;
_7.fld4 = Field::<usize>(Variant(_7.fld5, 3), 2);
_15 = [Field::<usize>(Variant(_7.fld5, 3), 2),_7.fld4,Field::<usize>(Variant(_7.fld5, 3), 2),Field::<usize>(Variant(_7.fld5, 3), 2),_7.fld4,Field::<usize>(Variant(_7.fld5, 3), 2),_7.fld4,_7.fld4];
_15 = [Field::<usize>(Variant(_7.fld5, 3), 2),_7.fld4,Field::<usize>(Variant(_7.fld5, 3), 2),_7.fld4,_7.fld4,_7.fld4,_7.fld4,_7.fld4];
RET = -Field::<i16>(Variant(_7.fld5, 3), 4);
match Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).1 {
0 => bb3,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
13048619933388555596 => bb18,
_ => bb17
}
}
bb13 = {
_7.fld6 = _8 as i64;
place!(Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3)).2 = _5 * _5;
_6 = _2;
_1 = [Field::<i32>(Variant(_7.fld5, 3), 5),Field::<i32>(Variant(_7.fld5, 3), 5),Field::<i32>(Variant(_7.fld5, 3), 5)];
_7.fld6 = 4040720398872897956_i64;
_4 = [Field::<i32>(Variant(_7.fld5, 3), 5),Field::<i32>(Variant(_7.fld5, 3), 5),Field::<i32>(Variant(_7.fld5, 3), 5)];
place!(Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3)).2 = 30079_u16 as f32;
_7.fld6 = 3304165095362513864_i64 << Field::<usize>(Variant(_7.fld5, 3), 2);
place!(Field::<i16>(Variant(_7.fld5, 3), 4)) = !RET;
place!(Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3)).0 = !483892552_u32;
_5 = (-123_i8) as f32;
place!(Field::<i16>(Variant(_7.fld5, 3), 4)) = _10 as i16;
_17 = _7.fld6 & _7.fld6;
_16 = -_11;
_7.fld4 = 9223372036854775807_isize as usize;
_17 = 39759601974265886145244465796912687172_i128 as i64;
RET = Field::<i16>(Variant(_7.fld5, 3), 4);
_18 = [Field::<i32>(Variant(_7.fld5, 3), 5)];
_11 = _16;
_4 = _1;
_15 = [Field::<usize>(Variant(_7.fld5, 3), 2),Field::<usize>(Variant(_7.fld5, 3), 2),Field::<usize>(Variant(_7.fld5, 3), 2),Field::<usize>(Variant(_7.fld5, 3), 2),_7.fld4,Field::<usize>(Variant(_7.fld5, 3), 2),Field::<usize>(Variant(_7.fld5, 3), 2),Field::<usize>(Variant(_7.fld5, 3), 2)];
_6 = [Field::<i32>(Variant(_7.fld5, 3), 5),Field::<i32>(Variant(_7.fld5, 3), 5),Field::<i32>(Variant(_7.fld5, 3), 5)];
Goto(bb12)
}
bb14 = {
Return()
}
bb15 = {
place!(Field::<usize>(Variant(_7.fld5, 3), 2)) = _7.fld4;
_12 = _7.fld4 > _7.fld4;
place!(Field::<char>(Variant(_7.fld5, 3), 1)) = _7.fld1;
_13 = [_8,_8,_8,_8];
_13 = [_8,_8,_8,_8];
_11 = 332638535866867593651584580140629515870_u128 as f64;
_10 = Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).2;
_11 = RET as f64;
match Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).1 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
13201935347575775442 => bb11,
_ => bb10
}
}
bb16 = {
_2 = _4;
place!(Field::<i16>(Variant(_7.fld5, 3), 4)) = RET ^ RET;
_5 = -Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).2;
_5 = Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).2 - Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).2;
_7.fld4 = Field::<usize>(Variant(_7.fld5, 3), 2);
_9 = _7.fld4 != Field::<usize>(Variant(_7.fld5, 3), 2);
place!(Field::<i32>(Variant(_7.fld5, 3), 5)) = !452437787_i32;
_8 = _7.fld1 as u8;
RET = !Field::<i16>(Variant(_7.fld5, 3), 4);
place!(Field::<i32>(Variant(_7.fld5, 3), 5)) = 1209210442_i32 | 580380944_i32;
_4 = _1;
Goto(bb4)
}
bb17 = {
_7.fld6 = 7560907826023019157_i64 >> Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).1;
place!(Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3)) = (113385595_u32, 13201935347575775442_u64, _5);
_7.fld0 = true;
Goto(bb3)
}
bb18 = {
_19 = [Field::<i32>(Variant(_7.fld5, 3), 5)];
place!(Field::<f32>(Variant(_7.fld5, 3), 0)) = (*_14) as f32;
place!(Field::<f32>(Variant(_7.fld5, 3), 0)) = _10;
_1 = _3;
_18 = _19;
_17 = 142241198024855490761634159688642439780_i128 as i64;
_21 = Field::<i32>(Variant(_7.fld5, 3), 5) + Field::<i32>(Variant(_7.fld5, 3), 5);
_7.fld3 = core::ptr::addr_of!(_20);
place!(Field::<i16>(Variant(_7.fld5, 3), 4)) = (-35397166443889475061837156293283900705_i128) as i16;
_10 = Field::<(u32, u64, f32)>(Variant(_7.fld5, 3), 3).2;
_25 = _20 ^ _20;
_21 = Field::<i32>(Variant(_7.fld5, 3), 5) & Field::<i32>(Variant(_7.fld5, 3), 5);
Goto(bb19)
}
bb19 = {
Call(_27 = dump_var(15_usize, 9_usize, Move(_9), 3_usize, Move(_3), 18_usize, Move(_18), 12_usize, Move(_12)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_27 = dump_var(15_usize, 19_usize, Move(_19), 1_usize, Move(_1), 15_usize, Move(_15), 21_usize, Move(_21)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: usize,mut _2: i64,mut _3: [u16; 8],mut _4: f32) -> char {
mir! {
type RET = char;
let _5: *const char;
let _6: &'static (f32, Adt50, Adt34);
let _7: char;
let _8: [usize; 8];
let _9: ([u128; 1],);
let _10: [u8; 4];
let _11: f64;
let _12: [usize; 8];
let _13: [i32; 3];
let _14: Adt71;
let _15: i128;
let _16: ([i32; 3],);
let _17: i128;
let _18: (Adt24, *const *const isize, u16, *const u8);
let _19: &'static bool;
let _20: f32;
let _21: i128;
let _22: *const isize;
let _23: *mut char;
let _24: isize;
let _25: *mut bool;
let _26: bool;
let _27: (*const isize, u128, Adt63);
let _28: *const Adt24;
let _29: (u32, u64, f32);
let _30: (*const isize, u128, Adt63);
let _31: usize;
let _32: &'static Adt34;
let _33: [isize; 6];
let _34: isize;
let _35: [i16; 5];
let _36: f64;
let _37: u64;
let _38: [u16; 8];
let _39: (&'static u32, &'static Adt34);
let _40: ();
let _41: ();
{
_1 = 7_usize;
_3 = [64724_u16,64880_u16,3175_u16,37559_u16,54048_u16,28792_u16,24958_u16,42301_u16];
_3[_1] = '\u{3c0ae}' as u16;
RET = '\u{bb992}';
RET = '\u{73ea9}';
_1 = 16508291848712792255_usize * 4_usize;
_5 = core::ptr::addr_of!(RET);
RET = '\u{54dd7}';
_5 = core::ptr::addr_of!((*_5));
_1 = 3_usize;
_1 = 18359929406346663058_usize >> _3[_1];
_1 = (-16044_i16) as usize;
_4 = (-9223372036854775808_isize) as f32;
(*_5) = '\u{95baa}';
_1 = 5043707032286372531_usize & 4_usize;
_4 = 537608356_u32 as f32;
_4 = 35372_u16 as f32;
_1 = 2_usize & 7314335971175465473_usize;
_2 = !(-3002154812929927949_i64);
(*_5) = '\u{8da00}';
_1 = !0_usize;
_3 = [45603_u16,48052_u16,4415_u16,51496_u16,59426_u16,27429_u16,6847_u16,12298_u16];
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
_2 = (-1679554741_i32) as i64;
_7 = (*_5);
Goto(bb1)
}
bb1 = {
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
_2 = !4251610246442731541_i64;
_3 = [21165_u16,27681_u16,53577_u16,43223_u16,44945_u16,52816_u16,51848_u16,46694_u16];
(*_5) = _7;
(*_5) = _7;
_7 = RET;
(*_5) = _7;
_3 = [10213_u16,31445_u16,23423_u16,33084_u16,55118_u16,42541_u16,64421_u16,20238_u16];
_4 = 6922_i16 as f32;
(*_5) = _7;
RET = _7;
_3 = [13783_u16,45683_u16,18825_u16,51649_u16,58132_u16,53068_u16,52281_u16,58509_u16];
_2 = (-1868745311117545657_i64);
RET = _7;
_11 = 300640534219565416177693096670043258257_u128 as f64;
_9.0 = [223426231112642418543636846247113636124_u128];
_10 = [1_u8,35_u8,84_u8,72_u8];
_10 = [141_u8,200_u8,49_u8,115_u8];
Goto(bb2)
}
bb2 = {
_3 = [37020_u16,950_u16,32520_u16,28515_u16,50486_u16,62459_u16,1707_u16,51748_u16];
_2 = !2912706882062859118_i64;
_11 = (-10199_i16) as f64;
_1 = 5_usize;
RET = _7;
_2 = -6953579204855299595_i64;
_5 = core::ptr::addr_of!((*_5));
_1 = _8[_1];
_11 = _2 as f64;
Goto(bb3)
}
bb3 = {
_10 = [113_u8,143_u8,7_u8,19_u8];
_9.0 = [268767455215335169584184158359621006767_u128];
(*_5) = _7;
_7 = RET;
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
RET = _7;
_2 = -7605453515749900884_i64;
_7 = RET;
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
_11 = 13814014098294383854_u64 as f64;
_1 = 5156032761487456708_usize + 1_usize;
_10 = [178_u8,69_u8,229_u8,213_u8];
_8 = [_1,_1,_1,_1,_1,_1,_1,_1];
_2 = 251_u8 as i64;
_12 = [_1,_1,_1,_1,_1,_1,_1,_1];
(*_5) = _7;
_11 = 2047481987_u32 as f64;
_2 = 4367166897054135016_i64 & (-907855847261366671_i64);
_7 = (*_5);
(*_5) = _7;
RET = _7;
Goto(bb4)
}
bb4 = {
_1 = !6_usize;
_16.0 = [(-1834925209_i32),(-81578409_i32),(-1815650498_i32)];
(*_5) = _7;
_18.0 = Adt24::Variant1 { fld0: 14299653434956803359_u64,fld1: RET,fld2: (-41_isize),fld3: 3834850524_u32,fld4: (-673_i16),fld5: _1,fld6: 171504873787065120634231390549093889453_u128,fld7: (-11532380057664217815320483902943088040_i128) };
_3 = [36183_u16,34402_u16,63682_u16,35619_u16,10271_u16,59349_u16,21472_u16,6226_u16];
place!(Field::<i16>(Variant(_18.0, 1), 4)) = -17043_i16;
place!(Field::<usize>(Variant(_18.0, 1), 5)) = _1;
place!(Field::<i128>(Variant(_18.0, 1), 7)) = -(-12054093516761536028235732109075573890_i128);
_15 = 14184773525252743549_u64 as i128;
_18.2 = 17621_u16 << Field::<i128>(Variant(_18.0, 1), 7);
RET = _7;
_13 = [1284769749_i32,47720150_i32,1005275658_i32];
RET = _7;
_13 = _16.0;
RET = Field::<char>(Variant(_18.0, 1), 1);
_1 = Field::<usize>(Variant(_18.0, 1), 5) + Field::<usize>(Variant(_18.0, 1), 5);
_10 = [160_u8,166_u8,102_u8,38_u8];
_21 = _15;
RET = Field::<char>(Variant(_18.0, 1), 1);
_20 = -_4;
_21 = _15;
_5 = core::ptr::addr_of!(place!(Field::<char>(Variant(_18.0, 1), 1)));
Goto(bb5)
}
bb5 = {
_4 = _20;
_1 = !Field::<usize>(Variant(_18.0, 1), 5);
place!(Field::<u64>(Variant(_18.0, 1), 0)) = !5622546438773022268_u64;
_12 = _8;
place!(Field::<u32>(Variant(_18.0, 1), 3)) = 110612771032377573329744137648823378859_u128 as u32;
_2 = (-2385913934889936201_i64) << _1;
_11 = _20 as f64;
_9.0 = [378955728497477560140224435272903004_u128];
_17 = Field::<u64>(Variant(_18.0, 1), 0) as i128;
place!(Field::<usize>(Variant(_18.0, 1), 5)) = _1 >> _17;
place!(Field::<u128>(Variant(_18.0, 1), 6)) = 141298923149586214768873837635474521022_u128;
place!(Field::<u128>(Variant(_18.0, 1), 6)) = 154306331810323836089684127904036502371_u128 + 313135616009702641435486296216720921498_u128;
(*_5) = _7;
_18.1 = core::ptr::addr_of!(_22);
Goto(bb6)
}
bb6 = {
place!(Field::<isize>(Variant(_18.0, 1), 2)) = !9223372036854775807_isize;
_23 = core::ptr::addr_of_mut!((*_5));
_18.2 = 12526_u16;
_12 = _8;
_20 = _4;
_18.1 = core::ptr::addr_of!(_22);
SetDiscriminant(_18.0, 1);
_18.1 = core::ptr::addr_of!(_22);
_3 = [_18.2,_18.2,_18.2,_18.2,_18.2,_18.2,_18.2,_18.2];
_22 = core::ptr::addr_of!(_24);
(*_22) = !(-9223372036854775808_isize);
Goto(bb7)
}
bb7 = {
RET = _7;
_21 = !_15;
Goto(bb8)
}
bb8 = {
place!(Field::<u64>(Variant(_18.0, 1), 0)) = _24 as u64;
place!(Field::<i128>(Variant(_18.0, 1), 7)) = false as i128;
_3 = [_18.2,_18.2,_18.2,_18.2,_18.2,_18.2,_18.2,_18.2];
place!(Field::<i128>(Variant(_18.0, 1), 7)) = -_15;
_4 = 54_i8 as f32;
(*_22) = Field::<u64>(Variant(_18.0, 1), 0) as isize;
_24 = 30_isize | (-47_isize);
place!(Field::<u128>(Variant(_18.0, 1), 6)) = 30338_i16 as u128;
_18.1 = core::ptr::addr_of!(_22);
_24 = (-9223372036854775808_isize) + (-73_isize);
_24 = 1885507969_u32 as isize;
Goto(bb9)
}
bb9 = {
_11 = Field::<u128>(Variant(_18.0, 1), 6) as f64;
place!(Field::<u128>(Variant(_18.0, 1), 6)) = !11422176065802472355282755696599956330_u128;
place!(Field::<isize>(Variant(_18.0, 1), 2)) = -_24;
place!(Field::<usize>(Variant(_18.0, 1), 5)) = Field::<u128>(Variant(_18.0, 1), 6) as usize;
_3 = [_18.2,_18.2,_18.2,_18.2,_18.2,_18.2,_18.2,_18.2];
_4 = _20;
place!(Field::<u128>(Variant(_18.0, 1), 6)) = !62031777480168911497123541511952532885_u128;
_20 = _4;
_13 = _16.0;
place!(Field::<char>(Variant(_18.0, 1), 1)) = _7;
_11 = Field::<usize>(Variant(_18.0, 1), 5) as f64;
place!(Field::<isize>(Variant(_18.0, 1), 2)) = (*_22) >> _18.2;
_1 = !Field::<usize>(Variant(_18.0, 1), 5);
place!(Field::<char>(Variant(_18.0, 1), 1)) = RET;
_8 = [_1,Field::<usize>(Variant(_18.0, 1), 5),Field::<usize>(Variant(_18.0, 1), 5),_1,Field::<usize>(Variant(_18.0, 1), 5),_1,Field::<usize>(Variant(_18.0, 1), 5),_1];
_15 = _21;
(*_22) = -Field::<isize>(Variant(_18.0, 1), 2);
place!(Field::<u128>(Variant(_18.0, 1), 6)) = _11 as u128;
_16.0 = [612409885_i32,(-2131055107_i32),1299618965_i32];
_15 = -_17;
_24 = Field::<isize>(Variant(_18.0, 1), 2) >> _15;
_18.0 = Adt24::Variant0 { fld0: 68504175_i32 };
_18.2 = (-1464357227_i32) as u16;
_27.0 = core::ptr::addr_of!((*_22));
_19 = &_26;
_25 = core::ptr::addr_of_mut!(_26);
Goto(bb10)
}
bb10 = {
_27.1 = 19_u8 as u128;
_24 = (-9223372036854775808_isize) << _27.1;
place!(Field::<i32>(Variant(_18.0, 0), 0)) = -(-2014102957_i32);
_27.1 = 215054752437634209263167883842448623422_u128 << (*_22);
_13 = _16.0;
_16.0 = [Field::<i32>(Variant(_18.0, 0), 0),Field::<i32>(Variant(_18.0, 0), 0),Field::<i32>(Variant(_18.0, 0), 0)];
_4 = _20;
_12 = _8;
_27.0 = core::ptr::addr_of!((*_22));
_29.0 = 215378119_u32;
SetDiscriminant(_18.0, 1);
_29.1 = 149104104793937587_u64;
_19 = &(*_25);
place!(Field::<u64>(Variant(_18.0, 1), 0)) = _29.1;
place!(Field::<isize>(Variant(_18.0, 1), 2)) = _24;
_26 = true;
_13 = _16.0;
_29.2 = _29.0 as f32;
_17 = -_15;
_22 = Move(_27.0);
_4 = -_20;
_11 = _27.1 as f64;
_22 = core::ptr::addr_of!(_24);
match Field::<u64>(Variant(_18.0, 1), 0) {
149104104793937587 => bb11,
_ => bb1
}
}
bb11 = {
(*_22) = -Field::<isize>(Variant(_18.0, 1), 2);
place!(Field::<i128>(Variant(_18.0, 1), 7)) = _21 - _15;
_28 = core::ptr::addr_of!(_18.0);
place!(Field::<char>(Variant(_18.0, 1), 1)) = _7;
place!(Field::<i16>(Variant(_18.0, 1), 4)) = (-20682_i16);
_18.1 = core::ptr::addr_of!(_27.0);
place!(Field::<usize>(Variant((*_28), 1), 5)) = _29.0 as usize;
place!(Field::<u128>(Variant((*_28), 1), 6)) = _27.1;
place!(Field::<u128>(Variant(_18.0, 1), 6)) = (-38_i8) as u128;
_24 = _26 as isize;
place!(Field::<u64>(Variant((*_28), 1), 0)) = _29.1 | _29.1;
_18.1 = core::ptr::addr_of!(_22);
_27.0 = Move(_22);
place!(Field::<i16>(Variant((*_28), 1), 4)) = Field::<isize>(Variant((*_28), 1), 2) as i16;
place!(Field::<u128>(Variant(_18.0, 1), 6)) = !_27.1;
_34 = Field::<isize>(Variant(_18.0, 1), 2) | Field::<isize>(Variant((*_28), 1), 2);
place!(Field::<u32>(Variant((*_28), 1), 3)) = Field::<u64>(Variant((*_28), 1), 0) as u32;
Goto(bb12)
}
bb12 = {
place!(Field::<isize>(Variant((*_28), 1), 2)) = _34;
_31 = !Field::<usize>(Variant((*_28), 1), 5);
_22 = Move(_27.0);
_35 = [Field::<i16>(Variant((*_28), 1), 4),Field::<i16>(Variant(_18.0, 1), 4),Field::<i16>(Variant((*_28), 1), 4),Field::<i16>(Variant((*_28), 1), 4),Field::<i16>(Variant((*_28), 1), 4)];
place!(Field::<char>(Variant((*_28), 1), 1)) = _7;
RET = Field::<char>(Variant((*_28), 1), 1);
place!(Field::<i16>(Variant((*_28), 1), 4)) = 31607_i16 - 20087_i16;
place!(Field::<u32>(Variant((*_28), 1), 3)) = _29.0;
_16 = (_13,);
SetDiscriminant((*_28), 1);
_18.0 = Adt24::Variant0 { fld0: (-1902424579_i32) };
_33 = [_34,_34,_34,_34,_24,_34];
_20 = -_4;
_3 = [_18.2,_18.2,_18.2,_18.2,_18.2,_18.2,_18.2,_18.2];
_29.2 = _20;
_35 = [15004_i16,1262_i16,(-14541_i16),19918_i16,30118_i16];
_13 = _16.0;
_30.1 = _26 as u128;
(*_28) = Adt24::Variant3 { fld0: _4,fld1: _7,fld2: _1,fld3: _29,fld4: 5087_i16,fld5: (-1731035162_i32) };
place!(Field::<(u32, u64, f32)>(Variant(_18.0, 3), 3)) = (_29.0, _29.1, Field::<f32>(Variant((*_28), 3), 0));
_18.1 = core::ptr::addr_of!(_27.0);
_2 = 9107787564557029243_i64 ^ (-9180868437336178911_i64);
_13 = _16.0;
_30.0 = core::ptr::addr_of!(_34);
match Field::<(u32, u64, f32)>(Variant((*_28), 3), 3).1 {
0 => bb13,
149104104793937587 => bb15,
_ => bb14
}
}
bb13 = {
RET = _7;
_21 = !_15;
Goto(bb8)
}
bb14 = {
_4 = _20;
_1 = !Field::<usize>(Variant(_18.0, 1), 5);
place!(Field::<u64>(Variant(_18.0, 1), 0)) = !5622546438773022268_u64;
_12 = _8;
place!(Field::<u32>(Variant(_18.0, 1), 3)) = 110612771032377573329744137648823378859_u128 as u32;
_2 = (-2385913934889936201_i64) << _1;
_11 = _20 as f64;
_9.0 = [378955728497477560140224435272903004_u128];
_17 = Field::<u64>(Variant(_18.0, 1), 0) as i128;
place!(Field::<usize>(Variant(_18.0, 1), 5)) = _1 >> _17;
place!(Field::<u128>(Variant(_18.0, 1), 6)) = 141298923149586214768873837635474521022_u128;
place!(Field::<u128>(Variant(_18.0, 1), 6)) = 154306331810323836089684127904036502371_u128 + 313135616009702641435486296216720921498_u128;
(*_5) = _7;
_18.1 = core::ptr::addr_of!(_22);
Goto(bb6)
}
bb15 = {
_24 = _34 ^ _34;
place!(Field::<(u32, u64, f32)>(Variant(_18.0, 3), 3)).1 = !_29.1;
place!(Field::<usize>(Variant(_18.0, 3), 2)) = _1;
RET = Field::<char>(Variant(_18.0, 3), 1);
_28 = core::ptr::addr_of!((*_28));
Goto(bb16)
}
bb16 = {
Call(_40 = dump_var(16_usize, 9_usize, Move(_9), 31_usize, Move(_31), 24_usize, Move(_24), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(16_usize, 3_usize, Move(_3), 12_usize, Move(_12), 1_usize, Move(_1), 26_usize, Move(_26)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(16_usize, 2_usize, Move(_2), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: u64,mut _2: i16,mut _3: [i32; 1],mut _4: i16,mut _5: f32,mut _6: f32,mut _7: f64,mut _8: f32) -> u32 {
mir! {
type RET = u32;
let _9: (*const u8,);
let _10: Adt66;
let _11: *mut u64;
let _12: isize;
let _13: &'static [u8; 4];
let _14: *const char;
let _15: [isize; 6];
let _16: [bool; 4];
let _17: i16;
let _18: isize;
let _19: char;
let _20: isize;
let _21: *mut [u128; 1];
let _22: Adt66;
let _23: (*const *const isize, *mut *const u8, [u16; 8], i16);
let _24: ([i32; 3],);
let _25: isize;
let _26: u8;
let _27: isize;
let _28: [bool; 4];
let _29: Adt24;
let _30: i8;
let _31: char;
let _32: [i16; 5];
let _33: isize;
let _34: u16;
let _35: *const char;
let _36: *const char;
let _37: u8;
let _38: char;
let _39: ();
let _40: ();
{
RET = 2229014597_u32 + 1098953515_u32;
RET = !1803793487_u32;
RET = 589943559_u32;
_3 = [1976993850_i32];
RET = 338254114_u32 ^ 3951350625_u32;
_6 = _8 - _5;
_4 = _2 >> RET;
RET = 2706487850_u32 + 2682586271_u32;
_8 = _6;
_5 = -_6;
_7 = _1 as f64;
RET = 27725_u16 as u32;
_2 = !_4;
_2 = _4;
_8 = -_5;
_8 = _5;
_4 = _2 >> _2;
RET = 2619815132_u32 + 3855836176_u32;
Goto(bb1)
}
bb1 = {
_3 = [(-1105244276_i32)];
_2 = (-5585036613563415646_i64) as i16;
_4 = 225_u8 as i16;
_3 = [(-369555305_i32)];
_6 = _1 as f32;
RET = 3880170819_u32 >> _1;
_4 = !_2;
_5 = (-1428299516_i32) as f32;
_11 = core::ptr::addr_of_mut!(_1);
Goto(bb2)
}
bb2 = {
_1 = 18210296194052844290_u64 * 9861925921102338389_u64;
_5 = -_8;
(*_11) = 11643100902569278623_u64 >> _4;
RET = (-910617118_i32) as u32;
_10 = Adt66::Variant2 { fld0: 212_u8 };
_6 = _5 - _5;
RET = 108_i8 as u32;
place!(Field::<u8>(Variant(_10, 2), 0)) = 149_u8 | 27_u8;
place!(Field::<u8>(Variant(_10, 2), 0)) = _2 as u8;
_12 = _4 as isize;
_15 = [_12,_12,_12,_12,_12,_12];
_1 = (-838840024415527943_i64) as u64;
_6 = _2 as f32;
_9.0 = core::ptr::addr_of!(place!(Field::<u8>(Variant(_10, 2), 0)));
_7 = 23677_u16 as f64;
_18 = 17292_u16 as isize;
_15 = [_18,_12,_18,_12,_18,_12];
_12 = (-6108999841887105548_i64) as isize;
place!(Field::<u8>(Variant(_10, 2), 0)) = 7_u8;
_8 = -_5;
_8 = _5;
Goto(bb3)
}
bb3 = {
_7 = Field::<u8>(Variant(_10, 2), 0) as f64;
_16 = [false,false,true,true];
_8 = _5;
_9.0 = core::ptr::addr_of!(place!(Field::<u8>(Variant(_10, 2), 0)));
_7 = (-1063963521_i32) as f64;
RET = _7 as u32;
_14 = core::ptr::addr_of!(_19);
_12 = _18;
_5 = -_8;
_20 = -_18;
_3 = [1955060859_i32];
_6 = _8;
_17 = !_4;
_15 = [_12,_12,_20,_12,_20,_20];
SetDiscriminant(_10, 2);
_9.0 = core::ptr::addr_of!(place!(Field::<u8>(Variant(_10, 2), 0)));
place!(Field::<u8>(Variant(_10, 2), 0)) = !28_u8;
_8 = -_6;
_22 = Move(_10);
_23.2 = [15263_u16,18527_u16,21032_u16,56671_u16,36393_u16,53275_u16,63578_u16,39794_u16];
_8 = _6 - _5;
place!(Field::<u8>(Variant(_22, 2), 0)) = 226_u8;
(*_11) = 14174118363487507419_u64 << _18;
_5 = -_8;
Goto(bb4)
}
bb4 = {
_1 = !9181225654142066700_u64;
_11 = core::ptr::addr_of_mut!(_1);
_25 = _12;
(*_14) = '\u{3b9}';
_26 = Field::<u8>(Variant(_22, 2), 0);
_20 = 16572284247025965723_usize as isize;
_7 = (-3026724346740020487_i64) as f64;
_3 = [(-636571024_i32)];
_23.2 = [22994_u16,43197_u16,62969_u16,28464_u16,22139_u16,15606_u16,20939_u16,29746_u16];
_26 = !Field::<u8>(Variant(_22, 2), 0);
SetDiscriminant(_22, 2);
Goto(bb5)
}
bb5 = {
_22 = Adt66::Variant2 { fld0: _26 };
_15 = [_25,_12,_12,_20,_18,_20];
_26 = Field::<u8>(Variant(_22, 2), 0);
_23.2 = [40027_u16,58767_u16,45674_u16,46713_u16,763_u16,345_u16,61419_u16,31741_u16];
(*_11) = 16947949871329813689_u64;
_15 = [_12,_20,_20,_18,_20,_12];
_23.1 = core::ptr::addr_of_mut!(_9.0);
_23.2 = [21674_u16,14816_u16,12185_u16,14248_u16,40111_u16,6160_u16,55698_u16,54274_u16];
_23.2 = [11692_u16,58888_u16,40642_u16,60268_u16,19137_u16,34647_u16,17423_u16,33561_u16];
(*_14) = '\u{fdc5b}';
SetDiscriminant(_22, 1);
place!(Field::<*mut char>(Variant(_22, 1), 0)) = core::ptr::addr_of_mut!((*_14));
match (*_11) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
16947949871329813689 => bb9,
_ => bb8
}
}
bb6 = {
_1 = !9181225654142066700_u64;
_11 = core::ptr::addr_of_mut!(_1);
_25 = _12;
(*_14) = '\u{3b9}';
_26 = Field::<u8>(Variant(_22, 2), 0);
_20 = 16572284247025965723_usize as isize;
_7 = (-3026724346740020487_i64) as f64;
_3 = [(-636571024_i32)];
_23.2 = [22994_u16,43197_u16,62969_u16,28464_u16,22139_u16,15606_u16,20939_u16,29746_u16];
_26 = !Field::<u8>(Variant(_22, 2), 0);
SetDiscriminant(_22, 2);
Goto(bb5)
}
bb7 = {
_7 = Field::<u8>(Variant(_10, 2), 0) as f64;
_16 = [false,false,true,true];
_8 = _5;
_9.0 = core::ptr::addr_of!(place!(Field::<u8>(Variant(_10, 2), 0)));
_7 = (-1063963521_i32) as f64;
RET = _7 as u32;
_14 = core::ptr::addr_of!(_19);
_12 = _18;
_5 = -_8;
_20 = -_18;
_3 = [1955060859_i32];
_6 = _8;
_17 = !_4;
_15 = [_12,_12,_20,_12,_20,_20];
SetDiscriminant(_10, 2);
_9.0 = core::ptr::addr_of!(place!(Field::<u8>(Variant(_10, 2), 0)));
place!(Field::<u8>(Variant(_10, 2), 0)) = !28_u8;
_8 = -_6;
_22 = Move(_10);
_23.2 = [15263_u16,18527_u16,21032_u16,56671_u16,36393_u16,53275_u16,63578_u16,39794_u16];
_8 = _6 - _5;
place!(Field::<u8>(Variant(_22, 2), 0)) = 226_u8;
(*_11) = 14174118363487507419_u64 << _18;
_5 = -_8;
Goto(bb4)
}
bb8 = {
_1 = 18210296194052844290_u64 * 9861925921102338389_u64;
_5 = -_8;
(*_11) = 11643100902569278623_u64 >> _4;
RET = (-910617118_i32) as u32;
_10 = Adt66::Variant2 { fld0: 212_u8 };
_6 = _5 - _5;
RET = 108_i8 as u32;
place!(Field::<u8>(Variant(_10, 2), 0)) = 149_u8 | 27_u8;
place!(Field::<u8>(Variant(_10, 2), 0)) = _2 as u8;
_12 = _4 as isize;
_15 = [_12,_12,_12,_12,_12,_12];
_1 = (-838840024415527943_i64) as u64;
_6 = _2 as f32;
_9.0 = core::ptr::addr_of!(place!(Field::<u8>(Variant(_10, 2), 0)));
_7 = 23677_u16 as f64;
_18 = 17292_u16 as isize;
_15 = [_18,_12,_18,_12,_18,_12];
_12 = (-6108999841887105548_i64) as isize;
place!(Field::<u8>(Variant(_10, 2), 0)) = 7_u8;
_8 = -_5;
_8 = _5;
Goto(bb3)
}
bb9 = {
_12 = !_25;
match (*_11) {
0 => bb1,
1 => bb10,
16947949871329813689 => bb12,
_ => bb11
}
}
bb10 = {
_1 = !9181225654142066700_u64;
_11 = core::ptr::addr_of_mut!(_1);
_25 = _12;
(*_14) = '\u{3b9}';
_26 = Field::<u8>(Variant(_22, 2), 0);
_20 = 16572284247025965723_usize as isize;
_7 = (-3026724346740020487_i64) as f64;
_3 = [(-636571024_i32)];
_23.2 = [22994_u16,43197_u16,62969_u16,28464_u16,22139_u16,15606_u16,20939_u16,29746_u16];
_26 = !Field::<u8>(Variant(_22, 2), 0);
SetDiscriminant(_22, 2);
Goto(bb5)
}
bb11 = {
_7 = Field::<u8>(Variant(_10, 2), 0) as f64;
_16 = [false,false,true,true];
_8 = _5;
_9.0 = core::ptr::addr_of!(place!(Field::<u8>(Variant(_10, 2), 0)));
_7 = (-1063963521_i32) as f64;
RET = _7 as u32;
_14 = core::ptr::addr_of!(_19);
_12 = _18;
_5 = -_8;
_20 = -_18;
_3 = [1955060859_i32];
_6 = _8;
_17 = !_4;
_15 = [_12,_12,_20,_12,_20,_20];
SetDiscriminant(_10, 2);
_9.0 = core::ptr::addr_of!(place!(Field::<u8>(Variant(_10, 2), 0)));
place!(Field::<u8>(Variant(_10, 2), 0)) = !28_u8;
_8 = -_6;
_22 = Move(_10);
_23.2 = [15263_u16,18527_u16,21032_u16,56671_u16,36393_u16,53275_u16,63578_u16,39794_u16];
_8 = _6 - _5;
place!(Field::<u8>(Variant(_22, 2), 0)) = 226_u8;
(*_11) = 14174118363487507419_u64 << _18;
_5 = -_8;
Goto(bb4)
}
bb12 = {
_9.0 = core::ptr::addr_of!(_26);
_19 = '\u{bbdcc}';
RET = 3721772111_u32 | 2344619589_u32;
_23.1 = core::ptr::addr_of_mut!(_9.0);
_1 = _7 as u64;
_23.3 = _17 ^ _17;
_3 = [690119173_i32];
place!(Field::<(*const isize, u128, Adt63)>(Variant(_22, 1), 3)).1 = false as u128;
_20 = _25 | _25;
_28 = _16;
_7 = 15499_u16 as f64;
place!(Field::<*mut u64>(Variant(_22, 1), 1)) = core::ptr::addr_of_mut!((*_11));
place!(Field::<[u16; 4]>(Variant(_22, 1), 4)) = [34503_u16,47442_u16,20004_u16,38545_u16];
_23.1 = core::ptr::addr_of_mut!(_9.0);
(*_11) = RET as u64;
_3 = [(-1362995755_i32)];
_2 = !_17;
_11 = core::ptr::addr_of_mut!((*_11));
_23.1 = core::ptr::addr_of_mut!(_9.0);
Goto(bb13)
}
bb13 = {
_29 = Adt24::Variant0 { fld0: 221437295_i32 };
_29 = Adt24::Variant0 { fld0: 841926147_i32 };
(*_11) = RET as u64;
_28 = [true,true,false,false];
place!(Field::<(*const isize, u128, Adt63)>(Variant(_22, 1), 3)).1 = _23.3 as u128;
Call(_5 = fn18(Move(Field::<*mut char>(Variant(_22, 1), 0)), _8, _8, Field::<(*const isize, u128, Adt63)>(Variant(_22, 1), 3).1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_23.1 = core::ptr::addr_of_mut!(_9.0);
_23.2 = [23803_u16,13747_u16,32620_u16,54856_u16,48359_u16,34116_u16,43346_u16,780_u16];
_7 = 789179498_i32 as f64;
_6 = -_5;
_3 = [1145222076_i32];
RET = 352066685_u32;
_34 = _7 as u16;
_31 = _19;
_9.0 = core::ptr::addr_of!(_26);
_32 = [_4,_23.3,_17,_4,_23.3];
_3 = [1818046314_i32];
RET = (-73275707277587859413528800967442221386_i128) as u32;
_24.0 = [1126376801_i32,(-416923578_i32),(-22817112_i32)];
_35 = Move(_14);
_29 = Adt24::Variant0 { fld0: 2088599329_i32 };
_34 = 17609_u16;
place!(Field::<*mut u64>(Variant(_22, 1), 1)) = core::ptr::addr_of_mut!(_1);
_12 = 22274965117238332315910633945103636826_i128 as isize;
_23.0 = core::ptr::addr_of!(place!(Field::<(*const isize, u128, Adt63)>(Variant(_22, 1), 3)).0);
_23.0 = core::ptr::addr_of!(place!(Field::<(*const isize, u128, Adt63)>(Variant(_22, 1), 3)).0);
_34 = Field::<(*const isize, u128, Adt63)>(Variant(_22, 1), 3).1 as u16;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(17_usize, 31_usize, Move(_31), 26_usize, Move(_26), 17_usize, Move(_17), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(17_usize, 18_usize, Move(_18), 19_usize, Move(_19), 32_usize, Move(_32), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(17_usize, 25_usize, Move(_25), 40_usize, _40, 40_usize, _40, 40_usize, _40), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: *mut char,mut _2: f32,mut _3: f32,mut _4: u128) -> f32 {
mir! {
type RET = f32;
let _5: *mut [u128; 1];
let _6: u32;
let _7: &'static bool;
let _8: [i16; 5];
let _9: (*mut *const u8, (*const *const isize, *mut *const u8, [u16; 8], i16), [u128; 1]);
let _10: usize;
let _11: [u64; 4];
let _12: [i16; 5];
let _13: [isize; 6];
let _14: [i8; 7];
let _15: *const isize;
let _16: [usize; 8];
let _17: ();
let _18: ();
{
RET = 3692276965_u32 as f32;
RET = 1870061201_i32 as f32;
_2 = 874930802_u32 as f32;
_3 = -RET;
_3 = RET;
_2 = 38_i8 as f32;
_6 = !3000078552_u32;
_3 = _2;
_6 = 533824685_u32 * 3626720906_u32;
RET = 6645753888854014138_usize as f32;
_3 = -RET;
RET = 16618_i16 as f32;
_2 = _3;
_6 = 3133707271_u32 * 2194324869_u32;
_6 = 1498255277_u32;
_3 = -RET;
RET = -_3;
Goto(bb1)
}
bb1 = {
_3 = _2;
_2 = -RET;
Goto(bb2)
}
bb2 = {
_3 = RET - _2;
RET = _3 + _3;
_4 = 292316551269745310768152195033686996410_u128 - 57398327144375639841234495317995417996_u128;
_3 = RET;
_4 = 219861681237357940489020409114803093296_u128 << _6;
RET = _3 + _2;
_4 = 35445673376334310917970240332335254120_u128;
Goto(bb3)
}
bb3 = {
_4 = 259978602957387446748800547776609705998_u128 << _6;
RET = _3;
_6 = 31530877493299820379985852005332863288_i128 as u32;
RET = -_3;
RET = _3;
_4 = !86294725546983163011864916876238236784_u128;
_2 = -RET;
_4 = 109958522657204669354608956103435019563_u128;
RET = 32178_i16 as f32;
RET = _3;
_6 = 3686318458_u32 ^ 3970566930_u32;
RET = _2;
Goto(bb4)
}
bb4 = {
_4 = (-9223372036854775808_isize) as u128;
_2 = -_3;
_8 = [24725_i16,(-21512_i16),17888_i16,22651_i16,(-21277_i16)];
_3 = -_2;
_9.1.2 = [63146_u16,36751_u16,52110_u16,22763_u16,9167_u16,61387_u16,55314_u16,36418_u16];
_6 = 454340586_u32 + 2490762223_u32;
_4 = (-1333742366_i32) as u128;
RET = (-763655846_i32) as f32;
_4 = 71606415041712260547403900403832364672_u128 * 35151738041349419079595204335008853571_u128;
_2 = (-95_i8) as f32;
RET = _3 - _3;
_9.2 = [_4];
_9.1.2 = [45377_u16,7711_u16,59708_u16,45293_u16,44434_u16,31820_u16,26660_u16,55581_u16];
_9.1.3 = 26648_i16 - 10743_i16;
_2 = RET;
_9.1.3 = 31706_i16 + 14168_i16;
_4 = '\u{4c631}' as u128;
_8 = [_9.1.3,_9.1.3,_9.1.3,_9.1.3,_9.1.3];
_5 = core::ptr::addr_of_mut!(_9.2);
Goto(bb5)
}
bb5 = {
_5 = core::ptr::addr_of_mut!((*_5));
(*_5) = [_4];
_9.2 = [_4];
(*_5) = [_4];
(*_5) = [_4];
RET = _2;
_4 = 102557279156660630937210137994399378400_u128 & 284554998416359058363838087431458139057_u128;
_10 = 2041922347828497100_usize;
Goto(bb6)
}
bb6 = {
_10 = !7_usize;
_4 = !187690111726159815316850860741003118334_u128;
_13 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,78_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_11 = [8896777250184870534_u64,17797535874482343806_u64,5285407389815381689_u64,11647151138595979250_u64];
_4 = !116063647844004554542492988438987965197_u128;
_9.1.3 = 10512_i16;
_12 = _8;
(*_5) = [_4];
_6 = 1134687981_u32 ^ 3498940737_u32;
_5 = core::ptr::addr_of_mut!(_9.2);
RET = _3;
Goto(bb7)
}
bb7 = {
_10 = 17601665757196764552_usize ^ 11136935105992170122_usize;
_8 = [_9.1.3,_9.1.3,_9.1.3,_9.1.3,_9.1.3];
Goto(bb8)
}
bb8 = {
RET = _2 - _2;
_6 = _4 as u32;
_4 = RET as u128;
_8 = [_9.1.3,_9.1.3,_9.1.3,_9.1.3,_9.1.3];
_5 = core::ptr::addr_of_mut!((*_5));
_9.1.0 = core::ptr::addr_of!(_15);
_14 = [109_i8,(-81_i8),(-45_i8),61_i8,1_i8,49_i8,(-8_i8)];
Goto(bb9)
}
bb9 = {
Call(_17 = dump_var(18_usize, 11_usize, Move(_11), 4_usize, Move(_4), 14_usize, Move(_14), 10_usize, Move(_10)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{8d23c}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box((-42_i8)), std::hint::black_box(1356_i16), std::hint::black_box(253060794817426634325481142219324696430_u128), std::hint::black_box((-7226267038394879687_i64)), std::hint::black_box(94882645585275313362734958478180646123_i128), std::hint::black_box(10055645253651375474_usize), std::hint::black_box(188_u8), std::hint::black_box(7575259304783801617_u64), std::hint::black_box(1280706149_u32));
                
            }
impl PrintFDebug for Adt24{
	unsafe fn printf_debug(&self){unsafe{printf("Adt24::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt24 {
Variant0{
fld0: i32,

},
Variant1{
fld0: u64,
fld1: char,
fld2: isize,
fld3: u32,
fld4: i16,
fld5: usize,
fld6: u128,
fld7: i128,

},
Variant2{
fld0: *const u8,
fld1: u128,
fld2: isize,
fld3: [u16; 8],
fld4: i16,

},
Variant3{
fld0: f32,
fld1: char,
fld2: usize,
fld3: (u32, u64, f32),
fld4: i16,
fld5: i32,

}}
impl PrintFDebug for Adt31{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt31{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt31 {
fld0: bool,
fld1: char,
fld2: [u16; 8],
fld3: *const isize,
fld4: usize,
fld5: Adt24,
fld6: i64,
}
impl PrintFDebug for Adt34{
	unsafe fn printf_debug(&self){unsafe{printf("Adt34::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt34 {
Variant0{
fld0: u8,
fld1: u32,
fld2: f32,
fld3: *const u8,

},
Variant1{
fld0: u8,
fld1: *const u8,
fld2: Adt31,

},
Variant2{
fld0: *mut u64,
fld1: *const i128,
fld2: u8,
fld3: f32,
fld4: *const *const isize,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: (u32, u64, f32),
fld1: [bool; 4],
fld2: [u16; 8],

},
Variant1{
fld0: i8,
fld1: f64,
fld2: [bool; 4],

},
Variant2{
fld0: bool,
fld1: u64,
fld2: u8,
fld3: [u16; 8],
fld4: (u32, u64, f32),
fld5: (Adt24, *const *const isize, u16, *const u8),
fld6: *const ([u128; 1],),
fld7: [bool; 4],

},
Variant3{
fld0: *const i128,
fld1: Adt34,
fld2: Adt31,

}}
impl PrintFDebug for Adt63{
	unsafe fn printf_debug(&self){unsafe{printf("Adt63::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt63 {
Variant0{
fld0: usize,
fld1: *mut u64,
fld2: [u16; 8],
fld3: *const *const isize,
fld4: [bool; 4],

},
Variant1{
fld0: usize,
fld1: char,
fld2: [u16; 4],
fld3: *const i128,
fld4: f32,

},
Variant2{
fld0: f64,
fld1: char,
fld2: [usize; 8],
fld3: *const ([u128; 1],),

}}
impl PrintFDebug for Adt66{
	unsafe fn printf_debug(&self){unsafe{printf("Adt66::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt66 {
Variant0{
fld0: [u8; 4],
fld1: *mut u64,
fld2: *mut *const u8,

},
Variant1{
fld0: *mut char,
fld1: *mut u64,
fld2: Adt63,
fld3: (*const isize, u128, Adt63),
fld4: [u16; 4],

},
Variant2{
fld0: u8,

},
Variant3{
fld0: bool,
fld1: char,
fld2: u64,
fld3: i8,
fld4: [u8; 4],
fld5: *const char,
fld6: ([u128; 1],),
fld7: [char; 7],

}}
impl PrintFDebug for Adt71{
	unsafe fn printf_debug(&self){unsafe{printf("Adt71::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt71 {
Variant0{
fld0: u128,
fld1: f64,
fld2: i128,
fld3: usize,
fld4: *const ([u128; 1],),
fld5: *mut *const u8,

},
Variant1{
fld0: bool,
fld1: [i16; 1],
fld2: isize,
fld3: f32,
fld4: (*mut *const u8, (*const *const isize, *mut *const u8, [u16; 8], i16), [u128; 1]),
fld5: i32,
fld6: [bool; 4],

},
Variant2{
fld0: u8,
fld1: f64,
fld2: isize,
fld3: f32,
fld4: (*mut *const u8, (*const *const isize, *mut *const u8, [u16; 8], i16), [u128; 1]),
fld5: ([bool; 4], [usize; 8]),

}}
impl PrintFDebug for Adt72{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt72{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt72 {
fld0: [i32; 3],
}

