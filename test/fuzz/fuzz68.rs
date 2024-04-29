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
pub fn fn0(mut _1: bool,mut _2: u128,mut _3: u8,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: u64) -> usize {
mir! {
type RET = usize;
let _8: isize;
let _9: [i128; 2];
let _10: u128;
let _11: f32;
let _12: usize;
let _13: [i32; 8];
let _14: [char; 4];
let _15: (i16, char, [u16; 3], i64);
let _16: char;
let _17: (usize,);
let _18: ();
let _19: ();
{
RET = 5_usize ^ 6_usize;
_1 = RET < RET;
_6 = !(-1159664875_i32);
Goto(bb1)
}
bb1 = {
_7 = !12220153973483360686_u64;
RET = 16164910457979011365_usize * 1_usize;
_8 = 9223372036854775807_isize * 102_isize;
RET = 3_usize;
_7 = 1783327840916698156_u64;
_2 = 323799754865381959444955047818340384153_u128 & 251055964170954599401614913695699768250_u128;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
4 => bb6,
3 => bb8,
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
_8 = 9223372036854775807_isize;
_2 = 239111597290562380370770315593020527632_u128 | 77226963931746790958647358251532452212_u128;
_13 = [_6,_6,_6,_6,_6,_6,_6,_6];
_9 = [47200336828278034054218309048455333294_i128,57036287499077537780290005937450754592_i128];
_3 = 187_u8;
RET = 4459325733217177270_usize >> _6;
_12 = _8 as usize;
_3 = !246_u8;
_4 = _2 as i8;
Call(_8 = fn1(_1, _1, _7, _3, _1, _4), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_5 = -9998_i16;
_12 = RET;
_1 = true;
_12 = RET;
_7 = 13447004699322841588_u64;
_8 = (-5702691996385250675_i64) as isize;
_13 = [_6,_6,_6,_6,_6,_6,_6,_6];
_6 = 10217_u16 as i32;
_11 = (-4549288500574217845_i64) as f32;
_5 = (-21358_i16) << RET;
_6 = 4291717993_u32 as i32;
_5 = -(-19807_i16);
_4 = 685011111260039988_i64 as i8;
match _7 {
0 => bb8,
1 => bb6,
13447004699322841588 => bb11,
_ => bb10
}
}
bb10 = {
_8 = 9223372036854775807_isize;
_2 = 239111597290562380370770315593020527632_u128 | 77226963931746790958647358251532452212_u128;
_13 = [_6,_6,_6,_6,_6,_6,_6,_6];
_9 = [47200336828278034054218309048455333294_i128,57036287499077537780290005937450754592_i128];
_3 = 187_u8;
RET = 4459325733217177270_usize >> _6;
_12 = _8 as usize;
_3 = !246_u8;
_4 = _2 as i8;
Call(_8 = fn1(_1, _1, _7, _3, _1, _4), ReturnTo(bb9), UnwindUnreachable())
}
bb11 = {
_11 = 8708_u16 as f32;
_9 = [99562140396744031244572060665087073405_i128,(-91337322616798603369079835388181030352_i128)];
_4 = 4177676768_u32 as i8;
_15.1 = '\u{2405e}';
_2 = 250543943654398959527517716783503730150_u128;
_8 = 9223372036854775807_isize;
_15.0 = _8 as i16;
_10 = _2;
_7 = 14540862125011524905_u64;
_15.2 = [59801_u16,45172_u16,60018_u16];
_15.1 = '\u{8abdb}';
RET = !_12;
Call(_15.3 = fn17(_8, _2, _12, _12, _15.2, _12), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_14 = [_15.1,_15.1,_15.1,_15.1];
_14 = [_15.1,_15.1,_15.1,_15.1];
_10 = _2 % _2;
_15.3 = -(-5111758590851715263_i64);
_13 = [_6,_6,_6,_6,_6,_6,_6,_6];
_15.1 = '\u{8915b}';
_15.2 = [5802_u16,37834_u16,58274_u16];
_6 = (-2143647564_i32);
_10 = !_2;
_9 = [(-167938165237401691299539383485436992806_i128),31429747442747998356687319375258875484_i128];
_10 = _2 & _2;
_14 = [_15.1,_15.1,_15.1,_15.1];
_10 = !_2;
_4 = (-26_i8);
_1 = !true;
_16 = _15.1;
_6 = 973882629_i32;
_17.0 = _12;
_1 = !false;
_5 = _3 as i16;
match _7 {
0 => bb10,
14540862125011524905 => bb14,
_ => bb13
}
}
bb13 = {
_8 = 9223372036854775807_isize;
_2 = 239111597290562380370770315593020527632_u128 | 77226963931746790958647358251532452212_u128;
_13 = [_6,_6,_6,_6,_6,_6,_6,_6];
_9 = [47200336828278034054218309048455333294_i128,57036287499077537780290005937450754592_i128];
_3 = 187_u8;
RET = 4459325733217177270_usize >> _6;
_12 = _8 as usize;
_3 = !246_u8;
_4 = _2 as i8;
Call(_8 = fn1(_1, _1, _7, _3, _1, _4), ReturnTo(bb9), UnwindUnreachable())
}
bb14 = {
_17 = (RET,);
_17 = (_12,);
_17 = (RET,);
RET = _12;
_15.2 = [24666_u16,7662_u16,20595_u16];
_8 = _11 as isize;
_17.0 = _6 as usize;
_2 = _10;
_15.1 = _16;
Goto(bb15)
}
bb15 = {
Call(_18 = dump_var(0_usize, 17_usize, Move(_17), 7_usize, Move(_7), 5_usize, Move(_5), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_18 = dump_var(0_usize, 10_usize, Move(_10), 2_usize, Move(_2), 15_usize, Move(_15), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: bool,mut _2: bool,mut _3: u64,mut _4: u8,mut _5: bool,mut _6: i8) -> isize {
mir! {
type RET = isize;
let _7: u8;
let _8: char;
let _9: (u32, usize, i32, u8);
let _10: *mut i32;
let _11: f32;
let _12: ();
let _13: ();
{
_1 = _2;
Call(RET = fn2(_1, _6, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = 89_i8;
_7 = _4;
_2 = _1 & _1;
_4 = _7;
_6 = (-93_i8) << RET;
_2 = !_5;
_2 = _5;
_2 = !_1;
Goto(bb2)
}
bb2 = {
Call(_12 = dump_var(1_usize, 5_usize, Move(_5), 4_usize, Move(_4), 6_usize, Move(_6), 13_usize, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: bool,mut _2: i8,mut _3: bool) -> isize {
mir! {
type RET = isize;
let _4: bool;
let _5: isize;
let _6: [u16; 3];
let _7: Adt47;
let _8: f64;
let _9: f64;
let _10: isize;
let _11: f64;
let _12: [usize; 3];
let _13: Adt46;
let _14: i16;
let _15: (i16, char, [u16; 3], i64);
let _16: i64;
let _17: u64;
let _18: f32;
let _19: (char, *const u32);
let _20: i32;
let _21: Adt47;
let _22: [i32; 8];
let _23: char;
let _24: f64;
let _25: char;
let _26: isize;
let _27: usize;
let _28: (char, *const u32);
let _29: i64;
let _30: i8;
let _31: (i16, char, [u16; 3], i64);
let _32: *const *mut f64;
let _33: u128;
let _34: (u32, usize, i32, u8);
let _35: ();
let _36: ();
{
RET = 9223372036854775807_isize;
_1 = _3 >= _3;
RET = (-9223372036854775808_isize);
RET = 9223372036854775807_isize - (-9223372036854775808_isize);
RET = 9223372036854775807_isize | 9223372036854775807_isize;
_4 = !_1;
_3 = _1 == _4;
RET = '\u{72409}' as isize;
_3 = _4 < _4;
_4 = !_3;
_3 = _4 == _4;
_2 = (-77_i8);
Goto(bb1)
}
bb1 = {
_1 = _3;
RET = (-9223372036854775808_isize) * (-9223372036854775808_isize);
RET = 167_u8 as isize;
RET = (-18_isize) ^ 9223372036854775807_isize;
_5 = RET;
_3 = _4 != _4;
RET = (-17101_i16) as isize;
_1 = _3;
_4 = _3;
_1 = _3 <= _4;
_4 = !_1;
_5 = RET << RET;
RET = -_5;
Call(_2 = fn3(_4, _4, _3, _4, _4, _3, _5, _4, _4, _3, _4, _4, _3, RET, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = _4 >= _4;
_9 = 173_u8 as f64;
_8 = -_9;
RET = _5 << _2;
_1 = _3 > _3;
RET = !_5;
_8 = _9;
RET = '\u{69e87}' as isize;
RET = 3044971473533135769_usize as isize;
_5 = 2313724841_u32 as isize;
_10 = -_5;
_8 = -_9;
_3 = _1;
_11 = _9;
_10 = (-247911692_i32) as isize;
_12 = [9557934789430998867_usize,17482582017008796644_usize,2_usize];
_1 = _3;
_9 = _8;
_3 = !_1;
_10 = (-17772_i16) as isize;
_4 = _1 ^ _1;
RET = -_5;
_13.fld3.3 = !1627510725411873299_usize;
_13.fld0.0 = 6363_i16;
_6 = [1291_u16,30909_u16,17395_u16];
_13.fld3.2 = '\u{6de0f}';
_13.fld6 = 12544310100240871712_u64 as i64;
match _13.fld0.0 {
0 => bb1,
1 => bb3,
2 => bb4,
6363 => bb6,
_ => bb5
}
}
bb3 = {
_1 = _3;
RET = (-9223372036854775808_isize) * (-9223372036854775808_isize);
RET = 167_u8 as isize;
RET = (-18_isize) ^ 9223372036854775807_isize;
_5 = RET;
_3 = _4 != _4;
RET = (-17101_i16) as isize;
_1 = _3;
_4 = _3;
_1 = _3 <= _4;
_4 = !_1;
_5 = RET << RET;
RET = -_5;
Call(_2 = fn3(_4, _4, _3, _4, _4, _3, _5, _4, _4, _3, _4, _4, _3, RET, _1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_13.fld0.0 = (-4899_i16);
_6 = [37254_u16,49243_u16,43793_u16];
_3 = _4 ^ _4;
_13.fld0.3 = _13.fld6;
_15.2 = _6;
Goto(bb7)
}
bb7 = {
_13.fld0 = (18164_i16, _13.fld3.2, _15.2, _13.fld6);
_13.fld1 = [_10,_10,_5,RET,RET,_10,_10,_10];
_13.fld3.3 = 16395560161300513917_usize ^ 3_usize;
RET = !_5;
_13.fld3.0 = _13.fld0.1;
_13.fld0.1 = _13.fld3.2;
_15.2 = _13.fld0.2;
_10 = 65169_u16 as isize;
_12 = [_13.fld3.3,_13.fld3.3,_13.fld3.3];
_15 = _13.fld0;
_13.fld6 = _1 as i64;
RET = _10;
_13.fld3.3 = 3_usize;
_13.fld3.4 = _4 != _3;
_13.fld4 = _15.0 + _15.0;
match _13.fld0.0 {
0 => bb5,
18164 => bb8,
_ => bb2
}
}
bb8 = {
_19.0 = _13.fld0.1;
_15.1 = _13.fld0.1;
_13.fld0 = (_13.fld4, _15.1, _15.2, _13.fld6);
_14 = -_15.0;
_10 = (-149511463630287182053175480750749699805_i128) as isize;
_1 = _3;
_23 = _13.fld0.1;
_13.fld0.1 = _13.fld3.2;
match _15.0 {
0 => bb5,
18164 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_13.fld3 = (_23, _13.fld6, _23, 4_usize, _4);
_15.3 = !_13.fld6;
_15.3 = _13.fld6;
_12 = [_13.fld3.3,_13.fld3.3,_13.fld3.3];
_16 = 106700944878185923793078853890810673867_i128 as i64;
_15.2 = _13.fld0.2;
_4 = _13.fld3.4 < _13.fld3.4;
_13.fld3.1 = _13.fld0.3 ^ _15.3;
_8 = _11;
_13.fld0.1 = _13.fld3.0;
_13.fld6 = !_15.3;
_13.fld3.2 = _13.fld0.1;
_15.0 = _13.fld0.0 >> _13.fld3.1;
_22 = [(-1794369877_i32),845505827_i32,455384750_i32,2040036639_i32,(-685170831_i32),(-831685215_i32),(-326903620_i32),(-1580486230_i32)];
_13.fld6 = _15.3 >> _13.fld3.3;
_13.fld4 = 208712887494417472777314626441810102141_u128 as i16;
_28.0 = _13.fld3.2;
_13.fld3.1 = _15.3;
_14 = _2 as i16;
_6 = [42685_u16,7587_u16,31060_u16];
_28.0 = _13.fld3.2;
_15.0 = _13.fld0.0;
_15.1 = _19.0;
Goto(bb11)
}
bb11 = {
_15 = _13.fld0;
_13.fld0.2 = [9495_u16,35343_u16,30710_u16];
_27 = _13.fld3.3 / _13.fld3.3;
_31.1 = _23;
_29 = _13.fld3.1;
_13.fld5 = 104229024_i32;
_31 = _13.fld0;
_13.fld0.2 = [17823_u16,33101_u16,60656_u16];
_9 = _8;
_14 = 7308089634660518188_u64 as i16;
_13.fld1 = [_10,_5,_10,RET,RET,_10,_10,RET];
Goto(bb12)
}
bb12 = {
_18 = 98803741740429598782376509285103799714_u128 as f32;
_25 = _31.1;
_19.0 = _15.1;
_15.1 = _25;
_10 = !_5;
_17 = !8864021920325647321_u64;
_28.0 = _19.0;
_13.fld3 = (_28.0, _29, _23, _27, _1);
_23 = _31.1;
_28.0 = _25;
_14 = _31.0;
_31.0 = _14;
_13.fld3.4 = _4;
_15.1 = _25;
RET = _29 as isize;
_26 = !_5;
Goto(bb13)
}
bb13 = {
Call(_35 = dump_var(2_usize, 5_usize, Move(_5), 10_usize, Move(_10), 31_usize, Move(_31), 23_usize, Move(_23)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_35 = dump_var(2_usize, 29_usize, Move(_29), 26_usize, Move(_26), 3_usize, Move(_3), 12_usize, Move(_12)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_35 = dump_var(2_usize, 17_usize, Move(_17), 36_usize, _36, 36_usize, _36, 36_usize, _36), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: isize,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: bool,mut _12: bool,mut _13: bool,mut _14: isize,mut _15: bool,mut _16: bool) -> i8 {
mir! {
type RET = i8;
let _17: [u16; 5];
let _18: *const i64;
let _19: Adt60;
let _20: Adt59;
let _21: Adt57;
let _22: [usize; 3];
let _23: [u16; 5];
let _24: Adt51;
let _25: isize;
let _26: [char; 7];
let _27: Adt61;
let _28: (char, i64, char, usize, bool);
let _29: usize;
let _30: ((i16, char, [u16; 3], i64), (*const *mut f64, i128), usize);
let _31: u128;
let _32: bool;
let _33: i128;
let _34: bool;
let _35: bool;
let _36: *mut [char; 7];
let _37: u8;
let _38: *mut f64;
let _39: ();
let _40: ();
{
_14 = -_7;
_15 = _9 | _9;
_1 = _15;
_13 = _10;
Call(_11 = fn4(_9, _8, _2, _2, _4, _15, _1, _2, _15, _4, _9, _15, _16, _9, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = -_14;
_3 = _11;
_14 = !_7;
RET = 7_i8 | 81_i8;
_16 = _6;
_7 = _14 & _14;
RET = (-44_i8) + 71_i8;
_11 = !_1;
_1 = _5;
_13 = _8;
_14 = -_7;
_14 = 10904_i16 as isize;
_9 = !_15;
RET = 8_i8;
_11 = _2;
_14 = _7 + _7;
RET = (-97_i8);
RET = -(-59_i8);
_16 = _15;
_17 = [37074_u16,5351_u16,23096_u16,39151_u16,13712_u16];
_4 = _10 != _16;
_11 = _15 | _12;
_6 = _3;
Call(_19 = fn6(_8, _8, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 3), 2)).2 = !(-1112489395_i32);
_14 = _7;
_9 = _3 >= _13;
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 3), 2)).3 = 241_u8;
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 3), 2)).1 = !7_usize;
_12 = _13 | _1;
_11 = _12 > _13;
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 3), 2)).0 = 2226001375_u32 - 579559508_u32;
_17 = [54635_u16,33759_u16,33121_u16,26500_u16,15392_u16];
Goto(bb3)
}
bb3 = {
place!(Field::<*mut i32>(Variant(_19.fld0, 3), 0)) = core::ptr::addr_of_mut!(place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 3), 2)).2);
_1 = !_6;
_8 = !_13;
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 3), 2)) = (3677311965_u32, 7940968951790546304_usize, (-757844153_i32), 226_u8);
place!(Field::<bool>(Variant(place!(Field::<Adt45>(Variant(_19.fld0, 3), 1)), 0), 0)) = _10;
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 3), 2)).2 = -(-1979952938_i32);
_20.fld3 = !Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 3), 2).1;
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 3), 2)).0 = 2071295539_u32;
_5 = _4 == _13;
_2 = _13;
_8 = !_5;
_3 = !_2;
_12 = _8;
RET = 52_i8 >> _7;
Call(place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 3), 2)).1 = core::intrinsics::transmute(_20.fld3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_19.fld0 = Adt53::Variant0 { fld0: _20.fld3 };
_14 = !_7;
place!(Field::<usize>(Variant(_19.fld0, 0), 0)) = _20.fld3 * _20.fld3;
_17 = [42454_u16,12293_u16,23531_u16,54485_u16,13239_u16];
RET = 18_u8 as i8;
place!(Field::<usize>(Variant(_19.fld0, 0), 0)) = _14 as usize;
_11 = !_12;
_11 = _5;
_2 = _11 == _15;
_17 = [52819_u16,58513_u16,27696_u16,2789_u16,8184_u16];
_6 = !_4;
_11 = _8;
_17 = [25981_u16,9298_u16,62805_u16,54782_u16,60457_u16];
_13 = _4;
_17 = [12296_u16,11144_u16,51436_u16,42625_u16,3827_u16];
SetDiscriminant(_19.fld0, 2);
_25 = _14 & _14;
_2 = _4;
Call(_16 = fn16(_3, _9, _10, _11, _8, _17, _1, _12), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_20.fld3 = 5_usize + 0_usize;
_17 = [41486_u16,20674_u16,4751_u16,23735_u16,41119_u16];
_22 = [_20.fld3,_20.fld3,_20.fld3];
place!(Field::<*const u32>(Variant(_19.fld0, 2), 3)) = core::ptr::addr_of!(place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 2), 1)).0);
place!(Field::<*const u32>(Variant(_19.fld0, 2), 3)) = core::ptr::addr_of!(place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 2), 1)).0);
_6 = _2 == _9;
Goto(bb6)
}
bb6 = {
_20.fld3 = 4_usize;
_24 = Adt51::Variant1 { fld0: '\u{732a9}' };
_12 = _8;
match _20.fld3 {
4 => bb7,
_ => bb4
}
}
bb7 = {
_8 = _10 != _12;
_1 = !_12;
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 2), 1)).0 = 8_u8 as u32;
_10 = _12 <= _2;
_6 = _9 | _16;
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 2), 1)).2 = 64407_u16 as i32;
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 2), 1)).2 = 3274300104570959201_i64 as i32;
_22 = [_20.fld3,_20.fld3,_20.fld3];
_23 = _17;
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 2), 1)) = (749067233_u32, _20.fld3, 691013229_i32, 250_u8);
match Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 2), 1).0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb8,
749067233 => bb10,
_ => bb9
}
}
bb8 = {
_7 = -_14;
_3 = _11;
_14 = !_7;
RET = 7_i8 | 81_i8;
_16 = _6;
_7 = _14 & _14;
RET = (-44_i8) + 71_i8;
_11 = !_1;
_1 = _5;
_13 = _8;
_14 = -_7;
_14 = 10904_i16 as isize;
_9 = !_15;
RET = 8_i8;
_11 = _2;
_14 = _7 + _7;
RET = (-97_i8);
RET = -(-59_i8);
_16 = _15;
_17 = [37074_u16,5351_u16,23096_u16,39151_u16,13712_u16];
_4 = _10 != _16;
_11 = _15 | _12;
_6 = _3;
Call(_19 = fn6(_8, _8, _11), ReturnTo(bb2), UnwindUnreachable())
}
bb9 = {
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 3), 2)).2 = !(-1112489395_i32);
_14 = _7;
_9 = _3 >= _13;
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 3), 2)).3 = 241_u8;
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 3), 2)).1 = !7_usize;
_12 = _13 | _1;
_11 = _12 > _13;
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 3), 2)).0 = 2226001375_u32 - 579559508_u32;
_17 = [54635_u16,33759_u16,33121_u16,26500_u16,15392_u16];
Goto(bb3)
}
bb10 = {
_28.1 = 1726253648431459103_i64 ^ (-1509117620800015443_i64);
_28 = ('\u{d3aa3}', (-4611986879920850613_i64), '\u{82c28}', Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 2), 1).1, _2);
_28.1 = (-2774941418377997741_i64) ^ 1369783567114142961_i64;
_18 = core::ptr::addr_of!(_28.1);
(*_18) = (-3172635426369907982_i64) | (-7343481240125658262_i64);
_9 = !_15;
_7 = -_25;
(*_18) = -(-7861845714872463430_i64);
_26 = [_28.0,_28.2,_28.0,_28.0,_28.0,_28.2,_28.0];
_7 = !_25;
RET = !(-105_i8);
Goto(bb11)
}
bb11 = {
_4 = !_13;
place!(Field::<u64>(Variant(_19.fld0, 2), 2)) = 29667_i16 as u64;
_23 = _17;
_16 = !_4;
_30.0.0 = 13884_u16 as i16;
_28.2 = _28.0;
(*_18) = (-8044566084297906202_i64);
match Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 2), 1).3 {
0 => bb9,
1 => bb2,
2 => bb8,
3 => bb6,
4 => bb10,
250 => bb13,
_ => bb12
}
}
bb12 = {
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 3), 2)).2 = !(-1112489395_i32);
_14 = _7;
_9 = _3 >= _13;
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 3), 2)).3 = 241_u8;
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 3), 2)).1 = !7_usize;
_12 = _13 | _1;
_11 = _12 > _13;
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 3), 2)).0 = 2226001375_u32 - 579559508_u32;
_17 = [54635_u16,33759_u16,33121_u16,26500_u16,15392_u16];
Goto(bb3)
}
bb13 = {
place!(Field::<char>(Variant(_24, 1), 0)) = _28.0;
_15 = _4 ^ _28.4;
_3 = !_6;
_30.2 = _28.3 / _28.3;
_8 = _3 | _13;
_28 = (Field::<char>(Variant(_24, 1), 0), (-3139532668342789660_i64), Field::<char>(Variant(_24, 1), 0), _30.2, _13);
(*_18) = -6083586811350047337_i64;
place!(Field::<u128>(Variant(_19.fld0, 2), 0)) = 109186127892813598116310675439065989899_u128 ^ 107467378700209063784885755830734139051_u128;
_30.0.1 = _28.2;
_30.0.2 = [15750_u16,14079_u16,61093_u16];
_22 = [_28.3,Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 2), 1).1,_30.2];
_28 = (_30.0.1, 436589406363178223_i64, Field::<char>(Variant(_24, 1), 0), _30.2, _1);
_32 = _10 != _4;
_31 = Field::<u128>(Variant(_19.fld0, 2), 0);
_28 = (_30.0.1, (-2950731388913079214_i64), Field::<char>(Variant(_24, 1), 0), Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 2), 1).1, _11);
_29 = _20.fld3;
_5 = _6;
_14 = !_7;
SetDiscriminant(_24, 1);
_17[_29] = _23[_29];
_8 = _15;
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 2), 1)).0 = 2379386203_u32 | 1229195106_u32;
RET = (-18_i8);
match (*_18) {
0 => bb4,
340282366920938463460423876042855132242 => bb14,
_ => bb2
}
}
bb14 = {
RET = -74_i8;
_18 = core::ptr::addr_of!((*_18));
_4 = _10;
_34 = _4 < _5;
_33 = Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 2), 1).0 as i128;
place!(Field::<(u32, usize, i32, u8)>(Variant(_19.fld0, 2), 1)).0 = 2954647015_u32;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(3_usize, 17_usize, Move(_17), 4_usize, Move(_4), 26_usize, Move(_26), 31_usize, Move(_31)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(3_usize, 28_usize, Move(_28), 3_usize, Move(_3), 15_usize, Move(_15), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(3_usize, 6_usize, Move(_6), 29_usize, Move(_29), 14_usize, Move(_14), 33_usize, Move(_33)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_39 = dump_var(3_usize, 13_usize, Move(_13), 40_usize, _40, 40_usize, _40, 40_usize, _40), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: bool,mut _12: bool,mut _13: bool,mut _14: bool,mut _15: bool) -> bool {
mir! {
type RET = bool;
let _16: Adt59;
let _17: f64;
let _18: char;
let _19: [u16; 5];
let _20: ();
let _21: ();
{
_2 = _1;
_10 = !_2;
Call(_3 = fn5(_10, _10, _15, _5, _1, _11, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = !_8;
_13 = _11;
_13 = _5;
_16.fld3 = !6421192001720187049_usize;
_14 = _6 > _4;
RET = !_8;
_4 = _10;
_18 = '\u{b5e30}';
_13 = !_2;
_8 = !_3;
_6 = _7;
_12 = _2;
_4 = _8;
_7 = _9;
Goto(bb2)
}
bb2 = {
Call(_20 = dump_var(4_usize, 15_usize, Move(_15), 6_usize, Move(_6), 4_usize, Move(_4), 7_usize, Move(_7)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_20 = dump_var(4_usize, 1_usize, Move(_1), 13_usize, Move(_13), 18_usize, Move(_18), 12_usize, Move(_12)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool) -> bool {
mir! {
type RET = bool;
let _8: [isize; 8];
let _9: ();
let _10: ();
{
RET = _5 >= _7;
_3 = !RET;
_5 = _1 != _7;
_6 = !RET;
_7 = _6;
_1 = _4;
_6 = !_2;
_1 = _7 > _5;
Goto(bb1)
}
bb1 = {
Call(_9 = dump_var(5_usize, 5_usize, Move(_5), 2_usize, Move(_2), 6_usize, Move(_6), 10_usize, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: bool,mut _2: bool,mut _3: bool) -> Adt60 {
mir! {
type RET = Adt60;
let _4: *const u32;
let _5: [i8; 2];
let _6: bool;
let _7: [u16; 5];
let _8: f64;
let _9: (i8,);
let _10: f64;
let _11: (i16, char, [u16; 3], i64);
let _12: isize;
let _13: isize;
let _14: Adt46;
let _15: *const *const *mut f64;
let _16: (u32, usize, i32, u8);
let _17: i64;
let _18: [usize; 3];
let _19: (usize,);
let _20: bool;
let _21: Adt61;
let _22: Adt60;
let _23: Adt51;
let _24: f64;
let _25: Adt49;
let _26: i8;
let _27: [usize; 3];
let _28: ();
let _29: ();
{
_1 = _3 < _3;
_1 = !_2;
_1 = _2 == _2;
_1 = !_2;
_3 = !_2;
_2 = !_3;
_3 = !_1;
_2 = _1;
_2 = _1;
Call(_5 = fn7(_1, _1, _3, _3, _3, _1, _3, _2, _3, _3, _1, _2, _3, _2, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = _1 >= _1;
_5 = [(-73_i8),18_i8];
_7 = [61656_u16,26231_u16,44911_u16,40163_u16,1860_u16];
_1 = !_3;
_9 = ((-126_i8),);
_9 = ((-58_i8),);
_9.0 = -113_i8;
_8 = 1629442863255760314_u64 as f64;
_8 = (-125372416458677844005361921175053785466_i128) as f64;
RET.fld0 = Adt53::Variant0 { fld0: 6_usize };
_10 = _8;
Goto(bb2)
}
bb2 = {
_1 = !_2;
place!(Field::<usize>(Variant(RET.fld0, 0), 0)) = (-136225318620687609849999912536623363137_i128) as usize;
place!(Field::<usize>(Variant(RET.fld0, 0), 0)) = 4_usize;
_9 = (17_i8,);
_9 = ((-121_i8),);
_7 = [55045_u16,25446_u16,59829_u16,45544_u16,50717_u16];
_9.0 = 6121465413819105837_u64 as i8;
_6 = !_2;
_7 = [58012_u16,49492_u16,39710_u16,22618_u16,34073_u16];
_11.1 = '\u{9e8be}';
_5 = [_9.0,_9.0];
_11.3 = 7412214844442264551_i64;
_11.2 = [18266_u16,49132_u16,8013_u16];
_11.2 = [21133_u16,18912_u16,9319_u16];
_10 = _8 - _8;
_11.0 = 31773_i16 * (-23984_i16);
_9.0 = (-22_i8) >> _11.3;
_12 = _6 as isize;
_8 = _10;
_1 = !_6;
Goto(bb3)
}
bb3 = {
_10 = _8;
_11.2 = [27021_u16,45860_u16,51249_u16];
_6 = !_1;
_8 = _10 + _10;
_9 = ((-10_i8),);
_1 = !_2;
_1 = _3 >= _6;
_11.3 = !2699269791993171432_i64;
_1 = _12 > _12;
_11.0 = 236_u8 as i16;
_11.1 = '\u{e6b8}';
_1 = _3;
_9 = (46_i8,);
_11.2 = [44359_u16,47015_u16,62617_u16];
_9 = (22_i8,);
_5 = [_9.0,_9.0];
Goto(bb4)
}
bb4 = {
_5 = [_9.0,_9.0];
_1 = !_3;
_11.1 = '\u{46f71}';
_14.fld3.3 = _11.3 as usize;
_14.fld0.3 = _11.3;
_9.0 = -57_i8;
_14.fld3.2 = _11.1;
_5 = [_9.0,_9.0];
_14.fld0.2 = _11.2;
_14.fld0.3 = _11.3 - _11.3;
_11.3 = _14.fld0.3 | _14.fld0.3;
_13 = _12;
_16.2 = !(-1582224990_i32);
_14.fld0.1 = _11.1;
_6 = _3 <= _3;
_14.fld3 = (_11.1, _11.3, _11.1, Field::<usize>(Variant(RET.fld0, 0), 0), _3);
_17 = _14.fld3.1;
_14.fld4 = 43004_u16 as i16;
_14.fld5 = _16.2 - _16.2;
place!(Field::<usize>(Variant(RET.fld0, 0), 0)) = _11.0 as usize;
_14.fld1 = [_13,_13,_13,_13,_12,_12,_13,_13];
_14.fld3.0 = _14.fld0.1;
_7 = [61065_u16,44095_u16,64130_u16,25519_u16,22743_u16];
SetDiscriminant(RET.fld0, 0);
place!(Field::<usize>(Variant(RET.fld0, 0), 0)) = 15025066801549893002_u64 as usize;
Goto(bb5)
}
bb5 = {
_11.0 = -_14.fld4;
_14.fld0 = _11;
_9.0 = !(-79_i8);
_14.fld0.3 = !_14.fld3.1;
_5 = [_9.0,_9.0];
SetDiscriminant(RET.fld0, 3);
place!(Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2)).3 = _13 as u8;
_17 = !_14.fld3.1;
_12 = _13;
_14.fld4 = _14.fld0.0 ^ _14.fld0.0;
place!(Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2)).3 = !31_u8;
place!(Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2)).0 = 2678692146_u32;
place!(Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2)).3 = 177_u8;
place!(Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2)).3 = 8_u8 * 224_u8;
_14.fld0.1 = _14.fld3.2;
place!(Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2)) = (4045722180_u32, _14.fld3.3, _16.2, 201_u8);
_14.fld3.1 = -_14.fld0.3;
Call(_17 = fn10(_12, _14.fld3.4, _6, _6, _12, _13, _14.fld0.0), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_14.fld5 = Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2).2 | Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2).2;
_2 = _14.fld3.4;
_11.0 = 2969036891356423898_u64 as i16;
_6 = _1;
_16.1 = Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2).3 as usize;
_1 = !_6;
_2 = _6;
_14.fld2 = Adt45::Variant0 { fld0: _1 };
match Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2).1 {
0 => bb4,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb12,
_ => bb11
}
}
bb7 = {
_11.0 = -_14.fld4;
_14.fld0 = _11;
_9.0 = !(-79_i8);
_14.fld0.3 = !_14.fld3.1;
_5 = [_9.0,_9.0];
SetDiscriminant(RET.fld0, 3);
place!(Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2)).3 = _13 as u8;
_17 = !_14.fld3.1;
_12 = _13;
_14.fld4 = _14.fld0.0 ^ _14.fld0.0;
place!(Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2)).3 = !31_u8;
place!(Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2)).0 = 2678692146_u32;
place!(Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2)).3 = 177_u8;
place!(Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2)).3 = 8_u8 * 224_u8;
_14.fld0.1 = _14.fld3.2;
place!(Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2)) = (4045722180_u32, _14.fld3.3, _16.2, 201_u8);
_14.fld3.1 = -_14.fld0.3;
Call(_17 = fn10(_12, _14.fld3.4, _6, _6, _12, _13, _14.fld0.0), ReturnTo(bb6), UnwindUnreachable())
}
bb8 = {
_5 = [_9.0,_9.0];
_1 = !_3;
_11.1 = '\u{46f71}';
_14.fld3.3 = _11.3 as usize;
_14.fld0.3 = _11.3;
_9.0 = -57_i8;
_14.fld3.2 = _11.1;
_5 = [_9.0,_9.0];
_14.fld0.2 = _11.2;
_14.fld0.3 = _11.3 - _11.3;
_11.3 = _14.fld0.3 | _14.fld0.3;
_13 = _12;
_16.2 = !(-1582224990_i32);
_14.fld0.1 = _11.1;
_6 = _3 <= _3;
_14.fld3 = (_11.1, _11.3, _11.1, Field::<usize>(Variant(RET.fld0, 0), 0), _3);
_17 = _14.fld3.1;
_14.fld4 = 43004_u16 as i16;
_14.fld5 = _16.2 - _16.2;
place!(Field::<usize>(Variant(RET.fld0, 0), 0)) = _11.0 as usize;
_14.fld1 = [_13,_13,_13,_13,_12,_12,_13,_13];
_14.fld3.0 = _14.fld0.1;
_7 = [61065_u16,44095_u16,64130_u16,25519_u16,22743_u16];
SetDiscriminant(RET.fld0, 0);
place!(Field::<usize>(Variant(RET.fld0, 0), 0)) = 15025066801549893002_u64 as usize;
Goto(bb5)
}
bb9 = {
_10 = _8;
_11.2 = [27021_u16,45860_u16,51249_u16];
_6 = !_1;
_8 = _10 + _10;
_9 = ((-10_i8),);
_1 = !_2;
_1 = _3 >= _6;
_11.3 = !2699269791993171432_i64;
_1 = _12 > _12;
_11.0 = 236_u8 as i16;
_11.1 = '\u{e6b8}';
_1 = _3;
_9 = (46_i8,);
_11.2 = [44359_u16,47015_u16,62617_u16];
_9 = (22_i8,);
_5 = [_9.0,_9.0];
Goto(bb4)
}
bb10 = {
_1 = !_2;
place!(Field::<usize>(Variant(RET.fld0, 0), 0)) = (-136225318620687609849999912536623363137_i128) as usize;
place!(Field::<usize>(Variant(RET.fld0, 0), 0)) = 4_usize;
_9 = (17_i8,);
_9 = ((-121_i8),);
_7 = [55045_u16,25446_u16,59829_u16,45544_u16,50717_u16];
_9.0 = 6121465413819105837_u64 as i8;
_6 = !_2;
_7 = [58012_u16,49492_u16,39710_u16,22618_u16,34073_u16];
_11.1 = '\u{9e8be}';
_5 = [_9.0,_9.0];
_11.3 = 7412214844442264551_i64;
_11.2 = [18266_u16,49132_u16,8013_u16];
_11.2 = [21133_u16,18912_u16,9319_u16];
_10 = _8 - _8;
_11.0 = 31773_i16 * (-23984_i16);
_9.0 = (-22_i8) >> _11.3;
_12 = _6 as isize;
_8 = _10;
_1 = !_6;
Goto(bb3)
}
bb11 = {
_6 = _1 >= _1;
_5 = [(-73_i8),18_i8];
_7 = [61656_u16,26231_u16,44911_u16,40163_u16,1860_u16];
_1 = !_3;
_9 = ((-126_i8),);
_9 = ((-58_i8),);
_9.0 = -113_i8;
_8 = 1629442863255760314_u64 as f64;
_8 = (-125372416458677844005361921175053785466_i128) as f64;
RET.fld0 = Adt53::Variant0 { fld0: 6_usize };
_10 = _8;
Goto(bb2)
}
bb12 = {
_14.fld3.0 = _14.fld0.1;
_14.fld0.2 = [25579_u16,44218_u16,10101_u16];
_14.fld3.1 = _17;
_18 = [_16.1,Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2).1,_16.1];
_14.fld1 = [_12,_13,_12,_13,_13,_13,_13,_12];
_16.2 = Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2).2 - _14.fld5;
_19 = (_16.1,);
_18 = [_19.0,_19.0,Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2).1];
place!(Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2)).3 = 254_u8;
_9.0 = _14.fld4 as i8;
place!(Field::<*mut i32>(Variant(RET.fld0, 3), 0)) = core::ptr::addr_of_mut!(_16.2);
_14.fld0.2 = [1995_u16,9794_u16,64184_u16];
_11.1 = _14.fld3.0;
_24 = -_10;
_14.fld5 = _12 as i32;
_22.fld0 = Adt53::Variant3 { fld0: Field::<*mut i32>(Variant(RET.fld0, 3), 0),fld1: Move(_14.fld2),fld2: Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2) };
place!(Field::<(u32, usize, i32, u8)>(Variant(_22.fld0, 3), 2)).1 = _19.0;
_14.fld6 = Field::<(u32, usize, i32, u8)>(Variant(_22.fld0, 3), 2).3 as i64;
_16.0 = !Field::<(u32, usize, i32, u8)>(Variant(RET.fld0, 3), 2).0;
_14.fld0.3 = _17 + _14.fld3.1;
_5 = [_9.0,_9.0];
place!(Field::<Adt45>(Variant(RET.fld0, 3), 1)) = Move(Field::<Adt45>(Variant(_22.fld0, 3), 1));
Goto(bb13)
}
bb13 = {
Call(_28 = dump_var(6_usize, 13_usize, Move(_13), 5_usize, Move(_5), 3_usize, Move(_3), 11_usize, Move(_11)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_28 = dump_var(6_usize, 7_usize, Move(_7), 1_usize, Move(_1), 29_usize, _29, 29_usize, _29), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: bool,mut _12: bool,mut _13: bool,mut _14: bool,mut _15: bool) -> [i8; 2] {
mir! {
type RET = [i8; 2];
let _16: char;
let _17: i32;
let _18: isize;
let _19: u64;
let _20: *const *mut f64;
let _21: [i32; 8];
let _22: f32;
let _23: [i32; 8];
let _24: [char; 7];
let _25: bool;
let _26: isize;
let _27: (usize,);
let _28: [isize; 8];
let _29: [usize; 3];
let _30: [i8; 2];
let _31: char;
let _32: Adt54;
let _33: Adt49;
let _34: (char, i64, char, usize, bool);
let _35: Adt46;
let _36: [i8; 2];
let _37: ();
let _38: ();
{
RET = [(-82_i8),(-75_i8)];
_2 = !_6;
_10 = _1;
_11 = _9;
_8 = !_2;
_3 = !_12;
_11 = !_9;
_8 = _13;
Goto(bb1)
}
bb1 = {
_18 = !(-42_isize);
_5 = _3;
_16 = '\u{ef547}';
_17 = -(-617437643_i32);
_7 = _9 <= _6;
_18 = 96_isize ^ (-33_isize);
_13 = _14;
_7 = !_2;
_6 = _12;
_9 = _4;
Goto(bb2)
}
bb2 = {
_16 = '\u{4c56c}';
_5 = _7 <= _10;
_2 = _13 != _13;
_19 = (-103450966027426056014043061432898120750_i128) as u64;
_13 = _2 > _8;
_19 = 3491637043_u32 as u64;
_18 = (-9223372036854775808_isize) >> _17;
Goto(bb3)
}
bb3 = {
_2 = !_11;
_17 = (-1076064030_i32);
_21 = [_17,_17,_17,_17,_17,_17,_17,_17];
_11 = !_6;
_5 = !_7;
_13 = !_10;
_1 = !_2;
_22 = (-3587937805391606844_i64) as f32;
_13 = _8 == _3;
_8 = !_9;
_8 = _9;
_2 = !_1;
_7 = _1 ^ _4;
_10 = !_11;
_21 = [_17,_17,_17,_17,_17,_17,_17,_17];
match _17 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
340282366920938463463374607430692147426 => bb12,
_ => bb11
}
}
bb4 = {
_16 = '\u{4c56c}';
_5 = _7 <= _10;
_2 = _13 != _13;
_19 = (-103450966027426056014043061432898120750_i128) as u64;
_13 = _2 > _8;
_19 = 3491637043_u32 as u64;
_18 = (-9223372036854775808_isize) >> _17;
Goto(bb3)
}
bb5 = {
_18 = !(-42_isize);
_5 = _3;
_16 = '\u{ef547}';
_17 = -(-617437643_i32);
_7 = _9 <= _6;
_18 = 96_isize ^ (-33_isize);
_13 = _14;
_7 = !_2;
_6 = _12;
_9 = _4;
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
_9 = _7;
_6 = _10 & _4;
_16 = '\u{72a37}';
_22 = 4088934333121062636_usize as f32;
_18 = 9223372036854775807_isize + (-9223372036854775808_isize);
_25 = _12;
RET = [75_i8,9_i8];
_1 = _7 ^ _9;
RET = [93_i8,76_i8];
_21 = [_17,_17,_17,_17,_17,_17,_17,_17];
_1 = _8;
_2 = !_13;
_11 = _12;
_13 = _1;
_12 = _25 | _5;
_9 = !_11;
_16 = '\u{e7010}';
_4 = _2;
_10 = _7;
Call(_17 = core::intrinsics::transmute(_16), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_26 = -_18;
_23 = [_17,_17,_17,_17,_17,_17,_17,_17];
_5 = !_12;
_24 = [_16,_16,_16,_16,_16,_16,_16];
_14 = _3 | _15;
_19 = 16362564508068131404_u64 * 10304615026758548295_u64;
_17 = 299965022_i32 & (-2068065330_i32);
_13 = !_11;
_28 = [_26,_18,_18,_18,_18,_18,_26,_26];
Call(_22 = fn8(_1, _15, _9, _5, _11, _2, _15, _10, _5, _4, _13, _12), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_30 = RET;
_34.0 = _16;
_34.2 = _34.0;
_35.fld3.2 = _34.2;
_5 = _10;
_34 = (_16, 7769487499668814724_i64, _35.fld3.2, 0_usize, _15);
_35.fld4 = (-13291_i16);
_35.fld3.0 = _34.0;
_35.fld0.2 = [25268_u16,9402_u16,15020_u16];
_17 = 1604751318_i32 * (-1339680774_i32);
_35.fld3.2 = _34.2;
_35.fld0.1 = _35.fld3.0;
_35.fld5 = _17;
_15 = _1 | _5;
_24 = [_35.fld0.1,_35.fld0.1,_34.2,_35.fld3.2,_35.fld3.2,_35.fld3.0,_35.fld3.0];
_16 = _35.fld3.2;
_35.fld6 = _34.1 ^ _34.1;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(7_usize, 8_usize, Move(_8), 16_usize, Move(_16), 2_usize, Move(_2), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(7_usize, 9_usize, Move(_9), 28_usize, Move(_28), 4_usize, Move(_4), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(7_usize, 15_usize, Move(_15), 26_usize, Move(_26), 13_usize, Move(_13), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_37 = dump_var(7_usize, 25_usize, Move(_25), 38_usize, _38, 38_usize, _38, 38_usize, _38), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: bool,mut _12: bool) -> f32 {
mir! {
type RET = f32;
let _13: char;
let _14: bool;
let _15: isize;
let _16: f64;
let _17: i128;
let _18: i16;
let _19: i16;
let _20: Adt48;
let _21: bool;
let _22: isize;
let _23: (i16, char, [u16; 3], i64);
let _24: [usize; 3];
let _25: u16;
let _26: *mut i32;
let _27: usize;
let _28: i8;
let _29: i128;
let _30: ();
let _31: ();
{
_4 = _1 & _9;
_1 = _11;
RET = (-64_i8) as f32;
_8 = _1 & _12;
RET = 80326796808464548160497914596502255901_i128 as f32;
_14 = _6 & _12;
_10 = _14;
_12 = _5;
_16 = 5718181303753661794_i64 as f64;
RET = 245258838720059844032236212986713271386_u128 as f32;
_3 = _6;
_3 = _2;
_9 = !_3;
_7 = !_14;
_2 = _1 <= _8;
_11 = _10 == _2;
_2 = _10 | _1;
_15 = (-9223372036854775808_isize);
_14 = _4;
_13 = '\u{585aa}';
_6 = !_12;
_10 = _9 ^ _2;
match _15 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463454151235394913435648 => bb9,
_ => bb8
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
Return()
}
bb9 = {
_16 = 15853555659362548086_usize as f64;
_13 = '\u{1f4a2}';
_15 = -9223372036854775807_isize;
_2 = !_10;
_1 = _12;
_7 = !_6;
_10 = !_2;
_9 = !_8;
_18 = (-13256_i16);
_13 = '\u{5fb5c}';
_18 = 225_u8 as i16;
_5 = _7;
_19 = _18 & _18;
_10 = _11 <= _9;
_6 = _7;
_16 = 967760213_u32 as f64;
_11 = _1 != _10;
_15 = 9223372036854775807_isize;
_10 = !_14;
_3 = _8 | _8;
_17 = 149082880906516580389250382729126982805_i128;
RET = 1396542974_u32 as f32;
_21 = !_7;
Call(_11 = fn9(_14, _3, _2, _7, _5, _3, _1, _2, _4, _21, _2, _2, _1, _7, _14, _1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_2 = _6 < _14;
_14 = _10 > _8;
Goto(bb11)
}
bb11 = {
_23.3 = 6561251095248885648_i64;
_22 = _15 ^ _15;
_10 = _3 ^ _1;
_8 = _5;
match _15 {
0 => bb2,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
9223372036854775807 => bb19,
_ => bb18
}
}
bb12 = {
Return()
}
bb13 = {
_16 = 15853555659362548086_usize as f64;
_13 = '\u{1f4a2}';
_15 = -9223372036854775807_isize;
_2 = !_10;
_1 = _12;
_7 = !_6;
_10 = !_2;
_9 = !_8;
_18 = (-13256_i16);
_13 = '\u{5fb5c}';
_18 = 225_u8 as i16;
_5 = _7;
_19 = _18 & _18;
_10 = _11 <= _9;
_6 = _7;
_16 = 967760213_u32 as f64;
_11 = _1 != _10;
_15 = 9223372036854775807_isize;
_10 = !_14;
_3 = _8 | _8;
_17 = 149082880906516580389250382729126982805_i128;
RET = 1396542974_u32 as f32;
_21 = !_7;
Call(_11 = fn9(_14, _3, _2, _7, _5, _3, _1, _2, _4, _21, _2, _2, _1, _7, _14, _1), ReturnTo(bb10), UnwindUnreachable())
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
Return()
}
bb18 = {
Return()
}
bb19 = {
_14 = _4 <= _21;
_23.1 = _13;
_13 = _23.1;
_14 = _9 > _7;
_23.0 = _18 * _19;
_23.2 = [55205_u16,62535_u16,3362_u16];
_19 = _23.0;
_10 = _1;
_19 = _18 & _23.0;
_21 = !_2;
_7 = _4;
_4 = _8;
_2 = _12;
_14 = _4;
_12 = _9;
_17 = _23.3 as i128;
_16 = _23.3 as f64;
_22 = RET as isize;
_14 = !_3;
_4 = _14 == _10;
_25 = 11151_u16;
Goto(bb20)
}
bb20 = {
Call(_30 = dump_var(8_usize, 8_usize, Move(_8), 4_usize, Move(_4), 5_usize, Move(_5), 11_usize, Move(_11)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_30 = dump_var(8_usize, 10_usize, Move(_10), 17_usize, Move(_17), 14_usize, Move(_14), 6_usize, Move(_6)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_30 = dump_var(8_usize, 3_usize, Move(_3), 1_usize, Move(_1), 2_usize, Move(_2), 31_usize, _31), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: bool,mut _12: bool,mut _13: bool,mut _14: bool,mut _15: bool,mut _16: bool) -> bool {
mir! {
type RET = bool;
let _17: Adt59;
let _18: [isize; 8];
let _19: [usize; 3];
let _20: (i16, char, [u16; 3], i64);
let _21: isize;
let _22: ();
let _23: ();
{
_8 = !_9;
_16 = _8 <= _6;
_11 = !_5;
_3 = _7 ^ _12;
_8 = _13;
_13 = !_6;
_14 = _13;
_1 = !_4;
_10 = !_13;
_8 = !_16;
RET = !_6;
_16 = !_8;
Goto(bb1)
}
bb1 = {
Call(_22 = dump_var(9_usize, 14_usize, Move(_14), 1_usize, Move(_1), 15_usize, Move(_15), 4_usize, Move(_4)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_22 = dump_var(9_usize, 9_usize, Move(_9), 7_usize, Move(_7), 8_usize, Move(_8), 2_usize, Move(_2)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: isize,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: isize,mut _6: isize,mut _7: i16) -> i64 {
mir! {
type RET = i64;
let _8: i64;
let _9: i128;
let _10: Adt60;
let _11: Adt51;
let _12: [char; 7];
let _13: *const u32;
let _14: (usize,);
let _15: char;
let _16: [char; 7];
let _17: (usize,);
let _18: f64;
let _19: [char; 4];
let _20: f32;
let _21: *mut [char; 7];
let _22: (char, i64, char, usize, bool);
let _23: bool;
let _24: (usize,);
let _25: (i8,);
let _26: bool;
let _27: ((i16, char, [u16; 3], i64), (*const *mut f64, i128), usize);
let _28: [i8; 2];
let _29: [char; 7];
let _30: f32;
let _31: bool;
let _32: bool;
let _33: [char; 7];
let _34: f64;
let _35: (i16, char, [u16; 3], i64);
let _36: f64;
let _37: isize;
let _38: u32;
let _39: i64;
let _40: i16;
let _41: isize;
let _42: [u16; 5];
let _43: isize;
let _44: isize;
let _45: (i8,);
let _46: ();
let _47: ();
{
RET = -893305762662962537_i64;
_1 = !_5;
_1 = _5 << _6;
_5 = !_1;
_2 = _4;
_2 = _3;
RET = (-7165854667154208233_i64);
_3 = _2;
_9 = 3647325516091625170_u64 as i128;
_10.fld0 = Adt53::Variant0 { fld0: 4484947133018648678_usize };
_8 = _7 as i64;
_8 = !RET;
RET = 4164863973_u32 as i64;
_7 = !32126_i16;
_3 = _4;
_6 = _5;
_2 = _4;
_12 = ['\u{54cc1}','\u{465ad}','\u{908e0}','\u{6f6cc}','\u{b0f6e}','\u{81768}','\u{acae4}'];
_11 = Adt51::Variant1 { fld0: '\u{a6505}' };
_2 = !_4;
Goto(bb1)
}
bb1 = {
_9 = (-111661494196871140104405383347204596794_i128);
place!(Field::<usize>(Variant(_10.fld0, 0), 0)) = 9836887680264626163_usize;
_14 = (Field::<usize>(Variant(_10.fld0, 0), 0),);
_8 = -RET;
_6 = _1;
_1 = !_5;
_14 = (Field::<usize>(Variant(_10.fld0, 0), 0),);
_4 = _6 <= _1;
_8 = 121_i8 as i64;
_15 = '\u{1984e}';
_4 = !_2;
_7 = _14.0 as i16;
place!(Field::<char>(Variant(_11, 1), 0)) = _15;
_16 = [_15,Field::<char>(Variant(_11, 1), 0),_15,_15,_15,Field::<char>(Variant(_11, 1), 0),Field::<char>(Variant(_11, 1), 0)];
_7 = Field::<char>(Variant(_11, 1), 0) as i16;
_17 = (Field::<usize>(Variant(_10.fld0, 0), 0),);
SetDiscriminant(_11, 1);
place!(Field::<usize>(Variant(_10.fld0, 0), 0)) = _17.0;
Goto(bb2)
}
bb2 = {
place!(Field::<usize>(Variant(_10.fld0, 0), 0)) = !_14.0;
_7 = (-22615_i16);
RET = _8;
_3 = _4;
place!(Field::<char>(Variant(_11, 1), 0)) = _15;
_18 = 69_u8 as f64;
_7 = 26855_i16 * (-25125_i16);
_1 = !_5;
_6 = _1 & _5;
SetDiscriminant(_11, 1);
RET = _8;
SetDiscriminant(_10.fld0, 1);
_1 = _5 << _5;
place!(Field::<u16>(Variant(_10.fld0, 1), 7)) = !42309_u16;
place!(Field::<*const u16>(Variant(_10.fld0, 1), 6)) = core::ptr::addr_of!(place!(Field::<u16>(Variant(_10.fld0, 1), 7)));
_16 = [_15,_15,_15,_15,_15,_15,_15];
_14 = _17;
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2)).1 = _15;
_5 = _6;
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2)).0 = _7 & _7;
_22.3 = _17.0;
Call(_14.0 = core::intrinsics::bswap(_22.3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
place!(Field::<[char; 7]>(Variant(_10.fld0, 1), 3)) = [Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1,Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1,Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1,Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1,Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1,Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1,_15];
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2)).2 = [Field::<u16>(Variant(_10.fld0, 1), 7),Field::<u16>(Variant(_10.fld0, 1), 7),Field::<u16>(Variant(_10.fld0, 1), 7)];
_15 = Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1;
_20 = _18 as f32;
_19 = [Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1,_15,_15,Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1];
_17.0 = _14.0 + _14.0;
_22.2 = _15;
_14 = (_17.0,);
_3 = _2 & _2;
place!(Field::<[char; 7]>(Variant(_10.fld0, 1), 3)) = _16;
_9 = !(-116665401304878070527910213460480227813_i128);
place!(Field::<f64>(Variant(_10.fld0, 1), 1)) = _18 * _18;
_15 = _22.2;
_24 = (_14.0,);
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2)).0 = Field::<f64>(Variant(_10.fld0, 1), 1) as i16;
_9 = !15348959498049541181730720192744471249_i128;
_22.0 = _22.2;
_1 = -_5;
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2)).2 = [Field::<u16>(Variant(_10.fld0, 1), 7),Field::<u16>(Variant(_10.fld0, 1), 7),Field::<u16>(Variant(_10.fld0, 1), 7)];
_1 = _5;
_22.2 = _22.0;
place!(Field::<f64>(Variant(_10.fld0, 1), 1)) = _18 * _18;
_17 = _24;
_22.3 = _14.0 & _14.0;
Call(_14.0 = core::intrinsics::transmute(_6), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_15 = Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1;
_22.4 = _2 | _2;
_20 = _5 as f32;
_23 = _2;
_25.0 = !(-10_i8);
_1 = _6 ^ _6;
_22.2 = Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1;
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2)).1 = _15;
_22 = (_15, RET, _15, _14.0, _2);
_12 = _16;
place!(Field::<[char; 7]>(Variant(_10.fld0, 1), 3)) = [_15,_15,_22.0,_15,_22.0,Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1,_22.0];
_27.2 = _22.3 * _14.0;
_19 = [Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1,_15,_22.0,Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1];
_27.0.0 = _7;
_22 = (Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1, RET, _15, _14.0, _23);
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2)).2 = [Field::<u16>(Variant(_10.fld0, 1), 7),Field::<u16>(Variant(_10.fld0, 1), 7),Field::<u16>(Variant(_10.fld0, 1), 7)];
_27.2 = !_22.3;
_9 = 3673390419739288327222533640336365574_i128 - (-108702080601317147713586366457804825307_i128);
_8 = !_22.1;
_22.2 = _15;
_22.4 = _4;
place!(Field::<[char; 7]>(Variant(_10.fld0, 1), 3)) = [_15,Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1,Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1,Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1,_15,Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1,Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1];
Call(_5 = fn11(_27.2, _1, _22.4, _22.3), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_25 = ((-11_i8),);
_29 = [_22.0,_15,_15,_15,_22.2,Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1,_15];
_21 = core::ptr::addr_of_mut!(_12);
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2)).3 = !_8;
_25.0 = _20 as i8;
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2)).0 = _22.1 as i16;
_9 = -57534902631471431218867509970412728632_i128;
place!(Field::<u16>(Variant(_10.fld0, 1), 7)) = !4300_u16;
_19 = [_22.2,_15,_15,_22.2];
_27.0.3 = !RET;
_22.0 = Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1;
_24.0 = _14.0;
_27.0.1 = _15;
_22.0 = Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1;
_31 = _4 >= _23;
_32 = !_4;
place!(Field::<[char; 7]>(Variant(_10.fld0, 1), 3)) = [_22.2,Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1,_22.0,_22.0,_27.0.1,_27.0.1,_15];
_27.2 = _22.3;
_2 = !_23;
_14 = (_27.2,);
RET = _27.0.3;
_22.4 = _31;
_28 = [_25.0,_25.0];
_30 = _20 + _20;
_17.0 = !_24.0;
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2)).2 = [Field::<u16>(Variant(_10.fld0, 1), 7),Field::<u16>(Variant(_10.fld0, 1), 7),Field::<u16>(Variant(_10.fld0, 1), 7)];
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2)).2 = [Field::<u16>(Variant(_10.fld0, 1), 7),Field::<u16>(Variant(_10.fld0, 1), 7),Field::<u16>(Variant(_10.fld0, 1), 7)];
_27.0.2 = [Field::<u16>(Variant(_10.fld0, 1), 7),Field::<u16>(Variant(_10.fld0, 1), 7),Field::<u16>(Variant(_10.fld0, 1), 7)];
Goto(bb6)
}
bb6 = {
_22.0 = _22.2;
_29 = [Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1,_22.2,_22.2,_22.0,_22.2,_15,_22.0];
_7 = _23 as i16;
_1 = _5 + _5;
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2)).1 = _22.2;
_26 = !_22.4;
_17 = (_27.2,);
_6 = _5 ^ _5;
_15 = _22.0;
_27.0.0 = -_7;
_26 = _30 == _20;
_6 = 221_u8 as isize;
_25 = ((-27_i8),);
_34 = _30 as f64;
_35.2 = [Field::<u16>(Variant(_10.fld0, 1), 7),Field::<u16>(Variant(_10.fld0, 1), 7),Field::<u16>(Variant(_10.fld0, 1), 7)];
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2)).1 = _22.2;
_17 = (_22.3,);
_35.1 = Field::<(i16, char, [u16; 3], i64)>(Variant(_10.fld0, 1), 2).1;
_10.fld0 = Adt53::Variant0 { fld0: _24.0 };
_8 = _27.0.3 * _27.0.3;
_26 = _3;
_22 = (_15, _8, _35.1, _14.0, _23);
_14 = (_24.0,);
_21 = core::ptr::addr_of_mut!(_33);
_19 = [_22.0,_15,_22.0,_22.2];
Call(_27.0.1 = fn12(_14, _5, _20, Move(_10.fld0), _31, _14, _23, _34, _14.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_29 = [_15,_22.2,_27.0.1,_35.1,_22.0,_22.0,_27.0.1];
_35.3 = RET ^ RET;
_39 = _22.1;
_1 = _5;
_25.0 = -34_i8;
(*_21) = [_35.1,_35.1,_22.0,_35.1,_35.1,_35.1,_22.2];
_27.0.3 = _22.1 >> _22.3;
_27.1.1 = _27.0.0 as i128;
_29 = _33;
_38 = 4264450105_u32 - 2561491791_u32;
_39 = _27.0.3 & _27.0.3;
_35.0 = -_7;
_40 = _7 - _27.0.0;
_25.0 = -(-5_i8);
_24.0 = _27.2 << _5;
_8 = -_39;
_26 = _23;
_29 = _12;
_17 = (_27.2,);
Goto(bb8)
}
bb8 = {
_44 = 51328_u16 as isize;
_40 = -_27.0.0;
_45.0 = _25.0;
_42 = [16158_u16,31770_u16,28867_u16,18886_u16,21833_u16];
_24 = (_27.2,);
_17 = _24;
_27.0.2 = [44259_u16,10734_u16,10816_u16];
_27.0.3 = 21759_u16 as i64;
_33 = _12;
_1 = _5;
_17.0 = _14.0;
_45.0 = _25.0 - _25.0;
_42 = [60971_u16,25579_u16,29080_u16,7260_u16,7527_u16];
_19 = [_22.2,_27.0.1,_15,_22.2];
_22.1 = _1 as i64;
RET = _8;
_3 = _2 <= _32;
_41 = _1;
_1 = _22.2 as isize;
_35.3 = _39;
Goto(bb9)
}
bb9 = {
Call(_46 = dump_var(10_usize, 8_usize, Move(_8), 41_usize, Move(_41), 19_usize, Move(_19), 31_usize, Move(_31)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_46 = dump_var(10_usize, 40_usize, Move(_40), 35_usize, Move(_35), 14_usize, Move(_14), 5_usize, Move(_5)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_46 = dump_var(10_usize, 4_usize, Move(_4), 7_usize, Move(_7), 44_usize, Move(_44), 32_usize, Move(_32)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_46 = dump_var(10_usize, 3_usize, Move(_3), 45_usize, Move(_45), 39_usize, Move(_39), 38_usize, Move(_38)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: usize,mut _2: isize,mut _3: bool,mut _4: usize) -> isize {
mir! {
type RET = isize;
let _5: [u16; 5];
let _6: ();
let _7: ();
{
RET = _2;
_1 = 891747508_i32 as usize;
Goto(bb1)
}
bb1 = {
Call(_6 = dump_var(11_usize, 2_usize, Move(_2), 4_usize, Move(_4), 7_usize, _7, 7_usize, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: (usize,),mut _2: isize,mut _3: f32,mut _4: Adt53,mut _5: bool,mut _6: (usize,),mut _7: bool,mut _8: f64,mut _9: usize) -> char {
mir! {
type RET = char;
let _10: (u32, usize, i32, u8);
let _11: (usize,);
let _12: (u32, usize, i32, u8);
let _13: [char; 7];
let _14: Adt52;
let _15: &'static f32;
let _16: *const u32;
let _17: char;
let _18: [char; 4];
let _19: u8;
let _20: Adt47;
let _21: u64;
let _22: (char, i64, char, usize, bool);
let _23: [i8; 2];
let _24: isize;
let _25: isize;
let _26: f32;
let _27: (u32, usize, i32, u8);
let _28: f32;
let _29: Adt46;
let _30: i16;
let _31: bool;
let _32: Adt45;
let _33: (usize,);
let _34: i128;
let _35: Adt50;
let _36: ();
let _37: ();
{
RET = '\u{108018}';
_6 = (_9,);
_3 = 73442030_u32 as f32;
SetDiscriminant(_4, 0);
_6 = _1;
_8 = (-128_i8) as f64;
_8 = 4119017287_u32 as f64;
_7 = !_5;
_7 = !_5;
place!(Field::<usize>(Variant(_4, 0), 0)) = _6.0;
RET = '\u{92a9c}';
_9 = _6.0 & _1.0;
_10.0 = 1849488772_u32;
_8 = (-145432068765032642142550302921983236311_i128) as f64;
_9 = Field::<usize>(Variant(_4, 0), 0);
_10.3 = 254_u8;
_11.0 = !_1.0;
_8 = 3980803320945772477_i64 as f64;
_1.0 = _6.0;
Goto(bb1)
}
bb1 = {
_10.0 = !3004366073_u32;
_11 = _1;
_10.1 = !_1.0;
_12.1 = _11.0 - Field::<usize>(Variant(_4, 0), 0);
_2 = 84_isize | (-51_isize);
_2 = 9223372036854775807_isize;
_10.3 = !218_u8;
_12.1 = !Field::<usize>(Variant(_4, 0), 0);
Goto(bb2)
}
bb2 = {
_8 = (-61_i8) as f64;
_12.1 = Field::<usize>(Variant(_4, 0), 0);
SetDiscriminant(_4, 1);
_12.2 = (-976009512_i32) * (-1290919040_i32);
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2)).0 = _10.0 as i16;
place!(Field::<*const u16>(Variant(_4, 1), 6)) = core::ptr::addr_of!(place!(Field::<u16>(Variant(_4, 1), 7)));
_12.1 = _11.0;
_17 = RET;
place!(Field::<*const u16>(Variant(_4, 1), 6)) = core::ptr::addr_of!(place!(Field::<u16>(Variant(_4, 1), 7)));
match _2 {
0 => bb1,
1 => bb3,
2 => bb4,
9223372036854775807 => bb6,
_ => bb5
}
}
bb3 = {
_10.0 = !3004366073_u32;
_11 = _1;
_10.1 = !_1.0;
_12.1 = _11.0 - Field::<usize>(Variant(_4, 0), 0);
_2 = 84_isize | (-51_isize);
_2 = 9223372036854775807_isize;
_10.3 = !218_u8;
_12.1 = !Field::<usize>(Variant(_4, 0), 0);
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
place!(Field::<f64>(Variant(_4, 1), 1)) = -_8;
_10.0 = 53400358451402713576060737414802662848_i128 as u32;
_12.2 = 1976705465_i32 - 330457762_i32;
_17 = RET;
_2 = 9223372036854775807_isize;
_16 = core::ptr::addr_of!(_12.0);
Goto(bb7)
}
bb7 = {
_2 = (-95_isize);
_17 = RET;
_10 = (14581708_u32, _9, _12.2, 5_u8);
_10.2 = -_12.2;
_22.0 = _17;
_2 = 108_isize;
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2)).2 = [22287_u16,60184_u16,52598_u16];
_10.1 = !_11.0;
(*_16) = Field::<f64>(Variant(_4, 1), 1) as u32;
_2 = -9223372036854775807_isize;
_12.3 = !_10.3;
_10.1 = _6.0;
place!(Field::<u16>(Variant(_4, 1), 7)) = 3282_u16;
_10.3 = _12.3;
_18 = [_22.0,_22.0,RET,_17];
match _10.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
14581708 => bb8,
_ => bb6
}
}
bb8 = {
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2)).3 = 6614946135357650415_i64;
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2)).2 = [Field::<u16>(Variant(_4, 1), 7),Field::<u16>(Variant(_4, 1), 7),Field::<u16>(Variant(_4, 1), 7)];
_8 = -Field::<f64>(Variant(_4, 1), 1);
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2)).1 = _17;
_22.1 = Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2).3;
_6 = (_11.0,);
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2)).2 = [Field::<u16>(Variant(_4, 1), 7),Field::<u16>(Variant(_4, 1), 7),Field::<u16>(Variant(_4, 1), 7)];
_22 = (RET, Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2).3, _17, _11.0, _5);
_2 = !9223372036854775807_isize;
_1.0 = !_11.0;
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2)).0 = 1962_i16 & 18333_i16;
_25 = _2 * _2;
_12.2 = _10.2 * _10.2;
place!(Field::<f64>(Variant(_4, 1), 1)) = _8;
(*_16) = _10.0;
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2)).3 = _22.1 ^ _22.1;
_10 = _12;
_13 = [_22.2,Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2).1,Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2).1,_22.0,_17,Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2).1,_17];
_10.3 = !_12.3;
_24 = _25;
(*_16) = _10.0;
_22 = (RET, Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2).3, RET, _1.0, _5);
_17 = RET;
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2)).3 = 276859133276034938779080481898439337428_u128 as i64;
match _10.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
14581708 => bb10,
_ => bb9
}
}
bb9 = {
_10.0 = !3004366073_u32;
_11 = _1;
_10.1 = !_1.0;
_12.1 = _11.0 - Field::<usize>(Variant(_4, 0), 0);
_2 = 84_isize | (-51_isize);
_2 = 9223372036854775807_isize;
_10.3 = !218_u8;
_12.1 = !Field::<usize>(Variant(_4, 0), 0);
Goto(bb2)
}
bb10 = {
_24 = -_25;
_1.0 = _9;
_11.0 = Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2).0 as usize;
_1.0 = _3 as usize;
_15 = &_26;
_23 = [26_i8,(-102_i8)];
_12 = (_10.0, _10.1, _10.2, _10.3);
_10.1 = _12.3 as usize;
_11.0 = !_6.0;
_12.1 = _5 as usize;
_22.1 = -Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2).3;
_3 = _10.1 as f32;
_27.0 = !(*_16);
_22 = (Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2).1, Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2).3, RET, _9, _7);
_22.0 = _22.2;
_9 = _6.0 << _12.3;
Goto(bb11)
}
bb11 = {
_12.0 = _10.0;
_16 = core::ptr::addr_of!((*_16));
place!(Field::<[char; 7]>(Variant(_4, 1), 3)) = [RET,Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2).1,RET,_22.0,_17,_22.0,_17];
Call(_10.3 = core::intrinsics::transmute(_7), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_15 = &_26;
_29.fld3.3 = _12.1;
_11.0 = _12.1;
RET = Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2).1;
_26 = _3;
_10.3 = _12.3 ^ _12.3;
_29.fld3.3 = _6.0;
_12.0 = (-11707607956957811035854829966674157671_i128) as u32;
_8 = Field::<f64>(Variant(_4, 1), 1) * Field::<f64>(Variant(_4, 1), 1);
Call(_29.fld0 = fn13(_5, _9, _29.fld3.3, _11, _12, _29.fld3.3, _11.0, _10.0, _11.0, _26, _11, _12.3), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_33 = (_22.3,);
_18 = [Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2).1,RET,_22.0,_22.0];
place!(Field::<[char; 7]>(Variant(_4, 1), 3)) = [RET,_22.2,_22.0,_22.0,Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2).1,_29.fld0.1,RET];
match _10.0 {
0 => bb14,
1 => bb15,
2 => bb16,
14581708 => bb18,
_ => bb17
}
}
bb14 = {
_2 = (-95_isize);
_17 = RET;
_10 = (14581708_u32, _9, _12.2, 5_u8);
_10.2 = -_12.2;
_22.0 = _17;
_2 = 108_isize;
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2)).2 = [22287_u16,60184_u16,52598_u16];
_10.1 = !_11.0;
(*_16) = Field::<f64>(Variant(_4, 1), 1) as u32;
_2 = -9223372036854775807_isize;
_12.3 = !_10.3;
_10.1 = _6.0;
place!(Field::<u16>(Variant(_4, 1), 7)) = 3282_u16;
_10.3 = _12.3;
_18 = [_22.0,_22.0,RET,_17];
match _10.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
14581708 => bb8,
_ => bb6
}
}
bb15 = {
_12.0 = _10.0;
_16 = core::ptr::addr_of!((*_16));
place!(Field::<[char; 7]>(Variant(_4, 1), 3)) = [RET,Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2).1,RET,_22.0,_17,_22.0,_17];
Call(_10.3 = core::intrinsics::transmute(_7), ReturnTo(bb12), UnwindUnreachable())
}
bb16 = {
Return()
}
bb17 = {
_10.0 = !3004366073_u32;
_11 = _1;
_10.1 = !_1.0;
_12.1 = _11.0 - Field::<usize>(Variant(_4, 0), 0);
_2 = 84_isize | (-51_isize);
_2 = 9223372036854775807_isize;
_10.3 = !218_u8;
_12.1 = !Field::<usize>(Variant(_4, 0), 0);
Goto(bb2)
}
bb18 = {
_27.1 = !_33.0;
_29.fld3.2 = _17;
_9 = _27.1 & _33.0;
place!(Field::<(i16, char, [u16; 3], i64)>(Variant(_4, 1), 2)) = (_29.fld0.0, _17, _29.fld0.2, _29.fld0.3);
_21 = _10.1 as u64;
_10.1 = _9;
Goto(bb19)
}
bb19 = {
Call(_36 = dump_var(12_usize, 5_usize, Move(_5), 24_usize, Move(_24), 13_usize, Move(_13), 25_usize, Move(_25)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_36 = dump_var(12_usize, 7_usize, Move(_7), 1_usize, Move(_1), 23_usize, Move(_23), 33_usize, Move(_33)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_36 = dump_var(12_usize, 21_usize, Move(_21), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: bool,mut _2: usize,mut _3: usize,mut _4: (usize,),mut _5: (u32, usize, i32, u8),mut _6: usize,mut _7: usize,mut _8: u32,mut _9: usize,mut _10: f32,mut _11: (usize,),mut _12: u8) -> (i16, char, [u16; 3], i64) {
mir! {
type RET = (i16, char, [u16; 3], i64);
let _13: bool;
let _14: [u16; 5];
let _15: *const u32;
let _16: i32;
let _17: ();
let _18: ();
{
RET.1 = '\u{ee4f9}';
_2 = _6 - _3;
RET.3 = 3218542435060885768_i64;
_5.1 = !_7;
_11 = (_9,);
_5.1 = _5.2 as usize;
match _8 {
0 => bb1,
1 => bb2,
2 => bb3,
14581708 => bb5,
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
_7 = _3 ^ _6;
RET.2 = [53072_u16,54167_u16,55688_u16];
_4 = (_11.0,);
Call(_11.0 = core::intrinsics::transmute(_9), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_12 = _5.3;
RET.3 = (-105_i8) as i64;
RET.0 = -2462_i16;
_13 = _1 | _1;
RET.0 = !(-29665_i16);
Call(RET.2 = fn14(_2, _1, _4.0, _8, _7, _2, _5, _11.0), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_5 = (_8, _9, (-641168571_i32), _12);
_4.0 = _6;
RET.3 = 2143364058396743522_i64 >> _11.0;
_8 = _5.2 as u32;
_8 = RET.0 as u32;
RET.1 = '\u{b5cc1}';
_6 = _3 * _4.0;
RET.3 = 2542897041945357525_i64 - 6804733934924710683_i64;
RET.3 = 4089054400279805556_i64 >> _6;
_3 = _9;
_10 = 31267_u16 as f32;
RET.3 = _2 as i64;
_2 = _9 ^ _7;
_7 = !_4.0;
RET.0 = (-685_i16) - 30352_i16;
RET.0 = 30446_i16 & 6107_i16;
_11.0 = _2 << _3;
_2 = !_3;
RET.3 = (-6575004101294075040_i64);
Goto(bb8)
}
bb8 = {
Call(_17 = dump_var(13_usize, 11_usize, Move(_11), 1_usize, Move(_1), 9_usize, Move(_9), 13_usize, Move(_13)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_17 = dump_var(13_usize, 12_usize, Move(_12), 7_usize, Move(_7), 18_usize, _18, 18_usize, _18), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: usize,mut _2: bool,mut _3: usize,mut _4: u32,mut _5: usize,mut _6: usize,mut _7: (u32, usize, i32, u8),mut _8: usize) -> [u16; 3] {
mir! {
type RET = [u16; 3];
let _9: (i16, char, [u16; 3], i64);
let _10: Adt59;
let _11: isize;
let _12: char;
let _13: char;
let _14: isize;
let _15: (char, i64, char, usize, bool);
let _16: f64;
let _17: Adt59;
let _18: usize;
let _19: ((i16, char, [u16; 3], i64), (*const *mut f64, i128), usize);
let _20: u16;
let _21: (char, i64, char, usize, bool);
let _22: i128;
let _23: bool;
let _24: bool;
let _25: Adt54;
let _26: [char; 7];
let _27: &'static f32;
let _28: isize;
let _29: *const *const *mut f64;
let _30: u16;
let _31: bool;
let _32: [usize; 3];
let _33: [u16; 3];
let _34: usize;
let _35: usize;
let _36: u8;
let _37: i64;
let _38: f32;
let _39: f32;
let _40: [i8; 2];
let _41: [i32; 8];
let _42: f64;
let _43: f32;
let _44: Adt57;
let _45: isize;
let _46: u8;
let _47: isize;
let _48: Adt52;
let _49: ();
let _50: ();
{
RET = [46638_u16,60068_u16,36884_u16];
_7.3 = 251_u8 * 39_u8;
RET = [27148_u16,9451_u16,6808_u16];
_8 = !_1;
_7.0 = !_4;
_5 = _1;
RET = [22337_u16,29889_u16,20721_u16];
_2 = true;
_7.1 = _1;
_7.0 = !_4;
_3 = !_8;
_1 = _7.1 ^ _8;
_4 = !_7.0;
_7.0 = _4 << _1;
_5 = !_6;
_9.2 = [33523_u16,23837_u16,27685_u16];
RET = [13912_u16,43845_u16,30305_u16];
_9.0 = _3 as i16;
_6 = 116896321486804703489642092960407033155_u128 as usize;
Call(_8 = core::intrinsics::transmute(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = 9223372036854775807_isize as usize;
_7.2 = 795288440_i32;
_9.1 = '\u{2cd70}';
_10.fld3 = 139433496641255086073554037626360333375_u128 as usize;
_8 = _9.0 as usize;
_7.2 = (-969377907_i32);
RET = [5373_u16,11593_u16,25690_u16];
_7 = (_4, _1, (-1173617810_i32), 59_u8);
_12 = _9.1;
_12 = _9.1;
_2 = false;
_14 = 9223372036854775807_isize * (-9223372036854775808_isize);
_15.1 = _3 as i64;
_13 = _9.1;
_4 = _7.0 | _7.0;
RET = _9.2;
_4 = _7.0 | _7.0;
_15.4 = _9.0 >= _9.0;
_15.3 = _1;
_3 = _7.0 as usize;
_7 = (_4, _15.3, (-2050009378_i32), 181_u8);
_9 = (195_i16, _13, RET, _15.1);
_7.3 = _13 as u8;
Goto(bb2)
}
bb2 = {
RET = [63301_u16,21993_u16,61778_u16];
_16 = _15.3 as f64;
_6 = !_3;
RET = _9.2;
_15 = (_13, _9.3, _13, _1, _2);
_9.2 = [14953_u16,55921_u16,43491_u16];
RET = [40675_u16,36039_u16,52925_u16];
_3 = _8;
_7.1 = _14 as usize;
_3 = !_6;
_19.0.1 = _12;
_9.1 = _15.0;
_15.4 = _7.2 <= _7.2;
_18 = _9.0 as usize;
_11 = !_14;
_19.2 = _11 as usize;
_10.fld3 = 4022892371036800905_u64 as usize;
_15.4 = _18 >= _1;
_15.4 = !_2;
_7 = (_4, _18, (-1998796212_i32), 157_u8);
_12 = _15.0;
_4 = _7.3 as u32;
_12 = _9.1;
_14 = _11;
Call(_8 = fn15(_7.3, _4, _16, _7.0, _15.3, _4, _7.1, _15.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_19.0.3 = 83164319137129585380683267665613404741_i128 as i64;
_3 = _8 & _15.3;
_15.4 = _3 >= _15.3;
_9.0 = (-26167_i16);
_10.fld3 = !_15.3;
_19.0.3 = -_15.1;
_9.1 = _15.2;
_9.1 = _15.0;
_5 = !_1;
_23 = _15.4;
_15.0 = _19.0.1;
_21.3 = _8;
_18 = !_15.3;
_13 = _15.0;
_21.1 = _9.3;
_14 = _16 as isize;
_8 = !_5;
_4 = _7.0;
_2 = !_15.4;
_11 = _14;
_9.3 = _15.1;
_15.1 = _19.0.3;
_15.1 = _19.0.3;
match _7.2 {
0 => bb1,
340282366920938463463374607429769415244 => bb4,
_ => bb2
}
}
bb4 = {
_7.2 = -(-1684949930_i32);
_21.0 = _9.1;
_23 = _8 != _6;
_17.fld3 = !_7.1;
_19.0.0 = _9.0;
_18 = 50594_u16 as usize;
_6 = !_10.fld3;
_19.2 = _7.1;
_15.1 = -_21.1;
_21.4 = !_2;
_19.2 = !_10.fld3;
_15 = (_19.0.1, _9.3, _12, _1, _21.4);
Goto(bb5)
}
bb5 = {
_26 = [_21.0,_15.2,_9.1,_21.0,_9.1,_12,_9.1];
_7.3 = 158_u8;
RET = [44866_u16,56036_u16,26619_u16];
_19.1.1 = (-63205245267539538671576537781587494666_i128) << _15.1;
_21.3 = 260608574553938266841233812417045582637_u128 as usize;
_7.2 = -(-1306079954_i32);
_28 = !_14;
RET = _9.2;
_16 = 37578_u16 as f64;
_19.0.3 = !_21.1;
_19.0.1 = _15.2;
_22 = _19.1.1;
_19.0.1 = _12;
_1 = _3;
_15.3 = _19.2;
_15.0 = _13;
_10.fld2 = Adt57::Variant3 { fld0: _19.1.1 };
_24 = !_21.4;
_18 = !_3;
_10.fld3 = 114_i8 as usize;
_7.3 = 112_u8;
_24 = _14 <= _14;
Goto(bb6)
}
bb6 = {
_19.2 = _3;
_30 = 16171_u16 >> _9.3;
_17.fld2 = Adt57::Variant3 { fld0: _22 };
_10.fld3 = 272903917692426476125279345370624101947_u128 as usize;
place!(Field::<i128>(Variant(_17.fld2, 3), 0)) = _22;
_7.0 = _4;
_28 = _19.0.0 as isize;
_20 = _15.1 as u16;
_9.1 = _15.2;
_26 = [_13,_9.1,_15.0,_19.0.1,_12,_12,_15.0];
_2 = _21.4 & _15.4;
_12 = _15.0;
_13 = _15.2;
place!(Field::<i128>(Variant(_17.fld2, 3), 0)) = _22;
_21.2 = _15.0;
_17.fld2 = Move(_10.fld2);
_7.0 = _4 | _4;
match _19.0.0 {
0 => bb5,
1 => bb2,
340282366920938463463374607431768185289 => bb7,
_ => bb3
}
}
bb7 = {
_19.0.2 = [_20,_30,_20];
_7.3 = 123_u8;
_16 = _8 as f64;
_19.1.1 = _22;
_10.fld2 = Move(_17.fld2);
match _9.0 {
0 => bb1,
1 => bb8,
340282366920938463463374607431768185289 => bb10,
_ => bb9
}
}
bb8 = {
RET = [63301_u16,21993_u16,61778_u16];
_16 = _15.3 as f64;
_6 = !_3;
RET = _9.2;
_15 = (_13, _9.3, _13, _1, _2);
_9.2 = [14953_u16,55921_u16,43491_u16];
RET = [40675_u16,36039_u16,52925_u16];
_3 = _8;
_7.1 = _14 as usize;
_3 = !_6;
_19.0.1 = _12;
_9.1 = _15.0;
_15.4 = _7.2 <= _7.2;
_18 = _9.0 as usize;
_11 = !_14;
_19.2 = _11 as usize;
_10.fld3 = 4022892371036800905_u64 as usize;
_15.4 = _18 >= _1;
_15.4 = !_2;
_7 = (_4, _18, (-1998796212_i32), 157_u8);
_12 = _15.0;
_4 = _7.3 as u32;
_12 = _9.1;
_14 = _11;
Call(_8 = fn15(_7.3, _4, _16, _7.0, _15.3, _4, _7.1, _15.1), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_26 = [_21.0,_15.2,_9.1,_21.0,_9.1,_12,_9.1];
_7.3 = 158_u8;
RET = [44866_u16,56036_u16,26619_u16];
_19.1.1 = (-63205245267539538671576537781587494666_i128) << _15.1;
_21.3 = 260608574553938266841233812417045582637_u128 as usize;
_7.2 = -(-1306079954_i32);
_28 = !_14;
RET = _9.2;
_16 = 37578_u16 as f64;
_19.0.3 = !_21.1;
_19.0.1 = _15.2;
_22 = _19.1.1;
_19.0.1 = _12;
_1 = _3;
_15.3 = _19.2;
_15.0 = _13;
_10.fld2 = Adt57::Variant3 { fld0: _19.1.1 };
_24 = !_21.4;
_18 = !_3;
_10.fld3 = 114_i8 as usize;
_7.3 = 112_u8;
_24 = _14 <= _14;
Goto(bb6)
}
bb10 = {
_19.0.3 = _21.1;
_15 = (_19.0.1, _19.0.3, _21.2, _5, _23);
_19.1.1 = -_22;
_8 = _14 as usize;
_15.4 = !_23;
_32 = [_8,_15.3,_19.2];
_30 = _16 as u16;
_31 = _23;
_24 = _23;
_29 = core::ptr::addr_of!(_19.1.0);
_8 = _6 * _17.fld3;
_6 = Field::<i128>(Variant(_10.fld2, 3), 0) as usize;
_7.3 = 31_i8 as u8;
match _19.0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb9,
5 => bb6,
340282366920938463463374607431768185289 => bb11,
_ => bb8
}
}
bb11 = {
_39 = _7.0 as f32;
_9.2 = _19.0.2;
_19.0.2 = [_30,_20,_30];
_27 = &_38;
_7.3 = (-24_i8) as u8;
_19.2 = _7.1;
_37 = -_21.1;
_7.3 = _30 as u8;
_15.3 = _4 as usize;
_2 = _15.4;
_15.0 = _19.0.1;
_20 = _30 + _30;
_17.fld2 = Move(_10.fld2);
_14 = _11;
_17.fld3 = _8;
_37 = !_15.1;
_40 = [(-19_i8),(-69_i8)];
_21.0 = _12;
_19.1.1 = _22;
_7.2 = 852035684_i32 - 1288297532_i32;
_21.3 = !_19.2;
_9.0 = !_19.0.0;
_3 = !_19.2;
Goto(bb12)
}
bb12 = {
_22 = -_19.1.1;
_23 = _31 & _21.4;
_35 = _16 as usize;
_1 = _15.3 - _35;
_19.0.0 = _9.0 << _9.3;
_7.0 = _16 as u32;
_43 = 1521870421633822821_u64 as f32;
Goto(bb13)
}
bb13 = {
_33 = _9.2;
_14 = !_11;
_15 = (_19.0.1, _37, _21.2, _8, _2);
_3 = _2 as usize;
_31 = _15.4;
SetDiscriminant(_17.fld2, 3);
_32 = [_7.1,_15.3,_7.1];
_46 = _7.3;
RET = [_30,_20,_20];
_21.1 = _9.3;
_41 = [_7.2,_7.2,_7.2,_7.2,_7.2,_7.2,_7.2,_7.2];
_42 = _16;
_32 = [_19.2,_8,_15.3];
Goto(bb14)
}
bb14 = {
Call(_49 = dump_var(14_usize, 35_usize, Move(_35), 14_usize, Move(_14), 2_usize, Move(_2), 11_usize, Move(_11)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_49 = dump_var(14_usize, 13_usize, Move(_13), 18_usize, Move(_18), 24_usize, Move(_24), 41_usize, Move(_41)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_49 = dump_var(14_usize, 37_usize, Move(_37), 26_usize, Move(_26), 4_usize, Move(_4), 46_usize, Move(_46)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = dump_var(14_usize, 32_usize, Move(_32), 30_usize, Move(_30), 6_usize, Move(_6), 50_usize, _50), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: u8,mut _2: u32,mut _3: f64,mut _4: u32,mut _5: usize,mut _6: u32,mut _7: usize,mut _8: i64) -> usize {
mir! {
type RET = usize;
let _9: i32;
let _10: bool;
let _11: f64;
let _12: ();
let _13: ();
{
RET = (-1131892724_i32) as usize;
Goto(bb1)
}
bb1 = {
_1 = !81_u8;
_2 = !_4;
_8 = 5717649624226518969_i64 + 6033001181387532684_i64;
_2 = !_4;
_5 = '\u{403ad}' as usize;
_2 = (-34_i8) as u32;
_7 = RET + RET;
_4 = 330314858152490336023893176513227411844_u128 as u32;
_3 = 50575_u16 as f64;
_3 = _6 as f64;
RET = _7;
_3 = _6 as f64;
_4 = _6;
_6 = !_2;
_7 = _5;
_4 = _2 - _2;
_9 = -1334453709_i32;
_9 = (-406483795_i32);
_4 = _6;
match _9 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431361727661 => bb8,
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
RET = _3 as usize;
_6 = _4;
_4 = _2 - _6;
_3 = 110_isize as f64;
_5 = RET;
Goto(bb9)
}
bb9 = {
Call(_12 = dump_var(15_usize, 2_usize, Move(_2), 9_usize, Move(_9), 6_usize, Move(_6), 7_usize, Move(_7)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: [u16; 5],mut _7: bool,mut _8: bool) -> bool {
mir! {
type RET = bool;
let _9: i32;
let _10: u32;
let _11: (i16, char, [u16; 3], i64);
let _12: f32;
let _13: ();
let _14: ();
{
RET = _2 > _7;
_3 = _1;
_1 = !_7;
_9 = 2150431576937767365_u64 as i32;
_5 = _1;
_1 = _2 <= _4;
_6 = [60219_u16,16549_u16,36070_u16,57035_u16,38759_u16];
_9 = (-1654741551_i32);
_11.2 = [38738_u16,18598_u16,50267_u16];
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(16_usize, 7_usize, Move(_7), 2_usize, Move(_2), 6_usize, Move(_6), 4_usize, Move(_4)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: isize,mut _2: u128,mut _3: usize,mut _4: usize,mut _5: [u16; 3],mut _6: usize) -> i64 {
mir! {
type RET = i64;
let _7: u128;
let _8: Adt58;
let _9: i32;
let _10: f32;
let _11: isize;
let _12: [u16; 3];
let _13: Adt45;
let _14: (u32, usize, i32, u8);
let _15: f32;
let _16: u128;
let _17: [char; 4];
let _18: (usize,);
let _19: bool;
let _20: u32;
let _21: [i128; 2];
let _22: isize;
let _23: [i32; 8];
let _24: [usize; 3];
let _25: *const i64;
let _26: ();
let _27: ();
{
_6 = _3 & _4;
RET = 8562146487170216358_i64;
_6 = _3;
_4 = 2529982591_u32 as usize;
_7 = true as u128;
_8.fld0 = (97_i8,);
RET = 1359178362_u32 as i64;
_8.fld0 = (0_i8,);
_1 = _2 as isize;
RET = (-6358661383139696062_i64) >> _6;
_8.fld1 = '\u{cbc59}';
_8.fld2 = _6 as isize;
_4 = true as usize;
Goto(bb1)
}
bb1 = {
_11 = _8.fld2;
_10 = _8.fld2 as f32;
_2 = _7;
_8.fld2 = -_11;
_8.fld0.0 = 68_i8 * 99_i8;
Call(_14.1 = core::intrinsics::bswap(_4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8.fld2 = -_1;
_4 = 11428593437863714166_u64 as usize;
_8.fld0.0 = (-73_i8) + 91_i8;
_14.3 = 240_u8;
_11 = _8.fld2 << _4;
_14.0 = 3982931675_u32 ^ 2473221849_u32;
_6 = !_3;
_8.fld0.0 = RET as i8;
_2 = _10 as u128;
_16 = !_7;
_3 = 18708_i16 as usize;
_14.0 = 2817232309_u32 & 1687699363_u32;
_1 = _8.fld2;
_14.1 = _3;
_8.fld0.0 = (-95_i8) + 99_i8;
_5 = [17251_u16,40552_u16,44759_u16];
_15 = _11 as f32;
_17 = [_8.fld1,_8.fld1,_8.fld1,_8.fld1];
_14.0 = 375977176_u32 | 1876069278_u32;
_9 = _14.3 as i32;
_14.2 = !_9;
_14.3 = 76464727229897143372329070851882789860_i128 as u8;
_9 = _14.2 >> _2;
_14 = (2763935640_u32, _4, _9, 187_u8);
_8.fld0 = ((-68_i8),);
match _14.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
2763935640 => bb10,
_ => bb9
}
}
bb3 = {
_11 = _8.fld2;
_10 = _8.fld2 as f32;
_2 = _7;
_8.fld2 = -_11;
_8.fld0.0 = 68_i8 * 99_i8;
Call(_14.1 = core::intrinsics::bswap(_4), ReturnTo(bb2), UnwindUnreachable())
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
_3 = !_6;
_1 = -_11;
_14 = (3542521157_u32, _3, _9, 12_u8);
_13 = Adt45::Variant0 { fld0: true };
_18 = (_3,);
Call(_14.1 = core::intrinsics::bswap(_18.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_14.2 = !_9;
_7 = !_16;
_14.0 = !2399778873_u32;
_19 = false;
place!(Field::<bool>(Variant(_13, 0), 0)) = !_19;
_7 = _16;
Goto(bb12)
}
bb12 = {
_9 = _14.2 << _14.3;
place!(Field::<bool>(Variant(_13, 0), 0)) = _14.3 > _14.3;
_8.fld0 = (126_i8,);
_11 = _8.fld2;
_20 = (-140414480749815215953757190162268503905_i128) as u32;
_18 = (_14.1,);
_19 = !Field::<bool>(Variant(_13, 0), 0);
_9 = _14.2;
_7 = RET as u128;
_1 = _14.3 as isize;
RET = -(-5096524357514049742_i64);
_3 = !_18.0;
_6 = _3 * _3;
place!(Field::<bool>(Variant(_13, 0), 0)) = _19;
_14.1 = _4;
_6 = 7911303559976353253_u64 as usize;
_18.0 = _4 | _4;
_15 = _10;
match _14.3 {
12 => bb13,
_ => bb6
}
}
bb13 = {
_12 = _5;
_17 = [_8.fld1,_8.fld1,_8.fld1,_8.fld1];
_8.fld1 = '\u{c5ecf}';
_5 = _12;
RET = 5181524412101892777_i64 >> _16;
_18.0 = _3 | _3;
Goto(bb14)
}
bb14 = {
_6 = _4;
place!(Field::<bool>(Variant(_13, 0), 0)) = _19;
_19 = Field::<bool>(Variant(_13, 0), 0);
_11 = _1 >> _20;
_10 = _15 + _15;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(17_usize, 12_usize, Move(_12), 9_usize, Move(_9), 18_usize, Move(_18), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(17_usize, 4_usize, Move(_4), 2_usize, Move(_2), 16_usize, Move(_16), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box(262547314916190444964048862460261995086_u128), std::hint::black_box(43_u8), std::hint::black_box(83_i8), std::hint::black_box((-24939_i16)), std::hint::black_box((-728214534_i32)), std::hint::black_box(8472056640962405890_u64));
                
            }
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: bool,

},
Variant1{
fld0: ((i16, char, [u16; 3], i64), (*const *mut f64, i128), usize),
fld1: char,
fld2: [u16; 3],
fld3: (char, i64, char, usize, bool),
fld4: *const *const *mut f64,
fld5: usize,

},
Variant2{
fld0: u128,
fld1: [char; 7],
fld2: (u32, usize, i32, u8),
fld3: *const u16,
fld4: [u16; 3],
fld5: f64,

},
Variant3{
fld0: (char, i64, char, usize, bool),
fld1: *mut i32,
fld2: (u32, usize, i32, u8),
fld3: [isize; 8],
fld4: u128,
fld5: f64,
fld6: u16,
fld7: *const *mut f64,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: (i16, char, [u16; 3], i64),
fld1: [isize; 8],
fld2: Adt45,
fld3: (char, i64, char, usize, bool),
fld4: i16,
fld5: i32,
fld6: i64,
}
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: *mut [char; 7],
fld1: [i8; 2],

},
Variant1{
fld0: [i32; 8],
fld1: (u32, usize, i32, u8),
fld2: [u16; 5],
fld3: *mut [char; 7],

},
Variant2{
fld0: *mut i32,

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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: (i8,),
fld1: u8,
fld2: [i128; 2],
fld3: i128,
fld4: i16,
fld5: i32,

},
Variant1{
fld0: bool,
fld1: [i128; 2],
fld2: u8,
fld3: (u32, usize, i32, u8),
fld4: i16,
fld5: *mut u8,
fld6: u32,
fld7: (i8,),

},
Variant2{
fld0: (char, i64, char, usize, bool),
fld1: [i128; 2],
fld2: isize,
fld3: usize,

},
Variant3{
fld0: *mut f64,
fld1: (char, i64, char, usize, bool),
fld2: *const *mut f64,
fld3: (u32, usize, i32, u8),
fld4: [char; 4],

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: bool,
fld1: f32,
fld2: (u32, usize, i32, u8),
fld3: *const u32,
fld4: *mut *mut u8,
fld5: usize,
fld6: ((i16, char, [u16; 3], i64), (*const *mut f64, i128), usize),
fld7: u16,

},
Variant1{
fld0: bool,
fld1: *const u32,
fld2: *const *const *mut f64,
fld3: (u32, usize, i32, u8),
fld4: i64,
fld5: *mut i32,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: *mut *mut u8,
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: [i8; 2],
fld1: [isize; 8],
fld2: (u32, usize, i32, u8),
fld3: *const *const *mut f64,
fld4: [u16; 3],
fld5: i32,

},
Variant1{
fld0: char,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: Adt46,
fld1: Adt47,
fld2: (char, i64, char, usize, bool),
fld3: *mut f64,

},
Variant1{
fld0: Adt50,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: usize,

},
Variant1{
fld0: *mut *mut u8,
fld1: f64,
fld2: (i16, char, [u16; 3], i64),
fld3: [char; 7],
fld4: Adt47,
fld5: Adt50,
fld6: *const u16,
fld7: u16,

},
Variant2{
fld0: u128,
fld1: (u32, usize, i32, u8),
fld2: u64,
fld3: *const u32,
fld4: Adt45,

},
Variant3{
fld0: *mut i32,
fld1: Adt45,
fld2: (u32, usize, i32, u8),

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt50,

},
Variant1{
fld0: u8,
fld1: *mut u8,

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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: [u16; 5],
fld1: char,
fld2: Adt46,
fld3: [i128; 2],
fld4: f32,
fld5: Adt47,

},
Variant1{
fld0: [char; 4],
fld1: (usize,),
fld2: (char, *const u32),

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: *mut u8,
fld1: f64,

},
Variant1{
fld0: Adt49,
fld1: char,
fld2: Adt47,

},
Variant2{
fld0: *const *const *mut f64,
fld1: u128,

},
Variant3{
fld0: Adt51,
fld1: *const u32,
fld2: [isize; 8],
fld3: [i128; 2],
fld4: i128,

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
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: *mut u8,
fld1: Adt54,
fld2: u16,
fld3: [i32; 8],
fld4: Adt52,

},
Variant1{
fld0: *mut [char; 7],

},
Variant2{
fld0: Adt52,
fld1: char,
fld2: Adt54,
fld3: u64,
fld4: (usize,),

},
Variant3{
fld0: i128,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt58{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt58 {
fld0: (i8,),
fld1: char,
fld2: isize,
fld3: Adt52,
}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt59{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt59 {
fld0: Adt49,
fld1: *mut *mut u8,
fld2: Adt57,
fld3: usize,
}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt60{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt60 {
fld0: Adt53,
}
impl PrintFDebug for Adt61{
	unsafe fn printf_debug(&self){unsafe{printf("Adt61::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt61 {
Variant0{
fld0: *mut *mut u8,

},
Variant1{
fld0: [isize; 8],
fld1: *mut *mut u8,
fld2: Adt54,
fld3: ((i16, char, [u16; 3], i64), (*const *mut f64, i128), usize),
fld4: *mut i32,

},
Variant2{
fld0: [char; 7],
fld1: u16,
fld2: Adt45,

},
Variant3{
fld0: (i8,),
fld1: Adt53,

}}

