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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: u128,mut _4: i8,mut _5: i16,mut _6: u32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u64) -> *const i8 {
mir! {
type RET = *const i8;
let _12: f64;
let _13: f32;
let _14: f64;
let _15: ([char; 8],);
let _16: [char; 8];
let _17: [u64; 8];
let _18: i32;
let _19: u8;
let _20: (*const *const (u32, [i8; 4], u16, [i8; 4]),);
let _21: f32;
let _22: u16;
let _23: [isize; 1];
let _24: (u128, *mut u8);
let _25: u32;
let _26: isize;
let _27: [u64; 8];
let _28: ();
let _29: ();
{
_9 = 6931906324142848297_usize ^ 4_usize;
_9 = 16781013789688718489_usize | 7_usize;
_10 = 119_i8 as u8;
_8 = 116687316275579716273342695873803256001_i128;
_1 = _9 > _9;
_10 = 79_u8;
_6 = 59802_u16 as u32;
RET = core::ptr::addr_of!(_4);
RET = core::ptr::addr_of!(_4);
(*RET) = _10 as i8;
_3 = (-34_isize) as u128;
(*RET) = 124_i8;
_6 = 9353206772322530388_u64 as u32;
_11 = 1433890919002223507_u64;
RET = core::ptr::addr_of!(_4);
_13 = 52698_u16 as f32;
_7 = (-3428715066595127745_i64) | 2649445590610175161_i64;
_12 = _8 as f64;
_9 = 1_usize;
_1 = !true;
_6 = 2865890980_u32 << _3;
_13 = 105_isize as f32;
_10 = !171_u8;
_5 = 26443_i16 >> _4;
_3 = 191641636153192214502552336906645548295_u128 + 206832904083403043605060163715817828115_u128;
RET = core::ptr::addr_of!(_4);
Call(_16 = fn1(_12, _7, RET, _9, _6, (*RET), _7, _6, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = _1 as u32;
_4 = 68_i8 | (-121_i8);
_13 = _11 as f32;
_14 = 29045_u16 as f64;
(*RET) = '\u{90568}' as i8;
_11 = 3982_u16 as u64;
(*RET) = -(-68_i8);
(*RET) = (-92_i8) + 73_i8;
_13 = _6 as f32;
_14 = -_12;
_2 = '\u{afd4c}';
_14 = _12 + _12;
_11 = _5 as u64;
(*RET) = _9 as i8;
(*RET) = (-61_i8) ^ 125_i8;
_6 = _9 as u32;
_1 = !false;
_8 = (-73922506824480953689605547970071510584_i128) + 127925945585559603603481291393807647991_i128;
Goto(bb2)
}
bb2 = {
_11 = 723079466561793657_u64;
(*RET) = -(-64_i8);
_10 = 42_u8;
_3 = _5 as u128;
_14 = _12 * _12;
(*RET) = _14 as i8;
_17 = [_11,_11,_11,_11,_11,_11,_11,_11];
_8 = _2 as i128;
_15 = (_16,);
(*RET) = _6 as i8;
RET = core::ptr::addr_of!(_4);
_18 = !456967690_i32;
RET = core::ptr::addr_of!((*RET));
(*RET) = (-87_i8);
_7 = 1178068111233053606_i64;
_5 = 13645_i16;
_18 = !(-782844424_i32);
_17 = [_11,_11,_11,_11,_11,_11,_11,_11];
_10 = !129_u8;
_13 = _8 as f32;
_15 = (_16,);
_2 = '\u{82857}';
_14 = _5 as f64;
_18 = (-61337084_i32);
_8 = !(-91561119996488258704616087458783411928_i128);
match _18 {
0 => bb3,
1 => bb4,
340282366920938463463374607431706874372 => bb6,
_ => bb5
}
}
bb3 = {
_6 = _1 as u32;
_4 = 68_i8 | (-121_i8);
_13 = _11 as f32;
_14 = 29045_u16 as f64;
(*RET) = '\u{90568}' as i8;
_11 = 3982_u16 as u64;
(*RET) = -(-68_i8);
(*RET) = (-92_i8) + 73_i8;
_13 = _6 as f32;
_14 = -_12;
_2 = '\u{afd4c}';
_14 = _12 + _12;
_11 = _5 as u64;
(*RET) = _9 as i8;
(*RET) = (-61_i8) ^ 125_i8;
_6 = _9 as u32;
_1 = !false;
_8 = (-73922506824480953689605547970071510584_i128) + 127925945585559603603481291393807647991_i128;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_3 = 180237452717258294275177511238725390351_u128;
(*RET) = (-91_i8) >> _10;
(*RET) = 56_i8 >> _18;
_6 = _1 as u32;
_13 = _3 as f32;
_21 = _6 as f32;
(*RET) = -(-34_i8);
(*RET) = _5 as i8;
_22 = !57932_u16;
_23 = [9223372036854775807_isize];
_22 = 29943_u16 + 47231_u16;
_17 = [_11,_11,_11,_11,_11,_11,_11,_11];
_15.0 = _16;
_19 = _10;
match _18 {
0 => bb7,
340282366920938463463374607431706874372 => bb9,
_ => bb8
}
}
bb7 = {
_11 = 723079466561793657_u64;
(*RET) = -(-64_i8);
_10 = 42_u8;
_3 = _5 as u128;
_14 = _12 * _12;
(*RET) = _14 as i8;
_17 = [_11,_11,_11,_11,_11,_11,_11,_11];
_8 = _2 as i128;
_15 = (_16,);
(*RET) = _6 as i8;
RET = core::ptr::addr_of!(_4);
_18 = !456967690_i32;
RET = core::ptr::addr_of!((*RET));
(*RET) = (-87_i8);
_7 = 1178068111233053606_i64;
_5 = 13645_i16;
_18 = !(-782844424_i32);
_17 = [_11,_11,_11,_11,_11,_11,_11,_11];
_10 = !129_u8;
_13 = _8 as f32;
_15 = (_16,);
_2 = '\u{82857}';
_14 = _5 as f64;
_18 = (-61337084_i32);
_8 = !(-91561119996488258704616087458783411928_i128);
match _18 {
0 => bb3,
1 => bb4,
340282366920938463463374607431706874372 => bb6,
_ => bb5
}
}
bb8 = {
Return()
}
bb9 = {
match _18 {
0 => bb1,
1 => bb8,
2 => bb3,
3 => bb10,
4 => bb11,
5 => bb12,
340282366920938463463374607431706874372 => bb14,
_ => bb13
}
}
bb10 = {
Return()
}
bb11 = {
_11 = 723079466561793657_u64;
(*RET) = -(-64_i8);
_10 = 42_u8;
_3 = _5 as u128;
_14 = _12 * _12;
(*RET) = _14 as i8;
_17 = [_11,_11,_11,_11,_11,_11,_11,_11];
_8 = _2 as i128;
_15 = (_16,);
(*RET) = _6 as i8;
RET = core::ptr::addr_of!(_4);
_18 = !456967690_i32;
RET = core::ptr::addr_of!((*RET));
(*RET) = (-87_i8);
_7 = 1178068111233053606_i64;
_5 = 13645_i16;
_18 = !(-782844424_i32);
_17 = [_11,_11,_11,_11,_11,_11,_11,_11];
_10 = !129_u8;
_13 = _8 as f32;
_15 = (_16,);
_2 = '\u{82857}';
_14 = _5 as f64;
_18 = (-61337084_i32);
_8 = !(-91561119996488258704616087458783411928_i128);
match _18 {
0 => bb3,
1 => bb4,
340282366920938463463374607431706874372 => bb6,
_ => bb5
}
}
bb12 = {
_3 = 180237452717258294275177511238725390351_u128;
(*RET) = (-91_i8) >> _10;
(*RET) = 56_i8 >> _18;
_6 = _1 as u32;
_13 = _3 as f32;
_21 = _6 as f32;
(*RET) = -(-34_i8);
(*RET) = _5 as i8;
_22 = !57932_u16;
_23 = [9223372036854775807_isize];
_22 = 29943_u16 + 47231_u16;
_17 = [_11,_11,_11,_11,_11,_11,_11,_11];
_15.0 = _16;
_19 = _10;
match _18 {
0 => bb7,
340282366920938463463374607431706874372 => bb9,
_ => bb8
}
}
bb13 = {
Return()
}
bb14 = {
_14 = -_12;
_7 = _22 as i64;
_16 = [_2,_2,_2,_2,_2,_2,_2,_2];
_19 = _10 << _3;
_6 = _5 as u32;
_8 = _6 as i128;
_22 = 63451_u16 << _19;
_5 = 14624_i16 ^ (-26383_i16);
_17 = [_11,_11,_11,_11,_11,_11,_11,_11];
_11 = 3565554590163298019_u64;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(0_usize, 4_usize, Move(_4), 10_usize, Move(_10), 19_usize, Move(_19), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(0_usize, 9_usize, Move(_9), 16_usize, Move(_16), 2_usize, Move(_2), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(0_usize, 18_usize, Move(_18), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: f64,mut _2: i64,mut _3: *const i8,mut _4: usize,mut _5: u32,mut _6: i8,mut _7: i64,mut _8: u32,mut _9: i16) -> [char; 8] {
mir! {
type RET = [char; 8];
let _10: f32;
let _11: bool;
let _12: [char; 8];
let _13: isize;
let _14: i32;
let _15: isize;
let _16: [i8; 4];
let _17: Adt46;
let _18: [isize; 1];
let _19: f32;
let _20: isize;
let _21: *const i128;
let _22: [i8; 4];
let _23: [i128; 2];
let _24: i16;
let _25: ([char; 8],);
let _26: [i128; 2];
let _27: f64;
let _28: [i8; 4];
let _29: i128;
let _30: ();
let _31: ();
{
RET = ['\u{3f552}','\u{56573}','\u{ee6e7}','\u{367d3}','\u{ba0f6}','\u{b77b}','\u{44efd}','\u{83f39}'];
(*_3) = _6 + _6;
RET[_4] = '\u{9aaba}';
(*_3) = _6;
RET = ['\u{8cf19}','\u{c06b7}','\u{7c3a}','\u{ab2fd}','\u{2b1bc}','\u{10d4e}','\u{54587}','\u{8ffd8}'];
RET = ['\u{5c994}','\u{8590b}','\u{4449b}','\u{10db23}','\u{c6e}','\u{9f5da}','\u{46c06}','\u{e46ff}'];
RET[_4] = '\u{52400}';
_5 = _8 * _8;
RET = ['\u{c86cc}','\u{4318}','\u{2d903}','\u{75e60}','\u{60e47}','\u{becb8}','\u{d345c}','\u{1edd5}'];
_10 = _9 as f32;
_5 = !_8;
_13 = (-98_isize) | (-20_isize);
_12[_4] = RET[_4];
_7 = -_2;
_9 = _12[_4] as i16;
_1 = _9 as f64;
_7 = _2;
_3 = core::ptr::addr_of!((*_3));
_9 = (-25979_i16) * (-27102_i16);
_4 = !11623331681824574741_usize;
match _6 {
124 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_2 = _7;
_11 = true;
_3 = core::ptr::addr_of!((*_3));
(*_3) = -_6;
Call(RET = fn2(_1, _1, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_12 = RET;
_12 = ['\u{4acd9}','\u{70941}','\u{c651}','\u{a7350}','\u{7b5b0}','\u{65087}','\u{a3746}','\u{d442b}'];
_14 = 692752659_i32 - 990163365_i32;
_3 = core::ptr::addr_of!((*_3));
Call(_6 = fn3(RET, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_5 = _8;
_11 = false;
_9 = _13 as i16;
_14 = 1387306296_i32;
_4 = 111941430515517871865125162450572337709_i128 as usize;
_15 = !_13;
_17.fld1.fld6 = -_1;
_15 = _13 ^ _13;
_8 = '\u{44d7}' as u32;
_3 = core::ptr::addr_of!(_6);
_1 = _17.fld1.fld6 + _17.fld1.fld6;
_17.fld1.fld1 = '\u{e8756}';
_7 = 136_u8 as i64;
_17.fld1.fld6 = -_1;
_17.fld1.fld5 = _14 >> _9;
_17.fld1.fld6 = _15 as f64;
_17.fld0 = [(*_3),_6,(*_3),(*_3)];
match _14 {
0 => bb3,
1 => bb2,
1387306296 => bb6,
_ => bb5
}
}
bb5 = {
_12 = RET;
_12 = ['\u{4acd9}','\u{70941}','\u{c651}','\u{a7350}','\u{7b5b0}','\u{65087}','\u{a3746}','\u{d442b}'];
_14 = 692752659_i32 - 990163365_i32;
_3 = core::ptr::addr_of!((*_3));
Call(_6 = fn3(RET, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_17.fld1.fld0 = _8 + _5;
_6 = !13_i8;
_3 = core::ptr::addr_of!((*_3));
(*_3) = -(-82_i8);
_7 = _2;
_17.fld1.fld2 = [(*_3),(*_3),(*_3),_6];
_19 = _10;
_1 = -_17.fld1.fld6;
(*_3) = _19 as i8;
_9 = (-12212_i16) * 32460_i16;
_16 = _17.fld1.fld2;
_18 = [_15];
_6 = 48_i8 ^ (-5_i8);
Goto(bb7)
}
bb7 = {
_5 = _17.fld1.fld0 - _17.fld1.fld0;
_17.fld1.fld2 = [(*_3),(*_3),(*_3),_6];
_16 = [(*_3),_6,(*_3),(*_3)];
_11 = true;
match _14 {
0 => bb3,
1 => bb5,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
1387306296 => bb13,
_ => bb12
}
}
bb8 = {
_17.fld1.fld0 = _8 + _5;
_6 = !13_i8;
_3 = core::ptr::addr_of!((*_3));
(*_3) = -(-82_i8);
_7 = _2;
_17.fld1.fld2 = [(*_3),(*_3),(*_3),_6];
_19 = _10;
_1 = -_17.fld1.fld6;
(*_3) = _19 as i8;
_9 = (-12212_i16) * 32460_i16;
_16 = _17.fld1.fld2;
_18 = [_15];
_6 = 48_i8 ^ (-5_i8);
Goto(bb7)
}
bb9 = {
_12 = RET;
_12 = ['\u{4acd9}','\u{70941}','\u{c651}','\u{a7350}','\u{7b5b0}','\u{65087}','\u{a3746}','\u{d442b}'];
_14 = 692752659_i32 - 990163365_i32;
_3 = core::ptr::addr_of!((*_3));
Call(_6 = fn3(RET, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
_12 = RET;
_12 = ['\u{4acd9}','\u{70941}','\u{c651}','\u{a7350}','\u{7b5b0}','\u{65087}','\u{a3746}','\u{d442b}'];
_14 = 692752659_i32 - 990163365_i32;
_3 = core::ptr::addr_of!((*_3));
Call(_6 = fn3(RET, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb12 = {
_2 = _7;
_11 = true;
_3 = core::ptr::addr_of!((*_3));
(*_3) = -_6;
Call(RET = fn2(_1, _1, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb13 = {
_14 = _17.fld1.fld5;
RET = [_17.fld1.fld1,_17.fld1.fld1,_17.fld1.fld1,_17.fld1.fld1,_17.fld1.fld1,_17.fld1.fld1,_17.fld1.fld1,_17.fld1.fld1];
_5 = _17.fld1.fld0;
_17.fld1.fld2 = _16;
_11 = true;
(*_3) = (-110_i8) ^ (-48_i8);
_9 = (-15842_i16) | (-3192_i16);
_20 = _15;
RET = _12;
_17.fld1.fld5 = _14;
_17.fld3 = _1 as u128;
(*_3) = 63_i8;
(*_3) = _11 as i8;
_22 = [_6,(*_3),_6,(*_3)];
_17.fld1.fld6 = _1;
(*_3) = 101_i8;
match (*_3) {
0 => bb8,
1 => bb14,
101 => bb16,
_ => bb15
}
}
bb14 = {
_2 = _7;
_11 = true;
_3 = core::ptr::addr_of!((*_3));
(*_3) = -_6;
Call(RET = fn2(_1, _1, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb15 = {
Return()
}
bb16 = {
_17.fld0 = [_6,(*_3),_6,(*_3)];
_8 = _5 ^ _5;
RET = _12;
_12 = RET;
_23 = [68152331260721384173984365397950035651_i128,160218441418122603875769602144964038221_i128];
_17.fld0 = [(*_3),(*_3),_6,(*_3)];
_17.fld1.fld4 = _23;
_19 = -_10;
_5 = _8;
_24 = -_9;
_17.fld1.fld4 = [(-45280322399826645456566466979651851096_i128),(-155274055579427391989020364509295761491_i128)];
_17.fld1.fld6 = -_1;
_20 = _15;
_17.fld3 = 123348254221282934859406729173137109976_u128 - 93650207976574156646169336437560723661_u128;
_23 = [(-151617845475505171090004772996986380125_i128),116162119330136681198821998131820070995_i128];
_6 = 114_i8 - 7_i8;
(*_3) = (-38_i8) & 21_i8;
_26 = [(-72769463354306359562955545858140538933_i128),(-39136053503208145527220133512687943320_i128)];
_7 = !_2;
_27 = _17.fld3 as f64;
_13 = _15 * _20;
_23 = _17.fld1.fld4;
_14 = -_17.fld1.fld5;
_17.fld1.fld6 = -_27;
_29 = _24 as i128;
Goto(bb17)
}
bb17 = {
Call(_30 = dump_var(1_usize, 24_usize, Move(_24), 14_usize, Move(_14), 20_usize, Move(_20), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_30 = dump_var(1_usize, 15_usize, Move(_15), 13_usize, Move(_13), 11_usize, Move(_11), 2_usize, Move(_2)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_30 = dump_var(1_usize, 5_usize, Move(_5), 7_usize, Move(_7), 31_usize, _31, 31_usize, _31), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: f64,mut _2: f64,mut _3: i8) -> [char; 8] {
mir! {
type RET = [char; 8];
let _4: (*const u8,);
let _5: [i128; 2];
let _6: [u64; 8];
let _7: (u64, u32, i16, u16);
let _8: [i128; 2];
let _9: [i8; 4];
let _10: isize;
let _11: i32;
let _12: ([char; 8],);
let _13: u128;
let _14: char;
let _15: [i8; 4];
let _16: char;
let _17: i128;
let _18: u64;
let _19: i64;
let _20: u32;
let _21: isize;
let _22: f64;
let _23: isize;
let _24: i64;
let _25: [isize; 1];
let _26: [i128; 2];
let _27: (*const u8,);
let _28: f64;
let _29: f32;
let _30: ();
let _31: ();
{
RET = ['\u{24567}','\u{b2f7d}','\u{8ebc3}','\u{10fe3c}','\u{a7b4b}','\u{c326b}','\u{5843e}','\u{b6c}'];
_1 = _2 * _2;
_3 = (-86_i8) & 117_i8;
_2 = _1;
_2 = _1;
RET = ['\u{66bd6}','\u{791e1}','\u{f6b05}','\u{49da0}','\u{18af2}','\u{72b71}','\u{ea669}','\u{101c64}'];
_2 = _1 - _1;
_3 = 84_i8;
RET = ['\u{785a7}','\u{ab71e}','\u{c871b}','\u{57c8c}','\u{b9b7a}','\u{3a805}','\u{c0210}','\u{3a4f4}'];
RET = ['\u{b82c2}','\u{33b25}','\u{ed1fe}','\u{e1b25}','\u{86e53}','\u{acae5}','\u{c9ba0}','\u{b1dd}'];
_2 = _1 - _1;
_2 = -_1;
_2 = _1;
RET = ['\u{b805b}','\u{812f6}','\u{53fed}','\u{995cc}','\u{3fa63}','\u{dba84}','\u{ac98b}','\u{102fac}'];
RET = ['\u{702d4}','\u{c6e7d}','\u{391b1}','\u{1c1f2}','\u{a8100}','\u{304e1}','\u{b4c4a}','\u{f209b}'];
_3 = (-47_i8);
_1 = _2;
RET = ['\u{33aad}','\u{b873}','\u{1c274}','\u{495c5}','\u{ce906}','\u{e26f8}','\u{a165e}','\u{68324}'];
_2 = -_1;
_2 = _1;
_1 = -_2;
_2 = _1 + _1;
match _3 {
340282366920938463463374607431768211409 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_5 = [5715555995954090530283721047588259780_i128,139001268050091331833182196384525989199_i128];
_5 = [(-92371057397976431368515372188024510980_i128),64645291523837622246288959905886230536_i128];
RET = ['\u{23d43}','\u{7130a}','\u{54047}','\u{f82e}','\u{26636}','\u{7ba9b}','\u{f6d72}','\u{1dc56}'];
_3 = !35_i8;
_3 = (-117_i8);
_3 = 40_i8 | (-44_i8);
_6 = [11669776478533920827_u64,17521628341653755365_u64,14840598150384461519_u64,15794968042119317903_u64,18445745884389915683_u64,17804198651598458840_u64,15524543015623579682_u64,3958385154561172524_u64];
RET = ['\u{294ac}','\u{1257c}','\u{382b0}','\u{cbea3}','\u{a2e8d}','\u{8e249}','\u{6d78}','\u{79ca}'];
RET = ['\u{106063}','\u{61044}','\u{bb1e2}','\u{1d1ec}','\u{d87de}','\u{5acbc}','\u{ba866}','\u{381a1}'];
RET = ['\u{e7e27}','\u{682b6}','\u{f9000}','\u{88744}','\u{c090b}','\u{59e64}','\u{a602d}','\u{e49b3}'];
RET = ['\u{1c67e}','\u{cbdcc}','\u{27c65}','\u{2e403}','\u{816d8}','\u{32f1a}','\u{10c79a}','\u{750}'];
_2 = _1;
_3 = 214113150612329649651918632854298048550_u128 as i8;
RET = ['\u{867f}','\u{6d288}','\u{88735}','\u{49fb3}','\u{d6175}','\u{6faad}','\u{d590}','\u{41e38}'];
_5 = [49503461837656609880304865856205154958_i128,139309017838105571118261291413668150323_i128];
_2 = _1 - _1;
Goto(bb3)
}
bb3 = {
_3 = -104_i8;
_7 = (6868510990271789419_u64, 1681371005_u32, (-20197_i16), 30738_u16);
_1 = _2;
RET = ['\u{ceb9b}','\u{2d176}','\u{73bce}','\u{3f63e}','\u{c0f56}','\u{385ab}','\u{704d5}','\u{e2f13}'];
_7.2 = -(-31347_i16);
_1 = -_2;
_7.3 = 22362_u16 & 49161_u16;
_7.1 = _7.0 as u32;
_7.2 = 7199_i16 | (-8593_i16);
RET = ['\u{9b479}','\u{e2165}','\u{a2ca0}','\u{d13cd}','\u{cf9e2}','\u{dd7ff}','\u{bafa9}','\u{3cb51}'];
_6 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
_2 = _1;
_1 = -_2;
_7.1 = !237690710_u32;
_9 = [_3,_3,_3,_3];
_1 = _2;
RET = ['\u{6e0fb}','\u{59d74}','\u{6bc0d}','\u{35db7}','\u{425a}','\u{fad9d}','\u{1c615}','\u{f72ba}'];
_7.2 = 8734_i16;
match _7.0 {
6868510990271789419 => bb4,
_ => bb1
}
}
bb4 = {
_2 = _1 * _1;
_3 = -43_i8;
_3 = -(-94_i8);
RET = ['\u{c76e9}','\u{f69d}','\u{10c83c}','\u{bc349}','\u{99a95}','\u{ac317}','\u{b972c}','\u{38335}'];
_1 = _2 + _2;
_7.3 = 35994_u16;
_7 = (3647631248785521617_u64, 1819384313_u32, (-9039_i16), 31931_u16);
_6 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
_9 = [_3,_3,_3,_3];
_12.0 = ['\u{e383e}','\u{2d3b1}','\u{febba}','\u{9b5be}','\u{e71f5}','\u{5df9b}','\u{40b0c}','\u{e0729}'];
_7.1 = !2260555837_u32;
_7.2 = 360698677_i32 as i16;
_11 = 634312734_i32 + 1054388592_i32;
_12.0 = ['\u{83967}','\u{671b4}','\u{deefc}','\u{ba0c7}','\u{45035}','\u{12645}','\u{6a4ad}','\u{3645d}'];
_10 = 234021388583692429016728614319161270893_u128 as isize;
_10 = 9223372036854775807_isize;
_5 = [26935461771365602411596488314217412964_i128,(-66369128659612513861222143078152824263_i128)];
_9 = [_3,_3,_3,_3];
_7 = (1555277385234543731_u64, 1669836558_u32, 23068_i16, 28860_u16);
_7 = (10360282781690892624_u64, 3801441112_u32, 18743_i16, 40453_u16);
_5 = [(-110338699031982479982173534903995603461_i128),120607114901729397611767745313795793702_i128];
_15 = _9;
Goto(bb5)
}
bb5 = {
_7 = (4258638743972688391_u64, 3366675231_u32, (-13300_i16), 39189_u16);
RET = ['\u{53c98}','\u{36464}','\u{69ec9}','\u{5943e}','\u{fa3d6}','\u{ee49a}','\u{4de02}','\u{409ee}'];
_15 = [_3,_3,_3,_3];
RET = ['\u{60e71}','\u{e8482}','\u{919d9}','\u{32cc1}','\u{a0757}','\u{a97d1}','\u{7590d}','\u{14338}'];
_17 = 150622851107255834737912195181997880884_i128 | 153895796508926808719576311332174593199_i128;
_2 = _17 as f64;
_5 = [_17,_17];
_7.0 = 5618170631436694605_u64;
match _7.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
39189 => bb7,
_ => bb6
}
}
bb6 = {
_3 = -104_i8;
_7 = (6868510990271789419_u64, 1681371005_u32, (-20197_i16), 30738_u16);
_1 = _2;
RET = ['\u{ceb9b}','\u{2d176}','\u{73bce}','\u{3f63e}','\u{c0f56}','\u{385ab}','\u{704d5}','\u{e2f13}'];
_7.2 = -(-31347_i16);
_1 = -_2;
_7.3 = 22362_u16 & 49161_u16;
_7.1 = _7.0 as u32;
_7.2 = 7199_i16 | (-8593_i16);
RET = ['\u{9b479}','\u{e2165}','\u{a2ca0}','\u{d13cd}','\u{cf9e2}','\u{dd7ff}','\u{bafa9}','\u{3cb51}'];
_6 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
_2 = _1;
_1 = -_2;
_7.1 = !237690710_u32;
_9 = [_3,_3,_3,_3];
_1 = _2;
RET = ['\u{6e0fb}','\u{59d74}','\u{6bc0d}','\u{35db7}','\u{425a}','\u{fad9d}','\u{1c615}','\u{f72ba}'];
_7.2 = 8734_i16;
match _7.0 {
6868510990271789419 => bb4,
_ => bb1
}
}
bb7 = {
_15 = [_3,_3,_3,_3];
_5 = [_17,_17];
_18 = true as u64;
Goto(bb8)
}
bb8 = {
_13 = !47035807421064546286246167840018098971_u128;
_17 = 5067220815246118401_i64 as i128;
_12 = (RET,);
_5 = [_17,_17];
_8 = _5;
_11 = false as i32;
_19 = _7.3 as i64;
Goto(bb9)
}
bb9 = {
_8 = [_17,_17];
_7.1 = !243708330_u32;
_17 = 113762564575627223228157252090661836256_i128 >> _7.0;
_14 = '\u{7359f}';
match _7.0 {
0 => bb5,
5618170631436694605 => bb11,
_ => bb10
}
}
bb10 = {
_7 = (4258638743972688391_u64, 3366675231_u32, (-13300_i16), 39189_u16);
RET = ['\u{53c98}','\u{36464}','\u{69ec9}','\u{5943e}','\u{fa3d6}','\u{ee49a}','\u{4de02}','\u{409ee}'];
_15 = [_3,_3,_3,_3];
RET = ['\u{60e71}','\u{e8482}','\u{919d9}','\u{32cc1}','\u{a0757}','\u{a97d1}','\u{7590d}','\u{14338}'];
_17 = 150622851107255834737912195181997880884_i128 | 153895796508926808719576311332174593199_i128;
_2 = _17 as f64;
_5 = [_17,_17];
_7.0 = 5618170631436694605_u64;
match _7.3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
39189 => bb7,
_ => bb6
}
}
bb11 = {
_21 = -_10;
RET = [_14,_14,_14,_14,_14,_14,_14,_14];
_10 = _21;
_7.1 = !355634328_u32;
_6 = [_18,_7.0,_7.0,_18,_7.0,_18,_18,_7.0];
_6 = [_18,_18,_18,_18,_7.0,_18,_7.0,_7.0];
_11 = (-1457497161_i32) ^ (-1861915311_i32);
_25 = [_21];
_16 = _14;
_22 = _19 as f64;
_7.3 = !29231_u16;
_13 = 6362178459268833873337321403021772778_u128;
_20 = _7.1 | _7.1;
match _7.2 {
0 => bb7,
1 => bb2,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
340282366920938463463374607431768198156 => bb17,
_ => bb16
}
}
bb12 = {
_3 = -104_i8;
_7 = (6868510990271789419_u64, 1681371005_u32, (-20197_i16), 30738_u16);
_1 = _2;
RET = ['\u{ceb9b}','\u{2d176}','\u{73bce}','\u{3f63e}','\u{c0f56}','\u{385ab}','\u{704d5}','\u{e2f13}'];
_7.2 = -(-31347_i16);
_1 = -_2;
_7.3 = 22362_u16 & 49161_u16;
_7.1 = _7.0 as u32;
_7.2 = 7199_i16 | (-8593_i16);
RET = ['\u{9b479}','\u{e2165}','\u{a2ca0}','\u{d13cd}','\u{cf9e2}','\u{dd7ff}','\u{bafa9}','\u{3cb51}'];
_6 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
_2 = _1;
_1 = -_2;
_7.1 = !237690710_u32;
_9 = [_3,_3,_3,_3];
_1 = _2;
RET = ['\u{6e0fb}','\u{59d74}','\u{6bc0d}','\u{35db7}','\u{425a}','\u{fad9d}','\u{1c615}','\u{f72ba}'];
_7.2 = 8734_i16;
match _7.0 {
6868510990271789419 => bb4,
_ => bb1
}
}
bb13 = {
_8 = [_17,_17];
_7.1 = !243708330_u32;
_17 = 113762564575627223228157252090661836256_i128 >> _7.0;
_14 = '\u{7359f}';
match _7.0 {
0 => bb5,
5618170631436694605 => bb11,
_ => bb10
}
}
bb14 = {
_13 = !47035807421064546286246167840018098971_u128;
_17 = 5067220815246118401_i64 as i128;
_12 = (RET,);
_5 = [_17,_17];
_8 = _5;
_11 = false as i32;
_19 = _7.3 as i64;
Goto(bb9)
}
bb15 = {
_15 = [_3,_3,_3,_3];
_5 = [_17,_17];
_18 = true as u64;
Goto(bb8)
}
bb16 = {
_5 = [5715555995954090530283721047588259780_i128,139001268050091331833182196384525989199_i128];
_5 = [(-92371057397976431368515372188024510980_i128),64645291523837622246288959905886230536_i128];
RET = ['\u{23d43}','\u{7130a}','\u{54047}','\u{f82e}','\u{26636}','\u{7ba9b}','\u{f6d72}','\u{1dc56}'];
_3 = !35_i8;
_3 = (-117_i8);
_3 = 40_i8 | (-44_i8);
_6 = [11669776478533920827_u64,17521628341653755365_u64,14840598150384461519_u64,15794968042119317903_u64,18445745884389915683_u64,17804198651598458840_u64,15524543015623579682_u64,3958385154561172524_u64];
RET = ['\u{294ac}','\u{1257c}','\u{382b0}','\u{cbea3}','\u{a2e8d}','\u{8e249}','\u{6d78}','\u{79ca}'];
RET = ['\u{106063}','\u{61044}','\u{bb1e2}','\u{1d1ec}','\u{d87de}','\u{5acbc}','\u{ba866}','\u{381a1}'];
RET = ['\u{e7e27}','\u{682b6}','\u{f9000}','\u{88744}','\u{c090b}','\u{59e64}','\u{a602d}','\u{e49b3}'];
RET = ['\u{1c67e}','\u{cbdcc}','\u{27c65}','\u{2e403}','\u{816d8}','\u{32f1a}','\u{10c79a}','\u{750}'];
_2 = _1;
_3 = 214113150612329649651918632854298048550_u128 as i8;
RET = ['\u{867f}','\u{6d288}','\u{88735}','\u{49fb3}','\u{d6175}','\u{6faad}','\u{d590}','\u{41e38}'];
_5 = [49503461837656609880304865856205154958_i128,139309017838105571118261291413668150323_i128];
_2 = _1 - _1;
Goto(bb3)
}
bb17 = {
_17 = (-71431599609795461984428405195949210705_i128) >> _7.1;
_29 = _11 as f32;
_7.1 = _20;
Goto(bb18)
}
bb18 = {
Call(_30 = dump_var(2_usize, 14_usize, Move(_14), 21_usize, Move(_21), 17_usize, Move(_17), 10_usize, Move(_10)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_30 = dump_var(2_usize, 25_usize, Move(_25), 6_usize, Move(_6), 18_usize, Move(_18), 5_usize, Move(_5)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_30 = dump_var(2_usize, 8_usize, Move(_8), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [char; 8],mut _2: *const i8) -> i8 {
mir! {
type RET = i8;
let _3: Adt52;
let _4: *mut u8;
let _5: u16;
let _6: isize;
let _7: Adt45;
let _8: Adt51;
let _9: isize;
let _10: f32;
let _11: *const *const ([char; 8],);
let _12: [i8; 4];
let _13: [char; 8];
let _14: char;
let _15: f32;
let _16: (*const *const (u32, [i8; 4], u16, [i8; 4]),);
let _17: [i8; 4];
let _18: u32;
let _19: f32;
let _20: u8;
let _21: (u32, [i8; 4], u16, [i8; 4]);
let _22: isize;
let _23: f32;
let _24: [isize; 1];
let _25: *const *const ([char; 8],);
let _26: bool;
let _27: bool;
let _28: ([char; 8],);
let _29: [i128; 2];
let _30: isize;
let _31: [char; 8];
let _32: (u128,);
let _33: f32;
let _34: (u64, u32, i16, u16);
let _35: ();
let _36: ();
{
RET = 2825921817_u32 as i8;
_2 = core::ptr::addr_of!((*_2));
_1 = ['\u{d84c8}','\u{9800}','\u{1c46d}','\u{a3592}','\u{7b235}','\u{8542a}','\u{cdff8}','\u{847ba}'];
_1 = ['\u{cff9c}','\u{c9c86}','\u{c6d29}','\u{f379a}','\u{18047}','\u{4b3da}','\u{b53be}','\u{3f80e}'];
RET = (*_2);
_2 = core::ptr::addr_of!((*_2));
Goto(bb1)
}
bb1 = {
_1 = ['\u{e0aff}','\u{aefbd}','\u{7d74c}','\u{ddf62}','\u{3ea79}','\u{b42e6}','\u{51cec}','\u{15e75}'];
Goto(bb2)
}
bb2 = {
(*_2) = RET | RET;
RET = (*_2);
_2 = core::ptr::addr_of!(RET);
RET = 54460_u16 as i8;
(*_2) = !(-123_i8);
_5 = 49703_u16;
_5 = (-9223372036854775808_isize) as u16;
RET = (-3_i8) >> _5;
_2 = core::ptr::addr_of!((*_2));
_2 = core::ptr::addr_of!(RET);
_2 = core::ptr::addr_of!(RET);
_2 = core::ptr::addr_of!(RET);
RET = 102_i8;
_1 = ['\u{cdc92}','\u{8dde7}','\u{b62a1}','\u{eee08}','\u{80ba6}','\u{a42a8}','\u{3b5cd}','\u{7770b}'];
_6 = _5 as isize;
_2 = core::ptr::addr_of!(RET);
_1 = ['\u{b482a}','\u{70b34}','\u{3a05f}','\u{3a7ed}','\u{242f}','\u{c765e}','\u{e93b2}','\u{e805a}'];
(*_2) = -(-62_i8);
RET = -(-89_i8);
_6 = '\u{a3f8d}' as isize;
_6 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_6 = (-9223372036854775808_isize);
(*_2) = 57824723559470750110622198994361140379_i128 as i8;
(*_2) = !123_i8;
_2 = core::ptr::addr_of!((*_2));
(*_2) = -(-82_i8);
RET = (-62_i8);
(*_2) = (-48_i8) >> _5;
Goto(bb3)
}
bb3 = {
_6 = !9223372036854775807_isize;
RET = (-33_i8) - (-53_i8);
RET = !0_i8;
_1 = ['\u{1b2d4}','\u{89544}','\u{9b5bc}','\u{4091c}','\u{3f9da}','\u{d7504}','\u{61de}','\u{106401}'];
_1 = ['\u{109946}','\u{98b4f}','\u{e8827}','\u{222c2}','\u{4e8a5}','\u{2eb89}','\u{10048f}','\u{da837}'];
_1 = ['\u{265bd}','\u{e249a}','\u{6b016}','\u{24b40}','\u{4f7ac}','\u{108837}','\u{3d31}','\u{8fd85}'];
RET = 115_i8 | 91_i8;
_2 = core::ptr::addr_of!(RET);
RET = 1968041984_u32 as i8;
_5 = 12051_u16 * 15912_u16;
_9 = !_6;
_5 = 64128_u16;
_5 = !26683_u16;
(*_2) = (-109_i8);
RET = 9403343981511914991_u64 as i8;
_10 = (*_2) as f32;
RET = (-6_i8);
_14 = '\u{838be}';
_6 = _9;
Goto(bb4)
}
bb4 = {
_10 = 29716_i16 as f32;
(*_2) = (-8_i8) - 126_i8;
_1 = [_14,_14,_14,_14,_14,_14,_14,_14];
(*_2) = _6 as i8;
RET = 53691681252477914877714338128856684495_i128 as i8;
_12 = [(*_2),(*_2),RET,(*_2)];
_18 = 925224340_u32 >> _9;
_5 = 22254_u16 * 52466_u16;
(*_2) = (-52_i8) * 14_i8;
Goto(bb5)
}
bb5 = {
_19 = -_10;
_17 = [(*_2),(*_2),RET,(*_2)];
_12 = [RET,(*_2),RET,(*_2)];
_12 = [(*_2),(*_2),RET,RET];
_12 = [(*_2),(*_2),RET,RET];
_17 = [(*_2),(*_2),(*_2),(*_2)];
_9 = _6 ^ _6;
_2 = core::ptr::addr_of!((*_2));
_13 = [_14,_14,_14,_14,_14,_14,_14,_14];
_4 = core::ptr::addr_of_mut!(_20);
_6 = _9 ^ _9;
_10 = _19 - _19;
Goto(bb6)
}
bb6 = {
_21 = (_18, _17, _5, _17);
_5 = _21.2 ^ _21.2;
(*_4) = 129_u8;
Call(RET = core::intrinsics::bswap((-18_i8)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
RET = -120_i8;
Goto(bb8)
}
bb8 = {
_21 = (_18, _17, _5, _17);
_10 = _19 - _19;
_15 = -_19;
_21.2 = _9 as u16;
(*_2) = -(-77_i8);
_13 = [_14,_14,_14,_14,_14,_14,_14,_14];
_24 = [_6];
_23 = _10 + _10;
_20 = 98_u8;
_10 = _23 - _23;
_24 = [_6];
_21.3 = [(*_2),(*_2),(*_2),(*_2)];
(*_2) = _14 as i8;
_10 = (*_4) as f32;
_21.2 = !_5;
RET = _20 as i8;
_21.3 = _21.1;
_9 = _6 + _6;
_5 = _21.2;
_24 = [_6];
Call(_4 = fn4(_24, _12, (*_4), _24, (*_4), _21.1, _23, _21.1, _23, _17, _9), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_6 = !_9;
_1 = [_14,_14,_14,_14,_14,_14,_14,_14];
_2 = core::ptr::addr_of!(RET);
_24 = [_9];
_21.1 = _21.3;
_29 = [(-166753536913103636785016717120384256721_i128),89857722772196485030151695345878584287_i128];
_27 = !false;
(*_2) = (-61_i8) | 124_i8;
_14 = '\u{63cfe}';
_21.1 = [RET,(*_2),(*_2),(*_2)];
_21.0 = _18 & _18;
_21 = (_18, _12, _5, _17);
_17 = _21.3;
_26 = _27;
(*_2) = !10_i8;
_26 = _23 == _23;
_19 = -_23;
_18 = !_21.0;
_5 = _21.2 >> _6;
_28.0 = [_14,_14,_14,_14,_14,_14,_14,_14];
_15 = _19 - _23;
_17 = _12;
match _20 {
98 => bb11,
_ => bb10
}
}
bb10 = {
_1 = ['\u{e0aff}','\u{aefbd}','\u{7d74c}','\u{ddf62}','\u{3ea79}','\u{b42e6}','\u{51cec}','\u{15e75}'];
Goto(bb2)
}
bb11 = {
RET = !15_i8;
RET = !47_i8;
_32.0 = _15 as u128;
Call(RET = core::intrinsics::bswap(50_i8), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_4 = core::ptr::addr_of_mut!(_20);
_9 = -_6;
match _20 {
0 => bb6,
1 => bb13,
2 => bb14,
3 => bb15,
98 => bb17,
_ => bb16
}
}
bb13 = {
(*_2) = RET | RET;
RET = (*_2);
_2 = core::ptr::addr_of!(RET);
RET = 54460_u16 as i8;
(*_2) = !(-123_i8);
_5 = 49703_u16;
_5 = (-9223372036854775808_isize) as u16;
RET = (-3_i8) >> _5;
_2 = core::ptr::addr_of!((*_2));
_2 = core::ptr::addr_of!(RET);
_2 = core::ptr::addr_of!(RET);
_2 = core::ptr::addr_of!(RET);
RET = 102_i8;
_1 = ['\u{cdc92}','\u{8dde7}','\u{b62a1}','\u{eee08}','\u{80ba6}','\u{a42a8}','\u{3b5cd}','\u{7770b}'];
_6 = _5 as isize;
_2 = core::ptr::addr_of!(RET);
_1 = ['\u{b482a}','\u{70b34}','\u{3a05f}','\u{3a7ed}','\u{242f}','\u{c765e}','\u{e93b2}','\u{e805a}'];
(*_2) = -(-62_i8);
RET = -(-89_i8);
_6 = '\u{a3f8d}' as isize;
_6 = 9223372036854775807_isize ^ 9223372036854775807_isize;
_6 = (-9223372036854775808_isize);
(*_2) = 57824723559470750110622198994361140379_i128 as i8;
(*_2) = !123_i8;
_2 = core::ptr::addr_of!((*_2));
(*_2) = -(-82_i8);
RET = (-62_i8);
(*_2) = (-48_i8) >> _5;
Goto(bb3)
}
bb14 = {
_6 = !9223372036854775807_isize;
RET = (-33_i8) - (-53_i8);
RET = !0_i8;
_1 = ['\u{1b2d4}','\u{89544}','\u{9b5bc}','\u{4091c}','\u{3f9da}','\u{d7504}','\u{61de}','\u{106401}'];
_1 = ['\u{109946}','\u{98b4f}','\u{e8827}','\u{222c2}','\u{4e8a5}','\u{2eb89}','\u{10048f}','\u{da837}'];
_1 = ['\u{265bd}','\u{e249a}','\u{6b016}','\u{24b40}','\u{4f7ac}','\u{108837}','\u{3d31}','\u{8fd85}'];
RET = 115_i8 | 91_i8;
_2 = core::ptr::addr_of!(RET);
RET = 1968041984_u32 as i8;
_5 = 12051_u16 * 15912_u16;
_9 = !_6;
_5 = 64128_u16;
_5 = !26683_u16;
(*_2) = (-109_i8);
RET = 9403343981511914991_u64 as i8;
_10 = (*_2) as f32;
RET = (-6_i8);
_14 = '\u{838be}';
_6 = _9;
Goto(bb4)
}
bb15 = {
_19 = -_10;
_17 = [(*_2),(*_2),RET,(*_2)];
_12 = [RET,(*_2),RET,(*_2)];
_12 = [(*_2),(*_2),RET,RET];
_12 = [(*_2),(*_2),RET,RET];
_17 = [(*_2),(*_2),(*_2),(*_2)];
_9 = _6 ^ _6;
_2 = core::ptr::addr_of!((*_2));
_13 = [_14,_14,_14,_14,_14,_14,_14,_14];
_4 = core::ptr::addr_of_mut!(_20);
_6 = _9 ^ _9;
_10 = _19 - _19;
Goto(bb6)
}
bb16 = {
_21 = (_18, _17, _5, _17);
_10 = _19 - _19;
_15 = -_19;
_21.2 = _9 as u16;
(*_2) = -(-77_i8);
_13 = [_14,_14,_14,_14,_14,_14,_14,_14];
_24 = [_6];
_23 = _10 + _10;
_20 = 98_u8;
_10 = _23 - _23;
_24 = [_6];
_21.3 = [(*_2),(*_2),(*_2),(*_2)];
(*_2) = _14 as i8;
_10 = (*_4) as f32;
_21.2 = !_5;
RET = _20 as i8;
_21.3 = _21.1;
_9 = _6 + _6;
_5 = _21.2;
_24 = [_6];
Call(_4 = fn4(_24, _12, (*_4), _24, (*_4), _21.1, _23, _21.1, _23, _17, _9), ReturnTo(bb9), UnwindUnreachable())
}
bb17 = {
_30 = 7203026594273410493_i64 as isize;
_29 = [(-144726679637169746444192882081827305329_i128),(-129987642568817253529885862899222193753_i128)];
_21.2 = !_5;
_27 = _26;
_31 = [_14,_14,_14,_14,_14,_14,_14,_14];
_21.0 = _21.2 as u32;
_21.3 = _21.1;
_1 = _13;
_18 = _14 as u32;
_22 = _6;
_28 = (_13,);
_21.3 = [(*_2),(*_2),(*_2),(*_2)];
_21.2 = _5;
_34.1 = (*_4) as u32;
_2 = core::ptr::addr_of!(RET);
_28.0 = [_14,_14,_14,_14,_14,_14,_14,_14];
_31 = [_14,_14,_14,_14,_14,_14,_14,_14];
Goto(bb18)
}
bb18 = {
Call(_35 = dump_var(3_usize, 26_usize, Move(_26), 18_usize, Move(_18), 14_usize, Move(_14), 21_usize, Move(_21)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_35 = dump_var(3_usize, 22_usize, Move(_22), 24_usize, Move(_24), 13_usize, Move(_13), 12_usize, Move(_12)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_35 = dump_var(3_usize, 9_usize, Move(_9), 29_usize, Move(_29), 36_usize, _36, 36_usize, _36), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: [isize; 1],mut _2: [i8; 4],mut _3: u8,mut _4: [isize; 1],mut _5: u8,mut _6: [i8; 4],mut _7: f32,mut _8: [i8; 4],mut _9: f32,mut _10: [i8; 4],mut _11: isize) -> *mut u8 {
mir! {
type RET = *mut u8;
let _12: Adt50;
let _13: isize;
let _14: i32;
let _15: isize;
let _16: usize;
let _17: [isize; 1];
let _18: (u128,);
let _19: Adt53;
let _20: bool;
let _21: Adt52;
let _22: u128;
let _23: (*const u8,);
let _24: (u128,);
let _25: f64;
let _26: [i8; 8];
let _27: u8;
let _28: (u64, u32, i16, u16);
let _29: isize;
let _30: char;
let _31: Adt56;
let _32: ();
let _33: ();
{
RET = core::ptr::addr_of_mut!(_3);
_12.fld4.1 = RET;
_11 = 9223372036854775807_isize | 9223372036854775807_isize;
_10 = [82_i8,(-37_i8),15_i8,53_i8];
_1 = [_11];
_10 = _8;
_12.fld4.0 = 138900480960478582594541589670047836856_u128 + 149166961167651289434020925303125540819_u128;
RET = core::ptr::addr_of_mut!(_5);
_4 = _1;
(*RET) = !_3;
(*RET) = _3 << _12.fld4.0;
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
98 => bb5,
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
_13 = (-84354049696758168333761759292352613649_i128) as isize;
_7 = _9 + _9;
_12.fld5 = (*RET) as i32;
(*RET) = _12.fld5 as u8;
_12.fld4.1 = RET;
_12.fld4.0 = 242186174778508923376088913386042469557_u128 | 80464622548068894795632564800725677744_u128;
_5 = !_3;
_9 = -_7;
_12.fld4 = (127578518833190962680055059922972803429_u128, RET);
_12.fld4.0 = 414464710924473745_usize as u128;
_4 = _1;
_11 = -_13;
_12.fld4 = (193242744062606283124802610846408240452_u128, RET);
(*RET) = _3 << _12.fld5;
_12.fld4 = (299801755406150228578450403626817389299_u128, RET);
_12.fld4.0 = 66708931825836746575646240795303740610_u128;
_20 = !false;
_16 = 14279855927953784371_usize;
Call(_13 = fn5(_6, _16, (*RET), _11, _5), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_19 = Adt53::Variant1 { fld0: _20,fld1: _13 };
match _3 {
0 => bb1,
1 => bb4,
98 => bb7,
_ => bb5
}
}
bb7 = {
_12.fld4 = (322559381498374884192816612672368808836_u128, RET);
_6 = [51_i8,(-41_i8),55_i8,19_i8];
place!(Field::<bool>(Variant(_19, 1), 0)) = _20;
_7 = _9 - _9;
_2 = [111_i8,126_i8,(-112_i8),26_i8];
_8 = _6;
_16 = !15684831680683857460_usize;
Goto(bb8)
}
bb8 = {
_18 = (_12.fld4.0,);
_16 = 0_usize;
_6 = _8;
_12.fld4.0 = _18.0;
_14 = !_12.fld5;
_4 = [_13];
_11 = _18.0 as isize;
_18.0 = _12.fld4.0 / _12.fld4.0;
_6[_16] = _10[_16];
_20 = _4[_16] >= Field::<isize>(Variant(_19, 1), 1);
place!(Field::<bool>(Variant(_19, 1), 0)) = _20 > _20;
(*RET) = !_3;
_17 = [_4[_16]];
SetDiscriminant(_19, 0);
_3 = _5 * (*RET);
(*RET) = _3;
_13 = 26393_u16 as isize;
place!(Field::<char>(Variant(_19, 0), 1)) = '\u{6d36c}';
_19 = Adt53::Variant1 { fld0: _20,fld1: _13 };
(*RET) = !_3;
_12.fld4.0 = !_18.0;
_10[_16] = _8[_16];
Goto(bb9)
}
bb9 = {
(*RET) = _3;
_18 = (_12.fld4.0,);
_20 = Field::<bool>(Variant(_19, 1), 0);
Call(_1 = core::intrinsics::transmute(_16), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_5 = !_3;
_14 = _12.fld5 << _12.fld5;
_12.fld4.0 = _18.0 >> _10[_16];
_25 = _7 as f64;
_1[_16] = _17[_16];
_22 = _12.fld4.0;
SetDiscriminant(_19, 0);
_6 = _8;
place!(Field::<(u64, u32, i16, u16)>(Variant(_19, 0), 5)).2 = !22865_i16;
RET = _12.fld4.1;
_2 = [_6[_16],_6[_16],_6[_16],_6[_16]];
_22 = _12.fld4.0;
_24.0 = _12.fld4.0 - _22;
place!(Field::<char>(Variant(_19, 0), 1)) = '\u{903e3}';
_1[_16] = 6694_u16 as isize;
_2[_16] = -_8[_16];
_6[_16] = _25 as i8;
_12.fld2 = Adt49::Variant1 { fld0: 5158035805945052593_u64,fld1: RET,fld2: (-4345140894955376625_i64),fld3: _24 };
place!(Field::<u64>(Variant(_12.fld2, 1), 0)) = 2648903283724049428_u64;
match _8[_16] {
0 => bb11,
1 => bb12,
2 => bb13,
51 => bb15,
_ => bb14
}
}
bb11 = {
Return()
}
bb12 = {
_18 = (_12.fld4.0,);
_16 = 0_usize;
_6 = _8;
_12.fld4.0 = _18.0;
_14 = !_12.fld5;
_4 = [_13];
_11 = _18.0 as isize;
_18.0 = _12.fld4.0 / _12.fld4.0;
_6[_16] = _10[_16];
_20 = _4[_16] >= Field::<isize>(Variant(_19, 1), 1);
place!(Field::<bool>(Variant(_19, 1), 0)) = _20 > _20;
(*RET) = !_3;
_17 = [_4[_16]];
SetDiscriminant(_19, 0);
_3 = _5 * (*RET);
(*RET) = _3;
_13 = 26393_u16 as isize;
place!(Field::<char>(Variant(_19, 0), 1)) = '\u{6d36c}';
_19 = Adt53::Variant1 { fld0: _20,fld1: _13 };
(*RET) = !_3;
_12.fld4.0 = !_18.0;
_10[_16] = _8[_16];
Goto(bb9)
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_12.fld3 = core::ptr::addr_of_mut!(place!(Field::<(u64, u32, i16, u16)>(Variant(_19, 0), 5)).3);
Goto(bb16)
}
bb16 = {
Call(_32 = dump_var(4_usize, 20_usize, Move(_20), 11_usize, Move(_11), 3_usize, Move(_3), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(4_usize, 22_usize, Move(_22), 4_usize, Move(_4), 8_usize, Move(_8), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [i8; 4],mut _2: usize,mut _3: u8,mut _4: isize,mut _5: u8) -> isize {
mir! {
type RET = isize;
let _6: *const *const (u32, [i8; 4], u16, [i8; 4]);
let _7: char;
let _8: bool;
let _9: [char; 8];
let _10: isize;
let _11: [i128; 2];
let _12: u64;
let _13: (*const *const (u32, [i8; 4], u16, [i8; 4]),);
let _14: [u128; 8];
let _15: char;
let _16: ();
let _17: ();
{
RET = _4;
RET = _4;
_2 = 12428818846637512404_usize;
_1 = [89_i8,(-92_i8),(-109_i8),7_i8];
RET = 213577542096151168370654233446245264964_u128 as isize;
_5 = _3 >> _3;
_2 = 4727825537369501864_usize + 6_usize;
_5 = _3 ^ _3;
_3 = _5;
_3 = _5 << _5;
_5 = _3 + _3;
RET = _4;
_2 = 7657523796217376020_usize >> _5;
_4 = 2482076974122090295_i64 as isize;
_4 = RET ^ RET;
RET = (-12498_i16) as isize;
RET = !_4;
_8 = true;
_4 = 2960368350399601530_u64 as isize;
_3 = 4336_i16 as u8;
_2 = _3 as usize;
_5 = _3;
_9 = ['\u{fb65c}','\u{25787}','\u{af8b1}','\u{3b667}','\u{26a98}','\u{10a749}','\u{fee01}','\u{a957e}'];
_3 = 936715974_i32 as u8;
_7 = '\u{70d5a}';
_4 = !RET;
_10 = !RET;
Call(RET = core::intrinsics::transmute(_10), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _5;
RET = !_10;
_4 = -_10;
_1 = [(-28_i8),81_i8,10_i8,92_i8];
_9 = [_7,_7,_7,_7,_7,_7,_7,_7];
_3 = !_5;
_1 = [(-121_i8),122_i8,(-85_i8),37_i8];
_10 = RET - RET;
_3 = _5 >> RET;
_8 = false;
RET = _10;
RET = _4;
_3 = (-939649961_i32) as u8;
_5 = !_3;
RET = _10 - _10;
_9 = [_7,_7,_7,_7,_7,_7,_7,_7];
_8 = true ^ true;
_8 = !true;
Call(_5 = fn6(_10, _10, _3, _4, RET, RET, RET, _9, RET, RET, RET, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = !_5;
_10 = RET;
_2 = 2_usize;
_9[_2] = _7;
_10 = RET & RET;
_9[_2] = _7;
_4 = RET ^ RET;
_12 = !14313465241850769028_u64;
_9 = [_7,_7,_7,_7,_7,_7,_7,_7];
_9 = [_7,_7,_7,_7,_7,_7,_7,_7];
RET = _2 as isize;
RET = _8 as isize;
_3 = _5 + _5;
_1 = [80_i8,(-88_i8),68_i8,71_i8];
_3 = 17517_i16 as u8;
match _1[_2] {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
68 => bb9,
_ => bb8
}
}
bb3 = {
_3 = _5;
RET = !_10;
_4 = -_10;
_1 = [(-28_i8),81_i8,10_i8,92_i8];
_9 = [_7,_7,_7,_7,_7,_7,_7,_7];
_3 = !_5;
_1 = [(-121_i8),122_i8,(-85_i8),37_i8];
_10 = RET - RET;
_3 = _5 >> RET;
_8 = false;
RET = _10;
RET = _4;
_3 = (-939649961_i32) as u8;
_5 = !_3;
RET = _10 - _10;
_9 = [_7,_7,_7,_7,_7,_7,_7,_7];
_8 = true ^ true;
_8 = !true;
Call(_5 = fn6(_10, _10, _3, _4, RET, RET, RET, _9, RET, RET, RET, _3), ReturnTo(bb2), UnwindUnreachable())
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
_8 = !true;
_3 = _12 as u8;
_10 = !_4;
_3 = _5;
_11 = [124458792026669269519540793086120413761_i128,(-152198750926816210733896058060698814970_i128)];
_10 = 3753177473_u32 as isize;
_10 = -_4;
_12 = 9955961737901759582_u64;
Goto(bb10)
}
bb10 = {
_7 = _9[_2];
RET = _10 - _4;
RET = _12 as isize;
_9[_2] = _7;
RET = -_4;
_14 = [144224203390096522976660686320209945348_u128,141809264812274178501786003870714123715_u128,71221764201075210414737824901867279434_u128,309467426495120753177862599738257069314_u128,43626016558785475685733795893412322440_u128,17735639896203716529848064791081982775_u128,176056381833514416599815940601700264107_u128,32936406501230357320195987348341493643_u128];
_7 = _9[_2];
_14 = [71217786705163820573794140480671298445_u128,307277420001417600721783447055841721981_u128,26572957009101101748113372050437966844_u128,179925396957110536572619057674265227085_u128,263535140269765177717287924263949339558_u128,203680293500831542337802361160962033458_u128,132593665490533086097007486591719609796_u128,173284339418249753605765471629930108979_u128];
Goto(bb11)
}
bb11 = {
Call(_16 = dump_var(5_usize, 2_usize, Move(_2), 10_usize, Move(_10), 8_usize, Move(_8), 7_usize, Move(_7)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_16 = dump_var(5_usize, 1_usize, Move(_1), 5_usize, Move(_5), 17_usize, _17, 17_usize, _17), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: isize,mut _2: isize,mut _3: u8,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: [char; 8],mut _9: isize,mut _10: isize,mut _11: isize,mut _12: u8) -> u8 {
mir! {
type RET = u8;
let _13: i64;
let _14: isize;
let _15: f64;
let _16: *const i128;
let _17: Adt42;
let _18: [isize; 1];
let _19: ([char; 8],);
let _20: [char; 8];
let _21: isize;
let _22: isize;
let _23: isize;
let _24: u64;
let _25: isize;
let _26: (u128,);
let _27: bool;
let _28: f32;
let _29: (u32, [i8; 4], u16, [i8; 4]);
let _30: [isize; 1];
let _31: f32;
let _32: f64;
let _33: *mut u16;
let _34: i16;
let _35: [i128; 2];
let _36: f64;
let _37: Adt49;
let _38: [i8; 4];
let _39: Adt45;
let _40: Adt48;
let _41: ([char; 8],);
let _42: isize;
let _43: i64;
let _44: f64;
let _45: ();
let _46: ();
{
RET = _3 ^ _3;
_6 = -_11;
Goto(bb1)
}
bb1 = {
_7 = _5 + _6;
_4 = _2;
_13 = (-4980715715256231785_i64);
RET = !_12;
_14 = _7;
Call(_10 = core::intrinsics::bswap(_14), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = '\u{7c5c0}' as isize;
_5 = _14;
_2 = !_7;
_3 = !_12;
_2 = _9 >> _7;
_8 = ['\u{cc620}','\u{f4482}','\u{fe5a0}','\u{90767}','\u{453c5}','\u{8263b}','\u{a6916}','\u{1c2e6}'];
_7 = _6 & _14;
_9 = _10 * _7;
_7 = 60131_u16 as isize;
_8 = ['\u{cf53e}','\u{eb6f6}','\u{9aeb9}','\u{a3d11}','\u{84ab3}','\u{8d516}','\u{232e8}','\u{1dc0b}'];
_13 = (-8577129044476250491_i64) - (-398338626306335126_i64);
_7 = _2;
_14 = !_9;
_14 = _4;
_1 = _5 - _7;
_9 = 63554_u16 as isize;
_3 = 116410394493164717086423724781635382911_i128 as u8;
RET = !_12;
_10 = !_7;
_14 = '\u{6e969}' as isize;
_7 = _2 | _2;
_3 = _12;
_7 = 2049060149_i32 as isize;
_11 = _1 ^ _10;
_5 = !_9;
_8 = ['\u{10c573}','\u{1015c6}','\u{2d278}','\u{d5f5a}','\u{b87a4}','\u{10d2f6}','\u{92ef2}','\u{d9a1f}'];
_6 = -_11;
_15 = _6 as f64;
_14 = -_6;
Goto(bb3)
}
bb3 = {
_17 = Adt42::Variant3 { fld0: true,fld1: 52801_u16,fld2: 18033712173756753328_u64,fld3: 141469286638693382869426611910991396685_i128,fld4: 11749_i16 };
_5 = _10 + _1;
_5 = _6 >> _14;
_10 = !_11;
_8 = ['\u{d41b1}','\u{47249}','\u{24a0a}','\u{b1fde}','\u{16fd9}','\u{27b53}','\u{d84cf}','\u{3c85f}'];
_6 = -_10;
place!(Field::<bool>(Variant(_17, 3), 0)) = true;
_16 = core::ptr::addr_of!(place!(Field::<i128>(Variant(_17, 3), 3)));
Goto(bb4)
}
bb4 = {
_4 = _1 * _6;
_12 = RET;
Goto(bb5)
}
bb5 = {
_6 = -_4;
_12 = 3199576206_u32 as u8;
_19.0 = ['\u{4a40c}','\u{10d437}','\u{c074e}','\u{efa13}','\u{ee9dc}','\u{4c537}','\u{e6161}','\u{10a436}'];
place!(Field::<i16>(Variant(_17, 3), 4)) = !30673_i16;
(*_16) = _11 as i128;
place!(Field::<u64>(Variant(_17, 3), 2)) = 7403644393272766641_u64;
_15 = 13299965652924517933_usize as f64;
_19 = (_8,);
_8 = ['\u{6828a}','\u{5d99f}','\u{73dc8}','\u{c13f3}','\u{52b69}','\u{c03fb}','\u{cd9dd}','\u{54a9c}'];
_19.0 = ['\u{6c8df}','\u{cfe21}','\u{e8fc5}','\u{4c10f}','\u{44d39}','\u{7d1e2}','\u{3bcc8}','\u{3a0ee}'];
_11 = _12 as isize;
_12 = !_3;
_6 = !_14;
place!(Field::<bool>(Variant(_17, 3), 0)) = !false;
place!(Field::<u64>(Variant(_17, 3), 2)) = _14 as u64;
place!(Field::<u16>(Variant(_17, 3), 1)) = RET as u16;
_23 = Field::<u16>(Variant(_17, 3), 1) as isize;
SetDiscriminant(_17, 1);
_10 = _1 * _1;
_8 = ['\u{144a2}','\u{54cf}','\u{99fa8}','\u{3244f}','\u{10cc18}','\u{43224}','\u{141ff}','\u{4c6e7}'];
_21 = 258592599782915660220644029074424450430_u128 as isize;
Goto(bb6)
}
bb6 = {
_6 = -_5;
_7 = 724643207_u32 as isize;
_18 = [_6];
RET = _3;
_14 = _6 + _5;
_15 = 0_usize as f64;
_22 = true as isize;
_22 = _14 - _10;
place!(Field::<i64>(Variant(_17, 1), 2)) = _13;
_15 = 28660_i16 as f64;
_15 = 28980_u16 as f64;
_11 = -_5;
Call(_20 = fn7(_14, _22, _14, _18, _1, _22, _2, _22, _5, _22, _10, _6), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_4 = _11;
_25 = '\u{92d4d}' as isize;
_2 = _6 ^ _22;
_26 = (185080193636264675754848586282653100375_u128,);
_24 = !16811241986607883360_u64;
_32 = -_15;
_2 = _5 & _14;
match _26.0 {
0 => bb2,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
185080193636264675754848586282653100375 => bb13,
_ => bb12
}
}
bb8 = {
_6 = -_5;
_7 = 724643207_u32 as isize;
_18 = [_6];
RET = _3;
_14 = _6 + _5;
_15 = 0_usize as f64;
_22 = true as isize;
_22 = _14 - _10;
place!(Field::<i64>(Variant(_17, 1), 2)) = _13;
_15 = 28660_i16 as f64;
_15 = 28980_u16 as f64;
_11 = -_5;
Call(_20 = fn7(_14, _22, _14, _18, _1, _22, _2, _22, _5, _22, _10, _6), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
_6 = -_4;
_12 = 3199576206_u32 as u8;
_19.0 = ['\u{4a40c}','\u{10d437}','\u{c074e}','\u{efa13}','\u{ee9dc}','\u{4c537}','\u{e6161}','\u{10a436}'];
place!(Field::<i16>(Variant(_17, 3), 4)) = !30673_i16;
(*_16) = _11 as i128;
place!(Field::<u64>(Variant(_17, 3), 2)) = 7403644393272766641_u64;
_15 = 13299965652924517933_usize as f64;
_19 = (_8,);
_8 = ['\u{6828a}','\u{5d99f}','\u{73dc8}','\u{c13f3}','\u{52b69}','\u{c03fb}','\u{cd9dd}','\u{54a9c}'];
_19.0 = ['\u{6c8df}','\u{cfe21}','\u{e8fc5}','\u{4c10f}','\u{44d39}','\u{7d1e2}','\u{3bcc8}','\u{3a0ee}'];
_11 = _12 as isize;
_12 = !_3;
_6 = !_14;
place!(Field::<bool>(Variant(_17, 3), 0)) = !false;
place!(Field::<u64>(Variant(_17, 3), 2)) = _14 as u64;
place!(Field::<u16>(Variant(_17, 3), 1)) = RET as u16;
_23 = Field::<u16>(Variant(_17, 3), 1) as isize;
SetDiscriminant(_17, 1);
_10 = _1 * _1;
_8 = ['\u{144a2}','\u{54cf}','\u{99fa8}','\u{3244f}','\u{10cc18}','\u{43224}','\u{141ff}','\u{4c6e7}'];
_21 = 258592599782915660220644029074424450430_u128 as isize;
Goto(bb6)
}
bb10 = {
_4 = _1 * _6;
_12 = RET;
Goto(bb5)
}
bb11 = {
_7 = _5 + _6;
_4 = _2;
_13 = (-4980715715256231785_i64);
RET = !_12;
_14 = _7;
Call(_10 = core::intrinsics::bswap(_14), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_5 = '\u{7c5c0}' as isize;
_5 = _14;
_2 = !_7;
_3 = !_12;
_2 = _9 >> _7;
_8 = ['\u{cc620}','\u{f4482}','\u{fe5a0}','\u{90767}','\u{453c5}','\u{8263b}','\u{a6916}','\u{1c2e6}'];
_7 = _6 & _14;
_9 = _10 * _7;
_7 = 60131_u16 as isize;
_8 = ['\u{cf53e}','\u{eb6f6}','\u{9aeb9}','\u{a3d11}','\u{84ab3}','\u{8d516}','\u{232e8}','\u{1dc0b}'];
_13 = (-8577129044476250491_i64) - (-398338626306335126_i64);
_7 = _2;
_14 = !_9;
_14 = _4;
_1 = _5 - _7;
_9 = 63554_u16 as isize;
_3 = 116410394493164717086423724781635382911_i128 as u8;
RET = !_12;
_10 = !_7;
_14 = '\u{6e969}' as isize;
_7 = _2 | _2;
_3 = _12;
_7 = 2049060149_i32 as isize;
_11 = _1 ^ _10;
_5 = !_9;
_8 = ['\u{10c573}','\u{1015c6}','\u{2d278}','\u{d5f5a}','\u{b87a4}','\u{10d2f6}','\u{92ef2}','\u{d9a1f}'];
_6 = -_11;
_15 = _6 as f64;
_14 = -_6;
Goto(bb3)
}
bb13 = {
_26 = (104170462265490041104071160221988569269_u128,);
_29.0 = 4054542664_u32 | 3069668101_u32;
place!(Field::<*const ([char; 8],)>(Variant(_17, 1), 3)) = core::ptr::addr_of!(_19);
place!(Field::<f32>(Variant(_17, 1), 0)) = 906184349_i32 as f32;
_29.3 = [126_i8,(-18_i8),121_i8,105_i8];
_28 = Field::<f32>(Variant(_17, 1), 0);
_34 = _26.0 as i16;
_31 = _15 as f32;
_33 = core::ptr::addr_of_mut!(_29.2);
_15 = -_32;
_2 = _6 + _4;
RET = !_3;
_27 = _14 >= _11;
(*_33) = !25546_u16;
_35 = [(-5616663279163440082486719885006041986_i128),34899567849513540144807746707768096003_i128];
place!(Field::<*const ([char; 8],)>(Variant(_17, 1), 3)) = core::ptr::addr_of!(place!(Field::<([char; 8],)>(Variant(_17, 1), 1)));
Call(_12 = core::intrinsics::transmute(_27), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
RET = !_12;
_19.0 = _20;
_8 = _20;
_32 = -_15;
_30 = _18;
place!(Field::<i16>(Variant(_17, 1), 4)) = '\u{c77a1}' as i16;
RET = _12;
(*_33) = 37905_u16 + 36257_u16;
_41 = (_19.0,);
_28 = -_31;
place!(Field::<([char; 8],)>(Variant(_17, 1), 1)) = _19;
_36 = _32 - _32;
_41 = (_20,);
_9 = _22 * _1;
_31 = _28;
_34 = RET as i16;
(*_33) = 12095_u16 >> _5;
_42 = _4 >> _2;
place!(Field::<f32>(Variant(_17, 1), 0)) = (-87_i8) as f32;
_2 = _22;
_11 = _5;
_44 = 4_usize as f64;
_7 = _1 >> _12;
RET = !_12;
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(6_usize, 9_usize, Move(_9), 7_usize, Move(_7), 20_usize, Move(_20), 30_usize, Move(_30)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(6_usize, 6_usize, Move(_6), 35_usize, Move(_35), 23_usize, Move(_23), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(6_usize, 11_usize, Move(_11), 8_usize, Move(_8), 5_usize, Move(_5), 24_usize, Move(_24)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(6_usize, 42_usize, Move(_42), 3_usize, Move(_3), 46_usize, _46, 46_usize, _46), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: [isize; 1],mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize) -> [char; 8] {
mir! {
type RET = [char; 8];
let _13: isize;
let _14: isize;
let _15: *const *const (u32, [i8; 4], u16, [i8; 4]);
let _16: u16;
let _17: [i8; 8];
let _18: Adt46;
let _19: i16;
let _20: *const u8;
let _21: *mut bool;
let _22: ([char; 8],);
let _23: bool;
let _24: f64;
let _25: u64;
let _26: isize;
let _27: f64;
let _28: [i8; 8];
let _29: ([char; 8],);
let _30: (u128,);
let _31: Adt58;
let _32: (*const *const (u32, [i8; 4], u16, [i8; 4]),);
let _33: [i8; 4];
let _34: char;
let _35: bool;
let _36: i8;
let _37: isize;
let _38: [u128; 8];
let _39: char;
let _40: [u128; 8];
let _41: [isize; 1];
let _42: [u128; 8];
let _43: *mut u8;
let _44: [isize; 1];
let _45: ([char; 8],);
let _46: f64;
let _47: Adt58;
let _48: Adt54;
let _49: *const *const (u32, [i8; 4], u16, [i8; 4]);
let _50: isize;
let _51: [i8; 4];
let _52: char;
let _53: ();
let _54: ();
{
_7 = _6;
RET = ['\u{bfbbe}','\u{7c8b8}','\u{75e46}','\u{1011ea}','\u{f6898}','\u{e7ba8}','\u{b0f36}','\u{2d648}'];
_13 = (-3526_i16) as isize;
_4 = [_3];
RET = ['\u{100852}','\u{d49ed}','\u{6f5e}','\u{5db17}','\u{5d27d}','\u{265d7}','\u{a0a70}','\u{e0611}'];
_4 = [_5];
_3 = -_2;
_3 = _1;
_11 = _3;
_8 = _1;
_2 = _11;
_9 = false as isize;
_5 = !_8;
_7 = _5 ^ _12;
RET = ['\u{e6717}','\u{8c44d}','\u{f3081}','\u{90e71}','\u{d71f6}','\u{2cab7}','\u{538bc}','\u{fa3f0}'];
_18.fld1.fld5 = 8368727243255751796540389314621274235_u128 as i32;
_16 = 6884_u16 | 9183_u16;
_18.fld1.fld1 = '\u{7982a}';
RET = [_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1];
Call(_18.fld1.fld0 = core::intrinsics::bswap(2955976953_u32), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = _1;
_18.fld1.fld1 = '\u{efbcc}';
_18.fld1.fld1 = '\u{109a7a}';
_12 = (-4271271696547124623_i64) as isize;
_18.fld0 = [(-117_i8),(-71_i8),14_i8,39_i8];
_9 = _6 * _1;
_18.fld1.fld2 = _18.fld0;
_8 = _18.fld1.fld5 as isize;
_6 = _3;
_18.fld3 = 47075982733379003396041643569679680522_u128 & 281824064989711928003052912975521255452_u128;
_18.fld1.fld6 = _18.fld1.fld5 as f64;
RET = [_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1];
_1 = -_3;
_18.fld1.fld0 = (-36_i8) as u32;
_1 = 58253609697749936841495155936800683686_i128 as isize;
_18.fld1.fld1 = '\u{10a93b}';
_18.fld0 = [39_i8,2_i8,(-3_i8),(-49_i8)];
_4 = [_6];
_10 = (-5397646033559526053_i64) as isize;
_7 = -_5;
_2 = _11 ^ _3;
_18.fld1.fld4 = [(-78515518443166825866205522617163462434_i128),150087206917918430876147669931404168706_i128];
_18.fld1.fld1 = '\u{10fdd9}';
_24 = _18.fld3 as f64;
Goto(bb2)
}
bb2 = {
_14 = _9;
_10 = (-152228321916634700455655401650513329782_i128) as isize;
_18.fld1.fld5 = 678160794_i32;
_13 = _11;
_22 = (RET,);
_23 = !false;
_1 = _18.fld1.fld6 as isize;
_18.fld1.fld5 = _13 as i32;
_27 = (-9620_i16) as f64;
RET = [_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1];
_17 = [65_i8,(-56_i8),(-51_i8),(-31_i8),(-124_i8),(-126_i8),(-40_i8),(-105_i8)];
_18.fld2 = core::ptr::addr_of_mut!(_16);
_11 = _18.fld1.fld6 as isize;
_13 = _5;
_2 = _14;
_8 = !_2;
_18.fld1.fld0 = 1_usize as u32;
_9 = 9103317475557376546_i64 as isize;
_18.fld1.fld1 = '\u{41194}';
_9 = _14;
Goto(bb3)
}
bb3 = {
_3 = _18.fld3 as isize;
_19 = 5989125501985982592_i64 as i16;
_18.fld1.fld6 = 109805003913545018375215923225983643892_i128 as f64;
_21 = core::ptr::addr_of_mut!(_23);
_2 = -_9;
_17 = [(-46_i8),(-126_i8),(-120_i8),15_i8,123_i8,(-32_i8),125_i8,(-95_i8)];
_25 = !16501933604133059851_u64;
_13 = _8;
_27 = _18.fld1.fld5 as f64;
_5 = _7 + _9;
_21 = core::ptr::addr_of_mut!((*_21));
_28 = [25_i8,(-60_i8),89_i8,(-23_i8),4_i8,59_i8,23_i8,(-95_i8)];
_29 = (_22.0,);
_18.fld0 = [(-30_i8),(-56_i8),(-29_i8),(-91_i8)];
_27 = _24;
_18.fld1.fld0 = 2533008854_u32 & 647580018_u32;
_5 = _8;
_5 = -_13;
_28 = [(-105_i8),(-34_i8),(-13_i8),4_i8,(-5_i8),(-122_i8),(-53_i8),79_i8];
_21 = core::ptr::addr_of_mut!((*_21));
Goto(bb4)
}
bb4 = {
_6 = _9;
_17 = [(-14_i8),20_i8,122_i8,(-38_i8),(-24_i8),109_i8,10_i8,(-90_i8)];
_14 = -_8;
_1 = _8 + _13;
_18.fld0 = _18.fld1.fld2;
_18.fld2 = core::ptr::addr_of_mut!(_16);
_18.fld1.fld2 = [51_i8,125_i8,19_i8,23_i8];
_18.fld1.fld1 = '\u{1e4f5}';
_1 = _6;
_9 = _1;
_22.0 = [_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1];
RET = [_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1];
_4 = [_8];
_5 = -_6;
_14 = _1 + _13;
_17 = [(-24_i8),89_i8,(-124_i8),103_i8,(-113_i8),(-77_i8),(-25_i8),30_i8];
Goto(bb5)
}
bb5 = {
_28 = _17;
(*_21) = _18.fld1.fld5 > _18.fld1.fld5;
_18.fld1.fld4 = [71878597074539748737361737456492487200_i128,(-26819802170169848240338589225471252109_i128)];
_8 = -_9;
_13 = _27 as isize;
(*_21) = !true;
RET = [_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1];
_30.0 = !_18.fld3;
_18.fld3 = !_30.0;
_17 = [(-86_i8),(-69_i8),(-76_i8),121_i8,(-15_i8),114_i8,20_i8,57_i8];
_22 = (RET,);
_18.fld1.fld6 = -_27;
_18.fld1.fld4 = [(-57388030364247417497261059123228050304_i128),(-74048346938109775876428127901910596063_i128)];
_5 = _1;
_18.fld1.fld5 = _18.fld3 as i32;
_14 = _16 as isize;
_18.fld1.fld0 = 63_i8 as u32;
Call(_33 = fn8(_4, _2, _6, _1, _28, _5, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_10 = _9;
Goto(bb7)
}
bb7 = {
_18.fld1.fld6 = _30.0 as f64;
_18.fld2 = core::ptr::addr_of_mut!(_16);
_18.fld1.fld0 = 3898095564_u32 * 3509537383_u32;
Goto(bb8)
}
bb8 = {
_11 = _1 << _6;
_1 = _5 & _7;
_18.fld1.fld2 = [86_i8,56_i8,60_i8,44_i8];
_22.0 = [_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1];
_5 = _18.fld1.fld5 as isize;
_18.fld3 = _30.0;
_21 = core::ptr::addr_of_mut!((*_21));
_29 = _22;
Goto(bb9)
}
bb9 = {
_26 = !_8;
_18.fld0 = _18.fld1.fld2;
_18.fld0 = [19_i8,65_i8,(-87_i8),53_i8];
_35 = _1 == _1;
Goto(bb10)
}
bb10 = {
_10 = _1;
_29.0 = [_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1];
_22 = _29;
_19 = 101_u8 as i16;
_11 = _7 ^ _6;
_39 = _18.fld1.fld1;
Goto(bb11)
}
bb11 = {
_14 = _8;
(*_21) = _35;
Call(_18.fld1.fld4 = core::intrinsics::transmute(RET), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_12 = -_26;
Goto(bb13)
}
bb13 = {
_42 = [_18.fld3,_18.fld3,_30.0,_18.fld3,_30.0,_18.fld3,_18.fld3,_30.0];
_13 = _19 as isize;
_22 = _29;
_27 = _25 as f64;
_21 = core::ptr::addr_of_mut!((*_21));
_35 = _23 | (*_21);
_45.0 = [_39,_39,_18.fld1.fld1,_18.fld1.fld1,_39,_18.fld1.fld1,_18.fld1.fld1,_18.fld1.fld1];
_36 = 11_i8;
_6 = _2;
(*_21) = _1 <= _26;
_8 = -_2;
_5 = _36 as isize;
_21 = core::ptr::addr_of_mut!(_35);
_36 = (*_21) as i8;
_3 = _7 | _6;
Goto(bb14)
}
bb14 = {
_18.fld1.fld6 = _18.fld1.fld5 as f64;
_9 = _19 as isize;
_25 = _36 as u64;
_46 = _30.0 as f64;
_33 = [_36,_36,_36,_36];
_18.fld1.fld2 = [_36,_36,_36,_36];
_45.0 = [_39,_39,_39,_39,_39,_18.fld1.fld1,_39,_18.fld1.fld1];
_11 = _6 - _10;
_18.fld2 = core::ptr::addr_of_mut!(_16);
_33 = [_36,_36,_36,_36];
_23 = !(*_21);
_28 = [_36,_36,_36,_36,_36,_36,_36,_36];
_38 = [_30.0,_30.0,_30.0,_18.fld3,_30.0,_18.fld3,_18.fld3,_18.fld3];
_52 = _18.fld1.fld1;
_4 = [_10];
_45 = _29;
RET = [_18.fld1.fld1,_52,_39,_52,_39,_18.fld1.fld1,_18.fld1.fld1,_39];
_8 = _26 * _14;
_46 = _18.fld1.fld6;
_30.0 = _18.fld3;
_50 = _25 as isize;
_42 = [_30.0,_18.fld3,_30.0,_30.0,_18.fld3,_30.0,_30.0,_18.fld3];
_45 = (_29.0,);
Goto(bb15)
}
bb15 = {
Call(_53 = dump_var(7_usize, 23_usize, Move(_23), 6_usize, Move(_6), 33_usize, Move(_33), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_53 = dump_var(7_usize, 26_usize, Move(_26), 30_usize, Move(_30), 1_usize, Move(_1), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_53 = dump_var(7_usize, 10_usize, Move(_10), 19_usize, Move(_19), 4_usize, Move(_4), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_53 = dump_var(7_usize, 2_usize, Move(_2), 52_usize, Move(_52), 28_usize, Move(_28), 50_usize, Move(_50)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: [isize; 1],mut _2: isize,mut _3: isize,mut _4: isize,mut _5: [i8; 8],mut _6: isize,mut _7: isize) -> [i8; 4] {
mir! {
type RET = [i8; 4];
let _8: Adt42;
let _9: Adt46;
let _10: [i128; 2];
let _11: [i8; 4];
let _12: f32;
let _13: i8;
let _14: [u64; 8];
let _15: (u128, *mut u8);
let _16: f32;
let _17: usize;
let _18: char;
let _19: i16;
let _20: isize;
let _21: [i8; 8];
let _22: (*const *const (u32, [i8; 4], u16, [i8; 4]),);
let _23: Adt51;
let _24: i64;
let _25: (u128,);
let _26: i64;
let _27: bool;
let _28: i16;
let _29: *const ([char; 8],);
let _30: ();
let _31: ();
{
_5 = [(-111_i8),(-57_i8),72_i8,(-71_i8),(-89_i8),28_i8,91_i8,(-112_i8)];
_4 = -_2;
RET = [76_i8,(-119_i8),82_i8,(-25_i8)];
RET = [(-71_i8),(-57_i8),75_i8,105_i8];
_5 = [78_i8,47_i8,(-77_i8),(-128_i8),117_i8,120_i8,(-102_i8),(-18_i8)];
_2 = _3 + _6;
_9.fld1.fld4 = [112106451910391724397272262447962157711_i128,40809409142157679895401640756375282909_i128];
_6 = _7;
_9.fld1.fld0 = 2651670384_u32 >> _3;
_9.fld1.fld5 = (-1502149771_i32);
_9.fld1.fld6 = 721159962762335969_i64 as f64;
_9.fld1.fld5 = !784201805_i32;
_9.fld3 = 248416849796799844978528926108724016153_u128;
_8 = Adt42::Variant3 { fld0: false,fld1: 47820_u16,fld2: 16580207385862337512_u64,fld3: (-60089153799023670026221686020648928932_i128),fld4: (-21974_i16) };
place!(Field::<bool>(Variant(_8, 3), 0)) = _7 >= _6;
_7 = !_2;
_6 = _4;
_1 = [_4];
_7 = !_6;
Call(_9.fld1.fld5 = fn9(_3, _3, Field::<bool>(Variant(_8, 3), 0)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9.fld1.fld2 = [0_i8,84_i8,100_i8,(-30_i8)];
_12 = 72_u8 as f32;
_9.fld1.fld6 = _7 as f64;
_4 = !_3;
_2 = 5_i8 as isize;
_9.fld1.fld2 = RET;
place!(Field::<i16>(Variant(_8, 3), 4)) = (-29074_i16) >> _6;
Call(place!(Field::<i128>(Variant(_8, 3), 3)) = fn10(_4, _3, _7, _4, _6, Field::<bool>(Variant(_8, 3), 0), _7, _3, _7, Field::<bool>(Variant(_8, 3), 0), _4, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9.fld2 = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_8, 3), 1)));
_13 = 30_i8 + 126_i8;
match _9.fld3 {
248416849796799844978528926108724016153 => bb4,
_ => bb3
}
}
bb3 = {
_9.fld1.fld2 = [0_i8,84_i8,100_i8,(-30_i8)];
_12 = 72_u8 as f32;
_9.fld1.fld6 = _7 as f64;
_4 = !_3;
_2 = 5_i8 as isize;
_9.fld1.fld2 = RET;
place!(Field::<i16>(Variant(_8, 3), 4)) = (-29074_i16) >> _6;
Call(place!(Field::<i128>(Variant(_8, 3), 3)) = fn10(_4, _3, _7, _4, _6, Field::<bool>(Variant(_8, 3), 0), _7, _3, _7, Field::<bool>(Variant(_8, 3), 0), _4, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
_6 = Field::<i16>(Variant(_8, 3), 4) as isize;
_9.fld1.fld4 = [Field::<i128>(Variant(_8, 3), 3),Field::<i128>(Variant(_8, 3), 3)];
_13 = 54_i8 - 78_i8;
_9.fld1.fld6 = _9.fld3 as f64;
_14 = [17730946768505351647_u64,9074232928784920701_u64,14475657885046305510_u64,920727804646078951_u64,10661661197233930277_u64,1915851643603836553_u64,2794694884870501169_u64,477622657132889790_u64];
_1 = [_6];
place!(Field::<i128>(Variant(_8, 3), 3)) = (-74537804951555347844082489790160562443_i128) + 135772405886390579911456284848613851195_i128;
_9.fld3 = !265360365202019211534573091279337470912_u128;
_13 = !122_i8;
_7 = _9.fld3 as isize;
_9.fld1.fld5 = -1674580182_i32;
_9.fld1.fld4 = [Field::<i128>(Variant(_8, 3), 3),Field::<i128>(Variant(_8, 3), 3)];
place!(Field::<i16>(Variant(_8, 3), 4)) = 3238505438175125415_usize as i16;
_9.fld1.fld4 = [Field::<i128>(Variant(_8, 3), 3),Field::<i128>(Variant(_8, 3), 3)];
place!(Field::<u64>(Variant(_8, 3), 2)) = 16795416414749660023_u64;
_1 = [_4];
place!(Field::<bool>(Variant(_8, 3), 0)) = !true;
_9.fld1.fld6 = 54626_u16 as f64;
_9.fld1.fld2 = RET;
_9.fld1.fld4 = [Field::<i128>(Variant(_8, 3), 3),Field::<i128>(Variant(_8, 3), 3)];
_1 = [_6];
Goto(bb5)
}
bb5 = {
_3 = !_4;
_15.0 = _6 as u128;
_11 = [_13,_13,_13,_13];
_21 = [_13,_13,_13,_13,_13,_13,_13,_13];
_15.0 = !_9.fld3;
RET = [_13,_13,_13,_13];
_9.fld1.fld5 = (-648203759_i32);
_13 = !76_i8;
Goto(bb6)
}
bb6 = {
_9.fld3 = _15.0 ^ _15.0;
_19 = _9.fld1.fld6 as i16;
_9.fld1.fld1 = '\u{4d82}';
_9.fld3 = !_15.0;
_2 = 21550_u16 as isize;
_21 = _5;
_12 = 5_usize as f32;
_5 = _21;
Goto(bb7)
}
bb7 = {
_9.fld1.fld0 = !2025355104_u32;
_18 = _9.fld1.fld1;
place!(Field::<u64>(Variant(_8, 3), 2)) = 12640940434896535406_u64;
place!(Field::<u64>(Variant(_8, 3), 2)) = 11703706983653064717_u64;
_2 = _6;
RET = [_13,_13,_13,_13];
_11 = [_13,_13,_13,_13];
place!(Field::<i16>(Variant(_8, 3), 4)) = Field::<bool>(Variant(_8, 3), 0) as i16;
place!(Field::<i128>(Variant(_8, 3), 3)) = (-162053569368253812273786269821591489522_i128);
_24 = (-1498235232263216240_i64);
_9.fld1.fld6 = _9.fld1.fld5 as f64;
_10 = _9.fld1.fld4;
_4 = _6;
_9.fld1.fld6 = Field::<i128>(Variant(_8, 3), 3) as f64;
_16 = -_12;
_25.0 = 10858632168099947021_usize as u128;
_7 = _9.fld1.fld6 as isize;
place!(Field::<u16>(Variant(_8, 3), 1)) = Field::<i16>(Variant(_8, 3), 4) as u16;
_9.fld1.fld5 = 1270630212_i32 >> _4;
place!(Field::<i128>(Variant(_8, 3), 3)) = -56744771055120157423827504238950647473_i128;
_18 = _9.fld1.fld1;
_17 = _9.fld3 as usize;
_9.fld1.fld6 = Field::<u64>(Variant(_8, 3), 2) as f64;
place!(Field::<i16>(Variant(_8, 3), 4)) = _13 as i16;
_15.0 = _25.0 & _25.0;
Goto(bb8)
}
bb8 = {
_24 = 9170723290744598062_i64;
SetDiscriminant(_8, 0);
_9.fld1.fld4 = [(-47297275439389235288788273600491354414_i128),143248366466305591011761933628786651279_i128];
_9.fld1.fld1 = _18;
_3 = 53_u8 as isize;
_15.0 = false as u128;
_14 = [11982175946084376660_u64,7900190086132846876_u64,432740176280638470_u64,15812150325054318915_u64,7424900477135203827_u64,669911624981276749_u64,2553413060647293613_u64,6042951270646556725_u64];
place!(Field::<u16>(Variant(_8, 0), 3)) = 26016781048700769426966556379202128249_i128 as u16;
Goto(bb9)
}
bb9 = {
_25 = (_9.fld3,);
_13 = -(-13_i8);
_9.fld1.fld0 = !4021470054_u32;
_11 = [_13,_13,_13,_13];
_9.fld0 = _9.fld1.fld2;
match _24 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
9170723290744598062 => bb11,
_ => bb10
}
}
bb10 = {
_9.fld1.fld2 = [0_i8,84_i8,100_i8,(-30_i8)];
_12 = 72_u8 as f32;
_9.fld1.fld6 = _7 as f64;
_4 = !_3;
_2 = 5_i8 as isize;
_9.fld1.fld2 = RET;
place!(Field::<i16>(Variant(_8, 3), 4)) = (-29074_i16) >> _6;
Call(place!(Field::<i128>(Variant(_8, 3), 3)) = fn10(_4, _3, _7, _4, _6, Field::<bool>(Variant(_8, 3), 0), _7, _3, _7, Field::<bool>(Variant(_8, 3), 0), _4, _7), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_1 = [_4];
place!(Field::<*mut u16>(Variant(_8, 0), 2)) = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_8, 0), 3)));
_6 = _2 * _4;
Goto(bb12)
}
bb12 = {
_14 = [9244620092148047709_u64,6723458453155378360_u64,13404181759077784165_u64,14391227127972561217_u64,15363911411966844262_u64,937201539798890495_u64,3648503013630619226_u64,1434350911134630788_u64];
_14 = [13890740200406695858_u64,7914758585087527340_u64,1635770281826700820_u64,14211213913462598881_u64,3932599645796676036_u64,4478326589121140414_u64,6944676230616614463_u64,4203158858175338592_u64];
_20 = -_3;
_16 = _12 - _12;
_19 = -(-8515_i16);
_9.fld1.fld4 = [(-39350046006782188702792661619507297152_i128),(-5847527204391857741541542098882683823_i128)];
match _24 {
0 => bb6,
1 => bb13,
2 => bb14,
9170723290744598062 => bb16,
_ => bb15
}
}
bb13 = {
_1 = [_4];
place!(Field::<*mut u16>(Variant(_8, 0), 2)) = core::ptr::addr_of_mut!(place!(Field::<u16>(Variant(_8, 0), 3)));
_6 = _2 * _4;
Goto(bb12)
}
bb14 = {
_9.fld3 = _15.0 ^ _15.0;
_19 = _9.fld1.fld6 as i16;
_9.fld1.fld1 = '\u{4d82}';
_9.fld3 = !_15.0;
_2 = 21550_u16 as isize;
_21 = _5;
_12 = 5_usize as f32;
_5 = _21;
Goto(bb7)
}
bb15 = {
_3 = !_4;
_15.0 = _6 as u128;
_11 = [_13,_13,_13,_13];
_21 = [_13,_13,_13,_13,_13,_13,_13,_13];
_15.0 = !_9.fld3;
RET = [_13,_13,_13,_13];
_9.fld1.fld5 = (-648203759_i32);
_13 = !76_i8;
Goto(bb6)
}
bb16 = {
_25.0 = _15.0 ^ _9.fld3;
_15.0 = !_25.0;
_5 = _21;
_9.fld1.fld2 = [_13,_13,_13,_13];
_9.fld1.fld2 = [_13,_13,_13,_13];
_9.fld1.fld1 = _18;
_9.fld1.fld2 = [_13,_13,_13,_13];
_9.fld1.fld6 = _12 as f64;
Goto(bb17)
}
bb17 = {
Call(_30 = dump_var(8_usize, 7_usize, Move(_7), 5_usize, Move(_5), 25_usize, Move(_25), 24_usize, Move(_24)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_30 = dump_var(8_usize, 1_usize, Move(_1), 19_usize, Move(_19), 3_usize, Move(_3), 21_usize, Move(_21)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_30 = dump_var(8_usize, 17_usize, Move(_17), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: isize,mut _2: isize,mut _3: bool) -> i32 {
mir! {
type RET = i32;
let _4: [u64; 8];
let _5: [isize; 1];
let _6: [u64; 8];
let _7: (u128,);
let _8: i64;
let _9: [u128; 8];
let _10: [i128; 2];
let _11: f64;
let _12: isize;
let _13: bool;
let _14: i32;
let _15: f32;
let _16: *const i128;
let _17: [i128; 2];
let _18: ();
let _19: ();
{
_1 = -_2;
RET = -(-506844281_i32);
_3 = false;
RET = (-9142503995204814210_i64) as i32;
RET = (-2065806653_i32) & 180453669_i32;
_4 = [15767275788655398582_u64,11668948443995262565_u64,13445750888911726946_u64,2547909381216254053_u64,2887309929435998903_u64,8549748223253265426_u64,17426576462969393856_u64,356772782113523544_u64];
_1 = _2 * _2;
_2 = _1 ^ _1;
_2 = _1;
_2 = _1 - _1;
_4 = [15515462490759583494_u64,4557655029110756933_u64,11818501000112843930_u64,2148255727244398530_u64,12119084825471288926_u64,5058544518980125013_u64,3042377961336318443_u64,10370018099299125944_u64];
_2 = _1;
_4 = [1536277150853786201_u64,13470859241950765020_u64,12185249966738848581_u64,10846454234154006266_u64,11982702746290264696_u64,11447372677831997972_u64,1863410213831044011_u64,18002566135114131108_u64];
RET = 1363596634_i32;
_6 = [6178436914513997738_u64,7702701147557250171_u64,1645119483663635922_u64,3954686969448994605_u64,6956502830833098821_u64,1792900539933726377_u64,8820012244859946594_u64,571254015979744762_u64];
_4 = [16094481315906964444_u64,2717002203416509585_u64,1673156896874395025_u64,2257882274336317515_u64,8704750125513895751_u64,16007527185353618594_u64,10277899643973822998_u64,12511705457475382563_u64];
_5 = [_1];
_6 = [4891083979621623897_u64,963878001650761725_u64,17527839916539561294_u64,17042074556120905360_u64,13793979775345631723_u64,17086084099344843565_u64,7742469391426123113_u64,9365526260864870394_u64];
_6 = [274051776943428017_u64,17199481939289117888_u64,12247087129288472830_u64,15674275168750380318_u64,8011415847698196136_u64,5092235866525761216_u64,9032169940242206498_u64,5691323005229499015_u64];
RET = (-104_i8) as i32;
_6 = _4;
_2 = _1 * _1;
_7 = (29974259728547599831131570012352990728_u128,);
Goto(bb1)
}
bb1 = {
_6 = _4;
_6 = _4;
RET = -(-220232202_i32);
_1 = 7_usize as isize;
_1 = (-1263753319997206391_i64) as isize;
_6 = [18259471312970565269_u64,3873567118219214634_u64,1547642362756325009_u64,8438059167977440579_u64,4896337618545166895_u64,11151288084483661753_u64,5549781311394021132_u64,5794717242810931862_u64];
_6 = [6527160860553844496_u64,8162657132268221050_u64,787237494359973726_u64,3900214650862849105_u64,2264451242743468206_u64,13089939601914353514_u64,2199839665093382422_u64,11859471146457561432_u64];
_3 = !true;
RET = -917718772_i32;
Goto(bb2)
}
bb2 = {
_8 = !(-1834299010219935574_i64);
_4 = [5411442315509387560_u64,7603209263391431157_u64,37300781735579242_u64,12801706419696737840_u64,4800865143486119751_u64,3279459950913813559_u64,3982671958959930325_u64,7305492929815694991_u64];
_7.0 = 131733200592730132132009873267262401355_u128 + 1373134065577090023630225869959716232_u128;
_11 = 93867917993129347344627532054277694758_i128 as f64;
_10 = [10986333262842638094042905213088644819_i128,83835329291579914619353663446519104243_i128];
_7 = (175871784000753428522053727376708679971_u128,);
RET = -(-1171685963_i32);
match _7.0 {
0 => bb1,
1 => bb3,
175871784000753428522053727376708679971 => bb5,
_ => bb4
}
}
bb3 = {
_6 = _4;
_6 = _4;
RET = -(-220232202_i32);
_1 = 7_usize as isize;
_1 = (-1263753319997206391_i64) as isize;
_6 = [18259471312970565269_u64,3873567118219214634_u64,1547642362756325009_u64,8438059167977440579_u64,4896337618545166895_u64,11151288084483661753_u64,5549781311394021132_u64,5794717242810931862_u64];
_6 = [6527160860553844496_u64,8162657132268221050_u64,787237494359973726_u64,3900214650862849105_u64,2264451242743468206_u64,13089939601914353514_u64,2199839665093382422_u64,11859471146457561432_u64];
_3 = !true;
RET = -917718772_i32;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
_11 = _7.0 as f64;
_6 = [7877602883611613220_u64,15358628299768575729_u64,15177812140164148582_u64,2162962050772153455_u64,3607337222039155315_u64,15985174025415602814_u64,17622228609550416837_u64,12255979514322013016_u64];
_1 = 47_u8 as isize;
Goto(bb6)
}
bb6 = {
RET = 29220111_i32;
_12 = _2 >> _2;
_14 = 35617_u16 as i32;
match _7.0 {
0 => bb7,
175871784000753428522053727376708679971 => bb9,
_ => bb8
}
}
bb7 = {
_6 = _4;
_6 = _4;
RET = -(-220232202_i32);
_1 = 7_usize as isize;
_1 = (-1263753319997206391_i64) as isize;
_6 = [18259471312970565269_u64,3873567118219214634_u64,1547642362756325009_u64,8438059167977440579_u64,4896337618545166895_u64,11151288084483661753_u64,5549781311394021132_u64,5794717242810931862_u64];
_6 = [6527160860553844496_u64,8162657132268221050_u64,787237494359973726_u64,3900214650862849105_u64,2264451242743468206_u64,13089939601914353514_u64,2199839665093382422_u64,11859471146457561432_u64];
_3 = !true;
RET = -917718772_i32;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_5 = [_12];
_9 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
_17 = [(-129575389628906354313726387375357940278_i128),11415186614716038900767671006781710993_i128];
_8 = (-3325653181213808398_i64);
_13 = !_3;
RET = _14 >> _2;
_15 = 20286_u16 as f32;
_9 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
_5 = [_12];
_11 = 2_usize as f64;
_7 = (131338762887643834308122436290116361847_u128,);
_2 = _12 ^ _12;
_13 = !_3;
_13 = _2 != _2;
_1 = !_2;
_14 = !RET;
_3 = RET <= _14;
Goto(bb10)
}
bb10 = {
Call(_18 = dump_var(9_usize, 3_usize, Move(_3), 1_usize, Move(_1), 4_usize, Move(_4), 17_usize, Move(_17)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_18 = dump_var(9_usize, 12_usize, Move(_12), 10_usize, Move(_10), 6_usize, Move(_6), 19_usize, _19), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: bool,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: bool,mut _11: isize,mut _12: isize) -> i128 {
mir! {
type RET = i128;
let _13: isize;
let _14: [i8; 8];
let _15: *const u8;
let _16: [i8; 8];
let _17: [i8; 8];
let _18: Adt45;
let _19: (u32, [i8; 4], u16, [i8; 4]);
let _20: Adt53;
let _21: f64;
let _22: i128;
let _23: Adt43;
let _24: f32;
let _25: char;
let _26: [i8; 4];
let _27: i128;
let _28: Adt50;
let _29: [i8; 8];
let _30: u128;
let _31: isize;
let _32: f32;
let _33: char;
let _34: [i8; 4];
let _35: char;
let _36: usize;
let _37: [u128; 8];
let _38: isize;
let _39: isize;
let _40: [i8; 8];
let _41: [u64; 8];
let _42: char;
let _43: (u128,);
let _44: u128;
let _45: (u32, [i8; 4], u16, [i8; 4]);
let _46: ([char; 8],);
let _47: f64;
let _48: isize;
let _49: [isize; 1];
let _50: &'static i128;
let _51: f64;
let _52: char;
let _53: u8;
let _54: (u32, [i8; 4], u16, [i8; 4]);
let _55: [u128; 8];
let _56: f64;
let _57: ();
let _58: ();
{
RET = !(-66796684121321595688108524405189770980_i128);
_6 = _5 <= _3;
RET = (-144908976996243842850612287582381783136_i128) << _4;
_1 = !_11;
_6 = _10;
_2 = _1 | _8;
_7 = !_11;
RET = (-153920290460091852883803031210486152340_i128);
_1 = -_11;
_14 = [88_i8,(-110_i8),(-32_i8),43_i8,46_i8,99_i8,43_i8,(-45_i8)];
_6 = _10;
RET = (-21555234385502038130377852005705220008_i128) & 49598117258610346507209954489007226122_i128;
_5 = _2;
_13 = (-573752334_i32) as isize;
Goto(bb1)
}
bb1 = {
_13 = _9;
_8 = '\u{c6a2e}' as isize;
_9 = _2;
_3 = _1 ^ _7;
_3 = _10 as isize;
_8 = -_11;
_16 = [73_i8,(-78_i8),(-71_i8),65_i8,(-55_i8),23_i8,34_i8,28_i8];
_13 = !_4;
_2 = _12 >> _1;
_14 = [(-68_i8),112_i8,(-75_i8),44_i8,94_i8,10_i8,(-43_i8),83_i8];
_8 = -_11;
_19.0 = 5323794622489776902_i64 as u32;
_6 = !_10;
_13 = -_1;
_16 = [(-33_i8),(-42_i8),34_i8,(-91_i8),9_i8,82_i8,(-68_i8),46_i8];
_9 = _7;
_19.3 = [(-68_i8),111_i8,(-63_i8),88_i8];
_7 = !_13;
_7 = _10 as isize;
_19.1 = [73_i8,(-59_i8),107_i8,(-103_i8)];
RET = '\u{2d61e}' as i128;
_10 = _3 == _3;
_14 = _16;
Goto(bb2)
}
bb2 = {
_20 = Adt53::Variant1 { fld0: _6,fld1: _4 };
_21 = 5010468684746117598_u64 as f64;
_7 = _4;
_14 = [72_i8,37_i8,45_i8,(-19_i8),(-8_i8),35_i8,(-40_i8),(-12_i8)];
_22 = RET;
_20 = Adt53::Variant2 { fld0: RET,fld1: _14 };
_12 = _13;
_4 = -_8;
place!(Field::<i128>(Variant(_20, 2), 0)) = _22;
_11 = _1;
_9 = _1;
_19.3 = [(-57_i8),79_i8,(-37_i8),9_i8];
_19.0 = 48128266100253540517959577278948676304_u128 as u32;
_22 = -RET;
_17 = [101_i8,14_i8,39_i8,(-120_i8),(-119_i8),(-43_i8),20_i8,38_i8];
_27 = _22 ^ Field::<i128>(Variant(_20, 2), 0);
SetDiscriminant(_20, 1);
_28.fld5 = 2073449988_i32 * (-1864686151_i32);
_24 = (-25125_i16) as f32;
_11 = _7;
_14 = _16;
Goto(bb3)
}
bb3 = {
_19.2 = 45121_u16 & 49713_u16;
_9 = _1;
Goto(bb4)
}
bb4 = {
_1 = 179702211279900854666928564598535290299_u128 as isize;
_8 = !_3;
_7 = _12 * _8;
_16 = _14;
_11 = !_8;
_13 = !_3;
_20 = Adt53::Variant2 { fld0: _27,fld1: _16 };
_28.fld4.0 = 156730054759324606033210795005380987043_u128 << _8;
_25 = '\u{b36c5}';
_5 = _10 as isize;
_8 = _3;
_17 = [94_i8,111_i8,121_i8,(-44_i8),(-85_i8),72_i8,28_i8,9_i8];
_28.fld3 = core::ptr::addr_of_mut!(_19.2);
RET = _27;
_7 = -_9;
_9 = _19.2 as isize;
_4 = _13 & _2;
SetDiscriminant(_20, 0);
_29 = [(-88_i8),92_i8,(-88_i8),122_i8,26_i8,(-95_i8),(-110_i8),7_i8];
_2 = _21 as isize;
_26 = _19.1;
_13 = -_5;
_5 = -_3;
place!(Field::<*mut u16>(Variant(_20, 0), 2)) = _28.fld3;
place!(Field::<Adt43>(Variant(_20, 0), 4)) = Adt43::Variant1 { fld0: _27,fld1: _16,fld2: _24 };
Goto(bb5)
}
bb5 = {
place!(Field::<Adt43>(Variant(_20, 0), 4)) = Adt43::Variant1 { fld0: _27,fld1: _29,fld2: _24 };
_20 = Adt53::Variant1 { fld0: _10,fld1: _11 };
_25 = '\u{b9b4d}';
RET = _27 + _27;
_25 = '\u{159f6}';
Goto(bb6)
}
bb6 = {
SetDiscriminant(_20, 0);
_28.fld3 = core::ptr::addr_of_mut!(place!(Field::<(u64, u32, i16, u16)>(Variant(_20, 0), 5)).3);
_19.1 = _26;
place!(Field::<bool>(Variant(_20, 0), 0)) = _10 ^ _10;
RET = _27;
place!(Field::<*mut u16>(Variant(_20, 0), 2)) = _28.fld3;
_26 = _19.1;
_28.fld4.0 = _19.0 as u128;
place!(Field::<(u64, u32, i16, u16)>(Variant(_20, 0), 5)) = (5341524260822286671_u64, _19.0, 4481_i16, _19.2);
place!(Field::<Adt43>(Variant(_20, 0), 4)) = Adt43::Variant1 { fld0: RET,fld1: _17,fld2: _24 };
_33 = _25;
place!(Field::<char>(Variant(_20, 0), 1)) = _33;
_32 = -_24;
place!(Field::<char>(Variant(_20, 0), 1)) = _25;
_34 = [(-24_i8),(-10_i8),(-4_i8),71_i8];
_13 = !_11;
_31 = _8 | _12;
_31 = -_8;
_4 = !_13;
_17 = [(-57_i8),(-87_i8),(-9_i8),43_i8,(-16_i8),(-5_i8),35_i8,107_i8];
_3 = -_13;
_16 = [(-78_i8),(-114_i8),(-64_i8),(-103_i8),(-12_i8),(-27_i8),(-101_i8),(-91_i8)];
RET = _27;
_37 = [_28.fld4.0,_28.fld4.0,_28.fld4.0,_28.fld4.0,_28.fld4.0,_28.fld4.0,_28.fld4.0,_28.fld4.0];
_10 = _12 < _7;
_39 = Field::<(u64, u32, i16, u16)>(Variant(_20, 0), 5).2 as isize;
Goto(bb7)
}
bb7 = {
SetDiscriminant(Field::<Adt43>(Variant(_20, 0), 4), 1);
_19.3 = _26;
_13 = _5;
_11 = _21 as isize;
_31 = !_4;
place!(Field::<[i8; 8]>(Variant(place!(Field::<Adt43>(Variant(_20, 0), 4)), 1), 1)) = _14;
place!(Field::<bool>(Variant(_20, 0), 0)) = _6;
RET = !_22;
_40 = _14;
_24 = -_32;
_13 = Field::<(u64, u32, i16, u16)>(Variant(_20, 0), 5).2 as isize;
Call(_17 = fn11(_4, _6, _8, _3, _8, _10, _5, _4), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_12 = -_4;
_29 = [114_i8,109_i8,(-113_i8),(-79_i8),(-88_i8),5_i8,20_i8,(-19_i8)];
_27 = Field::<bool>(Variant(_20, 0), 0) as i128;
_7 = !_3;
place!(Field::<i128>(Variant(place!(Field::<Adt43>(Variant(_20, 0), 4)), 1), 0)) = _27 & _27;
place!(Field::<(u64, u32, i16, u16)>(Variant(_20, 0), 5)).1 = Field::<(u64, u32, i16, u16)>(Variant(_20, 0), 5).0 as u32;
_11 = _5;
place!(Field::<i128>(Variant(place!(Field::<Adt43>(Variant(_20, 0), 4)), 1), 0)) = _27 << _27;
_3 = _11;
_35 = _25;
_13 = _5;
_6 = _10;
_45.3 = [(-43_i8),67_i8,9_i8,(-79_i8)];
place!(Field::<(u64, u32, i16, u16)>(Variant(_20, 0), 5)).0 = 7651012664843829609_u64;
_41 = [Field::<(u64, u32, i16, u16)>(Variant(_20, 0), 5).0,Field::<(u64, u32, i16, u16)>(Variant(_20, 0), 5).0,Field::<(u64, u32, i16, u16)>(Variant(_20, 0), 5).0,Field::<(u64, u32, i16, u16)>(Variant(_20, 0), 5).0,Field::<(u64, u32, i16, u16)>(Variant(_20, 0), 5).0,Field::<(u64, u32, i16, u16)>(Variant(_20, 0), 5).0,Field::<(u64, u32, i16, u16)>(Variant(_20, 0), 5).0,Field::<(u64, u32, i16, u16)>(Variant(_20, 0), 5).0];
_45.2 = !_19.2;
place!(Field::<*mut u16>(Variant(_20, 0), 2)) = core::ptr::addr_of_mut!(place!(Field::<(u64, u32, i16, u16)>(Variant(_20, 0), 5)).3);
_46.0 = [_33,Field::<char>(Variant(_20, 0), 1),_35,_33,_25,_35,_25,_33];
place!(Field::<f32>(Variant(place!(Field::<Adt43>(Variant(_20, 0), 4)), 1), 2)) = (-978084053585248038_i64) as f32;
_28.fld5 = -1849893444_i32;
_12 = _7 >> _8;
_38 = !_8;
_43.0 = 13827222060576158241_usize as u128;
_3 = _31 ^ _4;
Call(_16 = core::intrinsics::transmute(_38), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_2 = -_38;
_44 = !_43.0;
_21 = Field::<i128>(Variant(Field::<Adt43>(Variant(_20, 0), 4), 1), 0) as f64;
_16 = [70_i8,114_i8,(-125_i8),(-120_i8),4_i8,14_i8,(-34_i8),107_i8];
_20 = Adt53::Variant2 { fld0: _27,fld1: _16 };
_28.fld4.0 = 5365805614064481152_u64 as u128;
_35 = _25;
SetDiscriminant(_20, 2);
_2 = _31;
_42 = _35;
_38 = _4;
_10 = _2 < _31;
place!(Field::<[i8; 8]>(Variant(_20, 2), 1)) = [119_i8,10_i8,82_i8,(-29_i8),115_i8,(-13_i8),(-123_i8),23_i8];
_45.0 = _19.0;
_5 = -_13;
Goto(bb10)
}
bb10 = {
_33 = _42;
_20 = Adt53::Variant1 { fld0: _6,fld1: _8 };
_13 = _3 - _31;
_1 = _2;
_45.0 = !_19.0;
_1 = !_8;
Call(_11 = core::intrinsics::transmute(_31), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_28.fld3 = core::ptr::addr_of_mut!(_19.2);
_40 = [113_i8,(-71_i8),(-53_i8),(-57_i8),52_i8,(-92_i8),(-24_i8),(-57_i8)];
_43 = (_44,);
_36 = !5455818357570925671_usize;
_45 = _19;
_27 = _12 as i128;
_29 = _17;
_30 = _43.0 << _5;
_48 = _27 as isize;
_22 = !_27;
_19.0 = _45.0;
RET = 9712_i16 as i128;
_19 = (_45.0, _45.1, _45.2, _45.3);
_50 = &_27;
_11 = _13 ^ _38;
SetDiscriminant(_20, 1);
Goto(bb12)
}
bb12 = {
_43.0 = 7396389487537646070_i64 as u128;
RET = 49_i8 as i128;
Goto(bb13)
}
bb13 = {
_38 = 149_u8 as isize;
_28.fld5 = (-404761063_i32) - 245276819_i32;
_15 = core::ptr::addr_of!(_53);
_28.fld5 = 629173599_i32 + (-1719414000_i32);
_29 = [(-7_i8),57_i8,44_i8,64_i8,(-44_i8),(-58_i8),(-118_i8),(-116_i8)];
_43.0 = _30;
_19 = _45;
_13 = _8;
place!(Field::<bool>(Variant(_20, 1), 0)) = !_10;
_52 = _33;
_24 = 101_u8 as f32;
_11 = _28.fld5 as isize;
_28.fld4.0 = _2 as u128;
_3 = _4;
RET = (*_50);
_19.3 = [17_i8,45_i8,(-81_i8),(-65_i8)];
_52 = _42;
_3 = _28.fld5 as isize;
_14 = [17_i8,(-36_i8),(-31_i8),18_i8,(-114_i8),(-15_i8),(-47_i8),98_i8];
_12 = _13 & _4;
_16 = [109_i8,19_i8,86_i8,25_i8,83_i8,9_i8,(-75_i8),(-105_i8)];
_9 = _12;
_31 = !_1;
_19.2 = _45.2;
_43.0 = _28.fld4.0 + _30;
_54 = (_19.0, _45.3, _19.2, _19.3);
_11 = _28.fld5 as isize;
_16 = [(-14_i8),101_i8,(-1_i8),(-22_i8),(-37_i8),(-7_i8),(-48_i8),50_i8];
Goto(bb14)
}
bb14 = {
Call(_57 = dump_var(10_usize, 35_usize, Move(_35), 38_usize, Move(_38), 34_usize, Move(_34), 3_usize, Move(_3)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_57 = dump_var(10_usize, 36_usize, Move(_36), 13_usize, Move(_13), 54_usize, Move(_54), 52_usize, Move(_52)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_57 = dump_var(10_usize, 6_usize, Move(_6), 48_usize, Move(_48), 9_usize, Move(_9), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_57 = dump_var(10_usize, 43_usize, Move(_43), 16_usize, Move(_16), 26_usize, Move(_26), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_57 = dump_var(10_usize, 45_usize, Move(_45), 5_usize, Move(_5), 27_usize, Move(_27), 31_usize, Move(_31)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: isize,mut _2: bool,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: bool,mut _7: isize,mut _8: isize) -> [i8; 8] {
mir! {
type RET = [i8; 8];
let _9: [u64; 8];
let _10: u128;
let _11: [u128; 8];
let _12: i32;
let _13: *mut bool;
let _14: u128;
let _15: u16;
let _16: usize;
let _17: Adt42;
let _18: [i128; 2];
let _19: f64;
let _20: (*const *const (u32, [i8; 4], u16, [i8; 4]),);
let _21: Adt57;
let _22: (u128,);
let _23: isize;
let _24: char;
let _25: i64;
let _26: char;
let _27: usize;
let _28: [i8; 8];
let _29: [u128; 8];
let _30: u128;
let _31: [i8; 8];
let _32: isize;
let _33: isize;
let _34: u64;
let _35: [i8; 8];
let _36: (u32, [i8; 4], u16, [i8; 4]);
let _37: f32;
let _38: [i8; 8];
let _39: [i128; 2];
let _40: i32;
let _41: Adt57;
let _42: isize;
let _43: u16;
let _44: [u128; 8];
let _45: i16;
let _46: Adt49;
let _47: [u128; 8];
let _48: ([char; 8],);
let _49: *const u8;
let _50: Adt56;
let _51: ([char; 8],);
let _52: Adt55;
let _53: ();
let _54: ();
{
_1 = _5;
_4 = !_1;
_7 = _1;
RET = [8_i8,8_i8,(-13_i8),51_i8,(-26_i8),(-4_i8),(-115_i8),(-104_i8)];
_7 = !_3;
_5 = _1 * _4;
_1 = _7 * _7;
_10 = !132680525882370929717212388805777832543_u128;
Call(_2 = fn12(_1, _5, _7, _1, _8, _7, _4, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_10 = 307454163929936780224153781840265068091_u128;
Goto(bb2)
}
bb2 = {
_2 = !_6;
_11 = [_10,_10,_10,_10,_10,_10,_10,_10];
_6 = _7 == _1;
_7 = _5;
_9 = [17747522808243737225_u64,8045122393105831456_u64,13509812258377754379_u64,12828843951383346635_u64,11406448110220661165_u64,12518299010120757272_u64,8373795415548132716_u64,3997547309214036526_u64];
_13 = core::ptr::addr_of_mut!(_6);
_2 = !_6;
_6 = !_2;
_5 = (-103_i8) as isize;
_7 = _3 & _1;
RET = [(-87_i8),(-101_i8),(-99_i8),22_i8,(-16_i8),(-18_i8),109_i8,(-15_i8)];
(*_13) = _2;
(*_13) = _1 <= _1;
_15 = 8297_u16 ^ 32531_u16;
_12 = 0_usize as i32;
_10 = !125162298037644399475491279711628095012_u128;
_14 = !_10;
RET = [40_i8,106_i8,(-74_i8),(-64_i8),(-14_i8),38_i8,43_i8,(-95_i8)];
_10 = _14;
_16 = !14945092776073052936_usize;
_4 = (-22_i8) as isize;
_6 = !_2;
_2 = _1 != _1;
Goto(bb3)
}
bb3 = {
_18 = [(-160410130925820202038414389306117875690_i128),7797568014351635593933562778160200908_i128];
_2 = !(*_13);
_10 = _14;
RET = [58_i8,126_i8,(-121_i8),(-75_i8),(-65_i8),(-36_i8),90_i8,(-126_i8)];
RET = [(-100_i8),(-50_i8),(-85_i8),69_i8,64_i8,53_i8,(-54_i8),49_i8];
_9 = [16825481600883493430_u64,5765795095836921259_u64,3493783674545444856_u64,4239364214107579123_u64,4526953014248108559_u64,6585024958188051488_u64,6390835773272875628_u64,5088999224438953256_u64];
_15 = 30761_u16;
_9 = [7596326690518833213_u64,14991916567683581396_u64,12257913859479375008_u64,14468248086577103252_u64,11477173815727073162_u64,10860336255856165352_u64,17666799208998813180_u64,14240448623459428588_u64];
(*_13) = _8 != _1;
_9 = [10949794058357183377_u64,486473999823466115_u64,5945614750815040349_u64,17153926352886659504_u64,13886018031330407002_u64,17054787109140650076_u64,15683230826304126446_u64,11732648462204395102_u64];
RET = [25_i8,84_i8,(-81_i8),(-67_i8),(-109_i8),77_i8,26_i8,61_i8];
_22.0 = _10;
_23 = -_8;
_24 = '\u{215ac}';
_4 = _23;
_5 = _8;
_23 = _2 as isize;
_25 = -(-427199712844808870_i64);
_11 = [_22.0,_14,_10,_22.0,_22.0,_22.0,_22.0,_14];
_22 = (_14,);
_15 = !44500_u16;
Goto(bb4)
}
bb4 = {
_10 = _24 as u128;
_26 = _24;
_19 = _15 as f64;
_13 = core::ptr::addr_of_mut!(_2);
_22 = (_14,);
_4 = _1 >> _3;
_16 = !16931045857896089260_usize;
_22.0 = _6 as u128;
_7 = (-85321860673954931255506857894682984880_i128) as isize;
RET = [70_i8,5_i8,(-43_i8),(-25_i8),(-53_i8),114_i8,102_i8,(-8_i8)];
_27 = _16 + _16;
_11 = [_22.0,_22.0,_22.0,_22.0,_22.0,_22.0,_22.0,_22.0];
_28 = [103_i8,110_i8,57_i8,(-38_i8),(-38_i8),91_i8,(-118_i8),110_i8];
_6 = !_2;
_16 = _27 << _8;
_19 = 3252735403_u32 as f64;
_15 = 6382_u16 * 3189_u16;
_24 = _26;
_30 = _22.0 + _22.0;
(*_13) = _6 != _6;
_8 = _5 << _5;
Call(_4 = core::intrinsics::transmute(_5), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
(*_13) = _5 == _8;
_1 = _23;
_28 = RET;
_23 = !_5;
_28 = RET;
_31 = RET;
_29 = [_22.0,_22.0,_30,_22.0,_22.0,_22.0,_30,_22.0];
(*_13) = _5 != _8;
_35 = _31;
_22.0 = _30 * _30;
_24 = _26;
_30 = _25 as u128;
_23 = _5 << _5;
RET = [59_i8,(-98_i8),100_i8,(-6_i8),(-25_i8),(-58_i8),(-8_i8),57_i8];
_4 = _6 as isize;
_12 = !775490329_i32;
Call((*_13) = fn15(_4, _3, _3, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_26 = _24;
_3 = _19 as isize;
RET = _35;
_8 = _4 << _22.0;
RET = _31;
_22 = (_30,);
_4 = 849087824_u32 as isize;
_11 = [_22.0,_14,_30,_30,_10,_30,_30,_22.0];
_25 = 2537161380690744178_i64 >> _5;
_22 = (_30,);
_11 = [_14,_22.0,_22.0,_10,_30,_10,_22.0,_10];
_34 = 14760068703606833479_u64 + 5504775725864826888_u64;
_3 = _25 as isize;
_1 = !_8;
_8 = !_5;
_4 = _3 - _8;
_22.0 = _14;
_34 = 16835408904183111692_u64 - 9714531916801492111_u64;
_31 = RET;
_22.0 = _34 as u128;
_29 = [_10,_10,_10,_14,_14,_10,_10,_22.0];
_22.0 = (*_13) as u128;
Goto(bb7)
}
bb7 = {
_33 = _8;
_6 = _2;
_3 = _1 - _1;
_22.0 = !_30;
_32 = _3 << _33;
_37 = _12 as f32;
_37 = (-66836110834261680314799037173588224380_i128) as f32;
RET = [(-53_i8),(-35_i8),(-41_i8),(-23_i8),(-101_i8),(-15_i8),(-79_i8),127_i8];
_15 = 25432_u16;
_28 = RET;
_31 = RET;
_34 = (-70332513813046018398376400659662687508_i128) as u64;
_23 = !_1;
_36.0 = 4168341988_u32 - 2470379842_u32;
Call(_35 = fn16(_23, _5, _23, _3, _32, _25, _16), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_25 = 2647931639324160384_i64;
_44 = _29;
_3 = !_32;
_26 = _24;
Goto(bb9)
}
bb9 = {
_42 = -_3;
_36.0 = 2413030401_u32;
_35 = [(-5_i8),54_i8,1_i8,52_i8,91_i8,(-113_i8),6_i8,(-76_i8)];
_36.2 = _15;
_33 = _42 & _5;
_23 = _33 ^ _42;
_9 = [_34,_34,_34,_34,_34,_34,_34,_34];
_32 = _3 << _1;
_38 = [62_i8,(-92_i8),(-55_i8),109_i8,121_i8,(-65_i8),37_i8,72_i8];
Goto(bb10)
}
bb10 = {
(*_13) = _6;
RET = [37_i8,97_i8,(-118_i8),(-15_i8),30_i8,91_i8,74_i8,95_i8];
_36.0 = 2178943860_u32 * 583669795_u32;
_13 = core::ptr::addr_of_mut!((*_13));
_36.2 = _34 as u16;
_4 = _32;
RET = [90_i8,40_i8,4_i8,29_i8,(-2_i8),7_i8,(-72_i8),(-80_i8)];
_38 = [(-111_i8),(-83_i8),77_i8,79_i8,(-95_i8),(-8_i8),30_i8,(-21_i8)];
_33 = _32 >> _16;
_39 = [(-36058206502117458917495509159964152923_i128),(-158445474635481556355302546594387618694_i128)];
_36.3 = [33_i8,45_i8,97_i8,2_i8];
_22 = (_14,);
_29 = [_22.0,_22.0,_10,_22.0,_14,_10,_30,_14];
_47 = [_22.0,_14,_30,_30,_10,_10,_30,_10];
_14 = _36.0 as u128;
_43 = !_15;
_22.0 = _33 as u128;
Goto(bb11)
}
bb11 = {
_5 = _19 as isize;
_16 = _27;
_45 = 5525_i16;
_18 = [(-38971760001680930315256354337172862809_i128),(-119725209842480949454422337340737412355_i128)];
(*_13) = _6;
_35 = [(-17_i8),122_i8,(-111_i8),70_i8,111_i8,(-20_i8),(-69_i8),35_i8];
_37 = (-23_i8) as f32;
_2 = _3 <= _42;
_34 = 7398134308140953049_u64;
(*_13) = !_6;
_3 = _4 + _4;
_1 = _12 as isize;
_40 = _22.0 as i32;
_22 = (_14,);
_11 = _47;
_18 = [95109027803112780762706629811438036878_i128,(-155826052893084734989097077503570936091_i128)];
_2 = _6;
_42 = _32;
_7 = _33 | _33;
_48.0 = [_24,_26,_26,_26,_24,_26,_24,_24];
match _34 {
0 => bb12,
1 => bb13,
2 => bb14,
7398134308140953049 => bb16,
_ => bb15
}
}
bb12 = {
_10 = _24 as u128;
_26 = _24;
_19 = _15 as f64;
_13 = core::ptr::addr_of_mut!(_2);
_22 = (_14,);
_4 = _1 >> _3;
_16 = !16931045857896089260_usize;
_22.0 = _6 as u128;
_7 = (-85321860673954931255506857894682984880_i128) as isize;
RET = [70_i8,5_i8,(-43_i8),(-25_i8),(-53_i8),114_i8,102_i8,(-8_i8)];
_27 = _16 + _16;
_11 = [_22.0,_22.0,_22.0,_22.0,_22.0,_22.0,_22.0,_22.0];
_28 = [103_i8,110_i8,57_i8,(-38_i8),(-38_i8),91_i8,(-118_i8),110_i8];
_6 = !_2;
_16 = _27 << _8;
_19 = 3252735403_u32 as f64;
_15 = 6382_u16 * 3189_u16;
_24 = _26;
_30 = _22.0 + _22.0;
(*_13) = _6 != _6;
_8 = _5 << _5;
Call(_4 = core::intrinsics::transmute(_5), ReturnTo(bb5), UnwindUnreachable())
}
bb13 = {
_26 = _24;
_3 = _19 as isize;
RET = _35;
_8 = _4 << _22.0;
RET = _31;
_22 = (_30,);
_4 = 849087824_u32 as isize;
_11 = [_22.0,_14,_30,_30,_10,_30,_30,_22.0];
_25 = 2537161380690744178_i64 >> _5;
_22 = (_30,);
_11 = [_14,_22.0,_22.0,_10,_30,_10,_22.0,_10];
_34 = 14760068703606833479_u64 + 5504775725864826888_u64;
_3 = _25 as isize;
_1 = !_8;
_8 = !_5;
_4 = _3 - _8;
_22.0 = _14;
_34 = 16835408904183111692_u64 - 9714531916801492111_u64;
_31 = RET;
_22.0 = _34 as u128;
_29 = [_10,_10,_10,_14,_14,_10,_10,_22.0];
_22.0 = (*_13) as u128;
Goto(bb7)
}
bb14 = {
(*_13) = _5 == _8;
_1 = _23;
_28 = RET;
_23 = !_5;
_28 = RET;
_31 = RET;
_29 = [_22.0,_22.0,_30,_22.0,_22.0,_22.0,_30,_22.0];
(*_13) = _5 != _8;
_35 = _31;
_22.0 = _30 * _30;
_24 = _26;
_30 = _25 as u128;
_23 = _5 << _5;
RET = [59_i8,(-98_i8),100_i8,(-6_i8),(-25_i8),(-58_i8),(-8_i8),57_i8];
_4 = _6 as isize;
_12 = !775490329_i32;
Call((*_13) = fn15(_4, _3, _3, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb15 = {
_33 = _8;
_6 = _2;
_3 = _1 - _1;
_22.0 = !_30;
_32 = _3 << _33;
_37 = _12 as f32;
_37 = (-66836110834261680314799037173588224380_i128) as f32;
RET = [(-53_i8),(-35_i8),(-41_i8),(-23_i8),(-101_i8),(-15_i8),(-79_i8),127_i8];
_15 = 25432_u16;
_28 = RET;
_31 = RET;
_34 = (-70332513813046018398376400659662687508_i128) as u64;
_23 = !_1;
_36.0 = 4168341988_u32 - 2470379842_u32;
Call(_35 = fn16(_23, _5, _23, _3, _32, _25, _16), ReturnTo(bb8), UnwindUnreachable())
}
bb16 = {
_47 = [_22.0,_14,_14,_14,_22.0,_30,_30,_14];
_18 = [(-164311956270687573586794616915990106266_i128),153978297532165016222455487706256500797_i128];
_51 = (_48.0,);
_15 = !_36.2;
Goto(bb17)
}
bb17 = {
Call(_53 = dump_var(11_usize, 30_usize, Move(_30), 45_usize, Move(_45), 23_usize, Move(_23), 35_usize, Move(_35)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_53 = dump_var(11_usize, 6_usize, Move(_6), 2_usize, Move(_2), 8_usize, Move(_8), 31_usize, Move(_31)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_53 = dump_var(11_usize, 25_usize, Move(_25), 47_usize, Move(_47), 33_usize, Move(_33), 40_usize, Move(_40)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_53 = dump_var(11_usize, 51_usize, Move(_51), 26_usize, Move(_26), 29_usize, Move(_29), 12_usize, Move(_12)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_53 = dump_var(11_usize, 39_usize, Move(_39), 42_usize, Move(_42), 3_usize, Move(_3), 34_usize, Move(_34)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize) -> bool {
mir! {
type RET = bool;
let _9: [isize; 1];
let _10: i128;
let _11: *mut u8;
let _12: isize;
let _13: [u64; 8];
let _14: f64;
let _15: f64;
let _16: i32;
let _17: *const *const ([char; 8],);
let _18: i64;
let _19: [i8; 8];
let _20: i128;
let _21: char;
let _22: bool;
let _23: i128;
let _24: *const (u32, [i8; 4], u16, [i8; 4]);
let _25: ();
let _26: ();
{
RET = !true;
_6 = 158_u8 as isize;
_7 = _4;
_9 = [_7];
_3 = -_1;
_3 = _2;
_8 = RET as isize;
_9 = [_5];
_10 = !(-35427633861499879337254004996880402816_i128);
_4 = _5;
_9 = [_2];
RET = false ^ false;
RET = false;
_8 = 6_usize as isize;
RET = true ^ false;
_14 = 17080187165543219636_u64 as f64;
RET = false;
Call(_8 = fn13(_4, _9, _4, _5, _7), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = 27_u8 as isize;
_5 = -_2;
_1 = _5;
_8 = _5 >> _5;
_4 = -_5;
_8 = !_4;
RET = false;
_8 = -_4;
Call(_5 = fn14(_4, _4, _9, _8, _9, _2, _2, _8, _1, _8, _2, _8, _7, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = !_8;
_12 = _3;
_12 = _4 & _4;
Call(_1 = core::intrinsics::bswap(_12), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_9 = [_4];
RET = !true;
_7 = 8913105129176105268_u64 as isize;
_16 = 1073841789_i32;
_10 = !52135114023705962317295845493076302336_i128;
_16 = 46539416151448441820062227552778393430_u128 as i32;
_14 = (-23617_i16) as f64;
_6 = !_3;
_1 = _6 | _8;
_3 = _1 ^ _4;
_10 = 157458807687648259579324103714757197882_i128;
_19 = [94_i8,98_i8,(-93_i8),82_i8,(-50_i8),28_i8,7_i8,(-78_i8)];
_3 = !_4;
_6 = 3424274540_u32 as isize;
_16 = _5 as i32;
RET = !false;
_12 = _8 ^ _4;
_6 = RET as isize;
_18 = 3145608778771604756_i64 - 975172934912520793_i64;
_13 = [6783222437398801006_u64,12495712571695031259_u64,15326281013740908639_u64,3440473492916821152_u64,718554621196484133_u64,7641548016592560958_u64,14382882416249580479_u64,4532033926988413157_u64];
_10 = 84309125195584248168271988092795907009_i128;
_6 = _3 + _3;
_7 = _1 >> _4;
_7 = -_12;
match _10 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
84309125195584248168271988092795907009 => bb11,
_ => bb10
}
}
bb4 = {
_1 = !_8;
_12 = _3;
_12 = _4 & _4;
Call(_1 = core::intrinsics::bswap(_12), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_6 = 27_u8 as isize;
_5 = -_2;
_1 = _5;
_8 = _5 >> _5;
_4 = -_5;
_8 = !_4;
RET = false;
_8 = -_4;
Call(_5 = fn14(_4, _4, _9, _8, _9, _2, _2, _8, _1, _8, _2, _8, _7, _1), ReturnTo(bb2), UnwindUnreachable())
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
_15 = _14;
_2 = (-94_i8) as isize;
_16 = (-2072633107_i32) << _5;
_14 = -_15;
match _10 {
0 => bb12,
1 => bb13,
2 => bb14,
84309125195584248168271988092795907009 => bb16,
_ => bb15
}
}
bb12 = {
_9 = [_4];
RET = !true;
_7 = 8913105129176105268_u64 as isize;
_16 = 1073841789_i32;
_10 = !52135114023705962317295845493076302336_i128;
_16 = 46539416151448441820062227552778393430_u128 as i32;
_14 = (-23617_i16) as f64;
_6 = !_3;
_1 = _6 | _8;
_3 = _1 ^ _4;
_10 = 157458807687648259579324103714757197882_i128;
_19 = [94_i8,98_i8,(-93_i8),82_i8,(-50_i8),28_i8,7_i8,(-78_i8)];
_3 = !_4;
_6 = 3424274540_u32 as isize;
_16 = _5 as i32;
RET = !false;
_12 = _8 ^ _4;
_6 = RET as isize;
_18 = 3145608778771604756_i64 - 975172934912520793_i64;
_13 = [6783222437398801006_u64,12495712571695031259_u64,15326281013740908639_u64,3440473492916821152_u64,718554621196484133_u64,7641548016592560958_u64,14382882416249580479_u64,4532033926988413157_u64];
_10 = 84309125195584248168271988092795907009_i128;
_6 = _3 + _3;
_7 = _1 >> _4;
_7 = -_12;
match _10 {
0 => bb2,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
84309125195584248168271988092795907009 => bb11,
_ => bb10
}
}
bb13 = {
_6 = 27_u8 as isize;
_5 = -_2;
_1 = _5;
_8 = _5 >> _5;
_4 = -_5;
_8 = !_4;
RET = false;
_8 = -_4;
Call(_5 = fn14(_4, _4, _9, _8, _9, _2, _2, _8, _1, _8, _2, _8, _7, _1), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
Return()
}
bb15 = {
Return()
}
bb16 = {
_3 = 2964050215_u32 as isize;
_7 = _5;
_21 = '\u{6bbf5}';
_10 = -(-50899368422800834875478793443365470364_i128);
_16 = RET as i32;
_6 = 184_u8 as isize;
_3 = _1;
_1 = _4;
_4 = _1 + _1;
_1 = 6090_u16 as isize;
_9 = [_12];
_5 = _7;
_8 = _5 & _3;
Goto(bb17)
}
bb17 = {
Call(_25 = dump_var(12_usize, 8_usize, Move(_8), 2_usize, Move(_2), 3_usize, Move(_3), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_25 = dump_var(12_usize, 18_usize, Move(_18), 12_usize, Move(_12), 1_usize, Move(_1), 6_usize, Move(_6)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: isize,mut _2: [isize; 1],mut _3: isize,mut _4: isize,mut _5: isize) -> isize {
mir! {
type RET = isize;
let _6: char;
let _7: ();
let _8: ();
{
RET = _4 & _5;
_2 = [_5];
_5 = !_4;
_5 = _1;
_2 = [_1];
RET = 7824860247732060421_i64 as isize;
_5 = _1;
_1 = (-4395768256228989052_i64) as isize;
_3 = _5;
_2 = [_4];
RET = _3;
RET = _4 ^ _3;
_1 = '\u{d1a77}' as isize;
_2 = [_5];
_2 = [_4];
_5 = _4 - _4;
RET = _5;
_1 = _4 * _3;
_4 = _3 << RET;
_3 = -_1;
_5 = 46435_u16 as isize;
_5 = _4 >> _3;
RET = 14530348579145921435_u64 as isize;
RET = -_4;
_2 = [_5];
_2 = [_4];
RET = _4;
_6 = '\u{1038a}';
_4 = _5 + _1;
_4 = -RET;
_2 = [RET];
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(13_usize, 2_usize, Move(_2), 3_usize, Move(_3), 6_usize, Move(_6), 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: isize,mut _2: isize,mut _3: [isize; 1],mut _4: isize,mut _5: [isize; 1],mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize,mut _14: isize) -> isize {
mir! {
type RET = isize;
let _15: isize;
let _16: bool;
let _17: Adt49;
let _18: i16;
let _19: ();
let _20: ();
{
_15 = _10;
RET = -_13;
RET = !_1;
_11 = false as isize;
RET = -_4;
_9 = _13 & _12;
_6 = 73_u8 as isize;
RET = -_8;
_11 = _7;
_13 = RET;
_7 = _8;
_2 = !_10;
RET = _13 >> _14;
_16 = _11 <= _15;
_6 = _4;
_1 = -_7;
RET = _14 - _1;
RET = -_10;
RET = _12;
_18 = (-15505_i16);
_3 = [_1];
Goto(bb1)
}
bb1 = {
Call(_19 = dump_var(14_usize, 4_usize, Move(_4), 14_usize, Move(_14), 16_usize, Move(_16), 6_usize, Move(_6)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_19 = dump_var(14_usize, 15_usize, Move(_15), 3_usize, Move(_3), 11_usize, Move(_11), 2_usize, Move(_2)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize) -> bool {
mir! {
type RET = bool;
let _5: ();
let _6: ();
{
RET = !false;
_4 = _2 + _2;
_3 = _1 >> _2;
RET = _4 >= _1;
Goto(bb1)
}
bb1 = {
Call(_5 = dump_var(15_usize, 2_usize, Move(_2), 3_usize, Move(_3), 6_usize, _6, 6_usize, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: i64,mut _7: usize) -> [i8; 8] {
mir! {
type RET = [i8; 8];
let _8: Adt46;
let _9: u16;
let _10: *mut u16;
let _11: (u64, u32, i16, u16);
let _12: isize;
let _13: u32;
let _14: ([char; 8],);
let _15: Adt48;
let _16: isize;
let _17: u128;
let _18: isize;
let _19: [char; 8];
let _20: isize;
let _21: char;
let _22: f32;
let _23: (u128, *mut u8);
let _24: (u128,);
let _25: isize;
let _26: *mut u16;
let _27: ();
let _28: ();
{
RET = [(-81_i8),30_i8,(-5_i8),62_i8,(-65_i8),(-62_i8),(-113_i8),(-10_i8)];
_8.fld1.fld0 = 134_u8 as u32;
_8.fld1.fld2 = [79_i8,14_i8,(-8_i8),15_i8];
_2 = _4 ^ _5;
_8.fld1.fld4 = [(-19776969996498006378222677693585546861_i128),163892987296888760949664976546211544560_i128];
_8.fld1.fld4 = [(-35924339867457760549829118664266528741_i128),8295137472165964532515160745745985046_i128];
Goto(bb1)
}
bb1 = {
_8.fld1.fld6 = 134681271783071979480031089855075138329_i128 as f64;
_8.fld0 = [(-96_i8),22_i8,(-37_i8),52_i8];
_1 = !_2;
_8.fld1.fld5 = (-502408963_i32);
Call(_8.fld1.fld1 = fn17(_4, _5, _1, _2, _7, _3, _5, _7, _7, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = [(-60_i8),(-127_i8),(-107_i8),67_i8,109_i8,89_i8,55_i8,79_i8];
_4 = _8.fld1.fld6 as isize;
_8.fld1.fld0 = _8.fld1.fld1 as u32;
_11.2 = !(-31088_i16);
_2 = -_3;
_13 = !_8.fld1.fld0;
_5 = -_3;
_10 = core::ptr::addr_of_mut!(_9);
(*_10) = _11.2 as u16;
_12 = (*_10) as isize;
_11 = (7705760483394999469_u64, _8.fld1.fld0, (-9542_i16), (*_10));
RET = [(-116_i8),(-75_i8),(-10_i8),(-54_i8),(-69_i8),(-103_i8),122_i8,121_i8];
(*_10) = !_11.3;
_6 = 5897427718537534421_i64;
_10 = core::ptr::addr_of_mut!(_11.3);
RET = [123_i8,(-114_i8),29_i8,(-118_i8),21_i8,84_i8,(-58_i8),71_i8];
_8.fld3 = !264901971589952615505373701337092377086_u128;
_7 = !0_usize;
_1 = _3 << _3;
Goto(bb3)
}
bb3 = {
_6 = (-8137448144475845980_i64) | 1304427457579742495_i64;
Call(_14.0 = fn18(_2, _3, _2, _3, _3, _2, _1, _3, _1, _1, _8.fld1.fld1, _3, _3, _3, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_11.0 = !14698237257422915302_u64;
_11.1 = _8.fld1.fld0 - _8.fld1.fld0;
_13 = _8.fld1.fld0;
_8.fld0 = _8.fld1.fld2;
_11 = (13283805904569757736_u64, _13, 22241_i16, _9);
(*_10) = _9 ^ _9;
_8.fld0 = [(-14_i8),(-8_i8),109_i8,(-128_i8)];
(*_10) = _9 >> _2;
(*_10) = !_9;
_11.0 = 10682977236594752840_u64 & 7885397639953264979_u64;
_8.fld1.fld5 = 363666476_i32;
_13 = false as u32;
_4 = _8.fld1.fld1 as isize;
_1 = _5 - _5;
_3 = _2;
_8.fld3 = 121234803377319584503425981102437565914_u128 * 92498139889782774740938582242984301636_u128;
_6 = 567637804705269963_i64 << _2;
_5 = !_2;
_14.0 = [_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1];
match _8.fld1.fld5 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5 => bb7,
363666476 => bb9,
_ => bb8
}
}
bb5 = {
_6 = (-8137448144475845980_i64) | 1304427457579742495_i64;
Call(_14.0 = fn18(_2, _3, _2, _3, _3, _2, _1, _3, _1, _1, _8.fld1.fld1, _3, _3, _3, _5), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
RET = [(-60_i8),(-127_i8),(-107_i8),67_i8,109_i8,89_i8,55_i8,79_i8];
_4 = _8.fld1.fld6 as isize;
_8.fld1.fld0 = _8.fld1.fld1 as u32;
_11.2 = !(-31088_i16);
_2 = -_3;
_13 = !_8.fld1.fld0;
_5 = -_3;
_10 = core::ptr::addr_of_mut!(_9);
(*_10) = _11.2 as u16;
_12 = (*_10) as isize;
_11 = (7705760483394999469_u64, _8.fld1.fld0, (-9542_i16), (*_10));
RET = [(-116_i8),(-75_i8),(-10_i8),(-54_i8),(-69_i8),(-103_i8),122_i8,121_i8];
(*_10) = !_11.3;
_6 = 5897427718537534421_i64;
_10 = core::ptr::addr_of_mut!(_11.3);
RET = [123_i8,(-114_i8),29_i8,(-118_i8),21_i8,84_i8,(-58_i8),71_i8];
_8.fld3 = !264901971589952615505373701337092377086_u128;
_7 = !0_usize;
_1 = _3 << _3;
Goto(bb3)
}
bb7 = {
_8.fld1.fld6 = 134681271783071979480031089855075138329_i128 as f64;
_8.fld0 = [(-96_i8),22_i8,(-37_i8),52_i8];
_1 = !_2;
_8.fld1.fld5 = (-502408963_i32);
Call(_8.fld1.fld1 = fn17(_4, _5, _1, _2, _7, _3, _5, _7, _7, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
_8.fld1.fld5 = (-524580421_i32);
_14.0 = [_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1];
_8.fld1.fld2 = [(-38_i8),(-117_i8),86_i8,22_i8];
_6 = (-7281017831585343447_i64) & 3237110394220435143_i64;
_11 = (7458548206058754653_u64, _8.fld1.fld0, 7612_i16, _9);
_8.fld1.fld5 = (-1542701868_i32);
_4 = _2;
_11.0 = !7782196112195959085_u64;
_11.1 = _8.fld1.fld0;
Goto(bb10)
}
bb10 = {
RET = [51_i8,7_i8,85_i8,97_i8,(-24_i8),51_i8,62_i8,(-98_i8)];
_14.0 = [_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1];
_5 = -_4;
_6 = (-3273320127004169932_i64);
_5 = _1;
_8.fld0 = _8.fld1.fld2;
_9 = _11.0 as u16;
_8.fld1.fld6 = _8.fld1.fld5 as f64;
_8.fld3 = _8.fld1.fld0 as u128;
_12 = _5;
Goto(bb11)
}
bb11 = {
(*_10) = _9;
_8.fld1.fld6 = _8.fld3 as f64;
_10 = core::ptr::addr_of_mut!(_9);
_8.fld3 = _11.0 as u128;
_13 = _9 as u32;
_12 = _5 ^ _1;
_8.fld1.fld1 = '\u{8f999}';
_6 = 101_i8 as i64;
_10 = core::ptr::addr_of_mut!((*_10));
_16 = _5;
_8.fld3 = !228649410192280639240828428941271084959_u128;
_17 = !_8.fld3;
RET = [88_i8,(-47_i8),(-33_i8),63_i8,57_i8,18_i8,(-40_i8),94_i8];
RET = [80_i8,(-40_i8),25_i8,(-74_i8),84_i8,102_i8,36_i8,65_i8];
_4 = (*_10) as isize;
_8.fld1.fld6 = _8.fld3 as f64;
_11 = (17070849254740426034_u64, _13, (-10908_i16), _9);
match _11.2 {
0 => bb4,
340282366920938463463374607431768200548 => bb12,
_ => bb8
}
}
bb12 = {
(*_10) = _11.3;
RET = [79_i8,29_i8,12_i8,(-117_i8),(-2_i8),(-75_i8),(-70_i8),(-81_i8)];
_8.fld2 = core::ptr::addr_of_mut!((*_10));
_11.0 = 12938469549352768014_u64 ^ 4699536612709439229_u64;
_13 = _8.fld1.fld0;
_8.fld1.fld0 = _16 as u32;
_11 = (871324716672265620_u64, _8.fld1.fld0, 12533_i16, _9);
(*_10) = !_11.3;
_8.fld1.fld0 = !_11.1;
_3 = false as isize;
_17 = _7 as u128;
_9 = _11.3;
_18 = _12;
_8.fld1.fld2 = [(-101_i8),(-93_i8),(-63_i8),(-49_i8)];
_5 = _16;
_8.fld1.fld0 = _11.1 + _11.1;
_6 = !(-649547402160970757_i64);
_20 = _2 << _16;
_12 = -_16;
_8.fld0 = [43_i8,(-12_i8),(-90_i8),(-56_i8)];
_8.fld1.fld5 = _17 as i32;
_21 = _8.fld1.fld1;
_8.fld3 = _17;
(*_10) = _7 as u16;
_19 = _14.0;
Goto(bb13)
}
bb13 = {
_10 = _8.fld2;
_17 = _8.fld3 + _8.fld3;
_8.fld1.fld0 = _11.0 as u32;
_8.fld1.fld4 = [148954270978019393647074398468898701014_i128,(-90000969562054266340844751200323846616_i128)];
_12 = _18 | _5;
_8.fld1.fld5 = (-1676646497_i32);
_1 = _18;
RET = [(-65_i8),(-24_i8),(-91_i8),107_i8,36_i8,88_i8,(-114_i8),70_i8];
RET = [(-40_i8),48_i8,(-79_i8),(-80_i8),39_i8,(-58_i8),(-63_i8),(-15_i8)];
_23.0 = _8.fld3;
_14 = (_19,);
_3 = _11.0 as isize;
_8.fld1.fld4 = [(-126904454935491464617248901300423117392_i128),140557120163476386872927322264411453318_i128];
_2 = _8.fld1.fld0 as isize;
_11.0 = 13109522789884195235_u64 * 7681880856777625437_u64;
_11.1 = _8.fld1.fld0 | _8.fld1.fld0;
_8.fld1.fld1 = _21;
_7 = (-39_i8) as usize;
_22 = _8.fld1.fld0 as f32;
RET = [64_i8,(-125_i8),(-1_i8),(-73_i8),37_i8,(-66_i8),(-18_i8),105_i8];
match _11.2 {
0 => bb9,
1 => bb8,
2 => bb5,
3 => bb14,
12533 => bb16,
_ => bb15
}
}
bb14 = {
_8.fld1.fld6 = 134681271783071979480031089855075138329_i128 as f64;
_8.fld0 = [(-96_i8),22_i8,(-37_i8),52_i8];
_1 = !_2;
_8.fld1.fld5 = (-502408963_i32);
Call(_8.fld1.fld1 = fn17(_4, _5, _1, _2, _7, _3, _5, _7, _7, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
RET = [51_i8,7_i8,85_i8,97_i8,(-24_i8),51_i8,62_i8,(-98_i8)];
_14.0 = [_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1,_8.fld1.fld1];
_5 = -_4;
_6 = (-3273320127004169932_i64);
_5 = _1;
_8.fld0 = _8.fld1.fld2;
_9 = _11.0 as u16;
_8.fld1.fld6 = _8.fld1.fld5 as f64;
_8.fld3 = _8.fld1.fld0 as u128;
_12 = _5;
Goto(bb11)
}
bb16 = {
_11 = (17037091481222686651_u64, _8.fld1.fld0, 26938_i16, _9);
_18 = _2;
_8.fld1.fld4 = [130693345181296190695385191595798375652_i128,(-61564153105236749824220952227078885057_i128)];
_16 = _5;
_13 = _11.1 << _20;
_8.fld1.fld0 = _13;
_11.1 = 27_u8 as u32;
_8.fld1.fld4 = [112271190776312425377613937456325108492_i128,(-166444549934267769403571732638593331147_i128)];
(*_10) = (-128726031696054018870997199430699397041_i128) as u16;
_14 = (_19,);
_17 = _23.0 - _23.0;
_8.fld0 = [100_i8,(-34_i8),(-69_i8),30_i8];
_3 = _20;
_8.fld2 = core::ptr::addr_of_mut!(_11.3);
_13 = _8.fld1.fld0;
_11 = (15078764555854345772_u64, _13, 14597_i16, _9);
Goto(bb17)
}
bb17 = {
Call(_27 = dump_var(16_usize, 6_usize, Move(_6), 5_usize, Move(_5), 21_usize, Move(_21), 13_usize, Move(_13)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_27 = dump_var(16_usize, 17_usize, Move(_17), 18_usize, Move(_18), 1_usize, Move(_1), 9_usize, Move(_9)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_27 = dump_var(16_usize, 16_usize, Move(_16), 28_usize, _28, 28_usize, _28, 28_usize, _28), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: usize,mut _6: isize,mut _7: isize,mut _8: usize,mut _9: usize,mut _10: i64) -> char {
mir! {
type RET = char;
let _11: char;
let _12: f64;
let _13: i64;
let _14: [isize; 1];
let _15: [i128; 2];
let _16: u8;
let _17: i64;
let _18: (u32, [i8; 4], u16, [i8; 4]);
let _19: [i128; 2];
let _20: (u128,);
let _21: Adt48;
let _22: u8;
let _23: char;
let _24: [char; 8];
let _25: *const *const ([char; 8],);
let _26: bool;
let _27: Adt44;
let _28: Adt55;
let _29: isize;
let _30: bool;
let _31: [u128; 8];
let _32: [u128; 8];
let _33: (u32, [i8; 4], u16, [i8; 4]);
let _34: ();
let _35: ();
{
_9 = !_5;
_1 = '\u{e585e}' as isize;
_11 = '\u{27a6}';
Goto(bb1)
}
bb1 = {
_11 = '\u{93de}';
_2 = (-79_i8) as isize;
RET = _11;
_4 = -_6;
_13 = _10;
_14 = [_3];
_7 = _3 & _3;
_11 = RET;
_8 = _9 + _5;
_8 = !_9;
_1 = _3 - _3;
_13 = _10 & _10;
Call(_14 = core::intrinsics::transmute(_10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = _10 as isize;
Call(_1 = core::intrinsics::transmute(_9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = !_4;
RET = _11;
_15 = [56667429361084478287211004389194164610_i128,(-160839456209137186092599948165720691966_i128)];
Goto(bb4)
}
bb4 = {
_5 = !_9;
_9 = 91_i8 as usize;
_6 = _1;
_11 = RET;
_13 = !_10;
_9 = _8;
_16 = !190_u8;
_4 = _1 * _6;
RET = _11;
_16 = !75_u8;
_10 = _4 as i64;
_6 = -_4;
_4 = _1 * _3;
_11 = RET;
_2 = _1;
Goto(bb5)
}
bb5 = {
_11 = RET;
_6 = _7 * _2;
_9 = !_5;
Goto(bb6)
}
bb6 = {
_12 = _10 as f64;
_17 = _13 >> _13;
_20.0 = 57787738023384048558320511783717035770_u128;
_11 = RET;
_11 = RET;
_18.3 = [(-3_i8),15_i8,(-123_i8),19_i8];
_12 = 51925_u16 as f64;
_2 = !_6;
_12 = _10 as f64;
_13 = -_17;
_6 = !_3;
_6 = _2 << _17;
_15 = [78482738052623856836274785700570758512_i128,(-90908412941784155076599930395828077844_i128)];
_5 = _8 * _8;
Goto(bb7)
}
bb7 = {
_22 = !_16;
_3 = _1 & _6;
_9 = (-10_i8) as usize;
_16 = !_22;
_13 = -_10;
_6 = _20.0 as isize;
_27.fld0 = 874537839_u32 + 4138438774_u32;
_27.fld5 = (-2065492257_i32) * (-2104941186_i32);
_20 = (329645303323443371602272359534580364838_u128,);
_14 = [_4];
_18.2 = !3943_u16;
_27.fld6 = _27.fld5 as f64;
_22 = _16;
_13 = -_10;
_23 = _11;
_27.fld1 = _11;
_27.fld6 = _12;
_18.1 = _18.3;
_6 = _27.fld6 as isize;
_16 = _22;
Goto(bb8)
}
bb8 = {
_7 = _18.2 as isize;
_6 = _2;
_27.fld5 = _23 as i32;
_2 = _3 | _6;
_10 = _18.2 as i64;
_23 = RET;
_18.2 = 5742_u16 * 2357_u16;
_11 = _27.fld1;
_18.0 = _27.fld0 - _27.fld0;
_32 = [_20.0,_20.0,_20.0,_20.0,_20.0,_20.0,_20.0,_20.0];
_18.3 = _18.1;
_26 = !true;
_26 = false;
_1 = _27.fld5 as isize;
_6 = _3;
_7 = _3;
_32 = [_20.0,_20.0,_20.0,_20.0,_20.0,_20.0,_20.0,_20.0];
_4 = _7 - _7;
_6 = _3 >> _2;
_14 = [_3];
_27.fld0 = 117533371770408049370379013500224395601_i128 as u32;
match _20.0 {
0 => bb4,
1 => bb9,
329645303323443371602272359534580364838 => bb11,
_ => bb10
}
}
bb9 = {
_11 = RET;
_6 = _7 * _2;
_9 = !_5;
Goto(bb6)
}
bb10 = {
_12 = _10 as f64;
_17 = _13 >> _13;
_20.0 = 57787738023384048558320511783717035770_u128;
_11 = RET;
_11 = RET;
_18.3 = [(-3_i8),15_i8,(-123_i8),19_i8];
_12 = 51925_u16 as f64;
_2 = !_6;
_12 = _10 as f64;
_13 = -_17;
_6 = !_3;
_6 = _2 << _17;
_15 = [78482738052623856836274785700570758512_i128,(-90908412941784155076599930395828077844_i128)];
_5 = _8 * _8;
Goto(bb7)
}
bb11 = {
_17 = _13 | _13;
_31 = _32;
_26 = true ^ false;
_23 = _11;
_22 = _16 * _16;
_8 = _5;
_31 = [_20.0,_20.0,_20.0,_20.0,_20.0,_20.0,_20.0,_20.0];
_7 = _6;
_3 = _7 | _2;
_17 = _13;
_19 = [(-816905908386897868355024188361616732_i128),(-134418448381862512285333688247484252029_i128)];
_17 = !_13;
_27.fld1 = _11;
match _20.0 {
0 => bb12,
1 => bb13,
329645303323443371602272359534580364838 => bb15,
_ => bb14
}
}
bb12 = {
_12 = _10 as f64;
_17 = _13 >> _13;
_20.0 = 57787738023384048558320511783717035770_u128;
_11 = RET;
_11 = RET;
_18.3 = [(-3_i8),15_i8,(-123_i8),19_i8];
_12 = 51925_u16 as f64;
_2 = !_6;
_12 = _10 as f64;
_13 = -_17;
_6 = !_3;
_6 = _2 << _17;
_15 = [78482738052623856836274785700570758512_i128,(-90908412941784155076599930395828077844_i128)];
_5 = _8 * _8;
Goto(bb7)
}
bb13 = {
_22 = !_16;
_3 = _1 & _6;
_9 = (-10_i8) as usize;
_16 = !_22;
_13 = -_10;
_6 = _20.0 as isize;
_27.fld0 = 874537839_u32 + 4138438774_u32;
_27.fld5 = (-2065492257_i32) * (-2104941186_i32);
_20 = (329645303323443371602272359534580364838_u128,);
_14 = [_4];
_18.2 = !3943_u16;
_27.fld6 = _27.fld5 as f64;
_22 = _16;
_13 = -_10;
_23 = _11;
_27.fld1 = _11;
_27.fld6 = _12;
_18.1 = _18.3;
_6 = _27.fld6 as isize;
_16 = _22;
Goto(bb8)
}
bb14 = {
_11 = RET;
_6 = _7 * _2;
_9 = !_5;
Goto(bb6)
}
bb15 = {
_20.0 = !235375522213819956713830509547009582789_u128;
Goto(bb16)
}
bb16 = {
Call(_34 = dump_var(17_usize, 11_usize, Move(_11), 26_usize, Move(_26), 1_usize, Move(_1), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(17_usize, 18_usize, Move(_18), 8_usize, Move(_8), 17_usize, Move(_17), 16_usize, Move(_16)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(17_usize, 3_usize, Move(_3), 23_usize, Move(_23), 10_usize, Move(_10), 9_usize, Move(_9)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: char,mut _12: isize,mut _13: isize,mut _14: isize,mut _15: isize) -> [char; 8] {
mir! {
type RET = [char; 8];
let _16: ([char; 8],);
let _17: isize;
let _18: (u32, [i8; 4], u16, [i8; 4]);
let _19: u16;
let _20: [i128; 2];
let _21: f64;
let _22: usize;
let _23: u8;
let _24: Adt52;
let _25: Adt46;
let _26: isize;
let _27: [i128; 2];
let _28: &'static i128;
let _29: f32;
let _30: [char; 8];
let _31: ();
let _32: ();
{
_4 = _14;
_11 = '\u{ef683}';
_8 = _10;
_14 = _6 * _6;
_13 = false as isize;
RET = [_11,_11,_11,_11,_11,_11,_11,_11];
_11 = '\u{ee6e1}';
_9 = 13285071130973093248_u64 as isize;
_7 = (-36026115430672934766987648348096102226_i128) as isize;
_1 = _3 + _15;
_14 = 1195609055_u32 as isize;
_16 = (RET,);
_16 = (RET,);
RET = [_11,_11,_11,_11,_11,_11,_11,_11];
RET = _16.0;
_15 = _2 >> _12;
_6 = 4208508640_u32 as isize;
_8 = _15 >> _3;
_11 = '\u{b7351}';
_16 = (RET,);
Call(_18.0 = core::intrinsics::transmute(_11), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_18.3 = [16_i8,44_i8,106_i8,(-69_i8)];
_8 = _5 ^ _1;
_3 = _4;
Goto(bb2)
}
bb2 = {
_4 = false as isize;
Goto(bb3)
}
bb3 = {
RET = [_11,_11,_11,_11,_11,_11,_11,_11];
_8 = 10016822876766088572_usize as isize;
_17 = (-1454787306_i32) as isize;
RET = _16.0;
_15 = _1;
_5 = _1;
_16 = (RET,);
_1 = 123_i8 as isize;
_1 = 253069755681958113503348302161850260894_u128 as isize;
_4 = _10 & _5;
_10 = _5 - _4;
_17 = !_4;
_17 = !_4;
_19 = 7840_u16;
RET = [_11,_11,_11,_11,_11,_11,_11,_11];
_18.1 = [38_i8,35_i8,(-66_i8),55_i8];
_5 = _17;
Goto(bb4)
}
bb4 = {
_17 = _5;
_18.2 = _19 ^ _19;
_20 = [(-39823362186955422119690623191774369870_i128),110186410427734665823659081991700759248_i128];
_3 = !_2;
RET = [_11,_11,_11,_11,_11,_11,_11,_11];
_18.3 = _18.1;
_18.1 = [(-53_i8),105_i8,(-81_i8),13_i8];
Call(_5 = core::intrinsics::transmute(_17), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_20 = [25616201617971932567204341004724528034_i128,(-167040279246982903612618056859075989561_i128)];
_17 = _2 - _5;
_9 = _12 - _5;
_7 = -_2;
_18.0 = !243969335_u32;
_4 = 29550667955593823170467429305715116405_i128 as isize;
_3 = -_7;
_16.0 = [_11,_11,_11,_11,_11,_11,_11,_11];
_10 = _15;
_17 = (-147767364198945664893788603656331877217_i128) as isize;
_2 = 18627_i16 as isize;
_8 = _15 - _5;
_21 = 315079910887210893491108571324357461532_u128 as f64;
_21 = 6980079038481729143_i64 as f64;
_21 = 5289968883775892617_i64 as f64;
_8 = -_5;
_17 = 34_i8 as isize;
_23 = !182_u8;
_18.3 = [97_i8,64_i8,37_i8,(-16_i8)];
_5 = (-1849253719_i32) as isize;
match _19 {
0 => bb1,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
5 => bb10,
7840 => bb12,
_ => bb11
}
}
bb6 = {
_17 = _5;
_18.2 = _19 ^ _19;
_20 = [(-39823362186955422119690623191774369870_i128),110186410427734665823659081991700759248_i128];
_3 = !_2;
RET = [_11,_11,_11,_11,_11,_11,_11,_11];
_18.3 = _18.1;
_18.1 = [(-53_i8),105_i8,(-81_i8),13_i8];
Call(_5 = core::intrinsics::transmute(_17), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
RET = [_11,_11,_11,_11,_11,_11,_11,_11];
_8 = 10016822876766088572_usize as isize;
_17 = (-1454787306_i32) as isize;
RET = _16.0;
_15 = _1;
_5 = _1;
_16 = (RET,);
_1 = 123_i8 as isize;
_1 = 253069755681958113503348302161850260894_u128 as isize;
_4 = _10 & _5;
_10 = _5 - _4;
_17 = !_4;
_17 = !_4;
_19 = 7840_u16;
RET = [_11,_11,_11,_11,_11,_11,_11,_11];
_18.1 = [38_i8,35_i8,(-66_i8),55_i8];
_5 = _17;
Goto(bb4)
}
bb8 = {
_4 = false as isize;
Goto(bb3)
}
bb9 = {
_18.3 = [16_i8,44_i8,106_i8,(-69_i8)];
_8 = _5 ^ _1;
_3 = _4;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_22 = !16662196977560602144_usize;
_8 = !_15;
_18.2 = _19 | _19;
_12 = _9;
RET = _16.0;
_10 = _12;
_13 = _9 + _15;
_18.1 = _18.3;
_17 = 26952_i16 as isize;
_2 = -_15;
_18.1 = _18.3;
_18.3 = [(-124_i8),(-121_i8),(-106_i8),(-49_i8)];
_25.fld0 = [(-51_i8),84_i8,(-89_i8),(-106_i8)];
_25.fld0 = [117_i8,(-60_i8),(-126_i8),(-53_i8)];
_8 = _9 ^ _15;
_25.fld4 = core::ptr::addr_of!(_23);
_18.0 = 4209617806_u32;
_25.fld1.fld6 = _21;
_25.fld1.fld6 = -_21;
_25.fld1.fld5 = (-2096289568_i32);
_25.fld2 = core::ptr::addr_of_mut!(_19);
_17 = _3 ^ _3;
_9 = _2;
_17 = _21 as isize;
match _18.0 {
0 => bb5,
1 => bb11,
2 => bb7,
3 => bb4,
4209617806 => bb14,
_ => bb13
}
}
bb13 = {
RET = [_11,_11,_11,_11,_11,_11,_11,_11];
_8 = 10016822876766088572_usize as isize;
_17 = (-1454787306_i32) as isize;
RET = _16.0;
_15 = _1;
_5 = _1;
_16 = (RET,);
_1 = 123_i8 as isize;
_1 = 253069755681958113503348302161850260894_u128 as isize;
_4 = _10 & _5;
_10 = _5 - _4;
_17 = !_4;
_17 = !_4;
_19 = 7840_u16;
RET = [_11,_11,_11,_11,_11,_11,_11,_11];
_18.1 = [38_i8,35_i8,(-66_i8),55_i8];
_5 = _17;
Goto(bb4)
}
bb14 = {
_9 = _19 as isize;
_8 = _13 ^ _12;
_16.0 = [_11,_11,_11,_11,_11,_11,_11,_11];
_25.fld1.fld6 = _21 - _21;
_11 = '\u{c194}';
_27 = _20;
RET = [_11,_11,_11,_11,_11,_11,_11,_11];
_27 = _20;
_22 = _23 as usize;
_25.fld4 = core::ptr::addr_of!(_23);
_2 = _12 & _7;
_29 = _18.2 as f32;
RET = _16.0;
_25.fld0 = [54_i8,1_i8,(-2_i8),(-39_i8)];
_26 = _12 >> _7;
_9 = _25.fld1.fld6 as isize;
_30 = [_11,_11,_11,_11,_11,_11,_11,_11];
RET = [_11,_11,_11,_11,_11,_11,_11,_11];
_25.fld1.fld4 = [139095064644525905152415476087701577569_i128,(-17115586438696119669210965100154691673_i128)];
_21 = _25.fld1.fld6 + _25.fld1.fld6;
_18.3 = _18.1;
_1 = _26 - _7;
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(18_usize, 10_usize, Move(_10), 3_usize, Move(_3), 22_usize, Move(_22), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(18_usize, 19_usize, Move(_19), 4_usize, Move(_4), 30_usize, Move(_30), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_31 = dump_var(18_usize, 5_usize, Move(_5), 1_usize, Move(_1), 13_usize, Move(_13), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{6e5aa}'), std::hint::black_box(24104680585583041651232214642665968833_u128), std::hint::black_box(65_i8), std::hint::black_box((-13053_i16)), std::hint::black_box(2829195279_u32), std::hint::black_box((-4610548929963770475_i64)), std::hint::black_box((-12280019466432573015785186629381669970_i128)), std::hint::black_box(1624495680908271879_usize), std::hint::black_box(141_u8), std::hint::black_box(10539650195244071403_u64));
                
            }
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
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
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: (*const *const (u32, [i8; 4], u16, [i8; 4]),),
fld1: i16,
fld2: *mut u16,
fld3: u16,

},
Variant1{
fld0: f32,
fld1: ([char; 8],),
fld2: i64,
fld3: *const ([char; 8],),
fld4: i16,

},
Variant2{
fld0: [u64; 8],
fld1: *const i128,
fld2: *const *const (u32, [i8; 4], u16, [i8; 4]),
fld3: [u128; 8],

},
Variant3{
fld0: bool,
fld1: u16,
fld2: u64,
fld3: i128,
fld4: i16,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt43::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: *const ([char; 8],),
fld1: char,
fld2: *mut bool,
fld3: i128,
fld4: *const (u32, [i8; 4], u16, [i8; 4]),
fld5: *const i128,
fld6: u128,

},
Variant1{
fld0: i128,
fld1: [i8; 8],
fld2: f32,

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt44{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt44 {
fld0: u32,
fld1: char,
fld2: [i8; 4],
fld3: *const i128,
fld4: [i128; 2],
fld5: i32,
fld6: f64,
}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: (*const *const (u32, [i8; 4], u16, [i8; 4]),),
fld1: *mut u16,
fld2: isize,
fld3: usize,
fld4: i16,

},
Variant1{
fld0: bool,
fld1: (u32, [i8; 4], u16, [i8; 4]),
fld2: isize,
fld3: [i8; 8],
fld4: *const ([char; 8],),

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: [i8; 4],
fld1: Adt44,
fld2: *mut u16,
fld3: u128,
fld4: *const u8,
}
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: *mut u8,
fld1: [i128; 2],

},
Variant1{
fld0: u16,
fld1: *const u8,
fld2: isize,
fld3: *mut u16,
fld4: [u64; 8],
fld5: *const ([char; 8],),

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: (*const u8,),
fld1: (u32, [i8; 4], u16, [i8; 4]),
fld2: [i128; 2],
fld3: *const *const (u32, [i8; 4], u16, [i8; 4]),
fld4: *const i128,
fld5: usize,
fld6: [i8; 4],
fld7: i128,

},
Variant1{
fld0: [isize; 1],
fld1: Adt42,
fld2: Adt45,
fld3: u32,
fld4: Adt44,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: Adt46,
fld1: ([char; 8],),
fld2: *const i8,
fld3: Adt44,
fld4: i16,
fld5: [u128; 8],
fld6: (u64, u32, i16, u16),
fld7: *mut u16,

},
Variant1{
fld0: u64,
fld1: *mut u8,
fld2: i64,
fld3: (u128,),

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: Adt47,
fld1: Adt48,
fld2: Adt49,
fld3: *mut u16,
fld4: (u128, *mut u8),
fld5: i32,
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: bool,
fld1: u8,
fld2: *mut bool,
fld3: Adt43,
fld4: Adt50,

},
Variant1{
fld0: *const *const ([char; 8],),
fld1: i32,

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt52::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: (*const *const (u32, [i8; 4], u16, [i8; 4]),),
fld1: Adt51,
fld2: Adt47,
fld3: Adt44,
fld4: Adt48,

},
Variant1{
fld0: bool,
fld1: char,
fld2: *const *const (u32, [i8; 4], u16, [i8; 4]),
fld3: *const (u32, [i8; 4], u16, [i8; 4]),
fld4: Adt43,
fld5: *const u8,
fld6: *const ([char; 8],),

},
Variant2{
fld0: *const ([char; 8],),
fld1: usize,
fld2: isize,
fld3: Adt46,
fld4: *const (u32, [i8; 4], u16, [i8; 4]),

},
Variant3{
fld0: ([char; 8],),
fld1: char,
fld2: Adt46,
fld3: [isize; 1],
fld4: usize,
fld5: i32,
fld6: i64,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt53::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: bool,
fld1: char,
fld2: *mut u16,
fld3: u64,
fld4: Adt43,
fld5: (u64, u32, i16, u16),
fld6: Adt49,

},
Variant1{
fld0: bool,
fld1: isize,

},
Variant2{
fld0: i128,
fld1: [i8; 8],

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: *const i128,
fld1: Adt47,
fld2: *const *const ([char; 8],),
fld3: Adt48,
fld4: u8,
fld5: u16,
fld6: u32,
fld7: f32,

},
Variant1{
fld0: (u64, u32, i16, u16),

},
Variant2{
fld0: (u128, *mut u8),
fld1: Adt53,

},
Variant3{
fld0: Adt48,
fld1: *const ([char; 8],),
fld2: (u128,),
fld3: usize,
fld4: [i128; 2],
fld5: [i8; 8],

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: Adt51,
fld1: Adt42,
fld2: Adt45,
fld3: *const *const (u32, [i8; 4], u16, [i8; 4]),
fld4: Adt48,

},
Variant1{
fld0: bool,
fld1: u128,
fld2: isize,
fld3: [u64; 8],
fld4: Adt43,
fld5: *const i8,
fld6: Adt53,

},
Variant2{
fld0: [i8; 4],
fld1: u128,
fld2: u16,
fld3: i8,
fld4: (u64, u32, i16, u16),
fld5: f64,
fld6: i64,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,}=>{
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
},
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: (*const u8,),
fld1: *mut u16,
fld2: Adt45,
fld3: Adt52,

},
Variant1{
fld0: *const *const (u32, [i8; 4], u16, [i8; 4]),
fld1: *const *const ([char; 8],),

},
Variant2{
fld0: *const i128,
fld1: char,
fld2: [isize; 1],
fld3: *const ([char; 8],),
fld4: usize,

},
Variant3{
fld0: u128,
fld1: usize,
fld2: *const u8,
fld3: u64,
fld4: (u32, [i8; 4], u16, [i8; 4]),
fld5: i128,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
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
},
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: Adt55,
fld1: Adt45,
fld2: *const u8,

},
Variant1{
fld0: Adt43,
fld1: [u64; 8],

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt58::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
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
},
	Self::Variant2{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: u8,
fld1: Adt43,
fld2: Adt56,
fld3: Adt52,
fld4: *const *const ([char; 8],),
fld5: i128,

},
Variant1{
fld0: Adt44,
fld1: *const i8,
fld2: [char; 8],
fld3: *const ([char; 8],),
fld4: u128,

},
Variant2{
fld0: Adt42,
fld1: [i8; 4],
fld2: Adt43,
fld3: ([char; 8],),

}}

