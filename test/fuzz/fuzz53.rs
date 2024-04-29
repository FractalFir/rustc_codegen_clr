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
pub fn fn0(mut _1: bool,mut _2: u16,mut _3: isize,mut _4: u32,mut _5: u128,mut _6: i32,mut _7: u64,mut _8: i128) -> (i64, i16) {
mir! {
type RET = (i64, i16);
let _9: i128;
let _10: i32;
let _11: Adt42;
let _12: [isize; 3];
let _13: [i128; 8];
let _14: i8;
let _15: (u32,);
let _16: [i128; 8];
let _17: u8;
let _18: f64;
let _19: f64;
let _20: bool;
let _21: i16;
let _22: ();
let _23: ();
{
RET.0 = 827461871042122374_i64;
RET.1 = 2397_i16 - 7495_i16;
_5 = 111454800366039941158495262203764788930_u128 | 267474784018000747380240953941091083527_u128;
_7 = 10478798096650576959_u64;
_2 = 16285_u16;
_4 = 1694698283_u32 & 1439030351_u32;
RET = ((-7322533270166674070_i64), (-20274_i16));
RET.0 = !(-565600354371179348_i64);
_1 = false;
RET.1 = _2 as i16;
_9 = _5 as i128;
_8 = _9 & _9;
Call(RET.0 = fn1(_8, _8, _5, _7, _5, _2, _1, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _5 as isize;
RET = ((-9185503017404064506_i64), 3054_i16);
RET.0 = 8901667606022845769_i64;
_2 = _4 as u16;
_4 = !1115343790_u32;
RET.0 = (-4216667710981068581_i64) + (-1537332333161918933_i64);
RET.0 = (-5215731600600869088_i64);
_4 = 1462679502_u32 | 1518424471_u32;
_6 = !(-259987810_i32);
Call(_10 = core::intrinsics::bswap(_6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET.0 = (-4030980017278985738_i64);
_12 = [_3,_3,_3];
_10 = !_6;
_6 = RET.1 as i32;
_12 = [_3,_3,_3];
_14 = _2 as i8;
_14 = _1 as i8;
_1 = false;
RET.0 = 4257592174197414010_i64 * 8735465082611777773_i64;
_8 = _9 >> _14;
_5 = !48036337814791410675375297749601446562_u128;
Call(_5 = core::intrinsics::transmute(_8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = !8020544603193900731_u64;
_10 = _6 >> _8;
RET = ((-4036252454004716922_i64), (-20471_i16));
RET.1 = (-8881_i16);
_6 = _10;
RET = (4708186741608833079_i64, 30729_i16);
match RET.0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
4708186741608833079 => bb11,
_ => bb10
}
}
bb4 = {
RET.0 = (-4030980017278985738_i64);
_12 = [_3,_3,_3];
_10 = !_6;
_6 = RET.1 as i32;
_12 = [_3,_3,_3];
_14 = _2 as i8;
_14 = _1 as i8;
_1 = false;
RET.0 = 4257592174197414010_i64 * 8735465082611777773_i64;
_8 = _9 >> _14;
_5 = !48036337814791410675375297749601446562_u128;
Call(_5 = core::intrinsics::transmute(_8), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_3 = _5 as isize;
RET = ((-9185503017404064506_i64), 3054_i16);
RET.0 = 8901667606022845769_i64;
_2 = _4 as u16;
_4 = !1115343790_u32;
RET.0 = (-4216667710981068581_i64) + (-1537332333161918933_i64);
RET.0 = (-5215731600600869088_i64);
_4 = 1462679502_u32 | 1518424471_u32;
_6 = !(-259987810_i32);
Call(_10 = core::intrinsics::bswap(_6), ReturnTo(bb2), UnwindUnreachable())
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
_4 = _5 as u32;
RET.0 = (-4897078282767302669_i64);
_2 = !61791_u16;
_11.fld0 = (_8,);
RET = ((-8544401345442599161_i64), 3881_i16);
RET.0 = !6257549624693338925_i64;
_12 = [_3,_3,_3];
_15 = (_4,);
_10 = !_6;
_13 = [_9,_11.fld0.0,_11.fld0.0,_9,_9,_8,_11.fld0.0,_11.fld0.0];
_13 = [_8,_11.fld0.0,_9,_8,_8,_8,_9,_11.fld0.0];
_9 = _11.fld0.0;
_3 = _2 as isize;
_3 = 9223372036854775807_isize;
_3 = 9223372036854775807_isize;
RET.0 = !7044516977219292977_i64;
_10 = _3 as i32;
_17 = !0_u8;
RET.1 = _3 as i16;
_17 = 201_u8 | 148_u8;
RET.0 = !1667275057300547398_i64;
_17 = 56_u8;
_16 = [_8,_11.fld0.0,_8,_9,_8,_11.fld0.0,_8,_8];
RET.0 = _5 as i64;
match _17 {
0 => bb8,
1 => bb3,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
56 => bb18,
_ => bb17
}
}
bb12 = {
Return()
}
bb13 = {
RET.0 = (-4030980017278985738_i64);
_12 = [_3,_3,_3];
_10 = !_6;
_6 = RET.1 as i32;
_12 = [_3,_3,_3];
_14 = _2 as i8;
_14 = _1 as i8;
_1 = false;
RET.0 = 4257592174197414010_i64 * 8735465082611777773_i64;
_8 = _9 >> _14;
_5 = !48036337814791410675375297749601446562_u128;
Call(_5 = core::intrinsics::transmute(_8), ReturnTo(bb3), UnwindUnreachable())
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
RET.0 = (-4030980017278985738_i64);
_12 = [_3,_3,_3];
_10 = !_6;
_6 = RET.1 as i32;
_12 = [_3,_3,_3];
_14 = _2 as i8;
_14 = _1 as i8;
_1 = false;
RET.0 = 4257592174197414010_i64 * 8735465082611777773_i64;
_8 = _9 >> _14;
_5 = !48036337814791410675375297749601446562_u128;
Call(_5 = core::intrinsics::transmute(_8), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_3 = _5 as isize;
RET = ((-9185503017404064506_i64), 3054_i16);
RET.0 = 8901667606022845769_i64;
_2 = _4 as u16;
_4 = !1115343790_u32;
RET.0 = (-4216667710981068581_i64) + (-1537332333161918933_i64);
RET.0 = (-5215731600600869088_i64);
_4 = 1462679502_u32 | 1518424471_u32;
_6 = !(-259987810_i32);
Call(_10 = core::intrinsics::bswap(_6), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
_5 = 233837415745727945104247931952059118420_u128 + 308530506524216337520511160037327576963_u128;
_15 = (_4,);
_17 = !138_u8;
_4 = _15.0 & _15.0;
RET.0 = _11.fld0.0 as i64;
_6 = -_10;
_8 = -_11.fld0.0;
_1 = _11.fld0.0 != _11.fld0.0;
_15 = (_4,);
_4 = !_15.0;
_15.0 = _4;
RET.0 = (-5592777139688241129_i64) | (-3493924334829394478_i64);
Goto(bb19)
}
bb19 = {
Call(_22 = dump_var(0_usize, 9_usize, Move(_9), 8_usize, Move(_8), 10_usize, Move(_10), 4_usize, Move(_4)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_22 = dump_var(0_usize, 13_usize, Move(_13), 3_usize, Move(_3), 7_usize, Move(_7), 12_usize, Move(_12)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i128,mut _2: i128,mut _3: u128,mut _4: u64,mut _5: u128,mut _6: u16,mut _7: bool,mut _8: i128) -> i64 {
mir! {
type RET = i64;
let _9: usize;
let _10: f32;
let _11: usize;
let _12: [isize; 3];
let _13: bool;
let _14: Adt47;
let _15: [i64; 6];
let _16: u64;
let _17: bool;
let _18: Adt53;
let _19: [isize; 3];
let _20: f32;
let _21: u16;
let _22: [i128; 8];
let _23: (i128,);
let _24: ();
let _25: ();
{
RET = -8911397172048126568_i64;
_3 = !_5;
RET = 1054228772690331529_i64;
_3 = 15593_i16 as u128;
RET = _4 as i64;
_3 = _5;
_5 = !_3;
_6 = !9411_u16;
RET = -(-1441357472029124384_i64);
_2 = _7 as i128;
_5 = !_3;
Call(RET = core::intrinsics::transmute(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = 4332627691216619858_u64 >> _1;
_6 = 53281_u16 * 11812_u16;
_6 = 138_u8 as u16;
_6 = !35467_u16;
Call(_6 = fn2(_8, _1, _5, _4, _1, _1, _1, _5, _3, _4, _7, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = _1;
_2 = !_1;
match _6 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
17555 => bb10,
_ => bb9
}
}
bb3 = {
_4 = 4332627691216619858_u64 >> _1;
_6 = 53281_u16 * 11812_u16;
_6 = 138_u8 as u16;
_6 = !35467_u16;
Call(_6 = fn2(_8, _1, _5, _4, _1, _1, _1, _5, _3, _4, _7, _7), ReturnTo(bb2), UnwindUnreachable())
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
_6 = 59581_u16;
RET = 215_u8 as i64;
_9 = 16847382031374881764_usize * 731411852406128248_usize;
RET = -(-8260034498364342288_i64);
RET = (-1588172759_i32) as i64;
_8 = _1 * _2;
_4 = '\u{d698d}' as u64;
_13 = _8 == _8;
_8 = -_2;
_11 = !_9;
RET = 4382056716824554430_i64 << _8;
_6 = _4 as u16;
_12 = [(-9223372036854775808_isize),(-9223372036854775808_isize),47_isize];
_3 = _6 as u128;
_6 = 17399_u16;
_1 = _2 | _2;
match _6 {
0 => bb1,
1 => bb2,
2 => bb3,
17399 => bb11,
_ => bb8
}
}
bb11 = {
_16 = _4 ^ _4;
_17 = _1 <= _8;
_9 = _11;
_6 = 64540_u16;
_11 = _9;
_11 = !_9;
_16 = !_4;
_1 = _2;
RET = 9026049867717018881_i64;
_6 = !17870_u16;
_4 = _16 + _16;
_7 = _13;
_8 = !_2;
Call(_1 = fn4(_11, _13, _13, _13, _7, _11, _13, _7, _7, _13, _7, _12, RET, _5, _8, _17), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_9 = _11 << _1;
_19 = [(-9223372036854775808_isize),48_isize,0_isize];
_15 = [RET,RET,RET,RET,RET,RET];
_9 = _11 * _11;
_1 = -_2;
Goto(bb13)
}
bb13 = {
_2 = _5 as i128;
_12 = _19;
RET = -7217766838810080735_i64;
_10 = RET as f32;
_19 = [(-28_isize),93_isize,116_isize];
_4 = _16;
RET = (-6808041989115791700_i64) | 19930982139649922_i64;
RET = 9058012332576742934_i64;
_17 = _13;
_19 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-60_isize)];
_5 = !_3;
match RET {
0 => bb12,
1 => bb4,
9058012332576742934 => bb14,
_ => bb3
}
}
bb14 = {
_8 = -_1;
_1 = _16 as i128;
_17 = !_13;
_20 = _9 as f32;
_6 = !39416_u16;
_4 = _16 * _16;
_21 = RET as u16;
_11 = _9 ^ _9;
_23 = (_8,);
_19 = [(-9223372036854775808_isize),36_isize,(-64_isize)];
_23 = (_8,);
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(1_usize, 3_usize, Move(_3), 13_usize, Move(_13), 12_usize, Move(_12), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(1_usize, 19_usize, Move(_19), 8_usize, Move(_8), 15_usize, Move(_15), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_24 = dump_var(1_usize, 2_usize, Move(_2), 25_usize, _25, 25_usize, _25, 25_usize, _25), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: i128,mut _2: i128,mut _3: u128,mut _4: u64,mut _5: i128,mut _6: i128,mut _7: i128,mut _8: u128,mut _9: u128,mut _10: u64,mut _11: bool,mut _12: bool) -> u16 {
mir! {
type RET = u16;
let _13: [i128; 8];
let _14: char;
let _15: (u32,);
let _16: (i128,);
let _17: isize;
let _18: bool;
let _19: Adt48;
let _20: [i64; 6];
let _21: *mut *mut bool;
let _22: *const u64;
let _23: [i128; 8];
let _24: *mut f64;
let _25: [i64; 6];
let _26: [i128; 8];
let _27: (i128,);
let _28: char;
let _29: (u16,);
let _30: Adt42;
let _31: bool;
let _32: ();
let _33: ();
{
_1 = !_7;
_4 = _10;
_10 = (-124_i8) as u64;
RET = !61488_u16;
_4 = RET as u64;
_13 = [_2,_1,_7,_1,_5,_6,_5,_2];
RET = 0_u8 as u16;
_4 = 2558460015197062408_usize as u64;
_14 = '\u{b9d0d}';
_3 = _8 | _9;
_2 = -_7;
_10 = _4 << _2;
_16 = (_7,);
_12 = _1 == _2;
_15 = (970901219_u32,);
_9 = _8 | _3;
_15.0 = 377258608_u32 << RET;
_10 = _3 as u64;
_13 = [_7,_1,_2,_6,_16.0,_16.0,_6,_16.0];
_4 = _10 ^ _10;
_6 = _2;
RET = 13675_u16 << _5;
_17 = 9223372036854775807_isize;
_16 = (_6,);
Call(_5 = fn3(_8, _8, _13, _4, _16.0, _9, RET, _12, _16, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = !_12;
_18 = _11;
_16.0 = _6 | _5;
_14 = '\u{8d7c7}';
_3 = _8 | _8;
RET = !41809_u16;
_1 = _3 as i128;
Goto(bb2)
}
bb2 = {
_17 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_5 = _16.0;
_8 = _3 * _9;
_16 = (_2,);
_9 = 3_usize as u128;
_4 = 5_u8 as u64;
_9 = _3 + _3;
_3 = _8;
_22 = core::ptr::addr_of!(_4);
_13 = [_5,_5,_2,_5,_5,_5,_5,_5];
RET = 17555_u16;
_6 = _2;
_2 = RET as i128;
_20 = [(-2306469748777409493_i64),(-2622087911728220785_i64),4798869617429120387_i64,8665368792117004830_i64,4856762033855528623_i64,7860230306869701450_i64];
match RET {
17555 => bb3,
_ => bb1
}
}
bb3 = {
_11 = !_18;
_5 = _6;
_9 = (-15725_i16) as u128;
_15.0 = 82323557_u32 | 853412510_u32;
_13 = [_1,_5,_16.0,_16.0,_6,_16.0,_16.0,_6];
_14 = '\u{79b59}';
_16 = (_1,);
_9 = _8 + _8;
_6 = _5 - _5;
match RET {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
17555 => bb11,
_ => bb10
}
}
bb4 = {
_17 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_5 = _16.0;
_8 = _3 * _9;
_16 = (_2,);
_9 = 3_usize as u128;
_4 = 5_u8 as u64;
_9 = _3 + _3;
_3 = _8;
_22 = core::ptr::addr_of!(_4);
_13 = [_5,_5,_2,_5,_5,_5,_5,_5];
RET = 17555_u16;
_6 = _2;
_2 = RET as i128;
_20 = [(-2306469748777409493_i64),(-2622087911728220785_i64),4798869617429120387_i64,8665368792117004830_i64,4856762033855528623_i64,7860230306869701450_i64];
match RET {
17555 => bb3,
_ => bb1
}
}
bb5 = {
_11 = !_12;
_18 = _11;
_16.0 = _6 | _5;
_14 = '\u{8d7c7}';
_3 = _8 | _8;
RET = !41809_u16;
_1 = _3 as i128;
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
_14 = '\u{f77f1}';
_28 = _14;
(*_22) = 6943880935451080197_usize as u64;
_30 = Adt42 { fld0: _16 };
_29.0 = RET;
_26 = [_30.fld0.0,_16.0,_16.0,_1,_7,_16.0,_16.0,_30.fld0.0];
_26 = _13;
_1 = _16.0 << _3;
_22 = core::ptr::addr_of!((*_22));
Goto(bb12)
}
bb12 = {
_27.0 = _1 * _1;
_29 = (RET,);
match _29.0 {
0 => bb13,
1 => bb14,
17555 => bb16,
_ => bb15
}
}
bb13 = {
_14 = '\u{f77f1}';
_28 = _14;
(*_22) = 6943880935451080197_usize as u64;
_30 = Adt42 { fld0: _16 };
_29.0 = RET;
_26 = [_30.fld0.0,_16.0,_16.0,_1,_7,_16.0,_16.0,_30.fld0.0];
_26 = _13;
_1 = _16.0 << _3;
_22 = core::ptr::addr_of!((*_22));
Goto(bb12)
}
bb14 = {
Return()
}
bb15 = {
_17 = (-9223372036854775808_isize) & 9223372036854775807_isize;
_5 = _16.0;
_8 = _3 * _9;
_16 = (_2,);
_9 = 3_usize as u128;
_4 = 5_u8 as u64;
_9 = _3 + _3;
_3 = _8;
_22 = core::ptr::addr_of!(_4);
_13 = [_5,_5,_2,_5,_5,_5,_5,_5];
RET = 17555_u16;
_6 = _2;
_2 = RET as i128;
_20 = [(-2306469748777409493_i64),(-2622087911728220785_i64),4798869617429120387_i64,8665368792117004830_i64,4856762033855528623_i64,7860230306869701450_i64];
match RET {
17555 => bb3,
_ => bb1
}
}
bb16 = {
_15 = (1032853004_u32,);
_28 = _14;
Goto(bb17)
}
bb17 = {
Call(_32 = dump_var(2_usize, 28_usize, Move(_28), 6_usize, Move(_6), 5_usize, Move(_5), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_32 = dump_var(2_usize, 1_usize, Move(_1), 8_usize, Move(_8), 7_usize, Move(_7), 2_usize, Move(_2)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_32 = dump_var(2_usize, 9_usize, Move(_9), 10_usize, Move(_10), 12_usize, Move(_12), 33_usize, _33), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: u128,mut _2: u128,mut _3: [i128; 8],mut _4: u64,mut _5: i128,mut _6: u128,mut _7: u16,mut _8: bool,mut _9: (i128,),mut _10: i128) -> i128 {
mir! {
type RET = i128;
let _11: [i128; 8];
let _12: ();
let _13: ();
{
_2 = _6;
_3 = [_5,_5,_10,_9.0,_10,_9.0,_10,_9.0];
_5 = !_9.0;
_3 = [_9.0,_10,_5,_10,_5,_5,_9.0,_9.0];
_3 = [_5,_9.0,_5,_9.0,_10,_9.0,_5,_9.0];
_6 = _1 ^ _2;
_6 = _1;
RET = _5 * _9.0;
_9 = (RET,);
_6 = '\u{2c881}' as u128;
_5 = _9.0 | _9.0;
Goto(bb1)
}
bb1 = {
RET = _5 >> _4;
_2 = _1 ^ _6;
_11 = [RET,_5,_5,_5,RET,RET,_9.0,RET];
_3 = [_5,RET,_5,_10,RET,_5,RET,_9.0];
_7 = 3843_u16 - 38194_u16;
_5 = RET;
_5 = _2 as i128;
_5 = RET | RET;
_2 = (-8028639400165961132_i64) as u128;
_10 = (-107_i8) as i128;
RET = -_5;
Goto(bb2)
}
bb2 = {
Call(_12 = dump_var(3_usize, 5_usize, Move(_5), 9_usize, Move(_9), 2_usize, Move(_2), 1_usize, Move(_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_12 = dump_var(3_usize, 4_usize, Move(_4), 13_usize, _13, 13_usize, _13, 13_usize, _13), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: usize,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: usize,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: bool,mut _12: [isize; 3],mut _13: i64,mut _14: u128,mut _15: i128,mut _16: bool) -> i128 {
mir! {
type RET = i128;
let _17: [isize; 3];
let _18: f32;
let _19: isize;
let _20: [i128; 8];
let _21: Adt42;
let _22: *const i16;
let _23: [i128; 8];
let _24: f32;
let _25: ();
let _26: ();
{
_5 = _4;
_5 = !_16;
_1 = _6 * _6;
_3 = _16 >= _5;
_4 = _11 & _3;
_5 = _9;
_5 = _8 < _11;
RET = _15 * _15;
_16 = _11 & _10;
_10 = !_2;
_15 = 77_i8 as i128;
_2 = !_16;
_17 = [9223372036854775807_isize,124_isize,(-9223372036854775808_isize)];
_10 = _3;
_5 = _9;
RET = _15 << _14;
RET = 1295101026_u32 as i128;
_10 = !_16;
_16 = !_4;
_6 = !_1;
_19 = 8313325946668489588_u64 as isize;
Goto(bb1)
}
bb1 = {
_1 = _6 ^ _6;
Goto(bb2)
}
bb2 = {
_3 = _4 ^ _4;
_2 = _9;
_11 = !_4;
_15 = _2 as i128;
_3 = _16;
_2 = _8 != _11;
_20 = [_15,_15,_15,_15,_15,_15,_15,_15];
_11 = !_2;
_5 = !_9;
_10 = _4;
_13 = _1 as i64;
_2 = !_3;
_21.fld0.0 = _15 - _15;
RET = -_21.fld0.0;
RET = _21.fld0.0;
_2 = !_16;
_4 = _11 ^ _16;
_21.fld0 = (_15,);
_3 = _8;
_20 = [_21.fld0.0,_21.fld0.0,_15,RET,RET,_15,_21.fld0.0,RET];
Goto(bb3)
}
bb3 = {
Call(_25 = dump_var(4_usize, 16_usize, Move(_16), 20_usize, Move(_20), 17_usize, Move(_17), 7_usize, Move(_7)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_25 = dump_var(4_usize, 13_usize, Move(_13), 15_usize, Move(_15), 14_usize, Move(_14), 1_usize, Move(_1)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_25 = dump_var(4_usize, 6_usize, Move(_6), 26_usize, _26, 26_usize, _26, 26_usize, _26), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box(42653_u16), std::hint::black_box(72_isize), std::hint::black_box(1862890124_u32), std::hint::black_box(51334631355229937747235514300416418207_u128), std::hint::black_box((-1756141160_i32)), std::hint::black_box(16745231251890685640_u64), std::hint::black_box((-47132960849195368919621315339992637189_i128)));
                
            }
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf("Adt41::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: *mut bool,
fld1: [i64; 6],

},
Variant1{
fld0: (i16, *mut bool, *mut bool, u8),
fld1: *const *const i16,
fld2: (*mut (i64, i16), i64, *mut bool, *const *mut (i64, i16)),
fld3: usize,
fld4: i16,
fld5: u8,
fld6: *const u64,

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt42{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt42 {
fld0: (i128,),
}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: *const u32,
fld1: char,
fld2: usize,
fld3: *const *mut (i64, i16),
fld4: (i64, i16),

},
Variant1{
fld0: *mut bool,

},
Variant2{
fld0: u128,

}}
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: u64,
fld1: u8,
fld2: *const u64,
fld3: i8,
fld4: *const i16,

},
Variant1{
fld0: *mut (i64, i16),
fld1: char,
fld2: Adt41,
fld3: f64,
fld4: (i64, i16),
fld5: f32,
fld6: *const *const i16,

},
Variant2{
fld0: *mut (i64, i16),
fld1: f64,
fld2: *mut bool,
fld3: f32,
fld4: Adt43,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: bool,
fld1: Adt43,
fld2: *const i16,
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: Adt42,
fld1: char,
fld2: *const bool,
fld3: (i128,),
fld4: Adt43,
fld5: (*mut (i64, i16), i64, *mut bool, *const *mut (i64, i16)),
fld6: *mut (u16,),

},
Variant1{
fld0: f32,
fld1: *const u32,
fld2: isize,
fld3: i8,
fld4: u64,
fld5: i32,
fld6: Adt41,

},
Variant2{
fld0: Adt43,
fld1: char,
fld2: *mut (i64, i16),
fld3: *mut f64,

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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: bool,
fld1: *mut i128,
fld2: (i64, i16),
fld3: f64,
fld4: (*mut (i64, i16), i64, *mut bool, *const *mut (i64, i16)),

},
Variant1{
fld0: Adt41,
fld1: *mut bool,
fld2: isize,
fld3: u8,
fld4: u128,
fld5: *const *mut (i64, i16),

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: *mut *mut bool,
fld1: *mut (i64, i16),
fld2: *const u64,
fld3: (u16,),
fld4: *mut bool,
fld5: i128,

},
Variant1{
fld0: *mut bool,
fld1: (*mut (i64, i16), i64, *mut bool, *const *mut (i64, i16)),
fld2: isize,
fld3: *const u64,
fld4: (u16,),

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: (*mut (i64, i16), i64, *mut bool, *const *mut (i64, i16)),
fld1: u16,
fld2: Adt43,
fld3: *mut (u16,),
fld4: *const u64,
fld5: i32,
fld6: i64,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: (u32,),
fld1: *mut (i64, i16),
fld2: (u16,),
fld3: Adt44,
fld4: i16,
fld5: *const u64,
fld6: i64,
fld7: Adt41,

},
Variant1{
fld0: *mut *mut bool,
fld1: *const *const i16,

},
Variant2{
fld0: *mut bool,
fld1: u8,
fld2: (i16, *mut bool, *mut bool, u8),
fld3: Adt41,
fld4: i16,
fld5: i32,
fld6: Adt49,

},
Variant3{
fld0: u64,
fld1: char,
fld2: Adt41,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: Adt43,
fld1: *const bool,
fld2: [i128; 8],

},
Variant1{
fld0: [i64; 6],
fld1: *const *mut (i64, i16),
fld2: *mut *mut bool,
fld3: *const *const i16,
fld4: f64,
fld5: Adt48,

},
Variant2{
fld0: (*const u32, usize),
fld1: usize,
fld2: Adt49,
fld3: *const bool,
fld4: *const *mut (i64, i16),

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
fld0: Adt43,
fld1: (i64, i16),
fld2: i32,

},
Variant1{
fld0: Adt46,
fld1: *mut *mut bool,
fld2: (i16, *mut bool, *mut bool, u8),
fld3: [i128; 8],
fld4: (*mut (i64, i16), i64, *mut bool, *const *mut (i64, i16)),
fld5: i32,
fld6: (u32,),

},
Variant2{
fld0: *const *mut (i64, i16),
fld1: *mut f64,
fld2: [i128; 8],
fld3: (i64, i16),
fld4: usize,
fld5: u8,
fld6: Adt49,
fld7: (u16,),

}}
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: (u16,),
fld1: u128,
fld2: *mut bool,
fld3: *mut i128,
fld4: *const bool,

},
Variant1{
fld0: *mut (i64, i16),
fld1: (u16,),
fld2: f32,

},
Variant2{
fld0: bool,
fld1: f32,
fld2: (*mut (i64, i16), i64, *mut bool, *const *mut (i64, i16)),
fld3: (i64, i16),
fld4: (i16, *mut bool, *mut bool, u8),
fld5: Adt43,
fld6: *const u32,

},
Variant3{
fld0: Adt44,
fld1: *const bool,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
fld0: *const bool,

},
Variant1{
fld0: Adt52,
fld1: *mut f64,
fld2: *mut (i64, i16),
fld3: Adt50,
fld4: u32,

},
Variant2{
fld0: i128,
fld1: *const u32,

},
Variant3{
fld0: bool,
fld1: char,
fld2: Adt49,
fld3: *mut (i64, i16),
fld4: (i128,),
fld5: *mut f64,
fld6: Adt51,
fld7: i128,

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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: u8,
fld1: f32,
fld2: [i128; 8],
fld3: (i128,),
fld4: u128,
fld5: *const bool,
fld6: Adt45,
fld7: (*const u32, usize),

},
Variant1{
fld0: *mut (i64, i16),
fld1: char,
fld2: (*const u32, usize),
fld3: Adt53,

},
Variant2{
fld0: bool,
fld1: char,
fld2: (u32,),
fld3: u16,
fld4: Adt54,
fld5: (i64, i16),
fld6: *const bool,
fld7: Adt46,

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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: *mut (i64, i16),
fld1: char,
fld2: Adt46,

},
Variant1{
fld0: *mut (u16,),
fld1: u8,
fld2: (*const u32, usize),
fld3: u128,
fld4: i16,
fld5: f64,
fld6: (i16, *mut bool, *mut bool, u8),

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: u32,
fld1: Adt52,
fld2: [i64; 6],
fld3: Adt43,
fld4: Adt48,

},
Variant1{
fld0: *const u32,
fld1: *mut (i64, i16),
fld2: Adt53,
fld3: u8,
fld4: (i16, *mut bool, *mut bool, u8),
fld5: i32,
fld6: *mut f64,

}}

