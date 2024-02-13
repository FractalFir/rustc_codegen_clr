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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> i128 {
mir! {
type RET = i128;
let _15: [i64; 2];
let _16: i8;
let _17: f32;
let _18: *const f32;
let _19: Adt46;
let _20: u16;
let _21: f64;
let _22: Adt49;
let _23: *const (i32,);
let _24: isize;
let _25: ([i16; 1], i32);
let _26: i8;
let _27: (i32, i64, u128, i64, f32);
let _28: Adt49;
let _29: [i16; 1];
let _30: ([i32; 1], [char; 1]);
let _31: ([i32; 1], [char; 1]);
let _32: isize;
let _33: u16;
let _34: ();
let _35: ();
{
_4 = !119_i8;
Goto(bb1)
}
bb1 = {
RET = 55328_u16 as i128;
_15 = [(-8917212830315479469_i64),653258118424174913_i64];
RET = 3584540359955456316_u64 as i128;
_2 = '\u{2a92}';
_16 = (-28753_i16) as i8;
_10 = !255_u8;
_3 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_13 = !9036636384959981161_u64;
_14 = _4 as u128;
Goto(bb2)
}
bb2 = {
_5 = 1904_i16 >> _3;
_1 = !true;
_17 = 353136643_i32 as f32;
_11 = 34163_u16;
RET = 139708253440881512949191485117737064051_i128;
Goto(bb3)
}
bb3 = {
_16 = _17 as i8;
_16 = _4;
_8 = !RET;
_11 = _17 as u16;
_18 = core::ptr::addr_of!(_17);
_5 = _2 as i16;
_10 = 77_u8;
_2 = '\u{32346}';
_17 = RET as f32;
_20 = !_11;
Goto(bb5)
/* 
match _10 {
0 => bb1,
77 => bb5,
_ => bb4
}*/
}
/*
bb4 = {
_5 = 1904_i16 >> _3;
_1 = !true;
_17 = 353136643_i32 as f32;
_11 = 34163_u16;
RET = 139708253440881512949191485117737064051_i128;
Goto(bb3)
} */
bb5 = {
_17 = _11 as f32;
_21 = _17 as f64;
_1 = (*_18) > (*_18);
_22.fld3 = _21 * _21;
(*_18) = RET as f32;
(*_18) = _3 as f32;
_9 = 0_usize - 13689646433117539589_usize;
_18 = core::ptr::addr_of!(_17);
_17 = _13 as f32;
_22.fld0 = !_8;
_9 = _1 as usize;
_22.fld3 = _21 * _21;
_14 = !248124306340644285545211161932981134320_u128;
_24 = !_3;
_25.1 = -(-1047728042_i32);
Call(_25.0 = fn1(_10, _15, _24, _24, _3, _3, _3, _21, _8, _11, _11, _3, _10, _3), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_22.fld1 = _9 - _9;
_17 = 2703716109_u32 as f32;
_22.fld1 = _9 << _10;
_3 = _10 as isize;
_6 = -_25.1;
_21 = _22.fld3;
_14 = _5 as u128;
_4 = -_16;
_7 = 8599598274528185267_i64;
_16 = 3259580396_u32 as i8;
_22.fld0 = RET;
_2 = '\u{8e3ae}';
_1 = _22.fld0 <= RET;
_12 = !1124100861_u32;
_22.fld1 = _9 & _9;
_25.1 = _6;
RET = _6 as i128;
_22.fld1 = !_9;
_25.0 = [_5];
_5 = (-6982_i16) << _24;
_26 = _16;
_18 = core::ptr::addr_of!((*_18));
_1 = _8 < RET;
_27.3 = _16 as i64;
_28.fld4 = [_5,_5];
_28.fld3 = -_21;
_27 = (_6, _7, _14, _7, (*_18));
RET = !_22.fld0;
Goto(bb7)
}
bb7 = {
RET = _13 as i128;
_17 = _27.4;
RET = _8;
_28.fld2 = core::ptr::addr_of_mut!(_27.2);
_27 = (_6, _7, _14, _7, (*_18));
_4 = _16 | _26;
_17 = _10 as f32;
_24 = -_3;
_26 = -_4;
_29 = _25.0;
_6 = !_25.1;
(*_18) = -_27.4;
RET = -_8;
_1 = true ^ true;
_25.0 = [_5];
_28.fld1 = _20 as usize;
Goto(bb14)
/*
match _7 {

0 => bb4,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,

8599598274528185267 => bb14,
_ => bb13
}
 */
}
/*
bb8 = {
_22.fld1 = _9 - _9;
_17 = 2703716109_u32 as f32;
_22.fld1 = _9 << _10;
_3 = _10 as isize;
_6 = -_25.1;
_21 = _22.fld3;
_14 = _5 as u128;
_4 = -_16;
_7 = 8599598274528185267_i64;
_16 = 3259580396_u32 as i8;
_22.fld0 = RET;
_2 = '\u{8e3ae}';
_1 = _22.fld0 <= RET;
_12 = !1124100861_u32;
_22.fld1 = _9 & _9;
_25.1 = _6;
RET = _6 as i128;
_22.fld1 = !_9;
_25.0 = [_5];
_5 = (-6982_i16) << _24;
_26 = _16;
_18 = core::ptr::addr_of!((*_18));
_1 = _8 < RET;
_27.3 = _16 as i64;
_28.fld4 = [_5,_5];
_28.fld3 = -_21;
_27 = (_6, _7, _14, _7, (*_18));
RET = !_22.fld0;
Goto(bb7)
}
bb9 = {
_17 = _11 as f32;
_21 = _17 as f64;
_1 = (*_18) > (*_18);
_22.fld3 = _21 * _21;
(*_18) = RET as f32;
(*_18) = _3 as f32;
_9 = 0_usize - 13689646433117539589_usize;
_18 = core::ptr::addr_of!(_17);
_17 = _13 as f32;
_22.fld0 = !_8;
_9 = _1 as usize;
_22.fld3 = _21 * _21;
_14 = !248124306340644285545211161932981134320_u128;
_24 = !_3;
_25.1 = -(-1047728042_i32);
Call(_25.0 = fn1(_10, _15, _24, _24, _3, _3, _3, _21, _8, _11, _11, _3, _10, _3), ReturnTo(bb6), UnwindUnreachable())
}

bb10 = {
_5 = 1904_i16 >> _3;
_1 = !true;
_17 = 353136643_i32 as f32;
_11 = 34163_u16;
RET = 139708253440881512949191485117737064051_i128;
Goto(bb3)
}

bb11 = {
_16 = _17 as i8;
_16 = _4;
_8 = !RET;
_11 = _17 as u16;
_18 = core::ptr::addr_of!(_17);
_5 = _2 as i16;
_10 = 77_u8;
_2 = '\u{32346}';
_17 = RET as f32;
_20 = !_11;
match _10 {
0 => bb1,
77 => bb5,
_ => bb4
}
}
bb12 = {
_5 = 1904_i16 >> _3;
_1 = !true;
_17 = 353136643_i32 as f32;
_11 = 34163_u16;
RET = 139708253440881512949191485117737064051_i128;
Goto(bb3)
}
bb13 = {
RET = 55328_u16 as i128;
_15 = [(-8917212830315479469_i64),653258118424174913_i64];
RET = 3584540359955456316_u64 as i128;
_2 = '\u{2a92}';
_16 = (-28753_i16) as i8;
_10 = !255_u8;
_3 = (-9223372036854775808_isize) & (-9223372036854775808_isize);
_13 = !9036636384959981161_u64;
_14 = _4 as u128;
Goto(bb2)
}
 */
bb14 = {
_22 = Adt49 { fld0: _8,fld1: _9,fld2: _28.fld2,fld3: _21,fld4: _28.fld4 };
_13 = _20 as u64;
_28.fld2 = core::ptr::addr_of_mut!(_27.2);
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(0_usize, 8_usize, Move(_8), 11_usize, Move(_11), 12_usize, Move(_12), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(0_usize, 25_usize, Move(_25), 10_usize, Move(_10), 20_usize, Move(_20), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(0_usize, 29_usize, Move(_29), 4_usize, Move(_4), 35_usize, _35, 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(0_usize, 9_usize, Move(_9), 17_usize, Move(_17), 35_usize, _35, 35_usize, _35), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: u8,mut _2: [i64; 2],mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: f64,mut _9: i128,mut _10: u16,mut _11: u16,mut _12: isize,mut _13: u8,mut _14: isize) -> [i16; 1] {
mir! {
type RET = [i16; 1];
let _15: u32;
let _16: Adt45;
let _17: [char; 1];
let _18: Adt50;
let _19: [i16; 1];
let _20: [i64; 3];
let _21: f64;
let _22: u16;
let _23: u32;
let _24: (i32,);
let _25: i32;
let _26: *const i32;
let _27: i32;
let _28: bool;
let _29: [i64; 2];
let _30: u32;
let _31: f64;
let _32: isize;
let _33: char;
let _34: i16;
let _35: [i16; 7];
let _36: Adt49;
let _37: *const u8;
let _38: bool;
let _39: char;
let _40: *mut &'static u8;
let _41: [i16; 7];
let _42: Adt49;
let _43: char;
let _44: [i16; 1];
let _45: char;
let _46: char;
let _47: i32;
let _48: isize;
let _49: Adt58;
let _50: Adt45;
let _51: [i64; 3];
let _52: u64;
let _53: ();
let _54: ();
{
_9 = 73915990660463666582019907914137949877_i128 ^ 167627681930686791013002899178882327461_i128;
_2 = [1703765532051263521_i64,(-437433979681092327_i64)];
_5 = -_14;
_6 = 2588117978385807792_u64 as isize;
_12 = -_3;
_2 = [(-3991907196461028578_i64),(-6706843554936584327_i64)];
_10 = _9 as u16;
_12 = !_14;
Goto(bb1)
}
bb1 = {
_11 = !_10;
_6 = 14365814195506001479_u64 as isize;
_1 = 3419408639_u32 as u8;
_15 = !274123030_u32;
_9 = (-111365343419334990494733076332043163356_i128);
_7 = _12 & _12;
_13 = (-5612_i16) as u8;
_17 = ['\u{cc807}'];
_8 = _5 as f64;
_10 = 39379578094686619289757344386093721268_u128 as u16;
_8 = 8877462599929292884_u64 as f64;
_12 = _15 as isize;
_3 = -_7;
_14 = _4;
_10 = _11 * _11;
_19 = [(-26899_i16)];
_13 = _1;
_7 = _3;
_18 = Adt50::Variant1 { fld0: _15 };
_1 = _12 as u8;
RET = [(-2252_i16)];
match _9 {
0 => bb2,
1 => bb3,
2 => bb4,
228917023501603472968641531099725048100 => bb6,
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
SetDiscriminant(_18, 1);
_13 = _1;
_12 = !_7;
_9 = 118668311882454024514383761955505638244_i128;
_5 = 5207804209087930319_u64 as isize;
_19 = [(-6966_i16)];
RET = [(-11742_i16)];
_21 = _8;
_23 = _15;
_11 = (-11294_i16) as u16;
_9 = !35693871813859466261288826024318172157_i128;
_11 = _10;
_20 = [(-171096620861775318_i64),(-723240223308379630_i64),(-1258784535710767146_i64)];
_9 = (-67_i8) as i128;
_1 = !_13;
_22 = _11 + _11;
_2 = [(-5931916696634050058_i64),(-3696280900013923469_i64)];
_13 = _1 & _1;
_15 = _23 - _23;
_8 = _21;
_25 = (-695596604_i32) - (-2103403471_i32);
_19 = [20363_i16];
_8 = _21;
_14 = _3;
RET = [(-11486_i16)];
Call(RET = fn2(_3, _12, _11, _7, _3, _3, _4, _23), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_4 = -_7;
_15 = '\u{6d5d0}' as u32;
_21 = _8 - _8;
RET = _19;
_28 = false;
_29 = _2;
_8 = _13 as f64;
_20 = [(-645507040509490867_i64),(-1260611800634503849_i64),3636818993984421948_i64];
_9 = _13 as i128;
_8 = _21;
place!(Field::<u32>(Variant(_18, 1), 0)) = _15;
_24.0 = _25 << _12;
place!(Field::<u32>(Variant(_18, 1), 0)) = _15 ^ _23;
_18 = Adt50::Variant1 { fld0: _15 };
SetDiscriminant(_18, 1);
_20 = [(-5797654151843560418_i64),(-8100164354735005209_i64),(-2505840405202880254_i64)];
_19 = [11180_i16];
_7 = 6960317303150793857_i64 as isize;
_30 = _15;
_28 = false;
Goto(bb8)
}
bb8 = {
_25 = _10 as i32;
_5 = !_4;
RET = _19;
_15 = !_23;
_12 = _4 - _5;
_26 = core::ptr::addr_of!(_25);
_11 = !_22;
Goto(bb9)
}
bb9 = {
_33 = '\u{b148c}';
_28 = false;
Call(_9 = core::intrinsics::bswap(14232179924881147362093896812504466494_i128), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_17 = [_33];
(*_26) = _8 as i32;
_6 = _12 + _3;
_1 = _13;
_30 = _11 as u32;
place!(Field::<u32>(Variant(_18, 1), 0)) = _9 as u32;
_34 = 7595804259582211228_usize as i16;
_12 = -_6;
_4 = -_3;
_31 = -_8;
_29 = _2;
_19 = [_34];
_27 = _24.0 + _25;
_8 = _31 - _31;
_15 = _30 + Field::<u32>(Variant(_18, 1), 0);
_1 = _13;
_24 = (_27,);
_36.fld0 = -_9;
_36.fld3 = -_8;
Goto(bb11)
}
bb11 = {
SetDiscriminant(_18, 1);
_32 = 101123352945633478481453569926183646148_u128 as isize;
_13 = _1;
_4 = 171497829970019257333194645986782782453_u128 as isize;
_21 = 205967468279976229257007179374869480573_u128 as f64;
_36.fld1 = _15 as usize;
Goto(bb12)
}
bb12 = {
_37 = core::ptr::addr_of!(_13);
_11 = !_22;
_36.fld1 = 39440847852647928_usize + 17035764950953256313_usize;
_6 = _24.0 as isize;
_36.fld4 = [_34,_34];
_28 = true;
_35 = [_34,_34,_34,_34,_34,_34,_34];
_30 = _8 as u32;
_13 = _1;
_33 = '\u{e83ff}';
_15 = _36.fld1 as u32;
RET = _19;
(*_26) = -_24.0;
_36.fld0 = _12 as i128;
_42.fld0 = _36.fld0;
_22 = _10;
_9 = !_36.fld0;
_37 = core::ptr::addr_of!((*_37));
Goto(bb13)
}
bb13 = {
_6 = _5;
_38 = !_28;
_42.fld3 = -_8;
Goto(bb14)
}
bb14 = {
_11 = !_22;
_8 = _25 as f64;
_44 = [_34];
(*_26) = _27 * _27;
_37 = core::ptr::addr_of!((*_37));
_30 = !_15;
_30 = _15 & _15;
RET = _44;
_45 = _33;
RET = _19;
_41 = _35;
_48 = -_14;
_42.fld3 = -_8;
(*_26) = _27 - _27;
_3 = _48 & _12;
_39 = _45;
_42.fld3 = -_8;
_5 = _3 << _7;
_7 = _3;
_10 = _11 ^ _11;
_11 = _22 - _10;
Goto(bb15)
}
bb15 = {
Call(_53 = dump_var(1_usize, 10_usize, Move(_10), 23_usize, Move(_23), 28_usize, Move(_28), 32_usize, Move(_32)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_53 = dump_var(1_usize, 7_usize, Move(_7), 27_usize, Move(_27), 24_usize, Move(_24), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_53 = dump_var(1_usize, 15_usize, Move(_15), 3_usize, Move(_3), 30_usize, Move(_30), 11_usize, Move(_11)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_53 = dump_var(1_usize, 39_usize, Move(_39), 38_usize, Move(_38), 48_usize, Move(_48), 14_usize, Move(_14)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_53 = dump_var(1_usize, 33_usize, Move(_33), 54_usize, _54, 54_usize, _54, 54_usize, _54), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_53 = dump_var(1_usize, 9_usize, Move(_9), 54_usize, _54, 54_usize, _54, 54_usize, _54), ReturnTo(bb21), UnwindUnreachable())
}

bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: isize,mut _2: isize,mut _3: u16,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: u32) -> [i16; 1] {
mir! {
type RET = [i16; 1];
let _9: bool;
let _10: u16;
let _11: f64;
let _12: f64;
let _13: i32;
let _14: Adt42;
let _15: (i32,);
let _16: *const f32;
let _17: i64;
let _18: isize;
let _19: Adt54;
let _20: i32;
let _21: f64;
let _22: i32;
let _23: [isize; 3];
let _24: isize;
let _25: ();
let _26: ();
{
_2 = -_5;
_1 = _6;
_9 = _4 >= _1;
_7 = 32258_i16 as isize;
_6 = '\u{ee196}' as isize;
RET = [(-6761_i16)];
_3 = !13756_u16;
_3 = !28213_u16;
RET = [2057_i16];
_12 = _3 as f64;
RET = [(-5438_i16)];
RET = [(-2932_i16)];
_10 = !_3;
_13 = 1902101213_i32;
_12 = _2 as f64;
_11 = 23928_i16 as f64;
_10 = 25617149591689177832219757808023380496_u128 as u16;
_1 = _6;
_14.fld0 = 15372501865941247950_u64 >> _5;
_6 = _7 << _2;
_2 = _7 | _1;
_15.0 = 159_u8 as i32;
_2 = _6;
_7 = _6 + _6;
_15 = (_13,);
Goto(bb1)
}
bb1 = {
_12 = _11 * _11;
_15.0 = _13 & _13;
_11 = _10 as f64;
_15.0 = _13 ^ _13;
_15 = (_13,);
_3 = _10 | _10;
_12 = _11 - _11;
_11 = -_12;
_6 = _7 ^ _5;
_4 = '\u{5146}' as isize;
_15 = (_13,);
_10 = _3 - _3;
_13 = _15.0 ^ _15.0;
_4 = !_5;
match _15.0 {
0 => bb2,
1 => bb3,
2 => bb4,
1902101213 => bb6,
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
_14.fld0 = 14257594010412245572_u64 & 8494832118885541295_u64;
_1 = _2 + _7;
_2 = 120456117963250564359753693176435799485_u128 as isize;
_12 = 15865_i16 as f64;
_3 = 9054627424091552194_i64 as u16;
_8 = 202_u8 as u32;
_18 = 303567514306849263572565790269931172594_u128 as isize;
_17 = (-5887611616011952882_i64) | 7346325530194368422_i64;
_2 = _1;
_9 = true;
_10 = '\u{3cafc}' as u16;
_15 = (_13,);
_1 = !_2;
_2 = !_1;
_9 = !true;
RET = [(-6479_i16)];
Call(_16 = fn3(_15, _7, _5, _7, Move(_14), _1, _1, _5, _1, _7, _11, _18), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_4 = _7;
_6 = -_4;
_14.fld0 = !10272959331717986061_u64;
_14 = Adt42 { fld0: 6669021334344256376_u64 };
_13 = _15.0;
_6 = 4570816622038087923_usize as isize;
_10 = _3 >> _2;
_9 = true | false;
_11 = -_12;
_8 = !3911357017_u32;
_5 = -_18;
_9 = !false;
_15 = (_13,);
Call(_16 = fn17(_4, _4, _4, _4, _6, _6, _15.0, _2, _2, _1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_17 = 2366165192176362884_i64;
_17 = !5853598157018728874_i64;
_9 = true;
_7 = _1 | _2;
_18 = -_6;
_15.0 = '\u{f1990}' as i32;
_3 = _8 as u16;
_4 = _9 as isize;
_8 = 1532072713_u32 * 1105696891_u32;
_20 = (-48201366736137742317792895828155606413_i128) as i32;
_18 = 17447_i16 as isize;
Call(RET = core::intrinsics::transmute(_10), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_10 = '\u{277c7}' as u16;
_13 = _20;
_5 = !_1;
_20 = _14.fld0 as i32;
_6 = _5 ^ _2;
_14 = Adt42 { fld0: 12135864591147248535_u64 };
_22 = _8 as i32;
_11 = -_12;
_21 = _1 as f64;
_3 = _6 as u16;
_21 = _3 as f64;
Goto(bb10)
}
bb10 = {
Call(_25 = dump_var(2_usize, 2_usize, Move(_2), 6_usize, Move(_6), 17_usize, Move(_17), 13_usize, Move(_13)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_25 = dump_var(2_usize, 18_usize, Move(_18), 9_usize, Move(_9), 5_usize, Move(_5), 15_usize, Move(_15)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: (i32,),mut _2: isize,mut _3: isize,mut _4: isize,mut _5: Adt42,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: f64,mut _12: isize) -> *const f32 {
mir! {
type RET = *const f32;
let _13: [isize; 3];
let _14: (i32,);
let _15: *mut i64;
let _16: u128;
let _17: *mut u128;
let _18: *const u8;
let _19: u64;
let _20: [i64; 3];
let _21: [i16; 1];
let _22: f32;
let _23: Adt46;
let _24: (char,);
let _25: (char,);
let _26: [isize; 3];
let _27: [i64; 2];
let _28: [i32; 1];
let _29: i128;
let _30: i64;
let _31: ([i16; 1], i32);
let _32: f64;
let _33: isize;
let _34: usize;
let _35: [char; 1];
let _36: Adt53;
let _37: [char; 1];
let _38: [isize; 3];
let _39: ();
let _40: ();
{
_11 = 53302_u16 as f64;
_1.0 = -1684399811_i32;
Goto(bb1)
}
bb1 = {
_7 = -_9;
_13 = [_2,_10,_7];
_14.0 = -_1.0;
_16 = !215169334525818907170922916845531440643_u128;
_17 = core::ptr::addr_of_mut!(_16);
_6 = 34_i8 as isize;
(*_17) = 108879978294487766557791049348442346658_u128 + 85583995932111437259917718920072744593_u128;
_9 = -_10;
Call(_9 = core::intrinsics::bswap(_8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_13 = [_2,_9,_9];
_17 = core::ptr::addr_of_mut!((*_17));
_12 = 2890809997949254777_i64 as isize;
_14.0 = _1.0 * _1.0;
_7 = -_10;
_1 = _14;
_5.fld0 = 5374379060514047138_u64 - 11483252664394380954_u64;
_5.fld0 = !13478548805769145024_u64;
_19 = _11 as u64;
_5 = Adt42 { fld0: _19 };
_12 = _2 >> (*_17);
_2 = _7;
_14.0 = _11 as i32;
_16 = !212297095592977519710902252053943652212_u128;
_14.0 = (-60_i8) as i32;
_7 = _4;
_6 = (-161471608030554798697182040662807117062_i128) as isize;
_11 = 2385143552_u32 as f64;
_20 = [(-4937300625617556528_i64),2979256869360733554_i64,(-5555319636892553552_i64)];
_2 = !_10;
Goto(bb3)
}
bb3 = {
_20 = [(-65968899601662044_i64),5869768314544432675_i64,6971782998179164148_i64];
_13 = [_8,_9,_10];
_8 = true as isize;
_17 = core::ptr::addr_of_mut!(_16);
Call(_21 = fn4(_7, _7, _2, _12, _10, _7, _4, _9, _7, _9, _2, _12, _13, _16), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_13 = [_2,_10,_2];
_10 = _9 + _12;
_1.0 = _14.0;
RET = core::ptr::addr_of!(_22);
(*RET) = 36_u8 as f32;
_5 = Adt42 { fld0: _19 };
_8 = -_7;
_21 = [(-19402_i16)];
RET = core::ptr::addr_of!(_22);
Call((*RET) = fn13(_21, _10, _10, _10, _10, _13, _9, _13, _9, _11, _4, _9, _9, _9), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_8 = -_12;
_22 = (-2279_i16) as f32;
_1.0 = _14.0;
_1.0 = _14.0;
_24.0 = '\u{5aa60}';
Goto(bb6)
}
bb6 = {
_1 = _14;
(*_17) = 68924800105135388430268893338090978688_u128 ^ 270947458775518477461458323951167071677_u128;
_25 = _24;
(*RET) = (-8426137651278042423_i64) as f32;
(*RET) = 48462223_u32 as f32;
_1 = (_14.0,);
_19 = _5.fld0;
_24.0 = _25.0;
_14.0 = true as i32;
_27 = [(-893130227549368588_i64),3527720730211159840_i64];
_20 = [(-2774271294794639985_i64),8404541931841430472_i64,5471399347447030416_i64];
_5 = Adt42 { fld0: _19 };
_4 = (*RET) as isize;
Goto(bb7)
}
bb7 = {
_16 = _10 as u128;
_28 = [_1.0];
_4 = _12;
_4 = _10 >> (*_17);
_17 = core::ptr::addr_of_mut!((*_17));
_22 = _11 as f32;
_24 = (_25.0,);
_19 = !_5.fld0;
_24 = (_25.0,);
(*RET) = _19 as f32;
_11 = _14.0 as f64;
_32 = -_11;
(*RET) = _14.0 as f32;
_1 = _14;
_24 = (_25.0,);
RET = core::ptr::addr_of!(_22);
Goto(bb8)
}
bb8 = {
(*RET) = (-88546828988061177346002822228555512713_i128) as f32;
(*RET) = (-10664_i16) as f32;
_25.0 = _24.0;
Goto(bb9)
}
bb9 = {
_5 = Adt42 { fld0: _19 };
_24 = _25;
_30 = (-967919139697509262_i64);
_31.0 = [(-24871_i16)];
match _30 {
0 => bb8,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
340282366920938463462406688292070702194 => bb17,
_ => bb16
}
}
bb10 = {
(*RET) = (-88546828988061177346002822228555512713_i128) as f32;
(*RET) = (-10664_i16) as f32;
_25.0 = _24.0;
Goto(bb9)
}
bb11 = {
_16 = _10 as u128;
_28 = [_1.0];
_4 = _12;
_4 = _10 >> (*_17);
_17 = core::ptr::addr_of_mut!((*_17));
_22 = _11 as f32;
_24 = (_25.0,);
_19 = !_5.fld0;
_24 = (_25.0,);
(*RET) = _19 as f32;
_11 = _14.0 as f64;
_32 = -_11;
(*RET) = _14.0 as f32;
_1 = _14;
_24 = (_25.0,);
RET = core::ptr::addr_of!(_22);
Goto(bb8)
}
bb12 = {
_7 = -_9;
_13 = [_2,_10,_7];
_14.0 = -_1.0;
_16 = !215169334525818907170922916845531440643_u128;
_17 = core::ptr::addr_of_mut!(_16);
_6 = 34_i8 as isize;
(*_17) = 108879978294487766557791049348442346658_u128 + 85583995932111437259917718920072744593_u128;
_9 = -_10;
Call(_9 = core::intrinsics::bswap(_8), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_8 = -_12;
_22 = (-2279_i16) as f32;
_1.0 = _14.0;
_1.0 = _14.0;
_24.0 = '\u{5aa60}';
Goto(bb6)
}
bb14 = {
_13 = [_2,_10,_2];
_10 = _9 + _12;
_1.0 = _14.0;
RET = core::ptr::addr_of!(_22);
(*RET) = 36_u8 as f32;
_5 = Adt42 { fld0: _19 };
_8 = -_7;
_21 = [(-19402_i16)];
RET = core::ptr::addr_of!(_22);
Call((*RET) = fn13(_21, _10, _10, _10, _10, _13, _9, _13, _9, _11, _4, _9, _9, _9), ReturnTo(bb5), UnwindUnreachable())
}
bb15 = {
_20 = [(-65968899601662044_i64),5869768314544432675_i64,6971782998179164148_i64];
_13 = [_8,_9,_10];
_8 = true as isize;
_17 = core::ptr::addr_of_mut!(_16);
Call(_21 = fn4(_7, _7, _2, _12, _10, _7, _4, _9, _7, _9, _2, _12, _13, _16), ReturnTo(bb4), UnwindUnreachable())
}
bb16 = {
_13 = [_2,_9,_9];
_17 = core::ptr::addr_of_mut!((*_17));
_12 = 2890809997949254777_i64 as isize;
_14.0 = _1.0 * _1.0;
_7 = -_10;
_1 = _14;
_5.fld0 = 5374379060514047138_u64 - 11483252664394380954_u64;
_5.fld0 = !13478548805769145024_u64;
_19 = _11 as u64;
_5 = Adt42 { fld0: _19 };
_12 = _2 >> (*_17);
_2 = _7;
_14.0 = _11 as i32;
_16 = !212297095592977519710902252053943652212_u128;
_14.0 = (-60_i8) as i32;
_7 = _4;
_6 = (-161471608030554798697182040662807117062_i128) as isize;
_11 = 2385143552_u32 as f64;
_20 = [(-4937300625617556528_i64),2979256869360733554_i64,(-5555319636892553552_i64)];
_2 = !_10;
Goto(bb3)
}
bb17 = {
_31 = (_21, _14.0);
Goto(bb18)
}
bb18 = {
Call(_39 = dump_var(3_usize, 31_usize, Move(_31), 6_usize, Move(_6), 9_usize, Move(_9), 13_usize, Move(_13)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_39 = dump_var(3_usize, 8_usize, Move(_8), 14_usize, Move(_14), 7_usize, Move(_7), 16_usize, Move(_16)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_39 = dump_var(3_usize, 2_usize, Move(_2), 27_usize, Move(_27), 21_usize, Move(_21), 40_usize, _40), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: [isize; 3],mut _14: u128) -> [i16; 1] {
mir! {
type RET = [i16; 1];
let _15: i8;
let _16: Adt45;
let _17: f64;
let _18: u64;
let _19: u64;
let _20: Adt56;
let _21: isize;
let _22: u64;
let _23: *mut i64;
let _24: (i32, i64, u128, i64, f32);
let _25: isize;
let _26: ([i32; 1], [char; 1]);
let _27: f32;
let _28: ([i16; 1], i32);
let _29: (char,);
let _30: [i64; 3];
let _31: Adt46;
let _32: [char; 1];
let _33: [i32; 1];
let _34: [i16; 7];
let _35: f64;
let _36: i64;
let _37: i16;
let _38: (char,);
let _39: Adt55;
let _40: char;
let _41: ([i32; 1], [char; 1]);
let _42: f64;
let _43: char;
let _44: isize;
let _45: Adt58;
let _46: Adt54;
let _47: [i64; 3];
let _48: (i32,);
let _49: [i16; 7];
let _50: *const f32;
let _51: Adt57;
let _52: [char; 1];
let _53: char;
let _54: bool;
let _55: Adt42;
let _56: ();
let _57: ();
{
RET = [(-4765_i16)];
_10 = !_5;
_2 = !_4;
_7 = _4 | _12;
_7 = !_2;
RET = [10875_i16];
_1 = !_5;
_8 = _11 ^ _4;
_4 = _9 | _7;
Call(_12 = fn5(_5, _13, _1, _4, _4, _6, _5, _10, _8, _11, _4, _3, _5, _6, _4, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_13 = [_12,_5,_3];
_6 = 685978594803687999_u64 as isize;
_11 = '\u{4ba8e}' as isize;
_6 = _8 << _12;
_4 = _6 & _12;
_10 = _4 & _4;
_13 = [_10,_4,_10];
_13 = [_12,_12,_6];
_9 = _3 + _10;
_9 = -_10;
_8 = false as isize;
_5 = _12 * _10;
_18 = !13710691777430321758_u64;
_15 = 41789972_i32 as i8;
_15 = (-117_i8) * (-40_i8);
_17 = 93_u8 as f64;
_15 = -23_i8;
_14 = 116697135960607889611544219697482158282_u128;
_8 = 112_u8 as isize;
_8 = _10 | _6;
_3 = _6 >> _12;
_6 = _18 as isize;
_9 = _8;
_13 = [_3,_10,_9];
_9 = _10;
Call(RET = fn6(_5, _8, _12, _10, _9, _4, _3, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_14 = !155841522198544956721717660706821159746_u128;
_19 = 5859756743565351107_usize as u64;
_4 = _5 | _2;
_11 = -_7;
_13 = [_3,_5,_8];
Goto(bb3)
}
bb3 = {
_17 = 63_u8 as f64;
_4 = !_5;
_14 = !120156848925737305552242170660319220095_u128;
_3 = _12 - _5;
_9 = (-793541104_i32) as isize;
_1 = _3;
_3 = '\u{feba9}' as isize;
_13 = [_4,_10,_5];
_18 = _19 << _8;
_8 = _5 << _10;
_10 = (-41016088214288289031792908043667503222_i128) as isize;
_12 = _8;
_8 = _4;
_14 = !298583637697203263272300202352636688646_u128;
_21 = _8;
_1 = _21;
_11 = _4;
_14 = 285625553528235625006395559991602717772_u128;
_14 = 37632_u16 as u128;
_21 = _8 << _12;
_8 = -_4;
_19 = _18 << _4;
_13 = [_11,_4,_12];
_5 = (-26882_i16) as isize;
_15 = (-100_i8);
_11 = false as isize;
_21 = _17 as isize;
Goto(bb4)
}
bb4 = {
_1 = _2;
_18 = _19 | _19;
_7 = !_4;
_15 = (-23_i8);
_6 = 14159713652532135892_usize as isize;
_24.2 = _14 * _14;
_24.1 = (-6656336580410272729_i64) << _8;
_23 = core::ptr::addr_of_mut!(_24.1);
_2 = _8 * _8;
match _15 {
0 => bb5,
340282366920938463463374607431768211433 => bb7,
_ => bb6
}
}
bb5 = {
_17 = 63_u8 as f64;
_4 = !_5;
_14 = !120156848925737305552242170660319220095_u128;
_3 = _12 - _5;
_9 = (-793541104_i32) as isize;
_1 = _3;
_3 = '\u{feba9}' as isize;
_13 = [_4,_10,_5];
_18 = _19 << _8;
_8 = _5 << _10;
_10 = (-41016088214288289031792908043667503222_i128) as isize;
_12 = _8;
_8 = _4;
_14 = !298583637697203263272300202352636688646_u128;
_21 = _8;
_1 = _21;
_11 = _4;
_14 = 285625553528235625006395559991602717772_u128;
_14 = 37632_u16 as u128;
_21 = _8 << _12;
_8 = -_4;
_19 = _18 << _4;
_13 = [_11,_4,_12];
_5 = (-26882_i16) as isize;
_15 = (-100_i8);
_11 = false as isize;
_21 = _17 as isize;
Goto(bb4)
}
bb6 = {
_13 = [_12,_5,_3];
_6 = 685978594803687999_u64 as isize;
_11 = '\u{4ba8e}' as isize;
_6 = _8 << _12;
_4 = _6 & _12;
_10 = _4 & _4;
_13 = [_10,_4,_10];
_13 = [_12,_12,_6];
_9 = _3 + _10;
_9 = -_10;
_8 = false as isize;
_5 = _12 * _10;
_18 = !13710691777430321758_u64;
_15 = 41789972_i32 as i8;
_15 = (-117_i8) * (-40_i8);
_17 = 93_u8 as f64;
_15 = -23_i8;
_14 = 116697135960607889611544219697482158282_u128;
_8 = 112_u8 as isize;
_8 = _10 | _6;
_3 = _6 >> _12;
_6 = _18 as isize;
_9 = _8;
_13 = [_3,_10,_9];
_9 = _10;
Call(RET = fn6(_5, _8, _12, _10, _9, _4, _3, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_25 = _7 * _7;
_3 = _25 + _2;
_24.0 = 9109_i16 as i32;
_21 = _4;
_2 = _7 | _4;
_24.2 = _14 >> _21;
_30 = [(*_23),(*_23),_24.1];
_29.0 = '\u{775a7}';
RET = [(-8841_i16)];
_26.1 = [_29.0];
_24.4 = _24.2 as f32;
_24.3 = -(*_23);
_11 = _4;
_21 = _2 | _7;
_26.1 = [_29.0];
_28.0 = [19940_i16];
_28.0 = RET;
_28.1 = -_24.0;
_28.0 = [30418_i16];
_10 = _24.4 as isize;
_5 = _25 * _25;
_4 = _3 - _5;
Goto(bb8)
}
bb8 = {
_18 = _19 ^ _19;
_32 = [_29.0];
_17 = 2_usize as f64;
_32 = [_29.0];
_21 = _7;
_28.0 = [(-12939_i16)];
_10 = _7;
_13 = [_5,_12,_7];
_17 = _19 as f64;
_17 = _24.4 as f64;
_17 = 111595232255481484_usize as f64;
_18 = (*_23) as u64;
_32 = [_29.0];
_17 = _24.3 as f64;
_1 = 18392_i16 as isize;
_24.3 = (*_23) | (*_23);
_2 = -_25;
_33 = [_24.0];
_3 = -_7;
_5 = _3 ^ _8;
_2 = !_12;
Goto(bb9)
}
bb9 = {
_18 = _19;
_27 = _24.4 - _24.4;
_25 = !_5;
_29.0 = '\u{a3a4f}';
_24.1 = _24.3;
_10 = _25;
_5 = 5343919415921954940_usize as isize;
_25 = _7 & _10;
_15 = 75_i8 | (-53_i8);
_33 = [_28.1];
_26.0 = [_28.1];
_7 = -_12;
_28.0 = [(-10714_i16)];
_8 = false as isize;
_10 = 11970512129511826715_usize as isize;
_28.1 = _24.0;
_24 = (_28.1, 335168972672826689_i64, _14, 6172039553531958948_i64, _27);
_19 = _18 ^ _18;
_29 = ('\u{106c2b}',);
_12 = _14 as isize;
Goto(bb10)
}
bb10 = {
_23 = core::ptr::addr_of_mut!(_36);
_15 = _24.2 as i8;
_8 = _25 | _7;
_37 = !(-9908_i16);
RET = [_37];
_24.1 = !_24.3;
_13 = [_7,_3,_4];
_3 = _11 * _21;
_8 = -_2;
_8 = _19 as isize;
_17 = _15 as f64;
RET = [_37];
_28.0 = RET;
Goto(bb11)
}
bb11 = {
_29 = ('\u{39d76}',);
(*_23) = -_24.3;
_5 = -_4;
_24.2 = _37 as u128;
_23 = core::ptr::addr_of_mut!(_36);
_37 = _29.0 as i16;
_22 = _19 & _19;
_18 = false as u64;
_10 = _4 & _4;
_35 = _15 as f64;
_25 = !_2;
_29.0 = '\u{5de3e}';
_24 = (_28.1, (*_23), _14, (*_23), _27);
_25 = _11 >> _5;
_2 = _11;
_16 = Adt45::Variant2 { fld0: _24,fld1: 6969810005446736012_usize };
Call((*_23) = fn12(Field::<(i32, i64, u128, i64, f32)>(Variant(_16, 2), 0), _27), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_41.1 = [_29.0];
_38.0 = _29.0;
_40 = _38.0;
_1 = 2010045952_u32 as isize;
_38.0 = _40;
_42 = 22823_u16 as f64;
_44 = _3 << _21;
_18 = _22 >> _24.1;
_30 = [Field::<(i32, i64, u128, i64, f32)>(Variant(_16, 2), 0).1,(*_23),_36];
_10 = _11 << _7;
_44 = _25;
_41.0 = _33;
_19 = !_22;
_22 = _19;
RET = [_37];
place!(Field::<usize>(Variant(_16, 2), 1)) = 1_usize ^ 12563100371533708163_usize;
place!(Field::<(i32, i64, u128, i64, f32)>(Variant(_16, 2), 0)).3 = _24.1;
Goto(bb13)
}
bb13 = {
_41 = (_26.0, _26.1);
place!(Field::<(i32, i64, u128, i64, f32)>(Variant(_16, 2), 0)).3 = (*_23);
_7 = _44;
_38.0 = _29.0;
_42 = _17;
_47 = [Field::<(i32, i64, u128, i64, f32)>(Variant(_16, 2), 0).3,Field::<(i32, i64, u128, i64, f32)>(Variant(_16, 2), 0).3,(*_23)];
_41 = (_33, _26.1);
_36 = _24.3 ^ _24.3;
RET = _28.0;
_23 = core::ptr::addr_of_mut!(_36);
_15 = (-43_i8);
_33 = [_24.0];
_36 = Field::<(i32, i64, u128, i64, f32)>(Variant(_16, 2), 0).3 << _10;
_36 = _24.1;
Goto(bb14)
}
bb14 = {
_26 = (_41.0, _41.1);
_2 = _24.3 as isize;
_7 = -_11;
_38 = _29;
_18 = _22 | _19;
_16 = Adt45::Variant0 { fld0: _26,fld1: 10832232251811130355_usize };
_48 = (_24.0,);
(*_23) = -_24.1;
_41.1 = [_38.0];
_34 = [_37,_37,_37,_37,_37,_37,_37];
_14 = (-68391769753101994730453367878486973614_i128) as u128;
_43 = _38.0;
_24.3 = !(*_23);
_40 = _43;
(*_23) = _24.1 * _24.1;
_36 = _42 as i64;
_24 = (_48.0, (*_23), _14, (*_23), _27);
_26 = (Field::<([i32; 1], [char; 1])>(Variant(_16, 0), 0).0, _32);
_50 = core::ptr::addr_of!(_24.4);
_54 = !true;
_24.0 = _48.0;
_38 = (_29.0,);
_46 = Adt54::Variant2 { fld0: _50 };
Goto(bb15)
}
bb15 = {
Call(_56 = dump_var(4_usize, 5_usize, Move(_5), 34_usize, Move(_34), 36_usize, Move(_36), 37_usize, Move(_37)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_56 = dump_var(4_usize, 19_usize, Move(_19), 10_usize, Move(_10), 13_usize, Move(_13), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_56 = dump_var(4_usize, 7_usize, Move(_7), 26_usize, Move(_26), 9_usize, Move(_9), 43_usize, Move(_43)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_56 = dump_var(4_usize, 21_usize, Move(_21), 41_usize, Move(_41), 33_usize, Move(_33), 8_usize, Move(_8)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_56 = dump_var(4_usize, 44_usize, Move(_44), 32_usize, Move(_32), 57_usize, _57, 57_usize, _57), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: isize,mut _2: [isize; 3],mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize,mut _15: isize,mut _16: isize) -> isize {
mir! {
type RET = isize;
let _17: [char; 1];
let _18: bool;
let _19: Adt45;
let _20: f32;
let _21: ();
let _22: ();
{
_7 = _11 >> _13;
RET = 253481934428399693706571879521821733333_u128 as isize;
_15 = -_6;
_17 = ['\u{7a030}'];
_20 = 1604507684698994726_i64 as f32;
_1 = _9 & _9;
_3 = _1 - _9;
_8 = _3 << _3;
_2 = [_3,RET,_3];
_16 = -_4;
RET = _14 >> _8;
_17 = ['\u{7fc2}'];
Goto(bb1)
}
bb1 = {
Call(_21 = dump_var(5_usize, 12_usize, Move(_12), 13_usize, Move(_13), 1_usize, Move(_1), 5_usize, Move(_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_21 = dump_var(5_usize, 10_usize, Move(_10), 6_usize, Move(_6), 14_usize, Move(_14), 7_usize, Move(_7)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: [isize; 3]) -> [i16; 1] {
mir! {
type RET = [i16; 1];
let _9: Adt53;
let _10: [i64; 2];
let _11: char;
let _12: [i16; 7];
let _13: [char; 1];
let _14: isize;
let _15: [i16; 1];
let _16: u8;
let _17: bool;
let _18: Adt42;
let _19: isize;
let _20: Adt42;
let _21: *const *const i32;
let _22: isize;
let _23: i8;
let _24: char;
let _25: [i64; 3];
let _26: ([i32; 1], [char; 1]);
let _27: Adt46;
let _28: char;
let _29: isize;
let _30: [isize; 3];
let _31: f32;
let _32: Adt52;
let _33: f32;
let _34: bool;
let _35: ([i32; 1], [char; 1]);
let _36: isize;
let _37: isize;
let _38: i8;
let _39: bool;
let _40: [i64; 2];
let _41: ([i32; 1], [char; 1]);
let _42: char;
let _43: Adt45;
let _44: u16;
let _45: ();
let _46: ();
{
_6 = 23654_u16 as isize;
_6 = _3;
RET = [8502_i16];
_2 = -_6;
RET = [(-25177_i16)];
RET = [(-12886_i16)];
_8 = [_3,_4,_2];
_10 = [(-4910482331779889108_i64),(-8244278586844382888_i64)];
_8 = [_3,_1,_6];
_11 = '\u{f8ddc}';
_3 = _4 - _1;
_3 = _4 << _1;
_4 = 2876121804_u32 as isize;
Call(_12 = fn7(_5, _6, _2, _7, _5, _8, _5, _6, _6, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_15 = [25042_i16];
_7 = (-1901488827264884771_i64) as isize;
_16 = 96_u8;
_11 = '\u{632b7}';
_4 = _11 as isize;
_3 = _5;
_10 = [(-8088370517994985416_i64),(-4136178302182615122_i64)];
_8 = [_1,_3,_2];
_3 = _1;
_7 = _5 >> _5;
_6 = _2;
_14 = _1;
_1 = !_6;
_7 = _2;
_18 = Adt42 { fld0: 7687932021527741216_u64 };
_5 = _2 - _7;
_13 = [_11];
Goto(bb2)
}
bb2 = {
_20.fld0 = _18.fld0;
_19 = 31258_u16 as isize;
_8 = [_1,_1,_14];
_10 = [(-2902897758037979961_i64),(-6144998318963625320_i64)];
_2 = 2279266932_u32 as isize;
_17 = _7 > _1;
_12 = [(-24633_i16),(-27896_i16),(-30448_i16),(-30214_i16),(-29528_i16),(-14296_i16),(-16239_i16)];
_5 = _3 ^ _1;
_8 = [_3,_6,_3];
_6 = _14;
_16 = _11 as u8;
_13 = [_11];
_17 = true;
_15 = [19202_i16];
_18 = Adt42 { fld0: _20.fld0 };
_2 = _5;
_7 = _14 | _3;
_18.fld0 = _20.fld0 - _20.fld0;
RET = _15;
_22 = -_3;
_11 = '\u{8ded9}';
_3 = _2 & _14;
_22 = _7;
_19 = _7;
_16 = 207_u8;
_5 = !_22;
_18 = Move(_20);
Goto(bb3)
}
bb3 = {
RET = [(-16454_i16)];
_18.fld0 = 587023929871489795_u64 << _14;
_8 = [_7,_7,_1];
_11 = '\u{f5206}';
_22 = _17 as isize;
_24 = _11;
_12 = [(-9021_i16),(-1406_i16),28403_i16,21418_i16,266_i16,20045_i16,22951_i16];
_20 = Adt42 { fld0: _18.fld0 };
_15 = [(-14734_i16)];
_23 = 1117800754_u32 as i8;
_3 = !_1;
_14 = _6 * _6;
_4 = -_5;
RET = [(-31532_i16)];
RET = _15;
_19 = 5075463223981842901783388835142074404_u128 as isize;
_20 = Move(_18);
_23 = !(-87_i8);
RET = [7496_i16];
_18.fld0 = _20.fld0 << _7;
_18 = Adt42 { fld0: _20.fld0 };
Call(_9 = fn8(_5, _4, _20.fld0, _6, _6, _18.fld0, _1, _18.fld0, _20.fld0, _7, _8, _14, _6, Move(_18), _5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
SetDiscriminant(_9, 1);
_20.fld0 = !15057700088806710809_u64;
_26.1 = _13;
_12 = [8138_i16,(-12683_i16),(-9715_i16),10705_i16,29154_i16,(-29557_i16),16055_i16];
_8 = [_5,_3,_1];
RET = [16087_i16];
_28 = _11;
_16 = 104_u8;
_18 = Move(_20);
_23 = 55_i8;
_29 = _3 + _4;
_19 = _22;
place!(Field::<(*const i32, u16, [isize; 3], i8)>(Variant(_9, 1), 0)).2 = _8;
_6 = !_5;
_29 = _5 + _14;
place!(Field::<(*const i32, u16, [isize; 3], i8)>(Variant(_9, 1), 0)).2 = _8;
RET = [(-6516_i16)];
Goto(bb5)
}
bb5 = {
_20.fld0 = _18.fld0;
RET = _15;
RET = _15;
_23 = 53_i8 - (-51_i8);
_28 = _11;
_10 = [(-2370832136101153811_i64),(-8352093716241411780_i64)];
_18.fld0 = _23 as u64;
_31 = (-447847392_i32) as f32;
_24 = _28;
_1 = _3;
place!(Field::<(*const i32, u16, [isize; 3], i8)>(Variant(_9, 1), 0)).3 = _23 - _23;
place!(Field::<(*const i32, u16, [isize; 3], i8)>(Variant(_9, 1), 0)).2 = [_7,_5,_6];
_26.0 = [(-1702046115_i32)];
_7 = _14;
_12 = [23615_i16,29577_i16,11598_i16,(-14387_i16),(-17158_i16),9666_i16,(-26299_i16)];
_11 = _28;
RET = [32192_i16];
Goto(bb6)
}
bb6 = {
_34 = !_17;
_25 = [1828204329937878907_i64,(-3495635541832660969_i64),(-4522658408816659788_i64)];
Call(_7 = core::intrinsics::transmute(_29), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_3 = 13327_u16 as isize;
_25 = [(-8749745907223651437_i64),(-1407493532881144183_i64),400006093277790062_i64];
_18.fld0 = !_20.fld0;
_30 = [_29,_1,_6];
_6 = !_4;
match _16 {
0 => bb2,
1 => bb8,
2 => bb9,
3 => bb10,
104 => bb12,
_ => bb11
}
}
bb8 = {
_34 = !_17;
_25 = [1828204329937878907_i64,(-3495635541832660969_i64),(-4522658408816659788_i64)];
Call(_7 = core::intrinsics::transmute(_29), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
_20.fld0 = _18.fld0;
_19 = 31258_u16 as isize;
_8 = [_1,_1,_14];
_10 = [(-2902897758037979961_i64),(-6144998318963625320_i64)];
_2 = 2279266932_u32 as isize;
_17 = _7 > _1;
_12 = [(-24633_i16),(-27896_i16),(-30448_i16),(-30214_i16),(-29528_i16),(-14296_i16),(-16239_i16)];
_5 = _3 ^ _1;
_8 = [_3,_6,_3];
_6 = _14;
_16 = _11 as u8;
_13 = [_11];
_17 = true;
_15 = [19202_i16];
_18 = Adt42 { fld0: _20.fld0 };
_2 = _5;
_7 = _14 | _3;
_18.fld0 = _20.fld0 - _20.fld0;
RET = _15;
_22 = -_3;
_11 = '\u{8ded9}';
_3 = _2 & _14;
_22 = _7;
_19 = _7;
_16 = 207_u8;
_5 = !_22;
_18 = Move(_20);
Goto(bb3)
}
bb10 = {
SetDiscriminant(_9, 1);
_20.fld0 = !15057700088806710809_u64;
_26.1 = _13;
_12 = [8138_i16,(-12683_i16),(-9715_i16),10705_i16,29154_i16,(-29557_i16),16055_i16];
_8 = [_5,_3,_1];
RET = [16087_i16];
_28 = _11;
_16 = 104_u8;
_18 = Move(_20);
_23 = 55_i8;
_29 = _3 + _4;
_19 = _22;
place!(Field::<(*const i32, u16, [isize; 3], i8)>(Variant(_9, 1), 0)).2 = _8;
_6 = !_5;
_29 = _5 + _14;
place!(Field::<(*const i32, u16, [isize; 3], i8)>(Variant(_9, 1), 0)).2 = _8;
RET = [(-6516_i16)];
Goto(bb5)
}
bb11 = {
_15 = [25042_i16];
_7 = (-1901488827264884771_i64) as isize;
_16 = 96_u8;
_11 = '\u{632b7}';
_4 = _11 as isize;
_3 = _5;
_10 = [(-8088370517994985416_i64),(-4136178302182615122_i64)];
_8 = [_1,_3,_2];
_3 = _1;
_7 = _5 >> _5;
_6 = _2;
_14 = _1;
_1 = !_6;
_7 = _2;
_18 = Adt42 { fld0: 7687932021527741216_u64 };
_5 = _2 - _7;
_13 = [_11];
Goto(bb2)
}
bb12 = {
_25 = [(-8447816802029285613_i64),4988436333786464510_i64,(-8473862824481313060_i64)];
_17 = _34;
_36 = _6 << _29;
_7 = _29 ^ _29;
_13 = _26.1;
_3 = _24 as isize;
_7 = _4;
_1 = !_5;
place!(Field::<(*const i32, u16, [isize; 3], i8)>(Variant(_9, 1), 0)).3 = !_23;
_36 = _5;
_6 = _2 & _1;
_1 = _2 << _6;
_37 = 46978_u16 as isize;
_4 = -_36;
_18.fld0 = _31 as u64;
_37 = _5 >> _36;
_7 = _6 + _5;
_29 = _14 + _4;
_33 = -_31;
_35.0 = _26.0;
place!(Field::<(*const i32, u16, [isize; 3], i8)>(Variant(_9, 1), 0)).1 = 44526_u16 - 56559_u16;
_35.1 = [_11];
_8 = [_29,_29,_14];
_2 = _16 as isize;
_5 = -_29;
_22 = !_7;
_23 = _34 as i8;
Goto(bb13)
}
bb13 = {
_6 = !_22;
_12 = [(-1965_i16),9184_i16,7689_i16,(-23852_i16),(-5795_i16),(-7845_i16),21170_i16];
_19 = _7;
RET = [16228_i16];
_40 = [(-535965316995726080_i64),4375536114548757489_i64];
_26.0 = [1238493353_i32];
_3 = _4 & _6;
_6 = Field::<(*const i32, u16, [isize; 3], i8)>(Variant(_9, 1), 0).1 as isize;
_30 = [_5,_29,_36];
RET = [10054_i16];
_8 = _30;
_36 = _3;
_40 = [2111918859672191636_i64,7100225717120127759_i64];
_41.1 = _13;
_29 = _1;
_41 = _26;
_6 = _29 ^ _29;
Goto(bb14)
}
bb14 = {
_3 = -_14;
_14 = _19;
_41 = (_26.0, _35.1);
_41.1 = [_28];
_39 = _17;
_28 = _24;
_21 = core::ptr::addr_of!(place!(Field::<(*const i32, u16, [isize; 3], i8)>(Variant(_9, 1), 0)).0);
_2 = _7 << _22;
_26.1 = _13;
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(6_usize, 19_usize, Move(_19), 13_usize, Move(_13), 30_usize, Move(_30), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(6_usize, 41_usize, Move(_41), 17_usize, Move(_17), 4_usize, Move(_4), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(6_usize, 16_usize, Move(_16), 25_usize, Move(_25), 22_usize, Move(_22), 34_usize, Move(_34)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(6_usize, 7_usize, Move(_7), 3_usize, Move(_3), 23_usize, Move(_23), 36_usize, Move(_36)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: [isize; 3],mut _7: isize,mut _8: isize,mut _9: isize,mut _10: [isize; 3]) -> [i16; 7] {
mir! {
type RET = [i16; 7];
let _11: bool;
let _12: f32;
let _13: (i32,);
let _14: ([i16; 1], i32);
let _15: f32;
let _16: (i32,);
let _17: f32;
let _18: Adt48;
let _19: isize;
let _20: i16;
let _21: Adt58;
let _22: ();
let _23: ();
{
_10 = _6;
_8 = 1920495625_u32 as isize;
_10 = [_1,_2,_2];
_10 = [_2,_2,_1];
RET = [(-15414_i16),(-1067_i16),(-3203_i16),27011_i16,(-21019_i16),(-17113_i16),3546_i16];
_7 = _3 << _4;
_6 = [_7,_2,_2];
_11 = true;
_9 = _4;
_10 = [_7,_4,_3];
_9 = _5 ^ _3;
_9 = _1;
_4 = '\u{96802}' as isize;
_6 = _10;
_6 = [_1,_3,_2];
_12 = 1866685593_u32 as f32;
_13.0 = 1138913009_i32;
_9 = _3 ^ _3;
_9 = _5 - _7;
_8 = _2 + _1;
_10 = [_3,_9,_9];
RET = [(-32752_i16),31748_i16,(-13296_i16),(-26687_i16),(-22830_i16),(-31978_i16),5981_i16];
_3 = _1;
_13.0 = (-1844736227_i32);
match _13.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607429923475229 => bb9,
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
RET = [11809_i16,20683_i16,18020_i16,(-27102_i16),(-27770_i16),9151_i16,(-12037_i16)];
_8 = _9 - _2;
_13 = (1576066283_i32,);
_5 = -_1;
_7 = -_8;
RET = [3144_i16,(-2467_i16),1982_i16,28592_i16,(-18984_i16),10896_i16,16545_i16];
_12 = 1733750604_u32 as f32;
_4 = _8 + _5;
_4 = _2;
_11 = _8 >= _1;
_9 = !_8;
_12 = 126845872945191789505239838853684429398_u128 as f32;
RET = [18292_i16,12610_i16,28116_i16,(-32461_i16),(-3279_i16),30741_i16,(-25172_i16)];
_13 = ((-1369722632_i32),);
RET = [6664_i16,(-31425_i16),(-25398_i16),10654_i16,(-2162_i16),28602_i16,5715_i16];
_15 = _12 - _12;
_6 = [_7,_5,_3];
_14.1 = -_13.0;
_11 = !false;
_14.0 = [21295_i16];
RET = [(-4072_i16),(-2085_i16),(-11620_i16),(-15186_i16),(-15924_i16),(-30745_i16),7334_i16];
Goto(bb10)
}
bb10 = {
_10 = [_4,_2,_2];
_7 = !_5;
_10 = [_8,_7,_9];
_13 = (_14.1,);
_1 = -_2;
_14.0 = [(-8879_i16)];
_14.1 = -_13.0;
_7 = _5;
_13.0 = 1060550866858625462_u64 as i32;
_5 = -_4;
_6 = _10;
_16 = (_13.0,);
_1 = -_7;
_16.0 = _13.0 & _13.0;
_14.0 = [11806_i16];
_7 = !_8;
_17 = -_15;
_10 = [_5,_5,_1];
_17 = -_12;
Call(_1 = core::intrinsics::bswap(_4), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_12 = _17;
_13.0 = -_14.1;
_8 = _7;
_1 = !_4;
_10 = _6;
_17 = _12 * _15;
_9 = (-5141208928338064821_i64) as isize;
_19 = _1 - _8;
_13.0 = !_16.0;
_3 = _19 ^ _5;
_19 = _7 & _4;
_13.0 = _16.0 ^ _16.0;
_12 = _15;
_10 = [_8,_2,_5];
_8 = 3274943731_u32 as isize;
_7 = _19 - _2;
RET = [5692_i16,(-23724_i16),27078_i16,1615_i16,12538_i16,(-1187_i16),(-29385_i16)];
_12 = -_17;
_11 = false ^ false;
_10 = [_4,_5,_19];
_13 = (_16.0,);
_20 = 31106_i16 >> _5;
_8 = _3 - _3;
_15 = _17;
RET = [_20,_20,_20,_20,_20,_20,_20];
_5 = _8;
Goto(bb12)
}
bb12 = {
Call(_22 = dump_var(7_usize, 19_usize, Move(_19), 3_usize, Move(_3), 7_usize, Move(_7), 2_usize, Move(_2)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_22 = dump_var(7_usize, 13_usize, Move(_13), 8_usize, Move(_8), 11_usize, Move(_11), 4_usize, Move(_4)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: isize,mut _2: isize,mut _3: u64,mut _4: isize,mut _5: isize,mut _6: u64,mut _7: isize,mut _8: u64,mut _9: u64,mut _10: isize,mut _11: [isize; 3],mut _12: isize,mut _13: isize,mut _14: Adt42,mut _15: isize) -> Adt53 {
mir! {
type RET = Adt53;
let _16: [i16; 7];
let _17: isize;
let _18: bool;
let _19: isize;
let _20: isize;
let _21: f64;
let _22: Adt42;
let _23: [i32; 1];
let _24: [i16; 1];
let _25: isize;
let _26: char;
let _27: Adt44;
let _28: (*const i32, u16, [isize; 3], i8);
let _29: bool;
let _30: [i64; 3];
let _31: Adt55;
let _32: isize;
let _33: Adt56;
let _34: u128;
let _35: ([i16; 1], i32);
let _36: u128;
let _37: u128;
let _38: f32;
let _39: *const *const i32;
let _40: i32;
let _41: u32;
let _42: Adt43;
let _43: Adt54;
let _44: char;
let _45: [i16; 1];
let _46: *const u8;
let _47: *mut u16;
let _48: (i32,);
let _49: &'static u8;
let _50: [i64; 3];
let _51: Adt44;
let _52: ();
let _53: ();
{
_16 = [5711_i16,14909_i16,11247_i16,13537_i16,(-5552_i16),20582_i16,(-12185_i16)];
_7 = _1 ^ _2;
_15 = _1 | _5;
_2 = 152000794408445586374457903557898096165_i128 as isize;
_5 = !_4;
_1 = _12;
_5 = -_15;
_15 = _1;
_7 = _1 >> _9;
_6 = _8;
_16 = [(-23352_i16),(-30506_i16),(-5338_i16),13619_i16,2763_i16,(-27048_i16),25939_i16];
_19 = '\u{244a}' as isize;
_16 = [23647_i16,30800_i16,19157_i16,(-2676_i16),(-11665_i16),(-24693_i16),6066_i16];
_14 = Adt42 { fld0: _6 };
_14 = Adt42 { fld0: _3 };
_2 = _13 & _4;
_10 = _1;
_7 = _1;
Call(_17 = fn9(Move(_14), _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_18 = !false;
_10 = _13;
_7 = _17;
_7 = _15 | _1;
_9 = !_3;
_16 = [5410_i16,(-32451_i16),(-11940_i16),4732_i16,21190_i16,(-6488_i16),(-4345_i16)];
_11 = [_2,_2,_13];
_5 = _13 & _1;
_7 = -_10;
_20 = -_7;
_14 = Adt42 { fld0: _3 };
_20 = -_5;
_2 = _12 | _12;
_4 = 47734_u16 as isize;
_7 = !_12;
_19 = (-28341_i16) as isize;
_20 = 32373887468965620560134991036698578786_u128 as isize;
_18 = true;
_22 = Adt42 { fld0: _14.fld0 };
_21 = 89_i8 as f64;
_22 = Adt42 { fld0: _14.fld0 };
Goto(bb2)
}
bb2 = {
_2 = -_17;
_6 = _8 ^ _3;
_19 = _10;
_28.2 = [_19,_13,_17];
_15 = _19 << _3;
_15 = _13;
Call(_11 = fn10(_17, _13, _15, _28.2), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_25 = _2 & _19;
_8 = _3;
_13 = 125_i8 as isize;
_5 = _7;
_6 = _8;
_6 = (-6689_i16) as u64;
_13 = _2;
_23 = [(-770317855_i32)];
_13 = _1 + _5;
_5 = _12 - _12;
_24 = [(-25175_i16)];
_12 = (-22757970018587313016655636719588262433_i128) as isize;
_19 = (-7018065166167189933_i64) as isize;
_11 = _28.2;
_4 = _25;
_21 = 1_usize as f64;
_8 = !_14.fld0;
Goto(bb4)
}
bb4 = {
_26 = '\u{3f1d5}';
_3 = !_22.fld0;
_30 = [(-280127745376475163_i64),(-4071542524310673149_i64),8415573479110843629_i64];
_11 = [_17,_1,_5];
_32 = _10;
_1 = _18 as isize;
_17 = 89_i8 as isize;
_8 = _22.fld0;
_4 = _2;
_15 = _25;
_21 = 22279_u16 as f64;
_20 = _5;
_14 = Move(_22);
_20 = 4210539702_u32 as isize;
_24 = [(-7486_i16)];
_28.3 = (-78_i8) << _13;
_14 = Adt42 { fld0: _3 };
_22.fld0 = !_3;
_26 = '\u{99573}';
_21 = _5 as f64;
_35.1 = !1866245887_i32;
_13 = !_4;
_6 = _5 as u64;
_20 = _35.1 as isize;
_15 = 4082615855_u32 as isize;
Goto(bb5)
}
bb5 = {
_34 = 212253526770052155528980603047661508018_u128 ^ 15829970857064507109459598675502446115_u128;
_26 = '\u{e3851}';
_14 = Move(_22);
_35.0 = [(-15532_i16)];
_17 = _32;
_29 = !_18;
_22 = Adt42 { fld0: _6 };
_36 = _34;
_5 = _29 as isize;
Call(_35.1 = fn11(_32, _2, _13, _7, _10, Move(_22), _32, _7, _10, _25, _25, _13), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_23 = [_35.1];
Goto(bb7)
}
bb7 = {
_19 = _10 | _7;
_23 = [_35.1];
_26 = '\u{dfeca}';
_38 = _36 as f32;
_21 = _28.3 as f64;
_4 = _19 * _7;
_26 = '\u{2973e}';
_17 = _32;
_36 = _21 as u128;
_39 = core::ptr::addr_of!(_28.0);
_35 = (_24, (-1551121747_i32));
_37 = _36;
_28.1 = _36 as u16;
_11 = [_19,_2,_13];
_30 = [4264904180641902161_i64,(-1095242940903165845_i64),5882845395205983368_i64];
_23 = [_35.1];
_10 = -_32;
_28.2 = [_13,_17,_13];
_28.3 = (-109_i8);
_9 = _3;
_37 = _36;
Goto(bb8)
}
bb8 = {
_25 = _7 - _32;
_14.fld0 = _9 ^ _3;
_36 = _37;
_36 = !_34;
_41 = _18 as u32;
_8 = !_6;
Goto(bb9)
}
bb9 = {
_4 = _13;
_37 = _34 >> _14.fld0;
_44 = _26;
_20 = _13;
_28.0 = core::ptr::addr_of!(_40);
RET = Adt53::Variant1 { fld0: _28 };
_41 = 2786121927_u32 | 872716610_u32;
place!(Field::<(*const i32, u16, [isize; 3], i8)>(Variant(RET, 1), 0)).2 = [_7,_20,_10];
_7 = _29 as isize;
_14.fld0 = !_6;
_4 = _25;
_35 = (_24, (-1960463865_i32));
_22 = Adt42 { fld0: _14.fld0 };
_48 = (_35.1,);
Goto(bb10)
}
bb10 = {
Call(_52 = dump_var(8_usize, 41_usize, Move(_41), 3_usize, Move(_3), 34_usize, Move(_34), 2_usize, Move(_2)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_52 = dump_var(8_usize, 24_usize, Move(_24), 29_usize, Move(_29), 16_usize, Move(_16), 1_usize, Move(_1)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_52 = dump_var(8_usize, 23_usize, Move(_23), 18_usize, Move(_18), 30_usize, Move(_30), 48_usize, Move(_48)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_52 = dump_var(8_usize, 8_usize, Move(_8), 13_usize, Move(_13), 9_usize, Move(_9), 20_usize, Move(_20)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: Adt42,mut _2: isize) -> isize {
mir! {
type RET = isize;
let _3: ([i32; 1], [char; 1]);
let _4: *const *const i32;
let _5: Adt47;
let _6: [isize; 3];
let _7: ();
let _8: ();
{
_3.0 = [546185718_i32];
RET = _2;
_3.1 = ['\u{9e451}'];
_3.1 = ['\u{16c4b}'];
_3.0 = [719176443_i32];
_3.0 = [1236011625_i32];
_3.0 = [499551941_i32];
_2 = -RET;
_3.1 = ['\u{9c3ff}'];
_1 = Adt42 { fld0: 14829915951604603485_u64 };
RET = 151_u8 as isize;
_3.1 = ['\u{7f40e}'];
_3.1 = ['\u{58484}'];
RET = _2;
RET = _2 >> _2;
RET = _2;
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(9_usize, 2_usize, Move(_2), 8_usize, _8, 8_usize, _8, 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: [isize; 3]) -> [isize; 3] {
mir! {
type RET = [isize; 3];
let _5: ([i16; 1], i32);
let _6: ();
let _7: ();
{
_3 = _2;
_1 = (-1128097954_i32) as isize;
_1 = !_2;
_3 = 9103351074011377103_i64 as isize;
_3 = _1 & _2;
RET = _4;
RET = _4;
_5.1 = (-1529237399_i32) - 1675795035_i32;
RET = _4;
_2 = -_1;
RET = [_3,_1,_3];
_5.0 = [(-10096_i16)];
_5.1 = (-1686093558_i32) | 1136801255_i32;
RET = [_3,_3,_3];
_1 = _3;
_3 = _5.1 as isize;
Goto(bb1)
}
bb1 = {
Call(_6 = dump_var(10_usize, 5_usize, Move(_5), 3_usize, Move(_3), 7_usize, _7, 7_usize, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: Adt42,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize) -> i32 {
mir! {
type RET = i32;
let _13: Adt55;
let _14: [i64; 3];
let _15: [i16; 1];
let _16: usize;
let _17: isize;
let _18: ();
let _19: ();
{
_10 = _12;
_10 = !_8;
_5 = _4 | _9;
RET = 1915563126_i32;
_6.fld0 = 6_usize as u64;
_11 = _12 >> _4;
_7 = _8 + _11;
_10 = !_5;
_5 = false as isize;
_14 = [8081969326841001003_i64,4871157855101775116_i64,(-5903376022753519502_i64)];
_10 = _7;
_10 = _4;
_4 = _9;
_15 = [(-14155_i16)];
_16 = 7_usize ^ 0_usize;
RET = _10 as i32;
_12 = -_9;
_14 = [(-7102980691413136102_i64),3161592915206096556_i64,(-472081417209032958_i64)];
_3 = _6.fld0 as isize;
_5 = -_7;
_7 = _16 as isize;
_3 = 253_u8 as isize;
Goto(bb1)
}
bb1 = {
Call(_18 = dump_var(11_usize, 9_usize, Move(_9), 4_usize, Move(_4), 10_usize, Move(_10), 11_usize, Move(_11)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_18 = dump_var(11_usize, 2_usize, Move(_2), 12_usize, Move(_12), 15_usize, Move(_15), 19_usize, _19), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: (i32, i64, u128, i64, f32),mut _2: f32) -> i64 {
mir! {
type RET = i64;
let _3: bool;
let _4: (*const i32, u16, [isize; 3], i8);
let _5: ();
let _6: ();
{
RET = _1.1;
_1.1 = _1.3;
RET = _1.1 >> _1.1;
RET = _1.0 as i64;
_1.1 = _1.3;
_1.4 = _2;
_1.3 = (-9223372036854775808_isize) as i64;
RET = _1.2 as i64;
_3 = !false;
_1.0 = (-2088050664_i32) & 1093622692_i32;
_4.1 = 26718_u16 | 39798_u16;
RET = !_1.1;
_1.4 = _2;
Goto(bb1)
}
bb1 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: [i16; 1],mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: [isize; 3],mut _7: isize,mut _8: [isize; 3],mut _9: isize,mut _10: f64,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize) -> f32 {
mir! {
type RET = f32;
let _15: Adt45;
let _16: char;
let _17: Adt51;
let _18: [i16; 1];
let _19: usize;
let _20: ();
let _21: ();
{
_3 = 13868970786471444046_usize as isize;
_2 = _5;
Call(_1 = fn14(_6, _12, _9, _12, _13, _4, _5, _6, _4, _14), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12 = _11 + _9;
RET = _12 as f32;
_1 = [(-18824_i16)];
_1 = [(-8952_i16)];
RET = 11682390845573967230_usize as f32;
_2 = '\u{8b175}' as isize;
_7 = _9;
_14 = _4 << _11;
RET = 8179472367382280154_usize as f32;
RET = _12 as f32;
_12 = -_4;
_6 = [_9,_12,_9];
_14 = (-19062_i16) as isize;
_5 = !_11;
_11 = _5 << _12;
_1 = [30397_i16];
_18 = _1;
_9 = -_12;
_18 = _1;
_13 = _4 + _12;
_18 = [11631_i16];
_18 = [1528_i16];
_14 = (-7780_i16) as isize;
_2 = -_12;
_1 = [18046_i16];
_4 = _13;
_7 = (-88_i8) as isize;
Goto(bb2)
}
bb2 = {
Call(_20 = dump_var(13_usize, 1_usize, Move(_1), 9_usize, Move(_9), 8_usize, Move(_8), 14_usize, Move(_14)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_20 = dump_var(13_usize, 13_usize, Move(_13), 11_usize, Move(_11), 3_usize, Move(_3), 21_usize, _21), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: [isize; 3],mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: [isize; 3],mut _9: isize,mut _10: isize) -> [i16; 1] {
mir! {
type RET = [i16; 1];
let _11: i64;
let _12: ([i16; 1], i32);
let _13: u64;
let _14: Adt47;
let _15: *mut i64;
let _16: i32;
let _17: usize;
let _18: Adt46;
let _19: [i64; 3];
let _20: bool;
let _21: i16;
let _22: f32;
let _23: *const *mut u128;
let _24: *mut i64;
let _25: [char; 1];
let _26: isize;
let _27: i32;
let _28: [char; 1];
let _29: bool;
let _30: u128;
let _31: isize;
let _32: *const *mut u128;
let _33: *mut i64;
let _34: Adt49;
let _35: isize;
let _36: char;
let _37: ([i32; 1], [char; 1]);
let _38: Adt54;
let _39: [i32; 1];
let _40: [i32; 1];
let _41: ([i32; 1], [char; 1]);
let _42: [isize; 3];
let _43: Adt48;
let _44: ();
let _45: ();
{
_6 = _7;
_11 = 8883804233595516713_u64 as i64;
_12.0 = [10953_i16];
RET = [(-8069_i16)];
_7 = _9 << _6;
_12 = (RET, (-970085970_i32));
_6 = !_5;
RET = [(-5442_i16)];
_9 = _2;
_9 = _12.1 as isize;
_9 = !_7;
_8 = [_7,_9,_7];
Call(_5 = core::intrinsics::bswap(_9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = '\u{39488}' as isize;
RET = [(-29074_i16)];
_13 = !2927095726659175533_u64;
_2 = !_9;
_10 = _6;
RET = _12.0;
_2 = _6;
_1 = [_7,_3,_9];
_7 = _5;
_11 = (-159016545343208022_i64);
_12 = (RET, (-689894227_i32));
RET = _12.0;
_7 = _10 & _2;
_15 = core::ptr::addr_of_mut!(_11);
(*_15) = (-2580904434766805898_i64) - (-1557699374741572213_i64);
RET = [1624_i16];
_11 = _13 as i64;
_17 = !15399338426542346703_usize;
Goto(bb2)
}
bb2 = {
_3 = _7 | _2;
_3 = _7;
_3 = _9 & _9;
_12 = (RET, (-45170401_i32));
_12 = (RET, 119155783_i32);
_4 = _3 - _3;
_2 = _3 | _4;
_19 = [(*_15),(*_15),_11];
_13 = 10919267105457044518_u64;
_20 = false;
_7 = _4 | _2;
(*_15) = _4 as i64;
RET = [(-13132_i16)];
_16 = _12.1;
(*_15) = _17 as i64;
_4 = 59799250654291925143718189826354013918_u128 as isize;
_21 = !(-5087_i16);
RET = _12.0;
_1 = _8;
_17 = 215_u8 as usize;
_2 = _17 as isize;
_7 = -_3;
_9 = _7;
Goto(bb3)
}
bb3 = {
_15 = core::ptr::addr_of_mut!(_11);
_8 = [_9,_3,_7];
_10 = '\u{90104}' as isize;
_4 = _9;
_16 = _12.1 + _12.1;
_13 = 15681366338508037899_u64;
_9 = (-11797365854481517119458493788164937619_i128) as isize;
_9 = -_4;
_9 = 150194895610801923713295340327834072484_u128 as isize;
_21 = 28045_i16 | (-23980_i16);
_11 = -4599289702281178114_i64;
_22 = 207119957341239481118295343472372625407_u128 as f32;
RET = [_21];
_21 = 4491_i16 + (-11958_i16);
_8 = _1;
(*_15) = (-7957630843251644374_i64);
_1 = [_3,_7,_4];
_24 = _15;
(*_15) = -(-7562981911802587306_i64);
_21 = (-71_i8) as i16;
_11 = 798778781_u32 as i64;
_11 = 2213528912_u32 as i64;
Call(_3 = core::intrinsics::bswap(_4), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_25 = ['\u{68200}'];
(*_24) = _20 as i64;
_25 = ['\u{657a0}'];
_5 = _4 << _7;
_2 = !_4;
RET = [_21];
_12.0 = [_21];
_11 = 357625573585631544_i64 >> _3;
_24 = _15;
_26 = _13 as isize;
(*_15) = 26_i8 as i64;
_12.0 = [_21];
_12.1 = _16 | _16;
_13 = 140356420233369045_u64 << _2;
(*_15) = _3 as i64;
_27 = _16;
_5 = 331126839172154064957610632505874806031_u128 as isize;
(*_24) = 2853427218584988904_i64;
_20 = !false;
_4 = 290588180455653872628372964219739462561_u128 as isize;
match (*_24) {
0 => bb1,
1 => bb5,
2 => bb6,
2853427218584988904 => bb8,
_ => bb7
}
}
bb5 = {
_15 = core::ptr::addr_of_mut!(_11);
_8 = [_9,_3,_7];
_10 = '\u{90104}' as isize;
_4 = _9;
_16 = _12.1 + _12.1;
_13 = 15681366338508037899_u64;
_9 = (-11797365854481517119458493788164937619_i128) as isize;
_9 = -_4;
_9 = 150194895610801923713295340327834072484_u128 as isize;
_21 = 28045_i16 | (-23980_i16);
_11 = -4599289702281178114_i64;
_22 = 207119957341239481118295343472372625407_u128 as f32;
RET = [_21];
_21 = 4491_i16 + (-11958_i16);
_8 = _1;
(*_15) = (-7957630843251644374_i64);
_1 = [_3,_7,_4];
_24 = _15;
(*_15) = -(-7562981911802587306_i64);
_21 = (-71_i8) as i16;
_11 = 798778781_u32 as i64;
_11 = 2213528912_u32 as i64;
Call(_3 = core::intrinsics::bswap(_4), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_3 = _7 | _2;
_3 = _7;
_3 = _9 & _9;
_12 = (RET, (-45170401_i32));
_12 = (RET, 119155783_i32);
_4 = _3 - _3;
_2 = _3 | _4;
_19 = [(*_15),(*_15),_11];
_13 = 10919267105457044518_u64;
_20 = false;
_7 = _4 | _2;
(*_15) = _4 as i64;
RET = [(-13132_i16)];
_16 = _12.1;
(*_15) = _17 as i64;
_4 = 59799250654291925143718189826354013918_u128 as isize;
_21 = !(-5087_i16);
RET = _12.0;
_1 = _8;
_17 = 215_u8 as usize;
_2 = _17 as isize;
_7 = -_3;
_9 = _7;
Goto(bb3)
}
bb7 = {
_4 = '\u{39488}' as isize;
RET = [(-29074_i16)];
_13 = !2927095726659175533_u64;
_2 = !_9;
_10 = _6;
RET = _12.0;
_2 = _6;
_1 = [_7,_3,_9];
_7 = _5;
_11 = (-159016545343208022_i64);
_12 = (RET, (-689894227_i32));
RET = _12.0;
_7 = _10 & _2;
_15 = core::ptr::addr_of_mut!(_11);
(*_15) = (-2580904434766805898_i64) - (-1557699374741572213_i64);
RET = [1624_i16];
_11 = _13 as i64;
_17 = !15399338426542346703_usize;
Goto(bb2)
}
bb8 = {
_21 = _12.1 as i16;
_29 = _7 < _2;
_24 = core::ptr::addr_of_mut!(_11);
(*_24) = 195_u8 as i64;
_3 = _7;
_30 = (*_15) as u128;
_8 = [_7,_7,_3];
_20 = _29 ^ _29;
_24 = core::ptr::addr_of_mut!(_11);
RET = [_21];
_30 = !212167096196346531363515267430280405526_u128;
_25 = ['\u{11ec8}'];
_4 = -_7;
_10 = _7 + _2;
_4 = _7 << _10;
_22 = (-75724055912988964247936584018444688934_i128) as f32;
_15 = _24;
_31 = _7 + _3;
_9 = -_2;
_16 = _12.1;
Call(_6 = core::intrinsics::bswap(_7), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_12.1 = _16;
(*_24) = 28979_u16 as i64;
_28 = _25;
(*_24) = -2050589617974842244_i64;
RET = [_21];
_17 = 17866754416189816585_usize >> _4;
_16 = _4 as i32;
_2 = !_10;
_1 = [_4,_4,_10];
RET = [_21];
_7 = _30 as isize;
_7 = -_10;
_2 = _3 & _10;
_24 = _15;
_11 = 1688130970979278884_i64 - (-1510638508303743811_i64);
_3 = '\u{dd7cd}' as isize;
_3 = _2 << _10;
_6 = -_4;
_34.fld1 = _17 << _3;
_8 = [_9,_9,_9];
_34.fld0 = 26884263068270938316282652137232729961_i128;
Call(_34 = fn15(_4, _13, _4, _7, _9, _9, _2, _3, _8, _31, _8), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
RET = [_21];
_13 = !1977255754732829166_u64;
RET = [_21];
Goto(bb11)
}
bb11 = {
_26 = _3 << _2;
_30 = 295181245744114793021628455901460158132_u128;
_27 = -_16;
(*_15) = -(-3478241403899373868_i64);
_34.fld3 = _22 as f64;
_2 = _21 as isize;
_5 = 244_u8 as isize;
Goto(bb12)
}
bb12 = {
_32 = core::ptr::addr_of!(_34.fld2);
_34.fld1 = _17 & _17;
_22 = _34.fld3 as f32;
RET = _12.0;
_16 = -_27;
_31 = -_10;
_24 = core::ptr::addr_of_mut!((*_15));
_34.fld4 = [_21,_21];
_11 = _34.fld3 as i64;
_34.fld3 = 81_u8 as f64;
_3 = _26;
_35 = !_7;
_31 = !_26;
_30 = !199324558205387895423378615278688240889_u128;
_3 = _22 as isize;
_33 = _15;
(*_32) = core::ptr::addr_of_mut!(_30);
Goto(bb13)
}
bb13 = {
_23 = _32;
RET = [_21];
_30 = 74126548830294681562884538250484971355_u128 >> _34.fld1;
_30 = _4 as u128;
_25 = ['\u{15194}'];
RET = _12.0;
_13 = 15774100871882357832_u64;
_37.0 = [_16];
_37.1 = ['\u{3f8fc}'];
_12 = (RET, _16);
match _13 {
0 => bb1,
1 => bb2,
2 => bb10,
3 => bb7,
4 => bb14,
15774100871882357832 => bb16,
_ => bb15
}
}
bb14 = {
_12.1 = _16;
(*_24) = 28979_u16 as i64;
_28 = _25;
(*_24) = -2050589617974842244_i64;
RET = [_21];
_17 = 17866754416189816585_usize >> _4;
_16 = _4 as i32;
_2 = !_10;
_1 = [_4,_4,_10];
RET = [_21];
_7 = _30 as isize;
_7 = -_10;
_2 = _3 & _10;
_24 = _15;
_11 = 1688130970979278884_i64 - (-1510638508303743811_i64);
_3 = '\u{dd7cd}' as isize;
_3 = _2 << _10;
_6 = -_4;
_34.fld1 = _17 << _3;
_8 = [_9,_9,_9];
_34.fld0 = 26884263068270938316282652137232729961_i128;
Call(_34 = fn15(_4, _13, _4, _7, _9, _9, _2, _3, _8, _31, _8), ReturnTo(bb10), UnwindUnreachable())
}
bb15 = {
_26 = _3 << _2;
_30 = 295181245744114793021628455901460158132_u128;
_27 = -_16;
(*_15) = -(-3478241403899373868_i64);
_34.fld3 = _22 as f64;
_2 = _21 as isize;
_5 = 244_u8 as isize;
Goto(bb12)
}
bb16 = {
_2 = _4 + _4;
_42 = _1;
_41.1 = _25;
_34.fld3 = _27 as f64;
RET = _12.0;
_24 = _15;
_32 = _23;
_17 = _34.fld1 * _34.fld1;
Goto(bb17)
}
bb17 = {
Call(_44 = dump_var(14_usize, 29_usize, Move(_29), 17_usize, Move(_17), 11_usize, Move(_11), 20_usize, Move(_20)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(14_usize, 35_usize, Move(_35), 5_usize, Move(_5), 7_usize, Move(_7), 3_usize, Move(_3)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_44 = dump_var(14_usize, 13_usize, Move(_13), 30_usize, Move(_30), 31_usize, Move(_31), 1_usize, Move(_1)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_44 = dump_var(14_usize, 37_usize, Move(_37), 10_usize, Move(_10), 45_usize, _45, 45_usize, _45), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: isize,mut _2: u64,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: [isize; 3],mut _10: isize,mut _11: [isize; 3]) -> Adt49 {
mir! {
type RET = Adt49;
let _12: u32;
let _13: [i64; 3];
let _14: u64;
let _15: usize;
let _16: Adt50;
let _17: *const (i32,);
let _18: isize;
let _19: isize;
let _20: Adt42;
let _21: bool;
let _22: [i16; 2];
let _23: ();
let _24: ();
{
_5 = !_10;
_9 = _11;
_5 = _1;
_12 = !2050723508_u32;
_7 = _5;
RET.fld4 = [16210_i16,(-1389_i16)];
RET.fld0 = -169051749202903343716356781271770984223_i128;
_11 = [_8,_4,_1];
_10 = -_1;
Goto(bb1)
}
bb1 = {
RET.fld4 = [27670_i16,1147_i16];
_13 = [4781978099138139818_i64,5023099476356969096_i64,(-6258556240301398308_i64)];
_7 = true as isize;
RET.fld1 = (-3192638078020237059_i64) as usize;
RET.fld3 = 182_u8 as f64;
_13 = [6659270989588988724_i64,1728286997519511358_i64,(-5095475861784307926_i64)];
RET.fld3 = 27456_u16 as f64;
_7 = _4;
RET.fld0 = !12565375147425354485199694691080945852_i128;
_10 = -_1;
RET.fld0 = 116612383577823236199776588929091265753_i128;
RET.fld1 = 9710596538995023270_usize & 17044418533631460849_usize;
_12 = 1999672538_u32;
_8 = RET.fld1 as isize;
RET.fld4 = [18963_i16,8083_i16];
_8 = _10 * _4;
_12 = RET.fld0 as u32;
_7 = (-4156727980671914092_i64) as isize;
match RET.fld0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
116612383577823236199776588929091265753 => bb9,
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
RET.fld0 = 45309789206370638374159062917735577091_i128;
match RET.fld0 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb5,
45309789206370638374159062917735577091 => bb10,
_ => bb6
}
}
bb10 = {
RET.fld0 = -1469016698588486546871049624798786710_i128;
RET.fld3 = _2 as f64;
RET.fld3 = _5 as f64;
RET.fld1 = 2_usize;
_14 = _4 as u64;
RET.fld1 = 7530064266367453658_usize << _6;
RET.fld3 = 16309_i16 as f64;
RET.fld4 = [31828_i16,(-27582_i16)];
RET.fld1 = 5_usize >> _8;
RET.fld4 = [27044_i16,(-14174_i16)];
RET.fld0 = 43065340306640802227424924145479220303_i128 * (-114603922595072288965697124531993761654_i128);
_6 = !_3;
RET.fld0 = 6608_i16 as i128;
_3 = 89_u8 as isize;
_11 = [_6,_10,_1];
RET.fld1 = 174_u8 as usize;
RET.fld3 = _12 as f64;
_2 = _14 * _14;
_12 = 626579540_u32;
RET.fld3 = RET.fld1 as f64;
RET.fld3 = 212340296866791648732961375261767453955_u128 as f64;
match _12 {
0 => bb5,
1 => bb2,
2 => bb7,
626579540 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_6 = _1;
RET.fld3 = (-1047625451_i32) as f64;
_6 = 67238163865206745496494136989072337297_u128 as isize;
RET.fld0 = (-42513235341103784978890396454818818870_i128) & 50667659329786347384466478572542494486_i128;
_15 = RET.fld1;
_9 = [_5,_4,_10];
match _12 {
0 => bb11,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb10,
5 => bb13,
6 => bb14,
626579540 => bb16,
_ => bb15
}
}
bb13 = {
Return()
}
bb14 = {
RET.fld4 = [27670_i16,1147_i16];
_13 = [4781978099138139818_i64,5023099476356969096_i64,(-6258556240301398308_i64)];
_7 = true as isize;
RET.fld1 = (-3192638078020237059_i64) as usize;
RET.fld3 = 182_u8 as f64;
_13 = [6659270989588988724_i64,1728286997519511358_i64,(-5095475861784307926_i64)];
RET.fld3 = 27456_u16 as f64;
_7 = _4;
RET.fld0 = !12565375147425354485199694691080945852_i128;
_10 = -_1;
RET.fld0 = 116612383577823236199776588929091265753_i128;
RET.fld1 = 9710596538995023270_usize & 17044418533631460849_usize;
_12 = 1999672538_u32;
_8 = RET.fld1 as isize;
RET.fld4 = [18963_i16,8083_i16];
_8 = _10 * _4;
_12 = RET.fld0 as u32;
_7 = (-4156727980671914092_i64) as isize;
match RET.fld0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
116612383577823236199776588929091265753 => bb9,
_ => bb8
}
}
bb15 = {
RET.fld0 = 45309789206370638374159062917735577091_i128;
match RET.fld0 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb5,
45309789206370638374159062917735577091 => bb10,
_ => bb6
}
}
bb16 = {
_6 = RET.fld3 as isize;
_15 = RET.fld1 ^ RET.fld1;
_12 = 32975_u16 as u32;
_6 = -_5;
_11 = [_6,_4,_8];
_8 = _2 as isize;
Call(RET.fld2 = fn16(_8, _6, _6, _4), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
_7 = _4 << _10;
RET.fld0 = 83416714981050217055684437049513033002_i128;
RET.fld4 = [12408_i16,5185_i16];
RET.fld3 = 27599_i16 as f64;
_13 = [(-8536013874074492839_i64),1985841927411627954_i64,(-2981375079212012816_i64)];
RET.fld0 = (-98589089055617495647807626848442523741_i128) * (-135016200163334063021348342808265682554_i128);
_12 = !2080873963_u32;
_20.fld0 = _2 - _14;
_15 = 23891_i16 as usize;
_19 = _10;
_13 = [2150520376077351685_i64,5630967473701765063_i64,(-962037210507065553_i64)];
_3 = _19;
RET.fld1 = _15;
RET.fld1 = _15 - _15;
RET.fld0 = (-92766899064429529478526031255987702893_i128) + 122552493215501714290951777353858193065_i128;
_13 = [657860268300021772_i64,2445636614363757401_i64,(-7079693429136237496_i64)];
_13 = [2823586790325289434_i64,(-2616922255543108228_i64),(-7995665121536897764_i64)];
RET.fld0 = (-84889428215555925802503002502933324921_i128) | 7443356908101672846147208648650273139_i128;
_5 = _19 << _10;
_9 = _11;
RET.fld4 = [8226_i16,5643_i16];
_15 = _12 as usize;
_20 = Adt42 { fld0: _14 };
Goto(bb18)
}
bb18 = {
Call(_23 = dump_var(15_usize, 1_usize, Move(_1), 10_usize, Move(_10), 4_usize, Move(_4), 9_usize, Move(_9)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_23 = dump_var(15_usize, 11_usize, Move(_11), 14_usize, Move(_14), 6_usize, Move(_6), 19_usize, Move(_19)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize) -> *mut u128 {
mir! {
type RET = *mut u128;
let _5: i128;
let _6: [i64; 3];
let _7: f64;
let _8: *const f32;
let _9: f32;
let _10: ([i32; 1], [char; 1]);
let _11: isize;
let _12: [char; 1];
let _13: (i32,);
let _14: f32;
let _15: *mut u16;
let _16: (i32,);
let _17: Adt58;
let _18: Adt48;
let _19: char;
let _20: char;
let _21: Adt42;
let _22: bool;
let _23: Adt50;
let _24: i128;
let _25: f32;
let _26: (char,);
let _27: f32;
let _28: bool;
let _29: char;
let _30: char;
let _31: Adt54;
let _32: Adt53;
let _33: u32;
let _34: (i32, i64, u128, i64, f32);
let _35: f64;
let _36: Adt50;
let _37: bool;
let _38: Adt51;
let _39: isize;
let _40: [i16; 1];
let _41: i64;
let _42: *mut u16;
let _43: [i32; 1];
let _44: ([i32; 1], [char; 1]);
let _45: &'static u8;
let _46: bool;
let _47: bool;
let _48: u8;
let _49: isize;
let _50: i32;
let _51: isize;
let _52: Adt42;
let _53: ();
let _54: ();
{
_1 = _2 - _2;
_4 = _3 << _2;
_1 = _2 - _2;
_3 = _2;
_6 = [(-250268178769745177_i64),8650643098822174323_i64,9140159417201468392_i64];
_2 = _3;
_6 = [3666389651381330403_i64,(-6693444684005930803_i64),(-793562246530837976_i64)];
_3 = _1;
_6 = [273007744924527949_i64,(-1644562730126964230_i64),2893156924919643441_i64];
_5 = (-21591_i16) as i128;
_2 = -_3;
_2 = _4;
_6 = [7198168146585326644_i64,2360930282610496215_i64,(-3197815932306843182_i64)];
_1 = _2;
_4 = _1 >> _2;
_6 = [(-4309146519379801635_i64),(-594950033363476082_i64),6821730233442219889_i64];
_1 = 14566893162418574359_u64 as isize;
_6 = [(-4937824237856572969_i64),2916224399074720029_i64,7407717421869716308_i64];
_6 = [7634393444406401535_i64,196337923538570373_i64,(-6846864522608576058_i64)];
_3 = _2 << _4;
_1 = _4;
_6 = [(-5471157028972475224_i64),(-4584408505860581892_i64),3323082508931043982_i64];
_5 = -19013704332495474019058147917676495967_i128;
_3 = (-9111_i16) as isize;
_3 = 29710_u16 as isize;
_3 = -_4;
_2 = !_4;
_7 = (-1424164355_i32) as f64;
Goto(bb1)
}
bb1 = {
_6 = [(-4960724442063964298_i64),(-1021590427567145505_i64),5018657458740763144_i64];
_7 = 7132219913889208373_i64 as f64;
_5 = 70842546188017567399798685326234231171_i128;
match _5 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
70842546188017567399798685326234231171 => bb10,
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
_2 = !_4;
_7 = 58706_u16 as f64;
_9 = 118710410152482230051172883279086555272_u128 as f32;
_3 = !_1;
_5 = _7 as i128;
_3 = false as isize;
_2 = !_4;
_5 = (-159694571305042053174487700145657789928_i128) & (-163203677477618846844709182858857428247_i128);
_3 = _2;
_1 = 27_i8 as isize;
_5 = -29101037612074255163612997379276016247_i128;
_11 = _4 & _3;
Goto(bb11)
}
bb11 = {
_10.0 = [132069834_i32];
_8 = core::ptr::addr_of!(_9);
_8 = core::ptr::addr_of!(_9);
_12 = ['\u{9ecbf}'];
_13 = ((-155067741_i32),);
_5 = _13.0 as i128;
_13.0 = 234168200_i32 >> _2;
_2 = _5 as isize;
_10.1 = ['\u{6c9e2}'];
_13.0 = (-732782016_i32) >> _3;
_3 = 98_u8 as isize;
_9 = 88_u8 as f32;
_7 = _5 as f64;
_6 = [1247661823077459373_i64,(-2635822753034210648_i64),4161228120610425934_i64];
_14 = (*_8) - _9;
(*_8) = 5_usize as f32;
_11 = _4;
_12 = ['\u{5b5ad}'];
_9 = _14 - _14;
_2 = _11;
_12 = ['\u{e071a}'];
_10.0 = [_13.0];
_13 = ((-1022153765_i32),);
_10.1 = _12;
match _13.0 {
340282366920938463463374607430746057691 => bb12,
_ => bb3
}
}
bb12 = {
_4 = _2;
match _13.0 {
0 => bb5,
1 => bb8,
2 => bb11,
3 => bb13,
340282366920938463463374607430746057691 => bb15,
_ => bb14
}
}
bb13 = {
_10.0 = [132069834_i32];
_8 = core::ptr::addr_of!(_9);
_8 = core::ptr::addr_of!(_9);
_12 = ['\u{9ecbf}'];
_13 = ((-155067741_i32),);
_5 = _13.0 as i128;
_13.0 = 234168200_i32 >> _2;
_2 = _5 as isize;
_10.1 = ['\u{6c9e2}'];
_13.0 = (-732782016_i32) >> _3;
_3 = 98_u8 as isize;
_9 = 88_u8 as f32;
_7 = _5 as f64;
_6 = [1247661823077459373_i64,(-2635822753034210648_i64),4161228120610425934_i64];
_14 = (*_8) - _9;
(*_8) = 5_usize as f32;
_11 = _4;
_12 = ['\u{5b5ad}'];
_9 = _14 - _14;
_2 = _11;
_12 = ['\u{e071a}'];
_10.0 = [_13.0];
_13 = ((-1022153765_i32),);
_10.1 = _12;
match _13.0 {
340282366920938463463374607430746057691 => bb12,
_ => bb3
}
}
bb14 = {
Return()
}
bb15 = {
_12 = ['\u{ab143}'];
_11 = _7 as isize;
Call(_3 = core::intrinsics::transmute(_2), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_9 = -_14;
_3 = !_2;
_13.0 = 1871085105_i32 << _3;
(*_8) = 11365739147448944451_u64 as f32;
_3 = true as isize;
_16.0 = 142_u8 as i32;
_14 = -_9;
_8 = core::ptr::addr_of!(_14);
_3 = !_4;
_14 = -_9;
_8 = core::ptr::addr_of!((*_8));
_1 = _2 ^ _2;
_2 = 135_u8 as isize;
_14 = _9 - _9;
_7 = 8_u8 as f64;
_2 = 549258728_u32 as isize;
_19 = '\u{1383a}';
_9 = _14 - (*_8);
_21 = Adt42 { fld0: 13447380893432922079_u64 };
_10.1 = _12;
_10.0 = [_13.0];
_11 = _1;
_1 = _11 & _11;
_20 = _19;
_5 = (-26_i8) as i128;
_18 = Adt48::Variant0 { fld0: _10.0,fld1: _20 };
Goto(bb17)
}
bb17 = {
_4 = _11;
_1 = _19 as isize;
_10.1 = [Field::<char>(Variant(_18, 0), 1)];
_5 = 234273057076364361565875234299076487270_u128 as i128;
place!(Field::<[i32; 1]>(Variant(_18, 0), 0)) = [_13.0];
_13.0 = _16.0;
SetDiscriminant(_18, 0);
(*_8) = _9;
_11 = !_3;
_21 = Adt42 { fld0: 16730357379282503390_u64 };
_21 = Adt42 { fld0: 15127719382048858201_u64 };
match _21.fld0 {
0 => bb18,
1 => bb19,
2 => bb20,
3 => bb21,
4 => bb22,
5 => bb23,
15127719382048858201 => bb25,
_ => bb24
}
}
bb18 = {
_9 = -_14;
_3 = !_2;
_13.0 = 1871085105_i32 << _3;
(*_8) = 11365739147448944451_u64 as f32;
_3 = true as isize;
_16.0 = 142_u8 as i32;
_14 = -_9;
_8 = core::ptr::addr_of!(_14);
_3 = !_4;
_14 = -_9;
_8 = core::ptr::addr_of!((*_8));
_1 = _2 ^ _2;
_2 = 135_u8 as isize;
_14 = _9 - _9;
_7 = 8_u8 as f64;
_2 = 549258728_u32 as isize;
_19 = '\u{1383a}';
_9 = _14 - (*_8);
_21 = Adt42 { fld0: 13447380893432922079_u64 };
_10.1 = _12;
_10.0 = [_13.0];
_11 = _1;
_1 = _11 & _11;
_20 = _19;
_5 = (-26_i8) as i128;
_18 = Adt48::Variant0 { fld0: _10.0,fld1: _20 };
Goto(bb17)
}
bb19 = {
Return()
}
bb20 = {
Return()
}
bb21 = {
Return()
}
bb22 = {
_4 = _2;
match _13.0 {
0 => bb5,
1 => bb8,
2 => bb11,
3 => bb13,
340282366920938463463374607430746057691 => bb15,
_ => bb14
}
}
bb23 = {
Return()
}
bb24 = {
Return()
}
bb25 = {
_3 = _4 + _4;
_18 = Adt48::Variant0 { fld0: _10.0,fld1: _20 };
_2 = _4 & _4;
_14 = _9;
_26 = (_19,);
_27 = (*_8) * (*_8);
(*_8) = -_9;
_5 = (-103807884764103810735200680218273267597_i128) - 120699888135317990421112380692289895378_i128;
place!(Field::<char>(Variant(_18, 0), 1)) = _19;
place!(Field::<char>(Variant(_18, 0), 1)) = _26.0;
match _21.fld0 {
0 => bb1,
1 => bb15,
2 => bb6,
3 => bb26,
4 => bb27,
5 => bb28,
6 => bb29,
15127719382048858201 => bb31,
_ => bb30
}
}
bb26 = {
Return()
}
bb27 = {
Return()
}
bb28 = {
_12 = ['\u{ab143}'];
_11 = _7 as isize;
Call(_3 = core::intrinsics::transmute(_2), ReturnTo(bb16), UnwindUnreachable())
}
bb29 = {
Return()
}
bb30 = {
Return()
}
bb31 = {
_16.0 = _2 as i32;
_2 = _3 >> _3;
place!(Field::<char>(Variant(_18, 0), 1)) = _19;
_25 = _9;
_14 = _27;
_27 = -_14;
Call(_11 = core::intrinsics::bswap(_3), ReturnTo(bb32), UnwindUnreachable())
}
bb32 = {
_2 = 2_usize as isize;
_13.0 = _16.0 ^ _16.0;
_9 = 2412068843_u32 as f32;
place!(Field::<[i32; 1]>(Variant(_18, 0), 0)) = [_16.0];
_11 = 39_u8 as isize;
_28 = !false;
_14 = _27 * _27;
_29 = _19;
_5 = (-151447166517420368130133249483190370273_i128) ^ 139663447438887189722390364457085000739_i128;
_12 = _10.1;
_6 = [4021504556709097125_i64,1310839482962087833_i64,(-1492240361669415002_i64)];
_30 = Field::<char>(Variant(_18, 0), 1);
place!(Field::<[i32; 1]>(Variant(_18, 0), 0)) = [_13.0];
_23 = Adt50::Variant1 { fld0: 2436164086_u32 };
place!(Field::<[i32; 1]>(Variant(_18, 0), 0)) = [_16.0];
(*_8) = _9 * _25;
_27 = _14 + (*_8);
_6 = [5428593476175247323_i64,8410719359322596217_i64,4690006462269692743_i64];
_18 = Adt48::Variant0 { fld0: _10.0,fld1: _19 };
_21.fld0 = 10_u8 as u64;
_3 = _4 | _4;
_34.4 = _25 * _25;
_24 = _5;
Goto(bb33)
}
bb33 = {
_21.fld0 = 14516113828001335440_u64;
_20 = _19;
_33 = 3884960780_u32;
_25 = (*_8) + _14;
_29 = _26.0;
SetDiscriminant(_18, 1);
_24 = _5;
_12 = [_29];
_20 = _29;
_10.0 = [_16.0];
_34 = (_16.0, (-984537353991879974_i64), 232444366991255391818933620250926985105_u128, (-8435288788031984989_i64), _27);
place!(Field::<(i32, i64, u128, i64, f32)>(Variant(_18, 1), 0)).3 = _34.3;
place!(Field::<(i32, i64, u128, i64, f32)>(Variant(_18, 1), 0)).2 = !_34.2;
_40 = [7832_i16];
_9 = (*_8);
_14 = _25 * _9;
_7 = 134_u8 as f64;
match _34.2 {
0 => bb8,
232444366991255391818933620250926985105 => bb35,
_ => bb34
}
}
bb34 = {
_6 = [(-4960724442063964298_i64),(-1021590427567145505_i64),5018657458740763144_i64];
_7 = 7132219913889208373_i64 as f64;
_5 = 70842546188017567399798685326234231171_i128;
match _5 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
70842546188017567399798685326234231171 => bb10,
_ => bb9
}
}
bb35 = {
_19 = _29;
_20 = _19;
_28 = true & false;
_3 = _28 as isize;
place!(Field::<(i32, i64, u128, i64, f32)>(Variant(_18, 1), 0)) = (_13.0, _34.3, _34.2, _34.3, _27);
_41 = !_34.3;
_22 = !_28;
_37 = !_22;
_34.3 = _41 + _41;
_28 = _37 & _37;
_34.2 = _13.0 as u128;
place!(Field::<(i32, i64, u128, i64, f32)>(Variant(_18, 1), 0)).2 = _34.2;
_29 = _20;
Goto(bb36)
}
bb36 = {
_28 = _37;
_34 = Field::<(i32, i64, u128, i64, f32)>(Variant(_18, 1), 0);
_14 = _27;
_6 = [_41,_34.1,Field::<(i32, i64, u128, i64, f32)>(Variant(_18, 1), 0).1];
_26.0 = _30;
_21 = Adt42 { fld0: 14514393315832363446_u64 };
_11 = _4;
_16 = _13;
RET = core::ptr::addr_of_mut!(_34.2);
_46 = !_37;
_3 = _4;
(*_8) = _27;
_14 = -_27;
_7 = _16.0 as f64;
_16 = (_34.0,);
_44.1 = [_20];
_43 = [_13.0];
place!(Field::<*const u8>(Variant(_18, 1), 1)) = core::ptr::addr_of!(_48);
_3 = _11;
_45 = &_48;
_44.0 = [_34.0];
_36 = Adt50::Variant1 { fld0: _33 };
_34.0 = !_16.0;
(*RET) = !Field::<(i32, i64, u128, i64, f32)>(Variant(_18, 1), 0).2;
Goto(bb37)
}
bb37 = {
Call(_53 = dump_var(16_usize, 1_usize, Move(_1), 30_usize, Move(_30), 5_usize, Move(_5), 20_usize, Move(_20)), ReturnTo(bb38), UnwindUnreachable())
}
bb38 = {
Call(_53 = dump_var(16_usize, 22_usize, Move(_22), 41_usize, Move(_41), 24_usize, Move(_24), 12_usize, Move(_12)), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
Call(_53 = dump_var(16_usize, 43_usize, Move(_43), 29_usize, Move(_29), 11_usize, Move(_11), 33_usize, Move(_33)), ReturnTo(bb40), UnwindUnreachable())
}
bb40 = {
Call(_53 = dump_var(16_usize, 2_usize, Move(_2), 54_usize, _54, 54_usize, _54, 54_usize, _54), ReturnTo(bb41), UnwindUnreachable())
}
bb41 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: i32,mut _8: isize,mut _9: isize,mut _10: isize) -> *const f32 {
mir! {
type RET = *const f32;
let _11: i32;
let _12: *mut i64;
let _13: (i32,);
let _14: u8;
let _15: [i16; 1];
let _16: [isize; 3];
let _17: Adt48;
let _18: [i16; 2];
let _19: [i64; 3];
let _20: [i16; 7];
let _21: Adt54;
let _22: f64;
let _23: i64;
let _24: u32;
let _25: *const i32;
let _26: ([i32; 1], [char; 1]);
let _27: char;
let _28: f32;
let _29: (i32,);
let _30: (char,);
let _31: ();
let _32: ();
{
_7 = (-1015480480_i32) | (-1493647887_i32);
_10 = -_9;
_13 = (_7,);
_11 = 6689839707750045281_usize as i32;
_1 = _2;
_13 = (_7,);
_4 = 10915_i16 as isize;
_13 = (_7,);
_1 = '\u{7d376}' as isize;
_5 = -_10;
_14 = !83_u8;
_15 = [6445_i16];
_3 = 64544_u16 as isize;
_11 = _7 | _13.0;
_13.0 = _11;
_4 = -_10;
_6 = _4;
_14 = 120_u8 | 136_u8;
_8 = _6 | _5;
_16 = [_9,_10,_4];
_8 = _9 + _10;
Goto(bb1)
}
bb1 = {
Call(_9 = core::intrinsics::transmute(_6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = 4899087676186925433_usize as i32;
_6 = -_2;
_15 = [10940_i16];
_2 = (-65204369408897818798671266050949949801_i128) as isize;
_14 = 46_u8;
_14 = 132_u8;
_1 = !_8;
_9 = 329570455354966925907692699068975950167_u128 as isize;
_18 = [(-25959_i16),(-22018_i16)];
_18 = [(-25304_i16),(-21376_i16)];
_5 = -_1;
_2 = _14 as isize;
_16 = [_1,_5,_4];
_8 = true as isize;
_7 = 9256045334664913760_u64 as i32;
match _14 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
132 => bb10,
_ => bb9
}
}
bb3 = {
Call(_9 = core::intrinsics::transmute(_6), ReturnTo(bb2), UnwindUnreachable())
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
_14 = 7_u8;
_10 = _6 * _5;
_11 = _13.0;
_13 = (_11,);
match _14 {
0 => bb11,
1 => bb12,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
7 => bb18,
_ => bb17
}
}
bb11 = {
Call(_9 = core::intrinsics::transmute(_6), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_7 = 4899087676186925433_usize as i32;
_6 = -_2;
_15 = [10940_i16];
_2 = (-65204369408897818798671266050949949801_i128) as isize;
_14 = 46_u8;
_14 = 132_u8;
_1 = !_8;
_9 = 329570455354966925907692699068975950167_u128 as isize;
_18 = [(-25959_i16),(-22018_i16)];
_18 = [(-25304_i16),(-21376_i16)];
_5 = -_1;
_2 = _14 as isize;
_16 = [_1,_5,_4];
_8 = true as isize;
_7 = 9256045334664913760_u64 as i32;
match _14 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
132 => bb10,
_ => bb9
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
Call(_9 = core::intrinsics::transmute(_6), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
_14 = !145_u8;
Goto(bb19)
}
bb19 = {
_19 = [(-3909448937901750145_i64),4870803708845889285_i64,2930395801258168361_i64];
_18 = [(-17286_i16),7616_i16];
_14 = !27_u8;
_13.0 = !_11;
_12 = core::ptr::addr_of_mut!(_23);
_2 = _5;
_22 = _13.0 as f64;
_24 = 759367243_u32;
_20 = [10269_i16,120_i16,(-24824_i16),8739_i16,(-23958_i16),(-14863_i16),30003_i16];
_12 = core::ptr::addr_of_mut!((*_12));
_3 = _4 + _2;
_9 = _3;
_3 = 11355868241127160101_usize as isize;
_25 = core::ptr::addr_of!(_7);
_26.0 = [_11];
_10 = !_2;
_18 = [(-15644_i16),(-15607_i16)];
_14 = 54_u8 >> _2;
_22 = 8880_i16 as f64;
_14 = 45_u8;
_26.1 = ['\u{b9515}'];
(*_12) = !2328987985163464570_i64;
_17 = Adt48::Variant0 { fld0: _26.0,fld1: '\u{bc516}' };
_15 = [(-12136_i16)];
Goto(bb20)
}
bb20 = {
_26.0 = [_13.0];
_27 = '\u{6764b}';
_5 = _1 >> _1;
_25 = core::ptr::addr_of!((*_25));
_9 = (-107_i8) as isize;
_4 = _1;
(*_12) = _22 as i64;
_10 = -_6;
_13 = ((*_25),);
_28 = 12399329300429187813_usize as f32;
_10 = -_5;
_11 = -_13.0;
_28 = 7059077033535284563_u64 as f32;
RET = core::ptr::addr_of!(_28);
_13.0 = (*_25) << _6;
RET = core::ptr::addr_of!(_28);
_6 = _5 & _5;
_22 = (-17722_i16) as f64;
Goto(bb21)
}
bb21 = {
Call(_31 = dump_var(17_usize, 7_usize, Move(_7), 24_usize, Move(_24), 14_usize, Move(_14), 16_usize, Move(_16)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_31 = dump_var(17_usize, 27_usize, Move(_27), 19_usize, Move(_19), 9_usize, Move(_9), 4_usize, Move(_4)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_31 = dump_var(17_usize, 5_usize, Move(_5), 8_usize, Move(_8), 15_usize, Move(_15), 32_usize, _32), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{c7a22}'), std::hint::black_box(101_isize), std::hint::black_box((-78_i8)), std::hint::black_box((-26579_i16)), std::hint::black_box((-372726591_i32)), std::hint::black_box((-8508499456393573313_i64)), std::hint::black_box((-113080371074433175207860892393645643044_i128)), std::hint::black_box(2_usize), std::hint::black_box(199_u8), std::hint::black_box(38191_u16), std::hint::black_box(825855338_u32), std::hint::black_box(17287692192907862720_u64), std::hint::black_box(259990701066231106371175837248714725018_u128));
                
            }
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt42{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt42 {
fld0: u64,
}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: ([i16; 1], i32),
fld1: (char,),
fld2: *mut i64,
fld3: [i16; 7],
fld4: [i16; 1],
fld5: f64,
fld6: *mut u128,
fld7: *const f32,

},
Variant1{
fld0: (i32,),
fld1: [i64; 2],
fld2: u128,
fld3: Adt42,

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
fld0: (i32, i64, u128, i64, f32),
fld1: *const *mut u128,
fld2: f64,
fld3: *const u8,
fld4: i16,
fld5: i32,
fld6: [i64; 3],

},
Variant1{
fld0: bool,
fld1: f64,
fld2: *mut u16,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: ([i32; 1], [char; 1]),
fld1: usize,

},
Variant1{
fld0: (i32, i64, u128, i64, f32),
fld1: [i16; 1],
fld2: ([i32; 1], [char; 1]),

},
Variant2{
fld0: (i32, i64, u128, i64, f32),
fld1: usize,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: [i64; 3],
fld1: *const i32,
fld2: isize,
fld3: u16,
fld4: *const (i32,),
fld5: usize,
fld6: Adt44,

},
Variant1{
fld0: [char; 1],
fld1: Adt45,
fld2: ([i32; 1], [char; 1]),
fld3: (*const i32, u16, [isize; 3], i8),
fld4: i16,
fld5: (char,),
fld6: *mut u128,

},
Variant2{
fld0: ([i32; 1], [char; 1]),
fld1: *mut u128,

},
Variant3{
fld0: [i16; 1],
fld1: *const f32,
fld2: isize,
fld3: usize,
fld4: [i64; 2],
fld5: i32,
fld6: ([i16; 1], i32),
fld7: [isize; 3],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: *const *mut u128,
fld1: (*const i32, u16, [isize; 3], i8),
fld2: Adt44,

},
Variant1{
fld0: *const u8,
fld1: *mut u16,
fld2: *const *mut u128,
fld3: (*const i32, u16, [isize; 3], i8),

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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: [i32; 1],
fld1: char,

},
Variant1{
fld0: (i32, i64, u128, i64, f32),
fld1: *const u8,
fld2: u32,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: i128,
fld1: usize,
fld2: *mut u128,
fld3: f64,
fld4: [i16; 2],
}
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: u16,
fld1: *const f32,
fld2: Adt48,
fld3: Adt42,
fld4: Adt49,

},
Variant1{
fld0: u32,

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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: ([i32; 1], [char; 1]),
fld1: f64,
fld2: isize,
fld3: *mut i64,
fld4: *const *mut u128,
fld5: [i16; 2],

},
Variant1{
fld0: u128,
fld1: u64,
fld2: [i32; 1],
fld3: [i16; 1],

},
Variant2{
fld0: bool,
fld1: Adt49,
fld2: [i16; 7],
fld3: *mut i64,
fld4: i16,
fld5: Adt45,
fld6: ([i32; 1], [char; 1]),
fld7: *const *mut u128,

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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: (char,),
fld1: i128,
fld2: f32,
fld3: *const u8,
fld4: Adt44,

},
Variant1{
fld0: *const *const i32,
fld1: i8,

},
Variant2{
fld0: isize,
fld1: Adt50,

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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: [i16; 2],
fld1: (i32, i64, u128, i64, f32),
fld2: Adt48,

},
Variant1{
fld0: (*const i32, u16, [isize; 3], i8),

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: Adt53,

},
Variant1{
fld0: u8,
fld1: Adt44,
fld2: [isize; 3],
fld3: *const *mut u128,
fld4: *const i32,

},
Variant2{
fld0: *const f32,

},
Variant3{
fld0: bool,
fld1: (*const i32, u16, [isize; 3], i8),
fld2: *const u8,
fld3: Adt48,
fld4: u8,
fld5: *const *const i32,
fld6: Adt53,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: Adt48,
fld1: u16,
fld2: *const f32,
fld3: *mut i64,
fld4: ([i32; 1], [char; 1]),

},
Variant1{
fld0: bool,
fld1: ([i32; 1], [char; 1]),
fld2: Adt42,
fld3: u16,
fld4: u8,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: *const *const i32,
fld1: i32,
fld2: *const f32,
fld3: [i16; 1],
fld4: i16,

},
Variant1{
fld0: [isize; 3],
fld1: [i16; 7],
fld2: i128,
fld3: Adt51,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: Adt52,
fld1: u128,
fld2: (i32,),

},
Variant1{
fld0: *const *mut u128,
fld1: Adt46,
fld2: (i32,),
fld3: [i64; 2],
fld4: [i16; 1],
fld5: *const *const i32,
fld6: u32,
fld7: [i16; 7],

},
Variant2{
fld0: [i32; 1],
fld1: [i64; 3],

},
Variant3{
fld0: bool,
fld1: Adt50,
fld2: ([i16; 1], i32),
fld3: [char; 1],
fld4: *const *mut u128,
fld5: Adt44,
fld6: Adt53,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: *const i32,

},
Variant1{
fld0: (i32,),
fld1: [i64; 2],
fld2: [i16; 2],
fld3: u64,
fld4: *const i32,
fld5: u128,
fld6: [i32; 1],

}}

