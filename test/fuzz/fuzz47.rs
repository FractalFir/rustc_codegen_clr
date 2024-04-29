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
pub fn fn0(mut _1: bool,mut _2: u128,mut _3: isize,mut _4: i8,mut _5: u64,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32) -> f32 {
mir! {
type RET = f32;
let _13: (bool, char, [i16; 3], u128);
let _14: char;
let _15: [i64; 2];
let _16: Adt59;
let _17: isize;
let _18: [i64; 2];
let _19: ((*const char, u64, i32), i128, [u64; 3], i16);
let _20: [u8; 6];
let _21: isize;
let _22: f64;
let _23: isize;
let _24: bool;
let _25: [i64; 2];
let _26: [char; 4];
let _27: [char; 4];
let _28: char;
let _29: Adt50;
let _30: [u16; 2];
let _31: Adt47;
let _32: Adt49;
let _33: char;
let _34: bool;
let _35: ();
let _36: ();
{
RET = 110_u8 as f32;
Call(_4 = fn1(RET, RET, RET, RET, RET, RET, RET, RET, RET, RET, RET, RET, RET, RET, RET, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = (-399765227_i32) * (-1533195991_i32);
_11 = true as u16;
_3 = -97_isize;
_12 = !1603135886_u32;
_10 = 69_u8;
_5 = 11416553862444350940_u64;
RET = 119960385425370230167541753739527410619_u128 as f32;
_16.fld5.6.3 = 331383690718777088889358384790573176411_u128;
_15 = [4113566852925339023_i64,(-4272130062648557300_i64)];
_16.fld5.6.1 = '\u{41428}';
_16.fld5.5 = !false;
_16.fld1.fld0.fld2 = _3 << _3;
_16.fld1.fld0.fld3 = _12 as i8;
match _5 {
0 => bb2,
1 => bb3,
11416553862444350940 => bb5,
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
_16.fld5.6.2 = [11919_i16,(-6420_i16),5028_i16];
_16.fld5.2.0 = core::ptr::addr_of_mut!(_16.fld5.0);
_19.1 = (-92982826033599391073151136332005696929_i128) + 107406890563341983869644676887289299978_i128;
_16.fld5.3.0 = core::ptr::addr_of_mut!(_6);
_19.3 = 2517_i16 * (-1009_i16);
_19.2 = [_5,_5,_5];
_13.1 = _16.fld5.6.1;
_16.fld2.0 = _6;
_16.fld1.fld0.fld0 = [_16.fld1.fld0.fld3,_4,_4,_4,_16.fld1.fld0.fld3,_4,_4];
_13.3 = _16.fld5.6.3 % _16.fld5.6.3;
_14 = _13.1;
_1 = _16.fld5.5;
_16.fld4.1.2 = [_11,_11];
_16.fld4.1.2 = [_11,_11];
_4 = -_16.fld1.fld0.fld3;
_8 = _19.1;
_16.fld5.0 = _16.fld2.0;
Goto(bb6)
}
bb6 = {
_16.fld5.6.1 = _14;
Goto(bb7)
}
bb7 = {
_13 = (_16.fld5.5, _16.fld5.6.1, _16.fld5.6.2, _16.fld5.6.3);
_16.fld4.1.1 = (_16.fld5.3.0,);
_13.3 = _10 as u128;
_16.fld5.6.3 = _13.3;
_16.fld2.1.0 = _16.fld5.2.0;
_7 = (-5559491133977762887_i64);
_16.fld2.1.0 = _16.fld5.2.0;
_19.2 = [_5,_5,_5];
_16.fld5.0 = !_6;
_2 = _13.3 >> _6;
_16.fld5.6.0 = !_13.0;
_16.fld2.0 = _16.fld5.0;
match _5 {
11416553862444350940 => bb9,
_ => bb8
}
}
bb8 = {
_6 = (-399765227_i32) * (-1533195991_i32);
_11 = true as u16;
_3 = -97_isize;
_12 = !1603135886_u32;
_10 = 69_u8;
_5 = 11416553862444350940_u64;
RET = 119960385425370230167541753739527410619_u128 as f32;
_16.fld5.6.3 = 331383690718777088889358384790573176411_u128;
_15 = [4113566852925339023_i64,(-4272130062648557300_i64)];
_16.fld5.6.1 = '\u{41428}';
_16.fld5.5 = !false;
_16.fld1.fld0.fld2 = _3 << _3;
_16.fld1.fld0.fld3 = _12 as i8;
match _5 {
0 => bb2,
1 => bb3,
11416553862444350940 => bb5,
_ => bb4
}
}
bb9 = {
_19.3 = 9684_i16 - 6491_i16;
match _10 {
0 => bb7,
1 => bb6,
2 => bb10,
3 => bb11,
69 => bb13,
_ => bb12
}
}
bb10 = {
Return()
}
bb11 = {
_6 = (-399765227_i32) * (-1533195991_i32);
_11 = true as u16;
_3 = -97_isize;
_12 = !1603135886_u32;
_10 = 69_u8;
_5 = 11416553862444350940_u64;
RET = 119960385425370230167541753739527410619_u128 as f32;
_16.fld5.6.3 = 331383690718777088889358384790573176411_u128;
_15 = [4113566852925339023_i64,(-4272130062648557300_i64)];
_16.fld5.6.1 = '\u{41428}';
_16.fld5.5 = !false;
_16.fld1.fld0.fld2 = _3 << _3;
_16.fld1.fld0.fld3 = _12 as i8;
match _5 {
0 => bb2,
1 => bb3,
11416553862444350940 => bb5,
_ => bb4
}
}
bb12 = {
Return()
}
bb13 = {
RET = _8 as f32;
_16.fld4.1.0 = _16.fld5.0;
_17 = RET as isize;
_16.fld2.2 = [_11,_11];
_13.0 = _16.fld5.6.0;
_2 = !_13.3;
_16.fld5.0 = -_16.fld4.1.0;
_19.0.1 = _8 as u64;
_13.3 = _16.fld5.6.3 & _2;
_16.fld3 = [_10,_10,_10,_10];
_16.fld1.fld1 = core::ptr::addr_of_mut!(_16.fld1.fld0.fld4);
_16.fld5.1 = _10;
match _10 {
0 => bb1,
1 => bb10,
2 => bb6,
3 => bb8,
4 => bb5,
69 => bb15,
_ => bb14
}
}
bb14 = {
_16.fld5.6.1 = _14;
Goto(bb7)
}
bb15 = {
_2 = _13.3 * _13.3;
_16.fld5.2 = (_16.fld5.3.0,);
_19.0.2 = !_16.fld4.1.0;
_16.fld4 = (_10, _16.fld2);
_3 = _11 as isize;
_29.fld5.0.2 = _6;
_12 = 418723838_u32 & 941666391_u32;
_5 = _19.0.1;
_30 = [_11,_11];
_16.fld4.1.0 = _16.fld2.0;
_16.fld0 = !_16.fld5.6.0;
_13.0 = !_16.fld0;
_16.fld4 = (_16.fld5.1, _16.fld2);
_16.fld1.fld0.fld1 = core::ptr::addr_of!(_3);
_16.fld3 = [_16.fld4.0,_10,_16.fld5.1,_10];
Goto(bb16)
}
bb16 = {
Call(_35 = dump_var(0_usize, 8_usize, Move(_8), 11_usize, Move(_11), 15_usize, Move(_15), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(0_usize, 14_usize, Move(_14), 17_usize, Move(_17), 2_usize, Move(_2), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: f32,mut _2: f32,mut _3: f32,mut _4: f32,mut _5: f32,mut _6: f32,mut _7: f32,mut _8: f32,mut _9: f32,mut _10: f32,mut _11: f32,mut _12: f32,mut _13: f32,mut _14: f32,mut _15: f32,mut _16: f32) -> i8 {
mir! {
type RET = i8;
let _17: [i8; 7];
let _18: usize;
let _19: f64;
let _20: (i32, u8, (*mut i32,), (*mut i32,), i16, bool, (bool, char, [i16; 3], u128));
let _21: isize;
let _22: [i16; 3];
let _23: bool;
let _24: *mut [i16; 3];
let _25: [u8; 4];
let _26: Adt53;
let _27: [i64; 2];
let _28: u128;
let _29: Adt45;
let _30: [i8; 7];
let _31: [char; 4];
let _32: Adt59;
let _33: i128;
let _34: [i8; 7];
let _35: [i64; 2];
let _36: (u16,);
let _37: Adt52;
let _38: isize;
let _39: ();
let _40: ();
{
_9 = _7;
_1 = _11;
_14 = _8 + _5;
Call(RET = fn2(_13, _7, _4, _11, _8, _2, _8, _3, _2, _13, _10, _6, _14, _12, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_17 = [RET,RET,RET,RET,RET,RET,RET];
_3 = 99610807968835235884829281903061364224_u128 as f32;
_2 = _4 - _7;
_6 = _1 - _10;
_3 = -_7;
_16 = 114073848859116458056414785511722033882_u128 as f32;
_13 = _2;
_6 = _11;
_4 = _10;
_2 = _3;
RET = _8 as i8;
_18 = 5_usize;
match _18 {
0 => bb2,
1 => bb3,
5 => bb5,
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
_5 = _16;
_20.6.3 = 227293345169687303805104223724119835201_u128 * 325215486981807775688115817025284139609_u128;
_20.6.0 = !false;
_13 = _7;
_1 = -_10;
_21 = _17[_18] as isize;
_14 = -_13;
RET = -_17[_18];
_15 = _16;
_20.6.2 = [27776_i16,(-23886_i16),(-26032_i16)];
_22 = [28728_i16,16856_i16,20715_i16];
_20.3.0 = core::ptr::addr_of_mut!(_20.0);
_20.1 = !139_u8;
_20.0 = _20.1 as i32;
RET = 2722169540_u32 as i8;
_1 = _14 * _2;
_19 = _21 as f64;
_9 = _12 + _11;
_20.4 = (-18924_i16) << _20.6.3;
_20.6.2 = [_20.4,_20.4,_20.4];
Goto(bb6)
}
bb6 = {
_9 = _3 - _2;
Goto(bb7)
}
bb7 = {
_24 = core::ptr::addr_of_mut!(_20.6.2);
_25 = [_20.1,_20.1,_20.1,_20.1];
_20.6 = (false, '\u{6f67a}', _22, 142463121142100615162748528466696948321_u128);
_20.6.2 = [_20.4,_20.4,_20.4];
_13 = _16 - _5;
_20.6 = (true, '\u{4e19b}', _22, 185394732419375097767875015676732377980_u128);
_20.1 = 176_u8 << _20.6.3;
RET = _18 as i8;
_17[_18] = RET * RET;
(*_24) = _22;
_29 = Adt45::Variant0 { fld0: _17 };
match _20.6.3 {
0 => bb8,
185394732419375097767875015676732377980 => bb10,
_ => bb9
}
}
bb8 = {
Return()
}
bb9 = {
_5 = _16;
_20.6.3 = 227293345169687303805104223724119835201_u128 * 325215486981807775688115817025284139609_u128;
_20.6.0 = !false;
_13 = _7;
_1 = -_10;
_21 = _17[_18] as isize;
_14 = -_13;
RET = -_17[_18];
_15 = _16;
_20.6.2 = [27776_i16,(-23886_i16),(-26032_i16)];
_22 = [28728_i16,16856_i16,20715_i16];
_20.3.0 = core::ptr::addr_of_mut!(_20.0);
_20.1 = !139_u8;
_20.0 = _20.1 as i32;
RET = 2722169540_u32 as i8;
_1 = _14 * _2;
_19 = _21 as f64;
_9 = _12 + _11;
_20.4 = (-18924_i16) << _20.6.3;
_20.6.2 = [_20.4,_20.4,_20.4];
Goto(bb6)
}
bb10 = {
place!(Field::<[i8; 7]>(Variant(_29, 0), 0))[_18] = !_17[_18];
_32.fld4.1.1 = (_20.3.0,);
_32.fld0 = _20.6.1 != _20.6.1;
_14 = _1 * _1;
_32.fld4.1.0 = _20.0;
_32.fld5.6.3 = _20.6.3;
_32.fld5.5 = !_32.fld0;
match _20.6.3 {
0 => bb6,
1 => bb2,
2 => bb5,
3 => bb11,
4 => bb12,
5 => bb13,
185394732419375097767875015676732377980 => bb15,
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
_24 = core::ptr::addr_of_mut!(_20.6.2);
_25 = [_20.1,_20.1,_20.1,_20.1];
_20.6 = (false, '\u{6f67a}', _22, 142463121142100615162748528466696948321_u128);
_20.6.2 = [_20.4,_20.4,_20.4];
_13 = _16 - _5;
_20.6 = (true, '\u{4e19b}', _22, 185394732419375097767875015676732377980_u128);
_20.1 = 176_u8 << _20.6.3;
RET = _18 as i8;
_17[_18] = RET * RET;
(*_24) = _22;
_29 = Adt45::Variant0 { fld0: _17 };
match _20.6.3 {
0 => bb8,
185394732419375097767875015676732377980 => bb10,
_ => bb9
}
}
bb14 = {
_9 = _3 - _2;
Goto(bb7)
}
bb15 = {
SetDiscriminant(_29, 0);
_20.2.0 = core::ptr::addr_of_mut!(_20.0);
_32.fld5.2 = (_20.3.0,);
_28 = !_32.fld5.6.3;
_8 = _5 * _11;
_32.fld1.fld0.fld1 = core::ptr::addr_of!(_21);
RET = _17[_18];
_32.fld1.fld0.fld0[_18] = _17[_18];
_32.fld5.6.3 = _20.6.3 >> _20.6.3;
_20.3 = _20.2;
_32.fld3 = _25;
_32.fld5.2 = (_32.fld4.1.1.0,);
_32.fld4.0 = _20.1;
(*_24) = [_20.4,_20.4,_20.4];
_14 = -_9;
_32.fld4.1.2 = [23665_u16,14279_u16];
_20.2 = _20.3;
place!(Field::<[i8; 7]>(Variant(_29, 0), 0))[_18] = _32.fld1.fld0.fld0[_18];
Goto(bb16)
}
bb16 = {
Call(_39 = dump_var(1_usize, 17_usize, Move(_17), 18_usize, Move(_18), 28_usize, Move(_28), 40_usize, _40), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: f32,mut _2: f32,mut _3: f32,mut _4: f32,mut _5: f32,mut _6: f32,mut _7: f32,mut _8: f32,mut _9: f32,mut _10: f32,mut _11: f32,mut _12: f32,mut _13: f32,mut _14: f32,mut _15: f32) -> i8 {
mir! {
type RET = i8;
let _16: [char; 4];
let _17: (bool, char, [i16; 3], u128);
let _18: *const i128;
let _19: char;
let _20: [i16; 3];
let _21: (bool, char, [i16; 3], u128);
let _22: isize;
let _23: u16;
let _24: Adt49;
let _25: [u8; 4];
let _26: [u64; 3];
let _27: [u8; 6];
let _28: [char; 4];
let _29: Adt49;
let _30: char;
let _31: [u64; 3];
let _32: u128;
let _33: (*mut i32,);
let _34: f64;
let _35: i8;
let _36: i32;
let _37: (*const isize, (i32, u8, (*mut i32,), (*mut i32,), i16, bool, (bool, char, [i16; 3], u128)));
let _38: [i8; 7];
let _39: i32;
let _40: ();
let _41: ();
{
_13 = -_2;
_2 = _4;
RET = 4_u8 as i8;
_2 = 24971_i16 as f32;
_15 = _7 + _5;
_2 = -_14;
_8 = _7 + _5;
Goto(bb1)
}
bb1 = {
RET = !(-57_i8);
_9 = -_12;
_10 = _8 - _7;
_17.3 = 235170810988261436249111682618211944907_u128;
_16 = ['\u{5e71c}','\u{bcdfe}','\u{56f4a}','\u{d2a80}'];
_21.1 = '\u{8ef8d}';
_13 = -_10;
_11 = _2;
_17.0 = _10 >= _14;
_11 = _3 + _6;
_5 = 2010_i16 as f32;
_4 = (-898058933_i32) as f32;
_17.3 = 7188625020531787956_usize as u128;
_21.0 = _17.0;
Call(RET = fn3(_6, _13, _13, _17.3, _16, _14, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_21.2 = [25859_i16,5111_i16,(-27773_i16)];
_17 = (_21.0, _21.1, _21.2, 221954933917625720332227998753031699065_u128);
_10 = -_13;
_20 = _17.2;
_6 = 134_u8 as f32;
_9 = (-84878595807206527690598522274664020335_i128) as f32;
_17 = (_21.0, _21.1, _20, 125843068059722498701105331229761869127_u128);
_17 = (_21.0, _21.1, _21.2, 332243167341994464532680660274532422928_u128);
_21 = (_17.0, _17.1, _17.2, _17.3);
_13 = _8;
_23 = 12352_u16;
_20 = [28959_i16,14848_i16,(-28008_i16)];
RET = 2930184298_u32 as i8;
_13 = _6;
_5 = _8 - _2;
_21.3 = 9234707811612805871_u64 as u128;
_14 = -_5;
_19 = _17.1;
_10 = _1;
_17 = (_21.0, _21.1, _20, _21.3);
_2 = _5;
_15 = _1;
match _23 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
12352 => bb9,
_ => bb8
}
}
bb3 = {
RET = !(-57_i8);
_9 = -_12;
_10 = _8 - _7;
_17.3 = 235170810988261436249111682618211944907_u128;
_16 = ['\u{5e71c}','\u{bcdfe}','\u{56f4a}','\u{d2a80}'];
_21.1 = '\u{8ef8d}';
_13 = -_10;
_11 = _2;
_17.0 = _10 >= _14;
_11 = _3 + _6;
_5 = 2010_i16 as f32;
_4 = (-898058933_i32) as f32;
_17.3 = 7188625020531787956_usize as u128;
_21.0 = _17.0;
Call(RET = fn3(_6, _13, _13, _17.3, _16, _14, _12), ReturnTo(bb2), UnwindUnreachable())
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
_21.2 = [32759_i16,10185_i16,4008_i16];
_21.2 = [3757_i16,18200_i16,(-31029_i16)];
_17.2 = _21.2;
_21 = (_17.0, _17.1, _20, _17.3);
RET = _19 as i8;
_17.3 = _21.3 + _21.3;
_21.0 = _17.0;
_10 = -_1;
_28 = [_19,_17.1,_21.1,_19];
_21 = _17;
_17 = (_21.0, _21.1, _20, _21.3);
_27 = [173_u8,86_u8,53_u8,129_u8,239_u8,19_u8];
_14 = -_5;
_27 = [67_u8,250_u8,58_u8,143_u8,202_u8,222_u8];
_3 = _5;
_21.3 = _17.3 ^ _17.3;
_17.2 = _20;
RET = (-103_i8);
_7 = _3;
_17.3 = !_21.3;
_28 = [_21.1,_17.1,_17.1,_17.1];
_5 = 11584382579456512641_u64 as f32;
_17 = _21;
Call(_22 = core::intrinsics::bswap(9223372036854775807_isize), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_17.0 = !_21.0;
_17.1 = _21.1;
Goto(bb11)
}
bb11 = {
_21 = _17;
_5 = (-224_i16) as f32;
_19 = _21.1;
_6 = _11 * _8;
RET = 7_i8 | 52_i8;
_14 = _2;
_8 = _13 - _11;
_7 = -_8;
_13 = _8 - _12;
_22 = -58_isize;
_5 = _7 - _11;
_21.3 = !_17.3;
_17.0 = _6 < _5;
_25 = [31_u8,43_u8,15_u8,115_u8];
RET = (-106_i8) << _17.3;
Call(_2 = fn19(RET, _4, _16), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_21.1 = _19;
_17.0 = !_21.0;
_26 = [12740224963619338467_u64,5551653807617384232_u64,5142347912141451894_u64];
_30 = _19;
_17.0 = _21.0;
_15 = -_8;
_26 = [18202325826418663878_u64,6180379423103138099_u64,4032173866579134389_u64];
_27 = [99_u8,241_u8,171_u8,113_u8,11_u8,3_u8];
_26 = [14913740228800239451_u64,6950704450092751920_u64,16397955572370912207_u64];
_28 = _16;
_19 = _21.1;
_31 = _26;
_21.0 = _17.0;
_16 = [_19,_30,_17.1,_19];
_21 = (_17.0, _19, _17.2, _17.3);
_27 = [52_u8,168_u8,92_u8,207_u8,229_u8,155_u8];
_7 = _5;
_37.1.2.0 = core::ptr::addr_of_mut!(_37.1.0);
_37.1.6.0 = _21.0;
_37.1.6.2 = _17.2;
Goto(bb13)
}
bb13 = {
_17.3 = !_21.3;
match _23 {
0 => bb8,
1 => bb2,
2 => bb9,
3 => bb12,
4 => bb5,
12352 => bb14,
_ => bb11
}
}
bb14 = {
_2 = (-1627836078_i32) as f32;
_37.1.6 = (_17.0, _17.1, _17.2, _21.3);
_17.1 = _19;
_6 = _7;
_37.1.0 = 544787502_i32 << RET;
_37.1.6.1 = _30;
_31 = _26;
_16 = [_17.1,_37.1.6.1,_37.1.6.1,_37.1.6.1];
_36 = _37.1.0;
_37.1.2.0 = core::ptr::addr_of_mut!(_37.1.0);
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(2_usize, 22_usize, Move(_22), 26_usize, Move(_26), 28_usize, Move(_28), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(2_usize, 21_usize, Move(_21), 19_usize, Move(_19), 30_usize, Move(_30), 41_usize, _41), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: f32,mut _2: f32,mut _3: f32,mut _4: u128,mut _5: [char; 4],mut _6: f32,mut _7: f32) -> i8 {
mir! {
type RET = i8;
let _8: f64;
let _9: ((*const char, u64, i32), i128, [u64; 3], i16);
let _10: [u64; 3];
let _11: i64;
let _12: bool;
let _13: isize;
let _14: [i16; 3];
let _15: Adt50;
let _16: [char; 4];
let _17: f64;
let _18: u128;
let _19: u16;
let _20: u16;
let _21: i128;
let _22: isize;
let _23: (u16,);
let _24: Adt59;
let _25: [u16; 2];
let _26: f32;
let _27: (u16,);
let _28: usize;
let _29: char;
let _30: (u16,);
let _31: ();
let _32: ();
{
RET = (-49_i8) + 51_i8;
_6 = _3;
RET = -56_i8;
RET = 19_i8 + (-90_i8);
_2 = 18205182152293258150_u64 as f32;
_3 = _6;
_8 = 1696035837_u32 as f64;
Goto(bb1)
}
bb1 = {
_6 = 3853621726_u32 as f32;
_2 = _3;
RET = _8 as i8;
Call(_6 = fn4(_3, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = 30_i8;
_3 = _7 - _6;
_9.0.1 = 10463317467841992865_u64 + 6813712980480657761_u64;
_7 = -_3;
_6 = _7 + _3;
_1 = _4 as f32;
_2 = _6;
_9.0.2 = !(-160978317_i32);
_10 = [_9.0.1,_9.0.1,_9.0.1];
_10 = [_9.0.1,_9.0.1,_9.0.1];
_10 = [_9.0.1,_9.0.1,_9.0.1];
_9.3 = (-6794_i16) * (-20541_i16);
RET = (-116_i8);
_4 = 21665275303709431008184198083014270742_u128 << _9.0.1;
Goto(bb3)
}
bb3 = {
_14 = [_9.3,_9.3,_9.3];
_12 = true;
_6 = _7 * _3;
_9.3 = (-2117_i16) >> _9.0.1;
_13 = (-9223372036854775808_isize) << _9.0.2;
_3 = _2 * _7;
_5 = ['\u{ec0ab}','\u{22ef2}','\u{98358}','\u{dfe84}'];
_2 = _6;
_16 = ['\u{89346}','\u{75a0e}','\u{c3fdb}','\u{1b134}'];
_9.0.2 = (-374165487_i32);
_9.2 = _10;
_15.fld5.2 = _9.2;
_15.fld0 = _12;
_15.fld3 = 5_usize as i8;
_15.fld5.0.2 = -_9.0.2;
_15.fld4 = !1645244122_u32;
_13 = _4 as isize;
_15.fld7 = core::ptr::addr_of!(_15.fld2);
_15.fld4 = 749787957_u32;
_11 = _9.0.1 as i64;
match _9.0.2 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
340282366920938463463374607431394045969 => bb8,
_ => bb7
}
}
bb4 = {
RET = 30_i8;
_3 = _7 - _6;
_9.0.1 = 10463317467841992865_u64 + 6813712980480657761_u64;
_7 = -_3;
_6 = _7 + _3;
_1 = _4 as f32;
_2 = _6;
_9.0.2 = !(-160978317_i32);
_10 = [_9.0.1,_9.0.1,_9.0.1];
_10 = [_9.0.1,_9.0.1,_9.0.1];
_10 = [_9.0.1,_9.0.1,_9.0.1];
_9.3 = (-6794_i16) * (-20541_i16);
RET = (-116_i8);
_4 = 21665275303709431008184198083014270742_u128 << _9.0.1;
Goto(bb3)
}
bb5 = {
_6 = 3853621726_u32 as f32;
_2 = _3;
RET = _8 as i8;
Call(_6 = fn4(_3, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_15.fld5.1 = (-22566032431903437352526759849232731373_i128);
_15.fld1.0 = core::ptr::addr_of_mut!(_9.0.2);
Goto(bb9)
}
bb9 = {
_18 = _4 >> _9.3;
_6 = _9.0.2 as f32;
_15.fld3 = -RET;
_19 = !47192_u16;
_11 = 1739968169655447308_i64;
_9.0.2 = !_15.fld5.0.2;
_19 = 61063_u16;
_15.fld5.3 = _9.3;
_1 = _7 - _3;
Goto(bb10)
}
bb10 = {
_3 = _15.fld3 as f32;
Goto(bb11)
}
bb11 = {
_21 = _13 as i128;
match _15.fld5.1 {
0 => bb1,
1 => bb10,
2 => bb7,
317716334489035026110847847582535480083 => bb12,
_ => bb4
}
}
bb12 = {
_9.1 = _21;
_15.fld5.1 = !_9.1;
_22 = !_13;
_6 = -_1;
_17 = _8 - _8;
RET = _15.fld3 >> _9.1;
_12 = _15.fld0 & _15.fld0;
_24.fld5.3.0 = _15.fld1.0;
_24.fld4.1.1.0 = _15.fld1.0;
_24.fld1.fld0.fld1 = core::ptr::addr_of!(_22);
_24.fld1.fld0.fld2 = _22 >> _18;
_24.fld5.5 = _2 == _6;
match _11 {
0 => bb6,
1 => bb11,
2 => bb3,
3 => bb13,
1739968169655447308 => bb15,
_ => bb14
}
}
bb13 = {
RET = 30_i8;
_3 = _7 - _6;
_9.0.1 = 10463317467841992865_u64 + 6813712980480657761_u64;
_7 = -_3;
_6 = _7 + _3;
_1 = _4 as f32;
_2 = _6;
_9.0.2 = !(-160978317_i32);
_10 = [_9.0.1,_9.0.1,_9.0.1];
_10 = [_9.0.1,_9.0.1,_9.0.1];
_10 = [_9.0.1,_9.0.1,_9.0.1];
_9.3 = (-6794_i16) * (-20541_i16);
RET = (-116_i8);
_4 = 21665275303709431008184198083014270742_u128 << _9.0.1;
Goto(bb3)
}
bb14 = {
_6 = 3853621726_u32 as f32;
_2 = _3;
RET = _8 as i8;
Call(_6 = fn4(_3, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
_24.fld5.4 = _21 as i16;
_25 = [_19,_19];
_15.fld5.0.0 = core::ptr::addr_of!(_24.fld5.6.1);
_24.fld1.fld0.fld2 = _22 << _9.1;
_18 = _4;
_21 = _9.1 - _9.1;
RET = _15.fld3 & _15.fld3;
_7 = _1;
_27 = (_19,);
_24.fld3 = [184_u8,215_u8,82_u8,186_u8];
_20 = _19;
_15.fld0 = _24.fld5.5;
_24.fld1.fld0.fld0 = [RET,_15.fld3,RET,_15.fld3,_15.fld3,RET,_15.fld3];
Goto(bb16)
}
bb16 = {
Call(_31 = dump_var(3_usize, 21_usize, Move(_21), 10_usize, Move(_10), 5_usize, Move(_5), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(3_usize, 20_usize, Move(_20), 16_usize, Move(_16), 27_usize, Move(_27), 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: f32,mut _2: f64) -> f32 {
mir! {
type RET = f32;
let _3: f64;
let _4: *mut isize;
let _5: [char; 4];
let _6: [char; 4];
let _7: char;
let _8: f32;
let _9: [i16; 3];
let _10: f64;
let _11: [i64; 2];
let _12: f32;
let _13: [u16; 2];
let _14: bool;
let _15: ();
let _16: ();
{
RET = _1;
_1 = RET;
_2 = (-21483_i16) as f64;
_2 = (-128_i8) as f64;
RET = -_1;
_1 = -RET;
_1 = RET + RET;
RET = 150652565504943086147192033408574571691_u128 as f32;
_2 = 32259_i16 as f64;
_2 = 3263345350_u32 as f64;
_3 = _2 * _2;
_1 = -RET;
RET = _1;
_2 = _3 - _3;
RET = -_1;
RET = -_1;
RET = _1 - _1;
RET = _1 - _1;
RET = -_1;
_2 = -_3;
RET = _1 * _1;
_2 = 8356_i16 as f64;
_6 = ['\u{f9381}','\u{7ec20}','\u{ba537}','\u{4aa6f}'];
Goto(bb1)
}
bb1 = {
_1 = RET + RET;
RET = -_1;
Call(_5 = fn5(_6, _1, _1, _1, _1, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = RET;
_5 = _6;
RET = -_1;
_3 = (-9223372036854775808_isize) as f64;
RET = _1 - _1;
_1 = 10154257268968317130_u64 as f32;
_2 = -_3;
Call(_6 = fn17(RET, RET, RET, RET, RET, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3 = _2;
_9 = [(-10097_i16),(-11849_i16),(-20787_i16)];
_7 = '\u{22549}';
_10 = 6848_i16 as f64;
_8 = RET;
_7 = '\u{ccc1e}';
_1 = -_8;
_11 = [(-3009732345379203419_i64),(-8506994418477173699_i64)];
_1 = -RET;
_12 = _1 - _8;
_11 = [(-6581261344825493835_i64),8409965167256427974_i64];
_6 = _5;
_12 = (-45508678256432202422838087392215195090_i128) as f32;
_10 = -_2;
RET = -_1;
_2 = 297569582133723874514735972692620332756_u128 as f64;
_6 = [_7,_7,_7,_7];
RET = 13585407096412521387_u64 as f32;
_12 = (-2577204233119457689_i64) as f32;
_1 = _8;
Goto(bb4)
}
bb4 = {
RET = _8 * _1;
_12 = _8;
_13 = [26040_u16,9022_u16];
_10 = _3 * _3;
_2 = 1742740087_i32 as f64;
_2 = 7_u8 as f64;
_3 = -_10;
_10 = _2;
_6 = _5;
_7 = '\u{1005b5}';
_14 = true | true;
_7 = '\u{3c681}';
_1 = _12 + RET;
_3 = 103_u8 as f64;
_8 = _12 + _1;
_5 = _6;
_14 = false;
_1 = -_8;
_3 = _10 * _10;
_11 = [(-7460179298470164146_i64),(-3272815671908304370_i64)];
_10 = -_3;
Goto(bb5)
}
bb5 = {
Call(_15 = dump_var(4_usize, 9_usize, Move(_9), 14_usize, Move(_14), 5_usize, Move(_5), 16_usize, _16), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: [char; 4],mut _2: f32,mut _3: f32,mut _4: f32,mut _5: f32,mut _6: f64) -> [char; 4] {
mir! {
type RET = [char; 4];
let _7: u64;
let _8: *const isize;
let _9: [u8; 6];
let _10: i128;
let _11: u128;
let _12: [u16; 2];
let _13: i64;
let _14: [i64; 2];
let _15: bool;
let _16: u16;
let _17: [i16; 3];
let _18: char;
let _19: [u8; 4];
let _20: i16;
let _21: char;
let _22: [u16; 2];
let _23: f64;
let _24: (bool, char, [i16; 3], u128);
let _25: u16;
let _26: Adt57;
let _27: i64;
let _28: [char; 4];
let _29: ();
let _30: ();
{
_6 = 4807427318327723973_i64 as f64;
_2 = _5 - _3;
_6 = (-724034601_i32) as f64;
_3 = _4 * _2;
_4 = 12824695942151137570_u64 as f32;
RET = ['\u{b21f2}','\u{230fb}','\u{e2e3d}','\u{a775e}'];
_12 = [27232_u16,17551_u16];
_7 = 4146101986954806850_u64;
_6 = _2 as f64;
_7 = 6014131951441357405_usize as u64;
_11 = _3 as u128;
Goto(bb1)
}
bb1 = {
_5 = _2;
_15 = _6 > _6;
_3 = _7 as f32;
Call(_8 = fn6(_4, _5, RET, _15, _5, _11, _5, _15, _6, _11, _15, RET, _6, _11, _15), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_17 = [28294_i16,31671_i16,11333_i16];
_10 = !124299054036202735886359709427205264527_i128;
_13 = (-7935763425173331398_i64);
_14 = [_13,_13];
_10 = 62676889013663623163939178587771779979_i128;
_9 = [130_u8,254_u8,236_u8,115_u8,199_u8,182_u8];
_5 = _2;
_15 = true;
_9 = [132_u8,210_u8,162_u8,84_u8,109_u8,134_u8];
_2 = _3;
_11 = 158955751439018860112493853929557067243_u128 * 273007470042034092896646270522961605539_u128;
_16 = 57180_u16 >> _13;
_21 = '\u{3e17e}';
_5 = _10 as f32;
_18 = _21;
_16 = 23455_u16;
_13 = -736085387573179905_i64;
_7 = !7426414882065610734_u64;
_15 = true;
match _10 {
0 => bb3,
1 => bb4,
2 => bb5,
62676889013663623163939178587771779979 => bb7,
_ => bb6
}
}
bb3 = {
_5 = _2;
_15 = _6 > _6;
_3 = _7 as f32;
Call(_8 = fn6(_4, _5, RET, _15, _5, _11, _5, _15, _6, _11, _15, RET, _6, _11, _15), ReturnTo(bb2), UnwindUnreachable())
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
_1 = RET;
Goto(bb8)
}
bb8 = {
_22 = [_16,_16];
match _16 {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb9,
4 => bb10,
23455 => bb12,
_ => bb11
}
}
bb9 = {
_1 = RET;
Goto(bb8)
}
bb10 = {
_17 = [28294_i16,31671_i16,11333_i16];
_10 = !124299054036202735886359709427205264527_i128;
_13 = (-7935763425173331398_i64);
_14 = [_13,_13];
_10 = 62676889013663623163939178587771779979_i128;
_9 = [130_u8,254_u8,236_u8,115_u8,199_u8,182_u8];
_5 = _2;
_15 = true;
_9 = [132_u8,210_u8,162_u8,84_u8,109_u8,134_u8];
_2 = _3;
_11 = 158955751439018860112493853929557067243_u128 * 273007470042034092896646270522961605539_u128;
_16 = 57180_u16 >> _13;
_21 = '\u{3e17e}';
_5 = _10 as f32;
_18 = _21;
_16 = 23455_u16;
_13 = -736085387573179905_i64;
_7 = !7426414882065610734_u64;
_15 = true;
match _10 {
0 => bb3,
1 => bb4,
2 => bb5,
62676889013663623163939178587771779979 => bb7,
_ => bb6
}
}
bb11 = {
Return()
}
bb12 = {
_19 = [234_u8,200_u8,192_u8,225_u8];
_15 = true;
_12 = _22;
_7 = !8542653132544848117_u64;
_14 = [_13,_13];
_14 = [_13,_13];
_12 = [_16,_16];
_24 = (_15, _18, _17, _11);
_15 = _24.0 | _24.0;
_22 = [_16,_16];
_18 = _24.1;
_1 = [_18,_18,_21,_18];
_13 = !2843213166752231329_i64;
_23 = _6 - _6;
Call(_9 = core::intrinsics::transmute(_24.2), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_18 = _21;
_10 = (-95381521597589687061154347679880194079_i128) + 73802683345257380768222709358960994211_i128;
_20 = -5505_i16;
_15 = _24.0 & _24.0;
_24.3 = _11;
_25 = _23 as u16;
_14 = [_13,_13];
Goto(bb14)
}
bb14 = {
_2 = -_4;
_1 = RET;
_9 = [124_u8,78_u8,113_u8,234_u8,62_u8,188_u8];
_22 = [_25,_25];
_11 = _24.3;
_21 = _18;
_20 = 29072_i16;
_3 = _4;
RET = [_21,_24.1,_21,_24.1];
_24.2 = _17;
_18 = _24.1;
_10 = (-60454867947220850631358704400787413120_i128);
_24.2 = [_20,_20,_20];
_22 = [_25,_25];
_14 = [_13,_13];
_21 = _18;
_1 = RET;
_23 = _6;
_12 = _22;
_3 = _4;
_14 = [_13,_13];
_13 = _10 as i64;
_24.0 = _15;
_23 = _6 + _6;
_19 = [104_u8,31_u8,207_u8,16_u8];
_10 = _15 as i128;
_24.2 = [_20,_20,_20];
_24.0 = _15 ^ _15;
_17 = [_20,_20,_20];
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(5_usize, 13_usize, Move(_13), 16_usize, Move(_16), 22_usize, Move(_22), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(5_usize, 19_usize, Move(_19), 11_usize, Move(_11), 12_usize, Move(_12), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(5_usize, 14_usize, Move(_14), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: f32,mut _2: f32,mut _3: [char; 4],mut _4: bool,mut _5: f32,mut _6: u128,mut _7: f32,mut _8: bool,mut _9: f64,mut _10: u128,mut _11: bool,mut _12: [char; 4],mut _13: f64,mut _14: u128,mut _15: bool) -> *const isize {
mir! {
type RET = *const isize;
let _16: Adt47;
let _17: *mut isize;
let _18: [u64; 3];
let _19: u64;
let _20: *const char;
let _21: [u8; 4];
let _22: [i8; 7];
let _23: Adt45;
let _24: i16;
let _25: char;
let _26: isize;
let _27: bool;
let _28: bool;
let _29: [u64; 3];
let _30: [u8; 4];
let _31: [u8; 4];
let _32: char;
let _33: (i32, (*mut i32,), [u16; 2]);
let _34: [u16; 2];
let _35: (bool, char, [i16; 3], u128);
let _36: (bool, char, [i16; 3], u128);
let _37: (bool, char, [i16; 3], u128);
let _38: i8;
let _39: usize;
let _40: [u8; 4];
let _41: (bool, char, [i16; 3], u128);
let _42: ();
let _43: ();
{
_1 = _7;
_9 = -_13;
_11 = _7 < _5;
_3 = _12;
_12 = ['\u{94c2f}','\u{af14c}','\u{10a104}','\u{8b40}'];
_9 = _13;
_13 = _9;
_5 = _7 - _7;
_10 = _14 * _6;
_8 = !_11;
Call(_16.fld2 = fn7(_10, _6, _4, _9, _14, _4, _10, _5, _8, _4, _10, _15, _10, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = _10 >> _10;
_5 = -_7;
_11 = _15 | _4;
_7 = _1;
_6 = !_10;
_16.fld0 = -_2;
_16.fld0 = -_7;
_4 = _11;
_10 = _6 >> _6;
_15 = _4 != _11;
_15 = _7 == _16.fld0;
_7 = -_1;
_16.fld1 = ['\u{5e416}','\u{f7016}','\u{167b9}','\u{25e54}'];
_14 = _10;
_19 = 14759589259158676766_u64;
_15 = _6 > _6;
_14 = _6 ^ _10;
_16.fld0 = 0_usize as f32;
_21 = [252_u8,43_u8,198_u8,121_u8];
_14 = _10 ^ _10;
Call(_23 = fn9(_14, _14, _10, _11, _6, _14, _10, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = core::ptr::addr_of!(place!(Field::<isize>(Variant(_23, 1), 2)));
place!(Field::<[i16; 3]>(Variant(_23, 1), 3)) = [(-920_i16),(-18166_i16),(-6262_i16)];
_16.fld0 = 16371_i16 as f32;
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6)) = (_11, '\u{5f61a}', Field::<[i16; 3]>(Variant(_23, 1), 3), _14);
_16.fld5.0 = Field::<(*mut i32,)>(Variant(_23, 1), 1).0;
_3 = _12;
place!(Field::<(*mut i32,)>(Variant(_23, 1), 1)).0 = _16.fld5.0;
_20 = core::ptr::addr_of!(_25);
_2 = _16.fld0;
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6)).0 = _14 > Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).3;
(*RET) = Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1 as isize;
_27 = Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).0;
_16.fld5 = (Field::<(*mut i32,)>(Variant(_23, 1), 1).0,);
_1 = _5;
_12 = [Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1,Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1,Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1,Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1];
(*RET) = 9223372036854775807_isize >> Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).3;
(*RET) = 9223372036854775807_isize;
Goto(bb3)
}
bb3 = {
_8 = Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).0 | _27;
(*RET) = 66_isize >> _14;
_9 = _13 - _13;
_16.fld5.0 = Field::<(*mut i32,)>(Variant(_23, 1), 1).0;
place!(Field::<isize>(Variant(_23, 1), 2)) = !(-9223372036854775808_isize);
_13 = _9;
_16.fld1 = [Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1,Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1,Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1,Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1];
_22 = [(-36_i8),(-120_i8),(-112_i8),24_i8,22_i8,(-19_i8),(-72_i8)];
_11 = _8 & Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).0;
(*_20) = Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1;
RET = core::ptr::addr_of!(place!(Field::<isize>(Variant(_23, 1), 2)));
_16.fld1 = [(*_20),Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1,(*_20),(*_20)];
match _19 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
14759589259158676766 => bb9,
_ => bb8
}
}
bb4 = {
RET = core::ptr::addr_of!(place!(Field::<isize>(Variant(_23, 1), 2)));
place!(Field::<[i16; 3]>(Variant(_23, 1), 3)) = [(-920_i16),(-18166_i16),(-6262_i16)];
_16.fld0 = 16371_i16 as f32;
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6)) = (_11, '\u{5f61a}', Field::<[i16; 3]>(Variant(_23, 1), 3), _14);
_16.fld5.0 = Field::<(*mut i32,)>(Variant(_23, 1), 1).0;
_3 = _12;
place!(Field::<(*mut i32,)>(Variant(_23, 1), 1)).0 = _16.fld5.0;
_20 = core::ptr::addr_of!(_25);
_2 = _16.fld0;
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6)).0 = _14 > Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).3;
(*RET) = Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1 as isize;
_27 = Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).0;
_16.fld5 = (Field::<(*mut i32,)>(Variant(_23, 1), 1).0,);
_1 = _5;
_12 = [Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1,Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1,Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1,Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1];
(*RET) = 9223372036854775807_isize >> Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).3;
(*RET) = 9223372036854775807_isize;
Goto(bb3)
}
bb5 = {
_6 = _10 >> _10;
_5 = -_7;
_11 = _15 | _4;
_7 = _1;
_6 = !_10;
_16.fld0 = -_2;
_16.fld0 = -_7;
_4 = _11;
_10 = _6 >> _6;
_15 = _4 != _11;
_15 = _7 == _16.fld0;
_7 = -_1;
_16.fld1 = ['\u{5e416}','\u{f7016}','\u{167b9}','\u{25e54}'];
_14 = _10;
_19 = 14759589259158676766_u64;
_15 = _6 > _6;
_14 = _6 ^ _10;
_16.fld0 = 0_usize as f32;
_21 = [252_u8,43_u8,198_u8,121_u8];
_14 = _10 ^ _10;
Call(_23 = fn9(_14, _14, _10, _11, _6, _14, _10, _10), ReturnTo(bb2), UnwindUnreachable())
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
_4 = Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1 == (*_20);
RET = Field::<*const isize>(Variant(_23, 1), 0);
place!(Field::<isize>(Variant(_23, 1), 2)) = -84_isize;
_20 = core::ptr::addr_of!(place!(Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6)).1);
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6)).2 = Field::<[i16; 3]>(Variant(_23, 1), 3);
_23 = Adt45::Variant0 { fld0: _22 };
_7 = -_2;
_16.fld1 = [_25,_25,_25,_25];
_24 = !(-27511_i16);
match _19 {
0 => bb1,
1 => bb7,
14759589259158676766 => bb10,
_ => bb3
}
}
bb10 = {
_34 = [59361_u16,52563_u16];
_31 = [212_u8,23_u8,87_u8,250_u8];
_16.fld5.0 = core::ptr::addr_of_mut!(_33.0);
_33.0 = 272743134_i32 + 340839871_i32;
_33.1.0 = _16.fld5.0;
_35.2 = [_24,_24,_24];
Goto(bb11)
}
bb11 = {
_35.0 = !_4;
_34 = [40937_u16,56619_u16];
_28 = !_4;
_33.1.0 = _16.fld5.0;
_9 = _13;
_8 = !_27;
SetDiscriminant(_23, 1);
_17 = core::ptr::addr_of_mut!(_26);
place!(Field::<[u64; 3]>(Variant(_23, 1), 4)) = [_19,_19,_19];
_7 = _1;
_16.fld0 = -_1;
place!(Field::<(*mut i32,)>(Variant(_23, 1), 1)) = (_16.fld5.0,);
_35.1 = _25;
place!(Field::<isize>(Variant(_23, 1), 2)) = 3_usize as isize;
_1 = -_7;
_18 = [_19,_19,_19];
(*_17) = Field::<isize>(Variant(_23, 1), 2);
_30 = [157_u8,35_u8,181_u8,233_u8];
_13 = -_9;
match _19 {
0 => bb6,
1 => bb12,
2 => bb13,
3 => bb14,
14759589259158676766 => bb16,
_ => bb15
}
}
bb12 = {
RET = core::ptr::addr_of!(place!(Field::<isize>(Variant(_23, 1), 2)));
place!(Field::<[i16; 3]>(Variant(_23, 1), 3)) = [(-920_i16),(-18166_i16),(-6262_i16)];
_16.fld0 = 16371_i16 as f32;
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6)) = (_11, '\u{5f61a}', Field::<[i16; 3]>(Variant(_23, 1), 3), _14);
_16.fld5.0 = Field::<(*mut i32,)>(Variant(_23, 1), 1).0;
_3 = _12;
place!(Field::<(*mut i32,)>(Variant(_23, 1), 1)).0 = _16.fld5.0;
_20 = core::ptr::addr_of!(_25);
_2 = _16.fld0;
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6)).0 = _14 > Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).3;
(*RET) = Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1 as isize;
_27 = Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).0;
_16.fld5 = (Field::<(*mut i32,)>(Variant(_23, 1), 1).0,);
_1 = _5;
_12 = [Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1,Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1,Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1,Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).1];
(*RET) = 9223372036854775807_isize >> Field::<(bool, char, [i16; 3], u128)>(Variant(_23, 1), 6).3;
(*RET) = 9223372036854775807_isize;
Goto(bb3)
}
bb13 = {
_6 = _10 >> _10;
_5 = -_7;
_11 = _15 | _4;
_7 = _1;
_6 = !_10;
_16.fld0 = -_2;
_16.fld0 = -_7;
_4 = _11;
_10 = _6 >> _6;
_15 = _4 != _11;
_15 = _7 == _16.fld0;
_7 = -_1;
_16.fld1 = ['\u{5e416}','\u{f7016}','\u{167b9}','\u{25e54}'];
_14 = _10;
_19 = 14759589259158676766_u64;
_15 = _6 > _6;
_14 = _6 ^ _10;
_16.fld0 = 0_usize as f32;
_21 = [252_u8,43_u8,198_u8,121_u8];
_14 = _10 ^ _10;
Call(_23 = fn9(_14, _14, _10, _11, _6, _14, _10, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_6 = _10 >> _10;
_5 = -_7;
_11 = _15 | _4;
_7 = _1;
_6 = !_10;
_16.fld0 = -_2;
_16.fld0 = -_7;
_4 = _11;
_10 = _6 >> _6;
_15 = _4 != _11;
_15 = _7 == _16.fld0;
_7 = -_1;
_16.fld1 = ['\u{5e416}','\u{f7016}','\u{167b9}','\u{25e54}'];
_14 = _10;
_19 = 14759589259158676766_u64;
_15 = _6 > _6;
_14 = _6 ^ _10;
_16.fld0 = 0_usize as f32;
_21 = [252_u8,43_u8,198_u8,121_u8];
_14 = _10 ^ _10;
Call(_23 = fn9(_14, _14, _10, _11, _6, _14, _10, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
Return()
}
bb16 = {
_16.fld4 = [148_u8,113_u8,114_u8,247_u8];
_36.0 = _8 > _4;
_8 = _4;
_36.1 = _25;
_33.1 = (_16.fld5.0,);
_39 = 8347144186174568942_usize & 0_usize;
_24 = 30075_i16 | 13031_i16;
Goto(bb17)
}
bb17 = {
Call(_42 = dump_var(6_usize, 8_usize, Move(_8), 26_usize, Move(_26), 31_usize, Move(_31), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(6_usize, 27_usize, Move(_27), 25_usize, Move(_25), 4_usize, Move(_4), 6_usize, Move(_6)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_42 = dump_var(6_usize, 15_usize, Move(_15), 19_usize, Move(_19), 11_usize, Move(_11), 43_usize, _43), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: u128,mut _2: u128,mut _3: bool,mut _4: f64,mut _5: u128,mut _6: bool,mut _7: u128,mut _8: f32,mut _9: bool,mut _10: bool,mut _11: u128,mut _12: bool,mut _13: u128,mut _14: f64) -> *const i128 {
mir! {
type RET = *const i128;
let _15: [u8; 6];
let _16: [i16; 3];
let _17: u128;
let _18: i32;
let _19: u16;
let _20: [u16; 2];
let _21: char;
let _22: isize;
let _23: f32;
let _24: f32;
let _25: u128;
let _26: f32;
let _27: i16;
let _28: usize;
let _29: u128;
let _30: [char; 4];
let _31: [i64; 2];
let _32: [i8; 7];
let _33: [u8; 4];
let _34: f32;
let _35: [u8; 4];
let _36: [u64; 3];
let _37: char;
let _38: isize;
let _39: Adt51;
let _40: f32;
let _41: (bool, char, [i16; 3], u128);
let _42: i8;
let _43: [u8; 6];
let _44: [u64; 3];
let _45: [u8; 6];
let _46: (bool, char, [i16; 3], u128);
let _47: Adt46;
let _48: i16;
let _49: (*const isize, (i32, u8, (*mut i32,), (*mut i32,), i16, bool, (bool, char, [i16; 3], u128)));
let _50: isize;
let _51: ((*const char, u64, i32), i128, [u64; 3], i16);
let _52: u16;
let _53: i128;
let _54: i128;
let _55: u16;
let _56: Adt52;
let _57: [i16; 3];
let _58: ();
let _59: ();
{
_3 = _10;
_9 = _10;
_14 = 15540501770909858445_usize as f64;
_15 = [151_u8,1_u8,80_u8,213_u8,196_u8,81_u8];
_11 = !_7;
_6 = _10;
_6 = _10;
_13 = 9223372036854775807_isize as u128;
Goto(bb1)
}
bb1 = {
_5 = _11;
_6 = !_12;
_16 = [13630_i16,11766_i16,3513_i16];
_5 = _11 >> _11;
_10 = !_6;
_5 = _2;
_17 = _2;
Goto(bb2)
}
bb2 = {
_16 = [25325_i16,(-15442_i16),(-12564_i16)];
_12 = _6;
_15 = [129_u8,249_u8,140_u8,2_u8,206_u8,43_u8];
_15 = [226_u8,251_u8,80_u8,0_u8,116_u8,100_u8];
_17 = _11;
_13 = _11;
_2 = _1 ^ _17;
_9 = _2 == _2;
_7 = _2 + _2;
_17 = !_2;
_18 = 2064119085_i32;
_14 = _18 as f64;
_8 = _18 as f32;
_14 = -_4;
_3 = !_9;
_16 = [1180_i16,31607_i16,24279_i16];
_7 = _1 >> _2;
_18 = _8 as i32;
_4 = _14;
_17 = _2 & _2;
_16 = [(-25525_i16),22544_i16,(-16794_i16)];
_9 = _3;
_3 = _9;
_8 = 143_u8 as f32;
_5 = 52_u8 as u128;
Goto(bb3)
}
bb3 = {
_19 = 64968_u16;
_16 = [(-29229_i16),(-2774_i16),(-16786_i16)];
_20 = [_19,_19];
_12 = !_3;
_7 = (-4563760205122373378_i64) as u128;
_22 = 106645999125959778350342963130105949680_i128 as isize;
_12 = _3 > _3;
_13 = !_2;
_9 = !_3;
_1 = _17 >> _13;
_2 = _1;
_14 = _13 as f64;
_1 = _13 ^ _17;
_20 = [_19,_19];
_21 = '\u{7a200}';
_8 = _19 as f32;
_7 = !_2;
_4 = _14 * _14;
_21 = '\u{b4d95}';
_1 = (-8683318763490396098_i64) as u128;
_8 = _18 as f32;
_25 = _2 & _2;
_23 = _22 as f32;
_6 = _3;
_7 = (-5487743910766246896_i64) as u128;
_22 = _21 as isize;
_11 = _17;
match _19 {
0 => bb1,
64968 => bb5,
_ => bb4
}
}
bb4 = {
_16 = [25325_i16,(-15442_i16),(-12564_i16)];
_12 = _6;
_15 = [129_u8,249_u8,140_u8,2_u8,206_u8,43_u8];
_15 = [226_u8,251_u8,80_u8,0_u8,116_u8,100_u8];
_17 = _11;
_13 = _11;
_2 = _1 ^ _17;
_9 = _2 == _2;
_7 = _2 + _2;
_17 = !_2;
_18 = 2064119085_i32;
_14 = _18 as f64;
_8 = _18 as f32;
_14 = -_4;
_3 = !_9;
_16 = [1180_i16,31607_i16,24279_i16];
_7 = _1 >> _2;
_18 = _8 as i32;
_4 = _14;
_17 = _2 & _2;
_16 = [(-25525_i16),22544_i16,(-16794_i16)];
_9 = _3;
_3 = _9;
_8 = 143_u8 as f32;
_5 = 52_u8 as u128;
Goto(bb3)
}
bb5 = {
_15 = [167_u8,60_u8,76_u8,76_u8,24_u8,64_u8];
_21 = '\u{c529c}';
_15 = [0_u8,89_u8,140_u8,149_u8,170_u8,165_u8];
Call(_22 = core::intrinsics::bswap(109_isize), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_28 = 2632170450712206584_usize;
_2 = _11;
_20 = [_19,_19];
_14 = _18 as f64;
_18 = -713842989_i32;
_27 = (-24433_i16);
_13 = !_17;
_21 = '\u{77f70}';
_12 = _6;
_26 = -_8;
_26 = _8;
_26 = _27 as f32;
_29 = !_11;
_2 = _25;
_33 = [251_u8,183_u8,172_u8,156_u8];
_29 = !_17;
_12 = _9 != _6;
_2 = (-75_i8) as u128;
_21 = '\u{53bd8}';
_35 = _33;
_32 = [(-75_i8),(-15_i8),4_i8,101_i8,18_i8,92_i8,120_i8];
_10 = _17 > _29;
match _27 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb7,
340282366920938463463374607431768187023 => bb9,
_ => bb8
}
}
bb7 = {
_16 = [25325_i16,(-15442_i16),(-12564_i16)];
_12 = _6;
_15 = [129_u8,249_u8,140_u8,2_u8,206_u8,43_u8];
_15 = [226_u8,251_u8,80_u8,0_u8,116_u8,100_u8];
_17 = _11;
_13 = _11;
_2 = _1 ^ _17;
_9 = _2 == _2;
_7 = _2 + _2;
_17 = !_2;
_18 = 2064119085_i32;
_14 = _18 as f64;
_8 = _18 as f32;
_14 = -_4;
_3 = !_9;
_16 = [1180_i16,31607_i16,24279_i16];
_7 = _1 >> _2;
_18 = _8 as i32;
_4 = _14;
_17 = _2 & _2;
_16 = [(-25525_i16),22544_i16,(-16794_i16)];
_9 = _3;
_3 = _9;
_8 = 143_u8 as f32;
_5 = 52_u8 as u128;
Goto(bb3)
}
bb8 = {
_16 = [25325_i16,(-15442_i16),(-12564_i16)];
_12 = _6;
_15 = [129_u8,249_u8,140_u8,2_u8,206_u8,43_u8];
_15 = [226_u8,251_u8,80_u8,0_u8,116_u8,100_u8];
_17 = _11;
_13 = _11;
_2 = _1 ^ _17;
_9 = _2 == _2;
_7 = _2 + _2;
_17 = !_2;
_18 = 2064119085_i32;
_14 = _18 as f64;
_8 = _18 as f32;
_14 = -_4;
_3 = !_9;
_16 = [1180_i16,31607_i16,24279_i16];
_7 = _1 >> _2;
_18 = _8 as i32;
_4 = _14;
_17 = _2 & _2;
_16 = [(-25525_i16),22544_i16,(-16794_i16)];
_9 = _3;
_3 = _9;
_8 = 143_u8 as f32;
_5 = 52_u8 as u128;
Goto(bb3)
}
bb9 = {
_31 = [(-4168149057730375294_i64),(-6804289538984422670_i64)];
match _19 {
0 => bb5,
1 => bb10,
2 => bb11,
64968 => bb13,
_ => bb12
}
}
bb10 = {
_16 = [25325_i16,(-15442_i16),(-12564_i16)];
_12 = _6;
_15 = [129_u8,249_u8,140_u8,2_u8,206_u8,43_u8];
_15 = [226_u8,251_u8,80_u8,0_u8,116_u8,100_u8];
_17 = _11;
_13 = _11;
_2 = _1 ^ _17;
_9 = _2 == _2;
_7 = _2 + _2;
_17 = !_2;
_18 = 2064119085_i32;
_14 = _18 as f64;
_8 = _18 as f32;
_14 = -_4;
_3 = !_9;
_16 = [1180_i16,31607_i16,24279_i16];
_7 = _1 >> _2;
_18 = _8 as i32;
_4 = _14;
_17 = _2 & _2;
_16 = [(-25525_i16),22544_i16,(-16794_i16)];
_9 = _3;
_3 = _9;
_8 = 143_u8 as f32;
_5 = 52_u8 as u128;
Goto(bb3)
}
bb11 = {
_15 = [167_u8,60_u8,76_u8,76_u8,24_u8,64_u8];
_21 = '\u{c529c}';
_15 = [0_u8,89_u8,140_u8,149_u8,170_u8,165_u8];
Call(_22 = core::intrinsics::bswap(109_isize), ReturnTo(bb6), UnwindUnreachable())
}
bb12 = {
_16 = [25325_i16,(-15442_i16),(-12564_i16)];
_12 = _6;
_15 = [129_u8,249_u8,140_u8,2_u8,206_u8,43_u8];
_15 = [226_u8,251_u8,80_u8,0_u8,116_u8,100_u8];
_17 = _11;
_13 = _11;
_2 = _1 ^ _17;
_9 = _2 == _2;
_7 = _2 + _2;
_17 = !_2;
_18 = 2064119085_i32;
_14 = _18 as f64;
_8 = _18 as f32;
_14 = -_4;
_3 = !_9;
_16 = [1180_i16,31607_i16,24279_i16];
_7 = _1 >> _2;
_18 = _8 as i32;
_4 = _14;
_17 = _2 & _2;
_16 = [(-25525_i16),22544_i16,(-16794_i16)];
_9 = _3;
_3 = _9;
_8 = 143_u8 as f32;
_5 = 52_u8 as u128;
Goto(bb3)
}
bb13 = {
_19 = _22 as u16;
Goto(bb14)
}
bb14 = {
_19 = 2480_u16;
_14 = 208_u8 as f64;
_14 = _17 as f64;
_35 = _33;
_3 = !_9;
_34 = _23;
_24 = _23;
_28 = 7_usize | 4_usize;
_8 = _28 as f32;
_32 = [35_i8,66_i8,(-69_i8),(-24_i8),2_i8,33_i8,99_i8];
_21 = '\u{acc05}';
_29 = _25;
_36 = [7991502023480984819_u64,6940239882049515682_u64,10811641489155154697_u64];
_37 = _21;
_9 = _6 | _10;
_41 = (_10, _37, _16, _29);
_41.1 = _21;
_15 = [57_u8,168_u8,34_u8,51_u8,229_u8,245_u8];
_38 = _41.1 as isize;
_41.1 = _21;
Goto(bb15)
}
bb15 = {
_9 = !_41.0;
_37 = _21;
_18 = 10968577127311761792_u64 as i32;
_4 = _14;
_42 = !59_i8;
_7 = _41.3;
_11 = !_25;
_32 = [_42,_42,_42,_42,_42,_42,_42];
_41.1 = _21;
_10 = _6;
_20 = [_19,_19];
_37 = _21;
_30 = [_21,_41.1,_41.1,_37];
_41.2 = _16;
_4 = _14;
_6 = _9;
_11 = 6129371916454823504_u64 as u128;
match _19 {
0 => bb13,
2480 => bb17,
_ => bb16
}
}
bb16 = {
_15 = [167_u8,60_u8,76_u8,76_u8,24_u8,64_u8];
_21 = '\u{c529c}';
_15 = [0_u8,89_u8,140_u8,149_u8,170_u8,165_u8];
Call(_22 = core::intrinsics::bswap(109_isize), ReturnTo(bb6), UnwindUnreachable())
}
bb17 = {
_34 = _24;
_46.1 = _41.1;
_46.0 = _3;
_49.1.2.0 = core::ptr::addr_of_mut!(_18);
_47.fld0 = !3246726986_u32;
_49.0 = core::ptr::addr_of!(_22);
_5 = _17;
_12 = !_41.0;
_47.fld4 = Adt43 { fld0: _32,fld1: _49.0,fld2: _38,fld3: _42,fld4: _16 };
_3 = _9;
_49.1.6.0 = !_6;
_28 = 1395672778242155436_i64 as usize;
_47.fld1 = _18 as usize;
_49.1.6 = (_9, _41.1, _41.2, _25);
_44 = _36;
_47.fld2 = [10_u8,7_u8,78_u8,242_u8,221_u8,244_u8];
_49.1.1 = 134_u8 & 100_u8;
_13 = _29 >> _49.1.6.3;
_55 = _19;
_46.1 = _37;
Call(_51.0.1 = fn8(_9, _9, _10, _9, _41.3, _49.1.6.0, _6, _6, _49.1.6.3, _41, _5, _49.1.6.0, _41.0, _46.0, _12), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
_51.2 = [_51.0.1,_51.0.1,_51.0.1];
_49.1.3 = _49.1.2;
match _27 {
0 => bb14,
1 => bb16,
340282366920938463463374607431768187023 => bb20,
_ => bb19
}
}
bb19 = {
_16 = [25325_i16,(-15442_i16),(-12564_i16)];
_12 = _6;
_15 = [129_u8,249_u8,140_u8,2_u8,206_u8,43_u8];
_15 = [226_u8,251_u8,80_u8,0_u8,116_u8,100_u8];
_17 = _11;
_13 = _11;
_2 = _1 ^ _17;
_9 = _2 == _2;
_7 = _2 + _2;
_17 = !_2;
_18 = 2064119085_i32;
_14 = _18 as f64;
_8 = _18 as f32;
_14 = -_4;
_3 = !_9;
_16 = [1180_i16,31607_i16,24279_i16];
_7 = _1 >> _2;
_18 = _8 as i32;
_4 = _14;
_17 = _2 & _2;
_16 = [(-25525_i16),22544_i16,(-16794_i16)];
_9 = _3;
_3 = _9;
_8 = 143_u8 as f32;
_5 = 52_u8 as u128;
Goto(bb3)
}
bb20 = {
_48 = _27;
_29 = _17;
_41.3 = _17;
_38 = _28 as isize;
_46.1 = _37;
_41.2 = [_48,_27,_27];
_25 = _41.3;
_8 = -_23;
_45 = [_49.1.1,_49.1.1,_49.1.1,_49.1.1,_49.1.1,_49.1.1];
_36 = _51.2;
_41.1 = _49.1.6.1;
_32 = _47.fld4.fld0;
_49.1.4 = _48 * _48;
_47.fld1 = (-4952595276027429581_i64) as usize;
_41.3 = _7 ^ _25;
Goto(bb21)
}
bb21 = {
_41.0 = _13 != _17;
_37 = _21;
_47.fld4.fld1 = core::ptr::addr_of!(_50);
_47.fld4.fld0 = [_47.fld4.fld3,_47.fld4.fld3,_42,_47.fld4.fld3,_42,_42,_42];
_46 = _49.1.6;
RET = core::ptr::addr_of!(_53);
_55 = !_19;
_38 = _47.fld4.fld2 + _47.fld4.fld2;
Goto(bb22)
}
bb22 = {
Call(_58 = dump_var(7_usize, 2_usize, Move(_2), 12_usize, Move(_12), 38_usize, Move(_38), 37_usize, Move(_37)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_58 = dump_var(7_usize, 17_usize, Move(_17), 22_usize, Move(_22), 20_usize, Move(_20), 35_usize, Move(_35)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_58 = dump_var(7_usize, 9_usize, Move(_9), 19_usize, Move(_19), 42_usize, Move(_42), 15_usize, Move(_15)), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Call(_58 = dump_var(7_usize, 36_usize, Move(_36), 18_usize, Move(_18), 45_usize, Move(_45), 33_usize, Move(_33)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_58 = dump_var(7_usize, 29_usize, Move(_29), 28_usize, Move(_28), 7_usize, Move(_7), 59_usize, _59), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: u128,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: u128,mut _10: (bool, char, [i16; 3], u128),mut _11: u128,mut _12: bool,mut _13: bool,mut _14: bool,mut _15: bool) -> u64 {
mir! {
type RET = u64;
let _16: *mut i32;
let _17: isize;
let _18: char;
let _19: f32;
let _20: (i32, (*mut i32,), [u16; 2]);
let _21: char;
let _22: f32;
let _23: u8;
let _24: i64;
let _25: f32;
let _26: *const i128;
let _27: Adt48;
let _28: isize;
let _29: u128;
let _30: (u16,);
let _31: [i16; 3];
let _32: [u8; 4];
let _33: char;
let _34: char;
let _35: f64;
let _36: u8;
let _37: u128;
let _38: f64;
let _39: ();
let _40: ();
{
_11 = _10.3 + _10.3;
_1 = _7 > _15;
_7 = _12 <= _8;
_10.2 = [(-7143_i16),11549_i16,16877_i16];
_15 = _12 <= _7;
_15 = _2 == _7;
_8 = !_14;
_10.0 = _2 ^ _7;
_5 = _11;
_13 = _10.0;
_13 = !_15;
_11 = !_9;
RET = 10944791726636277370_usize as u64;
_8 = _3;
_15 = _8 == _3;
_6 = _15 >= _7;
RET = 18069375390501667124_usize as u64;
_10.1 = '\u{99733}';
_10.3 = _9 >> _11;
_13 = _12;
_7 = _12;
_1 = _8;
Call(_11 = core::intrinsics::transmute(_9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 8008636711991407890_u64 | 1643535183374546360_u64;
_10.0 = _12 ^ _2;
_8 = _5 < _10.3;
_15 = !_7;
RET = 1395030827953716354_u64 ^ 1006028305703400115_u64;
RET = !6694824300013598660_u64;
_17 = -(-24_isize);
_8 = !_10.0;
RET = _10.1 as u64;
_6 = _10.0 & _10.0;
_2 = _15 == _14;
_20.0 = !424200556_i32;
_21 = _10.1;
_18 = _21;
_12 = _3;
_10.0 = _10.3 >= _11;
_16 = core::ptr::addr_of_mut!(_20.0);
_9 = _3 as u128;
_20.1.0 = _16;
_13 = !_8;
Goto(bb2)
}
bb2 = {
_6 = _10.0;
_10.2 = [(-24060_i16),(-669_i16),28216_i16];
_11 = !_10.3;
_20.2 = [30829_u16,45848_u16];
_5 = !_11;
_1 = _2 >= _3;
_2 = _13 | _1;
_14 = !_13;
_11 = _9 + _10.3;
_10.1 = _21;
_8 = _7;
_2 = !_15;
_18 = _10.1;
_5 = _11;
_13 = _1;
_14 = _13;
_23 = 179_u8 - 134_u8;
_22 = 33140_u16 as f32;
_7 = _14 == _8;
_20.1.0 = _16;
_16 = core::ptr::addr_of_mut!((*_16));
_23 = 4_u8;
Goto(bb3)
}
bb3 = {
_23 = !235_u8;
_25 = _22;
RET = 5194642704596333940_u64 << _11;
_10.1 = _18;
_10.1 = _21;
_20.1 = (_16,);
_6 = _1;
_20.1.0 = _16;
_5 = 4670381526202937814_i64 as u128;
_21 = _10.1;
_16 = core::ptr::addr_of_mut!((*_16));
_15 = _7 | _14;
_23 = !26_u8;
_5 = _11;
_10.0 = _14;
_2 = _1 < _14;
_7 = _12;
_19 = (*_16) as f32;
_8 = !_7;
_6 = _8;
RET = 4581116995797304033_u64;
Goto(bb4)
}
bb4 = {
_6 = _10.0 | _1;
_13 = _4;
_2 = _6;
_24 = _22 as i64;
_2 = _13;
_19 = -_25;
_20.1 = (_16,);
match RET {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
4581116995797304033 => bb13,
_ => bb12
}
}
bb5 = {
_23 = !235_u8;
_25 = _22;
RET = 5194642704596333940_u64 << _11;
_10.1 = _18;
_10.1 = _21;
_20.1 = (_16,);
_6 = _1;
_20.1.0 = _16;
_5 = 4670381526202937814_i64 as u128;
_21 = _10.1;
_16 = core::ptr::addr_of_mut!((*_16));
_15 = _7 | _14;
_23 = !26_u8;
_5 = _11;
_10.0 = _14;
_2 = _1 < _14;
_7 = _12;
_19 = (*_16) as f32;
_8 = !_7;
_6 = _8;
RET = 4581116995797304033_u64;
Goto(bb4)
}
bb6 = {
_6 = _10.0;
_10.2 = [(-24060_i16),(-669_i16),28216_i16];
_11 = !_10.3;
_20.2 = [30829_u16,45848_u16];
_5 = !_11;
_1 = _2 >= _3;
_2 = _13 | _1;
_14 = !_13;
_11 = _9 + _10.3;
_10.1 = _21;
_8 = _7;
_2 = !_15;
_18 = _10.1;
_5 = _11;
_13 = _1;
_14 = _13;
_23 = 179_u8 - 134_u8;
_22 = 33140_u16 as f32;
_7 = _14 == _8;
_20.1.0 = _16;
_16 = core::ptr::addr_of_mut!((*_16));
_23 = 4_u8;
Goto(bb3)
}
bb7 = {
RET = 8008636711991407890_u64 | 1643535183374546360_u64;
_10.0 = _12 ^ _2;
_8 = _5 < _10.3;
_15 = !_7;
RET = 1395030827953716354_u64 ^ 1006028305703400115_u64;
RET = !6694824300013598660_u64;
_17 = -(-24_isize);
_8 = !_10.0;
RET = _10.1 as u64;
_6 = _10.0 & _10.0;
_2 = _15 == _14;
_20.0 = !424200556_i32;
_21 = _10.1;
_18 = _21;
_12 = _3;
_10.0 = _10.3 >= _11;
_16 = core::ptr::addr_of_mut!(_20.0);
_9 = _3 as u128;
_20.1.0 = _16;
_13 = !_8;
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
Return()
}
bb13 = {
_11 = _5 + _9;
_24 = (-7828610897351310104_i64);
(*_16) = _11 as i32;
_1 = !_13;
_22 = -_19;
_20.1.0 = core::ptr::addr_of_mut!(_20.0);
_30.0 = _13 as u16;
_31 = [17674_i16,20839_i16,(-8830_i16)];
_20.0 = _25 as i32;
_33 = _18;
_20.0 = _30.0 as i32;
_16 = core::ptr::addr_of_mut!((*_16));
_31 = _10.2;
_10.3 = (-5436795296464519441160375700345182596_i128) as u128;
_4 = _10.0;
_10 = (_13, _18, _31, _11);
_11 = !_5;
_3 = !_15;
_6 = _2;
RET = 11302830795628017892_u64 - 11641456671973327927_u64;
Goto(bb14)
}
bb14 = {
_31 = [(-11683_i16),3962_i16,16656_i16];
_30.0 = !62065_u16;
_21 = _18;
_29 = _9 | _10.3;
_6 = !_3;
_24 = _21 as i64;
_2 = _3;
_17 = 9223372036854775807_isize;
_20.2 = [_30.0,_30.0];
_7 = !_14;
_32 = [_23,_23,_23,_23];
_28 = _30.0 as isize;
_18 = _10.1;
_30 = (26162_u16,);
(*_16) = 345578865_i32;
_17 = _28;
_20.0 = _8 as i32;
_11 = !_9;
RET = 4091785742907635009_u64 & 17286654670345940550_u64;
_23 = 53_u8;
_30.0 = 52668_u16;
RET = _14 as u64;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(8_usize, 29_usize, Move(_29), 21_usize, Move(_21), 10_usize, Move(_10), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(8_usize, 13_usize, Move(_13), 28_usize, Move(_28), 3_usize, Move(_3), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(8_usize, 7_usize, Move(_7), 8_usize, Move(_8), 33_usize, Move(_33), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_39 = dump_var(8_usize, 14_usize, Move(_14), 40_usize, _40, 40_usize, _40, 40_usize, _40), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: u128,mut _2: u128,mut _3: u128,mut _4: bool,mut _5: u128,mut _6: u128,mut _7: u128,mut _8: u128) -> Adt45 {
mir! {
type RET = Adt45;
let _9: Adt55;
let _10: (*const char, u64, i32);
let _11: u16;
let _12: usize;
let _13: u32;
let _14: [char; 4];
let _15: i64;
let _16: bool;
let _17: u128;
let _18: f64;
let _19: [u8; 6];
let _20: u64;
let _21: (u8, (i32, (*mut i32,), [u16; 2]));
let _22: Adt58;
let _23: (bool, char, [i16; 3], u128);
let _24: char;
let _25: [u64; 3];
let _26: (bool, char, [i16; 3], u128);
let _27: Adt49;
let _28: isize;
let _29: i64;
let _30: f64;
let _31: *const isize;
let _32: [u8; 6];
let _33: ();
let _34: ();
{
_5 = !_1;
_7 = 1950492691_u32 as u128;
_6 = _1;
_4 = false;
_6 = _2 & _1;
_9 = Adt55::Variant1 { fld0: 16314184250707863721828484612749580183_i128 };
Call(place!(Field::<i128>(Variant(_9, 1), 0)) = fn10(_5, _7, _1, _1, _3, _1, _1, _6, _8, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = Adt55::Variant1 { fld0: (-31324107521303139995609913551120917302_i128) };
_4 = _8 <= _5;
place!(Field::<i128>(Variant(_9, 1), 0)) = 61449851923329956616384271739086169621_i128;
_10.1 = 4916688279999598372_u64 + 6690858874848302270_u64;
_7 = (-9223372036854775808_isize) as u128;
_4 = true;
_2 = _5;
_10.1 = !10144321006962729982_u64;
_9 = Adt55::Variant1 { fld0: 60427828722041021145318474583294477198_i128 };
_10.2 = 1252293785_i32 ^ (-1708124807_i32);
_2 = _6 ^ _8;
_2 = _5 | _1;
_2 = !_1;
_1 = _5 >> _2;
_10.2 = (-1655021845_i32) >> _1;
_5 = _1;
_6 = _5;
Call(RET = fn12(_10.2, _5, _1, _5, _6, _2, _5, _5, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = _6;
_10.1 = !9188995876990300079_u64;
_11 = !57365_u16;
_8 = _3;
place!(Field::<[i8; 7]>(Variant(RET, 0), 0)) = [(-9_i8),9_i8,(-120_i8),95_i8,(-16_i8),(-108_i8),(-42_i8)];
_1 = !_6;
_13 = 3412895660_u32;
_17 = '\u{32af2}' as u128;
_15 = (-91940461007059622614813976815714733807_i128) as i64;
_4 = !true;
_12 = 11145205729804142777_usize;
_7 = _2 * _5;
_14 = ['\u{35dba}','\u{4d8fa}','\u{eec64}','\u{ab4bf}'];
_16 = !_4;
_10.2 = (-105_isize) as i32;
_2 = !_6;
_18 = (-30202_i16) as f64;
_11 = 1349_u16;
_8 = !_6;
_16 = _8 == _6;
Call(_15 = fn14(_5, _8, _7, _5, _8, _16, _8, _2, _16, _5, _16, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = !_3;
_4 = !_16;
_2 = !_5;
_18 = _5 as f64;
_10.1 = 7899754690345035064_u64;
_1 = _6;
place!(Field::<i128>(Variant(_9, 1), 0)) = (-23049029219767099367739496994563631711_i128);
_7 = _8 >> _3;
_2 = 115_i8 as u128;
_3 = _6 << _7;
_5 = _1 ^ _1;
_3 = _16 as u128;
_16 = _6 == _8;
_11 = 8952_u16 << _3;
_3 = _1;
place!(Field::<i128>(Variant(_9, 1), 0)) = !(-53866499033973281188069333163282721250_i128);
Goto(bb4)
}
bb4 = {
_6 = _5;
SetDiscriminant(RET, 1);
SetDiscriminant(_9, 1);
place!(Field::<[i16; 3]>(Variant(RET, 1), 3)) = [18237_i16,(-12794_i16),19327_i16];
match _12 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
11145205729804142777 => bb8,
_ => bb7
}
}
bb5 = {
_7 = !_3;
_4 = !_16;
_2 = !_5;
_18 = _5 as f64;
_10.1 = 7899754690345035064_u64;
_1 = _6;
place!(Field::<i128>(Variant(_9, 1), 0)) = (-23049029219767099367739496994563631711_i128);
_7 = _8 >> _3;
_2 = 115_i8 as u128;
_3 = _6 << _7;
_5 = _1 ^ _1;
_3 = _16 as u128;
_16 = _6 == _8;
_11 = 8952_u16 << _3;
_3 = _1;
place!(Field::<i128>(Variant(_9, 1), 0)) = !(-53866499033973281188069333163282721250_i128);
Goto(bb4)
}
bb6 = {
_5 = _6;
_10.1 = !9188995876990300079_u64;
_11 = !57365_u16;
_8 = _3;
place!(Field::<[i8; 7]>(Variant(RET, 0), 0)) = [(-9_i8),9_i8,(-120_i8),95_i8,(-16_i8),(-108_i8),(-42_i8)];
_1 = !_6;
_13 = 3412895660_u32;
_17 = '\u{32af2}' as u128;
_15 = (-91940461007059622614813976815714733807_i128) as i64;
_4 = !true;
_12 = 11145205729804142777_usize;
_7 = _2 * _5;
_14 = ['\u{35dba}','\u{4d8fa}','\u{eec64}','\u{ab4bf}'];
_16 = !_4;
_10.2 = (-105_isize) as i32;
_2 = !_6;
_18 = (-30202_i16) as f64;
_11 = 1349_u16;
_8 = !_6;
_16 = _8 == _6;
Call(_15 = fn14(_5, _8, _7, _5, _8, _16, _8, _2, _16, _5, _16, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_9 = Adt55::Variant1 { fld0: (-31324107521303139995609913551120917302_i128) };
_4 = _8 <= _5;
place!(Field::<i128>(Variant(_9, 1), 0)) = 61449851923329956616384271739086169621_i128;
_10.1 = 4916688279999598372_u64 + 6690858874848302270_u64;
_7 = (-9223372036854775808_isize) as u128;
_4 = true;
_2 = _5;
_10.1 = !10144321006962729982_u64;
_9 = Adt55::Variant1 { fld0: 60427828722041021145318474583294477198_i128 };
_10.2 = 1252293785_i32 ^ (-1708124807_i32);
_2 = _6 ^ _8;
_2 = _5 | _1;
_2 = !_1;
_1 = _5 >> _2;
_10.2 = (-1655021845_i32) >> _1;
_5 = _1;
_6 = _5;
Call(RET = fn12(_10.2, _5, _1, _5, _6, _2, _5, _5, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
place!(Field::<*mut isize>(Variant(RET, 1), 5)) = core::ptr::addr_of_mut!(place!(Field::<isize>(Variant(RET, 1), 2)));
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 6)).0 = _16 & _16;
place!(Field::<[u64; 3]>(Variant(RET, 1), 4)) = [_10.1,_10.1,_10.1];
place!(Field::<(*mut i32,)>(Variant(RET, 1), 1)).0 = core::ptr::addr_of_mut!(_21.1.0);
place!(Field::<(*mut i32,)>(Variant(RET, 1), 1)).0 = core::ptr::addr_of_mut!(_10.2);
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 6)) = (_4, '\u{32b9a}', Field::<[i16; 3]>(Variant(RET, 1), 3), _3);
_6 = 9223372036854775807_isize as u128;
place!(Field::<*const isize>(Variant(RET, 1), 0)) = core::ptr::addr_of!(place!(Field::<isize>(Variant(RET, 1), 2)));
_9 = Adt55::Variant1 { fld0: (-46028933738649272651144241227228609315_i128) };
_21.1.1 = Field::<(*mut i32,)>(Variant(RET, 1), 1);
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 6)) = (_4, '\u{30887}', Field::<[i16; 3]>(Variant(RET, 1), 3), _3);
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 6)).0 = _16;
match _15 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb9,
5 => bb10,
340282366920938463458155737652861136695 => bb12,
_ => bb11
}
}
bb9 = {
_6 = _5;
SetDiscriminant(RET, 1);
SetDiscriminant(_9, 1);
place!(Field::<[i16; 3]>(Variant(RET, 1), 3)) = [18237_i16,(-12794_i16),19327_i16];
match _12 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
11145205729804142777 => bb8,
_ => bb7
}
}
bb10 = {
_5 = _6;
_10.1 = !9188995876990300079_u64;
_11 = !57365_u16;
_8 = _3;
place!(Field::<[i8; 7]>(Variant(RET, 0), 0)) = [(-9_i8),9_i8,(-120_i8),95_i8,(-16_i8),(-108_i8),(-42_i8)];
_1 = !_6;
_13 = 3412895660_u32;
_17 = '\u{32af2}' as u128;
_15 = (-91940461007059622614813976815714733807_i128) as i64;
_4 = !true;
_12 = 11145205729804142777_usize;
_7 = _2 * _5;
_14 = ['\u{35dba}','\u{4d8fa}','\u{eec64}','\u{ab4bf}'];
_16 = !_4;
_10.2 = (-105_isize) as i32;
_2 = !_6;
_18 = (-30202_i16) as f64;
_11 = 1349_u16;
_8 = !_6;
_16 = _8 == _6;
Call(_15 = fn14(_5, _8, _7, _5, _8, _16, _8, _2, _16, _5, _16, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_5 = _6;
_10.1 = !9188995876990300079_u64;
_11 = !57365_u16;
_8 = _3;
place!(Field::<[i8; 7]>(Variant(RET, 0), 0)) = [(-9_i8),9_i8,(-120_i8),95_i8,(-16_i8),(-108_i8),(-42_i8)];
_1 = !_6;
_13 = 3412895660_u32;
_17 = '\u{32af2}' as u128;
_15 = (-91940461007059622614813976815714733807_i128) as i64;
_4 = !true;
_12 = 11145205729804142777_usize;
_7 = _2 * _5;
_14 = ['\u{35dba}','\u{4d8fa}','\u{eec64}','\u{ab4bf}'];
_16 = !_4;
_10.2 = (-105_isize) as i32;
_2 = !_6;
_18 = (-30202_i16) as f64;
_11 = 1349_u16;
_8 = !_6;
_16 = _8 == _6;
Call(_15 = fn14(_5, _8, _7, _5, _8, _16, _8, _2, _16, _5, _16, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb12 = {
place!(Field::<*mut isize>(Variant(RET, 1), 5)) = core::ptr::addr_of_mut!(place!(Field::<isize>(Variant(RET, 1), 2)));
_1 = !Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 6).3;
_16 = !_4;
_13 = _10.2 as u32;
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 6)) = (_16, '\u{3d1e8}', Field::<[i16; 3]>(Variant(RET, 1), 3), _8);
_10.2 = 1142183394_i32;
_19 = [253_u8,254_u8,130_u8,73_u8,187_u8,146_u8];
_21.0 = !174_u8;
_11 = _10.2 as u16;
place!(Field::<*mut isize>(Variant(RET, 1), 5)) = core::ptr::addr_of_mut!(place!(Field::<isize>(Variant(RET, 1), 2)));
place!(Field::<*mut isize>(Variant(RET, 1), 5)) = core::ptr::addr_of_mut!(place!(Field::<isize>(Variant(RET, 1), 2)));
Call(place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 6)) = fn16(_8, _3, _7, _16, _7), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_5 = (-37_i8) as u128;
_26.3 = !_7;
_25 = [_10.1,_10.1,_10.1];
_20 = _10.1 & _10.1;
_28 = 25_isize;
place!(Field::<*const isize>(Variant(RET, 1), 0)) = core::ptr::addr_of!(_28);
place!(Field::<*mut isize>(Variant(RET, 1), 5)) = core::ptr::addr_of_mut!(_28);
_23.0 = _16;
_30 = -_18;
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 6)).2 = Field::<[i16; 3]>(Variant(RET, 1), 3);
_1 = !_3;
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 6)).2 = Field::<[i16; 3]>(Variant(RET, 1), 3);
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 6)).2 = [(-31248_i16),(-31736_i16),(-1691_i16)];
_23.2 = [(-11951_i16),(-9302_i16),25284_i16];
place!(Field::<(*mut i32,)>(Variant(RET, 1), 1)) = (_21.1.1.0,);
_26.2 = _23.2;
_23.3 = _12 as u128;
place!(Field::<i128>(Variant(_9, 1), 0)) = 21736112032666785558272153473094854840_i128 >> _3;
match _10.2 {
1142183394 => bb14,
_ => bb3
}
}
bb14 = {
_16 = !_23.0;
_2 = Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 6).3;
place!(Field::<*mut isize>(Variant(RET, 1), 5)) = core::ptr::addr_of_mut!(_28);
_21.1.1 = Field::<(*mut i32,)>(Variant(RET, 1), 1);
SetDiscriminant(_9, 0);
_29 = !_15;
place!(Field::<*mut isize>(Variant(_9, 0), 0)) = Field::<*mut isize>(Variant(RET, 1), 5);
_10.0 = core::ptr::addr_of!(place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 6)).1);
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 6)).3 = _2;
_23.1 = Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 6).1;
place!(Field::<((*const char, u64, i32), i128, [u64; 3], i16)>(Variant(_9, 0), 5)).0.1 = !_20;
place!(Field::<((*const char, u64, i32), i128, [u64; 3], i16)>(Variant(_9, 0), 5)).1 = (-28838746609017710063783243271576416473_i128) + 35818802339393553627962770173425038340_i128;
place!(Field::<((*const char, u64, i32), i128, [u64; 3], i16)>(Variant(_9, 0), 5)).2 = [_20,Field::<((*const char, u64, i32), i128, [u64; 3], i16)>(Variant(_9, 0), 5).0.1,Field::<((*const char, u64, i32), i128, [u64; 3], i16)>(Variant(_9, 0), 5).0.1];
_26.2 = [12934_i16,(-13591_i16),8810_i16];
place!(Field::<((*const char, u64, i32), i128, [u64; 3], i16)>(Variant(_9, 0), 5)).0.2 = !_10.2;
place!(Field::<((*const char, u64, i32), i128, [u64; 3], i16)>(Variant(_9, 0), 5)).2 = _25;
_15 = _29 - _29;
match _10.1 {
0 => bb1,
1 => bb11,
2 => bb12,
3 => bb6,
4 => bb15,
5 => bb16,
7899754690345035064 => bb18,
_ => bb17
}
}
bb15 = {
place!(Field::<*mut isize>(Variant(RET, 1), 5)) = core::ptr::addr_of_mut!(place!(Field::<isize>(Variant(RET, 1), 2)));
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 6)).0 = _16 & _16;
place!(Field::<[u64; 3]>(Variant(RET, 1), 4)) = [_10.1,_10.1,_10.1];
place!(Field::<(*mut i32,)>(Variant(RET, 1), 1)).0 = core::ptr::addr_of_mut!(_21.1.0);
place!(Field::<(*mut i32,)>(Variant(RET, 1), 1)).0 = core::ptr::addr_of_mut!(_10.2);
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 6)) = (_4, '\u{32b9a}', Field::<[i16; 3]>(Variant(RET, 1), 3), _3);
_6 = 9223372036854775807_isize as u128;
place!(Field::<*const isize>(Variant(RET, 1), 0)) = core::ptr::addr_of!(place!(Field::<isize>(Variant(RET, 1), 2)));
_9 = Adt55::Variant1 { fld0: (-46028933738649272651144241227228609315_i128) };
_21.1.1 = Field::<(*mut i32,)>(Variant(RET, 1), 1);
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 6)) = (_4, '\u{30887}', Field::<[i16; 3]>(Variant(RET, 1), 3), _3);
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 6)).0 = _16;
match _15 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb9,
5 => bb10,
340282366920938463458155737652861136695 => bb12,
_ => bb11
}
}
bb16 = {
_5 = _6;
_10.1 = !9188995876990300079_u64;
_11 = !57365_u16;
_8 = _3;
place!(Field::<[i8; 7]>(Variant(RET, 0), 0)) = [(-9_i8),9_i8,(-120_i8),95_i8,(-16_i8),(-108_i8),(-42_i8)];
_1 = !_6;
_13 = 3412895660_u32;
_17 = '\u{32af2}' as u128;
_15 = (-91940461007059622614813976815714733807_i128) as i64;
_4 = !true;
_12 = 11145205729804142777_usize;
_7 = _2 * _5;
_14 = ['\u{35dba}','\u{4d8fa}','\u{eec64}','\u{ab4bf}'];
_16 = !_4;
_10.2 = (-105_isize) as i32;
_2 = !_6;
_18 = (-30202_i16) as f64;
_11 = 1349_u16;
_8 = !_6;
_16 = _8 == _6;
Call(_15 = fn14(_5, _8, _7, _5, _8, _16, _8, _2, _16, _5, _16, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_6 = _5;
SetDiscriminant(RET, 1);
SetDiscriminant(_9, 1);
place!(Field::<[i16; 3]>(Variant(RET, 1), 3)) = [18237_i16,(-12794_i16),19327_i16];
match _12 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
11145205729804142777 => bb8,
_ => bb7
}
}
bb18 = {
_21.1.0 = -Field::<((*const char, u64, i32), i128, [u64; 3], i16)>(Variant(_9, 0), 5).0.2;
place!(Field::<isize>(Variant(RET, 1), 2)) = -_28;
place!(Field::<((*const char, u64, i32), i128, [u64; 3], i16)>(Variant(_9, 0), 5)).0.0 = _10.0;
Goto(bb19)
}
bb19 = {
Call(_33 = dump_var(9_usize, 29_usize, Move(_29), 5_usize, Move(_5), 28_usize, Move(_28), 6_usize, Move(_6)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_33 = dump_var(9_usize, 14_usize, Move(_14), 11_usize, Move(_11), 4_usize, Move(_4), 20_usize, Move(_20)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_33 = dump_var(9_usize, 23_usize, Move(_23), 1_usize, Move(_1), 34_usize, _34, 34_usize, _34), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: u128,mut _2: u128,mut _3: u128,mut _4: u128,mut _5: u128,mut _6: u128,mut _7: u128,mut _8: u128,mut _9: u128,mut _10: u128) -> i128 {
mir! {
type RET = i128;
let _11: i16;
let _12: isize;
let _13: isize;
let _14: bool;
let _15: char;
let _16: [u64; 3];
let _17: f32;
let _18: i64;
let _19: char;
let _20: Adt55;
let _21: [i16; 3];
let _22: char;
let _23: [u8; 6];
let _24: i16;
let _25: i128;
let _26: [u8; 4];
let _27: bool;
let _28: Adt48;
let _29: f64;
let _30: u64;
let _31: bool;
let _32: isize;
let _33: char;
let _34: ();
let _35: ();
{
RET = 13596572746682042412_usize as i128;
_9 = (-37_i8) as u128;
_8 = _1 - _7;
_2 = 8_u8 as u128;
_5 = !_8;
_2 = 54_isize as u128;
RET = !134181008479770824833112514718741679218_i128;
_11 = (-28594_i16) | 32654_i16;
_6 = _8 | _7;
_4 = _5;
_12 = -(-9223372036854775808_isize);
_1 = !_3;
RET = -(-136548446618875860478972361401462848259_i128);
_6 = _3 ^ _7;
_12 = 90_i8 as isize;
RET = 39458783982983663676601913565346943800_i128;
_1 = _4;
match RET {
0 => bb1,
39458783982983663676601913565346943800 => bb3,
_ => bb2
}
}
bb1 = {
Return()
}
bb2 = {
Return()
}
bb3 = {
_13 = !_12;
RET = !83487182892252744693775804021548339133_i128;
RET = (-59433300472386520863461071613033604993_i128) - 65735331074828563468110678951958060275_i128;
_11 = (-31428_i16);
_20 = Adt55::Variant1 { fld0: RET };
_2 = _10 - _8;
_16 = [14871057886920968878_u64,9900603139846453086_u64,1139293009356343093_u64];
_5 = _11 as u128;
_16 = [9046270691272207928_u64,3792351021376199675_u64,11908948423870426944_u64];
_17 = _13 as f32;
_1 = _2;
_19 = '\u{10c9e8}';
_13 = 139_u8 as isize;
SetDiscriminant(_20, 2);
match _11 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768180028 => bb8,
_ => bb7
}
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
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld4.fld0 = [(-113_i8),(-54_i8),(-21_i8),(-78_i8),86_i8,68_i8,(-118_i8)];
place!(Field::<[u8; 6]>(Variant(_20, 2), 4)) = [84_u8,43_u8,229_u8,113_u8,72_u8,25_u8];
_3 = _6;
_18 = -(-3126936625046579250_i64);
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld4.fld4 = [_11,_11,_11];
_12 = _13;
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld2 = [187_u8,41_u8,73_u8,227_u8,97_u8,141_u8];
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld4.fld4 = [_11,_11,_11];
_10 = 2317126582_u32 as u128;
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld0 = 3442957951_u32;
_15 = _19;
RET = 3319565167579287813450558373407737882_i128 + (-27005238652400425228009140494694698619_i128);
_22 = _15;
place!(Field::<i128>(Variant(_20, 2), 1)) = RET << Field::<Adt46>(Variant(_20, 2), 0).fld0;
RET = -Field::<i128>(Variant(_20, 2), 1);
_24 = _11 & _11;
_23 = [51_u8,21_u8,54_u8,245_u8,132_u8,82_u8];
_13 = Field::<i128>(Variant(_20, 2), 1) as isize;
_18 = -3468481824614281078_i64;
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld4.fld4 = [_24,_24,_24];
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld4.fld3 = _17 as i8;
place!(Field::<i128>(Variant(_20, 2), 1)) = 10959439546884185604_u64 as i128;
_15 = _19;
Call(_1 = fn11(_7, _4, _6, _2, _8, _8, _7, Field::<[u8; 6]>(Variant(_20, 2), 4), _6, _2, _3, _6), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
place!(Field::<i128>(Variant(_20, 2), 1)) = RET >> _6;
_16 = [6565475743169557601_u64,11700149348663698466_u64,7113154391983088820_u64];
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld4.fld1 = core::ptr::addr_of!(place!(Field::<Adt46>(Variant(_20, 2), 0)).fld4.fld2);
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld2 = [107_u8,103_u8,81_u8,37_u8,5_u8,214_u8];
place!(Field::<i128>(Variant(_20, 2), 1)) = RET;
_11 = _24;
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld1 = 7599870308846475867_usize * 1649272360858426017_usize;
match Field::<Adt46>(Variant(_20, 2), 0).fld0 {
0 => bb3,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
3442957951 => bb17,
_ => bb16
}
}
bb10 = {
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld4.fld0 = [(-113_i8),(-54_i8),(-21_i8),(-78_i8),86_i8,68_i8,(-118_i8)];
place!(Field::<[u8; 6]>(Variant(_20, 2), 4)) = [84_u8,43_u8,229_u8,113_u8,72_u8,25_u8];
_3 = _6;
_18 = -(-3126936625046579250_i64);
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld4.fld4 = [_11,_11,_11];
_12 = _13;
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld2 = [187_u8,41_u8,73_u8,227_u8,97_u8,141_u8];
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld4.fld4 = [_11,_11,_11];
_10 = 2317126582_u32 as u128;
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld0 = 3442957951_u32;
_15 = _19;
RET = 3319565167579287813450558373407737882_i128 + (-27005238652400425228009140494694698619_i128);
_22 = _15;
place!(Field::<i128>(Variant(_20, 2), 1)) = RET << Field::<Adt46>(Variant(_20, 2), 0).fld0;
RET = -Field::<i128>(Variant(_20, 2), 1);
_24 = _11 & _11;
_23 = [51_u8,21_u8,54_u8,245_u8,132_u8,82_u8];
_13 = Field::<i128>(Variant(_20, 2), 1) as isize;
_18 = -3468481824614281078_i64;
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld4.fld4 = [_24,_24,_24];
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld4.fld3 = _17 as i8;
place!(Field::<i128>(Variant(_20, 2), 1)) = 10959439546884185604_u64 as i128;
_15 = _19;
Call(_1 = fn11(_7, _4, _6, _2, _8, _8, _7, Field::<[u8; 6]>(Variant(_20, 2), 4), _6, _2, _3, _6), ReturnTo(bb9), UnwindUnreachable())
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
_13 = !_12;
RET = !83487182892252744693775804021548339133_i128;
RET = (-59433300472386520863461071613033604993_i128) - 65735331074828563468110678951958060275_i128;
_11 = (-31428_i16);
_20 = Adt55::Variant1 { fld0: RET };
_2 = _10 - _8;
_16 = [14871057886920968878_u64,9900603139846453086_u64,1139293009356343093_u64];
_5 = _11 as u128;
_16 = [9046270691272207928_u64,3792351021376199675_u64,11908948423870426944_u64];
_17 = _13 as f32;
_1 = _2;
_19 = '\u{10c9e8}';
_13 = 139_u8 as isize;
SetDiscriminant(_20, 2);
match _11 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768180028 => bb8,
_ => bb7
}
}
bb16 = {
Return()
}
bb17 = {
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld3 = [_19,_15,_15,_15];
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld4.fld3 = !127_i8;
_20 = Adt55::Variant1 { fld0: RET };
_26 = [8_u8,200_u8,45_u8,228_u8];
_24 = true as i16;
_12 = _13 << _6;
_9 = _6;
_21 = [_11,_11,_11];
_23 = [144_u8,224_u8,67_u8,16_u8,26_u8,201_u8];
SetDiscriminant(_20, 2);
_22 = _15;
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld1 = _17 as usize;
RET = 147423911740374130925884969626661175347_i128 - 41413930966275915139279989034097332458_i128;
_22 = _19;
_1 = _2;
place!(Field::<Adt46>(Variant(_20, 2), 0)).fld2 = [236_u8,20_u8,79_u8,208_u8,203_u8,91_u8];
_27 = !false;
_6 = 12803845947528766919_u64 as u128;
_16 = [13104639236471714937_u64,13609888075287206570_u64,6430407584368459633_u64];
_1 = _3;
_8 = !_2;
Goto(bb18)
}
bb18 = {
Call(_34 = dump_var(10_usize, 12_usize, Move(_12), 2_usize, Move(_2), 8_usize, Move(_8), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_34 = dump_var(10_usize, 7_usize, Move(_7), 9_usize, Move(_9), 26_usize, Move(_26), 15_usize, Move(_15)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_34 = dump_var(10_usize, 6_usize, Move(_6), 23_usize, Move(_23), 13_usize, Move(_13), 35_usize, _35), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: u128,mut _2: u128,mut _3: u128,mut _4: u128,mut _5: u128,mut _6: u128,mut _7: u128,mut _8: [u8; 6],mut _9: u128,mut _10: u128,mut _11: u128,mut _12: u128) -> u128 {
mir! {
type RET = u128;
let _13: f32;
let _14: (*const isize, (i32, u8, (*mut i32,), (*mut i32,), i16, bool, (bool, char, [i16; 3], u128)));
let _15: isize;
let _16: u64;
let _17: f64;
let _18: u16;
let _19: [char; 4];
let _20: ();
let _21: ();
{
_11 = 105_u8 as u128;
_12 = 1602148267_i32 as u128;
_4 = _11 | _10;
_5 = _3 >> _10;
_10 = !_7;
_10 = 4997_u16 as u128;
Goto(bb1)
}
bb1 = {
_14.1.4 = 7424_i16;
_4 = _7;
_14.1.6.2 = [_14.1.4,_14.1.4,_14.1.4];
_15 = 1293291849_i32 as isize;
RET = 101_u8 as u128;
_5 = !_9;
_13 = 403885435697932243_i64 as f32;
RET = _2;
_14.1.6.3 = _7 << _7;
_14.1.0 = '\u{da590}' as i32;
_14.1.1 = 89_u8 | 42_u8;
_10 = '\u{b423e}' as u128;
RET = !_2;
_14.1.6.0 = true;
_14.1.6.0 = false;
_11 = !_14.1.6.3;
_17 = _6 as f64;
_14.1.0 = 11611569992471469357_usize as i32;
_5 = _14.1.6.3;
Goto(bb2)
}
bb2 = {
Call(_20 = dump_var(11_usize, 9_usize, Move(_9), 3_usize, Move(_3), 2_usize, Move(_2), 15_usize, Move(_15)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_20 = dump_var(11_usize, 11_usize, Move(_11), 12_usize, Move(_12), 21_usize, _21, 21_usize, _21), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: i32,mut _2: u128,mut _3: u128,mut _4: u128,mut _5: u128,mut _6: u128,mut _7: u128,mut _8: u128,mut _9: u128) -> Adt45 {
mir! {
type RET = Adt45;
let _10: [u8; 4];
let _11: (i32, (*mut i32,), [u16; 2]);
let _12: bool;
let _13: u32;
let _14: *const isize;
let _15: i128;
let _16: i8;
let _17: [u8; 4];
let _18: i8;
let _19: u32;
let _20: [char; 4];
let _21: bool;
let _22: [i16; 3];
let _23: (*const char, u64, i32);
let _24: *const isize;
let _25: Adt45;
let _26: [char; 4];
let _27: [i16; 3];
let _28: f32;
let _29: Adt43;
let _30: isize;
let _31: isize;
let _32: Adt51;
let _33: [char; 4];
let _34: u16;
let _35: Adt57;
let _36: [i8; 7];
let _37: isize;
let _38: [u8; 6];
let _39: i128;
let _40: [u8; 4];
let _41: usize;
let _42: *mut [i16; 3];
let _43: isize;
let _44: u8;
let _45: [u64; 3];
let _46: f64;
let _47: [u64; 3];
let _48: ();
let _49: ();
{
_2 = _4 * _9;
_7 = _9 >> _3;
_10 = [242_u8,34_u8,178_u8,48_u8];
_5 = _4 << _4;
_4 = _6;
_3 = _7;
_6 = _2 + _8;
_1 = (-1271353001_i32);
_9 = (-9223372036854775808_isize) as u128;
_2 = _4;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463463374607430496858455 => bb7,
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
_11.0 = _1;
_11.2 = [15927_u16,54962_u16];
_11.1.0 = core::ptr::addr_of_mut!(_1);
_4 = 10825060699282265523_u64 as u128;
Goto(bb8)
}
bb8 = {
_6 = 61_u8 as u128;
_10 = [194_u8,168_u8,89_u8,160_u8];
_8 = _3 & _7;
_6 = !_8;
_1 = !_11.0;
_11.1.0 = core::ptr::addr_of_mut!(_1);
_1 = true as i32;
_3 = _2;
_6 = !_3;
_10 = [235_u8,102_u8,157_u8,84_u8];
_11.2 = [50480_u16,44110_u16];
_8 = _7 - _3;
_11.0 = _1 & _1;
_10 = [74_u8,171_u8,198_u8,182_u8];
_3 = !_2;
_1 = _11.0;
_8 = _3;
_4 = !_5;
_11.2 = [111_u16,40179_u16];
_2 = _7;
_4 = _8;
Goto(bb9)
}
bb9 = {
_15 = 42296572068341425705934431357904540049_i128 & 107934861915894986080571835849742533085_i128;
_15 = false as i128;
_13 = !1141721025_u32;
_12 = !true;
_3 = _7;
_3 = _7;
_10 = [48_u8,84_u8,161_u8,194_u8];
_12 = !false;
_10 = [202_u8,216_u8,61_u8,22_u8];
_5 = _4;
_6 = !_7;
_3 = !_6;
_3 = _4;
_5 = _8;
_1 = _11.0 + _11.0;
_8 = _3;
_9 = _3 | _3;
_15 = 3850507601488063343_usize as i128;
_11.0 = !_1;
_4 = _2;
_2 = !_7;
_9 = _2;
_9 = 12561_u16 as u128;
_8 = (-724055685878134052_i64) as u128;
_16 = 61017_u16 as i8;
Goto(bb10)
}
bb10 = {
_15 = (-9992499697962039713575694996810207247_i128);
_11.2 = [7407_u16,14129_u16];
Goto(bb11)
}
bb11 = {
_15 = (-58695347937426052349261657526719284112_i128);
Goto(bb12)
}
bb12 = {
_11.2 = [27220_u16,43230_u16];
_16 = 21_i8;
_2 = !_5;
_12 = false;
_15 = 45874140549292762795413912404473729643_i128;
_6 = _5 - _4;
_11.0 = _1 + _1;
_7 = _6;
_15 = (-71792696599528436376141849019798642321_i128) ^ (-46119171391900019378104382349757314841_i128);
_9 = !_4;
_11.1.0 = core::ptr::addr_of_mut!(_11.0);
_16 = !50_i8;
_3 = _15 as u128;
_1 = _15 as i32;
_5 = _6;
_10 = [106_u8,76_u8,74_u8,4_u8];
_16 = (-60_i8);
_6 = _5 << _2;
_5 = _6;
_13 = 55096_u16 as u32;
_2 = 26589_u16 as u128;
_4 = _7 + _6;
_13 = 3404396697_u32 + 3975498844_u32;
_7 = _4;
_1 = !_11.0;
Goto(bb13)
}
bb13 = {
_15 = 9223372036854775807_isize as i128;
_5 = 26728_u16 as u128;
_1 = _15 as i32;
_12 = true & false;
_11.2 = [27109_u16,9661_u16];
_13 = !4009336877_u32;
_10 = [250_u8,253_u8,40_u8,51_u8];
_6 = 207_u8 as u128;
_7 = (-615939164661048342_i64) as u128;
_12 = !true;
_3 = _4;
_11.1.0 = core::ptr::addr_of_mut!(_11.0);
_7 = _12 as u128;
match _16 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
340282366920938463463374607431768211396 => bb19,
_ => bb18
}
}
bb14 = {
Return()
}
bb15 = {
_15 = (-58695347937426052349261657526719284112_i128);
Goto(bb12)
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
_6 = 61_u8 as u128;
_10 = [194_u8,168_u8,89_u8,160_u8];
_8 = _3 & _7;
_6 = !_8;
_1 = !_11.0;
_11.1.0 = core::ptr::addr_of_mut!(_1);
_1 = true as i32;
_3 = _2;
_6 = !_3;
_10 = [235_u8,102_u8,157_u8,84_u8];
_11.2 = [50480_u16,44110_u16];
_8 = _7 - _3;
_11.0 = _1 & _1;
_10 = [74_u8,171_u8,198_u8,182_u8];
_3 = !_2;
_1 = _11.0;
_8 = _3;
_4 = !_5;
_11.2 = [111_u16,40179_u16];
_2 = _7;
_4 = _8;
Goto(bb9)
}
bb19 = {
_8 = 5005155557971476002_usize as u128;
_11.1.0 = core::ptr::addr_of_mut!(_11.0);
_3 = _9 << _9;
_10 = [218_u8,168_u8,249_u8,226_u8];
_5 = _11.0 as u128;
_3 = _9;
_8 = _9;
_11.1.0 = core::ptr::addr_of_mut!(_11.0);
_12 = _8 >= _3;
_1 = _11.0 >> _4;
_17 = [227_u8,132_u8,205_u8,213_u8];
Goto(bb20)
}
bb20 = {
_16 = 50_i8 * (-125_i8);
_5 = _3;
_2 = 239_u8 as u128;
_10 = [130_u8,230_u8,78_u8,5_u8];
Goto(bb21)
}
bb21 = {
_15 = _9 as i128;
_15 = 70299998454292297917269258367117770794_i128;
_5 = _8 >> _8;
_13 = 3203413458_u32 * 2453094109_u32;
_20 = ['\u{6d2da}','\u{32514}','\u{ef31b}','\u{22b17}'];
_3 = _5;
_15 = 133195904769394451007922119080391590950_i128 | 168181563408729400913243870646525913459_i128;
_6 = _4;
_16 = _15 as i8;
_11.2 = [38054_u16,30616_u16];
_9 = !_6;
_19 = _13 & _13;
_18 = _16 & _16;
_17 = [151_u8,81_u8,72_u8,217_u8];
Call(_15 = core::intrinsics::bswap(107236253490307718203223489773781786310_i128), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
_8 = _3 << _3;
_20 = ['\u{a4bbf}','\u{701c5}','\u{8b90c}','\u{f222c}'];
_21 = _12;
_8 = _5;
_6 = _4 - _3;
_20 = ['\u{25f89}','\u{1075a5}','\u{f874b}','\u{b93a5}'];
_19 = _13 >> _3;
_2 = _4;
_10 = [48_u8,203_u8,221_u8,196_u8];
_22 = [13245_i16,(-32742_i16),(-24377_i16)];
_10 = _17;
_11.1.0 = core::ptr::addr_of_mut!(_11.0);
_23.2 = _1 - _1;
_9 = _6;
_21 = _12;
_8 = _2 << _1;
_5 = _4 * _9;
_21 = _12;
_4 = _2;
Goto(bb23)
}
bb23 = {
_23.1 = 1500448370151100501_u64;
_27 = [(-17381_i16),(-17491_i16),17236_i16];
_7 = _2 | _8;
_9 = 175_u8 as u128;
_9 = _6 ^ _3;
_17 = [127_u8,106_u8,116_u8,103_u8];
_23.2 = -_1;
_11.1.0 = core::ptr::addr_of_mut!(_23.2);
_11.1.0 = core::ptr::addr_of_mut!(_23.2);
_26 = ['\u{2c77b}','\u{3300}','\u{ebfe4}','\u{f6a7}'];
_7 = _3;
_21 = _6 == _8;
_26 = _20;
_9 = !_8;
_12 = !_21;
_26 = ['\u{16cdd}','\u{f69a0}','\u{ddcce}','\u{4b60e}'];
_8 = _19 as u128;
_20 = ['\u{6fb2c}','\u{a61c4}','\u{4be3b}','\u{6b40c}'];
_23.1 = _5 as u64;
_11.0 = _1;
_14 = core::ptr::addr_of!(_29.fld2);
_17 = [167_u8,147_u8,199_u8,185_u8];
(*_14) = 11005228221439085768_usize as isize;
_29.fld3 = _18;
_23.2 = _12 as i32;
Goto(bb24)
}
bb24 = {
_5 = _8;
_22 = [19118_i16,15500_i16,14655_i16];
_13 = _19 >> _2;
_29.fld0 = [_18,_29.fld3,_16,_16,_16,_16,_29.fld3];
_4 = _5;
(*_14) = -(-9223372036854775808_isize);
_11.0 = 103_u8 as i32;
_14 = core::ptr::addr_of!(_29.fld2);
_15 = 19517078183307609912255842534460966854_i128 ^ (-108019894636250709923312699034689974553_i128);
_29.fld4 = _22;
_18 = _29.fld3 + _16;
_31 = (*_14);
_11.0 = _23.2 + _23.2;
_24 = _14;
_30 = _29.fld2;
_7 = !_9;
_29.fld0 = [_16,_18,_18,_18,_18,_29.fld3,_29.fld3];
_18 = _29.fld3 - _16;
Call(_32 = fn13(_5, _7, _13, _12, _6, _5, _13, _6, _19, _11.1.0, _3, _11.1, _3, _5, _11), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
SetDiscriminant(_32, 1);
_22 = [21386_i16,18643_i16,5901_i16];
_15 = 161733253088917262555685194323297340530_i128 * (-64738423897571052602856761006032512663_i128);
_31 = (*_24);
_8 = _9 + _2;
(*_24) = 1_usize as isize;
_4 = !_3;
Goto(bb26)
}
bb26 = {
_6 = _3 >> _13;
_3 = !_6;
_9 = _5;
_13 = (*_24) as u32;
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(_32, 1), 0)).3 = _29.fld3 as u128;
_11.2 = [12929_u16,40821_u16];
_27 = [5009_i16,8859_i16,816_i16];
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(_32, 1), 0)).0 = !_21;
_23.0 = core::ptr::addr_of!(place!(Field::<(bool, char, [i16; 3], u128)>(Variant(_32, 1), 0)).1);
_23.0 = core::ptr::addr_of!(place!(Field::<(bool, char, [i16; 3], u128)>(Variant(_32, 1), 0)).1);
_29.fld4 = _22;
_1 = _29.fld2 as i32;
_29.fld4 = [15112_i16,4472_i16,(-23414_i16)];
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(_32, 1), 0)).2 = _29.fld4;
_11.0 = _23.2;
_28 = (-858371686057753441_i64) as f32;
_19 = _13;
_4 = !_6;
Goto(bb27)
}
bb27 = {
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(_32, 1), 0)).3 = 52757_u16 as u128;
_11.2 = [43213_u16,14566_u16];
place!(Field::<(*mut i32,)>(Variant(_32, 1), 2)) = _11.1;
_13 = _19;
_45 = [_23.1,_23.1,_23.1];
place!(Field::<u32>(Variant(_32, 1), 1)) = _13;
_29.fld2 = _28 as isize;
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(_32, 1), 0)).0 = _4 >= _7;
_41 = _8 as usize;
(*_14) = -_31;
_44 = 144_u8;
(*_24) = _30;
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(_32, 1), 0)).2 = _29.fld4;
_23.1 = !18276624325529891385_u64;
_42 = core::ptr::addr_of_mut!(place!(Field::<(bool, char, [i16; 3], u128)>(Variant(_32, 1), 0)).2);
_47 = [_23.1,_23.1,_23.1];
_40 = _17;
_43 = _44 as isize;
_6 = _8;
_19 = _13 * _13;
_11.1 = (Field::<(*mut i32,)>(Variant(_32, 1), 2).0,);
_36 = [_18,_16,_18,_29.fld3,_29.fld3,_18,_29.fld3];
_30 = -(*_14);
place!(Field::<u32>(Variant(_32, 1), 1)) = !_13;
RET = Adt45::Variant0 { fld0: _29.fld0 };
_39 = _15 | _15;
_16 = -_29.fld3;
Goto(bb28)
}
bb28 = {
Call(_48 = dump_var(12_usize, 12_usize, Move(_12), 3_usize, Move(_3), 5_usize, Move(_5), 4_usize, Move(_4)), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Call(_48 = dump_var(12_usize, 2_usize, Move(_2), 19_usize, Move(_19), 16_usize, Move(_16), 18_usize, Move(_18)), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
Call(_48 = dump_var(12_usize, 6_usize, Move(_6), 44_usize, Move(_44), 20_usize, Move(_20), 17_usize, Move(_17)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Call(_48 = dump_var(12_usize, 13_usize, Move(_13), 21_usize, Move(_21), 31_usize, Move(_31), 7_usize, Move(_7)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: u128,mut _2: u128,mut _3: u32,mut _4: bool,mut _5: u128,mut _6: u128,mut _7: u32,mut _8: u128,mut _9: u32,mut _10: *mut i32,mut _11: u128,mut _12: (*mut i32,),mut _13: u128,mut _14: u128,mut _15: (i32, (*mut i32,), [u16; 2])) -> Adt51 {
mir! {
type RET = Adt51;
let _16: [i64; 2];
let _17: isize;
let _18: u64;
let _19: [u64; 3];
let _20: *mut isize;
let _21: char;
let _22: i128;
let _23: Adt43;
let _24: ();
let _25: ();
{
_4 = !false;
_1 = _8 >> _5;
_5 = !_11;
_11 = _9 as u128;
RET = Adt51::Variant0 { fld0: _4 };
_14 = 9223372036854775807_isize as u128;
_15.1 = (_12.0,);
_10 = _15.1.0;
SetDiscriminant(RET, 1);
place!(Field::<u32>(Variant(RET, 1), 1)) = !_9;
Goto(bb1)
}
bb1 = {
place!(Field::<(*mut i32,)>(Variant(RET, 1), 2)).0 = core::ptr::addr_of_mut!(_15.0);
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 0)).1 = '\u{7c2cf}';
_3 = Field::<u32>(Variant(RET, 1), 1) << _7;
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 0)).0 = _4;
Goto(bb2)
}
bb2 = {
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 0)).3 = !_5;
place!(Field::<(*mut i32,)>(Variant(RET, 1), 2)) = _15.1;
(*_10) = Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 0).1 as i32;
_19 = [3737441062795173540_u64,9383001611490774760_u64,18384026539858460866_u64];
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 0)).2 = [1315_i16,(-11444_i16),20809_i16];
_12 = (_15.1.0,);
_4 = _7 <= Field::<u32>(Variant(RET, 1), 1);
_19 = [5166714008509078887_u64,1000181034087223584_u64,5152711493954013074_u64];
(*_10) = _15.0 * _15.0;
Goto(bb3)
}
bb3 = {
_15.1.0 = _10;
_15.0 = 14929_i16 as i32;
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 0)).3 = _1;
_21 = Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 0).1;
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 0)).2 = [(-13567_i16),(-1654_i16),9803_i16];
Goto(bb4)
}
bb4 = {
_2 = (*_10) as u128;
place!(Field::<(bool, char, [i16; 3], u128)>(Variant(RET, 1), 0)).3 = 17413_i16 as u128;
RET = Adt51::Variant0 { fld0: _4 };
_8 = !_2;
_15.1.0 = _12.0;
_15.1 = _12;
_1 = !_11;
_10 = core::ptr::addr_of_mut!((*_10));
_9 = _7;
_15.1 = (_10,);
_16 = [(-896842632275802992_i64),8704111987308593109_i64];
_17 = (-9223372036854775808_isize);
Goto(bb5)
}
bb5 = {
Call(_24 = dump_var(13_usize, 21_usize, Move(_21), 7_usize, Move(_7), 16_usize, Move(_16), 4_usize, Move(_4)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_24 = dump_var(13_usize, 1_usize, Move(_1), 14_usize, Move(_14), 2_usize, Move(_2), 6_usize, Move(_6)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: u128,mut _2: u128,mut _3: u128,mut _4: u128,mut _5: u128,mut _6: bool,mut _7: u128,mut _8: u128,mut _9: bool,mut _10: u128,mut _11: bool,mut _12: u128) -> i64 {
mir! {
type RET = i64;
let _13: Adt44;
let _14: [i64; 2];
let _15: bool;
let _16: u64;
let _17: [i8; 7];
let _18: [i8; 7];
let _19: f64;
let _20: u16;
let _21: f64;
let _22: [u64; 3];
let _23: Adt50;
let _24: i8;
let _25: [i64; 2];
let _26: bool;
let _27: Adt48;
let _28: f64;
let _29: [u16; 2];
let _30: [u64; 3];
let _31: usize;
let _32: bool;
let _33: (bool, char, [i16; 3], u128);
let _34: char;
let _35: f32;
let _36: Adt54;
let _37: [u64; 3];
let _38: isize;
let _39: bool;
let _40: ();
let _41: ();
{
_12 = (-599462730_i32) as u128;
_7 = _4;
_10 = !_8;
RET = (-7109423633840174098_i64);
_12 = _2 << _5;
_9 = _11;
_12 = !_10;
RET = 5514712642803975606_i64 - 6216002771983985586_i64;
RET = 4653529317934743212_i64;
_4 = _11 as u128;
_7 = _1;
_6 = _11;
_9 = _6;
_3 = 102241006932698476226071241888469789835_i128 as u128;
_11 = _9;
_7 = !_10;
_3 = _8 ^ _7;
_2 = _5;
_10 = _5;
_11 = _6 & _9;
_2 = (-26819_i16) as u128;
_8 = 30_u8 as u128;
_8 = _12 + _5;
_4 = _12 | _3;
_5 = (-554213462_i32) as u128;
_7 = 117_i8 as u128;
RET = !(-5232699965584872379_i64);
Goto(bb1)
}
bb1 = {
_17 = [46_i8,(-68_i8),112_i8,(-53_i8),(-96_i8),(-98_i8),75_i8];
_16 = !18081538233378178310_u64;
_11 = _6;
_11 = !_6;
_6 = _9;
_8 = _16 as u128;
_15 = _12 >= _12;
_12 = 9149185369304846604_usize as u128;
_12 = _3 + _4;
_11 = !_9;
_8 = _3 ^ _12;
RET = (-4407388005276820122_i64);
_7 = _1 ^ _8;
_17 = [126_i8,65_i8,(-75_i8),96_i8,(-70_i8),67_i8,37_i8];
_14 = [RET,RET];
_3 = !_4;
_1 = !_12;
_16 = 173_u8 as u64;
RET = (-104832716252029347_i64) + 639267955846412302_i64;
_2 = _7;
Goto(bb2)
}
bb2 = {
_3 = _2 - _1;
_10 = _1 ^ _2;
_18 = [(-82_i8),(-88_i8),125_i8,(-118_i8),50_i8,(-32_i8),78_i8];
_20 = !30663_u16;
RET = 4200896784_u32 as i64;
_5 = 218_u8 as u128;
Goto(bb3)
}
bb3 = {
_10 = 16311408101483748273747494175521730621_i128 as u128;
_16 = 1734910210_u32 as u64;
_12 = _2;
_21 = _16 as f64;
_19 = _21 - _21;
_5 = _4;
_20 = 62766_u16;
_22 = [_16,_16,_16];
_6 = _11 & _9;
_23.fld0 = _6;
_12 = !_4;
_23.fld5.3 = 2805_i16 * 1013_i16;
_25 = [RET,RET];
_6 = _4 < _1;
_23.fld2 = !9223372036854775807_isize;
_14 = _25;
Goto(bb4)
}
bb4 = {
RET = _23.fld5.3 as i64;
_17 = [106_i8,(-124_i8),37_i8,(-110_i8),(-88_i8),34_i8,61_i8];
_23.fld2 = 34_isize << _2;
_23.fld4 = 153083988_u32;
_4 = _1 - _2;
_6 = !_11;
_14 = _25;
_1 = !_8;
_15 = !_23.fld0;
_23.fld1.0 = core::ptr::addr_of_mut!(_23.fld5.0.2);
_23.fld5.0.1 = _16 + _16;
_23.fld7 = core::ptr::addr_of!(_23.fld2);
_30 = [_23.fld5.0.1,_23.fld5.0.1,_16];
_28 = -_21;
_15 = !_9;
_23.fld4 = !1517097168_u32;
RET = (-5218869778907074761_i64);
_23.fld5.0.2 = 1632286455_i32 >> _12;
_29 = [_20,_20];
_23.fld3 = -9_i8;
_11 = !_15;
_23.fld5.0.1 = _23.fld5.0.2 as u64;
_31 = 77_u8 as usize;
_16 = _23.fld5.0.1 << _23.fld5.0.2;
_4 = _28 as u128;
_3 = _12 ^ _12;
_6 = _9 | _11;
_20 = 108022916288977546623895454108633178457_i128 as u16;
Goto(bb5)
}
bb5 = {
_19 = (-126972791422228832853446656228161002856_i128) as f64;
_26 = _11;
_24 = -_23.fld3;
_3 = _7;
_33.2 = [_23.fld5.3,_23.fld5.3,_23.fld5.3];
_18 = _17;
_6 = !_15;
_25 = _14;
_30 = [_23.fld5.0.1,_16,_23.fld5.0.1];
_23.fld5.1 = _23.fld5.3 as i128;
_32 = _8 != _12;
_33.0 = !_6;
_5 = !_12;
_3 = _8;
_23.fld2 = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
_23.fld7 = core::ptr::addr_of!(_23.fld2);
_23.fld3 = _24 + _24;
_33.3 = _12 ^ _8;
_18 = _17;
_5 = _8;
_24 = 145_u8 as i8;
_11 = !_15;
_1 = _8 * _7;
_17 = _18;
_34 = '\u{106134}';
_23.fld5.2 = [_16,_23.fld5.0.1,_16];
Goto(bb6)
}
bb6 = {
_14 = _25;
_23.fld5.0.2 = _23.fld3 as i32;
_20 = _23.fld5.1 as u16;
_4 = !_12;
_19 = _23.fld5.3 as f64;
_23.fld5.2 = [_16,_23.fld5.0.1,_23.fld5.0.1];
_23.fld5.1 = (-105435644989348612020849777902542745078_i128) << _7;
_23.fld5.2 = [_23.fld5.0.1,_23.fld5.0.1,_23.fld5.0.1];
_30 = _23.fld5.2;
_26 = _33.0;
_11 = _3 != _2;
Call(_24 = fn15(_23.fld5.1, _9, _23.fld0, _8), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_23.fld5.0.1 = _34 as u64;
_12 = 53_u8 as u128;
_23.fld0 = !_32;
_18 = [_24,_24,_24,_24,_24,_24,_24];
_38 = _23.fld2;
_35 = _16 as f32;
Goto(bb8)
}
bb8 = {
_15 = _6 == _32;
_23.fld1.0 = core::ptr::addr_of_mut!(_23.fld5.0.2);
match RET {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
340282366920938463458155737652861136695 => bb14,
_ => bb13
}
}
bb9 = {
_3 = _2 - _1;
_10 = _1 ^ _2;
_18 = [(-82_i8),(-88_i8),125_i8,(-118_i8),50_i8,(-32_i8),78_i8];
_20 = !30663_u16;
RET = 4200896784_u32 as i64;
_5 = 218_u8 as u128;
Goto(bb3)
}
bb10 = {
_14 = _25;
_23.fld5.0.2 = _23.fld3 as i32;
_20 = _23.fld5.1 as u16;
_4 = !_12;
_19 = _23.fld5.3 as f64;
_23.fld5.2 = [_16,_23.fld5.0.1,_23.fld5.0.1];
_23.fld5.1 = (-105435644989348612020849777902542745078_i128) << _7;
_23.fld5.2 = [_23.fld5.0.1,_23.fld5.0.1,_23.fld5.0.1];
_30 = _23.fld5.2;
_26 = _33.0;
_11 = _3 != _2;
Call(_24 = fn15(_23.fld5.1, _9, _23.fld0, _8), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
_19 = (-126972791422228832853446656228161002856_i128) as f64;
_26 = _11;
_24 = -_23.fld3;
_3 = _7;
_33.2 = [_23.fld5.3,_23.fld5.3,_23.fld5.3];
_18 = _17;
_6 = !_15;
_25 = _14;
_30 = [_23.fld5.0.1,_16,_23.fld5.0.1];
_23.fld5.1 = _23.fld5.3 as i128;
_32 = _8 != _12;
_33.0 = !_6;
_5 = !_12;
_3 = _8;
_23.fld2 = (-9223372036854775808_isize) ^ 9223372036854775807_isize;
_23.fld7 = core::ptr::addr_of!(_23.fld2);
_23.fld3 = _24 + _24;
_33.3 = _12 ^ _8;
_18 = _17;
_5 = _8;
_24 = 145_u8 as i8;
_11 = !_15;
_1 = _8 * _7;
_17 = _18;
_34 = '\u{106134}';
_23.fld5.2 = [_16,_23.fld5.0.1,_16];
Goto(bb6)
}
bb12 = {
RET = _23.fld5.3 as i64;
_17 = [106_i8,(-124_i8),37_i8,(-110_i8),(-88_i8),34_i8,61_i8];
_23.fld2 = 34_isize << _2;
_23.fld4 = 153083988_u32;
_4 = _1 - _2;
_6 = !_11;
_14 = _25;
_1 = !_8;
_15 = !_23.fld0;
_23.fld1.0 = core::ptr::addr_of_mut!(_23.fld5.0.2);
_23.fld5.0.1 = _16 + _16;
_23.fld7 = core::ptr::addr_of!(_23.fld2);
_30 = [_23.fld5.0.1,_23.fld5.0.1,_16];
_28 = -_21;
_15 = !_9;
_23.fld4 = !1517097168_u32;
RET = (-5218869778907074761_i64);
_23.fld5.0.2 = 1632286455_i32 >> _12;
_29 = [_20,_20];
_23.fld3 = -9_i8;
_11 = !_15;
_23.fld5.0.1 = _23.fld5.0.2 as u64;
_31 = 77_u8 as usize;
_16 = _23.fld5.0.1 << _23.fld5.0.2;
_4 = _28 as u128;
_3 = _12 ^ _12;
_6 = _9 | _11;
_20 = 108022916288977546623895454108633178457_i128 as u16;
Goto(bb5)
}
bb13 = {
_10 = 16311408101483748273747494175521730621_i128 as u128;
_16 = 1734910210_u32 as u64;
_12 = _2;
_21 = _16 as f64;
_19 = _21 - _21;
_5 = _4;
_20 = 62766_u16;
_22 = [_16,_16,_16];
_6 = _11 & _9;
_23.fld0 = _6;
_12 = !_4;
_23.fld5.3 = 2805_i16 * 1013_i16;
_25 = [RET,RET];
_6 = _4 < _1;
_23.fld2 = !9223372036854775807_isize;
_14 = _25;
Goto(bb4)
}
bb14 = {
_7 = !_4;
_33.3 = _4 & _3;
_23.fld3 = -_24;
_11 = !_6;
_22 = [_16,_16,_16];
_23.fld7 = core::ptr::addr_of!(_38);
_11 = _6;
_37 = [_16,_16,_16];
_19 = _28 * _28;
_31 = 7_usize;
_23.fld3 = _24;
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(14_usize, 16_usize, Move(_16), 3_usize, Move(_3), 24_usize, Move(_24), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(14_usize, 22_usize, Move(_22), 6_usize, Move(_6), 32_usize, Move(_32), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(14_usize, 8_usize, Move(_8), 29_usize, Move(_29), 10_usize, Move(_10), 30_usize, Move(_30)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(14_usize, 17_usize, Move(_17), 37_usize, Move(_37), 41_usize, _41, 41_usize, _41), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: i128,mut _2: bool,mut _3: bool,mut _4: u128) -> i8 {
mir! {
type RET = i8;
let _5: [i64; 2];
let _6: bool;
let _7: isize;
let _8: [i8; 7];
let _9: Adt57;
let _10: isize;
let _11: [u16; 2];
let _12: [i8; 7];
let _13: ();
let _14: ();
{
_3 = _2 ^ _2;
_1 = (-108194158470021673559129607978497338822_i128);
_4 = 63929335012701485533771156459386888063_u128 + 3818979178995134501396843227091257156_u128;
RET = 7851109890162316733_i64 as i8;
_5 = [7553321813571416638_i64,4496018243020170432_i64];
RET = !(-99_i8);
_1 = 10432094_i32 as i128;
RET = (-117_i8) ^ (-18_i8);
_3 = _2 == _2;
RET = (-52_i8);
_4 = 2548133463_u32 as u128;
_5 = [2710020292536064059_i64,6410577852695508893_i64];
_3 = !_2;
RET = 62_i8 + 25_i8;
Goto(bb1)
}
bb1 = {
RET = !47_i8;
RET = 12_i8;
_5 = [915892194492346675_i64,(-2833812988002051391_i64)];
_4 = !236265713450490467607642854097829974299_u128;
_2 = !_3;
_5 = [(-8978419458202933342_i64),2655975591084394524_i64];
_7 = (-93_isize) << _1;
_5 = [4932313940306962214_i64,(-5931734873956651700_i64)];
_6 = _2 & _3;
_8 = [RET,RET,RET,RET,RET,RET,RET];
_6 = _3 ^ _2;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
12 => bb9,
_ => bb8
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
_4 = !278564449084982858396609765312354921462_u128;
_7 = 9223372036854775807_isize;
_6 = _3;
_3 = _2;
_1 = _6 as i128;
_2 = !_3;
RET = (-87_i8) >> _1;
_6 = _3 & _2;
_4 = 170383671247443710259704896663619615534_u128 | 183264407289189147162782160669013986781_u128;
_2 = _6 | _6;
_10 = _7;
_10 = -_7;
_7 = 2773870193_u32 as isize;
_12 = _8;
Goto(bb10)
}
bb10 = {
Call(_13 = dump_var(15_usize, 10_usize, Move(_10), 6_usize, Move(_6), 12_usize, Move(_12), 4_usize, Move(_4)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_13 = dump_var(15_usize, 8_usize, Move(_8), 14_usize, _14, 14_usize, _14, 14_usize, _14), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: u128,mut _2: u128,mut _3: u128,mut _4: bool,mut _5: u128) -> (bool, char, [i16; 3], u128) {
mir! {
type RET = (bool, char, [i16; 3], u128);
let _6: [u64; 3];
let _7: *const char;
let _8: ((*const char, u64, i32), i128, [u64; 3], i16);
let _9: ();
let _10: ();
{
RET.2 = [(-13420_i16),25444_i16,11358_i16];
RET.1 = '\u{f0bb2}';
RET.2 = [21610_i16,(-17714_i16),(-11893_i16)];
RET.0 = _4 == _4;
RET.3 = !_5;
RET.2 = [3030_i16,(-17807_i16),(-19109_i16)];
RET.0 = _4;
_6 = [8032663435488964104_u64,10387950811596027365_u64,9735878068526742444_u64];
_2 = _1 & _3;
RET.1 = '\u{dc7f4}';
_7 = core::ptr::addr_of!(RET.1);
_1 = _2;
_2 = !_3;
Goto(bb1)
}
bb1 = {
Call(_9 = dump_var(16_usize, 3_usize, Move(_3), 6_usize, Move(_6), 4_usize, Move(_4), 10_usize, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: f32,mut _2: f32,mut _3: f32,mut _4: f32,mut _5: f32,mut _6: f32) -> [char; 4] {
mir! {
type RET = [char; 4];
let _7: [u8; 6];
let _8: isize;
let _9: i64;
let _10: f32;
let _11: *mut (*const isize, (i32, u8, (*mut i32,), (*mut i32,), i16, bool, (bool, char, [i16; 3], u128)));
let _12: Adt57;
let _13: isize;
let _14: [u8; 6];
let _15: char;
let _16: [u8; 4];
let _17: Adt45;
let _18: [u8; 6];
let _19: isize;
let _20: u64;
let _21: (bool, char, [i16; 3], u128);
let _22: i128;
let _23: (u16,);
let _24: [i8; 7];
let _25: Adt45;
let _26: f64;
let _27: Adt56;
let _28: ();
let _29: ();
{
_5 = _3;
RET = ['\u{3dc84}','\u{7cc8e}','\u{7f248}','\u{b5810}'];
_2 = _5;
_4 = (-142045861323474027975499269101693013524_i128) as f32;
_5 = 7111785471688039531_i64 as f32;
_2 = _3 + _3;
_5 = _3 - _1;
_4 = _2 * _5;
_7 = [123_u8,208_u8,65_u8,150_u8,37_u8,149_u8];
_4 = -_3;
_6 = (-23495_i16) as f32;
_1 = (-96_i8) as f32;
_6 = 3149039679_u32 as f32;
_7 = [183_u8,56_u8,116_u8,101_u8,17_u8,152_u8];
_7 = [244_u8,110_u8,9_u8,121_u8,213_u8,218_u8];
_7 = [130_u8,54_u8,231_u8,246_u8,180_u8,254_u8];
_4 = _5;
_5 = _2;
RET = ['\u{8c1fe}','\u{e9697}','\u{7d95d}','\u{ae912}'];
_1 = (-63_i8) as f32;
_3 = -_5;
_6 = -_2;
_9 = (-4485225709213057129_i64) >> 96_i8;
_8 = 9223372036854775807_isize | (-84_isize);
_3 = -_6;
_2 = _5;
_8 = 52473694714306509021804868343954997940_i128 as isize;
Goto(bb1)
}
bb1 = {
_9 = !(-4415095911521357735_i64);
Goto(bb2)
}
bb2 = {
_1 = _4 - _4;
_1 = _4 * _4;
_6 = (-119320486225972892071940062657517194258_i128) as f32;
RET = ['\u{a9ccb}','\u{1403d}','\u{bbc9c}','\u{43808}'];
_8 = 52531_u16 as isize;
_10 = _3 + _1;
_5 = -_10;
_4 = -_10;
_7 = [252_u8,67_u8,218_u8,231_u8,130_u8,244_u8];
RET = ['\u{642e}','\u{d1358}','\u{a2986}','\u{c5c97}'];
_10 = _2;
_2 = 5_usize as f32;
_8 = (-109_isize);
_3 = _8 as f32;
Goto(bb3)
}
bb3 = {
_3 = _8 as f32;
_7 = [31_u8,210_u8,35_u8,215_u8,97_u8,53_u8];
_1 = -_4;
_2 = _10;
_10 = (-94817504_i32) as f32;
_6 = -_5;
_5 = _1 * _4;
RET = ['\u{10dc22}','\u{29204}','\u{26c9}','\u{f4157}'];
_6 = _8 as f32;
_13 = 591544080_i32 as isize;
_5 = 3877912468_u32 as f32;
match _8 {
0 => bb1,
1 => bb4,
340282366920938463463374607431768211347 => bb6,
_ => bb5
}
}
bb4 = {
_1 = _4 - _4;
_1 = _4 * _4;
_6 = (-119320486225972892071940062657517194258_i128) as f32;
RET = ['\u{a9ccb}','\u{1403d}','\u{bbc9c}','\u{43808}'];
_8 = 52531_u16 as isize;
_10 = _3 + _1;
_5 = -_10;
_4 = -_10;
_7 = [252_u8,67_u8,218_u8,231_u8,130_u8,244_u8];
RET = ['\u{642e}','\u{d1358}','\u{a2986}','\u{c5c97}'];
_10 = _2;
_2 = 5_usize as f32;
_8 = (-109_isize);
_3 = _8 as f32;
Goto(bb3)
}
bb5 = {
_9 = !(-4415095911521357735_i64);
Goto(bb2)
}
bb6 = {
_15 = '\u{5bb3d}';
_2 = -_4;
_14 = [10_u8,64_u8,23_u8,47_u8,74_u8,136_u8];
_7 = _14;
_8 = _13 | _13;
_9 = (-6494756888307143733_i64);
_10 = _4;
_13 = !_8;
_9 = (-6048229818484070576_i64);
_3 = _10;
_15 = '\u{53631}';
_1 = _3 * _3;
_1 = _10 + _3;
_15 = '\u{12bbd}';
_8 = -_13;
_16 = [182_u8,148_u8,208_u8,142_u8];
Call(_4 = core::intrinsics::transmute(_15), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_5 = _1 * _3;
_16 = [111_u8,222_u8,87_u8,58_u8];
_20 = _9 as u64;
_2 = _3 * _1;
_2 = -_10;
match _9 {
0 => bb6,
1 => bb4,
2 => bb3,
340282366920938463457326377613284140880 => bb9,
_ => bb8
}
}
bb8 = {
_9 = !(-4415095911521357735_i64);
Goto(bb2)
}
bb9 = {
_18 = [203_u8,146_u8,61_u8,229_u8,93_u8,152_u8];
_6 = -_2;
_1 = _5;
_1 = -_6;
_7 = _14;
_4 = _1;
_20 = 2854249587_u32 as u64;
_19 = _20 as isize;
_1 = _10;
_13 = !_8;
_3 = 62638_u16 as f32;
_6 = -_5;
_4 = _2;
_22 = (-20405091606359156791691634521494135541_i128) | 5560921729234798991917673129164250310_i128;
_21.1 = _15;
_21.1 = _15;
match _9 {
0 => bb5,
1 => bb2,
2 => bb10,
3 => bb11,
340282366920938463457326377613284140880 => bb13,
_ => bb12
}
}
bb10 = {
_3 = _8 as f32;
_7 = [31_u8,210_u8,35_u8,215_u8,97_u8,53_u8];
_1 = -_4;
_2 = _10;
_10 = (-94817504_i32) as f32;
_6 = -_5;
_5 = _1 * _4;
RET = ['\u{10dc22}','\u{29204}','\u{26c9}','\u{f4157}'];
_6 = _8 as f32;
_13 = 591544080_i32 as isize;
_5 = 3877912468_u32 as f32;
match _8 {
0 => bb1,
1 => bb4,
340282366920938463463374607431768211347 => bb6,
_ => bb5
}
}
bb11 = {
_1 = _4 - _4;
_1 = _4 * _4;
_6 = (-119320486225972892071940062657517194258_i128) as f32;
RET = ['\u{a9ccb}','\u{1403d}','\u{bbc9c}','\u{43808}'];
_8 = 52531_u16 as isize;
_10 = _3 + _1;
_5 = -_10;
_4 = -_10;
_7 = [252_u8,67_u8,218_u8,231_u8,130_u8,244_u8];
RET = ['\u{642e}','\u{d1358}','\u{a2986}','\u{c5c97}'];
_10 = _2;
_2 = 5_usize as f32;
_8 = (-109_isize);
_3 = _8 as f32;
Goto(bb3)
}
bb12 = {
_1 = _4 - _4;
_1 = _4 * _4;
_6 = (-119320486225972892071940062657517194258_i128) as f32;
RET = ['\u{a9ccb}','\u{1403d}','\u{bbc9c}','\u{43808}'];
_8 = 52531_u16 as isize;
_10 = _3 + _1;
_5 = -_10;
_4 = -_10;
_7 = [252_u8,67_u8,218_u8,231_u8,130_u8,244_u8];
RET = ['\u{642e}','\u{d1358}','\u{a2986}','\u{c5c97}'];
_10 = _2;
_2 = 5_usize as f32;
_8 = (-109_isize);
_3 = _8 as f32;
Goto(bb3)
}
bb13 = {
_8 = _13 ^ _13;
_21.2 = [13617_i16,(-7008_i16),21308_i16];
_13 = 8568_u16 as isize;
_21.0 = false;
_23.0 = 43719_u16 | 58652_u16;
RET = [_15,_21.1,_15,_21.1];
_20 = 3832674373550265538_u64 << _13;
_23 = (58716_u16,);
_19 = _8 | _8;
RET = [_15,_21.1,_21.1,_21.1];
_13 = !_19;
_1 = _5;
_14 = [142_u8,92_u8,59_u8,181_u8,59_u8,181_u8];
_6 = _2;
_21.1 = _15;
_6 = _4;
_9 = (-28893_i16) as i64;
_18 = [221_u8,81_u8,171_u8,2_u8,241_u8,109_u8];
_3 = _5 - _10;
_16 = [19_u8,153_u8,102_u8,32_u8];
_10 = _4;
_14 = _18;
_18 = _7;
_5 = -_1;
RET = [_15,_15,_15,_15];
_23.0 = _21.0 as u16;
RET = [_15,_15,_15,_21.1];
_18 = _14;
Call(_5 = fn18(_6, _18), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_20 = 11514281206416286896_u64;
_9 = (-2470843744109269457_i64) ^ (-4357190527922413125_i64);
_24 = [(-85_i8),124_i8,67_i8,(-96_i8),83_i8,(-44_i8),(-107_i8)];
_21.2 = [18012_i16,19700_i16,9208_i16];
_3 = 11230_i16 as f32;
_21.2 = [18460_i16,(-24296_i16),(-21093_i16)];
_27.fld0.fld3 = (-70_i8) & 19_i8;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(17_usize, 7_usize, Move(_7), 14_usize, Move(_14), 22_usize, Move(_22), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(17_usize, 19_usize, Move(_19), 13_usize, Move(_13), 29_usize, _29, 29_usize, _29), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: f32,mut _2: [u8; 6]) -> f32 {
mir! {
type RET = f32;
let _3: bool;
let _4: isize;
let _5: [u16; 2];
let _6: bool;
let _7: u8;
let _8: isize;
let _9: [i16; 3];
let _10: f64;
let _11: i64;
let _12: [u8; 6];
let _13: [u8; 4];
let _14: [i64; 2];
let _15: (bool, char, [i16; 3], u128);
let _16: Adt53;
let _17: [i16; 3];
let _18: u16;
let _19: [i16; 3];
let _20: Adt46;
let _21: [u8; 6];
let _22: ((*const char, u64, i32), i128, [u64; 3], i16);
let _23: [i16; 3];
let _24: (*mut i32,);
let _25: [u64; 3];
let _26: (*const isize, (i32, u8, (*mut i32,), (*mut i32,), i16, bool, (bool, char, [i16; 3], u128)));
let _27: (*const char, u64, i32);
let _28: isize;
let _29: ();
let _30: ();
{
_1 = (-5061997863227752399_i64) as f32;
_3 = true;
_3 = _1 > _1;
RET = _1 - _1;
RET = 5155984480618240389_i64 as f32;
RET = (-3335638621375342295_i64) as f32;
RET = 57688_u16 as f32;
_4 = (-9223372036854775808_isize);
RET = _1 * _1;
_1 = (-3694481627612119516_i64) as f32;
_1 = -RET;
_1 = RET;
_2 = [71_u8,141_u8,248_u8,244_u8,94_u8,64_u8];
_1 = RET + RET;
RET = -_1;
RET = _1 * _1;
match _4 {
0 => bb1,
340282366920938463454151235394913435648 => bb3,
_ => bb2
}
}
bb1 = {
Return()
}
bb2 = {
Return()
}
bb3 = {
_5 = [65356_u16,34747_u16];
_5 = [6254_u16,17946_u16];
_3 = !true;
_2 = [28_u8,49_u8,110_u8,124_u8,198_u8,38_u8];
RET = (-5724_i16) as f32;
_3 = false ^ true;
_4 = 9223372036854775807_isize;
_2 = [100_u8,121_u8,155_u8,68_u8,9_u8,202_u8];
_6 = RET < _1;
RET = _1 * _1;
_4 = 66_isize;
RET = (-20_i8) as f32;
_2 = [46_u8,220_u8,183_u8,148_u8,62_u8,95_u8];
RET = -_1;
_1 = -RET;
_7 = '\u{1b52e}' as u8;
_3 = _6;
match _4 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
66 => bb8,
_ => bb7
}
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
_7 = !147_u8;
_4 = 9223372036854775807_isize;
_6 = _3;
RET = -_1;
_9 = [(-24030_i16),1802_i16,3463_i16];
_8 = -_4;
_8 = -_4;
_9 = [(-12428_i16),(-348_i16),10534_i16];
RET = _7 as f32;
_10 = 2789319370340179392_u64 as f64;
RET = 413240734_u32 as f32;
_8 = _4 * _4;
_1 = RET * RET;
_5 = [42887_u16,4072_u16];
_7 = 22_u8 + 158_u8;
_2 = [_7,_7,_7,_7,_7,_7];
_5 = [5399_u16,19143_u16];
_1 = RET;
_7 = !179_u8;
Call(_11 = core::intrinsics::bswap((-7421316754302731452_i64)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_9 = [5942_i16,(-26366_i16),(-27432_i16)];
_8 = 37147_u16 as isize;
_6 = _3 != _3;
_3 = _6 == _6;
_13 = [_7,_7,_7,_7];
_11 = -(-1499630074652413141_i64);
_4 = -_8;
_4 = _8;
_9 = [(-20175_i16),(-28584_i16),(-3382_i16)];
_1 = RET;
_1 = RET;
Goto(bb10)
}
bb10 = {
_9 = [(-13583_i16),(-7488_i16),(-4291_i16)];
_12 = [_7,_7,_7,_7,_7,_7];
RET = _1 + _1;
_12 = [_7,_7,_7,_7,_7,_7];
_4 = 415602072692283278_u64 as isize;
_15.2 = [(-17681_i16),8328_i16,(-27061_i16)];
_15.0 = _3;
_7 = 233_u8 ^ 119_u8;
_7 = 86_u8 + 102_u8;
_6 = !_3;
_10 = 4231277384_u32 as f64;
_15 = (_6, '\u{37286}', _9, 286922286345425722847806468095593988691_u128);
_15.1 = '\u{108373}';
_12 = [_7,_7,_7,_7,_7,_7];
_17 = [12711_i16,24461_i16,(-13613_i16)];
_1 = RET;
_9 = _15.2;
_12 = [_7,_7,_7,_7,_7,_7];
Call(RET = core::intrinsics::transmute(_15.1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_14 = [_11,_11];
_15 = (_6, '\u{c5eea}', _9, 215357362506053814820179014099235777964_u128);
_3 = _15.0;
_9 = _15.2;
Goto(bb12)
}
bb12 = {
_10 = 14243136158585094779_u64 as f64;
RET = _1;
_3 = _15.0;
_15.3 = !837439625383728957667088907232439636_u128;
_6 = _15.0;
_15 = (_6, '\u{8a2f8}', _9, 213776354013524577305317554009918015006_u128);
match _15.3 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb10,
4 => bb5,
5 => bb11,
6 => bb7,
213776354013524577305317554009918015006 => bb13,
_ => bb8
}
}
bb13 = {
_8 = !_4;
_15.3 = 82590589559014017067100390279793444443_u128;
_9 = [(-4067_i16),29941_i16,31066_i16];
_18 = 20568_u16;
_8 = (-57_i8) as isize;
_19 = _9;
_15 = (_3, '\u{ac1f6}', _9, 208187585464867050515690894396298913663_u128);
_5 = [_18,_18];
RET = _1 - _1;
_11 = 8258507435117554723_i64;
_20.fld4.fld1 = core::ptr::addr_of!(_8);
_18 = !47816_u16;
_18 = _1 as u16;
_3 = !_6;
_20.fld3 = [_15.1,_15.1,_15.1,_15.1];
_20.fld0 = !614646356_u32;
_22.0.1 = !748208822187659919_u64;
match _15.3 {
0 => bb10,
208187585464867050515690894396298913663 => bb14,
_ => bb6
}
}
bb14 = {
_15 = (_3, '\u{d6bf4}', _9, 119644988957628538363096906555769978933_u128);
_20.fld1 = !6441874793515776243_usize;
_18 = 49529_u16;
_21 = [_7,_7,_7,_7,_7,_7];
_22.0.0 = core::ptr::addr_of!(_15.1);
_20.fld4.fld3 = -(-74_i8);
_22.3 = 20937_i16 >> _8;
_25 = [_22.0.1,_22.0.1,_22.0.1];
_7 = !43_u8;
_20.fld3 = [_15.1,_15.1,_15.1,_15.1];
_8 = _20.fld1 as isize;
_24.0 = core::ptr::addr_of_mut!(_22.0.2);
_26.1.6 = (_3, _15.1, _17, _15.3);
_15 = _26.1.6;
_11 = 3453207428550023265_i64;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(18_usize, 17_usize, Move(_17), 13_usize, Move(_13), 15_usize, Move(_15), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(18_usize, 5_usize, Move(_5), 19_usize, Move(_19), 7_usize, Move(_7), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(18_usize, 12_usize, Move(_12), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: i8,mut _2: f32,mut _3: [char; 4]) -> f32 {
mir! {
type RET = f32;
let _4: [u8; 4];
let _5: [i16; 3];
let _6: Adt53;
let _7: Adt48;
let _8: u128;
let _9: (bool, char, [i16; 3], u128);
let _10: i64;
let _11: [u8; 6];
let _12: [u8; 6];
let _13: isize;
let _14: bool;
let _15: (i32, (u8, (i32, (*mut i32,), [u16; 2])), *mut [i16; 3], &'static isize, u128, char, (*const isize, (i32, u8, (*mut i32,), (*mut i32,), i16, bool, (bool, char, [i16; 3], u128))));
let _16: [i64; 2];
let _17: (u16,);
let _18: char;
let _19: [i64; 2];
let _20: [u16; 2];
let _21: [i16; 3];
let _22: isize;
let _23: u128;
let _24: ();
let _25: ();
{
RET = _2 + _2;
_1 = !81_i8;
RET = _2;
RET = -_2;
_1 = 80_i8 ^ 90_i8;
_4 = [185_u8,107_u8,234_u8,76_u8];
_2 = RET;
_4 = [133_u8,20_u8,40_u8,10_u8];
_2 = RET - RET;
_2 = (-8443371538303719451_i64) as f32;
_1 = 59_i8;
_2 = -RET;
_2 = RET + RET;
_4 = [112_u8,22_u8,202_u8,211_u8];
_2 = RET;
_3 = ['\u{79488}','\u{f95a5}','\u{28596}','\u{ef39f}'];
_3 = ['\u{74efe}','\u{10f188}','\u{a022f}','\u{518cb}'];
_1 = false as i8;
RET = 24555_u16 as f32;
_2 = RET - RET;
RET = _2;
_5 = [(-23335_i16),(-4526_i16),(-25643_i16)];
_3 = ['\u{100552}','\u{f6277}','\u{27dd5}','\u{70bce}'];
_2 = -RET;
Goto(bb1)
}
bb1 = {
_8 = 330731698324059295577006841998900514377_u128 - 125589896218835576029097363821251950091_u128;
_1 = !106_i8;
_9.2 = [4356_i16,20970_i16,(-3267_i16)];
_9.0 = !true;
_2 = RET;
_8 = !52446647396161274215538173079092362760_u128;
_9.0 = false;
_9.3 = _8 & _8;
_10 = 4717761792167760870_i64;
_9.1 = '\u{ac1c5}';
_9 = (false, '\u{2b190}', _5, _8);
_9.3 = _8;
_9.1 = '\u{c760d}';
_9.0 = true;
_9.0 = true & true;
_11 = [81_u8,84_u8,63_u8,60_u8,155_u8,35_u8];
_1 = 101_i8 - (-34_i8);
_11 = [105_u8,66_u8,97_u8,59_u8,144_u8,222_u8];
Goto(bb2)
}
bb2 = {
RET = _2 - _2;
_4 = [80_u8,22_u8,254_u8,39_u8];
_2 = 7_usize as f32;
_12 = [19_u8,130_u8,51_u8,88_u8,42_u8,148_u8];
_5 = [15435_i16,(-27454_i16),(-25740_i16)];
_9.3 = _8 & _8;
_2 = 13977059485198418032_u64 as f32;
_9.1 = '\u{982cd}';
RET = -_2;
Goto(bb3)
}
bb3 = {
_4 = [208_u8,5_u8,104_u8,169_u8];
_3 = [_9.1,_9.1,_9.1,_9.1];
_2 = RET;
match _10 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
4717761792167760870 => bb10,
_ => bb9
}
}
bb4 = {
RET = _2 - _2;
_4 = [80_u8,22_u8,254_u8,39_u8];
_2 = 7_usize as f32;
_12 = [19_u8,130_u8,51_u8,88_u8,42_u8,148_u8];
_5 = [15435_i16,(-27454_i16),(-25740_i16)];
_9.3 = _8 & _8;
_2 = 13977059485198418032_u64 as f32;
_9.1 = '\u{982cd}';
RET = -_2;
Goto(bb3)
}
bb5 = {
_8 = 330731698324059295577006841998900514377_u128 - 125589896218835576029097363821251950091_u128;
_1 = !106_i8;
_9.2 = [4356_i16,20970_i16,(-3267_i16)];
_9.0 = !true;
_2 = RET;
_8 = !52446647396161274215538173079092362760_u128;
_9.0 = false;
_9.3 = _8 & _8;
_10 = 4717761792167760870_i64;
_9.1 = '\u{ac1c5}';
_9 = (false, '\u{2b190}', _5, _8);
_9.3 = _8;
_9.1 = '\u{c760d}';
_9.0 = true;
_9.0 = true & true;
_11 = [81_u8,84_u8,63_u8,60_u8,155_u8,35_u8];
_1 = 101_i8 - (-34_i8);
_11 = [105_u8,66_u8,97_u8,59_u8,144_u8,222_u8];
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
_9.1 = '\u{817c2}';
_14 = !_9.0;
_12 = _11;
RET = _2 + _2;
RET = -_2;
_15.6.1.6.0 = _14 & _14;
_15.3 = &_13;
_15.6.1.6.2 = [4138_i16,8757_i16,25001_i16];
_9.1 = '\u{109fd2}';
_15.6.1.3.0 = core::ptr::addr_of_mut!(_15.0);
_15.1.1.2 = [57595_u16,50175_u16];
_15.6.1.4 = (-1159_i16) - (-15839_i16);
Goto(bb11)
}
bb11 = {
_15.6.1.5 = !_9.0;
_15.6.1.6 = (_14, _9.1, _9.2, _8);
Goto(bb12)
}
bb12 = {
_16 = [_10,_10];
_15.5 = _15.6.1.6.1;
_15.6.1.6.1 = _9.1;
_1 = !(-56_i8);
_15.6.0 = core::ptr::addr_of!(_13);
_8 = !_9.3;
RET = _2;
_4 = [205_u8,231_u8,183_u8,172_u8];
_11 = [38_u8,179_u8,168_u8,18_u8,202_u8,10_u8];
_15.0 = (-1097800312_i32) >> _9.3;
_15.6.1.2 = (_15.6.1.3.0,);
_15.6.1.3 = _15.6.1.2;
_15.6.1.0 = _15.0 + _15.0;
_14 = !_15.6.1.6.0;
_15.1.1.2 = [25893_u16,11138_u16];
Goto(bb13)
}
bb13 = {
_9.3 = !_8;
_15.6.1.3 = (_15.6.1.2.0,);
_9.3 = _8 | _8;
Goto(bb14)
}
bb14 = {
_21 = [_15.6.1.4,_15.6.1.4,_15.6.1.4];
RET = _2 * _2;
_17 = (46926_u16,);
_20 = _15.1.1.2;
_5 = _9.2;
_20 = [_17.0,_17.0];
_15.1.1.0 = _15.6.1.0 + _15.6.1.0;
_15.1.0 = 93_u8 - 182_u8;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(19_usize, 5_usize, Move(_5), 10_usize, Move(_10), 11_usize, Move(_11), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(19_usize, 14_usize, Move(_14), 12_usize, Move(_12), 3_usize, Move(_3), 25_usize, _25), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box(173954558853383529869442658643937711436_u128), std::hint::black_box(9223372036854775807_isize), std::hint::black_box(14_i8), std::hint::black_box(9550210100669750266_u64), std::hint::black_box((-1442742976_i32)), std::hint::black_box(6930410245149871793_i64), std::hint::black_box((-70450144663942651739284732157562964689_i128)), std::hint::black_box(6638340546129590973_usize), std::hint::black_box(118_u8), std::hint::black_box(54873_u16), std::hint::black_box(188861451_u32));
                
            }
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt43{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt43 {
fld0: [i8; 7],
fld1: *const isize,
fld2: isize,
fld3: i8,
fld4: [i16; 3],
}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: u8,
fld1: Adt43,
fld2: *const i128,
fld3: i8,
fld4: (u16,),

},
Variant1{
fld0: u64,
fld1: [u16; 2],
fld2: *mut [i16; 3],
fld3: [u8; 6],
fld4: u8,
fld5: f64,

},
Variant2{
fld0: *mut isize,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
fld0: [i8; 7],

},
Variant1{
fld0: *const isize,
fld1: (*mut i32,),
fld2: isize,
fld3: [i16; 3],
fld4: [u64; 3],
fld5: *mut isize,
fld6: (bool, char, [i16; 3], u128),

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: u32,
fld1: usize,
fld2: [u8; 6],
fld3: [char; 4],
fld4: Adt43,
}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt47{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt47 {
fld0: f32,
fld1: [char; 4],
fld2: *const i128,
fld3: Adt45,
fld4: [u8; 4],
fld5: (*mut i32,),
}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: *mut i32,
fld1: Adt44,
fld2: isize,
fld3: [i64; 2],
fld4: u8,
fld5: u128,
fld6: (*const isize, (i32, u8, (*mut i32,), (*mut i32,), i16, bool, (bool, char, [i16; 3], u128))),

},
Variant1{
fld0: [i8; 7],
fld1: *mut i32,
fld2: Adt44,
fld3: (i32, (*mut i32,), [u16; 2]),
fld4: (i32, u8, (*mut i32,), (*mut i32,), i16, bool, (bool, char, [i16; 3], u128)),
fld5: [i16; 3],

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: Adt43,
fld1: (i32, u8, (*mut i32,), (*mut i32,), i16, bool, (bool, char, [i16; 3], u128)),
fld2: *const i128,
fld3: i128,
fld4: i16,
fld5: (*const char, u64, i32),
fld6: Adt46,

},
Variant1{
fld0: [i64; 2],
fld1: (bool, char, [i16; 3], u128),
fld2: [char; 4],
fld3: Adt46,
fld4: [u64; 3],

},
Variant2{
fld0: [i8; 7],
fld1: (u16,),
fld2: Adt44,
fld3: [i16; 3],

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: bool,
fld1: (*mut i32,),
fld2: isize,
fld3: i8,
fld4: u32,
fld5: ((*const char, u64, i32), i128, [u64; 3], i16),
fld6: Adt44,
fld7: *const isize,
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: bool,

},
Variant1{
fld0: (bool, char, [i16; 3], u128),
fld1: u32,
fld2: (*mut i32,),
fld3: Adt48,

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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: (u16,),
fld1: *const char,
fld2: f32,
fld3: i8,
fld4: [u16; 2],
fld5: *mut [i16; 3],
fld6: [u64; 3],

},
Variant1{
fld0: (*mut i32,),
fld1: (bool, char, [i16; 3], u128),
fld2: Adt43,
fld3: f64,
fld4: Adt45,
fld5: Adt49,
fld6: *mut isize,
fld7: [u16; 2],

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: *mut (*const isize, (i32, u8, (*mut i32,), (*mut i32,), i16, bool, (bool, char, [i16; 3], u128))),
fld1: *mut isize,
fld2: [u8; 4],
fld3: Adt47,
fld4: (bool, char, [i16; 3], u128),
fld5: Adt49,
fld6: f64,
fld7: u128,

},
Variant1{
fld0: [u16; 2],
fld1: char,
fld2: isize,
fld3: (*mut i32,),
fld4: Adt50,
fld5: *const char,
fld6: *const i128,
fld7: [u8; 4],

},
Variant2{
fld0: bool,
fld1: [u16; 2],
fld2: (*mut i32,),
fld3: i8,
fld4: (i32, (*mut i32,), [u16; 2]),
fld5: i32,
fld6: ((*const char, u64, i32), i128, [u64; 3], i16),

},
Variant3{
fld0: u128,
fld1: Adt48,
fld2: i8,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: bool,
fld1: *mut (*const isize, (i32, u8, (*mut i32,), (*mut i32,), i16, bool, (bool, char, [i16; 3], u128))),
fld2: [u16; 2],
fld3: ((*const char, u64, i32), i128, [u64; 3], i16),
fld4: (u16,),
fld5: (bool, char, [i16; 3], u128),
fld6: Adt43,
fld7: *mut [i16; 3],

},
Variant1{
fld0: *mut i32,
fld1: [i16; 3],
fld2: Adt53,
fld3: Adt47,
fld4: i64,

},
Variant2{
fld0: bool,
fld1: (i32, (*mut i32,), [u16; 2]),
fld2: [u8; 4],
fld3: Adt45,
fld4: [char; 4],
fld5: Adt51,
fld6: [i16; 3],

},
Variant3{
fld0: [i64; 2],
fld1: *const isize,
fld2: [u64; 3],
fld3: u8,
fld4: i16,
fld5: (bool, char, [i16; 3], u128),
fld6: i64,
fld7: Adt44,

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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: *mut isize,
fld1: *mut (*const isize, (i32, u8, (*mut i32,), (*mut i32,), i16, bool, (bool, char, [i16; 3], u128))),
fld2: [u16; 2],
fld3: f32,
fld4: i16,
fld5: ((*const char, u64, i32), i128, [u64; 3], i16),
fld6: [i16; 3],
fld7: u32,

},
Variant1{
fld0: i128,

},
Variant2{
fld0: Adt46,
fld1: i128,
fld2: Adt48,
fld3: *mut i32,
fld4: [u8; 6],

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt56{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt56 {
fld0: Adt43,
fld1: *mut [i16; 3],
}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: (i32, (*mut i32,), [u16; 2]),
fld1: u128,

},
Variant1{
fld0: Adt56,
fld1: [u8; 4],
fld2: isize,
fld3: *const char,
fld4: [u8; 6],
fld5: Adt50,
fld6: i64,
fld7: [u64; 3],

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: u64,
fld1: u8,
fld2: u128,
fld3: *const char,

},
Variant1{
fld0: bool,
fld1: char,
fld2: *mut (*const isize, (i32, u8, (*mut i32,), (*mut i32,), i16, bool, (bool, char, [i16; 3], u128))),
fld3: usize,
fld4: u32,
fld5: i64,

},
Variant2{
fld0: (*const isize, (i32, u8, (*mut i32,), (*mut i32,), i16, bool, (bool, char, [i16; 3], u128))),
fld1: (u16,),
fld2: (i32, (*mut i32,), [u16; 2]),
fld3: Adt49,
fld4: [i8; 7],

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt59{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt59 {
fld0: bool,
fld1: Adt56,
fld2: (i32, (*mut i32,), [u16; 2]),
fld3: [u8; 4],
fld4: (u8, (i32, (*mut i32,), [u16; 2])),
fld5: (i32, u8, (*mut i32,), (*mut i32,), i16, bool, (bool, char, [i16; 3], u128)),
}

