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
pub fn fn0(mut _1: bool,mut _2: u8,mut _3: isize,mut _4: usize,mut _5: i64) -> bool {
mir! {
type RET = bool;
let _6: i32;
let _7: i64;
let _8: char;
let _9: i64;
let _10: isize;
let _11: i128;
let _12: (f64, (*const i16, (u8, u64, (u8, i64, u64)), [i32; 3]), char, &'static isize);
let _13: ();
let _14: ();
{
RET = false;
_3 = 19078021340008844362864282538929347974_u128 as isize;
_4 = !5_usize;
_5 = -(-8435239522887660808_i64);
_2 = 165_u8;
_2 = !76_u8;
_4 = !3223595178762734861_usize;
_2 = 160_u8;
_4 = !5_usize;
_6 = -(-1513943927_i32);
_2 = 158_u8 ^ 217_u8;
_1 = RET;
_2 = !64_u8;
RET = !_1;
_3 = !(-104_isize);
_5 = (-5903653127666658803_i64);
_3 = '\u{95396}' as isize;
RET = !_1;
_6 = (-471655331_i32) ^ 455256593_i32;
_6 = 1821042206_i32 + 43266290_i32;
_3 = (-9223372036854775808_isize) << _5;
_5 = 3137732875961135322_i64;
_4 = 1_usize << _3;
_2 = 223_u8;
_1 = !RET;
_5 = (-2154524469610224886_i64);
Call(_5 = fn1(_3, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = (-9102234909972474292_i64);
_6 = 61835395_i32 | (-1519239614_i32);
_4 = '\u{e7db}' as usize;
_2 = 41_u8 - 111_u8;
_3 = 111_isize;
_3 = 97_isize & 91_isize;
match _5 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463454272372521795737164 => bb10,
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
_6 = 1217569132_i32 & 2036072536_i32;
_5 = _2 as i64;
_8 = '\u{a2e25}';
_6 = -2136732848_i32;
_1 = _2 < _2;
_6 = !(-834878611_i32);
_7 = _2 as i64;
_9 = _7 & _7;
_4 = 12599717102913973514_usize << _9;
_1 = RET;
Goto(bb11)
}
bb11 = {
_8 = '\u{69f1e}';
_4 = 1876388325510811302_usize >> _2;
RET = _4 < _4;
_10 = 26330_i16 as isize;
_7 = _9 | _9;
_1 = !RET;
_1 = RET;
RET = _1 & _1;
_2 = 7_u8 & 245_u8;
_6 = (-759403949_i32) & 1605316615_i32;
Goto(bb12)
}
bb12 = {
Call(_13 = dump_var(0_usize, 9_usize, Move(_9), 2_usize, Move(_2), 1_usize, Move(_1), 8_usize, Move(_8)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_13 = dump_var(0_usize, 3_usize, Move(_3), 14_usize, _14, 14_usize, _14, 14_usize, _14), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: isize,mut _2: usize) -> i64 {
mir! {
type RET = i64;
let _3: f32;
let _4: i8;
let _5: u64;
let _6: bool;
let _7: char;
let _8: &'static &'static i128;
let _9: [i8; 7];
let _10: isize;
let _11: f64;
let _12: isize;
let _13: (i128, *const isize);
let _14: [i8; 7];
let _15: (Adt34, &'static *mut &'static u64, ((i128, *const isize), *mut u128, Adt44));
let _16: &'static Adt44;
let _17: (u8, i64, u64);
let _18: i32;
let _19: (Adt34, &'static *mut &'static u64, ((i128, *const isize), *mut u128, Adt44));
let _20: *const (i128, *const isize);
let _21: &'static Adt50;
let _22: *const *const isize;
let _23: [i8; 7];
let _24: Adt27;
let _25: *const i16;
let _26: i32;
let _27: [bool; 5];
let _28: ();
let _29: ();
{
_3 = 8010770388516283760_i64 as f32;
_2 = !2_usize;
_3 = (-1397545053_i32) as f32;
_2 = 12282576242105855859_usize;
_1 = 193660345680462720049727112733529864304_u128 as isize;
_2 = 170_u8 as usize;
RET = (-7317903202945108648_i64);
_6 = false;
_5 = 4260744150258788988_u64;
_7 = '\u{cec9d}';
_2 = !1_usize;
_3 = (-1920775318_i32) as f32;
_2 = 6_usize;
RET = (-8387209817019414571_i64) << _2;
_5 = 18201112267424515250_u64 | 11131895550437997004_u64;
_4 = 104_i8 ^ 111_i8;
_4 = (-33_i8);
_2 = !4_usize;
Call(_3 = fn2(_2, _6, _2, _7, _6, _5, _1, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = 5947340104939893701_i64 * 111362147149774392_i64;
_4 = _3 as i8;
_2 = !5150274115032835445_usize;
_5 = (-144652685706162878146804532183861230292_i128) as u64;
_4 = -(-106_i8);
_7 = '\u{374e5}';
RET = -1364045170706358904_i64;
_7 = '\u{e3b6e}';
RET = 6642161111313871744_i64;
_4 = 97_i8 * 70_i8;
_1 = (-55_isize);
RET = 5827973410338852283_i64;
_3 = 16214925932750706909467509746369878067_i128 as f32;
_1 = !9223372036854775807_isize;
RET = _3 as i64;
_2 = (-112072666139736801122283581982514343545_i128) as usize;
_1 = !(-9223372036854775808_isize);
_5 = 113_u8 as u64;
_5 = 1071074943469972083_u64;
_9 = [_4,_4,_4,_4,_4,_4,_4];
_3 = 3177917963_u32 as f32;
match _5 {
0 => bb2,
1 => bb3,
2 => bb4,
1071074943469972083 => bb6,
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
_1 = 9223372036854775807_isize & (-9223372036854775808_isize);
RET = 5716350668358325754_i64 + 3599518781844441166_i64;
_7 = '\u{c07a3}';
_13.0 = (-104706627139616302174419057617211891525_i128) | (-75064134609146401427746228043401676272_i128);
_15.0.fld1.3 = 3445239433_u32;
_15.0.fld2 = !_2;
_15.0.fld1.2 = (-1084198546_i32);
_15.0.fld5 = [RET,RET,RET,RET];
_15.0.fld1.3 = 119561164_u32;
_12 = _1 * _1;
match _15.0.fld1.3 {
0 => bb7,
1 => bb8,
119561164 => bb10,
_ => bb9
}
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
_15.0.fld6 = [_15.0.fld1.2,_15.0.fld1.2,_15.0.fld1.2];
_15.0.fld4 = _1 as i16;
_13.1 = core::ptr::addr_of!(_10);
_15.2.0 = Move(_13);
_17.2 = _5 * _5;
_1 = _12 + _12;
Goto(bb11)
}
bb11 = {
_13.0 = _15.2.0.0;
_13 = Move(_15.2.0);
_11 = _15.0.fld4 as f64;
_13.1 = core::ptr::addr_of!(_10);
_15.0.fld1.0 = _13.0 + _13.0;
_13.0 = _15.0.fld4 as i128;
_15.0.fld1.2 = _6 as i32;
_14 = [_4,_4,_4,_4,_4,_4,_4];
_15.0.fld4 = 12491_i16 << _15.0.fld2;
_15.2.0.1 = core::ptr::addr_of!(_1);
_9 = _14;
_15.0.fld1 = (_13.0, _7, (-582878583_i32), 3708827813_u32);
_5 = _17.2;
match _15.0.fld1.3 {
0 => bb9,
1 => bb2,
2 => bb3,
3 => bb7,
3708827813 => bb12,
_ => bb8
}
}
bb12 = {
_15.0.fld5 = [RET,RET,RET,RET];
_10 = _12;
_15.0.fld3 = Adt27::Variant0 { fld0: _15.0.fld1,fld1: _15.0.fld1.0,fld2: _15.0.fld4,fld3: 20066_u16 };
_6 = _4 < _4;
_17 = (228_u8, RET, _5);
_15.0.fld1.1 = Field::<(i128, char, i32, u32)>(Variant(_15.0.fld3, 0), 0).1;
RET = -_17.1;
_15.0.fld1.2 = Field::<(i128, char, i32, u32)>(Variant(_15.0.fld3, 0), 0).2 + Field::<(i128, char, i32, u32)>(Variant(_15.0.fld3, 0), 0).2;
_15.0.fld3 = Adt27::Variant0 { fld0: _15.0.fld1,fld1: _13.0,fld2: _15.0.fld4,fld3: 61728_u16 };
_15.0.fld1.2 = !Field::<(i128, char, i32, u32)>(Variant(_15.0.fld3, 0), 0).2;
_15.0.fld5 = [RET,RET,RET,RET];
_15.0.fld5 = [_17.1,RET,_17.1,RET];
_19.2.0 = Move(_13);
match _17.0 {
0 => bb11,
1 => bb5,
228 => bb13,
_ => bb7
}
}
bb13 = {
_11 = _19.2.0.0 as f64;
RET = _2 as i64;
_13 = (Field::<(i128, char, i32, u32)>(Variant(_15.0.fld3, 0), 0).0, Move(_15.2.0.1));
_19.2.0 = (_15.0.fld1.0, Move(_13.1));
_19.0.fld1.2 = -_15.0.fld1.2;
place!(Field::<(i128, char, i32, u32)>(Variant(_15.0.fld3, 0), 0)).3 = _6 as u32;
_10 = -_1;
_15.0.fld0 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_15.0.fld3, 0), 2)));
_15.2.0.0 = _19.2.0.0;
_15.0.fld0 = core::ptr::addr_of!(_19.0.fld4);
_19.0.fld1 = (_19.2.0.0, _15.0.fld1.1, _15.0.fld1.2, Field::<(i128, char, i32, u32)>(Variant(_15.0.fld3, 0), 0).3);
_17.0 = 39_u8 + 171_u8;
_13.0 = _17.0 as i128;
_19.0.fld5 = [RET,_17.1,_17.1,RET];
place!(Field::<i16>(Variant(_15.0.fld3, 0), 2)) = _10 as i16;
_6 = true;
_19.2.0.0 = _15.0.fld1.0 << _1;
place!(Field::<(i128, char, i32, u32)>(Variant(_15.0.fld3, 0), 0)).0 = -_19.0.fld1.0;
_22 = core::ptr::addr_of!(_19.2.0.1);
_13.0 = _17.1 as i128;
Goto(bb14)
}
bb14 = {
_10 = _12 | _12;
_14 = [_4,_4,_4,_4,_4,_4,_4];
_15.0.fld1.1 = Field::<(i128, char, i32, u32)>(Variant(_15.0.fld3, 0), 0).1;
_20 = core::ptr::addr_of!(_15.2.0);
_23 = [_4,_4,_4,_4,_4,_4,_4];
place!(Field::<(i128, char, i32, u32)>(Variant(_15.0.fld3, 0), 0)).2 = _15.0.fld1.2 | _19.0.fld1.2;
_15.0.fld6 = [Field::<(i128, char, i32, u32)>(Variant(_15.0.fld3, 0), 0).2,Field::<(i128, char, i32, u32)>(Variant(_15.0.fld3, 0), 0).2,_19.0.fld1.2];
(*_20).1 = core::ptr::addr_of!(_10);
_15.0.fld0 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_15.0.fld3, 0), 2)));
_13.0 = _19.2.0.0 - _19.2.0.0;
(*_22) = core::ptr::addr_of!(_1);
_15.0.fld0 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_15.0.fld3, 0), 2)));
RET = _17.1 << Field::<(i128, char, i32, u32)>(Variant(_15.0.fld3, 0), 0).3;
_19.0.fld2 = _15.0.fld2 - _15.0.fld2;
_1 = _10;
_17 = (169_u8, RET, _5);
_13.1 = Move(_19.2.0.1);
_27 = [_6,_6,_6,_6,_6];
place!(Field::<(i128, char, i32, u32)>(Variant(_15.0.fld3, 0), 0)).0 = !_13.0;
_11 = _19.2.0.0 as f64;
_9 = [_4,_4,_4,_4,_4,_4,_4];
_19.2.0.1 = core::ptr::addr_of!(_12);
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(1_usize, 4_usize, Move(_4), 17_usize, Move(_17), 27_usize, Move(_27), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(1_usize, 6_usize, Move(_6), 14_usize, Move(_14), 29_usize, _29, 29_usize, _29), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: usize,mut _2: bool,mut _3: usize,mut _4: char,mut _5: bool,mut _6: u64,mut _7: isize,mut _8: char) -> f32 {
mir! {
type RET = f32;
let _9: Adt53;
let _10: &'static Adt50;
let _11: &'static *mut &'static u64;
let _12: isize;
let _13: f64;
let _14: &'static Adt50;
let _15: char;
let _16: f32;
let _17: [i32; 3];
let _18: bool;
let _19: [isize; 2];
let _20: (u8, i64, u64);
let _21: &'static u64;
let _22: [bool; 6];
let _23: &'static Adt44;
let _24: &'static char;
let _25: i16;
let _26: f64;
let _27: f64;
let _28: bool;
let _29: f32;
let _30: (u128, [i64; 6]);
let _31: *const usize;
let _32: bool;
let _33: i64;
let _34: *const (u128, [i64; 6]);
let _35: (u128, [i64; 6]);
let _36: bool;
let _37: bool;
let _38: *const *const isize;
let _39: u16;
let _40: *const Adt53;
let _41: *const usize;
let _42: u32;
let _43: f32;
let _44: char;
let _45: isize;
let _46: ([i32; 7], &'static *mut &'static u64, [i16; 8]);
let _47: ();
let _48: ();
{
RET = _1 as f32;
_1 = _3 * _3;
_7 = RET as isize;
RET = (-1218_i16) as f32;
_7 = -(-127_isize);
_7 = !(-110_isize);
RET = (-72_i8) as f32;
_3 = _1 | _1;
RET = (-91_i8) as f32;
_7 = 9223372036854775807_isize;
RET = _1 as f32;
_6 = !4397112109015328226_u64;
_1 = _3 + _3;
RET = _6 as f32;
_5 = RET <= RET;
_3 = (-17815_i16) as usize;
_12 = 147223134200596428504873420154771514430_i128 as isize;
_4 = _8;
_5 = _2 | _2;
_8 = _4;
match _7 {
0 => bb1,
1 => bb2,
9223372036854775807 => bb4,
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
_7 = 75_i8 as isize;
Call(_3 = core::intrinsics::bswap(_1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET = 19_u8 as f32;
RET = 55586_u16 as f32;
_13 = _12 as f64;
_13 = 169059788789211579300178962820346936940_i128 as f64;
_16 = _6 as f32;
_15 = _8;
_12 = _7;
RET = -_16;
_5 = !_2;
_2 = _5 | _5;
_17 = [(-446444222_i32),1847648254_i32,892075670_i32];
_20.2 = _5 as u64;
_20.1 = 8339217297677265771_i64;
_21 = &_20.2;
_15 = _8;
_19 = [_12,_7];
Goto(bb6)
}
bb6 = {
_8 = _4;
_2 = _5;
_20 = (208_u8, (-8915898389989914026_i64), _6);
_15 = _4;
_21 = &_6;
_20 = (123_u8, 8730894840936661289_i64, (*_21));
_17 = [(-944645972_i32),222657148_i32,(-241703892_i32)];
_12 = -_7;
_18 = !_2;
_22 = [_18,_5,_5,_18,_2,_5];
_13 = _16 as f64;
_2 = (*_21) <= _6;
_13 = 1147068309_u32 as f64;
_1 = 40614351893551752364820498932474607287_u128 as usize;
_20.2 = _6;
_22 = [_2,_5,_2,_2,_5,_2];
_1 = !_3;
_1 = !_3;
_21 = &_6;
_21 = &(*_21);
Goto(bb7)
}
bb7 = {
_19 = [_12,_7];
RET = _16;
RET = _16;
_26 = _13;
_20.0 = !111_u8;
_6 = _12 as u64;
_15 = _4;
_5 = !_18;
_21 = &_20.2;
_19 = [_12,_7];
_20.2 = !_6;
_29 = RET;
_28 = _2 & _5;
_1 = 2709755081_u32 as usize;
_6 = _20.2;
_25 = _28 as i16;
_20 = (172_u8, 2639196014083837638_i64, _6);
_20.0 = 179_u8 + 185_u8;
_4 = _15;
_21 = &_6;
_28 = _2;
_15 = _8;
_18 = _20.0 >= _20.0;
_30.1 = [_20.1,_20.1,_20.1,_20.1,_20.1,_20.1];
match _20.1 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb4,
4 => bb5,
5 => bb8,
6 => bb9,
2639196014083837638 => bb11,
_ => bb10
}
}
bb8 = {
Return()
}
bb9 = {
RET = 19_u8 as f32;
RET = 55586_u16 as f32;
_13 = _12 as f64;
_13 = 169059788789211579300178962820346936940_i128 as f64;
_16 = _6 as f32;
_15 = _8;
_12 = _7;
RET = -_16;
_5 = !_2;
_2 = _5 | _5;
_17 = [(-446444222_i32),1847648254_i32,892075670_i32];
_20.2 = _5 as u64;
_20.1 = 8339217297677265771_i64;
_21 = &_20.2;
_15 = _8;
_19 = [_12,_7];
Goto(bb6)
}
bb10 = {
Return()
}
bb11 = {
_24 = &_4;
_28 = _25 >= _25;
_19 = [_12,_7];
_26 = -_13;
_17 = [(-669770420_i32),1782627721_i32,(-373558368_i32)];
Goto(bb12)
}
bb12 = {
_20.0 = (*_21) as u8;
_32 = !_2;
_15 = _8;
_30.0 = 62424234899149779053750824018552898611_u128 + 89202787237576667493306220138042396837_u128;
_25 = (*_21) as i16;
_35.0 = _30.0;
_21 = &(*_21);
_35 = (_30.0, _30.1);
_28 = !_32;
_36 = _18 > _32;
_35.1 = _30.1;
_1 = _3;
_19 = [_12,_12];
_35.1 = _30.1;
_25 = (-2947_i16) ^ (-12449_i16);
_7 = !_12;
_30.0 = _35.0 >> _3;
_17 = [(-801945746_i32),1386646746_i32,288931324_i32];
_31 = core::ptr::addr_of!(_1);
(*_31) = _3;
_20 = (94_u8, (-15068139245928351_i64), _6);
_35.0 = _20.2 as u128;
_8 = _4;
RET = _16;
_26 = _13;
_2 = !_18;
_34 = core::ptr::addr_of!(_35);
Call(_15 = fn3(Move(_24), _35.1, (*_34).0, (*_21), _20.0, (*_34), _30, (*_24), _4, Move(_34), _35), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_5 = _36;
(*_31) = _3 + _3;
_20.2 = (*_21);
_25 = (-17_i8) as i16;
_26 = _13 - _13;
_20.2 = (*_21) >> _25;
_19 = [_7,_7];
_26 = _13;
_33 = _20.1 - _20.1;
_34 = core::ptr::addr_of!(_30);
_27 = _26 + _26;
Call(_26 = core::intrinsics::transmute(_1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_35.1 = [_33,_33,_20.1,_33,_33,_33];
_39 = 7597_u16 << (*_21);
(*_34).1 = _35.1;
_20.1 = !_33;
_35.0 = _30.0;
_30 = _35;
_7 = _12 * _12;
_43 = _16;
_30 = _35;
_30 = _35;
_20.1 = -_33;
_6 = _20.2;
RET = _12 as f32;
_28 = _32 & _2;
_26 = _13;
Goto(bb15)
}
bb15 = {
Call(_47 = dump_var(2_usize, 25_usize, Move(_25), 20_usize, Move(_20), 22_usize, Move(_22), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_47 = dump_var(2_usize, 18_usize, Move(_18), 1_usize, Move(_1), 17_usize, Move(_17), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(2_usize, 35_usize, Move(_35), 8_usize, Move(_8), 28_usize, Move(_28), 48_usize, _48), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: &'static char,mut _2: [i64; 6],mut _3: u128,mut _4: u64,mut _5: u8,mut _6: (u128, [i64; 6]),mut _7: (u128, [i64; 6]),mut _8: char,mut _9: char,mut _10: *const (u128, [i64; 6]),mut _11: (u128, [i64; 6])) -> char {
mir! {
type RET = char;
let _12: bool;
let _13: i16;
let _14: isize;
let _15: (u8, u64, (u8, i64, u64));
let _16: &'static u64;
let _17: &'static isize;
let _18: (u8, i64, u64);
let _19: u128;
let _20: u16;
let _21: i8;
let _22: [bool; 5];
let _23: isize;
let _24: isize;
let _25: char;
let _26: usize;
let _27: ([i16; 8],);
let _28: [i32; 7];
let _29: char;
let _30: ([i64; 6], &'static f32, *const (u128, [i64; 6]));
let _31: f32;
let _32: *const (u8, i64, u64);
let _33: *const i128;
let _34: ();
let _35: ();
{
RET = _9;
_11.1 = [4350115124916222828_i64,(-45125454981399783_i64),(-8768645926661843091_i64),5861844627534586365_i64,(-5658227583071006299_i64),(-4071027861278732610_i64)];
_5 = !7_u8;
_1 = &_9;
_7.0 = !_11.0;
_6.0 = !_3;
_9 = RET;
_8 = RET;
_5 = 34_u8;
_1 = &RET;
_11.0 = 3300_i16 as u128;
_3 = _7.0 | _11.0;
_11.0 = !_6.0;
_8 = (*_1);
_12 = RET != (*_1);
_11.0 = 6871757273844896201_i64 as u128;
Goto(bb1)
}
bb1 = {
RET = _8;
_2 = _11.1;
_13 = !22902_i16;
RET = _9;
_11.1 = [3157805409897858561_i64,6284299806468714306_i64,(-7405494276966015191_i64),9164053906014820534_i64,5407632116220227673_i64,(-7336248276055671228_i64)];
_15.2.1 = (-614537396187306980_i64) << _4;
_15.0 = _5 << _3;
_7.0 = _11.0 & _11.0;
_10 = core::ptr::addr_of!(_6);
_14 = (-105_i8) as isize;
_6.1 = [_15.2.1,_15.2.1,_15.2.1,_15.2.1,_15.2.1,_15.2.1];
(*_10).0 = _7.0;
_7 = _11;
_15.2.1 = _14 as i64;
Goto(bb2)
}
bb2 = {
_1 = &_8;
_10 = core::ptr::addr_of!(_11);
_16 = &_4;
_15.2.0 = _5 * _15.0;
_11 = (_3, _7.1);
RET = (*_1);
_15.2.1 = 1078853507554948874_i64;
_8 = _9;
(*_10).0 = _6.0;
_17 = &_14;
_6.0 = _11.0;
_6.1 = [_15.2.1,_15.2.1,_15.2.1,_15.2.1,_15.2.1,_15.2.1];
_15.0 = 19723_u16 as u8;
(*_10).0 = _13 as u128;
(*_10).1 = [_15.2.1,_15.2.1,_15.2.1,_15.2.1,_15.2.1,_15.2.1];
_4 = 2395667981484474286_u64 & 17092480149889229596_u64;
_20 = !997_u16;
(*_10).0 = (-97_i8) as u128;
_9 = _8;
_6.1 = _2;
_1 = &RET;
_15.2.0 = !_15.0;
_17 = &_14;
Call(_2 = fn4(Move(_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6.0 = (*_10).0 & _11.0;
_6.1 = _7.1;
_1 = &_8;
_19 = _7.0;
(*_10).1 = _7.1;
_18.1 = (*_17) as i64;
(*_10).0 = !_7.0;
(*_10) = _6;
match _15.2.1 {
0 => bb4,
1 => bb5,
2 => bb6,
1078853507554948874 => bb8,
_ => bb7
}
}
bb4 = {
_1 = &_8;
_10 = core::ptr::addr_of!(_11);
_16 = &_4;
_15.2.0 = _5 * _15.0;
_11 = (_3, _7.1);
RET = (*_1);
_15.2.1 = 1078853507554948874_i64;
_8 = _9;
(*_10).0 = _6.0;
_17 = &_14;
_6.0 = _11.0;
_6.1 = [_15.2.1,_15.2.1,_15.2.1,_15.2.1,_15.2.1,_15.2.1];
_15.0 = 19723_u16 as u8;
(*_10).0 = _13 as u128;
(*_10).1 = [_15.2.1,_15.2.1,_15.2.1,_15.2.1,_15.2.1,_15.2.1];
_4 = 2395667981484474286_u64 & 17092480149889229596_u64;
_20 = !997_u16;
(*_10).0 = (-97_i8) as u128;
_9 = _8;
_6.1 = _2;
_1 = &RET;
_15.2.0 = !_15.0;
_17 = &_14;
Call(_2 = fn4(Move(_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
RET = _8;
_2 = _11.1;
_13 = !22902_i16;
RET = _9;
_11.1 = [3157805409897858561_i64,6284299806468714306_i64,(-7405494276966015191_i64),9164053906014820534_i64,5407632116220227673_i64,(-7336248276055671228_i64)];
_15.2.1 = (-614537396187306980_i64) << _4;
_15.0 = _5 << _3;
_7.0 = _11.0 & _11.0;
_10 = core::ptr::addr_of!(_6);
_14 = (-105_i8) as isize;
_6.1 = [_15.2.1,_15.2.1,_15.2.1,_15.2.1,_15.2.1,_15.2.1];
(*_10).0 = _7.0;
_7 = _11;
_15.2.1 = _14 as i64;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_7.1 = (*_10).1;
RET = _9;
(*_10).1 = [_18.1,_15.2.1,_15.2.1,_18.1,_15.2.1,_18.1];
(*_10).1 = [_18.1,_15.2.1,_15.2.1,_18.1,_18.1,_18.1];
RET = _8;
_18 = (_15.2.0, _15.2.1, _4);
_8 = RET;
_15.2.2 = _18.2;
_15.1 = _20 as u64;
(*_10).0 = _6.0 - _3;
(*_10).1 = [_18.1,_15.2.1,_18.1,_15.2.1,_18.1,_18.1];
_17 = &(*_17);
_2 = [_15.2.1,_15.2.1,_18.1,_15.2.1,_15.2.1,_18.1];
_17 = &(*_17);
_21 = -56_i8;
_16 = &_15.1;
_10 = core::ptr::addr_of!((*_10));
_6.0 = !_11.0;
_15.0 = !_5;
Call((*_10).0 = core::intrinsics::transmute(_6.0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_24 = _21 as isize;
_25 = RET;
_22 = [_12,_12,_12,_12,_12];
_3 = !(*_10).0;
_23 = (*_17) + _14;
RET = _8;
(*_10).1 = [_18.1,_18.1,_15.2.1,_18.1,_15.2.1,_15.2.1];
_7.0 = _19 | (*_10).0;
_1 = &_9;
_10 = core::ptr::addr_of!(_6);
RET = (*_1);
match _15.2.1 {
1078853507554948874 => bb11,
_ => bb10
}
}
bb10 = {
_6.0 = (*_10).0 & _11.0;
_6.1 = _7.1;
_1 = &_8;
_19 = _7.0;
(*_10).1 = _7.1;
_18.1 = (*_17) as i64;
(*_10).0 = !_7.0;
(*_10) = _6;
match _15.2.1 {
0 => bb4,
1 => bb5,
2 => bb6,
1078853507554948874 => bb8,
_ => bb7
}
}
bb11 = {
_20 = 7045368864704263731_usize as u16;
RET = (*_1);
_20 = 33795_u16 ^ 4585_u16;
(*_10).0 = _7.0 * _7.0;
Goto(bb12)
}
bb12 = {
_28 = [1817823208_i32,2096891192_i32,834253835_i32,1808913662_i32,1541866334_i32,(-770437810_i32),1652954064_i32];
_18.0 = !_15.2.0;
_18.2 = !_15.2.2;
_1 = &_8;
(*_10).1 = [_18.1,_18.1,_18.1,_18.1,_15.2.1,_15.2.1];
_9 = _25;
(*_10).1 = [_18.1,_15.2.1,_18.1,_18.1,_18.1,_18.1];
_9 = (*_1);
_18.0 = _15.2.0 * _5;
_18 = (_5, _15.2.1, _15.2.2);
_1 = &(*_1);
_7.1 = [_15.2.1,_18.1,_15.2.1,_15.2.1,_15.2.1,_18.1];
(*_10) = _7;
_6.1 = [_15.2.1,_18.1,_15.2.1,_15.2.1,_18.1,_18.1];
RET = _25;
_13 = _20 as i16;
_20 = 34732_u16 * 11873_u16;
_20 = 158_u16 * 33334_u16;
_1 = &_8;
_18.1 = -_15.2.1;
_11.1 = [_15.2.1,_18.1,_15.2.1,_15.2.1,_18.1,_15.2.1];
_10 = core::ptr::addr_of!((*_10));
_12 = !true;
_24 = (*_17);
_18.1 = 3976528458_u32 as i64;
_2 = [_15.2.1,_18.1,_15.2.1,_15.2.1,_18.1,_18.1];
_6.0 = _15.2.1 as u128;
_17 = &(*_17);
_6.1 = [_18.1,_18.1,_15.2.1,_15.2.1,_15.2.1,_18.1];
_20 = 3133585314_u32 as u16;
match _15.2.1 {
0 => bb1,
1078853507554948874 => bb14,
_ => bb13
}
}
bb13 = {
RET = _8;
_2 = _11.1;
_13 = !22902_i16;
RET = _9;
_11.1 = [3157805409897858561_i64,6284299806468714306_i64,(-7405494276966015191_i64),9164053906014820534_i64,5407632116220227673_i64,(-7336248276055671228_i64)];
_15.2.1 = (-614537396187306980_i64) << _4;
_15.0 = _5 << _3;
_7.0 = _11.0 & _11.0;
_10 = core::ptr::addr_of!(_6);
_14 = (-105_i8) as isize;
_6.1 = [_15.2.1,_15.2.1,_15.2.1,_15.2.1,_15.2.1,_15.2.1];
(*_10).0 = _7.0;
_7 = _11;
_15.2.1 = _14 as i64;
Goto(bb2)
}
bb14 = {
_15.1 = _18.2;
_22 = [_12,_12,_12,_12,_12];
_7.0 = _11.0 >> _19;
_21 = (-109_i8);
(*_10).1 = [_18.1,_18.1,_15.2.1,_15.2.1,_18.1,_15.2.1];
_24 = _23;
_14 = _24;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(3_usize, 22_usize, Move(_22), 20_usize, Move(_20), 3_usize, Move(_3), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(3_usize, 13_usize, Move(_13), 6_usize, Move(_6), 25_usize, Move(_25), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(3_usize, 24_usize, Move(_24), 7_usize, Move(_7), 11_usize, Move(_11), 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: &'static char) -> [i64; 6] {
mir! {
type RET = [i64; 6];
let _2: ((i128, *const isize), *mut u128, Adt44);
let _3: f32;
let _4: *const i128;
let _5: usize;
let _6: *const isize;
let _7: [i32; 3];
let _8: f64;
let _9: *mut Adt53;
let _10: u32;
let _11: isize;
let _12: i32;
let _13: [usize; 2];
let _14: bool;
let _15: [i64; 6];
let _16: f32;
let _17: i8;
let _18: char;
let _19: i64;
let _20: ();
let _21: ();
{
RET = [8985541107802438758_i64,(-5628133604473729063_i64),2714646084808562918_i64,6131579461673013423_i64,(-1591598380101495249_i64),2612289536679553571_i64];
_2.0.0 = 39362547556521991633625260107598307681_i128;
RET = [872200239851090686_i64,(-5700164335202262123_i64),(-8368544378143049703_i64),7130936007549198551_i64,(-6725119705020319450_i64),(-3800852796747142083_i64)];
_2.0.0 = 89409435423199276328018690107854585620_i128;
_2.2 = Adt44::Variant1 { fld0: RET,fld1: '\u{12372}' };
_1 = &place!(Field::<char>(Variant(_2.2, 1), 1));
place!(Field::<char>(Variant(_2.2, 1), 1)) = '\u{194be}';
RET = [(-5875589781699315891_i64),4693579271430606930_i64,(-2494459286373926195_i64),5193161191687514730_i64,4889792910095388338_i64,3707362908221624614_i64];
_1 = &place!(Field::<char>(Variant(_2.2, 1), 1));
_3 = 13964276469812614294_usize as f32;
_5 = 1_usize & 2827496178954699815_usize;
_5 = 5882166964729783283_usize;
RET = [3324346205168405520_i64,(-8091502669621948832_i64),(-2040487062857663775_i64),(-2847249075045934965_i64),(-6914212425212496759_i64),(-6293698779720973380_i64)];
_4 = core::ptr::addr_of!(_2.0.0);
_2.0.0 = 143160889278409281152609722688570809715_i128 | (-18118486366013338330755020672114995895_i128);
_3 = (-9223372036854775808_isize) as f32;
_8 = 9084238330169580967_u64 as f64;
RET = [(-7409808885524429968_i64),(-6276245360730727802_i64),(-8096616930015555249_i64),6104534110440762867_i64,8544876094834722235_i64,6709134031991707777_i64];
RET = [6618067377657254674_i64,(-1211939187759279786_i64),(-55608644342428403_i64),4857708683891435093_i64,4643534545119451568_i64,(-3442332107465907496_i64)];
place!(Field::<char>(Variant(_2.2, 1), 1)) = '\u{e6c80}';
Call(RET = fn5(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = core::ptr::addr_of!((*_4));
_2.0.0 = 146338932930020642263234725127650178611_i128 + 160640740217305763271927806416125937821_i128;
_8 = (-82_isize) as f64;
_1 = &place!(Field::<char>(Variant(_2.2, 1), 1));
place!(Field::<char>(Variant(_2.2, 1), 1)) = '\u{9c9d6}';
_7 = [422257915_i32,1616203944_i32,1263070027_i32];
_6 = core::ptr::addr_of!(_11);
_6 = core::ptr::addr_of!((*_6));
_1 = &place!(Field::<char>(Variant(_2.2, 1), 1));
(*_6) = (-9223372036854775808_isize) | 9223372036854775807_isize;
place!(Field::<char>(Variant(_2.2, 1), 1)) = '\u{17a3e}';
SetDiscriminant(_2.2, 2);
(*_6) = _8 as isize;
RET = [(-1738799609314839822_i64),(-6120596489443616218_i64),4622552805963240011_i64,1472518040572247550_i64,8466541591762150054_i64,(-690236669330564068_i64)];
_2.2 = Adt44::Variant1 { fld0: RET,fld1: '\u{76b4d}' };
Goto(bb2)
}
bb2 = {
_2.0.1 = core::ptr::addr_of!((*_6));
_2.0.0 = 108405568731308526897151507886960035937_i128 ^ 23618412121609987957105023102110912942_i128;
RET = [(-2203887816724570691_i64),2479591886025799349_i64,5995506008598897898_i64,(-6651210382556709619_i64),7699744383706052745_i64,5103736955717609871_i64];
(*_4) = (-39368174442412810063518324475518812294_i128);
(*_6) = -(-105_isize);
_8 = (*_6) as f64;
(*_6) = (-6489855596940322081_i64) as isize;
Goto(bb3)
}
bb3 = {
_2.0 = ((-756280842126566244575487616822753843_i128), Move(_6));
(*_4) = -(-41312151552255028181587096923820400527_i128);
_3 = 105382531354066693970060669885396854652_u128 as f32;
_12 = (-1269775555_i32) + (-2121676303_i32);
_10 = !3126994346_u32;
_2.0.0 = 44385032451539260924006205317001297199_i128;
_4 = core::ptr::addr_of!((*_4));
_13 = [_5,_5];
_2.0.0 = (-5720_i16) as i128;
_13 = [_5,_5];
_6 = core::ptr::addr_of!(_11);
_2.0 = ((-150736898321111260394414496786083470769_i128), Move(_6));
place!(Field::<char>(Variant(_2.2, 1), 1)) = '\u{715d1}';
RET = [(-8721740126905902195_i64),(-1412910455961326133_i64),748507666942936438_i64,8389684901992089888_i64,(-5923050609034096114_i64),8119185786026060140_i64];
_13 = [_5,_5];
_2.0.0 = -(-4109165152051492707625850590562112488_i128);
SetDiscriminant(_2.2, 0);
match _5 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
5882166964729783283 => bb9,
_ => bb8
}
}
bb4 = {
_2.0.1 = core::ptr::addr_of!((*_6));
_2.0.0 = 108405568731308526897151507886960035937_i128 ^ 23618412121609987957105023102110912942_i128;
RET = [(-2203887816724570691_i64),2479591886025799349_i64,5995506008598897898_i64,(-6651210382556709619_i64),7699744383706052745_i64,5103736955717609871_i64];
(*_4) = (-39368174442412810063518324475518812294_i128);
(*_6) = -(-105_isize);
_8 = (*_6) as f64;
(*_6) = (-6489855596940322081_i64) as isize;
Goto(bb3)
}
bb5 = {
_4 = core::ptr::addr_of!((*_4));
_2.0.0 = 146338932930020642263234725127650178611_i128 + 160640740217305763271927806416125937821_i128;
_8 = (-82_isize) as f64;
_1 = &place!(Field::<char>(Variant(_2.2, 1), 1));
place!(Field::<char>(Variant(_2.2, 1), 1)) = '\u{9c9d6}';
_7 = [422257915_i32,1616203944_i32,1263070027_i32];
_6 = core::ptr::addr_of!(_11);
_6 = core::ptr::addr_of!((*_6));
_1 = &place!(Field::<char>(Variant(_2.2, 1), 1));
(*_6) = (-9223372036854775808_isize) | 9223372036854775807_isize;
place!(Field::<char>(Variant(_2.2, 1), 1)) = '\u{17a3e}';
SetDiscriminant(_2.2, 2);
(*_6) = _8 as isize;
RET = [(-1738799609314839822_i64),(-6120596489443616218_i64),4622552805963240011_i64,1472518040572247550_i64,8466541591762150054_i64,(-690236669330564068_i64)];
_2.2 = Adt44::Variant1 { fld0: RET,fld1: '\u{76b4d}' };
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
_3 = 17213167473852625374_u64 as f32;
_14 = false;
place!(Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3)).2 = (52_u8, (-1846562928977442643_i64), 17249119038144487151_u64);
_5 = 51820_u16 as usize;
_13 = [_5,_5];
place!(Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3)).2.0 = 233_u8;
_13 = [_5,_5];
match Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3).2.2 {
17249119038144487151 => bb10,
_ => bb4
}
}
bb10 = {
place!(Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3)).1 = 102223845560060571828793110089658875998_u128 as u64;
place!(Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3)).2.1 = 7271531917989862756_i64;
RET = [Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3).2.1,Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3).2.1,Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3).2.1,Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3).2.1,Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3).2.1,Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3).2.1];
_8 = _12 as f64;
place!(Field::<i128>(Variant(_2.2, 0), 2)) = _2.0.0;
place!(Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3)).2.0 = 101_u8;
_8 = 109_i8 as f64;
_13 = [_5,_5];
place!(Field::<u8>(Variant(_2.2, 0), 5)) = Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3).2.0;
place!(Field::<u8>(Variant(_2.2, 0), 5)) = Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3).2.2 as u8;
match Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3).2.2 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb11,
5 => bb12,
17249119038144487151 => bb14,
_ => bb13
}
}
bb11 = {
_3 = 17213167473852625374_u64 as f32;
_14 = false;
place!(Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3)).2 = (52_u8, (-1846562928977442643_i64), 17249119038144487151_u64);
_5 = 51820_u16 as usize;
_13 = [_5,_5];
place!(Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3)).2.0 = 233_u8;
_13 = [_5,_5];
match Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3).2.2 {
17249119038144487151 => bb10,
_ => bb4
}
}
bb12 = {
Return()
}
bb13 = {
_2.0 = ((-756280842126566244575487616822753843_i128), Move(_6));
(*_4) = -(-41312151552255028181587096923820400527_i128);
_3 = 105382531354066693970060669885396854652_u128 as f32;
_12 = (-1269775555_i32) + (-2121676303_i32);
_10 = !3126994346_u32;
_2.0.0 = 44385032451539260924006205317001297199_i128;
_4 = core::ptr::addr_of!((*_4));
_13 = [_5,_5];
_2.0.0 = (-5720_i16) as i128;
_13 = [_5,_5];
_6 = core::ptr::addr_of!(_11);
_2.0 = ((-150736898321111260394414496786083470769_i128), Move(_6));
place!(Field::<char>(Variant(_2.2, 1), 1)) = '\u{715d1}';
RET = [(-8721740126905902195_i64),(-1412910455961326133_i64),748507666942936438_i64,8389684901992089888_i64,(-5923050609034096114_i64),8119185786026060140_i64];
_13 = [_5,_5];
_2.0.0 = -(-4109165152051492707625850590562112488_i128);
SetDiscriminant(_2.2, 0);
match _5 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
5882166964729783283 => bb9,
_ => bb8
}
}
bb14 = {
RET = [Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3).2.1,Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3).2.1,Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3).2.1,Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3).2.1,Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3).2.1,Field::<(u8, u64, (u8, i64, u64))>(Variant(_2.2, 0), 3).2.1];
place!(Field::<usize>(Variant(_2.2, 0), 4)) = !_5;
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(4_usize, 10_usize, Move(_10), 7_usize, Move(_7), 12_usize, Move(_12), 21_usize, _21), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5() -> [i64; 6] {
mir! {
type RET = [i64; 6];
let _1: *mut u128;
let _2: u64;
let _3: [i8; 7];
let _4: isize;
let _5: [bool; 5];
let _6: u64;
let _7: char;
let _8: f64;
let _9: ([i16; 8],);
let _10: usize;
let _11: &'static isize;
let _12: f64;
let _13: [usize; 4];
let _14: &'static &'static i128;
let _15: [i8; 7];
let _16: usize;
let _17: [i8; 7];
let _18: &'static ([i64; 6], &'static f32, *const (u128, [i64; 6]));
let _19: [u64; 6];
let _20: isize;
let _21: *const i128;
let _22: i16;
let _23: *const &'static u64;
let _24: f32;
let _25: i64;
let _26: [usize; 4];
let _27: &'static Adt34;
let _28: i64;
let _29: (i32, i128, i64, bool);
let _30: *mut [i64; 6];
let _31: (u128, [i64; 6]);
let _32: i64;
let _33: f64;
let _34: isize;
let _35: f64;
let _36: [i32; 7];
let _37: f32;
let _38: [usize; 2];
let _39: (i128, char, i32, u32);
let _40: &'static Adt50;
let _41: u32;
let _42: &'static char;
let _43: *mut &'static u64;
let _44: ([i16; 8],);
let _45: ();
let _46: ();
{
RET = [4528750294402242569_i64,(-767918897742898130_i64),8894761212758350392_i64,6134198952221038577_i64,(-6422351157304291202_i64),6848967843419877813_i64];
RET = [4779922012164279739_i64,(-6073379398827533147_i64),(-8873638566000177761_i64),3450899488392589831_i64,(-4095054088661251912_i64),(-1947308385481851006_i64)];
RET = [6263810297085494408_i64,(-8763650899569929490_i64),(-7529124814909770630_i64),(-7737742691643905254_i64),(-1074769923443136123_i64),(-773259504229838602_i64)];
RET = [(-1233663849822413858_i64),(-4070654267276718882_i64),1314646260094526164_i64,(-3375975975935171493_i64),3943142960106150274_i64,(-6137534782669718524_i64)];
RET = [(-4580460356099516463_i64),7863966312721862314_i64,(-2412749671798863539_i64),(-1062613156271484138_i64),282567794756790533_i64,(-7268766515466767370_i64)];
RET = [(-3543184683339576051_i64),(-6838518930419618442_i64),1337386635152274212_i64,4814430175151563135_i64,(-4088339486611007937_i64),(-2249869610603283386_i64)];
RET = [5122351989442556657_i64,(-5997733846054432579_i64),5952713323671171194_i64,3677551688229828022_i64,1701334127819881405_i64,(-7427068216142714961_i64)];
RET = [(-2465880983534015152_i64),4845489343975335965_i64,(-252321738872798405_i64),(-1477146462068995158_i64),(-9174655809641071421_i64),2142144442084231269_i64];
RET = [(-3683988672084241604_i64),(-7558599150748699211_i64),9222531573348113642_i64,(-4333727641340287886_i64),(-5365509175707497487_i64),(-1237478206007165955_i64)];
RET = [(-7547892725963773090_i64),(-8466204393953929583_i64),(-7409279214494379259_i64),7118593257347653402_i64,(-8699232792730286897_i64),(-8614709011896507661_i64)];
RET = [7784462599046150807_i64,(-4225495147416729742_i64),8922814350068286922_i64,2211065187184294827_i64,5235295034497432289_i64,5387416621520729937_i64];
_2 = 15849711019644391932_u64 - 13006261014064335544_u64;
_2 = 13282869332974518577_u64 << (-159528583679897286843564352906430594993_i128);
_2 = 9348692565679211452_u64;
_2 = 226318583524458745158974890548696242285_u128 as u64;
_2 = 2212816478292500650_u64 - 597392804978130387_u64;
_3 = [(-75_i8),(-9_i8),55_i8,97_i8,(-99_i8),(-57_i8),33_i8];
RET = [1918331284475611557_i64,(-949254229115407708_i64),(-904048853070350252_i64),3986722515091694043_i64,(-4456880331560120444_i64),(-4338649958191403301_i64)];
Goto(bb1)
}
bb1 = {
_5 = [true,false,true,true,false];
RET = [8008464791351833406_i64,(-757700757532895002_i64),(-9127252372117946880_i64),(-2718366950899798885_i64),(-4952975099946083070_i64),(-1213514705647425794_i64)];
RET = [1557944523584242199_i64,135151306936205777_i64,6111479071736902027_i64,(-3344167129612115012_i64),(-3357795941165650856_i64),768606536468006764_i64];
_2 = 11942516118588748467_u64;
_4 = -(-9223372036854775808_isize);
RET = [4807864878218438211_i64,(-9100174534494097723_i64),(-2754417879141907222_i64),6303306630389355741_i64,8610965806197167035_i64,1730598520371843474_i64];
Goto(bb2)
}
bb2 = {
RET = [(-5815948543369505313_i64),8749456105750120439_i64,(-6998350592292986307_i64),6035740136833865761_i64,134435081366006188_i64,(-8528109983354873876_i64)];
_5 = [true,true,true,true,true];
RET = [(-4130788157334486335_i64),(-7963151864951201883_i64),(-351495438489390360_i64),(-7137076616759128521_i64),1905951213671054664_i64,(-2198464458107682507_i64)];
_4 = (-122_isize);
_5 = [true,true,false,false,false];
RET = [(-5853508973265478223_i64),556753906515501172_i64,3754746550689346777_i64,(-7025960085252873218_i64),4960843402961078091_i64,(-8597266527899892575_i64)];
_5 = [false,true,false,true,true];
_3 = [(-127_i8),(-93_i8),47_i8,(-114_i8),(-29_i8),(-55_i8),(-21_i8)];
_3 = [(-27_i8),(-20_i8),33_i8,(-21_i8),(-115_i8),78_i8,(-79_i8)];
_2 = 10011257499539708510_u64 >> _4;
_3 = [(-128_i8),114_i8,68_i8,114_i8,(-117_i8),(-76_i8),85_i8];
_4 = (-16085_i16) as isize;
_7 = '\u{95882}';
_2 = _7 as u64;
_5 = [false,false,false,true,false];
_3 = [(-118_i8),25_i8,(-98_i8),(-64_i8),(-56_i8),(-4_i8),(-31_i8)];
RET = [4695654451590668566_i64,(-5926683522766174089_i64),(-8555239068194711460_i64),(-5469195222236423614_i64),4036516929904753457_i64,(-4480933670935579654_i64)];
Call(_6 = core::intrinsics::bswap(_2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = [(-8847576549974073920_i64),(-519614642004672003_i64),781317372860828892_i64,(-2957573181619797811_i64),2648102256220898589_i64,4754323022205939912_i64];
_6 = _2;
RET = [3981413373960986508_i64,(-2232339123728652600_i64),(-117121089492215260_i64),(-1521662691541941241_i64),4538319285967711129_i64,(-3941875791016971336_i64)];
RET = [5653176340877008578_i64,(-2394254138982725118_i64),(-5242732651633527701_i64),(-8028230895331193823_i64),9223177643336684528_i64,8793151056312274177_i64];
RET = [(-2753819898160290642_i64),(-4097410954895548701_i64),8412703812267813909_i64,1194410062715110873_i64,(-1744598182988284689_i64),(-5757999085800974140_i64)];
_3 = [61_i8,(-47_i8),43_i8,(-62_i8),(-112_i8),(-49_i8),49_i8];
_2 = !_6;
_6 = 1096513697_u32 as u64;
_9.0 = [(-25465_i16),26112_i16,(-21754_i16),8845_i16,3275_i16,(-4077_i16),6962_i16,10232_i16];
RET = [7827877512088779285_i64,(-6391586359507896330_i64),8982078800550730906_i64,(-4458932130003322508_i64),7225954633633968561_i64,80873788322963234_i64];
_2 = _6;
_2 = _6;
_2 = !_6;
_4 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
_8 = 23_u8 as f64;
_8 = 141304028449136105766561093656287549327_u128 as f64;
_8 = 2206638874_u32 as f64;
_5 = [true,false,false,false,false];
_4 = (-56559681865052170173700171359183716881_i128) as isize;
_2 = (-54686021738630855642324816868735688836_i128) as u64;
_2 = _6 >> _4;
_5 = [true,false,false,false,false];
RET = [3170274068209666376_i64,(-7781472961243462370_i64),4683852778357862908_i64,(-8655023486217513517_i64),5876293383019483993_i64,4975195708610376922_i64];
_3 = [108_i8,89_i8,(-63_i8),121_i8,(-117_i8),68_i8,44_i8];
_5 = [false,false,false,false,false];
Call(_4 = fn6(_5, _3, _9.0, _9.0, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_10 = 302852875756080606510636234402883309233_u128 as usize;
_4 = 85_isize;
_7 = '\u{c71e3}';
_4 = (-44_isize);
RET = [6750370871254856024_i64,512526450105900421_i64,2051910632963217964_i64,1965953781730099280_i64,8689441599144489580_i64,(-2778282626885016187_i64)];
RET = [5650246368664079929_i64,3856368170652561401_i64,2357225122325376865_i64,(-6034048862950496379_i64),8655963361143657521_i64,747869256872743308_i64];
_9.0 = [28317_i16,(-10317_i16),622_i16,25171_i16,28882_i16,(-14705_i16),(-8621_i16),15145_i16];
_7 = '\u{deae0}';
_12 = -_8;
_7 = '\u{825b5}';
_10 = 773568793_i32 as usize;
_9.0 = [(-32103_i16),(-30469_i16),(-22979_i16),(-14476_i16),30620_i16,13399_i16,(-32301_i16),23543_i16];
_15 = _3;
_5 = [true,false,false,false,false];
_11 = &_4;
Goto(bb5)
}
bb5 = {
RET = [(-2607077093964284836_i64),(-3251459713483483193_i64),937905371188760210_i64,8922134876838275710_i64,2223308782733677569_i64,(-5127065505036102054_i64)];
_3 = _15;
_7 = '\u{d28ea}';
_5 = [false,true,true,true,false];
RET = [(-5357704949228238211_i64),(-466928364075805523_i64),(-4891442021614442815_i64),7840556717761877015_i64,(-8533847579608500208_i64),4080804003773540221_i64];
_15 = [93_i8,(-54_i8),108_i8,(-39_i8),(-7_i8),104_i8,(-61_i8)];
_9.0 = [55_i16,(-31792_i16),(-1755_i16),(-1295_i16),28444_i16,20696_i16,15350_i16,22536_i16];
_10 = (-27_i8) as usize;
_12 = _8;
_17 = [13_i8,(-120_i8),7_i8,(-100_i8),4_i8,(-102_i8),(-38_i8)];
_7 = '\u{2c2e7}';
_11 = &_4;
_4 = 9223372036854775807_isize;
Goto(bb6)
}
bb6 = {
_4 = false as isize;
RET = [5432805339083079704_i64,(-2410563585476700735_i64),5318746778184130366_i64,3154673522970797427_i64,(-2386399772594967838_i64),7767931533915746633_i64];
RET = [(-2302084466465977958_i64),(-8690320053943158104_i64),(-3603185171211200880_i64),(-6975891388657283805_i64),7359800599826448423_i64,(-8579086782733599534_i64)];
_17 = [(-72_i8),24_i8,80_i8,(-89_i8),45_i8,24_i8,41_i8];
_19 = [_6,_2,_2,_6,_2,_2];
_9.0 = [29906_i16,(-7433_i16),6132_i16,(-3733_i16),3768_i16,16588_i16,12779_i16,2198_i16];
_4 = 9223372036854775807_isize + (-9223372036854775808_isize);
_11 = &_4;
_7 = '\u{209d9}';
_12 = -_8;
_8 = _12 * _12;
_19 = [_6,_2,_2,_2,_2,_2];
_5 = [false,true,false,false,false];
_8 = 95_i8 as f64;
RET = [(-3469220196709321773_i64),6420224507935415216_i64,(-3226008564033222329_i64),5293401641664638839_i64,(-1420971408716875592_i64),6613943530761195317_i64];
_16 = !_10;
_20 = -_4;
_15 = [104_i8,9_i8,(-112_i8),55_i8,(-54_i8),(-90_i8),85_i8];
_8 = -_12;
_24 = (-52672901351731945843126121614009006775_i128) as f32;
_8 = _12 - _12;
_19 = [_2,_2,_6,_2,_2,_2];
_12 = 150550662563585564698886972459726426527_u128 as f64;
_26 = [_16,_10,_10,_16];
_24 = _8 as f32;
Goto(bb7)
}
bb7 = {
_6 = 278753856613881059211073317555454785540_u128 as u64;
Goto(bb8)
}
bb8 = {
_4 = 139159304951154919706351987342610743528_u128 as isize;
RET = [6348820277962813275_i64,8003002175602885507_i64,1049660887243668960_i64,(-5257521736117079871_i64),(-5857602217850708268_i64),(-4107790858669022123_i64)];
_8 = _12;
RET = [2678493739758298494_i64,4561783142498119747_i64,8419725112510284137_i64,(-2136914565126630356_i64),(-874870471397070409_i64),(-538455142732617212_i64)];
_22 = (-19042_i16);
_19 = [_2,_2,_2,_6,_6,_2];
_17 = [15_i8,125_i8,57_i8,(-9_i8),33_i8,(-29_i8),(-69_i8)];
_28 = -(-7673027846955352603_i64);
RET = [_28,_28,_28,_28,_28,_28];
_8 = -_12;
_26 = [_10,_16,_10,_16];
RET = [_28,_28,_28,_28,_28,_28];
_4 = _20;
_26 = [_10,_10,_16,_16];
_12 = _8 + _8;
match _22 {
0 => bb6,
1 => bb4,
340282366920938463463374607431768192414 => bb10,
_ => bb9
}
}
bb9 = {
RET = [(-2607077093964284836_i64),(-3251459713483483193_i64),937905371188760210_i64,8922134876838275710_i64,2223308782733677569_i64,(-5127065505036102054_i64)];
_3 = _15;
_7 = '\u{d28ea}';
_5 = [false,true,true,true,false];
RET = [(-5357704949228238211_i64),(-466928364075805523_i64),(-4891442021614442815_i64),7840556717761877015_i64,(-8533847579608500208_i64),4080804003773540221_i64];
_15 = [93_i8,(-54_i8),108_i8,(-39_i8),(-7_i8),104_i8,(-61_i8)];
_9.0 = [55_i16,(-31792_i16),(-1755_i16),(-1295_i16),28444_i16,20696_i16,15350_i16,22536_i16];
_10 = (-27_i8) as usize;
_12 = _8;
_17 = [13_i8,(-120_i8),7_i8,(-100_i8),4_i8,(-102_i8),(-38_i8)];
_7 = '\u{2c2e7}';
_11 = &_4;
_4 = 9223372036854775807_isize;
Goto(bb6)
}
bb10 = {
_21 = core::ptr::addr_of!(_29.1);
_12 = (-500713798_i32) as f64;
_19 = [_2,_2,_6,_6,_2,_2];
_29.2 = _28 ^ _28;
_7 = '\u{ab7ee}';
_29.1 = (-169814195065468300988483911718598954901_i128) + (-132366529679377179532345111391835123772_i128);
_29.2 = _20 as i64;
_25 = _29.2;
_6 = !_2;
RET = [_28,_25,_28,_25,_25,_25];
RET = [_25,_25,_25,_28,_25,_28];
_29.3 = false;
_3 = [56_i8,43_i8,55_i8,40_i8,26_i8,(-71_i8),11_i8];
match _22 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb9,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607431768192414 => bb12,
_ => bb11
}
}
bb11 = {
_5 = [true,false,true,true,false];
RET = [8008464791351833406_i64,(-757700757532895002_i64),(-9127252372117946880_i64),(-2718366950899798885_i64),(-4952975099946083070_i64),(-1213514705647425794_i64)];
RET = [1557944523584242199_i64,135151306936205777_i64,6111479071736902027_i64,(-3344167129612115012_i64),(-3357795941165650856_i64),768606536468006764_i64];
_2 = 11942516118588748467_u64;
_4 = -(-9223372036854775808_isize);
RET = [4807864878218438211_i64,(-9100174534494097723_i64),(-2754417879141907222_i64),6303306630389355741_i64,8610965806197167035_i64,1730598520371843474_i64];
Goto(bb2)
}
bb12 = {
_21 = core::ptr::addr_of!((*_21));
_22 = (-21061_i16);
_29.0 = _10 as i32;
_19 = [_2,_2,_2,_6,_6,_2];
_29 = ((-183720833_i32), (-3848026906287052394009144590156579994_i128), _25, false);
_4 = 51_u8 as isize;
_1 = core::ptr::addr_of_mut!(_31.0);
_12 = _8;
(*_1) = 125253812752608820100437183317015428638_u128;
_34 = _20;
_33 = _12;
_11 = &_34;
_29.2 = !_28;
_10 = _16;
_29.2 = 45676_u16 as i64;
_5 = [_29.3,_29.3,_29.3,_29.3,_29.3];
RET = [_25,_29.2,_25,_25,_25,_28];
_19 = [_6,_2,_2,_2,_6,_2];
(*_1) = !303401236955917784873429428275701977427_u128;
_31.1 = [_25,_25,_29.2,_25,_25,_29.2];
_29.1 = 167234084846077955295147026403628259975_i128 * (-36109936917285381898161291461344515220_i128);
_3 = [89_i8,21_i8,37_i8,(-37_i8),(-69_i8),(-107_i8),(-112_i8)];
_19 = [_6,_2,_2,_6,_2,_6];
Call(RET = core::intrinsics::transmute(_31.1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_29.2 = 62180_u16 as i64;
_19 = [_6,_6,_6,_2,_2,_2];
_35 = _8 * _12;
(*_1) = 54058334737921157116010043604855926002_u128;
_30 = core::ptr::addr_of_mut!(_31.1);
_24 = (*_21) as f32;
_12 = -_33;
_6 = 11_u8 as u64;
_33 = _8 + _8;
match _29.0 {
0 => bb1,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
340282366920938463463374607431584490623 => bb19,
_ => bb18
}
}
bb14 = {
_21 = core::ptr::addr_of!((*_21));
_22 = (-21061_i16);
_29.0 = _10 as i32;
_19 = [_2,_2,_2,_6,_6,_2];
_29 = ((-183720833_i32), (-3848026906287052394009144590156579994_i128), _25, false);
_4 = 51_u8 as isize;
_1 = core::ptr::addr_of_mut!(_31.0);
_12 = _8;
(*_1) = 125253812752608820100437183317015428638_u128;
_34 = _20;
_33 = _12;
_11 = &_34;
_29.2 = !_28;
_10 = _16;
_29.2 = 45676_u16 as i64;
_5 = [_29.3,_29.3,_29.3,_29.3,_29.3];
RET = [_25,_29.2,_25,_25,_25,_28];
_19 = [_6,_2,_2,_2,_6,_2];
(*_1) = !303401236955917784873429428275701977427_u128;
_31.1 = [_25,_25,_29.2,_25,_25,_29.2];
_29.1 = 167234084846077955295147026403628259975_i128 * (-36109936917285381898161291461344515220_i128);
_3 = [89_i8,21_i8,37_i8,(-37_i8),(-69_i8),(-107_i8),(-112_i8)];
_19 = [_6,_2,_2,_6,_2,_6];
Call(RET = core::intrinsics::transmute(_31.1), ReturnTo(bb13), UnwindUnreachable())
}
bb15 = {
RET = [(-5815948543369505313_i64),8749456105750120439_i64,(-6998350592292986307_i64),6035740136833865761_i64,134435081366006188_i64,(-8528109983354873876_i64)];
_5 = [true,true,true,true,true];
RET = [(-4130788157334486335_i64),(-7963151864951201883_i64),(-351495438489390360_i64),(-7137076616759128521_i64),1905951213671054664_i64,(-2198464458107682507_i64)];
_4 = (-122_isize);
_5 = [true,true,false,false,false];
RET = [(-5853508973265478223_i64),556753906515501172_i64,3754746550689346777_i64,(-7025960085252873218_i64),4960843402961078091_i64,(-8597266527899892575_i64)];
_5 = [false,true,false,true,true];
_3 = [(-127_i8),(-93_i8),47_i8,(-114_i8),(-29_i8),(-55_i8),(-21_i8)];
_3 = [(-27_i8),(-20_i8),33_i8,(-21_i8),(-115_i8),78_i8,(-79_i8)];
_2 = 10011257499539708510_u64 >> _4;
_3 = [(-128_i8),114_i8,68_i8,114_i8,(-117_i8),(-76_i8),85_i8];
_4 = (-16085_i16) as isize;
_7 = '\u{95882}';
_2 = _7 as u64;
_5 = [false,false,false,true,false];
_3 = [(-118_i8),25_i8,(-98_i8),(-64_i8),(-56_i8),(-4_i8),(-31_i8)];
RET = [4695654451590668566_i64,(-5926683522766174089_i64),(-8555239068194711460_i64),(-5469195222236423614_i64),4036516929904753457_i64,(-4480933670935579654_i64)];
Call(_6 = core::intrinsics::bswap(_2), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
_4 = false as isize;
RET = [5432805339083079704_i64,(-2410563585476700735_i64),5318746778184130366_i64,3154673522970797427_i64,(-2386399772594967838_i64),7767931533915746633_i64];
RET = [(-2302084466465977958_i64),(-8690320053943158104_i64),(-3603185171211200880_i64),(-6975891388657283805_i64),7359800599826448423_i64,(-8579086782733599534_i64)];
_17 = [(-72_i8),24_i8,80_i8,(-89_i8),45_i8,24_i8,41_i8];
_19 = [_6,_2,_2,_6,_2,_2];
_9.0 = [29906_i16,(-7433_i16),6132_i16,(-3733_i16),3768_i16,16588_i16,12779_i16,2198_i16];
_4 = 9223372036854775807_isize + (-9223372036854775808_isize);
_11 = &_4;
_7 = '\u{209d9}';
_12 = -_8;
_8 = _12 * _12;
_19 = [_6,_2,_2,_2,_2,_2];
_5 = [false,true,false,false,false];
_8 = 95_i8 as f64;
RET = [(-3469220196709321773_i64),6420224507935415216_i64,(-3226008564033222329_i64),5293401641664638839_i64,(-1420971408716875592_i64),6613943530761195317_i64];
_16 = !_10;
_20 = -_4;
_15 = [104_i8,9_i8,(-112_i8),55_i8,(-54_i8),(-90_i8),85_i8];
_8 = -_12;
_24 = (-52672901351731945843126121614009006775_i128) as f32;
_8 = _12 - _12;
_19 = [_2,_2,_6,_2,_2,_2];
_12 = 150550662563585564698886972459726426527_u128 as f64;
_26 = [_16,_10,_10,_16];
_24 = _8 as f32;
Goto(bb7)
}
bb17 = {
_6 = 278753856613881059211073317555454785540_u128 as u64;
Goto(bb8)
}
bb18 = {
_4 = 139159304951154919706351987342610743528_u128 as isize;
RET = [6348820277962813275_i64,8003002175602885507_i64,1049660887243668960_i64,(-5257521736117079871_i64),(-5857602217850708268_i64),(-4107790858669022123_i64)];
_8 = _12;
RET = [2678493739758298494_i64,4561783142498119747_i64,8419725112510284137_i64,(-2136914565126630356_i64),(-874870471397070409_i64),(-538455142732617212_i64)];
_22 = (-19042_i16);
_19 = [_2,_2,_2,_6,_6,_2];
_17 = [15_i8,125_i8,57_i8,(-9_i8),33_i8,(-29_i8),(-69_i8)];
_28 = -(-7673027846955352603_i64);
RET = [_28,_28,_28,_28,_28,_28];
_8 = -_12;
_26 = [_10,_16,_10,_16];
RET = [_28,_28,_28,_28,_28,_28];
_4 = _20;
_26 = [_10,_10,_16,_16];
_12 = _8 + _8;
match _22 {
0 => bb6,
1 => bb4,
340282366920938463463374607431768192414 => bb10,
_ => bb9
}
}
bb19 = {
_32 = _28 ^ _28;
_3 = _15;
(*_1) = 175210815376884945839499333894420836896_u128 - 333091359711816850759817204083621217665_u128;
_39.3 = 180_u8 as u32;
_29.3 = !false;
_29 = (427196358_i32, (-145842673896983543634354565727582018094_i128), _32, true);
_33 = -_35;
_32 = (*_1) as i64;
_37 = -_24;
_15 = [(-28_i8),125_i8,(-75_i8),(-12_i8),(-76_i8),44_i8,(-7_i8)];
_32 = _29.2 * _28;
_19 = [_2,_2,_6,_2,_2,_2];
_38 = [_16,_10];
_39.2 = -_29.0;
_39.0 = -_29.1;
_32 = _28;
_39.1 = _7;
(*_30) = [_29.2,_25,_29.2,_29.2,_32,_25];
_39.1 = _7;
_4 = (*_11);
(*_30) = [_25,_25,_25,_32,_29.2,_29.2];
_16 = _25 as usize;
RET = _31.1;
_1 = core::ptr::addr_of_mut!((*_1));
_15 = _17;
_29.0 = !_39.2;
Goto(bb20)
}
bb20 = {
Call(_45 = dump_var(5_usize, 34_usize, Move(_34), 5_usize, Move(_5), 7_usize, Move(_7), 38_usize, Move(_38)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_45 = dump_var(5_usize, 28_usize, Move(_28), 15_usize, Move(_15), 3_usize, Move(_3), 4_usize, Move(_4)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_45 = dump_var(5_usize, 29_usize, Move(_29), 32_usize, Move(_32), 20_usize, Move(_20), 46_usize, _46), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: [bool; 5],mut _2: [i8; 7],mut _3: [i16; 8],mut _4: [i16; 8],mut _5: [i8; 7]) -> isize {
mir! {
type RET = isize;
let _6: isize;
let _7: i32;
let _8: isize;
let _9: (*const i16, (u8, u64, (u8, i64, u64)), [i32; 3]);
let _10: isize;
let _11: char;
let _12: (u8, i64, u64);
let _13: isize;
let _14: i64;
let _15: char;
let _16: &'static u64;
let _17: *mut u128;
let _18: bool;
let _19: Adt27;
let _20: (i32, i128, i64, bool);
let _21: &'static f32;
let _22: (i128, char, i32, u32);
let _23: ();
let _24: ();
{
_2 = [110_i8,(-110_i8),(-76_i8),85_i8,(-100_i8),18_i8,76_i8];
_5 = [(-105_i8),(-72_i8),(-33_i8),94_i8,(-16_i8),63_i8,(-128_i8)];
_6 = 9223372036854775807_isize + (-9223372036854775808_isize);
_2 = _5;
_4 = [(-666_i16),(-153_i16),(-26598_i16),5326_i16,734_i16,(-26301_i16),7710_i16,(-29578_i16)];
RET = !_6;
_2 = [25_i8,97_i8,(-27_i8),7_i8,16_i8,(-70_i8),(-98_i8)];
Call(_1 = fn7(), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = [true,true,true,false,true];
_6 = RET;
RET = !_6;
RET = _6;
_5 = [12_i8,37_i8,35_i8,74_i8,(-106_i8),(-24_i8),(-45_i8)];
RET = _6;
_2 = [(-105_i8),48_i8,(-11_i8),38_i8,61_i8,(-57_i8),54_i8];
RET = '\u{cdd00}' as isize;
RET = !_6;
_8 = _6;
_7 = (-1157311848_i32) - (-116177114_i32);
_4 = [(-28328_i16),6312_i16,(-9414_i16),(-7586_i16),(-18045_i16),(-3852_i16),(-15919_i16),14622_i16];
Goto(bb2)
}
bb2 = {
_9.1.1 = 339661579842819133925714145659790268555_u128 as u64;
_4 = _3;
_9.1.2 = (10_u8, 5296391520180123139_i64, _9.1.1);
_9.1.0 = !_9.1.2.0;
_2 = [36_i8,(-110_i8),(-87_i8),88_i8,13_i8,(-124_i8),(-40_i8)];
RET = !_8;
_9.2 = [_7,_7,_7];
_14 = _9.1.2.1 ^ _9.1.2.1;
Goto(bb3)
}
bb3 = {
Goto(bb4)
}
bb4 = {
_12.1 = _9.1.1 as i64;
_9.1.2.0 = _7 as u8;
RET = _8 + _8;
RET = (-31092_i16) as isize;
_1 = [false,false,true,true,true];
_9.1.2.0 = (-6256_i16) as u8;
_9.1.2.0 = _9.1.0 >> _14;
_12 = (_9.1.2.0, _14, _9.1.2.2);
_13 = _8 & _6;
_9.1.1 = !_12.2;
_11 = '\u{9d5c3}';
_16 = &_9.1.2.2;
_1 = [false,true,true,true,false];
_11 = '\u{718a7}';
match _9.1.2.1 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
5296391520180123139 => bb8,
_ => bb7
}
}
bb5 = {
Goto(bb4)
}
bb6 = {
_9.1.1 = 339661579842819133925714145659790268555_u128 as u64;
_4 = _3;
_9.1.2 = (10_u8, 5296391520180123139_i64, _9.1.1);
_9.1.0 = !_9.1.2.0;
_2 = [36_i8,(-110_i8),(-87_i8),88_i8,13_i8,(-124_i8),(-40_i8)];
RET = !_8;
_9.2 = [_7,_7,_7];
_14 = _9.1.2.1 ^ _9.1.2.1;
Goto(bb3)
}
bb7 = {
_1 = [true,true,true,false,true];
_6 = RET;
RET = !_6;
RET = _6;
_5 = [12_i8,37_i8,35_i8,74_i8,(-106_i8),(-24_i8),(-45_i8)];
RET = _6;
_2 = [(-105_i8),48_i8,(-11_i8),38_i8,61_i8,(-57_i8),54_i8];
RET = '\u{cdd00}' as isize;
RET = !_6;
_8 = _6;
_7 = (-1157311848_i32) - (-116177114_i32);
_4 = [(-28328_i16),6312_i16,(-9414_i16),(-7586_i16),(-18045_i16),(-3852_i16),(-15919_i16),14622_i16];
Goto(bb2)
}
bb8 = {
_2 = [(-29_i8),17_i8,(-66_i8),(-114_i8),7_i8,(-64_i8),(-53_i8)];
_15 = _11;
_11 = _15;
_5 = [(-27_i8),67_i8,(-94_i8),3_i8,(-87_i8),(-86_i8),46_i8];
match _9.1.2.1 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
5296391520180123139 => bb17,
_ => bb16
}
}
bb9 = {
_1 = [true,true,true,false,true];
_6 = RET;
RET = !_6;
RET = _6;
_5 = [12_i8,37_i8,35_i8,74_i8,(-106_i8),(-24_i8),(-45_i8)];
RET = _6;
_2 = [(-105_i8),48_i8,(-11_i8),38_i8,61_i8,(-57_i8),54_i8];
RET = '\u{cdd00}' as isize;
RET = !_6;
_8 = _6;
_7 = (-1157311848_i32) - (-116177114_i32);
_4 = [(-28328_i16),6312_i16,(-9414_i16),(-7586_i16),(-18045_i16),(-3852_i16),(-15919_i16),14622_i16];
Goto(bb2)
}
bb10 = {
_9.1.1 = 339661579842819133925714145659790268555_u128 as u64;
_4 = _3;
_9.1.2 = (10_u8, 5296391520180123139_i64, _9.1.1);
_9.1.0 = !_9.1.2.0;
_2 = [36_i8,(-110_i8),(-87_i8),88_i8,13_i8,(-124_i8),(-40_i8)];
RET = !_8;
_9.2 = [_7,_7,_7];
_14 = _9.1.2.1 ^ _9.1.2.1;
Goto(bb3)
}
bb11 = {
Goto(bb4)
}
bb12 = {
_12.1 = _9.1.1 as i64;
_9.1.2.0 = _7 as u8;
RET = _8 + _8;
RET = (-31092_i16) as isize;
_1 = [false,false,true,true,true];
_9.1.2.0 = (-6256_i16) as u8;
_9.1.2.0 = _9.1.0 >> _14;
_12 = (_9.1.2.0, _14, _9.1.2.2);
_13 = _8 & _6;
_9.1.1 = !_12.2;
_11 = '\u{9d5c3}';
_16 = &_9.1.2.2;
_1 = [false,true,true,true,false];
_11 = '\u{718a7}';
match _9.1.2.1 {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
5296391520180123139 => bb8,
_ => bb7
}
}
bb13 = {
Goto(bb4)
}
bb14 = {
_9.1.1 = 339661579842819133925714145659790268555_u128 as u64;
_4 = _3;
_9.1.2 = (10_u8, 5296391520180123139_i64, _9.1.1);
_9.1.0 = !_9.1.2.0;
_2 = [36_i8,(-110_i8),(-87_i8),88_i8,13_i8,(-124_i8),(-40_i8)];
RET = !_8;
_9.2 = [_7,_7,_7];
_14 = _9.1.2.1 ^ _9.1.2.1;
Goto(bb3)
}
bb15 = {
_1 = [true,true,true,false,true];
_6 = RET;
RET = !_6;
RET = _6;
_5 = [12_i8,37_i8,35_i8,74_i8,(-106_i8),(-24_i8),(-45_i8)];
RET = _6;
_2 = [(-105_i8),48_i8,(-11_i8),38_i8,61_i8,(-57_i8),54_i8];
RET = '\u{cdd00}' as isize;
RET = !_6;
_8 = _6;
_7 = (-1157311848_i32) - (-116177114_i32);
_4 = [(-28328_i16),6312_i16,(-9414_i16),(-7586_i16),(-18045_i16),(-3852_i16),(-15919_i16),14622_i16];
Goto(bb2)
}
bb16 = {
Return()
}
bb17 = {
_8 = _6 + _13;
_20.1 = 138894018418637690732980518132196664487_i128;
_4 = [24837_i16,(-13138_i16),5925_i16,(-24618_i16),17826_i16,(-25307_i16),2470_i16,29983_i16];
_20.2 = _12.0 as i64;
_8 = (-17901_i16) as isize;
_14 = _12.1;
_20.2 = 1886462119_u32 as i64;
_9.2 = [_7,_7,_7];
_8 = !_6;
_15 = _11;
Goto(bb18)
}
bb18 = {
Call(_23 = dump_var(6_usize, 6_usize, Move(_6), 7_usize, Move(_7), 3_usize, Move(_3), 12_usize, Move(_12)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_23 = dump_var(6_usize, 4_usize, Move(_4), 13_usize, Move(_13), 24_usize, _24, 24_usize, _24), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7() -> [bool; 5] {
mir! {
type RET = [bool; 5];
let _1: i128;
let _2: (i128, char, i32, u32);
let _3: i128;
let _4: f32;
let _5: f32;
let _6: [i32; 3];
let _7: u32;
let _8: i16;
let _9: &'static [bool; 5];
let _10: Adt50;
let _11: f32;
let _12: [isize; 2];
let _13: f32;
let _14: [i8; 7];
let _15: *const *const i128;
let _16: &'static isize;
let _17: *const i16;
let _18: u32;
let _19: &'static i128;
let _20: *const (u8, i64, u64);
let _21: f64;
let _22: (f64, (*const i16, (u8, u64, (u8, i64, u64)), [i32; 3]), char, &'static isize);
let _23: char;
let _24: *const i16;
let _25: Adt27;
let _26: u32;
let _27: isize;
let _28: char;
let _29: ([i64; 6], &'static f32, *const (u128, [i64; 6]));
let _30: &'static *mut &'static u64;
let _31: ();
let _32: ();
{
RET = [true,false,false,true,false];
Goto(bb1)
}
bb1 = {
RET = [false,false,false,true,true];
RET = [false,false,true,false,true];
_1 = -(-94739832439668573624527606749288843326_i128);
_2.3 = 9223372036854775807_isize as u32;
Goto(bb2)
}
bb2 = {
_2.1 = '\u{a09eb}';
_2.1 = '\u{c71fd}';
_2 = (_1, '\u{7e10b}', 153849135_i32, 1286775037_u32);
_2 = (_1, '\u{ca596}', 575549618_i32, 3120376954_u32);
_2.2 = !(-486594611_i32);
_2.0 = (-6745554436743662713_i64) as i128;
_2.2 = 1296024144_i32;
_2.0 = _1 + _1;
_2.0 = _1;
_2.1 = '\u{15ee7}';
_2 = (_1, '\u{c0858}', (-714233506_i32), 2958266874_u32);
_1 = _2.0;
RET = [false,false,true,false,false];
RET = [false,false,false,false,false];
_2 = (_1, '\u{3e360}', (-451748802_i32), 1266473855_u32);
Call(_2.0 = fn8(_2.1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_2.2 = (-7980725682291998316_i64) as i32;
RET = [false,true,true,false,false];
_2.3 = 11099_u16 as u32;
_2.1 = '\u{99e9b}';
_2 = (_1, '\u{7240c}', 111543300_i32, 2883271311_u32);
_2.2 = -(-1484462394_i32);
_1 = _2.0;
_2.0 = 91899843014958314838768596996297307262_u128 as i128;
_3 = 203_u8 as i128;
_2.2 = !(-1673363817_i32);
_2.3 = 465190999_u32 ^ 3236316728_u32;
_2.2 = 1678616120_i32;
Call(RET = fn10(_2, _2.2, _1, _2), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_2.0 = _2.3 as i128;
_2.3 = !3161085084_u32;
_2.1 = '\u{cf1bd}';
_2.1 = '\u{23195}';
match _2.2 {
1678616120 => bb5,
_ => bb2
}
}
bb5 = {
RET = [false,false,true,false,false];
Call(_2.3 = fn19(_3, _2.1, RET), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_2 = (_1, '\u{910e4}', (-1330952316_i32), 1639167849_u32);
_1 = (-18700_i16) as i128;
match _2.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
340282366920938463463374607430437259140 => bb9,
_ => bb8
}
}
bb7 = {
_2.2 = (-7980725682291998316_i64) as i32;
RET = [false,true,true,false,false];
_2.3 = 11099_u16 as u32;
_2.1 = '\u{99e9b}';
_2 = (_1, '\u{7240c}', 111543300_i32, 2883271311_u32);
_2.2 = -(-1484462394_i32);
_1 = _2.0;
_2.0 = 91899843014958314838768596996297307262_u128 as i128;
_3 = 203_u8 as i128;
_2.2 = !(-1673363817_i32);
_2.3 = 465190999_u32 ^ 3236316728_u32;
_2.2 = 1678616120_i32;
Call(RET = fn10(_2, _2.2, _1, _2), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_2.0 = _2.3 as i128;
_2.3 = !3161085084_u32;
_2.1 = '\u{cf1bd}';
_2.1 = '\u{23195}';
match _2.2 {
1678616120 => bb5,
_ => bb2
}
}
bb9 = {
_5 = (-2692751155532949540_i64) as f32;
RET = [false,true,true,true,false];
match _2.3 {
1639167849 => bb10,
_ => bb3
}
}
bb10 = {
_2 = (_3, '\u{be78a}', 50477218_i32, 663447044_u32);
RET = [true,true,true,false,true];
_1 = 85_i8 as i128;
_1 = 12102718007539592030_u64 as i128;
_1 = _3;
_3 = _2.0;
match _2.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
663447044 => bb11,
_ => bb8
}
}
bb11 = {
_8 = (-30508_i16);
RET = [false,true,false,true,true];
RET = [true,false,false,true,false];
_7 = _5 as u32;
_5 = 5353447922128747863_usize as f32;
_2 = (_1, '\u{404d0}', 741950275_i32, _7);
_2.3 = !_7;
_1 = 207_u8 as i128;
_10.fld0 = [2821841409102638396_i64,(-3921683631820942704_i64),4160331523457882876_i64,(-3771486964785310787_i64),3065577766612819237_i64,4483464483642053010_i64];
_5 = _8 as f32;
_6 = [_2.2,_2.2,_2.2];
_9 = &RET;
_2.1 = '\u{237d}';
_8 = 3336_i16;
_11 = 9223372036854775807_isize as f32;
_2 = (_3, '\u{66043}', (-568118289_i32), _7);
_5 = 89_u8 as f32;
_1 = _2.0 - _3;
Goto(bb12)
}
bb12 = {
_13 = 234_u8 as f32;
_6 = [_2.2,_2.2,_2.2];
_8 = (-1271_i16) + (-24076_i16);
_12 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
_6 = [_2.2,_2.2,_2.2];
_2 = (_1, '\u{e76bc}', 426478406_i32, _7);
_14 = [9_i8,(-100_i8),3_i8,23_i8,66_i8,124_i8,84_i8];
_9 = &(*_9);
_4 = _13 - _5;
match _2.2 {
0 => bb1,
1 => bb2,
2 => bb9,
3 => bb4,
4 => bb11,
426478406 => bb13,
_ => bb6
}
}
bb13 = {
_11 = 189_u8 as f32;
RET = [true,false,false,true,true];
_9 = &RET;
_1 = _2.0;
_17 = core::ptr::addr_of!(_8);
_2 = (_3, '\u{a99e8}', 1051548973_i32, _7);
_2.0 = _1;
_18 = !_7;
(*_17) = _1 as i16;
_21 = _5 as f64;
_22.1.1.2 = (37_u8, 9201157831976756820_i64, 16857796835763245611_u64);
_22.1.1.2.2 = _21 as u64;
_22.1.1.2 = (49_u8, 6418979271882488939_i64, 757191784163655186_u64);
(*_17) = 26885_i16;
_24 = Move(_17);
_19 = &_3;
_2.3 = 108727490393180096793628364297193587028_u128 as u32;
_22.2 = _2.1;
_5 = _13 + _4;
Goto(bb14)
}
bb14 = {
_2.1 = _22.2;
_25 = Adt27::Variant0 { fld0: _2,fld1: (*_19),fld2: _8,fld3: 36877_u16 };
_5 = 57_i8 as f32;
_22.2 = _2.1;
_17 = Move(_24);
_22.1.0 = Move(_17);
_25 = Adt27::Variant0 { fld0: _2,fld1: (*_19),fld2: _8,fld3: 36387_u16 };
place!(Field::<(i128, char, i32, u32)>(Variant(_25, 0), 0)) = (_2.0, _2.1, _2.2, _2.3);
_21 = 61349_u16 as f64;
_20 = core::ptr::addr_of!(_22.1.1.2);
(*_20).2 = 11437286769580506804_u64 | 9688934321509244761_u64;
_20 = core::ptr::addr_of!((*_20));
_2.1 = Field::<(i128, char, i32, u32)>(Variant(_25, 0), 0).1;
_22.1.1.2.0 = 40_u8;
(*_20).1 = (-2178428347453032176_i64) & 3088927607100453728_i64;
_23 = Field::<(i128, char, i32, u32)>(Variant(_25, 0), 0).1;
(*_20).0 = !87_u8;
_22.1.1.2.0 = 188_u8 & 94_u8;
_22.1.1.2.2 = !10289102384032491803_u64;
place!(Field::<u16>(Variant(_25, 0), 3)) = true as u16;
(*_20).2 = 11249572414158015447_u64;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(7_usize, 12_usize, Move(_12), 8_usize, Move(_8), 2_usize, Move(_2), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(7_usize, 18_usize, Move(_18), 32_usize, _32, 32_usize, _32, 32_usize, _32), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: char) -> i128 {
mir! {
type RET = i128;
let _2: Adt50;
let _3: Adt44;
let _4: usize;
let _5: f64;
let _6: Adt53;
let _7: char;
let _8: [usize; 2];
let _9: u8;
let _10: &'static [bool; 5];
let _11: ();
let _12: ();
{
_1 = '\u{e3c95}';
_1 = '\u{739e1}';
RET = (-98625294573619180172037596212186147737_i128);
_1 = '\u{52412}';
_1 = '\u{f1894}';
_1 = '\u{1cd44}';
_1 = '\u{efff8}';
RET = 109642397796247804250479541512213658449_i128;
_1 = '\u{d9f1a}';
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
109642397796247804250479541512213658449 => bb6,
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
_2.fld0 = [8204067442696030078_i64,7977622536384997926_i64,8811182833355203751_i64,1640289313011843336_i64,(-4870443723574247793_i64),(-1907405154721763481_i64)];
_2.fld0 = [328435303712648957_i64,(-2628818876864231610_i64),(-745956930460735314_i64),(-6077391193087132748_i64),4617320432413299220_i64,(-1532122738878591976_i64)];
_1 = '\u{e5d2f}';
RET = 37928520222011663574327904173330635191_i128;
RET = (-134978116779264240055320543308298747082_i128) >> 140791364120206597684713770686841042938_i128;
_4 = 2900646746723656501_usize + 4_usize;
RET = 18438_u16 as i128;
_4 = 7_usize << RET;
RET = 963426893_u32 as i128;
_1 = '\u{5a92f}';
_5 = RET as f64;
_4 = 3491233416_u32 as usize;
_3 = Adt44::Variant1 { fld0: _2.fld0,fld1: _1 };
place!(Field::<char>(Variant(_3, 1), 1)) = _1;
Call(RET = fn9(Move(_3), _2.fld0, _2.fld0, _1, _2.fld0, _5, _1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_7 = _1;
_7 = _1;
_3 = Adt44::Variant1 { fld0: _2.fld0,fld1: _1 };
place!(Field::<[i64; 6]>(Variant(_3, 1), 0)) = [(-5670611856629251270_i64),(-2096779634124200469_i64),(-2155553916373145515_i64),(-8259680106017547616_i64),864910694144609161_i64,(-6486872744992915637_i64)];
RET = 28824434457295137724783100756213097536_i128;
_7 = _1;
_5 = 95_isize as f64;
_5 = 42979_u16 as f64;
_7 = _1;
_4 = !11669251698360020478_usize;
_8 = [_4,_4];
place!(Field::<[i64; 6]>(Variant(_3, 1), 0)) = [321641064147930168_i64,(-2166681654051678797_i64),2189539200879722745_i64,(-4066066120622473919_i64),5255121085467043327_i64,(-2868139781476462495_i64)];
_2 = Adt50 { fld0: Field::<[i64; 6]>(Variant(_3, 1), 0) };
place!(Field::<[i64; 6]>(Variant(_3, 1), 0)) = [6565145176971011887_i64,(-8224044378400782951_i64),4113381826204123536_i64,2215289682612686825_i64,(-3447385681442287171_i64),6352383452838674672_i64];
_4 = 1_usize;
_5 = 243_u8 as f64;
RET = -97698651450452990928127611627407200802_i128;
_8[_4] = 119897774324094268814457176275710378875_u128 as usize;
_2.fld0[_4] = -Field::<[i64; 6]>(Variant(_3, 1), 0)[_4];
_3 = Adt44::Variant1 { fld0: _2.fld0,fld1: _7 };
_8 = [_4,_4];
RET = (-35699246713208887373695824687795342279_i128) ^ (-119241325553227127623347311759215718941_i128);
_2.fld0[_4] = Field::<[i64; 6]>(Variant(_3, 1), 0)[_4] | Field::<[i64; 6]>(Variant(_3, 1), 0)[_4];
_8[_4] = 9223372036854775807_isize as usize;
place!(Field::<[i64; 6]>(Variant(_3, 1), 0)) = [_2.fld0[_4],_2.fld0[_4],_2.fld0[_4],_2.fld0[_4],_2.fld0[_4],_2.fld0[_4]];
_2.fld0[_4] = RET as i64;
RET = (-142109229527252706957065360219398948335_i128) >> Field::<[i64; 6]>(Variant(_3, 1), 0)[_4];
_9 = !204_u8;
Goto(bb8)
}
bb8 = {
Call(_11 = dump_var(8_usize, 7_usize, Move(_7), 1_usize, Move(_1), 12_usize, _12, 12_usize, _12), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: Adt44,mut _2: [i64; 6],mut _3: [i64; 6],mut _4: char,mut _5: [i64; 6],mut _6: f64,mut _7: char) -> i128 {
mir! {
type RET = i128;
let _8: *mut [i64; 6];
let _9: &'static char;
let _10: *const i128;
let _11: *const i16;
let _12: [i64; 4];
let _13: *const (u8, i64, u64);
let _14: (i32, i128, i64, bool);
let _15: u16;
let _16: [u16; 8];
let _17: [i32; 7];
let _18: isize;
let _19: Adt46;
let _20: i16;
let _21: ();
let _22: ();
{
_4 = Field::<char>(Variant(_1, 1), 1);
RET = 60780748461620898893609372627542418021_i128;
_5 = [2119316505610998309_i64,(-7008762991594529308_i64),8653686566042656853_i64,2412715702532547840_i64,(-5364570304126636129_i64),(-534521870669389507_i64)];
RET = -114685511347482343515573272728320247656_i128;
_3 = _5;
place!(Field::<char>(Variant(_1, 1), 1)) = _4;
_3 = [(-4886506832560305848_i64),(-3762163131489632882_i64),5578298021277860575_i64,5258727171738074488_i64,(-7773144664463893402_i64),6443192256251596580_i64];
_5 = _3;
_8 = core::ptr::addr_of_mut!(_2);
RET = -19221846879167970660528858561829427609_i128;
(*_8) = [(-2716428693244272458_i64),(-9012896471206235088_i64),(-1692375054083288720_i64),(-8401815655664073455_i64),2139095141977282280_i64,(-5840217243579838802_i64)];
Call(RET = core::intrinsics::bswap(110613693365681038443121557583774263411_i128), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = 1663654625431573741_u64 as f64;
place!(Field::<[i64; 6]>(Variant(_1, 1), 0)) = (*_8);
_2 = [(-2190756241574409222_i64),8088738427624907077_i64,297059636528578075_i64,(-4523184813889318720_i64),(-1359804610026427544_i64),(-1695984398971003706_i64)];
SetDiscriminant(_1, 3);
place!(Field::<u8>(Variant(_1, 3), 2)) = 142_u8;
place!(Field::<i8>(Variant(_1, 3), 3)) = 101_i8;
_4 = _7;
_4 = _7;
_3 = [(-9065495633813146954_i64),(-4336281738312594870_i64),(-4231575684223993266_i64),6179373519662027017_i64,(-939053008249983946_i64),9075352202119232920_i64];
_9 = &_7;
_11 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_1, 3), 4)));
_6 = 2779679105_u32 as f64;
place!(Field::<i16>(Variant(_1, 3), 4)) = !31687_i16;
(*_11) = 2826_i16;
_7 = _4;
(*_8) = [(-4416037324740381574_i64),(-8066821620167855964_i64),1997941800647375366_i64,3504053031468756667_i64,(-5168158425641195302_i64),(-5891776069356360365_i64)];
RET = (-24510735385065637662300140876263842283_i128) + 41691219977475791087798371886858490612_i128;
_3 = [6545151677065084971_i64,3527983976812394286_i64,(-9047432579977829011_i64),(-4324627578190231073_i64),(-6980321672240723062_i64),(-1038752948060729294_i64)];
place!(Field::<i128>(Variant(_1, 3), 7)) = RET << (*_11);
RET = !Field::<i128>(Variant(_1, 3), 7);
(*_8) = [(-5575064675142270221_i64),8507476978347171660_i64,3733800408753650306_i64,(-3650017437217425970_i64),(-3463390160667218364_i64),(-3144897871684845209_i64)];
place!(Field::<[usize; 2]>(Variant(_1, 3), 1)) = [8713286675729034795_usize,6_usize];
place!(Field::<i16>(Variant(_1, 3), 4)) = _7 as i16;
match Field::<i8>(Variant(_1, 3), 3) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
101 => bb7,
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
place!(Field::<i8>(Variant(_1, 3), 3)) = 11861493340234675972_usize as i8;
_4 = _7;
(*_11) = (-19363_i16);
place!(Field::<i8>(Variant(_1, 3), 3)) = (-69_i8) >> RET;
_8 = core::ptr::addr_of_mut!(_3);
_12 = [4349337322416173111_i64,(-6906831269524966339_i64),(-135434018432776337_i64),8303615398247309497_i64];
_12 = [(-7665923265196795186_i64),(-1960022830264900742_i64),7905404595320906768_i64,(-6083203050248737275_i64)];
_4 = _7;
_3 = [6848231765186601401_i64,1649246291052499363_i64,4219565300783293933_i64,(-1392787332837864656_i64),5455151281742107314_i64,1277096093395176212_i64];
place!(Field::<u8>(Variant(_1, 3), 2)) = 185_u8 * 217_u8;
_10 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_1, 3), 7)));
_9 = &_4;
(*_8) = [(-1749726854484488185_i64),(-4966096287764150062_i64),8085588776073141464_i64,8529732312975154790_i64,7092274669325275976_i64,(-2978861242954239517_i64)];
_11 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_1, 3), 4)));
_14 = (1595633486_i32, (*_10), (-5775037135131740569_i64), false);
_14.3 = true & true;
place!(Field::<i16>(Variant(_1, 3), 4)) = (-13241_i16);
place!(Field::<[usize; 2]>(Variant(_1, 3), 1)) = [10697402242470829209_usize,15622250641227320289_usize];
_14.3 = !false;
_17 = [_14.0,_14.0,_14.0,_14.0,_14.0,_14.0,_14.0];
_16 = [19679_u16,39552_u16,48013_u16,54330_u16,31796_u16,18112_u16,785_u16,49924_u16];
place!(Field::<[usize; 2]>(Variant(_1, 3), 1)) = [6255500724759011493_usize,15107140750757703064_usize];
_8 = core::ptr::addr_of_mut!(_2);
_14.1 = (*_10) + RET;
_15 = 24554_u16;
(*_10) = Field::<i16>(Variant(_1, 3), 4) as i128;
(*_10) = 4_usize as i128;
Goto(bb8)
}
bb8 = {
_8 = core::ptr::addr_of_mut!(_2);
_4 = _7;
_12 = [_14.2,_14.2,_14.2,_14.2];
_18 = -9223372036854775807_isize;
place!(Field::<[usize; 2]>(Variant(_1, 3), 1)) = [12762385015477684767_usize,6_usize];
match _14.2 {
0 => bb1,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
340282366920938463457599570296636470887 => bb15,
_ => bb14
}
}
bb9 = {
place!(Field::<i8>(Variant(_1, 3), 3)) = 11861493340234675972_usize as i8;
_4 = _7;
(*_11) = (-19363_i16);
place!(Field::<i8>(Variant(_1, 3), 3)) = (-69_i8) >> RET;
_8 = core::ptr::addr_of_mut!(_3);
_12 = [4349337322416173111_i64,(-6906831269524966339_i64),(-135434018432776337_i64),8303615398247309497_i64];
_12 = [(-7665923265196795186_i64),(-1960022830264900742_i64),7905404595320906768_i64,(-6083203050248737275_i64)];
_4 = _7;
_3 = [6848231765186601401_i64,1649246291052499363_i64,4219565300783293933_i64,(-1392787332837864656_i64),5455151281742107314_i64,1277096093395176212_i64];
place!(Field::<u8>(Variant(_1, 3), 2)) = 185_u8 * 217_u8;
_10 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_1, 3), 7)));
_9 = &_4;
(*_8) = [(-1749726854484488185_i64),(-4966096287764150062_i64),8085588776073141464_i64,8529732312975154790_i64,7092274669325275976_i64,(-2978861242954239517_i64)];
_11 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_1, 3), 4)));
_14 = (1595633486_i32, (*_10), (-5775037135131740569_i64), false);
_14.3 = true & true;
place!(Field::<i16>(Variant(_1, 3), 4)) = (-13241_i16);
place!(Field::<[usize; 2]>(Variant(_1, 3), 1)) = [10697402242470829209_usize,15622250641227320289_usize];
_14.3 = !false;
_17 = [_14.0,_14.0,_14.0,_14.0,_14.0,_14.0,_14.0];
_16 = [19679_u16,39552_u16,48013_u16,54330_u16,31796_u16,18112_u16,785_u16,49924_u16];
place!(Field::<[usize; 2]>(Variant(_1, 3), 1)) = [6255500724759011493_usize,15107140750757703064_usize];
_8 = core::ptr::addr_of_mut!(_2);
_14.1 = (*_10) + RET;
_15 = 24554_u16;
(*_10) = Field::<i16>(Variant(_1, 3), 4) as i128;
(*_10) = 4_usize as i128;
Goto(bb8)
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
_6 = 1663654625431573741_u64 as f64;
place!(Field::<[i64; 6]>(Variant(_1, 1), 0)) = (*_8);
_2 = [(-2190756241574409222_i64),8088738427624907077_i64,297059636528578075_i64,(-4523184813889318720_i64),(-1359804610026427544_i64),(-1695984398971003706_i64)];
SetDiscriminant(_1, 3);
place!(Field::<u8>(Variant(_1, 3), 2)) = 142_u8;
place!(Field::<i8>(Variant(_1, 3), 3)) = 101_i8;
_4 = _7;
_4 = _7;
_3 = [(-9065495633813146954_i64),(-4336281738312594870_i64),(-4231575684223993266_i64),6179373519662027017_i64,(-939053008249983946_i64),9075352202119232920_i64];
_9 = &_7;
_11 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_1, 3), 4)));
_6 = 2779679105_u32 as f64;
place!(Field::<i16>(Variant(_1, 3), 4)) = !31687_i16;
(*_11) = 2826_i16;
_7 = _4;
(*_8) = [(-4416037324740381574_i64),(-8066821620167855964_i64),1997941800647375366_i64,3504053031468756667_i64,(-5168158425641195302_i64),(-5891776069356360365_i64)];
RET = (-24510735385065637662300140876263842283_i128) + 41691219977475791087798371886858490612_i128;
_3 = [6545151677065084971_i64,3527983976812394286_i64,(-9047432579977829011_i64),(-4324627578190231073_i64),(-6980321672240723062_i64),(-1038752948060729294_i64)];
place!(Field::<i128>(Variant(_1, 3), 7)) = RET << (*_11);
RET = !Field::<i128>(Variant(_1, 3), 7);
(*_8) = [(-5575064675142270221_i64),8507476978347171660_i64,3733800408753650306_i64,(-3650017437217425970_i64),(-3463390160667218364_i64),(-3144897871684845209_i64)];
place!(Field::<[usize; 2]>(Variant(_1, 3), 1)) = [8713286675729034795_usize,6_usize];
place!(Field::<i16>(Variant(_1, 3), 4)) = _7 as i16;
match Field::<i8>(Variant(_1, 3), 3) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
101 => bb7,
_ => bb6
}
}
bb14 = {
Return()
}
bb15 = {
(*_10) = 194155045313815914983064372900088903760_u128 as i128;
_8 = core::ptr::addr_of_mut!((*_8));
_14.2 = 6348970369850874969_i64 << RET;
_5 = [_14.2,_14.2,_14.2,_14.2,_14.2,_14.2];
Goto(bb16)
}
bb16 = {
Call(_21 = dump_var(9_usize, 4_usize, Move(_4), 18_usize, Move(_18), 2_usize, Move(_2), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_21 = dump_var(9_usize, 5_usize, Move(_5), 22_usize, _22, 22_usize, _22, 22_usize, _22), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: (i128, char, i32, u32),mut _2: i32,mut _3: i128,mut _4: (i128, char, i32, u32)) -> [bool; 5] {
mir! {
type RET = [bool; 5];
let _5: [i32; 3];
let _6: &'static i128;
let _7: [u64; 6];
let _8: *const i128;
let _9: u128;
let _10: &'static Adt50;
let _11: isize;
let _12: u8;
let _13: [usize; 2];
let _14: ([i32; 7], &'static *mut &'static u64, [i16; 8]);
let _15: &'static Adt44;
let _16: f32;
let _17: f64;
let _18: Adt73;
let _19: [i8; 7];
let _20: isize;
let _21: &'static (i32, i128, i64, bool);
let _22: [u64; 6];
let _23: u64;
let _24: (Adt34, &'static *mut &'static u64, ((i128, *const isize), *mut u128, Adt44));
let _25: isize;
let _26: f64;
let _27: bool;
let _28: isize;
let _29: [bool; 6];
let _30: *const Adt53;
let _31: f64;
let _32: Adt27;
let _33: [usize; 4];
let _34: &'static u64;
let _35: *const Adt53;
let _36: f64;
let _37: bool;
let _38: char;
let _39: char;
let _40: *const Adt53;
let _41: i8;
let _42: *const Adt53;
let _43: i128;
let _44: *const &'static u64;
let _45: [u64; 6];
let _46: ();
let _47: ();
{
_1.3 = _4.3;
_4.2 = _1.2;
_1.0 = _3 & _3;
_4.1 = _1.1;
_4.1 = _1.1;
_2 = _4.2;
RET = [true,true,true,true,false];
_2 = !_4.2;
RET = [false,true,true,false,false];
_4.3 = _1.3 - _1.3;
_1.0 = _3;
_1 = _4;
_1.3 = true as u32;
_1.0 = _4.1 as i128;
_5 = [_1.2,_4.2,_1.2];
_5 = [_1.2,_4.2,_2];
_4 = (_1.0, _1.1, _2, _1.3);
RET = [true,true,false,false,false];
_1.1 = _4.1;
_1.3 = 50734_u16 as u32;
_1.3 = _4.3;
Call(_1.2 = fn11(_1.1, _1.0), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1.2 = (-9223372036854775808_isize) as i32;
_1.2 = 17355_u16 as i32;
_7 = [4457165518718000193_u64,17760539378624156611_u64,11606501256869883908_u64,10487855372594861128_u64,4984760233956105238_u64,16401218311680956866_u64];
_5 = [_2,_4.2,_4.2];
_2 = _1.2;
_1.0 = _4.0;
_1.0 = _3;
_1.0 = _4.0 & _3;
_4.2 = _2 | _1.2;
_1.3 = _4.3 * _4.3;
_2 = 10325544418729041739_u64 as i32;
_8 = core::ptr::addr_of!(_1.0);
_5 = [_4.2,_4.2,_4.2];
_6 = &_1.0;
_4.2 = _2 | _2;
_6 = &(*_6);
_11 = 60_u8 as isize;
_6 = &(*_6);
_1.1 = _4.1;
RET = [true,false,false,true,true];
_1.2 = _4.2;
_7 = [17357098419039612353_u64,10102063409631626839_u64,10512768267302495588_u64,13814672242942658271_u64,9786978272476579231_u64,4639838132310286723_u64];
Goto(bb2)
}
bb2 = {
_4.2 = _1.3 as i32;
_9 = !208248399561309224083606896169008703229_u128;
_7 = [17111063325018773248_u64,2295143829113981171_u64,8547351150183238306_u64,16336231244799453639_u64,4546435293806074365_u64,2033952841406363643_u64];
RET = [true,true,true,false,false];
_1.1 = _4.1;
_12 = (-9571_i16) as u8;
_1.3 = _4.3;
_1.2 = _2 + _4.2;
_1.3 = _4.3;
_2 = !_4.2;
RET = [false,true,false,false,false];
RET = [true,true,true,false,false];
(*_8) = _4.0;
(*_8) = _4.0;
_8 = core::ptr::addr_of!(_4.0);
(*_8) = !_1.0;
_4.2 = -_2;
_1.2 = 1111793238343130736_usize as i32;
_3 = _11 as i128;
_7 = [2182229128963622240_u64,14134366389708828383_u64,9485048846805532345_u64,8529512255287127451_u64,16448078429356250773_u64,14841305853496453564_u64];
_4.1 = _1.1;
(*_8) = 4856796864443923370_u64 as i128;
_13 = [2443216735548368889_usize,2_usize];
(*_8) = false as i128;
_12 = 39619_u16 as u8;
_1 = _4;
Goto(bb3)
}
bb3 = {
_4.1 = _1.1;
_4 = (_3, _1.1, _2, _1.3);
_9 = !323548010341144390898548295420123131230_u128;
_14.0 = [_4.2,_1.2,_4.2,_4.2,_2,_4.2,_4.2];
_3 = -_1.0;
_6 = &(*_8);
RET = [false,true,true,true,true];
_2 = _4.2 | _1.2;
RET = [true,false,false,true,false];
_11 = true as isize;
_6 = &_4.0;
_14.2 = [27650_i16,(-32398_i16),(-27427_i16),(-17834_i16),(-20125_i16),(-22877_i16),(-29549_i16),25482_i16];
_14.0 = [_2,_2,_4.2,_4.2,_4.2,_1.2,_4.2];
_4.2 = -_2;
_2 = _1.2;
_2 = !_4.2;
_4.3 = false as u32;
Goto(bb4)
}
bb4 = {
_9 = !156017132103507056103817233910047808829_u128;
_4.3 = !_1.3;
_16 = _4.2 as f32;
_13 = [10003476332817859088_usize,5076072495954764755_usize];
(*_8) = _3 ^ _1.0;
_4.2 = _1.2;
_19 = [107_i8,(-101_i8),(-127_i8),19_i8,64_i8,72_i8,87_i8];
_4.3 = _1.3 - _1.3;
RET = [false,true,true,true,true];
_6 = &_4.0;
_9 = (*_8) as u128;
_2 = (-56_i8) as i32;
_1.3 = (*_6) as u32;
Call(_1.1 = fn17(Move(_6), _4.1, _14.2, Move(_8), _14.0, _4.1, _1.3), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_8 = core::ptr::addr_of!(_3);
_17 = _16 as f64;
_18 = Adt73::Variant0 { fld0: 33_i8,fld1: RET };
_9 = 106973555243543591620074685896033264922_u128 >> _1.2;
_9 = 20201146720288230682933456939277203150_u128;
_2 = _4.2 + _1.2;
_14.2 = [418_i16,(-5996_i16),12083_i16,(-16266_i16),(-31282_i16),(-22696_i16),28345_i16,29839_i16];
_1.2 = -_2;
_3 = _11 as i128;
_9 = !27440006751076355337711689912773026794_u128;
_24.0.fld6 = [_2,_1.2,_2];
_25 = 1890_u16 as isize;
_24.0.fld5 = [640489858147013248_i64,(-2200219150398509924_i64),(-8191624557213300927_i64),(-5519555462667180541_i64)];
_24.2.1 = core::ptr::addr_of_mut!(_9);
RET = Field::<[bool; 5]>(Variant(_18, 0), 1);
_5 = [_4.2,_1.2,_2];
(*_8) = _4.0;
_11 = _25;
_24.0.fld0 = core::ptr::addr_of!(_24.0.fld4);
_4 = (_1.0, _1.1, _1.2, _1.3);
_22 = [11788192927355552747_u64,7767300190323843770_u64,5339425978944331087_u64,3851819195917520418_u64,14007949036517958313_u64,10807568051484635891_u64];
Goto(bb6)
}
bb6 = {
_24.0.fld2 = _17 as usize;
_24.0.fld1.1 = _1.1;
_24.0.fld1.0 = (*_8);
_3 = _1.0 << _4.2;
_24.2.0.1 = core::ptr::addr_of!(_25);
_20 = !_11;
_3 = _24.0.fld1.0;
_24.0.fld1.3 = _1.3 & _1.3;
_1 = (_3, _24.0.fld1.1, _2, _4.3);
_15 = &_24.2.2;
_25 = -_11;
_24.0.fld3 = Adt27::Variant0 { fld0: _4,fld1: _3,fld2: 30839_i16,fld3: 28759_u16 };
_1.0 = (-49_i8) as i128;
_24.0.fld0 = core::ptr::addr_of!(_24.0.fld4);
_24.0.fld4 = !11098_i16;
_23 = 17855130183576673901_u64;
match _23 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb7,
5 => bb8,
6 => bb9,
17855130183576673901 => bb11,
_ => bb10
}
}
bb7 = {
_8 = core::ptr::addr_of!(_3);
_17 = _16 as f64;
_18 = Adt73::Variant0 { fld0: 33_i8,fld1: RET };
_9 = 106973555243543591620074685896033264922_u128 >> _1.2;
_9 = 20201146720288230682933456939277203150_u128;
_2 = _4.2 + _1.2;
_14.2 = [418_i16,(-5996_i16),12083_i16,(-16266_i16),(-31282_i16),(-22696_i16),28345_i16,29839_i16];
_1.2 = -_2;
_3 = _11 as i128;
_9 = !27440006751076355337711689912773026794_u128;
_24.0.fld6 = [_2,_1.2,_2];
_25 = 1890_u16 as isize;
_24.0.fld5 = [640489858147013248_i64,(-2200219150398509924_i64),(-8191624557213300927_i64),(-5519555462667180541_i64)];
_24.2.1 = core::ptr::addr_of_mut!(_9);
RET = Field::<[bool; 5]>(Variant(_18, 0), 1);
_5 = [_4.2,_1.2,_2];
(*_8) = _4.0;
_11 = _25;
_24.0.fld0 = core::ptr::addr_of!(_24.0.fld4);
_4 = (_1.0, _1.1, _1.2, _1.3);
_22 = [11788192927355552747_u64,7767300190323843770_u64,5339425978944331087_u64,3851819195917520418_u64,14007949036517958313_u64,10807568051484635891_u64];
Goto(bb6)
}
bb8 = {
_1.2 = (-9223372036854775808_isize) as i32;
_1.2 = 17355_u16 as i32;
_7 = [4457165518718000193_u64,17760539378624156611_u64,11606501256869883908_u64,10487855372594861128_u64,4984760233956105238_u64,16401218311680956866_u64];
_5 = [_2,_4.2,_4.2];
_2 = _1.2;
_1.0 = _4.0;
_1.0 = _3;
_1.0 = _4.0 & _3;
_4.2 = _2 | _1.2;
_1.3 = _4.3 * _4.3;
_2 = 10325544418729041739_u64 as i32;
_8 = core::ptr::addr_of!(_1.0);
_5 = [_4.2,_4.2,_4.2];
_6 = &_1.0;
_4.2 = _2 | _2;
_6 = &(*_6);
_11 = 60_u8 as isize;
_6 = &(*_6);
_1.1 = _4.1;
RET = [true,false,false,true,true];
_1.2 = _4.2;
_7 = [17357098419039612353_u64,10102063409631626839_u64,10512768267302495588_u64,13814672242942658271_u64,9786978272476579231_u64,4639838132310286723_u64];
Goto(bb2)
}
bb9 = {
_4.1 = _1.1;
_4 = (_3, _1.1, _2, _1.3);
_9 = !323548010341144390898548295420123131230_u128;
_14.0 = [_4.2,_1.2,_4.2,_4.2,_2,_4.2,_4.2];
_3 = -_1.0;
_6 = &(*_8);
RET = [false,true,true,true,true];
_2 = _4.2 | _1.2;
RET = [true,false,false,true,false];
_11 = true as isize;
_6 = &_4.0;
_14.2 = [27650_i16,(-32398_i16),(-27427_i16),(-17834_i16),(-20125_i16),(-22877_i16),(-29549_i16),25482_i16];
_14.0 = [_2,_2,_4.2,_4.2,_4.2,_1.2,_4.2];
_4.2 = -_2;
_2 = _1.2;
_2 = !_4.2;
_4.3 = false as u32;
Goto(bb4)
}
bb10 = {
_4.2 = _1.3 as i32;
_9 = !208248399561309224083606896169008703229_u128;
_7 = [17111063325018773248_u64,2295143829113981171_u64,8547351150183238306_u64,16336231244799453639_u64,4546435293806074365_u64,2033952841406363643_u64];
RET = [true,true,true,false,false];
_1.1 = _4.1;
_12 = (-9571_i16) as u8;
_1.3 = _4.3;
_1.2 = _2 + _4.2;
_1.3 = _4.3;
_2 = !_4.2;
RET = [false,true,false,false,false];
RET = [true,true,true,false,false];
(*_8) = _4.0;
(*_8) = _4.0;
_8 = core::ptr::addr_of!(_4.0);
(*_8) = !_1.0;
_4.2 = -_2;
_1.2 = 1111793238343130736_usize as i32;
_3 = _11 as i128;
_7 = [2182229128963622240_u64,14134366389708828383_u64,9485048846805532345_u64,8529512255287127451_u64,16448078429356250773_u64,14841305853496453564_u64];
_4.1 = _1.1;
(*_8) = 4856796864443923370_u64 as i128;
_13 = [2443216735548368889_usize,2_usize];
(*_8) = false as i128;
_12 = 39619_u16 as u8;
_1 = _4;
Goto(bb3)
}
bb11 = {
_12 = !160_u8;
_1.1 = _4.1;
_27 = false;
_28 = -_11;
place!(Field::<(i128, char, i32, u32)>(Variant(_24.0.fld3, 0), 0)).3 = _16 as u32;
place!(Field::<i8>(Variant(_18, 0), 0)) = 17_i8;
_24.0.fld5 = [(-4359751115089622203_i64),6890439526831654931_i64,(-5112444351451150025_i64),5622866118475658406_i64];
_24.0.fld1.3 = Field::<(i128, char, i32, u32)>(Variant(_24.0.fld3, 0), 0).3;
_15 = &(*_15);
Goto(bb12)
}
bb12 = {
_31 = _17;
_24.2.0.0 = _4.0;
_24.0.fld0 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_24.0.fld3, 0), 2)));
_29 = [_27,_27,_27,_27,_27,_27];
place!(Field::<i16>(Variant(_24.0.fld3, 0), 2)) = _24.0.fld4 | _24.0.fld4;
_1.2 = Field::<(i128, char, i32, u32)>(Variant(_24.0.fld3, 0), 0).2 ^ Field::<(i128, char, i32, u32)>(Variant(_24.0.fld3, 0), 0).2;
_14.0 = [_4.2,_2,_2,_1.2,_1.2,_4.2,_4.2];
_16 = _12 as f32;
place!(Field::<(i128, char, i32, u32)>(Variant(_24.0.fld3, 0), 0)).2 = _1.2;
_1.0 = _12 as i128;
_34 = &_23;
place!(Field::<[bool; 5]>(Variant(_18, 0), 1)) = [_27,_27,_27,_27,_27];
place!(Field::<(i128, char, i32, u32)>(Variant(_24.0.fld3, 0), 0)).0 = !Field::<i128>(Variant(_24.0.fld3, 0), 1);
_24.0.fld0 = core::ptr::addr_of!(_24.0.fld4);
_1.0 = -_24.2.0.0;
_1 = (_24.0.fld1.0, Field::<(i128, char, i32, u32)>(Variant(_24.0.fld3, 0), 0).1, Field::<(i128, char, i32, u32)>(Variant(_24.0.fld3, 0), 0).2, _24.0.fld1.3);
Goto(bb13)
}
bb13 = {
_24.2.0.1 = core::ptr::addr_of!(_11);
_26 = _31;
place!(Field::<(i128, char, i32, u32)>(Variant(_24.0.fld3, 0), 0)).0 = Field::<i128>(Variant(_24.0.fld3, 0), 1);
_6 = &_24.0.fld1.0;
SetDiscriminant(_18, 0);
place!(Field::<i128>(Variant(_24.0.fld3, 0), 1)) = !_1.0;
(*_8) = _25 as i128;
RET = [_27,_27,_27,_27,_27];
_24.0.fld1.2 = Field::<(i128, char, i32, u32)>(Variant(_24.0.fld3, 0), 0).2 - _2;
_12 = 53_u8;
_4 = ((*_8), _1.1, _24.0.fld1.2, Field::<(i128, char, i32, u32)>(Variant(_24.0.fld3, 0), 0).3);
_24.2.0.1 = core::ptr::addr_of!(_28);
_31 = _26;
_20 = _11 * _28;
_24.0.fld1.1 = _4.1;
_17 = -_26;
_13 = [_24.0.fld2,_24.0.fld2];
_1.3 = _4.3 + Field::<(i128, char, i32, u32)>(Variant(_24.0.fld3, 0), 0).3;
_19 = [9_i8,(-34_i8),68_i8,30_i8,37_i8,100_i8,(-106_i8)];
_20 = _11;
_17 = -_31;
_24.0.fld4 = Field::<i16>(Variant(_24.0.fld3, 0), 2);
place!(Field::<(i128, char, i32, u32)>(Variant(_24.0.fld3, 0), 0)).1 = _1.1;
_11 = -_20;
match (*_34) {
0 => bb6,
1 => bb8,
2 => bb3,
3 => bb5,
17855130183576673901 => bb15,
_ => bb14
}
}
bb14 = {
_8 = core::ptr::addr_of!(_3);
_17 = _16 as f64;
_18 = Adt73::Variant0 { fld0: 33_i8,fld1: RET };
_9 = 106973555243543591620074685896033264922_u128 >> _1.2;
_9 = 20201146720288230682933456939277203150_u128;
_2 = _4.2 + _1.2;
_14.2 = [418_i16,(-5996_i16),12083_i16,(-16266_i16),(-31282_i16),(-22696_i16),28345_i16,29839_i16];
_1.2 = -_2;
_3 = _11 as i128;
_9 = !27440006751076355337711689912773026794_u128;
_24.0.fld6 = [_2,_1.2,_2];
_25 = 1890_u16 as isize;
_24.0.fld5 = [640489858147013248_i64,(-2200219150398509924_i64),(-8191624557213300927_i64),(-5519555462667180541_i64)];
_24.2.1 = core::ptr::addr_of_mut!(_9);
RET = Field::<[bool; 5]>(Variant(_18, 0), 1);
_5 = [_4.2,_1.2,_2];
(*_8) = _4.0;
_11 = _25;
_24.0.fld0 = core::ptr::addr_of!(_24.0.fld4);
_4 = (_1.0, _1.1, _1.2, _1.3);
_22 = [11788192927355552747_u64,7767300190323843770_u64,5339425978944331087_u64,3851819195917520418_u64,14007949036517958313_u64,10807568051484635891_u64];
Goto(bb6)
}
bb15 = {
_4.3 = _1.3 & Field::<(i128, char, i32, u32)>(Variant(_24.0.fld3, 0), 0).3;
_24.0.fld3 = Adt27::Variant0 { fld0: _1,fld1: (*_6),fld2: _24.0.fld4,fld3: 8394_u16 };
_24.0.fld4 = Field::<i16>(Variant(_24.0.fld3, 0), 2) | Field::<i16>(Variant(_24.0.fld3, 0), 2);
_15 = &_24.2.2;
place!(Field::<[bool; 5]>(Variant(_18, 0), 1)) = RET;
_33 = [_24.0.fld2,_24.0.fld2,_24.0.fld2,_24.0.fld2];
_39 = _1.1;
place!(Field::<[bool; 5]>(Variant(_18, 0), 1)) = [_27,_27,_27,_27,_27];
_43 = _4.2 as i128;
place!(Field::<u16>(Variant(_24.0.fld3, 0), 3)) = _24.0.fld1.1 as u16;
_32 = Move(_24.0.fld3);
_1.1 = _24.0.fld1.1;
_24.0.fld3 = Move(_32);
_24.2.0.1 = core::ptr::addr_of!(_28);
_36 = 4791197729388866402_i64 as f64;
_4.1 = Field::<(i128, char, i32, u32)>(Variant(_24.0.fld3, 0), 0).1;
Goto(bb16)
}
bb16 = {
Call(_46 = dump_var(10_usize, 28_usize, Move(_28), 25_usize, Move(_25), 27_usize, Move(_27), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(10_usize, 23_usize, Move(_23), 5_usize, Move(_5), 39_usize, Move(_39), 29_usize, Move(_29)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_46 = dump_var(10_usize, 12_usize, Move(_12), 33_usize, Move(_33), 47_usize, _47, 47_usize, _47), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: char,mut _2: i128) -> i32 {
mir! {
type RET = i32;
let _3: char;
let _4: bool;
let _5: &'static ([i64; 6], &'static f32, *const (u128, [i64; 6]));
let _6: char;
let _7: [i32; 7];
let _8: isize;
let _9: u32;
let _10: *const i16;
let _11: f64;
let _12: [bool; 6];
let _13: &'static (i32, i128, i64, bool);
let _14: char;
let _15: Adt46;
let _16: &'static char;
let _17: &'static Adt44;
let _18: [bool; 5];
let _19: &'static *mut &'static u64;
let _20: isize;
let _21: isize;
let _22: [i32; 7];
let _23: f64;
let _24: ((i128, *const isize), *mut u128, Adt44);
let _25: ();
let _26: ();
{
Goto(bb1)
}
bb1 = {
_3 = _1;
RET = !(-1213647424_i32);
_4 = _3 <= _3;
_2 = (-105959532801808459285426818190700973972_i128) + 39173177955594119952119621877045791208_i128;
_4 = _3 > _3;
_2 = -34237264234626439105956545258781357455_i128;
_3 = _1;
_3 = _1;
_3 = _1;
_3 = _1;
_1 = _3;
RET = (-1324266735_i32);
RET = 33743396_i32 & 261737793_i32;
_1 = _3;
RET = 52175_u16 as i32;
_7 = [RET,RET,RET,RET,RET,RET,RET];
_3 = _1;
_7 = [RET,RET,RET,RET,RET,RET,RET];
Goto(bb2)
}
bb2 = {
_7 = [RET,RET,RET,RET,RET,RET,RET];
_1 = _3;
_8 = -9223372036854775807_isize;
_7 = [RET,RET,RET,RET,RET,RET,RET];
_7 = [RET,RET,RET,RET,RET,RET,RET];
_3 = _1;
RET = 3602228247_u32 as i32;
_4 = !false;
_6 = _1;
_6 = _1;
_7 = [RET,RET,RET,RET,RET,RET,RET];
_1 = _3;
Call(_10 = fn12(_6, _8, _3, _6, _6, _3, _1, _2, _2, _1, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3 = _1;
_6 = _3;
_11 = 326500513016477637307492231423114202693_u128 as f64;
RET = !545109103_i32;
_6 = _1;
_8 = !(-16_isize);
_8 = (-2_isize);
_9 = 3977691160_u32;
_14 = _6;
RET = (-5265959043859901322_i64) as i32;
RET = (-1453091864_i32);
_8 = -9223372036854775807_isize;
_11 = 8867890634508032065_usize as f64;
RET = (-857593153_i32);
_3 = _14;
_11 = 2886_u16 as f64;
Goto(bb4)
}
bb4 = {
_16 = &_14;
_14 = _6;
_11 = 5004111489937369264_i64 as f64;
RET = -(-711614575_i32);
_14 = _6;
_4 = true ^ false;
_16 = &_1;
_15 = Adt46::Variant0 { fld0: 22644_u16 };
_9 = 1299172580_u32;
_1 = _3;
_18 = [_4,_4,_4,_4,_4];
RET = (-748674085_i32) >> _9;
_4 = false | true;
_12 = [_4,_4,_4,_4,_4,_4];
_7 = [RET,RET,RET,RET,RET,RET,RET];
_12 = [_4,_4,_4,_4,_4,_4];
RET = 2112392869_i32;
_14 = _1;
_15 = Adt46::Variant0 { fld0: 29652_u16 };
_21 = !_8;
_6 = _1;
_16 = &_6;
_14 = (*_16);
match RET {
0 => bb1,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
2112392869 => bb10,
_ => bb9
}
}
bb5 = {
_3 = _1;
_6 = _3;
_11 = 326500513016477637307492231423114202693_u128 as f64;
RET = !545109103_i32;
_6 = _1;
_8 = !(-16_isize);
_8 = (-2_isize);
_9 = 3977691160_u32;
_14 = _6;
RET = (-5265959043859901322_i64) as i32;
RET = (-1453091864_i32);
_8 = -9223372036854775807_isize;
_11 = 8867890634508032065_usize as f64;
RET = (-857593153_i32);
_3 = _14;
_11 = 2886_u16 as f64;
Goto(bb4)
}
bb6 = {
_7 = [RET,RET,RET,RET,RET,RET,RET];
_1 = _3;
_8 = -9223372036854775807_isize;
_7 = [RET,RET,RET,RET,RET,RET,RET];
_7 = [RET,RET,RET,RET,RET,RET,RET];
_3 = _1;
RET = 3602228247_u32 as i32;
_4 = !false;
_6 = _1;
_6 = _1;
_7 = [RET,RET,RET,RET,RET,RET,RET];
_1 = _3;
Call(_10 = fn12(_6, _8, _3, _6, _6, _3, _1, _2, _2, _1, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_3 = _1;
RET = !(-1213647424_i32);
_4 = _3 <= _3;
_2 = (-105959532801808459285426818190700973972_i128) + 39173177955594119952119621877045791208_i128;
_4 = _3 > _3;
_2 = -34237264234626439105956545258781357455_i128;
_3 = _1;
_3 = _1;
_3 = _1;
_3 = _1;
_1 = _3;
RET = (-1324266735_i32);
RET = 33743396_i32 & 261737793_i32;
_1 = _3;
RET = 52175_u16 as i32;
_7 = [RET,RET,RET,RET,RET,RET,RET];
_3 = _1;
_7 = [RET,RET,RET,RET,RET,RET,RET];
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_1 = (*_16);
place!(Field::<u16>(Variant(_15, 0), 0)) = 23661_u16 ^ 37800_u16;
_14 = (*_16);
SetDiscriminant(_15, 0);
_21 = _8;
_1 = (*_16);
_16 = &_6;
RET = _4 as i32;
_9 = 3089933512_u32;
_3 = _14;
match _9 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
3089933512 => bb17,
_ => bb16
}
}
bb11 = {
_7 = [RET,RET,RET,RET,RET,RET,RET];
_1 = _3;
_8 = -9223372036854775807_isize;
_7 = [RET,RET,RET,RET,RET,RET,RET];
_7 = [RET,RET,RET,RET,RET,RET,RET];
_3 = _1;
RET = 3602228247_u32 as i32;
_4 = !false;
_6 = _1;
_6 = _1;
_7 = [RET,RET,RET,RET,RET,RET,RET];
_1 = _3;
Call(_10 = fn12(_6, _8, _3, _6, _6, _3, _1, _2, _2, _1, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb12 = {
_3 = _1;
_6 = _3;
_11 = 326500513016477637307492231423114202693_u128 as f64;
RET = !545109103_i32;
_6 = _1;
_8 = !(-16_isize);
_8 = (-2_isize);
_9 = 3977691160_u32;
_14 = _6;
RET = (-5265959043859901322_i64) as i32;
RET = (-1453091864_i32);
_8 = -9223372036854775807_isize;
_11 = 8867890634508032065_usize as f64;
RET = (-857593153_i32);
_3 = _14;
_11 = 2886_u16 as f64;
Goto(bb4)
}
bb13 = {
_3 = _1;
RET = !(-1213647424_i32);
_4 = _3 <= _3;
_2 = (-105959532801808459285426818190700973972_i128) + 39173177955594119952119621877045791208_i128;
_4 = _3 > _3;
_2 = -34237264234626439105956545258781357455_i128;
_3 = _1;
_3 = _1;
_3 = _1;
_3 = _1;
_1 = _3;
RET = (-1324266735_i32);
RET = 33743396_i32 & 261737793_i32;
_1 = _3;
RET = 52175_u16 as i32;
_7 = [RET,RET,RET,RET,RET,RET,RET];
_3 = _1;
_7 = [RET,RET,RET,RET,RET,RET,RET];
Goto(bb2)
}
bb14 = {
_7 = [RET,RET,RET,RET,RET,RET,RET];
_1 = _3;
_8 = -9223372036854775807_isize;
_7 = [RET,RET,RET,RET,RET,RET,RET];
_7 = [RET,RET,RET,RET,RET,RET,RET];
_3 = _1;
RET = 3602228247_u32 as i32;
_4 = !false;
_6 = _1;
_6 = _1;
_7 = [RET,RET,RET,RET,RET,RET,RET];
_1 = _3;
Call(_10 = fn12(_6, _8, _3, _6, _6, _3, _1, _2, _2, _1, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
_3 = _1;
_6 = _3;
_11 = 326500513016477637307492231423114202693_u128 as f64;
RET = !545109103_i32;
_6 = _1;
_8 = !(-16_isize);
_8 = (-2_isize);
_9 = 3977691160_u32;
_14 = _6;
RET = (-5265959043859901322_i64) as i32;
RET = (-1453091864_i32);
_8 = -9223372036854775807_isize;
_11 = 8867890634508032065_usize as f64;
RET = (-857593153_i32);
_3 = _14;
_11 = 2886_u16 as f64;
Goto(bb4)
}
bb16 = {
_3 = _1;
RET = !(-1213647424_i32);
_4 = _3 <= _3;
_2 = (-105959532801808459285426818190700973972_i128) + 39173177955594119952119621877045791208_i128;
_4 = _3 > _3;
_2 = -34237264234626439105956545258781357455_i128;
_3 = _1;
_3 = _1;
_3 = _1;
_3 = _1;
_1 = _3;
RET = (-1324266735_i32);
RET = 33743396_i32 & 261737793_i32;
_1 = _3;
RET = 52175_u16 as i32;
_7 = [RET,RET,RET,RET,RET,RET,RET];
_3 = _1;
_7 = [RET,RET,RET,RET,RET,RET,RET];
Goto(bb2)
}
bb17 = {
_8 = _21 ^ _21;
_23 = _11 * _11;
_16 = &(*_16);
RET = (-1299984003_i32) & 865036578_i32;
_1 = _14;
_1 = (*_16);
_8 = -_21;
_6 = _1;
_16 = &_3;
_7 = [RET,RET,RET,RET,RET,RET,RET];
_20 = -_21;
_24.0.0 = 33365033568422736644719539912370618373_u128 as i128;
_14 = (*_16);
Goto(bb18)
}
bb18 = {
Call(_25 = dump_var(11_usize, 20_usize, Move(_20), 1_usize, Move(_1), 21_usize, Move(_21), 9_usize, Move(_9)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_25 = dump_var(11_usize, 14_usize, Move(_14), 6_usize, Move(_6), 26_usize, _26, 26_usize, _26), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: char,mut _2: isize,mut _3: char,mut _4: char,mut _5: char,mut _6: char,mut _7: char,mut _8: i128,mut _9: i128,mut _10: char,mut _11: char) -> *const i16 {
mir! {
type RET = *const i16;
let _12: u16;
let _13: u32;
let _14: f64;
let _15: [i64; 4];
let _16: Adt44;
let _17: *const (u128, [i64; 6]);
let _18: isize;
let _19: *const usize;
let _20: (u128, [i64; 6]);
let _21: [u64; 6];
let _22: Adt50;
let _23: i16;
let _24: i128;
let _25: u64;
let _26: isize;
let _27: char;
let _28: &'static Adt44;
let _29: f64;
let _30: &'static *mut &'static u64;
let _31: *mut [i64; 6];
let _32: i16;
let _33: &'static (i32, i128, i64, bool);
let _34: bool;
let _35: ();
let _36: ();
{
_1 = _11;
_6 = _7;
_7 = _1;
_8 = _9 - _9;
_5 = _4;
_2 = (-9223372036854775808_isize) | 9223372036854775807_isize;
_9 = _8 - _8;
_11 = _6;
_10 = _4;
_9 = -_8;
_9 = _8 * _8;
_5 = _3;
_10 = _7;
_7 = _10;
_12 = 10407_u16;
match _12 {
0 => bb1,
10407 => bb3,
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
_3 = _10;
_2 = (-9223372036854775808_isize) ^ 13_isize;
_7 = _4;
_12 = 58671_u16;
match _12 {
0 => bb4,
58671 => bb6,
_ => bb5
}
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_8 = _9;
_8 = _9;
_1 = _11;
Goto(bb7)
}
bb7 = {
_1 = _4;
_4 = _6;
_4 = _7;
_5 = _6;
_6 = _7;
_15 = [4813257289417228931_i64,1666935732721766564_i64,5164719412409598503_i64,(-7224192886829766378_i64)];
_10 = _6;
_13 = 138879678_u32 + 3989882350_u32;
_5 = _3;
_3 = _7;
Call(_4 = fn13(_5, _11, _8, _13, _7, _1, _6, _15, _2, _15, _5), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_11 = _1;
Goto(bb9)
}
bb9 = {
_7 = _4;
_14 = _2 as f64;
_7 = _10;
_7 = _4;
_13 = _2 as u32;
_18 = (-2013011762_i32) as isize;
_1 = _7;
_15 = [4309273378275859944_i64,(-1016867795659111097_i64),(-3978980573659011030_i64),7118971858679193337_i64];
_1 = _10;
_15 = [7056867710969296894_i64,(-5649190890395673160_i64),3379150322399455159_i64,(-3288242548316048220_i64)];
_9 = _8;
_6 = _7;
_14 = _9 as f64;
_2 = _9 as isize;
_5 = _4;
Call(_6 = fn15(_7, _9, _11, _14), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_15 = [6876760106695791889_i64,(-1677277225344516964_i64),(-2070516789114007055_i64),(-1578696131598307069_i64)];
_9 = !_8;
_15 = [2754119133771003488_i64,5224699060653348305_i64,(-5606344065876251148_i64),568742600791208946_i64];
_6 = _1;
_5 = _1;
_10 = _6;
_20.0 = !237030551667124629525695032438011720585_u128;
_17 = core::ptr::addr_of!(_20);
_12 = 14125_u16;
_18 = (-1877140028_i32) as isize;
_20.0 = (-1568601645661129938_i64) as u128;
_17 = core::ptr::addr_of!((*_17));
_14 = 943_i16 as f64;
Call(_8 = core::intrinsics::transmute(_9), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_9 = _2 as i128;
Goto(bb12)
}
bb12 = {
_18 = _2 & _2;
_20.1 = [5308576517495946027_i64,(-3895190608027800760_i64),(-7048065501027295450_i64),(-4233349562404071802_i64),(-864367295158541919_i64),(-1530388176937668784_i64)];
RET = core::ptr::addr_of!(_23);
_22.fld0 = [5044291643135396722_i64,2377874279691750357_i64,7263628738129917292_i64,1844779957232774528_i64,(-6789397937825733527_i64),(-3712547934527179811_i64)];
_18 = _12 as isize;
(*_17).0 = 167113247783394228048953720766156267901_u128 ^ 307607972411555542628057812969404971090_u128;
_1 = _7;
_23 = (-4701_i16);
_3 = _7;
_6 = _11;
RET = core::ptr::addr_of!((*RET));
_13 = 2868135306_u32;
Goto(bb13)
}
bb13 = {
(*_17) = (168274877286891597003812571007974077097_u128, _22.fld0);
_6 = _11;
_8 = !_9;
_22 = Adt50 { fld0: (*_17).1 };
_20.1 = [2457189963098359097_i64,3618938613041922083_i64,(-3216464959428362890_i64),(-600143061024886872_i64),4978630947212068884_i64,8156764789022010027_i64];
(*_17).0 = 256442451330023685386035185568599212176_u128 | 261332202313338594278527050813192952252_u128;
_21 = [1658385953380578046_u64,16921539828668462014_u64,17242090577342303123_u64,10421996986697392580_u64,5827106338001485104_u64,15120839324793287000_u64];
_7 = _3;
_11 = _10;
(*_17) = (252094879723906842722692910683743659110_u128, _22.fld0);
_24 = _8;
_22.fld0 = [5671830830898696470_i64,6455042754496932215_i64,2139328572146817622_i64,(-247094491224792508_i64),98513091784146649_i64,586633680450011880_i64];
Goto(bb14)
}
bb14 = {
_22 = Adt50 { fld0: (*_17).1 };
_29 = _14 - _14;
_21 = [3611648655818867960_u64,377357412831248018_u64,5191611239348471260_u64,3956688946990246084_u64,6429455323489801656_u64,10292253983857242554_u64];
(*_17) = (312339818733436550815495108642975017730_u128, _22.fld0);
_9 = _8;
RET = core::ptr::addr_of!((*RET));
_26 = _2 & _2;
_12 = 20009_u16;
_1 = _3;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(12_usize, 6_usize, Move(_6), 18_usize, Move(_18), 12_usize, Move(_12), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(12_usize, 7_usize, Move(_7), 9_usize, Move(_9), 10_usize, Move(_10), 11_usize, Move(_11)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(12_usize, 23_usize, Move(_23), 1_usize, Move(_1), 36_usize, _36, 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: char,mut _2: char,mut _3: i128,mut _4: u32,mut _5: char,mut _6: char,mut _7: char,mut _8: [i64; 4],mut _9: isize,mut _10: [i64; 4],mut _11: char) -> char {
mir! {
type RET = char;
let _12: *const isize;
let _13: ([i64; 6], &'static f32, *const (u128, [i64; 6]));
let _14: (i128, *const isize);
let _15: i64;
let _16: f32;
let _17: [u16; 8];
let _18: &'static f32;
let _19: *const (i128, *const isize);
let _20: i64;
let _21: (i128, *const isize);
let _22: ((i128, *const isize), *mut u128, Adt44);
let _23: *const usize;
let _24: [i64; 6];
let _25: *const &'static u64;
let _26: char;
let _27: &'static u64;
let _28: *mut Adt53;
let _29: &'static [bool; 5];
let _30: *mut [bool; 5];
let _31: Adt37;
let _32: *const isize;
let _33: isize;
let _34: [isize; 2];
let _35: [u64; 6];
let _36: bool;
let _37: f32;
let _38: *const i16;
let _39: char;
let _40: ();
let _41: ();
{
RET = _7;
RET = _5;
_12 = core::ptr::addr_of!(_9);
_5 = _7;
_1 = _6;
_5 = RET;
(*_12) = -(-9223372036854775808_isize);
_8 = [7917423818351686782_i64,1125249368878105956_i64,2323031174995201109_i64,(-2697187534229893720_i64)];
_12 = core::ptr::addr_of!((*_12));
Goto(bb1)
}
bb1 = {
RET = _2;
_3 = !(-36897656763118244392342327899404201129_i128);
_5 = _2;
_12 = core::ptr::addr_of!((*_12));
_2 = _5;
(*_12) = (-50_isize) | 9223372036854775807_isize;
_4 = 982395407_u32 >> _3;
_9 = 9223372036854775807_isize;
_13.0 = [(-1319443840723866948_i64),4439143095173872961_i64,3508539076692486597_i64,337761015800456371_i64,7726340329862452422_i64,7120093201611509710_i64];
_12 = core::ptr::addr_of!((*_12));
_10 = [172705256647471172_i64,1731796703212675674_i64,(-7214062929404978965_i64),8718748244141684384_i64];
_5 = _11;
(*_12) = 192184409201292860141464147867681330275_u128 as isize;
_12 = core::ptr::addr_of!((*_12));
_12 = core::ptr::addr_of!(_9);
_14.0 = _3 >> _4;
RET = _7;
_12 = core::ptr::addr_of!((*_12));
_13.0 = [3691027426114852358_i64,(-5967859627906795264_i64),(-734587406760690590_i64),4718290362772400902_i64,(-7199728055340764395_i64),3443824855545006771_i64];
_3 = _14.0 << (*_12);
_4 = 4291134211_u32 ^ 2569002677_u32;
_14 = (_3, Move(_12));
_12 = core::ptr::addr_of!(_9);
_7 = _2;
_14.1 = Move(_12);
_15 = 3907731205690785882_i64 | (-1417112179435241825_i64);
_12 = core::ptr::addr_of!(_9);
Call(_14.1 = core::intrinsics::arith_offset(_12, 106_isize), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = _11;
_9 = _3 as isize;
_13.0 = [_15,_15,_15,_15,_15,_15];
Call(_7 = fn14(_13.0, _15, Move(_14)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = _7;
RET = _7;
(*_12) = (-9223372036854775808_isize);
_2 = _6;
(*_12) = 113_isize;
Goto(bb4)
}
bb4 = {
_1 = _2;
_14.0 = false as i128;
_7 = _5;
RET = _2;
_20 = (-15880_i16) as i64;
_21.0 = !_3;
_14 = (_3, Move(_12));
_22.0.1 = Move(_14.1);
_21.1 = Move(_22.0.1);
_1 = _5;
_14.1 = core::ptr::addr_of!(_9);
_2 = _6;
_22.2 = Adt44::Variant1 { fld0: _13.0,fld1: _6 };
_21.0 = 59652_u16 as i128;
_11 = _1;
SetDiscriminant(_22.2, 1);
_8 = [_15,_15,_15,_20];
place!(Field::<[i64; 6]>(Variant(_22.2, 1), 0)) = [_15,_15,_15,_20,_20,_15];
_20 = _15;
match _9 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
113 => bb9,
_ => bb8
}
}
bb5 = {
_1 = _7;
RET = _7;
(*_12) = (-9223372036854775808_isize);
_2 = _6;
(*_12) = 113_isize;
Goto(bb4)
}
bb6 = {
_7 = _11;
_9 = _3 as isize;
_13.0 = [_15,_15,_15,_15,_15,_15];
Call(_7 = fn14(_13.0, _15, Move(_14)), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
RET = _2;
_3 = !(-36897656763118244392342327899404201129_i128);
_5 = _2;
_12 = core::ptr::addr_of!((*_12));
_2 = _5;
(*_12) = (-50_isize) | 9223372036854775807_isize;
_4 = 982395407_u32 >> _3;
_9 = 9223372036854775807_isize;
_13.0 = [(-1319443840723866948_i64),4439143095173872961_i64,3508539076692486597_i64,337761015800456371_i64,7726340329862452422_i64,7120093201611509710_i64];
_12 = core::ptr::addr_of!((*_12));
_10 = [172705256647471172_i64,1731796703212675674_i64,(-7214062929404978965_i64),8718748244141684384_i64];
_5 = _11;
(*_12) = 192184409201292860141464147867681330275_u128 as isize;
_12 = core::ptr::addr_of!((*_12));
_12 = core::ptr::addr_of!(_9);
_14.0 = _3 >> _4;
RET = _7;
_12 = core::ptr::addr_of!((*_12));
_13.0 = [3691027426114852358_i64,(-5967859627906795264_i64),(-734587406760690590_i64),4718290362772400902_i64,(-7199728055340764395_i64),3443824855545006771_i64];
_3 = _14.0 << (*_12);
_4 = 4291134211_u32 ^ 2569002677_u32;
_14 = (_3, Move(_12));
_12 = core::ptr::addr_of!(_9);
_7 = _2;
_14.1 = Move(_12);
_15 = 3907731205690785882_i64 | (-1417112179435241825_i64);
_12 = core::ptr::addr_of!(_9);
Call(_14.1 = core::intrinsics::arith_offset(_12, 106_isize), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
_13.1 = &_16;
_17 = [44567_u16,53443_u16,24821_u16,61385_u16,20377_u16,11589_u16,5727_u16,51665_u16];
_18 = &_16;
_14.0 = _3 * _3;
_13.1 = &(*_18);
_22.0.1 = Move(_14.1);
RET = _2;
_10 = [_15,_20,_20,_20];
_14.1 = core::ptr::addr_of!(_9);
_13.1 = &(*_18);
_2 = _1;
Goto(bb10)
}
bb10 = {
_16 = 43646_u16 as f32;
_12 = Move(_21.1);
_19 = core::ptr::addr_of!(_21);
(*_19).0 = _14.0 + _14.0;
match _9 {
113 => bb12,
_ => bb11
}
}
bb11 = {
_13.1 = &_16;
_17 = [44567_u16,53443_u16,24821_u16,61385_u16,20377_u16,11589_u16,5727_u16,51665_u16];
_18 = &_16;
_14.0 = _3 * _3;
_13.1 = &(*_18);
_22.0.1 = Move(_14.1);
RET = _2;
_10 = [_15,_20,_20,_20];
_14.1 = core::ptr::addr_of!(_9);
_13.1 = &(*_18);
_2 = _1;
Goto(bb10)
}
bb12 = {
_24 = [_15,_20,_15,_15,_15,_20];
_14.0 = _9 as i128;
_25 = core::ptr::addr_of!(_27);
_14.0 = _21.0;
_12 = Move(_14.1);
Goto(bb13)
}
bb13 = {
_21 = (_14.0, Move(_12));
(*_19).0 = _14.0 ^ _14.0;
(*_19).0 = _14.0 >> _9;
_17 = [14334_u16,11744_u16,57463_u16,40562_u16,1895_u16,20812_u16,51429_u16,6006_u16];
_16 = _9 as f32;
_11 = _7;
_14 = Move(_21);
_26 = _6;
_14 = (_3, Move(_22.0.1));
_12 = Move(_14.1);
_22.0.1 = Move(_12);
_19 = core::ptr::addr_of!(_21);
_13.1 = &_16;
_21 = (_14.0, Move(_22.0.1));
_36 = !false;
_18 = &_16;
_15 = (-27409_i16) as i64;
match _9 {
0 => bb1,
113 => bb15,
_ => bb14
}
}
bb14 = {
_7 = _11;
_9 = _3 as isize;
_13.0 = [_15,_15,_15,_15,_15,_15];
Call(_7 = fn14(_13.0, _15, Move(_14)), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
place!(Field::<char>(Variant(_22.2, 1), 1)) = _11;
(*_19).1 = core::ptr::addr_of!(_33);
_14.1 = core::ptr::addr_of!(_9);
_3 = 18_i8 as i128;
_12 = core::ptr::addr_of!(_9);
_34 = [(*_12),_9];
_13.0 = _24;
_21.1 = Move(_14.1);
_32 = Move((*_19).1);
Goto(bb16)
}
bb16 = {
Call(_40 = dump_var(13_usize, 17_usize, Move(_17), 36_usize, Move(_36), 15_usize, Move(_15), 26_usize, Move(_26)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(13_usize, 3_usize, Move(_3), 6_usize, Move(_6), 4_usize, Move(_4), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_40 = dump_var(13_usize, 34_usize, Move(_34), 41_usize, _41, 41_usize, _41, 41_usize, _41), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: [i64; 6],mut _2: i64,mut _3: (i128, *const isize)) -> char {
mir! {
type RET = char;
let _4: isize;
let _5: i128;
let _6: [char; 8];
let _7: *const usize;
let _8: (i32, i128, i64, bool);
let _9: *mut Adt53;
let _10: (*const i16, (u8, u64, (u8, i64, u64)), [i32; 3]);
let _11: char;
let _12: isize;
let _13: f64;
let _14: u64;
let _15: *mut u128;
let _16: &'static i128;
let _17: &'static ([i64; 6], &'static f32, *const (u128, [i64; 6]));
let _18: i8;
let _19: isize;
let _20: i128;
let _21: *const isize;
let _22: i8;
let _23: [i16; 8];
let _24: isize;
let _25: *mut [bool; 5];
let _26: ();
let _27: ();
{
RET = '\u{c9cb}';
_4 = -(-9223372036854775808_isize);
_3.0 = 14975548278482529337117805589442700297_u128 as i128;
_5 = _4 as i128;
_3.0 = 11995261629758698335_usize as i128;
_2 = false as i64;
_2 = (-5213207125232537957_i64) * 3236311710457862724_i64;
_8.2 = _2 & _2;
_5 = !_3.0;
_5 = -_3.0;
_8 = ((-2028634147_i32), _5, _2, true);
_1 = [_2,_8.2,_2,_8.2,_2,_2];
RET = '\u{987e2}';
RET = '\u{be270}';
_8.1 = _5 >> _2;
_4 = (-9223372036854775808_isize) | (-9223372036854775808_isize);
_2 = 155437656_u32 as i64;
_8 = (1608397114_i32, _3.0, _2, false);
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_8.1 = _5;
Goto(bb1)
}
bb1 = {
_2 = 9800778022194677329_u64 as i64;
_8.1 = _8.3 as i128;
match _8.0 {
0 => bb2,
1 => bb3,
1608397114 => bb5,
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
RET = '\u{75fc2}';
RET = '\u{c85d7}';
_2 = _8.1 as i64;
_8.1 = _5 | _5;
_8.1 = _3.0 | _5;
_10.1.2.0 = !6_u8;
RET = '\u{1c9c0}';
RET = '\u{8470a}';
_10.1.0 = !_10.1.2.0;
_3.0 = !_8.1;
_2 = -_8.2;
_10.1.2 = (_10.1.0, _2, 3092852319873171457_u64);
_10.1.1 = !_10.1.2.2;
RET = '\u{432d3}';
_10.1.0 = _10.1.2.0 * _10.1.2.0;
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
Goto(bb6)
}
bb6 = {
_10.2 = [_8.0,_8.0,_8.0];
RET = '\u{9b171}';
_5 = _3.0;
_10.1.0 = _10.1.2.0 ^ _10.1.2.0;
_10.1.0 = _10.1.2.0 >> _8.1;
RET = '\u{108bc9}';
_8.2 = _10.1.2.1;
_8.2 = (-31629_i16) as i64;
_13 = _10.1.1 as f64;
_8.2 = _8.0 as i64;
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_8.3 = _8.0 != _8.0;
_10.2 = [_8.0,_8.0,_8.0];
_14 = _10.1.1 << _10.1.0;
_4 = !52_isize;
_10.1.0 = _10.1.2.0;
_8.1 = _5;
_8.0 = -(-586401514_i32);
_13 = 6_usize as f64;
_16 = &_5;
_2 = _10.1.2.1 >> _4;
_13 = 75_i8 as f64;
_10.1.2 = (_10.1.0, _8.2, _10.1.1);
_16 = &_3.0;
Goto(bb7)
}
bb7 = {
_8.0 = !(-610503020_i32);
Goto(bb8)
}
bb8 = {
_18 = -(-82_i8);
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
_13 = _4 as f64;
_8.0 = 509844561_i32;
RET = '\u{98b2}';
_8.3 = !false;
_19 = !_4;
_8.1 = _10.1.2.0 as i128;
_20 = _8.1 << _8.2;
_14 = _10.1.2.2 >> _5;
_10.1.1 = _14;
_11 = RET;
_21 = core::ptr::addr_of!(_12);
(*_21) = _19;
_19 = -_4;
_10.1.2 = (_10.1.0, _2, _10.1.1);
_21 = Move(_3.1);
_4 = (*_16) as isize;
_10.2 = [_8.0,_8.0,_8.0];
_19 = _8.3 as isize;
Goto(bb9)
}
bb9 = {
_10.1.2.0 = RET as u8;
_18 = 3071482788_u32 as i8;
Goto(bb10)
}
bb10 = {
RET = _11;
_8.2 = -_2;
_3 = (_8.1, Move(_21));
Call(_10.1.2.0 = core::intrinsics::bswap(_10.1.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_16 = &_5;
match _8.0 {
0 => bb5,
1 => bb12,
2 => bb13,
3 => bb14,
509844561 => bb16,
_ => bb15
}
}
bb12 = {
RET = '\u{75fc2}';
RET = '\u{c85d7}';
_2 = _8.1 as i64;
_8.1 = _5 | _5;
_8.1 = _3.0 | _5;
_10.1.2.0 = !6_u8;
RET = '\u{1c9c0}';
RET = '\u{8470a}';
_10.1.0 = !_10.1.2.0;
_3.0 = !_8.1;
_2 = -_8.2;
_10.1.2 = (_10.1.0, _2, 3092852319873171457_u64);
_10.1.1 = !_10.1.2.2;
RET = '\u{432d3}';
_10.1.0 = _10.1.2.0 * _10.1.2.0;
_6 = [RET,RET,RET,RET,RET,RET,RET,RET];
Goto(bb6)
}
bb13 = {
Return()
}
bb14 = {
_2 = 9800778022194677329_u64 as i64;
_8.1 = _8.3 as i128;
match _8.0 {
0 => bb2,
1 => bb3,
1608397114 => bb5,
_ => bb4
}
}
bb15 = {
Return()
}
bb16 = {
RET = _11;
_19 = _12;
_10.1.2.2 = !_14;
_3.0 = _8.1;
_24 = _13 as isize;
RET = _11;
_10.2 = [_8.0,_8.0,_8.0];
RET = _11;
_1 = [_10.1.2.1,_10.1.2.1,_8.2,_10.1.2.1,_8.2,_10.1.2.1];
_24 = _20 as isize;
Goto(bb17)
}
bb17 = {
Call(_26 = dump_var(14_usize, 24_usize, Move(_24), 5_usize, Move(_5), 19_usize, Move(_19), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_26 = dump_var(14_usize, 12_usize, Move(_12), 14_usize, Move(_14), 27_usize, _27, 27_usize, _27), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: char,mut _2: i128,mut _3: char,mut _4: f64) -> char {
mir! {
type RET = char;
let _5: [u64; 6];
let _6: [u16; 5];
let _7: [i64; 4];
let _8: (u8, u64, (u8, i64, u64));
let _9: [i8; 7];
let _10: char;
let _11: *const i16;
let _12: bool;
let _13: [bool; 6];
let _14: [bool; 5];
let _15: *const *const i128;
let _16: u16;
let _17: Adt50;
let _18: f32;
let _19: (*const i16, (u8, u64, (u8, i64, u64)), [i32; 3]);
let _20: [i8; 7];
let _21: Adt50;
let _22: *const i16;
let _23: *mut &'static u64;
let _24: i16;
let _25: u128;
let _26: *mut [i64; 6];
let _27: isize;
let _28: u64;
let _29: [i64; 6];
let _30: isize;
let _31: (i32, i128, i64, bool);
let _32: i8;
let _33: *const *const i128;
let _34: bool;
let _35: u32;
let _36: &'static [bool; 5];
let _37: *const isize;
let _38: Adt46;
let _39: f64;
let _40: u8;
let _41: ((i128, *const isize), *mut u128, Adt44);
let _42: isize;
let _43: &'static ([i64; 6], &'static f32, *const (u128, [i64; 6]));
let _44: u16;
let _45: [u16; 5];
let _46: f64;
let _47: &'static isize;
let _48: ();
let _49: ();
{
RET = _1;
RET = _3;
_3 = _1;
_1 = _3;
RET = _3;
_3 = _1;
_4 = 6015438038987223723_i64 as f64;
RET = _3;
_3 = RET;
_4 = 965778746_i32 as f64;
_4 = _2 as f64;
_1 = RET;
_3 = _1;
_2 = false as i128;
RET = _3;
_4 = 15598114189051809113_usize as f64;
RET = _1;
_1 = _3;
_5 = [17328272060211506092_u64,9302443640510399000_u64,399222933649175658_u64,3062598348333864923_u64,2601861930321671903_u64,1558571046298340450_u64];
_6 = [46334_u16,20676_u16,36930_u16,21585_u16,36844_u16];
_6 = [22883_u16,33898_u16,21250_u16,61127_u16,1113_u16];
_3 = _1;
_1 = _3;
_8.1 = 12082868462120569200_u64 * 10395034962368749373_u64;
_8.2.2 = !_8.1;
Call(_8 = fn16(_6, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8.0 = (-20_i8) as u8;
_7 = [_8.2.1,_8.2.1,_8.2.1,_8.2.1];
_5 = [_8.2.2,_8.2.2,_8.2.2,_8.2.2,_8.2.2,_8.1];
_1 = _3;
_6 = [43725_u16,43431_u16,30149_u16,61501_u16,11492_u16];
_6 = [597_u16,8044_u16,44004_u16,13917_u16,22581_u16];
_9 = [(-98_i8),(-42_i8),23_i8,(-116_i8),(-118_i8),33_i8,(-17_i8)];
_7 = [_8.2.1,_8.2.1,_8.2.1,_8.2.1];
_4 = 182825800_u32 as f64;
_10 = RET;
_6 = [62755_u16,52323_u16,35388_u16,14047_u16,16915_u16];
_12 = !false;
_3 = RET;
_3 = _1;
_7 = [_8.2.1,_8.2.1,_8.2.1,_8.2.1];
_8.0 = _10 as u8;
match _8.2.1 {
0 => bb2,
6053723525520133123 => bb4,
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
_8.2.0 = _4 as u8;
_8.2.0 = _2 as u8;
_8.0 = _8.2.0 ^ _8.2.0;
_8.2 = (_8.0, (-7986012090588868276_i64), _8.1);
_8.2.1 = 15310205961844942921908652314810878414_u128 as i64;
_8.2.2 = 4096872438945823432_usize as u64;
_5 = [_8.1,_8.2.2,_8.2.2,_8.1,_8.1,_8.1];
_8.0 = _8.2.0;
Goto(bb5)
}
bb5 = {
_5 = [_8.1,_8.1,_8.2.2,_8.2.2,_8.2.2,_8.2.2];
_13 = [_12,_12,_12,_12,_12,_12];
_2 = !(-52701414989462511470646409247631794342_i128);
_7 = [_8.2.1,_8.2.1,_8.2.1,_8.2.1];
_13 = [_12,_12,_12,_12,_12,_12];
_14 = [_12,_12,_12,_12,_12];
_7 = [_8.2.1,_8.2.1,_8.2.1,_8.2.1];
_8.2.1 = (-1897852990384577535_i64);
RET = _1;
_9 = [(-98_i8),112_i8,33_i8,(-115_i8),72_i8,60_i8,(-68_i8)];
_8.2.2 = _8.1;
_17.fld0 = [_8.2.1,_8.2.1,_8.2.1,_8.2.1,_8.2.1,_8.2.1];
_1 = _10;
_16 = _8.2.1 as u16;
_17.fld0 = [_8.2.1,_8.2.1,_8.2.1,_8.2.1,_8.2.1,_8.2.1];
_13 = [_12,_12,_12,_12,_12,_12];
_13 = [_12,_12,_12,_12,_12,_12];
_8.2.1 = 792398835223677476_i64 >> _8.1;
_8.2 = (_8.0, 441481741317357824_i64, _8.1);
_3 = _1;
_8.2 = (_8.0, 478269562580176590_i64, _8.1);
_18 = _2 as f32;
_13 = [_12,_12,_12,_12,_12,_12];
_19.1.2.2 = _8.2.2;
_19.1.1 = 2664351390_u32 as u64;
_14 = [_12,_12,_12,_12,_12];
Goto(bb6)
}
bb6 = {
_8.2.2 = _8.1 * _19.1.2.2;
_19.1.2 = (_8.2.0, _8.2.1, _19.1.1);
_8.1 = !_8.2.2;
_5 = [_8.2.2,_19.1.1,_8.1,_19.1.2.2,_19.1.1,_8.2.2];
_18 = 3_usize as f32;
_4 = _2 as f64;
_7 = [_8.2.1,_8.2.1,_19.1.2.1,_8.2.1];
_19.1.0 = _8.0;
_8.2.0 = _8.0;
_19.1.2.0 = _19.1.0;
_19.1.2.0 = _19.1.0;
_1 = _10;
_18 = _19.1.2.1 as f32;
_19.2 = [2039144432_i32,(-186840186_i32),(-887345027_i32)];
_8.0 = _19.1.0 - _19.1.2.0;
_19.1.2 = (_8.0, _8.2.1, _8.1);
_19.2 = [1444519435_i32,218125387_i32,1352061692_i32];
_19.1.2.2 = _1 as u64;
_13 = [_12,_12,_12,_12,_12,_12];
_17.fld0 = [_8.2.1,_8.2.1,_19.1.2.1,_19.1.2.1,_8.2.1,_19.1.2.1];
_12 = !false;
_3 = _10;
_19.2 = [(-1728804262_i32),(-1085743986_i32),295273652_i32];
_19.2 = [(-1620663135_i32),732121917_i32,1249996241_i32];
_24 = RET as i16;
_17.fld0 = [_8.2.1,_19.1.2.1,_19.1.2.1,_19.1.2.1,_19.1.2.1,_8.2.1];
Goto(bb7)
}
bb7 = {
_21.fld0 = [_19.1.2.1,_8.2.1,_8.2.1,_19.1.2.1,_8.2.1,_19.1.2.1];
_14 = [_12,_12,_12,_12,_12];
_14 = [_12,_12,_12,_12,_12];
_24 = !806_i16;
_2 = -68976591197209492797501655884407703117_i128;
_19.1 = (_8.2.0, _8.1, _8.2);
_22 = core::ptr::addr_of!(_24);
_21 = Adt50 { fld0: _17.fld0 };
_19.1.2.0 = !_8.0;
RET = _3;
_17.fld0 = _21.fld0;
_24 = 21516_i16 << _8.0;
_28 = _19.1.2.2;
RET = _3;
_28 = 3591077322_u32 as u64;
_6 = [_16,_16,_16,_16,_16];
_19.1.2.0 = _19.1.0;
_24 = !(-20861_i16);
_14 = [_12,_12,_12,_12,_12];
_25 = !130092514107896010721550154928990640398_u128;
match _8.2.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
478269562580176590 => bb8,
_ => bb5
}
}
bb8 = {
_8.0 = _19.1.2.0 + _19.1.0;
_14 = [_12,_12,_12,_12,_12];
_8 = (_19.1.0, _19.1.1, _19.1.2);
_17 = Adt50 { fld0: _21.fld0 };
_19.1 = (_8.0, _8.2.2, _8.2);
_17.fld0 = _21.fld0;
_19.1.2.0 = !_8.2.0;
_16 = 48426_u16 * 42379_u16;
_8.1 = !_19.1.2.2;
_19.1.0 = _8.0;
_19.2 = [1843503309_i32,135520649_i32,1064834856_i32];
_6 = [_16,_16,_16,_16,_16];
_19.0 = core::ptr::addr_of!((*_22));
(*_22) = 1810922026_u32 as i16;
_27 = !(-9223372036854775808_isize);
_11 = Move(_22);
_8.2.1 = _19.1.2.1 & _19.1.2.1;
_18 = _25 as f32;
_19.1.2.2 = _8.1;
_12 = false;
RET = _3;
_19.1.2.1 = _2 as i64;
_31.2 = -_8.2.1;
Goto(bb9)
}
bb9 = {
_16 = _25 as u16;
_3 = _1;
_19.1.0 = _8.0;
_3 = _1;
_27 = (-9223372036854775808_isize);
_19.1.1 = _24 as u64;
_8.2.2 = !_8.1;
_5 = [_8.2.2,_8.1,_8.1,_8.2.2,_19.1.2.2,_8.2.2];
_8.2.0 = _2 as u8;
_11 = Move(_19.0);
_31.3 = _8.2.0 >= _19.1.2.0;
_21 = _17;
_13 = [_12,_31.3,_12,_31.3,_31.3,_31.3];
_19.1.2.1 = _8.2.0 as i64;
_8.2.2 = _31.2 as u64;
_24 = 14476_i16 & 8791_i16;
_21.fld0 = _17.fld0;
_31 = ((-682391226_i32), _2, _8.2.1, _12);
_29 = [_8.2.1,_31.2,_31.2,_31.2,_8.2.1,_31.2];
_5 = [_8.2.2,_28,_8.1,_8.2.2,_8.1,_8.2.2];
_19.1.2.0 = _19.1.0 - _19.1.0;
_26 = core::ptr::addr_of_mut!(_17.fld0);
_18 = _27 as f32;
_11 = core::ptr::addr_of!(_24);
Goto(bb10)
}
bb10 = {
_30 = _27;
Goto(bb11)
}
bb11 = {
_35 = 334808133_u32;
_37 = core::ptr::addr_of!(_30);
_25 = 212899970768369924391407980136148201623_u128;
_8.2 = (_19.1.2.0, _31.2, _8.1);
_34 = !_31.3;
_8.1 = !_19.1.2.2;
_32 = !(-16_i8);
_2 = _31.1 & _31.1;
_38 = Adt46::Variant0 { fld0: _16 };
_19.1.2 = (_8.0, _8.2.1, _8.1);
_22 = core::ptr::addr_of!((*_11));
_19.1 = (_8.2.0, _28, _8.2);
_13 = [_34,_31.3,_34,_12,_34,_12];
_34 = !_12;
_37 = core::ptr::addr_of!(_30);
_18 = _25 as f32;
_21 = _17;
Goto(bb12)
}
bb12 = {
_16 = Field::<u16>(Variant(_38, 0), 0) >> _19.1.2.1;
_19.1 = _8;
_8.2.1 = !_31.2;
_19.1.2.0 = !_8.0;
_38 = Adt46::Variant0 { fld0: _16 };
SetDiscriminant(_38, 0);
_8 = _19.1;
_24 = (-354_i16);
_41.0.1 = Move(_37);
_19.0 = core::ptr::addr_of!((*_22));
_36 = &_14;
Goto(bb13)
}
bb13 = {
_14 = [_12,_34,_34,_31.3,_12];
_7 = [_31.2,_19.1.2.1,_19.1.2.1,_31.2];
(*_11) = 6907_i16 ^ 18708_i16;
_19.1.2.1 = _8.2.1 & _31.2;
_22 = core::ptr::addr_of!((*_22));
_8.2 = (_19.1.2.0, _19.1.2.1, _19.1.1);
match _31.0 {
0 => bb11,
1 => bb5,
2 => bb10,
340282366920938463463374607431085820230 => bb15,
_ => bb14
}
}
bb14 = {
_35 = 334808133_u32;
_37 = core::ptr::addr_of!(_30);
_25 = 212899970768369924391407980136148201623_u128;
_8.2 = (_19.1.2.0, _31.2, _8.1);
_34 = !_31.3;
_8.1 = !_19.1.2.2;
_32 = !(-16_i8);
_2 = _31.1 & _31.1;
_38 = Adt46::Variant0 { fld0: _16 };
_19.1.2 = (_8.0, _8.2.1, _8.1);
_22 = core::ptr::addr_of!((*_11));
_19.1 = (_8.2.0, _28, _8.2);
_13 = [_34,_31.3,_34,_12,_34,_12];
_34 = !_12;
_37 = core::ptr::addr_of!(_30);
_18 = _25 as f32;
_21 = _17;
Goto(bb12)
}
bb15 = {
_19.1.2.2 = _8.2.2;
_31.2 = _35 as i64;
(*_26) = [_19.1.2.1,_19.1.2.1,_8.2.1,_8.2.1,_19.1.2.1,_19.1.2.1];
_31.1 = -_2;
_8.2.2 = _30 as u64;
_31.2 = _8.2.1;
_19.1.1 = _28;
Goto(bb16)
}
bb16 = {
Call(_48 = dump_var(15_usize, 31_usize, Move(_31), 27_usize, Move(_27), 9_usize, Move(_9), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_48 = dump_var(15_usize, 7_usize, Move(_7), 28_usize, Move(_28), 8_usize, Move(_8), 35_usize, Move(_35)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_48 = dump_var(15_usize, 29_usize, Move(_29), 2_usize, Move(_2), 12_usize, Move(_12), 49_usize, _49), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: [u16; 5],mut _2: [u16; 5]) -> (u8, u64, (u8, i64, u64)) {
mir! {
type RET = (u8, u64, (u8, i64, u64));
let _3: u16;
let _4: isize;
let _5: *const isize;
let _6: (*const i16, (u8, u64, (u8, i64, u64)), [i32; 3]);
let _7: Adt73;
let _8: isize;
let _9: *const Adt53;
let _10: [i8; 7];
let _11: [char; 8];
let _12: f64;
let _13: f64;
let _14: ();
let _15: ();
{
RET.1 = 2161448134943424063_u64 - 1898380256691803900_u64;
RET.2.2 = RET.1;
RET.0 = 94_u8;
_3 = !19405_u16;
RET.2.0 = !RET.0;
_2 = [_3,_3,_3,_3,_3];
_1 = [_3,_3,_3,_3,_3];
RET.2.0 = !RET.0;
match RET.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
94 => bb9,
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
RET.2.1 = 6053723525520133123_i64;
RET.2.0 = (-1511980704_i32) as u8;
RET.2.2 = RET.1;
_3 = !29776_u16;
_6.1 = (RET.2.0, RET.1, RET.2);
RET.2.2 = _6.1.2.2;
_1 = [_3,_3,_3,_3,_3];
_3 = !27779_u16;
RET.2.0 = !_6.1.0;
_1 = [_3,_3,_3,_3,_3];
RET.0 = RET.2.0;
RET.0 = '\u{105430}' as u8;
RET.2.0 = true as u8;
_5 = core::ptr::addr_of!(_4);
RET.2.2 = false as u64;
RET.2.0 = (-98_i8) as u8;
_5 = core::ptr::addr_of!(_4);
_6.1 = RET;
RET.2 = (_6.1.0, _6.1.2.1, RET.1);
_6.1 = RET;
RET.2 = (_6.1.2.0, _6.1.2.1, _6.1.2.2);
RET = (_6.1.2.0, _6.1.2.2, _6.1.2);
RET.2.1 = -_6.1.2.1;
RET.2.1 = _6.1.2.1;
RET = (_6.1.2.0, _6.1.1, _6.1.2);
_4 = 9223372036854775807_isize & 9223372036854775807_isize;
RET.0 = !RET.2.0;
match _6.1.2.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb10,
6053723525520133123 => bb12,
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
RET.2 = (RET.0, _6.1.2.1, _6.1.1);
_6.1.2 = (RET.0, RET.2.1, RET.1);
RET.2.1 = !_6.1.2.1;
RET.2.0 = _6.1.0;
_6.1.2.2 = RET.2.0 as u64;
RET.2 = (_6.1.0, _6.1.2.1, _6.1.1);
_6.1.2 = (_6.1.0, RET.2.1, _6.1.1);
(*_5) = 114_isize | (-45_isize);
RET = _6.1;
match RET.2.1 {
0 => bb5,
1 => bb10,
6053723525520133123 => bb13,
_ => bb8
}
}
bb13 = {
_2 = [_3,_3,_3,_3,_3];
_2 = _1;
_1 = _2;
RET.2.2 = !_6.1.2.2;
RET.2.0 = !_6.1.2.0;
(*_5) = 9223372036854775807_isize;
_4 = (-9223372036854775808_isize);
RET.2.2 = RET.1 << _6.1.2.2;
RET.2.0 = _6.1.2.0 | RET.0;
Goto(bb14)
}
bb14 = {
RET.2.1 = _6.1.2.1;
_6.1.2.1 = RET.2.1 & RET.2.1;
_6.1.0 = !_6.1.2.0;
RET.0 = 51483301927155276460069933302506107536_i128 as u8;
_13 = 117284020727650073523807945942984923571_u128 as f64;
_2 = _1;
(*_5) = (-121_isize) & (-9223372036854775808_isize);
Goto(bb15)
}
bb15 = {
Call(_14 = dump_var(16_usize, 4_usize, Move(_4), 1_usize, Move(_1), 15_usize, _15, 15_usize, _15), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: &'static i128,mut _2: char,mut _3: [i16; 8],mut _4: *const i128,mut _5: [i32; 7],mut _6: char,mut _7: u32) -> char {
mir! {
type RET = char;
let _8: isize;
let _9: *const Adt53;
let _10: char;
let _11: *mut u128;
let _12: [i8; 7];
let _13: bool;
let _14: i128;
let _15: [u16; 5];
let _16: isize;
let _17: i16;
let _18: f32;
let _19: f32;
let _20: [usize; 2];
let _21: Adt50;
let _22: bool;
let _23: char;
let _24: f64;
let _25: u16;
let _26: isize;
let _27: isize;
let _28: char;
let _29: [u64; 6];
let _30: ([i64; 6], &'static f32, *const (u128, [i64; 6]));
let _31: bool;
let _32: i32;
let _33: bool;
let _34: isize;
let _35: *mut &'static u64;
let _36: &'static (i32, i128, i64, bool);
let _37: i64;
let _38: (u8, u64, (u8, i64, u64));
let _39: *const (u128, [i64; 6]);
let _40: [isize; 2];
let _41: u64;
let _42: ();
let _43: ();
{
RET = _2;
RET = _6;
RET = _2;
_8 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
RET = _6;
RET = _6;
RET = _2;
_10 = _6;
RET = _6;
_6 = RET;
_8 = _10 as isize;
_10 = _6;
_8 = 87_isize | 105_isize;
_10 = RET;
RET = _10;
_7 = 3480726086_u32;
RET = _6;
_6 = _10;
_6 = RET;
_12 = [44_i8,111_i8,57_i8,(-34_i8),2_i8,(-46_i8),101_i8];
_12 = [19_i8,(-50_i8),(-18_i8),83_i8,3_i8,92_i8,50_i8];
_6 = _2;
_7 = 2197_i16 as u32;
_10 = _2;
_10 = _2;
Goto(bb1)
}
bb1 = {
RET = _10;
_5 = [(-1474936339_i32),(-2125671365_i32),978831925_i32,(-2055580151_i32),1123476869_i32,(-1123883320_i32),601904598_i32];
_3 = [20250_i16,(-8774_i16),(-5352_i16),25493_i16,4680_i16,(-11458_i16),27280_i16,(-2725_i16)];
_7 = 1565253474_u32;
_7 = 145_u8 as u32;
_3 = [(-27467_i16),(-31145_i16),(-16684_i16),(-12487_i16),23620_i16,19627_i16,(-6783_i16),(-29823_i16)];
RET = _6;
_3 = [4072_i16,10318_i16,4568_i16,(-23076_i16),(-18560_i16),26220_i16,5831_i16,32554_i16];
_3 = [(-23742_i16),(-24104_i16),6881_i16,17872_i16,(-29328_i16),26825_i16,11303_i16,(-1874_i16)];
_7 = 3366766865_u32;
_2 = RET;
_6 = RET;
match _7 {
3366766865 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_12 = [108_i8,(-34_i8),48_i8,55_i8,78_i8,(-20_i8),(-102_i8)];
_2 = _6;
_14 = 19738289186630328987539778734509152839_i128;
_8 = !(-9223372036854775808_isize);
_1 = &_14;
Call(_3 = fn18(Move(_1)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_18 = 56385_u16 as f32;
_8 = 104189182183269670252326730751773986682_u128 as isize;
_13 = false;
match _14 {
19738289186630328987539778734509152839 => bb5,
_ => bb1
}
}
bb5 = {
_13 = true & true;
_18 = 7451010461700398277_u64 as f32;
_13 = !true;
_2 = _10;
_25 = 32571_u16;
_23 = _6;
_2 = RET;
_4 = core::ptr::addr_of!(_14);
_1 = &(*_4);
RET = _23;
_16 = !_8;
_20 = [4_usize,7_usize];
_27 = _16;
_27 = !_8;
_27 = 7939462741956049498_i64 as isize;
_29 = [12464114187584213856_u64,16510953856749739680_u64,1810930727293839036_u64,11471370063767512463_u64,11394151473081903937_u64,7964904309030834401_u64];
match (*_4) {
0 => bb6,
19738289186630328987539778734509152839 => bb8,
_ => bb7
}
}
bb6 = {
RET = _10;
_5 = [(-1474936339_i32),(-2125671365_i32),978831925_i32,(-2055580151_i32),1123476869_i32,(-1123883320_i32),601904598_i32];
_3 = [20250_i16,(-8774_i16),(-5352_i16),25493_i16,4680_i16,(-11458_i16),27280_i16,(-2725_i16)];
_7 = 1565253474_u32;
_7 = 145_u8 as u32;
_3 = [(-27467_i16),(-31145_i16),(-16684_i16),(-12487_i16),23620_i16,19627_i16,(-6783_i16),(-29823_i16)];
RET = _6;
_3 = [4072_i16,10318_i16,4568_i16,(-23076_i16),(-18560_i16),26220_i16,5831_i16,32554_i16];
_3 = [(-23742_i16),(-24104_i16),6881_i16,17872_i16,(-29328_i16),26825_i16,11303_i16,(-1874_i16)];
_7 = 3366766865_u32;
_2 = RET;
_6 = RET;
match _7 {
3366766865 => bb3,
_ => bb2
}
}
bb7 = {
_12 = [108_i8,(-34_i8),48_i8,55_i8,78_i8,(-20_i8),(-102_i8)];
_2 = _6;
_14 = 19738289186630328987539778734509152839_i128;
_8 = !(-9223372036854775808_isize);
_1 = &_14;
Call(_3 = fn18(Move(_1)), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
RET = _23;
_30.0 = [(-4630508940740905674_i64),(-8616209794816458609_i64),7699322059199853425_i64,5417856545450605135_i64,(-1405271720171190964_i64),(-4565665942918954941_i64)];
_3 = [(-26039_i16),9293_i16,1115_i16,(-11522_i16),(-30225_i16),(-27467_i16),5587_i16,14769_i16];
_10 = RET;
_21.fld0 = _30.0;
_7 = (-6764874654971523720_i64) as u32;
_15 = [_25,_25,_25,_25,_25];
_31 = _13;
_8 = _27;
_12 = [108_i8,86_i8,61_i8,(-56_i8),(-115_i8),48_i8,(-93_i8)];
_19 = _18 + _18;
_5 = [(-985002579_i32),7820233_i32,1530882316_i32,927874163_i32,(-975787732_i32),711194342_i32,1118895941_i32];
match (*_4) {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb4,
4 => bb5,
5 => bb9,
6 => bb10,
19738289186630328987539778734509152839 => bb12,
_ => bb11
}
}
bb9 = {
_12 = [108_i8,(-34_i8),48_i8,55_i8,78_i8,(-20_i8),(-102_i8)];
_2 = _6;
_14 = 19738289186630328987539778734509152839_i128;
_8 = !(-9223372036854775808_isize);
_1 = &_14;
Call(_3 = fn18(Move(_1)), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
_18 = 56385_u16 as f32;
_8 = 104189182183269670252326730751773986682_u128 as isize;
_13 = false;
match _14 {
19738289186630328987539778734509152839 => bb5,
_ => bb1
}
}
bb11 = {
_13 = true & true;
_18 = 7451010461700398277_u64 as f32;
_13 = !true;
_2 = _10;
_25 = 32571_u16;
_23 = _6;
_2 = RET;
_4 = core::ptr::addr_of!(_14);
_1 = &(*_4);
RET = _23;
_16 = !_8;
_20 = [4_usize,7_usize];
_27 = _16;
_27 = !_8;
_27 = 7939462741956049498_i64 as isize;
_29 = [12464114187584213856_u64,16510953856749739680_u64,1810930727293839036_u64,11471370063767512463_u64,11394151473081903937_u64,7964904309030834401_u64];
match (*_4) {
0 => bb6,
19738289186630328987539778734509152839 => bb8,
_ => bb7
}
}
bb12 = {
_25 = 58047_u16 - 14901_u16;
_33 = _31;
_28 = _6;
_15 = [_25,_25,_25,_25,_25];
_5 = [(-1980784759_i32),667249253_i32,(-467937882_i32),(-345934970_i32),(-1772861358_i32),(-1693185059_i32),(-987981161_i32)];
_25 = (-1458546073_i32) as u16;
_27 = _16;
_19 = _16 as f32;
_4 = core::ptr::addr_of!((*_1));
_2 = _23;
_34 = !_27;
_21 = Adt50 { fld0: _30.0 };
_24 = _25 as f64;
match (*_4) {
0 => bb13,
1 => bb14,
19738289186630328987539778734509152839 => bb16,
_ => bb15
}
}
bb13 = {
_18 = 56385_u16 as f32;
_8 = 104189182183269670252326730751773986682_u128 as isize;
_13 = false;
match _14 {
19738289186630328987539778734509152839 => bb5,
_ => bb1
}
}
bb14 = {
RET = _23;
_30.0 = [(-4630508940740905674_i64),(-8616209794816458609_i64),7699322059199853425_i64,5417856545450605135_i64,(-1405271720171190964_i64),(-4565665942918954941_i64)];
_3 = [(-26039_i16),9293_i16,1115_i16,(-11522_i16),(-30225_i16),(-27467_i16),5587_i16,14769_i16];
_10 = RET;
_21.fld0 = _30.0;
_7 = (-6764874654971523720_i64) as u32;
_15 = [_25,_25,_25,_25,_25];
_31 = _13;
_8 = _27;
_12 = [108_i8,86_i8,61_i8,(-56_i8),(-115_i8),48_i8,(-93_i8)];
_19 = _18 + _18;
_5 = [(-985002579_i32),7820233_i32,1530882316_i32,927874163_i32,(-975787732_i32),711194342_i32,1118895941_i32];
match (*_4) {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb4,
4 => bb5,
5 => bb9,
6 => bb10,
19738289186630328987539778734509152839 => bb12,
_ => bb11
}
}
bb15 = {
Return()
}
bb16 = {
_30.0 = [2289204928232245914_i64,4866484663121737832_i64,(-2046901141834677187_i64),5134235232453068845_i64,(-2229189853825903055_i64),9113498438973025914_i64];
_6 = RET;
_29 = [12679688743385687502_u64,6785700885305442147_u64,6775004007313841792_u64,9344121041901036283_u64,7444559740810227656_u64,16224988143935996812_u64];
_38.2 = (143_u8, (-3599551374605275528_i64), 12825944551794878444_u64);
_34 = _8 | _16;
RET = _23;
RET = _10;
_26 = -_8;
_38.2 = (18_u8, (-3922019461698387264_i64), 10666549359964002286_u64);
_22 = _31;
_30.1 = &_19;
RET = _10;
_16 = _7 as isize;
Goto(bb17)
}
bb17 = {
Call(_42 = dump_var(17_usize, 27_usize, Move(_27), 8_usize, Move(_8), 31_usize, Move(_31), 34_usize, Move(_34)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(17_usize, 14_usize, Move(_14), 29_usize, Move(_29), 25_usize, Move(_25), 2_usize, Move(_2)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_42 = dump_var(17_usize, 10_usize, Move(_10), 33_usize, Move(_33), 12_usize, Move(_12), 43_usize, _43), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: &'static i128) -> [i16; 8] {
mir! {
type RET = [i16; 8];
let _2: [u64; 6];
let _3: *const Adt53;
let _4: f64;
let _5: (i128, char, i32, u32);
let _6: ((i128, *const isize), *mut u128, Adt44);
let _7: [isize; 2];
let _8: isize;
let _9: u32;
let _10: f32;
let _11: &'static &'static i128;
let _12: char;
let _13: [usize; 4];
let _14: i64;
let _15: i64;
let _16: *const (u128, [i64; 6]);
let _17: f64;
let _18: [char; 8];
let _19: [i32; 3];
let _20: char;
let _21: i16;
let _22: f64;
let _23: ();
let _24: ();
{
RET = [(-6511_i16),(-20032_i16),16262_i16,32464_i16,23015_i16,(-1618_i16),(-5614_i16),(-23303_i16)];
_2 = [1171386804331063347_u64,7351002645413568241_u64,6052610830584802769_u64,481638914388756924_u64,1359344765770260399_u64,8277830481192493612_u64];
RET = [(-19573_i16),(-15510_i16),(-18458_i16),(-7397_i16),(-19142_i16),(-23237_i16),29250_i16,(-13572_i16)];
_2 = [113758047068489314_u64,5563120087046902149_u64,2447192091110619147_u64,10192680125961407483_u64,15320302895715664394_u64,4052194650368945665_u64];
_2 = [11101426477552020843_u64,10530689231930590145_u64,9943274642699754118_u64,485648322213705226_u64,11752300814597672006_u64,14684038602855724424_u64];
RET = [(-23496_i16),23045_i16,(-28463_i16),6924_i16,24359_i16,(-4175_i16),9240_i16,3852_i16];
_2 = [3181140223533961364_u64,12876621132957430390_u64,12411988328284666607_u64,11224963270186779070_u64,2236946584413863040_u64,10860700935486059192_u64];
Goto(bb1)
}
bb1 = {
_2 = [17313647605823064127_u64,15449570347692398585_u64,11182642466579517054_u64,15825847732673100646_u64,11645557931118399381_u64,9594891808040693527_u64];
_4 = 33094_u16 as f64;
_1 = &_5.0;
RET = [(-25394_i16),5275_i16,11914_i16,13872_i16,(-1471_i16),(-4435_i16),(-24943_i16),29224_i16];
_5.0 = -(-87568193023260661550554124713749979381_i128);
_5.1 = '\u{b10c2}';
_5.2 = 71251852_i32;
_1 = &_5.0;
_5 = ((-163609010548617315772568733517175588647_i128), '\u{bff64}', (-1410526165_i32), 2156340967_u32);
_2 = [2939617927960221148_u64,4012040330541444785_u64,8772548314331873657_u64,4313168959606952087_u64,2545254782654285799_u64,5982521806360303965_u64];
_5.2 = 90_isize as i32;
_5.1 = '\u{102145}';
_5.3 = 230_u8 as u32;
_6.0.1 = core::ptr::addr_of!(_8);
Call(_5.3 = core::intrinsics::bswap(159586104_u32), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5.1 = '\u{104012}';
_6.0.1 = core::ptr::addr_of!(_8);
_5.2 = (-1029145063_i32) - 1590456078_i32;
_5.2 = (-121078334_i32) * (-143534561_i32);
_5.3 = 2850766511_u32 & 3585527440_u32;
_8 = 23887_i16 as isize;
_5.2 = (-381920221_i32) - 448035069_i32;
_9 = _5.3 ^ _5.3;
Call(_5.0 = core::intrinsics::bswap((-14560581013402818009910092290989637864_i128)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = &_6.0.0;
_6.0.0 = 97_u8 as i128;
_7 = [_8,_8];
_1 = &_6.0.0;
_7 = [_8,_8];
_9 = _5.3 - _5.3;
_5 = (_6.0.0, '\u{6f983}', (-1755117923_i32), _9);
RET = [2713_i16,4224_i16,(-8269_i16),(-25067_i16),6881_i16,2444_i16,2064_i16,5131_i16];
Goto(bb4)
}
bb4 = {
_7 = [_8,_8];
_5.1 = '\u{ffb7b}';
_1 = &(*_1);
_9 = _5.3 >> _5.2;
_7 = [_8,_8];
_5.2 = (-1933896137_i32);
_5.2 = 619888860_i32 + (-1377889623_i32);
RET = [17388_i16,(-5767_i16),(-8051_i16),3976_i16,(-3351_i16),20252_i16,(-16487_i16),(-27519_i16)];
_5.1 = '\u{3223c}';
RET = [10398_i16,(-24309_i16),22291_i16,(-23913_i16),25192_i16,2116_i16,1893_i16,32353_i16];
_11 = &_1;
_9 = !_5.3;
_5 = (_6.0.0, '\u{ba566}', 1085498662_i32, _9);
_5.2 = (-1099434952_i32);
_8 = -9223372036854775807_isize;
_4 = 15185409067830800923_u64 as f64;
RET = [(-15362_i16),(-30455_i16),(-3636_i16),16160_i16,(-18048_i16),(-24159_i16),23973_i16,13598_i16];
_5.3 = !_9;
_12 = _5.1;
_11 = &(*_11);
RET = [(-26540_i16),(-6061_i16),(-7096_i16),(-1027_i16),348_i16,2913_i16,3314_i16,(-25958_i16)];
_5.3 = _9;
_5.3 = !_9;
Goto(bb5)
}
bb5 = {
_11 = &(*_11);
_1 = &(*_1);
_2 = [14184739432669161488_u64,10189581815444425721_u64,7308611226372819340_u64,14584904999898518073_u64,10001421497895911624_u64,4451014887847761687_u64];
_14 = 8147881553977633808_i64;
match _14 {
0 => bb1,
1 => bb2,
2 => bb3,
8147881553977633808 => bb6,
_ => bb4
}
}
bb6 = {
_4 = 58265_u16 as f64;
_5 = (_6.0.0, _12, (-1262934542_i32), _9);
_7 = [_8,_8];
_8 = 9223372036854775807_isize | (-9223372036854775808_isize);
_11 = &_1;
match _5.2 {
0 => bb5,
1 => bb2,
2 => bb3,
340282366920938463463374607430505276914 => bb7,
_ => bb4
}
}
bb7 = {
_1 = &(*_1);
_11 = &_1;
_13 = [3_usize,18381997171237453015_usize,17885799331912007564_usize,4011592640157748288_usize];
RET = [(-9596_i16),23234_i16,6778_i16,(-31932_i16),(-9319_i16),7896_i16,19236_i16,(-369_i16)];
_10 = 98855074261730183880375864510518657346_u128 as f32;
_5.2 = 86735106310829867876094356958444164754_u128 as i32;
_10 = (-54_i8) as f32;
_11 = &(*_11);
_7 = [_8,_8];
Goto(bb8)
}
bb8 = {
_11 = &(*_11);
_5.1 = _12;
_2 = [836633613590313813_u64,482226333138208065_u64,13743045494349543452_u64,11600997622049658321_u64,9786575983811488876_u64,15788074126010039124_u64];
_10 = 71_u8 as f32;
_18 = [_5.1,_12,_5.1,_12,_5.1,_5.1,_12,_5.1];
_6.0.0 = _5.0;
_11 = &(*_11);
_5.0 = !_6.0.0;
_10 = _8 as f32;
_15 = -_14;
_5.3 = _9 >> _8;
_5.0 = _6.0.0 << _15;
_19 = [_5.2,_5.2,_5.2];
_2 = [2972587018854119585_u64,6850263161346274692_u64,6161946662856131822_u64,17303865316757681184_u64,13598694658538416761_u64,14778336770105621712_u64];
_6.0.0 = _5.0;
_5.0 = _6.0.0;
_5.2 = 1555915432_i32;
_8 = (-9223372036854775808_isize);
_18 = [_5.1,_12,_12,_5.1,_5.1,_12,_12,_5.1];
_7 = [_8,_8];
_5.2 = (-1985862143_i32) >> _5.3;
_1 = &_5.0;
match _14 {
0 => bb9,
1 => bb10,
8147881553977633808 => bb12,
_ => bb11
}
}
bb9 = {
_7 = [_8,_8];
_5.1 = '\u{ffb7b}';
_1 = &(*_1);
_9 = _5.3 >> _5.2;
_7 = [_8,_8];
_5.2 = (-1933896137_i32);
_5.2 = 619888860_i32 + (-1377889623_i32);
RET = [17388_i16,(-5767_i16),(-8051_i16),3976_i16,(-3351_i16),20252_i16,(-16487_i16),(-27519_i16)];
_5.1 = '\u{3223c}';
RET = [10398_i16,(-24309_i16),22291_i16,(-23913_i16),25192_i16,2116_i16,1893_i16,32353_i16];
_11 = &_1;
_9 = !_5.3;
_5 = (_6.0.0, '\u{ba566}', 1085498662_i32, _9);
_5.2 = (-1099434952_i32);
_8 = -9223372036854775807_isize;
_4 = 15185409067830800923_u64 as f64;
RET = [(-15362_i16),(-30455_i16),(-3636_i16),16160_i16,(-18048_i16),(-24159_i16),23973_i16,13598_i16];
_5.3 = !_9;
_12 = _5.1;
_11 = &(*_11);
RET = [(-26540_i16),(-6061_i16),(-7096_i16),(-1027_i16),348_i16,2913_i16,3314_i16,(-25958_i16)];
_5.3 = _9;
_5.3 = !_9;
Goto(bb5)
}
bb10 = {
_5.1 = '\u{104012}';
_6.0.1 = core::ptr::addr_of!(_8);
_5.2 = (-1029145063_i32) - 1590456078_i32;
_5.2 = (-121078334_i32) * (-143534561_i32);
_5.3 = 2850766511_u32 & 3585527440_u32;
_8 = 23887_i16 as isize;
_5.2 = (-381920221_i32) - 448035069_i32;
_9 = _5.3 ^ _5.3;
Call(_5.0 = core::intrinsics::bswap((-14560581013402818009910092290989637864_i128)), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_2 = [17313647605823064127_u64,15449570347692398585_u64,11182642466579517054_u64,15825847732673100646_u64,11645557931118399381_u64,9594891808040693527_u64];
_4 = 33094_u16 as f64;
_1 = &_5.0;
RET = [(-25394_i16),5275_i16,11914_i16,13872_i16,(-1471_i16),(-4435_i16),(-24943_i16),29224_i16];
_5.0 = -(-87568193023260661550554124713749979381_i128);
_5.1 = '\u{b10c2}';
_5.2 = 71251852_i32;
_1 = &_5.0;
_5 = ((-163609010548617315772568733517175588647_i128), '\u{bff64}', (-1410526165_i32), 2156340967_u32);
_2 = [2939617927960221148_u64,4012040330541444785_u64,8772548314331873657_u64,4313168959606952087_u64,2545254782654285799_u64,5982521806360303965_u64];
_5.2 = 90_isize as i32;
_5.1 = '\u{102145}';
_5.3 = 230_u8 as u32;
_6.0.1 = core::ptr::addr_of!(_8);
Call(_5.3 = core::intrinsics::bswap(159586104_u32), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_1 = &_5.0;
_13 = [13567367552289790032_usize,4_usize,11722484884347754382_usize,1505259682545615365_usize];
Goto(bb13)
}
bb13 = {
_10 = _4 as f32;
_12 = _5.1;
_1 = &(*_1);
_11 = &_1;
_5.2 = (-266944016_i32);
RET = [(-21488_i16),18286_i16,(-7974_i16),(-6966_i16),(-23820_i16),(-17778_i16),18419_i16,2617_i16];
_10 = _5.2 as f32;
_12 = _5.1;
_13 = [8362623131236200918_usize,6_usize,2_usize,6204532525832698629_usize];
RET = [20330_i16,10343_i16,(-3375_i16),(-8663_i16),3964_i16,18069_i16,(-6384_i16),2477_i16];
_8 = !(-9223372036854775808_isize);
_17 = _4 + _4;
match _14 {
0 => bb1,
1 => bb8,
2 => bb3,
8147881553977633808 => bb14,
_ => bb12
}
}
bb14 = {
_5.2 = _17 as i32;
_2 = [13200306499004237213_u64,5295314489499388167_u64,5200082363317085051_u64,10711891512725050050_u64,18192585458595216484_u64,15288563193899276476_u64];
_20 = _5.1;
_20 = _12;
RET = [(-575_i16),(-10517_i16),(-9165_i16),(-32570_i16),25899_i16,(-29521_i16),9373_i16,(-13045_i16)];
_5.1 = _20;
_5.3 = _9 << (*_1);
_13 = [7_usize,5_usize,10358449306398852535_usize,5468539239747895674_usize];
_1 = &_6.0.0;
_6.0.0 = -_5.0;
_1 = &_6.0.0;
_2 = [16927843858579476290_u64,9214905767838093739_u64,2280710736929729324_u64,14504951801135608688_u64,5372321210524486742_u64,10091575668258619003_u64];
_9 = !_5.3;
_5.1 = _12;
_17 = _4 - _4;
_20 = _5.1;
_15 = _14 - _14;
_7 = [_8,_8];
_11 = &_1;
_11 = &_1;
_5 = ((*_1), _12, 1949887698_i32, _9);
_4 = -_17;
_5 = ((*_1), _20, 1113199626_i32, _9);
_5.0 = _8 as i128;
_21 = _4 as i16;
_17 = _4 + _4;
_4 = _17;
_11 = &(*_11);
Goto(bb15)
}
bb15 = {
Call(_23 = dump_var(18_usize, 9_usize, Move(_9), 2_usize, Move(_2), 5_usize, Move(_5), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_23 = dump_var(18_usize, 19_usize, Move(_19), 13_usize, Move(_13), 24_usize, _24, 24_usize, _24), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: i128,mut _2: char,mut _3: [bool; 5]) -> u32 {
mir! {
type RET = u32;
let _4: i32;
let _5: u64;
let _6: Adt53;
let _7: [i64; 6];
let _8: *mut [i64; 6];
let _9: isize;
let _10: u8;
let _11: isize;
let _12: *const isize;
let _13: i32;
let _14: u16;
let _15: &'static f32;
let _16: &'static Adt44;
let _17: [u16; 5];
let _18: Adt34;
let _19: bool;
let _20: &'static u64;
let _21: char;
let _22: [i32; 3];
let _23: bool;
let _24: bool;
let _25: u8;
let _26: f64;
let _27: (u8, u64, (u8, i64, u64));
let _28: ();
let _29: ();
{
_4 = (-1274257926_i32);
_3 = [true,true,true,true,false];
_3 = [false,false,false,false,true];
RET = !4079823670_u32;
RET = !3014267568_u32;
_5 = 13471614569441339879_u64 | 15575691423936378939_u64;
_1 = (-31556949999218719475439075741837476736_i128) * 147349644785728176531856593161027047007_i128;
_3 = [false,true,true,false,false];
RET = _5 as u32;
_1 = 82399741325800245376613699377460846660_i128 & 148923747550031973265252913707183892232_i128;
_5 = 5584425862048228259_u64 * 11475353759003359423_u64;
RET = 503611102_u32 - 733532959_u32;
RET = 3906263030_u32;
_7 = [781731918611876686_i64,(-1803899232176485318_i64),2776338793599079329_i64,(-1376785280772176677_i64),5538959853361583324_i64,(-4941920750346194960_i64)];
_2 = '\u{d9825}';
_1 = 138526839970964364023801209245240386108_i128 + 26450081738184645609265000397790336550_i128;
_1 = (-4047860575249899828867641303086194862_i128) * (-93713958592515380687797556572988646851_i128);
_1 = (-101479824396198492859149164792520807914_i128) + 22342393830716461696800259418295303459_i128;
_1 = (-98382288980272258390136496468637417054_i128);
RET = 2393439057_u32;
Goto(bb1)
}
bb1 = {
_1 = 10997196226848698594576514677106824010_i128;
_7 = [(-3236127557744501735_i64),6015924327831740451_i64,(-5034184542253427531_i64),(-8696398340303976813_i64),(-7926777543983447737_i64),(-1502956181301593763_i64)];
_3 = [true,true,true,false,true];
Goto(bb2)
}
bb2 = {
_5 = 4420485110593002658_u64;
_3 = [true,true,false,false,true];
match _5 {
0 => bb1,
1 => bb3,
2 => bb4,
4420485110593002658 => bb6,
_ => bb5
}
}
bb3 = {
_1 = 10997196226848698594576514677106824010_i128;
_7 = [(-3236127557744501735_i64),6015924327831740451_i64,(-5034184542253427531_i64),(-8696398340303976813_i64),(-7926777543983447737_i64),(-1502956181301593763_i64)];
_3 = [true,true,true,false,true];
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_1 = -(-169409794725705684042228042272466811030_i128);
_4 = 1551705671_i32 * 815373421_i32;
RET = 3625645997_u32;
_8 = core::ptr::addr_of_mut!(_7);
RET = !429947447_u32;
_1 = !(-108629144615646955972340142955457110436_i128);
(*_8) = [(-837753884902061051_i64),9220431088424379964_i64,6444395810731908365_i64,(-2562414392503408789_i64),2840583356135467488_i64,3494880282902200220_i64];
_9 = 9223372036854775807_isize << _4;
_5 = 2497505566298792856_u64 | 12772047237595100675_u64;
_4 = -189063046_i32;
_4 = 5_usize as i32;
_2 = '\u{108554}';
_7 = [4100706842117775625_i64,8457024202267165189_i64,2634387513767229880_i64,9066025648714656234_i64,5385493918639123225_i64,(-4938303047129886936_i64)];
Call(_5 = core::intrinsics::bswap(17300916349050118521_u64), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_10 = !121_u8;
RET = 4262592093038893523_usize as u32;
(*_8) = [(-1270233136649564940_i64),(-6338824521240165386_i64),(-7345616979656342479_i64),(-8177070034898858153_i64),6086770397118082910_i64,(-2631645740897326627_i64)];
_1 = -119928524969828977202603102925895483618_i128;
_3 = [true,true,false,false,true];
_4 = !(-1858587984_i32);
_8 = core::ptr::addr_of_mut!(_7);
_11 = _9 << _4;
RET = 3091602672_u32 ^ 3435726073_u32;
_9 = -_11;
RET = 4068104139_u32;
_2 = '\u{24b68}';
(*_8) = [(-5112303103281249365_i64),(-6491499835425121755_i64),8125167545408080065_i64,(-4863724137714153403_i64),5121498081793535483_i64,4469823474476962453_i64];
match RET {
0 => bb8,
4068104139 => bb10,
_ => bb9
}
}
bb8 = {
_1 = -(-169409794725705684042228042272466811030_i128);
_4 = 1551705671_i32 * 815373421_i32;
RET = 3625645997_u32;
_8 = core::ptr::addr_of_mut!(_7);
RET = !429947447_u32;
_1 = !(-108629144615646955972340142955457110436_i128);
(*_8) = [(-837753884902061051_i64),9220431088424379964_i64,6444395810731908365_i64,(-2562414392503408789_i64),2840583356135467488_i64,3494880282902200220_i64];
_9 = 9223372036854775807_isize << _4;
_5 = 2497505566298792856_u64 | 12772047237595100675_u64;
_4 = -189063046_i32;
_4 = 5_usize as i32;
_2 = '\u{108554}';
_7 = [4100706842117775625_i64,8457024202267165189_i64,2634387513767229880_i64,9066025648714656234_i64,5385493918639123225_i64,(-4938303047129886936_i64)];
Call(_5 = core::intrinsics::bswap(17300916349050118521_u64), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
_1 = 10997196226848698594576514677106824010_i128;
_7 = [(-3236127557744501735_i64),6015924327831740451_i64,(-5034184542253427531_i64),(-8696398340303976813_i64),(-7926777543983447737_i64),(-1502956181301593763_i64)];
_3 = [true,true,true,false,true];
Goto(bb2)
}
bb10 = {
_3 = [true,true,true,false,false];
_5 = 10875060459237217101_u64;
_3 = [false,true,false,false,true];
_1 = (-106667635186243983383507074666950737661_i128) + (-29444873333979544391719330099655235921_i128);
_5 = !7404821896381865653_u64;
_8 = core::ptr::addr_of_mut!((*_8));
_13 = RET as i32;
_17 = [36567_u16,14125_u16,37104_u16,19237_u16,6944_u16];
_12 = core::ptr::addr_of!(_11);
_2 = '\u{3fef6}';
(*_8) = [(-8721443205421687060_i64),1324845238588350753_i64,4801935602348845808_i64,5366010247079647881_i64,(-7276818578947806621_i64),7114332146716113675_i64];
_10 = 144_u8;
(*_8) = [(-1517388367005193970_i64),6954381950095080146_i64,(-1994203317502042859_i64),(-6018351810477714904_i64),4782579834664151306_i64,2272116279603529722_i64];
(*_8) = [2886449676462058752_i64,(-8077837320225606686_i64),(-3922912944323085152_i64),4724076945430040175_i64,4266462631637664249_i64,(-1191429070461305872_i64)];
Goto(bb11)
}
bb11 = {
_18.fld1 = (_1, _2, _13, RET);
_12 = core::ptr::addr_of!(_11);
_14 = _18.fld1.3 as u16;
_18.fld6 = [_18.fld1.2,_13,_13];
_10 = 201_u8;
_8 = core::ptr::addr_of_mut!((*_8));
_18.fld1.3 = !RET;
_18.fld2 = !0_usize;
_18.fld1.0 = !_1;
_18.fld0 = core::ptr::addr_of!(_18.fld4);
_8 = core::ptr::addr_of_mut!((*_8));
(*_8) = [2870593080313114328_i64,(-824050349278506209_i64),(-7574079805679394426_i64),(-3107173076429295797_i64),(-8395012263393701307_i64),3243720706505679696_i64];
(*_8) = [(-835506898166458629_i64),2066677869561738624_i64,(-6433685146037810960_i64),(-8910153290029838736_i64),552912319257656397_i64,(-2162164227518275376_i64)];
(*_12) = _9 | _9;
RET = _18.fld1.3;
_18.fld3 = Adt27::Variant0 { fld0: _18.fld1,fld1: _18.fld1.0,fld2: (-21720_i16),fld3: _14 };
_18.fld1.0 = -Field::<i128>(Variant(_18.fld3, 0), 1);
_18.fld1.2 = _4;
_18.fld5 = [(-8375035257081801152_i64),(-5134681132906684511_i64),2093733247292579075_i64,(-3797948692626851435_i64)];
_18.fld4 = (-13661_i16) << (*_12);
(*_8) = [(-5604517039909239787_i64),(-4834811653780990963_i64),9106171669697026567_i64,3905060150030106281_i64,7823218436437952479_i64,5215441802125988134_i64];
_14 = Field::<u16>(Variant(_18.fld3, 0), 3);
_4 = _5 as i32;
_10 = 21_u8 & 250_u8;
_4 = -_13;
_18.fld0 = core::ptr::addr_of!(place!(Field::<i16>(Variant(_18.fld3, 0), 2)));
Goto(bb12)
}
bb12 = {
place!(Field::<(i128, char, i32, u32)>(Variant(_18.fld3, 0), 0)) = _18.fld1;
_9 = -(*_12);
_18.fld2 = 20_i8 as usize;
_18.fld1.2 = !Field::<(i128, char, i32, u32)>(Variant(_18.fld3, 0), 0).2;
place!(Field::<i16>(Variant(_18.fld3, 0), 2)) = !_18.fld4;
Goto(bb13)
}
bb13 = {
_1 = _5 as i128;
_11 = -_9;
_18.fld1 = Field::<(i128, char, i32, u32)>(Variant(_18.fld3, 0), 0);
_24 = Field::<i128>(Variant(_18.fld3, 0), 1) < _18.fld1.0;
_5 = !4061936930327316282_u64;
_18.fld2 = (-6799382423784452439_i64) as usize;
_22 = [_13,Field::<(i128, char, i32, u32)>(Variant(_18.fld3, 0), 0).2,Field::<(i128, char, i32, u32)>(Variant(_18.fld3, 0), 0).2];
_18.fld1.2 = _4 ^ _13;
place!(Field::<i128>(Variant(_18.fld3, 0), 1)) = _18.fld1.0 | Field::<(i128, char, i32, u32)>(Variant(_18.fld3, 0), 0).0;
_21 = _2;
(*_12) = _9 | _9;
_18.fld6 = [_18.fld1.2,_18.fld1.2,_18.fld1.2];
_4 = _18.fld1.2;
(*_12) = !_9;
_18.fld1.0 = -Field::<i128>(Variant(_18.fld3, 0), 1);
_26 = _18.fld2 as f64;
_11 = _9 >> _9;
(*_8) = [(-5523031116449259285_i64),1017075526167898780_i64,1835772281471181094_i64,(-4911361024211813074_i64),3966827981345153837_i64,9054490557657341857_i64];
_27.2.0 = _4 as u8;
_19 = _24;
_8 = core::ptr::addr_of_mut!(_7);
_18.fld2 = !13200860534131929176_usize;
_5 = 13283879628530999817_u64 ^ 9678049183883018802_u64;
_18.fld1.3 = RET >> Field::<i16>(Variant(_18.fld3, 0), 2);
Goto(bb14)
}
bb14 = {
_18.fld6 = [_13,_13,_4];
_27.2 = (_10, (-5179692841164719899_i64), _5);
_20 = &_5;
_13 = _18.fld1.2;
_2 = _21;
_19 = (*_12) < _11;
_23 = _19 & _19;
place!(Field::<(i128, char, i32, u32)>(Variant(_18.fld3, 0), 0)).2 = !_18.fld1.2;
place!(Field::<(i128, char, i32, u32)>(Variant(_18.fld3, 0), 0)).2 = _13 * _18.fld1.2;
RET = (*_12) as u32;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(19_usize, 5_usize, Move(_5), 24_usize, Move(_24), 23_usize, Move(_23), 11_usize, Move(_11)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(19_usize, 2_usize, Move(_2), 10_usize, Move(_10), 3_usize, Move(_3), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box(62_u8), std::hint::black_box(51_isize), std::hint::black_box(7_usize), std::hint::black_box((-6881877492642552690_i64)));
                
            }
impl PrintFDebug for Adt27{
	unsafe fn printf_debug(&self){unsafe{printf("Adt27::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt27 {
Variant0{
fld0: (i128, char, i32, u32),
fld1: i128,
fld2: i16,
fld3: u16,

},
Variant1{
fld0: *const isize,
fld1: *const i16,
fld2: (u128, [i64; 6]),

},
Variant2{
fld0: f64,
fld1: *const isize,
fld2: isize,
fld3: i8,
fld4: (i128, char, i32, u32),
fld5: (u128, [i64; 6]),

}}
impl PrintFDebug for Adt34{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt34{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt34 {
fld0: *const i16,
fld1: (i128, char, i32, u32),
fld2: usize,
fld3: Adt27,
fld4: i16,
fld5: [i64; 4],
fld6: [i32; 3],
}
impl PrintFDebug for Adt37{
	unsafe fn printf_debug(&self){unsafe{printf("Adt37::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt37 {
Variant0{
fld0: f64,
fld1: usize,
fld2: [i64; 6],
fld3: i8,
fld4: *const i128,

},
Variant1{
fld0: i128,
fld1: *const *const isize,
fld2: *const i16,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: Adt37,
fld1: *mut u128,
fld2: i128,
fld3: (u8, u64, (u8, i64, u64)),
fld4: usize,
fld5: u8,
fld6: [usize; 2],

},
Variant1{
fld0: [i64; 6],
fld1: char,

},
Variant2{
fld0: *const usize,
fld1: *mut [i64; 6],

},
Variant3{
fld0: *const *const isize,
fld1: [usize; 2],
fld2: u8,
fld3: i8,
fld4: i16,
fld5: *mut u128,
fld6: Adt27,
fld7: i128,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: u16,

},
Variant1{
fld0: i64,
fld1: *const i16,
fld2: *mut u128,
fld3: Adt37,
fld4: usize,
fld5: [i64; 6],

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: [i64; 6],
}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: u16,
fld1: u64,
fld2: *mut [i64; 6],
fld3: i8,
fld4: Adt50,
fld5: (i128, *const isize),
fld6: *const usize,
fld7: usize,

},
Variant1{
fld0: *const *const i128,
fld1: [i32; 7],

},
Variant2{
fld0: u128,
fld1: char,
fld2: [i32; 3],
fld3: [i32; 7],
fld4: [bool; 6],
fld5: Adt50,

}}
impl PrintFDebug for Adt73{
	unsafe fn printf_debug(&self){unsafe{printf("Adt73::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt73 {
Variant0{
fld0: i8,
fld1: [bool; 5],

},
Variant1{
fld0: (u128, [i64; 6]),
fld1: Adt44,
fld2: Adt53,
fld3: Adt37,
fld4: *mut [bool; 5],
fld5: *mut u128,
fld6: (i128, *const isize),
fld7: u16,

},
Variant2{
fld0: *mut [bool; 5],
fld1: i64,

},
Variant3{
fld0: u128,
fld1: *const (u128, [i64; 6]),
fld2: *const *const isize,
fld3: *const isize,

}}

