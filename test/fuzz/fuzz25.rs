#![recursion_limit = "1024"]
    #![feature(custom_mir, core_intrinsics)]
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
            printf(c"%i".as_ptr(),*self as i8 as c_int);
        }
    }
    impl PrintFDebug for u8{
        unsafe fn printf_debug(&self){
            printf(c"%u".as_ptr(),*self as u8 as c_int);
        }
    } 
    impl PrintFDebug for i16{
        unsafe fn printf_debug(&self){
            printf(c"%i".as_ptr(),*self as i16 as c_int);
        }
    }
    impl PrintFDebug for u16{
        unsafe fn printf_debug(&self){
            printf(c"%u".as_ptr(),*self as u16 as c_int);
        }
    } 
    impl PrintFDebug for i32{
        unsafe fn printf_debug(&self){
            printf(c"%i".as_ptr(),*self);
        }
    }
    impl PrintFDebug for f32{
        unsafe fn printf_debug(&self){
            printf(c"%f".as_ptr(),*self as core::ffi::c_double);
        }
    }
    impl PrintFDebug for f64{
        unsafe fn printf_debug(&self){
            printf(c"%f".as_ptr(),*self as core::ffi::c_double);
        }
    }
    impl<T:PrintFDebug,const N:usize> PrintFDebug for [T;N]{
        unsafe fn printf_debug(&self){
            printf(c"[".as_ptr());
            for b in self{
                b.printf_debug();
                printf(c",".as_ptr());
            }
            printf(c"]".as_ptr());
        }
    }
    impl PrintFDebug for u32{
        unsafe fn printf_debug(&self){
            printf(c"%u".as_ptr(),*self);
        }
    } 
    impl PrintFDebug for char{
        unsafe fn printf_debug(&self){
            printf(c"%u".as_ptr(),*self as u64);
        }
    } 
    impl PrintFDebug for i64{
        unsafe fn printf_debug(&self){
            printf(c"%li".as_ptr(),*self);
        }
    }
    impl PrintFDebug for u64{
        unsafe fn printf_debug(&self){
            printf(c"%lu".as_ptr(),*self);
        }
    } 
    impl PrintFDebug for i128{
        unsafe fn printf_debug(&self){
            u128::printf_debug(&(*self as u128));
        }
    } 
    impl PrintFDebug for u128{
        unsafe fn printf_debug(&self){
            printf(c"%lx%lx".as_ptr(), (*self >> 64) as u64,*self as u64);
        }
    } 
    impl PrintFDebug for isize{
        unsafe fn printf_debug(&self){
            printf(c"%li".as_ptr(),*self as isize);
        }
    }
    impl PrintFDebug for usize{
        unsafe fn printf_debug(&self){
            printf(c"%lu".as_ptr(),*self as usize);
        }
    } 
    impl PrintFDebug for bool{
        unsafe fn printf_debug(&self){
            if *self{
                printf(c"true".as_ptr());
            }
            else{
                printf(c"false".as_ptr());
            }
        }
    } 
    impl PrintFDebug for (){
        unsafe fn printf_debug(&self){
            printf(c"()".as_ptr());
        }
    } 
    impl<A:PrintFDebug> PrintFDebug for (A,){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",)".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug> PrintFDebug for (A,B){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug> PrintFDebug for (A,B,C){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug> PrintFDebug for (A,B,C,D){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug> PrintFDebug for (A,B,C,D,E){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug> PrintFDebug for (A,B,C,D,E,F){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c",".as_ptr());
            self.8.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c",".as_ptr());
            self.8.printf_debug();
            printf(c",".as_ptr());
            self.9.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug,K:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J,K){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c",".as_ptr());
            self.8.printf_debug();
            printf(c",".as_ptr());
            self.9.printf_debug();
            printf(c",".as_ptr());
            self.10.printf_debug();
            printf(c")".as_ptr());
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug,K:PrintFDebug,L:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J,K,L){
        unsafe fn printf_debug(&self){
            printf(c"(".as_ptr());
            self.0.printf_debug();
            printf(c",".as_ptr());
            self.1.printf_debug();
            printf(c",".as_ptr());
            self.2.printf_debug();
            printf(c",".as_ptr());
            self.3.printf_debug();
            printf(c",".as_ptr());
            self.4.printf_debug();
            printf(c",".as_ptr());
            self.5.printf_debug();
            printf(c",".as_ptr());
            self.6.printf_debug();
            printf(c",".as_ptr());
            self.7.printf_debug();
            printf(c",".as_ptr());
            self.8.printf_debug();
            printf(c",".as_ptr());
            self.9.printf_debug();
            printf(c",".as_ptr());
            self.10.printf_debug();
            printf(c",".as_ptr());
            self.11.printf_debug();
            printf(c")".as_ptr());
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
            printf(c"fn%u:_%u = ".as_ptr(),f,var0);
            val0.printf_debug();
            printf(c"\n_%u = ".as_ptr(),var1);
            val1.printf_debug();
            printf(c"\n_%u = ".as_ptr(),var2);
            val2.printf_debug();
            printf(c"\n_%u = ".as_ptr(),var3);
            val3.printf_debug();
            printf(c"\n".as_ptr());
        }
    }
    #[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn0(mut _1: bool,mut _2: char,mut _3: u128,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: u64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16) -> (u32, u16, f64) {
mir! {
type RET = (u32, u16, f64);
let _12: Adt56;
let _13: Adt55;
let _14: Adt53;
let _15: i64;
let _16: ((f32, isize), i128, usize);
let _17: (i128,);
let _18: *mut (f32, isize);
let _19: f32;
let _20: [i16; 1];
let _21: (bool,);
let _22: isize;
let _23: Adt58;
let _24: isize;
let _25: (i32, *mut i32, u32, i32);
let _26: char;
let _27: i64;
let _28: &'static i128;
let _29: ();
let _30: ();
{
_11 = 58327_u16;
_13.fld2.0 = 16222375516418265441_usize;
_13.fld1 = [(-31987_i16)];
_12.fld4 = [17775863113177363861_u64];
RET.1 = (-2959_i16) as u16;
_4 = (-648339799_i32) as i8;
RET.2 = 3905838807_u32 as f64;
_2 = '\u{9568a}';
_13.fld6 = [_4,_4,_4,_4,_4,_4,_4];
_9 = _13.fld2.0 & _13.fld2.0;
_1 = !false;
_13.fld4 = [6105083086265834351_u64];
_13.fld2.0 = !_9;
_14.fld0.2 = _9;
_12.fld4 = _13.fld4;
_12.fld1.0 = 149564699046036054368314391295527244449_i128 as f32;
RET.2 = 13323769839286510973_u64 as f64;
match _11 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
58327 => bb6,
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
_9 = _14.fld0.2 >> _11;
_9 = !_14.fld0.2;
_13.fld2.0 = _14.fld0.2 & _14.fld0.2;
_7 = RET.2 as u64;
_12.fld6.0 = _14.fld0.2 << _9;
_14.fld0.0.1 = 9223372036854775807_isize;
_8 = (-111449608050592545186384091679351765619_i128);
_14.fld1 = 3745553295_u32 as f32;
_12.fld1 = (_14.fld1, _14.fld0.0.1);
RET.0 = 1798809124_u32;
RET.0 = !3214084416_u32;
_12.fld2 = core::ptr::addr_of_mut!(_6);
_12.fld6 = (_9,);
_13.fld1 = [(-11819_i16)];
_12.fld5.0 = !_8;
_12.fld0 = 130_u8 | 133_u8;
_12.fld1.1 = RET.0 as isize;
RET.1 = RET.2 as u16;
_12.fld4 = [_7];
_12.fld4 = [_7];
_3 = !266429728327623442383490004605751634341_u128;
_18 = core::ptr::addr_of_mut!(_16.0);
_16 = (_12.fld1, _8, _12.fld6.0);
_14.fld0.2 = _9 << _14.fld0.0.1;
Goto(bb7)
}
bb7 = {
_13.fld6 = [_4,_4,_4,_4,_4,_4,_4];
_12.fld6 = _13.fld2;
_12.fld6 = (_13.fld2.0,);
_7 = 13385273416157114763_u64 | 12366585225781722153_u64;
_14.fld0.0.1 = 427041842642296108_i64 as isize;
RET.1 = _11 + _11;
_17 = _12.fld5;
_13.fld6 = [_4,_4,_4,_4,_4,_4,_4];
_17.0 = !_16.1;
_11 = 1006309988_i32 as u16;
RET.2 = (-1267540381_i32) as f64;
_14.fld0.0 = (_16.0.0, (*_18).1);
_13.fld6 = [_4,_4,_4,_4,_4,_4,_4];
(*_18).0 = _14.fld0.0.0 + _14.fld1;
_1 = !true;
RET.1 = !_11;
_15 = !2186538186287855007_i64;
match _8 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb4,
4 => bb5,
5 => bb8,
228832758870345918276990515752416445837 => bb10,
_ => bb9
}
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_12.fld0 = 91_u8 & 227_u8;
_12.fld6.0 = _13.fld2.0;
_10 = !_12.fld0;
(*_18).1 = _12.fld1.1;
_13.fld3 = _12.fld6.0 as i8;
Call(_18 = fn1((*_18), RET, RET, _12.fld2, _14.fld0.2, _12.fld1.1, (*_18).0, _13.fld2.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_19 = -_14.fld0.0.0;
_13.fld2 = (_12.fld6.0,);
_23.fld4.0 = RET.2;
_13.fld1 = [(-31536_i16)];
_14.fld0 = (_16.0, _16.1, _12.fld6.0);
match _8 {
0 => bb1,
228832758870345918276990515752416445837 => bb12,
_ => bb3
}
}
bb12 = {
_23.fld3.0 = _3 as f32;
_23.fld4.2 = _13.fld3 - _13.fld3;
_14.fld0.2 = _9;
match _14.fld0.1 {
228832758870345918276990515752416445837 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_12.fld1.1 = _16.0.1;
_13.fld1 = [6371_i16];
_23.fld3 = (_12.fld1.0, _14.fld0.0.1);
_12.fld6.0 = _13.fld2.0 + _13.fld2.0;
_23.fld4.3 = _2;
_14.fld0.2 = _12.fld6.0;
_2 = _23.fld4.3;
_23.fld4.1 = _12.fld5.0 as u8;
_22 = _23.fld3.1 + _23.fld3.1;
_11 = RET.1;
_8 = _16.1 * _16.1;
_13.fld4 = _12.fld4;
_5 = (-17636_i16) >> _23.fld4.1;
_12.fld6.0 = _14.fld0.2 + _13.fld2.0;
_20 = [_5];
_25.2 = !RET.0;
_5 = -(-15064_i16);
_25 = (_6, _12.fld2, RET.0, _6);
_5 = RET.0 as i16;
_21.0 = !_1;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(0_usize, 22_usize, Move(_22), 10_usize, Move(_10), 5_usize, Move(_5), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(0_usize, 1_usize, Move(_1), 7_usize, Move(_7), 8_usize, Move(_8), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: (f32, isize),mut _2: (u32, u16, f64),mut _3: (u32, u16, f64),mut _4: *mut i32,mut _5: usize,mut _6: isize,mut _7: f32,mut _8: usize) -> *mut (f32, isize) {
mir! {
type RET = *mut (f32, isize);
let _9: *const u16;
let _10: Adt49;
let _11: ((f32, isize), i128, usize);
let _12: Adt49;
let _13: bool;
let _14: isize;
let _15: (f32, isize);
let _16: [i16; 1];
let _17: isize;
let _18: *mut u32;
let _19: (bool,);
let _20: isize;
let _21: Adt54;
let _22: [i64; 8];
let _23: bool;
let _24: Adt56;
let _25: u64;
let _26: i16;
let _27: (f64, u8, i8, char);
let _28: Adt58;
let _29: [u128; 6];
let _30: [bool; 7];
let _31: Adt45;
let _32: ();
let _33: ();
{
_2.0 = _3.0;
_3 = _2;
(*_4) = !(-1184337052_i32);
_1 = (_7, _6);
_3.2 = -_2.2;
_4 = core::ptr::addr_of_mut!((*_4));
_9 = core::ptr::addr_of!(_3.1);
_3 = _2;
_3.2 = _2.2 + _2.2;
_2.2 = -_3.2;
_7 = _1.0;
_11.2 = true as usize;
_3.2 = -_2.2;
_10.fld2 = 16334027731085417329794246960455401603_i128;
_3.2 = _2.2;
match _10.fld2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
16334027731085417329794246960455401603 => bb7,
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
_10.fld2 = 6928112056434584067_u64 as i128;
_2.2 = _3.2 * _3.2;
_3.0 = _2.0 + _2.0;
(*_4) = 319944111_i32;
_10.fld3 = Adt45::Variant1 { fld0: _9 };
_3.1 = 13_u8 as u16;
_11.0.1 = !_6;
_1.0 = -_7;
_11.1 = _10.fld2 | _10.fld2;
Call(RET = fn2(_2, _2.2, _11.0.1, _2, _3.0, (*_9), _7, _2.0), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_4 = core::ptr::addr_of_mut!((*_4));
_11.0.0 = -_7;
_9 = Field::<*const u16>(Variant(_10.fld3, 1), 0);
_10.fld0 = [16416590924373605089_u64];
_11.0.1 = 8780745677585486945_u64 as isize;
_2.2 = _3.2;
_3.2 = 11368817105188740855_u64 as f64;
_2 = (_3.0, _3.1, _3.2);
_9 = core::ptr::addr_of!((*_9));
_1 = (_7, _11.0.1);
_2.1 = (*_9) * (*_9);
_3.1 = 10180147107194299880_u64 as u16;
_12.fld2 = _11.1;
_4 = core::ptr::addr_of_mut!((*_4));
_3.1 = _2.1;
_1 = _11.0;
_8 = (*_4) as usize;
(*_4) = 1094992563_i32 << (*_9);
Goto(bb9)
}
bb9 = {
_10.fld2 = _11.1 >> (*_4);
(*_4) = 175919601506500336064215506279398947144_u128 as i32;
RET = core::ptr::addr_of_mut!(_11.0);
(*RET).0 = (-6186661901351390474_i64) as f32;
_12.fld2 = (*RET).1 as i128;
_2 = (_3.0, (*_9), _3.2);
_10.fld1 = [(-2895_i16)];
SetDiscriminant(_10.fld3, 1);
_13 = false;
(*_9) = _2.1 & _2.1;
(*RET).0 = -_1.0;
(*RET).1 = _6 ^ _1.1;
place!(Field::<*const u16>(Variant(_10.fld3, 1), 0)) = core::ptr::addr_of!(_3.1);
_15.1 = 28_u8 as isize;
Goto(bb10)
}
bb10 = {
(*_9) = !_2.1;
_10.fld0 = [8294446793309190038_u64];
_3 = (_2.0, _2.1, _2.2);
_2 = _3;
_19.0 = _2.0 <= _2.0;
_18 = core::ptr::addr_of_mut!(_2.0);
SetDiscriminant(_10.fld3, 2);
Call(_2.0 = core::intrinsics::transmute((*_4)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_6 = (*RET).1 + (*RET).1;
_15 = (*RET);
(*RET).0 = -_7;
place!(Field::<(i32, *mut i32, u32, i32)>(Variant(_10.fld3, 2), 0)).1 = _4;
place!(Field::<(i32, *mut i32, u32, i32)>(Variant(_10.fld3, 2), 0)).3 = (*_4) + (*_4);
_25 = !11881444690796657929_u64;
_3.2 = _2.2 - _2.2;
_10.fld0 = [_25];
place!(Field::<(f64, u8, i8, char)>(Variant(_10.fld3, 2), 7)).1 = 128_u8 * 188_u8;
place!(Field::<(i32, *mut i32, u32, i32)>(Variant(_10.fld3, 2), 0)).2 = !_2.0;
_24.fld4 = [_25];
_24.fld6.0 = Field::<(i32, *mut i32, u32, i32)>(Variant(_10.fld3, 2), 0).3 as usize;
_3.1 = _2.1 + _2.1;
_27 = (_2.2, Field::<(f64, u8, i8, char)>(Variant(_10.fld3, 2), 7).1, 1_i8, '\u{4965b}');
(*RET) = (_7, _1.1);
match _27.2 {
0 => bb7,
2 => bb3,
3 => bb4,
4 => bb12,
1 => bb14,
_ => bb13
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_28.fld4.1 = _11.2 as u8;
_3.0 = !(*_18);
place!(Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_10.fld3, 2), 4)).1 = (_3.2, _27.1, _27.2, _27.3);
_1.0 = (*RET).0;
place!(Field::<u32>(Variant(_10.fld3, 2), 3)) = _27.3 as u32;
(*RET).1 = -_15.1;
place!(Field::<(f64, u8, i8, char)>(Variant(_10.fld3, 2), 7)).3 = Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_10.fld3, 2), 4).1.3;
_28.fld4.2 = Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_10.fld3, 2), 4).1.2;
_1 = (*RET);
_23 = _19.0 ^ _19.0;
_28.fld5 = _2.2 + _2.2;
place!(Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_10.fld3, 2), 4)).1.1 = !_27.1;
_10.fld1 = [18631_i16];
(*RET).0 = _1.0 * _15.0;
_9 = core::ptr::addr_of!(_2.1);
(*RET).1 = (-4193536792986515297_i64) as isize;
place!(Field::<(i32, *mut i32, u32, i32)>(Variant(_10.fld3, 2), 0)).2 = Field::<u32>(Variant(_10.fld3, 2), 3) << _5;
_12.fld2 = !_10.fld2;
_12.fld2 = (*_9) as i128;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(1_usize, 5_usize, Move(_5), 13_usize, Move(_13), 8_usize, Move(_8), 33_usize, _33), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: (u32, u16, f64),mut _2: f64,mut _3: isize,mut _4: (u32, u16, f64),mut _5: u32,mut _6: u16,mut _7: f32,mut _8: u32) -> *mut (f32, isize) {
mir! {
type RET = *mut (f32, isize);
let _9: Adt51;
let _10: u64;
let _11: i32;
let _12: (usize,);
let _13: f32;
let _14: (f32, isize);
let _15: *mut u32;
let _16: Adt47;
let _17: bool;
let _18: f32;
let _19: isize;
let _20: i16;
let _21: u64;
let _22: f32;
let _23: [i8; 3];
let _24: *const u16;
let _25: isize;
let _26: ((f32, isize), i128, usize);
let _27: Adt53;
let _28: i8;
let _29: i16;
let _30: f32;
let _31: (f64, f32);
let _32: isize;
let _33: f64;
let _34: char;
let _35: isize;
let _36: (bool,);
let _37: ();
let _38: ();
{
_4.1 = !_6;
_4.0 = '\u{f5b0d}' as u32;
_6 = _4.1 & _1.1;
_4.0 = 55_i8 as u32;
_8 = _1.0;
_1.2 = 128720383833747253867254564216924797728_u128 as f64;
_7 = (-64291855372104309695205459618263911974_i128) as f32;
_3 = (-9223372036854775808_isize) * 9223372036854775807_isize;
_7 = 516_i16 as f32;
_7 = 285011347264085714413157801950026896252_u128 as f32;
Call(_2 = fn3(_1.2, _3, _3, _4.2, _8, _6, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4.1 = _6;
_3 = !121_isize;
_4.2 = -_2;
_4.2 = 29122677615145682936919504223517956455_i128 as f64;
_3 = !9223372036854775807_isize;
Goto(bb2)
}
bb2 = {
_6 = _4.1;
_6 = _4.1 | _4.1;
_10 = 17884380184573636892_u64;
_14.0 = 147316801838384875844482015065984457635_u128 as f32;
_13 = _7 * _7;
RET = core::ptr::addr_of_mut!(_14);
_4.0 = 163048909135969422992327186632537520889_u128 as u32;
_1.2 = -_2;
_14.0 = 69093623142539101087401493755449703123_i128 as f32;
_4.1 = _6 >> _6;
_4 = (_8, _6, _1.2);
_14.0 = -_13;
_13 = -_14.0;
_11 = !1822287079_i32;
_4.0 = !_1.0;
_8 = !_4.0;
_4.2 = -_1.2;
Call(_12 = fn5(RET, _2, _3, _2, _1.2, (*RET).0, _1, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
(*RET) = (_13, _3);
(*RET).0 = -_13;
(*RET).0 = _7;
_1.0 = _4.0;
_15 = core::ptr::addr_of_mut!(_4.0);
(*RET).1 = _3 - _3;
_1.2 = -_4.2;
(*RET) = (_7, _3);
_8 = _5 >> _1.1;
RET = core::ptr::addr_of_mut!((*RET));
_1.0 = _8 & _4.0;
_5 = _1.0;
(*RET).1 = -_3;
_4 = _1;
Goto(bb4)
}
bb4 = {
(*_15) = !_5;
_5 = _3 as u32;
_1.0 = !_8;
_13 = 4015525995066659760_i64 as f32;
_2 = 239_u8 as f64;
_4 = (_8, _6, _1.2);
_10 = 15857415987408960856_u64;
_17 = false;
_6 = _14.1 as u16;
(*_15) = _1.2 as u32;
_3 = -_14.1;
_1 = ((*_15), _4.1, _4.2);
_19 = !(*RET).1;
_17 = true;
(*RET).1 = _3 >> (*_15);
_4 = (_1.0, _1.1, _1.2);
_1.2 = _4.2;
(*RET) = (_7, _19);
_3 = 333514427893067440478230548247581053626_u128 as isize;
_14.0 = -_13;
(*RET).0 = 60_i8 as f32;
(*RET) = (_13, _3);
_4.1 = _1.1;
_12 = (10569691989146258008_usize,);
_4 = (_1.0, _6, _1.2);
_15 = core::ptr::addr_of_mut!(_1.0);
_15 = core::ptr::addr_of_mut!(_1.0);
(*RET).1 = _19;
(*_15) = _4.0;
Call((*_15) = core::intrinsics::bswap(_4.0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
(*RET) = (_7, _3);
(*_15) = _8 << _19;
(*RET) = (_13, _19);
_4.0 = !(*_15);
(*RET).0 = _7;
(*RET) = (_13, _3);
(*RET) = (_7, _3);
_4.0 = _8 * _1.0;
_17 = _5 > _1.0;
(*RET).0 = _19 as f32;
_13 = -(*RET).0;
(*RET) = (_13, _3);
_4.0 = _17 as u32;
_14 = (_13, _19);
_14 = (_13, _3);
(*_15) = !_4.0;
_22 = (*RET).0 + _13;
_20 = -27797_i16;
_14.0 = -_22;
_20 = 31522_i16 + 19831_i16;
_1.1 = !_4.1;
_4 = ((*_15), _6, _1.2);
(*_15) = _4.0;
(*RET).1 = -_3;
(*RET).1 = -_19;
_23 = [(-110_i8),106_i8,69_i8];
match _12.0 {
0 => bb6,
10569691989146258008 => bb8,
_ => bb7
}
}
bb6 = {
_6 = _4.1;
_6 = _4.1 | _4.1;
_10 = 17884380184573636892_u64;
_14.0 = 147316801838384875844482015065984457635_u128 as f32;
_13 = _7 * _7;
RET = core::ptr::addr_of_mut!(_14);
_4.0 = 163048909135969422992327186632537520889_u128 as u32;
_1.2 = -_2;
_14.0 = 69093623142539101087401493755449703123_i128 as f32;
_4.1 = _6 >> _6;
_4 = (_8, _6, _1.2);
_14.0 = -_13;
_13 = -_14.0;
_11 = !1822287079_i32;
_4.0 = !_1.0;
_8 = !_4.0;
_4.2 = -_1.2;
Call(_12 = fn5(RET, _2, _3, _2, _1.2, (*RET).0, _1, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_4.1 = _6;
_3 = !121_isize;
_4.2 = -_2;
_4.2 = 29122677615145682936919504223517956455_i128 as f64;
_3 = !9223372036854775807_isize;
Goto(bb2)
}
bb8 = {
(*RET) = (_13, _3);
_8 = _4.0;
_25 = _1.1 as isize;
_18 = (*RET).0;
_26.2 = (*RET).0 as usize;
_1 = (_8, _4.1, _4.2);
_4.2 = -_1.2;
_24 = core::ptr::addr_of!(_1.1);
Goto(bb9)
}
bb9 = {
_27.fld1 = _7;
(*RET).0 = _7;
_1.1 = _4.2 as u16;
_12 = (_26.2,);
(*RET).1 = _25;
match _10 {
0 => bb10,
15857415987408960856 => bb12,
_ => bb11
}
}
bb10 = {
(*RET) = (_7, _3);
(*_15) = _8 << _19;
(*RET) = (_13, _19);
_4.0 = !(*_15);
(*RET).0 = _7;
(*RET) = (_13, _3);
(*RET) = (_7, _3);
_4.0 = _8 * _1.0;
_17 = _5 > _1.0;
(*RET).0 = _19 as f32;
_13 = -(*RET).0;
(*RET) = (_13, _3);
_4.0 = _17 as u32;
_14 = (_13, _19);
_14 = (_13, _3);
(*_15) = !_4.0;
_22 = (*RET).0 + _13;
_20 = -27797_i16;
_14.0 = -_22;
_20 = 31522_i16 + 19831_i16;
_1.1 = !_4.1;
_4 = ((*_15), _6, _1.2);
(*_15) = _4.0;
(*RET).1 = -_3;
(*RET).1 = -_19;
_23 = [(-110_i8),106_i8,69_i8];
match _12.0 {
0 => bb6,
10569691989146258008 => bb8,
_ => bb7
}
}
bb11 = {
_4.1 = _6;
_3 = !121_isize;
_4.2 = -_2;
_4.2 = 29122677615145682936919504223517956455_i128 as f64;
_3 = !9223372036854775807_isize;
Goto(bb2)
}
bb12 = {
_5 = _4.0 + _4.0;
_22 = _14.0;
(*RET) = (_18, _25);
_28 = _10 as i8;
_27.fld0.2 = _26.2 - _26.2;
_26 = ((*RET), 94770797715451499787817358494442131411_i128, _27.fld0.2);
Call(_26 = fn14(_15, _24, _1, _1, RET, _4.2, _18, _24, _24, _1.1, _1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
(*RET).0 = _26.0.0 * _18;
_24 = core::ptr::addr_of!(_4.1);
_4.2 = _27.fld0.2 as f64;
_29 = _20 | _20;
_26.0 = (*RET);
RET = core::ptr::addr_of_mut!(_26.0);
_16 = Adt47::Variant0 { fld0: _24,fld1: _14 };
_23 = [_28,_28,_28];
(*_24) = _6 * _1.1;
SetDiscriminant(_16, 0);
_31.0 = _1.2 - _1.2;
_12 = (_26.2,);
_23 = [_28,_28,_28];
(*RET) = _14;
_31.1 = _26.1 as f32;
_24 = core::ptr::addr_of!((*_24));
_2 = _28 as f64;
_26.0.0 = _28 as f32;
_1.2 = -_31.0;
place!(Field::<(f32, isize)>(Variant(_16, 0), 1)) = (_31.1, _3);
_13 = _31.1 * _27.fld1;
_6 = _28 as u16;
_2 = (*RET).0 as f64;
match _26.2 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb14,
4 => bb15,
5 => bb16,
5072036645704638256 => bb18,
_ => bb17
}
}
bb14 = {
_5 = _4.0 + _4.0;
_22 = _14.0;
(*RET) = (_18, _25);
_28 = _10 as i8;
_27.fld0.2 = _26.2 - _26.2;
_26 = ((*RET), 94770797715451499787817358494442131411_i128, _27.fld0.2);
Call(_26 = fn14(_15, _24, _1, _1, RET, _4.2, _18, _24, _24, _1.1, _1), ReturnTo(bb13), UnwindUnreachable())
}
bb15 = {
_6 = _4.1;
_6 = _4.1 | _4.1;
_10 = 17884380184573636892_u64;
_14.0 = 147316801838384875844482015065984457635_u128 as f32;
_13 = _7 * _7;
RET = core::ptr::addr_of_mut!(_14);
_4.0 = 163048909135969422992327186632537520889_u128 as u32;
_1.2 = -_2;
_14.0 = 69093623142539101087401493755449703123_i128 as f32;
_4.1 = _6 >> _6;
_4 = (_8, _6, _1.2);
_14.0 = -_13;
_13 = -_14.0;
_11 = !1822287079_i32;
_4.0 = !_1.0;
_8 = !_4.0;
_4.2 = -_1.2;
Call(_12 = fn5(RET, _2, _3, _2, _1.2, (*RET).0, _1, _4), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
(*RET) = (_13, _3);
(*RET).0 = -_13;
(*RET).0 = _7;
_1.0 = _4.0;
_15 = core::ptr::addr_of_mut!(_4.0);
(*RET).1 = _3 - _3;
_1.2 = -_4.2;
(*RET) = (_7, _3);
_8 = _5 >> _1.1;
RET = core::ptr::addr_of_mut!((*RET));
_1.0 = _8 & _4.0;
_5 = _1.0;
(*RET).1 = -_3;
_4 = _1;
Goto(bb4)
}
bb17 = {
_27.fld1 = _7;
(*RET).0 = _7;
_1.1 = _4.2 as u16;
_12 = (_26.2,);
(*RET).1 = _25;
match _10 {
0 => bb10,
15857415987408960856 => bb12,
_ => bb11
}
}
bb18 = {
_27 = Adt53 { fld0: _26,fld1: Field::<(f32, isize)>(Variant(_16, 0), 1).0 };
_24 = core::ptr::addr_of!((*_24));
_4.0 = (*_15) >> (*_15);
_33 = _28 as f64;
_34 = '\u{d0634}';
_16 = Adt47::Variant0 { fld0: _24,fld1: (*RET) };
_21 = !_10;
_28 = 34_i8;
_31.1 = _3 as f32;
(*RET).0 = _27.fld0.0.0;
_3 = !_25;
_26.1 = _27.fld0.1;
_32 = _26.0.1;
_1.2 = -_31.0;
place!(Field::<(f32, isize)>(Variant(_16, 0), 1)).0 = (*_24) as f32;
_27.fld0.0 = (*RET);
Goto(bb19)
}
bb19 = {
Call(_37 = dump_var(2_usize, 25_usize, Move(_25), 23_usize, Move(_23), 11_usize, Move(_11), 12_usize, Move(_12)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_37 = dump_var(2_usize, 20_usize, Move(_20), 19_usize, Move(_19), 34_usize, Move(_34), 6_usize, Move(_6)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: f64,mut _2: isize,mut _3: isize,mut _4: f64,mut _5: u32,mut _6: u16,mut _7: isize) -> f64 {
mir! {
type RET = f64;
let _8: isize;
let _9: [i16; 1];
let _10: f64;
let _11: (bool,);
let _12: Adt53;
let _13: *const *mut i32;
let _14: Adt56;
let _15: Adt50;
let _16: u32;
let _17: [i8; 3];
let _18: (f32, (f64, u8, i8, char), u32, u64);
let _19: i16;
let _20: u16;
let _21: usize;
let _22: Adt52;
let _23: ();
let _24: ();
{
_1 = -_4;
RET = _1 + _1;
_8 = _2 + _7;
_8 = _2 * _2;
_7 = _3;
_7 = _8 ^ _8;
_9 = [(-11372_i16)];
_5 = 200477947_u32;
_2 = _7;
_1 = 19117_i16 as f64;
_10 = RET;
_9 = [4634_i16];
RET = -_4;
RET = -_4;
_6 = 6994_u16;
_4 = _10;
_2 = _7 * _8;
_4 = _10 * RET;
RET = _10;
_5 = _7 as u32;
_1 = RET;
_9 = [31932_i16];
_7 = _2 & _3;
_2 = !_8;
_4 = 254277813445167878154294902691382020364_u128 as f64;
_10 = -_1;
_4 = _10 * RET;
Call(_12 = fn4(_1, _7, _7, _2, _1, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _12.fld0.0.0 as f64;
_9 = [4202_i16];
_12.fld0.0.1 = _5 as isize;
_11.0 = !true;
_5 = 1229961994_u32 * 3687879412_u32;
_12.fld0.0.1 = _7;
RET = -_10;
_12.fld0.0.0 = -_12.fld1;
_1 = _4 + _4;
_10 = _1 + _1;
_12.fld0.2 = '\u{e59d1}' as usize;
_11 = (true,);
_6 = 2151883597388057332_u64 as u16;
_12.fld0.0.0 = _12.fld1 + _12.fld1;
_9 = [12360_i16];
_2 = _12.fld0.0.1;
_12.fld0.1 = _11.0 as i128;
_4 = _10 + _10;
_12.fld0.2 = !10139779183329658592_usize;
_14.fld1 = _12.fld0.0;
_12.fld0.1 = !58525768625196995592162152484323880444_i128;
_12.fld1 = -_14.fld1.0;
_9 = [(-13255_i16)];
_12.fld0.2 = 6_usize << _2;
Goto(bb2)
}
bb2 = {
_8 = !_7;
_12.fld0.0.0 = _4 as f32;
_3 = _2;
_1 = _4 + _10;
_13 = core::ptr::addr_of!(_14.fld2);
_14.fld1 = _12.fld0.0;
_14.fld6.0 = _12.fld0.2 + _12.fld0.2;
_14.fld0 = !133_u8;
_18.1.2 = -19_i8;
_8 = -_7;
_6 = 16670208326675920866_u64 as u16;
_18.1.3 = '\u{21de3}';
_19 = (-4186_i16) << _3;
_14.fld6.0 = _12.fld0.2;
_18.1.2 = (-124_i8);
_14.fld4 = [18048028013506794780_u64];
_18.0 = _14.fld1.0;
RET = 2772776342126092736_i64 as f64;
Goto(bb3)
}
bb3 = {
_14.fld6 = (_12.fld0.2,);
_12.fld0.0 = (_14.fld1.0, _3);
_12.fld0.0.0 = _14.fld1.0;
_11 = (true,);
_14.fld5 = (_12.fld0.1,);
_16 = _18.1.3 as u32;
match _18.1.2 {
0 => bb2,
340282366920938463463374607431768211332 => bb5,
_ => bb4
}
}
bb4 = {
_8 = !_7;
_12.fld0.0.0 = _4 as f32;
_3 = _2;
_1 = _4 + _10;
_13 = core::ptr::addr_of!(_14.fld2);
_14.fld1 = _12.fld0.0;
_14.fld6.0 = _12.fld0.2 + _12.fld0.2;
_14.fld0 = !133_u8;
_18.1.2 = -19_i8;
_8 = -_7;
_6 = 16670208326675920866_u64 as u16;
_18.1.3 = '\u{21de3}';
_19 = (-4186_i16) << _3;
_14.fld6.0 = _12.fld0.2;
_18.1.2 = (-124_i8);
_14.fld4 = [18048028013506794780_u64];
_18.0 = _14.fld1.0;
RET = 2772776342126092736_i64 as f64;
Goto(bb3)
}
bb5 = {
_18.3 = 7747462379969223576_u64;
_12.fld0.1 = _11.0 as i128;
_18.2 = 238231881837480422756230432855058295101_u128 as u32;
_18.1.0 = -_1;
_12.fld0.1 = _14.fld5.0;
RET = _8 as f64;
_18.0 = -_12.fld0.0.0;
_16 = _18.1.2 as u32;
_17 = [_18.1.2,_18.1.2,_18.1.2];
_18.1 = (_1, _14.fld0, (-63_i8), '\u{102f20}');
_12.fld0.2 = _12.fld0.0.0 as usize;
_12.fld0.0.0 = _18.0 - _14.fld1.0;
_12.fld0.1 = !_14.fld5.0;
_11.0 = true;
_19 = !29207_i16;
_18.0 = _18.2 as f32;
_11.0 = false;
_14.fld4 = [_18.3];
_14.fld4 = [_18.3];
_12.fld0.2 = _5 as usize;
Goto(bb6)
}
bb6 = {
Call(_23 = dump_var(3_usize, 17_usize, Move(_17), 8_usize, Move(_8), 19_usize, Move(_19), 3_usize, Move(_3)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_23 = dump_var(3_usize, 6_usize, Move(_6), 24_usize, _24, 24_usize, _24, 24_usize, _24), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: f64,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: f64,mut _6: isize) -> Adt53 {
mir! {
type RET = Adt53;
let _7: *mut i32;
let _8: Adt47;
let _9: isize;
let _10: (i8, u64);
let _11: [i8; 7];
let _12: i32;
let _13: [i8; 3];
let _14: ();
let _15: ();
{
RET.fld1 = 190847126955471199894999809042438336613_u128 as f32;
RET.fld0.1 = (-94254422428974119867651017421049492190_i128);
_1 = _5 - _5;
_5 = -_1;
_2 = _6 & _6;
RET.fld0.0 = (RET.fld1, _3);
_10.1 = 2672193967343323130_u64 & 11180980458537536712_u64;
_10 = ((-112_i8), 14295937935994818554_u64);
_4 = RET.fld0.0.1 | _6;
RET.fld0.2 = 1_usize + 2_usize;
_10.0 = _10.1 as i8;
RET.fld0.2 = 3_usize ^ 2407460431488660743_usize;
RET.fld0.0 = (RET.fld1, _2);
RET.fld0.0.1 = -_2;
_10.0 = false as i8;
_11 = [_10.0,_10.0,_10.0,_10.0,_10.0,_10.0,_10.0];
_12 = (-2032693984_i32);
_13 = [_10.0,_10.0,_10.0];
_12 = -(-610776483_i32);
RET.fld0.2 = 12746229323872507836_usize - 2_usize;
_10.1 = !4565694930195526914_u64;
Goto(bb1)
}
bb1 = {
Call(_14 = dump_var(4_usize, 10_usize, Move(_10), 12_usize, Move(_12), 11_usize, Move(_11), 2_usize, Move(_2)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: *mut (f32, isize),mut _2: f64,mut _3: isize,mut _4: f64,mut _5: f64,mut _6: f32,mut _7: (u32, u16, f64),mut _8: (u32, u16, f64)) -> (usize,) {
mir! {
type RET = (usize,);
let _9: Adt48;
let _10: bool;
let _11: Adt42;
let _12: f32;
let _13: (i8, u64);
let _14: f64;
let _15: Adt58;
let _16: ();
let _17: ();
{
(*_1).1 = !_3;
_5 = -_2;
(*_1).1 = _3;
(*_1) = (_6, _3);
(*_1).1 = -_3;
_4 = _8.2;
_8 = _7;
_1 = core::ptr::addr_of_mut!((*_1));
RET = (3_usize,);
(*_1).0 = -_6;
Call(_3 = fn6(_8.2, _2, _8.2, _1, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_1) = (_6, _3);
_13.1 = 8188551694668982073_u64 | 34335575225255121_u64;
(*_1) = (_6, _3);
_8.1 = _8.2 as u16;
_13.0 = -92_i8;
_6 = (*_1).0 * (*_1).0;
Goto(bb2)
}
bb2 = {
_8 = (_7.0, _7.1, _5);
RET = (265812077277478379_usize,);
_7.1 = '\u{6a12}' as u16;
_12 = _6 * (*_1).0;
_15.fld6 = (-102281885904856642021932907019696797215_i128) as i64;
_3 = (*_1).1 >> (*_1).1;
_15.fld4 = (_2, 82_u8, _13.0, '\u{c255}');
_1 = core::ptr::addr_of_mut!((*_1));
match _15.fld4.1 {
0 => bb3,
1 => bb4,
2 => bb5,
82 => bb7,
_ => bb6
}
}
bb3 = {
(*_1) = (_6, _3);
_13.1 = 8188551694668982073_u64 | 34335575225255121_u64;
(*_1) = (_6, _3);
_8.1 = _8.2 as u16;
_13.0 = -92_i8;
_6 = (*_1).0 * (*_1).0;
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
RET.0 = _15.fld4.1 as usize;
Goto(bb8)
}
bb8 = {
Call(_16 = dump_var(5_usize, 3_usize, Move(_3), 17_usize, _17, 17_usize, _17, 17_usize, _17), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: f64,mut _2: f64,mut _3: f64,mut _4: *mut (f32, isize),mut _5: f64) -> isize {
mir! {
type RET = isize;
let _6: u32;
let _7: ((f32, isize), i128, usize);
let _8: Adt50;
let _9: isize;
let _10: f32;
let _11: u32;
let _12: Adt44;
let _13: f32;
let _14: usize;
let _15: ();
let _16: ();
{
_2 = -_3;
_2 = _1 * _3;
RET = (*_4).1;
(*_4).0 = 6402963226233635572_usize as f32;
_2 = _3 + _5;
_6 = 17620225238208006487_usize as u32;
RET = (*_4).1;
_7 = ((*_4), (-132132185743311955496237685577484354296_i128), 15306280397947178137_usize);
_7.1 = (-47332045184558242450279996367304057726_i128);
_7.1 = (-156200455736964501575074703652046994543_i128);
(*_4).0 = (-54_i8) as f32;
_7.1 = (-156378143433800562193792814587009540082_i128) * 85674187319599189221546215504588002927_i128;
_4 = core::ptr::addr_of_mut!((*_4));
_7.0.1 = 111_i8 as isize;
(*_4).1 = _7.0.1 << _7.2;
_6 = 4137351215_u32 ^ 408105401_u32;
_3 = _6 as f64;
_9 = (*_4).1 ^ _7.0.1;
RET = (*_4).1;
RET = (*_4).1 | (*_4).1;
Call(_8 = fn7((*_4).1, _2, (*_4).1, RET, _7.2, _4, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_4).1 = Field::<i64>(Variant(_8, 0), 4) as isize;
_4 = core::ptr::addr_of_mut!((*_4));
Call(_11 = core::intrinsics::transmute(Field::<i32>(Variant(_8, 0), 2)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
place!(Field::<i32>(Variant(_8, 0), 2)) = !(-892256059_i32);
_7.0.1 = _9 * RET;
(*_4).1 = '\u{a8a87}' as isize;
_9 = false as isize;
_7.2 = _7.0.1 as usize;
RET = -_7.0.1;
(*_4) = _7.0;
(*_4) = _7.0;
(*_4).0 = -_7.0.0;
(*_4) = (_7.0.0, RET);
_7.2 = 2010509460501302003_usize;
place!(Field::<[i8; 7]>(Variant(_8, 0), 0)) = [Field::<(i8, u64)>(Variant(_8, 0), 1).0,Field::<(i8, u64)>(Variant(_8, 0), 1).0,Field::<(i8, u64)>(Variant(_8, 0), 1).0,Field::<(i8, u64)>(Variant(_8, 0), 1).0,Field::<(i8, u64)>(Variant(_8, 0), 1).0,Field::<(i8, u64)>(Variant(_8, 0), 1).0,Field::<(i8, u64)>(Variant(_8, 0), 1).0];
SetDiscriminant(_8, 0);
RET = (*_4).1 + (*_4).1;
place!(Field::<(i8, u64)>(Variant(_8, 0), 1)).1 = 10298012449611310847_u64;
_2 = _5 - _5;
(*_4) = (_7.0.0, _9);
RET = _7.0.1 ^ _7.0.1;
place!(Field::<(i8, u64)>(Variant(_8, 0), 1)) = (103_i8, 17807349819875659925_u64);
place!(Field::<i32>(Variant(_8, 0), 2)) = _11 as i32;
_7.0 = ((*_4).0, RET);
Goto(bb3)
}
bb3 = {
Call(_15 = dump_var(6_usize, 9_usize, Move(_9), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: isize,mut _2: f64,mut _3: isize,mut _4: isize,mut _5: usize,mut _6: *mut (f32, isize),mut _7: isize) -> Adt50 {
mir! {
type RET = Adt50;
let _8: bool;
let _9: char;
let _10: f64;
let _11: isize;
let _12: *mut i32;
let _13: isize;
let _14: Adt50;
let _15: (f64, u8, i8, char);
let _16: isize;
let _17: (u32, u16, f64);
let _18: isize;
let _19: ();
let _20: ();
{
_7 = _5 as isize;
_5 = (*_6).0 as usize;
(*_6).0 = _5 as f32;
_8 = false ^ true;
(*_6).1 = _4;
_5 = !12659589843417805806_usize;
(*_6).0 = (*_6).1 as f32;
_5 = 43568_u16 as usize;
(*_6).0 = 5226877335124957358_u64 as f32;
Call(RET = fn8((*_6).1, _6, _2, (*_6), _3, _1, (*_6).1, (*_6), _2, _6, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
place!(Field::<(i128,)>(Variant(RET, 3), 4)).0 = 50052910130273629516825229637586231785_i128 ^ 133165055628591437786944727353840450262_i128;
_3 = (*_6).1;
place!(Field::<(f64, f32)>(Variant(RET, 3), 5)).1 = (*_6).0 * (*_6).0;
(*_6).1 = !_3;
place!(Field::<(f64, f32)>(Variant(RET, 3), 5)).0 = _2 * _2;
place!(Field::<[u64; 1]>(Variant(RET, 3), 0)) = [14492955342832635097_u64];
SetDiscriminant(RET, 0);
(*_6).1 = _4 & _3;
place!(Field::<(i8, u64)>(Variant(RET, 0), 1)) = ((-84_i8), 17119165628324398392_u64);
place!(Field::<i64>(Variant(RET, 0), 4)) = !7304699043064554020_i64;
_9 = '\u{62c83}';
_6 = core::ptr::addr_of_mut!((*_6));
_8 = !false;
match Field::<(i8, u64)>(Variant(RET, 0), 1).1 {
0 => bb2,
1 => bb3,
2 => bb4,
17119165628324398392 => bb6,
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
_5 = 9179420302583752277_usize & 5_usize;
place!(Field::<(i8, u64)>(Variant(RET, 0), 1)).0 = 90_i8;
place!(Field::<(i8, u64)>(Variant(RET, 0), 1)) = (34_i8, 13598390109227636428_u64);
(*_6).1 = _4;
_5 = (-134569757773408686626274428627405065947_i128) as usize;
place!(Field::<(i8, u64)>(Variant(RET, 0), 1)).1 = 17837378741793706976_u64;
match Field::<(i8, u64)>(Variant(RET, 0), 1).1 {
0 => bb5,
1 => bb4,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
6 => bb11,
17837378741793706976 => bb13,
_ => bb12
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
Return()
}
bb11 = {
place!(Field::<(i128,)>(Variant(RET, 3), 4)).0 = 50052910130273629516825229637586231785_i128 ^ 133165055628591437786944727353840450262_i128;
_3 = (*_6).1;
place!(Field::<(f64, f32)>(Variant(RET, 3), 5)).1 = (*_6).0 * (*_6).0;
(*_6).1 = !_3;
place!(Field::<(f64, f32)>(Variant(RET, 3), 5)).0 = _2 * _2;
place!(Field::<[u64; 1]>(Variant(RET, 3), 0)) = [14492955342832635097_u64];
SetDiscriminant(RET, 0);
(*_6).1 = _4 & _3;
place!(Field::<(i8, u64)>(Variant(RET, 0), 1)) = ((-84_i8), 17119165628324398392_u64);
place!(Field::<i64>(Variant(RET, 0), 4)) = !7304699043064554020_i64;
_9 = '\u{62c83}';
_6 = core::ptr::addr_of_mut!((*_6));
_8 = !false;
match Field::<(i8, u64)>(Variant(RET, 0), 1).1 {
0 => bb2,
1 => bb3,
2 => bb4,
17119165628324398392 => bb6,
_ => bb5
}
}
bb12 = {
Return()
}
bb13 = {
_2 = 216759559580318834969502651668031453087_u128 as f64;
place!(Field::<[i8; 7]>(Variant(RET, 0), 0)) = [Field::<(i8, u64)>(Variant(RET, 0), 1).0,Field::<(i8, u64)>(Variant(RET, 0), 1).0,Field::<(i8, u64)>(Variant(RET, 0), 1).0,Field::<(i8, u64)>(Variant(RET, 0), 1).0,Field::<(i8, u64)>(Variant(RET, 0), 1).0,Field::<(i8, u64)>(Variant(RET, 0), 1).0,Field::<(i8, u64)>(Variant(RET, 0), 1).0];
_2 = (-139280490995648595949290702991336250350_i128) as f64;
_3 = (*_6).1 ^ _4;
(*_6).0 = _2 as f32;
place!(Field::<[i8; 7]>(Variant(RET, 0), 0)) = [Field::<(i8, u64)>(Variant(RET, 0), 1).0,Field::<(i8, u64)>(Variant(RET, 0), 1).0,Field::<(i8, u64)>(Variant(RET, 0), 1).0,Field::<(i8, u64)>(Variant(RET, 0), 1).0,Field::<(i8, u64)>(Variant(RET, 0), 1).0,Field::<(i8, u64)>(Variant(RET, 0), 1).0,Field::<(i8, u64)>(Variant(RET, 0), 1).0];
place!(Field::<[i8; 7]>(Variant(RET, 0), 0)) = [Field::<(i8, u64)>(Variant(RET, 0), 1).0,Field::<(i8, u64)>(Variant(RET, 0), 1).0,Field::<(i8, u64)>(Variant(RET, 0), 1).0,Field::<(i8, u64)>(Variant(RET, 0), 1).0,Field::<(i8, u64)>(Variant(RET, 0), 1).0,Field::<(i8, u64)>(Variant(RET, 0), 1).0,Field::<(i8, u64)>(Variant(RET, 0), 1).0];
place!(Field::<u16>(Variant(RET, 0), 3)) = _8 as u16;
_8 = !false;
_14 = Adt50::Variant0 { fld0: Field::<[i8; 7]>(Variant(RET, 0), 0),fld1: Field::<(i8, u64)>(Variant(RET, 0), 1),fld2: (-117227935_i32),fld3: Field::<u16>(Variant(RET, 0), 3),fld4: Field::<i64>(Variant(RET, 0), 4) };
_10 = _2;
match Field::<(i8, u64)>(Variant(RET, 0), 1).0 {
0 => bb9,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb10,
5 => bb11,
6 => bb7,
34 => bb14,
_ => bb12
}
}
bb14 = {
place!(Field::<i32>(Variant(RET, 0), 2)) = 1826004644_i32 << _4;
place!(Field::<(i8, u64)>(Variant(_14, 0), 1)).0 = Field::<(i8, u64)>(Variant(RET, 0), 1).0 ^ Field::<(i8, u64)>(Variant(RET, 0), 1).0;
place!(Field::<[i8; 7]>(Variant(RET, 0), 0)) = Field::<[i8; 7]>(Variant(_14, 0), 0);
place!(Field::<(i8, u64)>(Variant(RET, 0), 1)).1 = Field::<i64>(Variant(_14, 0), 4) as u64;
(*_6).1 = (*_6).0 as isize;
Goto(bb15)
}
bb15 = {
Call(_19 = dump_var(7_usize, 7_usize, Move(_7), 4_usize, Move(_4), 3_usize, Move(_3), 20_usize, _20), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: isize,mut _2: *mut (f32, isize),mut _3: f64,mut _4: (f32, isize),mut _5: isize,mut _6: isize,mut _7: isize,mut _8: (f32, isize),mut _9: f64,mut _10: *mut (f32, isize),mut _11: isize) -> Adt50 {
mir! {
type RET = Adt50;
let _12: u64;
let _13: Adt51;
let _14: *mut (f32, isize);
let _15: Adt46;
let _16: *mut u32;
let _17: isize;
let _18: char;
let _19: [u64; 1];
let _20: f32;
let _21: (f64, f32);
let _22: &'static i128;
let _23: (bool,);
let _24: isize;
let _25: Adt51;
let _26: Adt54;
let _27: Adt56;
let _28: Adt58;
let _29: f32;
let _30: f32;
let _31: char;
let _32: char;
let _33: u128;
let _34: char;
let _35: u64;
let _36: (u32, u16, f64);
let _37: u128;
let _38: f32;
let _39: f32;
let _40: char;
let _41: (bool,);
let _42: Adt58;
let _43: ([i8; 7], [i8; 3], (f64, u8, i8, char), &'static i128);
let _44: [i16; 1];
let _45: i64;
let _46: i64;
let _47: isize;
let _48: isize;
let _49: bool;
let _50: [u128; 6];
let _51: f32;
let _52: isize;
let _53: [i64; 8];
let _54: Adt43;
let _55: *const *mut i32;
let _56: ();
let _57: ();
{
_8.0 = -(*_10).0;
(*_2) = (_4.0, _5);
_11 = (*_2).1 << _1;
(*_10).0 = 566259575319571133_i64 as f32;
(*_10) = (_8.0, _1);
(*_2).1 = '\u{2066d}' as isize;
Goto(bb1)
}
bb1 = {
(*_10).0 = -_4.0;
_4.1 = 249_u8 as isize;
_6 = 1976374238_u32 as isize;
_4.1 = _1;
(*_10).1 = _8.1;
_8.0 = (*_10).0;
_8.1 = -_1;
_5 = -_11;
_12 = 110906526633907425742439724926544284958_u128 as u64;
_7 = (-8011869476590367737_i64) as isize;
(*_10).0 = _8.0;
_10 = core::ptr::addr_of_mut!(_4);
_4.0 = 9593_u16 as f32;
_4.1 = _3 as isize;
_10 = core::ptr::addr_of_mut!((*_10));
(*_10).1 = (*_2).1 << _11;
_11 = _4.1 & _4.1;
_6 = -_11;
_5 = -_6;
(*_2) = (*_10);
(*_2).1 = -_11;
_6 = _5;
_12 = !88868170144253216_u64;
_8.1 = (-38645029321940922901913714129985285454_i128) as isize;
Goto(bb2)
}
bb2 = {
(*_10).0 = (*_2).0;
_5 = !_8.1;
(*_10).1 = (-868259496_i32) as isize;
(*_10).0 = -(*_2).0;
(*_2) = (*_10);
(*_10).1 = _6;
_4.0 = -(*_2).0;
_7 = _11;
_1 = _7;
(*_2) = ((*_10).0, _11);
(*_2).0 = -_8.0;
_8.1 = 42_i8 as isize;
_8 = ((*_2).0, (*_10).1);
_15 = Adt46::Variant3 { fld0: _11,fld1: 3394111651548017046_i64 };
_8.1 = _1;
(*_2).0 = 3413112251_u32 as f32;
(*_10).0 = 327746648657145591241914452651400795732_u128 as f32;
_14 = _10;
(*_10) = ((*_2).0, _6);
_17 = (*_14).1;
Call(place!(Field::<isize>(Variant(_15, 3), 0)) = fn9(_8.1, _10, _14, _2, _8.1, _10, (*_2), (*_2), _14, _11), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4.1 = 34724_u16 as isize;
_8.0 = -(*_10).0;
_18 = '\u{642a5}';
_15 = Adt46::Variant3 { fld0: _6,fld1: 7304263614703404251_i64 };
_8.0 = 46876_u16 as f32;
(*_14).0 = (*_2).0 - _8.0;
_10 = _2;
(*_2).0 = _8.0 - _4.0;
_8.0 = 11401789094980033020_usize as f32;
_19 = [_12];
(*_2) = _8;
_1 = -_17;
_7 = (*_10).1;
(*_14).0 = -(*_2).0;
_1 = 385788352588834004_i64 as isize;
place!(Field::<isize>(Variant(_15, 3), 0)) = (*_10).1;
_8.0 = (*_2).0;
_7 = (*_2).1;
_8 = (*_2);
_19 = [_12];
(*_2).0 = (*_14).0 - (*_14).0;
(*_10).1 = -_17;
Call(_21.0 = fn10((*_10), (*_2).1, _9, _11), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_21 = (_3, (*_10).0);
_8 = (_21.1, _17);
(*_2).0 = 60_i8 as f32;
_4.1 = _7;
_10 = core::ptr::addr_of_mut!((*_2));
_14 = core::ptr::addr_of_mut!((*_2));
_20 = -_4.0;
_11 = (*_10).1;
Goto(bb5)
}
bb5 = {
(*_10).1 = _4.1 ^ _6;
(*_2) = _8;
place!(Field::<i64>(Variant(_15, 3), 1)) = 1394842425022967029_i64;
_6 = 115_u8 as isize;
_15 = Adt46::Variant3 { fld0: _11,fld1: 6795551792758565324_i64 };
_4.0 = (*_14).0 + _20;
_23.0 = !false;
_24 = (*_14).1;
(*_2).0 = 12052445272932995619_usize as f32;
_4.1 = _24 ^ (*_2).1;
_23.0 = true;
_8 = ((*_10).0, _17);
(*_2).0 = _21.0 as f32;
(*_2).0 = -_20;
_15 = Adt46::Variant3 { fld0: (*_14).1,fld1: 117345962304164462_i64 };
_2 = core::ptr::addr_of_mut!(_4);
(*_10) = (_4.0, (*_2).1);
place!(Field::<i64>(Variant(_15, 3), 1)) = (-8264340909453081928_i64);
SetDiscriminant(_15, 3);
_27.fld4 = [_12];
_27.fld1.0 = -_20;
_8 = ((*_2).0, (*_2).1);
Call(_4.0 = core::intrinsics::transmute(_18), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_23.0 = false | false;
_6 = _24 << _11;
_22 = &_27.fld5.0;
_28.fld3 = (_20, (*_14).1);
_27.fld0 = !62_u8;
_28.fld4.2 = !61_i8;
(*_14).1 = (*_2).1;
_24 = (*_2).1;
_27.fld5 = (84046739516472194879275894238640448200_i128,);
_28.fld4 = (_21.0, _27.fld0, 112_i8, _18);
_28.fld6 = -(-5998519819449941332_i64);
_28.fld1 = !11474669114531911176137484292707878568_u128;
_28.fld3.1 = (*_14).1;
_27.fld0 = !_28.fld4.1;
(*_14) = _28.fld3;
_27.fld6 = (12531293299197103410_usize,);
_28.fld0 = _23.0;
_31 = _28.fld4.3;
match _28.fld4.2 {
0 => bb1,
1 => bb2,
112 => bb7,
_ => bb5
}
}
bb7 = {
_20 = _21.1 * (*_10).0;
_27.fld4 = [_12];
_27.fld5 = ((-24279288145120438086192294949675089202_i128),);
(*_10) = (_21.1, _4.1);
_8.1 = _17 >> (*_14).1;
(*_10).1 = _8.1;
_1 = _27.fld0 as isize;
_19 = [_12];
(*_2).1 = _8.1;
_31 = _28.fld4.3;
_27.fld1.1 = (*_14).0 as isize;
_32 = _28.fld4.3;
_28.fld4.3 = _32;
_27.fld5.0 = !(-18951637046818437874154725404033342654_i128);
_7 = _23.0 as isize;
_15 = Adt46::Variant3 { fld0: (*_10).1,fld1: _28.fld6 };
_34 = _18;
(*_2) = (*_14);
_32 = _28.fld4.3;
SetDiscriminant(_15, 2);
(*_2) = ((*_14).0, _8.1);
_28.fld3.0 = -(*_2).0;
_28.fld4.3 = _34;
place!(Field::<(usize,)>(Variant(_15, 2), 1)) = (_27.fld6.0,);
_28.fld6 = (-4002943811252319937_i64);
_17 = (*_2).1;
Call(_28.fld1 = fn11(_6, (*_10).1, _2, (*_2), _2, _8, (*_10).1, _10, _28.fld4.2, _10, _2, _24, (*_2).1, _2, (*_14).1, (*_2)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
(*_14).1 = -(*_2).1;
place!(Field::<u32>(Variant(_15, 2), 2)) = 628054795_u32;
_1 = (*_14).1 * (*_2).1;
place!(Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4)).0 = _28.fld4.2 as f32;
place!(Field::<(i128,)>(Variant(_15, 2), 5)).0 = _27.fld5.0;
place!(Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4)).1.3 = _28.fld4.3;
(*_2) = (*_10);
place!(Field::<(u32, u16, f64)>(Variant(_15, 2), 3)) = (Field::<u32>(Variant(_15, 2), 2), 39502_u16, _9);
place!(Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4)).1.0 = (-11434_i16) as f64;
(*_2).1 = _28.fld6 as isize;
(*_10).0 = _21.1 + Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4).0;
(*_14).0 = Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4).0;
(*_10) = (Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4).0, _24);
_6 = !(*_14).1;
_36.0 = !Field::<u32>(Variant(_15, 2), 2);
place!(Field::<(i128,)>(Variant(_15, 2), 5)) = (_27.fld5.0,);
_28.fld3.0 = _28.fld1 as f32;
_20 = -_28.fld3.0;
_33 = _28.fld1 + _28.fld1;
_20 = -(*_14).0;
Goto(bb9)
}
bb9 = {
place!(Field::<[i8; 7]>(Variant(_15, 2), 0)) = [_28.fld4.2,_28.fld4.2,_28.fld4.2,_28.fld4.2,_28.fld4.2,_28.fld4.2,_28.fld4.2];
_4.1 = _11;
_18 = _31;
match Field::<(u32, u16, f64)>(Variant(_15, 2), 3).1 {
39502 => bb11,
_ => bb10
}
}
bb10 = {
(*_10).0 = -_4.0;
_4.1 = 249_u8 as isize;
_6 = 1976374238_u32 as isize;
_4.1 = _1;
(*_10).1 = _8.1;
_8.0 = (*_10).0;
_8.1 = -_1;
_5 = -_11;
_12 = 110906526633907425742439724926544284958_u128 as u64;
_7 = (-8011869476590367737_i64) as isize;
(*_10).0 = _8.0;
_10 = core::ptr::addr_of_mut!(_4);
_4.0 = 9593_u16 as f32;
_4.1 = _3 as isize;
_10 = core::ptr::addr_of_mut!((*_10));
(*_10).1 = (*_2).1 << _11;
_11 = _4.1 & _4.1;
_6 = -_11;
_5 = -_6;
(*_2) = (*_10);
(*_2).1 = -_11;
_6 = _5;
_12 = !88868170144253216_u64;
_8.1 = (-38645029321940922901913714129985285454_i128) as isize;
Goto(bb2)
}
bb11 = {
_5 = (*_10).1;
_4.1 = (*_14).1 + _17;
_21.0 = _27.fld5.0 as f64;
_36 = Field::<(u32, u16, f64)>(Variant(_15, 2), 3);
_14 = core::ptr::addr_of_mut!((*_14));
_39 = _28.fld3.0;
_27.fld4 = [_12];
_23.0 = _28.fld0;
_3 = _39 as f64;
match Field::<(u32, u16, f64)>(Variant(_15, 2), 3).1 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb10,
4 => bb12,
5 => bb13,
6 => bb14,
39502 => bb16,
_ => bb15
}
}
bb12 = {
(*_10).0 = -_4.0;
_4.1 = 249_u8 as isize;
_6 = 1976374238_u32 as isize;
_4.1 = _1;
(*_10).1 = _8.1;
_8.0 = (*_10).0;
_8.1 = -_1;
_5 = -_11;
_12 = 110906526633907425742439724926544284958_u128 as u64;
_7 = (-8011869476590367737_i64) as isize;
(*_10).0 = _8.0;
_10 = core::ptr::addr_of_mut!(_4);
_4.0 = 9593_u16 as f32;
_4.1 = _3 as isize;
_10 = core::ptr::addr_of_mut!((*_10));
(*_10).1 = (*_2).1 << _11;
_11 = _4.1 & _4.1;
_6 = -_11;
_5 = -_6;
(*_2) = (*_10);
(*_2).1 = -_11;
_6 = _5;
_12 = !88868170144253216_u64;
_8.1 = (-38645029321940922901913714129985285454_i128) as isize;
Goto(bb2)
}
bb13 = {
_21 = (_3, (*_10).0);
_8 = (_21.1, _17);
(*_2).0 = 60_i8 as f32;
_4.1 = _7;
_10 = core::ptr::addr_of_mut!((*_2));
_14 = core::ptr::addr_of_mut!((*_2));
_20 = -_4.0;
_11 = (*_10).1;
Goto(bb5)
}
bb14 = {
(*_10).0 = -_4.0;
_4.1 = 249_u8 as isize;
_6 = 1976374238_u32 as isize;
_4.1 = _1;
(*_10).1 = _8.1;
_8.0 = (*_10).0;
_8.1 = -_1;
_5 = -_11;
_12 = 110906526633907425742439724926544284958_u128 as u64;
_7 = (-8011869476590367737_i64) as isize;
(*_10).0 = _8.0;
_10 = core::ptr::addr_of_mut!(_4);
_4.0 = 9593_u16 as f32;
_4.1 = _3 as isize;
_10 = core::ptr::addr_of_mut!((*_10));
(*_10).1 = (*_2).1 << _11;
_11 = _4.1 & _4.1;
_6 = -_11;
_5 = -_6;
(*_2) = (*_10);
(*_2).1 = -_11;
_6 = _5;
_12 = !88868170144253216_u64;
_8.1 = (-38645029321940922901913714129985285454_i128) as isize;
Goto(bb2)
}
bb15 = {
(*_10).0 = (*_2).0;
_5 = !_8.1;
(*_10).1 = (-868259496_i32) as isize;
(*_10).0 = -(*_2).0;
(*_2) = (*_10);
(*_10).1 = _6;
_4.0 = -(*_2).0;
_7 = _11;
_1 = _7;
(*_2) = ((*_10).0, _11);
(*_2).0 = -_8.0;
_8.1 = 42_i8 as isize;
_8 = ((*_2).0, (*_10).1);
_15 = Adt46::Variant3 { fld0: _11,fld1: 3394111651548017046_i64 };
_8.1 = _1;
(*_2).0 = 3413112251_u32 as f32;
(*_10).0 = 327746648657145591241914452651400795732_u128 as f32;
_14 = _10;
(*_10) = ((*_2).0, _6);
_17 = (*_14).1;
Call(place!(Field::<isize>(Variant(_15, 3), 0)) = fn9(_8.1, _10, _14, _2, _8.1, _10, (*_2), (*_2), _14, _11), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
place!(Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4)).1.2 = _28.fld4.2 >> _8.1;
(*_14).1 = _27.fld6.0 as isize;
_39 = (*_14).0;
_35 = (-17652_i16) as u64;
place!(Field::<(u32, u16, f64)>(Variant(_15, 2), 3)).0 = _36.0;
_41.0 = _28.fld0 & _23.0;
_28.fld4 = (_9, _27.fld0, Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4).1.2, _32);
place!(Field::<usize>(Variant(_15, 2), 7)) = !_27.fld6.0;
_41 = _23;
(*_2).1 = _24;
place!(Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4)).1.0 = _3;
_43.2 = _28.fld4;
_42.fld5 = _28.fld4.0;
_42.fld4.1 = _27.fld0;
_41 = (_23.0,);
_28.fld6 = (-4786310878629177108_i64);
_37 = !_33;
_23 = (_28.fld0,);
_27.fld1 = (_20, _28.fld3.1);
_42.fld1 = _27.fld5.0 as u128;
Goto(bb17)
}
bb17 = {
_27.fld6.0 = Field::<(usize,)>(Variant(_15, 2), 1).0 >> _5;
(*_2) = ((*_10).0, _8.1);
_41 = (_28.fld0,);
_8 = (_20, _4.1);
place!(Field::<(usize,)>(Variant(_15, 2), 1)).0 = _36.2 as usize;
_7 = _4.1 - (*_2).1;
place!(Field::<(u32, u16, f64)>(Variant(_15, 2), 3)).2 = _27.fld0 as f64;
_6 = _7 << _28.fld4.2;
place!(Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4)).1.1 = _43.2.1;
Call(_29 = fn12(_28.fld3, (*_2), _3, _28.fld3.1, Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4).1.2, _7, _27.fld1, _28.fld3, _7, _28.fld3.0, _8.1, (*_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
_42.fld6 = _28.fld6;
_27.fld5.0 = _27.fld6.0 as i128;
_43.3 = &_27.fld5.0;
place!(Field::<(u32, u16, f64)>(Variant(_15, 2), 3)).0 = _36.0;
_42.fld0 = _23.0;
_6 = !_1;
(*_10).1 = _7;
_21.0 = _9 * _28.fld4.0;
match _36.1 {
0 => bb1,
1 => bb2,
39502 => bb19,
_ => bb11
}
}
bb19 = {
_42.fld4.1 = _27.fld5.0 as u8;
_38 = -_29;
_42.fld4.3 = Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4).1.3;
_42.fld4.2 = Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4).1.2 * _28.fld4.2;
_9 = _27.fld6.0 as f64;
(*_2) = (*_14);
(*_14) = _8;
_4.1 = (-21760_i16) as isize;
(*_2).0 = Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4).1.2 as f32;
_43.2 = (_21.0, _42.fld4.1, _28.fld4.2, _28.fld4.3);
_42.fld6 = !_28.fld6;
_10 = core::ptr::addr_of_mut!((*_2));
_9 = _21.0 + Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4).1.0;
_22 = Move(_43.3);
_19 = [_35];
_43.3 = &(*_22);
_4.0 = -_38;
(*_10).0 = _38;
_20 = -(*_2).0;
_43.0 = Field::<[i8; 7]>(Variant(_15, 2), 0);
_28.fld2 = Field::<(usize,)>(Variant(_15, 2), 1).0 << _42.fld4.2;
place!(Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4)).1.1 = _33 as u8;
_45 = _28.fld6;
Call(_6 = fn13(_7, _43.2.2, (*_2), _14, (*_2), _7, (*_10), _7, (*_14).1, _14, (*_14)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
_42 = Adt58 { fld0: _41.0,fld1: _28.fld1,fld2: Field::<(usize,)>(Variant(_15, 2), 1).0,fld3: (*_2),fld4: _43.2,fld5: Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4).1.0,fld6: _28.fld6 };
_28 = Adt58 { fld0: _42.fld0,fld1: _42.fld1,fld2: _42.fld2,fld3: _42.fld3,fld4: _43.2,fld5: _42.fld4.0,fld6: _42.fld6 };
_27.fld5 = Field::<(i128,)>(Variant(_15, 2), 5);
_45 = -_28.fld6;
_4.0 = _29 + _28.fld3.0;
_5 = _17;
_41 = (_42.fld0,);
match _36.1 {
0 => bb3,
1 => bb21,
39502 => bb23,
_ => bb22
}
}
bb21 = {
_27.fld6.0 = Field::<(usize,)>(Variant(_15, 2), 1).0 >> _5;
(*_2) = ((*_10).0, _8.1);
_41 = (_28.fld0,);
_8 = (_20, _4.1);
place!(Field::<(usize,)>(Variant(_15, 2), 1)).0 = _36.2 as usize;
_7 = _4.1 - (*_2).1;
place!(Field::<(u32, u16, f64)>(Variant(_15, 2), 3)).2 = _27.fld0 as f64;
_6 = _7 << _28.fld4.2;
place!(Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4)).1.1 = _43.2.1;
Call(_29 = fn12(_28.fld3, (*_2), _3, _28.fld3.1, Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4).1.2, _7, _27.fld1, _28.fld3, _7, _28.fld3.0, _8.1, (*_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb22 = {
_20 = _21.1 * (*_10).0;
_27.fld4 = [_12];
_27.fld5 = ((-24279288145120438086192294949675089202_i128),);
(*_10) = (_21.1, _4.1);
_8.1 = _17 >> (*_14).1;
(*_10).1 = _8.1;
_1 = _27.fld0 as isize;
_19 = [_12];
(*_2).1 = _8.1;
_31 = _28.fld4.3;
_27.fld1.1 = (*_14).0 as isize;
_32 = _28.fld4.3;
_28.fld4.3 = _32;
_27.fld5.0 = !(-18951637046818437874154725404033342654_i128);
_7 = _23.0 as isize;
_15 = Adt46::Variant3 { fld0: (*_10).1,fld1: _28.fld6 };
_34 = _18;
(*_2) = (*_14);
_32 = _28.fld4.3;
SetDiscriminant(_15, 2);
(*_2) = ((*_14).0, _8.1);
_28.fld3.0 = -(*_2).0;
_28.fld4.3 = _34;
place!(Field::<(usize,)>(Variant(_15, 2), 1)) = (_27.fld6.0,);
_28.fld6 = (-4002943811252319937_i64);
_17 = (*_2).1;
Call(_28.fld1 = fn11(_6, (*_10).1, _2, (*_2), _2, _8, (*_10).1, _10, _28.fld4.2, _10, _2, _24, (*_2).1, _2, (*_14).1, (*_2)), ReturnTo(bb8), UnwindUnreachable())
}
bb23 = {
place!(Field::<[u64; 1]>(Variant(_15, 2), 6)) = _19;
place!(Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4)) = (_28.fld3.0, _43.2, Field::<u32>(Variant(_15, 2), 2), _35);
_42.fld6 = !_28.fld6;
_13 = Adt51::Variant1 { fld0: Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4) };
(*_2).0 = _42.fld3.0 * _38;
_44 = [26939_i16];
match _28.fld6 {
0 => bb19,
1 => bb15,
2 => bb24,
3 => bb25,
4 => bb26,
5 => bb27,
6 => bb28,
340282366920938463458588296553139034348 => bb30,
_ => bb29
}
}
bb24 = {
_20 = _21.1 * (*_10).0;
_27.fld4 = [_12];
_27.fld5 = ((-24279288145120438086192294949675089202_i128),);
(*_10) = (_21.1, _4.1);
_8.1 = _17 >> (*_14).1;
(*_10).1 = _8.1;
_1 = _27.fld0 as isize;
_19 = [_12];
(*_2).1 = _8.1;
_31 = _28.fld4.3;
_27.fld1.1 = (*_14).0 as isize;
_32 = _28.fld4.3;
_28.fld4.3 = _32;
_27.fld5.0 = !(-18951637046818437874154725404033342654_i128);
_7 = _23.0 as isize;
_15 = Adt46::Variant3 { fld0: (*_10).1,fld1: _28.fld6 };
_34 = _18;
(*_2) = (*_14);
_32 = _28.fld4.3;
SetDiscriminant(_15, 2);
(*_2) = ((*_14).0, _8.1);
_28.fld3.0 = -(*_2).0;
_28.fld4.3 = _34;
place!(Field::<(usize,)>(Variant(_15, 2), 1)) = (_27.fld6.0,);
_28.fld6 = (-4002943811252319937_i64);
_17 = (*_2).1;
Call(_28.fld1 = fn11(_6, (*_10).1, _2, (*_2), _2, _8, (*_10).1, _10, _28.fld4.2, _10, _2, _24, (*_2).1, _2, (*_14).1, (*_2)), ReturnTo(bb8), UnwindUnreachable())
}
bb25 = {
(*_10).0 = -_4.0;
_4.1 = 249_u8 as isize;
_6 = 1976374238_u32 as isize;
_4.1 = _1;
(*_10).1 = _8.1;
_8.0 = (*_10).0;
_8.1 = -_1;
_5 = -_11;
_12 = 110906526633907425742439724926544284958_u128 as u64;
_7 = (-8011869476590367737_i64) as isize;
(*_10).0 = _8.0;
_10 = core::ptr::addr_of_mut!(_4);
_4.0 = 9593_u16 as f32;
_4.1 = _3 as isize;
_10 = core::ptr::addr_of_mut!((*_10));
(*_10).1 = (*_2).1 << _11;
_11 = _4.1 & _4.1;
_6 = -_11;
_5 = -_6;
(*_2) = (*_10);
(*_2).1 = -_11;
_6 = _5;
_12 = !88868170144253216_u64;
_8.1 = (-38645029321940922901913714129985285454_i128) as isize;
Goto(bb2)
}
bb26 = {
_20 = _21.1 * (*_10).0;
_27.fld4 = [_12];
_27.fld5 = ((-24279288145120438086192294949675089202_i128),);
(*_10) = (_21.1, _4.1);
_8.1 = _17 >> (*_14).1;
(*_10).1 = _8.1;
_1 = _27.fld0 as isize;
_19 = [_12];
(*_2).1 = _8.1;
_31 = _28.fld4.3;
_27.fld1.1 = (*_14).0 as isize;
_32 = _28.fld4.3;
_28.fld4.3 = _32;
_27.fld5.0 = !(-18951637046818437874154725404033342654_i128);
_7 = _23.0 as isize;
_15 = Adt46::Variant3 { fld0: (*_10).1,fld1: _28.fld6 };
_34 = _18;
(*_2) = (*_14);
_32 = _28.fld4.3;
SetDiscriminant(_15, 2);
(*_2) = ((*_14).0, _8.1);
_28.fld3.0 = -(*_2).0;
_28.fld4.3 = _34;
place!(Field::<(usize,)>(Variant(_15, 2), 1)) = (_27.fld6.0,);
_28.fld6 = (-4002943811252319937_i64);
_17 = (*_2).1;
Call(_28.fld1 = fn11(_6, (*_10).1, _2, (*_2), _2, _8, (*_10).1, _10, _28.fld4.2, _10, _2, _24, (*_2).1, _2, (*_14).1, (*_2)), ReturnTo(bb8), UnwindUnreachable())
}
bb27 = {
(*_10).0 = -_4.0;
_4.1 = 249_u8 as isize;
_6 = 1976374238_u32 as isize;
_4.1 = _1;
(*_10).1 = _8.1;
_8.0 = (*_10).0;
_8.1 = -_1;
_5 = -_11;
_12 = 110906526633907425742439724926544284958_u128 as u64;
_7 = (-8011869476590367737_i64) as isize;
(*_10).0 = _8.0;
_10 = core::ptr::addr_of_mut!(_4);
_4.0 = 9593_u16 as f32;
_4.1 = _3 as isize;
_10 = core::ptr::addr_of_mut!((*_10));
(*_10).1 = (*_2).1 << _11;
_11 = _4.1 & _4.1;
_6 = -_11;
_5 = -_6;
(*_2) = (*_10);
(*_2).1 = -_11;
_6 = _5;
_12 = !88868170144253216_u64;
_8.1 = (-38645029321940922901913714129985285454_i128) as isize;
Goto(bb2)
}
bb28 = {
(*_10).0 = (*_2).0;
_5 = !_8.1;
(*_10).1 = (-868259496_i32) as isize;
(*_10).0 = -(*_2).0;
(*_2) = (*_10);
(*_10).1 = _6;
_4.0 = -(*_2).0;
_7 = _11;
_1 = _7;
(*_2) = ((*_10).0, _11);
(*_2).0 = -_8.0;
_8.1 = 42_i8 as isize;
_8 = ((*_2).0, (*_10).1);
_15 = Adt46::Variant3 { fld0: _11,fld1: 3394111651548017046_i64 };
_8.1 = _1;
(*_2).0 = 3413112251_u32 as f32;
(*_10).0 = 327746648657145591241914452651400795732_u128 as f32;
_14 = _10;
(*_10) = ((*_2).0, _6);
_17 = (*_14).1;
Call(place!(Field::<isize>(Variant(_15, 3), 0)) = fn9(_8.1, _10, _14, _2, _8.1, _10, (*_2), (*_2), _14, _11), ReturnTo(bb3), UnwindUnreachable())
}
bb29 = {
(*_10).1 = _4.1 ^ _6;
(*_2) = _8;
place!(Field::<i64>(Variant(_15, 3), 1)) = 1394842425022967029_i64;
_6 = 115_u8 as isize;
_15 = Adt46::Variant3 { fld0: _11,fld1: 6795551792758565324_i64 };
_4.0 = (*_14).0 + _20;
_23.0 = !false;
_24 = (*_14).1;
(*_2).0 = 12052445272932995619_usize as f32;
_4.1 = _24 ^ (*_2).1;
_23.0 = true;
_8 = ((*_10).0, _17);
(*_2).0 = _21.0 as f32;
(*_2).0 = -_20;
_15 = Adt46::Variant3 { fld0: (*_14).1,fld1: 117345962304164462_i64 };
_2 = core::ptr::addr_of_mut!(_4);
(*_10) = (_4.0, (*_2).1);
place!(Field::<i64>(Variant(_15, 3), 1)) = (-8264340909453081928_i64);
SetDiscriminant(_15, 3);
_27.fld4 = [_12];
_27.fld1.0 = -_20;
_8 = ((*_2).0, (*_2).1);
Call(_4.0 = core::intrinsics::transmute(_18), ReturnTo(bb6), UnwindUnreachable())
}
bb30 = {
_25 = _13;
_43.3 = &_27.fld5.0;
_9 = _36.1 as f64;
_32 = _28.fld4.3;
SetDiscriminant(_13, 1);
place!(Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_13, 1), 0)).3 = _37 as u64;
(*_14).1 = !_6;
_43.2.3 = _31;
_29 = -Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4).0;
_43.2 = (Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4).1.0, _28.fld4.1, Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_25, 1), 0).1.2, _42.fld4.3);
place!(Field::<[u64; 1]>(Variant(_15, 2), 6)) = [Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_13, 1), 0).3];
_23.0 = _42.fld0 & _28.fld0;
_13 = _25;
_27.fld6 = Field::<(usize,)>(Variant(_15, 2), 1);
_48 = _27.fld1.1;
_36.2 = _42.fld4.0 - _42.fld4.0;
_17 = _7 + _48;
_41.0 = _23.0;
(*_14).0 = _38;
(*_14).1 = !_17;
_21.1 = Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4).0;
_27.fld0 = _28.fld4.1 ^ _28.fld4.1;
_7 = _17 ^ _5;
_43.2.1 = Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_25, 1), 0).1.1;
_28.fld4 = (_42.fld5, _43.2.1, Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_13, 1), 0).1.2, _32);
place!(Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_13, 1), 0)).0 = -_42.fld3.0;
place!(Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_25, 1), 0)).1 = (Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_13, 1), 0).1.0, Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_13, 1), 0).1.1, _42.fld4.2, _28.fld4.3);
place!(Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4)) = Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_25, 1), 0);
Goto(bb31)
}
bb31 = {
_42.fld2 = (-1956808348_i32) as usize;
SetDiscriminant(_25, 2);
place!(Field::<(usize,)>(Variant(_15, 2), 1)).0 = _28.fld2;
place!(Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4)) = ((*_14).0, _42.fld4, Field::<(u32, u16, f64)>(Variant(_15, 2), 3).0, _12);
_43.1 = [Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_13, 1), 0).1.2,Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_13, 1), 0).1.2,_42.fld4.2];
_28.fld4 = (_42.fld4.0, Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4).1.1, Field::<(f32, (f64, u8, i8, char), u32, u64)>(Variant(_15, 2), 4).1.2, _31);
_27.fld6 = (_28.fld2,);
(*_10) = (_21.1, _1);
place!(Field::<(i32, *mut i32, u32, i32)>(Variant(_25, 2), 3)).0 = _28.fld4.2 as i32;
RET = Adt50::Variant3 { fld0: Field::<[u64; 1]>(Variant(_15, 2), 6),fld1: _18,fld2: _27.fld6.0,fld3: Field::<[i8; 7]>(Variant(_15, 2), 0),fld4: Field::<(i128,)>(Variant(_15, 2), 5),fld5: _21 };
place!(Field::<(f64, f32)>(Variant(RET, 3), 5)).1 = _27.fld6.0 as f32;
_28.fld4.2 = !_43.2.2;
_28.fld3.0 = _38;
Goto(bb32)
}
bb32 = {
Call(_56 = dump_var(8_usize, 5_usize, Move(_5), 1_usize, Move(_1), 7_usize, Move(_7), 6_usize, Move(_6)), ReturnTo(bb33), UnwindUnreachable())
}
bb33 = {
Call(_56 = dump_var(8_usize, 45_usize, Move(_45), 31_usize, Move(_31), 18_usize, Move(_18), 44_usize, Move(_44)), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
Call(_56 = dump_var(8_usize, 32_usize, Move(_32), 33_usize, Move(_33), 57_usize, _57, 57_usize, _57), ReturnTo(bb35), UnwindUnreachable())
}
bb35 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: isize,mut _2: *mut (f32, isize),mut _3: *mut (f32, isize),mut _4: *mut (f32, isize),mut _5: isize,mut _6: *mut (f32, isize),mut _7: (f32, isize),mut _8: (f32, isize),mut _9: *mut (f32, isize),mut _10: isize) -> isize {
mir! {
type RET = isize;
let _11: f64;
let _12: ();
let _13: ();
{
_9 = core::ptr::addr_of_mut!((*_3));
RET = (*_3).1 * (*_6).1;
_5 = (-63069165125544618045048699653299598253_i128) as isize;
_5 = RET + _8.1;
(*_9) = (_8.0, (*_4).1);
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(9_usize, 5_usize, Move(_5), 13_usize, _13, 13_usize, _13, 13_usize, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: (f32, isize),mut _2: isize,mut _3: f64,mut _4: isize) -> f64 {
mir! {
type RET = f64;
let _5: &'static i128;
let _6: i64;
let _7: f32;
let _8: [i16; 1];
let _9: [i16; 1];
let _10: f32;
let _11: (f32, (f64, u8, i8, char), u32, u64);
let _12: u8;
let _13: i64;
let _14: bool;
let _15: ((f32, isize), i128, usize);
let _16: Adt57;
let _17: u8;
let _18: Adt55;
let _19: Adt52;
let _20: u32;
let _21: Adt42;
let _22: bool;
let _23: u8;
let _24: isize;
let _25: u128;
let _26: [u64; 1];
let _27: u16;
let _28: ();
let _29: ();
{
_4 = _1.1;
_1.0 = _1.1 as f32;
RET = _3 * _3;
_3 = 0_usize as f64;
_2 = RET as isize;
RET = (-49681847677709707001654998043904671748_i128) as f64;
RET = _3;
_6 = (-7824203948117344052_i64) ^ 1973418061216391620_i64;
_1.0 = 60614836381795295970021187261813810878_i128 as f32;
_4 = _2;
_3 = RET;
RET = _3 + _3;
RET = _3 + _3;
_3 = RET;
_7 = _1.0;
_1.1 = _4;
RET = -_3;
_1.1 = _4 * _2;
Goto(bb1)
}
bb1 = {
_10 = -_7;
_11.1 = (_3, 84_u8, 90_i8, '\u{f1b7c}');
RET = _11.1.0 + _11.1.0;
_1.1 = !_2;
_9 = [(-24953_i16)];
_1.1 = _2 ^ _2;
RET = _3;
_11.0 = _11.1.0 as f32;
_9 = [(-4492_i16)];
_10 = 7724_u16 as f32;
_12 = !_11.1.1;
_8 = [10594_i16];
_11.1.0 = -_3;
_6 = 8322504879887285576_i64 ^ 6321775043567332281_i64;
_11.0 = _10;
_6 = 643005683993974888_i64;
_11.2 = 638144516_u32 & 2311337567_u32;
Goto(bb2)
}
bb2 = {
_4 = _1.1 | _2;
_1 = (_7, _2);
_8 = [(-26812_i16)];
_5 = &_15.1;
_11.1.3 = '\u{80b2d}';
_11.1 = (_3, _12, (-93_i8), '\u{f190f}');
_16.fld5.0 = !_11.2;
_16.fld6 = (RET, _10);
_15.2 = _1.0 as usize;
_11.1.0 = _10 as f64;
RET = _11.1.0;
_16.fld7 = (true,);
_16.fld7.0 = _4 < _4;
_15.0.0 = _16.fld6.1;
_1.1 = _4 | _2;
_18.fld2.0 = _15.2;
Goto(bb3)
}
bb3 = {
_17 = _12;
_18.fld3 = _11.1.2 + _11.1.2;
_18.fld6 = [_18.fld3,_18.fld3,_11.1.2,_11.1.2,_18.fld3,_18.fld3,_11.1.2];
_18.fld2 = (_15.2,);
_16.fld1.fld2 = _4 as i128;
_16.fld3 = [_16.fld7.0,_16.fld7.0,_16.fld7.0,_16.fld7.0,_16.fld7.0,_16.fld7.0,_16.fld7.0];
_16.fld3 = [_16.fld7.0,_16.fld7.0,_16.fld7.0,_16.fld7.0,_16.fld7.0,_16.fld7.0,_16.fld7.0];
_18.fld4 = [2505888762560332436_u64];
_16.fld1.fld0 = [13494499163225249976_u64];
_18.fld1 = _8;
_15.1 = -_16.fld1.fld2;
_11.1.1 = !_17;
_15.0.0 = _10 * _11.0;
_14 = _16.fld7.0;
match _11.1.2 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
340282366920938463463374607431768211363 => bb9,
_ => bb8
}
}
bb4 = {
_4 = _1.1 | _2;
_1 = (_7, _2);
_8 = [(-26812_i16)];
_5 = &_15.1;
_11.1.3 = '\u{80b2d}';
_11.1 = (_3, _12, (-93_i8), '\u{f190f}');
_16.fld5.0 = !_11.2;
_16.fld6 = (RET, _10);
_15.2 = _1.0 as usize;
_11.1.0 = _10 as f64;
RET = _11.1.0;
_16.fld7 = (true,);
_16.fld7.0 = _4 < _4;
_15.0.0 = _16.fld6.1;
_1.1 = _4 | _2;
_18.fld2.0 = _15.2;
Goto(bb3)
}
bb5 = {
_10 = -_7;
_11.1 = (_3, 84_u8, 90_i8, '\u{f1b7c}');
RET = _11.1.0 + _11.1.0;
_1.1 = !_2;
_9 = [(-24953_i16)];
_1.1 = _2 ^ _2;
RET = _3;
_11.0 = _11.1.0 as f32;
_9 = [(-4492_i16)];
_10 = 7724_u16 as f32;
_12 = !_11.1.1;
_8 = [10594_i16];
_11.1.0 = -_3;
_6 = 8322504879887285576_i64 ^ 6321775043567332281_i64;
_11.0 = _10;
_6 = 643005683993974888_i64;
_11.2 = 638144516_u32 & 2311337567_u32;
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
_10 = _15.0.0 + _11.0;
_17 = _12 ^ _12;
_15.2 = _18.fld2.0;
_13 = _16.fld5.0 as i64;
_5 = &_16.fld1.fld2;
_20 = 22463_u16 as u32;
_16.fld0 = !_17;
_16.fld6 = (RET, _7);
Goto(bb10)
}
bb10 = {
_16.fld5.1 = 43888_u16;
_20 = _11.2;
_1.1 = !_4;
_1.1 = _4;
_16.fld6 = (_11.1.0, _10);
_11.1.3 = '\u{e1b7e}';
_11.1.2 = _18.fld3;
_20 = !_11.2;
_11.3 = 8321421595421302886_u64;
_18.fld6 = [_18.fld3,_11.1.2,_11.1.2,_11.1.2,_11.1.2,_18.fld3,_18.fld3];
_17 = _11.1.1 - _11.1.1;
_25 = 208201985081794616699531172781642659850_u128 << _2;
_23 = _11.3 as u8;
_16.fld6 = (_11.1.0, _1.0);
_11.1 = (RET, _17, _18.fld3, '\u{cad0f}');
_11.1.1 = _16.fld0;
_16.fld5.2 = _16.fld1.fld2 as f64;
_23 = _12 ^ _12;
_13 = _11.1.2 as i64;
Call(_11.1.1 = core::intrinsics::transmute(_14), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_16.fld1.fld1 = _9;
_11.1 = (_16.fld5.2, _23, _18.fld3, '\u{2d300}');
_5 = &(*_5);
_15.0.1 = _1.1;
_9 = [(-19179_i16)];
_26 = [_11.3];
_16.fld5.0 = _11.2;
_15.1 = !(*_5);
_16.fld5.0 = _11.2;
_14 = _2 > _1.1;
_16.fld6.1 = _10;
_11.1.2 = _18.fld3 ^ _18.fld3;
_8 = [7101_i16];
_5 = &_16.fld1.fld2;
_2 = -_1.1;
match _16.fld5.1 {
43888 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_11.1 = (_16.fld5.2, _23, _18.fld3, '\u{f8df2}');
_16.fld0 = _1.0 as u8;
_8 = _9;
match _11.3 {
0 => bb1,
1 => bb4,
2 => bb9,
3 => bb14,
4 => bb15,
8321421595421302886 => bb17,
_ => bb16
}
}
bb14 = {
Return()
}
bb15 = {
_16.fld1.fld1 = _9;
_11.1 = (_16.fld5.2, _23, _18.fld3, '\u{2d300}');
_5 = &(*_5);
_15.0.1 = _1.1;
_9 = [(-19179_i16)];
_26 = [_11.3];
_16.fld5.0 = _11.2;
_15.1 = !(*_5);
_16.fld5.0 = _11.2;
_14 = _2 > _1.1;
_16.fld6.1 = _10;
_11.1.2 = _18.fld3 ^ _18.fld3;
_8 = [7101_i16];
_5 = &_16.fld1.fld2;
_2 = -_1.1;
match _16.fld5.1 {
43888 => bb13,
_ => bb12
}
}
bb16 = {
_4 = _1.1 | _2;
_1 = (_7, _2);
_8 = [(-26812_i16)];
_5 = &_15.1;
_11.1.3 = '\u{80b2d}';
_11.1 = (_3, _12, (-93_i8), '\u{f190f}');
_16.fld5.0 = !_11.2;
_16.fld6 = (RET, _10);
_15.2 = _1.0 as usize;
_11.1.0 = _10 as f64;
RET = _11.1.0;
_16.fld7 = (true,);
_16.fld7.0 = _4 < _4;
_15.0.0 = _16.fld6.1;
_1.1 = _4 | _2;
_18.fld2.0 = _15.2;
Goto(bb3)
}
bb17 = {
_15.2 = _18.fld2.0;
_25 = _18.fld3 as u128;
_7 = -_15.0.0;
_16.fld6.1 = -_10;
_15.0 = (_10, _4);
_15.0 = _1;
RET = _11.1.0 + _16.fld5.2;
_16.fld6.0 = -_16.fld5.2;
Goto(bb18)
}
bb18 = {
Call(_28 = dump_var(10_usize, 23_usize, Move(_23), 26_usize, Move(_26), 6_usize, Move(_6), 9_usize, Move(_9)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_28 = dump_var(10_usize, 2_usize, Move(_2), 14_usize, Move(_14), 29_usize, _29, 29_usize, _29), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: isize,mut _2: isize,mut _3: *mut (f32, isize),mut _4: (f32, isize),mut _5: *mut (f32, isize),mut _6: (f32, isize),mut _7: isize,mut _8: *mut (f32, isize),mut _9: i8,mut _10: *mut (f32, isize),mut _11: *mut (f32, isize),mut _12: isize,mut _13: isize,mut _14: *mut (f32, isize),mut _15: isize,mut _16: (f32, isize)) -> u128 {
mir! {
type RET = u128;
let _17: ();
let _18: ();
{
RET = 295042606621062738164256364912494011486_u128 >> (*_10).1;
(*_11).1 = (*_10).1;
_16.0 = _6.0 - (*_10).0;
(*_10).0 = 851625615_u32 as f32;
_15 = 16381_i16 as isize;
_1 = (*_11).1 + (*_11).1;
(*_10).1 = (*_11).1 << _2;
_7 = 44571300335312181728332483620538132010_i128 as isize;
_16.0 = -(*_14).0;
(*_14).1 = (*_8).1 << (*_10).1;
(*_11).0 = -(*_10).0;
(*_14).0 = _2 as f32;
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(11_usize, 7_usize, Move(_7), 13_usize, Move(_13), 1_usize, Move(_1), 18_usize, _18), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: (f32, isize),mut _2: (f32, isize),mut _3: f64,mut _4: isize,mut _5: i8,mut _6: isize,mut _7: (f32, isize),mut _8: (f32, isize),mut _9: isize,mut _10: f32,mut _11: isize,mut _12: (f32, isize)) -> f32 {
mir! {
type RET = f32;
let _13: isize;
let _14: (usize,);
let _15: ();
let _16: ();
{
_10 = _1.0 + _1.0;
_2.1 = _11;
_2 = _7;
_8.1 = (-867776442_i32) as isize;
_1.1 = _7.1 << _7.1;
_8.0 = _10 * _1.0;
_2.1 = !_12.1;
_8 = _2;
_1.1 = !_6;
_6 = _2.1 >> _4;
_2.1 = _11 & _9;
_2.0 = _1.0 + _10;
RET = 0_usize as f32;
_12 = (_1.0, _2.1);
_6 = 1_usize as isize;
_12.1 = _2.1;
_2.0 = _12.0;
RET = _4 as f32;
_11 = _8.1 + _8.1;
_6 = _1.1;
_1.1 = 275322684582425477939526538377292072297_u128 as isize;
_12 = (_7.0, _6);
_2.0 = _7.0;
Goto(bb1)
}
bb1 = {
Call(_15 = dump_var(12_usize, 9_usize, Move(_9), 6_usize, Move(_6), 16_usize, _16, 16_usize, _16), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: isize,mut _2: i8,mut _3: (f32, isize),mut _4: *mut (f32, isize),mut _5: (f32, isize),mut _6: isize,mut _7: (f32, isize),mut _8: isize,mut _9: isize,mut _10: *mut (f32, isize),mut _11: (f32, isize)) -> isize {
mir! {
type RET = isize;
let _12: ();
let _13: ();
{
RET = true as isize;
RET = !_6;
_6 = _1;
_7 = (_5.0, (*_4).1);
Goto(bb1)
}
bb1 = {
Call(_12 = dump_var(13_usize, 9_usize, Move(_9), 6_usize, Move(_6), 13_usize, _13, 13_usize, _13), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: *mut u32,mut _2: *const u16,mut _3: (u32, u16, f64),mut _4: (u32, u16, f64),mut _5: *mut (f32, isize),mut _6: f64,mut _7: f32,mut _8: *const u16,mut _9: *const u16,mut _10: u16,mut _11: (u32, u16, f64)) -> ((f32, isize), i128, usize) {
mir! {
type RET = ((f32, isize), i128, usize);
let _12: Adt54;
let _13: char;
let _14: isize;
let _15: Adt42;
let _16: (i128,);
let _17: *const u16;
let _18: (f32, isize);
let _19: (usize,);
let _20: (bool,);
let _21: f64;
let _22: *mut u32;
let _23: Adt46;
let _24: u32;
let _25: u8;
let _26: isize;
let _27: (bool,);
let _28: Adt51;
let _29: Adt42;
let _30: i64;
let _31: ();
let _32: ();
{
RET.0.1 = !(*_5).1;
(*_5).0 = -_7;
_4.2 = 7127885777212926067_usize as f64;
RET.1 = !38530333716844467424317738220992573548_i128;
(*_5) = (_7, RET.0.1);
(*_5).0 = 6429_i16 as f32;
_13 = '\u{84fbf}';
RET = ((*_5), 123292008404721772992541204687600257475_i128, 1194004512613807402_usize);
(*_5) = (_7, RET.0.1);
_3.1 = (*_2);
(*_8) = _10;
(*_5).1 = RET.0.1;
RET.2 = 6_usize - 1_usize;
_4.0 = _3.0 >> _10;
(*_5).0 = RET.0.0;
(*_2) = _4.1 | _11.1;
RET = ((*_5), 123724500241746034094386091375580816637_i128, 7_usize);
match RET.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
7 => bb8,
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
(*_8) = _3.1;
RET = ((*_5), (-163924573484264555139850586328392459639_i128), 1759049419117253192_usize);
_4.1 = (*_9) << _3.1;
(*_9) = _10;
_16.0 = RET.1 + RET.1;
RET.1 = 7509146904538050178_i64 as i128;
_9 = core::ptr::addr_of!((*_9));
_11.1 = !_4.1;
(*_5) = RET.0;
(*_8) = _11.1;
_16 = (RET.1,);
RET.1 = _16.0;
_5 = core::ptr::addr_of_mut!((*_5));
RET = ((*_5), _16.0, 5_usize);
_11.1 = (*_2);
_3.0 = _4.0 ^ _4.0;
Goto(bb9)
}
bb9 = {
_8 = core::ptr::addr_of!(_10);
_4.1 = (*_2) + (*_8);
_3.2 = -_11.2;
RET.0 = (*_5);
(*_2) = (*_8) + _3.1;
_4.1 = (*_9);
(*_8) = 14559836933333556642_u64 as u16;
_16 = (RET.1,);
RET.1 = _6 as i128;
RET.0 = (_7, (*_5).1);
RET = ((*_5), _16.0, 9472791580111983003_usize);
(*_1) = _3.0;
_8 = _9;
_11 = ((*_1), (*_8), _3.2);
_4.2 = _6 - _3.2;
_2 = _9;
_18 = (*_5);
_5 = core::ptr::addr_of_mut!((*_5));
(*_5) = (RET.0.0, RET.0.1);
_18.1 = RET.0.1;
RET.0 = (_18.0, (*_5).1);
(*_5) = (_7, RET.0.1);
_5 = core::ptr::addr_of_mut!((*_5));
_11.0 = (*_1) - (*_1);
_5 = core::ptr::addr_of_mut!(RET.0);
(*_5) = _18;
(*_8) = _11.1;
Goto(bb10)
}
bb10 = {
_2 = core::ptr::addr_of!(_4.1);
_16.0 = RET.1;
(*_1) = _3.0 << _3.1;
_3.2 = -_6;
RET.1 = -_16.0;
(*_5).1 = (*_2) as isize;
(*_5).1 = _18.1 * _18.1;
(*_9) = !(*_2);
_16.0 = (*_2) as i128;
RET.2 = 5528384831770954904_usize;
_16.0 = RET.1;
_14 = RET.0.1;
RET.2 = (-1664005572_i32) as usize;
RET.0.0 = -_7;
_7 = (*_5).0;
_11 = _3;
_4.0 = _3.0 << (*_1);
_3.0 = _11.0 + (*_1);
RET.0 = _18;
_10 = !_4.1;
RET = (_18, _16.0, 5072036645704638256_usize);
_3 = (_4.0, _4.1, _6);
(*_8) = true as u16;
_9 = core::ptr::addr_of!((*_9));
_17 = _9;
(*_5).1 = _14;
_9 = core::ptr::addr_of!(_3.1);
(*_9) = (-101_i8) as u16;
_3.0 = _16.0 as u32;
_7 = (*_5).0 - _18.0;
(*_1) = !_4.0;
Call(_16.0 = core::intrinsics::bswap(RET.1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_19 = (RET.2,);
_8 = core::ptr::addr_of!((*_2));
_20.0 = (*_1) <= (*_1);
(*_5).1 = -_18.1;
_7 = (*_5).0;
_10 = (*_8);
_20 = (false,);
(*_8) = _11.1 | _10;
_16.0 = RET.1 | RET.1;
(*_17) = _7 as u16;
_26 = (*_5).0 as isize;
_4.2 = _6 + _11.2;
RET = (_18, _16.0, _19.0);
_3 = ((*_1), (*_8), _11.2);
RET.0.1 = _14;
(*_5).1 = _14 - _14;
_3.1 = (*_2) + (*_8);
_3 = (_11.0, (*_2), _11.2);
_20 = (true,);
_7 = RET.0.0;
_3 = ((*_1), (*_2), _4.2);
RET = (_18, _16.0, _19.0);
_13 = '\u{b57e7}';
match RET.2 {
0 => bb7,
1 => bb2,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
5072036645704638256 => bb17,
_ => bb16
}
}
bb12 = {
_2 = core::ptr::addr_of!(_4.1);
_16.0 = RET.1;
(*_1) = _3.0 << _3.1;
_3.2 = -_6;
RET.1 = -_16.0;
(*_5).1 = (*_2) as isize;
(*_5).1 = _18.1 * _18.1;
(*_9) = !(*_2);
_16.0 = (*_2) as i128;
RET.2 = 5528384831770954904_usize;
_16.0 = RET.1;
_14 = RET.0.1;
RET.2 = (-1664005572_i32) as usize;
RET.0.0 = -_7;
_7 = (*_5).0;
_11 = _3;
_4.0 = _3.0 << (*_1);
_3.0 = _11.0 + (*_1);
RET.0 = _18;
_10 = !_4.1;
RET = (_18, _16.0, 5072036645704638256_usize);
_3 = (_4.0, _4.1, _6);
(*_8) = true as u16;
_9 = core::ptr::addr_of!((*_9));
_17 = _9;
(*_5).1 = _14;
_9 = core::ptr::addr_of!(_3.1);
(*_9) = (-101_i8) as u16;
_3.0 = _16.0 as u32;
_7 = (*_5).0 - _18.0;
(*_1) = !_4.0;
Call(_16.0 = core::intrinsics::bswap(RET.1), ReturnTo(bb11), UnwindUnreachable())
}
bb13 = {
Return()
}
bb14 = {
(*_8) = _3.1;
RET = ((*_5), (-163924573484264555139850586328392459639_i128), 1759049419117253192_usize);
_4.1 = (*_9) << _3.1;
(*_9) = _10;
_16.0 = RET.1 + RET.1;
RET.1 = 7509146904538050178_i64 as i128;
_9 = core::ptr::addr_of!((*_9));
_11.1 = !_4.1;
(*_5) = RET.0;
(*_8) = _11.1;
_16 = (RET.1,);
RET.1 = _16.0;
_5 = core::ptr::addr_of_mut!((*_5));
RET = ((*_5), _16.0, 5_usize);
_11.1 = (*_2);
_3.0 = _4.0 ^ _4.0;
Goto(bb9)
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
(*_8) = !_10;
_24 = (*_1) * _4.0;
_27.0 = _20.0;
_8 = _2;
_3.1 = (*_8) - (*_2);
_4.1 = (*_9) | (*_9);
_4.2 = _6 * _3.2;
_13 = '\u{e190e}';
_11 = (_4.0, _4.1, _4.2);
_20.0 = (*_8) != _10;
_6 = -_3.2;
(*_5).0 = RET.1 as f32;
_11 = (_24, (*_9), _4.2);
_8 = _2;
_7 = RET.0.0;
_11.0 = _7 as u32;
(*_5).0 = _18.0;
_11.2 = _3.2;
_1 = core::ptr::addr_of_mut!(_11.0);
_3.1 = !_11.1;
(*_5).0 = _7;
_24 = _3.0 | _4.0;
Goto(bb18)
}
bb18 = {
Call(_31 = dump_var(14_usize, 13_usize, Move(_13), 24_usize, Move(_24), 20_usize, Move(_20), 27_usize, Move(_27)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{a1733}'), std::hint::black_box(71535120891539547104633723948304914084_u128), std::hint::black_box((-113_i8)), std::hint::black_box((-32733_i16)), std::hint::black_box(897012224_i32), std::hint::black_box(6135362454374108538_u64), std::hint::black_box(111282336933512716891483625733068885428_i128), std::hint::black_box(14580968434636393317_usize), std::hint::black_box(51_u8), std::hint::black_box(13367_u16));
                
            }
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: bool,
fld1: f32,
fld2: isize,
fld3: i64,
fld4: (i128,),
fld5: (f64, u8, i8, char),

},
Variant1{
fld0: bool,
fld1: (i8, u64),
fld2: i128,
fld3: [i8; 7],
fld4: [i64; 8],
fld5: i32,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: *const *mut i32,
fld1: *mut i32,
fld2: usize,
fld3: i8,
fld4: [u128; 6],

},
Variant1{
fld0: u8,
fld1: [i64; 8],
fld2: (i128,),
fld3: i8,

},
Variant2{
fld0: [i16; 1],

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: *mut (f32, isize),
fld1: u64,
fld2: isize,
fld3: [u128; 6],
fld4: f32,
fld5: u8,
fld6: u128,
fld7: usize,

},
Variant1{
fld0: [i8; 7],
fld1: [bool; 7],
fld2: [i16; 1],
fld3: i8,
fld4: (f64, f32),
fld5: *mut i32,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: *mut i32,
fld1: u64,
fld2: [u128; 6],
fld3: (u32, u16, f64),
fld4: f64,
fld5: (f64, f32),
fld6: *mut (f32, isize),

},
Variant1{
fld0: *const u16,

},
Variant2{
fld0: (i32, *mut i32, u32, i32),
fld1: *mut i32,
fld2: isize,
fld3: u32,
fld4: (f32, (f64, u8, i8, char), u32, u64),
fld5: (usize,),
fld6: u128,
fld7: (f64, u8, i8, char),

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: i128,

},
Variant1{
fld0: (f64, f32),
fld1: *mut (f32, isize),
fld2: isize,
fld3: Adt42,

},
Variant2{
fld0: [i8; 7],
fld1: (usize,),
fld2: u32,
fld3: (u32, u16, f64),
fld4: (f32, (f64, u8, i8, char), u32, u64),
fld5: (i128,),
fld6: [u64; 1],
fld7: usize,

},
Variant3{
fld0: isize,
fld1: i64,

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: *const u16,
fld1: (f32, isize),

},
Variant1{
fld0: (usize,),
fld1: u128,
fld2: Adt42,
fld3: *mut (f32, isize),
fld4: f64,
fld5: i32,
fld6: u32,
fld7: [i8; 7],

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: Adt47,
fld1: i16,

},
Variant1{
fld0: ((f32, isize), i128, usize),
fld1: u32,
fld2: (i32, *mut i32, u32, i32),
fld3: i8,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: [u64; 1],
fld1: [i16; 1],
fld2: i128,
fld3: Adt45,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: [i8; 7],
fld1: (i8, u64),
fld2: i32,
fld3: u16,
fld4: i64,

},
Variant1{
fld0: *mut i32,
fld1: u32,
fld2: [i8; 7],
fld3: i32,

},
Variant2{
fld0: [u64; 1],
fld1: u8,
fld2: *const u16,
fld3: (i32, *mut i32, u32, i32),
fld4: [bool; 7],
fld5: (f32, isize),
fld6: [i64; 8],
fld7: (f64, f32),

},
Variant3{
fld0: [u64; 1],
fld1: char,
fld2: usize,
fld3: [i8; 7],
fld4: (i128,),
fld5: (f64, f32),

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: (i32, *mut i32, u32, i32),
fld1: f32,
fld2: u16,
fld3: (i128,),
fld4: u128,
fld5: i32,
fld6: [i8; 7],

},
Variant1{
fld0: (f32, (f64, u8, i8, char), u32, u64),

},
Variant2{
fld0: Adt47,
fld1: usize,
fld2: [u64; 1],
fld3: (i32, *mut i32, u32, i32),
fld4: i128,
fld5: [bool; 7],

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: (u32, u16, f64),
fld1: u16,
fld2: isize,
fld3: (i128,),
fld4: [i64; 8],
fld5: Adt49,
fld6: Adt51,
fld7: (f32, (f64, u8, i8, char), u32, u64),

},
Variant1{
fld0: [i64; 8],
fld1: f32,
fld2: [bool; 7],
fld3: i8,
fld4: (f64, f32),
fld5: Adt46,
fld6: [i8; 7],
fld7: Adt51,

},
Variant2{
fld0: (u32, u16, f64),
fld1: (i128,),
fld2: [i64; 8],
fld3: (bool,),
fld4: u8,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: ((f32, isize), i128, usize),
fld1: f32,
}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld2:".as_ptr())};
		fld2.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld3:".as_ptr())};
		fld3.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld4:".as_ptr())};
		fld4.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld5:".as_ptr())};
		fld5.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld6:".as_ptr())};
		fld6.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: [i64; 8],
fld1: (f64, u8, i8, char),

},
Variant1{
fld0: Adt45,
fld1: *const *mut i32,
fld2: isize,
fld3: (f32, (f64, u8, i8, char), u32, u64),
fld4: *mut i32,
fld5: Adt53,
fld6: [i16; 1],
fld7: i128,

},
Variant2{
fld0: (i32, *mut i32, u32, i32),
fld1: char,
fld2: Adt46,
fld3: u16,
fld4: (i8, u64),
fld5: (usize,),

},
Variant3{
fld0: bool,
fld1: u128,
fld2: Adt52,
fld3: Adt50,
fld4: Adt42,
fld5: Adt46,
fld6: [bool; 7],

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt55{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt55 {
fld0: Adt45,
fld1: [i16; 1],
fld2: (usize,),
fld3: i8,
fld4: [u64; 1],
fld5: Adt42,
fld6: [i8; 7],
}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt56{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt56 {
fld0: u8,
fld1: (f32, isize),
fld2: *mut i32,
fld3: Adt48,
fld4: [u64; 1],
fld5: (i128,),
fld6: (usize,),
}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt57{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt57 {
fld0: u8,
fld1: Adt49,
fld2: Adt50,
fld3: [bool; 7],
fld4: Adt54,
fld5: (u32, u16, f64),
fld6: (f64, f32),
fld7: (bool,),
}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt58{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt58 {
fld0: bool,
fld1: u128,
fld2: usize,
fld3: (f32, isize),
fld4: (f64, u8, i8, char),
fld5: f64,
fld6: i64,
}

