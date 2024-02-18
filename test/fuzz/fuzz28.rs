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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: u128,mut _4: u32,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: u16,mut _9: usize) -> u8 {
mir! {
type RET = u8;
let _10: [i8; 1];
let _11: f32;
let _12: f32;
let _13: f64;
let _14: u16;
let _15: f64;
let _16: [u64; 8];
let _17: ([i128; 2], u16, &'static char, &'static *mut *const &'static i32);
let _18: *mut &'static i16;
let _19: usize;
let _20: [u8; 8];
let _21: u128;
let _22: i8;
let _23: f32;
let _24: isize;
let _25: *mut ([i8; 1], [i64; 7], f32, *mut *mut (u128, f32, (i32, char)));
let _26: *mut (u128, f32, (i32, char));
let _27: bool;
let _28: &'static Adt17;
let _29: f64;
let _30: ([i8; 1], [i64; 7], f32, *mut *mut (u128, f32, (i32, char)));
let _31: char;
let _32: bool;
let _33: i128;
let _34: *const ((u128, f32, (i32, char)), i32);
let _35: ([i32; 4],);
let _36: [u64; 3];
let _37: &'static &'static [u64; 3];
let _38: [u64; 3];
let _39: (u128, f32, (i32, char));
let _40: i128;
let _41: ();
let _42: ();
{
_4 = 4773_u16 as u32;
RET = 201_u8;
_4 = !3187820174_u32;
_1 = RET != RET;
_1 = !true;
_2 = '\u{587a9}';
RET = 91_u8;
RET = 60249960197126445300127033757392780837_u128 as u8;
_7 = !(-61906312049791166_i64);
_8 = !19895_u16;
_8 = 551_u16 ^ 47010_u16;
_9 = RET as usize;
_1 = true;
RET = !63_u8;
_11 = 51036476818714066763018757519806436263_i128 as f32;
_2 = '\u{dd2d8}';
RET = 93_isize as u8;
_5 = -(-2216_i16);
_5 = !2977_i16;
_1 = false;
Goto(bb1)
}
bb1 = {
_10 = [(-81_i8)];
_6 = (-1303283481_i32) ^ (-890930667_i32);
_6 = 69667399847163582675175137493835274126_u128 as i32;
_11 = _7 as f32;
_1 = !true;
_8 = 15567_u16;
_4 = 82466676_u32 << RET;
_8 = 17949_u16 - 38165_u16;
_13 = _6 as f64;
_9 = _7 as usize;
Goto(bb2)
}
bb2 = {
_14 = 313274513049412618012413937085575180342_u128 as u16;
_12 = _11 * _11;
_12 = _11 - _11;
_9 = (-100_i8) as usize;
_11 = _12;
_7 = 4155022877509672612_i64 << _8;
_10 = [(-100_i8)];
_8 = _2 as u16;
_1 = false;
Call(_17.1 = core::intrinsics::transmute(_14), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_1 = _9 > _9;
_9 = 996000851044166810_usize;
RET = _6 as u8;
_10 = [71_i8];
_11 = _12 * _12;
_5 = 25279_i16;
_8 = !_14;
_11 = _12 * _12;
_7 = 5617159453387580389_i64 - (-5224941157453411538_i64);
_15 = -_13;
_3 = 228954646066414073661884593338314359230_u128 ^ 32415394690215372006046010504529795109_u128;
_19 = _9;
_11 = _12;
_17.0 = [110977016529184532848687661165591889130_i128,(-26570089599232139672139284203121103369_i128)];
_17.1 = _8 ^ _14;
_7 = 1047727396699921116_i64;
_3 = _9 as u128;
_13 = _4 as f64;
Call(_22 = core::intrinsics::bswap((-18_i8)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_4 = 1709903989_u32;
_19 = _9;
_7 = (-4879889084277957022_i64) + (-2750161127768050617_i64);
_2 = '\u{cda08}';
_20 = [RET,RET,RET,RET,RET,RET,RET,RET];
_16 = [9043037086223317945_u64,14622821471325890183_u64,4049435432919095158_u64,10970789637045227921_u64,8184017698786134486_u64,2987914219236795241_u64,2433183595472793131_u64,1466084287954092059_u64];
_11 = _7 as f32;
_19 = _9 - _9;
_15 = (-142129589399847634681006794115203152429_i128) as f64;
_5 = 25399_i16 - 15364_i16;
_24 = -79_isize;
_21 = _3 | _3;
_20 = [RET,RET,RET,RET,RET,RET,RET,RET];
_27 = _17.1 != _14;
_6 = -(-1809090277_i32);
_23 = _11 * _12;
_11 = _6 as f32;
_11 = -_23;
_17.2 = &_2;
_9 = !_19;
RET = 47_u8 ^ 48_u8;
_13 = (-128405300021666792276713701861415210507_i128) as f64;
_27 = _1;
_7 = -1844001042611931537_i64;
_9 = _27 as usize;
_8 = !_17.1;
_6 = -(-756224706_i32);
_16 = [16568830763958314616_u64,11008531894987395121_u64,9861390587482073805_u64,14151305474118522256_u64,1384888036836959720_u64,17759586912022442514_u64,463414687421820796_u64,2170906251042405875_u64];
match _4 {
0 => bb2,
1 => bb5,
1709903989 => bb7,
_ => bb6
}
}
bb5 = {
_1 = _9 > _9;
_9 = 996000851044166810_usize;
RET = _6 as u8;
_10 = [71_i8];
_11 = _12 * _12;
_5 = 25279_i16;
_8 = !_14;
_11 = _12 * _12;
_7 = 5617159453387580389_i64 - (-5224941157453411538_i64);
_15 = -_13;
_3 = 228954646066414073661884593338314359230_u128 ^ 32415394690215372006046010504529795109_u128;
_19 = _9;
_11 = _12;
_17.0 = [110977016529184532848687661165591889130_i128,(-26570089599232139672139284203121103369_i128)];
_17.1 = _8 ^ _14;
_7 = 1047727396699921116_i64;
_3 = _9 as u128;
_13 = _4 as f64;
Call(_22 = core::intrinsics::bswap((-18_i8)), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
_10 = [(-81_i8)];
_6 = (-1303283481_i32) ^ (-890930667_i32);
_6 = 69667399847163582675175137493835274126_u128 as i32;
_11 = _7 as f32;
_1 = !true;
_8 = 15567_u16;
_4 = 82466676_u32 << RET;
_8 = 17949_u16 - 38165_u16;
_13 = _6 as f64;
_9 = _7 as usize;
Goto(bb2)
}
bb7 = {
_11 = -_12;
_19 = _9 - _9;
_15 = _13 - _13;
_4 = !3444592216_u32;
_11 = -_23;
_8 = !_17.1;
_4 = 15382066181220702678_u64 as u32;
_21 = !_3;
_11 = -_23;
_29 = _15;
_1 = _27;
_17.0 = [153333831309549430601336687976052020014_i128,71245719035301473776547799552850944212_i128];
RET = 164_u8;
_17.2 = &_2;
_5 = 22342_i16;
_17.2 = &_2;
_13 = _23 as f64;
_20 = [RET,RET,RET,RET,RET,RET,RET,RET];
_30.3 = core::ptr::addr_of_mut!(_26);
_15 = _29;
Call(_1 = fn1(_27, _24, _14, _17.1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_17.1 = _21 as u16;
_17.0 = [(-2844837528092878707199238126902871295_i128),167003583818226975301541756478590928855_i128];
RET = _24 as u8;
_14 = RET as u16;
_16 = [7773887712859163054_u64,14461649647905600281_u64,9702137420257967368_u64,18384680663116585092_u64,5024640135192541014_u64,16464731423333634621_u64,8243083348100680650_u64,17915800525499732293_u64];
_9 = !_19;
_22 = (-24_i8) >> _14;
_30.2 = _8 as f32;
_17.1 = _22 as u16;
_4 = !2957778985_u32;
match _5 {
0 => bb1,
1 => bb2,
2 => bb7,
3 => bb4,
4 => bb5,
5 => bb9,
22342 => bb11,
_ => bb10
}
}
bb9 = {
_4 = 1709903989_u32;
_19 = _9;
_7 = (-4879889084277957022_i64) + (-2750161127768050617_i64);
_2 = '\u{cda08}';
_20 = [RET,RET,RET,RET,RET,RET,RET,RET];
_16 = [9043037086223317945_u64,14622821471325890183_u64,4049435432919095158_u64,10970789637045227921_u64,8184017698786134486_u64,2987914219236795241_u64,2433183595472793131_u64,1466084287954092059_u64];
_11 = _7 as f32;
_19 = _9 - _9;
_15 = (-142129589399847634681006794115203152429_i128) as f64;
_5 = 25399_i16 - 15364_i16;
_24 = -79_isize;
_21 = _3 | _3;
_20 = [RET,RET,RET,RET,RET,RET,RET,RET];
_27 = _17.1 != _14;
_6 = -(-1809090277_i32);
_23 = _11 * _12;
_11 = _6 as f32;
_11 = -_23;
_17.2 = &_2;
_9 = !_19;
RET = 47_u8 ^ 48_u8;
_13 = (-128405300021666792276713701861415210507_i128) as f64;
_27 = _1;
_7 = -1844001042611931537_i64;
_9 = _27 as usize;
_8 = !_17.1;
_6 = -(-756224706_i32);
_16 = [16568830763958314616_u64,11008531894987395121_u64,9861390587482073805_u64,14151305474118522256_u64,1384888036836959720_u64,17759586912022442514_u64,463414687421820796_u64,2170906251042405875_u64];
match _4 {
0 => bb2,
1 => bb5,
1709903989 => bb7,
_ => bb6
}
}
bb10 = {
_1 = _9 > _9;
_9 = 996000851044166810_usize;
RET = _6 as u8;
_10 = [71_i8];
_11 = _12 * _12;
_5 = 25279_i16;
_8 = !_14;
_11 = _12 * _12;
_7 = 5617159453387580389_i64 - (-5224941157453411538_i64);
_15 = -_13;
_3 = 228954646066414073661884593338314359230_u128 ^ 32415394690215372006046010504529795109_u128;
_19 = _9;
_11 = _12;
_17.0 = [110977016529184532848687661165591889130_i128,(-26570089599232139672139284203121103369_i128)];
_17.1 = _8 ^ _14;
_7 = 1047727396699921116_i64;
_3 = _9 as u128;
_13 = _4 as f64;
Call(_22 = core::intrinsics::bswap((-18_i8)), ReturnTo(bb4), UnwindUnreachable())
}
bb11 = {
_30.1 = [_7,_7,_7,_7,_7,_7,_7];
_30.3 = core::ptr::addr_of_mut!(_26);
_17.2 = &_31;
_6 = 1290839199_i32;
_32 = !_1;
_19 = _5 as usize;
_27 = _32;
_22 = 9366688740594068265_u64 as i8;
_16 = [13393187387031471656_u64,7241943667109203386_u64,1588852625955561155_u64,13187580174123511351_u64,16499500327625638243_u64,9772478664223207588_u64,12472171178621727576_u64,5237788506791426157_u64];
_32 = _27;
_8 = !_14;
_2 = '\u{10486d}';
_33 = _1 as i128;
_23 = _11 + _12;
_29 = _15;
_15 = _11 as f64;
_25 = core::ptr::addr_of_mut!(_30);
_13 = RET as f64;
_12 = _11 * _23;
_3 = !_21;
_16 = [9616134372696763261_u64,975448189506866839_u64,8846125419456401186_u64,548928645708013988_u64,15774786945172498754_u64,5411649297988825299_u64,3605798116296015024_u64,9433732929673273397_u64];
_17.0 = [_33,_33];
_17.2 = &_31;
(*_25).0 = _10;
_30.3 = core::ptr::addr_of_mut!(_26);
Goto(bb12)
}
bb12 = {
_5 = _15 as i16;
_8 = _17.1;
(*_25).1 = [_7,_7,_7,_7,_7,_7,_7];
_6 = (-103841493_i32);
_30.0 = [_22];
_24 = !9223372036854775807_isize;
_23 = _24 as f32;
_11 = _12 + _12;
_21 = _3 | _3;
(*_25).2 = _22 as f32;
match _6 {
0 => bb3,
1 => bb11,
2 => bb13,
3 => bb14,
4 => bb15,
5 => bb16,
6 => bb17,
340282366920938463463374607431664369963 => bb19,
_ => bb18
}
}
bb13 = {
_30.1 = [_7,_7,_7,_7,_7,_7,_7];
_30.3 = core::ptr::addr_of_mut!(_26);
_17.2 = &_31;
_6 = 1290839199_i32;
_32 = !_1;
_19 = _5 as usize;
_27 = _32;
_22 = 9366688740594068265_u64 as i8;
_16 = [13393187387031471656_u64,7241943667109203386_u64,1588852625955561155_u64,13187580174123511351_u64,16499500327625638243_u64,9772478664223207588_u64,12472171178621727576_u64,5237788506791426157_u64];
_32 = _27;
_8 = !_14;
_2 = '\u{10486d}';
_33 = _1 as i128;
_23 = _11 + _12;
_29 = _15;
_15 = _11 as f64;
_25 = core::ptr::addr_of_mut!(_30);
_13 = RET as f64;
_12 = _11 * _23;
_3 = !_21;
_16 = [9616134372696763261_u64,975448189506866839_u64,8846125419456401186_u64,548928645708013988_u64,15774786945172498754_u64,5411649297988825299_u64,3605798116296015024_u64,9433732929673273397_u64];
_17.0 = [_33,_33];
_17.2 = &_31;
(*_25).0 = _10;
_30.3 = core::ptr::addr_of_mut!(_26);
Goto(bb12)
}
bb14 = {
_10 = [(-81_i8)];
_6 = (-1303283481_i32) ^ (-890930667_i32);
_6 = 69667399847163582675175137493835274126_u128 as i32;
_11 = _7 as f32;
_1 = !true;
_8 = 15567_u16;
_4 = 82466676_u32 << RET;
_8 = 17949_u16 - 38165_u16;
_13 = _6 as f64;
_9 = _7 as usize;
Goto(bb2)
}
bb15 = {
_4 = 1709903989_u32;
_19 = _9;
_7 = (-4879889084277957022_i64) + (-2750161127768050617_i64);
_2 = '\u{cda08}';
_20 = [RET,RET,RET,RET,RET,RET,RET,RET];
_16 = [9043037086223317945_u64,14622821471325890183_u64,4049435432919095158_u64,10970789637045227921_u64,8184017698786134486_u64,2987914219236795241_u64,2433183595472793131_u64,1466084287954092059_u64];
_11 = _7 as f32;
_19 = _9 - _9;
_15 = (-142129589399847634681006794115203152429_i128) as f64;
_5 = 25399_i16 - 15364_i16;
_24 = -79_isize;
_21 = _3 | _3;
_20 = [RET,RET,RET,RET,RET,RET,RET,RET];
_27 = _17.1 != _14;
_6 = -(-1809090277_i32);
_23 = _11 * _12;
_11 = _6 as f32;
_11 = -_23;
_17.2 = &_2;
_9 = !_19;
RET = 47_u8 ^ 48_u8;
_13 = (-128405300021666792276713701861415210507_i128) as f64;
_27 = _1;
_7 = -1844001042611931537_i64;
_9 = _27 as usize;
_8 = !_17.1;
_6 = -(-756224706_i32);
_16 = [16568830763958314616_u64,11008531894987395121_u64,9861390587482073805_u64,14151305474118522256_u64,1384888036836959720_u64,17759586912022442514_u64,463414687421820796_u64,2170906251042405875_u64];
match _4 {
0 => bb2,
1 => bb5,
1709903989 => bb7,
_ => bb6
}
}
bb16 = {
_14 = 313274513049412618012413937085575180342_u128 as u16;
_12 = _11 * _11;
_12 = _11 - _11;
_9 = (-100_i8) as usize;
_11 = _12;
_7 = 4155022877509672612_i64 << _8;
_10 = [(-100_i8)];
_8 = _2 as u16;
_1 = false;
Call(_17.1 = core::intrinsics::transmute(_14), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_11 = -_12;
_19 = _9 - _9;
_15 = _13 - _13;
_4 = !3444592216_u32;
_11 = -_23;
_8 = !_17.1;
_4 = 15382066181220702678_u64 as u32;
_21 = !_3;
_11 = -_23;
_29 = _15;
_1 = _27;
_17.0 = [153333831309549430601336687976052020014_i128,71245719035301473776547799552850944212_i128];
RET = 164_u8;
_17.2 = &_2;
_5 = 22342_i16;
_17.2 = &_2;
_13 = _23 as f64;
_20 = [RET,RET,RET,RET,RET,RET,RET,RET];
_30.3 = core::ptr::addr_of_mut!(_26);
_15 = _29;
Call(_1 = fn1(_27, _24, _14, _17.1), ReturnTo(bb8), UnwindUnreachable())
}
bb18 = {
_10 = [(-81_i8)];
_6 = (-1303283481_i32) ^ (-890930667_i32);
_6 = 69667399847163582675175137493835274126_u128 as i32;
_11 = _7 as f32;
_1 = !true;
_8 = 15567_u16;
_4 = 82466676_u32 << RET;
_8 = 17949_u16 - 38165_u16;
_13 = _6 as f64;
_9 = _7 as usize;
Goto(bb2)
}
bb19 = {
_6 = (-812315431_i32);
_30.3 = core::ptr::addr_of_mut!(_26);
_3 = _15 as u128;
_12 = _3 as f32;
_21 = _9 as u128;
_7 = _27 as i64;
_30.3 = core::ptr::addr_of_mut!(_26);
_36 = [557368131029092206_u64,3493052901097148222_u64,3369967135914658618_u64];
_26 = core::ptr::addr_of_mut!(_39);
_25 = core::ptr::addr_of_mut!((*_25));
(*_26).2.1 = _2;
Goto(bb20)
}
bb20 = {
Call(_41 = dump_var(0_usize, 24_usize, Move(_24), 2_usize, Move(_2), 9_usize, Move(_9), 4_usize, Move(_4)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_41 = dump_var(0_usize, 1_usize, Move(_1), 21_usize, Move(_21), 5_usize, Move(_5), 10_usize, Move(_10)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_41 = dump_var(0_usize, 7_usize, Move(_7), 16_usize, Move(_16), 42_usize, _42, 42_usize, _42), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: bool,mut _2: isize,mut _3: u16,mut _4: u16) -> bool {
mir! {
type RET = bool;
let _5: &'static char;
let _6: *mut ([i8; 1], [i64; 7], f32, *mut *mut (u128, f32, (i32, char)));
let _7: Adt24;
let _8: f64;
let _9: *mut [i128; 2];
let _10: *const *mut [i128; 2];
let _11: &'static [u64; 3];
let _12: char;
let _13: char;
let _14: [i8; 5];
let _15: Adt32;
let _16: &'static *mut *const &'static i32;
let _17: ((u128, f32, (i32, char)), i32);
let _18: *mut u16;
let _19: *mut *const *const &'static i32;
let _20: f32;
let _21: char;
let _22: ();
let _23: ();
{
RET = _3 <= _4;
_1 = RET;
RET = !_1;
_3 = _4 ^ _4;
_4 = _3;
RET = _3 >= _4;
_1 = !RET;
_2 = 51_isize;
_4 = !_3;
_3 = 47577983_u32 as u16;
_3 = _4 >> _4;
RET = _1 ^ _1;
_3 = _4;
_3 = !_4;
_7.fld0 = 8281223255770513848_u64;
_7.fld2 = !(-8717975651296053714_i64);
RET = _1;
_3 = (-66825407066146352148118430248300927934_i128) as u16;
_7.fld1 = [(-139108987034698303299080363187382106419_i128),(-62809139085985843066514748983934496231_i128)];
_3 = _4 | _4;
_7.fld2 = -4371668949854823388_i64;
_2 = 1831863692_u32 as isize;
_7.fld0 = 16870876245738539718_u64 + 12852635284818749259_u64;
Call(_6 = fn2(_2, RET, _1, _3, RET, _1, _4, _3, _2, RET, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_7.fld0 = !5129043398767037907_u64;
_8 = _3 as f64;
_8 = 2757617900_u32 as f64;
_9 = core::ptr::addr_of_mut!(_7.fld1);
_2 = RET as isize;
_7.fld2 = (-4014579303826560222_i64);
_4 = !_3;
_1 = RET;
_7.fld1 = [65917636950938555615324327132579046828_i128,30319883451888028615951114019145533116_i128];
_10 = core::ptr::addr_of!(_9);
RET = _1;
_10 = core::ptr::addr_of!((*_10));
(*_10) = core::ptr::addr_of_mut!((*_9));
match _7.fld2 {
340282366920938463459360028127941651234 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_7.fld1 = [104294315455034564614719806448783511508_i128,79949017775480840871385686855344568346_i128];
_7.fld1 = [(-43897749331177362579049402310112065574_i128),(-66911819763430291440878877330358629602_i128)];
_12 = '\u{f11b2}';
RET = _4 < _3;
(*_9) = [29412167338961475411074159967077328228_i128,(-95947092295377757903581052966869189533_i128)];
_10 = core::ptr::addr_of!((*_10));
(*_10) = core::ptr::addr_of_mut!((*_9));
_8 = 1388524372_i32 as f64;
(*_10) = core::ptr::addr_of_mut!((*_9));
_12 = '\u{3b0c3}';
RET = !_1;
_5 = &_13;
match _7.fld2 {
0 => bb1,
1 => bb4,
2 => bb5,
3 => bb6,
340282366920938463459360028127941651234 => bb8,
_ => bb7
}
}
bb4 = {
Return()
}
bb5 = {
_7.fld0 = !5129043398767037907_u64;
_8 = _3 as f64;
_8 = 2757617900_u32 as f64;
_9 = core::ptr::addr_of_mut!(_7.fld1);
_2 = RET as isize;
_7.fld2 = (-4014579303826560222_i64);
_4 = !_3;
_1 = RET;
_7.fld1 = [65917636950938555615324327132579046828_i128,30319883451888028615951114019145533116_i128];
_10 = core::ptr::addr_of!(_9);
RET = _1;
_10 = core::ptr::addr_of!((*_10));
(*_10) = core::ptr::addr_of_mut!((*_9));
match _7.fld2 {
340282366920938463459360028127941651234 => bb3,
_ => bb2
}
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_2 = (-9223372036854775808_isize) - 83_isize;
_7.fld2 = !(-8239533091284412905_i64);
_13 = _12;
_5 = &_12;
_13 = (*_5);
_7.fld1 = [155053393094831976441168410261785285953_i128,(-29253469468995921420157760702459969744_i128)];
(*_9) = [(-71689038425052102665635904690567861187_i128),105012957278503062284191193104587268440_i128];
_2 = (-9223372036854775808_isize);
_9 = core::ptr::addr_of_mut!((*_9));
_7.fld1 = [12013500438157731051028166833226469342_i128,(-99476352644590093945440934797419176992_i128)];
Goto(bb9)
}
bb9 = {
match _2 {
0 => bb10,
1 => bb11,
2 => bb12,
340282366920938463454151235394913435648 => bb14,
_ => bb13
}
}
bb10 = {
_2 = (-9223372036854775808_isize) - 83_isize;
_7.fld2 = !(-8239533091284412905_i64);
_13 = _12;
_5 = &_12;
_13 = (*_5);
_7.fld1 = [155053393094831976441168410261785285953_i128,(-29253469468995921420157760702459969744_i128)];
(*_9) = [(-71689038425052102665635904690567861187_i128),105012957278503062284191193104587268440_i128];
_2 = (-9223372036854775808_isize);
_9 = core::ptr::addr_of_mut!((*_9));
_7.fld1 = [12013500438157731051028166833226469342_i128,(-99476352644590093945440934797419176992_i128)];
Goto(bb9)
}
bb11 = {
_7.fld0 = !5129043398767037907_u64;
_8 = _3 as f64;
_8 = 2757617900_u32 as f64;
_9 = core::ptr::addr_of_mut!(_7.fld1);
_2 = RET as isize;
_7.fld2 = (-4014579303826560222_i64);
_4 = !_3;
_1 = RET;
_7.fld1 = [65917636950938555615324327132579046828_i128,30319883451888028615951114019145533116_i128];
_10 = core::ptr::addr_of!(_9);
RET = _1;
_10 = core::ptr::addr_of!((*_10));
(*_10) = core::ptr::addr_of_mut!((*_9));
match _7.fld2 {
340282366920938463459360028127941651234 => bb3,
_ => bb2
}
}
bb12 = {
Return()
}
bb13 = {
_7.fld0 = !5129043398767037907_u64;
_8 = _3 as f64;
_8 = 2757617900_u32 as f64;
_9 = core::ptr::addr_of_mut!(_7.fld1);
_2 = RET as isize;
_7.fld2 = (-4014579303826560222_i64);
_4 = !_3;
_1 = RET;
_7.fld1 = [65917636950938555615324327132579046828_i128,30319883451888028615951114019145533116_i128];
_10 = core::ptr::addr_of!(_9);
RET = _1;
_10 = core::ptr::addr_of!((*_10));
(*_10) = core::ptr::addr_of_mut!((*_9));
match _7.fld2 {
340282366920938463459360028127941651234 => bb3,
_ => bb2
}
}
bb14 = {
_17.0.0 = 25010637153300576167472162401756856552_u128;
_17.0.2 = (1415687332_i32, _12);
_9 = core::ptr::addr_of_mut!((*_9));
_18 = core::ptr::addr_of_mut!(_4);
_1 = RET | RET;
_7.fld0 = !10290432723266460230_u64;
(*_18) = _3;
_17.0.0 = !4264844726794136525467508005054381608_u128;
_17.0.1 = _17.0.0 as f32;
_17.0.1 = _7.fld2 as f32;
(*_10) = core::ptr::addr_of_mut!((*_9));
_9 = core::ptr::addr_of_mut!((*_9));
RET = _1;
_8 = (-647_i16) as f64;
_9 = core::ptr::addr_of_mut!(_7.fld1);
Goto(bb15)
}
bb15 = {
Call(_22 = dump_var(1_usize, 13_usize, Move(_13), 4_usize, Move(_4), 2_usize, Move(_2), 23_usize, _23), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: isize,mut _2: bool,mut _3: bool,mut _4: u16,mut _5: bool,mut _6: bool,mut _7: u16,mut _8: u16,mut _9: isize,mut _10: bool,mut _11: bool) -> *mut ([i8; 1], [i64; 7], f32, *mut *mut (u128, f32, (i32, char))) {
mir! {
type RET = *mut ([i8; 1], [i64; 7], f32, *mut *mut (u128, f32, (i32, char)));
let _12: [u128; 2];
let _13: isize;
let _14: bool;
let _15: bool;
let _16: f32;
let _17: Adt79;
let _18: i8;
let _19: (i32, char);
let _20: [u8; 8];
let _21: *mut i8;
let _22: i16;
let _23: char;
let _24: i128;
let _25: &'static u8;
let _26: isize;
let _27: usize;
let _28: isize;
let _29: bool;
let _30: [i8; 5];
let _31: ([i128; 2], u16, &'static char, &'static *mut *const &'static i32);
let _32: [i64; 8];
let _33: &'static [u64; 3];
let _34: [u64; 3];
let _35: *const *const &'static i32;
let _36: &'static [i8; 2];
let _37: *const ((u128, f32, (i32, char)), i32);
let _38: bool;
let _39: &'static [u64; 3];
let _40: i32;
let _41: *mut (u128, f32, (i32, char));
let _42: &'static Adt17;
let _43: *mut *mut (u128, f32, (i32, char));
let _44: ((isize, [i8; 1], char, f32), *mut ([i8; 1], [i64; 7], f32, *mut *mut (u128, f32, (i32, char))), [u64; 3], &'static i32);
let _45: ([i8; 1], [i64; 7], f32, *mut *mut (u128, f32, (i32, char)));
let _46: &'static char;
let _47: bool;
let _48: ();
let _49: ();
{
_10 = _2;
_5 = !_11;
_5 = _4 == _8;
_4 = 240100695659957042885078166913970853519_u128 as u16;
_6 = !_5;
_7 = _8;
_5 = !_6;
_5 = _11;
_5 = !_6;
_2 = !_6;
_6 = _5 >= _11;
_1 = _9;
_1 = -_9;
_8 = _4;
_12 = [329993435041471458737081860487090411041_u128,267219788835792161387817428697623961184_u128];
_5 = !_3;
_10 = _11;
_9 = _1 ^ _1;
_6 = _3 <= _5;
_2 = _3;
_2 = _10;
_5 = !_6;
_7 = _8 | _8;
Call(_11 = fn3(_4, _9, _9, _6, _4, _10, _10, _10, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _11;
_10 = !_3;
_5 = _11 ^ _3;
_13 = !_9;
_3 = _5 >= _10;
_14 = _3 & _3;
_13 = 8130616939715487315_u64 as isize;
_12 = [49821945596664152406910316609892873487_u128,295858592073205203426436548992046922345_u128];
Goto(bb2)
}
bb2 = {
_8 = !_7;
_1 = _13 - _9;
_4 = (-652736102_i32) as u16;
_13 = -_1;
_6 = _5 ^ _11;
_9 = -_1;
_15 = !_10;
_13 = _1 >> _8;
_8 = _4;
_17.fld0.2 = [1947710893_i32,(-329457421_i32),(-1073791560_i32),203221457_i32];
_17.fld0.2 = [93909618_i32,(-739171395_i32),(-1420474480_i32),780887302_i32];
_13 = _1;
_12 = [12449660922791159446672795273016634871_u128,295382867252729702034084072910159040515_u128];
_17.fld2 = (_17.fld0.2,);
_17.fld0.0 = 2025045689688704268248560816368463834_i128 as f64;
_17.fld0.2 = _17.fld2.0;
_17.fld0.0 = _13 as f64;
Call(_12 = fn4(_1, _15, _15, _15, _15, _9, _14, _11), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_19 = (267622213_i32, '\u{ed010}');
_10 = !_6;
_19.0 = (-409009156_i32) ^ (-1481308129_i32);
_7 = 116899363415870618190394906895111112487_i128 as u16;
_19.0 = -1559595150_i32;
_19 = (1537156615_i32, '\u{68963}');
_6 = _14 > _11;
_18 = 57_i8 - 30_i8;
_8 = _4;
_19 = ((-191803340_i32), '\u{3fb4f}');
_17.fld0.0 = 191_u8 as f64;
_5 = _3 > _11;
_18 = 16965581562995301173_u64 as i8;
_2 = _15 != _11;
_17.fld0.1 = _9 ^ _1;
_17.fld2.0 = _17.fld0.2;
_7 = _4;
_2 = _11 <= _6;
_17.fld2 = (_17.fld0.2,);
_17.fld5 = _14 as i32;
_17.fld0.0 = _17.fld5 as f64;
match _19.0 {
0 => bb4,
1 => bb5,
2 => bb6,
3 => bb7,
4 => bb8,
340282366920938463463374607431576408116 => bb10,
_ => bb9
}
}
bb4 = {
_8 = !_7;
_1 = _13 - _9;
_4 = (-652736102_i32) as u16;
_13 = -_1;
_6 = _5 ^ _11;
_9 = -_1;
_15 = !_10;
_13 = _1 >> _8;
_8 = _4;
_17.fld0.2 = [1947710893_i32,(-329457421_i32),(-1073791560_i32),203221457_i32];
_17.fld0.2 = [93909618_i32,(-739171395_i32),(-1420474480_i32),780887302_i32];
_13 = _1;
_12 = [12449660922791159446672795273016634871_u128,295382867252729702034084072910159040515_u128];
_17.fld2 = (_17.fld0.2,);
_17.fld0.0 = 2025045689688704268248560816368463834_i128 as f64;
_17.fld0.2 = _17.fld2.0;
_17.fld0.0 = _13 as f64;
Call(_12 = fn4(_1, _15, _15, _15, _15, _9, _14, _11), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_3 = _11;
_10 = !_3;
_5 = _11 ^ _3;
_13 = !_9;
_3 = _5 >= _10;
_14 = _3 & _3;
_13 = 8130616939715487315_u64 as isize;
_12 = [49821945596664152406910316609892873487_u128,295858592073205203426436548992046922345_u128];
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
_19 = (_17.fld5, '\u{8cb56}');
_17.fld0.1 = _9 >> _19.0;
_17.fld2.0 = [_19.0,_17.fld5,_19.0,_19.0];
_9 = _17.fld0.1 | _17.fld0.1;
_12 = [185033144896921911540906851321704436161_u128,248283475714855668936240989526590274184_u128];
_23 = _19.1;
_17.fld5 = _19.0;
Call(_19.0 = core::intrinsics::transmute(_17.fld5), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_16 = 137737571647197825984593061442126066675_i128 as f32;
_19 = (_17.fld5, _23);
_20 = [175_u8,206_u8,30_u8,145_u8,129_u8,88_u8,18_u8,132_u8];
_17.fld0.1 = _17.fld0.0 as isize;
_9 = _18 as isize;
_22 = -(-8632_i16);
_6 = _23 != _23;
_17.fld2 = (_17.fld0.2,);
_12 = [314380750095107071886368091198901500205_u128,113836738907131866389065046732215664563_u128];
_6 = _11 | _3;
_19.0 = _17.fld5;
_2 = _5;
_7 = _4 | _8;
_23 = _19.1;
_17.fld3 = _7 << _17.fld5;
_10 = !_15;
_16 = _17.fld5 as f32;
_3 = _5;
_17.fld2 = (_17.fld0.2,);
_24 = -119967798064982815361256241883301065636_i128;
_6 = !_2;
_20 = [25_u8,213_u8,194_u8,118_u8,156_u8,119_u8,157_u8,80_u8];
_3 = _17.fld0.0 >= _17.fld0.0;
Goto(bb12)
}
bb12 = {
_19.1 = _23;
Call(_17.fld0.1 = fn5(_15, _11, _16, _17.fld3, _17.fld3, _3, _19.0, _17.fld5, _15, _17.fld5, _19.1, _17.fld5), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_13 = _23 as isize;
_14 = _10;
_18 = !(-82_i8);
_19.1 = _23;
_17.fld0.1 = -_13;
_6 = _13 <= _13;
_16 = 12914655029319580158_u64 as f32;
_21 = core::ptr::addr_of_mut!(_18);
_24 = !156992835026514689194049608763509949095_i128;
_2 = !_11;
_26 = _17.fld0.1 & _13;
Goto(bb14)
}
bb14 = {
_28 = _26 * _17.fld0.1;
_17.fld3 = _8 * _4;
_17.fld2 = (_17.fld0.2,);
_19.0 = 3698395008839863160_i64 as i32;
_4 = !_17.fld3;
_19 = (_17.fld5, _23);
_17.fld5 = _19.0 << _17.fld0.1;
_5 = _6;
_23 = _19.1;
_12 = [167140749248494398957847133062683953797_u128,118823122212979473463465243442316925049_u128];
_3 = _14 >= _11;
_14 = !_3;
_24 = (-144415825513447860159177093383558679886_i128);
_17.fld2 = (_17.fld0.2,);
_1 = !_26;
_21 = core::ptr::addr_of_mut!((*_21));
_17.fld3 = _7 * _8;
_23 = _19.1;
_26 = _2 as isize;
_24 = -(-44583209377810777903425586608034017313_i128);
Goto(bb15)
}
bb15 = {
_5 = !_11;
_6 = _15;
_32 = [1354141632368393572_i64,(-9137919026073441488_i64),(-6053193319826875201_i64),(-276231339594128398_i64),7671791617116222700_i64,(-6762327229972101905_i64),(-4778248375480707959_i64),7319400948540292448_i64];
_31.2 = &_19.1;
_28 = _23 as isize;
_28 = _17.fld0.1 << _13;
_31.2 = &_19.1;
_6 = !_3;
_17.fld2.0 = _17.fld0.2;
_21 = core::ptr::addr_of_mut!(_18);
_17.fld5 = _19.0;
_27 = !0_usize;
_33 = &_34;
_5 = _15;
_31.0 = [_24,_24];
_23 = _19.1;
_15 = _5 ^ _3;
_17.fld2.0 = [_17.fld5,_19.0,_17.fld5,_17.fld5];
_17.fld3 = !_7;
_5 = !_14;
Goto(bb16)
}
bb16 = {
_2 = !_15;
_17.fld0.1 = !_13;
_21 = core::ptr::addr_of_mut!((*_21));
_19 = (_17.fld5, _23);
_9 = _13 * _26;
(*_21) = -(-60_i8);
_15 = _19.0 == _19.0;
Goto(bb17)
}
bb17 = {
_17.fld3 = !_7;
_28 = _26 | _1;
_31.0 = [_24,_24];
_31.1 = _22 as u16;
Goto(bb18)
}
bb18 = {
_15 = !_3;
_17.fld0.1 = !_9;
_30 = [(*_21),_18,_18,_18,_18];
_6 = !_15;
Goto(bb19)
}
bb19 = {
(*_21) = _16 as i8;
_2 = !_14;
(*_21) = -(-46_i8);
_31.2 = &_23;
_19.0 = _17.fld5 << _17.fld0.1;
_17.fld0.0 = _7 as f64;
_20 = [174_u8,215_u8,220_u8,52_u8,118_u8,205_u8,71_u8,159_u8];
_17.fld2.0 = [_19.0,_17.fld5,_19.0,_19.0];
_17.fld0.0 = 2151395881_u32 as f64;
_32 = [122299783987165496_i64,(-5554939621050268984_i64),(-6459395456663657998_i64),152310836692869889_i64,(-1558744562458645417_i64),(-7899828074293573186_i64),(-603274067283788952_i64),8885044038857409342_i64];
_7 = (*_21) as u16;
_8 = _4 ^ _17.fld3;
_26 = -_28;
_31.1 = _4 * _7;
_17.fld0.1 = !_13;
_40 = _24 as i32;
_19 = (_17.fld5, _23);
_16 = 163346077631503191351684954206433551022_u128 as f32;
_17.fld4 = [1170316850_u32,4135230779_u32,2436680545_u32,4026065610_u32];
_9 = _17.fld5 as isize;
_28 = _17.fld0.1;
Goto(bb20)
}
bb20 = {
_43 = core::ptr::addr_of_mut!(_41);
_8 = 2425830918_u32 as u16;
_10 = _3;
_45.0 = [_18];
_16 = 766116313_u32 as f32;
RET = core::ptr::addr_of_mut!(_45);
(*RET).1 = [(-1033527437461584255_i64),(-2008118105777931641_i64),(-825274871026896188_i64),3887397796685826364_i64,6093209470296905155_i64,(-6118094954248353598_i64),2932718542860395657_i64];
_39 = &_44.2;
_29 = _11;
_44.0 = (_17.fld0.1, _45.0, _23, _16);
_10 = _11;
_39 = &(*_39);
_5 = _44.0.2 == _19.1;
_1 = -_44.0.0;
_27 = 3_usize;
_45.0 = [(*_21)];
(*RET).3 = core::ptr::addr_of_mut!((*_43));
_44.3 = &_40;
_28 = (*_21) as isize;
(*RET).1 = [_32[_27],_32[_27],_32[_27],_32[_27],_32[_27],_32[_27],_32[_27]];
(*RET).2 = -_44.0.3;
Goto(bb21)
}
bb21 = {
Call(_48 = dump_var(2_usize, 23_usize, Move(_23), 27_usize, Move(_27), 11_usize, Move(_11), 12_usize, Move(_12)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_48 = dump_var(2_usize, 26_usize, Move(_26), 14_usize, Move(_14), 32_usize, Move(_32), 22_usize, Move(_22)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_48 = dump_var(2_usize, 40_usize, Move(_40), 28_usize, Move(_28), 18_usize, Move(_18), 19_usize, Move(_19)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_48 = dump_var(2_usize, 30_usize, Move(_30), 6_usize, Move(_6), 49_usize, _49, 49_usize, _49), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: u16,mut _2: isize,mut _3: isize,mut _4: bool,mut _5: u16,mut _6: bool,mut _7: bool,mut _8: bool,mut _9: bool) -> bool {
mir! {
type RET = bool;
let _10: i16;
let _11: Adt50;
let _12: char;
let _13: isize;
let _14: Adt50;
let _15: ();
let _16: ();
{
_7 = _9 & _9;
RET = _7;
_8 = RET;
_10 = 145_u8 as i16;
_8 = !RET;
_4 = _7 <= _7;
RET = _7 <= _4;
_8 = RET < _4;
_5 = _1 << _10;
_3 = 39818267281017411861634513773880194190_i128 as isize;
_6 = RET ^ _4;
_9 = !_6;
_9 = _6 & _6;
Goto(bb1)
}
bb1 = {
Call(_15 = dump_var(3_usize, 5_usize, Move(_5), 9_usize, Move(_9), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_15 = dump_var(3_usize, 10_usize, Move(_10), 16_usize, _16, 16_usize, _16, 16_usize, _16), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: isize,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: isize,mut _7: bool,mut _8: bool) -> [u128; 2] {
mir! {
type RET = [u128; 2];
let _9: f32;
let _10: isize;
let _11: &'static u128;
let _12: Adt76;
let _13: &'static *mut *const &'static i32;
let _14: ([i8; 1], [i64; 7], f32, *mut *mut (u128, f32, (i32, char)));
let _15: u128;
let _16: *mut u8;
let _17: (i32, char);
let _18: Adt45;
let _19: isize;
let _20: char;
let _21: [i8; 5];
let _22: [i128; 2];
let _23: i64;
let _24: *mut (u128, f32, (i32, char));
let _25: Adt62;
let _26: &'static [u64; 3];
let _27: (i64,);
let _28: char;
let _29: &'static &'static [u64; 3];
let _30: u16;
let _31: ();
let _32: ();
{
Goto(bb1)
}
bb1 = {
_4 = !_7;
RET = [127905517365806908625964423925633722489_u128,176373893843681303946902081047545543895_u128];
RET = [94474603094998896344688928326945689228_u128,8361721935404614720180384057533356332_u128];
_8 = _2 & _3;
_1 = !_6;
_4 = _2 < _7;
_9 = 12377305699337836489_u64 as f32;
_8 = !_5;
_3 = _5;
RET = [64492890937055558149564174082249579939_u128,328364952287642654360712050138371205743_u128];
_3 = _8 | _8;
_1 = -_6;
_10 = -_1;
RET = [261609989027561981262546709232606063668_u128,56710728526697843995784595445319877980_u128];
_9 = 194014544662004865438516837092239811922_u128 as f32;
_5 = !_7;
_8 = !_2;
_3 = !_7;
Goto(bb2)
}
bb2 = {
_2 = _4;
_12.fld1 = 12099294558299606637_u64 & 3958229699382685572_u64;
_1 = 205808998256240463375306393052354096790_u128 as isize;
_1 = _6 >> _12.fld1;
_7 = _3;
_8 = _4 != _4;
_12.fld0 = [(-1067923794_i32),290151916_i32,142393492_i32,(-1581360316_i32)];
RET = [80430648169431958173961211052703038659_u128,195995196783987842270106463821692875397_u128];
_8 = !_4;
_9 = 1041158200_u32 as f32;
_2 = _4 == _8;
RET = [141493750487742274599822111682549850956_u128,299294632224539015157428167894893876132_u128];
_8 = _5;
_3 = _8;
RET = [291459136855683598854425827667023885060_u128,332449899805072589082891490062785171698_u128];
_7 = _5 ^ _3;
Goto(bb3)
}
bb3 = {
_5 = _3;
_11 = &_15;
_17.1 = '\u{6e10b}';
_19 = -_6;
Goto(bb4)
}
bb4 = {
_11 = &(*_11);
_17 = ((-531608630_i32), '\u{6d426}');
_2 = !_3;
_12.fld0 = [_17.0,_17.0,_17.0,_17.0];
_19 = _12.fld1 as isize;
RET = [91384752139637968730615326080820983697_u128,243706173154186118706850891798021285087_u128];
_14.2 = 64_u8 as f32;
Call(_15 = core::intrinsics::transmute(_12.fld0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_12.fld1 = 7243429829125054203_u64 ^ 13680388149470595372_u64;
_14.1 = [(-6338897030089175572_i64),(-1564676924301743791_i64),(-5203131951299819636_i64),(-1347457427428616470_i64),6977289851236780880_i64,(-1187735008153912292_i64),3821431506689260787_i64];
_14.0 = [77_i8];
_20 = _17.1;
_2 = !_4;
_1 = -_10;
_6 = _19 - _1;
_20 = _17.1;
RET = [_15,_15];
_14.0 = [107_i8];
_8 = _5;
Goto(bb6)
}
bb6 = {
_1 = _10 * _6;
_9 = -_14.2;
_15 = _17.0 as u128;
_14.1 = [9028231634586192664_i64,(-7911924495406356968_i64),(-8702541171755526304_i64),7043617664722904163_i64,5286440358606809_i64,1912827734804838399_i64,(-1429667696543296481_i64)];
_5 = !_2;
_3 = _7;
_12.fld2 = Adt62::Variant3 { fld0: (-122_i8),fld1: _19 };
_4 = _5;
_14.1 = [(-7395460447658683256_i64),2491548391583629642_i64,(-3692012831468167651_i64),3517081550444322218_i64,(-3712528555713895740_i64),(-711818348450990013_i64),1126446069036690078_i64];
_4 = !_3;
place!(Field::<i8>(Variant(_12.fld2, 3), 0)) = !68_i8;
_21 = [Field::<i8>(Variant(_12.fld2, 3), 0),Field::<i8>(Variant(_12.fld2, 3), 0),Field::<i8>(Variant(_12.fld2, 3), 0),Field::<i8>(Variant(_12.fld2, 3), 0),Field::<i8>(Variant(_12.fld2, 3), 0)];
_8 = _3;
_20 = _17.1;
_12.fld0 = [_17.0,_17.0,_17.0,_17.0];
match _17.0 {
0 => bb7,
1 => bb8,
2 => bb9,
3 => bb10,
4 => bb11,
340282366920938463463374607431236602826 => bb13,
_ => bb12
}
}
bb7 = {
_12.fld1 = 7243429829125054203_u64 ^ 13680388149470595372_u64;
_14.1 = [(-6338897030089175572_i64),(-1564676924301743791_i64),(-5203131951299819636_i64),(-1347457427428616470_i64),6977289851236780880_i64,(-1187735008153912292_i64),3821431506689260787_i64];
_14.0 = [77_i8];
_20 = _17.1;
_2 = !_4;
_1 = -_10;
_6 = _19 - _1;
_20 = _17.1;
RET = [_15,_15];
_14.0 = [107_i8];
_8 = _5;
Goto(bb6)
}
bb8 = {
_11 = &(*_11);
_17 = ((-531608630_i32), '\u{6d426}');
_2 = !_3;
_12.fld0 = [_17.0,_17.0,_17.0,_17.0];
_19 = _12.fld1 as isize;
RET = [91384752139637968730615326080820983697_u128,243706173154186118706850891798021285087_u128];
_14.2 = 64_u8 as f32;
Call(_15 = core::intrinsics::transmute(_12.fld0), ReturnTo(bb5), UnwindUnreachable())
}
bb9 = {
_5 = _3;
_11 = &_15;
_17.1 = '\u{6e10b}';
_19 = -_6;
Goto(bb4)
}
bb10 = {
_2 = _4;
_12.fld1 = 12099294558299606637_u64 & 3958229699382685572_u64;
_1 = 205808998256240463375306393052354096790_u128 as isize;
_1 = _6 >> _12.fld1;
_7 = _3;
_8 = _4 != _4;
_12.fld0 = [(-1067923794_i32),290151916_i32,142393492_i32,(-1581360316_i32)];
RET = [80430648169431958173961211052703038659_u128,195995196783987842270106463821692875397_u128];
_8 = !_4;
_9 = 1041158200_u32 as f32;
_2 = _4 == _8;
RET = [141493750487742274599822111682549850956_u128,299294632224539015157428167894893876132_u128];
_8 = _5;
_3 = _8;
RET = [291459136855683598854425827667023885060_u128,332449899805072589082891490062785171698_u128];
_7 = _5 ^ _3;
Goto(bb3)
}
bb11 = {
_4 = !_7;
RET = [127905517365806908625964423925633722489_u128,176373893843681303946902081047545543895_u128];
RET = [94474603094998896344688928326945689228_u128,8361721935404614720180384057533356332_u128];
_8 = _2 & _3;
_1 = !_6;
_4 = _2 < _7;
_9 = 12377305699337836489_u64 as f32;
_8 = !_5;
_3 = _5;
RET = [64492890937055558149564174082249579939_u128,328364952287642654360712050138371205743_u128];
_3 = _8 | _8;
_1 = -_6;
_10 = -_1;
RET = [261609989027561981262546709232606063668_u128,56710728526697843995784595445319877980_u128];
_9 = 194014544662004865438516837092239811922_u128 as f32;
_5 = !_7;
_8 = !_2;
_3 = !_7;
Goto(bb2)
}
bb12 = {
Return()
}
bb13 = {
_8 = !_7;
_11 = &_15;
_8 = _4 != _2;
_25 = Move(_12.fld2);
_29 = &_26;
Goto(bb14)
}
bb14 = {
place!(Field::<i8>(Variant(_25, 3), 0)) = -(-53_i8);
_7 = !_3;
_4 = _2 > _2;
_17.0 = (-1973857333_i32);
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(4_usize, 6_usize, Move(_6), 4_usize, Move(_4), 3_usize, Move(_3), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(4_usize, 5_usize, Move(_5), 7_usize, Move(_7), 10_usize, Move(_10), 32_usize, _32), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: bool,mut _2: bool,mut _3: f32,mut _4: u16,mut _5: u16,mut _6: bool,mut _7: i32,mut _8: i32,mut _9: bool,mut _10: i32,mut _11: char,mut _12: i32) -> isize {
mir! {
type RET = isize;
let _13: isize;
let _14: &'static [i8; 2];
let _15: f32;
let _16: &'static *const *mut [i128; 2];
let _17: &'static &'static [u64; 3];
let _18: [i32; 4];
let _19: Adt24;
let _20: u8;
let _21: [i8; 1];
let _22: [u64; 8];
let _23: u64;
let _24: ();
let _25: ();
{
_11 = '\u{be0ce}';
_7 = _12;
_4 = 7641816596909806377_usize as u16;
RET = -100_isize;
RET = 117_i8 as isize;
_11 = '\u{4f12b}';
_4 = _5 ^ _5;
_11 = '\u{b5f4a}';
_1 = !_2;
_13 = RET;
_4 = _5;
_7 = _10 + _8;
_7 = _10;
Call(_6 = fn6(_2, _3, _2, _8, _3, _12, _12), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8 = !_10;
_8 = _10;
Goto(bb2)
}
bb2 = {
_7 = _8;
_5 = !_4;
_1 = _12 > _7;
_3 = 8935011108407537041_u64 as f32;
Goto(bb3)
}
bb3 = {
_6 = _1;
_13 = !RET;
_18 = [_8,_7,_7,_7];
_18 = [_7,_8,_8,_8];
_8 = _7;
_11 = '\u{cc734}';
RET = _13 + _13;
_8 = 2047827507631829723_i64 as i32;
_12 = _7 ^ _10;
_10 = _3 as i32;
Call(_3 = fn7(_7, _12, _4, _12, _1, _1, _6, _7, _2, _4, _1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_5 = _4 | _4;
_3 = 8930239891502573049_u64 as f32;
_2 = _6 != _6;
_15 = -_3;
RET = -_13;
_19.fld2 = (-168300339224081423082850593825605302181_i128) as i64;
_15 = _3;
_19.fld1 = [(-68029739725761671231282709459338499219_i128),(-91273977789637423203173220875005811190_i128)];
_11 = '\u{d46f7}';
_19.fld1 = [52895003351975028060459562366613292633_i128,136201840731987717367679165594635934462_i128];
_7 = !_12;
_18 = [_7,_12,_7,_7];
_12 = _10 - _10;
_3 = _15 * _15;
_1 = _6 <= _9;
_8 = _7 ^ _7;
_13 = RET;
_13 = _19.fld2 as isize;
_9 = _2;
_19.fld0 = 613623798328652537_u64;
RET = _7 as isize;
_5 = _4 - _4;
_23 = !_19.fld0;
_22 = [_23,_19.fld0,_23,_23,_23,_19.fld0,_19.fld0,_19.fld0];
_12 = _7;
_8 = _7;
Goto(bb5)
}
bb5 = {
Call(_24 = dump_var(5_usize, 8_usize, Move(_8), 12_usize, Move(_12), 4_usize, Move(_4), 7_usize, Move(_7)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_24 = dump_var(5_usize, 6_usize, Move(_6), 18_usize, Move(_18), 10_usize, Move(_10), 25_usize, _25), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: bool,mut _2: f32,mut _3: bool,mut _4: i32,mut _5: f32,mut _6: i32,mut _7: i32) -> bool {
mir! {
type RET = bool;
let _8: *mut i8;
let _9: ();
let _10: ();
{
_6 = _4;
RET = !_1;
_1 = _3 | RET;
_1 = !_3;
_6 = _7 ^ _4;
_5 = _2 + _2;
_5 = _2 - _2;
_2 = _5 * _5;
_3 = !_1;
_1 = RET;
RET = _1;
_3 = _1;
_3 = !_1;
Goto(bb1)
}
bb1 = {
Call(_9 = dump_var(6_usize, 1_usize, Move(_1), 6_usize, Move(_6), 10_usize, _10, 10_usize, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: i32,mut _2: i32,mut _3: u16,mut _4: i32,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: i32,mut _9: bool,mut _10: u16,mut _11: bool) -> f32 {
mir! {
type RET = f32;
let _12: (isize, [i8; 1], char, f32);
let _13: isize;
let _14: u32;
let _15: [i64; 8];
let _16: f64;
let _17: *mut ([i8; 1], [i64; 7], f32, *mut *mut (u128, f32, (i32, char)));
let _18: i16;
let _19: char;
let _20: &'static [u64; 3];
let _21: [u128; 2];
let _22: [i128; 2];
let _23: u8;
let _24: (&'static u8, *const *mut [i128; 2], &'static Adt17);
let _25: i64;
let _26: isize;
let _27: u8;
let _28: isize;
let _29: &'static i32;
let _30: u128;
let _31: [u64; 8];
let _32: i64;
let _33: *mut [u64; 1];
let _34: (&'static u8, *const i8);
let _35: f64;
let _36: [u64; 1];
let _37: isize;
let _38: char;
let _39: f64;
let _40: ([i128; 2], u16, &'static char, &'static *mut *const &'static i32);
let _41: &'static [u64; 3];
let _42: u64;
let _43: &'static [u64; 1];
let _44: (&'static u8, *const *mut [i128; 2], &'static Adt17);
let _45: (i64,);
let _46: u32;
let _47: &'static u128;
let _48: *mut u16;
let _49: ();
let _50: ();
{
_7 = !_9;
_11 = !_7;
_2 = !_1;
_10 = _3 | _3;
_8 = -_2;
_11 = _6 <= _9;
_3 = _10;
_11 = !_6;
_4 = _2;
RET = 16442002147514655853_usize as f32;
_5 = _7 != _7;
_8 = 78_u8 as i32;
RET = 2709748994012459510_i64 as f32;
_14 = _2 as u32;
_12.1 = [17_i8];
_14 = !3685116038_u32;
_9 = _7;
_7 = _5;
_4 = !_1;
_14 = '\u{108014}' as u32;
Goto(bb1)
}
bb1 = {
_5 = !_7;
_16 = 18001011101662844378_u64 as f64;
_1 = _4 | _2;
RET = 106997834196894943370004408370203303281_i128 as f32;
_14 = 644683503_u32 << _4;
_13 = (-9223372036854775808_isize);
_12.0 = _13;
_16 = 105_u8 as f64;
_9 = _5 != _5;
_1 = !_4;
_12.0 = _14 as isize;
_13 = _12.0 | _12.0;
_18 = -1078_i16;
_9 = _5;
_12.2 = '\u{21546}';
_14 = 51_u8 as u32;
_18 = (-29135_i16) << _10;
_12.2 = '\u{64167}';
_12.1 = [(-80_i8)];
_12.2 = '\u{7ee96}';
_5 = _12.0 > _13;
_1 = -_2;
_12.0 = 7916303308602145176_u64 as isize;
_1 = _4 << _13;
_12.2 = '\u{98079}';
_13 = _12.0 >> _4;
_1 = 105_i8 as i32;
Goto(bb2)
}
bb2 = {
_11 = _6;
_12.0 = _13;
_12.1 = [9_i8];
_1 = _4 - _2;
_3 = _2 as u16;
_12.0 = _13 - _13;
_15 = [(-8024698939808502745_i64),1258958180675483548_i64,(-2033310511018497774_i64),5226428386962739666_i64,(-1619833849903227_i64),5380720472312741238_i64,7021866917222637016_i64,9100997453316071864_i64];
_22 = [(-89484061617700126842425349850353520470_i128),169413455138397214053902661023939446627_i128];
_11 = !_6;
_21 = [71824858126877741388379847839892984475_u128,269441682126447951973114519682277657076_u128];
_12.3 = RET;
_2 = 122_i8 as i32;
_22 = [(-34810828660446781616629226529270824714_i128),30416945730334208046515506468463516694_i128];
_11 = _7;
_7 = _10 > _3;
_12.0 = _13 >> _13;
_12.1 = [(-106_i8)];
Call(_21 = fn8(_12.0, _3, _7, _13, _13, _3, _4, _12.0, _12, _11), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_28 = _13 >> _12.0;
_1 = _4 + _4;
_25 = 1161127945056672464_i64 * 4344500666590526773_i64;
_11 = _7 | _5;
_6 = _7;
_1 = _4;
_25 = (-5075127424647537217_i64) ^ (-8856661277319803696_i64);
_31 = [14769933055943581412_u64,3610389811716509087_u64,13943717547875710287_u64,2009718755891299178_u64,10956758254014147745_u64,6409285186243893031_u64,11169128887546513606_u64,3590898470141453215_u64];
_11 = _5 ^ _5;
_10 = _3;
_12.1 = [(-71_i8)];
_8 = _12.2 as i32;
_3 = _10 >> _10;
_3 = _10;
Goto(bb4)
}
bb4 = {
_15 = [_25,_25,_25,_25,_25,_25,_25,_25];
_34.0 = &_23;
_19 = _12.2;
_3 = _10;
Goto(bb5)
}
bb5 = {
_30 = 6782448476252070586105852003141552256_u128 + 1269946851781853486951622184620662314_u128;
_26 = !_28;
RET = -_12.3;
_12.1 = [(-33_i8)];
RET = _12.3;
_38 = _19;
_5 = _7 | _7;
_38 = _19;
_29 = &_4;
_23 = 241_u8 | 124_u8;
_21 = [_30,_30];
_23 = !68_u8;
Goto(bb6)
}
bb6 = {
_30 = 173504257260239758356609198506229183293_u128 - 318621614707951759028526289144783759815_u128;
_36 = [13132127267658514333_u64];
_38 = _12.2;
_14 = _18 as u32;
_30 = _14 as u128;
_31 = [14746705641480261109_u64,6113429158019984308_u64,18013016470116156568_u64,109933352266242426_u64,10529663586133813390_u64,12851225545221010409_u64,15805484514114162034_u64,14149785525502266211_u64];
_8 = !_4;
_12.3 = -RET;
_26 = _28 << (*_29);
_8 = (*_29) ^ _4;
_37 = _26 & _13;
_14 = 2474960874_u32 - 2862180655_u32;
_40.1 = _3 & _3;
_30 = 6812834766190934864634093481710872335_u128;
_28 = _37;
match _30 {
0 => bb7,
1 => bb8,
2 => bb9,
6812834766190934864634093481710872335 => bb11,
_ => bb10
}
}
bb7 = {
_30 = 6782448476252070586105852003141552256_u128 + 1269946851781853486951622184620662314_u128;
_26 = !_28;
RET = -_12.3;
_12.1 = [(-33_i8)];
RET = _12.3;
_38 = _19;
_5 = _7 | _7;
_38 = _19;
_29 = &_4;
_23 = 241_u8 | 124_u8;
_21 = [_30,_30];
_23 = !68_u8;
Goto(bb6)
}
bb8 = {
_15 = [_25,_25,_25,_25,_25,_25,_25,_25];
_34.0 = &_23;
_19 = _12.2;
_3 = _10;
Goto(bb5)
}
bb9 = {
_28 = _13 >> _12.0;
_1 = _4 + _4;
_25 = 1161127945056672464_i64 * 4344500666590526773_i64;
_11 = _7 | _5;
_6 = _7;
_1 = _4;
_25 = (-5075127424647537217_i64) ^ (-8856661277319803696_i64);
_31 = [14769933055943581412_u64,3610389811716509087_u64,13943717547875710287_u64,2009718755891299178_u64,10956758254014147745_u64,6409285186243893031_u64,11169128887546513606_u64,3590898470141453215_u64];
_11 = _5 ^ _5;
_10 = _3;
_12.1 = [(-71_i8)];
_8 = _12.2 as i32;
_3 = _10 >> _10;
_3 = _10;
Goto(bb4)
}
bb10 = {
_5 = !_7;
_16 = 18001011101662844378_u64 as f64;
_1 = _4 | _2;
RET = 106997834196894943370004408370203303281_i128 as f32;
_14 = 644683503_u32 << _4;
_13 = (-9223372036854775808_isize);
_12.0 = _13;
_16 = 105_u8 as f64;
_9 = _5 != _5;
_1 = !_4;
_12.0 = _14 as isize;
_13 = _12.0 | _12.0;
_18 = -1078_i16;
_9 = _5;
_12.2 = '\u{21546}';
_14 = 51_u8 as u32;
_18 = (-29135_i16) << _10;
_12.2 = '\u{64167}';
_12.1 = [(-80_i8)];
_12.2 = '\u{7ee96}';
_5 = _12.0 > _13;
_1 = -_2;
_12.0 = 7916303308602145176_u64 as isize;
_1 = _4 << _13;
_12.2 = '\u{98079}';
_13 = _12.0 >> _4;
_1 = 105_i8 as i32;
Goto(bb2)
}
bb11 = {
_12.0 = _28 + _26;
_25 = -4054399693674523009_i64;
_34.0 = &_27;
_27 = _16 as u8;
_12.3 = -RET;
_18 = 6625_i16;
match _18 {
6625 => bb12,
_ => bb6
}
}
bb12 = {
RET = -_12.3;
_12.0 = _26 >> _10;
_11 = _7;
_28 = _26;
_8 = _1 & (*_29);
_11 = _1 < _8;
_32 = _25;
_33 = core::ptr::addr_of_mut!(_36);
_2 = (*_29);
Goto(bb13)
}
bb13 = {
_28 = _12.0 ^ _12.0;
_38 = _19;
_15 = [_32,_25,_25,_32,_25,_32,_25,_25];
_10 = 3_usize as u16;
_7 = _6;
_40.2 = &_38;
_11 = _9 == _6;
_5 = _2 > (*_29);
_10 = _40.1 >> _37;
_43 = &(*_33);
_37 = -_13;
_18 = -(-12043_i16);
_38 = _12.2;
_40.2 = &_19;
_12.3 = -RET;
Goto(bb14)
}
bb14 = {
_40.1 = !_3;
_45.0 = _25;
_12.2 = _38;
_42 = 11598192708965498067_u64 >> (*_29);
(*_33) = [_42];
_15 = [_45.0,_25,_25,_45.0,_32,_45.0,_45.0,_32];
_6 = _12.0 <= _26;
_35 = _16 * _16;
_16 = _35;
_40.0 = _22;
_4 = _2 * _2;
_7 = _5;
_6 = _40.1 < _10;
(*_33) = [_42];
_5 = _9;
_5 = _11 | _11;
_38 = _19;
_3 = !_10;
_29 = &_4;
_43 = &(*_33);
_31 = [_42,_42,_42,_42,_42,_42,_42,_42];
_14 = !4269201325_u32;
Goto(bb15)
}
bb15 = {
Call(_49 = dump_var(7_usize, 27_usize, Move(_27), 10_usize, Move(_10), 2_usize, Move(_2), 31_usize, Move(_31)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_49 = dump_var(7_usize, 19_usize, Move(_19), 26_usize, Move(_26), 6_usize, Move(_6), 38_usize, Move(_38)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_49 = dump_var(7_usize, 5_usize, Move(_5), 14_usize, Move(_14), 4_usize, Move(_4), 42_usize, Move(_42)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_49 = dump_var(7_usize, 8_usize, Move(_8), 25_usize, Move(_25), 45_usize, Move(_45), 50_usize, _50), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: isize,mut _2: u16,mut _3: bool,mut _4: isize,mut _5: isize,mut _6: u16,mut _7: i32,mut _8: isize,mut _9: (isize, [i8; 1], char, f32),mut _10: bool) -> [u128; 2] {
mir! {
type RET = [u128; 2];
let _11: char;
let _12: char;
let _13: f32;
let _14: *const *const &'static i32;
let _15: isize;
let _16: isize;
let _17: u16;
let _18: bool;
let _19: &'static char;
let _20: char;
let _21: &'static &'static &'static [u64; 3];
let _22: f32;
let _23: f64;
let _24: [u64; 8];
let _25: i16;
let _26: [i32; 4];
let _27: [i8; 1];
let _28: [usize; 6];
let _29: *const &'static i32;
let _30: i64;
let _31: usize;
let _32: f32;
let _33: isize;
let _34: [i64; 2];
let _35: f64;
let _36: *mut (u128, f32, (i32, char));
let _37: (f64, isize, [i32; 4]);
let _38: isize;
let _39: &'static &'static [u64; 3];
let _40: &'static u32;
let _41: (u128, f32, (i32, char));
let _42: bool;
let _43: f32;
let _44: &'static i16;
let _45: ();
let _46: ();
{
RET = [129818936431761077210874561726945977584_u128,130294717994250761553115307931168413243_u128];
_7 = (-92_i8) as i32;
_7 = 516965787_i32;
_9.1 = [(-55_i8)];
_6 = _2;
_7 = (-1016084424_i32) & 1265039208_i32;
_1 = !_5;
_10 = !_3;
Goto(bb1)
}
bb1 = {
_9.0 = !_5;
_7 = (-354117663_i32) * (-249859848_i32);
_9.2 = '\u{79cb1}';
_9.0 = !_1;
_9.2 = '\u{4d781}';
_9.2 = '\u{ac678}';
_6 = !_2;
RET = [99687686349375545014532070664452353844_u128,137834881744157728037457528269087747146_u128];
_2 = _6;
_9.1 = [29_i8];
_11 = _9.2;
_4 = _5 >> _8;
Goto(bb2)
}
bb2 = {
_10 = !_3;
_9.3 = 4603_i16 as f32;
_6 = _2;
_3 = _10;
_12 = _11;
_3 = _10 > _10;
_9.0 = -_8;
_11 = _12;
_5 = _8 & _8;
_6 = !_2;
_15 = !_4;
_15 = 17728660684579809895_usize as isize;
RET = [100337448486154757031736223908736534076_u128,329135916583691135514054869162123974880_u128];
_4 = 150_u8 as isize;
_9.1 = [(-74_i8)];
_13 = -_9.3;
Call(_16 = fn9(_2, _5, _8, _9.0, _8, _5, _9, _10, _8, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_15 = -_16;
_10 = _3;
_18 = _3;
_6 = !_2;
_1 = _5 * _9.0;
_9.3 = _13 + _13;
_17 = _2 >> _5;
RET = [312225800242924507235839202611086313083_u128,223211248368624958251677940978386468136_u128];
_7 = (-1310281487_i32) | (-1029322252_i32);
_7 = !(-1058952214_i32);
_5 = _1;
_6 = _17;
_1 = -_9.0;
_22 = _9.3 - _13;
_9.3 = -_22;
_5 = -_1;
_15 = -_1;
_24 = [8899344476975047956_u64,6751004265771484483_u64,4782422666742130153_u64,2898707617655971156_u64,15911964708411126086_u64,14537544408396338664_u64,9508766039321106625_u64,9867183261514237950_u64];
_9.2 = _11;
_19 = &_11;
_6 = _2 * _17;
_9.0 = _1;
_8 = _15 >> _17;
_15 = _1 << _8;
_3 = _18;
Goto(bb4)
}
bb4 = {
_8 = 277609723502604012488218483246353292668_u128 as isize;
_12 = (*_19);
_24 = [6009723230567997386_u64,13293720205085113091_u64,1486892513826821996_u64,15876042355681130465_u64,9959328272573988064_u64,15246522764997301482_u64,12281862307405567288_u64,17439352165462817156_u64];
_5 = _15;
_20 = _11;
RET = [225742733172167047772442858448289684912_u128,2615224897539416991562153569505798009_u128];
_2 = _6;
_9.3 = _22 * _22;
RET = [105808497106531324434927589819603184045_u128,151507852787774263042442617329719824008_u128];
_12 = _20;
Goto(bb5)
}
bb5 = {
_13 = (-35_i8) as f32;
_9.0 = -_15;
_12 = _9.2;
_10 = _6 != _2;
_2 = 12885574943746908464_u64 as u16;
_2 = _6;
_20 = (*_19);
_26 = [_7,_7,_7,_7];
_23 = 2138034056_u32 as f64;
Goto(bb6)
}
bb6 = {
_22 = _9.3;
_26 = [_7,_7,_7,_7];
_28 = [8966309586291546078_usize,5_usize,15303035974412298974_usize,2_usize,17517340803912478021_usize,16910165946877637847_usize];
_24 = [8715219984198953857_u64,8007929793290483689_u64,17620727743250690904_u64,1448677242481985175_u64,18129181370692869250_u64,16351812445482664250_u64,5906498145590475824_u64,10494470001458721965_u64];
_18 = _10;
_27 = _9.1;
Goto(bb7)
}
bb7 = {
_23 = 91_i8 as f64;
_8 = 662599219_u32 as isize;
_5 = _15;
_27 = [(-1_i8)];
_19 = &_12;
RET = [261165726656297543089541965414885053403_u128,226465453239670090342741611852175127068_u128];
_9.0 = _15 << _17;
_27 = [101_i8];
_9.3 = -_13;
_11 = _12;
Call(_9 = fn11(_15, _1, _16, _5), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_25 = 4013104349_u32 as i16;
_24 = [3712222537365643839_u64,15209961615656469657_u64,9068152601239751978_u64,7665860138928018711_u64,15612250188458438861_u64,5602734767691128607_u64,12264987195888649673_u64,14686989279042897768_u64];
_24 = [3574822967314916953_u64,9711252442187052636_u64,2000922169828730876_u64,5526701573872213073_u64,10278202010242852700_u64,17313279227927251066_u64,9996472150881592397_u64,12443664075289620886_u64];
_4 = 93816506640606006135167404880779210085_u128 as isize;
_6 = !_2;
_13 = _22;
_31 = !5_usize;
_20 = _11;
_9.0 = _3 as isize;
_7 = (-1984056289_i32) - 1021994265_i32;
_28 = [_31,_31,_31,_31,_31,_31];
_31 = (-12660645451836742022526137613839628499_i128) as usize;
_9.2 = _12;
_27 = _9.1;
_1 = -_15;
Goto(bb9)
}
bb9 = {
_10 = !_18;
_30 = -4164885648850233785_i64;
_6 = _2;
_19 = &_9.2;
_22 = -_13;
_9.3 = _13 - _22;
Call(_29 = fn12(_9, _15, _5, _1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_3 = _18 < _10;
_28 = [_31,_31,_31,_31,_31,_31];
_19 = &_9.2;
_9 = (_15, _27, _12, _22);
_9 = (_5, _27, _11, _13);
_32 = _9.3;
_10 = !_18;
_28 = [_31,_31,_31,_31,_31,_31];
_13 = _9.3;
_3 = !_18;
_34 = [_30,_30];
_13 = -_22;
_17 = !_2;
_22 = _7 as f32;
_31 = 3_usize;
_14 = core::ptr::addr_of!(_29);
RET = [217870640490723858378310813238513808178_u128,278958930358138496039469487192624444989_u128];
_9.1 = [72_i8];
_12 = _11;
_35 = _1 as f64;
Goto(bb11)
}
bb11 = {
_4 = _9.0;
_7 = _26[_31] >> _15;
_35 = -_23;
_28[_31] = _25 as usize;
_26[_31] = _7 << _15;
_11 = _9.2;
_19 = &_12;
_34 = [_30,_30];
_13 = _26[_31] as f32;
_16 = _5 | _5;
_10 = !_18;
_37.2[_31] = _26[_31] << _4;
_19 = &_12;
_12 = _9.2;
_26 = [_37.2[_31],_37.2[_31],_37.2[_31],_37.2[_31]];
_37.2[_31] = _30 as i32;
_22 = _35 as f32;
_33 = _15 * _4;
_27 = [25_i8];
_2 = !_17;
_9.0 = _16 | _33;
_1 = _5 << _7;
_9.2 = _11;
RET = [161376932123261396551268662600402455766_u128,17378516748583993503228295054084634429_u128];
_2 = 57_u8 as u16;
Goto(bb12)
}
bb12 = {
_12 = _11;
_27 = [122_i8];
_37.1 = _15 * _4;
_38 = _5 ^ _15;
_41.1 = _13;
_37.0 = -_23;
_34 = [_30,_30];
_1 = _28[_31] as isize;
_19 = &_9.2;
_37.2 = _26;
_42 = _17 == _6;
_21 = &_39;
Call(_24 = fn18(Move((*_14)), Move(_14), Move(_19), _18, _41.1, _37.1, _17, _3, _9, _15, _7, _37.1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
RET = [149500618053866672678147073857452788360_u128,77987136803483999500513386832982861864_u128];
_9.0 = !_16;
_28 = [_31,_31,_31,_31,_31,_31];
_7 = !1052452072_i32;
_41.2.1 = _12;
match _31 {
0 => bb10,
1 => bb2,
2 => bb14,
4 => bb16,
5 => bb17,
6 => bb18,
3 => bb20,
_ => bb19
}
}
bb14 = {
_12 = _11;
_27 = [122_i8];
_37.1 = _15 * _4;
_38 = _5 ^ _15;
_41.1 = _13;
_37.0 = -_23;
_34 = [_30,_30];
_1 = _28[_31] as isize;
_19 = &_9.2;
_37.2 = _26;
_42 = _17 == _6;
_21 = &_39;
Call(_24 = fn18(Move((*_14)), Move(_14), Move(_19), _18, _41.1, _37.1, _17, _3, _9, _15, _7, _37.1), ReturnTo(bb13), UnwindUnreachable())
}
bb15 = {
_9.0 = !_5;
_7 = (-354117663_i32) * (-249859848_i32);
_9.2 = '\u{79cb1}';
_9.0 = !_1;
_9.2 = '\u{4d781}';
_9.2 = '\u{ac678}';
_6 = !_2;
RET = [99687686349375545014532070664452353844_u128,137834881744157728037457528269087747146_u128];
_2 = _6;
_9.1 = [29_i8];
_11 = _9.2;
_4 = _5 >> _8;
Goto(bb2)
}
bb16 = {
_22 = _9.3;
_26 = [_7,_7,_7,_7];
_28 = [8966309586291546078_usize,5_usize,15303035974412298974_usize,2_usize,17517340803912478021_usize,16910165946877637847_usize];
_24 = [8715219984198953857_u64,8007929793290483689_u64,17620727743250690904_u64,1448677242481985175_u64,18129181370692869250_u64,16351812445482664250_u64,5906498145590475824_u64,10494470001458721965_u64];
_18 = _10;
_27 = _9.1;
Goto(bb7)
}
bb17 = {
_13 = (-35_i8) as f32;
_9.0 = -_15;
_12 = _9.2;
_10 = _6 != _2;
_2 = 12885574943746908464_u64 as u16;
_2 = _6;
_20 = (*_19);
_26 = [_7,_7,_7,_7];
_23 = 2138034056_u32 as f64;
Goto(bb6)
}
bb18 = {
_25 = 4013104349_u32 as i16;
_24 = [3712222537365643839_u64,15209961615656469657_u64,9068152601239751978_u64,7665860138928018711_u64,15612250188458438861_u64,5602734767691128607_u64,12264987195888649673_u64,14686989279042897768_u64];
_24 = [3574822967314916953_u64,9711252442187052636_u64,2000922169828730876_u64,5526701573872213073_u64,10278202010242852700_u64,17313279227927251066_u64,9996472150881592397_u64,12443664075289620886_u64];
_4 = 93816506640606006135167404880779210085_u128 as isize;
_6 = !_2;
_13 = _22;
_31 = !5_usize;
_20 = _11;
_9.0 = _3 as isize;
_7 = (-1984056289_i32) - 1021994265_i32;
_28 = [_31,_31,_31,_31,_31,_31];
_31 = (-12660645451836742022526137613839628499_i128) as usize;
_9.2 = _12;
_27 = _9.1;
_1 = -_15;
Goto(bb9)
}
bb19 = {
_8 = 277609723502604012488218483246353292668_u128 as isize;
_12 = (*_19);
_24 = [6009723230567997386_u64,13293720205085113091_u64,1486892513826821996_u64,15876042355681130465_u64,9959328272573988064_u64,15246522764997301482_u64,12281862307405567288_u64,17439352165462817156_u64];
_5 = _15;
_20 = _11;
RET = [225742733172167047772442858448289684912_u128,2615224897539416991562153569505798009_u128];
_2 = _6;
_9.3 = _22 * _22;
RET = [105808497106531324434927589819603184045_u128,151507852787774263042442617329719824008_u128];
_12 = _20;
Goto(bb5)
}
bb20 = {
_7 = (-1224984383_i32);
_20 = _12;
_41.2.0 = 39_u8 as i32;
_1 = 12689891447335554309_u64 as isize;
_27 = [117_i8];
_14 = core::ptr::addr_of!(_29);
_30 = (-7024031722911295287_i64) - (-1139318153018978604_i64);
RET = [203627640825648385484673788176955829248_u128,292731293606533886199602580203971781926_u128];
_1 = !_4;
_41.2.1 = _11;
_14 = core::ptr::addr_of!((*_14));
_27 = [49_i8];
Goto(bb21)
}
bb21 = {
Call(_45 = dump_var(8_usize, 3_usize, Move(_3), 11_usize, Move(_11), 8_usize, Move(_8), 42_usize, Move(_42)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_45 = dump_var(8_usize, 12_usize, Move(_12), 17_usize, Move(_17), 6_usize, Move(_6), 18_usize, Move(_18)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_45 = dump_var(8_usize, 33_usize, Move(_33), 1_usize, Move(_1), 10_usize, Move(_10), 24_usize, Move(_24)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_45 = dump_var(8_usize, 27_usize, Move(_27), 46_usize, _46, 46_usize, _46, 46_usize, _46), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: u16,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: (isize, [i8; 1], char, f32),mut _8: bool,mut _9: isize,mut _10: (isize, [i8; 1], char, f32)) -> isize {
mir! {
type RET = isize;
let _11: *mut i8;
let _12: u16;
let _13: char;
let _14: u32;
let _15: u8;
let _16: [u8; 8];
let _17: isize;
let _18: isize;
let _19: *mut u8;
let _20: *const *mut [i128; 2];
let _21: bool;
let _22: ();
let _23: ();
{
RET = !_2;
_2 = _5;
RET = -_5;
RET = _6 - _4;
_7 = (_2, _10.1, _10.2, _10.3);
_4 = RET * _7.0;
_1 = 53193_u16 << _2;
_7.0 = _6 - _9;
_6 = _4;
_4 = !_10.0;
RET = _3 * _10.0;
_3 = _10.0 ^ _4;
_10 = (_9, _7.1, _7.2, _7.3);
_3 = 109_u8 as isize;
_1 = 7953_u16;
_7 = (_2, _10.1, _10.2, _10.3);
RET = (-23093_i16) as isize;
_7.3 = 8170601723216584579_usize as f32;
_7 = (_2, _10.1, _10.2, _10.3);
_8 = !false;
_7 = _10;
_7.1 = [17_i8];
_10.3 = _7.3;
match _1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
7953 => bb7,
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
_10.1 = [(-112_i8)];
_7.2 = _10.2;
Goto(bb8)
}
bb8 = {
_3 = _8 as isize;
_7 = _10;
_7.2 = _10.2;
_7.2 = _10.2;
_7 = (_10.0, _10.1, _10.2, _10.3);
_10.2 = _7.2;
_10.1 = [(-84_i8)];
_7 = (_4, _10.1, _10.2, _10.3);
Goto(bb9)
}
bb9 = {
_4 = 72_u8 as isize;
Call(_7.1 = fn10(_10, _7.0, _7.0, _2, _5, _10.0, _10.0, _6, _6, _7.0), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_8 = !false;
_5 = _6 - _10.0;
_2 = _8 as isize;
_14 = _9 as u32;
_16 = [34_u8,78_u8,243_u8,123_u8,139_u8,43_u8,247_u8,11_u8];
RET = -_5;
_7 = (_10.0, _10.1, _10.2, _10.3);
RET = _10.0;
RET = _10.0;
_10.1 = _7.1;
_10.2 = _7.2;
_17 = _6;
_10.1 = _7.1;
Goto(bb11)
}
bb11 = {
Call(_22 = dump_var(9_usize, 16_usize, Move(_16), 17_usize, Move(_17), 2_usize, Move(_2), 8_usize, Move(_8)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_22 = dump_var(9_usize, 3_usize, Move(_3), 23_usize, _23, 23_usize, _23, 23_usize, _23), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: (isize, [i8; 1], char, f32),mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize) -> [i8; 1] {
mir! {
type RET = [i8; 1];
let _11: bool;
let _12: isize;
let _13: f32;
let _14: isize;
let _15: i16;
let _16: bool;
let _17: bool;
let _18: Adt76;
let _19: &'static u32;
let _20: [u32; 4];
let _21: ((u128, f32, (i32, char)), i32);
let _22: bool;
let _23: f64;
let _24: (&'static u8, *const i8);
let _25: char;
let _26: &'static *mut *const &'static i32;
let _27: bool;
let _28: i16;
let _29: [i32; 4];
let _30: Adt79;
let _31: &'static [u64; 1];
let _32: Adt24;
let _33: *const *const &'static i32;
let _34: &'static *mut *const &'static i32;
let _35: ();
let _36: ();
{
_1.2 = '\u{e7e05}';
_5 = -_7;
_1.1 = [59_i8];
_1.0 = 40140038_i32 as isize;
_11 = true;
_1.2 = '\u{a5dc6}';
_8 = _10 - _7;
RET = [108_i8];
_1.1 = [96_i8];
_8 = _4;
_10 = _8 + _9;
_9 = (-1061920166_i32) as isize;
RET = [(-37_i8)];
_12 = _6 >> _6;
_10 = _6 ^ _12;
RET = [(-98_i8)];
Goto(bb1)
}
bb1 = {
_4 = 2923618194_u32 as isize;
_11 = _2 < _10;
_6 = -_8;
RET = [(-21_i8)];
_7 = -_8;
_3 = -_5;
_8 = -_7;
_4 = _1.2 as isize;
_6 = -_3;
_3 = _2 * _10;
_4 = -_2;
_16 = _11;
_1.2 = '\u{1f45d}';
_5 = _1.2 as isize;
_11 = _2 < _6;
_12 = _1.3 as isize;
_11 = !_16;
_10 = (-913118717_i32) as isize;
_1.2 = '\u{d95ac}';
_16 = _8 <= _4;
Goto(bb2)
}
bb2 = {
_18.fld1 = 9211241656987144136_u64;
_18.fld0 = [(-1354505493_i32),2113503608_i32,1071715223_i32,211694700_i32];
_6 = _1.2 as isize;
_5 = 584738931284853862_i64 as isize;
RET = [(-44_i8)];
_18.fld0 = [(-615965197_i32),(-815964922_i32),1931571801_i32,(-1678428751_i32)];
_18.fld1 = 10162861080857767072_u64 + 10070655354945657502_u64;
_2 = _3;
_13 = 5_usize as f32;
_13 = _1.3;
_17 = !_11;
_18.fld2 = Adt62::Variant3 { fld0: 24_i8,fld1: _7 };
_1.2 = '\u{15b0b}';
_1.2 = '\u{1008e9}';
_18.fld2 = Adt62::Variant3 { fld0: (-47_i8),fld1: _7 };
_14 = _2;
place!(Field::<i8>(Variant(_18.fld2, 3), 0)) = 1493924198_u32 as i8;
_16 = _11;
_1.2 = '\u{3542b}';
place!(Field::<i8>(Variant(_18.fld2, 3), 0)) = !(-83_i8);
_8 = _2 | _3;
_15 = (-17338_i16);
_18.fld1 = !16645819235892729397_u64;
_1.1 = RET;
place!(Field::<isize>(Variant(_18.fld2, 3), 1)) = _7;
_10 = _8 - _2;
Goto(bb3)
}
bb3 = {
_14 = -_7;
_5 = !_2;
_21.0.1 = _1.3 + _13;
_11 = _17;
_6 = _7 | _10;
_12 = -_7;
_15 = -1374_i16;
_17 = !_11;
_21.0.0 = 61886955593907966926126508918826445288_u128;
_21.0.2.0 = 707342831_i32 * 840654147_i32;
_21.1 = -_21.0.2.0;
RET = [Field::<i8>(Variant(_18.fld2, 3), 0)];
_10 = 46170_u16 as isize;
_18.fld1 = 12221577146745070701_u64;
_23 = _21.0.1 as f64;
_4 = _3;
RET = [Field::<i8>(Variant(_18.fld2, 3), 0)];
_1.2 = '\u{89f98}';
_6 = Field::<isize>(Variant(_18.fld2, 3), 1);
_22 = _11;
_20 = [26431802_u32,1913159026_u32,1478512953_u32,3543252833_u32];
_16 = _2 < Field::<isize>(Variant(_18.fld2, 3), 1);
_2 = Field::<i8>(Variant(_18.fld2, 3), 0) as isize;
_7 = -_3;
_21.0.1 = _13 + _1.3;
SetDiscriminant(_18.fld2, 1);
Goto(bb4)
}
bb4 = {
_12 = _5;
_9 = _8 * _4;
_21.0.1 = _1.3;
place!(Field::<[i64; 8]>(Variant(_18.fld2, 1), 2)) = [(-4057517057221288686_i64),(-8786150485973882246_i64),5890536492834136889_i64,(-3857116676930102140_i64),1179783304048535971_i64,7635973644106080051_i64,(-718743851752226447_i64),(-8072130955015487735_i64)];
_27 = !_22;
RET = [67_i8];
place!(Field::<[i64; 7]>(Variant(_18.fld2, 1), 0)) = [(-1055641618618371697_i64),7575842109173128079_i64,(-7639104326784034486_i64),(-3515452263066908472_i64),(-7640849151711997922_i64),(-7657932427642746495_i64),7290693031232943020_i64];
_21.0.2 = (_21.1, _1.2);
_21.0.0 = 176576052155010715424068496675548256124_u128 - 55357725807674731880517128613641884653_u128;
_18.fld1 = 15507559431403823114_u64;
_17 = _5 < _6;
_3 = -_6;
_11 = !_22;
Goto(bb5)
}
bb5 = {
RET = _1.1;
_29 = _18.fld0;
match _18.fld1 {
0 => bb1,
1 => bb4,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
15507559431403823114 => bb11,
_ => bb10
}
}
bb6 = {
_12 = _5;
_9 = _8 * _4;
_21.0.1 = _1.3;
place!(Field::<[i64; 8]>(Variant(_18.fld2, 1), 2)) = [(-4057517057221288686_i64),(-8786150485973882246_i64),5890536492834136889_i64,(-3857116676930102140_i64),1179783304048535971_i64,7635973644106080051_i64,(-718743851752226447_i64),(-8072130955015487735_i64)];
_27 = !_22;
RET = [67_i8];
place!(Field::<[i64; 7]>(Variant(_18.fld2, 1), 0)) = [(-1055641618618371697_i64),7575842109173128079_i64,(-7639104326784034486_i64),(-3515452263066908472_i64),(-7640849151711997922_i64),(-7657932427642746495_i64),7290693031232943020_i64];
_21.0.2 = (_21.1, _1.2);
_21.0.0 = 176576052155010715424068496675548256124_u128 - 55357725807674731880517128613641884653_u128;
_18.fld1 = 15507559431403823114_u64;
_17 = _5 < _6;
_3 = -_6;
_11 = !_22;
Goto(bb5)
}
bb7 = {
_14 = -_7;
_5 = !_2;
_21.0.1 = _1.3 + _13;
_11 = _17;
_6 = _7 | _10;
_12 = -_7;
_15 = -1374_i16;
_17 = !_11;
_21.0.0 = 61886955593907966926126508918826445288_u128;
_21.0.2.0 = 707342831_i32 * 840654147_i32;
_21.1 = -_21.0.2.0;
RET = [Field::<i8>(Variant(_18.fld2, 3), 0)];
_10 = 46170_u16 as isize;
_18.fld1 = 12221577146745070701_u64;
_23 = _21.0.1 as f64;
_4 = _3;
RET = [Field::<i8>(Variant(_18.fld2, 3), 0)];
_1.2 = '\u{89f98}';
_6 = Field::<isize>(Variant(_18.fld2, 3), 1);
_22 = _11;
_20 = [26431802_u32,1913159026_u32,1478512953_u32,3543252833_u32];
_16 = _2 < Field::<isize>(Variant(_18.fld2, 3), 1);
_2 = Field::<i8>(Variant(_18.fld2, 3), 0) as isize;
_7 = -_3;
_21.0.1 = _13 + _1.3;
SetDiscriminant(_18.fld2, 1);
Goto(bb4)
}
bb8 = {
_18.fld1 = 9211241656987144136_u64;
_18.fld0 = [(-1354505493_i32),2113503608_i32,1071715223_i32,211694700_i32];
_6 = _1.2 as isize;
_5 = 584738931284853862_i64 as isize;
RET = [(-44_i8)];
_18.fld0 = [(-615965197_i32),(-815964922_i32),1931571801_i32,(-1678428751_i32)];
_18.fld1 = 10162861080857767072_u64 + 10070655354945657502_u64;
_2 = _3;
_13 = 5_usize as f32;
_13 = _1.3;
_17 = !_11;
_18.fld2 = Adt62::Variant3 { fld0: 24_i8,fld1: _7 };
_1.2 = '\u{15b0b}';
_1.2 = '\u{1008e9}';
_18.fld2 = Adt62::Variant3 { fld0: (-47_i8),fld1: _7 };
_14 = _2;
place!(Field::<i8>(Variant(_18.fld2, 3), 0)) = 1493924198_u32 as i8;
_16 = _11;
_1.2 = '\u{3542b}';
place!(Field::<i8>(Variant(_18.fld2, 3), 0)) = !(-83_i8);
_8 = _2 | _3;
_15 = (-17338_i16);
_18.fld1 = !16645819235892729397_u64;
_1.1 = RET;
place!(Field::<isize>(Variant(_18.fld2, 3), 1)) = _7;
_10 = _8 - _2;
Goto(bb3)
}
bb9 = {
_4 = 2923618194_u32 as isize;
_11 = _2 < _10;
_6 = -_8;
RET = [(-21_i8)];
_7 = -_8;
_3 = -_5;
_8 = -_7;
_4 = _1.2 as isize;
_6 = -_3;
_3 = _2 * _10;
_4 = -_2;
_16 = _11;
_1.2 = '\u{1f45d}';
_5 = _1.2 as isize;
_11 = _2 < _6;
_12 = _1.3 as isize;
_11 = !_16;
_10 = (-913118717_i32) as isize;
_1.2 = '\u{d95ac}';
_16 = _8 <= _4;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
_2 = 52554_u16 as isize;
_15 = 30927_i16 >> _5;
_12 = _9;
_10 = _6;
place!(Field::<[u8; 8]>(Variant(_18.fld2, 1), 3)) = [164_u8,7_u8,106_u8,199_u8,93_u8,166_u8,192_u8,47_u8];
_23 = _13 as f64;
_29 = [_21.1,_21.1,_21.0.2.0,_21.1];
_30.fld0.2 = [_21.0.2.0,_21.1,_21.1,_21.0.2.0];
_2 = _5;
_9 = _14 ^ _7;
_18.fld2 = Adt62::Variant3 { fld0: 112_i8,fld1: _2 };
_1 = (_2, RET, _21.0.2.1, _21.0.1);
_1.2 = _21.0.2.1;
_3 = 1385625192_u32 as isize;
match _18.fld1 {
0 => bb10,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb9,
5 => bb6,
6 => bb7,
15507559431403823114 => bb12,
_ => bb8
}
}
bb12 = {
_12 = _27 as isize;
_6 = _18.fld1 as isize;
_14 = !_2;
_30.fld2.0 = [_21.1,_21.0.2.0,_21.1,_21.1];
_32.fld1 = [(-61250322464590552968833895601348980596_i128),(-34472766623806153853840382407257537952_i128)];
_10 = 126207037078841725896458925757264256865_i128 as isize;
_32.fld0 = _1.3 as u64;
_30.fld0.1 = _5;
_4 = !_12;
_17 = _27;
_13 = _21.0.1 - _1.3;
_17 = _11;
_1.2 = _21.0.2.1;
_18.fld2 = Adt62::Variant3 { fld0: 13_i8,fld1: _14 };
_30.fld0 = (_23, _9, _30.fld2.0);
_28 = _21.0.2.1 as i16;
match _18.fld1 {
15507559431403823114 => bb14,
_ => bb13
}
}
bb13 = {
_18.fld1 = 9211241656987144136_u64;
_18.fld0 = [(-1354505493_i32),2113503608_i32,1071715223_i32,211694700_i32];
_6 = _1.2 as isize;
_5 = 584738931284853862_i64 as isize;
RET = [(-44_i8)];
_18.fld0 = [(-615965197_i32),(-815964922_i32),1931571801_i32,(-1678428751_i32)];
_18.fld1 = 10162861080857767072_u64 + 10070655354945657502_u64;
_2 = _3;
_13 = 5_usize as f32;
_13 = _1.3;
_17 = !_11;
_18.fld2 = Adt62::Variant3 { fld0: 24_i8,fld1: _7 };
_1.2 = '\u{15b0b}';
_1.2 = '\u{1008e9}';
_18.fld2 = Adt62::Variant3 { fld0: (-47_i8),fld1: _7 };
_14 = _2;
place!(Field::<i8>(Variant(_18.fld2, 3), 0)) = 1493924198_u32 as i8;
_16 = _11;
_1.2 = '\u{3542b}';
place!(Field::<i8>(Variant(_18.fld2, 3), 0)) = !(-83_i8);
_8 = _2 | _3;
_15 = (-17338_i16);
_18.fld1 = !16645819235892729397_u64;
_1.1 = RET;
place!(Field::<isize>(Variant(_18.fld2, 3), 1)) = _7;
_10 = _8 - _2;
Goto(bb3)
}
bb14 = {
_21.0.1 = -_13;
_6 = !_5;
_30.fld0.1 = -_1.0;
_1.1 = [(-102_i8)];
_20 = [3925009379_u32,337243448_u32,3006951039_u32,2781159790_u32];
_27 = _11;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(10_usize, 16_usize, Move(_16), 28_usize, Move(_28), 11_usize, Move(_11), 12_usize, Move(_12)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(10_usize, 2_usize, Move(_2), 20_usize, Move(_20), 3_usize, Move(_3), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(10_usize, 5_usize, Move(_5), 9_usize, Move(_9), 36_usize, _36, 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize) -> (isize, [i8; 1], char, f32) {
mir! {
type RET = (isize, [i8; 1], char, f32);
let _5: [u32; 4];
let _6: isize;
let _7: i16;
let _8: i32;
let _9: ();
let _10: ();
{
RET.3 = 12680492785753069600_u64 as f32;
RET.3 = 291426908190638629428536309231310957746_u128 as f32;
RET.2 = '\u{b6d10}';
_3 = !_2;
RET.3 = 3927670714_u32 as f32;
RET.0 = _1;
RET.2 = '\u{d938c}';
_4 = _1 << _2;
_2 = _1;
_5 = [3694847316_u32,2758623318_u32,1832873929_u32,3424089496_u32];
_5 = [4252106957_u32,3323520133_u32,1895456912_u32,1634772631_u32];
RET.1 = [(-43_i8)];
_7 = 15223_i16 & (-28744_i16);
RET.1 = [(-7_i8)];
_3 = RET.0 - _2;
_3 = _2 + _1;
_1 = RET.0;
_6 = 17104878360888486281_usize as isize;
RET.3 = (-64366143897685252370124484800486806833_i128) as f32;
RET.3 = 16023_u16 as f32;
_4 = _3;
RET.1 = [(-36_i8)];
Goto(bb1)
}
bb1 = {
Call(_9 = dump_var(11_usize, 7_usize, Move(_7), 6_usize, Move(_6), 5_usize, Move(_5), 10_usize, _10), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: (isize, [i8; 1], char, f32),mut _2: isize,mut _3: isize,mut _4: isize) -> *const &'static i32 {
mir! {
type RET = *const &'static i32;
let _5: i32;
let _6: i32;
let _7: ((u128, f32, (i32, char)), i32);
let _8: bool;
let _9: ([i128; 2], u16, &'static char, &'static *mut *const &'static i32);
let _10: *mut (u128, f32, (i32, char));
let _11: char;
let _12: char;
let _13: usize;
let _14: Adt76;
let _15: (f64, isize, [i32; 4]);
let _16: bool;
let _17: i64;
let _18: i32;
let _19: bool;
let _20: [i8; 5];
let _21: isize;
let _22: *const *mut [i128; 2];
let _23: i128;
let _24: isize;
let _25: ((isize, [i8; 1], char, f32), *mut ([i8; 1], [i64; 7], f32, *mut *mut (u128, f32, (i32, char))), [u64; 3], &'static i32);
let _26: &'static *mut *const &'static i32;
let _27: isize;
let _28: *mut &'static i16;
let _29: f32;
let _30: Adt79;
let _31: ();
let _32: ();
{
_1.3 = (-1736986960838148985_i64) as f32;
_1.3 = 162298354684965413676626702599144510368_i128 as f32;
Goto(bb1)
}
bb1 = {
_4 = _1.0 >> _3;
_4 = 18136123080449772078_u64 as isize;
_1.0 = _3 - _3;
_2 = !_1.0;
_5 = 6_usize as i32;
_3 = _1.0;
_1.3 = 6996852865177436886_i64 as f32;
_1.0 = _2 + _3;
_1.1 = [99_i8];
_5 = (-1742807779_i32);
_5 = (-2892293816667070811_i64) as i32;
_4 = -_1.0;
_7.0.2.1 = _1.2;
_1.0 = -_4;
_3 = _2;
_7.0.2.0 = 6_usize as i32;
_2 = _4;
_1.0 = !_4;
Call(_7.1 = fn13(_2, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1.0 = _1.3 as isize;
_7.0.2 = (_7.1, _1.2);
_9.1 = 13700_u16;
_1.0 = _3 | _3;
_3 = _1.3 as isize;
_7.0.1 = 474049246783316386916273806693249252_i128 as f32;
_7.0.1 = _1.3;
_9.2 = &_7.0.2.1;
_11 = _1.2;
_9.0 = [(-19567251345756040151994892861685834427_i128),35937405364090390482620580544721922783_i128];
_7.0.2.1 = _1.2;
_12 = _1.2;
_12 = _7.0.2.1;
_9.1 = 16531_u16 << _2;
_9.0 = [(-106891606842828150439330007881244849120_i128),23066437261160285531445697089916146942_i128];
_7.0.1 = _1.3 * _1.3;
_6 = _9.1 as i32;
_9.1 = 1095_u16;
_4 = !_1.0;
match _9.1 {
0 => bb1,
1 => bb3,
1095 => bb5,
_ => bb4
}
}
bb3 = {
_4 = _1.0 >> _3;
_4 = 18136123080449772078_u64 as isize;
_1.0 = _3 - _3;
_2 = !_1.0;
_5 = 6_usize as i32;
_3 = _1.0;
_1.3 = 6996852865177436886_i64 as f32;
_1.0 = _2 + _3;
_1.1 = [99_i8];
_5 = (-1742807779_i32);
_5 = (-2892293816667070811_i64) as i32;
_4 = -_1.0;
_7.0.2.1 = _1.2;
_1.0 = -_4;
_3 = _2;
_7.0.2.0 = 6_usize as i32;
_2 = _4;
_1.0 = !_4;
Call(_7.1 = fn13(_2, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
_9.2 = &_1.2;
_7.0.0 = 121706110048752790162887584861735519255_u128;
_12 = _1.2;
_7.0.2.1 = _11;
_8 = _6 != _6;
Goto(bb6)
}
bb6 = {
_7.0.0 = 212504459655338380371437582791666870223_u128 << _4;
_1.3 = _7.0.1 - _7.0.1;
_13 = 7_usize;
_7.0.1 = _1.3;
_9.1 = !20128_u16;
_7.0.2 = (_6, _12);
_7.0.1 = -_1.3;
_7.0.1 = -_1.3;
_9.1 = !22902_u16;
_10 = core::ptr::addr_of_mut!(_7.0);
_6 = (*_10).2.0 & (*_10).2.0;
_12 = _1.2;
_1.0 = !_4;
_13 = 3057538730851308521_usize - 5_usize;
_9.0 = [63703523473724061854860470697628286744_i128,(-3695041041925701313713950207842627057_i128)];
_2 = !_1.0;
(*_10).0 = 226742798750237893693846056106755398912_u128 - 208315231489749248236546719863076200520_u128;
(*_10).1 = 244_u8 as f32;
(*_10).2.1 = _12;
_3 = _2;
(*_10).2.0 = 6493490513844073387_u64 as i32;
_14.fld0 = [_6,_6,_6,_6];
_3 = _4;
(*_10).1 = _1.3;
(*_10).2.1 = _12;
Goto(bb7)
}
bb7 = {
_7.0.0 = 282329494098375094373671777287786430304_u128 >> _4;
Call(_1.1 = fn16(Move(_9.2), _6, (*_10).0, _7.0, _7.0, _6, _7.0.0, _2, _2), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_15.0 = _7.0.0 as f64;
_14.fld1 = !8696741134917635749_u64;
(*_10).1 = -_1.3;
_1.1 = [(-80_i8)];
_14.fld1 = (*_10).1 as u64;
_7.0.0 = !112562618803684695224696391744331899613_u128;
_9.2 = &_11;
(*_10).2 = (_6, _11);
_16 = !_8;
_1.2 = _12;
_1.2 = (*_10).2.1;
(*_10).2.0 = _6;
_7.0.0 = 80214907748032475191952168859930774737_u128 | 326817394246591614475891231123966825386_u128;
_18 = -(*_10).2.0;
_9.2 = &_12;
Goto(bb9)
}
bb9 = {
_14.fld2 = Adt62::Variant3 { fld0: 126_i8,fld1: _3 };
place!(Field::<i8>(Variant(_14.fld2, 3), 0)) = !12_i8;
(*_10).2.0 = _6;
_2 = (*_10).1 as isize;
SetDiscriminant(_14.fld2, 1);
_11 = _7.0.2.1;
_4 = _3;
(*_10).0 = 139658393715126632937720980562737769715_u128 + 109575792724933199352840194771084595124_u128;
_14.fld1 = _9.1 as u64;
_7.0.2.1 = _11;
_17 = (-4704698181636354664_i64);
_10 = core::ptr::addr_of_mut!(_7.0);
_1.2 = _12;
(*_10).2.0 = -_18;
_7.0.2.1 = _12;
match _17 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
340282366920938463458669909250131856792 => bb18,
_ => bb17
}
}
bb10 = {
_15.0 = _7.0.0 as f64;
_14.fld1 = !8696741134917635749_u64;
(*_10).1 = -_1.3;
_1.1 = [(-80_i8)];
_14.fld1 = (*_10).1 as u64;
_7.0.0 = !112562618803684695224696391744331899613_u128;
_9.2 = &_11;
(*_10).2 = (_6, _11);
_16 = !_8;
_1.2 = _12;
_1.2 = (*_10).2.1;
(*_10).2.0 = _6;
_7.0.0 = 80214907748032475191952168859930774737_u128 | 326817394246591614475891231123966825386_u128;
_18 = -(*_10).2.0;
_9.2 = &_12;
Goto(bb9)
}
bb11 = {
_7.0.0 = 282329494098375094373671777287786430304_u128 >> _4;
Call(_1.1 = fn16(Move(_9.2), _6, (*_10).0, _7.0, _7.0, _6, _7.0.0, _2, _2), ReturnTo(bb8), UnwindUnreachable())
}
bb12 = {
_7.0.0 = 212504459655338380371437582791666870223_u128 << _4;
_1.3 = _7.0.1 - _7.0.1;
_13 = 7_usize;
_7.0.1 = _1.3;
_9.1 = !20128_u16;
_7.0.2 = (_6, _12);
_7.0.1 = -_1.3;
_7.0.1 = -_1.3;
_9.1 = !22902_u16;
_10 = core::ptr::addr_of_mut!(_7.0);
_6 = (*_10).2.0 & (*_10).2.0;
_12 = _1.2;
_1.0 = !_4;
_13 = 3057538730851308521_usize - 5_usize;
_9.0 = [63703523473724061854860470697628286744_i128,(-3695041041925701313713950207842627057_i128)];
_2 = !_1.0;
(*_10).0 = 226742798750237893693846056106755398912_u128 - 208315231489749248236546719863076200520_u128;
(*_10).1 = 244_u8 as f32;
(*_10).2.1 = _12;
_3 = _2;
(*_10).2.0 = 6493490513844073387_u64 as i32;
_14.fld0 = [_6,_6,_6,_6];
_3 = _4;
(*_10).1 = _1.3;
(*_10).2.1 = _12;
Goto(bb7)
}
bb13 = {
_9.2 = &_1.2;
_7.0.0 = 121706110048752790162887584861735519255_u128;
_12 = _1.2;
_7.0.2.1 = _11;
_8 = _6 != _6;
Goto(bb6)
}
bb14 = {
Return()
}
bb15 = {
_4 = _1.0 >> _3;
_4 = 18136123080449772078_u64 as isize;
_1.0 = _3 - _3;
_2 = !_1.0;
_5 = 6_usize as i32;
_3 = _1.0;
_1.3 = 6996852865177436886_i64 as f32;
_1.0 = _2 + _3;
_1.1 = [99_i8];
_5 = (-1742807779_i32);
_5 = (-2892293816667070811_i64) as i32;
_4 = -_1.0;
_7.0.2.1 = _1.2;
_1.0 = -_4;
_3 = _2;
_7.0.2.0 = 6_usize as i32;
_2 = _4;
_1.0 = !_4;
Call(_7.1 = fn13(_2, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb16 = {
_1.0 = _1.3 as isize;
_7.0.2 = (_7.1, _1.2);
_9.1 = 13700_u16;
_1.0 = _3 | _3;
_3 = _1.3 as isize;
_7.0.1 = 474049246783316386916273806693249252_i128 as f32;
_7.0.1 = _1.3;
_9.2 = &_7.0.2.1;
_11 = _1.2;
_9.0 = [(-19567251345756040151994892861685834427_i128),35937405364090390482620580544721922783_i128];
_7.0.2.1 = _1.2;
_12 = _1.2;
_12 = _7.0.2.1;
_9.1 = 16531_u16 << _2;
_9.0 = [(-106891606842828150439330007881244849120_i128),23066437261160285531445697089916146942_i128];
_7.0.1 = _1.3 * _1.3;
_6 = _9.1 as i32;
_9.1 = 1095_u16;
_4 = !_1.0;
match _9.1 {
0 => bb1,
1 => bb3,
1095 => bb5,
_ => bb4
}
}
bb17 = {
_4 = _1.0 >> _3;
_4 = 18136123080449772078_u64 as isize;
_1.0 = _3 - _3;
_2 = !_1.0;
_5 = 6_usize as i32;
_3 = _1.0;
_1.3 = 6996852865177436886_i64 as f32;
_1.0 = _2 + _3;
_1.1 = [99_i8];
_5 = (-1742807779_i32);
_5 = (-2892293816667070811_i64) as i32;
_4 = -_1.0;
_7.0.2.1 = _1.2;
_1.0 = -_4;
_3 = _2;
_7.0.2.0 = 6_usize as i32;
_2 = _4;
_1.0 = !_4;
Call(_7.1 = fn13(_2, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
(*_10).2.1 = _1.2;
_10 = core::ptr::addr_of_mut!(_7.0);
Call(_15.0 = core::intrinsics::transmute(_4), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
_7.0.2.1 = _1.2;
_9.1 = !64420_u16;
(*_10).2 = (_18, _11);
_25.0.0 = _1.0;
_15.2 = _14.fld0;
_4 = _3;
(*_10).1 = -_1.3;
_7.0.0 = 193621195438566593396012021048310131283_u128 | 268650597081371730746736619754956508199_u128;
(*_10).1 = (-69_i8) as f32;
_6 = _9.1 as i32;
_4 = _15.0 as isize;
(*_10).2.0 = _18 << _18;
_9.2 = &_25.0.2;
RET = core::ptr::addr_of!(_25.3);
(*RET) = &_5;
_24 = !_3;
_27 = -_4;
_2 = _16 as isize;
_7.0.2.1 = _1.2;
_29 = _1.3;
(*_10).1 = 181_u8 as f32;
Goto(bb20)
}
bb20 = {
Call(_31 = dump_var(12_usize, 13_usize, Move(_13), 8_usize, Move(_8), 5_usize, Move(_5), 27_usize, Move(_27)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_31 = dump_var(12_usize, 16_usize, Move(_16), 2_usize, Move(_2), 3_usize, Move(_3), 32_usize, _32), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: isize,mut _2: isize) -> i32 {
mir! {
type RET = i32;
let _3: f32;
let _4: [i64; 2];
let _5: isize;
let _6: u8;
let _7: i8;
let _8: [u64; 3];
let _9: char;
let _10: char;
let _11: &'static *mut *const &'static i32;
let _12: bool;
let _13: Adt17;
let _14: u128;
let _15: f64;
let _16: u32;
let _17: &'static u8;
let _18: ([i8; 1], [i64; 7], f32, *mut *mut (u128, f32, (i32, char)));
let _19: (f64, isize, [i32; 4]);
let _20: Adt76;
let _21: f32;
let _22: [u32; 4];
let _23: isize;
let _24: &'static char;
let _25: isize;
let _26: Adt62;
let _27: isize;
let _28: [i8; 5];
let _29: [i64; 8];
let _30: u16;
let _31: u16;
let _32: i32;
let _33: ();
let _34: ();
{
RET = (-1217731088_i32);
_2 = _1 | _1;
_1 = -_2;
_2 = _1;
_1 = _2 >> _2;
_5 = (-2534172158568326749_i64) as isize;
_1 = _5;
_3 = 48025_u16 as f32;
_5 = true as isize;
RET = -(-354917496_i32);
RET = 8627061747358778746_u64 as i32;
_4 = [1780118316649696953_i64,(-6346674091727034331_i64)];
_2 = _5;
_1 = _5;
_2 = 3612443126_u32 as isize;
_4 = [(-4843623132783681537_i64),1090397378227107210_i64];
_5 = _2;
_5 = 331872585189152534409693131110745669475_u128 as isize;
_1 = RET as isize;
_3 = 16_u8 as f32;
_2 = _5 << RET;
_1 = RET as isize;
Goto(bb1)
}
bb1 = {
_6 = 189_u8;
_4 = [4578273536424888609_i64,(-8067602453764000983_i64)];
_1 = 3289614410_u32 as isize;
_4 = [(-5195610502526621285_i64),8337423310069351160_i64];
_1 = 7273_u16 as isize;
_5 = !_1;
_5 = _6 as isize;
_3 = (-81005865827494820293253015781040834446_i128) as f32;
_8 = [12113745443171489893_u64,2625252014661524693_u64,1408678514204528280_u64];
_3 = RET as f32;
Call(_5 = core::intrinsics::transmute(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_2 = 51528_u16 as isize;
_6 = !2_u8;
RET = 48363888134963194795210522747070161998_u128 as i32;
_7 = -39_i8;
_2 = _5 | _1;
_1 = 4495270206112854967_usize as isize;
_8 = [4629210349735276426_u64,4636824341848731025_u64,13676764215789330602_u64];
_2 = _5 << RET;
_6 = !79_u8;
_6 = 50_u8 + 24_u8;
Call(_1 = fn14(_8, _8, _2, _7, _4, _2, _2, _3, _6, RET, _2, _6), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = !143_u8;
_9 = '\u{d1756}';
_1 = _2;
RET = !(-662167917_i32);
_9 = '\u{f28f0}';
RET = 1959911263_i32;
_10 = _9;
_8 = [3416700651949661378_u64,11437554808333311186_u64,9292297986363373865_u64];
_5 = !_1;
_10 = _9;
RET = (-1795583340_i32);
_9 = _10;
_4 = [(-3734723460070442183_i64),4584038511013293602_i64];
_6 = !191_u8;
_4 = [5992547694236376155_i64,(-5953792637233262117_i64)];
_9 = _10;
_4 = [5309619880679330787_i64,(-8231493455494164000_i64)];
_8 = [11753397952117818043_u64,6374218496183133845_u64,18052131500538751642_u64];
RET = (-4162660_i32) << _2;
_3 = 3817154273590627643_i64 as f32;
_2 = 26738054682445549389598822897209098929_i128 as isize;
_9 = _10;
_3 = 34298_u16 as f32;
_9 = _10;
_10 = _9;
Goto(bb4)
}
bb4 = {
_8 = [13586919174720416140_u64,3696816281350363909_u64,6158555965459683377_u64];
_4 = [(-3301722842324724763_i64),5517638898650310559_i64];
_1 = _2 >> _5;
_2 = _1 | _5;
_1 = _5;
_9 = _10;
_1 = !_2;
_10 = _9;
_3 = 18979870221722459991097246190955530533_i128 as f32;
_1 = 9089268660360345473_i64 as isize;
_6 = 85_u8 | 113_u8;
_7 = -16_i8;
_8 = [15055627301696293346_u64,17637838346581221668_u64,4262429270782663120_u64];
_1 = _2;
_6 = 197_u8 * 133_u8;
Goto(bb5)
}
bb5 = {
_9 = _10;
_8 = [11851356237343391075_u64,10006532124538946613_u64,6006616431150774626_u64];
_9 = _10;
_6 = 48249_u16 as u8;
_8 = [5817249670426763302_u64,4450084837570470555_u64,608240031570164252_u64];
_13.fld1 = 2305_u16;
_14 = 3_usize as u128;
_5 = _1;
_13.fld3 = _1 as u8;
_13 = Adt17 { fld0: 20491_i16,fld1: 44745_u16,fld2: RET,fld3: _6 };
_1 = !_2;
_13.fld0 = 17655_i16;
RET = _13.fld2;
_13.fld0 = (-3460_i16);
_13.fld1 = 21910_u16 * 8628_u16;
_9 = _10;
_13.fld3 = _6 << _5;
RET = _13.fld2;
_5 = _13.fld3 as isize;
_14 = !60248271264801323141041578540824322051_u128;
Goto(bb6)
}
bb6 = {
_12 = _9 > _10;
_1 = _5;
_10 = _9;
_13.fld0 = -29324_i16;
_16 = 3143713320_u32 >> _2;
_6 = _13.fld3;
_13.fld1 = (-120555343723939149536434579170928808402_i128) as u16;
_10 = _9;
_7 = 39_i8;
RET = !_13.fld2;
_13.fld2 = !RET;
Goto(bb7)
}
bb7 = {
_17 = &_13.fld3;
_6 = (*_17);
_13.fld1 = 3976328995291354856_i64 as u16;
_13 = Adt17 { fld0: (-12972_i16),fld1: 41434_u16,fld2: RET,fld3: _6 };
RET = !_13.fld2;
_18.0 = [_7];
_18.1 = [2532080075589715271_i64,(-3326422233549777499_i64),(-5533063214512827019_i64),(-2402818537055147762_i64),(-4154806004881839066_i64),6267169137105202452_i64,5019069317848436838_i64];
_16 = !2950698705_u32;
_13.fld0 = _2 as i16;
_20.fld0 = [RET,RET,RET,RET];
_13.fld1 = !28341_u16;
_21 = _3 + _3;
_19.2 = [_13.fld2,_13.fld2,RET,RET];
_18.2 = -_3;
_19.0 = _13.fld1 as f64;
_15 = _19.0 - _19.0;
Call(_13.fld1 = core::intrinsics::bswap(42291_u16), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_2 = !_1;
_17 = &_6;
_4 = [(-780594524223958604_i64),3969332504812989957_i64];
_1 = -_2;
_17 = &(*_17);
_17 = &(*_17);
_4 = [3475734037300330963_i64,801004926154785784_i64];
_19.1 = _2;
_16 = 3657471703_u32 | 2141719111_u32;
_17 = &(*_17);
_20.fld1 = _13.fld0 as u64;
_19 = (_15, _5, _20.fld0);
_19.2 = [RET,RET,RET,RET];
_22 = [_16,_16,_16,_16];
_14 = !317523061617546705335922560197333638023_u128;
_16 = 128933878418423006555835785709941389671_i128 as u32;
_6 = _13.fld3;
_2 = _5;
_5 = _1;
_17 = &_6;
_12 = _20.fld1 != _20.fld1;
_21 = -_3;
Goto(bb9)
}
bb9 = {
_19.2 = [_13.fld2,RET,_13.fld2,_13.fld2];
_20.fld2 = Adt62::Variant3 { fld0: _7,fld1: _1 };
_7 = Field::<i8>(Variant(_20.fld2, 3), 0);
RET = _13.fld2 ^ _13.fld2;
_19.0 = -_15;
_20.fld1 = _9 as u64;
_6 = _13.fld3;
_16 = 4262871002_u32 + 198084592_u32;
_21 = _18.2 * _3;
_13.fld2 = !RET;
_23 = _5;
SetDiscriminant(_20.fld2, 3);
_18.1 = [5153975863165317412_i64,930602570504786442_i64,2531132767134506211_i64,987305489008581757_i64,5042248585250540649_i64,783076027134606142_i64,(-1061432834320982532_i64)];
match _7 {
39 => bb10,
_ => bb7
}
}
bb10 = {
_20.fld0 = _19.2;
_26 = Adt62::Variant3 { fld0: _7,fld1: _5 };
_24 = &_10;
place!(Field::<isize>(Variant(_20.fld2, 3), 1)) = _1 + _23;
_8 = [_20.fld1,_20.fld1,_20.fld1];
_17 = &_13.fld3;
_17 = &(*_17);
_20 = Adt76 { fld0: _19.2,fld1: 8321719659628728542_u64,fld2: Move(_26) };
_25 = -_19.1;
_13.fld0 = _13.fld1 as i16;
_18.1 = [4500245856622534868_i64,4981085934319810070_i64,159992549318940877_i64,1783854551899996348_i64,(-5056908201350307696_i64),(-3822914920430698282_i64),(-4271879663645340684_i64)];
_8 = [_20.fld1,_20.fld1,_20.fld1];
_10 = _9;
SetDiscriminant(_20.fld2, 2);
_2 = _25 ^ _5;
_16 = 462728664_u32 * 2462454443_u32;
_18.0 = [_7];
Call(place!(Field::<([i8; 1], [i64; 7], f32, *mut *mut (u128, f32, (i32, char)))>(Variant(_20.fld2, 2), 0)).1 = core::intrinsics::transmute(_18.1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_7 = 30_i8;
_28 = [_7,_7,_7,_7,_7];
_7 = 99_i8 & 69_i8;
place!(Field::<[u64; 3]>(Variant(_20.fld2, 2), 2)) = [_20.fld1,_20.fld1,_20.fld1];
match _20.fld1 {
0 => bb1,
1 => bb3,
2 => bb12,
8321719659628728542 => bb14,
_ => bb13
}
}
bb12 = {
_20.fld0 = _19.2;
_26 = Adt62::Variant3 { fld0: _7,fld1: _5 };
_24 = &_10;
place!(Field::<isize>(Variant(_20.fld2, 3), 1)) = _1 + _23;
_8 = [_20.fld1,_20.fld1,_20.fld1];
_17 = &_13.fld3;
_17 = &(*_17);
_20 = Adt76 { fld0: _19.2,fld1: 8321719659628728542_u64,fld2: Move(_26) };
_25 = -_19.1;
_13.fld0 = _13.fld1 as i16;
_18.1 = [4500245856622534868_i64,4981085934319810070_i64,159992549318940877_i64,1783854551899996348_i64,(-5056908201350307696_i64),(-3822914920430698282_i64),(-4271879663645340684_i64)];
_8 = [_20.fld1,_20.fld1,_20.fld1];
_10 = _9;
SetDiscriminant(_20.fld2, 2);
_2 = _25 ^ _5;
_16 = 462728664_u32 * 2462454443_u32;
_18.0 = [_7];
Call(place!(Field::<([i8; 1], [i64; 7], f32, *mut *mut (u128, f32, (i32, char)))>(Variant(_20.fld2, 2), 0)).1 = core::intrinsics::transmute(_18.1), ReturnTo(bb11), UnwindUnreachable())
}
bb13 = {
_6 = 189_u8;
_4 = [4578273536424888609_i64,(-8067602453764000983_i64)];
_1 = 3289614410_u32 as isize;
_4 = [(-5195610502526621285_i64),8337423310069351160_i64];
_1 = 7273_u16 as isize;
_5 = !_1;
_5 = _6 as isize;
_3 = (-81005865827494820293253015781040834446_i128) as f32;
_8 = [12113745443171489893_u64,2625252014661524693_u64,1408678514204528280_u64];
_3 = RET as f32;
Call(_5 = core::intrinsics::transmute(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb14 = {
place!(Field::<[u64; 3]>(Variant(_20.fld2, 2), 2)) = [_20.fld1,_20.fld1,_20.fld1];
_13.fld2 = RET ^ RET;
_17 = &_13.fld3;
_28 = [_7,_7,_7,_7,_7];
_19 = (_15, _23, _20.fld0);
_8 = Field::<[u64; 3]>(Variant(_20.fld2, 2), 2);
_20.fld2 = Adt62::Variant3 { fld0: _7,fld1: _1 };
_28 = [_7,_7,_7,Field::<i8>(Variant(_20.fld2, 3), 0),Field::<i8>(Variant(_20.fld2, 3), 0)];
_19.2 = _20.fld0;
_19 = (_15, _5, _20.fld0);
_24 = &_9;
_20.fld0 = _19.2;
_27 = _2 << _25;
_23 = _27;
_7 = Field::<i8>(Variant(_20.fld2, 3), 0);
_21 = _18.2;
_19 = (_15, _27, _20.fld0);
_6 = _3 as u8;
_1 = _19.1 | _27;
_32 = Field::<i8>(Variant(_20.fld2, 3), 0) as i32;
_20.fld1 = 9568424224019571304_u64;
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(13_usize, 10_usize, Move(_10), 16_usize, Move(_16), 22_usize, Move(_22), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(13_usize, 4_usize, Move(_4), 32_usize, Move(_32), 25_usize, Move(_25), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(13_usize, 5_usize, Move(_5), 34_usize, _34, 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: [u64; 3],mut _2: [u64; 3],mut _3: isize,mut _4: i8,mut _5: [i64; 2],mut _6: isize,mut _7: isize,mut _8: f32,mut _9: u8,mut _10: i32,mut _11: isize,mut _12: u8) -> isize {
mir! {
type RET = isize;
let _13: &'static [i8; 2];
let _14: *const *mut [i128; 2];
let _15: isize;
let _16: i16;
let _17: f32;
let _18: f64;
let _19: i32;
let _20: isize;
let _21: i32;
let _22: *mut isize;
let _23: &'static [i8; 2];
let _24: Adt17;
let _25: *mut *const *const &'static i32;
let _26: ();
let _27: ();
{
_6 = _3 & _11;
_1 = [10951907983232794600_u64,15364314818011271924_u64,15380204605074444196_u64];
_12 = _9 | _9;
_4 = 104_i8 * (-124_i8);
RET = (-2855001169947559080_i64) as isize;
_15 = _11;
RET = -_3;
RET = !_15;
_4 = _9 as i8;
_2 = _1;
_7 = -RET;
_4 = 15_i8 * 79_i8;
_15 = -_6;
_1 = [15237493889391139510_u64,7314895280001836966_u64,9535680536626258040_u64];
Goto(bb1)
}
bb1 = {
_5 = [(-4302464651059608541_i64),(-8216734183709063722_i64)];
_3 = !_7;
_8 = _10 as f32;
_2 = [10916576195505849516_u64,3078825363853686359_u64,12695129433779816636_u64];
_17 = _8;
_3 = !_7;
_18 = _17 as f64;
_1 = [8970354165062274411_u64,14792421234850542892_u64,4955580439417583348_u64];
_5 = [1399286452826679123_i64,(-1864835243559350062_i64)];
_5 = [(-6533990469710228390_i64),(-6007428534307226633_i64)];
_16 = 19035_i16;
_10 = !(-685743449_i32);
_10 = 1316730373_i32 & 1182824857_i32;
_18 = 298192527015529828897325004674960231959_u128 as f64;
_4 = 1605419003_u32 as i8;
_9 = _12 - _12;
_8 = -_17;
_9 = _4 as u8;
_19 = _12 as i32;
_12 = _9;
RET = !_15;
_15 = -_6;
RET = 108889371323526813844707104882107181160_i128 as isize;
_5 = [4319768973747540039_i64,(-4753112111803474576_i64)];
_9 = _12 & _12;
Call(_11 = core::intrinsics::transmute(_7), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_8 = _17 * _17;
_6 = _11;
_18 = _16 as f64;
_6 = _15 >> _19;
_15 = _6;
_19 = _10 << _15;
Call(RET = fn15(), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_6 = _16 as isize;
_1 = [5089369910331483414_u64,13083774790576062463_u64,5874351649853984734_u64];
_9 = _12 + _12;
_3 = _11;
_19 = _10 >> _15;
_22 = core::ptr::addr_of_mut!(_20);
_21 = !_19;
_3 = _19 as isize;
(*_22) = !_15;
_20 = _18 as isize;
_1 = _2;
_17 = -_8;
_11 = _3 | _3;
_18 = 2729097725815917774_i64 as f64;
_17 = _8 * _8;
_2 = [15234614891944610852_u64,11821720056745252336_u64,11297073549708305286_u64];
RET = _16 as isize;
RET = _20 | _11;
_2 = [7593082147435176333_u64,13237328866720834840_u64,6809156795467101680_u64];
_24.fld3 = _12;
RET = (*_22) * _11;
_20 = -RET;
_24.fld0 = _11 as i16;
_19 = _21;
_6 = _11 | _15;
_22 = core::ptr::addr_of_mut!(_3);
(*_22) = -RET;
Goto(bb4)
}
bb4 = {
Call(_26 = dump_var(14_usize, 20_usize, Move(_20), 9_usize, Move(_9), 5_usize, Move(_5), 19_usize, Move(_19)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_26 = dump_var(14_usize, 6_usize, Move(_6), 11_usize, Move(_11), 10_usize, Move(_10), 4_usize, Move(_4)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn15() -> isize {
mir! {
type RET = isize;
let _1: ([i32; 4],);
let _2: Adt17;
let _3: u128;
let _4: f32;
let _5: (i64,);
let _6: bool;
let _7: *mut *const *const &'static i32;
let _8: (i32, char);
let _9: isize;
let _10: i64;
let _11: *mut isize;
let _12: (&'static u8, *const i8);
let _13: isize;
let _14: &'static u128;
let _15: ();
let _16: ();
{
RET = 74_isize;
RET = 25207_i16 as isize;
RET = 9223372036854775807_isize >> 167392438480614154361469827685852700053_u128;
RET = (-9223372036854775808_isize);
RET = 9223372036854775807_isize;
RET = -(-94_isize);
_1.0 = [(-1859870720_i32),1251068170_i32,1128277843_i32,(-1858045033_i32)];
RET = 9223372036854775807_isize - 9223372036854775807_isize;
_3 = !15445251033596297012783196315992638705_u128;
_2 = Adt17 { fld0: 27082_i16,fld1: 49491_u16,fld2: 528861582_i32,fld3: 178_u8 };
_2 = Adt17 { fld0: (-32558_i16),fld1: 47516_u16,fld2: (-2070321940_i32),fld3: 208_u8 };
_2.fld0 = 73_i8 as i16;
Call(_2.fld0 = core::intrinsics::transmute(_2.fld1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2.fld2 = 1191421950_i32 & (-676250722_i32);
_2.fld2 = (-312137104_i32);
_2.fld0 = !14306_i16;
RET = (-47_isize) ^ 9223372036854775807_isize;
RET = '\u{c8233}' as isize;
_2.fld1 = 692_u16;
_3 = 321205769366983545135397235538674662179_u128;
match _2.fld2 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
340282366920938463463374607431456074352 => bb10,
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
_2.fld2 = 899001168_i32;
RET = -(-9223372036854775808_isize);
_2.fld0 = 18977_i16;
_4 = 18584191610803151621917631512540165408_i128 as f32;
_3 = '\u{109e6}' as u128;
_5.0 = _2.fld2 as i64;
_2 = Adt17 { fld0: 5897_i16,fld1: 43014_u16,fld2: (-1395358164_i32),fld3: 76_u8 };
_6 = false | true;
_1.0 = [_2.fld2,_2.fld2,_2.fld2,_2.fld2];
_8.1 = '\u{c3337}';
_4 = _2.fld0 as f32;
_8.0 = _2.fld2 ^ _2.fld2;
_2.fld3 = 156_u8 * 201_u8;
_2.fld0 = 18706_i16 | (-19375_i16);
_1.0 = [_8.0,_8.0,_2.fld2,_8.0];
_3 = 138344607586224864898493921597095904498_u128 ^ 101504047765268422238313453249203023914_u128;
RET = !9223372036854775807_isize;
_2.fld1 = 47229_u16;
_8 = (_2.fld2, '\u{74bd2}');
_2 = Adt17 { fld0: 29151_i16,fld1: 28286_u16,fld2: _8.0,fld3: 182_u8 };
match _2.fld1 {
28286 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_2 = Adt17 { fld0: (-26733_i16),fld1: 60133_u16,fld2: _8.0,fld3: 66_u8 };
_2.fld3 = 40_u8;
_5 = ((-5935653236096907249_i64),);
RET = 9223372036854775807_isize;
_1.0 = [_2.fld2,_2.fld2,_2.fld2,_2.fld2];
_8.0 = _6 as i32;
_4 = 10250506739764154527_u64 as f32;
_4 = (-19_i8) as f32;
_2.fld0 = -25985_i16;
_2.fld1 = 53635_u16 + 31851_u16;
_3 = _2.fld1 as u128;
Goto(bb13)
}
bb13 = {
_1.0 = [_2.fld2,_2.fld2,_2.fld2,_2.fld2];
_2.fld0 = _5.0 as i16;
_10 = _4 as i64;
Goto(bb14)
}
bb14 = {
_14 = &_3;
_2.fld1 = !20718_u16;
Goto(bb15)
}
bb15 = {
Call(_15 = dump_var(15_usize, 10_usize, Move(_10), 6_usize, Move(_6), 3_usize, Move(_3), 16_usize, _16), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: &'static char,mut _2: i32,mut _3: u128,mut _4: (u128, f32, (i32, char)),mut _5: (u128, f32, (i32, char)),mut _6: i32,mut _7: u128,mut _8: isize,mut _9: isize) -> [i8; 1] {
mir! {
type RET = [i8; 1];
let _10: *mut [u64; 1];
let _11: f64;
let _12: *const ((u128, f32, (i32, char)), i32);
let _13: *mut &'static i16;
let _14: (i32, char);
let _15: [i8; 2];
let _16: [i8; 2];
let _17: f32;
let _18: *mut isize;
let _19: isize;
let _20: u32;
let _21: char;
let _22: u16;
let _23: &'static char;
let _24: bool;
let _25: i64;
let _26: Adt32;
let _27: isize;
let _28: f64;
let _29: i16;
let _30: &'static i16;
let _31: isize;
let _32: isize;
let _33: u128;
let _34: (u128, f32, (i32, char));
let _35: [i64; 8];
let _36: ();
let _37: ();
{
_5 = (_4.0, _4.1, _4.2);
_5.2.0 = true as i32;
_8 = 34726_u16 as isize;
_4.1 = _5.1;
RET = [(-48_i8)];
_4.2.1 = _5.2.1;
_4.2.1 = _5.2.1;
Call(_3 = fn17(_9, _4.0, _4, _5, _4.0, _6, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9 = !_8;
_3 = _5.0 + _5.0;
_1 = &_4.2.1;
_4.2.1 = _5.2.1;
_4.2.1 = _5.2.1;
_11 = 2304726299611807151_i64 as f64;
RET = [(-106_i8)];
_3 = _9 as u128;
_14 = (_2, _4.2.1);
_1 = &_5.2.1;
_5.0 = _11 as u128;
Goto(bb2)
}
bb2 = {
_4.2.0 = !_6;
_2 = _14.0 - _4.2.0;
_8 = _9;
_19 = _8;
_5.2 = (_4.2.0, _14.1);
_5.2.0 = _4.2.0;
_8 = 51485_u16 as isize;
_4.2.0 = -_14.0;
_22 = 20087_u16 | 39337_u16;
_22 = 51752_u16;
_15 = [121_i8,107_i8];
_21 = _5.2.1;
_17 = _5.1;
_2 = -_14.0;
_4.2 = (_2, _14.1);
_16 = _15;
_14.0 = _2;
_19 = -_8;
_18 = core::ptr::addr_of_mut!(_9);
_4.0 = _7 >> _14.0;
_15 = [79_i8,84_i8];
_5.2 = (_14.0, _4.2.1);
_18 = core::ptr::addr_of_mut!(_9);
Goto(bb3)
}
bb3 = {
_1 = &_14.1;
_14.1 = _5.2.1;
_6 = _2;
Goto(bb4)
}
bb4 = {
_5.0 = _4.0;
_25 = 7389376384257333571_i64 >> _5.0;
_4.2.0 = _2 << _25;
_21 = _4.2.1;
_16 = _15;
_5.2.1 = _21;
_4.2 = (_6, _5.2.1);
_23 = &_14.1;
_15 = [(-65_i8),(-119_i8)];
_4.2.0 = _6 * _6;
_16 = [(-29_i8),33_i8];
_5.0 = (-89_i8) as u128;
_14.1 = _5.2.1;
_4.2.0 = _5.2.0 | _5.2.0;
_1 = &_21;
_5.1 = _7 as f32;
_20 = 1807000077_u32;
_4.1 = _5.1 * _5.1;
_4.2 = _5.2;
Goto(bb5)
}
bb5 = {
_4.2.1 = _14.1;
_7 = !_4.0;
_20 = 408679901_u32;
_26 = Adt32::Variant1 { fld0: true,fld1: _5.2.1 };
_1 = &_14.1;
_4.0 = !_7;
_5.2.1 = (*_1);
_4.2.0 = -_5.2.0;
_5 = (_4.0, _4.1, _14);
_5.0 = (-9155_i16) as u128;
_28 = -_11;
_6 = _4.2.0 << _25;
_24 = true & true;
_5.2.0 = _24 as i32;
match _20 {
0 => bb1,
1 => bb2,
408679901 => bb7,
_ => bb6
}
}
bb6 = {
_4.2.0 = !_6;
_2 = _14.0 - _4.2.0;
_8 = _9;
_19 = _8;
_5.2 = (_4.2.0, _14.1);
_5.2.0 = _4.2.0;
_8 = 51485_u16 as isize;
_4.2.0 = -_14.0;
_22 = 20087_u16 | 39337_u16;
_22 = 51752_u16;
_15 = [121_i8,107_i8];
_21 = _5.2.1;
_17 = _5.1;
_2 = -_14.0;
_4.2 = (_2, _14.1);
_16 = _15;
_14.0 = _2;
_19 = -_8;
_18 = core::ptr::addr_of_mut!(_9);
_4.0 = _7 >> _14.0;
_15 = [79_i8,84_i8];
_5.2 = (_14.0, _4.2.1);
_18 = core::ptr::addr_of_mut!(_9);
Goto(bb3)
}
bb7 = {
_1 = &_14.1;
_7 = _4.0;
_21 = _14.1;
place!(Field::<char>(Variant(_26, 1), 1)) = _5.2.1;
_23 = &_5.2.1;
_13 = core::ptr::addr_of_mut!(_30);
RET = [121_i8];
_5.2.0 = _4.2.0;
_14.0 = _4.0 as i32;
_11 = _28;
_4.2.0 = _5.2.0 << _2;
_8 = (*_18);
RET = [0_i8];
_16 = [77_i8,92_i8];
RET = [80_i8];
_14 = _4.2;
_14 = (_2, (*_23));
_1 = Move(_23);
_18 = core::ptr::addr_of_mut!((*_18));
_19 = (*_18) << _4.2.0;
match _20 {
0 => bb1,
408679901 => bb9,
_ => bb8
}
}
bb8 = {
_5.0 = _4.0;
_25 = 7389376384257333571_i64 >> _5.0;
_4.2.0 = _2 << _25;
_21 = _4.2.1;
_16 = _15;
_5.2.1 = _21;
_4.2 = (_6, _5.2.1);
_23 = &_14.1;
_15 = [(-65_i8),(-119_i8)];
_4.2.0 = _6 * _6;
_16 = [(-29_i8),33_i8];
_5.0 = (-89_i8) as u128;
_14.1 = _5.2.1;
_4.2.0 = _5.2.0 | _5.2.0;
_1 = &_21;
_5.1 = _7 as f32;
_20 = 1807000077_u32;
_4.1 = _5.1 * _5.1;
_4.2 = _5.2;
Goto(bb5)
}
bb9 = {
_34 = (_4.0, _4.1, _14);
_24 = _7 >= _34.0;
_5.2.1 = _21;
_34.2.1 = _5.2.1;
match _20 {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb10,
4 => bb11,
5 => bb12,
6 => bb13,
408679901 => bb15,
_ => bb14
}
}
bb10 = {
_5.0 = _4.0;
_25 = 7389376384257333571_i64 >> _5.0;
_4.2.0 = _2 << _25;
_21 = _4.2.1;
_16 = _15;
_5.2.1 = _21;
_4.2 = (_6, _5.2.1);
_23 = &_14.1;
_15 = [(-65_i8),(-119_i8)];
_4.2.0 = _6 * _6;
_16 = [(-29_i8),33_i8];
_5.0 = (-89_i8) as u128;
_14.1 = _5.2.1;
_4.2.0 = _5.2.0 | _5.2.0;
_1 = &_21;
_5.1 = _7 as f32;
_20 = 1807000077_u32;
_4.1 = _5.1 * _5.1;
_4.2 = _5.2;
Goto(bb5)
}
bb11 = {
_1 = &_14.1;
_7 = _4.0;
_21 = _14.1;
place!(Field::<char>(Variant(_26, 1), 1)) = _5.2.1;
_23 = &_5.2.1;
_13 = core::ptr::addr_of_mut!(_30);
RET = [121_i8];
_5.2.0 = _4.2.0;
_14.0 = _4.0 as i32;
_11 = _28;
_4.2.0 = _5.2.0 << _2;
_8 = (*_18);
RET = [0_i8];
_16 = [77_i8,92_i8];
RET = [80_i8];
_14 = _4.2;
_14 = (_2, (*_23));
_1 = Move(_23);
_18 = core::ptr::addr_of_mut!((*_18));
_19 = (*_18) << _4.2.0;
match _20 {
0 => bb1,
408679901 => bb9,
_ => bb8
}
}
bb12 = {
_4.2.0 = !_6;
_2 = _14.0 - _4.2.0;
_8 = _9;
_19 = _8;
_5.2 = (_4.2.0, _14.1);
_5.2.0 = _4.2.0;
_8 = 51485_u16 as isize;
_4.2.0 = -_14.0;
_22 = 20087_u16 | 39337_u16;
_22 = 51752_u16;
_15 = [121_i8,107_i8];
_21 = _5.2.1;
_17 = _5.1;
_2 = -_14.0;
_4.2 = (_2, _14.1);
_16 = _15;
_14.0 = _2;
_19 = -_8;
_18 = core::ptr::addr_of_mut!(_9);
_4.0 = _7 >> _14.0;
_15 = [79_i8,84_i8];
_5.2 = (_14.0, _4.2.1);
_18 = core::ptr::addr_of_mut!(_9);
Goto(bb3)
}
bb13 = {
_4.2.0 = !_6;
_2 = _14.0 - _4.2.0;
_8 = _9;
_19 = _8;
_5.2 = (_4.2.0, _14.1);
_5.2.0 = _4.2.0;
_8 = 51485_u16 as isize;
_4.2.0 = -_14.0;
_22 = 20087_u16 | 39337_u16;
_22 = 51752_u16;
_15 = [121_i8,107_i8];
_21 = _5.2.1;
_17 = _5.1;
_2 = -_14.0;
_4.2 = (_2, _14.1);
_16 = _15;
_14.0 = _2;
_19 = -_8;
_18 = core::ptr::addr_of_mut!(_9);
_4.0 = _7 >> _14.0;
_15 = [79_i8,84_i8];
_5.2 = (_14.0, _4.2.1);
_18 = core::ptr::addr_of_mut!(_9);
Goto(bb3)
}
bb14 = {
_5.0 = _4.0;
_25 = 7389376384257333571_i64 >> _5.0;
_4.2.0 = _2 << _25;
_21 = _4.2.1;
_16 = _15;
_5.2.1 = _21;
_4.2 = (_6, _5.2.1);
_23 = &_14.1;
_15 = [(-65_i8),(-119_i8)];
_4.2.0 = _6 * _6;
_16 = [(-29_i8),33_i8];
_5.0 = (-89_i8) as u128;
_14.1 = _5.2.1;
_4.2.0 = _5.2.0 | _5.2.0;
_1 = &_21;
_5.1 = _7 as f32;
_20 = 1807000077_u32;
_4.1 = _5.1 * _5.1;
_4.2 = _5.2;
Goto(bb5)
}
bb15 = {
_2 = 2_usize as i32;
_3 = !_4.0;
_18 = core::ptr::addr_of_mut!((*_18));
_5.2.1 = _34.2.1;
_31 = _25 as isize;
_20 = _24 as u32;
Goto(bb16)
}
bb16 = {
Call(_36 = dump_var(16_usize, 21_usize, Move(_21), 7_usize, Move(_7), 20_usize, Move(_20), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(16_usize, 22_usize, Move(_22), 19_usize, Move(_19), 9_usize, Move(_9), 25_usize, Move(_25)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: isize,mut _2: u128,mut _3: (u128, f32, (i32, char)),mut _4: (u128, f32, (i32, char)),mut _5: u128,mut _6: i32,mut _7: i32) -> u128 {
mir! {
type RET = u128;
let _8: ();
let _9: ();
{
_3.2.1 = _4.2.1;
_1 = 65013_u16 as isize;
_4.2 = (_6, _3.2.1);
_3.1 = _4.1;
_2 = _3.0;
_3.1 = _4.1 - _4.1;
_3.2.0 = !_7;
RET = !_2;
_4.2.1 = _3.2.1;
_3 = (_2, _4.1, _4.2);
_1 = 80_i8 as isize;
_2 = !RET;
_3.1 = -_4.1;
RET = !_3.0;
Goto(bb1)
}
bb1 = {
Call(_8 = dump_var(17_usize, 6_usize, Move(_6), 2_usize, Move(_2), 9_usize, _9, 9_usize, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: *const &'static i32,mut _2: *const *const &'static i32,mut _3: &'static char,mut _4: bool,mut _5: f32,mut _6: isize,mut _7: u16,mut _8: bool,mut _9: (isize, [i8; 1], char, f32),mut _10: isize,mut _11: i32,mut _12: isize) -> [u64; 8] {
mir! {
type RET = [u64; 8];
let _13: i64;
let _14: &'static &'static [u64; 3];
let _15: &'static [i8; 2];
let _16: char;
let _17: &'static u32;
let _18: *const *const &'static i32;
let _19: u128;
let _20: [i64; 7];
let _21: char;
let _22: u64;
let _23: &'static u8;
let _24: bool;
let _25: Adt76;
let _26: isize;
let _27: u16;
let _28: *mut [u64; 1];
let _29: ([i128; 2], u16, &'static char, &'static *mut *const &'static i32);
let _30: bool;
let _31: ();
let _32: ();
{
_6 = _10;
_8 = _4 ^ _4;
RET = [15821922366293962725_u64,11718710777812230653_u64,16456290588761118364_u64,15754425937903481089_u64,14896089586453275680_u64,12085727583598729089_u64,8078435547251141001_u64,1462650427901263700_u64];
RET = [17773437073074886245_u64,13876075473196405783_u64,3721860449471062118_u64,8966801530872944906_u64,9005920121497584278_u64,2617313972286883749_u64,16773057095388756696_u64,13129991831701145726_u64];
_6 = 1_usize as isize;
_10 = _9.0 + _9.0;
Call(_9.1 = core::intrinsics::transmute(_8), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9.0 = _10;
_4 = _8;
RET = [13808508395639281981_u64,14186975753890736475_u64,14705241341806299262_u64,14149858242869805531_u64,9016056902107645693_u64,16539641772842184855_u64,1992630305841057379_u64,5505436996407217642_u64];
_16 = _9.2;
_8 = _11 < _11;
_4 = !_8;
_9.0 = _12 ^ _12;
_7 = 57033_u16;
_18 = Move(_2);
_9.0 = !_10;
_7 = !35449_u16;
_7 = 61963_u16 - 4228_u16;
_3 = &_16;
Goto(bb2)
}
bb2 = {
_5 = _9.3;
_5 = _9.3;
_9.3 = _5;
_10 = _12 >> _9.0;
_13 = -(-3767271839173655109_i64);
Goto(bb3)
}
bb3 = {
_9.3 = -_5;
Goto(bb4)
}
bb4 = {
_5 = -_9.3;
Goto(bb5)
}
bb5 = {
_4 = !_8;
_7 = 10063_u16;
_9.0 = _10;
_6 = !_9.0;
_6 = _10;
match _7 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
10063 => bb8,
_ => bb7
}
}
bb6 = {
_5 = -_9.3;
Goto(bb5)
}
bb7 = {
_9.3 = -_5;
Goto(bb4)
}
bb8 = {
_9.1 = [(-43_i8)];
_4 = !_8;
_20 = [_13,_13,_13,_13,_13,_13,_13];
_21 = (*_3);
_5 = _9.3;
_13 = (-3905346540011927118_i64);
match _13 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb4,
4 => bb7,
5 => bb9,
340282366920938463459469260891756284338 => bb11,
_ => bb10
}
}
bb9 = {
_9.3 = -_5;
Goto(bb4)
}
bb10 = {
_5 = -_9.3;
Goto(bb5)
}
bb11 = {
RET = [5910702427791784945_u64,9289441962126638130_u64,16320149925803925370_u64,4508179432213242109_u64,13343961267296806765_u64,7547987623726515965_u64,3346777562971683225_u64,8040161957325539911_u64];
_18 = core::ptr::addr_of!(_1);
_13 = 1604490963217307929_i64 | (-4852889638351204572_i64);
_2 = core::ptr::addr_of!((*_18));
_9.3 = -_5;
_5 = _13 as f32;
RET = [1754137326386904424_u64,3280709041465111653_u64,13676014290070542002_u64,17109482803476448454_u64,2109591031944072305_u64,4269883048612473873_u64,8263098333016517562_u64,8363534418076837965_u64];
_24 = _4 ^ _8;
_12 = -_10;
_9.2 = (*_3);
_10 = _6;
_22 = 172763427152250899026804051583915280392_u128 as u64;
_9.3 = _5;
_9.0 = _12 | _12;
_21 = (*_3);
RET = [_22,_22,_22,_22,_22,_22,_22,_22];
_8 = _24;
_9.2 = (*_3);
_27 = _4 as u16;
_19 = (*_3) as u128;
match _7 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
10063 => bb18,
_ => bb17
}
}
bb12 = {
_5 = -_9.3;
Goto(bb5)
}
bb13 = {
_5 = -_9.3;
Goto(bb5)
}
bb14 = {
_9.1 = [(-43_i8)];
_4 = !_8;
_20 = [_13,_13,_13,_13,_13,_13,_13];
_21 = (*_3);
_5 = _9.3;
_13 = (-3905346540011927118_i64);
match _13 {
0 => bb1,
1 => bb2,
2 => bb6,
3 => bb4,
4 => bb7,
5 => bb9,
340282366920938463459469260891756284338 => bb11,
_ => bb10
}
}
bb15 = {
_9.3 = -_5;
Goto(bb4)
}
bb16 = {
_5 = -_9.3;
Goto(bb5)
}
bb17 = {
_9.0 = _10;
_4 = _8;
RET = [13808508395639281981_u64,14186975753890736475_u64,14705241341806299262_u64,14149858242869805531_u64,9016056902107645693_u64,16539641772842184855_u64,1992630305841057379_u64,5505436996407217642_u64];
_16 = _9.2;
_8 = _11 < _11;
_4 = !_8;
_9.0 = _12 ^ _12;
_7 = 57033_u16;
_18 = Move(_2);
_9.0 = !_10;
_7 = !35449_u16;
_7 = 61963_u16 - 4228_u16;
_3 = &_16;
Goto(bb2)
}
bb18 = {
_25.fld0 = [_11,_11,_11,_11];
_13 = (-52461637113948015622594776238868863893_i128) as i64;
_27 = _7 - _7;
_20 = [_13,_13,_13,_13,_13,_13,_13];
_25.fld1 = _22 << _10;
_25.fld0 = [_11,_11,_11,_11];
_3 = &_21;
_22 = _25.fld1;
_9.1 = [98_i8];
_12 = _6;
_20 = [_13,_13,_13,_13,_13,_13,_13];
_25.fld1 = _22 | _22;
_9.1 = [(-57_i8)];
_3 = &_9.2;
_24 = _8;
_9.3 = _11 as f32;
RET = [_22,_25.fld1,_22,_25.fld1,_25.fld1,_22,_25.fld1,_25.fld1];
Goto(bb19)
}
bb19 = {
Call(_31 = dump_var(18_usize, 20_usize, Move(_20), 4_usize, Move(_4), 11_usize, Move(_11), 6_usize, Move(_6)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_31 = dump_var(18_usize, 16_usize, Move(_16), 13_usize, Move(_13), 12_usize, Move(_12), 32_usize, _32), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(true), std::hint::black_box('\u{1f280}'), std::hint::black_box(173209636980924832262882664239354020428_u128), std::hint::black_box(3472628732_u32), std::hint::black_box(20677_i16), std::hint::black_box(2032259324_i32), std::hint::black_box((-8088457563954403840_i64)), std::hint::black_box(47688_u16), std::hint::black_box(6_usize));
                
            }
impl PrintFDebug for Adt17{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt17{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt17 {
fld0: i16,
fld1: u16,
fld2: i32,
fld3: u8,
}
impl PrintFDebug for Adt24{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt24{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt24 {
fld0: u64,
fld1: [i128; 2],
fld2: i64,
}
impl PrintFDebug for Adt32{
	unsafe fn printf_debug(&self){unsafe{printf("Adt32::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt32 {
Variant0{
fld0: u64,
fld1: i128,
fld2: Adt17,
fld3: i8,
fld4: (i32, char),
fld5: Adt24,
fld6: *mut i8,

},
Variant1{
fld0: bool,
fld1: char,

},
Variant2{
fld0: [i128; 2],
fld1: u64,
fld2: u128,
fld3: u8,
fld4: f64,
fld5: i32,
fld6: *mut i8,

},
Variant3{
fld0: bool,
fld1: f64,
fld2: ((u128, f32, (i32, char)), i32),
fld3: Adt24,
fld4: u128,
fld5: u8,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: [i8; 1],
fld1: u16,
fld2: *mut i8,

},
Variant1{
fld0: bool,
fld1: [i128; 2],
fld2: *mut [i128; 2],
fld3: *mut i8,
fld4: u16,

}}
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: u8,
fld1: (u128, f32, (i32, char)),
fld2: isize,
fld3: ([i32; 4],),
fld4: i16,
fld5: f32,
fld6: [u8; 8],
fld7: (f64, isize, [i32; 4]),

},
Variant1{
fld0: [i8; 1],
fld1: u16,
fld2: isize,
fld3: [i64; 8],
fld4: [u8; 8],
fld5: *mut (u128, f32, (i32, char)),
fld6: i64,

}}
impl PrintFDebug for Adt62{
	unsafe fn printf_debug(&self){unsafe{printf("Adt62::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
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
#[derive(Copy,Clone)]pub enum Adt62 {
Variant0{
fld0: *mut [i128; 2],
fld1: [u128; 2],
fld2: Adt45,
fld3: [i64; 8],
fld4: (i32, char),
fld5: i128,
fld6: i64,

},
Variant1{
fld0: [i64; 7],
fld1: Adt45,
fld2: [i64; 8],
fld3: [u8; 8],

},
Variant2{
fld0: ([i8; 1], [i64; 7], f32, *mut *mut (u128, f32, (i32, char))),
fld1: [i64; 8],
fld2: [u64; 3],
fld3: *const i8,
fld4: *mut i8,

},
Variant3{
fld0: i8,
fld1: isize,

}}
impl PrintFDebug for Adt76{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt76{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt76 {
fld0: [i32; 4],
fld1: u64,
fld2: Adt62,
}
impl PrintFDebug for Adt79{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt79{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt79 {
fld0: (f64, isize, [i32; 4]),
fld1: Adt45,
fld2: ([i32; 4],),
fld3: u16,
fld4: [u32; 4],
fld5: i32,
}

