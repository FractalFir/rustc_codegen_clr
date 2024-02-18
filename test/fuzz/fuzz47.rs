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
pub fn fn0(mut _1: bool,mut _2: u64,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: u32,mut _7: u8,mut _8: u128,mut _9: usize) -> [i128; 5] {
mir! {
type RET = [i128; 5];
let _10: &'static *const i64;
let _11: isize;
let _12: ((f64, f64, i64, bool), &'static *const i64);
let _13: Adt82;
let _14: (f64, f64, i64, bool);
let _15: f32;
let _16: isize;
let _17: (*mut *const i64, i128, *const u128, ([u16; 5], *const i64));
let _18: bool;
let _19: isize;
let _20: [i128; 5];
let _21: bool;
let _22: char;
let _23: ([u16; 5], *const i64);
let _24: *const &'static i8;
let _25: isize;
let _26: &'static *const ([u16; 5], *const i64);
let _27: isize;
let _28: i8;
let _29: &'static [u8; 4];
let _30: i128;
let _31: char;
let _32: *mut u128;
let _33: &'static isize;
let _34: (u64, u16);
let _35: isize;
let _36: *const i32;
let _37: u8;
let _38: *const u16;
let _39: ();
let _40: ();
{
Goto(bb1)
}
bb1 = {
_6 = false as u32;
_1 = false;
_9 = 3385004861007518599_usize;
RET = [(-89176424117391442712182321708360717846_i128),(-3095084044439434848361765424330410622_i128),(-63818606563240696582546168291956412201_i128),(-127256479432097031451382883461747927713_i128),81002369270158910695984986134184722728_i128];
_2 = 5284429714037031222_u64 * 3160380402115604842_u64;
_4 = 101_i8 ^ 56_i8;
_8 = !160191143224407677843030198458691096747_u128;
_3 = (-1193370239208631577_i64) as isize;
RET = [37570509499554713395222790730148620425_i128,(-5578598509770941223546177763880605043_i128),21794027843680533102622083726641718835_i128,(-155497322510502545775348816389629214078_i128),(-57294249842131502925743343126814716224_i128)];
_4 = (-8_i8);
_7 = (-328699723_i32) as u8;
_4 = _2 as i8;
RET = [(-100300569677681360460567880026545021689_i128),(-43495163226415000783908739033754188639_i128),(-15025676790843342272860795240747728282_i128),(-6937066815483039578361834538553501306_i128),(-121694487445287868143021742352068575628_i128)];
_2 = !4418625775829842901_u64;
_8 = 145139937159136529129885423252611777539_u128 * 321785137320157289802191689370162692612_u128;
_11 = !_3;
_8 = (-23129_i16) as u128;
_12.0.0 = _6 as f64;
_12.0.1 = -_12.0.0;
_6 = 3207392648_u32;
_3 = _11;
_4 = !102_i8;
_8 = (-5144864221263472899_i64) as u128;
_6 = !3422655698_u32;
RET = [59704731268986871687803448232954380018_i128,(-132100291349886066204495760315505833768_i128),24344878917255299265035351541800069028_i128,159469702524898982838856173986700673368_i128,(-156055067744804035720261141312589782483_i128)];
Goto(bb2)
}
bb2 = {
_15 = _11 as f32;
_14.0 = _8 as f64;
_12.0.3 = !_1;
RET = [(-69619807556958216538781142606898102380_i128),160540016433528210798525344605313470325_i128,(-152730179427703660063515617441764292857_i128),(-84186516517672628770852500845015761605_i128),(-96814326818710618042742129714383405053_i128)];
_5 = 19960_i16 | 21790_i16;
_17.3.1 = core::ptr::addr_of!(_14.2);
_16 = _3;
_17.2 = core::ptr::addr_of!(_8);
_10 = &_17.3.1;
_12.0 = (_14.0, _14.0, 4639181848061354977_i64, _1);
_6 = !2756047845_u32;
_3 = -_11;
_14.3 = _9 != _9;
_6 = 4279216699_u32;
_1 = _12.0.3;
_17.3.0 = [61465_u16,52837_u16,61162_u16,51661_u16,56191_u16];
_17.1 = (-114564036519644818884962211832901443023_i128);
_12.1 = &(*_10);
_6 = 2187518640_u32;
_17.3.0 = [56330_u16,32708_u16,3347_u16,16085_u16,21655_u16];
_14.1 = _12.0.1 - _12.0.0;
Call(_7 = core::intrinsics::transmute(_14.3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = _7 as i8;
Call(_5 = fn1(Move(_12), _1, _16, _14.1, RET, _14.1, _3, RET, RET), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_12.0.0 = 43862_u16 as f64;
_8 = _17.1 as u128;
_17.3.0 = [39476_u16,26636_u16,55794_u16,1168_u16,56256_u16];
_12.0.1 = _17.1 as f64;
_2 = 754536455656677891_u64 << _4;
_12.0.3 = _1 ^ _1;
_18 = !_14.3;
_1 = _18;
_14.2 = _6 as i64;
_12.0 = _14;
_12 = (_14, Move(_10));
_9 = 3_usize << _3;
_17.1 = _7 as i128;
Goto(bb5)
}
bb5 = {
_14 = (_12.0.0, _12.0.0, _12.0.2, _1);
_12.0.3 = _1;
_17.3.1 = core::ptr::addr_of!(_14.2);
RET = [_17.1,_17.1,_17.1,_17.1,_17.1];
_9 = 4_usize + 3_usize;
_19 = _16 - _11;
_5 = (-22021_i16) * 18462_i16;
_22 = '\u{e80ef}';
_10 = &_17.3.1;
_17.2 = core::ptr::addr_of!(_8);
_18 = !_12.0.3;
Goto(bb6)
}
bb6 = {
_12.0.1 = _17.1 as f64;
_12.0.0 = _17.1 as f64;
_14.0 = _19 as f64;
_19 = _3 + _3;
_17.3.0 = [50095_u16,45729_u16,7510_u16,28525_u16,13119_u16];
_17.0 = core::ptr::addr_of_mut!((*_10));
_4 = (-16_i8);
_12.1 = Move(_10);
_5 = (-22583_i16) >> _17.1;
_20 = [_17.1,_17.1,_17.1,_17.1,_17.1];
_23.1 = core::ptr::addr_of!(_14.2);
_2 = 17907483324533983142_u64;
_14.3 = !_1;
_3 = -_16;
_21 = _12.0.3 & _14.3;
_14 = (_12.0.1, _12.0.0, _12.0.2, _21);
_1 = _12.0.3;
_12.0.0 = _12.0.1;
match _6 {
0 => bb7,
2187518640 => bb9,
_ => bb8
}
}
bb7 = {
_14 = (_12.0.0, _12.0.0, _12.0.2, _1);
_12.0.3 = _1;
_17.3.1 = core::ptr::addr_of!(_14.2);
RET = [_17.1,_17.1,_17.1,_17.1,_17.1];
_9 = 4_usize + 3_usize;
_19 = _16 - _11;
_5 = (-22021_i16) * 18462_i16;
_22 = '\u{e80ef}';
_10 = &_17.3.1;
_17.2 = core::ptr::addr_of!(_8);
_18 = !_12.0.3;
Goto(bb6)
}
bb8 = {
_12.0.0 = 43862_u16 as f64;
_8 = _17.1 as u128;
_17.3.0 = [39476_u16,26636_u16,55794_u16,1168_u16,56256_u16];
_12.0.1 = _17.1 as f64;
_2 = 754536455656677891_u64 << _4;
_12.0.3 = _1 ^ _1;
_18 = !_14.3;
_1 = _18;
_14.2 = _6 as i64;
_12.0 = _14;
_12 = (_14, Move(_10));
_9 = 3_usize << _3;
_17.1 = _7 as i128;
Goto(bb5)
}
bb9 = {
_5 = (-19217_i16);
_1 = _12.0.1 < _14.1;
_23.1 = core::ptr::addr_of!(_14.2);
_14.0 = -_12.0.0;
_10 = &_17.3.1;
_22 = '\u{bbed0}';
_15 = _12.0.0 as f32;
_4 = _14.0 as i8;
_14.0 = -_12.0.1;
_21 = !_14.3;
_19 = _11;
_14.1 = _12.0.0;
_12 = (_14, Move(_10));
_12.0.3 = !_21;
Goto(bb10)
}
bb10 = {
_30 = _17.1;
_12.1 = &_23.1;
_16 = _11 * _3;
_9 = !2889647810889490246_usize;
_31 = _22;
_14.3 = !_18;
RET = _20;
RET = [_30,_17.1,_30,_30,_30];
_6 = !4038997459_u32;
_12.0 = (_14.0, _14.1, _14.2, _18);
_6 = _14.2 as u32;
_23 = Move(_17.3);
_23.0 = [18584_u16,19282_u16,47361_u16,61776_u16,55432_u16];
Goto(bb11)
}
bb11 = {
_10 = &_23.1;
_28 = !_4;
_10 = &_23.1;
_12.0.1 = -_14.1;
_19 = _14.3 as isize;
_4 = _28 << _9;
_28 = !_4;
_16 = _19;
_17.0 = core::ptr::addr_of_mut!((*_10));
_8 = !315985038776500433167323665074820821491_u128;
_32 = core::ptr::addr_of_mut!(_8);
_6 = 1062888795_u32 - 1537087208_u32;
_34.0 = _2;
_15 = _19 as f32;
_14.2 = _12.0.2 & _12.0.2;
_33 = &_3;
_34 = (_2, 532_u16);
_9 = _12.0.2 as usize;
_17.0 = core::ptr::addr_of_mut!((*_10));
_34 = (_2, 52674_u16);
_3 = _16;
(*_32) = 775701343391273447207628596664172113_u128 - 61681332818434823629376952353979787512_u128;
_17.3.0 = _23.0;
_16 = _19 ^ _3;
_5 = (-16647_i16) >> (*_32);
_17.3.1 = core::ptr::addr_of!(_12.0.2);
_27 = -_11;
match _34.1 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb9,
4 => bb10,
5 => bb12,
6 => bb13,
52674 => bb15,
_ => bb14
}
}
bb12 = {
_12.0.1 = _17.1 as f64;
_12.0.0 = _17.1 as f64;
_14.0 = _19 as f64;
_19 = _3 + _3;
_17.3.0 = [50095_u16,45729_u16,7510_u16,28525_u16,13119_u16];
_17.0 = core::ptr::addr_of_mut!((*_10));
_4 = (-16_i8);
_12.1 = Move(_10);
_5 = (-22583_i16) >> _17.1;
_20 = [_17.1,_17.1,_17.1,_17.1,_17.1];
_23.1 = core::ptr::addr_of!(_14.2);
_2 = 17907483324533983142_u64;
_14.3 = !_1;
_3 = -_16;
_21 = _12.0.3 & _14.3;
_14 = (_12.0.1, _12.0.0, _12.0.2, _21);
_1 = _12.0.3;
_12.0.0 = _12.0.1;
match _6 {
0 => bb7,
2187518640 => bb9,
_ => bb8
}
}
bb13 = {
_5 = (-19217_i16);
_1 = _12.0.1 < _14.1;
_23.1 = core::ptr::addr_of!(_14.2);
_14.0 = -_12.0.0;
_10 = &_17.3.1;
_22 = '\u{bbed0}';
_15 = _12.0.0 as f32;
_4 = _14.0 as i8;
_14.0 = -_12.0.1;
_21 = !_14.3;
_19 = _11;
_14.1 = _12.0.0;
_12 = (_14, Move(_10));
_12.0.3 = !_21;
Goto(bb10)
}
bb14 = {
_12.0.0 = 43862_u16 as f64;
_8 = _17.1 as u128;
_17.3.0 = [39476_u16,26636_u16,55794_u16,1168_u16,56256_u16];
_12.0.1 = _17.1 as f64;
_2 = 754536455656677891_u64 << _4;
_12.0.3 = _1 ^ _1;
_18 = !_14.3;
_1 = _18;
_14.2 = _6 as i64;
_12.0 = _14;
_12 = (_14, Move(_10));
_9 = 3_usize << _3;
_17.1 = _7 as i128;
Goto(bb5)
}
bb15 = {
_14.3 = !_1;
_23 = (_17.3.0, Move(_17.3.1));
_23.1 = core::ptr::addr_of!(_14.2);
_34.1 = 61535_u16;
_14.3 = _21 | _1;
_7 = _31 as u8;
_10 = &_17.3.1;
_33 = &_16;
_38 = core::ptr::addr_of!(_34.1);
Goto(bb16)
}
bb16 = {
Call(_39 = dump_var(0_usize, 1_usize, Move(_1), 9_usize, Move(_9), 7_usize, Move(_7), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(0_usize, 28_usize, Move(_28), 20_usize, Move(_20), 6_usize, Move(_6), 27_usize, Move(_27)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_39 = dump_var(0_usize, 22_usize, Move(_22), 4_usize, Move(_4), 40_usize, _40, 40_usize, _40), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: ((f64, f64, i64, bool), &'static *const i64),mut _2: bool,mut _3: isize,mut _4: f64,mut _5: [i128; 5],mut _6: f64,mut _7: isize,mut _8: [i128; 5],mut _9: [i128; 5]) -> i16 {
mir! {
type RET = i16;
let _10: *const [char; 6];
let _11: ([u8; 4], char, [isize; 3]);
let _12: (f32,);
let _13: *const u128;
let _14: f64;
let _15: *const u16;
let _16: isize;
let _17: usize;
let _18: f32;
let _19: isize;
let _20: f32;
let _21: &'static [usize; 8];
let _22: Adt32;
let _23: ();
let _24: ();
{
RET = (-25662_i16) & 8634_i16;
_1.0.1 = _4;
Call(_10 = fn2(_1.0.2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = _1.0.3;
_1.0.1 = _6 * _4;
_9 = [(-43575872252292903040753832506303008367_i128),(-56097221168868945970616511926534241705_i128),(-130509539648011951789091980604948595663_i128),(-119379639330585099449215858552650145010_i128),(-78487791120208418182821404762876778961_i128)];
_1.0.2 = -(-1458129995467637150_i64);
_8 = [(-46600047670470588180767938335433550251_i128),76975366440786137476910130851486944840_i128,(-130011148519986622518537341628763023213_i128),59631769322627815606258438220291395272_i128,46656503680412734546027054342296138215_i128];
_6 = _4 + _1.0.1;
_9 = [(-35928014490345153894533919547137153001_i128),141579190763244973625955683288664361316_i128,155449614676427556267705453511424173172_i128,(-97196702295341156296533067815056029333_i128),(-33922382990892153471572899586771483973_i128)];
_9 = [(-112508357320701956982975015429833668139_i128),(-155711626963957375325470549583492760750_i128),134999267841754083805603649134522933885_i128,120561611285716017327150591522774759315_i128,(-155714863895832611466431491830645298003_i128)];
_1.0.3 = !_2;
_7 = -_3;
_2 = !_1.0.3;
_9 = [101783173751829396032827693877348379381_i128,(-166778664511724440421162722721057712367_i128),137891192007845817683514686748585272877_i128,135498576533923766618446840028211294477_i128,76607609432363016936136467087014778236_i128];
RET = (-22560_i16) ^ 19432_i16;
_9 = [(-69426061756886465789679148684941666356_i128),(-7660847512330528880763611118729277116_i128),71425830508088160408565685341510190644_i128,96123574264496419232448784927101523834_i128,101582058775375288137524511270592260665_i128];
_1.0.2 = RET as i64;
_9 = _8;
_3 = !_7;
_1.0.0 = -_1.0.1;
_2 = !_1.0.3;
_6 = 15111729842869032941_u64 as f64;
_3 = _7;
Goto(bb2)
}
bb2 = {
_1.0.2 = (-2137562248175788142_i64) | 298946526941800461_i64;
_2 = !_1.0.3;
_2 = _1.0.3 ^ _1.0.3;
_7 = _3;
_1.0 = (_6, _6, 3945743253883243483_i64, _2);
_4 = _1.0.1;
_1.0.1 = _4 * _4;
_9 = _8;
Goto(bb3)
}
bb3 = {
_1.0.2 = -(-3264036948314104692_i64);
_1.0.2 = !(-6518239593132302673_i64);
_4 = -_6;
_1.0.2 = !(-7668873438973869711_i64);
_1.0.1 = -_6;
RET = (-26861_i16);
_14 = _1.0.1;
RET = -(-30021_i16);
_7 = !_3;
_12.0 = 3976189877_u32 as f32;
_1.0.1 = _6;
_5 = [(-22441807350167514602337286923194989251_i128),(-36032501876191084723863952710571441548_i128),95359658663473739776470258679799058358_i128,(-139787084185018525226076544661580991268_i128),(-35259249425223416450518202386398033910_i128)];
RET = (-10131_i16) | (-25584_i16);
_12.0 = _3 as f32;
_1.0 = (_4, _4, (-7231304188719200493_i64), _2);
_6 = _1.0.0;
_12.0 = 262216704779122605504351891823658137440_u128 as f32;
_11.0 = [132_u8,57_u8,98_u8,217_u8];
_14 = 1121362910_i32 as f64;
_7 = 89394547700097943277088105754720897568_i128 as isize;
_1.0 = (_14, _4, (-6292945908754344769_i64), _2);
_1.0.0 = _4;
_11.2 = [_3,_7,_7];
_11.2 = [_7,_7,_7];
_8 = [117925344919761793093969661635789536422_i128,123652187989420500530032929213994393988_i128,(-6828868690120926371449531706887376659_i128),144573641364930816297524601759102861286_i128,(-131022689532632572617472307216295614235_i128)];
_1.0.1 = _14;
_1.0.0 = _1.0.2 as f64;
_7 = _3 ^ _3;
_1.0.1 = 8166630215178651470_u64 as f64;
match _1.0.2 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
340282366920938463457081661523013866687 => bb12,
_ => bb11
}
}
bb4 = {
_1.0.2 = (-2137562248175788142_i64) | 298946526941800461_i64;
_2 = !_1.0.3;
_2 = _1.0.3 ^ _1.0.3;
_7 = _3;
_1.0 = (_6, _6, 3945743253883243483_i64, _2);
_4 = _1.0.1;
_1.0.1 = _4 * _4;
_9 = _8;
Goto(bb3)
}
bb5 = {
_2 = _1.0.3;
_1.0.1 = _6 * _4;
_9 = [(-43575872252292903040753832506303008367_i128),(-56097221168868945970616511926534241705_i128),(-130509539648011951789091980604948595663_i128),(-119379639330585099449215858552650145010_i128),(-78487791120208418182821404762876778961_i128)];
_1.0.2 = -(-1458129995467637150_i64);
_8 = [(-46600047670470588180767938335433550251_i128),76975366440786137476910130851486944840_i128,(-130011148519986622518537341628763023213_i128),59631769322627815606258438220291395272_i128,46656503680412734546027054342296138215_i128];
_6 = _4 + _1.0.1;
_9 = [(-35928014490345153894533919547137153001_i128),141579190763244973625955683288664361316_i128,155449614676427556267705453511424173172_i128,(-97196702295341156296533067815056029333_i128),(-33922382990892153471572899586771483973_i128)];
_9 = [(-112508357320701956982975015429833668139_i128),(-155711626963957375325470549583492760750_i128),134999267841754083805603649134522933885_i128,120561611285716017327150591522774759315_i128,(-155714863895832611466431491830645298003_i128)];
_1.0.3 = !_2;
_7 = -_3;
_2 = !_1.0.3;
_9 = [101783173751829396032827693877348379381_i128,(-166778664511724440421162722721057712367_i128),137891192007845817683514686748585272877_i128,135498576533923766618446840028211294477_i128,76607609432363016936136467087014778236_i128];
RET = (-22560_i16) ^ 19432_i16;
_9 = [(-69426061756886465789679148684941666356_i128),(-7660847512330528880763611118729277116_i128),71425830508088160408565685341510190644_i128,96123574264496419232448784927101523834_i128,101582058775375288137524511270592260665_i128];
_1.0.2 = RET as i64;
_9 = _8;
_3 = !_7;
_1.0.0 = -_1.0.1;
_2 = !_1.0.3;
_6 = 15111729842869032941_u64 as f64;
_3 = _7;
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
_2 = _6 <= _4;
_1.0.3 = _2;
_11.0 = [125_u8,106_u8,176_u8,206_u8];
_3 = _7;
_11.0 = [38_u8,167_u8,150_u8,182_u8];
_6 = _1.0.0;
_1.0.0 = _14;
_16 = (-1550006042_i32) as isize;
_1.0.0 = _6 * _6;
RET = (-11427_i16) | 11781_i16;
_1.0 = (_4, _4, 7912689226481018288_i64, _2);
match _1.0.2 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb13,
7912689226481018288 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_3 = _7 >> RET;
_4 = _1.0.1;
_11.2 = [_3,_7,_16];
_20 = -_12.0;
_18 = _20 * _20;
_18 = -_20;
RET = 27859_i16 - 12399_i16;
_9 = [72303893409785742972396078276534965183_i128,(-6615765516046500980297087176938174683_i128),68015827182618978252036348822142701429_i128,107052077929023870074689381186648444529_i128,(-147128111644241303926890441687643728337_i128)];
_18 = _20;
_11.2 = [_3,_7,_3];
_20 = _18 + _12.0;
_1.0.2 = (-7021584492792098724_i64) - 7438345329576172451_i64;
RET = (-25087_i16);
_7 = _12.0 as isize;
_11.2 = [_3,_3,_16];
Goto(bb16)
}
bb16 = {
Call(_23 = dump_var(1_usize, 16_usize, Move(_16), 9_usize, Move(_9), 2_usize, Move(_2), 24_usize, _24), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: i64,mut _2: bool) -> *const [char; 6] {
mir! {
type RET = *const [char; 6];
let _3: u32;
let _4: char;
let _5: &'static [usize; 8];
let _6: *mut *const i64;
let _7: &'static *const i64;
let _8: &'static *const &'static i8;
let _9: bool;
let _10: u64;
let _11: u32;
let _12: i32;
let _13: i16;
let _14: f64;
let _15: i128;
let _16: [i16; 1];
let _17: f64;
let _18: f64;
let _19: bool;
let _20: [u64; 3];
let _21: (i64, *const u16, &'static isize, *const u16);
let _22: u16;
let _23: &'static &'static u64;
let _24: &'static i8;
let _25: Adt32;
let _26: char;
let _27: &'static i8;
let _28: usize;
let _29: [i16; 1];
let _30: u64;
let _31: i32;
let _32: (f64, f64, i64, bool);
let _33: (f64, f64, i64, bool);
let _34: isize;
let _35: *mut [bool; 3];
let _36: u32;
let _37: f64;
let _38: [u64; 3];
let _39: [u16; 5];
let _40: isize;
let _41: bool;
let _42: isize;
let _43: &'static char;
let _44: usize;
let _45: *const i16;
let _46: [char; 8];
let _47: f32;
let _48: i64;
let _49: [char; 6];
let _50: isize;
let _51: ();
let _52: ();
{
_2 = !true;
_1 = 24_i8 as i64;
_2 = _1 <= _1;
_2 = true | false;
_2 = _1 <= _1;
_2 = !true;
_2 = _1 <= _1;
_1 = -(-5477271547370237938_i64);
_4 = '\u{90a6d}';
_3 = 2423980822_u32;
_2 = _3 == _3;
_1 = 47074_u16 as i64;
_4 = '\u{a65a9}';
_2 = false;
_1 = !6384922706132452986_i64;
_2 = !true;
_4 = '\u{989f5}';
_2 = !true;
_2 = _3 > _3;
_3 = 924373134_u32 - 1837128810_u32;
_1 = (-111770691027806827516715469622639020094_i128) as i64;
Call(_4 = fn3(_3, _3, _2, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = 3774052559_u32 & 4210984306_u32;
_2 = false;
_3 = !449140697_u32;
_2 = false;
_3 = 3337968071_u32;
_1 = 219080231534166913_i64 << _3;
_2 = !false;
_1 = 26088_u16 as i64;
_4 = '\u{29c11}';
_3 = 3097321196_u32 - 1575512148_u32;
_4 = '\u{1eeef}';
_4 = '\u{10a844}';
_3 = !3421294249_u32;
_2 = false;
_3 = 2514438649_u32;
match _3 {
0 => bb2,
2514438649 => bb4,
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
_3 = 4063717366833925888_u64 as u32;
_9 = !_2;
_9 = _2;
_9 = _2;
_10 = 5405295346030176925_u64 - 14737392785266658564_u64;
Goto(bb5)
}
bb5 = {
_9 = _3 <= _3;
_2 = _3 == _3;
_9 = _4 != _4;
_4 = '\u{26fb6}';
_9 = _3 == _3;
_9 = !_2;
_10 = 18071271117127331571_u64;
_3 = (-76_i8) as u32;
Goto(bb6)
}
bb6 = {
_3 = !462758059_u32;
_2 = _9 <= _9;
_2 = _9;
_3 = 3222496720_u32;
_1 = 11624_i16 as i64;
Goto(bb7)
}
bb7 = {
_9 = _2 | _2;
_4 = '\u{91cbf}';
_3 = !2481660683_u32;
_9 = _2;
_12 = 1934463873_i32 ^ (-1798992573_i32);
_1 = -(-3824374492227418055_i64);
_12 = 1155989281_i32;
_4 = '\u{a3ac6}';
_9 = _2 >= _2;
_10 = 16011573440122335309_u64 + 17466630658432044844_u64;
_11 = _3 | _3;
_11 = !_3;
_14 = _12 as f64;
_9 = _3 == _11;
Goto(bb8)
}
bb8 = {
_1 = (-2796570772161441460_i64);
_16 = [(-27552_i16)];
_10 = !13571233874131376488_u64;
Goto(bb9)
}
bb9 = {
_10 = 433248125883190813_u64;
_11 = _3 + _3;
_9 = !_2;
_14 = 7534_i16 as f64;
_3 = 215_u8 as u32;
_13 = !32650_i16;
_1 = -4140836399887357022_i64;
_10 = 4131934293149676060_u64 << _13;
_13 = (-27248_i16) & (-8971_i16);
_17 = -_14;
_11 = !_3;
_9 = _2;
_18 = 9223372036854775807_isize as f64;
_4 = '\u{5e17f}';
_4 = '\u{5138b}';
_11 = !_3;
match _12 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb4,
4 => bb5,
5 => bb6,
1155989281 => bb11,
_ => bb10
}
}
bb10 = {
_9 = _2 | _2;
_4 = '\u{91cbf}';
_3 = !2481660683_u32;
_9 = _2;
_12 = 1934463873_i32 ^ (-1798992573_i32);
_1 = -(-3824374492227418055_i64);
_12 = 1155989281_i32;
_4 = '\u{a3ac6}';
_9 = _2 >= _2;
_10 = 16011573440122335309_u64 + 17466630658432044844_u64;
_11 = _3 | _3;
_11 = !_3;
_14 = _12 as f64;
_9 = _3 == _11;
Goto(bb8)
}
bb11 = {
_1 = 11772819364288682717_usize as i64;
_16 = [_13];
_13 = 6593_i16;
_1 = 4057640365678216424_i64;
_17 = _14 + _18;
_1 = 2989949884629198065_i64 * (-3495895641379323246_i64);
_18 = _14 - _17;
_2 = _9;
_18 = _17 + _17;
_9 = !_2;
_2 = !_9;
_9 = _2 & _2;
_10 = 15385886689275490601_u64 + 15197665535261272312_u64;
_1 = 23584599026837843067118351022729181456_i128 as i64;
_4 = '\u{ec343}';
_13 = (-25031_i16) & 4819_i16;
_22 = !54142_u16;
_20 = [_10,_10,_10];
_22 = 22770_u16;
_17 = _14 * _14;
_21.1 = core::ptr::addr_of!(_22);
Goto(bb12)
}
bb12 = {
_20 = [_10,_10,_10];
_4 = '\u{55e36}';
_15 = (-63396381752084098781899304952495140968_i128) ^ 90250220167108553154940962171544906644_i128;
_20 = [_10,_10,_10];
_21.3 = Move(_21.1);
Goto(bb13)
}
bb13 = {
_20 = [_10,_10,_10];
_22 = 10766_u16;
_10 = _9 as u64;
_17 = -_18;
_19 = _2;
_12 = _15 as i32;
_13 = 1819_i16;
_21.0 = _15 as i64;
_21.1 = Move(_21.3);
_13 = 9587470236780028238_usize as i16;
_18 = -_17;
_10 = 11857396164911660520_u64;
Goto(bb14)
}
bb14 = {
_4 = '\u{a57f}';
_21.3 = core::ptr::addr_of!(_22);
_20 = [_10,_10,_10];
_3 = _21.0 as u32;
_4 = '\u{f0ca9}';
_14 = _17;
_22 = 46033_u16 << _13;
_18 = -_17;
_21.3 = Move(_21.1);
_26 = _4;
_21.1 = Move(_21.3);
_12 = 13100345996670708391_usize as i32;
_13 = 3265_i16 | 25159_i16;
_14 = _10 as f64;
_28 = _15 as usize;
_3 = _11;
_14 = _17 + _17;
_20 = [_10,_10,_10];
_21.3 = Move(_21.1);
_29 = _16;
_16 = [_13];
_28 = !0_usize;
_17 = -_14;
_20 = [_10,_10,_10];
_28 = 15050125812117101191_usize;
Call(_28 = fn19(_17, _17, _17, _17, _21.0), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_16 = [_13];
_19 = !_9;
_31 = -_12;
_30 = _10 << _21.0;
_4 = _26;
_4 = _26;
_18 = _21.0 as f64;
_21.0 = 78_i8 as i64;
_11 = _3 << _13;
_32.3 = _9;
_33.3 = !_9;
_32.0 = -_14;
_33.1 = _31 as f64;
_21.2 = &_34;
_29 = _16;
_33.0 = _30 as f64;
_32.3 = !_33.3;
_21.1 = Move(_21.3);
_30 = _10 << _31;
match _28 {
0 => bb1,
1 => bb11,
2 => bb3,
3 => bb16,
4 => bb17,
5 => bb18,
6 => bb20,
_ => bb19
}
}
bb16 = {
_1 = (-2796570772161441460_i64);
_16 = [(-27552_i16)];
_10 = !13571233874131376488_u64;
Goto(bb9)
}
bb17 = {
_9 = _2 | _2;
_4 = '\u{91cbf}';
_3 = !2481660683_u32;
_9 = _2;
_12 = 1934463873_i32 ^ (-1798992573_i32);
_1 = -(-3824374492227418055_i64);
_12 = 1155989281_i32;
_4 = '\u{a3ac6}';
_9 = _2 >= _2;
_10 = 16011573440122335309_u64 + 17466630658432044844_u64;
_11 = _3 | _3;
_11 = !_3;
_14 = _12 as f64;
_9 = _3 == _11;
Goto(bb8)
}
bb18 = {
_3 = 3774052559_u32 & 4210984306_u32;
_2 = false;
_3 = !449140697_u32;
_2 = false;
_3 = 3337968071_u32;
_1 = 219080231534166913_i64 << _3;
_2 = !false;
_1 = 26088_u16 as i64;
_4 = '\u{29c11}';
_3 = 3097321196_u32 - 1575512148_u32;
_4 = '\u{1eeef}';
_4 = '\u{10a844}';
_3 = !3421294249_u32;
_2 = false;
_3 = 2514438649_u32;
match _3 {
0 => bb2,
2514438649 => bb4,
_ => bb3
}
}
bb19 = {
_3 = !462758059_u32;
_2 = _9 <= _9;
_2 = _9;
_3 = 3222496720_u32;
_1 = 11624_i16 as i64;
Goto(bb7)
}
bb20 = {
_4 = _26;
_16 = [_13];
_2 = _33.3 > _19;
_32.2 = _10 as i64;
_38 = [_30,_30,_30];
_11 = _2 as u32;
_33 = (_32.0, _17, _32.2, _19);
_37 = _11 as f64;
_14 = _37 * _37;
_32.1 = _14 + _33.0;
_40 = !(-44_isize);
_19 = _32.3;
_1 = _21.0;
_41 = !_32.3;
_32 = (_37, _33.0, _33.2, _2);
_33.0 = _32.1 - _17;
_21.2 = &_34;
_17 = _13 as f64;
_31 = _12;
_10 = !_30;
_13 = (-23376_i16) + (-19178_i16);
_30 = _10 ^ _10;
_17 = -_14;
match _28 {
0 => bb8,
1 => bb21,
2 => bb22,
3 => bb23,
4 => bb24,
5 => bb25,
6 => bb28,
_ => bb27
}
}
bb21 = {
_20 = [_10,_10,_10];
_4 = '\u{55e36}';
_15 = (-63396381752084098781899304952495140968_i128) ^ 90250220167108553154940962171544906644_i128;
_20 = [_10,_10,_10];
_21.3 = Move(_21.1);
Goto(bb13)
}
bb22 = {
_9 = _3 <= _3;
_2 = _3 == _3;
_9 = _4 != _4;
_4 = '\u{26fb6}';
_9 = _3 == _3;
_9 = !_2;
_10 = 18071271117127331571_u64;
_3 = (-76_i8) as u32;
Goto(bb6)
}
bb23 = {
_1 = (-2796570772161441460_i64);
_16 = [(-27552_i16)];
_10 = !13571233874131376488_u64;
Goto(bb9)
}
bb24 = {
_1 = (-2796570772161441460_i64);
_16 = [(-27552_i16)];
_10 = !13571233874131376488_u64;
Goto(bb9)
}
bb25 = {
_9 = _2 | _2;
_4 = '\u{91cbf}';
_3 = !2481660683_u32;
_9 = _2;
_12 = 1934463873_i32 ^ (-1798992573_i32);
_1 = -(-3824374492227418055_i64);
_12 = 1155989281_i32;
_4 = '\u{a3ac6}';
_9 = _2 >= _2;
_10 = 16011573440122335309_u64 + 17466630658432044844_u64;
_11 = _3 | _3;
_11 = !_3;
_14 = _12 as f64;
_9 = _3 == _11;
Goto(bb8)
}
bb26 = {
_3 = 3774052559_u32 & 4210984306_u32;
_2 = false;
_3 = !449140697_u32;
_2 = false;
_3 = 3337968071_u32;
_1 = 219080231534166913_i64 << _3;
_2 = !false;
_1 = 26088_u16 as i64;
_4 = '\u{29c11}';
_3 = 3097321196_u32 - 1575512148_u32;
_4 = '\u{1eeef}';
_4 = '\u{10a844}';
_3 = !3421294249_u32;
_2 = false;
_3 = 2514438649_u32;
match _3 {
0 => bb2,
2514438649 => bb4,
_ => bb3
}
}
bb27 = {
_9 = _2 | _2;
_4 = '\u{91cbf}';
_3 = !2481660683_u32;
_9 = _2;
_12 = 1934463873_i32 ^ (-1798992573_i32);
_1 = -(-3824374492227418055_i64);
_12 = 1155989281_i32;
_4 = '\u{a3ac6}';
_9 = _2 >= _2;
_10 = 16011573440122335309_u64 + 17466630658432044844_u64;
_11 = _3 | _3;
_11 = !_3;
_14 = _12 as f64;
_9 = _3 == _11;
Goto(bb8)
}
bb28 = {
_39 = [_22,_22,_22,_22,_22];
_1 = _32.2 * _32.2;
_1 = _22 as i64;
_43 = &_26;
_33 = _32;
_42 = !_40;
_44 = !_28;
_32 = _33;
_31 = _11 as i32;
_43 = &(*_43);
_34 = _42;
_21.1 = core::ptr::addr_of!(_22);
_21.0 = -_1;
_46[_28] = (*_43);
_28 = !_44;
_9 = _33.3;
_30 = _1 as u64;
_37 = _17 - _14;
_26 = _4;
_29 = [_13];
_44 = _31 as usize;
Call(_32.1 = core::intrinsics::transmute(_1), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
RET = core::ptr::addr_of!(_49);
(*RET) = [_4,_26,_4,_26,_4,_4];
_22 = 339183801639580555543569330016250976991_u128 as u16;
_46 = [_4,_4,_4,_26,_26,_26,_4,_4];
RET = core::ptr::addr_of!((*RET));
_32.1 = _21.0 as f64;
_45 = core::ptr::addr_of!(_13);
_21.3 = core::ptr::addr_of!(_22);
_34 = _10 as isize;
(*RET) = [_4,_26,_4,_4,_4,_4];
_37 = -_33.0;
_15 = !(-138037793051296234898845608211263706355_i128);
_46 = [_26,_26,_4,_4,_4,_4,_26,_26];
_26 = _4;
_32.0 = _44 as f64;
_50 = _34 * _34;
Goto(bb30)
}
bb30 = {
Call(_51 = dump_var(2_usize, 39_usize, Move(_39), 50_usize, Move(_50), 10_usize, Move(_10), 38_usize, Move(_38)), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
Call(_51 = dump_var(2_usize, 3_usize, Move(_3), 2_usize, Move(_2), 31_usize, Move(_31), 20_usize, Move(_20)), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
Call(_51 = dump_var(2_usize, 13_usize, Move(_13), 42_usize, Move(_42), 49_usize, Move(_49), 4_usize, Move(_4)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Call(_51 = dump_var(2_usize, 44_usize, Move(_44), 29_usize, Move(_29), 52_usize, _52, 52_usize, _52), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: u32,mut _2: u32,mut _3: bool,mut _4: u32) -> char {
mir! {
type RET = char;
let _5: isize;
let _6: f32;
let _7: f32;
let _8: *const u128;
let _9: &'static (u16,);
let _10: *const i64;
let _11: i64;
let _12: i64;
let _13: f32;
let _14: (f64, f64, i64, bool);
let _15: i32;
let _16: bool;
let _17: &'static char;
let _18: [isize; 3];
let _19: i8;
let _20: bool;
let _21: Adt63;
let _22: [u16; 5];
let _23: ();
let _24: ();
{
_5 = !(-121_isize);
RET = '\u{feb80}';
_1 = _2;
RET = '\u{a4d4}';
_5 = 10_isize * (-9223372036854775808_isize);
_6 = (-8628802205925482776_i64) as f32;
_6 = 4_usize as f32;
_2 = _1 | _1;
RET = '\u{63bb8}';
_3 = _4 == _1;
RET = '\u{a948c}';
_5 = 120_isize | (-67_isize);
RET = '\u{307bd}';
_2 = _3 as u32;
RET = '\u{ec064}';
_3 = true ^ true;
_2 = _4 * _1;
_2 = _1;
RET = '\u{67d1b}';
Call(RET = fn4(_2, _5, _5, _4, _5, _6, _1, _3, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = '\u{8f5de}';
RET = '\u{7930}';
_6 = 113_u8 as f32;
_1 = 165661276148381716648016242249622772775_i128 as u32;
_7 = _6 * _6;
_1 = 70971595012296348068061664777121336295_u128 as u32;
_3 = _2 > _2;
RET = '\u{f1458}';
RET = '\u{c7f4c}';
_3 = false;
_5 = 9223372036854775807_isize * (-9223372036854775808_isize);
_1 = _2;
_1 = _2 | _4;
RET = '\u{8e7c7}';
_6 = _7;
Goto(bb2)
}
bb2 = {
RET = '\u{4aa69}';
RET = '\u{ec263}';
_5 = 2119023555_i32 as isize;
_7 = _6 - _6;
_2 = !_1;
RET = '\u{cd3c2}';
_6 = _7;
_3 = !false;
RET = '\u{4699b}';
RET = '\u{8cbff}';
RET = '\u{50f7c}';
_4 = _1;
RET = '\u{86a7f}';
_5 = (-124_isize);
_6 = 62161957_i32 as f32;
_7 = _6;
_1 = _2 * _2;
_10 = core::ptr::addr_of!(_11);
_11 = 3471465082441875451_i64;
_1 = (-60_i8) as u32;
_14.0 = 120_u8 as f64;
_1 = !_2;
_2 = _1 >> _4;
match _5 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
340282366920938463463374607431768211332 => bb11,
_ => bb10
}
}
bb3 = {
RET = '\u{8f5de}';
RET = '\u{7930}';
_6 = 113_u8 as f32;
_1 = 165661276148381716648016242249622772775_i128 as u32;
_7 = _6 * _6;
_1 = 70971595012296348068061664777121336295_u128 as u32;
_3 = _2 > _2;
RET = '\u{f1458}';
RET = '\u{c7f4c}';
_3 = false;
_5 = 9223372036854775807_isize * (-9223372036854775808_isize);
_1 = _2;
_1 = _2 | _4;
RET = '\u{8e7c7}';
_6 = _7;
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
Return()
}
bb11 = {
_17 = &RET;
_14.0 = 32085_u16 as f64;
_14.2 = (*_10) + (*_10);
_3 = !true;
Goto(bb12)
}
bb12 = {
_11 = !_14.2;
_14.3 = (*_10) > _14.2;
RET = '\u{9891e}';
_14.2 = 6762075913805019086_usize as i64;
match _5 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
340282366920938463463374607431768211332 => bb18,
_ => bb17
}
}
bb13 = {
Return()
}
bb14 = {
RET = '\u{8f5de}';
RET = '\u{7930}';
_6 = 113_u8 as f32;
_1 = 165661276148381716648016242249622772775_i128 as u32;
_7 = _6 * _6;
_1 = 70971595012296348068061664777121336295_u128 as u32;
_3 = _2 > _2;
RET = '\u{f1458}';
RET = '\u{c7f4c}';
_3 = false;
_5 = 9223372036854775807_isize * (-9223372036854775808_isize);
_1 = _2;
_1 = _2 | _4;
RET = '\u{8e7c7}';
_6 = _7;
Goto(bb2)
}
bb15 = {
Return()
}
bb16 = {
RET = '\u{8f5de}';
RET = '\u{7930}';
_6 = 113_u8 as f32;
_1 = 165661276148381716648016242249622772775_i128 as u32;
_7 = _6 * _6;
_1 = 70971595012296348068061664777121336295_u128 as u32;
_3 = _2 > _2;
RET = '\u{f1458}';
RET = '\u{c7f4c}';
_3 = false;
_5 = 9223372036854775807_isize * (-9223372036854775808_isize);
_1 = _2;
_1 = _2 | _4;
RET = '\u{8e7c7}';
_6 = _7;
Goto(bb2)
}
bb17 = {
Return()
}
bb18 = {
_14.3 = !_3;
_4 = _1 & _1;
_16 = _5 != _5;
RET = '\u{da347}';
_12 = 1955543248_i32 as i64;
_21.fld4 = [26822_u16,23708_u16,36239_u16,29058_u16,48099_u16];
_19 = !(-59_i8);
_21.fld0.0 = RET as u16;
_6 = _7;
_14.1 = _11 as f64;
RET = '\u{f17f2}';
_15 = (-322753_i32);
_13 = -_6;
(*_10) = !_12;
Goto(bb19)
}
bb19 = {
Call(_23 = dump_var(3_usize, 4_usize, Move(_4), 15_usize, Move(_15), 16_usize, Move(_16), 1_usize, Move(_1)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_23 = dump_var(3_usize, 11_usize, Move(_11), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: u32,mut _2: isize,mut _3: isize,mut _4: u32,mut _5: isize,mut _6: f32,mut _7: u32,mut _8: bool,mut _9: u32) -> char {
mir! {
type RET = char;
let _10: (f32,);
let _11: usize;
let _12: (f32,);
let _13: u32;
let _14: bool;
let _15: usize;
let _16: (f64, f64, i64, bool);
let _17: ([u8; 4], char, [isize; 3]);
let _18: isize;
let _19: &'static i16;
let _20: u8;
let _21: f64;
let _22: [char; 6];
let _23: *const *const *mut u128;
let _24: (f64, f64, i64, bool);
let _25: u32;
let _26: [char; 8];
let _27: bool;
let _28: isize;
let _29: &'static *const &'static i8;
let _30: Adt24;
let _31: [u64; 5];
let _32: f32;
let _33: u64;
let _34: Adt24;
let _35: isize;
let _36: u32;
let _37: u64;
let _38: bool;
let _39: u32;
let _40: isize;
let _41: f32;
let _42: isize;
let _43: Adt32;
let _44: ();
let _45: ();
{
_5 = 14524451078389467915_u64 as isize;
RET = '\u{d1ef4}';
_5 = _2 >> _9;
RET = '\u{190ab}';
_1 = _4;
_5 = -_2;
_2 = _5 * _3;
_10.0 = -_6;
RET = '\u{975a7}';
_2 = !_3;
_6 = _10.0 + _10.0;
_10 = (_6,);
_8 = true;
_3 = _5;
_9 = _1 >> _2;
_3 = 6_usize as isize;
_11 = 0_usize;
_9 = !_1;
_9 = !_7;
RET = '\u{a3b35}';
RET = '\u{893ff}';
_6 = 68_i8 as f32;
Goto(bb1)
}
bb1 = {
_12.0 = (-5022_i16) as f32;
_8 = _11 != _11;
_9 = !_1;
_9 = _7 + _7;
_4 = RET as u32;
_12 = (_6,);
_7 = _1;
_2 = !_3;
_11 = 40806_u16 as usize;
_10.0 = -_12.0;
_13 = _7 | _1;
_2 = _5 ^ _5;
_12.0 = _10.0 - _10.0;
_5 = _2 & _3;
_4 = 114569578986573973554527303648642126849_i128 as u32;
_12 = _10;
_10 = (_12.0,);
RET = '\u{ca18a}';
_1 = _13 - _9;
_1 = _4;
_16.3 = !_8;
_16.1 = 5745885076316854423_u64 as f64;
RET = '\u{a9f5c}';
_16.0 = 472261069379607362_i64 as f64;
_6 = 11543_i16 as f32;
_16.3 = !_8;
Goto(bb2)
}
bb2 = {
_16.2 = (-7730997609841727384_i64) * (-1871427701942846365_i64);
_17.1 = RET;
_16.2 = 3198461705874987787_i64 + 1119050242586628256_i64;
RET = _17.1;
_18 = _16.0 as isize;
Call(_17.1 = fn5(_5, _5, _2, _16.1, _13, _5, _6, _13, _2, _2, _9, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_15 = !_11;
RET = _17.1;
_12.0 = -_6;
_15 = !_11;
_16.1 = _16.0 + _16.0;
_10 = _12;
_11 = !_15;
_17.1 = RET;
_17.2 = [_5,_2,_5];
_14 = !_8;
_10 = _12;
RET = _17.1;
_9 = _13;
_16.1 = _16.0;
_3 = _16.2 as isize;
_17.1 = RET;
_18 = _2 + _5;
Goto(bb4)
}
bb4 = {
_20 = 166_u8;
RET = _17.1;
_4 = _9;
_16.1 = _18 as f64;
_21 = -_16.1;
_17.0 = [_20,_20,_20,_20];
RET = _17.1;
_17.2 = [_18,_18,_3];
_6 = _10.0;
_16 = (_21, _21, (-301820992027193273_i64), _14);
_24 = (_16.1, _16.1, _16.2, _8);
_18 = _24.2 as isize;
_24.1 = _21;
Goto(bb5)
}
bb5 = {
_14 = !_16.3;
_4 = _1 >> _18;
_24.0 = _16.1 * _21;
_24.2 = _16.2;
RET = _17.1;
_16 = _24;
_1 = !_4;
_17.1 = RET;
_25 = _16.0 as u32;
match _16.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
340282366920938463463072786439741018183 => bb8,
_ => bb7
}
}
bb6 = {
_20 = 166_u8;
RET = _17.1;
_4 = _9;
_16.1 = _18 as f64;
_21 = -_16.1;
_17.0 = [_20,_20,_20,_20];
RET = _17.1;
_17.2 = [_18,_18,_3];
_6 = _10.0;
_16 = (_21, _21, (-301820992027193273_i64), _14);
_24 = (_16.1, _16.1, _16.2, _8);
_18 = _24.2 as isize;
_24.1 = _21;
Goto(bb5)
}
bb7 = {
_16.2 = (-7730997609841727384_i64) * (-1871427701942846365_i64);
_17.1 = RET;
_16.2 = 3198461705874987787_i64 + 1119050242586628256_i64;
RET = _17.1;
_18 = _16.0 as isize;
Call(_17.1 = fn5(_5, _5, _2, _16.1, _13, _5, _6, _13, _2, _2, _9, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb8 = {
_8 = _14;
Goto(bb9)
}
bb9 = {
_24.3 = _24.1 > _16.1;
_24.0 = _24.1 - _16.0;
_16.2 = _24.2;
_16 = (_24.0, _24.0, _24.2, _24.3);
_24.1 = _16.1;
_27 = !_14;
_22 = [_17.1,_17.1,_17.1,_17.1,_17.1,_17.1];
_14 = _24.3;
_24.3 = !_16.3;
_24.3 = _16.3 | _16.3;
Call(_24.2 = core::intrinsics::transmute(_16.2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_16.1 = _24.1 * _24.0;
_10.0 = _6;
_3 = !_5;
_16 = (_24.1, _24.1, _24.2, _24.3);
_13 = 81899135646679766366671701963749614760_i128 as u32;
_28 = (-120030021071701713729641818659692649882_i128) as isize;
_8 = _16.3;
_12.0 = -_6;
_10 = _12;
_11 = !_15;
_10.0 = _6 * _6;
_24 = (_16.0, _16.1, _16.2, _16.3);
_20 = !218_u8;
_16.0 = 143447714165324495061758815321563948840_i128 as f64;
_17.1 = RET;
_21 = _24.1 * _16.1;
_4 = 116_i8 as u32;
_14 = _16.3 == _16.3;
_10 = _12;
_30.fld3 = _20 as u64;
_1 = _9;
RET = _17.1;
_27 = !_16.3;
_30.fld3 = 6324712526909474801_u64 & 13792681477204047323_u64;
Goto(bb11)
}
bb11 = {
_34.fld3 = !_30.fld3;
_28 = _18 | _18;
_24.2 = !_16.2;
_14 = _27;
_30.fld4 = !19443_i16;
_17.0 = [_20,_20,_20,_20];
_30.fld2 = [_17.1,RET,RET,_17.1,RET,_17.1];
_36 = _25;
_16.2 = _24.2;
_34.fld5 = _30.fld3 as u32;
_17.1 = RET;
_24.0 = _24.1;
_24.1 = _24.2 as f64;
RET = _17.1;
_17.1 = RET;
_28 = -_18;
_2 = _18;
_27 = _21 != _24.1;
Goto(bb12)
}
bb12 = {
_37 = _30.fld3 - _30.fld3;
_40 = _18 & _2;
_34.fld2 = [RET,RET,RET,RET,RET,_17.1];
_30.fld4 = _14 as i16;
_17.2 = [_18,_5,_40];
Goto(bb13)
}
bb13 = {
_34.fld2 = _30.fld2;
_10 = (_12.0,);
_15 = _16.2 as usize;
_7 = _36;
_26 = [_17.1,RET,RET,_17.1,_17.1,RET,_17.1,RET];
_17.0 = [_20,_20,_20,_20];
_38 = _16.3 >= _27;
_11 = !_15;
_34.fld1 = [_8,_38,_8];
_32 = _12.0;
RET = _17.1;
_5 = !_3;
_34.fld0 = !337486985768491578252014997040558328018_u128;
_32 = _6;
_34.fld4 = -_30.fld4;
_31 = [_37,_30.fld3,_37,_34.fld3,_34.fld3];
_31 = [_37,_30.fld3,_37,_37,_37];
_24.3 = !_27;
_30.fld4 = _24.2 as i16;
_4 = (-1701157109_i32) as u32;
Goto(bb14)
}
bb14 = {
_34.fld2 = [_17.1,_17.1,RET,_17.1,_17.1,RET];
_3 = _40;
_21 = 35611_u16 as f64;
_38 = _27 > _27;
_19 = &_34.fld4;
_12 = _10;
_8 = _7 == _36;
_20 = !96_u8;
_30.fld3 = _37 ^ _37;
_38 = !_16.3;
_5 = _3 & _2;
_31 = [_30.fld3,_30.fld3,_30.fld3,_37,_34.fld3];
_34.fld3 = _37 & _30.fld3;
_24 = _16;
_28 = _5;
_37 = _34.fld3 ^ _34.fld3;
_34.fld0 = 16125042286518642834887023929280206681_u128;
_30.fld0 = _34.fld0 + _34.fld0;
_35 = _2;
_24.1 = _15 as f64;
_27 = _16.3;
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(4_usize, 9_usize, Move(_9), 11_usize, Move(_11), 14_usize, Move(_14), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(4_usize, 36_usize, Move(_36), 5_usize, Move(_5), 4_usize, Move(_4), 2_usize, Move(_2)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(4_usize, 15_usize, Move(_15), 13_usize, Move(_13), 38_usize, Move(_38), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(4_usize, 26_usize, Move(_26), 45_usize, _45, 45_usize, _45, 45_usize, _45), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: f64,mut _5: u32,mut _6: isize,mut _7: f32,mut _8: u32,mut _9: isize,mut _10: isize,mut _11: u32,mut _12: isize) -> char {
mir! {
type RET = char;
let _13: [u64; 3];
let _14: isize;
let _15: i64;
let _16: u16;
let _17: Adt69;
let _18: *const u32;
let _19: u16;
let _20: u16;
let _21: [usize; 8];
let _22: (*const *mut u128,);
let _23: u128;
let _24: i16;
let _25: isize;
let _26: &'static *const i64;
let _27: char;
let _28: char;
let _29: *const &'static i8;
let _30: &'static i8;
let _31: isize;
let _32: u16;
let _33: ();
let _34: ();
{
_9 = false as isize;
_12 = 13_i8 as isize;
_7 = _2 as f32;
_11 = 118_i8 as u32;
RET = '\u{b8d3b}';
_15 = 4215642208234642018_i64;
_9 = _1 * _1;
_11 = _5;
_13 = [6511000974814862843_u64,9578670962949398439_u64,17859777864331128715_u64];
RET = '\u{5246}';
_4 = (-18549111220393581127031507093189771659_i128) as f64;
_5 = !_11;
_13 = [5671997355942368308_u64,6437926209640162061_u64,14812314050759257060_u64];
RET = '\u{3fd70}';
_12 = 15536710185884214136_usize as isize;
match _15 {
0 => bb1,
4215642208234642018 => bb3,
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
_8 = !_5;
_16 = 38090_u16 >> _3;
_14 = 1522762684248072115_usize as isize;
_1 = _10;
RET = '\u{9f115}';
_9 = _1;
_8 = (-30466_i16) as u32;
_2 = -_1;
_2 = _5 as isize;
Goto(bb4)
}
bb4 = {
_9 = _12;
_2 = !_10;
_10 = (-405021227_i32) as isize;
_16 = 28635_u16;
_8 = _11;
_17 = Adt69::Variant2 { fld0: true,fld1: RET,fld2: _13,fld3: 4_usize,fld4: _5 };
place!(Field::<usize>(Variant(_17, 2), 3)) = !1_usize;
_13 = Field::<[u64; 3]>(Variant(_17, 2), 2);
_13 = Field::<[u64; 3]>(Variant(_17, 2), 2);
_14 = 1951190355_i32 as isize;
RET = Field::<char>(Variant(_17, 2), 1);
match _15 {
4215642208234642018 => bb6,
_ => bb5
}
}
bb5 = {
Return()
}
bb6 = {
place!(Field::<bool>(Variant(_17, 2), 0)) = true & true;
_3 = _10;
_18 = core::ptr::addr_of!(_5);
(*_18) = Field::<char>(Variant(_17, 2), 1) as u32;
(*_18) = _11;
_19 = _7 as u16;
place!(Field::<usize>(Variant(_17, 2), 3)) = Field::<bool>(Variant(_17, 2), 0) as usize;
Call(RET = fn6(_1, _6, Move(_17), _10, _13), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
RET = '\u{10081a}';
_1 = _2 ^ _12;
_5 = _8 + _11;
_16 = !_19;
_19 = _16;
_4 = _16 as f64;
_3 = _6 >> _2;
_20 = 2412057056753755654_u64 as u16;
(*_18) = !_8;
_3 = _4 as isize;
_2 = !_6;
_9 = -_1;
_8 = _11;
_8 = (*_18) & (*_18);
(*_18) = !_8;
Goto(bb8)
}
bb8 = {
RET = '\u{1b6e8}';
RET = '\u{8fef4}';
_12 = 827120674_i32 as isize;
(*_18) = 776932268_i32 as u32;
_1 = _4 as isize;
_21 = [1_usize,3_usize,3_usize,0_usize,4_usize,379462138309250866_usize,6_usize,2948961380518067662_usize];
_7 = 36_u8 as f32;
_6 = _3 | _14;
_21 = [5_usize,7_usize,14871185645923324917_usize,11139914558128414951_usize,8177284533843478393_usize,7733481703052756789_usize,7_usize,8046418254658778240_usize];
_20 = _19;
_7 = 134_u8 as f32;
_18 = core::ptr::addr_of!((*_18));
_23 = 177384377845906266887903378677906167898_u128;
_11 = !(*_18);
_8 = _11;
(*_18) = _11;
_24 = (-18135_i16) >> _14;
_18 = core::ptr::addr_of!((*_18));
_19 = _16 >> _2;
_17 = Adt69::Variant2 { fld0: false,fld1: RET,fld2: _13,fld3: 0_usize,fld4: (*_18) };
place!(Field::<bool>(Variant(_17, 2), 0)) = true;
_16 = !_20;
RET = Field::<char>(Variant(_17, 2), 1);
_19 = 554730949_i32 as u16;
match _23 {
0 => bb9,
177384377845906266887903378677906167898 => bb11,
_ => bb10
}
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_9 = -_6;
_27 = RET;
RET = Field::<char>(Variant(_17, 2), 1);
RET = Field::<char>(Variant(_17, 2), 1);
place!(Field::<[u64; 3]>(Variant(_17, 2), 2)) = [29430502631550464_u64,15433291063664424342_u64,10802875807138314713_u64];
_2 = (-51_i8) as isize;
(*_18) = !_11;
_19 = !_20;
RET = Field::<char>(Variant(_17, 2), 1);
_1 = _6;
match _23 {
0 => bb7,
1 => bb12,
177384377845906266887903378677906167898 => bb14,
_ => bb13
}
}
bb12 = {
Return()
}
bb13 = {
_9 = _12;
_2 = !_10;
_10 = (-405021227_i32) as isize;
_16 = 28635_u16;
_8 = _11;
_17 = Adt69::Variant2 { fld0: true,fld1: RET,fld2: _13,fld3: 4_usize,fld4: _5 };
place!(Field::<usize>(Variant(_17, 2), 3)) = !1_usize;
_13 = Field::<[u64; 3]>(Variant(_17, 2), 2);
_13 = Field::<[u64; 3]>(Variant(_17, 2), 2);
_14 = 1951190355_i32 as isize;
RET = Field::<char>(Variant(_17, 2), 1);
match _15 {
4215642208234642018 => bb6,
_ => bb5
}
}
bb14 = {
_12 = _9;
_11 = (*_18);
_8 = !Field::<u32>(Variant(_17, 2), 4);
place!(Field::<[u64; 3]>(Variant(_17, 2), 2)) = _13;
_6 = _9;
_19 = _27 as u16;
_8 = _5 & _11;
place!(Field::<usize>(Variant(_17, 2), 3)) = 12738243773721261276_u64 as usize;
(*_18) = _8;
_4 = _23 as f64;
_13 = [17466588224199287291_u64,14673775956565881006_u64,16461510031457807115_u64];
RET = _27;
_25 = Field::<usize>(Variant(_17, 2), 3) as isize;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(5_usize, 10_usize, Move(_10), 21_usize, Move(_21), 9_usize, Move(_9), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(5_usize, 19_usize, Move(_19), 5_usize, Move(_5), 24_usize, Move(_24), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(5_usize, 11_usize, Move(_11), 8_usize, Move(_8), 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: isize,mut _2: isize,mut _3: Adt69,mut _4: isize,mut _5: [u64; 3]) -> char {
mir! {
type RET = char;
let _6: bool;
let _7: i64;
let _8: *mut u128;
let _9: &'static *const &'static i8;
let _10: isize;
let _11: bool;
let _12: i128;
let _13: (f32,);
let _14: char;
let _15: (*const *mut u128,);
let _16: [i16; 1];
let _17: char;
let _18: *mut *const i64;
let _19: [char; 6];
let _20: [i16; 1];
let _21: *const i64;
let _22: Adt40;
let _23: isize;
let _24: bool;
let _25: &'static [isize; 3];
let _26: i8;
let _27: f32;
let _28: u32;
let _29: f64;
let _30: char;
let _31: isize;
let _32: char;
let _33: i16;
let _34: isize;
let _35: char;
let _36: char;
let _37: (*mut *const i64, i128, *const u128, ([u16; 5], *const i64));
let _38: Adt65;
let _39: [isize; 3];
let _40: f64;
let _41: i8;
let _42: isize;
let _43: *mut usize;
let _44: [char; 6];
let _45: f64;
let _46: bool;
let _47: &'static *const i64;
let _48: char;
let _49: f32;
let _50: ();
let _51: ();
{
SetDiscriminant(_3, 3);
place!(Field::<Adt63>(Variant(_3, 3), 3)).fld3 = 17625524820348163619_u64 * 5274250498175487259_u64;
RET = '\u{11237}';
place!(Field::<Adt63>(Variant(_3, 3), 3)).fld4 = [48674_u16,25492_u16,43758_u16,14597_u16,20371_u16];
_5 = [Field::<Adt63>(Variant(_3, 3), 3).fld3,Field::<Adt63>(Variant(_3, 3), 3).fld3,Field::<Adt63>(Variant(_3, 3), 3).fld3];
place!(Field::<[isize; 3]>(Variant(_3, 3), 0)) = [_2,_4,_1];
place!(Field::<[i128; 5]>(Variant(_3, 3), 2)) = [(-120300872969910456945229066757293184116_i128),44125105491263009665328565995995855296_i128,(-723781688197148372377646551763719395_i128),93717047822565776289689008084098466865_i128,49792123599701259332048156435465746149_i128];
RET = '\u{4179d}';
_6 = false & true;
place!(Field::<f32>(Variant(_3, 3), 1)) = 29554_u16 as f32;
place!(Field::<f32>(Variant(_3, 3), 1)) = 2052245219_i32 as f32;
place!(Field::<Adt63>(Variant(_3, 3), 3)).fld3 = !14677225687581347612_u64;
Goto(bb1)
}
bb1 = {
place!(Field::<Adt63>(Variant(_3, 3), 3)).fld0.0 = 42360_u16 >> _1;
_5 = [Field::<Adt63>(Variant(_3, 3), 3).fld3,Field::<Adt63>(Variant(_3, 3), 3).fld3,Field::<Adt63>(Variant(_3, 3), 3).fld3];
place!(Field::<Adt63>(Variant(_3, 3), 3)).fld4 = [Field::<Adt63>(Variant(_3, 3), 3).fld0.0,Field::<Adt63>(Variant(_3, 3), 3).fld0.0,Field::<Adt63>(Variant(_3, 3), 3).fld0.0,Field::<Adt63>(Variant(_3, 3), 3).fld0.0,Field::<Adt63>(Variant(_3, 3), 3).fld0.0];
Goto(bb2)
}
bb2 = {
_7 = -(-4333615154601695764_i64);
RET = '\u{a904d}';
_3 = Adt69::Variant2 { fld0: _6,fld1: RET,fld2: _5,fld3: 7_usize,fld4: 3880823252_u32 };
place!(Field::<u32>(Variant(_3, 2), 4)) = (-2075198150_i32) as u32;
_10 = _2 - _1;
place!(Field::<usize>(Variant(_3, 2), 3)) = 5_usize;
place!(Field::<u32>(Variant(_3, 2), 4)) = 1059025740_u32;
_16 = [10046_i16];
Goto(bb3)
}
bb3 = {
_11 = Field::<bool>(Variant(_3, 2), 0) & Field::<bool>(Variant(_3, 2), 0);
_15.0 = core::ptr::addr_of!(_8);
_7 = (-1517718536_i32) as i64;
place!(Field::<usize>(Variant(_3, 2), 3)) = RET as usize;
place!(Field::<char>(Variant(_3, 2), 1)) = RET;
_4 = _10;
_10 = !_2;
_13.0 = (-91_i8) as f32;
_12 = 31652_u16 as i128;
_6 = _11 & Field::<bool>(Variant(_3, 2), 0);
place!(Field::<[u64; 3]>(Variant(_3, 2), 2)) = _5;
_16 = [8524_i16];
_5 = [9453856694601682302_u64,16919826325667328400_u64,17887569969460994195_u64];
RET = Field::<char>(Variant(_3, 2), 1);
RET = Field::<char>(Variant(_3, 2), 1);
Goto(bb4)
}
bb4 = {
_2 = _4 ^ _4;
_10 = !_1;
_7 = (-7982751216357406426_i64) * 4312945434609406147_i64;
_19 = [Field::<char>(Variant(_3, 2), 1),Field::<char>(Variant(_3, 2), 1),Field::<char>(Variant(_3, 2), 1),Field::<char>(Variant(_3, 2), 1),RET,Field::<char>(Variant(_3, 2), 1)];
_2 = -_4;
RET = Field::<char>(Variant(_3, 2), 1);
place!(Field::<char>(Variant(_3, 2), 1)) = RET;
_16 = [17171_i16];
place!(Field::<bool>(Variant(_3, 2), 0)) = !_11;
place!(Field::<usize>(Variant(_3, 2), 3)) = !3_usize;
_10 = _4;
_14 = Field::<char>(Variant(_3, 2), 1);
_18 = core::ptr::addr_of_mut!(_21);
(*_18) = core::ptr::addr_of!(_7);
_1 = !_2;
(*_21) = -(-1695826684511853287_i64);
place!(Field::<[u64; 3]>(Variant(_3, 2), 2)) = [12057746815668322850_u64,2952280723707176235_u64,16715667686757059912_u64];
_21 = core::ptr::addr_of!((*_21));
(*_18) = core::ptr::addr_of!((*_21));
SetDiscriminant(_3, 2);
_14 = RET;
_19 = [_14,_14,_14,_14,_14,_14];
_17 = RET;
Goto(bb5)
}
bb5 = {
_15.0 = core::ptr::addr_of!(_8);
_16 = [(-20830_i16)];
place!(Field::<[u64; 3]>(Variant(_3, 2), 2)) = [14842889972282771858_u64,6560831273602417940_u64,10505272888909901232_u64];
place!(Field::<bool>(Variant(_3, 2), 0)) = _11;
_1 = 21_i8 as isize;
_5 = [2951326887480436848_u64,14092031703924754727_u64,5175874804411959404_u64];
_20 = _16;
_10 = _2;
_16 = [(-12019_i16)];
(*_18) = core::ptr::addr_of!((*_21));
place!(Field::<u32>(Variant(_3, 2), 4)) = !3197192836_u32;
_22.fld0 = core::ptr::addr_of!(place!(Field::<char>(Variant(_3, 2), 1)));
_22.fld1 = (-454946809_i32) as u64;
place!(Field::<u32>(Variant(_3, 2), 4)) = !2016487661_u32;
place!(Field::<char>(Variant(_3, 2), 1)) = RET;
_5 = [_22.fld1,_22.fld1,_22.fld1];
_21 = core::ptr::addr_of!((*_21));
(*_18) = core::ptr::addr_of!((*_21));
_16 = _20;
_16 = [(-16867_i16)];
_3 = Adt69::Variant2 { fld0: _6,fld1: _17,fld2: _5,fld3: 2439429347391151270_usize,fld4: 328417864_u32 };
_23 = -_10;
_23 = _2;
Call((*_18) = fn7(_10, _10, _4, _14, _5), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_17 = RET;
_13.0 = _12 as f32;
_23 = _14 as isize;
_11 = _6 >= Field::<bool>(Variant(_3, 2), 0);
_2 = _4 * _4;
_6 = !_11;
_23 = _10;
RET = _14;
Goto(bb7)
}
bb7 = {
_11 = !_6;
_22.fld1 = !3635242938076066518_u64;
_27 = -_13.0;
_18 = core::ptr::addr_of_mut!((*_18));
_15.0 = core::ptr::addr_of!(_8);
_11 = Field::<bool>(Variant(_3, 2), 0);
RET = _14;
_4 = _2 >> _23;
_11 = _4 < _4;
_24 = _11 == _6;
_17 = RET;
_29 = _22.fld1 as f64;
_24 = !_11;
_1 = _2;
(*_18) = core::ptr::addr_of!(_7);
_4 = _23;
_16 = [27234_i16];
(*_18) = core::ptr::addr_of!((*_21));
Goto(bb8)
}
bb8 = {
_18 = core::ptr::addr_of_mut!(_21);
_13 = (_27,);
Goto(bb9)
}
bb9 = {
(*_21) = 6647128968323183277_i64 << _23;
_32 = _17;
(*_21) = !(-6296363013082348325_i64);
_20 = [(-1755_i16)];
RET = _14;
Goto(bb10)
}
bb10 = {
_30 = RET;
_29 = _22.fld1 as f64;
_13.0 = _27 + _27;
_6 = !_11;
_10 = _4;
_15.0 = core::ptr::addr_of!(_8);
place!(Field::<usize>(Variant(_3, 2), 3)) = (-664931144_i32) as usize;
_11 = !_6;
(*_18) = core::ptr::addr_of!(_7);
_28 = _13.0 as u32;
_2 = !_4;
_7 = -(-4744080341341510291_i64);
place!(Field::<bool>(Variant(_3, 2), 0)) = !_24;
_10 = _4 * _23;
_11 = _1 > _1;
_31 = Field::<usize>(Variant(_3, 2), 3) as isize;
_5 = [_22.fld1,_22.fld1,_22.fld1];
_16 = [(-18426_i16)];
_3 = Adt69::Variant2 { fld0: _11,fld1: _14,fld2: _5,fld3: 7_usize,fld4: _28 };
Goto(bb11)
}
bb11 = {
_26 = !(-110_i8);
place!(Field::<char>(Variant(_3, 2), 1)) = RET;
_31 = 52547_u16 as isize;
_10 = _1 << _23;
_13.0 = _27;
_21 = core::ptr::addr_of!((*_21));
_24 = _6 | Field::<bool>(Variant(_3, 2), 0);
_33 = !(-23741_i16);
(*_18) = core::ptr::addr_of!(_7);
_24 = !Field::<bool>(Variant(_3, 2), 0);
_15.0 = core::ptr::addr_of!(_8);
RET = _32;
_30 = Field::<char>(Variant(_3, 2), 1);
_27 = _13.0;
_2 = 30_u8 as isize;
place!(Field::<usize>(Variant(_3, 2), 3)) = 1_usize ^ 6_usize;
_33 = Field::<usize>(Variant(_3, 2), 3) as i16;
_2 = _10;
_11 = _6;
_34 = -_1;
Goto(bb12)
}
bb12 = {
place!(Field::<bool>(Variant(_3, 2), 0)) = !_24;
_32 = _17;
_37.3.0 = [21660_u16,44936_u16,39232_u16,46595_u16,31210_u16];
RET = _32;
_23 = _2;
_14 = _30;
_19 = [Field::<char>(Variant(_3, 2), 1),Field::<char>(Variant(_3, 2), 1),_14,_30,_32,_30];
_35 = RET;
_12 = (-98782805128986315219709649336296058702_i128) | 135590250289359461646024069202515821820_i128;
_39 = [_23,_4,_23];
_23 = Field::<u32>(Variant(_3, 2), 4) as isize;
_10 = _4;
_37.1 = _12;
_22.fld0 = core::ptr::addr_of!(_35);
_22.fld0 = core::ptr::addr_of!(_36);
_23 = _28 as isize;
_34 = !_2;
_7 = 22_u8 as i64;
_40 = -_29;
_28 = 1617626751_i32 as u32;
_34 = (-1191590546_i32) as isize;
Call(_33 = fn18(Move(_3), _10, _5, _24, _30, _1, _11), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
(*_21) = !6660352078819972902_i64;
(*_18) = core::ptr::addr_of!(_7);
_41 = _7 as i8;
_37.3.0 = [55480_u16,19435_u16,495_u16,48070_u16,18493_u16];
_37.3.0 = [43767_u16,51785_u16,16977_u16,38091_u16,11634_u16];
_22.fld0 = core::ptr::addr_of!(_17);
_40 = -_29;
_36 = _14;
_35 = _14;
_37.3.1 = Move(_21);
_15.0 = core::ptr::addr_of!(_8);
(*_18) = core::ptr::addr_of!(_7);
_37.3.0 = [52074_u16,26463_u16,44396_u16,2221_u16,695_u16];
(*_18) = Move(_37.3.1);
_34 = _1;
_23 = _10 | _10;
_16 = [_33];
_1 = !_4;
_11 = !_6;
_41 = _26;
(*_18) = core::ptr::addr_of!(_7);
_40 = 38620_u16 as f64;
(*_18) = core::ptr::addr_of!((*_21));
Goto(bb14)
}
bb14 = {
(*_21) = _28 as i64;
_45 = _40;
_39 = [_2,_10,_34];
_7 = -(-5355780339038291489_i64);
_37.0 = core::ptr::addr_of_mut!((*_18));
_7 = 1925787416030176408_i64;
_37.0 = Move(_18);
_2 = _23;
_5 = [_22.fld1,_22.fld1,_22.fld1];
_22.fld1 = 11944424665783136379_u64;
(*_21) = _22.fld1 as i64;
Goto(bb15)
}
bb15 = {
Call(_50 = dump_var(6_usize, 2_usize, Move(_2), 14_usize, Move(_14), 1_usize, Move(_1), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_50 = dump_var(6_usize, 33_usize, Move(_33), 12_usize, Move(_12), 30_usize, Move(_30), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_50 = dump_var(6_usize, 24_usize, Move(_24), 26_usize, Move(_26), 35_usize, Move(_35), 10_usize, Move(_10)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_50 = dump_var(6_usize, 17_usize, Move(_17), 51_usize, _51, 51_usize, _51, 51_usize, _51), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: char,mut _5: [u64; 3]) -> *const i64 {
mir! {
type RET = *const i64;
let _6: [char; 6];
let _7: u16;
let _8: [u64; 5];
let _9: u32;
let _10: isize;
let _11: isize;
let _12: f32;
let _13: i16;
let _14: (usize, [char; 6], *const u32, u16);
let _15: isize;
let _16: i16;
let _17: isize;
let _18: isize;
let _19: i128;
let _20: bool;
let _21: ((f64, f64, i64, bool), &'static *const i64);
let _22: [i128; 5];
let _23: i128;
let _24: *const [u16; 5];
let _25: f64;
let _26: char;
let _27: &'static (u16,);
let _28: (f32,);
let _29: ();
let _30: ();
{
_1 = _2 & _3;
_1 = 37310963596850021595526859528260607443_u128 as isize;
_4 = '\u{8295}';
_1 = _3;
Goto(bb1)
}
bb1 = {
_6 = [_4,_4,_4,_4,_4,_4];
Goto(bb2)
}
bb2 = {
_1 = _2;
_4 = '\u{73b15}';
_3 = _1 << _2;
_5 = [7999586818201104536_u64,3711841340506805672_u64,16656524152478800514_u64];
_4 = '\u{dd278}';
_1 = _2;
_7 = 11678_u16 + 61287_u16;
_3 = _1;
_7 = !22022_u16;
_6 = [_4,_4,_4,_4,_4,_4];
Call(_5 = fn8(_3, _1, _1, _2, _7, _1, _3, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = [_4,_4,_4,_4,_4,_4];
_6 = [_4,_4,_4,_4,_4,_4];
_6 = [_4,_4,_4,_4,_4,_4];
_4 = '\u{7f79d}';
_7 = !55613_u16;
_7 = !60732_u16;
_6 = [_4,_4,_4,_4,_4,_4];
_3 = _1;
_7 = 28695_u16;
_7 = !26632_u16;
_6 = [_4,_4,_4,_4,_4,_4];
_6 = [_4,_4,_4,_4,_4,_4];
_10 = -_2;
_1 = _2 - _2;
_8 = [10357481249130056489_u64,10633182624964967231_u64,2673824627640239135_u64,6160225208143663368_u64,2179280025186542247_u64];
_5 = [15697942908750124767_u64,4108653720476894475_u64,8284357359339939764_u64];
_5 = [9959886158677251529_u64,18016258185003763851_u64,443080680355138214_u64];
_10 = _2;
_2 = _10 * _1;
Call(_8 = fn17(_2, _3, _10, _2, _2, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_9 = !3536150515_u32;
_10 = 11641009114575149307_u64 as isize;
_3 = _1 & _2;
_11 = _1 | _3;
_5 = [7937721273812622289_u64,8507628245711392061_u64,14953248743284528411_u64];
_14.0 = _7 as usize;
_14.0 = !15448478329907873633_usize;
_14.2 = core::ptr::addr_of!(_9);
_15 = 306819242798940728021040077506780088091_u128 as isize;
_14.3 = _7;
_13 = _14.3 as i16;
_16 = 11785466586361296291_u64 as i16;
_10 = _11 ^ _1;
_6 = [_4,_4,_4,_4,_4,_4];
_8 = [14835739184398951081_u64,13185555162477606918_u64,15604169048051759740_u64,11753536457955612486_u64,6337502426295526276_u64];
_14.1 = [_4,_4,_4,_4,_4,_4];
_10 = _9 as isize;
Goto(bb5)
}
bb5 = {
_14.2 = core::ptr::addr_of!(_9);
_18 = _1;
_17 = _18;
_14.0 = 14048639128740729094_usize * 7_usize;
_14.2 = core::ptr::addr_of!(_9);
_13 = !_16;
_15 = !_11;
_9 = 1838861550_u32;
_12 = 17708932834183754542_u64 as f32;
_14.3 = _7;
_15 = _11 | _1;
_18 = _15 << _11;
_1 = true as isize;
RET = core::ptr::addr_of!(_21.0.2);
_20 = !false;
_4 = '\u{7ce59}';
_22 = [(-90261465481416455974892783113495247380_i128),104605495609564772553671771484597382497_i128,(-135523821813092799377983265995773797105_i128),(-109166840064521731843375351484024684519_i128),10447103657332407919933686107510398960_i128];
_20 = _18 <= _15;
_19 = 136685005112864558944758271586998602502_i128;
_20 = !false;
_19 = 256288287431222221772258839484483277042_u128 as i128;
_14.3 = !_7;
_21.0.1 = _16 as f64;
RET = core::ptr::addr_of!((*RET));
(*RET) = 2676791286728280904_i64 << _15;
_6 = _14.1;
_4 = '\u{10a1b}';
_8 = [11573633298384407188_u64,8442330317510126707_u64,576952189808660573_u64,16375074322851343578_u64,11276423326534413510_u64];
match _9 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
1838861550 => bb12,
_ => bb11
}
}
bb6 = {
_9 = !3536150515_u32;
_10 = 11641009114575149307_u64 as isize;
_3 = _1 & _2;
_11 = _1 | _3;
_5 = [7937721273812622289_u64,8507628245711392061_u64,14953248743284528411_u64];
_14.0 = _7 as usize;
_14.0 = !15448478329907873633_usize;
_14.2 = core::ptr::addr_of!(_9);
_15 = 306819242798940728021040077506780088091_u128 as isize;
_14.3 = _7;
_13 = _14.3 as i16;
_16 = 11785466586361296291_u64 as i16;
_10 = _11 ^ _1;
_6 = [_4,_4,_4,_4,_4,_4];
_8 = [14835739184398951081_u64,13185555162477606918_u64,15604169048051759740_u64,11753536457955612486_u64,6337502426295526276_u64];
_14.1 = [_4,_4,_4,_4,_4,_4];
_10 = _9 as isize;
Goto(bb5)
}
bb7 = {
_6 = [_4,_4,_4,_4,_4,_4];
_6 = [_4,_4,_4,_4,_4,_4];
_6 = [_4,_4,_4,_4,_4,_4];
_4 = '\u{7f79d}';
_7 = !55613_u16;
_7 = !60732_u16;
_6 = [_4,_4,_4,_4,_4,_4];
_3 = _1;
_7 = 28695_u16;
_7 = !26632_u16;
_6 = [_4,_4,_4,_4,_4,_4];
_6 = [_4,_4,_4,_4,_4,_4];
_10 = -_2;
_1 = _2 - _2;
_8 = [10357481249130056489_u64,10633182624964967231_u64,2673824627640239135_u64,6160225208143663368_u64,2179280025186542247_u64];
_5 = [15697942908750124767_u64,4108653720476894475_u64,8284357359339939764_u64];
_5 = [9959886158677251529_u64,18016258185003763851_u64,443080680355138214_u64];
_10 = _2;
_2 = _10 * _1;
Call(_8 = fn17(_2, _3, _10, _2, _2, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_1 = _2;
_4 = '\u{73b15}';
_3 = _1 << _2;
_5 = [7999586818201104536_u64,3711841340506805672_u64,16656524152478800514_u64];
_4 = '\u{dd278}';
_1 = _2;
_7 = 11678_u16 + 61287_u16;
_3 = _1;
_7 = !22022_u16;
_6 = [_4,_4,_4,_4,_4,_4];
Call(_5 = fn8(_3, _1, _1, _2, _7, _1, _3, _2), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_6 = [_4,_4,_4,_4,_4,_4];
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_12 = _16 as f32;
_13 = _16 ^ _16;
_14.3 = _7 << _2;
_9 = !2537891847_u32;
_3 = _15;
_22 = [_19,_19,_19,_19,_19];
_16 = !_13;
_9 = 1119888847_u32;
_7 = _14.3;
_21.0.2 = _4 as i64;
_6 = [_4,_4,_4,_4,_4,_4];
match _9 {
0 => bb10,
1 => bb2,
2 => bb7,
3 => bb5,
4 => bb13,
5 => bb14,
1119888847 => bb16,
_ => bb15
}
}
bb13 = {
_14.2 = core::ptr::addr_of!(_9);
_18 = _1;
_17 = _18;
_14.0 = 14048639128740729094_usize * 7_usize;
_14.2 = core::ptr::addr_of!(_9);
_13 = !_16;
_15 = !_11;
_9 = 1838861550_u32;
_12 = 17708932834183754542_u64 as f32;
_14.3 = _7;
_15 = _11 | _1;
_18 = _15 << _11;
_1 = true as isize;
RET = core::ptr::addr_of!(_21.0.2);
_20 = !false;
_4 = '\u{7ce59}';
_22 = [(-90261465481416455974892783113495247380_i128),104605495609564772553671771484597382497_i128,(-135523821813092799377983265995773797105_i128),(-109166840064521731843375351484024684519_i128),10447103657332407919933686107510398960_i128];
_20 = _18 <= _15;
_19 = 136685005112864558944758271586998602502_i128;
_20 = !false;
_19 = 256288287431222221772258839484483277042_u128 as i128;
_14.3 = !_7;
_21.0.1 = _16 as f64;
RET = core::ptr::addr_of!((*RET));
(*RET) = 2676791286728280904_i64 << _15;
_6 = _14.1;
_4 = '\u{10a1b}';
_8 = [11573633298384407188_u64,8442330317510126707_u64,576952189808660573_u64,16375074322851343578_u64,11276423326534413510_u64];
match _9 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
1838861550 => bb12,
_ => bb11
}
}
bb14 = {
_6 = [_4,_4,_4,_4,_4,_4];
_6 = [_4,_4,_4,_4,_4,_4];
_6 = [_4,_4,_4,_4,_4,_4];
_4 = '\u{7f79d}';
_7 = !55613_u16;
_7 = !60732_u16;
_6 = [_4,_4,_4,_4,_4,_4];
_3 = _1;
_7 = 28695_u16;
_7 = !26632_u16;
_6 = [_4,_4,_4,_4,_4,_4];
_6 = [_4,_4,_4,_4,_4,_4];
_10 = -_2;
_1 = _2 - _2;
_8 = [10357481249130056489_u64,10633182624964967231_u64,2673824627640239135_u64,6160225208143663368_u64,2179280025186542247_u64];
_5 = [15697942908750124767_u64,4108653720476894475_u64,8284357359339939764_u64];
_5 = [9959886158677251529_u64,18016258185003763851_u64,443080680355138214_u64];
_10 = _2;
_2 = _10 * _1;
Call(_8 = fn17(_2, _3, _10, _2, _2, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb15 = {
_6 = [_4,_4,_4,_4,_4,_4];
Goto(bb2)
}
bb16 = {
_4 = '\u{72604}';
_14.2 = core::ptr::addr_of!(_9);
_9 = !744943965_u32;
_3 = _2 ^ _15;
_14.1 = _6;
RET = core::ptr::addr_of!((*RET));
_5 = [940395556835815463_u64,2354590605116134447_u64,3959703515244011299_u64];
_8 = [5088260595107463233_u64,2566278727681531278_u64,2393064362922252178_u64,15295256993622483698_u64,10210691656949252844_u64];
_21.0.0 = _21.0.1 + _21.0.1;
_12 = 9960118242925797869_u64 as f32;
_14.2 = core::ptr::addr_of!(_9);
_4 = '\u{4f979}';
_28.0 = _12 - _12;
_14.1 = [_4,_4,_4,_4,_4,_4];
(*RET) = -(-4272969513065777282_i64);
_7 = !_14.3;
_14.1 = _6;
_11 = _18 - _1;
Goto(bb17)
}
bb17 = {
Call(_29 = dump_var(7_usize, 11_usize, Move(_11), 10_usize, Move(_10), 13_usize, Move(_13), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(7_usize, 20_usize, Move(_20), 22_usize, Move(_22), 5_usize, Move(_5), 3_usize, Move(_3)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_29 = dump_var(7_usize, 2_usize, Move(_2), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: u16,mut _6: isize,mut _7: isize,mut _8: isize) -> [u64; 3] {
mir! {
type RET = [u64; 3];
let _9: [u64; 3];
let _10: char;
let _11: [bool; 3];
let _12: &'static &'static [u64; 5];
let _13: *const char;
let _14: &'static [usize; 8];
let _15: f32;
let _16: u16;
let _17: *const *const *mut u128;
let _18: (f32,);
let _19: (usize, [char; 6], *const u32, u16);
let _20: u16;
let _21: &'static isize;
let _22: [i128; 5];
let _23: isize;
let _24: Adt81;
let _25: [i16; 1];
let _26: Adt81;
let _27: [i128; 5];
let _28: *const i32;
let _29: &'static [u64; 5];
let _30: bool;
let _31: (*mut u128,);
let _32: [u64; 3];
let _33: char;
let _34: *const Adt32;
let _35: Adt82;
let _36: [usize; 8];
let _37: f32;
let _38: isize;
let _39: bool;
let _40: [u64; 3];
let _41: &'static [usize; 8];
let _42: [u16; 5];
let _43: (u64, u16);
let _44: char;
let _45: [i16; 1];
let _46: bool;
let _47: isize;
let _48: ();
let _49: ();
{
_4 = !_6;
_6 = -_2;
_7 = !_6;
_6 = !_2;
RET = [2369957542901788293_u64,13458749604661680837_u64,8581149862585013683_u64];
_4 = !_3;
RET = [9375120302492137616_u64,13699337591147223342_u64,2241795267474456499_u64];
_8 = -_3;
RET = [799109533102225909_u64,608505320338829573_u64,6145854705325581003_u64];
_1 = _3 + _8;
RET = [3066169297219466466_u64,12472266129111128887_u64,8733763138447881314_u64];
_10 = '\u{187d}';
_8 = !_4;
RET = [9609993236497150365_u64,15702272724029254181_u64,13461201325373532634_u64];
RET = [6747575736251694816_u64,10198769798574711510_u64,7247371137585386099_u64];
_3 = 217519639343563115062900470547506459966_u128 as isize;
Call(RET = fn9(_4, _1, _1, _1, _2, _8, _2, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = 191589201616429072526316536686096428820_u128 as isize;
_11 = [true,false,true];
_1 = !_6;
RET = [350890720748744145_u64,17190369908524567548_u64,450980922267798273_u64];
_6 = (-73_i8) as isize;
_6 = _8 | _4;
_6 = _4 * _8;
_9 = [10336422758841644005_u64,243636439381847514_u64,15628376520867597635_u64];
_5 = (-116547243053275860769325442535722551680_i128) as u16;
_11 = [false,true,false];
Call(_8 = fn13(_1, _2, _6, _4, _6, _5, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = [8688868342147569991_u64,12030755659120992356_u64,11924173025361824824_u64];
_2 = _10 as isize;
_4 = _8 | _8;
_1 = 20_u8 as isize;
_13 = core::ptr::addr_of!(_10);
Goto(bb3)
}
bb3 = {
_3 = _5 as isize;
_2 = _4;
(*_13) = '\u{84673}';
_6 = 97_u8 as isize;
RET = [1343048789557010171_u64,18186895977983609156_u64,6682640376084463516_u64];
_3 = -_4;
_3 = _2;
_10 = '\u{1094c2}';
_9 = RET;
(*_13) = '\u{6abf}';
_7 = -_3;
_11 = [true,true,false];
_8 = _4;
_5 = 51484_u16;
(*_13) = '\u{15210}';
_8 = _2;
RET = _9;
_11 = [true,true,false];
_2 = (*_13) as isize;
_16 = _5 ^ _5;
_13 = core::ptr::addr_of!(_10);
_8 = _5 as isize;
_6 = _3;
_18.0 = 1720516493263034466_u64 as f32;
Goto(bb4)
}
bb4 = {
_8 = _6 + _3;
RET = [13262493707603176322_u64,15244865267828451720_u64,16229938192742670441_u64];
_1 = 1986429261032329283_i64 as isize;
_20 = _16 / _5;
_20 = _5;
RET = _9;
_5 = !_16;
_19.1 = [(*_13),(*_13),(*_13),(*_13),_10,_10];
_19.3 = _5 / _20;
_3 = 7725134737767430138_usize as isize;
_4 = _6 * _8;
Goto(bb5)
}
bb5 = {
_4 = -_6;
_11 = [true,false,false];
_21 = &_1;
_22 = [(-142321641983921372677369525532855424708_i128),6852376250575576359616292147204762324_i128,24930527825024345428478846294082240610_i128,132518969134936499069909808252133407523_i128,126008127522497494421758360863023844986_i128];
_20 = _19.3;
Goto(bb6)
}
bb6 = {
_5 = _16;
_11 = [true,true,false];
_13 = core::ptr::addr_of!((*_13));
(*_13) = '\u{24d70}';
Call(_2 = core::intrinsics::bswap(_6), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_13 = core::ptr::addr_of!((*_13));
_22 = [167982953170600850075300108843167789500_i128,(-116417874501503134300542258098682498594_i128),(-35149658784218620948233380240211398189_i128),58295076409477641857365101766275685537_i128,(-119625788704575169498892220701620528339_i128)];
_25 = [29353_i16];
_13 = core::ptr::addr_of!(_10);
_7 = !_6;
_2 = !_8;
(*_13) = '\u{c6da3}';
(*_13) = '\u{9ec2f}';
_7 = _4 - _4;
_8 = _2;
_6 = _2 << _4;
_27 = [22317377988628369816528625424836458199_i128,100837546839548808251682407081814943860_i128,(-139684210167360365762227154956892671789_i128),(-38874795417942318758575756785285490475_i128),(-165478038612405799888946896990938245952_i128)];
_13 = core::ptr::addr_of!((*_13));
Goto(bb8)
}
bb8 = {
_10 = '\u{bfb7}';
_23 = _2 | _6;
_7 = -_6;
_27 = _22;
_11 = [false,false,true];
_12 = &_29;
_5 = _16;
Goto(bb9)
}
bb9 = {
(*_13) = '\u{e0af7}';
_15 = _18.0;
_4 = _6;
_16 = _5;
_8 = _2 + _23;
_16 = !_19.3;
_1 = -_23;
_5 = _16 >> _4;
Goto(bb10)
}
bb10 = {
_21 = &_3;
_22 = [111364899478918472336493535065907977267_i128,(-75786951137379844573924800916399051969_i128),(-77512695011807557418609931003816832406_i128),98445298262631851753989303725491598347_i128,9305095224480732433875543693415958053_i128];
_1 = _2 << _2;
_30 = false;
_18 = (_15,);
(*_13) = '\u{e5898}';
_18.0 = _15;
_1 = _2;
_16 = _5;
_20 = !_16;
_2 = 176_u8 as isize;
Call(_19.3 = fn14(Move(_21), _1, _1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_15 = _18.0 * _18.0;
_18 = (_15,);
Goto(bb12)
}
bb12 = {
_30 = _20 > _20;
_19.0 = 10739083051296605743_usize ^ 4_usize;
_15 = _18.0;
_21 = &_1;
_6 = _7 ^ (*_21);
_9 = RET;
_33 = (*_13);
_6 = 8076040724784258045_u64 as isize;
_37 = _15 + _15;
_14 = &_36;
_19.1 = [(*_13),_10,(*_13),_33,(*_13),_33];
_6 = _23 >> _7;
_22 = [30916027286624708468822625438987799348_i128,60694987834182124430717495586111918264_i128,(-159120305802644090909939968419024736146_i128),(-168688922835223483320774988767888012487_i128),74379193218117879347558805838251716436_i128];
_22 = [(-159720861805520701004406550505369364377_i128),147703330556699157560312539310447851790_i128,(-106441481579922286185000415814707666084_i128),(-56831530429568586896164767196911171593_i128),(-12508277716085411493843110785462976833_i128)];
_39 = _30 > _30;
Goto(bb13)
}
bb13 = {
_33 = (*_13);
RET = _9;
_4 = 4949292318717537339_i64 as isize;
_9 = [9059208634724693323_u64,3283496788764309863_u64,11262656500025199266_u64];
_7 = _20 as isize;
_22 = [(-59713244337069561507201143717085932548_i128),(-64943980984941379519190391285405770774_i128),76687980811747759823062053230311603112_i128,(-67608439685971021129929170554492326218_i128),(-29436292398091902387343863949283555566_i128)];
_13 = core::ptr::addr_of!(_33);
_13 = core::ptr::addr_of!((*_13));
_14 = &(*_14);
_18 = (_37,);
_18.0 = _15;
_22 = _27;
_36 = [_19.0,_19.0,_19.0,_19.0,_19.0,_19.0,_19.0,_19.0];
_32 = _9;
_8 = 276541555680353388306441476319591934596_u128 as isize;
(*_13) = _10;
_44 = (*_13);
_43.1 = _20;
Goto(bb14)
}
bb14 = {
_27 = _22;
_4 = (-85_i8) as isize;
_27 = [(-42039511137003120899364609747996511_i128),(-162966680972156836924505596561379471328_i128),111920166813357405017351517173840034286_i128,58177516133588146843667002039379828578_i128,(-62734307825963118110336995676692707636_i128)];
_27 = [(-84256925202837681238547033424483113236_i128),13740858116471260661604057100441042074_i128,164551148663910006979347342276859544577_i128,(-54138407530258858940274446116221225770_i128),31694735681021376012161570448457194848_i128];
_6 = _30 as isize;
_45 = _25;
RET = [8038027545324355371_u64,15131741961534460388_u64,13264419385215514043_u64];
_38 = _23;
_12 = &(*_12);
RET = [8551998033588428293_u64,414706350732021566_u64,9289160547744171029_u64];
_14 = &_36;
_20 = !_19.3;
_41 = Move(_14);
_45 = [(-32538_i16)];
_43.0 = 3908510894236472932_u64;
_5 = _20;
_6 = _30 as isize;
_23 = -_38;
(*_13) = _44;
_19.0 = !12650121720554479073_usize;
_9 = _32;
_46 = _39;
_19.0 = 1_usize - 6950282252867442546_usize;
_23 = _7;
Goto(bb15)
}
bb15 = {
Call(_48 = dump_var(8_usize, 23_usize, Move(_23), 2_usize, Move(_2), 6_usize, Move(_6), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_48 = dump_var(8_usize, 45_usize, Move(_45), 7_usize, Move(_7), 11_usize, Move(_11), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_48 = dump_var(8_usize, 32_usize, Move(_32), 38_usize, Move(_38), 30_usize, Move(_30), 46_usize, Move(_46)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(8_usize, 20_usize, Move(_20), 49_usize, _49, 49_usize, _49, 49_usize, _49), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize) -> [u64; 3] {
mir! {
type RET = [u64; 3];
let _9: &'static *const &'static i8;
let _10: ([u8; 4], char, [isize; 3]);
let _11: &'static isize;
let _12: (*mut *const i64, i128, *const u128, ([u16; 5], *const i64));
let _13: &'static [isize; 3];
let _14: isize;
let _15: *const u128;
let _16: isize;
let _17: [i128; 5];
let _18: ([u8; 4], char, [isize; 3]);
let _19: isize;
let _20: u128;
let _21: ((f64, f64, i64, bool), &'static *const i64);
let _22: Adt40;
let _23: [isize; 3];
let _24: (f64, f64, i64, bool);
let _25: [u16; 5];
let _26: f64;
let _27: isize;
let _28: Adt63;
let _29: [usize; 8];
let _30: &'static [isize; 3];
let _31: u8;
let _32: (*mut *const i64, i128, *const u128, ([u16; 5], *const i64));
let _33: isize;
let _34: char;
let _35: f32;
let _36: char;
let _37: *const &'static i8;
let _38: *const i64;
let _39: i8;
let _40: f32;
let _41: *const [u16; 5];
let _42: ();
let _43: ();
{
_6 = 998714072_i32 as isize;
RET = [4269869083212588728_u64,10952282713871337612_u64,448066411006362646_u64];
_2 = _4;
_5 = _4;
_2 = 23019154996473204430987495626524399059_u128 as isize;
RET = [13883470298091908445_u64,9763595024831841645_u64,16799603921199070665_u64];
_4 = 10518_i16 as isize;
_7 = -_5;
_2 = _7;
_3 = _8;
Goto(bb1)
}
bb1 = {
_10.0 = [48_u8,118_u8,42_u8,146_u8];
_4 = _2 + _2;
_10.1 = '\u{17270}';
_2 = false as isize;
_10.1 = '\u{8a0c1}';
_10.0 = [121_u8,96_u8,204_u8,173_u8];
RET = [5077989721313555922_u64,16886147418730660093_u64,1541288848313523254_u64];
_2 = _8;
_7 = _4 + _3;
_11 = &_1;
RET = [7638089397903972482_u64,5661383205921610320_u64,2227711928148893431_u64];
_13 = &_10.2;
_10.1 = '\u{669af}';
_10.2 = [(*_11),_7,_4];
_6 = _2;
_14 = (*_11) & _7;
_4 = 2952338607466053456_u64 as isize;
_7 = _14 | _2;
_2 = !_14;
_6 = 8840481603477012800_u64 as isize;
_2 = _4 | _8;
_12.1 = true as i128;
_13 = &_10.2;
_13 = &(*_13);
RET = [4157422789914311606_u64,8572521339738985969_u64,10023992909637571828_u64];
Call(_12.3.1 = fn10(Move(_11), (*_11), _14, _10, (*_11), _10.2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_11 = &_6;
_12.1 = !(-89017915382869674173865183361067933929_i128);
_10.2 = [_14,_14,_14];
_16 = _5;
_12.1 = (-39592699519591903877586597584451183924_i128);
_13 = &_10.2;
_4 = _14 << _7;
RET = [13364169521311096585_u64,9281168751590702323_u64,1033532100608446041_u64];
_5 = 13240837214160804983_usize as isize;
_12.3.0 = [35726_u16,50737_u16,19351_u16,44714_u16,21461_u16];
_11 = &(*_11);
_10.2 = [_14,_4,_4];
_11 = &(*_11);
_3 = !_4;
_13 = &_10.2;
_7 = _4;
RET = [4377735109052494782_u64,7409390864321131521_u64,7397418747917936361_u64];
_4 = _7 * _3;
_1 = _7 ^ _7;
_12.0 = core::ptr::addr_of_mut!(_12.3.1);
_13 = &(*_13);
_11 = &_7;
_19 = _7 | _1;
_13 = &(*_13);
_2 = _19 << _7;
_10.2 = [(*_11),_1,(*_11)];
_13 = &_10.2;
Call(_6 = core::intrinsics::bswap((*_11)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_15 = core::ptr::addr_of!(_20);
_18.1 = _10.1;
_8 = !(*_11);
_2 = _7;
Goto(bb4)
}
bb4 = {
_13 = &(*_13);
_13 = &_18.2;
_15 = core::ptr::addr_of!((*_15));
_12.2 = core::ptr::addr_of!((*_15));
_21.0.1 = 13_u8 as f64;
_1 = (*_11);
_18.2 = [_7,(*_11),_7];
_22.fld0 = core::ptr::addr_of!(_10.1);
_21.1 = &_12.3.1;
_21.0.2 = -(-8892677834492651192_i64);
_16 = _3 >> _7;
_21.0.2 = (-4649887942965573162_i64) << _14;
_18.0 = [160_u8,225_u8,61_u8,178_u8];
_12.3.1 = core::ptr::addr_of!(_21.0.2);
_20 = 1210_u16 as u128;
_1 = _7;
_21.0.2 = -(-5937711398791613112_i64);
(*_15) = false as u128;
_17 = [_12.1,_12.1,_12.1,_12.1,_12.1];
_5 = 10927292084770073846_u64 as isize;
_12.3.0 = [39475_u16,17227_u16,32257_u16,26357_u16,52145_u16];
_14 = 3798386350_u32 as isize;
_16 = !(*_11);
_7 = _16 << _3;
_21.0.0 = _21.0.1 + _21.0.1;
_16 = 20_u8 as isize;
match _12.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
300689667401346559585788009847317027532 => bb9,
_ => bb8
}
}
bb5 = {
_15 = core::ptr::addr_of!(_20);
_18.1 = _10.1;
_8 = !(*_11);
_2 = _7;
Goto(bb4)
}
bb6 = {
_11 = &_6;
_12.1 = !(-89017915382869674173865183361067933929_i128);
_10.2 = [_14,_14,_14];
_16 = _5;
_12.1 = (-39592699519591903877586597584451183924_i128);
_13 = &_10.2;
_4 = _14 << _7;
RET = [13364169521311096585_u64,9281168751590702323_u64,1033532100608446041_u64];
_5 = 13240837214160804983_usize as isize;
_12.3.0 = [35726_u16,50737_u16,19351_u16,44714_u16,21461_u16];
_11 = &(*_11);
_10.2 = [_14,_4,_4];
_11 = &(*_11);
_3 = !_4;
_13 = &_10.2;
_7 = _4;
RET = [4377735109052494782_u64,7409390864321131521_u64,7397418747917936361_u64];
_4 = _7 * _3;
_1 = _7 ^ _7;
_12.0 = core::ptr::addr_of_mut!(_12.3.1);
_13 = &(*_13);
_11 = &_7;
_19 = _7 | _1;
_13 = &(*_13);
_2 = _19 << _7;
_10.2 = [(*_11),_1,(*_11)];
_13 = &_10.2;
Call(_6 = core::intrinsics::bswap((*_11)), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_10.0 = [48_u8,118_u8,42_u8,146_u8];
_4 = _2 + _2;
_10.1 = '\u{17270}';
_2 = false as isize;
_10.1 = '\u{8a0c1}';
_10.0 = [121_u8,96_u8,204_u8,173_u8];
RET = [5077989721313555922_u64,16886147418730660093_u64,1541288848313523254_u64];
_2 = _8;
_7 = _4 + _3;
_11 = &_1;
RET = [7638089397903972482_u64,5661383205921610320_u64,2227711928148893431_u64];
_13 = &_10.2;
_10.1 = '\u{669af}';
_10.2 = [(*_11),_7,_4];
_6 = _2;
_14 = (*_11) & _7;
_4 = 2952338607466053456_u64 as isize;
_7 = _14 | _2;
_2 = !_14;
_6 = 8840481603477012800_u64 as isize;
_2 = _4 | _8;
_12.1 = true as i128;
_13 = &_10.2;
_13 = &(*_13);
RET = [4157422789914311606_u64,8572521339738985969_u64,10023992909637571828_u64];
Call(_12.3.1 = fn10(Move(_11), (*_11), _14, _10, (*_11), _10.2), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
_4 = 50564_u16 as isize;
_6 = _1 & _8;
_4 = _1;
_22.fld1 = 1536759644846650882_u64;
_22.fld1 = !15849743379945035183_u64;
match _12.1 {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
300689667401346559585788009847317027532 => bb10,
_ => bb7
}
}
bb10 = {
_10.2 = [_1,_19,_1];
_12.1 = (-15205037623919236704057145283401489616_i128) & 37983083559749190532219356110144493645_i128;
_18.2 = [_1,_2,_4];
_12.3.1 = core::ptr::addr_of!(_24.2);
_24 = (_21.0.1, _21.0.1, _21.0.2, true);
_12.0 = core::ptr::addr_of_mut!(_12.3.1);
_21.0.1 = _24.0 + _21.0.0;
_20 = _12.1 as u128;
_24.3 = false;
_21.0.3 = _19 < _6;
_21.0.3 = !_24.3;
_13 = &_10.2;
_23 = (*_13);
_21.0.2 = 76_u8 as i64;
_18.0 = [212_u8,154_u8,88_u8,181_u8];
_14 = !_7;
_25 = [15273_u16,8859_u16,31737_u16,24244_u16,46220_u16];
_14 = (-39_i8) as isize;
_18.0 = [28_u8,9_u8,14_u8,244_u8];
_25 = _12.3.0;
_17 = [_12.1,_12.1,_12.1,_12.1,_12.1];
_18.2 = [_6,_1,_2];
_11 = &_7;
_8 = _4;
_21.0.1 = -_24.1;
Goto(bb11)
}
bb11 = {
_21.0.2 = _21.0.3 as i64;
_15 = core::ptr::addr_of!((*_15));
RET = [_22.fld1,_22.fld1,_22.fld1];
_21.0.0 = -_24.0;
_22.fld0 = core::ptr::addr_of!(_10.1);
_27 = _1;
_12.3.0 = _25;
_26 = _24.0 - _21.0.0;
_12.1 = !(-67119283370883419506558188370892364283_i128);
Goto(bb12)
}
bb12 = {
_28.fld0.0 = _21.0.1 as u16;
_10.2 = [_7,(*_11),_19];
_24 = _21.0;
_12.3.0 = [_28.fld0.0,_28.fld0.0,_28.fld0.0,_28.fld0.0,_28.fld0.0];
_28.fld4 = _25;
_21.0.3 = !_24.3;
_21.1 = &_12.3.1;
_4 = _1;
_3 = _6;
_16 = _6;
_20 = 266089095962959694522311111787420692006_u128;
_8 = !_7;
_31 = _28.fld0.0 as u8;
_13 = &_10.2;
_14 = _8;
_21.0.0 = _24.0;
_7 = _21.0.3 as isize;
Call(_16 = core::intrinsics::transmute(_19), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_10.0 = [_31,_31,_31,_31];
_26 = -_21.0.1;
_32.1 = _12.1 + _12.1;
_28.fld3 = !_22.fld1;
_21.1 = &_12.3.1;
_14 = _3;
_18 = (_10.0, _10.1, (*_13));
_4 = -_3;
_17 = [_32.1,_32.1,_32.1,_32.1,_12.1];
_30 = Move(_13);
_28.fld0.0 = 76_i8 as u16;
_10 = (_18.0, _18.1, _23);
_10.2 = [_14,_1,_14];
_4 = _2 ^ _14;
_33 = _16;
_12.3.1 = core::ptr::addr_of!(_24.2);
RET = [_22.fld1,_28.fld3,_28.fld3];
_28.fld1 = Adt32::Variant0 { fld0: Move(_12.3) };
_32.1 = !_12.1;
_24 = (_21.0.1, _26, _21.0.2, _21.0.3);
SetDiscriminant(_28.fld1, 0);
_27 = _16 >> _19;
_19 = _2;
_17 = [_32.1,_12.1,_12.1,_12.1,_12.1];
_22.fld1 = _28.fld3;
match (*_15) {
0 => bb9,
1 => bb12,
266089095962959694522311111787420692006 => bb15,
_ => bb14
}
}
bb14 = {
_10.0 = [48_u8,118_u8,42_u8,146_u8];
_4 = _2 + _2;
_10.1 = '\u{17270}';
_2 = false as isize;
_10.1 = '\u{8a0c1}';
_10.0 = [121_u8,96_u8,204_u8,173_u8];
RET = [5077989721313555922_u64,16886147418730660093_u64,1541288848313523254_u64];
_2 = _8;
_7 = _4 + _3;
_11 = &_1;
RET = [7638089397903972482_u64,5661383205921610320_u64,2227711928148893431_u64];
_13 = &_10.2;
_10.1 = '\u{669af}';
_10.2 = [(*_11),_7,_4];
_6 = _2;
_14 = (*_11) & _7;
_4 = 2952338607466053456_u64 as isize;
_7 = _14 | _2;
_2 = !_14;
_6 = 8840481603477012800_u64 as isize;
_2 = _4 | _8;
_12.1 = true as i128;
_13 = &_10.2;
_13 = &(*_13);
RET = [4157422789914311606_u64,8572521339738985969_u64,10023992909637571828_u64];
Call(_12.3.1 = fn10(Move(_11), (*_11), _14, _10, (*_11), _10.2), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
_30 = &_23;
_32.2 = core::ptr::addr_of!((*_15));
_24.1 = _21.0.1 + _21.0.1;
_24.2 = _21.0.2;
_33 = _19 | _2;
_5 = _14;
_38 = core::ptr::addr_of!(_24.2);
_23 = [_27,_33,_16];
_24.0 = -_24.1;
_24 = (_21.0.0, _21.0.0, _21.0.2, _21.0.3);
_28.fld0 = (14262_u16,);
(*_38) = !_21.0.2;
_32.3 = (_28.fld4, Move(_38));
_35 = _28.fld3 as f32;
_28.fld2 = core::ptr::addr_of_mut!((*_15));
_28.fld3 = !_22.fld1;
_9 = &_37;
Goto(bb16)
}
bb16 = {
Call(_42 = dump_var(9_usize, 23_usize, Move(_23), 3_usize, Move(_3), 25_usize, Move(_25), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(9_usize, 4_usize, Move(_4), 8_usize, Move(_8), 6_usize, Move(_6), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(9_usize, 33_usize, Move(_33), 10_usize, Move(_10), 43_usize, _43, 43_usize, _43), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: &'static isize,mut _2: isize,mut _3: isize,mut _4: ([u8; 4], char, [isize; 3]),mut _5: isize,mut _6: [isize; 3]) -> *const i64 {
mir! {
type RET = *const i64;
let _7: f32;
let _8: [isize; 3];
let _9: f64;
let _10: isize;
let _11: f32;
let _12: u32;
let _13: isize;
let _14: &'static *const i64;
let _15: bool;
let _16: u64;
let _17: Adt65;
let _18: (u64, u16);
let _19: char;
let _20: (u16,);
let _21: i32;
let _22: i8;
let _23: f32;
let _24: Adt32;
let _25: u8;
let _26: isize;
let _27: usize;
let _28: *const u32;
let _29: f64;
let _30: &'static [u64; 5];
let _31: *const char;
let _32: ((i64, *const u16, &'static isize, *const u16),);
let _33: isize;
let _34: ();
let _35: ();
{
_6 = [_3,_3,_3];
_4.1 = '\u{f0e1d}';
_1 = &_5;
Call(_5 = fn11(_4.2, _3, _3, _3, _4.2, _6, _3, _3, _3, _6, _3, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = 2827697906324211734_i64 as f32;
_1 = &_3;
_6 = [(*_1),(*_1),_3];
_6 = [(*_1),_3,(*_1)];
_2 = 979084731_u32 as isize;
_2 = !_5;
_4.1 = '\u{a5e21}';
_6 = [_2,_3,(*_1)];
_3 = 1321718468_i32 as isize;
_3 = _5;
_7 = 158_u8 as f32;
_1 = &_5;
_8 = [(*_1),(*_1),_2];
_9 = 1991446303_u32 as f64;
_4.2 = [_2,(*_1),_3];
_4.1 = '\u{d1440}';
_5 = 281809387742514937116436479572866299748_u128 as isize;
_9 = (-7169874633635711239_i64) as f64;
_4.1 = '\u{ec724}';
_6 = [_3,_2,_2];
_7 = 6977947880205384962_i64 as f32;
_8 = [_2,_3,_3];
_8 = [_2,_3,_3];
_10 = -_2;
_8 = [_10,_10,_2];
Goto(bb2)
}
bb2 = {
_4.2 = [_10,_10,_10];
_2 = _3 - _3;
_4.2 = [_5,_3,_2];
_3 = -_2;
_10 = _3;
_4.2 = _6;
_8 = _4.2;
_5 = !_3;
_1 = &_5;
_10 = (*_1);
_12 = 3082002581_u32;
_1 = &(*_1);
_11 = _7 * _7;
_6 = _4.2;
match _12 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
3082002581 => bb8,
_ => bb7
}
}
bb3 = {
_7 = 2827697906324211734_i64 as f32;
_1 = &_3;
_6 = [(*_1),(*_1),_3];
_6 = [(*_1),_3,(*_1)];
_2 = 979084731_u32 as isize;
_2 = !_5;
_4.1 = '\u{a5e21}';
_6 = [_2,_3,(*_1)];
_3 = 1321718468_i32 as isize;
_3 = _5;
_7 = 158_u8 as f32;
_1 = &_5;
_8 = [(*_1),(*_1),_2];
_9 = 1991446303_u32 as f64;
_4.2 = [_2,(*_1),_3];
_4.1 = '\u{d1440}';
_5 = 281809387742514937116436479572866299748_u128 as isize;
_9 = (-7169874633635711239_i64) as f64;
_4.1 = '\u{ec724}';
_6 = [_3,_2,_2];
_7 = 6977947880205384962_i64 as f32;
_8 = [_2,_3,_3];
_8 = [_2,_3,_3];
_10 = -_2;
_8 = [_10,_10,_2];
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
_4.1 = '\u{38b9}';
Goto(bb9)
}
bb9 = {
_4.2 = _6;
_12 = 8458053969966556993_u64 as u32;
_4.0 = [108_u8,122_u8,73_u8,71_u8];
_5 = _10;
_10 = _2;
_2 = !_3;
_9 = 749308098_i32 as f64;
_8 = [_2,_2,_2];
_1 = &_3;
_4.1 = '\u{5f085}';
_5 = (-39_i8) as isize;
_4.0 = [55_u8,236_u8,231_u8,67_u8];
_2 = -_3;
_4.1 = '\u{2fe12}';
Goto(bb10)
}
bb10 = {
_1 = &_2;
_6 = [_10,_3,(*_1)];
_7 = -_11;
_13 = -_3;
_7 = _11 + _11;
_1 = &(*_1);
_10 = _2 + _13;
_4.0 = [223_u8,183_u8,220_u8,214_u8];
_4.0 = [126_u8,185_u8,169_u8,34_u8];
_4.2 = [_3,_10,_2];
_3 = -(*_1);
_4.1 = '\u{4cd38}';
_11 = _7 * _7;
_2 = _13;
_4.1 = '\u{e19cb}';
Goto(bb11)
}
bb11 = {
_18 = (13365551475166135726_u64, 33318_u16);
_15 = false;
_5 = -_13;
_4.2 = _8;
_14 = &RET;
_1 = &_5;
_18.0 = 13554074617604471247_u64 - 4546797124863771489_u64;
_10 = _3;
_16 = !_18.0;
_19 = _4.1;
_1 = &_13;
_4.2 = _8;
Goto(bb12)
}
bb12 = {
_12 = !2652370906_u32;
_18 = (_16, 11103_u16);
_12 = 1589809610_u32 & 2221571482_u32;
_5 = !_10;
_3 = -(*_1);
_19 = _4.1;
_19 = _4.1;
_4.2 = [(*_1),(*_1),_13];
_6 = [_13,(*_1),_2];
_4.0 = [77_u8,215_u8,161_u8,195_u8];
_12 = 2321096562_u32;
_20.0 = _7 as u16;
_13 = _20.0 as isize;
_7 = _11;
match _18.1 {
0 => bb6,
1 => bb4,
2 => bb13,
3 => bb14,
4 => bb15,
11103 => bb17,
_ => bb16
}
}
bb13 = {
_7 = 2827697906324211734_i64 as f32;
_1 = &_3;
_6 = [(*_1),(*_1),_3];
_6 = [(*_1),_3,(*_1)];
_2 = 979084731_u32 as isize;
_2 = !_5;
_4.1 = '\u{a5e21}';
_6 = [_2,_3,(*_1)];
_3 = 1321718468_i32 as isize;
_3 = _5;
_7 = 158_u8 as f32;
_1 = &_5;
_8 = [(*_1),(*_1),_2];
_9 = 1991446303_u32 as f64;
_4.2 = [_2,(*_1),_3];
_4.1 = '\u{d1440}';
_5 = 281809387742514937116436479572866299748_u128 as isize;
_9 = (-7169874633635711239_i64) as f64;
_4.1 = '\u{ec724}';
_6 = [_3,_2,_2];
_7 = 6977947880205384962_i64 as f32;
_8 = [_2,_3,_3];
_8 = [_2,_3,_3];
_10 = -_2;
_8 = [_10,_10,_2];
Goto(bb2)
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
_19 = _4.1;
_18.1 = !_20.0;
_23 = _7 + _7;
_21 = 113497465_i32;
_18.1 = _20.0;
_6 = _4.2;
_18.0 = _9 as u64;
_7 = -_23;
Goto(bb18)
}
bb18 = {
_15 = !true;
_19 = _4.1;
_4.2 = _6;
_8 = [_3,_5,_2];
_13 = !_10;
_8 = _4.2;
_18.0 = _16 ^ _16;
_12 = 3120247676_u32 - 1481914658_u32;
_27 = 0_usize + 2_usize;
_8 = [_13,_2,_13];
_7 = -_23;
match _21 {
0 => bb13,
1 => bb2,
2 => bb14,
3 => bb19,
4 => bb20,
5 => bb21,
6 => bb22,
113497465 => bb24,
_ => bb23
}
}
bb19 = {
_19 = _4.1;
_18.1 = !_20.0;
_23 = _7 + _7;
_21 = 113497465_i32;
_18.1 = _20.0;
_6 = _4.2;
_18.0 = _9 as u64;
_7 = -_23;
Goto(bb18)
}
bb20 = {
_4.2 = _6;
_12 = 8458053969966556993_u64 as u32;
_4.0 = [108_u8,122_u8,73_u8,71_u8];
_5 = _10;
_10 = _2;
_2 = !_3;
_9 = 749308098_i32 as f64;
_8 = [_2,_2,_2];
_1 = &_3;
_4.1 = '\u{5f085}';
_5 = (-39_i8) as isize;
_4.0 = [55_u8,236_u8,231_u8,67_u8];
_2 = -_3;
_4.1 = '\u{2fe12}';
Goto(bb10)
}
bb21 = {
_12 = !2652370906_u32;
_18 = (_16, 11103_u16);
_12 = 1589809610_u32 & 2221571482_u32;
_5 = !_10;
_3 = -(*_1);
_19 = _4.1;
_19 = _4.1;
_4.2 = [(*_1),(*_1),_13];
_6 = [_13,(*_1),_2];
_4.0 = [77_u8,215_u8,161_u8,195_u8];
_12 = 2321096562_u32;
_20.0 = _7 as u16;
_13 = _20.0 as isize;
_7 = _11;
match _18.1 {
0 => bb6,
1 => bb4,
2 => bb13,
3 => bb14,
4 => bb15,
11103 => bb17,
_ => bb16
}
}
bb22 = {
_4.1 = '\u{38b9}';
Goto(bb9)
}
bb23 = {
_7 = 2827697906324211734_i64 as f32;
_1 = &_3;
_6 = [(*_1),(*_1),_3];
_6 = [(*_1),_3,(*_1)];
_2 = 979084731_u32 as isize;
_2 = !_5;
_4.1 = '\u{a5e21}';
_6 = [_2,_3,(*_1)];
_3 = 1321718468_i32 as isize;
_3 = _5;
_7 = 158_u8 as f32;
_1 = &_5;
_8 = [(*_1),(*_1),_2];
_9 = 1991446303_u32 as f64;
_4.2 = [_2,(*_1),_3];
_4.1 = '\u{d1440}';
_5 = 281809387742514937116436479572866299748_u128 as isize;
_9 = (-7169874633635711239_i64) as f64;
_4.1 = '\u{ec724}';
_6 = [_3,_2,_2];
_7 = 6977947880205384962_i64 as f32;
_8 = [_2,_3,_3];
_8 = [_2,_3,_3];
_10 = -_2;
_8 = [_10,_10,_2];
Goto(bb2)
}
bb24 = {
_18.0 = (-165830842539099908158255716788558015841_i128) as u64;
_10 = _4.1 as isize;
_1 = &_10;
match _21 {
0 => bb20,
1 => bb25,
2 => bb26,
3 => bb27,
4 => bb28,
5 => bb29,
113497465 => bb31,
_ => bb30
}
}
bb25 = {
_1 = &_2;
_6 = [_10,_3,(*_1)];
_7 = -_11;
_13 = -_3;
_7 = _11 + _11;
_1 = &(*_1);
_10 = _2 + _13;
_4.0 = [223_u8,183_u8,220_u8,214_u8];
_4.0 = [126_u8,185_u8,169_u8,34_u8];
_4.2 = [_3,_10,_2];
_3 = -(*_1);
_4.1 = '\u{4cd38}';
_11 = _7 * _7;
_2 = _13;
_4.1 = '\u{e19cb}';
Goto(bb11)
}
bb26 = {
_4.2 = _6;
_12 = 8458053969966556993_u64 as u32;
_4.0 = [108_u8,122_u8,73_u8,71_u8];
_5 = _10;
_10 = _2;
_2 = !_3;
_9 = 749308098_i32 as f64;
_8 = [_2,_2,_2];
_1 = &_3;
_4.1 = '\u{5f085}';
_5 = (-39_i8) as isize;
_4.0 = [55_u8,236_u8,231_u8,67_u8];
_2 = -_3;
_4.1 = '\u{2fe12}';
Goto(bb10)
}
bb27 = {
_4.2 = [_10,_10,_10];
_2 = _3 - _3;
_4.2 = [_5,_3,_2];
_3 = -_2;
_10 = _3;
_4.2 = _6;
_8 = _4.2;
_5 = !_3;
_1 = &_5;
_10 = (*_1);
_12 = 3082002581_u32;
_1 = &(*_1);
_11 = _7 * _7;
_6 = _4.2;
match _12 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
3082002581 => bb8,
_ => bb7
}
}
bb28 = {
Return()
}
bb29 = {
_12 = !2652370906_u32;
_18 = (_16, 11103_u16);
_12 = 1589809610_u32 & 2221571482_u32;
_5 = !_10;
_3 = -(*_1);
_19 = _4.1;
_19 = _4.1;
_4.2 = [(*_1),(*_1),_13];
_6 = [_13,(*_1),_2];
_4.0 = [77_u8,215_u8,161_u8,195_u8];
_12 = 2321096562_u32;
_20.0 = _7 as u16;
_13 = _20.0 as isize;
_7 = _11;
match _18.1 {
0 => bb6,
1 => bb4,
2 => bb13,
3 => bb14,
4 => bb15,
11103 => bb17,
_ => bb16
}
}
bb30 = {
Return()
}
bb31 = {
_3 = -_13;
_8 = [_13,_13,_2];
_4.2 = [_13,_13,_2];
_3 = !_13;
_4.1 = _19;
_27 = 138815095958528073828832249135079028580_i128 as usize;
_18.1 = _20.0;
_10 = -_5;
_25 = !90_u8;
_23 = _11 * _7;
_11 = _23;
_1 = &_10;
_3 = 290078910394705168154109193449620892621_u128 as isize;
_13 = _2;
_6 = _8;
_2 = _5;
_18.1 = _20.0 | _20.0;
_27 = 6_usize;
_4.2 = [(*_1),_2,(*_1)];
_6 = [(*_1),(*_1),_13];
Call(RET = fn12(Move(_1), _10, _2, _2, _4.2, _8, _2, _13), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
_27 = 1_usize - 9082363924250375473_usize;
_4.0 = [_25,_25,_25,_25];
RET = core::ptr::addr_of!(_32.0.0);
_32.0.0 = 3457249933062501787_i64 * (-6082405156186312426_i64);
_32.0.3 = core::ptr::addr_of!(_20.0);
Goto(bb33)
}
bb33 = {
Call(_34 = dump_var(10_usize, 13_usize, Move(_13), 5_usize, Move(_5), 21_usize, Move(_21), 4_usize, Move(_4)), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Call(_34 = dump_var(10_usize, 20_usize, Move(_20), 18_usize, Move(_18), 25_usize, Move(_25), 6_usize, Move(_6)), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: [isize; 3],mut _2: isize,mut _3: isize,mut _4: isize,mut _5: [isize; 3],mut _6: [isize; 3],mut _7: isize,mut _8: isize,mut _9: isize,mut _10: [isize; 3],mut _11: isize,mut _12: isize) -> isize {
mir! {
type RET = isize;
let _13: &'static char;
let _14: ([u8; 4], char, [isize; 3]);
let _15: ();
let _16: ();
{
_6 = [_2,_4,_3];
_9 = _12;
_8 = _9 >> _7;
_10 = [_9,_2,_8];
RET = !_9;
_7 = _8 * _9;
_6 = _10;
_6 = _1;
_5 = [_9,_9,_7];
RET = _7;
_1 = [_8,_4,RET];
_6 = [_8,_7,_2];
RET = 92_u8 as isize;
_8 = !_3;
_3 = -_12;
RET = 717799549_i32 as isize;
Call(_3 = core::intrinsics::bswap(_11), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = 13084700154166113233_usize as isize;
_4 = _2 ^ _7;
_4 = 16937430221632930207_u64 as isize;
RET = _3;
_11 = _12;
_1 = _10;
_14.1 = '\u{2ca5}';
_3 = 2068_i16 as isize;
_14.2 = _10;
Goto(bb2)
}
bb2 = {
Call(_15 = dump_var(11_usize, 7_usize, Move(_7), 10_usize, Move(_10), 3_usize, Move(_3), 2_usize, Move(_2)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_15 = dump_var(11_usize, 1_usize, Move(_1), 5_usize, Move(_5), 16_usize, _16, 16_usize, _16), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: &'static isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: [isize; 3],mut _6: [isize; 3],mut _7: isize,mut _8: isize) -> *const i64 {
mir! {
type RET = *const i64;
let _9: bool;
let _10: [bool; 3];
let _11: (*mut u128,);
let _12: *mut usize;
let _13: isize;
let _14: (f64, f64, i64, bool);
let _15: &'static &'static u64;
let _16: &'static char;
let _17: isize;
let _18: (*const *mut u128,);
let _19: f32;
let _20: isize;
let _21: *mut u128;
let _22: i128;
let _23: isize;
let _24: isize;
let _25: usize;
let _26: bool;
let _27: i8;
let _28: f32;
let _29: u32;
let _30: isize;
let _31: Adt81;
let _32: ();
let _33: ();
{
_8 = _4 + _2;
_2 = !_4;
_6 = [_3,_4,_3];
_1 = &_3;
_4 = '\u{402fb}' as isize;
_4 = _8;
_7 = -_3;
_8 = _3 + _7;
_7 = (*_1);
_8 = _2 - _3;
_3 = (-14182416390470233857756803753056150948_i128) as isize;
_1 = &_3;
_2 = _8;
_8 = !_4;
_1 = &_3;
_2 = _8 & _7;
_3 = _7;
_5 = [_4,_8,_2];
_5 = [_3,_2,_8];
_9 = true;
_5 = [_8,_3,_7];
_3 = _8;
_3 = _4 & _8;
_7 = 50975_u16 as isize;
_6 = [_4,_2,_4];
_10 = [_9,_9,_9];
Goto(bb1)
}
bb1 = {
_13 = 214_u8 as isize;
_6 = _5;
_8 = _3 | _2;
_5 = _6;
_8 = 85232087978267866526487756943024969716_i128 as isize;
_13 = _2;
_1 = &_4;
_9 = (*_1) >= _2;
_10 = [_9,_9,_9];
_13 = 2788490198_u32 as isize;
_3 = -(*_1);
_5 = [_3,_2,(*_1)];
_7 = _2 - _4;
_10 = [_9,_9,_9];
_14.1 = 29463_u16 as f64;
Goto(bb2)
}
bb2 = {
_14.2 = 2837845957_u32 as i64;
_14.2 = _9 as i64;
_14.1 = (-26312_i16) as f64;
_14.2 = 1452367274090549898_i64;
_14.3 = _2 >= _7;
_7 = !_2;
_14.2 = 27298_u16 as i64;
_10 = [_9,_9,_9];
_2 = -_3;
_2 = _3;
_14.2 = 6177842850050966835_i64;
_14.0 = -_14.1;
_3 = _7 << _4;
_9 = _14.3;
_4 = _14.3 as isize;
_13 = _7;
match _14.2 {
0 => bb1,
6177842850050966835 => bb4,
_ => bb3
}
}
bb3 = {
_13 = 214_u8 as isize;
_6 = _5;
_8 = _3 | _2;
_5 = _6;
_8 = 85232087978267866526487756943024969716_i128 as isize;
_13 = _2;
_1 = &_4;
_9 = (*_1) >= _2;
_10 = [_9,_9,_9];
_13 = 2788490198_u32 as isize;
_3 = -(*_1);
_5 = [_3,_2,(*_1)];
_7 = _2 - _4;
_10 = [_9,_9,_9];
_14.1 = 29463_u16 as f64;
Goto(bb2)
}
bb4 = {
_14.0 = _14.1;
_6 = _5;
_14.0 = _14.2 as f64;
RET = core::ptr::addr_of!(_14.2);
RET = core::ptr::addr_of!(_14.2);
_13 = _7 * _2;
_3 = _2;
(*RET) = 189044429386715528_i64;
_13 = 55667_u16 as isize;
_14.1 = _14.0 - _14.0;
_4 = _7 >> _7;
_1 = &_3;
RET = core::ptr::addr_of!((*RET));
_10 = [_14.3,_14.3,_9];
_6 = [_7,_2,_4];
_2 = (*_1);
_14.2 = 14381_i16 as i64;
_7 = (*_1);
Goto(bb5)
}
bb5 = {
RET = core::ptr::addr_of!((*RET));
_9 = !_14.3;
(*RET) = 680040518251736667_i64;
_2 = (-631452228_i32) as isize;
(*RET) = -(-2215876341203946819_i64);
RET = core::ptr::addr_of!((*RET));
_8 = 15447_u16 as isize;
_14.1 = 39891096501806243553141982537923834477_u128 as f64;
Goto(bb6)
}
bb6 = {
_5 = [_8,_7,_4];
_17 = _7 ^ _7;
Call(_2 = core::intrinsics::bswap(_17), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
(*RET) = 280126514722388904_i64;
_6 = [_17,_17,_4];
_3 = -_4;
_8 = _17;
RET = core::ptr::addr_of!((*RET));
_6 = [_17,_17,_7];
Call(_14.2 = core::intrinsics::transmute(_3), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
RET = core::ptr::addr_of!(_14.2);
_17 = _8;
_14.2 = 62_i8 as i64;
RET = core::ptr::addr_of!(_14.2);
_14.0 = -_14.1;
_14.0 = 17653940950308238330_usize as f64;
_9 = _14.3;
Call(_3 = core::intrinsics::bswap(_4), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_19 = 3774678378558175926_u64 as f32;
_10 = [_14.3,_14.3,_9];
_14.2 = 4155405887098960284_i64 * (-5411895591334542728_i64);
RET = core::ptr::addr_of!(_14.2);
_10 = [_14.3,_14.3,_14.3];
(*RET) = 1862005223634997406_i64;
_14.2 = _9 as i64;
_6 = [_7,_3,_3];
_14.1 = _14.0 - _14.0;
_18.0 = core::ptr::addr_of!(_21);
(*RET) = (-8577648460430435827_i64) & (-8983013487352529872_i64);
_4 = 1017271449_u32 as isize;
_14.2 = 671124636539371073_i64;
_14.1 = _14.0;
_13 = -_3;
(*RET) = -(-7416818774674246539_i64);
_22 = !(-162548881498087119003776772980411886728_i128);
_9 = _13 <= _17;
RET = core::ptr::addr_of!((*RET));
(*RET) = -(-2013689474373405963_i64);
_7 = _13;
(*RET) = -(-9012506696434418768_i64);
(*RET) = !(-9000684269194078500_i64);
_13 = _14.2 as isize;
RET = core::ptr::addr_of!((*RET));
Goto(bb10)
}
bb10 = {
_14.3 = _9;
_13 = 4_usize as isize;
_24 = _3;
_1 = &_3;
_14.0 = _14.1;
Goto(bb11)
}
bb11 = {
_13 = _7;
_14.2 = (-156323367_i32) as i64;
_14.3 = !_9;
(*RET) = (-7236450302187168736_i64);
(*RET) = '\u{b5880}' as i64;
_4 = _3 * _13;
_3 = (-1875169433_i32) as isize;
_24 = -_4;
_14.3 = !_9;
_18.0 = core::ptr::addr_of!(_11.0);
_14.0 = _22 as f64;
(*RET) = (-2306010857471258862_i64) >> _24;
_20 = _4;
_9 = !_14.3;
_22 = !38657882252275504404051033927388253240_i128;
_4 = 94_i8 as isize;
(*RET) = _19 as i64;
_18.0 = core::ptr::addr_of!(_11.0);
_12 = core::ptr::addr_of_mut!(_25);
_24 = !_8;
_28 = _19 - _19;
_20 = '\u{e5ba0}' as isize;
_19 = _28 * _28;
(*RET) = !6880933069858595636_i64;
Goto(bb12)
}
bb12 = {
_27 = (-128_i8);
_29 = !3960829155_u32;
(*_12) = 1_usize;
_14.2 = 3898414255150220118_i64;
_1 = &_5[_25];
RET = core::ptr::addr_of!((*RET));
_14.3 = _13 < _24;
_24 = _5[_25] + (*_1);
_10[_25] = _14.3;
RET = core::ptr::addr_of!((*RET));
_18.0 = core::ptr::addr_of!(_21);
(*_12) = !13918197662395790736_usize;
_30 = -_13;
_30 = _28 as isize;
_2 = !_17;
_26 = !_14.3;
_4 = (*_1);
_24 = _7;
_12 = core::ptr::addr_of_mut!(_25);
(*RET) = -(-8157670819118516791_i64);
_18.0 = core::ptr::addr_of!(_21);
Goto(bb13)
}
bb13 = {
_20 = _19 as isize;
(*RET) = (-3633069466384332707_i64);
(*RET) = (-7885736401484503211_i64);
match _14.2 {
0 => bb1,
1 => bb12,
2 => bb3,
3 => bb9,
4 => bb5,
5 => bb11,
340282366920938463455488871030283708245 => bb14,
_ => bb10
}
}
bb14 = {
_3 = _22 as isize;
_23 = 44632_u16 as isize;
RET = core::ptr::addr_of!(_14.2);
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(12_usize, 6_usize, Move(_6), 23_usize, Move(_23), 24_usize, Move(_24), 27_usize, Move(_27)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(12_usize, 13_usize, Move(_13), 10_usize, Move(_10), 17_usize, Move(_17), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(12_usize, 4_usize, Move(_4), 29_usize, Move(_29), 33_usize, _33, 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: u16,mut _7: isize) -> isize {
mir! {
type RET = isize;
let _8: u8;
let _9: *const &'static i8;
let _10: [isize; 3];
let _11: ();
let _12: ();
{
_6 = 53419_u16 >> _2;
_6 = 42815_u16;
_5 = _7 * _7;
RET = _5;
RET = _5;
_3 = _1;
_3 = _5 ^ _7;
_8 = 142_u8 << _1;
_2 = _1 - _4;
_4 = _3;
_4 = _5 - _1;
_3 = -_5;
_3 = _2 | RET;
RET = !_3;
_3 = !_5;
_6 = !21222_u16;
_1 = RET + _3;
_4 = '\u{9defc}' as isize;
_3 = _1 + _1;
_3 = RET & RET;
RET = _1;
_2 = _3 | RET;
RET = -_3;
_7 = (-4414573094807230798_i64) as isize;
_6 = !11192_u16;
_3 = '\u{112}' as isize;
_10 = [_2,_1,_1];
_10 = [_2,_2,_5];
_2 = !RET;
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(13_usize, 3_usize, Move(_3), 6_usize, Move(_6), 7_usize, Move(_7), 1_usize, Move(_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: &'static isize,mut _2: isize,mut _3: isize) -> u16 {
mir! {
type RET = u16;
let _4: &'static u64;
let _5: Adt63;
let _6: f32;
let _7: ();
let _8: ();
{
_2 = -_3;
_5.fld3 = (-635755772_i32) as u64;
_5.fld0.0 = 6385_u16;
_5.fld4 = [_5.fld0.0,_5.fld0.0,_5.fld0.0,_5.fld0.0,_5.fld0.0];
Call(_5.fld0 = fn15(_3, _2, _3, _2, _3, _2, _5.fld3, _3, _3, _2, _2, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _2 as u16;
_4 = &_5.fld3;
_3 = -_2;
_5.fld0 = (RET,);
_6 = 3533027641_u32 as f32;
_5.fld0.0 = RET;
_2 = !_3;
_2 = !_3;
_1 = &_2;
_2 = _3;
_4 = &_5.fld3;
_5.fld0 = (RET,);
_2 = _3;
_1 = &_3;
_5.fld4 = [RET,_5.fld0.0,RET,RET,RET];
_1 = &(*_1);
_5.fld4 = [_5.fld0.0,RET,RET,_5.fld0.0,_5.fld0.0];
Goto(bb2)
}
bb2 = {
Call(_7 = dump_var(14_usize, 2_usize, Move(_2), 8_usize, _8, 8_usize, _8, 8_usize, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: u64,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize) -> (u16,) {
mir! {
type RET = (u16,);
let _13: *const [u16; 5];
let _14: [usize; 8];
let _15: [u64; 5];
let _16: &'static &'static [u64; 5];
let _17: *const i16;
let _18: [char; 6];
let _19: isize;
let _20: (*const *mut u128,);
let _21: &'static u64;
let _22: char;
let _23: char;
let _24: *const u16;
let _25: (f64, f64, i64, bool);
let _26: *const [u16; 5];
let _27: bool;
let _28: bool;
let _29: *const i64;
let _30: usize;
let _31: &'static *const i64;
let _32: Adt69;
let _33: &'static char;
let _34: u128;
let _35: *const u16;
let _36: *mut [bool; 3];
let _37: [u64; 5];
let _38: ();
let _39: ();
{
_8 = _6 << _3;
_3 = _8 ^ _10;
_8 = _10 * _4;
_12 = 11517_i16 as isize;
_1 = 209102205425339605274450576022743981797_u128 as isize;
RET = (55116_u16,);
RET.0 = 74007002897832573060168486043549891834_i128 as u16;
_2 = 3419090999_u32 as isize;
_1 = !_4;
_4 = _10;
_7 = 2450601609593778526_usize as u64;
_9 = _8 >> _4;
_8 = !_1;
RET.0 = !55833_u16;
_7 = 10725716888140086703_u64;
_3 = _7 as isize;
_5 = !_1;
match _7 {
10725716888140086703 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_8 = _9;
_9 = 527468464_i32 as isize;
_15 = [_7,_7,_7,_7,_7];
RET.0 = 21401_u16 - 65201_u16;
_14 = [6794426154487448633_usize,16105735185798154456_usize,2_usize,208471145620947330_usize,3100226983876806420_usize,1_usize,2_usize,15752862453350005216_usize];
_18 = ['\u{c2611}','\u{b3708}','\u{25917}','\u{184f1}','\u{244fc}','\u{619a6}'];
_1 = !_4;
RET = (25389_u16,);
_8 = _4;
_2 = _11;
_6 = _1;
Goto(bb3)
}
bb3 = {
_5 = _11 >> _6;
RET = (37575_u16,);
_11 = 14988_i16 as isize;
_19 = _6 - _10;
RET = (18861_u16,);
_8 = _6 + _1;
_5 = _8;
_15 = [_7,_7,_7,_7,_7];
_18 = ['\u{fc4a6}','\u{f73fa}','\u{1797d}','\u{ebdb2}','\u{1c7d9}','\u{68618}'];
_18 = ['\u{c9628}','\u{8ec3f}','\u{d4679}','\u{bc63c}','\u{b2495}','\u{10e39c}'];
_18 = ['\u{96f85}','\u{c8827}','\u{1ec57}','\u{983b6}','\u{c4bcd}','\u{524b9}'];
_14 = [5_usize,1_usize,7_usize,1649805019006395385_usize,1000795943063933063_usize,6000843426844436897_usize,11369018997407238746_usize,4_usize];
_2 = -_5;
_2 = 10150670992381599545_usize as isize;
_9 = -_5;
_3 = -_19;
RET.0 = _7 as u16;
_10 = _6 << _6;
_9 = _19 & _8;
_1 = -_5;
RET = (47321_u16,);
_7 = !5550671005481419669_u64;
RET.0 = !56861_u16;
RET = (43235_u16,);
_18 = ['\u{b1561}','\u{bd99e}','\u{214a9}','\u{e33a1}','\u{5f44c}','\u{3e0e3}'];
_2 = !_4;
_14 = [2_usize,5_usize,3285529501706054337_usize,11302980808386759563_usize,1_usize,10365852545393017507_usize,6533323081043278294_usize,8669475675626477216_usize];
RET = (25884_u16,);
match RET.0 {
0 => bb1,
1 => bb2,
25884 => bb5,
_ => bb4
}
}
bb4 = {
Return()
}
bb5 = {
_23 = '\u{235e9}';
RET = (30851_u16,);
_23 = '\u{f5a5e}';
_9 = _19;
_22 = _23;
Call(_12 = core::intrinsics::bswap(_5), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_6 = _10 - _5;
_9 = _5;
RET.0 = (-26846158909277720436993226067705325750_i128) as u16;
_11 = _5 + _3;
_6 = -_19;
RET = (65115_u16,);
_7 = !11762299192357106003_u64;
RET.0 = !7958_u16;
_4 = _5;
_19 = _10;
Call(_3 = fn16(_6, _5, _8, _1, _6, _1, _8, _2, _10, _11, _5, _11), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_19 = _5 - _1;
_24 = core::ptr::addr_of!(RET.0);
RET = (60628_u16,);
_10 = _6;
_1 = _4;
_8 = !_19;
RET.0 = 17901_i16 as u16;
_5 = !_6;
RET.0 = 22663_u16 ^ 14563_u16;
(*_24) = !38951_u16;
RET.0 = 39688_u16 - 49037_u16;
RET = (59973_u16,);
_25.3 = !false;
_14 = [13548146061953741453_usize,561194886206908042_usize,11912154226514710387_usize,7_usize,14053575453777601351_usize,6_usize,17798749833213521485_usize,5407990467722904034_usize];
match (*_24) {
0 => bb5,
59973 => bb8,
_ => bb2
}
}
bb8 = {
(*_24) = 13201_u16 - 8744_u16;
_27 = _3 <= _11;
_2 = _3 | _9;
_6 = (-30307_i16) as isize;
_8 = _10 >> _4;
_7 = (-1283202205_i32) as u64;
Goto(bb9)
}
bb9 = {
RET = (27591_u16,);
_19 = _2;
_14 = [7_usize,10453079531113262754_usize,3309298066367678080_usize,5_usize,8939643751583853842_usize,3_usize,0_usize,52819594877597818_usize];
_21 = &_7;
(*_24) = !39714_u16;
_1 = !_10;
_28 = _4 < _4;
RET.0 = 63502_u16;
_4 = _2;
_31 = &_29;
_8 = !_4;
_1 = _4;
_10 = _11 + _3;
_7 = !6129039143728072782_u64;
_4 = _1 << _3;
_29 = core::ptr::addr_of!(_25.2);
_3 = !_9;
match (*_24) {
0 => bb7,
1 => bb6,
2 => bb8,
3 => bb10,
4 => bb11,
5 => bb12,
63502 => bb14,
_ => bb13
}
}
bb10 = {
Return()
}
bb11 = {
_5 = _11 >> _6;
RET = (37575_u16,);
_11 = 14988_i16 as isize;
_19 = _6 - _10;
RET = (18861_u16,);
_8 = _6 + _1;
_5 = _8;
_15 = [_7,_7,_7,_7,_7];
_18 = ['\u{fc4a6}','\u{f73fa}','\u{1797d}','\u{ebdb2}','\u{1c7d9}','\u{68618}'];
_18 = ['\u{c9628}','\u{8ec3f}','\u{d4679}','\u{bc63c}','\u{b2495}','\u{10e39c}'];
_18 = ['\u{96f85}','\u{c8827}','\u{1ec57}','\u{983b6}','\u{c4bcd}','\u{524b9}'];
_14 = [5_usize,1_usize,7_usize,1649805019006395385_usize,1000795943063933063_usize,6000843426844436897_usize,11369018997407238746_usize,4_usize];
_2 = -_5;
_2 = 10150670992381599545_usize as isize;
_9 = -_5;
_3 = -_19;
RET.0 = _7 as u16;
_10 = _6 << _6;
_9 = _19 & _8;
_1 = -_5;
RET = (47321_u16,);
_7 = !5550671005481419669_u64;
RET.0 = !56861_u16;
RET = (43235_u16,);
_18 = ['\u{b1561}','\u{bd99e}','\u{214a9}','\u{e33a1}','\u{5f44c}','\u{3e0e3}'];
_2 = !_4;
_14 = [2_usize,5_usize,3285529501706054337_usize,11302980808386759563_usize,1_usize,10365852545393017507_usize,6533323081043278294_usize,8669475675626477216_usize];
RET = (25884_u16,);
match RET.0 {
0 => bb1,
1 => bb2,
25884 => bb5,
_ => bb4
}
}
bb12 = {
_6 = _10 - _5;
_9 = _5;
RET.0 = (-26846158909277720436993226067705325750_i128) as u16;
_11 = _5 + _3;
_6 = -_19;
RET = (65115_u16,);
_7 = !11762299192357106003_u64;
RET.0 = !7958_u16;
_4 = _5;
_19 = _10;
Call(_3 = fn16(_6, _5, _8, _1, _6, _1, _8, _2, _10, _11, _5, _11), ReturnTo(bb7), UnwindUnreachable())
}
bb13 = {
_23 = '\u{235e9}';
RET = (30851_u16,);
_23 = '\u{f5a5e}';
_9 = _19;
_22 = _23;
Call(_12 = core::intrinsics::bswap(_5), ReturnTo(bb6), UnwindUnreachable())
}
bb14 = {
_1 = (-10_i8) as isize;
RET.0 = (-104459496298340091577061049149041188584_i128) as u16;
_11 = -_3;
_30 = !4942167141621610865_usize;
_30 = !3_usize;
_3 = (-32_i8) as isize;
_25.3 = !_27;
_33 = &_22;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(15_usize, 19_usize, Move(_19), 11_usize, Move(_11), 6_usize, Move(_6), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(15_usize, 2_usize, Move(_2), 22_usize, Move(_22), 27_usize, Move(_27), 28_usize, Move(_28)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(15_usize, 5_usize, Move(_5), 18_usize, Move(_18), 39_usize, _39, 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize) -> isize {
mir! {
type RET = isize;
let _13: f64;
let _14: f32;
let _15: ();
let _16: ();
{
_12 = _2;
_4 = _11 ^ _12;
_12 = _4;
_1 = !_10;
_12 = -_1;
RET = !_1;
_2 = 4684_i16 as isize;
RET = -_4;
_13 = 4288537468992165201_i64 as f64;
_9 = 47169078286908628313947392142377202342_i128 as isize;
_5 = -_4;
_3 = _5;
_13 = 1246665718633914623_usize as f64;
_10 = _8 ^ _8;
Goto(bb1)
}
bb1 = {
Call(_15 = dump_var(16_usize, 1_usize, Move(_1), 10_usize, Move(_10), 6_usize, Move(_6), 7_usize, Move(_7)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_15 = dump_var(16_usize, 3_usize, Move(_3), 4_usize, Move(_4), 16_usize, _16, 16_usize, _16), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize) -> [u64; 5] {
mir! {
type RET = [u64; 5];
let _7: &'static i16;
let _8: [i16; 1];
let _9: isize;
let _10: *mut usize;
let _11: char;
let _12: f32;
let _13: char;
let _14: bool;
let _15: u32;
let _16: f64;
let _17: Adt32;
let _18: (i64, *const u16, &'static isize, *const u16);
let _19: u128;
let _20: ([u8; 4], char, [isize; 3]);
let _21: f32;
let _22: [usize; 8];
let _23: ();
let _24: ();
{
RET = [85937084412156254_u64,6815626382095600402_u64,16344862876462802336_u64,16280670563555974612_u64,2779927304972027928_u64];
_6 = _5;
_2 = _4 - _6;
_3 = -_4;
_4 = _1;
_2 = _3;
RET = [3999211228658128454_u64,6665459059359255949_u64,5343643232636629971_u64,2926008927162867785_u64,1254262142502660763_u64];
_5 = _3;
_1 = false as isize;
_5 = _2 - _6;
RET = [14694493493308404644_u64,8196038969495540415_u64,514527865679141885_u64,5144474820113458926_u64,8404348312019111944_u64];
_4 = _3 | _6;
Call(_2 = core::intrinsics::transmute(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = -_6;
RET = [12959279040446911745_u64,75626453027051965_u64,17037129785846990323_u64,241470251214368641_u64,13276061730734163838_u64];
_5 = _2;
_9 = -_2;
_9 = _5 * _6;
_5 = !_4;
_3 = _5 * _9;
_4 = 30323_u16 as isize;
_9 = 30_u8 as isize;
_3 = _5 << _6;
Goto(bb2)
}
bb2 = {
_8 = [28492_i16];
_6 = _2 + _3;
_8 = [15584_i16];
_1 = 229196705349147182760816492306692345814_u128 as isize;
RET = [13506651635333950639_u64,9134649915082226122_u64,9399392013763483514_u64,3710183276440118020_u64,16523982477720777785_u64];
_13 = '\u{92d73}';
_14 = true & true;
_12 = 17_i8 as f32;
_2 = _5;
_3 = _6 << _2;
_9 = 132_u8 as isize;
RET = [15441366885918289412_u64,5976965570981402370_u64,572488180483102487_u64,4321943277445121646_u64,7536955048476890822_u64];
_6 = (-2068779528939030492_i64) as isize;
RET = [16399780314208784495_u64,17423028651326882038_u64,1243715513095874809_u64,5807142327418083897_u64,3290292786668304633_u64];
_11 = _13;
_3 = !_2;
_11 = _13;
_13 = _11;
_5 = _2;
RET = [2003824717009767004_u64,11164577435849420626_u64,3987416550259720981_u64,12075035064635882385_u64,10266922999429584005_u64];
_11 = _13;
_1 = _2;
_14 = !true;
_1 = -_5;
Goto(bb3)
}
bb3 = {
RET = [16679031973338121245_u64,12768608980090131061_u64,17939433467801158214_u64,18403616502560824721_u64,1870201405336282077_u64];
_15 = 529381444_u32 | 26904420_u32;
_14 = true ^ true;
RET = [17907402465182376959_u64,6424056411761352486_u64,10269430628001822565_u64,9498666080898385143_u64,9993757799634429783_u64];
_14 = !false;
_3 = _2;
_13 = _11;
_13 = _11;
_3 = _2;
_11 = _13;
_16 = 57_u8 as f64;
_15 = 3946453130_u32;
_19 = !310268923723705064871079548860148933829_u128;
_15 = 2675089591_u32;
match _15 {
0 => bb4,
1 => bb5,
2675089591 => bb7,
_ => bb6
}
}
bb4 = {
_8 = [28492_i16];
_6 = _2 + _3;
_8 = [15584_i16];
_1 = 229196705349147182760816492306692345814_u128 as isize;
RET = [13506651635333950639_u64,9134649915082226122_u64,9399392013763483514_u64,3710183276440118020_u64,16523982477720777785_u64];
_13 = '\u{92d73}';
_14 = true & true;
_12 = 17_i8 as f32;
_2 = _5;
_3 = _6 << _2;
_9 = 132_u8 as isize;
RET = [15441366885918289412_u64,5976965570981402370_u64,572488180483102487_u64,4321943277445121646_u64,7536955048476890822_u64];
_6 = (-2068779528939030492_i64) as isize;
RET = [16399780314208784495_u64,17423028651326882038_u64,1243715513095874809_u64,5807142327418083897_u64,3290292786668304633_u64];
_11 = _13;
_3 = !_2;
_11 = _13;
_13 = _11;
_5 = _2;
RET = [2003824717009767004_u64,11164577435849420626_u64,3987416550259720981_u64,12075035064635882385_u64,10266922999429584005_u64];
_11 = _13;
_1 = _2;
_14 = !true;
_1 = -_5;
Goto(bb3)
}
bb5 = {
_3 = -_6;
RET = [12959279040446911745_u64,75626453027051965_u64,17037129785846990323_u64,241470251214368641_u64,13276061730734163838_u64];
_5 = _2;
_9 = -_2;
_9 = _5 * _6;
_5 = !_4;
_3 = _5 * _9;
_4 = 30323_u16 as isize;
_9 = 30_u8 as isize;
_3 = _5 << _6;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
RET = [827292216218621096_u64,12421119506454555235_u64,13458392747819531365_u64,6840970856215725387_u64,10374479700019577701_u64];
_21 = _12 * _12;
_1 = _3;
_21 = 89_i8 as f32;
_4 = _3 << _1;
_19 = 140266206794427243853213720386827813736_u128 | 122021716227557166317387062051629613759_u128;
_22 = [11490524296923087212_usize,4_usize,16766157462149842551_usize,8179348487847875264_usize,5051426908146648273_usize,6949581615074068808_usize,7_usize,1_usize];
match _15 {
0 => bb4,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
2675089591 => bb13,
_ => bb12
}
}
bb8 = {
Return()
}
bb9 = {
_3 = -_6;
RET = [12959279040446911745_u64,75626453027051965_u64,17037129785846990323_u64,241470251214368641_u64,13276061730734163838_u64];
_5 = _2;
_9 = -_2;
_9 = _5 * _6;
_5 = !_4;
_3 = _5 * _9;
_4 = 30323_u16 as isize;
_9 = 30_u8 as isize;
_3 = _5 << _6;
Goto(bb2)
}
bb10 = {
_8 = [28492_i16];
_6 = _2 + _3;
_8 = [15584_i16];
_1 = 229196705349147182760816492306692345814_u128 as isize;
RET = [13506651635333950639_u64,9134649915082226122_u64,9399392013763483514_u64,3710183276440118020_u64,16523982477720777785_u64];
_13 = '\u{92d73}';
_14 = true & true;
_12 = 17_i8 as f32;
_2 = _5;
_3 = _6 << _2;
_9 = 132_u8 as isize;
RET = [15441366885918289412_u64,5976965570981402370_u64,572488180483102487_u64,4321943277445121646_u64,7536955048476890822_u64];
_6 = (-2068779528939030492_i64) as isize;
RET = [16399780314208784495_u64,17423028651326882038_u64,1243715513095874809_u64,5807142327418083897_u64,3290292786668304633_u64];
_11 = _13;
_3 = !_2;
_11 = _13;
_13 = _11;
_5 = _2;
RET = [2003824717009767004_u64,11164577435849420626_u64,3987416550259720981_u64,12075035064635882385_u64,10266922999429584005_u64];
_11 = _13;
_1 = _2;
_14 = !true;
_1 = -_5;
Goto(bb3)
}
bb11 = {
_3 = -_6;
RET = [12959279040446911745_u64,75626453027051965_u64,17037129785846990323_u64,241470251214368641_u64,13276061730734163838_u64];
_5 = _2;
_9 = -_2;
_9 = _5 * _6;
_5 = !_4;
_3 = _5 * _9;
_4 = 30323_u16 as isize;
_9 = 30_u8 as isize;
_3 = _5 << _6;
Goto(bb2)
}
bb12 = {
_8 = [28492_i16];
_6 = _2 + _3;
_8 = [15584_i16];
_1 = 229196705349147182760816492306692345814_u128 as isize;
RET = [13506651635333950639_u64,9134649915082226122_u64,9399392013763483514_u64,3710183276440118020_u64,16523982477720777785_u64];
_13 = '\u{92d73}';
_14 = true & true;
_12 = 17_i8 as f32;
_2 = _5;
_3 = _6 << _2;
_9 = 132_u8 as isize;
RET = [15441366885918289412_u64,5976965570981402370_u64,572488180483102487_u64,4321943277445121646_u64,7536955048476890822_u64];
_6 = (-2068779528939030492_i64) as isize;
RET = [16399780314208784495_u64,17423028651326882038_u64,1243715513095874809_u64,5807142327418083897_u64,3290292786668304633_u64];
_11 = _13;
_3 = !_2;
_11 = _13;
_13 = _11;
_5 = _2;
RET = [2003824717009767004_u64,11164577435849420626_u64,3987416550259720981_u64,12075035064635882385_u64,10266922999429584005_u64];
_11 = _13;
_1 = _2;
_14 = !true;
_1 = -_5;
Goto(bb3)
}
bb13 = {
_18.2 = &_4;
_8 = [20873_i16];
_20.0 = [176_u8,22_u8,10_u8,69_u8];
_18.0 = (-7723979474201882237_i64);
_11 = _13;
_20.1 = _11;
Goto(bb14)
}
bb14 = {
_18.2 = &_5;
Goto(bb15)
}
bb15 = {
Call(_23 = dump_var(17_usize, 11_usize, Move(_11), 14_usize, Move(_14), 13_usize, Move(_13), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_23 = dump_var(17_usize, 2_usize, Move(_2), 3_usize, Move(_3), 1_usize, Move(_1), 24_usize, _24), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: Adt69,mut _2: isize,mut _3: [u64; 3],mut _4: bool,mut _5: char,mut _6: isize,mut _7: bool) -> i16 {
mir! {
type RET = i16;
let _8: isize;
let _9: u128;
let _10: *mut *const i64;
let _11: bool;
let _12: isize;
let _13: isize;
let _14: i128;
let _15: *mut *const i64;
let _16: &'static i8;
let _17: [bool; 3];
let _18: [i128; 5];
let _19: *const Adt32;
let _20: isize;
let _21: [usize; 8];
let _22: (f64, f64, i64, bool);
let _23: [i16; 1];
let _24: isize;
let _25: Adt40;
let _26: u128;
let _27: ((i64, *const u16, &'static isize, *const u16),);
let _28: ([u8; 4], char, [isize; 3]);
let _29: isize;
let _30: [u64; 5];
let _31: ();
let _32: ();
{
place!(Field::<usize>(Variant(_1, 2), 3)) = !3940997873674417682_usize;
place!(Field::<bool>(Variant(_1, 2), 0)) = _7 < _4;
_2 = _6;
_2 = !_6;
place!(Field::<u32>(Variant(_1, 2), 4)) = 1194686891_u32;
RET = -23118_i16;
place!(Field::<char>(Variant(_1, 2), 1)) = _5;
_8 = !_2;
_9 = 204607157102910037742625425493762057360_u128;
Goto(bb1)
}
bb1 = {
RET = (-8513_i16) * (-5486_i16);
_4 = Field::<bool>(Variant(_1, 2), 0);
_5 = Field::<char>(Variant(_1, 2), 1);
SetDiscriminant(_1, 3);
place!(Field::<Adt63>(Variant(_1, 3), 3)).fld3 = !3678377721890169486_u64;
place!(Field::<[isize; 3]>(Variant(_1, 3), 0)) = [_8,_6,_6];
match _9 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
204607157102910037742625425493762057360 => bb8,
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
_4 = _7;
place!(Field::<Adt63>(Variant(_1, 3), 3)).fld4 = [30910_u16,44893_u16,29117_u16,11154_u16,42563_u16];
place!(Field::<[isize; 3]>(Variant(_1, 3), 0)) = [_8,_2,_8];
place!(Field::<Adt63>(Variant(_1, 3), 3)).fld0 = (43890_u16,);
place!(Field::<Adt63>(Variant(_1, 3), 3)).fld2 = core::ptr::addr_of_mut!(_9);
place!(Field::<Adt63>(Variant(_1, 3), 3)).fld0 = (4803_u16,);
_11 = _4;
place!(Field::<Adt63>(Variant(_1, 3), 3)).fld4 = [Field::<Adt63>(Variant(_1, 3), 3).fld0.0,Field::<Adt63>(Variant(_1, 3), 3).fld0.0,Field::<Adt63>(Variant(_1, 3), 3).fld0.0,Field::<Adt63>(Variant(_1, 3), 3).fld0.0,Field::<Adt63>(Variant(_1, 3), 3).fld0.0];
place!(Field::<Adt63>(Variant(_1, 3), 3)).fld4 = [Field::<Adt63>(Variant(_1, 3), 3).fld0.0,Field::<Adt63>(Variant(_1, 3), 3).fld0.0,Field::<Adt63>(Variant(_1, 3), 3).fld0.0,Field::<Adt63>(Variant(_1, 3), 3).fld0.0,Field::<Adt63>(Variant(_1, 3), 3).fld0.0];
_5 = '\u{d52ec}';
_5 = '\u{6ad51}';
Goto(bb9)
}
bb9 = {
_1 = Adt69::Variant2 { fld0: _11,fld1: _5,fld2: _3,fld3: 2_usize,fld4: 2257576311_u32 };
_12 = !_6;
_7 = _4 ^ _4;
RET = 21470_i16 | (-27069_i16);
_8 = 2276917720_u32 as isize;
RET = 3368357481_u32 as i16;
_11 = !_7;
_8 = 49468_u16 as isize;
_12 = _6;
_4 = _7 == _11;
_5 = Field::<char>(Variant(_1, 2), 1);
match _9 {
0 => bb1,
1 => bb7,
2 => bb6,
3 => bb8,
204607157102910037742625425493762057360 => bb10,
_ => bb5
}
}
bb10 = {
_7 = !_11;
place!(Field::<[u64; 3]>(Variant(_1, 2), 2)) = [3553290914360382580_u64,523497504357421047_u64,10280718215392612266_u64];
_6 = _5 as isize;
_3 = [4869979094232586841_u64,14171409155830636215_u64,5127544121554763527_u64];
_3 = [6947325956985333704_u64,573133819442473249_u64,6456031078653816596_u64];
place!(Field::<u32>(Variant(_1, 2), 4)) = 3361859370_u32;
_6 = 123_u8 as isize;
_17 = [Field::<bool>(Variant(_1, 2), 0),Field::<bool>(Variant(_1, 2), 0),_11];
_7 = _4;
place!(Field::<u32>(Variant(_1, 2), 4)) = 2829095489_u32;
_8 = _12;
place!(Field::<char>(Variant(_1, 2), 1)) = _5;
_17 = [_4,_7,_11];
_13 = _2 << _12;
_21 = [2854280918570889361_usize,5_usize,7394312909971046949_usize,0_usize,1_usize,5_usize,4_usize,3_usize];
_22.3 = !_11;
_23 = [RET];
_24 = _8 * _13;
match _9 {
204607157102910037742625425493762057360 => bb12,
_ => bb11
}
}
bb11 = {
RET = (-8513_i16) * (-5486_i16);
_4 = Field::<bool>(Variant(_1, 2), 0);
_5 = Field::<char>(Variant(_1, 2), 1);
SetDiscriminant(_1, 3);
place!(Field::<Adt63>(Variant(_1, 3), 3)).fld3 = !3678377721890169486_u64;
place!(Field::<[isize; 3]>(Variant(_1, 3), 0)) = [_8,_6,_6];
match _9 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
204607157102910037742625425493762057360 => bb8,
_ => bb7
}
}
bb12 = {
_20 = Field::<u32>(Variant(_1, 2), 4) as isize;
_17 = [_22.3,_11,_4];
_6 = _24 & _12;
_25.fld1 = 7986794218291786500_u64 * 14062362774743930728_u64;
_3 = Field::<[u64; 3]>(Variant(_1, 2), 2);
_22.1 = _25.fld1 as f64;
place!(Field::<u32>(Variant(_1, 2), 4)) = 1125006710_u32;
place!(Field::<[u64; 3]>(Variant(_1, 2), 2)) = [_25.fld1,_25.fld1,_25.fld1];
_22.2 = 7016564758618854237_i64;
place!(Field::<usize>(Variant(_1, 2), 3)) = 2_usize;
SetDiscriminant(_1, 3);
_23 = [RET];
match _22.2 {
0 => bb4,
1 => bb13,
2 => bb14,
3 => bb15,
7016564758618854237 => bb17,
_ => bb16
}
}
bb13 = {
RET = (-8513_i16) * (-5486_i16);
_4 = Field::<bool>(Variant(_1, 2), 0);
_5 = Field::<char>(Variant(_1, 2), 1);
SetDiscriminant(_1, 3);
place!(Field::<Adt63>(Variant(_1, 3), 3)).fld3 = !3678377721890169486_u64;
place!(Field::<[isize; 3]>(Variant(_1, 3), 0)) = [_8,_6,_6];
match _9 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
204607157102910037742625425493762057360 => bb8,
_ => bb7
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_4 = _7;
place!(Field::<Adt63>(Variant(_1, 3), 3)).fld4 = [30910_u16,44893_u16,29117_u16,11154_u16,42563_u16];
place!(Field::<[isize; 3]>(Variant(_1, 3), 0)) = [_8,_2,_8];
place!(Field::<Adt63>(Variant(_1, 3), 3)).fld0 = (43890_u16,);
place!(Field::<Adt63>(Variant(_1, 3), 3)).fld2 = core::ptr::addr_of_mut!(_9);
place!(Field::<Adt63>(Variant(_1, 3), 3)).fld0 = (4803_u16,);
_11 = _4;
place!(Field::<Adt63>(Variant(_1, 3), 3)).fld4 = [Field::<Adt63>(Variant(_1, 3), 3).fld0.0,Field::<Adt63>(Variant(_1, 3), 3).fld0.0,Field::<Adt63>(Variant(_1, 3), 3).fld0.0,Field::<Adt63>(Variant(_1, 3), 3).fld0.0,Field::<Adt63>(Variant(_1, 3), 3).fld0.0];
place!(Field::<Adt63>(Variant(_1, 3), 3)).fld4 = [Field::<Adt63>(Variant(_1, 3), 3).fld0.0,Field::<Adt63>(Variant(_1, 3), 3).fld0.0,Field::<Adt63>(Variant(_1, 3), 3).fld0.0,Field::<Adt63>(Variant(_1, 3), 3).fld0.0,Field::<Adt63>(Variant(_1, 3), 3).fld0.0];
_5 = '\u{d52ec}';
_5 = '\u{6ad51}';
Goto(bb9)
}
bb17 = {
place!(Field::<Adt63>(Variant(_1, 3), 3)).fld4 = [23119_u16,30080_u16,29237_u16,54717_u16,29813_u16];
_2 = _13;
_5 = '\u{de500}';
place!(Field::<f32>(Variant(_1, 3), 1)) = RET as f32;
_22.3 = _24 < _6;
_14 = 153111535770935927993478807873273137029_i128 ^ (-32797238386109962909669829338041331651_i128);
place!(Field::<[i128; 5]>(Variant(_1, 3), 2)) = [_14,_14,_14,_14,_14];
_11 = !_4;
_27.0.0 = _22.2 >> _24;
_25.fld0 = core::ptr::addr_of!(_5);
_14 = -(-92593521107639945822281328406941673231_i128);
_2 = !_24;
RET = (-15494_i16) & (-18095_i16);
_4 = !_7;
_28.1 = _5;
_22.3 = _24 == _12;
_14 = 144729343606865380471904473628085127903_i128;
place!(Field::<Adt63>(Variant(_1, 3), 3)).fld2 = core::ptr::addr_of_mut!(_26);
_3 = [_25.fld1,_25.fld1,_25.fld1];
place!(Field::<Adt63>(Variant(_1, 3), 3)).fld3 = !_25.fld1;
RET = -5932_i16;
_14 = 135281047967227945544922370898049915421_i128 << _12;
Goto(bb18)
}
bb18 = {
Call(_31 = dump_var(18_usize, 13_usize, Move(_13), 5_usize, Move(_5), 21_usize, Move(_21), 14_usize, Move(_14)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_31 = dump_var(18_usize, 3_usize, Move(_3), 8_usize, Move(_8), 2_usize, Move(_2), 6_usize, Move(_6)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: f64,mut _2: f64,mut _3: f64,mut _4: f64,mut _5: i64) -> usize {
mir! {
type RET = usize;
let _6: (f32,);
let _7: &'static i16;
let _8: (f32,);
let _9: Adt24;
let _10: Adt82;
let _11: *const *const *mut u128;
let _12: Adt24;
let _13: Adt69;
let _14: i64;
let _15: f32;
let _16: f32;
let _17: [bool; 3];
let _18: i8;
let _19: char;
let _20: f32;
let _21: [i16; 1];
let _22: &'static *const u128;
let _23: (f64, f64, i64, bool);
let _24: char;
let _25: [u64; 5];
let _26: u8;
let _27: bool;
let _28: isize;
let _29: ();
let _30: ();
{
_5 = 92222849696299014793948835451612738635_i128 as i64;
RET = 17695483744201892311_usize & 4_usize;
RET = 2_usize >> _5;
_4 = _1;
RET = 4223233216_u32 as usize;
_4 = _3 * _3;
RET = 4_usize + 13505966179199323518_usize;
_1 = _2 + _3;
_6.0 = 8898625811479179404_u64 as f32;
_8.0 = (-9223372036854775808_isize) as f32;
_8.0 = _6.0 - _6.0;
_9.fld0 = (-70575447299715083288159204668743399619_i128) as u128;
_6.0 = _8.0;
_4 = _1;
_7 = &_9.fld4;
_9.fld2 = ['\u{b89f4}','\u{b3802}','\u{7e7a0}','\u{f3f7d}','\u{d4b92}','\u{e8cee}'];
RET = '\u{259ee}' as usize;
RET = !13552982645445027165_usize;
_9.fld1 = [false,false,true];
_2 = 13117996369243376849_u64 as f64;
_9.fld3 = 1871941274977547385_u64;
Goto(bb1)
}
bb1 = {
Goto(bb2)
}
bb2 = {
_3 = _1;
_14 = -_5;
_12.fld0 = '\u{10bcb0}' as u128;
match _9.fld3 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
1871941274977547385 => bb7,
_ => bb6
}
}
bb3 = {
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
_12.fld4 = !22290_i16;
RET = _5 as usize;
_4 = _1;
_12.fld4 = (-7989_i16);
RET = 4_usize;
_8.0 = _6.0 - _6.0;
_12.fld3 = _9.fld3 >> _12.fld0;
_3 = 148_u8 as f64;
RET = !2_usize;
_9.fld1 = [true,false,true];
_15 = _8.0;
_12.fld5 = 3923504706_u32 - 4200935858_u32;
_9.fld3 = _12.fld3 & _12.fld3;
_9.fld0 = _12.fld0 + _12.fld0;
_2 = _1 * _1;
_5 = _14 << _9.fld3;
_12.fld0 = _9.fld0;
_12.fld5 = 1784144795_u32 | 1161975037_u32;
_16 = _15;
_12 = Adt24 { fld0: _9.fld0,fld1: _9.fld1,fld2: _9.fld2,fld3: _9.fld3,fld4: 11951_i16,fld5: 962772912_u32 };
_16 = (-1278486160_i32) as f32;
_18 = 20_i8;
_12.fld1 = _9.fld1;
_7 = &(*_7);
_9.fld2 = ['\u{e0c26}','\u{a220d}','\u{f38b2}','\u{c8759}','\u{28a41}','\u{c116d}'];
Goto(bb8)
}
bb8 = {
_9.fld3 = _4 as u64;
_8.0 = _6.0;
_6 = (_15,);
_9.fld3 = !_12.fld3;
_12.fld4 = 63673_u16 as i16;
_8.0 = _9.fld3 as f32;
_19 = '\u{17cd0}';
_9.fld2 = [_19,_19,_19,_19,_19,_19];
_9.fld5 = !_12.fld5;
_9.fld0 = _12.fld0;
_9.fld4 = -_12.fld4;
_6 = _8;
_9 = Adt24 { fld0: _12.fld0,fld1: _12.fld1,fld2: _12.fld2,fld3: _12.fld3,fld4: _12.fld4,fld5: _12.fld5 };
_17 = [true,false,true];
_9.fld3 = _12.fld3;
_18 = _12.fld3 as i8;
_9.fld1 = [false,false,true];
_5 = _14 - _14;
_3 = _4;
Goto(bb9)
}
bb9 = {
_6.0 = -_15;
_19 = '\u{2ee4b}';
_18 = !85_i8;
_4 = _3;
_2 = _9.fld0 as f64;
_16 = RET as f32;
_9.fld1 = [false,false,false];
_12.fld3 = !_9.fld3;
_12.fld1 = [true,true,false];
_12.fld3 = _9.fld3;
_12.fld3 = _9.fld3;
_9.fld3 = _12.fld3;
_4 = _6.0 as f64;
_12.fld3 = _9.fld3 ^ _9.fld3;
_23 = (_3, _1, _5, false);
_3 = _1 * _1;
RET = 15711274731873876555_usize * 658752839796807391_usize;
_1 = RET as f64;
_23.1 = _3 * _3;
_12.fld3 = !_9.fld3;
_1 = _23.1;
match _9.fld5 {
0 => bb10,
1 => bb11,
2 => bb12,
962772912 => bb14,
_ => bb13
}
}
bb10 = {
Return()
}
bb11 = {
Goto(bb2)
}
bb12 = {
Goto(bb2)
}
bb13 = {
Return()
}
bb14 = {
_7 = &_9.fld4;
_4 = 114603465968689792290513064531405581845_i128 as f64;
RET = 4_usize ^ 5_usize;
_8.0 = _15;
_8 = (_15,);
_23 = (_3, _1, _5, false);
_12.fld5 = _9.fld5 % _9.fld5;
_19 = '\u{a33bb}';
_23 = (_1, _3, _14, false);
_12 = Adt24 { fld0: _9.fld0,fld1: _9.fld1,fld2: _9.fld2,fld3: _9.fld3,fld4: _9.fld4,fld5: _9.fld5 };
_23.0 = 154053394232298197983833429436120913015_i128 as f64;
_4 = _23.1 - _3;
_21 = [_12.fld4];
RET = 6_usize;
_5 = _14;
_9.fld0 = _12.fld0 + _12.fld0;
_24 = _19;
_9.fld4 = _9.fld5 as i16;
_9.fld1 = _12.fld1;
_17 = [_23.3,_23.3,_23.3];
_12.fld3 = _9.fld3 * _9.fld3;
_20 = _16;
_24 = _19;
_23.0 = _1 + _1;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(19_usize, 17_usize, Move(_17), 21_usize, Move(_21), 18_usize, Move(_18), 30_usize, _30), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box(7771979977442245139_u64), std::hint::black_box(9223372036854775807_isize), std::hint::black_box(1_i8), std::hint::black_box(11934_i16), std::hint::black_box(1777217014_u32), std::hint::black_box(157_u8), std::hint::black_box(31024174601277173761583425838805541059_u128), std::hint::black_box(6_usize));
                
            }
impl PrintFDebug for Adt24{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt24{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt24 {
fld0: u128,
fld1: [bool; 3],
fld2: [char; 6],
fld3: u64,
fld4: i16,
fld5: u32,
}
impl PrintFDebug for Adt32{
	unsafe fn printf_debug(&self){unsafe{printf("Adt32::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt32 {
Variant0{
fld0: ([u16; 5], *const i64),

},
Variant1{
fld0: *const i64,
fld1: *const char,
fld2: f64,
fld3: [bool; 3],
fld4: i16,
fld5: [char; 6],
fld6: u64,
fld7: i128,

}}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt40{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt40 {
fld0: *const char,
fld1: u64,
}
impl PrintFDebug for Adt63{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt63{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt63 {
fld0: (u16,),
fld1: Adt32,
fld2: *mut u128,
fld3: u64,
fld4: [u16; 5],
}
impl PrintFDebug for Adt65{
	unsafe fn printf_debug(&self){unsafe{printf("Adt65::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt65 {
Variant0{
fld0: i128,
fld1: u128,
fld2: [u16; 5],

},
Variant1{
fld0: i128,
fld1: *const ([u16; 5], *const i64),
fld2: [u64; 3],
fld3: *const i64,
fld4: [u8; 4],
fld5: ([u16; 5], *const i64),
fld6: Adt40,

},
Variant2{
fld0: *const [char; 6],
fld1: u128,
fld2: *const ([u16; 5], *const i64),
fld3: ([u16; 5], *const i64),

},
Variant3{
fld0: u8,
fld1: char,
fld2: (u16,),
fld3: [i16; 1],
fld4: *const u16,
fld5: Adt24,
fld6: *const char,

}}
impl PrintFDebug for Adt69{
	unsafe fn printf_debug(&self){unsafe{printf("Adt69::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt69 {
Variant0{
fld0: (usize, [char; 6], *const u32, u16),
fld1: f32,
fld2: u128,
fld3: Adt32,

},
Variant1{
fld0: i128,
fld1: u64,
fld2: *mut u128,

},
Variant2{
fld0: bool,
fld1: char,
fld2: [u64; 3],
fld3: usize,
fld4: u32,

},
Variant3{
fld0: [isize; 3],
fld1: f32,
fld2: [i128; 5],
fld3: Adt63,

}}
impl PrintFDebug for Adt81{
	unsafe fn printf_debug(&self){unsafe{printf("Adt81::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt81 {
Variant0{
fld0: [u8; 4],
fld1: Adt24,

},
Variant1{
fld0: Adt40,
fld1: [u8; 4],
fld2: [isize; 3],
fld3: [u16; 5],
fld4: u32,
fld5: *const Adt32,

},
Variant2{
fld0: *const [u16; 5],
fld1: u8,

},
Variant3{
fld0: *const u128,
fld1: [u64; 5],
fld2: [char; 8],
fld3: f32,

}}
impl PrintFDebug for Adt82{
	unsafe fn printf_debug(&self){unsafe{printf("Adt82::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt82 {
Variant0{
fld0: (usize, [char; 6], *const u32, u16),
fld1: [usize; 8],

},
Variant1{
fld0: f32,
fld1: *const [char; 6],
fld2: Adt81,
fld3: i8,
fld4: i16,
fld5: *const i32,
fld6: i64,
fld7: [usize; 8],

},
Variant2{
fld0: bool,
fld1: f32,
fld2: *const i64,
fld3: (*mut *const i64, i128, *const u128, ([u16; 5], *const i64)),
fld4: *mut u128,
fld5: [isize; 3],

},
Variant3{
fld0: *const char,
fld1: Adt69,
fld2: usize,
fld3: *const u32,
fld4: i16,
fld5: i32,
fld6: [u8; 4],

}}

