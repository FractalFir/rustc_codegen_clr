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
pub fn fn0(mut _1: u16,mut _2: char,mut _3: i8) -> i16 {
mir! {
type RET = i16;
let _4: i32;
let _5: f32;
let _6: u8;
let _7: (u64, (i8, u128));
let _8: [u8; 3];
let _9: isize;
let _10: Adt53;
let _11: Adt47;
let _12: Adt49;
let _13: i64;
let _14: bool;
let _15: u16;
let _16: [usize; 1];
let _17: ([u64; 7], char);
let _18: [u64; 7];
let _19: ();
let _20: ();
{
_2 = '\u{d7fe6}';
_3 = 28_i8 - 23_i8;
_1 = !48350_u16;
RET = (-15662_i16) & 12147_i16;
RET = -16772_i16;
_2 = '\u{c3aca}';
_3 = 87_i8 - 49_i8;
_2 = '\u{22781}';
RET = 132_u8 as i16;
_2 = '\u{21b54}';
RET = !22683_i16;
_2 = '\u{f71}';
_4 = true as i32;
RET = 17674_i16;
_1 = 42812_u16;
_4 = !1562483999_i32;
RET = (-24483_i16) >> _3;
_3 = true as i8;
_1 = 16363_u16 | 18263_u16;
_4 = 1967443050_i32;
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
1967443050 => bb6,
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
_6 = !246_u8;
_7.1 = (_3, 283032479659703201527029659198100896474_u128);
_5 = 9223372036854775807_isize as f32;
_2 = '\u{89a9b}';
_4 = (-9223372036854775808_isize) as i32;
_7.1 = (_3, 164264988265516513387701764247741151016_u128);
_4 = -(-576402429_i32);
_7.1 = (_3, 40967549921450549923687536320098756170_u128);
_8 = [_6,_6,_6];
_3 = _7.1.0;
RET = 29245_i16 >> _4;
_7.0 = 16090031756419661344_u64 + 12377098712313106006_u64;
RET = 9223372036854775807_isize as i16;
_8 = [_6,_6,_6];
match _7.1.1 {
40967549921450549923687536320098756170 => bb8,
_ => bb7
}
}
bb7 = {
Return()
}
bb8 = {
_7.0 = RET as u64;
Call(_7.1.1 = fn1(_2, RET, _6, _5, _7.1.0, _2, _7.1.0, _1, _7.1.0, _5, _7.1.0, _1, _1, _2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
RET = -28499_i16;
_8 = [_6,_6,_6];
_1 = 23379_u16 * 59281_u16;
_2 = '\u{24686}';
_7.0 = !6861923382199564309_u64;
_9 = 106_isize ^ (-9223372036854775808_isize);
RET = 3465_i16 * (-28224_i16);
_8 = [_6,_6,_6];
_7.1.1 = !203341113667686144463703781130978575514_u128;
_1 = 48798_u16 & 5692_u16;
_6 = !155_u8;
_2 = '\u{4d241}';
_5 = _7.1.0 as f32;
_4 = 4418495828463025176_i64 as i32;
_7.1.1 = (-127954302669335384273056367879024724883_i128) as u128;
RET = (-29243_i16) + 24835_i16;
_7.1 = (_3, 165220963471715269835221610821106533599_u128);
_4 = !(-1044857049_i32);
_8 = [_6,_6,_6];
_5 = RET as f32;
Goto(bb10)
}
bb10 = {
_7.0 = _7.1.1 as u64;
_1 = 34696_u16;
RET = 21558_i16 ^ 6671_i16;
_2 = '\u{efd33}';
_3 = _6 as i8;
_3 = -_7.1.0;
_14 = true;
_2 = '\u{3fcfc}';
_3 = _7.1.0 - _7.1.0;
_13 = 5890649934025440690_i64;
Goto(bb11)
}
bb11 = {
_14 = _4 != _4;
match _1 {
0 => bb7,
1 => bb6,
2 => bb10,
3 => bb5,
4 => bb12,
5 => bb13,
34696 => bb15,
_ => bb14
}
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_7.0 = RET as u64;
Call(_7.1.1 = fn1(_2, RET, _6, _5, _7.1.0, _2, _7.1.0, _1, _7.1.0, _5, _7.1.0, _1, _1, _2), ReturnTo(bb9), UnwindUnreachable())
}
bb15 = {
_8 = [_6,_6,_6];
_5 = 125281194766050625045452516497241116286_i128 as f32;
_14 = true ^ false;
RET = -20536_i16;
_7.0 = _9 as u64;
_2 = '\u{7ebd7}';
_7.1 = (_3, 336963321993917411204164335319661370952_u128);
_3 = !_7.1.0;
_6 = _13 as u8;
_7.1.1 = 2982536113_u32 as u128;
_13 = 4299737562544875018_i64 & (-1672154664823639429_i64);
RET = _5 as i16;
_7.0 = _7.1.0 as u64;
RET = !23358_i16;
_15 = _3 as u16;
RET = _14 as i16;
_7.1.0 = _3;
_7.1 = (_3, 136925999268452060034839890595782191391_u128);
_7.1.1 = _2 as u128;
_14 = !true;
_6 = !139_u8;
_17.0 = [_7.0,_7.0,_7.0,_7.0,_7.0,_7.0,_7.0];
Goto(bb16)
}
bb16 = {
Call(_19 = dump_var(0_usize, 15_usize, Move(_15), 2_usize, Move(_2), 14_usize, Move(_14), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_19 = dump_var(0_usize, 1_usize, Move(_1), 20_usize, _20, 20_usize, _20, 20_usize, _20), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: char,mut _2: i16,mut _3: u8,mut _4: f32,mut _5: i8,mut _6: char,mut _7: i8,mut _8: u16,mut _9: i8,mut _10: f32,mut _11: i8,mut _12: u16,mut _13: u16,mut _14: char) -> u128 {
mir! {
type RET = u128;
let _15: [u8; 3];
let _16: isize;
let _17: isize;
let _18: [u32; 5];
let _19: bool;
let _20: [u32; 5];
let _21: bool;
let _22: [i64; 2];
let _23: char;
let _24: [u64; 7];
let _25: u64;
let _26: bool;
let _27: [u64; 7];
let _28: i128;
let _29: (u64, u64);
let _30: u128;
let _31: f64;
let _32: f32;
let _33: bool;
let _34: (usize,);
let _35: isize;
let _36: Adt47;
let _37: [u128; 8];
let _38: *mut &'static [u8; 3];
let _39: *mut &'static [u8; 3];
let _40: i32;
let _41: ();
let _42: ();
{
_11 = _5;
_10 = _4;
RET = (-7295274519648925475_i64) as u128;
_10 = 3722801856049599_u64 as f32;
_8 = _2 as u16;
RET = 32361092016438552463234991569266147891_u128 + 134852343089874508610354384975288282951_u128;
_5 = _11 & _9;
_2 = (-31259_i16) & 18438_i16;
_10 = 11001670239641539730_usize as f32;
_10 = _4 + _4;
_14 = _1;
_8 = _13 | _12;
_3 = !92_u8;
_3 = 81_u8;
_14 = _6;
_12 = _13 & _8;
_6 = _14;
_10 = -_4;
_15 = [_3,_3,_3];
match _3 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
81 => bb6,
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
_2 = _1 as i16;
_14 = _6;
_17 = -(-9223372036854775808_isize);
_12 = _8 - _8;
_1 = _6;
_20 = [2908766747_u32,2220337992_u32,1875996081_u32,3213151950_u32,3787559427_u32];
_2 = (-31433_i16) | (-7260_i16);
_17 = -(-9223372036854775808_isize);
_18 = [591127596_u32,2142901655_u32,1677539224_u32,683144585_u32,2006678786_u32];
_1 = _6;
Call(_17 = fn2(_12, _13), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_2 = (-21390_i16);
_20 = _18;
_3 = 136_u8 | 25_u8;
_11 = !_5;
_4 = _10;
_18 = [1913185793_u32,2591942678_u32,1461467054_u32,2644730548_u32,224758448_u32];
_1 = _14;
Goto(bb8)
}
bb8 = {
_21 = true;
_2 = !4557_i16;
_4 = -_10;
_11 = !_5;
_16 = -_17;
_3 = !233_u8;
_5 = _9 ^ _7;
_22 = [(-1245699264624046274_i64),(-2533035683066824807_i64)];
_13 = _12;
_15 = [_3,_3,_3];
_15 = [_3,_3,_3];
_21 = !false;
_21 = !true;
_7 = !_11;
_5 = _7 << _7;
_7 = _11;
_11 = _5 * _5;
_19 = !_21;
_25 = 14896381064294432840_u64 & 7531809579533417938_u64;
Call(_25 = core::intrinsics::bswap(16493061387960748228_u64), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_2 = -26179_i16;
Goto(bb10)
}
bb10 = {
_20 = _18;
_25 = 13483056481311941024_u64;
_13 = _12;
_7 = !_11;
_22 = [(-5712061378342015612_i64),(-1656829480677253876_i64)];
_25 = 7404448571111174763_u64;
_6 = _1;
_21 = !_19;
_23 = _1;
_10 = _13 as f32;
_29.0 = !_25;
_11 = _25 as i8;
_13 = _25 as u16;
_31 = (-174508073_i32) as f64;
_6 = _14;
_20 = [1501071841_u32,349063725_u32,2650316126_u32,1447284696_u32,3910599339_u32];
_26 = _19;
RET = 38256581272033910036752289194871553910_u128;
_4 = -_10;
_9 = _6 as i8;
_9 = -_7;
Goto(bb11)
}
bb11 = {
_20 = [3341909309_u32,3437127188_u32,250623465_u32,3830163883_u32,1523791994_u32];
_8 = _12 | _12;
_28 = _2 as i128;
_16 = _17 >> _7;
_7 = -_9;
_30 = _17 as u128;
_17 = _16 ^ _16;
_11 = -_7;
_17 = !_16;
Goto(bb12)
}
bb12 = {
_13 = _8 + _12;
_29.1 = _29.0 - _29.0;
_33 = !_19;
_10 = _4 + _4;
_34 = (2455929422405980025_usize,);
_19 = _11 < _7;
_17 = _2 as isize;
_21 = !_19;
_30 = RET;
_24 = [_29.1,_29.0,_29.1,_29.0,_25,_29.1,_29.0];
match _25 {
7404448571111174763 => bb13,
_ => bb9
}
}
bb13 = {
_29.1 = !_29.0;
_26 = _21;
_29 = (_25, _25);
_34 = (8330878620942806007_usize,);
_20 = [685581239_u32,2814175215_u32,3969389573_u32,2266532679_u32,1657662927_u32];
_5 = _25 as i8;
_10 = _4;
Goto(bb14)
}
bb14 = {
_18 = [1106994238_u32,2216090576_u32,3590209839_u32,839274733_u32,80120264_u32];
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(1_usize, 21_usize, Move(_21), 29_usize, Move(_29), 25_usize, Move(_25), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(1_usize, 15_usize, Move(_15), 6_usize, Move(_6), 30_usize, Move(_30), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(1_usize, 13_usize, Move(_13), 5_usize, Move(_5), 2_usize, Move(_2), 20_usize, Move(_20)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_41 = dump_var(1_usize, 3_usize, Move(_3), 28_usize, Move(_28), 42_usize, _42, 42_usize, _42), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: u16,mut _2: u16) -> isize {
mir! {
type RET = isize;
let _3: [u8; 3];
let _4: u16;
let _5: f32;
let _6: [u32; 1];
let _7: f32;
let _8: ([u64; 7], char);
let _9: (u64, u64);
let _10: Adt50;
let _11: (i8, u128);
let _12: Adt45;
let _13: char;
let _14: (u64, (i8, u128));
let _15: *const [u32; 1];
let _16: (i64, i128, *mut i8);
let _17: u16;
let _18: [u64; 7];
let _19: Adt55;
let _20: isize;
let _21: [i32; 8];
let _22: f32;
let _23: f32;
let _24: isize;
let _25: [u64; 7];
let _26: u8;
let _27: [u8; 3];
let _28: [i32; 8];
let _29: u64;
let _30: i64;
let _31: *mut &'static [u8; 3];
let _32: (isize,);
let _33: *const [u32; 1];
let _34: ();
let _35: ();
{
_1 = _2 * _2;
RET = !9223372036854775807_isize;
_2 = _1;
RET = 30_isize - 9223372036854775807_isize;
RET = -34_isize;
_3 = [196_u8,9_u8,113_u8];
_1 = false as u16;
_1 = 111_i8 as u16;
RET = 185738408083473304202165250738039184098_u128 as isize;
RET = (-9223372036854775808_isize) * (-9223372036854775808_isize);
_4 = !_2;
_1 = !_2;
_3 = [139_u8,175_u8,165_u8];
_3 = [0_u8,18_u8,255_u8];
RET = 61_isize;
RET = (-9223372036854775808_isize) << _2;
_3 = [146_u8,94_u8,180_u8];
_4 = _1;
RET = (-9223372036854775808_isize);
_2 = _1 | _4;
_2 = 66997337448271302992518724348462700641_u128 as u16;
match RET {
0 => bb1,
1 => bb2,
340282366920938463454151235394913435648 => bb4,
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
_5 = (-77_i8) as f32;
_2 = (-1385551498699897137427005854906130675_i128) as u16;
_2 = _4;
_6 = [1077074232_u32];
_6 = [383635306_u32];
_9 = (813784745435492621_u64, 8392219579508794246_u64);
_6 = [2942981993_u32];
_5 = 85723544719632921441503710079262466125_i128 as f32;
RET = _9.0 as isize;
_7 = (-49882550960968712874157730540729447314_i128) as f32;
RET = !(-9223372036854775808_isize);
Goto(bb5)
}
bb5 = {
_6 = [3626215703_u32];
_9.1 = _9.0;
_9.1 = _9.0;
_8.1 = '\u{a14de}';
_9 = (11267530249985320964_u64, 7700838105766344261_u64);
_6 = [3945580852_u32];
_8.0 = [_9.1,_9.1,_9.1,_9.0,_9.1,_9.0,_9.1];
_1 = !_4;
_8.0 = [_9.1,_9.0,_9.0,_9.0,_9.0,_9.0,_9.0];
_2 = !_4;
RET = (-5255_i16) as isize;
_7 = -_5;
Call(_5 = core::intrinsics::transmute(_8.1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_4 = 11127304212582264303_usize as u16;
_11 = (54_i8, 173751754589835540613637431396498256501_u128);
_11.0 = 110_i8;
Call(_7 = core::intrinsics::transmute(_8.1), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_11.0 = 22_i8;
_14.1 = (_11.0, _11.1);
_7 = -_5;
_5 = (-7259534005665260931_i64) as f32;
RET = (-9223372036854775808_isize);
_3 = [54_u8,170_u8,68_u8];
_13 = _8.1;
_11.0 = !_14.1.0;
RET = _8.1 as isize;
_14.1 = _11;
match _9.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
7700838105766344261 => bb8,
_ => bb6
}
}
bb8 = {
_11.1 = !_14.1.1;
_11.0 = !_14.1.0;
_9 = (12695444730740830718_u64, 13833197035867712335_u64);
_6 = [709899981_u32];
_9 = (5360077394009618789_u64, 16307212196631358237_u64);
_14 = (_9.1, _11);
_2 = !_1;
_18 = [_14.0,_14.0,_9.1,_9.1,_9.0,_9.1,_14.0];
Call(_3 = fn3(RET, _5), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
RET = (-9223372036854775808_isize);
RET = 9223372036854775807_isize | 9223372036854775807_isize;
_7 = -_5;
_15 = core::ptr::addr_of!(_6);
_16.0 = (-6390110755355185822_i64) - (-6861661796463630649_i64);
_9.1 = !_14.0;
_17 = _1;
Call(_14.1.1 = core::intrinsics::transmute(_11.1), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_14.1.1 = _11.1;
_3 = [110_u8,217_u8,10_u8];
_14.1.1 = _11.1;
_4 = _7 as u16;
_16.0 = _2 as i64;
_13 = _8.1;
_15 = core::ptr::addr_of!(_6);
(*_15) = [1920426443_u32];
_15 = core::ptr::addr_of!(_6);
_11.0 = (-161945602797759815408795042503731570032_i128) as i8;
_14.1.0 = _11.0 - _11.0;
_16.2 = core::ptr::addr_of_mut!(_14.1.0);
_16.0 = (-166805223914111644_i64);
Goto(bb11)
}
bb11 = {
_20 = (-6366_i16) as isize;
_14.1 = (_11.0, _11.1);
_8.1 = _13;
_11.1 = _14.1.1;
(*_15) = [3412582175_u32];
_18 = [_9.1,_14.0,_9.1,_9.1,_14.0,_14.0,_14.0];
RET = 1968002763_i32 as isize;
_19 = Adt55::Variant1 { fld0: _9,fld1: _15 };
place!(Field::<*const [u32; 1]>(Variant(_19, 1), 1)) = core::ptr::addr_of!((*_15));
RET = _20 * _20;
_16.2 = core::ptr::addr_of_mut!(_14.1.0);
_7 = -_5;
_24 = RET;
SetDiscriminant(_19, 0);
(*_15) = [3885100139_u32];
_23 = _5 * _5;
place!(Field::<(u64, (i8, u128))>(Variant(_19, 0), 2)).1.1 = _14.1.1;
_14.1.1 = true as u128;
_14.1 = (_11.0, Field::<(u64, (i8, u128))>(Variant(_19, 0), 2).1.1);
_9.0 = _9.1 >> Field::<(u64, (i8, u128))>(Variant(_19, 0), 2).1.1;
_11.1 = !_14.1.1;
place!(Field::<(u64, (i8, u128))>(Variant(_19, 0), 2)).0 = !_14.0;
Call(place!(Field::<(i64, i128, *mut i8)>(Variant(_19, 0), 3)).1 = core::intrinsics::transmute(_14.1.1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
place!(Field::<(isize,)>(Variant(_19, 0), 1)).0 = -_24;
place!(Field::<(isize,)>(Variant(_19, 0), 1)) = (_24,);
RET = !Field::<(isize,)>(Variant(_19, 0), 1).0;
_16.1 = Field::<(i64, i128, *mut i8)>(Variant(_19, 0), 3).1 + Field::<(i64, i128, *mut i8)>(Variant(_19, 0), 3).1;
_25 = _8.0;
RET = _20;
place!(Field::<i16>(Variant(_19, 0), 4)) = (-31286_i16);
_25 = [_14.0,_14.0,Field::<(u64, (i8, u128))>(Variant(_19, 0), 2).0,_14.0,_14.0,_14.0,_9.1];
_14.0 = Field::<(i64, i128, *mut i8)>(Variant(_19, 0), 3).1 as u64;
_27 = [233_u8,149_u8,21_u8];
place!(Field::<(u64, (i8, u128))>(Variant(_19, 0), 2)).1.0 = _11.0 << Field::<(u64, (i8, u128))>(Variant(_19, 0), 2).0;
_8.1 = _13;
_9 = (Field::<(u64, (i8, u128))>(Variant(_19, 0), 2).0, Field::<(u64, (i8, u128))>(Variant(_19, 0), 2).0);
_15 = core::ptr::addr_of!((*_15));
_9 = (_14.0, _14.0);
Goto(bb13)
}
bb13 = {
_8.0 = [_14.0,Field::<(u64, (i8, u128))>(Variant(_19, 0), 2).0,Field::<(u64, (i8, u128))>(Variant(_19, 0), 2).0,_9.1,_9.1,_9.1,Field::<(u64, (i8, u128))>(Variant(_19, 0), 2).0];
place!(Field::<(u64, (i8, u128))>(Variant(_19, 0), 2)).1 = _14.1;
_16.1 = 121_u8 as i128;
_14.1 = (_11.0, _11.1);
RET = _5 as isize;
place!(Field::<i16>(Variant(_19, 0), 4)) = 17379_i16 * 15820_i16;
_22 = 0_u8 as f32;
place!(Field::<(i64, i128, *mut i8)>(Variant(_19, 0), 3)).0 = -_16.0;
Call(_2 = fn15(_14.1.1, Field::<(isize,)>(Variant(_19, 0), 1).0, Field::<(u64, (i8, u128))>(Variant(_19, 0), 2).1.1, _6, _4, _9.1, _14.1.1, Field::<(u64, (i8, u128))>(Variant(_19, 0), 2).1.1, _17, Field::<(u64, (i8, u128))>(Variant(_19, 0), 2).1.1, _14.1.1), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
place!(Field::<(u64, (i8, u128))>(Variant(_19, 0), 2)).0 = _11.0 as u64;
place!(Field::<[u128; 8]>(Variant(_19, 0), 5)) = [_14.1.1,Field::<(u64, (i8, u128))>(Variant(_19, 0), 2).1.1,_14.1.1,_11.1,_11.1,Field::<(u64, (i8, u128))>(Variant(_19, 0), 2).1.1,_11.1,Field::<(u64, (i8, u128))>(Variant(_19, 0), 2).1.1];
_11 = Field::<(u64, (i8, u128))>(Variant(_19, 0), 2).1;
_22 = _23;
_22 = -_23;
place!(Field::<(u64, (i8, u128))>(Variant(_19, 0), 2)).0 = Field::<i16>(Variant(_19, 0), 4) as u64;
_8 = (_18, _13);
_5 = _22;
_14.1.1 = !Field::<(u64, (i8, u128))>(Variant(_19, 0), 2).1.1;
_11.1 = _14.1.1;
_17 = _1 << _9.1;
RET = Field::<(isize,)>(Variant(_19, 0), 1).0;
RET = -_24;
Goto(bb15)
}
bb15 = {
Call(_34 = dump_var(2_usize, 27_usize, Move(_27), 9_usize, Move(_9), 11_usize, Move(_11), 24_usize, Move(_24)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_34 = dump_var(2_usize, 18_usize, Move(_18), 25_usize, Move(_25), 20_usize, Move(_20), 8_usize, Move(_8)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: isize,mut _2: f32) -> [u8; 3] {
mir! {
type RET = [u8; 3];
let _3: Adt47;
let _4: u32;
let _5: Adt44;
let _6: (isize,);
let _7: usize;
let _8: i64;
let _9: *mut &'static [u8; 3];
let _10: [u64; 7];
let _11: isize;
let _12: i64;
let _13: [i32; 8];
let _14: f64;
let _15: [u32; 1];
let _16: isize;
let _17: *const [u32; 1];
let _18: u32;
let _19: char;
let _20: isize;
let _21: *mut &'static [u8; 3];
let _22: ((i64, i128, *mut i8), (u64, (i8, u128)), (i64, i128, *mut i8));
let _23: isize;
let _24: [u32; 6];
let _25: (isize,);
let _26: i128;
let _27: isize;
let _28: [i64; 2];
let _29: (i8, u128);
let _30: i32;
let _31: u128;
let _32: bool;
let _33: Adt47;
let _34: isize;
let _35: *mut &'static [u8; 3];
let _36: u128;
let _37: bool;
let _38: char;
let _39: u64;
let _40: bool;
let _41: [usize; 1];
let _42: [u8; 3];
let _43: Adt46;
let _44: [u32; 5];
let _45: [u32; 6];
let _46: ();
let _47: ();
{
_1 = (-9223372036854775808_isize);
RET = [147_u8,209_u8,128_u8];
RET = [13_u8,68_u8,190_u8];
RET = [230_u8,99_u8,128_u8];
RET = [170_u8,97_u8,223_u8];
RET = [146_u8,116_u8,206_u8];
_1 = 15433874783560862267_u64 as isize;
RET = [20_u8,166_u8,251_u8];
Call(_3 = fn4(_1, RET, _1, _1, RET, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [36_u8,94_u8,42_u8];
_2 = 122_u8 as f32;
place!(Field::<bool>(Variant(_3, 0), 0)) = true;
_7 = !2_usize;
_4 = 1742119836_u32;
_6.0 = _1;
_2 = 1694059263785518192711581137279496782_u128 as f32;
_8 = 165304766500503808156690616796109083918_i128 as i64;
place!(Field::<bool>(Variant(_3, 0), 0)) = !false;
place!(Field::<bool>(Variant(_3, 0), 0)) = !false;
RET = [117_u8,210_u8,225_u8];
_2 = 3779_i16 as f32;
_7 = 160_u8 as usize;
_1 = _6.0;
_4 = 1869592552_u32 & 3565246079_u32;
SetDiscriminant(_3, 0);
Goto(bb2)
}
bb2 = {
_10 = [1521753302692551801_u64,15961234511464311381_u64,14825836652695732427_u64,10124482290193420577_u64,9668828134022230036_u64,5353661459148830791_u64,12346287747714848746_u64];
_4 = !1579994256_u32;
_12 = -_8;
_13 = [(-55657400_i32),1619157706_i32,670175525_i32,(-1828398619_i32),369170214_i32,(-1234665061_i32),(-250268717_i32),701031223_i32];
_11 = _1 >> _8;
Call(_5 = fn12(_10, _13, RET, _10, _6.0, _10, _13, _11, _6, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_4 = _8 as u32;
_6.0 = 28883_u16 as isize;
_8 = _12 >> _6.0;
place!(Field::<u32>(Variant(_5, 2), 0)) = !_4;
_15 = [_4];
_14 = _2 as f64;
_7 = Field::<i16>(Variant(_5, 2), 1) as usize;
Goto(bb4)
}
bb4 = {
RET = [148_u8,3_u8,130_u8];
place!(Field::<bool>(Variant(_3, 0), 0)) = false;
_4 = _11 as u32;
_8 = -_12;
place!(Field::<i16>(Variant(_5, 2), 1)) = 9627597616618437741_u64 as i16;
_11 = _1;
_12 = _8;
_7 = 13988959806500509988_usize;
_12 = _8;
_7 = 5_usize;
_4 = !Field::<u32>(Variant(_5, 2), 0);
_11 = _6.0;
_4 = Field::<u32>(Variant(_5, 2), 0);
_19 = '\u{a04a7}';
_19 = '\u{5c6c5}';
_12 = _8;
SetDiscriminant(_5, 0);
_18 = !_4;
RET = [56_u8,236_u8,136_u8];
Call(_11 = fn14(_13, _7, _10, _10, _13, Field::<bool>(Variant(_3, 0), 0), _13, _10[_7], _13[_7], _10[_7], _13[_7], _6, _10, _8, _19, _13[_7]), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_8 = _12;
_13 = [(-2097019912_i32),945527014_i32,709435706_i32,2035844743_i32,909647413_i32,(-1370673497_i32),(-1323722762_i32),(-1261762978_i32)];
Goto(bb6)
}
bb6 = {
_20 = !_11;
_16 = -_20;
_14 = _2 as f64;
_22.0.1 = _18 as i128;
place!(Field::<u64>(Variant(_5, 0), 6)) = _7 as u64;
_2 = _11 as f32;
place!(Field::<u128>(Variant(_5, 0), 5)) = 68947151471766284218169521268818983916_u128;
_22.1.1 = ((-97_i8), Field::<u128>(Variant(_5, 0), 5));
_22.1.1.1 = !Field::<u128>(Variant(_5, 0), 5);
RET = [166_u8,254_u8,181_u8];
_22.0.0 = _8;
place!(Field::<bool>(Variant(_5, 0), 0)) = !Field::<bool>(Variant(_3, 0), 0);
_8 = !_12;
_6 = (_11,);
_20 = _11 & _16;
place!(Field::<[u32; 1]>(Variant(_5, 0), 3)) = [_18];
_16 = -_20;
place!(Field::<isize>(Variant(_5, 0), 2)) = -_11;
_22.0.2 = core::ptr::addr_of_mut!(_22.1.1.0);
_6 = (_11,);
_24 = [_18,_18,_18,_4,_18,_18];
place!(Field::<*mut i8>(Variant(_3, 0), 1)) = core::ptr::addr_of_mut!(_22.1.1.0);
_24 = [_4,_18,_4,_18,_18,_18];
_1 = _6.0;
_22.2.2 = _22.0.2;
RET = [112_u8,154_u8,14_u8];
_19 = '\u{c0064}';
place!(Field::<*const [u32; 1]>(Variant(_5, 0), 1)) = core::ptr::addr_of!(place!(Field::<[u32; 1]>(Variant(_5, 0), 3)));
_22.2.2 = core::ptr::addr_of_mut!(_22.1.1.0);
Goto(bb7)
}
bb7 = {
place!(Field::<[u32; 1]>(Variant(_5, 0), 3)) = _15;
Goto(bb8)
}
bb8 = {
_22.1.0 = Field::<u64>(Variant(_5, 0), 6) + Field::<u64>(Variant(_5, 0), 6);
match Field::<u128>(Variant(_5, 0), 5) {
0 => bb1,
1 => bb7,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
68947151471766284218169521268818983916 => bb10,
_ => bb9
}
}
bb9 = {
_4 = _8 as u32;
_6.0 = 28883_u16 as isize;
_8 = _12 >> _6.0;
place!(Field::<u32>(Variant(_5, 2), 0)) = !_4;
_15 = [_4];
_14 = _2 as f64;
_7 = Field::<i16>(Variant(_5, 2), 1) as usize;
Goto(bb4)
}
bb10 = {
_22.2.0 = _12 * _8;
place!(Field::<u16>(Variant(_5, 0), 4)) = (-640893732_i32) as u16;
_19 = '\u{3b728}';
_22.1.0 = !Field::<u64>(Variant(_5, 0), 6);
_6 = (_20,);
_18 = !_4;
_18 = _4 * _4;
_22.0.2 = core::ptr::addr_of_mut!(_22.1.1.0);
place!(Field::<bool>(Variant(_5, 0), 0)) = Field::<bool>(Variant(_3, 0), 0);
_25.0 = _6.0 + _1;
_25.0 = _11;
Goto(bb11)
}
bb11 = {
_22.2.2 = Field::<*mut i8>(Variant(_3, 0), 1);
SetDiscriminant(_3, 0);
_23 = _6.0 | _20;
_22.1.0 = _22.1.1.0 as u64;
_29.1 = Field::<u128>(Variant(_5, 0), 5);
_12 = _22.0.1 as i64;
_17 = core::ptr::addr_of!(place!(Field::<[u32; 1]>(Variant(_5, 0), 3)));
_22.2 = (_12, _22.0.1, _22.0.2);
_18 = Field::<u16>(Variant(_5, 0), 4) as u32;
_22.1.0 = Field::<u64>(Variant(_5, 0), 6) ^ Field::<u64>(Variant(_5, 0), 6);
_15 = [_18];
_28 = [_22.0.0,_8];
_20 = !_23;
_22.2.1 = _22.0.1 + _22.0.1;
SetDiscriminant(_5, 3);
_30 = 1500402417_i32;
_22.0.1 = -_22.2.1;
place!(Field::<*mut i8>(Variant(_3, 0), 1)) = _22.0.2;
_22.2.1 = !_22.0.1;
RET = [114_u8,87_u8,111_u8];
_8 = _22.2.0 + _22.2.0;
match _22.1.1.0 {
0 => bb8,
1 => bb2,
2 => bb3,
3 => bb6,
340282366920938463463374607431768211359 => bb13,
_ => bb12
}
}
bb12 = {
_4 = _8 as u32;
_6.0 = 28883_u16 as isize;
_8 = _12 >> _6.0;
place!(Field::<u32>(Variant(_5, 2), 0)) = !_4;
_15 = [_4];
_14 = _2 as f64;
_7 = Field::<i16>(Variant(_5, 2), 1) as usize;
Goto(bb4)
}
bb13 = {
_16 = !_11;
_31 = _7 as u128;
_25 = (_23,);
place!(Field::<u8>(Variant(_5, 3), 1)) = 65172_u16 as u8;
place!(Field::<(u64, (i8, u128))>(Variant(_5, 3), 3)).1.0 = _2 as i8;
place!(Field::<(u64, (i8, u128))>(Variant(_5, 3), 3)) = (_22.1.0, _22.1.1);
_24 = [_18,_18,_18,_18,_18,_18];
_2 = _31 as f32;
_30 = 358312782_i32 - (-416467738_i32);
_20 = !_1;
place!(Field::<(isize,)>(Variant(_5, 3), 4)) = (_11,);
place!(Field::<(i64, i128, *mut i8)>(Variant(_5, 3), 2)).1 = !_22.0.1;
_10 = [_22.1.0,Field::<(u64, (i8, u128))>(Variant(_5, 3), 3).0,_22.1.0,Field::<(u64, (i8, u128))>(Variant(_5, 3), 3).0,Field::<(u64, (i8, u128))>(Variant(_5, 3), 3).0,Field::<(u64, (i8, u128))>(Variant(_5, 3), 3).0,Field::<(u64, (i8, u128))>(Variant(_5, 3), 3).0];
_36 = !_29.1;
_22.2 = (_22.0.0, _22.0.1, Field::<*mut i8>(Variant(_3, 0), 1));
match _22.1.1.0 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
340282366920938463463374607431768211359 => bb19,
_ => bb18
}
}
bb14 = {
_8 = _12;
_13 = [(-2097019912_i32),945527014_i32,709435706_i32,2035844743_i32,909647413_i32,(-1370673497_i32),(-1323722762_i32),(-1261762978_i32)];
Goto(bb6)
}
bb15 = {
RET = [36_u8,94_u8,42_u8];
_2 = 122_u8 as f32;
place!(Field::<bool>(Variant(_3, 0), 0)) = true;
_7 = !2_usize;
_4 = 1742119836_u32;
_6.0 = _1;
_2 = 1694059263785518192711581137279496782_u128 as f32;
_8 = 165304766500503808156690616796109083918_i128 as i64;
place!(Field::<bool>(Variant(_3, 0), 0)) = !false;
place!(Field::<bool>(Variant(_3, 0), 0)) = !false;
RET = [117_u8,210_u8,225_u8];
_2 = 3779_i16 as f32;
_7 = 160_u8 as usize;
_1 = _6.0;
_4 = 1869592552_u32 & 3565246079_u32;
SetDiscriminant(_3, 0);
Goto(bb2)
}
bb16 = {
place!(Field::<[u32; 1]>(Variant(_5, 0), 3)) = _15;
Goto(bb8)
}
bb17 = {
_4 = _8 as u32;
_6.0 = 28883_u16 as isize;
_8 = _12 >> _6.0;
place!(Field::<u32>(Variant(_5, 2), 0)) = !_4;
_15 = [_4];
_14 = _2 as f64;
_7 = Field::<i16>(Variant(_5, 2), 1) as usize;
Goto(bb4)
}
bb18 = {
_4 = _8 as u32;
_6.0 = 28883_u16 as isize;
_8 = _12 >> _6.0;
place!(Field::<u32>(Variant(_5, 2), 0)) = !_4;
_15 = [_4];
_14 = _2 as f64;
_7 = Field::<i16>(Variant(_5, 2), 1) as usize;
Goto(bb4)
}
bb19 = {
RET = [Field::<u8>(Variant(_5, 3), 1),Field::<u8>(Variant(_5, 3), 1),Field::<u8>(Variant(_5, 3), 1)];
_22.0.1 = _22.2.1;
_24 = [_18,_18,_18,_18,_4,_4];
_29 = (_22.1.1.0, _36);
_8 = !_12;
_22.0 = (_22.2.0, _22.2.1, Field::<*mut i8>(Variant(_3, 0), 1));
_5 = Adt44::Variant2 { fld0: _18,fld1: 17840_i16 };
_26 = _22.2.1;
_22.2.1 = _26 >> _6.0;
_31 = _2 as u128;
_27 = _6.0;
_22.0.0 = _29.1 as i64;
Goto(bb20)
}
bb20 = {
Call(_46 = dump_var(3_usize, 29_usize, Move(_29), 6_usize, Move(_6), 1_usize, Move(_1), 20_usize, Move(_20)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_46 = dump_var(3_usize, 27_usize, Move(_27), 24_usize, Move(_24), 23_usize, Move(_23), 36_usize, Move(_36)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_46 = dump_var(3_usize, 10_usize, Move(_10), 8_usize, Move(_8), 28_usize, Move(_28), 12_usize, Move(_12)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: isize,mut _2: [u8; 3],mut _3: isize,mut _4: isize,mut _5: [u8; 3],mut _6: f32) -> Adt47 {
mir! {
type RET = Adt47;
let _7: Adt45;
let _8: (usize,);
let _9: isize;
let _10: isize;
let _11: char;
let _12: Adt57;
let _13: [u32; 1];
let _14: Adt58;
let _15: i8;
let _16: i64;
let _17: usize;
let _18: ((i64, i128, *mut i8), (u64, (i8, u128)), (i64, i128, *mut i8));
let _19: isize;
let _20: bool;
let _21: [usize; 1];
let _22: *const f64;
let _23: bool;
let _24: *const [usize; 1];
let _25: ([i32; 8], char);
let _26: isize;
let _27: ([u64; 7], char);
let _28: f32;
let _29: i128;
let _30: Adt48;
let _31: [u32; 6];
let _32: [u32; 1];
let _33: ();
let _34: ();
{
_2 = [196_u8,215_u8,75_u8];
_2 = _5;
_3 = _4;
Goto(bb1)
}
bb1 = {
_2 = [208_u8,93_u8,109_u8];
_2 = [147_u8,149_u8,123_u8];
_2 = [162_u8,255_u8,16_u8];
_2 = _5;
_4 = !_3;
_1 = _3;
_1 = !_3;
_8 = (8256052934581918178_usize,);
_9 = _4;
_4 = _9;
_8 = (4054240619792742720_usize,);
Goto(bb2)
}
bb2 = {
_9 = _4 ^ _3;
_10 = _9 & _3;
_9 = !_10;
_8.0 = 10704651470090948091_usize;
_2 = _5;
_11 = '\u{12d94}';
_3 = _10;
_5 = [253_u8,149_u8,37_u8];
match _8.0 {
0 => bb3,
1 => bb4,
2 => bb5,
10704651470090948091 => bb7,
_ => bb6
}
}
bb3 = {
_2 = [208_u8,93_u8,109_u8];
_2 = [147_u8,149_u8,123_u8];
_2 = [162_u8,255_u8,16_u8];
_2 = _5;
_4 = !_3;
_1 = _3;
_1 = !_3;
_8 = (8256052934581918178_usize,);
_9 = _4;
_4 = _9;
_8 = (4054240619792742720_usize,);
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
_2 = _5;
_3 = _10;
_11 = '\u{a8050}';
_2 = [73_u8,253_u8,27_u8];
_3 = _9 | _10;
_8.0 = 0_usize ^ 0_usize;
_2 = _5;
_11 = '\u{43445}';
_3 = _6 as isize;
_4 = _1 ^ _9;
_8.0 = !0_usize;
_11 = '\u{6fc39}';
_8.0 = 13896751494479249855_usize ^ 14277498323847282962_usize;
_11 = '\u{7d40c}';
_2 = _5;
_9 = _11 as isize;
_9 = _1 >> _10;
Goto(bb8)
}
bb8 = {
_6 = (-2_i8) as f32;
_6 = _9 as f32;
_1 = 3988661297340143698_i64 as isize;
_11 = '\u{c6ace}';
_16 = 17658_i16 as i64;
Call(_11 = fn5(_9, _10, _4, _1, _3, _4, _3, _9, _9, _1, _4, _9, _9), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_2 = [70_u8,31_u8,122_u8];
_2 = [158_u8,200_u8,166_u8];
_14 = Adt58::Variant0 { fld0: _5,fld1: (-8_i8) };
_5 = [126_u8,162_u8,110_u8];
_4 = _1;
place!(Field::<i8>(Variant(_14, 0), 1)) = (-76_i8);
_8 = (18441068944389936012_usize,);
_11 = '\u{a27b5}';
_18.0.1 = (-7650740335706184332558993429458947168_i128);
_2 = [208_u8,208_u8,99_u8];
_13 = [3451183074_u32];
_18.1.1.1 = !160278123031524281340034185611976851687_u128;
_4 = _9;
_13 = [4150471291_u32];
_18.0.2 = core::ptr::addr_of_mut!(_18.1.1.0);
match _18.0.1 {
0 => bb7,
1 => bb10,
332631626585232279130815614002309264288 => bb12,
_ => bb11
}
}
bb10 = {
_6 = (-2_i8) as f32;
_6 = _9 as f32;
_1 = 3988661297340143698_i64 as isize;
_11 = '\u{c6ace}';
_16 = 17658_i16 as i64;
Call(_11 = fn5(_9, _10, _4, _1, _3, _4, _3, _9, _9, _1, _4, _9, _9), ReturnTo(bb9), UnwindUnreachable())
}
bb11 = {
Return()
}
bb12 = {
_19 = _1 * _9;
_15 = Field::<i8>(Variant(_14, 0), 1);
_5 = [16_u8,171_u8,48_u8];
_18.0.0 = _16 ^ _16;
_20 = !false;
_18.2.1 = 7253291930727113402_u64 as i128;
_18.1.1 = (Field::<i8>(Variant(_14, 0), 1), 72803285621834372856980116688500992812_u128);
place!(Field::<[u8; 3]>(Variant(_14, 0), 0)) = _2;
place!(Field::<i8>(Variant(_14, 0), 1)) = 40_u8 as i8;
_16 = !_18.0.0;
_13 = [377833693_u32];
SetDiscriminant(_14, 0);
_4 = _19;
place!(Field::<i8>(Variant(_14, 0), 1)) = _18.1.1.1 as i8;
_13 = [329169602_u32];
_18.2.0 = _18.0.0;
_6 = 30087_u16 as f32;
Goto(bb13)
}
bb13 = {
_15 = !_18.1.1.0;
_18.2.2 = _18.0.2;
_17 = _8.0 ^ _8.0;
_5 = _2;
_18.2.2 = _18.0.2;
_11 = '\u{cce34}';
place!(Field::<[u8; 3]>(Variant(_14, 0), 0)) = _5;
Goto(bb14)
}
bb14 = {
_25.1 = _11;
_21 = [_17];
_11 = _25.1;
_5 = _2;
_19 = -_4;
SetDiscriminant(_14, 0);
_27.0 = [4069342225248159608_u64,16828184159246675920_u64,10640613874001841778_u64,2714571629611930438_u64,276277466415995113_u64,18421694425434224378_u64,8188298035654782283_u64];
_18.0 = _18.2;
_10 = 185_u8 as isize;
_24 = core::ptr::addr_of!(_21);
_23 = !_20;
_18.0 = (_18.2.0, _18.2.1, _18.2.2);
_5 = [148_u8,254_u8,90_u8];
_8.0 = _17;
_16 = _11 as i64;
_24 = core::ptr::addr_of!((*_24));
_14 = Adt58::Variant0 { fld0: _5,fld1: _18.1.1.0 };
_18.0.2 = core::ptr::addr_of_mut!(_15);
_21 = [_8.0];
_18.1.0 = 681055792_i32 as u64;
_25.0 = [1957831978_i32,634987236_i32,383028766_i32,(-436935126_i32),1159761665_i32,(-1801386383_i32),(-1384318363_i32),191787314_i32];
_10 = _19 ^ _9;
RET = Adt47::Variant0 { fld0: _20,fld1: _18.0.2 };
Goto(bb15)
}
bb15 = {
Call(_33 = dump_var(4_usize, 10_usize, Move(_10), 11_usize, Move(_11), 21_usize, Move(_21), 23_usize, Move(_23)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_33 = dump_var(4_usize, 4_usize, Move(_4), 1_usize, Move(_1), 13_usize, Move(_13), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(4_usize, 19_usize, Move(_19), 34_usize, _34, 34_usize, _34, 34_usize, _34), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize) -> char {
mir! {
type RET = char;
let _14: i128;
let _15: f32;
let _16: bool;
let _17: u32;
let _18: u128;
let _19: i128;
let _20: char;
let _21: char;
let _22: *const [usize; 1];
let _23: u64;
let _24: f32;
let _25: f32;
let _26: *const [usize; 1];
let _27: [i32; 8];
let _28: Adt42;
let _29: f64;
let _30: char;
let _31: i32;
let _32: [u64; 7];
let _33: [u64; 7];
let _34: Adt58;
let _35: f64;
let _36: f64;
let _37: ((i64, i128, *mut i8), (u64, (i8, u128)), (i64, i128, *mut i8));
let _38: [usize; 1];
let _39: bool;
let _40: Adt55;
let _41: u32;
let _42: ();
let _43: ();
{
_11 = !_3;
Goto(bb1)
}
bb1 = {
_2 = _13;
_13 = -_2;
_4 = _8;
RET = '\u{8ad63}';
_6 = 2577683714890422018_i64 as isize;
_15 = (-26860_i16) as f32;
_13 = -_11;
RET = '\u{2a491}';
_7 = _4;
_11 = -_12;
_14 = 37029073374141598099946521477545734735_i128 | 110481541387578456643448388316902864956_i128;
_10 = RET as isize;
Goto(bb2)
}
bb2 = {
_1 = 1667823125_u32 as isize;
_4 = _7;
RET = '\u{931a0}';
_4 = _12;
RET = '\u{b5a6}';
_8 = -_7;
_10 = _11 | _12;
_17 = 1295484806_u32 << _13;
_2 = !_5;
_16 = _4 == _10;
_1 = !_8;
Goto(bb3)
}
bb3 = {
_1 = (-77_i8) as isize;
_8 = _13 | _3;
RET = '\u{4d5f7}';
_4 = _13 * _10;
_13 = _11;
_2 = _14 as isize;
_6 = _8 << _10;
_19 = -_14;
_4 = _12 | _6;
_4 = _12;
_18 = 201275573190585877597749694861107071034_u128 << _19;
Call(_19 = fn6(_6, _2, _17, _12), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_4 = _12 * _11;
Goto(bb5)
}
bb5 = {
_10 = _13 + _3;
_9 = _6;
_12 = -_6;
_16 = true;
_3 = _8;
_19 = _14;
_23 = !16594432423076729145_u64;
_25 = 165_u8 as f32;
_9 = _12 * _6;
_16 = true;
_12 = 174_u8 as isize;
_20 = RET;
_5 = _2;
_11 = _3;
_1 = _23 as isize;
_8 = _9;
_21 = _20;
RET = _20;
_23 = 16064610942875957597_u64 & 9264705388462700804_u64;
_6 = _10 | _10;
Goto(bb6)
}
bb6 = {
_24 = _15;
RET = _20;
_21 = _20;
_18 = 82557641796569158567156713116674992015_u128;
_12 = 7_usize as isize;
_16 = true;
_27 = [1846744629_i32,1901140079_i32,197497159_i32,(-1841965615_i32),(-1323142142_i32),(-679875554_i32),(-1720875048_i32),(-913949656_i32)];
_24 = 17844855226538995513_usize as f32;
_14 = _23 as i128;
_10 = _18 as isize;
_2 = _11;
_7 = (-1527623250_i32) as isize;
_27 = [(-397336943_i32),2040121280_i32,(-572229399_i32),421283431_i32,994526010_i32,854472996_i32,1500864554_i32,(-42023181_i32)];
_4 = _9;
match _18 {
0 => bb1,
1 => bb4,
82557641796569158567156713116674992015 => bb8,
_ => bb7
}
}
bb7 = {
_4 = _12 * _11;
Goto(bb5)
}
bb8 = {
_7 = (-1339944593_i32) as isize;
_20 = _21;
_23 = 10320585365761251242_usize as u64;
_21 = _20;
Goto(bb9)
}
bb9 = {
RET = _20;
_29 = _25 as f64;
_3 = -_7;
_6 = _9 >> _17;
_8 = _13;
_24 = -_15;
_14 = -_19;
_20 = RET;
_25 = _15 * _15;
_23 = 10917842816541130740_u64 * 11904168399334195838_u64;
_21 = RET;
_14 = _19;
_7 = -_10;
_20 = RET;
_4 = _6 - _11;
_8 = _25 as isize;
_25 = _15 - _15;
_31 = 30795_i16 as i32;
Goto(bb10)
}
bb10 = {
_27 = [_31,_31,_31,_31,_31,_31,_31,_31];
_21 = _20;
match _18 {
0 => bb7,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb9,
82557641796569158567156713116674992015 => bb11,
_ => bb6
}
}
bb11 = {
_24 = 59_u8 as f32;
_15 = 3668350529778932381_i64 as f32;
_30 = _21;
_3 = _11 ^ _4;
_20 = _21;
_16 = true;
_14 = _19;
_7 = _9;
_13 = _9 ^ _8;
_30 = _20;
_27 = [_31,_31,_31,_31,_31,_31,_31,_31];
_4 = -_3;
_24 = _19 as f32;
_32 = [_23,_23,_23,_23,_23,_23,_23];
_6 = _9 + _2;
_14 = _19 << _23;
_33 = _32;
_37.0.0 = -4771865001719981623_i64;
_33 = _32;
_37.2.2 = core::ptr::addr_of_mut!(_37.1.1.0);
_33 = _32;
Goto(bb12)
}
bb12 = {
_27 = [_31,_31,_31,_31,_31,_31,_31,_31];
_37.2.0 = !_37.0.0;
_37.0.2 = core::ptr::addr_of_mut!(_37.1.1.0);
_26 = core::ptr::addr_of!(_38);
_14 = !_19;
(*_26) = [4170695942729398324_usize];
_31 = -(-606241444_i32);
_22 = core::ptr::addr_of!((*_26));
_3 = _37.0.0 as isize;
_37.2.0 = _37.0.0 >> _6;
_20 = RET;
_2 = _9 << _7;
_37.2.1 = _31 as i128;
_6 = _24 as isize;
(*_22) = [7272158343006937414_usize];
_30 = _21;
Call(_37.2.0 = fn7(_9, _27, _2, _9), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_37.0.1 = _19;
_37.1.1 = ((-11_i8), _18);
Goto(bb14)
}
bb14 = {
_33 = [_23,_23,_23,_23,_23,_23,_23];
_2 = 91_u8 as isize;
_4 = _9 >> _7;
_11 = _14 as isize;
_23 = _16 as u64;
_23 = 54616_u16 as u64;
_13 = _4;
Goto(bb15)
}
bb15 = {
Call(_42 = dump_var(5_usize, 38_usize, Move(_38), 11_usize, Move(_11), 30_usize, Move(_30), 17_usize, Move(_17)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_42 = dump_var(5_usize, 19_usize, Move(_19), 8_usize, Move(_8), 33_usize, Move(_33), 14_usize, Move(_14)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_42 = dump_var(5_usize, 9_usize, Move(_9), 27_usize, Move(_27), 21_usize, Move(_21), 10_usize, Move(_10)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(5_usize, 16_usize, Move(_16), 43_usize, _43, 43_usize, _43, 43_usize, _43), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: isize,mut _2: isize,mut _3: u32,mut _4: isize) -> i128 {
mir! {
type RET = i128;
let _5: isize;
let _6: [u32; 6];
let _7: char;
let _8: (usize,);
let _9: (u64, u64);
let _10: isize;
let _11: i64;
let _12: i8;
let _13: char;
let _14: isize;
let _15: (i8, u128);
let _16: bool;
let _17: i64;
let _18: [u64; 7];
let _19: [u32; 5];
let _20: usize;
let _21: Adt46;
let _22: (i8, u128);
let _23: [u32; 5];
let _24: f32;
let _25: i32;
let _26: bool;
let _27: u128;
let _28: ();
let _29: ();
{
RET = (-162323362236079602675733952628330863073_i128) >> _2;
RET = (-146887405368354052995098657515004843857_i128) + (-25916213175415502045289879627482068858_i128);
_1 = _3 as isize;
RET = (-106879044074995648840812881889760675188_i128) * (-18887696688754760370208315760827425222_i128);
_1 = _4;
_4 = 14553745821207462355_u64 as isize;
RET = (-72524390384496885445539282429823701296_i128) * 68579444591367638876246732443028966873_i128;
RET = (-74726761054968383965478453595747072173_i128) | 154052086752761276004493254436343014789_i128;
_4 = 86_u8 as isize;
Goto(bb1)
}
bb1 = {
_2 = _1;
_9.1 = !16188084616522402907_u64;
_6 = [_3,_3,_3,_3,_3,_3];
_11 = _9.1 as i64;
_5 = _2;
_9.0 = false as u64;
_7 = '\u{19fd7}';
_9 = (18422609468658129668_u64, 7318577070836090549_u64);
_9.1 = _7 as u64;
_12 = _11 as i8;
_10 = _1;
_1 = -_5;
_8.0 = _9.0 as usize;
_6 = [_3,_3,_3,_3,_3,_3];
match _9.0 {
0 => bb2,
1 => bb3,
2 => bb4,
18422609468658129668 => bb6,
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
_11 = _3 as i64;
_14 = _3 as isize;
_13 = _7;
_14 = _9.0 as isize;
RET = (-149587704945636947987933391971037929694_i128) | (-155600674187093272451710025839319760461_i128);
_9.1 = !_9.0;
RET = (-8642104312323561746501306860984758767_i128) ^ (-114666978911550861681589042823594910790_i128);
_9 = (3456208943322912093_u64, 16715736450156433660_u64);
_18 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.0];
_15.1 = 197569032746167082159803230406909472334_u128;
_17 = !_11;
_3 = 1004912309_u32 + 1603493723_u32;
_8 = (2_usize,);
_6 = [_3,_3,_3,_3,_3,_3];
match _9.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
3456208943322912093 => bb8,
_ => bb7
}
}
bb7 = {
Return()
}
bb8 = {
_15.0 = (-24824_i16) as i8;
_11 = -_17;
_10 = _1 | _1;
_13 = _7;
_7 = _13;
_1 = _2 >> RET;
_17 = _11;
_4 = _10;
RET = -(-24085002308623542610903395071013786892_i128);
_22 = (_12, _15.1);
_10 = _3 as isize;
_5 = _4;
match _9.0 {
0 => bb6,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
5 => bb13,
3456208943322912093 => bb15,
_ => bb14
}
}
bb9 = {
Return()
}
bb10 = {
_11 = _3 as i64;
_14 = _3 as isize;
_13 = _7;
_14 = _9.0 as isize;
RET = (-149587704945636947987933391971037929694_i128) | (-155600674187093272451710025839319760461_i128);
_9.1 = !_9.0;
RET = (-8642104312323561746501306860984758767_i128) ^ (-114666978911550861681589042823594910790_i128);
_9 = (3456208943322912093_u64, 16715736450156433660_u64);
_18 = [_9.1,_9.1,_9.1,_9.1,_9.1,_9.1,_9.0];
_15.1 = 197569032746167082159803230406909472334_u128;
_17 = !_11;
_3 = 1004912309_u32 + 1603493723_u32;
_8 = (2_usize,);
_6 = [_3,_3,_3,_3,_3,_3];
match _9.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
3456208943322912093 => bb8,
_ => bb7
}
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
_2 = _1;
_9.1 = !16188084616522402907_u64;
_6 = [_3,_3,_3,_3,_3,_3];
_11 = _9.1 as i64;
_5 = _2;
_9.0 = false as u64;
_7 = '\u{19fd7}';
_9 = (18422609468658129668_u64, 7318577070836090549_u64);
_9.1 = _7 as u64;
_12 = _11 as i8;
_10 = _1;
_1 = -_5;
_8.0 = _9.0 as usize;
_6 = [_3,_3,_3,_3,_3,_3];
match _9.0 {
0 => bb2,
1 => bb3,
2 => bb4,
18422609468658129668 => bb6,
_ => bb5
}
}
bb15 = {
_6 = [_3,_3,_3,_3,_3,_3];
_5 = 31555_i16 as isize;
_22.0 = !_15.0;
_9 = (8377347068880477542_u64, 5000190856610958402_u64);
_19 = [_3,_3,_3,_3,_3];
_15 = (_22.0, _22.1);
_15 = (_12, _22.1);
_10 = !_2;
_5 = _4;
_2 = 194_u8 as isize;
_13 = _7;
_7 = _13;
_24 = (-5339_i16) as f32;
_15 = (_12, _22.1);
_22.0 = _15.0;
_9.1 = !_9.0;
_26 = !true;
_15.0 = _12;
_9.1 = _9.0 * _9.0;
_8 = (6_usize,);
_15.0 = _12 - _22.0;
_25 = RET as i32;
_22 = (_12, _15.1);
_17 = _5 as i64;
Goto(bb16)
}
bb16 = {
Call(_28 = dump_var(6_usize, 19_usize, Move(_19), 8_usize, Move(_8), 15_usize, Move(_15), 10_usize, Move(_10)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(6_usize, 4_usize, Move(_4), 22_usize, Move(_22), 2_usize, Move(_2), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_28 = dump_var(6_usize, 12_usize, Move(_12), 3_usize, Move(_3), 29_usize, _29, 29_usize, _29), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: isize,mut _2: [i32; 8],mut _3: isize,mut _4: isize) -> i64 {
mir! {
type RET = i64;
let _5: ([i32; 8], char);
let _6: [usize; 1];
let _7: (u64, u64);
let _8: bool;
let _9: isize;
let _10: f64;
let _11: i16;
let _12: ([i32; 8], char);
let _13: f64;
let _14: u64;
let _15: Adt58;
let _16: (i8, u128);
let _17: (i8, u128);
let _18: [i32; 8];
let _19: i32;
let _20: *const [u32; 1];
let _21: ((i64, i128, *mut i8), (u64, (i8, u128)), (i64, i128, *mut i8));
let _22: [i64; 2];
let _23: [u8; 3];
let _24: &'static [u8; 3];
let _25: f32;
let _26: (u64, (i8, u128));
let _27: [i64; 2];
let _28: i64;
let _29: i32;
let _30: bool;
let _31: (*const f64, [i64; 2]);
let _32: [u64; 7];
let _33: ([u64; 7], char);
let _34: f64;
let _35: [i64; 2];
let _36: Adt46;
let _37: f32;
let _38: usize;
let _39: [u8; 3];
let _40: isize;
let _41: ();
let _42: ();
{
RET = !(-6952957145436165997_i64);
RET = 7558963915262583701_i64;
RET = (-8827121311807424134_i64) ^ (-2263634521210578837_i64);
_4 = _3 * _1;
_4 = false as isize;
_5 = (_2, '\u{613b3}');
_4 = _3;
RET = -6312201798800294218_i64;
_6 = [3_usize];
_3 = -_4;
_5 = (_2, '\u{73131}');
_2 = _5.0;
RET = 138_u8 as i64;
RET = 7689636459874212730_i64;
_3 = _4 >> _4;
_5 = (_2, '\u{d6c83}');
RET = (-5727138734048523180_i64);
_7 = (16140109467908633432_u64, 5577711805402121502_u64);
_5.0 = [843300669_i32,(-1471908129_i32),(-452554257_i32),(-333976150_i32),522046531_i32,1757026817_i32,1895184295_i32,(-2067597210_i32)];
_1 = !_3;
_1 = -_3;
RET = 1015785858400206485_i64;
Call(_6 = fn8(_3, _1, _3, _3, _3, _3, _4, _3, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = (-10774_i16) as i64;
_5.1 = '\u{f9697}';
_5.0 = [1853715527_i32,409929831_i32,(-992114229_i32),1680435510_i32,1789906853_i32,1029183909_i32,(-1942315330_i32),762592188_i32];
_7.1 = _7.0 / _7.0;
_12.1 = _5.1;
_5.1 = _12.1;
_12.1 = _5.1;
_10 = (-55714133500846900118492172536304599775_i128) as f64;
_5.0 = [(-1491137526_i32),(-1350350802_i32),(-578725406_i32),1900017618_i32,(-1722314391_i32),1402593052_i32,(-455532101_i32),801939378_i32];
_4 = 34961_u16 as isize;
_5 = (_2, _12.1);
_5 = (_2, _12.1);
Goto(bb2)
}
bb2 = {
_7.0 = !_7.1;
_8 = true;
Goto(bb3)
}
bb3 = {
_9 = _1 + _3;
_13 = _10;
_7.0 = _7.1 | _7.1;
_12.0 = [(-1994978595_i32),(-817359618_i32),(-2072161955_i32),(-1985495632_i32),244644646_i32,292398974_i32,(-1636370035_i32),(-1819101398_i32)];
_14 = _7.0 | _7.0;
_7 = (_14, _14);
_8 = false;
_4 = _1 ^ _1;
_9 = !_3;
_7.0 = _7.1;
_7 = (_14, _14);
_6 = [5_usize];
Goto(bb4)
}
bb4 = {
_8 = true | true;
_7.1 = 254_u8 as u64;
_4 = -_9;
_5.1 = _12.1;
Goto(bb5)
}
bb5 = {
_3 = 3294379051430930693_usize as isize;
_12 = _5;
_10 = _13;
_12 = (_2, _5.1);
_7.0 = 7001903560490319167338718282870280431_u128 as u64;
_8 = false;
RET = (-9119991893764533690_i64) - 1482881021705196152_i64;
_16.0 = -(-46_i8);
_13 = _10;
_16.1 = 305642643393288999633358455469742898322_u128;
_17 = _16;
_12.0 = _2;
_5.1 = _12.1;
Goto(bb6)
}
bb6 = {
_16 = _17;
_18 = [323556012_i32,(-139784939_i32),(-406304176_i32),(-266775652_i32),1812272544_i32,(-1899425716_i32),(-354396028_i32),(-751984243_i32)];
_12.0 = [297254281_i32,141210680_i32,(-622231736_i32),(-489101774_i32),487330753_i32,(-1417626151_i32),(-1019855679_i32),1704720408_i32];
_7 = (_14, _14);
RET = 1267884616_u32 as i64;
_9 = 118607410635717482190355282416847286246_i128 as isize;
_11 = 58956_u16 as i16;
_9 = _1;
_3 = -_1;
_21.2.0 = _8 as i64;
_4 = _3 << _1;
_21.1.0 = _14 - _14;
_21.0.0 = _21.2.0 & RET;
_21.0.2 = core::ptr::addr_of_mut!(_17.0);
_5.1 = _12.1;
Call(_1 = fn11(_9, _3, _9, _21.1.0, _4, _9, _3, _9, _2, _3, _4, _9, _4), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_21.1.0 = 3_usize as u64;
_21.2 = (_21.0.0, (-25323521925685144106661313798671995877_i128), _21.0.2);
_4 = -_9;
_21.1 = (_7.1, _16);
_21.1.1 = (_16.0, _16.1);
_10 = _13 * _13;
_8 = true;
_17 = (_21.1.1.0, _16.1);
_21.0.1 = _21.2.1 | _21.2.1;
_21.2 = _21.0;
_16.0 = _17.0 << _9;
_17.1 = !_16.1;
_21.2.0 = RET;
_21.2.1 = !_21.0.1;
_18 = [262101478_i32,(-698658110_i32),212998158_i32,825297965_i32,(-1693627908_i32),(-234088285_i32),(-1451461926_i32),(-162123121_i32)];
_5 = (_18, _12.1);
_26.1.1 = _21.2.1 as u128;
_26.1 = (_16.0, _17.1);
_11 = (-14294_i16) << _21.0.1;
_21.0 = _21.2;
_26.1 = (_16.0, _21.1.1.1);
_22 = [_21.2.0,RET];
_5.0 = [2120217051_i32,655883198_i32,(-1443089475_i32),88937806_i32,553802971_i32,1280969829_i32,1056627138_i32,1667105279_i32];
match _26.1.1 {
305642643393288999633358455469742898322 => bb8,
_ => bb1
}
}
bb8 = {
_21.0.2 = core::ptr::addr_of_mut!(_26.1.0);
_12 = (_18, _5.1);
_4 = _3;
_5.0 = [1980609659_i32,(-2073330438_i32),187110589_i32,(-1329500466_i32),(-426731358_i32),271602503_i32,(-1634157572_i32),694672432_i32];
match _26.1.1 {
0 => bb9,
1 => bb10,
305642643393288999633358455469742898322 => bb12,
_ => bb11
}
}
bb9 = {
RET = (-10774_i16) as i64;
_5.1 = '\u{f9697}';
_5.0 = [1853715527_i32,409929831_i32,(-992114229_i32),1680435510_i32,1789906853_i32,1029183909_i32,(-1942315330_i32),762592188_i32];
_7.1 = _7.0 / _7.0;
_12.1 = _5.1;
_5.1 = _12.1;
_12.1 = _5.1;
_10 = (-55714133500846900118492172536304599775_i128) as f64;
_5.0 = [(-1491137526_i32),(-1350350802_i32),(-578725406_i32),1900017618_i32,(-1722314391_i32),1402593052_i32,(-455532101_i32),801939378_i32];
_4 = 34961_u16 as isize;
_5 = (_2, _12.1);
_5 = (_2, _12.1);
Goto(bb2)
}
bb10 = {
_7.0 = !_7.1;
_8 = true;
Goto(bb3)
}
bb11 = {
_9 = _1 + _3;
_13 = _10;
_7.0 = _7.1 | _7.1;
_12.0 = [(-1994978595_i32),(-817359618_i32),(-2072161955_i32),(-1985495632_i32),244644646_i32,292398974_i32,(-1636370035_i32),(-1819101398_i32)];
_14 = _7.0 | _7.0;
_7 = (_14, _14);
_8 = false;
_4 = _1 ^ _1;
_9 = !_3;
_7.0 = _7.1;
_7 = (_14, _14);
_6 = [5_usize];
Goto(bb4)
}
bb12 = {
_21.0 = _21.2;
_28 = !RET;
_21.1.1.1 = _26.1.1;
_7.0 = _21.1.0;
_26.0 = _7.1 * _7.0;
_29 = (-1468835528_i32);
_26.1.0 = _21.1.1.0 | _16.0;
_16.1 = _21.1.1.1 ^ _17.1;
_19 = _29 | _29;
_31.0 = core::ptr::addr_of!(_13);
_23 = [237_u8,63_u8,251_u8];
_21.0.2 = _21.2.2;
_28 = _26.1.0 as i64;
_21.1.0 = 2696762253_u32 as u64;
_9 = _3;
_16.1 = !_17.1;
_31.1 = _22;
_4 = _9;
_16.1 = !_17.1;
_19 = _10 as i32;
_19 = -_29;
match _26.1.1 {
0 => bb6,
1 => bb10,
2 => bb7,
305642643393288999633358455469742898322 => bb13,
_ => bb4
}
}
bb13 = {
_30 = !_8;
_7.1 = !_14;
_17.0 = _26.1.0;
_1 = 23940_u16 as isize;
_15 = Adt58::Variant0 { fld0: _23,fld1: _16.0 };
_21.0.2 = core::ptr::addr_of_mut!(place!(Field::<i8>(Variant(_15, 0), 1)));
_26 = _21.1;
_21.2 = (_28, _21.0.1, _21.0.2);
_29 = _19 + _19;
_21.1 = (_7.1, _17);
_6 = [14778295160868306214_usize];
_34 = _13 - _10;
_5.1 = _12.1;
_26.1.1 = !_16.1;
_21.1.1.0 = -_16.0;
Goto(bb14)
}
bb14 = {
_27 = _31.1;
_21.1.1 = (_17.0, _26.1.1);
_21.1.1.0 = _9 as i8;
_21.2 = (_28, _21.0.1, _21.0.2);
SetDiscriminant(_15, 1);
_12.0 = [_29,_29,_19,_19,_19,_29,_29,_19];
_28 = !_21.2.0;
Goto(bb15)
}
bb15 = {
Call(_41 = dump_var(7_usize, 1_usize, Move(_1), 27_usize, Move(_27), 19_usize, Move(_19), 6_usize, Move(_6)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_41 = dump_var(7_usize, 14_usize, Move(_14), 4_usize, Move(_4), 22_usize, Move(_22), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_41 = dump_var(7_usize, 28_usize, Move(_28), 18_usize, Move(_18), 5_usize, Move(_5), 42_usize, _42), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize) -> [usize; 1] {
mir! {
type RET = [usize; 1];
let _10: (isize,);
let _11: (u64, u64);
let _12: Adt48;
let _13: char;
let _14: i8;
let _15: [u8; 3];
let _16: Adt58;
let _17: isize;
let _18: Adt58;
let _19: isize;
let _20: (i64, i128, *mut i8);
let _21: (usize,);
let _22: usize;
let _23: [u32; 5];
let _24: Adt47;
let _25: u64;
let _26: [usize; 1];
let _27: (u64, (i8, u128));
let _28: f32;
let _29: *const [usize; 1];
let _30: usize;
let _31: [u32; 6];
let _32: f64;
let _33: (isize,);
let _34: f64;
let _35: i64;
let _36: (u64, (i8, u128));
let _37: u8;
let _38: i8;
let _39: (usize,);
let _40: [i64; 2];
let _41: Adt48;
let _42: [u32; 5];
let _43: i128;
let _44: ();
let _45: ();
{
_1 = _9 ^ _6;
RET = [4_usize];
_7 = 57628_u16 as isize;
RET = [11718238647257613735_usize];
_1 = 17519527169645215023_usize as isize;
_4 = _2 & _8;
_11.0 = !5143987548971101248_u64;
_6 = 3807103181_u32 as isize;
_8 = !_3;
_5 = _4;
_5 = _11.0 as isize;
RET = [4_usize];
_1 = _4;
_11.0 = 15437802638136469075_u64 ^ 6520045509634593621_u64;
_10.0 = _4;
_5 = _2 >> _9;
_11.1 = 117746029102425556188936922212227856043_i128 as u64;
_11.1 = _11.0 ^ _11.0;
_11.0 = _11.1 * _11.1;
_8 = _9;
_8 = 535045712_i32 as isize;
_10.0 = _2 << _2;
_4 = _9 & _9;
Call(_5 = core::intrinsics::bswap(_4), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_11 = (8312379615068965679_u64, 9549027213272344880_u64);
_4 = _9;
_9 = !_5;
_4 = _3 ^ _2;
_9 = 101_u8 as isize;
_11.0 = 8098313410169645468586089751463947146_u128 as u64;
_1 = _10.0 | _3;
_9 = !_5;
_11.1 = _9 as u64;
_11.0 = 1503433472_u32 as u64;
_13 = '\u{fcf02}';
_6 = -_10.0;
_10.0 = _1 & _1;
_10.0 = 2779729625_u32 as isize;
_10.0 = _9;
_1 = _5 << _6;
_11.1 = !_11.0;
Goto(bb2)
}
bb2 = {
_2 = _3 >> _5;
_3 = 17346_u16 as isize;
Call(_1 = fn9(_6, _10, _4, _2, _10, _5, _10, _10, _5, _5, _4, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_13 = '\u{6baac}';
_7 = (-1093241079618126963_i64) as isize;
_14 = 99_i8;
_9 = _2 * _4;
_15 = [109_u8,88_u8,54_u8];
_10.0 = 144_u8 as isize;
_1 = -_5;
_2 = 703550859_u32 as isize;
_18 = Adt58::Variant0 { fld0: _15,fld1: _14 };
_13 = '\u{6ea5a}';
_14 = 82_u8 as i8;
_20.1 = (-744111633926071035609675615460838522_i128) * (-99139954030657103268857009840967615569_i128);
_19 = _5;
_20.0 = !(-7276090474451002692_i64);
_20.0 = 8892516033175503326_i64;
place!(Field::<i8>(Variant(_18, 0), 1)) = 16140_u16 as i8;
_10 = (_5,);
Call(_10 = fn10(_11.0, Field::<i8>(Variant(_18, 0), 1), _19, _6, _5, _9, _19, _19, _5, _4, _19, Move(_18), _6, _7), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_21.0 = 2_usize;
_17 = _9;
_10.0 = _4;
_10.0 = _6;
_19 = _17 * _10.0;
_21 = (11363284549134392582_usize,);
_1 = -_10.0;
_20.1 = (-128945052818118183214183733570161989135_i128) ^ (-28723160262617741998411281032742199832_i128);
_4 = _21.0 as isize;
_20.2 = core::ptr::addr_of_mut!(_14);
_15 = [249_u8,138_u8,74_u8];
_23 = [3000969657_u32,4254478408_u32,771221473_u32,3615549626_u32,2614420957_u32];
_3 = _19 ^ _6;
_8 = _11.0 as isize;
_23 = [618056246_u32,1862524241_u32,1614648222_u32,3508498531_u32,864207077_u32];
_2 = _9 - _3;
_21.0 = !12505784959844789661_usize;
_3 = _21.0 as isize;
_14 = _21.0 as i8;
_27.0 = _11.1 ^ _11.0;
_25 = !_27.0;
_27.1 = (_14, 12132341515251323712244180011973694555_u128);
_28 = _27.0 as f32;
Goto(bb5)
}
bb5 = {
_13 = '\u{6b5dc}';
_27.1.0 = true as i8;
_20.0 = !7088088614074044410_i64;
_26 = [_21.0];
_13 = '\u{109b4b}';
_13 = '\u{38c8b}';
_10.0 = _5;
_27.1.0 = _13 as i8;
_11.0 = _11.1;
_27.0 = _25;
_4 = _10.0;
_9 = _6 + _2;
_3 = !_5;
_20.0 = !(-6017430793328234429_i64);
match _27.1.1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
12132341515251323712244180011973694555 => bb8,
_ => bb7
}
}
bb6 = {
_11 = (8312379615068965679_u64, 9549027213272344880_u64);
_4 = _9;
_9 = !_5;
_4 = _3 ^ _2;
_9 = 101_u8 as isize;
_11.0 = 8098313410169645468586089751463947146_u128 as u64;
_1 = _10.0 | _3;
_9 = !_5;
_11.1 = _9 as u64;
_11.0 = 1503433472_u32 as u64;
_13 = '\u{fcf02}';
_6 = -_10.0;
_10.0 = _1 & _1;
_10.0 = 2779729625_u32 as isize;
_10.0 = _9;
_1 = _5 << _6;
_11.1 = !_11.0;
Goto(bb2)
}
bb7 = {
_13 = '\u{6baac}';
_7 = (-1093241079618126963_i64) as isize;
_14 = 99_i8;
_9 = _2 * _4;
_15 = [109_u8,88_u8,54_u8];
_10.0 = 144_u8 as isize;
_1 = -_5;
_2 = 703550859_u32 as isize;
_18 = Adt58::Variant0 { fld0: _15,fld1: _14 };
_13 = '\u{6ea5a}';
_14 = 82_u8 as i8;
_20.1 = (-744111633926071035609675615460838522_i128) * (-99139954030657103268857009840967615569_i128);
_19 = _5;
_20.0 = !(-7276090474451002692_i64);
_20.0 = 8892516033175503326_i64;
place!(Field::<i8>(Variant(_18, 0), 1)) = 16140_u16 as i8;
_10 = (_5,);
Call(_10 = fn10(_11.0, Field::<i8>(Variant(_18, 0), 1), _19, _6, _5, _9, _19, _19, _5, _4, _19, Move(_18), _6, _7), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_9 = _19 * _6;
_11 = (_25, _27.0);
_27.1 = (_14, 315836049929959421476641808085672767962_u128);
_1 = _13 as isize;
_17 = 1187219039_u32 as isize;
_11.0 = _25;
_17 = _6 * _3;
_1 = -_9;
_20.1 = 3068064708_u32 as i128;
_22 = true as usize;
_10.0 = _5 - _5;
Goto(bb9)
}
bb9 = {
_2 = _19 >> _4;
_13 = '\u{2e3a0}';
Goto(bb10)
}
bb10 = {
_4 = _2 << _6;
_11.0 = !_11.1;
_21.0 = !_22;
_17 = _5 ^ _3;
_4 = _1 << _10.0;
_18 = Adt58::Variant0 { fld0: _15,fld1: _27.1.0 };
_14 = 28622_i16 as i8;
_8 = _1;
RET = [_21.0];
_16 = Adt58::Variant0 { fld0: Field::<[u8; 3]>(Variant(_18, 0), 0),fld1: _27.1.0 };
_33 = _10;
_20.2 = core::ptr::addr_of_mut!(_27.1.0);
SetDiscriminant(_18, 1);
place!(Field::<([u64; 7], char)>(Variant(_18, 1), 1)).0 = [_11.0,_11.0,_27.0,_11.0,_27.0,_25,_11.0];
_1 = _8 + _33.0;
place!(Field::<(u64, (i8, u128))>(Variant(_18, 1), 0)) = (_11.0, _27.1);
place!(Field::<u64>(Variant(_18, 1), 3)) = _25;
_20.2 = core::ptr::addr_of_mut!(_14);
_36.0 = 1484806821_i32 as u64;
Call(_20.0 = core::intrinsics::transmute(_19), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_36 = (_25, _27.1);
place!(Field::<i8>(Variant(_16, 0), 1)) = Field::<(u64, (i8, u128))>(Variant(_18, 1), 0).1.0 << _9;
_22 = _20.1 as usize;
_27.1.1 = _13 as u128;
_36.0 = !_25;
place!(Field::<f64>(Variant(_18, 1), 4)) = _20.0 as f64;
_11 = (_27.0, _27.0);
_30 = _21.0 << _6;
_8 = -_17;
_31 = [843718983_u32,329439314_u32,4108673397_u32,1386489474_u32,2357129121_u32,3518765398_u32];
Goto(bb12)
}
bb12 = {
_36.0 = _20.0 as u64;
_29 = core::ptr::addr_of!(RET);
place!(Field::<i8>(Variant(_16, 0), 1)) = !Field::<(u64, (i8, u128))>(Variant(_18, 1), 0).1.0;
_20.0 = 9156446797800652141_i64 & 1767282417657218541_i64;
_9 = !_2;
_35 = !_20.0;
_35 = _20.0;
place!(Field::<(u64, (i8, u128))>(Variant(_18, 1), 0)).0 = !_36.0;
_20.0 = _35 & _35;
_20.0 = _35 >> Field::<(u64, (i8, u128))>(Variant(_18, 1), 0).0;
match _36.1.1 {
0 => bb8,
1 => bb2,
2 => bb3,
315836049929959421476641808085672767962 => bb13,
_ => bb4
}
}
bb13 = {
_34 = Field::<f64>(Variant(_18, 1), 4) + Field::<f64>(Variant(_18, 1), 4);
_11.0 = _11.1 >> _6;
_38 = -_27.1.0;
_33.0 = _28 as isize;
_1 = Field::<(u64, (i8, u128))>(Variant(_18, 1), 0).1.1 as isize;
SetDiscriminant(_16, 0);
_39.0 = Field::<(u64, (i8, u128))>(Variant(_18, 1), 0).0 as usize;
place!(Field::<([u64; 7], char)>(Variant(_18, 1), 1)).1 = _13;
_21.0 = _36.1.0 as usize;
_1 = _3;
Call(_21.0 = core::intrinsics::bswap(_30), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
place!(Field::<(u64, (i8, u128))>(Variant(_18, 1), 0)) = _36;
_40 = [_20.0,_20.0];
_3 = _8;
_11 = (Field::<(u64, (i8, u128))>(Variant(_18, 1), 0).0, Field::<(u64, (i8, u128))>(Variant(_18, 1), 0).0);
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(8_usize, 35_usize, Move(_35), 17_usize, Move(_17), 7_usize, Move(_7), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(8_usize, 9_usize, Move(_9), 36_usize, Move(_36), 40_usize, Move(_40), 5_usize, Move(_5)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(8_usize, 4_usize, Move(_4), 26_usize, Move(_26), 15_usize, Move(_15), 3_usize, Move(_3)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_44 = dump_var(8_usize, 2_usize, Move(_2), 13_usize, Move(_13), 6_usize, Move(_6), 45_usize, _45), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: isize,mut _2: (isize,),mut _3: isize,mut _4: isize,mut _5: (isize,),mut _6: isize,mut _7: (isize,),mut _8: (isize,),mut _9: isize,mut _10: isize,mut _11: isize,mut _12: isize) -> isize {
mir! {
type RET = isize;
let _13: u16;
let _14: char;
let _15: isize;
let _16: [i64; 2];
let _17: f64;
let _18: ();
let _19: ();
{
_5 = (_1,);
_5 = (_1,);
_4 = _8.0 >> _7.0;
_7.0 = (-161566630086202785858495206868405552419_i128) as isize;
_5 = (_10,);
_3 = !_1;
_2 = (_5.0,);
Goto(bb1)
}
bb1 = {
_4 = 3191860451894019114_usize as isize;
_1 = _12 * _2.0;
_7.0 = _6 << _6;
_14 = '\u{88694}';
_13 = 4880_u16;
_7 = _8;
RET = _11;
_7 = _5;
_9 = _5.0;
_13 = 166583570443157566366280576098459005418_i128 as u16;
RET = _8.0;
_6 = _11;
_16 = [4465761367803582525_i64,(-1717400192596821921_i64)];
RET = _12 * _9;
Goto(bb2)
}
bb2 = {
Call(_18 = dump_var(9_usize, 14_usize, Move(_14), 9_usize, Move(_9), 13_usize, Move(_13), 7_usize, Move(_7)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_18 = dump_var(9_usize, 5_usize, Move(_5), 3_usize, Move(_3), 11_usize, Move(_11), 19_usize, _19), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: u64,mut _2: i8,mut _3: isize,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: isize,mut _10: isize,mut _11: isize,mut _12: Adt58,mut _13: isize,mut _14: isize) -> (isize,) {
mir! {
type RET = (isize,);
let _15: [u8; 3];
let _16: u8;
let _17: ([i32; 8], char);
let _18: u64;
let _19: f64;
let _20: ();
let _21: ();
{
SetDiscriminant(_12, 1);
_3 = _4 - _4;
_16 = _2 as u8;
Call(_16 = core::intrinsics::transmute(_2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = '\u{f68ff}' as isize;
place!(Field::<(u64, (i8, u128))>(Variant(_12, 1), 0)).0 = 41258425451050172289673238005872355830_u128 as u64;
_4 = _13 * _5;
_6 = true as isize;
place!(Field::<f64>(Variant(_12, 1), 4)) = 57012575_u32 as f64;
RET = (_4,);
RET = (_9,);
_19 = Field::<f64>(Variant(_12, 1), 4) + Field::<f64>(Variant(_12, 1), 4);
place!(Field::<(u64, (i8, u128))>(Variant(_12, 1), 0)).1 = (_2, 201001004003788764842691512534727800478_u128);
place!(Field::<(u64, (i8, u128))>(Variant(_12, 1), 0)).1.0 = '\u{10eb4a}' as i8;
_1 = !Field::<(u64, (i8, u128))>(Variant(_12, 1), 0).0;
place!(Field::<([u64; 7], char)>(Variant(_12, 1), 1)).0 = [Field::<(u64, (i8, u128))>(Variant(_12, 1), 0).0,_1,Field::<(u64, (i8, u128))>(Variant(_12, 1), 0).0,Field::<(u64, (i8, u128))>(Variant(_12, 1), 0).0,Field::<(u64, (i8, u128))>(Variant(_12, 1), 0).0,Field::<(u64, (i8, u128))>(Variant(_12, 1), 0).0,Field::<(u64, (i8, u128))>(Variant(_12, 1), 0).0];
_3 = _8;
RET = (_4,);
_11 = _10;
place!(Field::<([u64; 7], char)>(Variant(_12, 1), 1)).1 = '\u{4afa0}';
_17.0 = [856611631_i32,(-1310444539_i32),(-2092204237_i32),1098098883_i32,345851688_i32,(-416205968_i32),(-932528462_i32),2118300126_i32];
place!(Field::<(u64, (i8, u128))>(Variant(_12, 1), 0)).0 = !_1;
place!(Field::<(u64, (i8, u128))>(Variant(_12, 1), 0)).1 = (_2, 22296960664934932004474085325408741896_u128);
place!(Field::<([u64; 7], char)>(Variant(_12, 1), 1)).1 = '\u{365dd}';
_5 = RET.0;
place!(Field::<(u64, (i8, u128))>(Variant(_12, 1), 0)).1 = (_2, 61885784783769322394377077614399826722_u128);
_11 = _1 as isize;
_4 = Field::<(u64, (i8, u128))>(Variant(_12, 1), 0).1.0 as isize;
place!(Field::<([u64; 7], char)>(Variant(_12, 1), 1)).1 = '\u{34f09}';
Goto(bb2)
}
bb2 = {
Call(_20 = dump_var(10_usize, 10_usize, Move(_10), 14_usize, Move(_14), 6_usize, Move(_6), 1_usize, Move(_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_20 = dump_var(10_usize, 4_usize, Move(_4), 8_usize, Move(_8), 7_usize, Move(_7), 21_usize, _21), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: u64,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: isize,mut _9: [i32; 8],mut _10: isize,mut _11: isize,mut _12: isize,mut _13: isize) -> isize {
mir! {
type RET = isize;
let _14: isize;
let _15: char;
let _16: ();
let _17: ();
{
RET = _5 >> _13;
_11 = _7 << RET;
_5 = -RET;
_14 = 7_usize as isize;
_13 = _1 >> _7;
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(11_usize, 7_usize, Move(_7), 6_usize, Move(_6), 5_usize, Move(_5), 9_usize, Move(_9)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_16 = dump_var(11_usize, 4_usize, Move(_4), 2_usize, Move(_2), 1_usize, Move(_1), 17_usize, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: [u64; 7],mut _2: [i32; 8],mut _3: [u8; 3],mut _4: [u64; 7],mut _5: isize,mut _6: [u64; 7],mut _7: [i32; 8],mut _8: isize,mut _9: (isize,),mut _10: [u8; 3]) -> Adt44 {
mir! {
type RET = Adt44;
let _11: [i32; 8];
let _12: (u64, u64);
let _13: &'static [u8; 3];
let _14: *const f64;
let _15: [i32; 8];
let _16: char;
let _17: isize;
let _18: (isize,);
let _19: f32;
let _20: isize;
let _21: ([i32; 8], char);
let _22: u128;
let _23: Adt42;
let _24: (u64, (i8, u128));
let _25: char;
let _26: [u32; 6];
let _27: Adt44;
let _28: [u128; 8];
let _29: f64;
let _30: [u32; 5];
let _31: u32;
let _32: [u32; 5];
let _33: char;
let _34: [i64; 2];
let _35: [u32; 5];
let _36: isize;
let _37: isize;
let _38: usize;
let _39: ();
let _40: ();
{
_7 = [(-598018095_i32),1083997946_i32,(-1818901147_i32),(-1832159051_i32),(-1407259304_i32),(-1287210189_i32),(-1388137940_i32),119719054_i32];
_12.0 = false as u64;
RET = Adt44::Variant2 { fld0: 4142606939_u32,fld1: (-15953_i16) };
_1 = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
place!(Field::<i16>(Variant(RET, 2), 1)) = !2517_i16;
_9.0 = (-348055728_i32) as isize;
_6 = [_12.0,_12.0,_12.0,_12.0,_12.0,_12.0,_12.0];
_8 = (-111_i8) as isize;
_13 = &_10;
_11 = [(-957485803_i32),(-1205761899_i32),(-59005134_i32),927804124_i32,(-616581433_i32),2101048880_i32,718695144_i32,(-974468263_i32)];
_7 = [858714640_i32,(-1198889300_i32),374739887_i32,683483887_i32,(-299965705_i32),2078234509_i32,(-833939182_i32),(-1176513735_i32)];
Goto(bb1)
}
bb1 = {
RET = Adt44::Variant2 { fld0: 4259545488_u32,fld1: 29317_i16 };
_12.0 = 609915280_i32 as u64;
_7 = [(-1676176618_i32),1752889618_i32,(-1982466755_i32),1600721076_i32,467148480_i32,(-553764089_i32),393896267_i32,1720300195_i32];
_3 = [100_u8,186_u8,114_u8];
place!(Field::<u32>(Variant(RET, 2), 0)) = 1102730534_u32 ^ 2302122669_u32;
_10 = _3;
place!(Field::<i16>(Variant(RET, 2), 1)) = 18305_i16;
_3 = _10;
_12.1 = !_12.0;
_12.1 = _12.0 & _12.0;
_3 = [142_u8,80_u8,101_u8];
_7 = [(-1435435627_i32),(-1077514970_i32),1692148783_i32,(-801724656_i32),1894296688_i32,(-458675798_i32),26006187_i32,308068523_i32];
_10 = [72_u8,191_u8,154_u8];
_1 = _4;
_6 = [_12.1,_12.0,_12.1,_12.1,_12.1,_12.0,_12.1];
_15 = [(-1492817543_i32),1747890126_i32,(-1649835665_i32),(-95271454_i32),1648631007_i32,1893532348_i32,1291791952_i32,(-144749704_i32)];
_2 = _15;
_1 = [_12.1,_12.1,_12.0,_12.1,_12.0,_12.0,_12.0];
_6 = [_12.1,_12.1,_12.1,_12.0,_12.1,_12.1,_12.1];
_13 = &_10;
_5 = _9.0 - _8;
place!(Field::<i16>(Variant(RET, 2), 1)) = 5351_i16;
_4 = [_12.1,_12.1,_12.1,_12.0,_12.1,_12.1,_12.1];
match Field::<i16>(Variant(RET, 2), 1) {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
5351 => bb10,
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
_15 = _11;
_12 = (14898219996292585496_u64, 485229097157387176_u64);
_9 = (_5,);
_12.1 = _12.0 % _12.0;
_7 = _2;
_16 = '\u{3a198}';
_4 = _6;
_18 = (_5,);
_10 = [177_u8,48_u8,84_u8];
SetDiscriminant(RET, 3);
place!(Field::<(i64, i128, *mut i8)>(Variant(RET, 3), 2)).2 = core::ptr::addr_of_mut!(place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1.0);
Goto(bb11)
}
bb11 = {
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1.1 = !201801351929601779592577052227844863527_u128;
place!(Field::<[u32; 1]>(Variant(RET, 3), 5)) = [1741767920_u32];
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1 = (8_i8, 29965294234393443615534978044378329984_u128);
place!(Field::<u8>(Variant(RET, 3), 1)) = 133_u8;
_21 = (_7, _16);
_9 = _18;
_1 = _4;
place!(Field::<(isize,)>(Variant(RET, 3), 4)) = _18;
_18 = (_9.0,);
_13 = &_10;
_18 = (_8,);
_19 = Field::<u8>(Variant(RET, 3), 1) as f32;
_20 = true as isize;
_22 = Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1.1 & Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1.1;
place!(Field::<(isize,)>(Variant(RET, 3), 4)) = _9;
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1.1 = Field::<u8>(Variant(RET, 3), 1) as u128;
_24.1.1 = 4260560308_u32 as u128;
_17 = Field::<u8>(Variant(RET, 3), 1) as isize;
Goto(bb12)
}
bb12 = {
_15 = [1343423873_i32,988546577_i32,2005521386_i32,1865501568_i32,948847183_i32,90711663_i32,1230310431_i32,407396753_i32];
_3 = (*_13);
_24.1 = (Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1.0, _22);
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1 = (_24.1.0, _22);
_6 = [_12.0,_12.1,_12.0,_12.1,_12.1,_12.0,_12.0];
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).0 = _12.1 * _12.1;
_9 = _18;
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1.1 = _22 >> Field::<(isize,)>(Variant(RET, 3), 4).0;
_12.1 = Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).0 * Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).0;
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).0 = _12.1;
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)) = (_12.0, _24.1);
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)) = (_12.0, _24.1);
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).0 = _19 as u64;
place!(Field::<(isize,)>(Variant(RET, 3), 4)) = _18;
_7 = [(-2024470408_i32),(-2041672268_i32),(-1282164207_i32),(-451949725_i32),(-162579107_i32),19887750_i32,(-1373375191_i32),2140598280_i32];
match Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1.0 {
0 => bb7,
1 => bb6,
2 => bb8,
3 => bb10,
8 => bb13,
_ => bb5
}
}
bb13 = {
place!(Field::<[u32; 1]>(Variant(RET, 3), 5)) = [3820708128_u32];
place!(Field::<(i64, i128, *mut i8)>(Variant(RET, 3), 2)).0 = -(-6825554804235500736_i64);
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)) = (_12.1, _24.1);
_9.0 = _20 * _17;
place!(Field::<(i64, i128, *mut i8)>(Variant(RET, 3), 2)).0 = _19 as i64;
place!(Field::<(i64, i128, *mut i8)>(Variant(RET, 3), 2)).1 = _16 as i128;
_19 = 1160061510_u32 as f32;
_18 = (Field::<(isize,)>(Variant(RET, 3), 4).0,);
_28 = [_24.1.1,_24.1.1,Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1.1,Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1.1,_24.1.1,_22,Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1.1,_22];
_25 = _16;
_13 = &(*_13);
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).0 = _12.1 | _12.1;
_30 = [22676503_u32,135788501_u32,2985066120_u32,410418527_u32,654104262_u32];
_18.0 = Field::<(i64, i128, *mut i8)>(Variant(RET, 3), 2).0 as isize;
_3 = (*_13);
_13 = &_3;
_24.1.1 = Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1.1;
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1.0 = !_24.1.0;
_24 = (_12.1, Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1);
_15 = _2;
place!(Field::<(isize,)>(Variant(RET, 3), 4)).0 = !_18.0;
place!(Field::<(i64, i128, *mut i8)>(Variant(RET, 3), 2)).2 = core::ptr::addr_of_mut!(place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1.0);
_26 = [2761154271_u32,600512205_u32,3219494744_u32,830547619_u32,3490800893_u32,3605020984_u32];
_29 = 42984_u16 as f64;
_24.1 = (Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1.0, _22);
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1.1 = !_24.1.1;
Goto(bb14)
}
bb14 = {
_23 = Adt42::Variant1 { fld0: _12,fld1: _22,fld2: _29 };
place!(Field::<(isize,)>(Variant(RET, 3), 4)).0 = Field::<u8>(Variant(RET, 3), 1) as isize;
place!(Field::<[u32; 1]>(Variant(RET, 3), 5)) = [586280947_u32];
Call(_27 = fn13(_11, _24.0, Field::<(u64, (i8, u128))>(Variant(RET, 3), 3), Field::<(u64, u64)>(Variant(_23, 1), 0).1, Move(_23), _20, Field::<(u64, (i8, u128))>(Variant(RET, 3), 3), _22, Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1.1, _21, _11, _2, _18.0, _11, _12, _12), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
place!(Field::<*mut i8>(Variant(RET, 3), 0)) = core::ptr::addr_of_mut!(place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1.0);
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)) = (_12.0, _24.1);
place!(Field::<(isize,)>(Variant(RET, 3), 4)) = (_18.0,);
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1.1 = _22 >> _20;
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1.1 = _22;
place!(Field::<u8>(Variant(RET, 3), 1)) = Field::<(i64, i128, *mut i8)>(Variant(RET, 3), 2).1 as u8;
_18.0 = -_20;
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).0 = Field::<(usize,)>(Variant(_27, 1), 1).0 as u64;
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1.0 = Field::<(isize,)>(Variant(RET, 3), 4).0 as i8;
SetDiscriminant(RET, 0);
place!(Field::<[u32; 1]>(Variant(RET, 0), 3)) = [3817721655_u32];
SetDiscriminant(_27, 1);
place!(Field::<bool>(Variant(RET, 0), 0)) = !true;
place!(Field::<*const f64>(Variant(_27, 1), 2)) = core::ptr::addr_of!(_29);
_10 = _3;
place!(Field::<u16>(Variant(RET, 0), 4)) = !23003_u16;
_24.1.1 = 203_u8 as u128;
place!(Field::<u16>(Variant(RET, 0), 4)) = 36808_u16 & 21213_u16;
_18 = (_9.0,);
_13 = &_10;
_8 = _17 * _5;
match _12.0 {
0 => bb14,
1 => bb13,
2 => bb8,
3 => bb16,
4 => bb17,
14898219996292585496 => bb19,
_ => bb18
}
}
bb16 = {
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1.1 = !201801351929601779592577052227844863527_u128;
place!(Field::<[u32; 1]>(Variant(RET, 3), 5)) = [1741767920_u32];
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1 = (8_i8, 29965294234393443615534978044378329984_u128);
place!(Field::<u8>(Variant(RET, 3), 1)) = 133_u8;
_21 = (_7, _16);
_9 = _18;
_1 = _4;
place!(Field::<(isize,)>(Variant(RET, 3), 4)) = _18;
_18 = (_9.0,);
_13 = &_10;
_18 = (_8,);
_19 = Field::<u8>(Variant(RET, 3), 1) as f32;
_20 = true as isize;
_22 = Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1.1 & Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1.1;
place!(Field::<(isize,)>(Variant(RET, 3), 4)) = _9;
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1.1 = Field::<u8>(Variant(RET, 3), 1) as u128;
_24.1.1 = 4260560308_u32 as u128;
_17 = Field::<u8>(Variant(RET, 3), 1) as isize;
Goto(bb12)
}
bb17 = {
place!(Field::<[u32; 1]>(Variant(RET, 3), 5)) = [3820708128_u32];
place!(Field::<(i64, i128, *mut i8)>(Variant(RET, 3), 2)).0 = -(-6825554804235500736_i64);
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)) = (_12.1, _24.1);
_9.0 = _20 * _17;
place!(Field::<(i64, i128, *mut i8)>(Variant(RET, 3), 2)).0 = _19 as i64;
place!(Field::<(i64, i128, *mut i8)>(Variant(RET, 3), 2)).1 = _16 as i128;
_19 = 1160061510_u32 as f32;
_18 = (Field::<(isize,)>(Variant(RET, 3), 4).0,);
_28 = [_24.1.1,_24.1.1,Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1.1,Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1.1,_24.1.1,_22,Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1.1,_22];
_25 = _16;
_13 = &(*_13);
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).0 = _12.1 | _12.1;
_30 = [22676503_u32,135788501_u32,2985066120_u32,410418527_u32,654104262_u32];
_18.0 = Field::<(i64, i128, *mut i8)>(Variant(RET, 3), 2).0 as isize;
_3 = (*_13);
_13 = &_3;
_24.1.1 = Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1.1;
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1.0 = !_24.1.0;
_24 = (_12.1, Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1);
_15 = _2;
place!(Field::<(isize,)>(Variant(RET, 3), 4)).0 = !_18.0;
place!(Field::<(i64, i128, *mut i8)>(Variant(RET, 3), 2)).2 = core::ptr::addr_of_mut!(place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1.0);
_26 = [2761154271_u32,600512205_u32,3219494744_u32,830547619_u32,3490800893_u32,3605020984_u32];
_29 = 42984_u16 as f64;
_24.1 = (Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1.0, _22);
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1.1 = !_24.1.1;
Goto(bb14)
}
bb18 = {
_15 = [1343423873_i32,988546577_i32,2005521386_i32,1865501568_i32,948847183_i32,90711663_i32,1230310431_i32,407396753_i32];
_3 = (*_13);
_24.1 = (Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1.0, _22);
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1 = (_24.1.0, _22);
_6 = [_12.0,_12.1,_12.0,_12.1,_12.1,_12.0,_12.0];
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).0 = _12.1 * _12.1;
_9 = _18;
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).1.1 = _22 >> Field::<(isize,)>(Variant(RET, 3), 4).0;
_12.1 = Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).0 * Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).0;
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).0 = _12.1;
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)) = (_12.0, _24.1);
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)) = (_12.0, _24.1);
place!(Field::<(u64, (i8, u128))>(Variant(RET, 3), 3)).0 = _19 as u64;
place!(Field::<(isize,)>(Variant(RET, 3), 4)) = _18;
_7 = [(-2024470408_i32),(-2041672268_i32),(-1282164207_i32),(-451949725_i32),(-162579107_i32),19887750_i32,(-1373375191_i32),2140598280_i32];
match Field::<(u64, (i8, u128))>(Variant(RET, 3), 3).1.0 {
0 => bb7,
1 => bb6,
2 => bb8,
3 => bb10,
8 => bb13,
_ => bb5
}
}
bb19 = {
place!(Field::<isize>(Variant(RET, 0), 2)) = _9.0 & _20;
_9 = _18;
_13 = &_3;
_22 = !_24.1.1;
_24.0 = _12.1 & _12.1;
_13 = &_10;
_6 = [_24.0,_24.0,_24.0,_12.1,_24.0,_24.0,_12.1];
_12.0 = !_24.0;
_29 = 17540_i16 as f64;
_28 = [_24.1.1,_24.1.1,_22,_24.1.1,_22,_22,_22,_22];
RET = Adt44::Variant2 { fld0: 1659409680_u32,fld1: (-8997_i16) };
_21 = (_15, _25);
place!(Field::<i16>(Variant(RET, 2), 1)) = -9032_i16;
_32 = [4127684058_u32,3712778156_u32,3011173540_u32,226117963_u32,3999896073_u32];
Goto(bb20)
}
bb20 = {
place!(Field::<(usize,)>(Variant(_27, 1), 1)) = (2_usize,);
_15 = [577990545_i32,(-1924123104_i32),933830877_i32,1581692669_i32,(-342337581_i32),544270168_i32,(-1002755927_i32),444504654_i32];
_8 = _24.1.1 as isize;
_22 = _24.1.1;
_29 = _22 as f64;
_19 = _29 as f32;
_21.0 = [(-1000337803_i32),(-317770947_i32),784661118_i32,(-827242171_i32),(-1064233757_i32),320007516_i32,577875807_i32,(-1431359605_i32)];
place!(Field::<u32>(Variant(RET, 2), 0)) = Field::<(usize,)>(Variant(_27, 1), 1).0 as u32;
Goto(bb21)
}
bb21 = {
Call(_39 = dump_var(12_usize, 30_usize, Move(_30), 9_usize, Move(_9), 3_usize, Move(_3), 11_usize, Move(_11)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_39 = dump_var(12_usize, 32_usize, Move(_32), 15_usize, Move(_15), 7_usize, Move(_7), 25_usize, Move(_25)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_39 = dump_var(12_usize, 28_usize, Move(_28), 8_usize, Move(_8), 12_usize, Move(_12), 20_usize, Move(_20)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [i32; 8],mut _2: u64,mut _3: (u64, (i8, u128)),mut _4: u64,mut _5: Adt42,mut _6: isize,mut _7: (u64, (i8, u128)),mut _8: u128,mut _9: u128,mut _10: ([i32; 8], char),mut _11: [i32; 8],mut _12: [i32; 8],mut _13: isize,mut _14: [i32; 8],mut _15: (u64, u64),mut _16: (u64, u64)) -> Adt44 {
mir! {
type RET = Adt44;
let _17: bool;
let _18: (usize,);
let _19: (*const f64, [i64; 2]);
let _20: (i8, u128);
let _21: char;
let _22: f32;
let _23: Adt50;
let _24: *mut &'static [u8; 3];
let _25: isize;
let _26: [u64; 7];
let _27: [u64; 7];
let _28: bool;
let _29: Adt42;
let _30: *mut &'static [u8; 3];
let _31: isize;
let _32: [u64; 7];
let _33: [u64; 7];
let _34: bool;
let _35: i32;
let _36: i8;
let _37: u64;
let _38: [u32; 6];
let _39: i128;
let _40: ((i64, i128, *mut i8), (u64, (i8, u128)), (i64, i128, *mut i8));
let _41: Adt49;
let _42: ();
let _43: ();
{
_3.1.1 = true as u128;
_3.1 = (_7.1.0, _7.1.1);
_10.0 = [(-1024040014_i32),614520653_i32,(-1441972602_i32),(-1857079905_i32),1755341961_i32,1831738471_i32,(-575323760_i32),221376748_i32];
_3 = (_2, _7.1);
match _15.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
14898219996292585496 => bb8,
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
_7 = (_4, _3.1);
match Field::<(u64, u64)>(Variant(_5, 1), 0).0 {
0 => bb3,
14898219996292585496 => bb9,
_ => bb2
}
}
bb9 = {
_6 = _13;
_10.1 = '\u{e1719}';
_18.0 = 14252969123484065461_usize * 4_usize;
_18.0 = 520540108870376616_i64 as usize;
_16.0 = true as u64;
_19.1 = [343448253869921033_i64,(-7451474347392137445_i64)];
_3.1.1 = _8 << _2;
_2 = !_4;
SetDiscriminant(_5, 2);
_16.1 = _15.1;
place!(Field::<f64>(Variant(_5, 2), 5)) = 148_u8 as f64;
place!(Field::<f64>(Variant(_5, 2), 5)) = _18.0 as f64;
_20.1 = Field::<f64>(Variant(_5, 2), 5) as u128;
_20.1 = 147_u8 as u128;
_7.1.0 = _3.1.0;
_10.1 = '\u{6c653}';
place!(Field::<[u64; 7]>(Variant(_5, 2), 0)) = [_3.0,_7.0,_2,_15.1,_4,_3.0,_2];
_17 = !true;
_7.0 = Field::<f64>(Variant(_5, 2), 5) as u64;
place!(Field::<[u32; 6]>(Variant(_5, 2), 4)) = [174983638_u32,2443806945_u32,1446362072_u32,1760210911_u32,1287176367_u32,2430070988_u32];
Goto(bb10)
}
bb10 = {
_17 = true;
_18.0 = !7_usize;
_16.1 = _9 as u64;
_20.0 = _3.1.0;
_9 = _8 << _4;
_3.1.0 = !_7.1.0;
_9 = !_3.1.1;
RET = Adt44::Variant2 { fld0: 3814724444_u32,fld1: (-26623_i16) };
place!(Field::<i8>(Variant(_5, 2), 3)) = _20.0;
place!(Field::<u32>(Variant(RET, 2), 0)) = 3312890694_u32;
_8 = _7.1.1 - _3.1.1;
_3 = (_2, _7.1);
_9 = !_8;
_15 = (_2, _2);
_16.0 = _15.0 | _16.1;
_10 = (_11, '\u{d5407}');
place!(Field::<f64>(Variant(_5, 2), 5)) = (-145346666956020404607099803223823894012_i128) as f64;
place!(Field::<i16>(Variant(RET, 2), 1)) = -(-4032_i16);
_28 = _3.0 != _3.0;
SetDiscriminant(RET, 0);
place!(Field::<u16>(Variant(RET, 0), 4)) = 4007_u16 ^ 53648_u16;
_25 = _13;
Goto(bb11)
}
bb11 = {
place!(Field::<bool>(Variant(RET, 0), 0)) = !_28;
place!(Field::<*const [u32; 1]>(Variant(RET, 0), 1)) = core::ptr::addr_of!(place!(Field::<[u32; 1]>(Variant(RET, 0), 3)));
_19.0 = core::ptr::addr_of!(place!(Field::<f64>(Variant(_5, 2), 5)));
place!(Field::<u128>(Variant(RET, 0), 5)) = 697838919_u32 as u128;
_4 = _3.0 | _15.0;
place!(Field::<i8>(Variant(_5, 2), 3)) = _7.1.0 ^ _7.1.0;
_27 = Field::<[u64; 7]>(Variant(_5, 2), 0);
place!(Field::<bool>(Variant(RET, 0), 0)) = _28;
place!(Field::<u64>(Variant(RET, 0), 6)) = _16.0 * _3.0;
_26 = [_15.1,Field::<u64>(Variant(RET, 0), 6),_4,_3.0,_4,_2,_15.0];
_19.0 = core::ptr::addr_of!(place!(Field::<f64>(Variant(_5, 2), 5)));
_9 = !_8;
_21 = _10.1;
_14 = [(-944877520_i32),1942189228_i32,(-1269706411_i32),2049932486_i32,(-1296625419_i32),1461336193_i32,(-1318891719_i32),(-1818162525_i32)];
_33 = [_4,_4,_15.1,Field::<u64>(Variant(RET, 0), 6),_15.1,_3.0,Field::<u64>(Variant(RET, 0), 6)];
place!(Field::<u64>(Variant(RET, 0), 6)) = !_4;
_18 = (1_usize,);
_34 = !_28;
place!(Field::<(isize,)>(Variant(_5, 2), 6)) = (_6,);
_21 = _10.1;
RET = Adt44::Variant1 { fld0: _19.1,fld1: _18,fld2: _19.0 };
Goto(bb12)
}
bb12 = {
_10.1 = _21;
place!(Field::<i8>(Variant(_5, 2), 3)) = _7.1.0 ^ _7.1.0;
_31 = -_25;
_16.0 = !_4;
_6 = _25;
_16 = (_15.0, _2);
match _18.0 {
0 => bb2,
1 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_3.1.1 = _9 * _8;
_15.0 = _2 ^ _16.1;
SetDiscriminant(RET, 0);
_26 = [_15.0,_4,_2,_16.0,_16.1,_15.0,_16.0];
_7.1.0 = _21 as i8;
place!(Field::<u64>(Variant(RET, 0), 6)) = _4 * _15.1;
place!(Field::<[u32; 1]>(Variant(RET, 0), 3)) = [182648916_u32];
place!(Field::<*const [u32; 1]>(Variant(RET, 0), 1)) = core::ptr::addr_of!(place!(Field::<[u32; 1]>(Variant(RET, 0), 3)));
place!(Field::<[u32; 6]>(Variant(_5, 2), 4)) = [4130541703_u32,4144794054_u32,3100113072_u32,3197824747_u32,2844001083_u32,2130181390_u32];
place!(Field::<isize>(Variant(RET, 0), 2)) = _31 + _13;
_4 = !_16.1;
_36 = 3848704363451759477_i64 as i8;
_7 = _3;
_10.0 = _1;
_6 = Field::<f64>(Variant(_5, 2), 5) as isize;
_21 = _10.1;
place!(Field::<u128>(Variant(RET, 0), 5)) = 124_u8 as u128;
place!(Field::<u16>(Variant(_5, 2), 1)) = !58943_u16;
_29 = Adt42::Variant1 { fld0: _16,fld1: _7.1.1,fld2: Field::<f64>(Variant(_5, 2), 5) };
_10.1 = _21;
_9 = Field::<u128>(Variant(_29, 1), 1);
_31 = Field::<isize>(Variant(RET, 0), 2);
_3.1.1 = _8;
_15.0 = !_15.1;
place!(Field::<[u32; 1]>(Variant(RET, 0), 3)) = [1747080680_u32];
_26 = _33;
_3.1 = (_36, Field::<u128>(Variant(_29, 1), 1));
_35 = (-1923793641_i32);
_19.1 = [(-8989228556523840039_i64),1168416662073270297_i64];
place!(Field::<bool>(Variant(RET, 0), 0)) = _28;
Goto(bb15)
}
bb15 = {
_12 = _14;
_34 = !Field::<bool>(Variant(RET, 0), 0);
Goto(bb16)
}
bb16 = {
place!(Field::<u16>(Variant(_5, 2), 1)) = Field::<f64>(Variant(_29, 1), 2) as u16;
_17 = !_34;
RET = Adt44::Variant1 { fld0: _19.1,fld1: _18,fld2: _19.0 };
_18.0 = !Field::<(usize,)>(Variant(RET, 1), 1).0;
place!(Field::<isize>(Variant(_5, 2), 2)) = _31 - _25;
_22 = Field::<(u64, u64)>(Variant(_29, 1), 0).0 as f32;
place!(Field::<(usize,)>(Variant(RET, 1), 1)) = (_18.0,);
_13 = -Field::<isize>(Variant(_5, 2), 2);
_38 = [659132872_u32,3957293317_u32,2080873044_u32,1061181250_u32,3970774744_u32,3949922854_u32];
place!(Field::<(usize,)>(Variant(RET, 1), 1)) = (_18.0,);
_3.1.1 = _8;
_26 = _33;
SetDiscriminant(_29, 1);
_15 = (_16.0, _2);
_29 = Adt42::Variant2 { fld0: _26,fld1: Field::<u16>(Variant(_5, 2), 1),fld2: _25,fld3: _36,fld4: _38,fld5: Field::<f64>(Variant(_5, 2), 5),fld6: Field::<(isize,)>(Variant(_5, 2), 6) };
SetDiscriminant(_5, 2);
place!(Field::<[u32; 6]>(Variant(_5, 2), 4)) = Field::<[u32; 6]>(Variant(_29, 2), 4);
_19 = (Field::<*const f64>(Variant(RET, 1), 2), Field::<[i64; 2]>(Variant(RET, 1), 0));
Goto(bb17)
}
bb17 = {
Call(_42 = dump_var(13_usize, 15_usize, Move(_15), 14_usize, Move(_14), 12_usize, Move(_12), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(13_usize, 33_usize, Move(_33), 10_usize, Move(_10), 34_usize, Move(_34), 2_usize, Move(_2)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_42 = dump_var(13_usize, 25_usize, Move(_25), 17_usize, Move(_17), 18_usize, Move(_18), 31_usize, Move(_31)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_42 = dump_var(13_usize, 8_usize, Move(_8), 36_usize, Move(_36), 43_usize, _43, 43_usize, _43), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: [i32; 8],mut _2: usize,mut _3: [u64; 7],mut _4: [u64; 7],mut _5: [i32; 8],mut _6: bool,mut _7: [i32; 8],mut _8: u64,mut _9: i32,mut _10: u64,mut _11: i32,mut _12: (isize,),mut _13: [u64; 7],mut _14: i64,mut _15: char,mut _16: i32) -> isize {
mir! {
type RET = isize;
let _17: i128;
let _18: Adt51;
let _19: [u64; 7];
let _20: u16;
let _21: Adt45;
let _22: *mut i8;
let _23: [u128; 8];
let _24: (u64, u64);
let _25: Adt47;
let _26: [u64; 7];
let _27: (i8, u128);
let _28: [u32; 6];
let _29: Adt47;
let _30: f64;
let _31: usize;
let _32: (usize,);
let _33: bool;
let _34: u32;
let _35: *const f64;
let _36: isize;
let _37: Adt42;
let _38: [u32; 6];
let _39: i64;
let _40: f32;
let _41: usize;
let _42: ();
let _43: ();
{
RET = (-48_i8) as isize;
_14 = (-8636053418198003774_i64);
_1 = _5;
_9 = _1[_2];
_1[_2] = !_9;
_5 = [_9,_1[_2],_9,_16,_7[_2],_9,_9,_16];
RET = 77_u8 as isize;
_3[_2] = !_8;
_13 = [_3[_2],_3[_2],_4[_2],_8,_10,_8,_3[_2]];
_15 = '\u{2470e}';
_3 = _13;
_4 = _3;
_9 = _1[_2] & _16;
RET = 61484379285295516647241537772060579047_i128 as isize;
_7 = [_11,_9,_9,_9,_11,_9,_9,_11];
RET = -_12.0;
Goto(bb1)
}
bb1 = {
_10 = !_8;
_13[_2] = _6 as u64;
_7[_2] = !_16;
_13 = _4;
_5[_2] = _16;
_3 = [_4[_2],_10,_4[_2],_4[_2],_4[_2],_10,_10];
_15 = '\u{dee71}';
_3[_2] = _4[_2] * _8;
_7 = _1;
_7[_2] = _9 | _9;
_14 = (-2360312734583004721_i64);
_4 = [_3[_2],_10,_3[_2],_3[_2],_3[_2],_13[_2],_8];
_6 = !true;
_4[_2] = _8 | _10;
_14 = 9143908629462121266_i64;
_19[_2] = _13[_2];
_6 = false;
_2 = 3_usize * 3_usize;
_13 = [_8,_8,_8,_10,_8,_10,_10];
_19 = _4;
Goto(bb2)
}
bb2 = {
_8 = !_10;
_5 = [_9,_11,_9,_9,_9,_11,_11,_9];
RET = _12.0 * _12.0;
_3 = [_10,_10,_10,_10,_10,_10,_8];
_16 = _9;
_1 = [_9,_9,_9,_11,_9,_11,_9,_9];
_12.0 = RET & RET;
_4 = _3;
_11 = 15868_u16 as i32;
_12 = (RET,);
_17 = (-52527038444439932219523132078092815914_i128) | (-100986862076141492294770664765029779484_i128);
_16 = _9;
_13 = [_8,_10,_10,_10,_8,_8,_8];
_7 = [_9,_9,_16,_16,_9,_9,_9,_16];
_7 = [_9,_9,_16,_16,_9,_9,_16,_9];
_20 = (-99_i8) as u16;
_16 = _12.0 as i32;
_17 = !106556622083981901596085116038208424734_i128;
_12.0 = -RET;
_13 = _19;
_1 = [_9,_9,_16,_11,_9,_9,_11,_11];
Goto(bb3)
}
bb3 = {
RET = _12.0 & _12.0;
_10 = _8;
_5 = [_9,_9,_9,_9,_9,_16,_11,_9];
_2 = 5209332515825262814_usize;
_3 = _4;
_9 = -_16;
_23 = [50414383863896252952905084562492556956_u128,106850920554330172237460827835043166277_u128,264149333117436855621098123027800105105_u128,47595423590687856685033180171820354713_u128,281370475172269985295779667970759554457_u128,129396430353453192156359789599365717161_u128,331869042522828550587818793233472192837_u128,235982395364455240815458868233491882819_u128];
_5 = [_16,_9,_9,_9,_9,_9,_9,_9];
RET = _12.0 | _12.0;
_14 = (-3048107227864033488_i64) ^ 5059803589334577861_i64;
_19 = [_8,_10,_8,_8,_8,_8,_8];
_9 = _11 * _11;
_24.1 = _10;
_24 = (_10, _8);
_24 = (_10, _10);
_8 = (-23488_i16) as u64;
_24.0 = _17 as u64;
_23 = [141605618266495552969071818071021784944_u128,166123052455983660122621929362949932157_u128,288331717635504097441932882974596496503_u128,287759591244359706715144251217624720752_u128,107771016290972869076635131975488752940_u128,48371483334029682814201860338657202741_u128,85061490841910400047075141357398218332_u128,167740908403766689253880469024681749293_u128];
Goto(bb4)
}
bb4 = {
RET = !_12.0;
_26 = _4;
_8 = RET as u64;
_17 = (-135002916337408123397987489914958501100_i128);
_24.1 = _10;
_12.0 = 113_u8 as isize;
_22 = core::ptr::addr_of_mut!(_27.0);
_27.0 = (-55_i8);
_5 = _1;
_9 = !_16;
(*_22) = -(-67_i8);
_27 = (55_i8, 80002090196361763270116383009403564154_u128);
_5 = [_9,_16,_9,_11,_16,_9,_9,_9];
_23 = [_27.1,_27.1,_27.1,_27.1,_27.1,_27.1,_27.1,_27.1];
_24.0 = _10 + _24.1;
_5 = [_9,_11,_16,_9,_9,_16,_16,_11];
_22 = core::ptr::addr_of_mut!(_27.0);
_7 = [_9,_16,_11,_9,_11,_9,_9,_16];
_10 = _24.0;
_29 = Adt47::Variant0 { fld0: _6,fld1: _22 };
_26 = [_24.0,_24.0,_24.1,_10,_10,_10,_24.1];
_23 = [_27.1,_27.1,_27.1,_27.1,_27.1,_27.1,_27.1,_27.1];
_14 = (-4493981719830685720_i64);
(*_22) = (-33_i8);
SetDiscriminant(_29, 0);
match _2 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
5209332515825262814 => bb10,
_ => bb9
}
}
bb5 = {
RET = _12.0 & _12.0;
_10 = _8;
_5 = [_9,_9,_9,_9,_9,_16,_11,_9];
_2 = 5209332515825262814_usize;
_3 = _4;
_9 = -_16;
_23 = [50414383863896252952905084562492556956_u128,106850920554330172237460827835043166277_u128,264149333117436855621098123027800105105_u128,47595423590687856685033180171820354713_u128,281370475172269985295779667970759554457_u128,129396430353453192156359789599365717161_u128,331869042522828550587818793233472192837_u128,235982395364455240815458868233491882819_u128];
_5 = [_16,_9,_9,_9,_9,_9,_9,_9];
RET = _12.0 | _12.0;
_14 = (-3048107227864033488_i64) ^ 5059803589334577861_i64;
_19 = [_8,_10,_8,_8,_8,_8,_8];
_9 = _11 * _11;
_24.1 = _10;
_24 = (_10, _8);
_24 = (_10, _10);
_8 = (-23488_i16) as u64;
_24.0 = _17 as u64;
_23 = [141605618266495552969071818071021784944_u128,166123052455983660122621929362949932157_u128,288331717635504097441932882974596496503_u128,287759591244359706715144251217624720752_u128,107771016290972869076635131975488752940_u128,48371483334029682814201860338657202741_u128,85061490841910400047075141357398218332_u128,167740908403766689253880469024681749293_u128];
Goto(bb4)
}
bb6 = {
_8 = !_10;
_5 = [_9,_11,_9,_9,_9,_11,_11,_9];
RET = _12.0 * _12.0;
_3 = [_10,_10,_10,_10,_10,_10,_8];
_16 = _9;
_1 = [_9,_9,_9,_11,_9,_11,_9,_9];
_12.0 = RET & RET;
_4 = _3;
_11 = 15868_u16 as i32;
_12 = (RET,);
_17 = (-52527038444439932219523132078092815914_i128) | (-100986862076141492294770664765029779484_i128);
_16 = _9;
_13 = [_8,_10,_10,_10,_8,_8,_8];
_7 = [_9,_9,_16,_16,_9,_9,_9,_16];
_7 = [_9,_9,_16,_16,_9,_9,_16,_9];
_20 = (-99_i8) as u16;
_16 = _12.0 as i32;
_17 = !106556622083981901596085116038208424734_i128;
_12.0 = -RET;
_13 = _19;
_1 = [_9,_9,_16,_11,_9,_9,_11,_11];
Goto(bb3)
}
bb7 = {
_10 = !_8;
_13[_2] = _6 as u64;
_7[_2] = !_16;
_13 = _4;
_5[_2] = _16;
_3 = [_4[_2],_10,_4[_2],_4[_2],_4[_2],_10,_10];
_15 = '\u{dee71}';
_3[_2] = _4[_2] * _8;
_7 = _1;
_7[_2] = _9 | _9;
_14 = (-2360312734583004721_i64);
_4 = [_3[_2],_10,_3[_2],_3[_2],_3[_2],_13[_2],_8];
_6 = !true;
_4[_2] = _8 | _10;
_14 = 9143908629462121266_i64;
_19[_2] = _13[_2];
_6 = false;
_2 = 3_usize * 3_usize;
_13 = [_8,_8,_8,_10,_8,_10,_10];
_19 = _4;
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_11 = _14 as i32;
_17 = (-104634030008271223154904836313251890210_i128);
_7 = _1;
place!(Field::<*mut i8>(Variant(_29, 0), 1)) = _22;
_27 = ((-128_i8), 15432449814412617884946025467378859186_u128);
_2 = 3_usize - 1515740511018750926_usize;
place!(Field::<bool>(Variant(_29, 0), 0)) = !_6;
_17 = 30018647749080803242273106726646803577_i128 - (-69210494372843711080580508596037180998_i128);
_3 = [_24.1,_24.0,_24.1,_10,_10,_24.1,_24.0];
_30 = _27.1 as f64;
_24.1 = _24.0;
_32 = (_2,);
_32 = (_2,);
_34 = _27.0 as u32;
_30 = _34 as f64;
_32 = (_2,);
_12.0 = _17 as isize;
match (*_22) {
0 => bb11,
340282366920938463463374607431768211328 => bb13,
_ => bb12
}
}
bb11 = {
_10 = !_8;
_13[_2] = _6 as u64;
_7[_2] = !_16;
_13 = _4;
_5[_2] = _16;
_3 = [_4[_2],_10,_4[_2],_4[_2],_4[_2],_10,_10];
_15 = '\u{dee71}';
_3[_2] = _4[_2] * _8;
_7 = _1;
_7[_2] = _9 | _9;
_14 = (-2360312734583004721_i64);
_4 = [_3[_2],_10,_3[_2],_3[_2],_3[_2],_13[_2],_8];
_6 = !true;
_4[_2] = _8 | _10;
_14 = 9143908629462121266_i64;
_19[_2] = _13[_2];
_6 = false;
_2 = 3_usize * 3_usize;
_13 = [_8,_8,_8,_10,_8,_10,_10];
_19 = _4;
Goto(bb2)
}
bb12 = {
RET = _12.0 & _12.0;
_10 = _8;
_5 = [_9,_9,_9,_9,_9,_16,_11,_9];
_2 = 5209332515825262814_usize;
_3 = _4;
_9 = -_16;
_23 = [50414383863896252952905084562492556956_u128,106850920554330172237460827835043166277_u128,264149333117436855621098123027800105105_u128,47595423590687856685033180171820354713_u128,281370475172269985295779667970759554457_u128,129396430353453192156359789599365717161_u128,331869042522828550587818793233472192837_u128,235982395364455240815458868233491882819_u128];
_5 = [_16,_9,_9,_9,_9,_9,_9,_9];
RET = _12.0 | _12.0;
_14 = (-3048107227864033488_i64) ^ 5059803589334577861_i64;
_19 = [_8,_10,_8,_8,_8,_8,_8];
_9 = _11 * _11;
_24.1 = _10;
_24 = (_10, _8);
_24 = (_10, _10);
_8 = (-23488_i16) as u64;
_24.0 = _17 as u64;
_23 = [141605618266495552969071818071021784944_u128,166123052455983660122621929362949932157_u128,288331717635504097441932882974596496503_u128,287759591244359706715144251217624720752_u128,107771016290972869076635131975488752940_u128,48371483334029682814201860338657202741_u128,85061490841910400047075141357398218332_u128,167740908403766689253880469024681749293_u128];
Goto(bb4)
}
bb13 = {
SetDiscriminant(_29, 1);
_22 = core::ptr::addr_of_mut!(_27.0);
place!(Field::<(u64, u64)>(Variant(_29, 1), 4)) = _24;
_17 = RET as i128;
(*_22) = 8_i8;
place!(Field::<(u64, u64)>(Variant(_29, 1), 4)) = (_24.0, _24.1);
place!(Field::<[u32; 5]>(Variant(_29, 1), 6)) = [_34,_34,_34,_34,_34];
_25 = Adt47::Variant0 { fld0: _6,fld1: _22 };
RET = _12.0;
_2 = (-25381_i16) as usize;
_22 = Field::<*mut i8>(Variant(_25, 0), 1);
place!(Field::<(usize,)>(Variant(_29, 1), 2)).0 = _32.0 | _32.0;
_19 = _13;
_1 = [_9,_9,_16,_16,_9,_11,_9,_11];
_2 = !_32.0;
place!(Field::<(u64, u64)>(Variant(_29, 1), 4)) = _24;
_6 = Field::<bool>(Variant(_25, 0), 0) ^ Field::<bool>(Variant(_25, 0), 0);
place!(Field::<(*const f64, [i64; 2])>(Variant(_29, 1), 3)).1 = [_14,_14];
_32.0 = !Field::<(usize,)>(Variant(_29, 1), 2).0;
SetDiscriminant(_25, 1);
(*_22) = -28_i8;
place!(Field::<bool>(Variant(_29, 1), 0)) = _6;
_36 = _12.0 + _12.0;
place!(Field::<bool>(Variant(_29, 1), 0)) = _6 & _6;
place!(Field::<(u64, u64)>(Variant(_25, 1), 4)).1 = _27.0 as u64;
place!(Field::<[u32; 5]>(Variant(_25, 1), 6)) = [_34,_34,_34,_34,_34];
_28 = [_34,_34,_34,_34,_34,_34];
match _27.1 {
0 => bb12,
1 => bb7,
2 => bb11,
3 => bb14,
15432449814412617884946025467378859186 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
_10 = !_8;
_13[_2] = _6 as u64;
_7[_2] = !_16;
_13 = _4;
_5[_2] = _16;
_3 = [_4[_2],_10,_4[_2],_4[_2],_4[_2],_10,_10];
_15 = '\u{dee71}';
_3[_2] = _4[_2] * _8;
_7 = _1;
_7[_2] = _9 | _9;
_14 = (-2360312734583004721_i64);
_4 = [_3[_2],_10,_3[_2],_3[_2],_3[_2],_13[_2],_8];
_6 = !true;
_4[_2] = _8 | _10;
_14 = 9143908629462121266_i64;
_19[_2] = _13[_2];
_6 = false;
_2 = 3_usize * 3_usize;
_13 = [_8,_8,_8,_10,_8,_10,_10];
_19 = _4;
Goto(bb2)
}
bb16 = {
_31 = _32.0;
place!(Field::<u32>(Variant(_29, 1), 1)) = (*_22) as u32;
place!(Field::<(u64, u64)>(Variant(_29, 1), 4)).0 = _24.0 ^ _24.1;
place!(Field::<(u64, u64)>(Variant(_25, 1), 4)) = (Field::<(u64, u64)>(Variant(_29, 1), 4).0, _24.0);
_1 = [_9,_9,_11,_16,_16,_11,_11,_9];
_40 = _20 as f32;
place!(Field::<(u64, u64)>(Variant(_25, 1), 4)).1 = Field::<(u64, u64)>(Variant(_29, 1), 4).0;
_8 = Field::<(u64, u64)>(Variant(_25, 1), 4).0;
_4 = [_24.0,_24.0,Field::<(u64, u64)>(Variant(_25, 1), 4).0,Field::<(u64, u64)>(Variant(_29, 1), 4).0,_8,_24.0,Field::<(u64, u64)>(Variant(_25, 1), 4).1];
_7 = [_9,_11,_16,_9,_16,_9,_11,_9];
_11 = _16;
place!(Field::<u32>(Variant(_29, 1), 1)) = !_34;
place!(Field::<(usize,)>(Variant(_25, 1), 2)) = _32;
_32 = (_31,);
Goto(bb17)
}
bb17 = {
Call(_42 = dump_var(14_usize, 13_usize, Move(_13), 8_usize, Move(_8), 32_usize, Move(_32), 1_usize, Move(_1)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_42 = dump_var(14_usize, 34_usize, Move(_34), 27_usize, Move(_27), 11_usize, Move(_11), 31_usize, Move(_31)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_42 = dump_var(14_usize, 7_usize, Move(_7), 15_usize, Move(_15), 2_usize, Move(_2), 20_usize, Move(_20)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_42 = dump_var(14_usize, 12_usize, Move(_12), 4_usize, Move(_4), 43_usize, _43, 43_usize, _43), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: u128,mut _2: isize,mut _3: u128,mut _4: [u32; 1],mut _5: u16,mut _6: u64,mut _7: u128,mut _8: u128,mut _9: u16,mut _10: u128,mut _11: u128) -> u16 {
mir! {
type RET = u16;
let _12: i128;
let _13: [u32; 6];
let _14: bool;
let _15: f64;
let _16: i16;
let _17: i32;
let _18: i128;
let _19: isize;
let _20: [u32; 6];
let _21: &'static [u8; 3];
let _22: char;
let _23: (*const f64, [i64; 2]);
let _24: Adt46;
let _25: ([i32; 8], char);
let _26: (u64, (i8, u128));
let _27: usize;
let _28: f64;
let _29: bool;
let _30: char;
let _31: bool;
let _32: f32;
let _33: bool;
let _34: i16;
let _35: ();
let _36: ();
{
_6 = 2410839191956733629_u64;
_3 = _8;
_6 = 150_u8 as u64;
_3 = _1;
_10 = 22552_i16 as u128;
_1 = !_7;
_4 = [3758496368_u32];
_11 = _8 & _1;
RET = _9 >> _11;
_4 = [891397919_u32];
_9 = !RET;
_13 = [1847389853_u32,1259818009_u32,2109416137_u32,966298026_u32,206548512_u32,2449902645_u32];
_4 = [3044867401_u32];
_9 = RET;
_13 = [2010092834_u32,590357282_u32,2698843111_u32,760375391_u32,3703294272_u32,2875106688_u32];
_2 = 9223372036854775807_isize;
RET = _9;
_2 = -9223372036854775807_isize;
_12 = 82004378203191084893289615799984859294_i128;
_15 = 121_u8 as f64;
match _12 {
0 => bb1,
1 => bb2,
2 => bb3,
82004378203191084893289615799984859294 => bb5,
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
_3 = 207_u8 as u128;
match _12 {
0 => bb1,
1 => bb6,
82004378203191084893289615799984859294 => bb8,
_ => bb7
}
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
_12 = (-99432300513686032006776259312440597804_i128) * 56846369790167999998671895925022507361_i128;
_16 = -(-5501_i16);
RET = 792638849_i32 as u16;
_13 = [1509491575_u32,3031064272_u32,1602035032_u32,1673929738_u32,2729168662_u32,965550569_u32];
_12 = 263982377_i32 as i128;
_17 = (-297985007_i32) - (-1768420922_i32);
_2 = _15 as isize;
_4 = [2701723490_u32];
_5 = !RET;
_1 = _6 as u128;
_14 = _9 > _9;
_8 = _7;
_4 = [2147354743_u32];
_18 = _12;
_17 = (-188882958_i32);
_12 = _18 & _18;
RET = _2 as u16;
_17 = (-1046301177_i32);
_10 = _7;
_14 = !true;
_16 = 16550_i16;
_18 = _12;
_14 = false;
Call(_3 = fn16(_16, _17, _9, _13, _9, _13, _10, _11, _13, _9, _9, _8), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_3 = _1 >> _12;
_10 = _7;
RET = !_9;
RET = _9;
_20 = [4146685429_u32,1922337615_u32,2884143531_u32,155674943_u32,538914176_u32,226120043_u32];
_7 = _11 >> _11;
_2 = -(-9223372036854775808_isize);
_19 = _15 as isize;
_3 = _7 - _7;
_17 = (-980337753_i32) - 1577983404_i32;
_16 = -(-32479_i16);
_19 = _17 as isize;
_10 = _1 & _7;
RET = _17 as u16;
_3 = '\u{9e423}' as u128;
_22 = '\u{adac6}';
_8 = _7 >> _9;
_19 = _9 as isize;
_22 = '\u{1fe8f}';
_6 = !4680637159480108975_u64;
Goto(bb10)
}
bb10 = {
_17 = 5545669913014003565_i64 as i32;
_4 = [42409536_u32];
_1 = !_11;
_17 = !(-1333925420_i32);
_5 = RET;
_12 = _14 as i128;
_18 = _9 as i128;
_22 = '\u{1476d}';
Goto(bb11)
}
bb11 = {
_13 = [3141489383_u32,3007711194_u32,299311492_u32,2861944639_u32,2767434704_u32,1903606968_u32];
_25.1 = _22;
_10 = _7 * _7;
_26.1.0 = _17 as i8;
_26.0 = _6 - _6;
Goto(bb12)
}
bb12 = {
_26.1.1 = !_8;
_1 = _26.1.1 ^ _26.1.1;
_12 = RET as i128;
_25.0 = [_17,_17,_17,_17,_17,_17,_17,_17];
_7 = _1;
_11 = !_7;
_5 = _26.1.1 as u16;
_17 = -(-1127442233_i32);
_16 = !2339_i16;
_23.1 = [(-5419472416359389758_i64),(-2417560704598699468_i64)];
_14 = false ^ true;
_22 = _25.1;
_30 = _25.1;
_5 = RET;
Goto(bb13)
}
bb13 = {
_29 = _1 < _1;
_26.1 = ((-6_i8), _7);
_2 = 5436268214464758543_i64 as isize;
_3 = _26.1.0 as u128;
_26.0 = _6;
Goto(bb14)
}
bb14 = {
_29 = !_14;
_19 = -_2;
_11 = _15 as u128;
_19 = _2 | _2;
_20 = [795811556_u32,1615767985_u32,778238537_u32,1401296410_u32,950875777_u32,3498937013_u32];
_26.1.1 = _18 as u128;
_31 = !_14;
_27 = _26.1.0 as usize;
_1 = _3;
_23.0 = core::ptr::addr_of!(_28);
_7 = _1 ^ _1;
_23.1 = [(-8931334761891275364_i64),3975143854527270621_i64];
_26.1 = ((-54_i8), _1);
_26.1.1 = _15 as u128;
_30 = _22;
_28 = _9 as f64;
_33 = !_29;
_4 = [1542943094_u32];
_32 = _1 as f32;
_26.0 = _6 ^ _6;
_27 = 6483730249402315058_usize * 0_usize;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(15_usize, 11_usize, Move(_11), 13_usize, Move(_13), 27_usize, Move(_27), 29_usize, Move(_29)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(15_usize, 26_usize, Move(_26), 2_usize, Move(_2), 25_usize, Move(_25), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(15_usize, 12_usize, Move(_12), 5_usize, Move(_5), 14_usize, Move(_14), 20_usize, Move(_20)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_35 = dump_var(15_usize, 10_usize, Move(_10), 36_usize, _36, 36_usize, _36, 36_usize, _36), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: i16,mut _2: i32,mut _3: u16,mut _4: [u32; 6],mut _5: u16,mut _6: [u32; 6],mut _7: u128,mut _8: u128,mut _9: [u32; 6],mut _10: u16,mut _11: u16,mut _12: u128) -> u128 {
mir! {
type RET = u128;
let _13: Adt46;
let _14: (u64, u64);
let _15: [usize; 1];
let _16: bool;
let _17: i64;
let _18: (i8, u128);
let _19: isize;
let _20: char;
let _21: *const [usize; 1];
let _22: [u128; 8];
let _23: Adt42;
let _24: f32;
let _25: Adt54;
let _26: isize;
let _27: f64;
let _28: (isize,);
let _29: f64;
let _30: (usize,);
let _31: ([u64; 7], char);
let _32: f32;
let _33: bool;
let _34: u128;
let _35: u32;
let _36: [u32; 6];
let _37: [u8; 3];
let _38: (u64, (i8, u128));
let _39: i8;
let _40: i64;
let _41: i8;
let _42: (i64, i128, *mut i8);
let _43: Adt45;
let _44: ();
let _45: ();
{
RET = '\u{26e7f}' as u128;
_11 = '\u{7ab6f}' as u16;
_11 = !_5;
_8 = 2508204832434934890_i64 as u128;
RET = 3890595499406267677_i64 as u128;
_4 = _9;
_14.1 = '\u{16d37}' as u64;
_11 = !_5;
_11 = 146_u8 as u16;
_6 = [2015304142_u32,1860126409_u32,3613300246_u32,90574102_u32,1076281544_u32,185442168_u32];
_3 = _5;
_12 = _7;
_3 = _5;
Goto(bb1)
}
bb1 = {
_8 = !_7;
_14.1 = 15949371897521179302_u64;
_16 = !false;
_7 = _8;
_2 = _14.1 as i32;
_9 = [2628610918_u32,1146430527_u32,3609326318_u32,1282544972_u32,1077517035_u32,3272299563_u32];
_6 = [936439424_u32,1141251707_u32,1375082366_u32,483124874_u32,2203681929_u32,418556209_u32];
_14 = (5708237935718887813_u64, 947496560487010951_u64);
_12 = _8;
_14 = (7040558105174052963_u64, 5397250706056271803_u64);
_15 = [6_usize];
RET = _12 - _7;
_6 = _9;
_7 = _14.1 as u128;
_9 = [3168466459_u32,1963866628_u32,583036266_u32,3458415223_u32,3193429031_u32,2831483625_u32];
_2 = !(-110935946_i32);
_17 = 7257426083275419002_i64;
_5 = !_3;
_18.0 = 54_i8 + (-128_i8);
_17 = _18.0 as i64;
_18.1 = RET;
_4 = _9;
_12 = (-151849976816006874790676523413413753395_i128) as u128;
_18.1 = _7 - _8;
_3 = _5 | _10;
Call(_11 = fn17(_10, _12, _5, _3, _3, _14.1, _5, _18, _3, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_18 = ((-55_i8), _8);
_5 = _16 as u16;
_8 = !_12;
_5 = _11 - _11;
_2 = (-1601529152_i32) * 1838776505_i32;
_5 = _10;
_12 = _16 as u128;
_1 = _2 as i16;
_15 = [17877530934412929685_usize];
_18.1 = !RET;
RET = _1 as u128;
_15 = [2_usize];
_11 = !_5;
_12 = !_7;
_18 = (92_i8, RET);
match _14.1 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
5397250706056271803 => bb9,
_ => bb8
}
}
bb3 = {
_8 = !_7;
_14.1 = 15949371897521179302_u64;
_16 = !false;
_7 = _8;
_2 = _14.1 as i32;
_9 = [2628610918_u32,1146430527_u32,3609326318_u32,1282544972_u32,1077517035_u32,3272299563_u32];
_6 = [936439424_u32,1141251707_u32,1375082366_u32,483124874_u32,2203681929_u32,418556209_u32];
_14 = (5708237935718887813_u64, 947496560487010951_u64);
_12 = _8;
_14 = (7040558105174052963_u64, 5397250706056271803_u64);
_15 = [6_usize];
RET = _12 - _7;
_6 = _9;
_7 = _14.1 as u128;
_9 = [3168466459_u32,1963866628_u32,583036266_u32,3458415223_u32,3193429031_u32,2831483625_u32];
_2 = !(-110935946_i32);
_17 = 7257426083275419002_i64;
_5 = !_3;
_18.0 = 54_i8 + (-128_i8);
_17 = _18.0 as i64;
_18.1 = RET;
_4 = _9;
_12 = (-151849976816006874790676523413413753395_i128) as u128;
_18.1 = _7 - _8;
_3 = _5 | _10;
Call(_11 = fn17(_10, _12, _5, _3, _3, _14.1, _5, _18, _3, RET), ReturnTo(bb2), UnwindUnreachable())
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
_19 = 9223372036854775807_isize << _11;
_14.1 = 1664417824_u32 as u64;
_14.0 = _14.1;
_4 = _6;
_15 = [1_usize];
_1 = -25005_i16;
_10 = _3;
_25.fld0 = core::ptr::addr_of!(_15);
_24 = _14.1 as f32;
_14 = (17112749820245946521_u64, 11263984152086790798_u64);
_8 = _12;
_17 = (-4243573392441903014_i64) & (-6362799250308816992_i64);
_5 = _11 & _10;
_14.0 = _14.1;
_21 = core::ptr::addr_of!(_15);
_25.fld1 = -_18.0;
_6 = _9;
_6 = [651196853_u32,2651815085_u32,2186507724_u32,1121829715_u32,359736966_u32,4115559250_u32];
_28 = (_19,);
_6 = _9;
Goto(bb10)
}
bb10 = {
_30.0 = 38243629055153437626636318617787613258_i128 as usize;
_27 = 4152344168_u32 as f64;
_20 = '\u{ddcd7}';
_15 = [_30.0];
_26 = -_19;
RET = _18.1;
_19 = _28.0 | _26;
_21 = _25.fld0;
(*_21) = [_30.0];
_18.0 = _25.fld1 >> _30.0;
_25 = Adt54 { fld0: _21,fld1: _18.0 };
_10 = !_3;
_19 = _14.0 as isize;
_32 = -_24;
_18 = (_25.fld1, RET);
_29 = _27;
_33 = _16;
_18 = (_25.fld1, _12);
_4 = [2573097211_u32,1012584960_u32,2083026332_u32,256708934_u32,3334100183_u32,1712823638_u32];
_10 = _3 & _5;
_18 = (_25.fld1, _8);
Goto(bb11)
}
bb11 = {
_15 = [_30.0];
Goto(bb12)
}
bb12 = {
_28 = (_26,);
_28 = (_19,);
_25 = Adt54 { fld0: _21,fld1: _18.0 };
_36 = _6;
_34 = !_12;
_21 = core::ptr::addr_of!((*_21));
_35 = 3735378944_u32 ^ 1002906753_u32;
_38.1.1 = !_12;
_18.0 = _33 as i8;
_25.fld0 = core::ptr::addr_of!((*_21));
_18 = (_25.fld1, _8);
_35 = !1430086067_u32;
RET = _34 - _12;
_40 = _20 as i64;
_42.0 = _17;
match _14.0 {
0 => bb13,
1 => bb14,
2 => bb15,
11263984152086790798 => bb17,
_ => bb16
}
}
bb13 = {
_15 = [_30.0];
Goto(bb12)
}
bb14 = {
_8 = !_7;
_14.1 = 15949371897521179302_u64;
_16 = !false;
_7 = _8;
_2 = _14.1 as i32;
_9 = [2628610918_u32,1146430527_u32,3609326318_u32,1282544972_u32,1077517035_u32,3272299563_u32];
_6 = [936439424_u32,1141251707_u32,1375082366_u32,483124874_u32,2203681929_u32,418556209_u32];
_14 = (5708237935718887813_u64, 947496560487010951_u64);
_12 = _8;
_14 = (7040558105174052963_u64, 5397250706056271803_u64);
_15 = [6_usize];
RET = _12 - _7;
_6 = _9;
_7 = _14.1 as u128;
_9 = [3168466459_u32,1963866628_u32,583036266_u32,3458415223_u32,3193429031_u32,2831483625_u32];
_2 = !(-110935946_i32);
_17 = 7257426083275419002_i64;
_5 = !_3;
_18.0 = 54_i8 + (-128_i8);
_17 = _18.0 as i64;
_18.1 = RET;
_4 = _9;
_12 = (-151849976816006874790676523413413753395_i128) as u128;
_18.1 = _7 - _8;
_3 = _5 | _10;
Call(_11 = fn17(_10, _12, _5, _3, _3, _14.1, _5, _18, _3, RET), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
_19 = 9223372036854775807_isize << _11;
_14.1 = 1664417824_u32 as u64;
_14.0 = _14.1;
_4 = _6;
_15 = [1_usize];
_1 = -25005_i16;
_10 = _3;
_25.fld0 = core::ptr::addr_of!(_15);
_24 = _14.1 as f32;
_14 = (17112749820245946521_u64, 11263984152086790798_u64);
_8 = _12;
_17 = (-4243573392441903014_i64) & (-6362799250308816992_i64);
_5 = _11 & _10;
_14.0 = _14.1;
_21 = core::ptr::addr_of!(_15);
_25.fld1 = -_18.0;
_6 = _9;
_6 = [651196853_u32,2651815085_u32,2186507724_u32,1121829715_u32,359736966_u32,4115559250_u32];
_28 = (_19,);
_6 = _9;
Goto(bb10)
}
bb16 = {
Return()
}
bb17 = {
_38.1.1 = _26 as u128;
Goto(bb18)
}
bb18 = {
Call(_44 = dump_var(16_usize, 9_usize, Move(_9), 26_usize, Move(_26), 30_usize, Move(_30), 14_usize, Move(_14)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_44 = dump_var(16_usize, 15_usize, Move(_15), 33_usize, Move(_33), 8_usize, Move(_8), 20_usize, Move(_20)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_44 = dump_var(16_usize, 40_usize, Move(_40), 12_usize, Move(_12), 35_usize, Move(_35), 4_usize, Move(_4)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_44 = dump_var(16_usize, 2_usize, Move(_2), 45_usize, _45, 45_usize, _45, 45_usize, _45), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: u16,mut _2: u128,mut _3: u16,mut _4: u16,mut _5: u16,mut _6: u64,mut _7: u16,mut _8: (i8, u128),mut _9: u16,mut _10: u128) -> u16 {
mir! {
type RET = u16;
let _11: (isize,);
let _12: Adt55;
let _13: *const [u32; 1];
let _14: (*const f64, [i64; 2]);
let _15: (isize,);
let _16: [u32; 5];
let _17: [u32; 1];
let _18: f64;
let _19: Adt42;
let _20: ();
let _21: ();
{
_9 = _6 as u16;
_2 = _8.1 * _10;
_8.0 = 51_i8;
_8.0 = (-5721258993663300249_i64) as i8;
_7 = 13837625820257695825_usize as u16;
_9 = _5 | _5;
RET = !_4;
_4 = _9 << _2;
_3 = _1 - RET;
_3 = _4;
_11.0 = (-9223372036854775808_isize) << _10;
_8.1 = !_10;
_4 = _3;
_5 = 14924681383106911630_usize as u16;
_11.0 = _2 as isize;
_8.0 = !(-123_i8);
_8 = (13_i8, _2);
_10 = _8.1 - _2;
_2 = !_10;
_6 = 4450486833163700530_u64;
_9 = _3;
RET = _5;
_9 = (-8623362085925771949_i64) as u16;
_4 = !_1;
Goto(bb1)
}
bb1 = {
_7 = !_3;
_14.1 = [5459290507629321081_i64,(-8624335646044862452_i64)];
_8 = ((-35_i8), _2);
_6 = !9022983099257362535_u64;
_8.0 = 70_i8 | 84_i8;
_8 = (50_i8, _2);
_6 = !8501534040031011496_u64;
RET = _3;
_14.1 = [7447340851924648141_i64,887437329630607209_i64];
_11 = (36_isize,);
_15 = (_11.0,);
_15.0 = _11.0 ^ _11.0;
_16 = [2170102231_u32,731855666_u32,421499141_u32,424954167_u32,2312415351_u32];
_18 = 476197717_i32 as f64;
_6 = 3944775752_u32 as u64;
_5 = _7;
Goto(bb2)
}
bb2 = {
Call(_20 = dump_var(17_usize, 10_usize, Move(_10), 15_usize, Move(_15), 8_usize, Move(_8), 5_usize, Move(_5)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_20 = dump_var(17_usize, 3_usize, Move(_3), 4_usize, Move(_4), 21_usize, _21, 21_usize, _21), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(52318_u16), std::hint::black_box('\u{3a00c}'), std::hint::black_box((-83_i8)));
                
            }
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: ((i64, i128, *mut i8), (u64, (i8, u128)), (i64, i128, *mut i8)),
fld1: u64,
fld2: isize,
fld3: i8,

},
Variant1{
fld0: (u64, u64),
fld1: u128,
fld2: f64,

},
Variant2{
fld0: [u64; 7],
fld1: u16,
fld2: isize,
fld3: i8,
fld4: [u32; 6],
fld5: f64,
fld6: (isize,),

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt43{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt43 {
fld0: (*const f64, [i64; 2]),
fld1: [i32; 8],
}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: bool,
fld1: *const [u32; 1],
fld2: isize,
fld3: [u32; 1],
fld4: u16,
fld5: u128,
fld6: u64,

},
Variant1{
fld0: [i64; 2],
fld1: (usize,),
fld2: *const f64,

},
Variant2{
fld0: u32,
fld1: i16,

},
Variant3{
fld0: *mut i8,
fld1: u8,
fld2: (i64, i128, *mut i8),
fld3: (u64, (i8, u128)),
fld4: (isize,),
fld5: [u32; 1],

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: [u32; 6],
fld1: u128,
fld2: u16,
fld3: [u32; 1],
fld4: usize,

},
Variant1{
fld0: Adt42,
fld1: u32,
fld2: u8,
fld3: [u8; 3],

},
Variant2{
fld0: i64,
fld1: u64,
fld2: [u64; 7],
fld3: Adt42,
fld4: (i8, u128),

},
Variant3{
fld0: *const [usize; 1],
fld1: (i64, i128, *mut i8),
fld2: (u64, u64),
fld3: i8,
fld4: [u64; 7],
fld5: u8,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: Adt43,
fld1: ([u64; 7], char),
fld2: [u32; 5],
fld3: (isize,),
fld4: i64,
fld5: *const [usize; 1],

},
Variant1{
fld0: (i8, u128),
fld1: usize,
fld2: (u64, (i8, u128)),

}}
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: bool,
fld1: *mut i8,

},
Variant1{
fld0: bool,
fld1: u32,
fld2: (usize,),
fld3: (*const f64, [i64; 2]),
fld4: (u64, u64),
fld5: Adt46,
fld6: [u32; 5],

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf("Adt48::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: [i32; 8],

},
Variant1{
fld0: bool,
fld1: f64,
fld2: (*const f64, [i64; 2]),
fld3: (u64, (i8, u128)),
fld4: (i64, i128, *mut i8),

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: *const [usize; 1],

},
Variant1{
fld0: *mut i8,
fld1: [i64; 2],
fld2: i128,
fld3: (u64, (i8, u128)),
fld4: (*const f64, [i64; 2]),

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: Adt47,
fld1: [u32; 5],
fld2: i32,
fld3: ([u64; 7], char),
fld4: *const [u32; 1],

},
Variant1{
fld0: *const [u32; 1],
fld1: Adt49,
fld2: u8,
fld3: Adt44,
fld4: f32,
fld5: Adt48,
fld6: *mut i8,

},
Variant2{
fld0: (i64, i128, *mut i8),
fld1: char,
fld2: isize,
fld3: Adt48,
fld4: i16,
fld5: (usize,),
fld6: u128,
fld7: Adt49,

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){unsafe{printf("Adt51::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt51 {
Variant0{
fld0: Adt50,
fld1: u32,
fld2: *mut i8,
fld3: i8,
fld4: i16,
fld5: u128,
fld6: Adt48,

},
Variant1{
fld0: (*const f64, [i64; 2]),
fld1: f64,

},
Variant2{
fld0: ((i64, i128, *mut i8), (u64, (i8, u128)), (i64, i128, *mut i8)),
fld1: [u64; 7],
fld2: [u32; 1],
fld3: [usize; 1],
fld4: Adt42,
fld5: ([i32; 8], char),

},
Variant3{
fld0: *mut i8,
fld1: Adt42,
fld2: *const f64,
fld3: u8,
fld4: i16,
fld5: Adt47,
fld6: [usize; 1],

}}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: Adt43,
fld1: u64,
fld2: [u8; 3],
fld3: u128,
fld4: Adt48,
fld5: Adt50,

},
Variant1{
fld0: bool,
fld1: [u8; 3],

},
Variant2{
fld0: Adt46,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: (i64, i128, *mut i8),
fld1: Adt47,
fld2: Adt48,
fld3: [u128; 8],
fld4: f32,

},
Variant1{
fld0: ([u64; 7], char),
fld1: Adt43,
fld2: isize,

},
Variant2{
fld0: u32,
fld1: Adt44,
fld2: isize,
fld3: u64,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt54{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt54 {
fld0: *const [usize; 1],
fld1: i8,
}
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: Adt47,
fld1: (isize,),
fld2: (u64, (i8, u128)),
fld3: (i64, i128, *mut i8),
fld4: i16,
fld5: [u128; 8],

},
Variant1{
fld0: (u64, u64),
fld1: *const [u32; 1],

},
Variant2{
fld0: bool,
fld1: *const f64,
fld2: (u64, (i8, u128)),
fld3: *const [usize; 1],
fld4: Adt50,
fld5: ([i32; 8], char),
fld6: f32,
fld7: Adt48,

},
Variant3{
fld0: Adt52,
fld1: *const f64,
fld2: Adt44,
fld3: Adt47,
fld4: Adt49,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: Adt50,
fld1: u128,
fld2: Adt47,
fld3: Adt45,
fld4: u64,
fld5: *const f64,
fld6: Adt44,

},
Variant1{
fld0: Adt45,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){unsafe{printf("Adt57::\0".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: ([u64; 7], char),
fld1: Adt46,
fld2: Adt45,
fld3: (isize,),
fld4: u32,
fld5: usize,
fld6: f64,
fld7: Adt50,

},
Variant1{
fld0: [u32; 6],
fld1: (u64, (i8, u128)),
fld2: Adt49,
fld3: i8,
fld4: f64,
fld5: [u64; 7],
fld6: Adt54,
fld7: i128,

},
Variant2{
fld0: bool,
fld1: Adt55,
fld2: [u32; 5],
fld3: f64,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){unsafe{printf("Adt58::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt58 {
Variant0{
fld0: [u8; 3],
fld1: i8,

},
Variant1{
fld0: (u64, (i8, u128)),
fld1: ([u64; 7], char),
fld2: Adt53,
fld3: u64,
fld4: f64,

}}

