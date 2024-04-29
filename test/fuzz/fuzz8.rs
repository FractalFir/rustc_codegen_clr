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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u64) -> *const i128 {
mir! {
type RET = *const i128;
let _13: *const *mut u64;
let _14: (u128, u64);
let _15: i128;
let _16: (u8, [i64; 6]);
let _17: (u128, u64);
let _18: f64;
let _19: (u8, [i64; 6]);
let _20: [u16; 3];
let _21: *const *mut u64;
let _22: char;
let _23: i32;
let _24: Adt57;
let _25: Adt56;
let _26: char;
let _27: [u16; 3];
let _28: Adt54;
let _29: ();
let _30: ();
{
RET = core::ptr::addr_of!(_8);
_6 = 651561384_i32 ^ (-1556304407_i32);
_2 = '\u{558cc}';
_5 = 15103573157339144032_usize as i16;
_12 = 3277059390815202498_u64;
_6 = (-1881791956_i32);
_1 = _2 > _2;
Goto(bb1)
}
bb1 = {
(*RET) = 65248530607241753140992781770647876627_u128 as i128;
_3 = 9223372036854775807_isize;
(*RET) = _2 as i128;
_3 = 126_isize & 9223372036854775807_isize;
_12 = (-3273427502300787528_i64) as u64;
_9 = 187744026186940653747955440366657120136_u128 as usize;
RET = core::ptr::addr_of!((*RET));
_5 = 50_u8 as i16;
_4 = (-73_i8);
_14.1 = _12;
Goto(bb2)
}
bb2 = {
_4 = 313350026650895419887217248836198245988_u128 as i8;
_12 = _14.1;
_16.0 = !121_u8;
_8 = !(-23182393924793011227226280361762063056_i128);
_7 = (-8490064646761011518_i64) ^ 8943286189608285688_i64;
RET = core::ptr::addr_of!(_15);
(*RET) = _8 * _8;
_5 = 149148558909360264334647065865749242654_u128 as i16;
_14.0 = 105607552557864508513332321539447340720_u128 * 238445755714024059456217330468992777821_u128;
match _6 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463463374607429886419500 => bb9,
_ => bb8
}
}
bb3 = {
(*RET) = 65248530607241753140992781770647876627_u128 as i128;
_3 = 9223372036854775807_isize;
(*RET) = _2 as i128;
_3 = 126_isize & 9223372036854775807_isize;
_12 = (-3273427502300787528_i64) as u64;
_9 = 187744026186940653747955440366657120136_u128 as usize;
RET = core::ptr::addr_of!((*RET));
_5 = 50_u8 as i16;
_4 = (-73_i8);
_14.1 = _12;
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
_6 = 1194074553_i32 ^ (-687096858_i32);
_17.0 = _14.0;
_11 = 58887_u16;
_3 = !9223372036854775807_isize;
Call((*RET) = fn1(_11, _16.0, RET, _14, RET, _14.1, RET, _2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_18 = (*RET) as f64;
_17.1 = _12 + _12;
(*RET) = _8;
_3 = (-9223372036854775808_isize);
_6 = 1348472971_i32;
_14 = (_17.0, _17.1);
_15 = _4 as i128;
_17.0 = _14.0;
_23 = _6;
(*RET) = _17.1 as i128;
_14 = _17;
RET = core::ptr::addr_of!(_15);
_11 = 2005213366_u32 as u16;
(*RET) = _8;
(*RET) = _8 ^ _8;
_14 = (_17.0, _17.1);
_4 = (*RET) as i8;
_19.0 = 3398545599_u32 as u8;
_1 = _14.0 < _14.0;
match _23 {
0 => bb5,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
1348472971 => bb18,
_ => bb17
}
}
bb11 = {
_6 = 1194074553_i32 ^ (-687096858_i32);
_17.0 = _14.0;
_11 = 58887_u16;
_3 = !9223372036854775807_isize;
Call((*RET) = fn1(_11, _16.0, RET, _14, RET, _14.1, RET, _2), ReturnTo(bb10), UnwindUnreachable())
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
(*RET) = 65248530607241753140992781770647876627_u128 as i128;
_3 = 9223372036854775807_isize;
(*RET) = _2 as i128;
_3 = 126_isize & 9223372036854775807_isize;
_12 = (-3273427502300787528_i64) as u64;
_9 = 187744026186940653747955440366657120136_u128 as usize;
RET = core::ptr::addr_of!((*RET));
_5 = 50_u8 as i16;
_4 = (-73_i8);
_14.1 = _12;
Goto(bb2)
}
bb17 = {
(*RET) = 65248530607241753140992781770647876627_u128 as i128;
_3 = 9223372036854775807_isize;
(*RET) = _2 as i128;
_3 = 126_isize & 9223372036854775807_isize;
_12 = (-3273427502300787528_i64) as u64;
_9 = 187744026186940653747955440366657120136_u128 as usize;
RET = core::ptr::addr_of!((*RET));
_5 = 50_u8 as i16;
_4 = (-73_i8);
_14.1 = _12;
Goto(bb2)
}
bb18 = {
_10 = _19.0 + _19.0;
_16.1 = [_7,_7,_7,_7,_7,_7];
_6 = _23;
(*RET) = -_8;
_7 = (-3869676881324257284_i64) - 2354278311754314064_i64;
_5 = 24360_i16 * 6026_i16;
_23 = _6;
_20 = [_11,_11,_11];
_1 = false;
_14 = (_17.0, _12);
_11 = !32669_u16;
Goto(bb19)
}
bb19 = {
Call(_29 = dump_var(0_usize, 4_usize, Move(_4), 10_usize, Move(_10), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_29 = dump_var(0_usize, 5_usize, Move(_5), 9_usize, Move(_9), 23_usize, Move(_23), 7_usize, Move(_7)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_29 = dump_var(0_usize, 12_usize, Move(_12), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: u16,mut _2: u8,mut _3: *const i128,mut _4: (u128, u64),mut _5: *const i128,mut _6: u64,mut _7: *const i128,mut _8: char) -> i128 {
mir! {
type RET = i128;
let _9: (u64,);
let _10: f64;
let _11: (u128, u64);
let _12: bool;
let _13: [u16; 3];
let _14: [i64; 1];
let _15: usize;
let _16: [i64; 6];
let _17: f32;
let _18: bool;
let _19: Adt50;
let _20: Adt53;
let _21: *const i128;
let _22: u32;
let _23: (u8, [i64; 6]);
let _24: isize;
let _25: isize;
let _26: char;
let _27: [u16; 3];
let _28: f64;
let _29: (bool, f64, u32);
let _30: i64;
let _31: [i64; 6];
let _32: (bool, f64, u32);
let _33: f64;
let _34: Adt47;
let _35: Adt52;
let _36: f64;
let _37: isize;
let _38: Adt51;
let _39: Adt51;
let _40: (f32, *const u128, *mut u64);
let _41: [i128; 7];
let _42: usize;
let _43: [u16; 3];
let _44: ();
let _45: ();
{
RET = !137611762673005595085941618721995221111_i128;
Call(_8 = fn2(_7, _7, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = (_4.1,);
_6 = RET as u64;
_4.0 = 117711470633842133150059345738367071910_u128;
_4.0 = 39408484138047680494063261113803997884_u128;
_4.1 = _9.0;
_6 = _9.0 << _9.0;
_7 = core::ptr::addr_of!(RET);
_3 = _7;
_11.0 = !_4.0;
_11.1 = !_9.0;
_11 = (_4.0, _6);
_11.1 = _6;
_10 = _2 as f64;
(*_3) = (-91347048622060416595994252769083913317_i128);
(*_3) = (-1353467804_i32) as i128;
_7 = _5;
_3 = core::ptr::addr_of!(RET);
_11.0 = _2 as u128;
_9.0 = (-77_i8) as u64;
_4.0 = (-778121309_i32) as u128;
(*_3) = -159175021916209127160189951258507782174_i128;
(*_3) = (-168682232104569517546851105081472901144_i128) + (-99829490953616822986001904170284480617_i128);
_9.0 = _4.1 & _6;
_11.1 = _9.0;
Call(_4.1 = core::intrinsics::transmute(_9.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = core::ptr::addr_of!(RET);
_7 = _3;
_4.0 = !_11.0;
_3 = _5;
_4 = _11;
_1 = 33612_u16 * 51958_u16;
_1 = 22276_u16;
RET = (-52057760749249421872813702483696355804_i128) >> _11.1;
_3 = _7;
Call(_12 = fn14(_5, _5, _5, _7, (*_7), _7, (*_3), _7, (*_7), _9.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3 = core::ptr::addr_of!((*_7));
_9.0 = _11.1;
_4.0 = (*_7) as u128;
_4.1 = _9.0 & _11.1;
_4.0 = !_11.0;
(*_3) = !(-78947535787695422586571169228014382579_i128);
Goto(bb4)
}
bb4 = {
(*_3) = (-87851443938679527090785716057718548614_i128);
_14 = [5872197164664141523_i64];
_15 = 1955484055128223878_usize ^ 3_usize;
_6 = _4.1;
_4.0 = 5121628365198056604_i64 as u128;
_4.1 = _8 as u64;
_11 = (_4.0, _9.0);
_13 = [_1,_1,_1];
_4.1 = _6 & _9.0;
_9 = (_6,);
_11.0 = _4.0 | _4.0;
_11 = _4;
(*_3) = (-88654507443335458539899118344688336610_i128) + 135524748708837581582790413779413578772_i128;
_11 = (_4.0, _4.1);
(*_7) = !111289441093948451058804881705650453564_i128;
RET = -(-37776102757562830562699847600421681020_i128);
_4.1 = _9.0;
(*_7) = 78430391013066013714145872664192005622_i128;
_13 = [_1,_1,_1];
_11.0 = _4.0;
_19.fld2.fld0 = [_1,_1,_1];
_7 = _5;
Call(_19.fld4.0 = core::intrinsics::transmute(_12), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_19.fld3 = _4.1 as f64;
_17 = _19.fld3 as f32;
_15 = 4976987645328224813_usize;
_19.fld2.fld1 = core::ptr::addr_of!((*_3));
_20.fld2 = _11.1;
_18 = _12;
_9.0 = _11.1 * _6;
_19.fld2.fld0 = [_1,_1,_1];
_20.fld3.2 = !790639366_u32;
_12 = !_18;
_20.fld3.0 = !_12;
_20.fld1 = _8;
_5 = _7;
_10 = -_19.fld3;
_19.fld4.2 = _20.fld1;
_19.fld3 = _4.1 as f64;
RET = !159449893163051277584691972170738840936_i128;
_19.fld4.1 = _19.fld4.0;
_7 = _19.fld2.fld1;
_18 = !_20.fld3.0;
(*_7) = 99_isize as i128;
_14 = [5837325415371196627_i64];
_17 = _15 as f32;
_20.fld3.2 = _19.fld4.1 as u32;
_14 = [2183181199258019598_i64];
_18 = _20.fld3.0 | _12;
_19.fld0 = [_19.fld4.1];
_20.fld1 = _8;
Goto(bb6)
}
bb6 = {
_19.fld4.0 = _19.fld4.1 ^ _19.fld4.1;
RET = (-55790464593725331162189333241336678160_i128) + (-139860470742923903703628876957957470083_i128);
_19.fld0 = [_19.fld4.0];
_6 = _4.1 + _20.fld2;
_19.fld4.0 = !_19.fld4.1;
_5 = core::ptr::addr_of!((*_7));
_19.fld4.1 = _19.fld4.0 & _19.fld4.0;
_16 = [8475148408019313166_i64,(-7574397687322459433_i64),7914996334424672950_i64,(-2158722762734088874_i64),(-3243489974104076799_i64),(-7376090344204011273_i64)];
_11.1 = !_9.0;
_4.1 = _11.1 + _20.fld2;
_14 = [(-8980595054722336321_i64)];
Call(_19.fld3 = core::intrinsics::fmaf64(_10, _10, _10), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_24 = _20.fld3.2 as isize;
(*_3) = (-132938993484717989431985964754849035876_i128) + 153172617006138249843079716491535553788_i128;
_18 = _20.fld2 != _4.1;
_2 = _24 as u8;
_20.fld3.1 = _10;
_24 = (-9223372036854775808_isize) << _4.1;
_19.fld4 = ((-45_i8), (-17_i8), _20.fld1);
_4.1 = _17 as u64;
_24 = -(-9223372036854775808_isize);
_27 = [_1,_1,_1];
_26 = _19.fld4.2;
_19.fld2.fld0 = _27;
_4.0 = !_11.0;
_2 = 168_u8 * 8_u8;
_23.0 = _2 * _2;
_26 = _19.fld4.2;
Goto(bb8)
}
bb8 = {
_15 = 18381810107215490347_usize;
_9 = (_11.1,);
_22 = _20.fld3.2 << _20.fld2;
_5 = core::ptr::addr_of!(RET);
_15 = 4_usize ^ 12296019713933148221_usize;
_5 = _7;
_20.fld2 = !_11.1;
_23 = (_2, _16);
_4 = (_11.0, _11.1);
_19.fld2.fld0 = [_1,_1,_1];
(*_3) = (-122922387167782420284932244947166225508_i128);
_17 = _1 as f32;
_29.0 = !_12;
_2 = _20.fld3.2 as u8;
_7 = _19.fld2.fld1;
_17 = _19.fld3 as f32;
_15 = 0_usize;
_32.0 = _29.0;
(*_5) = 104162068153909241916586948195156311013_i128 + 64071690380841026413620405747790260390_i128;
_26 = _19.fld4.2;
_20.fld3.1 = _10 - _10;
_32.1 = _10;
_29 = _20.fld3;
_19.fld0 = [_19.fld4.1];
_31[_15] = _16[_15] & _16[_15];
_29.2 = (*_5) as u32;
_11 = (_4.0, _6);
_19.fld2.fld0[_15] = !_1;
match _23.1[_15] {
8475148408019313166 => bb9,
_ => bb4
}
}
bb9 = {
_17 = _10 as f32;
_20.fld2 = _9.0;
_19.fld1 = [_27[_15],_19.fld2.fld0[_15],_13[_15]];
_19.fld2.fld0 = [_1,_13[_15],_1];
_17 = _15 as f32;
_14[_15] = _31[_15] >> _9.0;
_31 = [_14[_15],_14[_15],_14[_15],_14[_15],_23.1[_15],_14[_15]];
_20.fld3.2 = _22 >> _22;
_20.fld4 = 18347_i16 << _20.fld3.2;
_31 = _16;
_26 = _19.fld4.2;
_21 = core::ptr::addr_of!(RET);
_11.1 = _20.fld2;
_1 = _19.fld1[_15] + _19.fld2.fld0[_15];
_33 = _24 as f64;
_30 = _14[_15];
match _23.1[_15] {
0 => bb3,
1 => bb10,
2 => bb11,
3 => bb12,
8475148408019313166 => bb14,
_ => bb13
}
}
bb10 = {
_15 = 18381810107215490347_usize;
_9 = (_11.1,);
_22 = _20.fld3.2 << _20.fld2;
_5 = core::ptr::addr_of!(RET);
_15 = 4_usize ^ 12296019713933148221_usize;
_5 = _7;
_20.fld2 = !_11.1;
_23 = (_2, _16);
_4 = (_11.0, _11.1);
_19.fld2.fld0 = [_1,_1,_1];
(*_3) = (-122922387167782420284932244947166225508_i128);
_17 = _1 as f32;
_29.0 = !_12;
_2 = _20.fld3.2 as u8;
_7 = _19.fld2.fld1;
_17 = _19.fld3 as f32;
_15 = 0_usize;
_32.0 = _29.0;
(*_5) = 104162068153909241916586948195156311013_i128 + 64071690380841026413620405747790260390_i128;
_26 = _19.fld4.2;
_20.fld3.1 = _10 - _10;
_32.1 = _10;
_29 = _20.fld3;
_19.fld0 = [_19.fld4.1];
_31[_15] = _16[_15] & _16[_15];
_29.2 = (*_5) as u32;
_11 = (_4.0, _6);
_19.fld2.fld0[_15] = !_1;
match _23.1[_15] {
8475148408019313166 => bb9,
_ => bb4
}
}
bb11 = {
_9 = (_4.1,);
_6 = RET as u64;
_4.0 = 117711470633842133150059345738367071910_u128;
_4.0 = 39408484138047680494063261113803997884_u128;
_4.1 = _9.0;
_6 = _9.0 << _9.0;
_7 = core::ptr::addr_of!(RET);
_3 = _7;
_11.0 = !_4.0;
_11.1 = !_9.0;
_11 = (_4.0, _6);
_11.1 = _6;
_10 = _2 as f64;
(*_3) = (-91347048622060416595994252769083913317_i128);
(*_3) = (-1353467804_i32) as i128;
_7 = _5;
_3 = core::ptr::addr_of!(RET);
_11.0 = _2 as u128;
_9.0 = (-77_i8) as u64;
_4.0 = (-778121309_i32) as u128;
(*_3) = -159175021916209127160189951258507782174_i128;
(*_3) = (-168682232104569517546851105081472901144_i128) + (-99829490953616822986001904170284480617_i128);
_9.0 = _4.1 & _6;
_11.1 = _9.0;
Call(_4.1 = core::intrinsics::transmute(_9.0), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_19.fld4.0 = _19.fld4.1 ^ _19.fld4.1;
RET = (-55790464593725331162189333241336678160_i128) + (-139860470742923903703628876957957470083_i128);
_19.fld0 = [_19.fld4.0];
_6 = _4.1 + _20.fld2;
_19.fld4.0 = !_19.fld4.1;
_5 = core::ptr::addr_of!((*_7));
_19.fld4.1 = _19.fld4.0 & _19.fld4.0;
_16 = [8475148408019313166_i64,(-7574397687322459433_i64),7914996334424672950_i64,(-2158722762734088874_i64),(-3243489974104076799_i64),(-7376090344204011273_i64)];
_11.1 = !_9.0;
_4.1 = _11.1 + _20.fld2;
_14 = [(-8980595054722336321_i64)];
Call(_19.fld3 = core::intrinsics::fmaf64(_10, _10, _10), ReturnTo(bb7), UnwindUnreachable())
}
bb13 = {
_19.fld3 = _4.1 as f64;
_17 = _19.fld3 as f32;
_15 = 4976987645328224813_usize;
_19.fld2.fld1 = core::ptr::addr_of!((*_3));
_20.fld2 = _11.1;
_18 = _12;
_9.0 = _11.1 * _6;
_19.fld2.fld0 = [_1,_1,_1];
_20.fld3.2 = !790639366_u32;
_12 = !_18;
_20.fld3.0 = !_12;
_20.fld1 = _8;
_5 = _7;
_10 = -_19.fld3;
_19.fld4.2 = _20.fld1;
_19.fld3 = _4.1 as f64;
RET = !159449893163051277584691972170738840936_i128;
_19.fld4.1 = _19.fld4.0;
_7 = _19.fld2.fld1;
_18 = !_20.fld3.0;
(*_7) = 99_isize as i128;
_14 = [5837325415371196627_i64];
_17 = _15 as f32;
_20.fld3.2 = _19.fld4.1 as u32;
_14 = [2183181199258019598_i64];
_18 = _20.fld3.0 | _12;
_19.fld0 = [_19.fld4.1];
_20.fld1 = _8;
Goto(bb6)
}
bb14 = {
_16 = _31;
_19.fld2.fld0 = _19.fld1;
_20.fld1 = _19.fld4.2;
_20.fld3.2 = !_22;
_19.fld4 = (_19.fld0[_15], _19.fld0[_15], _26);
_19.fld2.fld1 = _21;
_8 = _20.fld1;
_4.0 = _4.1 as u128;
_19.fld4.0 = _19.fld0[_15];
(*_5) = 10314570946313734142511644437872191238_i128 + 15696631819891321936437483401155567563_i128;
_36 = -_32.1;
RET = _31[_15] as i128;
_21 = _19.fld2.fld1;
_33 = _36 * _29.1;
_19.fld2 = Adt43 { fld0: _13,fld1: _3 };
_40.1 = core::ptr::addr_of!(_4.0);
_19.fld2.fld0[_15] = _1 | _19.fld1[_15];
_16[_15] = !_14[_15];
_2 = !_23.0;
_20.fld3 = (_12, _33, _22);
_9.0 = _4.1;
_33 = _19.fld2.fld0[_15] as f64;
_40.2 = core::ptr::addr_of_mut!(_11.1);
_5 = _21;
_20.fld3 = (_29.0, _36, _22);
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(1_usize, 24_usize, Move(_24), 27_usize, Move(_27), 13_usize, Move(_13), 16_usize, Move(_16)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(1_usize, 9_usize, Move(_9), 18_usize, Move(_18), 2_usize, Move(_2), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(1_usize, 22_usize, Move(_22), 30_usize, Move(_30), 45_usize, _45, 45_usize, _45), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: *const i128,mut _2: *const i128,mut _3: u8) -> char {
mir! {
type RET = char;
let _4: char;
let _5: [u32; 6];
let _6: u16;
let _7: isize;
let _8: Adt48;
let _9: isize;
let _10: i128;
let _11: ();
let _12: ();
{
RET = '\u{75b4c}';
RET = '\u{9086f}';
_3 = 416052096_u32 as u8;
RET = '\u{2d6b0}';
RET = '\u{477e8}';
_1 = _2;
_3 = !135_u8;
_1 = _2;
RET = '\u{17112}';
RET = '\u{bae27}';
RET = '\u{88cf9}';
_2 = _1;
RET = '\u{b7778}';
_3 = !56_u8;
RET = '\u{106414}';
Call(_1 = fn3(_2, RET, _2, _2, RET, RET, _2, _2, _2, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = !29_u8;
Goto(bb2)
}
bb2 = {
_3 = 244_u8;
_2 = _1;
RET = '\u{109c2c}';
_2 = _1;
_2 = _1;
_3 = 189_u8;
_1 = _2;
_2 = _1;
RET = '\u{953e8}';
_2 = _1;
RET = '\u{2c48b}';
_1 = _2;
RET = '\u{10c3f}';
_3 = 131_u8;
_3 = 109_u8 >> (-28191347981480226041505500305964565286_i128);
_3 = 8386353821300501337_u64 as u8;
RET = '\u{45480}';
_3 = 127_u8;
RET = '\u{10d3dd}';
_1 = _2;
_1 = _2;
RET = '\u{f8bc2}';
Goto(bb3)
}
bb3 = {
_5 = [3945435398_u32,2674135355_u32,1390318227_u32,1315990167_u32,344860524_u32,808667074_u32];
RET = '\u{7bb2}';
_6 = 54651_u16;
_4 = RET;
RET = _4;
RET = _4;
_4 = RET;
_1 = _2;
RET = _4;
_4 = RET;
RET = _4;
match _6 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
54651 => bb12,
_ => bb11
}
}
bb4 = {
_3 = 244_u8;
_2 = _1;
RET = '\u{109c2c}';
_2 = _1;
_2 = _1;
_3 = 189_u8;
_1 = _2;
_2 = _1;
RET = '\u{953e8}';
_2 = _1;
RET = '\u{2c48b}';
_1 = _2;
RET = '\u{10c3f}';
_3 = 131_u8;
_3 = 109_u8 >> (-28191347981480226041505500305964565286_i128);
_3 = 8386353821300501337_u64 as u8;
RET = '\u{45480}';
_3 = 127_u8;
RET = '\u{10d3dd}';
_1 = _2;
_1 = _2;
RET = '\u{f8bc2}';
Goto(bb3)
}
bb5 = {
_3 = !29_u8;
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
_6 = 3582_u16 & 17398_u16;
_1 = _2;
RET = _4;
RET = _4;
RET = _4;
_6 = (-110_i8) as u16;
_1 = _2;
RET = _4;
_1 = _2;
_4 = RET;
_1 = _2;
_7 = 26887_i16 as isize;
Goto(bb13)
}
bb13 = {
_4 = RET;
_4 = RET;
_3 = 22733_i16 as u8;
_3 = 54_u8 - 116_u8;
Goto(bb14)
}
bb14 = {
_4 = RET;
_7 = (-9223372036854775808_isize);
_4 = RET;
_6 = 69_u16 + 6306_u16;
_10 = (-1583007945_i32) as i128;
Goto(bb15)
}
bb15 = {
Call(_11 = dump_var(2_usize, 3_usize, Move(_3), 7_usize, Move(_7), 5_usize, Move(_5), 12_usize, _12), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: *const i128,mut _2: char,mut _3: *const i128,mut _4: *const i128,mut _5: char,mut _6: char,mut _7: *const i128,mut _8: *const i128,mut _9: *const i128,mut _10: u8) -> *const i128 {
mir! {
type RET = *const i128;
let _11: char;
let _12: usize;
let _13: i8;
let _14: [u128; 4];
let _15: Adt50;
let _16: [i8; 1];
let _17: [u32; 6];
let _18: *const i128;
let _19: (bool, f64, u32);
let _20: u32;
let _21: u16;
let _22: f32;
let _23: isize;
let _24: char;
let _25: bool;
let _26: char;
let _27: (i8, i8, char);
let _28: i64;
let _29: f32;
let _30: bool;
let _31: [u16; 3];
let _32: ();
let _33: ();
{
_9 = _4;
_4 = _8;
RET = _9;
_4 = _1;
_10 = !21_u8;
Goto(bb1)
}
bb1 = {
_6 = _2;
RET = _3;
_4 = _1;
_9 = _7;
_5 = _2;
_11 = _2;
RET = _7;
_2 = _5;
_14 = [278739837520251032913271474011424123830_u128,156510704020211692976954338833917506188_u128,240101494189672430758331739843417623563_u128,180614716320466615599809725560982771605_u128];
_4 = _7;
_2 = _11;
_12 = (-84_i8) as usize;
_2 = _5;
_13 = (-8_i8);
_5 = _11;
_15.fld2.fld1 = _8;
_14 = [148912242133061801473222600439983553019_u128,167846379716366745897932916180292958433_u128,241552856952822211778792641514152148292_u128,38597753539606500696212001446485299384_u128];
_6 = _2;
_8 = _1;
_12 = 2_usize >> _13;
_10 = _12 as u8;
_16 = [_13];
_15.fld4.0 = _13 & _13;
_15.fld1 = [57513_u16,24590_u16,30453_u16];
Goto(bb2)
}
bb2 = {
_15.fld1 = [27494_u16,20157_u16,18675_u16];
_15.fld2.fld0 = [9084_u16,21109_u16,20387_u16];
_14 = [261307234657491164147918517532193636000_u128,30778474780228905499011990277690869246_u128,142511643267484179103646381496154099564_u128,194825561947504367803758976868457207187_u128];
_3 = _9;
_5 = _6;
_15.fld2.fld0 = _15.fld1;
_11 = _2;
_15.fld4 = (_13, _13, _11);
_19.1 = 61388432969327428737479317989248091457_u128 as f64;
_15.fld3 = 58444_u16 as f64;
_19.2 = !1625665650_u32;
_1 = _8;
_13 = !_15.fld4.0;
_19 = (true, _15.fld3, 4042823089_u32);
_1 = _8;
_1 = _7;
_5 = _15.fld4.2;
Call(_18 = fn4(_19.0, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_13 = _15.fld4.1;
_15.fld3 = _19.1 + _19.1;
_8 = _3;
_8 = _1;
RET = _15.fld2.fld1;
_15.fld4 = (_13, _13, _5);
_15.fld1 = [32096_u16,34130_u16,32687_u16];
_9 = _4;
_15.fld0 = _16;
_4 = _7;
_8 = _3;
_3 = RET;
_11 = _5;
_3 = _4;
_17 = [_19.2,_19.2,_19.2,_19.2,_19.2,_19.2];
_20 = (-758499176_i32) as u32;
_7 = _8;
_19.1 = -_15.fld3;
_15.fld2 = Adt43 { fld0: _15.fld1,fld1: _18 };
RET = _15.fld2.fld1;
_18 = _3;
_3 = _8;
_6 = _5;
_10 = 294257861040836572626587774723151775604_u128 as u8;
_15.fld4 = (_13, _13, _11);
_8 = _9;
Call(_15.fld0 = fn8(_15.fld2.fld1, _13, _6, _8, _18, _14, _18, _8, _4, _15.fld4.2, Move(_15.fld2), _19.0, _18, _19.1, _3, _15.fld4.2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_6 = _5;
_5 = _2;
_7 = _9;
_14 = [173727336816107362162554689431967699914_u128,313902092708583257842787478282727024391_u128,36405134161901279960145732084903733104_u128,234999016938108949160707852614277578199_u128];
_5 = _2;
_15.fld0 = _16;
_19.1 = -_15.fld3;
_3 = _1;
_14 = [51067671332402539270794034969960741012_u128,59564158475270836684098994167562833282_u128,122736563888317900950727256370766725448_u128,303745770538144823514012110077558554103_u128];
_21 = !16298_u16;
_15.fld2.fld0 = [_21,_21,_21];
_3 = RET;
_20 = _19.0 as u32;
match _13 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463463374607431768211448 => bb9,
_ => bb8
}
}
bb5 = {
_13 = _15.fld4.1;
_15.fld3 = _19.1 + _19.1;
_8 = _3;
_8 = _1;
RET = _15.fld2.fld1;
_15.fld4 = (_13, _13, _5);
_15.fld1 = [32096_u16,34130_u16,32687_u16];
_9 = _4;
_15.fld0 = _16;
_4 = _7;
_8 = _3;
_3 = RET;
_11 = _5;
_3 = _4;
_17 = [_19.2,_19.2,_19.2,_19.2,_19.2,_19.2];
_20 = (-758499176_i32) as u32;
_7 = _8;
_19.1 = -_15.fld3;
_15.fld2 = Adt43 { fld0: _15.fld1,fld1: _18 };
RET = _15.fld2.fld1;
_18 = _3;
_3 = _8;
_6 = _5;
_10 = 294257861040836572626587774723151775604_u128 as u8;
_15.fld4 = (_13, _13, _11);
_8 = _9;
Call(_15.fld0 = fn8(_15.fld2.fld1, _13, _6, _8, _18, _14, _18, _8, _4, _15.fld4.2, Move(_15.fld2), _19.0, _18, _19.1, _3, _15.fld4.2), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_15.fld1 = [27494_u16,20157_u16,18675_u16];
_15.fld2.fld0 = [9084_u16,21109_u16,20387_u16];
_14 = [261307234657491164147918517532193636000_u128,30778474780228905499011990277690869246_u128,142511643267484179103646381496154099564_u128,194825561947504367803758976868457207187_u128];
_3 = _9;
_5 = _6;
_15.fld2.fld0 = _15.fld1;
_11 = _2;
_15.fld4 = (_13, _13, _11);
_19.1 = 61388432969327428737479317989248091457_u128 as f64;
_15.fld3 = 58444_u16 as f64;
_19.2 = !1625665650_u32;
_1 = _8;
_13 = !_15.fld4.0;
_19 = (true, _15.fld3, 4042823089_u32);
_1 = _8;
_1 = _7;
_5 = _15.fld4.2;
Call(_18 = fn4(_19.0, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_6 = _2;
RET = _3;
_4 = _1;
_9 = _7;
_5 = _2;
_11 = _2;
RET = _7;
_2 = _5;
_14 = [278739837520251032913271474011424123830_u128,156510704020211692976954338833917506188_u128,240101494189672430758331739843417623563_u128,180614716320466615599809725560982771605_u128];
_4 = _7;
_2 = _11;
_12 = (-84_i8) as usize;
_2 = _5;
_13 = (-8_i8);
_5 = _11;
_15.fld2.fld1 = _8;
_14 = [148912242133061801473222600439983553019_u128,167846379716366745897932916180292958433_u128,241552856952822211778792641514152148292_u128,38597753539606500696212001446485299384_u128];
_6 = _2;
_8 = _1;
_12 = 2_usize >> _13;
_10 = _12 as u8;
_16 = [_13];
_15.fld4.0 = _13 & _13;
_15.fld1 = [57513_u16,24590_u16,30453_u16];
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_9 = _3;
_4 = _3;
RET = _18;
_12 = _13 as usize;
_19 = (false, _15.fld3, _20);
_20 = 152313870349444288310304742256866593433_i128 as u32;
_15.fld4.2 = _11;
_14 = [256328454383754510007469422530791242881_u128,140115345285230575457021407857240497911_u128,162672311402789710597668018668302254874_u128,97512754730898105594934009279869166995_u128];
RET = _8;
_15.fld3 = _12 as f64;
_19.0 = !false;
_6 = _2;
_15.fld4 = (_13, _13, _11);
_19 = (false, _15.fld3, _20);
_22 = _20 as f32;
_18 = _9;
RET = _18;
_14 = [210527695121131905222211798169782786528_u128,192974968287071758604243294481990867380_u128,24514532486201304442625379558432957762_u128,315446468848478785556737302031431960220_u128];
_15.fld2 = Adt43 { fld0: _15.fld1,fld1: _7 };
RET = _4;
_24 = _11;
RET = _4;
_13 = _15.fld4.0;
_15.fld2.fld0 = [_21,_21,_21];
_23 = (-9223372036854775808_isize);
_15.fld2 = Adt43 { fld0: _15.fld1,fld1: _18 };
_16 = [_13];
match _23 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb10,
5 => bb11,
340282366920938463454151235394913435648 => bb13,
_ => bb12
}
}
bb10 = {
_13 = _15.fld4.1;
_15.fld3 = _19.1 + _19.1;
_8 = _3;
_8 = _1;
RET = _15.fld2.fld1;
_15.fld4 = (_13, _13, _5);
_15.fld1 = [32096_u16,34130_u16,32687_u16];
_9 = _4;
_15.fld0 = _16;
_4 = _7;
_8 = _3;
_3 = RET;
_11 = _5;
_3 = _4;
_17 = [_19.2,_19.2,_19.2,_19.2,_19.2,_19.2];
_20 = (-758499176_i32) as u32;
_7 = _8;
_19.1 = -_15.fld3;
_15.fld2 = Adt43 { fld0: _15.fld1,fld1: _18 };
RET = _15.fld2.fld1;
_18 = _3;
_3 = _8;
_6 = _5;
_10 = 294257861040836572626587774723151775604_u128 as u8;
_15.fld4 = (_13, _13, _11);
_8 = _9;
Call(_15.fld0 = fn8(_15.fld2.fld1, _13, _6, _8, _18, _14, _18, _8, _4, _15.fld4.2, Move(_15.fld2), _19.0, _18, _19.1, _3, _15.fld4.2), ReturnTo(bb4), UnwindUnreachable())
}
bb11 = {
_13 = _15.fld4.1;
_15.fld3 = _19.1 + _19.1;
_8 = _3;
_8 = _1;
RET = _15.fld2.fld1;
_15.fld4 = (_13, _13, _5);
_15.fld1 = [32096_u16,34130_u16,32687_u16];
_9 = _4;
_15.fld0 = _16;
_4 = _7;
_8 = _3;
_3 = RET;
_11 = _5;
_3 = _4;
_17 = [_19.2,_19.2,_19.2,_19.2,_19.2,_19.2];
_20 = (-758499176_i32) as u32;
_7 = _8;
_19.1 = -_15.fld3;
_15.fld2 = Adt43 { fld0: _15.fld1,fld1: _18 };
RET = _15.fld2.fld1;
_18 = _3;
_3 = _8;
_6 = _5;
_10 = 294257861040836572626587774723151775604_u128 as u8;
_15.fld4 = (_13, _13, _11);
_8 = _9;
Call(_15.fld0 = fn8(_15.fld2.fld1, _13, _6, _8, _18, _14, _18, _8, _4, _15.fld4.2, Move(_15.fld2), _19.0, _18, _19.1, _3, _15.fld4.2), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_6 = _5;
_5 = _2;
_7 = _9;
_14 = [173727336816107362162554689431967699914_u128,313902092708583257842787478282727024391_u128,36405134161901279960145732084903733104_u128,234999016938108949160707852614277578199_u128];
_5 = _2;
_15.fld0 = _16;
_19.1 = -_15.fld3;
_3 = _1;
_14 = [51067671332402539270794034969960741012_u128,59564158475270836684098994167562833282_u128,122736563888317900950727256370766725448_u128,303745770538144823514012110077558554103_u128];
_21 = !16298_u16;
_15.fld2.fld0 = [_21,_21,_21];
_3 = RET;
_20 = _19.0 as u32;
match _13 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463463374607431768211448 => bb9,
_ => bb8
}
}
bb13 = {
_19 = (true, _15.fld3, _20);
_9 = _3;
_20 = _19.2;
_15.fld1 = [_21,_21,_21];
_1 = _3;
_10 = !154_u8;
_27.1 = _13;
_26 = _11;
_16 = [_15.fld4.1];
match _23 {
0 => bb9,
1 => bb2,
2 => bb11,
3 => bb8,
4 => bb5,
340282366920938463454151235394913435648 => bb15,
_ => bb14
}
}
bb14 = {
_6 = _2;
RET = _3;
_4 = _1;
_9 = _7;
_5 = _2;
_11 = _2;
RET = _7;
_2 = _5;
_14 = [278739837520251032913271474011424123830_u128,156510704020211692976954338833917506188_u128,240101494189672430758331739843417623563_u128,180614716320466615599809725560982771605_u128];
_4 = _7;
_2 = _11;
_12 = (-84_i8) as usize;
_2 = _5;
_13 = (-8_i8);
_5 = _11;
_15.fld2.fld1 = _8;
_14 = [148912242133061801473222600439983553019_u128,167846379716366745897932916180292958433_u128,241552856952822211778792641514152148292_u128,38597753539606500696212001446485299384_u128];
_6 = _2;
_8 = _1;
_12 = 2_usize >> _13;
_10 = _12 as u8;
_16 = [_13];
_15.fld4.0 = _13 & _13;
_15.fld1 = [57513_u16,24590_u16,30453_u16];
Goto(bb2)
}
bb15 = {
_15.fld4.1 = _27.1 | _15.fld4.0;
_23 = _2 as isize;
RET = _18;
_28 = 1328878609_i32 as i64;
_5 = _15.fld4.2;
Goto(bb16)
}
bb16 = {
Call(_32 = dump_var(3_usize, 14_usize, Move(_14), 11_usize, Move(_11), 24_usize, Move(_24), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(3_usize, 6_usize, Move(_6), 13_usize, Move(_13), 17_usize, Move(_17), 20_usize, Move(_20)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: bool,mut _2: *const i128) -> *const i128 {
mir! {
type RET = *const i128;
let _3: [i64; 6];
let _4: Adt47;
let _5: Adt58;
let _6: bool;
let _7: i128;
let _8: *const u128;
let _9: char;
let _10: f64;
let _11: usize;
let _12: [u16; 3];
let _13: *mut u64;
let _14: Adt48;
let _15: i64;
let _16: *mut u64;
let _17: char;
let _18: (u8, [i64; 6]);
let _19: u128;
let _20: [u32; 6];
let _21: Adt54;
let _22: Adt46;
let _23: [u128; 4];
let _24: ();
let _25: ();
{
RET = _2;
RET = _2;
RET = _2;
_3 = [5343489302015734607_i64,(-792403910811916997_i64),8554949377321030380_i64,2550736312995139761_i64,5628176699135326844_i64,4707014561536742510_i64];
_1 = 230_u8 >= 125_u8;
_1 = !true;
RET = _2;
RET = _2;
RET = _2;
_2 = RET;
RET = _2;
_2 = RET;
_3 = [3209940008360267617_i64,(-4101137300350991311_i64),(-5964177301526333044_i64),829208334634470515_i64,4133233416823789476_i64,4762921992620156141_i64];
_1 = !true;
_1 = !true;
RET = _2;
RET = _2;
_1 = false;
RET = _2;
_2 = RET;
_3 = [(-4430873240605812909_i64),5323836340666666172_i64,(-5520893588730602982_i64),(-8219076905493053028_i64),(-4227492622200239031_i64),(-378047634929354655_i64)];
RET = _2;
RET = _2;
_2 = RET;
Goto(bb1)
}
bb1 = {
RET = _2;
RET = _2;
_2 = RET;
RET = _2;
Call(RET = fn5(_3, _2, _3, _3, _3, _1, _1, _1, _3, _2, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = !_1;
_1 = _6;
_2 = RET;
RET = _2;
Goto(bb3)
}
bb3 = {
_1 = !_6;
_2 = RET;
_6 = _1 >= _1;
_7 = 2434986499145353771412980042484854336_i128;
RET = _2;
_2 = RET;
_6 = _1;
_6 = _1 ^ _1;
RET = core::ptr::addr_of!(_7);
RET = _2;
RET = _2;
RET = _2;
_1 = _7 >= _7;
RET = core::ptr::addr_of!(_7);
RET = core::ptr::addr_of!((*RET));
_9 = '\u{5a0d}';
Goto(bb4)
}
bb4 = {
_9 = '\u{d60b0}';
_3 = [3423439778032667648_i64,(-4523008264884639510_i64),5106790649898776753_i64,2127054691740785168_i64,687322507842822690_i64,5310896696045972042_i64];
(*RET) = 67991488657116601658997200187054333270_i128;
_10 = 3863832954_u32 as f64;
_3 = [(-5618547852706248272_i64),2932762738862924857_i64,3999209718208395621_i64,5810317132444564987_i64,1921343213813279151_i64,7082196286181604656_i64];
(*RET) = (-42573115142002319158953954348316929710_i128) | 13576045687754185436644194885096436486_i128;
_9 = '\u{6c07e}';
RET = core::ptr::addr_of!((*RET));
(*RET) = (-67547349985236452189442135608533463122_i128);
_2 = core::ptr::addr_of!((*RET));
(*RET) = 147_u8 as i128;
(*_2) = -38377926055205142734837256322513125025_i128;
_9 = '\u{10aeac}';
(*RET) = 518939782695899846_i64 as i128;
_11 = 3563510428842612544_usize;
RET = core::ptr::addr_of!(_7);
_7 = 129728253197496457338776169392740111747_i128 - (-49287076759629513554414956688821939300_i128);
_12 = [63246_u16,1628_u16,63340_u16];
(*RET) = !(-107420042768963143333941934979476593273_i128);
_1 = !_6;
RET = core::ptr::addr_of!(_7);
(*RET) = (-101538835055308450825565281082794617956_i128) & 74945603369432635439543006528454083533_i128;
Goto(bb5)
}
bb5 = {
_15 = !4846819875239162547_i64;
_15 = -(-587993658459098562_i64);
_3 = [_15,_15,_15,_15,_15,_15];
RET = _2;
(*RET) = 58746610191068758828812440367092995793_i128 - (-85955096673898828950220638768916800936_i128);
(*RET) = 93117284_i32 as i128;
(*_2) = 255_u8 as i128;
_15 = 3282225725_u32 as i64;
_10 = 8875543356732038640_u64 as f64;
_12 = [22308_u16,34970_u16,57042_u16];
(*_2) = 84_i8 as i128;
_7 = 306794792094546432766471047324191217703_u128 as i128;
_7 = (-93271192027141603353430370586034519204_i128) - (-34286580243020262322830711406659723212_i128);
_18 = (178_u8, _3);
_18 = (191_u8, _3);
_1 = !_6;
(*_2) = (-83929217491827966866483781773412142293_i128);
_7 = -(-52809356274209501300025399441119393276_i128);
_17 = _9;
_6 = _1;
RET = core::ptr::addr_of!((*_2));
_9 = _17;
_7 = !(-157206156106849317809759807741383344209_i128);
_6 = _1;
(*RET) = _11 as i128;
(*RET) = 90958115969748791558195721505663208096_i128;
Goto(bb6)
}
bb6 = {
_7 = !52007044431421768537921413301207169765_i128;
_7 = (-95693163146925993758195823655910979239_i128);
RET = core::ptr::addr_of!((*RET));
_2 = core::ptr::addr_of!((*RET));
_1 = _6 >= _6;
_6 = !_1;
_6 = _1;
_18.1 = [_15,_15,_15,_15,_15,_15];
RET = core::ptr::addr_of!((*_2));
_18 = (209_u8, _3);
_10 = _15 as f64;
_19 = 158149371001145949625934395520143675896_u128;
(*RET) = (-24158853961816913429123965778219087526_i128) + 62276917258760724640975528431383128700_i128;
_18.0 = 52_u8;
_20 = [392440924_u32,1055107757_u32,3544232359_u32,2453250081_u32,726055242_u32,145361987_u32];
_18.0 = _10 as u8;
RET = core::ptr::addr_of!(_7);
_8 = core::ptr::addr_of!(_19);
_3 = [_15,_15,_15,_15,_15,_15];
_18.1 = _3;
_8 = core::ptr::addr_of!((*_8));
(*_2) = (-20_i8) as i128;
(*_8) = !17016602211228443234651591083552362056_u128;
(*_8) = !273665110813761584566481880909168303939_u128;
RET = _2;
RET = core::ptr::addr_of!((*RET));
match _11 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
3563510428842612544 => bb13,
_ => bb12
}
}
bb7 = {
_15 = !4846819875239162547_i64;
_15 = -(-587993658459098562_i64);
_3 = [_15,_15,_15,_15,_15,_15];
RET = _2;
(*RET) = 58746610191068758828812440367092995793_i128 - (-85955096673898828950220638768916800936_i128);
(*RET) = 93117284_i32 as i128;
(*_2) = 255_u8 as i128;
_15 = 3282225725_u32 as i64;
_10 = 8875543356732038640_u64 as f64;
_12 = [22308_u16,34970_u16,57042_u16];
(*_2) = 84_i8 as i128;
_7 = 306794792094546432766471047324191217703_u128 as i128;
_7 = (-93271192027141603353430370586034519204_i128) - (-34286580243020262322830711406659723212_i128);
_18 = (178_u8, _3);
_18 = (191_u8, _3);
_1 = !_6;
(*_2) = (-83929217491827966866483781773412142293_i128);
_7 = -(-52809356274209501300025399441119393276_i128);
_17 = _9;
_6 = _1;
RET = core::ptr::addr_of!((*_2));
_9 = _17;
_7 = !(-157206156106849317809759807741383344209_i128);
_6 = _1;
(*RET) = _11 as i128;
(*RET) = 90958115969748791558195721505663208096_i128;
Goto(bb6)
}
bb8 = {
_9 = '\u{d60b0}';
_3 = [3423439778032667648_i64,(-4523008264884639510_i64),5106790649898776753_i64,2127054691740785168_i64,687322507842822690_i64,5310896696045972042_i64];
(*RET) = 67991488657116601658997200187054333270_i128;
_10 = 3863832954_u32 as f64;
_3 = [(-5618547852706248272_i64),2932762738862924857_i64,3999209718208395621_i64,5810317132444564987_i64,1921343213813279151_i64,7082196286181604656_i64];
(*RET) = (-42573115142002319158953954348316929710_i128) | 13576045687754185436644194885096436486_i128;
_9 = '\u{6c07e}';
RET = core::ptr::addr_of!((*RET));
(*RET) = (-67547349985236452189442135608533463122_i128);
_2 = core::ptr::addr_of!((*RET));
(*RET) = 147_u8 as i128;
(*_2) = -38377926055205142734837256322513125025_i128;
_9 = '\u{10aeac}';
(*RET) = 518939782695899846_i64 as i128;
_11 = 3563510428842612544_usize;
RET = core::ptr::addr_of!(_7);
_7 = 129728253197496457338776169392740111747_i128 - (-49287076759629513554414956688821939300_i128);
_12 = [63246_u16,1628_u16,63340_u16];
(*RET) = !(-107420042768963143333941934979476593273_i128);
_1 = !_6;
RET = core::ptr::addr_of!(_7);
(*RET) = (-101538835055308450825565281082794617956_i128) & 74945603369432635439543006528454083533_i128;
Goto(bb5)
}
bb9 = {
_1 = !_6;
_2 = RET;
_6 = _1 >= _1;
_7 = 2434986499145353771412980042484854336_i128;
RET = _2;
_2 = RET;
_6 = _1;
_6 = _1 ^ _1;
RET = core::ptr::addr_of!(_7);
RET = _2;
RET = _2;
RET = _2;
_1 = _7 >= _7;
RET = core::ptr::addr_of!(_7);
RET = core::ptr::addr_of!((*RET));
_9 = '\u{5a0d}';
Goto(bb4)
}
bb10 = {
_6 = !_1;
_1 = _6;
_2 = RET;
RET = _2;
Goto(bb3)
}
bb11 = {
RET = _2;
RET = _2;
_2 = RET;
RET = _2;
Call(RET = fn5(_3, _2, _3, _3, _3, _1, _1, _1, _3, _2, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
Return()
}
bb13 = {
RET = _2;
_15 = -6186575849466436396_i64;
_18.1 = [_15,_15,_15,_15,_15,_15];
_8 = core::ptr::addr_of!(_19);
_15 = !1140452766876554714_i64;
RET = core::ptr::addr_of!((*_2));
_19 = 214589670701937427434398124788265601435_u128;
_18.1 = [_15,_15,_15,_15,_15,_15];
(*RET) = (-62232812168322946618460407746371592875_i128);
RET = _2;
_6 = !_1;
_18 = (135_u8, _3);
(*_2) = (-36520729244137394489219659147279604387_i128);
(*RET) = _15 as i128;
(*_8) = !340073426542359589606875891465138019945_u128;
RET = _2;
Goto(bb14)
}
bb14 = {
_6 = !_1;
(*RET) = (-52259733363829904971682638400728206587_i128) + 21769804267360213178061950351864796598_i128;
_18 = (229_u8, _3);
_19 = _6 as u128;
_9 = _17;
_3 = _18.1;
Goto(bb15)
}
bb15 = {
Call(_24 = dump_var(4_usize, 12_usize, Move(_12), 19_usize, Move(_19), 20_usize, Move(_20), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_24 = dump_var(4_usize, 18_usize, Move(_18), 11_usize, Move(_11), 25_usize, _25, 25_usize, _25), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: [i64; 6],mut _2: *const i128,mut _3: [i64; 6],mut _4: [i64; 6],mut _5: [i64; 6],mut _6: bool,mut _7: bool,mut _8: bool,mut _9: [i64; 6],mut _10: *const i128,mut _11: [i64; 6]) -> *const i128 {
mir! {
type RET = *const i128;
let _12: char;
let _13: [i64; 1];
let _14: f64;
let _15: Adt56;
let _16: bool;
let _17: (u8, [i64; 6]);
let _18: isize;
let _19: isize;
let _20: [i8; 1];
let _21: bool;
let _22: usize;
let _23: Adt47;
let _24: f64;
let _25: Adt55;
let _26: [u32; 6];
let _27: bool;
let _28: [i64; 6];
let _29: (i8, i8, char);
let _30: isize;
let _31: isize;
let _32: f64;
let _33: isize;
let _34: isize;
let _35: i16;
let _36: ();
let _37: ();
{
RET = _10;
_9 = _4;
RET = _2;
_12 = '\u{2943d}';
_17.0 = 176_u8;
_12 = '\u{87bdf}';
_16 = _17.0 >= _17.0;
_19 = (-9223372036854775808_isize);
_18 = 1_usize as isize;
_17 = (190_u8, _11);
_11 = _4;
_17.1 = _3;
_8 = _7;
_18 = _19 - _19;
RET = _10;
_19 = 106_i8 as isize;
_16 = _6;
_2 = _10;
_17 = (255_u8, _3);
_13 = [(-4913828011617845152_i64)];
_16 = _17.0 > _17.0;
_19 = _18;
match _17.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
255 => bb7,
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
_11 = [(-7952312053245379656_i64),(-5993235073058034339_i64),(-16192797376041116_i64),(-8025566629908812321_i64),(-8968152945554211752_i64),2759883110837269428_i64];
_11 = [(-499843679086976012_i64),(-2015329674392653714_i64),(-5573324920302310254_i64),7832876206845009730_i64,(-8404067911885167332_i64),4421466071655223165_i64];
_7 = _17.0 != _17.0;
_11 = [(-8254584110998853337_i64),(-7701982720601588927_i64),5115627703972766173_i64,(-573582826551124404_i64),(-6748648087793532801_i64),1535558885703400669_i64];
_21 = _16 ^ _6;
_4 = _1;
_13 = [(-595954315705135456_i64)];
_12 = '\u{cd526}';
_21 = _7;
_17 = (160_u8, _3);
_14 = (-24_i8) as f64;
_1 = _3;
_14 = 1_usize as f64;
_13 = [(-5874790279903027278_i64)];
_10 = RET;
_17.1 = [3399283282352238118_i64,(-4378458267240048536_i64),(-1083819312658970461_i64),(-214335301454800056_i64),4900424105711515250_i64,1414830234879257281_i64];
_22 = 7726018100724261059_i64 as usize;
match _17.0 {
160 => bb8,
_ => bb3
}
}
bb8 = {
_3 = [7573406511382205545_i64,5860655807874185372_i64,6997060299777791652_i64,7125479988400930828_i64,(-4554194504253927076_i64),(-1894096086704029028_i64)];
_4 = [(-387401018682260566_i64),(-8090665747010129962_i64),(-7736364928853502189_i64),(-6732179935195633940_i64),2265107194897602005_i64,9059181084050691296_i64];
_17.0 = !218_u8;
_20 = [7_i8];
_9 = _17.1;
_7 = !_16;
_4 = _9;
_3 = _17.1;
_9 = _11;
_5 = [(-1035473778080607805_i64),(-2294258833254764654_i64),8223895167986585573_i64,1363901000104479223_i64,(-8506256831010390866_i64),4779152520071569071_i64];
_4 = [(-1933070505176143693_i64),1029796396077840125_i64,3921875412763837167_i64,2360845446631984273_i64,3493718766764742682_i64,3466444339226746064_i64];
Call(_11 = fn6(RET, _16, _16, _5, _19, _10, _4, _19, _7), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_1 = [(-412967492872461621_i64),6349704307483055954_i64,(-8559678021570976096_i64),(-4858523563260704312_i64),4955230715156182852_i64,262908118439793497_i64];
_6 = _7 > _16;
_17.0 = 234_u8 * 69_u8;
_13 = [(-553137033550021138_i64)];
_7 = !_16;
_19 = _18;
_13 = [(-7897406202359849222_i64)];
_17.0 = 213_u8 ^ 177_u8;
_10 = _2;
_17.0 = 27_u8 - 226_u8;
_19 = 627433399325189580_i64 as isize;
_18 = _19;
_27 = !_6;
_29 = ((-85_i8), 99_i8, _12);
_3 = [1791613964716015839_i64,1802307708692317272_i64,(-8425075487491227012_i64),4500168076801326141_i64,5213060962645384134_i64,2917891432950518708_i64];
_17.0 = 118_u8 - 169_u8;
match _29.0 {
0 => bb10,
1 => bb11,
340282366920938463463374607431768211371 => bb13,
_ => bb12
}
}
bb10 = {
_3 = [7573406511382205545_i64,5860655807874185372_i64,6997060299777791652_i64,7125479988400930828_i64,(-4554194504253927076_i64),(-1894096086704029028_i64)];
_4 = [(-387401018682260566_i64),(-8090665747010129962_i64),(-7736364928853502189_i64),(-6732179935195633940_i64),2265107194897602005_i64,9059181084050691296_i64];
_17.0 = !218_u8;
_20 = [7_i8];
_9 = _17.1;
_7 = !_16;
_4 = _9;
_3 = _17.1;
_9 = _11;
_5 = [(-1035473778080607805_i64),(-2294258833254764654_i64),8223895167986585573_i64,1363901000104479223_i64,(-8506256831010390866_i64),4779152520071569071_i64];
_4 = [(-1933070505176143693_i64),1029796396077840125_i64,3921875412763837167_i64,2360845446631984273_i64,3493718766764742682_i64,3466444339226746064_i64];
Call(_11 = fn6(RET, _16, _16, _5, _19, _10, _4, _19, _7), ReturnTo(bb9), UnwindUnreachable())
}
bb11 = {
_11 = [(-7952312053245379656_i64),(-5993235073058034339_i64),(-16192797376041116_i64),(-8025566629908812321_i64),(-8968152945554211752_i64),2759883110837269428_i64];
_11 = [(-499843679086976012_i64),(-2015329674392653714_i64),(-5573324920302310254_i64),7832876206845009730_i64,(-8404067911885167332_i64),4421466071655223165_i64];
_7 = _17.0 != _17.0;
_11 = [(-8254584110998853337_i64),(-7701982720601588927_i64),5115627703972766173_i64,(-573582826551124404_i64),(-6748648087793532801_i64),1535558885703400669_i64];
_21 = _16 ^ _6;
_4 = _1;
_13 = [(-595954315705135456_i64)];
_12 = '\u{cd526}';
_21 = _7;
_17 = (160_u8, _3);
_14 = (-24_i8) as f64;
_1 = _3;
_14 = 1_usize as f64;
_13 = [(-5874790279903027278_i64)];
_10 = RET;
_17.1 = [3399283282352238118_i64,(-4378458267240048536_i64),(-1083819312658970461_i64),(-214335301454800056_i64),4900424105711515250_i64,1414830234879257281_i64];
_22 = 7726018100724261059_i64 as usize;
match _17.0 {
160 => bb8,
_ => bb3
}
}
bb12 = {
Return()
}
bb13 = {
_1 = [4593199064332539091_i64,7864499985430336571_i64,3548547852047260373_i64,4304413748110023296_i64,3212738496748546322_i64,(-2425348842097716585_i64)];
_10 = RET;
_13 = [(-5743907607704662215_i64)];
_7 = _16;
_7 = _29.1 >= _29.0;
_29.0 = _29.1 << _22;
RET = _10;
_29.0 = 66400464_u32 as i8;
_29.1 = _22 as i8;
_29.2 = _12;
_29 = (98_i8, (-20_i8), _12);
_12 = _29.2;
_29 = (28_i8, 49_i8, _12);
_26 = [3640310224_u32,4119296872_u32,3959031415_u32,2828794657_u32,2712485536_u32,142909243_u32];
_21 = _27 < _27;
Goto(bb14)
}
bb14 = {
_20 = [_29.0];
_1 = [(-4465042511093805509_i64),3528188546264274918_i64,5747759068602146275_i64,2720374161207092142_i64,(-4455038761450399044_i64),(-5841725000586549192_i64)];
_3 = _17.1;
RET = _10;
_30 = -_18;
_10 = _2;
_2 = _10;
_19 = !_18;
_17.0 = 5531725204445572741_i64 as u8;
_1 = [3228780263418620641_i64,(-949402189696301653_i64),(-5148247958286018456_i64),1462559482174026355_i64,7464042249465057850_i64,(-860700047898600924_i64)];
_19 = _30 << _29.1;
_27 = _6 | _16;
_17.1 = [7156346675951580939_i64,(-8949105563745432502_i64),9094984518275175168_i64,(-3503602072551857022_i64),(-7415001251411228065_i64),248588289454706059_i64];
_3 = _11;
_14 = 12535669829675897377_u64 as f64;
_29.2 = _12;
_11 = _3;
_21 = _6;
_24 = _14 - _14;
_6 = _21;
_17.1 = [8592179432878244611_i64,4476794736692785225_i64,9148661464844996892_i64,4354377719605117994_i64,2953931181450309747_i64,(-3174720906486870764_i64)];
_2 = _10;
_26 = [819930484_u32,3698129285_u32,3948850727_u32,3796448940_u32,3025634059_u32,2051705777_u32];
_3 = [170388620888604009_i64,(-6167677428302949685_i64),(-3815122254574889749_i64),2097091111947573608_i64,5062744877595429134_i64,9118470506395952780_i64];
_17.0 = !244_u8;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(5_usize, 3_usize, Move(_3), 21_usize, Move(_21), 17_usize, Move(_17), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(5_usize, 7_usize, Move(_7), 1_usize, Move(_1), 6_usize, Move(_6), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(5_usize, 19_usize, Move(_19), 22_usize, Move(_22), 4_usize, Move(_4), 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: *const i128,mut _2: bool,mut _3: bool,mut _4: [i64; 6],mut _5: isize,mut _6: *const i128,mut _7: [i64; 6],mut _8: isize,mut _9: bool) -> [i64; 6] {
mir! {
type RET = [i64; 6];
let _10: bool;
let _11: *mut u64;
let _12: (u16,);
let _13: isize;
let _14: [i8; 1];
let _15: [u128; 4];
let _16: i32;
let _17: Adt42;
let _18: (u16,);
let _19: [i64; 6];
let _20: u32;
let _21: isize;
let _22: Adt55;
let _23: isize;
let _24: Adt42;
let _25: isize;
let _26: i128;
let _27: u64;
let _28: bool;
let _29: Adt51;
let _30: isize;
let _31: (i8, i8, char);
let _32: [u128; 4];
let _33: (bool, i64, u32);
let _34: [i64; 6];
let _35: (bool, f64, u32);
let _36: u64;
let _37: f64;
let _38: isize;
let _39: [i64; 1];
let _40: f64;
let _41: ();
let _42: ();
{
RET = _4;
RET = _4;
_10 = !_3;
_9 = !_2;
_1 = _6;
_6 = _1;
_8 = 124_i8 as isize;
_2 = !_9;
_6 = _1;
_5 = 17873_i16 as isize;
RET = _7;
RET = [(-4492117317607460274_i64),(-7282297517879355294_i64),(-7638763933616121348_i64),4398988446758485085_i64,(-4910687178936760240_i64),6351724841635380235_i64];
_12.0 = _5 as u16;
_9 = !_2;
_3 = _9 <= _2;
_6 = _1;
Goto(bb1)
}
bb1 = {
_8 = '\u{6c011}' as isize;
_6 = _1;
_12.0 = '\u{8da09}' as u16;
Call(_9 = fn7(_8, _4, RET, _3, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = _3 == _3;
_2 = _10;
_9 = !_3;
_1 = _6;
_13 = _5;
_9 = _3 <= _3;
_12 = (18947_u16,);
_4 = _7;
_7 = RET;
Goto(bb3)
}
bb3 = {
_12 = (18326_u16,);
_3 = _2;
_8 = !_13;
_2 = _9 < _9;
_12 = (3955_u16,);
_17 = Adt42::Variant0 { fld0: _6 };
Goto(bb4)
}
bb4 = {
_16 = -(-1407361844_i32);
_3 = _9 > _2;
Goto(bb5)
}
bb5 = {
_15 = [82157961537585652787898705065293008974_u128,310698820166874819584891417493663634746_u128,65839720532663934042610643996096222228_u128,112919772571051684260079950706972669994_u128];
_12 = (3301_u16,);
RET = [6260794339170013317_i64,1489202113047237233_i64,2972403591670488684_i64,9191106459999701421_i64,(-8957409767193572593_i64),(-5414007798060532929_i64)];
_18 = _12;
_9 = _2;
_12.0 = _18.0 / _18.0;
_2 = !_9;
_5 = 4663292297970268021_u64 as isize;
_20 = !4072829893_u32;
_8 = -_13;
_8 = _13 >> _16;
_19 = [7813970604746386658_i64,5113557397603122376_i64,(-3479512924741508632_i64),2210497437273665191_i64,557455722872369683_i64,(-2270949086829108251_i64)];
_6 = Field::<*const i128>(Variant(_17, 0), 0);
_12.0 = _18.0;
_10 = _2;
_12.0 = _3 as u16;
_12 = (_18.0,);
Goto(bb6)
}
bb6 = {
_3 = _9 != _2;
place!(Field::<*const i128>(Variant(_17, 0), 0)) = _6;
RET = [(-1523117337757945241_i64),8585397202961249366_i64,5093414698297271326_i64,1187998979460913124_i64,(-700900043600835979_i64),7421204416948182155_i64];
_19 = [(-5831816280709275794_i64),(-3464371337908392655_i64),(-246936686427917229_i64),3012129582961151436_i64,(-357938930822681483_i64),8746878196496555221_i64];
_15 = [212305262130181913787929128000926234840_u128,95244480248110115133001596017037011717_u128,164947015794609019358188615376036044105_u128,333000911353581104340286746176312602679_u128];
_1 = _6;
_21 = _5 | _8;
_19 = _7;
RET = [928580492255760729_i64,7284303330292999351_i64,(-5166726513019916063_i64),(-5137333266917279103_i64),1795471336758893004_i64,1909127191537576673_i64];
_23 = _5;
_14 = [70_i8];
_7 = [6122544778375923922_i64,2392983748812812392_i64,(-3346955527694226849_i64),(-8992100138286662376_i64),(-7284702161413857812_i64),7012006435644677370_i64];
_24 = _17;
RET = [(-1700337189227936927_i64),(-4569487984496824755_i64),1687626579318156144_i64,(-9147669111232869194_i64),2533038502302900431_i64,3599272405904090170_i64];
_20 = 1562029839_u32 ^ 1479178351_u32;
SetDiscriminant(_24, 0);
_11 = core::ptr::addr_of_mut!(_27);
_28 = _3;
match _12.0 {
0 => bb4,
3301 => bb7,
_ => bb3
}
}
bb7 = {
_31.2 = '\u{3d7c}';
_25 = _21;
(*_11) = 137_u8 as u64;
place!(Field::<*const i128>(Variant(_17, 0), 0)) = core::ptr::addr_of!(_26);
_4 = [(-2974867250819236183_i64),(-1130418691376780429_i64),8673924070114391720_i64,4660507947983391906_i64,(-1070689841764465645_i64),(-7041795372872802235_i64)];
_6 = core::ptr::addr_of!(_26);
_33 = (_28, (-7325375127022514827_i64), _20);
_30 = _8 - _25;
_4 = [_33.1,_33.1,_33.1,_33.1,_33.1,_33.1];
_23 = _21;
_12.0 = !_18.0;
_2 = _9;
_18 = _12;
(*_11) = !1024780665802407329_u64;
(*_6) = _5 as i128;
SetDiscriminant(_17, 0);
_16 = (-1390989717_i32);
match _33.1 {
340282366920938463456049232304745696629 => bb9,
_ => bb8
}
}
bb8 = {
_2 = _3 == _3;
_2 = _10;
_9 = !_3;
_1 = _6;
_13 = _5;
_9 = _3 <= _3;
_12 = (18947_u16,);
_4 = _7;
_7 = RET;
Goto(bb3)
}
bb9 = {
_24 = Adt42::Variant0 { fld0: _1 };
match _33.1 {
0 => bb1,
1 => bb5,
2 => bb3,
3 => bb10,
340282366920938463456049232304745696629 => bb12,
_ => bb11
}
}
bb10 = {
_3 = _9 != _2;
place!(Field::<*const i128>(Variant(_17, 0), 0)) = _6;
RET = [(-1523117337757945241_i64),8585397202961249366_i64,5093414698297271326_i64,1187998979460913124_i64,(-700900043600835979_i64),7421204416948182155_i64];
_19 = [(-5831816280709275794_i64),(-3464371337908392655_i64),(-246936686427917229_i64),3012129582961151436_i64,(-357938930822681483_i64),8746878196496555221_i64];
_15 = [212305262130181913787929128000926234840_u128,95244480248110115133001596017037011717_u128,164947015794609019358188615376036044105_u128,333000911353581104340286746176312602679_u128];
_1 = _6;
_21 = _5 | _8;
_19 = _7;
RET = [928580492255760729_i64,7284303330292999351_i64,(-5166726513019916063_i64),(-5137333266917279103_i64),1795471336758893004_i64,1909127191537576673_i64];
_23 = _5;
_14 = [70_i8];
_7 = [6122544778375923922_i64,2392983748812812392_i64,(-3346955527694226849_i64),(-8992100138286662376_i64),(-7284702161413857812_i64),7012006435644677370_i64];
_24 = _17;
RET = [(-1700337189227936927_i64),(-4569487984496824755_i64),1687626579318156144_i64,(-9147669111232869194_i64),2533038502302900431_i64,3599272405904090170_i64];
_20 = 1562029839_u32 ^ 1479178351_u32;
SetDiscriminant(_24, 0);
_11 = core::ptr::addr_of_mut!(_27);
_28 = _3;
match _12.0 {
0 => bb4,
3301 => bb7,
_ => bb3
}
}
bb11 = {
_12 = (18326_u16,);
_3 = _2;
_8 = !_13;
_2 = _9 < _9;
_12 = (3955_u16,);
_17 = Adt42::Variant0 { fld0: _6 };
Goto(bb4)
}
bb12 = {
_35.1 = (-24381_i16) as f64;
_31 = ((-25_i8), (-12_i8), '\u{edc54}');
_34 = [_33.1,_33.1,_33.1,_33.1,_33.1,_33.1];
_31 = ((-98_i8), (-42_i8), '\u{37c0}');
(*_11) = 13154240540526149030_usize as u64;
_1 = core::ptr::addr_of!((*_6));
_33.2 = !_20;
_18.0 = _12.0;
SetDiscriminant(_24, 1);
_8 = _25;
_18.0 = !_12.0;
Goto(bb13)
}
bb13 = {
place!(Field::<(u128, u64)>(Variant(_24, 1), 6)).1 = (*_11);
_33.0 = _10;
place!(Field::<*const i128>(Variant(_17, 0), 0)) = _6;
place!(Field::<u64>(Variant(_24, 1), 1)) = (*_11);
_31.1 = _31.0;
_4 = [_33.1,_33.1,_33.1,_33.1,_33.1,_33.1];
(*_11) = Field::<(u128, u64)>(Variant(_24, 1), 6).1;
_9 = _3;
_10 = _9 | _28;
_11 = core::ptr::addr_of_mut!((*_11));
_33.2 = _31.1 as u32;
_9 = _28;
Call(place!(Field::<*const i128>(Variant(_24, 1), 3)) = core::intrinsics::arith_offset(Field::<*const i128>(Variant(_17, 0), 0), (-70_isize)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(6_usize, 33_usize, Move(_33), 23_usize, Move(_23), 31_usize, Move(_31), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(6_usize, 19_usize, Move(_19), 34_usize, Move(_34), 8_usize, Move(_8), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(6_usize, 18_usize, Move(_18), 5_usize, Move(_5), 28_usize, Move(_28), 3_usize, Move(_3)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(6_usize, 4_usize, Move(_4), 42_usize, _42, 42_usize, _42, 42_usize, _42), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: isize,mut _2: [i64; 6],mut _3: [i64; 6],mut _4: bool,mut _5: bool) -> bool {
mir! {
type RET = bool;
let _6: [i64; 6];
let _7: Adt48;
let _8: &'static u128;
let _9: isize;
let _10: (u8, [i64; 6]);
let _11: (bool, f64, u32);
let _12: [i64; 6];
let _13: u32;
let _14: isize;
let _15: (u128, u64);
let _16: ();
let _17: ();
{
_1 = (-112_isize) & (-9223372036854775808_isize);
_5 = _4;
_4 = !_5;
_6 = _2;
_4 = _5;
RET = _4 ^ _4;
_6 = [(-2345132941635863025_i64),(-6121539646714799051_i64),(-5436244114100631204_i64),(-6960158877524004519_i64),(-2160754997103980857_i64),2783438317917281594_i64];
RET = !_5;
_3 = [586106954643435167_i64,8825868208695418582_i64,893431284599365755_i64,6589097345334668683_i64,(-1019982998791251190_i64),4557897296806821099_i64];
_2 = [2954165985255201543_i64,2516584481568877173_i64,8796924143384536473_i64,(-1727392635071920022_i64),(-445179044990289376_i64),7363932325321891422_i64];
_5 = _4 | RET;
_2 = _3;
_10 = (114_u8, _6);
_3 = [8961658002211487608_i64,4198670848736543608_i64,446262218553982868_i64,1342489619738149852_i64,3192787007092871798_i64,(-1208943824464479450_i64)];
_8 = &_15.0;
_12 = _2;
_9 = _5 as isize;
_11.2 = 2526187238_u32 | 3074186193_u32;
_11.1 = 318567263855068086715017776845787033652_u128 as f64;
_3 = [206762874370723389_i64,3072481681842637027_i64,4465531833145797864_i64,3763744773566894334_i64,(-8866343855839905420_i64),(-1710056409190805417_i64)];
_13 = _11.2;
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(7_usize, 10_usize, Move(_10), 5_usize, Move(_5), 4_usize, Move(_4), 3_usize, Move(_3)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_16 = dump_var(7_usize, 2_usize, Move(_2), 17_usize, _17, 17_usize, _17, 17_usize, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: *const i128,mut _2: i8,mut _3: char,mut _4: *const i128,mut _5: *const i128,mut _6: [u128; 4],mut _7: *const i128,mut _8: *const i128,mut _9: *const i128,mut _10: char,mut _11: Adt43,mut _12: bool,mut _13: *const i128,mut _14: f64,mut _15: *const i128,mut _16: char) -> [i8; 1] {
mir! {
type RET = [i8; 1];
let _17: u16;
let _18: *mut (bool, i64, u32);
let _19: Adt42;
let _20: (i8, i8, char);
let _21: (i8, i8, char);
let _22: (bool, f64, u32);
let _23: bool;
let _24: i32;
let _25: f64;
let _26: (u8, [i64; 6]);
let _27: f64;
let _28: *const *mut u64;
let _29: [i128; 7];
let _30: Adt43;
let _31: isize;
let _32: Adt54;
let _33: (u16,);
let _34: (u64,);
let _35: Adt53;
let _36: char;
let _37: Adt43;
let _38: [u128; 4];
let _39: f32;
let _40: *const *mut u64;
let _41: (bool, i64, u32);
let _42: isize;
let _43: Adt43;
let _44: f32;
let _45: *mut i128;
let _46: f32;
let _47: (u128, u64);
let _48: Adt49;
let _49: ();
let _50: ();
{
_7 = _11.fld1;
RET = [_2];
_14 = 223362582082821652953274588079123271970_u128 as f64;
_11.fld1 = _5;
_9 = _5;
_14 = 14616494052101623500_u64 as f64;
_13 = _1;
_12 = _14 != _14;
_7 = _4;
_4 = _1;
_5 = _8;
_7 = _4;
_2 = (-82_i8) >> 9128_u16;
Call(_11.fld1 = fn9(_7, _6, _10, _13, _11.fld0, _16, _13), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [_2];
_3 = _10;
_15 = _8;
RET = [_2];
RET = [_2];
_12 = false;
RET = [_2];
_20.0 = (-1853531942_i32) as i8;
_17 = 9223372036854775807_isize as u16;
_17 = (-145118555269477564752031231906546231771_i128) as u16;
_1 = _9;
_2 = _17 as i8;
_21.2 = _16;
_21.1 = _20.0 ^ _2;
_21 = (_20.0, _2, _3);
_21 = (_20.0, _20.0, _16);
_20.1 = -_2;
_23 = _12;
_20.2 = _21.2;
Goto(bb2)
}
bb2 = {
_21.2 = _16;
Goto(bb3)
}
bb3 = {
_6 = [72953980587518529618596318342031486162_u128,160533461293491954843574135582655471510_u128,56686535111016776092705014927332772320_u128,215038609198342332898580014028431365525_u128];
_16 = _10;
_19 = Adt42::Variant0 { fld0: _5 };
_17 = 2021100290_i32 as u16;
_4 = _8;
_3 = _16;
_6 = [310558531653520736505056240793345805393_u128,7711093420255423553969210395583939841_u128,247162764322939845282881579635119470860_u128,63677929748649254826370492780304278914_u128];
_22.0 = _12;
_22 = (_23, _14, 1506174551_u32);
_20.1 = _2 + _2;
_12 = _22.0;
Goto(bb4)
}
bb4 = {
_20.0 = _20.1;
_25 = -_14;
_22.0 = _12;
_25 = -_14;
_9 = _1;
_23 = !_12;
_5 = _4;
_21.2 = _3;
_8 = _7;
_21 = (_20.0, _20.0, _20.2);
_7 = _9;
_21 = (_20.1, _20.1, _10);
SetDiscriminant(_19, 1);
_22 = (_23, _25, 252001665_u32);
_20 = (_21.0, _21.1, _16);
RET = [_20.1];
_17 = (-18676_i16) as u16;
place!(Field::<(u128, u64)>(Variant(_19, 1), 6)).1 = (-1346227374_i32) as u64;
place!(Field::<(u128, u64)>(Variant(_19, 1), 6)) = (225261477796326754727242066825167719299_u128, 4071483763936519822_u64);
_21.0 = _20.0;
place!(Field::<u64>(Variant(_19, 1), 1)) = _12 as u64;
_3 = _16;
place!(Field::<[i8; 1]>(Variant(_19, 1), 4)) = [_2];
_20.1 = _20.0 ^ _21.1;
_3 = _10;
_16 = _21.2;
place!(Field::<f32>(Variant(_19, 1), 2)) = _17 as f32;
Call(place!(Field::<i32>(Variant(_19, 1), 5)) = fn10(_9, _4, _5, _11.fld1, _25, RET, Field::<(u128, u64)>(Variant(_19, 1), 6).1, _1, _7, _10, _21.2, _22.0, Move(_11), _15, _5), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_30.fld1 = _5;
_1 = _4;
place!(Field::<(u128, u64)>(Variant(_19, 1), 6)) = (305166911661194115250264462322915315910_u128, Field::<u64>(Variant(_19, 1), 1));
place!(Field::<[i8; 1]>(Variant(_19, 1), 4)) = RET;
_21.0 = _20.0 - _21.1;
_24 = !Field::<i32>(Variant(_19, 1), 5);
_4 = _1;
_27 = _25;
_14 = _27 + _25;
_30.fld0 = [_17,_17,_17];
place!(Field::<*const i128>(Variant(_19, 1), 3)) = _7;
_11.fld0 = _30.fld0;
_29 = [(-129911876620587165706892287997862985078_i128),(-76438188842592257847392099916875901857_i128),(-108994004761888243298287461157765506299_i128),77883335685724700132374189343025795895_i128,156389070185720129267605035189962176750_i128,(-125122879150320817173538257551516491150_i128),111676699784629426785904031942663205283_i128];
_20 = (_21.0, _21.0, _16);
Goto(bb6)
}
bb6 = {
_31 = 70_isize;
_12 = _23;
_30 = Adt43 { fld0: _11.fld0,fld1: _13 };
_20.1 = _20.0 ^ _2;
_21.0 = -_20.1;
_21.1 = !_20.0;
_10 = _3;
_20 = _21;
_20.0 = _2 >> _17;
_10 = _16;
_19 = Adt42::Variant0 { fld0: _4 };
_20 = (_21.1, _21.1, _16);
_35.fld3 = _22;
_21.2 = _16;
Goto(bb7)
}
bb7 = {
_26.0 = !159_u8;
_7 = _1;
_3 = _20.2;
_3 = _20.2;
_17 = 20806_u16;
_9 = _15;
_3 = _16;
_35.fld2 = !6365859973232418615_u64;
_36 = _10;
_37.fld0 = _30.fld0;
_38 = [319265301435119483306554264988000718870_u128,176051771636849784744211175083285226965_u128,75706011517025856140813688965181934133_u128,183346084316968048014212177736494277609_u128];
_1 = _9;
_4 = _5;
_34.0 = _26.0 as u64;
SetDiscriminant(_19, 0);
_22.0 = _21.1 <= _21.1;
_35.fld4 = (-20710_i16) >> _24;
_33 = (_17,);
_8 = _9;
RET = [_21.0];
_23 = _16 >= _16;
_20.0 = !_20.1;
_27 = _35.fld3.1 - _14;
RET = [_21.0];
_22.0 = _12;
Call(_17 = core::intrinsics::transmute(_35.fld4), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_10 = _20.2;
_24 = (-1910477011_i32);
_22.1 = _35.fld3.1 - _25;
_35.fld3.0 = _23 ^ _12;
place!(Field::<*const i128>(Variant(_19, 0), 0)) = _8;
_30.fld0 = [_17,_17,_17];
_22.1 = _21.1 as f64;
_38 = [69604496304138397671799734609516362886_u128,301573978863059702689156287496240733314_u128,22575403639818711437769923704939605695_u128,192884514731461905998487409686876921273_u128];
_43.fld0 = [_17,_17,_17];
_17 = _33.0;
_22.2 = !_35.fld3.2;
match _24 {
0 => bb7,
340282366920938463463374607429857734445 => bb9,
_ => bb5
}
}
bb9 = {
_35.fld3.0 = !_22.0;
_43.fld1 = _7;
_42 = _24 as isize;
_23 = !_12;
_9 = _30.fld1;
_26.0 = !187_u8;
_41.0 = _21.1 > _20.1;
_35.fld2 = _34.0 + _34.0;
_24 = (-1094473059_i32);
_33.0 = _26.0 as u16;
Goto(bb10)
}
bb10 = {
_30 = Adt43 { fld0: _43.fld0,fld1: _9 };
_43.fld1 = _1;
RET = [_21.0];
_11 = Adt43 { fld0: _30.fld0,fld1: _7 };
_35.fld1 = _10;
match _24 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
340282366920938463463374607430673738397 => bb18,
_ => bb17
}
}
bb11 = {
_35.fld3.0 = !_22.0;
_43.fld1 = _7;
_42 = _24 as isize;
_23 = !_12;
_9 = _30.fld1;
_26.0 = !187_u8;
_41.0 = _21.1 > _20.1;
_35.fld2 = _34.0 + _34.0;
_24 = (-1094473059_i32);
_33.0 = _26.0 as u16;
Goto(bb10)
}
bb12 = {
_10 = _20.2;
_24 = (-1910477011_i32);
_22.1 = _35.fld3.1 - _25;
_35.fld3.0 = _23 ^ _12;
place!(Field::<*const i128>(Variant(_19, 0), 0)) = _8;
_30.fld0 = [_17,_17,_17];
_22.1 = _21.1 as f64;
_38 = [69604496304138397671799734609516362886_u128,301573978863059702689156287496240733314_u128,22575403639818711437769923704939605695_u128,192884514731461905998487409686876921273_u128];
_43.fld0 = [_17,_17,_17];
_17 = _33.0;
_22.2 = !_35.fld3.2;
match _24 {
0 => bb7,
340282366920938463463374607429857734445 => bb9,
_ => bb5
}
}
bb13 = {
_26.0 = !159_u8;
_7 = _1;
_3 = _20.2;
_3 = _20.2;
_17 = 20806_u16;
_9 = _15;
_3 = _16;
_35.fld2 = !6365859973232418615_u64;
_36 = _10;
_37.fld0 = _30.fld0;
_38 = [319265301435119483306554264988000718870_u128,176051771636849784744211175083285226965_u128,75706011517025856140813688965181934133_u128,183346084316968048014212177736494277609_u128];
_1 = _9;
_4 = _5;
_34.0 = _26.0 as u64;
SetDiscriminant(_19, 0);
_22.0 = _21.1 <= _21.1;
_35.fld4 = (-20710_i16) >> _24;
_33 = (_17,);
_8 = _9;
RET = [_21.0];
_23 = _16 >= _16;
_20.0 = !_20.1;
_27 = _35.fld3.1 - _14;
RET = [_21.0];
_22.0 = _12;
Call(_17 = core::intrinsics::transmute(_35.fld4), ReturnTo(bb8), UnwindUnreachable())
}
bb14 = {
_31 = 70_isize;
_12 = _23;
_30 = Adt43 { fld0: _11.fld0,fld1: _13 };
_20.1 = _20.0 ^ _2;
_21.0 = -_20.1;
_21.1 = !_20.0;
_10 = _3;
_20 = _21;
_20.0 = _2 >> _17;
_10 = _16;
_19 = Adt42::Variant0 { fld0: _4 };
_20 = (_21.1, _21.1, _16);
_35.fld3 = _22;
_21.2 = _16;
Goto(bb7)
}
bb15 = {
_30.fld1 = _5;
_1 = _4;
place!(Field::<(u128, u64)>(Variant(_19, 1), 6)) = (305166911661194115250264462322915315910_u128, Field::<u64>(Variant(_19, 1), 1));
place!(Field::<[i8; 1]>(Variant(_19, 1), 4)) = RET;
_21.0 = _20.0 - _21.1;
_24 = !Field::<i32>(Variant(_19, 1), 5);
_4 = _1;
_27 = _25;
_14 = _27 + _25;
_30.fld0 = [_17,_17,_17];
place!(Field::<*const i128>(Variant(_19, 1), 3)) = _7;
_11.fld0 = _30.fld0;
_29 = [(-129911876620587165706892287997862985078_i128),(-76438188842592257847392099916875901857_i128),(-108994004761888243298287461157765506299_i128),77883335685724700132374189343025795895_i128,156389070185720129267605035189962176750_i128,(-125122879150320817173538257551516491150_i128),111676699784629426785904031942663205283_i128];
_20 = (_21.0, _21.0, _16);
Goto(bb6)
}
bb16 = {
_20.0 = _20.1;
_25 = -_14;
_22.0 = _12;
_25 = -_14;
_9 = _1;
_23 = !_12;
_5 = _4;
_21.2 = _3;
_8 = _7;
_21 = (_20.0, _20.0, _20.2);
_7 = _9;
_21 = (_20.1, _20.1, _10);
SetDiscriminant(_19, 1);
_22 = (_23, _25, 252001665_u32);
_20 = (_21.0, _21.1, _16);
RET = [_20.1];
_17 = (-18676_i16) as u16;
place!(Field::<(u128, u64)>(Variant(_19, 1), 6)).1 = (-1346227374_i32) as u64;
place!(Field::<(u128, u64)>(Variant(_19, 1), 6)) = (225261477796326754727242066825167719299_u128, 4071483763936519822_u64);
_21.0 = _20.0;
place!(Field::<u64>(Variant(_19, 1), 1)) = _12 as u64;
_3 = _16;
place!(Field::<[i8; 1]>(Variant(_19, 1), 4)) = [_2];
_20.1 = _20.0 ^ _21.1;
_3 = _10;
_16 = _21.2;
place!(Field::<f32>(Variant(_19, 1), 2)) = _17 as f32;
Call(place!(Field::<i32>(Variant(_19, 1), 5)) = fn10(_9, _4, _5, _11.fld1, _25, RET, Field::<(u128, u64)>(Variant(_19, 1), 6).1, _1, _7, _10, _21.2, _22.0, Move(_11), _15, _5), ReturnTo(bb5), UnwindUnreachable())
}
bb17 = {
_6 = [72953980587518529618596318342031486162_u128,160533461293491954843574135582655471510_u128,56686535111016776092705014927332772320_u128,215038609198342332898580014028431365525_u128];
_16 = _10;
_19 = Adt42::Variant0 { fld0: _5 };
_17 = 2021100290_i32 as u16;
_4 = _8;
_3 = _16;
_6 = [310558531653520736505056240793345805393_u128,7711093420255423553969210395583939841_u128,247162764322939845282881579635119470860_u128,63677929748649254826370492780304278914_u128];
_22.0 = _12;
_22 = (_23, _14, 1506174551_u32);
_20.1 = _2 + _2;
_12 = _22.0;
Goto(bb4)
}
bb18 = {
_11.fld0 = [_33.0,_33.0,_17];
_12 = !_35.fld3.0;
_35.fld1 = _20.2;
_41.0 = _35.fld4 != _35.fld4;
_36 = _10;
_31 = _42;
_41.0 = !_22.0;
Goto(bb19)
}
bb19 = {
Call(_49 = dump_var(8_usize, 16_usize, Move(_16), 42_usize, Move(_42), 24_usize, Move(_24), 29_usize, Move(_29)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_49 = dump_var(8_usize, 23_usize, Move(_23), 33_usize, Move(_33), 2_usize, Move(_2), 17_usize, Move(_17)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_49 = dump_var(8_usize, 3_usize, Move(_3), 50_usize, _50, 50_usize, _50, 50_usize, _50), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: *const i128,mut _2: [u128; 4],mut _3: char,mut _4: *const i128,mut _5: [u16; 3],mut _6: char,mut _7: *const i128) -> *const i128 {
mir! {
type RET = *const i128;
let _8: f32;
let _9: i32;
let _10: [u32; 6];
let _11: i16;
let _12: (u16,);
let _13: i16;
let _14: u32;
let _15: Adt47;
let _16: (bool, i64, u32);
let _17: Adt47;
let _18: usize;
let _19: Adt48;
let _20: f64;
let _21: isize;
let _22: Adt47;
let _23: (u128, u64);
let _24: char;
let _25: u128;
let _26: *const i128;
let _27: usize;
let _28: [i128; 7];
let _29: u8;
let _30: [i64; 1];
let _31: (u16,);
let _32: *mut (bool, i64, u32);
let _33: u32;
let _34: i64;
let _35: Adt57;
let _36: Adt50;
let _37: [u32; 6];
let _38: i16;
let _39: f32;
let _40: [i64; 1];
let _41: i16;
let _42: (bool, i64, u32);
let _43: bool;
let _44: u16;
let _45: [i64; 1];
let _46: ();
let _47: ();
{
_1 = _7;
_3 = _6;
_2 = [136887271659815822352018955286844567770_u128,127547026212483941097506527733440507167_u128,184596273643749726648749145957451813593_u128,58047214795872118071481192890606313085_u128];
_4 = _7;
Goto(bb1)
}
bb1 = {
_3 = _6;
_1 = _4;
_1 = _4;
RET = _1;
RET = _4;
_1 = _7;
_6 = _3;
_7 = RET;
RET = _7;
_2 = [231683831056284346979514208893333328712_u128,196149075385760807742411905841194140879_u128,266798308974943507385790010739297898824_u128,98233432418959390078641415324598248647_u128];
_8 = (-9223372036854775808_isize) as f32;
_6 = _3;
RET = _4;
RET = _1;
_8 = 260896657834749902398802127113355322099_u128 as f32;
RET = _1;
_5 = [16748_u16,58775_u16,29166_u16];
_2 = [230151848486494468780687718490424781759_u128,223929371479307997592636163966418294269_u128,117950828062993401728342230707180086761_u128,204710627100441701375013621443536954467_u128];
_2 = [41628354325782421958940283611073789464_u128,213764234420195125801713714367486881289_u128,305513653920549115258432236791121731903_u128,129916446267659302967696402542673992181_u128];
_9 = 1904953793_i32;
_7 = _4;
_12 = (63125_u16,);
RET = _4;
_11 = (-8638_i16);
Goto(bb2)
}
bb2 = {
_9 = 398825237_i32;
_11 = (-838_i16);
_13 = _11 | _11;
_1 = _7;
_14 = !2337168725_u32;
_3 = _6;
_10 = [_14,_14,_14,_14,_14,_14];
RET = _1;
_10 = [_14,_14,_14,_14,_14,_14];
_18 = !5504842241559364548_usize;
_16.0 = _13 >= _13;
_16 = (false, 7444114635242379835_i64, _14);
_18 = 2_usize;
_22 = Adt47::Variant0 { fld0: _16,fld1: _13 };
_11 = -_13;
place!(Field::<i16>(Variant(_22, 0), 1)) = -_11;
_16.0 = Field::<i16>(Variant(_22, 0), 1) <= _13;
_2 = [130852601260890132370016160046552804371_u128,189154376269127034245015687657407699841_u128,62281749087458135822433406240615630891_u128,63006281476907699090735420055333320542_u128];
match _2[_18] {
0 => bb3,
62281749087458135822433406240615630891 => bb5,
_ => bb4
}
}
bb3 = {
_3 = _6;
_1 = _4;
_1 = _4;
RET = _1;
RET = _4;
_1 = _7;
_6 = _3;
_7 = RET;
RET = _7;
_2 = [231683831056284346979514208893333328712_u128,196149075385760807742411905841194140879_u128,266798308974943507385790010739297898824_u128,98233432418959390078641415324598248647_u128];
_8 = (-9223372036854775808_isize) as f32;
_6 = _3;
RET = _4;
RET = _1;
_8 = 260896657834749902398802127113355322099_u128 as f32;
RET = _1;
_5 = [16748_u16,58775_u16,29166_u16];
_2 = [230151848486494468780687718490424781759_u128,223929371479307997592636163966418294269_u128,117950828062993401728342230707180086761_u128,204710627100441701375013621443536954467_u128];
_2 = [41628354325782421958940283611073789464_u128,213764234420195125801713714367486881289_u128,305513653920549115258432236791121731903_u128,129916446267659302967696402542673992181_u128];
_9 = 1904953793_i32;
_7 = _4;
_12 = (63125_u16,);
RET = _4;
_11 = (-8638_i16);
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
_23.1 = !16196102444417333205_u64;
_16 = (Field::<(bool, i64, u32)>(Variant(_22, 0), 0).0, Field::<(bool, i64, u32)>(Variant(_22, 0), 0).1, _10[_18]);
_14 = !Field::<(bool, i64, u32)>(Variant(_22, 0), 0).2;
_24 = _6;
place!(Field::<(bool, i64, u32)>(Variant(_22, 0), 0)) = (_16.0, _16.1, _10[_18]);
_23.0 = !_2[_18];
_8 = 9223372036854775807_isize as f32;
_23.0 = !_2[_18];
_12.0 = _5[_18];
_15 = _22;
_4 = RET;
_27 = _23.0 as usize;
_28[_18] = 47916121282960993468288698848804083693_i128;
_23.1 = 15428891086053766417_u64 + 15972249547507157070_u64;
_7 = _1;
_20 = Field::<(bool, i64, u32)>(Variant(_22, 0), 0).2 as f64;
_10[_18] = Field::<(bool, i64, u32)>(Variant(_22, 0), 0).2 & _14;
_26 = RET;
place!(Field::<(bool, i64, u32)>(Variant(_22, 0), 0)).1 = -Field::<(bool, i64, u32)>(Variant(_15, 0), 0).1;
match _16.1 {
0 => bb1,
1 => bb2,
2 => bb3,
7444114635242379835 => bb7,
_ => bb6
}
}
bb6 = {
_3 = _6;
_1 = _4;
_1 = _4;
RET = _1;
RET = _4;
_1 = _7;
_6 = _3;
_7 = RET;
RET = _7;
_2 = [231683831056284346979514208893333328712_u128,196149075385760807742411905841194140879_u128,266798308974943507385790010739297898824_u128,98233432418959390078641415324598248647_u128];
_8 = (-9223372036854775808_isize) as f32;
_6 = _3;
RET = _4;
RET = _1;
_8 = 260896657834749902398802127113355322099_u128 as f32;
RET = _1;
_5 = [16748_u16,58775_u16,29166_u16];
_2 = [230151848486494468780687718490424781759_u128,223929371479307997592636163966418294269_u128,117950828062993401728342230707180086761_u128,204710627100441701375013621443536954467_u128];
_2 = [41628354325782421958940283611073789464_u128,213764234420195125801713714367486881289_u128,305513653920549115258432236791121731903_u128,129916446267659302967696402542673992181_u128];
_9 = 1904953793_i32;
_7 = _4;
_12 = (63125_u16,);
RET = _4;
_11 = (-8638_i16);
Goto(bb2)
}
bb7 = {
_5[_18] = _12.0;
_11 = _13 - _13;
_1 = RET;
_27 = _18;
_23 = (_2[_18], 8358750365260566244_u64);
_16 = (Field::<(bool, i64, u32)>(Variant(_15, 0), 0).0, Field::<(bool, i64, u32)>(Variant(_15, 0), 0).1, _14);
place!(Field::<(bool, i64, u32)>(Variant(_22, 0), 0)).1 = Field::<(bool, i64, u32)>(Variant(_15, 0), 0).1 + Field::<(bool, i64, u32)>(Variant(_15, 0), 0).1;
_26 = _7;
_21 = _28[_18] as isize;
Call(_25 = core::intrinsics::bswap(_2[_18]), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
place!(Field::<(bool, i64, u32)>(Variant(_15, 0), 0)).0 = !Field::<(bool, i64, u32)>(Variant(_22, 0), 0).0;
_15 = _22;
_21 = (-9223372036854775808_isize);
_5 = [_12.0,_12.0,_12.0];
_1 = RET;
_10[_18] = _16.2;
_28 = [(-56983959408592939062000530295146823784_i128),10243297658323711953737295669781305062_i128,8134726493788983024652730825546555517_i128,64128096000724670437900403565804320478_i128,(-41380484094063364753154191497880890877_i128),70127870038583107839784085897209220202_i128,(-30473409695929736375664211007909694922_i128)];
_3 = _24;
_17 = _22;
_16.0 = !Field::<(bool, i64, u32)>(Variant(_15, 0), 0).0;
_11 = !Field::<i16>(Variant(_17, 0), 1);
_21 = (-9223372036854775808_isize);
_18 = !_27;
_15 = _22;
Goto(bb9)
}
bb9 = {
_28[_27] = -(-80395167239744344460733299535566017342_i128);
_24 = _3;
_17 = Adt47::Variant0 { fld0: Field::<(bool, i64, u32)>(Variant(_22, 0), 0),fld1: _11 };
place!(Field::<(bool, i64, u32)>(Variant(_17, 0), 0)).2 = _16.2;
_23.0 = _2[_27] << _16.2;
RET = _26;
place!(Field::<(bool, i64, u32)>(Variant(_17, 0), 0)).1 = Field::<(bool, i64, u32)>(Variant(_15, 0), 0).1 * Field::<(bool, i64, u32)>(Variant(_15, 0), 0).1;
place!(Field::<(bool, i64, u32)>(Variant(_15, 0), 0)).2 = _14 * _16.2;
_17 = _15;
place!(Field::<i16>(Variant(_17, 0), 1)) = _13;
place!(Field::<i16>(Variant(_17, 0), 1)) = _21 as i16;
_4 = _26;
place!(Field::<(bool, i64, u32)>(Variant(_17, 0), 0)).2 = _20 as u32;
Call(_18 = core::intrinsics::transmute(_23.1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_29 = 19_u8;
_31.0 = !_12.0;
_36.fld4.1 = 59_i8;
Goto(bb11)
}
bb11 = {
_10 = [_16.2,_16.2,_16.2,Field::<(bool, i64, u32)>(Variant(_15, 0), 0).2,Field::<(bool, i64, u32)>(Variant(_15, 0), 0).2,Field::<(bool, i64, u32)>(Variant(_17, 0), 0).2];
_36.fld2 = Adt43 { fld0: _5,fld1: RET };
SetDiscriminant(_15, 1);
_4 = _36.fld2.fld1;
_36.fld0 = [_36.fld4.1];
_10[_27] = !_16.2;
_36.fld4.2 = _24;
match _27 {
0 => bb1,
1 => bb12,
3 => bb14,
4 => bb15,
2 => bb17,
_ => bb16
}
}
bb12 = {
_9 = 398825237_i32;
_11 = (-838_i16);
_13 = _11 | _11;
_1 = _7;
_14 = !2337168725_u32;
_3 = _6;
_10 = [_14,_14,_14,_14,_14,_14];
RET = _1;
_10 = [_14,_14,_14,_14,_14,_14];
_18 = !5504842241559364548_usize;
_16.0 = _13 >= _13;
_16 = (false, 7444114635242379835_i64, _14);
_18 = 2_usize;
_22 = Adt47::Variant0 { fld0: _16,fld1: _13 };
_11 = -_13;
place!(Field::<i16>(Variant(_22, 0), 1)) = -_11;
_16.0 = Field::<i16>(Variant(_22, 0), 1) <= _13;
_2 = [130852601260890132370016160046552804371_u128,189154376269127034245015687657407699841_u128,62281749087458135822433406240615630891_u128,63006281476907699090735420055333320542_u128];
match _2[_18] {
0 => bb3,
62281749087458135822433406240615630891 => bb5,
_ => bb4
}
}
bb13 = {
_28[_27] = -(-80395167239744344460733299535566017342_i128);
_24 = _3;
_17 = Adt47::Variant0 { fld0: Field::<(bool, i64, u32)>(Variant(_22, 0), 0),fld1: _11 };
place!(Field::<(bool, i64, u32)>(Variant(_17, 0), 0)).2 = _16.2;
_23.0 = _2[_27] << _16.2;
RET = _26;
place!(Field::<(bool, i64, u32)>(Variant(_17, 0), 0)).1 = Field::<(bool, i64, u32)>(Variant(_15, 0), 0).1 * Field::<(bool, i64, u32)>(Variant(_15, 0), 0).1;
place!(Field::<(bool, i64, u32)>(Variant(_15, 0), 0)).2 = _14 * _16.2;
_17 = _15;
place!(Field::<i16>(Variant(_17, 0), 1)) = _13;
place!(Field::<i16>(Variant(_17, 0), 1)) = _21 as i16;
_4 = _26;
place!(Field::<(bool, i64, u32)>(Variant(_17, 0), 0)).2 = _20 as u32;
Call(_18 = core::intrinsics::transmute(_23.1), ReturnTo(bb10), UnwindUnreachable())
}
bb14 = {
_3 = _6;
_1 = _4;
_1 = _4;
RET = _1;
RET = _4;
_1 = _7;
_6 = _3;
_7 = RET;
RET = _7;
_2 = [231683831056284346979514208893333328712_u128,196149075385760807742411905841194140879_u128,266798308974943507385790010739297898824_u128,98233432418959390078641415324598248647_u128];
_8 = (-9223372036854775808_isize) as f32;
_6 = _3;
RET = _4;
RET = _1;
_8 = 260896657834749902398802127113355322099_u128 as f32;
RET = _1;
_5 = [16748_u16,58775_u16,29166_u16];
_2 = [230151848486494468780687718490424781759_u128,223929371479307997592636163966418294269_u128,117950828062993401728342230707180086761_u128,204710627100441701375013621443536954467_u128];
_2 = [41628354325782421958940283611073789464_u128,213764234420195125801713714367486881289_u128,305513653920549115258432236791121731903_u128,129916446267659302967696402542673992181_u128];
_9 = 1904953793_i32;
_7 = _4;
_12 = (63125_u16,);
RET = _4;
_11 = (-8638_i16);
Goto(bb2)
}
bb15 = {
_5[_18] = _12.0;
_11 = _13 - _13;
_1 = RET;
_27 = _18;
_23 = (_2[_18], 8358750365260566244_u64);
_16 = (Field::<(bool, i64, u32)>(Variant(_15, 0), 0).0, Field::<(bool, i64, u32)>(Variant(_15, 0), 0).1, _14);
place!(Field::<(bool, i64, u32)>(Variant(_22, 0), 0)).1 = Field::<(bool, i64, u32)>(Variant(_15, 0), 0).1 + Field::<(bool, i64, u32)>(Variant(_15, 0), 0).1;
_26 = _7;
_21 = _28[_18] as isize;
Call(_25 = core::intrinsics::bswap(_2[_18]), ReturnTo(bb8), UnwindUnreachable())
}
bb16 = {
Return()
}
bb17 = {
_10[_27] = Field::<(bool, i64, u32)>(Variant(_22, 0), 0).2;
_36.fld2.fld0 = [_5[_27],_12.0,_12.0];
place!(Field::<(bool, i64, u32)>(Variant(_22, 0), 0)).0 = Field::<(bool, i64, u32)>(Variant(_17, 0), 0).0 & Field::<(bool, i64, u32)>(Variant(_17, 0), 0).0;
_16.1 = Field::<(bool, i64, u32)>(Variant(_22, 0), 0).1 ^ Field::<(bool, i64, u32)>(Variant(_22, 0), 0).1;
SetDiscriminant(_22, 0);
_24 = _36.fld4.2;
_36.fld1[_27] = _5[_27];
place!(Field::<i128>(Variant(_15, 1), 0)) = _28[_27];
_34 = _36.fld4.2 as i64;
_37 = _10;
_22 = _17;
RET = core::ptr::addr_of!(_28[_27]);
_36.fld1[_27] = Field::<i128>(Variant(_15, 1), 0) as u16;
_3 = _24;
_32 = core::ptr::addr_of_mut!(_16);
_42.0 = !_16.0;
SetDiscriminant(_17, 0);
(*_32).1 = !Field::<(bool, i64, u32)>(Variant(_22, 0), 0).1;
_10[_27] = (*_32).2;
_12.0 = !_31.0;
_10 = _37;
_29 = _31.0 as u8;
_36.fld1 = [_31.0,_31.0,_12.0];
Goto(bb18)
}
bb18 = {
Call(_46 = dump_var(9_usize, 37_usize, Move(_37), 9_usize, Move(_9), 27_usize, Move(_27), 16_usize, Move(_16)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_46 = dump_var(9_usize, 10_usize, Move(_10), 29_usize, Move(_29), 6_usize, Move(_6), 2_usize, Move(_2)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_46 = dump_var(9_usize, 18_usize, Move(_18), 14_usize, Move(_14), 25_usize, Move(_25), 47_usize, _47), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: *const i128,mut _2: *const i128,mut _3: *const i128,mut _4: *const i128,mut _5: f64,mut _6: [i8; 1],mut _7: u64,mut _8: *const i128,mut _9: *const i128,mut _10: char,mut _11: char,mut _12: bool,mut _13: Adt43,mut _14: *const i128,mut _15: *const i128) -> i32 {
mir! {
type RET = i32;
let _16: (u8, [i64; 6]);
let _17: usize;
let _18: f32;
let _19: (bool, i64, u32);
let _20: *const *const *mut u64;
let _21: [i8; 1];
let _22: i128;
let _23: isize;
let _24: [i64; 6];
let _25: (bool, f64, u32);
let _26: isize;
let _27: Adt48;
let _28: char;
let _29: *mut i128;
let _30: (u128, u64);
let _31: bool;
let _32: Adt44;
let _33: bool;
let _34: f32;
let _35: bool;
let _36: u8;
let _37: i8;
let _38: u8;
let _39: ();
let _40: ();
{
RET = !(-1569460359_i32);
_14 = _13.fld1;
RET = (-72906225498017039288639366610213686450_i128) as i32;
_12 = false | false;
_10 = _11;
_3 = _2;
_13.fld1 = _1;
_16.1 = [7926918988832208715_i64,6799183623377947338_i64,(-435750648814227062_i64),(-4963998918176087568_i64),(-4585042794783826493_i64),8011646091089946555_i64];
_8 = _3;
_16.1 = [(-2323186570154483797_i64),(-3043934782027433005_i64),5020059026058557918_i64,(-4007401306423988792_i64),(-2181775203020123661_i64),(-1314117552267805632_i64)];
match _7 {
0 => bb1,
1 => bb2,
4071483763936519822 => bb4,
_ => bb3
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
_12 = true;
_14 = _3;
_19.2 = !2003498778_u32;
_18 = 46886067964287959076475618054079713515_u128 as f32;
_12 = _5 > _5;
_18 = _19.2 as f32;
_4 = core::ptr::addr_of!(_22);
_19.1 = (-466479259043881017_i64);
_18 = RET as f32;
_16.1 = [_19.1,_19.1,_19.1,_19.1,_19.1,_19.1];
_7 = 14749649974064389523_u64;
_16.0 = _7 as u8;
_19.0 = !_12;
_10 = _11;
_1 = _2;
_9 = core::ptr::addr_of!((*_4));
_13.fld0 = [63754_u16,20850_u16,63584_u16];
RET = 143179842698646992167160002724814841334_i128 as i32;
_13.fld1 = _14;
match _19.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463462908128172724330439 => bb9,
_ => bb8
}
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
_2 = _1;
_17 = 14_i8 as usize;
_5 = _18 as f64;
_10 = _11;
_21 = _6;
_18 = _17 as f32;
RET = 807816478_i32;
_15 = core::ptr::addr_of!(_22);
_19.2 = 1722249097_u32 + 1556368248_u32;
RET = _5 as i32;
_4 = _3;
(*_9) = 34629788956697793483706079501164776586_i128 << _17;
_23 = 9223372036854775807_isize;
_17 = !9899175785109666165_usize;
_3 = _8;
_4 = core::ptr::addr_of!((*_9));
_21 = [(-1_i8)];
_13.fld1 = _9;
(*_15) = 168279037377483142772986775453099053093_i128;
_5 = _17 as f64;
_17 = 355709708547280259_usize;
_1 = core::ptr::addr_of!((*_9));
(*_4) = (-124106832395321169235169051854937179036_i128);
Call(_22 = fn11(_12, _15, _13.fld1, _14, _13.fld0, _6, _7, _21, _2, _13.fld0, _19.0, _19.2, _16, _11, _2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_18 = _16.0 as f32;
_25.1 = _5 * _5;
_3 = _1;
_26 = !_23;
(*_15) = -80720600412181316690567906782494239791_i128;
_8 = core::ptr::addr_of!((*_15));
_16.1 = [_19.1,_19.1,_19.1,_19.1,_19.1,_19.1];
Goto(bb11)
}
bb11 = {
_25.0 = _19.0 ^ _19.0;
(*_8) = 119867482642760341660448725036544119806_i128 ^ (-92179582553532622322842168594972897689_i128);
_25.2 = _5 as u32;
_19.1 = !(-26972733779990526_i64);
_7 = 13038174518974613404_u64 << (*_8);
_4 = core::ptr::addr_of!((*_3));
_25.2 = _19.2 + _19.2;
_28 = _11;
_28 = _11;
_17 = !7364095071540651093_usize;
_11 = _28;
(*_4) = 126197335506415742861900205298225805134_i128 - (-77382564577486494722353337554296256766_i128);
(*_1) = !(-26757230613845339698248576054927500706_i128);
(*_1) = _16.0 as i128;
_9 = _15;
(*_3) = -(-46550579956398828942253467980872791339_i128);
_19 = (_25.0, 448571405325956213_i64, _25.2);
_21 = _6;
_19.2 = _25.2 * _25.2;
_12 = _25.1 > _5;
_26 = _17 as isize;
Goto(bb12)
}
bb12 = {
(*_9) = 150271570165356109279777366557032807906_i128 - (-57104918698981143436202654005626883269_i128);
_19 = (_12, 7127143759594530946_i64, _25.2);
_7 = _16.0 as u64;
_17 = 7_usize;
(*_15) = _19.2 as i128;
_25.0 = _19.0;
(*_8) = (-129465373138403081440070149339572291871_i128) & 33273691587447295697493459335470405346_i128;
(*_9) = _19.0 as i128;
_28 = _11;
_2 = core::ptr::addr_of!((*_1));
_10 = _28;
_19.2 = _25.2 & _25.2;
_16.0 = 206_u8;
Goto(bb13)
}
bb13 = {
_30 = (235620142607997216726365782324246245111_u128, _7);
_30 = (137252629804553534259522452780559635493_u128, _7);
_23 = !_26;
_3 = core::ptr::addr_of!(_22);
_29 = core::ptr::addr_of_mut!((*_15));
_1 = _8;
(*_9) = !29804817887484117074380156348038894995_i128;
(*_8) = -(-109582934931340957453381550230096098013_i128);
_30.0 = !123140371300489469751808226409475152648_u128;
(*_3) = 114309290195451469978646297433044512672_i128 - (-8365592379555584820779817728170927525_i128);
(*_2) = _25.2 as i128;
(*_3) = (-35145354750339991962645554922646258252_i128) & 84751255792784676025970385464242982748_i128;
_16.0 = !108_u8;
_2 = _14;
_4 = core::ptr::addr_of!((*_1));
_31 = !_19.0;
Goto(bb14)
}
bb14 = {
(*_29) = _5 as i128;
(*_3) = -(-106120722994900159725300781602540531223_i128);
_21 = [101_i8];
RET = -(-998323599_i32);
RET = 255961822_i32 << _19.2;
_33 = !_12;
_37 = _11 as i8;
(*_8) = 81110632485281966389771539517654199242_i128;
_22 = _25.1 as i128;
_6 = [_37];
_3 = _1;
_24 = [_19.1,_19.1,_19.1,_19.1,_19.1,_19.1];
(*_29) = 154586865733277356118498591650916039849_i128 >> _30.1;
_35 = !_19.0;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(10_usize, 35_usize, Move(_35), 16_usize, Move(_16), 31_usize, Move(_31), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(10_usize, 21_usize, Move(_21), 11_usize, Move(_11), 37_usize, Move(_37), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(10_usize, 7_usize, Move(_7), 40_usize, _40, 40_usize, _40, 40_usize, _40), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: bool,mut _2: *const i128,mut _3: *const i128,mut _4: *const i128,mut _5: [u16; 3],mut _6: [i8; 1],mut _7: u64,mut _8: [i8; 1],mut _9: *const i128,mut _10: [u16; 3],mut _11: bool,mut _12: u32,mut _13: (u8, [i64; 6]),mut _14: char,mut _15: *const i128) -> i128 {
mir! {
type RET = i128;
let _16: *const *const *mut u64;
let _17: [i64; 1];
let _18: (u8, [i64; 6]);
let _19: f64;
let _20: isize;
let _21: *const *mut u64;
let _22: isize;
let _23: Adt43;
let _24: Adt56;
let _25: u32;
let _26: (u64,);
let _27: f64;
let _28: bool;
let _29: [u16; 3];
let _30: f64;
let _31: usize;
let _32: Adt52;
let _33: isize;
let _34: isize;
let _35: Adt49;
let _36: char;
let _37: u64;
let _38: f32;
let _39: ();
let _40: ();
{
_4 = _9;
RET = !(-140771456372223286027027076847261902750_i128);
_13.0 = !7_u8;
_15 = _3;
_15 = core::ptr::addr_of!(RET);
_13.0 = 122_u8;
_9 = core::ptr::addr_of!(RET);
_13.0 = (-9223372036854775808_isize) as u8;
(*_9) = -(-81608280287645269935199117390148127552_i128);
_10 = [60092_u16,65156_u16,22244_u16];
_13.0 = _14 as u8;
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
14749649974064389523 => bb7,
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
_12 = 3682037451_u32;
_2 = core::ptr::addr_of!((*_15));
(*_9) = _14 as i128;
_10 = _5;
_13.1 = [(-9117774641629676549_i64),5428424463045947799_i64,3574603904722840029_i64,167076473857847705_i64,(-3774982522628952123_i64),2187814210193937426_i64];
_17 = [6229960175972624993_i64];
_13.1 = [(-437006420813138192_i64),(-8382247447242002890_i64),3786706040348109958_i64,4561842702082642139_i64,(-4172944047240593435_i64),1354073476371470269_i64];
_18 = (_13.0, _13.1);
_13.0 = (-6660_i16) as u8;
_20 = (-9223372036854775808_isize);
Goto(bb8)
}
bb8 = {
_10 = [13291_u16,11247_u16,43364_u16];
(*_15) = (-43204419400461838235815370837509039766_i128);
_12 = 4005697468_u32 & 2967104166_u32;
_6 = [30_i8];
_19 = _7 as f64;
_10 = [35573_u16,32448_u16,18010_u16];
_7 = !13683832906877905502_u64;
(*_9) = (-84340990982512040857983583316941032164_i128) - (-58787664700324676537869741700970094258_i128);
_18 = _13;
_12 = 1672917535821876298_i64 as u32;
_12 = (-8_i8) as u32;
Call(_18 = fn12(_2, (*_2), _2, _1, _9, (*_15), _20, (*_15), _13, _2, (*_2), _13, _13, _4, (*_2)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_22 = 18749_i16 as isize;
_23 = Adt43 { fld0: _10,fld1: _9 };
_18.1 = _13.1;
_13.0 = _7 as u8;
_23 = Adt43 { fld0: _10,fld1: _3 };
_1 = _11;
(*_2) = (-69514424193637408880345046692527936752_i128) << _20;
_12 = 2646802909_u32 & 2969422975_u32;
_25 = _12 << _12;
(*_2) = 122924535762346783697118548898284438722_i128;
_6 = [90_i8];
_23.fld1 = _4;
(*_2) = _22 as i128;
_26 = (_7,);
_30 = _19;
_29 = _10;
_26.0 = !_7;
match _20 {
0 => bb2,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
340282366920938463454151235394913435648 => bb16,
_ => bb15
}
}
bb10 = {
_10 = [13291_u16,11247_u16,43364_u16];
(*_15) = (-43204419400461838235815370837509039766_i128);
_12 = 4005697468_u32 & 2967104166_u32;
_6 = [30_i8];
_19 = _7 as f64;
_10 = [35573_u16,32448_u16,18010_u16];
_7 = !13683832906877905502_u64;
(*_9) = (-84340990982512040857983583316941032164_i128) - (-58787664700324676537869741700970094258_i128);
_18 = _13;
_12 = 1672917535821876298_i64 as u32;
_12 = (-8_i8) as u32;
Call(_18 = fn12(_2, (*_2), _2, _1, _9, (*_15), _20, (*_15), _13, _2, (*_2), _13, _13, _4, (*_2)), ReturnTo(bb9), UnwindUnreachable())
}
bb11 = {
_12 = 3682037451_u32;
_2 = core::ptr::addr_of!((*_15));
(*_9) = _14 as i128;
_10 = _5;
_13.1 = [(-9117774641629676549_i64),5428424463045947799_i64,3574603904722840029_i64,167076473857847705_i64,(-3774982522628952123_i64),2187814210193937426_i64];
_17 = [6229960175972624993_i64];
_13.1 = [(-437006420813138192_i64),(-8382247447242002890_i64),3786706040348109958_i64,4561842702082642139_i64,(-4172944047240593435_i64),1354073476371470269_i64];
_18 = (_13.0, _13.1);
_13.0 = (-6660_i16) as u8;
_20 = (-9223372036854775808_isize);
Goto(bb8)
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
_2 = core::ptr::addr_of!((*_2));
_18.1 = _13.1;
_28 = _11 | _1;
_26.0 = _7 << _25;
_26 = (_7,);
RET = 1824330344_i32 as i128;
_6 = [(-46_i8)];
_1 = _28;
(*_9) = 87090358477943098381962941812362241764_i128;
_18.0 = _13.0 * _13.0;
_31 = 55754_u16 as usize;
_11 = !_28;
(*_9) = 78175682368123749330170364287805826201_i128 << _12;
_16 = core::ptr::addr_of!(_21);
_13 = (_18.0, _18.1);
_12 = _25 * _25;
(*_9) = -(-26689006126370122368447748243829431841_i128);
_25 = _18.0 as u32;
_36 = _14;
_12 = _25 * _25;
_38 = (-4964_i16) as f32;
Goto(bb17)
}
bb17 = {
Call(_39 = dump_var(11_usize, 29_usize, Move(_29), 25_usize, Move(_25), 13_usize, Move(_13), 10_usize, Move(_10)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_39 = dump_var(11_usize, 28_usize, Move(_28), 31_usize, Move(_31), 12_usize, Move(_12), 5_usize, Move(_5)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_39 = dump_var(11_usize, 1_usize, Move(_1), 6_usize, Move(_6), 40_usize, _40, 40_usize, _40), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: *const i128,mut _2: i128,mut _3: *const i128,mut _4: bool,mut _5: *const i128,mut _6: i128,mut _7: isize,mut _8: i128,mut _9: (u8, [i64; 6]),mut _10: *const i128,mut _11: i128,mut _12: (u8, [i64; 6]),mut _13: (u8, [i64; 6]),mut _14: *const i128,mut _15: i128) -> (u8, [i64; 6]) {
mir! {
type RET = (u8, [i64; 6]);
let _16: char;
let _17: isize;
let _18: i8;
let _19: f64;
let _20: (u64,);
let _21: f32;
let _22: Adt44;
let _23: (u64,);
let _24: i32;
let _25: char;
let _26: i128;
let _27: f64;
let _28: [i64; 6];
let _29: char;
let _30: i128;
let _31: [i64; 6];
let _32: [i128; 7];
let _33: f64;
let _34: f64;
let _35: i8;
let _36: f64;
let _37: (u64,);
let _38: f32;
let _39: ();
let _40: ();
{
(*_3) = -_2;
(*_10) = _9.0 as i128;
RET = (_9.0, _13.1);
_5 = _3;
(*_5) = _9.0 as i128;
_9 = (RET.0, RET.1);
_16 = '\u{96ca5}';
RET.0 = !_9.0;
Call((*_5) = fn13(_4, _11, _13, RET, _13.1, _13.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_3) = _6;
_10 = _1;
RET.1 = [8539354359668960301_i64,2950303751887809599_i64,1471378302454323047_i64,977435513705662153_i64,(-5535311702346011144_i64),(-4166766362640722344_i64)];
_7 = 60_isize >> _11;
(*_3) = !_6;
RET = (_12.0, _9.1);
_18 = 107_i8 - 114_i8;
Goto(bb2)
}
bb2 = {
RET.0 = !_9.0;
RET.1 = _9.1;
_16 = '\u{51c0}';
_10 = _1;
_2 = -_6;
RET.1 = [5026717818917195511_i64,5027708865844135857_i64,(-1138542115786584292_i64),(-950954516633108994_i64),(-5147593219173539395_i64),(-7426746290929838932_i64)];
RET = (_9.0, _12.1);
Goto(bb3)
}
bb3 = {
RET = _12;
_16 = '\u{3795e}';
_12 = RET;
_19 = 242842198580410782529332072143317388834_u128 as f64;
_20.0 = 271644214967609905440919735695518495119_u128 as u64;
RET = (_12.0, _13.1);
RET.0 = !_9.0;
_3 = _10;
_1 = _3;
_2 = (*_3) >> _12.0;
_12.0 = RET.0 >> (*_1);
(*_3) = _11;
(*_10) = _2;
_18 = -106_i8;
_6 = (*_5);
Goto(bb4)
}
bb4 = {
_12.1 = [(-3674837472116304830_i64),(-5393258214577389926_i64),(-4248087021462203787_i64),8795669315152823765_i64,4815145749906136860_i64,4364831529387000169_i64];
(*_5) = !_11;
_20 = (9025376790248412453_u64,);
_8 = _6;
_12 = (RET.0, RET.1);
RET.1 = _9.1;
_18 = 2_usize as i8;
(*_5) = _8 ^ _6;
_17 = -_7;
_9 = RET;
(*_10) = _2;
_12.1 = _13.1;
_20 = (7409948609158894294_u64,);
Call(_8 = core::intrinsics::bswap((*_1)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_23.0 = _20.0 / _20.0;
RET = (_9.0, _13.1);
_9.1 = _13.1;
(*_10) = _6;
_12 = _13;
_14 = _3;
_7 = -_17;
(*_1) = _15;
_18 = -53_i8;
Goto(bb6)
}
bb6 = {
RET = (_12.0, _12.1);
_19 = _20.0 as f64;
Goto(bb7)
}
bb7 = {
_3 = core::ptr::addr_of!((*_10));
_13 = (_9.0, RET.1);
RET.1 = [(-1409429426996885860_i64),8004241671421225295_i64,(-7623643730628490661_i64),(-2627532016847149850_i64),1451905676735026119_i64,1631749405587169108_i64];
_13.0 = _9.0 * _12.0;
_16 = '\u{35202}';
_8 = !(*_1);
_20 = _23;
(*_1) = !_15;
_12.1 = _9.1;
_6 = (*_14) & _11;
_6 = (*_1);
_12.1 = [(-4031670331499635659_i64),(-8335184346832541897_i64),2011577271674808223_i64,1660245224498919078_i64,(-4803478222183883045_i64),(-6952592384736417442_i64)];
_6 = (*_3) >> (*_3);
(*_3) = -_6;
_10 = core::ptr::addr_of!(_2);
_15 = _11 << _7;
_18 = 9_i8 + (-48_i8);
RET = (_9.0, _13.1);
Call(_14 = core::intrinsics::arith_offset(_1, (-9223372036854775808_isize)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_25 = _16;
_20 = _23;
_10 = _14;
_27 = _19 * _19;
RET.1 = _12.1;
_26 = (*_1) - _15;
_19 = _27 + _27;
_23.0 = !_20.0;
_23.0 = !_20.0;
_15 = (*_5);
_2 = 112650202078761539315205707479217589570_u128 as i128;
_21 = 7_usize as f32;
_11 = !_26;
_12 = (RET.0, _13.1);
_6 = _4 as i128;
_28 = [(-7506309628438499123_i64),1994454622808991407_i64,(-3167334367967235531_i64),3976485280688824877_i64,71256456010132779_i64,1079463555777024482_i64];
_23 = _20;
RET.1 = [(-7471873677900871839_i64),(-5147600877426223194_i64),(-7812523340813810533_i64),(-8110322748824849125_i64),6526097367698233435_i64,(-3258265580676086697_i64)];
_17 = _7 + _7;
_20 = _23;
Goto(bb9)
}
bb9 = {
_19 = _27 + _27;
_25 = _16;
_13.1 = RET.1;
_30 = _20.0 as i128;
_24 = 10458_i16 as i32;
RET.1 = [(-6247046111635697724_i64),3347344080532386199_i64,3090332283828961332_i64,(-7969790216236592646_i64),2383214883675596914_i64,(-7783128868988844085_i64)];
_20.0 = !_23.0;
_30 = _26;
_31 = _13.1;
_2 = (*_3) << (*_1);
_18 = (-21_i8);
Goto(bb10)
}
bb10 = {
_27 = _19 - _19;
(*_1) = -_2;
_12.0 = !_9.0;
_13.1 = [(-7403857166741830741_i64),6645761822932693709_i64,9046174797688640209_i64,(-2347734230711892640_i64),8147348865265583184_i64,(-2034771304649626315_i64)];
_12.1 = [5160234855321694178_i64,(-5932872684323350783_i64),(-592954581027572120_i64),8378018467988303585_i64,(-7576403978765572662_i64),4164308103739150164_i64];
_7 = _18 as isize;
(*_5) = _25 as i128;
_15 = _6;
_12.0 = _4 as u8;
_16 = _25;
_4 = true;
_9.0 = _12.0;
_12 = (_9.0, RET.1);
_5 = core::ptr::addr_of!((*_5));
_8 = _26;
match _18 {
0 => bb8,
340282366920938463463374607431768211435 => bb12,
_ => bb11
}
}
bb11 = {
_23.0 = _20.0 / _20.0;
RET = (_9.0, _13.1);
_9.1 = _13.1;
(*_10) = _6;
_12 = _13;
_14 = _3;
_7 = -_17;
(*_1) = _15;
_18 = -53_i8;
Goto(bb6)
}
bb12 = {
_21 = (-2592_i16) as f32;
RET.1 = _13.1;
_32 = [_11,_26,_30,_8,(*_3),_11,_11];
_15 = -_2;
_27 = _19;
_33 = _19 + _27;
(*_3) = !_2;
_18 = !61_i8;
_15 = _21 as i128;
(*_5) = _11;
_9.0 = _13.0;
Goto(bb13)
}
bb13 = {
_16 = _25;
_36 = -_19;
RET = _12;
_17 = _7;
RET.0 = !_9.0;
(*_1) = _2 | _11;
_12.1 = _28;
(*_3) = _13.0 as i128;
RET.0 = !_9.0;
_7 = _17;
_25 = _16;
RET.0 = !_9.0;
_20.0 = !_23.0;
_25 = _16;
_11 = -_30;
_2 = _8;
RET = _13;
_37 = (_20.0,);
_33 = -_36;
_16 = _25;
Goto(bb14)
}
bb14 = {
_27 = _33;
_36 = _19;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(12_usize, 2_usize, Move(_2), 23_usize, Move(_23), 28_usize, Move(_28), 32_usize, Move(_32)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(12_usize, 24_usize, Move(_24), 11_usize, Move(_11), 4_usize, Move(_4), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(12_usize, 9_usize, Move(_9), 31_usize, Move(_31), 26_usize, Move(_26), 40_usize, _40), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: bool,mut _2: i128,mut _3: (u8, [i64; 6]),mut _4: (u8, [i64; 6]),mut _5: [i64; 6],mut _6: [i64; 6]) -> i128 {
mir! {
type RET = i128;
let _7: (i8, i8, char);
let _8: char;
let _9: bool;
let _10: i64;
let _11: *mut i128;
let _12: *const u128;
let _13: [u128; 4];
let _14: i128;
let _15: *mut *const *mut u64;
let _16: *const i128;
let _17: [i8; 1];
let _18: [i128; 7];
let _19: *mut u64;
let _20: isize;
let _21: u16;
let _22: [i128; 7];
let _23: (u16,);
let _24: char;
let _25: Adt48;
let _26: u64;
let _27: (i8, i8, char);
let _28: u8;
let _29: Adt44;
let _30: ();
let _31: ();
{
_6 = [2962058671534671490_i64,(-7293238604458961143_i64),(-3797338142357373165_i64),3842469448541236430_i64,(-3091238065017555816_i64),(-2623805004468135133_i64)];
RET = 1681_u16 as i128;
_7.0 = 7778847762331244467901838081249570371_u128 as i8;
Goto(bb1)
}
bb1 = {
_7.1 = _7.0 | _7.0;
_5 = [6566344030529222359_i64,(-3002379418798860936_i64),(-3006825151872983192_i64),5442455187763800490_i64,(-8499571821129191045_i64),2150500223126715169_i64];
_7 = ((-123_i8), (-121_i8), '\u{109f43}');
_3 = _4;
_4.0 = _3.0;
RET = _2;
_9 = !_1;
_8 = _7.2;
_6 = [5477674501557515489_i64,(-6351677537384488155_i64),(-2316610595800079155_i64),7566252104843562667_i64,2911528749946182918_i64,(-669639275056555457_i64)];
_4.0 = !_3.0;
_1 = !_9;
_10 = 6309603736045209510_i64;
_4.1 = [_10,_10,_10,_10,_10,_10];
_1 = !_9;
_3.1 = [_10,_10,_10,_10,_10,_10];
_4.1 = [_10,_10,_10,_10,_10,_10];
_4.0 = !_3.0;
_7.2 = _8;
RET = 12928_u16 as i128;
_5 = [_10,_10,_10,_10,_10,_10];
Goto(bb2)
}
bb2 = {
_7 = ((-80_i8), (-97_i8), _8);
_7 = ((-53_i8), 47_i8, _8);
_9 = _7.1 >= _7.1;
RET = _10 as i128;
_1 = _7.0 != _7.1;
_4.1 = [_10,_10,_10,_10,_10,_10];
_7 = ((-44_i8), (-77_i8), _8);
_7.1 = _7.0;
Goto(bb3)
}
bb3 = {
_3 = (_4.0, _6);
Goto(bb4)
}
bb4 = {
_11 = core::ptr::addr_of_mut!(_14);
_7.0 = _7.2 as i8;
_4.1 = [_10,_10,_10,_10,_10,_10];
_6 = [_10,_10,_10,_10,_10,_10];
Goto(bb5)
}
bb5 = {
RET = _2 << _7.0;
match _7.1 {
0 => bb1,
1 => bb6,
2 => bb7,
340282366920938463463374607431768211412 => bb9,
_ => bb8
}
}
bb6 = {
_11 = core::ptr::addr_of_mut!(_14);
_7.0 = _7.2 as i8;
_4.1 = [_10,_10,_10,_10,_10,_10];
_6 = [_10,_10,_10,_10,_10,_10];
Goto(bb5)
}
bb7 = {
_3 = (_4.0, _6);
Goto(bb4)
}
bb8 = {
_7 = ((-80_i8), (-97_i8), _8);
_7 = ((-53_i8), 47_i8, _8);
_9 = _7.1 >= _7.1;
RET = _10 as i128;
_1 = _7.0 != _7.1;
_4.1 = [_10,_10,_10,_10,_10,_10];
_7 = ((-44_i8), (-77_i8), _8);
_7.1 = _7.0;
Goto(bb3)
}
bb9 = {
_1 = !_9;
_7.2 = _8;
_10 = RET as i64;
RET = _2;
_6 = [_10,_10,_10,_10,_10,_10];
_7.1 = _10 as i8;
_2 = !RET;
_5 = [_10,_10,_10,_10,_10,_10];
_13 = [459415125371258503508901275691634545_u128,185430092100540893949638440760261288135_u128,148343406397825525425364141204136766193_u128,326677681989793972675249138068699283667_u128];
(*_11) = _2 + RET;
_13 = [205468571410927634675372808304398739742_u128,44966143341127220409856304890115707403_u128,317028526188779393361690730893041182936_u128,320303903291548747529920862569152130942_u128];
_7 = (78_i8, (-27_i8), _8);
(*_11) = _4.0 as i128;
Goto(bb10)
}
bb10 = {
match _7.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607431768211429 => bb12,
_ => bb11
}
}
bb11 = {
_7 = ((-80_i8), (-97_i8), _8);
_7 = ((-53_i8), 47_i8, _8);
_9 = _7.1 >= _7.1;
RET = _10 as i128;
_1 = _7.0 != _7.1;
_4.1 = [_10,_10,_10,_10,_10,_10];
_7 = ((-44_i8), (-77_i8), _8);
_7.1 = _7.0;
Goto(bb3)
}
bb12 = {
_17 = [_7.0];
_13 = [306367891416368343968658800709928367672_u128,17268485485983563230995229169010550597_u128,339987548494855589281445755703351613188_u128,279378205159766631494724108851761539826_u128];
_20 = 9223372036854775807_isize + (-9223372036854775808_isize);
match _7.0 {
0 => bb1,
1 => bb13,
2 => bb14,
3 => bb15,
78 => bb17,
_ => bb16
}
}
bb13 = {
_7 = ((-80_i8), (-97_i8), _8);
_7 = ((-53_i8), 47_i8, _8);
_9 = _7.1 >= _7.1;
RET = _10 as i128;
_1 = _7.0 != _7.1;
_4.1 = [_10,_10,_10,_10,_10,_10];
_7 = ((-44_i8), (-77_i8), _8);
_7.1 = _7.0;
Goto(bb3)
}
bb14 = {
_3 = (_4.0, _6);
Goto(bb4)
}
bb15 = {
_1 = !_9;
_7.2 = _8;
_10 = RET as i64;
RET = _2;
_6 = [_10,_10,_10,_10,_10,_10];
_7.1 = _10 as i8;
_2 = !RET;
_5 = [_10,_10,_10,_10,_10,_10];
_13 = [459415125371258503508901275691634545_u128,185430092100540893949638440760261288135_u128,148343406397825525425364141204136766193_u128,326677681989793972675249138068699283667_u128];
(*_11) = _2 + RET;
_13 = [205468571410927634675372808304398739742_u128,44966143341127220409856304890115707403_u128,317028526188779393361690730893041182936_u128,320303903291548747529920862569152130942_u128];
_7 = (78_i8, (-27_i8), _8);
(*_11) = _4.0 as i128;
Goto(bb10)
}
bb16 = {
_7 = ((-80_i8), (-97_i8), _8);
_7 = ((-53_i8), 47_i8, _8);
_9 = _7.1 >= _7.1;
RET = _10 as i128;
_1 = _7.0 != _7.1;
_4.1 = [_10,_10,_10,_10,_10,_10];
_7 = ((-44_i8), (-77_i8), _8);
_7.1 = _7.0;
Goto(bb3)
}
bb17 = {
_7.1 = _7.2 as i8;
_8 = _7.2;
_23 = (42948_u16,);
_26 = _7.2 as u64;
_10 = _23.0 as i64;
_14 = (-2069_i16) as i128;
_27.0 = _7.0 & _7.0;
_27.2 = _8;
(*_11) = (-538769033_i32) as i128;
_27 = (_7.0, _7.0, _8);
RET = _14;
RET = -_14;
_7.0 = _7.1 << _27.0;
(*_11) = RET + _2;
Goto(bb18)
}
bb18 = {
Call(_30 = dump_var(13_usize, 26_usize, Move(_26), 27_usize, Move(_27), 13_usize, Move(_13), 4_usize, Move(_4)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_30 = dump_var(13_usize, 3_usize, Move(_3), 17_usize, Move(_17), 10_usize, Move(_10), 6_usize, Move(_6)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: *const i128,mut _2: *const i128,mut _3: *const i128,mut _4: *const i128,mut _5: i128,mut _6: *const i128,mut _7: i128,mut _8: *const i128,mut _9: i128,mut _10: u64) -> bool {
mir! {
type RET = bool;
let _11: bool;
let _12: isize;
let _13: (bool, i64, u32);
let _14: Adt48;
let _15: usize;
let _16: [u128; 4];
let _17: Adt51;
let _18: isize;
let _19: [i8; 1];
let _20: isize;
let _21: u128;
let _22: f32;
let _23: i8;
let _24: f32;
let _25: char;
let _26: (bool, i64, u32);
let _27: (i8, i8, char);
let _28: (u64,);
let _29: char;
let _30: i16;
let _31: Adt58;
let _32: usize;
let _33: *const i128;
let _34: Adt47;
let _35: (bool, i64, u32);
let _36: ();
let _37: ();
{
_9 = 18310062505907742801_usize as i128;
_3 = _6;
_11 = !false;
(*_3) = _5;
_9 = (*_4) - (*_8);
_13.2 = 29741_u16 as u32;
RET = _11;
_9 = 92039411324889159100217796991663652225_u128 as i128;
_9 = (*_3);
(*_8) = _9;
_13.0 = _11;
_8 = _6;
RET = _13.0;
_12 = (-9223372036854775808_isize) << (*_4);
(*_6) = _13.2 as i128;
_13.2 = 1244143064_u32;
_13 = (RET, (-8480897209405749581_i64), 4052389895_u32);
_12 = -(-9223372036854775808_isize);
_5 = -_7;
_2 = core::ptr::addr_of!((*_8));
(*_8) = 46678790598011603080614810087110578756_u128 as i128;
Goto(bb1)
}
bb1 = {
_9 = 152506360835011706374204826765771746347_u128 as i128;
_12 = (-9223372036854775808_isize);
_5 = -(*_6);
(*_2) = !_5;
(*_4) = 282530615420320481422980647265100822957_u128 as i128;
_15 = 6_usize;
(*_6) = _9;
_13.0 = RET;
RET = !_11;
_16 = [63990394117825653127865142530039386895_u128,141998395713249629162365491906174945654_u128,49636095132413637011627273041607773302_u128,172327705394908094918355712018396881148_u128];
RET = _13.0;
(*_6) = !_7;
(*_6) = _9 - _9;
(*_3) = (-103_i8) as i128;
(*_6) = _7;
(*_3) = _7;
_9 = (*_4);
_13.2 = 3628556120_u32 + 706945113_u32;
_2 = core::ptr::addr_of!(_9);
(*_2) = (*_4);
_13.0 = (*_6) == (*_2);
(*_6) = (*_2) + _7;
(*_4) = !(*_2);
_5 = !(*_3);
RET = !_13.0;
Call((*_2) = core::intrinsics::bswap((*_3)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_18 = 20201_u16 as isize;
(*_8) = _9;
_4 = _8;
(*_2) = _13.2 as i128;
_19 = [123_i8];
_21 = 65321012317042633334469263273298359859_u128;
_21 = 46596688380840708915053531623255144591_u128;
_13.2 = 769166468_u32;
_2 = _4;
(*_3) = _5;
_19 = [45_i8];
_22 = _13.2 as f32;
_16 = [_21,_21,_21,_21];
Goto(bb3)
}
bb3 = {
_2 = _8;
_8 = core::ptr::addr_of!(_7);
_3 = core::ptr::addr_of!((*_3));
_16 = [_21,_21,_21,_21];
_18 = _12 >> _5;
(*_4) = _5 * (*_8);
_19 = [(-119_i8)];
RET = !_13.0;
RET = !_13.0;
(*_8) = '\u{6f09}' as i128;
match _13.1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463454893710222362461875 => bb8,
_ => bb7
}
}
bb4 = {
_18 = 20201_u16 as isize;
(*_8) = _9;
_4 = _8;
(*_2) = _13.2 as i128;
_19 = [123_i8];
_21 = 65321012317042633334469263273298359859_u128;
_21 = 46596688380840708915053531623255144591_u128;
_13.2 = 769166468_u32;
_2 = _4;
(*_3) = _5;
_19 = [45_i8];
_22 = _13.2 as f32;
_16 = [_21,_21,_21,_21];
Goto(bb3)
}
bb5 = {
_9 = 152506360835011706374204826765771746347_u128 as i128;
_12 = (-9223372036854775808_isize);
_5 = -(*_6);
(*_2) = !_5;
(*_4) = 282530615420320481422980647265100822957_u128 as i128;
_15 = 6_usize;
(*_6) = _9;
_13.0 = RET;
RET = !_11;
_16 = [63990394117825653127865142530039386895_u128,141998395713249629162365491906174945654_u128,49636095132413637011627273041607773302_u128,172327705394908094918355712018396881148_u128];
RET = _13.0;
(*_6) = !_7;
(*_6) = _9 - _9;
(*_3) = (-103_i8) as i128;
(*_6) = _7;
(*_3) = _7;
_9 = (*_4);
_13.2 = 3628556120_u32 + 706945113_u32;
_2 = core::ptr::addr_of!(_9);
(*_2) = (*_4);
_13.0 = (*_6) == (*_2);
(*_6) = (*_2) + _7;
(*_4) = !(*_2);
_5 = !(*_3);
RET = !_13.0;
Call((*_2) = core::intrinsics::bswap((*_3)), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_13 = (RET, 4034559774437713949_i64, 217740625_u32);
_22 = _10 as f32;
_3 = _6;
_23 = (-67_i8) | 112_i8;
Call(_18 = core::intrinsics::transmute(_13.1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_21 = _13.1 as u128;
_19 = [_23];
_12 = !_18;
(*_2) = !_5;
RET = _13.0;
_2 = core::ptr::addr_of!(_9);
(*_6) = _7;
_24 = _22 + _22;
(*_8) = _5 + (*_3);
_13 = (RET, 3340061904029255596_i64, 4265095533_u32);
_2 = core::ptr::addr_of!(_7);
match _13.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
4265095533 => bb10,
_ => bb8
}
}
bb10 = {
(*_8) = !(*_6);
(*_2) = _15 as i128;
_2 = _6;
_20 = -_12;
_2 = _3;
_1 = core::ptr::addr_of!((*_6));
_13.0 = _18 < _12;
(*_6) = _9 >> _20;
_21 = !205695067056765652193387537598208794864_u128;
(*_3) = 26785_u16 as i128;
_9 = !_5;
_6 = core::ptr::addr_of!(_9);
_25 = '\u{71b4f}';
_18 = _13.0 as isize;
_13 = (RET, 8494701000851724367_i64, 492651167_u32);
_13.1 = 1630218452243082715_i64;
(*_6) = -(*_1);
_20 = !_18;
(*_8) = (*_2) * _9;
_13.0 = RET;
_7 = (*_2) >> _18;
_3 = _4;
(*_2) = _7;
(*_2) = -(*_8);
_22 = -_24;
_8 = _1;
Call((*_2) = core::intrinsics::bswap(_7), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_10 = 13059218722187606452_u64;
_9 = !(*_4);
(*_1) = -_7;
(*_8) = !_7;
_25 = '\u{5c5dd}';
_3 = core::ptr::addr_of!((*_8));
_26.1 = !_13.1;
RET = _13.0;
_27.0 = _23 << (*_8);
_27.1 = _27.0 ^ _27.0;
_28 = (_10,);
_9 = _15 as i128;
_26 = _13;
_16 = [_21,_21,_21,_21];
_26 = (RET, _13.1, _13.2);
(*_3) = _5 << _27.1;
(*_6) = (*_1) - (*_2);
_13 = (_26.0, _26.1, _26.2);
_27.2 = _25;
_8 = core::ptr::addr_of!((*_2));
_23 = _28.0 as i8;
_13 = _26;
_13.2 = _26.2;
(*_2) = _7;
match _26.2 {
0 => bb5,
1 => bb10,
2 => bb12,
3 => bb13,
4 => bb14,
492651167 => bb16,
_ => bb15
}
}
bb12 = {
_9 = 152506360835011706374204826765771746347_u128 as i128;
_12 = (-9223372036854775808_isize);
_5 = -(*_6);
(*_2) = !_5;
(*_4) = 282530615420320481422980647265100822957_u128 as i128;
_15 = 6_usize;
(*_6) = _9;
_13.0 = RET;
RET = !_11;
_16 = [63990394117825653127865142530039386895_u128,141998395713249629162365491906174945654_u128,49636095132413637011627273041607773302_u128,172327705394908094918355712018396881148_u128];
RET = _13.0;
(*_6) = !_7;
(*_6) = _9 - _9;
(*_3) = (-103_i8) as i128;
(*_6) = _7;
(*_3) = _7;
_9 = (*_4);
_13.2 = 3628556120_u32 + 706945113_u32;
_2 = core::ptr::addr_of!(_9);
(*_2) = (*_4);
_13.0 = (*_6) == (*_2);
(*_6) = (*_2) + _7;
(*_4) = !(*_2);
_5 = !(*_3);
RET = !_13.0;
Call((*_2) = core::intrinsics::bswap((*_3)), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_18 = 20201_u16 as isize;
(*_8) = _9;
_4 = _8;
(*_2) = _13.2 as i128;
_19 = [123_i8];
_21 = 65321012317042633334469263273298359859_u128;
_21 = 46596688380840708915053531623255144591_u128;
_13.2 = 769166468_u32;
_2 = _4;
(*_3) = _5;
_19 = [45_i8];
_22 = _13.2 as f32;
_16 = [_21,_21,_21,_21];
Goto(bb3)
}
bb14 = {
_2 = _8;
_8 = core::ptr::addr_of!(_7);
_3 = core::ptr::addr_of!((*_3));
_16 = [_21,_21,_21,_21];
_18 = _12 >> _5;
(*_4) = _5 * (*_8);
_19 = [(-119_i8)];
RET = !_13.0;
RET = !_13.0;
(*_8) = '\u{6f09}' as i128;
match _13.1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463454893710222362461875 => bb8,
_ => bb7
}
}
bb15 = {
Return()
}
bb16 = {
_13.1 = _26.1 | _26.1;
_1 = _6;
(*_1) = (*_3);
_4 = _8;
_16 = [_21,_21,_21,_21];
_26 = (_13.0, _13.1, _13.2);
RET = _13.0 ^ _13.0;
_32 = !_15;
_29 = _27.2;
_20 = _18;
_4 = core::ptr::addr_of!((*_2));
_27.2 = _25;
_11 = RET ^ _26.0;
_33 = core::ptr::addr_of!((*_1));
Goto(bb17)
}
bb17 = {
Call(_36 = dump_var(14_usize, 32_usize, Move(_32), 29_usize, Move(_29), 25_usize, Move(_25), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_36 = dump_var(14_usize, 19_usize, Move(_19), 10_usize, Move(_10), 21_usize, Move(_21), 18_usize, Move(_18)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_36 = dump_var(14_usize, 28_usize, Move(_28), 20_usize, Move(_20), 37_usize, _37, 37_usize, _37), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{7c8c2}'), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(57_i8), std::hint::black_box((-13525_i16)), std::hint::black_box(393537654_i32), std::hint::black_box((-738519098595690359_i64)), std::hint::black_box((-45473205664956123313396705760833483548_i128)), std::hint::black_box(1471448312308456521_usize), std::hint::black_box(12_u8), std::hint::black_box(30755_u16), std::hint::black_box(5169538454457655642_u64));
                
            }
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: *const i128,

},
Variant1{
fld0: bool,
fld1: u64,
fld2: f32,
fld3: *const i128,
fld4: [i8; 1],
fld5: i32,
fld6: (u128, u64),
fld7: [i64; 1],

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt43{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt43 {
fld0: [u16; 3],
fld1: *const i128,
}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: [i8; 1],
fld1: [u16; 3],
fld2: Adt42,

},
Variant1{
fld0: i16,
fld1: char,
fld2: i128,
fld3: *const *mut u64,

},
Variant2{
fld0: *mut i128,
fld1: (f32, *const u128, *mut u64),
fld2: [u128; 4],

},
Variant3{
fld0: bool,
fld1: *const *const *mut u64,
fld2: (u16,),
fld3: [u16; 3],
fld4: [i64; 6],
fld5: i32,
fld6: usize,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: [u16; 3],
fld1: i128,
fld2: u8,
fld3: *const *mut u64,
fld4: u64,
fld5: (i8, i8, char),

},
Variant1{
fld0: f64,
fld1: (bool, f64, u32),

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: *mut (bool, i64, u32),

},
Variant1{
fld0: f64,
fld1: i128,
fld2: [i128; 7],
fld3: usize,

},
Variant2{
fld0: (u8, [i64; 6]),
fld1: (i8, i8, char),
fld2: *const i128,
fld3: u64,
fld4: u8,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: (bool, i64, u32),
fld1: i16,

},
Variant1{
fld0: i128,
fld1: (u64,),
fld2: *mut *const *mut u64,

},
Variant2{
fld0: u16,
fld1: ([u16; 3], *const i128, [i128; 7], *const *const *mut u64, usize),
fld2: *const i128,
fld3: Adt42,
fld4: u128,

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
fld0: u128,
fld1: Adt43,
fld2: f32,
fld3: i8,
fld4: Adt44,
fld5: u32,

},
Variant1{
fld0: i32,
fld1: i128,
fld2: (u64,),
fld3: *const *mut u64,
fld4: *mut (bool, i64, u32),

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: [i8; 1],
fld1: f64,
fld2: [u128; 4],
fld3: u128,
fld4: [i64; 6],
fld5: (i8, i8, char),

},
Variant1{
fld0: (bool, i64, u32),
fld1: u8,
fld2: u64,
fld3: *const *mut u64,
fld4: usize,
fld5: (u128, u64),
fld6: Adt43,
fld7: [u32; 6],

},
Variant2{
fld0: [i128; 7],
fld1: (u64,),

},
Variant3{
fld0: f32,
fld1: *mut (bool, i64, u32),
fld2: Adt48,
fld3: [u32; 6],

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: [i8; 1],
fld1: [u16; 3],
fld2: Adt43,
fld3: f64,
fld4: (i8, i8, char),
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: f32,
fld1: (u64,),
fld2: *mut (bool, i64, u32),
fld3: i8,
fld4: Adt42,
fld5: *const *const *mut u64,

},
Variant1{
fld0: f32,
fld1: u64,
fld2: Adt48,
fld3: *const *mut u64,
fld4: u16,
fld5: (u16,),
fld6: u128,
fld7: (bool, f64, u32),

},
Variant2{
fld0: bool,
fld1: Adt42,
fld2: Adt44,
fld3: i8,
fld4: i16,
fld5: (u16,),

},
Variant3{
fld0: i128,
fld1: [i64; 1],
fld2: (u16,),
fld3: u16,
fld4: *const *mut u64,
fld5: u64,
fld6: usize,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: u32,
fld1: i64,
fld2: *mut *const *mut u64,
fld3: usize,
fld4: u8,

},
Variant1{
fld0: Adt45,
fld1: (u8, [i64; 6]),

},
Variant2{
fld0: bool,
fld1: [i64; 6],
fld2: [i128; 7],
fld3: u32,

},
Variant3{
fld0: Adt42,
fld1: (bool, f64, u32),
fld2: (u16,),

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: Adt44,
fld1: char,
fld2: u64,
fld3: (bool, f64, u32),
fld4: i16,
}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: [u128; 4],
fld1: *mut *const *mut u64,
fld2: i32,
fld3: u16,

},
Variant1{
fld0: *const u128,
fld1: [u32; 6],
fld2: *mut u64,
fld3: Adt50,
fld4: i16,
fld5: *mut *const *mut u64,
fld6: u16,

},
Variant2{
fld0: Adt46,
fld1: Adt44,

},
Variant3{
fld0: u32,
fld1: [i64; 6],
fld2: Adt50,
fld3: f64,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: *mut *const *mut u64,
fld1: u16,
fld2: *const u128,
fld3: usize,
fld4: Adt49,
fld5: *const *const *mut u64,

},
Variant1{
fld0: *mut u64,
fld1: *mut i128,
fld2: [i8; 1],
fld3: Adt49,
fld4: i16,
fld5: (bool, i64, u32),
fld6: (u64,),

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: *mut u64,
fld1: Adt54,
fld2: Adt45,
fld3: *const u128,

},
Variant1{
fld0: Adt42,
fld1: i16,
fld2: i64,
fld3: Adt55,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: u8,
fld1: *mut (bool, i64, u32),
fld2: Adt49,
fld3: u32,
fld4: (i8, i8, char),
fld5: *mut *const *mut u64,
fld6: i64,

},
Variant1{
fld0: (bool, f64, u32),
fld1: Adt50,
fld2: Adt47,
fld3: [u16; 3],
fld4: (i8, i8, char),
fld5: i32,

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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: u32,
fld1: (u128, u64),
fld2: Adt44,
fld3: Adt45,

},
Variant1{
fld0: Adt51,
fld1: [i64; 1],
fld2: (bool, i64, u32),

},
Variant2{
fld0: *mut i128,
fld1: char,
fld2: Adt52,
fld3: Adt49,
fld4: Adt47,

},
Variant3{
fld0: *const u128,
fld1: (u8, [i64; 6]),
fld2: i16,

}}

