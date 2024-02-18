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
pub fn fn0(mut _1: bool,mut _2: u128,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: u64) -> [usize; 1] {
mir! {
type RET = [usize; 1];
let _10: *const *mut &'static f64;
let _11: isize;
let _12: *const usize;
let _13: isize;
let _14: char;
let _15: *mut [usize; 8];
let _16: [i8; 7];
let _17: &'static u8;
let _18: i128;
let _19: [i32; 4];
let _20: &'static char;
let _21: Adt27;
let _22: [usize; 8];
let _23: [i16; 5];
let _24: Adt33;
let _25: &'static bool;
let _26: (char, i8, (&'static bool, (i128,)));
let _27: [i128; 5];
let _28: (*mut &'static u8, f64, i16);
let _29: ((&'static (&'static f64,), Adt34), *const i8, *mut &'static u8);
let _30: *mut Adt48;
let _31: &'static (&'static f64,);
let _32: Adt22;
let _33: [i128; 5];
let _34: ();
let _35: ();
{
_9 = 14787907281152219919_u64;
_7 = (-6677319028540936006_i64);
_1 = !true;
_2 = !227741112829997205940927569413599925921_u128;
_5 = (-24184_i16);
_3 = '\u{1090fa}' as isize;
RET = [4620244826141172030_usize];
_4 = 3166598483_u32 as i8;
_2 = !305555144812074971506916915993553260235_u128;
_2 = !310427284662759051744847569988947841100_u128;
_5 = 21572_i16 * (-31379_i16);
_9 = !14378158475731128211_u64;
_2 = !219098656010880355493733176918327371548_u128;
_11 = !_3;
_1 = _7 == _7;
RET = [3_usize];
_6 = 35732_u16 as i32;
Goto(bb1)
}
bb1 = {
RET = [13251647200864784842_usize];
_8 = (-60321518791457160023573731770747788865_i128) - (-73867455220789549957734829447832506704_i128);
_5 = !17274_i16;
_6 = -1682195457_i32;
_3 = 1_usize as isize;
RET = [3_usize];
_5 = _1 as i16;
RET = [5_usize];
_8 = (-88432776712436439725864532158682486565_i128) & (-66939985255622046596521223412582825770_i128);
_11 = !_3;
_5 = 14460_i16;
_4 = 75_i8 - 124_i8;
_13 = -_11;
_7 = 9013570620103624589_i64 | 1387072550832380121_i64;
_3 = -_13;
_5 = _2 as i16;
_11 = !_3;
_14 = '\u{f55f1}';
_1 = false;
_14 = '\u{47b02}';
_11 = -_3;
_2 = 20029920663113608864683435346124340195_u128;
_19 = [_6,_6,_6,_6];
RET = [3925535746592479475_usize];
match _2 {
0 => bb2,
1 => bb3,
2 => bb4,
20029920663113608864683435346124340195 => bb6,
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
_3 = _13 + _13;
_9 = 7663750048637871540_u64 ^ 7940812190440143654_u64;
_11 = !_3;
_2 = 120086935888495935870999157905198327134_u128;
_8 = -53328499407510451104253471768362388489_i128;
_13 = _3;
_4 = _6 as i8;
_5 = 4_usize as i16;
Goto(bb7)
}
bb7 = {
_14 = '\u{c5812}';
_1 = true;
RET = [6_usize];
_19 = [_6,_6,_6,_6];
_13 = !_11;
_14 = '\u{77dcc}';
_1 = true;
_4 = 22757_u16 as i8;
_5 = -31214_i16;
RET = [2_usize];
_2 = _14 as u128;
RET = [4_usize];
_5 = (-25072_i16) & (-3850_i16);
_18 = _8 >> _13;
_5 = 13927_i16 - 15719_i16;
_5 = 7771_i16;
_7 = 387943281791107981_i64;
_4 = !26_i8;
_6 = 1877240689_i32 >> _13;
_18 = -_8;
_6 = -842414556_i32;
RET = [239820150504607722_usize];
_16 = [_4,_4,_4,_4,_4,_4,_4];
_13 = 22857_u16 as isize;
_14 = '\u{e6fe4}';
Call(_4 = fn1(_7, _18, _13, _7, _3, _11, _7, _3, _3, _3), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_5 = _4 as i16;
_2 = 282847390924268228269006156026190190949_u128 | 47666957982544283469893104079876568813_u128;
_16 = [_4,_4,_4,_4,_4,_4,_4];
match _7 {
0 => bb4,
387943281791107981 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_7 = !4232049596368442443_i64;
_7 = 3901559669519927465_i64;
_4 = (-62_i8);
_6 = _14 as i32;
_14 = '\u{58952}';
_4 = !(-114_i8);
_20 = &_14;
_7 = (-2226423517586663989_i64) - (-3635172765961673105_i64);
_3 = _14 as isize;
_11 = _3 * _13;
_5 = 958_i16;
_2 = 39513920104499231363909686843265078986_u128 >> _13;
_3 = _11;
_11 = _7 as isize;
_20 = &(*_20);
_18 = _8;
_23 = [_5,_5,_5,_5,_5];
Goto(bb11)
}
bb11 = {
_20 = &_14;
_18 = _8 - _8;
RET = [4640686462188136453_usize];
_22 = [5_usize,4_usize,4_usize,1_usize,193160583137367215_usize,9190940489368236603_usize,5_usize,1_usize];
RET = [6_usize];
_5 = !(-10201_i16);
RET = [1_usize];
_4 = 93_i8 | (-108_i8);
_2 = 9862_u16 as u128;
_2 = 0_usize as u128;
_7 = !3375390908100612788_i64;
_22 = [6294405452854955574_usize,66096096843068959_usize,3363944861469625080_usize,11395548566090768347_usize,0_usize,4_usize,3493603492935914304_usize,4_usize];
_13 = _3;
_18 = _8 | _8;
_20 = &(*_20);
_13 = _11 + _3;
_25 = &_1;
Goto(bb12)
}
bb12 = {
_20 = &(*_20);
_15 = core::ptr::addr_of_mut!(_22);
_1 = !true;
_14 = '\u{eaa45}';
_26.0 = _14;
_18 = _2 as i128;
_19 = [_6,_6,_6,_6];
_26.2.1.0 = _6 as i128;
_26.0 = _14;
_22 = [9683795327818023132_usize,14637027752915277207_usize,4_usize,16084054309577845413_usize,7777675602456197425_usize,15930896055854982965_usize,7_usize,3_usize];
Goto(bb13)
}
bb13 = {
_27 = [_26.2.1.0,_26.2.1.0,_18,_8,_8];
RET = [5_usize];
_11 = _13;
(*_15) = [0_usize,6066499848377824493_usize,6_usize,4_usize,2_usize,8757044714762183466_usize,5_usize,6_usize];
RET = [1_usize];
_26.2.1.0 = _5 as i128;
_13 = _9 as isize;
_11 = -_13;
_13 = _3 + _3;
_26.2.0 = &_1;
_28.0 = core::ptr::addr_of_mut!(_17);
_4 = 118_i8 << _9;
_26.2.1.0 = _18;
_26.1 = _14 as i8;
_2 = _9 as u128;
_26.1 = -_4;
_26.2.1 = (_8,);
_25 = Move(_26.2.0);
_11 = 199803341_u32 as isize;
Goto(bb14)
}
bb14 = {
_19 = [_6,_6,_6,_6];
_1 = _26.0 != _14;
_15 = core::ptr::addr_of_mut!((*_15));
_22 = [7_usize,1_usize,6_usize,5_usize,17500510186723599562_usize,0_usize,10069165897491114357_usize,1_usize];
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(0_usize, 2_usize, Move(_2), 19_usize, Move(_19), 4_usize, Move(_4), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(0_usize, 27_usize, Move(_27), 22_usize, Move(_22), 5_usize, Move(_5), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(0_usize, 1_usize, Move(_1), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: i64,mut _2: i128,mut _3: isize,mut _4: i64,mut _5: isize,mut _6: isize,mut _7: i64,mut _8: isize,mut _9: isize,mut _10: isize) -> i8 {
mir! {
type RET = i8;
let _11: u16;
let _12: Adt33;
let _13: *mut Adt48;
let _14: Adt44;
let _15: ((char, Adt27), isize);
let _16: &'static f64;
let _17: isize;
let _18: Adt22;
let _19: [u32; 2];
let _20: (&'static *mut u8, bool);
let _21: *mut &'static u8;
let _22: *const *mut &'static f64;
let _23: *mut (&'static bool, (i128,));
let _24: Adt22;
let _25: isize;
let _26: Adt77;
let _27: &'static i32;
let _28: isize;
let _29: [i8; 3];
let _30: [i8; 7];
let _31: ();
let _32: ();
{
_5 = _9;
RET = (-91_i8) + 117_i8;
_6 = -_9;
_11 = _7 as u16;
RET = 27_i8 & 15_i8;
RET = (-71_i8);
_9 = _6 << _6;
_7 = !_4;
_4 = 15318661742963618322_u64 as i64;
_8 = _6;
_15.1 = !_6;
_15.0.0 = '\u{769c3}';
_15.1 = 20621_i16 as isize;
_7 = _1 + _1;
_5 = _6 | _6;
_8 = _5;
_2 = 3966032515_u32 as i128;
_8 = _5 ^ _6;
_2 = _15.0.0 as i128;
_18 = Adt22::Variant0 { fld0: 5316395235618639129_usize,fld1: _11 };
_10 = _8 - _9;
_3 = _10;
_2 = !(-38557280748299953226427490790172007724_i128);
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
387943281791107981 => bb8,
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
_11 = !Field::<u16>(Variant(_18, 0), 1);
_3 = !_10;
_9 = _15.1;
_11 = Field::<u16>(Variant(_18, 0), 1) | Field::<u16>(Variant(_18, 0), 1);
_8 = !_10;
_19 = [4195697287_u32,1534670012_u32];
_18 = Adt22::Variant1 { fld0: false,fld1: _15.0.0,fld2: 11851251109887993441_usize,fld3: _2,fld4: 2333746513986721145_u64 };
Call(_13 = fn2(_10, _15.0.0, Field::<i128>(Variant(_18, 1), 3), _10), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_6 = _3 ^ _10;
_11 = RET as u16;
match _1 {
0 => bb8,
1 => bb5,
2 => bb7,
387943281791107981 => bb10,
_ => bb4
}
}
bb10 = {
place!(Field::<u64>(Variant(_18, 1), 4)) = 15525085515069333733_u64 >> _8;
_3 = _10;
_11 = 20451_u16 - 3122_u16;
place!(Field::<bool>(Variant(_18, 1), 0)) = !false;
place!(Field::<i128>(Variant(_18, 1), 3)) = _2 & _2;
_7 = 7_usize as i64;
place!(Field::<char>(Variant(_18, 1), 1)) = _15.0.0;
_20.1 = Field::<bool>(Variant(_18, 1), 0);
_17 = _15.0.0 as isize;
RET = 18_i8 | 7_i8;
_2 = !Field::<i128>(Variant(_18, 1), 3);
place!(Field::<bool>(Variant(_18, 1), 0)) = !_20.1;
RET = 4_i8;
_8 = (-31620_i16) as isize;
place!(Field::<usize>(Variant(_18, 1), 2)) = 6_usize | 7_usize;
place!(Field::<bool>(Variant(_18, 1), 0)) = !_20.1;
match RET {
0 => bb5,
1 => bb7,
2 => bb6,
4 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_3 = (-30950_i16) as isize;
_6 = 508486505_u32 as isize;
place!(Field::<char>(Variant(_18, 1), 1)) = _15.0.0;
RET = !(-26_i8);
RET = 86_i8 + 91_i8;
_3 = _5 - _5;
RET = !112_i8;
_19 = [634252705_u32,2085844365_u32];
_19 = [3645108385_u32,3568149329_u32];
_1 = _4;
_11 = 35753_u16 << _3;
_17 = -_3;
RET = 67_i8 + 17_i8;
_9 = !_5;
_2 = Field::<i128>(Variant(_18, 1), 3);
_19 = [3891947208_u32,3402992198_u32];
_7 = 234603416102958451640384752166029986345_u128 as i64;
_2 = Field::<i128>(Variant(_18, 1), 3) << _10;
_17 = Field::<bool>(Variant(_18, 1), 0) as isize;
_6 = _10 ^ _10;
_11 = 208_u8 as u16;
_1 = RET as i64;
place!(Field::<char>(Variant(_18, 1), 1)) = _15.0.0;
place!(Field::<u64>(Variant(_18, 1), 4)) = 217870225670018891614649073138256298412_u128 as u64;
Goto(bb13)
}
bb13 = {
place!(Field::<u64>(Variant(_18, 1), 4)) = 15792468743719616240_u64;
_24 = _18;
_18 = Adt22::Variant1 { fld0: _20.1,fld1: _15.0.0,fld2: Field::<usize>(Variant(_24, 1), 2),fld3: _2,fld4: Field::<u64>(Variant(_24, 1), 4) };
place!(Field::<u64>(Variant(_18, 1), 4)) = Field::<u64>(Variant(_24, 1), 4);
place!(Field::<u64>(Variant(_18, 1), 4)) = !Field::<u64>(Variant(_24, 1), 4);
_24 = _18;
_26.fld1 = Field::<char>(Variant(_24, 1), 1);
_26.fld0.0 = Field::<char>(Variant(_18, 1), 1);
place!(Field::<usize>(Variant(_18, 1), 2)) = 148798845466526261190740374832247249478_u128 as usize;
_10 = -_6;
Goto(bb14)
}
bb14 = {
_26.fld5 = (-1996094151_i32) << _3;
place!(Field::<i128>(Variant(_24, 1), 3)) = RET as i128;
_6 = Field::<usize>(Variant(_24, 1), 2) as isize;
_1 = Field::<i128>(Variant(_18, 1), 3) as i64;
_26.fld2 = [RET,RET,RET];
SetDiscriminant(_18, 1);
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(1_usize, 7_usize, Move(_7), 5_usize, Move(_5), 2_usize, Move(_2), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(1_usize, 4_usize, Move(_4), 1_usize, Move(_1), 32_usize, _32, 32_usize, _32), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: isize,mut _2: char,mut _3: i128,mut _4: isize) -> *mut Adt48 {
mir! {
type RET = *mut Adt48;
let _5: isize;
let _6: u16;
let _7: f64;
let _8: i64;
let _9: char;
let _10: &'static i128;
let _11: char;
let _12: [i32; 4];
let _13: (u32,);
let _14: (i32, Adt22, *mut &'static f64, f64);
let _15: f64;
let _16: Adt34;
let _17: char;
let _18: [u128; 7];
let _19: (&'static (&'static f64,), Adt34);
let _20: u16;
let _21: i16;
let _22: char;
let _23: Adt22;
let _24: f64;
let _25: bool;
let _26: &'static f64;
let _27: *mut [u128; 6];
let _28: *mut &'static f64;
let _29: bool;
let _30: &'static i32;
let _31: isize;
let _32: bool;
let _33: (&'static bool, (i128,));
let _34: (&'static *mut u8, bool);
let _35: (f32,);
let _36: &'static (&'static f64,);
let _37: isize;
let _38: char;
let _39: isize;
let _40: bool;
let _41: u64;
let _42: (f32,);
let _43: [u64; 8];
let _44: isize;
let _45: [isize; 3];
let _46: f32;
let _47: isize;
let _48: [i8; 7];
let _49: u128;
let _50: [u8; 5];
let _51: i32;
let _52: &'static (&'static f64,);
let _53: (u32,);
let _54: bool;
let _55: isize;
let _56: usize;
let _57: i8;
let _58: Adt44;
let _59: i8;
let _60: u8;
let _61: [usize; 1];
let _62: u64;
let _63: &'static usize;
let _64: *const (&'static (&'static f64,), Adt34);
let _65: bool;
let _66: [u128; 7];
let _67: [usize; 8];
let _68: isize;
let _69: [u64; 8];
let _70: f32;
let _71: u128;
let _72: f32;
let _73: isize;
let _74: ((char, Adt27), isize);
let _75: i8;
let _76: ((char, Adt27), isize);
let _77: (&'static bool, (i128,));
let _78: isize;
let _79: u16;
let _80: isize;
let _81: [isize; 3];
let _82: [i16; 5];
let _83: isize;
let _84: *mut u8;
let _85: (u32,);
let _86: u16;
let _87: isize;
let _88: *mut u8;
let _89: *mut &'static f64;
let _90: &'static (f32,);
let _91: u128;
let _92: Adt44;
let _93: char;
let _94: &'static (f32,);
let _95: [u8; 5];
let _96: u16;
let _97: isize;
let _98: Adt48;
let _99: i128;
let _100: u128;
let _101: (*mut &'static u8, f64, i16);
let _102: &'static usize;
let _103: i128;
let _104: ();
let _105: ();
{
_4 = _1;
Goto(bb1)
}
bb1 = {
_4 = _1;
_1 = 67_u8 as isize;
_3 = (-11573747901544509960427678534969873932_i128) + (-25591647283680645766451168511462226714_i128);
_4 = -_1;
_1 = _4;
_6 = 10927_u16 * 32771_u16;
_9 = _2;
_1 = _4 | _4;
_8 = -2566805169843116096_i64;
_7 = _6 as f64;
_2 = _9;
_6 = 3982_u16 >> _3;
Goto(bb2)
}
bb2 = {
_11 = _2;
_7 = 15380_i16 as f64;
_10 = &_3;
_10 = &(*_10);
_1 = _2 as isize;
_11 = _9;
_5 = -_1;
_4 = !_5;
_7 = _1 as f64;
_11 = _2;
_14.0 = -(-2004849946_i32);
_14.0 = (-313572977_i32) * 1619239470_i32;
_3 = 151318286377741380922702346672174752940_i128 & (-56714421096236105286889236505616426564_i128);
_13.0 = !477581930_u32;
_7 = (-24340_i16) as f64;
_13.0 = _14.0 as u32;
_6 = true as u16;
Call(_14 = fn3(), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_13 = (3466652556_u32,);
_10 = &place!(Field::<i128>(Variant(_14.1, 1), 3));
_15 = (-108_i8) as f64;
_13.0 = 226_u8 as u32;
_15 = _14.3 - _14.3;
_14.3 = _15;
_4 = -_1;
_11 = _9;
_10 = &place!(Field::<i128>(Variant(_14.1, 1), 3));
_13 = (2252687589_u32,);
_19.1 = Adt34::Variant1 { fld0: _6,fld1: _2,fld2: _5,fld3: Field::<usize>(Variant(_14.1, 1), 2),fld4: Field::<u64>(Variant(_14.1, 1), 4) };
place!(Field::<u16>(Variant(_19.1, 1), 0)) = !_6;
_6 = Field::<u16>(Variant(_19.1, 1), 0);
_18 = [492655493257774656798101178654939581_u128,170772840693956319025304163013596381343_u128,332720112613257486578767614660938816639_u128,276365299683938817255948957136809260184_u128,283836503251204199654678274765524925744_u128,263222409768365279550718156152203012028_u128,304344689676637143995380415429398698068_u128];
_19.1 = Adt34::Variant1 { fld0: _6,fld1: Field::<char>(Variant(_14.1, 1), 1),fld2: _1,fld3: Field::<usize>(Variant(_14.1, 1), 2),fld4: Field::<u64>(Variant(_14.1, 1), 4) };
_5 = _4;
_18 = [203424846895942763601752465015188198210_u128,266691123185293558862326265865294935005_u128,173984826209418508906950497727865394621_u128,53419619450193895315384037279401913485_u128,246068925420892337938326354542329975880_u128,338257570430293733853180421373140914449_u128,154887960023491443045646203990095520823_u128];
place!(Field::<u64>(Variant(_14.1, 1), 4)) = Field::<u64>(Variant(_19.1, 1), 4) & Field::<u64>(Variant(_19.1, 1), 4);
_14.1 = Adt22::Variant0 { fld0: Field::<usize>(Variant(_19.1, 1), 3),fld1: _6 };
_21 = -(-8242_i16);
_7 = _14.0 as f64;
match _13.0 {
0 => bb2,
2252687589 => bb5,
_ => bb4
}
}
bb4 = {
_11 = _2;
_7 = 15380_i16 as f64;
_10 = &_3;
_10 = &(*_10);
_1 = _2 as isize;
_11 = _9;
_5 = -_1;
_4 = !_5;
_7 = _1 as f64;
_11 = _2;
_14.0 = -(-2004849946_i32);
_14.0 = (-313572977_i32) * 1619239470_i32;
_3 = 151318286377741380922702346672174752940_i128 & (-56714421096236105286889236505616426564_i128);
_13.0 = !477581930_u32;
_7 = (-24340_i16) as f64;
_13.0 = _14.0 as u32;
_6 = true as u16;
Call(_14 = fn3(), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
place!(Field::<u64>(Variant(_19.1, 1), 4)) = _13.0 as u64;
_17 = Field::<char>(Variant(_19.1, 1), 1);
_8 = 3319444281064151914_i64;
place!(Field::<u16>(Variant(_19.1, 1), 0)) = _6;
_16 = Move(_19.1);
_15 = _14.3;
_1 = _14.0 as isize;
_6 = Field::<u16>(Variant(_14.1, 0), 1);
SetDiscriminant(_14.1, 0);
_2 = _9;
place!(Field::<char>(Variant(_16, 1), 1)) = _2;
_23 = Adt22::Variant1 { fld0: true,fld1: _11,fld2: Field::<usize>(Variant(_16, 1), 3),fld3: _3,fld4: Field::<u64>(Variant(_16, 1), 4) };
_14.1 = Adt22::Variant1 { fld0: false,fld1: Field::<char>(Variant(_16, 1), 1),fld2: Field::<usize>(Variant(_16, 1), 3),fld3: _3,fld4: Field::<u64>(Variant(_16, 1), 4) };
place!(Field::<usize>(Variant(_14.1, 1), 2)) = !Field::<usize>(Variant(_16, 1), 3);
_10 = &place!(Field::<i128>(Variant(_23, 1), 3));
_12 = [_14.0,_14.0,_14.0,_14.0];
place!(Field::<i128>(Variant(_23, 1), 3)) = Field::<i128>(Variant(_14.1, 1), 3) & Field::<i128>(Variant(_14.1, 1), 3);
_6 = Field::<u16>(Variant(_16, 1), 0) >> _5;
place!(Field::<bool>(Variant(_14.1, 1), 0)) = true;
_8 = (-2918075639579581657_i64) >> Field::<usize>(Variant(_14.1, 1), 2);
_18 = [311625450402026015201779315308224337866_u128,66976719096271400255524684634811479084_u128,108253265891632790800212612916414531343_u128,207262479156475868831365774485871089248_u128,189633139390850010690749038995476911911_u128,318885991507943557275573685072059252011_u128,212708322158649671148942995719849562753_u128];
_3 = Field::<i128>(Variant(_23, 1), 3) * Field::<i128>(Variant(_14.1, 1), 3);
place!(Field::<usize>(Variant(_14.1, 1), 2)) = Field::<usize>(Variant(_16, 1), 3);
_23 = Adt22::Variant1 { fld0: Field::<bool>(Variant(_14.1, 1), 0),fld1: Field::<char>(Variant(_14.1, 1), 1),fld2: Field::<usize>(Variant(_14.1, 1), 2),fld3: _3,fld4: Field::<u64>(Variant(_14.1, 1), 4) };
_3 = -Field::<i128>(Variant(_14.1, 1), 3);
_11 = Field::<char>(Variant(_14.1, 1), 1);
Goto(bb6)
}
bb6 = {
place!(Field::<char>(Variant(_23, 1), 1)) = _9;
_18 = [17586709389422444088646628633331654795_u128,308191555931927013186980210448300953090_u128,70368941682601853433195064984363322023_u128,34017521389670593258595572810825227269_u128,277445860840245450891939097648824916964_u128,307104865605257501714720885361373790329_u128,31170248941258733136960225694997982521_u128];
_23 = Adt22::Variant0 { fld0: Field::<usize>(Variant(_16, 1), 3),fld1: _6 };
_24 = _6 as f64;
SetDiscriminant(_23, 1);
place!(Field::<i128>(Variant(_23, 1), 3)) = _3;
place!(Field::<usize>(Variant(_23, 1), 2)) = Field::<usize>(Variant(_16, 1), 3);
_26 = &_7;
_19.1 = Move(_16);
_14.3 = 294598395896040684018866024671477933845_u128 as f64;
_17 = _2;
_8 = 5113616277132350736_i64 ^ 8688279662538589019_i64;
_10 = &place!(Field::<i128>(Variant(_14.1, 1), 3));
_16 = Move(_19.1);
_4 = _1;
Call(_28 = fn19(Move(_14), Move(_26), Move(_16), Field::<usize>(Variant(_23, 1), 2), _5, Field::<usize>(Variant(_23, 1), 2)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_14.2 = core::ptr::addr_of_mut!(_26);
_4 = _5 - _1;
_14.1 = Adt22::Variant1 { fld0: false,fld1: _2,fld2: Field::<usize>(Variant(_23, 1), 2),fld3: Field::<i128>(Variant(_23, 1), 3),fld4: 14761091321002270678_u64 };
_4 = !_1;
_26 = &_15;
_9 = _2;
place!(Field::<i128>(Variant(_23, 1), 3)) = 22546251859734555703383701469846826631_u128 as i128;
_3 = Field::<i128>(Variant(_14.1, 1), 3) << Field::<usize>(Variant(_23, 1), 2);
place!(Field::<bool>(Variant(_23, 1), 0)) = true;
place!(Field::<bool>(Variant(_14.1, 1), 0)) = Field::<bool>(Variant(_23, 1), 0);
_22 = _17;
_5 = 98304415987225402447145369124828616576_u128 as isize;
_9 = Field::<char>(Variant(_14.1, 1), 1);
_30 = &_14.0;
_15 = -_7;
_14.3 = _15 * _24;
_15 = -_24;
_5 = _8 as isize;
_14.3 = _15;
place!(Field::<char>(Variant(_23, 1), 1)) = Field::<char>(Variant(_14.1, 1), 1);
_23 = Adt22::Variant0 { fld0: Field::<usize>(Variant(_14.1, 1), 2),fld1: _6 };
_3 = !Field::<i128>(Variant(_14.1, 1), 3);
_14.3 = _7 * _7;
_23 = Adt22::Variant0 { fld0: Field::<usize>(Variant(_14.1, 1), 2),fld1: _6 };
match _13.0 {
0 => bb4,
1 => bb6,
2252687589 => bb8,
_ => bb3
}
}
bb8 = {
_19.1 = Adt34::Variant1 { fld0: Field::<u16>(Variant(_23, 0), 1),fld1: _22,fld2: _4,fld3: Field::<usize>(Variant(_23, 0), 0),fld4: 16442051011389324260_u64 };
place!(Field::<u64>(Variant(_14.1, 1), 4)) = 8933100148514270695_u64;
_29 = Field::<bool>(Variant(_14.1, 1), 0);
match Field::<u64>(Variant(_14.1, 1), 4) {
0 => bb6,
1 => bb9,
2 => bb10,
3 => bb11,
8933100148514270695 => bb13,
_ => bb12
}
}
bb9 = {
_13 = (3466652556_u32,);
_10 = &place!(Field::<i128>(Variant(_14.1, 1), 3));
_15 = (-108_i8) as f64;
_13.0 = 226_u8 as u32;
_15 = _14.3 - _14.3;
_14.3 = _15;
_4 = -_1;
_11 = _9;
_10 = &place!(Field::<i128>(Variant(_14.1, 1), 3));
_13 = (2252687589_u32,);
_19.1 = Adt34::Variant1 { fld0: _6,fld1: _2,fld2: _5,fld3: Field::<usize>(Variant(_14.1, 1), 2),fld4: Field::<u64>(Variant(_14.1, 1), 4) };
place!(Field::<u16>(Variant(_19.1, 1), 0)) = !_6;
_6 = Field::<u16>(Variant(_19.1, 1), 0);
_18 = [492655493257774656798101178654939581_u128,170772840693956319025304163013596381343_u128,332720112613257486578767614660938816639_u128,276365299683938817255948957136809260184_u128,283836503251204199654678274765524925744_u128,263222409768365279550718156152203012028_u128,304344689676637143995380415429398698068_u128];
_19.1 = Adt34::Variant1 { fld0: _6,fld1: Field::<char>(Variant(_14.1, 1), 1),fld2: _1,fld3: Field::<usize>(Variant(_14.1, 1), 2),fld4: Field::<u64>(Variant(_14.1, 1), 4) };
_5 = _4;
_18 = [203424846895942763601752465015188198210_u128,266691123185293558862326265865294935005_u128,173984826209418508906950497727865394621_u128,53419619450193895315384037279401913485_u128,246068925420892337938326354542329975880_u128,338257570430293733853180421373140914449_u128,154887960023491443045646203990095520823_u128];
place!(Field::<u64>(Variant(_14.1, 1), 4)) = Field::<u64>(Variant(_19.1, 1), 4) & Field::<u64>(Variant(_19.1, 1), 4);
_14.1 = Adt22::Variant0 { fld0: Field::<usize>(Variant(_19.1, 1), 3),fld1: _6 };
_21 = -(-8242_i16);
_7 = _14.0 as f64;
match _13.0 {
0 => bb2,
2252687589 => bb5,
_ => bb4
}
}
bb10 = {
place!(Field::<char>(Variant(_23, 1), 1)) = _9;
_18 = [17586709389422444088646628633331654795_u128,308191555931927013186980210448300953090_u128,70368941682601853433195064984363322023_u128,34017521389670593258595572810825227269_u128,277445860840245450891939097648824916964_u128,307104865605257501714720885361373790329_u128,31170248941258733136960225694997982521_u128];
_23 = Adt22::Variant0 { fld0: Field::<usize>(Variant(_16, 1), 3),fld1: _6 };
_24 = _6 as f64;
SetDiscriminant(_23, 1);
place!(Field::<i128>(Variant(_23, 1), 3)) = _3;
place!(Field::<usize>(Variant(_23, 1), 2)) = Field::<usize>(Variant(_16, 1), 3);
_26 = &_7;
_19.1 = Move(_16);
_14.3 = 294598395896040684018866024671477933845_u128 as f64;
_17 = _2;
_8 = 5113616277132350736_i64 ^ 8688279662538589019_i64;
_10 = &place!(Field::<i128>(Variant(_14.1, 1), 3));
_16 = Move(_19.1);
_4 = _1;
Call(_28 = fn19(Move(_14), Move(_26), Move(_16), Field::<usize>(Variant(_23, 1), 2), _5, Field::<usize>(Variant(_23, 1), 2)), ReturnTo(bb7), UnwindUnreachable())
}
bb11 = {
_11 = _2;
_7 = 15380_i16 as f64;
_10 = &_3;
_10 = &(*_10);
_1 = _2 as isize;
_11 = _9;
_5 = -_1;
_4 = !_5;
_7 = _1 as f64;
_11 = _2;
_14.0 = -(-2004849946_i32);
_14.0 = (-313572977_i32) * 1619239470_i32;
_3 = 151318286377741380922702346672174752940_i128 & (-56714421096236105286889236505616426564_i128);
_13.0 = !477581930_u32;
_7 = (-24340_i16) as f64;
_13.0 = _14.0 as u32;
_6 = true as u16;
Call(_14 = fn3(), ReturnTo(bb3), UnwindUnreachable())
}
bb12 = {
_11 = _2;
_7 = 15380_i16 as f64;
_10 = &_3;
_10 = &(*_10);
_1 = _2 as isize;
_11 = _9;
_5 = -_1;
_4 = !_5;
_7 = _1 as f64;
_11 = _2;
_14.0 = -(-2004849946_i32);
_14.0 = (-313572977_i32) * 1619239470_i32;
_3 = 151318286377741380922702346672174752940_i128 & (-56714421096236105286889236505616426564_i128);
_13.0 = !477581930_u32;
_7 = (-24340_i16) as f64;
_13.0 = _14.0 as u32;
_6 = true as u16;
Call(_14 = fn3(), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_14 = (1015710405_i32, _23, Move(_28), _24);
_31 = _5;
_13 = (3628565102_u32,);
place!(Field::<u16>(Variant(_14.1, 0), 1)) = !_6;
_9 = Field::<char>(Variant(_19.1, 1), 1);
_25 = _29;
_33.0 = &_29;
_14.3 = _15 - _24;
_18 = [47424587388279252575107965815760804264_u128,26324420093084450897748838612585521133_u128,234579439991662048437063636343632680055_u128,264928389260548415146518976608731284687_u128,187371112025994898798092842157429654018_u128,123879549386806767204582853744483191450_u128,243782116556498684662530409631917233370_u128];
_12 = [_14.0,_14.0,_14.0,_14.0];
_14.3 = Field::<usize>(Variant(_14.1, 0), 0) as f64;
_13 = (903578997_u32,);
_10 = &_33.1.0;
SetDiscriminant(_23, 1);
_28 = Move(_14.2);
_17 = _11;
_28 = core::ptr::addr_of_mut!(_26);
_33.1.0 = _3 * _3;
_29 = _14.0 >= _14.0;
_10 = &place!(Field::<i128>(Variant(_23, 1), 3));
_8 = 754376923015991728_i64 << Field::<usize>(Variant(_14.1, 0), 0);
_25 = !_29;
_30 = &_14.0;
_10 = &_33.1.0;
_12 = [(*_30),(*_30),(*_30),(*_30)];
match _14.0 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
5 => bb19,
6 => bb20,
1015710405 => bb22,
_ => bb21
}
}
bb14 = {
_11 = _2;
_7 = 15380_i16 as f64;
_10 = &_3;
_10 = &(*_10);
_1 = _2 as isize;
_11 = _9;
_5 = -_1;
_4 = !_5;
_7 = _1 as f64;
_11 = _2;
_14.0 = -(-2004849946_i32);
_14.0 = (-313572977_i32) * 1619239470_i32;
_3 = 151318286377741380922702346672174752940_i128 & (-56714421096236105286889236505616426564_i128);
_13.0 = !477581930_u32;
_7 = (-24340_i16) as f64;
_13.0 = _14.0 as u32;
_6 = true as u16;
Call(_14 = fn3(), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_11 = _2;
_7 = 15380_i16 as f64;
_10 = &_3;
_10 = &(*_10);
_1 = _2 as isize;
_11 = _9;
_5 = -_1;
_4 = !_5;
_7 = _1 as f64;
_11 = _2;
_14.0 = -(-2004849946_i32);
_14.0 = (-313572977_i32) * 1619239470_i32;
_3 = 151318286377741380922702346672174752940_i128 & (-56714421096236105286889236505616426564_i128);
_13.0 = !477581930_u32;
_7 = (-24340_i16) as f64;
_13.0 = _14.0 as u32;
_6 = true as u16;
Call(_14 = fn3(), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
place!(Field::<char>(Variant(_23, 1), 1)) = _9;
_18 = [17586709389422444088646628633331654795_u128,308191555931927013186980210448300953090_u128,70368941682601853433195064984363322023_u128,34017521389670593258595572810825227269_u128,277445860840245450891939097648824916964_u128,307104865605257501714720885361373790329_u128,31170248941258733136960225694997982521_u128];
_23 = Adt22::Variant0 { fld0: Field::<usize>(Variant(_16, 1), 3),fld1: _6 };
_24 = _6 as f64;
SetDiscriminant(_23, 1);
place!(Field::<i128>(Variant(_23, 1), 3)) = _3;
place!(Field::<usize>(Variant(_23, 1), 2)) = Field::<usize>(Variant(_16, 1), 3);
_26 = &_7;
_19.1 = Move(_16);
_14.3 = 294598395896040684018866024671477933845_u128 as f64;
_17 = _2;
_8 = 5113616277132350736_i64 ^ 8688279662538589019_i64;
_10 = &place!(Field::<i128>(Variant(_14.1, 1), 3));
_16 = Move(_19.1);
_4 = _1;
Call(_28 = fn19(Move(_14), Move(_26), Move(_16), Field::<usize>(Variant(_23, 1), 2), _5, Field::<usize>(Variant(_23, 1), 2)), ReturnTo(bb7), UnwindUnreachable())
}
bb17 = {
_13 = (3466652556_u32,);
_10 = &place!(Field::<i128>(Variant(_14.1, 1), 3));
_15 = (-108_i8) as f64;
_13.0 = 226_u8 as u32;
_15 = _14.3 - _14.3;
_14.3 = _15;
_4 = -_1;
_11 = _9;
_10 = &place!(Field::<i128>(Variant(_14.1, 1), 3));
_13 = (2252687589_u32,);
_19.1 = Adt34::Variant1 { fld0: _6,fld1: _2,fld2: _5,fld3: Field::<usize>(Variant(_14.1, 1), 2),fld4: Field::<u64>(Variant(_14.1, 1), 4) };
place!(Field::<u16>(Variant(_19.1, 1), 0)) = !_6;
_6 = Field::<u16>(Variant(_19.1, 1), 0);
_18 = [492655493257774656798101178654939581_u128,170772840693956319025304163013596381343_u128,332720112613257486578767614660938816639_u128,276365299683938817255948957136809260184_u128,283836503251204199654678274765524925744_u128,263222409768365279550718156152203012028_u128,304344689676637143995380415429398698068_u128];
_19.1 = Adt34::Variant1 { fld0: _6,fld1: Field::<char>(Variant(_14.1, 1), 1),fld2: _1,fld3: Field::<usize>(Variant(_14.1, 1), 2),fld4: Field::<u64>(Variant(_14.1, 1), 4) };
_5 = _4;
_18 = [203424846895942763601752465015188198210_u128,266691123185293558862326265865294935005_u128,173984826209418508906950497727865394621_u128,53419619450193895315384037279401913485_u128,246068925420892337938326354542329975880_u128,338257570430293733853180421373140914449_u128,154887960023491443045646203990095520823_u128];
place!(Field::<u64>(Variant(_14.1, 1), 4)) = Field::<u64>(Variant(_19.1, 1), 4) & Field::<u64>(Variant(_19.1, 1), 4);
_14.1 = Adt22::Variant0 { fld0: Field::<usize>(Variant(_19.1, 1), 3),fld1: _6 };
_21 = -(-8242_i16);
_7 = _14.0 as f64;
match _13.0 {
0 => bb2,
2252687589 => bb5,
_ => bb4
}
}
bb18 = {
_19.1 = Adt34::Variant1 { fld0: Field::<u16>(Variant(_23, 0), 1),fld1: _22,fld2: _4,fld3: Field::<usize>(Variant(_23, 0), 0),fld4: 16442051011389324260_u64 };
place!(Field::<u64>(Variant(_14.1, 1), 4)) = 8933100148514270695_u64;
_29 = Field::<bool>(Variant(_14.1, 1), 0);
match Field::<u64>(Variant(_14.1, 1), 4) {
0 => bb6,
1 => bb9,
2 => bb10,
3 => bb11,
8933100148514270695 => bb13,
_ => bb12
}
}
bb19 = {
_4 = _1;
_1 = 67_u8 as isize;
_3 = (-11573747901544509960427678534969873932_i128) + (-25591647283680645766451168511462226714_i128);
_4 = -_1;
_1 = _4;
_6 = 10927_u16 * 32771_u16;
_9 = _2;
_1 = _4 | _4;
_8 = -2566805169843116096_i64;
_7 = _6 as f64;
_2 = _9;
_6 = 3982_u16 >> _3;
Goto(bb2)
}
bb20 = {
_11 = _2;
_7 = 15380_i16 as f64;
_10 = &_3;
_10 = &(*_10);
_1 = _2 as isize;
_11 = _9;
_5 = -_1;
_4 = !_5;
_7 = _1 as f64;
_11 = _2;
_14.0 = -(-2004849946_i32);
_14.0 = (-313572977_i32) * 1619239470_i32;
_3 = 151318286377741380922702346672174752940_i128 & (-56714421096236105286889236505616426564_i128);
_13.0 = !477581930_u32;
_7 = (-24340_i16) as f64;
_13.0 = _14.0 as u32;
_6 = true as u16;
Call(_14 = fn3(), ReturnTo(bb3), UnwindUnreachable())
}
bb21 = {
place!(Field::<u64>(Variant(_19.1, 1), 4)) = _13.0 as u64;
_17 = Field::<char>(Variant(_19.1, 1), 1);
_8 = 3319444281064151914_i64;
place!(Field::<u16>(Variant(_19.1, 1), 0)) = _6;
_16 = Move(_19.1);
_15 = _14.3;
_1 = _14.0 as isize;
_6 = Field::<u16>(Variant(_14.1, 0), 1);
SetDiscriminant(_14.1, 0);
_2 = _9;
place!(Field::<char>(Variant(_16, 1), 1)) = _2;
_23 = Adt22::Variant1 { fld0: true,fld1: _11,fld2: Field::<usize>(Variant(_16, 1), 3),fld3: _3,fld4: Field::<u64>(Variant(_16, 1), 4) };
_14.1 = Adt22::Variant1 { fld0: false,fld1: Field::<char>(Variant(_16, 1), 1),fld2: Field::<usize>(Variant(_16, 1), 3),fld3: _3,fld4: Field::<u64>(Variant(_16, 1), 4) };
place!(Field::<usize>(Variant(_14.1, 1), 2)) = !Field::<usize>(Variant(_16, 1), 3);
_10 = &place!(Field::<i128>(Variant(_23, 1), 3));
_12 = [_14.0,_14.0,_14.0,_14.0];
place!(Field::<i128>(Variant(_23, 1), 3)) = Field::<i128>(Variant(_14.1, 1), 3) & Field::<i128>(Variant(_14.1, 1), 3);
_6 = Field::<u16>(Variant(_16, 1), 0) >> _5;
place!(Field::<bool>(Variant(_14.1, 1), 0)) = true;
_8 = (-2918075639579581657_i64) >> Field::<usize>(Variant(_14.1, 1), 2);
_18 = [311625450402026015201779315308224337866_u128,66976719096271400255524684634811479084_u128,108253265891632790800212612916414531343_u128,207262479156475868831365774485871089248_u128,189633139390850010690749038995476911911_u128,318885991507943557275573685072059252011_u128,212708322158649671148942995719849562753_u128];
_3 = Field::<i128>(Variant(_23, 1), 3) * Field::<i128>(Variant(_14.1, 1), 3);
place!(Field::<usize>(Variant(_14.1, 1), 2)) = Field::<usize>(Variant(_16, 1), 3);
_23 = Adt22::Variant1 { fld0: Field::<bool>(Variant(_14.1, 1), 0),fld1: Field::<char>(Variant(_14.1, 1), 1),fld2: Field::<usize>(Variant(_14.1, 1), 2),fld3: _3,fld4: Field::<u64>(Variant(_14.1, 1), 4) };
_3 = -Field::<i128>(Variant(_14.1, 1), 3);
_11 = Field::<char>(Variant(_14.1, 1), 1);
Goto(bb6)
}
bb22 = {
_33.0 = &_29;
_7 = 16488461440644778682_u64 as f64;
_23 = Adt22::Variant0 { fld0: Field::<usize>(Variant(_19.1, 1), 3),fld1: _6 };
place!(Field::<u16>(Variant(_14.1, 0), 1)) = _6;
SetDiscriminant(_23, 1);
_11 = _22;
place!(Field::<char>(Variant(_19.1, 1), 1)) = _11;
place!(Field::<char>(Variant(_23, 1), 1)) = _22;
_32 = !_25;
_37 = _31 | _5;
_14.1 = Adt22::Variant0 { fld0: Field::<usize>(Variant(_19.1, 1), 3),fld1: Field::<u16>(Variant(_19.1, 1), 0) };
place!(Field::<char>(Variant(_23, 1), 1)) = _17;
_11 = Field::<char>(Variant(_23, 1), 1);
place!(Field::<usize>(Variant(_14.1, 0), 0)) = !Field::<usize>(Variant(_19.1, 1), 3);
match (*_30) {
1015710405 => bb24,
_ => bb23
}
}
bb23 = {
_11 = _2;
_7 = 15380_i16 as f64;
_10 = &_3;
_10 = &(*_10);
_1 = _2 as isize;
_11 = _9;
_5 = -_1;
_4 = !_5;
_7 = _1 as f64;
_11 = _2;
_14.0 = -(-2004849946_i32);
_14.0 = (-313572977_i32) * 1619239470_i32;
_3 = 151318286377741380922702346672174752940_i128 & (-56714421096236105286889236505616426564_i128);
_13.0 = !477581930_u32;
_7 = (-24340_i16) as f64;
_13.0 = _14.0 as u32;
_6 = true as u16;
Call(_14 = fn3(), ReturnTo(bb3), UnwindUnreachable())
}
bb24 = {
_24 = _14.3;
place!(Field::<u64>(Variant(_23, 1), 4)) = !13306694831177063567_u64;
_35.0 = (*_30) as f32;
_9 = _2;
_13 = (3076183215_u32,);
_26 = &_14.3;
_35.0 = (*_30) as f32;
_13 = (1412732748_u32,);
(*_28) = &_14.3;
_3 = _33.1.0 * _33.1.0;
_18 = [41731217643501115706178360956463770590_u128,18590351215559988985446848327360014423_u128,335551512966155982902438347898978953413_u128,126082629309998685375632194703186701181_u128,196136911921673345409338306828069813343_u128,222166075023915012307962773850227414888_u128,281950696051113569571225335164368666137_u128];
SetDiscriminant(_14.1, 0);
_40 = !_25;
_14.2 = core::ptr::addr_of_mut!((*_28));
Goto(bb25)
}
bb25 = {
_41 = Field::<u64>(Variant(_23, 1), 4);
place!(Field::<u64>(Variant(_19.1, 1), 4)) = !Field::<u64>(Variant(_23, 1), 4);
place!(Field::<usize>(Variant(_23, 1), 2)) = 270587113223901236904446111465515772195_u128 as usize;
_4 = Field::<isize>(Variant(_19.1, 1), 2);
place!(Field::<bool>(Variant(_23, 1), 0)) = _32;
_22 = Field::<char>(Variant(_23, 1), 1);
_17 = Field::<char>(Variant(_23, 1), 1);
_1 = -Field::<isize>(Variant(_19.1, 1), 2);
SetDiscriminant(_19.1, 0);
_18 = [6963335122472402745331730831423497053_u128,100190665418978800310588474276961874781_u128,89621767664475763117824362860087008755_u128,255199992871525798492196726371490526279_u128,294402626167709418271547538892563689399_u128,112906522067551479217532685575062856815_u128,337396008497588426781538974087746115066_u128];
_42.0 = -_35.0;
_11 = _22;
_20 = _6;
_7 = -(*_26);
Goto(bb26)
}
bb26 = {
_30 = &(*_30);
place!(Field::<i128>(Variant(_19.1, 0), 7)) = (-56_i8) as i128;
(*_28) = &_15;
_35 = (_42.0,);
_11 = Field::<char>(Variant(_23, 1), 1);
_14.1 = Adt22::Variant1 { fld0: _25,fld1: Field::<char>(Variant(_23, 1), 1),fld2: Field::<usize>(Variant(_23, 1), 2),fld3: (*_10),fld4: Field::<u64>(Variant(_23, 1), 4) };
_33.1.0 = _35.0 as i128;
_34.1 = !_32;
match _14.0 {
1015710405 => bb28,
_ => bb27
}
}
bb27 = {
place!(Field::<char>(Variant(_23, 1), 1)) = _9;
_18 = [17586709389422444088646628633331654795_u128,308191555931927013186980210448300953090_u128,70368941682601853433195064984363322023_u128,34017521389670593258595572810825227269_u128,277445860840245450891939097648824916964_u128,307104865605257501714720885361373790329_u128,31170248941258733136960225694997982521_u128];
_23 = Adt22::Variant0 { fld0: Field::<usize>(Variant(_16, 1), 3),fld1: _6 };
_24 = _6 as f64;
SetDiscriminant(_23, 1);
place!(Field::<i128>(Variant(_23, 1), 3)) = _3;
place!(Field::<usize>(Variant(_23, 1), 2)) = Field::<usize>(Variant(_16, 1), 3);
_26 = &_7;
_19.1 = Move(_16);
_14.3 = 294598395896040684018866024671477933845_u128 as f64;
_17 = _2;
_8 = 5113616277132350736_i64 ^ 8688279662538589019_i64;
_10 = &place!(Field::<i128>(Variant(_14.1, 1), 3));
_16 = Move(_19.1);
_4 = _1;
Call(_28 = fn19(Move(_14), Move(_26), Move(_16), Field::<usize>(Variant(_23, 1), 2), _5, Field::<usize>(Variant(_23, 1), 2)), ReturnTo(bb7), UnwindUnreachable())
}
bb28 = {
_14.0 = 1489483774_i32 | 1741935982_i32;
_13.0 = 2763962486_u32;
_46 = _35.0 - _42.0;
_23 = _14.1;
_29 = !_32;
_15 = -_24;
_44 = -_4;
(*_28) = &_15;
place!(Field::<f64>(Variant(_19.1, 0), 6)) = _42.0 as f64;
_14.2 = core::ptr::addr_of_mut!((*_28));
place!(Field::<bool>(Variant(_14.1, 1), 0)) = _14.3 != _7;
_30 = &_14.0;
place!(Field::<isize>(Variant(_19.1, 0), 2)) = _24 as isize;
_32 = _29;
_9 = _17;
_6 = _20 & _20;
_40 = Field::<bool>(Variant(_23, 1), 0);
_35.0 = _46;
place!(Field::<bool>(Variant(_19.1, 0), 0)) = Field::<bool>(Variant(_23, 1), 0);
_29 = _34.1;
_9 = Field::<char>(Variant(_23, 1), 1);
_39 = _37;
_33.1 = (_3,);
_43 = [Field::<u64>(Variant(_14.1, 1), 4),_41,Field::<u64>(Variant(_23, 1), 4),Field::<u64>(Variant(_23, 1), 4),Field::<u64>(Variant(_23, 1), 4),Field::<u64>(Variant(_23, 1), 4),Field::<u64>(Variant(_14.1, 1), 4),_41];
SetDiscriminant(_23, 1);
match _13.0 {
0 => bb10,
2763962486 => bb29,
_ => bb19
}
}
bb29 = {
_51 = (*_30);
_18 = [264947041258312907225926393175882586385_u128,261168401853280905078258399720889392688_u128,267882686917818320749982462967172997702_u128,9668513255421335330188412318999031641_u128,118949725140156556494602530748658943231_u128,336285482427869212370758330778869170794_u128,164372923081054236271990544336197612800_u128];
_2 = _9;
_32 = _34.1;
_31 = Field::<isize>(Variant(_19.1, 0), 2) + Field::<isize>(Variant(_19.1, 0), 2);
place!(Field::<(f32,)>(Variant(_19.1, 0), 4)) = (_35.0,);
_17 = _2;
place!(Field::<char>(Variant(_23, 1), 1)) = _22;
_15 = -_7;
_12 = [(*_30),_14.0,(*_30),_51];
_49 = !82250469090922619131884958627832763679_u128;
place!(Field::<usize>(Variant(_23, 1), 2)) = _33.1.0 as usize;
_40 = Field::<bool>(Variant(_19.1, 0), 0);
_57 = (-72_i8);
place!(Field::<i128>(Variant(_23, 1), 3)) = !_3;
Call(_57 = core::intrinsics::bswap(101_i8), ReturnTo(bb30), UnwindUnreachable())
}
bb30 = {
_5 = _57 as isize;
_35 = (Field::<(f32,)>(Variant(_19.1, 0), 4).0,);
_50 = [155_u8,34_u8,36_u8,181_u8,78_u8];
_54 = _32 & _34.1;
_41 = Field::<u64>(Variant(_14.1, 1), 4) - Field::<u64>(Variant(_14.1, 1), 4);
_13.0 = 1455088837_u32;
_9 = Field::<char>(Variant(_14.1, 1), 1);
Goto(bb31)
}
bb31 = {
_35 = (_42.0,);
_56 = Field::<usize>(Variant(_23, 1), 2);
_23 = Adt22::Variant1 { fld0: _29,fld1: _17,fld2: _56,fld3: _33.1.0,fld4: Field::<u64>(Variant(_14.1, 1), 4) };
_21 = Field::<u64>(Variant(_23, 1), 4) as i16;
_35 = (_42.0,);
SetDiscriminant(_23, 0);
_53.0 = _33.1.0 as u32;
place!(Field::<u16>(Variant(_23, 0), 1)) = !_6;
_40 = Field::<bool>(Variant(_19.1, 0), 0);
_19.1 = Adt34::Variant1 { fld0: _6,fld1: _9,fld2: _31,fld3: _56,fld4: Field::<u64>(Variant(_14.1, 1), 4) };
SetDiscriminant(_19.1, 2);
Goto(bb32)
}
bb32 = {
place!(Field::<i128>(Variant(_14.1, 1), 3)) = _33.1.0;
(*_28) = &_24;
_29 = _32;
_63 = &place!(Field::<usize>(Variant(_23, 0), 0));
_12 = [(*_30),(*_30),_14.0,_14.0];
_61 = [_56];
_33.0 = &_25;
_58 = Adt44::Variant1 { fld0: _42,fld1: _56,fld2: _57 };
_33.1 = (Field::<i128>(Variant(_14.1, 1), 3),);
_38 = _17;
_29 = _34.1;
_5 = !_31;
_5 = 234_u8 as isize;
_38 = _17;
_11 = _17;
_51 = _14.0 << _41;
place!(Field::<char>(Variant(_14.1, 1), 1)) = _22;
_68 = _31 + _31;
_37 = _38 as isize;
_14.3 = _49 as f64;
_20 = _9 as u16;
_62 = _49 as u64;
_59 = Field::<i8>(Variant(_58, 1), 2) << _68;
match _13.0 {
0 => bb23,
1 => bb15,
2 => bb24,
3 => bb13,
1455088837 => bb33,
_ => bb27
}
}
bb33 = {
_15 = (*_26) + (*_26);
_59 = -_57;
_60 = Field::<u64>(Variant(_14.1, 1), 4) as u8;
place!(Field::<i128>(Variant(_14.1, 1), 3)) = _3;
_11 = _9;
_53.0 = _13.0;
(*_28) = &_14.3;
place!(Field::<u16>(Variant(_23, 0), 1)) = _6 ^ _20;
_56 = Field::<(f32,)>(Variant(_58, 1), 0).0 as usize;
_13.0 = !_53.0;
_45 = [_31,_31,_31];
_31 = _13.0 as isize;
_63 = &(*_63);
_42 = (Field::<(f32,)>(Variant(_58, 1), 0).0,);
place!(Field::<i8>(Variant(_58, 1), 2)) = _46 as i8;
_59 = !Field::<i8>(Variant(_58, 1), 2);
_24 = -_15;
_55 = _7 as isize;
_28 = core::ptr::addr_of_mut!((*_28));
_33.0 = &_40;
_65 = _32 & _34.1;
_58 = Adt44::Variant1 { fld0: _42,fld1: _56,fld2: _59 };
_23 = _14.1;
_55 = _68 & _68;
_35 = (_42.0,);
_67 = [_56,Field::<usize>(Variant(_58, 1), 1),_56,_56,_56,Field::<usize>(Variant(_58, 1), 1),_56,_56];
_51 = Field::<i128>(Variant(_14.1, 1), 3) as i32;
Goto(bb34)
}
bb34 = {
_6 = _49 as u16;
_33.1.0 = Field::<i128>(Variant(_14.1, 1), 3) >> _59;
_72 = -_35.0;
place!(Field::<usize>(Variant(_14.1, 1), 2)) = _56 - Field::<usize>(Variant(_58, 1), 1);
(*_28) = &_15;
_32 = _40;
_69 = [Field::<u64>(Variant(_23, 1), 4),Field::<u64>(Variant(_14.1, 1), 4),Field::<u64>(Variant(_14.1, 1), 4),_41,_41,_41,_41,Field::<u64>(Variant(_23, 1), 4)];
_47 = -_68;
_5 = _47 ^ _55;
_73 = _49 as isize;
_57 = -_59;
_45 = [_55,_55,_55];
_33.0 = &place!(Field::<bool>(Variant(_23, 1), 0));
_40 = _25;
_14.3 = _15;
_29 = _34.1;
_10 = &_3;
Goto(bb35)
}
bb35 = {
_42 = (_72,);
_42.0 = _72;
_51 = _49 as i32;
_2 = Field::<char>(Variant(_14.1, 1), 1);
_12 = [_14.0,(*_30),(*_30),(*_30)];
_5 = _35.0 as isize;
_15 = _24 + _7;
_23 = Adt22::Variant0 { fld0: _56,fld1: _6 };
_39 = _55 - _47;
_39 = -_47;
place!(Field::<(char, Adt27)>(Variant(_19.1, 2), 0)).0 = _9;
_31 = _68 * _5;
_77.0 = &_29;
_76.0.0 = Field::<char>(Variant(_14.1, 1), 1);
match _53.0 {
0 => bb14,
1 => bb16,
2 => bb36,
3 => bb37,
4 => bb38,
1455088837 => bb40,
_ => bb39
}
}
bb36 = {
_11 = _2;
_7 = 15380_i16 as f64;
_10 = &_3;
_10 = &(*_10);
_1 = _2 as isize;
_11 = _9;
_5 = -_1;
_4 = !_5;
_7 = _1 as f64;
_11 = _2;
_14.0 = -(-2004849946_i32);
_14.0 = (-313572977_i32) * 1619239470_i32;
_3 = 151318286377741380922702346672174752940_i128 & (-56714421096236105286889236505616426564_i128);
_13.0 = !477581930_u32;
_7 = (-24340_i16) as f64;
_13.0 = _14.0 as u32;
_6 = true as u16;
Call(_14 = fn3(), ReturnTo(bb3), UnwindUnreachable())
}
bb37 = {
place!(Field::<u64>(Variant(_19.1, 1), 4)) = _13.0 as u64;
_17 = Field::<char>(Variant(_19.1, 1), 1);
_8 = 3319444281064151914_i64;
place!(Field::<u16>(Variant(_19.1, 1), 0)) = _6;
_16 = Move(_19.1);
_15 = _14.3;
_1 = _14.0 as isize;
_6 = Field::<u16>(Variant(_14.1, 0), 1);
SetDiscriminant(_14.1, 0);
_2 = _9;
place!(Field::<char>(Variant(_16, 1), 1)) = _2;
_23 = Adt22::Variant1 { fld0: true,fld1: _11,fld2: Field::<usize>(Variant(_16, 1), 3),fld3: _3,fld4: Field::<u64>(Variant(_16, 1), 4) };
_14.1 = Adt22::Variant1 { fld0: false,fld1: Field::<char>(Variant(_16, 1), 1),fld2: Field::<usize>(Variant(_16, 1), 3),fld3: _3,fld4: Field::<u64>(Variant(_16, 1), 4) };
place!(Field::<usize>(Variant(_14.1, 1), 2)) = !Field::<usize>(Variant(_16, 1), 3);
_10 = &place!(Field::<i128>(Variant(_23, 1), 3));
_12 = [_14.0,_14.0,_14.0,_14.0];
place!(Field::<i128>(Variant(_23, 1), 3)) = Field::<i128>(Variant(_14.1, 1), 3) & Field::<i128>(Variant(_14.1, 1), 3);
_6 = Field::<u16>(Variant(_16, 1), 0) >> _5;
place!(Field::<bool>(Variant(_14.1, 1), 0)) = true;
_8 = (-2918075639579581657_i64) >> Field::<usize>(Variant(_14.1, 1), 2);
_18 = [311625450402026015201779315308224337866_u128,66976719096271400255524684634811479084_u128,108253265891632790800212612916414531343_u128,207262479156475868831365774485871089248_u128,189633139390850010690749038995476911911_u128,318885991507943557275573685072059252011_u128,212708322158649671148942995719849562753_u128];
_3 = Field::<i128>(Variant(_23, 1), 3) * Field::<i128>(Variant(_14.1, 1), 3);
place!(Field::<usize>(Variant(_14.1, 1), 2)) = Field::<usize>(Variant(_16, 1), 3);
_23 = Adt22::Variant1 { fld0: Field::<bool>(Variant(_14.1, 1), 0),fld1: Field::<char>(Variant(_14.1, 1), 1),fld2: Field::<usize>(Variant(_14.1, 1), 2),fld3: _3,fld4: Field::<u64>(Variant(_14.1, 1), 4) };
_3 = -Field::<i128>(Variant(_14.1, 1), 3);
_11 = Field::<char>(Variant(_14.1, 1), 1);
Goto(bb6)
}
bb38 = {
_11 = _2;
_7 = 15380_i16 as f64;
_10 = &_3;
_10 = &(*_10);
_1 = _2 as isize;
_11 = _9;
_5 = -_1;
_4 = !_5;
_7 = _1 as f64;
_11 = _2;
_14.0 = -(-2004849946_i32);
_14.0 = (-313572977_i32) * 1619239470_i32;
_3 = 151318286377741380922702346672174752940_i128 & (-56714421096236105286889236505616426564_i128);
_13.0 = !477581930_u32;
_7 = (-24340_i16) as f64;
_13.0 = _14.0 as u32;
_6 = true as u16;
Call(_14 = fn3(), ReturnTo(bb3), UnwindUnreachable())
}
bb39 = {
_11 = _2;
_7 = 15380_i16 as f64;
_10 = &_3;
_10 = &(*_10);
_1 = _2 as isize;
_11 = _9;
_5 = -_1;
_4 = !_5;
_7 = _1 as f64;
_11 = _2;
_14.0 = -(-2004849946_i32);
_14.0 = (-313572977_i32) * 1619239470_i32;
_3 = 151318286377741380922702346672174752940_i128 & (-56714421096236105286889236505616426564_i128);
_13.0 = !477581930_u32;
_7 = (-24340_i16) as f64;
_13.0 = _14.0 as u32;
_6 = true as u16;
Call(_14 = fn3(), ReturnTo(bb3), UnwindUnreachable())
}
bb40 = {
_75 = Field::<char>(Variant(_14.1, 1), 1) as i8;
_61 = [Field::<usize>(Variant(_14.1, 1), 2)];
_42.0 = _72;
place!(Field::<(f32,)>(Variant(_58, 1), 0)).0 = _35.0;
_22 = _38;
_42 = (_35.0,);
_22 = _11;
_15 = -_14.3;
_14.3 = _8 as f64;
_33.1.0 = _3 + Field::<i128>(Variant(_14.1, 1), 3);
_51 = (*_30) & (*_30);
(*_28) = &_14.3;
SetDiscriminant(_58, 0);
_51 = (*_30);
place!(Field::<bool>(Variant(_14.1, 1), 0)) = _40;
_82 = [_21,_21,_21,_21,_21];
place!(Field::<i8>(Variant(_58, 0), 3)) = -_57;
_7 = -(*_26);
_79 = _49 as u16;
match _53.0 {
0 => bb11,
1455088837 => bb41,
_ => bb18
}
}
bb41 = {
_11 = Field::<(char, Adt27)>(Variant(_19.1, 2), 0).0;
_66 = [_49,_49,_49,_49,_49,_49,_49];
place!(Field::<u16>(Variant(_23, 0), 1)) = !_20;
_42.0 = -_46;
_42.0 = _46;
_45 = [_55,_68,_39];
_59 = Field::<i8>(Variant(_58, 0), 3) * _57;
place!(Field::<u8>(Variant(_58, 0), 5)) = _33.1.0 as u8;
place!(Field::<(i128,)>(Variant(_58, 0), 0)) = ((*_10),);
_61 = [Field::<usize>(Variant(_14.1, 1), 2)];
_83 = _39 - _31;
_85 = (_13.0,);
_5 = Field::<usize>(Variant(_14.1, 1), 2) as isize;
_39 = _83 * _83;
_74.0.0 = _2;
Goto(bb42)
}
bb42 = {
place!(Field::<i16>(Variant(_58, 0), 4)) = _21;
_88 = core::ptr::addr_of_mut!(_60);
match _53.0 {
0 => bb5,
1 => bb43,
1455088837 => bb45,
_ => bb44
}
}
bb43 = {
_4 = _1;
_1 = 67_u8 as isize;
_3 = (-11573747901544509960427678534969873932_i128) + (-25591647283680645766451168511462226714_i128);
_4 = -_1;
_1 = _4;
_6 = 10927_u16 * 32771_u16;
_9 = _2;
_1 = _4 | _4;
_8 = -2566805169843116096_i64;
_7 = _6 as f64;
_2 = _9;
_6 = 3982_u16 >> _3;
Goto(bb2)
}
bb44 = {
_14 = (1015710405_i32, _23, Move(_28), _24);
_31 = _5;
_13 = (3628565102_u32,);
place!(Field::<u16>(Variant(_14.1, 0), 1)) = !_6;
_9 = Field::<char>(Variant(_19.1, 1), 1);
_25 = _29;
_33.0 = &_29;
_14.3 = _15 - _24;
_18 = [47424587388279252575107965815760804264_u128,26324420093084450897748838612585521133_u128,234579439991662048437063636343632680055_u128,264928389260548415146518976608731284687_u128,187371112025994898798092842157429654018_u128,123879549386806767204582853744483191450_u128,243782116556498684662530409631917233370_u128];
_12 = [_14.0,_14.0,_14.0,_14.0];
_14.3 = Field::<usize>(Variant(_14.1, 0), 0) as f64;
_13 = (903578997_u32,);
_10 = &_33.1.0;
SetDiscriminant(_23, 1);
_28 = Move(_14.2);
_17 = _11;
_28 = core::ptr::addr_of_mut!(_26);
_33.1.0 = _3 * _3;
_29 = _14.0 >= _14.0;
_10 = &place!(Field::<i128>(Variant(_23, 1), 3));
_8 = 754376923015991728_i64 << Field::<usize>(Variant(_14.1, 0), 0);
_25 = !_29;
_30 = &_14.0;
_10 = &_33.1.0;
_12 = [(*_30),(*_30),(*_30),(*_30)];
match _14.0 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
5 => bb19,
6 => bb20,
1015710405 => bb22,
_ => bb21
}
}
bb45 = {
_34.0 = &_88;
_10 = &place!(Field::<(i128,)>(Variant(_58, 0), 0)).0;
_60 = Field::<u8>(Variant(_58, 0), 5) & Field::<u8>(Variant(_58, 0), 5);
_76.0.1 = Adt27::Variant0 { fld0: (*_30),fld1: _42.0,fld2: _24 };
_77.1.0 = -_33.1.0;
_57 = _85.0 as i8;
_83 = _55 + _31;
_60 = Field::<u8>(Variant(_58, 0), 5) ^ Field::<u8>(Variant(_58, 0), 5);
_13.0 = _53.0;
place!(Field::<*mut [usize; 8]>(Variant(_58, 0), 1)) = core::ptr::addr_of_mut!(_67);
_63 = &place!(Field::<usize>(Variant(_23, 0), 0));
_45 = [_39,_5,_31];
place!(Field::<(char, Adt27)>(Variant(_19.1, 2), 0)).0 = _17;
place!(Field::<f32>(Variant(_76.0.1, 0), 1)) = _46;
_16 = Adt34::Variant2 { fld0: Move(_76.0) };
_42.0 = -_46;
_14.1 = _23;
match _13.0 {
0 => bb23,
1455088837 => bb46,
_ => bb39
}
}
bb46 = {
_21 = !Field::<i16>(Variant(_58, 0), 4);
_85.0 = _53.0 - _13.0;
_30 = &_51;
_20 = Field::<i32>(Variant(Field::<(char, Adt27)>(Variant(_16, 2), 0).1, 0), 0) as u16;
_78 = _83 | _55;
_42.0 = _72 + _35.0;
_33.0 = &_29;
_19.1 = Adt34::Variant2 { fld0: Move(Field::<(char, Adt27)>(Variant(_16, 2), 0)) };
place!(Field::<(char, Adt27)>(Variant(_16, 2), 0)) = (_22, Move(Field::<(char, Adt27)>(Variant(_19.1, 2), 0).1));
_13.0 = _85.0 % _53.0;
_61 = [_56];
_95 = [Field::<u8>(Variant(_58, 0), 5),(*_88),(*_88),Field::<u8>(Variant(_58, 0), 5),_60];
place!(Field::<u8>(Variant(_58, 0), 5)) = !(*_88);
Goto(bb47)
}
bb47 = {
_43 = [_41,_41,_62,_41,_41,_41,_62,_41];
place!(Field::<i8>(Variant(_58, 0), 3)) = (*_63) as i8;
place!(Field::<i32>(Variant(place!(Field::<(char, Adt27)>(Variant(_16, 2), 0)).1, 0), 0)) = (*_30) * _51;
_82 = [_21,_21,Field::<i16>(Variant(_58, 0), 4),_21,_21];
place!(Field::<(char, Adt27)>(Variant(_19.1, 2), 0)) = (_22, Move(Field::<(char, Adt27)>(Variant(_16, 2), 0).1));
_91 = _49 + _49;
_85.0 = !_13.0;
_75 = Field::<i8>(Variant(_58, 0), 3) - _59;
_76.0 = Move(Field::<(char, Adt27)>(Variant(_19.1, 2), 0));
place!(Field::<(char, Adt27)>(Variant(_16, 2), 0)).0 = _9;
place!(Field::<[usize; 8]>(Variant(_58, 0), 7)) = [Field::<usize>(Variant(_23, 0), 0),(*_63),Field::<usize>(Variant(_14.1, 0), 0),_56,_56,(*_63),Field::<usize>(Variant(_14.1, 0), 0),_56];
_42.0 = Field::<f32>(Variant(_76.0.1, 0), 1) * _46;
_93 = Field::<(char, Adt27)>(Variant(_16, 2), 0).0;
Goto(bb48)
}
bb48 = {
_64 = core::ptr::addr_of!(_19);
_19.1 = Adt34::Variant2 { fld0: Move(_76.0) };
_34.1 = !_29;
_73 = _75 as isize;
match _53.0 {
0 => bb16,
1455088837 => bb49,
_ => bb31
}
}
bb49 = {
place!(Field::<i8>(Variant(_58, 0), 3)) = _59;
place!(Field::<(char, Adt27)>(Variant(_16, 2), 0)).0 = _93;
place!(Field::<*mut u8>(Variant(_58, 0), 2)) = core::ptr::addr_of_mut!((*_88));
_85 = _13;
_30 = &place!(Field::<i32>(Variant(place!(Field::<(char, Adt27)>(Variant((*_64).1, 2), 0)).1, 0), 0));
_101.2 = Field::<i16>(Variant(_58, 0), 4);
_85.0 = _14.0 as u32;
(*_88) = !Field::<u8>(Variant(_58, 0), 5);
RET = core::ptr::addr_of_mut!(_98);
_39 = _83;
place!(Field::<f64>(Variant(place!(Field::<(char, Adt27)>(Variant(_19.1, 2), 0)).1, 0), 2)) = (*_26) * _7;
_9 = _38;
_30 = &place!(Field::<i32>(Variant(place!(Field::<(char, Adt27)>(Variant((*_64).1, 2), 0)).1, 0), 0));
_69 = [_62,_41,_41,_41,_62,_41,_41,_41];
(*RET) = Adt48::Variant2 { fld0: _77.1.0,fld1: _41,fld2: Field::<f32>(Variant(Field::<(char, Adt27)>(Variant(_19.1, 2), 0).1, 0), 1),fld3: Move(Field::<(char, Adt27)>(Variant(_19.1, 2), 0)) };
_76.0.0 = Field::<(char, Adt27)>(Variant((*RET), 2), 3).0;
place!(Field::<i128>(Variant((*RET), 2), 0)) = _8 as i128;
_33 = (Move(_77.0), Field::<(i128,)>(Variant(_58, 0), 0));
place!(Field::<i16>(Variant(_58, 0), 4)) = !_21;
Goto(bb50)
}
bb50 = {
Call(_104 = dump_var(2_usize, 31_usize, Move(_31), 56_usize, Move(_56), 53_usize, Move(_53), 11_usize, Move(_11)), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
Call(_104 = dump_var(2_usize, 85_usize, Move(_85), 83_usize, Move(_83), 38_usize, Move(_38), 66_usize, Move(_66)), ReturnTo(bb52), UnwindUnreachable())
}
bb52 = {
Call(_104 = dump_var(2_usize, 54_usize, Move(_54), 3_usize, Move(_3), 37_usize, Move(_37), 78_usize, Move(_78)), ReturnTo(bb53), UnwindUnreachable())
}
bb53 = {
Call(_104 = dump_var(2_usize, 40_usize, Move(_40), 8_usize, Move(_8), 95_usize, Move(_95), 50_usize, Move(_50)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_104 = dump_var(2_usize, 9_usize, Move(_9), 43_usize, Move(_43), 5_usize, Move(_5), 18_usize, Move(_18)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_104 = dump_var(2_usize, 4_usize, Move(_4), 2_usize, Move(_2), 45_usize, Move(_45), 60_usize, Move(_60)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_104 = dump_var(2_usize, 17_usize, Move(_17), 39_usize, Move(_39), 51_usize, Move(_51), 20_usize, Move(_20)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3() -> (i32, Adt22, *mut &'static f64, f64) {
mir! {
type RET = (i32, Adt22, *mut &'static f64, f64);
let _1: *mut i16;
let _2: (char, i8, (&'static bool, (i128,)));
let _3: (u32,);
let _4: (&'static f64,);
let _5: &'static f64;
let _6: [isize; 3];
let _7: f32;
let _8: *const f64;
let _9: *const f64;
let _10: [i8; 7];
let _11: u128;
let _12: isize;
let _13: isize;
let _14: *mut [isize; 3];
let _15: (Adt27, *mut [usize; 8]);
let _16: f64;
let _17: [u64; 8];
let _18: f32;
let _19: *const *mut &'static f64;
let _20: i64;
let _21: [i16; 5];
let _22: (*mut &'static u8, f64, i16);
let _23: *mut [isize; 3];
let _24: [u64; 8];
let _25: Adt34;
let _26: Adt27;
let _27: ();
let _28: ();
{
RET.0 = 146607524679605283711529861427193379995_u128 as i32;
RET.0 = (-528003246_i32);
RET.1 = Adt22::Variant1 { fld0: false,fld1: '\u{8555f}',fld2: 6_usize,fld3: (-105512586756974036507669905484349949529_i128),fld4: 8200783119972571527_u64 };
RET.1 = Adt22::Variant0 { fld0: 5_usize,fld1: 33064_u16 };
place!(Field::<usize>(Variant(RET.1, 0), 0)) = 11365558421976226139_usize ^ 6_usize;
_2.0 = '\u{ebd80}';
_2.2.1 = ((-162182923771762225975965176258118462824_i128),);
RET.3 = (-111_i8) as f64;
place!(Field::<u16>(Variant(RET.1, 0), 1)) = !55612_u16;
_2.1 = 30_i8;
_3.0 = 2656858960_u32;
RET.1 = Adt22::Variant0 { fld0: 15669889090180267670_usize,fld1: 32247_u16 };
RET.1 = Adt22::Variant1 { fld0: true,fld1: _2.0,fld2: 6616085981925768838_usize,fld3: _2.2.1.0,fld4: 8713910890100397190_u64 };
place!(Field::<u64>(Variant(RET.1, 1), 4)) = 39167_u16 as u64;
_2.0 = Field::<char>(Variant(RET.1, 1), 1);
match RET.0 {
0 => bb1,
1 => bb2,
2 => bb3,
340282366920938463463374607431240208210 => bb5,
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
place!(Field::<usize>(Variant(RET.1, 1), 2)) = 4_usize | 0_usize;
_2.2.1 = (Field::<i128>(Variant(RET.1, 1), 3),);
RET.2 = core::ptr::addr_of_mut!(_4.0);
RET.1 = Adt22::Variant0 { fld0: 3775605834828803289_usize,fld1: 45285_u16 };
Goto(bb6)
}
bb6 = {
_3.0 = 2287340059_u32 & 3452221515_u32;
place!(Field::<u16>(Variant(RET.1, 0), 1)) = 209902406368560829577450163967814992237_u128 as u16;
_4.0 = &RET.3;
RET.0 = 322304659_i32;
RET.2 = core::ptr::addr_of_mut!(_4.0);
place!(Field::<u16>(Variant(RET.1, 0), 1)) = !42248_u16;
place!(Field::<usize>(Variant(RET.1, 0), 0)) = _2.2.1.0 as usize;
RET.0 = 1535732471_i32 + 1110894848_i32;
RET.2 = core::ptr::addr_of_mut!(_4.0);
RET.1 = Adt22::Variant1 { fld0: false,fld1: _2.0,fld2: 0_usize,fld3: _2.2.1.0,fld4: 5182198799087960522_u64 };
_2.2.0 = &place!(Field::<bool>(Variant(RET.1, 1), 0));
place!(Field::<char>(Variant(RET.1, 1), 1)) = _2.0;
RET.1 = Adt22::Variant1 { fld0: true,fld1: _2.0,fld2: 3865640353765693785_usize,fld3: _2.2.1.0,fld4: 12273647558355116878_u64 };
place!(Field::<u64>(Variant(RET.1, 1), 4)) = 9291313501659165818_u64 & 14671064211547274809_u64;
place!(Field::<i128>(Variant(RET.1, 1), 3)) = _2.2.1.0 & _2.2.1.0;
RET.1 = Adt22::Variant0 { fld0: 13146734851076958721_usize,fld1: 31771_u16 };
RET.0 = 1837151710_i32;
_2.0 = '\u{ed5f2}';
_6 = [(-62_isize),9223372036854775807_isize,9223372036854775807_isize];
_4.0 = &RET.3;
Goto(bb7)
}
bb7 = {
_4.0 = &RET.3;
place!(Field::<u16>(Variant(RET.1, 0), 1)) = _3.0 as u16;
_5 = &RET.3;
_2.0 = '\u{731b2}';
RET.0 = !(-1264839169_i32);
_9 = core::ptr::addr_of!((*_5));
place!(Field::<usize>(Variant(RET.1, 0), 0)) = !4326946058887987402_usize;
_4.0 = Move(_5);
place!(Field::<u16>(Variant(RET.1, 0), 1)) = !31898_u16;
place!(Field::<usize>(Variant(RET.1, 0), 0)) = 1_usize;
place!(Field::<usize>(Variant(RET.1, 0), 0)) = 17236241407349510178_usize - 10810488147190983461_usize;
RET.0 = 1375249376_i32;
RET.3 = _2.1 as f64;
_4.0 = &RET.3;
_11 = 294659001073805402762646946484220734526_u128;
RET.1 = Adt22::Variant1 { fld0: true,fld1: _2.0,fld2: 14351226783977166400_usize,fld3: _2.2.1.0,fld4: 12674670458398294450_u64 };
place!(Field::<usize>(Variant(RET.1, 1), 2)) = RET.3 as usize;
RET.0 = -1753597842_i32;
place!(Field::<i128>(Variant(RET.1, 1), 3)) = _3.0 as i128;
_8 = core::ptr::addr_of!(RET.3);
_2.2.0 = &place!(Field::<bool>(Variant(RET.1, 1), 0));
_8 = core::ptr::addr_of!(RET.3);
place!(Field::<bool>(Variant(RET.1, 1), 0)) = !true;
RET.2 = core::ptr::addr_of_mut!(_4.0);
_10 = [_2.1,_2.1,_2.1,_2.1,_2.1,_2.1,_2.1];
Call(_7 = fn4(Move(_4), Move(RET.2), _11, _2.0, RET.3, Move(_9), Field::<i128>(Variant(RET.1, 1), 3), Move(_8), _3.0, Field::<char>(Variant(RET.1, 1), 1)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_8 = core::ptr::addr_of!(_16);
RET.1 = Adt22::Variant0 { fld0: 9811336599560650438_usize,fld1: 19823_u16 };
RET.2 = core::ptr::addr_of_mut!(_5);
RET.1 = Adt22::Variant0 { fld0: 13740694537692536274_usize,fld1: 54049_u16 };
place!(Field::<usize>(Variant(RET.1, 0), 0)) = 5919_i16 as usize;
_14 = core::ptr::addr_of_mut!(_6);
_3 = (1606061687_u32,);
_2.2.1 = (117224261365061337104770054058500742418_i128,);
_18 = _7 + _7;
_13 = -(-9223372036854775808_isize);
_13 = _2.0 as isize;
(*_8) = -RET.3;
_16 = RET.3;
_14 = core::ptr::addr_of_mut!((*_14));
(*_14) = [_13,_13,_13];
(*_8) = RET.3 + RET.3;
place!(Field::<usize>(Variant(RET.1, 0), 0)) = 9658813403741556817_usize | 1_usize;
RET.0 = -1271688623_i32;
_14 = core::ptr::addr_of_mut!((*_14));
_12 = _13;
RET.1 = Adt22::Variant0 { fld0: 13846461984721367855_usize,fld1: 28174_u16 };
_11 = 220474713710521800669532207862253580314_u128;
_10 = [_2.1,_2.1,_2.1,_2.1,_2.1,_2.1,_2.1];
place!(Field::<usize>(Variant(RET.1, 0), 0)) = !12441602774358197211_usize;
_19 = core::ptr::addr_of!(RET.2);
match _11 {
220474713710521800669532207862253580314 => bb9,
_ => bb5
}
}
bb9 = {
RET.2 = core::ptr::addr_of_mut!(_5);
RET.1 = Adt22::Variant0 { fld0: 13930641563861796954_usize,fld1: 4225_u16 };
RET.1 = Adt22::Variant0 { fld0: 4614180297223387635_usize,fld1: 48532_u16 };
_10 = [_2.1,_2.1,_2.1,_2.1,_2.1,_2.1,_2.1];
RET.2 = core::ptr::addr_of_mut!(_5);
RET.1 = Adt22::Variant1 { fld0: true,fld1: _2.0,fld2: 15178875461135305417_usize,fld3: _2.2.1.0,fld4: 13313697507923179932_u64 };
_5 = &(*_8);
_4 = (Move(_5),);
_17 = [15768196286018161250_u64,3759147322581255462_u64,7882229973916706112_u64,12540652237969362742_u64,6068018793070612141_u64,17850635261707045771_u64,359153178604171571_u64,14926733532554969770_u64];
place!(Field::<usize>(Variant(RET.1, 1), 2)) = !3_usize;
RET.1 = Adt22::Variant1 { fld0: false,fld1: _2.0,fld2: 15227274217046744051_usize,fld3: _2.2.1.0,fld4: 11862586492305265923_u64 };
_4.0 = &RET.3;
(*_19) = core::ptr::addr_of_mut!(_4.0);
_7 = 132_u8 as f32;
_9 = Move(_8);
place!(Field::<u64>(Variant(RET.1, 1), 4)) = 14540175492353799917_u64;
Goto(bb10)
}
bb10 = {
place!(Field::<u64>(Variant(RET.1, 1), 4)) = 2064724963740996782_u64 ^ 5462862411080262971_u64;
_22.1 = -RET.3;
RET.3 = _16;
_3 = (2452757831_u32,);
place!(Field::<bool>(Variant(RET.1, 1), 0)) = !false;
(*_14) = [_13,_12,_12];
place!(Field::<usize>(Variant(RET.1, 1), 2)) = _18 as usize;
_2.1 = (-90_i8) << Field::<usize>(Variant(RET.1, 1), 2);
_12 = _13 * _13;
_20 = (-7514845742318177810_i64);
Goto(bb11)
}
bb11 = {
Call(_27 = dump_var(3_usize, 10_usize, Move(_10), 3_usize, Move(_3), 17_usize, Move(_17), 6_usize, Move(_6)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: (&'static f64,),mut _2: *mut &'static f64,mut _3: u128,mut _4: char,mut _5: f64,mut _6: *const f64,mut _7: i128,mut _8: *const f64,mut _9: u32,mut _10: char) -> f32 {
mir! {
type RET = f32;
let _11: bool;
let _12: [u128; 7];
let _13: i64;
let _14: f64;
let _15: [u16; 5];
let _16: (i128,);
let _17: char;
let _18: char;
let _19: isize;
let _20: u32;
let _21: i8;
let _22: f32;
let _23: [i8; 3];
let _24: [u8; 5];
let _25: bool;
let _26: &'static [u128; 7];
let _27: [usize; 8];
let _28: [i32; 4];
let _29: [u128; 7];
let _30: i16;
let _31: *mut [u128; 6];
let _32: bool;
let _33: *const usize;
let _34: &'static i32;
let _35: [isize; 3];
let _36: u128;
let _37: u32;
let _38: i64;
let _39: isize;
let _40: [u64; 8];
let _41: u128;
let _42: u32;
let _43: u8;
let _44: isize;
let _45: char;
let _46: i8;
let _47: i128;
let _48: (&'static *mut u8, bool);
let _49: ();
let _50: ();
{
_7 = (-133712884408918689234675118037357981014_i128) >> _9;
RET = 1594039653_i32 as f32;
_1.0 = &_5;
_3 = 302395971650582051860816659319959734679_u128;
RET = (-168621211_i32) as f32;
_10 = _4;
_3 = 181163112919398663987169022479708655333_u128 << _7;
_1.0 = &_5;
_4 = _10;
_1.0 = &_5;
_4 = _10;
_6 = core::ptr::addr_of!(_5);
_10 = _4;
_3 = 21540457186394105833657981683610505234_u128;
_1.0 = &(*_6);
RET = 32451_i16 as f32;
_3 = true as u128;
_4 = _10;
_2 = core::ptr::addr_of_mut!(_1.0);
_4 = _10;
RET = 8225084106494032724_i64 as f32;
(*_2) = &(*_6);
_6 = core::ptr::addr_of!(_5);
(*_2) = &(*_6);
_12 = [_3,_3,_3,_3,_3,_3,_3];
(*_2) = &(*_6);
_1.0 = &(*_6);
_8 = core::ptr::addr_of!((*_6));
(*_2) = &(*_8);
Goto(bb1)
}
bb1 = {
_4 = _10;
RET = (-9223372036854775808_isize) as f32;
_14 = -_5;
_1.0 = &_5;
_12 = [_3,_3,_3,_3,_3,_3,_3];
(*_2) = &_14;
(*_2) = &(*_6);
_13 = (-3623436786079712910_i64) ^ 4169100949051973131_i64;
_8 = core::ptr::addr_of!(_14);
_9 = 2333249277_u32;
_4 = _10;
_3 = 0_usize as u128;
_2 = core::ptr::addr_of_mut!((*_2));
_9 = true as u32;
Goto(bb2)
}
bb2 = {
_4 = _10;
_11 = (*_8) != (*_6);
_4 = _10;
_11 = true;
_10 = _4;
RET = _9 as f32;
_1.0 = &(*_8);
Goto(bb3)
}
bb3 = {
_14 = 11248357282313997808_u64 as f64;
_4 = _10;
_1.0 = &(*_6);
_6 = Move(_8);
_3 = !130777576432253606506667205222830842442_u128;
(*_2) = &_14;
_15 = [35216_u16,32113_u16,1898_u16,39994_u16,13081_u16];
_9 = !2314674395_u32;
_16 = (_7,);
(*_2) = &_5;
_2 = core::ptr::addr_of_mut!(_1.0);
_1.0 = &_5;
_1.0 = &_14;
RET = _3 as f32;
_4 = _10;
_11 = false;
_9 = 369226389_u32;
Goto(bb4)
}
bb4 = {
_18 = _10;
Call(_7 = core::intrinsics::bswap(_16.0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_8 = Move(_6);
RET = _9 as f32;
_13 = 1645904626658537700_i64 & (-5266684306580160299_i64);
_6 = Move(_8);
_12 = [_3,_3,_3,_3,_3,_3,_3];
_2 = core::ptr::addr_of_mut!((*_2));
_15 = [872_u16,20521_u16,55739_u16,11568_u16,2272_u16];
Call(_11 = fn5(Move(_2), _15, RET), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_16 = (_7,);
_1.0 = &_5;
_3 = _14 as u128;
_17 = _18;
_6 = core::ptr::addr_of!(_14);
_1.0 = &(*_6);
_2 = core::ptr::addr_of_mut!(_1.0);
_17 = _10;
Call(RET = core::intrinsics::transmute(_4), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_20 = !_9;
_22 = _3 as f32;
_18 = _17;
_1.0 = &_14;
_12 = [_3,_3,_3,_3,_3,_3,_3];
_1.0 = &_5;
_22 = RET + RET;
_9 = (-69_i8) as u32;
_16 = (_7,);
_21 = 82_i8;
_25 = _11;
_8 = core::ptr::addr_of!(_5);
Goto(bb8)
}
bb8 = {
RET = _22 + _22;
_4 = _17;
_26 = &_12;
(*_2) = &(*_6);
(*_2) = &_5;
_21 = 22_i8;
_7 = 10429425261903271828_usize as i128;
_16 = (_7,);
_19 = -67_isize;
_16.0 = _19 as i128;
_23 = [_21,_21,_21];
Goto(bb9)
}
bb9 = {
_12 = [_3,_3,_3,_3,_3,_3,_3];
_10 = _18;
_12 = [_3,_3,_3,_3,_3,_3,_3];
_11 = !_25;
(*_2) = &(*_8);
_16 = (_7,);
_6 = core::ptr::addr_of!((*_6));
_8 = Move(_6);
_15 = [60849_u16,64194_u16,50973_u16,24943_u16,35112_u16];
_6 = Move(_8);
_7 = _16.0;
_3 = 44542_u16 as u128;
_24 = [248_u8,209_u8,152_u8,65_u8,137_u8];
_13 = (-7135800816681171523_i64);
_9 = !_20;
_26 = &_12;
_27 = [5_usize,1_usize,0_usize,3_usize,9993446760878812918_usize,4_usize,6_usize,8846933350353518443_usize];
_16.0 = _7;
_6 = core::ptr::addr_of!(_14);
RET = _22 - _22;
_14 = -_5;
Call(_5 = core::intrinsics::fmaf64(_14, _14, _14), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_17 = _10;
_28 = [(-687489200_i32),1733808279_i32,(-2028749556_i32),945868785_i32];
(*_2) = &_14;
_16 = (_7,);
_13 = 5_usize as i64;
_14 = -_5;
_15 = [11187_u16,13454_u16,58346_u16,35297_u16,31643_u16];
_16 = (_7,);
_20 = _9;
_29 = _12;
_13 = _3 as i64;
_9 = _19 as u32;
_25 = _11;
_11 = _25;
_8 = Move(_6);
_10 = _17;
_7 = _18 as i128;
_25 = !_11;
_30 = 687857644626285888_u64 as i16;
_27 = [1_usize,0_usize,0_usize,7765005192997318780_usize,13536387702027761087_usize,12565326494527118429_usize,7014382974474734337_usize,9875274889841689469_usize];
_35 = [_19,_19,_19];
_18 = _4;
_18 = _17;
(*_2) = &_14;
_6 = Move(_8);
Goto(bb11)
}
bb11 = {
_5 = _14;
Call(_37 = fn7(Move(_2)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_22 = RET - RET;
_5 = _14;
_35 = [_19,_19,_19];
_7 = _10 as i128;
_18 = _17;
_12 = _29;
_11 = RET <= _22;
_26 = &_29;
_4 = _10;
_11 = _7 < _7;
_25 = !_11;
_23 = [_21,_21,_21];
_28 = [759094176_i32,(-115097068_i32),(-1818483518_i32),739400268_i32];
_13 = _5 as i64;
_37 = _30 as u32;
_32 = _11;
_22 = RET - RET;
_29 = _12;
_30 = (-29769_i16);
Goto(bb13)
}
bb13 = {
_9 = !_20;
_12 = [_3,_3,_3,_3,_3,_3,_3];
_41 = !_3;
Call(_33 = fn13(Move(_1), _17), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_1.0 = &_14;
_36 = _3 | _41;
_40 = [16010777274841416480_u64,1680294055865070070_u64,16727728408165309510_u64,7070068586267161619_u64,1344445521033058538_u64,3919063943210277393_u64,9417005957044537593_u64,3276278945261305563_u64];
_5 = -_14;
_17 = _4;
RET = -_22;
_11 = _32;
_9 = _37 & _37;
_18 = _4;
_23 = [_21,_21,_21];
_25 = RET >= _22;
RET = _22 - _22;
_39 = !_19;
_28 = [815550559_i32,912270047_i32,1970280083_i32,(-1196266377_i32)];
_1.0 = &_5;
Goto(bb15)
}
bb15 = {
Call(_49 = dump_var(4_usize, 37_usize, Move(_37), 4_usize, Move(_4), 21_usize, Move(_21), 35_usize, Move(_35)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_49 = dump_var(4_usize, 18_usize, Move(_18), 9_usize, Move(_9), 20_usize, Move(_20), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = dump_var(4_usize, 29_usize, Move(_29), 10_usize, Move(_10), 40_usize, Move(_40), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_49 = dump_var(4_usize, 13_usize, Move(_13), 3_usize, Move(_3), 50_usize, _50, 50_usize, _50), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: *mut &'static f64,mut _2: [u16; 5],mut _3: f32) -> bool {
mir! {
type RET = bool;
let _4: (Adt27, *mut [usize; 8]);
let _5: i128;
let _6: (Adt27, *mut [usize; 8]);
let _7: [i32; 4];
let _8: (*const (&'static (&'static f64,), Adt34),);
let _9: [isize; 3];
let _10: f32;
let _11: [isize; 4];
let _12: f64;
let _13: *const i8;
let _14: Adt44;
let _15: isize;
let _16: [isize; 4];
let _17: [usize; 1];
let _18: (&'static f64,);
let _19: [u32; 2];
let _20: *const (&'static (&'static f64,), Adt34);
let _21: i32;
let _22: f64;
let _23: [i128; 5];
let _24: u128;
let _25: *const Adt34;
let _26: isize;
let _27: u32;
let _28: [u8; 5];
let _29: &'static (f32,);
let _30: [u128; 7];
let _31: u8;
let _32: isize;
let _33: ();
let _34: ();
{
RET = !false;
RET = !false;
_2 = [28581_u16,50172_u16,9519_u16,5990_u16,10397_u16];
_3 = (-1_isize) as f32;
_2 = [46257_u16,24904_u16,403_u16,63115_u16,35392_u16];
RET = true | true;
RET = !false;
_3 = (-3361652636102050843_i64) as f32;
RET = _3 > _3;
_2 = [12365_u16,44261_u16,49422_u16,37260_u16,41568_u16];
_2 = [37002_u16,43757_u16,12462_u16,20398_u16,3190_u16];
RET = true;
_2 = [36395_u16,21281_u16,51790_u16,42421_u16,26110_u16];
Goto(bb1)
}
bb1 = {
_5 = (-124663550224402949709442980326672306892_i128);
RET = false;
_2 = [49993_u16,26159_u16,19995_u16,8409_u16,62101_u16];
_7 = [1138875285_i32,658718049_i32,1067901785_i32,63004219_i32];
RET = !false;
_2 = [41128_u16,1382_u16,23256_u16,25778_u16,20608_u16];
_7 = [12921507_i32,(-1841738879_i32),(-6264817_i32),323866278_i32];
_7 = [1246645694_i32,1995267856_i32,1587596041_i32,1050484602_i32];
RET = true & false;
RET = _5 <= _5;
_5 = !(-168803426754506633425686755415867045482_i128);
_2 = [10747_u16,2915_u16,25540_u16,28072_u16,6652_u16];
_3 = 60_i8 as f32;
RET = false;
_5 = (-67869607190059145251087790824838933037_i128);
_3 = 9223372036854775807_isize as f32;
_3 = 9223372036854775807_isize as f32;
_9 = [(-58_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_7 = [100817565_i32,(-1048503311_i32),1157204124_i32,(-2079216941_i32)];
RET = false;
RET = true;
RET = true;
RET = true;
_5 = 154906800262418716297611851567958096244_i128 + 158342317310009702575425612346131414045_i128;
RET = _3 <= _3;
Goto(bb2)
}
bb2 = {
_2 = [13235_u16,51493_u16,926_u16,29956_u16,17913_u16];
_5 = 69687820697062905784847610820667154630_i128 ^ 152358785176326529530331743427664351640_i128;
RET = false;
_7 = [(-1809818172_i32),1656481829_i32,(-1179874779_i32),(-1789320876_i32)];
RET = !true;
_5 = 32216874759469255839898548290229174763_i128 & 146724246405350427483542586259070275798_i128;
_2 = [15823_u16,10400_u16,11125_u16,34986_u16,45014_u16];
_7 = [564527538_i32,(-370793282_i32),(-1866245121_i32),1665486229_i32];
_5 = -(-47878607303673459396099663976833186242_i128);
_12 = 238695582_u32 as f64;
_6.0 = Adt27::Variant0 { fld0: (-1294916163_i32),fld1: _3,fld2: _12 };
_10 = (-82_i8) as f32;
RET = !false;
_3 = Field::<f32>(Variant(_6.0, 0), 1);
RET = false;
place!(Field::<i32>(Variant(_6.0, 0), 0)) = 1441648865_i32;
_10 = 13476416208727136341_u64 as f32;
place!(Field::<f64>(Variant(_6.0, 0), 2)) = _12;
_10 = _3;
_15 = 4_isize;
_9 = [_15,_15,_15];
place!(Field::<f32>(Variant(_6.0, 0), 1)) = Field::<f64>(Variant(_6.0, 0), 2) as f32;
_5 = 27925192095900668993215171381033737370_i128 * (-18741901136077433282316345559538478379_i128);
_11 = [_15,_15,_15,_15];
place!(Field::<f64>(Variant(_6.0, 0), 2)) = -_12;
Call(RET = fn6(Move(_1), Move(_6.0)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = [683381081_i32,193401921_i32,(-365856313_i32),636432712_i32];
RET = true | false;
_10 = 1965665043_u32 as f32;
_16 = [_15,_15,_15,_15];
_4.0 = Adt27::Variant0 { fld0: (-1033509775_i32),fld1: _10,fld2: _12 };
_4.0 = Adt27::Variant0 { fld0: 1072722040_i32,fld1: _3,fld2: _12 };
_18.0 = &_12;
_6.0 = Adt27::Variant0 { fld0: 1485738766_i32,fld1: Field::<f32>(Variant(_4.0, 0), 1),fld2: _12 };
_6.0 = Adt27::Variant0 { fld0: (-687144170_i32),fld1: Field::<f32>(Variant(_4.0, 0), 1),fld2: Field::<f64>(Variant(_4.0, 0), 2) };
_11 = [_15,_15,_15,_15];
match _15 {
0 => bb1,
1 => bb2,
2 => bb4,
4 => bb6,
_ => bb5
}
}
bb4 = {
_2 = [13235_u16,51493_u16,926_u16,29956_u16,17913_u16];
_5 = 69687820697062905784847610820667154630_i128 ^ 152358785176326529530331743427664351640_i128;
RET = false;
_7 = [(-1809818172_i32),1656481829_i32,(-1179874779_i32),(-1789320876_i32)];
RET = !true;
_5 = 32216874759469255839898548290229174763_i128 & 146724246405350427483542586259070275798_i128;
_2 = [15823_u16,10400_u16,11125_u16,34986_u16,45014_u16];
_7 = [564527538_i32,(-370793282_i32),(-1866245121_i32),1665486229_i32];
_5 = -(-47878607303673459396099663976833186242_i128);
_12 = 238695582_u32 as f64;
_6.0 = Adt27::Variant0 { fld0: (-1294916163_i32),fld1: _3,fld2: _12 };
_10 = (-82_i8) as f32;
RET = !false;
_3 = Field::<f32>(Variant(_6.0, 0), 1);
RET = false;
place!(Field::<i32>(Variant(_6.0, 0), 0)) = 1441648865_i32;
_10 = 13476416208727136341_u64 as f32;
place!(Field::<f64>(Variant(_6.0, 0), 2)) = _12;
_10 = _3;
_15 = 4_isize;
_9 = [_15,_15,_15];
place!(Field::<f32>(Variant(_6.0, 0), 1)) = Field::<f64>(Variant(_6.0, 0), 2) as f32;
_5 = 27925192095900668993215171381033737370_i128 * (-18741901136077433282316345559538478379_i128);
_11 = [_15,_15,_15,_15];
place!(Field::<f64>(Variant(_6.0, 0), 2)) = -_12;
Call(RET = fn6(Move(_1), Move(_6.0)), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_5 = (-124663550224402949709442980326672306892_i128);
RET = false;
_2 = [49993_u16,26159_u16,19995_u16,8409_u16,62101_u16];
_7 = [1138875285_i32,658718049_i32,1067901785_i32,63004219_i32];
RET = !false;
_2 = [41128_u16,1382_u16,23256_u16,25778_u16,20608_u16];
_7 = [12921507_i32,(-1841738879_i32),(-6264817_i32),323866278_i32];
_7 = [1246645694_i32,1995267856_i32,1587596041_i32,1050484602_i32];
RET = true & false;
RET = _5 <= _5;
_5 = !(-168803426754506633425686755415867045482_i128);
_2 = [10747_u16,2915_u16,25540_u16,28072_u16,6652_u16];
_3 = 60_i8 as f32;
RET = false;
_5 = (-67869607190059145251087790824838933037_i128);
_3 = 9223372036854775807_isize as f32;
_3 = 9223372036854775807_isize as f32;
_9 = [(-58_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_7 = [100817565_i32,(-1048503311_i32),1157204124_i32,(-2079216941_i32)];
RET = false;
RET = true;
RET = true;
RET = true;
_5 = 154906800262418716297611851567958096244_i128 + 158342317310009702575425612346131414045_i128;
RET = _3 <= _3;
Goto(bb2)
}
bb6 = {
_4.0 = Adt27::Variant0 { fld0: 736398641_i32,fld1: _3,fld2: _12 };
_7 = [(-1311416013_i32),(-298830138_i32),(-375180257_i32),(-1643754438_i32)];
_2 = [6711_u16,15762_u16,17833_u16,61229_u16,14379_u16];
_12 = 0_usize as f64;
_1 = core::ptr::addr_of_mut!(_18.0);
place!(Field::<i32>(Variant(_6.0, 0), 0)) = 239420823_i32;
SetDiscriminant(_6.0, 0);
RET = !true;
(*_1) = &_22;
_21 = !(-743330788_i32);
_2 = [3847_u16,6595_u16,5589_u16,31671_u16,62_u16];
(*_1) = &place!(Field::<f64>(Variant(_6.0, 0), 2));
_21 = 2069579548_i32 + (-709592324_i32);
(*_1) = &place!(Field::<f64>(Variant(_6.0, 0), 2));
_19 = [3219322063_u32,4118722569_u32];
place!(Field::<f32>(Variant(_4.0, 0), 1)) = _3 + _10;
(*_1) = &place!(Field::<f64>(Variant(_6.0, 0), 2));
place!(Field::<f32>(Variant(_4.0, 0), 1)) = 9865760845254437147_usize as f32;
_21 = (-1192416245_i32) - (-1229305911_i32);
_24 = 292438446642236683121012731187458757505_u128;
_1 = core::ptr::addr_of_mut!(_18.0);
RET = !false;
_3 = Field::<f32>(Variant(_4.0, 0), 1);
Goto(bb7)
}
bb7 = {
RET = true;
place!(Field::<f32>(Variant(_6.0, 0), 1)) = -_10;
_3 = -_10;
place!(Field::<f32>(Variant(_6.0, 0), 1)) = -Field::<f32>(Variant(_4.0, 0), 1);
_17 = [13358128111659064759_usize];
_3 = Field::<f32>(Variant(_4.0, 0), 1) - Field::<f32>(Variant(_4.0, 0), 1);
_18.0 = &place!(Field::<f64>(Variant(_4.0, 0), 2));
_24 = !125082808905953484390696464382694405928_u128;
place!(Field::<f64>(Variant(_6.0, 0), 2)) = -_12;
_9 = [_15,_15,_15];
_18.0 = &place!(Field::<f64>(Variant(_6.0, 0), 2));
match _15 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb4,
5 => bb8,
6 => bb9,
4 => bb11,
_ => bb10
}
}
bb8 = {
_7 = [683381081_i32,193401921_i32,(-365856313_i32),636432712_i32];
RET = true | false;
_10 = 1965665043_u32 as f32;
_16 = [_15,_15,_15,_15];
_4.0 = Adt27::Variant0 { fld0: (-1033509775_i32),fld1: _10,fld2: _12 };
_4.0 = Adt27::Variant0 { fld0: 1072722040_i32,fld1: _3,fld2: _12 };
_18.0 = &_12;
_6.0 = Adt27::Variant0 { fld0: 1485738766_i32,fld1: Field::<f32>(Variant(_4.0, 0), 1),fld2: _12 };
_6.0 = Adt27::Variant0 { fld0: (-687144170_i32),fld1: Field::<f32>(Variant(_4.0, 0), 1),fld2: Field::<f64>(Variant(_4.0, 0), 2) };
_11 = [_15,_15,_15,_15];
match _15 {
0 => bb1,
1 => bb2,
2 => bb4,
4 => bb6,
_ => bb5
}
}
bb9 = {
_2 = [13235_u16,51493_u16,926_u16,29956_u16,17913_u16];
_5 = 69687820697062905784847610820667154630_i128 ^ 152358785176326529530331743427664351640_i128;
RET = false;
_7 = [(-1809818172_i32),1656481829_i32,(-1179874779_i32),(-1789320876_i32)];
RET = !true;
_5 = 32216874759469255839898548290229174763_i128 & 146724246405350427483542586259070275798_i128;
_2 = [15823_u16,10400_u16,11125_u16,34986_u16,45014_u16];
_7 = [564527538_i32,(-370793282_i32),(-1866245121_i32),1665486229_i32];
_5 = -(-47878607303673459396099663976833186242_i128);
_12 = 238695582_u32 as f64;
_6.0 = Adt27::Variant0 { fld0: (-1294916163_i32),fld1: _3,fld2: _12 };
_10 = (-82_i8) as f32;
RET = !false;
_3 = Field::<f32>(Variant(_6.0, 0), 1);
RET = false;
place!(Field::<i32>(Variant(_6.0, 0), 0)) = 1441648865_i32;
_10 = 13476416208727136341_u64 as f32;
place!(Field::<f64>(Variant(_6.0, 0), 2)) = _12;
_10 = _3;
_15 = 4_isize;
_9 = [_15,_15,_15];
place!(Field::<f32>(Variant(_6.0, 0), 1)) = Field::<f64>(Variant(_6.0, 0), 2) as f32;
_5 = 27925192095900668993215171381033737370_i128 * (-18741901136077433282316345559538478379_i128);
_11 = [_15,_15,_15,_15];
place!(Field::<f64>(Variant(_6.0, 0), 2)) = -_12;
Call(RET = fn6(Move(_1), Move(_6.0)), ReturnTo(bb3), UnwindUnreachable())
}
bb10 = {
_2 = [13235_u16,51493_u16,926_u16,29956_u16,17913_u16];
_5 = 69687820697062905784847610820667154630_i128 ^ 152358785176326529530331743427664351640_i128;
RET = false;
_7 = [(-1809818172_i32),1656481829_i32,(-1179874779_i32),(-1789320876_i32)];
RET = !true;
_5 = 32216874759469255839898548290229174763_i128 & 146724246405350427483542586259070275798_i128;
_2 = [15823_u16,10400_u16,11125_u16,34986_u16,45014_u16];
_7 = [564527538_i32,(-370793282_i32),(-1866245121_i32),1665486229_i32];
_5 = -(-47878607303673459396099663976833186242_i128);
_12 = 238695582_u32 as f64;
_6.0 = Adt27::Variant0 { fld0: (-1294916163_i32),fld1: _3,fld2: _12 };
_10 = (-82_i8) as f32;
RET = !false;
_3 = Field::<f32>(Variant(_6.0, 0), 1);
RET = false;
place!(Field::<i32>(Variant(_6.0, 0), 0)) = 1441648865_i32;
_10 = 13476416208727136341_u64 as f32;
place!(Field::<f64>(Variant(_6.0, 0), 2)) = _12;
_10 = _3;
_15 = 4_isize;
_9 = [_15,_15,_15];
place!(Field::<f32>(Variant(_6.0, 0), 1)) = Field::<f64>(Variant(_6.0, 0), 2) as f32;
_5 = 27925192095900668993215171381033737370_i128 * (-18741901136077433282316345559538478379_i128);
_11 = [_15,_15,_15,_15];
place!(Field::<f64>(Variant(_6.0, 0), 2)) = -_12;
Call(RET = fn6(Move(_1), Move(_6.0)), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_7 = [_21,_21,_21,_21];
place!(Field::<f64>(Variant(_6.0, 0), 2)) = _3 as f64;
match _15 {
0 => bb6,
1 => bb12,
2 => bb13,
4 => bb15,
_ => bb14
}
}
bb12 = {
_5 = (-124663550224402949709442980326672306892_i128);
RET = false;
_2 = [49993_u16,26159_u16,19995_u16,8409_u16,62101_u16];
_7 = [1138875285_i32,658718049_i32,1067901785_i32,63004219_i32];
RET = !false;
_2 = [41128_u16,1382_u16,23256_u16,25778_u16,20608_u16];
_7 = [12921507_i32,(-1841738879_i32),(-6264817_i32),323866278_i32];
_7 = [1246645694_i32,1995267856_i32,1587596041_i32,1050484602_i32];
RET = true & false;
RET = _5 <= _5;
_5 = !(-168803426754506633425686755415867045482_i128);
_2 = [10747_u16,2915_u16,25540_u16,28072_u16,6652_u16];
_3 = 60_i8 as f32;
RET = false;
_5 = (-67869607190059145251087790824838933037_i128);
_3 = 9223372036854775807_isize as f32;
_3 = 9223372036854775807_isize as f32;
_9 = [(-58_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_7 = [100817565_i32,(-1048503311_i32),1157204124_i32,(-2079216941_i32)];
RET = false;
RET = true;
RET = true;
RET = true;
_5 = 154906800262418716297611851567958096244_i128 + 158342317310009702575425612346131414045_i128;
RET = _3 <= _3;
Goto(bb2)
}
bb13 = {
_2 = [13235_u16,51493_u16,926_u16,29956_u16,17913_u16];
_5 = 69687820697062905784847610820667154630_i128 ^ 152358785176326529530331743427664351640_i128;
RET = false;
_7 = [(-1809818172_i32),1656481829_i32,(-1179874779_i32),(-1789320876_i32)];
RET = !true;
_5 = 32216874759469255839898548290229174763_i128 & 146724246405350427483542586259070275798_i128;
_2 = [15823_u16,10400_u16,11125_u16,34986_u16,45014_u16];
_7 = [564527538_i32,(-370793282_i32),(-1866245121_i32),1665486229_i32];
_5 = -(-47878607303673459396099663976833186242_i128);
_12 = 238695582_u32 as f64;
_6.0 = Adt27::Variant0 { fld0: (-1294916163_i32),fld1: _3,fld2: _12 };
_10 = (-82_i8) as f32;
RET = !false;
_3 = Field::<f32>(Variant(_6.0, 0), 1);
RET = false;
place!(Field::<i32>(Variant(_6.0, 0), 0)) = 1441648865_i32;
_10 = 13476416208727136341_u64 as f32;
place!(Field::<f64>(Variant(_6.0, 0), 2)) = _12;
_10 = _3;
_15 = 4_isize;
_9 = [_15,_15,_15];
place!(Field::<f32>(Variant(_6.0, 0), 1)) = Field::<f64>(Variant(_6.0, 0), 2) as f32;
_5 = 27925192095900668993215171381033737370_i128 * (-18741901136077433282316345559538478379_i128);
_11 = [_15,_15,_15,_15];
place!(Field::<f64>(Variant(_6.0, 0), 2)) = -_12;
Call(RET = fn6(Move(_1), Move(_6.0)), ReturnTo(bb3), UnwindUnreachable())
}
bb14 = {
_4.0 = Adt27::Variant0 { fld0: 736398641_i32,fld1: _3,fld2: _12 };
_7 = [(-1311416013_i32),(-298830138_i32),(-375180257_i32),(-1643754438_i32)];
_2 = [6711_u16,15762_u16,17833_u16,61229_u16,14379_u16];
_12 = 0_usize as f64;
_1 = core::ptr::addr_of_mut!(_18.0);
place!(Field::<i32>(Variant(_6.0, 0), 0)) = 239420823_i32;
SetDiscriminant(_6.0, 0);
RET = !true;
(*_1) = &_22;
_21 = !(-743330788_i32);
_2 = [3847_u16,6595_u16,5589_u16,31671_u16,62_u16];
(*_1) = &place!(Field::<f64>(Variant(_6.0, 0), 2));
_21 = 2069579548_i32 + (-709592324_i32);
(*_1) = &place!(Field::<f64>(Variant(_6.0, 0), 2));
_19 = [3219322063_u32,4118722569_u32];
place!(Field::<f32>(Variant(_4.0, 0), 1)) = _3 + _10;
(*_1) = &place!(Field::<f64>(Variant(_6.0, 0), 2));
place!(Field::<f32>(Variant(_4.0, 0), 1)) = 9865760845254437147_usize as f32;
_21 = (-1192416245_i32) - (-1229305911_i32);
_24 = 292438446642236683121012731187458757505_u128;
_1 = core::ptr::addr_of_mut!(_18.0);
RET = !false;
_3 = Field::<f32>(Variant(_4.0, 0), 1);
Goto(bb7)
}
bb15 = {
_17 = [776625892299212777_usize];
place!(Field::<i32>(Variant(_6.0, 0), 0)) = _21;
_27 = 12901077177495627818_u64 as u32;
_9 = [_15,_15,_15];
_18.0 = &place!(Field::<f64>(Variant(_6.0, 0), 2));
RET = _5 == _5;
_17 = [0_usize];
place!(Field::<i32>(Variant(_6.0, 0), 0)) = 10934731404077320936_u64 as i32;
_5 = -(-21544497436162663478863289988753546877_i128);
place!(Field::<i32>(Variant(_6.0, 0), 0)) = !_21;
_22 = Field::<f64>(Variant(_6.0, 0), 2);
_12 = Field::<f64>(Variant(_6.0, 0), 2) + Field::<f64>(Variant(_6.0, 0), 2);
_28 = [14_u8,181_u8,136_u8,226_u8,153_u8];
_27 = 2309582925_u32;
SetDiscriminant(_6.0, 0);
_16 = [_15,_15,_15,_15];
_11 = [_15,_15,_15,_15];
place!(Field::<f32>(Variant(_6.0, 0), 1)) = _3;
place!(Field::<f32>(Variant(_6.0, 0), 1)) = _10;
_18.0 = &place!(Field::<f64>(Variant(_4.0, 0), 2));
RET = true & true;
Goto(bb16)
}
bb16 = {
Call(_33 = dump_var(5_usize, 19_usize, Move(_19), 7_usize, Move(_7), 11_usize, Move(_11), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(5_usize, 9_usize, Move(_9), 21_usize, Move(_21), 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: *mut &'static f64,mut _2: Adt27) -> bool {
mir! {
type RET = bool;
let _3: *mut i16;
let _4: [u64; 8];
let _5: i8;
let _6: [usize; 8];
let _7: [u8; 5];
let _8: char;
let _9: usize;
let _10: i16;
let _11: (f32,);
let _12: (&'static f64,);
let _13: bool;
let _14: isize;
let _15: (i128,);
let _16: [u64; 8];
let _17: u16;
let _18: [u32; 2];
let _19: isize;
let _20: char;
let _21: *mut i16;
let _22: i16;
let _23: f32;
let _24: *mut (&'static bool, (i128,));
let _25: Adt48;
let _26: *mut (&'static bool, (i128,));
let _27: Adt27;
let _28: *const Adt34;
let _29: *mut &'static u8;
let _30: &'static char;
let _31: isize;
let _32: (&'static *mut u8, bool);
let _33: isize;
let _34: char;
let _35: u128;
let _36: char;
let _37: char;
let _38: [u8; 5];
let _39: f64;
let _40: *mut &'static f64;
let _41: ();
let _42: ();
{
place!(Field::<f64>(Variant(_2, 0), 2)) = 89_u8 as f64;
SetDiscriminant(_2, 1);
RET = !false;
place!(Field::<isize>(Variant(_2, 1), 2)) = 161_u8 as isize;
place!(Field::<[usize; 8]>(Variant(_2, 1), 3)) = [7809715567825939142_usize,16610918274649294096_usize,5_usize,6_usize,17567973586234036095_usize,4_usize,525345723014056507_usize,0_usize];
RET = false;
_5 = 59_i8;
_4 = [10281131453315644309_u64,11737665933370033072_u64,17438243182004436674_u64,17516984157924598806_u64,15496590028753312105_u64,4547604398327405767_u64,493151942936165847_u64,14344412151053734159_u64];
_5 = !56_i8;
place!(Field::<u128>(Variant(_2, 1), 0)) = _5 as u128;
place!(Field::<f32>(Variant(_2, 1), 4)) = (-8591748937521564034_i64) as f32;
place!(Field::<f32>(Variant(_2, 1), 4)) = 15_u8 as f32;
RET = Field::<f32>(Variant(_2, 1), 4) > Field::<f32>(Variant(_2, 1), 4);
place!(Field::<u128>(Variant(_2, 1), 0)) = 330863779997751196844936018214739776334_u128 ^ 42975483958400771854828758026777119929_u128;
RET = !false;
place!(Field::<u128>(Variant(_2, 1), 0)) = 70451953222425603290048564643519499383_u128;
RET = !true;
RET = false;
Goto(bb1)
}
bb1 = {
place!(Field::<usize>(Variant(_2, 1), 1)) = (-3240859323699545385_i64) as usize;
place!(Field::<usize>(Variant(_2, 1), 1)) = 2038115726252840084_usize + 338630211113333288_usize;
_6 = [Field::<usize>(Variant(_2, 1), 1),Field::<usize>(Variant(_2, 1), 1),Field::<usize>(Variant(_2, 1), 1),Field::<usize>(Variant(_2, 1), 1),Field::<usize>(Variant(_2, 1), 1),Field::<usize>(Variant(_2, 1), 1),Field::<usize>(Variant(_2, 1), 1),Field::<usize>(Variant(_2, 1), 1)];
place!(Field::<[usize; 8]>(Variant(_2, 1), 3)) = _6;
place!(Field::<f32>(Variant(_2, 1), 4)) = Field::<u128>(Variant(_2, 1), 0) as f32;
place!(Field::<usize>(Variant(_2, 1), 1)) = 16213713286433848258_usize;
place!(Field::<usize>(Variant(_2, 1), 1)) = (-1235944224_i32) as usize;
Goto(bb2)
}
bb2 = {
_7 = [26_u8,142_u8,170_u8,210_u8,12_u8];
_4 = [3033044045150905053_u64,12446398987636260306_u64,5201043494818146041_u64,2523826326494970772_u64,4696670040318256158_u64,4671279106025836269_u64,5297201939706188344_u64,1526297227898643208_u64];
place!(Field::<usize>(Variant(_2, 1), 1)) = 28369_i16 as usize;
_4 = [7238825821324212207_u64,9964678340764654674_u64,2614099791059084646_u64,457286940949667721_u64,2549822290032186648_u64,11515480411860026367_u64,15527100388404037335_u64,8392875273300144307_u64];
_6 = Field::<[usize; 8]>(Variant(_2, 1), 3);
_8 = '\u{16cb3}';
_8 = '\u{60a7b}';
_9 = Field::<usize>(Variant(_2, 1), 1);
_11.0 = Field::<f32>(Variant(_2, 1), 4);
_8 = '\u{f4b13}';
_8 = '\u{4922c}';
RET = !false;
place!(Field::<usize>(Variant(_2, 1), 1)) = !_9;
_11 = (Field::<f32>(Variant(_2, 1), 4),);
match Field::<u128>(Variant(_2, 1), 0) {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
70451953222425603290048564643519499383 => bb9,
_ => bb8
}
}
bb3 = {
place!(Field::<usize>(Variant(_2, 1), 1)) = (-3240859323699545385_i64) as usize;
place!(Field::<usize>(Variant(_2, 1), 1)) = 2038115726252840084_usize + 338630211113333288_usize;
_6 = [Field::<usize>(Variant(_2, 1), 1),Field::<usize>(Variant(_2, 1), 1),Field::<usize>(Variant(_2, 1), 1),Field::<usize>(Variant(_2, 1), 1),Field::<usize>(Variant(_2, 1), 1),Field::<usize>(Variant(_2, 1), 1),Field::<usize>(Variant(_2, 1), 1),Field::<usize>(Variant(_2, 1), 1)];
place!(Field::<[usize; 8]>(Variant(_2, 1), 3)) = _6;
place!(Field::<f32>(Variant(_2, 1), 4)) = Field::<u128>(Variant(_2, 1), 0) as f32;
place!(Field::<usize>(Variant(_2, 1), 1)) = 16213713286433848258_usize;
place!(Field::<usize>(Variant(_2, 1), 1)) = (-1235944224_i32) as usize;
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
RET = Field::<isize>(Variant(_2, 1), 2) > Field::<isize>(Variant(_2, 1), 2);
place!(Field::<[usize; 8]>(Variant(_2, 1), 3)) = _6;
_9 = 10647_i16 as usize;
place!(Field::<u128>(Variant(_2, 1), 0)) = !164648477283455478798646769206796561113_u128;
_10 = 7581237368833979801_u64 as i16;
_1 = core::ptr::addr_of_mut!(_12.0);
place!(Field::<usize>(Variant(_2, 1), 1)) = _9 << _5;
_8 = '\u{1f61}';
place!(Field::<f32>(Variant(_2, 1), 4)) = Field::<isize>(Variant(_2, 1), 2) as f32;
_4 = [16549072598367835488_u64,2986476019387007178_u64,4466602421153692613_u64,15544448166060476403_u64,4013348003109437446_u64,11652267120746450267_u64,9674991468966920831_u64,14892159430418399747_u64];
RET = !false;
_13 = _11.0 < Field::<f32>(Variant(_2, 1), 4);
Call(_6 = core::intrinsics::transmute(Field::<[usize; 8]>(Variant(_2, 1), 3)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_3 = core::ptr::addr_of_mut!(_10);
place!(Field::<usize>(Variant(_2, 1), 1)) = 2448165978942971363_i64 as usize;
_11 = (Field::<f32>(Variant(_2, 1), 4),);
_1 = core::ptr::addr_of_mut!(_12.0);
_8 = '\u{e0609}';
_1 = core::ptr::addr_of_mut!((*_1));
place!(Field::<f32>(Variant(_2, 1), 4)) = _11.0 * _11.0;
_14 = -Field::<isize>(Variant(_2, 1), 2);
_8 = '\u{6fc2e}';
_1 = core::ptr::addr_of_mut!((*_1));
(*_3) = (-13995_i16);
(*_3) = Field::<u128>(Variant(_2, 1), 0) as i16;
_7 = [90_u8,139_u8,63_u8,33_u8,176_u8];
(*_3) = 191_u8 as i16;
_11 = (Field::<f32>(Variant(_2, 1), 4),);
(*_3) = 3971500335109705194_i64 as i16;
_10 = Field::<f32>(Variant(_2, 1), 4) as i16;
_14 = (*_3) as isize;
(*_3) = (-1410787101_i32) as i16;
Goto(bb11)
}
bb11 = {
_13 = !RET;
_7 = [229_u8,70_u8,46_u8,6_u8,93_u8];
_13 = RET;
_9 = Field::<usize>(Variant(_2, 1), 1);
_16 = _4;
_16 = [9040130550167386633_u64,7537415308306274811_u64,7104254660609069036_u64,10222431516611045979_u64,1357336307735080491_u64,15632861904365022575_u64,4976188725509596217_u64,3200829269970460021_u64];
_5 = !(-43_i8);
place!(Field::<u128>(Variant(_2, 1), 0)) = _5 as u128;
_15.0 = -(-6783444159599174771093528624209782149_i128);
(*_3) = (-21781_i16) | (-25172_i16);
(*_3) = -3279_i16;
_17 = !51838_u16;
RET = !_13;
place!(Field::<[usize; 8]>(Variant(_2, 1), 3)) = [_9,Field::<usize>(Variant(_2, 1), 1),_9,Field::<usize>(Variant(_2, 1), 1),_9,Field::<usize>(Variant(_2, 1), 1),_9,Field::<usize>(Variant(_2, 1), 1)];
_15.0 = -78414288074818686289986157541185172091_i128;
_3 = core::ptr::addr_of_mut!((*_3));
_16 = [14602721131471156113_u64,17617507391494665033_u64,5391611718729366722_u64,10503794080980660439_u64,2019150515374933348_u64,9617822720714582706_u64,1591012127505225536_u64,11826622955760011262_u64];
_19 = !_14;
place!(Field::<usize>(Variant(_2, 1), 1)) = _9;
_8 = '\u{46b68}';
_13 = RET;
_7 = [43_u8,172_u8,27_u8,105_u8,164_u8];
Goto(bb12)
}
bb12 = {
_10 = (-11106_i16) << _15.0;
_11 = (Field::<f32>(Variant(_2, 1), 4),);
_18 = [1022664707_u32,525777057_u32];
_17 = _19 as u16;
_13 = RET;
_5 = 6139042515956113630_i64 as i8;
_15.0 = _14 as i128;
_8 = '\u{974de}';
place!(Field::<u128>(Variant(_2, 1), 0)) = !180037281839457789971221280131397174931_u128;
RET = !_13;
_17 = !39082_u16;
_20 = _8;
_21 = core::ptr::addr_of_mut!(_10);
(*_3) = -8497_i16;
(*_3) = -(-19590_i16);
_17 = Field::<usize>(Variant(_2, 1), 1) as u16;
_1 = core::ptr::addr_of_mut!((*_1));
(*_3) = !636_i16;
_4 = [2596355170091213206_u64,13769180252538214306_u64,13398106518072762383_u64,11590623303618814768_u64,699850392414972531_u64,1100804359956355451_u64,6672836679932724211_u64,10912815503818508789_u64];
_20 = _8;
Call(_23 = core::intrinsics::transmute(_8), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
(*_21) = 15863_i16 * (-14322_i16);
_5 = _15.0 as i8;
place!(Field::<[usize; 8]>(Variant(_2, 1), 3)) = [Field::<usize>(Variant(_2, 1), 1),Field::<usize>(Variant(_2, 1), 1),Field::<usize>(Variant(_2, 1), 1),_9,_9,_9,Field::<usize>(Variant(_2, 1), 1),Field::<usize>(Variant(_2, 1), 1)];
place!(Field::<f32>(Variant(_2, 1), 4)) = _11.0;
place!(Field::<[usize; 8]>(Variant(_2, 1), 3)) = _6;
_15 = ((-107372065971328401462693631124238632404_i128),);
_11.0 = _23 - _23;
_31 = !Field::<isize>(Variant(_2, 1), 2);
_18 = [4196387260_u32,4233456269_u32];
_32.0 = &place!(Field::<*mut u8>(Variant(_2, 1), 5));
_32.1 = !_13;
_18 = [2577244698_u32,521975175_u32];
_21 = core::ptr::addr_of_mut!(_22);
_32.0 = &place!(Field::<*mut u8>(Variant(_2, 1), 5));
match _15.0 {
0 => bb11,
1 => bb2,
2 => bb10,
3 => bb9,
4 => bb5,
5 => bb6,
6 => bb7,
232910300949610062000680976307529579052 => bb14,
_ => bb8
}
}
bb14 = {
_21 = Move(_3);
_20 = _8;
_5 = !77_i8;
_20 = _8;
_10 = 14836_i16;
_36 = _20;
_21 = core::ptr::addr_of_mut!(_10);
_8 = _36;
RET = !_32.1;
_8 = _20;
_33 = Field::<u128>(Variant(_2, 1), 0) as isize;
_9 = Field::<usize>(Variant(_2, 1), 1) << _19;
(*_1) = &_39;
place!(Field::<[usize; 8]>(Variant(_2, 1), 3)) = [Field::<usize>(Variant(_2, 1), 1),_9,_9,_9,_9,Field::<usize>(Variant(_2, 1), 1),Field::<usize>(Variant(_2, 1), 1),Field::<usize>(Variant(_2, 1), 1)];
_22 = !(*_21);
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(6_usize, 36_usize, Move(_36), 13_usize, Move(_13), 19_usize, Move(_19), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(6_usize, 22_usize, Move(_22), 9_usize, Move(_9), 14_usize, Move(_14), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(6_usize, 16_usize, Move(_16), 42_usize, _42, 42_usize, _42, 42_usize, _42), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: *mut &'static f64) -> u32 {
mir! {
type RET = u32;
let _2: f64;
let _3: char;
let _4: bool;
let _5: f64;
let _6: Adt22;
let _7: bool;
let _8: [u32; 2];
let _9: u128;
let _10: (f32,);
let _11: [u64; 8];
let _12: u16;
let _13: &'static i32;
let _14: *mut [isize; 3];
let _15: &'static (&'static f64,);
let _16: i8;
let _17: bool;
let _18: i128;
let _19: u32;
let _20: isize;
let _21: bool;
let _22: bool;
let _23: i32;
let _24: f64;
let _25: i32;
let _26: ();
let _27: ();
{
RET = 1742598957_u32;
RET = !2595842729_u32;
_2 = RET as f64;
_2 = (-1128678163_i32) as f64;
RET = 853267779_u32 + 270037415_u32;
_3 = '\u{9e42c}';
RET = 1836596962_u32 & 361489689_u32;
RET = 14528294461954908504_usize as u32;
RET = 683835632_u32 - 4144122092_u32;
RET = 878501240_u32;
RET = 15390_i16 as u32;
RET = 3530533537_u32 & 3943711252_u32;
_4 = !false;
RET = _3 as u32;
_3 = '\u{108269}';
_4 = !true;
RET = 1954068831_u32;
Goto(bb1)
}
bb1 = {
_5 = -_2;
_5 = (-164455569591799089995959336679489205620_i128) as f64;
_3 = '\u{a9107}';
RET = 3708204742_u32 >> 7593555836495039588_usize;
_3 = '\u{9fc06}';
RET = 13150325148743258240_usize as u32;
_4 = _5 <= _2;
RET = 4179228501_u32;
_4 = _3 >= _3;
_4 = false | true;
_5 = 168171413884663541973147170682652004067_u128 as f64;
_2 = _5;
_6 = Adt22::Variant1 { fld0: _4,fld1: _3,fld2: 2_usize,fld3: 23955604027231960785568001664014222625_i128,fld4: 6814376178577960509_u64 };
place!(Field::<i128>(Variant(_6, 1), 3)) = 76120065358026968611654366159449052285_i128 - (-115150565082094209328445655588461384454_i128);
_2 = _5;
_6 = Adt22::Variant1 { fld0: _4,fld1: _3,fld2: 17884046506692335130_usize,fld3: (-137392114953937663214683502040620357695_i128),fld4: 2237863705889205215_u64 };
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
4179228501 => bb10,
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
_8 = [RET,RET];
_7 = _4 & Field::<bool>(Variant(_6, 1), 0);
_2 = _5;
_3 = Field::<char>(Variant(_6, 1), 1);
place!(Field::<u64>(Variant(_6, 1), 4)) = 17978875766396641413_u64;
place!(Field::<usize>(Variant(_6, 1), 2)) = 2057942666081395277_usize >> Field::<u64>(Variant(_6, 1), 4);
_3 = Field::<char>(Variant(_6, 1), 1);
_9 = (-1278539757_i32) as u128;
RET = Field::<char>(Variant(_6, 1), 1) as u32;
RET = _9 as u32;
place!(Field::<u64>(Variant(_6, 1), 4)) = Field::<bool>(Variant(_6, 1), 0) as u64;
_7 = !_4;
_6 = Adt22::Variant0 { fld0: 13171063985868535709_usize,fld1: 26201_u16 };
_10.0 = _5 as f32;
_11 = [13689666380438977989_u64,17236674439146598936_u64,3884636232760704682_u64,6448458764992286283_u64,9480347903261970614_u64,12242055082871578119_u64,8693428159510175527_u64,10688892711257221534_u64];
_10.0 = 1231624894_i32 as f32;
place!(Field::<usize>(Variant(_6, 0), 0)) = 6118882287082666058_usize;
match Field::<usize>(Variant(_6, 0), 0) {
0 => bb4,
1 => bb11,
6118882287082666058 => bb13,
_ => bb12
}
}
bb11 = {
Return()
}
bb12 = {
_5 = -_2;
_5 = (-164455569591799089995959336679489205620_i128) as f64;
_3 = '\u{a9107}';
RET = 3708204742_u32 >> 7593555836495039588_usize;
_3 = '\u{9fc06}';
RET = 13150325148743258240_usize as u32;
_4 = _5 <= _2;
RET = 4179228501_u32;
_4 = _3 >= _3;
_4 = false | true;
_5 = 168171413884663541973147170682652004067_u128 as f64;
_2 = _5;
_6 = Adt22::Variant1 { fld0: _4,fld1: _3,fld2: 2_usize,fld3: 23955604027231960785568001664014222625_i128,fld4: 6814376178577960509_u64 };
place!(Field::<i128>(Variant(_6, 1), 3)) = 76120065358026968611654366159449052285_i128 - (-115150565082094209328445655588461384454_i128);
_2 = _5;
_6 = Adt22::Variant1 { fld0: _4,fld1: _3,fld2: 17884046506692335130_usize,fld3: (-137392114953937663214683502040620357695_i128),fld4: 2237863705889205215_u64 };
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
4179228501 => bb10,
_ => bb9
}
}
bb13 = {
_16 = !(-35_i8);
_12 = !41955_u16;
_2 = _5 + _5;
_11 = [5335734925752549070_u64,2444071525707348346_u64,12122340475151431868_u64,5833775576097891004_u64,1371104194797366405_u64,2490233918671189704_u64,12425676279297508535_u64,9503222303763024423_u64];
_6 = Adt22::Variant0 { fld0: 6461017598012267622_usize,fld1: _12 };
_17 = !_4;
_18 = (-101623099462588771932718282821533482738_i128) << _9;
_5 = -_2;
_5 = 9223372036854775807_isize as f64;
_5 = (-9223372036854775808_isize) as f64;
RET = 1781993625_u32;
place!(Field::<usize>(Variant(_6, 0), 0)) = 11521572887425579483_usize - 12032472127091622788_usize;
_9 = !179545513110826187492706255733229583716_u128;
_11 = [16722079525723607991_u64,9170751164461016633_u64,7129663317903817262_u64,12784150018801243488_u64,14120617907367491851_u64,2906617697889996514_u64,3678608527645746383_u64,993825616012348378_u64];
_3 = '\u{dc6f0}';
_19 = RET;
_21 = !_4;
place!(Field::<u16>(Variant(_6, 0), 1)) = _12 * _12;
_19 = _10.0 as u32;
Call(_8 = fn8(Move(_1), _21, _7, Field::<u16>(Variant(_6, 0), 1), _3, _17), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_22 = _17;
_9 = _17 as u128;
_6 = Adt22::Variant0 { fld0: 6_usize,fld1: _12 };
_18 = !(-104604774825529939290494037558612276275_i128);
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(7_usize, 19_usize, Move(_19), 17_usize, Move(_17), 3_usize, Move(_3), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(7_usize, 8_usize, Move(_8), 18_usize, Move(_18), 27_usize, _27, 27_usize, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: *mut &'static f64,mut _2: bool,mut _3: bool,mut _4: u16,mut _5: char,mut _6: bool) -> [u32; 2] {
mir! {
type RET = [u32; 2];
let _7: [isize; 3];
let _8: (&'static f64,);
let _9: f32;
let _10: char;
let _11: (Adt27, *mut [usize; 8]);
let _12: (u32,);
let _13: &'static i32;
let _14: [isize; 4];
let _15: char;
let _16: [usize; 1];
let _17: Adt77;
let _18: *mut Adt48;
let _19: [i8; 7];
let _20: [u32; 2];
let _21: *mut [usize; 8];
let _22: [i8; 3];
let _23: isize;
let _24: &'static usize;
let _25: *const i8;
let _26: i16;
let _27: u128;
let _28: Adt48;
let _29: bool;
let _30: [i16; 5];
let _31: &'static (f32,);
let _32: f32;
let _33: i16;
let _34: ();
let _35: ();
{
_5 = '\u{f2f5d}';
RET = [911189105_u32,4032087045_u32];
_3 = _6 == _6;
_2 = _3 >= _3;
_4 = 44459_u16 & 15564_u16;
RET = [445166555_u32,503069364_u32];
_4 = 40207_u16;
_6 = !_3;
_2 = _6;
_2 = _6 != _6;
_7 = [(-9223372036854775808_isize),52_isize,(-9223372036854775808_isize)];
_1 = core::ptr::addr_of_mut!(_8.0);
RET = [3585522707_u32,171728096_u32];
_3 = _6;
_1 = core::ptr::addr_of_mut!(_8.0);
_5 = '\u{d9dc2}';
Goto(bb1)
}
bb1 = {
_1 = core::ptr::addr_of_mut!((*_1));
_5 = '\u{9e62a}';
_5 = '\u{9d850}';
Call(_9 = fn9(), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = '\u{8ee4d}';
_10 = _5;
match _4 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
40207 => bb7,
_ => bb6
}
}
bb3 = {
_1 = core::ptr::addr_of_mut!((*_1));
_5 = '\u{9e62a}';
_5 = '\u{9d850}';
Call(_9 = fn9(), ReturnTo(bb2), UnwindUnreachable())
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
_5 = _10;
RET = [1267133250_u32,2093213504_u32];
_12.0 = !2063981507_u32;
_6 = !_2;
RET = [_12.0,_12.0];
_14 = [(-9223372036854775808_isize),18_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_5 = _10;
RET = [_12.0,_12.0];
RET = [_12.0,_12.0];
_14 = [56_isize,122_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_17.fld7.0 = (-725783630_i32) as u32;
_13 = &_17.fld5;
_17.fld1 = _10;
_6 = _2;
_3 = _6 | _2;
_15 = _17.fld1;
_17.fld0.0 = _15;
_7 = [9223372036854775807_isize,(-65_isize),102_isize];
_14 = [9223372036854775807_isize,(-9223372036854775808_isize),(-126_isize),9223372036854775807_isize];
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
40207 => bb8,
_ => bb4
}
}
bb8 = {
_17.fld5 = -(-1940819968_i32);
_17.fld2 = [45_i8,(-81_i8),112_i8];
_17.fld2 = [127_i8,(-17_i8),90_i8];
_9 = 30809_i16 as f32;
_7 = [65_isize,(-18_isize),9223372036854775807_isize];
_17.fld7 = (_12.0,);
_17.fld1 = _15;
_18 = core::ptr::addr_of_mut!(_17.fld3);
_17.fld7 = (_12.0,);
_13 = &_17.fld5;
_17.fld5 = (-698076391_i32) * 1694892468_i32;
_5 = _15;
_14 = [9223372036854775807_isize,(-9223372036854775808_isize),124_isize,9223372036854775807_isize];
Goto(bb9)
}
bb9 = {
_18 = core::ptr::addr_of_mut!(_17.fld3);
_7 = [9223372036854775807_isize,(-9223372036854775808_isize),(-18_isize)];
_17.fld7.0 = _12.0;
_17.fld0.0 = _10;
_12 = (_17.fld7.0,);
_4 = !50311_u16;
_7 = [9223372036854775807_isize,7_isize,9223372036854775807_isize];
_19 = [(-49_i8),(-121_i8),(-94_i8),(-32_i8),(-51_i8),63_i8,(-6_i8)];
_15 = _17.fld1;
_16 = [4_usize];
_18 = core::ptr::addr_of_mut!((*_18));
_12.0 = _17.fld7.0;
_19 = [4_i8,(-77_i8),50_i8,(-18_i8),17_i8,68_i8,(-79_i8)];
RET = [_17.fld7.0,_17.fld7.0];
_20 = [_12.0,_12.0];
_17.fld5 = 39_u8 as i32;
_17.fld5 = -2065574401_i32;
_22 = [127_i8,(-89_i8),6_i8];
_17.fld2 = [93_i8,86_i8,7_i8];
_17.fld2 = [(-79_i8),(-30_i8),(-20_i8)];
_9 = 0_usize as f32;
Goto(bb10)
}
bb10 = {
_17.fld5 = 1889883684_i32;
_1 = core::ptr::addr_of_mut!(_8.0);
_18 = core::ptr::addr_of_mut!(_17.fld3);
_23 = -7_isize;
_14 = [_23,_23,_23,_23];
_17.fld2 = _22;
_17.fld2 = [(-104_i8),(-83_i8),61_i8];
RET = [_17.fld7.0,_12.0];
RET = _20;
_23 = (-86_isize) | 28_isize;
_26 = !14561_i16;
_14 = [_23,_23,_23,_23];
_1 = core::ptr::addr_of_mut!((*_1));
_18 = core::ptr::addr_of_mut!((*_18));
_7 = [_23,_23,_23];
_3 = _6 <= _6;
_26 = (-6587_i16);
_10 = _17.fld1;
_26 = 10003_i16 ^ 21928_i16;
Goto(bb11)
}
bb11 = {
_19 = [(-74_i8),47_i8,(-74_i8),(-81_i8),(-79_i8),6_i8,18_i8];
_12.0 = !_17.fld7.0;
_9 = (-135200731502238523970674410672698476519_i128) as f32;
RET = [_12.0,_12.0];
_2 = _3;
_1 = core::ptr::addr_of_mut!((*_1));
_5 = _17.fld1;
_29 = _3;
_17.fld7 = (_12.0,);
_4 = 37656029116510905119375377331291310025_i128 as u16;
_16 = [6484097914110649464_usize];
_17.fld7 = (_12.0,);
_19 = [65_i8,35_i8,45_i8,(-99_i8),(-83_i8),85_i8,(-72_i8)];
RET = [_12.0,_12.0];
_12.0 = _17.fld7.0 ^ _17.fld7.0;
_20 = [_12.0,_17.fld7.0];
_23 = _26 as isize;
_10 = _15;
_13 = &_17.fld5;
match _17.fld5 {
0 => bb12,
1 => bb13,
1889883684 => bb15,
_ => bb14
}
}
bb12 = {
Return()
}
bb13 = {
_1 = core::ptr::addr_of_mut!((*_1));
_5 = '\u{9e62a}';
_5 = '\u{9d850}';
Call(_9 = fn9(), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
Return()
}
bb15 = {
_12 = (_17.fld7.0,);
_17.fld7 = (_12.0,);
_1 = core::ptr::addr_of_mut!(_8.0);
_17.fld7.0 = _12.0 & _12.0;
_19 = [(-73_i8),(-34_i8),82_i8,(-65_i8),(-44_i8),22_i8,125_i8];
_26 = (-10748_i16) ^ 3024_i16;
_17.fld0.0 = _15;
RET = [_12.0,_12.0];
_9 = _26 as f32;
_26 = 15918_i16 & 31089_i16;
_30 = [_26,_26,_26,_26,_26];
_27 = 32374281733608924667369047078789976889_u128;
_17.fld1 = _10;
_16 = [3_usize];
_3 = _29;
RET = [_17.fld7.0,_12.0];
_26 = (-8736_i16) * (-24868_i16);
_32 = _9 * _9;
Goto(bb16)
}
bb16 = {
Call(_34 = dump_var(8_usize, 7_usize, Move(_7), 14_usize, Move(_14), 27_usize, Move(_27), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(8_usize, 5_usize, Move(_5), 12_usize, Move(_12), 2_usize, Move(_2), 26_usize, Move(_26)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(8_usize, 20_usize, Move(_20), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9() -> f32 {
mir! {
type RET = f32;
let _1: [i16; 5];
let _2: ((char, Adt27), isize);
let _3: [u128; 7];
let _4: f64;
let _5: ();
let _6: ();
{
_1 = [(-31939_i16),(-32459_i16),24916_i16,32399_i16,16411_i16];
RET = (-2245661083202879563_i64) as f32;
_2.0.0 = '\u{ef718}';
_1 = [(-11018_i16),(-21324_i16),(-32024_i16),(-13161_i16),(-12109_i16)];
_3 = [274683973680502417892410919676778103577_u128,307680349361086212032899083781633835284_u128,280646126391270937732513344569731545034_u128,21587466599028318394546520515279046049_u128,289277156912448955219466499914592408119_u128,275068201550719398959228066100319681175_u128,284461229525919811108890599186142847346_u128];
_2.1 = (-9223372036854775808_isize) >> 7830_u16;
Call(RET = fn10(_1, _2.1, _3, _3, _2.1, _3, _3, _2.1, _3, _3, _2.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = 798751159702936251_u64 as f64;
_2.0.1 = Adt27::Variant0 { fld0: (-1619609038_i32),fld1: RET,fld2: _4 };
_2.0.0 = '\u{1e5d2}';
Goto(bb2)
}
bb2 = {
Call(_5 = dump_var(9_usize, 3_usize, Move(_3), 6_usize, _6, 6_usize, _6, 6_usize, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: [i16; 5],mut _2: isize,mut _3: [u128; 7],mut _4: [u128; 7],mut _5: isize,mut _6: [u128; 7],mut _7: [u128; 7],mut _8: isize,mut _9: [u128; 7],mut _10: [u128; 7],mut _11: isize) -> f32 {
mir! {
type RET = f32;
let _12: *mut &'static f64;
let _13: *const i8;
let _14: u64;
let _15: *mut i16;
let _16: f32;
let _17: [u8; 5];
let _18: *const i8;
let _19: f64;
let _20: u16;
let _21: i16;
let _22: ();
let _23: ();
{
_4 = _10;
_9 = [252907220212883128000538049437130849991_u128,298216115672847441622728787568085333855_u128,132703243524637226050993152410836508275_u128,74452870436020879637813355727365462544_u128,71624671837937446828722735124265339463_u128,325534593928431659194088752695159840007_u128,133511490005005261083209177465148642196_u128];
_7 = [3667039237694940891754059376553774467_u128,278425557733106393224321322535293652786_u128,186134557849300118206786042861166116894_u128,191145640174021834295514264262495251710_u128,48888521046712291756358382596024561893_u128,110654926547269418431996102193251983081_u128,116544271109055238414091530411283844209_u128];
_11 = (-8_i8) as isize;
_6 = _10;
_8 = -_5;
Call(_2 = fn11(_7, _7, _10, _5, _8, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = [200587401348495929093022324242007395530_u128,189884331676994354594840790181455612604_u128,102860612716938415069066041099742660720_u128,159513430109852046169218295427078680277_u128,186359880661717814876205564743454105398_u128,249398919784514745911712335365831879721_u128,321813097031511281300443540721044599626_u128];
RET = (-3761446531891487509_i64) as f32;
_5 = _2 * _2;
_1 = [5159_i16,14139_i16,(-18264_i16),3241_i16,(-30974_i16)];
_1 = [(-19638_i16),(-14618_i16),15695_i16,(-31653_i16),18807_i16];
_11 = _5;
_4 = [330553164201410212914756867281991860334_u128,277428494678199511399686441102844257431_u128,21800898396927568108133215178405889232_u128,262658235736440314752448913827980416996_u128,221058055040873570934697900399111857504_u128,117788330034068557223797370715091886491_u128,80404721049859276387501463243021646934_u128];
_3 = [290505697099011302396717834794230708858_u128,263635833655857805219637502323841171148_u128,212784764792398492618991354863581017089_u128,202148542919081100405486223319507115731_u128,304861678820693077839678897597284352479_u128,210266170893515327378215437246411189528_u128,210534356565206887674832666936507221011_u128];
_9 = _3;
_7 = [185533534115821660595444158656214565359_u128,123775672426375347104028276288926596240_u128,144209966576866919257007392119235099540_u128,108048242769900289114504844571627015632_u128,308131199410672633920799491145622625551_u128,81783202868087611786657122204226404691_u128,194299037498027417526359816548409304453_u128];
_6 = _3;
_5 = !_8;
RET = 2393533480_u32 as f32;
_6 = [154581241698993734742147452512824370985_u128,27174029383434373320122243655282879377_u128,31134133216295066348539561240197553776_u128,16838174906562596627046429922278289843_u128,46726490902791951305423595595228520617_u128,84791918357307442419341393787236415056_u128,85759758455171858208414359082017069283_u128];
_5 = -_11;
_10 = _3;
RET = 4848383640764969145_u64 as f32;
Goto(bb2)
}
bb2 = {
_6 = [233605736700198597262598065954448104461_u128,1962970574913502011405605752217770964_u128,223968616650597254810639002265162769330_u128,92508250672896534577054691054958300789_u128,307565106313652671629046056257027391311_u128,46549104739018864653750566414870463889_u128,131023323991069941397027937775099833704_u128];
_2 = -_5;
_14 = _11 as u64;
_8 = _2;
_6 = [25471098915571358536991675143607388176_u128,133449761943936562989386265013044937676_u128,319266077797890908416828267221285624207_u128,106634343053987369685312506915281947683_u128,6160486402330256748873637099625789479_u128,225271527142163140309572595664176726800_u128,31013641648918433118686502503872761632_u128];
Call(RET = fn12(_8, _8, _8, _5, _11, _6, _11, _10, _11, _5, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_2 = _11;
_3 = [8107631147316367951031290919514969176_u128,67902341909365323780168618871631903367_u128,338751226379812597205273787265277308831_u128,35163115830146357749521140058624697072_u128,272504906732128313285517674563033594200_u128,125663582286236070657426000139041062009_u128,113131181008593721113546784814971005630_u128];
_16 = RET - RET;
_21 = -10631_i16;
Goto(bb4)
}
bb4 = {
Call(_22 = dump_var(10_usize, 14_usize, Move(_14), 11_usize, Move(_11), 8_usize, Move(_8), 7_usize, Move(_7)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_22 = dump_var(10_usize, 9_usize, Move(_9), 6_usize, Move(_6), 23_usize, _23, 23_usize, _23), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [u128; 7],mut _2: [u128; 7],mut _3: [u128; 7],mut _4: isize,mut _5: isize,mut _6: isize) -> isize {
mir! {
type RET = isize;
let _7: bool;
let _8: u16;
let _9: f32;
let _10: *const i8;
let _11: f64;
let _12: i16;
let _13: char;
let _14: &'static f64;
let _15: (&'static (&'static f64,), Adt34);
let _16: isize;
let _17: f32;
let _18: ();
let _19: ();
{
_4 = _5 ^ _5;
_5 = (-17722084805533983312958466609297282392_i128) as isize;
RET = -_4;
RET = !_6;
_6 = _4 & _4;
_6 = !_4;
_2 = [277501670251237150438645154437730278141_u128,284001036674382955803284526060034470463_u128,156067670196941427432800267346399720761_u128,170154224541246521281008688103586444004_u128,218629675989467449103418908170288721674_u128,31290954655516395976781000707114988065_u128,150184957376435426395212980078352384719_u128];
RET = 18934_u16 as isize;
_5 = -RET;
_9 = (-32585289914621493972550572816364406023_i128) as f32;
_11 = 5412_i16 as f64;
_2 = _3;
RET = '\u{d055a}' as isize;
_5 = !_6;
_1 = [224060862124584665434268046361513359366_u128,254263462406627194404163122652874789919_u128,302788798018616039871810264110050385670_u128,252410027207879927596824114171802090214_u128,235678915240804865124095713205127056518_u128,86959129418888018608838056204123091801_u128,82697939337071682133193889479740640500_u128];
_6 = 1722427221_u32 as isize;
_1 = _2;
_4 = true as isize;
_4 = _5 + _5;
RET = _4;
Goto(bb1)
}
bb1 = {
RET = _4 << _4;
_8 = (-70249018_i32) as u16;
_3 = [9951565024676618763322963550948812450_u128,111486974998372982754836237884843556376_u128,284500248269535984933850768522616732051_u128,242237625237072938122785724077921711270_u128,18632264539096086995356420307013462123_u128,256022465417619718793282670584593115692_u128,17992823982407965317874984961003309122_u128];
_11 = 98550945892318236438234473504590915558_u128 as f64;
_3 = [65388633975196252086094176178020796601_u128,47288389719741763251367865544679904551_u128,228601259352830277554023906014994838274_u128,175825591544805060238858730597575411045_u128,60811790074792448794292784183989282182_u128,152824275847396471057569496478073763368_u128,232820691030574569012078791120833541726_u128];
_12 = 7102_i16;
_7 = !true;
RET = _4 | _4;
_3 = [107232543710018278375369628074063066681_u128,3858146293332771882397661654229208034_u128,159478531616111128637800849287451082879_u128,158063786467467012325154430622619621828_u128,92490994503939935031900281278375534095_u128,148419239997750707880400083225873414810_u128,201433367788074958339202782094913657318_u128];
RET = _4 - _4;
_5 = RET;
_11 = 844111833_u32 as f64;
RET = !_4;
_9 = _4 as f32;
RET = !_5;
_5 = -_6;
_14 = &_11;
_13 = '\u{43bc5}';
Goto(bb2)
}
bb2 = {
Call(_18 = dump_var(11_usize, 13_usize, Move(_13), 3_usize, Move(_3), 7_usize, Move(_7), 12_usize, Move(_12)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_18 = dump_var(11_usize, 1_usize, Move(_1), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: [u128; 7],mut _7: isize,mut _8: [u128; 7],mut _9: isize,mut _10: isize,mut _11: isize) -> f32 {
mir! {
type RET = f32;
let _12: isize;
let _13: bool;
let _14: ();
let _15: ();
{
_11 = !_4;
_6 = [141417899514918843920910826462831570433_u128,270470529834398258601104227789552959232_u128,17054070992498719400774567955427653917_u128,71916776468353332054356219189885959586_u128,312725614018085143274752322499636851459_u128,277482705528740726948085310181827676199_u128,186651646924887367424224336252052383163_u128];
_5 = '\u{f9697}' as isize;
_2 = _7;
_11 = -_10;
_12 = 108_u8 as isize;
_11 = 6468103933030502113_usize as isize;
_7 = -_2;
_5 = _4;
_1 = _5 >> _9;
_5 = !_3;
_4 = _1 | _9;
_4 = _9;
_11 = !_9;
_4 = _1;
RET = 33_i8 as f32;
_7 = _10 + _4;
RET = 3078835011_u32 as f32;
_4 = -_7;
_7 = _4 * _10;
_7 = _5;
_1 = _4;
_5 = _4;
_3 = !_1;
_4 = !_10;
RET = _1 as f32;
_3 = -_1;
_13 = !true;
_2 = -_5;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(12_usize, 7_usize, Move(_7), 8_usize, Move(_8), 12_usize, Move(_12), 3_usize, Move(_3)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_14 = dump_var(12_usize, 1_usize, Move(_1), 13_usize, Move(_13), 15_usize, _15, 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: (&'static f64,),mut _2: char) -> *const usize {
mir! {
type RET = *const usize;
let _3: &'static (&'static f64,);
let _4: (i32, Adt22, *mut &'static f64, f64);
let _5: &'static (f32,);
let _6: [u128; 6];
let _7: (char, Adt27);
let _8: Adt27;
let _9: i128;
let _10: *const [i8; 7];
let _11: char;
let _12: (i128,);
let _13: *mut [u128; 6];
let _14: &'static f64;
let _15: *mut [usize; 8];
let _16: f32;
let _17: [bool; 4];
let _18: i32;
let _19: u64;
let _20: [u32; 2];
let _21: f32;
let _22: bool;
let _23: i128;
let _24: *mut [isize; 3];
let _25: char;
let _26: *mut u8;
let _27: u128;
let _28: *mut Adt48;
let _29: f64;
let _30: u16;
let _31: f64;
let _32: ((char, Adt27), isize);
let _33: f64;
let _34: *mut &'static u8;
let _35: *const [i8; 7];
let _36: [i16; 5];
let _37: (f32,);
let _38: ();
let _39: ();
{
_4.3 = 11657374169273170206_usize as f64;
_4.2 = core::ptr::addr_of_mut!(_1.0);
_4.1 = Adt22::Variant0 { fld0: 2538067868620229882_usize,fld1: 14792_u16 };
_4.1 = Adt22::Variant1 { fld0: false,fld1: _2,fld2: 3_usize,fld3: (-66006977645665191838790051059008004928_i128),fld4: 80319419525627133_u64 };
_4.1 = Adt22::Variant0 { fld0: 6_usize,fld1: 22076_u16 };
_4.0 = -(-1132820111_i32);
_4.2 = core::ptr::addr_of_mut!(_1.0);
_4.0 = 282458978654921625341524098975493535164_u128 as i32;
_4.1 = Adt22::Variant0 { fld0: 5_usize,fld1: 28468_u16 };
Goto(bb1)
}
bb1 = {
_4.1 = Adt22::Variant1 { fld0: true,fld1: _2,fld2: 11345117334238579285_usize,fld3: (-124312032195979191941971507700219412849_i128),fld4: 7478534593517332037_u64 };
_4.2 = core::ptr::addr_of_mut!(_1.0);
place!(Field::<bool>(Variant(_4.1, 1), 0)) = !true;
_3 = &_1;
RET = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4.1, 1), 2)));
(*RET) = (-26651295506128728257027838167225604175_i128) as usize;
_4.0 = -999852872_i32;
_6 = [249220288684202721742047412641583207158_u128,108866061660659194343571350506002284853_u128,282363594256597370143706264899245538884_u128,91602176239604307542116677773039387912_u128,182107316853628492410009110902773609839_u128,285448995464578939976650926362316315496_u128];
_1.0 = &_4.3;
place!(Field::<i128>(Variant(_4.1, 1), 3)) = (-43736277243335903736384923334963072496_i128);
_3 = &_1;
Goto(bb2)
}
bb2 = {
place!(Field::<usize>(Variant(_4.1, 1), 2)) = !6_usize;
(*RET) = 18673_i16 as usize;
_6 = [75078694536724309448309436249153790435_u128,293274956518229973177164679614052485136_u128,71194743130655938877415770904111616326_u128,69516331308511924679907728662385257681_u128,44170988827026088455662677117878531800_u128,287493791028553685541495176063558842526_u128];
(*RET) = !10197184366704145625_usize;
(*RET) = !6561664725510374407_usize;
_4.3 = Field::<usize>(Variant(_4.1, 1), 2) as f64;
(*RET) = 6_usize >> Field::<i128>(Variant(_4.1, 1), 3);
match Field::<i128>(Variant(_4.1, 1), 3) {
296546089677602559726989684096805138960 => bb4,
_ => bb3
}
}
bb3 = {
_4.1 = Adt22::Variant1 { fld0: true,fld1: _2,fld2: 11345117334238579285_usize,fld3: (-124312032195979191941971507700219412849_i128),fld4: 7478534593517332037_u64 };
_4.2 = core::ptr::addr_of_mut!(_1.0);
place!(Field::<bool>(Variant(_4.1, 1), 0)) = !true;
_3 = &_1;
RET = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4.1, 1), 2)));
(*RET) = (-26651295506128728257027838167225604175_i128) as usize;
_4.0 = -999852872_i32;
_6 = [249220288684202721742047412641583207158_u128,108866061660659194343571350506002284853_u128,282363594256597370143706264899245538884_u128,91602176239604307542116677773039387912_u128,182107316853628492410009110902773609839_u128,285448995464578939976650926362316315496_u128];
_1.0 = &_4.3;
place!(Field::<i128>(Variant(_4.1, 1), 3)) = (-43736277243335903736384923334963072496_i128);
_3 = &_1;
Goto(bb2)
}
bb4 = {
_1.0 = &_4.3;
(*RET) = 8925_u16 as usize;
RET = core::ptr::addr_of!((*RET));
(*RET) = _4.3 as usize;
(*RET) = 10709830076269692582_usize;
_4.0 = -555872596_i32;
_4.1 = Adt22::Variant0 { fld0: 3_usize,fld1: 3262_u16 };
place!(Field::<u16>(Variant(_4.1, 0), 1)) = _4.0 as u16;
_4.3 = 1305079783271175700_usize as f64;
place!(Field::<usize>(Variant(_4.1, 0), 0)) = !16763958706461818202_usize;
RET = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4.1, 0), 0)));
Goto(bb5)
}
bb5 = {
_1.0 = &_4.3;
RET = core::ptr::addr_of!((*RET));
_9 = (-47832096399354625091194940101279774689_i128) >> (*RET);
RET = core::ptr::addr_of!((*RET));
_1.0 = &_4.3;
_2 = '\u{6ce2}';
RET = core::ptr::addr_of!((*RET));
_7.0 = _2;
Call(_2 = fn14(Move(_4), _6, _6, _6, _7.0, _6, Move(RET), _6, _6, _9, _9, _6), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_4.2 = core::ptr::addr_of_mut!(_1.0);
_2 = _7.0;
Goto(bb7)
}
bb7 = {
_12.0 = _9;
_11 = _2;
_2 = _7.0;
_6 = [246952023717525145659372595895557023986_u128,301175364378000903868136403677675644428_u128,223473393051065804980291787663140666735_u128,248710297059975677873742264683799685941_u128,99483124244691553070071654974082838232_u128,337909755730796212543108189686636778943_u128];
_1.0 = &_4.3;
_12.0 = 2086352144_u32 as i128;
_1.0 = &_4.3;
_1.0 = &_4.3;
_7.0 = _11;
_13 = core::ptr::addr_of_mut!(_6);
_12 = (_9,);
_4.3 = 63_u8 as f64;
_12.0 = _9 * _9;
_2 = _7.0;
_3 = &_1;
_11 = _7.0;
_4.0 = 3100952073668789797_u64 as i32;
_14 = &_4.3;
_16 = (-113_i8) as f32;
_1.0 = &_4.3;
_12 = (_9,);
Goto(bb8)
}
bb8 = {
Goto(bb9)
}
bb9 = {
_4.1 = Adt22::Variant0 { fld0: 17603859067378215673_usize,fld1: 25568_u16 };
_3 = &_1;
_3 = &(*_3);
_1 = (Move(_14),);
_6 = [37558151166557926421521395149186972693_u128,148336871751826734176648570164193083736_u128,231054797260965362210808135525251728442_u128,155388785546876969757228743374107230238_u128,188990750218422218171439829286330231095_u128,42775776830500663256454674751056064618_u128];
_9 = _16 as i128;
_16 = 46843_u16 as f32;
_3 = &_1;
Goto(bb10)
}
bb10 = {
_4.2 = core::ptr::addr_of_mut!(_14);
_11 = _2;
_8 = Adt27::Variant0 { fld0: _4.0,fld1: _16,fld2: _4.3 };
_7 = (_11, Move(_8));
place!(Field::<u16>(Variant(_4.1, 0), 1)) = 4507_u16 + 7144_u16;
_17 = [true,true,false,false];
_14 = &_4.3;
place!(Field::<f64>(Variant(_7.1, 0), 2)) = (*_14) + (*_14);
_7.0 = _2;
SetDiscriminant(_7.1, 0);
_1 = (Move(_14),);
(*_13) = [281308415697935652715620708624929619788_u128,81218026070826122186192493731497206509_u128,328287709109754007653583285130137966302_u128,178696562212527802622514084574895411356_u128,74431818582378994442616730761584971844_u128,224324701754052528105423126377995956571_u128];
_21 = (-8036662577558349763_i64) as f32;
_14 = &place!(Field::<f64>(Variant(_7.1, 0), 2));
_19 = 834818732357584313_u64 << _4.0;
place!(Field::<f32>(Variant(_7.1, 0), 1)) = _21;
_17 = [true,true,false,false];
_4.0 = (-1642075052_i32);
RET = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4.1, 0), 0)));
_3 = &_1;
place!(Field::<i32>(Variant(_7.1, 0), 0)) = 213_u8 as i32;
place!(Field::<usize>(Variant(_4.1, 0), 0)) = !7090239115246521481_usize;
place!(Field::<f64>(Variant(_7.1, 0), 2)) = -_4.3;
Goto(bb11)
}
bb11 = {
(*RET) = 2_usize >> Field::<u16>(Variant(_4.1, 0), 1);
place!(Field::<u16>(Variant(_4.1, 0), 1)) = 39883_u16 & 56285_u16;
_6 = [142170067530022426761460246507040350720_u128,155428658532021017583346840555623604475_u128,303773022698477446308542866584888376841_u128,70289266028362972255459217133735496771_u128,36326993118253680316336005687016990121_u128,178407831803163794524454200739806745130_u128];
_16 = _21 - Field::<f32>(Variant(_7.1, 0), 1);
_17 = [false,true,false,false];
_7.0 = _2;
_16 = Field::<f64>(Variant(_7.1, 0), 2) as f32;
_8 = Move(_7.1);
(*RET) = !7_usize;
_2 = _7.0;
_7 = (_2, Move(_8));
_27 = 80438350506924913317571432894194231810_u128;
_20 = [2718948563_u32,2776395993_u32];
_9 = !_12.0;
_18 = Field::<i32>(Variant(_7.1, 0), 0);
match _4.0 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb5,
340282366920938463463374607430126136404 => bb13,
_ => bb12
}
}
bb12 = {
Goto(bb9)
}
bb13 = {
place!(Field::<usize>(Variant(_4.1, 0), 0)) = 9467938260368341708_usize - 8628391358443277406_usize;
SetDiscriminant(_7.1, 1);
place!(Field::<f32>(Variant(_7.1, 1), 4)) = -_21;
_13 = core::ptr::addr_of_mut!((*_13));
_15 = core::ptr::addr_of_mut!(place!(Field::<[usize; 8]>(Variant(_7.1, 1), 3)));
(*_13) = [_27,_27,_27,_27,_27,_27];
_29 = (-12528_i16) as f64;
_14 = &_31;
_15 = core::ptr::addr_of_mut!((*_15));
SetDiscriminant(_4.1, 0);
_29 = 36_isize as f64;
place!(Field::<isize>(Variant(_7.1, 1), 2)) = 9223372036854775807_isize << _18;
_19 = 11171_i16 as u64;
place!(Field::<usize>(Variant(_4.1, 0), 0)) = !2_usize;
_1.0 = &(*_14);
_25 = _7.0;
(*_15) = [Field::<usize>(Variant(_4.1, 0), 0),Field::<usize>(Variant(_4.1, 0), 0),Field::<usize>(Variant(_4.1, 0), 0),Field::<usize>(Variant(_4.1, 0), 0),Field::<usize>(Variant(_4.1, 0), 0),Field::<usize>(Variant(_4.1, 0), 0),Field::<usize>(Variant(_4.1, 0), 0),Field::<usize>(Variant(_4.1, 0), 0)];
match _4.0 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
5 => bb19,
6 => bb20,
340282366920938463463374607430126136404 => bb22,
_ => bb21
}
}
bb14 = {
_4.1 = Adt22::Variant1 { fld0: true,fld1: _2,fld2: 11345117334238579285_usize,fld3: (-124312032195979191941971507700219412849_i128),fld4: 7478534593517332037_u64 };
_4.2 = core::ptr::addr_of_mut!(_1.0);
place!(Field::<bool>(Variant(_4.1, 1), 0)) = !true;
_3 = &_1;
RET = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4.1, 1), 2)));
(*RET) = (-26651295506128728257027838167225604175_i128) as usize;
_4.0 = -999852872_i32;
_6 = [249220288684202721742047412641583207158_u128,108866061660659194343571350506002284853_u128,282363594256597370143706264899245538884_u128,91602176239604307542116677773039387912_u128,182107316853628492410009110902773609839_u128,285448995464578939976650926362316315496_u128];
_1.0 = &_4.3;
place!(Field::<i128>(Variant(_4.1, 1), 3)) = (-43736277243335903736384923334963072496_i128);
_3 = &_1;
Goto(bb2)
}
bb15 = {
(*RET) = 2_usize >> Field::<u16>(Variant(_4.1, 0), 1);
place!(Field::<u16>(Variant(_4.1, 0), 1)) = 39883_u16 & 56285_u16;
_6 = [142170067530022426761460246507040350720_u128,155428658532021017583346840555623604475_u128,303773022698477446308542866584888376841_u128,70289266028362972255459217133735496771_u128,36326993118253680316336005687016990121_u128,178407831803163794524454200739806745130_u128];
_16 = _21 - Field::<f32>(Variant(_7.1, 0), 1);
_17 = [false,true,false,false];
_7.0 = _2;
_16 = Field::<f64>(Variant(_7.1, 0), 2) as f32;
_8 = Move(_7.1);
(*RET) = !7_usize;
_2 = _7.0;
_7 = (_2, Move(_8));
_27 = 80438350506924913317571432894194231810_u128;
_20 = [2718948563_u32,2776395993_u32];
_9 = !_12.0;
_18 = Field::<i32>(Variant(_7.1, 0), 0);
match _4.0 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb5,
340282366920938463463374607430126136404 => bb13,
_ => bb12
}
}
bb16 = {
_4.2 = core::ptr::addr_of_mut!(_14);
_11 = _2;
_8 = Adt27::Variant0 { fld0: _4.0,fld1: _16,fld2: _4.3 };
_7 = (_11, Move(_8));
place!(Field::<u16>(Variant(_4.1, 0), 1)) = 4507_u16 + 7144_u16;
_17 = [true,true,false,false];
_14 = &_4.3;
place!(Field::<f64>(Variant(_7.1, 0), 2)) = (*_14) + (*_14);
_7.0 = _2;
SetDiscriminant(_7.1, 0);
_1 = (Move(_14),);
(*_13) = [281308415697935652715620708624929619788_u128,81218026070826122186192493731497206509_u128,328287709109754007653583285130137966302_u128,178696562212527802622514084574895411356_u128,74431818582378994442616730761584971844_u128,224324701754052528105423126377995956571_u128];
_21 = (-8036662577558349763_i64) as f32;
_14 = &place!(Field::<f64>(Variant(_7.1, 0), 2));
_19 = 834818732357584313_u64 << _4.0;
place!(Field::<f32>(Variant(_7.1, 0), 1)) = _21;
_17 = [true,true,false,false];
_4.0 = (-1642075052_i32);
RET = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4.1, 0), 0)));
_3 = &_1;
place!(Field::<i32>(Variant(_7.1, 0), 0)) = 213_u8 as i32;
place!(Field::<usize>(Variant(_4.1, 0), 0)) = !7090239115246521481_usize;
place!(Field::<f64>(Variant(_7.1, 0), 2)) = -_4.3;
Goto(bb11)
}
bb17 = {
_4.1 = Adt22::Variant1 { fld0: true,fld1: _2,fld2: 11345117334238579285_usize,fld3: (-124312032195979191941971507700219412849_i128),fld4: 7478534593517332037_u64 };
_4.2 = core::ptr::addr_of_mut!(_1.0);
place!(Field::<bool>(Variant(_4.1, 1), 0)) = !true;
_3 = &_1;
RET = core::ptr::addr_of!(place!(Field::<usize>(Variant(_4.1, 1), 2)));
(*RET) = (-26651295506128728257027838167225604175_i128) as usize;
_4.0 = -999852872_i32;
_6 = [249220288684202721742047412641583207158_u128,108866061660659194343571350506002284853_u128,282363594256597370143706264899245538884_u128,91602176239604307542116677773039387912_u128,182107316853628492410009110902773609839_u128,285448995464578939976650926362316315496_u128];
_1.0 = &_4.3;
place!(Field::<i128>(Variant(_4.1, 1), 3)) = (-43736277243335903736384923334963072496_i128);
_3 = &_1;
Goto(bb2)
}
bb18 = {
Goto(bb9)
}
bb19 = {
_12.0 = _9;
_11 = _2;
_2 = _7.0;
_6 = [246952023717525145659372595895557023986_u128,301175364378000903868136403677675644428_u128,223473393051065804980291787663140666735_u128,248710297059975677873742264683799685941_u128,99483124244691553070071654974082838232_u128,337909755730796212543108189686636778943_u128];
_1.0 = &_4.3;
_12.0 = 2086352144_u32 as i128;
_1.0 = &_4.3;
_1.0 = &_4.3;
_7.0 = _11;
_13 = core::ptr::addr_of_mut!(_6);
_12 = (_9,);
_4.3 = 63_u8 as f64;
_12.0 = _9 * _9;
_2 = _7.0;
_3 = &_1;
_11 = _7.0;
_4.0 = 3100952073668789797_u64 as i32;
_14 = &_4.3;
_16 = (-113_i8) as f32;
_1.0 = &_4.3;
_12 = (_9,);
Goto(bb8)
}
bb20 = {
_4.2 = core::ptr::addr_of_mut!(_1.0);
_2 = _7.0;
Goto(bb7)
}
bb21 = {
_1.0 = &_4.3;
RET = core::ptr::addr_of!((*RET));
_9 = (-47832096399354625091194940101279774689_i128) >> (*RET);
RET = core::ptr::addr_of!((*RET));
_1.0 = &_4.3;
_2 = '\u{6ce2}';
RET = core::ptr::addr_of!((*RET));
_7.0 = _2;
Call(_2 = fn14(Move(_4), _6, _6, _6, _7.0, _6, Move(RET), _6, _6, _9, _9, _6), ReturnTo(bb6), UnwindUnreachable())
}
bb22 = {
_31 = _4.3;
_23 = _12.0;
_12 = (_23,);
_27 = true as u128;
place!(Field::<[usize; 8]>(Variant(_7.1, 1), 3)) = [Field::<usize>(Variant(_4.1, 0), 0),Field::<usize>(Variant(_4.1, 0), 0),Field::<usize>(Variant(_4.1, 0), 0),Field::<usize>(Variant(_4.1, 0), 0),Field::<usize>(Variant(_4.1, 0), 0),Field::<usize>(Variant(_4.1, 0), 0),Field::<usize>(Variant(_4.1, 0), 0),Field::<usize>(Variant(_4.1, 0), 0)];
_32.1 = !Field::<isize>(Variant(_7.1, 1), 2);
_36 = [(-24581_i16),12906_i16,7097_i16,(-10658_i16),(-11672_i16)];
place!(Field::<usize>(Variant(_4.1, 0), 0)) = 8970434765217297174_usize | 9040042957685036052_usize;
_32.1 = !Field::<isize>(Variant(_7.1, 1), 2);
_14 = &_4.3;
_20 = [3112757849_u32,205297787_u32];
_32.0.0 = _7.0;
_4.1 = Adt22::Variant0 { fld0: 6_usize,fld1: 35700_u16 };
_5 = &_37;
_20 = [678278819_u32,438612150_u32];
(*_15) = [16339919015854697711_usize,6988924140725158509_usize,5_usize,4272435185457718833_usize,3_usize,4_usize,2_usize,1_usize];
_21 = -Field::<f32>(Variant(_7.1, 1), 4);
_37 = (Field::<f32>(Variant(_7.1, 1), 4),);
place!(Field::<usize>(Variant(_4.1, 0), 0)) = 183_u8 as usize;
Goto(bb23)
}
bb23 = {
Call(_38 = dump_var(13_usize, 19_usize, Move(_19), 23_usize, Move(_23), 12_usize, Move(_12), 25_usize, Move(_25)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_38 = dump_var(13_usize, 36_usize, Move(_36), 18_usize, Move(_18), 39_usize, _39, 39_usize, _39), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: (i32, Adt22, *mut &'static f64, f64),mut _2: [u128; 6],mut _3: [u128; 6],mut _4: [u128; 6],mut _5: char,mut _6: [u128; 6],mut _7: *const usize,mut _8: [u128; 6],mut _9: [u128; 6],mut _10: i128,mut _11: i128,mut _12: [u128; 6]) -> char {
mir! {
type RET = char;
let _13: [isize; 3];
let _14: isize;
let _15: bool;
let _16: (&'static (&'static f64,), Adt34);
let _17: i64;
let _18: [isize; 4];
let _19: isize;
let _20: (u32,);
let _21: u64;
let _22: f32;
let _23: f64;
let _24: f32;
let _25: (&'static *mut u8, bool);
let _26: isize;
let _27: ();
let _28: ();
{
_2 = [240133064079967662941450498523339559955_u128,338917035163355117295454081528516751130_u128,170934542764027505811448924115454838738_u128,286015949747862362357850091211076414111_u128,41884115763238949723797894987252042297_u128,178019613780248232463890783410882907069_u128];
Goto(bb1)
}
bb1 = {
SetDiscriminant(_1.1, 1);
_7 = core::ptr::addr_of!(place!(Field::<usize>(Variant(_1.1, 1), 2)));
RET = _5;
_1.3 = 109432264407796011853378435894942988674_u128 as f64;
Call(place!(Field::<u64>(Variant(_1.1, 1), 4)) = fn15(Move(_1.2), Move(_7), _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = RET;
place!(Field::<char>(Variant(_1.1, 1), 1)) = RET;
place!(Field::<char>(Variant(_1.1, 1), 1)) = RET;
_14 = 9223372036854775807_isize;
_1.1 = Adt22::Variant1 { fld0: false,fld1: RET,fld2: 0_usize,fld3: _10,fld4: 4286238731576359420_u64 };
_13 = [_14,_14,_14];
_8 = _6;
_9 = _6;
place!(Field::<bool>(Variant(_1.1, 1), 0)) = false ^ true;
place!(Field::<char>(Variant(_1.1, 1), 1)) = _5;
_5 = Field::<char>(Variant(_1.1, 1), 1);
RET = _5;
_14 = 42406_u16 as isize;
Goto(bb3)
}
bb3 = {
_14 = 200_u8 as isize;
_14 = 50_isize;
_5 = Field::<char>(Variant(_1.1, 1), 1);
_2 = _3;
_7 = core::ptr::addr_of!(place!(Field::<usize>(Variant(_1.1, 1), 2)));
_7 = core::ptr::addr_of!((*_7));
_15 = !Field::<bool>(Variant(_1.1, 1), 0);
_8 = [269839251425562768645819953779267368931_u128,129201632050038190007199584265808081042_u128,159175566355657539401111479001030468888_u128,36120914681372492213358063389945115646_u128,29712927787895789788899589882829611383_u128,51130457838949642659547248260344513225_u128];
Goto(bb4)
}
bb4 = {
_11 = !_10;
_16.1 = Adt34::Variant1 { fld0: 59113_u16,fld1: RET,fld2: _14,fld3: 4363236106609193432_usize,fld4: 99174026633828274_u64 };
(*_7) = 0_usize | 5_usize;
place!(Field::<usize>(Variant(_16.1, 1), 3)) = (*_7);
_3 = [182518086372443720295515302989769249961_u128,139507335849270557661107854592247073931_u128,230917351163454790714885467505094714164_u128,197712980150120895333808912954118731296_u128,118029290525975987182244471487279463452_u128,93715173419050609462920379844541476633_u128];
place!(Field::<usize>(Variant(_16.1, 1), 3)) = Field::<char>(Variant(_1.1, 1), 1) as usize;
_17 = 7533571287047445439_i64;
_11 = 1821038375_u32 as i128;
(*_7) = 214489783094587368712229835405509366247_u128 as usize;
_13 = [Field::<isize>(Variant(_16.1, 1), 2),Field::<isize>(Variant(_16.1, 1), 2),_14];
_11 = (-11_i8) as i128;
_12 = [292441097689767004135860804991687015902_u128,181379916660768462955651149306261864551_u128,12037856115131246506043372525003011621_u128,210870254959124914980537272220705901785_u128,4759661580045175638552547485963637267_u128,74235906237974298461410083601154210640_u128];
place!(Field::<isize>(Variant(_16.1, 1), 2)) = 474161331761298309_u64 as isize;
_18 = [_14,Field::<isize>(Variant(_16.1, 1), 2),_14,Field::<isize>(Variant(_16.1, 1), 2)];
place!(Field::<u16>(Variant(_16.1, 1), 0)) = !13553_u16;
place!(Field::<u16>(Variant(_16.1, 1), 0)) = 38368_u16;
place!(Field::<u64>(Variant(_1.1, 1), 4)) = 1448217190301533664_u64 >> Field::<usize>(Variant(_16.1, 1), 3);
_5 = RET;
Goto(bb5)
}
bb5 = {
place!(Field::<usize>(Variant(_1.1, 1), 2)) = 270161521318067539985918489951847802414_u128 as usize;
_18 = [_14,Field::<isize>(Variant(_16.1, 1), 2),_14,_14];
place!(Field::<i128>(Variant(_1.1, 1), 3)) = Field::<u16>(Variant(_16.1, 1), 0) as i128;
(*_7) = Field::<usize>(Variant(_16.1, 1), 3) - Field::<usize>(Variant(_16.1, 1), 3);
place!(Field::<char>(Variant(_16.1, 1), 1)) = RET;
(*_7) = Field::<usize>(Variant(_16.1, 1), 3) << _17;
_9 = _6;
place!(Field::<char>(Variant(_1.1, 1), 1)) = Field::<char>(Variant(_16.1, 1), 1);
place!(Field::<usize>(Variant(_16.1, 1), 3)) = (*_7) << (*_7);
place!(Field::<usize>(Variant(_1.1, 1), 2)) = Field::<usize>(Variant(_16.1, 1), 3) ^ Field::<usize>(Variant(_16.1, 1), 3);
SetDiscriminant(_1.1, 1);
Call(_13 = fn16(_1.0, _3, _8, _8, Field::<usize>(Variant(_16.1, 1), 3), _8, _2, _3, _8, _12, Field::<usize>(Variant(_16.1, 1), 3)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_1.3 = _17 as f64;
_14 = Field::<usize>(Variant(_16.1, 1), 3) as isize;
_1.0 = -1811275965_i32;
_2 = _8;
_19 = _14;
Goto(bb7)
}
bb7 = {
_8 = [63525408622650264483741208040133044908_u128,140015092172369684117401808528568350597_u128,100102870860135778589360658186478333629_u128,202717648246519771405038289140637782375_u128,165606332608307844861535786429602720750_u128,32457820887637074921267903826166717713_u128];
place!(Field::<i128>(Variant(_1.1, 1), 3)) = _10 | _10;
place!(Field::<isize>(Variant(_16.1, 1), 2)) = _17 as isize;
place!(Field::<bool>(Variant(_1.1, 1), 0)) = _15;
match Field::<u16>(Variant(_16.1, 1), 0) {
0 => bb4,
1 => bb8,
38368 => bb10,
_ => bb9
}
}
bb8 = {
_11 = !_10;
_16.1 = Adt34::Variant1 { fld0: 59113_u16,fld1: RET,fld2: _14,fld3: 4363236106609193432_usize,fld4: 99174026633828274_u64 };
(*_7) = 0_usize | 5_usize;
place!(Field::<usize>(Variant(_16.1, 1), 3)) = (*_7);
_3 = [182518086372443720295515302989769249961_u128,139507335849270557661107854592247073931_u128,230917351163454790714885467505094714164_u128,197712980150120895333808912954118731296_u128,118029290525975987182244471487279463452_u128,93715173419050609462920379844541476633_u128];
place!(Field::<usize>(Variant(_16.1, 1), 3)) = Field::<char>(Variant(_1.1, 1), 1) as usize;
_17 = 7533571287047445439_i64;
_11 = 1821038375_u32 as i128;
(*_7) = 214489783094587368712229835405509366247_u128 as usize;
_13 = [Field::<isize>(Variant(_16.1, 1), 2),Field::<isize>(Variant(_16.1, 1), 2),_14];
_11 = (-11_i8) as i128;
_12 = [292441097689767004135860804991687015902_u128,181379916660768462955651149306261864551_u128,12037856115131246506043372525003011621_u128,210870254959124914980537272220705901785_u128,4759661580045175638552547485963637267_u128,74235906237974298461410083601154210640_u128];
place!(Field::<isize>(Variant(_16.1, 1), 2)) = 474161331761298309_u64 as isize;
_18 = [_14,Field::<isize>(Variant(_16.1, 1), 2),_14,Field::<isize>(Variant(_16.1, 1), 2)];
place!(Field::<u16>(Variant(_16.1, 1), 0)) = !13553_u16;
place!(Field::<u16>(Variant(_16.1, 1), 0)) = 38368_u16;
place!(Field::<u64>(Variant(_1.1, 1), 4)) = 1448217190301533664_u64 >> Field::<usize>(Variant(_16.1, 1), 3);
_5 = RET;
Goto(bb5)
}
bb9 = {
_14 = 200_u8 as isize;
_14 = 50_isize;
_5 = Field::<char>(Variant(_1.1, 1), 1);
_2 = _3;
_7 = core::ptr::addr_of!(place!(Field::<usize>(Variant(_1.1, 1), 2)));
_7 = core::ptr::addr_of!((*_7));
_15 = !Field::<bool>(Variant(_1.1, 1), 0);
_8 = [269839251425562768645819953779267368931_u128,129201632050038190007199584265808081042_u128,159175566355657539401111479001030468888_u128,36120914681372492213358063389945115646_u128,29712927787895789788899589882829611383_u128,51130457838949642659547248260344513225_u128];
Goto(bb4)
}
bb10 = {
place!(Field::<char>(Variant(_1.1, 1), 1)) = _5;
_21 = !11753248754750471644_u64;
_20.0 = 1626212794_u32 << _19;
_12 = [41565251917138843439788862898179878969_u128,26916495486730179477167723585659912736_u128,183335198523425769254175348306349258060_u128,11403476250457737684052946404693583492_u128,338833504609150089926964967396314034323_u128,264313752743825033108682369344970953971_u128];
_1.1 = Adt22::Variant1 { fld0: _15,fld1: RET,fld2: Field::<usize>(Variant(_16.1, 1), 3),fld3: _10,fld4: _21 };
_1.0 = !807337449_i32;
SetDiscriminant(_1.1, 1);
_19 = -Field::<isize>(Variant(_16.1, 1), 2);
place!(Field::<bool>(Variant(_1.1, 1), 0)) = !_15;
_10 = 57_u8 as i128;
_18 = [_14,_14,Field::<isize>(Variant(_16.1, 1), 2),_14];
place!(Field::<u64>(Variant(_16.1, 1), 4)) = _21;
_19 = _20.0 as isize;
_1.1 = Adt22::Variant1 { fld0: _15,fld1: Field::<char>(Variant(_16.1, 1), 1),fld2: Field::<usize>(Variant(_16.1, 1), 3),fld3: _11,fld4: Field::<u64>(Variant(_16.1, 1), 4) };
RET = Field::<char>(Variant(_1.1, 1), 1);
_3 = _8;
_23 = _1.3;
RET = Field::<char>(Variant(_16.1, 1), 1);
_1.3 = _23;
SetDiscriminant(_16.1, 1);
place!(Field::<usize>(Variant(_16.1, 1), 3)) = Field::<usize>(Variant(_1.1, 1), 2);
place!(Field::<char>(Variant(_16.1, 1), 1)) = RET;
match _17 {
0 => bb11,
1 => bb12,
7533571287047445439 => bb14,
_ => bb13
}
}
bb11 = {
_1.3 = _17 as f64;
_14 = Field::<usize>(Variant(_16.1, 1), 3) as isize;
_1.0 = -1811275965_i32;
_2 = _8;
_19 = _14;
Goto(bb7)
}
bb12 = {
_14 = 200_u8 as isize;
_14 = 50_isize;
_5 = Field::<char>(Variant(_1.1, 1), 1);
_2 = _3;
_7 = core::ptr::addr_of!(place!(Field::<usize>(Variant(_1.1, 1), 2)));
_7 = core::ptr::addr_of!((*_7));
_15 = !Field::<bool>(Variant(_1.1, 1), 0);
_8 = [269839251425562768645819953779267368931_u128,129201632050038190007199584265808081042_u128,159175566355657539401111479001030468888_u128,36120914681372492213358063389945115646_u128,29712927787895789788899589882829611383_u128,51130457838949642659547248260344513225_u128];
Goto(bb4)
}
bb13 = {
SetDiscriminant(_1.1, 1);
_7 = core::ptr::addr_of!(place!(Field::<usize>(Variant(_1.1, 1), 2)));
RET = _5;
_1.3 = 109432264407796011853378435894942988674_u128 as f64;
Call(place!(Field::<u64>(Variant(_1.1, 1), 4)) = fn15(Move(_1.2), Move(_7), _4), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_13 = [_19,_19,_19];
_12 = [188098781633633629178350998765599815674_u128,329126553026191439472591199526981599055_u128,214261082251144996266288286697773145983_u128,340202690254772417651392297484643615697_u128,28886004929922187634335587042278418405_u128,27968987238936863862472348033077899886_u128];
_11 = _10;
place!(Field::<u64>(Variant(_16.1, 1), 4)) = _21 | _21;
_1.3 = _23 - _23;
_24 = _1.0 as f32;
_14 = _19;
_20 = (3229083094_u32,);
place!(Field::<u64>(Variant(_16.1, 1), 4)) = Field::<u64>(Variant(_1.1, 1), 4) * _21;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(14_usize, 9_usize, Move(_9), 5_usize, Move(_5), 10_usize, Move(_10), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(14_usize, 3_usize, Move(_3), 8_usize, Move(_8), 21_usize, Move(_21), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_27 = dump_var(14_usize, 15_usize, Move(_15), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: *mut &'static f64,mut _2: *const usize,mut _3: [u128; 6]) -> u64 {
mir! {
type RET = u64;
let _4: [isize; 4];
let _5: [usize; 1];
let _6: i8;
let _7: isize;
let _8: u128;
let _9: isize;
let _10: f32;
let _11: &'static usize;
let _12: [u128; 7];
let _13: (*const (&'static (&'static f64,), Adt34),);
let _14: (*mut &'static u8, f64, i16);
let _15: f32;
let _16: bool;
let _17: bool;
let _18: [i32; 4];
let _19: usize;
let _20: i8;
let _21: ();
let _22: ();
{
RET = 2742516977022639747_u64 & 1082147048379959647_u64;
RET = 16198881561861676398_u64;
RET = !6790162803034311202_u64;
Goto(bb1)
}
bb1 = {
_3 = [329397820077421664069796858494471798795_u128,221454954202140345165348018182514891598_u128,188815293389759599879573719996330934294_u128,27884429834251232112130027820705886747_u128,234180419698157661496976835566711956552_u128,327157519574277539138287326053027526630_u128];
_3 = [73048326555727218889458024480667621195_u128,141980814242547163369253512891366949199_u128,234975263062695204378069587232695400820_u128,132398601364993304345023243713761303689_u128,302147623590167182414489826816768919323_u128,77767686377056088626908667543219352270_u128];
_4 = [(-4_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
RET = 10501136500368662476_u64;
_5 = [13274976036959652443_usize];
_6 = 73_i8;
_6 = 57_i8;
_7 = 8_isize * 9223372036854775807_isize;
_8 = 283786521795713235476820186382384818955_u128;
_6 = (-124_i8) * 42_i8;
RET = 17181053951254839237_u64 & 17753297823224591211_u64;
_6 = (-103_i8) * 61_i8;
match _8 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
283786521795713235476820186382384818955 => bb7,
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
_7 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_4 = [_7,_7,_7,_7];
RET = !14151188979188179733_u64;
RET = 8522659214711368777_u64 ^ 12217654709531429680_u64;
_9 = -_7;
RET = 4451256900628549782_u64;
_4 = [_9,_7,_7,_7];
Call(_7 = core::intrinsics::bswap(_9), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_7 = _9 ^ _9;
_7 = -_9;
_10 = _6 as f32;
RET = 495854164872651876_u64;
_6 = (-110_i8);
RET = 3483286734340394985_u64;
_8 = !58281904091734747861349388059101152881_u128;
_7 = -_9;
RET = !12483982228751771584_u64;
_7 = _9;
_7 = _9 ^ _9;
_10 = _8 as f32;
_5 = [8148599651155970611_usize];
_3 = [_8,_8,_8,_8,_8,_8];
_10 = 643029619_i32 as f32;
_9 = !_7;
_12 = [_8,_8,_8,_8,_8,_8,_8];
_10 = _7 as f32;
RET = !12292569190570806830_u64;
_7 = _9 >> RET;
_10 = 320629632_i32 as f32;
RET = 572119281983047555_u64;
_10 = 2969841500_u32 as f32;
RET = (-27960_i16) as u64;
RET = !4184538123199946872_u64;
_6 = -(-20_i8);
Goto(bb9)
}
bb9 = {
_5 = [2_usize];
_5 = [2_usize];
_12 = [_8,_8,_8,_8,_8,_8,_8];
_3 = [_8,_8,_8,_8,_8,_8];
_7 = _9 * _9;
_14.1 = _10 as f64;
_6 = _14.1 as i8;
_15 = _10 * _10;
_9 = '\u{65b20}' as isize;
Goto(bb10)
}
bb10 = {
_16 = !false;
_14.2 = !30640_i16;
_14.2 = !1299_i16;
RET = 13057639789944220311_usize as u64;
_14.2 = (-21063_i16) ^ 6840_i16;
_3 = [_8,_8,_8,_8,_8,_8];
_9 = _7 - _7;
_7 = _9 * _9;
_15 = _10;
_7 = -_9;
_16 = true ^ false;
_10 = _15;
_6 = -47_i8;
_14.2 = 7170452538932701844_i64 as i16;
_15 = -_10;
_9 = _7;
_14.1 = 7765940947581902046_i64 as f64;
_16 = false;
_14.1 = _7 as f64;
_15 = _10 + _10;
_5 = [1934538173097312999_usize];
_8 = 339018763543289703513379513708657997006_u128;
_12 = [_8,_8,_8,_8,_8,_8,_8];
Goto(bb11)
}
bb11 = {
_8 = !84952636705311944984622954272539999424_u128;
_6 = (-35_i8);
_5 = [3_usize];
_19 = 4_usize - 6_usize;
_17 = _16;
_9 = _7 - _7;
_2 = core::ptr::addr_of!(_19);
_7 = _9 & _9;
_17 = _16;
_4 = [_7,_7,_7,_9];
_14.2 = (-27456_i16) << _7;
_19 = _14.1 as usize;
(*_2) = !15869895889553688710_usize;
_4 = [_7,_7,_7,_7];
RET = _7 as u64;
(*_2) = !4_usize;
_20 = _6;
(*_2) = '\u{97eb}' as usize;
_2 = core::ptr::addr_of!((*_2));
_14.2 = 153764007184605472676802112511554507520_i128 as i16;
_6 = _20;
_18 = [949466325_i32,1802848146_i32,(-2023140295_i32),1191045235_i32];
(*_2) = 4_usize;
_7 = _9;
Goto(bb12)
}
bb12 = {
Call(_21 = dump_var(15_usize, 7_usize, Move(_7), 19_usize, Move(_19), 18_usize, Move(_18), 12_usize, Move(_12)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_21 = dump_var(15_usize, 6_usize, Move(_6), 3_usize, Move(_3), 22_usize, _22, 22_usize, _22), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: i32,mut _2: [u128; 6],mut _3: [u128; 6],mut _4: [u128; 6],mut _5: usize,mut _6: [u128; 6],mut _7: [u128; 6],mut _8: [u128; 6],mut _9: [u128; 6],mut _10: [u128; 6],mut _11: usize) -> [isize; 3] {
mir! {
type RET = [isize; 3];
let _12: (char, Adt27);
let _13: &'static f64;
let _14: isize;
let _15: [i32; 4];
let _16: bool;
let _17: *mut [u128; 6];
let _18: [usize; 8];
let _19: Adt33;
let _20: isize;
let _21: bool;
let _22: i32;
let _23: f64;
let _24: u32;
let _25: &'static i32;
let _26: [i8; 7];
let _27: *mut &'static f64;
let _28: (char, Adt27);
let _29: [u128; 6];
let _30: f32;
let _31: *mut [isize; 3];
let _32: isize;
let _33: (i128,);
let _34: &'static [u128; 7];
let _35: &'static (&'static f64,);
let _36: bool;
let _37: u16;
let _38: *const usize;
let _39: [i16; 5];
let _40: ();
let _41: ();
{
_4 = [215378028505887729706811944794638125219_u128,223889290341246560958786163548965496104_u128,322712393243146752010579034009373420867_u128,231277221261471576211493177960187026888_u128,325766011260094690862884074135177580267_u128,80103109374461178420140392254135047118_u128];
_7 = _6;
_1 = 1996645536_i32;
_12.0 = '\u{f0a8a}';
_9 = [76486146768112896225269726253798507700_u128,33493379597091911027164746432857158028_u128,278391689751347808308125270443933712133_u128,66290349535615348703383806759358394686_u128,178650775060244229428838941702405419160_u128,295816155704556020819262280939058696571_u128];
_2 = [203992348961441655151854253750121487058_u128,317034059893216391559966289519399841898_u128,138421045593599403720396159055961053155_u128,83141246246648510722457093092423731338_u128,188026285396831860333495077333071436161_u128,99191205639759592558200988978975357313_u128];
_1 = !1873426362_i32;
_8 = [325622823592231730148283296703531141006_u128,308019695189887557599427346642766657046_u128,57202108824695358851175191911226624884_u128,15935869081013497705992221910299595001_u128,32946619516686407737528703550491754769_u128,334176853200979751082742121193292359751_u128];
RET = [(-9223372036854775808_isize),9223372036854775807_isize,64_isize];
_4 = [39149897281534731829085232298541942445_u128,8026828961162029705540331232215457579_u128,195480688406257570188917773873224465578_u128,312280746876342430414455646163618150094_u128,131325734110971519167846128752031410784_u128,84308123051940110419558265054552423001_u128];
_14 = (-9223372036854775808_isize);
_11 = _5;
_2 = [49419674369439140137891810752814769088_u128,116920190085865252622842210799124828678_u128,69362543357947287732736963090713913186_u128,1793481728073641808292408040361958614_u128,318672582226712773289327338707867727949_u128,16348288708938093477466959571702627304_u128];
RET = [_14,_14,_14];
_8 = [306462070814580198215571022675495342142_u128,95641476016188572849352051380226070746_u128,5096113174585572777963725195557058212_u128,150372436540842605873780971277161418144_u128,164586408174037866461860083319794274446_u128,138950414584718428411283829362592936207_u128];
_2 = _8;
_16 = true;
_15 = [_1,_1,_1,_1];
_2 = [126445494027893462644267810493655203955_u128,231176902255804016612908605222329912252_u128,21767362194864100577342173778921531473_u128,95441064404001791944821103256986268570_u128,190121310540737750177277902035393391383_u128,208759456719609275030023388900685582859_u128];
_7 = [249049619303393073627733528758146941847_u128,3101557108460830163802880684188493196_u128,277755696696228976827748087146657244621_u128,237684293490051163357470318990807554162_u128,141277630624104354416254469298305856226_u128,94646653482051839558043878917738345718_u128];
match _14 {
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
_4 = [334683615415529769864595272028614281767_u128,58136121759011396649157879216204273258_u128,151208450546448245447867749538296184989_u128,129508453543958798543619532222787519419_u128,77075141233564113473765525733539036260_u128,82923290289305434272797425485418998212_u128];
_4 = [190810147030841583241962882785768675884_u128,94212401382497926145892250694194870171_u128,157991125031229898656552858970828535049_u128,86238433971690490575108502671421881641_u128,273406691265489441028778030553613270650_u128,205540776874424894701926721075187363536_u128];
_7 = [316533797132242338147928403941294078787_u128,272695588439087797844138134938102711332_u128,11956789313650952642165956881635364675_u128,161180989476008143236345094365381306151_u128,77809684600641715827147068665944638141_u128,169038951712823834906197641425384208194_u128];
RET = [_14,_14,_14];
_2 = [95778369140150871910507352279606120455_u128,112713222039707343606368968852605891269_u128,37101541489804048438064741849606288621_u128,156690446239859350356102742203720615992_u128,44078716326591627932907705109103487892_u128,5230299661267392295544868732147249194_u128];
_17 = core::ptr::addr_of_mut!(_4);
_1 = 1563958853_i32;
RET = [_14,_14,_14];
_15 = [_1,_1,_1,_1];
match _14 {
0 => bb1,
1 => bb2,
340282366920938463454151235394913435648 => bb5,
_ => bb4
}
}
bb4 = {
Return()
}
bb5 = {
_1 = !(-1751426603_i32);
_5 = !_11;
_12.0 = '\u{1261d}';
_6 = [293642619690534427759565522614835572530_u128,132683572259533398483187889229540952415_u128,253876688574238980435239545512710839959_u128,324719149878910989418116841419174945125_u128,235984072990024647837539050481858220097_u128,298787411642579825980752565565026737944_u128];
_5 = !_11;
_3 = _7;
_6 = _4;
(*_17) = [282636701403222189839336233598168834581_u128,481408831480822994589203284990652854_u128,44076085899954108894125702898600089489_u128,19581825129471839265090437565899075599_u128,320058396796813142606863138953364466457_u128,18956826838015688574639076439121749894_u128];
_9 = [49347276877144614087772001083544120413_u128,226775075736636571364121655588889659341_u128,252285905693419464626591153004280894975_u128,122633523431708853440165298586729896676_u128,284869208955764302327433441836470424155_u128,105568281050701961910970722335520728573_u128];
_3 = [172856249629183455188519341336626979824_u128,1080520669472209601308564049530240571_u128,134281083575309001585021183175826794505_u128,230145777030732203034467766630519795204_u128,241560892633780766432295754026795086650_u128,182486317478100137117020827639039851094_u128];
_10 = [158310744276860285295333481872201501509_u128,176479771532939161385813092474254994714_u128,168316912315685426579746292744454509115_u128,91641248389457248368536610835690941016_u128,73907107203302424546727629716971540933_u128,338859562972447057427318073787751860663_u128];
_17 = core::ptr::addr_of_mut!(_7);
Goto(bb6)
}
bb6 = {
_20 = !_14;
_20 = 55166315658939472011587867000658480921_i128 as isize;
_6 = [223486640660779742768126251014412290500_u128,191172115534281635219496667882477580545_u128,238974405248211433910091444744695836395_u128,104243818415782003471045902633266667011_u128,228985932630405434004076201828239759589_u128,108725146947323375965556390297711143441_u128];
_1 = -2067806524_i32;
_3 = [178395306678883199607251161581305640315_u128,249883990629996001247638527693871619517_u128,213411695310157456615803606727801653723_u128,70218497672157638211343420365048264899_u128,333901228337322275553286370617946868749_u128,82239123656288176992780867087737796780_u128];
_18 = [_11,_11,_11,_11,_5,_11,_5,_5];
_18 = [_11,_11,_11,_11,_5,_11,_5,_5];
_14 = _20;
_10 = _8;
_21 = _16;
_12.0 = '\u{6e1a8}';
_5 = _11;
_7 = [201649733703408212432983117027651234390_u128,118363258081068144765938104756744743034_u128,10776367219524584050622518106262218038_u128,185302897500380799597331568824339837130_u128,9108512308080104801557986502159095814_u128,189792534381793767308966974137857284268_u128];
RET = [_20,_14,_20];
_20 = !_14;
(*_17) = _10;
_21 = _16;
_20 = !_14;
Goto(bb7)
}
bb7 = {
_15 = [_1,_1,_1,_1];
_4 = (*_17);
(*_17) = _6;
(*_17) = [137881511015991997624872273256982289449_u128,324201771018757008135532177066867012680_u128,82395998510995237612044160220687203958_u128,43098207464241167997238838897373105954_u128,223824693167581598238230119377162249593_u128,287540008291210388697177341289484147592_u128];
_23 = (-3663109210960110719_i64) as f64;
_2 = _8;
_20 = !_14;
_22 = _1 - _1;
Call(_9 = core::intrinsics::transmute(_3), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_8 = [300917832072372936444998500262912611287_u128,273909246612269186615619250320310265143_u128,210397818100608812385197794414784349986_u128,57439123797543146537106603812577786769_u128,69954146819378808636580585468007841011_u128,48397683875128336743660204174251292934_u128];
_16 = _21;
_13 = &_23;
(*_17) = _3;
_18 = [_5,_5,_11,_5,_5,_11,_11,_5];
_27 = core::ptr::addr_of_mut!(_13);
_1 = _22 << _11;
(*_17) = _9;
(*_17) = _9;
_26 = [(-20_i8),(-125_i8),(-18_i8),(-24_i8),(-102_i8),126_i8,99_i8];
_28.0 = _12.0;
_30 = _5 as f32;
_29 = [213445392408975981392435533663089896979_u128,277197326376453785735005641889836793473_u128,93824166958744799206270906626789093906_u128,286920249853341932458450129927809720280_u128,154180948703461519693509806314329971778_u128,48257880076422764725445384764815273923_u128];
_28.1 = Adt27::Variant0 { fld0: _1,fld1: _30,fld2: _23 };
_29 = [130792360445267521847881290581561404698_u128,156904619409192775816473090722091760848_u128,29712325561713727795714405252061773784_u128,204414475337472026182374654850689194148_u128,679603372574495293302439107239342234_u128,40146802127457388519052824446150272335_u128];
Goto(bb9)
}
bb9 = {
_31 = core::ptr::addr_of_mut!(RET);
_4 = [337354494144779380494443806054918076578_u128,105674998551541421589925646915755455408_u128,117483553716726322952253945374303731298_u128,297712561386922899650908953876316110094_u128,287506815519192160386618138439113519890_u128,182229872040224808688517241467133419614_u128];
(*_17) = [163838071458491897416219382432404870974_u128,290429693900071896467342050706059345794_u128,15022032029703555353797414626192852060_u128,201862715919600318156965793500150339676_u128,160848937426866796435162887933954469747_u128,113366781979061273613799687500509059898_u128];
_30 = Field::<f32>(Variant(_28.1, 0), 1);
_12.1 = Move(_28.1);
(*_27) = &place!(Field::<f64>(Variant(_12.1, 0), 2));
(*_31) = [_20,_20,_20];
SetDiscriminant(_12.1, 1);
_13 = &_23;
_26 = [63_i8,(-87_i8),(-37_i8),22_i8,42_i8,45_i8,15_i8];
Call(_12.1 = fn17(Move(_27), Move(_13), _9, _12.0, Move(_31), Move(_17), _18, (*_17), _21, _11), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_17 = core::ptr::addr_of_mut!(_29);
_5 = _11;
_4 = [168240969312741811030636589284668349804_u128,228197955792611231990533119337411319006_u128,85874201743082282498826850744408982655_u128,225391754278705887464718355785138786723_u128,139827758777307990621438354854904158023_u128,180581520344263281242592528749662655694_u128];
Goto(bb11)
}
bb11 = {
_6 = [243680902670414111902802613663786601510_u128,61973524345079135142292116375435488754_u128,239963123305945147200341971551506472622_u128,160945630290708708600691545662266833799_u128,220842266918365900529720102792408950395_u128,146804262568157856925023631784907079585_u128];
_32 = !_20;
_28.1 = Move(_12.1);
_18 = [_11,_5,_5,_5,_11,_11,_11,_11];
place!(Field::<i32>(Variant(_28.1, 0), 0)) = !_1;
(*_17) = [221828701178454269972712204351005576036_u128,164684057836566073728932892144920359699_u128,53781681981580393563219162733449339302_u128,63267165585037497676193127544608481501_u128,50364884941121832888808349123554089860_u128,83750005198405579776702073324246764541_u128];
_1 = Field::<i32>(Variant(_28.1, 0), 0) ^ Field::<i32>(Variant(_28.1, 0), 0);
_18 = [_11,_5,_11,_11,_5,_11,_5,_11];
_25 = &_22;
_1 = (-55734149181326717653691955937872288287_i128) as i32;
_17 = core::ptr::addr_of_mut!(_6);
_17 = core::ptr::addr_of_mut!(_2);
_23 = -Field::<f64>(Variant(_28.1, 0), 2);
_33.0 = 108137898750755983457438390443204779067_i128;
match _33.0 {
0 => bb12,
108137898750755983457438390443204779067 => bb14,
_ => bb13
}
}
bb12 = {
Return()
}
bb13 = {
_20 = !_14;
_20 = 55166315658939472011587867000658480921_i128 as isize;
_6 = [223486640660779742768126251014412290500_u128,191172115534281635219496667882477580545_u128,238974405248211433910091444744695836395_u128,104243818415782003471045902633266667011_u128,228985932630405434004076201828239759589_u128,108725146947323375965556390297711143441_u128];
_1 = -2067806524_i32;
_3 = [178395306678883199607251161581305640315_u128,249883990629996001247638527693871619517_u128,213411695310157456615803606727801653723_u128,70218497672157638211343420365048264899_u128,333901228337322275553286370617946868749_u128,82239123656288176992780867087737796780_u128];
_18 = [_11,_11,_11,_11,_5,_11,_5,_5];
_18 = [_11,_11,_11,_11,_5,_11,_5,_5];
_14 = _20;
_10 = _8;
_21 = _16;
_12.0 = '\u{6e1a8}';
_5 = _11;
_7 = [201649733703408212432983117027651234390_u128,118363258081068144765938104756744743034_u128,10776367219524584050622518106262218038_u128,185302897500380799597331568824339837130_u128,9108512308080104801557986502159095814_u128,189792534381793767308966974137857284268_u128];
RET = [_20,_14,_20];
_20 = !_14;
(*_17) = _10;
_21 = _16;
_20 = !_14;
Goto(bb7)
}
bb14 = {
RET = [_32,_20,_32];
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(16_usize, 16_usize, Move(_16), 20_usize, Move(_20), 11_usize, Move(_11), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(16_usize, 10_usize, Move(_10), 8_usize, Move(_8), 7_usize, Move(_7), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(16_usize, 21_usize, Move(_21), 6_usize, Move(_6), 14_usize, Move(_14), 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: *mut &'static f64,mut _2: &'static f64,mut _3: [u128; 6],mut _4: char,mut _5: *mut [isize; 3],mut _6: *mut [u128; 6],mut _7: [usize; 8],mut _8: [u128; 6],mut _9: bool,mut _10: usize) -> Adt27 {
mir! {
type RET = Adt27;
let _11: f64;
let _12: (i32, Adt22, *mut &'static f64, f64);
let _13: &'static bool;
let _14: i32;
let _15: *const f64;
let _16: [usize; 1];
let _17: [i16; 5];
let _18: (*mut &'static u8, f64, i16);
let _19: [bool; 4];
let _20: u8;
let _21: char;
let _22: usize;
let _23: u16;
let _24: *mut (&'static bool, (i128,));
let _25: i32;
let _26: bool;
let _27: bool;
let _28: i32;
let _29: [isize; 3];
let _30: [u32; 2];
let _31: char;
let _32: u64;
let _33: [u16; 5];
let _34: [bool; 4];
let _35: bool;
let _36: bool;
let _37: isize;
let _38: &'static [u128; 7];
let _39: *mut Adt48;
let _40: &'static f64;
let _41: f64;
let _42: isize;
let _43: (i128,);
let _44: u8;
let _45: u16;
let _46: f32;
let _47: i64;
let _48: bool;
let _49: char;
let _50: bool;
let _51: u128;
let _52: &'static [u128; 7];
let _53: ();
let _54: ();
{
_10 = !5773450887238813249_usize;
_9 = true;
_9 = true;
_7 = [_10,_10,_10,_10,_10,_10,_10,_10];
_6 = core::ptr::addr_of_mut!(_8);
_8 = [98028021102353357897673562018174436136_u128,327812391759900172359643771979735721485_u128,209453580880594314661150765686545819534_u128,296217787106643656165253972558737680231_u128,240820135872460012169242316877385989110_u128,40060640669976988641787977543323542636_u128];
_1 = core::ptr::addr_of_mut!(_2);
_8 = _3;
(*_6) = _3;
(*_6) = _3;
_7 = [_10,_10,_10,_10,_10,_10,_10,_10];
(*_6) = [92338801980711364444667889260150353739_u128,329384607733632433991075533375233211912_u128,278174837776003627925661040371914712002_u128,291576096587987737448823953782522422720_u128,239029991587645855963154377738169636178_u128,77996099406754534823686485450827409735_u128];
(*_6) = [282378779516429798386060946951066085452_u128,171098271282264741774012455179313452408_u128,24137295109660185026996499274833997593_u128,91355022200186701553532446301246380194_u128,267509541284573164596048520053588569536_u128,41191313939331308771057079898283098481_u128];
_3 = [48949091164758603209825597848863563887_u128,212080273872934392443823871497183563867_u128,115063253124902623846629564669351125512_u128,75412446572565568537335966564050673478_u128,137367994050265530222683018297632530287_u128,185419744441730292767225298112411757485_u128];
_7 = [_10,_10,_10,_10,_10,_10,_10,_10];
_12.1 = Adt22::Variant0 { fld0: _10,fld1: 10826_u16 };
_8 = _3;
_12.2 = core::ptr::addr_of_mut!((*_1));
(*_1) = &_12.3;
(*_6) = [54052603153357464820736512313635746063_u128,98588749400484990336528416854286440465_u128,285482287670901550538782117916227869515_u128,225343747773401327912566914217756938738_u128,276104571530913912282925372562179513104_u128,192519964000219748316418666049788040912_u128];
_14 = 10030509743237429166149813352296065441_u128 as i32;
place!(Field::<u16>(Variant(_12.1, 0), 1)) = !7695_u16;
_9 = true | true;
place!(Field::<usize>(Variant(_12.1, 0), 0)) = !_10;
_8 = [320673391909642845392977924320445166552_u128,311481047985740820956083950538773455053_u128,87099348623186584491996405444515232958_u128,115862089375828959619484560286646139119_u128,288249007226963926141218844343559732590_u128,50034443859035643041243871967144595476_u128];
_11 = 4488110597837147626_i64 as f64;
Goto(bb1)
}
bb1 = {
_10 = Field::<usize>(Variant(_12.1, 0), 0) << Field::<u16>(Variant(_12.1, 0), 1);
_12.1 = Adt22::Variant1 { fld0: _9,fld1: _4,fld2: _10,fld3: (-102514449814618058688626544930061771557_i128),fld4: 9825013114863408345_u64 };
place!(Field::<u64>(Variant(_12.1, 1), 4)) = 2129843635790389723_u64;
_12.0 = -_14;
(*_6) = _3;
_4 = Field::<char>(Variant(_12.1, 1), 1);
_13 = &place!(Field::<bool>(Variant(_12.1, 1), 0));
place!(Field::<char>(Variant(_12.1, 1), 1)) = _4;
_9 = (*_13) ^ (*_13);
(*_6) = _3;
place!(Field::<bool>(Variant(_12.1, 1), 0)) = !_9;
_3 = [153579102077352665301396811406115798879_u128,212012650993975285688541361214793218978_u128,42939997504436221679614573047883066688_u128,103557426843168121484552929014635595036_u128,195521640658602306646461569305990538296_u128,61097876117164047014794557899158969463_u128];
match Field::<u64>(Variant(_12.1, 1), 4) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
2129843635790389723 => bb9,
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
place!(Field::<usize>(Variant(_12.1, 1), 2)) = _10;
place!(Field::<i128>(Variant(_12.1, 1), 3)) = 130679342783695086444853601052903111358_i128;
(*_1) = &_18.1;
(*_6) = [272845329392890620585733289709203186207_u128,186230981798282028082314242030885368986_u128,301917771367286139267772697979928338707_u128,38204548495354007952611985141793257794_u128,164561314717980790143113324461557909274_u128,303964872154712240864764819615530946498_u128];
_19 = [Field::<bool>(Variant(_12.1, 1), 0),Field::<bool>(Variant(_12.1, 1), 0),_9,Field::<bool>(Variant(_12.1, 1), 0)];
_4 = Field::<char>(Variant(_12.1, 1), 1);
_10 = Field::<usize>(Variant(_12.1, 1), 2) + Field::<usize>(Variant(_12.1, 1), 2);
_18.1 = _11 - _11;
match Field::<u64>(Variant(_12.1, 1), 4) {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
2129843635790389723 => bb17,
_ => bb16
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
Return()
}
bb15 = {
Return()
}
bb16 = {
_10 = Field::<usize>(Variant(_12.1, 0), 0) << Field::<u16>(Variant(_12.1, 0), 1);
_12.1 = Adt22::Variant1 { fld0: _9,fld1: _4,fld2: _10,fld3: (-102514449814618058688626544930061771557_i128),fld4: 9825013114863408345_u64 };
place!(Field::<u64>(Variant(_12.1, 1), 4)) = 2129843635790389723_u64;
_12.0 = -_14;
(*_6) = _3;
_4 = Field::<char>(Variant(_12.1, 1), 1);
_13 = &place!(Field::<bool>(Variant(_12.1, 1), 0));
place!(Field::<char>(Variant(_12.1, 1), 1)) = _4;
_9 = (*_13) ^ (*_13);
(*_6) = _3;
place!(Field::<bool>(Variant(_12.1, 1), 0)) = !_9;
_3 = [153579102077352665301396811406115798879_u128,212012650993975285688541361214793218978_u128,42939997504436221679614573047883066688_u128,103557426843168121484552929014635595036_u128,195521640658602306646461569305990538296_u128,61097876117164047014794557899158969463_u128];
match Field::<u64>(Variant(_12.1, 1), 4) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
2129843635790389723 => bb9,
_ => bb8
}
}
bb17 = {
_16 = [Field::<usize>(Variant(_12.1, 1), 2)];
_18.2 = 27486_i16;
_19 = [_9,Field::<bool>(Variant(_12.1, 1), 0),Field::<bool>(Variant(_12.1, 1), 0),_9];
_2 = &_11;
_18.2 = 29203_i16 * 22528_i16;
_12.0 = 224629199799037393237136027311023499555_u128 as i32;
_15 = core::ptr::addr_of!((*_2));
place!(Field::<char>(Variant(_12.1, 1), 1)) = _4;
_13 = &place!(Field::<bool>(Variant(_12.1, 1), 0));
_12.2 = core::ptr::addr_of_mut!((*_1));
SetDiscriminant(_12.1, 0);
_20 = 56_u8;
(*_6) = [233786299083292424563232093583504904315_u128,49745686884645911583364883990042488322_u128,57182171150424045519288934238256964904_u128,326475901285319310509320892078902192677_u128,115077405457668251679368708072116959116_u128,3812417001414396158375399951822952159_u128];
_21 = _4;
(*_1) = &(*_15);
_22 = !_10;
(*_1) = &(*_15);
_13 = &_9;
_9 = (*_2) <= _18.1;
Goto(bb18)
}
bb18 = {
_17 = [_18.2,_18.2,_18.2,_18.2,_18.2];
_1 = Move(_12.2);
_12.2 = Move(_1);
_22 = _10 | _10;
_1 = Move(_12.2);
_12.0 = -_14;
_7 = [_22,_10,_22,_22,_22,_22,_22,_22];
_29 = [(-9223372036854775808_isize),(-55_isize),(-9223372036854775808_isize)];
_12.0 = _14;
_5 = core::ptr::addr_of_mut!(_29);
(*_5) = [9223372036854775807_isize,(-9223372036854775808_isize),55_isize];
_12.1 = Adt22::Variant1 { fld0: _9,fld1: _21,fld2: _22,fld3: 77371869768256317385007226038489540355_i128,fld4: 2487122898070426062_u64 };
_16 = [_10];
_23 = 61210_u16 & 42107_u16;
match _20 {
56 => bb20,
_ => bb19
}
}
bb19 = {
Return()
}
bb20 = {
_31 = _4;
place!(Field::<i128>(Variant(_12.1, 1), 3)) = 135110895629981086050181088920031818004_i128;
_6 = core::ptr::addr_of_mut!(_3);
_32 = 50502997295455196_u64;
_28 = _14 ^ _14;
_11 = _18.1;
_22 = Field::<usize>(Variant(_12.1, 1), 2);
_33 = [_23,_23,_23,_23,_23];
_3 = [259012947719778709041914118913225042823_u128,304960278663204353956490832756057599873_u128,74194298726084731518410422034627664094_u128,275826103204437382784726021023303791576_u128,141304332848847908155289827507486507853_u128,261403165281001881323018721313833684295_u128];
_13 = &_26;
_32 = !13172553739815742553_u64;
_20 = !148_u8;
_22 = Field::<usize>(Variant(_12.1, 1), 2);
_16 = [Field::<usize>(Variant(_12.1, 1), 2)];
Goto(bb21)
}
bb21 = {
_12.3 = _18.1 * _11;
_8 = [255683124823827315582045654530616115129_u128,278422293721014708401858901152945256532_u128,67424128215840400853089757462945850518_u128,325570030466958659870536236052189130108_u128,82505987775291662201312052293832212198_u128,263500034851143648270547329456217888603_u128];
_29 = [110_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_25 = -_14;
_8 = [177927452338954183737075090526303269613_u128,333682940506839250905585397782957143885_u128,67772012460529345277766439116801459393_u128,186029426071738058449648136012169661177_u128,129641671518643470362573953372054671179_u128,148077397780781982011947032570746946076_u128];
(*_6) = [89818586092597774640433779385745984152_u128,48701565514164041137393193447691514707_u128,215708009300329760963578354540488375746_u128,37101511222286885457245392832580401600_u128,125252803284150301284738776004232630512_u128,153184951000742329932040253304200320228_u128];
_28 = _14;
place!(Field::<u64>(Variant(_12.1, 1), 4)) = _32;
(*_5) = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_26 = !_9;
_2 = &_18.1;
_35 = _9 ^ _26;
_22 = _10;
_23 = _18.2 as u16;
_18.2 = _18.1 as i16;
_11 = -_18.1;
_12.1 = Adt22::Variant0 { fld0: _22,fld1: _23 };
_22 = Field::<usize>(Variant(_12.1, 0), 0) + Field::<usize>(Variant(_12.1, 0), 0);
(*_5) = [9223372036854775807_isize,(-55_isize),(-9223372036854775808_isize)];
_17 = [_18.2,_18.2,_18.2,_18.2,_18.2];
_29 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_7 = [_22,_22,_10,_22,Field::<usize>(Variant(_12.1, 0), 0),_22,_10,_22];
_17 = [_18.2,_18.2,_18.2,_18.2,_18.2];
_13 = &_27;
SetDiscriminant(_12.1, 0);
Goto(bb22)
}
bb22 = {
_17 = [_18.2,_18.2,_18.2,_18.2,_18.2];
place!(Field::<u16>(Variant(_12.1, 0), 1)) = !_23;
_28 = _14;
_36 = !_9;
_12.1 = Adt22::Variant1 { fld0: _35,fld1: _21,fld2: _22,fld3: 110655155481322340549357580699659406609_i128,fld4: _32 };
_5 = core::ptr::addr_of_mut!(_29);
_18.1 = _11;
Call(_12.2 = fn18(Move(_1), _11, _3, _7, (*_5), Field::<bool>(Variant(_12.1, 1), 0), _22, _3), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
_15 = core::ptr::addr_of!(_12.3);
_26 = _35;
_19 = [_35,_9,_26,_26];
(*_15) = 9223372036854775807_isize as f64;
_9 = _36;
_12.1 = Adt22::Variant1 { fld0: _36,fld1: _4,fld2: _10,fld3: (-129829686312360848273794983821098590369_i128),fld4: _32 };
place!(Field::<i128>(Variant(_12.1, 1), 3)) = (-141479014649853654712915040505219831828_i128);
_5 = core::ptr::addr_of_mut!((*_5));
_36 = _26;
Goto(bb24)
}
bb24 = {
(*_6) = _8;
_30 = [1847997570_u32,333707069_u32];
_12.0 = _28 * _14;
_27 = Field::<bool>(Variant(_12.1, 1), 0);
_28 = _25;
_17 = [_18.2,_18.2,_18.2,_18.2,_18.2];
(*_5) = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_6 = core::ptr::addr_of_mut!(_3);
_21 = Field::<char>(Variant(_12.1, 1), 1);
(*_5) = [9223372036854775807_isize,(-9223372036854775808_isize),120_isize];
_37 = 9223372036854775807_isize;
_30 = [3042409621_u32,2637577536_u32];
_29 = [_37,_37,_37];
_8 = (*_6);
Goto(bb25)
}
bb25 = {
(*_6) = [184519585138180209925482262961542070172_u128,240676176427543605017439635187733162925_u128,328880846869271194770767096140085494923_u128,181287385391154311642337463627206287544_u128,120989659285660552667386249770077426762_u128,163527932201003848737986459757847487026_u128];
_29 = [_37,_37,_37];
_36 = !_26;
_3 = [110834547842685340905991648084343121136_u128,256802980803491228694470929590451276600_u128,139554298659349519218424129970255143967_u128,14731817061391251799085185162176485579_u128,122507291781980132339131894419068451417_u128,269526943489613704506233365146722539258_u128];
_11 = (*_15) + (*_15);
_40 = &_12.3;
_23 = _35 as u16;
_2 = Move(_40);
_15 = core::ptr::addr_of!(_11);
SetDiscriminant(_12.1, 1);
_1 = core::ptr::addr_of_mut!(_40);
(*_15) = _18.1 + _12.3;
Goto(bb26)
}
bb26 = {
_34 = _19;
_12.0 = !_14;
_36 = !_9;
_7 = [_22,_22,_22,_22,_22,_22,_22,_22];
_31 = _4;
place!(Field::<char>(Variant(_12.1, 1), 1)) = _31;
_15 = core::ptr::addr_of!(_41);
_27 = !_26;
_13 = &_27;
_14 = _28 ^ _12.0;
_12.2 = core::ptr::addr_of_mut!(_40);
_2 = &_12.3;
_42 = _25 as isize;
place!(Field::<char>(Variant(_12.1, 1), 1)) = _4;
_25 = !_28;
place!(Field::<i128>(Variant(_12.1, 1), 3)) = !(-146622774927067678377723430656629890265_i128);
place!(Field::<i128>(Variant(_12.1, 1), 3)) = 49752164852939560671262229484058388956_i128;
(*_15) = _32 as f64;
_4 = Field::<char>(Variant(_12.1, 1), 1);
place!(Field::<bool>(Variant(_12.1, 1), 0)) = _27;
_22 = _31 as usize;
Goto(bb27)
}
bb27 = {
_2 = &_11;
_42 = !_37;
_2 = &_12.3;
(*_5) = [_37,_42,_37];
_9 = _26;
_11 = -_18.1;
_14 = _12.0 - _25;
_10 = (-4509830998896128070_i64) as usize;
_12.1 = Adt22::Variant0 { fld0: _10,fld1: _23 };
SetDiscriminant(_12.1, 1);
_12.3 = -_11;
_46 = 1497297172_u32 as f32;
match _37 {
0 => bb28,
1 => bb29,
2 => bb30,
3 => bb31,
4 => bb32,
9223372036854775807 => bb34,
_ => bb33
}
}
bb28 = {
Return()
}
bb29 = {
(*_6) = [184519585138180209925482262961542070172_u128,240676176427543605017439635187733162925_u128,328880846869271194770767096140085494923_u128,181287385391154311642337463627206287544_u128,120989659285660552667386249770077426762_u128,163527932201003848737986459757847487026_u128];
_29 = [_37,_37,_37];
_36 = !_26;
_3 = [110834547842685340905991648084343121136_u128,256802980803491228694470929590451276600_u128,139554298659349519218424129970255143967_u128,14731817061391251799085185162176485579_u128,122507291781980132339131894419068451417_u128,269526943489613704506233365146722539258_u128];
_11 = (*_15) + (*_15);
_40 = &_12.3;
_23 = _35 as u16;
_2 = Move(_40);
_15 = core::ptr::addr_of!(_11);
SetDiscriminant(_12.1, 1);
_1 = core::ptr::addr_of_mut!(_40);
(*_15) = _18.1 + _12.3;
Goto(bb26)
}
bb30 = {
Return()
}
bb31 = {
_15 = core::ptr::addr_of!(_12.3);
_26 = _35;
_19 = [_35,_9,_26,_26];
(*_15) = 9223372036854775807_isize as f64;
_9 = _36;
_12.1 = Adt22::Variant1 { fld0: _36,fld1: _4,fld2: _10,fld3: (-129829686312360848273794983821098590369_i128),fld4: _32 };
place!(Field::<i128>(Variant(_12.1, 1), 3)) = (-141479014649853654712915040505219831828_i128);
_5 = core::ptr::addr_of_mut!((*_5));
_36 = _26;
Goto(bb24)
}
bb32 = {
Return()
}
bb33 = {
Return()
}
bb34 = {
_10 = !_22;
RET = Adt27::Variant0 { fld0: _12.0,fld1: _46,fld2: _11 };
SetDiscriminant(RET, 1);
_43 = ((-15802211071881335817795180804280795172_i128),);
_13 = &_26;
RET = Adt27::Variant0 { fld0: _25,fld1: _46,fld2: (*_15) };
_29 = [_37,_37,_37];
_28 = _14;
_50 = !_35;
place!(Field::<u64>(Variant(_12.1, 1), 4)) = _32;
place!(Field::<i32>(Variant(RET, 0), 0)) = _12.0;
(*_15) = -_18.1;
_9 = _26 ^ _50;
place!(Field::<char>(Variant(_12.1, 1), 1)) = _21;
_29 = [_42,_42,_42];
_12.1 = Adt22::Variant0 { fld0: _22,fld1: _23 };
(*_5) = [_42,_42,_42];
Goto(bb35)
}
bb35 = {
Call(_53 = dump_var(17_usize, 37_usize, Move(_37), 36_usize, Move(_36), 34_usize, Move(_34), 35_usize, Move(_35)), ReturnTo(bb36), UnwindUnreachable())
}
bb36 = {
Call(_53 = dump_var(17_usize, 19_usize, Move(_19), 4_usize, Move(_4), 27_usize, Move(_27), 17_usize, Move(_17)), ReturnTo(bb37), UnwindUnreachable())
}
bb37 = {
Call(_53 = dump_var(17_usize, 14_usize, Move(_14), 20_usize, Move(_20), 28_usize, Move(_28), 7_usize, Move(_7)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Call(_53 = dump_var(17_usize, 25_usize, Move(_25), 10_usize, Move(_10), 26_usize, Move(_26), 54_usize, _54), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: *mut &'static f64,mut _2: f64,mut _3: [u128; 6],mut _4: [usize; 8],mut _5: [isize; 3],mut _6: bool,mut _7: usize,mut _8: [u128; 6]) -> *mut &'static f64 {
mir! {
type RET = *mut &'static f64;
let _9: u16;
let _10: isize;
let _11: Adt70;
let _12: i32;
let _13: Adt22;
let _14: [u128; 7];
let _15: (&'static *mut u8, bool);
let _16: f64;
let _17: &'static usize;
let _18: isize;
let _19: f32;
let _20: u16;
let _21: i64;
let _22: isize;
let _23: [i8; 3];
let _24: (*const (&'static (&'static f64,), Adt34),);
let _25: char;
let _26: usize;
let _27: ();
let _28: ();
{
RET = Move(_1);
_3 = [81713776793639719749966987793105968978_u128,148239507506623077112418473133936264261_u128,167032007365044321668244194406318001568_u128,135461295744076520478252603899438383885_u128,34042843385344970286630680593353298498_u128,247766479362556203584163631035250393622_u128];
_5 = [9223372036854775807_isize,(-86_isize),(-9223372036854775808_isize)];
_4 = [_7,_7,_7,_7,_7,_7,_7,_7];
_1 = Move(RET);
RET = Move(_1);
_1 = Move(RET);
_3 = [165737401028826890574179151891992711765_u128,198849684598776865767921703520239946460_u128,127641386196921881320536629995062202874_u128,331800796240442003312094082308959624371_u128,295576003766557228199729216447120041894_u128,163001680307988328084363974754257686905_u128];
_8 = [88617566068606083409122558941199584340_u128,307512404840173351001943161977229294733_u128,120916537362532087907355920895003290694_u128,73881837318816450569195016889595659325_u128,98606761687455058105544335276960052024_u128,15407162863163053717743406868166536817_u128];
_5 = [9223372036854775807_isize,(-75_isize),9223372036854775807_isize];
_2 = _7 as f64;
_7 = 13921851225798599190_usize ^ 4_usize;
_7 = 1380568598478521940_usize | 7_usize;
_9 = 44471_u16;
_6 = _2 >= _2;
_5 = [(-9223372036854775808_isize),25_isize,4_isize];
RET = Move(_1);
_9 = 60408_u16 - 42287_u16;
_1 = Move(RET);
RET = Move(_1);
Goto(bb1)
}
bb1 = {
_8 = [22309715950409373178021051103615209507_u128,53017058058092260913665989790138787457_u128,281373804178562397189429077622635671655_u128,317611586466816529256370553017382806579_u128,258803004596276999906735099642171427955_u128,186368861577940274916504416435775852400_u128];
_8 = [181066485982238981461578494538829917209_u128,203436988283482976280042119647210136021_u128,43145453115636231969678676321744558719_u128,78544040177342843789448292569846967170_u128,229875210101507253432293505596250339603_u128,209982084825934836961022697777474088014_u128];
_7 = 2087721122072934219_usize;
_3 = [161801931138672992765852117854149411240_u128,126777990177345668393514510176973563157_u128,110180713142336350089106392308128103026_u128,120289407247377202164803485182669149603_u128,240134874031819164386074322211596284036_u128,259859454041108975000569155999463289541_u128];
_6 = _2 != _2;
_10 = (-55_isize);
_2 = (-394999548_i32) as f64;
_8 = _3;
_6 = _2 > _2;
_10 = !91_isize;
_7 = 18020344659440020384438582475009577517_i128 as usize;
_5 = [_10,_10,_10];
_2 = 3_i8 as f64;
_7 = '\u{fa0aa}' as usize;
_6 = false;
_8 = [86035209267076954087007528240256905913_u128,3084261499498603997777494228060594395_u128,301573265860974738537796921996662803983_u128,89833098278082367218937135335855179546_u128,197409242289962688475110516333735443876_u128,305270102959580559180345401199686078543_u128];
Goto(bb2)
}
bb2 = {
_1 = Move(RET);
_12 = (-683043505_i32) & (-169301249_i32);
Goto(bb3)
}
bb3 = {
_6 = !false;
_3 = [177506018499576021859076398134047861279_u128,185525632416192438727877791685782871605_u128,202998657140081405479741471340225057685_u128,191994725200527923253551413163147085691_u128,164193809327546233404369988822354676211_u128,17879698967790963428910788446080827112_u128];
_9 = 35285_u16;
Goto(bb4)
}
bb4 = {
_12 = !550712543_i32;
_4 = [_7,_7,_7,_7,_7,_7,_7,_7];
RET = Move(_1);
_1 = Move(RET);
_13 = Adt22::Variant1 { fld0: _6,fld1: '\u{aa84d}',fld2: _7,fld3: (-43316404916861822231131726978319431080_i128),fld4: 8006973007324415431_u64 };
_12 = _7 as i32;
_4 = [_7,Field::<usize>(Variant(_13, 1), 2),_7,_7,_7,_7,_7,Field::<usize>(Variant(_13, 1), 2)];
_9 = !60914_u16;
_13 = Adt22::Variant0 { fld0: _7,fld1: _9 };
place!(Field::<u16>(Variant(_13, 0), 1)) = _10 as u16;
_8 = [28006583386528770709390998251963252058_u128,314447621337371135397105916347473333660_u128,156488616034674189761408574611479074458_u128,296536974284403610573372333374275960274_u128,239278433502561175373117730235972139700_u128,44539207547637005564674889890864695146_u128];
RET = Move(_1);
Goto(bb5)
}
bb5 = {
_10 = 154325888520267598830950191448763479738_u128 as isize;
_1 = Move(RET);
_13 = Adt22::Variant0 { fld0: _7,fld1: _9 };
place!(Field::<u16>(Variant(_13, 0), 1)) = 313329591372095834416896076751654951482_u128 as u16;
_15.1 = Field::<u16>(Variant(_13, 0), 1) > Field::<u16>(Variant(_13, 0), 1);
_10 = -(-7_isize);
_8 = [127230877095953192453008591766976231724_u128,331250820469810901657458889815569000229_u128,304774244582427134678253126991182551482_u128,63483081517038389508496461169206057968_u128,280913286330706399247167243802794973806_u128,248679709167303806248360163116142330640_u128];
_14 = [271703018597884681772545546925093787737_u128,110153587863131586401896494331262384206_u128,294613162908314542146105780072814513274_u128,239413654686492098456766379123559868671_u128,210342984675418864504903622559427531570_u128,128346567791739264091648182037499977408_u128,194228933549809191289140727709727117456_u128];
_15.1 = _6;
_9 = Field::<u16>(Variant(_13, 0), 1) << _10;
place!(Field::<u16>(Variant(_13, 0), 1)) = _9 >> _12;
_10 = 99_isize;
_4 = [Field::<usize>(Variant(_13, 0), 0),_7,_7,Field::<usize>(Variant(_13, 0), 0),Field::<usize>(Variant(_13, 0), 0),Field::<usize>(Variant(_13, 0), 0),_7,_7];
_9 = Field::<u16>(Variant(_13, 0), 1) << Field::<u16>(Variant(_13, 0), 1);
RET = Move(_1);
_16 = _2 - _2;
_2 = _16 - _16;
_13 = Adt22::Variant0 { fld0: _7,fld1: _9 };
_7 = _9 as usize;
place!(Field::<u16>(Variant(_13, 0), 1)) = _9;
_7 = 289402963773604237095810193977365112292_u128 as usize;
Goto(bb6)
}
bb6 = {
_8 = [167701389318857606679191436616904729490_u128,272658534132730853556052433367405887347_u128,221676993065824822944103131327633859485_u128,301617156063808796274974085237603137102_u128,91040110063039400929498148435250612234_u128,111488905069853103493042330840172259076_u128];
_16 = _2;
place!(Field::<usize>(Variant(_13, 0), 0)) = 932_i16 as usize;
_15.1 = _6 & _6;
_16 = _2;
_15.1 = _6;
_13 = Adt22::Variant0 { fld0: _7,fld1: _9 };
_13 = Adt22::Variant0 { fld0: _7,fld1: _9 };
_6 = !_15.1;
_14 = [10690506167395295821465571452421581181_u128,293972679834555908398318436386213884707_u128,195304769231941045201493971009547275417_u128,50175101218016862879490546576840741477_u128,269842732607556976488594056294540363458_u128,319507801753344202031765251730305071329_u128,133685401955631529538892595483133447159_u128];
_7 = Field::<usize>(Variant(_13, 0), 0) >> _9;
_1 = Move(RET);
_4 = [_7,_7,_7,_7,_7,_7,_7,_7];
_14 = [310860151198838309612566052006550660666_u128,63534583856350101328321079078958667925_u128,166033341932429902306825735939538733066_u128,282212923472765905426156366952396931261_u128,63596153230817709195799146459443376786_u128,255706656751211912040309504065746339676_u128,109139802717521978475388484681205842317_u128];
_20 = _9 ^ Field::<u16>(Variant(_13, 0), 1);
_13 = Adt22::Variant0 { fld0: _7,fld1: _9 };
_9 = _20 * Field::<u16>(Variant(_13, 0), 1);
_6 = _15.1;
_12 = 1300207719_i32;
_17 = &_7;
_19 = _10 as f32;
_9 = 132_u8 as u16;
_8 = _3;
_15.1 = Field::<usize>(Variant(_13, 0), 0) != Field::<usize>(Variant(_13, 0), 0);
_13 = Adt22::Variant0 { fld0: (*_17),fld1: _20 };
match _12 {
0 => bb1,
1 => bb7,
1300207719 => bb9,
_ => bb8
}
}
bb7 = {
_8 = [22309715950409373178021051103615209507_u128,53017058058092260913665989790138787457_u128,281373804178562397189429077622635671655_u128,317611586466816529256370553017382806579_u128,258803004596276999906735099642171427955_u128,186368861577940274916504416435775852400_u128];
_8 = [181066485982238981461578494538829917209_u128,203436988283482976280042119647210136021_u128,43145453115636231969678676321744558719_u128,78544040177342843789448292569846967170_u128,229875210101507253432293505596250339603_u128,209982084825934836961022697777474088014_u128];
_7 = 2087721122072934219_usize;
_3 = [161801931138672992765852117854149411240_u128,126777990177345668393514510176973563157_u128,110180713142336350089106392308128103026_u128,120289407247377202164803485182669149603_u128,240134874031819164386074322211596284036_u128,259859454041108975000569155999463289541_u128];
_6 = _2 != _2;
_10 = (-55_isize);
_2 = (-394999548_i32) as f64;
_8 = _3;
_6 = _2 > _2;
_10 = !91_isize;
_7 = 18020344659440020384438582475009577517_i128 as usize;
_5 = [_10,_10,_10];
_2 = 3_i8 as f64;
_7 = '\u{fa0aa}' as usize;
_6 = false;
_8 = [86035209267076954087007528240256905913_u128,3084261499498603997777494228060594395_u128,301573265860974738537796921996662803983_u128,89833098278082367218937135335855179546_u128,197409242289962688475110516333735443876_u128,305270102959580559180345401199686078543_u128];
Goto(bb2)
}
bb8 = {
_1 = Move(RET);
_12 = (-683043505_i32) & (-169301249_i32);
Goto(bb3)
}
bb9 = {
_6 = !_15.1;
_18 = _10;
_4 = [(*_17),Field::<usize>(Variant(_13, 0), 0),_7,Field::<usize>(Variant(_13, 0), 0),Field::<usize>(Variant(_13, 0), 0),_7,(*_17),Field::<usize>(Variant(_13, 0), 0)];
RET = Move(_1);
_6 = _15.1 ^ _15.1;
Goto(bb10)
}
bb10 = {
_9 = Field::<u16>(Variant(_13, 0), 1);
_14 = [219514260536760482340951198717874956014_u128,162165777235889895706701328633509279083_u128,61015106981428875513250222087028588024_u128,77930179486992516518092473377916399298_u128,176933279714351973981858822537292645924_u128,336064789246382195839276001820176572465_u128,224798947557744227980018039034707117239_u128];
_12 = 1257352823_i32;
_10 = 26489_i16 as isize;
_9 = !Field::<u16>(Variant(_13, 0), 1);
_16 = 15941299957593595297_u64 as f64;
_21 = 3603538715111128494_i64 >> _20;
_12 = (-1055309746_i32) ^ 2034727212_i32;
_22 = _12 as isize;
_15.1 = _6;
Call(_22 = core::intrinsics::bswap(_18), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_15.1 = !_6;
_8 = [241801302740852170075324642056625201518_u128,72244836939458474285131790801165239510_u128,253209464003347172456729322809073659403_u128,24550714926383307698210959646229902486_u128,312325241502642777203304473781394907526_u128,134351068414067417400626094194242513646_u128];
_15.1 = _20 == _20;
_13 = Adt22::Variant1 { fld0: _6,fld1: '\u{10dc4e}',fld2: _7,fld3: (-21331801789098908351365430653793607188_i128),fld4: 10553461596278827897_u64 };
Call(_12 = core::intrinsics::bswap((-237996822_i32)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_5 = [_10,_22,_22];
_18 = _21 as isize;
_21 = (-7902936438852210211_i64) * (-9148373566571173698_i64);
place!(Field::<u64>(Variant(_13, 1), 4)) = !14307635447516489428_u64;
place!(Field::<char>(Variant(_13, 1), 1)) = '\u{acb4}';
_2 = 23306_i16 as f64;
_5 = [_18,_18,_18];
_5 = [_18,_18,_18];
_13 = Adt22::Variant0 { fld0: (*_17),fld1: _20 };
_22 = -_18;
_1 = Move(RET);
_20 = _9;
RET = Move(_1);
_13 = Adt22::Variant0 { fld0: (*_17),fld1: _20 };
_13 = Adt22::Variant0 { fld0: (*_17),fld1: _9 };
_1 = Move(RET);
_8 = _3;
_19 = _20 as f32;
_19 = _12 as f32;
_14 = [215265298678102499242981849683484975283_u128,260806369448877756759754702383116284863_u128,248404363333650485277590826443662487279_u128,320513948825367347866897139432215919747_u128,51481308515615239448405163532596514739_u128,192567227784668887948377953067338949237_u128,75369126139879201296212504536027576441_u128];
Goto(bb13)
}
bb13 = {
_18 = -_22;
_9 = 14_u8 as u16;
RET = Move(_1);
Goto(bb14)
}
bb14 = {
_13 = Adt22::Variant1 { fld0: _6,fld1: '\u{d65a3}',fld2: (*_17),fld3: 112124541865483501956520656510714482065_i128,fld4: 6118993300729035258_u64 };
_20 = (-48486589166733836421353757984546370182_i128) as u16;
_5 = [_18,_18,_22];
place!(Field::<i128>(Variant(_13, 1), 3)) = _21 as i128;
_4 = [_7,_7,(*_17),(*_17),(*_17),(*_17),Field::<usize>(Variant(_13, 1), 2),(*_17)];
place!(Field::<u64>(Variant(_13, 1), 4)) = !7255733451823379395_u64;
_16 = 32122_i16 as f64;
_9 = _20;
_17 = &(*_17);
place!(Field::<u64>(Variant(_13, 1), 4)) = _12 as u64;
place!(Field::<usize>(Variant(_13, 1), 2)) = (*_17) + (*_17);
place!(Field::<char>(Variant(_13, 1), 1)) = '\u{d61b2}';
place!(Field::<usize>(Variant(_13, 1), 2)) = _7 >> _18;
_15.1 = !Field::<bool>(Variant(_13, 1), 0);
_1 = Move(RET);
place!(Field::<usize>(Variant(_13, 1), 2)) = _15.1 as usize;
_23 = [(-57_i8),38_i8,33_i8];
_25 = Field::<char>(Variant(_13, 1), 1);
RET = Move(_1);
_3 = _8;
_26 = (-15131_i16) as usize;
place!(Field::<char>(Variant(_13, 1), 1)) = _25;
_23 = [(-36_i8),(-21_i8),8_i8];
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(18_usize, 18_usize, Move(_18), 8_usize, Move(_8), 12_usize, Move(_12), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(18_usize, 5_usize, Move(_5), 22_usize, Move(_22), 21_usize, Move(_21), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: (i32, Adt22, *mut &'static f64, f64),mut _2: &'static f64,mut _3: Adt34,mut _4: usize,mut _5: isize,mut _6: usize) -> *mut &'static f64 {
mir! {
type RET = *mut &'static f64;
let _7: i128;
let _8: f32;
let _9: isize;
let _10: (&'static bool, (i128,));
let _11: *mut &'static u8;
let _12: i32;
let _13: [u32; 2];
let _14: f32;
let _15: &'static i32;
let _16: char;
let _17: &'static usize;
let _18: bool;
let _19: ();
let _20: ();
{
RET = Move(_1.2);
_1.2 = core::ptr::addr_of_mut!(_2);
place!(Field::<i128>(Variant(_1.1, 1), 3)) = 8850426183251423416_i64 as i128;
place!(Field::<usize>(Variant(_3, 1), 3)) = Field::<usize>(Variant(_1.1, 1), 2);
place!(Field::<char>(Variant(_3, 1), 1)) = Field::<char>(Variant(_1.1, 1), 1);
_8 = Field::<u64>(Variant(_1.1, 1), 4) as f32;
place!(Field::<u64>(Variant(_1.1, 1), 4)) = Field::<usize>(Variant(_3, 1), 3) as u64;
place!(Field::<i128>(Variant(_1.1, 1), 3)) = -(-5000979370895826132161583179775354088_i128);
place!(Field::<usize>(Variant(_3, 1), 3)) = _4;
_5 = Field::<isize>(Variant(_3, 1), 2) | Field::<isize>(Variant(_3, 1), 2);
_7 = !Field::<i128>(Variant(_1.1, 1), 3);
RET = core::ptr::addr_of_mut!(_2);
_9 = -_5;
place!(Field::<u64>(Variant(_1.1, 1), 4)) = Field::<u64>(Variant(_3, 1), 4);
place!(Field::<char>(Variant(_1.1, 1), 1)) = Field::<char>(Variant(_3, 1), 1);
_1.1 = Adt22::Variant1 { fld0: false,fld1: Field::<char>(Variant(_3, 1), 1),fld2: Field::<usize>(Variant(_3, 1), 3),fld3: _7,fld4: Field::<u64>(Variant(_3, 1), 4) };
_9 = !Field::<isize>(Variant(_3, 1), 2);
_1.1 = Adt22::Variant0 { fld0: Field::<usize>(Variant(_3, 1), 3),fld1: Field::<u16>(Variant(_3, 1), 0) };
_2 = &_1.3;
_10.1.0 = _7 * _7;
_2 = &(*_2);
place!(Field::<isize>(Variant(_3, 1), 2)) = _9 | _5;
_10.1.0 = _8 as i128;
place!(Field::<usize>(Variant(_1.1, 0), 0)) = Field::<usize>(Variant(_3, 1), 3);
Goto(bb1)
}
bb1 = {
(*RET) = &_1.3;
_10.1.0 = 208275244131286252418673098541044871051_u128 as i128;
(*RET) = &_1.3;
SetDiscriminant(_1.1, 0);
place!(Field::<u16>(Variant(_1.1, 0), 1)) = Field::<u16>(Variant(_3, 1), 0) * Field::<u16>(Variant(_3, 1), 0);
_5 = Field::<isize>(Variant(_3, 1), 2) >> Field::<usize>(Variant(_3, 1), 3);
(*RET) = &(*_2);
_4 = 217_u8 as usize;
SetDiscriminant(_3, 0);
place!(Field::<bool>(Variant(_3, 0), 0)) = false & false;
place!(Field::<isize>(Variant(_3, 0), 2)) = _5;
Goto(bb2)
}
bb2 = {
_14 = _6 as f32;
_7 = !_10.1.0;
_2 = &(*_2);
_10.0 = &place!(Field::<bool>(Variant(_3, 0), 0));
_13 = [1817882837_u32,3003474659_u32];
(*RET) = &place!(Field::<f64>(Variant(_3, 0), 6));
(*RET) = &place!(Field::<f64>(Variant(_3, 0), 6));
_1.2 = Move(RET);
place!(Field::<i128>(Variant(_3, 0), 7)) = _10.1.0 << Field::<u16>(Variant(_1.1, 0), 1);
_10.1 = (Field::<i128>(Variant(_3, 0), 7),);
_6 = _4 * _4;
_1.2 = core::ptr::addr_of_mut!(_2);
_5 = Field::<isize>(Variant(_3, 0), 2) >> _7;
_1.1 = Adt22::Variant1 { fld0: Field::<bool>(Variant(_3, 0), 0),fld1: '\u{fde53}',fld2: _6,fld3: Field::<i128>(Variant(_3, 0), 7),fld4: 9332848482679095603_u64 };
place!(Field::<f64>(Variant(_3, 0), 6)) = _1.3;
place!(Field::<bool>(Variant(_3, 0), 0)) = _1.0 <= _1.0;
Goto(bb3)
}
bb3 = {
place!(Field::<(f32,)>(Variant(_3, 0), 4)) = (_14,);
_16 = '\u{c0632}';
place!(Field::<char>(Variant(_3, 0), 1)) = _16;
_14 = 8613_i16 as f32;
place!(Field::<u64>(Variant(_1.1, 1), 4)) = !12722219514159069601_u64;
RET = core::ptr::addr_of_mut!(_2);
Goto(bb4)
}
bb4 = {
Call(_19 = dump_var(19_usize, 9_usize, Move(_9), 7_usize, Move(_7), 16_usize, Move(_16), 20_usize, _20), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box(249390535029437434561388950057891772350_u128), std::hint::black_box(9223372036854775807_isize), std::hint::black_box((-91_i8)), std::hint::black_box(21239_i16), std::hint::black_box((-1613100446_i32)), std::hint::black_box((-1235088845663285843_i64)), std::hint::black_box(131419060446593640354226818564613241478_i128), std::hint::black_box(14149713904087602954_u64));
                
            }
impl PrintFDebug for Adt22{
	unsafe fn printf_debug(&self){unsafe{printf("Adt22::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt22 {
Variant0{
fld0: usize,
fld1: u16,

},
Variant1{
fld0: bool,
fld1: char,
fld2: usize,
fld3: i128,
fld4: u64,

}}
impl PrintFDebug for Adt27{
	unsafe fn printf_debug(&self){unsafe{printf("Adt27::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt27 {
Variant0{
fld0: i32,
fld1: f32,
fld2: f64,

},
Variant1{
fld0: u128,
fld1: usize,
fld2: isize,
fld3: [usize; 8],
fld4: f32,
fld5: *mut u8,

}}
impl PrintFDebug for Adt33{
	unsafe fn printf_debug(&self){unsafe{printf("Adt33::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt33 {
Variant0{
fld0: Adt27,
fld1: [u64; 8],
fld2: i8,

},
Variant1{
fld0: Adt27,
fld1: u16,
fld2: isize,
fld3: [usize; 8],
fld4: *const usize,

}}
impl PrintFDebug for Adt34{
	unsafe fn printf_debug(&self){unsafe{printf("Adt34::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt34 {
Variant0{
fld0: bool,
fld1: char,
fld2: isize,
fld3: Adt33,
fld4: (f32,),
fld5: i32,
fld6: f64,
fld7: i128,

},
Variant1{
fld0: u16,
fld1: char,
fld2: isize,
fld3: usize,
fld4: u64,

},
Variant2{
fld0: (char, Adt27),

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: (i128,),
fld1: *mut [usize; 8],
fld2: *mut u8,
fld3: i8,
fld4: i16,
fld5: u8,
fld6: Adt22,
fld7: [usize; 8],

},
Variant1{
fld0: (f32,),
fld1: usize,
fld2: i8,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: u32,
fld1: (f32,),

},
Variant1{
fld0: Adt34,
fld1: [usize; 8],
fld2: u8,
fld3: u32,
fld4: i16,
fld5: i32,
fld6: [u128; 6],
fld7: (char, Adt27),

},
Variant2{
fld0: i128,
fld1: u64,
fld2: f32,
fld3: (char, Adt27),

}}
impl PrintFDebug for Adt70{
	unsafe fn printf_debug(&self){unsafe{printf("Adt70::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt70 {
Variant0{
fld0: (char, Adt27),
fld1: *mut u8,

},
Variant1{
fld0: u64,
fld1: [u16; 5],
fld2: Adt22,
fld3: i32,

},
Variant2{
fld0: *const usize,
fld1: [i8; 3],

}}
impl PrintFDebug for Adt77{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt77{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt77 {
fld0: (char, Adt27),
fld1: char,
fld2: [i8; 3],
fld3: Adt48,
fld4: *const i8,
fld5: i32,
fld6: *mut [u128; 6],
fld7: (u32,),
}

