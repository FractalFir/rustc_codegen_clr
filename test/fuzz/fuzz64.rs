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
pub fn fn0(mut _1: i64,mut _2: usize,mut _3: isize,mut _4: u16,mut _5: i128,mut _6: i32) -> (*const f64, char, [char; 4]) {
mir! {
type RET = (*const f64, char, [char; 4]);
let _7: (u8, i32, [u16; 7], u64);
let _8: bool;
let _9: char;
let _10: [u64; 1];
let _11: *const char;
let _12: (i64, i16, &'static u64);
let _13: [i8; 6];
let _14: ();
let _15: ();
{
_2 = 3153108520_u32 as usize;
RET.2 = ['\u{43285}','\u{664cc}','\u{34e08}','\u{bde39}'];
_1 = (-777478933935200723_i64) - (-4961782936459522794_i64);
_4 = 31927272302367184422383818775339011664_i128 as u16;
RET.1 = '\u{1c60a}';
RET.2 = [RET.1,RET.1,RET.1,RET.1];
_3 = 9223372036854775807_isize;
_1 = !(-4454341861580732439_i64);
RET.2 = [RET.1,RET.1,RET.1,RET.1];
_5 = (-1448460990_i32) as i128;
Goto(bb1)
}
bb1 = {
RET.1 = '\u{875ea}';
RET.1 = '\u{8db6d}';
_7.3 = 3506305698_u32 as u64;
_3 = 5_isize;
match _3 {
5 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_7.2 = [_4,_4,_4,_4,_4,_4,_4];
_1 = 4284741836142080724_i64 * (-238393588182110922_i64);
Call(RET.0 = fn1(_3, RET.1, _1, _3, RET.2, _2, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_7.3 = _3 as u64;
_7.0 = 183_u8 >> _5;
_7.0 = 254_u8;
_6 = true as i32;
RET.2 = [RET.1,RET.1,RET.1,RET.1];
RET.2 = [RET.1,RET.1,RET.1,RET.1];
RET.1 = '\u{108a5b}';
_1 = (-3280977532924148697_i64);
_5 = -75835104925516008729290443621095563136_i128;
_4 = 54199_u16;
_1 = 1770682504527332152_i64 - (-6043442918005858141_i64);
RET.1 = '\u{c6c2b}';
_1 = 8371319865063748987_i64 | 5268312493864643843_i64;
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
54199 => bb7,
_ => bb6
}
}
bb5 = {
_7.2 = [_4,_4,_4,_4,_4,_4,_4];
_1 = 4284741836142080724_i64 * (-238393588182110922_i64);
Call(RET.0 = fn1(_3, RET.1, _1, _3, RET.2, _2, _3), ReturnTo(bb4), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
_2 = 11292372103308970675_usize;
_8 = true | true;
_7.1 = _6 - _6;
_7.0 = RET.1 as u8;
_10 = [_7.3];
RET.2 = [RET.1,RET.1,RET.1,RET.1];
Goto(bb8)
}
bb8 = {
_5 = !(-123719117534695223470281882648369826572_i128);
_1 = _7.0 as i64;
_3 = (-9223372036854775808_isize);
_10 = [_7.3];
Goto(bb9)
}
bb9 = {
_1 = !(-4795508719869473465_i64);
_4 = !11492_u16;
_9 = RET.1;
_11 = core::ptr::addr_of!(_9);
_7.3 = 2975667667691085998_u64;
_8 = true;
_5 = (-3536168295027195278197635501030570868_i128) - 37055760359734415295967531804377877571_i128;
_6 = _7.0 as i32;
(*_11) = RET.1;
_10 = [_7.3];
_12.2 = &_7.3;
_5 = 17384244479891164145584989154638541611_i128;
_12.0 = _1;
_12.0 = _1;
_13 = [9_i8,(-113_i8),29_i8,16_i8,82_i8,(-45_i8)];
_7.2 = [_4,_4,_4,_4,_4,_4,_4];
_10 = [_7.3];
_8 = !false;
_4 = 84_i8 as u16;
(*_11) = RET.1;
_10 = [_7.3];
_11 = core::ptr::addr_of!((*_11));
_7.2 = [_4,_4,_4,_4,_4,_4,_4];
match _7.3 {
0 => bb10,
1 => bb11,
2 => bb12,
2975667667691085998 => bb14,
_ => bb13
}
}
bb10 = {
_5 = !(-123719117534695223470281882648369826572_i128);
_1 = _7.0 as i64;
_3 = (-9223372036854775808_isize);
_10 = [_7.3];
Goto(bb9)
}
bb11 = {
_2 = 11292372103308970675_usize;
_8 = true | true;
_7.1 = _6 - _6;
_7.0 = RET.1 as u8;
_10 = [_7.3];
RET.2 = [RET.1,RET.1,RET.1,RET.1];
Goto(bb8)
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_2 = !15579368676612267459_usize;
RET.2 = [(*_11),RET.1,(*_11),_9];
(*_11) = RET.1;
_7.0 = 228_u8;
_12.1 = 16317_i16 | (-25779_i16);
_8 = true;
Goto(bb15)
}
bb15 = {
Call(_14 = dump_var(0_usize, 4_usize, Move(_4), 6_usize, Move(_6), 7_usize, Move(_7), 8_usize, Move(_8)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_14 = dump_var(0_usize, 2_usize, Move(_2), 15_usize, _15, 15_usize, _15, 15_usize, _15), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: isize,mut _2: char,mut _3: i64,mut _4: isize,mut _5: [char; 4],mut _6: usize,mut _7: isize) -> *const f64 {
mir! {
type RET = *const f64;
let _8: i64;
let _9: isize;
let _10: [u128; 1];
let _11: usize;
let _12: bool;
let _13: (u64, i64);
let _14: isize;
let _15: isize;
let _16: isize;
let _17: f64;
let _18: char;
let _19: [u32; 2];
let _20: &'static *const Adt54;
let _21: u8;
let _22: u64;
let _23: isize;
let _24: *const char;
let _25: i16;
let _26: *mut bool;
let _27: i128;
let _28: u8;
let _29: f32;
let _30: usize;
let _31: u64;
let _32: i8;
let _33: [char; 4];
let _34: isize;
let _35: i128;
let _36: ();
let _37: ();
{
_5 = [_2,_2,_2,_2];
_1 = _4 * _7;
_8 = -_3;
_8 = (-72916271527268752856116072233457128999_i128) as i64;
_9 = _4;
match _4 {
0 => bb1,
1 => bb2,
5 => bb4,
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
_2 = '\u{fc7a0}';
_5 = [_2,_2,_2,_2];
_7 = _1 - _1;
_1 = -_7;
_7 = _9;
Call(_3 = fn2(_1, _1, _6, _2, _7), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_5 = [_2,_2,_2,_2];
Goto(bb6)
}
bb6 = {
_6 = 7495891314630831886_usize ^ 2054697225590158826_usize;
_10 = [15696813127329166683643590783972171215_u128];
_5 = [_2,_2,_2,_2];
_4 = _9;
_11 = 161834163159231091053949710946905498485_u128 as usize;
_1 = _7;
_8 = _3;
_9 = 301863045061233286915389154848109948042_u128 as isize;
_10 = [293994098369149496556651932138565336067_u128];
_6 = _11;
_8 = _3;
_6 = !_11;
_6 = _11 | _11;
_6 = !_11;
_3 = -_8;
_6 = _11 - _11;
_9 = _7 << _3;
_12 = false;
_11 = !_6;
_6 = 5732132053833100783_u64 as usize;
_4 = _6 as isize;
_13.1 = _8 * _3;
_12 = _13.1 <= _8;
_14 = 14358008743638309528_u64 as isize;
match _1 {
0 => bb4,
1 => bb7,
2 => bb8,
5 => bb10,
_ => bb9
}
}
bb7 = {
_5 = [_2,_2,_2,_2];
Goto(bb6)
}
bb8 = {
_2 = '\u{fc7a0}';
_5 = [_2,_2,_2,_2];
_7 = _1 - _1;
_1 = -_7;
_7 = _9;
Call(_3 = fn2(_1, _1, _6, _2, _7), ReturnTo(bb5), UnwindUnreachable())
}
bb9 = {
Return()
}
bb10 = {
_13.0 = !12840300849412090525_u64;
_14 = _9;
_9 = -_1;
Goto(bb11)
}
bb11 = {
_7 = -_4;
_13.1 = !_8;
_13.1 = _3 ^ _8;
_10 = [188425447412530747207121370806776572939_u128];
_14 = _1 * _4;
_15 = -_14;
_6 = _11 | _11;
_11 = !_6;
_11 = !_6;
_9 = 3507587443_u32 as isize;
_13.1 = -_8;
_1 = _15;
_16 = -_14;
_4 = _15;
_3 = _11 as i64;
_3 = _13.1;
Call(_7 = core::intrinsics::bswap(_16), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_3 = !_13.1;
_11 = _6;
_5 = [_2,_2,_2,_2];
RET = core::ptr::addr_of!(_17);
RET = core::ptr::addr_of!((*RET));
_14 = !_15;
_11 = 1619_i16 as usize;
(*RET) = 47723_u16 as f64;
_11 = _6;
_14 = !_1;
_1 = -_16;
_2 = '\u{edd40}';
_18 = _2;
_2 = _18;
_5 = [_18,_2,_18,_18];
Goto(bb13)
}
bb13 = {
_5 = [_2,_2,_18,_18];
_13 = (11146408380900464856_u64, _8);
_2 = _18;
_4 = !_14;
_12 = true;
_3 = (-81_i8) as i64;
_21 = 75_u8;
_12 = true;
_17 = 3849923251_u32 as f64;
_12 = _14 < _14;
_14 = _4;
_22 = _13.0;
_6 = _11 * _11;
_26 = core::ptr::addr_of_mut!(_12);
_12 = !false;
(*_26) = true | false;
_22 = _13.0;
RET = core::ptr::addr_of!((*RET));
_24 = core::ptr::addr_of!(_2);
Goto(bb14)
}
bb14 = {
RET = core::ptr::addr_of!(_17);
(*RET) = _21 as f64;
_13.1 = -_8;
_14 = -_7;
_13.1 = !_8;
_29 = _22 as f32;
_15 = _1;
_6 = _1 as usize;
_23 = !_16;
_12 = false;
RET = core::ptr::addr_of!(_17);
_4 = -_1;
(*RET) = 162009265761588386229071423621236195127_u128 as f64;
_13.1 = _8;
_19 = [3074717061_u32,1862576017_u32];
_7 = _29 as isize;
_7 = -_14;
(*RET) = 1518103441_i32 as f64;
_1 = -_7;
_8 = !_3;
_10 = [94584737303248360221003610406574004050_u128];
_4 = _23;
_32 = 27_i8;
_4 = _23;
Goto(bb15)
}
bb15 = {
Call(_36 = dump_var(1_usize, 2_usize, Move(_2), 14_usize, Move(_14), 10_usize, Move(_10), 32_usize, Move(_32)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_36 = dump_var(1_usize, 9_usize, Move(_9), 19_usize, Move(_19), 1_usize, Move(_1), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_36 = dump_var(1_usize, 12_usize, Move(_12), 23_usize, Move(_23), 18_usize, Move(_18), 37_usize, _37), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn2(mut _1: isize,mut _2: isize,mut _3: usize,mut _4: char,mut _5: isize) -> i64 {
mir! {
type RET = i64;
let _6: Adt53;
let _7: *mut *mut u16;
let _8: *mut *mut u16;
let _9: &'static *mut u16;
let _10: (u64, i64);
let _11: u32;
let _12: f64;
let _13: &'static (u8, i32, [u16; 7], u64);
let _14: &'static u64;
let _15: *const (u8, i32, [u16; 7], u64);
let _16: f64;
let _17: [i64; 1];
let _18: isize;
let _19: (u128, [i32; 8], bool);
let _20: u64;
let _21: ();
let _22: ();
{
RET = -(-9161048754397568837_i64);
Goto(bb1)
}
bb1 = {
RET = 9140708964305120285_i64;
_2 = !_1;
RET = !495653285956651371_i64;
_3 = 6_usize;
Call(_2 = core::intrinsics::bswap(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = (-17186_i16) as isize;
_6.fld0 = [_4,_4,_4,_4];
_6.fld2 = core::ptr::addr_of_mut!(_6.fld1.0);
_6.fld1.1 = !0_i8;
RET = (-6938258014205146306_i64);
_6.fld1.3 = 22631_u16 as isize;
Goto(bb3)
}
bb3 = {
_1 = _5 >> _2;
_6.fld1.1 = 6643_i16 as i8;
_6.fld2 = core::ptr::addr_of_mut!(_6.fld1.0);
_6.fld2 = core::ptr::addr_of_mut!(_6.fld1.0);
_10.0 = !13774543014373091130_u64;
_6.fld1.2 = (-1799935388_i32);
_6.fld1.1 = (-117_i8) | (-125_i8);
RET = 371588464082024853_i64;
_6.fld1.0 = _10.0 as i16;
_6.fld1 = ((-14930_i16), (-39_i8), (-897451085_i32), _1);
_6.fld0 = [_4,_4,_4,_4];
Call(_6.fld3 = fn3(Move(_6.fld2), _6.fld1, _6.fld1.2, _6.fld1.2, _1, _1, _5, _6.fld1.0, _6.fld1.0, _6.fld1.3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_6.fld2 = core::ptr::addr_of_mut!(_6.fld1.0);
_10.0 = 7024241233693726163_u64 | 15818734843039502460_u64;
_10.1 = RET;
Goto(bb5)
}
bb5 = {
_5 = false as isize;
_2 = _1 & _6.fld1.3;
_10.1 = RET * RET;
_6.fld1 = (18547_i16, 103_i8, 1100676268_i32, _1);
_6.fld1.1 = 79_i8 << _6.fld1.3;
_5 = !_6.fld1.3;
_5 = _2 * _1;
_10.1 = RET | RET;
_6.fld2 = core::ptr::addr_of_mut!(_6.fld1.0);
_10.1 = _10.0 as i64;
_10 = (7003965056863179450_u64, RET);
_6.fld1 = (16426_i16, (-90_i8), 1194925610_i32, _5);
_10.1 = RET | RET;
_6.fld1 = ((-22438_i16), (-49_i8), 352897478_i32, _5);
_1 = _5 << _5;
_3 = !14128769248484261233_usize;
_10.0 = 18145001759112009063_u64;
_10.0 = 8469073235981985477_u64 + 755653390251135663_u64;
_2 = _5;
_6.fld0 = [_4,_4,_4,_4];
_11 = !3824050095_u32;
_4 = '\u{4cd0}';
_2 = _5 << _6.fld1.3;
Goto(bb6)
}
bb6 = {
_6.fld2 = core::ptr::addr_of_mut!(_6.fld1.0);
Goto(bb7)
}
bb7 = {
_12 = _3 as f64;
_10.0 = !953441105100138527_u64;
match _6.fld1.1 {
0 => bb8,
1 => bb9,
2 => bb10,
3 => bb11,
340282366920938463463374607431768211407 => bb13,
_ => bb12
}
}
bb8 = {
_6.fld2 = core::ptr::addr_of_mut!(_6.fld1.0);
Goto(bb7)
}
bb9 = {
_5 = false as isize;
_2 = _1 & _6.fld1.3;
_10.1 = RET * RET;
_6.fld1 = (18547_i16, 103_i8, 1100676268_i32, _1);
_6.fld1.1 = 79_i8 << _6.fld1.3;
_5 = !_6.fld1.3;
_5 = _2 * _1;
_10.1 = RET | RET;
_6.fld2 = core::ptr::addr_of_mut!(_6.fld1.0);
_10.1 = _10.0 as i64;
_10 = (7003965056863179450_u64, RET);
_6.fld1 = (16426_i16, (-90_i8), 1194925610_i32, _5);
_10.1 = RET | RET;
_6.fld1 = ((-22438_i16), (-49_i8), 352897478_i32, _5);
_1 = _5 << _5;
_3 = !14128769248484261233_usize;
_10.0 = 18145001759112009063_u64;
_10.0 = 8469073235981985477_u64 + 755653390251135663_u64;
_2 = _5;
_6.fld0 = [_4,_4,_4,_4];
_11 = !3824050095_u32;
_4 = '\u{4cd0}';
_2 = _5 << _6.fld1.3;
Goto(bb6)
}
bb10 = {
_6.fld2 = core::ptr::addr_of_mut!(_6.fld1.0);
_10.0 = 7024241233693726163_u64 | 15818734843039502460_u64;
_10.1 = RET;
Goto(bb5)
}
bb11 = {
RET = 9140708964305120285_i64;
_2 = !_1;
RET = !495653285956651371_i64;
_3 = 6_usize;
Call(_2 = core::intrinsics::bswap(_1), ReturnTo(bb2), UnwindUnreachable())
}
bb12 = {
_5 = (-17186_i16) as isize;
_6.fld0 = [_4,_4,_4,_4];
_6.fld2 = core::ptr::addr_of_mut!(_6.fld1.0);
_6.fld1.1 = !0_i8;
RET = (-6938258014205146306_i64);
_6.fld1.3 = 22631_u16 as isize;
Goto(bb3)
}
bb13 = {
_6.fld0 = [_4,_4,_4,_4];
_15 = Move(_6.fld3);
_6.fld1.0 = true as i16;
_10.0 = !8458893692410823510_u64;
_6.fld1.2 = 649725343_i32 * 1810970723_i32;
_6.fld3 = Move(_15);
_3 = !6_usize;
_6.fld1.3 = _1 | _1;
_4 = '\u{42ef4}';
RET = _10.1;
Goto(bb14)
}
bb14 = {
_16 = _12 * _12;
_3 = 268014896684967641896424982855452413943_u128 as usize;
_6.fld2 = core::ptr::addr_of_mut!(_6.fld1.0);
Goto(bb15)
}
bb15 = {
Call(_21 = dump_var(2_usize, 11_usize, Move(_11), 2_usize, Move(_2), 4_usize, Move(_4), 22_usize, _22), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: *mut i16,mut _2: (i16, i8, i32, isize),mut _3: i32,mut _4: i32,mut _5: isize,mut _6: isize,mut _7: isize,mut _8: i16,mut _9: i16,mut _10: isize) -> *const (u8, i32, [u16; 7], u64) {
mir! {
type RET = *const (u8, i32, [u16; 7], u64);
let _11: f32;
let _12: *mut *mut u16;
let _13: Adt73;
let _14: *mut *const (u128, [i32; 8], bool);
let _15: *const Adt54;
let _16: *const Adt54;
let _17: isize;
let _18: bool;
let _19: &'static &'static *const *mut i16;
let _20: [usize; 6];
let _21: f32;
let _22: [u64; 1];
let _23: isize;
let _24: isize;
let _25: (*const [u16; 7], Adt54);
let _26: [u16; 7];
let _27: (u64, i64);
let _28: f32;
let _29: Adt21;
let _30: (u64, i64);
let _31: f64;
let _32: u8;
let _33: Adt50;
let _34: [u16; 7];
let _35: usize;
let _36: usize;
let _37: ([bool; 7], (u128, [i32; 8], bool), f32);
let _38: char;
let _39: (u8, i32, [u16; 7], u64);
let _40: isize;
let _41: (*const f64, char, [char; 4]);
let _42: char;
let _43: f32;
let _44: i128;
let _45: f32;
let _46: char;
let _47: i64;
let _48: isize;
let _49: [i64; 5];
let _50: &'static u16;
let _51: ();
let _52: ();
{
_5 = 103_u8 as isize;
_2.3 = _6 + _6;
_10 = !_6;
_8 = _9;
_10 = _2.3 + _6;
_11 = 6_usize as f32;
_8 = 2308669476986567124_i64 as i16;
_4 = 2522413408_u32 as i32;
_3 = _2.2;
_2.1 = 51_i8 + 96_i8;
_2.3 = !_10;
_6 = _2.3 + _5;
_7 = _2.3 ^ _6;
_9 = -_8;
_10 = _2.3 & _7;
_2.2 = _4;
_4 = -_3;
_13 = Adt73::Variant0 { fld0: false,fld1: 3742860178189409901_usize };
place!(Field::<bool>(Variant(_13, 0), 0)) = true;
_2.3 = _10 & _6;
_5 = -_2.3;
_1 = core::ptr::addr_of_mut!(_8);
_2.1 = 18789075510879739712580904455254549081_u128 as i8;
_3 = _4;
_3 = _5 as i32;
(*_1) = _9;
Call(_12 = fn4(_5, _5, _6, _3, _2.3, _3, _2.3, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_13 = Adt73::Variant0 { fld0: false,fld1: 15073968871282498546_usize };
_6 = _5;
_3 = -_4;
_1 = core::ptr::addr_of_mut!(_9);
_13 = Adt73::Variant0 { fld0: false,fld1: 4_usize };
_17 = _11 as isize;
_18 = false ^ false;
place!(Field::<bool>(Variant(_13, 0), 0)) = _18;
_8 = _7 as i16;
_3 = -_4;
_17 = 15767989637945762353_u64 as isize;
_7 = _6 >> _2.3;
_2.1 = (-60_i8) | 29_i8;
(*_1) = _2.1 as i16;
_1 = core::ptr::addr_of_mut!(_8);
match _2.0 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768196526 => bb8,
_ => bb7
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
_8 = 118272191496783745773622180713673479609_u128 as i16;
place!(Field::<usize>(Variant(_13, 0), 1)) = !8479875568437020207_usize;
_11 = _6 as f32;
_3 = !_2.2;
_5 = _2.3 ^ _6;
(*_1) = _18 as i16;
place!(Field::<bool>(Variant(_13, 0), 0)) = _18;
_2.1 = 100_i8;
_5 = _6;
(*_1) = _9;
_5 = 154085859886099621764195630844134670656_u128 as isize;
_2 = (_9, (-123_i8), _4, _7);
_11 = _10 as f32;
_2.1 = _11 as i8;
_6 = Field::<bool>(Variant(_13, 0), 0) as isize;
_18 = Field::<bool>(Variant(_13, 0), 0);
_4 = 12156236487686662322_u64 as i32;
_10 = _2.3 ^ _2.3;
(*_1) = -_2.0;
_22 = [7814098030832781895_u64];
Goto(bb9)
}
bb9 = {
_24 = _10;
_10 = _7 & _7;
_3 = _2.2 | _2.2;
Goto(bb10)
}
bb10 = {
(*_1) = _9 ^ _9;
_1 = core::ptr::addr_of_mut!((*_1));
_2 = ((*_1), (-52_i8), _3, _10);
_7 = _10;
_11 = _3 as f32;
match _2.1 {
340282366920938463463374607431768211404 => bb12,
_ => bb11
}
}
bb11 = {
Return()
}
bb12 = {
_27 = (11075521723333833449_u64, 9107323346530650395_i64);
_27 = (11852098207705739789_u64, (-6962115496148628860_i64));
_8 = !_9;
_17 = -_2.3;
_26 = [14837_u16,25486_u16,12286_u16,49531_u16,6326_u16,61583_u16,62509_u16];
_25.0 = core::ptr::addr_of!(_26);
_7 = _17;
_20 = [Field::<usize>(Variant(_13, 0), 1),Field::<usize>(Variant(_13, 0), 1),Field::<usize>(Variant(_13, 0), 1),Field::<usize>(Variant(_13, 0), 1),Field::<usize>(Variant(_13, 0), 1),Field::<usize>(Variant(_13, 0), 1)];
_16 = core::ptr::addr_of!(_25.1);
_2 = ((*_1), 17_i8, _3, _17);
(*_1) = _2.0 >> _2.3;
_1 = core::ptr::addr_of_mut!(_9);
_27.0 = 11666635219808593792_u64 * 9952003821512282185_u64;
place!(Field::<bool>(Variant(_13, 0), 0)) = _18;
_2.0 = !_8;
_31 = 29163328703831771197232525069813881001_i128 as f64;
Goto(bb13)
}
bb13 = {
_30.1 = _27.1;
_29 = Adt21::Variant1 { fld0: _27.1,fld1: _2,fld2: _17,fld3: Move(_1),fld4: _11 };
place!(Field::<f32>(Variant(_29, 1), 4)) = _11;
_31 = Field::<i64>(Variant(_29, 1), 0) as f64;
_24 = Field::<(i16, i8, i32, isize)>(Variant(_29, 1), 1).3;
_32 = 26_u8;
_30.0 = _27.0 >> _2.0;
_1 = core::ptr::addr_of_mut!(_8);
_28 = -_11;
_31 = Field::<usize>(Variant(_13, 0), 1) as f64;
_2.0 = _8 << _7;
_11 = -_28;
_26 = [62668_u16,57578_u16,6405_u16,20943_u16,4798_u16,41303_u16,17434_u16];
place!(Field::<usize>(Variant(_13, 0), 1)) = 14635176536044541641_usize * 5422094619478099251_usize;
place!(Field::<(i16, i8, i32, isize)>(Variant(_29, 1), 1)).0 = -_2.0;
_2 = Field::<(i16, i8, i32, isize)>(Variant(_29, 1), 1);
_25.0 = core::ptr::addr_of!(_26);
_15 = core::ptr::addr_of!(_25.1);
_26 = [17277_u16,28074_u16,7916_u16,2636_u16,60874_u16,17741_u16,22743_u16];
_20 = [Field::<usize>(Variant(_13, 0), 1),Field::<usize>(Variant(_13, 0), 1),Field::<usize>(Variant(_13, 0), 1),Field::<usize>(Variant(_13, 0), 1),Field::<usize>(Variant(_13, 0), 1),Field::<usize>(Variant(_13, 0), 1)];
_10 = _7;
_27.0 = _30.0 * _30.0;
_32 = (*_1) as u8;
_2.0 = Field::<(i16, i8, i32, isize)>(Variant(_29, 1), 1).2 as i16;
place!(Field::<(i16, i8, i32, isize)>(Variant(_29, 1), 1)).0 = -(*_1);
match Field::<(i16, i8, i32, isize)>(Variant(_29, 1), 1).1 {
0 => bb8,
1 => bb11,
2 => bb9,
3 => bb14,
4 => bb15,
17 => bb17,
_ => bb16
}
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
_27.1 = _31 as i64;
place!(Field::<isize>(Variant(_29, 1), 2)) = _24;
place!(Field::<(i16, i8, i32, isize)>(Variant(_29, 1), 1)).3 = _24;
_30.1 = !Field::<i64>(Variant(_29, 1), 0);
place!(Field::<*mut i16>(Variant(_29, 1), 3)) = core::ptr::addr_of_mut!(_9);
_24 = _10;
_3 = _17 as i32;
place!(Field::<*mut i16>(Variant(_29, 1), 3)) = core::ptr::addr_of_mut!(_2.0);
_17 = _24;
_8 = _32 as i16;
_30 = _27;
(*_1) = !Field::<(i16, i8, i32, isize)>(Variant(_29, 1), 1).0;
_5 = _2.3;
place!(Field::<bool>(Variant(_13, 0), 0)) = _18;
_11 = _28 - Field::<f32>(Variant(_29, 1), 4);
place!(Field::<usize>(Variant(_13, 0), 1)) = 8483718132976177025_usize ^ 4722584083146131252_usize;
_26 = [15573_u16,56060_u16,50046_u16,25584_u16,41799_u16,29292_u16,23642_u16];
place!(Field::<*mut i16>(Variant(_29, 1), 3)) = core::ptr::addr_of_mut!(_9);
match Field::<(i16, i8, i32, isize)>(Variant(_29, 1), 1).1 {
0 => bb6,
1 => bb18,
2 => bb19,
17 => bb21,
_ => bb20
}
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
_2.3 = _7;
_23 = _24;
_4 = Field::<(i16, i8, i32, isize)>(Variant(_29, 1), 1).2;
_35 = Field::<usize>(Variant(_13, 0), 1);
SetDiscriminant(_13, 2);
_5 = Field::<(i16, i8, i32, isize)>(Variant(_29, 1), 1).3;
_39 = (_32, _3, _26, _30.0);
_2.0 = Field::<(i16, i8, i32, isize)>(Variant(_29, 1), 1).0;
_42 = '\u{75aee}';
place!(Field::<(i16, i8, i32, isize)>(Variant(_29, 1), 1)).2 = !_4;
_38 = _42;
place!(Field::<(*const [u16; 7], Adt54)>(Variant(_13, 2), 1)).0 = Move(_25.0);
_27 = _30;
(*_1) = Field::<(i16, i8, i32, isize)>(Variant(_29, 1), 1).0 | _2.0;
SetDiscriminant(_29, 1);
match _2.1 {
17 => bb23,
_ => bb22
}
}
bb22 = {
Return()
}
bb23 = {
_37.1.1 = [_2.2,_39.1,_39.1,_39.1,_3,_3,_3,_39.1];
_10 = _7;
_44 = (-34439737994481749647737281861215614014_i128);
_26 = [50992_u16,10346_u16,46137_u16,15576_u16,46188_u16,55520_u16,48440_u16];
(*_1) = _42 as i16;
_15 = core::ptr::addr_of!((*_16));
_2.1 = (-35_i8) << _32;
Call((*_1) = fn18(_4, _37.1.1), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
_39 = (_32, _4, _26, _27.0);
_40 = !_2.3;
_34 = [64985_u16,30334_u16,1258_u16,29509_u16,32849_u16,3568_u16,6267_u16];
place!(Field::<i8>(Variant(_13, 2), 2)) = _2.1;
_41.0 = core::ptr::addr_of!(_31);
RET = core::ptr::addr_of!(_39);
_39.2 = [16588_u16,50480_u16,36615_u16,357_u16,15059_u16,9778_u16,13202_u16];
_41.2 = [_42,_42,_38,_42];
_27 = (_30.0, _30.1);
Goto(bb25)
}
bb25 = {
Call(_51 = dump_var(3_usize, 6_usize, Move(_6), 9_usize, Move(_9), 40_usize, Move(_40), 7_usize, Move(_7)), ReturnTo(bb26), UnwindUnreachable())
}
bb26 = {
Call(_51 = dump_var(3_usize, 4_usize, Move(_4), 23_usize, Move(_23), 42_usize, Move(_42), 2_usize, Move(_2)), ReturnTo(bb27), UnwindUnreachable())
}
bb27 = {
Call(_51 = dump_var(3_usize, 35_usize, Move(_35), 22_usize, Move(_22), 27_usize, Move(_27), 17_usize, Move(_17)), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
Call(_51 = dump_var(3_usize, 10_usize, Move(_10), 52_usize, _52, 52_usize, _52, 52_usize, _52), ReturnTo(bb29), UnwindUnreachable())
}
bb29 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: isize,mut _2: isize,mut _3: isize,mut _4: i32,mut _5: isize,mut _6: i32,mut _7: isize,mut _8: (i16, i8, i32, isize)) -> *mut *mut u16 {
mir! {
type RET = *mut *mut u16;
let _9: *const f64;
let _10: char;
let _11: i64;
let _12: usize;
let _13: bool;
let _14: [bool; 5];
let _15: &'static (i64, i16, &'static u64);
let _16: [u32; 2];
let _17: [usize; 6];
let _18: i8;
let _19: &'static (*const f64, char, [char; 4]);
let _20: u128;
let _21: &'static (u8, i32, [u16; 7], u64);
let _22: *mut *const (u128, [i32; 8], bool);
let _23: isize;
let _24: [u16; 7];
let _25: u16;
let _26: f64;
let _27: *mut [i64; 5];
let _28: isize;
let _29: isize;
let _30: isize;
let _31: *const [i8; 6];
let _32: isize;
let _33: char;
let _34: f32;
let _35: char;
let _36: f64;
let _37: [bool; 5];
let _38: char;
let _39: &'static &'static u16;
let _40: Adt54;
let _41: i64;
let _42: [i64; 1];
let _43: u16;
let _44: isize;
let _45: *const (u8, i32, [u16; 7], u64);
let _46: &'static *mut u16;
let _47: [bool; 7];
let _48: *mut *const (u128, [i32; 8], bool);
let _49: isize;
let _50: i128;
let _51: usize;
let _52: [bool; 7];
let _53: char;
let _54: [char; 4];
let _55: f32;
let _56: ([bool; 7], (u128, [i32; 8], bool), f32);
let _57: [i32; 8];
let _58: i64;
let _59: [i64; 5];
let _60: (u8, (i64, i16, &'static u64));
let _61: (u8, (i64, i16, &'static u64));
let _62: [char; 2];
let _63: i32;
let _64: u8;
let _65: &'static &'static *const *mut i16;
let _66: Adt21;
let _67: &'static *mut u16;
let _68: ([bool; 7], (u128, [i32; 8], bool), f32);
let _69: (i64, i16, &'static u64);
let _70: (*const [u16; 7], Adt54);
let _71: [bool; 7];
let _72: Adt56;
let _73: [i8; 6];
let _74: [u16; 7];
let _75: [u64; 1];
let _76: f64;
let _77: [u64; 1];
let _78: isize;
let _79: i128;
let _80: &'static (u128, [i32; 8], bool);
let _81: u32;
let _82: bool;
let _83: *mut &'static u16;
let _84: bool;
let _85: usize;
let _86: f32;
let _87: *mut [i64; 5];
let _88: (i16, i8, i32, isize);
let _89: [u32; 2];
let _90: bool;
let _91: [i8; 6];
let _92: bool;
let _93: (u128, [i32; 8], bool);
let _94: f32;
let _95: *const (u8, i32, [u16; 7], u64);
let _96: isize;
let _97: [usize; 6];
let _98: i8;
let _99: (u8, i32, [u16; 7], u64);
let _100: u8;
let _101: [u64; 1];
let _102: &'static (*const f64, char, [char; 4]);
let _103: *mut *mut u16;
let _104: u128;
let _105: i32;
let _106: char;
let _107: i16;
let _108: u32;
let _109: i64;
let _110: f64;
let _111: isize;
let _112: isize;
let _113: f32;
let _114: u8;
let _115: &'static (i64, i16, &'static u64);
let _116: i16;
let _117: &'static [i64; 1];
let _118: [char; 4];
let _119: [char; 2];
let _120: u128;
let _121: f64;
let _122: isize;
let _123: f32;
let _124: [bool; 7];
let _125: *const u128;
let _126: *const (u128, [i32; 8], bool);
let _127: i8;
let _128: (Adt50, *const *mut i16, *const char, [i64; 5]);
let _129: [char; 4];
let _130: [u32; 2];
let _131: isize;
let _132: [char; 4];
let _133: [i8; 6];
let _134: *mut &'static u16;
let _135: u16;
let _136: u8;
let _137: Adt54;
let _138: char;
let _139: &'static (*const f64, char, [char; 4]);
let _140: f64;
let _141: *mut u16;
let _142: f64;
let _143: (i16, i8, i32, isize);
let _144: isize;
let _145: f64;
let _146: &'static i128;
let _147: ();
let _148: ();
{
_2 = 89_u8 as isize;
_8.1 = '\u{c7bed}' as i8;
_8.2 = _4 << _8.3;
_8.3 = _3 << _8.0;
_3 = -_1;
_8.3 = _1 << _8.2;
_3 = 41_u8 as isize;
_8 = (23026_i16, (-21_i8), _4, _5);
Call(_4 = core::intrinsics::bswap(_6), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8.2 = _6 ^ _4;
_8 = (24836_i16, (-69_i8), _4, _5);
_8.3 = 16807_u16 as isize;
_8.1 = 30_i8 & (-2_i8);
_12 = _4 as usize;
_8.2 = !_4;
_10 = '\u{b2460}';
_13 = false;
_11 = _8.1 as i64;
_5 = !_7;
_8.3 = _7 << _8.2;
_5 = _8.0 as isize;
_8.1 = _11 as i8;
_8.2 = _4;
_10 = '\u{89dd}';
_11 = 9162194355355040533_i64 - (-7395801450320766740_i64);
_5 = 8389_u16 as isize;
_8.3 = 1111103742_u32 as isize;
_17 = [_12,_12,_12,_12,_12,_12];
_11 = -(-8540784336066177847_i64);
_16 = [4225272899_u32,662081188_u32];
_4 = -_6;
_8.1 = -39_i8;
_3 = _7;
Goto(bb2)
}
bb2 = {
_13 = !true;
_1 = _7;
_18 = _8.1;
_16 = [2588323111_u32,2332409637_u32];
_8.3 = _8.1 as isize;
_11 = !3516496347730073545_i64;
Goto(bb3)
}
bb3 = {
_2 = !_5;
_3 = _10 as isize;
_7 = 58_u8 as isize;
_3 = 30679_u16 as isize;
_13 = true;
_10 = '\u{e6a69}';
_1 = !_3;
_13 = true | false;
_8.1 = _18;
_4 = _6 << _8.0;
_1 = _5;
Goto(bb4)
}
bb4 = {
_4 = 3116095708_u32 as i32;
_4 = _13 as i32;
_11 = (-4765780660949012882_i64);
Call(_5 = fn5(_12, _8.3, _8.2, _8.0, _8, _8, _8, _8, _8.0, _8, _12, _8.0), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_14 = [_13,_13,_13,_13,_13];
_10 = '\u{fef3e}';
_7 = _8.0 as isize;
_1 = !_7;
_23 = _7;
_17 = [_12,_12,_12,_12,_12,_12];
_23 = _7;
_12 = _8.2 as usize;
_17 = [_12,_12,_12,_12,_12,_12];
Call(_17 = fn6(_7, _1, _8, _12), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_5 = -_1;
_5 = _1 * _23;
_3 = _8.0 as isize;
_20 = 234_u8 as u128;
_11 = _12 as i64;
_12 = 3_usize & 5_usize;
_16 = [128466063_u32,2620055812_u32];
_17 = [_12,_12,_12,_12,_12,_12];
_8.3 = _8.0 as isize;
_8.0 = 1796_i16;
_23 = _12 as isize;
_11 = _20 as i64;
_11 = -8634160839873280644_i64;
_8.0 = !(-5879_i16);
_4 = _8.2;
_6 = 125473163454452009470547905249533531543_i128 as i32;
_8.3 = _7;
_10 = '\u{3d34}';
_8 = (4929_i16, _18, _4, _1);
Call(_22 = fn7(_3, _8), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
_10 = '\u{96412}';
_11 = 4694224147395772581_i64 ^ (-5005655418092625509_i64);
_8 = (12912_i16, _18, _4, _1);
_9 = core::ptr::addr_of!(_26);
_11 = _10 as i64;
(*_9) = 426193801_u32 as f64;
_1 = -_5;
_29 = _8.3 + _3;
_1 = _8.3 * _3;
_11 = (*_9) as i64;
match _8.0 {
0 => bb1,
12912 => bb9,
_ => bb8
}
}
bb8 = {
_4 = 3116095708_u32 as i32;
_4 = _13 as i32;
_11 = (-4765780660949012882_i64);
Call(_5 = fn5(_12, _8.3, _8.2, _8.0, _8, _8, _8, _8, _8.0, _8, _12, _8.0), ReturnTo(bb5), UnwindUnreachable())
}
bb9 = {
_33 = _10;
_30 = _5;
_23 = _30;
_25 = 21475_u16 << _3;
_18 = _8.1 | _8.1;
(*_9) = 499084138_u32 as f64;
_8.2 = -_4;
_33 = _10;
_3 = _7 - _29;
_3 = _8.3;
_8.1 = _18;
_25 = (*_9) as u16;
_3 = _5 - _30;
Goto(bb10)
}
bb10 = {
_7 = _8.3;
_32 = 383161840_u32 as isize;
_8.3 = 16692168535961580241_u64 as isize;
_28 = !_23;
_12 = 18221595987819317703_usize ^ 16496384888513144478_usize;
_35 = _10;
_34 = 141_u8 as f32;
_24 = [_25,_25,_25,_25,_25,_25,_25];
_2 = -_1;
_11 = 8896127283183104920_i64;
_2 = 14341834075022397107_u64 as isize;
_3 = _1;
_9 = core::ptr::addr_of!(_26);
_16 = [2022554645_u32,3333962256_u32];
(*_9) = 3300545476_u32 as f64;
_6 = _4;
_23 = -_28;
Goto(bb11)
}
bb11 = {
_29 = _13 as isize;
_3 = !_1;
_25 = 1944_u16 & 30895_u16;
_6 = _8.2 & _4;
_23 = _30;
(*_9) = _34 as f64;
_2 = _28;
_14 = [_13,_13,_13,_13,_13];
_6 = _8.2 << _8.0;
_16 = [2465624501_u32,1971323079_u32];
_11 = _12 as i64;
_23 = _7 >> _3;
Goto(bb12)
}
bb12 = {
_12 = 4_usize * 2577259045961208503_usize;
_20 = _11 as u128;
_12 = !16317686247727632670_usize;
_28 = -_23;
_23 = _3;
_10 = _33;
_2 = !_30;
match _8.0 {
12912 => bb13,
_ => bb5
}
}
bb13 = {
_26 = _25 as f64;
_5 = _12 as isize;
Goto(bb14)
}
bb14 = {
_32 = 17065677436192744491_u64 as isize;
_8.0 = _6 as i16;
_2 = _28 - _23;
_8.1 = _18;
_8.1 = _18;
_28 = _2;
_7 = _1 << _2;
_6 = _4 ^ _4;
_36 = (-82780221295592020640627755180635780750_i128) as f64;
_16 = [4077703699_u32,4020358811_u32];
_8.0 = -(-9129_i16);
_28 = _8.0 as isize;
_38 = _35;
_4 = !_6;
_4 = _13 as i32;
_34 = (*_9) as f32;
_8.3 = _3;
(*_9) = _36;
_18 = _12 as i8;
_17 = [_12,_12,_12,_12,_12,_12];
_8 = (21710_i16, _18, _6, _7);
Goto(bb15)
}
bb15 = {
_5 = -_1;
_6 = _8.2;
_2 = _30 >> _8.2;
_44 = 104277019981947830445687728012547033056_i128 as isize;
_23 = _2;
_37 = [_13,_13,_13,_13,_13];
_41 = _3 as i64;
(*_9) = _25 as f64;
_7 = -_5;
_8.0 = 31816_i16;
_9 = core::ptr::addr_of!(_26);
_43 = _34 as u16;
_6 = _8.2;
_32 = _30;
_41 = -_11;
_8.2 = _3 as i32;
_10 = _35;
_1 = -_3;
match _8.0 {
0 => bb6,
1 => bb14,
31816 => bb16,
_ => bb3
}
}
bb16 = {
_14 = _37;
_8.0 = 2650_i16 ^ (-19463_i16);
_48 = Move(_22);
_12 = 10475850453921047425_usize;
_13 = !true;
_37 = [_13,_13,_13,_13,_13];
_1 = _2 ^ _23;
_36 = 3162561201_u32 as f64;
_34 = _8.0 as f32;
_8.3 = -_32;
_9 = core::ptr::addr_of!((*_9));
_23 = (-99236719733144589038927902687804950332_i128) as isize;
_28 = 255_u8 as isize;
Goto(bb17)
}
bb17 = {
_9 = core::ptr::addr_of!((*_9));
match _12 {
0 => bb5,
1 => bb10,
2 => bb3,
10475850453921047425 => bb18,
_ => bb6
}
}
bb18 = {
_24 = [_25,_25,_25,_25,_43,_43,_25];
_28 = !_1;
_5 = _8.2 as isize;
_47 = [_13,_13,_13,_13,_13,_13,_13];
_33 = _10;
Goto(bb19)
}
bb19 = {
_8.3 = _28;
_8.3 = _32;
_38 = _35;
_37 = _14;
_12 = 9878110657783449020_usize << _8.2;
_22 = Move(_48);
_9 = core::ptr::addr_of!((*_9));
_4 = _6 * _6;
_9 = core::ptr::addr_of!((*_9));
(*_9) = _36;
_3 = 127719272347208180624887326959294527186_i128 as isize;
_16 = [3158478987_u32,2509579199_u32];
_2 = -_5;
_49 = _30 & _1;
_2 = _8.3;
_18 = _8.1;
_11 = _41;
_50 = _34 as i128;
Goto(bb20)
}
bb20 = {
_52 = [_13,_13,_13,_13,_13,_13,_13];
_34 = 4259165451696326424_u64 as f32;
_20 = _18 as u128;
_32 = _30 & _1;
_42 = [_11];
_25 = _43 | _43;
_42 = [_11];
_26 = _36 + _36;
_47 = [_13,_13,_13,_13,_13,_13,_13];
_24 = [_43,_25,_43,_43,_43,_25,_25];
_16 = [779748045_u32,1337991001_u32];
_44 = 15975635142897361937_u64 as isize;
_51 = _25 as usize;
_3 = _1;
_41 = _11 - _11;
_11 = _41 | _41;
_26 = -_36;
_53 = _38;
_36 = (*_9);
_17 = [_12,_12,_12,_12,_12,_12];
_12 = _34 as usize;
_16 = [294648180_u32,270926667_u32];
Goto(bb21)
}
bb21 = {
_37 = [_13,_13,_13,_13,_13];
_56.1.0 = _20;
_47 = [_13,_13,_13,_13,_13,_13,_13];
_8.1 = 677226800_u32 as i8;
_44 = _13 as isize;
_4 = _6;
_34 = _26 as f32;
_6 = -_8.2;
_7 = -_2;
_56.1.2 = !_13;
_9 = core::ptr::addr_of!((*_9));
_30 = !_2;
_42 = [_41];
_37 = _14;
_5 = _49;
_16 = [3370857047_u32,834422681_u32];
_8.0 = (-2976_i16) << _1;
_4 = _6 * _8.2;
_56.0 = _47;
_17 = [_51,_51,_12,_51,_51,_51];
_7 = _32;
Goto(bb22)
}
bb22 = {
_2 = _3;
_5 = _8.3 << _28;
(*_9) = _36 - _36;
_7 = _28 | _32;
_4 = _8.2;
_44 = !_1;
_11 = _20 as i64;
_56.1.1 = [_4,_4,_4,_4,_8.2,_4,_6,_6];
_60.1.0 = _11 << _49;
_23 = _49 + _2;
_8.1 = _60.1.0 as i8;
_44 = 18171716270100546725_u64 as isize;
_7 = _8.3 - _1;
_26 = -_36;
_3 = _20 as isize;
_8.0 = (-21526_i16) << _8.2;
_60.0 = 114_u8 ^ 1_u8;
_42 = [_60.1.0];
(*_9) = _32 as f64;
_54 = [_53,_33,_10,_53];
_25 = !_43;
_53 = _38;
_6 = -_4;
_50 = !(-81737963315601038721345211237650506814_i128);
_38 = _10;
_2 = -_7;
_59 = [_60.1.0,_60.1.0,_60.1.0,_60.1.0,_60.1.0];
_30 = _8.1 as isize;
Goto(bb23)
}
bb23 = {
_63 = _4;
_30 = -_5;
_47 = [_56.1.2,_56.1.2,_13,_13,_56.1.2,_13,_13];
_61.0 = _60.0;
_20 = _56.1.0;
_2 = !_8.3;
_48 = Move(_22);
_6 = _4 | _63;
_34 = _8.1 as f32;
_58 = _60.1.0 - _60.1.0;
_30 = _51 as isize;
_53 = _38;
_33 = _35;
_17 = [_12,_51,_12,_51,_12,_12];
_47 = [_13,_56.1.2,_56.1.2,_13,_13,_13,_56.1.2];
_20 = 3511150181249767438_u64 as u128;
_38 = _35;
_8.0 = _35 as i16;
_23 = _32;
_60.1.1 = _4 as i16;
_26 = -_36;
_5 = _2 - _23;
Goto(bb24)
}
bb24 = {
_25 = _56.1.2 as u16;
_38 = _10;
_56.1.2 = _13;
_27 = core::ptr::addr_of_mut!(_59);
_41 = _58;
_8.0 = _60.1.1 & _60.1.1;
_63 = _6;
_61.1.0 = _60.1.0 ^ _58;
_56.0 = _47;
_7 = _49;
_16 = [1953917000_u32,1217267007_u32];
_55 = -_34;
_35 = _33;
_51 = _12 - _12;
_10 = _38;
_69.0 = -_60.1.0;
_8.3 = _8.0 as isize;
_34 = (*_9) as f32;
_69.1 = !_60.1.1;
_56.2 = 2040459019_u32 as f32;
_71 = _47;
_56.0 = _47;
_17 = [_51,_51,_51,_12,_51,_51];
_44 = _7 ^ _7;
_5 = _1 ^ _49;
Call(_56 = fn16(_8.2, _32, _59, _8.2, _8), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
_62 = [_53,_33];
_33 = _53;
_56.1.1 = [_8.2,_8.2,_6,_8.2,_8.2,_4,_8.2,_8.2];
_61.0 = _60.1.0 as u8;
_31 = core::ptr::addr_of!(_73);
_44 = !_7;
_47 = [_56.1.2,_13,_13,_13,_13,_13,_13];
_71 = [_56.1.2,_13,_13,_56.1.2,_13,_56.1.2,_13];
(*_31) = [_8.1,_8.1,_8.1,_8.1,_8.1,_8.1];
_47 = _71;
_71 = [_13,_13,_56.1.2,_56.1.2,_56.1.2,_56.1.2,_13];
_74 = [_43,_25,_43,_43,_43,_43,_43];
_56.1.2 = _32 > _1;
_37 = [_56.1.2,_56.1.2,_56.1.2,_56.1.2,_56.1.2];
_57 = _56.1.1;
_62 = [_35,_35];
_62 = [_38,_53];
_41 = -_69.0;
_27 = core::ptr::addr_of_mut!(_59);
_68.1.2 = !_56.1.2;
_77 = [12655426567579049171_u64];
Goto(bb26)
}
bb26 = {
_68 = (_56.0, _56.1, _56.2);
_35 = _38;
_74 = _24;
_7 = _44 ^ _44;
_15 = &_60.1;
_60.0 = _8.1 as u8;
_64 = _50 as u8;
_24 = [_43,_43,_43,_25,_43,_43,_43];
_56.1.0 = _68.1.0;
_62 = [_10,_53];
_49 = _44;
Goto(bb27)
}
bb27 = {
_58 = _50 as i64;
_62 = [_53,_10];
_49 = _1 & _5;
_41 = _60.1.0 & _69.0;
_69.0 = _38 as i64;
_61.1.1 = _56.1.0 as i16;
_70.0 = core::ptr::addr_of!(_24);
_29 = _5;
_55 = _56.2 * _68.2;
_75 = [12306600268578632776_u64];
_52 = [_56.1.2,_56.1.2,_56.1.2,_68.1.2,_56.1.2,_56.1.2,_56.1.2];
_68.1.2 = !_56.1.2;
_17 = [_12,_12,_12,_12,_12,_12];
_20 = !_68.1.0;
_26 = _36;
Call(_7 = fn17(Move(_15), _1, _69.1, Move(_31), (*_27), _5, _5, _56.2, _8.1, _8, _61.0), ReturnTo(bb28), UnwindUnreachable())
}
bb28 = {
_68.1.0 = _55 as u128;
_8.0 = _60.1.1;
_59 = [_60.1.0,_60.1.0,_61.1.0,_61.1.0,_41];
_12 = !_51;
_57 = _56.1.1;
_82 = _68.1.2;
_34 = _50 as f32;
Goto(bb29)
}
bb29 = {
_56.1.1 = [_4,_63,_4,_6,_4,_63,_8.2,_63];
_38 = _35;
(*_9) = -_36;
_57 = [_4,_8.2,_4,_6,_8.2,_63,_63,_8.2];
_41 = !_61.1.0;
_22 = Move(_48);
_48 = Move(_22);
_61.1.0 = -_60.1.0;
_13 = !_68.1.2;
_16 = [4196893877_u32,3487214851_u32];
_24 = _74;
_68.0 = [_68.1.2,_56.1.2,_13,_68.1.2,_68.1.2,_56.1.2,_56.1.2];
_8.3 = -_28;
Goto(bb30)
}
bb30 = {
_24 = [_43,_43,_25,_25,_43,_25,_43];
_11 = _60.1.0 + _60.1.0;
(*_27) = [_60.1.0,_69.0,_11,_11,_60.1.0];
_88.3 = -_5;
_88.2 = _6 + _4;
_76 = _36;
_14 = _37;
_68.0 = [_13,_56.1.2,_82,_82,_56.1.2,_56.1.2,_82];
_69.1 = _29 as i16;
_4 = _6;
_70.0 = core::ptr::addr_of!(_74);
_26 = _76 * _76;
_91 = _73;
_1 = -_28;
_15 = &_60.1;
_36 = (*_9) + (*_9);
_88 = (_8.0, _8.1, _6, _44);
_79 = -_50;
_88.3 = !_5;
_42 = [_61.1.0];
_76 = -(*_9);
_7 = !_88.3;
Goto(bb31)
}
bb31 = {
_88.2 = _8.2;
_32 = _76 as isize;
_36 = _26 - _26;
_7 = _23 - _29;
_58 = (*_15).0 ^ (*_15).0;
_76 = -_36;
_88.1 = _43 as i8;
_91 = [_8.1,_8.1,_8.1,_8.1,_88.1,_8.1];
_36 = _26;
_86 = _55;
_90 = (*_15).1 > (*_15).1;
_92 = _82;
_20 = _68.1.0;
(*_9) = _36 - _36;
_18 = _8.1;
_93.0 = !_68.1.0;
Goto(bb32)
}
bb32 = {
_4 = _8.2;
Goto(bb33)
}
bb33 = {
_63 = -_4;
_22 = Move(_48);
_84 = !_92;
_18 = _25 as i8;
_69.0 = -_11;
_68.2 = _25 as f32;
_41 = _25 as i64;
_8.2 = _4 * _4;
_44 = _2;
_51 = _12;
Goto(bb34)
}
bb34 = {
_13 = _49 == _8.3;
_87 = Move(_27);
_28 = !_2;
_27 = Move(_87);
_93.2 = !_90;
_87 = Move(_27);
_93 = (_68.1.0, _68.1.1, _90);
_56.1.2 = _84 ^ _90;
_99 = (_60.0, _8.2, _74, 11756324306337845741_u64);
_24 = [_43,_43,_25,_43,_43,_43,_25];
_50 = !_79;
_88.2 = _8.2;
_68 = _56;
_59 = [_11,(*_15).0,_60.1.0,_69.0,(*_15).0];
_91 = _73;
_8.1 = !_88.1;
_30 = _29 & _23;
_20 = _93.0 >> _11;
_68.1.0 = !_20;
_88.1 = _8.1;
_61.1.2 = &_99.3;
Goto(bb35)
}
bb35 = {
_56.1.0 = _68.1.0 >> _20;
_75 = [_99.3];
_14 = _37;
(*_9) = _55 as f64;
_89 = [1926160639_u32,123127229_u32];
_69.0 = _61.1.0;
_76 = (*_9) * _26;
_97 = _17;
_82 = _88.2 != _4;
_81 = _76 as u32;
_99 = (_60.0, _88.2, _74, 3609412088939413641_u64);
_96 = _10 as isize;
_90 = !_84;
_88.3 = _8.3 | _8.3;
_101 = [_99.3];
_15 = &(*_15);
_13 = _90;
_76 = _26 + _26;
_86 = _68.2 + _56.2;
_3 = !_29;
_70.0 = core::ptr::addr_of!(_24);
Goto(bb36)
}
bb36 = {
_4 = !_8.2;
_69.0 = _30 as i64;
_43 = !_25;
_93.2 = _2 > _2;
_69.0 = _56.1.0 as i64;
_98 = _88.1 >> _3;
_85 = _12;
_93.0 = _4 as u128;
_93.0 = (*_9) as u128;
_37 = [_90,_56.1.2,_90,_84,_82];
_55 = -_86;
_101 = [_99.3];
_61.1.2 = &_99.3;
_60.1 = (_69.0, _8.0, Move(_61.1.2));
_42 = [_61.1.0];
Goto(bb37)
}
bb37 = {
_105 = (*_9) as i32;
_74 = [_25,_25,_43,_43,_25,_25,_43];
_95 = core::ptr::addr_of!(_99);
_56 = _68;
_61.1.2 = &(*_95).3;
_88.3 = !_3;
_68.1.0 = _93.0;
_68.1.0 = _93.0;
_82 = _56.1.2;
_109 = _60.1.0;
_106 = _35;
match (*_95).3 {
0 => bb36,
1 => bb13,
3609412088939413641 => bb39,
_ => bb38
}
}
bb38 = {
_88.2 = _8.2;
_32 = _76 as isize;
_36 = _26 - _26;
_7 = _23 - _29;
_58 = (*_15).0 ^ (*_15).0;
_76 = -_36;
_88.1 = _43 as i8;
_91 = [_8.1,_8.1,_8.1,_8.1,_88.1,_8.1];
_36 = _26;
_86 = _55;
_90 = (*_15).1 > (*_15).1;
_92 = _82;
_20 = _68.1.0;
(*_9) = _36 - _36;
_18 = _8.1;
_93.0 = !_68.1.0;
Goto(bb32)
}
bb39 = {
_16 = _89;
_70.1 = Adt54::Variant3 { fld0: _68.1.0,fld1: _68.1 };
_38 = _35;
_31 = core::ptr::addr_of!(_91);
_111 = _98 as isize;
_80 = &place!(Field::<(u128, [i32; 8], bool)>(Variant(_70.1, 3), 1));
_8.0 = -_60.1.1;
_112 = _1;
_110 = (*_9);
_15 = &_60.1;
_78 = _50 as isize;
_21 = &(*_95);
_114 = _43 as u8;
_107 = _85 as i16;
SetDiscriminant(_70.1, 2);
_105 = (*_21).1;
_56.0 = [_68.1.2,_56.1.2,_13,_82,_68.1.2,_56.1.2,_84];
Goto(bb40)
}
bb40 = {
_35 = _10;
_113 = _55;
_115 = Move(_15);
Goto(bb41)
}
bb41 = {
_114 = (*_95).0 | (*_95).0;
_60.1 = (_69.0, _8.0, Move(_61.1.2));
_14 = [_68.1.2,_93.2,_92,_68.1.2,_84];
_76 = (*_9) + (*_9);
_82 = !_93.2;
_60.1.1 = _69.1 & _88.0;
(*_95).0 = _114 | _114;
_56 = (_52, _93, _68.2);
_56 = _68;
_71 = _52;
_8.3 = _7;
_3 = _7 ^ _23;
_76 = _26;
_100 = !(*_95).0;
_114 = _60.1.1 as u8;
_92 = _68.2 <= _113;
_49 = _2;
_41 = _60.1.0 >> _88.3;
_24 = [_43,_25,_25,_43,_43,_43,_25];
_99.3 = 13209948797785258807_u64 << _6;
place!(Field::<i16>(Variant(_70.1, 2), 4)) = _68.2 as i16;
_69.2 = &_99.3;
_5 = !_30;
_8.1 = _98 + _98;
_18 = _8.1 << _8.3;
_89 = [_81,_81];
_118 = [_10,_38,_38,_106];
_107 = Field::<i16>(Variant(_70.1, 2), 4) - _69.1;
Goto(bb42)
}
bb42 = {
_88.3 = _8.3;
_88.2 = (*_9) as i32;
_14 = [_92,_84,_93.2,_92,_56.1.2];
_87 = core::ptr::addr_of_mut!(_59);
_58 = _109 - _41;
Goto(bb43)
}
bb43 = {
_93.2 = _99.3 != _99.3;
_104 = _99.3 as u128;
_99.3 = 3589771221699000296_u64;
_108 = !_81;
_86 = _25 as f32;
_93.0 = !_104;
_68.1.1 = [_88.2,_88.2,(*_21).1,(*_95).1,_105,(*_95).1,_88.2,_99.1];
_45 = Move(_95);
_16 = _89;
_119 = _62;
_118 = [_33,_38,_10,_38];
_115 = &_60.1;
_25 = _43;
Goto(bb44)
}
bb44 = {
_99.3 = _25 as u64;
_120 = _68.1.0;
_44 = _3 ^ _30;
place!(Field::<*const (u128, [i32; 8], bool)>(Variant(_70.1, 2), 7)) = core::ptr::addr_of!(_56.1);
_44 = _7 & _1;
_8.0 = (*_115).1 >> _98;
_15 = Move(_115);
_110 = _55 as f64;
_15 = &_69;
_88.0 = _69.1;
_56.1.0 = _104;
place!(Field::<char>(Variant(_70.1, 2), 1)) = _38;
_81 = !_108;
_47 = [_90,_13,_13,_84,_93.2,_93.2,_90];
Goto(bb45)
}
bb45 = {
_47 = _52;
_115 = &(*_15);
_88.0 = -_8.0;
_69.0 = _109;
_125 = core::ptr::addr_of!(_93.0);
_64 = _61.0 * _60.0;
_94 = _68.2 - _68.2;
_61.0 = _109 as u8;
place!(Field::<i8>(Variant(_70.1, 2), 3)) = _61.1.0 as i8;
_61.1.2 = &_99.3;
_99.1 = _63 + _105;
_63 = !_8.2;
_27 = Move(_87);
_60 = Move(_61);
Goto(bb46)
}
bb46 = {
_112 = _5;
_91 = _73;
_36 = (*_9) - (*_9);
place!(Field::<(i16, i8, i32, isize)>(Variant(_70.1, 2), 2)).3 = _2 | _111;
_68.2 = _56.2 + _113;
_78 = _4 as isize;
_123 = _94 - _55;
_64 = _99.0 | _60.0;
_3 = _8.1 as isize;
_60.1.1 = _105 as i16;
_128.3 = _59;
_118 = [_53,_33,_53,_33];
_125 = core::ptr::addr_of!((*_125));
_104 = _56.1.0;
_68.1 = _93;
place!(Field::<[u16; 7]>(Variant(_70.1, 2), 0)) = [_43,_25,_43,_25,_25,_25,_25];
_130 = _89;
_122 = _58 as isize;
_43 = _8.1 as u16;
Goto(bb47)
}
bb47 = {
_132 = _118;
_16 = [_108,_108];
_82 = !_90;
_1 = (*_125) as isize;
_44 = _8.3;
_100 = _64;
_88.3 = _69.0 as isize;
_86 = -_68.2;
_69.1 = _8.0;
_68.0 = [_13,_92,_90,_90,_93.2,_68.1.2,_56.1.2];
_121 = -(*_9);
(*_125) = !_20;
_88.3 = _100 as isize;
_56.2 = _113 - _94;
_53 = _35;
Goto(bb48)
}
bb48 = {
_21 = &_99;
_13 = !_68.1.2;
_68.1 = (_104, _56.1.1, _93.2);
_7 = _2 >> _4;
_48 = Move(_22);
_73 = (*_31);
_68.2 = -_86;
_94 = _113 + _123;
_88 = _8;
_12 = (*_21).3 as usize;
_130 = [_81,_108];
_135 = !_43;
_81 = _108;
_49 = _8.2 as isize;
_68.1.0 = !_93.0;
_136 = _100;
_68.1.2 = !_84;
_60.1.0 = _78 as i64;
_136 = _86 as u8;
_60.1.2 = &(*_21).3;
_31 = core::ptr::addr_of!(_133);
_93 = (_68.1.0, _56.1.1, _90);
_92 = _13;
_109 = _135 as i64;
_88.2 = (*_21).1 ^ _105;
_42 = [_69.0];
_61.1.0 = _69.0;
_13 = _30 >= _88.3;
Goto(bb49)
}
bb49 = {
_99.1 = _63;
_89 = [_108,_81];
_42 = [_60.1.0];
_99.0 = _64 & _114;
_129 = _54;
_56.1 = ((*_125), _68.1.1, _90);
_94 = _6 as f32;
_69.2 = Move(_60.1.2);
_10 = _35;
_55 = -_56.2;
_60.1.0 = _109;
_42 = [_11];
_97 = [_12,_51,_85,_12,_85,_51];
_15 = &_61.1;
_117 = &_42;
_8.3 = _112 | _88.3;
_113 = _68.2;
_22 = core::ptr::addr_of_mut!(place!(Field::<*const (u128, [i32; 8], bool)>(Variant(_70.1, 2), 7)));
_77 = [(*_21).3];
_81 = !_108;
_88.1 = Field::<i8>(Variant(_70.1, 2), 3);
_90 = _120 < _104;
Goto(bb50)
}
bb50 = {
place!(Field::<(i16, i8, i32, isize)>(Variant(_70.1, 2), 2)).1 = _41 as i8;
(*_125) = _135 as u128;
place!(Field::<(i16, i8, i32, isize)>(Variant(_70.1, 2), 2)).2 = !_4;
_8.2 = (*_125) as i32;
_8.0 = !_60.1.1;
_59 = _128.3;
_106 = Field::<char>(Variant(_70.1, 2), 1);
(*_125) = _56.1.0 << _109;
_17 = [_85,_51,_85,_85,_51,_51];
_58 = -(*_15).0;
_88.3 = _7 ^ _29;
_132 = [_35,_38,_10,_35];
_63 = _4 & _99.1;
_22 = Move(_48);
_84 = _90 ^ _90;
Call(_17 = core::intrinsics::transmute(_97), ReturnTo(bb51), UnwindUnreachable())
}
bb51 = {
_98 = Field::<i8>(Variant(_70.1, 2), 3);
_105 = !_63;
_28 = -_44;
Goto(bb52)
}
bb52 = {
_32 = -_49;
_61.1.0 = _11 ^ _41;
_61.1.2 = &_99.3;
_9 = core::ptr::addr_of!(_140);
_141 = core::ptr::addr_of_mut!(_135);
_140 = -_36;
Goto(bb53)
}
bb53 = {
_60.1.2 = &(*_21).3;
_85 = !_51;
_60.1.1 = -Field::<i16>(Variant(_70.1, 2), 4);
_145 = _99.3 as f64;
_128.0 = Adt50::Variant0 { fld0: _37,fld1: _108,fld2: _85 };
_143.3 = _32;
_21 = &_99;
_61 = ((*_21).0, Move(_60.1));
RET = core::ptr::addr_of_mut!(_141);
_73 = [_8.1,_8.1,_8.1,_88.1,Field::<i8>(Variant(_70.1, 2), 3),Field::<(i16, i8, i32, isize)>(Variant(_70.1, 2), 2).1];
Goto(bb54)
}
bb54 = {
Call(_147 = dump_var(4_usize, 93_usize, Move(_93), 42_usize, Move(_42), 3_usize, Move(_3), 37_usize, Move(_37)), ReturnTo(bb55), UnwindUnreachable())
}
bb55 = {
Call(_147 = dump_var(4_usize, 59_usize, Move(_59), 71_usize, Move(_71), 97_usize, Move(_97), 50_usize, Move(_50)), ReturnTo(bb56), UnwindUnreachable())
}
bb56 = {
Call(_147 = dump_var(4_usize, 20_usize, Move(_20), 12_usize, Move(_12), 62_usize, Move(_62), 75_usize, Move(_75)), ReturnTo(bb57), UnwindUnreachable())
}
bb57 = {
Call(_147 = dump_var(4_usize, 112_usize, Move(_112), 119_usize, Move(_119), 101_usize, Move(_101), 79_usize, Move(_79)), ReturnTo(bb58), UnwindUnreachable())
}
bb58 = {
Call(_147 = dump_var(4_usize, 13_usize, Move(_13), 4_usize, Move(_4), 74_usize, Move(_74), 100_usize, Move(_100)), ReturnTo(bb59), UnwindUnreachable())
}
bb59 = {
Call(_147 = dump_var(4_usize, 43_usize, Move(_43), 6_usize, Move(_6), 92_usize, Move(_92), 24_usize, Move(_24)), ReturnTo(bb60), UnwindUnreachable())
}
bb60 = {
Call(_147 = dump_var(4_usize, 107_usize, Move(_107), 111_usize, Move(_111), 1_usize, Move(_1), 109_usize, Move(_109)), ReturnTo(bb61), UnwindUnreachable())
}
bb61 = {
Call(_147 = dump_var(4_usize, 8_usize, Move(_8), 64_usize, Move(_64), 53_usize, Move(_53), 14_usize, Move(_14)), ReturnTo(bb62), UnwindUnreachable())
}
bb62 = {
Call(_147 = dump_var(4_usize, 99_usize, Move(_99), 88_usize, Move(_88), 32_usize, Move(_32), 90_usize, Move(_90)), ReturnTo(bb63), UnwindUnreachable())
}
bb63 = {
Call(_147 = dump_var(4_usize, 105_usize, Move(_105), 130_usize, Move(_130), 47_usize, Move(_47), 98_usize, Move(_98)), ReturnTo(bb64), UnwindUnreachable())
}
bb64 = {
Call(_147 = dump_var(4_usize, 7_usize, Move(_7), 120_usize, Move(_120), 132_usize, Move(_132), 148_usize, _148), ReturnTo(bb65), UnwindUnreachable())
}
bb65 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: usize,mut _2: isize,mut _3: i32,mut _4: i16,mut _5: (i16, i8, i32, isize),mut _6: (i16, i8, i32, isize),mut _7: (i16, i8, i32, isize),mut _8: (i16, i8, i32, isize),mut _9: i16,mut _10: (i16, i8, i32, isize),mut _11: usize,mut _12: i16) -> isize {
mir! {
type RET = isize;
let _13: ();
let _14: ();
{
_5 = (_9, _7.1, _8.2, _10.3);
_6.1 = -_7.1;
_3 = !_8.2;
Call(_11 = core::intrinsics::transmute(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = _10.1 as isize;
_7.1 = _3 as i8;
_12 = -_5.0;
RET = -_7.3;
RET = _3 as isize;
Goto(bb2)
}
bb2 = {
Call(_13 = dump_var(5_usize, 3_usize, Move(_3), 9_usize, Move(_9), 7_usize, Move(_7), 12_usize, Move(_12)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_13 = dump_var(5_usize, 6_usize, Move(_6), 5_usize, Move(_5), 14_usize, _14, 14_usize, _14), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: isize,mut _2: isize,mut _3: (i16, i8, i32, isize),mut _4: usize) -> [usize; 6] {
mir! {
type RET = [usize; 6];
let _5: Adt77;
let _6: (*const f64, char, [char; 4]);
let _7: &'static (u8, i32, [u16; 7], u64);
let _8: bool;
let _9: bool;
let _10: *const Adt43;
let _11: i32;
let _12: &'static (u128, [i32; 8], bool);
let _13: (u8, (i64, i16, &'static u64));
let _14: (i64, i16, &'static u64);
let _15: isize;
let _16: u16;
let _17: (*const [i8; 6], [i32; 8], *const Adt43, *mut i16);
let _18: u8;
let _19: i128;
let _20: ((*const [u16; 7], Adt54), &'static *const Adt54, &'static u16);
let _21: [u16; 7];
let _22: char;
let _23: isize;
let _24: ();
let _25: ();
{
_4 = 15235923685796277913_usize;
_3.2 = !959604724_i32;
_3.0 = 20532_i16 - (-31973_i16);
_1 = !_2;
_3.1 = 89_i8;
_3 = (29398_i16, (-39_i8), (-427866702_i32), _1);
RET = [_4,_4,_4,_4,_4,_4];
_3.2 = 1782163301_i32;
_6.1 = '\u{8538c}';
_3.1 = 63_i8 | (-102_i8);
_2 = -_3.3;
_3.0 = (-20685_i16) | (-24356_i16);
_3.0 = 19706_i16 * 20957_i16;
RET = [_4,_4,_4,_4,_4,_4];
_6.2 = [_6.1,_6.1,_6.1,_6.1];
RET = [_4,_4,_4,_4,_4,_4];
_8 = _3.3 != _3.3;
RET = [_4,_4,_4,_4,_4,_4];
_6.1 = '\u{77811}';
_6.2 = [_6.1,_6.1,_6.1,_6.1];
Goto(bb1)
}
bb1 = {
_3.0 = 4976792648562811847_u64 as i16;
_11 = _8 as i32;
_1 = _2;
_9 = _1 != _2;
_3.0 = (-6287_i16) >> _3.3;
_6.1 = '\u{a4583}';
_3.3 = _1;
_4 = !4_usize;
_6.1 = '\u{82669}';
_13.0 = !4_u8;
Goto(bb2)
}
bb2 = {
_3.2 = -_11;
_3.0 = 66316703841188511163240262161517494548_i128 as i16;
_6.1 = '\u{d154c}';
RET = [_4,_4,_4,_4,_4,_4];
_13.1.1 = -_3.0;
_14.0 = -7551920615507822626_i64;
_13.1.1 = -_3.0;
_3.3 = _1 >> _1;
_8 = _3.3 >= _1;
_9 = _3.3 == _1;
_3.0 = -_13.1.1;
_6.2 = [_6.1,_6.1,_6.1,_6.1];
_6.2 = [_6.1,_6.1,_6.1,_6.1];
_3.3 = 5492597110023333472_u64 as isize;
RET = [_4,_4,_4,_4,_4,_4];
_6.1 = '\u{7f5b1}';
_4 = 9724863859666447380_usize + 3_usize;
_6.1 = '\u{bcabe}';
_9 = _8;
_3.0 = _2 as i16;
_11 = _3.2;
RET = [_4,_4,_4,_4,_4,_4];
_3.3 = _1 | _1;
RET = [_4,_4,_4,_4,_4,_4];
_3.3 = 54670_u16 as isize;
Goto(bb3)
}
bb3 = {
_9 = _8;
_15 = -_2;
_14.0 = 27121_u16 as i64;
_3.2 = _11;
_14.0 = (-6823841488411959839_i64) * (-2236571946780515002_i64);
_6.1 = '\u{2f8be}';
_4 = !3_usize;
_11 = !_3.2;
_16 = _6.1 as u16;
_3 = (_13.1.1, 49_i8, _11, _2);
_3.0 = !_13.1.1;
_13.1.1 = 168172388802312769673390095580318891744_i128 as i16;
_14.1 = 30616640703317315911389004541504568344_i128 as i16;
_13.1.0 = !_14.0;
_3 = (_13.1.1, (-89_i8), _11, _2);
_14.1 = _13.1.1;
_13.0 = 243_u8;
_14.0 = _13.1.0;
_18 = _13.0 >> _3.1;
match _3.1 {
0 => bb2,
340282366920938463463374607431768211367 => bb5,
_ => bb4
}
}
bb4 = {
_3.0 = 4976792648562811847_u64 as i16;
_11 = _8 as i32;
_1 = _2;
_9 = _1 != _2;
_3.0 = (-6287_i16) >> _3.3;
_6.1 = '\u{a4583}';
_3.3 = _1;
_4 = !4_usize;
_6.1 = '\u{82669}';
_13.0 = !4_u8;
Goto(bb2)
}
bb5 = {
_18 = _13.0 * _13.0;
_4 = _3.1 as usize;
_3.3 = _1;
_3.0 = _11 as i16;
_6.2 = [_6.1,_6.1,_6.1,_6.1];
_17.1 = [_11,_3.2,_11,_11,_11,_3.2,_11,_11];
_13.0 = _18;
_3.0 = _13.1.1 ^ _14.1;
_14.1 = _13.1.1;
_11 = 31503794952922982724301769200182989508_u128 as i32;
_17.3 = core::ptr::addr_of_mut!(_13.1.1);
_8 = !_9;
RET = [_4,_4,_4,_4,_4,_4];
_19 = _6.1 as i128;
_20.2 = &_16;
_9 = _8 & _8;
Goto(bb6)
}
bb6 = {
Call(_24 = dump_var(6_usize, 16_usize, Move(_16), 18_usize, Move(_18), 15_usize, Move(_15), 8_usize, Move(_8)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_24 = dump_var(6_usize, 19_usize, Move(_19), 25_usize, _25, 25_usize, _25, 25_usize, _25), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: isize,mut _2: (i16, i8, i32, isize)) -> *mut *const (u128, [i32; 8], bool) {
mir! {
type RET = *mut *const (u128, [i32; 8], bool);
let _3: bool;
let _4: (*const [u16; 7], Adt54);
let _5: *const *const *mut i16;
let _6: i64;
let _7: &'static &'static u16;
let _8: (Adt50, *const *mut i16, *const char, [i64; 5]);
let _9: u16;
let _10: *const *mut i16;
let _11: [i64; 5];
let _12: f32;
let _13: u128;
let _14: (*const [u16; 7], Adt54);
let _15: *mut *const (u128, [i32; 8], bool);
let _16: [u128; 1];
let _17: i16;
let _18: &'static Adt50;
let _19: ([bool; 7], (u128, [i32; 8], bool), f32);
let _20: isize;
let _21: i64;
let _22: *const [u16; 7];
let _23: f64;
let _24: f64;
let _25: u16;
let _26: usize;
let _27: bool;
let _28: *mut i16;
let _29: [i64; 1];
let _30: i64;
let _31: [bool; 5];
let _32: *mut [i64; 5];
let _33: *mut u16;
let _34: [u32; 2];
let _35: (u8, i32, [u16; 7], u64);
let _36: bool;
let _37: *const *const *mut i16;
let _38: &'static u16;
let _39: Adt56;
let _40: [u128; 1];
let _41: *mut i16;
let _42: &'static (u128, [i32; 8], bool);
let _43: isize;
let _44: u32;
let _45: u16;
let _46: isize;
let _47: bool;
let _48: usize;
let _49: &'static (i64, i16, &'static u64);
let _50: u8;
let _51: [char; 2];
let _52: Adt56;
let _53: [char; 2];
let _54: char;
let _55: (u8, i32, [u16; 7], u64);
let _56: u64;
let _57: *const u128;
let _58: [u128; 1];
let _59: (*const [u16; 7], Adt54);
let _60: f32;
let _61: *const f64;
let _62: i128;
let _63: [i8; 6];
let _64: (u64, i64);
let _65: i16;
let _66: f32;
let _67: &'static [i64; 1];
let _68: u64;
let _69: [u16; 7];
let _70: [i32; 8];
let _71: [u64; 1];
let _72: *const *mut i16;
let _73: f32;
let _74: ();
let _75: ();
{
_1 = 6932857504682225665_u64 as isize;
_2.2 = _2.0 as i32;
_2 = (28374_i16, (-77_i8), 1800003620_i32, _1);
_2.3 = _1;
_2.1 = (-5592388591808838087_i64) as i8;
_1 = _2.3 ^ _2.3;
_2.1 = (-107_i8) - (-16_i8);
_2.3 = -_1;
_2 = (16996_i16, 121_i8, (-884375006_i32), _1);
_1 = -_2.3;
_2 = (22817_i16, (-48_i8), 184140516_i32, _1);
_2 = (12843_i16, 55_i8, (-1476350304_i32), _1);
_2.3 = _1 >> _2.1;
match _2.2 {
0 => bb1,
340282366920938463463374607430291861152 => bb3,
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
_2.3 = _1;
_2.1 = (-64_i8);
_2.1 = (-3_i8);
_2.2 = (-1263081160_i32) - (-590223074_i32);
_2.2 = 561481229_i32;
_3 = false;
_2.0 = (-3475_i16) | (-2656_i16);
_2.0 = !(-32588_i16);
_3 = false;
_2.1 = (-88_i8) ^ (-37_i8);
_2.3 = _1 | _1;
_2.3 = _2.2 as isize;
Goto(bb4)
}
bb4 = {
_2.0 = 94_u8 as i16;
_3 = _2.1 >= _2.1;
_2 = (11824_i16, 107_i8, 950240383_i32, _1);
_2.3 = _3 as isize;
_2.1 = -(-80_i8);
_2 = ((-24204_i16), (-14_i8), 1977287649_i32, _1);
_2.0 = 25422_i16;
_2.0 = _2.3 as i16;
_2.1 = 101_i8 * (-8_i8);
_2.2 = !1681667026_i32;
_2.0 = (-3426_i16);
_2.2 = -1935123650_i32;
_6 = 1464448260554563213_i64 + 2472537154278297575_i64;
Goto(bb5)
}
bb5 = {
_3 = !false;
_2.2 = 11223_u16 as i32;
_2.2 = -2048438051_i32;
_1 = _2.3 + _2.3;
_2.3 = _1;
_2.2 = (-1828348933_i32) - 1222269230_i32;
_5 = core::ptr::addr_of!(_10);
_2.0 = 105273797027946692641226557848473760235_u128 as i16;
_2.0 = -1445_i16;
_2.1 = !105_i8;
_2.1 = (-120_i8);
_6 = !(-7743990215896619235_i64);
_3 = true;
_9 = 48721_u16 & 7003_u16;
_6 = -(-211529912603004803_i64);
_5 = core::ptr::addr_of!((*_5));
_2.0 = _2.1 as i16;
match _2.1 {
0 => bb6,
1 => bb7,
2 => bb8,
340282366920938463463374607431768211336 => bb10,
_ => bb9
}
}
bb6 = {
_2.0 = 94_u8 as i16;
_3 = _2.1 >= _2.1;
_2 = (11824_i16, 107_i8, 950240383_i32, _1);
_2.3 = _3 as isize;
_2.1 = -(-80_i8);
_2 = ((-24204_i16), (-14_i8), 1977287649_i32, _1);
_2.0 = 25422_i16;
_2.0 = _2.3 as i16;
_2.1 = 101_i8 * (-8_i8);
_2.2 = !1681667026_i32;
_2.0 = (-3426_i16);
_2.2 = -1935123650_i32;
_6 = 1464448260554563213_i64 + 2472537154278297575_i64;
Goto(bb5)
}
bb7 = {
_2.3 = _1;
_2.1 = (-64_i8);
_2.1 = (-3_i8);
_2.2 = (-1263081160_i32) - (-590223074_i32);
_2.2 = 561481229_i32;
_3 = false;
_2.0 = (-3475_i16) | (-2656_i16);
_2.0 = !(-32588_i16);
_3 = false;
_2.1 = (-88_i8) ^ (-37_i8);
_2.3 = _1 | _1;
_2.3 = _2.2 as isize;
Goto(bb4)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_2 = (18381_i16, (-68_i8), 727554789_i32, _1);
Goto(bb11)
}
bb11 = {
_2.2 = (-812709399_i32) >> _2.0;
_2.0 = (-580_i16) - 9107_i16;
_2 = ((-29373_i16), 6_i8, (-520529128_i32), _1);
_8.3 = [_6,_6,_6,_6,_6];
_12 = 23_u8 as f32;
_11 = [_6,_6,_6,_6,_6];
_2 = (26303_i16, (-103_i8), 718300880_i32, _1);
Goto(bb12)
}
bb12 = {
_1 = -_2.3;
_5 = core::ptr::addr_of!((*_5));
_2.1 = 87_i8 + (-29_i8);
_2.0 = 17571381825929514859026614102275359584_i128 as i16;
_2.1 = 68_i8 + 31_i8;
_1 = !_2.3;
_2.1 = !10_i8;
_3 = !false;
_2.2 = 175496056_i32;
_2.3 = _2.1 as isize;
_12 = 14030785896608755515_u64 as f32;
_11 = _8.3;
_11 = [_6,_6,_6,_6,_6];
_9 = !49836_u16;
_2 = ((-19493_i16), 100_i8, 1228996299_i32, _1);
_12 = _2.0 as f32;
_9 = 4131252130_u32 as u16;
_9 = 64555_u16 - 15845_u16;
_12 = _2.0 as f32;
_6 = 4329701934611357257_i64;
_2 = (30453_i16, (-50_i8), (-1734073546_i32), _1);
_5 = core::ptr::addr_of!(_8.1);
Call(_5 = fn8(_11, _2, _2.1, _1, _6, _2.0, _2.0, _2.2, _1, _6, _2.1, _1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_9 = 7670_u16 - 31606_u16;
_11 = _8.3;
_2.3 = _1 & _1;
_2 = ((-14228_i16), 111_i8, (-144222740_i32), _1);
_2.2 = 178804174727278240477697511756243089690_u128 as i32;
_2.1 = (-51_i8) << _2.3;
_13 = 240526398045304356020858556456096966067_u128 + 171176456170119788057168393462158891288_u128;
_12 = _2.3 as f32;
_8.3 = _11;
_2.0 = !(-21691_i16);
_16 = [_13];
_1 = -_2.3;
_16 = [_13];
Goto(bb14)
}
bb14 = {
_17 = _2.0;
_8.3 = _11;
_2.1 = (-128_i8);
_11 = [_6,_6,_6,_6,_6];
_19.2 = _12;
_19.1.2 = _3;
_17 = 18054596881447727593_u64 as i16;
_19.1.0 = _13;
_19.1.1 = [_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2];
_19.1.1 = [_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2];
_12 = _19.2 - _19.2;
_1 = _19.2 as isize;
_9 = !23079_u16;
_13 = !_19.1.0;
_16 = [_19.1.0];
_19.0 = [_3,_3,_19.1.2,_3,_19.1.2,_19.1.2,_19.1.2];
_2 = (_17, 98_i8, (-1105098467_i32), _1);
Call(_10 = fn9(_19.1.1, _2.2, _12, _12, _1, _19, _2.1, _16), ReturnTo(bb15), UnwindUnreachable())
}
bb15 = {
_19.1.2 = !_3;
_20 = -_1;
_19.1.2 = _3;
_16 = [_13];
_11 = [_6,_6,_6,_6,_6];
_13 = !_19.1.0;
_21 = _2.2 as i64;
_18 = &_8.0;
_19.1.2 = _3;
_9 = 39228_u16;
_3 = _19.2 != _19.2;
_20 = _1;
_8.1 = Move(_10);
_2.2 = -332320419_i32;
_19.1.1 = [_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2];
_21 = _6;
_23 = 9931252532302479052_usize as f64;
_17 = -_2.0;
Call(_1 = fn10(Move(_8.1), _20, _2, _3, _19.1.1, _2, _20, _20), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
_19.0 = [_3,_3,_3,_19.1.2,_3,_3,_3];
_16 = [_13];
_26 = !3_usize;
_19.1.1 = [_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2];
_10 = core::ptr::addr_of!(_28);
_6 = -_21;
(*_10) = core::ptr::addr_of_mut!(_2.0);
_1 = _2.3 | _20;
_26 = _1 as usize;
_27 = _3 & _3;
_18 = &(*_18);
_18 = &(*_18);
_19.1.1 = [_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2];
_16 = [_13];
Call(_16 = fn15(_3, _2.1, _19.1, _19, _2.1, _12, _2.3, _2.1, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
_29 = [_6];
_29 = [_6];
_30 = _6;
_19.1.1 = [_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2];
_19.1.0 = _27 as u128;
_17 = !_2.0;
_2.3 = (-68102096897343409569432405261170054330_i128) as isize;
_24 = -_23;
_10 = core::ptr::addr_of!(_28);
_35.0 = _23 as u8;
_25 = !_9;
_25 = !_9;
_8.1 = Move(_10);
_2.3 = _12 as isize;
_19.1.1 = [_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2];
_34 = [3778185023_u32,2186075552_u32];
_31 = [_19.1.2,_27,_27,_27,_3];
_3 = _27;
_33 = core::ptr::addr_of_mut!(_9);
match _2.1 {
0 => bb18,
1 => bb19,
98 => bb21,
_ => bb20
}
}
bb18 = {
Return()
}
bb19 = {
_1 = -_2.3;
_5 = core::ptr::addr_of!((*_5));
_2.1 = 87_i8 + (-29_i8);
_2.0 = 17571381825929514859026614102275359584_i128 as i16;
_2.1 = 68_i8 + 31_i8;
_1 = !_2.3;
_2.1 = !10_i8;
_3 = !false;
_2.2 = 175496056_i32;
_2.3 = _2.1 as isize;
_12 = 14030785896608755515_u64 as f32;
_11 = _8.3;
_11 = [_6,_6,_6,_6,_6];
_9 = !49836_u16;
_2 = ((-19493_i16), 100_i8, 1228996299_i32, _1);
_12 = _2.0 as f32;
_9 = 4131252130_u32 as u16;
_9 = 64555_u16 - 15845_u16;
_12 = _2.0 as f32;
_6 = 4329701934611357257_i64;
_2 = (30453_i16, (-50_i8), (-1734073546_i32), _1);
_5 = core::ptr::addr_of!(_8.1);
Call(_5 = fn8(_11, _2, _2.1, _1, _6, _2.0, _2.0, _2.2, _1, _6, _2.1, _1), ReturnTo(bb13), UnwindUnreachable())
}
bb20 = {
_2.0 = 94_u8 as i16;
_3 = _2.1 >= _2.1;
_2 = (11824_i16, 107_i8, 950240383_i32, _1);
_2.3 = _3 as isize;
_2.1 = -(-80_i8);
_2 = ((-24204_i16), (-14_i8), 1977287649_i32, _1);
_2.0 = 25422_i16;
_2.0 = _2.3 as i16;
_2.1 = 101_i8 * (-8_i8);
_2.2 = !1681667026_i32;
_2.0 = (-3426_i16);
_2.2 = -1935123650_i32;
_6 = 1464448260554563213_i64 + 2472537154278297575_i64;
Goto(bb5)
}
bb21 = {
(*_28) = -_17;
_21 = _6;
_38 = &_9;
_35.2 = [_9,(*_38),(*_38),(*_33),_9,(*_38),(*_33)];
_19.2 = 2710127158480923450_u64 as f32;
_35.1 = _2.2;
_37 = core::ptr::addr_of!(_10);
match _2.1 {
0 => bb16,
1 => bb8,
2 => bb3,
3 => bb4,
4 => bb14,
5 => bb22,
6 => bb23,
98 => bb25,
_ => bb24
}
}
bb22 = {
Return()
}
bb23 = {
_9 = 7670_u16 - 31606_u16;
_11 = _8.3;
_2.3 = _1 & _1;
_2 = ((-14228_i16), 111_i8, (-144222740_i32), _1);
_2.2 = 178804174727278240477697511756243089690_u128 as i32;
_2.1 = (-51_i8) << _2.3;
_13 = 240526398045304356020858556456096966067_u128 + 171176456170119788057168393462158891288_u128;
_12 = _2.3 as f32;
_8.3 = _11;
_2.0 = !(-21691_i16);
_16 = [_13];
_1 = -_2.3;
_16 = [_13];
Goto(bb14)
}
bb24 = {
_19.0 = [_3,_3,_3,_19.1.2,_3,_3,_3];
_16 = [_13];
_26 = !3_usize;
_19.1.1 = [_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2];
_10 = core::ptr::addr_of!(_28);
_6 = -_21;
(*_10) = core::ptr::addr_of_mut!(_2.0);
_1 = _2.3 | _20;
_26 = _1 as usize;
_27 = _3 & _3;
_18 = &(*_18);
_18 = &(*_18);
_19.1.1 = [_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2];
_16 = [_13];
Call(_16 = fn15(_3, _2.1, _19.1, _19, _2.1, _12, _2.3, _2.1, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb25 = {
_2 = (_17, 116_i8, _35.1, _1);
_35.3 = !14030826051573205316_u64;
_40 = _16;
_7 = &_38;
_18 = &(*_18);
_32 = core::ptr::addr_of_mut!(_11);
_14.0 = core::ptr::addr_of!(_35.2);
_38 = &(*_38);
_35.1 = -_2.2;
_36 = _26 < _26;
_19.2 = _12;
_30 = _21 - _6;
_42 = &_19.1;
_18 = &(*_18);
Goto(bb26)
}
bb26 = {
(*_37) = Move(_8.1);
_38 = &(*_38);
_20 = 1574342131_u32 as isize;
_30 = !_21;
_6 = _21;
_5 = Move(_37);
_8.3 = [_21,_30,_30,_6,_6];
_37 = core::ptr::addr_of!(_10);
_19.2 = _12;
_4.0 = core::ptr::addr_of!(_35.2);
_19.1.0 = _13 - _13;
_45 = !_9;
_22 = Move(_4.0);
_19.2 = -_12;
match _2.1 {
0 => bb10,
116 => bb28,
_ => bb27
}
}
bb27 = {
_9 = 7670_u16 - 31606_u16;
_11 = _8.3;
_2.3 = _1 & _1;
_2 = ((-14228_i16), 111_i8, (-144222740_i32), _1);
_2.2 = 178804174727278240477697511756243089690_u128 as i32;
_2.1 = (-51_i8) << _2.3;
_13 = 240526398045304356020858556456096966067_u128 + 171176456170119788057168393462158891288_u128;
_12 = _2.3 as f32;
_8.3 = _11;
_2.0 = !(-21691_i16);
_16 = [_13];
_1 = -_2.3;
_16 = [_13];
Goto(bb14)
}
bb28 = {
_2 = (_17, (-106_i8), _35.1, _1);
_23 = _24 * _24;
(*_32) = _8.3;
_19.1.0 = _13;
_17 = _23 as i16;
_35.3 = 9185832026379662799613664481076710920_i128 as u64;
_20 = (*_28) as isize;
_46 = _2.3;
(*_32) = [_21,_21,_21,_21,_21];
match _2.1 {
0 => bb29,
1 => bb30,
2 => bb31,
340282366920938463463374607431768211350 => bb33,
_ => bb32
}
}
bb29 = {
_2 = (18381_i16, (-68_i8), 727554789_i32, _1);
Goto(bb11)
}
bb30 = {
_19.1.2 = !_3;
_20 = -_1;
_19.1.2 = _3;
_16 = [_13];
_11 = [_6,_6,_6,_6,_6];
_13 = !_19.1.0;
_21 = _2.2 as i64;
_18 = &_8.0;
_19.1.2 = _3;
_9 = 39228_u16;
_3 = _19.2 != _19.2;
_20 = _1;
_8.1 = Move(_10);
_2.2 = -332320419_i32;
_19.1.1 = [_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2];
_21 = _6;
_23 = 9931252532302479052_usize as f64;
_17 = -_2.0;
Call(_1 = fn10(Move(_8.1), _20, _2, _3, _19.1.1, _2, _20, _20), ReturnTo(bb16), UnwindUnreachable())
}
bb31 = {
_2.3 = _1;
_2.1 = (-64_i8);
_2.1 = (-3_i8);
_2.2 = (-1263081160_i32) - (-590223074_i32);
_2.2 = 561481229_i32;
_3 = false;
_2.0 = (-3475_i16) | (-2656_i16);
_2.0 = !(-32588_i16);
_3 = false;
_2.1 = (-88_i8) ^ (-37_i8);
_2.3 = _1 | _1;
_2.3 = _2.2 as isize;
Goto(bb4)
}
bb32 = {
_19.0 = [_3,_3,_3,_19.1.2,_3,_3,_3];
_16 = [_13];
_26 = !3_usize;
_19.1.1 = [_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2];
_10 = core::ptr::addr_of!(_28);
_6 = -_21;
(*_10) = core::ptr::addr_of_mut!(_2.0);
_1 = _2.3 | _20;
_26 = _1 as usize;
_27 = _3 & _3;
_18 = &(*_18);
_18 = &(*_18);
_19.1.1 = [_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2];
_16 = [_13];
Call(_16 = fn15(_3, _2.1, _19.1, _19, _2.1, _12, _2.3, _2.1, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb33 = {
(*_32) = [_21,_6,_21,_21,_21];
(*_37) = core::ptr::addr_of!(_41);
Call(_8.1 = core::intrinsics::arith_offset(_10, (-6_isize)), ReturnTo(bb34), UnwindUnreachable())
}
bb34 = {
_40 = [_13];
_19.2 = _12 * _12;
_51 = ['\u{12db4}','\u{facdd}'];
_35.3 = _17 as u64;
_24 = -_23;
_17 = !(*_28);
_14.0 = core::ptr::addr_of!(_35.2);
_1 = _2.3;
_25 = _46 as u16;
_24 = _23 * _23;
_2.2 = _35.1 - _35.1;
_14.1 = Adt54::Variant3 { fld0: _13,fld1: _19.1 };
place!(Field::<(u128, [i32; 8], bool)>(Variant(_14.1, 3), 1)).2 = _19.2 != _12;
_20 = _46;
_2 = (_17, 93_i8, _35.1, _46);
_19.1.1 = [_35.1,_2.2,_2.2,_2.2,_35.1,_2.2,_2.2,_2.2];
_43 = -_20;
_32 = core::ptr::addr_of_mut!(_11);
(*_33) = _25 & _45;
_7 = &_38;
match _2.1 {
0 => bb28,
1 => bb17,
2 => bb33,
93 => bb35,
_ => bb4
}
}
bb35 = {
_42 = &_19.1;
_48 = _26;
_23 = -_24;
_57 = core::ptr::addr_of!(_13);
_25 = _9;
_6 = -_30;
_4 = (Move(_14.0), Move(_14.1));
_20 = !_2.3;
(*_57) = _19.1.0;
_35.1 = _2.2 & _2.2;
_40 = [(*_57)];
_41 = Move(_28);
SetDiscriminant(_4.1, 3);
_7 = &_38;
_19.1.0 = _2.2 as u128;
Goto(bb36)
}
bb36 = {
_60 = _21 as f32;
_46 = _20;
(*_33) = _25;
_8.0 = Adt50::Variant0 { fld0: _31,fld1: 3550558457_u32,fld2: _48 };
(*_57) = _19.1.0 >> _20;
(*_10) = core::ptr::addr_of_mut!(_17);
_41 = core::ptr::addr_of_mut!((*_41));
_19.0 = [_3,_36,_3,_36,_36,_27,_27];
match _2.1 {
0 => bb23,
93 => bb37,
_ => bb2
}
}
bb37 = {
_55.3 = _35.3 ^ _35.3;
_16 = _40;
(*_57) = !_19.1.0;
_8.3 = [_30,_30,_30,_6,_30];
_1 = _46 + _46;
_58 = [(*_57)];
_10 = core::ptr::addr_of!(_41);
_55 = (_35.0, _2.2, _35.2, _35.3);
_10 = core::ptr::addr_of!((*_10));
place!(Field::<(u128, [i32; 8], bool)>(Variant(_4.1, 3), 1)).2 = !_36;
_20 = _46 << _48;
_35.2 = [_25,_25,_25,(*_33),_9,(*_33),_25];
(*_41) = _19.1.0 as i16;
_19.0 = [_3,_27,_27,_27,Field::<(u128, [i32; 8], bool)>(Variant(_4.1, 3), 1).2,_3,_36];
_55 = _35;
_55.3 = _35.3 << _1;
_60 = (*_57) as f32;
_41 = core::ptr::addr_of_mut!((*_41));
Goto(bb38)
}
bb38 = {
_8.2 = core::ptr::addr_of!(_54);
_56 = _35.3;
_36 = !Field::<(u128, [i32; 8], bool)>(Variant(_4.1, 3), 1).2;
_65 = (*_41) | _2.0;
_27 = _3;
(*_57) = _19.1.0;
place!(Field::<(u128, [i32; 8], bool)>(Variant(_4.1, 3), 1)).0 = !_13;
_47 = _36;
_47 = _3;
_7 = &(*_7);
(*_57) = _19.1.0 ^ Field::<(u128, [i32; 8], bool)>(Variant(_4.1, 3), 1).0;
_18 = &_8.0;
_53 = ['\u{bf11}','\u{ef9c7}'];
_2.2 = _35.1;
_16 = _58;
_23 = -_24;
_19.1.0 = _55.3 as u128;
_40 = _16;
_51 = ['\u{d118f}','\u{b98d0}'];
_58 = [_19.1.0];
_45 = !_9;
_14.1 = Adt54::Variant3 { fld0: _19.1.0,fld1: _19.1 };
place!(Field::<(u128, [i32; 8], bool)>(Variant(_4.1, 3), 1)).1 = (*_42).1;
_33 = core::ptr::addr_of_mut!(_25);
_55.1 = (*_33) as i32;
_2.3 = _17 as isize;
_2.0 = !_65;
match _2.1 {
0 => bb11,
1 => bb16,
2 => bb39,
3 => bb40,
93 => bb42,
_ => bb41
}
}
bb39 = {
_19.0 = [_3,_3,_3,_19.1.2,_3,_3,_3];
_16 = [_13];
_26 = !3_usize;
_19.1.1 = [_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2];
_10 = core::ptr::addr_of!(_28);
_6 = -_21;
(*_10) = core::ptr::addr_of_mut!(_2.0);
_1 = _2.3 | _20;
_26 = _1 as usize;
_27 = _3 & _3;
_18 = &(*_18);
_18 = &(*_18);
_19.1.1 = [_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2,_2.2];
_16 = [_13];
Call(_16 = fn15(_3, _2.1, _19.1, _19, _2.1, _12, _2.3, _2.1, _27), ReturnTo(bb17), UnwindUnreachable())
}
bb40 = {
_9 = 7670_u16 - 31606_u16;
_11 = _8.3;
_2.3 = _1 & _1;
_2 = ((-14228_i16), 111_i8, (-144222740_i32), _1);
_2.2 = 178804174727278240477697511756243089690_u128 as i32;
_2.1 = (-51_i8) << _2.3;
_13 = 240526398045304356020858556456096966067_u128 + 171176456170119788057168393462158891288_u128;
_12 = _2.3 as f32;
_8.3 = _11;
_2.0 = !(-21691_i16);
_16 = [_13];
_1 = -_2.3;
_16 = [_13];
Goto(bb14)
}
bb41 = {
_42 = &_19.1;
_48 = _26;
_23 = -_24;
_57 = core::ptr::addr_of!(_13);
_25 = _9;
_6 = -_30;
_4 = (Move(_14.0), Move(_14.1));
_20 = !_2.3;
(*_57) = _19.1.0;
_35.1 = _2.2 & _2.2;
_40 = [(*_57)];
_41 = Move(_28);
SetDiscriminant(_4.1, 3);
_7 = &_38;
_19.1.0 = _2.2 as u128;
Goto(bb36)
}
bb42 = {
SetDiscriminant(_14.1, 0);
_17 = !_2.0;
_2 = (_17, 25_i8, _55.1, _20);
_43 = 91119482153441796363526166257245741812_i128 as isize;
_55 = _35;
_64 = (_55.3, _6);
place!(Field::<u32>(Variant(_8.0, 0), 1)) = !4062395468_u32;
_6 = _30;
SetDiscriminant(_8.0, 0);
_48 = _26 * _26;
place!(Field::<[char; 2]>(Variant(_14.1, 0), 6)) = ['\u{40361}','\u{d3298}'];
(*_10) = core::ptr::addr_of_mut!(_65);
_55 = _35;
_61 = core::ptr::addr_of!(_23);
_42 = &place!(Field::<(u128, [i32; 8], bool)>(Variant(_4.1, 3), 1));
_42 = &place!(Field::<(u128, [i32; 8], bool)>(Variant(_4.1, 3), 1));
_56 = _2.1 as u64;
_55 = (_35.0, _2.2, _35.2, _56);
place!(Field::<i128>(Variant(_14.1, 0), 7)) = -78631897940619529847693556063609357725_i128;
(*_10) = core::ptr::addr_of_mut!((*_41));
RET = core::ptr::addr_of_mut!(place!(Field::<*const (u128, [i32; 8], bool)>(Variant(_14.1, 0), 2)));
_55.2 = [(*_33),_25,(*_33),_25,(*_33),(*_33),(*_33)];
place!(Field::<(u128, [i32; 8], bool)>(Variant(_4.1, 3), 1)).2 = Field::<(u128, [i32; 8], bool)>(Variant(_4.1, 3), 1).0 > _19.1.0;
Goto(bb43)
}
bb43 = {
Call(_74 = dump_var(7_usize, 35_usize, Move(_35), 64_usize, Move(_64), 20_usize, Move(_20), 11_usize, Move(_11)), ReturnTo(bb44), UnwindUnreachable())
}
bb44 = {
Call(_74 = dump_var(7_usize, 2_usize, Move(_2), 9_usize, Move(_9), 29_usize, Move(_29), 26_usize, Move(_26)), ReturnTo(bb45), UnwindUnreachable())
}
bb45 = {
Call(_74 = dump_var(7_usize, 36_usize, Move(_36), 1_usize, Move(_1), 51_usize, Move(_51), 31_usize, Move(_31)), ReturnTo(bb46), UnwindUnreachable())
}
bb46 = {
Call(_74 = dump_var(7_usize, 43_usize, Move(_43), 21_usize, Move(_21), 40_usize, Move(_40), 53_usize, Move(_53)), ReturnTo(bb47), UnwindUnreachable())
}
bb47 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: [i64; 5],mut _2: (i16, i8, i32, isize),mut _3: i8,mut _4: isize,mut _5: i64,mut _6: i16,mut _7: i16,mut _8: i32,mut _9: isize,mut _10: i64,mut _11: i8,mut _12: isize) -> *const *const *mut i16 {
mir! {
type RET = *const *const *mut i16;
let _13: char;
let _14: char;
let _15: f64;
let _16: Adt43;
let _17: &'static i128;
let _18: *mut &'static *mut u16;
let _19: f64;
let _20: char;
let _21: [char; 2];
let _22: *const (u128, [i32; 8], bool);
let _23: &'static [i64; 1];
let _24: i128;
let _25: f32;
let _26: [u128; 1];
let _27: f32;
let _28: [i64; 5];
let _29: u128;
let _30: bool;
let _31: usize;
let _32: f64;
let _33: u16;
let _34: *mut [i64; 5];
let _35: [i64; 5];
let _36: &'static *const *mut i16;
let _37: *mut u16;
let _38: [u16; 7];
let _39: &'static (u8, i32, [u16; 7], u64);
let _40: i16;
let _41: f32;
let _42: [char; 2];
let _43: *mut Adt54;
let _44: char;
let _45: isize;
let _46: i32;
let _47: &'static *const Adt54;
let _48: [u32; 2];
let _49: *const *mut i16;
let _50: &'static &'static *const *mut i16;
let _51: ();
let _52: ();
{
_11 = _2.1 * _3;
_2.2 = _8 << _11;
_10 = _5 * _5;
match _2.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
30453 => bb7,
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
_13 = '\u{90f87}';
_2.1 = _10 as i8;
_10 = _13 as i64;
_2.0 = 9001644576931657415935823470224832051_i128 as i16;
_14 = _13;
_2 = (_7, _11, _8, _9);
_13 = _14;
_2.0 = _6;
_2.2 = _8;
_2.0 = 172503220815626115900698610843942115686_u128 as i16;
_10 = _5;
_2.1 = _9 as i8;
_14 = _13;
_14 = _13;
_15 = _9 as f64;
_9 = _12 << _11;
_11 = _2.1 * _2.1;
_2.3 = _9;
_16.fld0 = 12120_u16;
_2.2 = !_8;
_3 = _11;
_13 = _14;
_14 = _13;
Goto(bb8)
}
bb8 = {
_3 = true as i8;
_19 = 57601853112550870648680288818763596731_u128 as f64;
_2.1 = _11;
_2.3 = !_9;
_20 = _14;
_8 = _2.1 as i32;
_2.1 = _11 * _11;
_15 = _19 - _19;
_16.fld2.0 = _7;
_16.fld0 = 42922_u16 * 48244_u16;
_13 = _14;
_14 = _13;
_16.fld2 = (_7, _11, _2.2, _4);
_21 = [_14,_14];
_16.fld2.0 = _15 as i16;
Goto(bb9)
}
bb9 = {
_2.1 = -_16.fld2.1;
_7 = _16.fld2.0;
match _6 {
30453 => bb10,
_ => bb3
}
}
bb10 = {
_20 = _13;
_5 = _10;
_3 = _16.fld2.1 & _11;
_24 = (-26654105454861845078819997142874769348_i128);
_15 = -_19;
_16.fld3 = _24 as f64;
_10 = _5;
_16.fld3 = _15 + _15;
_28 = [_5,_10,_5,_10,_10];
_27 = _16.fld0 as f32;
_6 = _7;
_16.fld2.2 = _2.2;
_2.0 = !_6;
_24 = 7441502623915568857497096310905434255_i128;
_25 = -_27;
_13 = _20;
_17 = &_24;
_19 = _16.fld3;
_16.fld2.1 = _3 - _11;
_31 = !7_usize;
_16.fld2.1 = _5 as i8;
_5 = _10 << (*_17);
Goto(bb11)
}
bb11 = {
_7 = !_16.fld2.0;
_16.fld2.0 = _6;
_8 = _2.2;
_30 = !false;
_32 = _16.fld3;
_16.fld2.2 = -_8;
_13 = _20;
_2.1 = _11 * _3;
_33 = _16.fld0;
_4 = 1930136271_u32 as isize;
_32 = _8 as f64;
_16.fld2.2 = _8;
_32 = _16.fld3;
_14 = _13;
_35 = _1;
_35 = _1;
_26 = [236900391699684535705185805545426371723_u128];
_24 = (-18580797254238204287856232537449136848_i128);
_17 = &_24;
_5 = _10 << _31;
_2.2 = 2429725258_u32 as i32;
Goto(bb12)
}
bb12 = {
_34 = core::ptr::addr_of_mut!(_35);
(*_34) = [_5,_10,_5,_10,_5];
_15 = _16.fld3;
_24 = 8577334860505752786961142674524711511_i128;
_38 = [_33,_33,_16.fld0,_16.fld0,_16.fld0,_33,_16.fld0];
_31 = _9 as usize;
_3 = !_2.1;
_26 = [256127393869466466875580775527953343080_u128];
_16.fld3 = _10 as f64;
_16.fld2.0 = _6 * _6;
_11 = _30 as i8;
_37 = core::ptr::addr_of_mut!(_16.fld0);
_4 = _2.3 - _2.3;
_14 = _20;
_17 = &_24;
_16.fld3 = _19 + _15;
_40 = _7 - _7;
_10 = _5 | _5;
_2.1 = _3;
_5 = !_10;
_37 = core::ptr::addr_of_mut!((*_37));
_2.3 = _2.1 as isize;
_15 = _19;
_27 = -_25;
_6 = _7;
match _24 {
0 => bb5,
1 => bb2,
2 => bb11,
3 => bb9,
8577334860505752786961142674524711511 => bb14,
_ => bb13
}
}
bb13 = {
_7 = !_16.fld2.0;
_16.fld2.0 = _6;
_8 = _2.2;
_30 = !false;
_32 = _16.fld3;
_16.fld2.2 = -_8;
_13 = _20;
_2.1 = _11 * _3;
_33 = _16.fld0;
_4 = 1930136271_u32 as isize;
_32 = _8 as f64;
_16.fld2.2 = _8;
_32 = _16.fld3;
_14 = _13;
_35 = _1;
_35 = _1;
_26 = [236900391699684535705185805545426371723_u128];
_24 = (-18580797254238204287856232537449136848_i128);
_17 = &_24;
_5 = _10 << _31;
_2.2 = 2429725258_u32 as i32;
Goto(bb12)
}
bb14 = {
_3 = _2.1;
_44 = _20;
_7 = !_40;
_31 = 17064848935460130739_usize;
_42 = _21;
_41 = -_27;
_25 = _41;
_35 = [_5,_10,_10,_10,_10];
_2.2 = 215_u8 as i32;
_29 = 206029103669343633622861144767978555245_u128;
_2.3 = _4 ^ _4;
_15 = -_16.fld3;
_2 = _16.fld2;
(*_34) = [_10,_5,_5,_10,_10];
_27 = (*_37) as f32;
_16.fld2.0 = _2.0;
RET = core::ptr::addr_of!(_49);
_20 = _14;
_25 = _27 * _27;
_37 = core::ptr::addr_of_mut!(_16.fld0);
_48 = [3154372200_u32,336659196_u32];
Goto(bb15)
}
bb15 = {
Call(_51 = dump_var(8_usize, 26_usize, Move(_26), 42_usize, Move(_42), 38_usize, Move(_38), 9_usize, Move(_9)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_51 = dump_var(8_usize, 5_usize, Move(_5), 4_usize, Move(_4), 35_usize, Move(_35), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_51 = dump_var(8_usize, 8_usize, Move(_8), 44_usize, Move(_44), 48_usize, Move(_48), 33_usize, Move(_33)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_51 = dump_var(8_usize, 14_usize, Move(_14), 31_usize, Move(_31), 52_usize, _52, 52_usize, _52), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [i32; 8],mut _2: i32,mut _3: f32,mut _4: f32,mut _5: isize,mut _6: ([bool; 7], (u128, [i32; 8], bool), f32),mut _7: i8,mut _8: [u128; 1]) -> *const *mut i16 {
mir! {
type RET = *const *mut i16;
let _9: char;
let _10: *const (u128, [i32; 8], bool);
let _11: u64;
let _12: u64;
let _13: Adt53;
let _14: [char; 2];
let _15: f64;
let _16: u16;
let _17: i16;
let _18: [i8; 6];
let _19: Adt53;
let _20: bool;
let _21: Adt77;
let _22: (u64, i64);
let _23: bool;
let _24: bool;
let _25: (u64, i64);
let _26: *mut char;
let _27: i32;
let _28: (u8, i32, [u16; 7], u64);
let _29: ();
let _30: ();
{
_5 = 9223372036854775807_isize << _2;
_9 = '\u{7638a}';
_6.1.1 = [_2,_2,_2,_2,_2,_2,_2,_2];
_4 = (-119703172743722019653734199900330806114_i128) as f32;
_6.1.2 = !true;
_3 = _4 * _6.2;
_6.1.1 = _1;
_6.1.2 = _7 <= _7;
_6.1.0 = 163662541638515572435243959778379197882_u128 << _5;
_6.0 = [_6.1.2,_6.1.2,_6.1.2,_6.1.2,_6.1.2,_6.1.2,_6.1.2];
_6.0 = [_6.1.2,_6.1.2,_6.1.2,_6.1.2,_6.1.2,_6.1.2,_6.1.2];
_10 = core::ptr::addr_of!(_6.1);
_6.2 = _3;
_11 = 262729962195510981_u64;
(*_10) = (84821166813502016824947304764658269754_u128, _1, false);
_5 = -9223372036854775807_isize;
_5 = 9223372036854775807_isize;
(*_10).2 = true;
(*_10).2 = false & true;
_10 = core::ptr::addr_of!((*_10));
_12 = _11 * _11;
_5 = _2 as isize;
match _6.1.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
84821166813502016824947304764658269754 => bb6,
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
_2 = _11 as i32;
_13.fld2 = core::ptr::addr_of_mut!(_13.fld1.0);
_6.0 = [_6.1.2,_6.1.2,(*_10).2,(*_10).2,(*_10).2,_6.1.2,(*_10).2];
Goto(bb7)
}
bb7 = {
_13.fld1.1 = _7 ^ _7;
_10 = core::ptr::addr_of!(_6.1);
_13.fld1.1 = _7 | _7;
_16 = 6875_u16 & 25546_u16;
(*_10).1 = [_2,_2,_2,_2,_2,_2,_2,_2];
(*_10) = (89926656318067517270167783802195225872_u128, _1, true);
_14 = [_9,_9];
_13.fld1.2 = (*_10).0 as i32;
_13.fld1.0 = (-16003_i16) ^ 964_i16;
(*_10).1 = [_13.fld1.2,_13.fld1.2,_13.fld1.2,_2,_13.fld1.2,_2,_13.fld1.2,_13.fld1.2];
_15 = 643851343_u32 as f64;
_6.1.2 = true;
(*_10).2 = true;
(*_10).2 = false;
_13.fld1 = (6320_i16, _7, _2, _5);
_4 = _13.fld1.0 as f32;
Goto(bb8)
}
bb8 = {
_9 = '\u{ee336}';
_13.fld0 = [_9,_9,_9,_9];
RET = core::ptr::addr_of!(_13.fld2);
RET = core::ptr::addr_of!((*RET));
_13.fld1.1 = _7 + _7;
_6.1.1 = _1;
(*RET) = core::ptr::addr_of_mut!(_17);
_4 = _3 * _3;
_1 = [_13.fld1.2,_2,_2,_13.fld1.2,_13.fld1.2,_2,_13.fld1.2,_13.fld1.2];
(*_10).1 = [_13.fld1.2,_2,_2,_13.fld1.2,_13.fld1.2,_13.fld1.2,_2,_2];
_13.fld1 = (26938_i16, _7, _2, _5);
_17 = _13.fld1.0;
_13.fld1 = (_17, _7, _2, _5);
_9 = '\u{b3353}';
RET = core::ptr::addr_of!((*RET));
_19.fld1.0 = !_13.fld1.0;
_12 = _11;
_18 = [_13.fld1.1,_7,_7,_13.fld1.1,_13.fld1.1,_13.fld1.1];
_6.0 = [(*_10).2,(*_10).2,_6.1.2,_6.1.2,_6.1.2,(*_10).2,(*_10).2];
_13.fld1.1 = _7 * _7;
_19.fld1 = _13.fld1;
_6.1.1 = [_19.fld1.2,_13.fld1.2,_13.fld1.2,_2,_19.fld1.2,_2,_19.fld1.2,_19.fld1.2];
_2 = !_19.fld1.2;
_19.fld2 = core::ptr::addr_of_mut!(_19.fld1.0);
(*_10) = (106125844450150769670291900740851260338_u128, _1, true);
_6.1.2 = true ^ true;
_19.fld2 = core::ptr::addr_of_mut!(_13.fld1.0);
_13.fld0 = [_9,_9,_9,_9];
(*_10).1 = [_19.fld1.2,_19.fld1.2,_13.fld1.2,_2,_19.fld1.2,_2,_13.fld1.2,_2];
Goto(bb9)
}
bb9 = {
Goto(bb10)
}
bb10 = {
RET = core::ptr::addr_of!(_13.fld2);
(*_10).1 = _1;
_20 = _6.1.2;
_13.fld1 = (_19.fld1.0, _7, _2, _19.fld1.3);
_8 = [_6.1.0];
_19.fld1.1 = _7;
(*_10).0 = !181494976528557390301711177206049601549_u128;
(*_10).1 = _1;
(*_10).1 = _1;
_6.1.1 = _1;
_23 = (*_10).2;
(*_10) = (315374947567735685704983939798825230616_u128, _1, _23);
_2 = !_13.fld1.2;
_5 = !_19.fld1.3;
RET = core::ptr::addr_of!(_19.fld2);
_17 = _19.fld1.0;
_14 = [_9,_9];
(*RET) = Move(_13.fld2);
_7 = !_13.fld1.1;
(*_10).1 = [_19.fld1.2,_19.fld1.2,_19.fld1.2,_2,_2,_19.fld1.2,_2,_13.fld1.2];
_26 = core::ptr::addr_of_mut!(_9);
_13.fld1.1 = -_7;
_13.fld0 = [_9,(*_26),(*_26),(*_26)];
_13.fld1.3 = _5;
_19.fld3 = core::ptr::addr_of!(_28);
Goto(bb11)
}
bb11 = {
Call(_29 = dump_var(9_usize, 17_usize, Move(_17), 2_usize, Move(_2), 12_usize, Move(_12), 1_usize, Move(_1)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_29 = dump_var(9_usize, 20_usize, Move(_20), 7_usize, Move(_7), 5_usize, Move(_5), 30_usize, _30), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: *const *mut i16,mut _2: isize,mut _3: (i16, i8, i32, isize),mut _4: bool,mut _5: [i32; 8],mut _6: (i16, i8, i32, isize),mut _7: isize,mut _8: isize) -> isize {
mir! {
type RET = isize;
let _9: f64;
let _10: u8;
let _11: [i64; 5];
let _12: isize;
let _13: *mut i16;
let _14: i16;
let _15: (u64, i64);
let _16: ();
let _17: ();
{
_5 = [_3.2,_6.2,_6.2,_3.2,_3.2,_3.2,_3.2,_3.2];
RET = '\u{94afb}' as isize;
_9 = 138192226380311880798428171689692553916_i128 as f64;
_3.1 = -_6.1;
_4 = !true;
_5 = [_3.2,_6.2,_3.2,_6.2,_6.2,_3.2,_6.2,_6.2];
_8 = _6.3 >> _2;
RET = _7 << _3.3;
_5 = [_6.2,_3.2,_6.2,_6.2,_6.2,_3.2,_3.2,_3.2];
_3.1 = _6.1 << RET;
_3.0 = _6.0;
_4 = _8 <= _8;
_3 = _6;
_6.2 = _3.2;
_6.2 = 240_u8 as i32;
_3.1 = _6.1 << RET;
_3.2 = _6.2;
_3.3 = '\u{5db87}' as isize;
Goto(bb1)
}
bb1 = {
_6.1 = _3.1 >> _3.1;
_6.2 = -_3.2;
_7 = _2;
_5 = [_6.2,_3.2,_3.2,_3.2,_6.2,_6.2,_3.2,_6.2];
_3 = (_6.0, _6.1, _6.2, _2);
_6.3 = _2;
_2 = _4 as isize;
RET = _8;
_6.2 = _3.2 * _3.2;
_3.0 = _6.0;
_10 = (-6400201177022792162_i64) as u8;
_3.2 = -_6.2;
_3.2 = _6.2 | _6.2;
_3.1 = -_6.1;
_4 = !true;
_6.1 = _3.1 * _3.1;
_11 = [5193832783825874592_i64,8565065502234814285_i64,7447860447956163542_i64,(-306961244355511661_i64),(-3495102401750722254_i64)];
_3.3 = _2;
_2 = _8 & _3.3;
_10 = 158_u8;
_3 = _6;
Call(_6.0 = fn11(_3, _7, RET, _2, _3, RET, _3.1, _3, _3.1, _6.1, _3), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_7 = _2;
_6.2 = _3.2;
_7 = _2;
_8 = _2;
_11 = [(-9160926744731788391_i64),(-5760726996152809565_i64),(-2496077554459115080_i64),3944372950265684898_i64,8939611824615071899_i64];
_12 = -_8;
_3.1 = _6.1;
_3.1 = _6.1;
_13 = core::ptr::addr_of_mut!(_6.0);
RET = -_7;
_6.3 = _8;
_3.3 = -_7;
_6.0 = -_3.0;
RET = 54631_u16 as isize;
_11 = [8140701522921092291_i64,(-3050831146973680838_i64),6495076818393759517_i64,(-6942100009663167427_i64),(-4657936343416404494_i64)];
match _10 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
158 => bb8,
_ => bb7
}
}
bb3 = {
_6.1 = _3.1 >> _3.1;
_6.2 = -_3.2;
_7 = _2;
_5 = [_6.2,_3.2,_3.2,_3.2,_6.2,_6.2,_3.2,_6.2];
_3 = (_6.0, _6.1, _6.2, _2);
_6.3 = _2;
_2 = _4 as isize;
RET = _8;
_6.2 = _3.2 * _3.2;
_3.0 = _6.0;
_10 = (-6400201177022792162_i64) as u8;
_3.2 = -_6.2;
_3.2 = _6.2 | _6.2;
_3.1 = -_6.1;
_4 = !true;
_6.1 = _3.1 * _3.1;
_11 = [5193832783825874592_i64,8565065502234814285_i64,7447860447956163542_i64,(-306961244355511661_i64),(-3495102401750722254_i64)];
_3.3 = _2;
_2 = _8 & _3.3;
_10 = 158_u8;
_3 = _6;
Call(_6.0 = fn11(_3, _7, RET, _2, _3, RET, _3.1, _3, _3.1, _6.1, _3), ReturnTo(bb2), UnwindUnreachable())
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
RET = _8 << _6.1;
(*_13) = 201003568926847037530372708510789636114_u128 as i16;
(*_13) = _3.0;
_12 = _3.3;
_8 = _2;
RET = !_7;
_8 = 337037252206266932661341382230810365648_u128 as isize;
_3.0 = (*_13);
_15 = (15323079586543500908_u64, (-5954249242210464118_i64));
_6.1 = _3.1;
_3 = _6;
_3 = ((*_13), _6.1, _6.2, RET);
Goto(bb9)
}
bb9 = {
Call(_16 = dump_var(10_usize, 7_usize, Move(_7), 4_usize, Move(_4), 10_usize, Move(_10), 8_usize, Move(_8)), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Call(_16 = dump_var(10_usize, 12_usize, Move(_12), 17_usize, _17, 17_usize, _17, 17_usize, _17), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: (i16, i8, i32, isize),mut _2: isize,mut _3: isize,mut _4: isize,mut _5: (i16, i8, i32, isize),mut _6: isize,mut _7: i8,mut _8: (i16, i8, i32, isize),mut _9: i8,mut _10: i8,mut _11: (i16, i8, i32, isize)) -> i16 {
mir! {
type RET = i16;
let _12: *mut u16;
let _13: i16;
let _14: bool;
let _15: Adt50;
let _16: f64;
let _17: isize;
let _18: Adt53;
let _19: u16;
let _20: &'static u16;
let _21: ();
let _22: ();
{
_8.3 = !_3;
_8.1 = -_11.1;
_1.1 = _5.1 + _7;
_11.3 = _5.3 & _4;
_1.1 = 3341256369_u32 as i8;
_11 = (_8.0, _7, _8.2, _3);
_5.2 = -_1.2;
_3 = -_5.3;
RET = 93092164381663873902329732759010573944_u128 as i16;
Call(_11.0 = fn12(_5, _9, _11.1, _8, _8, _11.3, _11.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_8.2 = !_5.2;
_11 = (RET, _9, _5.2, _4);
_1 = (RET, _11.1, _5.2, _11.3);
_13 = 184829880128788829549920778464794825432_u128 as i16;
_11.1 = !_5.1;
_11.1 = _9 >> _1.1;
RET = _8.0 << _10;
_7 = -_8.1;
_11.0 = RET | RET;
_1.3 = _4;
_9 = !_5.1;
_5.0 = RET >> _1.2;
_13 = RET | _5.0;
_4 = _6 & _6;
_10 = !_5.1;
_7 = -_8.1;
_11 = (_5.0, _7, _5.2, _1.3);
RET = 261707896746765825374759184926908593357_u128 as i16;
_5 = (_11.0, _11.1, _1.2, _4);
_14 = _7 == _7;
_16 = _11.2 as f64;
_18.fld1.1 = _7;
Call(_11.0 = fn13(_5, _1, _5, _1, _11.3, _13, _14), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_18.fld1.0 = 4069358812849253729_usize as i16;
_11.2 = (-6221169142676025531_i64) as i32;
_13 = _11.0;
Goto(bb3)
}
bb3 = {
Call(RET = fn14(_5.0, _14, _7, _11.3, _5.3, _8.3, _8.1, _10, _11.3), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_19 = 60234_u16;
_17 = -_11.3;
_20 = &_19;
_11.1 = '\u{7a53a}' as i8;
_18.fld1 = _11;
Goto(bb5)
}
bb5 = {
Call(_21 = dump_var(11_usize, 17_usize, Move(_17), 5_usize, Move(_5), 2_usize, Move(_2), 6_usize, Move(_6)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_21 = dump_var(11_usize, 9_usize, Move(_9), 19_usize, Move(_19), 13_usize, Move(_13), 22_usize, _22), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: (i16, i8, i32, isize),mut _2: i8,mut _3: i8,mut _4: (i16, i8, i32, isize),mut _5: (i16, i8, i32, isize),mut _6: isize,mut _7: i8) -> i16 {
mir! {
type RET = i16;
let _8: ([bool; 7], (u128, [i32; 8], bool), f32);
let _9: i8;
let _10: (Adt50, *const *mut i16, *const char, [i64; 5]);
let _11: (*const [u16; 7], Adt54);
let _12: isize;
let _13: ();
let _14: ();
{
_1.3 = _6 + _6;
RET = _2 as i16;
_5.2 = -_1.2;
_1 = (RET, _4.1, _5.2, _4.3);
_1.2 = '\u{ecad6}' as i32;
_5.0 = _1.0;
_4.1 = -_3;
_1.1 = 507098023_u32 as i8;
_4.3 = _6;
_1.3 = -_6;
_6 = _4.3;
RET = !_5.0;
_5 = (RET, _3, _1.2, _1.3);
_6 = -_1.3;
_9 = 5294140762109928871665333690618378219_u128 as i8;
_4.3 = 100645933900964302404770128931170596478_i128 as isize;
_8.1.2 = true;
_4.3 = _1.3 ^ _5.3;
_2 = !_7;
_10.3 = [(-5101567753518038579_i64),881744458980374569_i64,8856035062109149547_i64,8371115140196869835_i64,3482566073311507697_i64];
_4.1 = _2;
_8.1.0 = !258997540330563930444471979137152036434_u128;
_8.1.1 = [_4.2,_1.2,_5.2,_1.2,_5.2,_4.2,_1.2,_1.2];
Goto(bb1)
}
bb1 = {
Call(_13 = dump_var(12_usize, 4_usize, Move(_4), 9_usize, Move(_9), 2_usize, Move(_2), 1_usize, Move(_1)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn13(mut _1: (i16, i8, i32, isize),mut _2: (i16, i8, i32, isize),mut _3: (i16, i8, i32, isize),mut _4: (i16, i8, i32, isize),mut _5: isize,mut _6: i16,mut _7: bool) -> i16 {
mir! {
type RET = i16;
let _8: &'static Adt50;
let _9: ([bool; 7], (u128, [i32; 8], bool), f32);
let _10: f32;
let _11: usize;
let _12: isize;
let _13: Adt73;
let _14: u32;
let _15: f64;
let _16: char;
let _17: [i64; 5];
let _18: ();
let _19: ();
{
_2.2 = _4.2 & _4.2;
_3 = (_1.0, _4.1, _1.2, _1.3);
_9.2 = 171_u8 as f32;
_5 = '\u{235b6}' as isize;
_4 = (_6, _1.1, _1.2, _3.3);
_9.2 = 3686601525_u32 as f32;
_6 = !_1.0;
_7 = true | false;
_3.3 = _2.1 as isize;
_9.2 = _3.3 as f32;
_4.2 = _3.0 as i32;
_3.1 = -_2.1;
_14 = _4.1 as u32;
Goto(bb1)
}
bb1 = {
RET = _4.0 * _4.0;
_9.1.1 = [_4.2,_4.2,_4.2,_4.2,_4.2,_4.2,_4.2,_1.2];
_9.1.0 = 68405161550638092162772751703849690328_u128;
_12 = _1.3 * _3.3;
_1.1 = _3.1;
_9.1.2 = !_7;
_4.1 = _2.1;
_12 = 14880090680852792809_u64 as isize;
Goto(bb2)
}
bb2 = {
Call(_18 = dump_var(13_usize, 6_usize, Move(_6), 5_usize, Move(_5), 14_usize, Move(_14), 4_usize, Move(_4)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: i16,mut _2: bool,mut _3: i8,mut _4: isize,mut _5: isize,mut _6: isize,mut _7: i8,mut _8: i8,mut _9: isize) -> i16 {
mir! {
type RET = i16;
let _10: &'static Adt50;
let _11: [char; 4];
let _12: (u128, [i32; 8], bool);
let _13: f64;
let _14: (*const [u16; 7], Adt54);
let _15: *mut u16;
let _16: ();
let _17: ();
{
_2 = !true;
RET = _1;
_7 = 64789721553384274822424286394644600665_u128 as i8;
_1 = RET & RET;
_11 = ['\u{61141}','\u{3a8e2}','\u{3b962}','\u{38d2}'];
_2 = !true;
_3 = -_8;
_12.0 = !321479755193413766306683279425502429051_u128;
_12.0 = !287918686297334046534651367848992507433_u128;
_2 = !false;
_13 = (-7777957933724742764_i64) as f64;
_12.2 = !_2;
_8 = _13 as i8;
_2 = _12.2;
Goto(bb1)
}
bb1 = {
Call(_16 = dump_var(14_usize, 4_usize, Move(_4), 6_usize, Move(_6), 8_usize, Move(_8), 5_usize, Move(_5)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Call(_16 = dump_var(14_usize, 3_usize, Move(_3), 17_usize, _17, 17_usize, _17, 17_usize, _17), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: bool,mut _2: i8,mut _3: (u128, [i32; 8], bool),mut _4: ([bool; 7], (u128, [i32; 8], bool), f32),mut _5: i8,mut _6: f32,mut _7: isize,mut _8: i8,mut _9: bool) -> [u128; 1] {
mir! {
type RET = [u128; 1];
let _10: f64;
let _11: bool;
let _12: *mut *const (u128, [i32; 8], bool);
let _13: [i64; 5];
let _14: &'static u16;
let _15: [bool; 5];
let _16: isize;
let _17: Adt77;
let _18: i64;
let _19: *const u128;
let _20: ();
let _21: ();
{
_2 = _8 + _5;
_9 = _1;
_3 = (_4.1.0, _4.1.1, _1);
_7 = (-9223372036854775808_isize) << _5;
_4.1.2 = _9;
_5 = _2 << _2;
Goto(bb1)
}
bb1 = {
_9 = _4.2 >= _4.2;
_6 = 1_usize as f32;
_4.1.0 = 28322_i16 as u128;
_1 = _9;
_6 = _5 as f32;
_2 = 1746852856_i32 as i8;
RET = [_3.0];
_7 = -(-9223372036854775808_isize);
_2 = -_5;
_8 = _5 | _2;
_7 = (-9223372036854775808_isize);
_10 = (-26922683102038199_i64) as f64;
_4.1.1 = _3.1;
_4.1 = (_3.0, _3.1, _3.2);
_3.0 = _4.1.0;
_3.2 = _9;
_4.2 = _6;
_3.0 = _4.1.0 * _4.1.0;
_4.0 = [_4.1.2,_4.1.2,_9,_3.2,_3.2,_9,_9];
_4.1 = (_3.0, _3.1, _1);
_8 = _5 ^ _2;
_4.2 = _6;
_7 = 9223372036854775807_isize;
Goto(bb2)
}
bb2 = {
RET = [_3.0];
_11 = !_9;
_1 = !_3.2;
_3.0 = _4.1.0;
_1 = _4.2 <= _6;
_10 = 5595176685082008090_i64 as f64;
_3.0 = !_4.1.0;
_1 = _2 < _8;
RET = [_3.0];
_13 = [7313925559079458004_i64,4324336526334789733_i64,(-9054671139303116615_i64),(-4624851555039720721_i64),1009939611781538679_i64];
_5 = 2_usize as i8;
_4.0 = [_1,_3.2,_1,_1,_1,_1,_3.2];
_3.2 = !_11;
_9 = !_1;
RET = [_4.1.0];
_3.1 = _4.1.1;
_4.1.0 = !_3.0;
Goto(bb3)
}
bb3 = {
_1 = _9;
_3 = (_4.1.0, _4.1.1, _1);
RET = [_4.1.0];
RET = [_3.0];
_4.1.1 = [(-606860862_i32),786042691_i32,(-1606092745_i32),(-79853183_i32),(-65367497_i32),811140684_i32,2141729581_i32,970418530_i32];
_6 = -_4.2;
_16 = !_7;
_4.1.1 = [841793058_i32,(-2058503995_i32),(-1376768310_i32),1318178762_i32,574536721_i32,1307689433_i32,2141702931_i32,(-60226153_i32)];
_15 = [_9,_3.2,_9,_3.2,_9];
Goto(bb4)
}
bb4 = {
_2 = _8;
_3 = (_4.1.0, _4.1.1, _9);
_10 = _8 as f64;
_3.0 = _4.1.0 & _4.1.0;
_3.1 = [633449581_i32,513615460_i32,848172127_i32,(-537289531_i32),(-338805657_i32),886686984_i32,(-129008590_i32),(-710387163_i32)];
_7 = 65_u8 as isize;
_15 = [_3.2,_9,_1,_11,_3.2];
_8 = _2 | _2;
_4.1.1 = [(-794964211_i32),383194168_i32,601589660_i32,(-1288411767_i32),1503073126_i32,1758970372_i32,(-1339371822_i32),1450117735_i32];
_2 = _8;
_4.1.0 = !_3.0;
_4.1 = _3;
_5 = !_8;
_7 = 863146581_i32 as isize;
RET = [_3.0];
_2 = _5 + _8;
_4.1.1 = _3.1;
RET = [_3.0];
_3 = (_4.1.0, _4.1.1, _4.1.2);
_4.1.0 = _3.0 >> _8;
_2 = -_5;
_11 = _1 ^ _1;
_3.1 = [(-26430348_i32),867572598_i32,1063263505_i32,2010318220_i32,(-1140084141_i32),(-819321898_i32),(-1772496707_i32),(-1273433205_i32)];
_2 = _5 - _8;
_19 = core::ptr::addr_of!(_3.0);
RET = [_4.1.0];
_13 = [6778359019155889453_i64,6987674911286675547_i64,2755581017175950201_i64,6650517856260025211_i64,(-7366737700055069944_i64)];
_1 = _3.2 != _4.1.2;
_13 = [(-6674721737866679366_i64),2277363644384303570_i64,891983283909518766_i64,(-4847162543701437247_i64),3464054319233038241_i64];
Goto(bb5)
}
bb5 = {
Call(_20 = dump_var(15_usize, 13_usize, Move(_13), 5_usize, Move(_5), 7_usize, Move(_7), 15_usize, Move(_15)), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
Call(_20 = dump_var(15_usize, 11_usize, Move(_11), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn16(mut _1: i32,mut _2: isize,mut _3: [i64; 5],mut _4: i32,mut _5: (i16, i8, i32, isize)) -> ([bool; 7], (u128, [i32; 8], bool), f32) {
mir! {
type RET = ([bool; 7], (u128, [i32; 8], bool), f32);
let _6: (*const [u16; 7], Adt54);
let _7: char;
let _8: ();
let _9: ();
{
RET.1.2 = _5.3 >= _2;
_3 = [(-4247288105284437074_i64),2309353354132786870_i64,(-2216338053267529733_i64),(-2209981572380409246_i64),2158625348927336944_i64];
_4 = -_1;
RET.2 = _4 as f32;
RET.1.0 = !23201305805710870357711479916255748244_u128;
_5.3 = 12270149833569271328_u64 as isize;
RET.0 = [RET.1.2,RET.1.2,RET.1.2,RET.1.2,RET.1.2,RET.1.2,RET.1.2];
RET.1.0 = !234408533370218102532847597487563049363_u128;
RET.1.2 = true;
RET.1.1 = [_5.2,_4,_4,_5.2,_1,_1,_1,_4];
RET.2 = _5.0 as f32;
_6.1 = Adt54::Variant3 { fld0: RET.1.0,fld1: RET.1 };
RET.0 = [RET.1.2,Field::<(u128, [i32; 8], bool)>(Variant(_6.1, 3), 1).2,Field::<(u128, [i32; 8], bool)>(Variant(_6.1, 3), 1).2,RET.1.2,RET.1.2,Field::<(u128, [i32; 8], bool)>(Variant(_6.1, 3), 1).2,Field::<(u128, [i32; 8], bool)>(Variant(_6.1, 3), 1).2];
place!(Field::<u128>(Variant(_6.1, 3), 0)) = !Field::<(u128, [i32; 8], bool)>(Variant(_6.1, 3), 1).0;
_5.0 = -(-15630_i16);
RET.1.2 = !Field::<(u128, [i32; 8], bool)>(Variant(_6.1, 3), 1).2;
SetDiscriminant(_6.1, 2);
place!(Field::<(i16, i8, i32, isize)>(Variant(_6.1, 2), 2)).0 = -_5.0;
_5.3 = _2 - _2;
place!(Field::<char>(Variant(_6.1, 2), 1)) = '\u{2066d}';
Goto(bb1)
}
bb1 = {
Call(_8 = dump_var(16_usize, 1_usize, Move(_1), 2_usize, Move(_2), 9_usize, _9, 9_usize, _9), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn17(mut _1: &'static (i64, i16, &'static u64),mut _2: isize,mut _3: i16,mut _4: *const [i8; 6],mut _5: [i64; 5],mut _6: isize,mut _7: isize,mut _8: f32,mut _9: i8,mut _10: (i16, i8, i32, isize),mut _11: u8) -> isize {
mir! {
type RET = isize;
let _12: *const *mut i16;
let _13: isize;
let _14: f64;
let _15: ();
let _16: ();
{
RET = -_6;
RET = 24174_u16 as isize;
_6 = -_2;
Goto(bb1)
}
bb1 = {
_9 = _10.1;
RET = _8 as isize;
_7 = -_6;
RET = _10.0 as isize;
_10.0 = -_3;
_11 = '\u{42f6b}' as u8;
_10.3 = !_2;
_8 = 1_usize as f32;
RET = _2;
RET = _6;
RET = _2 ^ _7;
_2 = !_7;
_3 = _10.0;
_10.3 = -_7;
_10.3 = _2 | RET;
_6 = -RET;
_2 = 34141_u16 as isize;
Goto(bb2)
}
bb2 = {
Call(_15 = dump_var(17_usize, 2_usize, Move(_2), 9_usize, Move(_9), 3_usize, Move(_3), 11_usize, Move(_11)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn18(mut _1: i32,mut _2: [i32; 8]) -> i16 {
mir! {
type RET = i16;
let _3: [u128; 1];
let _4: isize;
let _5: &'static *mut u16;
let _6: *mut *mut u16;
let _7: [i32; 8];
let _8: isize;
let _9: &'static u64;
let _10: i16;
let _11: Adt73;
let _12: [usize; 6];
let _13: *mut *mut u16;
let _14: u128;
let _15: u128;
let _16: f32;
let _17: f64;
let _18: *const [u16; 7];
let _19: *mut bool;
let _20: ();
let _21: ();
{
RET = (-8184147436507380760_i64) as i16;
_1 = !692632819_i32;
_1 = 1148260961_i32 & 1050908148_i32;
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
_1 = -1412902305_i32;
RET = '\u{f7ae2}' as i16;
_1 = 1100631287_i32;
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
RET = (-12263_i16);
RET = (-70_i8) as i16;
_1 = 13033370881403665058_u64 as i32;
RET = (-4046_i16);
_1 = (-73_i8) as i32;
RET = !(-6552_i16);
_3 = [193222699922131876731979455049587380850_u128];
RET = (-5206_i16);
_4 = 25_isize + (-31_isize);
RET = (-9500_i16) | 15265_i16;
RET = !(-11789_i16);
Goto(bb1)
}
bb1 = {
Goto(bb2)
}
bb2 = {
_8 = _4 - _4;
_7 = _2;
RET = 15204319087941954031_u64 as i16;
Goto(bb3)
}
bb3 = {
_7 = _2;
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
RET = -11255_i16;
_7 = [_1,_1,_1,_1,_1,_1,_1,_1];
_4 = _8;
_7 = [_1,_1,_1,_1,_1,_1,_1,_1];
RET = (-20753_i16) ^ (-12739_i16);
RET = (-600_i16) >> _4;
_7 = [_1,_1,_1,_1,_1,_1,_1,_1];
_1 = -(-890591553_i32);
RET = 20716_i16;
match RET {
20716 => bb5,
_ => bb4
}
}
bb4 = {
_8 = _4 - _4;
_7 = _2;
RET = 15204319087941954031_u64 as i16;
Goto(bb3)
}
bb5 = {
_8 = (-145129971815334899713266798753703451160_i128) as isize;
_2 = _7;
RET = 32384_i16 << _4;
_10 = -RET;
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
RET = 9535417688887309215_usize as i16;
_12 = [7_usize,16482445807195956255_usize,3_usize,3_usize,0_usize,7_usize];
_10 = -RET;
_1 = !(-453423699_i32);
_11 = Adt73::Variant0 { fld0: false,fld1: 2_usize };
place!(Field::<usize>(Variant(_11, 0), 1)) = !6_usize;
_8 = _4;
RET = _10;
Goto(bb6)
}
bb6 = {
_1 = -(-1759608045_i32);
_4 = -_8;
_12 = [Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1)];
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
_12 = [Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1)];
_4 = _8 * _8;
_8 = _4;
place!(Field::<usize>(Variant(_11, 0), 1)) = 15004737570334940187_usize;
RET = !_10;
_8 = _4;
_14 = 120723558964184554266970434697388369183_u128;
RET = _1 as i16;
_7 = [_1,_1,_1,_1,_1,_1,_1,_1];
_4 = !_8;
_10 = RET;
_3 = [_14];
_4 = _1 as isize;
place!(Field::<usize>(Variant(_11, 0), 1)) = false as usize;
_8 = _4 | _4;
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
place!(Field::<bool>(Variant(_11, 0), 0)) = !false;
_4 = !_8;
place!(Field::<usize>(Variant(_11, 0), 1)) = !4191216562653881490_usize;
_1 = !134473558_i32;
_7 = _2;
_14 = 88_u8 as u128;
_12 = [Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1)];
_8 = !_4;
Goto(bb7)
}
bb7 = {
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
_1 = 19909208_i32;
place!(Field::<usize>(Variant(_11, 0), 1)) = !0_usize;
_14 = 132076702743725235580548151605088232824_u128 - 139503116218829235422194865869492248994_u128;
place!(Field::<usize>(Variant(_11, 0), 1)) = 18393306695424249862_usize;
_10 = -RET;
place!(Field::<usize>(Variant(_11, 0), 1)) = !3_usize;
match _1 {
0 => bb6,
1 => bb2,
2 => bb8,
3 => bb9,
4 => bb10,
5 => bb11,
19909208 => bb13,
_ => bb12
}
}
bb8 = {
_1 = -(-1759608045_i32);
_4 = -_8;
_12 = [Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1)];
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
_12 = [Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1)];
_4 = _8 * _8;
_8 = _4;
place!(Field::<usize>(Variant(_11, 0), 1)) = 15004737570334940187_usize;
RET = !_10;
_8 = _4;
_14 = 120723558964184554266970434697388369183_u128;
RET = _1 as i16;
_7 = [_1,_1,_1,_1,_1,_1,_1,_1];
_4 = !_8;
_10 = RET;
_3 = [_14];
_4 = _1 as isize;
place!(Field::<usize>(Variant(_11, 0), 1)) = false as usize;
_8 = _4 | _4;
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
place!(Field::<bool>(Variant(_11, 0), 0)) = !false;
_4 = !_8;
place!(Field::<usize>(Variant(_11, 0), 1)) = !4191216562653881490_usize;
_1 = !134473558_i32;
_7 = _2;
_14 = 88_u8 as u128;
_12 = [Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1),Field::<usize>(Variant(_11, 0), 1)];
_8 = !_4;
Goto(bb7)
}
bb9 = {
_8 = (-145129971815334899713266798753703451160_i128) as isize;
_2 = _7;
RET = 32384_i16 << _4;
_10 = -RET;
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
RET = 9535417688887309215_usize as i16;
_12 = [7_usize,16482445807195956255_usize,3_usize,3_usize,0_usize,7_usize];
_10 = -RET;
_1 = !(-453423699_i32);
_11 = Adt73::Variant0 { fld0: false,fld1: 2_usize };
place!(Field::<usize>(Variant(_11, 0), 1)) = !6_usize;
_8 = _4;
RET = _10;
Goto(bb6)
}
bb10 = {
Goto(bb2)
}
bb11 = {
_7 = _2;
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
RET = -11255_i16;
_7 = [_1,_1,_1,_1,_1,_1,_1,_1];
_4 = _8;
_7 = [_1,_1,_1,_1,_1,_1,_1,_1];
RET = (-20753_i16) ^ (-12739_i16);
RET = (-600_i16) >> _4;
_7 = [_1,_1,_1,_1,_1,_1,_1,_1];
_1 = -(-890591553_i32);
RET = 20716_i16;
match RET {
20716 => bb5,
_ => bb4
}
}
bb12 = {
_8 = _4 - _4;
_7 = _2;
RET = 15204319087941954031_u64 as i16;
Goto(bb3)
}
bb13 = {
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
_7 = [_1,_1,_1,_1,_1,_1,_1,_1];
_8 = _4;
_7 = [_1,_1,_1,_1,_1,_1,_1,_1];
_14 = !144391620352195530132821978884932992314_u128;
SetDiscriminant(_11, 2);
place!(Field::<i8>(Variant(_11, 2), 2)) = 7_usize as i8;
place!(Field::<*mut i16>(Variant(_11, 2), 0)) = core::ptr::addr_of_mut!(_10);
_4 = !_8;
_15 = _14 << _10;
_8 = _15 as isize;
_14 = _15;
place!(Field::<*mut i16>(Variant(_11, 2), 0)) = core::ptr::addr_of_mut!(RET);
RET = _1 as i16;
_14 = '\u{f8922}' as u128;
_8 = _4 & _4;
_3 = [_15];
_7 = _2;
_16 = 14157548589941116850_usize as f32;
_2 = _7;
_16 = _15 as f32;
_17 = RET as f64;
_16 = _1 as f32;
_4 = 152_u8 as isize;
_3 = [_14];
match _1 {
19909208 => bb14,
_ => bb1
}
}
bb14 = {
_4 = _8 & _8;
_17 = 7088546466446898919_u64 as f64;
place!(Field::<*mut i16>(Variant(_11, 2), 0)) = core::ptr::addr_of_mut!(_10);
_12 = [14327538745457460605_usize,4497425711314581807_usize,1488825857891066109_usize,2_usize,2_usize,3_usize];
place!(Field::<i8>(Variant(_11, 2), 2)) = 844057202_u32 as i8;
RET = -_10;
_14 = _15;
_14 = !_15;
_2 = [_1,_1,_1,_1,_1,_1,_1,_1];
RET = Field::<i8>(Variant(_11, 2), 2) as i16;
_7 = [_1,_1,_1,_1,_1,_1,_1,_1];
Goto(bb15)
}
bb15 = {
Call(_20 = dump_var(18_usize, 4_usize, Move(_4), 3_usize, Move(_3), 7_usize, Move(_7), 15_usize, Move(_15)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_20 = dump_var(18_usize, 12_usize, Move(_12), 21_usize, _21, 21_usize, _21, 21_usize, _21), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box((-5980316573822418793_i64)), std::hint::black_box(5730811694116637323_usize), std::hint::black_box(9223372036854775807_isize), std::hint::black_box(52237_u16), std::hint::black_box((-166104092255879248139513019674590520834_i128)), std::hint::black_box(1129500843_i32));
                
            }
impl PrintFDebug for Adt21{
	unsafe fn printf_debug(&self){unsafe{printf("Adt21::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt21 {
Variant0{
fld0: u16,
fld1: *mut i16,
fld2: (i16, i8, i32, isize),
fld3: f32,
fld4: u64,
fld5: f64,
fld6: usize,
fld7: i128,

},
Variant1{
fld0: i64,
fld1: (i16, i8, i32, isize),
fld2: isize,
fld3: *mut i16,
fld4: f32,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt43{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt43 {
fld0: u16,
fld1: *const (u8, i32, [u16; 7], u64),
fld2: (i16, i8, i32, isize),
fld3: f64,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf("Adt50::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{".as_ptr() as *const c_char)};
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: [bool; 5],
fld1: u32,
fld2: usize,

},
Variant1{
fld0: Adt43,
fld1: *const *mut i16,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt53{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt53 {
fld0: [char; 4],
fld1: (i16, i8, i32, isize),
fld2: *mut i16,
fld3: *const (u8, i32, [u16; 7], u64),
fld4: Adt50,
}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf("Adt54::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: bool,
fld1: [bool; 5],
fld2: *const (u128, [i32; 8], bool),
fld3: u32,
fld4: i16,
fld5: usize,
fld6: [char; 2],
fld7: i128,

},
Variant1{
fld0: Adt53,
fld1: [i32; 8],
fld2: *const (u8, i32, [u16; 7], u64),
fld3: i32,
fld4: *mut u16,

},
Variant2{
fld0: [u16; 7],
fld1: char,
fld2: (i16, i8, i32, isize),
fld3: i8,
fld4: i16,
fld5: usize,
fld6: Adt50,
fld7: *const (u128, [i32; 8], bool),

},
Variant3{
fld0: u128,
fld1: (u128, [i32; 8], bool),

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
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: Adt53,
fld1: *mut u16,
fld2: isize,
fld3: [bool; 5],
fld4: f64,

},
Variant1{
fld0: [i8; 6],
fld1: [bool; 7],
fld2: *const (u128, [i32; 8], bool),
fld3: Adt43,
fld4: Adt21,

},
Variant2{
fld0: u8,
fld1: Adt43,
fld2: Adt21,
fld3: i8,
fld4: f64,
fld5: [u16; 7],

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
#[derive(Copy,Clone)]pub enum Adt73 {
Variant0{
fld0: bool,
fld1: usize,

},
Variant1{
fld0: u64,
fld1: u128,
fld2: f64,
fld3: [u64; 1],
fld4: [u128; 1],

},
Variant2{
fld0: *mut i16,
fld1: (*const [u16; 7], Adt54),
fld2: i8,

}}
impl PrintFDebug for Adt77{
	unsafe fn printf_debug(&self){unsafe{printf("Adt77::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt77 {
Variant0{
fld0: [i64; 5],
fld1: u32,
fld2: usize,
fld3: *const [i8; 6],

},
Variant1{
fld0: [i8; 6],
fld1: [char; 2],
fld2: *const Adt43,

},
Variant2{
fld0: u8,
fld1: [bool; 7],
fld2: [u64; 1],
fld3: i8,
fld4: (*const [i8; 6], [i32; 8], *const Adt43, *mut i16),
fld5: *mut Adt54,
fld6: Adt53,
fld7: Adt56,

}}

