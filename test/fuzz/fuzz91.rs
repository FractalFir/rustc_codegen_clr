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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: u128,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64) -> u128 {
mir! {
type RET = u128;
let _14: (u32, i16);
let _15: ([u64; 4], u32, u8);
let _16: u64;
let _17: i16;
let _18: u16;
let _19: u128;
let _20: Adt46;
let _21: ([u64; 4], u32, u8);
let _22: [u64; 6];
let _23: [i128; 3];
let _24: i128;
let _25: Adt60;
let _26: f64;
let _27: usize;
let _28: [u8; 7];
let _29: ();
let _30: ();
{
_13 = 4051776927588545585_u64 >> 33196764565105869114274528942821687488_i128;
RET = !80042481656530314875600322686681484656_u128;
_4 = 98_i8 * 91_i8;
_9 = RET;
Goto(bb1)
}
bb1 = {
_11 = 37104_u16 * 55514_u16;
_12 = 1875511884_u32;
_14.1 = !15678_i16;
_2 = '\u{3161e}';
_6 = _2 as i32;
RET = _9;
_8 = 24261581525741195548914104053691782736_i128;
_1 = false & true;
_14 = (_12, 28774_i16);
_6 = 1857587117_i32 * (-409276260_i32);
_11 = (-9223372036854775808_isize) as u16;
_2 = '\u{1ddd6}';
_5 = _14.1;
_8 = !62592521368598028745044470934068839439_i128;
_6 = (-1376912701_i32);
_7 = 29_u8 as i64;
_9 = _2 as u128;
_3 = _12 as isize;
_8 = !(-43257281527007134621286188176808528899_i128);
_9 = _6 as u128;
_12 = _11 as u32;
_12 = 4885846221108632911_usize as u32;
RET = _9;
RET = _9 ^ _9;
_7 = (-8075468452452640124_i64) ^ 3122392434041163954_i64;
_14 = (_12, _5);
_11 = 16432_u16;
match _14.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
28774 => bb10,
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
_1 = false;
_6 = 680711622_i32;
_14 = (_12, _5);
Call(_15.1 = fn1(_1, _3, _3, _5, _13, _1, _5, _7, _6, _5), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_17 = -_5;
_16 = _13 >> _11;
_17 = _14.1;
_18 = _6 as u16;
_14.0 = _15.1 & _15.1;
_15.0 = [_16,_13,_13,_13];
_1 = false;
_18 = !_11;
_20.fld0 = _8 as u8;
_21.0 = [_13,_13,_16,_16];
_14.0 = !_12;
_15 = (_21.0, _12, _20.fld0);
_20.fld0 = _15.2 + _15.2;
RET = !_9;
Goto(bb12)
}
bb12 = {
_14.1 = _17;
RET = _17 as u128;
_20.fld0 = _3 as u8;
_20.fld0 = !_15.2;
_12 = _7 as u32;
match _14.1 {
0 => bb9,
28774 => bb13,
_ => bb11
}
}
bb13 = {
_7 = 5419679890389181650_i64 - 7277450878423105078_i64;
_2 = '\u{911ae}';
_22 = [_13,_16,_16,_16,_13,_16];
_12 = _8 as u32;
_8 = 125458382051542081853582548233069043579_i128;
_9 = RET * RET;
_22 = [_13,_13,_16,_13,_16,_16];
RET = !_9;
_14 = (_15.1, _17);
_21.0 = [_16,_16,_16,_13];
Call(_10 = core::intrinsics::bswap(_15.2), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_15.1 = _2 as u32;
_14.1 = _5;
_15 = (_21.0, _12, _10);
_11 = !_18;
_14.1 = _17;
_21.2 = RET as u8;
_20 = Adt46 { fld0: _21.2 };
_2 = '\u{aa0bf}';
_15 = (_21.0, _14.0, _10);
_15.0 = [_16,_16,_13,_13];
_4 = _11 as i8;
_26 = _14.1 as f64;
_13 = _4 as u64;
_14.0 = 17399506205874511390_usize as u32;
RET = !_9;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(0_usize, 3_usize, Move(_3), 12_usize, Move(_12), 6_usize, Move(_6), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(0_usize, 2_usize, Move(_2), 10_usize, Move(_10), 14_usize, Move(_14), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(0_usize, 13_usize, Move(_13), 30_usize, _30, 30_usize, _30, 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: bool,mut _2: isize,mut _3: isize,mut _4: i16,mut _5: u64,mut _6: bool,mut _7: i16,mut _8: i64,mut _9: i32,mut _10: i16) -> u32 {
mir! {
type RET = u32;
let _11: u16;
let _12: [usize; 6];
let _13: usize;
let _14: ([u64; 4], u32, u8);
let _15: i32;
let _16: u16;
let _17: usize;
let _18: Adt56;
let _19: isize;
let _20: *const (&'static u16, f64);
let _21: *mut &'static u16;
let _22: *mut u32;
let _23: bool;
let _24: [i128; 3];
let _25: [u64; 4];
let _26: char;
let _27: f64;
let _28: i8;
let _29: usize;
let _30: char;
let _31: *mut u64;
let _32: i32;
let _33: [u16; 1];
let _34: [i128; 3];
let _35: [u64; 6];
let _36: [u64; 4];
let _37: [i128; 3];
let _38: i16;
let _39: Adt47;
let _40: Adt60;
let _41: ();
let _42: ();
{
_6 = !_1;
_9 = -(-995721423_i32);
_11 = !23067_u16;
_6 = _1;
_11 = 59611_u16 - 12760_u16;
_8 = 2546987076415986319_i64 | (-2768241926765578693_i64);
_10 = !_4;
RET = 3395677372_u32 ^ 1228052720_u32;
_5 = 339776979249735993_usize as u64;
_9 = (-940619815_i32) * (-710046478_i32);
_1 = !_6;
_3 = _2 - _2;
_5 = !16063067223395421640_u64;
_12 = [3_usize,9526185958529315918_usize,4_usize,2_usize,15406718714746868107_usize,5_usize];
_3 = _2;
_8 = (-2169879404830697200_i64);
_2 = _3 & _3;
_6 = !_1;
_2 = -_3;
_4 = _10 << _11;
Goto(bb1)
}
bb1 = {
_4 = _10;
_6 = _1;
RET = !942877272_u32;
RET = !2325338684_u32;
RET = 1018549018_u32;
RET = 4042703634_u32 & 2843636411_u32;
_13 = 6_usize >> _3;
_7 = !_4;
_3 = _2 * _2;
_3 = (-60_i8) as isize;
_1 = _6;
RET = 1035613049_u32;
Call(_7 = core::intrinsics::bswap(_4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3 = _5 as isize;
_10 = _7 & _4;
_3 = !_2;
_3 = _2 + _2;
_9 = (-155560744_i32);
RET = 190_u8 as u32;
_13 = !3_usize;
_11 = !9581_u16;
_2 = _3;
_1 = !_6;
_1 = _6;
Call(_15 = fn2(_12, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_14.0 = [_5,_5,_5,_5];
RET = 255211276907175340949224392627765780587_u128 as u32;
_13 = !6579104746881625561_usize;
_1 = _6;
_14.1 = !RET;
_2 = _3;
_2 = _3 & _3;
_5 = 13163111355212184475_u64;
_5 = _10 as u64;
_14.2 = !91_u8;
_8 = 6237364270983632804_i64;
_16 = !_11;
_14.1 = RET;
_14.2 = 18_u8;
_7 = !_10;
_8 = (-2016227280052279016_i64);
_14.0 = [_5,_5,_5,_5];
_11 = _14.2 as u16;
_16 = _13 as u16;
_3 = _2;
_15 = !_9;
_19 = _2;
_5 = 49918934422812811053418497231709282821_u128 as u64;
_17 = _13;
Goto(bb4)
}
bb4 = {
_7 = _3 as i16;
_2 = -_19;
_14.1 = RET | RET;
_5 = !7170553561107242100_u64;
_19 = '\u{277ea}' as isize;
_14.1 = 48712013700370453858462210258718380281_u128 as u32;
_14.0 = [_5,_5,_5,_5];
RET = _14.1;
_11 = _16;
_7 = !_4;
_17 = !_13;
_13 = !_17;
_4 = _10;
RET = _14.1 & _14.1;
_8 = 306808367114220319235742683864141955574_u128 as i64;
_9 = !_15;
match _14.2 {
0 => bb5,
18 => bb7,
_ => bb6
}
}
bb5 = {
_14.0 = [_5,_5,_5,_5];
RET = 255211276907175340949224392627765780587_u128 as u32;
_13 = !6579104746881625561_usize;
_1 = _6;
_14.1 = !RET;
_2 = _3;
_2 = _3 & _3;
_5 = 13163111355212184475_u64;
_5 = _10 as u64;
_14.2 = !91_u8;
_8 = 6237364270983632804_i64;
_16 = !_11;
_14.1 = RET;
_14.2 = 18_u8;
_7 = !_10;
_8 = (-2016227280052279016_i64);
_14.0 = [_5,_5,_5,_5];
_11 = _14.2 as u16;
_16 = _13 as u16;
_3 = _2;
_15 = !_9;
_19 = _2;
_5 = 49918934422812811053418497231709282821_u128 as u64;
_17 = _13;
Goto(bb4)
}
bb6 = {
_4 = _10;
_6 = _1;
RET = !942877272_u32;
RET = !2325338684_u32;
RET = 1018549018_u32;
RET = 4042703634_u32 & 2843636411_u32;
_13 = 6_usize >> _3;
_7 = !_4;
_3 = _2 * _2;
_3 = (-60_i8) as isize;
_1 = _6;
RET = 1035613049_u32;
Call(_7 = core::intrinsics::bswap(_4), ReturnTo(bb2), UnwindUnreachable())
}
bb7 = {
_14.2 = !26_u8;
_19 = _2 & _3;
_14.1 = RET;
_5 = !4003946406782521996_u64;
_13 = !_17;
_8 = -(-7701194427542242761_i64);
_4 = _10 >> _7;
_24 = [(-105752188906448765170524713324985274384_i128),(-76958774176818983778335972022275602674_i128),(-90012314098417945364900254888484752107_i128)];
RET = _14.1 + _14.1;
_1 = _6 | _6;
_14.0 = [_5,_5,_5,_5];
_2 = _3 >> _15;
_6 = !_1;
_11 = _16 ^ _16;
_2 = 100098472793217260306832097676007921905_u128 as isize;
_25 = [_5,_5,_5,_5];
_22 = core::ptr::addr_of_mut!(RET);
_14.1 = _11 as u32;
_1 = !_6;
_2 = _3 << (*_22);
_11 = _16;
(*_22) = _17 as u32;
Goto(bb8)
}
bb8 = {
_1 = _6;
_29 = _17 << _10;
_13 = _14.2 as usize;
_26 = '\u{e649b}';
_10 = _4 - _4;
_24 = [93112233585677156055489895783380410850_i128,168314725013245465321643056451977019470_i128,58368291208172536324112536175675482448_i128];
_2 = 51934025881980371736905588161076366701_u128 as isize;
_2 = _19 >> _10;
(*_22) = _19 as u32;
_13 = _11 as usize;
Call(_19 = fn19(_10, _24, (*_22)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_14.0 = [_5,_5,_5,_5];
_10 = 160890754940038643393016074660040823934_u128 as i16;
_4 = -_7;
_14.2 = 200_u8;
_27 = RET as f64;
RET = _14.1;
_28 = 87_i8;
_27 = 105528881077419820482788224370675695158_i128 as f64;
_14 = (_25, (*_22), 246_u8);
_31 = core::ptr::addr_of_mut!(_5);
_10 = -_4;
_27 = _15 as f64;
_24 = [59127264961852175144110791618819391636_i128,(-74430974528167446949886901587569531463_i128),(-69154419076912861224863821810130094986_i128)];
_1 = !_6;
_32 = !_15;
_4 = _26 as i16;
_7 = _10 ^ _10;
_33 = [_11];
_13 = _16 as usize;
_13 = !_29;
_24 = [(-156490834275125681004805692393251685965_i128),(-62341469035951516245732105283712607881_i128),(-89866194082155042345280113160529924915_i128)];
_13 = _2 as usize;
_35 = [(*_31),_5,(*_31),(*_31),(*_31),(*_31)];
_31 = core::ptr::addr_of_mut!((*_31));
_5 = 6315742321944530274_u64;
match _14.2 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
246 => bb11,
_ => bb10
}
}
bb10 = {
_7 = _3 as i16;
_2 = -_19;
_14.1 = RET | RET;
_5 = !7170553561107242100_u64;
_19 = '\u{277ea}' as isize;
_14.1 = 48712013700370453858462210258718380281_u128 as u32;
_14.0 = [_5,_5,_5,_5];
RET = _14.1;
_11 = _16;
_7 = !_4;
_17 = !_13;
_13 = !_17;
_4 = _10;
RET = _14.1 & _14.1;
_8 = 306808367114220319235742683864141955574_u128 as i64;
_9 = !_15;
match _14.2 {
0 => bb5,
18 => bb7,
_ => bb6
}
}
bb11 = {
_23 = _1;
_24 = [145947542162500140538650391792687950894_i128,3734165958488257629836721873235590350_i128,23300557867352221593096560789275491386_i128];
_33 = [_16];
_22 = core::ptr::addr_of_mut!(_14.1);
_11 = _28 as u16;
_4 = _10 ^ _7;
_37 = [104713786252168020289302976229204043747_i128,128592884574524942280709176371818108582_i128,137491819682880242594715786002384726691_i128];
_24 = [(-36446010539969067350324198829317034014_i128),(-18565249202255376253216247201235059080_i128),(-30184942828506427591181869557557054772_i128)];
_36 = [(*_31),_5,(*_31),_5];
_35 = [(*_31),(*_31),(*_31),(*_31),(*_31),_5];
_11 = _16;
_8 = _28 as i64;
RET = _27 as u32;
_4 = _7 * _7;
_1 = _6;
_8 = (*_22) as i64;
_30 = _26;
_16 = _11 >> _2;
_14.1 = _8 as u32;
_30 = _26;
_9 = !_32;
match _5 {
0 => bb7,
1 => bb6,
2 => bb12,
6315742321944530274 => bb14,
_ => bb13
}
}
bb12 = {
_14.2 = !26_u8;
_19 = _2 & _3;
_14.1 = RET;
_5 = !4003946406782521996_u64;
_13 = !_17;
_8 = -(-7701194427542242761_i64);
_4 = _10 >> _7;
_24 = [(-105752188906448765170524713324985274384_i128),(-76958774176818983778335972022275602674_i128),(-90012314098417945364900254888484752107_i128)];
RET = _14.1 + _14.1;
_1 = _6 | _6;
_14.0 = [_5,_5,_5,_5];
_2 = _3 >> _15;
_6 = !_1;
_11 = _16 ^ _16;
_2 = 100098472793217260306832097676007921905_u128 as isize;
_25 = [_5,_5,_5,_5];
_22 = core::ptr::addr_of_mut!(RET);
_14.1 = _11 as u32;
_1 = !_6;
_2 = _3 << (*_22);
_11 = _16;
(*_22) = _17 as u32;
Goto(bb8)
}
bb13 = {
_3 = _5 as isize;
_10 = _7 & _4;
_3 = !_2;
_3 = _2 + _2;
_9 = (-155560744_i32);
RET = 190_u8 as u32;
_13 = !3_usize;
_11 = !9581_u16;
_2 = _3;
_1 = !_6;
_1 = _6;
Call(_15 = fn2(_12, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb14 = {
_33 = [_16];
_6 = !_1;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(1_usize, 1_usize, Move(_1), 8_usize, Move(_8), 33_usize, Move(_33), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(1_usize, 7_usize, Move(_7), 24_usize, Move(_24), 30_usize, Move(_30), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(1_usize, 25_usize, Move(_25), 32_usize, Move(_32), 23_usize, Move(_23), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(1_usize, 11_usize, Move(_11), 6_usize, Move(_6), 10_usize, Move(_10), 42_usize, _42), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: [usize; 6],mut _2: u64) -> i32 {
mir! {
type RET = i32;
let _3: Adt60;
let _4: u8;
let _5: i32;
let _6: u32;
let _7: [u16; 1];
let _8: i128;
let _9: f32;
let _10: u64;
let _11: u16;
let _12: (u32, [i128; 5], ([u8; 7], [u64; 4]));
let _13: [u64; 6];
let _14: i128;
let _15: usize;
let _16: ([u64; 4], u32, u8);
let _17: bool;
let _18: usize;
let _19: ([u8; 7], [u64; 4]);
let _20: ([u8; 7], [u64; 4]);
let _21: [i128; 3];
let _22: [i32; 7];
let _23: ();
let _24: ();
{
RET = (-1724296506_i32) & 380045738_i32;
RET = 1722049621_i32;
_2 = 10345178683112766384_u64;
_1 = [13650082278510356139_usize,6_usize,7693632771463667204_usize,2_usize,11465105896718403155_usize,3_usize];
_2 = !17384178455495052021_u64;
RET = (-239177049_i32);
RET = 1129130396_i32 & (-691176381_i32);
_4 = 173_u8;
_4 = 56_u8 - 176_u8;
RET = 1342178146_i32;
_4 = 11_u8;
_5 = 94_isize as i32;
_1 = [1655465987799383076_usize,7853027183794800005_usize,1_usize,0_usize,4315263928283393145_usize,9775298804131188454_usize];
Call(_5 = fn3(_1, _1, _2, _1, _1, _1, _1, _1, RET, _1, RET, _4, _4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = 14448972833867777439_u64;
_1 = [7_usize,8654479267503775927_usize,3_usize,438178262541666823_usize,4648511993056498768_usize,0_usize];
_2 = !12034075987852924707_u64;
_6 = !3307995264_u32;
_2 = !16950004348255950534_u64;
_5 = _4 as i32;
RET = _5 >> _2;
_5 = '\u{e6568}' as i32;
_6 = !2763800464_u32;
_2 = 1648052380611829846_u64 + 17218948706702590652_u64;
_5 = RET;
RET = _4 as i32;
RET = _5 << _5;
_5 = RET ^ RET;
_6 = 3_usize as u32;
_1 = [5577554509803341695_usize,7_usize,0_usize,4_usize,15259494355708151989_usize,4_usize];
_4 = 210_u8 >> _5;
Goto(bb2)
}
bb2 = {
RET = (-98_isize) as i32;
_7 = [41656_u16];
_7 = [61413_u16];
_6 = !3860963422_u32;
_4 = !89_u8;
_2 = !15612754315811083093_u64;
RET = !_5;
_6 = 358549237_u32;
_6 = 2592798805_u32;
_9 = 293528583833074316576814751185946106544_u128 as f32;
Goto(bb3)
}
bb3 = {
_12.0 = _6 << _6;
_13 = [_2,_2,_2,_2,_2,_2];
_13 = [_2,_2,_2,_2,_2,_2];
_13 = [_2,_2,_2,_2,_2,_2];
_14 = -20224890962242906553638370272432214859_i128;
_12.1 = [_14,_14,_14,_14,_14];
match _6 {
0 => bb1,
2592798805 => bb4,
_ => bb2
}
}
bb4 = {
_7 = [42756_u16];
_12.2.1 = [_2,_2,_2,_2];
_5 = !RET;
RET = -_5;
_13 = [_2,_2,_2,_2,_2,_2];
_15 = (-32328_i16) as usize;
_16.0 = [_2,_2,_2,_2];
_16.1 = _6;
_9 = _5 as f32;
_16.1 = _6 + _12.0;
_11 = !37666_u16;
_12.2.0 = [_4,_4,_4,_4,_4,_4,_4];
RET = !_5;
_16.1 = _4 as u32;
_15 = 1_usize;
_16.0[_15] = _12.2.1[_15] - _2;
_10 = _2 >> _4;
_16 = (_12.2.1, _12.0, _12.2.0[_15]);
_9 = 9223372036854775807_isize as f32;
_10 = _2 | _2;
Call(_15 = core::intrinsics::bswap(_1[_15]), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_8 = _5 as i128;
_12.2.0 = [_16.2,_16.2,_16.2,_4,_16.2,_4,_4];
_12.0 = (-86_i8) as u32;
_9 = 108629548718169489626558434218294408778_u128 as f32;
_1 = [_15,_15,_15,_15,_15,_15];
_13 = [_10,_10,_10,_10,_10,_10];
RET = _5 ^ _5;
RET = _5 - _5;
_6 = _16.1;
_12.2.1 = [_10,_10,_2,_10];
_16 = (_12.2.1, _6, _4);
_19.0 = [_16.2,_4,_4,_4,_4,_16.2,_4];
_17 = true;
_14 = _8 * _8;
_16.0 = _12.2.1;
_12.0 = _16.1 << RET;
_12.2.0 = _19.0;
_7 = [_11];
_15 = 5_usize + 7_usize;
_16.0 = _12.2.1;
Goto(bb6)
}
bb6 = {
Call(_23 = dump_var(2_usize, 10_usize, Move(_10), 17_usize, Move(_17), 11_usize, Move(_11), 13_usize, Move(_13)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_23 = dump_var(2_usize, 15_usize, Move(_15), 14_usize, Move(_14), 5_usize, Move(_5), 24_usize, _24), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: [usize; 6],mut _2: [usize; 6],mut _3: u64,mut _4: [usize; 6],mut _5: [usize; 6],mut _6: [usize; 6],mut _7: [usize; 6],mut _8: [usize; 6],mut _9: i32,mut _10: [usize; 6],mut _11: i32,mut _12: u8,mut _13: u8) -> i32 {
mir! {
type RET = i32;
let _14: [i128; 5];
let _15: char;
let _16: f64;
let _17: &'static u16;
let _18: u8;
let _19: [i32; 7];
let _20: *const i8;
let _21: i16;
let _22: f32;
let _23: char;
let _24: u128;
let _25: usize;
let _26: [i128; 5];
let _27: bool;
let _28: ();
let _29: ();
{
_13 = _12;
_6 = [3_usize,17120572250285538981_usize,8023437832321639970_usize,12467509286586090688_usize,6_usize,4_usize];
RET = _11;
_11 = _9 << _9;
_6 = _4;
_7 = [6_usize,4_usize,10351558231128564246_usize,5_usize,15560855710624752246_usize,2920995990483798586_usize];
RET = -_11;
_7 = _10;
_3 = 11718783093044015215_u64 << RET;
RET = 1255769220_u32 as i32;
Goto(bb1)
}
bb1 = {
_13 = _12 ^ _12;
_14 = [87680646724878756090913217488035483731_i128,152210358778902526813125820063941660523_i128,34254452593122304325047913776352785987_i128,259714821736330907156371776571238937_i128,(-21625920046857907779970967124152035823_i128)];
_5 = _10;
_11 = 66_isize as i32;
_7 = [4_usize,6_usize,15148991331659289964_usize,12814324529566002667_usize,1467701665010627027_usize,9446652325915576483_usize];
_1 = [770403590258526166_usize,5_usize,8857179672478425344_usize,18374743734905967539_usize,0_usize,4_usize];
_15 = '\u{3968a}';
RET = -_9;
RET = _9;
_8 = [0_usize,7392885518594128709_usize,15543951690900883322_usize,12254034754196889548_usize,7_usize,5491633395086514421_usize];
_2 = _8;
_8 = _10;
Call(_2 = fn4(_13, RET, _6, _5, _1, _10, _9, _10, _5, _12, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = _9;
_2 = [7_usize,18278191280859548669_usize,1_usize,3_usize,3_usize,1_usize];
RET = _9 + _11;
_2 = _10;
_1 = _7;
_3 = 8152360334417123058_u64;
_3 = !1220146899208324479_u64;
_15 = '\u{d8cdb}';
_16 = 11644_i16 as f64;
RET = _11 + _9;
_4 = [3_usize,10114312057199985097_usize,11325837363337422684_usize,1_usize,18350981445653854443_usize,12616754013720367989_usize];
_12 = _13 >> RET;
_8 = [5_usize,8618137238494604912_usize,8684753030462893625_usize,11633232243891357170_usize,17961614689697226877_usize,10804068538040922218_usize];
_7 = [6025961758393353430_usize,7_usize,14868861858680447487_usize,0_usize,9517637488087041635_usize,9947089408121919687_usize];
_5 = [16991773019927609166_usize,4691397406551318476_usize,2_usize,3298689782314845135_usize,14634796371655754446_usize,9648960379705972204_usize];
_11 = !_9;
_7 = [12228238237162866219_usize,6_usize,0_usize,7_usize,1400029321112423441_usize,8972217491080461013_usize];
_5 = [3_usize,16767775379516015126_usize,1_usize,17766339294204572636_usize,2367390560778437672_usize,10266269941819713293_usize];
_13 = !_12;
_14 = [(-58951629514374616998093483299542338272_i128),58893266347494907224945559441744541526_i128,(-157256151169311420771874118539437729407_i128),(-57122156960453488352523833943712497287_i128),(-152778566013268132947490678035570443200_i128)];
RET = _11;
RET = _13 as i32;
_18 = _12 | _13;
_15 = '\u{37c3e}';
_1 = [2_usize,401628249114051961_usize,11856411289228281993_usize,6_usize,4063846888284515768_usize,1_usize];
_4 = [11920401820683492105_usize,6_usize,17301739478923984380_usize,18200554389435775180_usize,3_usize,2_usize];
_7 = _6;
_1 = [1509714223982179933_usize,6_usize,6_usize,16711074875855557846_usize,6_usize,5491232880210313278_usize];
Goto(bb3)
}
bb3 = {
_8 = _6;
_1 = [1_usize,11656760967679716153_usize,15418507137495767352_usize,7_usize,7_usize,17105554582380499341_usize];
_9 = 42709_u16 as i32;
_9 = RET;
_2 = [10465295812770493672_usize,0_usize,18116737385299175524_usize,1_usize,14529007526249004105_usize,3987608419117610960_usize];
_7 = _8;
_7 = [3_usize,3_usize,13800573653326664535_usize,0_usize,17922609126243047267_usize,15889499387602879879_usize];
RET = _9 ^ _11;
_8 = [12764309259854363306_usize,2421882286946247522_usize,7_usize,0_usize,3_usize,1604488480987548072_usize];
_16 = 61944_u16 as f64;
_9 = !RET;
_2 = [5_usize,13815816957447135104_usize,4634886196680762324_usize,6_usize,14720549628204908255_usize,7_usize];
_1 = [2_usize,0_usize,3_usize,0_usize,13889045012541236434_usize,5319448184359994142_usize];
_10 = [6_usize,11865509862499351850_usize,6922928972846950183_usize,635762978773245553_usize,6_usize,8519094863701601177_usize];
_7 = [3_usize,7_usize,7_usize,7852910470763274222_usize,4_usize,11997790991138413255_usize];
_5 = [7198732354254895791_usize,9877632796627887479_usize,1975970363730854678_usize,13757349273089517337_usize,4_usize,7886460109441123493_usize];
_15 = '\u{382ef}';
_19 = [_9,RET,_11,RET,_9,RET,RET];
_6 = [1_usize,3439458116008711502_usize,10575616900583778804_usize,7_usize,3_usize,17120446708965907655_usize];
RET = _9;
_12 = !_18;
_19 = [_9,_9,_9,_9,_9,_9,_9];
_15 = '\u{8a1f}';
_21 = 860_i16 & 9115_i16;
_7 = _5;
Call(_11 = fn18(_6, _10, _14, RET, _14, _4, _6, RET, _14, _9, _4, _13, _8, _7), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_1 = [0_usize,14559011891171645922_usize,4_usize,2_usize,5_usize,1528337299052384327_usize];
_13 = !_18;
_10 = [2_usize,3483861943569792923_usize,14034275304451092813_usize,5_usize,0_usize,1_usize];
_16 = 9159668266606031912_i64 as f64;
_18 = _13;
_18 = !_12;
_19 = [RET,_9,_9,RET,_9,_9,_11];
_2 = [5_usize,12048287561405245764_usize,2_usize,6390948174096767606_usize,14887684755016320074_usize,4_usize];
_1 = [15184917179037189030_usize,2_usize,1_usize,0_usize,6_usize,7294181322126097792_usize];
RET = true as i32;
_14 = [11108538186237095833446829453940009708_i128,95325379476852401751706467710914327645_i128,(-113964688552036899466927124560356075104_i128),105388101312146189734067714407479639199_i128,(-59036024046465790882873841122671221612_i128)];
_6 = [6678667007801493792_usize,5_usize,1_usize,17648003312527123940_usize,1_usize,6_usize];
_22 = 5473806695386835150655324420312704362_u128 as f32;
Goto(bb5)
}
bb5 = {
_12 = _18;
_16 = _22 as f64;
_24 = !280350384211736583385875125455678337355_u128;
_3 = !356221586145845418_u64;
_16 = _12 as f64;
_8 = [0_usize,2267290836639177936_usize,5_usize,7_usize,11536039345139352158_usize,5_usize];
RET = -_11;
_14 = [(-146245043361061476024150484862112247555_i128),166607043457294127272864229770348635311_i128,(-33802274658584884105703867977624254384_i128),5209653640057314645256410271502082715_i128,(-15508885890145773338358692008954606037_i128)];
_5 = [11054891279733093962_usize,4_usize,6801066483321273817_usize,1_usize,1483924702558785782_usize,2_usize];
_2 = _8;
_3 = _15 as u64;
_25 = (-128_i8) as usize;
_6 = [_25,_25,_25,_25,_25,_25];
_14 = [16438086722918702360612917104000547167_i128,(-116456247403454057540209216272908931331_i128),105328737329553026390162891974867319475_i128,122263631898402887628740189636452701194_i128,(-37458840063253231964932675093879881867_i128)];
_10 = [_25,_25,_25,_25,_25,_25];
_12 = _13 | _13;
_21 = 31150_i16;
Goto(bb6)
}
bb6 = {
Call(_28 = dump_var(3_usize, 1_usize, Move(_1), 14_usize, Move(_14), 25_usize, Move(_25), 9_usize, Move(_9)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_28 = dump_var(3_usize, 10_usize, Move(_10), 5_usize, Move(_5), 2_usize, Move(_2), 3_usize, Move(_3)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_28 = dump_var(3_usize, 24_usize, Move(_24), 15_usize, Move(_15), 29_usize, _29, 29_usize, _29), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: u8,mut _2: i32,mut _3: [usize; 6],mut _4: [usize; 6],mut _5: [usize; 6],mut _6: [usize; 6],mut _7: i32,mut _8: [usize; 6],mut _9: [usize; 6],mut _10: u8,mut _11: u64) -> [usize; 6] {
mir! {
type RET = [usize; 6];
let _12: bool;
let _13: Adt49;
let _14: [i32; 7];
let _15: bool;
let _16: [i128; 3];
let _17: char;
let _18: [i64; 6];
let _19: [i32; 7];
let _20: Adt57;
let _21: *mut u32;
let _22: u32;
let _23: Adt57;
let _24: (u32, &'static u16, &'static u16);
let _25: Adt49;
let _26: ((i128, *mut i128, *mut i128, f32, i64), *mut i128, i128);
let _27: i8;
let _28: bool;
let _29: bool;
let _30: ([u8; 7], [u64; 4]);
let _31: Adt47;
let _32: bool;
let _33: f64;
let _34: u128;
let _35: *const (&'static u16, f64);
let _36: Adt49;
let _37: &'static u16;
let _38: [i32; 7];
let _39: i32;
let _40: char;
let _41: ([u64; 4], u32, u8);
let _42: f64;
let _43: isize;
let _44: i64;
let _45: f64;
let _46: i16;
let _47: f64;
let _48: [i128; 3];
let _49: f64;
let _50: ();
let _51: ();
{
_4 = [15132867502925754578_usize,11822576977157372278_usize,1_usize,7988328211347028399_usize,3779667372346290582_usize,9395598071193832371_usize];
_11 = 167467141837396502683898826538063992511_u128 as u64;
_6 = [5_usize,7_usize,3_usize,17519757953075581198_usize,3_usize,3969599714847920843_usize];
_6 = [0_usize,3_usize,1525193039227703399_usize,6_usize,17301436799829610336_usize,11547659874151998092_usize];
_2 = _7;
RET = _5;
_12 = !false;
_7 = _2;
_4 = [17107596933171937322_usize,11010119807418614197_usize,6223892346750291522_usize,6_usize,12991069438635106543_usize,5_usize];
_2 = (-22_isize) as i32;
_1 = (-21556_i16) as u8;
_6 = _3;
_1 = 3_usize as u8;
_10 = _1 + _1;
_11 = 3419727151678059949_u64 | 496968517025140702_u64;
_10 = _7 as u8;
_9 = _6;
_12 = _2 >= _7;
RET = [7_usize,12757968189407743710_usize,14648909533727750305_usize,9767040998474275932_usize,14318850669157599451_usize,17723792385434772179_usize];
RET = _9;
_11 = (-63_i8) as u64;
_8 = [0_usize,0_usize,2612647055841221234_usize,903757576938454986_usize,4_usize,4523327414257962145_usize];
_13 = Adt49::Variant3 { fld0: 103_i16,fld1: 151179678499534521183721268484738009419_u128 };
match _7 {
1342178146 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
RET = _3;
_7 = _2;
_11 = 14564461882762282988_u64 + 8183359720299703546_u64;
RET = [2_usize,2545535903152239322_usize,3_usize,5_usize,6_usize,2_usize];
_10 = 1_usize as u8;
_2 = !_7;
_3 = [13173521102610195778_usize,5_usize,1831272937364954003_usize,3_usize,5_usize,4048820481932836482_usize];
place!(Field::<i16>(Variant(_13, 3), 0)) = -(-4479_i16);
Call(_1 = fn5(_3, _9, RET, _5, _9, _9, _4, _9, _4, _3, _5, RET, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_3 = [0_usize,7570687807055957210_usize,2_usize,1_usize,2634404701178103817_usize,10042774224489793698_usize];
_9 = [6_usize,1_usize,17796213049220679257_usize,2_usize,0_usize,4_usize];
place!(Field::<u128>(Variant(_13, 3), 1)) = 480457788115412649_i64 as u128;
_7 = _2;
_6 = [4_usize,3_usize,11025430170476613074_usize,7_usize,1_usize,7_usize];
_11 = 15325595634979691929_u64 ^ 2353691351142133556_u64;
_5 = [12760671829904293774_usize,12445419714751229887_usize,4_usize,2_usize,3_usize,2_usize];
place!(Field::<i16>(Variant(_13, 3), 0)) = _1 as i16;
place!(Field::<i16>(Variant(_13, 3), 0)) = 298_i16 << _1;
_16 = [106976601372287957119791229175683696896_i128,(-39679685480373773148699271630804706942_i128),(-31448034622992927580211276837318935449_i128)];
_16 = [123796182609813967686860955889545748266_i128,40590145657756173165431598690446334220_i128,(-5821896670618797637307269696017897510_i128)];
_11 = !6414956829662020009_u64;
_7 = 138631732625815422295842966483033132059_i128 as i32;
_3 = [0_usize,5_usize,3_usize,1247144625593620095_usize,0_usize,14781737976587560521_usize];
_3 = [2_usize,6361247800390670651_usize,4_usize,1531310050066303337_usize,4_usize,1727173330363266047_usize];
_9 = _4;
_2 = _7 | _7;
_6 = [7358804663309860542_usize,9603600425229396645_usize,2708384151778836023_usize,3_usize,17864095899503269904_usize,6_usize];
_10 = 33310_u16 as u8;
Goto(bb4)
}
bb4 = {
_10 = _1 | _1;
_16 = [(-144864255317533363853485700256556014339_i128),(-106137659983880116285720205898204948166_i128),(-13416011617199148438426639457290193724_i128)];
_18 = [(-915855252051619534_i64),8150080836785547830_i64,(-7942776673502161477_i64),904051821379846947_i64,1714401291704773073_i64,7476354805340754593_i64];
_14 = [_2,_2,_2,_2,_2,_2,_2];
RET = [5702475125352948380_usize,13506547299862716473_usize,4414110202738715343_usize,14853969680454540496_usize,1_usize,6232396151282232969_usize];
_11 = 7716183504772901618_u64;
_21 = core::ptr::addr_of_mut!(_22);
_24.0 = 1803839872_u32 - 160023635_u32;
_1 = _10;
(*_21) = 14_i8 as u32;
_17 = '\u{d4964}';
SetDiscriminant(_13, 1);
place!(Field::<[u64; 4]>(Variant(_13, 1), 4)) = [_11,_11,_11,_11];
_15 = _22 == _22;
_26.1 = core::ptr::addr_of_mut!(_26.0.0);
_6 = [5_usize,2_usize,0_usize,12396450506067450793_usize,6_usize,6_usize];
place!(Field::<Adt46>(Variant(_13, 1), 1)).fld0 = _10;
_4 = [6901084336180325473_usize,11172427602969806586_usize,2_usize,1_usize,6_usize,11798083622785826862_usize];
_24.0 = 2688303063007650774_usize as u32;
_5 = [12382993943571971489_usize,2_usize,14698441923537194336_usize,3_usize,3_usize,14575061271472834653_usize];
_2 = _7;
place!(Field::<i32>(Variant(_13, 1), 5)) = !_7;
Call(_26.0 = fn16(_1, _1, _18, _14, RET, Move(Field::<Adt46>(Variant(_13, 1), 1)), _10, _6, RET, _7, _5, _12, _1, _18, _11), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_25 = Adt49::Variant3 { fld0: 28166_i16,fld1: 77921887016602908139688289582882223593_u128 };
place!(Field::<usize>(Variant(_13, 1), 6)) = !13901656429354017098_usize;
_28 = _1 >= _10;
_25 = Adt49::Variant2 { fld0: _26.0.3,fld1: 334114278458236334014410085318696073226_u128 };
_15 = _28 | _28;
(*_21) = _24.0;
_25 = Adt49::Variant3 { fld0: (-7599_i16),fld1: 110310732822324143562694242281974988343_u128 };
place!(Field::<i32>(Variant(_13, 1), 5)) = _7 + _2;
_29 = _15;
_5 = _9;
_26.0.3 = (-6807_i16) as f32;
_21 = core::ptr::addr_of_mut!((*_21));
_29 = !_15;
_2 = Field::<i32>(Variant(_13, 1), 5) - Field::<i32>(Variant(_13, 1), 5);
_14 = [_2,_2,_2,Field::<i32>(Variant(_13, 1), 5),_7,_2,_2];
_32 = !_29;
_34 = _28 as u128;
match _11 {
0 => bb4,
7716183504772901618 => bb7,
_ => bb6
}
}
bb6 = {
_3 = [0_usize,7570687807055957210_usize,2_usize,1_usize,2634404701178103817_usize,10042774224489793698_usize];
_9 = [6_usize,1_usize,17796213049220679257_usize,2_usize,0_usize,4_usize];
place!(Field::<u128>(Variant(_13, 3), 1)) = 480457788115412649_i64 as u128;
_7 = _2;
_6 = [4_usize,3_usize,11025430170476613074_usize,7_usize,1_usize,7_usize];
_11 = 15325595634979691929_u64 ^ 2353691351142133556_u64;
_5 = [12760671829904293774_usize,12445419714751229887_usize,4_usize,2_usize,3_usize,2_usize];
place!(Field::<i16>(Variant(_13, 3), 0)) = _1 as i16;
place!(Field::<i16>(Variant(_13, 3), 0)) = 298_i16 << _1;
_16 = [106976601372287957119791229175683696896_i128,(-39679685480373773148699271630804706942_i128),(-31448034622992927580211276837318935449_i128)];
_16 = [123796182609813967686860955889545748266_i128,40590145657756173165431598690446334220_i128,(-5821896670618797637307269696017897510_i128)];
_11 = !6414956829662020009_u64;
_7 = 138631732625815422295842966483033132059_i128 as i32;
_3 = [0_usize,5_usize,3_usize,1247144625593620095_usize,0_usize,14781737976587560521_usize];
_3 = [2_usize,6361247800390670651_usize,4_usize,1531310050066303337_usize,4_usize,1727173330363266047_usize];
_9 = _4;
_2 = _7 | _7;
_6 = [7358804663309860542_usize,9603600425229396645_usize,2708384151778836023_usize,3_usize,17864095899503269904_usize,6_usize];
_10 = 33310_u16 as u8;
Goto(bb4)
}
bb7 = {
_11 = _26.0.4 as u64;
_26.0.1 = core::ptr::addr_of_mut!(_26.0.0);
_12 = !_15;
_27 = 37_i8 - (-107_i8);
_6 = [Field::<usize>(Variant(_13, 1), 6),Field::<usize>(Variant(_13, 1), 6),Field::<usize>(Variant(_13, 1), 6),Field::<usize>(Variant(_13, 1), 6),Field::<usize>(Variant(_13, 1), 6),Field::<usize>(Variant(_13, 1), 6)];
_22 = 9223372036854775807_isize as u32;
_19 = [_2,Field::<i32>(Variant(_13, 1), 5),_2,_2,Field::<i32>(Variant(_13, 1), 5),_2,Field::<i32>(Variant(_13, 1), 5)];
_22 = _24.0 + _24.0;
_15 = !_12;
_34 = !186850345171773934261442756801140893873_u128;
Goto(bb8)
}
bb8 = {
_25 = Adt49::Variant2 { fld0: _26.0.3,fld1: _34 };
RET = [Field::<usize>(Variant(_13, 1), 6),Field::<usize>(Variant(_13, 1), 6),Field::<usize>(Variant(_13, 1), 6),Field::<usize>(Variant(_13, 1), 6),Field::<usize>(Variant(_13, 1), 6),Field::<usize>(Variant(_13, 1), 6)];
_22 = Field::<usize>(Variant(_13, 1), 6) as u32;
_32 = !_15;
place!(Field::<usize>(Variant(_13, 1), 6)) = 2934945739384368486_usize << _1;
_26.0 = (79668251546943022741992350476042821318_i128, _26.1, _26.1, Field::<f32>(Variant(_25, 2), 0), 4405254603014700272_i64);
place!(Field::<Adt46>(Variant(_13, 1), 1)).fld0 = _10 * _1;
(*_21) = _24.0 ^ _24.0;
place!(Field::<[u64; 4]>(Variant(_13, 1), 4)) = [_11,_11,_11,_11];
_31 = Adt47::Variant0 { fld0: _26.0.3,fld1: _17,fld2: (-9223372036854775808_isize),fld3: _21,fld4: (-16283_i16),fld5: 31812_u16 };
_36 = Adt49::Variant2 { fld0: _26.0.3,fld1: Field::<u128>(Variant(_25, 2), 1) };
_26.0.1 = _26.0.2;
_29 = !_12;
_21 = Field::<*mut u32>(Variant(_31, 0), 3);
_41.2 = Field::<char>(Variant(_31, 0), 1) as u8;
place!(Field::<u128>(Variant(_25, 2), 1)) = _27 as u128;
_9 = _4;
place!(Field::<u128>(Variant(_36, 2), 1)) = !_34;
place!(Field::<u128>(Variant(_25, 2), 1)) = _34 + _34;
_41.0 = [_11,_11,_11,_11];
match _26.0.4 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb4,
4405254603014700272 => bb9,
_ => bb5
}
}
bb9 = {
(*_21) = _24.0 * _24.0;
_42 = _26.0.4 as f64;
_39 = Field::<i32>(Variant(_13, 1), 5) << _22;
_8 = [Field::<usize>(Variant(_13, 1), 6),Field::<usize>(Variant(_13, 1), 6),Field::<usize>(Variant(_13, 1), 6),Field::<usize>(Variant(_13, 1), 6),Field::<usize>(Variant(_13, 1), 6),Field::<usize>(Variant(_13, 1), 6)];
_14 = [_39,_39,Field::<i32>(Variant(_13, 1), 5),_7,_39,_2,_7];
place!(Field::<u16>(Variant(_31, 0), 5)) = Field::<u128>(Variant(_36, 2), 1) as u16;
_13 = Move(_25);
_30.0 = [_1,_10,_1,_10,_10,_1,_1];
_33 = _42 + _42;
_19 = _14;
_26.0 = (139438463287533042262363143196113469463_i128, _26.1, _26.1, Field::<f32>(Variant(_31, 0), 0), (-2452315038052993939_i64));
place!(Field::<f32>(Variant(_13, 2), 0)) = Field::<f32>(Variant(_31, 0), 0);
_26.0.1 = _26.1;
_25 = Move(_13);
place!(Field::<i16>(Variant(_31, 0), 4)) = -28952_i16;
_41.2 = _10;
place!(Field::<f32>(Variant(_31, 0), 0)) = _26.0.3 - _26.0.3;
place!(Field::<f32>(Variant(_25, 2), 0)) = _26.0.3 + Field::<f32>(Variant(_31, 0), 0);
_37 = &place!(Field::<u16>(Variant(_31, 0), 5));
_40 = Field::<char>(Variant(_31, 0), 1);
_41.2 = _22 as u8;
place!(Field::<u16>(Variant(_31, 0), 5)) = !4997_u16;
_16 = [_26.0.0,_26.0.0,_26.0.0];
_30.1 = _41.0;
_13 = Adt49::Variant3 { fld0: Field::<i16>(Variant(_31, 0), 4),fld1: Field::<u128>(Variant(_25, 2), 1) };
_31 = Adt47::Variant1 { fld0: _26.1 };
_44 = -_26.0.4;
_38 = [_7,_39,_2,_2,_39,_2,_2];
match _26.0.0 {
0 => bb6,
139438463287533042262363143196113469463 => bb11,
_ => bb10
}
}
bb10 = {
_3 = [0_usize,7570687807055957210_usize,2_usize,1_usize,2634404701178103817_usize,10042774224489793698_usize];
_9 = [6_usize,1_usize,17796213049220679257_usize,2_usize,0_usize,4_usize];
place!(Field::<u128>(Variant(_13, 3), 1)) = 480457788115412649_i64 as u128;
_7 = _2;
_6 = [4_usize,3_usize,11025430170476613074_usize,7_usize,1_usize,7_usize];
_11 = 15325595634979691929_u64 ^ 2353691351142133556_u64;
_5 = [12760671829904293774_usize,12445419714751229887_usize,4_usize,2_usize,3_usize,2_usize];
place!(Field::<i16>(Variant(_13, 3), 0)) = _1 as i16;
place!(Field::<i16>(Variant(_13, 3), 0)) = 298_i16 << _1;
_16 = [106976601372287957119791229175683696896_i128,(-39679685480373773148699271630804706942_i128),(-31448034622992927580211276837318935449_i128)];
_16 = [123796182609813967686860955889545748266_i128,40590145657756173165431598690446334220_i128,(-5821896670618797637307269696017897510_i128)];
_11 = !6414956829662020009_u64;
_7 = 138631732625815422295842966483033132059_i128 as i32;
_3 = [0_usize,5_usize,3_usize,1247144625593620095_usize,0_usize,14781737976587560521_usize];
_3 = [2_usize,6361247800390670651_usize,4_usize,1531310050066303337_usize,4_usize,1727173330363266047_usize];
_9 = _4;
_2 = _7 | _7;
_6 = [7358804663309860542_usize,9603600425229396645_usize,2708384151778836023_usize,3_usize,17864095899503269904_usize,6_usize];
_10 = 33310_u16 as u8;
Goto(bb4)
}
bb11 = {
place!(Field::<*mut i128>(Variant(_31, 1), 0)) = core::ptr::addr_of_mut!(_26.0.0);
_24.0 = (*_21);
place!(Field::<f32>(Variant(_25, 2), 0)) = _26.0.3 + _26.0.3;
_26.2 = 6_usize as i128;
_26.0.4 = _44 & _44;
(*_21) = !_24.0;
_3 = [0_usize,0_usize,4_usize,15915327980981616423_usize,0_usize,14621620896647741186_usize];
place!(Field::<f32>(Variant(_25, 2), 0)) = 59773_u16 as f32;
_29 = _12;
_11 = 14505588530235085400_u64 * 18058896306852509851_u64;
_12 = _32 <= _15;
_1 = _10 * _10;
_4 = [7431098392975506458_usize,1_usize,10350265935775598685_usize,2_usize,6_usize,0_usize];
_3 = _4;
_46 = Field::<i16>(Variant(_13, 3), 0);
_11 = 5770026690010342635_u64;
_26.0.0 = _32 as i128;
_30.0 = [_41.2,_1,_1,_1,_10,_10,_10];
_41.1 = _22;
_11 = 5567722175732317269_u64 & 5737052354247788121_u64;
Call(_34 = fn17(_26.0.2, _44, _26.0, _12, _26.1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_26.0.1 = core::ptr::addr_of_mut!(_26.2);
_4 = [16714325491519376005_usize,3255860516874072577_usize,6832748479029479885_usize,6_usize,344728959530231981_usize,17309177675135314860_usize];
_1 = _10 ^ _10;
place!(Field::<u128>(Variant(_36, 2), 1)) = _34;
_43 = !(-9223372036854775808_isize);
place!(Field::<u128>(Variant(_25, 2), 1)) = _32 as u128;
_6 = _8;
_43 = 3_usize as isize;
_15 = _32 & _29;
_41.2 = _1 + _1;
_14 = _19;
_25 = Move(_36);
_39 = _2 + _2;
_41.2 = 14069385647291958052_usize as u8;
place!(Field::<*mut i128>(Variant(_31, 1), 0)) = _26.1;
Goto(bb13)
}
bb13 = {
place!(Field::<f32>(Variant(_25, 2), 0)) = _26.0.3;
_34 = Field::<u128>(Variant(_25, 2), 1) << Field::<u128>(Variant(_25, 2), 1);
_2 = _39;
_24.0 = !(*_21);
_19 = [_39,_39,_2,_2,_2,_2,_2];
_24.0 = (*_21);
SetDiscriminant(_31, 1);
_42 = -_33;
_10 = 51732_u16 as u8;
_26.2 = _26.0.0 * _26.0.0;
_41 = (_30.1, _22, _1);
_33 = -_42;
_41.1 = !(*_21);
_28 = _12 & _15;
_23 = Adt57::Variant3 { fld0: _44,fld1: 8338037968329262105_usize };
place!(Field::<f32>(Variant(_25, 2), 0)) = _2 as f32;
_12 = !_32;
Goto(bb14)
}
bb14 = {
_43 = 12_isize;
_30.1 = [_11,_11,_11,_11];
_31 = Adt47::Variant0 { fld0: Field::<f32>(Variant(_25, 2), 0),fld1: _17,fld2: _43,fld3: _21,fld4: _46,fld5: 47210_u16 };
_29 = !_15;
_23 = Adt57::Variant3 { fld0: _26.0.4,fld1: 6_usize };
_13 = Adt49::Variant3 { fld0: _46,fld1: _34 };
(*_21) = _24.0;
SetDiscriminant(_25, 2);
place!(Field::<f32>(Variant(_25, 2), 0)) = Field::<f32>(Variant(_31, 0), 0) - Field::<f32>(Variant(_31, 0), 0);
_47 = _42;
_3 = [0_usize,0_usize,0_usize,11991695365646053228_usize,8685774336528080410_usize,2_usize];
_13 = Adt49::Variant2 { fld0: Field::<f32>(Variant(_25, 2), 0),fld1: _34 };
_26.0.0 = -_26.2;
SetDiscriminant(_13, 0);
_41.1 = _24.0 & _24.0;
_34 = _47 as u128;
_41.2 = _1;
_24.1 = &place!(Field::<u16>(Variant(_31, 0), 5));
_26.0.0 = _11 as i128;
Goto(bb15)
}
bb15 = {
Call(_50 = dump_var(4_usize, 39_usize, Move(_39), 12_usize, Move(_12), 43_usize, Move(_43), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_50 = dump_var(4_usize, 32_usize, Move(_32), 41_usize, Move(_41), 5_usize, Move(_5), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_50 = dump_var(4_usize, 46_usize, Move(_46), 11_usize, Move(_11), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_50 = dump_var(4_usize, 27_usize, Move(_27), 38_usize, Move(_38), 29_usize, Move(_29), 7_usize, Move(_7)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [usize; 6],mut _2: [usize; 6],mut _3: [usize; 6],mut _4: [usize; 6],mut _5: [usize; 6],mut _6: [usize; 6],mut _7: [usize; 6],mut _8: [usize; 6],mut _9: [usize; 6],mut _10: [usize; 6],mut _11: [usize; 6],mut _12: [usize; 6],mut _13: [usize; 6]) -> u8 {
mir! {
type RET = u8;
let _14: u8;
let _15: ((i128, *mut i128, *mut i128, f32, i64), *mut i128, i128);
let _16: char;
let _17: *mut u32;
let _18: i64;
let _19: [u64; 4];
let _20: [i64; 6];
let _21: (u32, [i128; 5], ([u8; 7], [u64; 4]));
let _22: u32;
let _23: [i128; 3];
let _24: bool;
let _25: isize;
let _26: &'static u16;
let _27: bool;
let _28: (&'static u16, f64);
let _29: (u32, i16);
let _30: isize;
let _31: isize;
let _32: Adt50;
let _33: Adt46;
let _34: Adt46;
let _35: (u32, i16);
let _36: Adt60;
let _37: isize;
let _38: ((i128, *mut i128, *mut i128, f32, i64), *mut i128, i128);
let _39: isize;
let _40: ([u8; 7], [u64; 4]);
let _41: f64;
let _42: ();
let _43: ();
{
RET = !209_u8;
_7 = [1_usize,158811597863190618_usize,3379429378243693595_usize,3_usize,6_usize,7_usize];
_10 = [3_usize,3_usize,5_usize,0_usize,5_usize,6_usize];
_14 = RET;
RET = _14 ^ _14;
_10 = _8;
Call(_6 = fn6(_4, _9, _1, _10, _3, _4, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = false as u8;
RET = !_14;
_15.0.4 = -8554111353149034720_i64;
_11 = [4430984132903272871_usize,5_usize,4_usize,0_usize,13498429764190874568_usize,6321270909795211207_usize];
_14 = RET * RET;
_12 = _11;
_13 = [6280827148507368152_usize,1_usize,2_usize,3_usize,5_usize,0_usize];
_15.0.0 = (-99948465509936487176176512263814110682_i128) - (-1439935961548420698062820948768111360_i128);
_9 = [6_usize,6_usize,3_usize,4975094944699298630_usize,0_usize,5_usize];
_1 = [10051039448322881415_usize,1_usize,5_usize,2922890706653809491_usize,3_usize,18386385727664761879_usize];
_9 = [3054108059187269787_usize,16042437448155123382_usize,4_usize,7_usize,5_usize,8876374461579565704_usize];
RET = _14;
_1 = _8;
_12 = [4_usize,7_usize,1_usize,6954260896788951565_usize,0_usize,5_usize];
_15.0.0 = _14 as i128;
_14 = RET;
_15.0.3 = 6883_u16 as f32;
_3 = _2;
_15.0.3 = 19949_u16 as f32;
_8 = [2577486586229621814_usize,1_usize,9585918218028927181_usize,3_usize,3_usize,1158675254326570026_usize];
RET = 0_usize as u8;
_15.0.3 = 10738178007575542922_u64 as f32;
_15.0.4 = !(-1100286881423399231_i64);
_12 = [3_usize,7368203965034866962_usize,2_usize,15808421313655528700_usize,7330135188914963475_usize,5938659118338194126_usize];
_15.0.1 = core::ptr::addr_of_mut!(_15.0.0);
Goto(bb2)
}
bb2 = {
_15.1 = _15.0.1;
_16 = '\u{8ba3}';
_13 = [1_usize,1_usize,6859845670336593273_usize,0_usize,12404032786830612237_usize,3_usize];
_7 = _2;
_12 = [3756343637467263289_usize,6_usize,4697222083917787935_usize,4498847957795038439_usize,2_usize,15644621441138197301_usize];
_15.0.3 = 5451_u16 as f32;
_2 = [7631526099980404181_usize,17570944901925858636_usize,6733238744610020288_usize,1_usize,18214410945856023146_usize,15437710374871940714_usize];
_15.0.4 = -5145466529075771602_i64;
RET = 1834897324_u32 as u8;
_15.0.0 = 1733259236_i32 as i128;
_5 = [3553586105775199189_usize,0_usize,3_usize,3_usize,3895755979370464333_usize,1551221529550534061_usize];
_6 = [11486353858879046394_usize,6_usize,3_usize,16402009846794226551_usize,7269729776795403487_usize,11392158003500825894_usize];
RET = _14;
_5 = [2_usize,18386595769791141171_usize,1241930509711402010_usize,9224322847837162528_usize,13655169311062043254_usize,10117641026963101105_usize];
Call(_15.2 = fn7(_10, _15.0.1, _8, _3, _13, _13), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_5 = [2_usize,8690448776350333298_usize,4590403905074927449_usize,6_usize,4_usize,1_usize];
_16 = '\u{e4f94}';
_18 = _15.0.4;
_6 = [17636327408659266656_usize,2712688274647621396_usize,5339749328107947864_usize,13507999148834442557_usize,4975640153773309224_usize,14872463872662181177_usize];
Goto(bb4)
}
bb4 = {
_11 = [17813451920063224008_usize,6561773307804776148_usize,8967970253079894869_usize,5_usize,2_usize,2453446279146215989_usize];
RET = _14 ^ _14;
_21.1 = [_15.0.0,_15.0.0,_15.0.0,_15.2,_15.2];
_5 = [2830840640650839762_usize,6_usize,1_usize,5_usize,15240484039232660561_usize,9290863267366001016_usize];
_13 = [1_usize,1_usize,12030633511866144133_usize,12781727803076704456_usize,2840886434541466302_usize,0_usize];
Goto(bb5)
}
bb5 = {
_1 = [2_usize,10653607806901960299_usize,10654012752839750779_usize,5_usize,3_usize,0_usize];
_10 = _7;
_20 = [_18,_18,_15.0.4,_15.0.4,_15.0.4,_18];
_21.2.0 = [_14,_14,RET,_14,_14,RET,RET];
_20 = [_15.0.4,_15.0.4,_15.0.4,_18,_18,_18];
_8 = [957980369826927545_usize,0_usize,7670598917962050356_usize,8278655789497526458_usize,10616441477032236319_usize,5_usize];
_20 = [_15.0.4,_15.0.4,_15.0.4,_15.0.4,_15.0.4,_18];
_21.2.1 = [11934354071769280157_u64,13115688211573233460_u64,8616579314325630437_u64,16723585152438545922_u64];
_15.0.2 = core::ptr::addr_of_mut!(_15.2);
_17 = core::ptr::addr_of_mut!(_21.0);
_15.0.1 = _15.1;
RET = _14 + _14;
_21.0 = 1396760193_u32 - 2884967101_u32;
_5 = _12;
_5 = [15888276672822444107_usize,14171694945226431412_usize,16979052207150787266_usize,3600282257621057721_usize,0_usize,2_usize];
_21.2.0 = [RET,RET,RET,_14,RET,RET,RET];
_15.0.4 = _18;
_6 = _13;
_11 = [0_usize,0_usize,7_usize,1_usize,17126344135254654676_usize,15867918098009532096_usize];
_15.0.3 = 30_i8 as f32;
RET = !_14;
_21.0 = RET as u32;
_4 = _5;
_19 = _21.2.1;
_21.1 = [_15.2,_15.0.0,_15.2,_15.2,_15.0.0];
_15.2 = -_15.0.0;
_17 = core::ptr::addr_of_mut!((*_17));
_18 = _15.0.4;
_4 = [6037884110695520799_usize,9451768400893336513_usize,13148447088063267080_usize,15188613087394678085_usize,6_usize,10727541491402514952_usize];
_15.1 = core::ptr::addr_of_mut!(_15.0.0);
_7 = [7_usize,11208548370705688794_usize,5_usize,1_usize,2093617040312570904_usize,6_usize];
Goto(bb6)
}
bb6 = {
_1 = [7_usize,0_usize,3_usize,6_usize,5_usize,4_usize];
_17 = core::ptr::addr_of_mut!((*_17));
_5 = _9;
_9 = _4;
_18 = (-23543_i16) as i64;
(*_17) = false as u32;
_10 = [16679731832051340356_usize,1775727190005551796_usize,0_usize,17477668856974609322_usize,6466492861823612485_usize,6517822611665246254_usize];
_2 = [6846650662845542857_usize,2_usize,14072990403766526284_usize,1_usize,6_usize,0_usize];
_10 = [3745076139146385166_usize,7824607539232736452_usize,1967372600617388880_usize,1_usize,5450963464926575191_usize,2999272039649657059_usize];
_8 = _5;
_1 = _5;
_11 = _1;
_15.0.3 = 6459509471696701076_u64 as f32;
_2 = [1_usize,3_usize,16102703591030933424_usize,16396871465592725019_usize,8164905623123820302_usize,7_usize];
RET = 8968_i16 as u8;
_22 = !(*_17);
_25 = !9223372036854775807_isize;
_21.2.0 = [_14,_14,_14,_14,RET,_14,RET];
Goto(bb7)
}
bb7 = {
_7 = _3;
_15.0.4 = !_18;
_17 = core::ptr::addr_of_mut!((*_17));
Goto(bb8)
}
bb8 = {
_6 = [923302039443919082_usize,10472037556544863757_usize,7611476582387657822_usize,4874648477324640764_usize,6_usize,6_usize];
_4 = _8;
(*_17) = !_22;
_20 = [_18,_15.0.4,_15.0.4,_15.0.4,_18,_15.0.4];
_14 = RET;
_8 = _7;
_15.2 = _18 as i128;
(*_17) = _22;
(*_17) = _22 * _22;
_23 = [_15.0.0,_15.2,_15.0.0];
_8 = [6684870140073668914_usize,1863846219986769163_usize,7_usize,12768309318600229992_usize,7_usize,1_usize];
_3 = [4_usize,11904856340577022762_usize,4543917705363837634_usize,5_usize,6_usize,2_usize];
_16 = '\u{b15e3}';
_3 = [0_usize,12481744586992802329_usize,4587216966470593387_usize,1930929139743980937_usize,15784998804575987925_usize,3465629739694309507_usize];
_22 = (*_17) | (*_17);
_15.0.2 = core::ptr::addr_of_mut!(_15.0.0);
_21.2.1 = [8762353239093026558_u64,4108948545737061964_u64,1008757692661869677_u64,8713575535929344432_u64];
Goto(bb9)
}
bb9 = {
_22 = !(*_17);
_20 = [_18,_18,_18,_18,_18,_15.0.4];
_15.0.0 = _18 as i128;
_15.0.1 = core::ptr::addr_of_mut!(_15.0.0);
_29.0 = true as u32;
_29.1 = 9862_i16;
_15.0.0 = _15.2;
_28.1 = _18 as f64;
_11 = [3_usize,17160802178052234942_usize,9112834509097279054_usize,1_usize,13526147030569931220_usize,0_usize];
_21.2.1 = [7731516732915009755_u64,2229515431056933071_u64,2574454555204707013_u64,4723614245486156168_u64];
_1 = _6;
_10 = [7_usize,1_usize,0_usize,2_usize,6934456207144817448_usize,1818032229543166710_usize];
(*_17) = _22;
_11 = _5;
_23 = [_15.0.0,_15.0.0,_15.2];
Goto(bb10)
}
bb10 = {
_12 = _5;
_7 = [9583265430636549770_usize,5_usize,6_usize,0_usize,9387249005081919782_usize,3_usize];
_15.2 = _15.0.0 - _15.0.0;
_11 = [8169772940561715300_usize,16356286759419419247_usize,1_usize,5537906933518889509_usize,14350209662512372516_usize,12355979468271565540_usize];
_21.2.0 = [RET,RET,RET,RET,_14,_14,RET];
_28.1 = 99265811272310176648165311708008611077_u128 as f64;
_15.0.1 = _15.0.2;
_7 = [6_usize,17736206905717718823_usize,14400545771645257859_usize,16154864938580913425_usize,11327398550403560993_usize,2_usize];
_7 = _3;
_15.0.2 = core::ptr::addr_of_mut!(_15.2);
_29.1 = 1918_i16;
_15.1 = core::ptr::addr_of_mut!(_15.0.0);
Goto(bb11)
}
bb11 = {
RET = _14 >> _21.0;
Goto(bb12)
}
bb12 = {
_34.fld0 = RET;
_21.2.0 = [_14,_14,RET,RET,RET,_34.fld0,_34.fld0];
_33 = Adt46 { fld0: RET };
_31 = _18 as isize;
_15.2 = _15.0.0 | _15.0.0;
_15.0.0 = _15.2 + _15.2;
_9 = _6;
_35.1 = _29.1;
_29.0 = !_22;
_20 = [_15.0.4,_15.0.4,_15.0.4,_18,_15.0.4,_15.0.4];
_34.fld0 = _33.fld0 + _33.fld0;
_28.1 = _31 as f64;
_13 = _6;
_24 = false ^ false;
_31 = 63_i8 as isize;
Goto(bb13)
}
bb13 = {
_38.1 = _15.0.2;
_2 = [3_usize,2_usize,7_usize,7660920373358464683_usize,1292890605043168803_usize,1371408741178278523_usize];
_37 = -_31;
_12 = [13662803209499639774_usize,18176667658607926989_usize,7_usize,5_usize,6_usize,10504291790204183001_usize];
_15.0.3 = 15726_u16 as f32;
_38.0.2 = core::ptr::addr_of_mut!(_38.2);
_14 = _34.fld0;
_38.0.4 = -_18;
Goto(bb14)
}
bb14 = {
_38.0 = (_15.0.0, _15.1, _15.0.2, _15.0.3, _15.0.4);
RET = !_14;
_21.2.1 = [11221750677651906759_u64,18405581597848479213_u64,1331455461449595004_u64,136605222674284893_u64];
(*_17) = !_29.0;
_38.0.2 = _15.0.1;
_32 = Adt50::Variant0 { fld0: _18,fld1: Move(_34),fld2: 14406120295716889839672542206228850814_u128,fld3: 6_i8 };
_40.0 = _21.2.0;
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(5_usize, 20_usize, Move(_20), 22_usize, Move(_22), 24_usize, Move(_24), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(5_usize, 19_usize, Move(_19), 37_usize, Move(_37), 7_usize, Move(_7), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(5_usize, 6_usize, Move(_6), 5_usize, Move(_5), 29_usize, Move(_29), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(5_usize, 25_usize, Move(_25), 43_usize, _43, 43_usize, _43, 43_usize, _43), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: [usize; 6],mut _2: [usize; 6],mut _3: [usize; 6],mut _4: [usize; 6],mut _5: [usize; 6],mut _6: [usize; 6],mut _7: [usize; 6]) -> [usize; 6] {
mir! {
type RET = [usize; 6];
let _8: *mut i128;
let _9: bool;
let _10: f32;
let _11: [u16; 1];
let _12: *mut u32;
let _13: Adt53;
let _14: Adt56;
let _15: (u32, i16);
let _16: [u8; 7];
let _17: Adt46;
let _18: char;
let _19: ([u64; 4], u32, u8);
let _20: char;
let _21: Adt46;
let _22: isize;
let _23: [u64; 6];
let _24: [u64; 4];
let _25: isize;
let _26: Adt62;
let _27: i16;
let _28: i64;
let _29: [i128; 5];
let _30: u64;
let _31: [u8; 7];
let _32: Adt54;
let _33: (u32, i16);
let _34: ();
let _35: ();
{
_5 = [2932969886250837762_usize,4653914420104092332_usize,2242275155617661284_usize,6_usize,16873120262085434154_usize,3_usize];
_1 = _4;
RET = [7_usize,5_usize,3500775389289789320_usize,17599027554405422045_usize,0_usize,1_usize];
_5 = _7;
RET = [5_usize,6_usize,6_usize,3_usize,3343049895658044665_usize,4337921910314277127_usize];
RET = [0_usize,1_usize,9569100713320407326_usize,8393790756290007767_usize,7_usize,7_usize];
_4 = RET;
_1 = _4;
_3 = [2_usize,1_usize,2_usize,0_usize,7634177220731713011_usize,5_usize];
_9 = false;
_9 = 6025535936092753966_u64 != 3668231649319237894_u64;
_9 = true;
RET = _2;
_1 = [2_usize,4506341646773031202_usize,7983545196606351849_usize,0_usize,2_usize,15144588178659767151_usize];
RET = _1;
_3 = [4516066876334180266_usize,7_usize,10145186622859278087_usize,7538261451444575368_usize,4_usize,3_usize];
RET = [0_usize,1144167498328141584_usize,2_usize,4204384952876215208_usize,2188707327048953779_usize,4_usize];
Goto(bb1)
}
bb1 = {
_5 = _7;
_7 = RET;
_5 = [17806646991579472195_usize,7200141649737588670_usize,14212435334137373048_usize,17696678998359137457_usize,9537431531312477437_usize,3_usize];
_7 = [0_usize,1_usize,8825850194253240119_usize,15307584302472149237_usize,0_usize,5941210104936020618_usize];
_10 = 12131289705763479466_usize as f32;
_3 = [1600416763204824761_usize,5952973787141009570_usize,6120537196219247728_usize,13381385032687723449_usize,4_usize,1_usize];
_4 = RET;
RET = [5_usize,2975878726039971693_usize,1724577078519645644_usize,5_usize,8274388411713789853_usize,6_usize];
_10 = 117_u8 as f32;
_5 = _2;
_11 = [18594_u16];
_2 = [6842344318484870756_usize,4525695418755797537_usize,17656086964760527864_usize,8449643343572725286_usize,4_usize,6563415130446487396_usize];
_2 = _5;
Goto(bb2)
}
bb2 = {
_11 = [19716_u16];
_3 = [12148959667365713065_usize,6_usize,2_usize,16510998763825580095_usize,4_usize,12695294270502316486_usize];
_5 = [16315410263412451498_usize,3_usize,4440002371699329497_usize,4_usize,5_usize,1_usize];
_1 = [12505490482699063012_usize,6_usize,12313295475095753714_usize,4_usize,3_usize,1_usize];
_7 = _2;
RET = [14137484738133643988_usize,4588178360504047238_usize,2_usize,7_usize,4213286099089610914_usize,17155264895220119745_usize];
_9 = !false;
_6 = [4543430401123092881_usize,1_usize,12527553905098628505_usize,5_usize,5_usize,13504575339657305031_usize];
_10 = 291630237_i32 as f32;
_13.fld0 = _10;
_13.fld0 = _10;
_7 = [4_usize,0_usize,3085459826668424415_usize,6_usize,6_usize,2063563059228046642_usize];
_10 = (-132692666855847642282240099793417037808_i128) as f32;
_16 = [26_u8,59_u8,162_u8,38_u8,160_u8,29_u8,110_u8];
_4 = [2863165230538017248_usize,7_usize,2_usize,0_usize,5110543172484797674_usize,7285090443901177235_usize];
_15.0 = 2501229358_u32;
_3 = _5;
_13.fld0 = -_10;
_6 = _5;
RET = [7_usize,2893294871878744614_usize,13704759044367087751_usize,6845025313215493592_usize,3629757948669312088_usize,7_usize];
_16 = [237_u8,56_u8,165_u8,35_u8,151_u8,41_u8,15_u8];
_9 = false;
_5 = [14708774173771983264_usize,7_usize,6_usize,1647103898478791736_usize,1_usize,0_usize];
_7 = [8069501821448256775_usize,9456023207533577938_usize,4472721831447875563_usize,6954205809070496067_usize,2_usize,9176212844071109260_usize];
RET = _1;
match _15.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
2501229358 => bb10,
_ => bb9
}
}
bb3 = {
_5 = _7;
_7 = RET;
_5 = [17806646991579472195_usize,7200141649737588670_usize,14212435334137373048_usize,17696678998359137457_usize,9537431531312477437_usize,3_usize];
_7 = [0_usize,1_usize,8825850194253240119_usize,15307584302472149237_usize,0_usize,5941210104936020618_usize];
_10 = 12131289705763479466_usize as f32;
_3 = [1600416763204824761_usize,5952973787141009570_usize,6120537196219247728_usize,13381385032687723449_usize,4_usize,1_usize];
_4 = RET;
RET = [5_usize,2975878726039971693_usize,1724577078519645644_usize,5_usize,8274388411713789853_usize,6_usize];
_10 = 117_u8 as f32;
_5 = _2;
_11 = [18594_u16];
_2 = [6842344318484870756_usize,4525695418755797537_usize,17656086964760527864_usize,8449643343572725286_usize,4_usize,6563415130446487396_usize];
_2 = _5;
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
Return()
}
bb10 = {
_17.fld0 = 120_u8;
_13.fld0 = _10 + _10;
RET = _1;
_13.fld2 = core::ptr::addr_of!(_18);
_19.1 = !_15.0;
_14 = Adt56::Variant1 { fld0: Move(_17) };
_12 = core::ptr::addr_of_mut!(_15.0);
_17.fld0 = Field::<Adt46>(Variant(_14, 1), 0).fld0;
_19.0 = [9548295345827788072_u64,2247262621561998006_u64,12844082254087449957_u64,5143120395358365863_u64];
_20 = '\u{9af38}';
RET = [15114064524136354531_usize,16370291392170211237_usize,2225158746305388692_usize,5325463438239164592_usize,14310168923590592597_usize,6180501882157277649_usize];
_2 = [12752910523692669414_usize,3_usize,5_usize,3_usize,6079901274979351956_usize,1_usize];
_19.1 = (*_12);
_15 = (_19.1, (-6831_i16));
SetDiscriminant(_14, 0);
_19.1 = _15.0;
place!(Field::<Adt46>(Variant(_14, 0), 0)).fld0 = !_17.fld0;
RET = _2;
Goto(bb11)
}
bb11 = {
_2 = RET;
match (*_12) {
0 => bb1,
1 => bb10,
2 => bb7,
3 => bb12,
4 => bb13,
5 => bb14,
2501229358 => bb16,
_ => bb15
}
}
bb12 = {
_5 = _7;
_7 = RET;
_5 = [17806646991579472195_usize,7200141649737588670_usize,14212435334137373048_usize,17696678998359137457_usize,9537431531312477437_usize,3_usize];
_7 = [0_usize,1_usize,8825850194253240119_usize,15307584302472149237_usize,0_usize,5941210104936020618_usize];
_10 = 12131289705763479466_usize as f32;
_3 = [1600416763204824761_usize,5952973787141009570_usize,6120537196219247728_usize,13381385032687723449_usize,4_usize,1_usize];
_4 = RET;
RET = [5_usize,2975878726039971693_usize,1724577078519645644_usize,5_usize,8274388411713789853_usize,6_usize];
_10 = 117_u8 as f32;
_5 = _2;
_11 = [18594_u16];
_2 = [6842344318484870756_usize,4525695418755797537_usize,17656086964760527864_usize,8449643343572725286_usize,4_usize,6563415130446487396_usize];
_2 = _5;
Goto(bb2)
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
_7 = _2;
place!(Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(_14, 0), 3)).2 = (_16, _19.0);
_27 = (-38_isize) as i16;
_17 = Adt46 { fld0: Field::<Adt46>(Variant(_14, 0), 0).fld0 };
_25 = -9223372036854775807_isize;
place!(Field::<Adt46>(Variant(_14, 0), 0)) = Move(_17);
_12 = core::ptr::addr_of_mut!(_15.0);
_19.1 = !_15.0;
place!(Field::<usize>(Variant(_14, 0), 2)) = 10473119100929483210_usize >> _15.1;
_6 = _7;
(*_12) = !_19.1;
_13.fld1 = core::ptr::addr_of_mut!(_30);
_10 = -_13.fld0;
RET = [Field::<usize>(Variant(_14, 0), 2),Field::<usize>(Variant(_14, 0), 2),Field::<usize>(Variant(_14, 0), 2),Field::<usize>(Variant(_14, 0), 2),Field::<usize>(Variant(_14, 0), 2),Field::<usize>(Variant(_14, 0), 2)];
_25 = !(-88_isize);
place!(Field::<[u64; 4]>(Variant(_14, 0), 4)) = [11167397418321546100_u64,3793444705049722716_u64,9258218640734696044_u64,6335966942104910595_u64];
_23 = [2431091002899866332_u64,16492993944753047947_u64,3265803197657931032_u64,12169711753518763240_u64,16630387250298439924_u64,3383231136333714431_u64];
_28 = !1663283055119824456_i64;
_18 = _20;
_15 = (_19.1, _27);
_15.0 = _19.1;
_19 = (Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(_14, 0), 3).2.1, (*_12), Field::<Adt46>(Variant(_14, 0), 0).fld0);
_13.fld0 = _10;
_21 = Adt46 { fld0: _19.2 };
_5 = [Field::<usize>(Variant(_14, 0), 2),Field::<usize>(Variant(_14, 0), 2),Field::<usize>(Variant(_14, 0), 2),Field::<usize>(Variant(_14, 0), 2),Field::<usize>(Variant(_14, 0), 2),Field::<usize>(Variant(_14, 0), 2)];
Goto(bb17)
}
bb17 = {
Call(_34 = dump_var(6_usize, 3_usize, Move(_3), 25_usize, Move(_25), 7_usize, Move(_7), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(6_usize, 23_usize, Move(_23), 15_usize, Move(_15), 16_usize, Move(_16), 27_usize, Move(_27)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_34 = dump_var(6_usize, 19_usize, Move(_19), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: [usize; 6],mut _2: *mut i128,mut _3: [usize; 6],mut _4: [usize; 6],mut _5: [usize; 6],mut _6: [usize; 6]) -> i128 {
mir! {
type RET = i128;
let _7: [usize; 6];
let _8: *mut &'static u16;
let _9: Adt56;
let _10: Adt57;
let _11: isize;
let _12: Adt51;
let _13: isize;
let _14: [usize; 6];
let _15: *const char;
let _16: ();
let _17: ();
{
RET = (*_2) & (*_2);
RET = -(*_2);
RET = !(*_2);
_5 = [10234843077627845609_usize,2_usize,9760864366650200437_usize,0_usize,3_usize,7_usize];
_5 = [5_usize,5_usize,1_usize,18093777032967019839_usize,6_usize,5_usize];
(*_2) = !RET;
(*_2) = 8972732013728395690_u64 as i128;
_3 = [7531632709780317145_usize,6752108940959949068_usize,2166584795444967611_usize,8517332057919072327_usize,7070586143273469757_usize,7_usize];
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = 9962371917806521700_usize as i128;
(*_2) = -RET;
RET = (*_2);
RET = (-8025836298111573275_i64) as i128;
_6 = _3;
_5 = [13347001073147213082_usize,18322830449885637615_usize,7_usize,6_usize,6_usize,4_usize];
RET = (*_2) * (*_2);
_5 = _4;
(*_2) = -RET;
(*_2) = 1847721480_u32 as i128;
_5 = _1;
(*_2) = RET & RET;
(*_2) = RET;
_3 = [1_usize,4_usize,5947456229116239685_usize,7_usize,7_usize,3_usize];
RET = (*_2) & (*_2);
_2 = core::ptr::addr_of_mut!((*_2));
RET = (*_2);
Goto(bb1)
}
bb1 = {
(*_2) = 0_i8 as i128;
(*_2) = RET - RET;
_3 = [6_usize,8726167948673202276_usize,3039510727040410339_usize,1701374179243951018_usize,12038257639366102157_usize,12244623155918705162_usize];
(*_2) = RET;
_1 = [3_usize,2719160919245058952_usize,6566956536412313159_usize,0_usize,14402917004018775609_usize,12047087028531379285_usize];
_6 = [1_usize,4100656813977456094_usize,5_usize,18426295948668045825_usize,6_usize,9135005562357943007_usize];
_4 = _3;
_2 = core::ptr::addr_of_mut!((*_2));
_11 = !(-9223372036854775808_isize);
(*_2) = 7_usize as i128;
_11 = !120_isize;
(*_2) = -RET;
_5 = [4_usize,1_usize,18007205578544885273_usize,7135100377764879537_usize,1202926392717205838_usize,2_usize];
_2 = core::ptr::addr_of_mut!((*_2));
_11 = (-9223372036854775808_isize);
_6 = [0_usize,1_usize,2_usize,1422591871592899978_usize,1_usize,3_usize];
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = RET;
match _11 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463454151235394913435648 => bb7,
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
(*_2) = !RET;
Call(_9 = fn8(_2, _1, _1, (*_2), _1, RET, _2, _1, _4, _3, _2, _6, _1, _11), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_4 = _3;
place!(Field::<Adt46>(Variant(_9, 1), 0)) = Adt46 { fld0: 9_u8 };
_6 = _4;
place!(Field::<Adt46>(Variant(_9, 1), 0)) = Adt46 { fld0: 227_u8 };
_2 = core::ptr::addr_of_mut!((*_2));
RET = (*_2);
RET = !(*_2);
place!(Field::<Adt46>(Variant(_9, 1), 0)).fld0 = 19_i8 as u8;
_11 = (-111_isize);
_3 = _1;
_10 = Adt57::Variant3 { fld0: 2518481806757830126_i64,fld1: 3_usize };
Goto(bb9)
}
bb9 = {
_14 = [16248102751309073271_usize,13807922170311331251_usize,11388756320786201192_usize,4451978905807735842_usize,10075149775084163947_usize,17233038817125076246_usize];
RET = (*_2);
_7 = [10896439802492751497_usize,3_usize,13436720002021378704_usize,2_usize,3_usize,4_usize];
place!(Field::<i64>(Variant(_10, 3), 0)) = _11 as i64;
_2 = core::ptr::addr_of_mut!((*_2));
match _11 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
340282366920938463463374607431768211345 => bb17,
_ => bb16
}
}
bb10 = {
(*_2) = 0_i8 as i128;
(*_2) = RET - RET;
_3 = [6_usize,8726167948673202276_usize,3039510727040410339_usize,1701374179243951018_usize,12038257639366102157_usize,12244623155918705162_usize];
(*_2) = RET;
_1 = [3_usize,2719160919245058952_usize,6566956536412313159_usize,0_usize,14402917004018775609_usize,12047087028531379285_usize];
_6 = [1_usize,4100656813977456094_usize,5_usize,18426295948668045825_usize,6_usize,9135005562357943007_usize];
_4 = _3;
_2 = core::ptr::addr_of_mut!((*_2));
_11 = !(-9223372036854775808_isize);
(*_2) = 7_usize as i128;
_11 = !120_isize;
(*_2) = -RET;
_5 = [4_usize,1_usize,18007205578544885273_usize,7135100377764879537_usize,1202926392717205838_usize,2_usize];
_2 = core::ptr::addr_of_mut!((*_2));
_11 = (-9223372036854775808_isize);
_6 = [0_usize,1_usize,2_usize,1422591871592899978_usize,1_usize,3_usize];
_2 = core::ptr::addr_of_mut!((*_2));
(*_2) = RET;
match _11 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463454151235394913435648 => bb7,
_ => bb6
}
}
bb11 = {
(*_2) = !RET;
Call(_9 = fn8(_2, _1, _1, (*_2), _1, RET, _2, _1, _4, _3, _2, _6, _1, _11), ReturnTo(bb8), UnwindUnreachable())
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
Return()
}
bb17 = {
_5 = [4_usize,2_usize,4_usize,17946378879847115747_usize,3709865622675535459_usize,6031142605663953067_usize];
place!(Field::<i64>(Variant(_10, 3), 0)) = _11 as i64;
_13 = _11;
Goto(bb18)
}
bb18 = {
Call(_16 = dump_var(7_usize, 11_usize, Move(_11), 5_usize, Move(_5), 6_usize, Move(_6), 4_usize, Move(_4)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: *mut i128,mut _2: [usize; 6],mut _3: [usize; 6],mut _4: i128,mut _5: [usize; 6],mut _6: i128,mut _7: *mut i128,mut _8: [usize; 6],mut _9: [usize; 6],mut _10: [usize; 6],mut _11: *mut i128,mut _12: [usize; 6],mut _13: [usize; 6],mut _14: isize) -> Adt56 {
mir! {
type RET = Adt56;
let _15: isize;
let _16: ([u64; 4], u32, u8);
let _17: (u32, i16);
let _18: [i32; 7];
let _19: Adt46;
let _20: f64;
let _21: f32;
let _22: [u64; 6];
let _23: [u16; 1];
let _24: Adt50;
let _25: [i128; 3];
let _26: ([usize; 6], &'static u16, *const i128, u32, i8, i64);
let _27: (u32, [i128; 5], ([u8; 7], [u64; 4]));
let _28: f64;
let _29: isize;
let _30: isize;
let _31: ([u64; 4], u32, u8);
let _32: ([u8; 7], [u64; 4]);
let _33: *const i128;
let _34: bool;
let _35: char;
let _36: i128;
let _37: ([usize; 6], &'static u16, *const i128, u32, i8, i64);
let _38: Adt46;
let _39: Adt46;
let _40: bool;
let _41: *mut u32;
let _42: (u32, [i128; 5], ([u8; 7], [u64; 4]));
let _43: [usize; 6];
let _44: isize;
let _45: Adt51;
let _46: bool;
let _47: ();
let _48: ();
{
_4 = -(*_11);
_12 = [7860402469615381088_usize,7_usize,38220089591792280_usize,7_usize,6_usize,8196721954105804089_usize];
_4 = 116_i8 as i128;
_11 = _1;
_10 = [0_usize,1_usize,5325080550742682764_usize,1_usize,10789078674215308297_usize,6_usize];
_4 = _14 as i128;
_12 = [15281544044939073679_usize,11417999620465247395_usize,17836604999722695423_usize,18157255840977142293_usize,16669874166565781948_usize,5422431028203456896_usize];
(*_11) = (-44_i8) as i128;
_5 = [3784575150631768960_usize,935960033362621348_usize,10277429940505268934_usize,2_usize,10566256028955120419_usize,1185251379100827078_usize];
_11 = core::ptr::addr_of_mut!((*_11));
_14 = 72_u8 as isize;
Call((*_11) = fn9(_9, _3, _3, _3, _5, _5, _9), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = [0_usize,14715321832219009439_usize,7458984200414139896_usize,4_usize,5_usize,16839457851851290404_usize];
_14 = -(-72_isize);
_9 = _12;
_15 = 23_u8 as isize;
(*_1) = _6 + _6;
Goto(bb2)
}
bb2 = {
(*_7) = 169_u8 as i128;
_2 = _13;
(*_7) = -_4;
_9 = _2;
_2 = [16856196429012494848_usize,3_usize,7_usize,5_usize,8884882452576272131_usize,14109562806539684468_usize];
_6 = (*_7);
_17.0 = !369503603_u32;
_9 = [2566968593524475777_usize,4_usize,6_usize,11171974751179974903_usize,4_usize,1050074190109493883_usize];
_8 = [5_usize,7884484119419781426_usize,12235529170536884090_usize,6_usize,0_usize,2_usize];
_5 = _12;
Goto(bb3)
}
bb3 = {
_18 = [(-476093918_i32),(-2136266796_i32),(-874889321_i32),(-1178272659_i32),(-187554517_i32),1086249190_i32,75155960_i32];
(*_11) = _6;
_11 = core::ptr::addr_of_mut!((*_11));
_13 = [15265393829470026765_usize,10554761007018247205_usize,6_usize,17564318198701143612_usize,8335936541143090440_usize,2_usize];
(*_7) = !_4;
_19.fld0 = !249_u8;
_13 = [5_usize,4893408067460475095_usize,7827306867857073396_usize,7_usize,4733900777988062107_usize,7_usize];
_16.1 = 12815871099039056334_usize as u32;
_4 = -(*_11);
_17 = (_16.1, (-29211_i16));
_8 = [4_usize,14100366739244018910_usize,6_usize,7_usize,7400907837611400678_usize,4_usize];
_17.1 = -(-8907_i16);
_16.0 = [4994333933494251621_u64,2952727487697619840_u64,5619460322519197844_u64,12390435846169336940_u64];
_5 = _3;
_15 = _14 * _14;
_16.0 = [4486801885339320577_u64,13311061371232978568_u64,3110528454305834634_u64,9394638751745996305_u64];
_21 = 16328677135063339824_u64 as f32;
_2 = [4532642026626931187_usize,1_usize,5_usize,1277586403406556253_usize,3_usize,9250454650786244937_usize];
_12 = _5;
RET = Adt56::Variant1 { fld0: Move(_19) };
Goto(bb4)
}
bb4 = {
(*_1) = _6 * _4;
place!(Field::<Adt46>(Variant(RET, 1), 0)) = Adt46 { fld0: 90_u8 };
_1 = _7;
_21 = 10154878059603633872_usize as f32;
place!(Field::<Adt46>(Variant(RET, 1), 0)) = Adt46 { fld0: 111_u8 };
_16.2 = !Field::<Adt46>(Variant(RET, 1), 0).fld0;
_20 = _16.2 as f64;
_19.fld0 = _16.2 | _16.2;
_14 = _15;
_19.fld0 = _16.2 * Field::<Adt46>(Variant(RET, 1), 0).fld0;
_7 = _1;
_4 = (*_7);
place!(Field::<Adt46>(Variant(RET, 1), 0)).fld0 = _16.2 | _16.2;
(*_1) = 29223_u16 as i128;
_2 = [6003056575783382116_usize,6_usize,3_usize,7_usize,14463232273511507137_usize,3_usize];
Call((*_1) = core::intrinsics::transmute(_4), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_4 = 2434_u16 as i128;
_1 = core::ptr::addr_of_mut!(_6);
_19 = Adt46 { fld0: Field::<Adt46>(Variant(RET, 1), 0).fld0 };
_23 = [12413_u16];
_17.0 = (-116_i8) as u32;
_6 = 4_usize as i128;
_13 = _12;
_24 = Adt50::Variant0 { fld0: 8058246189322001352_i64,fld1: Move(Field::<Adt46>(Variant(RET, 1), 0)),fld2: 166180680859319898228648733056065792874_u128,fld3: 15_i8 };
_19 = Move(Field::<Adt46>(Variant(_24, 0), 1));
place!(Field::<Adt46>(Variant(_24, 0), 1)).fld0 = !_19.fld0;
(*_7) = _15 as i128;
_17.0 = 36218_u16 as u32;
_26.3 = _16.1;
place!(Field::<u128>(Variant(_24, 0), 2)) = !64086099867041021664260176119440002756_u128;
_16.0 = [16380897148343465260_u64,5670414773312871348_u64,10460629914384434774_u64,742677711149299010_u64];
_27.0 = 6613517676842384351_usize as u32;
RET = Adt56::Variant1 { fld0: Move(_19) };
_12 = _9;
_19.fld0 = _16.2 - Field::<Adt46>(Variant(RET, 1), 0).fld0;
_7 = _11;
_3 = _5;
place!(Field::<Adt46>(Variant(RET, 1), 0)) = Move(Field::<Adt46>(Variant(_24, 0), 1));
_6 = (*_7) | (*_11);
_21 = 7425607052878835659_u64 as f32;
place!(Field::<i64>(Variant(_24, 0), 0)) = !(-1858068589289655515_i64);
Call(_22 = fn15(_10, _9, _18), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_26.4 = 63_i8;
_26.3 = _27.0 ^ _17.0;
_23 = [1772_u16];
place!(Field::<Adt46>(Variant(_24, 0), 1)) = Adt46 { fld0: Field::<Adt46>(Variant(RET, 1), 0).fld0 };
_28 = Field::<i64>(Variant(_24, 0), 0) as f64;
_27.0 = _16.1 * _26.3;
_7 = _1;
(*_7) = !(*_11);
_4 = -(*_11);
RET = Adt56::Variant1 { fld0: Move(Field::<Adt46>(Variant(_24, 0), 1)) };
_14 = _15;
_25 = [(*_7),_4,(*_7)];
(*_7) = _4;
SetDiscriminant(RET, 1);
_27.2.1 = [13805774077653489677_u64,10818548726496596393_u64,1073039265401809341_u64,16834065367541455704_u64];
place!(Field::<Adt46>(Variant(RET, 1), 0)) = Adt46 { fld0: _19.fld0 };
_19 = Move(Field::<Adt46>(Variant(RET, 1), 0));
place!(Field::<u128>(Variant(_24, 0), 2)) = 162386652687494758188021937826786875155_u128 >> _14;
_9 = [6_usize,2673015431814381678_usize,7_usize,4_usize,10873475420764063881_usize,0_usize];
Call(_30 = core::intrinsics::bswap(_15), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_20 = 7036693449384369052_u64 as f64;
_18 = [(-1071573118_i32),(-2041064343_i32),213888173_i32,(-337673041_i32),198108563_i32,1196117742_i32,(-239720424_i32)];
_18 = [(-430670028_i32),1071975009_i32,(-2075250790_i32),(-1586883077_i32),1434298588_i32,625927846_i32,(-804292017_i32)];
_1 = core::ptr::addr_of_mut!((*_1));
_26.5 = !Field::<i64>(Variant(_24, 0), 0);
(*_7) = (*_11);
match _26.4 {
0 => bb8,
1 => bb9,
63 => bb11,
_ => bb10
}
}
bb8 = {
(*_7) = 169_u8 as i128;
_2 = _13;
(*_7) = -_4;
_9 = _2;
_2 = [16856196429012494848_usize,3_usize,7_usize,5_usize,8884882452576272131_usize,14109562806539684468_usize];
_6 = (*_7);
_17.0 = !369503603_u32;
_9 = [2566968593524475777_usize,4_usize,6_usize,11171974751179974903_usize,4_usize,1050074190109493883_usize];
_8 = [5_usize,7884484119419781426_usize,12235529170536884090_usize,6_usize,0_usize,2_usize];
_5 = _12;
Goto(bb3)
}
bb9 = {
_4 = 2434_u16 as i128;
_1 = core::ptr::addr_of_mut!(_6);
_19 = Adt46 { fld0: Field::<Adt46>(Variant(RET, 1), 0).fld0 };
_23 = [12413_u16];
_17.0 = (-116_i8) as u32;
_6 = 4_usize as i128;
_13 = _12;
_24 = Adt50::Variant0 { fld0: 8058246189322001352_i64,fld1: Move(Field::<Adt46>(Variant(RET, 1), 0)),fld2: 166180680859319898228648733056065792874_u128,fld3: 15_i8 };
_19 = Move(Field::<Adt46>(Variant(_24, 0), 1));
place!(Field::<Adt46>(Variant(_24, 0), 1)).fld0 = !_19.fld0;
(*_7) = _15 as i128;
_17.0 = 36218_u16 as u32;
_26.3 = _16.1;
place!(Field::<u128>(Variant(_24, 0), 2)) = !64086099867041021664260176119440002756_u128;
_16.0 = [16380897148343465260_u64,5670414773312871348_u64,10460629914384434774_u64,742677711149299010_u64];
_27.0 = 6613517676842384351_usize as u32;
RET = Adt56::Variant1 { fld0: Move(_19) };
_12 = _9;
_19.fld0 = _16.2 - Field::<Adt46>(Variant(RET, 1), 0).fld0;
_7 = _11;
_3 = _5;
place!(Field::<Adt46>(Variant(RET, 1), 0)) = Move(Field::<Adt46>(Variant(_24, 0), 1));
_6 = (*_7) | (*_11);
_21 = 7425607052878835659_u64 as f32;
place!(Field::<i64>(Variant(_24, 0), 0)) = !(-1858068589289655515_i64);
Call(_22 = fn15(_10, _9, _18), ReturnTo(bb6), UnwindUnreachable())
}
bb10 = {
_3 = [0_usize,14715321832219009439_usize,7458984200414139896_usize,4_usize,5_usize,16839457851851290404_usize];
_14 = -(-72_isize);
_9 = _12;
_15 = 23_u8 as isize;
(*_1) = _6 + _6;
Goto(bb2)
}
bb11 = {
place!(Field::<Adt46>(Variant(RET, 1), 0)) = Adt46 { fld0: _19.fld0 };
_25 = [_4,_6,(*_11)];
(*_7) = true as i128;
_34 = true;
(*_11) = -_4;
SetDiscriminant(RET, 0);
place!(Field::<f32>(Variant(RET, 0), 5)) = _28 as f32;
place!(Field::<[u64; 4]>(Variant(RET, 0), 4)) = _16.0;
_27.2.0 = [_16.2,_19.fld0,_16.2,_19.fld0,_19.fld0,_19.fld0,_19.fld0];
_21 = -Field::<f32>(Variant(RET, 0), 5);
_32 = (_27.2.0, _27.2.1);
_26.2 = core::ptr::addr_of!((*_1));
_35 = '\u{3ede4}';
_7 = core::ptr::addr_of_mut!(_36);
_1 = core::ptr::addr_of_mut!((*_1));
_26.0 = [12565097618964505565_usize,9613201875947359009_usize,1_usize,9297884464486399647_usize,9591058735120450806_usize,15498366147612344939_usize];
place!(Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(RET, 0), 3)).1 = [(*_1),(*_11),(*_11),(*_1),(*_11)];
place!(Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(RET, 0), 3)).1 = [(*_1),(*_11),(*_11),(*_11),(*_11)];
place!(Field::<Adt46>(Variant(_24, 0), 1)).fld0 = !_19.fld0;
_37.3 = _27.0 + _26.3;
_7 = _1;
match _26.4 {
0 => bb6,
1 => bb8,
2 => bb5,
3 => bb12,
4 => bb13,
5 => bb14,
6 => bb15,
63 => bb17,
_ => bb16
}
}
bb12 = {
_3 = [0_usize,14715321832219009439_usize,7458984200414139896_usize,4_usize,5_usize,16839457851851290404_usize];
_14 = -(-72_isize);
_9 = _12;
_15 = 23_u8 as isize;
(*_1) = _6 + _6;
Goto(bb2)
}
bb13 = {
_18 = [(-476093918_i32),(-2136266796_i32),(-874889321_i32),(-1178272659_i32),(-187554517_i32),1086249190_i32,75155960_i32];
(*_11) = _6;
_11 = core::ptr::addr_of_mut!((*_11));
_13 = [15265393829470026765_usize,10554761007018247205_usize,6_usize,17564318198701143612_usize,8335936541143090440_usize,2_usize];
(*_7) = !_4;
_19.fld0 = !249_u8;
_13 = [5_usize,4893408067460475095_usize,7827306867857073396_usize,7_usize,4733900777988062107_usize,7_usize];
_16.1 = 12815871099039056334_usize as u32;
_4 = -(*_11);
_17 = (_16.1, (-29211_i16));
_8 = [4_usize,14100366739244018910_usize,6_usize,7_usize,7400907837611400678_usize,4_usize];
_17.1 = -(-8907_i16);
_16.0 = [4994333933494251621_u64,2952727487697619840_u64,5619460322519197844_u64,12390435846169336940_u64];
_5 = _3;
_15 = _14 * _14;
_16.0 = [4486801885339320577_u64,13311061371232978568_u64,3110528454305834634_u64,9394638751745996305_u64];
_21 = 16328677135063339824_u64 as f32;
_2 = [4532642026626931187_usize,1_usize,5_usize,1277586403406556253_usize,3_usize,9250454650786244937_usize];
_12 = _5;
RET = Adt56::Variant1 { fld0: Move(_19) };
Goto(bb4)
}
bb14 = {
_3 = [0_usize,14715321832219009439_usize,7458984200414139896_usize,4_usize,5_usize,16839457851851290404_usize];
_14 = -(-72_isize);
_9 = _12;
_15 = 23_u8 as isize;
(*_1) = _6 + _6;
Goto(bb2)
}
bb15 = {
_20 = 7036693449384369052_u64 as f64;
_18 = [(-1071573118_i32),(-2041064343_i32),213888173_i32,(-337673041_i32),198108563_i32,1196117742_i32,(-239720424_i32)];
_18 = [(-430670028_i32),1071975009_i32,(-2075250790_i32),(-1586883077_i32),1434298588_i32,625927846_i32,(-804292017_i32)];
_1 = core::ptr::addr_of_mut!((*_1));
_26.5 = !Field::<i64>(Variant(_24, 0), 0);
(*_7) = (*_11);
match _26.4 {
0 => bb8,
1 => bb9,
63 => bb11,
_ => bb10
}
}
bb16 = {
(*_1) = _6 * _4;
place!(Field::<Adt46>(Variant(RET, 1), 0)) = Adt46 { fld0: 90_u8 };
_1 = _7;
_21 = 10154878059603633872_usize as f32;
place!(Field::<Adt46>(Variant(RET, 1), 0)) = Adt46 { fld0: 111_u8 };
_16.2 = !Field::<Adt46>(Variant(RET, 1), 0).fld0;
_20 = _16.2 as f64;
_19.fld0 = _16.2 | _16.2;
_14 = _15;
_19.fld0 = _16.2 * Field::<Adt46>(Variant(RET, 1), 0).fld0;
_7 = _1;
_4 = (*_7);
place!(Field::<Adt46>(Variant(RET, 1), 0)).fld0 = _16.2 | _16.2;
(*_1) = 29223_u16 as i128;
_2 = [6003056575783382116_usize,6_usize,3_usize,7_usize,14463232273511507137_usize,3_usize];
Call((*_1) = core::intrinsics::transmute(_4), ReturnTo(bb5), UnwindUnreachable())
}
bb17 = {
place!(Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(RET, 0), 3)).1 = [(*_11),(*_7),_6,_6,_4];
_17.1 = (-11817_i16);
_29 = !_14;
_17 = (_37.3, (-7508_i16));
_37.5 = -_26.5;
(*_7) = _4;
_28 = -_20;
_41 = core::ptr::addr_of_mut!(_37.3);
_18 = [463615021_i32,(-1854280438_i32),(-762174722_i32),1374704892_i32,276361716_i32,(-105123518_i32),(-1789221267_i32)];
_36 = _4;
(*_11) = (*_7) << _6;
place!(Field::<Adt46>(Variant(RET, 0), 0)) = Adt46 { fld0: Field::<Adt46>(Variant(_24, 0), 1).fld0 };
_33 = _26.2;
_14 = _15 | _15;
RET = Adt56::Variant1 { fld0: Move(Field::<Adt46>(Variant(_24, 0), 1)) };
_32.0 = [Field::<Adt46>(Variant(RET, 1), 0).fld0,_19.fld0,Field::<Adt46>(Variant(RET, 1), 0).fld0,_19.fld0,_16.2,Field::<Adt46>(Variant(RET, 1), 0).fld0,_19.fld0];
_27.2.1 = [886797012453790105_u64,2024331550330358499_u64,4079435905717196693_u64,12736073814564674085_u64];
_12 = _9;
_11 = core::ptr::addr_of_mut!(_6);
place!(Field::<Adt46>(Variant(RET, 1), 0)) = Adt46 { fld0: _19.fld0 };
_43 = [17055357973376359005_usize,5_usize,2_usize,4_usize,18036393144429599291_usize,4_usize];
_39 = Adt46 { fld0: _16.2 };
Goto(bb18)
}
bb18 = {
Call(_47 = dump_var(8_usize, 43_usize, Move(_43), 32_usize, Move(_32), 34_usize, Move(_34), 25_usize, Move(_25)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_47 = dump_var(8_usize, 13_usize, Move(_13), 36_usize, Move(_36), 22_usize, Move(_22), 2_usize, Move(_2)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_47 = dump_var(8_usize, 3_usize, Move(_3), 23_usize, Move(_23), 12_usize, Move(_12), 30_usize, Move(_30)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: [usize; 6],mut _2: [usize; 6],mut _3: [usize; 6],mut _4: [usize; 6],mut _5: [usize; 6],mut _6: [usize; 6],mut _7: [usize; 6]) -> i128 {
mir! {
type RET = i128;
let _8: Adt62;
let _9: f32;
let _10: u64;
let _11: *const (u32, &'static u16, &'static u16);
let _12: Adt53;
let _13: i64;
let _14: (u32, [i128; 5], ([u8; 7], [u64; 4]));
let _15: isize;
let _16: [u8; 7];
let _17: isize;
let _18: [u64; 4];
let _19: isize;
let _20: [i128; 3];
let _21: ((i128, *mut i128, *mut i128, f32, i64), *mut i128, i128);
let _22: [usize; 6];
let _23: u16;
let _24: f32;
let _25: isize;
let _26: isize;
let _27: *mut i128;
let _28: u16;
let _29: [u64; 6];
let _30: Adt56;
let _31: [u64; 6];
let _32: f64;
let _33: ();
let _34: ();
{
RET = 106021947415145504642332005856381559404_i128 + (-136943938618188676130610915255949503362_i128);
_3 = _1;
_4 = [2316842590203280774_usize,8303365440531327806_usize,13839884082384064225_usize,2_usize,7_usize,4_usize];
_1 = _4;
_3 = _4;
_2 = _1;
_6 = [3_usize,2_usize,7_usize,10647001332188509103_usize,12316293462539370830_usize,3040648656727230221_usize];
RET = 36879720983210898073646653843318645971_i128 * 84341926390018641839655417900825159964_i128;
_3 = [13758314520625827558_usize,16629352383307008903_usize,18086224161063247602_usize,12918724879271627071_usize,3_usize,1_usize];
_9 = 11419695409706989926_usize as f32;
_9 = (-1491384268_i32) as f32;
_9 = 80_u8 as f32;
_9 = 9060197503423313745_u64 as f32;
_6 = [3_usize,1_usize,12734285187124375748_usize,13840856797090732202_usize,3391734869246104307_usize,3_usize];
_1 = _2;
_7 = [13389617923708020766_usize,1541032723403585035_usize,12421518383816244016_usize,3283873387359509308_usize,17669793049698963183_usize,7793779833082705691_usize];
_4 = _6;
_5 = [15142678406778236522_usize,5347241366800611366_usize,6_usize,7_usize,4_usize,0_usize];
_10 = 14544637690599613364_u64;
_6 = [4_usize,5_usize,2230873594539066372_usize,6_usize,4_usize,7_usize];
_7 = _4;
Goto(bb1)
}
bb1 = {
_5 = _7;
_10 = !10289279963854854990_u64;
_9 = (-42_isize) as f32;
_7 = [4_usize,5772076104127249025_usize,1_usize,0_usize,0_usize,3_usize];
_9 = 4829592284890107639_i64 as f32;
_7 = _3;
_7 = [7_usize,13752466681411435901_usize,3_usize,6_usize,15998637785180837277_usize,2_usize];
_5 = [4417263127855396516_usize,9471902214749644266_usize,1061348563141938617_usize,123159658496669476_usize,13678447187336141816_usize,3_usize];
_2 = [9236295147900282651_usize,892512613918454675_usize,6_usize,4_usize,0_usize,7331088792566095744_usize];
_2 = [4569250557277448765_usize,2_usize,4_usize,4170090840368141746_usize,4_usize,2_usize];
_7 = [0_usize,3_usize,9636348632166666186_usize,1_usize,5_usize,1_usize];
_9 = 17523846400377858307_usize as f32;
_1 = [12657148601528347951_usize,2_usize,9033100021488676097_usize,1473354164832712723_usize,4_usize,1_usize];
_9 = (-79_i8) as f32;
_3 = [15234969123820564345_usize,1_usize,1_usize,4_usize,2_usize,3711934040684380383_usize];
_3 = [10455436372774070468_usize,11143715454160857263_usize,0_usize,4_usize,7_usize,0_usize];
_10 = 3450844890980862710_u64 * 16813414681930942146_u64;
_10 = !7116380814305418193_u64;
_7 = [10812195269992086644_usize,4338933105667447834_usize,10148363440686331388_usize,13740187390441565572_usize,7195474294749441563_usize,2_usize];
_6 = [3_usize,6_usize,0_usize,13333542620181134967_usize,17877049517871614596_usize,3_usize];
RET = 108632952242541223094426137652507720210_i128 & 53978304844256091218942389684234689220_i128;
RET = 1588154881499514583_usize as i128;
_1 = _4;
_7 = [16276428584588788018_usize,6_usize,1_usize,11484219223666648480_usize,1_usize,13257627869508670028_usize];
_5 = [15440045502918538267_usize,3_usize,5_usize,1_usize,12403116107253936191_usize,4_usize];
_12.fld0 = _9;
_12.fld0 = _9 + _9;
_9 = -_12.fld0;
Call(_7 = fn10(_2, _2, _1, _5, _4, _1, _3, _5, _12.fld0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_12.fld0 = _9 * _9;
_7 = [9704855349744102061_usize,13849661481910038025_usize,9725201733853188631_usize,11757046281880012926_usize,4_usize,7_usize];
_2 = _4;
_9 = _10 as f32;
_2 = _7;
_6 = [4864192623087494378_usize,2_usize,16401458512603132595_usize,1_usize,0_usize,7_usize];
_9 = -_12.fld0;
_12.fld0 = -_9;
_14.1 = [RET,RET,RET,RET,RET];
_12.fld0 = -_9;
_13 = (-5038387733927461628_i64) * (-5919606586169287277_i64);
_1 = [4_usize,5_usize,4_usize,0_usize,2_usize,0_usize];
RET = 5_usize as i128;
_14.1 = [RET,RET,RET,RET,RET];
_15 = !9223372036854775807_isize;
_3 = [13638676517956094632_usize,0_usize,3646709146957742914_usize,15376669927055502101_usize,1_usize,7_usize];
_2 = [1_usize,7611458454408286237_usize,7_usize,2_usize,11704126808086119911_usize,7000031661889135863_usize];
_1 = [6841064655387170003_usize,5226959752016860225_usize,8639838510714546909_usize,6_usize,1_usize,3_usize];
_14.2.0 = [110_u8,155_u8,90_u8,134_u8,205_u8,87_u8,179_u8];
Goto(bb3)
}
bb3 = {
_14.2.1 = [_10,_10,_10,_10];
_2 = [0_usize,3_usize,1_usize,17251206991632973621_usize,1_usize,14683871925906138934_usize];
_3 = [6191600250916512514_usize,3_usize,14112590435184306937_usize,5_usize,2504361748107550280_usize,13376657231991477071_usize];
_17 = _15 * _15;
_15 = _17;
_16 = [103_u8,183_u8,146_u8,131_u8,43_u8,67_u8,190_u8];
_12.fld1 = core::ptr::addr_of_mut!(_10);
_4 = [257203982721666336_usize,8978554405698877492_usize,16913983460078503099_usize,3804245059142578643_usize,3_usize,639186438335269803_usize];
_5 = [17877269335705101633_usize,9732129314963517530_usize,2533367944846010065_usize,3285380412419764031_usize,0_usize,9472028762864065809_usize];
_15 = RET as isize;
_17 = -_15;
_18 = [_10,_10,_10,_10];
_12.fld1 = core::ptr::addr_of_mut!(_10);
_1 = _3;
_14.2 = (_16, _18);
Call(_8 = fn11(_5, _14.2.0, _7), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
place!(Field::<(u32, i16)>(Variant(_8, 1), 3)).0 = 74715597843479563070591681662775372754_u128 as u32;
_14.0 = Field::<(u32, i16)>(Variant(_8, 1), 3).0 * Field::<(u32, i16)>(Variant(_8, 1), 3).0;
_14.2 = (_16, _18);
Goto(bb5)
}
bb5 = {
_3 = [3385767095847152344_usize,6326227029462523538_usize,3_usize,760147285213183149_usize,6608265763562507376_usize,15563708589278446575_usize];
place!(Field::<Adt46>(Variant(place!(Field::<Adt56>(Variant(_8, 1), 2)), 1), 0)) = Adt46 { fld0: 77_u8 };
_12.fld0 = Field::<(u32, i16)>(Variant(_8, 1), 3).1 as f32;
_16 = [Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0];
_17 = _15 * _15;
_12.fld1 = core::ptr::addr_of_mut!(_10);
_16 = [Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0];
_21.0.3 = 120522910623085664731613250523062009806_u128 as f32;
_19 = _15 << _13;
_21.0 = (RET, Field::<*mut i128>(Variant(_8, 1), 0), Field::<*mut i128>(Variant(_8, 1), 0), _9, _13);
_14.1 = [_21.0.0,RET,_21.0.0,RET,RET];
_19 = Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(_8, 1), 4).0 as isize;
place!(Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(_8, 1), 4)).1 = [RET,_21.0.0,_21.0.0,RET,RET];
_21.0.2 = core::ptr::addr_of_mut!(_21.2);
place!(Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(_8, 1), 4)).2 = (_14.2.0, _18);
_20 = [RET,_21.0.0,RET];
match Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
77 => bb9,
_ => bb8
}
}
bb6 = {
place!(Field::<(u32, i16)>(Variant(_8, 1), 3)).0 = 74715597843479563070591681662775372754_u128 as u32;
_14.0 = Field::<(u32, i16)>(Variant(_8, 1), 3).0 * Field::<(u32, i16)>(Variant(_8, 1), 3).0;
_14.2 = (_16, _18);
Goto(bb5)
}
bb7 = {
_5 = _7;
_10 = !10289279963854854990_u64;
_9 = (-42_isize) as f32;
_7 = [4_usize,5772076104127249025_usize,1_usize,0_usize,0_usize,3_usize];
_9 = 4829592284890107639_i64 as f32;
_7 = _3;
_7 = [7_usize,13752466681411435901_usize,3_usize,6_usize,15998637785180837277_usize,2_usize];
_5 = [4417263127855396516_usize,9471902214749644266_usize,1061348563141938617_usize,123159658496669476_usize,13678447187336141816_usize,3_usize];
_2 = [9236295147900282651_usize,892512613918454675_usize,6_usize,4_usize,0_usize,7331088792566095744_usize];
_2 = [4569250557277448765_usize,2_usize,4_usize,4170090840368141746_usize,4_usize,2_usize];
_7 = [0_usize,3_usize,9636348632166666186_usize,1_usize,5_usize,1_usize];
_9 = 17523846400377858307_usize as f32;
_1 = [12657148601528347951_usize,2_usize,9033100021488676097_usize,1473354164832712723_usize,4_usize,1_usize];
_9 = (-79_i8) as f32;
_3 = [15234969123820564345_usize,1_usize,1_usize,4_usize,2_usize,3711934040684380383_usize];
_3 = [10455436372774070468_usize,11143715454160857263_usize,0_usize,4_usize,7_usize,0_usize];
_10 = 3450844890980862710_u64 * 16813414681930942146_u64;
_10 = !7116380814305418193_u64;
_7 = [10812195269992086644_usize,4338933105667447834_usize,10148363440686331388_usize,13740187390441565572_usize,7195474294749441563_usize,2_usize];
_6 = [3_usize,6_usize,0_usize,13333542620181134967_usize,17877049517871614596_usize,3_usize];
RET = 108632952242541223094426137652507720210_i128 & 53978304844256091218942389684234689220_i128;
RET = 1588154881499514583_usize as i128;
_1 = _4;
_7 = [16276428584588788018_usize,6_usize,1_usize,11484219223666648480_usize,1_usize,13257627869508670028_usize];
_5 = [15440045502918538267_usize,3_usize,5_usize,1_usize,12403116107253936191_usize,4_usize];
_12.fld0 = _9;
_12.fld0 = _9 + _9;
_9 = -_12.fld0;
Call(_7 = fn10(_2, _2, _1, _5, _4, _1, _3, _5, _12.fld0), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_12.fld0 = _9 * _9;
_7 = [9704855349744102061_usize,13849661481910038025_usize,9725201733853188631_usize,11757046281880012926_usize,4_usize,7_usize];
_2 = _4;
_9 = _10 as f32;
_2 = _7;
_6 = [4864192623087494378_usize,2_usize,16401458512603132595_usize,1_usize,0_usize,7_usize];
_9 = -_12.fld0;
_12.fld0 = -_9;
_14.1 = [RET,RET,RET,RET,RET];
_12.fld0 = -_9;
_13 = (-5038387733927461628_i64) * (-5919606586169287277_i64);
_1 = [4_usize,5_usize,4_usize,0_usize,2_usize,0_usize];
RET = 5_usize as i128;
_14.1 = [RET,RET,RET,RET,RET];
_15 = !9223372036854775807_isize;
_3 = [13638676517956094632_usize,0_usize,3646709146957742914_usize,15376669927055502101_usize,1_usize,7_usize];
_2 = [1_usize,7611458454408286237_usize,7_usize,2_usize,11704126808086119911_usize,7000031661889135863_usize];
_1 = [6841064655387170003_usize,5226959752016860225_usize,8639838510714546909_usize,6_usize,1_usize,3_usize];
_14.2.0 = [110_u8,155_u8,90_u8,134_u8,205_u8,87_u8,179_u8];
Goto(bb3)
}
bb9 = {
place!(Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(_8, 1), 4)) = (_14.0, Field::<[i128; 5]>(Variant(_8, 1), 1), _14.2);
_1 = [2119870662785998025_usize,0_usize,6_usize,5_usize,3680808300665979860_usize,0_usize];
_17 = _15 >> _21.0.4;
_7 = [2_usize,3_usize,13536648728170313835_usize,15014591417228475537_usize,13768610953074894573_usize,5407463821030315629_usize];
_3 = [0_usize,5_usize,2_usize,13847669443355616761_usize,2_usize,5_usize];
place!(Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(_8, 1), 4)).2.0 = _14.2.0;
_22 = _4;
_20 = [RET,_21.0.0,RET];
_19 = _17 << Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(_8, 1), 4).0;
place!(Field::<Adt46>(Variant(place!(Field::<Adt56>(Variant(_8, 1), 2)), 1), 0)).fld0 = !114_u8;
Goto(bb10)
}
bb10 = {
_17 = _19;
_16 = [Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0];
place!(Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(_8, 1), 4)).1 = [_21.0.0,RET,RET,_21.0.0,RET];
place!(Field::<*mut i128>(Variant(_8, 1), 0)) = _21.0.1;
place!(Field::<Adt46>(Variant(place!(Field::<Adt56>(Variant(_8, 1), 2)), 1), 0)).fld0 = 153_u8 & 161_u8;
_24 = -_21.0.3;
_9 = _21.0.3 * _12.fld0;
place!(Field::<Adt46>(Variant(place!(Field::<Adt56>(Variant(_8, 1), 2)), 1), 0)) = Adt46 { fld0: 186_u8 };
_22 = _6;
_18 = _14.2.1;
_7 = [1024268934276600957_usize,7768559277069142751_usize,6_usize,406566002136172923_usize,247145157076154720_usize,0_usize];
_14.2 = Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(_8, 1), 4).2;
_12.fld1 = core::ptr::addr_of_mut!(_10);
_18 = [_10,_10,_10,_10];
_10 = 16998721177863715302_u64;
_1 = _6;
Goto(bb11)
}
bb11 = {
_12.fld0 = _24 - _9;
_15 = _17 & _17;
_14.2.1 = [_10,_10,_10,_10];
Goto(bb12)
}
bb12 = {
_21.0.1 = Field::<*mut i128>(Variant(_8, 1), 0);
_24 = _21.0.0 as f32;
_9 = -_12.fld0;
_14.1 = [_21.0.0,RET,_21.0.0,_21.0.0,_21.0.0];
_25 = !_17;
_14.1 = [RET,_21.0.0,_21.0.0,RET,RET];
place!(Field::<Adt46>(Variant(place!(Field::<Adt56>(Variant(_8, 1), 2)), 1), 0)) = Adt46 { fld0: 218_u8 };
_15 = -_17;
_19 = 5_usize as isize;
_3 = [4698020217266549335_usize,3699601676007211790_usize,7866512763358053737_usize,4_usize,5_usize,2_usize];
_21.2 = !RET;
_19 = false as isize;
_3 = [9090679328195284020_usize,4_usize,11531786816171103005_usize,7_usize,1_usize,1_usize];
place!(Field::<*mut i128>(Variant(_8, 1), 0)) = core::ptr::addr_of_mut!(RET);
_18 = [_10,_10,_10,_10];
place!(Field::<*mut i128>(Variant(_8, 1), 0)) = _21.0.1;
_20 = [RET,RET,_21.2];
_14.1 = Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(_8, 1), 4).1;
_26 = _17 * _15;
_21.0 = (RET, Field::<*mut i128>(Variant(_8, 1), 0), Field::<*mut i128>(Variant(_8, 1), 0), _12.fld0, _13);
_21.0.1 = Field::<*mut i128>(Variant(_8, 1), 0);
_5 = _4;
_14.1 = [_21.2,_21.2,_21.0.0,_21.2,_21.0.0];
_28 = !51167_u16;
_14.2.0 = [Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0,Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0];
_17 = _26;
_9 = _12.fld0 * _24;
match Field::<Adt46>(Variant(Field::<Adt56>(Variant(_8, 1), 2), 1), 0).fld0 {
218 => bb14,
_ => bb13
}
}
bb13 = {
place!(Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(_8, 1), 4)) = (_14.0, Field::<[i128; 5]>(Variant(_8, 1), 1), _14.2);
_1 = [2119870662785998025_usize,0_usize,6_usize,5_usize,3680808300665979860_usize,0_usize];
_17 = _15 >> _21.0.4;
_7 = [2_usize,3_usize,13536648728170313835_usize,15014591417228475537_usize,13768610953074894573_usize,5407463821030315629_usize];
_3 = [0_usize,5_usize,2_usize,13847669443355616761_usize,2_usize,5_usize];
place!(Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(_8, 1), 4)).2.0 = _14.2.0;
_22 = _4;
_20 = [RET,_21.0.0,RET];
_19 = _17 << Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(_8, 1), 4).0;
place!(Field::<Adt46>(Variant(place!(Field::<Adt56>(Variant(_8, 1), 2)), 1), 0)).fld0 = !114_u8;
Goto(bb10)
}
bb14 = {
_21.0.3 = _10 as f32;
_10 = 5274642054700992161_u64;
_20 = [_21.0.0,RET,_21.2];
_21.0.2 = core::ptr::addr_of_mut!(_21.0.0);
_19 = false as isize;
_28 = !33106_u16;
_3 = _6;
_28 = !29021_u16;
_17 = -_25;
_25 = _26 & _17;
_14.1 = [RET,_21.0.0,RET,_21.2,RET];
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(9_usize, 28_usize, Move(_28), 15_usize, Move(_15), 20_usize, Move(_20), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(9_usize, 16_usize, Move(_16), 6_usize, Move(_6), 17_usize, Move(_17), 25_usize, Move(_25)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(9_usize, 10_usize, Move(_10), 2_usize, Move(_2), 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: [usize; 6],mut _2: [usize; 6],mut _3: [usize; 6],mut _4: [usize; 6],mut _5: [usize; 6],mut _6: [usize; 6],mut _7: [usize; 6],mut _8: [usize; 6],mut _9: f32) -> [usize; 6] {
mir! {
type RET = [usize; 6];
let _10: f32;
let _11: u32;
let _12: *const (u32, &'static u16, &'static u16);
let _13: u8;
let _14: i64;
let _15: *const i128;
let _16: Adt60;
let _17: f32;
let _18: i16;
let _19: Adt53;
let _20: *mut u32;
let _21: isize;
let _22: i32;
let _23: ([u8; 7], [u64; 4]);
let _24: *const (u32, &'static u16, &'static u16);
let _25: ([u8; 7], [u64; 4]);
let _26: i16;
let _27: u32;
let _28: f64;
let _29: ();
let _30: ();
{
RET = _5;
RET = [14311842613273315969_usize,16725764478668778349_usize,11305410870181098758_usize,6_usize,12142198634605647996_usize,5_usize];
_1 = _3;
Call(_1 = core::intrinsics::transmute(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = [16305833873858249147_usize,14918260740584594782_usize,4_usize,14151400741391601161_usize,7594200400200126592_usize,15034522061783290197_usize];
_3 = [6_usize,9410287652605824523_usize,3_usize,7_usize,14047498483164752921_usize,0_usize];
_5 = _2;
_10 = -_9;
RET = [5923070389594241802_usize,2626802772649680421_usize,6_usize,6_usize,3_usize,5040312452591481564_usize];
_2 = [5973315209772235710_usize,3_usize,5_usize,9915139430292578714_usize,1064290532158310340_usize,3514298589011109585_usize];
_9 = 9223372036854775807_isize as f32;
RET = [13147475410402578510_usize,2207602293468951748_usize,10515878662632144808_usize,4_usize,4317803176776800531_usize,2849780463297238783_usize];
_10 = 8525687723387621969_i64 as f32;
_13 = (-123_isize) as u8;
_10 = _9 - _9;
_5 = _6;
RET = _8;
_4 = [7_usize,2_usize,3_usize,18065899846497837982_usize,4_usize,171100862043562307_usize];
_14 = !(-8486854367791767325_i64);
_13 = 7452287266943611611794811549688649621_u128 as u8;
_10 = (-13689_i16) as f32;
_3 = [3_usize,14070915654672231521_usize,117303185377121466_usize,1_usize,16189532395936914807_usize,757245463776420198_usize];
_7 = [12221418231813034394_usize,5_usize,6688440434413506427_usize,4_usize,6_usize,6_usize];
_4 = _3;
_14 = 12330935321119607504_u64 as i64;
_4 = RET;
_5 = [6_usize,5323748270445268103_usize,3_usize,15843910637044012477_usize,13683998264321812116_usize,6389341711451352659_usize];
_13 = !88_u8;
Goto(bb2)
}
bb2 = {
_1 = [5_usize,2_usize,9298920492302914556_usize,9142279873520312796_usize,0_usize,1279109098904070677_usize];
_9 = _10 - _10;
_11 = !3404014972_u32;
_13 = 5_usize as u8;
_6 = [611144375405726263_usize,7181159433875196780_usize,3_usize,7_usize,15719633294151149484_usize,5305774161102060586_usize];
RET = [7_usize,11618174132767659458_usize,12855487526336731219_usize,14820503534670707987_usize,1282294775455128496_usize,2_usize];
_13 = 176_u8;
_2 = [4970893267540877532_usize,3_usize,5_usize,16924353219920549473_usize,7_usize,1_usize];
_4 = _1;
_7 = _5;
_18 = (-64_i8) as i16;
_14 = 55337_u16 as i64;
_9 = 8602938393252932282_u64 as f32;
_10 = _9;
match _13 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
176 => bb11,
_ => bb10
}
}
bb3 = {
_1 = [16305833873858249147_usize,14918260740584594782_usize,4_usize,14151400741391601161_usize,7594200400200126592_usize,15034522061783290197_usize];
_3 = [6_usize,9410287652605824523_usize,3_usize,7_usize,14047498483164752921_usize,0_usize];
_5 = _2;
_10 = -_9;
RET = [5923070389594241802_usize,2626802772649680421_usize,6_usize,6_usize,3_usize,5040312452591481564_usize];
_2 = [5973315209772235710_usize,3_usize,5_usize,9915139430292578714_usize,1064290532158310340_usize,3514298589011109585_usize];
_9 = 9223372036854775807_isize as f32;
RET = [13147475410402578510_usize,2207602293468951748_usize,10515878662632144808_usize,4_usize,4317803176776800531_usize,2849780463297238783_usize];
_10 = 8525687723387621969_i64 as f32;
_13 = (-123_isize) as u8;
_10 = _9 - _9;
_5 = _6;
RET = _8;
_4 = [7_usize,2_usize,3_usize,18065899846497837982_usize,4_usize,171100862043562307_usize];
_14 = !(-8486854367791767325_i64);
_13 = 7452287266943611611794811549688649621_u128 as u8;
_10 = (-13689_i16) as f32;
_3 = [3_usize,14070915654672231521_usize,117303185377121466_usize,1_usize,16189532395936914807_usize,757245463776420198_usize];
_7 = [12221418231813034394_usize,5_usize,6688440434413506427_usize,4_usize,6_usize,6_usize];
_4 = _3;
_14 = 12330935321119607504_u64 as i64;
_4 = RET;
_5 = [6_usize,5323748270445268103_usize,3_usize,15843910637044012477_usize,13683998264321812116_usize,6389341711451352659_usize];
_13 = !88_u8;
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
Return()
}
bb10 = {
Return()
}
bb11 = {
_7 = _3;
_20 = core::ptr::addr_of_mut!(_11);
_2 = [0_usize,3_usize,1777939474386233002_usize,4_usize,1_usize,5_usize];
match _13 {
176 => bb12,
_ => bb6
}
}
bb12 = {
_14 = (-359890434997935798_i64) + 4931802923785456837_i64;
_8 = [2_usize,9587753413913619055_usize,10022287743142143560_usize,1281076577630819281_usize,12850510840866415293_usize,6224646134638131201_usize];
(*_20) = 3630370785_u32 - 420288716_u32;
_8 = _5;
Goto(bb13)
}
bb13 = {
_19.fld0 = 64646759230554220147516394493205647545_u128 as f32;
_22 = (-1901783513_i32);
_21 = -(-9223372036854775808_isize);
_7 = [6_usize,2_usize,17966073190636577111_usize,4_usize,11388163352402784313_usize,1_usize];
_22 = 206897202_i32;
_6 = [14195844310908323802_usize,6_usize,0_usize,4314663382376223489_usize,16979794333015777092_usize,14072602702163027565_usize];
_22 = 133280573671246196962720159755048355312_u128 as i32;
_11 = !110182513_u32;
_8 = [12045909333523241646_usize,6_usize,4_usize,3033550646242027622_usize,12876500770955805512_usize,1039160176561917947_usize];
_18 = !24694_i16;
_14 = 7990000005568896039_i64;
(*_20) = 17087159342618119326_u64 as u32;
_2 = [6_usize,17342809602338390892_usize,0_usize,0_usize,14428154594809150440_usize,17557476819337463310_usize];
_2 = [2107130194667724064_usize,5_usize,14592056735287782595_usize,0_usize,17836764267439426820_usize,17378973606839232194_usize];
_19.fld0 = _9 + _10;
_23.1 = [11582382139812999035_u64,9529991880072134713_u64,6104737195094739119_u64,10550704777790364385_u64];
RET = [4715941243497146829_usize,13520569377878180287_usize,2_usize,4231789819863256535_usize,11173016725703635167_usize,10195439295393770320_usize];
_23.0 = [_13,_13,_13,_13,_13,_13,_13];
_23.1 = [17432109903065810410_u64,14768042443311142173_u64,11472411065165940764_u64,10820599055818355268_u64];
_23.0 = [_13,_13,_13,_13,_13,_13,_13];
_19.fld0 = _9 * _9;
(*_20) = 3522479447_u32 * 3990832981_u32;
_19.fld0 = _10;
Goto(bb14)
}
bb14 = {
_10 = 141308222099790787138513787949307098205_i128 as f32;
_2 = _1;
_18 = 22617_i16 | (-16853_i16);
_17 = _9 - _9;
_22 = !(-594605157_i32);
_23.0 = [_13,_13,_13,_13,_13,_13,_13];
_25.1 = _23.1;
_1 = RET;
RET = [7960086294732572451_usize,11183438412695861049_usize,0_usize,7_usize,2438682260851275407_usize,7_usize];
RET = [7896630789340112423_usize,12695008701311612449_usize,13856119957684755254_usize,9685334933846741581_usize,3265849721798222711_usize,3_usize];
_25 = (_23.0, _23.1);
_25.1 = [16414686234078726651_u64,14012908090728150693_u64,1362099334776625324_u64,13178022387533104538_u64];
_25.1 = [8438871600396291592_u64,11679171651074834155_u64,10079400711104370405_u64,6612971404805348096_u64];
RET = [7623874926957700998_usize,4_usize,3474157872269423814_usize,4266287098605752733_usize,0_usize,11152407448529325339_usize];
_26 = 179953990854551308671230199019411125998_u128 as i16;
_21 = _22 as isize;
_18 = '\u{45fdd}' as i16;
_13 = !167_u8;
RET = [13049404111498866828_usize,2065048248363425135_usize,1_usize,3_usize,17677415387257174469_usize,0_usize];
_11 = 4208985869_u32;
_3 = [2_usize,3_usize,6_usize,1_usize,8764352590605964340_usize,6213831762813339087_usize];
RET = _6;
_10 = _19.fld0 - _17;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(10_usize, 26_usize, Move(_26), 4_usize, Move(_4), 2_usize, Move(_2), 14_usize, Move(_14)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(10_usize, 3_usize, Move(_3), 22_usize, Move(_22), 8_usize, Move(_8), 21_usize, Move(_21)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: [usize; 6],mut _2: [u8; 7],mut _3: [usize; 6]) -> Adt62 {
mir! {
type RET = Adt62;
let _4: [i64; 6];
let _5: isize;
let _6: f32;
let _7: char;
let _8: Adt46;
let _9: (i128, *mut i128, *mut i128, f32, i64);
let _10: f64;
let _11: i64;
let _12: [i32; 7];
let _13: bool;
let _14: [i128; 3];
let _15: u16;
let _16: Adt61;
let _17: u32;
let _18: isize;
let _19: Adt58;
let _20: (u32, i16);
let _21: Adt60;
let _22: i16;
let _23: f32;
let _24: [i128; 3];
let _25: [i32; 7];
let _26: (u32, [i128; 5], ([u8; 7], [u64; 4]));
let _27: [u64; 6];
let _28: isize;
let _29: [usize; 6];
let _30: isize;
let _31: [i32; 7];
let _32: bool;
let _33: [u64; 4];
let _34: isize;
let _35: [usize; 6];
let _36: *mut u64;
let _37: isize;
let _38: Adt50;
let _39: [i128; 5];
let _40: Adt59;
let _41: *mut u16;
let _42: Adt56;
let _43: Adt50;
let _44: isize;
let _45: f32;
let _46: i8;
let _47: [i32; 7];
let _48: u16;
let _49: u128;
let _50: [i64; 6];
let _51: i128;
let _52: f32;
let _53: f64;
let _54: i32;
let _55: Adt53;
let _56: [u64; 4];
let _57: ();
let _58: ();
{
_1 = [17270231680915161400_usize,9758980322033747923_usize,1_usize,2_usize,3180257387188027780_usize,5682407988396866065_usize];
_1 = _3;
_2 = [63_u8,94_u8,40_u8,130_u8,199_u8,122_u8,45_u8];
_1 = [12767689614593261221_usize,7_usize,3651912257669289425_usize,0_usize,6199123644284880656_usize,3_usize];
_1 = _3;
_3 = [6_usize,7_usize,11710419775319406326_usize,1_usize,6519287638407074293_usize,17661006955588569682_usize];
_3 = [6_usize,5_usize,7_usize,13069782564532520280_usize,2_usize,7_usize];
_3 = [12972424700638743329_usize,9785905608512538282_usize,5_usize,12172383128476295827_usize,16658918230881858492_usize,1851775790121049226_usize];
_2 = [202_u8,136_u8,211_u8,35_u8,170_u8,161_u8,219_u8];
Call(_4 = core::intrinsics::transmute(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = [254_u8,164_u8,172_u8,89_u8,68_u8,35_u8,27_u8];
_2 = [42_u8,232_u8,149_u8,79_u8,7_u8,183_u8,29_u8];
_3 = _1;
_1 = _3;
_1 = _3;
_3 = [5_usize,8495262148257949794_usize,1234886146931490905_usize,9363767465286899740_usize,16211672287506825925_usize,17508404950053149757_usize];
_4 = [4499281871616697630_i64,(-6535192831358762811_i64),3766825126601551658_i64,7703102923650085728_i64,2803347946323581985_i64,2137294000817194791_i64];
_1 = _3;
_2 = [198_u8,65_u8,21_u8,181_u8,177_u8,71_u8,222_u8];
_2 = [116_u8,55_u8,24_u8,39_u8,17_u8,34_u8,244_u8];
_5 = !(-9223372036854775808_isize);
_4 = [4104308270231372460_i64,5135529235206991216_i64,9132174183875925523_i64,(-33497378242385522_i64),8112361026991435505_i64,8028868246848501242_i64];
_4 = [6692476310694247751_i64,8969522972965528585_i64,(-5956777261609576254_i64),(-1127637112126744881_i64),593244597096695506_i64,4626649079473976512_i64];
_4 = [7794631727802790803_i64,7727115117831589078_i64,6991354590766843217_i64,(-1659518972237610808_i64),385222192599165144_i64,(-220295123815178203_i64)];
_6 = 147_u8 as f32;
_5 = -94_isize;
_5 = 195913538421721368529202923415271020664_u128 as isize;
_1 = _3;
_7 = '\u{f17a0}';
_7 = '\u{d4367}';
_4 = [(-5467887722195682236_i64),(-3493473797662853816_i64),509349643516396265_i64,3221875863545707763_i64,8855322469467824921_i64,3058158935395458460_i64];
_1 = _3;
Goto(bb2)
}
bb2 = {
_7 = '\u{c1e2e}';
_3 = [4960337533098790709_usize,5_usize,2_usize,1_usize,9022878566401183202_usize,2_usize];
_7 = '\u{eb692}';
_4 = [(-166772909513863873_i64),(-6172283678206354086_i64),2561437579584931415_i64,5822580885061447938_i64,2460251131141667967_i64,(-724275588488255582_i64)];
Goto(bb3)
}
bb3 = {
_4 = [(-1042391596870371288_i64),(-7503744802289170502_i64),(-7698639525157655930_i64),7316424821627899724_i64,(-2573363205121665580_i64),253543041992959095_i64];
_4 = [(-2833218557785621650_i64),8715503555736910112_i64,(-298625540840539877_i64),(-2650903432689227970_i64),(-2027469722787239511_i64),3997508207694097311_i64];
_7 = '\u{bbea8}';
_9.0 = (-10098936225148503303400531651431152344_i128);
_4 = [(-5876475503204380489_i64),(-1263975654039237604_i64),8639750293275430137_i64,8811924474866747467_i64,(-6517579436753265724_i64),(-1935877733773353977_i64)];
_9.1 = core::ptr::addr_of_mut!(_9.0);
_9.3 = _6 - _6;
_9.4 = !1402258966718552523_i64;
_8.fld0 = 132_u8 << _9.4;
_7 = '\u{101fcb}';
_2 = [_8.fld0,_8.fld0,_8.fld0,_8.fld0,_8.fld0,_8.fld0,_8.fld0];
match _9.0 {
0 => bb4,
330183430695789960159974075780337059112 => bb6,
_ => bb5
}
}
bb4 = {
_7 = '\u{c1e2e}';
_3 = [4960337533098790709_usize,5_usize,2_usize,1_usize,9022878566401183202_usize,2_usize];
_7 = '\u{eb692}';
_4 = [(-166772909513863873_i64),(-6172283678206354086_i64),2561437579584931415_i64,5822580885061447938_i64,2460251131141667967_i64,(-724275588488255582_i64)];
Goto(bb3)
}
bb5 = {
_2 = [254_u8,164_u8,172_u8,89_u8,68_u8,35_u8,27_u8];
_2 = [42_u8,232_u8,149_u8,79_u8,7_u8,183_u8,29_u8];
_3 = _1;
_1 = _3;
_1 = _3;
_3 = [5_usize,8495262148257949794_usize,1234886146931490905_usize,9363767465286899740_usize,16211672287506825925_usize,17508404950053149757_usize];
_4 = [4499281871616697630_i64,(-6535192831358762811_i64),3766825126601551658_i64,7703102923650085728_i64,2803347946323581985_i64,2137294000817194791_i64];
_1 = _3;
_2 = [198_u8,65_u8,21_u8,181_u8,177_u8,71_u8,222_u8];
_2 = [116_u8,55_u8,24_u8,39_u8,17_u8,34_u8,244_u8];
_5 = !(-9223372036854775808_isize);
_4 = [4104308270231372460_i64,5135529235206991216_i64,9132174183875925523_i64,(-33497378242385522_i64),8112361026991435505_i64,8028868246848501242_i64];
_4 = [6692476310694247751_i64,8969522972965528585_i64,(-5956777261609576254_i64),(-1127637112126744881_i64),593244597096695506_i64,4626649079473976512_i64];
_4 = [7794631727802790803_i64,7727115117831589078_i64,6991354590766843217_i64,(-1659518972237610808_i64),385222192599165144_i64,(-220295123815178203_i64)];
_6 = 147_u8 as f32;
_5 = -94_isize;
_5 = 195913538421721368529202923415271020664_u128 as isize;
_1 = _3;
_7 = '\u{f17a0}';
_7 = '\u{d4367}';
_4 = [(-5467887722195682236_i64),(-3493473797662853816_i64),509349643516396265_i64,3221875863545707763_i64,8855322469467824921_i64,3058158935395458460_i64];
_1 = _3;
Goto(bb2)
}
bb6 = {
_4 = [_9.4,_9.4,_9.4,_9.4,_9.4,_9.4];
_9.0 = !(-32050718456928936163955825970589746167_i128);
_4 = [_9.4,_9.4,_9.4,_9.4,_9.4,_9.4];
_9.4 = 172856673181104032626295109902874135434_u128 as i64;
_10 = 6_usize as f64;
_1 = [7564566672211409460_usize,15165302642014896867_usize,1550061230230451290_usize,1892909788705973788_usize,0_usize,0_usize];
_9.0 = -(-37768140247439264666846045230703963412_i128);
_9.3 = 274205644285057884086480650048635605625_u128 as f32;
_3 = [5_usize,0_usize,7884136570871984645_usize,478144912112714943_usize,15108767381307099817_usize,7_usize];
_10 = _9.4 as f64;
_4 = [_9.4,_9.4,_9.4,_9.4,_9.4,_9.4];
_9.2 = core::ptr::addr_of_mut!(_9.0);
_6 = _9.3;
_7 = '\u{d5814}';
_9.1 = core::ptr::addr_of_mut!(_9.0);
Goto(bb7)
}
bb7 = {
_13 = !false;
_9.3 = -_6;
_9.2 = _9.1;
_9.2 = _9.1;
_6 = -_9.3;
_12 = [(-1258984584_i32),(-1279599796_i32),725847054_i32,2053849558_i32,1434353661_i32,1258174494_i32,1517312260_i32];
_9.2 = core::ptr::addr_of_mut!(_9.0);
_12 = [(-940848134_i32),428515818_i32,(-1476027421_i32),11755974_i32,(-929973642_i32),70387749_i32,1636504531_i32];
_10 = 7_usize as f64;
_9.3 = -_6;
_13 = !true;
_11 = !_9.4;
_15 = 58983_u16;
_9.4 = _11 | _11;
_10 = _8.fld0 as f64;
_7 = '\u{106776}';
_15 = 58912_u16 << _11;
_9.3 = 169457488070935095628079881235562120351_u128 as f32;
_8.fld0 = !22_u8;
_4 = [_11,_11,_11,_9.4,_9.4,_9.4];
_14 = [_9.0,_9.0,_9.0];
_8 = Adt46 { fld0: 23_u8 };
_17 = 985550939_u32;
_1 = _3;
_10 = _9.4 as f64;
match _17 {
0 => bb1,
1 => bb6,
2 => bb3,
3 => bb5,
985550939 => bb9,
_ => bb8
}
}
bb8 = {
_4 = [_9.4,_9.4,_9.4,_9.4,_9.4,_9.4];
_9.0 = !(-32050718456928936163955825970589746167_i128);
_4 = [_9.4,_9.4,_9.4,_9.4,_9.4,_9.4];
_9.4 = 172856673181104032626295109902874135434_u128 as i64;
_10 = 6_usize as f64;
_1 = [7564566672211409460_usize,15165302642014896867_usize,1550061230230451290_usize,1892909788705973788_usize,0_usize,0_usize];
_9.0 = -(-37768140247439264666846045230703963412_i128);
_9.3 = 274205644285057884086480650048635605625_u128 as f32;
_3 = [5_usize,0_usize,7884136570871984645_usize,478144912112714943_usize,15108767381307099817_usize,7_usize];
_10 = _9.4 as f64;
_4 = [_9.4,_9.4,_9.4,_9.4,_9.4,_9.4];
_9.2 = core::ptr::addr_of_mut!(_9.0);
_6 = _9.3;
_7 = '\u{d5814}';
_9.1 = core::ptr::addr_of_mut!(_9.0);
Goto(bb7)
}
bb9 = {
_11 = !_9.4;
_3 = [4_usize,15601006817079645180_usize,16122887303419897263_usize,3_usize,17148167193460673182_usize,6_usize];
_15 = _10 as u16;
_18 = 497210606_i32 as isize;
_18 = 1994142485_i32 as isize;
_22 = 17950_i16 * 5815_i16;
_20.0 = _10 as u32;
_9.3 = _6 + _6;
_20 = (_17, _22);
_24 = [_9.0,_9.0,_9.0];
match _20.0 {
0 => bb1,
1 => bb8,
2 => bb6,
985550939 => bb10,
_ => bb4
}
}
bb10 = {
_17 = _20.0 & _20.0;
_25 = _12;
_9.2 = _9.1;
_9.0 = _8.fld0 as i128;
_26.2.0 = [_8.fld0,_8.fld0,_8.fld0,_8.fld0,_8.fld0,_8.fld0,_8.fld0];
_6 = -_9.3;
_8 = Adt46 { fld0: 247_u8 };
_23 = _9.3;
_26.0 = !_17;
_7 = '\u{f3fbe}';
_4 = [_11,_11,_9.4,_11,_9.4,_11];
_30 = _5 ^ _18;
_8.fld0 = 117447827529451772476226297916065137878_u128 as u8;
_26.0 = _17;
_28 = (-1357262716_i32) as isize;
_17 = _26.0 % _20.0;
_6 = 0_usize as f32;
_25 = [(-1716282436_i32),1535550124_i32,651313707_i32,(-721720556_i32),(-347896839_i32),(-1677747844_i32),(-1243475447_i32)];
_17 = (-72351870_i32) as u32;
_11 = _9.4 << _18;
_3 = [13017962166991909285_usize,2_usize,11183568746001811033_usize,6_usize,3_usize,490468020455224974_usize];
_6 = _23 * _9.3;
Call(_8 = fn12(_9.3, _18, _9.3, _12, _9, _30, _30, _10, _25, _26.2.0, _23, _2, _12), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_3 = [14865120090619974261_usize,2566082486072533130_usize,4_usize,4_usize,3_usize,0_usize];
_26.1 = [_9.0,_9.0,_9.0,_9.0,_9.0];
_29 = [1_usize,16690406775007514723_usize,1_usize,11448576983578003922_usize,1_usize,7_usize];
_9.1 = _9.2;
_28 = _30 * _5;
_30 = 0_usize as isize;
Call(_2 = core::intrinsics::transmute(_26.2.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_14 = _24;
_24 = [_9.0,_9.0,_9.0];
_20.0 = !_26.0;
_6 = _9.3 * _23;
_5 = !_30;
_26.0 = 33643775586600819394461244459648922185_u128 as u32;
_31 = [204531676_i32,(-938270547_i32),(-1253029360_i32),(-1611649676_i32),613308264_i32,(-1517952636_i32),(-1653298711_i32)];
_20.0 = !_17;
_18 = _30;
_5 = _28;
_26.0 = _20.0;
_1 = [16790828787514749288_usize,2_usize,1_usize,12377821728032335300_usize,5850846376242808481_usize,5_usize];
_4 = [_9.4,_11,_9.4,_9.4,_11,_11];
_26.2.0 = _2;
Goto(bb13)
}
bb13 = {
_32 = !_13;
_18 = _10 as isize;
_23 = 126693369571182631104596087586188253079_u128 as f32;
_22 = _15 as i16;
_24 = [_9.0,_9.0,_9.0];
_26.2.1 = [9623879652694172569_u64,8047731511739264491_u64,5170478172370723499_u64,8703103208557137377_u64];
_26.2.0 = _2;
_26.2.1 = [8730230082141338283_u64,17392674664706240914_u64,13549205366925774194_u64,7647133110461987084_u64];
_3 = [1_usize,4_usize,6402190167623455749_usize,6_usize,6157402258458679617_usize,8678092703844060217_usize];
_29 = _3;
Goto(bb14)
}
bb14 = {
_27 = [7760132302936559052_u64,17181506359726326054_u64,15230324812136536681_u64,17456710399790901951_u64,17781201579108199205_u64,9603059428979212033_u64];
_30 = _11 as isize;
_24 = [_9.0,_9.0,_9.0];
_34 = _5 ^ _30;
_14 = [_9.0,_9.0,_9.0];
_4 = [_11,_11,_11,_9.4,_9.4,_9.4];
_14 = [_9.0,_9.0,_9.0];
_11 = _9.4;
_8.fld0 = !25_u8;
_33 = [2400287342956492046_u64,4226664421133693407_u64,9996122683287568857_u64,12075290912620066281_u64];
_8 = Adt46 { fld0: 52_u8 };
_5 = -_30;
_20 = (_26.0, _22);
_37 = _9.0 as isize;
_22 = _20.1;
_8.fld0 = !84_u8;
_15 = _22 as u16;
_18 = _5;
_20.0 = !_26.0;
_9.2 = _9.1;
_26.0 = _20.0 | _20.0;
_20 = (_26.0, _22);
_35 = _3;
_15 = 38435_u16 ^ 47849_u16;
_9.1 = _9.2;
Goto(bb15)
}
bb15 = {
_26.2.1 = [9162724408623289979_u64,7158376484694336941_u64,6465166139213432769_u64,10871124166395146651_u64];
_35 = [3_usize,7_usize,4_usize,16451822392400998837_usize,7_usize,1_usize];
_9.1 = core::ptr::addr_of_mut!(_9.0);
_11 = _9.0 as i64;
_40.fld3 = 328216403488001147786728937226197976482_u128 as u64;
_18 = !_34;
_27 = [_40.fld3,_40.fld3,_40.fld3,_40.fld3,_40.fld3,_40.fld3];
_32 = _13;
_31 = [(-1470432652_i32),(-1593162579_i32),(-2071769681_i32),820317671_i32,(-787068964_i32),(-1894072654_i32),(-2026454368_i32)];
_20.1 = -_22;
_3 = [1307287051507330648_usize,2102656927818892717_usize,15500597637842620053_usize,6_usize,6052631515929302647_usize,12161495079101527555_usize];
_40.fld5 = !_15;
_36 = core::ptr::addr_of_mut!(_40.fld3);
_34 = _37;
_26.2 = (_2, _33);
(*_36) = 4076811337826702245_u64 >> _15;
_20.0 = _26.0 | _17;
_22 = _20.1 & _20.1;
_20.0 = (*_36) as u32;
_26.1 = [_9.0,_9.0,_9.0,_9.0,_9.0];
_8 = Adt46 { fld0: 237_u8 };
_40.fld1 = Move(_8);
_26.2.1 = _33;
_33 = [(*_36),_40.fld3,(*_36),(*_36)];
Call(_3 = fn14(_12, _1, _36, _12, _33, _34, _9.2, _15), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_21 = Adt60::Variant0 { fld0: _26.2,fld1: _7,fld2: _24 };
_20 = (_17, _22);
_25 = [2088351802_i32,(-276366251_i32),1208698401_i32,1016935164_i32,977175376_i32,815144242_i32,2106791579_i32];
_20 = (_17, _22);
_9.0 = Field::<char>(Variant(_21, 0), 1) as i128;
_20 = (_17, _22);
_31 = [(-657574685_i32),2041898340_i32,35911966_i32,748587179_i32,(-1142238837_i32),950301231_i32,1806869751_i32];
_26.1 = [_9.0,_9.0,_9.0,_9.0,_9.0];
_36 = core::ptr::addr_of_mut!((*_36));
_26.2.1 = [_40.fld3,(*_36),(*_36),(*_36)];
_42 = Adt56::Variant1 { fld0: Move(_40.fld1) };
place!(Field::<Adt46>(Variant(_42, 1), 0)).fld0 = 248_u8;
_51 = _13 as i128;
_20.0 = !_17;
_32 = !_13;
_49 = Field::<Adt46>(Variant(_42, 1), 0).fld0 as u128;
_4 = [_9.4,_9.4,_9.4,_9.4,_11,_11];
Goto(bb17)
}
bb17 = {
_17 = !_26.0;
_40.fld1.fld0 = _20.1 as u8;
place!(Field::<[i128; 3]>(Variant(_21, 0), 2)) = [_51,_9.0,_51];
RET = Adt62::Variant1 { fld0: _9.2,fld1: _26.1,fld2: Move(_42),fld3: _20,fld4: _26 };
place!(Field::<Adt46>(Variant(place!(Field::<Adt56>(Variant(RET, 1), 2)), 1), 0)).fld0 = _40.fld1.fld0 << _40.fld5;
_42 = Move(Field::<Adt56>(Variant(RET, 1), 2));
_44 = 1057850820_i32 as isize;
SetDiscriminant(_21, 0);
place!(Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(RET, 1), 4)).1 = _26.1;
_18 = !_5;
place!(Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(RET, 1), 4)).2.1 = [(*_36),(*_36),(*_36),(*_36)];
_24 = [_9.0,_51,_51];
place!(Field::<([u8; 7], [u64; 4])>(Variant(_21, 0), 0)).0 = [_40.fld1.fld0,Field::<Adt46>(Variant(_42, 1), 0).fld0,Field::<Adt46>(Variant(_42, 1), 0).fld0,_40.fld1.fld0,Field::<Adt46>(Variant(_42, 1), 0).fld0,_40.fld1.fld0,Field::<Adt46>(Variant(_42, 1), 0).fld0];
place!(Field::<[i128; 5]>(Variant(RET, 1), 1)) = [_51,_9.0,_9.0,_51,_9.0];
_46 = (-83_i8);
place!(Field::<[i128; 3]>(Variant(_21, 0), 2)) = [_51,_9.0,_9.0];
place!(Field::<([u8; 7], [u64; 4])>(Variant(_21, 0), 0)).0 = [Field::<Adt46>(Variant(_42, 1), 0).fld0,_40.fld1.fld0,Field::<Adt46>(Variant(_42, 1), 0).fld0,Field::<Adt46>(Variant(_42, 1), 0).fld0,_40.fld1.fld0,Field::<Adt46>(Variant(_42, 1), 0).fld0,Field::<Adt46>(Variant(_42, 1), 0).fld0];
_6 = -_23;
_13 = !_32;
_47 = [737760609_i32,(-1488669313_i32),1336144700_i32,1921495350_i32,1240364551_i32,618386647_i32,1020692843_i32];
place!(Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(RET, 1), 4)) = _26;
place!(Field::<[i128; 3]>(Variant(_21, 0), 2)) = [_51,_9.0,_51];
_25 = [2098049548_i32,1155362539_i32,(-393827221_i32),702774786_i32,1697549286_i32,(-1313060699_i32),705960336_i32];
_7 = '\u{f091f}';
_2 = [Field::<Adt46>(Variant(_42, 1), 0).fld0,Field::<Adt46>(Variant(_42, 1), 0).fld0,Field::<Adt46>(Variant(_42, 1), 0).fld0,_40.fld1.fld0,Field::<Adt46>(Variant(_42, 1), 0).fld0,Field::<Adt46>(Variant(_42, 1), 0).fld0,Field::<Adt46>(Variant(_42, 1), 0).fld0];
_55.fld0 = _9.3;
Goto(bb18)
}
bb18 = {
_9.0 = _11 as i128;
_20.0 = (*_36) as u32;
_21 = Adt60::Variant0 { fld0: Field::<(u32, [i128; 5], ([u8; 7], [u64; 4]))>(Variant(RET, 1), 4).2,fld1: _7,fld2: _14 };
_3 = [17856799960837611443_usize,7712641228330047737_usize,4451388754715028878_usize,3_usize,5514484680842978320_usize,1_usize];
place!(Field::<Adt46>(Variant(_42, 1), 0)).fld0 = _40.fld1.fld0 * _40.fld1.fld0;
_3 = _29;
_40.fld6 = [Field::<Adt46>(Variant(_42, 1), 0).fld0,_40.fld1.fld0,Field::<Adt46>(Variant(_42, 1), 0).fld0,Field::<Adt46>(Variant(_42, 1), 0).fld0,Field::<Adt46>(Variant(_42, 1), 0).fld0,Field::<Adt46>(Variant(_42, 1), 0).fld0,Field::<Adt46>(Variant(_42, 1), 0).fld0];
place!(Field::<[i128; 5]>(Variant(RET, 1), 1)) = [_51,_51,_51,_9.0,_9.0];
_18 = Field::<(u32, i16)>(Variant(RET, 1), 3).0 as isize;
_10 = (-1998143193_i32) as f64;
place!(Field::<Adt56>(Variant(RET, 1), 2)) = Move(_42);
_22 = _7 as i16;
_53 = -_10;
Goto(bb19)
}
bb19 = {
Call(_57 = dump_var(11_usize, 13_usize, Move(_13), 14_usize, Move(_14), 15_usize, Move(_15), 30_usize, Move(_30)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_57 = dump_var(11_usize, 1_usize, Move(_1), 24_usize, Move(_24), 17_usize, Move(_17), 49_usize, Move(_49)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_57 = dump_var(11_usize, 37_usize, Move(_37), 20_usize, Move(_20), 4_usize, Move(_4), 18_usize, Move(_18)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_57 = dump_var(11_usize, 3_usize, Move(_3), 32_usize, Move(_32), 29_usize, Move(_29), 51_usize, Move(_51)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: f32,mut _2: isize,mut _3: f32,mut _4: [i32; 7],mut _5: (i128, *mut i128, *mut i128, f32, i64),mut _6: isize,mut _7: isize,mut _8: f64,mut _9: [i32; 7],mut _10: [u8; 7],mut _11: f32,mut _12: [u8; 7],mut _13: [i32; 7]) -> Adt46 {
mir! {
type RET = Adt46;
let _14: [usize; 6];
let _15: [i64; 6];
let _16: u8;
let _17: bool;
let _18: *const i128;
let _19: f32;
let _20: Adt46;
let _21: i64;
let _22: Adt51;
let _23: bool;
let _24: i16;
let _25: Adt52;
let _26: u8;
let _27: isize;
let _28: ([u64; 4], u32, u8);
let _29: Adt62;
let _30: Adt46;
let _31: u8;
let _32: Adt54;
let _33: i32;
let _34: *const (&'static u16, f64);
let _35: [i32; 7];
let _36: char;
let _37: [i64; 6];
let _38: i128;
let _39: ();
let _40: ();
{
RET.fld0 = 125_u8;
_8 = _1 as f64;
_8 = 3_usize as f64;
_6 = !_2;
_7 = '\u{72e9f}' as isize;
_1 = _3 + _5.3;
RET.fld0 = 138_u8 >> _5.4;
_10 = [RET.fld0,RET.fld0,RET.fld0,RET.fld0,RET.fld0,RET.fld0,RET.fld0];
_7 = _6;
_5.4 = -8870841045936467043_i64;
_5.3 = _11;
Goto(bb1)
}
bb1 = {
_10 = [RET.fld0,RET.fld0,RET.fld0,RET.fld0,RET.fld0,RET.fld0,RET.fld0];
_5.1 = core::ptr::addr_of_mut!(_5.0);
_5.3 = _1;
_4 = _13;
_15 = [_5.4,_5.4,_5.4,_5.4,_5.4,_5.4];
Call(_3 = fn13(RET.fld0, _15, _12, _8, _12, _6, _9, _13, _10, _5.3, _13, _7, _5.3, _5.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET.fld0 = !155_u8;
_7 = false as isize;
_14 = [1515229948278954646_usize,9819998422763758226_usize,16926801121529899416_usize,8583761346651672244_usize,4_usize,7_usize];
_11 = _3 + _5.3;
_1 = -_3;
RET.fld0 = 19082_u16 as u8;
_5.0 = !95248813322684418359156598242968328659_i128;
_4 = [310023752_i32,692625161_i32,578486044_i32,911895511_i32,(-1478065858_i32),1399299639_i32,(-1205385972_i32)];
_19 = _5.3 - _5.3;
RET = Adt46 { fld0: 205_u8 };
_8 = 1041469165_u32 as f64;
_8 = RET.fld0 as f64;
RET = Adt46 { fld0: 65_u8 };
_5.2 = _5.1;
_16 = 12519641471350876318_usize as u8;
RET.fld0 = _16;
_19 = 242974905742703835899034415486296874134_u128 as f32;
_16 = RET.fld0;
_19 = (-127174179_i32) as f32;
_5.2 = core::ptr::addr_of_mut!(_5.0);
_3 = _1 - _11;
_7 = false as isize;
_13 = _9;
_18 = core::ptr::addr_of!(_5.0);
Goto(bb3)
}
bb3 = {
_5.2 = core::ptr::addr_of_mut!((*_18));
_20.fld0 = !_16;
_1 = 18722163283498967072651921725810800737_u128 as f32;
_9 = [(-1039968767_i32),861509979_i32,(-1346179432_i32),576992335_i32,467262679_i32,(-1728823013_i32),1578502373_i32];
_21 = !_5.4;
_9 = [(-592819142_i32),1077106446_i32,(-1931248980_i32),1158404575_i32,(-308041507_i32),378942057_i32,1585449674_i32];
_14 = [3_usize,4_usize,3_usize,3223813120926418945_usize,3330164505116590113_usize,18297392146254231242_usize];
RET = Adt46 { fld0: _16 };
_17 = !true;
_5.2 = core::ptr::addr_of_mut!((*_18));
_9 = [1172194558_i32,(-1087726186_i32),1492217170_i32,1240471624_i32,836589182_i32,(-830709899_i32),321085622_i32];
_13 = _9;
RET = Move(_20);
_19 = _11 + _3;
_14 = [7_usize,7_usize,11561773007808569956_usize,16092506483497215493_usize,2_usize,4_usize];
_21 = 1140699914_i32 as i64;
_11 = _3;
_5.1 = _5.2;
Goto(bb4)
}
bb4 = {
RET.fld0 = 1709080166934986181_u64 as u8;
_19 = _11 - _3;
_5.0 = 16066754710616437473337774395691474007_i128;
_17 = false;
RET.fld0 = _16 << _5.0;
_2 = _6;
_12 = [_16,RET.fld0,_16,RET.fld0,RET.fld0,RET.fld0,RET.fld0];
_18 = core::ptr::addr_of!((*_18));
_5.2 = core::ptr::addr_of_mut!((*_18));
RET = Adt46 { fld0: _16 };
_24 = !10597_i16;
_15 = [_5.4,_5.4,_5.4,_5.4,_5.4,_21];
match (*_18) {
0 => bb2,
1 => bb5,
2 => bb6,
3 => bb7,
16066754710616437473337774395691474007 => bb9,
_ => bb8
}
}
bb5 = {
_5.2 = core::ptr::addr_of_mut!((*_18));
_20.fld0 = !_16;
_1 = 18722163283498967072651921725810800737_u128 as f32;
_9 = [(-1039968767_i32),861509979_i32,(-1346179432_i32),576992335_i32,467262679_i32,(-1728823013_i32),1578502373_i32];
_21 = !_5.4;
_9 = [(-592819142_i32),1077106446_i32,(-1931248980_i32),1158404575_i32,(-308041507_i32),378942057_i32,1585449674_i32];
_14 = [3_usize,4_usize,3_usize,3223813120926418945_usize,3330164505116590113_usize,18297392146254231242_usize];
RET = Adt46 { fld0: _16 };
_17 = !true;
_5.2 = core::ptr::addr_of_mut!((*_18));
_9 = [1172194558_i32,(-1087726186_i32),1492217170_i32,1240471624_i32,836589182_i32,(-830709899_i32),321085622_i32];
_13 = _9;
RET = Move(_20);
_19 = _11 + _3;
_14 = [7_usize,7_usize,11561773007808569956_usize,16092506483497215493_usize,2_usize,4_usize];
_21 = 1140699914_i32 as i64;
_11 = _3;
_5.1 = _5.2;
Goto(bb4)
}
bb6 = {
RET.fld0 = !155_u8;
_7 = false as isize;
_14 = [1515229948278954646_usize,9819998422763758226_usize,16926801121529899416_usize,8583761346651672244_usize,4_usize,7_usize];
_11 = _3 + _5.3;
_1 = -_3;
RET.fld0 = 19082_u16 as u8;
_5.0 = !95248813322684418359156598242968328659_i128;
_4 = [310023752_i32,692625161_i32,578486044_i32,911895511_i32,(-1478065858_i32),1399299639_i32,(-1205385972_i32)];
_19 = _5.3 - _5.3;
RET = Adt46 { fld0: 205_u8 };
_8 = 1041469165_u32 as f64;
_8 = RET.fld0 as f64;
RET = Adt46 { fld0: 65_u8 };
_5.2 = _5.1;
_16 = 12519641471350876318_usize as u8;
RET.fld0 = _16;
_19 = 242974905742703835899034415486296874134_u128 as f32;
_16 = RET.fld0;
_19 = (-127174179_i32) as f32;
_5.2 = core::ptr::addr_of_mut!(_5.0);
_3 = _1 - _11;
_7 = false as isize;
_13 = _9;
_18 = core::ptr::addr_of!(_5.0);
Goto(bb3)
}
bb7 = {
_10 = [RET.fld0,RET.fld0,RET.fld0,RET.fld0,RET.fld0,RET.fld0,RET.fld0];
_5.1 = core::ptr::addr_of_mut!(_5.0);
_5.3 = _1;
_4 = _13;
_15 = [_5.4,_5.4,_5.4,_5.4,_5.4,_5.4];
Call(_3 = fn13(RET.fld0, _15, _12, _8, _12, _6, _9, _13, _10, _5.3, _13, _7, _5.3, _5.0), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
Return()
}
bb9 = {
_5.0 = _16 as i128;
_5.4 = _21 - _21;
_19 = _11;
_8 = 2492473141089505789_usize as f64;
_20.fld0 = !RET.fld0;
_23 = !_17;
_9 = [1938532798_i32,774527878_i32,(-1117252950_i32),(-952888779_i32),1844684378_i32,1752600079_i32,542981229_i32];
_10 = _12;
_10 = [_20.fld0,_20.fld0,_16,_16,_16,RET.fld0,RET.fld0];
_6 = !_2;
_20 = Adt46 { fld0: _16 };
RET.fld0 = _20.fld0 & _16;
Goto(bb10)
}
bb10 = {
_17 = !_23;
_26 = _16;
_6 = 16576_u16 as isize;
_2 = _7 - _6;
RET = Adt46 { fld0: _20.fld0 };
(*_18) = 96029409012272864376377418873207838254_i128;
(*_18) = !7492944393513114691734745916204765291_i128;
_13 = _4;
_1 = 21686722304286262684538750414099716642_u128 as f32;
_5.1 = _5.2;
_27 = -_7;
Call(_10 = core::intrinsics::transmute(_12), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_6 = _2 | _27;
RET.fld0 = _16 * _16;
_5.4 = !_21;
_14 = [0_usize,2839668117390886153_usize,13587969784558201311_usize,12976473684382898110_usize,0_usize,16244905471184378094_usize];
_5.3 = 13650436522641599148_u64 as f32;
_13 = [(-1710889267_i32),(-827177841_i32),(-1556277124_i32),(-1392854075_i32),36301167_i32,806660138_i32,1071527391_i32];
_19 = -_3;
_1 = _3;
_28.1 = 3451323666_u32;
_18 = core::ptr::addr_of!((*_18));
_28.0 = [200556066633605534_u64,4371075854071826122_u64,6059636660221769818_u64,2362871414909349490_u64];
_13 = _9;
_24 = -(-7542_i16);
Goto(bb12)
}
bb12 = {
_30 = Adt46 { fld0: _20.fld0 };
_6 = _27;
_13 = [(-674341522_i32),861445427_i32,(-156698575_i32),1934222135_i32,(-951975679_i32),1901173049_i32,(-1480577959_i32)];
_21 = _5.4;
_11 = _19;
_27 = _21 as isize;
_28.0 = [14005459497731635214_u64,5679267013585289818_u64,6040188547458497371_u64,12227748478427809944_u64];
_18 = core::ptr::addr_of!(_5.0);
(*_18) = (-88537172261698956979850185688460144055_i128);
_6 = '\u{15181}' as isize;
_28.2 = RET.fld0 ^ _30.fld0;
_20.fld0 = _28.2 >> _21;
_30.fld0 = _20.fld0;
_2 = -_27;
_30.fld0 = !_28.2;
_1 = -_11;
_20.fld0 = _17 as u8;
_2 = -_27;
_30 = Adt46 { fld0: _20.fld0 };
match (*_18) {
0 => bb1,
1 => bb4,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
251745194659239506483524421743308067401 => bb18,
_ => bb17
}
}
bb13 = {
_6 = _2 | _27;
RET.fld0 = _16 * _16;
_5.4 = !_21;
_14 = [0_usize,2839668117390886153_usize,13587969784558201311_usize,12976473684382898110_usize,0_usize,16244905471184378094_usize];
_5.3 = 13650436522641599148_u64 as f32;
_13 = [(-1710889267_i32),(-827177841_i32),(-1556277124_i32),(-1392854075_i32),36301167_i32,806660138_i32,1071527391_i32];
_19 = -_3;
_1 = _3;
_28.1 = 3451323666_u32;
_18 = core::ptr::addr_of!((*_18));
_28.0 = [200556066633605534_u64,4371075854071826122_u64,6059636660221769818_u64,2362871414909349490_u64];
_13 = _9;
_24 = -(-7542_i16);
Goto(bb12)
}
bb14 = {
_10 = [RET.fld0,RET.fld0,RET.fld0,RET.fld0,RET.fld0,RET.fld0,RET.fld0];
_5.1 = core::ptr::addr_of_mut!(_5.0);
_5.3 = _1;
_4 = _13;
_15 = [_5.4,_5.4,_5.4,_5.4,_5.4,_5.4];
Call(_3 = fn13(RET.fld0, _15, _12, _8, _12, _6, _9, _13, _10, _5.3, _13, _7, _5.3, _5.0), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
RET.fld0 = !155_u8;
_7 = false as isize;
_14 = [1515229948278954646_usize,9819998422763758226_usize,16926801121529899416_usize,8583761346651672244_usize,4_usize,7_usize];
_11 = _3 + _5.3;
_1 = -_3;
RET.fld0 = 19082_u16 as u8;
_5.0 = !95248813322684418359156598242968328659_i128;
_4 = [310023752_i32,692625161_i32,578486044_i32,911895511_i32,(-1478065858_i32),1399299639_i32,(-1205385972_i32)];
_19 = _5.3 - _5.3;
RET = Adt46 { fld0: 205_u8 };
_8 = 1041469165_u32 as f64;
_8 = RET.fld0 as f64;
RET = Adt46 { fld0: 65_u8 };
_5.2 = _5.1;
_16 = 12519641471350876318_usize as u8;
RET.fld0 = _16;
_19 = 242974905742703835899034415486296874134_u128 as f32;
_16 = RET.fld0;
_19 = (-127174179_i32) as f32;
_5.2 = core::ptr::addr_of_mut!(_5.0);
_3 = _1 - _11;
_7 = false as isize;
_13 = _9;
_18 = core::ptr::addr_of!(_5.0);
Goto(bb3)
}
bb16 = {
Return()
}
bb17 = {
_5.2 = core::ptr::addr_of_mut!((*_18));
_20.fld0 = !_16;
_1 = 18722163283498967072651921725810800737_u128 as f32;
_9 = [(-1039968767_i32),861509979_i32,(-1346179432_i32),576992335_i32,467262679_i32,(-1728823013_i32),1578502373_i32];
_21 = !_5.4;
_9 = [(-592819142_i32),1077106446_i32,(-1931248980_i32),1158404575_i32,(-308041507_i32),378942057_i32,1585449674_i32];
_14 = [3_usize,4_usize,3_usize,3223813120926418945_usize,3330164505116590113_usize,18297392146254231242_usize];
RET = Adt46 { fld0: _16 };
_17 = !true;
_5.2 = core::ptr::addr_of_mut!((*_18));
_9 = [1172194558_i32,(-1087726186_i32),1492217170_i32,1240471624_i32,836589182_i32,(-830709899_i32),321085622_i32];
_13 = _9;
RET = Move(_20);
_19 = _11 + _3;
_14 = [7_usize,7_usize,11561773007808569956_usize,16092506483497215493_usize,2_usize,4_usize];
_21 = 1140699914_i32 as i64;
_11 = _3;
_5.1 = _5.2;
Goto(bb4)
}
bb18 = {
RET = Move(_20);
_28.1 = 3811905498_u32 + 3056091094_u32;
_20.fld0 = _16 + _28.2;
_23 = !_17;
_28.0 = [10851733543471247979_u64,685047814858728620_u64,14924237750396267737_u64,8368492957113863355_u64];
_15 = [_21,_5.4,_5.4,_21,_5.4,_5.4];
_31 = 2136160891_i32 as u8;
_17 = _11 <= _11;
_5.1 = core::ptr::addr_of_mut!(_5.0);
_27 = -_6;
_16 = !_28.2;
_35 = _9;
_26 = (-1503852093_i32) as u8;
_14 = [1490586272573660176_usize,17454070044695516764_usize,3779505449582048473_usize,3_usize,7_usize,13953423819628727010_usize];
_26 = _28.2 << _24;
_32 = Adt54::Variant2 { fld0: _24,fld1: '\u{7c10b}' };
_33 = !390329595_i32;
RET.fld0 = !_16;
_31 = !_20.fld0;
(*_18) = 78261616706649236394669640604327446459_i128;
_6 = _27;
_4 = [_33,_33,_33,_33,_33,_33,_33];
_28.1 = !1190273058_u32;
_5.4 = -_21;
Goto(bb19)
}
bb19 = {
Call(_39 = dump_var(12_usize, 28_usize, Move(_28), 21_usize, Move(_21), 24_usize, Move(_24), 13_usize, Move(_13)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_39 = dump_var(12_usize, 23_usize, Move(_23), 4_usize, Move(_4), 33_usize, Move(_33), 14_usize, Move(_14)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_39 = dump_var(12_usize, 12_usize, Move(_12), 31_usize, Move(_31), 40_usize, _40, 40_usize, _40), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: u8,mut _2: [i64; 6],mut _3: [u8; 7],mut _4: f64,mut _5: [u8; 7],mut _6: isize,mut _7: [i32; 7],mut _8: [i32; 7],mut _9: [u8; 7],mut _10: f32,mut _11: [i32; 7],mut _12: isize,mut _13: f32,mut _14: i128) -> f32 {
mir! {
type RET = f32;
let _15: (u32, i16);
let _16: isize;
let _17: i64;
let _18: Adt55;
let _19: bool;
let _20: Adt56;
let _21: (u32, &'static u16, &'static u16);
let _22: (u32, i16);
let _23: [usize; 6];
let _24: isize;
let _25: [u16; 1];
let _26: (u32, i16);
let _27: i128;
let _28: [i64; 6];
let _29: [i128; 3];
let _30: ();
let _31: ();
{
_13 = -_10;
_11 = _8;
_4 = _13 as f64;
RET = 1218206582_u32 as f32;
_6 = -_12;
RET = _13;
RET = 87_i8 as f32;
_15 = (940636098_u32, (-30966_i16));
_7 = [(-362393850_i32),(-57045289_i32),2052238342_i32,(-1393585068_i32),(-630182643_i32),1070853287_i32,307419572_i32];
_1 = 212_u8 << _15.0;
_12 = _6;
_13 = _10 + _10;
_8 = _7;
_2 = [(-8890329930694400043_i64),(-8332124935273016917_i64),579301042494451165_i64,7968432449795242175_i64,4079086621436768513_i64,7767416139334485560_i64];
_6 = -_12;
_10 = _12 as f32;
_2 = [(-6030395839266396779_i64),1511655918287444118_i64,(-1543757030522312172_i64),(-5025738031155939508_i64),5081452921890385567_i64,3839624975497456516_i64];
_10 = RET;
_4 = _13 as f64;
_12 = _6 + _6;
_7 = [(-2014078555_i32),(-1154410234_i32),1005044267_i32,(-1271864123_i32),(-1489799188_i32),1558777841_i32,2078044415_i32];
_12 = _6;
_17 = -7906646537628953087_i64;
_15.0 = 3553286739_u32 | 151989869_u32;
_16 = _15.1 as isize;
_1 = 143_u8;
match _15.1 {
340282366920938463463374607431768180490 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_17 = -582027592523505803_i64;
_17 = -(-3831525447516250367_i64);
_4 = _13 as f64;
_4 = (-128_i8) as f64;
_12 = _16;
_1 = 199_u8 << _15.0;
RET = _13 - _10;
RET = -_13;
_18 = Adt55::Variant2 { fld0: 7452438152621573008_u64 };
Call(_8 = core::intrinsics::transmute(_11), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = [1710989629_i32,(-740287797_i32),2111541184_i32,597702076_i32,1193852472_i32,2007066723_i32,(-1306113792_i32)];
_17 = 75492535853607539828590223204936585597_u128 as i64;
_4 = _15.1 as f64;
_7 = _11;
place!(Field::<u64>(Variant(_18, 2), 0)) = !7776970928989228029_u64;
SetDiscriminant(_18, 2);
_3 = [_1,_1,_1,_1,_1,_1,_1];
_11 = [(-2052308357_i32),593306433_i32,289473326_i32,43426102_i32,1749892368_i32,346304470_i32,(-1165877550_i32)];
_18 = Adt55::Variant2 { fld0: 10055982363034386784_u64 };
_13 = RET;
_15.0 = !2491394990_u32;
_3 = [_1,_1,_1,_1,_1,_1,_1];
_13 = RET;
_10 = RET + _13;
_4 = _6 as f64;
_12 = !_16;
_13 = -_10;
_6 = _12;
_1 = true as u8;
_17 = 5900178139924127395_i64 ^ 2882499160489108757_i64;
Goto(bb4)
}
bb4 = {
_19 = _13 >= RET;
_12 = !_16;
_3 = [_1,_1,_1,_1,_1,_1,_1];
_21.0 = !_15.0;
_22.0 = _15.0 & _21.0;
_11 = _7;
_6 = _12;
_2 = [_17,_17,_17,_17,_17,_17];
_22.1 = _15.1 << _22.0;
_4 = _22.0 as f64;
place!(Field::<u64>(Variant(_18, 2), 0)) = 14776505228681436602_u64 >> _22.0;
_10 = -_13;
Goto(bb5)
}
bb5 = {
_18 = Adt55::Variant2 { fld0: 4162642690946761958_u64 };
place!(Field::<u64>(Variant(_18, 2), 0)) = 226184863031405746_u64;
_26 = _22;
Call(_22.1 = core::intrinsics::bswap(_15.1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_8 = [410413531_i32,(-1897930854_i32),(-23101276_i32),(-219265853_i32),(-1083540513_i32),(-1917066393_i32),876946177_i32];
_26.1 = _15.1 * _22.1;
_24 = _6;
_8 = [(-2080877232_i32),1845635356_i32,2087531287_i32,1047259826_i32,321460052_i32,481946813_i32,(-299007368_i32)];
RET = -_10;
_5 = [_1,_1,_1,_1,_1,_1,_1];
Goto(bb7)
}
bb7 = {
Call(_30 = dump_var(13_usize, 3_usize, Move(_3), 14_usize, Move(_14), 12_usize, Move(_12), 16_usize, Move(_16)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_30 = dump_var(13_usize, 24_usize, Move(_24), 17_usize, Move(_17), 5_usize, Move(_5), 22_usize, Move(_22)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_30 = dump_var(13_usize, 11_usize, Move(_11), 31_usize, _31, 31_usize, _31, 31_usize, _31), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: [i32; 7],mut _2: [usize; 6],mut _3: *mut u64,mut _4: [i32; 7],mut _5: [u64; 4],mut _6: isize,mut _7: *mut i128,mut _8: u16) -> [usize; 6] {
mir! {
type RET = [usize; 6];
let _9: Adt59;
let _10: char;
let _11: Adt57;
let _12: f32;
let _13: f32;
let _14: [i32; 7];
let _15: (i128, *mut i128, *mut i128, f32, i64);
let _16: i64;
let _17: u16;
let _18: [u16; 1];
let _19: Adt62;
let _20: [u16; 1];
let _21: isize;
let _22: isize;
let _23: isize;
let _24: [u16; 1];
let _25: isize;
let _26: [i128; 5];
let _27: isize;
let _28: Adt46;
let _29: (u32, i16);
let _30: isize;
let _31: isize;
let _32: ();
let _33: ();
{
_6 = (-9223372036854775808_isize);
RET = _2;
_3 = core::ptr::addr_of_mut!((*_3));
RET = [10067964327409843683_usize,5_usize,7857873721512948531_usize,4375417419803052475_usize,11849729506788836525_usize,2114716639081846594_usize];
RET = [3964764701616311664_usize,15336607734670513136_usize,5734528731653104343_usize,16659863627911226273_usize,8507501784496414951_usize,9452072397860234681_usize];
(*_7) = 47894418028450944443626259490133993178_i128 >> (*_3);
(*_3) = 8125908025803795016_u64;
(*_3) = !7925628311361221933_u64;
_3 = core::ptr::addr_of_mut!((*_3));
_4 = _1;
_8 = 54912_u16 ^ 31316_u16;
(*_7) = (-151193503395220635879012632554873368670_i128);
_8 = '\u{4066e}' as u16;
_9.fld6 = [179_u8,107_u8,229_u8,124_u8,50_u8,1_u8,36_u8];
_9.fld5 = !_8;
_7 = core::ptr::addr_of_mut!((*_7));
_9.fld1.fld0 = (-20572_i16) as u8;
_7 = core::ptr::addr_of_mut!((*_7));
_3 = core::ptr::addr_of_mut!((*_3));
_9.fld1.fld0 = 153_u8 & 15_u8;
RET = [16971189357471060469_usize,7686312387886888429_usize,3554430888940615611_usize,6154037374721980205_usize,4_usize,2_usize];
_4 = [(-1306295079_i32),1633380449_i32,637864446_i32,873493329_i32,(-1365144671_i32),(-1740894297_i32),(-1846402110_i32)];
Goto(bb1)
}
bb1 = {
_9.fld1.fld0 = 211_u8;
_9.fld6 = [_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0];
_3 = core::ptr::addr_of_mut!((*_3));
match _9.fld1.fld0 {
0 => bb2,
1 => bb3,
2 => bb4,
211 => bb6,
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
_4 = [471829189_i32,278796064_i32,(-1414035960_i32),911153068_i32,1155435165_i32,582995578_i32,1781015551_i32];
_2 = [6_usize,15240488405977832495_usize,5_usize,3838818900692140900_usize,2_usize,4742658195444641262_usize];
_7 = core::ptr::addr_of_mut!((*_7));
_3 = core::ptr::addr_of_mut!(_9.fld3);
_9.fld1 = Adt46 { fld0: 73_u8 };
_5 = [15196823893886151072_u64,1628427149810723605_u64,12980677291730416393_u64,13752253910194881814_u64];
_1 = [(-993400084_i32),830303384_i32,(-175880902_i32),(-1238366028_i32),(-1174099599_i32),487421_i32,(-946648780_i32)];
_3 = core::ptr::addr_of_mut!(_9.fld3);
(*_3) = 792903082342218461_u64 & 5248966959419968332_u64;
_1 = _4;
_7 = core::ptr::addr_of_mut!((*_7));
(*_7) = 9633679986112247958_usize as i128;
_9.fld1.fld0 = 162694330229774501784871358694343486311_u128 as u8;
_15.4 = -8974694922151557509_i64;
_15.1 = _7;
_15.3 = _6 as f32;
match _6 {
0 => bb7,
1 => bb8,
340282366920938463454151235394913435648 => bb10,
_ => bb9
}
}
bb7 = {
Return()
}
bb8 = {
_9.fld1.fld0 = 211_u8;
_9.fld6 = [_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0];
_3 = core::ptr::addr_of_mut!((*_3));
match _9.fld1.fld0 {
0 => bb2,
1 => bb3,
2 => bb4,
211 => bb6,
_ => bb5
}
}
bb9 = {
Return()
}
bb10 = {
Goto(bb11)
}
bb11 = {
_21 = _6 << (*_3);
_14 = [(-1419039893_i32),(-68091611_i32),786239075_i32,(-1074711394_i32),554160417_i32,(-472938290_i32),(-473769734_i32)];
_12 = -_15.3;
_16 = !_15.4;
_13 = _9.fld1.fld0 as f32;
_15.3 = _16 as f32;
_15.2 = core::ptr::addr_of_mut!(_15.0);
_20 = [_9.fld5];
_22 = (*_3) as isize;
_9.fld1 = Adt46 { fld0: 218_u8 };
_15.1 = core::ptr::addr_of_mut!((*_7));
_17 = !_8;
_3 = core::ptr::addr_of_mut!(_9.fld3);
match _6 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
340282366920938463454151235394913435648 => bb17,
_ => bb16
}
}
bb12 = {
Goto(bb11)
}
bb13 = {
_9.fld1.fld0 = 211_u8;
_9.fld6 = [_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0];
_3 = core::ptr::addr_of_mut!((*_3));
match _9.fld1.fld0 {
0 => bb2,
1 => bb3,
2 => bb4,
211 => bb6,
_ => bb5
}
}
bb14 = {
_9.fld1.fld0 = 211_u8;
_9.fld6 = [_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0,_9.fld1.fld0];
_3 = core::ptr::addr_of_mut!((*_3));
match _9.fld1.fld0 {
0 => bb2,
1 => bb3,
2 => bb4,
211 => bb6,
_ => bb5
}
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
RET = _2;
_9.fld5 = !_8;
_17 = _8;
_15 = ((*_7), _7, _7, _13, _16);
_6 = _21 + _22;
_15.1 = core::ptr::addr_of_mut!((*_7));
_15.0 = (*_7);
RET = [7_usize,18291155578692251468_usize,2322825811405610127_usize,17335846532022213758_usize,5245557966500048978_usize,7_usize];
_7 = _15.1;
_23 = _6 * _21;
_15.2 = _7;
_28 = Adt46 { fld0: _9.fld1.fld0 };
_15 = ((*_7), _7, _7, _12, _16);
_28 = Adt46 { fld0: _9.fld1.fld0 };
_9.fld1.fld0 = _28.fld0;
_18 = [_9.fld5];
_8 = _9.fld5 - _17;
_21 = _23 ^ _6;
_29.1 = -26039_i16;
_25 = _6 | _23;
_26 = [_15.0,(*_7),(*_7),(*_7),(*_7)];
(*_3) = 16003709362241262895_u64 >> _6;
Goto(bb18)
}
bb18 = {
Call(_32 = dump_var(14_usize, 4_usize, Move(_4), 1_usize, Move(_1), 25_usize, Move(_25), 14_usize, Move(_14)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_32 = dump_var(14_usize, 21_usize, Move(_21), 8_usize, Move(_8), 22_usize, Move(_22), 6_usize, Move(_6)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15(mut _1: [usize; 6],mut _2: [usize; 6],mut _3: [i32; 7]) -> [u64; 6] {
mir! {
type RET = [u64; 6];
let _4: u32;
let _5: [u8; 7];
let _6: char;
let _7: i32;
let _8: (u32, i16);
let _9: (u32, i16);
let _10: Adt46;
let _11: Adt50;
let _12: (u32, [i128; 5], ([u8; 7], [u64; 4]));
let _13: [usize; 6];
let _14: *mut u32;
let _15: Adt46;
let _16: ();
let _17: ();
{
_1 = [0_usize,4364721274702331783_usize,6546752761473113940_usize,0_usize,13512305367309983728_usize,1_usize];
_2 = _1;
_4 = 583219382_u32;
_3 = [(-278996808_i32),1441468921_i32,1467630970_i32,1803382214_i32,1662076340_i32,227354115_i32,1714160426_i32];
RET = [13386522657216137671_u64,12963770016273589820_u64,9148806748170533660_u64,14527976957490925232_u64,7807796445378410854_u64,16920642533314428357_u64];
_2 = [1520457696351283935_usize,6651103154736240346_usize,1_usize,8080386100469490250_usize,7_usize,7_usize];
RET = [14281315395909967294_u64,5811177262046151031_u64,10154992667174307016_u64,3862358717733915207_u64,3278064744417924892_u64,15145718497331064888_u64];
_2 = _1;
_4 = 3021379400_u32;
_1 = [2_usize,3043852773186797839_usize,6_usize,9503322953976248432_usize,16748352208967933403_usize,14074564448648667232_usize];
_3 = [(-295610540_i32),(-1203064106_i32),388127261_i32,780530938_i32,461236447_i32,662848437_i32,(-649900800_i32)];
_3 = [(-444171431_i32),(-182107823_i32),1564082872_i32,(-1726447434_i32),658605253_i32,(-1934280693_i32),(-2074077017_i32)];
_5 = [161_u8,121_u8,202_u8,76_u8,7_u8,186_u8,24_u8];
_3 = [2104377337_i32,1331784958_i32,368602134_i32,1427032110_i32,493699027_i32,274947898_i32,(-584672406_i32)];
RET = [8933870770052580879_u64,11292931084040451983_u64,12377572751649983739_u64,908547861148500396_u64,9559804544511499749_u64,1010520256618958491_u64];
_2 = [14634893699847885130_usize,0_usize,4_usize,13126546089051751528_usize,0_usize,14762075450467352852_usize];
_4 = 1517076989_u32;
_5 = [253_u8,171_u8,18_u8,194_u8,89_u8,8_u8,194_u8];
_1 = _2;
_2 = [16653711231526326990_usize,17123150088362538084_usize,13216038219015389328_usize,1536002103086509149_usize,5_usize,2524442218425703975_usize];
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
1517076989 => bb7,
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
_5 = [245_u8,17_u8,132_u8,62_u8,93_u8,73_u8,210_u8];
_5 = [216_u8,33_u8,37_u8,37_u8,71_u8,54_u8,199_u8];
_5 = [222_u8,60_u8,217_u8,101_u8,222_u8,106_u8,142_u8];
_3 = [(-667479542_i32),(-2027928916_i32),1168412597_i32,944727765_i32,(-587418020_i32),623449122_i32,665014678_i32];
_5 = [119_u8,169_u8,229_u8,210_u8,39_u8,163_u8,51_u8];
RET = [15248840599888490348_u64,9474722219787241292_u64,13108920040657934466_u64,11822901234453011314_u64,6833902041978473385_u64,10411492745055699942_u64];
_2 = [16393202319249764476_usize,1_usize,6098162808409637590_usize,11255388872848241039_usize,16900875045637498049_usize,10838196628158712310_usize];
_6 = '\u{80b76}';
_2 = [6_usize,188995333446401969_usize,6891993314415159758_usize,5_usize,13259930553508625337_usize,7785430386265055774_usize];
_2 = [678623845660255798_usize,15640234670581914108_usize,5_usize,6611985712187058636_usize,1_usize,5_usize];
_4 = 1821592502_u32;
_6 = '\u{4bdd0}';
match _4 {
0 => bb6,
1 => bb8,
2 => bb9,
1821592502 => bb11,
_ => bb10
}
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
_1 = _2;
_8.1 = 7780830416784388450_u64 as i16;
RET = [871522131032300622_u64,6311653657086972234_u64,9125012160121360028_u64,7610122029542780503_u64,6220004812561279663_u64,15124642982821363493_u64];
_7 = !(-995421793_i32);
RET = [5748320111355674682_u64,10843899055776461560_u64,3139995189986472671_u64,13080235488022295307_u64,13435222247974110116_u64,5869663161846145904_u64];
RET = [9635359785500097067_u64,7841583510406253039_u64,8966626288751118608_u64,14357880290892704533_u64,18074454143805084730_u64,2336432303656851982_u64];
_6 = '\u{104e89}';
_2 = _1;
_7 = 76188528_i32;
_1 = [7954276002149525167_usize,5780741723446151475_usize,6_usize,1500279305351000808_usize,381562118618480806_usize,308352253310137145_usize];
_9 = (_4, _8.1);
_2 = [0_usize,4_usize,3_usize,4_usize,6_usize,3_usize];
_4 = _9.0 << _9.1;
_7 = !(-1504560436_i32);
RET = [5649144807095118322_u64,15286886174697095450_u64,804009319343130523_u64,765342095883317106_u64,2609925601426515092_u64,7040098121208258354_u64];
_10 = Adt46 { fld0: 195_u8 };
_9.0 = _4;
_12.0 = _9.0 >> _4;
_9 = (_4, _8.1);
_12.1 = [(-26126400421193235917029812543816859964_i128),(-131464149365776150043427013942254726146_i128),113101551224155901590616215532176358067_i128,114289046706652577203849570750365402296_i128,(-95368853242067105214776981633965295810_i128)];
match _10.fld0 {
0 => bb10,
1 => bb12,
195 => bb14,
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
Goto(bb15)
}
bb15 = {
Call(_16 = dump_var(15_usize, 1_usize, Move(_1), 9_usize, Move(_9), 4_usize, Move(_4), 2_usize, Move(_2)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: u8,mut _2: u8,mut _3: [i64; 6],mut _4: [i32; 7],mut _5: [usize; 6],mut _6: Adt46,mut _7: u8,mut _8: [usize; 6],mut _9: [usize; 6],mut _10: i32,mut _11: [usize; 6],mut _12: bool,mut _13: u8,mut _14: [i64; 6],mut _15: u64) -> (i128, *mut i128, *mut i128, f32, i64) {
mir! {
type RET = (i128, *mut i128, *mut i128, f32, i64);
let _16: i8;
let _17: char;
let _18: ([u64; 4], u32, u8);
let _19: bool;
let _20: f64;
let _21: (&'static u16, f64);
let _22: ();
let _23: ();
{
_16 = 95_i8;
RET.1 = core::ptr::addr_of_mut!(RET.0);
RET.1 = core::ptr::addr_of_mut!(RET.0);
_5 = [17500499790402192261_usize,14545359416652279006_usize,7_usize,9176275394182178775_usize,7_usize,453928764717376304_usize];
_18.1 = !1516054599_u32;
RET.2 = core::ptr::addr_of_mut!(RET.0);
RET.3 = 7_usize as f32;
_14 = [7733020957441575422_i64,(-6694530899833764221_i64),4182712225677449846_i64,(-3946229554691279519_i64),(-2778124531161643172_i64),4535255801070509011_i64];
_18.2 = _7;
_18.2 = (-69809323757426724156555096806169242431_i128) as u8;
_18.0 = [_15,_15,_15,_15];
RET.4 = '\u{4147d}' as i64;
RET.0 = _1 as i128;
_7 = _6.fld0 >> _1;
Goto(bb1)
}
bb1 = {
Call(_22 = dump_var(16_usize, 14_usize, Move(_14), 2_usize, Move(_2), 1_usize, Move(_1), 9_usize, Move(_9)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_22 = dump_var(16_usize, 16_usize, Move(_16), 11_usize, Move(_11), 15_usize, Move(_15), 12_usize, Move(_12)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: *mut i128,mut _2: i64,mut _3: (i128, *mut i128, *mut i128, f32, i64),mut _4: bool,mut _5: *mut i128) -> u128 {
mir! {
type RET = u128;
let _6: f32;
let _7: *const (u32, &'static u16, &'static u16);
let _8: usize;
let _9: bool;
let _10: isize;
let _11: isize;
let _12: ();
let _13: ();
{
RET = 58040926141967410893293539514647722424_u128;
_3.0 = (*_1);
_6 = _3.3;
_3 = ((*_5), _1, _1, _6, _2);
_3.0 = (*_5) ^ (*_5);
_5 = core::ptr::addr_of_mut!((*_5));
_2 = 232_u8 as i64;
(*_1) = !_3.0;
_3.0 = (*_5);
(*_5) = _3.0;
(*_1) = _3.0 | _3.0;
(*_5) = -_3.0;
_3.3 = _6;
(*_1) = _3.0 ^ _3.0;
(*_1) = _3.0 - _3.0;
Goto(bb1)
}
bb1 = {
_1 = _3.1;
_5 = core::ptr::addr_of_mut!((*_1));
_4 = !true;
_3.3 = _6;
_5 = core::ptr::addr_of_mut!((*_1));
_3.0 = (*_5) + (*_5);
RET = _3.3 as u128;
(*_5) = !_3.0;
RET = 65107037582753693613905731645127619999_u128 & 46266275016563878765898349667448730251_u128;
_3.3 = _6 * _6;
RET = 335453524515473102390755490211735552577_u128;
_5 = _3.1;
Goto(bb2)
}
bb2 = {
_3.1 = _3.2;
_5 = core::ptr::addr_of_mut!((*_1));
RET = 216020591422661354141647638143335381636_u128 >> (*_5);
(*_5) = _3.0 >> _3.0;
_4 = !true;
RET = _6 as u128;
_1 = _3.2;
_5 = core::ptr::addr_of_mut!((*_1));
_8 = 16171863808830500200_usize * 4_usize;
RET = !86957455470458800780728692879627489595_u128;
_9 = (*_5) > (*_5);
_6 = -_3.3;
_3 = ((*_5), _5, _1, _6, _2);
RET = 191042533682690865346278498235892102561_u128;
_3.4 = (-777819087_i32) as i64;
(*_5) = _3.0;
RET = _3.0 as u128;
_3.3 = -_6;
_3 = ((*_1), _5, _5, _6, _2);
_11 = -9223372036854775807_isize;
_5 = _3.2;
(*_5) = _3.0;
(*_5) = _3.0 + _3.0;
_3.3 = -_6;
_3.4 = _2;
(*_1) = _3.0 ^ _3.0;
_3 = ((*_5), _1, _1, _6, _2);
Goto(bb3)
}
bb3 = {
Call(_12 = dump_var(17_usize, 8_usize, Move(_8), 4_usize, Move(_4), 13_usize, _13, 13_usize, _13), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: [usize; 6],mut _2: [usize; 6],mut _3: [i128; 5],mut _4: i32,mut _5: [i128; 5],mut _6: [usize; 6],mut _7: [usize; 6],mut _8: i32,mut _9: [i128; 5],mut _10: i32,mut _11: [usize; 6],mut _12: u8,mut _13: [usize; 6],mut _14: [usize; 6]) -> i32 {
mir! {
type RET = i32;
let _15: Adt50;
let _16: [i128; 3];
let _17: [i32; 7];
let _18: [usize; 6];
let _19: [u8; 7];
let _20: isize;
let _21: u32;
let _22: ();
let _23: ();
{
_3 = _9;
_7 = [13932966192804331721_usize,4_usize,7_usize,10759988881291202139_usize,1974224991371760224_usize,10111758417312500139_usize];
_7 = [13467122043657205580_usize,10764105159972106476_usize,12705577283807819929_usize,5_usize,3402166766290329954_usize,8665357580092514535_usize];
_2 = _6;
_1 = _7;
RET = _4;
RET = _10 & _10;
_7 = [4832945754602054660_usize,4_usize,3_usize,0_usize,2_usize,2_usize];
_9 = [(-68603347012947565771080403025678352122_i128),(-561120763168773455317942529261261956_i128),40649392208895957340640736684416598691_i128,(-89230313454983714520117299223923596037_i128),50176564241285899616010203304009488281_i128];
_13 = [2875201475235821764_usize,4_usize,7_usize,5_usize,11531517417518337209_usize,8988869705088823654_usize];
_7 = [7_usize,2_usize,15303234664074771897_usize,10446951601519399946_usize,10057770073802550903_usize,4_usize];
_10 = !RET;
_7 = [3665668926852278797_usize,6_usize,4_usize,0_usize,10353595189098892402_usize,4_usize];
Goto(bb1)
}
bb1 = {
_14 = [6_usize,1822381515428354428_usize,15020881825075100888_usize,5896936968011789616_usize,153179922561278839_usize,7_usize];
_9 = [13569268882089705688562084382928303045_i128,(-41533593310636213533156685976980698521_i128),162209231481317442702412263853245069508_i128,(-39514675584615535665125920241783654259_i128),(-124289874563994352783068703232623117864_i128)];
RET = (-91603788758273256156321369117768196046_i128) as i32;
_6 = [4_usize,6498647571162437704_usize,1_usize,2985261083949697950_usize,0_usize,5_usize];
_6 = [1_usize,4_usize,2_usize,6_usize,5_usize,1_usize];
_16 = [(-140637306522829435846857184288406140502_i128),84070645989276312747269185972880090727_i128,(-139007592315863266602325653744535333638_i128)];
RET = _10;
_2 = [9783286687510060628_usize,7320551144374109293_usize,0_usize,6_usize,4540243746046558122_usize,15173344785100069452_usize];
_17 = [_4,_10,RET,_10,_10,_4,RET];
_1 = [1_usize,6_usize,10051295044905339483_usize,6160554425216931970_usize,4_usize,14590343085651990619_usize];
_3 = _9;
RET = _8 | _10;
_17 = [RET,RET,_4,RET,RET,RET,RET];
RET = !_8;
RET = _10 | _4;
_3 = [(-69452891421129477280965807385901487252_i128),7223889494538433222209164689022212019_i128,(-147413937858031894146138771408160195533_i128),48675275175358147273591103902504606311_i128,(-150395201592058396345816725437838199631_i128)];
_7 = [17833338366100889418_usize,9105119395725835489_usize,5_usize,11888146967302335624_usize,4_usize,2_usize];
Goto(bb2)
}
bb2 = {
Call(_22 = dump_var(18_usize, 16_usize, Move(_16), 13_usize, Move(_13), 7_usize, Move(_7), 1_usize, Move(_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_22 = dump_var(18_usize, 12_usize, Move(_12), 10_usize, Move(_10), 3_usize, Move(_3), 4_usize, Move(_4)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn19(mut _1: i16,mut _2: [i128; 3],mut _3: u32) -> isize {
mir! {
type RET = isize;
let _4: f32;
let _5: *const char;
let _6: [usize; 6];
let _7: f64;
let _8: isize;
let _9: [u8; 7];
let _10: char;
let _11: ([u64; 4], u32, u8);
let _12: [i128; 3];
let _13: [i128; 3];
let _14: f32;
let _15: u128;
let _16: i8;
let _17: isize;
let _18: u64;
let _19: [usize; 6];
let _20: u16;
let _21: i64;
let _22: [u16; 1];
let _23: char;
let _24: [i32; 7];
let _25: Adt58;
let _26: i128;
let _27: u128;
let _28: bool;
let _29: f64;
let _30: f32;
let _31: u128;
let _32: i8;
let _33: isize;
let _34: isize;
let _35: f64;
let _36: f32;
let _37: i64;
let _38: isize;
let _39: [i128; 5];
let _40: char;
let _41: [i64; 6];
let _42: ();
let _43: ();
{
RET = 126_isize;
_2 = [26808663230384702129007011982524287903_i128,156734039389044000245130088116381131399_i128,66508182603147973827477825515577370395_i128];
_4 = 10359152992705119654_usize as f32;
_4 = 162_u8 as f32;
_3 = !3770414639_u32;
RET = (-66_isize);
_2 = [(-134881866651812718182285858651358776111_i128),(-9675502049141621528779380520932847808_i128),37131429603569111529942901232166371902_i128];
_3 = 405012113_u32;
_4 = (-3094274487360465485_i64) as f32;
RET = 9223372036854775807_isize;
_1 = RET as i16;
_6 = [12693596195592806334_usize,3_usize,3_usize,3_usize,9461450820672722547_usize,5159819750052440718_usize];
_3 = 2320129237_u32;
_6 = [0_usize,6045985550535578266_usize,2_usize,17705946483451282612_usize,1_usize,1_usize];
_2 = [(-140710489689178438440201439183823793326_i128),(-14839776175917426095132334762977174474_i128),60769373455172811954674359246178716292_i128];
_8 = RET & RET;
_1 = 599899887_i32 as i16;
match RET {
9223372036854775807 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
RET = !_8;
_3 = 2871930213_u32;
_3 = 22840366_u32 ^ 2833035640_u32;
_6 = [4_usize,16723321470249905391_usize,6_usize,3_usize,15216824336136701488_usize,0_usize];
_6 = [12571573822887186130_usize,4481766978402727651_usize,14842100215053883257_usize,2_usize,5_usize,7_usize];
_1 = 7921_i16 >> _8;
_7 = _1 as f64;
_4 = 112_u8 as f32;
_7 = 312944451394292479488008758244717417784_u128 as f64;
_6 = [7_usize,8382796897153810953_usize,3_usize,7_usize,12906109813487614028_usize,13033262058363675388_usize];
_10 = '\u{f34fb}';
Goto(bb3)
}
bb3 = {
_4 = (-6950400792108398891_i64) as f32;
_10 = '\u{ee8f2}';
_5 = core::ptr::addr_of!(_10);
_6 = [6_usize,12066669273477271492_usize,0_usize,10937675634465092006_usize,5_usize,12138462010936443993_usize];
_12 = _2;
_12 = [47253259299982804803676041560628499291_i128,(-72190797462476004738156226106098166201_i128),(-150703287284695383146062291049774113806_i128)];
_12 = [38890514404449744444131828499161700464_i128,8496278700628784177833496275263345012_i128,(-121601980875299581666698136711293291153_i128)];
_11.1 = _3 << _1;
_12 = [(-143367474800458197408086900641447344676_i128),102439569117636189925552232530347012505_i128,15521148986577438830088197492389549601_i128];
Goto(bb4)
}
bb4 = {
_8 = 47255871132007507503803287123927840866_u128 as isize;
(*_5) = '\u{51d07}';
_13 = _2;
_3 = _1 as u32;
RET = 791441294_i32 as isize;
RET = _8 + _8;
_11.0 = [2281719866209187941_u64,7302809329773695833_u64,9240397246712505732_u64,8244455523326584380_u64];
_6 = [7895405094509585001_usize,4_usize,13075075826221926776_usize,3_usize,1_usize,3_usize];
_17 = _8 - RET;
_6 = [11953544489453362926_usize,16712974686839835154_usize,2_usize,1_usize,17405384188822375571_usize,0_usize];
_16 = 77_u8 as i8;
_13 = [(-86669921953863590109030846491323999036_i128),(-130815005236167520435137674177155580629_i128),(-80828581060700062026756335564746692889_i128)];
Goto(bb5)
}
bb5 = {
_11.2 = 0_u8 + 33_u8;
(*_5) = '\u{eff7c}';
_5 = core::ptr::addr_of!((*_5));
_3 = _11.1 | _11.1;
(*_5) = '\u{95ab5}';
_6 = [6898357379972813101_usize,7_usize,6_usize,16283386666691424282_usize,15024268065490503793_usize,3_usize];
_13 = [93634951983744898100971311041947408410_i128,(-128374817702037630358499747305904430899_i128),124104939956460206242304674451124520812_i128];
_15 = 7130512721205645705751005756435256095_u128;
_2 = [(-9344709617157186770656587940082871131_i128),63509530703553965401267076897227757804_i128,150584612389788511383823911213678018288_i128];
RET = -_17;
_16 = (-55_i8);
_11.2 = 41_u8;
_4 = _11.2 as f32;
_9 = [_11.2,_11.2,_11.2,_11.2,_11.2,_11.2,_11.2];
_5 = core::ptr::addr_of!(_10);
_13 = [90318329629653278268979966815333237837_i128,(-43093394955577862628553876491589646944_i128),(-101853695107320409364444307543049284400_i128)];
_5 = core::ptr::addr_of!(_10);
_8 = RET;
_22 = [59521_u16];
_22 = [39783_u16];
_17 = _8;
_23 = _10;
match _15 {
0 => bb1,
1 => bb3,
7130512721205645705751005756435256095 => bb7,
_ => bb6
}
}
bb6 = {
Return()
}
bb7 = {
_11.0 = [10025053512777753794_u64,7407105942330003065_u64,8915956338321260713_u64,17531550253110341362_u64];
_20 = 26692_u16 >> RET;
_4 = 0_usize as f32;
_2 = [168241221532912060242419404037015336721_i128,36137022486156120666784667070504306425_i128,(-15972422368606989577881847212177608749_i128)];
_12 = [106363607916797373561869776805518385260_i128,(-155873583005813402467607794972434074467_i128),(-28539562530823150240122875823700712020_i128)];
_13 = [149584860384746743619810336506639711215_i128,(-141306112457081388433526368577546062544_i128),33767590549457077133196012089431485968_i128];
_13 = [(-45760638025283930669436823060653828908_i128),(-158264133980747594087516100376006731252_i128),(-99551500901294897386549999536484565198_i128)];
_19 = [17547830761360341391_usize,3_usize,3_usize,6633494176914904622_usize,12528737621923418036_usize,3_usize];
_6 = [5_usize,10392471791467308207_usize,2028723165407477388_usize,7101847944315929907_usize,17350097416933339816_usize,15977002253896835763_usize];
_21 = -8206572205404556108_i64;
_4 = _7 as f32;
_8 = -_17;
_24 = [194161520_i32,(-1519853460_i32),292360614_i32,(-1503193926_i32),971553163_i32,(-1059589583_i32),368398289_i32];
_18 = _11.2 as u64;
_18 = 4696739836302170898_u64;
(*_5) = _23;
_6 = [594640592021984889_usize,7546664307526941959_usize,2_usize,13350008346279185989_usize,4_usize,0_usize];
_14 = _4 - _4;
(*_5) = _23;
_19 = _6;
RET = !_17;
_11.2 = !30_u8;
_20 = 44573_u16;
_12 = [(-83405233495175945711371848068400014850_i128),157379027122003647622351108338077454501_i128,6710517131677568777064087588627315896_i128];
Goto(bb8)
}
bb8 = {
_18 = 15795851612842384655_u64 ^ 13956466514179174537_u64;
_1 = _14 as i16;
_22 = [_20];
_4 = _14 * _14;
_8 = RET;
_3 = _16 as u32;
_11.2 = 104_u8 >> _8;
_3 = !_11.1;
_3 = !_11.1;
RET = !_17;
_14 = -_4;
_2 = [33518365442658466291192309014299808703_i128,86780032129355741753534033228248223833_i128,(-91436586334036620158326369597425114441_i128)];
_14 = _4;
_17 = _8;
_29 = -_7;
_33 = RET << _11.2;
_26 = !(-3442396819747596570810821592964335154_i128);
_36 = -_4;
_20 = !21319_u16;
match _16 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
340282366920938463463374607431768211401 => bb15,
_ => bb14
}
}
bb9 = {
_11.0 = [10025053512777753794_u64,7407105942330003065_u64,8915956338321260713_u64,17531550253110341362_u64];
_20 = 26692_u16 >> RET;
_4 = 0_usize as f32;
_2 = [168241221532912060242419404037015336721_i128,36137022486156120666784667070504306425_i128,(-15972422368606989577881847212177608749_i128)];
_12 = [106363607916797373561869776805518385260_i128,(-155873583005813402467607794972434074467_i128),(-28539562530823150240122875823700712020_i128)];
_13 = [149584860384746743619810336506639711215_i128,(-141306112457081388433526368577546062544_i128),33767590549457077133196012089431485968_i128];
_13 = [(-45760638025283930669436823060653828908_i128),(-158264133980747594087516100376006731252_i128),(-99551500901294897386549999536484565198_i128)];
_19 = [17547830761360341391_usize,3_usize,3_usize,6633494176914904622_usize,12528737621923418036_usize,3_usize];
_6 = [5_usize,10392471791467308207_usize,2028723165407477388_usize,7101847944315929907_usize,17350097416933339816_usize,15977002253896835763_usize];
_21 = -8206572205404556108_i64;
_4 = _7 as f32;
_8 = -_17;
_24 = [194161520_i32,(-1519853460_i32),292360614_i32,(-1503193926_i32),971553163_i32,(-1059589583_i32),368398289_i32];
_18 = _11.2 as u64;
_18 = 4696739836302170898_u64;
(*_5) = _23;
_6 = [594640592021984889_usize,7546664307526941959_usize,2_usize,13350008346279185989_usize,4_usize,0_usize];
_14 = _4 - _4;
(*_5) = _23;
_19 = _6;
RET = !_17;
_11.2 = !30_u8;
_20 = 44573_u16;
_12 = [(-83405233495175945711371848068400014850_i128),157379027122003647622351108338077454501_i128,6710517131677568777064087588627315896_i128];
Goto(bb8)
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_8 = 47255871132007507503803287123927840866_u128 as isize;
(*_5) = '\u{51d07}';
_13 = _2;
_3 = _1 as u32;
RET = 791441294_i32 as isize;
RET = _8 + _8;
_11.0 = [2281719866209187941_u64,7302809329773695833_u64,9240397246712505732_u64,8244455523326584380_u64];
_6 = [7895405094509585001_usize,4_usize,13075075826221926776_usize,3_usize,1_usize,3_usize];
_17 = _8 - RET;
_6 = [11953544489453362926_usize,16712974686839835154_usize,2_usize,1_usize,17405384188822375571_usize,0_usize];
_16 = 77_u8 as i8;
_13 = [(-86669921953863590109030846491323999036_i128),(-130815005236167520435137674177155580629_i128),(-80828581060700062026756335564746692889_i128)];
Goto(bb5)
}
bb13 = {
_4 = (-6950400792108398891_i64) as f32;
_10 = '\u{ee8f2}';
_5 = core::ptr::addr_of!(_10);
_6 = [6_usize,12066669273477271492_usize,0_usize,10937675634465092006_usize,5_usize,12138462010936443993_usize];
_12 = _2;
_12 = [47253259299982804803676041560628499291_i128,(-72190797462476004738156226106098166201_i128),(-150703287284695383146062291049774113806_i128)];
_12 = [38890514404449744444131828499161700464_i128,8496278700628784177833496275263345012_i128,(-121601980875299581666698136711293291153_i128)];
_11.1 = _3 << _1;
_12 = [(-143367474800458197408086900641447344676_i128),102439569117636189925552232530347012505_i128,15521148986577438830088197492389549601_i128];
Goto(bb4)
}
bb14 = {
RET = !_8;
_3 = 2871930213_u32;
_3 = 22840366_u32 ^ 2833035640_u32;
_6 = [4_usize,16723321470249905391_usize,6_usize,3_usize,15216824336136701488_usize,0_usize];
_6 = [12571573822887186130_usize,4481766978402727651_usize,14842100215053883257_usize,2_usize,5_usize,7_usize];
_1 = 7921_i16 >> _8;
_7 = _1 as f64;
_4 = 112_u8 as f32;
_7 = 312944451394292479488008758244717417784_u128 as f64;
_6 = [7_usize,8382796897153810953_usize,3_usize,7_usize,12906109813487614028_usize,13033262058363675388_usize];
_10 = '\u{f34fb}';
Goto(bb3)
}
bb15 = {
_34 = _33;
_30 = 1786100271_i32 as f32;
_23 = (*_5);
_32 = !_16;
_7 = _29 + _29;
_37 = _21 >> _8;
_20 = !27825_u16;
_12 = [_26,_26,_26];
(*_5) = _23;
_28 = !true;
_13 = [_26,_26,_26];
_41 = [_37,_37,_21,_37,_37,_37];
Goto(bb16)
}
bb16 = {
Call(_42 = dump_var(19_usize, 6_usize, Move(_6), 23_usize, Move(_23), 22_usize, Move(_22), 17_usize, Move(_17)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(19_usize, 34_usize, Move(_34), 28_usize, Move(_28), 20_usize, Move(_20), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(19_usize, 15_usize, Move(_15), 10_usize, Move(_10), 13_usize, Move(_13), 19_usize, Move(_19)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_42 = dump_var(19_usize, 32_usize, Move(_32), 43_usize, _43, 43_usize, _43, 43_usize, _43), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{1042f}'), std::hint::black_box(9223372036854775807_isize), std::hint::black_box((-54_i8)), std::hint::black_box((-10122_i16)), std::hint::black_box(1837074068_i32), std::hint::black_box((-2469142019794795566_i64)), std::hint::black_box(13420552591567857701473429629674269896_i128), std::hint::black_box(256036572152532702462167560824824232600_u128), std::hint::black_box(177_u8), std::hint::black_box(58891_u16), std::hint::black_box(3593951961_u32), std::hint::black_box(9651114016855479042_u64));
                
            }
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt46{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt46 {
fld0: u8,
}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: f32,
fld1: char,
fld2: isize,
fld3: *mut u32,
fld4: i16,
fld5: u16,

},
Variant1{
fld0: *mut i128,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: [u8; 7],
fld1: [u64; 4],
fld2: [i64; 6],
fld3: [i32; 7],
fld4: [u64; 6],
fld5: i32,
fld6: *mut u64,

},
Variant1{
fld0: *const i8,
fld1: *mut i128,
fld2: u128,
fld3: i8,
fld4: i16,
fld5: [usize; 6],
fld6: u16,
fld7: *mut u16,

},
Variant2{
fld0: (u32, [i128; 5], ([u8; 7], [u64; 4])),
fld1: [i128; 3],
fld2: [u8; 7],

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt49::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: bool,
fld1: usize,
fld2: [i32; 7],
fld3: ([u8; 7], [u64; 4]),
fld4: (i128, *mut i128, *mut i128, f32, i64),

},
Variant1{
fld0: *const char,
fld1: Adt46,
fld2: isize,
fld3: Adt47,
fld4: [u64; 4],
fld5: i32,
fld6: usize,

},
Variant2{
fld0: f32,
fld1: u128,

},
Variant3{
fld0: i16,
fld1: u128,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: i64,
fld1: Adt46,
fld2: u128,
fld3: i8,

},
Variant1{
fld0: u128,
fld1: u32,
fld2: *mut i128,
fld3: ([u64; 4], u32, u8),
fld4: [u64; 6],
fld5: ([u8; 7], [u64; 4]),
fld6: *mut u64,
fld7: [i128; 3],

},
Variant2{
fld0: bool,
fld1: [i128; 3],
fld2: (i128, *mut i128, *mut i128, f32, i64),
fld3: [u16; 1],
fld4: f32,
fld5: i32,

},
Variant3{
fld0: [i32; 7],
fld1: [usize; 6],

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt51::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: u64,
fld1: *const char,
fld2: Adt47,
fld3: i16,

},
Variant1{
fld0: *const i8,
fld1: f64,
fld2: ([u64; 4], u32, u8),
fld3: *mut u32,
fld4: Adt49,

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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: ([u64; 4], u32, u8),
fld1: Adt48,
fld2: Adt46,
fld3: *mut u64,
fld4: [u8; 7],
fld5: Adt47,
fld6: u8,
fld7: ((i128, *mut i128, *mut i128, f32, i64), *mut i128, i128),

},
Variant1{
fld0: [i128; 5],
fld1: u64,
fld2: (u32, [i128; 5], ([u8; 7], [u64; 4])),

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: f32,
fld1: *mut u64,
fld2: *const char,
}
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: [i128; 5],
fld1: *const i128,
fld2: isize,
fld3: f32,
fld4: u16,
fld5: [u16; 1],
fld6: f64,
fld7: [u8; 7],

},
Variant1{
fld0: [i128; 5],
fld1: Adt50,
fld2: Adt48,
fld3: ([u64; 4], u32, u8),

},
Variant2{
fld0: i16,
fld1: char,

},
Variant3{
fld0: (u32, i16),

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt55::".as_ptr()  as *const c_char)};match self{
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
fld0: (u32, [i128; 5], ([u8; 7], [u64; 4])),
fld1: *mut u16,
fld2: isize,
fld3: *const i8,
fld4: [i64; 6],
fld5: [i32; 7],
fld6: [u16; 1],
fld7: i128,

},
Variant1{
fld0: bool,
fld1: *mut u64,
fld2: [u16; 1],
fld3: Adt51,

},
Variant2{
fld0: u64,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf(c"Variant1{".as_ptr())};
		unsafe{printf(c"fld0:".as_ptr())};
		fld0.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: Adt46,
fld1: Adt52,
fld2: usize,
fld3: (u32, [i128; 5], ([u8; 7], [u64; 4])),
fld4: [u64; 4],
fld5: f32,

},
Variant1{
fld0: Adt46,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt57::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: Adt56,

},
Variant1{
fld0: Adt54,
fld1: ([u8; 7], [u64; 4]),
fld2: Adt48,
fld3: ([u64; 4], u32, u8),
fld4: *mut u32,

},
Variant2{
fld0: Adt50,
fld1: Adt56,
fld2: u16,
fld3: [u8; 7],

},
Variant3{
fld0: i64,
fld1: usize,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt58::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: Adt55,
fld1: [i128; 5],
fld2: Adt49,

},
Variant1{
fld0: u128,
fld1: Adt54,
fld2: Adt47,
fld3: [i64; 6],
fld4: u8,
fld5: *const i8,

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt59{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt59 {
fld0: Adt52,
fld1: Adt46,
fld2: Adt51,
fld3: u64,
fld4: Adt58,
fld5: u16,
fld6: [u8; 7],
}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt60::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: ([u8; 7], [u64; 4]),
fld1: char,
fld2: [i128; 3],

},
Variant1{
fld0: Adt47,
fld1: [i64; 6],
fld2: [u64; 4],

}}
impl PrintFDebug for Adt61{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt61::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt61 {
Variant0{
fld0: usize,
fld1: Adt57,
fld2: Adt48,
fld3: Adt59,
fld4: *mut u32,
fld5: *mut u64,

},
Variant1{
fld0: *mut i128,
fld1: Adt48,
fld2: (u32, i16),
fld3: Adt55,
fld4: f32,
fld5: [i128; 5],
fld6: i64,

}}
impl PrintFDebug for Adt62{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt62::".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt62 {
Variant0{
fld0: *const i8,
fld1: Adt49,
fld2: isize,
fld3: *const char,
fld4: Adt58,
fld5: [usize; 6],

},
Variant1{
fld0: *mut i128,
fld1: [i128; 5],
fld2: Adt56,
fld3: (u32, i16),
fld4: (u32, [i128; 5], ([u8; 7], [u64; 4])),

},
Variant2{
fld0: i128,
fld1: Adt48,
fld2: i8,

}}

