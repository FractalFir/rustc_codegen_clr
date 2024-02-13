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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> f32 {
mir! {
type RET = f32;
let _15: f32;
let _16: (bool, i16, i8);
let _17: (u64, f32);
let _18: Adt46;
let _19: u8;
let _20: isize;
let _21: i64;
let _22: [i16; 7];
let _23: *mut [i64; 7];
let _24: i8;
let _25: char;
let _26: isize;
let _27: i32;
let _28: &'static f32;
let _29: *mut (i8, i16);
let _30: ();
let _31: ();
{
_5 = 5727626493343990021_i64 as i16;
_7 = 4224645205307566475_i64;
_2 = '\u{f3b12}';
_2 = '\u{1d9bd}';
_3 = 91_isize + (-111_isize);
_8 = _2 as i128;
RET = _7 as f32;
_4 = _2 as i8;
_10 = 245_u8;
_11 = _7 as u16;
_8 = 15755137530571645425_u64 as i128;
_16.1 = -_5;
RET = _10 as f32;
_14 = 216751116021432662308477271105841385768_u128;
_6 = !1235157419_i32;
_15 = -RET;
_1 = !false;
RET = _15 + _15;
_12 = _1 as u32;
_9 = _12 as usize;
_16 = (_1, _5, _4);
_17 = (3308659582975500809_u64, RET);
_3 = !9223372036854775807_isize;
match _10 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
245 => bb6,
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
_17 = (9517177520738803498_u64, RET);
_17.0 = 12716752407637638492_u64;
_8 = -5979378823750736607850227356778454050_i128;
_12 = 1679451566_u32 >> _8;
_14 = 142661387163316706908893268368059186721_u128;
_16.0 = _16.1 < _16.1;
_1 = _16.0;
_9 = 16047734061040267228_usize * 5_usize;
_2 = '\u{f3a9c}';
_1 = !_16.0;
_6 = (-2047897667_i32);
match _14 {
142661387163316706908893268368059186721 => bb7,
_ => bb3
}
}
bb7 = {
_16 = (_1, _5, _4);
_10 = !202_u8;
_14 = 312809000176314375838630968356149956641_u128;
_16.1 = -_5;
_6 = _2 as i32;
_11 = !30593_u16;
_14 = !15659833097057109711230644987214304736_u128;
_13 = _12 as u64;
Goto(bb8)
}
bb8 = {
_16.1 = _16.2 as i16;
_17.1 = RET + _15;
_9 = 2448355714636231488_usize;
_17.1 = RET;
_16.0 = _1;
Call(_18 = fn1(_1, _16, _12, _1, _3, _4), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_16.0 = !_1;
place!(Field::<f32>(Variant(_18, 3), 2)) = RET;
_2 = '\u{8de43}';
_12 = _16.1 as u32;
RET = -Field::<f32>(Variant(_18, 3), 2);
_4 = _16.2;
_1 = _16.0 > _16.0;
RET = Field::<f32>(Variant(_18, 3), 2) - _15;
_10 = !32_u8;
_19 = !_10;
_16.0 = _4 > _16.2;
_8 = (-148355591000206388928070124439390136487_i128) & (-5527036373880336141733121992036725708_i128);
_20 = _3 - _3;
RET = Field::<f32>(Variant(_18, 3), 2);
_17 = (_13, RET);
_6 = (-622503127_i32);
place!(Field::<i16>(Variant(_18, 3), 0)) = _16.1 | _16.1;
_16.2 = _20 as i8;
_17.0 = !_13;
match _7 {
0 => bb2,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
4224645205307566475 => bb17,
_ => bb16
}
}
bb10 = {
_16.1 = _16.2 as i16;
_17.1 = RET + _15;
_9 = 2448355714636231488_usize;
_17.1 = RET;
_16.0 = _1;
Call(_18 = fn1(_1, _16, _12, _1, _3, _4), ReturnTo(bb9), UnwindUnreachable())
}
bb11 = {
_16 = (_1, _5, _4);
_10 = !202_u8;
_14 = 312809000176314375838630968356149956641_u128;
_16.1 = -_5;
_6 = _2 as i32;
_11 = !30593_u16;
_14 = !15659833097057109711230644987214304736_u128;
_13 = _12 as u64;
Goto(bb8)
}
bb12 = {
_17 = (9517177520738803498_u64, RET);
_17.0 = 12716752407637638492_u64;
_8 = -5979378823750736607850227356778454050_i128;
_12 = 1679451566_u32 >> _8;
_14 = 142661387163316706908893268368059186721_u128;
_16.0 = _16.1 < _16.1;
_1 = _16.0;
_9 = 16047734061040267228_usize * 5_usize;
_2 = '\u{f3a9c}';
_1 = !_16.0;
_6 = (-2047897667_i32);
match _14 {
142661387163316706908893268368059186721 => bb7,
_ => bb3
}
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
Return()
}
bb17 = {
_22 = [_5,Field::<i16>(Variant(_18, 3), 0),Field::<i16>(Variant(_18, 3), 0),_5,Field::<i16>(Variant(_18, 3), 0),Field::<i16>(Variant(_18, 3), 0),_5];
place!(Field::<[bool; 2]>(Variant(_18, 3), 1)) = [_16.0,_1];
place!(Field::<i16>(Variant(_18, 3), 0)) = _16.0 as i16;
_15 = RET + Field::<f32>(Variant(_18, 3), 2);
place!(Field::<f32>(Variant(_18, 3), 2)) = _16.2 as f32;
_7 = _13 as i64;
_17.1 = _19 as f32;
_21 = -_7;
_10 = _20 as u8;
_16.0 = !_1;
_26 = _20 << _8;
_9 = !2_usize;
SetDiscriminant(_18, 1);
place!(Field::<usize>(Variant(_18, 1), 3)) = !_9;
place!(Field::<Adt41>(Variant(_18, 1), 2)) = Adt41 { fld0: _12,fld1: _14 };
place!(Field::<Adt41>(Variant(_18, 1), 2)).fld0 = _12;
Goto(bb18)
}
bb18 = {
Call(_30 = dump_var(0_usize, 1_usize, Move(_1), 22_usize, Move(_22), 2_usize, Move(_2), 11_usize, Move(_11)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_30 = dump_var(0_usize, 3_usize, Move(_3), 14_usize, Move(_14), 10_usize, Move(_10), 26_usize, Move(_26)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_30 = dump_var(0_usize, 7_usize, Move(_7), 4_usize, Move(_4), 31_usize, _31, 31_usize, _31), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: bool,mut _2: (bool, i16, i8),mut _3: u32,mut _4: bool,mut _5: isize,mut _6: i8) -> Adt46 {
mir! {
type RET = Adt46;
let _7: *mut [i64; 7];
let _8: (u64, f32);
let _9: Adt42;
let _10: *mut [u32; 5];
let _11: Adt43;
let _12: Adt48;
let _13: [i64; 7];
let _14: ((u64, f32), u8, f32, [bool; 2]);
let _15: (i32, char, i8);
let _16: ();
let _17: ();
{
_4 = _2.0;
Goto(bb1)
}
bb1 = {
_2.1 = 13100_i16 << _3;
_4 = _1;
_2.1 = (-2147389626_i32) as i16;
_2.0 = !_1;
_3 = !2714736403_u32;
_2.1 = 11869_i16 & 3739_i16;
_8.0 = !429029490207186029_u64;
_2 = (_1, (-15126_i16), _6);
_8.0 = 10737112536437426954_u64;
_2.2 = _6 - _6;
_2 = (_1, 11043_i16, _6);
_2.1 = 30903_i16 | (-27649_i16);
_8.1 = (-1613184661970805356_i64) as f32;
_2.2 = 63212_u16 as i8;
_2.2 = _6 * _6;
_11 = Adt43::Variant1 { fld0: _5 };
_4 = _1;
place!(Field::<isize>(Variant(_11, 1), 0)) = _5;
place!(Field::<isize>(Variant(_11, 1), 0)) = !_5;
_2.0 = !_4;
_2.0 = _8.1 <= _8.1;
Call(RET = fn2(Field::<isize>(Variant(_11, 1), 0), _2.1, _6, _2.1, _8.0, _2.1, Field::<isize>(Variant(_11, 1), 0), _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
place!(Field::<i16>(Variant(RET, 3), 0)) = -_2.1;
place!(Field::<i16>(Variant(RET, 3), 0)) = _2.1 * _2.1;
place!(Field::<f32>(Variant(RET, 3), 2)) = -_8.1;
_2.2 = _6 << Field::<i16>(Variant(RET, 3), 0);
_8.0 = 4259630915000670149_u64 | 14110815682362409823_u64;
_8 = (14007312047491795978_u64, Field::<f32>(Variant(RET, 3), 2));
_1 = _6 >= _2.2;
place!(Field::<i16>(Variant(RET, 3), 0)) = -_2.1;
place!(Field::<[bool; 2]>(Variant(RET, 3), 1)) = [_4,_4];
_3 = _8.0 as u32;
_4 = _8.0 != _8.0;
_8.1 = Field::<f32>(Variant(RET, 3), 2) - Field::<f32>(Variant(RET, 3), 2);
_1 = _2.2 > _6;
place!(Field::<[bool; 2]>(Variant(RET, 3), 1)) = [_4,_1];
_5 = -Field::<isize>(Variant(_11, 1), 0);
_14.2 = _8.1 - Field::<f32>(Variant(RET, 3), 2);
_14.3 = [_4,_4];
_4 = Field::<isize>(Variant(_11, 1), 0) >= Field::<isize>(Variant(_11, 1), 0);
place!(Field::<f32>(Variant(RET, 3), 2)) = -_8.1;
_3 = !3229141032_u32;
place!(Field::<[bool; 2]>(Variant(RET, 3), 1)) = _14.3;
SetDiscriminant(_11, 0);
place!(Field::<f32>(Variant(RET, 3), 2)) = -_8.1;
place!(Field::<f32>(Variant(RET, 3), 2)) = 247683659843386481560986854688322974045_u128 as f32;
place!(Field::<i16>(Variant(RET, 3), 0)) = -_2.1;
Goto(bb3)
}
bb3 = {
Call(_16 = dump_var(1_usize, 1_usize, Move(_1), 6_usize, Move(_6), 2_usize, Move(_2), 17_usize, _17), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: isize,mut _2: i16,mut _3: i8,mut _4: i16,mut _5: u64,mut _6: i16,mut _7: isize,mut _8: (bool, i16, i8)) -> Adt46 {
mir! {
type RET = Adt46;
let _9: ((u128, (i128, i8, i8, u16)),);
let _10: (bool, i16, i8);
let _11: Adt56;
let _12: isize;
let _13: u8;
let _14: (bool, i16, i8);
let _15: ((u128, (i128, i8, i8, u16)),);
let _16: u16;
let _17: Adt46;
let _18: (i32, char, i8);
let _19: isize;
let _20: Adt48;
let _21: u32;
let _22: Adt50;
let _23: i128;
let _24: ();
let _25: ();
{
_7 = !_1;
_8 = (true, _2, _3);
_3 = 153_u8 as i8;
Goto(bb1)
}
bb1 = {
_9.0.1.0 = 142612166823813134742624903906672094603_i128 * 169573495642508180369222034500822576643_i128;
_9.0.1 = (144683830766258998950248383968181012501_i128, _3, _8.2, 63059_u16);
_8 = (true, _2, _9.0.1.1);
_10.0 = _8.0 | _8.0;
_10.1 = _2;
_4 = _6;
_2 = _6;
_9.0.0 = 193979779692292350756141307858539226199_u128;
_3 = _9.0.1.2;
match _9.0.1.3 {
0 => bb2,
63059 => bb4,
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
_10.1 = !_8.1;
_8 = (_10.0, _6, _9.0.1.2);
_10.2 = _3 ^ _9.0.1.1;
_9.0.0 = 173197260384275751876277683955901156781_u128;
_2 = _8.1 - _10.1;
_4 = !_2;
_12 = _1;
_8.2 = 7_usize as i8;
_8.2 = !_9.0.1.2;
_12 = _1;
_13 = !72_u8;
_5 = _9.0.1.0 as u64;
_14 = _8;
_3 = 1802486556_u32 as i8;
_14.2 = (-5745591533184651779_i64) as i8;
_4 = _6 * _8.1;
_8.1 = _6 & _4;
_15.0 = (_9.0.0, _9.0.1);
_9.0.1 = _15.0.1;
_10.2 = -_9.0.1.1;
_15.0.1.2 = _10.2;
_10.2 = !_8.2;
match _9.0.1.3 {
0 => bb3,
1 => bb5,
2 => bb6,
3 => bb7,
63059 => bb9,
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
_9.0.1.0 = 142612166823813134742624903906672094603_i128 * 169573495642508180369222034500822576643_i128;
_9.0.1 = (144683830766258998950248383968181012501_i128, _3, _8.2, 63059_u16);
_8 = (true, _2, _9.0.1.1);
_10.0 = _8.0 | _8.0;
_10.1 = _2;
_4 = _6;
_2 = _6;
_9.0.0 = 193979779692292350756141307858539226199_u128;
_3 = _9.0.1.2;
match _9.0.1.3 {
0 => bb2,
63059 => bb4,
_ => bb3
}
}
bb8 = {
Return()
}
bb9 = {
_16 = _15.0.1.3 >> _3;
_15.0.1.0 = _9.0.1.0 >> _9.0.0;
_9.0.1.0 = '\u{5b829}' as i128;
_7 = 11927233955078697418_usize as isize;
_5 = _7 as u64;
_15 = _9;
_9.0.1.3 = _5 as u16;
_14.2 = _8.2 ^ _9.0.1.2;
_9.0.1.3 = !_16;
_9.0.1.2 = _3 * _10.2;
_9 = _15;
Call(RET = fn3(_12, _9.0.1.3, _10, _8, _14.0, _15.0.1.3, _10.0, _9.0.1.3, _10, _10, _14), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_9.0.1.0 = _15.0.1.0;
_9.0.1 = (_15.0.1.0, _10.2, _14.2, _15.0.1.3);
_9 = (_15.0,);
_18.0 = (-822684345_i32);
_12 = 3129707139166503384_i64 as isize;
_19 = _1;
_8.0 = _14.0;
_21 = _19 as u32;
_13 = Field::<f32>(Variant(RET, 3), 2) as u8;
_5 = !12949666052452359284_u64;
_6 = Field::<i16>(Variant(RET, 3), 0);
Goto(bb11)
}
bb11 = {
Call(_24 = dump_var(2_usize, 5_usize, Move(_5), 21_usize, Move(_21), 19_usize, Move(_19), 14_usize, Move(_14)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_24 = dump_var(2_usize, 1_usize, Move(_1), 16_usize, Move(_16), 6_usize, Move(_6), 13_usize, Move(_13)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: isize,mut _2: u16,mut _3: (bool, i16, i8),mut _4: (bool, i16, i8),mut _5: bool,mut _6: u16,mut _7: bool,mut _8: u16,mut _9: (bool, i16, i8),mut _10: (bool, i16, i8),mut _11: (bool, i16, i8)) -> Adt46 {
mir! {
type RET = Adt46;
let _12: ((isize,), u32);
let _13: &'static f32;
let _14: (i8, i16);
let _15: i8;
let _16: (i8, i16);
let _17: Adt53;
let _18: i64;
let _19: isize;
let _20: i16;
let _21: [i64; 7];
let _22: (bool, i16, i8);
let _23: [i16; 7];
let _24: u64;
let _25: usize;
let _26: Adt45;
let _27: (isize,);
let _28: &'static f32;
let _29: (i128, i8, i8, u16);
let _30: ((isize,), u32);
let _31: isize;
let _32: (i8, i16);
let _33: f64;
let _34: *mut (i8, i16);
let _35: u8;
let _36: Adt51;
let _37: *const u64;
let _38: ([bool; 8], i128, (isize,));
let _39: ((isize,), u32);
let _40: Adt56;
let _41: bool;
let _42: u128;
let _43: f64;
let _44: *mut (i8, i16);
let _45: f32;
let _46: isize;
let _47: bool;
let _48: [bool; 2];
let _49: f32;
let _50: i16;
let _51: [u32; 5];
let _52: Adt50;
let _53: [i64; 7];
let _54: isize;
let _55: (i128, i8, i8, u16);
let _56: usize;
let _57: [i64; 7];
let _58: f32;
let _59: isize;
let _60: bool;
let _61: ((isize,), u32);
let _62: ((isize,), u32);
let _63: f32;
let _64: [bool; 8];
let _65: Adt54;
let _66: *mut [u32; 5];
let _67: (i128, i8, i8, u16);
let _68: u32;
let _69: [u32; 5];
let _70: ((u128, (i128, i8, i8, u16)),);
let _71: u64;
let _72: Adt48;
let _73: f64;
let _74: Adt43;
let _75: f32;
let _76: ();
let _77: ();
{
_9.0 = _4.0 & _10.0;
_5 = _11.0;
_11.1 = -_10.1;
_11.2 = 1_usize as i8;
_4.2 = -_3.2;
_5 = _4.0 & _9.0;
match _6 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
63059 => bb7,
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
_4.2 = _9.2 + _9.2;
_3.1 = !_4.1;
_11.0 = !_5;
_14 = (_11.2, _11.1);
_3 = (_5, _11.1, _9.2);
Goto(bb8)
}
bb8 = {
_9.1 = _3.1 & _11.1;
_16 = _14;
_12.0.0 = !_1;
_11.1 = _9.1;
_15 = _4.2 | _4.2;
_12.1 = !3749099093_u32;
_7 = _11.0;
_8 = 13280701645545751709_usize as u16;
_16.1 = -_11.1;
_4.0 = _5 & _5;
_16.1 = _9.1 ^ _11.1;
_11.0 = _4.0 ^ _4.0;
_8 = _6;
_22 = (_3.0, _11.1, _11.2);
_6 = !_2;
Call(_10.1 = fn4(_11.0, _16.1, _11.0), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_19 = _16.1 as isize;
_12.0.0 = 312891427607090988260151539893311197346_u128 as isize;
_11.0 = _10.0 < _7;
_3.1 = _11.1;
_21 = [(-1746033306125876774_i64),(-6243431704175824451_i64),3356520780167336351_i64,4744918910943228926_i64,(-436704728425655099_i64),4197494344823560503_i64,(-2406061192335022122_i64)];
_23 = [_22.1,_3.1,_16.1,_16.1,_11.1,_9.1,_10.1];
match _2 {
0 => bb7,
1 => bb2,
2 => bb5,
3 => bb4,
4 => bb10,
5 => bb11,
63059 => bb13,
_ => bb12
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
_22.1 = -_10.1;
_16.1 = _22.1;
_24 = 17956591006251172661_u64;
Goto(bb14)
}
bb14 = {
_14 = (_4.2, _4.1);
_22.0 = _4.0 | _4.0;
_9 = (_22.0, _4.1, _15);
_4.0 = !_9.0;
_20 = '\u{3657c}' as i16;
_16.0 = 70_u8 as i8;
_3.0 = !_22.0;
_2 = !_8;
_4.2 = -_9.2;
_23 = [_9.1,_14.1,_16.1,_22.1,_4.1,_9.1,_16.1];
_9 = (_4.0, _16.1, _3.2);
_10.2 = _16.0 * _22.2;
_16.0 = _4.2 >> _19;
Goto(bb15)
}
bb15 = {
_2 = _6;
_25 = !677448465140129235_usize;
_10.2 = -_15;
_29.0 = -131063452500654178814772443266454688360_i128;
_20 = _4.1;
_10.2 = _16.0;
_12.0 = (_1,);
_10.0 = _3.0;
Call(_22.1 = core::intrinsics::bswap(_9.1), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_3 = (_4.0, _16.1, _15);
_22 = _9;
_11.2 = _5 as i8;
_18 = 107026916743619533266990049444246630550_u128 as i64;
_16.1 = _24 as i16;
_19 = _1 - _1;
_11.2 = _10.2;
_18 = (-4090506931550145803_i64);
_29.3 = !_2;
_9.1 = -_14.1;
_25 = 1_usize - 3_usize;
_14.1 = _10.1 + _20;
_29.1 = _12.0.0 as i8;
_32.1 = _22.1 + _14.1;
_27 = (_19,);
match _8 {
0 => bb4,
1 => bb5,
2 => bb3,
3 => bb17,
4 => bb18,
5 => bb19,
63059 => bb21,
_ => bb20
}
}
bb17 = {
_2 = _6;
_25 = !677448465140129235_usize;
_10.2 = -_15;
_29.0 = -131063452500654178814772443266454688360_i128;
_20 = _4.1;
_10.2 = _16.0;
_12.0 = (_1,);
_10.0 = _3.0;
Call(_22.1 = core::intrinsics::bswap(_9.1), ReturnTo(bb16), UnwindUnreachable())
}
bb18 = {
Return()
}
bb19 = {
Return()
}
bb20 = {
Return()
}
bb21 = {
_10.0 = _4.0;
_31 = 6922043125176293523664890408165021175_u128 as isize;
_16.1 = _3.1 >> _14.1;
_32 = _14;
_27.0 = !_19;
_29.2 = _19 as i8;
_10.1 = _24 as i16;
_22.2 = _16.0;
_12 = (_27, 2337662785_u32);
_19 = _31;
_2 = !_6;
_29 = ((-7762989980752278880570334652618143637_i128), _10.2, _11.2, _8);
_5 = _3.0 != _9.0;
_38.0 = [_11.0,_22.0,_22.0,_22.0,_10.0,_5,_22.0,_9.0];
_10 = (_11.0, _4.1, _29.1);
_33 = _24 as f64;
_12 = (_27, 3082511995_u32);
_14.1 = _12.1 as i16;
_22.0 = _10.1 == _9.1;
_3.1 = _10.1 & _16.1;
_33 = 176285486_i32 as f64;
_29 = ((-114697174457768128219742264601780296160_i128), _11.2, _14.0, _2);
_4.2 = _9.0 as i8;
_20 = _22.1;
match _29.0 {
0 => bb10,
1 => bb2,
2 => bb3,
3 => bb12,
4 => bb5,
5 => bb6,
6 => bb18,
225585192463170335243632342829987915296 => bb22,
_ => bb8
}
}
bb22 = {
_34 = core::ptr::addr_of_mut!(_32);
_11.1 = 215_u8 as i16;
_39.0 = _12.0;
_27 = (_19,);
_3.2 = _15;
_37 = core::ptr::addr_of!(_24);
_3 = (_9.0, _16.1, _16.0);
match _24 {
0 => bb7,
1 => bb6,
17956591006251172661 => bb24,
_ => bb23
}
}
bb23 = {
_10.0 = _4.0;
_31 = 6922043125176293523664890408165021175_u128 as isize;
_16.1 = _3.1 >> _14.1;
_32 = _14;
_27.0 = !_19;
_29.2 = _19 as i8;
_10.1 = _24 as i16;
_22.2 = _16.0;
_12 = (_27, 2337662785_u32);
_19 = _31;
_2 = !_6;
_29 = ((-7762989980752278880570334652618143637_i128), _10.2, _11.2, _8);
_5 = _3.0 != _9.0;
_38.0 = [_11.0,_22.0,_22.0,_22.0,_10.0,_5,_22.0,_9.0];
_10 = (_11.0, _4.1, _29.1);
_33 = _24 as f64;
_12 = (_27, 3082511995_u32);
_14.1 = _12.1 as i16;
_22.0 = _10.1 == _9.1;
_3.1 = _10.1 & _16.1;
_33 = 176285486_i32 as f64;
_29 = ((-114697174457768128219742264601780296160_i128), _11.2, _14.0, _2);
_4.2 = _9.0 as i8;
_20 = _22.1;
match _29.0 {
0 => bb10,
1 => bb2,
2 => bb3,
3 => bb12,
4 => bb5,
5 => bb6,
6 => bb18,
225585192463170335243632342829987915296 => bb22,
_ => bb8
}
}
bb24 = {
_4.0 = _3.0 | _9.0;
_8 = _6 >> _4.1;
_34 = core::ptr::addr_of_mut!((*_34));
_2 = !_8;
_43 = _33;
_29 = (59449886650734917265664756762985867463_i128, _3.2, _4.2, _6);
_15 = _29.2 * _22.2;
_30.1 = !_12.1;
_11 = (_7, _3.1, _4.2);
Goto(bb25)
}
bb25 = {
_2 = _30.1 as u16;
_48 = [_22.0,_4.0];
_42 = _25 as u128;
_32.0 = 169_u8 as i8;
_12.0.0 = _42 as isize;
_18 = (-1775822849_i32) as i64;
_12.1 = _30.1 | _30.1;
match _29.0 {
0 => bb13,
1 => bb3,
2 => bb26,
59449886650734917265664756762985867463 => bb28,
_ => bb27
}
}
bb26 = {
_4.0 = _3.0 | _9.0;
_8 = _6 >> _4.1;
_34 = core::ptr::addr_of_mut!((*_34));
_2 = !_8;
_43 = _33;
_29 = (59449886650734917265664756762985867463_i128, _3.2, _4.2, _6);
_15 = _29.2 * _22.2;
_30.1 = !_12.1;
_11 = (_7, _3.1, _4.2);
Goto(bb25)
}
bb27 = {
Return()
}
bb28 = {
(*_37) = _12.1 as u64;
(*_37) = 17608774646051540661_u64 >> _29.0;
_10.1 = !_3.1;
_32.1 = _10.1;
_9.0 = _10.2 > _15;
_7 = _11.2 < _16.0;
_32.0 = _4.2;
_41 = (*_37) <= (*_37);
_38.2.0 = _43 as isize;
_10 = (_4.0, (*_34).1, _11.2);
_16.0 = -(*_34).0;
_46 = 199_u8 as isize;
_32.1 = _10.1;
_48 = [_4.0,_5];
_16.0 = _10.2;
_34 = core::ptr::addr_of_mut!(_14);
(*_34).0 = _11.2 ^ _10.2;
Call(_50 = core::intrinsics::bswap(_16.1), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
(*_34).0 = _11.2;
_35 = 27_u8;
_3.0 = _31 >= _12.0.0;
_39 = _12;
_10.1 = !_3.1;
_12.0.0 = _18 as isize;
_30.0.0 = _31 + _12.0.0;
_15 = _39.1 as i8;
_53 = [_18,_18,_18,_18,_18,_18,_18];
_29.0 = !112784297489285418799896979300783579856_i128;
_31 = _46 + _30.0.0;
_37 = core::ptr::addr_of!((*_37));
_14 = (_16.0, _22.1);
_33 = _42 as f64;
_38.2.0 = !_27.0;
_16 = ((*_34).0, _11.1);
_3.1 = _32.1;
_13 = &_45;
_51 = [_30.1,_39.1,_12.1,_12.1,_12.1];
_12 = (_30.0, _39.1);
match _35 {
0 => bb1,
1 => bb22,
2 => bb19,
3 => bb25,
4 => bb5,
5 => bb30,
27 => bb32,
_ => bb31
}
}
bb30 = {
Return()
}
bb31 = {
_19 = _16.1 as isize;
_12.0.0 = 312891427607090988260151539893311197346_u128 as isize;
_11.0 = _10.0 < _7;
_3.1 = _11.1;
_21 = [(-1746033306125876774_i64),(-6243431704175824451_i64),3356520780167336351_i64,4744918910943228926_i64,(-436704728425655099_i64),4197494344823560503_i64,(-2406061192335022122_i64)];
_23 = [_22.1,_3.1,_16.1,_16.1,_11.1,_9.1,_10.1];
match _2 {
0 => bb7,
1 => bb2,
2 => bb5,
3 => bb4,
4 => bb10,
5 => bb11,
63059 => bb13,
_ => bb12
}
}
bb32 = {
_11.2 = _4.2;
_29.3 = 777254915_i32 as u16;
_44 = core::ptr::addr_of_mut!(_14);
_22.1 = (*_34).1;
_16.1 = _25 as i16;
_48 = [_10.0,_10.0];
_12.0 = (_31,);
(*_37) = 12597190326234339554_u64;
(*_37) = _6 as u64;
_48 = [_10.0,_5];
(*_44).0 = -_32.0;
_55.3 = !_8;
_4.0 = _9.0;
_55.1 = _29.2;
_29 = (50505077953551156718970583781138322667_i128, (*_44).0, _16.0, _2);
_33 = (*_37) as f64;
_55.0 = !_29.0;
Goto(bb33)
}
bb33 = {
(*_44) = _32;
_29.1 = _35 as i8;
_30.0.0 = _2 as isize;
_56 = !_25;
(*_34).1 = !_10.1;
_61.1 = _30.1 | _39.1;
_20 = !_14.1;
_32.0 = _16.0 | _4.2;
_60 = !_7;
_29.1 = _32.0 << (*_34).1;
_47 = _4.0 <= _4.0;
_1 = !_30.0.0;
_62.0.0 = _12.0.0 + _12.0.0;
_39 = _12;
_29 = (_55.0, (*_34).0, _16.0, _55.3);
_22.2 = _20 as i8;
_7 = _5 & _60;
_60 = (*_34).0 != _22.2;
_45 = (*_34).0 as f32;
_27.0 = _11.0 as isize;
_12.1 = _27.0 as u32;
Goto(bb34)
}
bb34 = {
_5 = _60;
_3 = (_7, _9.1, (*_34).0);
(*_37) = 14215984539589124403_u64 | 5904790966693072831_u64;
match _35 {
0 => bb10,
1 => bb22,
2 => bb19,
3 => bb30,
4 => bb5,
27 => bb36,
_ => bb35
}
}
bb35 = {
_4.0 = _3.0 | _9.0;
_8 = _6 >> _4.1;
_34 = core::ptr::addr_of_mut!((*_34));
_2 = !_8;
_43 = _33;
_29 = (59449886650734917265664756762985867463_i128, _3.2, _4.2, _6);
_15 = _29.2 * _22.2;
_30.1 = !_12.1;
_11 = (_7, _3.1, _4.2);
Goto(bb25)
}
bb36 = {
_66 = core::ptr::addr_of_mut!(_51);
_61.0.0 = _27.0 >> (*_34).0;
(*_34) = (_55.1, _20);
_28 = &_45;
_62 = (_27, _12.1);
_39.0.0 = 1016026684_i32 as isize;
_62.0.0 = !_61.0.0;
_22.0 = _3.0;
_5 = _29.0 < _55.0;
_11.0 = _5;
_13 = Move(_28);
_34 = _44;
_66 = core::ptr::addr_of_mut!((*_66));
_38.0 = [_9.0,_22.0,_47,_11.0,_47,_4.0,_60,_9.0];
_16.1 = _4.1;
_29 = (_55.0, _10.2, (*_44).0, _8);
_4.1 = -_22.1;
_12.1 = (*_34).1 as u32;
_11.1 = _42 as i16;
_34 = _44;
(*_34).1 = _10.1;
(*_44).0 = _11.2;
_49 = _29.0 as f32;
Goto(bb37)
}
bb37 = {
_67.0 = _29.0;
_55.1 = _42 as i8;
_22.1 = !_9.1;
_32 = (*_44);
_52 = Adt50::Variant1 { fld0: _9.0 };
_70.0.1.3 = !_2;
_12.1 = _61.1 + _62.1;
_35 = !95_u8;
_22.1 = _61.0.0 as i16;
_61.1 = _42 as u32;
_41 = !_5;
_24 = 17979364511349446613_u64;
_66 = core::ptr::addr_of_mut!((*_66));
_44 = _34;
_16 = (_29.2, _22.1);
_62.1 = _39.1 ^ _12.1;
_28 = Move(_13);
_61 = (_62.0, _62.1);
_70.0.1 = (_67.0, (*_34).0, (*_44).0, _8);
_28 = &_45;
_3.2 = _14.0;
SetDiscriminant(_52, 2);
_63 = _42 as f32;
_52 = Adt50::Variant2 { fld0: _38.0 };
RET = Adt46::Variant3 { fld0: _16.1,fld1: _48,fld2: _45 };
_67.3 = !_8;
Goto(bb38)
}
bb38 = {
Call(_76 = dump_var(3_usize, 10_usize, Move(_10), 25_usize, Move(_25), 7_usize, Move(_7), 9_usize, Move(_9)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_76 = dump_var(3_usize, 11_usize, Move(_11), 1_usize, Move(_1), 53_usize, Move(_53), 21_usize, Move(_21)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Call(_76 = dump_var(3_usize, 19_usize, Move(_19), 27_usize, Move(_27), 48_usize, Move(_48), 60_usize, Move(_60)), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Call(_76 = dump_var(3_usize, 42_usize, Move(_42), 39_usize, Move(_39), 50_usize, Move(_50), 41_usize, Move(_41)), ReturnTo(bb42), UnwindUnreachable())
}
bb42 = {
Call(_76 = dump_var(3_usize, 2_usize, Move(_2), 35_usize, Move(_35), 14_usize, Move(_14), 5_usize, Move(_5)), ReturnTo(bb43), UnwindUnreachable())
}
bb43 = {
Call(_76 = dump_var(3_usize, 4_usize, Move(_4), 77_usize, _77, 77_usize, _77, 77_usize, _77), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: bool,mut _2: i16,mut _3: bool) -> i16 {
mir! {
type RET = i16;
let _4: isize;
let _5: f32;
let _6: *mut [u32; 5];
let _7: Adt49;
let _8: ((u64, f32), u8, f32, [bool; 2]);
let _9: i8;
let _10: isize;
let _11: bool;
let _12: bool;
let _13: isize;
let _14: isize;
let _15: u16;
let _16: *mut (i8, i16);
let _17: [bool; 8];
let _18: f32;
let _19: ((u64, f32), u8, f32, [bool; 2]);
let _20: Adt48;
let _21: ((u64, f32), u8, f32, [bool; 2]);
let _22: isize;
let _23: ([bool; 8], i128, (isize,));
let _24: isize;
let _25: char;
let _26: *const u64;
let _27: (bool, i16, i8);
let _28: ((u128, (i128, i8, i8, u16)),);
let _29: Adt41;
let _30: i16;
let _31: Adt43;
let _32: i128;
let _33: (isize,);
let _34: Adt56;
let _35: Adt41;
let _36: u64;
let _37: isize;
let _38: Adt43;
let _39: isize;
let _40: Adt53;
let _41: isize;
let _42: ();
let _43: ();
{
_2 = (-4376_i16) ^ (-2325_i16);
_4 = _3 as isize;
_2 = (-8463_i16);
RET = 249580395563435997756431246195360412939_u128 as i16;
_4 = -(-97_isize);
_5 = 3273191783806407636_u64 as f32;
_1 = _3 | _3;
Call(_1 = fn5(_3, _3, _3, _3, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = -_2;
RET = 2145867833_u32 as i16;
_8.0.1 = 3541816080220648530_usize as f32;
_8.3 = [_3,_3];
_8.0.1 = _5 * _5;
_8.1 = !109_u8;
_3 = !_1;
_8.1 = 245_u8;
_8.1 = !148_u8;
RET = _2 | _2;
_8.1 = _5 as u8;
_7 = Adt49 { fld0: 2511316766808508534_u64 };
_7 = Adt49 { fld0: 235126022164773357_u64 };
_9 = !(-63_i8);
_8.0.0 = RET as u64;
_5 = -_8.0.1;
_8.1 = _9 as u8;
_8.0 = (_7.fld0, _5);
_3 = _1;
_5 = (-1964071519_i32) as f32;
_10 = _4;
RET = _2;
RET = _2;
_8.2 = _5 - _5;
Call(_7 = fn6(_1, _8, _3, _3, _5, _8.3, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_11 = _1 < _1;
_12 = _11;
_8.1 = 233880350085938979430143083260068180128_u128 as u8;
_8.2 = RET as f32;
_8.3 = [_12,_3];
_8.0.1 = RET as f32;
_8.0.0 = _7.fld0 & _7.fld0;
_7 = Adt49 { fld0: _8.0.0 };
_8.3 = [_12,_12];
RET = _2 | _2;
_10 = _4 - _4;
RET = _2 << _10;
_3 = !_11;
_2 = RET;
_13 = _4 & _10;
_3 = _1 ^ _12;
_7 = Adt49 { fld0: _8.0.0 };
_4 = 3_usize as isize;
_1 = _11;
_5 = -_8.0.1;
_4 = _10;
_1 = _7.fld0 == _8.0.0;
_1 = _12;
_15 = _2 as u16;
_8.0.1 = _5 + _5;
_3 = !_12;
_7.fld0 = _8.0.0 << _2;
Goto(bb3)
}
bb3 = {
_9 = 1799158112_u32 as i8;
Call(_8.0.0 = core::intrinsics::transmute(_7.fld0), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = -_2;
_1 = _11 == _12;
_7 = Adt49 { fld0: _8.0.0 };
_8.0.0 = _9 as u64;
_11 = _1 & _12;
_4 = -_13;
_7 = Adt49 { fld0: _8.0.0 };
_10 = -_13;
_10 = -_4;
_2 = -RET;
_17 = [_3,_12,_11,_3,_3,_3,_3,_1];
RET = _7.fld0 as i16;
_18 = _5 * _8.0.1;
_8.0.0 = _7.fld0 | _7.fld0;
_12 = !_1;
Goto(bb5)
}
bb5 = {
_12 = _11;
_14 = !_13;
Goto(bb6)
}
bb6 = {
_8.0 = (_7.fld0, _8.2);
RET = -_2;
Goto(bb7)
}
bb7 = {
_21.2 = -_18;
RET = _2 >> _7.fld0;
_19.0.0 = _8.0.0;
_21.1 = 2236208863_u32 as u8;
_21.0.1 = _21.2 * _18;
_8.0.0 = _19.0.0 & _7.fld0;
Call(_8.3 = fn16(_11, _1, _3, _1, _11, _17, _12, _17, _11, _11, _12), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_21.1 = _8.1;
_21.0 = (_8.0.0, _5);
_19.1 = _8.1;
_17 = [_1,_3,_11,_3,_11,_12,_1,_3];
_10 = 1368621307_i32 as isize;
_19 = (_8.0, _21.1, _21.0.1, _8.3);
_5 = _18 * _19.2;
_21.1 = !_8.1;
_3 = !_11;
_21.0.1 = 754652412_u32 as f32;
_21.2 = -_18;
_23.2.0 = -_14;
_21.0.1 = _8.1 as f32;
_10 = -_14;
_21 = (_19.0, _19.1, _18, _8.3);
_21.0.0 = _8.0.0 >> _15;
_23.1 = '\u{8eada}' as i128;
_21.0.1 = _5;
_7 = Adt49 { fld0: _21.0.0 };
_3 = _12;
_24 = _23.2.0 * _13;
RET = _2 * _2;
Goto(bb9)
}
bb9 = {
_8.0.1 = 2_usize as f32;
_8.3 = [_11,_3];
RET = _2 | _2;
_22 = _14;
_19.2 = _24 as f32;
Goto(bb10)
}
bb10 = {
_8.0.0 = _7.fld0 + _7.fld0;
_21 = _8;
_1 = _12;
_15 = 13121_u16;
_19.0 = (_8.0.0, _19.2);
_23.0 = [_11,_1,_1,_12,_3,_3,_1,_1];
Call(_27.0 = fn17(_21.0, _23.0, _8.3, _23.0, _12, _19, _23, _19.3, _8.3, _21), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_8.3 = [_27.0,_27.0];
_29 = Adt41 { fld0: 1906080324_u32,fld1: 202099819270428165802004206618387856233_u128 };
_14 = _24 + _23.2.0;
_27.1 = RET;
_13 = _10 | _14;
_19 = (_21.0, _8.1, _5, _21.3);
_9 = !6_i8;
_28.0.1 = (_23.1, _9, _9, _15);
Call(_25 = fn18(_23.0, _19.3, _23, _23, _12, _19.3, _1, _21.3, _21, _23, _3, _8), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_27 = (_11, RET, _9);
_25 = '\u{c3caf}';
_29.fld0 = 1146615012_u32 + 2386387955_u32;
_12 = _1;
_18 = -_19.2;
_28.0.1 = (_23.1, _27.2, _27.2, _15);
_21 = (_19.0, _19.1, _18, _8.3);
_28.0.0 = _29.fld1;
_30 = _2 << _13;
_12 = _27.0 > _11;
_33 = (_4,);
_8 = _19;
_4 = _24 ^ _13;
_27.0 = _13 <= _13;
_7.fld0 = !_21.0.0;
_8.3 = [_11,_12];
_28.0.1.3 = _15;
_10 = _4 | _14;
_19.2 = -_18;
_22 = -_4;
_29 = Adt41 { fld0: 1018897300_u32,fld1: _28.0.0 };
_23.1 = 3_usize as i128;
match _29.fld0 {
0 => bb1,
1 => bb10,
2 => bb3,
3 => bb7,
4 => bb11,
5 => bb13,
1018897300 => bb15,
_ => bb14
}
}
bb13 = {
_12 = _11;
_14 = !_13;
Goto(bb6)
}
bb14 = {
_8.0.1 = 2_usize as f32;
_8.3 = [_11,_3];
RET = _2 | _2;
_22 = _14;
_19.2 = _24 as f32;
Goto(bb10)
}
bb15 = {
_29.fld1 = _28.0.0 - _28.0.0;
_14 = _10;
_18 = -_21.2;
Goto(bb16)
}
bb16 = {
Call(_42 = dump_var(4_usize, 11_usize, Move(_11), 3_usize, Move(_3), 13_usize, Move(_13), 23_usize, Move(_23)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(4_usize, 22_usize, Move(_22), 1_usize, Move(_1), 10_usize, Move(_10), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(4_usize, 30_usize, Move(_30), 12_usize, Move(_12), 43_usize, _43, 43_usize, _43), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool) -> bool {
mir! {
type RET = bool;
let _6: f32;
let _7: Adt40;
let _8: i8;
let _9: (i128, i8, i8, u16);
let _10: (u64, f32);
let _11: char;
let _12: ();
let _13: ();
{
_2 = !_3;
_2 = !_1;
RET = _3;
_5 = _1 <= _3;
_6 = 3787263694390129703_u64 as f32;
_3 = !RET;
_5 = _4 > _3;
_4 = !_2;
_4 = _2 <= _2;
_1 = _2 < _3;
RET = _5 == _1;
_3 = !_1;
_1 = _4 == RET;
_5 = _2;
_3 = _5;
_3 = _5;
_2 = _5;
_2 = _5 != _1;
_2 = _3 <= _4;
_4 = _1;
_9.2 = (-106_i8) >> 681697595_i32;
_9.1 = 247_u8 as i8;
_9 = (26143172137080479438101136215016661955_i128, 84_i8, (-76_i8), 40727_u16);
RET = !_3;
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(5_usize, 2_usize, Move(_2), 1_usize, Move(_1), 3_usize, Move(_3), 13_usize, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: bool,mut _2: ((u64, f32), u8, f32, [bool; 2]),mut _3: bool,mut _4: bool,mut _5: f32,mut _6: [bool; 2],mut _7: ((u64, f32), u8, f32, [bool; 2])) -> Adt49 {
mir! {
type RET = Adt49;
let _8: i8;
let _9: (u64, f32);
let _10: bool;
let _11: &'static f32;
let _12: isize;
let _13: Adt40;
let _14: Adt44;
let _15: i8;
let _16: usize;
let _17: bool;
let _18: i16;
let _19: Adt45;
let _20: ((u64, f32), u8, f32, [bool; 2]);
let _21: u64;
let _22: ((u128, (i128, i8, i8, u16)), (isize,), char, i128);
let _23: char;
let _24: u32;
let _25: &'static f32;
let _26: u128;
let _27: Adt49;
let _28: Adt47;
let _29: isize;
let _30: Adt43;
let _31: u16;
let _32: Adt40;
let _33: u16;
let _34: isize;
let _35: u8;
let _36: Adt53;
let _37: u16;
let _38: Adt41;
let _39: *const u64;
let _40: (bool, i16, i8);
let _41: ((isize,), u32);
let _42: ();
let _43: ();
{
RET = Adt49 { fld0: _7.0.0 };
RET = Adt49 { fld0: _7.0.0 };
_2 = (_7.0, _7.1, _7.0.1, _6);
_1 = !_4;
_2.0.1 = _2.2;
_2.0.0 = !_7.0.0;
match RET.fld0 {
235126022164773357 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_2.0 = _7.0;
_2.3 = [_3,_1];
_7.0 = (RET.fld0, _7.2);
_7.2 = _2.0.1;
_7.0.1 = -_2.0.1;
RET.fld0 = _7.0.0;
_2.0.1 = _2.2 + _7.2;
_7.1 = _2.1;
_2.0.1 = -_5;
RET.fld0 = (-9223372036854775808_isize) as u64;
_2.2 = 492121246_i32 as f32;
_7.0.0 = 44465_u16 as u64;
_2.0.0 = !_7.0.0;
_7.3 = [_4,_4];
_5 = _2.0.1;
RET = Adt49 { fld0: _2.0.0 };
_2.2 = _2.0.1 + _7.2;
RET.fld0 = '\u{8e4cc}' as u64;
_5 = _7.2 + _2.0.1;
RET = Adt49 { fld0: _7.0.0 };
_2.1 = _7.1;
RET = Adt49 { fld0: _2.0.0 };
Goto(bb3)
}
bb3 = {
_2 = (_7.0, _7.1, _5, _6);
_8 = 6609_i16 as i8;
_2.3 = _6;
_10 = _3;
_7.0 = _2.0;
_9.1 = _8 as f32;
_2.0.0 = RET.fld0 - _7.0.0;
_9.1 = _2.2 * _2.2;
_9.0 = !_7.0.0;
_12 = -9223372036854775807_isize;
_7 = (_2.0, _2.1, _5, _2.3);
_14.fld0.3 = 36653_u16 << _2.0.0;
_8 = (-117_i8) | 118_i8;
_7.3 = [_10,_10];
_10 = _1 > _3;
_2.0 = (_7.0.0, _9.1);
_7 = (_9, _2.1, _2.0.1, _6);
_3 = _1 | _1;
_7 = (_9, _2.1, _9.1, _2.3);
Goto(bb4)
}
bb4 = {
_14.fld0.2 = _8 << _14.fld0.3;
_14.fld0.1 = _14.fld0.2;
_6 = [_4,_10];
_14.fld0.0 = (-163606657165569673596565049598099808941_i128) + 99454157015944824489773998411950922392_i128;
_11 = &_5;
_7.3 = [_10,_10];
RET.fld0 = _2.0.0;
Goto(bb5)
}
bb5 = {
_9 = (RET.fld0, _7.2);
_9.0 = _7.0.0;
_14.fld0.3 = 57552_u16;
_17 = !_1;
_12 = 9223372036854775807_isize ^ 120_isize;
RET = Adt49 { fld0: _9.0 };
_18 = 20904_i16;
RET = Adt49 { fld0: _7.0.0 };
_15 = _14.fld0.2 >> _7.1;
_2.0 = (RET.fld0, _2.2);
RET.fld0 = !_7.0.0;
RET.fld0 = _9.0 | _7.0.0;
_16 = 7_usize - 1_usize;
_2.2 = -_2.0.1;
RET = Adt49 { fld0: _7.0.0 };
RET = Adt49 { fld0: _9.0 };
_10 = _3 > _17;
_15 = _12 as i8;
_2 = _7;
_14.fld0.3 = 29256_u16;
_10 = _1 ^ _4;
_17 = !_4;
_16 = !4506840345852438220_usize;
_16 = 168659395306756312781147786200361469731_u128 as usize;
_15 = _14.fld0.1;
_2.0 = _7.0;
_9 = (RET.fld0, _7.0.1);
_7.3 = _6;
Goto(bb6)
}
bb6 = {
_20.0.1 = 1381948972_i32 as f32;
_9.0 = _7.0.0;
_4 = _3 ^ _17;
_20.0 = _7.0;
_7.0.1 = _20.0.1 + _2.2;
_20.1 = !_2.1;
_11 = &(*_11);
_20.0.1 = -_7.0.1;
_22.0.1.3 = _14.fld0.3;
_18 = _14.fld0.0 as i16;
_22.0.1.2 = _8;
_7.1 = 72025999414116491051161218712036640953_u128 as u8;
_23 = '\u{74b28}';
_22.0.0 = (-144199373_i32) as u128;
_29 = _12 | _12;
Call(_9 = fn7(_3, _2, _2, _17, _2, _1, _2, _7.3, _7.3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
RET = Adt49 { fld0: _7.0.0 };
_9.0 = 869855573_u32 as u64;
Call(_27.fld0 = fn13(_2, _3, _9, _1, _10, _9.1, _2.3), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_14.fld0.2 = _18 as i8;
_22.3 = _14.fld0.0 - _14.fld0.0;
_20.3 = _6;
_27.fld0 = RET.fld0 * _20.0.0;
_27.fld0 = _9.0 | _2.0.0;
_22.0 = (303476654277541587498429162371835177522_u128, _14.fld0);
_1 = _3;
_2.2 = _22.0.1.3 as f32;
_22.2 = _23;
_22.1.0 = _1 as isize;
_24 = 908583958_u32;
_10 = _9.1 >= _2.0.1;
_14.fld0.3 = _16 as u16;
_22.1 = (_29,);
match _24 {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb4,
4 => bb5,
5 => bb9,
908583958 => bb11,
_ => bb10
}
}
bb9 = {
RET = Adt49 { fld0: _7.0.0 };
_9.0 = 869855573_u32 as u64;
Call(_27.fld0 = fn13(_2, _3, _9, _1, _10, _9.1, _2.3), ReturnTo(bb8), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
_16 = 6234612246678679756_usize + 3853977375397431127_usize;
_26 = _22.0.0 - _22.0.0;
_2.0 = (_7.0.0, _7.0.1);
_22.1.0 = (*_11) as isize;
RET = _27;
_7.3 = _2.3;
_20 = (_7.0, _2.1, _9.1, _2.3);
_22.0.1 = (_22.3, _8, _14.fld0.1, _14.fld0.3);
_10 = _4;
_9 = _20.0;
_18 = -5350_i16;
_9.0 = _18 as u64;
_3 = _4;
_11 = &_20.2;
_2.1 = _22.3 as u8;
_22.3 = _24 as i128;
_10 = !_4;
_32 = Adt40::Variant0 { fld0: _7,fld1: _22,fld2: _14.fld0 };
_9.0 = Field::<((u128, (i128, i8, i8, u16)), (isize,), char, i128)>(Variant(_32, 0), 1).0.0 as u64;
SetDiscriminant(_32, 2);
_11 = &(*_11);
_22.0.1 = (_14.fld0.0, _15, _14.fld0.1, _14.fld0.3);
_17 = _4;
_32 = Adt40::Variant0 { fld0: _20,fld1: _22,fld2: _14.fld0 };
RET = _27;
_30 = Adt43::Variant1 { fld0: _22.1.0 };
_25 = &(*_11);
place!(Field::<((u64, f32), u8, f32, [bool; 2])>(Variant(_32, 0), 0)).0.1 = _18 as f32;
Call(_13 = fn15(Field::<((u64, f32), u8, f32, [bool; 2])>(Variant(_32, 0), 0), Move(_32), _20.2, _20), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_2.0.1 = (*_11);
place!(Field::<((u64, f32), u8, f32, [bool; 2])>(Variant(_13, 0), 0)).0.0 = _27.fld0;
_2 = (_20.0, Field::<((u64, f32), u8, f32, [bool; 2])>(Variant(_13, 0), 0).1, (*_11), Field::<((u64, f32), u8, f32, [bool; 2])>(Variant(_13, 0), 0).3);
_2.0.0 = Field::<((u128, (i128, i8, i8, u16)), (isize,), char, i128)>(Variant(_13, 0), 1).0.0 as u64;
_9.0 = !_27.fld0;
_20.0 = (_9.0, _2.2);
_32 = Move(_13);
place!(Field::<((u128, (i128, i8, i8, u16)), (isize,), char, i128)>(Variant(_32, 0), 1)).1.0 = !_12;
_22.0.1.2 = _14.fld0.1;
_34 = _27.fld0 as isize;
_31 = _22.0.1.3;
match _24 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb10,
5 => bb11,
6 => bb7,
908583958 => bb13,
_ => bb9
}
}
bb13 = {
place!(Field::<((u128, (i128, i8, i8, u16)), (isize,), char, i128)>(Variant(_32, 0), 1)).0.1.0 = _2.0.0 as i128;
_33 = Field::<((u128, (i128, i8, i8, u16)), (isize,), char, i128)>(Variant(_32, 0), 1).0.1.3;
_14.fld0 = Field::<((u128, (i128, i8, i8, u16)), (isize,), char, i128)>(Variant(_32, 0), 1).0.1;
_20.0 = (_9.0, (*_11));
_7.3 = [_10,_17];
_2.0 = _9;
_22.3 = Field::<((u128, (i128, i8, i8, u16)), (isize,), char, i128)>(Variant(_32, 0), 1).0.1.0 >> _14.fld0.3;
_7.0.1 = -_20.0.1;
RET.fld0 = !_20.0.0;
_22.0.1.1 = _22.0.1.2 & _14.fld0.1;
_12 = Field::<isize>(Variant(_30, 1), 0) - _34;
place!(Field::<((u128, (i128, i8, i8, u16)), (isize,), char, i128)>(Variant(_32, 0), 1)).1.0 = _22.1.0;
_38.fld0 = _24;
place!(Field::<((u128, (i128, i8, i8, u16)), (isize,), char, i128)>(Variant(_32, 0), 1)).0.1.0 = _14.fld0.0 ^ _14.fld0.0;
_38 = Adt41 { fld0: _24,fld1: Field::<((u128, (i128, i8, i8, u16)), (isize,), char, i128)>(Variant(_32, 0), 1).0.0 };
_18 = (-13959_i16);
_21 = !_2.0.0;
_37 = Field::<((u128, (i128, i8, i8, u16)), (isize,), char, i128)>(Variant(_32, 0), 1).0.1.3;
_17 = _10 == _1;
_10 = _1;
SetDiscriminant(_32, 0);
match _24 {
0 => bb14,
908583958 => bb16,
_ => bb15
}
}
bb14 = {
_14.fld0.2 = _18 as i8;
_22.3 = _14.fld0.0 - _14.fld0.0;
_20.3 = _6;
_27.fld0 = RET.fld0 * _20.0.0;
_27.fld0 = _9.0 | _2.0.0;
_22.0 = (303476654277541587498429162371835177522_u128, _14.fld0);
_1 = _3;
_2.2 = _22.0.1.3 as f32;
_22.2 = _23;
_22.1.0 = _1 as isize;
_24 = 908583958_u32;
_10 = _9.1 >= _2.0.1;
_14.fld0.3 = _16 as u16;
_22.1 = (_29,);
match _24 {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb4,
4 => bb5,
5 => bb9,
908583958 => bb11,
_ => bb10
}
}
bb15 = {
_16 = 6234612246678679756_usize + 3853977375397431127_usize;
_26 = _22.0.0 - _22.0.0;
_2.0 = (_7.0.0, _7.0.1);
_22.1.0 = (*_11) as isize;
RET = _27;
_7.3 = _2.3;
_20 = (_7.0, _2.1, _9.1, _2.3);
_22.0.1 = (_22.3, _8, _14.fld0.1, _14.fld0.3);
_10 = _4;
_9 = _20.0;
_18 = -5350_i16;
_9.0 = _18 as u64;
_3 = _4;
_11 = &_20.2;
_2.1 = _22.3 as u8;
_22.3 = _24 as i128;
_10 = !_4;
_32 = Adt40::Variant0 { fld0: _7,fld1: _22,fld2: _14.fld0 };
_9.0 = Field::<((u128, (i128, i8, i8, u16)), (isize,), char, i128)>(Variant(_32, 0), 1).0.0 as u64;
SetDiscriminant(_32, 2);
_11 = &(*_11);
_22.0.1 = (_14.fld0.0, _15, _14.fld0.1, _14.fld0.3);
_17 = _4;
_32 = Adt40::Variant0 { fld0: _20,fld1: _22,fld2: _14.fld0 };
RET = _27;
_30 = Adt43::Variant1 { fld0: _22.1.0 };
_25 = &(*_11);
place!(Field::<((u64, f32), u8, f32, [bool; 2])>(Variant(_32, 0), 0)).0.1 = _18 as f32;
Call(_13 = fn15(Field::<((u64, f32), u8, f32, [bool; 2])>(Variant(_32, 0), 0), Move(_32), _20.2, _20), ReturnTo(bb12), UnwindUnreachable())
}
bb16 = {
place!(Field::<((u128, (i128, i8, i8, u16)), (isize,), char, i128)>(Variant(_32, 0), 1)).0.1.3 = (-7015294061919364680_i64) as u16;
place!(Field::<((u128, (i128, i8, i8, u16)), (isize,), char, i128)>(Variant(_32, 0), 1)).0 = (_38.fld1, _22.0.1);
_27 = Adt49 { fld0: RET.fld0 };
_9.0 = Field::<((u128, (i128, i8, i8, u16)), (isize,), char, i128)>(Variant(_32, 0), 1).0.1.0 as u64;
place!(Field::<(i128, i8, i8, u16)>(Variant(_32, 0), 2)).1 = _22.0.1.1;
place!(Field::<((u128, (i128, i8, i8, u16)), (isize,), char, i128)>(Variant(_32, 0), 1)).0 = (_26, _14.fld0);
_9.1 = _2.0.1 + (*_25);
place!(Field::<((u64, f32), u8, f32, [bool; 2])>(Variant(_32, 0), 0)).3 = [_17,_4];
_7 = (_9, _2.1, _20.2, _6);
place!(Field::<(i128, i8, i8, u16)>(Variant(_32, 0), 2)).3 = _22.2 as u16;
_38.fld1 = _22.0.0;
_8 = !_15;
_18 = !23040_i16;
_7.1 = !_20.1;
Goto(bb17)
}
bb17 = {
Call(_42 = dump_var(6_usize, 34_usize, Move(_34), 37_usize, Move(_37), 3_usize, Move(_3), 29_usize, Move(_29)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(6_usize, 24_usize, Move(_24), 16_usize, Move(_16), 22_usize, Move(_22), 26_usize, Move(_26)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_42 = dump_var(6_usize, 23_usize, Move(_23), 10_usize, Move(_10), 43_usize, _43, 43_usize, _43), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: bool,mut _2: ((u64, f32), u8, f32, [bool; 2]),mut _3: ((u64, f32), u8, f32, [bool; 2]),mut _4: bool,mut _5: ((u64, f32), u8, f32, [bool; 2]),mut _6: bool,mut _7: ((u64, f32), u8, f32, [bool; 2]),mut _8: [bool; 2],mut _9: [bool; 2]) -> (u64, f32) {
mir! {
type RET = (u64, f32);
let _10: (i32, char, i8);
let _11: bool;
let _12: bool;
let _13: ((u128, (i128, i8, i8, u16)),);
let _14: i8;
let _15: i64;
let _16: f32;
let _17: ();
let _18: ();
{
RET.0 = 2756129869_u32 as u64;
_7.0 = (_5.0.0, _2.0.1);
_5.0.0 = _7.0.0;
_7.0 = _5.0;
_5.2 = 1089548822322255909_i64 as f32;
RET = _2.0;
Call(RET.1 = fn8(_5.3, _2, _5, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = (RET, _5.1, RET.1, _9);
_5.0.0 = _2.0.0;
_4 = _1 > _1;
_3.0.1 = RET.1;
RET = (_5.0.0, _5.2);
_3.0.1 = -_3.2;
_4 = !_6;
_3.0 = (_2.0.0, _3.2);
_12 = _3.1 != _7.1;
_9 = [_4,_4];
_5.0.0 = _3.0.0 * RET.0;
_4 = !_6;
_7.1 = _3.2 as u8;
_7.0 = (_5.0.0, _3.2);
Call(_5.2 = fn10(_7.0.1, _5.3, _5.3, _5.0.1, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_4 = _6;
RET.0 = _3.0.0;
Call(_13.0.1.1 = fn11(_3, _8, _8), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = (_7.0.0, _3.0.1);
_5 = _3;
_3.2 = -_5.2;
_3 = (_5.0, _7.1, RET.1, _8);
_13.0.1.2 = _5.0.1 as i8;
_13.0.1.0 = -(-126742218510139622703449698809541299573_i128);
_5.0 = (_7.0.0, _3.2);
_13.0.1.2 = _13.0.1.1;
_2.0 = (RET.0, RET.1);
_9 = _3.3;
_13.0.1.0 = (-122423812548742201574933390057111872865_i128);
_2.2 = _2.0.1;
_3.0.0 = _2.0.0;
_2.1 = !_7.1;
RET.0 = 248169577194687292749118733847370657804_u128 as u64;
_3 = (RET, _2.1, _7.0.1, _5.3);
Goto(bb4)
}
bb4 = {
Call(_17 = dump_var(7_usize, 1_usize, Move(_1), 6_usize, Move(_6), 4_usize, Move(_4), 18_usize, _18), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [bool; 2],mut _2: ((u64, f32), u8, f32, [bool; 2]),mut _3: ((u64, f32), u8, f32, [bool; 2]),mut _4: ((u64, f32), u8, f32, [bool; 2])) -> f32 {
mir! {
type RET = f32;
let _5: f32;
let _6: isize;
let _7: isize;
let _8: (isize,);
let _9: *mut [i64; 7];
let _10: u128;
let _11: *mut (i8, i16);
let _12: u32;
let _13: ((u128, (i128, i8, i8, u16)), (isize,), char, i128);
let _14: u16;
let _15: Adt45;
let _16: ();
let _17: ();
{
RET = _3.2;
_4 = (_3.0, _2.1, _3.0.1, _3.3);
_3 = (_4.0, _4.1, _4.2, _1);
_3.1 = _4.1;
_3 = (_4.0, _2.1, _4.2, _4.3);
_2.3 = _1;
_2.0.0 = '\u{ce104}' as u64;
_4.0 = _2.0;
_2.1 = 292792073562824646032202964839900058293_u128 as u8;
_3.1 = (-89940055_i32) as u8;
_3.3 = _1;
_4.2 = -_4.0.1;
_4.3 = _1;
_1 = _4.3;
_2.0.1 = _2.1 as f32;
_4 = (_3.0, _3.1, _2.2, _3.3);
Goto(bb1)
}
bb1 = {
_5 = _3.2 + _2.2;
_4.3 = [false,true];
RET = 612644746838656363_i64 as f32;
_2.1 = 47347_u16 as u8;
_2 = (_4.0, _4.1, _3.0.1, _3.3);
_3.0.1 = -_2.0.1;
_3.3 = [true,true];
_3.2 = -_4.0.1;
_3 = (_4.0, _2.1, _5, _2.3);
_4.3 = [true,false];
_2.0.0 = _3.0.0 & _4.0.0;
_4.3 = _3.3;
_4 = (_2.0, _2.1, _2.0.1, _2.3);
_5 = 752268007_u32 as f32;
_3.0 = (_2.0.0, _2.0.1);
_6 = (-9223372036854775808_isize);
Goto(bb2)
}
bb2 = {
_8 = (_6,);
Call(_2.0 = fn9(_2.3, _2.3, _3, _1, _4.3, _2.2, _3, _2.3, _1, _4.3, _3.3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4.2 = _2.2 * _2.0.1;
_1 = _4.3;
_3.3 = [true,true];
_3.0.0 = !_4.0.0;
_2.3 = [false,true];
_10 = 12880_u16 as u128;
_2 = (_4.0, _3.1, _4.2, _4.3);
_2.0.1 = _2.2 + _4.2;
_8 = (_6,);
RET = -_4.2;
_4.0 = _3.0;
_3 = (_2.0, _2.1, _2.2, _4.3);
_3.3 = [false,true];
_2.0 = _3.0;
_4.0 = _3.0;
_2.0.0 = _4.0.0;
_4.0.0 = _3.0.0;
_7 = !_6;
_3.0.1 = _4.0.1;
_5 = -_3.0.1;
_2.0.1 = _3.0.1 - _2.2;
_13.0.1.3 = 53391_u16 * 4022_u16;
_2.2 = _3.2;
Goto(bb4)
}
bb4 = {
Call(_16 = dump_var(8_usize, 8_usize, Move(_8), 7_usize, Move(_7), 17_usize, _17, 17_usize, _17), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [bool; 2],mut _2: [bool; 2],mut _3: ((u64, f32), u8, f32, [bool; 2]),mut _4: [bool; 2],mut _5: [bool; 2],mut _6: f32,mut _7: ((u64, f32), u8, f32, [bool; 2]),mut _8: [bool; 2],mut _9: [bool; 2],mut _10: [bool; 2],mut _11: [bool; 2]) -> (u64, f32) {
mir! {
type RET = (u64, f32);
let _12: [u32; 5];
let _13: *const u64;
let _14: u64;
let _15: *mut [u32; 5];
let _16: Adt47;
let _17: isize;
let _18: ();
let _19: ();
{
_4 = [false,true];
RET.1 = (-169010060418862683379121960930093019387_i128) as f32;
_4 = [true,true];
_3.2 = -_6;
_7.3 = [false,false];
_1 = _9;
_3.0 = (_7.0.0, _7.2);
_11 = [true,true];
_6 = _3.0.1 - _3.0.1;
_2 = [true,true];
Goto(bb1)
}
bb1 = {
_7.1 = !_3.1;
_3.0.1 = _6 + _3.2;
_3.0 = _7.0;
RET.1 = _6;
_13 = core::ptr::addr_of!(_3.0.0);
_7.3 = [true,false];
_3.0.0 = _7.0.0 << _7.1;
RET = _7.0;
Goto(bb2)
}
bb2 = {
_14 = _3.0.0;
_12 = [422449357_u32,2084987652_u32,331843714_u32,3291608890_u32,3633721511_u32];
RET.1 = -_6;
_4 = _3.3;
_7.2 = 225457313826054965439369794075880438955_u128 as f32;
Goto(bb3)
}
bb3 = {
Call(_18 = dump_var(9_usize, 11_usize, Move(_11), 9_usize, Move(_9), 12_usize, Move(_12), 2_usize, Move(_2)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_18 = dump_var(9_usize, 1_usize, Move(_1), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: f32,mut _2: [bool; 2],mut _3: [bool; 2],mut _4: f32,mut _5: bool) -> f32 {
mir! {
type RET = f32;
let _6: bool;
let _7: i8;
let _8: usize;
let _9: i64;
let _10: i16;
let _11: ();
let _12: ();
{
RET = _1 - _1;
RET = -_4;
_5 = !false;
_5 = !true;
RET = 5458038748377293483_i64 as f32;
_5 = true;
_4 = _1;
_5 = false;
RET = _4;
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(10_usize, 2_usize, Move(_2), 12_usize, _12, 12_usize, _12, 12_usize, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: ((u64, f32), u8, f32, [bool; 2]),mut _2: [bool; 2],mut _3: [bool; 2]) -> i8 {
mir! {
type RET = i8;
let _4: Adt52;
let _5: [bool; 2];
let _6: isize;
let _7: Adt41;
let _8: (bool, i16, i8);
let _9: ([bool; 8], i128, (isize,));
let _10: i128;
let _11: f64;
let _12: (u64, f32);
let _13: isize;
let _14: (bool, i16, i8);
let _15: i8;
let _16: [i16; 7];
let _17: i128;
let _18: ((u64, f32), u8, f32, [bool; 2]);
let _19: i16;
let _20: isize;
let _21: *const u64;
let _22: Adt54;
let _23: isize;
let _24: bool;
let _25: isize;
let _26: Adt47;
let _27: *mut u32;
let _28: (isize,);
let _29: Adt51;
let _30: ();
let _31: ();
{
RET = 28_i8 + 10_i8;
_1.1 = 151_u8 | 12_u8;
_1.1 = (-509025333_i32) as u8;
RET = 102_i8;
_1.0.0 = !4571763804087047191_u64;
RET = !72_i8;
_5 = [false,false];
_3 = [true,true];
_5 = [false,false];
_1.0 = (15321827869940655728_u64, _1.2);
_6 = (-5_isize) ^ 9223372036854775807_isize;
match _1.0.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
15321827869940655728 => bb9,
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
_1.3 = _2;
_7 = Adt41 { fld0: 1983117205_u32,fld1: 104443861149346606110095323653445963821_u128 };
_5 = [true,false];
_7.fld1 = !277625124647029106427050499610485377430_u128;
_7 = Adt41 { fld0: 3829972494_u32,fld1: 139224918469136383729854250229652171101_u128 };
_1.1 = !118_u8;
_1.0.1 = _6 as f32;
_1.0 = (9735749788366052505_u64, _1.2);
_8.2 = _1.2 as i8;
_8 = (true, 15380_i16, RET);
_8 = (true, (-13335_i16), RET);
_7.fld1 = (-1917165972_i32) as u128;
_1.0.0 = 17859612604258462770_u64 | 8205270320582568364_u64;
_8.1 = (-24059_i16) >> RET;
_1.0 = (546366348523642977_u64, _1.2);
_7.fld1 = !79585614642995819086915872355054784866_u128;
_11 = _8.1 as f64;
_9.1 = -(-144944501818949031210644207013154883941_i128);
_1.0.0 = !5410135153925098402_u64;
_2 = [_8.0,_8.0];
_5 = [_8.0,_8.0];
_1.0.0 = _11 as u64;
_1.0.1 = _7.fld1 as f32;
_6 = !(-9223372036854775808_isize);
_7 = Adt41 { fld0: 259906926_u32,fld1: 330797816280489201121173148597061138556_u128 };
Goto(bb10)
}
bb10 = {
_7.fld0 = !1322024291_u32;
_10 = -_9.1;
_11 = (-923430426_i32) as f64;
RET = 6216253546771960994_i64 as i8;
_9.2 = (_6,);
_6 = 16786348387951900285_usize as isize;
_8 = (false, 13935_i16, RET);
_11 = RET as f64;
_7.fld1 = !246417574592780520046662206124192637707_u128;
Goto(bb11)
}
bb11 = {
_3 = [_8.0,_8.0];
_3 = [_8.0,_8.0];
_8.1 = _7.fld1 as i16;
_12.1 = _1.2;
_9.0 = [_8.0,_8.0,_8.0,_8.0,_8.0,_8.0,_8.0,_8.0];
_7.fld0 = _7.fld1 as u32;
_14 = _8;
_12 = (_1.0.0, _1.2);
_14.1 = _8.1;
_9.2 = (_6,);
_18.3 = [_8.0,_8.0];
_12 = _1.0;
Call(_18.1 = core::intrinsics::bswap(_1.1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_20 = -_6;
_16 = [_14.1,_14.1,_8.1,_8.1,_14.1,_14.1,_14.1];
_3 = [_8.0,_8.0];
_14.2 = _8.2 | RET;
_20 = _6 - _6;
_1.1 = (-670352490_i32) as u8;
_19 = !_14.1;
RET = _8.2 * _14.2;
_9.2.0 = -_6;
_1 = (_12, 207_u8, _12.1, _3);
_18.0.0 = (-8247257014397483644_i64) as u64;
_18.3 = [_8.0,_8.0];
_18 = (_12, _1.1, _1.0.1, _2);
_23 = _19 as isize;
_21 = core::ptr::addr_of!(_1.0.0);
_1 = _18;
_14.1 = !_8.1;
_18.0.0 = _1.2 as u64;
_7.fld0 = _18.1 as u32;
_9.2 = (_6,);
RET = _14.2;
_9.2 = (_6,);
_18.0.0 = (*_21) - _1.0.0;
_2 = _5;
Call(_12 = fn12(_8.1, _9, _21, _18.1, _9.2, _1, _18.0.0, _7, _14.0, _21, _7.fld0, _7.fld0, _8.1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_9.0 = [_8.0,_8.0,_8.0,_14.0,_8.0,_8.0,_14.0,_14.0];
_23 = !_6;
_18.0.0 = _14.1 as u64;
_1.2 = -_1.0.1;
_8.0 = _7.fld0 > _7.fld0;
_18.3 = _5;
_6 = _11 as isize;
_16 = [_19,_8.1,_19,_8.1,_8.1,_8.1,_19];
_8.0 = !_14.0;
_7.fld1 = !77663783968852393104542357339995901133_u128;
_25 = _14.1 as isize;
_14.2 = !RET;
_1 = (_12, _18.1, _12.1, _2);
_9.2 = (_23,);
_14.1 = _8.1 | _19;
Call(_20 = core::intrinsics::bswap(_25), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_13 = _25;
_8.2 = _14.2 * _14.2;
_9.0 = [_8.0,_14.0,_8.0,_14.0,_8.0,_8.0,_8.0,_14.0];
_14.1 = RET as i16;
_9.2 = (_25,);
_6 = _20 * _25;
_21 = core::ptr::addr_of!(_18.0.0);
(*_21) = _12.0;
_19 = _7.fld0 as i16;
_14 = _8;
_18.2 = 5_usize as f32;
_8.1 = _19 & _19;
_8 = _14;
(*_21) = _12.0;
_2 = [_14.0,_8.0];
_6 = _13 - _9.2.0;
_18 = (_12, _1.1, _1.2, _2);
_18.0.0 = !_1.0.0;
_9.1 = '\u{655a6}' as i128;
_18 = _1;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(11_usize, 6_usize, Move(_6), 2_usize, Move(_2), 5_usize, Move(_5), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(11_usize, 25_usize, Move(_25), 13_usize, Move(_13), 8_usize, Move(_8), 31_usize, _31), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: i16,mut _2: ([bool; 8], i128, (isize,)),mut _3: *const u64,mut _4: u8,mut _5: (isize,),mut _6: ((u64, f32), u8, f32, [bool; 2]),mut _7: u64,mut _8: Adt41,mut _9: bool,mut _10: *const u64,mut _11: u32,mut _12: u32,mut _13: i16) -> (u64, f32) {
mir! {
type RET = (u64, f32);
let _14: i8;
let _15: isize;
let _16: i16;
let _17: ([bool; 8], i128, (isize,));
let _18: ((u128, (i128, i8, i8, u16)),);
let _19: f64;
let _20: f32;
let _21: ((u128, (i128, i8, i8, u16)), (isize,), char, i128);
let _22: &'static f32;
let _23: u16;
let _24: u8;
let _25: ();
let _26: ();
{
_6.3 = [_9,_9];
_2.1 = (-102775802744026493681264455974109301147_i128) >> _12;
_10 = _3;
_10 = _3;
(*_3) = 29312_u16 as u64;
_8.fld0 = !_12;
RET = ((*_10), _6.0.1);
_3 = _10;
RET.1 = _6.0.1;
_9 = !false;
_1 = _2.1 as i16;
_6.0 = RET;
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
207 => bb5,
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
_6.0.0 = (*_10);
_8 = Adt41 { fld0: _12,fld1: 253253772375027196898467405883640288698_u128 };
Goto(bb6)
}
bb6 = {
_8.fld1 = !219780627100846351709947698089848631056_u128;
_9 = !false;
_8 = Adt41 { fld0: _11,fld1: 107180918280532531879615618074599726037_u128 };
_6.1 = !_4;
_2.0 = [_9,_9,_9,_9,_9,_9,_9,_9];
_17.1 = -_2.1;
_17.2 = (_5.0,);
_14 = 61_i8;
_1 = _13;
_7 = (*_3) >> _12;
_6.0.0 = _6.0.1 as u64;
_11 = !_8.fld0;
_9 = true;
_5 = (_2.2.0,);
RET.0 = _14 as u64;
_18.0.1.0 = _2.1;
_17 = (_2.0, _2.1, _5);
_18.0.0 = _8.fld1 + _8.fld1;
_6.2 = RET.1 * _6.0.1;
_21.0.1 = (_17.1, _14, _14, 5680_u16);
Goto(bb7)
}
bb7 = {
RET = (_6.0.0, _6.0.1);
_18.0.1.3 = _21.0.1.3;
_14 = -_21.0.1.2;
(*_10) = !_7;
_6.1 = _4;
RET.0 = _7 & (*_10);
RET.0 = (*_3);
_10 = core::ptr::addr_of!((*_3));
_22 = &_6.2;
_16 = _13;
_12 = _8.fld0 - _8.fld0;
_2.2.0 = _21.0.1.0 as isize;
_8.fld1 = _4 as u128;
_17.1 = _2.1 ^ _18.0.1.0;
_18.0.1 = (_17.1, _21.0.1.2, _21.0.1.2, _21.0.1.3);
(*_3) = _12 as u64;
_6.0 = ((*_3), (*_22));
_18.0.1.1 = _6.1 as i8;
_7 = !(*_10);
_6.1 = _4;
RET = (_7, (*_22));
_20 = (*_22) + (*_22);
_6.1 = _4;
_18.0.1.3 = _8.fld1 as u16;
Goto(bb8)
}
bb8 = {
Call(_25 = dump_var(12_usize, 4_usize, Move(_4), 17_usize, Move(_17), 7_usize, Move(_7), 18_usize, Move(_18)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_25 = dump_var(12_usize, 11_usize, Move(_11), 9_usize, Move(_9), 26_usize, _26, 26_usize, _26), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: ((u64, f32), u8, f32, [bool; 2]),mut _2: bool,mut _3: (u64, f32),mut _4: bool,mut _5: bool,mut _6: f32,mut _7: [bool; 2]) -> u64 {
mir! {
type RET = u64;
let _8: (bool, i16, i8);
let _9: u32;
let _10: ((isize,), u32);
let _11: (i8, i16);
let _12: i128;
let _13: ((u128, (i128, i8, i8, u16)), (isize,), char, i128);
let _14: isize;
let _15: isize;
let _16: ();
let _17: ();
{
_1.0 = (_3.0, _6);
_4 = !_2;
_1.0 = (_3.0, _3.1);
RET = 165445012769727611152637420914983584220_u128 as u64;
_3.1 = 2335828963021925509611975798508484291_u128 as f32;
RET = _1.0.0 + _3.0;
_1.0 = _3;
_1.0 = (RET, _1.2);
RET = !_1.0.0;
_7 = [_2,_4];
_8.2 = (-12_i8) - 17_i8;
_1.2 = _6;
_5 = _4;
_6 = 89540581866349320793925946427266302612_u128 as f32;
_1.0.1 = _1.2 * _1.2;
_3.1 = _1.2;
_1.0 = (RET, _1.2);
_4 = !_2;
_7 = _1.3;
Call(_8.0 = fn14(_2, _1.2, _1, _5, _5, _5, _5, _4, _3, _5, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = _2 >= _8.0;
_1.0.0 = RET;
_5 = _1.2 < _3.1;
_8.0 = _4 <= _2;
_9 = 3105774862_u32;
_1 = (_3, 116_u8, _3.1, _7);
_10.0 = ((-82_isize),);
_9 = 3492877569_u32 ^ 2410435547_u32;
_3.1 = -_1.0.1;
_10.0 = (9223372036854775807_isize,);
_1.0.1 = _6;
_5 = _4;
RET = !_3.0;
_11 = (_8.2, (-4523_i16));
_1.3 = _7;
_13.0.0 = !288992619045260104214212232964439383494_u128;
_11.1 = !6608_i16;
_7 = [_2,_4];
_12 = 151139259821040071730677522447599823526_i128;
match _1.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
116 => bb10,
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
_3.1 = -_1.2;
_10.1 = !_9;
_3.1 = _1.2;
_13.0.1 = (_12, _11.0, _11.0, 57107_u16);
_3.0 = _13.0.1.3 as u64;
_13.3 = !_12;
_13.1 = (_10.0.0,);
_3.0 = !RET;
_3.0 = !_1.0.0;
_13.0.1.1 = _3.0 as i8;
_13.2 = '\u{73895}';
_3.1 = _13.0.1.0 as f32;
_10 = (_13.1, _9);
_6 = 6_usize as f32;
_13.2 = '\u{4c697}';
Goto(bb11)
}
bb11 = {
_13.0.1.2 = !_11.0;
_10.1 = !_9;
_13.0.0 = 113893313057273688726654081396925279186_u128;
_3.1 = _11.0 as f32;
_1.0 = (_3.0, _1.2);
_14 = _13.1.0 | _13.1.0;
_6 = _1.0.1;
_1.0.0 = _3.0 * RET;
_1.0.1 = 2_usize as f32;
_2 = !_5;
_13.1 = _10.0;
_5 = !_2;
_11.0 = _1.2 as i8;
_4 = _6 <= _1.2;
RET = _13.0.0 as u64;
_3.0 = _1.0.0;
_1.0.0 = !_3.0;
_7 = [_2,_5];
_8.1 = _14 as i16;
_1.0 = _3;
_8.1 = !_11.1;
_6 = RET as f32;
_1.0.1 = -_1.2;
_1.1 = (-1574342957_i32) as u8;
_3.0 = !_1.0.0;
_13.0.1.3 = 19991_u16 & 22753_u16;
match _13.0.0 {
0 => bb12,
113893313057273688726654081396925279186 => bb14,
_ => bb13
}
}
bb12 = {
_3.1 = -_1.2;
_10.1 = !_9;
_3.1 = _1.2;
_13.0.1 = (_12, _11.0, _11.0, 57107_u16);
_3.0 = _13.0.1.3 as u64;
_13.3 = !_12;
_13.1 = (_10.0.0,);
_3.0 = !RET;
_3.0 = !_1.0.0;
_13.0.1.1 = _3.0 as i8;
_13.2 = '\u{73895}';
_3.1 = _13.0.1.0 as f32;
_10 = (_13.1, _9);
_6 = 6_usize as f32;
_13.2 = '\u{4c697}';
Goto(bb11)
}
bb13 = {
_5 = _2 >= _8.0;
_1.0.0 = RET;
_5 = _1.2 < _3.1;
_8.0 = _4 <= _2;
_9 = 3105774862_u32;
_1 = (_3, 116_u8, _3.1, _7);
_10.0 = ((-82_isize),);
_9 = 3492877569_u32 ^ 2410435547_u32;
_3.1 = -_1.0.1;
_10.0 = (9223372036854775807_isize,);
_1.0.1 = _6;
_5 = _4;
RET = !_3.0;
_11 = (_8.2, (-4523_i16));
_1.3 = _7;
_13.0.0 = !288992619045260104214212232964439383494_u128;
_11.1 = !6608_i16;
_7 = [_2,_4];
_12 = 151139259821040071730677522447599823526_i128;
match _1.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
116 => bb10,
_ => bb9
}
}
bb14 = {
_6 = _1.2;
_11.0 = _3.0 as i8;
_8.1 = !_11.1;
_9 = _13.0.0 as u32;
_13.1.0 = _14;
_10 = (_13.1, _9);
_15 = _13.1.0 | _10.0.0;
_3.0 = RET;
_13.1 = _10.0;
_11 = (_8.2, _8.1);
Goto(bb15)
}
bb15 = {
Call(_16 = dump_var(13_usize, 15_usize, Move(_15), 10_usize, Move(_10), 13_usize, Move(_13), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_16 = dump_var(13_usize, 8_usize, Move(_8), 5_usize, Move(_5), 17_usize, _17, 17_usize, _17), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: bool,mut _2: f32,mut _3: ((u64, f32), u8, f32, [bool; 2]),mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: (u64, f32),mut _10: bool,mut _11: ((u64, f32), u8, f32, [bool; 2])) -> bool {
mir! {
type RET = bool;
let _12: f32;
let _13: ();
let _14: ();
{
_11.1 = 91233063511300093086479964482490887184_u128 as u8;
_3.0.1 = _11.1 as f32;
_2 = 3344022057490249211_usize as f32;
_3.1 = !_11.1;
_4 = _1;
_11.3 = _3.3;
_11 = (_9, _3.1, _3.2, _3.3);
_11.3 = [_8,_5];
_9.0 = _3.0.0;
_6 = _1;
_3.2 = _9.1;
_6 = _8 != _7;
_3.0.0 = _9.0;
_11.0.0 = _9.0 + _3.0.0;
RET = !_6;
_11.3 = [_5,_6];
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(14_usize, 10_usize, Move(_10), 5_usize, Move(_5), 1_usize, Move(_1), 14_usize, _14), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: ((u64, f32), u8, f32, [bool; 2]),mut _2: Adt40,mut _3: f32,mut _4: ((u64, f32), u8, f32, [bool; 2])) -> Adt40 {
mir! {
type RET = Adt40;
let _5: ();
let _6: ();
{
place!(Field::<((u64, f32), u8, f32, [bool; 2])>(Variant(_2, 0), 0)).3 = _4.3;
place!(Field::<((u128, (i128, i8, i8, u16)), (isize,), char, i128)>(Variant(_2, 0), 1)).2 = '\u{23b5}';
place!(Field::<((u128, (i128, i8, i8, u16)), (isize,), char, i128)>(Variant(_2, 0), 1)).1 = (9223372036854775807_isize,);
RET = Move(_2);
Goto(bb1)
}
bb1 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: bool,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: [bool; 8],mut _7: bool,mut _8: [bool; 8],mut _9: bool,mut _10: bool,mut _11: bool) -> [bool; 2] {
mir! {
type RET = [bool; 2];
let _12: Adt48;
let _13: isize;
let _14: ();
let _15: ();
{
RET = [_10,_3];
_3 = _1;
_9 = _4 <= _2;
_8 = [_11,_9,_4,_1,_7,_11,_2,_9];
_5 = _7 != _10;
_7 = !_1;
_6 = [_11,_4,_5,_11,_2,_1,_11,_11];
RET = [_3,_1];
_8 = _6;
_8 = [_7,_2,_2,_3,_9,_11,_10,_3];
_10 = _2;
RET = [_11,_2];
_6 = [_3,_7,_1,_11,_11,_7,_4,_2];
_2 = _9 < _5;
_9 = _11;
_11 = _2;
_3 = _2;
_13 = 9223372036854775807_isize;
_5 = !_2;
_2 = !_3;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(16_usize, 2_usize, Move(_2), 1_usize, Move(_1), 11_usize, Move(_11), 7_usize, Move(_7)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_14 = dump_var(16_usize, 8_usize, Move(_8), 9_usize, Move(_9), 15_usize, _15, 15_usize, _15), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: (u64, f32),mut _2: [bool; 8],mut _3: [bool; 2],mut _4: [bool; 8],mut _5: bool,mut _6: ((u64, f32), u8, f32, [bool; 2]),mut _7: ([bool; 8], i128, (isize,)),mut _8: [bool; 2],mut _9: [bool; 2],mut _10: ((u64, f32), u8, f32, [bool; 2])) -> bool {
mir! {
type RET = bool;
let _11: ();
let _12: ();
{
_1.1 = _6.0.1;
_6.2 = -_1.1;
_3 = [_5,_5];
RET = _5 & _5;
_6.3 = [_5,_5];
_10.0 = _1;
_3 = _10.3;
_5 = RET <= RET;
_6 = _10;
_7.0 = [RET,_5,_5,_5,_5,RET,_5,_5];
_1.0 = _6.0.0;
_6.0.0 = _1.0;
_1 = _6.0;
_7.2.0 = (-55_isize) | (-9223372036854775808_isize);
_6.0.1 = _1.1;
Goto(bb1)
}
bb1 = {
Call(_11 = dump_var(17_usize, 5_usize, Move(_5), 3_usize, Move(_3), 7_usize, Move(_7), 12_usize, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: [bool; 8],mut _2: [bool; 2],mut _3: ([bool; 8], i128, (isize,)),mut _4: ([bool; 8], i128, (isize,)),mut _5: bool,mut _6: [bool; 2],mut _7: bool,mut _8: [bool; 2],mut _9: ((u64, f32), u8, f32, [bool; 2]),mut _10: ([bool; 8], i128, (isize,)),mut _11: bool,mut _12: ((u64, f32), u8, f32, [bool; 2])) -> char {
mir! {
type RET = char;
let _13: [u32; 5];
let _14: isize;
let _15: (i32, char, i8);
let _16: i16;
let _17: i32;
let _18: bool;
let _19: ((u128, (i128, i8, i8, u16)), (isize,), char, i128);
let _20: (i128, i8, i8, u16);
let _21: isize;
let _22: Adt51;
let _23: (u128, (i128, i8, i8, u16));
let _24: i128;
let _25: *mut [i64; 7];
let _26: i8;
let _27: *mut [i64; 7];
let _28: char;
let _29: Adt54;
let _30: Adt41;
let _31: [u32; 5];
let _32: ();
let _33: ();
{
_10.2.0 = '\u{103f23}' as isize;
_4.2 = (_3.2.0,);
_10.2.0 = (-121_i8) as isize;
_12.0 = (_9.0.0, _9.0.1);
_9.1 = _12.1;
_3 = (_4.0, _10.1, _4.2);
_9.0 = (_12.0.0, _12.0.1);
RET = '\u{b439c}';
_3 = _4;
_12.0 = _9.0;
_9 = _12;
_10 = _3;
_8 = [_5,_7];
_9.0.1 = (-2486312248499470022_i64) as f32;
_1 = [_5,_5,_5,_11,_11,_5,_7,_7];
_6 = [_7,_11];
Goto(bb1)
}
bb1 = {
_3.2.0 = !_10.2.0;
_7 = _11 >= _11;
_3.0 = [_7,_7,_11,_11,_5,_7,_7,_11];
_10.2 = (_4.2.0,);
_4.2.0 = _12.2 as isize;
_9.2 = _9.0.1 - _12.2;
_9.0.1 = 29383_i16 as f32;
_12.1 = _9.1 & _9.1;
_9.2 = -_9.0.1;
Goto(bb2)
}
bb2 = {
_3 = (_10.0, _4.1, _10.2);
Call(_4.2.0 = core::intrinsics::transmute(_4.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_12.1 = !_9.1;
_5 = !_11;
_5 = _11;
_4.0 = [_5,_5,_7,_5,_5,_5,_5,_11];
_12.2 = _9.0.1 - _9.0.1;
_15.1 = RET;
_16 = (-5761_i16);
_12.0 = _9.0;
_12.3 = [_11,_11];
_9.0.1 = (-5392312905709225287_i64) as f32;
match _16 {
340282366920938463463374607431768205695 => bb5,
_ => bb4
}
}
bb4 = {
_3.2.0 = !_10.2.0;
_7 = _11 >= _11;
_3.0 = [_7,_7,_11,_11,_5,_7,_7,_11];
_10.2 = (_4.2.0,);
_4.2.0 = _12.2 as isize;
_9.2 = _9.0.1 - _12.2;
_9.0.1 = 29383_i16 as f32;
_12.1 = _9.1 & _9.1;
_9.2 = -_9.0.1;
Goto(bb2)
}
bb5 = {
_4 = (_10.0, _3.1, _3.2);
_14 = -_3.2.0;
RET = _15.1;
_5 = _11;
_8 = [_11,_7];
_15.1 = RET;
_17 = !523516883_i32;
_4.2.0 = _11 as isize;
_12.1 = !_9.1;
match _16 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
340282366920938463463374607431768205695 => bb11,
_ => bb10
}
}
bb6 = {
_3.2.0 = !_10.2.0;
_7 = _11 >= _11;
_3.0 = [_7,_7,_11,_11,_5,_7,_7,_11];
_10.2 = (_4.2.0,);
_4.2.0 = _12.2 as isize;
_9.2 = _9.0.1 - _12.2;
_9.0.1 = 29383_i16 as f32;
_12.1 = _9.1 & _9.1;
_9.2 = -_9.0.1;
Goto(bb2)
}
bb7 = {
_12.1 = !_9.1;
_5 = !_11;
_5 = _11;
_4.0 = [_5,_5,_7,_5,_5,_5,_5,_11];
_12.2 = _9.0.1 - _9.0.1;
_15.1 = RET;
_16 = (-5761_i16);
_12.0 = _9.0;
_12.3 = [_11,_11];
_9.0.1 = (-5392312905709225287_i64) as f32;
match _16 {
340282366920938463463374607431768205695 => bb5,
_ => bb4
}
}
bb8 = {
_3 = (_10.0, _4.1, _10.2);
Call(_4.2.0 = core::intrinsics::transmute(_4.0), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_3.2.0 = !_10.2.0;
_7 = _11 >= _11;
_3.0 = [_7,_7,_11,_11,_5,_7,_7,_11];
_10.2 = (_4.2.0,);
_4.2.0 = _12.2 as isize;
_9.2 = _9.0.1 - _12.2;
_9.0.1 = 29383_i16 as f32;
_12.1 = _9.1 & _9.1;
_9.2 = -_9.0.1;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
_12.2 = _9.2;
_19.0.1.0 = -_10.1;
_19.0.0 = _4.2.0 as u128;
_10.2 = _4.2;
_12 = _9;
_19.0.1.2 = 90_i8;
_20.3 = 31386_u16;
_9.1 = _12.1 & _12.1;
_19.1 = (_10.2.0,);
_4 = (_1, _10.1, _19.1);
_12.1 = !_9.1;
_15.2 = _19.0.1.2;
_19.0.1.1 = _19.0.1.2;
_19.0.1.3 = !_20.3;
_19.0.0 = 24553418810666413190058127976189290497_u128 >> _19.1.0;
_19.2 = _15.1;
_13 = [1598954598_u32,2297897873_u32,4293660_u32,1537206621_u32,1697079873_u32];
_19.2 = _15.1;
_12.3 = _9.3;
_15 = (_17, RET, _19.0.1.1);
_19.3 = !_4.1;
_12.1 = 8461420206435614528_usize as u8;
_9.3 = _8;
_19.0.1 = (_19.3, _15.2, _15.2, _20.3);
_20.1 = !_15.2;
_3.2.0 = _19.1.0;
_19.1 = (_10.2.0,);
_19.0.1.3 = _20.3;
_13 = [4132498619_u32,2120553495_u32,1368187023_u32,500424268_u32,1971382680_u32];
_4 = (_3.0, _19.0.1.0, _19.1);
Goto(bb12)
}
bb12 = {
_20.0 = _4.1;
_4 = _3;
_8 = [_5,_5];
_13 = [3995405720_u32,3256583678_u32,1909833256_u32,464373227_u32,3555997949_u32];
_16 = 29893_i16;
_23 = (_19.0.0, _19.0.1);
_12.0 = _9.0;
_19.0.1.0 = _23.1.3 as i128;
_19.0.1 = (_10.1, _23.1.1, _23.1.2, _20.3);
_9.1 = _12.1;
_20.2 = !_19.0.1.2;
Goto(bb13)
}
bb13 = {
_15 = (_17, _19.2, _20.1);
_19.0.1.0 = _10.1 << _19.1.0;
_4.2 = (_3.2.0,);
_20.3 = _23.1.3;
_10 = (_4.0, _19.0.1.0, _4.2);
_23.1 = (_3.1, _20.1, _19.0.1.1, _19.0.1.3);
_1 = _4.0;
_12.0 = (_9.0.0, _9.2);
_4.2.0 = !_10.2.0;
_20 = (_19.0.1.0, _23.1.2, _15.2, _19.0.1.3);
_9.0 = (_12.0.0, _12.0.1);
_12.3 = [_5,_5];
_1 = [_7,_5,_11,_11,_11,_11,_5,_7];
_9.3 = [_7,_7];
_19.0.1.3 = _23.1.3 << _20.0;
_13 = [2159556178_u32,2487598276_u32,3065751483_u32,456376648_u32,3151024377_u32];
_5 = _7;
_10.2 = (_19.1.0,);
_8 = _9.3;
_15.1 = _19.2;
_23 = (_19.0.0, _19.0.1);
_15 = (_17, _19.2, _23.1.1);
_19.0.1.2 = _23.0 as i8;
_9.3 = [_11,_7];
Goto(bb14)
}
bb14 = {
_20 = _23.1;
_18 = _5;
_19.0 = _23;
RET = _19.2;
_9.0 = (_12.0.0, _12.0.1);
_15 = (_17, RET, _23.1.2);
_4.0 = _3.0;
_15.0 = -_17;
_12 = (_9.0, _9.1, _9.0.1, _9.3);
_20.3 = !_19.0.1.3;
_30.fld0 = 3902295819_u32 | 2216016492_u32;
_6 = [_5,_7];
_30 = Adt41 { fld0: 1426785708_u32,fld1: _19.0.0 };
_4.1 = _19.0.1.0;
_12.2 = _9.0.1 - _9.2;
_9.0 = _12.0;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(18_usize, 23_usize, Move(_23), 1_usize, Move(_1), 4_usize, Move(_4), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(18_usize, 13_usize, Move(_13), 14_usize, Move(_14), 16_usize, Move(_16), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(18_usize, 10_usize, Move(_10), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{589ea}'), std::hint::black_box((-15_isize)), std::hint::black_box((-88_i8)), std::hint::black_box(7435_i16), std::hint::black_box((-472870630_i32)), std::hint::black_box(7675076535321132781_i64), std::hint::black_box((-25529925778249855087585504095293304169_i128)), std::hint::black_box(9773957077793597678_usize), std::hint::black_box(96_u8), std::hint::black_box(32550_u16), std::hint::black_box(328278619_u32), std::hint::black_box(9100384109035215234_u64), std::hint::black_box(204224232034591802215192571262808178036_u128));
                
            }
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf("Adt40::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: ((u64, f32), u8, f32, [bool; 2]),
fld1: ((u128, (i128, i8, i8, u16)), (isize,), char, i128),
fld2: (i128, i8, i8, u16),

},
Variant1{
fld0: i16,
fld1: *mut [u32; 5],
fld2: (u64, f32),
fld3: [i64; 7],

},
Variant2{
fld0: i16,
fld1: (i32, char, i8),
fld2: (isize,),

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt41{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt41 {
fld0: u32,
fld1: u128,
}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: bool,
fld1: u16,
fld2: u128,
fld3: (i8, i16),

},
Variant1{
fld0: i32,
fld1: *mut [u32; 5],
fld2: u32,
fld3: u16,
fld4: [bool; 2],

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: [bool; 8],
fld1: char,
fld2: i16,

},
Variant1{
fld0: isize,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: (i128, i8, i8, u16),
}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf("Variant3{".as_ptr() as *const c_char)};
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: i64,
fld1: (u64, f32),
fld2: [i16; 7],
fld3: u16,

},
Variant1{
fld0: Adt40,
fld1: i32,
fld2: [u32; 5],
fld3: [bool; 2],
fld4: u32,

},
Variant2{
fld0: (u128, (i128, i8, i8, u16)),
fld1: *mut u32,
fld2: isize,
fld3: Adt41,

},
Variant3{
fld0: ((u128, (i128, i8, i8, u16)), (isize,), char, i128),
fld1: i8,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: f64,
fld1: *mut [u32; 5],

},
Variant1{
fld0: bool,
fld1: Adt40,
fld2: Adt41,
fld3: usize,

},
Variant2{
fld0: Adt41,
fld1: usize,
fld2: i16,
fld3: (u128, (i128, i8, i8, u16)),

},
Variant3{
fld0: i16,
fld1: [bool; 2],
fld2: f32,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: bool,
fld1: ((isize,), u32),
fld2: i128,
fld3: usize,
fld4: f32,
fld5: u32,

},
Variant1{
fld0: ((isize,), u32),
fld1: Adt44,
fld2: [bool; 8],
fld3: (bool, i16, i8),

},
Variant2{
fld0: (isize,),
fld1: [bool; 8],
fld2: Adt40,
fld3: f64,
fld4: [i16; 7],
fld5: Adt42,
fld6: u64,

},
Variant3{
fld0: (i128, i8, i8, u16),
fld1: ((u128, (i128, i8, i8, u16)), (isize,), char, i128),
fld2: (u64, f32),
fld3: *mut [u32; 5],

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: *const u64,
fld1: (i128, i8, i8, u16),
fld2: u64,
fld3: i32,

},
Variant1{
fld0: (i8, i16),

},
Variant2{
fld0: [i16; 7],
fld1: *const u64,
fld2: u8,

},
Variant3{
fld0: (i8, i16),

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: u64,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: *mut [i64; 7],
fld1: f32,
fld2: ((isize,), u32),
fld3: i8,
fld4: u8,
fld5: Adt42,
fld6: i64,
fld7: [i64; 7],

},
Variant1{
fld0: bool,

},
Variant2{
fld0: [bool; 8],

},
Variant3{
fld0: Adt43,
fld1: ((u128, (i128, i8, i8, u16)),),
fld2: u64,
fld3: *const u64,
fld4: f32,
fld5: [i64; 7],
fld6: *mut [u32; 5],

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: bool,
fld1: (i8, i16),
fld2: (isize,),
fld3: Adt44,
fld4: ([bool; 8], i128, (isize,)),
fld5: (i128, i8, i8, u16),

},
Variant1{
fld0: Adt41,

},
Variant2{
fld0: ((u64, f32), u8, f32, [bool; 2]),
fld1: ((u128, (i128, i8, i8, u16)), (isize,), char, i128),
fld2: (isize,),
fld3: i8,
fld4: *const u64,
fld5: (u128, (i128, i8, i8, u16)),
fld6: ([bool; 8], i128, (isize,)),

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: (u128, (i128, i8, i8, u16)),
fld1: [i16; 7],
fld2: (isize,),
fld3: i8,
fld4: (i32, char, i8),

},
Variant1{
fld0: ((u64, f32), u8, f32, [bool; 2]),
fld1: Adt47,
fld2: (bool, i16, i8),
fld3: usize,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: u128,
fld1: i64,
fld2: isize,
fld3: Adt45,

},
Variant1{
fld0: usize,
fld1: *const u64,
fld2: u16,
fld3: ((u64, f32), u8, f32, [bool; 2]),
fld4: i16,
fld5: Adt46,
fld6: (isize,),

},
Variant2{
fld0: *mut u32,
fld1: ([bool; 8], i128, (isize,)),

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt53,

},
Variant1{
fld0: char,

},
Variant2{
fld0: Adt53,
fld1: Adt43,
fld2: u16,
fld3: i8,

},
Variant3{
fld0: u128,
fld1: Adt48,
fld2: (u128, (i128, i8, i8, u16)),
fld3: Adt42,
fld4: Adt44,
fld5: ((isize,), u32),

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: Adt43,
fld1: *const u64,
fld2: Adt40,
fld3: i128,
}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: i128,
fld1: Adt40,
fld2: Adt49,
fld3: Adt51,

},
Variant1{
fld0: Adt44,
fld1: u16,
fld2: (u128, (i128, i8, i8, u16)),
fld3: i128,
fld4: Adt41,

},
Variant2{
fld0: ((isize,), u32),
fld1: char,
fld2: u128,
fld3: i8,
fld4: Adt47,
fld5: Adt44,
fld6: i64,
fld7: Adt42,

},
Variant3{
fld0: Adt49,
fld1: u16,
fld2: *const u64,
fld3: i8,
fld4: f32,
fld5: *mut [u32; 5],

}}

