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
pub fn fn0(mut _1: u128,mut _2: u16,mut _3: isize,mut _4: i8,mut _5: u64,mut _6: i32,mut _7: i64) -> [u32; 6] {
mir! {
type RET = [u32; 6];
let _8: *mut i32;
let _9: [i32; 7];
let _10: Adt52;
let _11: isize;
let _12: (*mut i32, i16, ([i32; 7], u32, *const isize));
let _13: ();
let _14: ();
{
_3 = 99_isize >> 290996159594118226245011883234469732117_u128;
_6 = -(-2005974793_i32);
_4 = -(-114_i8);
_4 = _6 as i8;
_5 = '\u{d4d69}' as u64;
RET = [1688583971_u32,2535807416_u32,2339040439_u32,3327351510_u32,2952610108_u32,598876104_u32];
_7 = (-5463290166492721118_i64) | (-8081507901873172234_i64);
Call(_6 = fn1(RET, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_1 = !24815034663467484787410343378664520393_u128;
RET = [700692558_u32,2669315966_u32,3902189144_u32,4242749433_u32,294164201_u32,1513209421_u32];
_4 = (-85_i8);
_6 = (-1261454304_i32) << _1;
_3 = false as isize;
RET = [491476653_u32,1760606366_u32,1811755067_u32,2850863403_u32,3288225604_u32,3619356584_u32];
RET = [1675683339_u32,3579569703_u32,2017801621_u32,3566570989_u32,54135929_u32,3960968433_u32];
_7 = 2856214872_u32 as i64;
_1 = 116677067025104293662712261820070933632_u128;
match _1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
116677067025104293662712261820070933632 => bb10,
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
_8 = core::ptr::addr_of_mut!(_6);
RET = [239995024_u32,2101465627_u32,386114195_u32,3326808347_u32,304860301_u32,3625007271_u32];
(*_8) = '\u{fc1c1}' as i32;
_4 = (-62_i8);
_4 = 54_i8 + 1_i8;
_8 = core::ptr::addr_of_mut!((*_8));
_9 = [(*_8),_6,(*_8),_6,_6,(*_8),(*_8)];
_7 = 1036588612841912676_i64 & 9055017066263280549_i64;
_8 = core::ptr::addr_of_mut!((*_8));
_2 = !31056_u16;
_9 = [(*_8),_6,(*_8),(*_8),_6,(*_8),_6];
RET = [1012507648_u32,489528816_u32,1122821464_u32,2071744970_u32,2029910927_u32,3120382801_u32];
_3 = _4 as isize;
_3 = 27_isize;
_4 = !27_i8;
_2 = (-132728403564956360723646000178866411090_i128) as u16;
Goto(bb11)
}
bb11 = {
RET = [182813736_u32,1863880946_u32,2150088521_u32,1017397368_u32,4048005243_u32,611150107_u32];
(*_8) = (-1803753891_i32);
_9 = [(*_8),_6,(*_8),_6,(*_8),(*_8),_6];
_7 = (-8511019859500290104_i64) + 4365072235846075289_i64;
_8 = core::ptr::addr_of_mut!((*_8));
_4 = _5 as i8;
(*_8) = !(-601541780_i32);
_4 = _2 as i8;
_8 = core::ptr::addr_of_mut!((*_8));
(*_8) = 67220264971578203507026002437699663391_i128 as i32;
_4 = (-2_i8) >> (*_8);
_1 = 10358204826117603739270956146408368852_u128;
(*_8) = (-41201372_i32);
RET = [2133907332_u32,2185511867_u32,500067984_u32,149621017_u32,1385157899_u32,120086000_u32];
_9 = [_6,(*_8),_6,_6,_6,_6,(*_8)];
_6 = -(-602321869_i32);
_8 = core::ptr::addr_of_mut!((*_8));
match _3 {
0 => bb12,
1 => bb13,
27 => bb15,
_ => bb14
}
}
bb12 = {
_8 = core::ptr::addr_of_mut!(_6);
RET = [239995024_u32,2101465627_u32,386114195_u32,3326808347_u32,304860301_u32,3625007271_u32];
(*_8) = '\u{fc1c1}' as i32;
_4 = (-62_i8);
_4 = 54_i8 + 1_i8;
_8 = core::ptr::addr_of_mut!((*_8));
_9 = [(*_8),_6,(*_8),_6,_6,(*_8),(*_8)];
_7 = 1036588612841912676_i64 & 9055017066263280549_i64;
_8 = core::ptr::addr_of_mut!((*_8));
_2 = !31056_u16;
_9 = [(*_8),_6,(*_8),(*_8),_6,(*_8),_6];
RET = [1012507648_u32,489528816_u32,1122821464_u32,2071744970_u32,2029910927_u32,3120382801_u32];
_3 = _4 as isize;
_3 = 27_isize;
_4 = !27_i8;
_2 = (-132728403564956360723646000178866411090_i128) as u16;
Goto(bb11)
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
(*_8) = -(-2045492802_i32);
_8 = core::ptr::addr_of_mut!((*_8));
_12.2.2 = core::ptr::addr_of!(_11);
Goto(bb16)
}
bb16 = {
Call(_13 = dump_var(0_usize, 9_usize, Move(_9), 6_usize, Move(_6), 1_usize, Move(_1), 3_usize, Move(_3)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: [u32; 6],mut _2: [u32; 6]) -> i32 {
mir! {
type RET = i32;
let _3: f64;
let _4: [u64; 3];
let _5: f64;
let _6: i8;
let _7: u32;
let _8: [i32; 7];
let _9: (i64, i64);
let _10: f64;
let _11: *mut usize;
let _12: [isize; 2];
let _13: i32;
let _14: [u16; 8];
let _15: [isize; 4];
let _16: ();
let _17: ();
{
RET = 227124355_i32 + 1085971659_i32;
RET = (-107841616_i32);
_1 = [1007152066_u32,3884417932_u32,3492753073_u32,2121051179_u32,940221348_u32,3890523184_u32];
_1 = [3329417571_u32,1204670352_u32,2456306120_u32,1417619156_u32,3748392821_u32,3923073103_u32];
RET = !691592053_i32;
RET = 4093485729_u32 as i32;
_2 = [3437843785_u32,755192230_u32,4045320968_u32,4221994355_u32,2886464264_u32,3332435489_u32];
RET = !(-1633000789_i32);
_3 = 6649584399925424479_u64 as f64;
_2 = _1;
RET = (-340374479_i32);
RET = !(-1636295549_i32);
RET = -(-286973949_i32);
_2 = [259181522_u32,3895802204_u32,484099636_u32,2517606617_u32,1944259356_u32,3640745642_u32];
RET = (-1626936337860818162_i64) as i32;
_1 = _2;
RET = (-7465_i16) as i32;
Call(_5 = fn2(_1, _2, _1, _2, _2, _1, _1, _1, _2, _1, _1, _1, _3, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = [3932178204_u32,1916076450_u32,4274647689_u32,2424991675_u32,437215453_u32,2453904764_u32];
_4 = [17717197347988689961_u64,10847794813809049262_u64,4760840747686841146_u64];
RET = 2006179663_i32;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
2006179663 => bb9,
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
_1 = [3291803139_u32,577902632_u32,732011649_u32,4001227823_u32,1276083477_u32,838257760_u32];
_7 = 4022606001_u32 >> RET;
_6 = 73_i8 - (-112_i8);
_1 = [_7,_7,_7,_7,_7,_7];
RET = (-1588002465_i32);
_9.0 = 468785391917578252_i64 << _6;
RET = _5 as i32;
_3 = 169500170545919124113773159726771643874_u128 as f64;
_9.1 = _9.0 * _9.0;
RET = 945959903_i32;
_4 = [14057240841533462189_u64,1889489668447581147_u64,6352121056140634611_u64];
_7 = 974807936_u32;
_3 = -_5;
_10 = _5;
_2 = _1;
_9 = (7831714156957815513_i64, (-6683156332190863199_i64));
_10 = _3 * _5;
_12 = [(-9223372036854775808_isize),(-9223372036854775808_isize)];
match RET {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
945959903 => bb18,
_ => bb17
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
_2 = [3932178204_u32,1916076450_u32,4274647689_u32,2424991675_u32,437215453_u32,2453904764_u32];
_4 = [17717197347988689961_u64,10847794813809049262_u64,4760840747686841146_u64];
RET = 2006179663_i32;
match RET {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
2006179663 => bb9,
_ => bb8
}
}
bb18 = {
_1 = _2;
_8 = [RET,RET,RET,RET,RET,RET,RET];
_5 = _3 - _3;
_9.1 = 4292352122416362205_u64 as i64;
_5 = 32523_i16 as f64;
_9 = ((-6820709645830511617_i64), 1090105450677972796_i64);
_6 = 64_u8 as i8;
_5 = _10;
_12 = [(-97_isize),5_isize];
_7 = 2981428141_u32;
_14 = [6980_u16,4137_u16,18045_u16,58462_u16,43940_u16,23434_u16,12248_u16,11271_u16];
_5 = _10 * _10;
_12 = [9223372036854775807_isize,(-9223372036854775808_isize)];
_5 = _10;
RET = 831927252_i32 << _9.0;
_2 = [_7,_7,_7,_7,_7,_7];
_15 = [9223372036854775807_isize,109_isize,(-73_isize),(-18_isize)];
RET = !2076260778_i32;
_2 = [_7,_7,_7,_7,_7,_7];
_9.1 = _9.0 | _9.0;
_15 = [9223372036854775807_isize,(-9223372036854775808_isize),(-40_isize),9223372036854775807_isize];
_10 = _5 + _3;
Goto(bb19)
}
bb19 = {
Call(_16 = dump_var(1_usize, 9_usize, Move(_9), 6_usize, Move(_6), 7_usize, Move(_7), 8_usize, Move(_8)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_16 = dump_var(1_usize, 12_usize, Move(_12), 17_usize, _17, 17_usize, _17, 17_usize, _17), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: [u32; 6],mut _2: [u32; 6],mut _3: [u32; 6],mut _4: [u32; 6],mut _5: [u32; 6],mut _6: [u32; 6],mut _7: [u32; 6],mut _8: [u32; 6],mut _9: [u32; 6],mut _10: [u32; 6],mut _11: [u32; 6],mut _12: [u32; 6],mut _13: f64,mut _14: [u32; 6]) -> f64 {
mir! {
type RET = f64;
let _15: f32;
let _16: [isize; 4];
let _17: f32;
let _18: f32;
let _19: [u16; 8];
let _20: char;
let _21: &'static i64;
let _22: isize;
let _23: [u32; 6];
let _24: Adt51;
let _25: [u16; 8];
let _26: f64;
let _27: bool;
let _28: u8;
let _29: char;
let _30: Adt49;
let _31: [u64; 3];
let _32: usize;
let _33: [isize; 4];
let _34: (i8, *const (isize, bool, (u8, i32, *const i16)));
let _35: [u16; 8];
let _36: [u64; 3];
let _37: f64;
let _38: &'static i64;
let _39: (i64, i64);
let _40: i8;
let _41: [i128; 8];
let _42: u32;
let _43: isize;
let _44: char;
let _45: *mut usize;
let _46: ();
let _47: ();
{
_13 = (-2233355877322470997_i64) as f64;
_16 = [(-9223372036854775808_isize),5_isize,(-27_isize),9223372036854775807_isize];
_13 = 23060_u16 as f64;
Call(_11 = fn3(_10, _9, _16, _12, _13, _2, _13, _14), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_17 = 179241783801697842226936940706905204978_u128 as f32;
_10 = [126327865_u32,1037227727_u32,3949692518_u32,2043875363_u32,887948708_u32,3821116762_u32];
RET = _13 * _13;
_16 = [9223372036854775807_isize,(-80_isize),(-9_isize),(-9223372036854775808_isize)];
_18 = _17 - _17;
_19 = [35295_u16,25347_u16,193_u16,55291_u16,18382_u16,46489_u16,46661_u16,25795_u16];
RET = -_13;
_13 = RET - RET;
_4 = [39121378_u32,4176281659_u32,2634345879_u32,3010160174_u32,2705612122_u32,484791059_u32];
_10 = [3533834693_u32,1701555819_u32,53736736_u32,3232540145_u32,239027351_u32,1005389225_u32];
_6 = [4051208283_u32,3937952217_u32,2176238745_u32,713840544_u32,408865423_u32,672917757_u32];
_11 = _1;
_18 = 144_u8 as f32;
_15 = _18;
_3 = _4;
_6 = _9;
_23 = _5;
_20 = '\u{bd3f4}';
_7 = [3860610611_u32,1277495796_u32,3508156703_u32,3067737571_u32,4141612599_u32,1258934477_u32];
_10 = _11;
_15 = 264755080933456104654622188346557320620_u128 as f32;
Call(_5 = core::intrinsics::transmute(_14), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_22 = 3232276527502510982_usize as isize;
_25 = _19;
_19 = _25;
_24.fld1 = 150_u8;
_17 = 183146208_i32 as f32;
_22 = -(-9223372036854775808_isize);
_24 = Adt51 { fld0: 16150_i16,fld1: 180_u8 };
_5 = [748933654_u32,1904580059_u32,3299065140_u32,3332204505_u32,150139313_u32,3185053530_u32];
_24 = Adt51 { fld0: (-28675_i16),fld1: 59_u8 };
_10 = _23;
RET = _13;
match _24.fld0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
340282366920938463463374607431768182781 => bb7,
_ => bb6
}
}
bb3 = {
_17 = 179241783801697842226936940706905204978_u128 as f32;
_10 = [126327865_u32,1037227727_u32,3949692518_u32,2043875363_u32,887948708_u32,3821116762_u32];
RET = _13 * _13;
_16 = [9223372036854775807_isize,(-80_isize),(-9_isize),(-9223372036854775808_isize)];
_18 = _17 - _17;
_19 = [35295_u16,25347_u16,193_u16,55291_u16,18382_u16,46489_u16,46661_u16,25795_u16];
RET = -_13;
_13 = RET - RET;
_4 = [39121378_u32,4176281659_u32,2634345879_u32,3010160174_u32,2705612122_u32,484791059_u32];
_10 = [3533834693_u32,1701555819_u32,53736736_u32,3232540145_u32,239027351_u32,1005389225_u32];
_6 = [4051208283_u32,3937952217_u32,2176238745_u32,713840544_u32,408865423_u32,672917757_u32];
_11 = _1;
_18 = 144_u8 as f32;
_15 = _18;
_3 = _4;
_6 = _9;
_23 = _5;
_20 = '\u{bd3f4}';
_7 = [3860610611_u32,1277495796_u32,3508156703_u32,3067737571_u32,4141612599_u32,1258934477_u32];
_10 = _11;
_15 = 264755080933456104654622188346557320620_u128 as f32;
Call(_5 = core::intrinsics::transmute(_14), ReturnTo(bb2), UnwindUnreachable())
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
_11 = [4093177673_u32,3566028891_u32,2790486309_u32,1415419460_u32,196991190_u32,3096064268_u32];
_10 = _14;
_26 = RET + _13;
_16 = [_22,_22,_22,_22];
RET = -_26;
_13 = _26 * _26;
_9 = [1940542671_u32,161869444_u32,2436053372_u32,1863279347_u32,2784751465_u32,1382510504_u32];
Goto(bb8)
}
bb8 = {
_5 = [402956883_u32,1522512758_u32,2543609234_u32,1336490859_u32,1612985703_u32,1968737153_u32];
_28 = 78103813188618506056688285415598622195_u128 as u8;
_8 = _10;
_30.fld2.0 = core::ptr::addr_of!(_24.fld0);
_23 = _9;
_30.fld5 = (-918120927_i32) + (-347090844_i32);
Goto(bb9)
}
bb9 = {
_30.fld1 = _30.fld2.0;
_1 = [318796159_u32,2133503813_u32,1482598747_u32,1667683937_u32,1178018014_u32,2610163491_u32];
_29 = _20;
_32 = 6_usize;
_28 = 63_i8 as u8;
_31 = [9527230164835203864_u64,18110506581990543984_u64,9069111604551858861_u64];
_25[_32] = _30.fld5 as u16;
_30.fld0 = false;
_18 = _15 * _17;
_11 = [2604042987_u32,1869376660_u32,1432258112_u32,3148326574_u32,3494348524_u32,391306034_u32];
_7 = [1371379174_u32,1084079810_u32,460872121_u32,3725260246_u32,897837506_u32,343126978_u32];
_5 = [966631300_u32,1958097456_u32,736742611_u32,1386308363_u32,2367773599_u32,1570649512_u32];
_33 = _16;
_31 = [17919255116274896041_u64,8088493722671954392_u64,558313795799595842_u64];
_24.fld0 = _19[_32] as i16;
_24 = Adt51 { fld0: 7088_i16,fld1: _28 };
_30.fld1 = core::ptr::addr_of!(_24.fld0);
Goto(bb10)
}
bb10 = {
_26 = -_13;
_14 = [2183096020_u32,2868105753_u32,1332002141_u32,2699697277_u32,455509768_u32,4036266939_u32];
_10 = [1326036706_u32,2378952250_u32,3139128031_u32,334589362_u32,3304209334_u32,4011655835_u32];
_24.fld1 = _28 | _28;
_35[_32] = !_25[_32];
_30.fld5 = (-1424589779_i32) - (-815405388_i32);
_34.0 = 331411975323294070407599940685755045211_u128 as i8;
_30.fld2.0 = _30.fld1;
_31 = [15481601863371938685_u64,10453969508211788224_u64,3670456758450801643_u64];
_37 = _13;
_7 = [3829469125_u32,3066088625_u32,808097408_u32,4280061248_u32,3224695281_u32,292716452_u32];
_2 = _9;
_30.fld4 = [17709466998325839935_u64,14951268985296218249_u64,6310079245595998684_u64];
_15 = _17 - _18;
_30.fld2 = (_30.fld1,);
_30.fld4 = [10191342065769586973_u64,12847181833451249452_u64,1350280923268449258_u64];
_19 = [_25[_32],_35[_32],_25[_32],_35[_32],_25[_32],_25[_32],_35[_32],_25[_32]];
Goto(bb11)
}
bb11 = {
_24.fld0 = _18 as i16;
_4 = [3601786732_u32,224124226_u32,3450120590_u32,1306695535_u32,1522663926_u32,3945676445_u32];
_24.fld1 = 460993365308027256_i64 as u8;
_26 = -RET;
_30.fld4 = [1720307386369335531_u64,5377346743932251811_u64,6999185217869885708_u64];
_16 = [_22,_22,_22,_22];
_26 = _13 + _13;
_18 = _17;
_30.fld2 = (_30.fld1,);
_30.fld3 = _34.0;
_40 = (-5000729583479200431_i64) as i8;
_29 = _20;
Goto(bb12)
}
bb12 = {
_41[_32] = (-164801797890999316756646589452848588452_i128) << _24.fld0;
_35[_32] = _15 as u16;
_24 = Adt51 { fld0: (-2847_i16),fld1: _28 };
RET = -_26;
_30.fld5 = 1987548778_i32 << _24.fld0;
_35 = _25;
_24.fld0 = (-29960_i16);
_35 = [_19[_32],_19[_32],_25[_32],_25[_32],_25[_32],_19[_32],_19[_32],_19[_32]];
_33 = [_22,_22,_22,_22];
_10 = [2026500415_u32,1463876408_u32,2758898980_u32,3389211522_u32,3516535445_u32,2914791595_u32];
_26 = _37;
_35 = [_25[_32],_25[_32],_25[_32],_25[_32],_25[_32],_25[_32],_19[_32],_19[_32]];
_7 = _23;
_30.fld0 = !true;
_23 = [4213456788_u32,227171690_u32,2825180802_u32,2399830996_u32,1464548421_u32,387678165_u32];
RET = -_13;
_35 = _25;
_38 = &_39.1;
_24.fld0 = !16467_i16;
_39.0 = !9039072511223711263_i64;
_30.fld2 = (_30.fld1,);
_24 = Adt51 { fld0: (-11883_i16),fld1: _28 };
_30.fld4 = [14393189629093695114_u64,11306696671774640965_u64,16778222912152204797_u64];
Goto(bb13)
}
bb13 = {
_11 = _4;
_20 = _29;
_31 = [13644042625129534098_u64,2076723839420630379_u64,11851296774653415833_u64];
Call(_42 = core::intrinsics::bswap(2209030606_u32), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_36 = [10112301182412535325_u64,12122203581084016234_u64,6805863680339707960_u64];
Goto(bb15)
}
bb15 = {
Call(_46 = dump_var(2_usize, 14_usize, Move(_14), 42_usize, Move(_42), 4_usize, Move(_4), 5_usize, Move(_5)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_46 = dump_var(2_usize, 7_usize, Move(_7), 12_usize, Move(_12), 28_usize, Move(_28), 1_usize, Move(_1)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_46 = dump_var(2_usize, 3_usize, Move(_3), 11_usize, Move(_11), 32_usize, Move(_32), 36_usize, Move(_36)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_46 = dump_var(2_usize, 16_usize, Move(_16), 35_usize, Move(_35), 47_usize, _47, 47_usize, _47), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3(mut _1: [u32; 6],mut _2: [u32; 6],mut _3: [isize; 4],mut _4: [u32; 6],mut _5: f64,mut _6: [u32; 6],mut _7: f64,mut _8: [u32; 6]) -> [u32; 6] {
mir! {
type RET = [u32; 6];
let _9: isize;
let _10: bool;
let _11: usize;
let _12: *const u32;
let _13: (i64, i64);
let _14: (i64, i64);
let _15: isize;
let _16: f32;
let _17: ([i32; 7], u32, *const isize);
let _18: isize;
let _19: f64;
let _20: i8;
let _21: *const u32;
let _22: [u32; 6];
let _23: f32;
let _24: char;
let _25: u16;
let _26: [u16; 8];
let _27: [i128; 8];
let _28: Adt48;
let _29: (u8, i32, *const i16);
let _30: bool;
let _31: i32;
let _32: isize;
let _33: isize;
let _34: Adt59;
let _35: [i32; 7];
let _36: u64;
let _37: Adt44;
let _38: char;
let _39: u32;
let _40: *mut usize;
let _41: f32;
let _42: [isize; 2];
let _43: [u16; 8];
let _44: f64;
let _45: f32;
let _46: [isize; 4];
let _47: [i32; 7];
let _48: [isize; 2];
let _49: ();
let _50: ();
{
_6 = [767709031_u32,2723180480_u32,244407012_u32,3230459178_u32,2170134305_u32,1522035870_u32];
_6 = [446561644_u32,777421207_u32,3899688345_u32,2491232729_u32,442290163_u32,371017513_u32];
_2 = [3936961491_u32,3983109154_u32,3939400283_u32,16649819_u32,2896302057_u32,3146143657_u32];
_7 = _5;
RET = [4054288061_u32,4220954836_u32,4089509985_u32,4160192510_u32,4090747555_u32,2800959995_u32];
_5 = -_7;
_4 = [3412696543_u32,1923958670_u32,1462592727_u32,1773526534_u32,4015127639_u32,257653856_u32];
_8 = [526694580_u32,2770513345_u32,2114455618_u32,2835424116_u32,3078377414_u32,2264981644_u32];
_6 = [1688268810_u32,345211868_u32,3537358551_u32,900231618_u32,80605330_u32,2261734164_u32];
_2 = [4249942734_u32,4108256204_u32,68549559_u32,546678103_u32,3551840777_u32,3788632671_u32];
_7 = -_5;
_3 = [44_isize,(-9223372036854775808_isize),9223372036854775807_isize,78_isize];
Call(_2 = fn4(_1, _3, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = [4294828833_u32,804580142_u32,787023070_u32,2100596731_u32,1983490284_u32,1750949526_u32];
_5 = _7;
_11 = 3_usize & 7_usize;
_9 = -(-9223372036854775808_isize);
_2 = [3682068123_u32,1532419138_u32,3507154869_u32,1975654978_u32,2703034433_u32,715362989_u32];
_13.1 = 5300735308178587252_i64;
_7 = _5;
match _13.1 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
5300735308178587252 => bb7,
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
RET = [3991595142_u32,950183418_u32,2879735803_u32,918451873_u32,1756181606_u32,2974863310_u32];
RET = [1399682836_u32,1802403352_u32,109549667_u32,565996435_u32,1481683823_u32,404049381_u32];
_10 = true;
_13.1 = !5554434232278232142_i64;
_9 = !(-9223372036854775808_isize);
_14.0 = -_13.1;
_7 = _11 as f64;
_11 = !5618034363454192583_usize;
RET = _2;
_10 = true;
_4 = [1217626134_u32,2578359323_u32,700343531_u32,2348110441_u32,1681767506_u32,4166823060_u32];
_6 = [3300705539_u32,2023642597_u32,2540091421_u32,2117592656_u32,3585549636_u32,549387439_u32];
_2 = [3704801326_u32,3062957472_u32,3702248279_u32,2051485152_u32,1336356339_u32,710434648_u32];
_14.1 = !_14.0;
_12 = core::ptr::addr_of!(_17.1);
_7 = _14.0 as f64;
_13 = (_14.0, _14.1);
Call(_9 = fn11(_3, _6, _2, _3, _8, _1, _2), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_17.1 = 1827884711_u32;
_13.1 = _9 as i64;
_5 = 49_u8 as f64;
_15 = _5 as isize;
RET = [(*_12),(*_12),(*_12),_17.1,_17.1,(*_12)];
_5 = _7 + _7;
_17.0 = [1019737818_i32,(-445906464_i32),(-1614333352_i32),1725019068_i32,(-587301190_i32),(-84389314_i32),1588957566_i32];
_6 = _2;
_16 = _13.0 as f32;
_8 = [_17.1,(*_12),(*_12),_17.1,(*_12),(*_12)];
_10 = !true;
_14 = (_13.0, _13.1);
_19 = _7;
_1 = [(*_12),(*_12),_17.1,(*_12),(*_12),(*_12)];
_17.1 = 3082197818_u32 + 3992640267_u32;
_1 = _6;
_13 = _14;
_15 = -_9;
_6 = [(*_12),(*_12),_17.1,_17.1,(*_12),(*_12)];
_6 = _2;
_17.2 = core::ptr::addr_of!(_9);
Call(_22 = fn14(_3, RET, _17.0, _14.1, _4, _3, _6, _17, _17, _17, _3, _17.0, _12, _17, (*_12), _17), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_17.2 = core::ptr::addr_of!(_15);
_13.1 = _14.0;
_17.1 = !1185743224_u32;
_20 = '\u{4d8cd}' as i8;
_17.1 = (-1743001326_i32) as u32;
_11 = 446273019327873693_usize;
_22 = [(*_12),_17.1,(*_12),_17.1,_17.1,_17.1];
_13 = (_14.0, _14.1);
_13.1 = _10 as i64;
_10 = true;
_16 = _13.1 as f32;
_13.0 = _11 as i64;
Goto(bb10)
}
bb10 = {
_4 = [(*_12),_17.1,(*_12),(*_12),(*_12),_17.1];
_19 = _5 - _5;
_21 = _12;
_24 = '\u{8a59d}';
_7 = 9043239516734230858_u64 as f64;
RET = [(*_21),(*_21),_17.1,_17.1,(*_21),_17.1];
_4 = RET;
_12 = _21;
_4 = [_17.1,(*_12),(*_21),(*_12),(*_21),(*_21)];
_5 = _7;
_21 = core::ptr::addr_of!((*_21));
_26 = [58541_u16,65123_u16,51088_u16,57973_u16,19967_u16,44264_u16,9190_u16,62650_u16];
_18 = _15;
(*_12) = _20 as u32;
_5 = _7;
_13.0 = _14.1 - _14.1;
_14.0 = _13.0;
_22 = [(*_12),(*_21),(*_21),_17.1,_17.1,(*_21)];
Goto(bb11)
}
bb11 = {
_13.0 = _14.1 >> (*_12);
_21 = core::ptr::addr_of!((*_21));
_23 = _16;
_13 = _14;
RET = [(*_12),(*_12),(*_12),(*_21),(*_21),(*_21)];
RET = [(*_21),(*_12),(*_12),(*_12),(*_12),(*_21)];
_25 = 12398_u16 << _18;
_18 = _15;
_13.0 = _23 as i64;
_19 = _16 as f64;
_14 = (_13.1, _13.0);
_29.0 = _13.1 as u8;
_26 = [_25,_25,_25,_25,_25,_25,_25,_25];
_21 = core::ptr::addr_of!((*_12));
(*_12) = _16 as u32;
_27 = [(-123501961163460997251258577546012765268_i128),(-133153985214938889137341344206261495148_i128),164888251999909455157863922819586136696_i128,(-153547385612321334501592248143293552758_i128),(-40883472629634515226728450588996790107_i128),89488929169204058707719175570906727675_i128,(-143507156814350679106480783664689018992_i128),(-40447649244420699134017447000869515456_i128)];
_15 = -_9;
_27 = [3830554375115932124366505885482637954_i128,(-12730761024892926053277717113299794956_i128),(-43677307836897321117915714735301120348_i128),86032642091734329758448332298978685859_i128,2435954423539397399063813767741379366_i128,(-53437319902644286202910721661257493239_i128),129382610217312471310756381609426596520_i128,161327184070995043209106247776748091425_i128];
_29.1 = (-1846352470_i32) + (-690114790_i32);
_27 = [166380121927233145288621235072973806787_i128,104078620867305140921743647504036580195_i128,(-153506235566187766242330793016320819421_i128),(-55713902825086444146942865748277841205_i128),(-108953282780425067223491950623508181674_i128),(-78841611084473714187560379054769468761_i128),18128926337988186331838821538380111140_i128,143848478504172059169405695663982337026_i128];
_19 = -_7;
Call((*_21) = core::intrinsics::transmute(_29.1), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_29.1 = (-1730291127_i32) >> _15;
_4 = [(*_21),(*_12),_17.1,(*_21),_17.1,(*_12)];
_32 = _18 * _18;
_15 = _18;
_14 = _13;
_22 = _6;
_29.1 = 531595105_i32;
_18 = _9;
_2 = [(*_21),(*_21),(*_21),(*_21),(*_12),_17.1];
_21 = core::ptr::addr_of!((*_21));
_4 = [(*_21),(*_12),(*_12),(*_12),(*_12),(*_21)];
_3 = [_9,_18,_15,_9];
_27 = [80966316485965433346613137144244572584_i128,94194605861595147604496358443985039468_i128,(-90404713282816040063282222412215053598_i128),(-104115113874516899314742985204702446370_i128),(-65362977554324396737797423144216556857_i128),76076887741436530382511841906114077078_i128,102382937010543150134240361768273033345_i128,129119364140217080662698217195309004552_i128];
Goto(bb13)
}
bb13 = {
_36 = 14098250916339304263_u64;
_13.0 = -_14.1;
_16 = _23 + _23;
_35 = _17.0;
_22 = [(*_21),_17.1,(*_12),(*_12),(*_21),(*_12)];
_10 = false;
_29.0 = 116_u8 << (*_21);
_41 = _19 as f32;
_17.1 = 3770740519_u32 ^ 887246442_u32;
_13.0 = _13.1 & _14.1;
_25 = _41 as u16;
_29.1 = !523934513_i32;
_40 = core::ptr::addr_of_mut!(_11);
_20 = _17.1 as i8;
_32 = _15 >> _15;
match (*_40) {
0 => bb3,
1 => bb14,
446273019327873693 => bb16,
_ => bb15
}
}
bb14 = {
_29.1 = (-1730291127_i32) >> _15;
_4 = [(*_21),(*_12),_17.1,(*_21),_17.1,(*_12)];
_32 = _18 * _18;
_15 = _18;
_14 = _13;
_22 = _6;
_29.1 = 531595105_i32;
_18 = _9;
_2 = [(*_21),(*_21),(*_21),(*_21),(*_12),_17.1];
_21 = core::ptr::addr_of!((*_21));
_4 = [(*_21),(*_12),(*_12),(*_12),(*_12),(*_21)];
_3 = [_9,_18,_15,_9];
_27 = [80966316485965433346613137144244572584_i128,94194605861595147604496358443985039468_i128,(-90404713282816040063282222412215053598_i128),(-104115113874516899314742985204702446370_i128),(-65362977554324396737797423144216556857_i128),76076887741436530382511841906114077078_i128,102382937010543150134240361768273033345_i128,129119364140217080662698217195309004552_i128];
Goto(bb13)
}
bb15 = {
Return()
}
bb16 = {
_14 = (_13.0, _13.1);
_40 = core::ptr::addr_of_mut!(_11);
(*_21) = 2837663065_u32;
_14.1 = _14.0;
_39 = !_17.1;
_20 = (-51_i8) + (-25_i8);
_14.0 = _36 as i64;
_9 = -_32;
_33 = _9 ^ _9;
_24 = '\u{ce5af}';
_33 = !_18;
_32 = _9;
_41 = _23 - _16;
_38 = _24;
_43 = [_25,_25,_25,_25,_25,_25,_25,_25];
_30 = _5 != _5;
_15 = _18 & _33;
_42 = [_9,_32];
_16 = _41;
RET = _6;
_38 = _24;
RET = [(*_12),(*_12),_39,(*_21),(*_12),_17.1];
Goto(bb17)
}
bb17 = {
Call(_49 = dump_var(3_usize, 43_usize, Move(_43), 4_usize, Move(_4), 33_usize, Move(_33), 2_usize, Move(_2)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_49 = dump_var(3_usize, 35_usize, Move(_35), 22_usize, Move(_22), 14_usize, Move(_14), 3_usize, Move(_3)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_49 = dump_var(3_usize, 20_usize, Move(_20), 27_usize, Move(_27), 30_usize, Move(_30), 26_usize, Move(_26)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_49 = dump_var(3_usize, 8_usize, Move(_8), 6_usize, Move(_6), 50_usize, _50, 50_usize, _50), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: [u32; 6],mut _2: [isize; 4],mut _3: [isize; 4]) -> [u32; 6] {
mir! {
type RET = [u32; 6];
let _4: [isize; 4];
let _5: (isize, bool, (u8, i32, *const i16));
let _6: char;
let _7: Adt44;
let _8: isize;
let _9: i8;
let _10: *const u32;
let _11: char;
let _12: isize;
let _13: [isize; 2];
let _14: [u32; 6];
let _15: [u64; 3];
let _16: [i128; 8];
let _17: u128;
let _18: [u64; 3];
let _19: u16;
let _20: char;
let _21: Adt51;
let _22: [u32; 6];
let _23: i8;
let _24: char;
let _25: [i128; 8];
let _26: [isize; 4];
let _27: Adt51;
let _28: ();
let _29: ();
{
_1 = [2283430027_u32,1918047940_u32,961440718_u32,3246518761_u32,1482951287_u32,1849964583_u32];
RET = _1;
RET = [1131885132_u32,3335232579_u32,3687673563_u32,451995383_u32,2098031386_u32,4191560179_u32];
_1 = [3404387111_u32,3649927452_u32,1692174298_u32,72835421_u32,3873638852_u32,4014588201_u32];
_2 = [96_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_3 = [(-44_isize),9223372036854775807_isize,86_isize,9223372036854775807_isize];
_3 = [(-9223372036854775808_isize),(-32_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_1 = RET;
RET = [3369557298_u32,2320146479_u32,1189822388_u32,1255234478_u32,1050973281_u32,3026815385_u32];
RET = [2108570738_u32,977580704_u32,1493351113_u32,3048762520_u32,1758539310_u32,3403647969_u32];
_2 = _3;
_4 = _2;
_4 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
_1 = [3239516243_u32,2916234322_u32,1914361915_u32,1796408196_u32,2526764119_u32,3964845181_u32];
_2 = [9223372036854775807_isize,28_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = [172367224_u32,3458526403_u32,3065837612_u32,3578229192_u32,3818226071_u32,3574511509_u32];
RET = _1;
_4 = _2;
_2 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_4 = [122_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
RET = [3953793490_u32,1311593040_u32,1920423872_u32,2356878472_u32,900373114_u32,1593055009_u32];
_1 = RET;
_1 = RET;
Goto(bb1)
}
bb1 = {
_4 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_6 = '\u{80c94}';
_2 = [(-67_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_5.2.0 = 216_u8;
_5.0 = 43_isize * 104_isize;
RET = [3291993854_u32,3759659496_u32,3307603423_u32,3875210928_u32,1240154300_u32,2050320521_u32];
_5.0 = 9223372036854775807_isize;
_2 = [_5.0,_5.0,_5.0,_5.0];
_2 = [_5.0,_5.0,_5.0,_5.0];
_5.0 = (-9223372036854775808_isize) * (-46_isize);
_5.2.1 = (-1102720482_i32);
_5.2.0 = 120_u8 & 101_u8;
RET = [3652725659_u32,3645125912_u32,2676867127_u32,4291027147_u32,2461717924_u32,1044706333_u32];
_1 = [1870329990_u32,4228031261_u32,876816668_u32,1047412370_u32,1687205046_u32,1842118367_u32];
_5.2.1 = -(-1910248877_i32);
RET = _1;
_5.1 = false;
_8 = (-140365153608911925996780155812194378395_i128) as isize;
RET = [3225932617_u32,1703960849_u32,3841438984_u32,2098393620_u32,417602564_u32,204918006_u32];
RET = _1;
RET = _1;
_5.1 = _6 <= _6;
_5.1 = _5.0 <= _5.0;
RET = [1010805951_u32,571099775_u32,391709742_u32,4293745404_u32,2592941839_u32,2872082676_u32];
Goto(bb2)
}
bb2 = {
_6 = '\u{e5752}';
_5.2.0 = (-13011884354035507047811729560341424933_i128) as u8;
_5.0 = _8 * _8;
RET = [2135219419_u32,708142049_u32,1150486090_u32,2121741506_u32,3731389478_u32,1693122105_u32];
_4 = [_5.0,_5.0,_8,_5.0];
_5.1 = false;
_8 = -_5.0;
RET = [1268150331_u32,920750667_u32,1105776141_u32,959686508_u32,224514647_u32,2459439253_u32];
RET = _1;
_5.2.0 = _5.1 as u8;
_5.1 = !false;
RET = [1692598386_u32,1959946278_u32,3705138436_u32,3565540993_u32,247693531_u32,3752878207_u32];
_1 = [225648700_u32,1928246837_u32,3912872874_u32,31911256_u32,2856854386_u32,3183927044_u32];
_5.0 = _8 - _8;
_9 = 32_i8 & 98_i8;
_4 = _3;
_5.2.0 = 203_u8;
_4 = [_8,_8,_5.0,_5.0];
_2 = [_5.0,_8,_5.0,_8];
_2 = [_5.0,_8,_8,_5.0];
_1 = RET;
RET = [1644728818_u32,857204209_u32,1118922938_u32,3306753963_u32,4078692521_u32,1532857842_u32];
_8 = -_5.0;
_5.1 = !false;
Call(_5.2.2 = fn5(RET, _4, _3, _1, _5.0, _1, _1, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = [3805537347_u32,1330029887_u32,2874072172_u32,1383164722_u32,3798110360_u32,3917558998_u32];
_4 = [_5.0,_8,_8,_5.0];
RET = [1190490524_u32,2554518809_u32,2478344576_u32,4260387804_u32,3923880222_u32,3505017074_u32];
RET = [2750555420_u32,1559220079_u32,3586782034_u32,1186046207_u32,2361100724_u32,252472629_u32];
_12 = -_5.0;
RET = [1450491921_u32,2157915051_u32,3283642009_u32,3198386381_u32,1217704490_u32,2641996716_u32];
_3 = _2;
_13 = [_12,_5.0];
_9 = (-70_i8);
RET = [406502785_u32,2733416464_u32,2782478225_u32,2479764158_u32,484477814_u32,3136716747_u32];
_8 = _6 as isize;
_5.0 = _6 as isize;
_11 = _6;
_6 = _11;
_4 = _3;
_1 = [703007591_u32,1495582664_u32,2210393974_u32,1065656519_u32,937305323_u32,1710927112_u32];
_5.2.1 = 193306678_i32;
_5.2.1 = (-1957432261_i32) >> _5.2.0;
_11 = _6;
_6 = _11;
_11 = _6;
_5.2.0 = 136_u8 + 215_u8;
Goto(bb4)
}
bb4 = {
_5.1 = !false;
_16 = [137837457768363061374516437952792493381_i128,72056643997066093984240533864989639376_i128,(-83730581125438017977950486861922139426_i128),64416416602274256295654247572751748815_i128,(-151950570596827188202134340002586077930_i128),(-48970002547898336624397378696649963424_i128),(-166151068913545290919742200759460404321_i128),21070770304017683978734710745669837997_i128];
_5.2.1 = _9 as i32;
_1 = [1392200169_u32,4225764280_u32,2794755055_u32,2832701353_u32,49564235_u32,2279970895_u32];
_12 = _5.0 ^ _5.0;
_5.1 = true;
_5.1 = !false;
_16 = [(-36902274782280683948815480054126478599_i128),155055326251564297631189047175447083558_i128,41162799561259903273453935623089410352_i128,41474640844710868742984793718174792929_i128,154053433501869103931922535003096982405_i128,21421132894789373508093947014598532582_i128,38394500614220793819067398416378666615_i128,58031938032267249320922146884014670479_i128];
_5.0 = _12;
_5.2.1 = 2016858434991904714_i64 as i32;
_17 = !136617001512125158002966199186281491335_u128;
_6 = _11;
_5.2.0 = 2657086822_u32 as u8;
_3 = _2;
_11 = _6;
match _9 {
0 => bb3,
1 => bb2,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
340282366920938463463374607431768211386 => bb10,
_ => bb9
}
}
bb5 = {
RET = [3805537347_u32,1330029887_u32,2874072172_u32,1383164722_u32,3798110360_u32,3917558998_u32];
_4 = [_5.0,_8,_8,_5.0];
RET = [1190490524_u32,2554518809_u32,2478344576_u32,4260387804_u32,3923880222_u32,3505017074_u32];
RET = [2750555420_u32,1559220079_u32,3586782034_u32,1186046207_u32,2361100724_u32,252472629_u32];
_12 = -_5.0;
RET = [1450491921_u32,2157915051_u32,3283642009_u32,3198386381_u32,1217704490_u32,2641996716_u32];
_3 = _2;
_13 = [_12,_5.0];
_9 = (-70_i8);
RET = [406502785_u32,2733416464_u32,2782478225_u32,2479764158_u32,484477814_u32,3136716747_u32];
_8 = _6 as isize;
_5.0 = _6 as isize;
_11 = _6;
_6 = _11;
_4 = _3;
_1 = [703007591_u32,1495582664_u32,2210393974_u32,1065656519_u32,937305323_u32,1710927112_u32];
_5.2.1 = 193306678_i32;
_5.2.1 = (-1957432261_i32) >> _5.2.0;
_11 = _6;
_6 = _11;
_11 = _6;
_5.2.0 = 136_u8 + 215_u8;
Goto(bb4)
}
bb6 = {
_6 = '\u{e5752}';
_5.2.0 = (-13011884354035507047811729560341424933_i128) as u8;
_5.0 = _8 * _8;
RET = [2135219419_u32,708142049_u32,1150486090_u32,2121741506_u32,3731389478_u32,1693122105_u32];
_4 = [_5.0,_5.0,_8,_5.0];
_5.1 = false;
_8 = -_5.0;
RET = [1268150331_u32,920750667_u32,1105776141_u32,959686508_u32,224514647_u32,2459439253_u32];
RET = _1;
_5.2.0 = _5.1 as u8;
_5.1 = !false;
RET = [1692598386_u32,1959946278_u32,3705138436_u32,3565540993_u32,247693531_u32,3752878207_u32];
_1 = [225648700_u32,1928246837_u32,3912872874_u32,31911256_u32,2856854386_u32,3183927044_u32];
_5.0 = _8 - _8;
_9 = 32_i8 & 98_i8;
_4 = _3;
_5.2.0 = 203_u8;
_4 = [_8,_8,_5.0,_5.0];
_2 = [_5.0,_8,_5.0,_8];
_2 = [_5.0,_8,_8,_5.0];
_1 = RET;
RET = [1644728818_u32,857204209_u32,1118922938_u32,3306753963_u32,4078692521_u32,1532857842_u32];
_8 = -_5.0;
_5.1 = !false;
Call(_5.2.2 = fn5(RET, _4, _3, _1, _5.0, _1, _1, _3), ReturnTo(bb3), UnwindUnreachable())
}
bb7 = {
_4 = [9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_6 = '\u{80c94}';
_2 = [(-67_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_5.2.0 = 216_u8;
_5.0 = 43_isize * 104_isize;
RET = [3291993854_u32,3759659496_u32,3307603423_u32,3875210928_u32,1240154300_u32,2050320521_u32];
_5.0 = 9223372036854775807_isize;
_2 = [_5.0,_5.0,_5.0,_5.0];
_2 = [_5.0,_5.0,_5.0,_5.0];
_5.0 = (-9223372036854775808_isize) * (-46_isize);
_5.2.1 = (-1102720482_i32);
_5.2.0 = 120_u8 & 101_u8;
RET = [3652725659_u32,3645125912_u32,2676867127_u32,4291027147_u32,2461717924_u32,1044706333_u32];
_1 = [1870329990_u32,4228031261_u32,876816668_u32,1047412370_u32,1687205046_u32,1842118367_u32];
_5.2.1 = -(-1910248877_i32);
RET = _1;
_5.1 = false;
_8 = (-140365153608911925996780155812194378395_i128) as isize;
RET = [3225932617_u32,1703960849_u32,3841438984_u32,2098393620_u32,417602564_u32,204918006_u32];
RET = _1;
RET = _1;
_5.1 = _6 <= _6;
_5.1 = _5.0 <= _5.0;
RET = [1010805951_u32,571099775_u32,391709742_u32,4293745404_u32,2592941839_u32,2872082676_u32];
Goto(bb2)
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_14 = [1373977137_u32,1191181561_u32,808956781_u32,2026969049_u32,3756755915_u32,2508645760_u32];
_5.1 = true & false;
_17 = _5.1 as u128;
_4 = [_12,_5.0,_12,_5.0];
_15 = [15941200245181048599_u64,14335868526028988621_u64,15505677273698460879_u64];
Goto(bb11)
}
bb11 = {
_6 = _11;
_1 = RET;
_16 = [(-32841428755580603137527687717446553123_i128),(-4211943079611908567475661936839554544_i128),(-21783556651159793346812303808157896410_i128),(-151451260076903792655154830923848266621_i128),(-15817005097256386892592747013149265494_i128),44590964659942926202234397123249452708_i128,6024686465558934970559594401319693130_i128,(-117452353710436670919961956075765012949_i128)];
_8 = _12;
_19 = !53279_u16;
Goto(bb12)
}
bb12 = {
_8 = _17 as isize;
_6 = _11;
_16 = [(-6589821880172533915248715808756731343_i128),(-29914665680723576328119690983195087227_i128),137548577002214145499534097079364629069_i128,95616802359988420011944579843750301046_i128,(-158149895144502913382016685442742022612_i128),34229424333826627778952264992089188756_i128,40467908745378462634691080991608931050_i128,(-163690604607593040584326786160706546416_i128)];
RET = _1;
_5.2.1 = (-8963239611157238005_i64) as i32;
_1 = _14;
_20 = _6;
_6 = _11;
_18 = [3850250124274644827_u64,3581186048621330351_u64,8513991274418341389_u64];
_21 = Adt51 { fld0: (-28928_i16),fld1: _5.2.0 };
_18 = [4253535103752837087_u64,14388589154985274575_u64,4245941000459375264_u64];
_18 = [829508662224676656_u64,13350959505245777178_u64,6746866360759388170_u64];
_17 = 29516896524195412954819076613692263716_u128 ^ 65164514110077546682425396203406493434_u128;
_6 = _20;
_21 = Adt51 { fld0: (-28431_i16),fld1: _5.2.0 };
_5.2.1 = 8282391194619694498_i64 as i32;
_21.fld1 = _5.2.0;
_13 = [_5.0,_5.0];
match _21.fld0 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
340282366920938463463374607431768183025 => bb13,
_ => bb7
}
}
bb13 = {
_15 = [7567195654575511376_u64,10544663184651682471_u64,8597417681647856661_u64];
_5.2.2 = core::ptr::addr_of!(_21.fld0);
_22 = [1894128202_u32,3223960023_u32,788681149_u32,2410574588_u32,21666428_u32,2425451623_u32];
_18 = [13475057440574532888_u64,963262408879307158_u64,4119209343934457642_u64];
_8 = _5.0;
_16 = [(-57723233035344659857790829819488921675_i128),(-95361320793894828683855139655250294156_i128),(-35473923571267937935634066700887021878_i128),(-117028465593365542324444480352442482855_i128),77458339321159928681310115263707404186_i128,(-22567731225373362626068980490866447122_i128),(-117947942585278405566259243597488521364_i128),9842533785144604584969896256636980329_i128];
_5.2.0 = !_21.fld1;
_17 = _21.fld0 as u128;
_5.0 = _8;
_21.fld1 = _5.2.0;
_12 = _8 | _8;
_4 = [_8,_12,_8,_12];
_17 = _5.2.1 as u128;
_22 = _14;
_4 = _3;
_21.fld0 = (-27278_i16) >> _5.0;
_11 = _6;
_5.2.1 = 894510854_i32;
_4 = [_8,_12,_12,_12];
_20 = _11;
_26 = [_8,_12,_8,_5.0];
Goto(bb14)
}
bb14 = {
_12 = _21.fld1 as isize;
_21.fld1 = !_5.2.0;
_15 = _18;
_22 = [4020407439_u32,454670555_u32,2858137139_u32,3462043843_u32,2145241127_u32,713091648_u32];
_22 = [3936550299_u32,3380477549_u32,2365533863_u32,3268827992_u32,2912812776_u32,1697060792_u32];
_5.2.1 = _6 as i32;
_21.fld1 = _5.2.0 * _5.2.0;
_12 = _5.2.1 as isize;
_23 = -_9;
_20 = _11;
_15 = [9632699172780315398_u64,10121035906286656389_u64,1138382701455255919_u64];
_8 = -_12;
_5.2.1 = 1023483305_i32;
_9 = -_23;
_9 = _5.2.1 as i8;
_21.fld1 = 6_usize as u8;
Goto(bb15)
}
bb15 = {
Call(_28 = dump_var(4_usize, 11_usize, Move(_11), 18_usize, Move(_18), 4_usize, Move(_4), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_28 = dump_var(4_usize, 2_usize, Move(_2), 23_usize, Move(_23), 1_usize, Move(_1), 19_usize, Move(_19)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(4_usize, 12_usize, Move(_12), 15_usize, Move(_15), 29_usize, _29, 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [u32; 6],mut _2: [isize; 4],mut _3: [isize; 4],mut _4: [u32; 6],mut _5: isize,mut _6: [u32; 6],mut _7: [u32; 6],mut _8: [isize; 4]) -> *const i16 {
mir! {
type RET = *const i16;
let _9: u8;
let _10: Adt51;
let _11: isize;
let _12: (i64, i64);
let _13: [isize; 4];
let _14: isize;
let _15: (i64, i64);
let _16: i128;
let _17: [isize; 4];
let _18: Adt55;
let _19: [isize; 4];
let _20: bool;
let _21: Adt44;
let _22: usize;
let _23: u64;
let _24: u64;
let _25: i64;
let _26: i16;
let _27: i32;
let _28: u128;
let _29: [u32; 6];
let _30: Adt53;
let _31: [i128; 8];
let _32: usize;
let _33: &'static i64;
let _34: *mut u32;
let _35: *const i16;
let _36: bool;
let _37: [i32; 7];
let _38: [u16; 8];
let _39: ();
let _40: ();
{
_6 = [2058992418_u32,3476981377_u32,4281526064_u32,2038469099_u32,4010764095_u32,1691568642_u32];
_1 = _7;
_4 = [2601722620_u32,394357132_u32,2069289184_u32,2205140994_u32,3166763606_u32,1548244698_u32];
_3 = _2;
_2 = _3;
_3 = _2;
_3 = [_5,_5,_5,_5];
_2 = _8;
_2 = _3;
RET = core::ptr::addr_of!(_10.fld0);
_10 = Adt51 { fld0: (-171_i16),fld1: 193_u8 };
_2 = [_5,_5,_5,_5];
_11 = 3987355933_u32 as isize;
(*RET) = 31011_i16;
_10.fld0 = !(-14292_i16);
Goto(bb1)
}
bb1 = {
_3 = [_5,_5,_5,_5];
_10.fld0 = (-25279_i16) + 16583_i16;
(*RET) = 9508887054945618263_u64 as i16;
_11 = 43107_u16 as isize;
_10.fld1 = 34691_u16 as u8;
_5 = _11 - _11;
_12 = (4396909619357669503_i64, 4154561443450700370_i64);
RET = core::ptr::addr_of!(_10.fld0);
_3 = [_5,_11,_11,_5];
_9 = _10.fld1 - _10.fld1;
_8 = [_5,_11,_5,_5];
_5 = 2676544742_u32 as isize;
_14 = _5;
_15.1 = 4_usize as i64;
_8 = _3;
_10.fld0 = 31166_i16;
RET = core::ptr::addr_of!(_10.fld0);
_10.fld1 = !_9;
RET = core::ptr::addr_of!(_10.fld0);
_14 = !_11;
_15 = (_12.0, _12.1);
(*RET) = 3863_i16 >> _10.fld1;
_13 = _2;
match _15.1 {
0 => bb2,
4154561443450700370 => bb4,
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
_6 = _1;
_16 = (-161693603800047413457664737620990567349_i128);
_7 = [2453026911_u32,2503684346_u32,1247557463_u32,2611494984_u32,1440788093_u32,4062722219_u32];
_1 = [1075702086_u32,1033769717_u32,1636358314_u32,975978992_u32,3329023428_u32,1631163726_u32];
_15.0 = _12.1;
_4 = [2519764379_u32,1593126454_u32,158285644_u32,2903562875_u32,589163942_u32,3379647363_u32];
_11 = !_14;
_17 = _13;
_8 = [_11,_5,_14,_14];
_15 = _12;
_13 = [_5,_14,_11,_14];
Goto(bb5)
}
bb5 = {
_2 = _3;
_10.fld0 = (-22418_i16);
_8 = [_5,_5,_11,_14];
_13 = [_5,_14,_11,_5];
_12.0 = _12.1 << _10.fld1;
_5 = !_11;
_12.1 = 103_i8 as i64;
_4 = [2695479829_u32,3776334969_u32,4161974151_u32,2985632209_u32,2250489071_u32,1149880465_u32];
_15.0 = 334479798_u32 as i64;
_10.fld0 = -27175_i16;
_5 = _11;
(*RET) = (-19043_i16) & 21681_i16;
_12 = (_15.1, _15.1);
match _16 {
0 => bb1,
1 => bb6,
2 => bb7,
178588763120891050005709869810777644107 => bb9,
_ => bb8
}
}
bb6 = {
_6 = _1;
_16 = (-161693603800047413457664737620990567349_i128);
_7 = [2453026911_u32,2503684346_u32,1247557463_u32,2611494984_u32,1440788093_u32,4062722219_u32];
_1 = [1075702086_u32,1033769717_u32,1636358314_u32,975978992_u32,3329023428_u32,1631163726_u32];
_15.0 = _12.1;
_4 = [2519764379_u32,1593126454_u32,158285644_u32,2903562875_u32,589163942_u32,3379647363_u32];
_11 = !_14;
_17 = _13;
_8 = [_11,_5,_14,_14];
_15 = _12;
_13 = [_5,_14,_11,_14];
Goto(bb5)
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
_1 = _4;
_20 = _5 > _14;
_6 = _4;
_4 = [1147264303_u32,1971521528_u32,2988346735_u32,433551034_u32,973973216_u32,1919908566_u32];
_19 = [_5,_11,_14,_11];
Goto(bb10)
}
bb10 = {
(*RET) = -(-10225_i16);
(*RET) = _16 as i16;
_20 = _12.0 >= _15.1;
_15.1 = -_12.0;
_11 = _14 + _5;
_15.1 = _12.0 << _11;
_12.1 = 17522794034190750381_u64 as i64;
_23 = 6200962878727847890_u64;
RET = core::ptr::addr_of!((*RET));
_22 = 2422225094625318435_usize;
_19 = _17;
_13 = [_11,_11,_11,_14];
_15 = (_12.0, _12.0);
_20 = true;
_12.0 = _23 as i64;
_14 = _11 + _5;
_23 = 57077059033182675112906504850142428388_u128 as u64;
Call((*RET) = fn6(_3, _17, _17, _2), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_3 = _2;
_10.fld1 = !_9;
(*RET) = _10.fld1 as i16;
(*RET) = (-1490755726_i32) as i16;
_11 = _5;
_5 = _14;
_8 = [_14,_5,_5,_5];
_15 = _12;
_24 = _23;
_8 = [_14,_14,_14,_14];
_9 = '\u{f1d7f}' as u8;
_4 = [3970773049_u32,1419458325_u32,88182856_u32,3299367065_u32,2026071305_u32,353311279_u32];
_8 = [_5,_14,_14,_14];
_10 = Adt51 { fld0: (-22488_i16),fld1: _9 };
(*RET) = (-14977_i16);
(*RET) = -(-13665_i16);
_10.fld1 = _9;
_6 = [3970042376_u32,2507163418_u32,4133262453_u32,3270266274_u32,562797423_u32,2898346379_u32];
_20 = true;
_15.0 = _12.0 >> _10.fld0;
_29 = [2876137964_u32,1896086821_u32,1836106373_u32,1690920581_u32,3678300937_u32,1843937483_u32];
_10.fld1 = _9 - _9;
_10.fld1 = 184266385968249075508082010773917476307_u128 as u8;
match _22 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb8,
5 => bb10,
6 => bb9,
2422225094625318435 => bb13,
_ => bb12
}
}
bb12 = {
Return()
}
bb13 = {
_31 = [_16,_16,_16,_16,_16,_16,_16,_16];
_10.fld0 = -(-22860_i16);
(*RET) = 3505_i16;
_29 = [1185405233_u32,2876195000_u32,176138772_u32,710868626_u32,175777828_u32,1843634842_u32];
_9 = !_10.fld1;
_15.1 = _20 as i64;
_3 = _13;
_24 = _23;
_15 = (_12.1, _12.0);
_20 = true ^ false;
RET = core::ptr::addr_of!(_10.fld0);
_13 = [_5,_14,_5,_5];
_24 = '\u{d1275}' as u64;
_32 = 191609561889558520592062199863109156444_u128 as usize;
(*RET) = (-5188_i16) >> _16;
_15 = _12;
_20 = _5 > _14;
_33 = &_12.1;
_28 = (-110812257_i32) as u128;
_27 = 381560459_i32 << _24;
_9 = !_10.fld1;
RET = core::ptr::addr_of!(_10.fld0);
_5 = -_14;
Call(_20 = fn9(_15.0, _5, _17, _24, RET), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_15.1 = _22 as i64;
(*RET) = _20 as i16;
_16 = (-52578912119930135323336709379501450985_i128) << _24;
_10.fld1 = _28 as u8;
_15.1 = -(*_33);
_26 = _10.fld0;
_27 = 1097518129_i32;
RET = core::ptr::addr_of!((*RET));
_9 = _10.fld1;
_10.fld1 = !_9;
(*RET) = _26;
Goto(bb15)
}
bb15 = {
Call(_39 = dump_var(5_usize, 19_usize, Move(_19), 23_usize, Move(_23), 22_usize, Move(_22), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_39 = dump_var(5_usize, 32_usize, Move(_32), 24_usize, Move(_24), 5_usize, Move(_5), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_39 = dump_var(5_usize, 8_usize, Move(_8), 17_usize, Move(_17), 29_usize, Move(_29), 7_usize, Move(_7)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_39 = dump_var(5_usize, 27_usize, Move(_27), 40_usize, _40, 40_usize, _40, 40_usize, _40), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [isize; 4],mut _2: [isize; 4],mut _3: [isize; 4],mut _4: [isize; 4]) -> i16 {
mir! {
type RET = i16;
let _5: *const u32;
let _6: f64;
let _7: (i64, i64);
let _8: [u32; 6];
let _9: u32;
let _10: Adt51;
let _11: char;
let _12: f64;
let _13: char;
let _14: *const i16;
let _15: isize;
let _16: [u32; 6];
let _17: u16;
let _18: bool;
let _19: i16;
let _20: [isize; 2];
let _21: u16;
let _22: Adt51;
let _23: [isize; 2];
let _24: f64;
let _25: char;
let _26: f64;
let _27: i16;
let _28: i16;
let _29: ();
let _30: ();
{
RET = 3930258429_u32 as i16;
Goto(bb1)
}
bb1 = {
_4 = _3;
_2 = _3;
Goto(bb2)
}
bb2 = {
_2 = _3;
RET = (-4837_i16);
_4 = _3;
_4 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_2 = [(-9223372036854775808_isize),(-49_isize),(-9223372036854775808_isize),(-3_isize)];
_1 = _4;
_7.0 = 6380_u16 as i64;
_7.1 = _7.0 + _7.0;
_6 = 1971698809_u32 as f64;
RET = 1228336826_i32 as i16;
_2 = _3;
_7.0 = -_7.1;
_9 = !3639994495_u32;
Call(_7.1 = core::intrinsics::transmute(_7.0), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = 7108_i16 << _7.1;
_4 = [(-13_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
RET = 6555405337512833071_usize as i16;
_7 = (3352733618489522008_i64, 6407132446585224210_i64);
_3 = [9223372036854775807_isize,(-9223372036854775808_isize),2_isize,(-9223372036854775808_isize)];
_8 = [_9,_9,_9,_9,_9,_9];
_7.0 = !_7.1;
RET = 19850_i16 & (-11739_i16);
_7.1 = _7.0 & _7.0;
Call(_7.0 = core::intrinsics::transmute(_7.1), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
_5 = core::ptr::addr_of!(_9);
_7 = (8967471299245217153_i64, 3520895527494387036_i64);
(*_5) = 2585986386_u32;
_8 = [_9,(*_5),(*_5),(*_5),(*_5),(*_5)];
RET = (-29973_i16) ^ 380_i16;
RET = !18513_i16;
_6 = (-1823362482_i32) as f64;
_2 = [(-123_isize),91_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_3 = [(-36_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_8 = [(*_5),_9,(*_5),(*_5),(*_5),(*_5)];
(*_5) = 3277866048419929532_u64 as u32;
_7.0 = 30_u8 as i64;
_8 = [(*_5),_9,_9,(*_5),(*_5),(*_5)];
(*_5) = (-2067442570_i32) as u32;
_2 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_7.1 = _7.0;
_10.fld0 = RET | RET;
_5 = core::ptr::addr_of!((*_5));
_1 = [9223372036854775807_isize,(-99_isize),(-9223372036854775808_isize),(-17_isize)];
_10 = Adt51 { fld0: RET,fld1: 221_u8 };
_11 = '\u{30030}';
_7.1 = _7.0;
_12 = _10.fld1 as f64;
RET = -_10.fld0;
RET = _7.0 as i16;
Goto(bb5)
}
bb5 = {
_4 = _1;
_2 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_4 = [(-9223372036854775808_isize),(-64_isize),9223372036854775807_isize,(-87_isize)];
match _10.fld1 {
0 => bb3,
1 => bb4,
2 => bb6,
221 => bb8,
_ => bb7
}
}
bb6 = {
_4 = _3;
_2 = _3;
Goto(bb2)
}
bb7 = {
RET = 7108_i16 << _7.1;
_4 = [(-13_isize),(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize];
RET = 6555405337512833071_usize as i16;
_7 = (3352733618489522008_i64, 6407132446585224210_i64);
_3 = [9223372036854775807_isize,(-9223372036854775808_isize),2_isize,(-9223372036854775808_isize)];
_8 = [_9,_9,_9,_9,_9,_9];
_7.0 = !_7.1;
RET = 19850_i16 & (-11739_i16);
_7.1 = _7.0 & _7.0;
Call(_7.0 = core::intrinsics::transmute(_7.1), ReturnTo(bb4), UnwindUnreachable())
}
bb8 = {
_10.fld0 = RET;
_4 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-33_isize),66_isize];
_13 = _11;
_15 = 98_isize;
_7.0 = _7.1;
_7 = (7650430219007063161_i64, 5425640105050776464_i64);
_16 = [(*_5),(*_5),(*_5),(*_5),_9,_9];
_8 = [_9,(*_5),(*_5),(*_5),(*_5),(*_5)];
_4 = _2;
_4 = [_15,_15,_15,_15];
_12 = _6;
_9 = 1905761196_u32 & 3289575327_u32;
_8 = _16;
_10 = Adt51 { fld0: RET,fld1: 26_u8 };
_6 = _12 + _12;
_10.fld0 = 140247054323064567492396246180373432215_u128 as i16;
_7.0 = !_7.1;
RET = !_10.fld0;
_8 = _16;
_10.fld1 = _11 as u8;
Call(_2 = fn7(_1, (*_5)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_10 = Adt51 { fld0: RET,fld1: 62_u8 };
_10.fld1 = 110_u8 - 26_u8;
_4 = [_15,_15,_15,_15];
RET = _10.fld0;
_7 = ((-2259184619118857207_i64), (-950229531087632655_i64));
_3 = [_15,_15,_15,_15];
_15 = _12 as isize;
_10.fld1 = _11 as u8;
_18 = _9 < (*_5);
_14 = core::ptr::addr_of!(_10.fld0);
RET = (*_14);
_13 = _11;
RET = (*_14);
(*_5) = 849629021_u32;
_9 = _13 as u32;
(*_5) = 1542768916_u32 - 3611932942_u32;
_11 = _13;
(*_5) = !1681577958_u32;
_17 = 20611_u16;
_18 = true;
(*_5) = !2987359171_u32;
(*_5) = !4065511647_u32;
_7 = ((-2660096525707057691_i64), (-8655085439508155931_i64));
RET = _10.fld0 - _10.fld0;
_10 = Adt51 { fld0: RET,fld1: 136_u8 };
_10.fld0 = RET;
_7.0 = _9 as i64;
_1 = _2;
RET = (*_14) * (*_14);
(*_5) = !169734648_u32;
_14 = core::ptr::addr_of!(RET);
Call((*_5) = core::intrinsics::transmute(_13), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_15 = (-83_i8) as isize;
_15 = 9223372036854775807_isize ^ (-57_isize);
_10.fld1 = _13 as u8;
_15 = 9223372036854775807_isize | (-9223372036854775808_isize);
(*_14) = _10.fld0;
(*_14) = 4658813693255270219623843040839664390_i128 as i16;
_10 = Adt51 { fld0: (*_14),fld1: 175_u8 };
_10.fld1 = 79_u8;
_4 = [_15,_15,_15,_15];
_10 = Adt51 { fld0: (*_14),fld1: 188_u8 };
_15 = 7_isize;
_7.1 = _7.0;
_10.fld0 = -RET;
_11 = _13;
(*_5) = _17 as u32;
RET = _10.fld0;
_19 = !_10.fld0;
_15 = !9223372036854775807_isize;
_15 = (-9223372036854775808_isize);
_7.0 = _7.1;
Goto(bb11)
}
bb11 = {
_7 = (3623447078326031404_i64, 2725613309202315785_i64);
_17 = 59767728022135408963177233588840156883_u128 as u16;
(*_5) = 1785555805_u32;
_13 = _11;
_5 = core::ptr::addr_of!((*_5));
_11 = _13;
Goto(bb12)
}
bb12 = {
_14 = core::ptr::addr_of!(RET);
_17 = 29595_u16 + 29700_u16;
_7.1 = _10.fld1 as i64;
_10.fld1 = 78_u8 * 232_u8;
_10 = Adt51 { fld0: (*_14),fld1: 172_u8 };
(*_14) = 16971250079902960228_usize as i16;
_12 = 16359798322449946272_u64 as f64;
_7 = ((-6950679303867828854_i64), (-2885633972393877817_i64));
(*_5) = 2983351331_u32;
_5 = core::ptr::addr_of!(_9);
(*_14) = _17 as i16;
_14 = core::ptr::addr_of!(RET);
match _7.1 {
340282366920938463460488973459374333639 => bb13,
_ => bb7
}
}
bb13 = {
_22.fld0 = _18 as i16;
_19 = !(*_14);
(*_5) = 3010779906_u32 ^ 960022662_u32;
_6 = -_12;
_15 = (-9223372036854775808_isize) * (-9223372036854775808_isize);
_7.0 = 88_i8 as i64;
_7 = ((-2186887999968572986_i64), 2398472781882420553_i64);
(*_14) = !_19;
(*_5) = 3747696769_u32;
_4 = [_15,_15,_15,_15];
_22.fld1 = _10.fld1;
_8 = _16;
_11 = _13;
_3 = [_15,_15,_15,_15];
_1 = [_15,_15,_15,_15];
_10.fld1 = _22.fld1;
_20 = [_15,_15];
_10.fld0 = RET << _22.fld1;
_19 = !_10.fld0;
_1 = [_15,_15,_15,_15];
_22 = Adt51 { fld0: (*_14),fld1: _10.fld1 };
_23 = _20;
_18 = !false;
match (*_5) {
0 => bb1,
1 => bb4,
2 => bb9,
3 => bb14,
4 => bb15,
5 => bb16,
3747696769 => bb18,
_ => bb17
}
}
bb14 = {
_4 = _3;
_2 = _3;
Goto(bb2)
}
bb15 = {
_4 = _1;
_2 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_4 = [(-9223372036854775808_isize),(-64_isize),9223372036854775807_isize,(-87_isize)];
match _10.fld1 {
0 => bb3,
1 => bb4,
2 => bb6,
221 => bb8,
_ => bb7
}
}
bb16 = {
_5 = core::ptr::addr_of!(_9);
_7 = (8967471299245217153_i64, 3520895527494387036_i64);
(*_5) = 2585986386_u32;
_8 = [_9,(*_5),(*_5),(*_5),(*_5),(*_5)];
RET = (-29973_i16) ^ 380_i16;
RET = !18513_i16;
_6 = (-1823362482_i32) as f64;
_2 = [(-123_isize),91_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_3 = [(-36_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_8 = [(*_5),_9,(*_5),(*_5),(*_5),(*_5)];
(*_5) = 3277866048419929532_u64 as u32;
_7.0 = 30_u8 as i64;
_8 = [(*_5),_9,_9,(*_5),(*_5),(*_5)];
(*_5) = (-2067442570_i32) as u32;
_2 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_7.1 = _7.0;
_10.fld0 = RET | RET;
_5 = core::ptr::addr_of!((*_5));
_1 = [9223372036854775807_isize,(-99_isize),(-9223372036854775808_isize),(-17_isize)];
_10 = Adt51 { fld0: RET,fld1: 221_u8 };
_11 = '\u{30030}';
_7.1 = _7.0;
_12 = _10.fld1 as f64;
RET = -_10.fld0;
RET = _7.0 as i16;
Goto(bb5)
}
bb17 = {
_10.fld0 = RET;
_4 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-33_isize),66_isize];
_13 = _11;
_15 = 98_isize;
_7.0 = _7.1;
_7 = (7650430219007063161_i64, 5425640105050776464_i64);
_16 = [(*_5),(*_5),(*_5),(*_5),_9,_9];
_8 = [_9,(*_5),(*_5),(*_5),(*_5),(*_5)];
_4 = _2;
_4 = [_15,_15,_15,_15];
_12 = _6;
_9 = 1905761196_u32 & 3289575327_u32;
_8 = _16;
_10 = Adt51 { fld0: RET,fld1: 26_u8 };
_6 = _12 + _12;
_10.fld0 = 140247054323064567492396246180373432215_u128 as i16;
_7.0 = !_7.1;
RET = !_10.fld0;
_8 = _16;
_10.fld1 = _11 as u8;
Call(_2 = fn7(_1, (*_5)), ReturnTo(bb9), UnwindUnreachable())
}
bb18 = {
_3 = _2;
_10.fld0 = _18 as i16;
(*_5) = 3680477304_u32;
(*_5) = 10410509_u32;
_19 = _10.fld0;
(*_14) = _22.fld0;
_4 = [_15,_15,_15,_15];
_12 = _6 + _6;
_7.1 = _10.fld1 as i64;
_2 = [_15,_15,_15,_15];
Goto(bb19)
}
bb19 = {
Call(_29 = dump_var(6_usize, 19_usize, Move(_19), 11_usize, Move(_11), 17_usize, Move(_17), 18_usize, Move(_18)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_29 = dump_var(6_usize, 15_usize, Move(_15), 1_usize, Move(_1), 4_usize, Move(_4), 7_usize, Move(_7)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: [isize; 4],mut _2: u32) -> [isize; 4] {
mir! {
type RET = [isize; 4];
let _3: [isize; 4];
let _4: u16;
let _5: usize;
let _6: f32;
let _7: f64;
let _8: *const (i128, (u8, i32, *const i16));
let _9: [isize; 4];
let _10: i128;
let _11: Adt51;
let _12: [i128; 8];
let _13: i16;
let _14: isize;
let _15: Adt51;
let _16: Adt59;
let _17: ();
let _18: ();
{
RET = _1;
_2 = 2476866393_u32;
_1 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
RET = _1;
_2 = 49879249897021560172976076764270006636_i128 as u32;
_2 = !860449232_u32;
_1 = [99_isize,47_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_2 = 1846419290_u32 | 2648899674_u32;
RET = _1;
RET = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_1 = [9223372036854775807_isize,9223372036854775807_isize,(-10_isize),(-9223372036854775808_isize)];
_4 = !26516_u16;
_1 = RET;
_4 = 10337_u16;
_2 = (-404011277_i32) as u32;
_5 = !5073532951599866152_usize;
_3 = [(-9223372036854775808_isize),9223372036854775807_isize,(-52_isize),9223372036854775807_isize];
_3 = [(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize)];
_6 = (-8022356046004280798_i64) as f32;
RET = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
_2 = 89412448_u32;
RET = _1;
_7 = _6 as f64;
_1 = RET;
match _4 {
10337 => bb2,
_ => bb1
}
}
bb1 = {
Return()
}
bb2 = {
_5 = !11364443138815205387_usize;
_3 = [9223372036854775807_isize,9223372036854775807_isize,(-104_isize),(-9223372036854775808_isize)];
_6 = (-23_i8) as f32;
match _2 {
0 => bb1,
1 => bb3,
89412448 => bb5,
_ => bb4
}
}
bb3 = {
Return()
}
bb4 = {
Return()
}
bb5 = {
_4 = !44273_u16;
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = _3;
_2 = !3128707268_u32;
_7 = 73457390780368836198696235748714381478_i128 as f64;
_9 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-81_isize)];
_9 = [113_isize,26_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_6 = _5 as f32;
_4 = '\u{d5c0a}' as u16;
_10 = -96122620702115428952466835933226874789_i128;
_12 = [_10,_10,_10,_10,_10,_10,_10,_10];
_11.fld0 = 10836_i16 | (-27693_i16);
_11.fld0 = '\u{3b35a}' as i16;
Goto(bb6)
}
bb6 = {
_11 = Adt51 { fld0: 9119_i16,fld1: 140_u8 };
_4 = 15531_u16 | 62185_u16;
_5 = _10 as usize;
_11 = Adt51 { fld0: (-1888_i16),fld1: 197_u8 };
_2 = 4059451171_u32;
Goto(bb7)
}
bb7 = {
_3 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),123_isize];
_10 = -108867525003437146712280130248475346180_i128;
Call(_7 = fn8(_11.fld0, Move(_11), _1), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_11.fld1 = !218_u8;
_7 = _2 as f64;
_12 = [_10,_10,_10,_10,_10,_10,_10,_10];
_11.fld1 = 249_u8 | 225_u8;
_7 = 9223372036854775807_isize as f64;
_4 = !62869_u16;
_11.fld0 = !(-6231_i16);
_11.fld0 = (-9223372036854775808_isize) as i16;
_10 = '\u{cae9b}' as i128;
_11 = Adt51 { fld0: (-28415_i16),fld1: 167_u8 };
RET = [9223372036854775807_isize,(-19_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_3 = [(-9_isize),(-9223372036854775808_isize),16_isize,(-9223372036854775808_isize)];
RET = _9;
_6 = _11.fld0 as f32;
_10 = (-146851029724071575763524155670741623886_i128);
_9 = [(-9223372036854775808_isize),9223372036854775807_isize,(-82_isize),(-9223372036854775808_isize)];
_11.fld1 = _5 as u8;
_15.fld1 = _11.fld1;
_11.fld1 = _15.fld1 >> _11.fld0;
_12 = [_10,_10,_10,_10,_10,_10,_10,_10];
_14 = (-12_isize) >> _2;
_7 = 1722602513_i32 as f64;
_5 = 1_usize ^ 7653598506775813302_usize;
_10 = (-11823286650063109760032376613322903123_i128) << _14;
match _11.fld0 {
0 => bb7,
1 => bb9,
2 => bb10,
3 => bb11,
4 => bb12,
340282366920938463463374607431768183041 => bb14,
_ => bb13
}
}
bb9 = {
_3 = [9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),123_isize];
_10 = -108867525003437146712280130248475346180_i128;
Call(_7 = fn8(_11.fld0, Move(_11), _1), ReturnTo(bb8), UnwindUnreachable())
}
bb10 = {
_5 = !11364443138815205387_usize;
_3 = [9223372036854775807_isize,9223372036854775807_isize,(-104_isize),(-9223372036854775808_isize)];
_6 = (-23_i8) as f32;
match _2 {
0 => bb1,
1 => bb3,
89412448 => bb5,
_ => bb4
}
}
bb11 = {
_4 = !44273_u16;
_1 = [(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = _3;
_2 = !3128707268_u32;
_7 = 73457390780368836198696235748714381478_i128 as f64;
_9 = [(-9223372036854775808_isize),9223372036854775807_isize,9223372036854775807_isize,(-81_isize)];
_9 = [113_isize,26_isize,9223372036854775807_isize,(-9223372036854775808_isize)];
_6 = _5 as f32;
_4 = '\u{d5c0a}' as u16;
_10 = -96122620702115428952466835933226874789_i128;
_12 = [_10,_10,_10,_10,_10,_10,_10,_10];
_11.fld0 = 10836_i16 | (-27693_i16);
_11.fld0 = '\u{3b35a}' as i16;
Goto(bb6)
}
bb12 = {
Return()
}
bb13 = {
Return()
}
bb14 = {
_15 = Adt51 { fld0: _11.fld0,fld1: _11.fld1 };
_13 = !_15.fld0;
_12 = [_10,_10,_10,_10,_10,_10,_10,_10];
_15.fld1 = _6 as u8;
_15.fld0 = _11.fld0;
Goto(bb15)
}
bb15 = {
Call(_17 = dump_var(7_usize, 4_usize, Move(_4), 5_usize, Move(_5), 9_usize, Move(_9), 3_usize, Move(_3)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_17 = dump_var(7_usize, 13_usize, Move(_13), 18_usize, _18, 18_usize, _18, 18_usize, _18), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn8(mut _1: i16,mut _2: Adt51,mut _3: [isize; 4]) -> f64 {
mir! {
type RET = f64;
let _4: f64;
let _5: Adt57;
let _6: i128;
let _7: u32;
let _8: (i64, i64);
let _9: *mut *const (i128, (u8, i32, *const i16));
let _10: *mut (i64, i64);
let _11: [u64; 3];
let _12: isize;
let _13: [isize; 2];
let _14: f64;
let _15: [i128; 8];
let _16: ();
let _17: ();
{
_2.fld1 = 241_u8 | 75_u8;
RET = 1397934743_u32 as f64;
_1 = !_2.fld0;
RET = 73118421880433425180547057576465941279_i128 as f64;
_1 = _2.fld0;
_2 = Adt51 { fld0: _1,fld1: 143_u8 };
_4 = RET + RET;
RET = 83_i8 as f64;
_2 = Adt51 { fld0: _1,fld1: 42_u8 };
RET = 3662988335_u32 as f64;
RET = _4 - _4;
RET = -_4;
_2.fld0 = _1;
_2 = Adt51 { fld0: _1,fld1: 95_u8 };
_3 = [9223372036854775807_isize,9223372036854775807_isize,(-9223372036854775808_isize),(-9223372036854775808_isize)];
RET = _4 * _4;
match _2.fld1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
95 => bb8,
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
_3 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_1 = _2.fld0;
_2 = Adt51 { fld0: _1,fld1: 183_u8 };
_6 = -(-51360944801330815704522777876771973378_i128);
_2.fld0 = 1081665802_u32 as i16;
RET = _4 * _4;
RET = -_4;
_4 = -RET;
_3 = [9223372036854775807_isize,(-100_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_8.1 = 7579353273089343361_i64;
_8 = (668464283357915976_i64, (-1465617823728536346_i64));
_2.fld1 = 90_u8 - 62_u8;
_6 = 48_isize as i128;
RET = _4;
_1 = _2.fld0 - _2.fld0;
_1 = _2.fld0 << _8.1;
_3 = [67_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),3_isize];
_5 = Adt57::Variant3 { fld0: 2_usize };
_2.fld0 = 601216068_i32 as i16;
RET = -_4;
_7 = !998156249_u32;
_2.fld0 = _1 * _1;
_3 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_2.fld1 = 76654247901636775867777296995502047026_u128 as u8;
_10 = core::ptr::addr_of_mut!(_8);
place!(Field::<usize>(Variant(_5, 3), 0)) = 6_usize;
_2.fld1 = !118_u8;
_4 = -RET;
Call((*_10).0 = core::intrinsics::transmute(_8.1), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
(*_10).0 = (*_10).1;
_8 = ((-4864410986581282670_i64), (-1213603764121542992_i64));
_1 = -_2.fld0;
_8 = ((-5642499774854612973_i64), (-7786507906239170821_i64));
match (*_10).1 {
0 => bb10,
1 => bb11,
2 => bb12,
3 => bb13,
4 => bb14,
5 => bb15,
6 => bb16,
340282366920938463455588099525529040635 => bb18,
_ => bb17
}
}
bb10 = {
_3 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
_1 = _2.fld0;
_2 = Adt51 { fld0: _1,fld1: 183_u8 };
_6 = -(-51360944801330815704522777876771973378_i128);
_2.fld0 = 1081665802_u32 as i16;
RET = _4 * _4;
RET = -_4;
_4 = -RET;
_3 = [9223372036854775807_isize,(-100_isize),(-9223372036854775808_isize),(-9223372036854775808_isize)];
_8.1 = 7579353273089343361_i64;
_8 = (668464283357915976_i64, (-1465617823728536346_i64));
_2.fld1 = 90_u8 - 62_u8;
_6 = 48_isize as i128;
RET = _4;
_1 = _2.fld0 - _2.fld0;
_1 = _2.fld0 << _8.1;
_3 = [67_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),3_isize];
_5 = Adt57::Variant3 { fld0: 2_usize };
_2.fld0 = 601216068_i32 as i16;
RET = -_4;
_7 = !998156249_u32;
_2.fld0 = _1 * _1;
_3 = [9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize,9223372036854775807_isize];
_2.fld1 = 76654247901636775867777296995502047026_u128 as u8;
_10 = core::ptr::addr_of_mut!(_8);
place!(Field::<usize>(Variant(_5, 3), 0)) = 6_usize;
_2.fld1 = !118_u8;
_4 = -RET;
Call((*_10).0 = core::intrinsics::transmute(_8.1), ReturnTo(bb9), UnwindUnreachable())
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
Return()
}
bb15 = {
Return()
}
bb16 = {
Return()
}
bb17 = {
Return()
}
bb18 = {
(*_10).1 = (*_10).0;
_8 = (9052196667802903153_i64, 8255294868948652115_i64);
_8.0 = _8.1 & (*_10).1;
(*_10).0 = (*_10).1;
_3 = [(-9223372036854775808_isize),9223372036854775807_isize,(-9223372036854775808_isize),9223372036854775807_isize];
place!(Field::<usize>(Variant(_5, 3), 0)) = !12278434659385600636_usize;
_10 = core::ptr::addr_of_mut!(_8);
_10 = core::ptr::addr_of_mut!(_8);
Goto(bb19)
}
bb19 = {
Call(_16 = dump_var(8_usize, 3_usize, Move(_3), 6_usize, Move(_6), 17_usize, _17, 17_usize, _17), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: i64,mut _2: isize,mut _3: [isize; 4],mut _4: u64,mut _5: *const i16) -> bool {
mir! {
type RET = bool;
let _6: f64;
let _7: [isize; 4];
let _8: (i64, i64);
let _9: isize;
let _10: i32;
let _11: [isize; 2];
let _12: [u16; 8];
let _13: (i8, *const (isize, bool, (u8, i32, *const i16)));
let _14: f32;
let _15: [isize; 4];
let _16: (i128, (u8, i32, *const i16));
let _17: ([i32; 7], u32, *const isize);
let _18: u8;
let _19: usize;
let _20: f64;
let _21: f32;
let _22: char;
let _23: Adt52;
let _24: ();
let _25: ();
{
RET = false;
_4 = 3483409177395414706_u64;
_5 = core::ptr::addr_of!((*_5));
_4 = 7116590456610893865_u64;
_2 = !(-9223372036854775808_isize);
_3 = [_2,_2,_2,_2];
_1 = !(-289586608367762363_i64);
_3 = [_2,_2,_2,_2];
(*_5) = 6360_i16;
_5 = core::ptr::addr_of!((*_5));
_7 = [_2,_2,_2,_2];
_3 = _7;
_7 = [_2,_2,_2,_2];
_8.0 = _1 << _4;
_4 = 230_u8 as u64;
_2 = (-113_isize) * (-81_isize);
_6 = 4_usize as f64;
_8.1 = !_8.0;
_2 = (-9223372036854775808_isize);
_6 = (-96651921_i32) as f64;
_5 = core::ptr::addr_of!((*_5));
_11 = [_2,_2];
_3 = [_2,_2,_2,_2];
_9 = _2 + _2;
_5 = core::ptr::addr_of!((*_5));
match (*_5) {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
5 => bb6,
6360 => bb8,
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
_5 = core::ptr::addr_of!((*_5));
_8 = (_1, _1);
_8 = (_1, _1);
_9 = _2;
_4 = 8429838704913326322_u64;
RET = true;
_10 = 1817414556_i32 * 1392832473_i32;
_5 = core::ptr::addr_of!((*_5));
_6 = (*_5) as f64;
(*_5) = RET as i16;
_11 = [_9,_9];
_10 = 1542836867_i32 - (-488775035_i32);
_8.0 = !_1;
_3 = [_9,_2,_9,_9];
_7 = [_9,_9,_9,_9];
_15 = [_9,_9,_2,_2];
_16.1.2 = core::ptr::addr_of!((*_5));
_6 = _10 as f64;
_16.1.1 = _10;
match _2 {
0 => bb4,
1 => bb9,
340282366920938463454151235394913435648 => bb11,
_ => bb10
}
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_4 = 15849350333856830201_u64 | 16715805147311684670_u64;
_16.1 = (68_u8, _10, _5);
_3 = [_2,_9,_2,_2];
_3 = [_9,_2,_2,_2];
_17.1 = 879923250_u32;
_16.1 = (131_u8, _10, _5);
_8 = (_1, _1);
_12 = [32930_u16,43382_u16,36172_u16,55074_u16,53893_u16,48369_u16,7007_u16,52003_u16];
_14 = (*_5) as f32;
_16.0 = (-163521153139234796060501242028670939052_i128) << _1;
_1 = _8.0;
_11 = [_9,_2];
_2 = _9 & _9;
_8 = (_1, _1);
_13.0 = !(-117_i8);
_16.0 = 90913277508204684175347232375127477454_i128 - (-91234392096262626988994427675934868067_i128);
_2 = -_9;
_16.1 = (116_u8, _10, _5);
_7 = [_2,_2,_2,_2];
_16.0 = (-41481144376504050812121869961366028817_i128);
Call(_18 = fn10((*_5), (*_5), _16.1.0, _12, _15, _17.1, _16.1, _16, _16, _16.1, _16.1, RET, _16.1.0), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_1 = _8.1 | _8.0;
(*_5) = 16958_i16 << _2;
(*_5) = (-10433_i16);
(*_5) = (-10925_i16) ^ 13118_i16;
_17.0 = [_16.1.1,_10,_10,_10,_10,_10,_16.1.1];
_4 = !4544327620968492863_u64;
_17.2 = core::ptr::addr_of!(_9);
_8.1 = _1 - _8.0;
_17.1 = _16.1.0 as u32;
RET = _17.1 > _17.1;
match _16.1.0 {
0 => bb8,
1 => bb11,
2 => bb13,
3 => bb14,
4 => bb15,
116 => bb17,
_ => bb16
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
RET = _17.1 < _17.1;
_8.0 = _8.1;
_4 = !3154975399157661295_u64;
_12 = [64772_u16,53525_u16,12169_u16,1219_u16,12165_u16,49856_u16,33541_u16,44319_u16];
RET = false;
_14 = _16.1.0 as f32;
_1 = _8.0;
_13.0 = (-83_i8) * (-104_i8);
_4 = 8179783399887083027_u64;
_16.1.0 = _18;
_14 = _8.1 as f32;
_21 = _14 - _14;
_17.2 = core::ptr::addr_of!(_2);
_6 = _14 as f64;
_21 = 3_usize as f32;
_13.0 = 14_i8 - 8_i8;
_16.1.1 = -_10;
_17.2 = core::ptr::addr_of!(_2);
_17.1 = !415045506_u32;
_17.1 = _6 as u32;
Goto(bb18)
}
bb18 = {
Call(_24 = dump_var(9_usize, 8_usize, Move(_8), 3_usize, Move(_3), 18_usize, Move(_18), 9_usize, Move(_9)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_24 = dump_var(9_usize, 12_usize, Move(_12), 11_usize, Move(_11), 25_usize, _25, 25_usize, _25), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: i16,mut _2: i16,mut _3: u8,mut _4: [u16; 8],mut _5: [isize; 4],mut _6: u32,mut _7: (u8, i32, *const i16),mut _8: (i128, (u8, i32, *const i16)),mut _9: (i128, (u8, i32, *const i16)),mut _10: (u8, i32, *const i16),mut _11: (u8, i32, *const i16),mut _12: bool,mut _13: u8) -> u8 {
mir! {
type RET = u8;
let _14: i64;
let _15: isize;
let _16: f64;
let _17: (i128, (u8, i32, *const i16));
let _18: [u32; 6];
let _19: [i128; 8];
let _20: i32;
let _21: u128;
let _22: Adt54;
let _23: f64;
let _24: (i64, i64);
let _25: f32;
let _26: f64;
let _27: f64;
let _28: isize;
let _29: f64;
let _30: [isize; 2];
let _31: [isize; 2];
let _32: isize;
let _33: ();
let _34: ();
{
_8.1.2 = core::ptr::addr_of!(_1);
RET = 0_usize as u8;
_7.2 = core::ptr::addr_of!(_1);
_11.2 = core::ptr::addr_of!(_2);
_7.0 = !_11.0;
_8.1.1 = _10.1;
_13 = !_9.1.0;
_7.2 = core::ptr::addr_of!(_2);
_8.1.1 = _10.1;
_7.0 = _13;
_9.1 = (_11.0, _11.1, _11.2);
_14 = !4920399492184850196_i64;
_9.1 = (_11.0, _10.1, _8.1.2);
_11.1 = _8.1.1 << _11.0;
_7.2 = core::ptr::addr_of!(_2);
Goto(bb1)
}
bb1 = {
_16 = 54763_u16 as f64;
_9.1.0 = !_8.1.0;
Goto(bb2)
}
bb2 = {
_10 = (_9.1.0, _11.1, _11.2);
_15 = (-9223372036854775808_isize) * 119_isize;
_7.1 = -_11.1;
_17.1.0 = !_3;
_9.1.2 = _10.2;
_20 = !_7.1;
_19 = [_8.0,_8.0,_9.0,_9.0,_8.0,_9.0,_9.0,_8.0];
_7.2 = _8.1.2;
_6 = 1214969708_u32;
_1 = _2 << _10.1;
_2 = !_1;
RET = !_13;
_17.1.1 = _1 as i32;
_13 = _8.1.0;
RET = _13;
_3 = _8.1.0;
match _9.0 {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
298801222544434412651252737470402182639 => bb9,
_ => bb8
}
}
bb3 = {
_16 = 54763_u16 as f64;
_9.1.0 = !_8.1.0;
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
_8.0 = !_9.0;
_9.1.0 = !_7.0;
_2 = !_1;
_14 = (-2996145349796942116_i64);
_13 = _12 as u8;
_3 = !_8.1.0;
_4 = [19171_u16,33741_u16,54323_u16,28931_u16,32628_u16,32805_u16,20268_u16,62351_u16];
_11 = (_17.1.0, _10.1, _8.1.2);
_9 = (_8.0, _10);
_17 = (_8.0, _11);
Goto(bb10)
}
bb10 = {
_23 = _16 + _16;
_3 = _9.1.0 * _8.1.0;
Call(_10.0 = core::intrinsics::transmute(_7.0), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_21 = 36102_u16 as u128;
_9.1.2 = core::ptr::addr_of!(_2);
_17.1.0 = !_8.1.0;
_11.1 = 32736_u16 as i32;
_5 = [_15,_15,_15,_15];
_4 = [14114_u16,62095_u16,9854_u16,36230_u16,1129_u16,27749_u16,30369_u16,49839_u16];
_10 = _7;
_14 = (-5513149776737591684_i64);
match _6 {
0 => bb5,
1214969708 => bb12,
_ => bb2
}
}
bb12 = {
_24.1 = -_14;
_15 = 9223372036854775807_isize - 9223372036854775807_isize;
_19 = [_8.0,_9.0,_8.0,_8.0,_17.0,_17.0,_8.0,_9.0];
_17.1.0 = _9.1.0;
_17.1.1 = _10.1;
_7 = _9.1;
_10.2 = _7.2;
_20 = _9.1.1;
_9 = (_8.0, _17.1);
_10.0 = _11.0 + _9.1.0;
_10.2 = _11.2;
_18 = [_6,_6,_6,_6,_6,_6];
_16 = _23;
_8.1.0 = _6 as u8;
_11.0 = _10.0 % RET;
_10.1 = !_7.1;
_25 = 3_usize as f32;
_17.1.1 = _15 as i32;
_7.0 = !_9.1.0;
_8 = _17;
_25 = 324957228753330954_u64 as f32;
Goto(bb13)
}
bb13 = {
_19 = [_17.0,_9.0,_8.0,_17.0,_9.0,_9.0,_8.0,_8.0];
_19 = [_17.0,_9.0,_8.0,_9.0,_9.0,_17.0,_9.0,_17.0];
_6 = 888902683_u32;
_10.1 = _8.1.1 | _7.1;
_6 = !4202044710_u32;
_11.1 = -_20;
_6 = 1584233709_u32;
_26 = _6 as f64;
_17 = _9;
_13 = _16 as u8;
_17.1 = (_11.0, _8.1.1, _10.2);
_10 = (_9.1.0, _20, _7.2);
_17.1.0 = _13;
_10.0 = _8.1.0;
_17.1 = _11;
_24 = (_14, _14);
_8.1.2 = core::ptr::addr_of!(_1);
RET = _8.1.0 >> _10.0;
match _6 {
0 => bb1,
1 => bb9,
2 => bb3,
3 => bb7,
1584233709 => bb15,
_ => bb14
}
}
bb14 = {
Return()
}
bb15 = {
_18 = [_6,_6,_6,_6,_6,_6];
_17.1.1 = _11.1 | _11.1;
_20 = _21 as i32;
_9.1 = (_17.1.0, _17.1.1, _8.1.2);
_17.0 = _9.0 << _2;
_2 = 431157901296562062_u64 as i16;
_7.1 = 25574_u16 as i32;
_31 = [_15,_15];
_13 = _1 as u8;
_10.2 = _11.2;
Goto(bb16)
}
bb16 = {
Call(_33 = dump_var(10_usize, 31_usize, Move(_31), 24_usize, Move(_24), 1_usize, Move(_1), 21_usize, Move(_21)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_33 = dump_var(10_usize, 18_usize, Move(_18), 3_usize, Move(_3), 2_usize, Move(_2), 4_usize, Move(_4)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: [isize; 4],mut _2: [u32; 6],mut _3: [u32; 6],mut _4: [isize; 4],mut _5: [u32; 6],mut _6: [u32; 6],mut _7: [u32; 6]) -> isize {
mir! {
type RET = isize;
let _8: u64;
let _9: Adt45;
let _10: usize;
let _11: *mut u32;
let _12: *const i16;
let _13: u8;
let _14: bool;
let _15: ([i32; 7], u32, *const isize);
let _16: (i64, i64);
let _17: i16;
let _18: f64;
let _19: Adt49;
let _20: Adt51;
let _21: char;
let _22: [i32; 7];
let _23: [isize; 4];
let _24: [i32; 7];
let _25: f64;
let _26: [i128; 8];
let _27: (i8, *const (isize, bool, (u8, i32, *const i16)));
let _28: u16;
let _29: *mut i32;
let _30: u16;
let _31: (i64, i64);
let _32: i64;
let _33: *const i16;
let _34: [i128; 8];
let _35: u16;
let _36: f64;
let _37: Adt53;
let _38: [i128; 8];
let _39: *mut u32;
let _40: ();
let _41: ();
{
_7 = [775843698_u32,1353344233_u32,3418716939_u32,1055113209_u32,2074660922_u32,535122783_u32];
_2 = [1233880753_u32,391299724_u32,3372154799_u32,2347540720_u32,3024304994_u32,1154751012_u32];
RET = -(-19_isize);
_6 = [1184090037_u32,1387286047_u32,225931175_u32,2416471736_u32,1306973378_u32,3322900290_u32];
_7 = _5;
_3 = _7;
_7 = [2264571679_u32,1564411788_u32,3343544302_u32,3232466485_u32,1106578207_u32,878243816_u32];
_4 = [RET,RET,RET,RET];
_1 = _4;
_2 = [2541997197_u32,4183118085_u32,4128402622_u32,4128245116_u32,599174845_u32,3047310951_u32];
RET = '\u{171c6}' as isize;
_2 = [1727206370_u32,1540107832_u32,737539592_u32,550757204_u32,2133906405_u32,4289399552_u32];
_1 = _4;
_8 = 9_u8 as u64;
_7 = [3669341716_u32,3751756171_u32,925945512_u32,1140037468_u32,499546056_u32,555123478_u32];
_9.fld3 = 121618721002579890332379115254590770410_i128 & 72598358044202338068961257925797881387_i128;
_9.fld2 = RET + RET;
_10 = !1203422446257022805_usize;
_3 = [1855866801_u32,1547104934_u32,1912740864_u32,264252849_u32,3412742567_u32,3281734537_u32];
_8 = 10555002265473358763_u64 >> _10;
_1 = [_9.fld2,RET,_9.fld2,_9.fld2];
_3 = _5;
_9.fld5.1 = _9.fld3 as i64;
Goto(bb1)
}
bb1 = {
_9.fld5 = ((-1876636750998196561_i64), 5566777419714501209_i64);
_3 = [3489309664_u32,3286094541_u32,173497511_u32,2394392369_u32,404441054_u32,258987539_u32];
_6 = [922425895_u32,365231775_u32,177723949_u32,2307756552_u32,1965095260_u32,3086510970_u32];
_7 = [4043182771_u32,823380884_u32,2701774335_u32,1612807497_u32,3602521676_u32,821673206_u32];
_1 = _4;
_9.fld2 = RET & RET;
_9.fld5 = ((-594853301684689773_i64), 4976726535692656717_i64);
_9.fld0 = core::ptr::addr_of_mut!(_9.fld4);
_9.fld2 = !RET;
_9.fld3 = !169465228431596288561226392392069334893_i128;
_8 = !3292889856181621784_u64;
Call(_9.fld5.1 = fn12(_9.fld5.0, _5, _2, _9.fld5.0, _9.fld5.0, _2, _5, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
RET = _9.fld2;
_5 = [3391534184_u32,2847604616_u32,747435007_u32,730981558_u32,2181184020_u32,946613410_u32];
_9.fld2 = _10 as isize;
_4 = _1;
_9.fld5.1 = _9.fld5.0 >> _9.fld3;
_7 = [1287981322_u32,3852118955_u32,4193355315_u32,3385075520_u32,2858471600_u32,2594054666_u32];
_9.fld5 = (3971543311299520954_i64, 9121084377178474958_i64);
RET = _8 as isize;
_10 = 2_usize << _9.fld2;
_9.fld5.0 = _9.fld5.1;
_14 = _9.fld3 <= _9.fld3;
_9.fld5.0 = 24919745246876570609293740572884070571_u128 as i64;
_15.2 = core::ptr::addr_of!(_9.fld2);
_12 = core::ptr::addr_of!(_17);
_15.2 = core::ptr::addr_of!(_9.fld2);
_10 = _14 as usize;
_15.1 = _9.fld3 as u32;
RET = _9.fld2 + _9.fld2;
_15.0 = [(-1289207463_i32),(-340295444_i32),328961232_i32,1272877639_i32,152823051_i32,376443310_i32,248802477_i32];
_19.fld1 = core::ptr::addr_of!((*_12));
(*_12) = !25398_i16;
_22 = _15.0;
Goto(bb3)
}
bb3 = {
_19.fld5 = (-2031342905_i32) | (-214456342_i32);
_16.1 = _9.fld5.1;
_13 = 210_u8;
_11 = core::ptr::addr_of_mut!(_15.1);
_19.fld3 = (-104_i8) ^ (-2_i8);
_7 = [_15.1,_15.1,(*_11),(*_11),(*_11),(*_11)];
_16.0 = _9.fld5.1 | _16.1;
_9.fld5.0 = _16.0 >> _16.1;
Goto(bb4)
}
bb4 = {
_7 = _2;
_19.fld2 = (_19.fld1,);
(*_11) = 3641343043_u32 >> _16.1;
_20.fld0 = _19.fld3 as i16;
_20.fld1 = _13;
_14 = false;
_9.fld1 = core::ptr::addr_of!((*_11));
_25 = _9.fld5.0 as f64;
_3 = _2;
_23 = [_9.fld2,_9.fld2,RET,_9.fld2];
_22 = [_19.fld5,_19.fld5,_19.fld5,_19.fld5,_19.fld5,_19.fld5,_19.fld5];
RET = _9.fld2 * _9.fld2;
_8 = 17397360607424540832_u64 << _9.fld5.0;
_24 = _22;
_25 = 124636054895008100767227053777393795447_u128 as f64;
(*_11) = 1537252716_u32 << _9.fld5.0;
_19.fld2 = (_12,);
_17 = 178870376965462056121425369153444935331_u128 as i16;
Goto(bb5)
}
bb5 = {
_3 = _2;
_2 = _6;
_31.0 = _14 as i64;
_31.1 = '\u{20d44}' as i64;
match _20.fld1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
6 => bb8,
210 => bb10,
_ => bb9
}
}
bb6 = {
_7 = _2;
_19.fld2 = (_19.fld1,);
(*_11) = 3641343043_u32 >> _16.1;
_20.fld0 = _19.fld3 as i16;
_20.fld1 = _13;
_14 = false;
_9.fld1 = core::ptr::addr_of!((*_11));
_25 = _9.fld5.0 as f64;
_3 = _2;
_23 = [_9.fld2,_9.fld2,RET,_9.fld2];
_22 = [_19.fld5,_19.fld5,_19.fld5,_19.fld5,_19.fld5,_19.fld5,_19.fld5];
RET = _9.fld2 * _9.fld2;
_8 = 17397360607424540832_u64 << _9.fld5.0;
_24 = _22;
_25 = 124636054895008100767227053777393795447_u128 as f64;
(*_11) = 1537252716_u32 << _9.fld5.0;
_19.fld2 = (_12,);
_17 = 178870376965462056121425369153444935331_u128 as i16;
Goto(bb5)
}
bb7 = {
_19.fld5 = (-2031342905_i32) | (-214456342_i32);
_16.1 = _9.fld5.1;
_13 = 210_u8;
_11 = core::ptr::addr_of_mut!(_15.1);
_19.fld3 = (-104_i8) ^ (-2_i8);
_7 = [_15.1,_15.1,(*_11),(*_11),(*_11),(*_11)];
_16.0 = _9.fld5.1 | _16.1;
_9.fld5.0 = _16.0 >> _16.1;
Goto(bb4)
}
bb8 = {
RET = _9.fld2;
_5 = [3391534184_u32,2847604616_u32,747435007_u32,730981558_u32,2181184020_u32,946613410_u32];
_9.fld2 = _10 as isize;
_4 = _1;
_9.fld5.1 = _9.fld5.0 >> _9.fld3;
_7 = [1287981322_u32,3852118955_u32,4193355315_u32,3385075520_u32,2858471600_u32,2594054666_u32];
_9.fld5 = (3971543311299520954_i64, 9121084377178474958_i64);
RET = _8 as isize;
_10 = 2_usize << _9.fld2;
_9.fld5.0 = _9.fld5.1;
_14 = _9.fld3 <= _9.fld3;
_9.fld5.0 = 24919745246876570609293740572884070571_u128 as i64;
_15.2 = core::ptr::addr_of!(_9.fld2);
_12 = core::ptr::addr_of!(_17);
_15.2 = core::ptr::addr_of!(_9.fld2);
_10 = _14 as usize;
_15.1 = _9.fld3 as u32;
RET = _9.fld2 + _9.fld2;
_15.0 = [(-1289207463_i32),(-340295444_i32),328961232_i32,1272877639_i32,152823051_i32,376443310_i32,248802477_i32];
_19.fld1 = core::ptr::addr_of!((*_12));
(*_12) = !25398_i16;
_22 = _15.0;
Goto(bb3)
}
bb9 = {
_9.fld5 = ((-1876636750998196561_i64), 5566777419714501209_i64);
_3 = [3489309664_u32,3286094541_u32,173497511_u32,2394392369_u32,404441054_u32,258987539_u32];
_6 = [922425895_u32,365231775_u32,177723949_u32,2307756552_u32,1965095260_u32,3086510970_u32];
_7 = [4043182771_u32,823380884_u32,2701774335_u32,1612807497_u32,3602521676_u32,821673206_u32];
_1 = _4;
_9.fld2 = RET & RET;
_9.fld5 = ((-594853301684689773_i64), 4976726535692656717_i64);
_9.fld0 = core::ptr::addr_of_mut!(_9.fld4);
_9.fld2 = !RET;
_9.fld3 = !169465228431596288561226392392069334893_i128;
_8 = !3292889856181621784_u64;
Call(_9.fld5.1 = fn12(_9.fld5.0, _5, _2, _9.fld5.0, _9.fld5.0, _2, _5, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_19.fld3 = (*_11) as i8;
_4 = [RET,_9.fld2,_9.fld2,_9.fld2];
_33 = _12;
_9.fld2 = RET;
(*_12) = _9.fld5.0 as i16;
_33 = _12;
_9.fld5 = (_16.0, _16.0);
(*_33) = _31.1 as i16;
_20.fld0 = (*_12);
(*_12) = -_20.fld0;
_7 = _6;
_9.fld5.1 = !_9.fld5.0;
(*_33) = _20.fld0;
_29 = core::ptr::addr_of_mut!(_19.fld5);
_19.fld5 = (-1655524208_i32) ^ (-789435105_i32);
_22 = [(*_29),(*_29),(*_29),(*_29),(*_29),(*_29),(*_29)];
Goto(bb11)
}
bb11 = {
_25 = 277559810261097320527939905500082482219_u128 as f64;
_9.fld3 = 26460911102031215064991085398819678463_u128 as i128;
_18 = _25 - _25;
match _16.1 {
0 => bb9,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb12,
9121084377178474958 => bb14,
_ => bb13
}
}
bb12 = {
_3 = _2;
_2 = _6;
_31.0 = _14 as i64;
_31.1 = '\u{20d44}' as i64;
match _20.fld1 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb6,
5 => bb7,
6 => bb8,
210 => bb10,
_ => bb9
}
}
bb13 = {
RET = _9.fld2;
_5 = [3391534184_u32,2847604616_u32,747435007_u32,730981558_u32,2181184020_u32,946613410_u32];
_9.fld2 = _10 as isize;
_4 = _1;
_9.fld5.1 = _9.fld5.0 >> _9.fld3;
_7 = [1287981322_u32,3852118955_u32,4193355315_u32,3385075520_u32,2858471600_u32,2594054666_u32];
_9.fld5 = (3971543311299520954_i64, 9121084377178474958_i64);
RET = _8 as isize;
_10 = 2_usize << _9.fld2;
_9.fld5.0 = _9.fld5.1;
_14 = _9.fld3 <= _9.fld3;
_9.fld5.0 = 24919745246876570609293740572884070571_u128 as i64;
_15.2 = core::ptr::addr_of!(_9.fld2);
_12 = core::ptr::addr_of!(_17);
_15.2 = core::ptr::addr_of!(_9.fld2);
_10 = _14 as usize;
_15.1 = _9.fld3 as u32;
RET = _9.fld2 + _9.fld2;
_15.0 = [(-1289207463_i32),(-340295444_i32),328961232_i32,1272877639_i32,152823051_i32,376443310_i32,248802477_i32];
_19.fld1 = core::ptr::addr_of!((*_12));
(*_12) = !25398_i16;
_22 = _15.0;
Goto(bb3)
}
bb14 = {
_32 = _9.fld2 as i64;
_19.fld5 = -(-1619217180_i32);
_16 = (_32, _9.fld5.0);
_5 = [_15.1,_15.1,(*_11),_15.1,_15.1,(*_11)];
_4 = _1;
(*_11) = _10 as u32;
_10 = _31.1 as usize;
_20 = Adt51 { fld0: _17,fld1: _13 };
_21 = '\u{84275}';
_5 = _6;
_9.fld3 = (-130569974710087310052755317202051852850_i128);
_29 = core::ptr::addr_of_mut!((*_29));
_16.0 = _9.fld5.1;
_9.fld5.1 = _32 ^ _9.fld5.0;
_6 = [(*_11),(*_11),(*_11),_15.1,_15.1,(*_11)];
_23 = [_9.fld2,RET,RET,_9.fld2];
_16.0 = _32;
_34 = [_9.fld3,_9.fld3,_9.fld3,_9.fld3,_9.fld3,_9.fld3,_9.fld3,_9.fld3];
_9.fld0 = core::ptr::addr_of_mut!(_15.2);
_38 = [_9.fld3,_9.fld3,_9.fld3,_9.fld3,_9.fld3,_9.fld3,_9.fld3,_9.fld3];
Goto(bb15)
}
bb15 = {
Call(_40 = dump_var(11_usize, 5_usize, Move(_5), 32_usize, Move(_32), 31_usize, Move(_31), 13_usize, Move(_13)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_40 = dump_var(11_usize, 8_usize, Move(_8), 17_usize, Move(_17), 21_usize, Move(_21), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_40 = dump_var(11_usize, 34_usize, Move(_34), 1_usize, Move(_1), 41_usize, _41, 41_usize, _41), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn12(mut _1: i64,mut _2: [u32; 6],mut _3: [u32; 6],mut _4: i64,mut _5: i64,mut _6: [u32; 6],mut _7: [u32; 6],mut _8: [u32; 6]) -> i64 {
mir! {
type RET = i64;
let _9: f64;
let _10: f32;
let _11: u32;
let _12: (i64, i64);
let _13: isize;
let _14: isize;
let _15: f32;
let _16: (i64, i64);
let _17: [isize; 2];
let _18: Adt58;
let _19: ();
let _20: ();
{
RET = _5;
_5 = _4 | _1;
_9 = 13685933673752129413_usize as f64;
_1 = '\u{58948}' as i64;
_9 = 5123005622403985836_usize as f64;
_1 = '\u{f01f5}' as i64;
_9 = 37843_u16 as f64;
_3 = [939616516_u32,3792877882_u32,250807820_u32,1466698741_u32,1498430696_u32,3414042074_u32];
RET = 32354_i16 as i64;
_2 = [3576804988_u32,3311417700_u32,2810342233_u32,1859145497_u32,4077736247_u32,2560180560_u32];
_3 = [3811594389_u32,2578258326_u32,727164637_u32,669947750_u32,3625328341_u32,383804320_u32];
_1 = -_4;
_4 = !_5;
_12.1 = _1;
_12 = (_5, _1);
_12 = (_4, _4);
_12.0 = !_4;
_5 = !_12.1;
_9 = 136594851_i32 as f64;
_6 = [2240246610_u32,3468027526_u32,863766656_u32,759304496_u32,4256678155_u32,1749232366_u32];
_10 = (-148137901158395787629644014594001430706_i128) as f32;
_2 = [1694139622_u32,3069816611_u32,48777099_u32,1800651359_u32,3137924445_u32,2954738001_u32];
Call(RET = fn13(_6, _3, _7, _7, _12, _4, _12, _1, _9, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12.1 = !RET;
_8 = [2062056166_u32,1439896811_u32,1560329328_u32,670289301_u32,1462697951_u32,1810424358_u32];
_2 = [3543010310_u32,1386093166_u32,521387060_u32,3056100046_u32,3689377603_u32,1516037800_u32];
_10 = 2299003236_u32 as f32;
_12.1 = _9 as i64;
_4 = RET * _12.0;
_6 = [49529729_u32,457946593_u32,264724972_u32,4082369957_u32,1934352514_u32,1882298375_u32];
RET = 15868617742602409011_u64 as i64;
_16.0 = -_4;
_11 = 4134206019_u32;
_4 = 841933440_i32 as i64;
RET = _16.0 + _5;
_11 = 2111873002_u32 * 250663111_u32;
_14 = _11 as isize;
_16.1 = _5;
_5 = !_12.0;
_9 = _14 as f64;
_5 = _16.0;
_18.fld1.0 = (-80316432561039387507214658310908024192_i128) + (-16501041938297933444362121711166975383_i128);
_18.fld0 = [_14,_14];
_12.0 = 11528576339803274890_u64 as i64;
_13 = (-1239954398_i32) as isize;
RET = _5 >> _5;
_5 = -_16.0;
_12 = (_5, RET);
_18.fld0 = [_14,_14];
Goto(bb2)
}
bb2 = {
Call(_19 = dump_var(12_usize, 11_usize, Move(_11), 8_usize, Move(_8), 4_usize, Move(_4), 1_usize, Move(_1)), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
Call(_19 = dump_var(12_usize, 16_usize, Move(_16), 5_usize, Move(_5), 20_usize, _20, 20_usize, _20), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: [u32; 6],mut _2: [u32; 6],mut _3: [u32; 6],mut _4: [u32; 6],mut _5: (i64, i64),mut _6: i64,mut _7: (i64, i64),mut _8: i64,mut _9: f64,mut _10: [u32; 6]) -> i64 {
mir! {
type RET = i64;
let _11: f64;
let _12: i8;
let _13: f64;
let _14: (i64, i64);
let _15: bool;
let _16: char;
let _17: u32;
let _18: ();
let _19: ();
{
_10 = _3;
_10 = _1;
_7 = _5;
RET = _7.0 | _5.0;
_7 = (_6, _6);
_13 = _9 * _9;
_8 = _7.1 * _7.0;
RET = (-20737_i16) as i64;
_7.0 = _6;
_12 = 7_i8;
match _12 {
0 => bb1,
1 => bb2,
7 => bb4,
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
RET = 2867731310_u32 as i64;
_13 = -_9;
_5 = (_7.0, _7.0);
_12 = (-60_i8) * (-112_i8);
_14 = _7;
_14.1 = _14.0 << _6;
_11 = -_13;
_8 = _7.0 ^ _5.0;
_14.1 = !_7.1;
_3 = [184787758_u32,1248255296_u32,3387861429_u32,1215639528_u32,3075601364_u32,1192860110_u32];
Call(RET = core::intrinsics::bswap(_14.1), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
_4 = [2207965676_u32,2963629865_u32,2108987277_u32,192610161_u32,1400893319_u32,81140677_u32];
_8 = RET;
_8 = _14.1 * _7.1;
_2 = [521756178_u32,45560707_u32,3219617590_u32,428054560_u32,857809880_u32,2796063274_u32];
_7.0 = !_5.0;
_7.0 = -_14.0;
_4 = _1;
_15 = !true;
_8 = _7.0;
_4 = _3;
RET = !_8;
_17 = !3366248526_u32;
_15 = true;
_13 = _9 - _9;
_12 = (-99_i8);
_5.0 = -_7.1;
RET = !_7.1;
_8 = _5.0 & _5.1;
RET = _8 | _8;
_7 = (_14.1, RET);
_5 = (_7.1, _14.0);
Goto(bb6)
}
bb6 = {
Call(_18 = dump_var(13_usize, 3_usize, Move(_3), 8_usize, Move(_8), 4_usize, Move(_4), 12_usize, Move(_12)), ReturnTo(bb7), UnwindUnreachable())
}
bb7 = {
Call(_18 = dump_var(13_usize, 15_usize, Move(_15), 14_usize, Move(_14), 19_usize, _19, 19_usize, _19), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn14(mut _1: [isize; 4],mut _2: [u32; 6],mut _3: [i32; 7],mut _4: i64,mut _5: [u32; 6],mut _6: [isize; 4],mut _7: [u32; 6],mut _8: ([i32; 7], u32, *const isize),mut _9: ([i32; 7], u32, *const isize),mut _10: ([i32; 7], u32, *const isize),mut _11: [isize; 4],mut _12: [i32; 7],mut _13: *const u32,mut _14: ([i32; 7], u32, *const isize),mut _15: u32,mut _16: ([i32; 7], u32, *const isize)) -> [u32; 6] {
mir! {
type RET = [u32; 6];
let _17: isize;
let _18: [i32; 7];
let _19: u8;
let _20: f32;
let _21: i8;
let _22: (i64, i64);
let _23: isize;
let _24: (i64, i64);
let _25: u16;
let _26: &'static i64;
let _27: bool;
let _28: f64;
let _29: i8;
let _30: f64;
let _31: [u16; 8];
let _32: u64;
let _33: isize;
let _34: Adt44;
let _35: ();
let _36: ();
{
_8.1 = _14.1;
_5 = _7;
_16 = (_14.0, _9.1, _10.2);
_8.0 = [(-1523395789_i32),1035984021_i32,337704197_i32,(-810651069_i32),(-628413216_i32),(-1173795033_i32),332977039_i32];
_8.2 = core::ptr::addr_of!(_17);
_12 = _3;
_10.0 = [2015603463_i32,(-1503025642_i32),(-703202913_i32),256628443_i32,(-463172034_i32),(-758223212_i32),(-483561238_i32)];
_1 = [(-112_isize),(-9223372036854775808_isize),(-9223372036854775808_isize),9223372036854775807_isize];
_10 = _9;
_8.1 = 247897097383301136595255058239271589639_u128 as u32;
_7 = [_15,_14.1,_10.1,(*_13),_15,_15];
_4 = 8054883404451394522_i64;
Goto(bb1)
}
bb1 = {
_2 = [_9.1,_8.1,_15,_10.1,_16.1,_10.1];
_8.0 = _12;
RET = [_10.1,(*_13),(*_13),_9.1,_9.1,_15];
_8.0 = _16.0;
_2 = [_9.1,_8.1,_10.1,_16.1,_9.1,(*_13)];
_11 = _1;
RET = [(*_13),_15,_16.1,_9.1,(*_13),_9.1];
_11 = _1;
_4 = (-3507463114760341333_i64) + (-470670722724741790_i64);
_14.0 = [482643048_i32,1241020429_i32,1535660821_i32,(-389703898_i32),(-89215208_i32),(-1446691133_i32),(-1660435694_i32)];
_15 = _9.1;
(*_13) = !_15;
_8.2 = _9.2;
Goto(bb2)
}
bb2 = {
_14.0 = [(-1388739917_i32),(-2065265372_i32),1649689661_i32,(-1472905553_i32),(-954733984_i32),(-1191547894_i32),(-777421520_i32)];
_4 = (-5066412292833190347_i64);
_19 = !45_u8;
(*_13) = (-40396573853364447363447653538984862587_i128) as u32;
_10 = (_12, _9.1, _14.2);
_8.1 = _10.1 * _16.1;
(*_13) = 8_i8 as u32;
match _4 {
0 => bb1,
340282366920938463458308195138935021109 => bb4,
_ => bb3
}
}
bb3 = {
_2 = [_9.1,_8.1,_15,_10.1,_16.1,_10.1];
_8.0 = _12;
RET = [_10.1,(*_13),(*_13),_9.1,_9.1,_15];
_8.0 = _16.0;
_2 = [_9.1,_8.1,_10.1,_16.1,_9.1,(*_13)];
_11 = _1;
RET = [(*_13),_15,_16.1,_9.1,(*_13),_9.1];
_11 = _1;
_4 = (-3507463114760341333_i64) + (-470670722724741790_i64);
_14.0 = [482643048_i32,1241020429_i32,1535660821_i32,(-389703898_i32),(-89215208_i32),(-1446691133_i32),(-1660435694_i32)];
_15 = _9.1;
(*_13) = !_15;
_8.2 = _9.2;
Goto(bb2)
}
bb4 = {
_9 = (_16.0, _8.1, _8.2);
_2 = [(*_13),_8.1,_15,_14.1,_9.1,_9.1];
_10.1 = (*_13) + _8.1;
_8.2 = core::ptr::addr_of!(_17);
_15 = true as u32;
_10.2 = _14.2;
_9.2 = _14.2;
(*_13) = _10.1 - _8.1;
_14.2 = _9.2;
_8.0 = [(-382716695_i32),1195662934_i32,1722016008_i32,419003869_i32,1928402586_i32,23158835_i32,154908144_i32];
_9.1 = _8.1 - _16.1;
(*_13) = _9.1 - _10.1;
_22.1 = !_4;
_17 = (-9223372036854775808_isize) * 9223372036854775807_isize;
(*_13) = _10.1 - _14.1;
_21 = 16178366668329325783_usize as i8;
_9 = (_3, _16.1, _10.2);
_22 = (_4, _4);
RET = [(*_13),_10.1,(*_13),_10.1,(*_13),_14.1];
_18 = _10.0;
_21 = 64_i8;
_24.1 = _4;
Goto(bb5)
}
bb5 = {
_9.2 = core::ptr::addr_of!(_17);
_9 = (_16.0, (*_13), _16.2);
_14.1 = _16.1;
_14.1 = (*_13);
_24.1 = _4 * _22.0;
_20 = _19 as f32;
_24.1 = _20 as i64;
_14.2 = core::ptr::addr_of!(_23);
_8.0 = _9.0;
_9.1 = (-88676151697415092940810154297068246749_i128) as u32;
_8.0 = [1473234747_i32,881073087_i32,(-973535430_i32),(-1641709561_i32),919859621_i32,(-1998133938_i32),(-1245087524_i32)];
_1 = _11;
_24 = _22;
_9 = (_10.0, _16.1, _16.2);
_9 = (_8.0, _10.1, _10.2);
_22.1 = 11609_u16 as i64;
match _22.0 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb6,
340282366920938463458308195138935021109 => bb8,
_ => bb7
}
}
bb6 = {
_14.0 = [(-1388739917_i32),(-2065265372_i32),1649689661_i32,(-1472905553_i32),(-954733984_i32),(-1191547894_i32),(-777421520_i32)];
_4 = (-5066412292833190347_i64);
_19 = !45_u8;
(*_13) = (-40396573853364447363447653538984862587_i128) as u32;
_10 = (_12, _9.1, _14.2);
_8.1 = _10.1 * _16.1;
(*_13) = 8_i8 as u32;
match _4 {
0 => bb1,
340282366920938463458308195138935021109 => bb4,
_ => bb3
}
}
bb7 = {
_2 = [_9.1,_8.1,_15,_10.1,_16.1,_10.1];
_8.0 = _12;
RET = [_10.1,(*_13),(*_13),_9.1,_9.1,_15];
_8.0 = _16.0;
_2 = [_9.1,_8.1,_10.1,_16.1,_9.1,(*_13)];
_11 = _1;
RET = [(*_13),_15,_16.1,_9.1,(*_13),_9.1];
_11 = _1;
_4 = (-3507463114760341333_i64) + (-470670722724741790_i64);
_14.0 = [482643048_i32,1241020429_i32,1535660821_i32,(-389703898_i32),(-89215208_i32),(-1446691133_i32),(-1660435694_i32)];
_15 = _9.1;
(*_13) = !_15;
_8.2 = _9.2;
Goto(bb2)
}
bb8 = {
_23 = -_17;
_16.1 = _14.1 >> (*_13);
_10.0 = [(-643035794_i32),290739209_i32,(-990588977_i32),394371859_i32,(-637457933_i32),(-1603120790_i32),741461432_i32];
_30 = _9.1 as f64;
_6 = [_23,_17,_17,_23];
_9.0 = [1150319015_i32,(-1487473512_i32),(-2027387330_i32),1208309782_i32,(-732296717_i32),(-1974079215_i32),(-684858837_i32)];
_14.1 = _16.1;
_29 = -_21;
_24.0 = _20 as i64;
_20 = _29 as f32;
_19 = !0_u8;
_8 = (_16.0, (*_13), _9.2);
_8.0 = [731532241_i32,(-1870750220_i32),(-1240213133_i32),(-662955994_i32),1894987058_i32,1443228802_i32,1776587274_i32];
_30 = _20 as f64;
_28 = _30 - _30;
_33 = _20 as isize;
_10.0 = _18;
_2 = [_16.1,_14.1,(*_13),_16.1,(*_13),_14.1];
Call(RET = core::intrinsics::transmute(_2), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
_12 = [1739057059_i32,1181282846_i32,(-1830757213_i32),(-1060350939_i32),27743783_i32,(-2090617642_i32),(-652471541_i32)];
_10.2 = _14.2;
_22 = (_4, _4);
_33 = _23 >> _17;
_27 = !true;
Goto(bb10)
}
bb10 = {
Call(_35 = dump_var(14_usize, 29_usize, Move(_29), 1_usize, Move(_1), 17_usize, Move(_17), 12_usize, Move(_12)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Call(_35 = dump_var(14_usize, 7_usize, Move(_7), 27_usize, Move(_27), 4_usize, Move(_4), 15_usize, Move(_15)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
Call(_35 = dump_var(14_usize, 18_usize, Move(_18), 22_usize, Move(_22), 36_usize, _36, 36_usize, _36), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(135740683910909737122656451764508473462_u128), std::hint::black_box(64529_u16), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(21_i8), std::hint::black_box(7513225479086515174_u64), std::hint::black_box(2010546438_i32), std::hint::black_box(1948107854155033823_i64));
                
            }
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt44::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: (*const i16,),
fld1: [isize; 2],
fld2: isize,
fld3: u64,
fld4: *const isize,
fld5: (i128, (u8, i32, *const i16)),
fld6: *mut *const isize,

},
Variant1{
fld0: *mut usize,
fld1: *mut i32,
fld2: (u8, i32, *const i16),

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt45{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt45 {
fld0: *mut *const isize,
fld1: *const u32,
fld2: isize,
fld3: i128,
fld4: *const isize,
fld5: (i64, i64),
}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt46::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: *const isize,
fld1: *mut usize,
fld2: [u16; 8],
fld3: i64,

},
Variant1{
fld0: *const u32,
fld1: i128,
fld2: *mut u32,

},
Variant2{
fld0: u128,
fld1: i128,
fld2: (*mut i32, i16, ([i32; 7], u32, *const isize)),

},
Variant3{
fld0: f64,
fld1: (u8, i32, *const i16),
fld2: *mut (i64, i64),
fld3: ([i32; 7], u32, *const isize),
fld4: u128,
fld5: (i128, (u8, i32, *const i16)),
fld6: [isize; 4],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt47::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: bool,
fld1: u128,
fld2: *mut *const (i128, (u8, i32, *const i16)),
fld3: u8,
fld4: Adt46,
fld5: *const (i128, (u8, i32, *const i16)),
fld6: [i32; 7],
fld7: [u16; 8],

},
Variant1{
fld0: (u8, i32, *const i16),

},
Variant2{
fld0: i64,
fld1: char,
fld2: *const u32,

},
Variant3{
fld0: *const isize,
fld1: (*mut i32, i16, ([i32; 7], u32, *const isize)),
fld2: ([i32; 7], u32, *const isize),
fld3: [u32; 6],
fld4: (i64, i64),
fld5: i32,
fld6: *mut (i64, i64),

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt48::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt48 {
Variant0{
fld0: *mut usize,
fld1: char,
fld2: [isize; 4],

},
Variant1{
fld0: [isize; 2],
fld1: (*mut i32, i16, ([i32; 7], u32, *const isize)),
fld2: u16,
fld3: u128,
fld4: *const (i128, (u8, i32, *const i16)),

},
Variant2{
fld0: (*const i16,),
fld1: u16,
fld2: (i64, i64),

},
Variant3{
fld0: (*const i16,),
fld1: (i8, *const (isize, bool, (u8, i32, *const i16))),
fld2: f32,
fld3: [isize; 2],
fld4: *const u32,
fld5: u64,
fld6: Adt46,

}}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt49{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt49 {
fld0: bool,
fld1: *const i16,
fld2: (*const i16,),
fld3: i8,
fld4: [u64; 3],
fld5: i32,
fld6: u16,
}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt50::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt50 {
Variant0{
fld0: [isize; 4],
fld1: char,
fld2: *mut (i64, i64),
fld3: u32,
fld4: usize,
fld5: u128,
fld6: *const u32,

},
Variant1{
fld0: [u16; 8],
fld1: *mut usize,
fld2: isize,
fld3: (i8, *const (isize, bool, (u8, i32, *const i16))),

}}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: i16,
fld1: u8,
}
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
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: *mut usize,
fld1: *mut *const (i128, (u8, i32, *const i16)),
fld2: u16,
fld3: [u64; 3],
fld4: u8,

},
Variant1{
fld0: *const i16,
fld1: i128,
fld2: Adt47,
fld3: Adt51,

},
Variant2{
fld0: [isize; 4],
fld1: *mut *const isize,
fld2: [i32; 7],
fld3: i8,
fld4: (*const i16,),
fld5: Adt45,

},
Variant3{
fld0: *const u32,

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
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
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
		unsafe{printf(c"fld7:".as_ptr())};
		fld7.printf_debug();
unsafe{printf(c",".as_ptr())};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: Adt44,
fld1: [isize; 4],
fld2: (isize, bool, (u8, i32, *const i16)),
fld3: *mut *const (i128, (u8, i32, *const i16)),
fld4: i128,
fld5: [i32; 7],
fld6: i64,

},
Variant1{
fld0: *mut *const (i128, (u8, i32, *const i16)),

},
Variant2{
fld0: (*mut i32, i16, ([i32; 7], u32, *const isize)),
fld1: char,
fld2: [i32; 7],
fld3: usize,
fld4: i16,
fld5: f32,

},
Variant3{
fld0: [u64; 3],
fld1: *const i16,
fld2: isize,
fld3: u8,
fld4: *const u32,
fld5: f64,
fld6: *mut usize,
fld7: f32,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt54::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt54 {
Variant0{
fld0: *const isize,
fld1: char,
fld2: *mut *const (i128, (u8, i32, *const i16)),
fld3: [u64; 3],
fld4: u128,
fld5: usize,
fld6: [u32; 6],

},
Variant1{
fld0: Adt49,
fld1: [i32; 7],
fld2: isize,
fld3: Adt50,
fld4: Adt47,
fld5: i64,

},
Variant2{
fld0: i64,
fld1: (i128, (u8, i32, *const i16)),
fld2: *mut u32,
fld3: i8,
fld4: [u64; 3],
fld5: i128,

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
	Self::Variant3{fld0,fld1,fld2,fld3,}=>{
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
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: (i64, i64),
fld1: [i32; 7],
fld2: *mut (i64, i64),
fld3: [isize; 2],
fld4: *mut u32,
fld5: i32,
fld6: u16,
fld7: [u16; 8],

},
Variant1{
fld0: [u64; 3],
fld1: (i128, (u8, i32, *const i16)),
fld2: [isize; 2],
fld3: (*mut i32, i16, ([i32; 7], u32, *const isize)),
fld4: (u8, i32, *const i16),
fld5: *mut *const isize,

},
Variant2{
fld0: Adt49,
fld1: char,
fld2: f64,
fld3: [i128; 8],
fld4: i16,
fld5: Adt52,
fld6: [isize; 4],
fld7: Adt47,

},
Variant3{
fld0: u64,
fld1: (u8, i32, *const i16),
fld2: [u64; 3],
fld3: Adt45,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt56::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: (i8, *const (isize, bool, (u8, i32, *const i16))),
fld1: *const i16,
fld2: Adt44,
fld3: *mut *const (i128, (u8, i32, *const i16)),
fld4: Adt54,
fld5: i32,
fld6: *const isize,
fld7: (i128, (u8, i32, *const i16)),

},
Variant1{
fld0: Adt50,
fld1: i16,

},
Variant2{
fld0: i16,
fld1: *mut i32,
fld2: *const i16,
fld3: *const (i128, (u8, i32, *const i16)),

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
#[derive(Copy,Clone)]pub enum Adt57 {
Variant0{
fld0: *const i16,
fld1: u16,
fld2: *const (isize, bool, (u8, i32, *const i16)),

},
Variant1{
fld0: Adt46,
fld1: i16,

},
Variant2{
fld0: ([i32; 7], u32, *const isize),
fld1: *const u32,
fld2: Adt48,
fld3: [u16; 8],
fld4: Adt51,
fld5: i32,

},
Variant3{
fld0: usize,

}}
impl PrintFDebug for Adt58{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt58{ ".as_ptr()  as *const c_char)};
	unsafe{printf(c"}".as_ptr())};}
}
#[derive(Copy,Clone)]pub struct Adt58 {
fld0: [isize; 2],
fld1: (i128, (u8, i32, *const i16)),
fld2: Adt55,
}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt59::".as_ptr()  as *const c_char)};match self{
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
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: Adt45,
fld1: (i64, i64),
fld2: isize,
fld3: Adt51,
fld4: i128,

},
Variant1{
fld0: Adt45,
fld1: ([i32; 7], u32, *const isize),
fld2: [isize; 4],
fld3: i8,
fld4: *const i16,
fld5: Adt53,
fld6: (i128, (u8, i32, *const i16)),

}}
impl PrintFDebug for Adt60{
	unsafe fn printf_debug(&self){unsafe{printf(c"Adt60::".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt60 {
Variant0{
fld0: Adt59,
fld1: (i64, i64),
fld2: Adt54,
fld3: *mut (i64, i64),
fld4: (i8, *const (isize, bool, (u8, i32, *const i16))),
fld5: f64,
fld6: i64,
fld7: (isize, bool, (u8, i32, *const i16)),

},
Variant1{
fld0: u128,
fld1: u64,
fld2: Adt52,
fld3: f64,

},
Variant2{
fld0: u32,
fld1: [isize; 4],
fld2: [u32; 6],
fld3: [i128; 8],
fld4: Adt47,
fld5: *const i16,
fld6: Adt49,

}}

