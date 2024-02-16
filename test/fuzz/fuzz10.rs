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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> i16 {
mir! {
type RET = i16;
let _15: i16;
let _16: [i64; 3];
let _17: [bool; 3];
let _18: u16;
let _19: isize;
let _20: isize;
let _21: [bool; 5];
let _22: [bool; 3];
let _23: Adt60;
let _24: *const ([usize; 4],);
let _25: isize;
let _26: [u32; 7];
let _27: f64;
let _28: bool;
let _29: [usize; 4];
let _30: Adt47;
let _31: [i16; 7];
let _32: i64;
let _33: u32;
let _34: Adt50;
let _35: Adt50;
let _36: isize;
let _37: isize;
let _38: isize;
let _39: [u16; 6];
let _40: u16;
let _41: ();
let _42: ();
{
_2 = '\u{77707}';
_10 = 12221816496610605980_u64 as u8;
RET = (-21984_i16) * 10172_i16;
_13 = (-353981918_i32) as u64;
_11 = 31740_u16 - 64359_u16;
_3 = 9223372036854775807_isize;
_1 = _10 > _10;
Goto(bb1)
}
bb1 = {
_14 = 86880066377479544592937493296808690362_u128 | 304540224975124103792368387492299345906_u128;
_6 = _11 as i32;
_7 = (-3797382232615952609_i64) | (-7852128771379882526_i64);
_5 = RET << _11;
_14 = 1467491568380923521282995924574907913_u128;
_15 = _5;
_16 = [_7,_7,_7];
_19 = _3 + _3;
_14 = 58457661368257548706429646437129559157_u128;
_4 = 6_usize as i8;
_20 = !_19;
_3 = !_20;
_1 = true;
_7 = 14723045104235600222_usize as i64;
_19 = 1515649264_u32 as isize;
_8 = -164942442258267664544255968987879013723_i128;
_9 = !2_usize;
_16 = [_7,_7,_7];
_16 = [_7,_7,_7];
_16 = [_7,_7,_7];
_19 = -_20;
Call(_3 = fn1(_8, _15, _13, _11, _16, _14, _20, _20, _19, _5, _20, _19, _9, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = _15;
_10 = 25_u8 | 204_u8;
_6 = (-275639938_i32) | (-158969516_i32);
_17 = [_1,_1,_1];
_18 = !_11;
_22 = [_1,_1,_1];
_18 = _6 as u16;
match _14 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
58457661368257548706429646437129559157 => bb8,
_ => bb7
}
}
bb3 = {
_14 = 86880066377479544592937493296808690362_u128 | 304540224975124103792368387492299345906_u128;
_6 = _11 as i32;
_7 = (-3797382232615952609_i64) | (-7852128771379882526_i64);
_5 = RET << _11;
_14 = 1467491568380923521282995924574907913_u128;
_15 = _5;
_16 = [_7,_7,_7];
_19 = _3 + _3;
_14 = 58457661368257548706429646437129559157_u128;
_4 = 6_usize as i8;
_20 = !_19;
_3 = !_20;
_1 = true;
_7 = 14723045104235600222_usize as i64;
_19 = 1515649264_u32 as isize;
_8 = -164942442258267664544255968987879013723_i128;
_9 = !2_usize;
_16 = [_7,_7,_7];
_16 = [_7,_7,_7];
_16 = [_7,_7,_7];
_19 = -_20;
Call(_3 = fn1(_8, _15, _13, _11, _16, _14, _20, _20, _19, _5, _20, _19, _9, _1), ReturnTo(bb2), UnwindUnreachable())
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
_18 = _11;
_3 = _13 as isize;
RET = _20 as i16;
RET = _4 as i16;
Goto(bb9)
}
bb9 = {
_22 = [_1,_1,_1];
_14 = 209128600243981224628244713241019576013_u128 | 230397243714425582510450633487877822076_u128;
_12 = 3507802341_u32;
_5 = _15;
_18 = _1 as u16;
_12 = 1906376523_u32 + 2113194016_u32;
_6 = -(-2064090059_i32);
_1 = true;
_21 = [_1,_1,_1,_1,_1];
Goto(bb10)
}
bb10 = {
RET = _10 as i16;
_18 = _14 as u16;
_9 = 6_usize;
_21 = [_1,_1,_1,_1,_1];
_21 = [_1,_1,_1,_1,_1];
_31[_9] = _5 >> _11;
_26[_9] = _12;
_15 = _26[_9] as i16;
_22 = _17;
_26[_9] = !_12;
_36 = _20 - _20;
_1 = !false;
_27 = _13 as f64;
match _9 {
6 => bb12,
_ => bb11
}
}
bb11 = {
_22 = [_1,_1,_1];
_14 = 209128600243981224628244713241019576013_u128 | 230397243714425582510450633487877822076_u128;
_12 = 3507802341_u32;
_5 = _15;
_18 = _1 as u16;
_12 = 1906376523_u32 + 2113194016_u32;
_6 = -(-2064090059_i32);
_1 = true;
_21 = [_1,_1,_1,_1,_1];
Goto(bb10)
}
bb12 = {
_32 = _6 as i64;
_26 = [_12,_12,_12,_12,_12,_12,_12];
_16 = [_32,_7,_7];
_17 = [_1,_1,_1];
_11 = _18;
_6 = _9 as i32;
_14 = !223774609150302519489238261748430505084_u128;
_29 = [_9,_9,_9,_9];
match _9 {
0 => bb9,
6 => bb14,
_ => bb13
}
}
bb13 = {
_14 = 86880066377479544592937493296808690362_u128 | 304540224975124103792368387492299345906_u128;
_6 = _11 as i32;
_7 = (-3797382232615952609_i64) | (-7852128771379882526_i64);
_5 = RET << _11;
_14 = 1467491568380923521282995924574907913_u128;
_15 = _5;
_16 = [_7,_7,_7];
_19 = _3 + _3;
_14 = 58457661368257548706429646437129559157_u128;
_4 = 6_usize as i8;
_20 = !_19;
_3 = !_20;
_1 = true;
_7 = 14723045104235600222_usize as i64;
_19 = 1515649264_u32 as isize;
_8 = -164942442258267664544255968987879013723_i128;
_9 = !2_usize;
_16 = [_7,_7,_7];
_16 = [_7,_7,_7];
_16 = [_7,_7,_7];
_19 = -_20;
Call(_3 = fn1(_8, _15, _13, _11, _16, _14, _20, _20, _19, _5, _20, _19, _9, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
_32 = _7 + _7;
_22 = _17;
_7 = _32 + _32;
_6 = (-1451985497_i32);
_37 = _19 | _19;
_3 = _37;
_38 = _36 & _3;
_25 = _2 as isize;
_8 = !69263676875911857488901080204308235943_i128;
_2 = '\u{dd631}';
_32 = _7 * _7;
_13 = !7984140260981080610_u64;
_38 = !_3;
_2 = '\u{10fcfc}';
_10 = _1 as u8;
_40 = _18 ^ _18;
_39 = [_11,_40,_40,_40,_40,_18];
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(0_usize, 39_usize, Move(_39), 25_usize, Move(_25), 6_usize, Move(_6), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(0_usize, 21_usize, Move(_21), 9_usize, Move(_9), 14_usize, Move(_14), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(0_usize, 36_usize, Move(_36), 5_usize, Move(_5), 32_usize, Move(_32), 16_usize, Move(_16)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(0_usize, 8_usize, Move(_8), 4_usize, Move(_4), 1_usize, Move(_1), 42_usize, _42), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i128,mut _2: i16,mut _3: u64,mut _4: u16,mut _5: [i64; 3],mut _6: u128,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: i16,mut _11: isize,mut _12: isize,mut _13: usize,mut _14: bool) -> isize {
mir! {
type RET = isize;
let _15: Adt57;
let _16: [i64; 3];
let _17: [u16; 4];
let _18: Adt57;
let _19: f32;
let _20: i8;
let _21: i16;
let _22: f32;
let _23: bool;
let _24: bool;
let _25: ([u16; 4], u128, u8, f32);
let _26: [i8; 3];
let _27: f32;
let _28: Adt50;
let _29: ([u16; 4], u128, u8, f32);
let _30: [i16; 1];
let _31: ([usize; 4],);
let _32: u32;
let _33: [i64; 3];
let _34: ([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4]);
let _35: char;
let _36: i128;
let _37: [u16; 6];
let _38: [i64; 3];
let _39: [bool; 3];
let _40: i8;
let _41: *const usize;
let _42: Adt61;
let _43: ([usize; 4],);
let _44: ();
let _45: ();
{
_1 = 133810108674709598456320536881179958210_i128 | 131729098414843004319809999003291774129_i128;
RET = _8 + _11;
_13 = _14 as usize;
_11 = -RET;
_6 = !212655256671722558709624582014132229839_u128;
_16 = [(-4257131690975071746_i64),7049212019044343415_i64,33139215343130590_i64];
_5 = _16;
RET = !_12;
RET = _12 + _11;
_11 = !_9;
_3 = 4265617574_u32 as u64;
_6 = 201533433926988188052499373067823828476_u128 - 308744827548223181311700032235141390409_u128;
_14 = true | true;
_19 = _13 as f32;
_17 = [_4,_4,_4,_4];
_3 = 8184140266841275323_u64 - 17564216789331268414_u64;
_19 = RET as f32;
_7 = !_11;
_10 = _2;
_16 = [(-5444054468444280996_i64),7775633488961018760_i64,8304741133915614030_i64];
_7 = !_9;
_13 = 0_usize & 8742648572927473168_usize;
Goto(bb1)
}
bb1 = {
_14 = !false;
_22 = _4 as f32;
_16 = [5979747191720963158_i64,289017699713762973_i64,(-693093592916459944_i64)];
_20 = !39_i8;
_25.1 = _6 - _6;
_23 = _14 ^ _14;
_7 = _1 as isize;
_26 = [_20,_20,_20];
_24 = !_23;
_14 = !_24;
_25.0 = _17;
_25.2 = 173_u8;
_26 = [_20,_20,_20];
_21 = _14 as i16;
_6 = !_25.1;
_16 = [5666087957716067475_i64,855410560898404323_i64,(-1561785154782584786_i64)];
_5 = [(-8495615176895399825_i64),1950772832764414810_i64,(-8466550921867540405_i64)];
_10 = _3 as i16;
_6 = _14 as u128;
_27 = _19;
_12 = _8;
Call(_25.2 = fn2(RET, _1, _2, _12, _27), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_14 = _23 & _24;
_8 = _9 | RET;
_17 = [_4,_4,_4,_4];
_9 = -_12;
_25.3 = _3 as f32;
_3 = 13274107013846230738_u64;
_16 = [(-8279989496600720556_i64),5855605653982203677_i64,6122390933000767418_i64];
_5 = _16;
_7 = _12;
_26 = [_20,_20,_20];
RET = _9 ^ _7;
_29 = (_25.0, _25.1, _25.2, _25.3);
_10 = _2 & _21;
Goto(bb3)
}
bb3 = {
_9 = _4 as isize;
_12 = RET;
_8 = !RET;
_3 = 10023566641149673831_u64;
_3 = 1952789949162616102_u64;
_30 = [_10];
_29.0 = _17;
_23 = !_24;
_1 = 22655776209867881111342155940170853141_i128 | 16385774721183328110396789830230224363_i128;
_26 = [_20,_20,_20];
RET = _9;
Call(RET = core::intrinsics::bswap(_8), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_2 = _10 >> _29.1;
_11 = _8 - _7;
_32 = _4 as u32;
_4 = 1972_u16 * 36001_u16;
_10 = -_2;
_20 = 1326698554_i32 as i8;
_32 = 4276061007_u32 + 3316965000_u32;
Goto(bb5)
}
bb5 = {
_16 = _5;
_20 = (-95_i8) - (-86_i8);
Goto(bb6)
}
bb6 = {
_34.0 = [_2,_10,_2,_10,_10,_10,_21];
_29.0 = [_4,_4,_4,_4];
match _3 {
0 => bb2,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
1952789949162616102 => bb13,
_ => bb12
}
}
bb7 = {
_16 = _5;
_20 = (-95_i8) - (-86_i8);
Goto(bb6)
}
bb8 = {
_2 = _10 >> _29.1;
_11 = _8 - _7;
_32 = _4 as u32;
_4 = 1972_u16 * 36001_u16;
_10 = -_2;
_20 = 1326698554_i32 as i8;
_32 = 4276061007_u32 + 3316965000_u32;
Goto(bb5)
}
bb9 = {
_9 = _4 as isize;
_12 = RET;
_8 = !RET;
_3 = 10023566641149673831_u64;
_3 = 1952789949162616102_u64;
_30 = [_10];
_29.0 = _17;
_23 = !_24;
_1 = 22655776209867881111342155940170853141_i128 | 16385774721183328110396789830230224363_i128;
_26 = [_20,_20,_20];
RET = _9;
Call(RET = core::intrinsics::bswap(_8), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
_14 = _23 & _24;
_8 = _9 | RET;
_17 = [_4,_4,_4,_4];
_9 = -_12;
_25.3 = _3 as f32;
_3 = 13274107013846230738_u64;
_16 = [(-8279989496600720556_i64),5855605653982203677_i64,6122390933000767418_i64];
_5 = _16;
_7 = _12;
_26 = [_20,_20,_20];
RET = _9 ^ _7;
_29 = (_25.0, _25.1, _25.2, _25.3);
_10 = _2 & _21;
Goto(bb3)
}
bb11 = {
_14 = !false;
_22 = _4 as f32;
_16 = [5979747191720963158_i64,289017699713762973_i64,(-693093592916459944_i64)];
_20 = !39_i8;
_25.1 = _6 - _6;
_23 = _14 ^ _14;
_7 = _1 as isize;
_26 = [_20,_20,_20];
_24 = !_23;
_14 = !_24;
_25.0 = _17;
_25.2 = 173_u8;
_26 = [_20,_20,_20];
_21 = _14 as i16;
_6 = !_25.1;
_16 = [5666087957716067475_i64,855410560898404323_i64,(-1561785154782584786_i64)];
_5 = [(-8495615176895399825_i64),1950772832764414810_i64,(-8466550921867540405_i64)];
_10 = _3 as i16;
_6 = _14 as u128;
_27 = _19;
_12 = _8;
Call(_25.2 = fn2(RET, _1, _2, _12, _27), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
_34.3 = [_14,_23,_14,_23,_24,_14,_23,_23];
_12 = _8 * _9;
_12 = !_11;
_25.1 = _29.1;
_4 = 21408_u16 >> _21;
_34.0 = [_10,_10,_2,_10,_2,_21,_10];
_34.5 = [_4,_4,_4,_4];
_34.2 = _25.1 as u8;
_25 = _29;
_22 = 6167103672792881251_i64 as f32;
_3 = 13575110293436927984_u64;
_25.1 = (-490599186_i32) as u128;
_36 = -_1;
_33 = _5;
_34.1 = [_23,_14,_24];
_29.1 = _25.1 + _25.1;
_4 = !20312_u16;
_34.0 = [_2,_2,_10,_10,_10,_10,_2];
_39 = [_14,_23,_14];
_11 = _32 as isize;
_30 = [_10];
_12 = !_11;
Goto(bb14)
}
bb14 = {
_9 = _11;
_37 = [_4,_4,_4,_4,_4,_4];
_24 = _34.2 > _34.2;
_29.2 = !_34.2;
_35 = '\u{5d89}';
_38 = [3277787623969578005_i64,1637193471224510427_i64,(-3808158357881497768_i64)];
_22 = _19;
_19 = -_25.3;
_34.0 = [_2,_2,_10,_10,_2,_2,_10];
_7 = !_11;
_37 = [_4,_4,_4,_4,_4,_4];
_26 = [_20,_20,_20];
_25.3 = _22 - _29.3;
_31.0 = [_13,_13,_13,_13];
_20 = (-42_i8);
_25.2 = _29.2;
_34.0 = [_2,_2,_2,_10,_2,_10,_2];
_34.4 = [_29.1,_6,_29.1,_29.1,_25.1,_6];
_34.3 = [_23,_14,_24,_14,_24,_14,_24,_24];
_29.0 = _17;
_21 = _2;
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(1_usize, 23_usize, Move(_23), 31_usize, Move(_31), 35_usize, Move(_35), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(1_usize, 36_usize, Move(_36), 8_usize, Move(_8), 39_usize, Move(_39), 32_usize, Move(_32)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(1_usize, 17_usize, Move(_17), 38_usize, Move(_38), 6_usize, Move(_6), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(1_usize, 14_usize, Move(_14), 9_usize, Move(_9), 7_usize, Move(_7), 45_usize, _45), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: i128,mut _3: i16,mut _4: isize,mut _5: f32) -> u8 {
mir! {
type RET = u8;
let _6: i16;
let _7: char;
let _8: i128;
let _9: *mut [i8; 3];
let _10: isize;
let _11: (f32, f64, u8);
let _12: [i16; 1];
let _13: Adt55;
let _14: bool;
let _15: isize;
let _16: bool;
let _17: ([u16; 4], u128, u8, f32);
let _18: [i16; 1];
let _19: bool;
let _20: i8;
let _21: char;
let _22: f32;
let _23: f64;
let _24: i64;
let _25: *const usize;
let _26: ([u16; 4], u128, u8, f32);
let _27: *const ([usize; 4],);
let _28: isize;
let _29: i64;
let _30: isize;
let _31: char;
let _32: [i16; 7];
let _33: Adt59;
let _34: *const [bool; 5];
let _35: Adt48;
let _36: *const usize;
let _37: isize;
let _38: [u128; 6];
let _39: Adt52;
let _40: char;
let _41: bool;
let _42: *mut ([u16; 4], u128, u8, f32);
let _43: isize;
let _44: [i16; 7];
let _45: ();
let _46: ();
{
_4 = false as isize;
_5 = 2499306918_u32 as f32;
RET = !12_u8;
_2 = false as i128;
RET = (-443749609_i32) as u8;
_7 = '\u{c1b6f}';
_3 = (-26440_i16) | (-14912_i16);
_6 = _3;
_7 = '\u{7608a}';
_3 = !_6;
RET = !16_u8;
_4 = 65446_u16 as isize;
RET = !188_u8;
_6 = _3 * _3;
_8 = -_2;
_2 = !_8;
Call(_4 = fn3(_1, _6, _2, _1, _7, _3, _7, _3, _6, _5, _1, _1, _2, _6, _3, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = _4 >> _8;
_1 = _2 as isize;
_1 = _4 - _4;
_10 = 14099769828883046593_u64 as isize;
RET = !251_u8;
RET = 7552024317580243237_i64 as u8;
_11.0 = _5;
_1 = _4;
_2 = _3 as i128;
_12 = [_6];
_11.2 = 652921622869620367_i64 as u8;
_4 = 3358939654454545834_usize as isize;
_5 = _11.0;
RET = _11.2 & _11.2;
_11.0 = -_5;
_11.2 = !RET;
RET = !_11.2;
_8 = (-111_i8) as i128;
_10 = -_1;
_12 = [_6];
_11.1 = 7_usize as f64;
_4 = -_10;
_3 = -_6;
_4 = _10 + _1;
_1 = -_4;
_3 = _6 << _4;
Goto(bb2)
}
bb2 = {
_11.2 = RET;
Goto(bb3)
}
bb3 = {
Goto(bb4)
}
bb4 = {
_4 = -_10;
_17.1 = 264179397895922802060618485447745570452_u128 >> _4;
_17.3 = -_11.0;
_17.3 = 18476_u16 as f32;
_3 = 17471_u16 as i16;
RET = _11.2 & _11.2;
_14 = false;
_14 = !false;
_19 = _4 <= _1;
_10 = 70_i8 as isize;
Goto(bb5)
}
bb5 = {
_20 = !(-60_i8);
_18 = [_6];
_22 = _5 + _11.0;
_4 = !_1;
_15 = _1;
_17.0 = [10476_u16,1442_u16,56969_u16,34394_u16];
_22 = _17.3;
_5 = 1698713104_i32 as f32;
_21 = _7;
_2 = _8 | _8;
_17.0 = [21931_u16,63657_u16,59054_u16,38572_u16];
_18 = [_6];
_21 = _7;
_11.1 = 15969347542952650902_usize as f64;
_17.2 = 3333740494_u32 as u8;
_2 = _17.3 as i128;
_12 = [_6];
RET = _11.2;
_17.2 = 11592_u16 as u8;
_17.0 = [49602_u16,36368_u16,31101_u16,25439_u16];
_20 = 47_i8 >> _17.1;
_24 = !5903748738993402525_i64;
_8 = _2 - _2;
_17.3 = _6 as f32;
_11.0 = _22 + _17.3;
_17.1 = !42818323980612658961096989963320221394_u128;
_19 = _15 < _4;
_22 = _11.0 + _17.3;
_3 = _6 * _6;
_1 = _15;
Goto(bb6)
}
bb6 = {
_23 = _11.1;
_3 = !_6;
_7 = _21;
_11.0 = _24 as f32;
Goto(bb7)
}
bb7 = {
_6 = _3 - _3;
_11.0 = _24 as f32;
_15 = _17.3 as isize;
_21 = _7;
_3 = _6 - _6;
_20 = !(-89_i8);
_11.0 = _22 * _22;
_16 = _19;
_26.2 = !_11.2;
_11.2 = _19 as u8;
_29 = _15 as i64;
_16 = !_19;
_26 = _17;
_15 = -_1;
_5 = _26.3 + _11.0;
_5 = _22;
_21 = _7;
_11.2 = _7 as u8;
_10 = _20 as isize;
Goto(bb8)
}
bb8 = {
_11.1 = _23;
_1 = _26.2 as isize;
_2 = _7 as i128;
_24 = _29;
_17.1 = _26.1 >> _29;
_22 = _5;
_28 = !_15;
_8 = _5 as i128;
_17 = _26;
Goto(bb9)
}
bb9 = {
_11.0 = -_17.3;
_21 = _7;
_24 = _29 & _29;
_17.1 = !_26.1;
_15 = _4 & _1;
RET = !_17.2;
_21 = _7;
_17 = (_26.0, _26.1, RET, _22);
_7 = _21;
_2 = !_8;
_26.3 = _5;
_23 = -_11.1;
_21 = _7;
_8 = _19 as i128;
_18 = [_3];
_8 = _11.1 as i128;
Goto(bb10)
}
bb10 = {
_17.3 = -_5;
_1 = _28 & _4;
_26.3 = _17.3;
_31 = _21;
_11.0 = 57610_u16 as f32;
_17.0 = [64382_u16,63928_u16,14742_u16,18500_u16];
_30 = !_15;
_3 = !_6;
_28 = _3 as isize;
_7 = _21;
_17.3 = _22;
_32 = [_6,_3,_3,_3,_3,_6,_3];
_15 = -_1;
_5 = _22;
_23 = -_11.1;
_26.1 = _16 as u128;
_14 = _5 <= _26.3;
_26.0 = [14329_u16,27739_u16,13693_u16,61896_u16];
_26.2 = _17.2 - _17.2;
_35.fld2 = [16829743590333381395_usize,10844676639788321975_usize,629872633748996962_usize,10718226289222549004_usize,12044584872168241023_usize,1_usize,17666796517563299012_usize,8883250718973533371_usize];
RET = !_17.2;
Goto(bb11)
}
bb11 = {
Goto(bb12)
}
bb12 = {
_35.fld1.0 = _32;
Goto(bb13)
}
bb13 = {
_14 = !_16;
_26.0 = [58888_u16,3354_u16,26757_u16,18362_u16];
_26.3 = _5;
_35.fld3 = _26.3 + _5;
_38 = [_26.1,_26.1,_26.1,_26.1,_26.1,_26.1];
_37 = _1 & _15;
_8 = -_2;
_6 = -_3;
_15 = _4;
_5 = _35.fld3;
_17.1 = _26.1 - _26.1;
_31 = _21;
_35.fld1.5 = _17.0;
_11 = (_22, _23, _26.2);
_11.2 = !_26.2;
_35.fld2 = [0_usize,5_usize,6_usize,7797529011720863974_usize,3_usize,3_usize,6_usize,6141328442730962468_usize];
Goto(bb14)
}
bb14 = {
_35.fld2 = [12681432495187619129_usize,11984365958217907668_usize,2_usize,1_usize,7481022168953524546_usize,6_usize,966510709863191760_usize,6416456352368864661_usize];
_35.fld1.1 = [_14,_19,_16];
_42 = core::ptr::addr_of_mut!(_17);
_22 = (*_42).3 + (*_42).3;
_37 = _30;
_26.1 = _17.1 & (*_42).1;
_35.fld4 = [_6,_3,_3,_3,_6,_3,_6];
_34 = core::ptr::addr_of!(_35.fld5);
(*_42) = (_35.fld1.5, _26.1, _26.2, _22);
(*_42).3 = _26.3;
_41 = _26.1 == (*_42).1;
_12 = [_3];
_35.fld1.5 = [5280_u16,36438_u16,53107_u16,60815_u16];
(*_42).0 = [55138_u16,34958_u16,42140_u16,8877_u16];
_37 = _30;
_35.fld1.4 = [_17.1,_26.1,(*_42).1,(*_42).1,(*_42).1,(*_42).1];
_11.2 = _17.2 << _29;
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(2_usize, 41_usize, Move(_41), 38_usize, Move(_38), 28_usize, Move(_28), 37_usize, Move(_37)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(2_usize, 29_usize, Move(_29), 16_usize, Move(_16), 4_usize, Move(_4), 30_usize, Move(_30)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(2_usize, 3_usize, Move(_3), 32_usize, Move(_32), 24_usize, Move(_24), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: isize,mut _2: i16,mut _3: i128,mut _4: isize,mut _5: char,mut _6: i16,mut _7: char,mut _8: i16,mut _9: i16,mut _10: f32,mut _11: isize,mut _12: isize,mut _13: i128,mut _14: i16,mut _15: i16,mut _16: isize) -> isize {
mir! {
type RET = isize;
let _17: char;
let _18: Adt46;
let _19: ([usize; 4],);
let _20: *const [bool; 5];
let _21: [bool; 5];
let _22: i64;
let _23: Adt49;
let _24: [i64; 3];
let _25: isize;
let _26: isize;
let _27: Adt59;
let _28: [usize; 8];
let _29: u64;
let _30: [bool; 8];
let _31: [u32; 7];
let _32: Adt52;
let _33: [u32; 7];
let _34: char;
let _35: i32;
let _36: f32;
let _37: isize;
let _38: *mut bool;
let _39: ();
let _40: ();
{
_3 = _13;
_14 = _2;
_18.fld2.1 = [false,true,true];
_3 = !_13;
_1 = _16;
_18.fld0 = core::ptr::addr_of_mut!(_9);
_18.fld2.0 = [_2,_8,_8,_9,_15,_14,_9];
_4 = 34168_u16 as isize;
_18.fld2.0 = [_9,_2,_9,_6,_9,_14,_6];
_18.fld2.1 = [true,false,true];
_6 = (-9030691089569066015_i64) as i16;
RET = (-7452822118859765250_i64) as isize;
_17 = _7;
_14 = _2;
RET = _11 + _16;
_7 = _5;
_18.fld2.2 = !2_u8;
RET = !_16;
_15 = _17 as i16;
_18.fld2.5 = [32897_u16,29538_u16,35256_u16,9895_u16];
_18.fld1 = !11764208925448178701_usize;
_21 = [true,true,true,true,true];
Goto(bb1)
}
bb1 = {
_3 = 2022601946_i32 as i128;
_18.fld2.0 = [_2,_2,_8,_2,_8,_14,_2];
_18.fld2.5 = [64836_u16,64954_u16,54062_u16,58404_u16];
_18.fld1 = 13499143698973563388_usize | 4056624121146017906_usize;
_5 = _7;
_7 = _5;
_13 = !_3;
_7 = _5;
Call(_8 = fn4(_11, _16), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_14 = _9 >> _2;
_18.fld2.0 = [_9,_9,_9,_2,_14,_6,_14];
_6 = -_14;
_18.fld2.3 = [false,false,false,false,true,true,true,true];
_6 = _14 - _14;
_11 = -_4;
_10 = 263873840169679068161878190926491905316_u128 as f32;
_18.fld0 = core::ptr::addr_of_mut!(_2);
Call(_18.fld3 = fn19(RET, _10, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = !_16;
_1 = (-5623822548979452897_i64) as isize;
_7 = _5;
_22 = 408777715073043249_i64;
_7 = _17;
_25 = _16;
_25 = !_16;
_26 = (-708299503_i32) as isize;
_18.fld2.0 = [_6,_8,_9,_2,_2,_2,_2];
_18.fld0 = core::ptr::addr_of_mut!(_9);
_24 = [_22,_22,_22];
Goto(bb4)
}
bb4 = {
_8 = -_9;
_18.fld1 = !12145481085940361414_usize;
_16 = _25 * RET;
_11 = _12;
_14 = _8 << _16;
_11 = _17 as isize;
_20 = core::ptr::addr_of!(_21);
_20 = core::ptr::addr_of!((*_20));
_6 = -_9;
_13 = _3;
_18.fld2.1 = [false,true,false];
_24 = [_22,_22,_22];
_1 = _16;
_4 = !_12;
(*_20) = [true,false,false,true,true];
_13 = _3 * _3;
match _22 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
408777715073043249 => bb13,
_ => bb12
}
}
bb5 = {
RET = !_16;
_1 = (-5623822548979452897_i64) as isize;
_7 = _5;
_22 = 408777715073043249_i64;
_7 = _17;
_25 = _16;
_25 = !_16;
_26 = (-708299503_i32) as isize;
_18.fld2.0 = [_6,_8,_9,_2,_2,_2,_2];
_18.fld0 = core::ptr::addr_of_mut!(_9);
_24 = [_22,_22,_22];
Goto(bb4)
}
bb6 = {
_14 = _9 >> _2;
_18.fld2.0 = [_9,_9,_9,_2,_14,_6,_14];
_6 = -_14;
_18.fld2.3 = [false,false,false,false,true,true,true,true];
_6 = _14 - _14;
_11 = -_4;
_10 = 263873840169679068161878190926491905316_u128 as f32;
_18.fld0 = core::ptr::addr_of_mut!(_2);
Call(_18.fld3 = fn19(RET, _10, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_3 = 2022601946_i32 as i128;
_18.fld2.0 = [_2,_2,_8,_2,_8,_14,_2];
_18.fld2.5 = [64836_u16,64954_u16,54062_u16,58404_u16];
_18.fld1 = 13499143698973563388_usize | 4056624121146017906_usize;
_5 = _7;
_7 = _5;
_13 = !_3;
_7 = _5;
Call(_8 = fn4(_11, _16), ReturnTo(bb2), UnwindUnreachable())
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
_7 = _5;
_8 = 1554869361_i32 as i16;
_8 = _15;
_2 = 42482_u16 as i16;
_13 = !_3;
_19.0 = [_18.fld1,_18.fld1,_18.fld1,_18.fld1];
_3 = _18.fld2.2 as i128;
_9 = _15 ^ _14;
_9 = _8;
_18.fld2.4 = [86591920691961892174103703874847775855_u128,129816125802090865725844656066796948383_u128,254732163486744877421130330476729019034_u128,17408780902248456017926726339745545808_u128,3314025669806542208140609517439633801_u128,270242040879783693090444066298023149775_u128];
_29 = !12724637044320886362_u64;
match _22 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
408777715073043249 => bb20,
_ => bb19
}
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_3 = 2022601946_i32 as i128;
_18.fld2.0 = [_2,_2,_8,_2,_8,_14,_2];
_18.fld2.5 = [64836_u16,64954_u16,54062_u16,58404_u16];
_18.fld1 = 13499143698973563388_usize | 4056624121146017906_usize;
_5 = _7;
_7 = _5;
_13 = !_3;
_7 = _5;
Call(_8 = fn4(_11, _16), ReturnTo(bb2), UnwindUnreachable())
}
bb17 = {
_8 = -_9;
_18.fld1 = !12145481085940361414_usize;
_16 = _25 * RET;
_11 = _12;
_14 = _8 << _16;
_11 = _17 as isize;
_20 = core::ptr::addr_of!(_21);
_20 = core::ptr::addr_of!((*_20));
_6 = -_9;
_13 = _3;
_18.fld2.1 = [false,true,false];
_24 = [_22,_22,_22];
_1 = _16;
_4 = !_12;
(*_20) = [true,false,false,true,true];
_13 = _3 * _3;
match _22 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
408777715073043249 => bb13,
_ => bb12
}
}
bb18 = {
_14 = _9 >> _2;
_18.fld2.0 = [_9,_9,_9,_2,_14,_6,_14];
_6 = -_14;
_18.fld2.3 = [false,false,false,false,true,true,true,true];
_6 = _14 - _14;
_11 = -_4;
_10 = 263873840169679068161878190926491905316_u128 as f32;
_18.fld0 = core::ptr::addr_of_mut!(_2);
Call(_18.fld3 = fn19(RET, _10, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb19 = {
_3 = 2022601946_i32 as i128;
_18.fld2.0 = [_2,_2,_8,_2,_8,_14,_2];
_18.fld2.5 = [64836_u16,64954_u16,54062_u16,58404_u16];
_18.fld1 = 13499143698973563388_usize | 4056624121146017906_usize;
_5 = _7;
_7 = _5;
_13 = !_3;
_7 = _5;
Call(_8 = fn4(_11, _16), ReturnTo(bb2), UnwindUnreachable())
}
bb20 = {
_23 = Adt49::Variant0 { fld0: _18.fld2.2 };
_12 = -_25;
_4 = _1;
RET = _17 as isize;
_31 = [2095255392_u32,313610936_u32,1993522062_u32,653581369_u32,697647037_u32,151288370_u32,61730862_u32];
RET = _12;
_15 = _14;
_18.fld2.0 = [_2,_6,_15,_6,_15,_15,_15];
Goto(bb21)
}
bb21 = {
Call(_39 = dump_var(3_usize, 14_usize, Move(_14), 3_usize, Move(_3), 19_usize, Move(_19), 31_usize, Move(_31)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_39 = dump_var(3_usize, 1_usize, Move(_1), 13_usize, Move(_13), 15_usize, Move(_15), 25_usize, Move(_25)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_39 = dump_var(3_usize, 9_usize, Move(_9), 26_usize, Move(_26), 11_usize, Move(_11), 6_usize, Move(_6)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: isize,mut _2: isize) -> i16 {
mir! {
type RET = i16;
let _3: char;
let _4: u32;
let _5: Adt56;
let _6: *const [bool; 5];
let _7: i8;
let _8: [i16; 7];
let _9: *mut bool;
let _10: [i16; 7];
let _11: i64;
let _12: Adt52;
let _13: [i16; 7];
let _14: [u128; 6];
let _15: Adt45;
let _16: ([u16; 4], u128, u8, f32);
let _17: [usize; 8];
let _18: [u16; 6];
let _19: ([bool; 3], [i8; 3], i128, [i8; 3], i16);
let _20: char;
let _21: f32;
let _22: Adt58;
let _23: isize;
let _24: isize;
let _25: ();
let _26: ();
{
_1 = _2;
_2 = 5803031970238643795_u64 as isize;
RET = (-18914_i16);
RET = '\u{2b691}' as i16;
_3 = '\u{e6b57}';
Goto(bb1)
}
bb1 = {
_1 = (-113918842824483885448000283515648373306_i128) as isize;
_3 = '\u{da356}';
_2 = 220_u8 as isize;
_2 = 828170794_u32 as isize;
_2 = !_1;
RET = !13108_i16;
RET = (-26071_i16);
_1 = _2;
_2 = 9218911785388859272_i64 as isize;
_2 = !_1;
_3 = '\u{8e5c5}';
_2 = _1;
_4 = (-7231055002341441651_i64) as u32;
_1 = _2 ^ _2;
RET = -15594_i16;
Goto(bb2)
}
bb2 = {
_1 = _2 + _2;
_5.fld3.fld0.0 = [43021_u16,5696_u16,13788_u16,18104_u16];
_5.fld5.fld3 = -(-67_i8);
_5.fld5.fld0 = core::ptr::addr_of_mut!(RET);
_5.fld2 = [14017679978465960263_usize,4_usize,0_usize,8591286515254814213_usize];
_5.fld3.fld0.1 = false as u128;
_5.fld3.fld1 = [_4,_4,_4,_4,_4,_4,_4];
_5.fld3.fld0.2 = !143_u8;
_5.fld0.0 = [3_usize,6614486415571358865_usize,11672227076354867_usize,3_usize];
_5.fld4 = !RET;
_2 = _1;
_5.fld5.fld2.4 = [_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1];
_5.fld5.fld2.5 = _5.fld3.fld0.0;
_5.fld2 = [7_usize,14447612003985206915_usize,15444978719868620014_usize,12675488967700357909_usize];
_5.fld5.fld1 = 3_usize;
_5.fld5.fld2.0 = [_5.fld4,_5.fld4,_5.fld4,RET,RET,RET,_5.fld4];
_1 = -_2;
_2 = 1268964979263152859_u64 as isize;
_5.fld4 = _4 as i16;
RET = _5.fld4 & _5.fld4;
Call(_5.fld1 = fn5(_3, _5.fld2, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Goto(bb4)
}
bb4 = {
_5.fld5.fld1 = !3567389601717220870_usize;
_5.fld3.fld0.2 = !12_u8;
Call(_5.fld5.fld2.2 = fn18(_5.fld1, _5.fld1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_1 = 45491_u16 as isize;
_7 = _5.fld5.fld3 ^ _5.fld5.fld3;
_4 = _5.fld3.fld0.2 as u32;
RET = !_5.fld4;
_3 = '\u{978a7}';
_8 = _5.fld5.fld2.0;
_5.fld5.fld2.3 = [false,true,false,false,false,false,true,false];
_5.fld5.fld2.4 = [_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1];
_5.fld0.0 = [_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1];
_5.fld3.fld2 = _5.fld4 - RET;
_10 = [_5.fld4,_5.fld3.fld2,_5.fld4,_5.fld3.fld2,_5.fld3.fld2,_5.fld3.fld2,_5.fld3.fld2];
_5.fld5.fld2.3 = [true,true,false,false,false,false,true,true];
_5.fld3.fld0.0 = [19827_u16,35663_u16,503_u16,56760_u16];
_5.fld3.fld0.2 = !_5.fld5.fld2.2;
RET = _5.fld4 ^ _5.fld3.fld2;
RET = !_5.fld3.fld2;
_5.fld3.fld2 = _5.fld4;
_5.fld5.fld3 = _7 >> _7;
_2 = _1;
Goto(bb6)
}
bb6 = {
_5.fld3.fld2 = !RET;
_5.fld5.fld3 = !_7;
_10 = _5.fld5.fld2.0;
_5.fld5.fld3 = (-4476639639861501389_i64) as i8;
Goto(bb7)
}
bb7 = {
_7 = _5.fld5.fld3;
_10 = [_5.fld4,RET,_5.fld3.fld2,_5.fld3.fld2,_5.fld4,_5.fld4,_5.fld4];
_11 = (-2194165572371667021_i64) * 7520263628499634032_i64;
_5.fld1 = core::ptr::addr_of_mut!(_5.fld3.fld2);
_5.fld4 = -_5.fld3.fld2;
_5.fld5.fld2.4 = [_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1];
_5.fld2 = [_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1];
_5.fld4 = _2 as i16;
_5.fld2 = [_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1];
_5.fld5.fld1 = _5.fld3.fld0.2 as usize;
_5.fld1 = _5.fld5.fld0;
_4 = _5.fld5.fld1 as u32;
_14 = [_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1];
_5.fld3.fld0.0 = _5.fld5.fld2.5;
_5.fld3.fld1 = [_4,_4,_4,_4,_4,_4,_4];
_8 = [RET,RET,_5.fld3.fld2,RET,_5.fld3.fld2,RET,_5.fld4];
_5.fld5.fld2.1 = [true,false,true];
_15.fld0 = !11350_u16;
_5.fld5.fld1 = !7_usize;
_16.3 = _5.fld3.fld2 as f32;
_5.fld5.fld3 = -_7;
_3 = '\u{3f24c}';
_5.fld3.fld3.1 = [_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1];
_5.fld5.fld3 = -_7;
Goto(bb8)
}
bb8 = {
_5.fld0.0 = [_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1];
_16.1 = _5.fld3.fld0.1 >> _5.fld5.fld1;
_15 = Adt45 { fld0: 34337_u16 };
_16.2 = _5.fld3.fld0.2;
_5.fld5.fld2.3 = [true,true,true,true,false,true,true,false];
_16.1 = !_5.fld3.fld0.1;
_5.fld3.fld3.2 = core::ptr::addr_of_mut!(_19.1);
_3 = '\u{e8171}';
_5.fld0.0 = _5.fld2;
RET = _5.fld3.fld2;
_15 = Adt45 { fld0: 27111_u16 };
_18 = [_15.fld0,_15.fld0,_15.fld0,_15.fld0,_15.fld0,_15.fld0];
_18 = [_15.fld0,_15.fld0,_15.fld0,_15.fld0,_15.fld0,_15.fld0];
_4 = 717879805_u32 << _5.fld5.fld1;
_10 = [_5.fld4,_5.fld3.fld2,_5.fld3.fld2,_5.fld4,RET,RET,_5.fld3.fld2];
_5.fld3.fld0.3 = _16.3;
_19.2 = !(-145780960696803381147006757424712673773_i128);
_5.fld3.fld3.2 = core::ptr::addr_of_mut!(_19.1);
_5.fld3.fld0.0 = [_15.fld0,_15.fld0,_15.fld0,_15.fld0];
_5.fld4 = _11 as i16;
_18 = [_15.fld0,_15.fld0,_15.fld0,_15.fld0,_15.fld0,_15.fld0];
_5.fld5.fld1 = 4922923994570341355_usize;
_17 = [_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1];
_1 = _5.fld3.fld0.3 as isize;
Goto(bb9)
}
bb9 = {
_19.3 = [_7,_5.fld5.fld3,_7];
_5.fld2 = _5.fld0.0;
match _15.fld0 {
0 => bb7,
1 => bb2,
2 => bb8,
27111 => bb11,
_ => bb10
}
}
bb10 = {
_1 = 45491_u16 as isize;
_7 = _5.fld5.fld3 ^ _5.fld5.fld3;
_4 = _5.fld3.fld0.2 as u32;
RET = !_5.fld4;
_3 = '\u{978a7}';
_8 = _5.fld5.fld2.0;
_5.fld5.fld2.3 = [false,true,false,false,false,false,true,false];
_5.fld5.fld2.4 = [_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1];
_5.fld0.0 = [_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1];
_5.fld3.fld2 = _5.fld4 - RET;
_10 = [_5.fld4,_5.fld3.fld2,_5.fld4,_5.fld3.fld2,_5.fld3.fld2,_5.fld3.fld2,_5.fld3.fld2];
_5.fld5.fld2.3 = [true,true,false,false,false,false,true,true];
_5.fld3.fld0.0 = [19827_u16,35663_u16,503_u16,56760_u16];
_5.fld3.fld0.2 = !_5.fld5.fld2.2;
RET = _5.fld4 ^ _5.fld3.fld2;
RET = !_5.fld3.fld2;
_5.fld3.fld2 = _5.fld4;
_5.fld5.fld3 = _7 >> _7;
_2 = _1;
Goto(bb6)
}
bb11 = {
_5.fld5.fld1 = _3 as usize;
match _15.fld0 {
0 => bb8,
1 => bb12,
27111 => bb14,
_ => bb13
}
}
bb12 = {
_1 = 45491_u16 as isize;
_7 = _5.fld5.fld3 ^ _5.fld5.fld3;
_4 = _5.fld3.fld0.2 as u32;
RET = !_5.fld4;
_3 = '\u{978a7}';
_8 = _5.fld5.fld2.0;
_5.fld5.fld2.3 = [false,true,false,false,false,false,true,false];
_5.fld5.fld2.4 = [_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1,_5.fld3.fld0.1];
_5.fld0.0 = [_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1,_5.fld5.fld1];
_5.fld3.fld2 = _5.fld4 - RET;
_10 = [_5.fld4,_5.fld3.fld2,_5.fld4,_5.fld3.fld2,_5.fld3.fld2,_5.fld3.fld2,_5.fld3.fld2];
_5.fld5.fld2.3 = [true,true,false,false,false,false,true,true];
_5.fld3.fld0.0 = [19827_u16,35663_u16,503_u16,56760_u16];
_5.fld3.fld0.2 = !_5.fld5.fld2.2;
RET = _5.fld4 ^ _5.fld3.fld2;
RET = !_5.fld3.fld2;
_5.fld3.fld2 = _5.fld4;
_5.fld5.fld3 = _7 >> _7;
_2 = _1;
Goto(bb6)
}
bb13 = {
_5.fld3.fld2 = !RET;
_5.fld5.fld3 = !_7;
_10 = _5.fld5.fld2.0;
_5.fld5.fld3 = (-4476639639861501389_i64) as i8;
Goto(bb7)
}
bb14 = {
RET = _5.fld5.fld1 as i16;
_16.2 = _5.fld5.fld2.2 ^ _5.fld5.fld2.2;
_15 = Adt45 { fld0: 31319_u16 };
RET = _5.fld3.fld2;
_19.0 = [false,false,false];
_5.fld3.fld3.1 = _17;
_14 = _5.fld5.fld2.4;
_19.1 = [_7,_7,_7];
_5.fld3.fld0 = (_5.fld5.fld2.5, _16.1, _5.fld5.fld2.2, _16.3);
_20 = _3;
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(4_usize, 20_usize, Move(_20), 3_usize, Move(_3), 1_usize, Move(_1), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(4_usize, 8_usize, Move(_8), 14_usize, Move(_14), 26_usize, _26, 26_usize, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: char,mut _2: [usize; 4],mut _3: char) -> *mut i16 {
mir! {
type RET = *mut i16;
let _4: ([u16; 4], u128, u8, f32);
let _5: *mut [i8; 3];
let _6: Adt54;
let _7: Adt59;
let _8: ([bool; 3], [i8; 3], i128, [i8; 3], i16);
let _9: i64;
let _10: ([usize; 4],);
let _11: bool;
let _12: Adt45;
let _13: i8;
let _14: bool;
let _15: f32;
let _16: isize;
let _17: [u32; 7];
let _18: [i64; 3];
let _19: f64;
let _20: [u32; 7];
let _21: Adt59;
let _22: Adt45;
let _23: i64;
let _24: [bool; 8];
let _25: [usize; 4];
let _26: isize;
let _27: i8;
let _28: *mut i16;
let _29: i64;
let _30: [usize; 4];
let _31: isize;
let _32: *mut ([u16; 4], u128, u8, f32);
let _33: [bool; 8];
let _34: [i64; 3];
let _35: Adt47;
let _36: isize;
let _37: Adt52;
let _38: [u128; 6];
let _39: [i16; 7];
let _40: bool;
let _41: char;
let _42: i32;
let _43: ();
let _44: ();
{
_3 = _1;
_2 = [1_usize,4_usize,7523970345723873580_usize,8570966901964552449_usize];
_4.2 = _1 as u8;
_4.3 = 9903553230674449671_u64 as f32;
_4.3 = (-22129_i16) as f32;
_2 = [2_usize,12464975937038112453_usize,2049795966624410095_usize,13936467907527810554_usize];
_1 = _3;
_4.1 = !129965505722711869356866380379954172254_u128;
_4.1 = !141852170148030318101291225525916716571_u128;
_4.2 = !85_u8;
_6.fld5 = [_4.1,_4.1,_4.1,_4.1,_4.1,_4.1];
_6.fld1 = _4.2;
_6.fld6 = 9210045503413356414_i64 - (-4499455902405502616_i64);
_6.fld1 = _4.2;
_6.fld2 = (-61_isize) << _6.fld1;
_8.1 = [106_i8,16_i8,(-31_i8)];
_8.1 = [(-88_i8),102_i8,(-120_i8)];
_1 = _3;
_4.3 = _6.fld2 as f32;
_6.fld2 = 45541_u16 as isize;
_4.3 = 1_i8 as f32;
_6.fld4 = Adt45 { fld0: 12250_u16 };
_4.2 = _6.fld1 | _6.fld1;
_4.0 = [_6.fld4.fld0,_6.fld4.fld0,_6.fld4.fld0,_6.fld4.fld0];
Goto(bb1)
}
bb1 = {
_8.4 = _4.3 as i16;
_6.fld2 = !67_isize;
_8.2 = 17476234712436178139899078868920993675_i128;
_8.0 = [true,true,true];
_1 = _3;
RET = core::ptr::addr_of_mut!(_8.4);
_9 = _6.fld6;
_8.4 = 37_i8 as i16;
_1 = _3;
_6.fld4 = Adt45 { fld0: 26655_u16 };
(*RET) = 11965_i16;
_2 = [6_usize,1_usize,666385590125491193_usize,7_usize];
_11 = false;
_6.fld0 = (_2,);
_6.fld0.0 = _2;
_2 = _6.fld0.0;
_6.fld0 = (_2,);
_6.fld0.0 = _2;
_2 = [2_usize,2120325687487031467_usize,0_usize,17989486061242279928_usize];
_8.0 = [_11,_11,_11];
_8.0 = [_11,_11,_11];
Call(RET = fn6(_2, (*RET), _11, _4, _4.0, _2, _8.2, _8.2, _4.2, _8.4, _4.2, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8.0 = [_11,_11,_11];
_5 = core::ptr::addr_of_mut!(_8.1);
_8.2 = (-37594292564930362963187783360417012275_i128) * (-56750266770667796353636060441418308783_i128);
RET = core::ptr::addr_of_mut!(_8.4);
_10 = (_2,);
_6.fld4.fld0 = 50900_u16;
_3 = _1;
(*RET) = -(-20060_i16);
_12 = _6.fld4;
(*_5) = [56_i8,(-104_i8),(-120_i8)];
(*RET) = 11581123910300995138_u64 as i16;
_8.4 = _6.fld2 as i16;
_8.4 = _11 as i16;
_10.0 = [15965518086262345751_usize,7962908134230716654_usize,1_usize,5_usize];
_5 = core::ptr::addr_of_mut!(_8.3);
_8.0 = [_11,_11,_11];
_4.2 = _6.fld1;
_8.3 = _8.1;
_2 = [15859703651879837774_usize,16785725617873328270_usize,7_usize,5040866658446088179_usize];
Goto(bb3)
}
bb3 = {
_8.1 = (*_5);
_8.4 = 3197_i16 + 3060_i16;
(*RET) = 29906_i16;
_4.1 = 10676126323607252680_usize as u128;
(*RET) = _4.1 as i16;
_1 = _3;
_6.fld3 = core::ptr::addr_of_mut!(_4);
_4.3 = (-751504894_i32) as f32;
_4.1 = !102068168113048250378046951970573215499_u128;
_2 = _10.0;
_1 = _3;
_8.2 = 23108984392540103468230006813028584524_i128 & (-73632949823599817048516398976773951998_i128);
_8.3 = _8.1;
_8.3 = [67_i8,66_i8,(-34_i8)];
_4.1 = _1 as u128;
_10 = (_2,);
_15 = _4.3 - _4.3;
_18 = [_6.fld6,_9,_9];
_10 = (_6.fld0.0,);
_8.3 = _8.1;
Goto(bb4)
}
bb4 = {
_12 = Adt45 { fld0: _6.fld4.fld0 };
_15 = _4.3 + _4.3;
_16 = _6.fld2;
_20 = [2946234983_u32,1234063920_u32,1584046789_u32,3390087255_u32,652517818_u32,3803859051_u32,2667999037_u32];
_15 = -_4.3;
(*RET) = 1567_i16 ^ (-25508_i16);
_17 = [729669712_u32,2241489734_u32,3796776105_u32,2648979386_u32,4277432955_u32,1107069507_u32,387301969_u32];
_12.fld0 = _8.2 as u16;
_10 = (_6.fld0.0,);
_12 = Adt45 { fld0: _6.fld4.fld0 };
_19 = 2668500836_u32 as f64;
_22 = _6.fld4;
_14 = _11 & _11;
_6.fld4 = _22;
_4.2 = _6.fld1 & _6.fld1;
_3 = _1;
_6.fld3 = core::ptr::addr_of_mut!(_4);
_16 = 1472090358_i32 as isize;
(*RET) = (-13061_i16);
_14 = _9 == _9;
_13 = !(-59_i8);
_4.1 = 219044726849205798350879550350459346476_u128;
_6.fld4 = Adt45 { fld0: _12.fld0 };
_6.fld6 = _6.fld4.fld0 as i64;
_23 = _9;
_2 = [4_usize,2_usize,6_usize,6_usize];
Call(_8.1 = core::intrinsics::transmute((*_5)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_13 = !(-114_i8);
_10.0 = _6.fld0.0;
_8.0 = [_14,_14,_14];
_26 = _16 - _16;
_9 = _19 as i64;
(*RET) = !(-24234_i16);
_22 = Adt45 { fld0: _6.fld4.fld0 };
_8.2 = 167273463318677159465591228904881255550_i128;
Call(_7 = fn7(_23, _10.0, _9, _5, _23, _20, _14, _6), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_6.fld2 = 3216732035_u32 as isize;
(*RET) = 11658_i16;
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_7, 1), 0)).2 = _4.2;
(*RET) = (-32673_i16);
_22 = Adt45 { fld0: Field::<Adt54>(Variant(_7, 1), 1).fld4.fld0 };
place!(Field::<Adt54>(Variant(_7, 1), 1)).fld5 = Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_7, 1), 0).4;
_4.3 = (-1992682275_i32) as f32;
_20 = [3280618402_u32,2188641203_u32,1334426731_u32,985600011_u32,2083139366_u32,3477691508_u32,3257987513_u32];
RET = core::ptr::addr_of_mut!((*RET));
_6.fld1 = !_4.2;
_22.fld0 = 841525895_i32 as u16;
(*RET) = _1 as i16;
_19 = 267015174_u32 as f64;
_25 = [0_usize,3415753786081772914_usize,10184762936463349579_usize,5_usize];
SetDiscriminant(_7, 2);
_16 = -_6.fld2;
place!(Field::<Adt46>(Variant(_7, 2), 7)).fld2.2 = _4.2 & _6.fld1;
match _6.fld4.fld0 {
0 => bb5,
1 => bb2,
50900 => bb7,
_ => bb4
}
}
bb7 = {
place!(Field::<Adt46>(Variant(_7, 2), 7)).fld2.3 = [_14,_14,_11,_14,_11,_11,_14,_11];
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_7, 2), 1)).3 = [_14,_11,_14,_11,_14,_14,_14,_14];
_25 = _2;
_28 = core::ptr::addr_of_mut!((*RET));
_20 = [3768990228_u32,1051493768_u32,3775964445_u32,3151748005_u32,744973847_u32,973741770_u32,231419276_u32];
place!(Field::<Adt46>(Variant(_7, 2), 7)).fld3 = _13;
_13 = _15 as i8;
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_7, 2), 1)).1 = _8.0;
place!(Field::<Adt46>(Variant(_7, 2), 7)).fld3 = _13 >> (*RET);
place!(Field::<u16>(Variant(_7, 2), 0)) = _1 as u16;
_31 = Field::<Adt46>(Variant(_7, 2), 7).fld3 as isize;
place!(Field::<Adt46>(Variant(_7, 2), 7)).fld1 = !2_usize;
_3 = _1;
_8.4 = _19 as i16;
_6.fld0 = (_25,);
place!(Field::<*mut bool>(Variant(_7, 2), 2)) = core::ptr::addr_of_mut!(_11);
_6.fld6 = -_23;
place!(Field::<Adt45>(Variant(_7, 2), 6)).fld0 = _22.fld0;
_1 = _3;
(*RET) = 32622_i16 * (-8586_i16);
RET = _28;
match _4.1 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
6 => bb14,
57505889213264984005432618420863526061 => bb16,
_ => bb15
}
}
bb8 = {
_6.fld2 = 3216732035_u32 as isize;
(*RET) = 11658_i16;
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_7, 1), 0)).2 = _4.2;
(*RET) = (-32673_i16);
_22 = Adt45 { fld0: Field::<Adt54>(Variant(_7, 1), 1).fld4.fld0 };
place!(Field::<Adt54>(Variant(_7, 1), 1)).fld5 = Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_7, 1), 0).4;
_4.3 = (-1992682275_i32) as f32;
_20 = [3280618402_u32,2188641203_u32,1334426731_u32,985600011_u32,2083139366_u32,3477691508_u32,3257987513_u32];
RET = core::ptr::addr_of_mut!((*RET));
_6.fld1 = !_4.2;
_22.fld0 = 841525895_i32 as u16;
(*RET) = _1 as i16;
_19 = 267015174_u32 as f64;
_25 = [0_usize,3415753786081772914_usize,10184762936463349579_usize,5_usize];
SetDiscriminant(_7, 2);
_16 = -_6.fld2;
place!(Field::<Adt46>(Variant(_7, 2), 7)).fld2.2 = _4.2 & _6.fld1;
match _6.fld4.fld0 {
0 => bb5,
1 => bb2,
50900 => bb7,
_ => bb4
}
}
bb9 = {
_13 = !(-114_i8);
_10.0 = _6.fld0.0;
_8.0 = [_14,_14,_14];
_26 = _16 - _16;
_9 = _19 as i64;
(*RET) = !(-24234_i16);
_22 = Adt45 { fld0: _6.fld4.fld0 };
_8.2 = 167273463318677159465591228904881255550_i128;
Call(_7 = fn7(_23, _10.0, _9, _5, _23, _20, _14, _6), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_12 = Adt45 { fld0: _6.fld4.fld0 };
_15 = _4.3 + _4.3;
_16 = _6.fld2;
_20 = [2946234983_u32,1234063920_u32,1584046789_u32,3390087255_u32,652517818_u32,3803859051_u32,2667999037_u32];
_15 = -_4.3;
(*RET) = 1567_i16 ^ (-25508_i16);
_17 = [729669712_u32,2241489734_u32,3796776105_u32,2648979386_u32,4277432955_u32,1107069507_u32,387301969_u32];
_12.fld0 = _8.2 as u16;
_10 = (_6.fld0.0,);
_12 = Adt45 { fld0: _6.fld4.fld0 };
_19 = 2668500836_u32 as f64;
_22 = _6.fld4;
_14 = _11 & _11;
_6.fld4 = _22;
_4.2 = _6.fld1 & _6.fld1;
_3 = _1;
_6.fld3 = core::ptr::addr_of_mut!(_4);
_16 = 1472090358_i32 as isize;
(*RET) = (-13061_i16);
_14 = _9 == _9;
_13 = !(-59_i8);
_4.1 = 219044726849205798350879550350459346476_u128;
_6.fld4 = Adt45 { fld0: _12.fld0 };
_6.fld6 = _6.fld4.fld0 as i64;
_23 = _9;
_2 = [4_usize,2_usize,6_usize,6_usize];
Call(_8.1 = core::intrinsics::transmute((*_5)), ReturnTo(bb5), UnwindUnreachable())
}
bb11 = {
_8.1 = (*_5);
_8.4 = 3197_i16 + 3060_i16;
(*RET) = 29906_i16;
_4.1 = 10676126323607252680_usize as u128;
(*RET) = _4.1 as i16;
_1 = _3;
_6.fld3 = core::ptr::addr_of_mut!(_4);
_4.3 = (-751504894_i32) as f32;
_4.1 = !102068168113048250378046951970573215499_u128;
_2 = _10.0;
_1 = _3;
_8.2 = 23108984392540103468230006813028584524_i128 & (-73632949823599817048516398976773951998_i128);
_8.3 = _8.1;
_8.3 = [67_i8,66_i8,(-34_i8)];
_4.1 = _1 as u128;
_10 = (_2,);
_15 = _4.3 - _4.3;
_18 = [_6.fld6,_9,_9];
_10 = (_6.fld0.0,);
_8.3 = _8.1;
Goto(bb4)
}
bb12 = {
_8.0 = [_11,_11,_11];
_5 = core::ptr::addr_of_mut!(_8.1);
_8.2 = (-37594292564930362963187783360417012275_i128) * (-56750266770667796353636060441418308783_i128);
RET = core::ptr::addr_of_mut!(_8.4);
_10 = (_2,);
_6.fld4.fld0 = 50900_u16;
_3 = _1;
(*RET) = -(-20060_i16);
_12 = _6.fld4;
(*_5) = [56_i8,(-104_i8),(-120_i8)];
(*RET) = 11581123910300995138_u64 as i16;
_8.4 = _6.fld2 as i16;
_8.4 = _11 as i16;
_10.0 = [15965518086262345751_usize,7962908134230716654_usize,1_usize,5_usize];
_5 = core::ptr::addr_of_mut!(_8.3);
_8.0 = [_11,_11,_11];
_4.2 = _6.fld1;
_8.3 = _8.1;
_2 = [15859703651879837774_usize,16785725617873328270_usize,7_usize,5040866658446088179_usize];
Goto(bb3)
}
bb13 = {
_8.4 = _4.3 as i16;
_6.fld2 = !67_isize;
_8.2 = 17476234712436178139899078868920993675_i128;
_8.0 = [true,true,true];
_1 = _3;
RET = core::ptr::addr_of_mut!(_8.4);
_9 = _6.fld6;
_8.4 = 37_i8 as i16;
_1 = _3;
_6.fld4 = Adt45 { fld0: 26655_u16 };
(*RET) = 11965_i16;
_2 = [6_usize,1_usize,666385590125491193_usize,7_usize];
_11 = false;
_6.fld0 = (_2,);
_6.fld0.0 = _2;
_2 = _6.fld0.0;
_6.fld0 = (_2,);
_6.fld0.0 = _2;
_2 = [2_usize,2120325687487031467_usize,0_usize,17989486061242279928_usize];
_8.0 = [_11,_11,_11];
_8.0 = [_11,_11,_11];
Call(RET = fn6(_2, (*RET), _11, _4, _4.0, _2, _8.2, _8.2, _4.2, _8.4, _4.2, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_32 = core::ptr::addr_of_mut!(_4);
_24 = [_11,_14,_11,_11,_14,_14,_14,_14];
place!(Field::<i16>(Variant(_7, 2), 4)) = (*RET) * _8.4;
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_7, 2), 1)).5 = (*_32).0;
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_7, 2), 1)).5 = (*_32).0;
_6.fld1 = _14 as u8;
_34 = [_6.fld6,_23,_9];
(*_28) = Field::<i16>(Variant(_7, 2), 4);
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_7, 2), 1)).2 = _4.2;
_27 = Field::<Adt46>(Variant(_7, 2), 7).fld3 - Field::<Adt46>(Variant(_7, 2), 7).fld3;
(*_5) = [_13,Field::<Adt46>(Variant(_7, 2), 7).fld3,_27];
_34 = _18;
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_7, 2), 1)).3 = Field::<Adt46>(Variant(_7, 2), 7).fld2.3;
_8.3 = _8.1;
(*_32).0 = Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_7, 2), 1).5;
RET = core::ptr::addr_of_mut!((*RET));
_4.3 = _15;
place!(Field::<u16>(Variant(_7, 2), 0)) = !_12.fld0;
_6.fld0 = _10;
(*_32).1 = 313727165889555561323948851991847823112_u128 ^ 220139072991747545186856997172297834011_u128;
_42 = (-1525165929_i32);
Goto(bb17)
}
bb17 = {
Call(_43 = dump_var(5_usize, 3_usize, Move(_3), 1_usize, Move(_1), 10_usize, Move(_10), 23_usize, Move(_23)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_43 = dump_var(5_usize, 42_usize, Move(_42), 11_usize, Move(_11), 13_usize, Move(_13), 24_usize, Move(_24)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_43 = dump_var(5_usize, 27_usize, Move(_27), 34_usize, Move(_34), 44_usize, _44, 44_usize, _44), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: [usize; 4],mut _2: i16,mut _3: bool,mut _4: ([u16; 4], u128, u8, f32),mut _5: [u16; 4],mut _6: [usize; 4],mut _7: i128,mut _8: i128,mut _9: u8,mut _10: i16,mut _11: u8,mut _12: i64) -> *mut i16 {
mir! {
type RET = *mut i16;
let _13: ([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4]);
let _14: *mut [i8; 3];
let _15: bool;
let _16: u16;
let _17: i64;
let _18: *const ([usize; 4],);
let _19: isize;
let _20: Adt57;
let _21: [u16; 4];
let _22: f32;
let _23: *mut bool;
let _24: f32;
let _25: isize;
let _26: ();
let _27: ();
{
_11 = _9 * _9;
_4.1 = !124731608228972720207336643103382950252_u128;
_6 = _1;
_9 = !_11;
_2 = _4.1 as i16;
_2 = _10;
RET = core::ptr::addr_of_mut!(_2);
_6 = _1;
_1 = [9123631566305209438_usize,3987715008659206154_usize,5_usize,5_usize];
_4.2 = _9;
_1 = [4_usize,7_usize,6_usize,10229336319766066183_usize];
_4.1 = 292966963449039967097221412708136529228_u128;
RET = core::ptr::addr_of_mut!(_10);
_10 = _2 >> _12;
_10 = !_2;
_6 = [6_usize,1_usize,1_usize,18377230493003442467_usize];
_1 = [9749329039832334380_usize,1019244613279200322_usize,2_usize,7_usize];
_4.0 = [17533_u16,2154_u16,22302_u16,30249_u16];
(*RET) = _9 as i16;
_3 = true;
(*RET) = _2 >> _4.2;
_11 = !_4.2;
_7 = _8 ^ _8;
(*RET) = _2;
Goto(bb1)
}
bb1 = {
_11 = _4.1 as u8;
_4.2 = _11;
_4.0 = [17555_u16,64389_u16,1850_u16,13819_u16];
_4.2 = 17768_u16 as u8;
_4.3 = _7 as f32;
_13.3 = [_3,_3,_3,_3,_3,_3,_3,_3];
(*RET) = !_2;
_13.2 = '\u{9c5d4}' as u8;
_3 = true;
_2 = (*RET) + (*RET);
_13.2 = _9 | _4.2;
_13.1 = [_3,_3,_3];
_4.3 = 25296_u16 as f32;
(*RET) = -_2;
(*RET) = _2;
_13.0 = [(*RET),(*RET),_2,_2,(*RET),_2,_10];
RET = core::ptr::addr_of_mut!((*RET));
_13.5 = [51783_u16,3209_u16,63744_u16,23848_u16];
_13.5 = _5;
_2 = !(*RET);
_7 = -_8;
_16 = 30330_u16;
_15 = _3;
match _4.1 {
0 => bb2,
1 => bb3,
292966963449039967097221412708136529228 => bb5,
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
_8 = -_7;
_11 = _13.2 | _9;
_16 = 38131_u16;
_8 = (*RET) as i128;
_13.0 = [(*RET),_2,_10,_2,(*RET),(*RET),_2];
_13.1 = [_3,_15,_15];
_13.4 = [_4.1,_4.1,_4.1,_4.1,_4.1,_4.1];
_4.0 = [_16,_16,_16,_16];
_13.1 = [_3,_15,_3];
RET = core::ptr::addr_of_mut!((*RET));
_13.5 = [_16,_16,_16,_16];
_13.2 = !_11;
Goto(bb6)
}
bb6 = {
_11 = !_9;
RET = core::ptr::addr_of_mut!((*RET));
_4.0 = _13.5;
_13.2 = _7 as u8;
_16 = 57017_u16 & 23463_u16;
_13.1 = [_3,_15,_3];
(*RET) = _2;
_13.3 = [_3,_15,_3,_15,_15,_15,_3,_3];
_11 = _9;
(*RET) = _2;
Goto(bb7)
}
bb7 = {
_7 = _8 << (*RET);
_13.2 = !_9;
_13.4 = [_4.1,_4.1,_4.1,_4.1,_4.1,_4.1];
_13.1 = [_3,_15,_15];
_15 = _11 >= _9;
_13.4 = [_4.1,_4.1,_4.1,_4.1,_4.1,_4.1];
(*RET) = _4.3 as i16;
(*RET) = !_2;
_13.3 = [_15,_15,_15,_15,_15,_15,_15,_15];
_4.0 = _5;
_6 = _1;
RET = core::ptr::addr_of_mut!((*RET));
_13.1 = [_15,_15,_15];
_13.1 = [_15,_15,_15];
match _4.1 {
0 => bb1,
1 => bb4,
2 => bb8,
3 => bb9,
4 => bb10,
292966963449039967097221412708136529228 => bb12,
_ => bb11
}
}
bb8 = {
_11 = !_9;
RET = core::ptr::addr_of_mut!((*RET));
_4.0 = _13.5;
_13.2 = _7 as u8;
_16 = 57017_u16 & 23463_u16;
_13.1 = [_3,_15,_3];
(*RET) = _2;
_13.3 = [_3,_15,_3,_15,_15,_15,_3,_3];
_11 = _9;
(*RET) = _2;
Goto(bb7)
}
bb9 = {
_8 = -_7;
_11 = _13.2 | _9;
_16 = 38131_u16;
_8 = (*RET) as i128;
_13.0 = [(*RET),_2,_10,_2,(*RET),(*RET),_2];
_13.1 = [_3,_15,_15];
_13.4 = [_4.1,_4.1,_4.1,_4.1,_4.1,_4.1];
_4.0 = [_16,_16,_16,_16];
_13.1 = [_3,_15,_3];
RET = core::ptr::addr_of_mut!((*RET));
_13.5 = [_16,_16,_16,_16];
_13.2 = !_11;
Goto(bb6)
}
bb10 = {
_11 = _4.1 as u8;
_4.2 = _11;
_4.0 = [17555_u16,64389_u16,1850_u16,13819_u16];
_4.2 = 17768_u16 as u8;
_4.3 = _7 as f32;
_13.3 = [_3,_3,_3,_3,_3,_3,_3,_3];
(*RET) = !_2;
_13.2 = '\u{9c5d4}' as u8;
_3 = true;
_2 = (*RET) + (*RET);
_13.2 = _9 | _4.2;
_13.1 = [_3,_3,_3];
_4.3 = 25296_u16 as f32;
(*RET) = -_2;
(*RET) = _2;
_13.0 = [(*RET),(*RET),_2,_2,(*RET),_2,_10];
RET = core::ptr::addr_of_mut!((*RET));
_13.5 = [51783_u16,3209_u16,63744_u16,23848_u16];
_13.5 = _5;
_2 = !(*RET);
_7 = -_8;
_16 = 30330_u16;
_15 = _3;
match _4.1 {
0 => bb2,
1 => bb3,
292966963449039967097221412708136529228 => bb5,
_ => bb4
}
}
bb11 = {
Return()
}
bb12 = {
_23 = core::ptr::addr_of_mut!(_15);
_22 = 2_usize as f32;
_11 = !_4.2;
_2 = (*RET) ^ (*RET);
match _4.1 {
0 => bb1,
1 => bb10,
2 => bb3,
3 => bb11,
4 => bb5,
5 => bb13,
292966963449039967097221412708136529228 => bb15,
_ => bb14
}
}
bb13 = {
Return()
}
bb14 = {
_11 = !_9;
RET = core::ptr::addr_of_mut!((*RET));
_4.0 = _13.5;
_13.2 = _7 as u8;
_16 = 57017_u16 & 23463_u16;
_13.1 = [_3,_15,_3];
(*RET) = _2;
_13.3 = [_3,_15,_3,_15,_15,_15,_3,_3];
_11 = _9;
(*RET) = _2;
Goto(bb7)
}
bb15 = {
_12 = (-8677989751119654139_i64);
_13.3 = [(*_23),(*_23),(*_23),(*_23),(*_23),(*_23),(*_23),_15];
_12 = 13077441678484449491_usize as i64;
_4.2 = !_13.2;
_24 = _7 as f32;
RET = core::ptr::addr_of_mut!((*RET));
_4 = (_13.5, 44982602311553153056579945153039398436_u128, _13.2, _24);
(*_23) = _3;
_13.5 = [_16,_16,_16,_16];
_8 = 9223372036854775807_isize as i128;
Goto(bb16)
}
bb16 = {
Call(_26 = dump_var(6_usize, 11_usize, Move(_11), 8_usize, Move(_8), 2_usize, Move(_2), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_26 = dump_var(6_usize, 5_usize, Move(_5), 16_usize, Move(_16), 13_usize, Move(_13), 27_usize, _27), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: i64,mut _2: [usize; 4],mut _3: i64,mut _4: *mut [i8; 3],mut _5: i64,mut _6: [u32; 7],mut _7: bool,mut _8: Adt54) -> Adt59 {
mir! {
type RET = Adt59;
let _9: Adt45;
let _10: [i8; 3];
let _11: [u32; 7];
let _12: isize;
let _13: [u16; 6];
let _14: f64;
let _15: Adt60;
let _16: i16;
let _17: &'static f64;
let _18: bool;
let _19: u16;
let _20: Adt60;
let _21: i64;
let _22: u32;
let _23: Adt60;
let _24: i8;
let _25: [u128; 6];
let _26: Adt46;
let _27: Adt45;
let _28: f64;
let _29: Adt60;
let _30: u8;
let _31: ([usize; 4],);
let _32: *const ([usize; 4],);
let _33: Adt56;
let _34: Adt46;
let _35: Adt45;
let _36: isize;
let _37: char;
let _38: [u16; 4];
let _39: bool;
let _40: Adt45;
let _41: [i16; 1];
let _42: isize;
let _43: Adt49;
let _44: Adt45;
let _45: [bool; 3];
let _46: ([bool; 3], [i8; 3], i128, [i8; 3], i16);
let _47: bool;
let _48: u16;
let _49: Adt51;
let _50: Adt58;
let _51: usize;
let _52: i64;
let _53: f32;
let _54: Adt58;
let _55: f32;
let _56: *mut bool;
let _57: Adt45;
let _58: Adt54;
let _59: u8;
let _60: [u16; 4];
let _61: [bool; 5];
let _62: bool;
let _63: i64;
let _64: i16;
let _65: [u32; 7];
let _66: [u32; 7];
let _67: f64;
let _68: [usize; 4];
let _69: Adt50;
let _70: Adt54;
let _71: [u128; 6];
let _72: bool;
let _73: [bool; 8];
let _74: *mut [i8; 3];
let _75: [usize; 8];
let _76: f32;
let _77: Adt50;
let _78: f64;
let _79: ([usize; 4],);
let _80: [i16; 7];
let _81: ();
let _82: ();
{
_2 = _8.fld0.0;
_8.fld0.0 = [2_usize,6_usize,5_usize,2580904155376180651_usize];
_8.fld2 = (-74_isize);
_8.fld0.0 = [2_usize,7759864673260616417_usize,10718923049151673722_usize,2408753192700449775_usize];
_8.fld2 = (-68_isize) >> _1;
_2 = _8.fld0.0;
_6 = [3022750597_u32,1562901240_u32,1437813568_u32,3310507269_u32,2369580593_u32,2936083065_u32,661716789_u32];
(*_4) = [(-19_i8),59_i8,84_i8];
_5 = -_8.fld6;
_4 = core::ptr::addr_of_mut!((*_4));
_7 = !true;
_6 = [545380719_u32,388668420_u32,245987966_u32,3012269878_u32,3793637629_u32,2776335976_u32,936309047_u32];
_6 = [1022198142_u32,4026484041_u32,1338144995_u32,4246863596_u32,3382365774_u32,2148414069_u32,3597998452_u32];
_3 = _8.fld6 - _8.fld6;
_4 = core::ptr::addr_of_mut!((*_4));
_1 = 126104796602364690780316281041396017185_u128 as i64;
_8.fld5 = [103393912875764905820464207276308548198_u128,82724561532725434369086549856781730850_u128,220194090777906979054229956066026326895_u128,127545904231914581366717438457211713894_u128,94326760633663710705525267717263174889_u128,292931982975776602834837700224420456877_u128];
(*_4) = [(-51_i8),(-95_i8),(-125_i8)];
(*_4) = [(-82_i8),(-119_i8),100_i8];
(*_4) = [42_i8,(-5_i8),24_i8];
_6 = [3199051044_u32,2883993158_u32,683602210_u32,2297583289_u32,1821115258_u32,3330214348_u32,3878473676_u32];
_5 = _1 * _1;
_8.fld1 = 1329179704_i32 as u8;
_8.fld5 = [333950847941561025025645249570172236710_u128,158626903282185807164170187294654096422_u128,180978841959578599946547574193662265527_u128,254253773104932830992153169907000218212_u128,74661499317860208121512496058673576576_u128,316283155646465065485923038399305461961_u128];
_10 = [(-74_i8),65_i8,40_i8];
_8.fld2 = !9223372036854775807_isize;
_9.fld0 = !_8.fld4.fld0;
_8.fld2 = '\u{2d4b7}' as isize;
Goto(bb1)
}
bb1 = {
_9.fld0 = _8.fld4.fld0;
match _9.fld0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
50900 => bb9,
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
(*_4) = _10;
_9 = _8.fld4;
match _8.fld4.fld0 {
0 => bb8,
1 => bb5,
2 => bb6,
50900 => bb10,
_ => bb4
}
}
bb10 = {
_2 = [5_usize,6692998301771350561_usize,17868691558313263017_usize,5_usize];
_6 = [1951366111_u32,3705817874_u32,2274315895_u32,3393897427_u32,2733819540_u32,883512061_u32,1124294265_u32];
_8.fld0.0 = _2;
_9 = Adt45 { fld0: _8.fld4.fld0 };
_10 = [(-37_i8),(-46_i8),72_i8];
_6 = [881595163_u32,15070176_u32,3919116564_u32,2277619518_u32,1008338423_u32,3296383837_u32,1478925567_u32];
_3 = _5;
_8.fld2 = (-96_isize);
_6 = [4236679551_u32,2091751586_u32,3954284516_u32,1043147824_u32,1340161030_u32,3743941452_u32,2149436355_u32];
_8.fld4 = _9;
_9 = Adt45 { fld0: _8.fld4.fld0 };
_9 = _8.fld4;
_8.fld6 = _9.fld0 as i64;
_11 = [1977554323_u32,2269421316_u32,3770471559_u32,1963373480_u32,2014570573_u32,2490702475_u32,1418206519_u32];
_2 = [6_usize,1876507858103100993_usize,1_usize,17644618520419819493_usize];
_8.fld6 = 46782232844029282445742332545334933632_i128 as i64;
_3 = !_5;
_8.fld4.fld0 = 148503452307126677628834183170280032459_u128 as u16;
_12 = _8.fld2;
_8.fld1 = 59_u8 & 92_u8;
_8.fld1 = 59_u8 * 136_u8;
_8.fld4.fld0 = _9.fld0 / _9.fld0;
(*_4) = _10;
_11 = _6;
Goto(bb11)
}
bb11 = {
_13 = [_8.fld4.fld0,_8.fld4.fld0,_8.fld4.fld0,_8.fld4.fld0,_8.fld4.fld0,_8.fld4.fld0];
_11 = [3580209927_u32,2954523840_u32,3578856913_u32,627408042_u32,1840432673_u32,890809820_u32,1119069261_u32];
_1 = _3;
_14 = 12979330256238994012_u64 as f64;
_5 = _1 << _8.fld1;
_8.fld0 = (_2,);
_3 = _8.fld2 as i64;
_9.fld0 = _8.fld4.fld0;
_6 = [110873257_u32,2246729160_u32,130150432_u32,199779644_u32,473978245_u32,2720592681_u32,2281912691_u32];
_3 = _5;
_10 = [(-119_i8),(-102_i8),(-16_i8)];
_3 = !_5;
_8.fld0.0 = _2;
_7 = false;
_8.fld0.0 = [1_usize,0_usize,4_usize,13407031353226396714_usize];
_8.fld5 = [276298046225475819398044469953726834353_u128,38310459476035984586850161552311877822_u128,138132044392789232385035067718016242509_u128,103251215960670508751927339577394874804_u128,142485603844227206955966630227913968910_u128,202170457046298260567866672264319933074_u128];
_16 = !(-26842_i16);
_14 = 153230009948542050071834554445260843489_u128 as f64;
_6 = [2772980366_u32,2477233178_u32,1672247939_u32,468936474_u32,924848805_u32,1045235194_u32,4058293363_u32];
_2 = [3094533044928770989_usize,1_usize,3_usize,3500780573323420479_usize];
_2 = [7550557281833128208_usize,6_usize,2_usize,0_usize];
(*_4) = _10;
_7 = _8.fld2 != _12;
match _12 {
0 => bb1,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
340282366920938463463374607431768211360 => bb17,
_ => bb16
}
}
bb12 = {
Return()
}
bb13 = {
(*_4) = _10;
_9 = _8.fld4;
match _8.fld4.fld0 {
0 => bb8,
1 => bb5,
2 => bb6,
50900 => bb10,
_ => bb4
}
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
_13 = [_8.fld4.fld0,_8.fld4.fld0,_8.fld4.fld0,_9.fld0,_8.fld4.fld0,_9.fld0];
Goto(bb18)
}
bb18 = {
_9.fld0 = 772550281_u32 as u16;
_12 = _8.fld2;
_13 = [_8.fld4.fld0,_9.fld0,_9.fld0,_9.fld0,_8.fld4.fld0,_8.fld4.fld0];
_8.fld6 = -_3;
_11 = [1348520889_u32,2891149780_u32,4262708802_u32,1534367851_u32,1444891088_u32,4176396819_u32,2402272247_u32];
_2 = [8197978551546425032_usize,10185988279935496413_usize,1432034357633110421_usize,6_usize];
match _12 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb9,
340282366920938463463374607431768211360 => bb19,
_ => bb14
}
}
bb19 = {
_4 = core::ptr::addr_of_mut!((*_4));
_8.fld0 = (_2,);
_9.fld0 = _8.fld4.fld0;
_12 = _8.fld2 >> _5;
_2 = [9470007771409218618_usize,1_usize,0_usize,14434952181750145985_usize];
_8.fld5 = [282166001511773361840493380376314791363_u128,303718668303029559677635363165771199748_u128,90946953899070603301140369898985871870_u128,278532545027079441494012469879026467956_u128,221573572531914607692027507268308555189_u128,3274099351667622481936659865825677943_u128];
_9.fld0 = 24891032707938222197459885033026680320_i128 as u16;
_8.fld4 = Adt45 { fld0: _9.fld0 };
(*_4) = [(-43_i8),(-74_i8),(-53_i8)];
(*_4) = [(-31_i8),39_i8,(-113_i8)];
_17 = &_14;
_9.fld0 = _8.fld4.fld0;
_6 = [4107957092_u32,4247525315_u32,684128680_u32,865046105_u32,1881566686_u32,3728631019_u32,1842917743_u32];
_8.fld0.0 = _2;
_2 = [3_usize,0_usize,3_usize,8072435248625604478_usize];
_8.fld5 = [139426270901018693490026387150941951014_u128,18773303471098393651482243620769040632_u128,10816092876249149705188788645148817772_u128,86026076647971218103455099266983403924_u128,208920819750663310026920970245528960015_u128,98642799070786295581851203045858172322_u128];
_9 = Adt45 { fld0: _8.fld4.fld0 };
_8.fld1 = 130_u8 - 50_u8;
match _8.fld2 {
0 => bb20,
340282366920938463463374607431768211360 => bb22,
_ => bb21
}
}
bb20 = {
_2 = [5_usize,6692998301771350561_usize,17868691558313263017_usize,5_usize];
_6 = [1951366111_u32,3705817874_u32,2274315895_u32,3393897427_u32,2733819540_u32,883512061_u32,1124294265_u32];
_8.fld0.0 = _2;
_9 = Adt45 { fld0: _8.fld4.fld0 };
_10 = [(-37_i8),(-46_i8),72_i8];
_6 = [881595163_u32,15070176_u32,3919116564_u32,2277619518_u32,1008338423_u32,3296383837_u32,1478925567_u32];
_3 = _5;
_8.fld2 = (-96_isize);
_6 = [4236679551_u32,2091751586_u32,3954284516_u32,1043147824_u32,1340161030_u32,3743941452_u32,2149436355_u32];
_8.fld4 = _9;
_9 = Adt45 { fld0: _8.fld4.fld0 };
_9 = _8.fld4;
_8.fld6 = _9.fld0 as i64;
_11 = [1977554323_u32,2269421316_u32,3770471559_u32,1963373480_u32,2014570573_u32,2490702475_u32,1418206519_u32];
_2 = [6_usize,1876507858103100993_usize,1_usize,17644618520419819493_usize];
_8.fld6 = 46782232844029282445742332545334933632_i128 as i64;
_3 = !_5;
_8.fld4.fld0 = 148503452307126677628834183170280032459_u128 as u16;
_12 = _8.fld2;
_8.fld1 = 59_u8 & 92_u8;
_8.fld1 = 59_u8 * 136_u8;
_8.fld4.fld0 = _9.fld0 / _9.fld0;
(*_4) = _10;
_11 = _6;
Goto(bb11)
}
bb21 = {
Return()
}
bb22 = {
_14 = _12 as f64;
_1 = !_8.fld6;
_13 = [_8.fld4.fld0,_8.fld4.fld0,_9.fld0,_8.fld4.fld0,_8.fld4.fld0,_9.fld0];
_8.fld6 = _16 as i64;
(*_4) = [(-31_i8),76_i8,124_i8];
_13 = [_8.fld4.fld0,_9.fld0,_9.fld0,_8.fld4.fld0,_8.fld4.fld0,_9.fld0];
_9.fld0 = _12 as u16;
_12 = -_8.fld2;
_18 = _8.fld1 == _8.fld1;
(*_4) = [102_i8,(-7_i8),114_i8];
(*_4) = [59_i8,97_i8,119_i8];
_4 = core::ptr::addr_of_mut!((*_4));
_5 = !_3;
_8.fld5 = [195701109540726278710978079617568607608_u128,302080598203250669787289261871051054637_u128,268670610746398130451969187395860089160_u128,27122963177959437534821782624417048120_u128,94618063587368170582990685047854073216_u128,335283489358546047966074204583932680855_u128];
Call(_21 = fn8(_8, _8.fld4, _1, _4, _4, _5, _8.fld1, (*_4), _2, _8.fld5, _12, _13, _8.fld1, _11, _3, _8), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
_9.fld0 = _8.fld4.fld0 & _8.fld4.fld0;
_8.fld0 = (_2,);
_5 = -_1;
_9.fld0 = 4241458136062936863_u64 as u16;
_8.fld0 = (_2,);
match _8.fld2 {
0 => bb15,
1 => bb8,
2 => bb17,
3 => bb5,
4 => bb24,
340282366920938463463374607431768211360 => bb26,
_ => bb25
}
}
bb24 = {
Return()
}
bb25 = {
Return()
}
bb26 = {
_19 = 1420937938_i32 as u16;
_8.fld0 = (_2,);
_9.fld0 = !_8.fld4.fld0;
_8.fld4.fld0 = _9.fld0 >> _21;
_8.fld1 = 4592709146463085892_u64 as u8;
_16 = -15810_i16;
(*_4) = _10;
_9.fld0 = _8.fld4.fld0;
_22 = 1190420732_u32;
_16 = -(-27978_i16);
Goto(bb27)
}
bb27 = {
_6 = _11;
_5 = _1 + _3;
_9.fld0 = _19;
_24 = 1179927630_i32 as i8;
_14 = _24 as f64;
_25 = [86028383521809991538890541813695953563_u128,135872916347571871414111522270194568318_u128,331183986878499413768940276998714759625_u128,99303558738392022589076368266101182751_u128,241689016281182439558358815218550339744_u128,327891067268947482607648289715763748430_u128];
match _22 {
0 => bb5,
1 => bb6,
2 => bb19,
3 => bb4,
4 => bb28,
1190420732 => bb30,
_ => bb29
}
}
bb28 = {
Return()
}
bb29 = {
Return()
}
bb30 = {
_11 = _6;
_8.fld1 = !255_u8;
_1 = _5 * _3;
_17 = &_14;
_9.fld0 = _8.fld4.fld0;
_16 = '\u{be24f}' as i16;
_4 = core::ptr::addr_of_mut!(_10);
_16 = -(-15976_i16);
_8.fld4 = Adt45 { fld0: _9.fld0 };
_8.fld6 = _21;
_10 = [_24,_24,_24];
_10 = [_24,_24,_24];
_5 = !_1;
_22 = 3610253741494989437_u64 as u32;
_4 = core::ptr::addr_of_mut!((*_4));
_8.fld4.fld0 = _8.fld2 as u16;
_12 = _8.fld2;
(*_4) = [_24,_24,_24];
Call(_5 = core::intrinsics::bswap(_8.fld6), ReturnTo(bb31), UnwindUnreachable())
}
bb31 = {
_8.fld2 = -_12;
_2 = [12762764108775955094_usize,7113096544223592449_usize,12389379944126327160_usize,78810312280088943_usize];
_26.fld3 = -_24;
_8.fld4 = _9;
_8.fld1 = 193_u8;
_2 = _8.fld0.0;
_9.fld0 = _8.fld4.fld0;
(*_4) = [_26.fld3,_24,_24];
_18 = _9.fld0 < _8.fld4.fld0;
_12 = _8.fld2 << _21;
_4 = core::ptr::addr_of_mut!(_10);
_26.fld2.3 = [_7,_18,_18,_18,_18,_18,_18,_7];
_22 = !3858546845_u32;
_16 = !(-9178_i16);
_8.fld4.fld0 = _9.fld0 - _9.fld0;
_8.fld2 = 7_usize as isize;
_26.fld2.0 = [_16,_16,_16,_16,_16,_16,_16];
_14 = _8.fld1 as f64;
_27 = Adt45 { fld0: _9.fld0 };
_26.fld2.1 = [_18,_18,_18];
_26.fld1 = 1_usize;
_8.fld0 = (_2,);
_6 = [_22,_22,_22,_22,_22,_22,_22];
_24 = _26.fld3 & _26.fld3;
_27.fld0 = 228039504695164693602626293742178225809_u128 as u16;
_26.fld3 = _24 | _24;
match _8.fld1 {
0 => bb9,
1 => bb2,
2 => bb29,
3 => bb22,
4 => bb14,
5 => bb23,
193 => bb33,
_ => bb32
}
}
bb32 = {
Return()
}
bb33 = {
_22 = !3784640458_u32;
_26.fld2.5 = [_8.fld4.fld0,_8.fld4.fld0,_8.fld4.fld0,_9.fld0];
_5 = _1 ^ _21;
_26.fld2.5 = [_8.fld4.fld0,_19,_9.fld0,_9.fld0];
Goto(bb34)
}
bb34 = {
_25 = [57484528447146719526969820286532992592_u128,32303583407173931025173042313562267111_u128,266849955569693949328020718609764802506_u128,1134537709875668402887039533496915909_u128,128968255000406332953022913100549346949_u128,69347927647338566106273927202326523398_u128];
_8.fld0.0 = [_26.fld1,_26.fld1,_26.fld1,_26.fld1];
_33.fld3.fld3.1 = [_26.fld1,_26.fld1,_26.fld1,_26.fld1,_26.fld1,_26.fld1,_26.fld1,_26.fld1];
_33.fld0.0 = [_26.fld1,_26.fld1,_26.fld1,_26.fld1];
_10 = [_26.fld3,_26.fld3,_26.fld3];
_23 = Adt60::Variant1 { fld0: _26.fld2.0,fld1: _8.fld1,fld2: _33.fld3.fld3.1 };
_28 = 901094619_i32 as f64;
_33.fld4 = _16;
_33.fld5.fld2.2 = _8.fld1;
_33.fld5.fld2.5 = [_8.fld4.fld0,_8.fld4.fld0,_9.fld0,_8.fld4.fld0];
_26.fld0 = core::ptr::addr_of_mut!(_16);
_34.fld2.3 = [_7,_18,_18,_18,_18,_18,_18,_18];
SetDiscriminant(_23, 0);
_34.fld2 = (_26.fld2.0, _26.fld2.1, _33.fld5.fld2.2, _26.fld2.3, _8.fld5, _26.fld2.5);
place!(Field::<Adt48>(Variant(_23, 0), 1)).fld6 = _13;
place!(Field::<Adt51>(Variant(_23, 0), 4)).fld3.2 = _4;
_32 = core::ptr::addr_of!(_31);
place!(Field::<Adt48>(Variant(_23, 0), 1)).fld1.4 = [59126858765562117422570012366505531630_u128,176702204470988663320037477215447791020_u128,141046487868059584786270310539547295190_u128,291281563382382761761189244203137048350_u128,336756441293406813169813140305918704136_u128,225151994358144696974183849251308460710_u128];
place!(Field::<Adt51>(Variant(_23, 0), 4)).fld0.1 = 174450553588233226653078738468654998100_u128 << _26.fld1;
_8.fld0.0 = [_26.fld1,_26.fld1,_26.fld1,_26.fld1];
_36 = _12;
match _26.fld1 {
0 => bb29,
2 => bb3,
3 => bb28,
4 => bb11,
5 => bb6,
1 => bb36,
_ => bb35
}
}
bb35 = {
_9.fld0 = _8.fld4.fld0 & _8.fld4.fld0;
_8.fld0 = (_2,);
_5 = -_1;
_9.fld0 = 4241458136062936863_u64 as u16;
_8.fld0 = (_2,);
match _8.fld2 {
0 => bb15,
1 => bb8,
2 => bb17,
3 => bb5,
4 => bb24,
340282366920938463463374607431768211360 => bb26,
_ => bb25
}
}
bb36 = {
_33.fld1 = _26.fld0;
_35.fld0 = _8.fld4.fld0;
place!(Field::<Adt51>(Variant(_23, 0), 4)).fld0.0 = _34.fld2.5;
place!(Field::<Adt51>(Variant(_23, 0), 4)).fld3.1 = [_26.fld1,_26.fld1,_26.fld1,_26.fld1,_26.fld1,_26.fld1,_26.fld1,_26.fld1];
(*_32) = (_33.fld0.0,);
place!(Field::<Adt48>(Variant(_23, 0), 1)).fld3 = _14 as f32;
Goto(bb37)
}
bb37 = {
place!(Field::<Adt48>(Variant(_23, 0), 1)).fld1.4 = [Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1];
_34.fld2.5 = [_8.fld4.fld0,_35.fld0,_8.fld4.fld0,_8.fld4.fld0];
place!(Field::<Adt49>(Variant(_23, 0), 0)) = Adt49::Variant0 { fld0: _34.fld2.2 };
_34.fld2.4 = [Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1];
_33.fld3.fld3.0 = [_33.fld4,_33.fld4,_16,_33.fld4,_33.fld4,_33.fld4,_16];
place!(Field::<i64>(Variant(_23, 0), 5)) = _5 << _12;
_33.fld5.fld2.5 = [_8.fld4.fld0,_9.fld0,_35.fld0,_35.fld0];
place!(Field::<Adt51>(Variant(_23, 0), 4)).fld3.2 = _4;
_33.fld0.0 = [_26.fld1,_26.fld1,_26.fld1,_26.fld1];
_26.fld2.5 = [_8.fld4.fld0,_8.fld4.fld0,_8.fld4.fld0,_8.fld4.fld0];
_33.fld5.fld1 = _26.fld1 << _21;
_6 = _11;
_33.fld5.fld2.1 = [_18,_18,_18];
place!(Field::<Adt48>(Variant(_23, 0), 1)).fld1.2 = _16 as u8;
_40 = Adt45 { fld0: _8.fld4.fld0 };
(*_32).0 = [_33.fld5.fld1,_33.fld5.fld1,_33.fld5.fld1,_33.fld5.fld1];
place!(Field::<Adt48>(Variant(_23, 0), 1)).fld1.1 = _33.fld5.fld2.1;
_33.fld3.fld2 = Field::<Adt51>(Variant(_23, 0), 4).fld0.1 as i16;
match _26.fld1 {
0 => bb20,
1 => bb38,
_ => bb26
}
}
bb38 = {
_33.fld5.fld2.4 = [Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1];
(*_32) = _8.fld0;
Call(_16 = core::intrinsics::bswap(_33.fld4), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
_33.fld1 = core::ptr::addr_of_mut!(_33.fld4);
_26.fld2.0 = [_33.fld3.fld2,_33.fld4,_16,_33.fld3.fld2,_33.fld3.fld2,_33.fld3.fld2,_33.fld4];
(*_32).0 = [_33.fld5.fld1,_33.fld5.fld1,_26.fld1,_33.fld5.fld1];
place!(Field::<Adt48>(Variant(_23, 0), 1)).fld1 = _34.fld2;
_40 = Adt45 { fld0: _9.fld0 };
(*_32).0 = [_33.fld5.fld1,_33.fld5.fld1,_33.fld5.fld1,_33.fld5.fld1];
_34.fld2.3 = [_18,_18,_18,_18,_7,_18,_18,_18];
_26.fld2.0 = [_16,_33.fld4,_33.fld3.fld2,_33.fld4,_16,_33.fld3.fld2,_33.fld4];
_1 = _21;
place!(Field::<Adt48>(Variant(_23, 0), 1)).fld1.3 = [_18,_18,_18,_18,_18,_18,_18,_18];
_26.fld1 = _26.fld3 as usize;
SetDiscriminant(Field::<Adt49>(Variant(_23, 0), 0), 0);
_35 = _8.fld4;
Goto(bb40)
}
bb40 = {
_33.fld3.fld0.3 = 110818784_i32 as f32;
_8.fld0.0 = [_33.fld5.fld1,_33.fld5.fld1,_33.fld5.fld1,_33.fld5.fld1];
place!(Field::<Adt51>(Variant(_23, 0), 4)).fld3.2 = core::ptr::addr_of_mut!(_46.3);
_33.fld0.0 = [_33.fld5.fld1,_33.fld5.fld1,_33.fld5.fld1,_33.fld5.fld1];
_2 = [_33.fld5.fld1,_33.fld5.fld1,_33.fld5.fld1,_33.fld5.fld1];
_34.fld2 = (Field::<Adt48>(Variant(_23, 0), 1).fld1.0, _33.fld5.fld2.1, _8.fld1, Field::<Adt48>(Variant(_23, 0), 1).fld1.3, _8.fld5, _33.fld5.fld2.5);
place!(Field::<Adt51>(Variant(_23, 0), 4)).fld0.0 = [_40.fld0,_35.fld0,_40.fld0,_35.fld0];
_33.fld4 = !_16;
_28 = _14 - _14;
_24 = _26.fld3;
_33.fld3.fld0.0 = Field::<Adt48>(Variant(_23, 0), 1).fld1.5;
_33.fld5.fld2 = (_26.fld2.0, _34.fld2.1, Field::<Adt48>(Variant(_23, 0), 1).fld1.2, Field::<Adt48>(Variant(_23, 0), 1).fld1.3, _25, _34.fld2.5);
_37 = '\u{cfac4}';
_8.fld2 = _36;
_39 = _18;
_46.4 = _33.fld4 | _33.fld4;
place!(Field::<Adt48>(Variant(_23, 0), 1)).fld3 = _33.fld3.fld0.3 + _33.fld3.fld0.3;
_33.fld0.0 = [_26.fld1,_33.fld5.fld1,_33.fld5.fld1,_33.fld5.fld1];
_31.0 = _33.fld0.0;
_18 = !_39;
_44 = _9;
match _33.fld5.fld2.2 {
0 => bb6,
193 => bb42,
_ => bb41
}
}
bb41 = {
_22 = !3784640458_u32;
_26.fld2.5 = [_8.fld4.fld0,_8.fld4.fld0,_8.fld4.fld0,_9.fld0];
_5 = _1 ^ _21;
_26.fld2.5 = [_8.fld4.fld0,_19,_9.fld0,_9.fld0];
Goto(bb34)
}
bb42 = {
place!(Field::<u8>(Variant(place!(Field::<Adt49>(Variant(_23, 0), 0)), 0), 0)) = Field::<Adt48>(Variant(_23, 0), 1).fld1.2;
place!(Field::<Adt51>(Variant(_23, 0), 4)).fld0.1 = _33.fld5.fld2.2 as u128;
place!(Field::<Adt53>(Variant(_23, 0), 2)) = Adt53::Variant2 { fld0: _34.fld2.4 };
_42 = (-31724843464496359416526981179524884446_i128) as isize;
_14 = _28 - _28;
_47 = Field::<i64>(Variant(_23, 0), 5) != _5;
_19 = !_8.fld4.fld0;
_10 = [_24,_26.fld3,_26.fld3];
place!(Field::<Adt51>(Variant(_23, 0), 4)).fld0.2 = !Field::<Adt48>(Variant(_23, 0), 1).fld1.2;
_34.fld2.4 = _33.fld5.fld2.4;
_33.fld5.fld2.2 = _22 as u8;
_26.fld2.1 = Field::<Adt48>(Variant(_23, 0), 1).fld1.1;
_49.fld3.1 = _33.fld3.fld3.1;
Goto(bb43)
}
bb43 = {
_33.fld2 = (*_32).0;
_25 = [Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1];
_49.fld0.2 = _34.fld2.2;
_8.fld0 = (_2,);
_33.fld5.fld0 = _33.fld1;
_9 = Adt45 { fld0: _19 };
_32 = core::ptr::addr_of!((*_32));
_53 = Field::<Adt48>(Variant(_23, 0), 1).fld3 - Field::<Adt48>(Variant(_23, 0), 1).fld3;
_37 = '\u{b7040}';
_33.fld5.fld2.0 = [_46.4,_33.fld4,_46.4,_33.fld4,_46.4,_46.4,_33.fld3.fld2];
_26.fld2.5 = [_19,_19,_35.fld0,_19];
_26 = Adt46 { fld0: _33.fld1,fld1: _33.fld5.fld1,fld2: _34.fld2,fld3: _24 };
_45 = _33.fld5.fld2.1;
_33.fld5.fld1 = _53 as usize;
SetDiscriminant(Field::<Adt49>(Variant(_23, 0), 0), 2);
_56 = core::ptr::addr_of_mut!(_39);
_33.fld3.fld1 = _11;
_35 = Adt45 { fld0: _8.fld4.fld0 };
_49.fld0.1 = _46.4 as u128;
place!(Field::<Adt51>(Variant(_23, 0), 4)).fld0.2 = _14 as u8;
SetDiscriminant(Field::<Adt53>(Variant(_23, 0), 2), 0);
Goto(bb44)
}
bb44 = {
_34.fld2.2 = !Field::<Adt51>(Variant(_23, 0), 4).fld0.2;
_21 = _9.fld0 as i64;
_51 = _33.fld5.fld1;
_32 = core::ptr::addr_of!(_58.fld0);
place!(Field::<Adt51>(Variant(_23, 0), 4)).fld0.3 = _53;
place!(Field::<Adt51>(Variant(_23, 0), 4)).fld0.2 = !_34.fld2.2;
_46.0 = [_39,_47,(*_56)];
_8.fld0 = (_33.fld0.0,);
_17 = &_28;
_26.fld0 = core::ptr::addr_of_mut!(_33.fld4);
_34.fld2.0 = [_16,_33.fld3.fld2,_46.4,_33.fld3.fld2,_33.fld3.fld2,_33.fld4,_33.fld4];
_8.fld0.0 = [_26.fld1,_26.fld1,_33.fld5.fld1,_26.fld1];
_49.fld0.1 = _37 as u128;
_8.fld1 = _26.fld2.2;
_51 = _26.fld1;
_33.fld2 = [_51,_26.fld1,_26.fld1,_26.fld1];
_58.fld0 = (_33.fld0.0,);
place!(Field::<i64>(Variant(_23, 0), 5)) = _1;
_46.2 = (-94587262434999899639197392291299891201_i128);
_58.fld4.fld0 = _22 as u16;
_34.fld2.2 = !Field::<Adt51>(Variant(_23, 0), 4).fld0.2;
_60 = [_35.fld0,_58.fld4.fld0,_35.fld0,_35.fld0];
_26.fld2.0 = _34.fld2.0;
_2 = [_51,_51,_33.fld5.fld1,_26.fld1];
_49.fld3.0 = [_46.4,_46.4,_46.4,_46.4,_33.fld3.fld2,_16,_46.4];
_12 = _8.fld2 >> _8.fld2;
Goto(bb45)
}
bb45 = {
place!(Field::<Adt51>(Variant(_23, 0), 4)).fld1 = [_22,_22,_22,_22,_22,_22,_22];
_8.fld3 = core::ptr::addr_of_mut!(_49.fld0);
_8.fld5 = [Field::<Adt51>(Variant(_23, 0), 4).fld0.1,_49.fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,_49.fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,_49.fld0.1];
place!(Field::<i128>(Variant(place!(Field::<Adt49>(Variant(_23, 0), 0)), 2), 0)) = _46.2 - _46.2;
_58 = Adt54 { fld0: _33.fld0,fld1: _34.fld2.2,fld2: _36,fld3: _8.fld3,fld4: _9,fld5: _34.fld2.4,fld6: _5 };
(*_32) = _31;
_33.fld3.fld3.2 = _4;
_33.fld5.fld3 = _8.fld4.fld0 as i8;
_57 = Adt45 { fld0: _8.fld4.fld0 };
_49 = Adt51 { fld0: Field::<Adt51>(Variant(_23, 0), 4).fld0,fld1: _6,fld2: _46.4,fld3: _33.fld3.fld3 };
_17 = &(*_17);
(*_56) = _36 > _12;
_64 = !_46.4;
place!(Field::<Adt48>(Variant(_23, 0), 1)).fld4 = [_49.fld2,_33.fld3.fld2,_49.fld2,_33.fld4,_49.fld2,_64,_49.fld2];
Goto(bb46)
}
bb46 = {
_33.fld3.fld0.2 = Field::<Adt51>(Variant(_23, 0), 4).fld0.2;
place!(Field::<Adt51>(Variant(_23, 0), 4)).fld3 = (Field::<Adt48>(Variant(_23, 0), 1).fld4, _33.fld3.fld3.1, _33.fld3.fld3.2);
_49.fld2 = _16;
_57 = Adt45 { fld0: _58.fld4.fld0 };
_36 = _64 as isize;
place!(Field::<Adt55>(Variant(_23, 0), 3)) = Adt55::Variant1 { fld0: _49.fld3.2 };
_51 = _26.fld1 - _26.fld1;
place!(Field::<Adt51>(Variant(_23, 0), 4)).fld1 = [_22,_22,_22,_22,_22,_22,_22];
place!(Field::<Adt48>(Variant(_23, 0), 1)).fld2 = _49.fld3.1;
_58.fld2 = _36;
SetDiscriminant(Field::<Adt55>(Variant(_23, 0), 3), 3);
_26.fld2.2 = _34.fld2.2;
_46.0 = _45;
place!(Field::<([i16; 7], [usize; 8], *mut [i8; 3])>(Variant(place!(Field::<Adt55>(Variant(_23, 0), 3)), 3), 2)) = (_26.fld2.0, Field::<Adt48>(Variant(_23, 0), 1).fld2, _33.fld3.fld3.2);
_34.fld2.3 = _33.fld5.fld2.3;
_26.fld2.1 = [(*_56),_39,_18];
_7 = _18 == (*_56);
Goto(bb47)
}
bb47 = {
place!(Field::<[usize; 8]>(Variant(place!(Field::<Adt49>(Variant(_23, 0), 0)), 2), 1)) = [_51,_51,_51,_51,_33.fld5.fld1,_26.fld1,_51,_51];
_58.fld4 = _57;
Call(place!(Field::<i64>(Variant(_23, 0), 5)) = core::intrinsics::transmute(_60), ReturnTo(bb48), UnwindUnreachable())
}
bb48 = {
_46.4 = _33.fld4;
_61 = [_39,_7,_39,_7,_39];
_46.0 = _45;
place!(Field::<Adt48>(Variant(place!(Field::<Adt55>(Variant(_23, 0), 3)), 3), 6)) = Adt48 { fld0: _58.fld3,fld1: _26.fld2,fld2: Field::<[usize; 8]>(Variant(Field::<Adt49>(Variant(_23, 0), 0), 2), 1),fld3: Field::<Adt48>(Variant(_23, 0), 1).fld3,fld4: _34.fld2.0,fld5: _61,fld6: Field::<Adt48>(Variant(_23, 0), 1).fld6 };
_67 = (*_17) * _28;
_30 = Field::<Adt51>(Variant(_23, 0), 4).fld0.2;
_33.fld3.fld2 = _49.fld0.1 as i16;
place!(Field::<f32>(Variant(place!(Field::<Adt53>(Variant(_23, 0), 2)), 0), 0)) = Field::<Adt51>(Variant(_23, 0), 4).fld0.3;
place!(Field::<Adt48>(Variant(_23, 0), 1)).fld0 = core::ptr::addr_of_mut!(_49.fld0);
SetDiscriminant(Field::<Adt49>(Variant(_23, 0), 0), 0);
_70.fld4 = _8.fld4;
_6 = [_22,_22,_22,_22,_22,_22,_22];
_33.fld1 = core::ptr::addr_of_mut!(_33.fld3.fld2);
_8.fld0 = ((*_32).0,);
_26.fld2.3 = Field::<Adt48>(Variant(_23, 0), 1).fld1.3;
_72 = _47 | _47;
_58 = Adt54 { fld0: _31,fld1: Field::<Adt48>(Variant(Field::<Adt55>(Variant(_23, 0), 3), 3), 6).fld1.2,fld2: _12,fld3: Field::<Adt48>(Variant(_23, 0), 1).fld0,fld4: _35,fld5: Field::<Adt48>(Variant(Field::<Adt55>(Variant(_23, 0), 3), 3), 6).fld1.4,fld6: _5 };
place!(Field::<[bool; 5]>(Variant(place!(Field::<Adt53>(Variant(_23, 0), 2)), 0), 1)) = _61;
place!(Field::<Adt48>(Variant(_23, 0), 1)).fld1.5 = _34.fld2.5;
place!(Field::<[u128; 6]>(Variant(place!(Field::<Adt55>(Variant(_23, 0), 3)), 3), 0)) = [_49.fld0.1,_49.fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1];
_70.fld1 = _33.fld3.fld0.2 << _33.fld5.fld1;
_19 = Field::<Adt51>(Variant(_23, 0), 4).fld0.1 as u16;
_70 = _58;
_24 = _5 as i8;
_8.fld0.0 = [_26.fld1,_51,_51,_26.fld1];
_35.fld0 = _70.fld4.fld0 + _57.fld0;
_8.fld2 = !_12;
_49.fld3 = (_33.fld5.fld2.0, Field::<([i16; 7], [usize; 8], *mut [i8; 3])>(Variant(Field::<Adt55>(Variant(_23, 0), 3), 3), 2).1, Field::<([i16; 7], [usize; 8], *mut [i8; 3])>(Variant(Field::<Adt55>(Variant(_23, 0), 3), 3), 2).2);
_38 = [_35.fld0,_58.fld4.fld0,_44.fld0,_9.fld0];
Goto(bb49)
}
bb49 = {
_34.fld1 = _51;
_55 = Field::<f32>(Variant(Field::<Adt53>(Variant(_23, 0), 2), 0), 0) - Field::<Adt51>(Variant(_23, 0), 4).fld0.3;
_26.fld2.0 = [_33.fld3.fld2,_64,_64,_64,_33.fld4,_33.fld4,_33.fld3.fld2];
place!(Field::<Adt48>(Variant(place!(Field::<Adt55>(Variant(_23, 0), 3)), 3), 6)).fld1.5 = [_35.fld0,_70.fld4.fld0,_44.fld0,_40.fld0];
_72 = !_47;
_63 = !_70.fld6;
_33.fld5.fld1 = !_51;
_18 = _58.fld1 <= _49.fld0.2;
_22 = 3687567056_u32 * 1913627509_u32;
place!(Field::<Adt48>(Variant(_23, 0), 1)).fld1.5 = [_8.fld4.fld0,_57.fld0,_35.fld0,_44.fld0];
place!(Field::<Adt48>(Variant(_23, 0), 1)).fld1.4 = [Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,_49.fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,_49.fld0.1,_49.fld0.1];
_49.fld3.0 = [_46.4,_46.4,_64,_33.fld4,_33.fld4,_64,_49.fld2];
place!(Field::<Adt45>(Variant(place!(Field::<Adt55>(Variant(_23, 0), 3)), 3), 4)) = Adt45 { fld0: _35.fld0 };
_49.fld0.1 = (-1578937890_i32) as u128;
_33.fld3 = Adt51 { fld0: _49.fld0,fld1: _49.fld1,fld2: _64,fld3: Field::<([i16; 7], [usize; 8], *mut [i8; 3])>(Variant(Field::<Adt55>(Variant(_23, 0), 3), 3), 2) };
place!(Field::<u8>(Variant(place!(Field::<Adt49>(Variant(_23, 0), 0)), 0), 0)) = _26.fld2.2;
place!(Field::<Adt48>(Variant(_23, 0), 1)).fld1.4 = [Field::<Adt51>(Variant(_23, 0), 4).fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,_33.fld3.fld0.1,_33.fld3.fld0.1,Field::<Adt51>(Variant(_23, 0), 4).fld0.1,_49.fld0.1];
_58 = Adt54 { fld0: _31,fld1: Field::<Adt51>(Variant(_23, 0), 4).fld0.2,fld2: _12,fld3: _8.fld3,fld4: _8.fld4,fld5: _70.fld5,fld6: _21 };
place!(Field::<i64>(Variant(_23, 0), 5)) = _58.fld6 & _3;
_47 = _7;
_34.fld2.3 = [(*_56),_47,_7,_7,(*_56),_7,(*_56),(*_56)];
_38 = _49.fld0.0;
place!(Field::<Adt48>(Variant(place!(Field::<Adt55>(Variant(_23, 0), 3)), 3), 6)).fld1.3 = [_7,_7,_72,_39,_47,_72,_7,_72];
_33.fld5.fld2.1 = _26.fld2.1;
_70.fld5 = _58.fld5;
place!(Field::<Adt45>(Variant(place!(Field::<Adt55>(Variant(_23, 0), 3)), 3), 4)) = Adt45 { fld0: _44.fld0 };
place!(Field::<Adt51>(Variant(_23, 0), 4)).fld2 = !_33.fld3.fld2;
Goto(bb50)
}
bb50 = {
_41 = [_64];
_58.fld0.0 = [_26.fld1,_51,_33.fld5.fld1,_34.fld1];
_49.fld0.0 = [_9.fld0,_9.fld0,_35.fld0,_70.fld4.fld0];
_18 = !(*_56);
place!(Field::<f32>(Variant(place!(Field::<Adt53>(Variant(_23, 0), 2)), 0), 0)) = -Field::<Adt51>(Variant(_23, 0), 4).fld0.3;
place!(Field::<Adt53>(Variant(_23, 0), 2)) = Adt53::Variant0 { fld0: _55,fld1: _61,fld2: _33.fld5.fld2.1 };
_33 = Adt56 { fld0: (*_32),fld1: _26.fld0,fld2: (*_32).0,fld3: Move(_49),fld4: Field::<Adt51>(Variant(_23, 0), 4).fld2,fld5: Move(_26) };
Goto(bb51)
}
bb51 = {
_43 = Adt49::Variant1 { fld0: Field::<Adt48>(Variant(Field::<Adt55>(Variant(_23, 0), 3), 3), 6).fld1,fld1: _33.fld3.fld3.2,fld2: _45 };
_26.fld2.3 = [_7,_18,_47,_47,_72,_72,_7,_18];
_74 = core::ptr::addr_of_mut!(_10);
_34.fld2.3 = [_7,_47,_7,_18,_39,(*_56),(*_56),_47];
SetDiscriminant(Field::<Adt53>(Variant(_23, 0), 2), 0);
_40.fld0 = _35.fld0;
_69 = Adt50::Variant0 { fld0: Field::<*mut [i8; 3]>(Variant(_43, 1), 1),fld1: _22,fld2: Field::<Adt48>(Variant(Field::<Adt55>(Variant(_23, 0), 3), 3), 6).fld5,fld3: _26.fld2.3,fld4: _34.fld2 };
_26.fld2.2 = !Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_69, 0), 4).2;
_74 = Field::<*mut [i8; 3]>(Variant(_69, 0), 0);
_33.fld3 = Move(Field::<Adt51>(Variant(_23, 0), 4));
place!(Field::<[bool; 3]>(Variant(place!(Field::<Adt53>(Variant(_23, 0), 2)), 0), 2)) = [_47,(*_56),_47];
_13 = [Field::<Adt45>(Variant(Field::<Adt55>(Variant(_23, 0), 3), 3), 4).fld0,_40.fld0,_9.fld0,_8.fld4.fld0,Field::<Adt45>(Variant(Field::<Adt55>(Variant(_23, 0), 3), 3), 4).fld0,_8.fld4.fld0];
_26.fld2.5 = _33.fld3.fld0.0;
_1 = Field::<i64>(Variant(_23, 0), 5) * Field::<i64>(Variant(_23, 0), 5);
(*_32) = (_31.0,);
_34 = Move(_33.fld5);
_34 = Adt46 { fld0: _33.fld1,fld1: _51,fld2: Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_43, 1), 0),fld3: _24 };
_18 = !_47;
Goto(bb52)
}
bb52 = {
SetDiscriminant(_43, 1);
place!(Field::<Adt48>(Variant(_23, 0), 1)).fld1.4 = [_33.fld3.fld0.1,_33.fld3.fld0.1,_33.fld3.fld0.1,_33.fld3.fld0.1,_33.fld3.fld0.1,_33.fld3.fld0.1];
_70.fld1 = !Field::<Adt48>(Variant(Field::<Adt55>(Variant(_23, 0), 3), 3), 6).fld1.2;
_32 = core::ptr::addr_of!(_8.fld0);
RET = Adt59::Variant1 { fld0: Field::<Adt48>(Variant(_23, 0), 1).fld1,fld1: _8,fld2: (*_32),fld3: _32 };
place!(Field::<([usize; 4],)>(Variant(RET, 1), 2)) = (_70.fld0.0,);
Goto(bb53)
}
bb53 = {
Call(_81 = dump_var(7_usize, 42_usize, Move(_42), 1_usize, Move(_1), 60_usize, Move(_60), 38_usize, Move(_38)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_81 = dump_var(7_usize, 45_usize, Move(_45), 41_usize, Move(_41), 47_usize, Move(_47), 25_usize, Move(_25)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_81 = dump_var(7_usize, 2_usize, Move(_2), 13_usize, Move(_13), 63_usize, Move(_63), 3_usize, Move(_3)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_81 = dump_var(7_usize, 24_usize, Move(_24), 31_usize, Move(_31), 11_usize, Move(_11), 51_usize, Move(_51)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: Adt54,mut _2: Adt45,mut _3: i64,mut _4: *mut [i8; 3],mut _5: *mut [i8; 3],mut _6: i64,mut _7: u8,mut _8: [i8; 3],mut _9: [usize; 4],mut _10: [u128; 6],mut _11: isize,mut _12: [u16; 6],mut _13: u8,mut _14: [u32; 7],mut _15: i64,mut _16: Adt54) -> i64 {
mir! {
type RET = i64;
let _17: [i8; 3];
let _18: [u16; 6];
let _19: (f32, f64, u8);
let _20: u32;
let _21: &'static f64;
let _22: isize;
let _23: u32;
let _24: [i16; 7];
let _25: i64;
let _26: isize;
let _27: u64;
let _28: i128;
let _29: u128;
let _30: [u128; 6];
let _31: u8;
let _32: char;
let _33: [u128; 6];
let _34: [i64; 3];
let _35: f64;
let _36: char;
let _37: char;
let _38: Adt58;
let _39: bool;
let _40: f32;
let _41: u128;
let _42: f32;
let _43: i64;
let _44: u64;
let _45: char;
let _46: ();
let _47: ();
{
_9 = _1.fld0.0;
(*_4) = [67_i8,19_i8,(-28_i8)];
RET = _3;
_16.fld0.0 = _9;
_1.fld1 = _7 ^ _16.fld1;
_1.fld3 = _16.fld3;
_1.fld2 = 8993549367355622853_usize as isize;
(*_5) = [(-32_i8),6_i8,55_i8];
Call(RET = fn9((*_4), _16.fld2, _1.fld1, _16.fld5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_19.0 = _1.fld2 as f32;
_1.fld4.fld0 = _16.fld4.fld0 * _16.fld4.fld0;
_16.fld4.fld0 = _1.fld4.fld0;
_17 = (*_5);
_1.fld3 = _16.fld3;
_2.fld0 = !_16.fld4.fld0;
_8 = [(-125_i8),(-40_i8),127_i8];
_22 = 5_usize as isize;
_16.fld2 = _22;
_9 = _16.fld0.0;
RET = _6 & _3;
_10 = [44624840439401454602445618239695021759_u128,318807155856490026749899881485322751606_u128,263251840809968727843700027955394299622_u128,190523685923223979969412332507399429573_u128,24106349958868786295530130705362200133_u128,51720264902124397437582872852998465214_u128];
_1.fld4 = Adt45 { fld0: _2.fld0 };
_16.fld2 = _11;
_25 = _1.fld6;
_23 = 3903096625_u32;
_1.fld5 = [275468248237072052350526205964681103479_u128,181936093529315050668265663809415094761_u128,88464148493835113343812317847518396159_u128,245997478052726863537163375431884076936_u128,212721011307454518002133522106337471434_u128,104221319236320377997248040588985643248_u128];
_11 = -_1.fld2;
_16.fld4 = Adt45 { fld0: _2.fld0 };
Call(_16.fld1 = fn12(_16.fld0, _1, _1.fld5, _7, _2.fld0, _1.fld1, _10, _16.fld5, _16.fld3, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_16.fld3 = _1.fld3;
_1 = Adt54 { fld0: _16.fld0,fld1: _16.fld1,fld2: _22,fld3: _16.fld3,fld4: _2,fld5: _10,fld6: _15 };
_16.fld0 = _1.fld0;
_16.fld0.0 = [2_usize,5178991200932659257_usize,11667947055295405551_usize,5_usize];
_1 = Adt54 { fld0: _16.fld0,fld1: _16.fld1,fld2: _16.fld2,fld3: _16.fld3,fld4: _16.fld4,fld5: _10,fld6: _6 };
_4 = core::ptr::addr_of_mut!((*_5));
(*_5) = _8;
(*_4) = [(-60_i8),58_i8,91_i8];
_20 = _23;
_24 = [30509_i16,2614_i16,31397_i16,(-9058_i16),(-23082_i16),(-12307_i16),15379_i16];
_19.1 = _2.fld0 as f64;
_1 = Adt54 { fld0: _16.fld0,fld1: _13,fld2: _11,fld3: _16.fld3,fld4: _16.fld4,fld5: _16.fld5,fld6: _15 };
Call(_19.0 = core::intrinsics::transmute(_20), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_11 = 169574963157319794572014560888109266860_i128 as isize;
_16.fld4 = Adt45 { fld0: _1.fld4.fld0 };
RET = _6 - _15;
_12 = [_2.fld0,_1.fld4.fld0,_16.fld4.fld0,_16.fld4.fld0,_1.fld4.fld0,_1.fld4.fld0];
_1.fld4.fld0 = !_16.fld4.fld0;
_22 = 5303169429213070733_usize as isize;
_12 = [_16.fld4.fld0,_16.fld4.fld0,_16.fld4.fld0,_1.fld4.fld0,_1.fld4.fld0,_16.fld4.fld0];
_1.fld0.0 = [3_usize,7_usize,13624955587737637426_usize,5_usize];
Call(_16.fld4.fld0 = core::intrinsics::transmute(_2.fld0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_25 = _1.fld6 ^ _6;
_1.fld6 = _3 >> _16.fld1;
_5 = _4;
_16 = Adt54 { fld0: _1.fld0,fld1: _13,fld2: _1.fld2,fld3: _1.fld3,fld4: _1.fld4,fld5: _1.fld5,fld6: _3 };
_21 = &_19.1;
_1.fld2 = -_22;
_1.fld6 = !RET;
_1.fld4.fld0 = _2.fld0;
_19.1 = 4_usize as f64;
_32 = '\u{5754f}';
_16 = _1;
(*_4) = [(-30_i8),68_i8,(-59_i8)];
(*_4) = _8;
_14 = [_20,_20,_20,_20,_20,_23,_23];
_12 = [_2.fld0,_2.fld0,_16.fld4.fld0,_16.fld4.fld0,_2.fld0,_2.fld0];
_32 = '\u{a0b38}';
_21 = &_35;
_7 = false as u8;
_19.0 = _1.fld6 as f32;
_1.fld4.fld0 = !_2.fld0;
_27 = 13035174397559967449_u64 + 13099076912573574443_u64;
_35 = _19.1;
_31 = !_1.fld1;
Goto(bb5)
}
bb5 = {
(*_4) = [(-119_i8),(-24_i8),(-60_i8)];
(*_5) = [81_i8,85_i8,(-1_i8)];
_2 = Adt45 { fld0: _1.fld4.fld0 };
_16.fld4.fld0 = _19.1 as u16;
match _20 {
0 => bb1,
1 => bb4,
2 => bb6,
3903096625 => bb8,
_ => bb7
}
}
bb6 = {
_19.0 = _1.fld2 as f32;
_1.fld4.fld0 = _16.fld4.fld0 * _16.fld4.fld0;
_16.fld4.fld0 = _1.fld4.fld0;
_17 = (*_5);
_1.fld3 = _16.fld3;
_2.fld0 = !_16.fld4.fld0;
_8 = [(-125_i8),(-40_i8),127_i8];
_22 = 5_usize as isize;
_16.fld2 = _22;
_9 = _16.fld0.0;
RET = _6 & _3;
_10 = [44624840439401454602445618239695021759_u128,318807155856490026749899881485322751606_u128,263251840809968727843700027955394299622_u128,190523685923223979969412332507399429573_u128,24106349958868786295530130705362200133_u128,51720264902124397437582872852998465214_u128];
_1.fld4 = Adt45 { fld0: _2.fld0 };
_16.fld2 = _11;
_25 = _1.fld6;
_23 = 3903096625_u32;
_1.fld5 = [275468248237072052350526205964681103479_u128,181936093529315050668265663809415094761_u128,88464148493835113343812317847518396159_u128,245997478052726863537163375431884076936_u128,212721011307454518002133522106337471434_u128,104221319236320377997248040588985643248_u128];
_11 = -_1.fld2;
_16.fld4 = Adt45 { fld0: _2.fld0 };
Call(_16.fld1 = fn12(_16.fld0, _1, _1.fld5, _7, _2.fld0, _1.fld1, _10, _16.fld5, _16.fld3, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_16.fld3 = _1.fld3;
_1 = Adt54 { fld0: _16.fld0,fld1: _16.fld1,fld2: _22,fld3: _16.fld3,fld4: _2,fld5: _10,fld6: _15 };
_16.fld0 = _1.fld0;
_16.fld0.0 = [2_usize,5178991200932659257_usize,11667947055295405551_usize,5_usize];
_1 = Adt54 { fld0: _16.fld0,fld1: _16.fld1,fld2: _16.fld2,fld3: _16.fld3,fld4: _16.fld4,fld5: _10,fld6: _6 };
_4 = core::ptr::addr_of_mut!((*_5));
(*_5) = _8;
(*_4) = [(-60_i8),58_i8,91_i8];
_20 = _23;
_24 = [30509_i16,2614_i16,31397_i16,(-9058_i16),(-23082_i16),(-12307_i16),15379_i16];
_19.1 = _2.fld0 as f64;
_1 = Adt54 { fld0: _16.fld0,fld1: _13,fld2: _11,fld3: _16.fld3,fld4: _16.fld4,fld5: _16.fld5,fld6: _15 };
Call(_19.0 = core::intrinsics::transmute(_20), ReturnTo(bb3), UnwindUnreachable())
}
bb8 = {
_1.fld0 = (_9,);
_10 = [45514258855967394866243943087115594299_u128,65939731474088476741776202100160641668_u128,275973211117098487386673285738797150647_u128,233760976609668526348761047366274876577_u128,335582697749736901680291869205158435524_u128,25321035857507390317844744848932201323_u128];
_22 = _32 as isize;
_19.0 = 504275654_i32 as f32;
_1.fld5 = [69284834049432867961163599458586949597_u128,147728352306480241919467645088669868542_u128,222455764296889666352553714732445524600_u128,250787547919017234692501269149117020447_u128,140405867529427933586485453065047172180_u128,231247233030101466437529521326564316178_u128];
_1.fld6 = -_16.fld6;
_29 = 162187379635757828033012465975577643947_u128 ^ 86064236763496940749085117680019628439_u128;
_18 = _12;
_21 = &_19.1;
Goto(bb9)
}
bb9 = {
_19.0 = _29 as f32;
_4 = core::ptr::addr_of_mut!(_8);
_18 = _12;
_1.fld3 = _16.fld3;
_1.fld6 = _16.fld6;
(*_5) = [124_i8,85_i8,76_i8];
_39 = true;
_19.0 = 12404366682638491605_usize as f32;
_2.fld0 = _1.fld4.fld0 >> _7;
_36 = _32;
_27 = 3879499272957095522_u64;
_13 = _31 & _31;
_14 = [_20,_20,_23,_20,_20,_23,_20];
match _20 {
0 => bb7,
3903096625 => bb11,
_ => bb10
}
}
bb10 = {
_16.fld3 = _1.fld3;
_1 = Adt54 { fld0: _16.fld0,fld1: _16.fld1,fld2: _22,fld3: _16.fld3,fld4: _2,fld5: _10,fld6: _15 };
_16.fld0 = _1.fld0;
_16.fld0.0 = [2_usize,5178991200932659257_usize,11667947055295405551_usize,5_usize];
_1 = Adt54 { fld0: _16.fld0,fld1: _16.fld1,fld2: _16.fld2,fld3: _16.fld3,fld4: _16.fld4,fld5: _10,fld6: _6 };
_4 = core::ptr::addr_of_mut!((*_5));
(*_5) = _8;
(*_4) = [(-60_i8),58_i8,91_i8];
_20 = _23;
_24 = [30509_i16,2614_i16,31397_i16,(-9058_i16),(-23082_i16),(-12307_i16),15379_i16];
_19.1 = _2.fld0 as f64;
_1 = Adt54 { fld0: _16.fld0,fld1: _13,fld2: _11,fld3: _16.fld3,fld4: _16.fld4,fld5: _16.fld5,fld6: _15 };
Call(_19.0 = core::intrinsics::transmute(_20), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_4 = _5;
_1.fld6 = -_16.fld6;
_2 = Adt45 { fld0: _1.fld4.fld0 };
_30 = [_29,_29,_29,_29,_29,_29];
_26 = _1.fld2 >> _16.fld6;
_34 = [_15,RET,_16.fld6];
_33 = [_29,_29,_29,_29,_29,_29];
_19.2 = !_1.fld1;
_37 = _32;
match _27 {
0 => bb8,
1 => bb9,
2 => bb5,
3879499272957095522 => bb13,
_ => bb12
}
}
bb12 = {
_25 = _1.fld6 ^ _6;
_1.fld6 = _3 >> _16.fld1;
_5 = _4;
_16 = Adt54 { fld0: _1.fld0,fld1: _13,fld2: _1.fld2,fld3: _1.fld3,fld4: _1.fld4,fld5: _1.fld5,fld6: _3 };
_21 = &_19.1;
_1.fld2 = -_22;
_1.fld6 = !RET;
_1.fld4.fld0 = _2.fld0;
_19.1 = 4_usize as f64;
_32 = '\u{5754f}';
_16 = _1;
(*_4) = [(-30_i8),68_i8,(-59_i8)];
(*_4) = _8;
_14 = [_20,_20,_20,_20,_20,_23,_23];
_12 = [_2.fld0,_2.fld0,_16.fld4.fld0,_16.fld4.fld0,_2.fld0,_2.fld0];
_32 = '\u{a0b38}';
_21 = &_35;
_7 = false as u8;
_19.0 = _1.fld6 as f32;
_1.fld4.fld0 = !_2.fld0;
_27 = 13035174397559967449_u64 + 13099076912573574443_u64;
_35 = _19.1;
_31 = !_1.fld1;
Goto(bb5)
}
bb13 = {
_1.fld3 = _16.fld3;
_40 = _19.0 - _19.0;
_16.fld2 = _26 + _1.fld2;
_42 = _40;
_19 = (_40, _35, _7);
_1.fld2 = (-168956835711293438177470162966882500287_i128) as isize;
_37 = _36;
_1.fld0 = _16.fld0;
_43 = _1.fld6;
_22 = -_16.fld2;
RET = _3 ^ _16.fld6;
_16.fld4 = _2;
_1.fld0.0 = _16.fld0.0;
_11 = (-1284764428_i32) as isize;
Goto(bb14)
}
bb14 = {
Call(_46 = dump_var(8_usize, 34_usize, Move(_34), 29_usize, Move(_29), 7_usize, Move(_7), 33_usize, Move(_33)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_46 = dump_var(8_usize, 22_usize, Move(_22), 32_usize, Move(_32), 3_usize, Move(_3), 37_usize, Move(_37)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_46 = dump_var(8_usize, 23_usize, Move(_23), 9_usize, Move(_9), 25_usize, Move(_25), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(8_usize, 24_usize, Move(_24), 12_usize, Move(_12), 14_usize, Move(_14), 47_usize, _47), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [i8; 3],mut _2: isize,mut _3: u8,mut _4: [u128; 6]) -> i64 {
mir! {
type RET = i64;
let _5: f32;
let _6: *mut ([u16; 4], u128, u8, f32);
let _7: char;
let _8: [usize; 4];
let _9: [u32; 7];
let _10: Adt58;
let _11: isize;
let _12: &'static f64;
let _13: Adt51;
let _14: [u16; 4];
let _15: [bool; 8];
let _16: isize;
let _17: Adt59;
let _18: ();
let _19: ();
{
_1 = [(-74_i8),(-6_i8),(-34_i8)];
RET = -3795171634925588785_i64;
_1 = [59_i8,105_i8,(-5_i8)];
_5 = (-16185_i16) as f32;
_3 = true as u8;
RET = 49310288069623194712818861598414792704_u128 as i64;
_1 = [59_i8,(-50_i8),114_i8];
_4 = [114953839057728231708038773152134711734_u128,170534969380690993336796785020981592479_u128,6083021540845540752751578157005749256_u128,119965009450087649061791952016853227754_u128,243878180156649697302088353957975416318_u128,276713223588850904428610042042490557376_u128];
RET = 10320746307998546682_u64 as i64;
_2 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_7 = '\u{ff1f8}';
_4 = [87197550142386369630100474905821278500_u128,151840728593100067235142791909694849099_u128,264205278468338063559373351131186006261_u128,297049112667845359842843496038430205220_u128,149862057112087393565499964088888290791_u128,149309130477961473546770470467367031603_u128];
_8 = [4_usize,501913431366475508_usize,11381961257423624596_usize,354285670256047122_usize];
_1 = [(-5_i8),98_i8,(-52_i8)];
_4 = [193397708132453699848133033806788600056_u128,194267911378215538480098362318543223412_u128,15724080703483867452105124198847892121_u128,178308000914058792638100058362499857920_u128,264043937445690133928307290917259509641_u128,134245532865934999997057374397541964469_u128];
_4 = [140516328306835332458808457970851688670_u128,131284879176387437120828867236603527369_u128,299696154138110427561646122274544783758_u128,104473528698693001485005168928942655493_u128,228937022367670558589956290067260495732_u128,31218402371315012270499122168601112105_u128];
Call(_3 = fn10(_1, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = '\u{c5b97}';
RET = !(-3348480569482614796_i64);
_8 = [7518748180049360918_usize,4740994634860832065_usize,6_usize,7_usize];
_1 = [95_i8,(-2_i8),(-77_i8)];
_8 = [13450236342123899088_usize,7_usize,5674477617863189689_usize,0_usize];
_2 = 15289296480478404713_usize as isize;
_5 = (-21079_i16) as f32;
_2 = _5 as isize;
Goto(bb2)
}
bb2 = {
RET = 7267645494593342736_i64;
RET = 33067959927282935_i64;
RET = 2196551716067502497_i64 << _2;
RET = -5422130030408250927_i64;
_8 = [4_usize,4_usize,2_usize,6_usize];
_9 = [3162822943_u32,3984296714_u32,4120931761_u32,924514731_u32,2861431356_u32,3095233841_u32,3545557944_u32];
RET = (-97315256_i32) as i64;
_8 = [0_usize,572677585120835701_usize,5792898176329432029_usize,9679387665653177314_usize];
_8 = [2_usize,3851560140813843287_usize,14388098814791777319_usize,2_usize];
_4 = [124195399131311552766235460196921530334_u128,8707657546384741114991159552000068830_u128,272016283205133974039359570170138438323_u128,225657063071718435088105458953736542631_u128,93453069195616310593319953612044262260_u128,121369776298864592943355984680550980594_u128];
_2 = 9223372036854775807_isize;
_7 = '\u{58fc5}';
RET = (-492491286191590557_i64);
match RET {
0 => bb1,
340282366920938463462882116145576620899 => bb4,
_ => bb3
}
}
bb3 = {
_7 = '\u{c5b97}';
RET = !(-3348480569482614796_i64);
_8 = [7518748180049360918_usize,4740994634860832065_usize,6_usize,7_usize];
_1 = [95_i8,(-2_i8),(-77_i8)];
_8 = [13450236342123899088_usize,7_usize,5674477617863189689_usize,0_usize];
_2 = 15289296480478404713_usize as isize;
_5 = (-21079_i16) as f32;
_2 = _5 as isize;
Goto(bb2)
}
bb4 = {
RET = (-6489636086811905461_i64);
_5 = _2 as f32;
RET = _3 as i64;
RET = 6379027520952099749_i64 | (-4476841288230487272_i64);
_3 = _7 as u8;
_4 = [267052588890518369542115256799396360609_u128,212277398256980699423623546137935414163_u128,253367620149087165632355711635832813417_u128,100067425895389042855596674992184451486_u128,43987470052202536879806083530027572712_u128,270611292929299715139493729070891124727_u128];
_9 = [3290094060_u32,292986223_u32,3411146535_u32,660370554_u32,3497918532_u32,293802970_u32,3569191570_u32];
_7 = '\u{27254}';
_9 = [374497342_u32,183121456_u32,4137223043_u32,3887954899_u32,525695202_u32,4009576445_u32,1993733465_u32];
_4 = [159394593821400469585764144067511007379_u128,125116444192437016498994702816952453861_u128,143861173589262816848928870200951851138_u128,232983094299866439532643622390033145930_u128,19629803968115584388898213808747784373_u128,24613191107129134766767193851835163051_u128];
_7 = '\u{fb983}';
_2 = 885209586_u32 as isize;
RET = !(-7236021996024992919_i64);
_3 = !103_u8;
_5 = 308316630090393120870183660094023750449_u128 as f32;
RET = (-966652961769167505_i64);
_4 = [1210689888869632671082242243312153839_u128,100783840852538003800806949170181511039_u128,13024522685526722414239121047937646340_u128,100735386714697435641835729247999390001_u128,218590197074091232383376978222911206229_u128,99195600142643438016850237537527311982_u128];
_5 = 1213420312_i32 as f32;
_4 = [246694470666877971583133349779636253734_u128,141068728530108505666087456407020827777_u128,28590535312186087504886936400972829913_u128,147320207970782579253183391806147307088_u128,95794529435281969753866633371384038070_u128,177159911082844675473236432769001883609_u128];
_4 = [326283406746554189668975524408161716316_u128,46247513364234913320591833295261684923_u128,14650907460110550617862936910130054989_u128,223916405201671506957799211661159658919_u128,333184114981533538621723976410260149016_u128,133531775569330915118438888891352013534_u128];
_9 = [3538427795_u32,3052182493_u32,4210233203_u32,3978633593_u32,1156837249_u32,3097316933_u32,353683475_u32];
RET = 1643665343247207234_i64;
_2 = 124311869998306621408262680675242453740_u128 as isize;
_8 = [13653647323843197408_usize,12416325563698372953_usize,8168237335278399136_usize,752097784444708571_usize];
RET = (-3310614953111219063_i64);
_2 = (-9223372036854775808_isize) * 9223372036854775807_isize;
Call(_5 = fn11(_9, _9, _4, _3), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET = (-5883900401301826443_i64) << _2;
_5 = 14474645988990024486_u64 as f32;
_5 = 36_i8 as f32;
RET = 323039601131524491735739068407143945890_u128 as i64;
_5 = 17936190559660189306_usize as f32;
_1 = [(-61_i8),14_i8,(-95_i8)];
RET = _3 as i64;
_9 = [2606184672_u32,2595164289_u32,1350506601_u32,3259194761_u32,822452146_u32,4117377034_u32,3084317787_u32];
_3 = 235_u8;
_2 = 9223372036854775807_isize;
_2 = 79_isize;
_8 = [5059578899449890210_usize,5_usize,6_usize,4955135838776718917_usize];
_2 = (-9223372036854775808_isize);
_5 = _3 as f32;
_3 = 5_u8;
RET = 5402735274529756970_i64 >> _2;
RET = !(-301707984472317159_i64);
_13.fld3.1 = [0_usize,0_usize,0_usize,7403538228283273918_usize,8361695408216117222_usize,3106188036934074125_usize,4_usize,637675462374579486_usize];
match _3 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb13,
_ => bb12
}
}
bb6 = {
RET = (-6489636086811905461_i64);
_5 = _2 as f32;
RET = _3 as i64;
RET = 6379027520952099749_i64 | (-4476841288230487272_i64);
_3 = _7 as u8;
_4 = [267052588890518369542115256799396360609_u128,212277398256980699423623546137935414163_u128,253367620149087165632355711635832813417_u128,100067425895389042855596674992184451486_u128,43987470052202536879806083530027572712_u128,270611292929299715139493729070891124727_u128];
_9 = [3290094060_u32,292986223_u32,3411146535_u32,660370554_u32,3497918532_u32,293802970_u32,3569191570_u32];
_7 = '\u{27254}';
_9 = [374497342_u32,183121456_u32,4137223043_u32,3887954899_u32,525695202_u32,4009576445_u32,1993733465_u32];
_4 = [159394593821400469585764144067511007379_u128,125116444192437016498994702816952453861_u128,143861173589262816848928870200951851138_u128,232983094299866439532643622390033145930_u128,19629803968115584388898213808747784373_u128,24613191107129134766767193851835163051_u128];
_7 = '\u{fb983}';
_2 = 885209586_u32 as isize;
RET = !(-7236021996024992919_i64);
_3 = !103_u8;
_5 = 308316630090393120870183660094023750449_u128 as f32;
RET = (-966652961769167505_i64);
_4 = [1210689888869632671082242243312153839_u128,100783840852538003800806949170181511039_u128,13024522685526722414239121047937646340_u128,100735386714697435641835729247999390001_u128,218590197074091232383376978222911206229_u128,99195600142643438016850237537527311982_u128];
_5 = 1213420312_i32 as f32;
_4 = [246694470666877971583133349779636253734_u128,141068728530108505666087456407020827777_u128,28590535312186087504886936400972829913_u128,147320207970782579253183391806147307088_u128,95794529435281969753866633371384038070_u128,177159911082844675473236432769001883609_u128];
_4 = [326283406746554189668975524408161716316_u128,46247513364234913320591833295261684923_u128,14650907460110550617862936910130054989_u128,223916405201671506957799211661159658919_u128,333184114981533538621723976410260149016_u128,133531775569330915118438888891352013534_u128];
_9 = [3538427795_u32,3052182493_u32,4210233203_u32,3978633593_u32,1156837249_u32,3097316933_u32,353683475_u32];
RET = 1643665343247207234_i64;
_2 = 124311869998306621408262680675242453740_u128 as isize;
_8 = [13653647323843197408_usize,12416325563698372953_usize,8168237335278399136_usize,752097784444708571_usize];
RET = (-3310614953111219063_i64);
_2 = (-9223372036854775808_isize) * 9223372036854775807_isize;
Call(_5 = fn11(_9, _9, _4, _3), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_7 = '\u{c5b97}';
RET = !(-3348480569482614796_i64);
_8 = [7518748180049360918_usize,4740994634860832065_usize,6_usize,7_usize];
_1 = [95_i8,(-2_i8),(-77_i8)];
_8 = [13450236342123899088_usize,7_usize,5674477617863189689_usize,0_usize];
_2 = 15289296480478404713_usize as isize;
_5 = (-21079_i16) as f32;
_2 = _5 as isize;
Goto(bb2)
}
bb8 = {
RET = 7267645494593342736_i64;
RET = 33067959927282935_i64;
RET = 2196551716067502497_i64 << _2;
RET = -5422130030408250927_i64;
_8 = [4_usize,4_usize,2_usize,6_usize];
_9 = [3162822943_u32,3984296714_u32,4120931761_u32,924514731_u32,2861431356_u32,3095233841_u32,3545557944_u32];
RET = (-97315256_i32) as i64;
_8 = [0_usize,572677585120835701_usize,5792898176329432029_usize,9679387665653177314_usize];
_8 = [2_usize,3851560140813843287_usize,14388098814791777319_usize,2_usize];
_4 = [124195399131311552766235460196921530334_u128,8707657546384741114991159552000068830_u128,272016283205133974039359570170138438323_u128,225657063071718435088105458953736542631_u128,93453069195616310593319953612044262260_u128,121369776298864592943355984680550980594_u128];
_2 = 9223372036854775807_isize;
_7 = '\u{58fc5}';
RET = (-492491286191590557_i64);
match RET {
0 => bb1,
340282366920938463462882116145576620899 => bb4,
_ => bb3
}
}
bb9 = {
_7 = '\u{c5b97}';
RET = !(-3348480569482614796_i64);
_8 = [7518748180049360918_usize,4740994634860832065_usize,6_usize,7_usize];
_1 = [95_i8,(-2_i8),(-77_i8)];
_8 = [13450236342123899088_usize,7_usize,5674477617863189689_usize,0_usize];
_2 = 15289296480478404713_usize as isize;
_5 = (-21079_i16) as f32;
_2 = _5 as isize;
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
_11 = _2;
_5 = (-669167658_i32) as f32;
_9 = [3261118442_u32,4142323098_u32,403124637_u32,1853755496_u32,1467072476_u32,2911810522_u32,4109249195_u32];
_13.fld0.3 = -_5;
_3 = !79_u8;
_11 = 6920683897894862560_u64 as isize;
_13.fld3.0 = [28233_i16,(-27630_i16),(-12965_i16),(-17610_i16),17474_i16,12876_i16,(-26435_i16)];
_13.fld1 = [2145025595_u32,1407606488_u32,3423400700_u32,1101374974_u32,397755762_u32,3041009017_u32,4162154063_u32];
_3 = 140_u8 >> RET;
_14 = [62218_u16,44456_u16,60852_u16,60091_u16];
match _2 {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
340282366920938463454151235394913435648 => bb19,
_ => bb18
}
}
bb14 = {
RET = (-6489636086811905461_i64);
_5 = _2 as f32;
RET = _3 as i64;
RET = 6379027520952099749_i64 | (-4476841288230487272_i64);
_3 = _7 as u8;
_4 = [267052588890518369542115256799396360609_u128,212277398256980699423623546137935414163_u128,253367620149087165632355711635832813417_u128,100067425895389042855596674992184451486_u128,43987470052202536879806083530027572712_u128,270611292929299715139493729070891124727_u128];
_9 = [3290094060_u32,292986223_u32,3411146535_u32,660370554_u32,3497918532_u32,293802970_u32,3569191570_u32];
_7 = '\u{27254}';
_9 = [374497342_u32,183121456_u32,4137223043_u32,3887954899_u32,525695202_u32,4009576445_u32,1993733465_u32];
_4 = [159394593821400469585764144067511007379_u128,125116444192437016498994702816952453861_u128,143861173589262816848928870200951851138_u128,232983094299866439532643622390033145930_u128,19629803968115584388898213808747784373_u128,24613191107129134766767193851835163051_u128];
_7 = '\u{fb983}';
_2 = 885209586_u32 as isize;
RET = !(-7236021996024992919_i64);
_3 = !103_u8;
_5 = 308316630090393120870183660094023750449_u128 as f32;
RET = (-966652961769167505_i64);
_4 = [1210689888869632671082242243312153839_u128,100783840852538003800806949170181511039_u128,13024522685526722414239121047937646340_u128,100735386714697435641835729247999390001_u128,218590197074091232383376978222911206229_u128,99195600142643438016850237537527311982_u128];
_5 = 1213420312_i32 as f32;
_4 = [246694470666877971583133349779636253734_u128,141068728530108505666087456407020827777_u128,28590535312186087504886936400972829913_u128,147320207970782579253183391806147307088_u128,95794529435281969753866633371384038070_u128,177159911082844675473236432769001883609_u128];
_4 = [326283406746554189668975524408161716316_u128,46247513364234913320591833295261684923_u128,14650907460110550617862936910130054989_u128,223916405201671506957799211661159658919_u128,333184114981533538621723976410260149016_u128,133531775569330915118438888891352013534_u128];
_9 = [3538427795_u32,3052182493_u32,4210233203_u32,3978633593_u32,1156837249_u32,3097316933_u32,353683475_u32];
RET = 1643665343247207234_i64;
_2 = 124311869998306621408262680675242453740_u128 as isize;
_8 = [13653647323843197408_usize,12416325563698372953_usize,8168237335278399136_usize,752097784444708571_usize];
RET = (-3310614953111219063_i64);
_2 = (-9223372036854775808_isize) * 9223372036854775807_isize;
Call(_5 = fn11(_9, _9, _4, _3), ReturnTo(bb5), UnwindUnreachable())
}
bb15 = {
RET = 7267645494593342736_i64;
RET = 33067959927282935_i64;
RET = 2196551716067502497_i64 << _2;
RET = -5422130030408250927_i64;
_8 = [4_usize,4_usize,2_usize,6_usize];
_9 = [3162822943_u32,3984296714_u32,4120931761_u32,924514731_u32,2861431356_u32,3095233841_u32,3545557944_u32];
RET = (-97315256_i32) as i64;
_8 = [0_usize,572677585120835701_usize,5792898176329432029_usize,9679387665653177314_usize];
_8 = [2_usize,3851560140813843287_usize,14388098814791777319_usize,2_usize];
_4 = [124195399131311552766235460196921530334_u128,8707657546384741114991159552000068830_u128,272016283205133974039359570170138438323_u128,225657063071718435088105458953736542631_u128,93453069195616310593319953612044262260_u128,121369776298864592943355984680550980594_u128];
_2 = 9223372036854775807_isize;
_7 = '\u{58fc5}';
RET = (-492491286191590557_i64);
match RET {
0 => bb1,
340282366920938463462882116145576620899 => bb4,
_ => bb3
}
}
bb16 = {
_7 = '\u{c5b97}';
RET = !(-3348480569482614796_i64);
_8 = [7518748180049360918_usize,4740994634860832065_usize,6_usize,7_usize];
_1 = [95_i8,(-2_i8),(-77_i8)];
_8 = [13450236342123899088_usize,7_usize,5674477617863189689_usize,0_usize];
_2 = 15289296480478404713_usize as isize;
_5 = (-21079_i16) as f32;
_2 = _5 as isize;
Goto(bb2)
}
bb17 = {
_7 = '\u{c5b97}';
RET = !(-3348480569482614796_i64);
_8 = [7518748180049360918_usize,4740994634860832065_usize,6_usize,7_usize];
_1 = [95_i8,(-2_i8),(-77_i8)];
_8 = [13450236342123899088_usize,7_usize,5674477617863189689_usize,0_usize];
_2 = 15289296480478404713_usize as isize;
_5 = (-21079_i16) as f32;
_2 = _5 as isize;
Goto(bb2)
}
bb18 = {
RET = 7267645494593342736_i64;
RET = 33067959927282935_i64;
RET = 2196551716067502497_i64 << _2;
RET = -5422130030408250927_i64;
_8 = [4_usize,4_usize,2_usize,6_usize];
_9 = [3162822943_u32,3984296714_u32,4120931761_u32,924514731_u32,2861431356_u32,3095233841_u32,3545557944_u32];
RET = (-97315256_i32) as i64;
_8 = [0_usize,572677585120835701_usize,5792898176329432029_usize,9679387665653177314_usize];
_8 = [2_usize,3851560140813843287_usize,14388098814791777319_usize,2_usize];
_4 = [124195399131311552766235460196921530334_u128,8707657546384741114991159552000068830_u128,272016283205133974039359570170138438323_u128,225657063071718435088105458953736542631_u128,93453069195616310593319953612044262260_u128,121369776298864592943355984680550980594_u128];
_2 = 9223372036854775807_isize;
_7 = '\u{58fc5}';
RET = (-492491286191590557_i64);
match RET {
0 => bb1,
340282366920938463462882116145576620899 => bb4,
_ => bb3
}
}
bb19 = {
_16 = _2;
_3 = 128_u8;
Goto(bb20)
}
bb20 = {
Call(_18 = dump_var(9_usize, 7_usize, Move(_7), 14_usize, Move(_14), 9_usize, Move(_9), 8_usize, Move(_8)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_18 = dump_var(9_usize, 16_usize, Move(_16), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: [i8; 3],mut _2: f32) -> u8 {
mir! {
type RET = u8;
let _3: [i8; 3];
let _4: isize;
let _5: [bool; 3];
let _6: bool;
let _7: isize;
let _8: f32;
let _9: bool;
let _10: Adt45;
let _11: [u16; 4];
let _12: ([usize; 4],);
let _13: Adt55;
let _14: *const [bool; 5];
let _15: Adt49;
let _16: f32;
let _17: ();
let _18: ();
{
RET = 10507_u16 as u8;
_3 = [92_i8,(-10_i8),(-41_i8)];
_4 = 75_i8 as isize;
_2 = (-20361_i16) as f32;
RET = true as u8;
RET = _4 as u8;
RET = 202_u8;
_1 = _3;
_4 = 9223372036854775807_isize | (-17_isize);
_3 = [(-37_i8),(-64_i8),77_i8];
_5 = [false,false,false];
_6 = false;
_5 = [_6,_6,_6];
RET = 133_u8 & 190_u8;
_5 = [_6,_6,_6];
_1 = [(-126_i8),(-87_i8),(-16_i8)];
RET = 172_u8 * 51_u8;
_1 = _3;
_1 = [(-40_i8),106_i8,10_i8];
_7 = _4;
_3 = [118_i8,115_i8,(-62_i8)];
_4 = -_7;
_3 = [4_i8,(-59_i8),(-64_i8)];
RET = _6 as u8;
Goto(bb1)
}
bb1 = {
_4 = 4695_u16 as isize;
_4 = _7 << _7;
_2 = 12371794430849096259_usize as f32;
_8 = _2 + _2;
_8 = _2;
RET = 214_u8 + 90_u8;
_2 = _8;
_5 = [_6,_6,_6];
_8 = (-55_i8) as f32;
_3 = [49_i8,14_i8,(-46_i8)];
_2 = 35273_u16 as f32;
_4 = _7;
_1 = _3;
_6 = true;
_1 = [47_i8,89_i8,(-112_i8)];
_6 = !false;
RET = 2_usize as u8;
_1 = [(-41_i8),53_i8,99_i8];
_4 = _7;
_6 = !false;
_3 = [(-6_i8),127_i8,74_i8];
_10 = Adt45 { fld0: 50300_u16 };
_5 = [_6,_6,_6];
_1 = _3;
_3 = _1;
_9 = !_6;
_1 = [(-37_i8),(-43_i8),(-104_i8)];
match _10.fld0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
50300 => bb10,
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
_9 = !_6;
_8 = _10.fld0 as f32;
_10.fld0 = 5750_u16;
_9 = _6;
_9 = _6;
_1 = [35_i8,(-53_i8),18_i8];
_1 = [66_i8,(-87_i8),(-64_i8)];
_4 = '\u{ccc05}' as isize;
_11 = [_10.fld0,_10.fld0,_10.fld0,_10.fld0];
_12.0 = [4_usize,15552478260155086407_usize,7_usize,13602154986119918559_usize];
_12.0 = [4_usize,2_usize,2_usize,3317906940810950549_usize];
_8 = _2;
_3 = [24_i8,64_i8,62_i8];
_7 = _4;
_5 = [_9,_9,_6];
RET = 117_u8 | 241_u8;
RET = 89_u8;
_7 = _10.fld0 as isize;
RET = 1673943930_u32 as u8;
_1 = _3;
match _10.fld0 {
0 => bb9,
1 => bb8,
2 => bb5,
3 => bb7,
4 => bb11,
5 => bb12,
5750 => bb14,
_ => bb13
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
_16 = _2 - _8;
RET = 134_u8 + 131_u8;
_1 = _3;
_8 = -_16;
_10 = Adt45 { fld0: 38337_u16 };
_4 = _7 + _7;
_6 = !_9;
_10 = Adt45 { fld0: 2800_u16 };
_6 = _9;
Goto(bb15)
}
bb15 = {
Call(_17 = dump_var(10_usize, 5_usize, Move(_5), 7_usize, Move(_7), 12_usize, Move(_12), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [u32; 7],mut _2: [u32; 7],mut _3: [u128; 6],mut _4: u8) -> f32 {
mir! {
type RET = f32;
let _5: *const [bool; 5];
let _6: Adt45;
let _7: [bool; 3];
let _8: Adt48;
let _9: [bool; 3];
let _10: isize;
let _11: [u128; 6];
let _12: [u128; 6];
let _13: [i16; 1];
let _14: [u16; 6];
let _15: i128;
let _16: [usize; 4];
let _17: u16;
let _18: u32;
let _19: char;
let _20: Adt52;
let _21: i64;
let _22: [usize; 4];
let _23: ([u16; 4], u128, u8, f32);
let _24: ([usize; 4],);
let _25: ([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4]);
let _26: f32;
let _27: isize;
let _28: Adt53;
let _29: i64;
let _30: Adt49;
let _31: f64;
let _32: i32;
let _33: bool;
let _34: ();
let _35: ();
{
RET = (-62_i8) as f32;
_3 = [155924180679535619830730017628933663376_u128,335262384103854325066591191368457121008_u128,197013191233762973789708072015490540580_u128,48117317377101540344307573899581655897_u128,27398938501090910551889896979827495583_u128,63021491041685302317554964122611005368_u128];
_4 = 255_u8;
RET = 3788247158_u32 as f32;
_4 = !19_u8;
RET = 1011066831_u32 as f32;
_3 = [273671862095995214950544087054041845995_u128,123484685582319530798089620628408176185_u128,295729487436618239993072630800476841093_u128,206643789394138443290022993419239509857_u128,125778167298891361745476641598964320145_u128,183496686028105092529998370203427658738_u128];
RET = 9223372036854775807_isize as f32;
_4 = 85_u8 >> (-48646659651157103741644248407850146775_i128);
_4 = 134_u8;
RET = 169164968792908473952484376496587837734_i128 as f32;
_2 = [3944205139_u32,376734886_u32,2755811470_u32,2169732097_u32,2112766198_u32,3529065139_u32,1976195240_u32];
_3 = [331636216006496504984043615040165288363_u128,278823748844293719737237231398441097878_u128,136577610001434581302416251523885281002_u128,292474239819313320728900467742906512890_u128,176309802462527269038997493861149411978_u128,132785769900967959082386828701381256587_u128];
_4 = 18431465511296843665760854839244001429_i128 as u8;
_6.fld0 = 14766_u16;
_1 = _2;
RET = 130260765_i32 as f32;
RET = 3847537730_u32 as f32;
RET = _4 as f32;
_3 = [86725847522931708141375392421422572161_u128,316565338417312333345670448235566242803_u128,84672986680887928457070389407280324260_u128,243692076519536926145147303532128026920_u128,232635333185647345147062023961938842113_u128,264433816052522222083158448079362404165_u128];
_4 = 81_u8;
Goto(bb1)
}
bb1 = {
_6 = Adt45 { fld0: 20227_u16 };
RET = _4 as f32;
RET = 18595_i16 as f32;
_7 = [false,false,false];
_1 = [3212905288_u32,1990870758_u32,1302662847_u32,1222091790_u32,3226152745_u32,1552367434_u32,3031388728_u32];
_6 = Adt45 { fld0: 150_u16 };
_1 = _2;
Goto(bb2)
}
bb2 = {
_8.fld1.2 = _4;
_8.fld5 = [true,false,true,false,false];
_8.fld1.0 = [(-23645_i16),30912_i16,(-859_i16),(-8986_i16),22528_i16,(-21502_i16),(-9237_i16)];
_8.fld1.4 = _3;
_8.fld2 = [7630582852338455563_usize,1_usize,3641610184060015191_usize,17904754680378946604_usize,10674660707222008406_usize,17783341069676604843_usize,15109558310029017326_usize,8089374232667316482_usize];
_1 = _2;
RET = 9223372036854775807_isize as f32;
_8.fld1.3 = [true,true,false,false,true,true,true,true];
RET = 9223372036854775807_isize as f32;
_8.fld5 = [false,true,false,true,true];
_6 = Adt45 { fld0: 32026_u16 };
_8.fld3 = _6.fld0 as f32;
_8.fld1.2 = _4;
_8.fld1.4 = [73388381618702174938553505424345322451_u128,298872510826790136739925426470326124133_u128,97948796876634758582704439631991618103_u128,316959118374217580344321876265404738054_u128,116867346569646352554079585106686582862_u128,250497790467392614780868705634934855506_u128];
_8.fld1.5 = [_6.fld0,_6.fld0,_6.fld0,_6.fld0];
RET = -_8.fld3;
_6.fld0 = 2_usize as u16;
_8.fld4 = [26010_i16,12863_i16,18309_i16,1140_i16,30927_i16,(-22451_i16),7001_i16];
_3 = _8.fld1.4;
_10 = (-9223372036854775808_isize) >> _6.fld0;
RET = -_8.fld3;
_8.fld6 = [_6.fld0,_6.fld0,_6.fld0,_6.fld0,_6.fld0,_6.fld0];
_6.fld0 = 33538_u16;
Goto(bb3)
}
bb3 = {
_8.fld1.4 = [100508164451572482675229534244589095684_u128,130789180058186376380224552767315293912_u128,61058090156472531567458463073035468888_u128,307564916555757346444521689649665247400_u128,105606937730234408799128858440866175000_u128,138004002191210871043626943322359787052_u128];
_6.fld0 = 10751_u16 << _4;
_8.fld5 = [false,true,true,true,false];
_16 = [6_usize,5_usize,1_usize,4_usize];
_10 = 115_isize >> _6.fld0;
_3 = [312228760934568549636069683811080247146_u128,44748398831228904922751545458702281965_u128,85934079477819660187920901855118956793_u128,259036338078288758149431472564568990355_u128,254615526600498482189466361255583903915_u128,116109945805979143781963623018806579980_u128];
_16 = [12958834888984506145_usize,8678019538396531017_usize,1_usize,4_usize];
_15 = !108750283980792092813876515526033570205_i128;
_15 = -44801376749780182762244650363221330424_i128;
_3 = [50940746419477812279735777798567850562_u128,243174574251782043064516017711457540474_u128,332476859016641114984279827591161487485_u128,110813933550464931924818870216525871846_u128,109438209886467326160754386834116722927_u128,215136741532094354103575987795919521529_u128];
_8.fld1.1 = _7;
_12 = _3;
_8.fld1.4 = _3;
_9 = [true,true,true];
_5 = core::ptr::addr_of!(_8.fld5);
Goto(bb4)
}
bb4 = {
_8.fld1.3 = [true,false,true,false,true,false,true,true];
_17 = 2235501845_u32 as u16;
_10 = !(-9223372036854775808_isize);
_13 = [(-185_i16)];
_8.fld1.0 = [4773_i16,17563_i16,(-20425_i16),23082_i16,9006_i16,15298_i16,5497_i16];
_14 = [_6.fld0,_6.fld0,_6.fld0,_17,_17,_6.fld0];
_6 = Adt45 { fld0: _17 };
_8.fld1.2 = _4;
_9 = _8.fld1.1;
_19 = '\u{4291}';
(*_5) = [true,false,true,true,false];
_8.fld1.0 = [(-22018_i16),1996_i16,1366_i16,(-19767_i16),(-5776_i16),(-11997_i16),(-14111_i16)];
_10 = (-14665_i16) as isize;
_18 = 1781016092_u32;
_23.3 = 3269141556850314525_usize as f32;
_15 = 42272882564506747942966481823089391404_i128 ^ (-53042741062695747620574885155538222685_i128);
_9 = _8.fld1.1;
_8.fld1.1 = [true,false,true];
_19 = '\u{ec719}';
_23.3 = RET * RET;
_8.fld6 = [_6.fld0,_6.fld0,_17,_17,_6.fld0,_17];
_22 = [12402128707910698475_usize,18216895725528234040_usize,2_usize,5_usize];
_8.fld0 = core::ptr::addr_of_mut!(_23);
_23.1 = 48920663485355191317905293624186647026_u128;
_1 = [_18,_18,_18,_18,_18,_18,_18];
match _8.fld1.2 {
0 => bb1,
1 => bb5,
2 => bb6,
81 => bb8,
_ => bb7
}
}
bb5 = {
_8.fld1.4 = [100508164451572482675229534244589095684_u128,130789180058186376380224552767315293912_u128,61058090156472531567458463073035468888_u128,307564916555757346444521689649665247400_u128,105606937730234408799128858440866175000_u128,138004002191210871043626943322359787052_u128];
_6.fld0 = 10751_u16 << _4;
_8.fld5 = [false,true,true,true,false];
_16 = [6_usize,5_usize,1_usize,4_usize];
_10 = 115_isize >> _6.fld0;
_3 = [312228760934568549636069683811080247146_u128,44748398831228904922751545458702281965_u128,85934079477819660187920901855118956793_u128,259036338078288758149431472564568990355_u128,254615526600498482189466361255583903915_u128,116109945805979143781963623018806579980_u128];
_16 = [12958834888984506145_usize,8678019538396531017_usize,1_usize,4_usize];
_15 = !108750283980792092813876515526033570205_i128;
_15 = -44801376749780182762244650363221330424_i128;
_3 = [50940746419477812279735777798567850562_u128,243174574251782043064516017711457540474_u128,332476859016641114984279827591161487485_u128,110813933550464931924818870216525871846_u128,109438209886467326160754386834116722927_u128,215136741532094354103575987795919521529_u128];
_8.fld1.1 = _7;
_12 = _3;
_8.fld1.4 = _3;
_9 = [true,true,true];
_5 = core::ptr::addr_of!(_8.fld5);
Goto(bb4)
}
bb6 = {
_8.fld1.2 = _4;
_8.fld5 = [true,false,true,false,false];
_8.fld1.0 = [(-23645_i16),30912_i16,(-859_i16),(-8986_i16),22528_i16,(-21502_i16),(-9237_i16)];
_8.fld1.4 = _3;
_8.fld2 = [7630582852338455563_usize,1_usize,3641610184060015191_usize,17904754680378946604_usize,10674660707222008406_usize,17783341069676604843_usize,15109558310029017326_usize,8089374232667316482_usize];
_1 = _2;
RET = 9223372036854775807_isize as f32;
_8.fld1.3 = [true,true,false,false,true,true,true,true];
RET = 9223372036854775807_isize as f32;
_8.fld5 = [false,true,false,true,true];
_6 = Adt45 { fld0: 32026_u16 };
_8.fld3 = _6.fld0 as f32;
_8.fld1.2 = _4;
_8.fld1.4 = [73388381618702174938553505424345322451_u128,298872510826790136739925426470326124133_u128,97948796876634758582704439631991618103_u128,316959118374217580344321876265404738054_u128,116867346569646352554079585106686582862_u128,250497790467392614780868705634934855506_u128];
_8.fld1.5 = [_6.fld0,_6.fld0,_6.fld0,_6.fld0];
RET = -_8.fld3;
_6.fld0 = 2_usize as u16;
_8.fld4 = [26010_i16,12863_i16,18309_i16,1140_i16,30927_i16,(-22451_i16),7001_i16];
_3 = _8.fld1.4;
_10 = (-9223372036854775808_isize) >> _6.fld0;
RET = -_8.fld3;
_8.fld6 = [_6.fld0,_6.fld0,_6.fld0,_6.fld0,_6.fld0,_6.fld0];
_6.fld0 = 33538_u16;
Goto(bb3)
}
bb7 = {
_6 = Adt45 { fld0: 20227_u16 };
RET = _4 as f32;
RET = 18595_i16 as f32;
_7 = [false,false,false];
_1 = [3212905288_u32,1990870758_u32,1302662847_u32,1222091790_u32,3226152745_u32,1552367434_u32,3031388728_u32];
_6 = Adt45 { fld0: 150_u16 };
_1 = _2;
Goto(bb2)
}
bb8 = {
_8.fld3 = RET;
_18 = _23.1 as u32;
_23.0 = [_6.fld0,_17,_17,_17];
_23.0 = [_6.fld0,_17,_6.fld0,_6.fld0];
_1 = [_18,_18,_18,_18,_18,_18,_18];
_8.fld1.5 = _23.0;
_21 = false as i64;
_22 = [2_usize,8359826605111039238_usize,7_usize,6_usize];
_25.5 = [_6.fld0,_6.fld0,_6.fld0,_6.fld0];
_26 = _21 as f32;
_8.fld3 = _23.3;
_8.fld1.5 = _23.0;
_25.4 = _8.fld1.4;
match _4 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb9,
6 => bb10,
81 => bb12,
_ => bb11
}
}
bb9 = {
_8.fld1.4 = [100508164451572482675229534244589095684_u128,130789180058186376380224552767315293912_u128,61058090156472531567458463073035468888_u128,307564916555757346444521689649665247400_u128,105606937730234408799128858440866175000_u128,138004002191210871043626943322359787052_u128];
_6.fld0 = 10751_u16 << _4;
_8.fld5 = [false,true,true,true,false];
_16 = [6_usize,5_usize,1_usize,4_usize];
_10 = 115_isize >> _6.fld0;
_3 = [312228760934568549636069683811080247146_u128,44748398831228904922751545458702281965_u128,85934079477819660187920901855118956793_u128,259036338078288758149431472564568990355_u128,254615526600498482189466361255583903915_u128,116109945805979143781963623018806579980_u128];
_16 = [12958834888984506145_usize,8678019538396531017_usize,1_usize,4_usize];
_15 = !108750283980792092813876515526033570205_i128;
_15 = -44801376749780182762244650363221330424_i128;
_3 = [50940746419477812279735777798567850562_u128,243174574251782043064516017711457540474_u128,332476859016641114984279827591161487485_u128,110813933550464931924818870216525871846_u128,109438209886467326160754386834116722927_u128,215136741532094354103575987795919521529_u128];
_8.fld1.1 = _7;
_12 = _3;
_8.fld1.4 = _3;
_9 = [true,true,true];
_5 = core::ptr::addr_of!(_8.fld5);
Goto(bb4)
}
bb10 = {
_8.fld1.2 = _4;
_8.fld5 = [true,false,true,false,false];
_8.fld1.0 = [(-23645_i16),30912_i16,(-859_i16),(-8986_i16),22528_i16,(-21502_i16),(-9237_i16)];
_8.fld1.4 = _3;
_8.fld2 = [7630582852338455563_usize,1_usize,3641610184060015191_usize,17904754680378946604_usize,10674660707222008406_usize,17783341069676604843_usize,15109558310029017326_usize,8089374232667316482_usize];
_1 = _2;
RET = 9223372036854775807_isize as f32;
_8.fld1.3 = [true,true,false,false,true,true,true,true];
RET = 9223372036854775807_isize as f32;
_8.fld5 = [false,true,false,true,true];
_6 = Adt45 { fld0: 32026_u16 };
_8.fld3 = _6.fld0 as f32;
_8.fld1.2 = _4;
_8.fld1.4 = [73388381618702174938553505424345322451_u128,298872510826790136739925426470326124133_u128,97948796876634758582704439631991618103_u128,316959118374217580344321876265404738054_u128,116867346569646352554079585106686582862_u128,250497790467392614780868705634934855506_u128];
_8.fld1.5 = [_6.fld0,_6.fld0,_6.fld0,_6.fld0];
RET = -_8.fld3;
_6.fld0 = 2_usize as u16;
_8.fld4 = [26010_i16,12863_i16,18309_i16,1140_i16,30927_i16,(-22451_i16),7001_i16];
_3 = _8.fld1.4;
_10 = (-9223372036854775808_isize) >> _6.fld0;
RET = -_8.fld3;
_8.fld6 = [_6.fld0,_6.fld0,_6.fld0,_6.fld0,_6.fld0,_6.fld0];
_6.fld0 = 33538_u16;
Goto(bb3)
}
bb11 = {
_8.fld1.4 = [100508164451572482675229534244589095684_u128,130789180058186376380224552767315293912_u128,61058090156472531567458463073035468888_u128,307564916555757346444521689649665247400_u128,105606937730234408799128858440866175000_u128,138004002191210871043626943322359787052_u128];
_6.fld0 = 10751_u16 << _4;
_8.fld5 = [false,true,true,true,false];
_16 = [6_usize,5_usize,1_usize,4_usize];
_10 = 115_isize >> _6.fld0;
_3 = [312228760934568549636069683811080247146_u128,44748398831228904922751545458702281965_u128,85934079477819660187920901855118956793_u128,259036338078288758149431472564568990355_u128,254615526600498482189466361255583903915_u128,116109945805979143781963623018806579980_u128];
_16 = [12958834888984506145_usize,8678019538396531017_usize,1_usize,4_usize];
_15 = !108750283980792092813876515526033570205_i128;
_15 = -44801376749780182762244650363221330424_i128;
_3 = [50940746419477812279735777798567850562_u128,243174574251782043064516017711457540474_u128,332476859016641114984279827591161487485_u128,110813933550464931924818870216525871846_u128,109438209886467326160754386834116722927_u128,215136741532094354103575987795919521529_u128];
_8.fld1.1 = _7;
_12 = _3;
_8.fld1.4 = _3;
_9 = [true,true,true];
_5 = core::ptr::addr_of!(_8.fld5);
Goto(bb4)
}
bb12 = {
_25.0 = _8.fld4;
_25.1 = _9;
_23.2 = !_4;
match _8.fld1.2 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb8,
4 => bb5,
5 => bb11,
81 => bb14,
_ => bb13
}
}
bb13 = {
_8.fld1.2 = _4;
_8.fld5 = [true,false,true,false,false];
_8.fld1.0 = [(-23645_i16),30912_i16,(-859_i16),(-8986_i16),22528_i16,(-21502_i16),(-9237_i16)];
_8.fld1.4 = _3;
_8.fld2 = [7630582852338455563_usize,1_usize,3641610184060015191_usize,17904754680378946604_usize,10674660707222008406_usize,17783341069676604843_usize,15109558310029017326_usize,8089374232667316482_usize];
_1 = _2;
RET = 9223372036854775807_isize as f32;
_8.fld1.3 = [true,true,false,false,true,true,true,true];
RET = 9223372036854775807_isize as f32;
_8.fld5 = [false,true,false,true,true];
_6 = Adt45 { fld0: 32026_u16 };
_8.fld3 = _6.fld0 as f32;
_8.fld1.2 = _4;
_8.fld1.4 = [73388381618702174938553505424345322451_u128,298872510826790136739925426470326124133_u128,97948796876634758582704439631991618103_u128,316959118374217580344321876265404738054_u128,116867346569646352554079585106686582862_u128,250497790467392614780868705634934855506_u128];
_8.fld1.5 = [_6.fld0,_6.fld0,_6.fld0,_6.fld0];
RET = -_8.fld3;
_6.fld0 = 2_usize as u16;
_8.fld4 = [26010_i16,12863_i16,18309_i16,1140_i16,30927_i16,(-22451_i16),7001_i16];
_3 = _8.fld1.4;
_10 = (-9223372036854775808_isize) >> _6.fld0;
RET = -_8.fld3;
_8.fld6 = [_6.fld0,_6.fld0,_6.fld0,_6.fld0,_6.fld0,_6.fld0];
_6.fld0 = 33538_u16;
Goto(bb3)
}
bb14 = {
RET = _26 * _23.3;
_23.2 = _4 + _4;
_8.fld1.1 = [false,false,true];
_8.fld2 = [13088950361508503217_usize,3_usize,1_usize,14633497695717137967_usize,13884783442654804627_usize,7238703810755648931_usize,5480427671184444869_usize,2_usize];
_8.fld4 = [(-29274_i16),25928_i16,(-15696_i16),(-27023_i16),23501_i16,(-30776_i16),5399_i16];
_11 = [_23.1,_23.1,_23.1,_23.1,_23.1,_23.1];
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(11_usize, 9_usize, Move(_9), 7_usize, Move(_7), 12_usize, Move(_12), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(11_usize, 22_usize, Move(_22), 10_usize, Move(_10), 2_usize, Move(_2), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(11_usize, 14_usize, Move(_14), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: ([usize; 4],),mut _2: Adt54,mut _3: [u128; 6],mut _4: u8,mut _5: u16,mut _6: u8,mut _7: [u128; 6],mut _8: [u128; 6],mut _9: *mut ([u16; 4], u128, u8, f32),mut _10: [u128; 6]) -> u8 {
mir! {
type RET = u8;
let _11: Adt53;
let _12: Adt49;
let _13: [i64; 3];
let _14: [usize; 8];
let _15: Adt48;
let _16: f32;
let _17: i16;
let _18: [u128; 6];
let _19: f32;
let _20: Adt48;
let _21: f32;
let _22: ([u16; 4], u128, u8, f32);
let _23: [bool; 3];
let _24: i8;
let _25: ();
let _26: ();
{
_5 = _2.fld4.fld0;
_1.0 = _2.fld0.0;
_1.0 = [4_usize,10508862321134118497_usize,0_usize,5905676049097075793_usize];
_2.fld5 = [(*_9).1,(*_9).1,(*_9).1,(*_9).1,(*_9).1,(*_9).1];
RET = (*_9).2 ^ _2.fld1;
_8 = _3;
(*_9).0 = [_5,_5,_5,_5];
_3 = _10;
_7 = [(*_9).1,(*_9).1,(*_9).1,(*_9).1,(*_9).1,(*_9).1];
Call((*_9) = fn13(_2.fld3, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_9).3 = (*_9).1 as f32;
_2.fld4 = Adt45 { fld0: _5 };
Goto(bb2)
}
bb2 = {
_15.fld3 = -(*_9).3;
_6 = !_4;
_5 = (-13189_i16) as u16;
_15.fld0 = core::ptr::addr_of_mut!((*_9));
_15.fld1.1 = [false,true,false];
_12 = Adt49::Variant0 { fld0: RET };
_4 = (*_9).3 as u8;
_14 = [2_usize,11117096613808645847_usize,2_usize,6_usize,15692333493039325131_usize,3_usize,7_usize,7_usize];
_2.fld2 = !9223372036854775807_isize;
SetDiscriminant(_12, 1);
_15.fld1.0 = [13275_i16,17264_i16,29604_i16,10464_i16,(-11846_i16),4709_i16,(-26117_i16)];
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_12, 1), 0)).3 = [false,false,false,true,false,false,true,true];
_14 = [6732287839325629291_usize,2_usize,16182028472964485817_usize,3_usize,8895191875999714521_usize,3_usize,0_usize,4_usize];
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_12, 1), 0)).2 = (-107_i8) as u8;
_15.fld1.4 = _3;
Goto(bb3)
}
bb3 = {
_4 = !RET;
_15.fld1.2 = _2.fld1 ^ _2.fld1;
_14 = [16717142368403679097_usize,4297994854864837692_usize,1_usize,5604808335313204111_usize,3_usize,13277475370433333487_usize,13697361253924008580_usize,4338560308143485883_usize];
_15.fld3 = (*_9).3 * (*_9).3;
_9 = core::ptr::addr_of_mut!((*_9));
_15.fld1.4 = [(*_9).1,(*_9).1,(*_9).1,(*_9).1,(*_9).1,(*_9).1];
_15.fld1.3 = [true,true,true,true,true,false,false,true];
_2.fld0 = _1;
_15.fld1.5 = [_5,_2.fld4.fld0,_2.fld4.fld0,_2.fld4.fld0];
Goto(bb4)
}
bb4 = {
(*_9) = (_15.fld1.5, 267207559134576255766239212047021770968_u128, _4, _15.fld3);
_3 = [(*_9).1,(*_9).1,(*_9).1,(*_9).1,(*_9).1,(*_9).1];
(*_9).0 = _15.fld1.5;
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_12, 1), 0)).4 = [(*_9).1,(*_9).1,(*_9).1,(*_9).1,(*_9).1,(*_9).1];
_15.fld1.4 = [(*_9).1,(*_9).1,(*_9).1,(*_9).1,(*_9).1,(*_9).1];
_7 = [(*_9).1,(*_9).1,(*_9).1,(*_9).1,(*_9).1,(*_9).1];
_15.fld2 = _14;
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_12, 1), 0)).3 = [false,false,true,false,true,false,false,true];
_15.fld4 = [15385_i16,13200_i16,18898_i16,(-12597_i16),32007_i16,(-13719_i16),11434_i16];
_15.fld3 = _15.fld1.2 as f32;
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_12, 1), 0)) = (_15.fld1.0, _15.fld1.1, _15.fld1.2, _15.fld1.3, _7, (*_9).0);
_2.fld4 = Adt45 { fld0: _5 };
_15.fld6 = [_2.fld4.fld0,_5,_5,_5,_5,_2.fld4.fld0];
_18 = [(*_9).1,(*_9).1,(*_9).1,(*_9).1,(*_9).1,(*_9).1];
(*_9).1 = 57505889213264984005432618420863526061_u128;
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_12, 1), 0)).2 = _2.fld2 as u8;
match (*_9).1 {
0 => bb5,
1 => bb6,
2 => bb7,
57505889213264984005432618420863526061 => bb9,
_ => bb8
}
}
bb5 = {
_4 = !RET;
_15.fld1.2 = _2.fld1 ^ _2.fld1;
_14 = [16717142368403679097_usize,4297994854864837692_usize,1_usize,5604808335313204111_usize,3_usize,13277475370433333487_usize,13697361253924008580_usize,4338560308143485883_usize];
_15.fld3 = (*_9).3 * (*_9).3;
_9 = core::ptr::addr_of_mut!((*_9));
_15.fld1.4 = [(*_9).1,(*_9).1,(*_9).1,(*_9).1,(*_9).1,(*_9).1];
_15.fld1.3 = [true,true,true,true,true,false,false,true];
_2.fld0 = _1;
_15.fld1.5 = [_5,_2.fld4.fld0,_2.fld4.fld0,_2.fld4.fld0];
Goto(bb4)
}
bb6 = {
_15.fld3 = -(*_9).3;
_6 = !_4;
_5 = (-13189_i16) as u16;
_15.fld0 = core::ptr::addr_of_mut!((*_9));
_15.fld1.1 = [false,true,false];
_12 = Adt49::Variant0 { fld0: RET };
_4 = (*_9).3 as u8;
_14 = [2_usize,11117096613808645847_usize,2_usize,6_usize,15692333493039325131_usize,3_usize,7_usize,7_usize];
_2.fld2 = !9223372036854775807_isize;
SetDiscriminant(_12, 1);
_15.fld1.0 = [13275_i16,17264_i16,29604_i16,10464_i16,(-11846_i16),4709_i16,(-26117_i16)];
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_12, 1), 0)).3 = [false,false,false,true,false,false,true,true];
_14 = [6732287839325629291_usize,2_usize,16182028472964485817_usize,3_usize,8895191875999714521_usize,3_usize,0_usize,4_usize];
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_12, 1), 0)).2 = (-107_i8) as u8;
_15.fld1.4 = _3;
Goto(bb3)
}
bb7 = {
(*_9).3 = (*_9).1 as f32;
_2.fld4 = Adt45 { fld0: _5 };
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_17 = 11175_i16 - 31406_i16;
_20.fld2 = _15.fld2;
_20.fld1.5 = [_2.fld4.fld0,_5,_2.fld4.fld0,_2.fld4.fld0];
_15.fld2 = _14;
_15.fld0 = core::ptr::addr_of_mut!((*_9));
_1 = (_2.fld0.0,);
_22 = (Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_12, 1), 0).5, (*_9).1, _2.fld1, (*_9).3);
_14 = [5_usize,2400919026991585487_usize,1084633199780438985_usize,13743195724840036042_usize,3570147030942128478_usize,8283002327360349107_usize,15754755751414270784_usize,9478354505646700739_usize];
Goto(bb10)
}
bb10 = {
_2.fld0.0 = _1.0;
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_12, 1), 0)) = _15.fld1;
_16 = _22.3 - _15.fld3;
_20.fld1.0 = [_17,_17,_17,_17,_17,_17,_17];
match _22.1 {
0 => bb2,
1 => bb11,
2 => bb12,
57505889213264984005432618420863526061 => bb14,
_ => bb13
}
}
bb11 = {
(*_9).3 = (*_9).1 as f32;
_2.fld4 = Adt45 { fld0: _5 };
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
_15.fld3 = -(*_9).3;
_6 = !_4;
_5 = (-13189_i16) as u16;
_15.fld0 = core::ptr::addr_of_mut!((*_9));
_15.fld1.1 = [false,true,false];
_12 = Adt49::Variant0 { fld0: RET };
_4 = (*_9).3 as u8;
_14 = [2_usize,11117096613808645847_usize,2_usize,6_usize,15692333493039325131_usize,3_usize,7_usize,7_usize];
_2.fld2 = !9223372036854775807_isize;
SetDiscriminant(_12, 1);
_15.fld1.0 = [13275_i16,17264_i16,29604_i16,10464_i16,(-11846_i16),4709_i16,(-26117_i16)];
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_12, 1), 0)).3 = [false,false,false,true,false,false,true,true];
_14 = [6732287839325629291_usize,2_usize,16182028472964485817_usize,3_usize,8895191875999714521_usize,3_usize,0_usize,4_usize];
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_12, 1), 0)).2 = (-107_i8) as u8;
_15.fld1.4 = _3;
Goto(bb3)
}
bb14 = {
_23 = [false,true,true];
(*_9).3 = _15.fld3 + _16;
place!(Field::<([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4])>(Variant(_12, 1), 0)).2 = (*_9).2;
_15.fld1.5 = [_5,_5,_5,_5];
_2.fld4 = Adt45 { fld0: _5 };
Goto(bb15)
}
bb15 = {
Call(_25 = dump_var(12_usize, 23_usize, Move(_23), 7_usize, Move(_7), 14_usize, Move(_14), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_25 = dump_var(12_usize, 10_usize, Move(_10), 3_usize, Move(_3), 26_usize, _26, 26_usize, _26), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: *mut ([u16; 4], u128, u8, f32),mut _2: ([usize; 4],)) -> ([u16; 4], u128, u8, f32) {
mir! {
type RET = ([u16; 4], u128, u8, f32);
let _3: Adt58;
let _4: char;
let _5: [usize; 8];
let _6: Adt48;
let _7: i32;
let _8: Adt54;
let _9: [i64; 3];
let _10: Adt48;
let _11: *const ([usize; 4],);
let _12: isize;
let _13: bool;
let _14: isize;
let _15: Adt46;
let _16: i8;
let _17: ();
let _18: ();
{
RET.2 = 69_u8 - 52_u8;
Call(RET.0 = fn14(_2, _2, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
Goto(bb2)
}
bb2 = {
RET.1 = 142380037886356489145334981298310446969_u128;
RET.3 = 5_usize as f32;
_1 = core::ptr::addr_of_mut!(RET);
RET.0 = [65065_u16,64597_u16,28197_u16,48740_u16];
(*_1).0 = [21970_u16,25235_u16,58356_u16,35781_u16];
_4 = '\u{c6444}';
Goto(bb3)
}
bb3 = {
_2.0 = [13585948202018756215_usize,4315033988987808918_usize,4853016464421964862_usize,11418308565955050838_usize];
RET.2 = 105_u8 & 150_u8;
_5 = [4452048692930260980_usize,1_usize,16719387962119825376_usize,4_usize,13182996712971780666_usize,7535033021110960454_usize,2_usize,4_usize];
RET.2 = (-27_i8) as u8;
RET.0 = [1187_u16,50364_u16,55651_u16,57767_u16];
Goto(bb4)
}
bb4 = {
_4 = '\u{aecc9}';
Goto(bb5)
}
bb5 = {
_6.fld1.5 = [21043_u16,63674_u16,19843_u16,9061_u16];
_6.fld2 = [8026425615368899107_usize,4_usize,17853179478816352939_usize,1_usize,5_usize,16328545372582156393_usize,1_usize,1_usize];
_6.fld1.1 = [false,true,true];
RET.3 = 45_i8 as f32;
_6.fld0 = _1;
_7 = (-24354120_i32);
_6.fld4 = [(-12408_i16),1700_i16,16521_i16,16797_i16,1149_i16,24570_i16,1001_i16];
RET.2 = 73_u8 ^ 162_u8;
_8.fld5 = [RET.1,(*_1).1,(*_1).1,RET.1,(*_1).1,(*_1).1];
(*_1).3 = (*_1).2 as f32;
_4 = '\u{af4da}';
Goto(bb6)
}
bb6 = {
_6.fld5 = [true,false,true,false,false];
_6.fld1.5 = (*_1).0;
_8.fld0 = (_2.0,);
_6.fld0 = _1;
_8.fld4 = Adt45 { fld0: 44338_u16 };
_10.fld1.3 = [true,true,true,false,true,false,false,true];
_5 = [7552621497432132226_usize,5_usize,6632919148327883873_usize,2579967868018199739_usize,10970801873540183030_usize,4_usize,5011938685287035091_usize,0_usize];
_6.fld3 = -RET.3;
_8.fld1 = !RET.2;
_8.fld4 = Adt45 { fld0: 65470_u16 };
_10.fld5 = [true,true,true,true,true];
_10.fld6 = [_8.fld4.fld0,_8.fld4.fld0,_8.fld4.fld0,_8.fld4.fld0,_8.fld4.fld0,_8.fld4.fld0];
_10.fld1.2 = !(*_1).2;
(*_1).2 = (*_1).3 as u8;
(*_1).2 = _10.fld1.2;
_7 = 1039921834_i32 + (-566610723_i32);
RET.3 = _6.fld3;
(*_1) = (_6.fld1.5, 338710935564423101188851514092678748214_u128, _8.fld1, _6.fld3);
Call(_10.fld0 = fn17(_6.fld2, _6.fld1.5, RET.1, (*_1).1, _5, _6.fld0, _1, RET.0, (*_1), _6.fld0, RET.1, _5, _6.fld0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
(*_1).3 = _6.fld3;
_1 = _6.fld0;
_10.fld4 = _6.fld4;
_6.fld6 = _10.fld6;
_15.fld2.2 = (*_1).2;
Goto(bb8)
}
bb8 = {
Call(_17 = dump_var(13_usize, 4_usize, Move(_4), 5_usize, Move(_5), 18_usize, _18, 18_usize, _18), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: ([usize; 4],),mut _2: ([usize; 4],),mut _3: *mut ([u16; 4], u128, u8, f32)) -> [u16; 4] {
mir! {
type RET = [u16; 4];
let _4: u128;
let _5: [i8; 3];
let _6: u128;
let _7: i128;
let _8: Adt51;
let _9: [u32; 7];
let _10: char;
let _11: ([u16; 4], u128, u8, f32);
let _12: [u16; 4];
let _13: ([usize; 4],);
let _14: *mut [i8; 3];
let _15: ([u16; 4], u128, u8, f32);
let _16: u8;
let _17: ([usize; 4],);
let _18: [u128; 6];
let _19: [usize; 8];
let _20: ([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4]);
let _21: f32;
let _22: *mut (f32, f64, u8);
let _23: i128;
let _24: [u16; 6];
let _25: [i16; 7];
let _26: f64;
let _27: bool;
let _28: [usize; 4];
let _29: [u128; 6];
let _30: f32;
let _31: f32;
let _32: [u16; 6];
let _33: *mut ([u16; 4], u128, u8, f32);
let _34: isize;
let _35: isize;
let _36: *mut bool;
let _37: ();
let _38: ();
{
RET = [61382_u16,6274_u16,8051_u16,38514_u16];
_4 = !276945836019635149432865180378080662261_u128;
_2 = (_1.0,);
_1 = (_2.0,);
RET = [37741_u16,19375_u16,63171_u16,22111_u16];
_2 = _1;
_1 = (_2.0,);
_1.0 = [3_usize,3277319715078963940_usize,3_usize,4_usize];
RET = [7998_u16,62215_u16,50859_u16,14914_u16];
_5 = [(-124_i8),(-65_i8),61_i8];
_5 = [(-110_i8),122_i8,(-60_i8)];
RET = [10302_u16,43613_u16,58385_u16,9696_u16];
_1.0 = [15276943033603975688_usize,3_usize,2_usize,2_usize];
_2.0 = _1.0;
_6 = _4 + _4;
_1.0 = [3111149738471933841_usize,4_usize,1_usize,6446720601416518639_usize];
RET = [12_u16,16086_u16,117_u16,21817_u16];
RET = [20224_u16,51476_u16,52941_u16,32026_u16];
RET = [51382_u16,16044_u16,32952_u16,11363_u16];
_2.0 = [7_usize,14342385593498277599_usize,1651653019742612058_usize,4_usize];
_8.fld3.0 = [(-31792_i16),(-30729_i16),32734_i16,(-472_i16),(-3563_i16),12649_i16,22064_i16];
_8.fld0.3 = 1_usize as f32;
_5 = [43_i8,(-87_i8),42_i8];
_3 = core::ptr::addr_of_mut!(_8.fld0);
Goto(bb1)
}
bb1 = {
(*_3).0 = [10190_u16,31675_u16,26382_u16,20330_u16];
(*_3).2 = !55_u8;
_4 = _6;
_8.fld3.2 = core::ptr::addr_of_mut!(_5);
_8.fld3.2 = core::ptr::addr_of_mut!(_5);
_1 = _2;
RET = [38901_u16,11917_u16,61081_u16,45151_u16];
_2 = _1;
_10 = '\u{64a1d}';
_7 = 114791404274311363664560803333602857748_i128 & 135103472978803619880567254077865778463_i128;
(*_3).3 = (-64_i8) as f32;
_11.2 = (*_3).2;
Goto(bb2)
}
bb2 = {
_12 = [51383_u16,53167_u16,61384_u16,59434_u16];
_14 = core::ptr::addr_of_mut!(_5);
Goto(bb3)
}
bb3 = {
_11.1 = !_4;
_11.0 = [32343_u16,41187_u16,38143_u16,31777_u16];
(*_3).0 = [6514_u16,28984_u16,16249_u16,36506_u16];
_12 = [34149_u16,45050_u16,10922_u16,28484_u16];
_8.fld0.0 = [26444_u16,38021_u16,12519_u16,7784_u16];
_8.fld3.1 = [15384183176791207094_usize,3612012686300154619_usize,7_usize,6_usize,7359545184135914864_usize,3_usize,2_usize,0_usize];
_8.fld0.1 = _6 - _6;
_8.fld3.0 = [(-9060_i16),4150_i16,23164_i16,(-28340_i16),469_i16,14082_i16,(-18383_i16)];
_15.3 = 62_i8 as f32;
_10 = '\u{c203f}';
_11 = (*_3);
_8.fld3.1 = [6756623429739582513_usize,5_usize,1_usize,13787548161845484112_usize,4_usize,3_usize,17537718008337578735_usize,8853853736348368559_usize];
_3 = core::ptr::addr_of_mut!(_8.fld0);
_11.0 = [25071_u16,8203_u16,59094_u16,45179_u16];
_16 = _8.fld0.2 + (*_3).2;
_14 = _8.fld3.2;
_8.fld0.3 = (-4093424397837507993_i64) as f32;
_15 = _8.fld0;
_14 = core::ptr::addr_of_mut!((*_14));
_11 = _8.fld0;
(*_3).1 = _15.1;
Goto(bb4)
}
bb4 = {
RET = [12903_u16,63199_u16,63654_u16,19973_u16];
_15.0 = [34462_u16,21190_u16,3884_u16,20237_u16];
_9 = [2281140412_u32,4135478536_u32,3119948269_u32,2596991022_u32,3455909425_u32,3354268979_u32,4250279897_u32];
(*_3).2 = 39519_u16 as u8;
_16 = (*_3).2;
_11.2 = 1444853419_u32 as u8;
_4 = _8.fld0.1 * _15.1;
(*_3).3 = -_15.3;
_8.fld0.2 = !_15.2;
_11.2 = _16 - _16;
_16 = !_11.2;
_7 = 148946043313458507304003966500371393220_i128;
_14 = core::ptr::addr_of_mut!(_5);
_1 = (_2.0,);
_17.0 = [3_usize,0_usize,11064011264221790481_usize,0_usize];
_14 = core::ptr::addr_of_mut!(_5);
_19 = [7_usize,3_usize,1536350213941271126_usize,3_usize,1_usize,842064145420051860_usize,1_usize,2_usize];
_8.fld0 = _11;
_20.0 = [31118_i16,16636_i16,(-4930_i16),(-27319_i16),(-10003_i16),(-20440_i16),12739_i16];
_8.fld3.2 = _14;
_8.fld3 = (_20.0, _19, _14);
_11.0 = [26079_u16,225_u16,16838_u16,54067_u16];
_8.fld0.0 = [52144_u16,49128_u16,1310_u16,22423_u16];
_1 = _2;
(*_3) = _11;
_8.fld3 = (_20.0, _19, _14);
(*_3).3 = 89_i8 as f32;
_15.3 = -(*_3).3;
_16 = !_8.fld0.2;
_20.1 = [false,false,false];
Call(_8.fld3.1 = fn15(_11.0, _11.2, _14, _17, (*_3), _8.fld3.2, RET, (*_3).2, _8.fld0, _1, _2.0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_18 = [(*_3).1,(*_3).1,_6,_11.1,_15.1,_4];
(*_3).0 = [15347_u16,65450_u16,53556_u16,39015_u16];
_15.2 = _16;
_1.0 = [0_usize,1_usize,6_usize,6_usize];
_15.2 = !_11.2;
(*_14) = [(-84_i8),(-68_i8),(-29_i8)];
_11.1 = _15.3 as u128;
(*_3).0 = [39723_u16,36332_u16,11553_u16,6176_u16];
_3 = core::ptr::addr_of_mut!(_8.fld0);
_21 = -_15.3;
_8.fld0.0 = [43822_u16,65114_u16,42801_u16,11430_u16];
_21 = _15.3 - _8.fld0.3;
_17 = _1;
_8.fld0.2 = _11.2 & _11.2;
_1.0 = _17.0;
(*_3).3 = 17949852014544831129_usize as f32;
(*_3).2 = !_11.2;
RET = [13334_u16,11507_u16,58054_u16,63670_u16];
_8.fld3.1 = _19;
_20.2 = _11.2;
_5 = [(-46_i8),57_i8,(-39_i8)];
_13 = _2;
_13.0 = [2315554008390550030_usize,1_usize,3_usize,18135771018471888099_usize];
_23 = _7;
_8.fld0.0 = _11.0;
(*_3).1 = _4;
Goto(bb6)
}
bb6 = {
(*_3).2 = _16 - _15.2;
_11 = (_12, (*_3).1, (*_3).2, _8.fld0.3);
_4 = 3863628632_u32 as u128;
_6 = _11.1 * (*_3).1;
_11.2 = !_8.fld0.2;
_8.fld3.2 = core::ptr::addr_of_mut!((*_14));
Goto(bb7)
}
bb7 = {
_20.2 = _16;
(*_14) = [119_i8,3_i8,56_i8];
_11.2 = !_15.2;
_15 = (RET, (*_3).1, _16, _21);
_17.0 = [16685972046158490341_usize,13117769647833162266_usize,3_usize,0_usize];
_11.3 = _15.3 - _21;
(*_14) = [53_i8,110_i8,72_i8];
_27 = false;
_20.5 = [51895_u16,14381_u16,26345_u16,31962_u16];
(*_3) = (RET, _6, _20.2, _11.3);
(*_3) = (_12, _6, _11.2, _11.3);
_20.5 = [7070_u16,58138_u16,457_u16,54834_u16];
_20.3 = [_27,_27,_27,_27,_27,_27,_27,_27];
(*_14) = [74_i8,9_i8,(-53_i8)];
_15.0 = _12;
_17 = (_2.0,);
_31 = _7 as f32;
_8.fld2 = (-13064_i16);
Call(_20.5 = core::intrinsics::transmute(_11.0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_20.5 = [17909_u16,43746_u16,29996_u16,4280_u16];
_13 = (_17.0,);
(*_3).2 = !_11.2;
(*_3).3 = _11.3;
_4 = !(*_3).1;
_8.fld0.2 = _16;
_3 = core::ptr::addr_of_mut!((*_3));
_11.3 = _11.1 as f32;
_1.0 = _2.0;
_33 = _3;
(*_33).1 = !_4;
_3 = _33;
(*_33).0 = [50677_u16,24750_u16,4089_u16,63270_u16];
_16 = _20.2;
_20.4 = _18;
_34 = 9223372036854775807_isize;
_11 = (_12, (*_33).1, _20.2, (*_3).3);
(*_3).3 = _34 as f32;
_27 = true;
_32 = [54215_u16,36251_u16,42168_u16,53933_u16,26850_u16,51458_u16];
Goto(bb9)
}
bb9 = {
_30 = -(*_33).3;
_25 = [_8.fld2,_8.fld2,_8.fld2,_8.fld2,_8.fld2,_8.fld2,_8.fld2];
_29 = [(*_3).1,(*_33).1,_11.1,(*_3).1,_15.1,(*_3).1];
_30 = _8.fld2 as f32;
(*_3).2 = _20.2;
_8.fld1 = [3932314471_u32,41043974_u32,3325971945_u32,2109423290_u32,1417912158_u32,4196373112_u32,2022103887_u32];
_15.3 = 6502993529750065866_u64 as f32;
_9 = _8.fld1;
(*_3).1 = 7440578517300105008_usize as u128;
_11.0 = (*_3).0;
_11.1 = _6;
(*_3).1 = 3864_u16 as u128;
match _34 {
0 => bb10,
1 => bb11,
2 => bb12,
9223372036854775807 => bb14,
_ => bb13
}
}
bb10 = {
_12 = [51383_u16,53167_u16,61384_u16,59434_u16];
_14 = core::ptr::addr_of_mut!(_5);
Goto(bb3)
}
bb11 = {
RET = [12903_u16,63199_u16,63654_u16,19973_u16];
_15.0 = [34462_u16,21190_u16,3884_u16,20237_u16];
_9 = [2281140412_u32,4135478536_u32,3119948269_u32,2596991022_u32,3455909425_u32,3354268979_u32,4250279897_u32];
(*_3).2 = 39519_u16 as u8;
_16 = (*_3).2;
_11.2 = 1444853419_u32 as u8;
_4 = _8.fld0.1 * _15.1;
(*_3).3 = -_15.3;
_8.fld0.2 = !_15.2;
_11.2 = _16 - _16;
_16 = !_11.2;
_7 = 148946043313458507304003966500371393220_i128;
_14 = core::ptr::addr_of_mut!(_5);
_1 = (_2.0,);
_17.0 = [3_usize,0_usize,11064011264221790481_usize,0_usize];
_14 = core::ptr::addr_of_mut!(_5);
_19 = [7_usize,3_usize,1536350213941271126_usize,3_usize,1_usize,842064145420051860_usize,1_usize,2_usize];
_8.fld0 = _11;
_20.0 = [31118_i16,16636_i16,(-4930_i16),(-27319_i16),(-10003_i16),(-20440_i16),12739_i16];
_8.fld3.2 = _14;
_8.fld3 = (_20.0, _19, _14);
_11.0 = [26079_u16,225_u16,16838_u16,54067_u16];
_8.fld0.0 = [52144_u16,49128_u16,1310_u16,22423_u16];
_1 = _2;
(*_3) = _11;
_8.fld3 = (_20.0, _19, _14);
(*_3).3 = 89_i8 as f32;
_15.3 = -(*_3).3;
_16 = !_8.fld0.2;
_20.1 = [false,false,false];
Call(_8.fld3.1 = fn15(_11.0, _11.2, _14, _17, (*_3), _8.fld3.2, RET, (*_3).2, _8.fld0, _1, _2.0), ReturnTo(bb5), UnwindUnreachable())
}
bb12 = {
(*_3).2 = _16 - _15.2;
_11 = (_12, (*_3).1, (*_3).2, _8.fld0.3);
_4 = 3863628632_u32 as u128;
_6 = _11.1 * (*_3).1;
_11.2 = !_8.fld0.2;
_8.fld3.2 = core::ptr::addr_of_mut!((*_14));
Goto(bb7)
}
bb13 = {
(*_3).0 = [10190_u16,31675_u16,26382_u16,20330_u16];
(*_3).2 = !55_u8;
_4 = _6;
_8.fld3.2 = core::ptr::addr_of_mut!(_5);
_8.fld3.2 = core::ptr::addr_of_mut!(_5);
_1 = _2;
RET = [38901_u16,11917_u16,61081_u16,45151_u16];
_2 = _1;
_10 = '\u{64a1d}';
_7 = 114791404274311363664560803333602857748_i128 & 135103472978803619880567254077865778463_i128;
(*_3).3 = (-64_i8) as f32;
_11.2 = (*_3).2;
Goto(bb2)
}
bb14 = {
_14 = _8.fld3.2;
(*_3).0 = RET;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(14_usize, 32_usize, Move(_32), 4_usize, Move(_4), 1_usize, Move(_1), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(14_usize, 7_usize, Move(_7), 6_usize, Move(_6), 9_usize, Move(_9), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(14_usize, 25_usize, Move(_25), 34_usize, Move(_34), 38_usize, _38, 38_usize, _38), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: [u16; 4],mut _2: u8,mut _3: *mut [i8; 3],mut _4: ([usize; 4],),mut _5: ([u16; 4], u128, u8, f32),mut _6: *mut [i8; 3],mut _7: [u16; 4],mut _8: u8,mut _9: ([u16; 4], u128, u8, f32),mut _10: ([usize; 4],),mut _11: [usize; 4]) -> [usize; 8] {
mir! {
type RET = [usize; 8];
let _12: Adt53;
let _13: (f32, f64, u8);
let _14: [u16; 4];
let _15: Adt59;
let _16: Adt47;
let _17: u8;
let _18: u32;
let _19: f32;
let _20: ([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4]);
let _21: isize;
let _22: [u32; 7];
let _23: i64;
let _24: ([usize; 4],);
let _25: i16;
let _26: ([u16; 4], u128, u8, f32);
let _27: [u16; 4];
let _28: [u128; 6];
let _29: ([i16; 7], [usize; 8], *mut [i8; 3]);
let _30: ([usize; 4],);
let _31: f64;
let _32: isize;
let _33: *const usize;
let _34: Adt46;
let _35: isize;
let _36: Adt48;
let _37: f64;
let _38: *mut ([u16; 4], u128, u8, f32);
let _39: [i16; 7];
let _40: u128;
let _41: ([i16; 7], [usize; 8], *mut [i8; 3]);
let _42: char;
let _43: i16;
let _44: isize;
let _45: i64;
let _46: i128;
let _47: Adt49;
let _48: ();
let _49: ();
{
_7 = _1;
_5.0 = [22569_u16,43721_u16,23300_u16,22473_u16];
_5.3 = _9.3 - _9.3;
_5.1 = !_9.1;
_5.2 = _2;
RET = [16784587135375766283_usize,7_usize,11517557675256435569_usize,2920476541491663658_usize,7_usize,11982757771314724789_usize,12904867002147389231_usize,0_usize];
(*_6) = [(-122_i8),(-75_i8),36_i8];
_2 = _9.2 & _9.2;
_9.2 = _5.2;
_6 = core::ptr::addr_of_mut!((*_3));
(*_6) = [(-29_i8),(-85_i8),(-115_i8)];
_4 = (_10.0,);
(*_3) = [(-70_i8),(-41_i8),(-76_i8)];
_9 = (_7, _5.1, _2, _5.3);
_11 = _4.0;
_6 = _3;
_1 = _5.0;
_9.2 = _2;
_4 = (_10.0,);
Goto(bb1)
}
bb1 = {
RET = [15644847932902507199_usize,17187133785221404650_usize,0_usize,7657181295949021814_usize,12302107630007702084_usize,2_usize,2_usize,1_usize];
_9.1 = _5.3 as u128;
(*_6) = [54_i8,(-78_i8),80_i8];
_9.1 = _5.1;
_11 = _10.0;
_9.3 = -_5.3;
_9.2 = !_2;
_4 = (_10.0,);
RET = [5_usize,13539240136003689780_usize,2_usize,371714960839927029_usize,5789599910831038023_usize,2_usize,1_usize,1_usize];
_9.2 = _2 << _2;
_2 = !_9.2;
_7 = [21655_u16,59417_u16,60679_u16,15608_u16];
_6 = core::ptr::addr_of_mut!((*_6));
RET = [7_usize,5090325465273030207_usize,9290929215157094047_usize,6_usize,4563936944384078496_usize,7791614290013675744_usize,8194492979122351429_usize,3412451647910099601_usize];
(*_3) = [(-58_i8),(-86_i8),66_i8];
_9.1 = 1_usize as u128;
_4.0 = [2668704994944246578_usize,1159914715877250331_usize,721190560144379833_usize,5_usize];
_9.0 = [46758_u16,26740_u16,26410_u16,30050_u16];
Goto(bb2)
}
bb2 = {
_9.2 = !_5.2;
_7 = _1;
_4 = (_11,);
_13.0 = -_9.3;
(*_3) = [(-101_i8),(-55_i8),(-35_i8)];
_10 = _4;
(*_3) = [(-50_i8),(-113_i8),(-9_i8)];
_13.2 = _5.2;
_9.1 = _5.1;
_5.0 = _9.0;
Goto(bb3)
}
bb3 = {
(*_6) = [(-42_i8),(-123_i8),55_i8];
_4.0 = [4_usize,1_usize,5_usize,6_usize];
_10 = _4;
(*_3) = [(-26_i8),99_i8,76_i8];
_5 = _9;
_2 = !_8;
_9.0 = [25530_u16,22962_u16,65429_u16,3109_u16];
_9 = (_7, _5.1, _2, _13.0);
_5.2 = 4305128113900683837_i64 as u8;
_10 = _4;
_9.1 = _5.1 * _5.1;
_4.0 = [4_usize,17657233225358189070_usize,6_usize,2_usize];
_9.3 = _13.0 + _5.3;
_4.0 = [12856183155483313362_usize,4_usize,6_usize,2_usize];
(*_3) = [10_i8,(-93_i8),(-117_i8)];
_5.1 = _9.1 ^ _9.1;
_17 = _9.2;
_5.0 = _9.0;
_4 = (_10.0,);
_13.0 = _9.3 - _5.3;
_1 = [28381_u16,43371_u16,50455_u16,5838_u16];
_9.2 = _8;
_5.0 = _7;
(*_3) = [(-87_i8),55_i8,11_i8];
_5.2 = _9.2;
Call(_9.0 = core::intrinsics::transmute(_5.0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_10.0 = [13033731245844148769_usize,6847640825834217492_usize,16916447608405308732_usize,2_usize];
_13.1 = 7_usize as f64;
_13.0 = 2246314609587975594_usize as f32;
_5.2 = _8;
_4.0 = _10.0;
_4.0 = [4_usize,6521033243864805094_usize,6_usize,6416947539945539424_usize];
_10.0 = [11774710790991423338_usize,2_usize,6_usize,14790598832243485565_usize];
_3 = _6;
(*_6) = [(-4_i8),110_i8,(-60_i8)];
_20.1 = [false,false,true];
_21 = 59786_u16 as isize;
_5.0 = [34736_u16,47879_u16,44573_u16,55430_u16];
_20.0 = [(-11441_i16),639_i16,366_i16,12291_i16,7417_i16,9056_i16,(-29532_i16)];
_6 = core::ptr::addr_of_mut!((*_3));
_5.0 = [1304_u16,18951_u16,45099_u16,54958_u16];
_19 = (-166521015903739276260406090716779111004_i128) as f32;
_14 = [21987_u16,65507_u16,39678_u16,36761_u16];
_13.0 = _21 as f32;
(*_6) = [(-54_i8),90_i8,39_i8];
_20.4 = [_5.1,_5.1,_5.1,_5.1,_5.1,_5.1];
_8 = _17;
_20.3 = [false,false,false,true,true,false,true,false];
_6 = core::ptr::addr_of_mut!((*_3));
_5 = (_14, _9.1, _8, _9.3);
_23 = -7525324907643046011_i64;
Call(_9.2 = core::intrinsics::bswap(_5.2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_22 = [3589925045_u32,1740330420_u32,931185330_u32,3985277095_u32,1838946994_u32,2844107624_u32,449627015_u32];
_26 = _5;
_13.1 = _26.3 as f64;
_23 = 25601_u16 as i64;
(*_6) = [12_i8,71_i8,122_i8];
_20.1 = [false,false,false];
_10.0 = [6_usize,17569499215110345405_usize,2456246974400893019_usize,6871896345577609672_usize];
_6 = _3;
_26.3 = _2 as f32;
_2 = !_8;
_5.0 = _1;
_19 = -_13.0;
_20.1 = [true,true,true];
_30 = (_10.0,);
_19 = -_5.3;
_25 = 9679289177966772781_u64 as i16;
_29.1 = [8195288320792348447_usize,2_usize,6_usize,5_usize,17905724434304680507_usize,5_usize,8747741250713500280_usize,14578779969134229206_usize];
_5.0 = [31416_u16,17994_u16,8346_u16,19363_u16];
_30 = (_11,);
(*_3) = [(-105_i8),(-93_i8),(-33_i8)];
_29 = (_20.0, RET, _6);
_25 = (-32564_i16);
_9.3 = -_19;
_7 = [27089_u16,42996_u16,54259_u16,45408_u16];
match _25 {
0 => bb2,
340282366920938463463374607431768178892 => bb7,
_ => bb6
}
}
bb6 = {
_10.0 = [13033731245844148769_usize,6847640825834217492_usize,16916447608405308732_usize,2_usize];
_13.1 = 7_usize as f64;
_13.0 = 2246314609587975594_usize as f32;
_5.2 = _8;
_4.0 = _10.0;
_4.0 = [4_usize,6521033243864805094_usize,6_usize,6416947539945539424_usize];
_10.0 = [11774710790991423338_usize,2_usize,6_usize,14790598832243485565_usize];
_3 = _6;
(*_6) = [(-4_i8),110_i8,(-60_i8)];
_20.1 = [false,false,true];
_21 = 59786_u16 as isize;
_5.0 = [34736_u16,47879_u16,44573_u16,55430_u16];
_20.0 = [(-11441_i16),639_i16,366_i16,12291_i16,7417_i16,9056_i16,(-29532_i16)];
_6 = core::ptr::addr_of_mut!((*_3));
_5.0 = [1304_u16,18951_u16,45099_u16,54958_u16];
_19 = (-166521015903739276260406090716779111004_i128) as f32;
_14 = [21987_u16,65507_u16,39678_u16,36761_u16];
_13.0 = _21 as f32;
(*_6) = [(-54_i8),90_i8,39_i8];
_20.4 = [_5.1,_5.1,_5.1,_5.1,_5.1,_5.1];
_8 = _17;
_20.3 = [false,false,false,true,true,false,true,false];
_6 = core::ptr::addr_of_mut!((*_3));
_5 = (_14, _9.1, _8, _9.3);
_23 = -7525324907643046011_i64;
Call(_9.2 = core::intrinsics::bswap(_5.2), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_9.2 = !_2;
_26.2 = !_17;
RET = _29.1;
RET = [7_usize,3_usize,3874858601656965317_usize,11174630882537628339_usize,13307712869264602189_usize,1_usize,2943444741884979173_usize,7_usize];
_9.2 = !_26.2;
(*_3) = [69_i8,80_i8,17_i8];
_5.3 = 44288_u16 as f32;
_10.0 = [1336441013203612567_usize,0_usize,8039277686653110776_usize,7_usize];
_26 = _5;
_20.1 = [true,false,true];
_12 = Adt53::Variant2 { fld0: _20.4 };
_20.5 = [61971_u16,12310_u16,33571_u16,29600_u16];
_29.0 = [_25,_25,_25,_25,_25,_25,_25];
Call(_2 = core::intrinsics::transmute(_17), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
(*_3) = [44_i8,(-95_i8),(-42_i8)];
_30 = _10;
_30.0 = [1_usize,4007907352815828722_usize,3_usize,2_usize];
_7 = [64447_u16,18385_u16,37377_u16,13003_u16];
_18 = !1918853279_u32;
_28 = [_5.1,_5.1,_5.1,_5.1,_5.1,_5.1];
_20.5 = _14;
place!(Field::<[u128; 6]>(Variant(_12, 2), 0)) = [_26.1,_26.1,_5.1,_9.1,_9.1,_26.1];
_30 = _4;
_8 = !_17;
RET = [2249096731811702611_usize,11227115422717189491_usize,7_usize,9360704104126614876_usize,5_usize,4_usize,1037103059485644938_usize,0_usize];
_17 = _9.2;
_3 = core::ptr::addr_of_mut!((*_6));
_25 = 17594095817304333694_u64 as i16;
_11 = [1237289819466496364_usize,5_usize,6_usize,14532038351064418694_usize];
_26.2 = _8;
_18 = 962894430_u32;
_29 = (_20.0, RET, _6);
_36.fld3 = _9.3;
_14 = [55056_u16,63292_u16,19042_u16,3819_u16];
_5.1 = 1463529368_i32 as u128;
_36.fld1.2 = _2 ^ _5.2;
match _18 {
0 => bb1,
1 => bb4,
2 => bb6,
3 => bb9,
4 => bb10,
962894430 => bb12,
_ => bb11
}
}
bb9 = {
_9.2 = !_2;
_26.2 = !_17;
RET = _29.1;
RET = [7_usize,3_usize,3874858601656965317_usize,11174630882537628339_usize,13307712869264602189_usize,1_usize,2943444741884979173_usize,7_usize];
_9.2 = !_26.2;
(*_3) = [69_i8,80_i8,17_i8];
_5.3 = 44288_u16 as f32;
_10.0 = [1336441013203612567_usize,0_usize,8039277686653110776_usize,7_usize];
_26 = _5;
_20.1 = [true,false,true];
_12 = Adt53::Variant2 { fld0: _20.4 };
_20.5 = [61971_u16,12310_u16,33571_u16,29600_u16];
_29.0 = [_25,_25,_25,_25,_25,_25,_25];
Call(_2 = core::intrinsics::transmute(_17), ReturnTo(bb8), UnwindUnreachable())
}
bb10 = {
RET = [15644847932902507199_usize,17187133785221404650_usize,0_usize,7657181295949021814_usize,12302107630007702084_usize,2_usize,2_usize,1_usize];
_9.1 = _5.3 as u128;
(*_6) = [54_i8,(-78_i8),80_i8];
_9.1 = _5.1;
_11 = _10.0;
_9.3 = -_5.3;
_9.2 = !_2;
_4 = (_10.0,);
RET = [5_usize,13539240136003689780_usize,2_usize,371714960839927029_usize,5789599910831038023_usize,2_usize,1_usize,1_usize];
_9.2 = _2 << _2;
_2 = !_9.2;
_7 = [21655_u16,59417_u16,60679_u16,15608_u16];
_6 = core::ptr::addr_of_mut!((*_6));
RET = [7_usize,5090325465273030207_usize,9290929215157094047_usize,6_usize,4563936944384078496_usize,7791614290013675744_usize,8194492979122351429_usize,3412451647910099601_usize];
(*_3) = [(-58_i8),(-86_i8),66_i8];
_9.1 = 1_usize as u128;
_4.0 = [2668704994944246578_usize,1159914715877250331_usize,721190560144379833_usize,5_usize];
_9.0 = [46758_u16,26740_u16,26410_u16,30050_u16];
Goto(bb2)
}
bb11 = {
_22 = [3589925045_u32,1740330420_u32,931185330_u32,3985277095_u32,1838946994_u32,2844107624_u32,449627015_u32];
_26 = _5;
_13.1 = _26.3 as f64;
_23 = 25601_u16 as i64;
(*_6) = [12_i8,71_i8,122_i8];
_20.1 = [false,false,false];
_10.0 = [6_usize,17569499215110345405_usize,2456246974400893019_usize,6871896345577609672_usize];
_6 = _3;
_26.3 = _2 as f32;
_2 = !_8;
_5.0 = _1;
_19 = -_13.0;
_20.1 = [true,true,true];
_30 = (_10.0,);
_19 = -_5.3;
_25 = 9679289177966772781_u64 as i16;
_29.1 = [8195288320792348447_usize,2_usize,6_usize,5_usize,17905724434304680507_usize,5_usize,8747741250713500280_usize,14578779969134229206_usize];
_5.0 = [31416_u16,17994_u16,8346_u16,19363_u16];
_30 = (_11,);
(*_3) = [(-105_i8),(-93_i8),(-33_i8)];
_29 = (_20.0, RET, _6);
_25 = (-32564_i16);
_9.3 = -_19;
_7 = [27089_u16,42996_u16,54259_u16,45408_u16];
match _25 {
0 => bb2,
340282366920938463463374607431768178892 => bb7,
_ => bb6
}
}
bb12 = {
_34.fld2.4 = Field::<[u128; 6]>(Variant(_12, 2), 0);
_18 = 1189782376_u32 >> _5.2;
_20.2 = _36.fld1.2 - _8;
RET = [10829248989544854905_usize,5_usize,17023109305526760151_usize,0_usize,12421848808002704305_usize,8300648043200950431_usize,1900162132098367186_usize,0_usize];
_34.fld2.0 = [_25,_25,_25,_25,_25,_25,_25];
_33 = core::ptr::addr_of!(_34.fld1);
_34.fld2.2 = _20.2;
_20.1 = [true,true,true];
_34.fld2 = (_29.0, _20.1, _20.2, _20.3, Field::<[u128; 6]>(Variant(_12, 2), 0), _14);
_32 = 25453_u16 as isize;
_30.0 = _11;
_11 = _4.0;
Call(_36.fld4 = fn16(_20.4, (*_3), Move(_12), _6, _20.1, _26.1, _13, _13), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_34.fld1 = 16818265620988513724_usize ^ 8780004550342075391_usize;
_9.1 = _26.1 & _26.1;
_36.fld1.5 = [63527_u16,45680_u16,34079_u16,54901_u16];
(*_3) = [101_i8,(-5_i8),20_i8];
(*_6) = [110_i8,32_i8,114_i8];
_36.fld0 = core::ptr::addr_of_mut!(_9);
_36.fld1.1 = [false,true,false];
_34.fld3 = (-37_i8);
match _34.fld3 {
0 => bb8,
1 => bb2,
2 => bb9,
3 => bb7,
4 => bb14,
5 => bb15,
340282366920938463463374607431768211419 => bb17,
_ => bb16
}
}
bb14 = {
_34.fld2.4 = Field::<[u128; 6]>(Variant(_12, 2), 0);
_18 = 1189782376_u32 >> _5.2;
_20.2 = _36.fld1.2 - _8;
RET = [10829248989544854905_usize,5_usize,17023109305526760151_usize,0_usize,12421848808002704305_usize,8300648043200950431_usize,1900162132098367186_usize,0_usize];
_34.fld2.0 = [_25,_25,_25,_25,_25,_25,_25];
_33 = core::ptr::addr_of!(_34.fld1);
_34.fld2.2 = _20.2;
_20.1 = [true,true,true];
_34.fld2 = (_29.0, _20.1, _20.2, _20.3, Field::<[u128; 6]>(Variant(_12, 2), 0), _14);
_32 = 25453_u16 as isize;
_30.0 = _11;
_11 = _4.0;
Call(_36.fld4 = fn16(_20.4, (*_3), Move(_12), _6, _20.1, _26.1, _13, _13), ReturnTo(bb13), UnwindUnreachable())
}
bb15 = {
_10.0 = [13033731245844148769_usize,6847640825834217492_usize,16916447608405308732_usize,2_usize];
_13.1 = 7_usize as f64;
_13.0 = 2246314609587975594_usize as f32;
_5.2 = _8;
_4.0 = _10.0;
_4.0 = [4_usize,6521033243864805094_usize,6_usize,6416947539945539424_usize];
_10.0 = [11774710790991423338_usize,2_usize,6_usize,14790598832243485565_usize];
_3 = _6;
(*_6) = [(-4_i8),110_i8,(-60_i8)];
_20.1 = [false,false,true];
_21 = 59786_u16 as isize;
_5.0 = [34736_u16,47879_u16,44573_u16,55430_u16];
_20.0 = [(-11441_i16),639_i16,366_i16,12291_i16,7417_i16,9056_i16,(-29532_i16)];
_6 = core::ptr::addr_of_mut!((*_3));
_5.0 = [1304_u16,18951_u16,45099_u16,54958_u16];
_19 = (-166521015903739276260406090716779111004_i128) as f32;
_14 = [21987_u16,65507_u16,39678_u16,36761_u16];
_13.0 = _21 as f32;
(*_6) = [(-54_i8),90_i8,39_i8];
_20.4 = [_5.1,_5.1,_5.1,_5.1,_5.1,_5.1];
_8 = _17;
_20.3 = [false,false,false,true,true,false,true,false];
_6 = core::ptr::addr_of_mut!((*_3));
_5 = (_14, _9.1, _8, _9.3);
_23 = -7525324907643046011_i64;
Call(_9.2 = core::intrinsics::bswap(_5.2), ReturnTo(bb5), UnwindUnreachable())
}
bb16 = {
RET = [15644847932902507199_usize,17187133785221404650_usize,0_usize,7657181295949021814_usize,12302107630007702084_usize,2_usize,2_usize,1_usize];
_9.1 = _5.3 as u128;
(*_6) = [54_i8,(-78_i8),80_i8];
_9.1 = _5.1;
_11 = _10.0;
_9.3 = -_5.3;
_9.2 = !_2;
_4 = (_10.0,);
RET = [5_usize,13539240136003689780_usize,2_usize,371714960839927029_usize,5789599910831038023_usize,2_usize,1_usize,1_usize];
_9.2 = _2 << _2;
_2 = !_9.2;
_7 = [21655_u16,59417_u16,60679_u16,15608_u16];
_6 = core::ptr::addr_of_mut!((*_6));
RET = [7_usize,5090325465273030207_usize,9290929215157094047_usize,6_usize,4563936944384078496_usize,7791614290013675744_usize,8194492979122351429_usize,3412451647910099601_usize];
(*_3) = [(-58_i8),(-86_i8),66_i8];
_9.1 = 1_usize as u128;
_4.0 = [2668704994944246578_usize,1159914715877250331_usize,721190560144379833_usize,5_usize];
_9.0 = [46758_u16,26740_u16,26410_u16,30050_u16];
Goto(bb2)
}
bb17 = {
_34.fld2.2 = _36.fld1.2 | _26.2;
(*_3) = [_34.fld3,_34.fld3,_34.fld3];
_41.0 = _29.0;
_31 = _13.1;
_32 = _21;
_24.0 = [_34.fld1,(*_33),(*_33),(*_33)];
_11 = _30.0;
_41.0 = _34.fld2.0;
_2 = (*_33) as u8;
_34.fld0 = core::ptr::addr_of_mut!(_43);
_4 = _30;
_33 = core::ptr::addr_of!((*_33));
_20.3 = _34.fld2.3;
_36.fld1.3 = [false,true,false,true,false,true,false,true];
_36.fld1.4 = [_9.1,_26.1,_9.1,_26.1,_26.1,_9.1];
_5 = (_7, _26.1, _20.2, _36.fld3);
_1 = [40254_u16,28899_u16,51015_u16,58595_u16];
_36.fld0 = core::ptr::addr_of_mut!(_26);
_42 = '\u{10e91b}';
_45 = true as i64;
_32 = -_21;
_14 = [49061_u16,11279_u16,6628_u16,39680_u16];
(*_6) = [_34.fld3,_34.fld3,_34.fld3];
_26.2 = _34.fld2.2;
_45 = -_23;
Goto(bb18)
}
bb18 = {
Call(_48 = dump_var(15_usize, 23_usize, Move(_23), 28_usize, Move(_28), 45_usize, Move(_45), 10_usize, Move(_10)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_48 = dump_var(15_usize, 25_usize, Move(_25), 4_usize, Move(_4), 22_usize, Move(_22), 21_usize, Move(_21)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_48 = dump_var(15_usize, 11_usize, Move(_11), 14_usize, Move(_14), 49_usize, _49, 49_usize, _49), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: [u128; 6],mut _2: [i8; 3],mut _3: Adt53,mut _4: *mut [i8; 3],mut _5: [bool; 3],mut _6: u128,mut _7: (f32, f64, u8),mut _8: (f32, f64, u8)) -> [i16; 7] {
mir! {
type RET = [i16; 7];
let _9: [u32; 7];
let _10: bool;
let _11: Adt58;
let _12: i64;
let _13: isize;
let _14: u8;
let _15: bool;
let _16: ([usize; 4],);
let _17: [i8; 3];
let _18: *mut [i8; 3];
let _19: [i16; 1];
let _20: usize;
let _21: (f32, f64, u8);
let _22: [usize; 4];
let _23: [bool; 5];
let _24: char;
let _25: *const ([usize; 4],);
let _26: i64;
let _27: f32;
let _28: i128;
let _29: Adt54;
let _30: Adt60;
let _31: ();
let _32: ();
{
RET = [(-19073_i16),25055_i16,(-27775_i16),26848_i16,(-5510_i16),7802_i16,(-18977_i16)];
place!(Field::<[u128; 6]>(Variant(_3, 2), 0)) = [_6,_6,_6,_6,_6,_6];
(*_4) = _2;
RET = [(-21928_i16),15959_i16,(-12253_i16),4220_i16,(-7061_i16),17287_i16,14085_i16];
_7.1 = _8.1;
_8.1 = _7.1 - _7.1;
(*_4) = _2;
_8.2 = !_7.2;
_5 = [false,false,true];
_1 = [_6,_6,_6,_6,_6,_6];
_4 = core::ptr::addr_of_mut!((*_4));
_8.1 = -_7.1;
Goto(bb1)
}
bb1 = {
_7.0 = _8.0 + _8.0;
_8.1 = -_7.1;
(*_4) = _2;
(*_4) = _2;
_8.2 = !_7.2;
RET = [5641_i16,19744_i16,(-18316_i16),(-21388_i16),27131_i16,(-13540_i16),(-6738_i16)];
RET = [20546_i16,(-2839_i16),13653_i16,6277_i16,1743_i16,(-5706_i16),(-12064_i16)];
SetDiscriminant(_3, 2);
_5 = [true,false,true];
_8.1 = _7.1;
(*_4) = [(-59_i8),70_i8,14_i8];
_7.0 = -_8.0;
(*_4) = [93_i8,(-110_i8),(-110_i8)];
Goto(bb2)
}
bb2 = {
_2 = [(-95_i8),(-53_i8),87_i8];
place!(Field::<[u128; 6]>(Variant(_3, 2), 0)) = [_6,_6,_6,_6,_6,_6];
(*_4) = [(-35_i8),(-117_i8),(-59_i8)];
_7.2 = _8.2 - _8.2;
_2 = (*_4);
_10 = _6 >= _6;
_12 = !(-4835856875631959535_i64);
_8 = (_7.0, _7.1, _7.2);
SetDiscriminant(_3, 0);
(*_4) = _2;
_5 = [_10,_10,_10];
place!(Field::<f32>(Variant(_3, 0), 0)) = _8.2 as f32;
Goto(bb3)
}
bb3 = {
_1 = [_6,_6,_6,_6,_6,_6];
_13 = (-1297829327_i32) as isize;
_9 = [739756013_u32,4007075314_u32,3555597531_u32,3866755936_u32,1393367014_u32,1944922100_u32,3407899294_u32];
_13 = _7.1 as isize;
place!(Field::<f32>(Variant(_3, 0), 0)) = -_8.0;
_5 = [_10,_10,_10];
place!(Field::<f32>(Variant(_3, 0), 0)) = _8.0 + _8.0;
Goto(bb4)
}
bb4 = {
_13 = (-9223372036854775808_isize);
_15 = _8.2 <= _8.2;
_7.0 = _6 as f32;
_2 = (*_4);
_8.1 = 16152079612626223172_usize as f64;
_17 = [(-79_i8),100_i8,45_i8];
_17 = [(-112_i8),114_i8,88_i8];
_2 = [54_i8,67_i8,(-69_i8)];
_17 = [(-41_i8),67_i8,107_i8];
_17 = [(-42_i8),45_i8,(-8_i8)];
_1 = [_6,_6,_6,_6,_6,_6];
_8.2 = _7.2 ^ _7.2;
_8 = (_7.0, _7.1, _7.2);
_8 = (Field::<f32>(Variant(_3, 0), 0), _7.1, _7.2);
_9 = [3119862826_u32,3828740550_u32,3161126534_u32,3032228712_u32,1061087272_u32,893340370_u32,4162207680_u32];
_18 = core::ptr::addr_of_mut!(_17);
_2 = [(-16_i8),(-11_i8),(-92_i8)];
_14 = !_7.2;
_14 = !_8.2;
(*_4) = [98_i8,69_i8,86_i8];
_19 = [28489_i16];
_2 = (*_18);
(*_4) = [(-36_i8),(-46_i8),9_i8];
_19 = [(-24146_i16)];
_17 = [38_i8,81_i8,(-28_i8)];
place!(Field::<[bool; 5]>(Variant(_3, 0), 1)) = [_15,_15,_15,_10,_15];
(*_4) = _17;
match _13 {
0 => bb5,
1 => bb6,
2 => bb7,
3 => bb8,
340282366920938463454151235394913435648 => bb10,
_ => bb9
}
}
bb5 = {
_1 = [_6,_6,_6,_6,_6,_6];
_13 = (-1297829327_i32) as isize;
_9 = [739756013_u32,4007075314_u32,3555597531_u32,3866755936_u32,1393367014_u32,1944922100_u32,3407899294_u32];
_13 = _7.1 as isize;
place!(Field::<f32>(Variant(_3, 0), 0)) = -_8.0;
_5 = [_10,_10,_10];
place!(Field::<f32>(Variant(_3, 0), 0)) = _8.0 + _8.0;
Goto(bb4)
}
bb6 = {
_2 = [(-95_i8),(-53_i8),87_i8];
place!(Field::<[u128; 6]>(Variant(_3, 2), 0)) = [_6,_6,_6,_6,_6,_6];
(*_4) = [(-35_i8),(-117_i8),(-59_i8)];
_7.2 = _8.2 - _8.2;
_2 = (*_4);
_10 = _6 >= _6;
_12 = !(-4835856875631959535_i64);
_8 = (_7.0, _7.1, _7.2);
SetDiscriminant(_3, 0);
(*_4) = _2;
_5 = [_10,_10,_10];
place!(Field::<f32>(Variant(_3, 0), 0)) = _8.2 as f32;
Goto(bb3)
}
bb7 = {
_7.0 = _8.0 + _8.0;
_8.1 = -_7.1;
(*_4) = _2;
(*_4) = _2;
_8.2 = !_7.2;
RET = [5641_i16,19744_i16,(-18316_i16),(-21388_i16),27131_i16,(-13540_i16),(-6738_i16)];
RET = [20546_i16,(-2839_i16),13653_i16,6277_i16,1743_i16,(-5706_i16),(-12064_i16)];
SetDiscriminant(_3, 2);
_5 = [true,false,true];
_8.1 = _7.1;
(*_4) = [(-59_i8),70_i8,14_i8];
_7.0 = -_8.0;
(*_4) = [93_i8,(-110_i8),(-110_i8)];
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_9 = [606831372_u32,1853616603_u32,4218528388_u32,3307526340_u32,2050341429_u32,4119099287_u32,1302176255_u32];
place!(Field::<[bool; 5]>(Variant(_3, 0), 1)) = [_10,_10,_15,_10,_10];
_2 = [(-42_i8),(-17_i8),33_i8];
_8.2 = _7.2 - _7.2;
RET = [(-15298_i16),(-3381_i16),4371_i16,(-17204_i16),(-935_i16),7581_i16,(-17131_i16)];
_7.2 = 1801523997_i32 as u8;
_6 = 722953639_i32 as u128;
_8.2 = !_14;
_16.0 = [0_usize,2_usize,13163056264837763899_usize,1635522741528802352_usize];
_2 = (*_4);
Goto(bb11)
}
bb11 = {
_14 = _7.2 ^ _8.2;
_8 = (Field::<f32>(Variant(_3, 0), 0), _7.1, _14);
_7.1 = -_8.1;
(*_4) = _2;
_20 = !3_usize;
_15 = _7.0 == Field::<f32>(Variant(_3, 0), 0);
place!(Field::<[bool; 3]>(Variant(_3, 0), 2)) = [_10,_15,_10];
_14 = _7.2 * _8.2;
(*_4) = [(-75_i8),(-112_i8),(-80_i8)];
_21.2 = !_7.2;
_12 = '\u{e1061}' as i64;
SetDiscriminant(_3, 2);
_1 = [_6,_6,_6,_6,_6,_6];
_18 = core::ptr::addr_of_mut!(_2);
_4 = core::ptr::addr_of_mut!(_2);
_25 = core::ptr::addr_of!(_16);
_21.1 = _7.1 + _7.1;
_21.0 = _14 as f32;
_21.0 = _7.0;
_27 = _7.0;
_23 = [_10,_10,_10,_10,_10];
_9 = [4221996436_u32,3846254495_u32,360908767_u32,3027099440_u32,485018389_u32,713458898_u32,299037747_u32];
_27 = -_7.0;
_13 = (-104_i8) as isize;
Goto(bb12)
}
bb12 = {
(*_18) = [(-12_i8),54_i8,37_i8];
_7.2 = _12 as u8;
_7 = (_21.0, _21.1, _8.2);
RET = [25795_i16,13466_i16,10998_i16,10029_i16,(-29951_i16),(-2526_i16),(-13425_i16)];
_26 = _12;
place!(Field::<[u128; 6]>(Variant(_3, 2), 0)) = [_6,_6,_6,_6,_6,_6];
_1 = Field::<[u128; 6]>(Variant(_3, 2), 0);
_17 = [(-92_i8),(-106_i8),(-111_i8)];
_27 = _21.0 + _21.0;
SetDiscriminant(_3, 2);
_15 = !_10;
place!(Field::<[u128; 6]>(Variant(_3, 2), 0)) = [_6,_6,_6,_6,_6,_6];
Goto(bb13)
}
bb13 = {
_21 = (_27, _8.1, _8.2);
_6 = 100251957611219642156617856196567170077_u128 + 323053789866440416759758903167271003518_u128;
_29.fld4.fld0 = 31751_u16;
_26 = _12;
_29.fld1 = _14;
(*_4) = _17;
_19 = [4954_i16];
_28 = _8.2 as i128;
_29.fld0 = (*_25);
match _29.fld4.fld0 {
0 => bb6,
1 => bb2,
2 => bb9,
3 => bb14,
4 => bb15,
5 => bb16,
31751 => bb18,
_ => bb17
}
}
bb14 = {
(*_18) = [(-12_i8),54_i8,37_i8];
_7.2 = _12 as u8;
_7 = (_21.0, _21.1, _8.2);
RET = [25795_i16,13466_i16,10998_i16,10029_i16,(-29951_i16),(-2526_i16),(-13425_i16)];
_26 = _12;
place!(Field::<[u128; 6]>(Variant(_3, 2), 0)) = [_6,_6,_6,_6,_6,_6];
_1 = Field::<[u128; 6]>(Variant(_3, 2), 0);
_17 = [(-92_i8),(-106_i8),(-111_i8)];
_27 = _21.0 + _21.0;
SetDiscriminant(_3, 2);
_15 = !_10;
place!(Field::<[u128; 6]>(Variant(_3, 2), 0)) = [_6,_6,_6,_6,_6,_6];
Goto(bb13)
}
bb15 = {
_7.0 = _8.0 + _8.0;
_8.1 = -_7.1;
(*_4) = _2;
(*_4) = _2;
_8.2 = !_7.2;
RET = [5641_i16,19744_i16,(-18316_i16),(-21388_i16),27131_i16,(-13540_i16),(-6738_i16)];
RET = [20546_i16,(-2839_i16),13653_i16,6277_i16,1743_i16,(-5706_i16),(-12064_i16)];
SetDiscriminant(_3, 2);
_5 = [true,false,true];
_8.1 = _7.1;
(*_4) = [(-59_i8),70_i8,14_i8];
_7.0 = -_8.0;
(*_4) = [93_i8,(-110_i8),(-110_i8)];
Goto(bb2)
}
bb16 = {
_1 = [_6,_6,_6,_6,_6,_6];
_13 = (-1297829327_i32) as isize;
_9 = [739756013_u32,4007075314_u32,3555597531_u32,3866755936_u32,1393367014_u32,1944922100_u32,3407899294_u32];
_13 = _7.1 as isize;
place!(Field::<f32>(Variant(_3, 0), 0)) = -_8.0;
_5 = [_10,_10,_10];
place!(Field::<f32>(Variant(_3, 0), 0)) = _8.0 + _8.0;
Goto(bb4)
}
bb17 = {
Return()
}
bb18 = {
(*_18) = [108_i8,67_i8,22_i8];
(*_4) = _17;
_20 = !6657205716177673157_usize;
_16.0 = _29.fld0.0;
(*_4) = [119_i8,21_i8,(-60_i8)];
_21.2 = _29.fld1 >> _13;
_7.1 = -_21.1;
_7.2 = _29.fld1;
place!(Field::<[u128; 6]>(Variant(_3, 2), 0)) = [_6,_6,_6,_6,_6,_6];
_8.2 = !_21.2;
_20 = 2098581143194647481_usize;
_21 = _8;
_3 = Adt53::Variant2 { fld0: _1 };
_7.1 = _8.1 - _8.1;
_8.0 = _7.0;
_7.2 = _29.fld1;
_21.0 = _27;
_1 = [_6,_6,_6,_6,_6,_6];
_16.0 = [_20,_20,_20,_20];
_21.2 = _20 as u8;
_21.0 = -_27;
_10 = _15;
_17 = (*_4);
(*_25).0 = _29.fld0.0;
_27 = _20 as f32;
_21 = (_7.0, _7.1, _8.2);
(*_4) = [119_i8,94_i8,79_i8];
_14 = !_7.2;
(*_4) = _17;
(*_25).0 = _29.fld0.0;
_29.fld1 = _7.2;
Goto(bb19)
}
bb19 = {
Call(_31 = dump_var(16_usize, 13_usize, Move(_13), 26_usize, Move(_26), 14_usize, Move(_14), 6_usize, Move(_6)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_31 = dump_var(16_usize, 1_usize, Move(_1), 15_usize, Move(_15), 10_usize, Move(_10), 16_usize, Move(_16)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: [usize; 8],mut _2: [u16; 4],mut _3: u128,mut _4: u128,mut _5: [usize; 8],mut _6: *mut ([u16; 4], u128, u8, f32),mut _7: *mut ([u16; 4], u128, u8, f32),mut _8: [u16; 4],mut _9: ([u16; 4], u128, u8, f32),mut _10: *mut ([u16; 4], u128, u8, f32),mut _11: u128,mut _12: [usize; 8],mut _13: *mut ([u16; 4], u128, u8, f32)) -> *mut ([u16; 4], u128, u8, f32) {
mir! {
type RET = *mut ([u16; 4], u128, u8, f32);
let _14: i64;
let _15: &'static f64;
let _16: [u16; 4];
let _17: ([u16; 4], u128, u8, f32);
let _18: char;
let _19: Adt48;
let _20: ();
let _21: ();
{
(*_7).1 = !_4;
RET = _6;
(*_10).2 = !_9.2;
(*RET).1 = !_9.1;
(*_10) = (_2, _3, _9.2, _9.3);
_10 = core::ptr::addr_of_mut!((*_7));
(*_6).1 = _9.1;
(*_6).1 = 13830628390825137185_u64 as u128;
(*_6) = (_2, _4, _9.2, _9.3);
(*_13).2 = false as u8;
_1 = [7_usize,7188967941771319979_usize,1_usize,17154875363008895147_usize,2_usize,13825028772915695655_usize,0_usize,4265099292793821253_usize];
(*_6).3 = 116_i8 as f32;
_9.0 = [41203_u16,18347_u16,25520_u16,8157_u16];
(*RET).1 = _3 | _11;
(*_7).1 = _3;
(*RET).1 = _3 + _4;
RET = core::ptr::addr_of_mut!((*_7));
_7 = core::ptr::addr_of_mut!((*_10));
_9.1 = _4 << (*RET).1;
(*RET).2 = _9.2 | _9.2;
(*_6).3 = -_9.3;
(*_10).2 = 345567441219038954_u64 as u8;
_9.3 = -(*RET).3;
Goto(bb1)
}
bb1 = {
Call(_20 = dump_var(17_usize, 5_usize, Move(_5), 8_usize, Move(_8), 1_usize, Move(_1), 2_usize, Move(_2)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: *mut i16,mut _2: *mut i16) -> u8 {
mir! {
type RET = u8;
let _3: char;
let _4: *mut ([u16; 4], u128, u8, f32);
let _5: *mut i16;
let _6: i8;
let _7: [u16; 6];
let _8: u64;
let _9: u64;
let _10: ([u16; 4], u128, u8, f32);
let _11: [u16; 4];
let _12: ([usize; 4],);
let _13: char;
let _14: char;
let _15: [i16; 1];
let _16: Adt46;
let _17: *mut ([u16; 4], u128, u8, f32);
let _18: *const [bool; 5];
let _19: f32;
let _20: (f32, f64, u8);
let _21: f64;
let _22: [bool; 8];
let _23: *const [bool; 5];
let _24: usize;
let _25: f32;
let _26: u16;
let _27: char;
let _28: f32;
let _29: Adt53;
let _30: char;
let _31: bool;
let _32: isize;
let _33: f64;
let _34: u128;
let _35: Adt48;
let _36: ();
let _37: ();
{
RET = 238_u8;
RET = !205_u8;
RET = 76_u8;
_1 = _2;
RET = !157_u8;
_3 = '\u{6458c}';
_3 = '\u{5eb28}';
_2 = _1;
_2 = _1;
_3 = '\u{d48a3}';
_2 = _1;
_1 = _2;
RET = 65_u8;
RET = !160_u8;
_3 = '\u{460a1}';
_7 = [12736_u16,54615_u16,7868_u16,6624_u16,39584_u16,41292_u16];
_6 = -64_i8;
_5 = _2;
RET = 17450207865627842736_u64 as u8;
_3 = '\u{ab793}';
_1 = _5;
_8 = !13203968137595426595_u64;
_8 = 2549811581625459793_u64 * 6310481087970049301_u64;
Goto(bb1)
}
bb1 = {
_6 = !(-78_i8);
_1 = _2;
RET = !25_u8;
_3 = '\u{32c2a}';
_3 = '\u{5eecf}';
_6 = 7_i8;
_9 = !_8;
_5 = _2;
_1 = _2;
_5 = _1;
_9 = 108322378024055906894618851726645693572_i128 as u64;
RET = 16554_u16 as u8;
_3 = '\u{10e5af}';
_7 = [2931_u16,45287_u16,17180_u16,37491_u16,62715_u16,15250_u16];
_7 = [43038_u16,831_u16,55649_u16,58768_u16,54809_u16,37210_u16];
_7 = [54373_u16,33172_u16,36774_u16,48312_u16,52106_u16,41835_u16];
_10.3 = 632815336_i32 as f32;
_10.3 = RET as f32;
_10.2 = RET & RET;
_10.1 = !12142509585471116759744115350403178256_u128;
RET = !_10.2;
_2 = _1;
Goto(bb2)
}
bb2 = {
_6 = !(-93_i8);
_5 = _1;
_5 = _1;
Goto(bb3)
}
bb3 = {
_5 = _1;
_11 = [837_u16,45994_u16,62891_u16,35221_u16];
_11 = [47055_u16,36553_u16,30685_u16,7112_u16];
_4 = core::ptr::addr_of_mut!(_10);
(*_4).2 = 1705724729_i32 as u8;
(*_4).0 = _11;
_10.1 = 3167588282649745755_usize as u128;
_10.2 = 40334_u16 as u8;
(*_4).0 = _11;
RET = (*_4).2 >> (*_4).2;
(*_4).2 = RET;
RET = (*_4).2;
_12.0 = [3979707083546216292_usize,1_usize,6_usize,1_usize];
_7 = [25563_u16,7962_u16,25641_u16,48607_u16,45145_u16,23244_u16];
_1 = _5;
_7 = [17264_u16,7072_u16,15193_u16,18761_u16,6334_u16,14399_u16];
(*_4).1 = !298654064530414031363296900301492362990_u128;
_16.fld2.5 = (*_4).0;
_16.fld2.2 = true as u8;
Goto(bb4)
}
bb4 = {
_16.fld0 = _5;
_2 = _1;
_16.fld2.2 = !(*_4).2;
_14 = _3;
(*_4).2 = (-5365534915687396169_i64) as u8;
Goto(bb5)
}
bb5 = {
_16.fld2.2 = RET;
_13 = _3;
_16.fld2.4 = [(*_4).1,(*_4).1,(*_4).1,(*_4).1,(*_4).1,_10.1];
(*_4).0 = [3080_u16,43253_u16,24715_u16,49706_u16];
_16.fld2.4 = [(*_4).1,(*_4).1,(*_4).1,(*_4).1,(*_4).1,(*_4).1];
Goto(bb6)
}
bb6 = {
_9 = (*_4).1 as u64;
_16.fld2.1 = [false,false,true];
(*_4).1 = 174741069109423126378716497942298312742_u128;
(*_4).0 = _11;
(*_4).0 = [52159_u16,18210_u16,54402_u16,21840_u16];
_16.fld2.3 = [false,true,false,true,false,false,true,false];
_15 = [27084_i16];
(*_4).2 = _16.fld2.2 | RET;
_7 = [15405_u16,65474_u16,55795_u16,59349_u16,35833_u16,12781_u16];
_11 = [63292_u16,43137_u16,60860_u16,13407_u16];
_20.1 = (*_4).1 as f64;
_17 = core::ptr::addr_of_mut!((*_4));
_20.0 = (*_4).3 + (*_4).3;
_11 = [24166_u16,11052_u16,49824_u16,22269_u16];
(*_4) = (_16.fld2.5, 250076667887608315044698302766700490944_u128, _16.fld2.2, _20.0);
_10.1 = 21868788884240834837392781407998960758_u128 - 3063891269981738133831215591705700709_u128;
_16.fld3 = !_6;
_20.2 = (*_17).2;
_10.3 = -_20.0;
(*_17).0 = [47433_u16,62064_u16,12266_u16,60131_u16];
(*_17).0 = [29937_u16,51020_u16,34523_u16,28539_u16];
Goto(bb7)
}
bb7 = {
_3 = _13;
_16.fld2.5 = [2482_u16,24396_u16,24785_u16,14038_u16];
_12.0 = [4_usize,7_usize,1_usize,11615774637168735667_usize];
(*_17).2 = !_16.fld2.2;
_10.0 = [33798_u16,11773_u16,54990_u16,34634_u16];
_16.fld2.3 = [true,true,true,false,true,true,false,true];
(*_17).2 = _20.1 as u8;
(*_17) = (_16.fld2.5, 100868668294410581133792673432655693124_u128, RET, _20.0);
_16.fld2.0 = [2217_i16,10603_i16,(-24721_i16),28971_i16,14354_i16,25759_i16,(-14343_i16)];
(*_4).1 = !111477608779992397701126669824883440926_u128;
RET = !_16.fld2.2;
RET = !_20.2;
_10.3 = _20.1 as f32;
_1 = _2;
_16.fld2.1 = [false,false,false];
Goto(bb8)
}
bb8 = {
_24 = 7_usize * 4_usize;
(*_17).2 = _24 as u8;
_16.fld1 = _24 | _24;
_12.0 = [_24,_24,_24,_16.fld1];
_24 = 1004383818_u32 as usize;
(*_4).3 = _20.0 * _20.0;
_2 = _5;
_9 = _24 as u64;
_2 = _1;
(*_17) = (_16.fld2.5, 279548374398751235438548552166377583966_u128, _16.fld2.2, _20.0);
(*_4).3 = _20.0;
_20.0 = -_10.3;
_26 = false as u16;
(*_4).2 = _16.fld2.2 >> (*_4).1;
_8 = _9;
match (*_17).1 {
0 => bb1,
1 => bb9,
2 => bb10,
3 => bb11,
279548374398751235438548552166377583966 => bb13,
_ => bb12
}
}
bb9 = {
_3 = _13;
_16.fld2.5 = [2482_u16,24396_u16,24785_u16,14038_u16];
_12.0 = [4_usize,7_usize,1_usize,11615774637168735667_usize];
(*_17).2 = !_16.fld2.2;
_10.0 = [33798_u16,11773_u16,54990_u16,34634_u16];
_16.fld2.3 = [true,true,true,false,true,true,false,true];
(*_17).2 = _20.1 as u8;
(*_17) = (_16.fld2.5, 100868668294410581133792673432655693124_u128, RET, _20.0);
_16.fld2.0 = [2217_i16,10603_i16,(-24721_i16),28971_i16,14354_i16,25759_i16,(-14343_i16)];
(*_4).1 = !111477608779992397701126669824883440926_u128;
RET = !_16.fld2.2;
RET = !_20.2;
_10.3 = _20.1 as f32;
_1 = _2;
_16.fld2.1 = [false,false,false];
Goto(bb8)
}
bb10 = {
_6 = !(-78_i8);
_1 = _2;
RET = !25_u8;
_3 = '\u{32c2a}';
_3 = '\u{5eecf}';
_6 = 7_i8;
_9 = !_8;
_5 = _2;
_1 = _2;
_5 = _1;
_9 = 108322378024055906894618851726645693572_i128 as u64;
RET = 16554_u16 as u8;
_3 = '\u{10e5af}';
_7 = [2931_u16,45287_u16,17180_u16,37491_u16,62715_u16,15250_u16];
_7 = [43038_u16,831_u16,55649_u16,58768_u16,54809_u16,37210_u16];
_7 = [54373_u16,33172_u16,36774_u16,48312_u16,52106_u16,41835_u16];
_10.3 = 632815336_i32 as f32;
_10.3 = RET as f32;
_10.2 = RET & RET;
_10.1 = !12142509585471116759744115350403178256_u128;
RET = !_10.2;
_2 = _1;
Goto(bb2)
}
bb11 = {
_16.fld2.2 = RET;
_13 = _3;
_16.fld2.4 = [(*_4).1,(*_4).1,(*_4).1,(*_4).1,(*_4).1,_10.1];
(*_4).0 = [3080_u16,43253_u16,24715_u16,49706_u16];
_16.fld2.4 = [(*_4).1,(*_4).1,(*_4).1,(*_4).1,(*_4).1,(*_4).1];
Goto(bb6)
}
bb12 = {
_16.fld0 = _5;
_2 = _1;
_16.fld2.2 = !(*_4).2;
_14 = _3;
(*_4).2 = (-5365534915687396169_i64) as u8;
Goto(bb5)
}
bb13 = {
_16.fld2.5 = [_26,_26,_26,_26];
(*_17).0 = [_26,_26,_26,_26];
(*_4).2 = _20.2 >> (*_17).1;
(*_17).1 = 27591592397323197760122036020538499917_u128;
_16.fld2.2 = !_10.2;
(*_17).3 = _20.0;
(*_4).2 = _16.fld2.2 ^ _16.fld2.2;
_28 = (*_4).3;
_16.fld2.4 = [(*_17).1,(*_17).1,(*_4).1,_10.1,(*_4).1,(*_17).1];
(*_17) = (_11, 312767492797361154453833891698889189414_u128, _16.fld2.2, _28);
(*_17).1 = !258169217297732535903725128202410440734_u128;
_10.0 = [_26,_26,_26,_26];
_27 = _3;
_4 = _17;
_2 = _1;
_16.fld2.5 = (*_17).0;
(*_4).0 = [_26,_26,_26,_26];
(*_17).3 = _28;
(*_4).1 = 111774104658146729964442424813530347190_u128 | 88158977597794863160386390073137515364_u128;
_31 = _16.fld2.2 >= (*_17).2;
_4 = core::ptr::addr_of_mut!(_10);
_19 = -_20.0;
_20.0 = -(*_4).3;
Goto(bb14)
}
bb14 = {
_16.fld2.1 = [_31,_31,_31];
(*_4).2 = !_16.fld2.2;
_14 = _3;
(*_4).1 = 289173381050221086023640508814307412890_u128 | 161549213038991550468586342929386754769_u128;
_16.fld2.1 = [_31,_31,_31];
(*_4).1 = !248820751887288945094416187501089044687_u128;
_22 = [_31,_31,_31,_31,_31,_31,_31,_31];
_35.fld1.4 = _16.fld2.4;
_35.fld1.2 = 400931837_i32 as u8;
_4 = _17;
(*_17).1 = 315644918535101518413482347342173939100_u128 << _10.2;
(*_4) = (_11, 204762548192719204023676179238488208932_u128, RET, _28);
_35.fld1.4 = [(*_4).1,(*_4).1,(*_4).1,(*_17).1,_10.1,(*_17).1];
_16.fld2.3 = [_31,_31,_31,_31,_31,_31,_31,_31];
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(18_usize, 3_usize, Move(_3), 11_usize, Move(_11), 15_usize, Move(_15), 26_usize, Move(_26)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(18_usize, 22_usize, Move(_22), 6_usize, Move(_6), 7_usize, Move(_7), 37_usize, _37), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: isize,mut _2: f32,mut _3: isize) -> i8 {
mir! {
type RET = i8;
let _4: Adt57;
let _5: i8;
let _6: Adt57;
let _7: Adt54;
let _8: isize;
let _9: [i16; 7];
let _10: char;
let _11: bool;
let _12: [u32; 7];
let _13: [bool; 8];
let _14: ([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4]);
let _15: f32;
let _16: [bool; 5];
let _17: Adt51;
let _18: [usize; 4];
let _19: bool;
let _20: i16;
let _21: ([u16; 4], u128, u8, f32);
let _22: [i16; 1];
let _23: bool;
let _24: ();
let _25: ();
{
RET = -(-107_i8);
RET = '\u{2dff}' as i8;
_3 = _1;
_1 = -_3;
_3 = !_1;
_2 = RET as f32;
RET = (-56_i8) - 104_i8;
_2 = 125_u8 as f32;
_2 = 959585362_i32 as f32;
_1 = 1878635255616469170_usize as isize;
_3 = !_1;
_2 = RET as f32;
_3 = 724863501_i32 as isize;
_1 = 2959004384_u32 as isize;
_2 = 1994750542_i32 as f32;
_5 = -RET;
RET = _5 + _5;
_5 = _1 as i8;
_5 = RET;
_7.fld0.0 = [6_usize,3_usize,15683091383097096161_usize,4694898467328142146_usize];
RET = _5;
_3 = !_1;
_7.fld4.fld0 = !17168_u16;
_7.fld2 = _3;
_5 = 7285726187951140558_u64 as i8;
Goto(bb1)
}
bb1 = {
_7.fld2 = _1 >> _1;
RET = _5;
_7.fld4.fld0 = 49970_u16;
_7.fld4 = Adt45 { fld0: 339_u16 };
_7.fld4.fld0 = 91807728732595591_i64 as u16;
_7.fld4.fld0 = 5447403098275733459_u64 as u16;
_7.fld2 = _3 * _3;
_1 = _7.fld2;
_7.fld5 = [167942285147210928375490788150683162831_u128,284317223403493832179034594738937522015_u128,74888530015364502523378639008093968249_u128,175041320347982556845296951492182929299_u128,129271172570231500611278125490209798856_u128,194826863577144431771759247849229668210_u128];
RET = _5 ^ _5;
_7.fld6 = (-3509617100143413583_i64) * 1796537418328341272_i64;
_8 = _3;
RET = _5 ^ _5;
_8 = !_1;
_1 = -_7.fld2;
_7.fld0.0 = [13942501310703602026_usize,4157208844556872244_usize,2215662024628234241_usize,1_usize];
_7.fld4.fld0 = '\u{bf5c5}' as u16;
_7.fld1 = !16_u8;
_2 = 5_usize as f32;
Goto(bb2)
}
bb2 = {
RET = _7.fld4.fld0 as i8;
_9 = [22471_i16,(-4334_i16),13037_i16,(-16779_i16),(-2624_i16),(-12268_i16),3663_i16];
Call(_1 = core::intrinsics::transmute(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9 = [23435_i16,(-28262_i16),(-21470_i16),28150_i16,(-14862_i16),(-28355_i16),31827_i16];
_8 = !_3;
_10 = '\u{f0717}';
Goto(bb4)
}
bb4 = {
_8 = _1;
_7.fld1 = 135_u8 & 38_u8;
_5 = _2 as i8;
_7.fld1 = 128_u8 << _7.fld6;
_2 = _7.fld4.fld0 as f32;
_7.fld4.fld0 = 59667_u16;
_7.fld6 = !(-8263632955277462181_i64);
_7.fld4.fld0 = 3473_u16;
RET = _5;
_7.fld1 = !36_u8;
_5 = RET;
_7.fld4.fld0 = 152196549068458201_usize as u16;
_10 = '\u{1012ad}';
_7.fld5 = [177833332904098019945880256707799076719_u128,336827865886931756232726393566845225202_u128,45312512474726943994530057784143612748_u128,190883493287240328164277928190250377277_u128,20105234492876889918527664785948722063_u128,183300342058784337620811503120311255277_u128];
_7.fld5 = [335353226627025018782764625967615412686_u128,34497928361240723725201295253403263608_u128,1441167485358467005734810178447740353_u128,148305747915902966668947770610327210759_u128,5557973915610007529845574383671058794_u128,154741562418571144427682978881236601187_u128];
_7.fld0.0 = [4515086749623548464_usize,11835402135525959140_usize,17734789702859113391_usize,5_usize];
Goto(bb5)
}
bb5 = {
_12 = [1526642091_u32,4138218818_u32,2937499268_u32,3612345621_u32,3344781884_u32,3417436823_u32,576220704_u32];
_2 = _5 as f32;
_7.fld5 = [188511850063956958254272319194382355845_u128,77412287271220633266783299468682532328_u128,28881254168614827341882802126243272910_u128,197099429034261776228693718380078629530_u128,36860391500282085640129951418592872397_u128,224783081006820255717085349380427343390_u128];
Goto(bb6)
}
bb6 = {
_7.fld4 = Adt45 { fld0: 44508_u16 };
RET = _5 << _7.fld1;
RET = _5;
_2 = 9398804354192579183_u64 as f32;
_10 = '\u{9c586}';
_12 = [2797515131_u32,3825370886_u32,3903708212_u32,2460578213_u32,1678955052_u32,2395122718_u32,3512438842_u32];
_7.fld0.0 = [3_usize,11827503766083465689_usize,13092572134893399037_usize,6_usize];
_9 = [26481_i16,25513_i16,17742_i16,19095_i16,(-10307_i16),(-30310_i16),(-17792_i16)];
_12 = [3778004524_u32,4030235927_u32,3352624948_u32,4080331531_u32,2643141568_u32,2982798923_u32,3327612540_u32];
_7.fld0.0 = [9069610855013849721_usize,8158725659538543267_usize,5462155836441912984_usize,7_usize];
_11 = false;
RET = -_5;
RET = _5;
_7.fld6 = 1843044773334169071_i64;
_7.fld5 = [242146457206571562389112291792060614428_u128,7311533010484558108753439819885291532_u128,109494128522168644188468026357339085759_u128,66897512657716261859959546957256360158_u128,310595454665641660290131863895839399_u128,183687911974050605051759752135034198721_u128];
_7.fld0.0 = [6_usize,5248629301419619491_usize,4_usize,8298444360630038997_usize];
Goto(bb7)
}
bb7 = {
_7.fld4.fld0 = !15141_u16;
_7.fld2 = _1;
_7.fld2 = (-2038930183_i32) as isize;
_5 = _11 as i8;
RET = _10 as i8;
_7.fld4.fld0 = _8 as u16;
_7.fld2 = _1;
_7.fld6 = 3398129623284045943_i64 & (-7579612668673140148_i64);
_3 = -_8;
Goto(bb8)
}
bb8 = {
_2 = (-261654622_i32) as f32;
_1 = _7.fld2 >> _7.fld6;
_3 = (-65596108503363346741723419919284151639_i128) as isize;
_11 = true;
_9 = [29331_i16,12519_i16,(-25790_i16),(-18085_i16),(-10239_i16),9725_i16,(-29032_i16)];
_7.fld5 = [319900119905234280763694068060633618481_u128,121039344033012562150010203445707652240_u128,274762128656120031706963194681478840559_u128,50389091425440305073249710206448276619_u128,280870708549882469641038833243309394232_u128,320429217046103176042572444998718679278_u128];
_10 = '\u{e74d1}';
Goto(bb9)
}
bb9 = {
_7.fld6 = 5803846633117439752_i64;
_7.fld0.0 = [13089604728878351133_usize,5537368372908257520_usize,5_usize,10327904609738532997_usize];
_7.fld1 = !129_u8;
_7.fld1 = !66_u8;
_14.0 = _9;
_12 = [3075330443_u32,624036692_u32,421911731_u32,2043952781_u32,1799781595_u32,4011735962_u32,635147385_u32];
_12 = [4248678999_u32,2312612209_u32,3075459424_u32,2482708871_u32,1472062390_u32,445224800_u32,1628255885_u32];
_14.5 = [_7.fld4.fld0,_7.fld4.fld0,_7.fld4.fld0,_7.fld4.fld0];
_14.3 = [_11,_11,_11,_11,_11,_11,_11,_11];
_11 = false;
_14.2 = !_7.fld1;
_7.fld4.fld0 = 4967_u16 ^ 8251_u16;
_7.fld6 = 13274767963053544638_usize as i64;
_14.1 = [_11,_11,_11];
_15 = _2 + _2;
_13 = [_11,_11,_11,_11,_11,_11,_11,_11];
_14.1 = [_11,_11,_11];
_15 = -_2;
_2 = _15 - _15;
_7.fld4 = Adt45 { fld0: 6038_u16 };
Goto(bb10)
}
bb10 = {
_18 = [7311813167005742789_usize,14926115460825962086_usize,6776058889606809173_usize,4659080842996751359_usize];
_17.fld3.1 = [2_usize,18047540098269479611_usize,11862261006935241948_usize,4_usize,1_usize,0_usize,17803282342983431533_usize,17832627416688790435_usize];
_10 = '\u{e197a}';
_17.fld3.1 = [1_usize,10257627775751393076_usize,2615716890667643854_usize,14763590767845993153_usize,0_usize,4_usize,8786079148550986820_usize,7836858665367289172_usize];
_7.fld6 = 196211425400434607686816679044396879397_u128 as i64;
_3 = -_1;
_14.1 = [_11,_11,_11];
_16 = [_11,_11,_11,_11,_11];
_17.fld0.1 = 310116084434648389840674545976324489595_u128 * 82501461664384728299624487986037087547_u128;
_14.0 = [(-25850_i16),26935_i16,(-5494_i16),23861_i16,17904_i16,23601_i16,15297_i16];
_13 = [_11,_11,_11,_11,_11,_11,_11,_11];
_7.fld4.fld0 = 4594_u16 & 27101_u16;
Goto(bb11)
}
bb11 = {
_17.fld1 = [3432628943_u32,192816164_u32,4051635973_u32,4208221341_u32,4166017637_u32,2882722877_u32,2132588784_u32];
_15 = _2 + _2;
_15 = -_2;
_17.fld0.0 = _14.5;
_5 = 5412398622543798962_usize as i8;
_10 = '\u{e962f}';
_14.5 = [_7.fld4.fld0,_7.fld4.fld0,_7.fld4.fld0,_7.fld4.fld0];
_7.fld0.0 = [12137586375133464362_usize,12087830516222180927_usize,10897775673022817192_usize,7630279842228814882_usize];
_20 = (-9714_i16) ^ (-32196_i16);
_5 = RET >> _1;
_12 = [2827177254_u32,4036332865_u32,4164277985_u32,2914624699_u32,3801486900_u32,2769329918_u32,3361446856_u32];
_7.fld6 = (-1125082127294216134_i64) & (-1535214839110398488_i64);
_8 = _3 << RET;
_17.fld3.0 = [_20,_20,_20,_20,_20,_20,_20];
_17.fld0 = (_14.5, 253206047705418109178744246143227607138_u128, _7.fld1, _15);
_21.0 = [_7.fld4.fld0,_7.fld4.fld0,_7.fld4.fld0,_7.fld4.fld0];
_14.4 = _7.fld5;
_14.4 = _7.fld5;
_7.fld0.0 = [2_usize,2913304924216971034_usize,6231740534603910153_usize,5_usize];
_14.0 = _17.fld3.0;
_14.4 = [_17.fld0.1,_17.fld0.1,_17.fld0.1,_17.fld0.1,_17.fld0.1,_17.fld0.1];
_19 = _11;
_11 = _19 ^ _19;
_14.0 = [_20,_20,_20,_20,_20,_20,_20];
match _17.fld0.1 {
0 => bb9,
1 => bb12,
2 => bb13,
253206047705418109178744246143227607138 => bb15,
_ => bb14
}
}
bb12 = {
RET = _7.fld4.fld0 as i8;
_9 = [22471_i16,(-4334_i16),13037_i16,(-16779_i16),(-2624_i16),(-12268_i16),3663_i16];
Call(_1 = core::intrinsics::transmute(_3), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_7.fld6 = 5803846633117439752_i64;
_7.fld0.0 = [13089604728878351133_usize,5537368372908257520_usize,5_usize,10327904609738532997_usize];
_7.fld1 = !129_u8;
_7.fld1 = !66_u8;
_14.0 = _9;
_12 = [3075330443_u32,624036692_u32,421911731_u32,2043952781_u32,1799781595_u32,4011735962_u32,635147385_u32];
_12 = [4248678999_u32,2312612209_u32,3075459424_u32,2482708871_u32,1472062390_u32,445224800_u32,1628255885_u32];
_14.5 = [_7.fld4.fld0,_7.fld4.fld0,_7.fld4.fld0,_7.fld4.fld0];
_14.3 = [_11,_11,_11,_11,_11,_11,_11,_11];
_11 = false;
_14.2 = !_7.fld1;
_7.fld4.fld0 = 4967_u16 ^ 8251_u16;
_7.fld6 = 13274767963053544638_usize as i64;
_14.1 = [_11,_11,_11];
_15 = _2 + _2;
_13 = [_11,_11,_11,_11,_11,_11,_11,_11];
_14.1 = [_11,_11,_11];
_15 = -_2;
_2 = _15 - _15;
_7.fld4 = Adt45 { fld0: 6038_u16 };
Goto(bb10)
}
bb14 = {
_9 = [23435_i16,(-28262_i16),(-21470_i16),28150_i16,(-14862_i16),(-28355_i16),31827_i16];
_8 = !_3;
_10 = '\u{f0717}';
Goto(bb4)
}
bb15 = {
_17.fld0.3 = _15 + _2;
Goto(bb16)
}
bb16 = {
Call(_24 = dump_var(19_usize, 14_usize, Move(_14), 16_usize, Move(_16), 8_usize, Move(_8), 18_usize, Move(_18)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_24 = dump_var(19_usize, 19_usize, Move(_19), 11_usize, Move(_11), 1_usize, Move(_1), 25_usize, _25), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{4ea24}'), std::hint::black_box((-2_isize)), std::hint::black_box(42_i8), std::hint::black_box((-15156_i16)), std::hint::black_box((-392533584_i32)), std::hint::black_box((-8771154506405072928_i64)), std::hint::black_box(47000880671648424534088705221381124618_i128), std::hint::black_box(7_usize), std::hint::black_box(105_u8), std::hint::black_box(51074_u16), std::hint::black_box(213304949_u32), std::hint::black_box(4119932013102029768_u64), std::hint::black_box(320509151980464371870539912725770691463_u128));
                
            }
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: u16,
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: *mut i16,
fld1: usize,
fld2: ([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4]),
fld3: i8,
}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: bool,
fld1: ([bool; 3], [i8; 3], i128, [i8; 3], i16),
fld2: isize,
fld3: Adt45,
fld4: *const usize,
fld5: u16,
fld6: u64,

},
Variant1{
fld0: u64,
fld1: i128,
fld2: *const ([usize; 4],),
fld3: [bool; 8],
fld4: f64,

},
Variant2{
fld0: [u128; 6],
fld1: f64,
fld2: u16,
fld3: u128,
fld4: [usize; 8],
fld5: [i16; 7],

},
Variant3{
fld0: i32,
fld1: *mut [i8; 3],
fld2: *mut i16,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: *mut ([u16; 4], u128, u8, f32),
fld1: ([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4]),
fld2: [usize; 8],
fld3: f32,
fld4: [i16; 7],
fld5: [bool; 5],
fld6: [u16; 6],
}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: u8,

},
Variant1{
fld0: ([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4]),
fld1: *mut [i8; 3],
fld2: [bool; 3],

},
Variant2{
fld0: i128,
fld1: [usize; 8],

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: *mut [i8; 3],
fld1: u32,
fld2: [bool; 5],
fld3: [bool; 8],
fld4: ([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4]),

},
Variant1{
fld0: *const ([usize; 4],),
fld1: *mut (f32, f64, u8),
fld2: usize,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: ([u16; 4], u128, u8, f32),
fld1: [u32; 7],
fld2: i16,
fld3: ([i16; 7], [usize; 8], *mut [i8; 3]),
}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: Adt49,
fld1: ([usize; 4],),
fld2: [bool; 3],
fld3: Adt47,
fld4: [i64; 3],
fld5: [i8; 3],

},
Variant1{
fld0: ([bool; 3], [i8; 3], i128, [i8; 3], i16),
fld1: [usize; 8],

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: f32,
fld1: [bool; 5],
fld2: [bool; 3],

},
Variant1{
fld0: Adt51,

},
Variant2{
fld0: [u128; 6],

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt54{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt54 {
fld0: ([usize; 4],),
fld1: u8,
fld2: isize,
fld3: *mut ([u16; 4], u128, u8, f32),
fld4: Adt45,
fld5: [u128; 6],
fld6: i64,
}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: f64,
fld1: [bool; 8],
fld2: *mut (f32, f64, u8),
fld3: ([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4]),

},
Variant1{
fld0: *mut [i8; 3],

},
Variant2{
fld0: ([usize; 4],),
fld1: [i8; 3],
fld2: [usize; 4],
fld3: Adt49,
fld4: u16,
fld5: [u32; 7],
fld6: [bool; 8],
fld7: i128,

},
Variant3{
fld0: [u128; 6],
fld1: char,
fld2: ([i16; 7], [usize; 8], *mut [i8; 3]),
fld3: *mut i16,
fld4: Adt45,
fld5: u8,
fld6: Adt48,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt56{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt56 {
fld0: ([usize; 4],),
fld1: *mut i16,
fld2: [usize; 4],
fld3: Adt51,
fld4: i16,
fld5: Adt46,
}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: *mut i16,
fld1: Adt53,
fld2: Adt51,
fld3: i32,
fld4: ([usize; 4],),

},
Variant1{
fld0: [u16; 6],
fld1: ([bool; 3], [i8; 3], i128, [i8; 3], i16),

},
Variant2{
fld0: Adt51,
fld1: ([u16; 4], u128, u8, f32),
fld2: *const ([usize; 4],),
fld3: Adt47,
fld4: Adt46,
fld5: Adt49,

},
Variant3{
fld0: u128,
fld1: u8,
fld2: *mut bool,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: [i64; 3],

},
Variant1{
fld0: *mut ([u16; 4], u128, u8, f32),
fld1: Adt52,
fld2: Adt53,
fld3: ([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4]),
fld4: i16,
fld5: [i16; 7],

},
Variant2{
fld0: Adt56,
fld1: [i16; 1],
fld2: Adt48,
fld3: *mut (f32, f64, u8),
fld4: [u32; 7],
fld5: [u16; 4],

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf("Adt59::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: *const usize,
fld1: usize,
fld2: Adt51,
fld3: u128,
fld4: *const ([usize; 4],),

},
Variant1{
fld0: ([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4]),
fld1: Adt54,
fld2: ([usize; 4],),
fld3: *const ([usize; 4],),

},
Variant2{
fld0: u16,
fld1: ([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4]),
fld2: *mut bool,
fld3: *mut (f32, f64, u8),
fld4: i16,
fld5: *mut ([u16; 4], u128, u8, f32),
fld6: Adt45,
fld7: Adt46,

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf("Adt60::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: Adt49,
fld1: Adt48,
fld2: Adt53,
fld3: Adt55,
fld4: Adt51,
fld5: i64,

},
Variant1{
fld0: [i16; 7],
fld1: u8,
fld2: [usize; 8],

}}
impl PrintFDebug for Adt61{
	unsafe fn printf_debug(&self){unsafe{printf("Adt61::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt61 {
Variant0{
fld0: (f32, f64, u8),

},
Variant1{
fld0: Adt57,
fld1: [usize; 8],
fld2: [bool; 3],
fld3: ([i16; 7], [bool; 3], u8, [bool; 8], [u128; 6], [u16; 4]),
fld4: Adt56,
fld5: [u16; 4],

},
Variant2{
fld0: bool,
fld1: u128,
fld2: [i16; 7],
fld3: Adt54,
fld4: *mut ([u16; 4], u128, u8, f32),
fld5: usize,
fld6: *mut (f32, f64, u8),
fld7: i128,

},
Variant3{
fld0: *const ([usize; 4],),
fld1: *mut (f32, f64, u8),
fld2: u128,
fld3: Adt46,
fld4: i128,

}}

