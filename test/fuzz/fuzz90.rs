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
pub fn fn0(mut _1: usize,mut _2: i128,mut _3: isize,mut _4: i32,mut _5: i16) -> u16 {
mir! {
type RET = u16;
let _6: [bool; 1];
let _7: bool;
let _8: i8;
let _9: isize;
let _10: u128;
let _11: f64;
let _12: Adt47;
let _13: bool;
let _14: Adt46;
let _15: f32;
let _16: i16;
let _17: u8;
let _18: ();
let _19: ();
{
_2 = -(-146131039827204217688258213871616356878_i128);
RET = false as u16;
RET = !25557_u16;
Call(_1 = fn1(_2, RET, RET, _2, _2, RET, RET, RET, _2, RET, _2, _2, _2, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_5 = !18297_i16;
_6 = [true];
Goto(bb2)
}
bb2 = {
_4 = false as i32;
_3 = 71_isize;
_4 = (-1675192630_i32) + (-124167429_i32);
RET = 55043_u16;
RET = !3959_u16;
_1 = 3603854988_u32 as usize;
_5 = 8944_i16 << _3;
_6 = [true];
_6 = [false];
_5 = 170185233139657315671885949518513126687_u128 as i16;
_1 = 16140591609333780197_usize & 4_usize;
RET = 55925_u16;
_7 = true;
_1 = 6113979689058648335_usize;
_6 = [_7];
RET = 47003_u16 >> _1;
RET = 43544_u16 ^ 56445_u16;
RET = 40762_u16;
RET = 56598_u16;
RET = !10125_u16;
RET = 6583_u16;
_4 = !(-509561059_i32);
Call(_6 = fn19(_3, _3, _3, RET, _1, _3, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3 = !9223372036854775807_isize;
_1 = !9916548557537760049_usize;
_8 = !(-14_i8);
_3 = 9223372036854775807_isize - 9223372036854775807_isize;
RET = _1 as u16;
RET = _7 as u16;
_4 = 860036099_i32 >> _2;
_3 = -(-109_isize);
_2 = 92874747276370672710138828400328241371_i128 ^ 82739710885987740189729465719813056650_i128;
_5 = -5165_i16;
_4 = 1491875887_i32 >> _1;
_1 = 11044164909593250106_usize;
_1 = 35_u8 as usize;
_8 = (-74_i8);
_5 = 134194465491319693783286119450185931320_u128 as i16;
_7 = _4 != _4;
RET = 64905_u16 >> _3;
_1 = !5_usize;
RET = _8 as u16;
_10 = !306539693815436193000705573468648283974_u128;
_2 = _10 as i128;
_8 = 3480665001109635958_i64 as i8;
_9 = 149_u8 as isize;
RET = 21865_u16 + 13099_u16;
Goto(bb4)
}
bb4 = {
_11 = _4 as f64;
_2 = (-28787318193170219466700214015196369155_i128) * (-18685533010859567761767140994703502945_i128);
_10 = !242190022008342152981685114125827063685_u128;
_4 = 1469917724_u32 as i32;
_11 = _8 as f64;
Call(_10 = core::intrinsics::transmute(_2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_9 = _3 ^ _3;
_1 = _5 as usize;
_8 = (-91_i8);
_3 = _9 * _9;
_11 = _1 as f64;
RET = !64401_u16;
_7 = !false;
_8 = (-80_i8) | (-40_i8);
_4 = !(-2111710543_i32);
_6 = [_7];
_3 = !_9;
_1 = 13391557586712705403_usize;
_13 = !_7;
_8 = RET as i8;
match _1 {
0 => bb6,
1 => bb7,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
6 => bb12,
13391557586712705403 => bb14,
_ => bb13
}
}
bb6 = {
_11 = _4 as f64;
_2 = (-28787318193170219466700214015196369155_i128) * (-18685533010859567761767140994703502945_i128);
_10 = !242190022008342152981685114125827063685_u128;
_4 = 1469917724_u32 as i32;
_11 = _8 as f64;
Call(_10 = core::intrinsics::transmute(_2), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_3 = !9223372036854775807_isize;
_1 = !9916548557537760049_usize;
_8 = !(-14_i8);
_3 = 9223372036854775807_isize - 9223372036854775807_isize;
RET = _1 as u16;
RET = _7 as u16;
_4 = 860036099_i32 >> _2;
_3 = -(-109_isize);
_2 = 92874747276370672710138828400328241371_i128 ^ 82739710885987740189729465719813056650_i128;
_5 = -5165_i16;
_4 = 1491875887_i32 >> _1;
_1 = 11044164909593250106_usize;
_1 = 35_u8 as usize;
_8 = (-74_i8);
_5 = 134194465491319693783286119450185931320_u128 as i16;
_7 = _4 != _4;
RET = 64905_u16 >> _3;
_1 = !5_usize;
RET = _8 as u16;
_10 = !306539693815436193000705573468648283974_u128;
_2 = _10 as i128;
_8 = 3480665001109635958_i64 as i8;
_9 = 149_u8 as isize;
RET = 21865_u16 + 13099_u16;
Goto(bb4)
}
bb8 = {
_4 = false as i32;
_3 = 71_isize;
_4 = (-1675192630_i32) + (-124167429_i32);
RET = 55043_u16;
RET = !3959_u16;
_1 = 3603854988_u32 as usize;
_5 = 8944_i16 << _3;
_6 = [true];
_6 = [false];
_5 = 170185233139657315671885949518513126687_u128 as i16;
_1 = 16140591609333780197_usize & 4_usize;
RET = 55925_u16;
_7 = true;
_1 = 6113979689058648335_usize;
_6 = [_7];
RET = 47003_u16 >> _1;
RET = 43544_u16 ^ 56445_u16;
RET = 40762_u16;
RET = 56598_u16;
RET = !10125_u16;
RET = 6583_u16;
_4 = !(-509561059_i32);
Call(_6 = fn19(_3, _3, _3, RET, _1, _3, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_5 = !18297_i16;
_6 = [true];
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
Return()
}
bb14 = {
_6 = [_13];
_1 = 7121894337100667475_usize;
_10 = !114198795828813847910834863465005043077_u128;
_13 = _7;
_7 = !_13;
_17 = _9 as u8;
_5 = -1719_i16;
_3 = -_9;
_9 = !_3;
_6 = [_13];
_10 = '\u{2678}' as u128;
_4 = RET as i32;
Goto(bb15)
}
bb15 = {
Call(_18 = dump_var(0_usize, 17_usize, Move(_17), 2_usize, Move(_2), 13_usize, Move(_13), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_18 = dump_var(0_usize, 5_usize, Move(_5), 1_usize, Move(_1), 19_usize, _19, 19_usize, _19), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: i128,mut _2: u16,mut _3: u16,mut _4: i128,mut _5: i128,mut _6: u16,mut _7: u16,mut _8: u16,mut _9: i128,mut _10: u16,mut _11: i128,mut _12: i128,mut _13: i128,mut _14: u16) -> usize {
mir! {
type RET = usize;
let _15: u128;
let _16: f64;
let _17: [i64; 1];
let _18: [char; 6];
let _19: u64;
let _20: [isize; 8];
let _21: i32;
let _22: i8;
let _23: f64;
let _24: [usize; 5];
let _25: isize;
let _26: Adt46;
let _27: [i8; 1];
let _28: [bool; 1];
let _29: Adt40;
let _30: ([i8; 1], u128, bool);
let _31: f64;
let _32: ();
let _33: ();
{
_2 = _6 + _7;
RET = _14 as usize;
_9 = !_5;
_3 = _6;
_8 = _2;
_9 = !_11;
_4 = !_1;
_8 = 1348926190_u32 as u16;
_3 = _2;
_5 = _1;
_2 = !_10;
_3 = _10;
_1 = 884286188_u32 as i128;
_14 = _7 << _5;
_7 = _6 | _3;
RET = 151_u8 as usize;
_15 = 336969736423947218690407737630094974265_u128 - 84645189715735160646212937857739969850_u128;
RET = 736853061_i32 as usize;
_1 = _11;
_6 = _8 >> RET;
Call(RET = fn2(_7, _6, _9, _7, _8, _6, _3, _12, _10, _1, _2, _10, _7, _14, _13, _8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = _6 & _14;
RET = 9223372036854775807_isize as usize;
_2 = _15 as u16;
_16 = (-106_isize) as f64;
_4 = _9 * _13;
_16 = RET as f64;
_8 = !_2;
_11 = _9 - _4;
_17 = [7734711093545718846_i64];
_4 = _16 as i128;
RET = !15013417702311523693_usize;
_7 = _2 << _11;
_5 = _9;
Goto(bb2)
}
bb2 = {
_6 = !_14;
_12 = _5 | _13;
RET = 7754543734288836413_usize;
RET = 3_usize;
_20 = [(-35_isize),(-9223372036854775808_isize),5_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_20[RET] = 9223372036854775807_isize;
_13 = 3538393741_u32 as i128;
RET = 3_usize;
_4 = _5;
_8 = false as u16;
_16 = (-1295401567_i32) as f64;
Goto(bb3)
}
bb3 = {
RET = _7 as usize;
_12 = _11;
_6 = _10 << RET;
_7 = _2;
_12 = -_4;
_19 = 104_u8 as u64;
_7 = _19 as u16;
RET = 6_usize;
_17 = [9163224081054520033_i64];
_6 = (-17438_i16) as u16;
_25 = !_20[RET];
RET = 1_usize + 16175487220594274380_usize;
_2 = !_3;
Goto(bb4)
}
bb4 = {
_2 = _7;
_22 = !(-87_i8);
_19 = (-652780782_i32) as u64;
_7 = _8 * _14;
_23 = _16;
_23 = -_16;
_20 = [_25,_25,_25,_25,_25,_25,_25,_25];
_22 = 21_i8;
RET = !1681850741716361262_usize;
_3 = _8;
_4 = _1;
_8 = _7;
_8 = _7 >> _25;
_10 = !_8;
_15 = !37346489944868591091035293572321363029_u128;
_7 = !_8;
_1 = -_11;
_12 = _15 as i128;
RET = (-401874375_i32) as usize;
match _22 {
0 => bb1,
1 => bb3,
2 => bb5,
3 => bb6,
4 => bb7,
21 => bb9,
_ => bb8
}
}
bb5 = {
RET = _7 as usize;
_12 = _11;
_6 = _10 << RET;
_7 = _2;
_12 = -_4;
_19 = 104_u8 as u64;
_7 = _19 as u16;
RET = 6_usize;
_17 = [9163224081054520033_i64];
_6 = (-17438_i16) as u16;
_25 = !_20[RET];
RET = 1_usize + 16175487220594274380_usize;
_2 = !_3;
Goto(bb4)
}
bb6 = {
_6 = !_14;
_12 = _5 | _13;
RET = 7754543734288836413_usize;
RET = 3_usize;
_20 = [(-35_isize),(-9223372036854775808_isize),5_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_20[RET] = 9223372036854775807_isize;
_13 = 3538393741_u32 as i128;
RET = 3_usize;
_4 = _5;
_8 = false as u16;
_16 = (-1295401567_i32) as f64;
Goto(bb3)
}
bb7 = {
_7 = _6 & _14;
RET = 9223372036854775807_isize as usize;
_2 = _15 as u16;
_16 = (-106_isize) as f64;
_4 = _9 * _13;
_16 = RET as f64;
_8 = !_2;
_11 = _9 - _4;
_17 = [7734711093545718846_i64];
_4 = _16 as i128;
RET = !15013417702311523693_usize;
_7 = _2 << _11;
_5 = _9;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
_4 = (-29877_i16) as i128;
_18 = ['\u{3d368}','\u{8d86c}','\u{1079cb}','\u{1c89f}','\u{5acf}','\u{10ec0b}'];
_21 = !(-1541062580_i32);
_28 = [false];
_27 = [_22];
_11 = false as i128;
_21 = _19 as i32;
_23 = -_16;
_7 = _8;
_9 = -_12;
_8 = _7 >> _12;
match _22 {
0 => bb6,
1 => bb2,
2 => bb3,
3 => bb10,
4 => bb11,
5 => bb12,
21 => bb14,
_ => bb13
}
}
bb10 = {
Return()
}
bb11 = {
_7 = _6 & _14;
RET = 9223372036854775807_isize as usize;
_2 = _15 as u16;
_16 = (-106_isize) as f64;
_4 = _9 * _13;
_16 = RET as f64;
_8 = !_2;
_11 = _9 - _4;
_17 = [7734711093545718846_i64];
_4 = _16 as i128;
RET = !15013417702311523693_usize;
_7 = _2 << _11;
_5 = _9;
Goto(bb2)
}
bb12 = {
_7 = _6 & _14;
RET = 9223372036854775807_isize as usize;
_2 = _15 as u16;
_16 = (-106_isize) as f64;
_4 = _9 * _13;
_16 = RET as f64;
_8 = !_2;
_11 = _9 - _4;
_17 = [7734711093545718846_i64];
_4 = _16 as i128;
RET = !15013417702311523693_usize;
_7 = _2 << _11;
_5 = _9;
Goto(bb2)
}
bb13 = {
RET = _7 as usize;
_12 = _11;
_6 = _10 << RET;
_7 = _2;
_12 = -_4;
_19 = 104_u8 as u64;
_7 = _19 as u16;
RET = 6_usize;
_17 = [9163224081054520033_i64];
_6 = (-17438_i16) as u16;
_25 = !_20[RET];
RET = 1_usize + 16175487220594274380_usize;
_2 = !_3;
Goto(bb4)
}
bb14 = {
_16 = _1 as f64;
_3 = _10;
RET = 1_usize | 3754055125071715090_usize;
_28 = [true];
_30 = (_27, _15, false);
_1 = _12;
_18 = ['\u{10cf95}','\u{c36b6}','\u{668bd}','\u{9bcd4}','\u{fd17c}','\u{6354f}'];
_21 = (-1496868278_i32) & (-1776784150_i32);
RET = !14722458047810081131_usize;
_25 = 9223372036854775807_isize;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(1_usize, 14_usize, Move(_14), 28_usize, Move(_28), 2_usize, Move(_2), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(1_usize, 30_usize, Move(_30), 6_usize, Move(_6), 7_usize, Move(_7), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(1_usize, 17_usize, Move(_17), 10_usize, Move(_10), 4_usize, Move(_4), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: u16,mut _2: u16,mut _3: i128,mut _4: u16,mut _5: u16,mut _6: u16,mut _7: u16,mut _8: i128,mut _9: u16,mut _10: i128,mut _11: u16,mut _12: u16,mut _13: u16,mut _14: u16,mut _15: i128,mut _16: u16) -> usize {
mir! {
type RET = usize;
let _17: Adt51;
let _18: [i8; 1];
let _19: [i128; 6];
let _20: isize;
let _21: isize;
let _22: char;
let _23: &'static u8;
let _24: Adt41;
let _25: u32;
let _26: f32;
let _27: [u64; 8];
let _28: [usize; 7];
let _29: Adt46;
let _30: u32;
let _31: isize;
let _32: u8;
let _33: i32;
let _34: ();
let _35: ();
{
_12 = _6;
RET = 17260446948388020215_usize + 3_usize;
_6 = _1;
Goto(bb1)
}
bb1 = {
_8 = _15 >> _5;
RET = 8261350575273297023_usize - 1_usize;
_17.fld2 = 296074846_u32 + 416704736_u32;
_17.fld0 = 1214829028130227486_u64 + 13982472907556764185_u64;
_4 = false as u16;
_1 = !_11;
_7 = _14;
_17.fld2 = 44923779_u32;
_10 = _17.fld2 as i128;
_17.fld6 = !_14;
_1 = 219_u8 as u16;
_17.fld6 = _2 + _5;
_12 = _11;
_12 = (-3921337759673843655_i64) as u16;
_17.fld5 = ['\u{843bd}','\u{20463}','\u{108314}','\u{d815e}','\u{8fbdf}','\u{dc695}'];
_9 = _14 << _7;
RET = 2_usize >> _9;
_8 = 2997907471667218213_i64 as i128;
_17.fld2 = _17.fld0 as u32;
_17.fld3 = -(-94_i8);
_18 = [_17.fld3];
_17.fld5 = ['\u{55167}','\u{ec9a5}','\u{4d3b0}','\u{10a4be}','\u{55ada}','\u{528d3}'];
Call(_17.fld5 = fn3(RET, _13, _6, RET, _16, _9, RET, _3, _5, _14, _5, RET, _14, _17.fld2, _11, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Goto(bb3)
}
bb3 = {
_13 = _17.fld2 as u16;
_11 = _9;
_17.fld3 = -4_i8;
_14 = !_4;
RET = _17.fld2 as usize;
_22 = '\u{b5f11}';
_22 = '\u{71683}';
_10 = 4886634574974224760_i64 as i128;
_17.fld5 = [_22,_22,_22,_22,_22,_22];
_5 = _9;
_17.fld5 = [_22,_22,_22,_22,_22,_22];
Goto(bb4)
}
bb4 = {
_21 = _22 as isize;
_17.fld7 = Adt45::Variant2 { fld0: 1993636772_i32,fld1: 18418_i16 };
_19 = [_8,_15,_3,_10,_8,_3];
_1 = _6;
_20 = _21;
_2 = _1 | _5;
_3 = _15;
_5 = _13;
_11 = _2;
RET = _22 as usize;
place!(Field::<i16>(Variant(_17.fld7, 2), 1)) = 4374_i16 << _17.fld0;
_7 = !_14;
_7 = !_11;
_4 = _17.fld2 as u16;
_13 = _9;
_15 = _22 as i128;
_1 = _17.fld6 + _11;
_8 = _20 as i128;
_15 = _10;
Call(_25 = core::intrinsics::bswap(_17.fld2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET = _17.fld3 as usize;
_16 = _11;
_6 = _11;
place!(Field::<i32>(Variant(_17.fld7, 2), 0)) = 237795686_i32;
_5 = _11 << _11;
_7 = _17.fld0 as u16;
_17.fld3 = !7_i8;
_14 = _17.fld2 as u16;
_17.fld1 = [_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0];
_17.fld5 = [_22,_22,_22,_22,_22,_22];
_12 = _5;
_22 = '\u{cea0b}';
_7 = _5 + _1;
_18 = [_17.fld3];
_22 = '\u{4273f}';
_5 = _2;
_26 = _17.fld2 as f32;
_13 = _17.fld6;
_17.fld5 = [_22,_22,_22,_22,_22,_22];
_20 = -_21;
_27 = _17.fld1;
_20 = _21;
_6 = !_7;
_14 = _6 >> _12;
_14 = _6 + _6;
_3 = 314614777424007742679800830345842051139_u128 as i128;
_17.fld0 = _22 as u64;
_15 = _17.fld3 as i128;
Goto(bb6)
}
bb6 = {
_17.fld0 = 11974807873053092389_u64 * 2031239977677401929_u64;
_16 = !_7;
_9 = _12;
_17.fld5 = [_22,_22,_22,_22,_22,_22];
_6 = _5;
place!(Field::<i16>(Variant(_17.fld7, 2), 1)) = -633_i16;
RET = 6_usize * 0_usize;
_25 = _17.fld2;
Call(place!(Field::<i32>(Variant(_17.fld7, 2), 0)) = core::intrinsics::bswap(1776325816_i32), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_17.fld2 = !_25;
_21 = _20;
_17.fld5 = [_22,_22,_22,_22,_22,_22];
_14 = _1;
_13 = _5;
_15 = !_3;
_17.fld0 = 15786966902964534728_u64;
_17.fld4 = [Field::<i16>(Variant(_17.fld7, 2), 1),Field::<i16>(Variant(_17.fld7, 2), 1),Field::<i16>(Variant(_17.fld7, 2), 1)];
match _17.fld0 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
15786966902964534728 => bb13,
_ => bb12
}
}
bb8 = {
_17.fld0 = 11974807873053092389_u64 * 2031239977677401929_u64;
_16 = !_7;
_9 = _12;
_17.fld5 = [_22,_22,_22,_22,_22,_22];
_6 = _5;
place!(Field::<i16>(Variant(_17.fld7, 2), 1)) = -633_i16;
RET = 6_usize * 0_usize;
_25 = _17.fld2;
Call(place!(Field::<i32>(Variant(_17.fld7, 2), 0)) = core::intrinsics::bswap(1776325816_i32), ReturnTo(bb7), UnwindUnreachable())
}
bb9 = {
RET = _17.fld3 as usize;
_16 = _11;
_6 = _11;
place!(Field::<i32>(Variant(_17.fld7, 2), 0)) = 237795686_i32;
_5 = _11 << _11;
_7 = _17.fld0 as u16;
_17.fld3 = !7_i8;
_14 = _17.fld2 as u16;
_17.fld1 = [_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0,_17.fld0];
_17.fld5 = [_22,_22,_22,_22,_22,_22];
_12 = _5;
_22 = '\u{cea0b}';
_7 = _5 + _1;
_18 = [_17.fld3];
_22 = '\u{4273f}';
_5 = _2;
_26 = _17.fld2 as f32;
_13 = _17.fld6;
_17.fld5 = [_22,_22,_22,_22,_22,_22];
_20 = -_21;
_27 = _17.fld1;
_20 = _21;
_6 = !_7;
_14 = _6 >> _12;
_14 = _6 + _6;
_3 = 314614777424007742679800830345842051139_u128 as i128;
_17.fld0 = _22 as u64;
_15 = _17.fld3 as i128;
Goto(bb6)
}
bb10 = {
_21 = _22 as isize;
_17.fld7 = Adt45::Variant2 { fld0: 1993636772_i32,fld1: 18418_i16 };
_19 = [_8,_15,_3,_10,_8,_3];
_1 = _6;
_20 = _21;
_2 = _1 | _5;
_3 = _15;
_5 = _13;
_11 = _2;
RET = _22 as usize;
place!(Field::<i16>(Variant(_17.fld7, 2), 1)) = 4374_i16 << _17.fld0;
_7 = !_14;
_7 = !_11;
_4 = _17.fld2 as u16;
_13 = _9;
_15 = _22 as i128;
_1 = _17.fld6 + _11;
_8 = _20 as i128;
_15 = _10;
Call(_25 = core::intrinsics::bswap(_17.fld2), ReturnTo(bb5), UnwindUnreachable())
}
bb11 = {
_13 = _17.fld2 as u16;
_11 = _9;
_17.fld3 = -4_i8;
_14 = !_4;
RET = _17.fld2 as usize;
_22 = '\u{b5f11}';
_22 = '\u{71683}';
_10 = 4886634574974224760_i64 as i128;
_17.fld5 = [_22,_22,_22,_22,_22,_22];
_5 = _9;
_17.fld5 = [_22,_22,_22,_22,_22,_22];
Goto(bb4)
}
bb12 = {
Goto(bb3)
}
bb13 = {
_22 = '\u{7027f}';
_16 = _2;
_7 = Field::<i32>(Variant(_17.fld7, 2), 0) as u16;
_27 = _17.fld1;
_10 = _15;
_21 = RET as isize;
_14 = _13 ^ _13;
_20 = _21 << _9;
RET = false as usize;
place!(Field::<i16>(Variant(_17.fld7, 2), 1)) = _15 as i16;
Goto(bb14)
}
bb14 = {
_2 = !_12;
_17.fld0 = !2515171878224126127_u64;
_19 = [_15,_10,_15,_15,_15,_10];
SetDiscriminant(_17.fld7, 1);
_26 = _17.fld2 as f32;
_32 = !91_u8;
_17.fld3 = false as i8;
_22 = '\u{93a44}';
_17.fld2 = !_25;
_21 = _20;
_17.fld4 = [22613_i16,1668_i16,(-26527_i16)];
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(2_usize, 9_usize, Move(_9), 2_usize, Move(_2), 22_usize, Move(_22), 32_usize, Move(_32)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(2_usize, 10_usize, Move(_10), 19_usize, Move(_19), 1_usize, Move(_1), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(2_usize, 21_usize, Move(_21), 8_usize, Move(_8), 27_usize, Move(_27), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: usize,mut _2: u16,mut _3: u16,mut _4: usize,mut _5: u16,mut _6: u16,mut _7: usize,mut _8: i128,mut _9: u16,mut _10: u16,mut _11: u16,mut _12: usize,mut _13: u16,mut _14: u32,mut _15: u16,mut _16: u16) -> [char; 6] {
mir! {
type RET = [char; 6];
let _17: Adt50;
let _18: f32;
let _19: isize;
let _20: f32;
let _21: isize;
let _22: [char; 6];
let _23: [u64; 8];
let _24: char;
let _25: [isize; 8];
let _26: [u64; 8];
let _27: f64;
let _28: u8;
let _29: [i128; 6];
let _30: *mut (bool,);
let _31: isize;
let _32: [u64; 8];
let _33: *const f32;
let _34: bool;
let _35: *const f32;
let _36: bool;
let _37: i16;
let _38: isize;
let _39: char;
let _40: [usize; 7];
let _41: f32;
let _42: ();
let _43: ();
{
_6 = _16;
_6 = _15;
RET = ['\u{c0b40}','\u{fd210}','\u{25f88}','\u{38359}','\u{cff9a}','\u{9907c}'];
_7 = !_4;
_8 = -(-72958062530075797052098015961703852850_i128);
_12 = 1700480388_i32 as usize;
_12 = !_7;
_18 = 26754_i16 as f32;
RET = ['\u{12fb7}','\u{1afec}','\u{ed6ea}','\u{9b6ae}','\u{b0f8e}','\u{e7008}'];
_5 = 2385119462760084036_u64 as u16;
_16 = 26_isize as u16;
_5 = 29599_i16 as u16;
_16 = !_10;
_9 = !_2;
_14 = 2900878914_u32 | 4160780079_u32;
_10 = !_13;
_7 = _4;
_10 = !_9;
Goto(bb1)
}
bb1 = {
_5 = _6 >> _12;
_8 = (-133896515976165204919260731899438250693_i128);
RET = ['\u{10e394}','\u{e20e7}','\u{aae8c}','\u{e5d02}','\u{8248a}','\u{b73bc}'];
_4 = _7;
_9 = _3 | _5;
_1 = !_7;
_21 = 9223372036854775807_isize;
RET = ['\u{c3f8a}','\u{76953}','\u{fc385}','\u{5589b}','\u{57887}','\u{10f3e7}'];
_23 = [5669903979069880060_u64,3760334229488746790_u64,5362736114926534054_u64,11779270553794434959_u64,2699167671209730948_u64,9936517872361634680_u64,16480656420098811534_u64,1230165789783039215_u64];
RET = ['\u{7f1fa}','\u{a826f}','\u{10c8bf}','\u{f7b0e}','\u{49056}','\u{e4e35}'];
RET = ['\u{107b7a}','\u{ecee4}','\u{f43d}','\u{8518c}','\u{ef7ec}','\u{bdc40}'];
_14 = 3754868437_u32 * 2337739430_u32;
_2 = _5 | _9;
_4 = !_12;
_22 = ['\u{95f63}','\u{23dba}','\u{f9cda}','\u{5032c}','\u{cbcb2}','\u{54f76}'];
_20 = _18;
_7 = _1;
Call(_26 = fn4(_21, _2, _15, _23, _22), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_25 = [_21,_21,_21,_21,_21,_21,_21,_21];
_28 = (-487102994_i32) as u8;
_23 = _26;
_23 = [13719370650487826182_u64,8267707450744918521_u64,17674992539899469584_u64,1492469855084468204_u64,18179396049076865872_u64,11004783875515472465_u64,7784188320058228603_u64,12645865748961917889_u64];
_20 = -_18;
_19 = 542115206_i32 as isize;
_16 = _2;
_20 = _18;
_3 = _5;
_7 = _1;
_10 = _5 - _16;
_8 = _14 as i128;
_2 = _3 ^ _10;
_8 = !(-98626388815082289694959899674576770218_i128);
_13 = 4803090759407369742_u64 as u16;
_25 = [_21,_19,_19,_19,_19,_21,_19,_19];
_25 = [_21,_19,_19,_21,_21,_21,_21,_21];
_3 = _2;
RET = ['\u{946ea}','\u{933b9}','\u{a9102}','\u{1098c6}','\u{7e9bb}','\u{52843}'];
_29 = [_8,_8,_8,_8,_8,_8];
Call(_4 = core::intrinsics::transmute(_19), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = _22;
_18 = 143742935695220234706046362633785178151_u128 as f32;
_10 = _5 | _2;
_5 = _16;
_24 = '\u{2e5e1}';
_24 = '\u{f98d0}';
_13 = _2;
_9 = _5 * _16;
_7 = !_12;
_24 = '\u{9f5c3}';
_9 = _10;
_10 = !_16;
_2 = !_3;
_28 = 102_u8;
_19 = _21 + _21;
_12 = !_7;
_8 = _18 as i128;
_18 = _20;
match _21 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
9223372036854775807 => bb10,
_ => bb9
}
}
bb4 = {
_25 = [_21,_21,_21,_21,_21,_21,_21,_21];
_28 = (-487102994_i32) as u8;
_23 = _26;
_23 = [13719370650487826182_u64,8267707450744918521_u64,17674992539899469584_u64,1492469855084468204_u64,18179396049076865872_u64,11004783875515472465_u64,7784188320058228603_u64,12645865748961917889_u64];
_20 = -_18;
_19 = 542115206_i32 as isize;
_16 = _2;
_20 = _18;
_3 = _5;
_7 = _1;
_10 = _5 - _16;
_8 = _14 as i128;
_2 = _3 ^ _10;
_8 = !(-98626388815082289694959899674576770218_i128);
_13 = 4803090759407369742_u64 as u16;
_25 = [_21,_19,_19,_19,_19,_21,_19,_19];
_25 = [_21,_19,_19,_21,_21,_21,_21,_21];
_3 = _2;
RET = ['\u{946ea}','\u{933b9}','\u{a9102}','\u{1098c6}','\u{7e9bb}','\u{52843}'];
_29 = [_8,_8,_8,_8,_8,_8];
Call(_4 = core::intrinsics::transmute(_19), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_5 = _6 >> _12;
_8 = (-133896515976165204919260731899438250693_i128);
RET = ['\u{10e394}','\u{e20e7}','\u{aae8c}','\u{e5d02}','\u{8248a}','\u{b73bc}'];
_4 = _7;
_9 = _3 | _5;
_1 = !_7;
_21 = 9223372036854775807_isize;
RET = ['\u{c3f8a}','\u{76953}','\u{fc385}','\u{5589b}','\u{57887}','\u{10f3e7}'];
_23 = [5669903979069880060_u64,3760334229488746790_u64,5362736114926534054_u64,11779270553794434959_u64,2699167671209730948_u64,9936517872361634680_u64,16480656420098811534_u64,1230165789783039215_u64];
RET = ['\u{7f1fa}','\u{a826f}','\u{10c8bf}','\u{f7b0e}','\u{49056}','\u{e4e35}'];
RET = ['\u{107b7a}','\u{ecee4}','\u{f43d}','\u{8518c}','\u{ef7ec}','\u{bdc40}'];
_14 = 3754868437_u32 * 2337739430_u32;
_2 = _5 | _9;
_4 = !_12;
_22 = ['\u{95f63}','\u{23dba}','\u{f9cda}','\u{5032c}','\u{cbcb2}','\u{54f76}'];
_20 = _18;
_7 = _1;
Call(_26 = fn4(_21, _2, _15, _23, _22), ReturnTo(bb2), UnwindUnreachable())
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
_12 = _7 * _1;
_10 = _16 | _9;
_22 = [_24,_24,_24,_24,_24,_24];
_23 = [17696775441280693075_u64,6528587970316426540_u64,6910079371367188778_u64,11552600543207245152_u64,1512711094750850592_u64,4768548000377012037_u64,8742349894106185853_u64,10631153680535056672_u64];
_31 = _21 ^ _21;
_28 = 4063845176202684319_i64 as u8;
RET = [_24,_24,_24,_24,_24,_24];
_27 = (-81_i8) as f64;
_11 = _5 + _2;
_5 = _3 & _2;
_20 = _18;
_3 = !_10;
_4 = _20 as usize;
_11 = _9;
_21 = !_19;
_21 = -_19;
_34 = false;
_35 = core::ptr::addr_of!(_18);
_27 = (-23406_i16) as f64;
_35 = core::ptr::addr_of!(_18);
_1 = _12;
_17 = Adt50::Variant0 { fld0: _14,fld1: _24 };
_26 = _23;
_36 = !_34;
_14 = Field::<u32>(Variant(_17, 0), 0) + Field::<u32>(Variant(_17, 0), 0);
_34 = !_36;
_5 = !_9;
Goto(bb11)
}
bb11 = {
_20 = (*_35) * (*_35);
SetDiscriminant(_17, 1);
_13 = _3;
_13 = _3 + _10;
place!(Field::<f32>(Variant(_17, 1), 0)) = -_20;
_12 = _7 | _1;
_37 = 58_i8 as i16;
_6 = _10;
RET = _22;
_13 = _5 << _6;
Goto(bb12)
}
bb12 = {
place!(Field::<(u32,)>(Variant(_17, 1), 2)) = (_14,);
place!(Field::<[i16; 3]>(Variant(_17, 1), 3)) = [_37,_37,_37];
place!(Field::<[isize; 8]>(Variant(_17, 1), 1)) = [_19,_21,_21,_31,_19,_19,_19,_31];
_10 = !_2;
_22 = [_24,_24,_24,_24,_24,_24];
_32 = [14291499296071327701_u64,16881458648380396729_u64,8159800219464585720_u64,3616555922625690812_u64,12935917949143560247_u64,2012314382036455230_u64,15597903184387040552_u64,12414058459298105629_u64];
_27 = _28 as f64;
Goto(bb13)
}
bb13 = {
_9 = (-779631669_i32) as u16;
_8 = 16957267565678693615_u64 as i128;
_24 = '\u{860}';
RET = _22;
_10 = !_6;
_13 = !_10;
_29 = [_8,_8,_8,_8,_8,_8];
RET = [_24,_24,_24,_24,_24,_24];
place!(Field::<(u32,)>(Variant(_17, 1), 2)) = (_14,);
_4 = _1;
_18 = _16 as f32;
_27 = 1213094650_i32 as f64;
place!(Field::<[i16; 3]>(Variant(_17, 1), 3)) = [_37,_37,_37];
_35 = core::ptr::addr_of!(_20);
_22 = [_24,_24,_24,_24,_24,_24];
place!(Field::<i16>(Variant(_17, 1), 4)) = _24 as i16;
_39 = _24;
Goto(bb14)
}
bb14 = {
_40 = [_4,_12,_7,_1,_7,_12,_1];
place!(Field::<i16>(Variant(_17, 1), 4)) = Field::<(u32,)>(Variant(_17, 1), 2).0 as i16;
_26 = _32;
(*_35) = _18 * _18;
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(3_usize, 28_usize, Move(_28), 5_usize, Move(_5), 15_usize, Move(_15), 1_usize, Move(_1)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(3_usize, 19_usize, Move(_19), 9_usize, Move(_9), 32_usize, Move(_32), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(3_usize, 8_usize, Move(_8), 13_usize, Move(_13), 23_usize, Move(_23), 14_usize, Move(_14)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(3_usize, 12_usize, Move(_12), 10_usize, Move(_10), 31_usize, Move(_31), 16_usize, Move(_16)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: isize,mut _2: u16,mut _3: u16,mut _4: [u64; 8],mut _5: [char; 6]) -> [u64; 8] {
mir! {
type RET = [u64; 8];
let _6: char;
let _7: f32;
let _8: bool;
let _9: (u32,);
let _10: Adt51;
let _11: bool;
let _12: f32;
let _13: (&'static u8, usize);
let _14: isize;
let _15: i32;
let _16: [usize; 7];
let _17: f64;
let _18: u128;
let _19: usize;
let _20: u32;
let _21: [i128; 4];
let _22: Adt49;
let _23: bool;
let _24: [usize; 5];
let _25: u8;
let _26: Adt46;
let _27: [u16; 5];
let _28: char;
let _29: bool;
let _30: [usize; 7];
let _31: isize;
let _32: Adt40;
let _33: *mut (bool,);
let _34: ();
let _35: ();
{
RET = [13023050275361109055_u64,12072418808229381958_u64,1412963181980138572_u64,17276417962702027792_u64,15287023920871979869_u64,11697112015197708474_u64,10591899427899817277_u64,7419746923624330668_u64];
_1 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
RET = [9682849663995632338_u64,6781617169269007738_u64,15831477275439241546_u64,10000146499613005395_u64,18405870781115390022_u64,8058729599431260716_u64,4724589578543914119_u64,9551964790369506518_u64];
Call(_1 = fn5(RET, _4, _2, _2, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = _3;
_4 = [1639936047810881802_u64,9452096445977569238_u64,15359718893179605044_u64,11626285718611162886_u64,15560078284313628820_u64,4849760341579649146_u64,5674480041323640674_u64,1336672250949074164_u64];
_4 = RET;
_1 = (-531201901_i32) as isize;
_4 = [10081434852348156529_u64,11762203037200950427_u64,9121545200941200797_u64,3436210364831826796_u64,13410321870602681901_u64,8977162689017460880_u64,3445651174492318461_u64,17834060054701540244_u64];
_6 = '\u{3efa0}';
RET = _4;
_1 = (-9223372036854775808_isize);
_4 = RET;
_6 = '\u{10ef88}';
_6 = '\u{48950}';
_6 = '\u{a62f5}';
RET = [10239551067020349416_u64,2129037067218996846_u64,10790988368521988917_u64,10799384227622897348_u64,1296161036483592691_u64,3409764903440373399_u64,4328358476695024789_u64,4128785613965724700_u64];
_1 = _6 as isize;
_3 = 31305246655908506546025201513829682499_i128 as u16;
_1 = (-9223372036854775808_isize) * (-125_isize);
_7 = 22859_i16 as f32;
_4 = [4848278016294179643_u64,637349907151497152_u64,18209975686548238104_u64,13718653555577844132_u64,14909777215579957273_u64,9256149840687787725_u64,16858287482907569790_u64,6044809905142602796_u64];
_8 = false;
_1 = 9223372036854775807_isize << _2;
_4 = [3016910951772975946_u64,2481130429282913490_u64,234164236672944134_u64,8816602038569855806_u64,4380648334150734323_u64,11671927927075650958_u64,6054164017004921175_u64,8647218785597510570_u64];
_10.fld1 = _4;
_3 = !_2;
_6 = '\u{c2102}';
_11 = _8;
Goto(bb2)
}
bb2 = {
_10.fld5 = _5;
_10.fld6 = !_3;
_10.fld0 = 1167952349517905424_u64;
_9 = (4225473852_u32,);
_10.fld3 = _7 as i8;
_10.fld4 = [(-27218_i16),9583_i16,(-11611_i16)];
_5 = [_6,_6,_6,_6,_6,_6];
_5 = _10.fld5;
_10.fld6 = 9085584844753262268_usize as u16;
_10.fld3 = 1163364414_i32 as i8;
_3 = _1 as u16;
_2 = 1001527409331532714_i64 as u16;
_9.0 = 145961052618498587304423724826289062315_i128 as u32;
_14 = _1 << _10.fld0;
_10.fld2 = (-18923_i16) as u32;
_4 = [_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0];
_15 = -236318228_i32;
_10.fld6 = (-6010113744097153863_i64) as u16;
_10.fld0 = 5159576659491570411_u64;
_10.fld6 = !_2;
_15 = !204895393_i32;
_9 = (_10.fld2,);
_5 = _10.fld5;
_10.fld0 = !14852705355885058555_u64;
Goto(bb3)
}
bb3 = {
_8 = _11 > _11;
_2 = _7 as u16;
RET = [_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0];
_12 = _7;
RET = _10.fld1;
_14 = _1;
_10.fld4 = [25670_i16,7735_i16,(-5019_i16)];
_10.fld0 = 12581374724116219119_u64;
_5 = _10.fld5;
_9 = (_10.fld2,);
_6 = '\u{e63f6}';
_10.fld2 = _9.0;
_6 = '\u{53096}';
_13.1 = _10.fld0 as usize;
_10.fld0 = 37_u8 as u64;
_10.fld2 = !_9.0;
_15 = 1698610320_i32 >> _2;
_11 = _8;
_12 = (-158444640943343023624834404537121790473_i128) as f32;
_10.fld4 = [(-16720_i16),(-12442_i16),1035_i16];
Goto(bb4)
}
bb4 = {
_10.fld4 = [(-30437_i16),12992_i16,(-5182_i16)];
_7 = -_12;
_2 = !_3;
_3 = _12 as u16;
_10.fld6 = _3 - _2;
_9 = (_10.fld2,);
_15 = (-923314190_i32);
_11 = _8 & _8;
_7 = _12;
_19 = _10.fld6 as usize;
_16 = [_19,_19,_19,_19,_19,_13.1,_13.1];
_10.fld7 = Adt45::Variant2 { fld0: _15,fld1: 14996_i16 };
_12 = _7;
_10.fld6 = _10.fld2 as u16;
_17 = 22792737388925526975549655115036150679_i128 as f64;
_8 = _11;
_16 = [_19,_19,_19,_19,_13.1,_19,_19];
_5 = [_6,_6,_6,_6,_6,_6];
_9.0 = _10.fld2;
_21 = [(-105373243434157096521452287113548221917_i128),7828618960977303818419119788932950467_i128,139477396109572474416150939435870997401_i128,16209577871504980743208861997129962000_i128];
_10.fld1 = [_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0];
RET = [_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0];
_10.fld6 = 25643697290064337944520674788246334398_u128 as u16;
_10.fld5 = _5;
place!(Field::<i32>(Variant(_10.fld7, 2), 0)) = !_15;
_6 = '\u{475b9}';
match _15 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
340282366920938463463374607430844897266 => bb7,
_ => bb6
}
}
bb5 = {
_2 = _3;
_4 = [1639936047810881802_u64,9452096445977569238_u64,15359718893179605044_u64,11626285718611162886_u64,15560078284313628820_u64,4849760341579649146_u64,5674480041323640674_u64,1336672250949074164_u64];
_4 = RET;
_1 = (-531201901_i32) as isize;
_4 = [10081434852348156529_u64,11762203037200950427_u64,9121545200941200797_u64,3436210364831826796_u64,13410321870602681901_u64,8977162689017460880_u64,3445651174492318461_u64,17834060054701540244_u64];
_6 = '\u{3efa0}';
RET = _4;
_1 = (-9223372036854775808_isize);
_4 = RET;
_6 = '\u{10ef88}';
_6 = '\u{48950}';
_6 = '\u{a62f5}';
RET = [10239551067020349416_u64,2129037067218996846_u64,10790988368521988917_u64,10799384227622897348_u64,1296161036483592691_u64,3409764903440373399_u64,4328358476695024789_u64,4128785613965724700_u64];
_1 = _6 as isize;
_3 = 31305246655908506546025201513829682499_i128 as u16;
_1 = (-9223372036854775808_isize) * (-125_isize);
_7 = 22859_i16 as f32;
_4 = [4848278016294179643_u64,637349907151497152_u64,18209975686548238104_u64,13718653555577844132_u64,14909777215579957273_u64,9256149840687787725_u64,16858287482907569790_u64,6044809905142602796_u64];
_8 = false;
_1 = 9223372036854775807_isize << _2;
_4 = [3016910951772975946_u64,2481130429282913490_u64,234164236672944134_u64,8816602038569855806_u64,4380648334150734323_u64,11671927927075650958_u64,6054164017004921175_u64,8647218785597510570_u64];
_10.fld1 = _4;
_3 = !_2;
_6 = '\u{c2102}';
_11 = _8;
Goto(bb2)
}
bb6 = {
_10.fld5 = _5;
_10.fld6 = !_3;
_10.fld0 = 1167952349517905424_u64;
_9 = (4225473852_u32,);
_10.fld3 = _7 as i8;
_10.fld4 = [(-27218_i16),9583_i16,(-11611_i16)];
_5 = [_6,_6,_6,_6,_6,_6];
_5 = _10.fld5;
_10.fld6 = 9085584844753262268_usize as u16;
_10.fld3 = 1163364414_i32 as i8;
_3 = _1 as u16;
_2 = 1001527409331532714_i64 as u16;
_9.0 = 145961052618498587304423724826289062315_i128 as u32;
_14 = _1 << _10.fld0;
_10.fld2 = (-18923_i16) as u32;
_4 = [_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0];
_15 = -236318228_i32;
_10.fld6 = (-6010113744097153863_i64) as u16;
_10.fld0 = 5159576659491570411_u64;
_10.fld6 = !_2;
_15 = !204895393_i32;
_9 = (_10.fld2,);
_5 = _10.fld5;
_10.fld0 = !14852705355885058555_u64;
Goto(bb3)
}
bb7 = {
_10.fld5 = [_6,_6,_6,_6,_6,_6];
_19 = !_13.1;
match _15 {
0 => bb4,
1 => bb6,
2 => bb8,
3 => bb9,
4 => bb10,
340282366920938463463374607430844897266 => bb12,
_ => bb11
}
}
bb8 = {
_2 = _3;
_4 = [1639936047810881802_u64,9452096445977569238_u64,15359718893179605044_u64,11626285718611162886_u64,15560078284313628820_u64,4849760341579649146_u64,5674480041323640674_u64,1336672250949074164_u64];
_4 = RET;
_1 = (-531201901_i32) as isize;
_4 = [10081434852348156529_u64,11762203037200950427_u64,9121545200941200797_u64,3436210364831826796_u64,13410321870602681901_u64,8977162689017460880_u64,3445651174492318461_u64,17834060054701540244_u64];
_6 = '\u{3efa0}';
RET = _4;
_1 = (-9223372036854775808_isize);
_4 = RET;
_6 = '\u{10ef88}';
_6 = '\u{48950}';
_6 = '\u{a62f5}';
RET = [10239551067020349416_u64,2129037067218996846_u64,10790988368521988917_u64,10799384227622897348_u64,1296161036483592691_u64,3409764903440373399_u64,4328358476695024789_u64,4128785613965724700_u64];
_1 = _6 as isize;
_3 = 31305246655908506546025201513829682499_i128 as u16;
_1 = (-9223372036854775808_isize) * (-125_isize);
_7 = 22859_i16 as f32;
_4 = [4848278016294179643_u64,637349907151497152_u64,18209975686548238104_u64,13718653555577844132_u64,14909777215579957273_u64,9256149840687787725_u64,16858287482907569790_u64,6044809905142602796_u64];
_8 = false;
_1 = 9223372036854775807_isize << _2;
_4 = [3016910951772975946_u64,2481130429282913490_u64,234164236672944134_u64,8816602038569855806_u64,4380648334150734323_u64,11671927927075650958_u64,6054164017004921175_u64,8647218785597510570_u64];
_10.fld1 = _4;
_3 = !_2;
_6 = '\u{c2102}';
_11 = _8;
Goto(bb2)
}
bb9 = {
_10.fld5 = _5;
_10.fld6 = !_3;
_10.fld0 = 1167952349517905424_u64;
_9 = (4225473852_u32,);
_10.fld3 = _7 as i8;
_10.fld4 = [(-27218_i16),9583_i16,(-11611_i16)];
_5 = [_6,_6,_6,_6,_6,_6];
_5 = _10.fld5;
_10.fld6 = 9085584844753262268_usize as u16;
_10.fld3 = 1163364414_i32 as i8;
_3 = _1 as u16;
_2 = 1001527409331532714_i64 as u16;
_9.0 = 145961052618498587304423724826289062315_i128 as u32;
_14 = _1 << _10.fld0;
_10.fld2 = (-18923_i16) as u32;
_4 = [_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0];
_15 = -236318228_i32;
_10.fld6 = (-6010113744097153863_i64) as u16;
_10.fld0 = 5159576659491570411_u64;
_10.fld6 = !_2;
_15 = !204895393_i32;
_9 = (_10.fld2,);
_5 = _10.fld5;
_10.fld0 = !14852705355885058555_u64;
Goto(bb3)
}
bb10 = {
_10.fld4 = [(-30437_i16),12992_i16,(-5182_i16)];
_7 = -_12;
_2 = !_3;
_3 = _12 as u16;
_10.fld6 = _3 - _2;
_9 = (_10.fld2,);
_15 = (-923314190_i32);
_11 = _8 & _8;
_7 = _12;
_19 = _10.fld6 as usize;
_16 = [_19,_19,_19,_19,_19,_13.1,_13.1];
_10.fld7 = Adt45::Variant2 { fld0: _15,fld1: 14996_i16 };
_12 = _7;
_10.fld6 = _10.fld2 as u16;
_17 = 22792737388925526975549655115036150679_i128 as f64;
_8 = _11;
_16 = [_19,_19,_19,_19,_13.1,_19,_19];
_5 = [_6,_6,_6,_6,_6,_6];
_9.0 = _10.fld2;
_21 = [(-105373243434157096521452287113548221917_i128),7828618960977303818419119788932950467_i128,139477396109572474416150939435870997401_i128,16209577871504980743208861997129962000_i128];
_10.fld1 = [_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0];
RET = [_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0];
_10.fld6 = 25643697290064337944520674788246334398_u128 as u16;
_10.fld5 = _5;
place!(Field::<i32>(Variant(_10.fld7, 2), 0)) = !_15;
_6 = '\u{475b9}';
match _15 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
340282366920938463463374607430844897266 => bb7,
_ => bb6
}
}
bb11 = {
_8 = _11 > _11;
_2 = _7 as u16;
RET = [_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0];
_12 = _7;
RET = _10.fld1;
_14 = _1;
_10.fld4 = [25670_i16,7735_i16,(-5019_i16)];
_10.fld0 = 12581374724116219119_u64;
_5 = _10.fld5;
_9 = (_10.fld2,);
_6 = '\u{e63f6}';
_10.fld2 = _9.0;
_6 = '\u{53096}';
_13.1 = _10.fld0 as usize;
_10.fld0 = 37_u8 as u64;
_10.fld2 = !_9.0;
_15 = 1698610320_i32 >> _2;
_11 = _8;
_12 = (-158444640943343023624834404537121790473_i128) as f32;
_10.fld4 = [(-16720_i16),(-12442_i16),1035_i16];
Goto(bb4)
}
bb12 = {
_10.fld5 = _5;
_21 = [64399287256027388163395156667126497341_i128,152057282140931115266365031545118115467_i128,107064482502422535795041472806825605830_i128,(-87086298922867384352208102349522361599_i128)];
_13.1 = _19 + _19;
_18 = 148058680910140652573482629629694740630_u128;
_20 = _10.fld2;
_16 = [_13.1,_13.1,_19,_19,_13.1,_19,_13.1];
_9.0 = _10.fld3 as u32;
_10.fld2 = _9.0 & _9.0;
_6 = '\u{e4bb}';
_10.fld6 = _3;
_23 = _8 & _11;
_15 = Field::<i32>(Variant(_10.fld7, 2), 0);
_25 = !133_u8;
_12 = _7;
_4 = [_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0];
_10.fld2 = _17 as u32;
place!(Field::<i16>(Variant(_10.fld7, 2), 1)) = (-601592956421828979_i64) as i16;
_10.fld1 = _4;
_4 = [_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0];
_13.1 = _19;
_1 = _14 * _14;
_4 = [_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0];
_1 = _10.fld0 as isize;
_10.fld1 = [_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0];
_2 = _10.fld6 - _10.fld6;
RET = [_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0];
_14 = -_1;
_3 = _2 ^ _10.fld6;
RET = [_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0];
match _18 {
148058680910140652573482629629694740630 => bb14,
_ => bb13
}
}
bb13 = {
_10.fld5 = _5;
_10.fld6 = !_3;
_10.fld0 = 1167952349517905424_u64;
_9 = (4225473852_u32,);
_10.fld3 = _7 as i8;
_10.fld4 = [(-27218_i16),9583_i16,(-11611_i16)];
_5 = [_6,_6,_6,_6,_6,_6];
_5 = _10.fld5;
_10.fld6 = 9085584844753262268_usize as u16;
_10.fld3 = 1163364414_i32 as i8;
_3 = _1 as u16;
_2 = 1001527409331532714_i64 as u16;
_9.0 = 145961052618498587304423724826289062315_i128 as u32;
_14 = _1 << _10.fld0;
_10.fld2 = (-18923_i16) as u32;
_4 = [_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0];
_15 = -236318228_i32;
_10.fld6 = (-6010113744097153863_i64) as u16;
_10.fld0 = 5159576659491570411_u64;
_10.fld6 = !_2;
_15 = !204895393_i32;
_9 = (_10.fld2,);
_5 = _10.fld5;
_10.fld0 = !14852705355885058555_u64;
Goto(bb3)
}
bb14 = {
_19 = !_13.1;
_9.0 = _25 as u32;
_10.fld1 = [_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0];
_19 = _6 as usize;
_27 = [_10.fld6,_3,_10.fld6,_10.fld6,_2];
_21 = [(-114198511348962684237072211518481024773_i128),139296819799593933688470668094215472385_i128,114337786119689655249272366417390138471_i128,131866145273866463971570625729008607891_i128];
_24 = [_19,_19,_19,_13.1,_19];
_25 = _10.fld6 as u8;
_28 = _6;
_10.fld5 = [_6,_6,_28,_28,_6,_6];
SetDiscriminant(_10.fld7, 2);
place!(Field::<i32>(Variant(_10.fld7, 2), 0)) = 116107521941173401175437313121580825981_i128 as i32;
_15 = !Field::<i32>(Variant(_10.fld7, 2), 0);
_14 = _1;
_7 = -_12;
_10.fld1 = [_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0,_10.fld0];
_31 = _1;
_19 = _13.1;
_23 = _15 != Field::<i32>(Variant(_10.fld7, 2), 0);
_24 = [_13.1,_19,_13.1,_19,_19];
_13.0 = &_25;
_5 = _10.fld5;
_7 = -_12;
_2 = !_3;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(4_usize, 31_usize, Move(_31), 4_usize, Move(_4), 16_usize, Move(_16), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(4_usize, 8_usize, Move(_8), 11_usize, Move(_11), 19_usize, Move(_19), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(4_usize, 14_usize, Move(_14), 15_usize, Move(_15), 21_usize, Move(_21), 35_usize, _35), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: [u64; 8],mut _2: [u64; 8],mut _3: u16,mut _4: u16,mut _5: [char; 6]) -> isize {
mir! {
type RET = isize;
let _6: [u64; 8];
let _7: char;
let _8: isize;
let _9: bool;
let _10: [i64; 1];
let _11: char;
let _12: Adt39;
let _13: u128;
let _14: [i64; 1];
let _15: (bool,);
let _16: i8;
let _17: char;
let _18: Adt48;
let _19: ([i8; 1], u128, bool);
let _20: u64;
let _21: Adt39;
let _22: i32;
let _23: Adt42;
let _24: Adt43;
let _25: *const f32;
let _26: [bool; 1];
let _27: char;
let _28: Adt51;
let _29: Adt48;
let _30: u8;
let _31: Adt50;
let _32: usize;
let _33: f64;
let _34: char;
let _35: [u64; 8];
let _36: [u16; 5];
let _37: char;
let _38: isize;
let _39: i8;
let _40: [i128; 6];
let _41: i8;
let _42: Adt40;
let _43: [usize; 7];
let _44: isize;
let _45: f64;
let _46: [i64; 1];
let _47: *mut (bool,);
let _48: i16;
let _49: usize;
let _50: ();
let _51: ();
{
_2 = _1;
_6 = _2;
RET = 64_isize * (-9223372036854775808_isize);
RET = !(-9223372036854775808_isize);
_2 = [10601159375816129558_u64,7949304595580623804_u64,8691919020266192563_u64,11152801936082099759_u64,14497213894435197611_u64,8697728399964346848_u64,15767433457440098024_u64,300279786962045121_u64];
Call(RET = core::intrinsics::bswap(9223372036854775807_isize), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7 = '\u{781e1}';
_3 = _4;
_10 = [(-7359766159952640736_i64)];
_8 = RET * RET;
_10 = [2718159696188713493_i64];
_8 = -RET;
_1 = _6;
_6 = [11627185476079106226_u64,2268123271350718655_u64,14263607786752847097_u64,13343128580099882021_u64,135610634399857036_u64,15627304025422745557_u64,16263668271592595388_u64,9653461190115868602_u64];
_2 = _1;
_7 = '\u{7b2f5}';
_13 = !93446783316674815817288753739646277197_u128;
_13 = !192733041809260745473943095424868121569_u128;
_10 = [(-1669905626503579766_i64)];
RET = !_8;
Goto(bb2)
}
bb2 = {
_14 = [4827269602992090578_i64];
_10 = _14;
_11 = _7;
_3 = _4 * _4;
_17 = _11;
_13 = 58352462347388254873689198328399334460_u128 - 24569897780952328535967978445040118813_u128;
_13 = !260416605266525709066969359555359582883_u128;
_9 = !true;
_13 = !93398730620208974538971635847100981607_u128;
_6 = [7950558740635558600_u64,3628217307812617189_u64,12702152996400025104_u64,7648496502982139736_u64,8994433041062208035_u64,3245655614467625323_u64,10601721503365458039_u64,8209625753067442963_u64];
_4 = _3;
_1 = _6;
_3 = (-1245584095849824168_i64) as u16;
Goto(bb3)
}
bb3 = {
_17 = _7;
RET = _8 | _8;
_15 = (_9,);
_16 = 86_i8;
_15.0 = !_9;
_4 = (-3018216163376262955_i64) as u16;
_7 = _11;
_7 = _11;
_4 = _3 + _3;
_16 = 126_i8 - 66_i8;
_15.0 = _4 < _4;
Goto(bb4)
}
bb4 = {
_20 = 12782759540718257465_u64;
_6 = [_20,_20,_20,_20,_20,_20,_20,_20];
_19.0 = [_16];
Call(_10 = fn6(_1, _15.0, _2, _5), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_14 = [1744398846053472810_i64];
_9 = _15.0;
_11 = _7;
_10 = [453986140990261177_i64];
_19.2 = _15.0;
_11 = _7;
_15.0 = RET == RET;
_20 = 1_usize as u64;
_13 = 54275599429359898232476439151680288061_u128 + 171138962717263932533543192258314057721_u128;
_14 = _10;
_10 = _14;
Goto(bb6)
}
bb6 = {
_28.fld6 = _3;
RET = !_8;
_27 = _17;
_22 = -(-436045441_i32);
Goto(bb7)
}
bb7 = {
_9 = _17 == _17;
_22 = (-1048747116_i32);
RET = -_8;
_19.2 = _9 == _15.0;
_19.1 = _8 as u128;
_13 = _19.1;
_28.fld5 = _5;
_6 = [_20,_20,_20,_20,_20,_20,_20,_20];
RET = 2555736296_u32 as isize;
_28.fld4 = [32323_i16,5220_i16,28449_i16];
RET = (-2268_i16) as isize;
_6 = [_20,_20,_20,_20,_20,_20,_20,_20];
_17 = _11;
_2 = [_20,_20,_20,_20,_20,_20,_20,_20];
_30 = 150_u8;
_28.fld4 = [(-29776_i16),(-1501_i16),(-26223_i16)];
_11 = _17;
Goto(bb8)
}
bb8 = {
_19.0 = [_16];
_27 = _17;
_9 = !_15.0;
_28.fld1 = [_20,_20,_20,_20,_20,_20,_20,_20];
_29 = Adt48::Variant2 { fld0: (-3076856279907359605_i64),fld1: _11 };
_32 = !4592483025424891318_usize;
_35 = [_20,_20,_20,_20,_20,_20,_20,_20];
_28.fld7 = Adt45::Variant2 { fld0: _22,fld1: (-7268_i16) };
_26 = [_9];
_3 = _4 >> Field::<i32>(Variant(_28.fld7, 2), 0);
_1 = [_20,_20,_20,_20,_20,_20,_20,_20];
_1 = [_20,_20,_20,_20,_20,_20,_20,_20];
_15.0 = _3 != _4;
_27 = _7;
_14 = [(-195658771383064533_i64)];
_35 = [_20,_20,_20,_20,_20,_20,_20,_20];
_28.fld0 = Field::<char>(Variant(_29, 2), 1) as u64;
_8 = RET;
_28.fld6 = _4 ^ _3;
_28.fld5 = _5;
_4 = _28.fld6;
_18 = Adt48::Variant2 { fld0: (-6270465560222738386_i64),fld1: _11 };
_28.fld0 = _20;
_15 = (_19.2,);
_3 = _28.fld6;
place!(Field::<i64>(Variant(_29, 2), 0)) = 9085728028612655296_i64;
_28.fld2 = 2939796135_u32;
Call(_25 = fn8(_6, _28.fld2, RET, _13, Move(_29), _3, _6, _26, _30, _9, _7, _9, _7, _4, _26), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_34 = _7;
_5 = [_34,Field::<char>(Variant(_18, 2), 1),_34,Field::<char>(Variant(_18, 2), 1),_34,Field::<char>(Variant(_18, 2), 1)];
_31 = Adt50::Variant0 { fld0: _28.fld2,fld1: _11 };
_8 = _3 as isize;
_36 = [_3,_3,_28.fld6,_4,_3];
_28.fld3 = -_16;
_19.0 = [_28.fld3];
_7 = Field::<char>(Variant(_18, 2), 1);
place!(Field::<i64>(Variant(_18, 2), 0)) = -(-184244093593933653_i64);
_28.fld4 = [3065_i16,(-7827_i16),25976_i16];
_10 = [Field::<i64>(Variant(_18, 2), 0)];
RET = _8;
SetDiscriminant(_18, 1);
_7 = _27;
_19.0 = [_28.fld3];
_37 = _11;
_5 = [_37,_11,_34,_7,Field::<char>(Variant(_31, 0), 1),_11];
_22 = (-4466_i16) as i32;
place!(Field::<u32>(Variant(_31, 0), 0)) = !_28.fld2;
_3 = _28.fld6 | _28.fld6;
Call(_40 = fn10(RET, _34, _19.2, _28.fld3, _14, RET, _28.fld6, _6, _19, _15.0, _27, Field::<i32>(Variant(_28.fld7, 2), 0), _3, _28.fld6), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_33 = _19.1 as f64;
SetDiscriminant(_31, 1);
place!(Field::<[isize; 8]>(Variant(_31, 1), 1)) = [RET,RET,_8,_8,RET,RET,_8,_8];
_10 = [(-7007232384749148065_i64)];
_21 = Adt39::Variant1 { fld0: _19.0,fld1: _32,fld2: Field::<[isize; 8]>(Variant(_31, 1), 1),fld3: (-2195872431394451876_i64),fld4: _30,fld5: _28.fld4 };
_21 = Adt39::Variant0 { fld0: Field::<[isize; 8]>(Variant(_31, 1), 1),fld1: _28.fld0,fld2: _6,fld3: _19,fld4: _14 };
place!(Field::<(u32,)>(Variant(_31, 1), 2)) = (_28.fld2,);
place!(Field::<([i8; 1], u128, bool)>(Variant(_21, 0), 3)).2 = _15.0 <= _9;
_6 = _2;
place!(Field::<f32>(Variant(_31, 1), 0)) = (-2616_i16) as f32;
_19.0 = [_28.fld3];
_38 = RET - RET;
_43 = [_32,_32,_32,_32,_32,_32,_32];
_22 = _33 as i32;
_19 = (Field::<([i8; 1], u128, bool)>(Variant(_21, 0), 3).0, Field::<([i8; 1], u128, bool)>(Variant(_21, 0), 3).1, _15.0);
place!(Field::<[i16; 3]>(Variant(_31, 1), 3)) = [(-12668_i16),(-5867_i16),(-3151_i16)];
_15.0 = Field::<([i8; 1], u128, bool)>(Variant(_21, 0), 3).2;
_35 = _28.fld1;
_23 = Adt42::Variant1 { fld0: Field::<[i64; 1]>(Variant(_21, 0), 4),fld1: Field::<([i8; 1], u128, bool)>(Variant(_21, 0), 3),fld2: Field::<([i8; 1], u128, bool)>(Variant(_21, 0), 3).0,fld3: _28.fld4,fld4: _33,fld5: _25,fld6: Field::<(u32,)>(Variant(_31, 1), 2) };
_28.fld4 = [(-18837_i16),(-9082_i16),26865_i16];
_28.fld4 = [253_i16,652_i16,(-20844_i16)];
_32 = _33 as usize;
_37 = _34;
place!(Field::<[i64; 1]>(Variant(_23, 1), 0)) = [(-5133015486337048996_i64)];
_44 = Field::<([i8; 1], u128, bool)>(Variant(_21, 0), 3).1 as isize;
_28.fld5 = [_27,_27,_7,_7,_34,_11];
_39 = _28.fld3;
match Field::<(u32,)>(Variant(_31, 1), 2).0 {
0 => bb5,
1 => bb11,
2 => bb12,
2939796135 => bb14,
_ => bb13
}
}
bb11 = {
_34 = _7;
_5 = [_34,Field::<char>(Variant(_18, 2), 1),_34,Field::<char>(Variant(_18, 2), 1),_34,Field::<char>(Variant(_18, 2), 1)];
_31 = Adt50::Variant0 { fld0: _28.fld2,fld1: _11 };
_8 = _3 as isize;
_36 = [_3,_3,_28.fld6,_4,_3];
_28.fld3 = -_16;
_19.0 = [_28.fld3];
_7 = Field::<char>(Variant(_18, 2), 1);
place!(Field::<i64>(Variant(_18, 2), 0)) = -(-184244093593933653_i64);
_28.fld4 = [3065_i16,(-7827_i16),25976_i16];
_10 = [Field::<i64>(Variant(_18, 2), 0)];
RET = _8;
SetDiscriminant(_18, 1);
_7 = _27;
_19.0 = [_28.fld3];
_37 = _11;
_5 = [_37,_11,_34,_7,Field::<char>(Variant(_31, 0), 1),_11];
_22 = (-4466_i16) as i32;
place!(Field::<u32>(Variant(_31, 0), 0)) = !_28.fld2;
_3 = _28.fld6 | _28.fld6;
Call(_40 = fn10(RET, _34, _19.2, _28.fld3, _14, RET, _28.fld6, _6, _19, _15.0, _27, Field::<i32>(Variant(_28.fld7, 2), 0), _3, _28.fld6), ReturnTo(bb10), UnwindUnreachable())
}
bb12 = {
_19.0 = [_16];
_27 = _17;
_9 = !_15.0;
_28.fld1 = [_20,_20,_20,_20,_20,_20,_20,_20];
_29 = Adt48::Variant2 { fld0: (-3076856279907359605_i64),fld1: _11 };
_32 = !4592483025424891318_usize;
_35 = [_20,_20,_20,_20,_20,_20,_20,_20];
_28.fld7 = Adt45::Variant2 { fld0: _22,fld1: (-7268_i16) };
_26 = [_9];
_3 = _4 >> Field::<i32>(Variant(_28.fld7, 2), 0);
_1 = [_20,_20,_20,_20,_20,_20,_20,_20];
_1 = [_20,_20,_20,_20,_20,_20,_20,_20];
_15.0 = _3 != _4;
_27 = _7;
_14 = [(-195658771383064533_i64)];
_35 = [_20,_20,_20,_20,_20,_20,_20,_20];
_28.fld0 = Field::<char>(Variant(_29, 2), 1) as u64;
_8 = RET;
_28.fld6 = _4 ^ _3;
_28.fld5 = _5;
_4 = _28.fld6;
_18 = Adt48::Variant2 { fld0: (-6270465560222738386_i64),fld1: _11 };
_28.fld0 = _20;
_15 = (_19.2,);
_3 = _28.fld6;
place!(Field::<i64>(Variant(_29, 2), 0)) = 9085728028612655296_i64;
_28.fld2 = 2939796135_u32;
Call(_25 = fn8(_6, _28.fld2, RET, _13, Move(_29), _3, _6, _26, _30, _9, _7, _9, _7, _4, _26), ReturnTo(bb9), UnwindUnreachable())
}
bb13 = {
_7 = '\u{781e1}';
_3 = _4;
_10 = [(-7359766159952640736_i64)];
_8 = RET * RET;
_10 = [2718159696188713493_i64];
_8 = -RET;
_1 = _6;
_6 = [11627185476079106226_u64,2268123271350718655_u64,14263607786752847097_u64,13343128580099882021_u64,135610634399857036_u64,15627304025422745557_u64,16263668271592595388_u64,9653461190115868602_u64];
_2 = _1;
_7 = '\u{7b2f5}';
_13 = !93446783316674815817288753739646277197_u128;
_13 = !192733041809260745473943095424868121569_u128;
_10 = [(-1669905626503579766_i64)];
RET = !_8;
Goto(bb2)
}
bb14 = {
_19 = (Field::<([i8; 1], u128, bool)>(Variant(_21, 0), 3).0, Field::<([i8; 1], u128, bool)>(Variant(_21, 0), 3).1, _9);
_41 = -_39;
place!(Field::<[u64; 8]>(Variant(_21, 0), 2)) = [_20,_20,_20,_20,_28.fld0,Field::<u64>(Variant(_21, 0), 1),_28.fld0,Field::<u64>(Variant(_21, 0), 1)];
_10 = [(-4042608595340515652_i64)];
_18 = Adt48::Variant1 { fld0: Move(_23) };
_19.1 = Field::<f32>(Variant(_31, 1), 0) as u128;
RET = !_38;
place!(Field::<([i8; 1], u128, bool)>(Variant(_21, 0), 3)).0 = _19.0;
_28.fld7 = Adt45::Variant2 { fld0: _22,fld1: 9895_i16 };
place!(Field::<[i8; 1]>(Variant(place!(Field::<Adt42>(Variant(_18, 1), 0)), 1), 2)) = _19.0;
_11 = _37;
Goto(bb15)
}
bb15 = {
Call(_50 = dump_var(5_usize, 37_usize, Move(_37), 16_usize, Move(_16), 17_usize, Move(_17), 43_usize, Move(_43)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_50 = dump_var(5_usize, 10_usize, Move(_10), 9_usize, Move(_9), 30_usize, Move(_30), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_50 = dump_var(5_usize, 39_usize, Move(_39), 40_usize, Move(_40), 41_usize, Move(_41), 22_usize, Move(_22)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_50 = dump_var(5_usize, 8_usize, Move(_8), 5_usize, Move(_5), 6_usize, Move(_6), 36_usize, Move(_36)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [u64; 8],mut _2: bool,mut _3: [u64; 8],mut _4: [char; 6]) -> [i64; 1] {
mir! {
type RET = [i64; 1];
let _5: Adt41;
let _6: Adt52;
let _7: Adt45;
let _8: u16;
let _9: Adt40;
let _10: u64;
let _11: isize;
let _12: isize;
let _13: [i128; 6];
let _14: char;
let _15: f32;
let _16: [u16; 5];
let _17: Adt44;
let _18: Adt44;
let _19: (&'static u8, (u16,));
let _20: u32;
let _21: (&'static u8, usize);
let _22: u128;
let _23: Adt46;
let _24: [bool; 1];
let _25: [usize; 7];
let _26: f32;
let _27: f64;
let _28: [usize; 7];
let _29: [i64; 1];
let _30: i128;
let _31: Adt55;
let _32: f64;
let _33: ();
let _34: ();
{
RET = [(-8125099583448726787_i64)];
RET = [5726770679709478178_i64];
_2 = '\u{2eeef}' == '\u{ea0e7}';
RET = [1508546146473963403_i64];
RET = [(-5488469158490082783_i64)];
_5 = Adt41::Variant2 { fld0: (-134074718076485547208504955083670434571_i128),fld1: _1,fld2: (-9223372036854775808_isize),fld3: RET,fld4: _4,fld5: 808430612_i32,fld6: 2_usize };
RET = [(-8400538348972629308_i64)];
_2 = !true;
place!(Field::<i128>(Variant(_5, 2), 0)) = 30498360380164290122430240008747215733_i128;
_5 = Adt41::Variant0 { fld0: 990278433_i32 };
_6.fld2.0 = (-406126040_i32) as u32;
_6.fld3.0 = [15_i8];
_5 = Adt41::Variant0 { fld0: 1175511893_i32 };
_6.fld3.1 = !284108012607489682888175872973575665483_u128;
_6.fld3.2 = _6.fld3.1 > _6.fld3.1;
_6.fld1 = [14456479931609168430_u64,9753921407548750162_u64,7771210975045033021_u64,12064520730644705642_u64,15401861572582380064_u64,6322974526595614532_u64,10154787170576328469_u64,2131719722424693960_u64];
_6.fld2.0 = !383973769_u32;
RET = [(-1693188247135941147_i64)];
_5 = Adt41::Variant2 { fld0: (-68408696956351687956339620966380900117_i128),fld1: _6.fld1,fld2: 9223372036854775807_isize,fld3: RET,fld4: _4,fld5: (-1817541898_i32),fld6: 6_usize };
RET = [(-2570091723159431856_i64)];
_6.fld2.0 = 4122650791_u32;
_6.fld3.2 = _2;
_6.fld2 = (2739952618_u32,);
_6.fld3.0 = [27_i8];
place!(Field::<isize>(Variant(_5, 2), 2)) = _6.fld3.2 as isize;
_8 = 39636_u16 - 62978_u16;
match _6.fld2.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
2739952618 => bb6,
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
place!(Field::<usize>(Variant(_5, 2), 6)) = 13867150291669808782_u64 as usize;
place!(Field::<isize>(Variant(_5, 2), 2)) = -(-9223372036854775808_isize);
_4 = ['\u{5a8ac}','\u{4993f}','\u{1d819}','\u{edb4f}','\u{727be}','\u{b029}'];
place!(Field::<isize>(Variant(_5, 2), 2)) = 11_isize;
_6.fld2 = (1175216215_u32,);
place!(Field::<[u64; 8]>(Variant(_5, 2), 1)) = [5853471430015335743_u64,12341028396403092010_u64,1329813446265340589_u64,2977361791028858334_u64,7316967317415906712_u64,1790891758094534111_u64,673251610625124499_u64,1089525952522288580_u64];
_6.fld2.0 = _2 as u32;
_6.fld2.0 = _6.fld3.2 as u32;
_11 = 159004581_i32 as isize;
_3 = _1;
_5 = Adt41::Variant0 { fld0: 318820654_i32 };
_6.fld3.0 = [(-126_i8)];
_3 = [3960488987910295204_u64,16197689996746086790_u64,9180500521399405787_u64,7691224469171621392_u64,12069084834956733493_u64,3944199986515364541_u64,2532887713797777593_u64,2644531677308642698_u64];
place!(Field::<i32>(Variant(_5, 0), 0)) = _2 as i32;
SetDiscriminant(_5, 2);
place!(Field::<i32>(Variant(_5, 2), 5)) = _6.fld3.2 as i32;
RET = [(-485796181925812292_i64)];
_1 = _6.fld1;
_13 = [270677212666754632488907867768820007_i128,7041957881523210803951792588570867406_i128,141888223757013525133996267505842950343_i128,9540053450649545170746662973705243853_i128,150656444331939924786671126311986578944_i128,16787889398811864149941875674266309569_i128];
place!(Field::<usize>(Variant(_5, 2), 6)) = 4199398153283989873_usize << _8;
place!(Field::<i128>(Variant(_5, 2), 0)) = (-149927056984095069391777256171756998684_i128) & 6012906219470113365548782449207726713_i128;
_10 = _11 as u64;
place!(Field::<[i64; 1]>(Variant(_5, 2), 3)) = RET;
RET = Field::<[i64; 1]>(Variant(_5, 2), 3);
_1 = [_10,_10,_10,_10,_10,_10,_10,_10];
_4 = ['\u{b98a5}','\u{817eb}','\u{dc3bc}','\u{d8d40}','\u{8cf00}','\u{1b5f8}'];
_4 = ['\u{e99da}','\u{80e29}','\u{70f18}','\u{4b274}','\u{2b51a}','\u{f9c82}'];
_6.fld2 = (1396016791_u32,);
Call(place!(Field::<[i64; 1]>(Variant(_5, 2), 3)) = fn7(_6.fld1, Field::<usize>(Variant(_5, 2), 6), _2, RET, _3, _4, RET, Field::<usize>(Variant(_5, 2), 6), _2, _3, _3, _10, _6.fld3.2, _6.fld1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_3 = _6.fld1;
place!(Field::<[i64; 1]>(Variant(_5, 2), 3)) = [8565951182354370713_i64];
_5 = Adt41::Variant2 { fld0: 40752891434516330826221447000742266239_i128,fld1: _6.fld1,fld2: _11,fld3: RET,fld4: _4,fld5: 1447960435_i32,fld6: 3_usize };
_11 = !Field::<isize>(Variant(_5, 2), 2);
_7 = Adt45::Variant2 { fld0: (-1475175608_i32),fld1: 9194_i16 };
_16 = [_8,_8,_8,_8,_8];
_6.fld3.0 = [(-121_i8)];
place!(Field::<i32>(Variant(_7, 2), 0)) = 1329559024_i32;
place!(Field::<isize>(Variant(_5, 2), 2)) = _11 ^ _11;
_15 = _6.fld3.1 as f32;
place!(Field::<i16>(Variant(_7, 2), 1)) = _6.fld3.1 as i16;
place!(Field::<i32>(Variant(_7, 2), 0)) = !1440864728_i32;
_12 = _6.fld3.1 as isize;
SetDiscriminant(_7, 0);
Goto(bb8)
}
bb8 = {
place!(Field::<isize>(Variant(_5, 2), 2)) = -_12;
_1 = _6.fld1;
place!(Field::<[i64; 1]>(Variant(_5, 2), 3)) = RET;
place!(Field::<isize>(Variant(_7, 0), 2)) = _11 + Field::<isize>(Variant(_5, 2), 2);
place!(Field::<usize>(Variant(_7, 0), 0)) = 120_i8 as usize;
_14 = '\u{a419}';
place!(Field::<usize>(Variant(_5, 2), 6)) = _10 as usize;
_8 = Field::<isize>(Variant(_7, 0), 2) as u16;
_15 = _6.fld3.1 as f32;
_11 = -Field::<isize>(Variant(_7, 0), 2);
_18 = Adt44::Variant2 { fld0: Field::<usize>(Variant(_5, 2), 6) };
_1 = [_10,_10,_10,_10,_10,_10,_10,_10];
_17 = Adt44::Variant2 { fld0: Field::<usize>(Variant(_5, 2), 6) };
_17 = Move(_18);
place!(Field::<[usize; 7]>(Variant(_7, 0), 3)) = [Field::<usize>(Variant(_17, 2), 0),Field::<usize>(Variant(_17, 2), 0),Field::<usize>(Variant(_7, 0), 0),Field::<usize>(Variant(_7, 0), 0),Field::<usize>(Variant(_17, 2), 0),Field::<usize>(Variant(_17, 2), 0),Field::<usize>(Variant(_7, 0), 0)];
place!(Field::<[char; 6]>(Variant(_5, 2), 4)) = [_14,_14,_14,_14,_14,_14];
place!(Field::<i32>(Variant(_5, 2), 5)) = 1441490363_i32 ^ (-721716055_i32);
_2 = _6.fld3.2;
place!(Field::<usize>(Variant(_7, 0), 0)) = Field::<i32>(Variant(_5, 2), 5) as usize;
place!(Field::<[char; 6]>(Variant(_5, 2), 4)) = [_14,_14,_14,_14,_14,_14];
SetDiscriminant(_17, 3);
_19.1 = (_8,);
place!(Field::<isize>(Variant(_5, 2), 2)) = !Field::<isize>(Variant(_7, 0), 2);
_18 = Adt44::Variant2 { fld0: Field::<usize>(Variant(_5, 2), 6) };
match _6.fld2.0 {
0 => bb4,
1 => bb7,
2 => bb6,
1396016791 => bb10,
_ => bb9
}
}
bb9 = {
Return()
}
bb10 = {
_20 = !_6.fld2.0;
_5 = Adt41::Variant2 { fld0: (-132958333673997496431584117157859369742_i128),fld1: _6.fld1,fld2: Field::<isize>(Variant(_7, 0), 2),fld3: RET,fld4: _4,fld5: (-1150523590_i32),fld6: Field::<usize>(Variant(_18, 2), 0) };
_6.fld3.0 = [(-6_i8)];
place!(Field::<[usize; 7]>(Variant(_7, 0), 3)) = [Field::<usize>(Variant(_7, 0), 0),Field::<usize>(Variant(_7, 0), 0),Field::<usize>(Variant(_5, 2), 6),Field::<usize>(Variant(_7, 0), 0),Field::<usize>(Variant(_7, 0), 0),Field::<usize>(Variant(_7, 0), 0),Field::<usize>(Variant(_18, 2), 0)];
place!(Field::<i32>(Variant(_5, 2), 5)) = 1088628268_i32 + 1001808138_i32;
_8 = _19.1.0;
place!(Field::<[u64; 8]>(Variant(_5, 2), 1)) = _3;
_6.fld1 = _3;
SetDiscriminant(_18, 0);
place!(Field::<[u64; 8]>(Variant(_5, 2), 1)) = _6.fld1;
place!(Field::<[char; 6]>(Variant(_5, 2), 4)) = [_14,_14,_14,_14,_14,_14];
_27 = _15 as f64;
_27 = _15 as f64;
_26 = -_15;
_21.1 = Field::<usize>(Variant(_5, 2), 6) * Field::<usize>(Variant(_5, 2), 6);
place!(Field::<(bool,)>(Variant(_17, 3), 0)) = (_2,);
Goto(bb11)
}
bb11 = {
place!(Field::<*const f32>(Variant(_18, 0), 4)) = core::ptr::addr_of!(_26);
_22 = _27 as u128;
_22 = _6.fld3.1;
SetDiscriminant(_17, 0);
place!(Field::<(u32,)>(Variant(_17, 0), 5)).0 = !_6.fld2.0;
_6.fld3.0 = [(-107_i8)];
place!(Field::<i8>(Variant(_18, 0), 3)) = (-121_i8) * (-109_i8);
_21.1 = Field::<usize>(Variant(_7, 0), 0) << Field::<i8>(Variant(_18, 0), 3);
place!(Field::<(u32,)>(Variant(_18, 0), 5)) = (_6.fld2.0,);
_22 = _6.fld3.1;
match Field::<(u32,)>(Variant(_18, 0), 5).0 {
0 => bb1,
1 => bb5,
2 => bb8,
3 => bb4,
4 => bb12,
5 => bb13,
6 => bb14,
1396016791 => bb16,
_ => bb15
}
}
bb12 = {
_20 = !_6.fld2.0;
_5 = Adt41::Variant2 { fld0: (-132958333673997496431584117157859369742_i128),fld1: _6.fld1,fld2: Field::<isize>(Variant(_7, 0), 2),fld3: RET,fld4: _4,fld5: (-1150523590_i32),fld6: Field::<usize>(Variant(_18, 2), 0) };
_6.fld3.0 = [(-6_i8)];
place!(Field::<[usize; 7]>(Variant(_7, 0), 3)) = [Field::<usize>(Variant(_7, 0), 0),Field::<usize>(Variant(_7, 0), 0),Field::<usize>(Variant(_5, 2), 6),Field::<usize>(Variant(_7, 0), 0),Field::<usize>(Variant(_7, 0), 0),Field::<usize>(Variant(_7, 0), 0),Field::<usize>(Variant(_18, 2), 0)];
place!(Field::<i32>(Variant(_5, 2), 5)) = 1088628268_i32 + 1001808138_i32;
_8 = _19.1.0;
place!(Field::<[u64; 8]>(Variant(_5, 2), 1)) = _3;
_6.fld1 = _3;
SetDiscriminant(_18, 0);
place!(Field::<[u64; 8]>(Variant(_5, 2), 1)) = _6.fld1;
place!(Field::<[char; 6]>(Variant(_5, 2), 4)) = [_14,_14,_14,_14,_14,_14];
_27 = _15 as f64;
_27 = _15 as f64;
_26 = -_15;
_21.1 = Field::<usize>(Variant(_5, 2), 6) * Field::<usize>(Variant(_5, 2), 6);
place!(Field::<(bool,)>(Variant(_17, 3), 0)) = (_2,);
Goto(bb11)
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
place!(Field::<*const f32>(Variant(_17, 0), 4)) = core::ptr::addr_of!(_15);
_12 = Field::<isize>(Variant(_5, 2), 2);
place!(Field::<([i8; 1], u128, bool)>(Variant(_18, 0), 6)).2 = !_6.fld3.2;
_21.1 = _22 as usize;
place!(Field::<Adt41>(Variant(_17, 0), 7)) = Adt41::Variant2 { fld0: 152283505034331027058597397701227005161_i128,fld1: _6.fld1,fld2: _11,fld3: Field::<[i64; 1]>(Variant(_5, 2), 3),fld4: _4,fld5: Field::<i32>(Variant(_5, 2), 5),fld6: Field::<usize>(Variant(_5, 2), 6) };
Goto(bb17)
}
bb17 = {
Call(_33 = dump_var(6_usize, 8_usize, Move(_8), 4_usize, Move(_4), 3_usize, Move(_3), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_33 = dump_var(6_usize, 20_usize, Move(_20), 10_usize, Move(_10), 34_usize, _34, 34_usize, _34), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: [u64; 8],mut _2: usize,mut _3: bool,mut _4: [i64; 1],mut _5: [u64; 8],mut _6: [char; 6],mut _7: [i64; 1],mut _8: usize,mut _9: bool,mut _10: [u64; 8],mut _11: [u64; 8],mut _12: u64,mut _13: bool,mut _14: [u64; 8]) -> [i64; 1] {
mir! {
type RET = [i64; 1];
let _15: Adt52;
let _16: &'static u8;
let _17: (&'static u8, (u16,));
let _18: [i128; 6];
let _19: bool;
let _20: &'static u8;
let _21: Adt40;
let _22: f32;
let _23: Adt40;
let _24: [usize; 7];
let _25: Adt41;
let _26: Adt41;
let _27: u8;
let _28: char;
let _29: isize;
let _30: i64;
let _31: Adt54;
let _32: [usize; 5];
let _33: u16;
let _34: Adt41;
let _35: u128;
let _36: Adt51;
let _37: Adt39;
let _38: isize;
let _39: Adt55;
let _40: u8;
let _41: ();
let _42: ();
{
RET = _7;
_5 = _1;
RET = [4909519342134113289_i64];
_4 = [(-707661801411381382_i64)];
_14 = _11;
RET = [708679799688082491_i64];
_15.fld2.0 = 1898293282_i32 as u32;
_15.fld1 = [_12,_12,_12,_12,_12,_12,_12,_12];
_12 = 14298769394215867458_u64 - 7486225131959271349_u64;
_15.fld2.0 = !3393786772_u32;
_5 = [_12,_12,_12,_12,_12,_12,_12,_12];
_15.fld0 = !_9;
_17.1.0 = !56825_u16;
_13 = !_9;
_10 = [_12,_12,_12,_12,_12,_12,_12,_12];
_15.fld3.1 = !27675145939662630668301211119774529425_u128;
_15.fld3.1 = 317684725403402601027762026515397057434_u128 * 278638441930598655299581103971572427540_u128;
_18 = [58388091058766595905891159851682865945_i128,(-101276581548850489583134804483727510936_i128),(-27361411746402786999113434136804877735_i128),132654418410734811772142627662386467305_i128,(-152761067444321333494313235400718100552_i128),166985251362227019595199861093662430470_i128];
_2 = _8 << _8;
RET = _4;
_15.fld2.0 = _17.1.0 as u32;
_8 = !_2;
_4 = RET;
_15.fld3.0 = [109_i8];
_15.fld3.2 = _13;
Goto(bb1)
}
bb1 = {
_13 = !_3;
_19 = !_15.fld0;
_15.fld2 = (799687771_u32,);
_12 = !3809485403035048210_u64;
_2 = _8 - _8;
_10 = [_12,_12,_12,_12,_12,_12,_12,_12];
_14 = _15.fld1;
_12 = !12428937209206287649_u64;
match _15.fld2.0 {
0 => bb2,
799687771 => bb4,
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
_15.fld0 = _3;
_5 = _1;
_15.fld2.0 = 2592510057_u32 & 2184898636_u32;
_15.fld2.0 = 1459043393_u32 - 3856856554_u32;
_15.fld3.1 = !279696124734459004994385971315865873641_u128;
RET = _7;
_5 = _1;
_8 = _2 + _2;
_15.fld2.0 = !3503506264_u32;
_18 = [(-114156964628342703769685274962469896750_i128),78005215052263122441187835446276281853_i128,107132518189473464369258181509550137502_i128,123559648480744326387942084853921922731_i128,49749599510160507242517504577275819089_i128,132241081639326917969765064878686919387_i128];
_2 = _3 as usize;
RET = [(-4165651059091006904_i64)];
_12 = 3177817412998622797_u64 >> _8;
Goto(bb5)
}
bb5 = {
_15.fld3.2 = _3;
_17.1.0 = !53263_u16;
_5 = [_12,_12,_12,_12,_12,_12,_12,_12];
_22 = 108_u8 as f32;
RET = [(-1288189705508041693_i64)];
_17.1.0 = 49664_u16 | 48815_u16;
_18 = [(-166841850069079776743657294322986161268_i128),(-19571182117680297350198088086435706996_i128),(-24299209387106615473257913848940915645_i128),79042632280166975661762447615778305583_i128,148133221976128613432886316832915316898_i128,94779858520061234720401108511090623733_i128];
_2 = _8;
_15.fld1 = _11;
_27 = 76_u8;
RET = [(-4264387679740574266_i64)];
_14 = [_12,_12,_12,_12,_12,_12,_12,_12];
_15.fld0 = _3;
_5 = [_12,_12,_12,_12,_12,_12,_12,_12];
_17.0 = &_27;
_7 = [(-2525244296888698590_i64)];
_15.fld3.2 = !_15.fld0;
_3 = !_15.fld0;
_2 = !_8;
_13 = _12 == _12;
_20 = &_27;
match _27 {
0 => bb1,
1 => bb4,
76 => bb7,
_ => bb6
}
}
bb6 = {
Return()
}
bb7 = {
_29 = (-105_isize);
_3 = _13;
_17.1.0 = 13712_u16 | 16252_u16;
_26 = Adt41::Variant2 { fld0: (-10746963032856880792627120409362624493_i128),fld1: _5,fld2: _29,fld3: _7,fld4: _6,fld5: (-519608564_i32),fld6: _2 };
_31.fld4.fld2.0 = _15.fld2.0 >> _2;
_20 = Move(_17.0);
place!(Field::<isize>(Variant(_26, 2), 2)) = _29;
_32 = [Field::<usize>(Variant(_26, 2), 6),_8,Field::<usize>(Variant(_26, 2), 6),_2,_2];
_10 = [_12,_12,_12,_12,_12,_12,_12,_12];
_31.fld4.fld3.1 = _15.fld3.1;
_12 = 10637637329237507457_u64;
_31.fld0 = !_3;
_20 = &(*_20);
_33 = _17.1.0;
_9 = _31.fld0;
_8 = Field::<usize>(Variant(_26, 2), 6) << Field::<usize>(Variant(_26, 2), 6);
_31.fld4.fld3 = (_15.fld3.0, _15.fld3.1, _13);
_15.fld2.0 = !_31.fld4.fld2.0;
_17.1.0 = _33;
_28 = '\u{42747}';
Goto(bb8)
}
bb8 = {
_28 = '\u{101cc7}';
_26 = Adt41::Variant0 { fld0: 1412752225_i32 };
match (*_20) {
0 => bb5,
1 => bb2,
2 => bb9,
3 => bb10,
4 => bb11,
5 => bb12,
76 => bb14,
_ => bb13
}
}
bb9 = {
_29 = (-105_isize);
_3 = _13;
_17.1.0 = 13712_u16 | 16252_u16;
_26 = Adt41::Variant2 { fld0: (-10746963032856880792627120409362624493_i128),fld1: _5,fld2: _29,fld3: _7,fld4: _6,fld5: (-519608564_i32),fld6: _2 };
_31.fld4.fld2.0 = _15.fld2.0 >> _2;
_20 = Move(_17.0);
place!(Field::<isize>(Variant(_26, 2), 2)) = _29;
_32 = [Field::<usize>(Variant(_26, 2), 6),_8,Field::<usize>(Variant(_26, 2), 6),_2,_2];
_10 = [_12,_12,_12,_12,_12,_12,_12,_12];
_31.fld4.fld3.1 = _15.fld3.1;
_12 = 10637637329237507457_u64;
_31.fld0 = !_3;
_20 = &(*_20);
_33 = _17.1.0;
_9 = _31.fld0;
_8 = Field::<usize>(Variant(_26, 2), 6) << Field::<usize>(Variant(_26, 2), 6);
_31.fld4.fld3 = (_15.fld3.0, _15.fld3.1, _13);
_15.fld2.0 = !_31.fld4.fld2.0;
_17.1.0 = _33;
_28 = '\u{42747}';
Goto(bb8)
}
bb10 = {
Return()
}
bb11 = {
_15.fld3.2 = _3;
_17.1.0 = !53263_u16;
_5 = [_12,_12,_12,_12,_12,_12,_12,_12];
_22 = 108_u8 as f32;
RET = [(-1288189705508041693_i64)];
_17.1.0 = 49664_u16 | 48815_u16;
_18 = [(-166841850069079776743657294322986161268_i128),(-19571182117680297350198088086435706996_i128),(-24299209387106615473257913848940915645_i128),79042632280166975661762447615778305583_i128,148133221976128613432886316832915316898_i128,94779858520061234720401108511090623733_i128];
_2 = _8;
_15.fld1 = _11;
_27 = 76_u8;
RET = [(-4264387679740574266_i64)];
_14 = [_12,_12,_12,_12,_12,_12,_12,_12];
_15.fld0 = _3;
_5 = [_12,_12,_12,_12,_12,_12,_12,_12];
_17.0 = &_27;
_7 = [(-2525244296888698590_i64)];
_15.fld3.2 = !_15.fld0;
_3 = !_15.fld0;
_2 = !_8;
_13 = _12 == _12;
_20 = &_27;
match _27 {
0 => bb1,
1 => bb4,
76 => bb7,
_ => bb6
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_15.fld1 = [_12,_12,_12,_12,_12,_12,_12,_12];
_31.fld4.fld0 = _31.fld4.fld3.2;
_17.1.0 = _33 - _33;
_31.fld4.fld3.1 = _15.fld3.1 ^ _15.fld3.1;
_12 = 14396780126342924598_u64;
_8 = _2;
_12 = 16748581820767949348_u64;
_24 = [_8,_8,_8,_8,_8,_2,_8];
_36.fld6 = _29 as u16;
_7 = [(-2669117986295121859_i64)];
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(7_usize, 4_usize, Move(_4), 18_usize, Move(_18), 6_usize, Move(_6), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(7_usize, 9_usize, Move(_9), 7_usize, Move(_7), 1_usize, Move(_1), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(7_usize, 12_usize, Move(_12), 29_usize, Move(_29), 19_usize, Move(_19), 42_usize, _42), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: [u64; 8],mut _2: u32,mut _3: isize,mut _4: u128,mut _5: Adt48,mut _6: u16,mut _7: [u64; 8],mut _8: [bool; 1],mut _9: u8,mut _10: bool,mut _11: char,mut _12: bool,mut _13: char,mut _14: u16,mut _15: [bool; 1]) -> *const f32 {
mir! {
type RET = *const f32;
let _16: char;
let _17: u64;
let _18: [char; 6];
let _19: Adt54;
let _20: Adt42;
let _21: [i128; 6];
let _22: [u64; 8];
let _23: i8;
let _24: f32;
let _25: i8;
let _26: u64;
let _27: bool;
let _28: [i128; 6];
let _29: char;
let _30: i8;
let _31: ();
let _32: ();
{
_4 = 89124352161594622168534343162052366567_u128;
_7 = [2100636608134918801_u64,5302715780166767555_u64,358432099065587212_u64,7529487155310105504_u64,15204311739528085681_u64,12678394064250982510_u64,760451133672269774_u64,15973799410432714914_u64];
SetDiscriminant(_5, 1);
_12 = _10;
_6 = _14;
match _9 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
150 => bb8,
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
_2 = 3996355938_u32;
_13 = _11;
Goto(bb9)
}
bb9 = {
_15 = [_12];
_14 = !_6;
_10 = _6 < _14;
_1 = [7921279084016745480_u64,11933645389766228506_u64,1496125983035688842_u64,15061353672750834412_u64,8172648861841129221_u64,14028622720445415645_u64,12052315480435311819_u64,12863193333542616373_u64];
_14 = _9 as u16;
_14 = _2 as u16;
Goto(bb10)
}
bb10 = {
_16 = _13;
_9 = 59_i8 as u8;
_7 = [1092110514675598285_u64,17582650771250154593_u64,1054975228877084683_u64,8395053001617286085_u64,10326268222106081188_u64,9575963930800281108_u64,5684708958416404405_u64,3094864402744725384_u64];
_15 = _8;
_12 = _10 <= _10;
_19.fld0 = _12 < _10;
_6 = _14 << _9;
_7 = [10808732379030258278_u64,6562973343533258919_u64,14050233406189289469_u64,13410016822833585962_u64,13607594653913864024_u64,736748009105895894_u64,3347356678165304287_u64,7097657858288612425_u64];
_1 = [8643892288697195510_u64,6518717324251938524_u64,6896923189536552763_u64,4119227791126634752_u64,2325872602150211582_u64,17140046489191850841_u64,13392212507299831420_u64,10209532487161934093_u64];
_12 = _10 ^ _19.fld0;
_14 = _6;
_16 = _11;
_19.fld4.fld0 = _12 & _19.fld0;
_18 = [_11,_13,_16,_11,_16,_13];
Goto(bb11)
}
bb11 = {
_18 = [_11,_11,_16,_16,_16,_16];
_1 = _7;
_14 = _6 >> _4;
_9 = 6966800908681813975_usize as u8;
_19.fld4.fld3.0 = [26_i8];
_14 = _6;
_18 = [_11,_16,_16,_11,_13,_16];
_19.fld4.fld1 = [10916302537223234008_u64,9679002242374650856_u64,7014012188931333268_u64,11737066508330042049_u64,6099268901005421670_u64,15311025230984100865_u64,358424814259519211_u64,5251619420569260659_u64];
_8 = [_19.fld4.fld0];
_3 = -9223372036854775807_isize;
_19.fld4.fld3.2 = _12;
_19.fld4.fld2 = (_2,);
_21 = [(-123218655264208843004523365545212134430_i128),(-48315074214967238612769951362429232731_i128),60952686409249518336382694878007456773_i128,38552126027432177251184713693823057550_i128,18350810427038871682780444350996723740_i128,(-115143560361257474433490767739145494725_i128)];
_13 = _11;
_19.fld4.fld0 = !_19.fld4.fld3.2;
_6 = (-808301330_i32) as u16;
_17 = !14979465114213283607_u64;
_18 = [_16,_13,_13,_11,_13,_13];
_19.fld4.fld3.2 = _12;
_14 = _9 as u16;
match _2 {
3996355938 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_21 = [10768114403535668486204118560562503964_i128,(-21710157563227639200494169680765306041_i128),143398549653591496260846139752192926773_i128,(-108004841574262198962359048576090068440_i128),21723561296783295728158120108708742289_i128,(-25667358898226373935890509956755712111_i128)];
_1 = [_17,_17,_17,_17,_17,_17,_17,_17];
_13 = _11;
_19.fld4.fld2 = (_2,);
_4 = 217105009367545189767259815121580204590_u128 >> _3;
_4 = 128114288705831944265253810095634076297_u128 - 133439888932143649905146921220842237346_u128;
_19.fld4.fld0 = !_12;
_2 = _19.fld4.fld2.0 % _19.fld4.fld2.0;
_14 = _6 - _6;
_9 = 142_u8;
_9 = 235_u8;
Call(_19.fld4 = fn9(_8, _7, _4, _19.fld0, _9, _12, _12, _21, _12, _10, _21, _8, _12, _12, _19.fld0), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_3 = 9223372036854775807_isize + (-9223372036854775808_isize);
_19.fld4.fld1 = [_17,_17,_17,_17,_17,_17,_17,_17];
_3 = (-9223372036854775808_isize) + (-116_isize);
_8 = [_19.fld4.fld3.2];
_1 = _7;
_12 = _19.fld0;
_2 = _17 as u32;
_15 = _8;
_3 = (-9223372036854775808_isize);
_2 = (-78_i8) as u32;
_24 = _4 as f32;
_26 = _6 as u64;
_16 = _13;
_25 = 106_i8;
_14 = _2 as u16;
_27 = _19.fld4.fld0;
_10 = _19.fld4.fld0 == _12;
_12 = _19.fld4.fld3.2 != _19.fld4.fld3.2;
_5 = Adt48::Variant2 { fld0: (-3586143151816442954_i64),fld1: _16 };
_26 = _17;
_12 = _10;
match _25 {
0 => bb1,
1 => bb6,
106 => bb16,
_ => bb15
}
}
bb15 = {
Return()
}
bb16 = {
_23 = _6 as i8;
_3 = 9223372036854775807_isize << _6;
_19.fld0 = _12 <= _19.fld4.fld3.2;
_12 = _10 | _19.fld0;
place!(Field::<i64>(Variant(_5, 2), 0)) = (-1373243147296480427_i64) | (-2160189957849583530_i64);
_8 = [_19.fld4.fld3.2];
_7 = _1;
Goto(bb17)
}
bb17 = {
RET = core::ptr::addr_of!(_24);
Goto(bb18)
}
bb18 = {
Call(_31 = dump_var(8_usize, 18_usize, Move(_18), 14_usize, Move(_14), 6_usize, Move(_6), 8_usize, Move(_8)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_31 = dump_var(8_usize, 10_usize, Move(_10), 26_usize, Move(_26), 7_usize, Move(_7), 17_usize, Move(_17)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_31 = dump_var(8_usize, 13_usize, Move(_13), 11_usize, Move(_11), 27_usize, Move(_27), 32_usize, _32), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: [bool; 1],mut _2: [u64; 8],mut _3: u128,mut _4: bool,mut _5: u8,mut _6: bool,mut _7: bool,mut _8: [i128; 6],mut _9: bool,mut _10: bool,mut _11: [i128; 6],mut _12: [bool; 1],mut _13: bool,mut _14: bool,mut _15: bool) -> Adt52 {
mir! {
type RET = Adt52;
let _16: i8;
let _17: u64;
let _18: f32;
let _19: *const f32;
let _20: ();
let _21: ();
{
RET.fld0 = !_7;
RET.fld1 = [10999397464855835713_u64,13587424827057832283_u64,3300359007498311189_u64,14693564457524788844_u64,1019193288472656407_u64,6941898284948004208_u64,11397932911207084886_u64,11600304108207336035_u64];
RET.fld2 = (1582242037_u32,);
RET.fld3.2 = _4 | _6;
_6 = _4 & RET.fld3.2;
Goto(bb1)
}
bb1 = {
RET.fld3.1 = _3;
RET.fld3.1 = _3;
RET.fld3.0 = [(-44_i8)];
_13 = _14 >= RET.fld3.2;
_6 = !_14;
_5 = 41_u8;
RET.fld1 = [10695032091940761514_u64,682443516926328498_u64,5925088991491072383_u64,4958687096953714918_u64,10252610488752584837_u64,13497163460244514537_u64,12884775680816701461_u64,10686672998095164051_u64];
_15 = _7 != _13;
_6 = !RET.fld3.2;
RET.fld3.0 = [(-7_i8)];
_5 = (-29940_i16) as u8;
RET.fld2.0 = 3934773196_u32 >> _3;
_5 = '\u{27726}' as u8;
RET.fld2.0 = 2544540959_u32;
_13 = RET.fld3.2 >= _10;
_16 = (-111_i8);
_6 = RET.fld0 | _9;
RET.fld3.0 = [_16];
RET.fld1 = [4713951214098848806_u64,14635858250844054699_u64,900982529279077050_u64,7100146023702681554_u64,5041944434013099732_u64,18151652348228959871_u64,5673553203771382183_u64,500063608331631417_u64];
_18 = 2_usize as f32;
_6 = _13;
_17 = _16 as u64;
_19 = core::ptr::addr_of!(_18);
Goto(bb2)
}
bb2 = {
Call(_20 = dump_var(9_usize, 1_usize, Move(_1), 4_usize, Move(_4), 3_usize, Move(_3), 5_usize, Move(_5)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_20 = dump_var(9_usize, 11_usize, Move(_11), 17_usize, Move(_17), 14_usize, Move(_14), 9_usize, Move(_9)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: isize,mut _2: char,mut _3: bool,mut _4: i8,mut _5: [i64; 1],mut _6: isize,mut _7: u16,mut _8: [u64; 8],mut _9: ([i8; 1], u128, bool),mut _10: bool,mut _11: char,mut _12: i32,mut _13: u16,mut _14: u16) -> [i128; 6] {
mir! {
type RET = [i128; 6];
let _15: u32;
let _16: [isize; 8];
let _17: bool;
let _18: [i8; 1];
let _19: [u64; 8];
let _20: Adt42;
let _21: f32;
let _22: char;
let _23: u128;
let _24: u32;
let _25: [i64; 1];
let _26: Adt41;
let _27: f32;
let _28: bool;
let _29: Adt49;
let _30: isize;
let _31: f32;
let _32: Adt41;
let _33: *mut (bool,);
let _34: Adt43;
let _35: Adt41;
let _36: i8;
let _37: i64;
let _38: isize;
let _39: char;
let _40: Adt55;
let _41: Adt45;
let _42: [i64; 1];
let _43: ();
let _44: ();
{
RET = [67644151450539649151280760794272272294_i128,(-8977497116191770326037240773674035174_i128),(-79776395456639553365461312805142041069_i128),(-887191245599994081541913787222695353_i128),(-157388109547848763486743019575364929552_i128),(-91407832148908891569426323493577123899_i128)];
_1 = -_6;
_15 = _13 as u32;
_3 = _9.2 < _10;
_8 = [13550354197945982673_u64,15915247763980108923_u64,4256987665592195452_u64,6793176798311755867_u64,4732972184270050433_u64,13569778569532237899_u64,8014665508433114695_u64,14094389067792697343_u64];
_13 = !_7;
RET = [157455685999173146707297205501098146521_i128,69690278326246704559602845554680538322_i128,(-38205857744620598303423633731532440471_i128),(-16401286834826008985179983286520610635_i128),148239113757555613669759439267653418234_i128,49949166577384785195915276081270909549_i128];
_15 = 3870005134_u32 << _14;
_4 = 4_i8 + (-53_i8);
_17 = _9.2;
_2 = _11;
Call(_13 = fn11(_9.2, _6, _9.0, _8, _9.0, RET, _9.2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = _15 as i8;
_6 = _1 | _1;
_17 = _9.2;
_14 = (-17744_i16) as u16;
_9.2 = !_3;
_9.1 = 145107465673119747384893515184316426176_u128 ^ 216751754685230419432160417558664770992_u128;
_15 = 1612060455_u32 >> _6;
_15 = !763843171_u32;
_8 = [9343855313030400639_u64,3372501190417861090_u64,15024362654759872402_u64,1138882378600196890_u64,12864664229249234405_u64,11674960194363803560_u64,5868602278607004755_u64,10462722101251073687_u64];
_5 = [(-4891735137155335146_i64)];
RET = [(-65319642942295850087081454523726869609_i128),(-145924065530886828681399682642101584113_i128),(-47274121361816997859616913055988809800_i128),(-116034373452303315233014050618969689188_i128),(-22124908128170338776409513795335680164_i128),131099305346385303835009454606023103111_i128];
_1 = 8812761654000678475_i64 as isize;
_4 = 46_i8;
_8 = [10195920773210236706_u64,5579645688618973146_u64,8380793449916173691_u64,17218106354727979019_u64,313804516651033322_u64,8625889645521212656_u64,12665721906178638104_u64,2548197222458268439_u64];
_11 = _2;
_9.1 = 263914876862156341821170220536750066587_u128 ^ 36723822929278072187114437509687029551_u128;
RET = [(-32101049532806841626113902111302091578_i128),20356688133919835479852668346609107447_i128,145109693364614422743939767288747340224_i128,9609059553536915807833922068534834919_i128,(-169799726850482491003213780799904978224_i128),(-147848222315971846849337586008119743785_i128)];
_16 = [_6,_1,_6,_6,_6,_6,_6,_6];
_21 = _6 as f32;
_5 = [7566910956060330935_i64];
_17 = _21 > _21;
Call(_15 = core::intrinsics::transmute(_12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9.1 = 109531750793469738725488980312794201928_i128 as u128;
_3 = _6 <= _6;
_19 = _8;
_11 = _2;
_9.2 = !_10;
_8 = [14887135028345175168_u64,9366091109863510671_u64,14144763471680777589_u64,10270658708469706720_u64,6270018911951485706_u64,15966010083623042255_u64,5892187553196781127_u64,11362834964224717829_u64];
_2 = _11;
_6 = _1 ^ _1;
_3 = _17;
_18 = [_4];
_9.2 = !_10;
_23 = _9.1;
RET = [150852137416352801751765628176151225696_i128,58086897484001122537474528002796277861_i128,(-150318199331824964583130242340464142396_i128),62589498791303890918653664174632206140_i128,133865235506416467826208259728815976931_i128,10477563594632177557117604843373493244_i128];
_13 = 62_u8 as u16;
_21 = _4 as f32;
_21 = 4_usize as f32;
_5 = [3782772446474829534_i64];
_3 = !_10;
_12 = _4 as i32;
_4 = (-85_i8);
_10 = _3 > _17;
_3 = _17;
_19 = [220964240253941822_u64,9347129553054525219_u64,14353296283586452243_u64,15544855498329082471_u64,6069099364129671362_u64,11424659870448222838_u64,18113546719327255701_u64,8839160566735757809_u64];
match _4 {
0 => bb3,
340282366920938463463374607431768211371 => bb5,
_ => bb4
}
}
bb3 = {
_4 = _15 as i8;
_6 = _1 | _1;
_17 = _9.2;
_14 = (-17744_i16) as u16;
_9.2 = !_3;
_9.1 = 145107465673119747384893515184316426176_u128 ^ 216751754685230419432160417558664770992_u128;
_15 = 1612060455_u32 >> _6;
_15 = !763843171_u32;
_8 = [9343855313030400639_u64,3372501190417861090_u64,15024362654759872402_u64,1138882378600196890_u64,12864664229249234405_u64,11674960194363803560_u64,5868602278607004755_u64,10462722101251073687_u64];
_5 = [(-4891735137155335146_i64)];
RET = [(-65319642942295850087081454523726869609_i128),(-145924065530886828681399682642101584113_i128),(-47274121361816997859616913055988809800_i128),(-116034373452303315233014050618969689188_i128),(-22124908128170338776409513795335680164_i128),131099305346385303835009454606023103111_i128];
_1 = 8812761654000678475_i64 as isize;
_4 = 46_i8;
_8 = [10195920773210236706_u64,5579645688618973146_u64,8380793449916173691_u64,17218106354727979019_u64,313804516651033322_u64,8625889645521212656_u64,12665721906178638104_u64,2548197222458268439_u64];
_11 = _2;
_9.1 = 263914876862156341821170220536750066587_u128 ^ 36723822929278072187114437509687029551_u128;
RET = [(-32101049532806841626113902111302091578_i128),20356688133919835479852668346609107447_i128,145109693364614422743939767288747340224_i128,9609059553536915807833922068534834919_i128,(-169799726850482491003213780799904978224_i128),(-147848222315971846849337586008119743785_i128)];
_16 = [_6,_1,_6,_6,_6,_6,_6,_6];
_21 = _6 as f32;
_5 = [7566910956060330935_i64];
_17 = _21 > _21;
Call(_15 = core::intrinsics::transmute(_12), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
_11 = _2;
RET = [40178821260887824387365478365996180282_i128,48657308677322897768914295904036741985_i128,(-117760671617155363556215056003448362087_i128),53867401404375594514052302354100448679_i128,(-86564710437356341106234661000955223319_i128),106216857548892235545759604758437272359_i128];
_6 = _1 * _1;
_7 = _9.1 as u16;
_12 = !(-1931829428_i32);
_19 = [3343518498359739902_u64,3470234818703008074_u64,16026234262511783658_u64,16189320666400006255_u64,10593746683130157596_u64,2588624514032069927_u64,12755825801167040738_u64,12351621719741635981_u64];
_11 = _2;
_14 = !_13;
_3 = _10 > _10;
_12 = (-590383745066620282_i64) as i32;
_16 = [_1,_6,_6,_6,_6,_6,_1,_6];
Goto(bb6)
}
bb6 = {
_17 = _3;
RET = [(-59817940286057783454500694311138281687_i128),(-112738200275238228155747064606429532644_i128),39784004851516689961502768996232001398_i128,(-88278134880291655741251708714760798692_i128),69308768518411432075571599096466415953_i128,87194400117635718114937878475140685165_i128];
_22 = _11;
_2 = _11;
_9 = (_18, _23, _17);
_21 = _12 as f32;
match _4 {
0 => bb1,
1 => bb2,
2 => bb5,
340282366920938463463374607431768211371 => bb7,
_ => bb4
}
}
bb7 = {
_11 = _22;
_17 = _10 != _3;
_27 = _21;
_9.1 = !_23;
_19 = [16019069727775536635_u64,12404850724691200219_u64,17163175719659341543_u64,9106981740719259008_u64,14508544472550153942_u64,16124813202920862686_u64,10693000714642767409_u64,8775019790656538347_u64];
_21 = _27 + _27;
_30 = !_6;
_25 = _5;
_12 = 1235250694_i32;
_12 = (-21978464168888605345373267093225902889_i128) as i32;
_9.1 = _22 as u128;
Goto(bb8)
}
bb8 = {
_7 = _14;
_6 = !_30;
_28 = _9.2;
_23 = _9.1;
_24 = _15 ^ _15;
_17 = _9.2 < _10;
_1 = _12 as isize;
_1 = _30 - _6;
_1 = 352550393815792417_u64 as isize;
_24 = !_15;
_9.1 = !_23;
_10 = _3;
_8 = _19;
_18 = _9.0;
_10 = !_3;
match _4 {
340282366920938463463374607431768211371 => bb9,
_ => bb5
}
}
bb9 = {
_9.2 = !_28;
_4 = 34_i8 & (-91_i8);
_26 = Adt41::Variant0 { fld0: _12 };
Goto(bb10)
}
bb10 = {
_32 = Move(_26);
_22 = _2;
_9.0 = [_4];
_6 = -_30;
_22 = _2;
_27 = (-105840833548441800387884003990976501766_i128) as f32;
_26 = Adt41::Variant0 { fld0: _12 };
_17 = !_9.2;
_22 = _11;
_30 = -_1;
_22 = _2;
_26 = Move(_32);
Goto(bb11)
}
bb11 = {
_10 = _28 > _9.2;
RET = [79709173008545694586057594683551488462_i128,(-1669994824311017578710690216302758652_i128),137293024621909785945271485319415700245_i128,(-14740166426531957901900677077399950209_i128),(-80645947607139777244964840707270777663_i128),140515201661620933258592838933669749690_i128];
_15 = !_24;
_24 = !_15;
_9.0 = [_4];
_3 = _28;
_28 = _10;
_7 = _13 + _13;
_5 = [8321001690873901399_i64];
_5 = _25;
_7 = !_14;
place!(Field::<i32>(Variant(_26, 0), 0)) = _12 & _12;
place!(Field::<i32>(Variant(_26, 0), 0)) = _12 >> _7;
_36 = _4;
_6 = 8637158959329905142_u64 as isize;
_25 = [1490518292994756966_i64];
_32 = Adt41::Variant0 { fld0: _12 };
_9.1 = !_23;
_26 = Move(_32);
Call(_3 = fn18(_9, _9.2, _10, _17, _10, _8, _28, _9, _28, _28, _10, _28), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_27 = _21;
_1 = _30;
_36 = _27 as i8;
_11 = _22;
_5 = [(-9161922834059032748_i64)];
_3 = !_10;
_32 = Move(_26);
RET = [(-33487702190604588695467343001426433438_i128),(-68452552506985789411185575377049103066_i128),(-36530267293372898098249205171957691124_i128),168690567947514935324375468567131301662_i128,63043367085317631373635586389642132839_i128,78851722391460882388479379991085123555_i128];
_31 = -_27;
_8 = _19;
Goto(bb13)
}
bb13 = {
_7 = _14 >> _13;
_1 = _7 as isize;
place!(Field::<i32>(Variant(_32, 0), 0)) = _30 as i32;
Goto(bb14)
}
bb14 = {
_5 = [(-3655017725168043618_i64)];
_31 = _21 - _21;
_31 = _27 * _27;
SetDiscriminant(_32, 2);
_22 = _11;
place!(Field::<[i64; 1]>(Variant(_32, 2), 3)) = [(-5468638175470494651_i64)];
_4 = _10 as i8;
_27 = _21;
_2 = _22;
RET = [(-20291951319426077830623475141603677039_i128),167297322375065150096632960641313547876_i128,163604259456234143113419365524994352871_i128,9100885846283254006019482703188218908_i128,113344384609441438450540450400084225424_i128,99659955096249646855911617993305790775_i128];
_38 = !_1;
_37 = _27 as i64;
_19 = _8;
RET = [149306385483538301145648679279573936962_i128,(-20702364504726546351001185126753642331_i128),116344049239389959476946089991319781672_i128,(-10017424909466510516686873810704357990_i128),12126523039673449623413533385824810590_i128,20458847276771385249921285345654193_i128];
Goto(bb15)
}
bb15 = {
Call(_43 = dump_var(10_usize, 23_usize, Move(_23), 10_usize, Move(_10), 5_usize, Move(_5), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_43 = dump_var(10_usize, 28_usize, Move(_28), 17_usize, Move(_17), 30_usize, Move(_30), 38_usize, Move(_38)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_43 = dump_var(10_usize, 3_usize, Move(_3), 6_usize, Move(_6), 15_usize, Move(_15), 25_usize, Move(_25)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_43 = dump_var(10_usize, 13_usize, Move(_13), 16_usize, Move(_16), 44_usize, _44, 44_usize, _44), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: bool,mut _2: isize,mut _3: [i8; 1],mut _4: [u64; 8],mut _5: [i8; 1],mut _6: [i128; 6],mut _7: bool) -> u16 {
mir! {
type RET = u16;
let _8: f64;
let _9: [u16; 5];
let _10: Adt49;
let _11: Adt50;
let _12: [bool; 1];
let _13: u8;
let _14: Adt51;
let _15: (u16,);
let _16: bool;
let _17: f64;
let _18: Adt55;
let _19: char;
let _20: [u64; 8];
let _21: Adt45;
let _22: Adt50;
let _23: isize;
let _24: f32;
let _25: [isize; 8];
let _26: (u32,);
let _27: [i128; 6];
let _28: (&'static u8, (u16,));
let _29: *const (&'static u8, (u16,));
let _30: u64;
let _31: isize;
let _32: [i128; 4];
let _33: bool;
let _34: f32;
let _35: isize;
let _36: Adt54;
let _37: char;
let _38: ();
let _39: ();
{
_5 = [(-67_i8)];
_3 = [91_i8];
_1 = _7 ^ _7;
_3 = [10_i8];
_8 = 26850_u16 as f64;
Goto(bb1)
}
bb1 = {
RET = !42208_u16;
_8 = 9553545337209802865_u64 as f64;
_6 = [(-95517435986282079705450328284146881492_i128),76688118449405075885275796730132734273_i128,(-105830362553304990777959126428770172494_i128),(-9501949582017797400178716629711556705_i128),(-126784793405644001043230525813897210989_i128),55387469555753388874569799102984974260_i128];
_9 = [RET,RET,RET,RET,RET];
_2 = 3_usize as isize;
_2 = -(-9223372036854775808_isize);
_7 = _1;
_3 = [71_i8];
_6 = [(-24868540710832394674629817528898594736_i128),32805863539300247197062971971658578017_i128,(-141228686358622974265193416726767484466_i128),8335349449690954253441448351936356771_i128,(-15847022865929791833806444531866887441_i128),122078688453797601986681412841345009617_i128];
_8 = 782080169004583718_i64 as f64;
Goto(bb2)
}
bb2 = {
_8 = 4680616115819175939_usize as f64;
_3 = _5;
Goto(bb3)
}
bb3 = {
_8 = _2 as f64;
_9 = [RET,RET,RET,RET,RET];
_9 = [RET,RET,RET,RET,RET];
_4 = [8313979915369652515_u64,440335646303978136_u64,2107928673192081018_u64,378263661703534978_u64,7236889140614745164_u64,16602639969287695648_u64,5725351860468391206_u64,12856456133557022148_u64];
_5 = _3;
Goto(bb4)
}
bb4 = {
_9 = [RET,RET,RET,RET,RET];
Goto(bb5)
}
bb5 = {
_4 = [17875796510218731764_u64,16080739537696555402_u64,17006969804144282784_u64,9049790764723557957_u64,4090168881278012469_u64,948779750321090927_u64,6029516487064314943_u64,1212183534668279977_u64];
_9 = [RET,RET,RET,RET,RET];
Call(_13 = core::intrinsics::transmute(_1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_8 = 310152276501981451140860361645386449788_u128 as f64;
_14.fld5 = ['\u{260a0}','\u{26f43}','\u{4ac14}','\u{9bc14}','\u{e50e2}','\u{20e03}'];
_7 = !_1;
_14.fld2 = !3730385362_u32;
_14.fld4 = [(-12046_i16),10875_i16,8874_i16];
_14.fld2 = 1574746063_u32 >> _13;
_15.0 = RET;
_14.fld5 = ['\u{15e7c}','\u{e3b6f}','\u{8d5ce}','\u{d8420}','\u{a8f5a}','\u{100723}'];
_14.fld1 = [9782585913731529856_u64,9171214843568206948_u64,14419634896111815149_u64,16617138082704751315_u64,4338214080041753097_u64,553445756124615315_u64,15067091149105081581_u64,10153873856578104648_u64];
_16 = !_7;
Call(_14.fld3 = fn12(_16, _8, _13, _14.fld4), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_14.fld2 = 1227090929_u32 << RET;
_14.fld6 = _15.0;
RET = !_14.fld6;
_14.fld0 = 15121548635406705103_u64 ^ 3920008732920456933_u64;
_6 = [165967043154828046885815661297635640204_i128,20321989282835184676779103588852663634_i128,135238685645698982060097134607471211705_i128,68385240587466038335699022466127502036_i128,(-45551499977725637956384239114765727626_i128),(-122393456576533237352228440697311869666_i128)];
_5 = [_14.fld3];
_14.fld3 = 40_i8 + 16_i8;
_9 = [_14.fld6,_14.fld6,RET,_14.fld6,_14.fld6];
RET = _14.fld6;
Goto(bb8)
}
bb8 = {
_19 = '\u{e953a}';
_15.0 = 3_usize as u16;
_14.fld1 = [_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0];
_13 = (-91476276878555028865747258345484687565_i128) as u8;
_14.fld3 = (-95_i8) & (-28_i8);
Goto(bb9)
}
bb9 = {
_12 = [_7];
_12 = [_16];
RET = !_15.0;
_13 = 2535502729325690300_usize as u8;
_14.fld0 = _2 as u64;
_19 = '\u{4c81a}';
_14.fld5 = [_19,_19,_19,_19,_19,_19];
_16 = !_1;
_16 = _7;
_20 = [_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0];
_14.fld2 = 3725343593_u32;
RET = !_14.fld6;
_14.fld1 = _4;
_14.fld4 = [(-7703_i16),23250_i16,(-28661_i16)];
_1 = _16;
_14.fld6 = RET;
_8 = 130397477829134591662175198647967256181_i128 as f64;
_12 = [_1];
_2 = 67_isize + 67_isize;
_9 = [RET,RET,_15.0,RET,RET];
_23 = _2;
_24 = 111001608157413412989960198192454994350_u128 as f32;
_11 = Adt50::Variant0 { fld0: _14.fld2,fld1: _19 };
_5 = [_14.fld3];
_23 = _2 | _2;
_16 = !_7;
match _14.fld2 {
0 => bb6,
3725343593 => bb11,
_ => bb10
}
}
bb10 = {
_4 = [17875796510218731764_u64,16080739537696555402_u64,17006969804144282784_u64,9049790764723557957_u64,4090168881278012469_u64,948779750321090927_u64,6029516487064314943_u64,1212183534668279977_u64];
_9 = [RET,RET,RET,RET,RET];
Call(_13 = core::intrinsics::transmute(_1), ReturnTo(bb6), UnwindUnreachable())
}
bb11 = {
RET = _24 as u16;
_25 = [_2,_23,_2,_23,_23,_2,_23,_2];
_22 = Move(_11);
_9 = [_14.fld6,_15.0,RET,_15.0,RET];
_21 = Adt45::Variant2 { fld0: 1761117030_i32,fld1: (-8680_i16) };
_11 = Move(_22);
RET = !_14.fld6;
_3 = _5;
_6 = [55650335047625797216078306594264948090_i128,62664873409402001939664122457914205222_i128,130676891962805832706555392375897265681_i128,(-164319269643723971304182775871215645663_i128),(-13175415916045017863011844260831745167_i128),167430133687473616449302624750498730964_i128];
_14.fld4 = [16857_i16,14382_i16,(-12681_i16)];
_24 = _15.0 as f32;
_7 = _16 == _16;
_22 = Move(_11);
_25 = [_2,_23,_2,_23,_23,_23,_23,_2];
place!(Field::<u32>(Variant(_22, 0), 0)) = _14.fld2 << _14.fld3;
_2 = -_23;
_19 = Field::<char>(Variant(_22, 0), 1);
_6 = [(-52858802405149069754610470036445897800_i128),136552477924908411228352641709639240262_i128,84154100013012305159863749224965655751_i128,(-96894259662661111345332090565788241067_i128),146373450338817402431420359391496055793_i128,(-139528997162389779685356520019314470891_i128)];
_16 = !_7;
_6 = [(-169832419418692729300923518236655635851_i128),104011083489933461840055455592287724369_i128,(-35111216945603224168013421311343501705_i128),91390964006426720966901264702801086240_i128,163712630618145635914782141741211465748_i128,(-38233295922346095919339279887436283263_i128)];
_9 = [RET,RET,_14.fld6,RET,_14.fld6];
_25 = [_23,_23,_2,_2,_2,_23,_23,_23];
_6 = [116302775292159446752122231911639528489_i128,27819299158852429828216491541539892112_i128,(-66860164994219021586180657762818949945_i128),(-161179344537578443602098238667776365316_i128),(-104010306231240625259451036146081720832_i128),84305292499594612916276501255550766714_i128];
SetDiscriminant(_22, 2);
match _14.fld2 {
0 => bb10,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb12,
5 => bb13,
3725343593 => bb15,
_ => bb14
}
}
bb12 = {
_4 = [17875796510218731764_u64,16080739537696555402_u64,17006969804144282784_u64,9049790764723557957_u64,4090168881278012469_u64,948779750321090927_u64,6029516487064314943_u64,1212183534668279977_u64];
_9 = [RET,RET,RET,RET,RET];
Call(_13 = core::intrinsics::transmute(_1), ReturnTo(bb6), UnwindUnreachable())
}
bb13 = {
_12 = [_7];
_12 = [_16];
RET = !_15.0;
_13 = 2535502729325690300_usize as u8;
_14.fld0 = _2 as u64;
_19 = '\u{4c81a}';
_14.fld5 = [_19,_19,_19,_19,_19,_19];
_16 = !_1;
_16 = _7;
_20 = [_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0];
_14.fld2 = 3725343593_u32;
RET = !_14.fld6;
_14.fld1 = _4;
_14.fld4 = [(-7703_i16),23250_i16,(-28661_i16)];
_1 = _16;
_14.fld6 = RET;
_8 = 130397477829134591662175198647967256181_i128 as f64;
_12 = [_1];
_2 = 67_isize + 67_isize;
_9 = [RET,RET,_15.0,RET,RET];
_23 = _2;
_24 = 111001608157413412989960198192454994350_u128 as f32;
_11 = Adt50::Variant0 { fld0: _14.fld2,fld1: _19 };
_5 = [_14.fld3];
_23 = _2 | _2;
_16 = !_7;
match _14.fld2 {
0 => bb6,
3725343593 => bb11,
_ => bb10
}
}
bb14 = {
_4 = [17875796510218731764_u64,16080739537696555402_u64,17006969804144282784_u64,9049790764723557957_u64,4090168881278012469_u64,948779750321090927_u64,6029516487064314943_u64,1212183534668279977_u64];
_9 = [RET,RET,RET,RET,RET];
Call(_13 = core::intrinsics::transmute(_1), ReturnTo(bb6), UnwindUnreachable())
}
bb15 = {
place!(Field::<*const f32>(Variant(_22, 2), 3)) = core::ptr::addr_of!(_24);
place!(Field::<i32>(Variant(_21, 2), 0)) = _8 as i32;
_14.fld1 = [_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0,_14.fld0];
place!(Field::<(u32,)>(Variant(_22, 2), 0)).0 = !_14.fld2;
_30 = _24 as u64;
_11 = Adt50::Variant1 { fld0: _24,fld1: _25,fld2: Field::<(u32,)>(Variant(_22, 2), 0),fld3: _14.fld4,fld4: (-27539_i16) };
_1 = _7;
place!(Field::<u128>(Variant(_22, 2), 5)) = 161613962659752764414415304776408510873_u128 | 244887386279970143240841932680525511122_u128;
_21 = Adt45::Variant2 { fld0: 249564372_i32,fld1: 26080_i16 };
_27 = [67139187601732486661812722117262243091_i128,(-130192808871365396167603362604199123953_i128),43523719196551561768459220660856232470_i128,(-70913831424984715367089409048261869591_i128),60548216605129016916008107267399346153_i128,81201192225547183037731910836611363990_i128];
_26 = Field::<(u32,)>(Variant(_22, 2), 0);
_35 = _23 - _2;
place!(Field::<(u32,)>(Variant(_22, 2), 0)).0 = Field::<(u32,)>(Variant(_11, 1), 2).0 >> _35;
_36.fld4.fld3.0 = [_14.fld3];
_29 = core::ptr::addr_of!(_28);
_27 = _6;
Goto(bb16)
}
bb16 = {
Call(_38 = dump_var(11_usize, 30_usize, Move(_30), 26_usize, Move(_26), 9_usize, Move(_9), 20_usize, Move(_20)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(11_usize, 35_usize, Move(_35), 12_usize, Move(_12), 16_usize, Move(_16), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_38 = dump_var(11_usize, 1_usize, Move(_1), 5_usize, Move(_5), 39_usize, _39, 39_usize, _39), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: bool,mut _2: f64,mut _3: u8,mut _4: [i16; 3]) -> i8 {
mir! {
type RET = i8;
let _5: [i128; 6];
let _6: Adt45;
let _7: [usize; 5];
let _8: f32;
let _9: u128;
let _10: Adt52;
let _11: i32;
let _12: isize;
let _13: i16;
let _14: i64;
let _15: char;
let _16: f32;
let _17: *const (&'static u8, (u16,));
let _18: isize;
let _19: u128;
let _20: [char; 6];
let _21: bool;
let _22: [char; 6];
let _23: *const (&'static u8, (u16,));
let _24: Adt39;
let _25: Adt47;
let _26: [u16; 5];
let _27: *mut (bool,);
let _28: *const (&'static u8, (u16,));
let _29: [i128; 4];
let _30: isize;
let _31: *mut (bool,);
let _32: f32;
let _33: [isize; 8];
let _34: i128;
let _35: [bool; 1];
let _36: *const (&'static u8, (u16,));
let _37: u16;
let _38: Adt51;
let _39: isize;
let _40: Adt51;
let _41: *mut (bool,);
let _42: Adt54;
let _43: i128;
let _44: ();
let _45: ();
{
RET = 0_usize as i8;
_3 = 172_u8;
RET = (-14_i8) << _3;
_1 = false;
_1 = !true;
_2 = 240283336086870107675342761989830306811_u128 as f64;
_2 = (-528_i16) as f64;
_1 = !true;
_3 = 104_u8 ^ 30_u8;
_1 = true;
_2 = 6_usize as f64;
_1 = false | true;
_4 = [31312_i16,12187_i16,(-27042_i16)];
RET = -(-33_i8);
_1 = true;
Goto(bb1)
}
bb1 = {
RET = (-37_i8);
_2 = 4520570350330365735_u64 as f64;
_1 = _3 < _3;
_2 = RET as f64;
RET = !(-59_i8);
_2 = 48992_u16 as f64;
RET = 1958966140703269057448527708881162835_i128 as i8;
_3 = 189_u8;
_4 = [3928_i16,(-4695_i16),1286_i16];
RET = 101_i8 & 34_i8;
_1 = false & false;
RET = (-9223372036854775808_isize) as i8;
_3 = !195_u8;
_3 = 139369563006947235144127625685140999892_u128 as u8;
RET = -(-33_i8);
_4 = [(-6536_i16),(-31880_i16),13246_i16];
_4 = [(-27971_i16),(-29089_i16),30337_i16];
_4 = [21159_i16,(-10965_i16),2613_i16];
RET = -(-114_i8);
RET = 59_i8 ^ 38_i8;
_3 = 37_u8;
_3 = !82_u8;
_3 = 9223372036854775807_isize as u8;
RET = 118_i8;
_7 = [4_usize,15887939330225234562_usize,3_usize,0_usize,9633233534975841242_usize];
_5 = [113382531699514390117702943238880472052_i128,(-65229877940100872170126327109117860540_i128),49134592050429337327117807931993244281_i128,(-63105523813464167815809745865781779802_i128),6346012643818426927959729275323185892_i128,90862588896406877060981983325337370184_i128];
Goto(bb2)
}
bb2 = {
_5 = [(-11231383785447152383384133255709381230_i128),(-59750347374618973317103638243619579534_i128),88556373387156909008071227878193767619_i128,31469412356974795299639848513643457068_i128,(-42424629522949339961587305905370258641_i128),(-6008899412113834359309952359871681932_i128)];
_10.fld3.0 = [RET];
_10.fld2 = (846534340_u32,);
Goto(bb3)
}
bb3 = {
_10.fld3.1 = 64374331357932932788461952322238434916_u128 + 318997608347472743826943235157747940950_u128;
_10.fld3.2 = _1;
_5 = [77136038515431012741843836629469760480_i128,(-71155668231475642655064444768234487536_i128),(-6208334903104315661165069585595425542_i128),(-95251899177103590623409210856839350340_i128),(-136790683398573401369131573631495206369_i128),6860635921343275634180689483641082303_i128];
_2 = 9223372036854775807_isize as f64;
_1 = _10.fld3.2 ^ _10.fld3.2;
RET = 11_i8;
_2 = 569_u16 as f64;
_9 = _10.fld3.1 << _10.fld3.1;
_10.fld0 = !_10.fld3.2;
_10.fld3.2 = _10.fld0 ^ _1;
_7 = [16440537868252403105_usize,5_usize,705441836804399702_usize,5850027026921839894_usize,3153320845451008036_usize];
_10.fld3.0 = [RET];
_3 = 139_u8;
_10.fld2.0 = 2752522041_u32 ^ 2652016432_u32;
_6 = Adt45::Variant2 { fld0: (-555168783_i32),fld1: (-9560_i16) };
_10.fld1 = [8157046444555074970_u64,9801526581142545609_u64,17251928228178059191_u64,9461391244199738810_u64,10640546489576225991_u64,10246768037778241903_u64,16938066908088420929_u64,342204400435641133_u64];
RET = !(-8_i8);
place!(Field::<i32>(Variant(_6, 2), 0)) = 694596237_i32 & 1902837004_i32;
RET = 9_i8;
_10.fld2 = (2694248612_u32,);
Goto(bb4)
}
bb4 = {
_10.fld0 = !_10.fld3.2;
_2 = (-115381931534623953301285403882872142938_i128) as f64;
_14 = 2850511217103683201_i64 * (-3278221577008093727_i64);
_10.fld3.0 = [RET];
_13 = (-11434_i16);
_12 = RET as isize;
_4 = [_13,_13,_13];
_10.fld3.2 = _10.fld0 < _10.fld0;
_10.fld2.0 = 2187033424_u32 >> Field::<i32>(Variant(_6, 2), 0);
match _13 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
340282366920938463463374607431768200022 => bb8,
_ => bb7
}
}
bb5 = {
_10.fld3.1 = 64374331357932932788461952322238434916_u128 + 318997608347472743826943235157747940950_u128;
_10.fld3.2 = _1;
_5 = [77136038515431012741843836629469760480_i128,(-71155668231475642655064444768234487536_i128),(-6208334903104315661165069585595425542_i128),(-95251899177103590623409210856839350340_i128),(-136790683398573401369131573631495206369_i128),6860635921343275634180689483641082303_i128];
_2 = 9223372036854775807_isize as f64;
_1 = _10.fld3.2 ^ _10.fld3.2;
RET = 11_i8;
_2 = 569_u16 as f64;
_9 = _10.fld3.1 << _10.fld3.1;
_10.fld0 = !_10.fld3.2;
_10.fld3.2 = _10.fld0 ^ _1;
_7 = [16440537868252403105_usize,5_usize,705441836804399702_usize,5850027026921839894_usize,3153320845451008036_usize];
_10.fld3.0 = [RET];
_3 = 139_u8;
_10.fld2.0 = 2752522041_u32 ^ 2652016432_u32;
_6 = Adt45::Variant2 { fld0: (-555168783_i32),fld1: (-9560_i16) };
_10.fld1 = [8157046444555074970_u64,9801526581142545609_u64,17251928228178059191_u64,9461391244199738810_u64,10640546489576225991_u64,10246768037778241903_u64,16938066908088420929_u64,342204400435641133_u64];
RET = !(-8_i8);
place!(Field::<i32>(Variant(_6, 2), 0)) = 694596237_i32 & 1902837004_i32;
RET = 9_i8;
_10.fld2 = (2694248612_u32,);
Goto(bb4)
}
bb6 = {
_5 = [(-11231383785447152383384133255709381230_i128),(-59750347374618973317103638243619579534_i128),88556373387156909008071227878193767619_i128,31469412356974795299639848513643457068_i128,(-42424629522949339961587305905370258641_i128),(-6008899412113834359309952359871681932_i128)];
_10.fld3.0 = [RET];
_10.fld2 = (846534340_u32,);
Goto(bb3)
}
bb7 = {
RET = (-37_i8);
_2 = 4520570350330365735_u64 as f64;
_1 = _3 < _3;
_2 = RET as f64;
RET = !(-59_i8);
_2 = 48992_u16 as f64;
RET = 1958966140703269057448527708881162835_i128 as i8;
_3 = 189_u8;
_4 = [3928_i16,(-4695_i16),1286_i16];
RET = 101_i8 & 34_i8;
_1 = false & false;
RET = (-9223372036854775808_isize) as i8;
_3 = !195_u8;
_3 = 139369563006947235144127625685140999892_u128 as u8;
RET = -(-33_i8);
_4 = [(-6536_i16),(-31880_i16),13246_i16];
_4 = [(-27971_i16),(-29089_i16),30337_i16];
_4 = [21159_i16,(-10965_i16),2613_i16];
RET = -(-114_i8);
RET = 59_i8 ^ 38_i8;
_3 = 37_u8;
_3 = !82_u8;
_3 = 9223372036854775807_isize as u8;
RET = 118_i8;
_7 = [4_usize,15887939330225234562_usize,3_usize,0_usize,9633233534975841242_usize];
_5 = [113382531699514390117702943238880472052_i128,(-65229877940100872170126327109117860540_i128),49134592050429337327117807931993244281_i128,(-63105523813464167815809745865781779802_i128),6346012643818426927959729275323185892_i128,90862588896406877060981983325337370184_i128];
Goto(bb2)
}
bb8 = {
_5 = [64454034775437565604051524713024239326_i128,(-76153630882637035768278559874018775907_i128),(-62748892470012755008184861623187166470_i128),114714613705101050174132799614435556297_i128,105456758964586753531167142018378998627_i128,611594289894643514225579831674472812_i128];
_10.fld0 = _9 < _9;
_12 = _9 as isize;
_10.fld3.0 = [RET];
RET = -65_i8;
_7 = [18287012085325551601_usize,7_usize,3_usize,1_usize,1_usize];
_10.fld0 = _12 >= _12;
_10.fld3.2 = _10.fld0 != _10.fld0;
_10.fld3.1 = !_9;
_2 = Field::<i32>(Variant(_6, 2), 0) as f64;
place!(Field::<i16>(Variant(_6, 2), 1)) = _13 ^ _13;
_9 = !_10.fld3.1;
_18 = _12;
_8 = _9 as f32;
_10.fld2.0 = !1568203411_u32;
Call(_11 = fn13(_12, _2, _18, Move(_10), Move(_6)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_15 = '\u{d4b7e}';
_13 = (-5797_i16) * 20128_i16;
_20 = [_15,_15,_15,_15,_15,_15];
_10.fld2 = (2629618478_u32,);
_10.fld3.2 = !_1;
_14 = _13 as i64;
_10.fld2 = (3692631274_u32,);
_19 = _9 - _9;
_10.fld3.0 = [RET];
_18 = _10.fld3.2 as isize;
_8 = _3 as f32;
_3 = !138_u8;
_14 = -(-4292509808720404248_i64);
_13 = !(-20295_i16);
_14 = -(-122882582783594092_i64);
_8 = (-28344741790722402495977132809464623077_i128) as f32;
_6 = Adt45::Variant2 { fld0: _11,fld1: _13 };
_21 = _1;
_16 = _8;
_1 = _19 < _9;
_10.fld0 = _1 & _1;
_10.fld3.1 = _14 as u128;
_10.fld2.0 = 1774240557_u32 * 655425186_u32;
_5 = [(-99451128050461834328143239224155178828_i128),38962407915445424067761322340606172971_i128,(-148576543947740521159252019208194047237_i128),9059809615630931930057366624311217777_i128,19213621596186462310889963131674454989_i128,60183104981755785864506374448364092853_i128];
match _11 {
0 => bb1,
1 => bb2,
340282366920938463463374607430187457005 => bb11,
_ => bb10
}
}
bb10 = {
RET = (-37_i8);
_2 = 4520570350330365735_u64 as f64;
_1 = _3 < _3;
_2 = RET as f64;
RET = !(-59_i8);
_2 = 48992_u16 as f64;
RET = 1958966140703269057448527708881162835_i128 as i8;
_3 = 189_u8;
_4 = [3928_i16,(-4695_i16),1286_i16];
RET = 101_i8 & 34_i8;
_1 = false & false;
RET = (-9223372036854775808_isize) as i8;
_3 = !195_u8;
_3 = 139369563006947235144127625685140999892_u128 as u8;
RET = -(-33_i8);
_4 = [(-6536_i16),(-31880_i16),13246_i16];
_4 = [(-27971_i16),(-29089_i16),30337_i16];
_4 = [21159_i16,(-10965_i16),2613_i16];
RET = -(-114_i8);
RET = 59_i8 ^ 38_i8;
_3 = 37_u8;
_3 = !82_u8;
_3 = 9223372036854775807_isize as u8;
RET = 118_i8;
_7 = [4_usize,15887939330225234562_usize,3_usize,0_usize,9633233534975841242_usize];
_5 = [113382531699514390117702943238880472052_i128,(-65229877940100872170126327109117860540_i128),49134592050429337327117807931993244281_i128,(-63105523813464167815809745865781779802_i128),6346012643818426927959729275323185892_i128,90862588896406877060981983325337370184_i128];
Goto(bb2)
}
bb11 = {
_1 = _19 > _19;
_22 = _20;
RET = 11_i8;
SetDiscriminant(_6, 2);
_6 = Adt45::Variant2 { fld0: _11,fld1: _13 };
_18 = _21 as isize;
_16 = _8 - _8;
_10.fld1 = [3896263432313893648_u64,7613781654795318502_u64,2039164068327335923_u64,10739844500642453206_u64,5668269991168053027_u64,12263723316241163184_u64,13130843610677145748_u64,15039351962666363742_u64];
_9 = 7_usize as u128;
_10.fld2.0 = 12875_u16 as u32;
_12 = _18 * _18;
Goto(bb12)
}
bb12 = {
place!(Field::<i32>(Variant(_6, 2), 0)) = _11 * _11;
_10.fld2 = (2217369577_u32,);
_10.fld2 = (838691867_u32,);
_22 = [_15,_15,_15,_15,_15,_15];
_4 = [_13,_13,_13];
_14 = -(-9120966799485735828_i64);
_5 = [(-113038801426055188667667392121950902243_i128),(-50744725918238541484312572259548761029_i128),15366971534360180005861088858222669000_i128,(-23481768122136794946671149173019390936_i128),79410367423973519968319127400217329021_i128,49579538835812021418839950885148956311_i128];
_19 = _18 as u128;
_19 = _10.fld3.1 - _9;
_3 = 106_u8;
_10.fld3.0 = [RET];
_22 = [_15,_15,_15,_15,_15,_15];
_5 = [(-21807809657630247613686495455195579630_i128),(-7475314369319988527277872224228922867_i128),(-118132524818449992762070191888985456233_i128),(-116467717667218876906567167561578456690_i128),(-108402916766388362052588821392715716411_i128),30753401644304586371965228105846055346_i128];
_10.fld3.2 = _21 & _1;
_22 = [_15,_15,_15,_15,_15,_15];
_2 = 28303_u16 as f64;
_16 = _8;
_29 = [50653775283795687558069163275041146316_i128,118808277603313909862511645158719069699_i128,102770769487947461511457955474112014057_i128,(-159066083634871667449693479733505249863_i128)];
_10.fld2.0 = 3737329948_u32;
_10.fld2 = (3692066132_u32,);
_10.fld2 = (535628632_u32,);
_20 = _22;
_10.fld2 = (2897168325_u32,);
_12 = _18;
Goto(bb13)
}
bb13 = {
_10.fld2.0 = !2862282518_u32;
SetDiscriminant(_6, 1);
RET = 54_i8 << _12;
place!(Field::<u16>(Variant(_6, 1), 2)) = 55660_u16;
_33 = [_18,_12,_12,_12,_12,_18,_18,_12];
_34 = _16 as i128;
_10.fld2.0 = 2321209906_u32 + 279834194_u32;
_19 = _9;
_20 = [_15,_15,_15,_15,_15,_15];
_8 = _16 + _16;
_11 = 666263728_i32;
_1 = !_10.fld3.2;
_38.fld1 = [2127565827383828645_u64,6084781388902328029_u64,3425405087473514661_u64,4229675641238721464_u64,7071229308174559401_u64,17512254033235144341_u64,14241156147541300106_u64,17935044952701727237_u64];
match Field::<u16>(Variant(_6, 1), 2) {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb4,
4 => bb11,
5 => bb6,
55660 => bb14,
_ => bb7
}
}
bb14 = {
_32 = -_8;
_8 = 16814745058920142078_usize as f32;
_19 = _10.fld3.1 | _10.fld3.1;
_39 = !_12;
_39 = _3 as isize;
_30 = -_18;
_13 = (-10171_i16);
place!(Field::<char>(Variant(_6, 1), 1)) = _15;
_38.fld5 = [_15,Field::<char>(Variant(_6, 1), 1),_15,_15,_15,Field::<char>(Variant(_6, 1), 1)];
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(12_usize, 1_usize, Move(_1), 3_usize, Move(_3), 15_usize, Move(_15), 34_usize, Move(_34)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(12_usize, 7_usize, Move(_7), 33_usize, Move(_33), 19_usize, Move(_19), 21_usize, Move(_21)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(12_usize, 20_usize, Move(_20), 9_usize, Move(_9), 45_usize, _45, 45_usize, _45), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: isize,mut _2: f64,mut _3: isize,mut _4: Adt52,mut _5: Adt45) -> i32 {
mir! {
type RET = i32;
let _6: i64;
let _7: i128;
let _8: isize;
let _9: Adt40;
let _10: (u32,);
let _11: char;
let _12: [i64; 1];
let _13: [i16; 3];
let _14: [isize; 8];
let _15: [char; 6];
let _16: char;
let _17: u8;
let _18: [i64; 1];
let _19: &'static u8;
let _20: *const (&'static u8, (u16,));
let _21: u64;
let _22: Adt47;
let _23: Adt47;
let _24: Adt52;
let _25: isize;
let _26: char;
let _27: (u32,);
let _28: i32;
let _29: ([i8; 1], u128, bool);
let _30: isize;
let _31: Adt52;
let _32: Adt44;
let _33: bool;
let _34: ();
let _35: ();
{
_4.fld0 = _1 != _3;
_3 = (-128_i8) as isize;
_4.fld0 = _4.fld3.2;
Call(place!(Field::<i32>(Variant(_5, 2), 0)) = fn14(_4.fld0, _4.fld1, Move(_4), _1, _1, _1, _1, _2, _1, _1, Field::<i16>(Variant(_5, 2), 1)), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _2 as isize;
_4.fld3.2 = _1 < _3;
_4.fld3.1 = 267962923920801986893871091814096159166_u128 * 175272726013300685040650695251267486196_u128;
RET = -Field::<i32>(Variant(_5, 2), 0);
place!(Field::<i32>(Variant(_5, 2), 0)) = 17402926037290628205_usize as i32;
_4.fld1 = [6712754645101495352_u64,10293152899566743950_u64,2265870145069119241_u64,7735275869926714157_u64,3663472410194605391_u64,15434007756711820359_u64,11069923897659169033_u64,8749530584302847472_u64];
_4.fld3.1 = !11370204692199044704944691254984344375_u128;
_4.fld3.1 = 132662913917150884745417957456354081760_u128 * 338335302966123762069947218179468155385_u128;
_4.fld3.0 = [(-63_i8)];
_7 = 108698514683805379080332188216176931643_i128 - 93384451869087566231230233752849060251_i128;
RET = Field::<i32>(Variant(_5, 2), 0) << _3;
place!(Field::<i16>(Variant(_5, 2), 1)) = RET as i16;
_10.0 = 13317962651607860290_u64 as u32;
_8 = _1 ^ _1;
_4.fld2.0 = 55434_u16 as u32;
_3 = 22234_u16 as isize;
_4.fld3.1 = 235198698357486767663886327899792724425_u128 & 138262852938568804067657218030359058864_u128;
_7 = (-28145400221100294010416965904222679447_i128);
_6 = 184_u8 as i64;
Goto(bb2)
}
bb2 = {
_10.0 = _4.fld2.0 + _4.fld2.0;
_10 = (_4.fld2.0,);
_10 = _4.fld2;
Goto(bb3)
}
bb3 = {
_10.0 = RET as u32;
_4.fld2 = _10;
_6 = (-4075998141147558430_i64) | (-5380402926008623421_i64);
_3 = _8 - _8;
_7 = (-103122937210864174500717851669415038100_i128) & 34231266123169416774057214540886733106_i128;
_4.fld3.0 = [48_i8];
place!(Field::<i16>(Variant(_5, 2), 1)) = -1605_i16;
_4.fld2.0 = _10.0 + _10.0;
RET = Field::<i32>(Variant(_5, 2), 0);
SetDiscriminant(_5, 0);
place!(Field::<[usize; 7]>(Variant(_5, 0), 3)) = [10444028308990355572_usize,14494818441101308519_usize,3919624517582451904_usize,6_usize,0_usize,0_usize,16702165383442879344_usize];
RET = 1100535403_i32 & 695502390_i32;
_4.fld2.0 = 6_usize as u32;
_3 = _1;
RET = (-1981955707_i32);
place!(Field::<isize>(Variant(_5, 0), 2)) = _8;
place!(Field::<isize>(Variant(_5, 0), 2)) = _8;
place!(Field::<isize>(Variant(_5, 0), 2)) = _3 | _8;
_11 = '\u{3775a}';
RET = -(-1350739827_i32);
Call(_3 = fn15(_8, _8, _10.0, _8, _4.fld3, _4.fld3.0, _4.fld3.2, _8, Field::<isize>(Variant(_5, 0), 2), _8, Field::<isize>(Variant(_5, 0), 2), _1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
RET = 979556210_i32;
_4.fld1 = [17990903834199775981_u64,2425285637724093965_u64,3376774522913580203_u64,7654877582579987208_u64,14118028527174662799_u64,14455533529173378896_u64,9838592195035563158_u64,11673619796708623695_u64];
place!(Field::<isize>(Variant(_5, 0), 2)) = -_8;
place!(Field::<usize>(Variant(_5, 0), 0)) = 6997153395675949916_usize - 5391145568199051067_usize;
place!(Field::<usize>(Variant(_5, 0), 0)) = 10805563996950244426_usize;
_7 = (-1735814913581034275476613016173872996_i128);
_14 = [_8,Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),_8,Field::<isize>(Variant(_5, 0), 2),_8];
_3 = _4.fld3.2 as isize;
_10 = _4.fld2;
_10.0 = _4.fld3.1 as u32;
_13 = [(-23709_i16),16737_i16,(-7223_i16)];
place!(Field::<isize>(Variant(_5, 0), 2)) = _8 ^ _8;
place!(Field::<usize>(Variant(_5, 0), 0)) = 858505363333753267_usize + 15314915456239355564_usize;
_13 = [(-4196_i16),(-22233_i16),(-32748_i16)];
_4.fld0 = !_4.fld3.2;
RET = -(-1958727150_i32);
_10.0 = !_4.fld2.0;
_12 = [_6];
_2 = 1774727185194928478_u64 as f64;
_11 = '\u{248c8}';
Call(_14 = fn17(_4.fld3, _10.0, _4.fld3.2, Field::<isize>(Variant(_5, 0), 2), _12, Field::<[usize; 7]>(Variant(_5, 0), 3)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_4.fld3.0 = [(-120_i8)];
_7 = 42468665349016962722509158692123743587_i128 * 10070082768427328251497863245978580663_i128;
place!(Field::<[usize; 7]>(Variant(_5, 0), 3)) = [Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0)];
_15 = [_11,_11,_11,_11,_11,_11];
_14 = [_1,_3,Field::<isize>(Variant(_5, 0), 2),_1,_3,_3,Field::<isize>(Variant(_5, 0), 2),_8];
_16 = _11;
_17 = 32_u8;
_10.0 = _4.fld2.0;
_4.fld2.0 = _10.0 + _10.0;
_10.0 = _4.fld3.2 as u32;
_16 = _11;
place!(Field::<usize>(Variant(_5, 0), 0)) = !1_usize;
_18 = [_6];
_17 = 96_u8 + 58_u8;
_1 = -_3;
_4.fld3.1 = !323798616865393899379317728777476252604_u128;
RET = (-1941371921_i32) - (-418702265_i32);
_11 = _16;
_4.fld0 = !_4.fld3.2;
_14 = [_1,_8,Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),_3,_3];
Goto(bb6)
}
bb6 = {
place!(Field::<[usize; 7]>(Variant(_5, 0), 3)) = [Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0)];
_4.fld3.1 = 305819973454178454270318589082327879093_u128;
_17 = !129_u8;
place!(Field::<usize>(Variant(_5, 0), 0)) = 3_usize;
_1 = !Field::<isize>(Variant(_5, 0), 2);
_2 = Field::<usize>(Variant(_5, 0), 0) as f64;
_6 = (-5369641892569415392_i64);
_5 = Adt45::Variant2 { fld0: RET,fld1: (-27568_i16) };
_10.0 = _4.fld2.0;
_17 = _2 as u8;
_11 = _16;
RET = -Field::<i32>(Variant(_5, 2), 0);
_10 = (_4.fld2.0,);
place!(Field::<i16>(Variant(_5, 2), 1)) = (-30196_i16);
Goto(bb7)
}
bb7 = {
_5 = Adt45::Variant2 { fld0: RET,fld1: (-14386_i16) };
RET = 50628_u16 as i32;
place!(Field::<i16>(Variant(_5, 2), 1)) = (-17783_i16);
_16 = _11;
_15 = [_16,_11,_11,_16,_16,_11];
_15 = [_11,_16,_16,_11,_16,_16];
_15 = [_16,_11,_16,_11,_16,_16];
place!(Field::<i16>(Variant(_5, 2), 1)) = (-19550_i16);
_17 = Field::<i32>(Variant(_5, 2), 0) as u8;
_17 = 2_u8;
_19 = &_17;
place!(Field::<i16>(Variant(_5, 2), 1)) = 19301_u16 as i16;
place!(Field::<i16>(Variant(_5, 2), 1)) = _2 as i16;
_4.fld0 = _4.fld3.2;
_24.fld0 = _4.fld3.2;
_24 = Move(_4);
_21 = !9261847014960926423_u64;
_8 = _21 as isize;
_24.fld2.0 = _10.0;
_10 = _24.fld2;
Goto(bb8)
}
bb8 = {
place!(Field::<i32>(Variant(_5, 2), 0)) = RET;
_4.fld1 = [_21,_21,_21,_21,_21,_21,_21,_21];
_12 = [_6];
_21 = !8925683028630051184_u64;
_10 = _24.fld2;
_21 = 3782737527114213243_u64;
_17 = !86_u8;
_4.fld3.0 = [85_i8];
Goto(bb9)
}
bb9 = {
_25 = _1;
_12 = [_6];
_24.fld3.0 = [21_i8];
_6 = (-8789992073181478544_i64);
_14 = [_25,_25,_1,_25,_25,_1,_25,_1];
_24.fld1 = [_21,_21,_21,_21,_21,_21,_21,_21];
SetDiscriminant(_5, 1);
_24.fld2 = _10;
_14 = [_25,_1,_1,_1,_1,_1,_1,_3];
_15 = [_16,_16,_16,_11,_11,_11];
match _24.fld3.1 {
0 => bb7,
305819973454178454270318589082327879093 => bb11,
_ => bb10
}
}
bb10 = {
place!(Field::<[usize; 7]>(Variant(_5, 0), 3)) = [Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0)];
_4.fld3.1 = 305819973454178454270318589082327879093_u128;
_17 = !129_u8;
place!(Field::<usize>(Variant(_5, 0), 0)) = 3_usize;
_1 = !Field::<isize>(Variant(_5, 0), 2);
_2 = Field::<usize>(Variant(_5, 0), 0) as f64;
_6 = (-5369641892569415392_i64);
_5 = Adt45::Variant2 { fld0: RET,fld1: (-27568_i16) };
_10.0 = _4.fld2.0;
_17 = _2 as u8;
_11 = _16;
RET = -Field::<i32>(Variant(_5, 2), 0);
_10 = (_4.fld2.0,);
place!(Field::<i16>(Variant(_5, 2), 1)) = (-30196_i16);
Goto(bb7)
}
bb11 = {
_4.fld3 = (_24.fld3.0, _24.fld3.1, _24.fld0);
_24 = Adt52 { fld0: _4.fld3.2,fld1: _4.fld1,fld2: _10,fld3: _4.fld3 };
_24.fld3 = _4.fld3;
_21 = 14401621501987070403_u64 | 13108643671319221273_u64;
place!(Field::<(bool,)>(Variant(_5, 1), 3)).0 = !_24.fld3.2;
RET = (-1580754451_i32);
_29.2 = !_24.fld0;
_16 = _11;
_27.0 = !_10.0;
match _4.fld3.1 {
0 => bb7,
1 => bb2,
2 => bb5,
3 => bb12,
4 => bb13,
305819973454178454270318589082327879093 => bb15,
_ => bb14
}
}
bb12 = {
place!(Field::<[usize; 7]>(Variant(_5, 0), 3)) = [Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0)];
_4.fld3.1 = 305819973454178454270318589082327879093_u128;
_17 = !129_u8;
place!(Field::<usize>(Variant(_5, 0), 0)) = 3_usize;
_1 = !Field::<isize>(Variant(_5, 0), 2);
_2 = Field::<usize>(Variant(_5, 0), 0) as f64;
_6 = (-5369641892569415392_i64);
_5 = Adt45::Variant2 { fld0: RET,fld1: (-27568_i16) };
_10.0 = _4.fld2.0;
_17 = _2 as u8;
_11 = _16;
RET = -Field::<i32>(Variant(_5, 2), 0);
_10 = (_4.fld2.0,);
place!(Field::<i16>(Variant(_5, 2), 1)) = (-30196_i16);
Goto(bb7)
}
bb13 = {
_4.fld3.0 = [(-120_i8)];
_7 = 42468665349016962722509158692123743587_i128 * 10070082768427328251497863245978580663_i128;
place!(Field::<[usize; 7]>(Variant(_5, 0), 3)) = [Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0),Field::<usize>(Variant(_5, 0), 0)];
_15 = [_11,_11,_11,_11,_11,_11];
_14 = [_1,_3,Field::<isize>(Variant(_5, 0), 2),_1,_3,_3,Field::<isize>(Variant(_5, 0), 2),_8];
_16 = _11;
_17 = 32_u8;
_10.0 = _4.fld2.0;
_4.fld2.0 = _10.0 + _10.0;
_10.0 = _4.fld3.2 as u32;
_16 = _11;
place!(Field::<usize>(Variant(_5, 0), 0)) = !1_usize;
_18 = [_6];
_17 = 96_u8 + 58_u8;
_1 = -_3;
_4.fld3.1 = !323798616865393899379317728777476252604_u128;
RET = (-1941371921_i32) - (-418702265_i32);
_11 = _16;
_4.fld0 = !_4.fld3.2;
_14 = [_1,_8,Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),_3,_3];
Goto(bb6)
}
bb14 = {
RET = 979556210_i32;
_4.fld1 = [17990903834199775981_u64,2425285637724093965_u64,3376774522913580203_u64,7654877582579987208_u64,14118028527174662799_u64,14455533529173378896_u64,9838592195035563158_u64,11673619796708623695_u64];
place!(Field::<isize>(Variant(_5, 0), 2)) = -_8;
place!(Field::<usize>(Variant(_5, 0), 0)) = 6997153395675949916_usize - 5391145568199051067_usize;
place!(Field::<usize>(Variant(_5, 0), 0)) = 10805563996950244426_usize;
_7 = (-1735814913581034275476613016173872996_i128);
_14 = [_8,Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),Field::<isize>(Variant(_5, 0), 2),_8,Field::<isize>(Variant(_5, 0), 2),_8];
_3 = _4.fld3.2 as isize;
_10 = _4.fld2;
_10.0 = _4.fld3.1 as u32;
_13 = [(-23709_i16),16737_i16,(-7223_i16)];
place!(Field::<isize>(Variant(_5, 0), 2)) = _8 ^ _8;
place!(Field::<usize>(Variant(_5, 0), 0)) = 858505363333753267_usize + 15314915456239355564_usize;
_13 = [(-4196_i16),(-22233_i16),(-32748_i16)];
_4.fld0 = !_4.fld3.2;
RET = -(-1958727150_i32);
_10.0 = !_4.fld2.0;
_12 = [_6];
_2 = 1774727185194928478_u64 as f64;
_11 = '\u{248c8}';
Call(_14 = fn17(_4.fld3, _10.0, _4.fld3.2, Field::<isize>(Variant(_5, 0), 2), _12, Field::<[usize; 7]>(Variant(_5, 0), 3)), ReturnTo(bb5), UnwindUnreachable())
}
bb15 = {
_24.fld3.2 = _24.fld0;
_28 = 1_usize as i32;
_33 = _29.2;
_24.fld3.1 = _4.fld3.1 - _4.fld3.1;
_31 = Adt52 { fld0: _33,fld1: _24.fld1,fld2: _27,fld3: _4.fld3 };
Goto(bb16)
}
bb16 = {
Call(_34 = dump_var(13_usize, 13_usize, Move(_13), 10_usize, Move(_10), 18_usize, Move(_18), 12_usize, Move(_12)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_34 = dump_var(13_usize, 14_usize, Move(_14), 33_usize, Move(_33), 1_usize, Move(_1), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(13_usize, 16_usize, Move(_16), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: bool,mut _2: [u64; 8],mut _3: Adt52,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: f64,mut _9: isize,mut _10: isize,mut _11: i16) -> i32 {
mir! {
type RET = i32;
let _12: [bool; 1];
let _13: (u32,);
let _14: (u32,);
let _15: *const (&'static u8, (u16,));
let _16: Adt40;
let _17: Adt53;
let _18: *const (&'static u8, (u16,));
let _19: (bool,);
let _20: Adt53;
let _21: Adt43;
let _22: isize;
let _23: f32;
let _24: [isize; 8];
let _25: char;
let _26: i32;
let _27: Adt41;
let _28: (u16,);
let _29: ();
let _30: ();
{
_6 = _4;
RET = 88_i8 as i32;
_3.fld1 = _2;
_9 = !_6;
_9 = !_6;
_3.fld0 = _3.fld3.2;
_3.fld2.0 = 882610831_u32;
_9 = _6 * _4;
_3.fld3.1 = _9 as u128;
_3.fld1 = [13641071212478290944_u64,3104491366038226725_u64,7036645141307367512_u64,1655061128809356065_u64,12814483078878641763_u64,6943052888179331714_u64,12937506309292326910_u64,2285627645180604813_u64];
_3.fld2 = (3248565960_u32,);
_12 = [_3.fld0];
_9 = -_5;
_11 = (-24679_i16);
RET = 3_usize as i32;
_3.fld2 = (1798595872_u32,);
_3.fld2.0 = !1078972643_u32;
_6 = _4 << _3.fld2.0;
Goto(bb1)
}
bb1 = {
RET = (-118987205_i32);
_3.fld2 = (2137318739_u32,);
_3.fld3.1 = 202327806823425329556605462945419060372_u128 | 156884995303480620377875795220257341212_u128;
_3.fld3.0 = [75_i8];
_2 = _3.fld1;
_13.0 = !_3.fld2.0;
_7 = 2_usize as isize;
_13 = (_3.fld2.0,);
_14.0 = _3.fld2.0 | _13.0;
_17.fld0 = [91413877876276260601499239795706320224_i128,95580712599986419268144075496377048892_i128,(-156207413074302133987518913633856631322_i128),134491896615456287106515116354707068067_i128,(-140575675948681065804850682565973710551_i128),(-161327162400770251832922067461485431724_i128)];
_1 = _10 >= _10;
_2 = [14330455871925835829_u64,17455367604195064664_u64,761261012627457309_u64,11177468454130666385_u64,12784069758082913911_u64,4002369431114344946_u64,7188258884011142292_u64,16324289658080880430_u64];
_2 = _3.fld1;
Goto(bb2)
}
bb2 = {
_8 = 180_u8 as f64;
_9 = _8 as isize;
_5 = _3.fld3.1 as isize;
_17.fld1 = [(-128049584931551390643278391553739568993_i128),129090175907131551369164147327037047396_i128,5865971285699612765986121683551387844_i128,105738163941406733035831731220818765352_i128];
RET = -980309516_i32;
_3.fld3.0 = [20_i8];
_10 = _6 >> _6;
RET = (-584542688_i32) + (-969578872_i32);
_3.fld3.0 = [(-56_i8)];
_3.fld3.1 = '\u{776f8}' as u128;
_9 = 3781778643397230428_usize as isize;
_17.fld1 = [107714344609709582171208901111889972065_i128,(-63384676885445474126649924463589144679_i128),(-143277589515221537097326657886266357139_i128),114099915810439143579489652770119067933_i128];
_17.fld1 = [137435039123760139674667423890062676518_i128,108990766271375757587077135178417366470_i128,72493094035954128170315284156097060983_i128,(-26633385783755132438801966846016382191_i128)];
_5 = _10 >> _4;
_14 = _3.fld2;
_17.fld0 = [47179144503049974979680072067343637536_i128,80670721158207516790822861990971560649_i128,(-14271189250123917162825063405187183333_i128),101911844696019915286142488730437628571_i128,(-144920716891591032508477185802217834321_i128),9889998826714136822782341150038149957_i128];
_19 = (_1,);
_3.fld0 = !_3.fld3.2;
_17.fld2 = core::ptr::addr_of_mut!(_19);
_3.fld3.2 = !_3.fld0;
_7 = 240_u8 as isize;
_13.0 = _3.fld2.0;
_17.fld0 = [(-28107139232086930026868044974792748505_i128),(-457817886676943303162868379337822825_i128),14578327753887646231690762392930891367_i128,(-168682191641181956750814842990577389926_i128),84612961503094664337323048091152342640_i128,(-126779740766478924998786312805006294836_i128)];
Goto(bb3)
}
bb3 = {
_17.fld0 = [(-79658645739352156095339826218236204812_i128),(-130196283440319275599200308525967304998_i128),98145240768244264094432491655371290172_i128,129794129882657550055244474389788419120_i128,(-53122303883468505353080866243098836302_i128),85124922508186722284079089584328852455_i128];
_14 = (_3.fld2.0,);
_8 = (-59_i8) as f64;
_4 = _3.fld3.2 as isize;
_14 = (_3.fld2.0,);
_3.fld3.2 = _3.fld0;
_3.fld0 = _19.0;
_3.fld3.1 = !275391068628741938622571359552529691026_u128;
_19.0 = !_3.fld3.2;
_3.fld2 = _14;
_8 = 107_i8 as f64;
_12 = [_19.0];
_19.0 = !_3.fld3.2;
_3.fld0 = _13.0 >= _3.fld2.0;
_4 = !_10;
_1 = !_3.fld3.2;
_19.0 = _3.fld3.2 ^ _3.fld3.2;
_3.fld3.0 = [54_i8];
_10 = !_6;
_17.fld2 = core::ptr::addr_of_mut!(_19);
_9 = -_5;
_14.0 = _3.fld2.0;
match _14.0 {
0 => bb4,
1 => bb5,
2 => bb6,
2137318739 => bb8,
_ => bb7
}
}
bb4 = {
_8 = 180_u8 as f64;
_9 = _8 as isize;
_5 = _3.fld3.1 as isize;
_17.fld1 = [(-128049584931551390643278391553739568993_i128),129090175907131551369164147327037047396_i128,5865971285699612765986121683551387844_i128,105738163941406733035831731220818765352_i128];
RET = -980309516_i32;
_3.fld3.0 = [20_i8];
_10 = _6 >> _6;
RET = (-584542688_i32) + (-969578872_i32);
_3.fld3.0 = [(-56_i8)];
_3.fld3.1 = '\u{776f8}' as u128;
_9 = 3781778643397230428_usize as isize;
_17.fld1 = [107714344609709582171208901111889972065_i128,(-63384676885445474126649924463589144679_i128),(-143277589515221537097326657886266357139_i128),114099915810439143579489652770119067933_i128];
_17.fld1 = [137435039123760139674667423890062676518_i128,108990766271375757587077135178417366470_i128,72493094035954128170315284156097060983_i128,(-26633385783755132438801966846016382191_i128)];
_5 = _10 >> _4;
_14 = _3.fld2;
_17.fld0 = [47179144503049974979680072067343637536_i128,80670721158207516790822861990971560649_i128,(-14271189250123917162825063405187183333_i128),101911844696019915286142488730437628571_i128,(-144920716891591032508477185802217834321_i128),9889998826714136822782341150038149957_i128];
_19 = (_1,);
_3.fld0 = !_3.fld3.2;
_17.fld2 = core::ptr::addr_of_mut!(_19);
_3.fld3.2 = !_3.fld0;
_7 = 240_u8 as isize;
_13.0 = _3.fld2.0;
_17.fld0 = [(-28107139232086930026868044974792748505_i128),(-457817886676943303162868379337822825_i128),14578327753887646231690762392930891367_i128,(-168682191641181956750814842990577389926_i128),84612961503094664337323048091152342640_i128,(-126779740766478924998786312805006294836_i128)];
Goto(bb3)
}
bb5 = {
RET = (-118987205_i32);
_3.fld2 = (2137318739_u32,);
_3.fld3.1 = 202327806823425329556605462945419060372_u128 | 156884995303480620377875795220257341212_u128;
_3.fld3.0 = [75_i8];
_2 = _3.fld1;
_13.0 = !_3.fld2.0;
_7 = 2_usize as isize;
_13 = (_3.fld2.0,);
_14.0 = _3.fld2.0 | _13.0;
_17.fld0 = [91413877876276260601499239795706320224_i128,95580712599986419268144075496377048892_i128,(-156207413074302133987518913633856631322_i128),134491896615456287106515116354707068067_i128,(-140575675948681065804850682565973710551_i128),(-161327162400770251832922067461485431724_i128)];
_1 = _10 >= _10;
_2 = [14330455871925835829_u64,17455367604195064664_u64,761261012627457309_u64,11177468454130666385_u64,12784069758082913911_u64,4002369431114344946_u64,7188258884011142292_u64,16324289658080880430_u64];
_2 = _3.fld1;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_17.fld0 = [(-92554192631519543137444768191915688140_i128),(-123687883775455263719892195713378578360_i128),147767293848101656878161211880694983061_i128,142189091680129128921066127366746824697_i128,58199091421208881642024360065590303643_i128,109419988410454465103776284549655149573_i128];
_3.fld3.1 = 103_u8 as u128;
_3.fld0 = _1 | _19.0;
_14 = (_13.0,);
_13 = (_14.0,);
RET = 968619509_i32 | 363371117_i32;
match _14.0 {
2137318739 => bb10,
_ => bb9
}
}
bb9 = {
RET = (-118987205_i32);
_3.fld2 = (2137318739_u32,);
_3.fld3.1 = 202327806823425329556605462945419060372_u128 | 156884995303480620377875795220257341212_u128;
_3.fld3.0 = [75_i8];
_2 = _3.fld1;
_13.0 = !_3.fld2.0;
_7 = 2_usize as isize;
_13 = (_3.fld2.0,);
_14.0 = _3.fld2.0 | _13.0;
_17.fld0 = [91413877876276260601499239795706320224_i128,95580712599986419268144075496377048892_i128,(-156207413074302133987518913633856631322_i128),134491896615456287106515116354707068067_i128,(-140575675948681065804850682565973710551_i128),(-161327162400770251832922067461485431724_i128)];
_1 = _10 >= _10;
_2 = [14330455871925835829_u64,17455367604195064664_u64,761261012627457309_u64,11177468454130666385_u64,12784069758082913911_u64,4002369431114344946_u64,7188258884011142292_u64,16324289658080880430_u64];
_2 = _3.fld1;
Goto(bb2)
}
bb10 = {
_6 = _10;
_13 = _3.fld2;
_3.fld0 = !_19.0;
_20.fld0 = _17.fld0;
_3.fld2 = (_14.0,);
_9 = 76884021751341241424050326512797244024_i128 as isize;
_19.0 = _3.fld3.2;
_20.fld2 = _17.fld2;
match _3.fld2.0 {
0 => bb1,
1 => bb9,
2 => bb8,
2137318739 => bb11,
_ => bb4
}
}
bb11 = {
_3.fld0 = _19.0 | _3.fld3.2;
_22 = !_6;
_20.fld1 = [54223707869356485818379235157795324778_i128,(-98676767237564731675058348796794764224_i128),(-2252861066846470482105758849329209974_i128),(-86046785478236132105748037812624375392_i128)];
_20.fld2 = core::ptr::addr_of_mut!(_19);
_3.fld2.0 = _11 as u32;
_4 = _22;
_20.fld1 = [158151876894914494153963705148114397980_i128,34210660942946723685230262538114540109_i128,36851474412997139468857756271466758372_i128,(-19731788654325956396095751765442320646_i128)];
_3.fld2.0 = 135172405051247495323098711132622755448_i128 as u32;
_17 = _20;
_20.fld0 = _17.fld0;
Goto(bb12)
}
bb12 = {
_17.fld2 = _20.fld2;
_13 = (_14.0,);
_26 = _22 as i32;
_3.fld2 = _14;
Goto(bb13)
}
bb13 = {
_20.fld0 = [81792712894318825493234982660368755616_i128,(-166251862453666567279114357199862616325_i128),(-16155848896052025236133173755072497053_i128),(-166417003175655805433355759330260124928_i128),(-78667681461158939566237373734528341597_i128),(-165388986444634360576460115393319647418_i128)];
_6 = _5;
_13.0 = !_3.fld2.0;
match _11 {
0 => bb7,
1 => bb11,
2 => bb14,
340282366920938463463374607431768186777 => bb16,
_ => bb15
}
}
bb14 = {
_17.fld0 = [(-92554192631519543137444768191915688140_i128),(-123687883775455263719892195713378578360_i128),147767293848101656878161211880694983061_i128,142189091680129128921066127366746824697_i128,58199091421208881642024360065590303643_i128,109419988410454465103776284549655149573_i128];
_3.fld3.1 = 103_u8 as u128;
_3.fld0 = _1 | _19.0;
_14 = (_13.0,);
_13 = (_14.0,);
RET = 968619509_i32 | 363371117_i32;
match _14.0 {
2137318739 => bb10,
_ => bb9
}
}
bb15 = {
Return()
}
bb16 = {
_26 = RET + RET;
Goto(bb17)
}
bb17 = {
Call(_29 = dump_var(14_usize, 13_usize, Move(_13), 22_usize, Move(_22), 5_usize, Move(_5), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_29 = dump_var(14_usize, 9_usize, Move(_9), 2_usize, Move(_2), 6_usize, Move(_6), 30_usize, _30), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: isize,mut _2: isize,mut _3: u32,mut _4: isize,mut _5: ([i8; 1], u128, bool),mut _6: [i8; 1],mut _7: bool,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize) -> isize {
mir! {
type RET = isize;
let _13: i16;
let _14: i8;
let _15: isize;
let _16: isize;
let _17: isize;
let _18: Adt41;
let _19: Adt50;
let _20: char;
let _21: isize;
let _22: f32;
let _23: bool;
let _24: isize;
let _25: [i128; 6];
let _26: [i128; 4];
let _27: f32;
let _28: i128;
let _29: ();
let _30: ();
{
_9 = !_4;
_11 = 289347083606546979_u64 as isize;
_5.1 = 283172363367822382634008501365263627006_u128;
_5.1 = !198874769414790662505841849373327195260_u128;
_12 = _9;
RET = -_8;
_3 = 1414144147_u32;
_1 = 31138_i16 as isize;
_15 = _8 * _8;
_15 = _2;
_8 = 6856971018369723236_u64 as isize;
_10 = '\u{eae5b}' as isize;
_13 = (-129320786619845825044487536979282725334_i128) as i16;
Goto(bb1)
}
bb1 = {
RET = _2;
_2 = _4 << RET;
_8 = 105_i8 as isize;
_7 = !_5.2;
_2 = _4 + _12;
_3 = 3141419899_u32;
_16 = 1196818757260026311_u64 as isize;
_9 = _5.1 as isize;
_5.2 = _7;
_7 = _5.2;
_8 = !_2;
_14 = (-3_i8) - 87_i8;
Goto(bb2)
}
bb2 = {
_1 = _8;
_7 = _5.2 & _5.2;
_8 = _2;
_5.1 = '\u{171ad}' as u128;
_15 = !_2;
Goto(bb3)
}
bb3 = {
_13 = 26760_i16;
Call(_2 = fn16(_1, _12, _1, _5, _4, _15, _7, _8, RET, _8, _7, _8, _15), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_18 = Adt41::Variant0 { fld0: (-414877352_i32) };
_2 = _12;
_18 = Adt41::Variant0 { fld0: (-436917455_i32) };
_5.0 = [_14];
_21 = _1 << _2;
_16 = _13 as isize;
_12 = -_2;
place!(Field::<i32>(Variant(_18, 0), 0)) = 19117_u16 as i32;
_6 = _5.0;
_17 = _1 >> _1;
RET = _5.1 as isize;
_21 = !_10;
_21 = 1_usize as isize;
_20 = '\u{103e89}';
Goto(bb5)
}
bb5 = {
_14 = 46_i8;
_4 = _17;
_16 = _5.1 as isize;
_1 = -_15;
_4 = _17;
Goto(bb6)
}
bb6 = {
_24 = _5.1 as isize;
_23 = !_5.2;
_14 = -91_i8;
_9 = _17;
_9 = RET;
_8 = _7 as isize;
_20 = '\u{d71ee}';
place!(Field::<i32>(Variant(_18, 0), 0)) = (-1167750728_i32) - 131919046_i32;
_23 = _5.2;
_9 = !_1;
match _3 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
3141419899 => bb13,
_ => bb12
}
}
bb7 = {
_14 = 46_i8;
_4 = _17;
_16 = _5.1 as isize;
_1 = -_15;
_4 = _17;
Goto(bb6)
}
bb8 = {
_18 = Adt41::Variant0 { fld0: (-414877352_i32) };
_2 = _12;
_18 = Adt41::Variant0 { fld0: (-436917455_i32) };
_5.0 = [_14];
_21 = _1 << _2;
_16 = _13 as isize;
_12 = -_2;
place!(Field::<i32>(Variant(_18, 0), 0)) = 19117_u16 as i32;
_6 = _5.0;
_17 = _1 >> _1;
RET = _5.1 as isize;
_21 = !_10;
_21 = 1_usize as isize;
_20 = '\u{103e89}';
Goto(bb5)
}
bb9 = {
_13 = 26760_i16;
Call(_2 = fn16(_1, _12, _1, _5, _4, _15, _7, _8, RET, _8, _7, _8, _15), ReturnTo(bb4), UnwindUnreachable())
}
bb10 = {
_1 = _8;
_7 = _5.2 & _5.2;
_8 = _2;
_5.1 = '\u{171ad}' as u128;
_15 = !_2;
Goto(bb3)
}
bb11 = {
RET = _2;
_2 = _4 << RET;
_8 = 105_i8 as isize;
_7 = !_5.2;
_2 = _4 + _12;
_3 = 3141419899_u32;
_16 = 1196818757260026311_u64 as isize;
_9 = _5.1 as isize;
_5.2 = _7;
_7 = _5.2;
_8 = !_2;
_14 = (-3_i8) - 87_i8;
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
_21 = _8;
_14 = !91_i8;
_24 = !_4;
_11 = !_4;
_26 = [158690743558210431158115568217674324523_i128,(-162395583683551912827836824499955284964_i128),145348761421379366806810346565793286194_i128,(-6337663324687639880871696227814288969_i128)];
Goto(bb14)
}
bb14 = {
_5.1 = _7 as u128;
_3 = 250_u8 as u32;
_27 = Field::<i32>(Variant(_18, 0), 0) as f32;
_9 = _24;
place!(Field::<i32>(Variant(_18, 0), 0)) = (-1094318560_i32) * 1615695641_i32;
_9 = _5.1 as isize;
_19 = Adt50::Variant0 { fld0: _3,fld1: _20 };
_22 = _27;
_4 = _27 as isize;
_5 = (_6, 13124017298440637803804454251260239756_u128, _7);
_12 = _22 as isize;
place!(Field::<char>(Variant(_19, 0), 1)) = _20;
_5.2 = !_7;
_21 = _9;
_25 = [16619677172973510711838233694012662791_i128,(-156770271770818537631493865169413793085_i128),45263547456271131143883819128230281746_i128,66762502691160599425446845120505675744_i128,169244500478074573336747621212979351747_i128,(-93859082054942970561519732563391086293_i128)];
_2 = _8;
_16 = -_15;
_1 = -_24;
_23 = _7;
_8 = !_1;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(15_usize, 20_usize, Move(_20), 6_usize, Move(_6), 1_usize, Move(_1), 25_usize, Move(_25)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(15_usize, 17_usize, Move(_17), 13_usize, Move(_13), 4_usize, Move(_4), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(15_usize, 26_usize, Move(_26), 16_usize, Move(_16), 24_usize, Move(_24), 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: ([i8; 1], u128, bool),mut _5: isize,mut _6: isize,mut _7: bool,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: bool,mut _12: isize,mut _13: isize) -> isize {
mir! {
type RET = isize;
let _14: Adt49;
let _15: bool;
let _16: Adt50;
let _17: Adt44;
let _18: i32;
let _19: ();
let _20: ();
{
_2 = 40_u8 as isize;
_4.1 = _6 as u128;
_9 = _8 * _8;
_5 = -_13;
RET = !_1;
_9 = (-1689082753_i32) as isize;
RET = _8;
_7 = _11;
_15 = _7;
_4.0 = [(-120_i8)];
_2 = _8;
_9 = _6;
_1 = (-1425306878548720419_i64) as isize;
RET = _10;
_10 = 14064764020035781373_usize as isize;
RET = _8 * _5;
RET = -_9;
RET = _15 as isize;
_12 = _13 << _9;
RET = _8;
_5 = RET;
_10 = _15 as isize;
RET = _5 << _4.1;
_12 = !_5;
_17 = Adt44::Variant2 { fld0: 8026631430475394350_usize };
_6 = (-146971426892186287876561358463479565973_i128) as isize;
_10 = _1;
Goto(bb1)
}
bb1 = {
Call(_19 = dump_var(16_usize, 15_usize, Move(_15), 11_usize, Move(_11), 2_usize, Move(_2), 13_usize, Move(_13)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_19 = dump_var(16_usize, 8_usize, Move(_8), 4_usize, Move(_4), 10_usize, Move(_10), 20_usize, _20), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: ([i8; 1], u128, bool),mut _2: u32,mut _3: bool,mut _4: isize,mut _5: [i64; 1],mut _6: [usize; 7]) -> [isize; 8] {
mir! {
type RET = [isize; 8];
let _7: f32;
let _8: i16;
let _9: ();
let _10: ();
{
_6 = [15455718089629652785_usize,10294883082128040688_usize,6_usize,269939347661299024_usize,12145709691021573142_usize,16569246698331396451_usize,2_usize];
_5 = [(-6472214300824411931_i64)];
_7 = _4 as f32;
RET = [_4,_4,_4,_4,_4,_4,_4,_4];
RET = [_4,_4,_4,_4,_4,_4,_4,_4];
RET = [_4,_4,_4,_4,_4,_4,_4,_4];
_2 = 1892147264_u32 | 296788705_u32;
_5 = [(-5910275504233888124_i64)];
_1.0 = [(-21_i8)];
_1.2 = _3 | _3;
_3 = _4 < _4;
_8 = '\u{ee7fe}' as i16;
_1.1 = 50644005302286943746684983764482472208_u128 + 123004354322920841691235214690564591913_u128;
RET = [_4,_4,_4,_4,_4,_4,_4,_4];
_4 = _8 as isize;
_4 = _3 as isize;
_3 = _1.2;
_8 = 17000_i16 & (-31892_i16);
_8 = (-733_i16) * 4277_i16;
_4 = (-8_isize) * (-57_isize);
_7 = _1.1 as f32;
Goto(bb1)
}
bb1 = {
Call(_9 = dump_var(17_usize, 3_usize, Move(_3), 2_usize, Move(_2), 6_usize, Move(_6), 10_usize, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn18(mut _1: ([i8; 1], u128, bool),mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: [u64; 8],mut _7: bool,mut _8: ([i8; 1], u128, bool),mut _9: bool,mut _10: bool,mut _11: bool,mut _12: bool) -> bool {
mir! {
type RET = bool;
let _13: ();
let _14: ();
{
_4 = _1.2 ^ _2;
_1 = (_8.0, _8.1, _3);
_5 = _9 | _1.2;
_1.2 = !_12;
_8.1 = !_1.1;
_5 = _10;
RET = !_8.2;
_1.2 = !_4;
_1 = (_8.0, _8.1, _11);
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(18_usize, 8_usize, Move(_8), 1_usize, Move(_1), 4_usize, Move(_4), 5_usize, Move(_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_13 = dump_var(18_usize, 6_usize, Move(_6), 9_usize, Move(_9), 14_usize, _14, 14_usize, _14), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn19(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: u16,mut _5: usize,mut _6: isize,mut _7: u16) -> [bool; 1] {
mir! {
type RET = [bool; 1];
let _8: Adt40;
let _9: (u16,);
let _10: [char; 6];
let _11: [u16; 5];
let _12: isize;
let _13: [char; 6];
let _14: ([i8; 1], u128, bool);
let _15: Adt49;
let _16: [bool; 1];
let _17: [u64; 8];
let _18: Adt44;
let _19: *mut (bool,);
let _20: [i64; 1];
let _21: bool;
let _22: f64;
let _23: isize;
let _24: isize;
let _25: Adt52;
let _26: [u16; 5];
let _27: Adt54;
let _28: f32;
let _29: (u16,);
let _30: [usize; 5];
let _31: f64;
let _32: Adt51;
let _33: Adt44;
let _34: u16;
let _35: usize;
let _36: [i128; 4];
let _37: ([i8; 1], u128, bool);
let _38: f64;
let _39: ();
let _40: ();
{
RET = [false];
_10 = ['\u{3440c}','\u{10a0e}','\u{34ef8}','\u{ee54e}','\u{acd53}','\u{7a3ff}'];
RET = [false];
_5 = 2137888218_i32 as usize;
_11 = [_4,_4,_4,_4,_7];
_2 = -_1;
_12 = _2 << _6;
_14.0 = [115_i8];
_1 = _12;
_9.0 = _7;
_1 = _2 & _6;
_14.0 = [(-71_i8)];
_10 = ['\u{5e786}','\u{53fa7}','\u{126d3}','\u{2f422}','\u{8b752}','\u{32262}'];
RET = [false];
_3 = !_12;
match _6 {
71 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_14.2 = false | true;
_11 = [_7,_7,_9.0,_7,_7];
_7 = !_4;
_13 = ['\u{6ee4b}','\u{28c18}','\u{cfb7c}','\u{a457f}','\u{c97e0}','\u{10b50e}'];
RET = [_14.2];
_9.0 = _4 << _1;
Goto(bb3)
}
bb3 = {
_12 = -_1;
_5 = _9.0 as usize;
_13 = _10;
_14.0 = [(-107_i8)];
_14.1 = 127195818443047235678497980652586811447_u128;
_16 = [_14.2];
_14.1 = 81200495853926684118412139473138711023_i128 as u128;
_7 = _9.0 - _4;
_6 = _12 & _2;
_16 = [_14.2];
_10 = _13;
_13 = _10;
RET = [_14.2];
_6 = -_1;
_14.2 = false;
_10 = ['\u{82ddb}','\u{16d2f}','\u{9f25a}','\u{a38e9}','\u{5d8c0}','\u{df05b}'];
_13 = _10;
RET = _16;
_14.0 = [(-125_i8)];
Goto(bb4)
}
bb4 = {
_10 = ['\u{104aae}','\u{352d2}','\u{5e579}','\u{73440}','\u{e9458}','\u{e0f05}'];
_8 = Adt40::Variant0 { fld0: _9,fld1: _11,fld2: _3,fld3: _14 };
place!(Field::<isize>(Variant(_8, 0), 2)) = !_3;
SetDiscriminant(_8, 1);
RET = _16;
_17 = [5002950792190682262_u64,11755692784705907376_u64,2027606000920605312_u64,5079234102211843369_u64,6841068465882099963_u64,2992319226758322049_u64,13239122509945848175_u64,17353967370994452368_u64];
_8 = Adt40::Variant0 { fld0: _9,fld1: _11,fld2: _12,fld3: _14 };
_20 = [6838230479004306803_i64];
_13 = ['\u{d2350}','\u{17bf}','\u{ecf74}','\u{38857}','\u{c2114}','\u{d9f18}'];
SetDiscriminant(_8, 1);
_14.0 = [(-74_i8)];
_6 = 828287363_u32 as isize;
place!(Field::<f32>(Variant(_8, 1), 3)) = (-803_i16) as f32;
_16 = [_14.2];
_16 = [_14.2];
Goto(bb5)
}
bb5 = {
_21 = _7 <= _4;
_6 = _2;
_13 = _10;
place!(Field::<usize>(Variant(_8, 1), 1)) = (-38_i8) as usize;
place!(Field::<[usize; 5]>(Variant(_8, 1), 4)) = [_5,_5,_5,Field::<usize>(Variant(_8, 1), 1),_5];
_17 = [3666136748909131314_u64,2107091061490445752_u64,16623916087417337655_u64,15962274881579860994_u64,16817479585273293895_u64,3406390086447565947_u64,12379294119563453193_u64,3125682889183615350_u64];
place!(Field::<[usize; 5]>(Variant(_8, 1), 4)) = [_5,_5,_5,_5,Field::<usize>(Variant(_8, 1), 1)];
_12 = 1395023368_i32 as isize;
_3 = _6;
_22 = (-72_i8) as f64;
place!(Field::<[usize; 5]>(Variant(_8, 1), 4)) = [Field::<usize>(Variant(_8, 1), 1),_5,_5,Field::<usize>(Variant(_8, 1), 1),_5];
_14.1 = 917021390_i32 as u128;
_1 = _3;
place!(Field::<f64>(Variant(_8, 1), 5)) = _22;
place!(Field::<f32>(Variant(_8, 1), 3)) = (-50_i8) as f32;
_13 = _10;
_17 = [14437294683378107540_u64,5344578661092356381_u64,9860639146385792055_u64,1984156567340116920_u64,3407700028430224489_u64,16999423255567696704_u64,7661147487512045370_u64,7648327011152933898_u64];
_9 = (_7,);
place!(Field::<bool>(Variant(_8, 1), 0)) = _21 != _21;
place!(Field::<f32>(Variant(_8, 1), 3)) = 15094642448296669098_u64 as f32;
_9 = (_7,);
Goto(bb6)
}
bb6 = {
_25.fld3 = _14;
_6 = -_3;
place!(Field::<bool>(Variant(_8, 1), 0)) = !_21;
_5 = _22 as usize;
_25.fld2 = (3762461787_u32,);
_25.fld2.0 = 1925017519_u32 >> _5;
place!(Field::<usize>(Variant(_8, 1), 1)) = 3670659854608345058_u64 as usize;
_24 = _3 >> _25.fld3.1;
place!(Field::<[i128; 4]>(Variant(_8, 1), 2)) = [(-50642519640811926328765314590645249791_i128),(-12978146198932071546212676449680564967_i128),43329127194738115841102439868966436336_i128,31027305347698421509565335155522965144_i128];
_11 = [_7,_9.0,_9.0,_9.0,_7];
_24 = _6;
_18 = Adt44::Variant2 { fld0: _5 };
RET = [_21];
place!(Field::<f64>(Variant(_8, 1), 5)) = _22;
_25.fld0 = _7 >= _9.0;
_25.fld2 = (2269882936_u32,);
_12 = 25676_i16 as isize;
_25.fld2.0 = 2120637331_u32 << _25.fld3.1;
_25.fld0 = !Field::<bool>(Variant(_8, 1), 0);
place!(Field::<f64>(Variant(_8, 1), 5)) = _22 * _22;
_26 = [_7,_7,_4,_9.0,_7];
_17 = [14394296052670710939_u64,2849773630762885585_u64,2239865319296051764_u64,1889753460677273882_u64,1765147215104316413_u64,6449615634333836668_u64,17461784055074263146_u64,5760251370876442_u64];
_12 = '\u{10c641}' as isize;
Goto(bb7)
}
bb7 = {
_25.fld2.0 = Field::<f32>(Variant(_8, 1), 3) as u32;
_25.fld3 = _14;
_1 = _6 - _3;
_27.fld5 = Move(_8);
_27.fld4.fld0 = !_25.fld0;
_27.fld0 = Field::<bool>(Variant(_27.fld5, 1), 0);
_25.fld2 = (3813712701_u32,);
_28 = -Field::<f32>(Variant(_27.fld5, 1), 3);
_14 = _25.fld3;
_23 = _25.fld2.0 as isize;
_27.fld2 = Move(_18);
_7 = !_9.0;
_1 = _25.fld0 as isize;
_27.fld4.fld3.1 = _5 as u128;
_14.1 = (-2108719801_i32) as u128;
_27.fld4.fld0 = Field::<bool>(Variant(_27.fld5, 1), 0);
place!(Field::<usize>(Variant(_27.fld2, 2), 0)) = 1351101407_i32 as usize;
_29 = (_7,);
_27.fld4.fld2 = (_25.fld2.0,);
_27.fld4 = Adt52 { fld0: _25.fld0,fld1: _17,fld2: _25.fld2,fld3: _14 };
match _25.fld2.0 {
3813712701 => bb9,
_ => bb8
}
}
bb8 = {
_25.fld3 = _14;
_6 = -_3;
place!(Field::<bool>(Variant(_8, 1), 0)) = !_21;
_5 = _22 as usize;
_25.fld2 = (3762461787_u32,);
_25.fld2.0 = 1925017519_u32 >> _5;
place!(Field::<usize>(Variant(_8, 1), 1)) = 3670659854608345058_u64 as usize;
_24 = _3 >> _25.fld3.1;
place!(Field::<[i128; 4]>(Variant(_8, 1), 2)) = [(-50642519640811926328765314590645249791_i128),(-12978146198932071546212676449680564967_i128),43329127194738115841102439868966436336_i128,31027305347698421509565335155522965144_i128];
_11 = [_7,_9.0,_9.0,_9.0,_7];
_24 = _6;
_18 = Adt44::Variant2 { fld0: _5 };
RET = [_21];
place!(Field::<f64>(Variant(_8, 1), 5)) = _22;
_25.fld0 = _7 >= _9.0;
_25.fld2 = (2269882936_u32,);
_12 = 25676_i16 as isize;
_25.fld2.0 = 2120637331_u32 << _25.fld3.1;
_25.fld0 = !Field::<bool>(Variant(_8, 1), 0);
place!(Field::<f64>(Variant(_8, 1), 5)) = _22 * _22;
_26 = [_7,_7,_4,_9.0,_7];
_17 = [14394296052670710939_u64,2849773630762885585_u64,2239865319296051764_u64,1889753460677273882_u64,1765147215104316413_u64,6449615634333836668_u64,17461784055074263146_u64,5760251370876442_u64];
_12 = '\u{10c641}' as isize;
Goto(bb7)
}
bb9 = {
SetDiscriminant(_27.fld2, 3);
_27.fld4.fld3.0 = [(-29_i8)];
place!(Field::<(bool,)>(Variant(_27.fld2, 3), 0)) = (_27.fld4.fld0,);
_27.fld4.fld2.0 = _5 as u32;
place!(Field::<[usize; 5]>(Variant(_27.fld5, 1), 4)) = [Field::<usize>(Variant(_27.fld5, 1), 1),Field::<usize>(Variant(_27.fld5, 1), 1),_5,_5,Field::<usize>(Variant(_27.fld5, 1), 1)];
_30 = [Field::<usize>(Variant(_27.fld5, 1), 1),_5,Field::<usize>(Variant(_27.fld5, 1), 1),Field::<usize>(Variant(_27.fld5, 1), 1),Field::<usize>(Variant(_27.fld5, 1), 1)];
_11 = [_29.0,_29.0,_9.0,_9.0,_7];
_32.fld5 = _10;
_14.2 = _25.fld0;
place!(Field::<bool>(Variant(_27.fld5, 1), 0)) = !_14.2;
place!(Field::<usize>(Variant(_27.fld5, 1), 1)) = !_5;
_25.fld3.0 = _27.fld4.fld3.0;
_28 = 12411187394466483877_u64 as f32;
_14 = (_25.fld3.0, _25.fld3.1, _21);
_24 = _23 - _23;
_27.fld4.fld2 = (_25.fld2.0,);
_31 = Field::<f64>(Variant(_27.fld5, 1), 5) - _22;
_29.0 = 2098926959_i32 as u16;
_26 = _11;
_25 = Move(_27.fld4);
match _25.fld2.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb7,
5 => bb6,
6 => bb10,
3813712701 => bb12,
_ => bb11
}
}
bb10 = {
_10 = ['\u{104aae}','\u{352d2}','\u{5e579}','\u{73440}','\u{e9458}','\u{e0f05}'];
_8 = Adt40::Variant0 { fld0: _9,fld1: _11,fld2: _3,fld3: _14 };
place!(Field::<isize>(Variant(_8, 0), 2)) = !_3;
SetDiscriminant(_8, 1);
RET = _16;
_17 = [5002950792190682262_u64,11755692784705907376_u64,2027606000920605312_u64,5079234102211843369_u64,6841068465882099963_u64,2992319226758322049_u64,13239122509945848175_u64,17353967370994452368_u64];
_8 = Adt40::Variant0 { fld0: _9,fld1: _11,fld2: _12,fld3: _14 };
_20 = [6838230479004306803_i64];
_13 = ['\u{d2350}','\u{17bf}','\u{ecf74}','\u{38857}','\u{c2114}','\u{d9f18}'];
SetDiscriminant(_8, 1);
_14.0 = [(-74_i8)];
_6 = 828287363_u32 as isize;
place!(Field::<f32>(Variant(_8, 1), 3)) = (-803_i16) as f32;
_16 = [_14.2];
_16 = [_14.2];
Goto(bb5)
}
bb11 = {
_25.fld2.0 = Field::<f32>(Variant(_8, 1), 3) as u32;
_25.fld3 = _14;
_1 = _6 - _3;
_27.fld5 = Move(_8);
_27.fld4.fld0 = !_25.fld0;
_27.fld0 = Field::<bool>(Variant(_27.fld5, 1), 0);
_25.fld2 = (3813712701_u32,);
_28 = -Field::<f32>(Variant(_27.fld5, 1), 3);
_14 = _25.fld3;
_23 = _25.fld2.0 as isize;
_27.fld2 = Move(_18);
_7 = !_9.0;
_1 = _25.fld0 as isize;
_27.fld4.fld3.1 = _5 as u128;
_14.1 = (-2108719801_i32) as u128;
_27.fld4.fld0 = Field::<bool>(Variant(_27.fld5, 1), 0);
place!(Field::<usize>(Variant(_27.fld2, 2), 0)) = 1351101407_i32 as usize;
_29 = (_7,);
_27.fld4.fld2 = (_25.fld2.0,);
_27.fld4 = Adt52 { fld0: _25.fld0,fld1: _17,fld2: _25.fld2,fld3: _14 };
match _25.fld2.0 {
3813712701 => bb9,
_ => bb8
}
}
bb12 = {
_27.fld4.fld1 = _25.fld1;
_23 = _6;
Goto(bb13)
}
bb13 = {
_7 = _9.0 * _9.0;
_33 = Adt44::Variant3 { fld0: Field::<(bool,)>(Variant(_27.fld2, 3), 0) };
_32.fld2 = _25.fld2.0;
_27.fld4.fld3.2 = _27.fld0 | _27.fld0;
_12 = !_6;
_10 = _13;
_32.fld3 = !111_i8;
_27.fld4.fld2 = (_25.fld2.0,);
SetDiscriminant(_27.fld2, 2);
_25.fld3 = (_14.0, _14.1, _14.2);
_2 = _1;
SetDiscriminant(_33, 2);
_27.fld4.fld0 = _25.fld0;
_7 = 6381681825308835142_u64 as u16;
_14.0 = [_32.fld3];
_3 = _2;
_32.fld6 = _9.0;
Goto(bb14)
}
bb14 = {
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(19_usize, 4_usize, Move(_4), 10_usize, Move(_10), 5_usize, Move(_5), 30_usize, Move(_30)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(19_usize, 1_usize, Move(_1), 6_usize, Move(_6), 23_usize, Move(_23), 29_usize, Move(_29)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(19_usize, 21_usize, Move(_21), 16_usize, Move(_16), 7_usize, Move(_7), 40_usize, _40), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(4_usize), std::hint::black_box(51832025091845404993946352697986177123_i128), std::hint::black_box(28_isize), std::hint::black_box((-1627365603_i32)), std::hint::black_box(14474_i16));
                
            }
impl PrintFDebug for Adt39{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt39::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt39 {
Variant0{
fld0: [isize; 8],
fld1: u64,
fld2: [u64; 8],
fld3: ([i8; 1], u128, bool),
fld4: [i64; 1],

},
Variant1{
fld0: [i8; 1],
fld1: usize,
fld2: [isize; 8],
fld3: i64,
fld4: u8,
fld5: [i16; 3],

},
Variant2{
fld0: [i128; 6],
fld1: [i16; 3],
fld2: isize,
fld3: i8,
fld4: i16,

}}
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt40::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: (u16,),
fld1: [u16; 5],
fld2: isize,
fld3: ([i8; 1], u128, bool),

},
Variant1{
fld0: bool,
fld1: usize,
fld2: [i128; 4],
fld3: f32,
fld4: [usize; 5],
fld5: f64,

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt41::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt41 {
Variant0{
fld0: i32,

},
Variant1{
fld0: u8,
fld1: u128,
fld2: isize,
fld3: Adt40,
fld4: [bool; 1],

},
Variant2{
fld0: i128,
fld1: [u64; 8],
fld2: isize,
fld3: [i64; 1],
fld4: [char; 6],
fld5: i32,
fld6: usize,

}}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt42::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: u8,
fld1: char,
fld2: u128,
fld3: [char; 6],
fld4: [usize; 5],
fld5: [bool; 1],
fld6: [isize; 8],

},
Variant1{
fld0: [i64; 1],
fld1: ([i8; 1], u128, bool),
fld2: [i8; 1],
fld3: [i16; 3],
fld4: f64,
fld5: *const f32,
fld6: (u32,),

},
Variant2{
fld0: u8,
fld1: ([i8; 1], u128, bool),
fld2: [isize; 8],
fld3: (u32,),
fld4: i16,

},
Variant3{
fld0: (u16,),
fld1: i16,
fld2: [i64; 1],

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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: Adt42,
fld1: Adt39,
fld2: [i8; 1],
fld3: i128,
fld4: [usize; 7],
fld5: [isize; 8],
fld6: *mut (bool,),

},
Variant1{
fld0: bool,
fld1: u8,
fld2: [usize; 5],
fld3: i8,
fld4: u64,
fld5: u128,

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
	Self::Variant2{fld0,}=>{
unsafe{printf(c"Variant2{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
	Self::Variant3{fld0,}=>{
unsafe{printf(c"Variant3{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: bool,
fld1: f32,
fld2: [i8; 1],
fld3: i8,
fld4: *const f32,
fld5: (u32,),
fld6: ([i8; 1], u128, bool),
fld7: Adt41,

},
Variant1{
fld0: [usize; 7],
fld1: [i128; 6],
fld2: (u32,),

},
Variant2{
fld0: usize,

},
Variant3{
fld0: (bool,),

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt45::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: usize,
fld1: *mut (bool,),
fld2: isize,
fld3: [usize; 7],

},
Variant1{
fld0: Adt40,
fld1: char,
fld2: u16,
fld3: (bool,),

},
Variant2{
fld0: i32,
fld1: i16,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: *const f32,

},
Variant1{
fld0: i128,
fld1: f32,

},
Variant2{
fld0: (bool,),
fld1: Adt45,
fld2: [i64; 1],
fld3: Adt41,

},
Variant3{
fld0: u16,
fld1: [isize; 8],
fld2: *mut (bool,),
fld3: f32,
fld4: [usize; 5],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: bool,
fld1: (u32,),
fld2: Adt41,
fld3: u8,

},
Variant1{
fld0: Adt42,
fld1: (u32,),
fld2: f64,
fld3: [u64; 8],
fld4: [i128; 6],

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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: [i8; 1],
fld1: *const f32,
fld2: (u16,),
fld3: (u32,),
fld4: u32,
fld5: i32,
fld6: [usize; 5],
fld7: f64,

},
Variant1{
fld0: Adt42,

},
Variant2{
fld0: i64,
fld1: char,

},
Variant3{
fld0: bool,
fld1: [usize; 5],

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: Adt40,
fld1: f64,
fld2: u32,
fld3: i8,

},
Variant1{
fld0: [char; 6],
fld1: [i16; 3],
fld2: [i128; 4],
fld3: [i128; 6],
fld4: *const f32,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf(c"Variant0{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
		unsafe{printf(c"fld1:".as_ptr())};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: u32,
fld1: char,

},
Variant1{
fld0: f32,
fld1: [isize; 8],
fld2: (u32,),
fld3: [i16; 3],
fld4: i16,

},
Variant2{
fld0: (u32,),
fld1: [char; 6],
fld2: Adt41,
fld3: *const f32,
fld4: [i64; 1],
fld5: u128,
fld6: u16,
fld7: Adt42,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: u64,
fld1: [u64; 8],
fld2: u32,
fld3: i8,
fld4: [i16; 3],
fld5: [char; 6],
fld6: u16,
fld7: Adt45,
}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: bool,
fld1: [u64; 8],
fld2: (u32,),
fld3: ([i8; 1], u128, bool),
}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: [i128; 6],
fld1: [i128; 4],
fld2: *mut (bool,),
}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt54{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt54 {
fld0: bool,
fld1: Adt39,
fld2: Adt44,
fld3: Adt45,
fld4: Adt52,
fld5: Adt40,
}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: [char; 6],
fld1: Adt41,
fld2: [usize; 7],
fld3: i8,
fld4: i16,
fld5: [i64; 1],

},
Variant1{
fld0: (u32,),
fld1: Adt52,
fld2: u64,
fld3: Adt39,
fld4: *const f32,
fld5: [i8; 1],

},
Variant2{
fld0: [u64; 8],

}}

