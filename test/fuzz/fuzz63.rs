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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: i8,mut _5: i16,mut _6: i32,mut _7: i64,mut _8: i128,mut _9: usize,mut _10: u8,mut _11: u16,mut _12: u32,mut _13: u64,mut _14: u128) -> [u128; 4] {
mir! {
type RET = [u128; 4];
let _15: [i64; 5];
let _16: [i64; 5];
let _17: f64;
let _18: u16;
let _19: Adt43;
let _20: [i32; 1];
let _21: u64;
let _22: ([i64; 5], [u16; 4], bool, i32);
let _23: [u128; 4];
let _24: f32;
let _25: f64;
let _26: [i32; 1];
let _27: u8;
let _28: i128;
let _29: char;
let _30: Adt59;
let _31: usize;
let _32: [usize; 5];
let _33: u16;
let _34: i64;
let _35: Adt54;
let _36: f32;
let _37: isize;
let _38: [i64; 5];
let _39: f64;
let _40: bool;
let _41: [char; 3];
let _42: [u128; 4];
let _43: [u16; 4];
let _44: (*mut u32, *mut i16);
let _45: ();
let _46: ();
{
RET = [157118168693686926174221817693158951026_u128,312206765456626737277330949269445850793_u128,153721627005795544881497820587532584620_u128,150246313851143454290117005257438023101_u128];
_9 = !0_usize;
_7 = 6359294531597912010_i64 ^ (-5533962191141915147_i64);
_11 = !2209_u16;
_9 = _11 as usize;
_12 = 3753939519_u32;
RET = [18102437500930353695454368313701559775_u128,33130291431908021286151574635034495589_u128,12177160913596877860544255481146406785_u128,176486851730325081506593204482096301827_u128];
_14 = (-68_isize) as u128;
_12 = 4248718011_u32 & 848084560_u32;
_6 = -(-1213370383_i32);
_8 = _6 as i128;
Goto(bb1)
}
bb1 = {
_15 = [_7,_7,_7,_7,_7];
_2 = '\u{6789f}';
_4 = (-97_i8) & (-51_i8);
_13 = !9012193813702012266_u64;
_7 = (-9109637635144785328_i64) * 2900209241454165695_i64;
_17 = 182_u8 as f64;
_4 = (-75_i8) ^ 15_i8;
Goto(bb2)
}
bb2 = {
_1 = false | true;
_9 = 2505457218117934952_usize << _8;
_16 = [_7,_7,_7,_7,_7];
_4 = 84_i8 * (-54_i8);
RET = [_14,_14,_14,_14];
_1 = !true;
_1 = true;
_10 = 122_u8;
_15 = [_7,_7,_7,_7,_7];
_3 = (-29_isize) - 9223372036854775807_isize;
_16 = _15;
_15 = _16;
_15 = _16;
_12 = 3378106193_u32;
_2 = '\u{238ed}';
_6 = -(-333227356_i32);
_5 = !(-13594_i16);
_8 = -168131547545844446019735084461734325186_i128;
RET = [_14,_14,_14,_14];
_6 = -1479458428_i32;
_9 = 3_usize;
_6 = -884763546_i32;
_22.1[_9] = !_11;
_22.0[_9] = -_15[_9];
match _12 {
0 => bb3,
3378106193 => bb5,
_ => bb4
}
}
bb3 = {
_15 = [_7,_7,_7,_7,_7];
_2 = '\u{6789f}';
_4 = (-97_i8) & (-51_i8);
_13 = !9012193813702012266_u64;
_7 = (-9109637635144785328_i64) * 2900209241454165695_i64;
_17 = 182_u8 as f64;
_4 = (-75_i8) ^ 15_i8;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
_16[_9] = -_7;
_22.3 = _6;
_14 = RET[_9] + RET[_9];
_23 = [RET[_9],RET[_9],_14,RET[_9]];
_23 = [RET[_9],_14,_14,_14];
_9 = !7_usize;
_13 = 5759655256740752660_u64 << _8;
match _10 {
0 => bb1,
122 => bb7,
_ => bb6
}
}
bb6 = {
_1 = false | true;
_9 = 2505457218117934952_usize << _8;
_16 = [_7,_7,_7,_7,_7];
_4 = 84_i8 * (-54_i8);
RET = [_14,_14,_14,_14];
_1 = !true;
_1 = true;
_10 = 122_u8;
_15 = [_7,_7,_7,_7,_7];
_3 = (-29_isize) - 9223372036854775807_isize;
_16 = _15;
_15 = _16;
_15 = _16;
_12 = 3378106193_u32;
_2 = '\u{238ed}';
_6 = -(-333227356_i32);
_5 = !(-13594_i16);
_8 = -168131547545844446019735084461734325186_i128;
RET = [_14,_14,_14,_14];
_6 = -1479458428_i32;
_9 = 3_usize;
_6 = -884763546_i32;
_22.1[_9] = !_11;
_22.0[_9] = -_15[_9];
match _12 {
0 => bb3,
3378106193 => bb5,
_ => bb4
}
}
bb7 = {
_23 = RET;
_7 = (-9155479734420836775_i64);
_12 = !325777657_u32;
_21 = !_13;
_11 = 16186_u16;
_25 = -_17;
_2 = '\u{c2567}';
_4 = _10 as i8;
_5 = !1331_i16;
_22.2 = _1;
_24 = _3 as f32;
_18 = _11;
RET = [_14,_14,_14,_14];
_22.1 = [_11,_11,_18,_11];
_22.3 = !_6;
_20 = [_22.3];
_7 = _24 as i64;
RET = [_14,_14,_14,_14];
_7 = !6661618648131323081_i64;
_28 = _8 + _8;
_23 = [_14,_14,_14,_14];
_3 = _2 as isize;
_14 = !236446905633237515990362270582541920937_u128;
_25 = -_17;
_24 = _8 as f32;
_22.0 = [_7,_7,_7,_7,_7];
_22.1 = [_11,_18,_18,_11];
Goto(bb8)
}
bb8 = {
_20 = [_6];
_16 = _22.0;
_14 = 327760783507019586261075241733446131087_u128;
_2 = '\u{bb85e}';
_26 = [_6];
_4 = 62_i8 & (-123_i8);
_20 = _26;
RET = [_14,_14,_14,_14];
_3 = !9223372036854775807_isize;
Goto(bb9)
}
bb9 = {
_16 = [_7,_7,_7,_7,_7];
_7 = 6112025166843669858_i64;
_10 = !238_u8;
_23 = [_14,_14,_14,_14];
_15 = [_7,_7,_7,_7,_7];
_7 = (-8663663839985712050_i64) * (-3313317999246715643_i64);
_21 = _6 as u64;
_26 = [_22.3];
_22.0 = _16;
_18 = _17 as u16;
_22.1 = [_11,_18,_11,_11];
_24 = _3 as f32;
_7 = -(-377256578684251471_i64);
_4 = _5 as i8;
_17 = _7 as f64;
_26 = [_6];
_11 = _24 as u16;
_8 = _13 as i128;
_16 = [_7,_7,_7,_7,_7];
RET = [_14,_14,_14,_14];
_31 = _9;
_12 = _10 as u32;
_29 = _2;
_11 = _18 + _18;
_26 = _20;
_20 = [_22.3];
Call(_12 = fn1(_14, _1, _15, _5, _16, _22.3), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_29 = _2;
_1 = _22.2 ^ _22.2;
_9 = _31;
_31 = !_9;
_22.0 = [_7,_7,_7,_7,_7];
_6 = _22.3;
_8 = _28;
_29 = _2;
_15 = _22.0;
_20 = [_22.3];
_23 = [_14,_14,_14,_14];
_28 = _8 + _8;
_18 = _11;
_31 = _9 >> _8;
_29 = _2;
_22.1 = [_18,_18,_18,_18];
_37 = _3 * _3;
_22.2 = _1;
_17 = _25 - _25;
_8 = _5 as i128;
Goto(bb11)
}
bb11 = {
_3 = _37;
_8 = !_28;
_37 = _21 as isize;
_7 = 1357294241420194202_i64 & 5410897887834656787_i64;
_33 = _11;
_37 = _10 as isize;
_32 = [_31,_9,_31,_31,_31];
_33 = !_18;
match _14 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb9,
4 => bb10,
5 => bb12,
327760783507019586261075241733446131087 => bb14,
_ => bb13
}
}
bb12 = {
_16[_9] = -_7;
_22.3 = _6;
_14 = RET[_9] + RET[_9];
_23 = [RET[_9],RET[_9],_14,RET[_9]];
_23 = [RET[_9],_14,_14,_14];
_9 = !7_usize;
_13 = 5759655256740752660_u64 << _8;
match _10 {
0 => bb1,
122 => bb7,
_ => bb6
}
}
bb13 = {
_1 = false | true;
_9 = 2505457218117934952_usize << _8;
_16 = [_7,_7,_7,_7,_7];
_4 = 84_i8 * (-54_i8);
RET = [_14,_14,_14,_14];
_1 = !true;
_1 = true;
_10 = 122_u8;
_15 = [_7,_7,_7,_7,_7];
_3 = (-29_isize) - 9223372036854775807_isize;
_16 = _15;
_15 = _16;
_15 = _16;
_12 = 3378106193_u32;
_2 = '\u{238ed}';
_6 = -(-333227356_i32);
_5 = !(-13594_i16);
_8 = -168131547545844446019735084461734325186_i128;
RET = [_14,_14,_14,_14];
_6 = -1479458428_i32;
_9 = 3_usize;
_6 = -884763546_i32;
_22.1[_9] = !_11;
_22.0[_9] = -_15[_9];
match _12 {
0 => bb3,
3378106193 => bb5,
_ => bb4
}
}
bb14 = {
_22.0 = [_7,_7,_7,_7,_7];
_28 = _24 as i128;
_27 = _11 as u8;
_22.2 = _1;
_24 = _11 as f32;
_34 = _8 as i64;
_38 = _22.0;
_23 = [_14,_14,_14,_14];
_21 = _13;
_21 = _13 ^ _13;
_39 = _17 * _17;
_25 = _39 + _39;
_13 = _24 as u64;
_28 = !_8;
_29 = _2;
_7 = _34;
_8 = _5 as i128;
_20 = _26;
_18 = !_11;
_36 = _34 as f32;
_44.1 = core::ptr::addr_of_mut!(_5);
_20 = [_6];
_41 = [_29,_2,_29];
_34 = _7 >> _31;
_42 = [_14,_14,_14,_14];
_33 = _11;
Goto(bb15)
}
bb15 = {
Call(_45 = dump_var(0_usize, 37_usize, Move(_37), 26_usize, Move(_26), 11_usize, Move(_11), 29_usize, Move(_29)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_45 = dump_var(0_usize, 12_usize, Move(_12), 21_usize, Move(_21), 28_usize, Move(_28), 13_usize, Move(_13)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_45 = dump_var(0_usize, 2_usize, Move(_2), 22_usize, Move(_22), 41_usize, Move(_41), 6_usize, Move(_6)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_45 = dump_var(0_usize, 27_usize, Move(_27), 7_usize, Move(_7), 38_usize, Move(_38), 5_usize, Move(_5)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn1(mut _1: u128,mut _2: bool,mut _3: [i64; 5],mut _4: i16,mut _5: [i64; 5],mut _6: i32) -> u32 {
mir! {
type RET = u32;
let _7: [i32; 1];
let _8: Adt54;
let _9: Adt55;
let _10: [u16; 4];
let _11: ([i64; 5], [u16; 4], bool, i32);
let _12: &'static i16;
let _13: char;
let _14: Adt44;
let _15: isize;
let _16: [char; 3];
let _17: isize;
let _18: [char; 3];
let _19: u32;
let _20: [u128; 4];
let _21: (i64, (*mut u32, *mut i16));
let _22: *mut *mut i16;
let _23: i64;
let _24: isize;
let _25: *mut u32;
let _26: f32;
let _27: ([i64; 5], [u16; 4], bool, i32);
let _28: i8;
let _29: Adt50;
let _30: [usize; 5];
let _31: isize;
let _32: ();
let _33: ();
{
_4 = -(-31738_i16);
RET = 242853975_u32 * 1913057881_u32;
RET = 141761997_u32;
_2 = false | false;
_3 = _5;
_2 = !false;
RET = 1450044891_u32;
_3 = [(-7164482223597259890_i64),3970513264744349196_i64,(-4974748455870024186_i64),(-6842340981518023263_i64),3726207752562659397_i64];
_1 = !103929593785731049305841818633999915084_u128;
RET = 2197143697_u32 >> _6;
_4 = -25589_i16;
_5 = [(-566847088805044293_i64),6207790534965504856_i64,(-6000826033818098495_i64),(-4477617045046881174_i64),1623098303011792346_i64];
_2 = !true;
Call(_1 = fn2(_2, _2, RET, _5, _2, _3, RET, _2, RET, _4, RET, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = (-28281_i16);
_7 = [_6];
Goto(bb2)
}
bb2 = {
RET = !855057008_u32;
_2 = !true;
_5 = [(-288362194392422261_i64),8412578024117416503_i64,(-4519964687906257821_i64),6144041072175413001_i64,1078383920333468633_i64];
_6 = (-297806453_i32) | 994420860_i32;
_2 = false;
_2 = !false;
RET = 2869729039_u32 + 740094998_u32;
_2 = !true;
match _1 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
85194383276779326259350019192408872371 => bb8,
_ => bb7
}
}
bb3 = {
_4 = (-28281_i16);
_7 = [_6];
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
_6 = 530881412_i32;
RET = 2293489277_u32;
_9 = Adt55::Variant2 { fld0: _2,fld1: 105_isize };
_5 = _3;
_1 = 26673150220288208622074177705995065918_u128;
Goto(bb9)
}
bb9 = {
_1 = 169240391093789561144835853222451239011_u128 & 40135853720805962443095452787283609124_u128;
RET = 7860720329373357150_i64 as u32;
_6 = !2118047755_i32;
_10 = [54965_u16,6566_u16,39087_u16,57912_u16];
place!(Field::<isize>(Variant(_9, 2), 1)) = 9223372036854775807_isize;
RET = !577539302_u32;
place!(Field::<isize>(Variant(_9, 2), 1)) = (-9223372036854775808_isize);
_2 = _4 != _4;
_5 = [(-7569164638165984485_i64),3575051381295494912_i64,(-7003047115075238567_i64),8187107650685306965_i64,3420470175748266735_i64];
_4 = 32453_i16;
_11 = (_3, _10, _2, _6);
_6 = -_11.3;
_2 = _11.2 & Field::<bool>(Variant(_9, 2), 0);
_6 = 14151854627202562768_usize as i32;
_11 = (_3, _10, _2, _6);
_11 = (_3, _10, _2, _6);
SetDiscriminant(_9, 1);
_4 = !(-1481_i16);
_5 = _11.0;
_2 = _11.2;
_11.1 = [61475_u16,17089_u16,37202_u16,35773_u16];
place!(Field::<usize>(Variant(_9, 1), 3)) = !11540576813199216447_usize;
_7 = [_11.3];
place!(Field::<u32>(Variant(_9, 1), 0)) = !RET;
_4 = 6645_i16;
match _4 {
0 => bb6,
1 => bb2,
2 => bb5,
6645 => bb10,
_ => bb4
}
}
bb10 = {
_11.2 = _6 <= _6;
_10 = [52819_u16,802_u16,60977_u16,53178_u16];
_1 = !96553612728553891071983601806886886207_u128;
_7 = [_11.3];
_15 = (-9223372036854775808_isize);
_6 = '\u{be1b2}' as i32;
place!(Field::<i128>(Variant(_9, 1), 7)) = !(-90927896985593802922462226411721845452_i128);
place!(Field::<Adt44>(Variant(_9, 1), 4)) = Adt44::Variant2 { fld0: Field::<i128>(Variant(_9, 1), 7),fld1: 3706004786732294610_u64,fld2: _15,fld3: 92_i8,fld4: 108_u8 };
place!(Field::<Adt44>(Variant(_9, 1), 4)) = Adt44::Variant2 { fld0: Field::<i128>(Variant(_9, 1), 7),fld1: 13602405612807407168_u64,fld2: _15,fld3: 28_i8,fld4: 107_u8 };
place!(Field::<usize>(Variant(_9, 1), 3)) = !10054150135458457431_usize;
place!(Field::<i8>(Variant(place!(Field::<Adt44>(Variant(_9, 1), 4)), 2), 3)) = (-116_i8) | 94_i8;
_16 = ['\u{10b221}','\u{109944}','\u{92174}'];
_11.0 = [(-1627535680331433675_i64),(-316946524784451633_i64),2387526396847519036_i64,(-570806450920256594_i64),5414715706630922253_i64];
_17 = _15 ^ _15;
_11.2 = !_2;
place!(Field::<u64>(Variant(place!(Field::<Adt44>(Variant(_9, 1), 4)), 2), 1)) = _6 as u64;
_17 = -Field::<isize>(Variant(Field::<Adt44>(Variant(_9, 1), 4), 2), 2);
Call(_14 = fn3(_16, _5, _11.1, _11, _17, _11.1), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
RET = !Field::<u32>(Variant(_9, 1), 0);
place!(Field::<i8>(Variant(_14, 2), 3)) = -Field::<i8>(Variant(Field::<Adt44>(Variant(_9, 1), 4), 2), 3);
place!(Field::<i8>(Variant(_14, 2), 3)) = _4 as i8;
place!(Field::<u64>(Variant(_14, 2), 1)) = !Field::<u64>(Variant(Field::<Adt44>(Variant(_9, 1), 4), 2), 1);
SetDiscriminant(_14, 3);
place!(Field::<i128>(Variant(place!(Field::<Adt44>(Variant(_9, 1), 4)), 2), 0)) = Field::<i128>(Variant(_9, 1), 7) * Field::<i128>(Variant(_9, 1), 7);
place!(Field::<u64>(Variant(_14, 3), 4)) = Field::<u64>(Variant(Field::<Adt44>(Variant(_9, 1), 4), 2), 1) | Field::<u64>(Variant(Field::<Adt44>(Variant(_9, 1), 4), 2), 1);
_13 = '\u{85e2f}';
_18 = _16;
place!(Field::<*const *const char>(Variant(_9, 1), 2)) = core::ptr::addr_of!(place!(Field::<*const char>(Variant(_14, 3), 1)));
place!(Field::<*const *const char>(Variant(_9, 1), 2)) = core::ptr::addr_of!(place!(Field::<*const char>(Variant(_14, 3), 1)));
place!(Field::<u32>(Variant(_9, 1), 0)) = RET | RET;
_11.1 = [44244_u16,52402_u16,32065_u16,42133_u16];
RET = 13493_u16 as u32;
_11.2 = _1 <= _1;
_21.1.0 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_9, 1), 0)));
_21.0 = 6987375622460354397_i64 + 3553511668143831924_i64;
place!(Field::<*mut *mut i16>(Variant(_9, 1), 5)) = core::ptr::addr_of_mut!(_21.1.1);
_11 = (_5, _10, _2, _6);
_13 = '\u{50f6a}';
place!(Field::<*const *const char>(Variant(_9, 1), 2)) = core::ptr::addr_of!(place!(Field::<*const char>(Variant(_14, 3), 1)));
place!(Field::<[usize; 5]>(Variant(_14, 3), 0)) = [Field::<usize>(Variant(_9, 1), 3),Field::<usize>(Variant(_9, 1), 3),Field::<usize>(Variant(_9, 1), 3),Field::<usize>(Variant(_9, 1), 3),Field::<usize>(Variant(_9, 1), 3)];
place!(Field::<u8>(Variant(place!(Field::<Adt44>(Variant(_9, 1), 4)), 2), 4)) = !103_u8;
place!(Field::<u8>(Variant(place!(Field::<Adt44>(Variant(_9, 1), 4)), 2), 4)) = 77_u8 ^ 43_u8;
place!(Field::<u64>(Variant(place!(Field::<Adt44>(Variant(_9, 1), 4)), 2), 1)) = !Field::<u64>(Variant(_14, 3), 4);
match _15 {
0 => bb9,
340282366920938463454151235394913435648 => bb12,
_ => bb10
}
}
bb12 = {
_20 = [_1,_1,_1,_1];
place!(Field::<i128>(Variant(place!(Field::<Adt44>(Variant(_9, 1), 4)), 2), 0)) = Field::<i128>(Variant(_9, 1), 7) >> Field::<u64>(Variant(Field::<Adt44>(Variant(_9, 1), 4), 2), 1);
place!(Field::<[char; 3]>(Variant(_14, 3), 5)) = [_13,_13,_13];
_22 = core::ptr::addr_of_mut!(_21.1.1);
place!(Field::<u32>(Variant(_9, 1), 0)) = !RET;
place!(Field::<*const *const char>(Variant(_9, 1), 2)) = core::ptr::addr_of!(place!(Field::<*const char>(Variant(_14, 3), 1)));
(*_22) = core::ptr::addr_of_mut!(_4);
place!(Field::<isize>(Variant(_14, 3), 2)) = -Field::<isize>(Variant(Field::<Adt44>(Variant(_9, 1), 4), 2), 2);
place!(Field::<f32>(Variant(_14, 3), 6)) = RET as f32;
place!(Field::<u8>(Variant(place!(Field::<Adt44>(Variant(_9, 1), 4)), 2), 4)) = 236_u8;
_9 = Adt55::Variant2 { fld0: _2,fld1: _17 };
place!(Field::<u64>(Variant(_14, 3), 4)) = !15459457331157432488_u64;
_16 = _18;
_7 = [_11.3];
_1 = !182028052569962499274443883539580615658_u128;
match _15 {
0 => bb11,
1 => bb7,
2 => bb3,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
340282366920938463454151235394913435648 => bb18,
_ => bb17
}
}
bb13 = {
RET = !Field::<u32>(Variant(_9, 1), 0);
place!(Field::<i8>(Variant(_14, 2), 3)) = -Field::<i8>(Variant(Field::<Adt44>(Variant(_9, 1), 4), 2), 3);
place!(Field::<i8>(Variant(_14, 2), 3)) = _4 as i8;
place!(Field::<u64>(Variant(_14, 2), 1)) = !Field::<u64>(Variant(Field::<Adt44>(Variant(_9, 1), 4), 2), 1);
SetDiscriminant(_14, 3);
place!(Field::<i128>(Variant(place!(Field::<Adt44>(Variant(_9, 1), 4)), 2), 0)) = Field::<i128>(Variant(_9, 1), 7) * Field::<i128>(Variant(_9, 1), 7);
place!(Field::<u64>(Variant(_14, 3), 4)) = Field::<u64>(Variant(Field::<Adt44>(Variant(_9, 1), 4), 2), 1) | Field::<u64>(Variant(Field::<Adt44>(Variant(_9, 1), 4), 2), 1);
_13 = '\u{85e2f}';
_18 = _16;
place!(Field::<*const *const char>(Variant(_9, 1), 2)) = core::ptr::addr_of!(place!(Field::<*const char>(Variant(_14, 3), 1)));
place!(Field::<*const *const char>(Variant(_9, 1), 2)) = core::ptr::addr_of!(place!(Field::<*const char>(Variant(_14, 3), 1)));
place!(Field::<u32>(Variant(_9, 1), 0)) = RET | RET;
_11.1 = [44244_u16,52402_u16,32065_u16,42133_u16];
RET = 13493_u16 as u32;
_11.2 = _1 <= _1;
_21.1.0 = core::ptr::addr_of_mut!(place!(Field::<u32>(Variant(_9, 1), 0)));
_21.0 = 6987375622460354397_i64 + 3553511668143831924_i64;
place!(Field::<*mut *mut i16>(Variant(_9, 1), 5)) = core::ptr::addr_of_mut!(_21.1.1);
_11 = (_5, _10, _2, _6);
_13 = '\u{50f6a}';
place!(Field::<*const *const char>(Variant(_9, 1), 2)) = core::ptr::addr_of!(place!(Field::<*const char>(Variant(_14, 3), 1)));
place!(Field::<[usize; 5]>(Variant(_14, 3), 0)) = [Field::<usize>(Variant(_9, 1), 3),Field::<usize>(Variant(_9, 1), 3),Field::<usize>(Variant(_9, 1), 3),Field::<usize>(Variant(_9, 1), 3),Field::<usize>(Variant(_9, 1), 3)];
place!(Field::<u8>(Variant(place!(Field::<Adt44>(Variant(_9, 1), 4)), 2), 4)) = !103_u8;
place!(Field::<u8>(Variant(place!(Field::<Adt44>(Variant(_9, 1), 4)), 2), 4)) = 77_u8 ^ 43_u8;
place!(Field::<u64>(Variant(place!(Field::<Adt44>(Variant(_9, 1), 4)), 2), 1)) = !Field::<u64>(Variant(_14, 3), 4);
match _15 {
0 => bb9,
340282366920938463454151235394913435648 => bb12,
_ => bb10
}
}
bb14 = {
_11.2 = _6 <= _6;
_10 = [52819_u16,802_u16,60977_u16,53178_u16];
_1 = !96553612728553891071983601806886886207_u128;
_7 = [_11.3];
_15 = (-9223372036854775808_isize);
_6 = '\u{be1b2}' as i32;
place!(Field::<i128>(Variant(_9, 1), 7)) = !(-90927896985593802922462226411721845452_i128);
place!(Field::<Adt44>(Variant(_9, 1), 4)) = Adt44::Variant2 { fld0: Field::<i128>(Variant(_9, 1), 7),fld1: 3706004786732294610_u64,fld2: _15,fld3: 92_i8,fld4: 108_u8 };
place!(Field::<Adt44>(Variant(_9, 1), 4)) = Adt44::Variant2 { fld0: Field::<i128>(Variant(_9, 1), 7),fld1: 13602405612807407168_u64,fld2: _15,fld3: 28_i8,fld4: 107_u8 };
place!(Field::<usize>(Variant(_9, 1), 3)) = !10054150135458457431_usize;
place!(Field::<i8>(Variant(place!(Field::<Adt44>(Variant(_9, 1), 4)), 2), 3)) = (-116_i8) | 94_i8;
_16 = ['\u{10b221}','\u{109944}','\u{92174}'];
_11.0 = [(-1627535680331433675_i64),(-316946524784451633_i64),2387526396847519036_i64,(-570806450920256594_i64),5414715706630922253_i64];
_17 = _15 ^ _15;
_11.2 = !_2;
place!(Field::<u64>(Variant(place!(Field::<Adt44>(Variant(_9, 1), 4)), 2), 1)) = _6 as u64;
_17 = -Field::<isize>(Variant(Field::<Adt44>(Variant(_9, 1), 4), 2), 2);
Call(_14 = fn3(_16, _5, _11.1, _11, _17, _11.1), ReturnTo(bb11), UnwindUnreachable())
}
bb15 = {
Return()
}
bb16 = {
_6 = 530881412_i32;
RET = 2293489277_u32;
_9 = Adt55::Variant2 { fld0: _2,fld1: 105_isize };
_5 = _3;
_1 = 26673150220288208622074177705995065918_u128;
Goto(bb9)
}
bb17 = {
Return()
}
bb18 = {
_23 = Field::<isize>(Variant(_9, 2), 1) as i64;
_27 = (_11.0, _10, Field::<bool>(Variant(_9, 2), 0), _11.3);
_15 = Field::<isize>(Variant(_14, 3), 2);
_15 = Field::<isize>(Variant(_14, 3), 2);
_21.0 = _23;
_26 = Field::<f32>(Variant(_14, 3), 6);
_21.0 = (-28_i8) as i64;
_11.3 = 1_usize as i32;
_7 = [_11.3];
_5 = _3;
_4 = (-29935_i16) ^ 15689_i16;
place!(Field::<*const char>(Variant(_14, 3), 1)) = core::ptr::addr_of!(_13);
place!(Field::<bool>(Variant(_9, 2), 0)) = _27.2;
_12 = &_4;
_29.fld2.0 = RET as i64;
_30 = [3581404818691393185_usize,4_usize,11597075831547033276_usize,2_usize,16780120972342481181_usize];
_1 = 106777400168834445310568360043792256683_u128;
_27.1 = [27397_u16,47776_u16,3268_u16,7066_u16];
_11.0 = [_23,_29.fld2.0,_21.0,_29.fld2.0,_29.fld2.0];
_27.1 = [6190_u16,33078_u16,34005_u16,24354_u16];
_19 = !RET;
_29.fld2.1 = _21.1;
Goto(bb19)
}
bb19 = {
Call(_32 = dump_var(1_usize, 15_usize, Move(_15), 4_usize, Move(_4), 13_usize, Move(_13), 10_usize, Move(_10)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_32 = dump_var(1_usize, 7_usize, Move(_7), 27_usize, Move(_27), 17_usize, Move(_17), 16_usize, Move(_16)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_32 = dump_var(1_usize, 11_usize, Move(_11), 33_usize, _33, 33_usize, _33, 33_usize, _33), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: bool,mut _2: bool,mut _3: u32,mut _4: [i64; 5],mut _5: bool,mut _6: [i64; 5],mut _7: u32,mut _8: bool,mut _9: u32,mut _10: i16,mut _11: u32,mut _12: [i64; 5]) -> u128 {
mir! {
type RET = u128;
let _13: (char, (u32, (*mut u32, *mut i16), i128));
let _14: i64;
let _15: ([i64; 5], [u16; 4], bool, i32);
let _16: Adt46;
let _17: [u16; 4];
let _18: [u128; 4];
let _19: [i64; 5];
let _20: i128;
let _21: f32;
let _22: u8;
let _23: f64;
let _24: [i64; 5];
let _25: Adt58;
let _26: bool;
let _27: Adt45;
let _28: isize;
let _29: i64;
let _30: [i32; 1];
let _31: f32;
let _32: i8;
let _33: ();
let _34: ();
{
_7 = !_11;
_7 = !_9;
_6 = _4;
RET = 308198965009425164833685317299510062460_u128 >> _3;
_9 = _11 >> _7;
_10 = RET as i16;
RET = 42675488310180035353280008068055414739_u128 - 84092386266655177271762233912884069236_u128;
_10 = (-2444_i16) & (-26743_i16);
_13.1.1.1 = core::ptr::addr_of_mut!(_10);
_7 = _9;
_3 = _5 as u32;
_3 = !_11;
_3 = !_7;
_1 = !_2;
_15.0 = [(-1832174136427400280_i64),6789944830979105367_i64,(-2397025038371015783_i64),5481445940874563754_i64,2872101625479677094_i64];
_10 = 17829_i16;
_13.1.1.0 = core::ptr::addr_of_mut!(_13.1.0);
RET = 311587170327951949461813086800406805350_u128;
Call(_11 = core::intrinsics::bswap(_3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_3 = _7 << _11;
_16 = Adt46::Variant3 { fld0: _15.0 };
_13.0 = '\u{50b5}';
_15.2 = _1;
_13.1.0 = !_3;
_8 = _9 == _13.1.0;
_14 = (-3952204580737716864_i64);
match _10 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
17829 => bb7,
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
_6 = [_14,_14,_14,_14,_14];
_13.1.1.0 = core::ptr::addr_of_mut!(_13.1.0);
_13.1.0 = _7 * _3;
_1 = !_8;
_2 = !_1;
_11 = _9 + _9;
_13.1.1.1 = core::ptr::addr_of_mut!(_10);
RET = !75182323993826381631544325765301069139_u128;
_2 = _8 | _1;
_15.0 = _6;
RET = 94446958122922707280467243488137655078_u128;
_1 = _2;
_15.2 = _1;
RET = 229207027937692613463916699810582467337_u128;
_3 = _13.1.0;
_15.3 = (-304506677_i32);
Goto(bb8)
}
bb8 = {
_17 = [61673_u16,56715_u16,22195_u16,29263_u16];
_9 = 48_isize as u32;
_15.1 = [48679_u16,44267_u16,50315_u16,35588_u16];
_10 = RET as i16;
SetDiscriminant(_16, 1);
_14 = (-6844396189699653406_i64);
place!(Field::<[usize; 5]>(Variant(_16, 1), 0)) = [4380525171025969335_usize,1_usize,7_usize,13256335085095778895_usize,4_usize];
_20 = -(-95062966817959941848288237129639702553_i128);
_9 = _3;
_20 = (-168787426182926482996417262238941013038_i128);
_13.1.2 = _20 + _20;
match _14 {
340282366920938463456530211242068558050 => bb10,
_ => bb9
}
}
bb9 = {
_6 = [_14,_14,_14,_14,_14];
_13.1.1.0 = core::ptr::addr_of_mut!(_13.1.0);
_13.1.0 = _7 * _3;
_1 = !_8;
_2 = !_1;
_11 = _9 + _9;
_13.1.1.1 = core::ptr::addr_of_mut!(_10);
RET = !75182323993826381631544325765301069139_u128;
_2 = _8 | _1;
_15.0 = _6;
RET = 94446958122922707280467243488137655078_u128;
_1 = _2;
_15.2 = _1;
RET = 229207027937692613463916699810582467337_u128;
_3 = _13.1.0;
_15.3 = (-304506677_i32);
Goto(bb8)
}
bb10 = {
_12 = [_14,_14,_14,_14,_14];
_15.2 = _2 > _1;
_20 = _13.1.2 - _13.1.2;
_13.1.1.1 = core::ptr::addr_of_mut!(_10);
_15.1 = _17;
_15.3 = (-1693600091_i32) | (-773783764_i32);
_15.0 = [_14,_14,_14,_14,_14];
_15.0 = [_14,_14,_14,_14,_14];
_22 = _13.1.2 as u8;
_13.1.1.0 = core::ptr::addr_of_mut!(_9);
_23 = 18144522431354483373_usize as f64;
_22 = _23 as u8;
_21 = 5120420258712086065_u64 as f32;
_13.1.1.1 = core::ptr::addr_of_mut!(_10);
_19 = [_14,_14,_14,_14,_14];
_10 = !7551_i16;
_15.1 = _17;
_12 = _4;
_13.1.1.1 = core::ptr::addr_of_mut!(_10);
_18 = [RET,RET,RET,RET];
_24 = [_14,_14,_14,_14,_14];
match RET {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb7,
229207027937692613463916699810582467337 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_15.1 = _17;
_10 = 18135_i16 + (-30562_i16);
Goto(bb13)
}
bb13 = {
RET = 59772890086364704662994520975554316060_u128 ^ 189715231130269420262016497094953139804_u128;
_21 = 9223372036854775807_isize as f32;
_27.fld3 = core::ptr::addr_of_mut!(_13.1.2);
SetDiscriminant(_16, 2);
place!(Field::<(i64, (*mut u32, *mut i16))>(Variant(_16, 2), 0)).1.0 = core::ptr::addr_of_mut!(_11);
place!(Field::<(i64, (*mut u32, *mut i16))>(Variant(_16, 2), 0)).1 = (_13.1.1.0, _13.1.1.1);
place!(Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_16, 2), 2)).2 = _15.2 & _1;
_27.fld3 = core::ptr::addr_of_mut!(_13.1.2);
_26 = Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_16, 2), 2).2 & _15.2;
place!(Field::<(i64, (*mut u32, *mut i16))>(Variant(_16, 2), 0)) = (_14, _13.1.1);
place!(Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_16, 2), 2)).1 = 16109_u16 ^ 50915_u16;
place!(Field::<(i64, (*mut u32, *mut i16))>(Variant(_16, 2), 0)).1.1 = core::ptr::addr_of_mut!(_10);
place!(Field::<char>(Variant(_16, 2), 1)) = _13.0;
place!(Field::<char>(Variant(_16, 2), 1)) = _13.0;
_28 = 9223372036854775807_isize << _13.1.0;
_8 = _15.2 > _2;
_24 = _19;
place!(Field::<(i64, (*mut u32, *mut i16))>(Variant(_16, 2), 0)).1.1 = _13.1.1.1;
RET = 85194383276779326259350019192408872371_u128;
match RET {
0 => bb3,
85194383276779326259350019192408872371 => bb14,
_ => bb11
}
}
bb14 = {
place!(Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_16, 2), 2)) = (_23, 50083_u16, _26, _23, _13.1);
place!(Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_16, 2), 2)).4 = (_13.1.0, _13.1.1, _20);
_13.1.1.0 = Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_16, 2), 2).4.1.0;
_14 = Field::<(i64, (*mut u32, *mut i16))>(Variant(_16, 2), 0).0 ^ Field::<(i64, (*mut u32, *mut i16))>(Variant(_16, 2), 0).0;
_27.fld5 = -_21;
_17 = [Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_16, 2), 2).1,Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_16, 2), 2).1,Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_16, 2), 2).1,Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_16, 2), 2).1];
_3 = _11 ^ _11;
SetDiscriminant(_16, 0);
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(2_usize, 26_usize, Move(_26), 6_usize, Move(_6), 11_usize, Move(_11), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(2_usize, 12_usize, Move(_12), 22_usize, Move(_22), 17_usize, Move(_17), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(2_usize, 5_usize, Move(_5), 18_usize, Move(_18), 28_usize, Move(_28), 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: [char; 3],mut _2: [i64; 5],mut _3: [u16; 4],mut _4: ([i64; 5], [u16; 4], bool, i32),mut _5: isize,mut _6: [u16; 4]) -> Adt44 {
mir! {
type RET = Adt44;
let _7: char;
let _8: i64;
let _9: [u16; 4];
let _10: u16;
let _11: (char, (u32, (*mut u32, *mut i16), i128));
let _12: Adt50;
let _13: [char; 3];
let _14: isize;
let _15: u64;
let _16: bool;
let _17: Adt44;
let _18: *mut *mut i16;
let _19: isize;
let _20: bool;
let _21: [char; 3];
let _22: u128;
let _23: Adt53;
let _24: Adt57;
let _25: [usize; 5];
let _26: *mut u32;
let _27: ([i64; 5], [u16; 4], bool, i32);
let _28: (*const (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),);
let _29: [u64; 2];
let _30: u128;
let _31: [u128; 4];
let _32: [i64; 5];
let _33: f64;
let _34: [i32; 1];
let _35: *mut u32;
let _36: bool;
let _37: i128;
let _38: Adt48;
let _39: ([i64; 5], [u16; 4], bool, i32);
let _40: char;
let _41: bool;
let _42: usize;
let _43: char;
let _44: Adt52;
let _45: Adt52;
let _46: i128;
let _47: [i32; 1];
let _48: [i64; 5];
let _49: i64;
let _50: f64;
let _51: isize;
let _52: isize;
let _53: f32;
let _54: f32;
let _55: u128;
let _56: char;
let _57: *const [usize; 5];
let _58: i64;
let _59: [i32; 1];
let _60: ();
let _61: ();
{
_7 = '\u{be3f1}';
_1 = [_7,_7,_7];
_8 = 14143_u16 as i64;
RET = Adt44::Variant2 { fld0: (-113663432205792350842252735220006399992_i128),fld1: 10308321951421965962_u64,fld2: _5,fld3: (-53_i8),fld4: 218_u8 };
Goto(bb1)
}
bb1 = {
RET = Adt44::Variant2 { fld0: (-83000458336367370065562002437571590612_i128),fld1: 2666621103983038844_u64,fld2: _5,fld3: (-99_i8),fld4: 131_u8 };
_8 = 5216791176191372597_i64 << _5;
_4.1 = [35541_u16,58098_u16,16017_u16,61837_u16];
_4.2 = false;
_11.1.2 = _4.3 as i128;
_11.0 = _7;
place!(Field::<i8>(Variant(RET, 2), 3)) = !(-101_i8);
place!(Field::<u64>(Variant(RET, 2), 1)) = 14715218596053851979_u64;
_12.fld4 = _11.1.2 >> _5;
place!(Field::<i128>(Variant(RET, 2), 0)) = _12.fld4;
_6 = [46976_u16,9121_u16,3825_u16,21946_u16];
_10 = 21018_u16 * 28575_u16;
_2 = _4.0;
_4.1 = _6;
place!(Field::<isize>(Variant(RET, 2), 2)) = _5;
place!(Field::<u8>(Variant(RET, 2), 4)) = !165_u8;
_10 = _4.3 as u16;
_11.0 = _7;
_4.2 = true;
_12.fld2.1.0 = core::ptr::addr_of_mut!(_11.1.0);
Call(_4.0 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = _3;
_2 = _4.0;
place!(Field::<i128>(Variant(RET, 2), 0)) = _12.fld4;
_9 = [_10,_10,_10,_10];
_4 = (_2, _6, true, 1694264059_i32);
_3 = [_10,_10,_10,_10];
_12.fld2.0 = _8;
place!(Field::<i8>(Variant(RET, 2), 3)) = Field::<u8>(Variant(RET, 2), 4) as i8;
_14 = -Field::<isize>(Variant(RET, 2), 2);
SetDiscriminant(RET, 3);
_11.1.1.0 = core::ptr::addr_of_mut!(_11.1.0);
Goto(bb3)
}
bb3 = {
_11.1.0 = _14 as u32;
place!(Field::<*const [usize; 5]>(Variant(RET, 3), 3)) = core::ptr::addr_of!(place!(Field::<[usize; 5]>(Variant(RET, 3), 0)));
place!(Field::<*const char>(Variant(RET, 3), 1)) = core::ptr::addr_of!(_7);
_11.1.1.0 = core::ptr::addr_of_mut!(_11.1.0);
place!(Field::<isize>(Variant(RET, 3), 2)) = _14 << _8;
place!(Field::<[usize; 5]>(Variant(RET, 3), 0)) = [3_usize,3_usize,1_usize,7987533281541914784_usize,703196411921244881_usize];
_2 = [_12.fld2.0,_12.fld2.0,_8,_8,_8];
_13 = _1;
_4.0 = [_12.fld2.0,_8,_8,_12.fld2.0,_12.fld2.0];
match _4.3 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
1694264059 => bb9,
_ => bb8
}
}
bb4 = {
_6 = _3;
_2 = _4.0;
place!(Field::<i128>(Variant(RET, 2), 0)) = _12.fld4;
_9 = [_10,_10,_10,_10];
_4 = (_2, _6, true, 1694264059_i32);
_3 = [_10,_10,_10,_10];
_12.fld2.0 = _8;
place!(Field::<i8>(Variant(RET, 2), 3)) = Field::<u8>(Variant(RET, 2), 4) as i8;
_14 = -Field::<isize>(Variant(RET, 2), 2);
SetDiscriminant(RET, 3);
_11.1.1.0 = core::ptr::addr_of_mut!(_11.1.0);
Goto(bb3)
}
bb5 = {
RET = Adt44::Variant2 { fld0: (-83000458336367370065562002437571590612_i128),fld1: 2666621103983038844_u64,fld2: _5,fld3: (-99_i8),fld4: 131_u8 };
_8 = 5216791176191372597_i64 << _5;
_4.1 = [35541_u16,58098_u16,16017_u16,61837_u16];
_4.2 = false;
_11.1.2 = _4.3 as i128;
_11.0 = _7;
place!(Field::<i8>(Variant(RET, 2), 3)) = !(-101_i8);
place!(Field::<u64>(Variant(RET, 2), 1)) = 14715218596053851979_u64;
_12.fld4 = _11.1.2 >> _5;
place!(Field::<i128>(Variant(RET, 2), 0)) = _12.fld4;
_6 = [46976_u16,9121_u16,3825_u16,21946_u16];
_10 = 21018_u16 * 28575_u16;
_2 = _4.0;
_4.1 = _6;
place!(Field::<isize>(Variant(RET, 2), 2)) = _5;
place!(Field::<u8>(Variant(RET, 2), 4)) = !165_u8;
_10 = _4.3 as u16;
_11.0 = _7;
_4.2 = true;
_12.fld2.1.0 = core::ptr::addr_of_mut!(_11.1.0);
Call(_4.0 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
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
place!(Field::<u64>(Variant(RET, 3), 4)) = _12.fld4 as u64;
_4.2 = true;
_15 = 1434_i16 as u64;
place!(Field::<*const [usize; 5]>(Variant(RET, 3), 3)) = core::ptr::addr_of!(place!(Field::<[usize; 5]>(Variant(RET, 3), 0)));
_14 = -Field::<isize>(Variant(RET, 3), 2);
place!(Field::<[char; 3]>(Variant(RET, 3), 5)) = [_7,_7,_11.0];
place!(Field::<[char; 3]>(Variant(RET, 3), 5)) = [_7,_7,_11.0];
_2 = _4.0;
place!(Field::<*const [usize; 5]>(Variant(RET, 3), 3)) = core::ptr::addr_of!(place!(Field::<[usize; 5]>(Variant(RET, 3), 0)));
_4.3 = 957523069_i32;
place!(Field::<f32>(Variant(RET, 3), 6)) = Field::<u64>(Variant(RET, 3), 4) as f32;
_4.0 = [_12.fld2.0,_12.fld2.0,_12.fld2.0,_8,_12.fld2.0];
_3 = _6;
_4.1 = _3;
_6 = [_10,_10,_10,_10];
_4 = (_2, _3, true, (-1992120477_i32));
_15 = !Field::<u64>(Variant(RET, 3), 4);
_4.0 = [_8,_12.fld2.0,_8,_12.fld2.0,_12.fld2.0];
place!(Field::<u64>(Variant(RET, 3), 4)) = !_15;
_15 = !Field::<u64>(Variant(RET, 3), 4);
_8 = !_12.fld2.0;
_11.1.1.0 = _12.fld2.1.0;
SetDiscriminant(RET, 0);
_12.fld4 = _11.1.2 - _11.1.2;
match _4.3 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607429776090979 => bb11,
_ => bb10
}
}
bb10 = {
RET = Adt44::Variant2 { fld0: (-83000458336367370065562002437571590612_i128),fld1: 2666621103983038844_u64,fld2: _5,fld3: (-99_i8),fld4: 131_u8 };
_8 = 5216791176191372597_i64 << _5;
_4.1 = [35541_u16,58098_u16,16017_u16,61837_u16];
_4.2 = false;
_11.1.2 = _4.3 as i128;
_11.0 = _7;
place!(Field::<i8>(Variant(RET, 2), 3)) = !(-101_i8);
place!(Field::<u64>(Variant(RET, 2), 1)) = 14715218596053851979_u64;
_12.fld4 = _11.1.2 >> _5;
place!(Field::<i128>(Variant(RET, 2), 0)) = _12.fld4;
_6 = [46976_u16,9121_u16,3825_u16,21946_u16];
_10 = 21018_u16 * 28575_u16;
_2 = _4.0;
_4.1 = _6;
place!(Field::<isize>(Variant(RET, 2), 2)) = _5;
place!(Field::<u8>(Variant(RET, 2), 4)) = !165_u8;
_10 = _4.3 as u16;
_11.0 = _7;
_4.2 = true;
_12.fld2.1.0 = core::ptr::addr_of_mut!(_11.1.0);
Call(_4.0 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_2 = [_12.fld2.0,_8,_8,_12.fld2.0,_12.fld2.0];
_16 = !_4.2;
_8 = _12.fld2.0;
_4.1 = _3;
_16 = _4.2;
_7 = _11.0;
_12.fld2.1.0 = core::ptr::addr_of_mut!(_11.1.0);
_12.fld4 = !_11.1.2;
_13 = [_11.0,_7,_7];
_16 = _4.2 & _4.2;
Call(_12.fld3 = fn4(_7, _2, _7, _4, _16, _4.3, _4.2, _16, _14, _16, _11.1.1.0, _16, _14, _4.3), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
place!(Field::<u32>(Variant(RET, 0), 0)) = _11.1.0 - _11.1.0;
_12.fld2.1.0 = core::ptr::addr_of_mut!(_11.1.0);
_8 = _12.fld2.0 >> _11.1.2;
_4 = (_2, _6, _16, (-979875470_i32));
_7 = _11.0;
_4 = (_2, _3, _16, (-1532683338_i32));
_15 = _7 as u64;
_6 = [_10,_10,_10,_10];
place!(Field::<u32>(Variant(RET, 0), 0)) = !_11.1.0;
_25 = [2_usize,3652653892606988480_usize,7_usize,0_usize,0_usize];
_26 = core::ptr::addr_of_mut!(_11.1.0);
_27.3 = -_4.3;
_20 = !_16;
_7 = _11.0;
Call(place!(Field::<u32>(Variant(RET, 0), 0)) = fn5(_4.3, _4.2, _14, _4.3, _27.3, _4, _16, _4.3, _4, _4.2, _4.3, _4.2, _13, _4.2, _26), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_6 = [_10,_10,_10,_10];
_9 = [_10,_10,_10,_10];
_6 = [_10,_10,_10,_10];
_27.0 = _2;
_12.fld2.1.0 = core::ptr::addr_of_mut!((*_26));
place!(Field::<u32>(Variant(RET, 0), 0)) = (-23657_i16) as u32;
_13 = [_11.0,_11.0,_7];
_7 = _11.0;
_13 = _1;
_7 = _11.0;
_8 = _12.fld2.0 + _12.fld2.0;
_27 = _4;
_25 = [9199712987339935079_usize,3_usize,3176385159075471192_usize,7_usize,3_usize];
_27.1 = _4.1;
_14 = -_5;
_3 = [_10,_10,_10,_10];
_6 = [_10,_10,_10,_10];
_1 = _13;
_27.2 = _16 <= _16;
(*_26) = _12.fld4 as u32;
(*_26) = !Field::<u32>(Variant(RET, 0), 0);
_4.3 = _12.fld3 as i32;
_11.1.2 = !_12.fld4;
_11.1.0 = Field::<u32>(Variant(RET, 0), 0);
_8 = _15 as i64;
_11.0 = _7;
_12.fld4 = _11.1.2 ^ _11.1.2;
_21 = _13;
_9 = [_10,_10,_10,_10];
Goto(bb14)
}
bb14 = {
_22 = 331913435410514156892795615208635453681_u128;
_4.0 = [_8,_12.fld2.0,_8,_12.fld2.0,_12.fld2.0];
_4.2 = _27.2;
_4.1 = [_10,_10,_10,_10];
place!(Field::<u32>(Variant(RET, 0), 0)) = (*_26) | _11.1.0;
match _27.3 {
0 => bb15,
340282366920938463463374607430235528118 => bb17,
_ => bb16
}
}
bb15 = {
_2 = [_12.fld2.0,_8,_8,_12.fld2.0,_12.fld2.0];
_16 = !_4.2;
_8 = _12.fld2.0;
_4.1 = _3;
_16 = _4.2;
_7 = _11.0;
_12.fld2.1.0 = core::ptr::addr_of_mut!(_11.1.0);
_12.fld4 = !_11.1.2;
_13 = [_11.0,_7,_7];
_16 = _4.2 & _4.2;
Call(_12.fld3 = fn4(_7, _2, _7, _4, _16, _4.3, _4.2, _16, _14, _16, _11.1.1.0, _16, _14, _4.3), ReturnTo(bb12), UnwindUnreachable())
}
bb16 = {
place!(Field::<u32>(Variant(RET, 0), 0)) = _11.1.0 - _11.1.0;
_12.fld2.1.0 = core::ptr::addr_of_mut!(_11.1.0);
_8 = _12.fld2.0 >> _11.1.2;
_4 = (_2, _6, _16, (-979875470_i32));
_7 = _11.0;
_4 = (_2, _3, _16, (-1532683338_i32));
_15 = _7 as u64;
_6 = [_10,_10,_10,_10];
place!(Field::<u32>(Variant(RET, 0), 0)) = !_11.1.0;
_25 = [2_usize,3652653892606988480_usize,7_usize,0_usize,0_usize];
_26 = core::ptr::addr_of_mut!(_11.1.0);
_27.3 = -_4.3;
_20 = !_16;
_7 = _11.0;
Call(place!(Field::<u32>(Variant(RET, 0), 0)) = fn5(_4.3, _4.2, _14, _4.3, _27.3, _4, _16, _4.3, _4, _4.2, _4.3, _4.2, _13, _4.2, _26), ReturnTo(bb13), UnwindUnreachable())
}
bb17 = {
_25 = [16820904976001500026_usize,1078442045716031504_usize,15219168947660892782_usize,14827472316776671276_usize,2_usize];
_27.3 = -_4.3;
_4.3 = 3_usize as i32;
_20 = !_16;
_11.1.2 = _10 as i128;
_19 = _5 >> _12.fld4;
Call(_11.1 = fn6(_20, _26, _27.0, _12.fld2.1.0, _4, _4.2, _11.0, _27.2, _4.2, _4.2, _21, _27, _4, _27, _27.2), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
_12.fld2 = (_8, _11.1.1);
_30 = _15 as u128;
_12.fld2.1.1 = _11.1.1.1;
place!(Field::<u32>(Variant(RET, 0), 0)) = !_11.1.0;
_32 = [_12.fld2.0,_8,_12.fld2.0,_8,_8];
_12.fld2 = (_8, _11.1.1);
_4.3 = _27.3 & _27.3;
_4.2 = !_16;
match _22 {
0 => bb1,
1 => bb19,
331913435410514156892795615208635453681 => bb21,
_ => bb20
}
}
bb19 = {
Return()
}
bb20 = {
RET = Adt44::Variant2 { fld0: (-83000458336367370065562002437571590612_i128),fld1: 2666621103983038844_u64,fld2: _5,fld3: (-99_i8),fld4: 131_u8 };
_8 = 5216791176191372597_i64 << _5;
_4.1 = [35541_u16,58098_u16,16017_u16,61837_u16];
_4.2 = false;
_11.1.2 = _4.3 as i128;
_11.0 = _7;
place!(Field::<i8>(Variant(RET, 2), 3)) = !(-101_i8);
place!(Field::<u64>(Variant(RET, 2), 1)) = 14715218596053851979_u64;
_12.fld4 = _11.1.2 >> _5;
place!(Field::<i128>(Variant(RET, 2), 0)) = _12.fld4;
_6 = [46976_u16,9121_u16,3825_u16,21946_u16];
_10 = 21018_u16 * 28575_u16;
_2 = _4.0;
_4.1 = _6;
place!(Field::<isize>(Variant(RET, 2), 2)) = _5;
place!(Field::<u8>(Variant(RET, 2), 4)) = !165_u8;
_10 = _4.3 as u16;
_11.0 = _7;
_4.2 = true;
_12.fld2.1.0 = core::ptr::addr_of_mut!(_11.1.0);
Call(_4.0 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb21 = {
_12.fld2.1 = _11.1.1;
_4.0 = [_8,_12.fld2.0,_12.fld2.0,_8,_12.fld2.0];
_7 = _11.0;
(*_26) = Field::<u32>(Variant(RET, 0), 0);
match _22 {
0 => bb22,
1 => bb23,
2 => bb24,
3 => bb25,
4 => bb26,
5 => bb27,
6 => bb28,
331913435410514156892795615208635453681 => bb30,
_ => bb29
}
}
bb22 = {
RET = Adt44::Variant2 { fld0: (-83000458336367370065562002437571590612_i128),fld1: 2666621103983038844_u64,fld2: _5,fld3: (-99_i8),fld4: 131_u8 };
_8 = 5216791176191372597_i64 << _5;
_4.1 = [35541_u16,58098_u16,16017_u16,61837_u16];
_4.2 = false;
_11.1.2 = _4.3 as i128;
_11.0 = _7;
place!(Field::<i8>(Variant(RET, 2), 3)) = !(-101_i8);
place!(Field::<u64>(Variant(RET, 2), 1)) = 14715218596053851979_u64;
_12.fld4 = _11.1.2 >> _5;
place!(Field::<i128>(Variant(RET, 2), 0)) = _12.fld4;
_6 = [46976_u16,9121_u16,3825_u16,21946_u16];
_10 = 21018_u16 * 28575_u16;
_2 = _4.0;
_4.1 = _6;
place!(Field::<isize>(Variant(RET, 2), 2)) = _5;
place!(Field::<u8>(Variant(RET, 2), 4)) = !165_u8;
_10 = _4.3 as u16;
_11.0 = _7;
_4.2 = true;
_12.fld2.1.0 = core::ptr::addr_of_mut!(_11.1.0);
Call(_4.0 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb23 = {
Return()
}
bb24 = {
_12.fld2 = (_8, _11.1.1);
_30 = _15 as u128;
_12.fld2.1.1 = _11.1.1.1;
place!(Field::<u32>(Variant(RET, 0), 0)) = !_11.1.0;
_32 = [_12.fld2.0,_8,_12.fld2.0,_8,_8];
_12.fld2 = (_8, _11.1.1);
_4.3 = _27.3 & _27.3;
_4.2 = !_16;
match _22 {
0 => bb1,
1 => bb19,
331913435410514156892795615208635453681 => bb21,
_ => bb20
}
}
bb25 = {
_25 = [16820904976001500026_usize,1078442045716031504_usize,15219168947660892782_usize,14827472316776671276_usize,2_usize];
_27.3 = -_4.3;
_4.3 = 3_usize as i32;
_20 = !_16;
_11.1.2 = _10 as i128;
_19 = _5 >> _12.fld4;
Call(_11.1 = fn6(_20, _26, _27.0, _12.fld2.1.0, _4, _4.2, _11.0, _27.2, _4.2, _4.2, _21, _27, _4, _27, _27.2), ReturnTo(bb18), UnwindUnreachable())
}
bb26 = {
place!(Field::<u32>(Variant(RET, 0), 0)) = _11.1.0 - _11.1.0;
_12.fld2.1.0 = core::ptr::addr_of_mut!(_11.1.0);
_8 = _12.fld2.0 >> _11.1.2;
_4 = (_2, _6, _16, (-979875470_i32));
_7 = _11.0;
_4 = (_2, _3, _16, (-1532683338_i32));
_15 = _7 as u64;
_6 = [_10,_10,_10,_10];
place!(Field::<u32>(Variant(RET, 0), 0)) = !_11.1.0;
_25 = [2_usize,3652653892606988480_usize,7_usize,0_usize,0_usize];
_26 = core::ptr::addr_of_mut!(_11.1.0);
_27.3 = -_4.3;
_20 = !_16;
_7 = _11.0;
Call(place!(Field::<u32>(Variant(RET, 0), 0)) = fn5(_4.3, _4.2, _14, _4.3, _27.3, _4, _16, _4.3, _4, _4.2, _4.3, _4.2, _13, _4.2, _26), ReturnTo(bb13), UnwindUnreachable())
}
bb27 = {
place!(Field::<u64>(Variant(RET, 3), 4)) = _12.fld4 as u64;
_4.2 = true;
_15 = 1434_i16 as u64;
place!(Field::<*const [usize; 5]>(Variant(RET, 3), 3)) = core::ptr::addr_of!(place!(Field::<[usize; 5]>(Variant(RET, 3), 0)));
_14 = -Field::<isize>(Variant(RET, 3), 2);
place!(Field::<[char; 3]>(Variant(RET, 3), 5)) = [_7,_7,_11.0];
place!(Field::<[char; 3]>(Variant(RET, 3), 5)) = [_7,_7,_11.0];
_2 = _4.0;
place!(Field::<*const [usize; 5]>(Variant(RET, 3), 3)) = core::ptr::addr_of!(place!(Field::<[usize; 5]>(Variant(RET, 3), 0)));
_4.3 = 957523069_i32;
place!(Field::<f32>(Variant(RET, 3), 6)) = Field::<u64>(Variant(RET, 3), 4) as f32;
_4.0 = [_12.fld2.0,_12.fld2.0,_12.fld2.0,_8,_12.fld2.0];
_3 = _6;
_4.1 = _3;
_6 = [_10,_10,_10,_10];
_4 = (_2, _3, true, (-1992120477_i32));
_15 = !Field::<u64>(Variant(RET, 3), 4);
_4.0 = [_8,_12.fld2.0,_8,_12.fld2.0,_12.fld2.0];
place!(Field::<u64>(Variant(RET, 3), 4)) = !_15;
_15 = !Field::<u64>(Variant(RET, 3), 4);
_8 = !_12.fld2.0;
_11.1.1.0 = _12.fld2.1.0;
SetDiscriminant(RET, 0);
_12.fld4 = _11.1.2 - _11.1.2;
match _4.3 {
0 => bb1,
1 => bb2,
2 => bb8,
3 => bb4,
4 => bb5,
5 => bb6,
6 => bb7,
340282366920938463463374607429776090979 => bb11,
_ => bb10
}
}
bb28 = {
_2 = [_12.fld2.0,_8,_8,_12.fld2.0,_12.fld2.0];
_16 = !_4.2;
_8 = _12.fld2.0;
_4.1 = _3;
_16 = _4.2;
_7 = _11.0;
_12.fld2.1.0 = core::ptr::addr_of_mut!(_11.1.0);
_12.fld4 = !_11.1.2;
_13 = [_11.0,_7,_7];
_16 = _4.2 & _4.2;
Call(_12.fld3 = fn4(_7, _2, _7, _4, _16, _4.3, _4.2, _16, _14, _16, _11.1.1.0, _16, _14, _4.3), ReturnTo(bb12), UnwindUnreachable())
}
bb29 = {
_6 = [_10,_10,_10,_10];
_9 = [_10,_10,_10,_10];
_6 = [_10,_10,_10,_10];
_27.0 = _2;
_12.fld2.1.0 = core::ptr::addr_of_mut!((*_26));
place!(Field::<u32>(Variant(RET, 0), 0)) = (-23657_i16) as u32;
_13 = [_11.0,_11.0,_7];
_7 = _11.0;
_13 = _1;
_7 = _11.0;
_8 = _12.fld2.0 + _12.fld2.0;
_27 = _4;
_25 = [9199712987339935079_usize,3_usize,3176385159075471192_usize,7_usize,3_usize];
_27.1 = _4.1;
_14 = -_5;
_3 = [_10,_10,_10,_10];
_6 = [_10,_10,_10,_10];
_1 = _13;
_27.2 = _16 <= _16;
(*_26) = _12.fld4 as u32;
(*_26) = !Field::<u32>(Variant(RET, 0), 0);
_4.3 = _12.fld3 as i32;
_11.1.2 = !_12.fld4;
_11.1.0 = Field::<u32>(Variant(RET, 0), 0);
_8 = _15 as i64;
_11.0 = _7;
_12.fld4 = _11.1.2 ^ _11.1.2;
_21 = _13;
_9 = [_10,_10,_10,_10];
Goto(bb14)
}
bb30 = {
_12.fld2.0 = _4.3 as i64;
_25 = [2841921030949783493_usize,10181356825118810688_usize,12346620459878946478_usize,3583503551506761051_usize,5_usize];
_35 = _12.fld2.1.0;
_29 = [_15,_15];
_6 = [_10,_10,_10,_10];
_37 = _11.1.2;
_38.fld1 = _7;
_27 = (_4.0, _4.1, _20, _4.3);
_4.2 = !_27.2;
_38.fld3.2.1 = _10;
_38.fld3.2.4 = (Field::<u32>(Variant(RET, 0), 0), _12.fld2.1, _12.fld4);
_38.fld0 = core::ptr::addr_of_mut!(_11.1.2);
_12.fld1 = core::ptr::addr_of!(_38.fld3.2);
_12.fld0.0 = core::ptr::addr_of!(_38.fld6);
_4.2 = !_20;
_8 = _12.fld2.0 ^ _12.fld2.0;
_38.fld6.2 = _4.2;
_38.fld3.2.0 = _38.fld3.2.1 as f64;
_3 = _4.1;
_29 = [_15,_15];
_20 = !_38.fld6.2;
_38.fld3.2.4.1.0 = core::ptr::addr_of_mut!(_11.1.0);
_38.fld6.4.0 = (*_26) ^ Field::<u32>(Variant(RET, 0), 0);
Goto(bb31)
}
bb31 = {
_44.fld7.0.1.1.0 = _11.1.1.0;
_41 = _38.fld3.2.4.0 > (*_26);
_38.fld6.0 = _38.fld3.2.0;
_44.fld7.1.3 = _15 as i32;
_44.fld1 = (_2, _27.1, _41, _4.3);
match _22 {
0 => bb32,
1 => bb33,
2 => bb34,
3 => bb35,
4 => bb36,
331913435410514156892795615208635453681 => bb38,
_ => bb37
}
}
bb32 = {
_22 = 331913435410514156892795615208635453681_u128;
_4.0 = [_8,_12.fld2.0,_8,_12.fld2.0,_12.fld2.0];
_4.2 = _27.2;
_4.1 = [_10,_10,_10,_10];
place!(Field::<u32>(Variant(RET, 0), 0)) = (*_26) | _11.1.0;
match _27.3 {
0 => bb15,
340282366920938463463374607430235528118 => bb17,
_ => bb16
}
}
bb33 = {
_11.1.0 = _14 as u32;
place!(Field::<*const [usize; 5]>(Variant(RET, 3), 3)) = core::ptr::addr_of!(place!(Field::<[usize; 5]>(Variant(RET, 3), 0)));
place!(Field::<*const char>(Variant(RET, 3), 1)) = core::ptr::addr_of!(_7);
_11.1.1.0 = core::ptr::addr_of_mut!(_11.1.0);
place!(Field::<isize>(Variant(RET, 3), 2)) = _14 << _8;
place!(Field::<[usize; 5]>(Variant(RET, 3), 0)) = [3_usize,3_usize,1_usize,7987533281541914784_usize,703196411921244881_usize];
_2 = [_12.fld2.0,_12.fld2.0,_8,_8,_8];
_13 = _1;
_4.0 = [_12.fld2.0,_8,_8,_12.fld2.0,_12.fld2.0];
match _4.3 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
1694264059 => bb9,
_ => bb8
}
}
bb34 = {
Return()
}
bb35 = {
RET = Adt44::Variant2 { fld0: (-83000458336367370065562002437571590612_i128),fld1: 2666621103983038844_u64,fld2: _5,fld3: (-99_i8),fld4: 131_u8 };
_8 = 5216791176191372597_i64 << _5;
_4.1 = [35541_u16,58098_u16,16017_u16,61837_u16];
_4.2 = false;
_11.1.2 = _4.3 as i128;
_11.0 = _7;
place!(Field::<i8>(Variant(RET, 2), 3)) = !(-101_i8);
place!(Field::<u64>(Variant(RET, 2), 1)) = 14715218596053851979_u64;
_12.fld4 = _11.1.2 >> _5;
place!(Field::<i128>(Variant(RET, 2), 0)) = _12.fld4;
_6 = [46976_u16,9121_u16,3825_u16,21946_u16];
_10 = 21018_u16 * 28575_u16;
_2 = _4.0;
_4.1 = _6;
place!(Field::<isize>(Variant(RET, 2), 2)) = _5;
place!(Field::<u8>(Variant(RET, 2), 4)) = !165_u8;
_10 = _4.3 as u16;
_11.0 = _7;
_4.2 = true;
_12.fld2.1.0 = core::ptr::addr_of_mut!(_11.1.0);
Call(_4.0 = core::intrinsics::transmute(_2), ReturnTo(bb2), UnwindUnreachable())
}
bb36 = {
Return()
}
bb37 = {
_6 = [_10,_10,_10,_10];
_9 = [_10,_10,_10,_10];
_6 = [_10,_10,_10,_10];
_27.0 = _2;
_12.fld2.1.0 = core::ptr::addr_of_mut!((*_26));
place!(Field::<u32>(Variant(RET, 0), 0)) = (-23657_i16) as u32;
_13 = [_11.0,_11.0,_7];
_7 = _11.0;
_13 = _1;
_7 = _11.0;
_8 = _12.fld2.0 + _12.fld2.0;
_27 = _4;
_25 = [9199712987339935079_usize,3_usize,3176385159075471192_usize,7_usize,3_usize];
_27.1 = _4.1;
_14 = -_5;
_3 = [_10,_10,_10,_10];
_6 = [_10,_10,_10,_10];
_1 = _13;
_27.2 = _16 <= _16;
(*_26) = _12.fld4 as u32;
(*_26) = !Field::<u32>(Variant(RET, 0), 0);
_4.3 = _12.fld3 as i32;
_11.1.2 = !_12.fld4;
_11.1.0 = Field::<u32>(Variant(RET, 0), 0);
_8 = _15 as i64;
_11.0 = _7;
_12.fld4 = _11.1.2 ^ _11.1.2;
_21 = _13;
_9 = [_10,_10,_10,_10];
Goto(bb14)
}
bb38 = {
_38.fld4 = _38.fld3.2.1;
_38.fld6 = (_38.fld3.2.0, _38.fld3.2.1, _44.fld1.2, _38.fld3.2.0, _38.fld3.2.4);
Call(_44.fld7.2.4.0 = core::intrinsics::transmute(_38.fld3.2.4.0), ReturnTo(bb39), UnwindUnreachable())
}
bb39 = {
_38.fld3.2.4.1 = (_11.1.1.0, _38.fld6.4.1.1);
_44.fld1.2 = _38.fld6.2;
_12.fld0.0 = _12.fld1;
_44.fld7.1.2 = !_41;
_27.1 = _9;
_44.fld7.0 = (_38.fld1, _11.1);
_38.fld3.1.1 = _44.fld1.1;
_45.fld7.0.1.0 = _38.fld3.2.4.0 - _11.1.0;
_44.fld7.2.4.1.1 = _38.fld6.4.1.1;
_38.fld6.4.1.1 = _12.fld2.1.1;
_44.fld7.0.1 = ((*_26), _12.fld2.1, _11.1.2);
_44.fld7.2.4.1.0 = _38.fld6.4.1.0;
_38.fld3.0.0 = _11.0;
_38.fld3.2.4.2 = 15029862211716143607_usize as i128;
_44.fld7.2.4.1.0 = _38.fld3.2.4.1.0;
_45.fld1.3 = _4.2 as i32;
_45.fld1.0 = _4.0;
_44.fld7.2.3 = _38.fld6.0 + _38.fld6.0;
_44.fld7.1.3 = -_45.fld1.3;
_45.fld7.1.2 = Field::<u32>(Variant(RET, 0), 0) != _45.fld7.0.1.0;
_45.fld7.0 = _44.fld7.0;
_11.0 = _38.fld3.0.0;
_38.fld3 = (_44.fld7.0, _44.fld1, _38.fld6);
_45.fld7.1 = (_45.fld1.0, _9, _44.fld1.2, _27.3);
_45.fld6 = Adt44::Variant1 { fld0: _12.fld0.0,fld1: _38.fld1,fld2: Field::<u32>(Variant(RET, 0), 0),fld3: _12.fld3,fld4: _12.fld0,fld5: _38.fld0 };
match _12.fld3 {
0 => bb9,
1 => bb40,
2 => bb41,
340282366920938463463374607431768211331 => bb43,
_ => bb42
}
}
bb40 = {
place!(Field::<u32>(Variant(RET, 0), 0)) = _11.1.0 - _11.1.0;
_12.fld2.1.0 = core::ptr::addr_of_mut!(_11.1.0);
_8 = _12.fld2.0 >> _11.1.2;
_4 = (_2, _6, _16, (-979875470_i32));
_7 = _11.0;
_4 = (_2, _3, _16, (-1532683338_i32));
_15 = _7 as u64;
_6 = [_10,_10,_10,_10];
place!(Field::<u32>(Variant(RET, 0), 0)) = !_11.1.0;
_25 = [2_usize,3652653892606988480_usize,7_usize,0_usize,0_usize];
_26 = core::ptr::addr_of_mut!(_11.1.0);
_27.3 = -_4.3;
_20 = !_16;
_7 = _11.0;
Call(place!(Field::<u32>(Variant(RET, 0), 0)) = fn5(_4.3, _4.2, _14, _4.3, _27.3, _4, _16, _4.3, _4, _4.2, _4.3, _4.2, _13, _4.2, _26), ReturnTo(bb13), UnwindUnreachable())
}
bb41 = {
_2 = [_12.fld2.0,_8,_8,_12.fld2.0,_12.fld2.0];
_16 = !_4.2;
_8 = _12.fld2.0;
_4.1 = _3;
_16 = _4.2;
_7 = _11.0;
_12.fld2.1.0 = core::ptr::addr_of_mut!(_11.1.0);
_12.fld4 = !_11.1.2;
_13 = [_11.0,_7,_7];
_16 = _4.2 & _4.2;
Call(_12.fld3 = fn4(_7, _2, _7, _4, _16, _4.3, _4.2, _16, _14, _16, _11.1.1.0, _16, _14, _4.3), ReturnTo(bb12), UnwindUnreachable())
}
bb42 = {
place!(Field::<u32>(Variant(RET, 0), 0)) = _11.1.0 - _11.1.0;
_12.fld2.1.0 = core::ptr::addr_of_mut!(_11.1.0);
_8 = _12.fld2.0 >> _11.1.2;
_4 = (_2, _6, _16, (-979875470_i32));
_7 = _11.0;
_4 = (_2, _3, _16, (-1532683338_i32));
_15 = _7 as u64;
_6 = [_10,_10,_10,_10];
place!(Field::<u32>(Variant(RET, 0), 0)) = !_11.1.0;
_25 = [2_usize,3652653892606988480_usize,7_usize,0_usize,0_usize];
_26 = core::ptr::addr_of_mut!(_11.1.0);
_27.3 = -_4.3;
_20 = !_16;
_7 = _11.0;
Call(place!(Field::<u32>(Variant(RET, 0), 0)) = fn5(_4.3, _4.2, _14, _4.3, _27.3, _4, _16, _4.3, _4, _4.2, _4.3, _4.2, _13, _4.2, _26), ReturnTo(bb13), UnwindUnreachable())
}
bb43 = {
_27.2 = _41;
_11 = _38.fld3.0;
_12.fld4 = -_11.1.2;
_38.fld3.0.0 = _7;
_45.fld7.2.4.0 = _38.fld3.0.1.0 << _44.fld7.0.1.0;
_28 = (Field::<*const (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_45.fld6, 1), 0),);
_39.3 = _44.fld1.3 * _45.fld1.3;
_45.fld7.1.3 = _39.3;
_31 = [_30,_30,_30,_22];
SetDiscriminant(_45.fld6, 2);
_45.fld7.2.4.1.1 = _38.fld3.2.4.1.1;
_45.fld7.2.4.1.0 = core::ptr::addr_of_mut!(_45.fld7.0.1.0);
_45.fld2 = _14 + _19;
_44.fld7.2.4.0 = !_38.fld3.2.4.0;
_45.fld7.0.1.1.0 = _12.fld2.1.0;
_44.fld7.2.4.2 = _12.fld4 - _12.fld4;
_45.fld1.0 = _32;
_45.fld1.1 = [_38.fld6.1,_38.fld4,_10,_10];
place!(Field::<isize>(Variant(_45.fld6, 2), 2)) = _19;
_44.fld5 = _38.fld6.4.1.0;
_45.fld7.2 = _38.fld3.2;
_38.fld3 = (_45.fld7.0, _27, _38.fld6);
_38.fld3.0.1.0 = _15 as u32;
_38.fld3.0.1.1.1 = _44.fld7.2.4.1.1;
_38.fld3.2.4.1 = (_38.fld3.0.1.1.0, _44.fld7.2.4.1.1);
_44.fld7.1.1 = [_38.fld6.1,_38.fld4,_45.fld7.2.1,_45.fld7.2.1];
match _22 {
0 => bb34,
1 => bb30,
2 => bb3,
3 => bb32,
4 => bb31,
5 => bb44,
6 => bb45,
331913435410514156892795615208635453681 => bb47,
_ => bb46
}
}
bb44 = {
_44.fld7.0.1.1.0 = _11.1.1.0;
_41 = _38.fld3.2.4.0 > (*_26);
_38.fld6.0 = _38.fld3.2.0;
_44.fld7.1.3 = _15 as i32;
_44.fld1 = (_2, _27.1, _41, _4.3);
match _22 {
0 => bb32,
1 => bb33,
2 => bb34,
3 => bb35,
4 => bb36,
331913435410514156892795615208635453681 => bb38,
_ => bb37
}
}
bb45 = {
_6 = [_10,_10,_10,_10];
_9 = [_10,_10,_10,_10];
_6 = [_10,_10,_10,_10];
_27.0 = _2;
_12.fld2.1.0 = core::ptr::addr_of_mut!((*_26));
place!(Field::<u32>(Variant(RET, 0), 0)) = (-23657_i16) as u32;
_13 = [_11.0,_11.0,_7];
_7 = _11.0;
_13 = _1;
_7 = _11.0;
_8 = _12.fld2.0 + _12.fld2.0;
_27 = _4;
_25 = [9199712987339935079_usize,3_usize,3176385159075471192_usize,7_usize,3_usize];
_27.1 = _4.1;
_14 = -_5;
_3 = [_10,_10,_10,_10];
_6 = [_10,_10,_10,_10];
_1 = _13;
_27.2 = _16 <= _16;
(*_26) = _12.fld4 as u32;
(*_26) = !Field::<u32>(Variant(RET, 0), 0);
_4.3 = _12.fld3 as i32;
_11.1.2 = !_12.fld4;
_11.1.0 = Field::<u32>(Variant(RET, 0), 0);
_8 = _15 as i64;
_11.0 = _7;
_12.fld4 = _11.1.2 ^ _11.1.2;
_21 = _13;
_9 = [_10,_10,_10,_10];
Goto(bb14)
}
bb46 = {
_22 = 331913435410514156892795615208635453681_u128;
_4.0 = [_8,_12.fld2.0,_8,_12.fld2.0,_12.fld2.0];
_4.2 = _27.2;
_4.1 = [_10,_10,_10,_10];
place!(Field::<u32>(Variant(RET, 0), 0)) = (*_26) | _11.1.0;
match _27.3 {
0 => bb15,
340282366920938463463374607430235528118 => bb17,
_ => bb16
}
}
bb47 = {
(*_26) = _44.fld7.2.4.0;
_28.0 = _12.fld1;
_46 = _11.1.2 * _38.fld6.4.2;
_45.fld4 = _12.fld3 as u16;
_38.fld5 = _10 as u8;
_27.3 = -_4.3;
_44.fld7.0.1.0 = _45.fld2 as u32;
_45.fld7.0.1.1.1 = _38.fld3.0.1.1.1;
_38.fld3.0.1 = (Field::<u32>(Variant(RET, 0), 0), _11.1.1, _44.fld7.2.4.2);
_45.fld7.0.1.1.1 = _44.fld7.2.4.1.1;
_43 = _38.fld3.0.0;
_44.fld1.0 = [_8,_8,_8,_8,_8];
_52 = _22 as isize;
_7 = _45.fld7.0.0;
_35 = core::ptr::addr_of_mut!((*_26));
_4.1 = [_10,_38.fld4,_45.fld4,_38.fld4];
match _12.fld3 {
0 => bb40,
1 => bb48,
2 => bb49,
340282366920938463463374607431768211331 => bb51,
_ => bb50
}
}
bb48 = {
_2 = [_12.fld2.0,_8,_8,_12.fld2.0,_12.fld2.0];
_16 = !_4.2;
_8 = _12.fld2.0;
_4.1 = _3;
_16 = _4.2;
_7 = _11.0;
_12.fld2.1.0 = core::ptr::addr_of_mut!(_11.1.0);
_12.fld4 = !_11.1.2;
_13 = [_11.0,_7,_7];
_16 = _4.2 & _4.2;
Call(_12.fld3 = fn4(_7, _2, _7, _4, _16, _4.3, _4.2, _16, _14, _16, _11.1.1.0, _16, _14, _4.3), ReturnTo(bb12), UnwindUnreachable())
}
bb49 = {
place!(Field::<u32>(Variant(RET, 0), 0)) = _11.1.0 - _11.1.0;
_12.fld2.1.0 = core::ptr::addr_of_mut!(_11.1.0);
_8 = _12.fld2.0 >> _11.1.2;
_4 = (_2, _6, _16, (-979875470_i32));
_7 = _11.0;
_4 = (_2, _3, _16, (-1532683338_i32));
_15 = _7 as u64;
_6 = [_10,_10,_10,_10];
place!(Field::<u32>(Variant(RET, 0), 0)) = !_11.1.0;
_25 = [2_usize,3652653892606988480_usize,7_usize,0_usize,0_usize];
_26 = core::ptr::addr_of_mut!(_11.1.0);
_27.3 = -_4.3;
_20 = !_16;
_7 = _11.0;
Call(place!(Field::<u32>(Variant(RET, 0), 0)) = fn5(_4.3, _4.2, _14, _4.3, _27.3, _4, _16, _4.3, _4, _4.2, _4.3, _4.2, _13, _4.2, _26), ReturnTo(bb13), UnwindUnreachable())
}
bb50 = {
Return()
}
bb51 = {
_44.fld1.3 = -_45.fld7.1.3;
_20 = !_44.fld7.1.2;
_38.fld3.0.1.1 = (_44.fld7.0.1.1.0, _44.fld7.2.4.1.1);
_40 = _44.fld7.0.0;
_39 = (_45.fld1.0, _9, _38.fld3.2.2, _45.fld1.3);
place!(Field::<i8>(Variant(_45.fld6, 2), 3)) = _12.fld3 & _12.fld3;
_35 = core::ptr::addr_of_mut!(_44.fld0);
_44.fld7.1.3 = _39.3 | _45.fld7.1.3;
_44.fld7.2.4 = (_38.fld3.0.1.0, _45.fld7.2.4.1, _11.1.2);
_38.fld6.2 = _38.fld3.2.2;
_5 = !Field::<isize>(Variant(_45.fld6, 2), 2);
Goto(bb52)
}
bb52 = {
_55 = !_30;
_38.fld3.0.1 = ((*_26), _38.fld6.4.1, _38.fld6.4.2);
_12.fld1 = _12.fld0.0;
_45.fld6 = Adt44::Variant2 { fld0: _12.fld4,fld1: _15,fld2: _5,fld3: _12.fld3,fld4: _38.fld5 };
_45.fld7.2.3 = _22 as f64;
_45.fld1.0 = _2;
_38.fld3 = (_11, _27, _45.fld7.2);
_12.fld2.1.1 = _44.fld7.0.1.1.1;
_41 = _45.fld7.0.1.0 <= _45.fld7.0.1.0;
RET = Move(_45.fld6);
_45.fld1 = (_44.fld1.0, _4.1, _45.fld7.1.2, _39.3);
_38.fld3.1.2 = _20 ^ _44.fld1.2;
Goto(bb53)
}
bb53 = {
Call(_60 = dump_var(3_usize, 2_usize, Move(_2), 39_usize, Move(_39), 30_usize, Move(_30), 19_usize, Move(_19)), ReturnTo(bb54), UnwindUnreachable())
}
bb54 = {
Call(_60 = dump_var(3_usize, 5_usize, Move(_5), 16_usize, Move(_16), 14_usize, Move(_14), 20_usize, Move(_20)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_60 = dump_var(3_usize, 7_usize, Move(_7), 37_usize, Move(_37), 40_usize, Move(_40), 3_usize, Move(_3)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_60 = dump_var(3_usize, 15_usize, Move(_15), 41_usize, Move(_41), 31_usize, Move(_31), 25_usize, Move(_25)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: char,mut _2: [i64; 5],mut _3: char,mut _4: ([i64; 5], [u16; 4], bool, i32),mut _5: bool,mut _6: i32,mut _7: bool,mut _8: bool,mut _9: isize,mut _10: bool,mut _11: *mut u32,mut _12: bool,mut _13: isize,mut _14: i32) -> i8 {
mir! {
type RET = i8;
let _15: [char; 3];
let _16: f32;
let _17: Adt53;
let _18: bool;
let _19: (u32, (*mut u32, *mut i16), i128);
let _20: Adt44;
let _21: u32;
let _22: u16;
let _23: ();
let _24: ();
{
_4.3 = !_14;
_4.0 = [1776532554715526076_i64,(-955682621174795474_i64),(-5450503755400792400_i64),6354009122858372127_i64,4648574595805645896_i64];
RET = 101_i8 & 24_i8;
RET = 56_i8;
(*_11) = 29047_u16 as u32;
_5 = _4.3 > _14;
_4.3 = -_14;
_4.2 = !_5;
(*_11) = 1502252795_u32 * 4070566341_u32;
_16 = _9 as f32;
_1 = _3;
_4.0 = _2;
_13 = _9;
_15 = [_3,_1,_1];
_9 = _13;
_16 = 32527_u16 as f32;
_1 = _3;
RET = -47_i8;
_16 = (*_11) as f32;
_16 = 4_usize as f32;
_4.3 = -_6;
(*_11) = 3418014233_u32;
_8 = !_5;
(*_11) = !2807394844_u32;
_2 = _4.0;
_7 = _4.2;
(*_11) = 57583_u16 as u32;
_7 = _8 & _12;
Goto(bb1)
}
bb1 = {
_12 = _7;
(*_11) = 1523234657_u32;
_4.2 = _5 <= _7;
Goto(bb2)
}
bb2 = {
RET = (-125_i8);
Goto(bb3)
}
bb3 = {
_11 = core::ptr::addr_of_mut!((*_11));
match _14 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607429776090979 => bb8,
_ => bb7
}
}
bb4 = {
RET = (-125_i8);
Goto(bb3)
}
bb5 = {
_12 = _7;
(*_11) = 1523234657_u32;
_4.2 = _5 <= _7;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_4.1 = [7949_u16,56922_u16,5702_u16,23469_u16];
_4.0 = [8633934248843401930_i64,(-5790279908721490319_i64),8889418511262145833_i64,8644274514166182228_i64,(-752572328629949509_i64)];
_9 = _13 - _13;
_12 = !_4.2;
_15 = [_1,_1,_3];
_4.1 = [27116_u16,54743_u16,18199_u16,44096_u16];
match _6 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
340282366920938463463374607429776090979 => bb15,
_ => bb14
}
}
bb9 = {
_12 = _7;
(*_11) = 1523234657_u32;
_4.2 = _5 <= _7;
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
_12 = _7;
(*_11) = 1523234657_u32;
_4.2 = _5 <= _7;
Goto(bb2)
}
bb12 = {
RET = (-125_i8);
Goto(bb3)
}
bb13 = {
_11 = core::ptr::addr_of_mut!((*_11));
match _14 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607429776090979 => bb8,
_ => bb7
}
}
bb14 = {
RET = (-125_i8);
Goto(bb3)
}
bb15 = {
_19.1.0 = _11;
_18 = _4.2;
_21 = !(*_11);
_4.1 = [49763_u16,51251_u16,35427_u16,30506_u16];
_4.1 = [57283_u16,62365_u16,22640_u16,37805_u16];
_18 = _12 != _4.2;
Goto(bb16)
}
bb16 = {
Call(_23 = dump_var(4_usize, 9_usize, Move(_9), 12_usize, Move(_12), 10_usize, Move(_10), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_23 = dump_var(4_usize, 5_usize, Move(_5), 2_usize, Move(_2), 7_usize, Move(_7), 18_usize, Move(_18)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: i32,mut _2: bool,mut _3: isize,mut _4: i32,mut _5: i32,mut _6: ([i64; 5], [u16; 4], bool, i32),mut _7: bool,mut _8: i32,mut _9: ([i64; 5], [u16; 4], bool, i32),mut _10: bool,mut _11: i32,mut _12: bool,mut _13: [char; 3],mut _14: bool,mut _15: *mut u32) -> u32 {
mir! {
type RET = u32;
let _16: ([i64; 5], [u16; 4], bool, i32);
let _17: char;
let _18: u16;
let _19: ();
let _20: ();
{
_9.2 = !_2;
_1 = _5 << _6.3;
_15 = core::ptr::addr_of_mut!(RET);
RET = 126372166_u32;
_6.0 = [(-5301157445181607079_i64),(-4997136105005935725_i64),1005580453169057742_i64,(-668199430118937675_i64),2744013146960414043_i64];
_5 = (-13595193553186579604112029112227476428_i128) as i32;
_9.0 = _6.0;
_3 = !9223372036854775807_isize;
_9.2 = _2;
_12 = _10;
_10 = _9.2;
_5 = _1;
_9.1 = [64557_u16,47972_u16,16915_u16,15124_u16];
_11 = !_1;
(*_15) = 1675670210_u32 & 2401797906_u32;
_6.0 = _9.0;
_1 = !_4;
_11 = 8685507356507529561_u64 as i32;
_9.1 = [31168_u16,23768_u16,18183_u16,8542_u16];
Goto(bb1)
}
bb1 = {
_16.3 = _6.3 | _6.3;
_16.2 = !_2;
_17 = '\u{aa9d0}';
_9.0 = [3073500470305730095_i64,(-1486982350911559128_i64),(-5790367499994060679_i64),(-2200513928069396931_i64),8859690968894425832_i64];
_9.3 = RET as i32;
_16 = (_9.0, _9.1, _10, _4);
_16 = _9;
_3 = 76_isize;
RET = 3128153270_u32 << _8;
_9 = _16;
Goto(bb2)
}
bb2 = {
Call(_19 = dump_var(5_usize, 8_usize, Move(_8), 6_usize, Move(_6), 5_usize, Move(_5), 10_usize, Move(_10)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_19 = dump_var(5_usize, 1_usize, Move(_1), 7_usize, Move(_7), 14_usize, Move(_14), 17_usize, Move(_17)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: bool,mut _2: *mut u32,mut _3: [i64; 5],mut _4: *mut u32,mut _5: ([i64; 5], [u16; 4], bool, i32),mut _6: bool,mut _7: char,mut _8: bool,mut _9: bool,mut _10: bool,mut _11: [char; 3],mut _12: ([i64; 5], [u16; 4], bool, i32),mut _13: ([i64; 5], [u16; 4], bool, i32),mut _14: ([i64; 5], [u16; 4], bool, i32),mut _15: bool) -> (u32, (*mut u32, *mut i16), i128) {
mir! {
type RET = (u32, (*mut u32, *mut i16), i128);
let _16: u128;
let _17: char;
let _18: i64;
let _19: usize;
let _20: ([i64; 5], [u16; 4], bool, i32);
let _21: ((char, (u32, (*mut u32, *mut i16), i128)), ([i64; 5], [u16; 4], bool, i32), (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)));
let _22: isize;
let _23: [i64; 5];
let _24: bool;
let _25: (*mut u32, *mut i16);
let _26: char;
let _27: f32;
let _28: u16;
let _29: isize;
let _30: f32;
let _31: ();
let _32: ();
{
_6 = _8;
_5.2 = _9;
_4 = _2;
RET.2 = (-21_i8) as i128;
_13.0 = _3;
_14.2 = _13.2;
_5.1 = _12.1;
RET.1.0 = _4;
_5.1 = [26007_u16,47172_u16,43177_u16,35211_u16];
_14.1 = _12.1;
RET.0 = 198459371_u32;
_20 = (_13.0, _5.1, _8, _14.3);
RET.0 = _7 as u32;
_13.2 = _9;
_20.2 = _5.2 | _6;
RET.1.0 = core::ptr::addr_of_mut!(RET.0);
_13 = _12;
_12.0 = [3544582027611922629_i64,(-9175905177626070311_i64),(-345960011759877375_i64),2619376525847435884_i64,3798044200870234610_i64];
Call(_5.3 = fn7(_20, _20.2, _5.2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = _2;
RET.1.0 = _2;
_10 = _5.2;
_5.0 = [(-6206090135387732464_i64),3452705449014020664_i64,4877023458220125839_i64,(-7368305012561053938_i64),(-2352360334184068180_i64)];
_16 = _10 as u128;
RET.0 = RET.2 as u32;
_14.0 = [(-4107933300714836153_i64),3041717106783602571_i64,(-6222528852608969326_i64),1284172145024695130_i64,2225794525551465085_i64];
RET.0 = 4000796433_u32;
_20.3 = _12.3;
_14 = (_12.0, _20.1, _15, _12.3);
_14.0 = _12.0;
_14.0 = [(-8440553135165703118_i64),(-7637660670573142291_i64),2208611715019732094_i64,2677784731714722516_i64,2779592620987984898_i64];
RET.0 = 3723545836_u32 << _16;
_18 = (-4869446838506927143_i64);
_21.1.0 = [_18,_18,_18,_18,_18];
_21.0.1.2 = RET.2;
_2 = _4;
_21.2.2 = _20.2 >= _13.2;
_3 = [_18,_18,_18,_18,_18];
_23 = [_18,_18,_18,_18,_18];
_10 = _5.2;
_21.0.1.0 = RET.0;
_1 = _13.2;
_15 = !_20.2;
_13 = (_21.1.0, _14.1, _15, _5.3);
Call(_21.0.1.1 = fn8(RET.0, _14.0, _13.1, _10, _21.2.2, _20.2, _1, _4, _13.2, _21.0.1.0, RET.0, _16, _13.2, _12), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET.1.1 = _21.0.1.1.1;
_22 = _16 as isize;
_20.3 = _5.3;
_25.1 = _21.0.1.1.1;
_15 = _8;
_10 = _1 & _13.2;
_12.2 = _16 < _16;
_21.1 = _13;
_21.2.4.1.0 = _21.0.1.1.0;
_21.0.0 = _7;
_21.0 = (_7, RET);
_4 = _21.2.4.1.0;
_12.3 = !_5.3;
_7 = _21.0.0;
_3 = [_18,_18,_18,_18,_18];
_21.1.0 = [_18,_18,_18,_18,_18];
_13.2 = _12.2;
_21.0.1.1 = (_4, RET.1.1);
_20.3 = _21.1.3 * _5.3;
_12.3 = -_20.3;
_21.2.4 = (_21.0.1.0, _21.0.1.1, RET.2);
_29 = _22 + _22;
_21.0.1.1.0 = _21.2.4.1.0;
_21.1.0 = [_18,_18,_18,_18,_18];
RET = (_21.0.1.0, _21.0.1.1, _21.2.4.2);
_21.2.4 = (_21.0.1.0, _21.0.1.1, RET.2);
Goto(bb3)
}
bb3 = {
Call(_31 = dump_var(6_usize, 11_usize, Move(_11), 29_usize, Move(_29), 18_usize, Move(_18), 20_usize, Move(_20)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_31 = dump_var(6_usize, 8_usize, Move(_8), 23_usize, Move(_23), 1_usize, Move(_1), 6_usize, Move(_6)), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Call(_31 = dump_var(6_usize, 16_usize, Move(_16), 32_usize, _32, 32_usize, _32, 32_usize, _32), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: ([i64; 5], [u16; 4], bool, i32),mut _2: bool,mut _3: bool) -> i32 {
mir! {
type RET = i32;
let _4: isize;
let _5: [usize; 5];
let _6: f32;
let _7: bool;
let _8: i32;
let _9: Adt46;
let _10: [usize; 5];
let _11: isize;
let _12: bool;
let _13: i16;
let _14: u16;
let _15: i64;
let _16: Adt58;
let _17: u16;
let _18: Adt46;
let _19: Adt54;
let _20: ([i64; 5], [u16; 4], bool, i32);
let _21: i16;
let _22: u32;
let _23: i32;
let _24: char;
let _25: Adt45;
let _26: char;
let _27: i32;
let _28: [u16; 4];
let _29: ();
let _30: ();
{
_1.2 = !_2;
_1.0 = [4921107584175920068_i64,(-6530273334812887997_i64),5207834367779708993_i64,(-536830775935490411_i64),(-8161231854506092713_i64)];
_7 = _1.2;
_3 = _7;
_3 = !_2;
_1.0 = [656974639035382966_i64,4939069434497147522_i64,3821944116655570035_i64,3150227696606424322_i64,4697382090362082939_i64];
RET = 141999990558692341723636526317601965663_u128 as i32;
RET = _1.3 ^ _1.3;
_5 = [1_usize,9025732230942018994_usize,5217890706799917315_usize,1_usize,7_usize];
_9 = Adt46::Variant3 { fld0: _1.0 };
_5 = [6_usize,3704229599776427996_usize,1_usize,17517570753625614745_usize,4_usize];
_1.1 = [37163_u16,6131_u16,55803_u16,12484_u16];
_1.0 = Field::<[i64; 5]>(Variant(_9, 3), 0);
_3 = _1.2 < _2;
_6 = 79_u8 as f32;
Goto(bb1)
}
bb1 = {
_3 = !_2;
_8 = RET ^ _1.3;
_2 = _3 <= _1.2;
_5 = [4_usize,4767760420970602299_usize,7_usize,1800785250053093678_usize,1_usize];
_11 = (-97_isize);
_10 = [12670559647497438300_usize,7607384857493853930_usize,2_usize,6_usize,3621786317783188000_usize];
_1.0 = Field::<[i64; 5]>(Variant(_9, 3), 0);
_7 = _1.2;
_1.2 = _7;
_6 = 91_i8 as f32;
RET = 45028742594921903349363945586014527132_i128 as i32;
_8 = 13339942521798868914_u64 as i32;
_1.0 = [(-2872274164119036145_i64),(-2225891445135044843_i64),(-8198625593228740591_i64),(-3141313619460481312_i64),(-6519962980938813400_i64)];
_5 = [2172805802596785656_usize,4626198884109507544_usize,2_usize,5297042029820045159_usize,5020998125343202424_usize];
_3 = _2 >= _2;
_1.2 = _2 & _7;
_8 = _1.3 ^ RET;
_1.3 = (-156820854702824047223675766009616279760_i128) as i32;
place!(Field::<[i64; 5]>(Variant(_9, 3), 0)) = [8506182721878104327_i64,8999275679735033818_i64,(-3601454993090711466_i64),(-1186969300131610923_i64),8140338713213502584_i64];
RET = _1.3 << _11;
_2 = _1.2;
_2 = _1.2 > _1.2;
match _11 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211359 => bb7,
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
_12 = _1.2;
_7 = _12 >= _12;
_4 = 30_u8 as isize;
_1.2 = _2 == _12;
_1.3 = _8 * RET;
_1.3 = _8;
RET = !_8;
_5 = [7_usize,6_usize,10999413410260638403_usize,5_usize,0_usize];
_1.0 = Field::<[i64; 5]>(Variant(_9, 3), 0);
place!(Field::<[i64; 5]>(Variant(_9, 3), 0)) = [(-5644751829725537388_i64),3430776064908509199_i64,7815487583567101244_i64,(-3337396735996329103_i64),(-3072314344466381601_i64)];
RET = _8 << _1.3;
_1.0 = Field::<[i64; 5]>(Variant(_9, 3), 0);
_1.2 = _2 | _3;
_9 = Adt46::Variant3 { fld0: _1.0 };
_9 = Adt46::Variant1 { fld0: _5 };
_1.3 = RET + _8;
Goto(bb8)
}
bb8 = {
_10 = Field::<[usize; 5]>(Variant(_9, 1), 0);
SetDiscriminant(_9, 1);
_5 = [6_usize,6_usize,7_usize,7_usize,17770739086167139498_usize];
_7 = !_2;
_1.1 = [2153_u16,10850_u16,54840_u16,40772_u16];
Goto(bb9)
}
bb9 = {
_7 = !_12;
_5 = [2_usize,2_usize,6_usize,1305058367423907173_usize,1_usize];
_10 = _5;
_2 = _3;
place!(Field::<[usize; 5]>(Variant(_9, 1), 0)) = _10;
_17 = 63608_u16;
_12 = _3 ^ _1.2;
_18 = _9;
_8 = _1.3 + _1.3;
SetDiscriminant(_18, 3);
_4 = !_11;
_13 = (-16654_i16) ^ (-27054_i16);
_12 = _2 > _2;
SetDiscriminant(_9, 2);
place!(Field::<[i64; 5]>(Variant(_18, 3), 0)) = _1.0;
place!(Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_9, 2), 2)).1 = _17 / _17;
_1.1 = [_17,_17,_17,Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_9, 2), 2).1];
_14 = _17 >> RET;
place!(Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_9, 2), 2)).4.1.0 = core::ptr::addr_of_mut!(place!(Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_9, 2), 2)).4.0);
Goto(bb10)
}
bb10 = {
place!(Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_9, 2), 2)).2 = _12 ^ _3;
RET = 176132012148825901377860212731531456513_u128 as i32;
place!(Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_9, 2), 2)).0 = 15_i8 as f64;
_20 = (Field::<[i64; 5]>(Variant(_18, 3), 0), _1.1, _12, _8);
_23 = _20.3;
_19 = Adt54::Variant2 { fld0: 7304042115706207187_i64 };
place!(Field::<(i64, (*mut u32, *mut i16))>(Variant(_9, 2), 0)).1.0 = core::ptr::addr_of_mut!(_22);
place!(Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_9, 2), 2)).4.2 = (-119387467307974228329596442039675944812_i128) | (-144817988429810173092540159702089022988_i128);
_21 = _13;
place!(Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_9, 2), 2)).3 = Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_9, 2), 2).0;
_1.2 = _2;
_7 = _1.2 > _12;
place!(Field::<(f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))>(Variant(_9, 2), 2)).4.1.1 = core::ptr::addr_of_mut!(_13);
_13 = 248604031399098680535282213044330355267_u128 as i16;
place!(Field::<i64>(Variant(_19, 2), 0)) = -3775402077657313157_i64;
SetDiscriminant(_19, 0);
_9 = _18;
_21 = !_13;
_25.fld4 = [_8];
_22 = !1074503971_u32;
place!(Field::<*const [usize; 5]>(Variant(_19, 0), 1)) = core::ptr::addr_of!(_5);
_23 = _8;
RET = 175540428728525593692148126722637246739_u128 as i32;
_25.fld0 = 9148016539185727110_i64 as u64;
Goto(bb11)
}
bb11 = {
_1.2 = !_7;
_20.1 = [_14,_14,_14,_14];
_24 = '\u{52be7}';
place!(Field::<*const [usize; 5]>(Variant(_19, 0), 1)) = core::ptr::addr_of!(_10);
Goto(bb12)
}
bb12 = {
_23 = -_1.3;
_13 = -_21;
place!(Field::<*const [usize; 5]>(Variant(_19, 0), 1)) = core::ptr::addr_of!(_5);
_5 = [9731580474247436275_usize,4290594812248494826_usize,6_usize,4_usize,8206251682244149216_usize];
_1 = _20;
_20.0 = [6325681203570445251_i64,5117217989392064978_i64,(-303030390910946469_i64),6069515454288278291_i64,681666050972326626_i64];
_3 = _12;
_21 = _13;
place!(Field::<[i64; 5]>(Variant(_9, 3), 0)) = _20.0;
_27 = _1.3;
_12 = !_2;
_11 = !_4;
_26 = _24;
_2 = _20.2;
Goto(bb13)
}
bb13 = {
_7 = _3 > _1.2;
Goto(bb14)
}
bb14 = {
_1.0 = [(-44783880126754891_i64),3708399312893069397_i64,2433949236471653156_i64,(-3798175227771472569_i64),1519860375087713461_i64];
_20.3 = _1.3 + _27;
_22 = !3151923391_u32;
_8 = 5643154145816230379_i64 as i32;
_12 = _2;
_25.fld5 = _25.fld0 as f32;
_28 = [_14,_14,_14,_14];
_1.3 = !_20.3;
_20.0 = [4212193494807295489_i64,(-352519275712268496_i64),(-8824731002477215658_i64),150007326909855723_i64,8384459330791059841_i64];
_20.3 = _23;
_5 = _10;
_12 = !_3;
place!(Field::<*const [usize; 5]>(Variant(_19, 0), 1)) = core::ptr::addr_of!(_10);
_7 = !_20.2;
_23 = _27 << _1.3;
_11 = _26 as isize;
Goto(bb15)
}
bb15 = {
Call(_29 = dump_var(7_usize, 24_usize, Move(_24), 4_usize, Move(_4), 7_usize, Move(_7), 21_usize, Move(_21)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_29 = dump_var(7_usize, 12_usize, Move(_12), 22_usize, Move(_22), 13_usize, Move(_13), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_29 = dump_var(7_usize, 10_usize, Move(_10), 17_usize, Move(_17), 30_usize, _30, 30_usize, _30), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: u32,mut _2: [i64; 5],mut _3: [u16; 4],mut _4: bool,mut _5: bool,mut _6: bool,mut _7: bool,mut _8: *mut u32,mut _9: bool,mut _10: u32,mut _11: u32,mut _12: u128,mut _13: bool,mut _14: ([i64; 5], [u16; 4], bool, i32)) -> (*mut u32, *mut i16) {
mir! {
type RET = (*mut u32, *mut i16);
let _15: [char; 3];
let _16: [u16; 4];
let _17: Adt51;
let _18: f64;
let _19: ([i64; 5], [u16; 4], bool, i32);
let _20: *mut i128;
let _21: (*const (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),);
let _22: *mut *mut i16;
let _23: i8;
let _24: isize;
let _25: isize;
let _26: ([i64; 5], [u16; 4], bool, i32);
let _27: f32;
let _28: Adt54;
let _29: [i32; 1];
let _30: f32;
let _31: u8;
let _32: [i32; 1];
let _33: [i32; 1];
let _34: [char; 3];
let _35: [u64; 2];
let _36: isize;
let _37: Adt47;
let _38: [i64; 5];
let _39: ();
let _40: ();
{
_14.2 = _13 == _6;
_10 = _1 | _1;
_14 = (_2, _3, _5, 81690053_i32);
_7 = !_13;
_19.1 = [29984_u16,47328_u16,34097_u16,10113_u16];
_19.3 = -_14.3;
_19 = _14;
_15 = ['\u{5bd52}','\u{17be8}','\u{6a4c8}'];
_14.0 = [(-8487600259529595538_i64),290413998010778663_i64,(-182098532026887253_i64),772411704897621536_i64,3396227859671731172_i64];
_15 = ['\u{41f4a}','\u{d74fa}','\u{800d4}'];
_2 = [3380551355007055504_i64,7773112560214937067_i64,7684476130488322066_i64,2127444562054765592_i64,(-8008889671651081349_i64)];
_17 = Adt51::Variant0 { fld0: _12 };
SetDiscriminant(_17, 1);
_19.1 = _14.1;
RET.0 = core::ptr::addr_of_mut!(_1);
_16 = [51497_u16,29089_u16,18230_u16,10762_u16];
_15 = ['\u{7241a}','\u{2d1c4}','\u{7e741}'];
_16 = [19920_u16,33145_u16,4521_u16,6004_u16];
_4 = _14.2 >= _14.2;
Call(_8 = fn9(_19.3, _19.2, _4, _19.2, _5, _14.3, _19.2, _6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_14.0 = _2;
RET.0 = _8;
_11 = _10 & _10;
_24 = (-8_isize) - 9223372036854775807_isize;
_14.0 = _19.0;
_9 = _19.2;
_16 = [4072_u16,35212_u16,48360_u16,19568_u16];
RET.0 = _8;
_25 = -_24;
RET.0 = _8;
_4 = !_5;
_5 = !_4;
RET.0 = core::ptr::addr_of_mut!(_10);
Goto(bb2)
}
bb2 = {
RET.0 = core::ptr::addr_of_mut!(_1);
_14.2 = _13;
_19.3 = -_14.3;
_23 = (-55_i8) ^ (-128_i8);
_14 = (_19.0, _19.1, _19.2, _19.3);
_27 = _12 as f32;
_1 = _10;
_26.1 = [31710_u16,59104_u16,49709_u16,16718_u16];
_8 = core::ptr::addr_of_mut!(_1);
_26.2 = _6 > _7;
_24 = (-6549_i16) as isize;
_14.3 = !_19.3;
_18 = (*_8) as f64;
_17 = Adt51::Variant0 { fld0: _12 };
_26.0 = [5521271670653731766_i64,(-2871772630915273269_i64),5706494712039923298_i64,6685066335313399781_i64,5534035093640140506_i64];
_15 = ['\u{5ec9f}','\u{5569}','\u{bbe98}'];
_27 = 45206_u16 as f32;
_30 = _27 * _27;
RET.0 = _8;
_31 = 134933177260697526049333965614257118208_i128 as u8;
_12 = Field::<u128>(Variant(_17, 0), 0) | Field::<u128>(Variant(_17, 0), 0);
_32 = [_14.3];
SetDiscriminant(_17, 0);
_4 = _9;
Goto(bb3)
}
bb3 = {
_26.0 = _19.0;
place!(Field::<u128>(Variant(_17, 0), 0)) = _12;
_29 = _32;
_33 = [_14.3];
Goto(bb4)
}
bb4 = {
_11 = !(*_8);
Call(RET.1 = fn10(_14, _14, (*_8), _14.3, Move(_17), _19.3, _14.2, _1, (*_8), _19.2, _18, _19.3), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_7 = !_19.2;
_35 = [14031425611853611255_u64,903039200999389997_u64];
_26.0 = [3080157034177649921_i64,(-3193578398911272336_i64),(-1962883603105638841_i64),5641226593665402149_i64,7353591984576343822_i64];
_14.3 = 125472655108898213230415909290564101825_i128 as i32;
_14 = (_2, _19.1, _5, _19.3);
(*_8) = _10 >> _11;
_24 = (-77517609001489693843235249414321949248_i128) as isize;
_26.0 = [7584467959803960185_i64,(-4317295240121778932_i64),(-2730799774360565589_i64),3382782618298412186_i64,4823549964897386861_i64];
_19.2 = _14.2;
_16 = [64303_u16,20362_u16,64576_u16,15754_u16];
Goto(bb6)
}
bb6 = {
Call(_39 = dump_var(8_usize, 29_usize, Move(_29), 23_usize, Move(_23), 31_usize, Move(_31), 19_usize, Move(_19)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_39 = dump_var(8_usize, 2_usize, Move(_2), 1_usize, Move(_1), 24_usize, Move(_24), 14_usize, Move(_14)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_39 = dump_var(8_usize, 13_usize, Move(_13), 5_usize, Move(_5), 3_usize, Move(_3), 9_usize, Move(_9)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: i32,mut _2: bool,mut _3: bool,mut _4: bool,mut _5: bool,mut _6: i32,mut _7: bool,mut _8: bool) -> *mut u32 {
mir! {
type RET = *mut u32;
let _9: usize;
let _10: [u64; 2];
let _11: [usize; 5];
let _12: ([i64; 5], [u16; 4], bool, i32);
let _13: [usize; 5];
let _14: [u16; 4];
let _15: u32;
let _16: u128;
let _17: bool;
let _18: i64;
let _19: isize;
let _20: *mut f32;
let _21: (*mut u32, *mut i16);
let _22: u16;
let _23: *mut f32;
let _24: isize;
let _25: i64;
let _26: bool;
let _27: [u16; 4];
let _28: ();
let _29: ();
{
_7 = _1 != _1;
_1 = (-5605201050765577328019617325874878274_i128) as i32;
_3 = !_2;
_2 = !_5;
_1 = _6 * _6;
_7 = _5;
_7 = _4;
_1 = _6 + _6;
_7 = _2;
_5 = _8;
_5 = _3 | _7;
_1 = (-93_isize) as i32;
_8 = !_5;
_3 = _4;
_9 = 9420832479888339366_usize;
_8 = _4;
_9 = !16139343694133917063_usize;
_7 = _8 ^ _5;
_1 = 2372481275_u32 as i32;
_10 = [47965281407223529_u64,11809803579029995913_u64];
_6 = _1 & _1;
_9 = 5835705698931996391_usize;
_1 = _6 - _6;
_11 = [_9,_9,_9,_9,_9];
match _9 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
5835705698931996391 => bb8,
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
_1 = _6;
_8 = !_2;
_9 = 6053537822193757289_usize;
_3 = _4 > _4;
_2 = _5 ^ _3;
_9 = 2_usize;
_12.1[_9] = (-113_i8) as u16;
_12.1 = [47628_u16,55775_u16,3006_u16,31906_u16];
_1 = _6;
_12.1[_9] = 59478_u16 << _6;
_13[_9] = _11[_9];
_12.2 = _4;
RET = core::ptr::addr_of_mut!(_15);
_12.1[_9] = 542188945_u32 as u16;
_7 = !_8;
_12.0 = [438070240892821417_i64,3287323410822932675_i64,(-2196537075552182225_i64),(-3772453704713536890_i64),(-2161605251799811304_i64)];
(*RET) = 113013802466203157767943844713071172801_u128 as u32;
_14 = [_12.1[_9],_12.1[_9],_12.1[_9],_12.1[_9]];
_8 = !_5;
Goto(bb9)
}
bb9 = {
_2 = !_8;
_12.0[_9] = !(-2178465012802247433_i64);
_13 = [_9,_9,_11[_9],_9,_9];
_10 = [6383139591881372598_u64,14101011131986829823_u64];
_13 = [_11[_9],_11[_9],_11[_9],_9,_11[_9]];
RET = core::ptr::addr_of_mut!((*RET));
_12.0 = [(-411698378305672320_i64),(-6965109805918139191_i64),(-2983375896810679251_i64),3134648997112062332_i64,(-4324795215751420929_i64)];
_15 = 13126946502520438236_u64 as u32;
(*RET) = 21_isize as u32;
_15 = 1873779190_u32 - 4265440458_u32;
(*RET) = !2205564062_u32;
_5 = _3;
_8 = _7 != _7;
_11 = [_13[_9],_13[_9],_9,_9,_13[_9]];
_14 = [_12.1[_9],_12.1[_9],_12.1[_9],_12.1[_9]];
_12.0 = [(-4206693763033913359_i64),3137720511448053357_i64,(-8182099534625291903_i64),(-4949277425465278217_i64),2471450117933428567_i64];
_10 = [1334926443811720386_u64,5778894543372402708_u64];
_12.2 = _5 ^ _3;
_8 = !_3;
_13[_9] = !_11[_9];
_12.1 = _14;
_3 = !_8;
_12.0[_9] = 7237606166632651902_i64;
RET = core::ptr::addr_of_mut!((*RET));
_12.0[_9] = 6506364043555398263_i64;
_9 = _13[_9] / _11[_9];
_18 = 39_i8 as i64;
Goto(bb10)
}
bb10 = {
_17 = _3 == _8;
_11 = [_9,_9,_9,_9,_9];
_3 = !_5;
_16 = !246094927213683486476573409022365892500_u128;
_3 = _12.2 == _8;
_21.0 = RET;
_16 = 17307591349428137422009726517520843047_u128 << _1;
RET = _21.0;
_4 = !_3;
_19 = (-9223372036854775808_isize);
_9 = 5_usize + 2_usize;
RET = core::ptr::addr_of_mut!((*RET));
_7 = _3;
_14 = _12.1;
_1 = _6 - _6;
_12.0 = [_18,_18,_18,_18,_18];
_12.3 = _16 as i32;
_12.0 = [_18,_18,_18,_18,_18];
_22 = 6342_i16 as u16;
_12.0 = [_18,_18,_18,_18,_18];
_1 = !_12.3;
RET = core::ptr::addr_of_mut!((*RET));
Goto(bb11)
}
bb11 = {
_25 = -_18;
_3 = _17 <= _12.2;
_12.1 = [_22,_22,_22,_22];
_18 = '\u{d617a}' as i64;
_15 = !1865051627_u32;
_25 = _16 as i64;
_7 = !_2;
_22 = (*RET) as u16;
_14 = [_22,_22,_22,_22];
_1 = -_6;
RET = _21.0;
(*RET) = 875960545_u32;
(*RET) = 2581384670_u32 * 4136701024_u32;
_7 = _4 >= _4;
_24 = _19;
match _24 {
0 => bb3,
1 => bb2,
2 => bb12,
3 => bb13,
4 => bb14,
340282366920938463454151235394913435648 => bb16,
_ => bb15
}
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
_15 = 2944649451_u32 - 3542744301_u32;
_27 = [_22,_22,_22,_22];
_25 = '\u{641f}' as i64;
Goto(bb17)
}
bb17 = {
Call(_28 = dump_var(9_usize, 5_usize, Move(_5), 3_usize, Move(_3), 25_usize, Move(_25), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_28 = dump_var(9_usize, 24_usize, Move(_24), 1_usize, Move(_1), 11_usize, Move(_11), 8_usize, Move(_8)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_28 = dump_var(9_usize, 13_usize, Move(_13), 6_usize, Move(_6), 7_usize, Move(_7), 29_usize, _29), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: ([i64; 5], [u16; 4], bool, i32),mut _2: ([i64; 5], [u16; 4], bool, i32),mut _3: u32,mut _4: i32,mut _5: Adt51,mut _6: i32,mut _7: bool,mut _8: u32,mut _9: u32,mut _10: bool,mut _11: f64,mut _12: i32) -> *mut i16 {
mir! {
type RET = *mut i16;
let _13: char;
let _14: [usize; 5];
let _15: char;
let _16: [i32; 1];
let _17: *mut u32;
let _18: [i32; 1];
let _19: [char; 3];
let _20: *const char;
let _21: [u64; 2];
let _22: [u64; 2];
let _23: Adt44;
let _24: [u64; 2];
let _25: Adt54;
let _26: [i64; 5];
let _27: (*const (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),);
let _28: [u64; 2];
let _29: (u32, (*mut u32, *mut i16), i128);
let _30: isize;
let _31: i8;
let _32: f64;
let _33: u64;
let _34: f32;
let _35: bool;
let _36: i32;
let _37: ();
let _38: ();
{
_6 = _4 | _1.3;
_1.2 = _7;
Goto(bb1)
}
bb1 = {
_2 = _1;
_10 = _1.2 | _7;
_1.0 = [7972078282730639759_i64,1373530167501890804_i64,(-7372269168023526115_i64),(-5376824062393375946_i64),(-3696360967588159994_i64)];
_4 = _2.3 ^ _2.3;
_1.2 = _7;
Goto(bb2)
}
bb2 = {
_2.3 = _12 ^ _6;
_11 = 68_u8 as f64;
_7 = _1.2 ^ _1.2;
_1.0 = _2.0;
_7 = _1.2;
_12 = _6;
Goto(bb3)
}
bb3 = {
_2.2 = !_1.2;
Call(RET = fn11(Field::<u128>(Variant(_5, 0), 0), _6, _9, _12), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_12 = _4;
_10 = _7;
_10 = !_1.2;
_3 = _9 ^ _9;
_2.0 = [(-5170603592868940347_i64),(-5947273322395348640_i64),2585858629617451437_i64,7023407812128110034_i64,(-8751931140524304161_i64)];
_5 = Adt51::Variant0 { fld0: 286207789419146954762580305437072381936_u128 };
_19 = ['\u{57dc2}','\u{353d8}','\u{e53d5}'];
_22 = [69482059696170100_u64,860598413020883028_u64];
_14 = [611724172825906927_usize,5_usize,2_usize,10889742969488235400_usize,11687289954338512481_usize];
_21 = [12988539653347794355_u64,3889108514497661469_u64];
Call(_4 = fn15(_10, _2, _2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_11 = _3 as f64;
_2.1 = [21094_u16,16698_u16,217_u16,45053_u16];
_13 = '\u{105d70}';
_13 = '\u{83061}';
_1.1 = _2.1;
_17 = core::ptr::addr_of_mut!(_3);
_8 = _9 | (*_17);
_3 = !_9;
_4 = _6;
(*_17) = _8 + _9;
_1.2 = !_10;
_3 = !_9;
_12 = 4_usize as i32;
Goto(bb6)
}
bb6 = {
_7 = _2.2;
place!(Field::<u128>(Variant(_5, 0), 0)) = 32826332045246985762298550135280542696_u128;
_1.1 = [40740_u16,46657_u16,25015_u16,35703_u16];
_2 = (_1.0, _1.1, _10, _4);
_20 = core::ptr::addr_of!(_13);
(*_20) = '\u{d9f2a}';
_1.0 = _2.0;
_1.2 = _2.2 ^ _7;
_18 = [_4];
_1.2 = !_10;
_15 = _13;
SetDiscriminant(_5, 2);
_21 = [2047671321911383311_u64,8293628163165456573_u64];
_4 = _1.3;
_9 = _8;
Call(_9 = core::intrinsics::transmute(_3), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_5 = Adt51::Variant0 { fld0: 269460717655910077828466484078801683662_u128 };
place!(Field::<u128>(Variant(_5, 0), 0)) = 95134591092630542364940618693261968584_u128 & 23435922136457355206251775758879659981_u128;
Goto(bb8)
}
bb8 = {
_2 = (_1.0, _1.1, _7, _4);
_1.2 = (*_17) >= _8;
(*_20) = _15;
_2 = _1;
_18 = [_4];
Call(_8 = core::intrinsics::bswap((*_17)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_1 = _2;
_3 = (-159027315720200016718127913293487891545_i128) as u32;
_2.0 = [5436428568383040509_i64,(-4757328237489453968_i64),6443564470266486711_i64,5535951343004759007_i64,8952485930837294633_i64];
_8 = _9 | _9;
_4 = _6 + _2.3;
Goto(bb10)
}
bb10 = {
_4 = -_1.3;
_26 = [6060200278724285844_i64,649312063146533449_i64,7010008882092305857_i64,2900021238139070907_i64,540999287606098404_i64];
_24 = [3964594382431607851_u64,4844593963372397045_u64];
_8 = (-129679154749506797484214768248153304742_i128) as u32;
_11 = 1601707920890750276_u64 as f64;
SetDiscriminant(_5, 2);
_5 = Adt51::Variant0 { fld0: 321804670848908579441595328347224060280_u128 };
_24 = [7346906387517119477_u64,6311999201999491491_u64];
(*_17) = _9;
Goto(bb11)
}
bb11 = {
_18 = [_6];
_20 = core::ptr::addr_of!(_15);
_1.0 = _2.0;
(*_17) = _9 & _9;
_18 = [_2.3];
_13 = (*_20);
_13 = _15;
_1 = (_2.0, _2.1, _7, _6);
_14 = [3_usize,13252897920184275644_usize,10514338790135990247_usize,2_usize,4_usize];
_1 = _2;
_20 = core::ptr::addr_of!((*_20));
_13 = (*_20);
_2.1 = [59093_u16,54672_u16,50531_u16,13238_u16];
(*_20) = _13;
_15 = _13;
Call(_2.0 = fn16(_17, (*_17), _7, _10, _4, _7, _10, (*_17), _1.3, _2.2, _2.2, _18, _1.3, _3), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_19 = [(*_20),_13,_15];
_20 = core::ptr::addr_of!(_15);
_2.1 = [53062_u16,21005_u16,15016_u16,64702_u16];
_10 = _1.2;
_20 = core::ptr::addr_of!((*_20));
_16 = _18;
_1.1 = _2.1;
_22 = _21;
_29.2 = 102138019793203841917708640431696865739_i128;
(*_17) = _9;
_5 = Adt51::Variant0 { fld0: 79440960413217467116383589092605191846_u128 };
_16 = [_1.3];
_1 = (_2.0, _2.1, _10, _4);
_29.1.1 = RET;
_30 = (-121_isize) * (-128_isize);
RET = _29.1.1;
Goto(bb13)
}
bb13 = {
RET = _29.1.1;
_2 = (_1.0, _1.1, _1.2, _4);
_7 = (*_17) >= (*_17);
_13 = (*_20);
Goto(bb14)
}
bb14 = {
_20 = core::ptr::addr_of!((*_20));
_32 = _11;
_21 = [3963558024327964413_u64,8498935466019983531_u64];
_16 = [_2.3];
_1.1 = [13385_u16,34870_u16,22560_u16,41159_u16];
_13 = _15;
_13 = _15;
_2 = (_26, _1.1, _10, _1.3);
_33 = _6 as u64;
_1 = _2;
RET = _29.1.1;
_28 = _24;
_2.1 = _1.1;
_18 = [_1.3];
_16 = _18;
_31 = (-13200_i16) as i8;
place!(Field::<u128>(Variant(_5, 0), 0)) = !175464383671811470364425242686285748780_u128;
_29.1.1 = RET;
_8 = _30 as u32;
Goto(bb15)
}
bb15 = {
Call(_37 = dump_var(10_usize, 33_usize, Move(_33), 6_usize, Move(_6), 21_usize, Move(_21), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_37 = dump_var(10_usize, 30_usize, Move(_30), 31_usize, Move(_31), 26_usize, Move(_26), 7_usize, Move(_7)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_37 = dump_var(10_usize, 9_usize, Move(_9), 4_usize, Move(_4), 13_usize, Move(_13), 15_usize, Move(_15)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: u128,mut _2: i32,mut _3: u32,mut _4: i32) -> *mut i16 {
mir! {
type RET = *mut i16;
let _5: u128;
let _6: ([i64; 5], [u16; 4], bool, i32);
let _7: f64;
let _8: i32;
let _9: [i64; 5];
let _10: f32;
let _11: bool;
let _12: f64;
let _13: f64;
let _14: f32;
let _15: [u16; 4];
let _16: Adt52;
let _17: [u128; 4];
let _18: (char, (u32, (*mut u32, *mut i16), i128));
let _19: [u64; 2];
let _20: char;
let _21: f32;
let _22: Adt56;
let _23: *mut i16;
let _24: bool;
let _25: ([i64; 5], [u16; 4], bool, i32);
let _26: f32;
let _27: usize;
let _28: bool;
let _29: [i32; 1];
let _30: f64;
let _31: isize;
let _32: [u64; 2];
let _33: [u16; 4];
let _34: f64;
let _35: [char; 3];
let _36: ();
let _37: ();
{
_5 = _1;
_3 = 2203753369_u32 & 1528546830_u32;
_1 = _2 as u128;
_2 = (-1400_i16) as i32;
_5 = !_1;
_5 = '\u{8e0ba}' as u128;
_3 = !1887216940_u32;
_6.1 = [55418_u16,14100_u16,25113_u16,48325_u16];
_6.3 = _4 - _4;
_5 = !_1;
_6.1 = [35353_u16,30212_u16,18737_u16,17031_u16];
_6.0 = [(-8210599234781338591_i64),1764312681920776729_i64,(-640028832419876278_i64),5796973213325839663_i64,(-3008758717444333928_i64)];
_6.1 = [31936_u16,9895_u16,19570_u16,32324_u16];
_6.2 = _6.3 > _6.3;
_5 = _1;
_6.3 = -_4;
_1 = _5 >> _6.3;
_6.1 = [17811_u16,39417_u16,3253_u16,54152_u16];
_6.1 = [12441_u16,49485_u16,54573_u16,17063_u16];
_6.2 = !true;
_6.3 = !_4;
_4 = _6.3 + _2;
_4 = -_6.3;
_6.3 = _4 << _5;
_1 = !_5;
_6.3 = 15972_u16 as i32;
Goto(bb1)
}
bb1 = {
_6.3 = 32012_i16 as i32;
_7 = (-110754373540550823595403128975144510534_i128) as f64;
_4 = _2;
_7 = (-6687_i16) as f64;
_6.2 = !false;
_1 = _5 + _5;
Goto(bb2)
}
bb2 = {
_5 = _6.2 as u128;
_3 = 1124736297_u32 ^ 463512493_u32;
_1 = _5 >> _5;
_2 = _6.3 | _6.3;
_9 = [6071560787154609830_i64,2369409686108876828_i64,(-1482800623159814758_i64),7986297489880285627_i64,8261375449570978975_i64];
_2 = _6.3;
_8 = _6.3 >> _3;
_3 = 1475301359_u32 << _4;
_1 = !_5;
_6.1 = [64416_u16,49087_u16,38838_u16,26590_u16];
_6.3 = '\u{fe164}' as i32;
_2 = -_8;
_4 = -_8;
_8 = -_4;
_9 = _6.0;
_7 = 31592090211871051400314857762087632951_i128 as f64;
Goto(bb3)
}
bb3 = {
_9 = _6.0;
_6.0 = [5572017994274002787_i64,7578678607708503871_i64,2854902967921361988_i64,1300661132529381371_i64,2665989575379097030_i64];
_7 = _3 as f64;
_7 = 1886596969625654523_usize as f64;
_13 = -_7;
_3 = 133630421_u32 >> _8;
_12 = _13;
_9 = [(-3747745468919957425_i64),(-1287627536371852810_i64),(-8004956035937332169_i64),(-6197332155139728467_i64),2858328503283890832_i64];
_12 = (-4933609677766859955_i64) as f64;
_11 = !_6.2;
_11 = _6.2 & _6.2;
_4 = _2 | _2;
_2 = -_4;
_8 = 5_usize as i32;
Goto(bb4)
}
bb4 = {
_1 = !_5;
_10 = _3 as f32;
_7 = (-2155796289279283111_i64) as f64;
_7 = 772419733413132477_u64 as f64;
_6.2 = !_11;
_5 = (-47683087344994462164869636295816714465_i128) as u128;
_16.fld3 = (-94_i8);
_16.fld7.1.3 = 126_isize as i32;
_16.fld5 = core::ptr::addr_of_mut!(_16.fld7.2.4.0);
_16.fld7.2.1 = '\u{112e7}' as u16;
_17 = [_5,_5,_1,_5];
_16.fld7.2.0 = _13;
_16.fld6 = Adt44::Variant2 { fld0: 62056082622653948181723175783325809364_i128,fld1: 828140320147684759_u64,fld2: (-92_isize),fld3: _16.fld3,fld4: 8_u8 };
_6.0 = [(-988728716885291397_i64),402816136680872100_i64,(-8102309670383249662_i64),(-3631204602736176314_i64),8404856264894749581_i64];
_16.fld7.1.2 = !_11;
_16.fld1.2 = !_11;
_18.1.0 = _3 * _3;
_18.1.2 = _5 as i128;
_16.fld7.0.0 = '\u{691c7}';
_17 = [_5,_5,_5,_5];
_16.fld7.1.0 = [2885940009673674678_i64,(-4634648799716972614_i64),(-4550797332970969588_i64),5052339650799995104_i64,(-4655415657899816165_i64)];
_16.fld7.2.4.1.0 = core::ptr::addr_of_mut!(_16.fld0);
place!(Field::<i8>(Variant(_16.fld6, 2), 3)) = -_16.fld3;
_16.fld1.1 = [_16.fld7.2.1,_16.fld7.2.1,_16.fld7.2.1,_16.fld7.2.1];
Call(_16.fld7.0.1 = fn12(_18.1.0, _16.fld1.1, _2, _9, _16.fld7.1.0, _11, _4, _16.fld7.1.0, _16.fld7.2.1, _16.fld7.1.0, _6, _9, _6.1, _16.fld3, _6), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
match _16.fld3 {
0 => bb1,
1 => bb4,
2 => bb3,
3 => bb6,
340282366920938463463374607431768211362 => bb8,
_ => bb7
}
}
bb6 = {
_1 = !_5;
_10 = _3 as f32;
_7 = (-2155796289279283111_i64) as f64;
_7 = 772419733413132477_u64 as f64;
_6.2 = !_11;
_5 = (-47683087344994462164869636295816714465_i128) as u128;
_16.fld3 = (-94_i8);
_16.fld7.1.3 = 126_isize as i32;
_16.fld5 = core::ptr::addr_of_mut!(_16.fld7.2.4.0);
_16.fld7.2.1 = '\u{112e7}' as u16;
_17 = [_5,_5,_1,_5];
_16.fld7.2.0 = _13;
_16.fld6 = Adt44::Variant2 { fld0: 62056082622653948181723175783325809364_i128,fld1: 828140320147684759_u64,fld2: (-92_isize),fld3: _16.fld3,fld4: 8_u8 };
_6.0 = [(-988728716885291397_i64),402816136680872100_i64,(-8102309670383249662_i64),(-3631204602736176314_i64),8404856264894749581_i64];
_16.fld7.1.2 = !_11;
_16.fld1.2 = !_11;
_18.1.0 = _3 * _3;
_18.1.2 = _5 as i128;
_16.fld7.0.0 = '\u{691c7}';
_17 = [_5,_5,_5,_5];
_16.fld7.1.0 = [2885940009673674678_i64,(-4634648799716972614_i64),(-4550797332970969588_i64),5052339650799995104_i64,(-4655415657899816165_i64)];
_16.fld7.2.4.1.0 = core::ptr::addr_of_mut!(_16.fld0);
place!(Field::<i8>(Variant(_16.fld6, 2), 3)) = -_16.fld3;
_16.fld1.1 = [_16.fld7.2.1,_16.fld7.2.1,_16.fld7.2.1,_16.fld7.2.1];
Call(_16.fld7.0.1 = fn12(_18.1.0, _16.fld1.1, _2, _9, _16.fld7.1.0, _11, _4, _16.fld7.1.0, _16.fld7.2.1, _16.fld7.1.0, _6, _9, _6.1, _16.fld3, _6), ReturnTo(bb5), UnwindUnreachable())
}
bb7 = {
_6.3 = 32012_i16 as i32;
_7 = (-110754373540550823595403128975144510534_i128) as f64;
_4 = _2;
_7 = (-6687_i16) as f64;
_6.2 = !false;
_1 = _5 + _5;
Goto(bb2)
}
bb8 = {
_16.fld7.2.4.1.1 = _16.fld7.0.1.1.1;
_6.3 = _16.fld3 as i32;
_16.fld7.1 = _6;
_6.0 = [(-8128972742439885917_i64),(-1909206251934504083_i64),2827311282789282119_i64,(-1653159452679761441_i64),(-4829416393974406196_i64)];
_18.1.1.1 = _16.fld7.2.4.1.1;
Goto(bb9)
}
bb9 = {
RET = _18.1.1.1;
_25.3 = _16.fld7.1.3;
_16.fld7.2 = (_12, 54575_u16, _11, _13, _16.fld7.0.1);
_6.1 = [_16.fld7.2.1,_16.fld7.2.1,_16.fld7.2.1,_16.fld7.2.1];
place!(Field::<u64>(Variant(_16.fld6, 2), 1)) = 16435669913847901233_u64;
_6.2 = _16.fld7.1.2 & _16.fld1.2;
Call(_16.fld7.2.1 = fn13(_18.1.0, _16.fld7.0.1.1.1, _16.fld7.1, _3, _16.fld7.0.1, _16.fld7.0.1.2, _16.fld7.0.1, _16.fld7.2.4, _16.fld7.2.2, _16.fld7.2.4, _6.1, _16.fld7.0.1, _16.fld7.2.4, _16.fld1.2), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_18.1.0 = !_16.fld7.0.1.0;
_8 = _4 * _4;
_14 = _10;
place!(Field::<i128>(Variant(_16.fld6, 2), 0)) = _16.fld7.0.1.2 << _16.fld7.2.4.2;
place!(Field::<isize>(Variant(_16.fld6, 2), 2)) = 6_u8 as isize;
_18.1.1 = (_16.fld5, _16.fld7.0.1.1.1);
_13 = -_16.fld7.2.3;
_3 = _16.fld7.0.1.0;
_11 = _16.fld7.1.2 ^ _6.2;
_16.fld7.2.0 = -_12;
_16.fld7.1.2 = !_16.fld1.2;
place!(Field::<u64>(Variant(_16.fld6, 2), 1)) = 3683927041086746832_i64 as u64;
_15 = [_16.fld7.2.1,_16.fld7.2.1,_16.fld7.2.1,_16.fld7.2.1];
_25 = _6;
_16.fld7.2.4.1 = (_16.fld5, _18.1.1.1);
_4 = !_8;
_25.3 = _8 & _8;
_16.fld7.1.0 = [(-6139346257960272317_i64),(-413610873643998761_i64),(-2315243303158271310_i64),(-6499503322620747055_i64),4742623222511877421_i64];
Goto(bb11)
}
bb11 = {
_16.fld0 = !_3;
_18.0 = _16.fld7.0.0;
_31 = -Field::<isize>(Variant(_16.fld6, 2), 2);
_21 = -_14;
_16.fld1.3 = _5 as i32;
_20 = _16.fld7.0.0;
_26 = 141_u8 as f32;
_10 = _21;
_9 = _16.fld7.1.0;
_16.fld7.2.4.1.1 = RET;
_16.fld7.2.4.0 = _18.1.0 & _3;
_27 = 1_usize;
match _25.1[_27] {
0 => bb4,
1 => bb2,
2 => bb12,
3 => bb13,
54575 => bb15,
_ => bb14
}
}
bb12 = {
_6.3 = 32012_i16 as i32;
_7 = (-110754373540550823595403128975144510534_i128) as f64;
_4 = _2;
_7 = (-6687_i16) as f64;
_6.2 = !false;
_1 = _5 + _5;
Goto(bb2)
}
bb13 = {
_1 = !_5;
_10 = _3 as f32;
_7 = (-2155796289279283111_i64) as f64;
_7 = 772419733413132477_u64 as f64;
_6.2 = !_11;
_5 = (-47683087344994462164869636295816714465_i128) as u128;
_16.fld3 = (-94_i8);
_16.fld7.1.3 = 126_isize as i32;
_16.fld5 = core::ptr::addr_of_mut!(_16.fld7.2.4.0);
_16.fld7.2.1 = '\u{112e7}' as u16;
_17 = [_5,_5,_1,_5];
_16.fld7.2.0 = _13;
_16.fld6 = Adt44::Variant2 { fld0: 62056082622653948181723175783325809364_i128,fld1: 828140320147684759_u64,fld2: (-92_isize),fld3: _16.fld3,fld4: 8_u8 };
_6.0 = [(-988728716885291397_i64),402816136680872100_i64,(-8102309670383249662_i64),(-3631204602736176314_i64),8404856264894749581_i64];
_16.fld7.1.2 = !_11;
_16.fld1.2 = !_11;
_18.1.0 = _3 * _3;
_18.1.2 = _5 as i128;
_16.fld7.0.0 = '\u{691c7}';
_17 = [_5,_5,_5,_5];
_16.fld7.1.0 = [2885940009673674678_i64,(-4634648799716972614_i64),(-4550797332970969588_i64),5052339650799995104_i64,(-4655415657899816165_i64)];
_16.fld7.2.4.1.0 = core::ptr::addr_of_mut!(_16.fld0);
place!(Field::<i8>(Variant(_16.fld6, 2), 3)) = -_16.fld3;
_16.fld1.1 = [_16.fld7.2.1,_16.fld7.2.1,_16.fld7.2.1,_16.fld7.2.1];
Call(_16.fld7.0.1 = fn12(_18.1.0, _16.fld1.1, _2, _9, _16.fld7.1.0, _11, _4, _16.fld7.1.0, _16.fld7.2.1, _16.fld7.1.0, _6, _9, _6.1, _16.fld3, _6), ReturnTo(bb5), UnwindUnreachable())
}
bb14 = {
_16.fld7.2.4.1.1 = _16.fld7.0.1.1.1;
_6.3 = _16.fld3 as i32;
_16.fld7.1 = _6;
_6.0 = [(-8128972742439885917_i64),(-1909206251934504083_i64),2827311282789282119_i64,(-1653159452679761441_i64),(-4829416393974406196_i64)];
_18.1.1.1 = _16.fld7.2.4.1.1;
Goto(bb9)
}
bb15 = {
_16.fld7.2.4.2 = 200_u8 as i128;
_18 = _16.fld7.0;
_16.fld1.3 = _25.3 - _8;
_16.fld7.1.1[_27] = _3 as u16;
_28 = _25.2 & _11;
_24 = _15[_27] == _15[_27];
_16.fld4 = !_25.1[_27];
_16.fld7.2.4.1.1 = _18.1.1.1;
_16.fld7.1.0 = _25.0;
_32 = [Field::<u64>(Variant(_16.fld6, 2), 1),Field::<u64>(Variant(_16.fld6, 2), 1)];
_20 = _18.0;
_9[_27] = _15[_27] as i64;
_18.1.1.0 = core::ptr::addr_of_mut!(_16.fld7.0.1.0);
_28 = _24 | _24;
_25.1[_27] = _27 as u16;
_6.2 = _28;
_16.fld1.1 = [_6.1[_27],_15[_27],_16.fld7.2.1,_16.fld7.2.1];
RET = _16.fld7.2.4.1.1;
_31 = _6.1[_27] as isize;
_16.fld1.2 = !_6.2;
_16.fld7.2.4.1 = (_16.fld7.0.1.1.0, _18.1.1.1);
_9[_27] = _16.fld7.1.0[_27];
_22 = Adt56::Variant1 { fld0: Field::<i128>(Variant(_16.fld6, 2), 0),fld1: _16.fld7.0 };
_16.fld5 = Field::<(char, (u32, (*mut u32, *mut i16), i128))>(Variant(_22, 1), 1).1.1.0;
_18.1.1.1 = _16.fld7.0.1.1.1;
_25.0[_27] = _16.fld7.1.0[_27];
_16.fld7.2.4.1 = (_16.fld5, _16.fld7.0.1.1.1);
Goto(bb16)
}
bb16 = {
Call(_36 = dump_var(11_usize, 6_usize, Move(_6), 32_usize, Move(_32), 25_usize, Move(_25), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(11_usize, 5_usize, Move(_5), 17_usize, Move(_17), 11_usize, Move(_11), 24_usize, Move(_24)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_36 = dump_var(11_usize, 20_usize, Move(_20), 37_usize, _37, 37_usize, _37, 37_usize, _37), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: u32,mut _2: [u16; 4],mut _3: i32,mut _4: [i64; 5],mut _5: [i64; 5],mut _6: bool,mut _7: i32,mut _8: [i64; 5],mut _9: u16,mut _10: [i64; 5],mut _11: ([i64; 5], [u16; 4], bool, i32),mut _12: [i64; 5],mut _13: [u16; 4],mut _14: i8,mut _15: ([i64; 5], [u16; 4], bool, i32)) -> (u32, (*mut u32, *mut i16), i128) {
mir! {
type RET = (u32, (*mut u32, *mut i16), i128);
let _16: char;
let _17: f64;
let _18: i128;
let _19: i16;
let _20: i128;
let _21: i32;
let _22: char;
let _23: [u16; 4];
let _24: ([i64; 5], [u16; 4], bool, i32);
let _25: f32;
let _26: isize;
let _27: *mut i16;
let _28: (u32, (*mut u32, *mut i16), i128);
let _29: [i32; 1];
let _30: (*const (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),);
let _31: u8;
let _32: ();
let _33: ();
{
_6 = _14 >= _14;
_2 = [_9,_9,_9,_9];
_15.0 = [944028085180252816_i64,1217337786253560293_i64,5441831692268993603_i64,(-1850406946845432394_i64),8900272344631793455_i64];
_15 = (_12, _11.1, _6, _3);
RET.1.0 = core::ptr::addr_of_mut!(_1);
_11.1 = _15.1;
RET.2 = 72911448298731119302635805422333817129_i128;
RET.0 = (-18_isize) as u32;
_10 = _4;
_17 = _14 as f64;
_16 = '\u{a9a67}';
_17 = _14 as f64;
_19 = !4473_i16;
_15.2 = _11.2;
_13 = [_9,_9,_9,_9];
_11 = (_5, _13, _6, _15.3);
_15.0 = [(-9180629529687053953_i64),5628726948754268982_i64,6255166258723281527_i64,4446622379712063658_i64,956015576308201830_i64];
_6 = _11.3 > _15.3;
RET.1.0 = core::ptr::addr_of_mut!(RET.0);
match RET.2 {
0 => bb1,
1 => bb2,
72911448298731119302635805422333817129 => bb4,
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
_20 = 4571980382770539216_i64 as i128;
_18 = _20 | RET.2;
RET.2 = _18 ^ _18;
_16 = '\u{c6813}';
_24 = (_4, _13, _6, _15.3);
_11 = (_12, _15.1, _24.2, _3);
_17 = 61_u8 as f64;
RET.1.1 = core::ptr::addr_of_mut!(_19);
_25 = 8113031751507249435_u64 as f32;
_17 = 1460569277503962999_u64 as f64;
_24.3 = _11.3 - _11.3;
_8 = _24.0;
_11 = _24;
_6 = _11.2 >= _11.2;
_22 = _16;
RET.1.0 = core::ptr::addr_of_mut!(RET.0);
RET.0 = !_1;
_4 = _10;
_14 = -(-127_i8);
_27 = RET.1.1;
_28 = (RET.0, RET.1, RET.2);
_28.0 = RET.0;
_24 = _11;
_11.3 = _15.3;
_13 = _15.1;
Goto(bb5)
}
bb5 = {
_8 = [8756166998834297226_i64,(-1750106304942745154_i64),(-7252733756873577874_i64),(-434956220388944912_i64),3599793792534583774_i64];
_28.0 = 0_usize as u32;
_25 = 157_u8 as f32;
_28.1.1 = RET.1.1;
(*_27) = 22946_i16;
_11.3 = _24.3;
_7 = _24.3;
_2 = [_9,_9,_9,_9];
RET.1.0 = core::ptr::addr_of_mut!(RET.0);
_9 = !55251_u16;
_4 = [(-8780327565732305682_i64),(-761704875316194400_i64),7929829701076417457_i64,(-2662032546160324754_i64),(-367298863442694753_i64)];
_15.3 = (-3319307024711863688_i64) as i32;
match _19 {
0 => bb1,
22946 => bb6,
_ => bb3
}
}
bb6 = {
_1 = 7_usize as u32;
_24.2 = _6;
RET.2 = 6107779904510275568_u64 as i128;
_15.3 = _7;
_10 = [5799223636234306589_i64,(-1706629519693554939_i64),4643890280997150960_i64,(-3148049123505245380_i64),(-7985758514449351240_i64)];
RET = (_1, _28.1, _28.2);
_15.2 = _11.2 ^ _24.2;
_29 = [_7];
(*_27) = 11942_i16;
RET.1.1 = core::ptr::addr_of_mut!(_19);
RET.2 = _28.2 >> _7;
Goto(bb7)
}
bb7 = {
Call(_32 = dump_var(12_usize, 18_usize, Move(_18), 1_usize, Move(_1), 22_usize, Move(_22), 15_usize, Move(_15)), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Call(_32 = dump_var(12_usize, 3_usize, Move(_3), 5_usize, Move(_5), 20_usize, Move(_20), 29_usize, Move(_29)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_32 = dump_var(12_usize, 12_usize, Move(_12), 24_usize, Move(_24), 2_usize, Move(_2), 33_usize, _33), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: u32,mut _2: *mut i16,mut _3: ([i64; 5], [u16; 4], bool, i32),mut _4: u32,mut _5: (u32, (*mut u32, *mut i16), i128),mut _6: i128,mut _7: (u32, (*mut u32, *mut i16), i128),mut _8: (u32, (*mut u32, *mut i16), i128),mut _9: bool,mut _10: (u32, (*mut u32, *mut i16), i128),mut _11: [u16; 4],mut _12: (u32, (*mut u32, *mut i16), i128),mut _13: (u32, (*mut u32, *mut i16), i128),mut _14: bool) -> u16 {
mir! {
type RET = u16;
let _15: f32;
let _16: u32;
let _17: f32;
let _18: i64;
let _19: [u128; 4];
let _20: [u16; 4];
let _21: char;
let _22: ([i64; 5], [u16; 4], bool, i32);
let _23: isize;
let _24: f32;
let _25: f32;
let _26: isize;
let _27: *const [usize; 5];
let _28: u64;
let _29: isize;
let _30: [char; 3];
let _31: isize;
let _32: *const [usize; 5];
let _33: [char; 3];
let _34: [u16; 4];
let _35: *mut i128;
let _36: u128;
let _37: [char; 3];
let _38: [u128; 4];
let _39: f64;
let _40: [u64; 2];
let _41: [u128; 4];
let _42: ([i64; 5], [u16; 4], bool, i32);
let _43: isize;
let _44: [u64; 2];
let _45: Adt44;
let _46: u8;
let _47: i8;
let _48: [char; 3];
let _49: ([i64; 5], [u16; 4], bool, i32);
let _50: i32;
let _51: bool;
let _52: ();
let _53: ();
{
_7.0 = _8.0;
_10.1 = (_12.1.0, _8.1.1);
_4 = _1 | _1;
_13 = (_10.0, _12.1, _5.2);
_10.1.0 = core::ptr::addr_of_mut!(_13.0);
_12 = (_1, _8.1, _13.2);
_8 = (_1, _10.1, _13.2);
_8.2 = _12.2 + _12.2;
_7.1.1 = _13.1.1;
_7 = (_8.0, _10.1, _5.2);
_12.1.0 = _10.1.0;
_12.2 = _7.2 ^ _8.2;
_8.0 = _4 & _5.0;
_8.1.0 = core::ptr::addr_of_mut!(_4);
_13.1.1 = _7.1.1;
_10.0 = _8.0;
RET = _4 as u16;
_7.0 = (-9223372036854775808_isize) as u32;
_13 = (_8.0, _5.1, _10.2);
_17 = _13.0 as f32;
_15 = (-2153118814221400757_i64) as f32;
RET = !20719_u16;
_15 = _17;
Goto(bb1)
}
bb1 = {
_5.2 = -_10.2;
_10.2 = -_5.2;
Call(_14 = fn14(_10, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_6 = _12.2;
_11 = [RET,RET,RET,RET];
_5.1 = (_12.1.0, _2);
_10 = (_13.0, _5.1, _6);
_17 = 6_usize as f32;
_5.1.0 = _10.1.0;
_8.1.1 = _13.1.1;
_5.0 = !_1;
_13 = (_4, _10.1, _8.2);
_5.1.0 = core::ptr::addr_of_mut!(_8.0);
_8.1.1 = _13.1.1;
_22.2 = _14;
Goto(bb3)
}
bb3 = {
_22 = (_3.0, _11, _14, _3.3);
_22 = (_3.0, _3.1, _14, _3.3);
_7 = _10;
_24 = _15 - _15;
_5.1 = _7.1;
_7 = _8;
_19 = [246300615862788550847772960141446562221_u128,90020016957269967374721240942448427021_u128,100433529344467052778036702027203697244_u128,63905316107459105615582803011368894789_u128];
RET = !33542_u16;
_10.1 = (_13.1.0, _5.1.1);
Goto(bb4)
}
bb4 = {
_13.1.1 = _8.1.1;
_8.1 = _5.1;
_5.1.1 = _10.1.1;
_3.0 = _22.0;
_16 = 14638626530970709483_u64 as u32;
_3.2 = _14 == _14;
_7.2 = _12.2 + _6;
RET = 5921_u16 | 54459_u16;
_4 = _7.0;
_10.1 = (_5.1.0, _5.1.1);
_25 = _24 - _24;
_10 = (_4, _8.1, _12.2);
_22.0 = _3.0;
_3.2 = _4 == _13.0;
RET = !42330_u16;
_12 = (_1, _8.1, _10.2);
_1 = _13.0 * _8.0;
_26 = (-32_isize) ^ (-9223372036854775808_isize);
_21 = '\u{775c1}';
_3.3 = _22.3;
_9 = _22.2;
_10.2 = _6 | _8.2;
_10.0 = _1;
Goto(bb5)
}
bb5 = {
_12.0 = 12990_i16 as u32;
_3 = (_22.0, _22.1, _14, _22.3);
_12.1 = (_10.1.0, _10.1.1);
_12.0 = _4;
_17 = _25;
_3.3 = _22.3;
_24 = _17 - _25;
_6 = _8.2 - _12.2;
_12 = _10;
_12.1 = _8.1;
_10.2 = _7.2;
_5.1.1 = _13.1.1;
_12 = _10;
_2 = _12.1.1;
_12.2 = !_7.2;
_22.1 = [RET,RET,RET,RET];
_18 = 6780067639612004570_i64;
_21 = '\u{da7ae}';
_11 = [RET,RET,RET,RET];
_13 = (_7.0, _5.1, _8.2);
_6 = -_7.2;
_7.1.1 = _8.1.1;
_10 = _13;
Call(_20 = core::intrinsics::transmute(_3.1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_28 = !68826165700688406_u64;
_22.1 = [RET,RET,RET,RET];
_10.1 = _12.1;
_17 = _15 + _25;
RET = 62777_u16 - 32872_u16;
_8.2 = _6 << _6;
_22 = (_3.0, _20, _14, _3.3);
_4 = !_12.0;
match _18 {
6780067639612004570 => bb7,
_ => bb1
}
}
bb7 = {
_18 = (-7807_i16) as i64;
_24 = _18 as f32;
_7.2 = _6 << _12.2;
_12.2 = _6;
_10.1.1 = _8.1.1;
_12.2 = _10.2 * _7.2;
_22.0 = _3.0;
_10 = (_13.0, _12.1, _8.2);
_7 = (_13.0, _12.1, _8.2);
_13.1 = _10.1;
_18 = (-4080688042221404582_i64) - 7316688319994409121_i64;
_30 = [_21,_21,_21];
_12.0 = !_13.0;
_34 = [RET,RET,RET,RET];
_5.0 = !_10.0;
_21 = '\u{61761}';
_38 = [126940782929206222757994113448177684462_u128,234680415069421472097320735121476269052_u128,55135554974520377640693519791912129720_u128,31590277771952841933495295553884225846_u128];
_29 = -_26;
_11 = [RET,RET,RET,RET];
_5.1 = _13.1;
_7.1.0 = _5.1.0;
_3 = _22;
Goto(bb8)
}
bb8 = {
_6 = _22.3 as i128;
_13.1.0 = core::ptr::addr_of_mut!(_12.0);
_22.1 = [RET,RET,RET,RET];
Goto(bb9)
}
bb9 = {
_17 = _25;
_42.1 = _3.1;
_22.1 = [RET,RET,RET,RET];
_40 = [_28,_28];
_42.2 = !_14;
_5.2 = 43341016618331069638433344900863374398_u128 as i128;
_5.1 = (_12.1.0, _12.1.1);
_7.2 = _12.2;
_42 = (_22.0, _20, _14, _22.3);
_38 = [129605770407444832834999327559877297751_u128,289210963643715925039223067970444662159_u128,203040157860482801477261752396892995076_u128,187438363414047642354268521787504684640_u128];
Goto(bb10)
}
bb10 = {
RET = _21 as u16;
_11 = [RET,RET,RET,RET];
_23 = _26 << _8.2;
_8 = (_4, _5.1, _13.2);
_12.2 = _13.2 - _7.2;
_2 = _5.1.1;
_43 = _7.2 as isize;
_42.3 = _22.3 ^ _3.3;
_35 = core::ptr::addr_of_mut!(_8.2);
_22.1 = [RET,RET,RET,RET];
_3.3 = _3.2 as i32;
_3 = (_42.0, _11, _14, _42.3);
_22.0 = _3.0;
_31 = _23 + _43;
_12.0 = !_1;
_13.1.0 = _12.1.0;
_47 = (-45_i8) + 18_i8;
_3 = _42;
_33 = [_21,_21,_21];
Goto(bb11)
}
bb11 = {
_7.1.1 = _13.1.1;
_18 = (*_35) as i64;
_44 = _40;
_46 = 63_u8 ^ 204_u8;
_8.1 = _10.1;
_10.1.1 = _5.1.1;
_42.3 = -_3.3;
_13 = (_8.0, _10.1, _12.2);
_3.3 = _42.3;
_41 = [21732618041130006830970391416572251145_u128,3438311222693001939646303360158396172_u128,235573038323083218364957249571126101627_u128,97446886398462471322370226129607655724_u128];
_7.1 = (_13.1.0, _2);
RET = 33885_u16 >> _43;
_7 = (_8.0, _13.1, _13.2);
_5.1 = (_7.1.0, _7.1.1);
_42 = _22;
_3.3 = _46 as i32;
_4 = _31 as u32;
_34 = [RET,RET,RET,RET];
_29 = _43;
_26 = _31 | _43;
Goto(bb12)
}
bb12 = {
Call(_52 = dump_var(13_usize, 18_usize, Move(_18), 44_usize, Move(_44), 4_usize, Move(_4), 31_usize, Move(_31)), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Call(_52 = dump_var(13_usize, 38_usize, Move(_38), 11_usize, Move(_11), 9_usize, Move(_9), 29_usize, Move(_29)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
Call(_52 = dump_var(13_usize, 43_usize, Move(_43), 23_usize, Move(_23), 16_usize, Move(_16), 1_usize, Move(_1)), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
Call(_52 = dump_var(13_usize, 46_usize, Move(_46), 47_usize, Move(_47), 53_usize, _53, 53_usize, _53), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: (u32, (*mut u32, *mut i16), i128),mut _2: i128) -> bool {
mir! {
type RET = bool;
let _3: *mut i128;
let _4: (*mut u32, *mut i16);
let _5: Adt55;
let _6: [u64; 2];
let _7: ();
let _8: ();
{
RET = !true;
RET = !true;
_1.2 = _2 + _2;
_1.2 = _2;
_3 = core::ptr::addr_of_mut!(_1.2);
_3 = core::ptr::addr_of_mut!(_2);
RET = _1.2 > _2;
_4 = (_1.1.0, _1.1.1);
(*_3) = _1.2 - _1.2;
_4.1 = _1.1.1;
_4 = (_1.1.0, _1.1.1);
_5 = Adt55::Variant2 { fld0: RET,fld1: 45_isize };
_1.1.0 = _4.0;
place!(Field::<isize>(Variant(_5, 2), 1)) = 11_isize;
Goto(bb1)
}
bb1 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: bool,mut _2: ([i64; 5], [u16; 4], bool, i32),mut _3: ([i64; 5], [u16; 4], bool, i32)) -> i32 {
mir! {
type RET = i32;
let _4: [u64; 2];
let _5: isize;
let _6: u16;
let _7: ();
let _8: ();
{
_2.3 = (-9223372036854775808_isize) as i32;
_2 = (_3.0, _3.1, _3.2, _3.3);
RET = !_3.3;
_2 = (_3.0, _3.1, _1, RET);
_6 = 58819_u16;
_4 = [8073191549077164155_u64,18116846244738557145_u64];
_2 = (_3.0, _3.1, _3.2, _3.3);
_5 = !9223372036854775807_isize;
_3.0 = [(-3244678387305923149_i64),7477674962290878548_i64,(-7351686428866120405_i64),4197640573563563305_i64,(-7613268789075742898_i64)];
_3.0 = _2.0;
_2.2 = _1 > _1;
Goto(bb1)
}
bb1 = {
Call(_7 = dump_var(15_usize, 4_usize, Move(_4), 1_usize, Move(_1), 2_usize, Move(_2), 8_usize, _8), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: *mut u32,mut _2: u32,mut _3: bool,mut _4: bool,mut _5: i32,mut _6: bool,mut _7: bool,mut _8: u32,mut _9: i32,mut _10: bool,mut _11: bool,mut _12: [i32; 1],mut _13: i32,mut _14: u32) -> [i64; 5] {
mir! {
type RET = [i64; 5];
let _15: isize;
let _16: char;
let _17: *mut u32;
let _18: Adt51;
let _19: [i64; 5];
let _20: *const i32;
let _21: char;
let _22: i8;
let _23: u128;
let _24: [char; 3];
let _25: [u128; 4];
let _26: u8;
let _27: *const char;
let _28: ([i64; 5], [u16; 4], bool, i32);
let _29: ((char, (u32, (*mut u32, *mut i16), i128)), ([i64; 5], [u16; 4], bool, i32), (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)));
let _30: [char; 3];
let _31: [i32; 1];
let _32: isize;
let _33: Adt54;
let _34: *mut *mut i16;
let _35: ();
let _36: ();
{
RET = [8647621980931974117_i64,478398681446676624_i64,3959332424233877867_i64,1904992923822691176_i64,(-3253955275680424095_i64)];
(*_1) = !_2;
_14 = (-9223372036854775808_isize) as u32;
_8 = !_2;
_12 = [_13];
_7 = _4 & _10;
_6 = _10;
Goto(bb1)
}
bb1 = {
_3 = !_10;
_9 = _5;
_11 = _10 > _6;
_9 = _13 ^ _13;
_12 = [_13];
_8 = (*_1);
RET = [7245375258776804757_i64,288976820812348238_i64,8328619584994690714_i64,(-3569433827513858129_i64),(-2552289770532895308_i64)];
_16 = '\u{ac9fa}';
RET = [(-7976120318266673790_i64),(-5640223024871447959_i64),(-4426833408869002996_i64),(-9044843601959975715_i64),6404705011799219232_i64];
_15 = 9223372036854775807_isize << (*_1);
_10 = _8 > (*_1);
_7 = !_6;
_10 = _4 | _7;
_13 = _9;
(*_1) = _5 as u32;
_14 = !_2;
_15 = (-114_isize);
_17 = core::ptr::addr_of_mut!(_2);
Goto(bb2)
}
bb2 = {
_9 = _13 >> (*_1);
_2 = _15 as u32;
_4 = _3;
_3 = _6 | _11;
_4 = _11;
(*_17) = (*_1) * _8;
_20 = core::ptr::addr_of!(_9);
(*_1) = _2;
_6 = !_10;
(*_1) = !_2;
match _15 {
0 => bb3,
340282366920938463463374607431768211342 => bb5,
_ => bb4
}
}
bb3 = {
_3 = !_10;
_9 = _5;
_11 = _10 > _6;
_9 = _13 ^ _13;
_12 = [_13];
_8 = (*_1);
RET = [7245375258776804757_i64,288976820812348238_i64,8328619584994690714_i64,(-3569433827513858129_i64),(-2552289770532895308_i64)];
_16 = '\u{ac9fa}';
RET = [(-7976120318266673790_i64),(-5640223024871447959_i64),(-4426833408869002996_i64),(-9044843601959975715_i64),6404705011799219232_i64];
_15 = 9223372036854775807_isize << (*_1);
_10 = _8 > (*_1);
_7 = !_6;
_10 = _4 | _7;
_13 = _9;
(*_1) = _5 as u32;
_14 = !_2;
_15 = (-114_isize);
_17 = core::ptr::addr_of_mut!(_2);
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
_3 = !_10;
_17 = core::ptr::addr_of_mut!((*_1));
_12 = [_9];
Call(_15 = fn17(_8, _9, _8, _3, _1, _17), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_19 = RET;
_21 = _16;
_23 = 174366596803160294560616823529578594650_u128 << _13;
_3 = (*_1) >= _2;
Goto(bb7)
}
bb7 = {
_22 = !(-41_i8);
_25 = [_23,_23,_23,_23];
_1 = core::ptr::addr_of_mut!(_2);
(*_20) = _13;
_21 = _16;
_16 = _21;
_17 = core::ptr::addr_of_mut!(_2);
(*_20) = -_5;
_8 = !_14;
_14 = (*_1) + (*_17);
(*_17) = 5150240299295711168_u64 as u32;
_19 = RET;
(*_1) = (*_20) as u32;
_19 = [(-5895887404664762791_i64),(-1481554687766349559_i64),2051988753579112746_i64,(-423589311165148587_i64),(-2712573861657773214_i64)];
_5 = (*_20);
(*_1) = 109_u8 as u32;
RET = _19;
(*_17) = _8;
_24 = [_16,_16,_21];
(*_1) = !_8;
(*_17) = _14;
_19 = [(-2703786718872626521_i64),(-3084765185313035813_i64),103386204459079165_i64,6502344449354067293_i64,(-131293148663044623_i64)];
_21 = _16;
_7 = _4 & _6;
_20 = core::ptr::addr_of!((*_20));
RET = [(-1084083064207314730_i64),(-1225174815691835308_i64),(-6242017629004080528_i64),4860617836192584559_i64,(-5847714698066730443_i64)];
_11 = _4 ^ _6;
_14 = _16 as u32;
RET = [(-4588055131521158341_i64),(-5732732502807877561_i64),7622273147683320193_i64,1390663009636666499_i64,(-4936498402702052999_i64)];
(*_20) = !_5;
_12 = [_9];
Goto(bb8)
}
bb8 = {
RET = [(-743222217834377334_i64),2861781408238729051_i64,4343739683206587222_i64,4288739430260532447_i64,(-3814978905101271809_i64)];
_20 = core::ptr::addr_of!(_9);
Goto(bb9)
}
bb9 = {
_26 = !191_u8;
_23 = !311780429639958899699755591602108773051_u128;
_16 = _21;
_17 = _1;
_14 = _2;
_24 = [_21,_16,_21];
_1 = _17;
_7 = _10;
(*_1) = _14 * _14;
_16 = _21;
_20 = core::ptr::addr_of!(_9);
Goto(bb10)
}
bb10 = {
_7 = _11 != _10;
_14 = !(*_17);
(*_17) = _8;
_23 = 268119395717822129701615550235802263718_u128;
_5 = _14 as i32;
_26 = 73_u8 ^ 128_u8;
_28.1 = [40340_u16,26032_u16,63414_u16,5289_u16];
_3 = _4 & _6;
_29.2.1 = 21439_u16 << _14;
_28.2 = !_4;
match _23 {
0 => bb11,
268119395717822129701615550235802263718 => bb13,
_ => bb12
}
}
bb11 = {
_3 = !_10;
_9 = _5;
_11 = _10 > _6;
_9 = _13 ^ _13;
_12 = [_13];
_8 = (*_1);
RET = [7245375258776804757_i64,288976820812348238_i64,8328619584994690714_i64,(-3569433827513858129_i64),(-2552289770532895308_i64)];
_16 = '\u{ac9fa}';
RET = [(-7976120318266673790_i64),(-5640223024871447959_i64),(-4426833408869002996_i64),(-9044843601959975715_i64),6404705011799219232_i64];
_15 = 9223372036854775807_isize << (*_1);
_10 = _8 > (*_1);
_7 = !_6;
_10 = _4 | _7;
_13 = _9;
(*_1) = _5 as u32;
_14 = !_2;
_15 = (-114_isize);
_17 = core::ptr::addr_of_mut!(_2);
Goto(bb2)
}
bb12 = {
RET = [(-743222217834377334_i64),2861781408238729051_i64,4343739683206587222_i64,4288739430260532447_i64,(-3814978905101271809_i64)];
_20 = core::ptr::addr_of!(_9);
Goto(bb9)
}
bb13 = {
_29.1.2 = _4;
_16 = _21;
(*_20) = _15 as i32;
_29.2.4.2 = 16941395593478170564_usize as i128;
Goto(bb14)
}
bb14 = {
_9 = _5 * _5;
(*_20) = _5 | _5;
_29.2.1 = 14778697076048951745_u64 as u16;
_2 = _14;
_29.0.1.1.0 = _17;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(16_usize, 12_usize, Move(_12), 25_usize, Move(_25), 16_usize, Move(_16), 26_usize, Move(_26)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(16_usize, 21_usize, Move(_21), 4_usize, Move(_4), 8_usize, Move(_8), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(16_usize, 3_usize, Move(_3), 23_usize, Move(_23), 7_usize, Move(_7), 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: u32,mut _2: i32,mut _3: u32,mut _4: bool,mut _5: *mut u32,mut _6: *mut u32) -> isize {
mir! {
type RET = isize;
let _7: *mut f32;
let _8: [u128; 4];
let _9: bool;
let _10: Adt56;
let _11: *mut i128;
let _12: ((char, (u32, (*mut u32, *mut i16), i128)), ([i64; 5], [u16; 4], bool, i32), (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)));
let _13: [i32; 1];
let _14: isize;
let _15: isize;
let _16: isize;
let _17: [i32; 1];
let _18: isize;
let _19: [u128; 4];
let _20: [usize; 5];
let _21: i128;
let _22: [char; 3];
let _23: (i64, (*mut u32, *mut i16));
let _24: [u128; 4];
let _25: f32;
let _26: [i64; 5];
let _27: *mut i128;
let _28: isize;
let _29: [i32; 1];
let _30: [usize; 5];
let _31: u16;
let _32: Adt57;
let _33: Adt55;
let _34: char;
let _35: ();
let _36: ();
{
_2 = (-904728857_i32) >> (*_5);
_3 = !(*_5);
_5 = core::ptr::addr_of_mut!((*_5));
_5 = core::ptr::addr_of_mut!((*_5));
(*_5) = _3;
(*_5) = _1 & _3;
_1 = (*_5);
_8 = [93239568555010234976594702316564647613_u128,63477621595290994957075479032768521492_u128,199266071148424628297084137563543978531_u128,163483580542250625023720003810359727687_u128];
_8 = [277334743045514237692960051782836554101_u128,50567518576639834031429907900399243915_u128,139387448464743929802420778170646020301_u128,67340693326220429585480492814530978337_u128];
RET = '\u{b3458}' as isize;
_5 = _6;
Call((*_5) = core::intrinsics::bswap(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_5) = !_3;
(*_5) = !_3;
_5 = _6;
_5 = core::ptr::addr_of_mut!(_1);
_6 = core::ptr::addr_of_mut!((*_6));
RET = 86_isize >> _2;
(*_5) = !(*_6);
_2 = 3904972343480790340_u64 as i32;
_5 = core::ptr::addr_of_mut!((*_6));
(*_5) = 3634430023603180009_usize as u32;
RET = 60_isize;
RET = (-73_isize);
RET = (-3525350196484457715_i64) as isize;
_12.1.0 = [2623767936635833040_i64,1195460268253596970_i64,6542024245274464168_i64,(-8250330452352118669_i64),(-4769119743640963923_i64)];
_12.2.4.0 = !_1;
_12.0.0 = '\u{58d33}';
Goto(bb2)
}
bb2 = {
_12.1.2 = !_4;
_12.2.1 = _12.2.4.0 as u16;
_11 = core::ptr::addr_of_mut!(_12.0.1.2);
Call(_12.0.1.0 = core::intrinsics::transmute(_12.2.4.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = !(-9223372036854775808_isize);
_12.0.0 = '\u{41905}';
(*_11) = 6988819000119667037858888125631881228_i128;
match (*_11) {
0 => bb1,
1 => bb4,
6988819000119667037858888125631881228 => bb6,
_ => bb5
}
}
bb4 = {
_12.1.2 = !_4;
_12.2.1 = _12.2.4.0 as u16;
_11 = core::ptr::addr_of_mut!(_12.0.1.2);
Call(_12.0.1.0 = core::intrinsics::transmute(_12.2.4.0), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
(*_5) = !_3;
(*_5) = !_3;
_5 = _6;
_5 = core::ptr::addr_of_mut!(_1);
_6 = core::ptr::addr_of_mut!((*_6));
RET = 86_isize >> _2;
(*_5) = !(*_6);
_2 = 3904972343480790340_u64 as i32;
_5 = core::ptr::addr_of_mut!((*_6));
(*_5) = 3634430023603180009_usize as u32;
RET = 60_isize;
RET = (-73_isize);
RET = (-3525350196484457715_i64) as isize;
_12.1.0 = [2623767936635833040_i64,1195460268253596970_i64,6542024245274464168_i64,(-8250330452352118669_i64),(-4769119743640963923_i64)];
_12.2.4.0 = !_1;
_12.0.0 = '\u{58d33}';
Goto(bb2)
}
bb6 = {
_15 = RET;
RET = -_15;
_12.2.4.1.0 = core::ptr::addr_of_mut!(_3);
_12.2.1 = 41544_u16 - 56539_u16;
_9 = _12.1.2 != _12.1.2;
_12.2.4.1.0 = _5;
match (*_11) {
0 => bb7,
6988819000119667037858888125631881228 => bb9,
_ => bb8
}
}
bb7 = {
(*_5) = !_3;
(*_5) = !_3;
_5 = _6;
_5 = core::ptr::addr_of_mut!(_1);
_6 = core::ptr::addr_of_mut!((*_6));
RET = 86_isize >> _2;
(*_5) = !(*_6);
_2 = 3904972343480790340_u64 as i32;
_5 = core::ptr::addr_of_mut!((*_6));
(*_5) = 3634430023603180009_usize as u32;
RET = 60_isize;
RET = (-73_isize);
RET = (-3525350196484457715_i64) as isize;
_12.1.0 = [2623767936635833040_i64,1195460268253596970_i64,6542024245274464168_i64,(-8250330452352118669_i64),(-4769119743640963923_i64)];
_12.2.4.0 = !_1;
_12.0.0 = '\u{58d33}';
Goto(bb2)
}
bb8 = {
_12.1.2 = !_4;
_12.2.1 = _12.2.4.0 as u16;
_11 = core::ptr::addr_of_mut!(_12.0.1.2);
Call(_12.0.1.0 = core::intrinsics::transmute(_12.2.4.0), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
(*_11) = _15 as i128;
_12.1.1 = [_12.2.1,_12.2.1,_12.2.1,_12.2.1];
_6 = core::ptr::addr_of_mut!(_3);
_12.2.3 = (*_6) as f64;
_12.1.3 = _2;
(*_5) = _12.0.0 as u32;
_12.1.3 = _3 as i32;
_13 = [_12.1.3];
_5 = core::ptr::addr_of_mut!((*_5));
_3 = _1 | _12.0.1.0;
_12.0.1.2 = (-529630536376316337_i64) as i128;
_14 = _15 | _15;
RET = _12.0.1.2 as isize;
_16 = 0_usize as isize;
_17 = [_12.1.3];
_20 = [4_usize,2_usize,9153935138722547899_usize,5_usize,6319042013478146771_usize];
RET = !_15;
Goto(bb10)
}
bb10 = {
RET = _16 | _15;
_12.2.4.1.0 = _6;
_12.1.1 = [_12.2.1,_12.2.1,_12.2.1,_12.2.1];
_6 = _12.2.4.1.0;
_12.2.1 = 25423_u16;
_20 = [5_usize,279895882295737888_usize,15516115418446860779_usize,5648624259769515308_usize,8081763571822605893_usize];
_12.2.4.2 = _12.0.1.2;
_6 = _12.2.4.1.0;
_12.2.3 = _3 as f64;
_12.0.1.1.0 = core::ptr::addr_of_mut!((*_5));
RET = -_14;
_11 = core::ptr::addr_of_mut!(_12.0.1.2);
_21 = -_12.0.1.2;
_24 = [251363182850997268121837841944152408772_u128,108413245333076254811547070670817113533_u128,25036598350728985321154028555631142161_u128,137918808654704398643974786066689273142_u128];
_12.2.4.0 = _3;
(*_5) = _3 << _12.2.4.0;
match _12.2.1 {
25423 => bb12,
_ => bb11
}
}
bb11 = {
(*_5) = !_3;
(*_5) = !_3;
_5 = _6;
_5 = core::ptr::addr_of_mut!(_1);
_6 = core::ptr::addr_of_mut!((*_6));
RET = 86_isize >> _2;
(*_5) = !(*_6);
_2 = 3904972343480790340_u64 as i32;
_5 = core::ptr::addr_of_mut!((*_6));
(*_5) = 3634430023603180009_usize as u32;
RET = 60_isize;
RET = (-73_isize);
RET = (-3525350196484457715_i64) as isize;
_12.1.0 = [2623767936635833040_i64,1195460268253596970_i64,6542024245274464168_i64,(-8250330452352118669_i64),(-4769119743640963923_i64)];
_12.2.4.0 = !_1;
_12.0.0 = '\u{58d33}';
Goto(bb2)
}
bb12 = {
_2 = 1059132667777135252_usize as i32;
_12.1.3 = _2 ^ _2;
_12.1.1 = [_12.2.1,_12.2.1,_12.2.1,_12.2.1];
_22 = [_12.0.0,_12.0.0,_12.0.0];
(*_11) = _12.2.1 as i128;
_26 = [(-6131200069410082257_i64),7567424690435361643_i64,(-1420225774076683685_i64),164543186459978615_i64,7981440644860562293_i64];
_28 = _12.2.3 as isize;
_17 = [_12.1.3];
_12.2.4.0 = _1 ^ (*_6);
_12.2.1 = 36777_u16;
Call(_12.2.4.0 = core::intrinsics::transmute(_13), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_31 = !_12.2.1;
_3 = !(*_5);
_12.2.2 = _9 ^ _4;
_12.0.0 = '\u{e2b99}';
_7 = core::ptr::addr_of_mut!(_25);
_12.0.1.2 = !_12.2.4.2;
match _12.2.1 {
0 => bb6,
1 => bb14,
36777 => bb16,
_ => bb15
}
}
bb14 = {
(*_11) = _15 as i128;
_12.1.1 = [_12.2.1,_12.2.1,_12.2.1,_12.2.1];
_6 = core::ptr::addr_of_mut!(_3);
_12.2.3 = (*_6) as f64;
_12.1.3 = _2;
(*_5) = _12.0.0 as u32;
_12.1.3 = _3 as i32;
_13 = [_12.1.3];
_5 = core::ptr::addr_of_mut!((*_5));
_3 = _1 | _12.0.1.0;
_12.0.1.2 = (-529630536376316337_i64) as i128;
_14 = _15 | _15;
RET = _12.0.1.2 as isize;
_16 = 0_usize as isize;
_17 = [_12.1.3];
_20 = [4_usize,2_usize,9153935138722547899_usize,5_usize,6319042013478146771_usize];
RET = !_15;
Goto(bb10)
}
bb15 = {
_12.1.2 = !_4;
_12.2.1 = _12.2.4.0 as u16;
_11 = core::ptr::addr_of_mut!(_12.0.1.2);
Call(_12.0.1.0 = core::intrinsics::transmute(_12.2.4.0), ReturnTo(bb3), UnwindUnreachable())
}
bb16 = {
_18 = _28 << (*_6);
_1 = (*_5);
_12.1.3 = _2 << _28;
_23.1.0 = _12.2.4.1.0;
Goto(bb17)
}
bb17 = {
Call(_35 = dump_var(17_usize, 24_usize, Move(_24), 9_usize, Move(_9), 18_usize, Move(_18), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_35 = dump_var(17_usize, 22_usize, Move(_22), 26_usize, Move(_26), 31_usize, Move(_31), 3_usize, Move(_3)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_35 = dump_var(17_usize, 13_usize, Move(_13), 36_usize, _36, 36_usize, _36, 36_usize, _36), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{a666a}'), std::hint::black_box(41_isize), std::hint::black_box(100_i8), std::hint::black_box(23297_i16), std::hint::black_box((-1988711820_i32)), std::hint::black_box(6056108914620251034_i64), std::hint::black_box(27099163609871229672713543665681175963_i128), std::hint::black_box(0_usize), std::hint::black_box(74_u8), std::hint::black_box(49964_u16), std::hint::black_box(3194939693_u32), std::hint::black_box(13362939255857547341_u64), std::hint::black_box(66019837912786446458463730615672416967_u128));
                
            }
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: u32,
fld1: *const (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),
fld2: [i32; 1],

},
Variant1{
fld0: (*const (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),),
fld1: *const *const char,
fld2: isize,
fld3: *mut u32,
fld4: [i64; 5],

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
fld0: u32,
fld1: *const *const char,

},
Variant1{
fld0: *const (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),
fld1: char,
fld2: u32,
fld3: i8,
fld4: (*const (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),),
fld5: *mut i128,

},
Variant2{
fld0: i128,
fld1: u64,
fld2: isize,
fld3: i8,
fld4: u8,

},
Variant3{
fld0: [usize; 5],
fld1: *const char,
fld2: isize,
fld3: *const [usize; 5],
fld4: u64,
fld5: [char; 3],
fld6: f32,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: u64,
fld1: [u64; 2],
fld2: [u16; 4],
fld3: *mut i128,
fld4: [i32; 1],
fld5: f32,
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: (i64, (*mut u32, *mut i16)),
fld1: u128,
fld2: [u16; 4],
fld3: i8,
fld4: u32,
fld5: *const *const char,
fld6: (*mut u32, *mut i16),
fld7: i128,

},
Variant1{
fld0: [usize; 5],

},
Variant2{
fld0: (i64, (*mut u32, *mut i16)),
fld1: char,
fld2: (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),

},
Variant3{
fld0: [i64; 5],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
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
fld0: u64,
fld1: char,
fld2: [i64; 5],
fld3: [u128; 4],
fld4: [i32; 1],
fld5: (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),
fld6: u16,
fld7: [u16; 4],

},
Variant1{
fld0: bool,
fld1: u64,
fld2: f64,
fld3: Adt43,
fld4: Adt44,
fld5: i32,
fld6: i128,

},
Variant2{
fld0: [u16; 4],
fld1: char,
fld2: ([i64; 5], [u16; 4], bool, i32),
fld3: *mut i128,
fld4: Adt45,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: *mut i128,
fld1: char,
fld2: isize,
fld3: ((char, (u32, (*mut u32, *mut i16), i128)), ([i64; 5], [u16; 4], bool, i32), (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))),
fld4: u16,
fld5: u8,
fld6: (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),
}
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: (*mut u32, *mut i16),
fld1: [i32; 1],
fld2: [usize; 5],
fld3: (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),
fld4: usize,
fld5: [i64; 5],
fld6: (char, (u32, (*mut u32, *mut i16), i128)),
fld7: [u128; 4],

},
Variant1{
fld0: *mut f32,

},
Variant2{
fld0: i128,
fld1: u128,
fld2: Adt46,
fld3: (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),
fld4: [i32; 1],
fld5: *const *const char,
fld6: Adt45,

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: (*const (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),),
fld1: *const (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),
fld2: (i64, (*mut u32, *mut i16)),
fld3: i8,
fld4: i128,
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: u128,

},
Variant1{
fld0: Adt44,

},
Variant2{
fld0: usize,
fld1: *mut f32,
fld2: isize,
fld3: u32,
fld4: (*const (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),),

},
Variant3{
fld0: Adt44,
fld1: *const char,
fld2: (u32, (*mut u32, *mut i16), i128),
fld3: (char, (u32, (*mut u32, *mut i16), i128)),

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt52{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt52 {
fld0: u32,
fld1: ([i64; 5], [u16; 4], bool, i32),
fld2: isize,
fld3: i8,
fld4: u16,
fld5: *mut u32,
fld6: Adt44,
fld7: ((char, (u32, (*mut u32, *mut i16), i128)), ([i64; 5], [u16; 4], bool, i32), (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128))),
}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: Adt44,
fld1: (char, (u32, (*mut u32, *mut i16), i128)),
fld2: Adt51,
fld3: [i64; 5],
fld4: u16,
fld5: [i32; 1],
fld6: (*const (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),),
fld7: *const (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),

},
Variant1{
fld0: Adt52,

},
Variant2{
fld0: Adt47,
fld1: *const *const char,

},
Variant3{
fld0: [u128; 4],
fld1: char,
fld2: (i64, (*mut u32, *mut i16)),
fld3: Adt50,
fld4: [char; 3],
fld5: [u16; 4],
fld6: f32,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: [u64; 2],
fld1: *const [usize; 5],

},
Variant1{
fld0: Adt50,
fld1: ([i64; 5], [u16; 4], bool, i32),
fld2: [u128; 4],

},
Variant2{
fld0: i64,

}}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: *const (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),
fld1: *mut *mut i16,
fld2: f32,
fld3: [i64; 5],
fld4: u32,
fld5: *const *const char,
fld6: Adt50,

},
Variant1{
fld0: u32,
fld1: Adt51,
fld2: *const *const char,
fld3: usize,
fld4: Adt44,
fld5: *mut *mut i16,
fld6: *const (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),
fld7: i128,

},
Variant2{
fld0: bool,
fld1: isize,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: [usize; 5],
fld1: Adt46,
fld2: (char, (u32, (*mut u32, *mut i16), i128)),
fld3: u64,
fld4: *const (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),
fld5: [u16; 4],
fld6: (u32, (*mut u32, *mut i16), i128),
fld7: Adt44,

},
Variant1{
fld0: i128,
fld1: (char, (u32, (*mut u32, *mut i16), i128)),

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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: Adt54,
fld1: Adt56,
fld2: u128,
fld3: [u64; 2],
fld4: Adt46,
fld5: *mut i16,
fld6: i64,

},
Variant1{
fld0: *const *const char,
fld1: char,
fld2: i16,
fld3: f64,

},
Variant2{
fld0: bool,
fld1: [u16; 4],
fld2: Adt51,
fld3: i16,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: *const [usize; 5],
fld1: (char, (u32, (*mut u32, *mut i16), i128)),
fld2: *const (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),
fld3: i128,
fld4: Adt54,
fld5: *const *const char,

},
Variant1{
fld0: [u16; 4],
fld1: ([i64; 5], [u16; 4], bool, i32),
fld2: i64,
fld3: Adt54,
fld4: [u64; 2],

},
Variant2{
fld0: (*const (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),),
fld1: Adt55,

},
Variant3{
fld0: Adt50,

}}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf("Adt59::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
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
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: *mut i128,

},
Variant1{
fld0: (i64, (*mut u32, *mut i16)),
fld1: (*mut u32, *mut i16),
fld2: Adt49,

},
Variant2{
fld0: Adt53,
fld1: Adt48,
fld2: isize,
fld3: (f64, u16, bool, f64, (u32, (*mut u32, *mut i16), i128)),
fld4: [i32; 1],
fld5: Adt49,
fld6: (i64, (*mut u32, *mut i16)),

}}

