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
pub fn fn0(mut _1: i64,mut _2: char,mut _3: u16,mut _4: u64,mut _5: u128) -> u128 {
mir! {
type RET = u128;
let _6: Adt18;
let _7: Adt73;
let _8: char;
let _9: [u8; 6];
let _10: bool;
let _11: isize;
let _12: &'static i128;
let _13: ();
let _14: ();
{
_4 = 1682247318299834670_u64;
_3 = 26755_u16;
RET = 54904886327586708642082465206258339894_i128 as u128;
_4 = 161245724573966481_u64 - 7193618330553281218_u64;
_1 = !3446039308360065202_i64;
_5 = 3557446793_u32 as u128;
_1 = -7900553956780180766_i64;
_2 = '\u{46d06}';
_1 = _4 as i64;
_5 = !RET;
_5 = RET | RET;
_4 = (-8031_i16) as u64;
_6 = Adt18::Variant1 { fld0: (-13372915_i32),fld1: 3955698014_u32 };
_4 = 2983336215_u32 as u64;
_6 = Adt18::Variant1 { fld0: 646895803_i32,fld1: 3729378530_u32 };
_3 = 53708_u16;
_5 = RET & RET;
_1 = !(-5815867295042418724_i64);
place!(Field::<u32>(Variant(_6, 1), 1)) = 1418520043_u32 >> _4;
_8 = _2;
_5 = (-1541_i16) as u128;
_9 = [56_u8,90_u8,204_u8,113_u8,156_u8,250_u8];
Goto(bb1)
}
bb1 = {
place!(Field::<u32>(Variant(_6, 1), 1)) = _4 as u32;
_8 = _2;
Goto(bb2)
}
bb2 = {
_9 = [48_u8,99_u8,197_u8,136_u8,184_u8,1_u8];
_4 = 11542866516018248270_u64;
_6 = Adt18::Variant0 { fld0: true,fld1: 17742998201220793122_usize,fld2: 9223372036854775807_isize,fld3: _5 };
_2 = _8;
_3 = 62825_u16;
_8 = _2;
match _3 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
62825 => bb10,
_ => bb9
}
}
bb3 = {
place!(Field::<u32>(Variant(_6, 1), 1)) = _4 as u32;
_8 = _2;
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
_6 = Adt18::Variant1 { fld0: (-1309992352_i32),fld1: 2027557157_u32 };
place!(Field::<i32>(Variant(_6, 1), 0)) = 248_u8 as i32;
Call(_8 = fn1(_9, _1, _9), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
place!(Field::<u32>(Variant(_6, 1), 1)) = 42945454_u32 * 3084843409_u32;
RET = Field::<u32>(Variant(_6, 1), 1) as u128;
_4 = _8 as u64;
match _3 {
0 => bb12,
1 => bb13,
62825 => bb15,
_ => bb14
}
}
bb12 = {
_6 = Adt18::Variant1 { fld0: (-1309992352_i32),fld1: 2027557157_u32 };
place!(Field::<i32>(Variant(_6, 1), 0)) = 248_u8 as i32;
Call(_8 = fn1(_9, _1, _9), ReturnTo(bb11), UnwindUnreachable())
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_11 = (-16_i8) as isize;
place!(Field::<u32>(Variant(_6, 1), 1)) = 1923824597_u32;
Goto(bb16)
}
bb16 = {
Call(_13 = dump_var(0_usize, 5_usize, Move(_5), 1_usize, Move(_1), 11_usize, Move(_11), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: [u8; 6],mut _2: i64,mut _3: [u8; 6]) -> char {
mir! {
type RET = char;
let _4: i8;
let _5: *const i128;
let _6: char;
let _7: f64;
let _8: [i32; 7];
let _9: u32;
let _10: i128;
let _11: bool;
let _12: isize;
let _13: f32;
let _14: u128;
let _15: *mut char;
let _16: [u128; 2];
let _17: u8;
let _18: f64;
let _19: *const Adt19;
let _20: f64;
let _21: Adt78;
let _22: Adt32;
let _23: [u128; 2];
let _24: *const usize;
let _25: *const &'static *const Adt19;
let _26: &'static &'static Adt19;
let _27: &'static u128;
let _28: u64;
let _29: &'static u128;
let _30: *const usize;
let _31: *mut f32;
let _32: u16;
let _33: f32;
let _34: isize;
let _35: [i32; 6];
let _36: Adt32;
let _37: (usize, i128);
let _38: &'static *mut u128;
let _39: &'static &'static [u128; 5];
let _40: [i32; 6];
let _41: [u32; 7];
let _42: f32;
let _43: Adt73;
let _44: i32;
let _45: ();
let _46: ();
{
RET = '\u{6e123}';
_3 = [248_u8,219_u8,111_u8,233_u8,72_u8,79_u8];
_3 = [224_u8,175_u8,113_u8,229_u8,73_u8,38_u8];
_1 = [231_u8,11_u8,40_u8,41_u8,37_u8,210_u8];
_3 = _1;
_2 = 5680346190173684053_i64 * 4924269655404076030_i64;
RET = '\u{92593}';
_3 = [216_u8,159_u8,44_u8,45_u8,76_u8,130_u8];
_1 = _3;
_2 = !6217611373827631005_i64;
_2 = (-5698658993222025787_i64);
_2 = 6297675094196623678_i64 >> (-41_i8);
RET = '\u{3d20c}';
RET = '\u{b0bb3}';
_2 = 4732722257913790425_i64 | 7434229460161684356_i64;
_3 = [1_u8,213_u8,140_u8,71_u8,2_u8,7_u8];
_1 = [83_u8,186_u8,158_u8,128_u8,178_u8,68_u8];
Goto(bb1)
}
bb1 = {
RET = '\u{10d45a}';
_1 = [77_u8,85_u8,47_u8,99_u8,106_u8,217_u8];
_4 = (-52_i8) | 99_i8;
_3 = _1;
_3 = [244_u8,39_u8,192_u8,43_u8,82_u8,115_u8];
_3 = _1;
_3 = [40_u8,87_u8,117_u8,169_u8,26_u8,230_u8];
_4 = (-75_i8) >> _2;
_6 = RET;
_3 = [55_u8,215_u8,149_u8,128_u8,221_u8,157_u8];
_3 = [113_u8,219_u8,34_u8,227_u8,178_u8,82_u8];
_1 = [106_u8,189_u8,140_u8,189_u8,77_u8,45_u8];
_2 = 1577610292_i32 as i64;
_1 = [17_u8,81_u8,214_u8,202_u8,248_u8,93_u8];
Goto(bb2)
}
bb2 = {
RET = _6;
_2 = !4608888383834441092_i64;
_7 = 9223372036854775807_isize as f64;
_3 = _1;
_7 = 81_u8 as f64;
_1 = [21_u8,190_u8,103_u8,245_u8,227_u8,204_u8];
_1 = _3;
_7 = 12405502999569489809_u64 as f64;
RET = _6;
RET = _6;
_3 = _1;
RET = _6;
_3 = [233_u8,72_u8,191_u8,185_u8,3_u8,20_u8];
_7 = 18_u8 as f64;
_3 = _1;
Call(_4 = fn2(_1, _1, _2, _6, _1, _6, _1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5 = core::ptr::addr_of!(_10);
_10 = 15914259050715166462_u64 as i128;
_3 = _1;
_12 = (-9223372036854775808_isize) >> (*_5);
(*_5) = !(-118468199467224395594705775080830436856_i128);
_15 = core::ptr::addr_of_mut!(_6);
RET = (*_15);
_14 = 230_u8 as u128;
(*_15) = RET;
_10 = 51873276728719544658477061068884807600_i128 >> _12;
_1 = _3;
_6 = RET;
(*_15) = RET;
Goto(bb4)
}
bb4 = {
_3 = [243_u8,64_u8,185_u8,251_u8,211_u8,7_u8];
_3 = [25_u8,35_u8,177_u8,186_u8,50_u8,140_u8];
_8 = [(-1956041503_i32),(-250636737_i32),1929119537_i32,2100831593_i32,1787146545_i32,(-16866454_i32),(-19972065_i32)];
(*_5) = 148006143794790733402035251448231793336_i128 | 43531360560744208535808496487588592289_i128;
RET = (*_15);
_7 = 17806241815629153683_usize as f64;
(*_15) = RET;
_11 = !true;
_13 = _12 as f32;
_14 = _11 as u128;
_9 = 1111139619_u32;
_15 = core::ptr::addr_of_mut!((*_15));
_10 = 59593_u16 as i128;
_18 = _7;
_23 = [_14,_14];
_16 = [_14,_14];
RET = (*_15);
_15 = core::ptr::addr_of_mut!((*_15));
Goto(bb5)
}
bb5 = {
_12 = !9223372036854775807_isize;
_20 = _18 + _18;
(*_5) = -(-87182255444756155882134393434607192408_i128);
_7 = _20 * _20;
_12 = -(-9223372036854775808_isize);
_3 = _1;
_17 = 139_u8 + 157_u8;
_28 = _6 as u64;
_12 = 26146_u16 as isize;
_17 = 216_u8;
_29 = &_14;
_12 = 3_isize;
RET = (*_15);
_15 = core::ptr::addr_of_mut!((*_15));
RET = (*_15);
_8 = [(-462796595_i32),(-1912914873_i32),1809620169_i32,1296174317_i32,925866876_i32,(-1028776380_i32),1516259274_i32];
_6 = RET;
_1 = [_17,_17,_17,_17,_17,_17];
_29 = &(*_29);
_17 = !27_u8;
match _12 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb8,
_ => bb7
}
}
bb6 = {
_3 = [243_u8,64_u8,185_u8,251_u8,211_u8,7_u8];
_3 = [25_u8,35_u8,177_u8,186_u8,50_u8,140_u8];
_8 = [(-1956041503_i32),(-250636737_i32),1929119537_i32,2100831593_i32,1787146545_i32,(-16866454_i32),(-19972065_i32)];
(*_5) = 148006143794790733402035251448231793336_i128 | 43531360560744208535808496487588592289_i128;
RET = (*_15);
_7 = 17806241815629153683_usize as f64;
(*_15) = RET;
_11 = !true;
_13 = _12 as f32;
_14 = _11 as u128;
_9 = 1111139619_u32;
_15 = core::ptr::addr_of_mut!((*_15));
_10 = 59593_u16 as i128;
_18 = _7;
_23 = [_14,_14];
_16 = [_14,_14];
RET = (*_15);
_15 = core::ptr::addr_of_mut!((*_15));
Goto(bb5)
}
bb7 = {
RET = '\u{10d45a}';
_1 = [77_u8,85_u8,47_u8,99_u8,106_u8,217_u8];
_4 = (-52_i8) | 99_i8;
_3 = _1;
_3 = [244_u8,39_u8,192_u8,43_u8,82_u8,115_u8];
_3 = _1;
_3 = [40_u8,87_u8,117_u8,169_u8,26_u8,230_u8];
_4 = (-75_i8) >> _2;
_6 = RET;
_3 = [55_u8,215_u8,149_u8,128_u8,221_u8,157_u8];
_3 = [113_u8,219_u8,34_u8,227_u8,178_u8,82_u8];
_1 = [106_u8,189_u8,140_u8,189_u8,77_u8,45_u8];
_2 = 1577610292_i32 as i64;
_1 = [17_u8,81_u8,214_u8,202_u8,248_u8,93_u8];
Goto(bb2)
}
bb8 = {
_14 = 245698163874848630871390827252707755251_u128;
_23 = [_14,_14];
_20 = _7 + _7;
_27 = &_14;
_18 = _20;
_9 = !1948412484_u32;
_13 = _2 as f32;
_2 = (-2470547663871857857_i64) - (-4742739673744294038_i64);
_7 = -_18;
_15 = core::ptr::addr_of_mut!(RET);
_14 = 125914958267869613441507312678524273105_u128 >> _9;
Goto(bb9)
}
bb9 = {
(*_5) = 61387_u16 as i128;
_6 = (*_15);
_13 = 11549_u16 as f32;
_18 = _7;
(*_15) = _6;
_13 = _28 as f32;
_29 = &_14;
_2 = (-3785030632210472655_i64);
_18 = _17 as f64;
_1 = [_17,_17,_17,_17,_17,_17];
_5 = core::ptr::addr_of!(_10);
RET = _6;
_31 = core::ptr::addr_of_mut!(_13);
_15 = core::ptr::addr_of_mut!((*_15));
(*_5) = !(-150899893451492114843990199720071818380_i128);
_27 = Move(_29);
RET = _6;
_15 = core::ptr::addr_of_mut!(_6);
_33 = -(*_31);
_28 = 14176291497200854903_u64 * 12180411748241926155_u64;
_28 = 9057089834314129279_u64 * 6584620570752697709_u64;
_17 = !246_u8;
_27 = &_14;
_13 = _33;
_13 = _33;
_33 = _13 - (*_31);
_3 = [_17,_17,_17,_17,_17,_17];
Goto(bb10)
}
bb10 = {
_28 = 5110940451784962631_u64;
_23 = _16;
_12 = 9223372036854775807_isize;
_30 = core::ptr::addr_of!(_37.0);
_1 = [_17,_17,_17,_17,_17,_17];
_1 = [_17,_17,_17,_17,_17,_17];
_37.1 = _9 as i128;
_15 = core::ptr::addr_of_mut!(_6);
_35 = [(-1607675713_i32),(-887100728_i32),(-91023865_i32),2095148260_i32,687562277_i32,(-266449476_i32)];
_24 = core::ptr::addr_of!((*_30));
match _12 {
0 => bb11,
1 => bb12,
2 => bb13,
9223372036854775807 => bb15,
_ => bb14
}
}
bb11 = {
_5 = core::ptr::addr_of!(_10);
_10 = 15914259050715166462_u64 as i128;
_3 = _1;
_12 = (-9223372036854775808_isize) >> (*_5);
(*_5) = !(-118468199467224395594705775080830436856_i128);
_15 = core::ptr::addr_of_mut!(_6);
RET = (*_15);
_14 = 230_u8 as u128;
(*_15) = RET;
_10 = 51873276728719544658477061068884807600_i128 >> _12;
_1 = _3;
_6 = RET;
(*_15) = RET;
Goto(bb4)
}
bb12 = {
_14 = 245698163874848630871390827252707755251_u128;
_23 = [_14,_14];
_20 = _7 + _7;
_27 = &_14;
_18 = _20;
_9 = !1948412484_u32;
_13 = _2 as f32;
_2 = (-2470547663871857857_i64) - (-4742739673744294038_i64);
_7 = -_18;
_15 = core::ptr::addr_of_mut!(RET);
_14 = 125914958267869613441507312678524273105_u128 >> _9;
Goto(bb9)
}
bb13 = {
RET = '\u{10d45a}';
_1 = [77_u8,85_u8,47_u8,99_u8,106_u8,217_u8];
_4 = (-52_i8) | 99_i8;
_3 = _1;
_3 = [244_u8,39_u8,192_u8,43_u8,82_u8,115_u8];
_3 = _1;
_3 = [40_u8,87_u8,117_u8,169_u8,26_u8,230_u8];
_4 = (-75_i8) >> _2;
_6 = RET;
_3 = [55_u8,215_u8,149_u8,128_u8,221_u8,157_u8];
_3 = [113_u8,219_u8,34_u8,227_u8,178_u8,82_u8];
_1 = [106_u8,189_u8,140_u8,189_u8,77_u8,45_u8];
_2 = 1577610292_i32 as i64;
_1 = [17_u8,81_u8,214_u8,202_u8,248_u8,93_u8];
Goto(bb2)
}
bb14 = {
_3 = [243_u8,64_u8,185_u8,251_u8,211_u8,7_u8];
_3 = [25_u8,35_u8,177_u8,186_u8,50_u8,140_u8];
_8 = [(-1956041503_i32),(-250636737_i32),1929119537_i32,2100831593_i32,1787146545_i32,(-16866454_i32),(-19972065_i32)];
(*_5) = 148006143794790733402035251448231793336_i128 | 43531360560744208535808496487588592289_i128;
RET = (*_15);
_7 = 17806241815629153683_usize as f64;
(*_15) = RET;
_11 = !true;
_13 = _12 as f32;
_14 = _11 as u128;
_9 = 1111139619_u32;
_15 = core::ptr::addr_of_mut!((*_15));
_10 = 59593_u16 as i128;
_18 = _7;
_23 = [_14,_14];
_16 = [_14,_14];
RET = (*_15);
_15 = core::ptr::addr_of_mut!((*_15));
Goto(bb5)
}
bb15 = {
(*_24) = _12 as usize;
(*_15) = RET;
_29 = &(*_27);
RET = (*_15);
_32 = 23980_u16;
_13 = 1915551256_i32 as f32;
(*_5) = _37.1 * _37.1;
(*_24) = 14529343240448380391_usize;
_35 = [2118661421_i32,(-461557278_i32),(-305205718_i32),(-563396491_i32),(-1353979212_i32),(-1358898972_i32)];
_40 = [(-97849167_i32),709371447_i32,700503435_i32,1412308410_i32,(-1008946264_i32),871598344_i32];
_16 = [(*_27),(*_29)];
_20 = _12 as f64;
_30 = core::ptr::addr_of!((*_30));
_28 = 8216202097721399980_u64;
(*_24) = 7895851851669910178_usize;
(*_30) = !0_usize;
_6 = RET;
_35 = [1760368532_i32,98414530_i32,404202558_i32,830478167_i32,935854918_i32,(-1111649309_i32)];
_41 = [_9,_9,_9,_9,_9,_9,_9];
(*_15) = RET;
(*_30) = 3_usize + 14532027508718699716_usize;
(*_24) = 15315782995610995419_usize;
Goto(bb16)
}
bb16 = {
Call(_45 = dump_var(1_usize, 8_usize, Move(_8), 3_usize, Move(_3), 37_usize, Move(_37), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(1_usize, 1_usize, Move(_1), 41_usize, Move(_41), 35_usize, Move(_35), 28_usize, Move(_28)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(1_usize, 32_usize, Move(_32), 10_usize, Move(_10), 46_usize, _46, 46_usize, _46), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: [u8; 6],mut _2: [u8; 6],mut _3: i64,mut _4: char,mut _5: [u8; 6],mut _6: char,mut _7: [u8; 6]) -> i8 {
mir! {
type RET = i8;
let _8: [u32; 7];
let _9: char;
let _10: [i32; 8];
let _11: [u128; 5];
let _12: f32;
let _13: u64;
let _14: *const &'static isize;
let _15: i32;
let _16: f64;
let _17: [i32; 8];
let _18: [u32; 7];
let _19: (((*const Adt19, usize), (*const Adt19, usize), u64), Adt32, &'static *const Adt19, usize);
let _20: f64;
let _21: bool;
let _22: f64;
let _23: isize;
let _24: i32;
let _25: i128;
let _26: ();
let _27: ();
{
_8 = [1804104085_u32,3015288110_u32,1257651199_u32,177943063_u32,3177781416_u32,1625332464_u32,50711849_u32];
_3 = 5431012908285239445_i64 - (-4608673625618128095_i64);
_5 = [151_u8,157_u8,12_u8,35_u8,107_u8,195_u8];
_8 = [4182000708_u32,4051285404_u32,2245843829_u32,3191710165_u32,3823343590_u32,3775830337_u32,1253611299_u32];
RET = (-62_i8);
RET = 2995091173_u32 as i8;
_7 = _5;
_10 = [1926927114_i32,2020138632_i32,1670622135_i32,148860171_i32,1417423996_i32,(-692207764_i32),(-1306764433_i32),(-91220118_i32)];
_11 = [227277437685306049143357103865876662319_u128,78020982138361145223957327606966147538_u128,39818240427739542127591640272447353854_u128,188051836010084496536348318904701535455_u128,15136778667417239102848302348735888000_u128];
_10 = [(-845729994_i32),2146477858_i32,(-1273289040_i32),1437025692_i32,450420299_i32,(-424241266_i32),(-518128557_i32),43454634_i32];
_6 = _4;
_9 = _6;
_9 = _4;
_1 = [14_u8,246_u8,135_u8,204_u8,47_u8,20_u8];
RET = (-109_i8) * 84_i8;
_4 = _6;
Call(_3 = fn3(_1, _8, _10, _4, _2, _7, _8, _8, _2, _1, _5, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = 1261234099554342776_i64;
RET = -(-23_i8);
_11 = [60110620394630165589499516206906098623_u128,164609735578040108778935712857727378440_u128,147326203079117705240090953941451170713_u128,42873073612983693444543174327222235664_u128,184366745357162570688862188880921868867_u128];
_1 = _2;
_7 = _1;
RET = 70_i8;
_2 = [169_u8,79_u8,132_u8,174_u8,210_u8,206_u8];
_7 = [35_u8,183_u8,51_u8,208_u8,44_u8,250_u8];
_2 = [118_u8,88_u8,136_u8,164_u8,134_u8,235_u8];
_4 = _6;
RET = 72324358_u32 as i8;
_2 = [148_u8,24_u8,102_u8,46_u8,65_u8,150_u8];
_5 = [105_u8,197_u8,5_u8,68_u8,151_u8,160_u8];
_6 = _9;
_12 = 1783861896_i32 as f32;
_12 = 2848325894_u32 as f32;
_10 = [(-1351028240_i32),(-737470533_i32),194085397_i32,1901467612_i32,1888211733_i32,(-1008219937_i32),38269344_i32,1203834482_i32];
_7 = [154_u8,243_u8,118_u8,254_u8,220_u8,66_u8];
_12 = 1090746129_u32 as f32;
_12 = (-106018980919245666272651230993906562335_i128) as f32;
_8 = [1448593689_u32,1228688746_u32,3460411420_u32,1801067588_u32,568482644_u32,528014933_u32,2482063951_u32];
_13 = 9773742147473016554_u64 << _3;
_2 = [80_u8,2_u8,7_u8,189_u8,216_u8,173_u8];
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
1261234099554342776 => bb10,
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
_1 = [44_u8,179_u8,241_u8,50_u8,6_u8,169_u8];
_11 = [325470650922087782566201707747406508846_u128,18582419644029349838055499986594981044_u128,31012536466932061863156241870387050321_u128,84169801666071036996239519745627402945_u128,282755312261322158421313084793981351169_u128];
_12 = _3 as f32;
_9 = _4;
match _3 {
1261234099554342776 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_13 = !630291894805110241_u64;
_13 = 14162950145717938170_u64 ^ 9488070230320127804_u64;
_7 = _1;
_3 = 6218272374079133492_i64 >> _13;
_12 = 305_i16 as f32;
_3 = -6447512179318838854_i64;
_15 = (-599939639_i32);
_4 = _9;
RET = (-20_i8) ^ (-116_i8);
_13 = !15047125536879606586_u64;
_7 = [24_u8,77_u8,2_u8,148_u8,163_u8,55_u8];
match _15 {
0 => bb10,
340282366920938463463374607431168271817 => bb13,
_ => bb5
}
}
bb13 = {
_17 = [_15,_15,_15,_15,_15,_15,_15,_15];
_5 = [108_u8,136_u8,135_u8,220_u8,245_u8,66_u8];
_9 = _4;
_16 = _3 as f64;
_7 = [7_u8,226_u8,67_u8,134_u8,119_u8,90_u8];
_13 = !3642749620245441354_u64;
RET = (-108_i8);
_18 = _8;
_17 = [_15,_15,_15,_15,_15,_15,_15,_15];
_7 = _2;
_19.2 = &_19.0.1.0;
RET = !(-38_i8);
_18 = [4270533004_u32,2168671732_u32,4252669731_u32,3125201011_u32,2460620588_u32,105521635_u32,814829697_u32];
_22 = 129079372623113223104528772889598103217_i128 as f64;
_5 = [137_u8,74_u8,4_u8,94_u8,101_u8,180_u8];
_19.0.0.1 = 4820533586626028613_usize;
_9 = _6;
_2 = [2_u8,182_u8,97_u8,184_u8,40_u8,92_u8];
_19.0.2 = 320900009336336623641528996218674735771_u128 as u64;
match _19.0.0.1 {
0 => bb2,
1 => bb14,
4820533586626028613 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
_3 = 1261234099554342776_i64;
RET = -(-23_i8);
_11 = [60110620394630165589499516206906098623_u128,164609735578040108778935712857727378440_u128,147326203079117705240090953941451170713_u128,42873073612983693444543174327222235664_u128,184366745357162570688862188880921868867_u128];
_1 = _2;
_7 = _1;
RET = 70_i8;
_2 = [169_u8,79_u8,132_u8,174_u8,210_u8,206_u8];
_7 = [35_u8,183_u8,51_u8,208_u8,44_u8,250_u8];
_2 = [118_u8,88_u8,136_u8,164_u8,134_u8,235_u8];
_4 = _6;
RET = 72324358_u32 as i8;
_2 = [148_u8,24_u8,102_u8,46_u8,65_u8,150_u8];
_5 = [105_u8,197_u8,5_u8,68_u8,151_u8,160_u8];
_6 = _9;
_12 = 1783861896_i32 as f32;
_12 = 2848325894_u32 as f32;
_10 = [(-1351028240_i32),(-737470533_i32),194085397_i32,1901467612_i32,1888211733_i32,(-1008219937_i32),38269344_i32,1203834482_i32];
_7 = [154_u8,243_u8,118_u8,254_u8,220_u8,66_u8];
_12 = 1090746129_u32 as f32;
_12 = (-106018980919245666272651230993906562335_i128) as f32;
_8 = [1448593689_u32,1228688746_u32,3460411420_u32,1801067588_u32,568482644_u32,528014933_u32,2482063951_u32];
_13 = 9773742147473016554_u64 << _3;
_2 = [80_u8,2_u8,7_u8,189_u8,216_u8,173_u8];
match _3 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
1261234099554342776 => bb10,
_ => bb9
}
}
bb16 = {
_20 = 18904_u16 as f64;
_7 = [173_u8,155_u8,187_u8,45_u8,92_u8,40_u8];
_18 = [2300490112_u32,529590801_u32,3146599116_u32,3389429724_u32,1464450183_u32,2601377462_u32,1719643934_u32];
RET = (-21050_i16) as i8;
_5 = [194_u8,239_u8,194_u8,208_u8,217_u8,248_u8];
_25 = !48168757443951873164316066630122067860_i128;
_21 = true ^ true;
_19.0.0.1 = 12001_u16 as usize;
_21 = !true;
_19.3 = _19.0.0.1;
_19.0.2 = _13 * _13;
_19.0.0.1 = _19.3 << _15;
_10 = _17;
_19.2 = &_19.0.1.0;
_20 = -_16;
_24 = !_15;
_3 = 1671404439934299984_i64;
_13 = _19.0.2;
_19.0.2 = _13;
_19.3 = _24 as usize;
_15 = _24 * _24;
_13 = _19.0.2;
Goto(bb17)
}
bb17 = {
Call(_26 = dump_var(2_usize, 17_usize, Move(_17), 9_usize, Move(_9), 13_usize, Move(_13), 3_usize, Move(_3)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_26 = dump_var(2_usize, 10_usize, Move(_10), 21_usize, Move(_21), 6_usize, Move(_6), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_26 = dump_var(2_usize, 15_usize, Move(_15), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [u8; 6],mut _2: [u32; 7],mut _3: [i32; 8],mut _4: char,mut _5: [u8; 6],mut _6: [u8; 6],mut _7: [u32; 7],mut _8: [u32; 7],mut _9: [u8; 6],mut _10: [u8; 6],mut _11: [u8; 6],mut _12: [u8; 6]) -> i64 {
mir! {
type RET = i64;
let _13: i8;
let _14: Adt32;
let _15: isize;
let _16: isize;
let _17: [u16; 1];
let _18: &'static Adt35;
let _19: [u128; 5];
let _20: [u8; 3];
let _21: isize;
let _22: bool;
let _23: u128;
let _24: usize;
let _25: f64;
let _26: bool;
let _27: bool;
let _28: Adt18;
let _29: u16;
let _30: [u16; 1];
let _31: &'static [u128; 2];
let _32: ();
let _33: ();
{
_13 = 49_i8;
_2 = _8;
_8 = [2882141157_u32,3897161841_u32,926462685_u32,365027580_u32,318383463_u32,1291879260_u32,2364849826_u32];
_15 = _13 as isize;
Call(_4 = fn4(_1, _11, _2, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = _12;
RET = (-5529959961410711245_i64) >> _15;
_7 = [3486758689_u32,3650456742_u32,2939825676_u32,4090108901_u32,4143384555_u32,68254096_u32,1466044234_u32];
_10 = _12;
_7 = [66143158_u32,3145184302_u32,144156342_u32,3443132214_u32,3357928239_u32,1698354015_u32,4191056478_u32];
_7 = _8;
Goto(bb2)
}
bb2 = {
RET = -(-6499573694973057953_i64);
_11 = _12;
_7 = [3650800733_u32,2110622735_u32,1153092549_u32,941936298_u32,2352690541_u32,1243602037_u32,533699250_u32];
_17 = [41743_u16];
_16 = _15;
Goto(bb3)
}
bb3 = {
_2 = [3494878749_u32,1025403149_u32,1143179985_u32,1241584857_u32,1053248024_u32,4247957426_u32,425651223_u32];
_1 = _5;
_6 = [240_u8,218_u8,125_u8,168_u8,49_u8,5_u8];
Goto(bb4)
}
bb4 = {
_4 = '\u{5d286}';
_12 = _10;
_12 = [183_u8,110_u8,151_u8,177_u8,77_u8,135_u8];
_16 = !_15;
_12 = [129_u8,95_u8,38_u8,170_u8,168_u8,53_u8];
_1 = [223_u8,170_u8,183_u8,198_u8,28_u8,123_u8];
_3 = [1591637364_i32,871726239_i32,1400352417_i32,382908436_i32,(-1751494544_i32),1964211194_i32,(-1923648193_i32),(-238082593_i32)];
_16 = -_15;
_5 = _10;
_5 = [12_u8,191_u8,213_u8,81_u8,129_u8,159_u8];
_2 = _7;
_7 = _8;
RET = 10796890137016072149_usize as i64;
_12 = _6;
_19 = [166301438664948121652794863902508316264_u128,32948722973468847925884911580302543754_u128,99275696221334868022559023457117888984_u128,6085519667669086904120268531369130282_u128,198781050252755251339331522996576552858_u128];
_2 = _8;
match _13 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
49 => bb8,
_ => bb7
}
}
bb5 = {
_2 = [3494878749_u32,1025403149_u32,1143179985_u32,1241584857_u32,1053248024_u32,4247957426_u32,425651223_u32];
_1 = _5;
_6 = [240_u8,218_u8,125_u8,168_u8,49_u8,5_u8];
Goto(bb4)
}
bb6 = {
RET = -(-6499573694973057953_i64);
_11 = _12;
_7 = [3650800733_u32,2110622735_u32,1153092549_u32,941936298_u32,2352690541_u32,1243602037_u32,533699250_u32];
_17 = [41743_u16];
_16 = _15;
Goto(bb3)
}
bb7 = {
_10 = _12;
RET = (-5529959961410711245_i64) >> _15;
_7 = [3486758689_u32,3650456742_u32,2939825676_u32,4090108901_u32,4143384555_u32,68254096_u32,1466044234_u32];
_10 = _12;
_7 = [66143158_u32,3145184302_u32,144156342_u32,3443132214_u32,3357928239_u32,1698354015_u32,4191056478_u32];
_7 = _8;
Goto(bb2)
}
bb8 = {
_21 = !_15;
_15 = _21;
_19 = [146892930700647865703756222745970996656_u128,58290209434087056543973936412312827597_u128,101008401548692303627705693425351647873_u128,23956735354785117566512941826194793197_u128,280987702690713033451219193079966296793_u128];
_22 = !true;
_12 = [115_u8,163_u8,106_u8,196_u8,211_u8,214_u8];
match _13 {
0 => bb7,
1 => bb9,
2 => bb10,
3 => bb11,
49 => bb13,
_ => bb12
}
}
bb9 = {
_2 = [3494878749_u32,1025403149_u32,1143179985_u32,1241584857_u32,1053248024_u32,4247957426_u32,425651223_u32];
_1 = _5;
_6 = [240_u8,218_u8,125_u8,168_u8,49_u8,5_u8];
Goto(bb4)
}
bb10 = {
RET = -(-6499573694973057953_i64);
_11 = _12;
_7 = [3650800733_u32,2110622735_u32,1153092549_u32,941936298_u32,2352690541_u32,1243602037_u32,533699250_u32];
_17 = [41743_u16];
_16 = _15;
Goto(bb3)
}
bb11 = {
_2 = [3494878749_u32,1025403149_u32,1143179985_u32,1241584857_u32,1053248024_u32,4247957426_u32,425651223_u32];
_1 = _5;
_6 = [240_u8,218_u8,125_u8,168_u8,49_u8,5_u8];
Goto(bb4)
}
bb12 = {
_10 = _12;
RET = (-5529959961410711245_i64) >> _15;
_7 = [3486758689_u32,3650456742_u32,2939825676_u32,4090108901_u32,4143384555_u32,68254096_u32,1466044234_u32];
_10 = _12;
_7 = [66143158_u32,3145184302_u32,144156342_u32,3443132214_u32,3357928239_u32,1698354015_u32,4191056478_u32];
_7 = _8;
Goto(bb2)
}
bb13 = {
_10 = _11;
_1 = [236_u8,189_u8,146_u8,30_u8,206_u8,254_u8];
_19 = [190068604460498107656679938831898072845_u128,311988248294252931850918496579723661292_u128,277298603555484701578614388190227929084_u128,230007359897353924834459349055993192582_u128,244550077712100325653211770227185447666_u128];
_10 = [206_u8,180_u8,130_u8,98_u8,112_u8,123_u8];
_23 = 72357592985811845673366571491784264903_u128;
_22 = true & true;
_25 = RET as f64;
_4 = '\u{36b28}';
_4 = '\u{2e62a}';
_1 = [169_u8,133_u8,250_u8,149_u8,107_u8,222_u8];
_4 = '\u{5cf0c}';
_5 = [163_u8,240_u8,60_u8,152_u8,228_u8,230_u8];
_10 = [38_u8,99_u8,133_u8,152_u8,181_u8,77_u8];
match _13 {
0 => bb10,
1 => bb5,
2 => bb9,
3 => bb4,
4 => bb14,
5 => bb15,
6 => bb16,
49 => bb18,
_ => bb17
}
}
bb14 = {
RET = -(-6499573694973057953_i64);
_11 = _12;
_7 = [3650800733_u32,2110622735_u32,1153092549_u32,941936298_u32,2352690541_u32,1243602037_u32,533699250_u32];
_17 = [41743_u16];
_16 = _15;
Goto(bb3)
}
bb15 = {
_2 = [3494878749_u32,1025403149_u32,1143179985_u32,1241584857_u32,1053248024_u32,4247957426_u32,425651223_u32];
_1 = _5;
_6 = [240_u8,218_u8,125_u8,168_u8,49_u8,5_u8];
Goto(bb4)
}
bb16 = {
RET = -(-6499573694973057953_i64);
_11 = _12;
_7 = [3650800733_u32,2110622735_u32,1153092549_u32,941936298_u32,2352690541_u32,1243602037_u32,533699250_u32];
_17 = [41743_u16];
_16 = _15;
Goto(bb3)
}
bb17 = {
_10 = _12;
RET = (-5529959961410711245_i64) >> _15;
_7 = [3486758689_u32,3650456742_u32,2939825676_u32,4090108901_u32,4143384555_u32,68254096_u32,1466044234_u32];
_10 = _12;
_7 = [66143158_u32,3145184302_u32,144156342_u32,3443132214_u32,3357928239_u32,1698354015_u32,4191056478_u32];
_7 = _8;
Goto(bb2)
}
bb18 = {
_10 = [69_u8,149_u8,109_u8,168_u8,56_u8,81_u8];
_24 = 1_usize + 6_usize;
_28 = Adt18::Variant2 { fld0: 63206_u16 };
place!(Field::<u16>(Variant(_28, 2), 0)) = RET as u16;
_16 = _15;
_9 = [168_u8,247_u8,51_u8,3_u8,184_u8,117_u8];
_16 = 31_u8 as isize;
_23 = 184986712622245759705996139799256158729_u128;
place!(Field::<u16>(Variant(_28, 2), 0)) = 34077_u16;
Goto(bb19)
}
bb19 = {
Call(_32 = dump_var(3_usize, 3_usize, Move(_3), 12_usize, Move(_12), 11_usize, Move(_11), 6_usize, Move(_6)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_32 = dump_var(3_usize, 16_usize, Move(_16), 2_usize, Move(_2), 21_usize, Move(_21), 9_usize, Move(_9)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_32 = dump_var(3_usize, 4_usize, Move(_4), 24_usize, Move(_24), 33_usize, _33, 33_usize, _33), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: [u8; 6],mut _2: [u8; 6],mut _3: [u32; 7],mut _4: [u32; 7]) -> char {
mir! {
type RET = char;
let _5: i128;
let _6: *mut char;
let _7: isize;
let _8: isize;
let _9: bool;
let _10: f64;
let _11: char;
let _12: (&'static i8, *mut *mut u128, &'static isize);
let _13: i16;
let _14: isize;
let _15: [u128; 5];
let _16: *const Adt19;
let _17: (&'static i8, *mut *mut u128, &'static isize);
let _18: Adt37;
let _19: [u16; 1];
let _20: [u128; 7];
let _21: ();
let _22: ();
{
_4 = [2313803588_u32,818517237_u32,3411020869_u32,1407002293_u32,3850010215_u32,1825996288_u32,395993872_u32];
Goto(bb1)
}
bb1 = {
_4 = _3;
RET = '\u{10ba2c}';
_5 = !(-104395016689158677146889638814660357849_i128);
_3 = [2287276048_u32,1584528495_u32,4033676015_u32,614149729_u32,3707832443_u32,2473944573_u32,858614878_u32];
_2 = _1;
_3 = [624835748_u32,1915166350_u32,3719160278_u32,2823358907_u32,1974229882_u32,651339732_u32,1193707023_u32];
RET = '\u{33d2f}';
_2 = [222_u8,101_u8,105_u8,97_u8,239_u8,79_u8];
_6 = core::ptr::addr_of_mut!(RET);
(*_6) = '\u{d7e8}';
_7 = 3122717931_u32 as isize;
_1 = _2;
_5 = 244033096141346394647267412742626681351_u128 as i128;
RET = '\u{294e1}';
_5 = 3138707248450501429_i64 as i128;
(*_6) = '\u{73398}';
_7 = !9223372036854775807_isize;
_3 = [2671607733_u32,2839840155_u32,4279080117_u32,1693470160_u32,1727114481_u32,3641203899_u32,2150458056_u32];
_1 = [157_u8,241_u8,253_u8,73_u8,10_u8,67_u8];
RET = '\u{6d1c6}';
_7 = 9223372036854775807_isize + 9223372036854775807_isize;
_4 = [3813296450_u32,2881152046_u32,269718966_u32,3991925936_u32,421960926_u32,4199198924_u32,3232593001_u32];
_8 = -_7;
_7 = -_8;
_8 = _7;
Goto(bb2)
}
bb2 = {
RET = '\u{ff776}';
_4 = [3089248939_u32,173736435_u32,4293424741_u32,3312679131_u32,538530911_u32,2401095635_u32,3355776243_u32];
RET = '\u{61b91}';
_7 = _8;
(*_6) = '\u{6c0d1}';
_5 = -111283163990829531867202723773668390346_i128;
_6 = core::ptr::addr_of_mut!(RET);
_4 = [1340661230_u32,2118083529_u32,2180072827_u32,2262002319_u32,1015194931_u32,793622541_u32,2081753426_u32];
(*_6) = '\u{10e374}';
_6 = core::ptr::addr_of_mut!(RET);
Goto(bb3)
}
bb3 = {
_4 = [920453132_u32,2484762479_u32,2589835023_u32,873398493_u32,4071932842_u32,964352690_u32,2689192653_u32];
(*_6) = '\u{49799}';
_9 = _5 < _5;
_13 = (-22224_i16);
_10 = 294933851270695651658718937132136494298_u128 as f64;
Call(_2 = fn5(_7, _7, _1, _3, _4), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_12.2 = &_8;
_6 = core::ptr::addr_of_mut!((*_6));
_1 = [111_u8,181_u8,15_u8,136_u8,78_u8,173_u8];
Goto(bb5)
}
bb5 = {
_7 = _8 >> _13;
_2 = [196_u8,122_u8,6_u8,17_u8,224_u8,243_u8];
RET = '\u{7d1e2}';
_14 = -_8;
RET = '\u{bec11}';
_9 = false | false;
_3 = _4;
_11 = RET;
_14 = _8;
(*_6) = _11;
_15 = [253680526836627588602587873288613821591_u128,168901078344235806987621931420817259369_u128,181145611551484060763503102114094178999_u128,238434701156924573510907643186395524034_u128,64590349701990041043635227708601038145_u128];
(*_6) = _11;
_12.2 = &_14;
_12.2 = &_7;
match _13 {
0 => bb3,
1 => bb6,
2 => bb7,
3 => bb8,
340282366920938463463374607431768189232 => bb10,
_ => bb9
}
}
bb6 = {
_12.2 = &_8;
_6 = core::ptr::addr_of_mut!((*_6));
_1 = [111_u8,181_u8,15_u8,136_u8,78_u8,173_u8];
Goto(bb5)
}
bb7 = {
_4 = [920453132_u32,2484762479_u32,2589835023_u32,873398493_u32,4071932842_u32,964352690_u32,2689192653_u32];
(*_6) = '\u{49799}';
_9 = _5 < _5;
_13 = (-22224_i16);
_10 = 294933851270695651658718937132136494298_u128 as f64;
Call(_2 = fn5(_7, _7, _1, _3, _4), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
RET = '\u{ff776}';
_4 = [3089248939_u32,173736435_u32,4293424741_u32,3312679131_u32,538530911_u32,2401095635_u32,3355776243_u32];
RET = '\u{61b91}';
_7 = _8;
(*_6) = '\u{6c0d1}';
_5 = -111283163990829531867202723773668390346_i128;
_6 = core::ptr::addr_of_mut!(RET);
_4 = [1340661230_u32,2118083529_u32,2180072827_u32,2262002319_u32,1015194931_u32,793622541_u32,2081753426_u32];
(*_6) = '\u{10e374}';
_6 = core::ptr::addr_of_mut!(RET);
Goto(bb3)
}
bb9 = {
_4 = _3;
RET = '\u{10ba2c}';
_5 = !(-104395016689158677146889638814660357849_i128);
_3 = [2287276048_u32,1584528495_u32,4033676015_u32,614149729_u32,3707832443_u32,2473944573_u32,858614878_u32];
_2 = _1;
_3 = [624835748_u32,1915166350_u32,3719160278_u32,2823358907_u32,1974229882_u32,651339732_u32,1193707023_u32];
RET = '\u{33d2f}';
_2 = [222_u8,101_u8,105_u8,97_u8,239_u8,79_u8];
_6 = core::ptr::addr_of_mut!(RET);
(*_6) = '\u{d7e8}';
_7 = 3122717931_u32 as isize;
_1 = _2;
_5 = 244033096141346394647267412742626681351_u128 as i128;
RET = '\u{294e1}';
_5 = 3138707248450501429_i64 as i128;
(*_6) = '\u{73398}';
_7 = !9223372036854775807_isize;
_3 = [2671607733_u32,2839840155_u32,4279080117_u32,1693470160_u32,1727114481_u32,3641203899_u32,2150458056_u32];
_1 = [157_u8,241_u8,253_u8,73_u8,10_u8,67_u8];
RET = '\u{6d1c6}';
_7 = 9223372036854775807_isize + 9223372036854775807_isize;
_4 = [3813296450_u32,2881152046_u32,269718966_u32,3991925936_u32,421960926_u32,4199198924_u32,3232593001_u32];
_8 = -_7;
_7 = -_8;
_8 = _7;
Goto(bb2)
}
bb10 = {
RET = _11;
_2 = [107_u8,153_u8,19_u8,244_u8,119_u8,88_u8];
_4 = _3;
_14 = _8;
_2 = [33_u8,157_u8,144_u8,80_u8,222_u8,216_u8];
_12.2 = &_7;
(*_6) = _11;
_3 = _4;
_12.2 = &_7;
_19 = [43590_u16];
_7 = -_14;
_2 = [60_u8,217_u8,82_u8,75_u8,207_u8,112_u8];
_10 = _7 as f64;
match _13 {
0 => bb8,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
340282366920938463463374607431768189232 => bb17,
_ => bb16
}
}
bb11 = {
_4 = _3;
RET = '\u{10ba2c}';
_5 = !(-104395016689158677146889638814660357849_i128);
_3 = [2287276048_u32,1584528495_u32,4033676015_u32,614149729_u32,3707832443_u32,2473944573_u32,858614878_u32];
_2 = _1;
_3 = [624835748_u32,1915166350_u32,3719160278_u32,2823358907_u32,1974229882_u32,651339732_u32,1193707023_u32];
RET = '\u{33d2f}';
_2 = [222_u8,101_u8,105_u8,97_u8,239_u8,79_u8];
_6 = core::ptr::addr_of_mut!(RET);
(*_6) = '\u{d7e8}';
_7 = 3122717931_u32 as isize;
_1 = _2;
_5 = 244033096141346394647267412742626681351_u128 as i128;
RET = '\u{294e1}';
_5 = 3138707248450501429_i64 as i128;
(*_6) = '\u{73398}';
_7 = !9223372036854775807_isize;
_3 = [2671607733_u32,2839840155_u32,4279080117_u32,1693470160_u32,1727114481_u32,3641203899_u32,2150458056_u32];
_1 = [157_u8,241_u8,253_u8,73_u8,10_u8,67_u8];
RET = '\u{6d1c6}';
_7 = 9223372036854775807_isize + 9223372036854775807_isize;
_4 = [3813296450_u32,2881152046_u32,269718966_u32,3991925936_u32,421960926_u32,4199198924_u32,3232593001_u32];
_8 = -_7;
_7 = -_8;
_8 = _7;
Goto(bb2)
}
bb12 = {
RET = '\u{ff776}';
_4 = [3089248939_u32,173736435_u32,4293424741_u32,3312679131_u32,538530911_u32,2401095635_u32,3355776243_u32];
RET = '\u{61b91}';
_7 = _8;
(*_6) = '\u{6c0d1}';
_5 = -111283163990829531867202723773668390346_i128;
_6 = core::ptr::addr_of_mut!(RET);
_4 = [1340661230_u32,2118083529_u32,2180072827_u32,2262002319_u32,1015194931_u32,793622541_u32,2081753426_u32];
(*_6) = '\u{10e374}';
_6 = core::ptr::addr_of_mut!(RET);
Goto(bb3)
}
bb13 = {
_4 = [920453132_u32,2484762479_u32,2589835023_u32,873398493_u32,4071932842_u32,964352690_u32,2689192653_u32];
(*_6) = '\u{49799}';
_9 = _5 < _5;
_13 = (-22224_i16);
_10 = 294933851270695651658718937132136494298_u128 as f64;
Call(_2 = fn5(_7, _7, _1, _3, _4), ReturnTo(bb4), UnwindUnreachable())
}
bb14 = {
_12.2 = &_8;
_6 = core::ptr::addr_of_mut!((*_6));
_1 = [111_u8,181_u8,15_u8,136_u8,78_u8,173_u8];
Goto(bb5)
}
bb15 = {
RET = '\u{ff776}';
_4 = [3089248939_u32,173736435_u32,4293424741_u32,3312679131_u32,538530911_u32,2401095635_u32,3355776243_u32];
RET = '\u{61b91}';
_7 = _8;
(*_6) = '\u{6c0d1}';
_5 = -111283163990829531867202723773668390346_i128;
_6 = core::ptr::addr_of_mut!(RET);
_4 = [1340661230_u32,2118083529_u32,2180072827_u32,2262002319_u32,1015194931_u32,793622541_u32,2081753426_u32];
(*_6) = '\u{10e374}';
_6 = core::ptr::addr_of_mut!(RET);
Goto(bb3)
}
bb16 = {
_12.2 = &_8;
_6 = core::ptr::addr_of_mut!((*_6));
_1 = [111_u8,181_u8,15_u8,136_u8,78_u8,173_u8];
Goto(bb5)
}
bb17 = {
_13 = -(-13839_i16);
_17.2 = &_14;
_14 = _8 ^ _8;
_9 = !false;
_13 = -(-21099_i16);
_15 = [184849532762201420717557990579870189957_u128,243451210722397105439150982243261336063_u128,243764739745479194219519109806514582109_u128,177318880625608185714488699565518913426_u128,291919536146410587376102161376844783579_u128];
_4 = _3;
Goto(bb18)
}
bb18 = {
Call(_21 = dump_var(4_usize, 4_usize, Move(_4), 14_usize, Move(_14), 3_usize, Move(_3), 7_usize, Move(_7)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_21 = dump_var(4_usize, 11_usize, Move(_11), 5_usize, Move(_5), 22_usize, _22, 22_usize, _22), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: isize,mut _2: isize,mut _3: [u8; 6],mut _4: [u32; 7],mut _5: [u32; 7]) -> [u8; 6] {
mir! {
type RET = [u8; 6];
let _6: usize;
let _7: (*const usize, *const Adt19, (i128, &'static Adt19, Adt19, u32), &'static i128);
let _8: u16;
let _9: isize;
let _10: &'static isize;
let _11: bool;
let _12: f64;
let _13: u16;
let _14: char;
let _15: [u8; 3];
let _16: char;
let _17: isize;
let _18: [u128; 8];
let _19: *const i128;
let _20: [i128; 7];
let _21: bool;
let _22: [i32; 6];
let _23: [i32; 8];
let _24: f32;
let _25: ();
let _26: ();
{
RET = [192_u8,124_u8,146_u8,6_u8,87_u8,178_u8];
_1 = (-1236_i16) as isize;
RET = [10_u8,128_u8,47_u8,255_u8,19_u8,184_u8];
_6 = !7_usize;
_1 = _2;
_7.3 = &_7.2.0;
_1 = !_2;
_5 = [1495984914_u32,3654565952_u32,792528703_u32,264541852_u32,2281115009_u32,1625895954_u32,1735693779_u32];
_7.2.1 = &_7.2.2;
_7.0 = core::ptr::addr_of!(_6);
_5 = _4;
_7.1 = core::ptr::addr_of!(_7.2.2);
_7.2.3 = 3668846795_u32 | 2768223604_u32;
_6 = !2_usize;
_7.2.3 = !1405699963_u32;
_3 = RET;
_7.2.0 = _6 as i128;
_7.2.0 = (-101435288241243221462971515503623536632_i128) & (-11352736602804093757663130083147077680_i128);
_8 = 46184_u16;
_6 = 3_usize >> _7.2.0;
_5 = [_7.2.3,_7.2.3,_7.2.3,_7.2.3,_7.2.3,_7.2.3,_7.2.3];
_5 = _4;
_6 = (-417170852_i32) as usize;
_1 = _2 + _2;
_9 = _2;
Call(_5 = fn6(RET, _1, _9, _3, _1, _4, _2, _8, _4, _3, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7.0 = core::ptr::addr_of!(_6);
_8 = 61622_u16;
_3 = [101_u8,125_u8,103_u8,178_u8,249_u8,161_u8];
_2 = _9 << _9;
RET = [107_u8,186_u8,90_u8,25_u8,217_u8,93_u8];
_7.2.3 = 217103191_u32 * 3508252286_u32;
_6 = !11637576436207604696_usize;
RET = [14_u8,250_u8,8_u8,128_u8,153_u8,234_u8];
RET = _3;
_7.0 = core::ptr::addr_of!(_6);
_7.1 = core::ptr::addr_of!(_7.2.2);
_7.1 = core::ptr::addr_of!(_7.2.2);
match _8 {
61622 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_7.2.3 = !2152029916_u32;
_10 = &_2;
_7.2.1 = &_7.2.2;
_7.2.1 = &_7.2.2;
_10 = &(*_10);
_9 = (*_10);
_9 = _1 + _2;
_13 = _8;
_7.2.1 = &_7.2.2;
_7.0 = core::ptr::addr_of!(_6);
_13 = _8 & _8;
_5 = _4;
_1 = _9;
RET = _3;
_13 = _8;
_11 = !false;
_5 = [_7.2.3,_7.2.3,_7.2.3,_7.2.3,_7.2.3,_7.2.3,_7.2.3];
_14 = '\u{7c447}';
_7.2.1 = &_7.2.2;
_15 = [91_u8,191_u8,166_u8];
_11 = _1 > (*_10);
_7.2.3 = 5872900505854205877_u64 as u32;
_2 = _9 + _1;
_7.3 = &_7.2.0;
_6 = (-26658_i16) as usize;
match _13 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
61622 => bb9,
_ => bb8
}
}
bb4 = {
Return()
}
bb5 = {
_7.0 = core::ptr::addr_of!(_6);
_8 = 61622_u16;
_3 = [101_u8,125_u8,103_u8,178_u8,249_u8,161_u8];
_2 = _9 << _9;
RET = [107_u8,186_u8,90_u8,25_u8,217_u8,93_u8];
_7.2.3 = 217103191_u32 * 3508252286_u32;
_6 = !11637576436207604696_usize;
RET = [14_u8,250_u8,8_u8,128_u8,153_u8,234_u8];
RET = _3;
_7.0 = core::ptr::addr_of!(_6);
_7.1 = core::ptr::addr_of!(_7.2.2);
_7.1 = core::ptr::addr_of!(_7.2.2);
match _8 {
61622 => bb3,
_ => bb2
}
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
_17 = _13 as isize;
_16 = _14;
_12 = 1061922157_i32 as f64;
Goto(bb10)
}
bb10 = {
RET = [144_u8,14_u8,225_u8,17_u8,219_u8,145_u8];
_14 = _16;
_7.1 = core::ptr::addr_of!(_7.2.2);
RET = [158_u8,124_u8,32_u8,171_u8,178_u8,72_u8];
_11 = !true;
RET = _3;
_13 = _8;
_16 = _14;
_4 = _5;
_7.3 = &_7.2.0;
_13 = _8 & _8;
_2 = !_9;
_16 = _14;
_19 = core::ptr::addr_of!(_7.2.0);
_12 = _13 as f64;
_7.3 = &(*_19);
_10 = &_9;
_13 = _16 as u16;
_15 = [117_u8,233_u8,202_u8];
match _8 {
0 => bb9,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb6,
5 => bb11,
61622 => bb13,
_ => bb12
}
}
bb11 = {
Return()
}
bb12 = {
_7.2.3 = !2152029916_u32;
_10 = &_2;
_7.2.1 = &_7.2.2;
_7.2.1 = &_7.2.2;
_10 = &(*_10);
_9 = (*_10);
_9 = _1 + _2;
_13 = _8;
_7.2.1 = &_7.2.2;
_7.0 = core::ptr::addr_of!(_6);
_13 = _8 & _8;
_5 = _4;
_1 = _9;
RET = _3;
_13 = _8;
_11 = !false;
_5 = [_7.2.3,_7.2.3,_7.2.3,_7.2.3,_7.2.3,_7.2.3,_7.2.3];
_14 = '\u{7c447}';
_7.2.1 = &_7.2.2;
_15 = [91_u8,191_u8,166_u8];
_11 = _1 > (*_10);
_7.2.3 = 5872900505854205877_u64 as u32;
_2 = _9 + _1;
_7.3 = &_7.2.0;
_6 = (-26658_i16) as usize;
match _13 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
61622 => bb9,
_ => bb8
}
}
bb13 = {
_20 = [(*_19),(*_19),(*_19),(*_19),(*_19),(*_19),_7.2.0];
_7.0 = core::ptr::addr_of!(_6);
_18 = [213826649941855993323090553652352446549_u128,324612134008937741590052500728844694324_u128,25360672342254827611452340658980461237_u128,61093325122660113812551743086502749751_u128,230262381720663220843640773956299585425_u128,25290520794613559654365125473302210528_u128,285316366584345445820458859418576909443_u128,331091651671708617110351293626868667604_u128];
_4 = [_7.2.3,_7.2.3,_7.2.3,_7.2.3,_7.2.3,_7.2.3,_7.2.3];
_10 = &_17;
_9 = (-2010921768_i32) as isize;
_5 = [_7.2.3,_7.2.3,_7.2.3,_7.2.3,_7.2.3,_7.2.3,_7.2.3];
_6 = 13519162430674308375_usize >> _2;
_16 = _14;
_7.2.1 = &_7.2.2;
Goto(bb14)
}
bb14 = {
_7.3 = &(*_19);
_7.2.3 = 2587816429_u32;
_9 = _1 >> _6;
_7.0 = core::ptr::addr_of!(_6);
_7.1 = core::ptr::addr_of!(_7.2.2);
_12 = 6700651901034430208_i64 as f64;
_24 = _1 as f32;
_20 = [(*_19),_7.2.0,(*_19),(*_19),(*_19),(*_19),(*_19)];
_3 = [46_u8,240_u8,196_u8,174_u8,91_u8,76_u8];
_10 = &(*_10);
_9 = !_2;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(5_usize, 4_usize, Move(_4), 8_usize, Move(_8), 16_usize, Move(_16), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(5_usize, 14_usize, Move(_14), 18_usize, Move(_18), 13_usize, Move(_13), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [u8; 6],mut _2: isize,mut _3: isize,mut _4: [u8; 6],mut _5: isize,mut _6: [u32; 7],mut _7: isize,mut _8: u16,mut _9: [u32; 7],mut _10: [u8; 6],mut _11: u16) -> [u32; 7] {
mir! {
type RET = [u32; 7];
let _12: &'static &'static Adt19;
let _13: &'static Adt19;
let _14: u16;
let _15: u32;
let _16: [u128; 8];
let _17: u8;
let _18: *mut *mut u128;
let _19: u64;
let _20: &'static [u128; 5];
let _21: u8;
let _22: char;
let _23: &'static u128;
let _24: Adt73;
let _25: usize;
let _26: &'static u8;
let _27: isize;
let _28: f32;
let _29: i32;
let _30: &'static &'static Adt19;
let _31: &'static u8;
let _32: *mut f32;
let _33: usize;
let _34: Adt18;
let _35: [bool; 6];
let _36: (*const usize, *const Adt19, (i128, &'static Adt19, Adt19, u32), &'static i128);
let _37: isize;
let _38: *const &'static Adt35;
let _39: (i128, &'static Adt19, Adt19, u32);
let _40: i64;
let _41: [u32; 7];
let _42: ();
let _43: ();
{
_7 = 222_u8 as isize;
_8 = _11;
_9 = _6;
_4 = _1;
_12 = &_13;
_5 = '\u{3412d}' as isize;
_9 = [3014337108_u32,3143777947_u32,1323042755_u32,32914312_u32,3295751220_u32,2634316125_u32,1766752928_u32];
RET = _6;
_12 = &(*_12);
RET = [4128024886_u32,83830464_u32,1764098089_u32,319972479_u32,3667235003_u32,2432884030_u32,2055615053_u32];
_2 = _3 | _7;
_3 = _2;
_12 = &(*_12);
_2 = _7;
_6 = RET;
Goto(bb1)
}
bb1 = {
_10 = [104_u8,194_u8,4_u8,142_u8,180_u8,15_u8];
_11 = !_8;
_8 = _11;
_3 = _2;
_5 = 1871132786_i32 as isize;
Goto(bb2)
}
bb2 = {
_7 = 27_i8 as isize;
_10 = _1;
_1 = _4;
_4 = _10;
_2 = !_3;
_3 = _5;
_10 = [24_u8,39_u8,22_u8,77_u8,241_u8,8_u8];
_12 = &_13;
_5 = 14865670857476729427_u64 as isize;
_5 = -_2;
_12 = &(*_12);
_8 = _11;
_5 = _3 | _7;
_3 = 6_usize as isize;
_14 = 462748243_u32 as u16;
_6 = [28020016_u32,1326661578_u32,3406733652_u32,3211838170_u32,417432118_u32,3805060877_u32,1131618628_u32];
_1 = [140_u8,98_u8,146_u8,24_u8,164_u8,252_u8];
_7 = _5;
_2 = !_7;
_12 = &(*_12);
RET = _9;
_14 = _11 * _11;
_10 = [154_u8,156_u8,15_u8,148_u8,10_u8,250_u8];
_3 = -_7;
_2 = 3816813007_u32 as isize;
_12 = &_13;
RET = [2256431330_u32,1295383000_u32,3446579497_u32,1221766856_u32,2789276120_u32,2099656481_u32,3225449542_u32];
_14 = !_11;
_2 = _7;
Goto(bb3)
}
bb3 = {
_15 = 3409705028_u32;
_16 = [70516029328707577863360895670641452688_u128,29309907105885635933832634384431070487_u128,255986801175483125113665261127495291990_u128,224065726458695815154473801879813031112_u128,170684064413387127383494157226840237343_u128,250375129856609260326546183463502133446_u128,98365226650994070916920906860251752975_u128,55271857529397962950300192128560183831_u128];
_9 = [_15,_15,_15,_15,_15,_15,_15];
_11 = _14 ^ _14;
_7 = 7564483_i32 as isize;
_16 = [332717952815680732140839384041520298408_u128,194331456510249385905751585095700747334_u128,190532026664594198278090720974717078630_u128,103778333091428603970425513986855620921_u128,198707143597413888040029820039465905670_u128,313528061731353783958333405200670182766_u128,270371657872677208203016316542046605212_u128,295004078136630579927346764676912396950_u128];
_8 = _11 + _14;
Goto(bb4)
}
bb4 = {
_4 = [166_u8,24_u8,74_u8,220_u8,64_u8,159_u8];
_2 = !_3;
_19 = '\u{cab3a}' as u64;
_11 = _19 as u16;
_17 = 231_u8;
_11 = _8;
RET = [_15,_15,_15,_15,_15,_15,_15];
_1 = [_17,_17,_17,_17,_17,_17];
_12 = &(*_12);
Goto(bb5)
}
bb5 = {
_6 = [_15,_15,_15,_15,_15,_15,_15];
RET = [_15,_15,_15,_15,_15,_15,_15];
_8 = 5_usize as u16;
_14 = _11;
_4 = [_17,_17,_17,_17,_17,_17];
Goto(bb6)
}
bb6 = {
_16 = [255060287841362628383549781030467011291_u128,244901621420716085058890453603159153769_u128,278938975034974802241310987832836145871_u128,223176170880768915445978925325561251176_u128,195038649692350331701478350849835232428_u128,29940290091048766352600139898998548552_u128,319977028230195610114026892543597216338_u128,159023263137177966432873806783970315514_u128];
_2 = _19 as isize;
_15 = 1548948497_u32 + 1378136140_u32;
_19 = !12536477613662305913_u64;
_11 = !_14;
_12 = &(*_12);
_14 = !_8;
match _17 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
231 => bb9,
_ => bb8
}
}
bb7 = {
_6 = [_15,_15,_15,_15,_15,_15,_15];
RET = [_15,_15,_15,_15,_15,_15,_15];
_8 = 5_usize as u16;
_14 = _11;
_4 = [_17,_17,_17,_17,_17,_17];
Goto(bb6)
}
bb8 = {
_15 = 3409705028_u32;
_16 = [70516029328707577863360895670641452688_u128,29309907105885635933832634384431070487_u128,255986801175483125113665261127495291990_u128,224065726458695815154473801879813031112_u128,170684064413387127383494157226840237343_u128,250375129856609260326546183463502133446_u128,98365226650994070916920906860251752975_u128,55271857529397962950300192128560183831_u128];
_9 = [_15,_15,_15,_15,_15,_15,_15];
_11 = _14 ^ _14;
_7 = 7564483_i32 as isize;
_16 = [332717952815680732140839384041520298408_u128,194331456510249385905751585095700747334_u128,190532026664594198278090720974717078630_u128,103778333091428603970425513986855620921_u128,198707143597413888040029820039465905670_u128,313528061731353783958333405200670182766_u128,270371657872677208203016316542046605212_u128,295004078136630579927346764676912396950_u128];
_8 = _11 + _14;
Goto(bb4)
}
bb9 = {
_9 = [_15,_15,_15,_15,_15,_15,_15];
_9 = _6;
_6 = [_15,_15,_15,_15,_15,_15,_15];
_11 = _8;
Call(_25 = core::intrinsics::bswap(6_usize), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_16 = [148536846341692695571427246751098879241_u128,297949752828373650322876288852219168378_u128,104816230512931829981821208261786764168_u128,304830172132377025816581757433791025847_u128,54439229168033659849289707503331064995_u128,30067452132103734792833683704244836294_u128,61633612122878110180344551082367482877_u128,204474769275909187510607346882855811654_u128];
_5 = !_7;
_21 = _17 - _17;
_25 = 30036_i16 as usize;
_22 = '\u{6b646}';
_26 = &_21;
_6 = [_15,_15,_15,_15,_15,_15,_15];
_4 = [(*_26),(*_26),(*_26),(*_26),(*_26),(*_26)];
_27 = _2 << (*_26);
_19 = 7749671698520531593_u64 - 11753991117618690919_u64;
_11 = !_8;
_27 = -_3;
_6 = [_15,_15,_15,_15,_15,_15,_15];
RET = [_15,_15,_15,_15,_15,_15,_15];
_19 = 10484945012895266151_u64 - 11373070642626880347_u64;
_12 = &(*_12);
_17 = _22 as u8;
_12 = &(*_12);
_16 = [206409949860778779947005225106062297147_u128,180436517151072546720456066563777198015_u128,301764987379283223518817337826886734213_u128,291959582244674681091344242163435203492_u128,312458127229023420589740521595929804496_u128,74933278484026214934791443379301635465_u128,22352772398550248876935401746746844792_u128,286208991643453754624934428257987716008_u128];
RET = [_15,_15,_15,_15,_15,_15,_15];
_1 = [(*_26),(*_26),(*_26),(*_26),(*_26),_21];
_29 = -1658862311_i32;
_4 = [(*_26),_17,_17,(*_26),_21,_21];
_1 = [_21,(*_26),(*_26),(*_26),(*_26),(*_26)];
Call(_3 = fn7(), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_25 = 3_usize;
_7 = _27;
match _9[_25] {
0 => bb1,
1 => bb9,
2 => bb7,
3409705028 => bb12,
_ => bb8
}
}
bb12 = {
_28 = RET[_25] as f32;
_1 = [(*_26),_10[_25],_10[_25],_4[_25],_21,_4[_25]];
_26 = &(*_26);
_23 = &_16[_25];
_6[_25] = _19 as u32;
_33 = (-291263680557346710_i64) as usize;
_37 = -_5;
_35 = [true,true,true,false,false,true];
_26 = &_1[_25];
_37 = _5;
_30 = &(*_12);
_3 = _27;
_35[_25] = false;
_6 = [RET[_25],RET[_25],_15,_9[_25],_15,RET[_25],_9[_25]];
RET[_25] = _15;
_36.2.3 = !_6[_25];
_36.2.3 = _8 as u32;
match (*_23) {
0 => bb6,
1 => bb5,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
291959582244674681091344242163435203492 => bb19,
_ => bb18
}
}
bb13 = {
_7 = 27_i8 as isize;
_10 = _1;
_1 = _4;
_4 = _10;
_2 = !_3;
_3 = _5;
_10 = [24_u8,39_u8,22_u8,77_u8,241_u8,8_u8];
_12 = &_13;
_5 = 14865670857476729427_u64 as isize;
_5 = -_2;
_12 = &(*_12);
_8 = _11;
_5 = _3 | _7;
_3 = 6_usize as isize;
_14 = 462748243_u32 as u16;
_6 = [28020016_u32,1326661578_u32,3406733652_u32,3211838170_u32,417432118_u32,3805060877_u32,1131618628_u32];
_1 = [140_u8,98_u8,146_u8,24_u8,164_u8,252_u8];
_7 = _5;
_2 = !_7;
_12 = &(*_12);
RET = _9;
_14 = _11 * _11;
_10 = [154_u8,156_u8,15_u8,148_u8,10_u8,250_u8];
_3 = -_7;
_2 = 3816813007_u32 as isize;
_12 = &_13;
RET = [2256431330_u32,1295383000_u32,3446579497_u32,1221766856_u32,2789276120_u32,2099656481_u32,3225449542_u32];
_14 = !_11;
_2 = _7;
Goto(bb3)
}
bb14 = {
_16 = [148536846341692695571427246751098879241_u128,297949752828373650322876288852219168378_u128,104816230512931829981821208261786764168_u128,304830172132377025816581757433791025847_u128,54439229168033659849289707503331064995_u128,30067452132103734792833683704244836294_u128,61633612122878110180344551082367482877_u128,204474769275909187510607346882855811654_u128];
_5 = !_7;
_21 = _17 - _17;
_25 = 30036_i16 as usize;
_22 = '\u{6b646}';
_26 = &_21;
_6 = [_15,_15,_15,_15,_15,_15,_15];
_4 = [(*_26),(*_26),(*_26),(*_26),(*_26),(*_26)];
_27 = _2 << (*_26);
_19 = 7749671698520531593_u64 - 11753991117618690919_u64;
_11 = !_8;
_27 = -_3;
_6 = [_15,_15,_15,_15,_15,_15,_15];
RET = [_15,_15,_15,_15,_15,_15,_15];
_19 = 10484945012895266151_u64 - 11373070642626880347_u64;
_12 = &(*_12);
_17 = _22 as u8;
_12 = &(*_12);
_16 = [206409949860778779947005225106062297147_u128,180436517151072546720456066563777198015_u128,301764987379283223518817337826886734213_u128,291959582244674681091344242163435203492_u128,312458127229023420589740521595929804496_u128,74933278484026214934791443379301635465_u128,22352772398550248876935401746746844792_u128,286208991643453754624934428257987716008_u128];
RET = [_15,_15,_15,_15,_15,_15,_15];
_1 = [(*_26),(*_26),(*_26),(*_26),(*_26),_21];
_29 = -1658862311_i32;
_4 = [(*_26),_17,_17,(*_26),_21,_21];
_1 = [_21,(*_26),(*_26),(*_26),(*_26),(*_26)];
Call(_3 = fn7(), ReturnTo(bb11), UnwindUnreachable())
}
bb15 = {
_15 = 3409705028_u32;
_16 = [70516029328707577863360895670641452688_u128,29309907105885635933832634384431070487_u128,255986801175483125113665261127495291990_u128,224065726458695815154473801879813031112_u128,170684064413387127383494157226840237343_u128,250375129856609260326546183463502133446_u128,98365226650994070916920906860251752975_u128,55271857529397962950300192128560183831_u128];
_9 = [_15,_15,_15,_15,_15,_15,_15];
_11 = _14 ^ _14;
_7 = 7564483_i32 as isize;
_16 = [332717952815680732140839384041520298408_u128,194331456510249385905751585095700747334_u128,190532026664594198278090720974717078630_u128,103778333091428603970425513986855620921_u128,198707143597413888040029820039465905670_u128,313528061731353783958333405200670182766_u128,270371657872677208203016316542046605212_u128,295004078136630579927346764676912396950_u128];
_8 = _11 + _14;
Goto(bb4)
}
bb16 = {
_10 = [104_u8,194_u8,4_u8,142_u8,180_u8,15_u8];
_11 = !_8;
_8 = _11;
_3 = _2;
_5 = 1871132786_i32 as isize;
Goto(bb2)
}
bb17 = {
_6 = [_15,_15,_15,_15,_15,_15,_15];
RET = [_15,_15,_15,_15,_15,_15,_15];
_8 = 5_usize as u16;
_14 = _11;
_4 = [_17,_17,_17,_17,_17,_17];
Goto(bb6)
}
bb18 = {
_16 = [255060287841362628383549781030467011291_u128,244901621420716085058890453603159153769_u128,278938975034974802241310987832836145871_u128,223176170880768915445978925325561251176_u128,195038649692350331701478350849835232428_u128,29940290091048766352600139898998548552_u128,319977028230195610114026892543597216338_u128,159023263137177966432873806783970315514_u128];
_2 = _19 as isize;
_15 = 1548948497_u32 + 1378136140_u32;
_19 = !12536477613662305913_u64;
_11 = !_14;
_12 = &(*_12);
_14 = !_8;
match _17 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
231 => bb9,
_ => bb8
}
}
bb19 = {
_19 = !16053410655449967854_u64;
_32 = core::ptr::addr_of_mut!(_28);
_10 = _4;
_7 = _11 as isize;
_36.2.3 = !_15;
_36.1 = core::ptr::addr_of!(_36.2.2);
_22 = '\u{e7e57}';
RET = [_36.2.3,_6[_25],_15,_6[_25],_36.2.3,_36.2.3,_36.2.3];
_4[_25] = _1[_25];
_5 = _27 * _27;
_22 = '\u{a9697}';
Goto(bb20)
}
bb20 = {
Call(_42 = dump_var(6_usize, 2_usize, Move(_2), 29_usize, Move(_29), 22_usize, Move(_22), 9_usize, Move(_9)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_42 = dump_var(6_usize, 17_usize, Move(_17), 33_usize, Move(_33), 35_usize, Move(_35), 10_usize, Move(_10)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_42 = dump_var(6_usize, 25_usize, Move(_25), 11_usize, Move(_11), 3_usize, Move(_3), 16_usize, Move(_16)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7() -> isize {
mir! {
type RET = isize;
let _1: &'static i8;
let _2: ((*const Adt19, usize), (*const Adt19, usize), u64);
let _3: ([bool; 6], &'static u32);
let _4: (*const Adt19, usize);
let _5: [u32; 7];
let _6: i64;
let _7: u8;
let _8: (&'static i8, *mut *mut u128, &'static isize);
let _9: [i32; 8];
let _10: [u32; 7];
let _11: f64;
let _12: f32;
let _13: isize;
let _14: [u8; 6];
let _15: i64;
let _16: ();
let _17: ();
{
RET = 9223372036854775807_isize;
RET = 9223372036854775807_isize << 302129689727448300638134122799809366659_u128;
RET = 9223372036854775807_isize;
RET = 648709770_u32 as isize;
RET = -9223372036854775807_isize;
RET = (-9223372036854775808_isize) ^ 82_isize;
RET = (-84_isize);
RET = -(-9223372036854775808_isize);
RET = (-9223372036854775808_isize) & 9223372036854775807_isize;
_2.0.1 = !9379807612679111969_usize;
RET = 2103710005319860072_i64 as isize;
_2.2 = 4877878496934964988_u64;
_2.1.1 = !_2.0.1;
_2.1.1 = _2.0.1 & _2.0.1;
_2.2 = !876679427506041293_u64;
RET = 120_u8 as isize;
_2.2 = 8073136412660587848_u64 - 2110133896017969260_u64;
RET = 999486689_i32 as isize;
_2.1.1 = _2.0.1;
_2.0.1 = !_2.1.1;
Call(_2.1.1 = fn8(RET, _2.0.1, _2.2, _2.0.1, _2.2, RET, _2.2, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2.2 = !15396804101748981959_u64;
Goto(bb2)
}
bb2 = {
RET = 9223372036854775807_isize;
_2.1.1 = _2.0.1 - _2.0.1;
_3.0 = [true,false,false,false,false,true];
_2.1.1 = _2.0.1;
_2.0.1 = (-14517262530051243493933266131308928054_i128) as usize;
_2.1.1 = !_2.0.1;
_2.0.1 = _2.1.1 - _2.1.1;
_2.1.1 = _2.0.1;
_2.0.1 = _2.1.1 << _2.1.1;
_2.2 = !10380222277009677523_u64;
RET = (-9223372036854775808_isize);
_2.2 = !8616600611225288476_u64;
_2.2 = 13186163293606430688_u64;
_2.0.1 = _2.1.1;
_4.1 = _2.1.1;
_3.0 = [false,false,true,false,false,true];
_2.2 = 869155877107317492_u64;
_2.1.1 = _4.1;
_2.2 = 16963087597728384117_u64 * 133077662714394854_u64;
RET = 12667_i16 as isize;
_2.2 = 11594_u16 as u64;
_3.0 = [false,true,true,false,false,true];
RET = -(-50_isize);
RET = (-9223372036854775808_isize);
_3.0 = [true,true,true,true,true,false];
_2.1.1 = !_2.0.1;
_3.0 = [false,true,false,true,false,true];
RET = (-9223372036854775808_isize) ^ (-105_isize);
Goto(bb3)
}
bb3 = {
_5 = [1006829420_u32,2750609763_u32,25937855_u32,2003298347_u32,4075161261_u32,1453953830_u32,890688605_u32];
_5 = [3802966645_u32,4209991854_u32,2742322722_u32,656000138_u32,1653584748_u32,1947519876_u32,3262225904_u32];
_6 = (-9171937896131841665_i64);
RET = (-80_isize);
_2.1.1 = _4.1 + _2.0.1;
RET = 9223372036854775807_isize;
RET = (-9223372036854775808_isize);
_4.1 = 83240159841240793064666345447914884896_i128 as usize;
_5 = [3068158392_u32,3862252507_u32,2027656259_u32,1326884586_u32,1837001418_u32,2671627812_u32,3102741556_u32];
_2.1.1 = _4.1;
_4.1 = !_2.0.1;
_3.0 = [false,true,true,true,false,false];
_5 = [354947708_u32,2334312613_u32,101326975_u32,3213631983_u32,172928704_u32,2967015251_u32,851911830_u32];
_7 = 81_u8;
_2.1.1 = false as usize;
_4.1 = _2.1.1 - _2.0.1;
RET = 58414_u16 as isize;
_3.0 = [true,false,false,true,true,false];
_2.0.1 = _6 as usize;
match _6 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
340282366920938463454202669535636369791 => bb9,
_ => bb8
}
}
bb4 = {
RET = 9223372036854775807_isize;
_2.1.1 = _2.0.1 - _2.0.1;
_3.0 = [true,false,false,false,false,true];
_2.1.1 = _2.0.1;
_2.0.1 = (-14517262530051243493933266131308928054_i128) as usize;
_2.1.1 = !_2.0.1;
_2.0.1 = _2.1.1 - _2.1.1;
_2.1.1 = _2.0.1;
_2.0.1 = _2.1.1 << _2.1.1;
_2.2 = !10380222277009677523_u64;
RET = (-9223372036854775808_isize);
_2.2 = !8616600611225288476_u64;
_2.2 = 13186163293606430688_u64;
_2.0.1 = _2.1.1;
_4.1 = _2.1.1;
_3.0 = [false,false,true,false,false,true];
_2.2 = 869155877107317492_u64;
_2.1.1 = _4.1;
_2.2 = 16963087597728384117_u64 * 133077662714394854_u64;
RET = 12667_i16 as isize;
_2.2 = 11594_u16 as u64;
_3.0 = [false,true,true,false,false,true];
RET = -(-50_isize);
RET = (-9223372036854775808_isize);
_3.0 = [true,true,true,true,true,false];
_2.1.1 = !_2.0.1;
_3.0 = [false,true,false,true,false,true];
RET = (-9223372036854775808_isize) ^ (-105_isize);
Goto(bb3)
}
bb5 = {
_2.2 = !15396804101748981959_u64;
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
_2.0.1 = (-11_i8) as usize;
_2.0.1 = '\u{481ff}' as usize;
_6 = -(-1123715631739358153_i64);
_4.1 = _2.0.1 + _2.1.1;
_5 = [1802325189_u32,4066695437_u32,2401709873_u32,2933990778_u32,2473336003_u32,2036045141_u32,1871452743_u32];
RET = 9223372036854775807_isize;
_6 = !5984000680791319433_i64;
_3.0 = [false,false,true,false,true,false];
match _7 {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
81 => bb11,
_ => bb10
}
}
bb10 = {
Return()
}
bb11 = {
_6 = (-7812635222684687592_i64);
_2.1.1 = _4.1;
RET = -(-9223372036854775808_isize);
_3.0 = [false,true,true,false,false,true];
_5 = [444688995_u32,3713340326_u32,561159451_u32,3146414015_u32,3550417034_u32,2054935699_u32,2178767109_u32];
_8.2 = &RET;
_2.2 = 16659085477010789489_u64 >> _2.1.1;
_8.2 = &RET;
Goto(bb12)
}
bb12 = {
RET = 107_isize + 9223372036854775807_isize;
_5 = [288056702_u32,3707493717_u32,4010956381_u32,3123348636_u32,832717401_u32,2548685739_u32,320661169_u32];
_8.2 = &RET;
_4.1 = !_2.0.1;
RET = 37593_u16 as isize;
_8.2 = &RET;
_2.2 = !6013017617947306534_u64;
_2.2 = !405913165572878707_u64;
_10 = [1577226307_u32,1145854236_u32,332244360_u32,2703650375_u32,2342770503_u32,2095993917_u32,987988730_u32];
RET = (-9223372036854775808_isize);
_11 = _7 as f64;
_2.2 = !16597509198527060863_u64;
_8.2 = &RET;
Goto(bb13)
}
bb13 = {
_3.0 = [false,false,false,true,true,false];
RET = false as isize;
_9 = [1916960327_i32,(-53851958_i32),(-2130823664_i32),774469686_i32,1950019616_i32,182885204_i32,904870292_i32,1627708921_i32];
_12 = _6 as f32;
_2.0.1 = _4.1 + _4.1;
_14 = [_7,_7,_7,_7,_7,_7];
_2.2 = !12596001461196879083_u64;
_8.2 = &RET;
_3.0 = [false,true,true,true,true,true];
_10 = _5;
match _6 {
0 => bb9,
1 => bb11,
2 => bb14,
3 => bb15,
4 => bb16,
340282366920938463455561972209083523864 => bb18,
_ => bb17
}
}
bb14 = {
_2.2 = !15396804101748981959_u64;
Goto(bb2)
}
bb15 = {
_6 = (-7812635222684687592_i64);
_2.1.1 = _4.1;
RET = -(-9223372036854775808_isize);
_3.0 = [false,true,true,false,false,true];
_5 = [444688995_u32,3713340326_u32,561159451_u32,3146414015_u32,3550417034_u32,2054935699_u32,2178767109_u32];
_8.2 = &RET;
_2.2 = 16659085477010789489_u64 >> _2.1.1;
_8.2 = &RET;
Goto(bb12)
}
bb16 = {
Return()
}
bb17 = {
_2.0.1 = (-11_i8) as usize;
_2.0.1 = '\u{481ff}' as usize;
_6 = -(-1123715631739358153_i64);
_4.1 = _2.0.1 + _2.1.1;
_5 = [1802325189_u32,4066695437_u32,2401709873_u32,2933990778_u32,2473336003_u32,2036045141_u32,1871452743_u32];
RET = 9223372036854775807_isize;
_6 = !5984000680791319433_i64;
_3.0 = [false,false,true,false,true,false];
match _7 {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
81 => bb11,
_ => bb10
}
}
bb18 = {
_9 = [2028386693_i32,(-1118863733_i32),199704222_i32,2066940194_i32,(-337455053_i32),1542246470_i32,950079362_i32,1823495236_i32];
_13 = _4.1 as isize;
_8.2 = &_13;
_10 = [4226247816_u32,1629882035_u32,2953314471_u32,2289554846_u32,1366719565_u32,1322861241_u32,2525857411_u32];
_9 = [(-1529037183_i32),(-290119305_i32),(-1184265293_i32),758823261_i32,(-879411432_i32),968056324_i32,(-1773068731_i32),69736820_i32];
_9 = [(-933857616_i32),(-558745383_i32),(-753235388_i32),(-856660026_i32),(-421612673_i32),(-1921816284_i32),(-702888437_i32),1628802880_i32];
_10 = [159708736_u32,37503971_u32,3245293823_u32,464268321_u32,2295742904_u32,2973373398_u32,4081247458_u32];
_8.2 = &RET;
Goto(bb19)
}
bb19 = {
Call(_16 = dump_var(7_usize, 10_usize, Move(_10), 13_usize, Move(_13), 14_usize, Move(_14), 17_usize, _17), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: isize,mut _2: usize,mut _3: u64,mut _4: usize,mut _5: u64,mut _6: isize,mut _7: u64,mut _8: isize) -> usize {
mir! {
type RET = usize;
let _9: char;
let _10: isize;
let _11: *const &'static i8;
let _12: f64;
let _13: char;
let _14: char;
let _15: [char; 4];
let _16: [u8; 3];
let _17: f64;
let _18: bool;
let _19: f64;
let _20: u64;
let _21: isize;
let _22: char;
let _23: &'static u32;
let _24: char;
let _25: isize;
let _26: &'static i8;
let _27: isize;
let _28: [i32; 6];
let _29: f64;
let _30: [i32; 7];
let _31: [u128; 5];
let _32: [u16; 1];
let _33: (((*const Adt19, usize), (*const Adt19, usize), u64), Adt32, &'static *const Adt19, usize);
let _34: f64;
let _35: i32;
let _36: [u128; 8];
let _37: *const *mut f32;
let _38: &'static u128;
let _39: u8;
let _40: &'static i64;
let _41: *const &'static *const Adt19;
let _42: [i32; 8];
let _43: isize;
let _44: [i128; 7];
let _45: &'static *mut u128;
let _46: &'static u128;
let _47: [u32; 7];
let _48: Adt35;
let _49: ();
let _50: ();
{
RET = !_2;
RET = 309806548848113060656877940834604607887_u128 as usize;
RET = !_2;
Call(_6 = core::intrinsics::transmute(RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = '\u{c66d6}';
_9 = '\u{f6aa8}';
_13 = _9;
_14 = _9;
_6 = _8 & _8;
_1 = false as isize;
_16 = [248_u8,71_u8,48_u8];
_15 = [_13,_13,_13,_9];
_17 = _6 as f64;
_1 = _6;
_17 = _5 as f64;
_14 = _9;
RET = _9 as usize;
_17 = 55_u8 as f64;
_6 = false as isize;
_16 = [213_u8,195_u8,166_u8];
_10 = _17 as isize;
_13 = _14;
_3 = 4546_u16 as u64;
Goto(bb2)
}
bb2 = {
_7 = _17 as u64;
_6 = 1882423597912297295_i64 as isize;
_5 = !_3;
_18 = true;
_9 = _13;
_15 = [_14,_13,_14,_13];
_9 = _14;
RET = !_4;
_1 = _10 ^ _10;
Call(_4 = fn9(_1, _2, _14, _9, _1, _5, _13, _1, _6, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_14 = _9;
_18 = _17 != _17;
_8 = _10 | _10;
_6 = -_10;
Goto(bb4)
}
bb4 = {
_2 = _4;
_20 = _5 ^ _3;
_19 = _17 * _17;
_8 = !_10;
Goto(bb5)
}
bb5 = {
_2 = !_4;
_15 = [_14,_9,_14,_13];
_2 = _18 as usize;
_6 = -_1;
_12 = _17 * _19;
_24 = _14;
_1 = -_6;
_21 = 3749635706_u32 as isize;
_20 = _3;
_1 = -_6;
_1 = _2 as isize;
_22 = _24;
_14 = _24;
_9 = _24;
Goto(bb6)
}
bb6 = {
_20 = _3;
_8 = 7449_i16 as isize;
RET = !_2;
_12 = (-47_i8) as f64;
_3 = _20 ^ _20;
_15 = [_24,_13,_22,_14];
_1 = _10 * _10;
_25 = _6 >> _4;
_24 = _9;
RET = _19 as usize;
_21 = _25 + _8;
_14 = _22;
_4 = !_2;
_24 = _9;
_3 = _7 >> _21;
Goto(bb7)
}
bb7 = {
_4 = _2 & RET;
_2 = _4;
_29 = _19;
_22 = _13;
_28 = [1674764305_i32,1635997528_i32,(-1107740685_i32),(-1700819714_i32),(-1025142194_i32),(-457447372_i32)];
_4 = _2;
_11 = core::ptr::addr_of!(_26);
_3 = _7;
_2 = RET;
Goto(bb8)
}
bb8 = {
_25 = _21 << _21;
_19 = 1158846625_i32 as f64;
_27 = 65_u8 as isize;
_11 = core::ptr::addr_of!((*_11));
_14 = _22;
_29 = _17;
_28 = [1539622412_i32,(-472861427_i32),(-319127366_i32),1091546948_i32,958278404_i32,2118096747_i32];
_19 = _29 - _17;
_18 = _7 < _20;
_28 = [(-1872342150_i32),(-657798525_i32),(-744224411_i32),1526775647_i32,449685792_i32,(-624073406_i32)];
Goto(bb9)
}
bb9 = {
_5 = !_3;
_13 = _22;
_3 = _7 | _7;
_2 = !RET;
_11 = core::ptr::addr_of!((*_11));
_14 = _9;
_30 = [(-900562247_i32),115437994_i32,1905477764_i32,(-739874408_i32),1390702641_i32,(-1177390200_i32),596029509_i32];
_17 = -_19;
_15 = [_22,_14,_9,_22];
_29 = (-23597_i16) as f64;
_5 = _3;
_8 = _25 << _25;
_13 = _22;
_24 = _9;
Call(_18 = fn18(), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_9 = _24;
_14 = _22;
_12 = _19 * _19;
_33.2 = &_33.0.0.0;
_33.0.2 = (-8793360677090143390_i64) as u64;
_17 = -_12;
_33.3 = _18 as usize;
RET = _4;
_6 = _8;
_7 = (-8255573670477170039_i64) as u64;
Goto(bb11)
}
bb11 = {
_19 = -_12;
_29 = -_12;
_14 = _22;
_1 = 9653_u16 as isize;
_10 = _6;
_33.0.2 = _3;
_34 = 60072345204305376607252886017866452759_u128 as f64;
Goto(bb12)
}
bb12 = {
_18 = false;
_35 = 22884_u16 as i32;
_17 = _29;
_33.0.1.1 = RET - RET;
_13 = _14;
_33.0.0.1 = 236903411061001963313673110226579528155_u128 as usize;
_35 = (-1599269707_i32);
_27 = !_10;
_14 = _22;
_9 = _13;
_19 = -_17;
_33.2 = &_33.0.0.0;
_27 = -_21;
_36 = [28222470689210899385724389244594351248_u128,136935706767276466329328937236885489786_u128,8063996150590011087273871229730601589_u128,42270045631458694692808104436995108995_u128,201767977149520541945845296172474224192_u128,179069884631016393823984950358076889300_u128,175800305455464784579056821391324506939_u128,190521047471711306780323195269148932745_u128];
_39 = (-1442530429127391338_i64) as u8;
_20 = !_5;
Goto(bb13)
}
bb13 = {
_19 = -_12;
_29 = -_17;
_34 = _29 + _29;
_4 = _33.3;
_4 = _33.0.1.1;
_33.0.1.1 = !_33.3;
_18 = true;
_9 = _22;
_41 = core::ptr::addr_of!(_33.2);
_2 = 2922993564_u32 as usize;
_41 = core::ptr::addr_of!(_33.2);
_24 = _9;
_13 = _24;
_19 = _33.0.1.1 as f64;
_24 = _22;
_5 = _33.0.2 >> _10;
_13 = _9;
match _35 {
0 => bb4,
1 => bb14,
2 => bb15,
3 => bb16,
340282366920938463463374607430168941749 => bb18,
_ => bb17
}
}
bb14 = {
_18 = false;
_35 = 22884_u16 as i32;
_17 = _29;
_33.0.1.1 = RET - RET;
_13 = _14;
_33.0.0.1 = 236903411061001963313673110226579528155_u128 as usize;
_35 = (-1599269707_i32);
_27 = !_10;
_14 = _22;
_9 = _13;
_19 = -_17;
_33.2 = &_33.0.0.0;
_27 = -_21;
_36 = [28222470689210899385724389244594351248_u128,136935706767276466329328937236885489786_u128,8063996150590011087273871229730601589_u128,42270045631458694692808104436995108995_u128,201767977149520541945845296172474224192_u128,179069884631016393823984950358076889300_u128,175800305455464784579056821391324506939_u128,190521047471711306780323195269148932745_u128];
_39 = (-1442530429127391338_i64) as u8;
_20 = !_5;
Goto(bb13)
}
bb15 = {
_19 = -_12;
_29 = -_12;
_14 = _22;
_1 = 9653_u16 as isize;
_10 = _6;
_33.0.2 = _3;
_34 = 60072345204305376607252886017866452759_u128 as f64;
Goto(bb12)
}
bb16 = {
_9 = '\u{c66d6}';
_9 = '\u{f6aa8}';
_13 = _9;
_14 = _9;
_6 = _8 & _8;
_1 = false as isize;
_16 = [248_u8,71_u8,48_u8];
_15 = [_13,_13,_13,_9];
_17 = _6 as f64;
_1 = _6;
_17 = _5 as f64;
_14 = _9;
RET = _9 as usize;
_17 = 55_u8 as f64;
_6 = false as isize;
_16 = [213_u8,195_u8,166_u8];
_10 = _17 as isize;
_13 = _14;
_3 = 4546_u16 as u64;
Goto(bb2)
}
bb17 = {
_5 = !_3;
_13 = _22;
_3 = _7 | _7;
_2 = !RET;
_11 = core::ptr::addr_of!((*_11));
_14 = _9;
_30 = [(-900562247_i32),115437994_i32,1905477764_i32,(-739874408_i32),1390702641_i32,(-1177390200_i32),596029509_i32];
_17 = -_19;
_15 = [_22,_14,_9,_22];
_29 = (-23597_i16) as f64;
_5 = _3;
_8 = _25 << _25;
_13 = _22;
_24 = _9;
Call(_18 = fn18(), ReturnTo(bb10), UnwindUnreachable())
}
bb18 = {
_33.3 = !_4;
_20 = _5;
_33.0.1.1 = _4;
_27 = !_6;
_28 = [_35,_35,_35,_35,_35,_35];
_33.0.1.1 = _35 as usize;
_19 = -_12;
(*_41) = &_33.0.0.0;
_28 = [_35,_35,_35,_35,_35,_35];
_35 = 1841434220_i32 << _5;
_8 = 116976880604781603395241185913692487679_u128 as isize;
_28 = [_35,_35,_35,_35,_35,_35];
_15 = [_9,_24,_22,_14];
_47 = [3213049156_u32,1781250245_u32,582711427_u32,2390497100_u32,1939195387_u32,3622689073_u32,3352675947_u32];
_25 = (-162542883479126500468856638952701038444_i128) as isize;
_28 = [_35,_35,_35,_35,_35,_35];
_43 = !_27;
_42 = [_35,_35,_35,_35,_35,_35,_35,_35];
_33.0.0.1 = RET + _4;
Goto(bb19)
}
bb19 = {
Call(_49 = dump_var(8_usize, 14_usize, Move(_14), 2_usize, Move(_2), 25_usize, Move(_25), 35_usize, Move(_35)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_49 = dump_var(8_usize, 3_usize, Move(_3), 15_usize, Move(_15), 22_usize, Move(_22), 30_usize, Move(_30)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_49 = dump_var(8_usize, 10_usize, Move(_10), 27_usize, Move(_27), 36_usize, Move(_36), 6_usize, Move(_6)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_49 = dump_var(8_usize, 47_usize, Move(_47), 5_usize, Move(_5), 50_usize, _50, 50_usize, _50), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: isize,mut _2: usize,mut _3: char,mut _4: char,mut _5: isize,mut _6: u64,mut _7: char,mut _8: isize,mut _9: isize,mut _10: isize) -> usize {
mir! {
type RET = usize;
let _11: &'static i8;
let _12: u128;
let _13: *const usize;
let _14: f32;
let _15: bool;
let _16: f64;
let _17: (usize, i128);
let _18: *const usize;
let _19: (i128, &'static Adt19, Adt19, u32);
let _20: *const &'static i8;
let _21: char;
let _22: char;
let _23: [u128; 7];
let _24: u64;
let _25: bool;
let _26: i32;
let _27: [u16; 1];
let _28: Adt37;
let _29: f32;
let _30: isize;
let _31: f32;
let _32: char;
let _33: isize;
let _34: i16;
let _35: &'static &'static Adt19;
let _36: &'static Adt35;
let _37: bool;
let _38: ();
let _39: ();
{
_3 = _7;
RET = 156701198214584010842411053295728324727_i128 as usize;
_4 = _7;
_7 = _3;
_8 = _1;
Goto(bb1)
}
bb1 = {
_6 = 16210233078199704379_u64 >> _5;
RET = _2;
_8 = -_10;
_6 = 1250893376071880382_u64;
_8 = _5;
_2 = RET + RET;
_4 = _7;
_9 = _8;
_9 = !_8;
_7 = _3;
_9 = _1 * _10;
_4 = _3;
_4 = _3;
_10 = 106567704982990774042047825167943502868_u128 as isize;
RET = _6 as usize;
_7 = _4;
_6 = false as u64;
_2 = (-24167_i16) as usize;
Call(_8 = fn10(_5, _10, _5, _9, _7, _7, _5), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = _3;
RET = !_2;
_2 = !RET;
_7 = _3;
_4 = _7;
_3 = _7;
_9 = _8;
_7 = _4;
_1 = _9;
RET = !_2;
_8 = _9;
_1 = 19_u8 as isize;
_9 = _5;
_8 = _9 >> _9;
RET = _6 as usize;
_7 = _4;
_8 = _5 * _9;
_1 = -_8;
_13 = core::ptr::addr_of!(_2);
_8 = !_9;
_7 = _4;
Goto(bb3)
}
bb3 = {
(*_13) = 30_i8 as usize;
_2 = RET | RET;
_12 = 155650149890575128329842549603446610919_u128;
_15 = _1 == _5;
_3 = _7;
_4 = _3;
RET = 54395_u16 as usize;
_10 = _9;
_8 = _10 & _9;
_17.0 = !(*_13);
_17.1 = !(-167969824834614092639471761063359526874_i128);
_4 = _3;
_16 = _1 as f64;
_16 = _6 as f64;
_19.3 = 1881753091_u32;
_12 = 131031274504083664532213438251814107565_u128 * 184669162165523276310745413601722507617_u128;
_4 = _3;
RET = 4677017193893468778_i64 as usize;
_3 = _7;
_15 = !true;
Goto(bb4)
}
bb4 = {
_2 = !_17.0;
_14 = 38666_u16 as f32;
_20 = core::ptr::addr_of!(_11);
_4 = _7;
_19.3 = !3612841119_u32;
(*_13) = _17.0 + _17.0;
_3 = _4;
_7 = _4;
_6 = 17851399262959668524_u64 << _1;
_19.2 = Adt19::Variant0 { fld0: _14,fld1: _3,fld2: _16,fld3: _12,fld4: (*_13),fld5: 1702036255_i32,fld6: 2076469587748345532_i64,fld7: _17.1 };
_3 = _4;
Goto(bb5)
}
bb5 = {
place!(Field::<i128>(Variant(_19.2, 0), 7)) = _17.1 >> _10;
Goto(bb6)
}
bb6 = {
(*_13) = _7 as usize;
(*_13) = _17.0;
RET = (*_13) >> _2;
place!(Field::<i32>(Variant(_19.2, 0), 5)) = (-147878359_i32);
_19.1 = &_19.2;
place!(Field::<u128>(Variant(_19.2, 0), 3)) = (-503346852598066412_i64) as u128;
_4 = Field::<char>(Variant(_19.2, 0), 1);
place!(Field::<f32>(Variant(_19.2, 0), 0)) = _14;
_10 = _5 >> _8;
(*_13) = _17.0 + Field::<usize>(Variant(_19.2, 0), 4);
_19.3 = 1656568615_u32;
_19.1 = &_19.2;
place!(Field::<i32>(Variant(_19.2, 0), 5)) = 216_u8 as i32;
_22 = _7;
place!(Field::<i64>(Variant(_19.2, 0), 6)) = _12 as i64;
_23 = [_12,_12,_12,_12,_12,Field::<u128>(Variant(_19.2, 0), 3),_12];
_19.0 = -Field::<i128>(Variant(_19.2, 0), 7);
_25 = !_15;
place!(Field::<i32>(Variant(_19.2, 0), 5)) = (-1021185905_i32);
_14 = Field::<f32>(Variant(_19.2, 0), 0) + Field::<f32>(Variant(_19.2, 0), 0);
_24 = _6 + _6;
place!(Field::<f64>(Variant(_19.2, 0), 2)) = _16 + _16;
_15 = _25;
Goto(bb7)
}
bb7 = {
_22 = _4;
_17 = (_2, _19.0);
(*_13) = Field::<i64>(Variant(_19.2, 0), 6) as usize;
_17.0 = RET | Field::<usize>(Variant(_19.2, 0), 4);
_9 = _19.3 as isize;
_24 = Field::<f32>(Variant(_19.2, 0), 0) as u64;
_14 = _19.3 as f32;
_4 = _7;
Goto(bb8)
}
bb8 = {
_7 = _22;
_2 = RET ^ _17.0;
place!(Field::<i32>(Variant(_19.2, 0), 5)) = 1407022374_i32;
place!(Field::<usize>(Variant(_19.2, 0), 4)) = RET;
match Field::<i32>(Variant(_19.2, 0), 5) {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb7,
1407022374 => bb9,
_ => bb5
}
}
bb9 = {
_28 = Adt37::Variant2 { fld0: _19.0,fld1: _7,fld2: _6 };
_4 = _3;
Goto(bb10)
}
bb10 = {
_3 = _22;
_5 = _10;
_27 = [11618_u16];
RET = !Field::<usize>(Variant(_19.2, 0), 4);
_6 = _10 as u64;
_19.1 = &_19.2;
_19.1 = &_19.2;
_22 = Field::<char>(Variant(_28, 2), 1);
_3 = _4;
_9 = -_1;
place!(Field::<i64>(Variant(_19.2, 0), 6)) = Field::<i32>(Variant(_19.2, 0), 5) as i64;
place!(Field::<char>(Variant(_28, 2), 1)) = _22;
_3 = _4;
_19.3 = !438308019_u32;
_19.3 = _15 as u32;
match Field::<i32>(Variant(_19.2, 0), 5) {
0 => bb7,
1 => bb6,
2 => bb3,
3 => bb9,
4 => bb8,
1407022374 => bb12,
_ => bb11
}
}
bb11 = {
_4 = _3;
RET = !_2;
_2 = !RET;
_7 = _3;
_4 = _7;
_3 = _7;
_9 = _8;
_7 = _4;
_1 = _9;
RET = !_2;
_8 = _9;
_1 = 19_u8 as isize;
_9 = _5;
_8 = _9 >> _9;
RET = _6 as usize;
_7 = _4;
_8 = _5 * _9;
_1 = -_8;
_13 = core::ptr::addr_of!(_2);
_8 = !_9;
_7 = _4;
Goto(bb3)
}
bb12 = {
_31 = Field::<f32>(Variant(_19.2, 0), 0);
_14 = _12 as f32;
_23 = [Field::<u128>(Variant(_19.2, 0), 3),_12,_12,_12,Field::<u128>(Variant(_19.2, 0), 3),_12,_12];
_1 = _8 | _9;
_19.3 = 2330026648_u32;
place!(Field::<u64>(Variant(_28, 2), 2)) = _19.3 as u64;
RET = !_17.0;
_32 = _4;
_17.0 = Field::<char>(Variant(_28, 2), 1) as usize;
place!(Field::<char>(Variant(_28, 2), 1)) = _32;
_3 = _4;
_18 = Move(_13);
RET = _2 | Field::<usize>(Variant(_19.2, 0), 4);
_18 = core::ptr::addr_of!(place!(Field::<usize>(Variant(_19.2, 0), 4)));
_15 = Field::<i128>(Variant(_19.2, 0), 7) < _17.1;
_15 = _25;
_30 = _5 >> _6;
_33 = _30;
_20 = core::ptr::addr_of!((*_20));
SetDiscriminant(_19.2, 2);
Goto(bb13)
}
bb13 = {
Call(_38 = dump_var(9_usize, 30_usize, Move(_30), 4_usize, Move(_4), 27_usize, Move(_27), 25_usize, Move(_25)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_38 = dump_var(9_usize, 15_usize, Move(_15), 1_usize, Move(_1), 33_usize, Move(_33), 5_usize, Move(_5)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_38 = dump_var(9_usize, 8_usize, Move(_8), 24_usize, Move(_24), 39_usize, _39, 39_usize, _39), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: char,mut _6: char,mut _7: isize) -> isize {
mir! {
type RET = isize;
let _8: &'static [u128; 2];
let _9: f32;
let _10: i16;
let _11: [u8; 3];
let _12: u64;
let _13: char;
let _14: isize;
let _15: &'static i64;
let _16: f32;
let _17: *const i32;
let _18: &'static i128;
let _19: Adt19;
let _20: bool;
let _21: ([bool; 6], &'static u32);
let _22: (usize, i128);
let _23: isize;
let _24: [i32; 7];
let _25: &'static isize;
let _26: f32;
let _27: isize;
let _28: isize;
let _29: *mut u128;
let _30: [u128; 5];
let _31: u32;
let _32: u16;
let _33: i128;
let _34: f64;
let _35: bool;
let _36: [i32; 6];
let _37: Adt18;
let _38: f64;
let _39: isize;
let _40: &'static &'static Adt19;
let _41: ();
let _42: ();
{
_7 = -_1;
_6 = _5;
_5 = _6;
RET = _1;
_1 = !_7;
_4 = RET;
_1 = _3 ^ RET;
_12 = 2448162320547017743_u64;
_1 = _4;
_6 = _5;
_12 = (-1097674975_i32) as u64;
_10 = _12 as i16;
_5 = _6;
_6 = _5;
_11 = [113_u8,145_u8,70_u8];
_7 = -_1;
_3 = _1 << _1;
_3 = -RET;
_13 = _5;
Goto(bb1)
}
bb1 = {
_2 = 349312657_u32 as isize;
_9 = 3560972654_u32 as f32;
_12 = 16679013588884797908_u64;
_5 = _6;
RET = !_7;
_10 = 1537_u16 as i16;
_6 = _13;
_13 = _6;
_7 = _3;
_3 = -_1;
Goto(bb2)
}
bb2 = {
_14 = -_1;
_3 = !_14;
RET = _14;
RET = _1 | _1;
_13 = _6;
_2 = _13 as isize;
RET = 33720_u16 as isize;
_6 = _13;
_7 = _4;
_16 = _9 * _9;
_3 = _2;
RET = -_3;
RET = -_7;
_13 = _5;
_14 = !_2;
_12 = 15903206711023618937_u64;
_1 = -_14;
_14 = _4;
_7 = 63_u8 as isize;
_12 = 5233228612705399022_u64;
_9 = _16;
_13 = _6;
_6 = _5;
_2 = -_14;
_11 = [236_u8,174_u8,163_u8];
_9 = _16 - _16;
Goto(bb3)
}
bb3 = {
_5 = _13;
_4 = RET;
_12 = 17655427161036627053_u64 >> _1;
_11 = [147_u8,217_u8,170_u8];
_7 = false as isize;
_13 = _6;
_6 = _13;
RET = _14 >> _12;
_9 = 992193498_i32 as f32;
_5 = _6;
_5 = _6;
_10 = !(-29955_i16);
_14 = _4;
_3 = !RET;
_2 = 1896191146_i32 as isize;
_20 = _3 <= _14;
_1 = RET & _2;
_9 = _16;
_1 = RET;
_2 = RET + _3;
_11 = [63_u8,89_u8,61_u8];
Goto(bb4)
}
bb4 = {
_6 = _13;
_9 = _16 * _16;
_13 = _6;
_14 = _3 | _3;
_2 = _3;
_1 = (-2083707604_i32) as isize;
_7 = -RET;
_10 = _20 as i16;
_13 = _5;
Goto(bb5)
}
bb5 = {
RET = _2 | _2;
_3 = _14 & _7;
_10 = 317603535_u32 as i16;
_4 = _14 | _14;
_10 = 23184_i16 + 21144_i16;
_16 = _9 + _9;
_16 = _9 - _9;
_10 = (-5733_i16);
_16 = (-86516005203627308412913718406710667456_i128) as f32;
_11 = [50_u8,251_u8,13_u8];
_1 = !_2;
_21.0 = [_20,_20,_20,_20,_20,_20];
_4 = _10 as isize;
_13 = _5;
_12 = 47_u8 as u64;
_2 = !_1;
_3 = _14 >> RET;
_3 = _7;
RET = _2 << _3;
_1 = !_14;
_21.0 = [_20,_20,_20,_20,_20,_20];
_2 = !_7;
_18 = &_22.1;
_6 = _13;
match _10 {
0 => bb4,
340282366920938463463374607431768205723 => bb7,
_ => bb6
}
}
bb6 = {
_6 = _13;
_9 = _16 * _16;
_13 = _6;
_14 = _3 | _3;
_2 = _3;
_1 = (-2083707604_i32) as isize;
_7 = -RET;
_10 = _20 as i16;
_13 = _5;
Goto(bb5)
}
bb7 = {
_5 = _6;
_16 = _9 + _9;
_25 = &_3;
_25 = &(*_25);
_22 = (3923990511818806665_usize, (-67996501744383308262732224167886533839_i128));
_12 = 11770832437778497470_u64;
_24 = [651419184_i32,671558083_i32,483462060_i32,1909865734_i32,1654353767_i32,1329448040_i32,896896405_i32];
_6 = _13;
_23 = _14;
_6 = _13;
_27 = _20 as isize;
_18 = &_22.1;
_3 = _14;
match _22.1 {
0 => bb1,
1 => bb5,
2 => bb6,
3 => bb8,
272285865176555155200642383263881677617 => bb10,
_ => bb9
}
}
bb8 = {
_2 = 349312657_u32 as isize;
_9 = 3560972654_u32 as f32;
_12 = 16679013588884797908_u64;
_5 = _6;
RET = !_7;
_10 = 1537_u16 as i16;
_6 = _13;
_13 = _6;
_7 = _3;
_3 = -_1;
Goto(bb2)
}
bb9 = {
_5 = _13;
_4 = RET;
_12 = 17655427161036627053_u64 >> _1;
_11 = [147_u8,217_u8,170_u8];
_7 = false as isize;
_13 = _6;
_6 = _13;
RET = _14 >> _12;
_9 = 992193498_i32 as f32;
_5 = _6;
_5 = _6;
_10 = !(-29955_i16);
_14 = _4;
_3 = !RET;
_2 = 1896191146_i32 as isize;
_20 = _3 <= _14;
_1 = RET & _2;
_9 = _16;
_1 = RET;
_2 = RET + _3;
_11 = [63_u8,89_u8,61_u8];
Goto(bb4)
}
bb10 = {
_11 = [24_u8,15_u8,74_u8];
_9 = -_16;
_16 = _9 - _9;
_25 = &_3;
_11 = [74_u8,77_u8,79_u8];
_6 = _13;
_28 = _12 as isize;
_16 = (-8419411393192620499_i64) as f32;
_5 = _6;
_22.0 = (-1024478123_i32) as usize;
_22.1 = (-18924009489263077389537675799089290297_i128);
_9 = _22.0 as f32;
_21.0 = [_20,_20,_20,_20,_20,_20];
_18 = &_22.1;
_12 = 12737330131958664590_u64 | 1010938974776699077_u64;
_26 = -_9;
_28 = _7 + _23;
RET = _1;
_21.0 = [_20,_20,_20,_20,_20,_20];
_11 = [210_u8,147_u8,194_u8];
_9 = _16;
_33 = _22.1;
Call(_34 = fn11(Move(_25), Move(_18), RET, _28, _2, _14, RET), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_6 = _13;
_27 = !RET;
_1 = -_23;
_7 = _6 as isize;
_23 = _1 - _3;
_35 = _34 > _34;
_9 = _16;
_21.0 = [_35,_35,_35,_35,_35,_35];
_24 = [983777037_i32,(-1906568336_i32),597406930_i32,(-1366950303_i32),(-1668793170_i32),(-1548319422_i32),(-1274588870_i32)];
_1 = _14;
_37 = Adt18::Variant3 { fld0: _12,fld1: _13,fld2: _22.0,fld3: _34,fld4: _10,fld5: (-1222049798_i32),fld6: 11168_u16,fld7: 1261890488_u32 };
Call(_32 = core::intrinsics::bswap(9643_u16), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_30 = [156574408844352933293631591679434928757_u128,12169339863491190751436239086878381738_u128,13314145274480707573023117819543742276_u128,261444789014215106660082134541364391367_u128,123280112279685203598119693686455095927_u128];
_3 = _2 & _28;
RET = _23;
place!(Field::<char>(Variant(_37, 3), 1)) = _5;
place!(Field::<char>(Variant(_37, 3), 1)) = _13;
_22.0 = !Field::<usize>(Variant(_37, 3), 2);
_2 = 2393746928_u32 as isize;
_12 = Field::<u64>(Variant(_37, 3), 0) - Field::<u64>(Variant(_37, 3), 0);
_13 = Field::<char>(Variant(_37, 3), 1);
_20 = !_35;
place!(Field::<f64>(Variant(_37, 3), 3)) = _34 * _34;
place!(Field::<i16>(Variant(_37, 3), 4)) = _10;
_7 = _23 >> RET;
_4 = _7;
_38 = Field::<f64>(Variant(_37, 3), 3);
_25 = &_7;
_2 = !_4;
_38 = Field::<f64>(Variant(_37, 3), 3) - Field::<f64>(Variant(_37, 3), 3);
_17 = core::ptr::addr_of!(place!(Field::<i32>(Variant(_37, 3), 5)));
place!(Field::<usize>(Variant(_37, 3), 2)) = _22.0 - _22.0;
_39 = _7 & _23;
_38 = Field::<f64>(Variant(_37, 3), 3);
Goto(bb13)
}
bb13 = {
Call(_41 = dump_var(10_usize, 14_usize, Move(_14), 3_usize, Move(_3), 13_usize, Move(_13), 6_usize, Move(_6)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_41 = dump_var(10_usize, 28_usize, Move(_28), 35_usize, Move(_35), 4_usize, Move(_4), 23_usize, Move(_23)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_41 = dump_var(10_usize, 5_usize, Move(_5), 10_usize, Move(_10), 1_usize, Move(_1), 42_usize, _42), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: &'static isize,mut _2: &'static i128,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize) -> f64 {
mir! {
type RET = f64;
let _8: *const usize;
let _9: *const i32;
let _10: u16;
let _11: [bool; 6];
let _12: &'static isize;
let _13: [i16; 3];
let _14: [i16; 3];
let _15: &'static i8;
let _16: Adt19;
let _17: [bool; 1];
let _18: ();
let _19: ();
{
_1 = &_7;
RET = 6_usize as f64;
_3 = -_4;
_1 = &_3;
Goto(bb1)
}
bb1 = {
RET = 45871270686104253388643705737965797127_i128 as f64;
_1 = &_3;
_1 = &_5;
_1 = &_5;
_7 = !_4;
_1 = &_7;
RET = (-38_i8) as f64;
_5 = _6 * _3;
_6 = _7;
_1 = &_5;
_5 = _6 & _3;
RET = 6_usize as f64;
Call(_8 = fn12(), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = &_3;
_4 = _7 + _5;
_10 = 1828_u16;
RET = (-12184_i16) as f64;
RET = 281976503001208632969082795852396133377_u128 as f64;
_11 = [false,true,false,false,false,true];
_4 = _6;
_3 = -_4;
_1 = &_3;
_7 = '\u{10bcc5}' as isize;
_6 = !_4;
_1 = &(*_1);
_11 = [true,true,true,true,true,false];
RET = 16847638372221740251_u64 as f64;
_1 = &(*_1);
_3 = _6 ^ _6;
_3 = RET as isize;
_1 = &_7;
_10 = !17958_u16;
_1 = &_7;
_11 = [true,false,false,true,false,true];
_1 = &_4;
_12 = &_6;
_6 = -_5;
Goto(bb3)
}
bb3 = {
RET = 221_u8 as f64;
_3 = RET as isize;
_3 = -_5;
_12 = &_7;
_4 = _5 + _5;
_1 = &(*_12);
_11 = [false,true,false,false,true,false];
_3 = (-125_i8) as isize;
RET = (-64_i8) as f64;
_1 = Move(_12);
_13 = [(-29323_i16),(-409_i16),(-8974_i16)];
_12 = &_7;
_4 = !_5;
_1 = &_3;
_14 = [(-8487_i16),(-23780_i16),(-15451_i16)];
_13 = [(-6978_i16),10408_i16,10997_i16];
_6 = -_4;
_4 = 54_u8 as isize;
_14 = _13;
_14 = [22403_i16,(-18632_i16),(-9121_i16)];
RET = 2_usize as f64;
_3 = _5 & _6;
_3 = _5;
_4 = _3;
_10 = 30443_u16;
_10 = (-1235268086_i32) as u16;
_10 = 21611_u16 & 22705_u16;
RET = (-13_i8) as f64;
Call(_6 = core::intrinsics::transmute(_5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_3 = !_6;
_11 = [true,false,false,true,false,false];
_4 = -_3;
_10 = 12495_u16 >> _6;
_12 = &_6;
_3 = _4;
_12 = &_4;
_13 = [(-9840_i16),(-3955_i16),11844_i16];
_1 = &_4;
RET = _3 as f64;
Goto(bb5)
}
bb5 = {
Call(_18 = dump_var(11_usize, 6_usize, Move(_6), 7_usize, Move(_7), 5_usize, Move(_5), 10_usize, Move(_10)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12() -> *const usize {
mir! {
type RET = *const usize;
let _1: f64;
let _2: i64;
let _3: i16;
let _4: Adt32;
let _5: &'static isize;
let _6: u128;
let _7: &'static Adt35;
let _8: isize;
let _9: [u16; 1];
let _10: usize;
let _11: isize;
let _12: isize;
let _13: isize;
let _14: *const i16;
let _15: bool;
let _16: [bool; 1];
let _17: (*const usize, *const Adt19, (i128, &'static Adt19, Adt19, u32), &'static i128);
let _18: char;
let _19: [bool; 1];
let _20: ();
let _21: ();
{
_1 = 223998073885071096347737231764250026665_u128 as f64;
_1 = 1315920045653614584_i64 as f64;
_2 = -4693750079200190980_i64;
_2 = -414010981104997746_i64;
_2 = 35059583803260337_u64 as i64;
_2 = 278363177052156053433590881531906546628_u128 as i64;
Call(RET = fn13(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = 24350_i16 << _2;
_3 = 7029_i16;
_2 = (-74_i8) as i64;
_3 = 13205_i16;
Goto(bb2)
}
bb2 = {
_3 = 3_usize as i16;
_1 = _2 as f64;
_1 = 2_usize as f64;
_3 = (-24602_i16);
_3 = 9223372036854775807_isize as i16;
_2 = 1703539354216531011_usize as i64;
_3 = 25296_i16;
_2 = !8959557111845954168_i64;
_3 = 9223372036854775807_isize as i16;
_2 = (-8915015643638640156_i64);
_1 = 858589080_u32 as f64;
_2 = !(-621349876073047469_i64);
_2 = 456860296997501796_i64 >> _3;
_3 = (-9803_i16);
_3 = (-24254_i16);
_1 = _3 as f64;
_2 = -(-2479733068472345628_i64);
_1 = _2 as f64;
_1 = 9223372036854775807_isize as f64;
match _3 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768187202 => bb8,
_ => bb7
}
}
bb3 = {
_3 = 24350_i16 << _2;
_3 = 7029_i16;
_2 = (-74_i8) as i64;
_3 = 13205_i16;
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
_2 = 1042157448559097165_i64;
_1 = _3 as f64;
_1 = 60168_u16 as f64;
_6 = !163619626242337056124387916504357893981_u128;
_3 = !14663_i16;
_3 = !(-1819_i16);
_3 = (-132944852261184140071799098407626253979_i128) as i16;
_1 = _3 as f64;
_3 = 13051_i16;
_6 = 82261601086428139376898106803328121913_u128;
_2 = _3 as i64;
_2 = -2449764987204452513_i64;
_8 = !(-9223372036854775808_isize);
Goto(bb9)
}
bb9 = {
_1 = 3_usize as f64;
_2 = (-221296733749911014_i64) >> _6;
_8 = 9223372036854775807_isize;
_9 = [25138_u16];
_11 = 3105527068836835168_usize as isize;
_2 = 5043465401742105653_i64 ^ (-462069676581380565_i64);
_2 = !(-7042491260374533976_i64);
RET = core::ptr::addr_of!(_10);
RET = core::ptr::addr_of!((*RET));
(*RET) = !1496829684815832557_usize;
match _3 {
0 => bb1,
1 => bb10,
13051 => bb12,
_ => bb11
}
}
bb10 = {
_2 = 1042157448559097165_i64;
_1 = _3 as f64;
_1 = 60168_u16 as f64;
_6 = !163619626242337056124387916504357893981_u128;
_3 = !14663_i16;
_3 = !(-1819_i16);
_3 = (-132944852261184140071799098407626253979_i128) as i16;
_1 = _3 as f64;
_3 = 13051_i16;
_6 = 82261601086428139376898106803328121913_u128;
_2 = _3 as i64;
_2 = -2449764987204452513_i64;
_8 = !(-9223372036854775808_isize);
Goto(bb9)
}
bb11 = {
_3 = 24350_i16 << _2;
_3 = 7029_i16;
_2 = (-74_i8) as i64;
_3 = 13205_i16;
Goto(bb2)
}
bb12 = {
RET = core::ptr::addr_of!((*RET));
Goto(bb13)
}
bb13 = {
(*RET) = !2897555539249717261_usize;
RET = core::ptr::addr_of!(_10);
(*RET) = 2_usize;
_11 = !_8;
_5 = &_8;
RET = core::ptr::addr_of!(_10);
RET = core::ptr::addr_of!((*RET));
_17.1 = core::ptr::addr_of!(_17.2.2);
match (*RET) {
0 => bb12,
1 => bb11,
3 => bb14,
4 => bb15,
5 => bb16,
2 => bb18,
_ => bb17
}
}
bb14 = {
RET = core::ptr::addr_of!((*RET));
Goto(bb13)
}
bb15 = {
Return()
}
bb16 = {
_2 = 1042157448559097165_i64;
_1 = _3 as f64;
_1 = 60168_u16 as f64;
_6 = !163619626242337056124387916504357893981_u128;
_3 = !14663_i16;
_3 = !(-1819_i16);
_3 = (-132944852261184140071799098407626253979_i128) as i16;
_1 = _3 as f64;
_3 = 13051_i16;
_6 = 82261601086428139376898106803328121913_u128;
_2 = _3 as i64;
_2 = -2449764987204452513_i64;
_8 = !(-9223372036854775808_isize);
Goto(bb9)
}
bb17 = {
_3 = 3_usize as i16;
_1 = _2 as f64;
_1 = 2_usize as f64;
_3 = (-24602_i16);
_3 = 9223372036854775807_isize as i16;
_2 = 1703539354216531011_usize as i64;
_3 = 25296_i16;
_2 = !8959557111845954168_i64;
_3 = 9223372036854775807_isize as i16;
_2 = (-8915015643638640156_i64);
_1 = 858589080_u32 as f64;
_2 = !(-621349876073047469_i64);
_2 = 456860296997501796_i64 >> _3;
_3 = (-9803_i16);
_3 = (-24254_i16);
_1 = _3 as f64;
_2 = -(-2479733068472345628_i64);
_1 = _2 as f64;
_1 = 9223372036854775807_isize as f64;
match _3 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768187202 => bb8,
_ => bb7
}
}
bb18 = {
_1 = _2 as f64;
_3 = 1835886255_i32 as i16;
_15 = (*_5) <= (*_5);
_13 = (*_5);
(*RET) = 4_usize | 15414587741659086626_usize;
_17.0 = core::ptr::addr_of!(_10);
_13 = _8;
Goto(bb19)
}
bb19 = {
Call(_20 = dump_var(12_usize, 3_usize, Move(_3), 8_usize, Move(_8), 9_usize, Move(_9), 15_usize, Move(_15)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13() -> *const usize {
mir! {
type RET = *const usize;
let _1: isize;
let _2: char;
let _3: isize;
let _4: &'static Adt19;
let _5: [i32; 7];
let _6: [u8; 3];
let _7: isize;
let _8: [u32; 7];
let _9: [bool; 6];
let _10: isize;
let _11: (((*const Adt19, usize), (*const Adt19, usize), u64), Adt32, &'static *const Adt19, usize);
let _12: [char; 4];
let _13: [u32; 7];
let _14: u16;
let _15: *const i16;
let _16: u64;
let _17: Adt35;
let _18: *const i32;
let _19: f64;
let _20: Adt29;
let _21: [u128; 5];
let _22: char;
let _23: [u128; 2];
let _24: [i32; 8];
let _25: Adt37;
let _26: (*const Adt19, usize);
let _27: Adt19;
let _28: *mut char;
let _29: u64;
let _30: &'static &'static [u128; 5];
let _31: [u128; 2];
let _32: *mut f32;
let _33: ([bool; 6], &'static u32);
let _34: &'static i64;
let _35: i64;
let _36: bool;
let _37: i64;
let _38: [u8; 3];
let _39: [i16; 3];
let _40: f32;
let _41: bool;
let _42: [i32; 8];
let _43: [u32; 7];
let _44: f64;
let _45: (((*const Adt19, usize), (*const Adt19, usize), u64), Adt32, &'static *const Adt19, usize);
let _46: ();
let _47: ();
{
_1 = -9223372036854775807_isize;
_2 = '\u{54d0}';
_1 = -9223372036854775807_isize;
_1 = (-9223372036854775808_isize) | (-110_isize);
_3 = !_1;
_3 = 9193176265755156790_usize as isize;
_2 = '\u{3b581}';
_2 = '\u{ebb41}';
_1 = (-2803378704589144920_i64) as isize;
_1 = _3 >> _3;
_5 = [597876664_i32,1382642659_i32,(-1375217218_i32),(-542315228_i32),2004426060_i32,1815422886_i32,(-2010476552_i32)];
_2 = '\u{14965}';
_3 = !_1;
_5 = [1193059265_i32,1533621480_i32,(-1678728195_i32),1714416821_i32,(-1603098420_i32),(-367515437_i32),(-83604864_i32)];
_6 = [249_u8,243_u8,243_u8];
_2 = '\u{7cad7}';
Goto(bb1)
}
bb1 = {
_5 = [942731140_i32,1084692324_i32,1144835627_i32,1756755594_i32,(-936109588_i32),(-1876412818_i32),(-2052133510_i32)];
_5 = [609644282_i32,(-521366987_i32),(-377141587_i32),1099261342_i32,(-694421579_i32),(-534025359_i32),(-1758964427_i32)];
_6 = [12_u8,189_u8,69_u8];
Goto(bb2)
}
bb2 = {
_6 = [190_u8,153_u8,43_u8];
_5 = [(-650108471_i32),839517726_i32,(-1410627984_i32),(-800392936_i32),(-1339544605_i32),1111563664_i32,(-1277306956_i32)];
_3 = -_1;
_6 = [56_u8,149_u8,216_u8];
_2 = '\u{41a1d}';
_3 = -_1;
_3 = !_1;
_9 = [false,true,false,true,true,true];
_9 = [false,true,false,false,false,true];
_10 = _3;
_11.0.1.1 = 732225502_i32 as usize;
RET = core::ptr::addr_of!(_11.0.0.1);
_7 = _1;
_7 = !_3;
_11.2 = &_11.0.0.0;
(*RET) = _11.0.1.1 + _11.0.1.1;
_11.2 = &_11.0.1.0;
_5 = [(-1172218205_i32),734301662_i32,885456799_i32,74465508_i32,437061002_i32,(-1102447267_i32),601882065_i32];
Goto(bb3)
}
bb3 = {
_12 = [_2,_2,_2,_2];
_5 = [1903081370_i32,410817296_i32,1777286883_i32,1279466876_i32,(-1700558619_i32),1245501437_i32,(-1411377088_i32)];
_3 = _7 >> (*RET);
(*RET) = _11.0.1.1;
_9 = [false,true,false,true,true,true];
_11.0.0.1 = _11.0.1.1 ^ _11.0.1.1;
_2 = '\u{42e9f}';
_11.0.2 = _10 as u64;
_11.2 = &_11.0.1.0;
Goto(bb4)
}
bb4 = {
_11.3 = !_11.0.1.1;
_13 = [372389002_u32,2752771605_u32,1741824480_u32,1566042751_u32,617109330_u32,560821876_u32,1166986734_u32];
_6 = [54_u8,33_u8,250_u8];
_11.0.0.1 = _10 as usize;
_9 = [true,true,false,true,false,false];
_1 = 559752894_u32 as isize;
_12 = [_2,_2,_2,_2];
Goto(bb5)
}
bb5 = {
_14 = 28161_u16 ^ 16415_u16;
_5 = [(-420002510_i32),146865302_i32,(-448483635_i32),(-2095027230_i32),(-714624727_i32),1227995533_i32,(-273244271_i32)];
_7 = !_1;
_8 = [3167636955_u32,720615454_u32,3445221958_u32,4204230904_u32,1316990173_u32,883699317_u32,2557670554_u32];
(*RET) = !_11.0.1.1;
_10 = _11.0.2 as isize;
(*RET) = _11.0.1.1;
(*RET) = 3094213724995959768_i64 as usize;
_2 = '\u{2f8fb}';
_11.2 = &_11.0.1.0;
_1 = 1661194241_u32 as isize;
_12 = [_2,_2,_2,_2];
_17 = Adt35::Variant1 { fld0: 4762481890267584049_i64 };
_16 = (-82783176678896634171712022775230685232_i128) as u64;
_6 = [181_u8,118_u8,147_u8];
_1 = _10;
_11.2 = &_11.0.1.0;
Goto(bb6)
}
bb6 = {
_14 = !65456_u16;
_16 = !_11.0.2;
_1 = (-8228922000481450188_i64) as isize;
RET = core::ptr::addr_of!((*RET));
(*RET) = false as usize;
RET = core::ptr::addr_of!(_11.0.0.1);
_8 = _13;
_2 = '\u{8168f}';
_11.2 = &_11.0.1.0;
(*RET) = _11.0.1.1;
_11.0.2 = !_16;
_16 = !_11.0.2;
_11.0.1.1 = (*RET) & _11.3;
RET = core::ptr::addr_of!((*RET));
(*RET) = 32690952461179978877982955413728583782_i128 as usize;
place!(Field::<i64>(Variant(_17, 1), 0)) = (-5788660402425249304_i64) + (-9040432251759439062_i64);
place!(Field::<i64>(Variant(_17, 1), 0)) = (-6173827801838432126_i64) * (-7389750842905453798_i64);
_3 = _10;
_11.2 = &_11.0.0.0;
(*RET) = _2 as usize;
place!(Field::<i64>(Variant(_17, 1), 0)) = (-1355004392141235860_i64) + 6700444914477096079_i64;
_1 = _14 as isize;
_11.2 = &_11.0.0.0;
SetDiscriminant(_17, 1);
Goto(bb7)
}
bb7 = {
_11.0.1.1 = _11.3;
_13 = _8;
_20.fld0 = _16 as u128;
RET = core::ptr::addr_of!(_11.3);
_20.fld0 = 95088805596327046465466003191897631442_u128 << _11.0.2;
_21 = [_20.fld0,_20.fld0,_20.fld0,_20.fld0,_20.fld0];
_13 = [1381155717_u32,3006875853_u32,4003857544_u32,451172701_u32,1490377458_u32,1975371537_u32,2467546761_u32];
_11.2 = &_20.fld2;
_20.fld3 = _14 ^ _14;
_23 = [_20.fld0,_20.fld0];
_23 = [_20.fld0,_20.fld0];
RET = core::ptr::addr_of!(_11.3);
Goto(bb8)
}
bb8 = {
RET = core::ptr::addr_of!((*RET));
_9 = [true,true,false,false,false,false];
_11.0.2 = 36_u8 as u64;
Goto(bb9)
}
bb9 = {
_11.0.0.0 = core::ptr::addr_of!(_27);
_1 = _10 - _10;
_14 = _20.fld3;
_1 = _10 ^ _10;
_11.0.2 = !_16;
_28 = core::ptr::addr_of_mut!(_22);
_26.1 = !(*RET);
(*RET) = _26.1 & _11.0.0.1;
_3 = _1;
_20.fld1 = 1810024123383842506_i64 - 7456918079737479973_i64;
_10 = -_3;
_23 = [_20.fld0,_20.fld0];
_8 = [2077475068_u32,1315698311_u32,2903971227_u32,834203328_u32,1854192429_u32,1206616255_u32,2770291407_u32];
_10 = !_1;
_13 = [1758435683_u32,2398662517_u32,1950339373_u32,3990008303_u32,894074975_u32,2434019877_u32,1667252736_u32];
_17 = Adt35::Variant1 { fld0: _20.fld1 };
Goto(bb10)
}
bb10 = {
SetDiscriminant(_17, 0);
RET = core::ptr::addr_of!(_11.0.0.1);
_11.0.1.0 = core::ptr::addr_of!(_27);
_22 = _2;
_20.fld0 = 183143699452048332459062285324768080836_u128 << _10;
_23 = [_20.fld0,_20.fld0];
_11.0.0.1 = _11.0.1.1;
RET = core::ptr::addr_of!(_11.0.1.1);
_22 = _2;
_9 = [false,false,false,false,true,true];
_2 = _22;
place!(Field::<(usize, i128)>(Variant(_17, 0), 2)).1 = !(-45410263986988882648841781334900373057_i128);
_11.2 = &_11.0.1.0;
_20.fld1 = !7693536681150776804_i64;
_25 = Adt37::Variant2 { fld0: Field::<(usize, i128)>(Variant(_17, 0), 2).1,fld1: (*_28),fld2: _11.0.2 };
_11.0.0.1 = (-174646383_i32) as usize;
place!(Field::<u128>(Variant(_17, 0), 3)) = _20.fld0;
_24 = [112604085_i32,1155620038_i32,794921931_i32,(-824944156_i32),(-1325217404_i32),358696817_i32,182685427_i32,531016992_i32];
_3 = -_1;
(*_28) = Field::<char>(Variant(_25, 2), 1);
Call(_8 = fn14(), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_2 = Field::<char>(Variant(_25, 2), 1);
_11.0.1 = Move(_11.0.0);
_16 = _11.0.2 | Field::<u64>(Variant(_25, 2), 2);
place!(Field::<char>(Variant(_25, 2), 1)) = _22;
_11.0.0.0 = core::ptr::addr_of!(_27);
_4 = &_27;
_1 = !_3;
_3 = _10;
_37 = !_20.fld1;
_29 = _16 >> _16;
place!(Field::<(usize, i128)>(Variant(_17, 0), 2)).1 = _20.fld1 as i128;
SetDiscriminant(_25, 0);
_2 = (*_28);
_11.0.0.1 = 3639792086_u32 as usize;
place!(Field::<(Adt35, isize, [u128; 2])>(Variant(_25, 0), 3)).1 = (*_28) as isize;
_13 = [2630250535_u32,3790873429_u32,1755910292_u32,188851903_u32,647519045_u32,1354343249_u32,955367207_u32];
(*RET) = _29 as usize;
_35 = (*RET) as i64;
_40 = Field::<u128>(Variant(_17, 0), 3) as f32;
_28 = core::ptr::addr_of_mut!((*_28));
_12 = [_22,_2,_22,_2];
place!(Field::<Adt18>(Variant(_25, 0), 2)) = Adt18::Variant2 { fld0: _20.fld3 };
_3 = !_1;
place!(Field::<(Adt35, isize, [u128; 2])>(Variant(_25, 0), 3)).2 = [_20.fld0,Field::<u128>(Variant(_17, 0), 3)];
_8 = [800644604_u32,4127706089_u32,543520087_u32,476035683_u32,3303242086_u32,935345626_u32,2122311603_u32];
_1 = -_3;
_36 = !true;
Call(_11.0.0.1 = fn15(_9, _5, _10, Move(_11.0.1), _35, _40), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
place!(Field::<i128>(Variant(_25, 0), 5)) = -Field::<(usize, i128)>(Variant(_17, 0), 2).1;
place!(Field::<Adt18>(Variant(_25, 0), 2)) = Adt18::Variant2 { fld0: _14 };
(*RET) = _26.1 << _1;
_28 = core::ptr::addr_of_mut!(_2);
place!(Field::<u16>(Variant(place!(Field::<Adt18>(Variant(_25, 0), 2)), 2), 0)) = _20.fld3;
_24 = [1865519754_i32,465103619_i32,(-1473171925_i32),1836674499_i32,(-1979031246_i32),(-218812170_i32),(-265672943_i32),(-1633600596_i32)];
_11.0.0.1 = Field::<u128>(Variant(_17, 0), 3) as usize;
_25 = Adt37::Variant3 { fld0: 4043_i16 };
_7 = _29 as isize;
_20.fld3 = _2 as u16;
(*RET) = _11.3 | _11.3;
place!(Field::<u16>(Variant(_17, 0), 4)) = _20.fld0 as u16;
_20.fld1 = 438902760_i32 as i64;
Goto(bb13)
}
bb13 = {
_11.0.1 = Move(_11.0.0);
_42 = [1559062510_i32,(-984379926_i32),1177343265_i32,1028655567_i32,2125739866_i32,(-487848695_i32),(-1354665215_i32),680980854_i32];
_39 = [32644_i16,(-18311_i16),30692_i16];
_40 = (*RET) as f32;
place!(Field::<u16>(Variant(_17, 0), 4)) = _20.fld3 >> _10;
_26 = Move(_11.0.1);
place!(Field::<i16>(Variant(_25, 3), 0)) = -(-6410_i16);
_12 = [_22,(*_28),(*_28),_2];
_19 = Field::<u16>(Variant(_17, 0), 4) as f64;
place!(Field::<bool>(Variant(_17, 0), 0)) = !_36;
SetDiscriminant(_25, 0);
_13 = [629560646_u32,2102775934_u32,3026017397_u32,2496129469_u32,3039097683_u32,585346109_u32,3335803499_u32];
_31 = [_20.fld0,Field::<u128>(Variant(_17, 0), 3)];
place!(Field::<u16>(Variant(_17, 0), 4)) = _14;
_33.0 = _9;
(*RET) = _26.1 - _26.1;
place!(Field::<(usize, i128)>(Variant(_17, 0), 2)).1 = 126_i8 as i128;
_39 = [13856_i16,(-5800_i16),9254_i16];
RET = core::ptr::addr_of!(_11.3);
place!(Field::<(usize, i128)>(Variant(_17, 0), 2)).0 = _26.1 - _26.1;
_20.fld1 = _35 ^ _35;
Goto(bb14)
}
bb14 = {
_11.0.1 = Move(_26);
_39 = [9401_i16,21427_i16,(-24677_i16)];
_45.0.1.0 = core::ptr::addr_of!((*_4));
Goto(bb15)
}
bb15 = {
Call(_46 = dump_var(13_usize, 8_usize, Move(_8), 36_usize, Move(_36), 1_usize, Move(_1), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_46 = dump_var(13_usize, 29_usize, Move(_29), 9_usize, Move(_9), 21_usize, Move(_21), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(13_usize, 24_usize, Move(_24), 5_usize, Move(_5), 35_usize, Move(_35), 42_usize, Move(_42)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14() -> [u32; 7] {
mir! {
type RET = [u32; 7];
let _1: [i32; 6];
let _2: f64;
let _3: u128;
let _4: &'static &'static [u128; 5];
let _5: isize;
let _6: f32;
let _7: ([bool; 6], &'static u32);
let _8: ((*const Adt19, usize), (*const Adt19, usize), u64);
let _9: bool;
let _10: f32;
let _11: i64;
let _12: &'static u8;
let _13: Adt19;
let _14: bool;
let _15: (usize, i128);
let _16: [u128; 8];
let _17: &'static Adt19;
let _18: ();
let _19: ();
{
RET = [1375021003_u32,176889912_u32,4238225600_u32,4089651699_u32,1751816595_u32,1505863737_u32,763090306_u32];
RET = [2487351087_u32,691086592_u32,2661255343_u32,2520614620_u32,2777521983_u32,1741869201_u32,3713722060_u32];
RET = [600066879_u32,3802499782_u32,1152869538_u32,2563455357_u32,3079723724_u32,1649426375_u32,3570866693_u32];
RET = [3182853927_u32,1040448915_u32,2939176355_u32,871374995_u32,2899424912_u32,2657711807_u32,254273190_u32];
RET = [2281798697_u32,2044543139_u32,3082854847_u32,3175152616_u32,3615903903_u32,1691070120_u32,2117987847_u32];
RET = [795964503_u32,2438168983_u32,973946049_u32,388857231_u32,725061288_u32,530068896_u32,686202424_u32];
RET = [1040331856_u32,1848553823_u32,1783712188_u32,1015856769_u32,3942806549_u32,2746041109_u32,283432086_u32];
RET = [2382122878_u32,2638297361_u32,3633032133_u32,3418478133_u32,886089319_u32,3220159731_u32,433362332_u32];
RET = [2692514556_u32,2600922040_u32,2551798792_u32,1386872206_u32,646981171_u32,4097858186_u32,1675186083_u32];
RET = [1556160479_u32,4221507196_u32,2562105667_u32,1058610268_u32,3678639411_u32,750346195_u32,1384309781_u32];
_1 = [(-645893062_i32),1986859023_i32,(-1405929697_i32),(-589300192_i32),1601683840_i32,1771636018_i32];
RET = [150194791_u32,2905907033_u32,2147170743_u32,1852473300_u32,2818508157_u32,4248825744_u32,2761296042_u32];
RET = [639137489_u32,894643950_u32,590352482_u32,3762754200_u32,3228337552_u32,1153547564_u32,402701106_u32];
_2 = (-83300928294190456396950293134016856722_i128) as f64;
_1 = [800019_i32,(-2034275014_i32),423234566_i32,1183473705_i32,(-1516899393_i32),371378809_i32];
RET = [2449480475_u32,4163576954_u32,3711798217_u32,1221533834_u32,2186172196_u32,1270581690_u32,1484062401_u32];
_1 = [(-146889377_i32),(-861242933_i32),865385484_i32,(-1125790881_i32),1889198777_i32,1192161658_i32];
RET = [265108912_u32,1012212781_u32,3987503853_u32,968411557_u32,2931386930_u32,1733898077_u32,522250683_u32];
RET = [2694277004_u32,1175255174_u32,1562788383_u32,1172159405_u32,802907352_u32,3667360416_u32,4240444157_u32];
_1 = [(-1676217793_i32),(-150962864_i32),877713985_i32,1866443165_i32,(-1766125843_i32),292638971_i32];
_1 = [(-1530453861_i32),(-158805147_i32),(-1126485053_i32),(-968633987_i32),1037832874_i32,1555394587_i32];
_1 = [1729461260_i32,1283493457_i32,1794021482_i32,(-20468481_i32),2018739259_i32,345319558_i32];
Goto(bb1)
}
bb1 = {
_1 = [(-1314352987_i32),497810492_i32,(-1274028505_i32),2068344768_i32,(-1673069426_i32),762302043_i32];
_1 = [(-2019125075_i32),(-976126480_i32),(-1301137963_i32),(-668036424_i32),85520975_i32,(-1032466416_i32)];
RET = [1893472043_u32,4066526278_u32,885599678_u32,3441356207_u32,1810546736_u32,3777964167_u32,3549114536_u32];
_1 = [(-1628903495_i32),(-241986627_i32),(-766487205_i32),(-1498299502_i32),(-824318937_i32),960717054_i32];
RET = [3669384581_u32,3428591015_u32,2096184003_u32,3492972034_u32,84003217_u32,188376245_u32,517843536_u32];
_1 = [(-2115732674_i32),1199263351_i32,1934648326_i32,(-1142361922_i32),(-1476116151_i32),1860441224_i32];
_1 = [(-1263401833_i32),(-1363471599_i32),362120932_i32,577896872_i32,(-1662222018_i32),(-1165974527_i32)];
RET = [3977355632_u32,3247366122_u32,3744236928_u32,3307144377_u32,538943136_u32,1714166738_u32,1912227109_u32];
_2 = 6705_u16 as f64;
RET = [12934622_u32,4120125814_u32,2644318192_u32,416539302_u32,4157735268_u32,2416626066_u32,1286208027_u32];
RET = [2853661463_u32,489892305_u32,954749675_u32,2448861036_u32,2910873254_u32,1883181218_u32,2274994115_u32];
RET = [3483090867_u32,3413276932_u32,130656953_u32,2347628503_u32,1509321051_u32,1648698360_u32,4168105112_u32];
RET = [1258757410_u32,823826224_u32,3427688960_u32,1360453589_u32,670544997_u32,3638353478_u32,1090050852_u32];
_1 = [1077177541_i32,384891040_i32,(-127086068_i32),1495029646_i32,914531897_i32,(-1065564076_i32)];
_2 = (-5538509198974013038_i64) as f64;
_2 = 40702_u16 as f64;
RET = [1674061123_u32,1010702295_u32,711205522_u32,1761442068_u32,202283677_u32,1908837724_u32,4232780157_u32];
RET = [1031175171_u32,2524214381_u32,2174751236_u32,1944711756_u32,2110297046_u32,2097897804_u32,4200078789_u32];
_2 = 3688278945_u32 as f64;
RET = [3717006561_u32,1052908937_u32,137131515_u32,2990726107_u32,1671177160_u32,3728392209_u32,905622674_u32];
RET = [2478519291_u32,667781213_u32,2901794988_u32,3277347788_u32,1825404783_u32,3080669368_u32,1512958743_u32];
RET = [2116965696_u32,1379655212_u32,592510540_u32,995329145_u32,232708131_u32,1406903078_u32,3427487225_u32];
_1 = [495730059_i32,(-877638247_i32),1094133969_i32,668193367_i32,2058915781_i32,(-1223112087_i32)];
RET = [4119164050_u32,3641572406_u32,439537970_u32,4204197521_u32,1249504912_u32,3564914941_u32,1577031767_u32];
_2 = (-60859050117813211145644444281059360273_i128) as f64;
_1 = [1820328946_i32,(-1076075463_i32),(-372265148_i32),(-88957607_i32),1733443078_i32,1659258108_i32];
RET = [3324181454_u32,3954820147_u32,1009144004_u32,1994865721_u32,4268623030_u32,4030742507_u32,1928365380_u32];
Goto(bb2)
}
bb2 = {
RET = [3838916882_u32,2985201448_u32,180762384_u32,435319161_u32,2706707623_u32,2428388473_u32,2544599362_u32];
RET = [3428013188_u32,3793456832_u32,1476941562_u32,2444838098_u32,1688771663_u32,3376453019_u32,35339106_u32];
RET = [1759973822_u32,1296840701_u32,3583075662_u32,1930901229_u32,3992390997_u32,1993396946_u32,479184865_u32];
Goto(bb3)
}
bb3 = {
_1 = [1002469059_i32,1626682928_i32,(-174538033_i32),(-56270247_i32),(-479818876_i32),(-218296943_i32)];
_2 = 245899634580651643865052141380754090296_u128 as f64;
RET = [2353665576_u32,3351987023_u32,1951436028_u32,1356629174_u32,1064794144_u32,2940279554_u32,2151330199_u32];
RET = [3612155474_u32,3055521756_u32,507231056_u32,3894635075_u32,268597098_u32,3318307902_u32,41951629_u32];
_1 = [469518947_i32,(-159837447_i32),(-1017532332_i32),1873375502_i32,177363396_i32,(-927120590_i32)];
_1 = [(-367585757_i32),(-878293456_i32),1422545177_i32,(-355686828_i32),1551229808_i32,1089069085_i32];
_6 = (-56086057_i32) as f32;
_3 = 231855231423995828669459318228970146702_u128;
_5 = (-14_isize);
_2 = 66_i8 as f64;
_1 = [710961621_i32,678138133_i32,(-1712723237_i32),570728237_i32,197311448_i32,810232999_i32];
_3 = _2 as u128;
_1 = [(-1008113081_i32),1016331160_i32,942908363_i32,(-1861993559_i32),(-232976601_i32),(-317361236_i32)];
_1 = [(-1532905900_i32),(-1863755431_i32),1094988180_i32,319867378_i32,1772469814_i32,(-439001259_i32)];
_7.0 = [true,true,false,true,true,true];
match _5 {
0 => bb1,
340282366920938463463374607431768211442 => bb4,
_ => bb2
}
}
bb4 = {
RET = [2247033956_u32,164099662_u32,744740625_u32,2822015689_u32,1282630385_u32,2910138134_u32,1662484210_u32];
match _5 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
340282366920938463463374607431768211442 => bb12,
_ => bb11
}
}
bb5 = {
_1 = [1002469059_i32,1626682928_i32,(-174538033_i32),(-56270247_i32),(-479818876_i32),(-218296943_i32)];
_2 = 245899634580651643865052141380754090296_u128 as f64;
RET = [2353665576_u32,3351987023_u32,1951436028_u32,1356629174_u32,1064794144_u32,2940279554_u32,2151330199_u32];
RET = [3612155474_u32,3055521756_u32,507231056_u32,3894635075_u32,268597098_u32,3318307902_u32,41951629_u32];
_1 = [469518947_i32,(-159837447_i32),(-1017532332_i32),1873375502_i32,177363396_i32,(-927120590_i32)];
_1 = [(-367585757_i32),(-878293456_i32),1422545177_i32,(-355686828_i32),1551229808_i32,1089069085_i32];
_6 = (-56086057_i32) as f32;
_3 = 231855231423995828669459318228970146702_u128;
_5 = (-14_isize);
_2 = 66_i8 as f64;
_1 = [710961621_i32,678138133_i32,(-1712723237_i32),570728237_i32,197311448_i32,810232999_i32];
_3 = _2 as u128;
_1 = [(-1008113081_i32),1016331160_i32,942908363_i32,(-1861993559_i32),(-232976601_i32),(-317361236_i32)];
_1 = [(-1532905900_i32),(-1863755431_i32),1094988180_i32,319867378_i32,1772469814_i32,(-439001259_i32)];
_7.0 = [true,true,false,true,true,true];
match _5 {
0 => bb1,
340282366920938463463374607431768211442 => bb4,
_ => bb2
}
}
bb6 = {
RET = [3838916882_u32,2985201448_u32,180762384_u32,435319161_u32,2706707623_u32,2428388473_u32,2544599362_u32];
RET = [3428013188_u32,3793456832_u32,1476941562_u32,2444838098_u32,1688771663_u32,3376453019_u32,35339106_u32];
RET = [1759973822_u32,1296840701_u32,3583075662_u32,1930901229_u32,3992390997_u32,1993396946_u32,479184865_u32];
Goto(bb3)
}
bb7 = {
_1 = [(-1314352987_i32),497810492_i32,(-1274028505_i32),2068344768_i32,(-1673069426_i32),762302043_i32];
_1 = [(-2019125075_i32),(-976126480_i32),(-1301137963_i32),(-668036424_i32),85520975_i32,(-1032466416_i32)];
RET = [1893472043_u32,4066526278_u32,885599678_u32,3441356207_u32,1810546736_u32,3777964167_u32,3549114536_u32];
_1 = [(-1628903495_i32),(-241986627_i32),(-766487205_i32),(-1498299502_i32),(-824318937_i32),960717054_i32];
RET = [3669384581_u32,3428591015_u32,2096184003_u32,3492972034_u32,84003217_u32,188376245_u32,517843536_u32];
_1 = [(-2115732674_i32),1199263351_i32,1934648326_i32,(-1142361922_i32),(-1476116151_i32),1860441224_i32];
_1 = [(-1263401833_i32),(-1363471599_i32),362120932_i32,577896872_i32,(-1662222018_i32),(-1165974527_i32)];
RET = [3977355632_u32,3247366122_u32,3744236928_u32,3307144377_u32,538943136_u32,1714166738_u32,1912227109_u32];
_2 = 6705_u16 as f64;
RET = [12934622_u32,4120125814_u32,2644318192_u32,416539302_u32,4157735268_u32,2416626066_u32,1286208027_u32];
RET = [2853661463_u32,489892305_u32,954749675_u32,2448861036_u32,2910873254_u32,1883181218_u32,2274994115_u32];
RET = [3483090867_u32,3413276932_u32,130656953_u32,2347628503_u32,1509321051_u32,1648698360_u32,4168105112_u32];
RET = [1258757410_u32,823826224_u32,3427688960_u32,1360453589_u32,670544997_u32,3638353478_u32,1090050852_u32];
_1 = [1077177541_i32,384891040_i32,(-127086068_i32),1495029646_i32,914531897_i32,(-1065564076_i32)];
_2 = (-5538509198974013038_i64) as f64;
_2 = 40702_u16 as f64;
RET = [1674061123_u32,1010702295_u32,711205522_u32,1761442068_u32,202283677_u32,1908837724_u32,4232780157_u32];
RET = [1031175171_u32,2524214381_u32,2174751236_u32,1944711756_u32,2110297046_u32,2097897804_u32,4200078789_u32];
_2 = 3688278945_u32 as f64;
RET = [3717006561_u32,1052908937_u32,137131515_u32,2990726107_u32,1671177160_u32,3728392209_u32,905622674_u32];
RET = [2478519291_u32,667781213_u32,2901794988_u32,3277347788_u32,1825404783_u32,3080669368_u32,1512958743_u32];
RET = [2116965696_u32,1379655212_u32,592510540_u32,995329145_u32,232708131_u32,1406903078_u32,3427487225_u32];
_1 = [495730059_i32,(-877638247_i32),1094133969_i32,668193367_i32,2058915781_i32,(-1223112087_i32)];
RET = [4119164050_u32,3641572406_u32,439537970_u32,4204197521_u32,1249504912_u32,3564914941_u32,1577031767_u32];
_2 = (-60859050117813211145644444281059360273_i128) as f64;
_1 = [1820328946_i32,(-1076075463_i32),(-372265148_i32),(-88957607_i32),1733443078_i32,1659258108_i32];
RET = [3324181454_u32,3954820147_u32,1009144004_u32,1994865721_u32,4268623030_u32,4030742507_u32,1928365380_u32];
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
_6 = _2 as f32;
_2 = 97_u8 as f64;
_8.0.1 = !9992422201854983498_usize;
RET = [3524092431_u32,183248654_u32,2057590293_u32,1443585473_u32,2330811260_u32,1712526310_u32,805757875_u32];
_1 = [(-1496804333_i32),1028778144_i32,(-2119634463_i32),(-776877120_i32),2006706854_i32,(-1937779835_i32)];
_7.0 = [true,true,true,false,false,true];
_6 = 12446715005356091725_u64 as f32;
_10 = _6 - _6;
_9 = true;
RET = [2904179854_u32,2961169548_u32,2796177467_u32,332111883_u32,1076551289_u32,27872979_u32,1870044838_u32];
_9 = true ^ false;
_9 = true;
_8.0.1 = !1_usize;
match _5 {
0 => bb10,
1 => bb2,
2 => bb13,
3 => bb14,
340282366920938463463374607431768211442 => bb16,
_ => bb15
}
}
bb13 = {
_1 = [1002469059_i32,1626682928_i32,(-174538033_i32),(-56270247_i32),(-479818876_i32),(-218296943_i32)];
_2 = 245899634580651643865052141380754090296_u128 as f64;
RET = [2353665576_u32,3351987023_u32,1951436028_u32,1356629174_u32,1064794144_u32,2940279554_u32,2151330199_u32];
RET = [3612155474_u32,3055521756_u32,507231056_u32,3894635075_u32,268597098_u32,3318307902_u32,41951629_u32];
_1 = [469518947_i32,(-159837447_i32),(-1017532332_i32),1873375502_i32,177363396_i32,(-927120590_i32)];
_1 = [(-367585757_i32),(-878293456_i32),1422545177_i32,(-355686828_i32),1551229808_i32,1089069085_i32];
_6 = (-56086057_i32) as f32;
_3 = 231855231423995828669459318228970146702_u128;
_5 = (-14_isize);
_2 = 66_i8 as f64;
_1 = [710961621_i32,678138133_i32,(-1712723237_i32),570728237_i32,197311448_i32,810232999_i32];
_3 = _2 as u128;
_1 = [(-1008113081_i32),1016331160_i32,942908363_i32,(-1861993559_i32),(-232976601_i32),(-317361236_i32)];
_1 = [(-1532905900_i32),(-1863755431_i32),1094988180_i32,319867378_i32,1772469814_i32,(-439001259_i32)];
_7.0 = [true,true,false,true,true,true];
match _5 {
0 => bb1,
340282366920938463463374607431768211442 => bb4,
_ => bb2
}
}
bb14 = {
RET = [3838916882_u32,2985201448_u32,180762384_u32,435319161_u32,2706707623_u32,2428388473_u32,2544599362_u32];
RET = [3428013188_u32,3793456832_u32,1476941562_u32,2444838098_u32,1688771663_u32,3376453019_u32,35339106_u32];
RET = [1759973822_u32,1296840701_u32,3583075662_u32,1930901229_u32,3992390997_u32,1993396946_u32,479184865_u32];
Goto(bb3)
}
bb15 = {
RET = [3838916882_u32,2985201448_u32,180762384_u32,435319161_u32,2706707623_u32,2428388473_u32,2544599362_u32];
RET = [3428013188_u32,3793456832_u32,1476941562_u32,2444838098_u32,1688771663_u32,3376453019_u32,35339106_u32];
RET = [1759973822_u32,1296840701_u32,3583075662_u32,1930901229_u32,3992390997_u32,1993396946_u32,479184865_u32];
Goto(bb3)
}
bb16 = {
_10 = -_6;
_8.2 = 1349696426241035795_u64;
_7.0 = [_9,_9,_9,_9,_9,_9];
_8.2 = !8855554861868270053_u64;
RET = [3707325947_u32,4249540283_u32,2334467278_u32,3533069727_u32,2182432663_u32,2328583699_u32,2734412466_u32];
RET = [2750380360_u32,2165825647_u32,430966629_u32,2896262820_u32,4018779580_u32,2287023402_u32,3199516760_u32];
RET = [2995862517_u32,691555027_u32,2529982483_u32,314756873_u32,3918958665_u32,4185809817_u32,2958743043_u32];
_8.2 = 3975704090223862707_u64;
_8.1.0 = core::ptr::addr_of!(_13);
_11 = (-2186870572124053767_i64) | (-5066803880905030143_i64);
_14 = _9;
_14 = !_9;
_15.0 = !_8.0.1;
_8.0.0 = core::ptr::addr_of!(_13);
_6 = _10;
_16 = [_3,_3,_3,_3,_3,_3,_3,_3];
_13 = Adt19::Variant0 { fld0: _10,fld1: '\u{7bfb}',fld2: _2,fld3: _3,fld4: _15.0,fld5: 1052675719_i32,fld6: _11,fld7: 140944176003275330123709367737218559946_i128 };
place!(Field::<i128>(Variant(_13, 0), 7)) = (-162631212133018449476917978656686442523_i128);
_15 = (Field::<usize>(Variant(_13, 0), 4), Field::<i128>(Variant(_13, 0), 7));
_17 = &_13;
Goto(bb17)
}
bb17 = {
Call(_18 = dump_var(14_usize, 14_usize, Move(_14), 15_usize, Move(_15), 3_usize, Move(_3), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: [bool; 6],mut _2: [i32; 7],mut _3: isize,mut _4: (*const Adt19, usize),mut _5: i64,mut _6: f32) -> usize {
mir! {
type RET = usize;
let _7: Adt18;
let _8: f32;
let _9: (*const usize, *const Adt19, (i128, &'static Adt19, Adt19, u32), &'static i128);
let _10: f64;
let _11: u32;
let _12: Adt78;
let _13: isize;
let _14: &'static u32;
let _15: [bool; 6];
let _16: &'static Adt19;
let _17: (&'static u128, Adt32);
let _18: bool;
let _19: isize;
let _20: i32;
let _21: (((*const Adt19, usize), (*const Adt19, usize), u64), Adt32, &'static *const Adt19, usize);
let _22: isize;
let _23: [u128; 7];
let _24: char;
let _25: [char; 4];
let _26: ();
let _27: ();
{
RET = !_4.1;
RET = !_4.1;
_5 = (-7688202183255423947_i64);
_1 = [true,false,true,true,false,false];
RET = 1979178582_i32 as usize;
_3 = 94_i8 as isize;
_5 = (-911143681761019865_i64) & 7991231169418152105_i64;
_3 = 43_isize - (-124_isize);
_3 = (-9223372036854775808_isize);
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
340282366920938463454151235394913435648 => bb7,
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
_6 = (-121137109960834206260503247174064857610_i128) as f32;
_6 = (-99349828865430715541219926933376705024_i128) as f32;
_4.1 = RET;
_4.1 = RET | RET;
_4.1 = !RET;
_2 = [471948857_i32,952252771_i32,(-1317179540_i32),(-1425401035_i32),(-2113630900_i32),(-188720659_i32),128006648_i32];
_2 = [(-1258221462_i32),(-117786341_i32),1382925052_i32,(-614378973_i32),(-1580930596_i32),(-434174045_i32),(-1119994854_i32)];
_3 = 12216616741264460882_u64 as isize;
RET = 2257522286404651436_u64 as usize;
_7 = Adt18::Variant2 { fld0: 26937_u16 };
place!(Field::<u16>(Variant(_7, 2), 0)) = 36027_u16;
_9.0 = core::ptr::addr_of!(_4.1);
_2 = [474816031_i32,(-705938195_i32),(-94498570_i32),1558420377_i32,(-1435675089_i32),(-1284557999_i32),(-760614813_i32)];
RET = _4.1;
_9.3 = &_9.2.0;
RET = _4.1;
RET = _4.1;
_9.3 = &_9.2.0;
_3 = !(-9223372036854775808_isize);
_9.2.1 = &_9.2.2;
match Field::<u16>(Variant(_7, 2), 0) {
0 => bb6,
1 => bb2,
36027 => bb8,
_ => bb4
}
}
bb8 = {
_9.3 = &_9.2.0;
_9.2.0 = 159276604887819808488406508690603608329_i128 | 126707411143394885885807973372457398974_i128;
_11 = 2298035715_u32;
_7 = Adt18::Variant1 { fld0: 413896913_i32,fld1: _11 };
_4.1 = RET;
_4.1 = !RET;
_7 = Adt18::Variant0 { fld0: false,fld1: RET,fld2: _3,fld3: 274045908862124765413052769406070576679_u128 };
_9.3 = &_9.2.0;
_9.1 = core::ptr::addr_of!(_9.2.2);
_9.1 = core::ptr::addr_of!(_9.2.2);
place!(Field::<usize>(Variant(_7, 0), 1)) = !_4.1;
_3 = Field::<isize>(Variant(_7, 0), 2) + Field::<isize>(Variant(_7, 0), 2);
_13 = _3;
_1 = [true,true,false,false,false,false];
_1 = [true,false,false,true,false,false];
_1 = [true,true,false,false,true,false];
_11 = 2328930612_u32;
_7 = Adt18::Variant2 { fld0: 63811_u16 };
_3 = _13;
RET = (-95_i8) as usize;
Goto(bb9)
}
bb9 = {
place!(Field::<u16>(Variant(_7, 2), 0)) = 55031_u16;
_9.0 = core::ptr::addr_of!(_4.1);
_14 = &_9.2.3;
_9.0 = core::ptr::addr_of!(_4.1);
_10 = 9008228669585031975_u64 as f64;
RET = (-12861_i16) as usize;
_4.0 = core::ptr::addr_of!(_9.2.2);
_10 = Field::<u16>(Variant(_7, 2), 0) as f64;
_9.0 = core::ptr::addr_of!(RET);
Call(_12 = fn16(Move(_9.3), Move(_7), _2, Move(_4), _2, _13, _5), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_9.3 = &_9.2.0;
_4.1 = !RET;
RET = _4.1 & _4.1;
Goto(bb11)
}
bb11 = {
_9.2.3 = _11;
_14 = &_9.2.3;
_3 = !_13;
_4.0 = core::ptr::addr_of!(_9.2.2);
_9.2.3 = 78175722847775439258312509125391850644_u128 as u32;
_14 = &_9.2.3;
_7 = Adt18::Variant2 { fld0: 7813_u16 };
place!(Field::<bool>(Variant(_12, 1), 0)) = true;
_9.3 = &_9.2.0;
RET = _4.1 + _4.1;
_15 = [Field::<bool>(Variant(_12, 1), 0),Field::<bool>(Variant(_12, 1), 0),Field::<bool>(Variant(_12, 1), 0),Field::<bool>(Variant(_12, 1), 0),Field::<bool>(Variant(_12, 1), 0),Field::<bool>(Variant(_12, 1), 0)];
_9.3 = &_9.2.0;
_9.2.1 = &_9.2.2;
place!(Field::<u16>(Variant(_7, 2), 0)) = 10573_u16;
_11 = _9.2.3 >> _13;
_9.3 = &_9.2.0;
RET = _4.1 * _4.1;
_5 = (-592412183406413245_i64);
_9.2.1 = &_9.2.2;
_8 = -_6;
_8 = _6 + _6;
_4.0 = core::ptr::addr_of!(_9.2.2);
_1 = _15;
_11 = (*_14);
Goto(bb12)
}
bb12 = {
_9.2.1 = &_9.2.2;
_5 = 6937345138414013908_i64;
_9.2.2 = Adt19::Variant1 { fld0: Field::<bool>(Variant(_12, 1), 0),fld1: 15_u8,fld2: _4.1,fld3: 24_i8,fld4: Move(_7),fld5: (-787178297_i32),fld6: _5 };
_9.3 = &_9.2.0;
place!(Field::<bool>(Variant(_9.2.2, 1), 0)) = !Field::<bool>(Variant(_12, 1), 0);
place!(Field::<i64>(Variant(_9.2.2, 1), 6)) = _5 * _5;
SetDiscriminant(Field::<Adt18>(Variant(_9.2.2, 1), 4), 3);
_13 = !_3;
_9.1 = core::ptr::addr_of!(_9.2.2);
_9.3 = &_9.2.0;
place!(Field::<u64>(Variant(place!(Field::<Adt18>(Variant(_9.2.2, 1), 4)), 3), 0)) = 1413167714323690405_u64 & 5285224172477534927_u64;
place!(Field::<u32>(Variant(place!(Field::<Adt18>(Variant(_9.2.2, 1), 4)), 3), 7)) = _5 as u32;
place!(Field::<Adt18>(Variant(_9.2.2, 1), 4)) = Adt18::Variant2 { fld0: 10911_u16 };
_21.3 = Field::<usize>(Variant(_9.2.2, 1), 2);
place!(Field::<i32>(Variant(_9.2.2, 1), 5)) = (-270666414_i32) | 856408006_i32;
place!(Field::<i64>(Variant(_9.2.2, 1), 6)) = _5;
_21.0 = (Move(_4), Move(_4), 17504522938184906189_u64);
_21.0.0.1 = (-54_i8) as usize;
_16 = &_9.2.2;
match Field::<i64>(Variant((*_16), 1), 6) {
0 => bb9,
1 => bb10,
2 => bb3,
6937345138414013908 => bb13,
_ => bb11
}
}
bb13 = {
_4 = Move(_21.0.1);
_21.2 = &_9.1;
place!(Field::<Adt18>(Variant(_9.2.2, 1), 4)) = Adt18::Variant3 { fld0: _21.0.2,fld1: '\u{67299}',fld2: _21.0.0.1,fld3: _10,fld4: (-12442_i16),fld5: Field::<i32>(Variant(_9.2.2, 1), 5),fld6: 8315_u16,fld7: (*_14) };
place!(Field::<bool>(Variant(_9.2.2, 1), 0)) = _9.2.0 < _9.2.0;
_2 = [Field::<i32>(Variant(_9.2.2, 1), 5),Field::<i32>(Variant(_9.2.2, 1), 5),Field::<i32>(Variant(_9.2.2, 1), 5),Field::<i32>(Variant(Field::<Adt18>(Variant(_9.2.2, 1), 4), 3), 5),Field::<i32>(Variant(_9.2.2, 1), 5),Field::<i32>(Variant(_9.2.2, 1), 5),Field::<i32>(Variant(Field::<Adt18>(Variant(_9.2.2, 1), 4), 3), 5)];
_21.0.1.1 = _21.0.0.1;
_21.2 = &_4.0;
place!(Field::<u64>(Variant(place!(Field::<Adt18>(Variant(_9.2.2, 1), 4)), 3), 0)) = _21.0.2;
place!(Field::<bool>(Variant(_12, 1), 0)) = Field::<bool>(Variant(_9.2.2, 1), 0) < Field::<bool>(Variant(_9.2.2, 1), 0);
_18 = Field::<bool>(Variant(_12, 1), 0);
_21.2 = &_21.0.0.0;
_21.2 = &_21.0.1.0;
_4 = (Move(_9.1), RET);
_4.1 = Field::<usize>(Variant((*_16), 1), 2);
_2 = [Field::<i32>(Variant(Field::<Adt18>(Variant(_9.2.2, 1), 4), 3), 5),Field::<i32>(Variant(_9.2.2, 1), 5),Field::<i32>(Variant((*_16), 1), 5),Field::<i32>(Variant((*_16), 1), 5),Field::<i32>(Variant((*_16), 1), 5),Field::<i32>(Variant(Field::<Adt18>(Variant(_9.2.2, 1), 4), 3), 5),Field::<i32>(Variant((*_16), 1), 5)];
RET = _21.0.1.1;
match Field::<i64>(Variant((*_16), 1), 6) {
0 => bb9,
6937345138414013908 => bb14,
_ => bb2
}
}
bb14 = {
_21.2 = &_9.1;
place!(Field::<Adt18>(Variant(_9.2.2, 1), 4)) = Adt18::Variant0 { fld0: _18,fld1: _21.0.1.1,fld2: _3,fld3: 148476700943692370334055860854120116068_u128 };
place!(Field::<bool>(Variant(_9.2.2, 1), 0)) = Field::<bool>(Variant(_12, 1), 0);
_20 = 45_i8 as i32;
place!(Field::<bool>(Variant(_9.2.2, 1), 0)) = Field::<bool>(Variant(_12, 1), 0) | Field::<bool>(Variant(Field::<Adt18>(Variant(_9.2.2, 1), 4), 0), 0);
_21.0.1 = (Move(_4.0), RET);
place!(Field::<bool>(Variant(_9.2.2, 1), 0)) = Field::<bool>(Variant(_12, 1), 0);
_22 = _13 & _13;
_23 = [148533663274745149449272263879680896651_u128,76376125026316563957190890550621048762_u128,103015994195037366429406004843874118663_u128,98544866344977703161380668136512927971_u128,146029428977898050167905838852454853257_u128,269281244796269117068394215571253852624_u128,255579913898546244195316284413074758556_u128];
place!(Field::<i32>(Variant(_9.2.2, 1), 5)) = '\u{e00d0}' as i32;
_9.2.0 = (-60_i8) as i128;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(15_usize, 13_usize, Move(_13), 22_usize, Move(_22), 23_usize, Move(_23), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(15_usize, 3_usize, Move(_3), 27_usize, _27, 27_usize, _27, 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: &'static i128,mut _2: Adt18,mut _3: [i32; 7],mut _4: (*const Adt19, usize),mut _5: [i32; 7],mut _6: isize,mut _7: i64) -> Adt78 {
mir! {
type RET = Adt78;
let _8: u128;
let _9: *const Adt19;
let _10: u8;
let _11: *mut u128;
let _12: isize;
let _13: u128;
let _14: [bool; 1];
let _15: char;
let _16: [char; 4];
let _17: *mut Adt35;
let _18: u64;
let _19: [i32; 7];
let _20: [u16; 1];
let _21: bool;
let _22: isize;
let _23: (&'static u128, Adt32);
let _24: isize;
let _25: isize;
let _26: char;
let _27: i128;
let _28: ();
let _29: ();
{
_6 = 9223372036854775807_isize;
_2 = Adt18::Variant2 { fld0: 31692_u16 };
_4.1 = 50_i8 as usize;
Call(_10 = fn17(Move(_4), _5, _3, _3, _6, _5, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<u16>(Variant(_2, 2), 0)) = !28141_u16;
_10 = 216_u8 - 133_u8;
_10 = !123_u8;
match _6 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
9223372036854775807 => bb9,
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
_11 = core::ptr::addr_of_mut!(_8);
_8 = (-10491_i16) as u128;
_4.1 = 15585740458142391476_usize ^ 5_usize;
(*_11) = 254732210422978264924049528517619190206_u128 & 320866096430663665153339125288575929702_u128;
_6 = !9223372036854775807_isize;
(*_11) = 216863580555220590925225964395402089991_u128;
(*_11) = 89453390967704165158925380527207931067_u128 - 256592825823741581475860103333949282726_u128;
Goto(bb10)
}
bb10 = {
place!(Field::<u16>(Variant(_2, 2), 0)) = !53684_u16;
_12 = _6 << _6;
_8 = 164296057489209794868941433020962578987_u128;
_3 = [(-1072661956_i32),(-2028590955_i32),1748316151_i32,360955459_i32,795541067_i32,(-2115531523_i32),(-1091154273_i32)];
_12 = _4.1 as isize;
_14 = [false];
(*_11) = 69326230689264477718209145959606204793_u128;
place!(Field::<u16>(Variant(_2, 2), 0)) = !42473_u16;
_13 = _8 >> _7;
_12 = (-746329514_i32) as isize;
_11 = core::ptr::addr_of_mut!((*_11));
Goto(bb11)
}
bb11 = {
SetDiscriminant(_2, 1);
(*_11) = _13 ^ _13;
place!(Field::<u32>(Variant(_2, 1), 1)) = 2905849623_u32 - 1154046943_u32;
_8 = !_13;
(*_11) = _13;
_15 = '\u{c269d}';
_8 = (-1342237865_i32) as u128;
Goto(bb12)
}
bb12 = {
_3 = [(-661579556_i32),404013151_i32,11591691_i32,(-1979102600_i32),(-2111595557_i32),(-125699406_i32),(-2066005511_i32)];
place!(Field::<u32>(Variant(_2, 1), 1)) = 2606553493_u32;
_3 = _5;
_13 = (*_11) ^ (*_11);
_2 = Adt18::Variant2 { fld0: 39045_u16 };
_12 = -_6;
_3 = _5;
place!(Field::<u16>(Variant(_2, 2), 0)) = 6328_u16 >> _10;
_16 = [_15,_15,_15,_15];
(*_11) = _13;
_8 = _13;
_7 = 808216353752715752_i64 + (-9131456343512680373_i64);
SetDiscriminant(_2, 2);
_19 = [1372366875_i32,(-1828699478_i32),(-201660824_i32),1020727501_i32,(-558349418_i32),460640394_i32,99050742_i32];
_2 = Adt18::Variant0 { fld0: false,fld1: _4.1,fld2: _6,fld3: _13 };
(*_11) = Field::<u128>(Variant(_2, 0), 3) & Field::<u128>(Variant(_2, 0), 3);
Goto(bb13)
}
bb13 = {
_6 = 1054629453_i32 as isize;
_20 = [26878_u16];
_18 = 16298137288258093703_u64;
_4.1 = !Field::<usize>(Variant(_2, 0), 1);
_14 = [false];
_21 = true & false;
_3 = _5;
_10 = 3825012211_u32 as u8;
_3 = [(-812820355_i32),961750865_i32,1630788504_i32,390524576_i32,(-1240419823_i32),(-2020566670_i32),1288813726_i32];
_2 = Adt18::Variant1 { fld0: (-2054274606_i32),fld1: 1423936269_u32 };
_12 = _6;
_19 = [(-433162159_i32),(-1493627457_i32),1373369091_i32,(-703753903_i32),1639484337_i32,(-658538483_i32),(-1346115329_i32)];
(*_11) = _13 >> _7;
_4.1 = _18 as usize;
place!(Field::<i32>(Variant(_2, 1), 0)) = 941963065_i32;
_4.1 = _8 as usize;
RET = Adt78::Variant1 { fld0: _21,fld1: _20 };
_19 = [Field::<i32>(Variant(_2, 1), 0),Field::<i32>(Variant(_2, 1), 0),Field::<i32>(Variant(_2, 1), 0),Field::<i32>(Variant(_2, 1), 0),Field::<i32>(Variant(_2, 1), 0),Field::<i32>(Variant(_2, 1), 0),Field::<i32>(Variant(_2, 1), 0)];
_22 = 7764_u16 as isize;
_18 = 4004523729114262309_u64;
_23.0 = &_13;
(*_11) = _13;
_23.0 = &_13;
place!(Field::<u32>(Variant(_2, 1), 1)) = !1420828726_u32;
_11 = core::ptr::addr_of_mut!((*_11));
_15 = '\u{d9b91}';
match _18 {
0 => bb6,
1 => bb12,
2 => bb14,
3 => bb15,
4004523729114262309 => bb17,
_ => bb16
}
}
bb14 = {
_11 = core::ptr::addr_of_mut!(_8);
_8 = (-10491_i16) as u128;
_4.1 = 15585740458142391476_usize ^ 5_usize;
(*_11) = 254732210422978264924049528517619190206_u128 & 320866096430663665153339125288575929702_u128;
_6 = !9223372036854775807_isize;
(*_11) = 216863580555220590925225964395402089991_u128;
(*_11) = 89453390967704165158925380527207931067_u128 - 256592825823741581475860103333949282726_u128;
Goto(bb10)
}
bb15 = {
Return()
}
bb16 = {
place!(Field::<u16>(Variant(_2, 2), 0)) = !53684_u16;
_12 = _6 << _6;
_8 = 164296057489209794868941433020962578987_u128;
_3 = [(-1072661956_i32),(-2028590955_i32),1748316151_i32,360955459_i32,795541067_i32,(-2115531523_i32),(-1091154273_i32)];
_12 = _4.1 as isize;
_14 = [false];
(*_11) = 69326230689264477718209145959606204793_u128;
place!(Field::<u16>(Variant(_2, 2), 0)) = !42473_u16;
_13 = _8 >> _7;
_12 = (-746329514_i32) as isize;
_11 = core::ptr::addr_of_mut!((*_11));
Goto(bb11)
}
bb17 = {
_23.0 = &(*_11);
_24 = !_6;
_26 = _15;
SetDiscriminant(_2, 0);
place!(Field::<u128>(Variant(_2, 0), 3)) = (*_11);
place!(Field::<usize>(Variant(_2, 0), 1)) = _18 as usize;
_25 = _6;
_2 = Adt18::Variant2 { fld0: 45369_u16 };
_8 = _13 >> _4.1;
_27 = !(-113944819472728539282459892013686076638_i128);
_11 = core::ptr::addr_of_mut!(_8);
_27 = (*_11) as i128;
Goto(bb18)
}
bb18 = {
Call(_28 = dump_var(16_usize, 8_usize, Move(_8), 25_usize, Move(_25), 20_usize, Move(_20), 22_usize, Move(_22)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_28 = dump_var(16_usize, 21_usize, Move(_21), 14_usize, Move(_14), 19_usize, Move(_19), 18_usize, Move(_18)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_28 = dump_var(16_usize, 3_usize, Move(_3), 12_usize, Move(_12), 29_usize, _29, 29_usize, _29), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: (*const Adt19, usize),mut _2: [i32; 7],mut _3: [i32; 7],mut _4: [i32; 7],mut _5: isize,mut _6: [i32; 7],mut _7: [i32; 7]) -> u8 {
mir! {
type RET = u8;
let _8: [u8; 3];
let _9: bool;
let _10: [u16; 1];
let _11: Adt29;
let _12: Adt35;
let _13: *const &'static i8;
let _14: [i32; 7];
let _15: u128;
let _16: f32;
let _17: char;
let _18: char;
let _19: f64;
let _20: isize;
let _21: &'static [u128; 5];
let _22: Adt18;
let _23: f32;
let _24: ();
let _25: ();
{
RET = !64_u8;
_6 = [2064234438_i32,(-814809633_i32),1648934584_i32,(-1672144115_i32),478862165_i32,(-131573302_i32),192639071_i32];
RET = _1.1 as u8;
_4 = _6;
_4 = [(-1336422090_i32),256638816_i32,804472151_i32,(-383547080_i32),1291469171_i32,(-873034766_i32),1153729827_i32];
_2 = [2016532586_i32,36588551_i32,(-443151256_i32),1826479905_i32,(-614628474_i32),1968512833_i32,764725631_i32];
_8 = [RET,RET,RET];
_7 = [1795604170_i32,(-1143966738_i32),(-1286334380_i32),1115534714_i32,2064190323_i32,(-621700887_i32),385081151_i32];
_3 = [1190368514_i32,1562250695_i32,(-1336908219_i32),1776629654_i32,1724849911_i32,(-1132884391_i32),518040877_i32];
RET = 19767_i16 as u8;
_2 = _4;
_9 = !true;
_9 = !true;
_1.1 = 373719392509837117_i64 as usize;
RET = !230_u8;
_9 = false;
RET = 126_u8 << _5;
_6 = [(-809963663_i32),998122204_i32,(-1449148422_i32),1169758053_i32,195731591_i32,(-1963357578_i32),1848310959_i32];
_8 = [RET,RET,RET];
RET = (-128773707067310728895372130940072893702_i128) as u8;
RET = 136_u8;
_2 = [46629188_i32,(-589158098_i32),1329627246_i32,(-1416032865_i32),(-2057812084_i32),851956350_i32,(-454386195_i32)];
_2 = [(-251879532_i32),946188015_i32,367002061_i32,206439077_i32,1041443868_i32,(-1997111790_i32),(-1198482871_i32)];
RET = 70753782627715359048033999906531450511_i128 as u8;
_1.1 = (-1942409986_i32) as usize;
_9 = true;
_8 = [RET,RET,RET];
RET = 225_u8;
Goto(bb1)
}
bb1 = {
_10 = [53503_u16];
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
225 => bb8,
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
_11.fld3 = (-1520339050_i32) as u16;
_3 = [(-1262607642_i32),1573927956_i32,(-1836615491_i32),125621169_i32,2104175498_i32,(-2103752830_i32),(-709289644_i32)];
_6 = _3;
_5 = 9223372036854775807_isize;
_5 = 27083_i16 as isize;
_2 = _3;
_11.fld0 = !235773409580318670018789100900015593023_u128;
_10 = [_11.fld3];
_11.fld3 = !58237_u16;
_11.fld1 = 7573_i16 as i64;
_7 = [861697782_i32,(-103750595_i32),(-1080918221_i32),(-561398609_i32),(-1101257390_i32),(-1344647946_i32),(-1613418993_i32)];
RET = 1062399790_i32 as u8;
_2 = [(-1852840435_i32),1583342409_i32,(-57141109_i32),2145856932_i32,494035329_i32,1699326806_i32,9151823_i32];
_4 = [402491246_i32,(-1871672138_i32),(-2067943310_i32),264693597_i32,(-2125521572_i32),2104774059_i32,413951319_i32];
_3 = [(-192405115_i32),1365330698_i32,317195832_i32,(-2092605122_i32),(-1527713297_i32),691870488_i32,(-2137773471_i32)];
_9 = !true;
_6 = [1816738715_i32,1731175505_i32,1090904883_i32,(-1735575652_i32),(-1236321813_i32),148538690_i32,2028036782_i32];
_11.fld1 = 1567544879_i32 as i64;
_10 = [_11.fld3];
_2 = [578408111_i32,1286664550_i32,1051371058_i32,(-528695007_i32),(-933560565_i32),580914455_i32,1658345348_i32];
_10 = [_11.fld3];
_10 = [_11.fld3];
_10 = [_11.fld3];
_7 = _6;
_6 = [(-1171624551_i32),(-710252758_i32),153576913_i32,1554776431_i32,(-1370142090_i32),1221465071_i32,(-1877966186_i32)];
_12 = Adt35::Variant1 { fld0: _11.fld1 };
_11.fld3 = 24918_u16;
RET = 4_u8;
_11.fld3 = !62573_u16;
_11.fld1 = (-19448_i16) as i64;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
5 => bb6,
4 => bb9,
_ => bb7
}
}
bb9 = {
_9 = false | true;
_9 = true;
_11.fld1 = !Field::<i64>(Variant(_12, 1), 0);
RET = 70_u8;
place!(Field::<i64>(Variant(_12, 1), 0)) = _11.fld1 | _11.fld1;
RET = !124_u8;
RET = (-101_i8) as u8;
_3 = [1698977127_i32,(-103792464_i32),1721090329_i32,(-2097291108_i32),(-1812770409_i32),(-788002684_i32),533733692_i32];
_11.fld3 = 25659_u16;
_14 = _7;
_11.fld0 = 324442998562699500281881581260003478276_u128 + 89368596198959846615597050034570178174_u128;
_4 = [(-2081631507_i32),(-2115503936_i32),(-1812999440_i32),36938485_i32,692125005_i32,965480483_i32,3728126_i32];
_6 = [1382858945_i32,1523435090_i32,475713221_i32,(-493629587_i32),1124015709_i32,(-1835501438_i32),(-1217981361_i32)];
_10 = [_11.fld3];
_1.1 = 13311206647108187271_usize;
_14 = [(-485655775_i32),(-2056722260_i32),(-234469334_i32),1869634308_i32,(-499831158_i32),(-1134900540_i32),(-460974299_i32)];
match _1.1 {
0 => bb8,
1 => bb4,
2 => bb3,
3 => bb10,
13311206647108187271 => bb12,
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
RET = 187_u8;
_8 = [RET,RET,RET];
_11.fld0 = !201840300544725148497345723363983995291_u128;
_4 = [(-1944205289_i32),(-1298708595_i32),(-949293335_i32),1117172999_i32,(-987989350_i32),1401542180_i32,1336538373_i32];
_4 = [1051869826_i32,1913610672_i32,449138984_i32,(-1321742355_i32),1404332826_i32,1282848154_i32,1057001848_i32];
SetDiscriminant(_12, 1);
_2 = _3;
_11.fld3 = 3755060161_u32 as u16;
_4 = _6;
_17 = '\u{eb865}';
match RET {
0 => bb6,
1 => bb3,
2 => bb13,
3 => bb14,
187 => bb16,
_ => bb15
}
}
bb13 = {
Return()
}
bb14 = {
_10 = [53503_u16];
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
225 => bb8,
_ => bb7
}
}
bb15 = {
_11.fld3 = (-1520339050_i32) as u16;
_3 = [(-1262607642_i32),1573927956_i32,(-1836615491_i32),125621169_i32,2104175498_i32,(-2103752830_i32),(-709289644_i32)];
_6 = _3;
_5 = 9223372036854775807_isize;
_5 = 27083_i16 as isize;
_2 = _3;
_11.fld0 = !235773409580318670018789100900015593023_u128;
_10 = [_11.fld3];
_11.fld3 = !58237_u16;
_11.fld1 = 7573_i16 as i64;
_7 = [861697782_i32,(-103750595_i32),(-1080918221_i32),(-561398609_i32),(-1101257390_i32),(-1344647946_i32),(-1613418993_i32)];
RET = 1062399790_i32 as u8;
_2 = [(-1852840435_i32),1583342409_i32,(-57141109_i32),2145856932_i32,494035329_i32,1699326806_i32,9151823_i32];
_4 = [402491246_i32,(-1871672138_i32),(-2067943310_i32),264693597_i32,(-2125521572_i32),2104774059_i32,413951319_i32];
_3 = [(-192405115_i32),1365330698_i32,317195832_i32,(-2092605122_i32),(-1527713297_i32),691870488_i32,(-2137773471_i32)];
_9 = !true;
_6 = [1816738715_i32,1731175505_i32,1090904883_i32,(-1735575652_i32),(-1236321813_i32),148538690_i32,2028036782_i32];
_11.fld1 = 1567544879_i32 as i64;
_10 = [_11.fld3];
_2 = [578408111_i32,1286664550_i32,1051371058_i32,(-528695007_i32),(-933560565_i32),580914455_i32,1658345348_i32];
_10 = [_11.fld3];
_10 = [_11.fld3];
_10 = [_11.fld3];
_7 = _6;
_6 = [(-1171624551_i32),(-710252758_i32),153576913_i32,1554776431_i32,(-1370142090_i32),1221465071_i32,(-1877966186_i32)];
_12 = Adt35::Variant1 { fld0: _11.fld1 };
_11.fld3 = 24918_u16;
RET = 4_u8;
_11.fld3 = !62573_u16;
_11.fld1 = (-19448_i16) as i64;
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
5 => bb6,
4 => bb9,
_ => bb7
}
}
bb16 = {
_19 = _11.fld0 as f64;
_8 = [RET,RET,RET];
_2 = [(-871739548_i32),(-1100693389_i32),87979204_i32,333044167_i32,1058502645_i32,(-1527300370_i32),1950767200_i32];
_14 = [339379227_i32,(-1406862947_i32),1641127708_i32,(-591134410_i32),938938079_i32,1002780333_i32,(-127093387_i32)];
_18 = _17;
_16 = _11.fld1 as f32;
_11.fld3 = 40943_u16 & 64777_u16;
_5 = (-9223372036854775808_isize);
_11.fld3 = 17532_u16 + 29042_u16;
_22 = Adt18::Variant3 { fld0: 1774227434644558217_u64,fld1: _18,fld2: _1.1,fld3: _19,fld4: (-16857_i16),fld5: (-1567609914_i32),fld6: _11.fld3,fld7: 4076567658_u32 };
place!(Field::<i32>(Variant(_22, 3), 5)) = 1949817825_i32 >> _1.1;
_19 = Field::<f64>(Variant(_22, 3), 3);
_20 = Field::<i32>(Variant(_22, 3), 5) as isize;
place!(Field::<i16>(Variant(_22, 3), 4)) = 19313_i16;
_3 = [Field::<i32>(Variant(_22, 3), 5),Field::<i32>(Variant(_22, 3), 5),Field::<i32>(Variant(_22, 3), 5),Field::<i32>(Variant(_22, 3), 5),Field::<i32>(Variant(_22, 3), 5),Field::<i32>(Variant(_22, 3), 5),Field::<i32>(Variant(_22, 3), 5)];
_1.1 = RET as usize;
_14 = [Field::<i32>(Variant(_22, 3), 5),Field::<i32>(Variant(_22, 3), 5),Field::<i32>(Variant(_22, 3), 5),Field::<i32>(Variant(_22, 3), 5),Field::<i32>(Variant(_22, 3), 5),Field::<i32>(Variant(_22, 3), 5),Field::<i32>(Variant(_22, 3), 5)];
place!(Field::<i16>(Variant(_22, 3), 4)) = 19310_i16 ^ 18268_i16;
RET = _9 as u8;
_3 = _14;
_23 = _16;
Goto(bb17)
}
bb17 = {
Call(_24 = dump_var(17_usize, 14_usize, Move(_14), 6_usize, Move(_6), 10_usize, Move(_10), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_24 = dump_var(17_usize, 20_usize, Move(_20), 7_usize, Move(_7), 25_usize, _25, 25_usize, _25), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18() -> bool {
mir! {
type RET = bool;
let _1: Adt35;
let _2: isize;
let _3: *mut f32;
let _4: [u128; 7];
let _5: bool;
let _6: char;
let _7: [u128; 8];
let _8: (usize, i128);
let _9: isize;
let _10: f64;
let _11: isize;
let _12: f64;
let _13: &'static [u128; 7];
let _14: &'static &'static [u128; 5];
let _15: *const &'static *const Adt19;
let _16: i8;
let _17: &'static i8;
let _18: isize;
let _19: char;
let _20: char;
let _21: bool;
let _22: *const i128;
let _23: isize;
let _24: char;
let _25: bool;
let _26: bool;
let _27: bool;
let _28: isize;
let _29: i8;
let _30: (usize, i128);
let _31: *const usize;
let _32: bool;
let _33: char;
let _34: &'static u128;
let _35: [u128; 8];
let _36: ([bool; 6], &'static u32);
let _37: (&'static i8, *mut *mut u128, &'static isize);
let _38: ();
let _39: ();
{
RET = true;
RET = false | false;
RET = !false;
RET = !true;
_4 = [207588013718157424906598617988409287101_u128,296387378507062755401970175972908034852_u128,154223690340566355379945528423038735299_u128,298518475262485699536141427755636806871_u128,183894824133749719790498203277171719823_u128,104539784911479117129709089430162253951_u128,192205264306083624248579281623091922325_u128];
RET = 8882271197805164950_u64 < 6893116458843198570_u64;
_2 = 9223372036854775807_isize;
_5 = !RET;
_5 = RET == RET;
_5 = RET & RET;
_6 = '\u{dfb9a}';
_6 = '\u{6175e}';
_5 = RET;
_1 = Adt35::Variant1 { fld0: 7282492140013668254_i64 };
place!(Field::<i64>(Variant(_1, 1), 0)) = (-4412296522288345472_i64);
SetDiscriminant(_1, 1);
_8 = (4_usize, 21780282407713016857772708157315712738_i128);
_8.0 = 14338027552273846846_usize - 14148036474735204701_usize;
RET = _5;
_5 = RET;
_6 = '\u{8a983}';
match _2 {
0 => bb1,
9223372036854775807 => bb3,
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
_6 = '\u{19d3d}';
_4 = [259289767371405707609219425521175552582_u128,273639086001801207962643433170854158338_u128,100140637907768671137623834976437403186_u128,89038160685654787104183009894579662375_u128,192343843957029635226879220780771635687_u128,284872450872553468120730390676315279217_u128,106003489914599960747732671664308973707_u128];
_9 = _6 as isize;
_7 = [72054363489584311132953091543111389020_u128,201091548815753033006924101971267751080_u128,70302781693006050534094791319998748955_u128,292044173681201501992792897249336050827_u128,189640795216689955943167097038533200934_u128,277458352611574228755815404812360032983_u128,1384317500686993641102596999888402775_u128,335195029504462467573284336632292012408_u128];
_1 = Adt35::Variant0 { fld0: _5,fld1: _6,fld2: _8,fld3: 235848425538880455713871252505017832799_u128,fld4: 24026_u16 };
_8.1 = Field::<(usize, i128)>(Variant(_1, 0), 2).1 - Field::<(usize, i128)>(Variant(_1, 0), 2).1;
place!(Field::<char>(Variant(_1, 0), 1)) = _6;
_1 = Adt35::Variant0 { fld0: RET,fld1: _6,fld2: _8,fld3: 324708650934765676530434979879303924366_u128,fld4: 49932_u16 };
_10 = Field::<(usize, i128)>(Variant(_1, 0), 2).0 as f64;
_5 = RET & Field::<bool>(Variant(_1, 0), 0);
place!(Field::<(usize, i128)>(Variant(_1, 0), 2)) = _8;
place!(Field::<(usize, i128)>(Variant(_1, 0), 2)) = (_8.0, _8.1);
_11 = _10 as isize;
_11 = Field::<bool>(Variant(_1, 0), 0) as isize;
place!(Field::<bool>(Variant(_1, 0), 0)) = _8.1 <= Field::<(usize, i128)>(Variant(_1, 0), 2).1;
_10 = 4085630449981639040_u64 as f64;
place!(Field::<u128>(Variant(_1, 0), 3)) = !263741922979507109957352493350872772583_u128;
_6 = Field::<char>(Variant(_1, 0), 1);
place!(Field::<(usize, i128)>(Variant(_1, 0), 2)).0 = !_8.0;
_6 = Field::<char>(Variant(_1, 0), 1);
match _2 {
9223372036854775807 => bb4,
_ => bb1
}
}
bb4 = {
place!(Field::<u16>(Variant(_1, 0), 4)) = 51795_u16 >> Field::<(usize, i128)>(Variant(_1, 0), 2).1;
place!(Field::<(usize, i128)>(Variant(_1, 0), 2)) = _8;
_13 = &_4;
_10 = Field::<(usize, i128)>(Variant(_1, 0), 2).0 as f64;
place!(Field::<char>(Variant(_1, 0), 1)) = _6;
_5 = Field::<bool>(Variant(_1, 0), 0);
_11 = _2 >> Field::<u16>(Variant(_1, 0), 4);
place!(Field::<(usize, i128)>(Variant(_1, 0), 2)).0 = _8.0;
Call(_6 = fn19(Move(_13), Field::<(usize, i128)>(Variant(_1, 0), 2), Move(_1), _8, (*_13), _8, (*_13), (*_13), _11, _11, _11, _11), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_11 = !_9;
_5 = !RET;
_5 = RET ^ RET;
_5 = !RET;
_11 = !_9;
_8.0 = 3893967577_u32 as usize;
_1 = Adt35::Variant1 { fld0: (-1705880217272750997_i64) };
_6 = '\u{32d36}';
_12 = 106_u8 as f64;
RET = !_5;
_6 = '\u{48411}';
RET = !_5;
_6 = '\u{836d7}';
_8.0 = 458722351733067295_u64 as usize;
_16 = 5598360966417453099_i64 as i8;
_2 = _9 ^ _11;
place!(Field::<i64>(Variant(_1, 1), 0)) = 639216446208202496_i64 & (-5478398625147440133_i64);
_16 = (-34_i8);
_4 = [44524928457084261272082000165805621572_u128,290985829676409209266923346205589238048_u128,168479637576694119797263096294570059023_u128,92861295229165685661933668868309495675_u128,262686846027979607638531519101812399294_u128,99675708226186456939221718508340533581_u128,229988956583672858093932695447234931046_u128];
SetDiscriminant(_1, 2);
Goto(bb6)
}
bb6 = {
_13 = &_4;
_20 = _6;
_8 = (18292166398761761755_usize, 107415437659375708908724466711674935373_i128);
_17 = &_16;
_18 = 11447_u16 as isize;
Goto(bb7)
}
bb7 = {
_10 = (-207015471_i32) as f64;
_8.0 = !3_usize;
_18 = (-769412550_i32) as isize;
_7 = [31725845306714783370492315010242464509_u128,123933788944155157450056941475515327305_u128,57017257236979351218128374693270023387_u128,99643446910983449063631487873668449591_u128,226802993609879252129113119397024719033_u128,275899527397701013062464992391685459835_u128,319900034334604738500585081157999198258_u128,204362580091158788795425690283353556543_u128];
_18 = _11 << (*_17);
_8.1 = 163012367149880922386706983761404052813_i128 + 55014370196871556665058553602364322340_i128;
_11 = 1381086739_u32 as isize;
_5 = RET;
_21 = (*_17) != (*_17);
_4 = [82005613311935621008852144177736705367_u128,257797792046949251842088661283082396623_u128,255377310517309578858648999738709744584_u128,308574796970170382407372267992518767875_u128,226358498706067246993283510229565039236_u128,260120666136986368057669627940151646050_u128,227578614037262466372301693093776388362_u128];
_17 = &(*_17);
_16 = _8.0 as i8;
_4 = [35148673396590461232684586439142155770_u128,55771255410396240725047157770762297233_u128,165116284115413368404410318363153147858_u128,123322532643841291997273586070761667060_u128,23853639309703345771705412909147289475_u128,272753166633474839623544442059225473958_u128,302984296959243895419074990102969669368_u128];
RET = !_21;
_12 = 17059_i16 as f64;
_8.0 = 5_usize >> _18;
_8.1 = !(-84546022157100142854642713371523695386_i128);
_13 = &_4;
Call(_8.0 = core::intrinsics::transmute(_11), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_18 = _11 + _11;
_1 = Adt35::Variant1 { fld0: (-1138239429546464262_i64) };
_6 = _20;
_11 = -_2;
_22 = core::ptr::addr_of!(_8.1);
_19 = _6;
(*_22) = -(-80602563462597449826797382048562939274_i128);
_10 = _16 as f64;
_5 = !RET;
Call(place!(Field::<i64>(Variant(_1, 1), 0)) = core::intrinsics::transmute(_2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_8 = (8542139023531925046_usize, (-9217057810846408039570914698003389513_i128));
_22 = core::ptr::addr_of!((*_22));
_20 = _6;
_8.0 = 4223402460816992424_usize >> (*_22);
_13 = &_4;
_12 = 201_u8 as f64;
_13 = &_4;
_10 = _12;
_23 = _2;
_24 = _20;
_12 = _10;
_12 = -_10;
_17 = &_16;
RET = _21 == _5;
SetDiscriminant(_1, 2);
_25 = _21 & _5;
Goto(bb10)
}
bb10 = {
_13 = &(*_13);
_13 = &(*_13);
_1 = Adt35::Variant1 { fld0: (-7903188572835368963_i64) };
_27 = _19 <= _6;
(*_22) = !98589003446224340932862612850458724952_i128;
_13 = &(*_13);
_8.1 = (-151211549430952539970976108476063700639_i128);
Goto(bb11)
}
bb11 = {
_24 = _6;
Goto(bb12)
}
bb12 = {
_26 = _5;
_25 = (*_17) == (*_17);
_2 = -_23;
_5 = _26 ^ _27;
_20 = _6;
Goto(bb13)
}
bb13 = {
_12 = _23 as f64;
_13 = &(*_13);
match (*_22) {
0 => bb14,
1 => bb15,
189070817489985923492398498955704510817 => bb17,
_ => bb16
}
}
bb14 = {
_26 = _5;
_25 = (*_17) == (*_17);
_2 = -_23;
_5 = _26 ^ _27;
_20 = _6;
Goto(bb13)
}
bb15 = {
_8 = (8542139023531925046_usize, (-9217057810846408039570914698003389513_i128));
_22 = core::ptr::addr_of!((*_22));
_20 = _6;
_8.0 = 4223402460816992424_usize >> (*_22);
_13 = &_4;
_12 = 201_u8 as f64;
_13 = &_4;
_10 = _12;
_23 = _2;
_24 = _20;
_12 = _10;
_12 = -_10;
_17 = &_16;
RET = _21 == _5;
SetDiscriminant(_1, 2);
_25 = _21 & _5;
Goto(bb10)
}
bb16 = {
_13 = &_4;
_20 = _6;
_8 = (18292166398761761755_usize, 107415437659375708908724466711674935373_i128);
_17 = &_16;
_18 = 11447_u16 as isize;
Goto(bb7)
}
bb17 = {
_19 = _24;
_8 = (3626291339424708001_usize, (-95960789843978008352359424104920129326_i128));
_25 = _5 <= _27;
_17 = &(*_17);
_19 = _6;
_33 = _20;
_16 = 38_i8;
_20 = _6;
_13 = &_4;
_16 = (-88_i8) >> (*_22);
_30.0 = !_8.0;
_28 = _8.1 as isize;
_9 = _11;
_1 = Adt35::Variant0 { fld0: _21,fld1: _33,fld2: _8,fld3: 305763333806902093317502658429220615414_u128,fld4: 31773_u16 };
_30.0 = _16 as usize;
RET = !Field::<bool>(Variant(_1, 0), 0);
place!(Field::<bool>(Variant(_1, 0), 0)) = !_25;
_2 = _11;
Goto(bb18)
}
bb18 = {
Call(_38 = dump_var(18_usize, 28_usize, Move(_28), 2_usize, Move(_2), 11_usize, Move(_11), 4_usize, Move(_4)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_38 = dump_var(18_usize, 18_usize, Move(_18), 9_usize, Move(_9), 5_usize, Move(_5), 8_usize, Move(_8)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_38 = dump_var(18_usize, 23_usize, Move(_23), 24_usize, Move(_24), 39_usize, _39, 39_usize, _39), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: &'static [u128; 7],mut _2: (usize, i128),mut _3: Adt35,mut _4: (usize, i128),mut _5: [u128; 7],mut _6: (usize, i128),mut _7: [u128; 7],mut _8: [u128; 7],mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize) -> char {
mir! {
type RET = char;
let _13: *const &'static Adt35;
let _14: [u128; 5];
let _15: u32;
let _16: &'static isize;
let _17: Adt78;
let _18: Adt19;
let _19: f32;
let _20: &'static [u128; 2];
let _21: [i32; 6];
let _22: f64;
let _23: bool;
let _24: [bool; 6];
let _25: char;
let _26: bool;
let _27: *mut Adt35;
let _28: f32;
let _29: (Adt35, isize, [u128; 2]);
let _30: Adt19;
let _31: isize;
let _32: *mut u128;
let _33: ((*const Adt19, usize), (*const Adt19, usize), u64);
let _34: usize;
let _35: [i128; 7];
let _36: isize;
let _37: isize;
let _38: *const i32;
let _39: &'static u32;
let _40: bool;
let _41: usize;
let _42: &'static *mut u128;
let _43: usize;
let _44: [i128; 7];
let _45: f64;
let _46: (&'static i8, *mut *mut u128, &'static isize);
let _47: (Adt35, isize, [u128; 2]);
let _48: *const &'static *const Adt19;
let _49: f32;
let _50: [i32; 6];
let _51: ();
let _52: ();
{
place!(Field::<(usize, i128)>(Variant(_3, 0), 2)).1 = 245_u8 as i128;
_2.0 = _4.0 << _4.1;
_4.0 = _2.0;
_7 = [Field::<u128>(Variant(_3, 0), 3),Field::<u128>(Variant(_3, 0), 3),Field::<u128>(Variant(_3, 0), 3),Field::<u128>(Variant(_3, 0), 3),Field::<u128>(Variant(_3, 0), 3),Field::<u128>(Variant(_3, 0), 3),Field::<u128>(Variant(_3, 0), 3)];
_3 = Adt35::Variant1 { fld0: 8827308982281244566_i64 };
Goto(bb1)
}
bb1 = {
_4.1 = _6.1;
_5 = [331052988634269336062437460862403198307_u128,332658688735934383086468576807815493017_u128,222867794266185597977212632550321300526_u128,258237486008932372452194252358021772979_u128,256884543111183861709083613802578281701_u128,2614171028042194183742260883163409091_u128,73564748711438432273264332549649917330_u128];
_8 = [282904923025686609107192609167920658633_u128,112095955757972839555900566336854418789_u128,27647321879562084966912059818772418822_u128,90344369190477584252291859552617001188_u128,320629038910752566741286289477218611280_u128,238303552664352429216122511051328459603_u128,28102894659711016637352884811824981339_u128];
_2.1 = _4.1;
_6.1 = _2.1 ^ _2.1;
RET = '\u{81b2a}';
_4.0 = !_2.0;
place!(Field::<i64>(Variant(_3, 1), 0)) = (-2235534901890638557_i64) << _9;
_14 = [324397978107571917536777328281539876357_u128,276316802526063701634468273096130609150_u128,335436799911537238109794583147806336370_u128,284941150270598639291928224080018528350_u128,5428524250892449240864668092466515456_u128];
_3 = Adt35::Variant1 { fld0: (-6485418600573081930_i64) };
_3 = Adt35::Variant1 { fld0: (-124539054952436163_i64) };
_8 = [179676163703993018976788726993762549321_u128,65348298566620593992484247846582133997_u128,257120863079408115835416181995920782838_u128,118098980806385132376746287659590648664_u128,281270497997328158901008008061410152541_u128,187989885778143864456616043737422265736_u128,138751255696197024651495893862387304976_u128];
_9 = -_11;
_8 = [178946271177201576828624058886618330383_u128,34327638703864742331406129550388844642_u128,39201431577217833291599134742167611161_u128,169635845254760103759165168566582582986_u128,125122541203640662334125631822028751244_u128,191030925429844043670803545096059434619_u128,152715982438805641536143604398964985030_u128];
_15 = !1410158291_u32;
_6.0 = _4.0 - _2.0;
_5 = [201799125163133574589669893355373403785_u128,207347183654259833400969147447123573437_u128,161956240788017141086114571067135557126_u128,179382345291419804830397357251014344605_u128,132524742130720919080755655644818644678_u128,207387026759251522517397191756500580354_u128,304075923939230818309280519233173040539_u128];
_14 = [84504790194413810077114573422608228513_u128,243929498787072288199974644285939502739_u128,304325078824012065341997537262885687712_u128,730368530830030661902186017514083908_u128,209337646380827884796765973337402248419_u128];
Goto(bb2)
}
bb2 = {
_2 = (_6.0, _4.1);
_6.0 = _2.0;
_2.1 = _4.1 + _4.1;
_6.1 = -_2.1;
RET = '\u{b44c3}';
_12 = _9 >> _6.1;
_7 = [215594398083322026166255967279654492993_u128,176130294379445419863248613852943266013_u128,65665141751696608224155136564308215183_u128,333235834332054002579266183047809305679_u128,31262502484890223550892204223012654418_u128,320636112534749280708395834929866258384_u128,49809442775223193271632606540936359900_u128];
_16 = &_10;
place!(Field::<i64>(Variant(_3, 1), 0)) = -6628106778547249997_i64;
_1 = &_5;
_12 = _10;
SetDiscriminant(_3, 1);
_16 = &_10;
_19 = 15_u8 as f32;
_14 = [194330929620376789464298759053542869620_u128,109134748726375754068834462645758217240_u128,194563536085368466760114762809883450584_u128,67167418143228805430288922759447224094_u128,178761298113977389816837373324695684841_u128];
place!(Field::<i64>(Variant(_3, 1), 0)) = (-1582283005145272058_i64);
_22 = (-30_i8) as f64;
match Field::<i64>(Variant(_3, 1), 0) {
340282366920938463461792324426622939398 => bb4,
_ => bb3
}
}
bb3 = {
_4.1 = _6.1;
_5 = [331052988634269336062437460862403198307_u128,332658688735934383086468576807815493017_u128,222867794266185597977212632550321300526_u128,258237486008932372452194252358021772979_u128,256884543111183861709083613802578281701_u128,2614171028042194183742260883163409091_u128,73564748711438432273264332549649917330_u128];
_8 = [282904923025686609107192609167920658633_u128,112095955757972839555900566336854418789_u128,27647321879562084966912059818772418822_u128,90344369190477584252291859552617001188_u128,320629038910752566741286289477218611280_u128,238303552664352429216122511051328459603_u128,28102894659711016637352884811824981339_u128];
_2.1 = _4.1;
_6.1 = _2.1 ^ _2.1;
RET = '\u{81b2a}';
_4.0 = !_2.0;
place!(Field::<i64>(Variant(_3, 1), 0)) = (-2235534901890638557_i64) << _9;
_14 = [324397978107571917536777328281539876357_u128,276316802526063701634468273096130609150_u128,335436799911537238109794583147806336370_u128,284941150270598639291928224080018528350_u128,5428524250892449240864668092466515456_u128];
_3 = Adt35::Variant1 { fld0: (-6485418600573081930_i64) };
_3 = Adt35::Variant1 { fld0: (-124539054952436163_i64) };
_8 = [179676163703993018976788726993762549321_u128,65348298566620593992484247846582133997_u128,257120863079408115835416181995920782838_u128,118098980806385132376746287659590648664_u128,281270497997328158901008008061410152541_u128,187989885778143864456616043737422265736_u128,138751255696197024651495893862387304976_u128];
_9 = -_11;
_8 = [178946271177201576828624058886618330383_u128,34327638703864742331406129550388844642_u128,39201431577217833291599134742167611161_u128,169635845254760103759165168566582582986_u128,125122541203640662334125631822028751244_u128,191030925429844043670803545096059434619_u128,152715982438805641536143604398964985030_u128];
_15 = !1410158291_u32;
_6.0 = _4.0 - _2.0;
_5 = [201799125163133574589669893355373403785_u128,207347183654259833400969147447123573437_u128,161956240788017141086114571067135557126_u128,179382345291419804830397357251014344605_u128,132524742130720919080755655644818644678_u128,207387026759251522517397191756500580354_u128,304075923939230818309280519233173040539_u128];
_14 = [84504790194413810077114573422608228513_u128,243929498787072288199974644285939502739_u128,304325078824012065341997537262885687712_u128,730368530830030661902186017514083908_u128,209337646380827884796765973337402248419_u128];
Goto(bb2)
}
bb4 = {
_14 = [306781704687706198293088560235298932749_u128,228179143892302477886619251583781761528_u128,243811627016254746380899644120014939682_u128,249026339171662210775823969160123890925_u128,66377795348995364750383614728748364571_u128];
_2 = (_6.0, _6.1);
_23 = _9 == _10;
_6.0 = 83669352964542568965670198461255111306_u128 as usize;
_8 = [158050359870866689011228856383456987894_u128,134847072665314949088300852265617797896_u128,335722337139601552895196954405175852942_u128,254702071555503382546263950792199701744_u128,83368175734286556447025519899414574777_u128,28552722500903352127363740654011797514_u128,152134782524571636885551154736103133218_u128];
RET = '\u{a508e}';
_2 = (_4.0, _6.1);
_8 = [179485849893014145851158408992091347996_u128,130362095671094638515656368284595398999_u128,182412085507366677923611315868029295568_u128,169783715414188632348735162629898009021_u128,210966007570529315546880300633134813329_u128,282116761123941830376671551698890893475_u128,247638724796254528751824214690397105368_u128];
_10 = -_9;
_2.1 = !_6.1;
_9 = _10;
_10 = _15 as isize;
_16 = &_9;
_9 = -_11;
_2.1 = -_6.1;
_10 = _11 | _12;
match Field::<i64>(Variant(_3, 1), 0) {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
340282366920938463461792324426622939398 => bb8,
_ => bb7
}
}
bb5 = {
_4.1 = _6.1;
_5 = [331052988634269336062437460862403198307_u128,332658688735934383086468576807815493017_u128,222867794266185597977212632550321300526_u128,258237486008932372452194252358021772979_u128,256884543111183861709083613802578281701_u128,2614171028042194183742260883163409091_u128,73564748711438432273264332549649917330_u128];
_8 = [282904923025686609107192609167920658633_u128,112095955757972839555900566336854418789_u128,27647321879562084966912059818772418822_u128,90344369190477584252291859552617001188_u128,320629038910752566741286289477218611280_u128,238303552664352429216122511051328459603_u128,28102894659711016637352884811824981339_u128];
_2.1 = _4.1;
_6.1 = _2.1 ^ _2.1;
RET = '\u{81b2a}';
_4.0 = !_2.0;
place!(Field::<i64>(Variant(_3, 1), 0)) = (-2235534901890638557_i64) << _9;
_14 = [324397978107571917536777328281539876357_u128,276316802526063701634468273096130609150_u128,335436799911537238109794583147806336370_u128,284941150270598639291928224080018528350_u128,5428524250892449240864668092466515456_u128];
_3 = Adt35::Variant1 { fld0: (-6485418600573081930_i64) };
_3 = Adt35::Variant1 { fld0: (-124539054952436163_i64) };
_8 = [179676163703993018976788726993762549321_u128,65348298566620593992484247846582133997_u128,257120863079408115835416181995920782838_u128,118098980806385132376746287659590648664_u128,281270497997328158901008008061410152541_u128,187989885778143864456616043737422265736_u128,138751255696197024651495893862387304976_u128];
_9 = -_11;
_8 = [178946271177201576828624058886618330383_u128,34327638703864742331406129550388844642_u128,39201431577217833291599134742167611161_u128,169635845254760103759165168566582582986_u128,125122541203640662334125631822028751244_u128,191030925429844043670803545096059434619_u128,152715982438805641536143604398964985030_u128];
_15 = !1410158291_u32;
_6.0 = _4.0 - _2.0;
_5 = [201799125163133574589669893355373403785_u128,207347183654259833400969147447123573437_u128,161956240788017141086114571067135557126_u128,179382345291419804830397357251014344605_u128,132524742130720919080755655644818644678_u128,207387026759251522517397191756500580354_u128,304075923939230818309280519233173040539_u128];
_14 = [84504790194413810077114573422608228513_u128,243929498787072288199974644285939502739_u128,304325078824012065341997537262885687712_u128,730368530830030661902186017514083908_u128,209337646380827884796765973337402248419_u128];
Goto(bb2)
}
bb6 = {
_2 = (_6.0, _4.1);
_6.0 = _2.0;
_2.1 = _4.1 + _4.1;
_6.1 = -_2.1;
RET = '\u{b44c3}';
_12 = _9 >> _6.1;
_7 = [215594398083322026166255967279654492993_u128,176130294379445419863248613852943266013_u128,65665141751696608224155136564308215183_u128,333235834332054002579266183047809305679_u128,31262502484890223550892204223012654418_u128,320636112534749280708395834929866258384_u128,49809442775223193271632606540936359900_u128];
_16 = &_10;
place!(Field::<i64>(Variant(_3, 1), 0)) = -6628106778547249997_i64;
_1 = &_5;
_12 = _10;
SetDiscriminant(_3, 1);
_16 = &_10;
_19 = 15_u8 as f32;
_14 = [194330929620376789464298759053542869620_u128,109134748726375754068834462645758217240_u128,194563536085368466760114762809883450584_u128,67167418143228805430288922759447224094_u128,178761298113977389816837373324695684841_u128];
place!(Field::<i64>(Variant(_3, 1), 0)) = (-1582283005145272058_i64);
_22 = (-30_i8) as f64;
match Field::<i64>(Variant(_3, 1), 0) {
340282366920938463461792324426622939398 => bb4,
_ => bb3
}
}
bb7 = {
_4.1 = _6.1;
_5 = [331052988634269336062437460862403198307_u128,332658688735934383086468576807815493017_u128,222867794266185597977212632550321300526_u128,258237486008932372452194252358021772979_u128,256884543111183861709083613802578281701_u128,2614171028042194183742260883163409091_u128,73564748711438432273264332549649917330_u128];
_8 = [282904923025686609107192609167920658633_u128,112095955757972839555900566336854418789_u128,27647321879562084966912059818772418822_u128,90344369190477584252291859552617001188_u128,320629038910752566741286289477218611280_u128,238303552664352429216122511051328459603_u128,28102894659711016637352884811824981339_u128];
_2.1 = _4.1;
_6.1 = _2.1 ^ _2.1;
RET = '\u{81b2a}';
_4.0 = !_2.0;
place!(Field::<i64>(Variant(_3, 1), 0)) = (-2235534901890638557_i64) << _9;
_14 = [324397978107571917536777328281539876357_u128,276316802526063701634468273096130609150_u128,335436799911537238109794583147806336370_u128,284941150270598639291928224080018528350_u128,5428524250892449240864668092466515456_u128];
_3 = Adt35::Variant1 { fld0: (-6485418600573081930_i64) };
_3 = Adt35::Variant1 { fld0: (-124539054952436163_i64) };
_8 = [179676163703993018976788726993762549321_u128,65348298566620593992484247846582133997_u128,257120863079408115835416181995920782838_u128,118098980806385132376746287659590648664_u128,281270497997328158901008008061410152541_u128,187989885778143864456616043737422265736_u128,138751255696197024651495893862387304976_u128];
_9 = -_11;
_8 = [178946271177201576828624058886618330383_u128,34327638703864742331406129550388844642_u128,39201431577217833291599134742167611161_u128,169635845254760103759165168566582582986_u128,125122541203640662334125631822028751244_u128,191030925429844043670803545096059434619_u128,152715982438805641536143604398964985030_u128];
_15 = !1410158291_u32;
_6.0 = _4.0 - _2.0;
_5 = [201799125163133574589669893355373403785_u128,207347183654259833400969147447123573437_u128,161956240788017141086114571067135557126_u128,179382345291419804830397357251014344605_u128,132524742130720919080755655644818644678_u128,207387026759251522517397191756500580354_u128,304075923939230818309280519233173040539_u128];
_14 = [84504790194413810077114573422608228513_u128,243929498787072288199974644285939502739_u128,304325078824012065341997537262885687712_u128,730368530830030661902186017514083908_u128,209337646380827884796765973337402248419_u128];
Goto(bb2)
}
bb8 = {
_6 = (_2.0, _2.1);
_21 = [242287790_i32,834502041_i32,1691148339_i32,1669992724_i32,(-1769741674_i32),1063197358_i32];
_23 = true | false;
_4.0 = _2.0 + _6.0;
_10 = _9 << _9;
_2 = (_4.0, _6.1);
_15 = !2873434897_u32;
_11 = !_10;
RET = '\u{c9401}';
_26 = _23 & _23;
_1 = &_8;
_25 = RET;
_6.1 = 10318145539692412136_u64 as i128;
_1 = &_8;
_6.1 = 31596_i16 as i128;
Goto(bb9)
}
bb9 = {
_6 = _4;
SetDiscriminant(_3, 0);
_6 = (_4.0, _2.1);
_20 = &_29.2;
place!(Field::<char>(Variant(_3, 0), 1)) = _25;
_15 = 2357836788_u32 | 894555704_u32;
place!(Field::<u16>(Variant(_3, 0), 4)) = !38076_u16;
_14 = [312973348075758379535458102279770614149_u128,272973814138486013824865121178567002060_u128,265534027146710408739887990745176666908_u128,1379931924930923568159496325995488536_u128,24761286117574330473709970209521981788_u128];
_2 = (_4.0, _6.1);
_29.0 = Adt35::Variant1 { fld0: (-5051595103292897111_i64) };
Goto(bb10)
}
bb10 = {
_24 = [_26,_26,_26,_26,_23,_23];
_24 = [_26,_23,_23,_26,_26,_26];
_3 = Adt35::Variant1 { fld0: (-3936375458695257095_i64) };
_14 = [157865792659635956118086976429475040929_u128,163347417849660317044825825166023175267_u128,263739320559170498572847724594399563369_u128,322189340457909356434087629604940568100_u128,158655609051967144511598816208771680132_u128];
_30 = Adt19::Variant0 { fld0: _19,fld1: RET,fld2: _22,fld3: 284692697149758294850902148976234527424_u128,fld4: _2.0,fld5: 642456942_i32,fld6: 6273469209085746836_i64,fld7: _2.1 };
_1 = &(*_1);
_23 = _11 != _10;
_28 = _2.1 as f32;
place!(Field::<i64>(Variant(_30, 0), 6)) = 8664728683941325198_i64 & 4833672304417253531_i64;
_26 = _23 ^ _23;
_31 = 92_i8 as isize;
_2 = (Field::<usize>(Variant(_30, 0), 4), Field::<i128>(Variant(_30, 0), 7));
_23 = _2.0 != Field::<usize>(Variant(_30, 0), 4);
_22 = -Field::<f64>(Variant(_30, 0), 2);
_33.1.1 = Field::<usize>(Variant(_30, 0), 4) << _4.0;
_20 = &(*_20);
_8 = [121527467332288891853232626702264746870_u128,227085071590810284913261586758901051244_u128,29631470847699598530813051733724836827_u128,96247850456017681471027534460406292288_u128,45409015976317907218274195448052800380_u128,249544140614729068315938091231586941211_u128,198761657187233538539970136890822577249_u128];
_20 = &_29.2;
_29.0 = Adt35::Variant1 { fld0: Field::<i64>(Variant(_30, 0), 6) };
_16 = &_11;
_27 = core::ptr::addr_of_mut!(_29.0);
_33.1.1 = _2.0 << (*_16);
_33.0.0 = core::ptr::addr_of!(_18);
Call(_33.2 = core::intrinsics::transmute(Field::<usize>(Variant(_30, 0), 4)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
place!(Field::<i64>(Variant((*_27), 1), 0)) = Field::<i64>(Variant(_30, 0), 6);
_27 = core::ptr::addr_of_mut!((*_27));
_32 = core::ptr::addr_of_mut!(place!(Field::<u128>(Variant(_30, 0), 3)));
_33.0.1 = _33.1.1 + _4.0;
_4.0 = 4315_u16 as usize;
(*_27) = Adt35::Variant1 { fld0: Field::<i64>(Variant(_30, 0), 6) };
place!(Field::<usize>(Variant(_30, 0), 4)) = 235399510696021890213572533568360384228_u128 as usize;
(*_27) = Adt35::Variant1 { fld0: Field::<i64>(Variant(_30, 0), 6) };
Goto(bb12)
}
bb12 = {
_10 = (*_16);
_8 = [304145298779676956495814693769338090291_u128,325290089178432607150233964621032278540_u128,171588973490169648358235752749724324261_u128,229121633230092813600029581955572067447_u128,158729748811975652593879293362514514744_u128,90742161583319118867231010022079794946_u128,210604943910087071932494818449484028671_u128];
(*_32) = (*_16) as u128;
SetDiscriminant(_29.0, 2);
_4.0 = _19 as usize;
_3 = Adt35::Variant0 { fld0: _23,fld1: RET,fld2: _6,fld3: (*_32),fld4: 45778_u16 };
place!(Field::<u16>(Variant(_3, 0), 4)) = 15790_u16 << Field::<u128>(Variant(_3, 0), 3);
_1 = &_7;
_33.2 = Field::<(usize, i128)>(Variant(_3, 0), 2).0 as u64;
_38 = core::ptr::addr_of!(place!(Field::<i32>(Variant(_30, 0), 5)));
place!(Field::<*const Adt19>(Variant((*_27), 2), 0)) = core::ptr::addr_of!(_18);
_18 = Adt19::Variant0 { fld0: _28,fld1: Field::<char>(Variant(_3, 0), 1),fld2: Field::<f64>(Variant(_30, 0), 2),fld3: (*_32),fld4: _2.0,fld5: (-1243945384_i32),fld6: Field::<i64>(Variant(_30, 0), 6),fld7: _2.1 };
_22 = Field::<u128>(Variant(_18, 0), 3) as f64;
place!(Field::<*const Adt19>(Variant(_29.0, 2), 0)) = core::ptr::addr_of!(_30);
_10 = (-1155364458_i32) as isize;
_33.1.0 = core::ptr::addr_of!(_18);
_6.0 = (-21537_i16) as usize;
_2 = (Field::<(usize, i128)>(Variant(_3, 0), 2).0, _4.1);
_6.1 = -Field::<i128>(Variant(_30, 0), 7);
_11 = !_9;
_20 = &(*_20);
_4 = _6;
_39 = &_15;
Goto(bb13)
}
bb13 = {
_14 = [Field::<u128>(Variant(_18, 0), 3),(*_32),(*_32),Field::<u128>(Variant(_18, 0), 3),(*_32)];
RET = _25;
place!(Field::<bool>(Variant(_3, 0), 0)) = _26 <= _26;
place!(Field::<bool>(Variant(_3, 0), 0)) = Field::<i128>(Variant(_18, 0), 7) >= _6.1;
_28 = -Field::<f32>(Variant(_18, 0), 0);
(*_38) = -2003762848_i32;
_2.0 = Field::<u128>(Variant(_18, 0), 3) as usize;
_12 = _31 + _9;
_9 = !_12;
_16 = &_10;
_19 = -Field::<f32>(Variant(_18, 0), 0);
SetDiscriminant((*_27), 2);
_2.0 = _33.0.1 ^ _33.0.1;
_27 = core::ptr::addr_of_mut!(_29.0);
Goto(bb14)
}
bb14 = {
_7 = [Field::<u128>(Variant(_18, 0), 3),(*_32),Field::<u128>(Variant(_30, 0), 3),(*_32),(*_32),(*_32),Field::<u128>(Variant(_18, 0), 3)];
_40 = Field::<u16>(Variant(_3, 0), 4) != Field::<u16>(Variant(_3, 0), 4);
place!(Field::<*const Adt19>(Variant(_29.0, 2), 0)) = core::ptr::addr_of!(_18);
_46.1 = core::ptr::addr_of_mut!(_32);
_35 = [Field::<i128>(Variant(_30, 0), 7),Field::<i128>(Variant(_18, 0), 7),_4.1,Field::<i128>(Variant(_30, 0), 7),_6.1,_6.1,_6.1];
place!(Field::<u16>(Variant(_3, 0), 4)) = 40121_u16 + 26238_u16;
_5 = [Field::<u128>(Variant(_3, 0), 3),Field::<u128>(Variant(_3, 0), 3),Field::<u128>(Variant(_18, 0), 3),(*_32),Field::<u128>(Variant(_3, 0), 3),(*_32),(*_32)];
place!(Field::<u16>(Variant(_3, 0), 4)) = 2276_u16;
_27 = core::ptr::addr_of_mut!(_3);
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(19_usize, 10_usize, Move(_10), 25_usize, Move(_25), 15_usize, Move(_15), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(19_usize, 35_usize, Move(_35), 5_usize, Move(_5), 9_usize, Move(_9), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(19_usize, 2_usize, Move(_2), 26_usize, Move(_26), 52_usize, _52, 52_usize, _52), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(4200710084963387626_i64), std::hint::black_box('\u{d6c37}'), std::hint::black_box(40277_u16), std::hint::black_box(9278894669272286463_u64), std::hint::black_box(66252676023053924069612458779473764445_u128));
                
            }
impl PrintFDebug for Adt18{
	unsafe fn printf_debug(&self){unsafe{printf("Adt18::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt18 {
Variant0{
fld0: bool,
fld1: usize,
fld2: isize,
fld3: u128,

},
Variant1{
fld0: i32,
fld1: u32,

},
Variant2{
fld0: u16,

},
Variant3{
fld0: u64,
fld1: char,
fld2: usize,
fld3: f64,
fld4: i16,
fld5: i32,
fld6: u16,
fld7: u32,

}}
impl PrintFDebug for Adt19{
	unsafe fn printf_debug(&self){unsafe{printf("Adt19::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt19 {
Variant0{
fld0: f32,
fld1: char,
fld2: f64,
fld3: u128,
fld4: usize,
fld5: i32,
fld6: i64,
fld7: i128,

},
Variant1{
fld0: bool,
fld1: u8,
fld2: usize,
fld3: i8,
fld4: Adt18,
fld5: i32,
fld6: i64,

},
Variant2{
fld0: i32,
fld1: char,
fld2: isize,
fld3: i64,
fld4: Adt18,

}}
impl PrintFDebug for Adt29{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt29{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt29 {
fld0: u128,
fld1: i64,
fld2: *const Adt19,
fld3: u16,
}
impl PrintFDebug for Adt32{
	unsafe fn printf_debug(&self){unsafe{printf("Adt32::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt32 {
Variant0{
fld0: bool,
fld1: char,
fld2: Adt19,
fld3: f64,
fld4: i16,
fld5: u16,
fld6: usize,
fld7: i128,

},
Variant1{
fld0: f32,
fld1: ((*const Adt19, usize), (*const Adt19, usize), u64),
fld2: [bool; 6],
fld3: Adt18,
fld4: *const Adt19,
fld5: i32,

},
Variant2{
fld0: Adt29,
fld1: u32,
fld2: ((*const Adt19, usize), (*const Adt19, usize), u64),
fld3: [bool; 6],
fld4: *const Adt19,
fld5: u128,

}}
impl PrintFDebug for Adt35{
	unsafe fn printf_debug(&self){unsafe{printf("Adt35::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt35 {
Variant0{
fld0: bool,
fld1: char,
fld2: (usize, i128),
fld3: u128,
fld4: u16,

},
Variant1{
fld0: i64,

},
Variant2{
fld0: *const Adt19,

}}
impl PrintFDebug for Adt37{
	unsafe fn printf_debug(&self){unsafe{printf("Adt37::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt37 {
Variant0{
fld0: bool,
fld1: char,
fld2: Adt18,
fld3: (Adt35, isize, [u128; 2]),
fld4: Adt35,
fld5: i128,

},
Variant1{
fld0: (Adt35, isize, [u128; 2]),
fld1: char,
fld2: (usize, i128),
fld3: i8,
fld4: [u128; 2],
fld5: Adt35,
fld6: *const Adt19,

},
Variant2{
fld0: i128,
fld1: char,
fld2: u64,

},
Variant3{
fld0: i16,

}}
impl PrintFDebug for Adt73{
	unsafe fn printf_debug(&self){unsafe{printf("Adt73::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt73 {
Variant0{
fld0: [bool; 1],
fld1: [u16; 1],
fld2: u32,

},
Variant1{
fld0: *mut f32,
fld1: i32,
fld2: f32,
fld3: u64,
fld4: (Adt35, isize, [u128; 2]),

},
Variant2{
fld0: [bool; 1],
fld1: [bool; 6],

}}
impl PrintFDebug for Adt78{
	unsafe fn printf_debug(&self){unsafe{printf("Adt78::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt78 {
Variant0{
fld0: [char; 4],
fld1: Adt29,
fld2: (Adt35, isize, [u128; 2]),

},
Variant1{
fld0: bool,
fld1: [u16; 1],

}}

