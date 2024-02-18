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
pub fn fn0(mut _1: bool,mut _2: char,mut _3: isize,mut _4: usize,mut _5: u32,mut _6: i32,mut _7: i64,mut _8: u64) -> [i16; 6] {
mir! {
type RET = [i16; 6];
let _9: (&'static [i128; 5],);
let _10: bool;
let _11: *const Adt57;
let _12: bool;
let _13: char;
let _14: bool;
let _15: f64;
let _16: *const [u32; 4];
let _17: [char; 7];
let _18: (u128,);
let _19: i8;
let _20: f64;
let _21: (i128,);
let _22: [i16; 6];
let _23: u32;
let _24: (u32, Adt57, &'static Adt26, u16);
let _25: isize;
let _26: char;
let _27: f32;
let _28: bool;
let _29: isize;
let _30: char;
let _31: ((f64, &'static [i128; 5]), [isize; 4], ((f32, isize), *const [u32; 4], &'static (f32, isize)));
let _32: [isize; 4];
let _33: char;
let _34: Adt78;
let _35: isize;
let _36: &'static [i128; 5];
let _37: ();
let _38: ();
{
RET = [(-24823_i16),(-15322_i16),(-29929_i16),(-1393_i16),(-1666_i16),(-2143_i16)];
Call(_5 = core::intrinsics::bswap(162528626_u32), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = '\u{9fb50}';
RET = [(-10520_i16),(-29137_i16),(-6759_i16),5071_i16,(-22130_i16),(-10445_i16)];
_1 = !false;
_6 = (-1922217679_i32);
_7 = _2 as i64;
_4 = 12398819138308814570_usize;
RET = [12512_i16,(-8809_i16),3410_i16,(-25827_i16),(-29575_i16),10082_i16];
_2 = '\u{103d3d}';
Goto(bb2)
}
bb2 = {
_3 = (-9223372036854775808_isize);
_8 = 7554593219056302465_u64;
_6 = _7 as i32;
_1 = true;
_5 = _1 as u32;
RET = [5409_i16,26694_i16,3799_i16,(-31530_i16),(-4449_i16),8207_i16];
RET = [(-2160_i16),(-2725_i16),31599_i16,(-24870_i16),(-6534_i16),30806_i16];
_7 = (-4069049339648670889_i64) << _6;
_12 = !_1;
_5 = 980462322_u32;
_5 = 2993638473_u32;
_8 = 10971355533579753336_u64;
_5 = !1914354729_u32;
_2 = '\u{a5783}';
RET = [15439_i16,7191_i16,27750_i16,4498_i16,(-578_i16),6409_i16];
_7 = 6511798026538465801_i64 << _4;
Call(_7 = fn1(RET, _1, _4, RET, RET, _2, _3, _8, _3, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = [29502_i16,20969_i16,(-4653_i16),(-7170_i16),10258_i16,30750_i16];
_3 = 9223372036854775807_isize >> _6;
_5 = (-25016_i16) as u32;
_6 = (-1497441972_i32);
_1 = !_12;
_2 = '\u{f1bb0}';
_2 = '\u{ad9c5}';
_3 = (-47_isize) | (-9223372036854775808_isize);
_10 = !_1;
_8 = 3154618742123910943_u64;
_13 = _2;
_13 = _2;
_5 = 437087803_u32 & 3600835479_u32;
_1 = _3 <= _3;
_4 = 0_usize << _6;
_5 = 2217539017_u32 | 1929034492_u32;
_15 = _7 as f64;
_14 = !_1;
_12 = !_14;
_6 = !(-952241233_i32);
_14 = !_12;
_4 = _5 as usize;
_4 = 1_usize << _7;
_4 = !13554341621407204667_usize;
_18 = (17593524111101625020435129804486352651_u128,);
_17 = [_2,_2,_2,_2,_2,_13,_2];
Goto(bb4)
}
bb4 = {
_17 = [_13,_13,_2,_2,_13,_2,_13];
_17 = [_2,_13,_2,_13,_13,_13,_13];
_4 = 14696924637286340020_usize;
_18.0 = 85990567901107903731101733849666198035_u128;
RET = [20077_i16,6485_i16,(-25282_i16),(-4376_i16),(-23090_i16),(-6325_i16)];
_2 = _13;
_8 = !16662506654313889966_u64;
Goto(bb5)
}
bb5 = {
_4 = _5 as usize;
_20 = _15 * _15;
_18.0 = (-122_i8) as u128;
Goto(bb6)
}
bb6 = {
_12 = _7 < _7;
_21.0 = -(-89813892324391375427028272542580028940_i128);
RET = [16495_i16,(-7886_i16),15979_i16,(-16528_i16),(-24313_i16),(-30293_i16)];
_14 = _21.0 == _21.0;
_3 = !9223372036854775807_isize;
_8 = !5284508143851797180_u64;
_20 = _15 - _15;
_19 = (-79_i8) << _7;
_6 = !(-77608160_i32);
_12 = _14 ^ _1;
RET = [(-7391_i16),28910_i16,(-13527_i16),(-13805_i16),(-8398_i16),(-10435_i16)];
_18 = (181663149596540832442667190313033586550_u128,);
_18.0 = !208947443520215348653587198776423439434_u128;
_1 = !_12;
_22 = [(-25044_i16),(-20609_i16),(-556_i16),6967_i16,21829_i16,(-13786_i16)];
_13 = _2;
_6 = 911487355_i32;
_7 = 29318_i16 as i64;
_2 = _13;
_17 = [_2,_2,_13,_13,_13,_13,_2];
_18 = (315431948494125121136842418214297936045_u128,);
_24.1.fld3 = (_4,);
_24.1.fld1 = !_24.1.fld3.0;
_5 = _19 as u32;
_24.1.fld2 = [_3,_3,_3,_3,_3,_3];
match _18.0 {
0 => bb1,
1 => bb4,
315431948494125121136842418214297936045 => bb7,
_ => bb5
}
}
bb7 = {
_13 = _2;
_8 = 9719867328771574212_u64;
_4 = _24.1.fld1 >> _21.0;
_21.0 = (-24587662401782933983857384325550325612_i128) - (-68934096671088325823748665404462813303_i128);
_24.3 = 225_u8 as u16;
_12 = _1;
_24.1.fld0 = _2 as i32;
match _8 {
0 => bb8,
1 => bb9,
9719867328771574212 => bb11,
_ => bb10
}
}
bb8 = {
_2 = '\u{9fb50}';
RET = [(-10520_i16),(-29137_i16),(-6759_i16),5071_i16,(-22130_i16),(-10445_i16)];
_1 = !false;
_6 = (-1922217679_i32);
_7 = _2 as i64;
_4 = 12398819138308814570_usize;
RET = [12512_i16,(-8809_i16),3410_i16,(-25827_i16),(-29575_i16),10082_i16];
_2 = '\u{103d3d}';
Goto(bb2)
}
bb9 = {
_4 = _5 as usize;
_20 = _15 * _15;
_18.0 = (-122_i8) as u128;
Goto(bb6)
}
bb10 = {
_3 = (-9223372036854775808_isize);
_8 = 7554593219056302465_u64;
_6 = _7 as i32;
_1 = true;
_5 = _1 as u32;
RET = [5409_i16,26694_i16,3799_i16,(-31530_i16),(-4449_i16),8207_i16];
RET = [(-2160_i16),(-2725_i16),31599_i16,(-24870_i16),(-6534_i16),30806_i16];
_7 = (-4069049339648670889_i64) << _6;
_12 = !_1;
_5 = 980462322_u32;
_5 = 2993638473_u32;
_8 = 10971355533579753336_u64;
_5 = !1914354729_u32;
_2 = '\u{a5783}';
RET = [15439_i16,7191_i16,27750_i16,4498_i16,(-578_i16),6409_i16];
_7 = 6511798026538465801_i64 << _4;
Call(_7 = fn1(RET, _1, _4, RET, RET, _2, _3, _8, _3, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_17 = [_13,_13,_13,_2,_2,_2,_2];
_18 = (235107677448169928556907889250627834892_u128,);
RET = [(-6820_i16),9142_i16,(-21821_i16),8810_i16,24773_i16,(-3423_i16)];
_19 = (-27_i8);
_18.0 = !323041873606863914905198704762541844651_u128;
_7 = (-8625675054604334309_i64) << _4;
_24.1.fld3 = (_4,);
RET = [(-1466_i16),(-17531_i16),31272_i16,18126_i16,6108_i16,31279_i16];
_18.0 = 161972664050335528796766573602739844164_u128;
_24.0 = _5 << _24.1.fld3.0;
_22 = [(-27808_i16),(-31648_i16),12425_i16,(-24122_i16),5460_i16,(-21433_i16)];
_24.1.fld4 = 29381_i16;
_24.1.fld1 = _3 as usize;
_24.1.fld3.0 = _4;
_4 = _12 as usize;
_14 = _12;
_24.1.fld1 = !_24.1.fld3.0;
_18.0 = 48103786796576957792208589093016133162_u128;
_8 = 7469459759292104703_u64;
_6 = _24.1.fld0;
Goto(bb12)
}
bb12 = {
_20 = -_15;
_12 = _4 >= _4;
_11 = core::ptr::addr_of!(_24.1);
_12 = _10;
_24.1.fld0 = -_6;
(*_11).fld3 = (_24.1.fld1,);
_26 = _13;
(*_11).fld0 = _6 << _4;
_24.3 = !58556_u16;
_24.1.fld4 = 21074_i16;
_24.1.fld3.0 = _14 as usize;
_24.1.fld4 = _18.0 as i16;
_27 = _24.3 as f32;
(*_11).fld4 = 13998_i16 ^ 2661_i16;
_11 = core::ptr::addr_of!((*_11));
Goto(bb13)
}
bb13 = {
_10 = _1 != _14;
RET = [(*_11).fld4,(*_11).fld4,(*_11).fld4,(*_11).fld4,(*_11).fld4,(*_11).fld4];
_25 = !_3;
_28 = _1;
_24.1.fld2 = [_3,_25,_3,_3,_3,_25];
_31.2.0 = (_27, _3);
_8 = !6343080598596241377_u64;
_31.0.0 = -_20;
_11 = core::ptr::addr_of!(_24.1);
_24.1.fld2 = [_31.2.0.1,_25,_25,_25,_3,_31.2.0.1];
(*_11).fld2 = [_31.2.0.1,_25,_31.2.0.1,_31.2.0.1,_25,_31.2.0.1];
_31.1 = [_25,_25,_3,_31.2.0.1];
_24.3 = _10 as u16;
match _18.0 {
0 => bb14,
1 => bb15,
2 => bb16,
3 => bb17,
4 => bb18,
48103786796576957792208589093016133162 => bb20,
_ => bb19
}
}
bb14 = {
_20 = -_15;
_12 = _4 >= _4;
_11 = core::ptr::addr_of!(_24.1);
_12 = _10;
_24.1.fld0 = -_6;
(*_11).fld3 = (_24.1.fld1,);
_26 = _13;
(*_11).fld0 = _6 << _4;
_24.3 = !58556_u16;
_24.1.fld4 = 21074_i16;
_24.1.fld3.0 = _14 as usize;
_24.1.fld4 = _18.0 as i16;
_27 = _24.3 as f32;
(*_11).fld4 = 13998_i16 ^ 2661_i16;
_11 = core::ptr::addr_of!((*_11));
Goto(bb13)
}
bb15 = {
_17 = [_13,_13,_13,_2,_2,_2,_2];
_18 = (235107677448169928556907889250627834892_u128,);
RET = [(-6820_i16),9142_i16,(-21821_i16),8810_i16,24773_i16,(-3423_i16)];
_19 = (-27_i8);
_18.0 = !323041873606863914905198704762541844651_u128;
_7 = (-8625675054604334309_i64) << _4;
_24.1.fld3 = (_4,);
RET = [(-1466_i16),(-17531_i16),31272_i16,18126_i16,6108_i16,31279_i16];
_18.0 = 161972664050335528796766573602739844164_u128;
_24.0 = _5 << _24.1.fld3.0;
_22 = [(-27808_i16),(-31648_i16),12425_i16,(-24122_i16),5460_i16,(-21433_i16)];
_24.1.fld4 = 29381_i16;
_24.1.fld1 = _3 as usize;
_24.1.fld3.0 = _4;
_4 = _12 as usize;
_14 = _12;
_24.1.fld1 = !_24.1.fld3.0;
_18.0 = 48103786796576957792208589093016133162_u128;
_8 = 7469459759292104703_u64;
_6 = _24.1.fld0;
Goto(bb12)
}
bb16 = {
_3 = (-9223372036854775808_isize);
_8 = 7554593219056302465_u64;
_6 = _7 as i32;
_1 = true;
_5 = _1 as u32;
RET = [5409_i16,26694_i16,3799_i16,(-31530_i16),(-4449_i16),8207_i16];
RET = [(-2160_i16),(-2725_i16),31599_i16,(-24870_i16),(-6534_i16),30806_i16];
_7 = (-4069049339648670889_i64) << _6;
_12 = !_1;
_5 = 980462322_u32;
_5 = 2993638473_u32;
_8 = 10971355533579753336_u64;
_5 = !1914354729_u32;
_2 = '\u{a5783}';
RET = [15439_i16,7191_i16,27750_i16,4498_i16,(-578_i16),6409_i16];
_7 = 6511798026538465801_i64 << _4;
Call(_7 = fn1(RET, _1, _4, RET, RET, _2, _3, _8, _3, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb17 = {
_3 = (-9223372036854775808_isize);
_8 = 7554593219056302465_u64;
_6 = _7 as i32;
_1 = true;
_5 = _1 as u32;
RET = [5409_i16,26694_i16,3799_i16,(-31530_i16),(-4449_i16),8207_i16];
RET = [(-2160_i16),(-2725_i16),31599_i16,(-24870_i16),(-6534_i16),30806_i16];
_7 = (-4069049339648670889_i64) << _6;
_12 = !_1;
_5 = 980462322_u32;
_5 = 2993638473_u32;
_8 = 10971355533579753336_u64;
_5 = !1914354729_u32;
_2 = '\u{a5783}';
RET = [15439_i16,7191_i16,27750_i16,4498_i16,(-578_i16),6409_i16];
_7 = 6511798026538465801_i64 << _4;
Call(_7 = fn1(RET, _1, _4, RET, RET, _2, _3, _8, _3, _5), ReturnTo(bb3), UnwindUnreachable())
}
bb18 = {
_2 = '\u{9fb50}';
RET = [(-10520_i16),(-29137_i16),(-6759_i16),5071_i16,(-22130_i16),(-10445_i16)];
_1 = !false;
_6 = (-1922217679_i32);
_7 = _2 as i64;
_4 = 12398819138308814570_usize;
RET = [12512_i16,(-8809_i16),3410_i16,(-25827_i16),(-29575_i16),10082_i16];
_2 = '\u{103d3d}';
Goto(bb2)
}
bb19 = {
_13 = _2;
_8 = 9719867328771574212_u64;
_4 = _24.1.fld1 >> _21.0;
_21.0 = (-24587662401782933983857384325550325612_i128) - (-68934096671088325823748665404462813303_i128);
_24.3 = 225_u8 as u16;
_12 = _1;
_24.1.fld0 = _2 as i32;
match _8 {
0 => bb8,
1 => bb9,
9719867328771574212 => bb11,
_ => bb10
}
}
bb20 = {
(*_11).fld2 = [_3,_31.2.0.1,_31.2.0.1,_3,_3,_31.2.0.1];
(*_11).fld0 = _6;
(*_11).fld2 = [_3,_3,_31.2.0.1,_31.2.0.1,_25,_25];
_32 = [_3,_25,_25,_25];
RET = _22;
_10 = !_1;
_22 = [(*_11).fld4,(*_11).fld4,_24.1.fld4,(*_11).fld4,(*_11).fld4,(*_11).fld4];
_24.1.fld2 = [_31.2.0.1,_25,_3,_31.2.0.1,_31.2.0.1,_25];
_31.2.0.0 = -_27;
_31.1 = [_25,_25,_3,_3];
_30 = _26;
(*_11).fld3.0 = _4;
RET = _22;
_8 = 4479292459865007539_u64;
_12 = _10;
_24.1.fld4 = (-28399_i16) * 12892_i16;
_24.1.fld0 = _1 as i32;
_20 = _15 + _31.0.0;
_23 = _8 as u32;
_19 = 95_i8 * 74_i8;
(*_11).fld1 = !_24.1.fld3.0;
_18.0 = (*_11).fld0 as u128;
_24.1.fld3 = ((*_11).fld1,);
_33 = _26;
_35 = _24.1.fld4 as isize;
_3 = _35;
Goto(bb21)
}
bb21 = {
Call(_37 = dump_var(0_usize, 19_usize, Move(_19), 6_usize, Move(_6), 35_usize, Move(_35), 10_usize, Move(_10)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_37 = dump_var(0_usize, 26_usize, Move(_26), 8_usize, Move(_8), 7_usize, Move(_7), 14_usize, Move(_14)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_37 = dump_var(0_usize, 30_usize, Move(_30), 2_usize, Move(_2), 1_usize, Move(_1), 17_usize, Move(_17)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: [i16; 6],mut _2: bool,mut _3: usize,mut _4: [i16; 6],mut _5: [i16; 6],mut _6: char,mut _7: isize,mut _8: u64,mut _9: isize,mut _10: u32) -> i64 {
mir! {
type RET = i64;
let _11: u8;
let _12: i128;
let _13: ((&'static [i128; 5],),);
let _14: (&'static (i64, u128, &'static [i16; 6]), [u8; 3]);
let _15: &'static [i128; 5];
let _16: u16;
let _17: ((f32, isize), i32);
let _18: Adt78;
let _19: f64;
let _20: u32;
let _21: [i32; 3];
let _22: *mut [u32; 4];
let _23: isize;
let _24: char;
let _25: [char; 7];
let _26: bool;
let _27: char;
let _28: Adt69;
let _29: isize;
let _30: bool;
let _31: u16;
let _32: ();
let _33: ();
{
_1 = [(-15071_i16),2175_i16,(-15276_i16),(-4348_i16),(-14438_i16),(-20149_i16)];
_1 = [9391_i16,3267_i16,2784_i16,(-23865_i16),(-4309_i16),(-27554_i16)];
_2 = _7 != _9;
_10 = !3044878130_u32;
_4 = [(-32381_i16),(-15478_i16),27544_i16,2387_i16,(-18347_i16),5721_i16];
RET = 5400434819593497405_i64;
_2 = !true;
RET = (-8451802907448260118_i64) >> _3;
_5 = [19212_i16,(-17840_i16),9140_i16,(-11457_i16),(-12829_i16),21758_i16];
_7 = _2 as isize;
_9 = _7 | _7;
_5 = _4;
Goto(bb1)
}
bb1 = {
_1 = [21317_i16,(-12708_i16),3196_i16,(-8426_i16),29824_i16,26345_i16];
Call(_11 = fn2(_3, _9, _10, _4, _10, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_9 = _7;
_14.1 = [_11,_11,_11];
_16 = 49233_u16 >> RET;
_17.1 = RET as i32;
RET = (-4739597104191219537_i64);
RET = !(-8457366442195276866_i64);
_3 = 0_usize;
match _4[_3] {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768179075 => bb8,
_ => bb7
}
}
bb3 = {
_1 = [21317_i16,(-12708_i16),3196_i16,(-8426_i16),29824_i16,26345_i16];
Call(_11 = fn2(_3, _9, _10, _4, _10, _4), ReturnTo(bb2), UnwindUnreachable())
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
RET = (-2367020194422533260_i64) - (-5656365494330827296_i64);
_12 = _3 as i128;
RET = -(-4459864103130253060_i64);
_8 = _3 as u64;
_4[_3] = _5[_3] | _5[_3];
RET = !(-5590893532766291783_i64);
_12 = (-91852489368045736412749369046600324437_i128);
_17.1 = _5[_3] as i32;
_14.1 = [_11,_11,_11];
_2 = !true;
_17.0.1 = _8 as isize;
_5[_3] = _10 as i16;
_21 = [_17.1,_17.1,_17.1];
_5[_3] = RET as i16;
_23 = !_9;
_9 = _7;
_20 = _10;
_16 = 51161_u16;
_10 = _20;
_14.1[_3] = !_11;
_24 = _6;
_12 = -(-146032641500952390018781410876568558440_i128);
_21[_3] = _17.1 | _17.1;
_3 = 4_usize;
match _4[_3] {
0 => bb1,
1 => bb9,
340282366920938463463374607431768193109 => bb11,
_ => bb10
}
}
bb9 = {
Return()
}
bb10 = {
_1 = [21317_i16,(-12708_i16),3196_i16,(-8426_i16),29824_i16,26345_i16];
Call(_11 = fn2(_3, _9, _10, _4, _10, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_17.0.0 = _11 as f32;
_24 = _6;
_17.0.1 = _10 as isize;
_9 = !_23;
_19 = _9 as f64;
_7 = _23 * _23;
_25[_3] = _24;
_25 = [_6,_6,_6,_24,_6,_24,_24];
_1 = [_4[_3],_4[_3],_4[_3],_5[_3],_5[_3],_5[_3]];
_24 = _25[_3];
_1[_3] = -_4[_3];
_25 = [_6,_6,_24,_24,_6,_6,_24];
match _5[_3] {
0 => bb3,
1 => bb6,
2 => bb12,
340282366920938463463374607431768193109 => bb14,
_ => bb13
}
}
bb12 = {
_1 = [21317_i16,(-12708_i16),3196_i16,(-8426_i16),29824_i16,26345_i16];
Call(_11 = fn2(_3, _9, _10, _4, _10, _4), ReturnTo(bb2), UnwindUnreachable())
}
bb13 = {
_9 = _7;
_14.1 = [_11,_11,_11];
_16 = 49233_u16 >> RET;
_17.1 = RET as i32;
RET = (-4739597104191219537_i64);
RET = !(-8457366442195276866_i64);
_3 = 0_usize;
match _4[_3] {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
340282366920938463463374607431768179075 => bb8,
_ => bb7
}
}
bb14 = {
_17.1 = !354486000_i32;
_10 = !_20;
_1 = [_5[_3],_4[_3],_5[_3],_5[_3],_5[_3],_5[_3]];
_21 = [_17.1,_17.1,_17.1];
_7 = _17.0.1 << _5[_3];
_7 = _23;
RET = 104518318807670128_i64 * (-8915358611999062372_i64);
_18 = Adt78::Variant1 { fld0: _14.1 };
_5[_3] = _4[_3];
_25 = [_24,_6,_6,_24,_24,_24,_24];
_4[_3] = 76_i8 as i16;
_14.1 = [_11,_11,_11];
RET = (-394479230237271194_i64) + 349616716577768814_i64;
RET = _19 as i64;
_11 = 160_u8;
_4 = [_5[_3],_1[_3],_1[_3],_5[_3],_5[_3],_5[_3]];
_12 = (-44651407376684026372335956799222097054_i128);
_5 = [_1[_3],_4[_3],_4[_3],_1[_3],_1[_3],_1[_3]];
_29 = _9 * _9;
_30 = !_2;
_9 = _17.1 as isize;
_16 = _11 as u16;
_7 = _3 as isize;
_6 = _24;
Goto(bb15)
}
bb15 = {
Call(_32 = dump_var(1_usize, 3_usize, Move(_3), 21_usize, Move(_21), 9_usize, Move(_9), 20_usize, Move(_20)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_32 = dump_var(1_usize, 12_usize, Move(_12), 10_usize, Move(_10), 11_usize, Move(_11), 24_usize, Move(_24)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_32 = dump_var(1_usize, 6_usize, Move(_6), 16_usize, Move(_16), 33_usize, _33, 33_usize, _33), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: usize,mut _2: isize,mut _3: u32,mut _4: [i16; 6],mut _5: u32,mut _6: [i16; 6]) -> u8 {
mir! {
type RET = u8;
let _7: ([i16; 6], usize);
let _8: char;
let _9: [i32; 3];
let _10: Adt79;
let _11: bool;
let _12: ((f32, isize), *const [u32; 4], &'static (f32, isize));
let _13: char;
let _14: i128;
let _15: bool;
let _16: ((f64, &'static [i128; 5]), u128, (f64, &'static [i128; 5]));
let _17: (u16, u64, i128, f32);
let _18: [u32; 4];
let _19: *const [u32; 4];
let _20: u16;
let _21: u32;
let _22: u16;
let _23: char;
let _24: u32;
let _25: isize;
let _26: i32;
let _27: &'static u8;
let _28: i128;
let _29: f32;
let _30: u32;
let _31: [isize; 4];
let _32: Adt69;
let _33: isize;
let _34: isize;
let _35: i8;
let _36: [isize; 6];
let _37: (f32, isize);
let _38: &'static (f32, isize);
let _39: [i32; 3];
let _40: (f64, &'static [i128; 5]);
let _41: ((bool, i8, i16), f32, [u32; 4]);
let _42: isize;
let _43: char;
let _44: ((f32, isize), i32);
let _45: i8;
let _46: bool;
let _47: ();
let _48: ();
{
RET = 76_u8 << _3;
_7 = (_6, _1);
RET = 172_u8 + 158_u8;
Call(_7.1 = core::intrinsics::bswap(_1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = [(-9700_i16),27924_i16,2579_i16,15322_i16,(-21783_i16),15513_i16];
_3 = 6712530468442945835_i64 as u32;
_9 = [1018944637_i32,1170053673_i32,1679951166_i32];
_1 = 2891337437303343599_i64 as usize;
RET = 255_u8 + 1_u8;
_9 = [1225733053_i32,(-1945974256_i32),(-898543421_i32)];
_8 = '\u{5fe0a}';
_1 = _7.1 << RET;
_8 = '\u{78439}';
_7.1 = (-3470610953200331045246089311786422727_i128) as usize;
_4 = [12048_i16,5369_i16,(-19663_i16),(-27917_i16),(-7036_i16),(-28955_i16)];
_6 = _7.0;
_9 = [(-1555837704_i32),1551326204_i32,(-452959412_i32)];
_1 = _7.1 << RET;
_7 = (_4, _1);
_4 = _7.0;
_8 = '\u{93795}';
_2 = _1 as isize;
_11 = _5 < _3;
_5 = _2 as u32;
_12.0.0 = 7684941743466442054_i64 as f32;
_3 = _5;
Goto(bb2)
}
bb2 = {
_5 = _3 ^ _3;
_10 = Adt79::Variant1 { fld0: _12.0.0,fld1: _8 };
_7.1 = _1;
_15 = !_11;
RET = !229_u8;
_12.0.1 = -_2;
SetDiscriminant(_10, 0);
_12.0.0 = _2 as f32;
_7.1 = _1;
_16.2.0 = 178242988696544285309594781427948331308_u128 as f64;
_18 = [_5,_5,_5,_5];
_13 = _8;
_17 = (36856_u16, 14404339071190472813_u64, (-107513955768782156088600173409823433154_i128), _12.0.0);
_17 = (5041_u16, 13227385670216303684_u64, 164699300817933022098076794507372880830_i128, _12.0.0);
_14 = !_17.2;
place!(Field::<bool>(Variant(_10, 0), 0)) = _15;
_15 = !Field::<bool>(Variant(_10, 0), 0);
RET = (-8901209769301458038_i64) as u8;
_12.1 = core::ptr::addr_of!(_18);
Goto(bb3)
}
bb3 = {
_1 = _7.1 - _7.1;
_17.0 = 37459_u16 + 42465_u16;
_21 = 29292_i16 as u32;
Goto(bb4)
}
bb4 = {
_5 = (-3818785902482460122_i64) as u32;
_12.0.0 = _17.3 - _17.3;
_19 = Move(_12.1);
_9 = [1584859175_i32,(-679567160_i32),(-1285853505_i32)];
_2 = (-105_i8) as isize;
_3 = _5;
_22 = !_17.0;
_16.0.0 = _16.2.0;
_9 = [(-2079656908_i32),870074432_i32,(-349270146_i32)];
_3 = _8 as u32;
_17.0 = !_22;
_25 = _2 - _12.0.1;
_17.1 = (-18546011_i32) as u64;
RET = 47_u8;
_19 = core::ptr::addr_of!(_18);
_16.1 = !66883564127574330988801835034260631508_u128;
_12.2 = &_12.0;
_13 = _8;
_12.0.0 = -_17.3;
_20 = RET as u16;
_18 = [_5,_3,_5,_21];
match _17.2 {
164699300817933022098076794507372880830 => bb5,
_ => bb1
}
}
bb5 = {
_13 = _8;
_5 = RET as u32;
_7.1 = _17.1 as usize;
_12.1 = core::ptr::addr_of!((*_19));
_2 = _25 ^ _12.0.1;
RET = (-6035465826819419391_i64) as u8;
_16.0.0 = _17.1 as f64;
_23 = _13;
_17.3 = _16.1 as f32;
_12.0 = (_17.3, _25);
_29 = -_12.0.0;
_2 = _16.0.0 as isize;
Goto(bb6)
}
bb6 = {
_30 = _3;
_16.0.0 = _16.2.0;
_26 = 1364381120_i32;
_14 = _17.2 ^ _17.2;
RET = !135_u8;
_22 = _17.0 << _26;
_2 = _25;
_24 = !_3;
RET = 95_u8;
_22 = _16.1 as u16;
_4 = _7.0;
_28 = -_14;
_24 = Field::<bool>(Variant(_10, 0), 0) as u32;
_13 = _23;
_22 = _17.0;
_22 = _17.0 + _17.0;
_7.1 = !_1;
match _17.2 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb4,
4 => bb5,
164699300817933022098076794507372880830 => bb8,
_ => bb7
}
}
bb7 = {
_1 = _7.1 - _7.1;
_17.0 = 37459_u16 + 42465_u16;
_21 = 29292_i16 as u32;
Goto(bb4)
}
bb8 = {
(*_19) = [_24,_21,_24,_30];
_12.0 = (_29, _25);
_18 = [_24,_30,_5,_24];
_20 = _17.1 as u16;
_33 = (-16_i8) as isize;
Goto(bb9)
}
bb9 = {
_16.2.0 = _16.0.0 * _16.0.0;
_14 = _17.1 as i128;
match _26 {
1364381120 => bb10,
_ => bb4
}
}
bb10 = {
_14 = _17.2;
_35 = _8 as i8;
RET = !192_u8;
_31 = [_25,_2,_33,_25];
_12.0 = (_17.3, _2);
_17 = (_22, 419895826668983860_u64, _28, _29);
_15 = _11;
_37.0 = -_17.3;
_37.0 = -_17.3;
Call(_34 = fn3(), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
place!(Field::<bool>(Variant(_10, 0), 0)) = _17.0 > _20;
_15 = Field::<bool>(Variant(_10, 0), 0);
_39 = _9;
_4 = _6;
Call(_34 = fn4(_26, _17.1, Move(_19)), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_40.0 = _30 as f64;
_17.1 = 1547891942992563188_u64;
_20 = !_17.0;
_16.1 = 298476778000997560084513020807554356856_u128 & 74005381141459064454533228011797450621_u128;
_16.1 = 163504945530024415269947847242391620750_u128;
_12.2 = &_37;
_27 = &RET;
_44 = (_12.0, _26);
_41.2 = _18;
_41.0 = (_15, _35, (-19549_i16));
_38 = Move(_12.2);
_27 = &(*_27);
_36 = [_34,_34,_34,_12.0.1,_34,_34];
_35 = _41.0.1;
_18 = [_30,_5,_24,_24];
Call(_40.0 = fn5(_17.3, _36), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_16.0.0 = _16.2.0 * _16.2.0;
_26 = _21 as i32;
_37.1 = _34;
place!(Field::<bool>(Variant(_10, 0), 0)) = !_15;
_16.0.0 = _16.2.0;
_16.1 = 79327326339813386749104128785624609722_u128 + 99005415787922167903642477409798878715_u128;
_21 = _16.2.0 as u32;
_44.0 = (_37.0, _37.1);
_38 = &_44.0;
Call(_37 = fn12(Move(_38)), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
_41.0.1 = -_35;
_33 = _17.2 as isize;
_45 = _41.0.1 & _41.0.1;
_22 = _17.0;
_17.0 = _14 as u16;
_14 = _41.0.2 as i128;
_45 = _41.0.1;
Goto(bb15)
}
bb15 = {
Call(_47 = dump_var(2_usize, 3_usize, Move(_3), 24_usize, Move(_24), 30_usize, Move(_30), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_47 = dump_var(2_usize, 34_usize, Move(_34), 8_usize, Move(_8), 4_usize, Move(_4), 9_usize, Move(_9)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_47 = dump_var(2_usize, 11_usize, Move(_11), 1_usize, Move(_1), 7_usize, Move(_7), 21_usize, Move(_21)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_47 = dump_var(2_usize, 15_usize, Move(_15), 6_usize, Move(_6), 28_usize, Move(_28), 48_usize, _48), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn3() -> isize {
mir! {
type RET = isize;
let _1: f32;
let _2: ((f32, isize), *const [u32; 4], &'static (f32, isize));
let _3: (char, [i128; 5]);
let _4: char;
let _5: (i128,);
let _6: isize;
let _7: Adt78;
let _8: i64;
let _9: ((f32, isize), *const [u32; 4], &'static (f32, isize));
let _10: u32;
let _11: ();
let _12: ();
{
RET = -9223372036854775807_isize;
RET = -39_isize;
_1 = 21761_u16 as f32;
_1 = RET as f32;
_1 = 7156918928835534915_usize as f32;
_2.2 = &_2.0;
_2.0 = (_1, RET);
_3.0 = '\u{cc696}';
_4 = _3.0;
_5 = (74183807563455503450598586662453810000_i128,);
Call(_5.0 = core::intrinsics::bswap(92437874013448791433652206652390485103_i128), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_4 = _3.0;
_6 = -_2.0.1;
_3.1 = [_5.0,_5.0,_5.0,_5.0,_5.0];
_2.2 = &_2.0;
_6 = _2.0.1 + _2.0.1;
_2.0 = (_1, RET);
RET = _2.0.1;
_4 = _3.0;
_2.0.1 = _6 + _6;
_5 = ((-138495644537258272205811588692979582673_i128),);
_1 = _2.0.0;
RET = _2.0.1;
_2.0.1 = RET | RET;
_3.0 = _4;
_2.2 = &_2.0;
_1 = 207_u8 as f32;
_5.0 = 135057995000400690820929491238219270507_i128;
match _5.0 {
0 => bb2,
135057995000400690820929491238219270507 => bb4,
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
_2.0 = (_1, RET);
_2.0 = (_1, RET);
Goto(bb5)
}
bb5 = {
_3.1 = [_5.0,_5.0,_5.0,_5.0,_5.0];
_2.0.0 = _1 * _1;
_4 = _3.0;
_2.2 = &_2.0;
_4 = _3.0;
_8 = 1867126641225034195_i64 ^ (-6828224775537596328_i64);
_1 = _2.0.0;
_2.0 = (_1, _6);
_3.0 = _4;
_9.2 = &_2.0;
_3.1 = [_5.0,_5.0,_5.0,_5.0,_5.0];
_4 = _3.0;
_9.0.0 = _1 - _2.0.0;
_9.0.1 = -RET;
_2.2 = Move(_9.2);
_10 = 3086031642_u32;
Goto(bb6)
}
bb6 = {
_9.0.1 = !_6;
_5.0 = !112258393632201521586014219248206486456_i128;
RET = _2.0.1 + _6;
_3.1 = [_5.0,_5.0,_5.0,_5.0,_5.0];
RET = _2.0.1 + _6;
_6 = !RET;
_3.1 = [_5.0,_5.0,_5.0,_5.0,_5.0];
_3.0 = _4;
RET = _6 ^ _6;
_3.0 = _4;
_2.0.1 = 61645_u16 as isize;
RET = !_9.0.1;
_6 = _9.0.1 & _9.0.1;
_2.2 = &_2.0;
RET = false as isize;
RET = !_6;
_6 = _2.0.1;
_9.0 = (_1, _6);
_8 = (-7285630759870261606_i64) * (-245220090631566427_i64);
_2.0.1 = RET ^ RET;
_6 = _2.0.1 & _9.0.1;
_8 = -275390033900358150_i64;
_9.0.1 = -_2.0.1;
_9.0 = (_2.0.0, _6);
RET = _2.0.1 >> _9.0.1;
Goto(bb7)
}
bb7 = {
Call(_11 = dump_var(3_usize, 4_usize, Move(_4), 10_usize, Move(_10), 5_usize, Move(_5), 12_usize, _12), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn4(mut _1: i32,mut _2: u64,mut _3: *const [u32; 4]) -> isize {
mir! {
type RET = isize;
let _4: isize;
let _5: ();
let _6: ();
{
_4 = 47_isize << _2;
RET = -_4;
RET = _4 - _4;
Goto(bb1)
}
bb1 = {
Call(_5 = dump_var(4_usize, 4_usize, Move(_4), 6_usize, _6, 6_usize, _6, 6_usize, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn5(mut _1: f32,mut _2: [isize; 6]) -> f64 {
mir! {
type RET = f64;
let _3: usize;
let _4: &'static i16;
let _5: [i16; 6];
let _6: ((u16, u64, i128, f32),);
let _7: u32;
let _8: i16;
let _9: [u8; 6];
let _10: &'static &'static *const u64;
let _11: isize;
let _12: bool;
let _13: ((f32, isize), i32);
let _14: (&'static [i128; 5],);
let _15: (u8,);
let _16: i64;
let _17: ((f32, isize), i32);
let _18: Adt59;
let _19: i32;
let _20: ([u8; 6], usize, *mut (u16, u64, i128, f32));
let _21: (&'static [i16; 6], &'static (f32, isize), (u8,));
let _22: char;
let _23: i8;
let _24: u16;
let _25: &'static &'static &'static (f32, isize);
let _26: f32;
let _27: ((u16, u64, i128, f32),);
let _28: [u32; 4];
let _29: &'static [i16; 6];
let _30: ((f32, isize), i32);
let _31: ();
let _32: ();
{
RET = 13031801420920064487042553011308962488_i128 as f64;
_1 = 23_u8 as f32;
RET = 3_usize as f64;
RET = 45_u8 as f64;
_2 = [(-19_isize),(-9223372036854775808_isize),75_isize,(-9223372036854775808_isize),(-9223372036854775808_isize),(-125_isize)];
RET = 2151728235762960425_u64 as f64;
Goto(bb1)
}
bb1 = {
_3 = 7_usize;
_1 = 42760_u16 as f32;
_1 = 30914526055600712171008198140273647929_i128 as f32;
_3 = 4524485799006500016_usize >> (-31610866234268286791781668987140106048_i128);
_8 = (-26261_i16);
_6.0.1 = !1946540876794579741_u64;
_7 = 35_i8 as u32;
_7 = !2951018802_u32;
_4 = &_8;
RET = _6.0.1 as f64;
_6.0.3 = 1179848907_i32 as f32;
Call(_6.0.1 = fn6(Move(_4), _2, RET, _7, _2, _7, _2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_5 = [_8,_8,_8,_8,_8,_8];
_9 = [208_u8,215_u8,247_u8,82_u8,3_u8,56_u8];
_6.0.2 = (-78505926460316886870524671947689174726_i128) & (-49006331296987291480116413042507554990_i128);
_7 = 764144529_u32;
_8 = (-3005_i16);
_9 = [242_u8,112_u8,105_u8,88_u8,149_u8,98_u8];
_7 = !3619426446_u32;
_6.0.2 = (-164062341662509066670484091147717795394_i128);
_4 = &_8;
_7 = !413375108_u32;
_9 = [57_u8,192_u8,26_u8,134_u8,166_u8,188_u8];
_4 = &(*_4);
_6.0.0 = !26754_u16;
_6.0.0 = !53875_u16;
_4 = &_8;
_6.0.0 = !23283_u16;
_7 = !2094891800_u32;
_6.0.1 = 5209067461852819550_u64 & 16272491456827908669_u64;
_6.0.1 = 15452755392814997962_u64;
_5 = [_8,(*_4),_8,(*_4),(*_4),_8];
RET = 2214331056365064914_i64 as f64;
_5 = [(*_4),(*_4),(*_4),(*_4),_8,(*_4)];
_5 = [(*_4),(*_4),(*_4),(*_4),(*_4),(*_4)];
_4 = &(*_4);
_6.0.3 = RET as f32;
match (*_4) {
0 => bb1,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
340282366920938463463374607431768208451 => bb9,
_ => bb8
}
}
bb3 = {
_3 = 7_usize;
_1 = 42760_u16 as f32;
_1 = 30914526055600712171008198140273647929_i128 as f32;
_3 = 4524485799006500016_usize >> (-31610866234268286791781668987140106048_i128);
_8 = (-26261_i16);
_6.0.1 = !1946540876794579741_u64;
_7 = 35_i8 as u32;
_7 = !2951018802_u32;
_4 = &_8;
RET = _6.0.1 as f64;
_6.0.3 = 1179848907_i32 as f32;
Call(_6.0.1 = fn6(Move(_4), _2, RET, _7, _2, _7, _2), ReturnTo(bb2), UnwindUnreachable())
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
_11 = !9_isize;
_6.0.2 = (-79205344675506378730794654797827373340_i128) << (*_4);
_8 = 22325_i16 & (-24116_i16);
_12 = !true;
_13.1 = (-7908091635206079575_i64) as i32;
_13.0.1 = _11 - _11;
Call(_6.0.2 = core::intrinsics::bswap(136067911886006586651202198537234611030_i128), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
_13.0 = (_1, _11);
_2 = [_13.0.1,_11,_11,_13.0.1,_13.0.1,_13.0.1];
_4 = &_8;
RET = _6.0.1 as f64;
_9 = [53_u8,80_u8,115_u8,230_u8,35_u8,140_u8];
_13.0 = (_1, _11);
_4 = &_8;
_7 = 3630008301_u32 & 877787421_u32;
_13.1 = _13.0.1 as i32;
_7 = 413208846_u32 - 567647882_u32;
Goto(bb11)
}
bb11 = {
_17.0 = (_1, _13.0.1);
_17.1 = _13.1 << _13.0.1;
_9 = [107_u8,43_u8,148_u8,122_u8,222_u8,230_u8];
_6.0.3 = _13.0.0;
_16 = 6636633246478308545_i64 >> (*_4);
_13.0 = (_6.0.3, _17.0.1);
_2 = [_17.0.1,_11,_17.0.1,_17.0.1,_17.0.1,_17.0.1];
Goto(bb12)
}
bb12 = {
_4 = &_8;
_13.0 = (_6.0.3, _11);
_2 = [_11,_17.0.1,_11,_13.0.1,_11,_17.0.1];
_13.0 = (_17.0.0, _17.0.1);
_13.0 = _17.0;
_6.0 = (38095_u16, 12256512012254093293_u64, 23018329961396388579823699272111844093_i128, _17.0.0);
_13.0.0 = (*_4) as f32;
_9 = [197_u8,24_u8,34_u8,30_u8,0_u8,185_u8];
_17 = (_13.0, _13.1);
_9 = [115_u8,18_u8,114_u8,195_u8,121_u8,46_u8];
_7 = 3735013790_u32;
_19 = !_17.1;
_20.0 = [247_u8,64_u8,32_u8,42_u8,32_u8,120_u8];
_17 = _13;
_6.0.1 = 3825647424298122978_u64 - 8827115941318919850_u64;
_6.0.2 = 124317369412756582734791397160085066109_i128;
_11 = _13.0.1 * _13.0.1;
_17.0.0 = _11 as f32;
_1 = _13.0.0;
_6.0.3 = _3 as f32;
_20.0 = _9;
_20.1 = !_3;
_15 = (96_u8,);
_4 = &_8;
_5 = [(*_4),(*_4),_8,(*_4),(*_4),(*_4)];
_12 = true;
_6.0.2 = _20.1 as i128;
_11 = _15.0 as isize;
_6.0.2 = 109179205075077517262992836161938480247_i128;
Call(_13.0.1 = fn11(_17.0.1, _6.0), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_6.0.1 = 12287337835080072242_u64;
_20.1 = _3;
_11 = '\u{efb48}' as isize;
_13.1 = _19;
RET = _3 as f64;
_12 = _17.1 >= _19;
_8 = _6.0.3 as i16;
_13.1 = _7 as i32;
_9 = [_15.0,_15.0,_15.0,_15.0,_15.0,_15.0];
_6.0 = (10331_u16, 4675723591279978705_u64, (-160376555047944878176052527666484539037_i128), _13.0.0);
_6.0.0 = RET as u16;
_21.1 = &_17.0;
_17.0.0 = -_13.0.0;
_13.1 = _17.1;
_21.0 = &_5;
_21.0 = &_5;
_17.0.0 = _13.0.0 + _6.0.3;
_17 = (_13.0, _13.1);
_21.1 = &_13.0;
RET = _1 as f64;
_20.2 = core::ptr::addr_of_mut!(_6.0);
_5 = [_8,_8,_8,_8,_8,_8];
_21.2 = (_15.0,);
_17.0.1 = _13.0.1 - _13.0.1;
Goto(bb14)
}
bb14 = {
_13.1 = -_19;
_13.0 = (_1, _17.0.1);
_17.0.0 = _1 + _1;
_6.0.1 = (-39_i8) as u64;
_21.2 = _15;
_13.0.0 = _17.0.0 + _17.0.0;
_21.0 = &_5;
_6.0.1 = _1 as u64;
_18 = Adt59::Variant0 { fld0: _12,fld1: _20.0,fld2: _13.0.1,fld3: Move(_20.2),fld4: _6.0.2 };
_20 = (Field::<[u8; 6]>(Variant(_18, 0), 1), _3, Move(Field::<*mut (u16, u64, i128, f32)>(Variant(_18, 0), 3)));
_23 = 9_i8 + 103_i8;
_21.2.0 = _15.0 | _15.0;
_2 = [Field::<isize>(Variant(_18, 0), 2),_13.0.1,Field::<isize>(Variant(_18, 0), 2),_17.0.1,_13.0.1,_17.0.1];
_1 = _13.0.0;
_6.0.1 = !10132195950666421937_u64;
_22 = '\u{8ca16}';
_8 = -(-7604_i16);
Goto(bb15)
}
bb15 = {
Call(_31 = dump_var(5_usize, 15_usize, Move(_15), 8_usize, Move(_8), 3_usize, Move(_3), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_31 = dump_var(5_usize, 12_usize, Move(_12), 23_usize, Move(_23), 32_usize, _32, 32_usize, _32), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn6(mut _1: &'static i16,mut _2: [isize; 6],mut _3: f64,mut _4: u32,mut _5: [isize; 6],mut _6: u32,mut _7: [isize; 6]) -> u64 {
mir! {
type RET = u64;
let _8: isize;
let _9: [isize; 4];
let _10: isize;
let _11: *const (f32, isize);
let _12: i128;
let _13: (f32, isize);
let _14: *const Adt57;
let _15: Adt73;
let _16: Adt57;
let _17: isize;
let _18: Adt75;
let _19: Adt78;
let _20: ((f64, &'static [i128; 5]), u128, (f64, &'static [i128; 5]));
let _21: ();
let _22: ();
{
RET = 5827_i16 as u64;
_8 = (-50_isize);
RET = 6299966486355341616_u64;
RET = 17876841350391485203_u64;
_3 = 171_u8 as f64;
_10 = -_8;
_2 = _5;
_9 = [_10,_10,_10,_10];
_10 = !_8;
_5 = _2;
_4 = _6;
_7 = [_10,_8,_10,_10,_8,_10];
_8 = _10 * _10;
RET = true as u64;
_10 = _4 as isize;
_2 = [_8,_10,_8,_8,_10,_8];
_12 = (-41581407534297375199706487080487745256_i128) - (-142711213568631577172208538696609958551_i128);
_11 = core::ptr::addr_of!(_13);
Goto(bb1)
}
bb1 = {
(*_11).0 = _3 as f32;
_9 = [_8,_8,_8,_10];
_13.1 = RET as isize;
_13.1 = _8;
_5 = [(*_11).1,_13.1,(*_11).1,_8,_10,_8];
_9 = [_13.1,_8,_13.1,_13.1];
(*_11).1 = -_10;
_4 = _6;
_13.0 = 7809714453703990462_usize as f32;
_15.fld3.0 = '\u{98089}';
_13.1 = _8 & _10;
_15.fld3.1 = [_12,_12,_12,_12,_12];
_15.fld2 = (-395685584315826038_i64) as i32;
(*_11).0 = 4118_i16 as f32;
RET = !12867456543129382208_u64;
Call(_15.fld1 = fn7(_9, _13.1, Move(_11), (*_11), _5, _15.fld3.1, (*_11).1, (*_11).1, _5, (*_11), (*_11)), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_1 = &_16.fld4;
_16.fld1 = !4004876497561673765_usize;
RET = _15.fld1;
_18.fld0.1 = _10 | _13.1;
_4 = 1145_u16 as u32;
_17 = true as isize;
_16.fld4 = (-7145_i16);
_14 = core::ptr::addr_of!(_16);
_15.fld4 = core::ptr::addr_of!((*_14));
_12 = 23401354396553801177494667445152045509_i128;
_16.fld2 = [_18.fld0.1,_8,_13.1,_13.1,_13.1,_13.1];
_1 = &(*_14).fld4;
(*_14).fld3 = (_16.fld1,);
_16.fld3.0 = (*_14).fld1;
RET = _15.fld1;
_6 = 2993930377300632722_i64 as u32;
Goto(bb3)
}
bb3 = {
Call(_21 = dump_var(6_usize, 17_usize, Move(_17), 6_usize, Move(_6), 7_usize, Move(_7), 8_usize, Move(_8)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_21 = dump_var(6_usize, 12_usize, Move(_12), 22_usize, _22, 22_usize, _22, 22_usize, _22), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn7(mut _1: [isize; 4],mut _2: isize,mut _3: *const (f32, isize),mut _4: (f32, isize),mut _5: [isize; 6],mut _6: [i128; 5],mut _7: isize,mut _8: isize,mut _9: [isize; 6],mut _10: (f32, isize),mut _11: (f32, isize)) -> u64 {
mir! {
type RET = u64;
let _12: &'static &'static (f32, isize);
let _13: (u128,);
let _14: i64;
let _15: bool;
let _16: i32;
let _17: char;
let _18: *const [u8; 3];
let _19: (&'static [i16; 6], &'static (f32, isize), (u8,));
let _20: *const [u32; 4];
let _21: ();
let _22: ();
{
_7 = -_8;
_11.1 = -_2;
_7 = _8;
_10 = (_4.0, _2);
_11.0 = -_4.0;
_10.0 = _4.0;
_11.0 = 166703194128357312921863002298872286338_u128 as f32;
RET = 2599762927766727532_u64;
_10 = _4;
_10.0 = 6_usize as f32;
_4 = _10;
_9 = _5;
_7 = _2 & _11.1;
_10.1 = -_8;
_3 = core::ptr::addr_of!(_4);
(*_3).1 = _10.1;
_4 = _11;
(*_3).1 = _7 & _11.1;
(*_3).0 = _10.0;
RET = 1554943558677873057_usize as u64;
_9 = [_4.1,_7,_4.1,_2,_4.1,_7];
(*_3).1 = _7 * _8;
_4.1 = _7 & _8;
_4.1 = _2 - _7;
(*_3).0 = RET as f32;
Call((*_3).0 = fn8((*_3).1, _9, _6, _4.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_3).0 = -_10.0;
_8 = _7;
(*_3).1 = _8;
_5 = [(*_3).1,_10.1,_7,_11.1,(*_3).1,(*_3).1];
_7 = 8643_i16 as isize;
_5 = _9;
_7 = !_10.1;
RET = !13784583581239624959_u64;
_6 = [30647826152461228773687731737709132553_i128,63429665537392544881573121229074993542_i128,(-2494030771931392184961629832519538892_i128),25627927319595251738335578147881447580_i128,89531650118126065517835515989076796959_i128];
_10 = (_4.0, (*_3).1);
RET = 11585541083447656845_u64 | 3764405123360714803_u64;
_10 = (*_3);
(*_3) = (_11.0, _8);
_13.0 = (-599896730_i32) as u128;
_9 = [(*_3).1,(*_3).1,_4.1,_8,_10.1,_10.1];
_9 = [_11.1,(*_3).1,_8,_2,_7,_8];
_2 = -(*_3).1;
_11.0 = 147_u8 as f32;
_5 = [_10.1,(*_3).1,_2,(*_3).1,(*_3).1,_7];
(*_3).1 = (-43929004525890379804928610346589374250_i128) as isize;
(*_3).1 = !_11.1;
Goto(bb2)
}
bb2 = {
_4 = (_11.0, _2);
(*_3) = (_11.0, _8);
_10.1 = -(*_3).1;
_7 = _4.1;
_11.1 = !(*_3).1;
RET = 12878056045629558339_u64 + 5685212593504091788_u64;
_14 = (-2642162697279748353_i64) | (-7745404929110369048_i64);
_8 = 105542692317711442787240845310059705247_i128 as isize;
(*_3) = (_10.0, _8);
_5 = _9;
Call((*_3).1 = fn10(_11.0, _11, _2, _7, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = 11042819129668250628_u64;
_8 = _10.1;
_9 = [(*_3).1,_11.1,(*_3).1,(*_3).1,_2,_8];
_11 = (_10.0, (*_3).1);
_10.1 = _4.1 * (*_3).1;
(*_3).0 = -_10.0;
_4.1 = -_10.1;
_10.0 = _11.0 * _11.0;
_13 = (111644975822402077469422735333442664522_u128,);
(*_3).1 = _11.1;
_1 = [_4.1,_10.1,_2,(*_3).1];
RET = 3401082000785786011_u64;
(*_3).0 = _10.0 - _10.0;
RET = 5949891947553781370_u64 + 9213620815245069508_u64;
(*_3).0 = 19_u8 as f32;
_3 = core::ptr::addr_of!((*_3));
_4.1 = !_10.1;
_4.1 = !_10.1;
_14 = 3414996651800355331_i64 - 5398804179365837437_i64;
_10.0 = _4.0;
_17 = '\u{7e41a}';
_9 = [(*_3).1,_4.1,_7,_10.1,_4.1,_4.1];
match _13.0 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
111644975822402077469422735333442664522 => bb7,
_ => bb6
}
}
bb4 = {
_4 = (_11.0, _2);
(*_3) = (_11.0, _8);
_10.1 = -(*_3).1;
_7 = _4.1;
_11.1 = !(*_3).1;
RET = 12878056045629558339_u64 + 5685212593504091788_u64;
_14 = (-2642162697279748353_i64) | (-7745404929110369048_i64);
_8 = 105542692317711442787240845310059705247_i128 as isize;
(*_3) = (_10.0, _8);
_5 = _9;
Call((*_3).1 = fn10(_11.0, _11, _2, _7, _9), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
(*_3).0 = -_10.0;
_8 = _7;
(*_3).1 = _8;
_5 = [(*_3).1,_10.1,_7,_11.1,(*_3).1,(*_3).1];
_7 = 8643_i16 as isize;
_5 = _9;
_7 = !_10.1;
RET = !13784583581239624959_u64;
_6 = [30647826152461228773687731737709132553_i128,63429665537392544881573121229074993542_i128,(-2494030771931392184961629832519538892_i128),25627927319595251738335578147881447580_i128,89531650118126065517835515989076796959_i128];
_10 = (_4.0, (*_3).1);
RET = 11585541083447656845_u64 | 3764405123360714803_u64;
_10 = (*_3);
(*_3) = (_11.0, _8);
_13.0 = (-599896730_i32) as u128;
_9 = [(*_3).1,(*_3).1,_4.1,_8,_10.1,_10.1];
_9 = [_11.1,(*_3).1,_8,_2,_7,_8];
_2 = -(*_3).1;
_11.0 = 147_u8 as f32;
_5 = [_10.1,(*_3).1,_2,(*_3).1,(*_3).1,_7];
(*_3).1 = (-43929004525890379804928610346589374250_i128) as isize;
(*_3).1 = !_11.1;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
_16 = _4.0 as i32;
_17 = '\u{81e8c}';
_4 = _11;
_7 = _10.1 * (*_3).1;
(*_3).1 = _11.1;
_8 = (*_3).1 * (*_3).1;
_15 = !false;
_4.0 = _10.0;
_10 = (_4.0, _4.1);
(*_3).1 = -_8;
RET = _7 as u64;
_14 = -3036109292642095313_i64;
_14 = 3798024244825200630_i64;
(*_3).1 = _7 | _7;
_2 = _17 as isize;
(*_3).1 = _11.1;
(*_3).0 = -_10.0;
_10.1 = !_7;
_5 = _9;
(*_3).0 = _11.0 - _10.0;
_13 = (160144329446155815119031723228635002191_u128,);
_1 = [_10.1,_4.1,_8,_7];
_14 = -2741030918519145339_i64;
_6 = [100907723081865215864539876305643525816_i128,(-2501261158187750179551957466877746711_i128),(-126649008503760339338787917213832654897_i128),(-152405580175775669417413853242243705327_i128),145710078546987823968567272980003149993_i128];
(*_3).1 = _16 as isize;
Goto(bb8)
}
bb8 = {
Call(_21 = dump_var(7_usize, 7_usize, Move(_7), 16_usize, Move(_16), 6_usize, Move(_6), 5_usize, Move(_5)), ReturnTo(bb9), UnwindUnreachable())
}
bb9 = {
Call(_21 = dump_var(7_usize, 9_usize, Move(_9), 1_usize, Move(_1), 22_usize, _22, 22_usize, _22), ReturnTo(bb10), UnwindUnreachable())
}
bb10 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: isize,mut _2: [isize; 6],mut _3: [i128; 5],mut _4: isize) -> f32 {
mir! {
type RET = f32;
let _5: bool;
let _6: *mut [u32; 4];
let _7: (char, [i128; 5]);
let _8: i32;
let _9: u128;
let _10: (&'static (i64, u128, &'static [i16; 6]), [u8; 3]);
let _11: i64;
let _12: i8;
let _13: &'static (f32, isize);
let _14: ([i16; 6], usize);
let _15: ([i16; 6], usize);
let _16: i32;
let _17: (i128,);
let _18: (u128,);
let _19: f32;
let _20: (&'static *const u64, *mut (u16, u64, i128, f32), [isize; 6]);
let _21: [u64; 2];
let _22: &'static usize;
let _23: bool;
let _24: &'static Adt26;
let _25: i64;
let _26: char;
let _27: *mut (u16, u64, i128, f32);
let _28: *const u64;
let _29: bool;
let _30: u128;
let _31: (&'static [i128; 5],);
let _32: Adt78;
let _33: &'static usize;
let _34: isize;
let _35: ();
let _36: ();
{
RET = 8487_u16 as f32;
RET = 8456767705931816900_u64 as f32;
_3 = [(-2828738023612528122097759541159257197_i128),(-9012401757123846646175603137177160910_i128),49062937591552752954644126168773140621_i128,70154371712795437140761595100049239355_i128,25484859448959010443688782924351673089_i128];
_1 = !_4;
_1 = _4 * _4;
_5 = !false;
RET = (-7942_i16) as f32;
RET = 7_usize as f32;
_1 = 6737535550193831566_i64 as isize;
_3 = [102790138666608344991796520028058818570_i128,(-156797657581265854165904965059391227478_i128),117706048639146676163445541542746433023_i128,(-6826006236874186925246747814865821629_i128),102054647305056091236148200325389130518_i128];
RET = 6295035303853323303823313658509136211_i128 as f32;
_7 = ('\u{d541a}', _3);
RET = 44129_u16 as f32;
_3 = _7.1;
_7.1 = [(-13100025167957264000858395123023226943_i128),117287125565606855304495377791683939181_i128,(-153302708077556137893752784622613508321_i128),46548088829479470315330254547866966829_i128,59115391610563520336614670118250626552_i128];
_7.0 = '\u{acaa7}';
_5 = false ^ false;
_7.1 = [159403256562033718310161765888865672655_i128,(-34096282974125469731320563334868566778_i128),15378645973859550297531370067642536276_i128,125674802853534710790537849338713369717_i128,82669217469655903670075113270130053806_i128];
_3 = [(-54671003518504515406917883398719329080_i128),(-88808300682706801599306586794849917573_i128),135117565880180372649579270845823955364_i128,9508298667212062071518180009348369099_i128,150524929333219969366003672857956300958_i128];
_4 = 61_u8 as isize;
_2 = [_4,_1,_1,_1,_4,_1];
_3 = _7.1;
_7.0 = '\u{24fb9}';
_5 = true;
RET = 171_u8 as f32;
_7.0 = '\u{afb7f}';
_2 = [_1,_4,_1,_4,_4,_4];
RET = 254_u8 as f32;
Goto(bb1)
}
bb1 = {
_1 = _4 * _4;
_4 = !_1;
_3 = [92496924486162001583861636289661510810_i128,64616862326278982277043492351654112363_i128,4729449411574999288978182914655479017_i128,28953667700736731726692145193075050277_i128,(-129484546486810910348859640053918585979_i128)];
_2 = [_4,_1,_4,_4,_4,_1];
_1 = _4 + _4;
_7.1 = _3;
_3 = [(-27980933204249323221016427409215117224_i128),169174331221919692637815411058696133079_i128,138154089148485156049190946649960406448_i128,(-119521646830912128021449564147922600217_i128),(-34999074987844480255836060768649547750_i128)];
Goto(bb2)
}
bb2 = {
_10.1 = [250_u8,126_u8,141_u8];
_2 = [_1,_1,_1,_1,_1,_4];
_2 = [_4,_4,_1,_1,_1,_1];
_1 = _4;
_4 = _1 >> _1;
_10.1 = [164_u8,146_u8,127_u8];
RET = _1 as f32;
RET = 415935998_i32 as f32;
RET = _4 as f32;
_8 = -(-1553446484_i32);
_3 = _7.1;
Call(_7.0 = fn9(_7.1, _5, _2, _1, _2, _2, _7.1, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
_7 = ('\u{102d3a}', _3);
RET = 323442579071072164056384274768845665624_u128 as f32;
_11 = !6844379414446595569_i64;
_12 = 9_i8 - (-97_i8);
_9 = 4567939_u32 as u128;
_11 = 8235779753889018735_i64 << _12;
_7 = ('\u{5545a}', _3);
_7.1 = [(-133121149470377938424753269568220701575_i128),79355900229359707086664224657616146784_i128,(-66755662394296637137010991176216798647_i128),51846395316565488134025013286965128107_i128,155863404166328361359097730353230819964_i128];
_12 = 36108_u16 as i8;
_3 = _7.1;
_7.0 = '\u{d9027}';
_9 = 146151855256246338375763169264792878705_u128;
_3 = [(-52474362077105924836690263899089278965_i128),74319130569765225783101459425293696861_i128,(-122728589371965424068324486231618963050_i128),42088761388006720404598723441405165533_i128,(-128691148950586847549024836579521538687_i128)];
_7 = ('\u{57bac}', _3);
_3 = [(-22975078320094723559208418218729700193_i128),(-66694508484816060411255348695953691657_i128),(-139309926156666166801060641438232900673_i128),(-3725319539057568293861988913890077396_i128),(-103528830534350763324659733207560441424_i128)];
RET = 15069_i16 as f32;
RET = 21_u8 as f32;
_3 = _7.1;
match _9 {
0 => bb2,
146151855256246338375763169264792878705 => bb5,
_ => bb4
}
}
bb4 = {
_10.1 = [250_u8,126_u8,141_u8];
_2 = [_1,_1,_1,_1,_1,_4];
_2 = [_4,_4,_1,_1,_1,_1];
_1 = _4;
_4 = _1 >> _1;
_10.1 = [164_u8,146_u8,127_u8];
RET = _1 as f32;
RET = 415935998_i32 as f32;
RET = _4 as f32;
_8 = -(-1553446484_i32);
_3 = _7.1;
Call(_7.0 = fn9(_7.1, _5, _2, _1, _2, _2, _7.1, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb5 = {
_2 = [_4,_4,_1,_4,_1,_1];
_2 = [_1,_4,_4,_1,_4,_1];
_11 = 16292279713698576662_u64 as i64;
_8 = 1060637318_i32 | (-1033533748_i32);
_8 = (-621999672_i32) | (-293948078_i32);
_14.0 = [8168_i16,8342_i16,2587_i16,(-14163_i16),9491_i16,8392_i16];
_7.0 = '\u{2d59a}';
_12 = _9 as i8;
_7.1 = [(-101567469722585713892389357797305837549_i128),143962032400222780855070087073549349403_i128,(-128852959959014756569931408899199607523_i128),(-33951373156884121902914268888699559835_i128),(-84138900299198080471904614372874808214_i128)];
_15 = (_14.0, 6541347190756130758_usize);
_7.1 = [(-153813157486589485483066348552958748339_i128),(-97466240508382878218075162380804448926_i128),(-6166971275901466349623201181780996307_i128),(-26651007282974701492328292146703346438_i128),(-49341832765071497806057717297797247385_i128)];
_3 = [78303523921069631483811565662219689762_i128,(-129248235489810602578216256314151714634_i128),148280319689073859330484132592992263297_i128,(-143012081604330473942831219410996334465_i128),(-130117349312973097341803010852262642422_i128)];
Goto(bb6)
}
bb6 = {
_14 = (_15.0, _15.1);
_14 = (_15.0, _15.1);
_15.0 = _14.0;
_7.1 = [90814490333839912764384569444601817899_i128,21932876231917245033750097395312952887_i128,64121833374522480853241771491427077960_i128,24466500558200050780589610859641293893_i128,100553785967060063676483595244516309983_i128];
_4 = _1 ^ _1;
_12 = _11 as i8;
_1 = -_4;
RET = _9 as f32;
_10.1 = [213_u8,238_u8,145_u8];
_7 = ('\u{879a0}', _3);
_18.0 = !_9;
_15 = (_14.0, _14.1);
match _14.1 {
0 => bb5,
1 => bb2,
2 => bb7,
3 => bb8,
4 => bb9,
6541347190756130758 => bb11,
_ => bb10
}
}
bb7 = {
_1 = _4 * _4;
_4 = !_1;
_3 = [92496924486162001583861636289661510810_i128,64616862326278982277043492351654112363_i128,4729449411574999288978182914655479017_i128,28953667700736731726692145193075050277_i128,(-129484546486810910348859640053918585979_i128)];
_2 = [_4,_1,_4,_4,_4,_1];
_1 = _4 + _4;
_7.1 = _3;
_3 = [(-27980933204249323221016427409215117224_i128),169174331221919692637815411058696133079_i128,138154089148485156049190946649960406448_i128,(-119521646830912128021449564147922600217_i128),(-34999074987844480255836060768649547750_i128)];
Goto(bb2)
}
bb8 = {
_10.1 = [250_u8,126_u8,141_u8];
_2 = [_1,_1,_1,_1,_1,_4];
_2 = [_4,_4,_1,_1,_1,_1];
_1 = _4;
_4 = _1 >> _1;
_10.1 = [164_u8,146_u8,127_u8];
RET = _1 as f32;
RET = 415935998_i32 as f32;
RET = _4 as f32;
_8 = -(-1553446484_i32);
_3 = _7.1;
Call(_7.0 = fn9(_7.1, _5, _2, _1, _2, _2, _7.1, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb9 = {
_7 = ('\u{102d3a}', _3);
RET = 323442579071072164056384274768845665624_u128 as f32;
_11 = !6844379414446595569_i64;
_12 = 9_i8 - (-97_i8);
_9 = 4567939_u32 as u128;
_11 = 8235779753889018735_i64 << _12;
_7 = ('\u{5545a}', _3);
_7.1 = [(-133121149470377938424753269568220701575_i128),79355900229359707086664224657616146784_i128,(-66755662394296637137010991176216798647_i128),51846395316565488134025013286965128107_i128,155863404166328361359097730353230819964_i128];
_12 = 36108_u16 as i8;
_3 = _7.1;
_7.0 = '\u{d9027}';
_9 = 146151855256246338375763169264792878705_u128;
_3 = [(-52474362077105924836690263899089278965_i128),74319130569765225783101459425293696861_i128,(-122728589371965424068324486231618963050_i128),42088761388006720404598723441405165533_i128,(-128691148950586847549024836579521538687_i128)];
_7 = ('\u{57bac}', _3);
_3 = [(-22975078320094723559208418218729700193_i128),(-66694508484816060411255348695953691657_i128),(-139309926156666166801060641438232900673_i128),(-3725319539057568293861988913890077396_i128),(-103528830534350763324659733207560441424_i128)];
RET = 15069_i16 as f32;
RET = 21_u8 as f32;
_3 = _7.1;
match _9 {
0 => bb2,
146151855256246338375763169264792878705 => bb5,
_ => bb4
}
}
bb10 = {
_10.1 = [250_u8,126_u8,141_u8];
_2 = [_1,_1,_1,_1,_1,_4];
_2 = [_4,_4,_1,_1,_1,_1];
_1 = _4;
_4 = _1 >> _1;
_10.1 = [164_u8,146_u8,127_u8];
RET = _1 as f32;
RET = 415935998_i32 as f32;
RET = _4 as f32;
_8 = -(-1553446484_i32);
_3 = _7.1;
Call(_7.0 = fn9(_7.1, _5, _2, _1, _2, _2, _7.1, RET), ReturnTo(bb3), UnwindUnreachable())
}
bb11 = {
_15.0 = [(-11121_i16),(-19363_i16),(-32322_i16),30009_i16,(-13436_i16),(-9745_i16)];
_17 = ((-45332095696686826571152885608529276698_i128),);
_1 = _4 ^ _4;
_14.1 = _15.1 & _15.1;
_11 = _12 as i64;
_19 = -RET;
_8 = (-673686172_i32);
_12 = (-109_i8) << _15.1;
_3 = [_17.0,_17.0,_17.0,_17.0,_17.0];
_1 = _4;
_7.1 = _3;
Goto(bb12)
}
bb12 = {
_15 = _14;
_4 = 125_u8 as isize;
_5 = !false;
_4 = !_1;
_10.1 = [148_u8,177_u8,130_u8];
_14.0 = [19463_i16,(-15331_i16),23607_i16,20114_i16,(-11077_i16),(-22965_i16)];
_7 = ('\u{9f7bc}', _3);
_7.1 = [_17.0,_17.0,_17.0,_17.0,_17.0];
Goto(bb13)
}
bb13 = {
_16 = _8 * _8;
_16 = 2192271751_u32 as i32;
RET = _19 - _19;
RET = _19;
_1 = _4;
_7.0 = '\u{e81c0}';
_26 = _7.0;
_20.0 = &_28;
_5 = false;
_19 = -RET;
_12 = (-60_i8);
RET = 203_u8 as f32;
_20.0 = &_28;
_19 = -RET;
_1 = _4;
_8 = !_16;
_21 = [14968457190165875327_u64,1906876376246958442_u64];
_18.0 = _9 * _9;
_4 = _1;
_10.1 = [96_u8,163_u8,241_u8];
_26 = _7.0;
match _9 {
0 => bb1,
1 => bb7,
146151855256246338375763169264792878705 => bb14,
_ => bb6
}
}
bb14 = {
_7.0 = _26;
_29 = !_5;
_14.0 = [25707_i16,(-17652_i16),2347_i16,19245_i16,30911_i16,(-4886_i16)];
RET = _19;
_4 = _1;
_16 = _1 as i32;
_11 = 6642988098677035241_i64 >> _14.1;
_8 = _19 as i32;
_18 = (_9,);
_14.1 = RET as usize;
_14.0 = [(-26192_i16),(-7282_i16),22749_i16,30103_i16,19652_i16,(-22189_i16)];
_18.0 = _9 << _12;
_26 = _7.0;
RET = -_19;
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(8_usize, 5_usize, Move(_5), 8_usize, Move(_8), 15_usize, Move(_15), 4_usize, Move(_4)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(8_usize, 12_usize, Move(_12), 29_usize, Move(_29), 1_usize, Move(_1), 16_usize, Move(_16)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(8_usize, 17_usize, Move(_17), 36_usize, _36, 36_usize, _36, 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn9(mut _1: [i128; 5],mut _2: bool,mut _3: [isize; 6],mut _4: isize,mut _5: [isize; 6],mut _6: [isize; 6],mut _7: [i128; 5],mut _8: f32) -> char {
mir! {
type RET = char;
let _9: [i32; 3];
let _10: bool;
let _11: char;
let _12: u8;
let _13: usize;
let _14: bool;
let _15: (u16, u64, i128, f32);
let _16: (&'static *const u64, *mut (u16, u64, i128, f32), [isize; 6]);
let _17: (&'static *const u64, *mut (u16, u64, i128, f32), [isize; 6]);
let _18: u32;
let _19: &'static [i16; 6];
let _20: ();
let _21: ();
{
_2 = true;
RET = '\u{3575f}';
_8 = (-1016975784_i32) as f32;
_8 = (-32_i8) as f32;
_7 = [(-4925053882110050569833544052117299783_i128),65313817731624495848287634862291209414_i128,(-119913985387967783306744892508322719786_i128),136512428911291166617351710368375378900_i128,(-159482143661039813603273184436822905271_i128)];
_8 = (-96_i8) as f32;
_4 = 9223372036854775807_isize;
_11 = RET;
RET = _11;
RET = _11;
_7 = [(-86345592143285202991317719449224157714_i128),11143458139321229745092838555162253460_i128,(-159009127579629110846322598385828202832_i128),164542078293312664216491732316065966727_i128,48914620160334610376315029094887542019_i128];
_10 = !_2;
_12 = 46_u8;
_12 = 27_u8;
_4 = (-37_isize);
_9 = [976114673_i32,1062832754_i32,(-733654143_i32)];
_3 = _6;
RET = _11;
_10 = !_2;
_2 = !_10;
_8 = (-88814562735008742965784369917821425746_i128) as f32;
_1 = [127894795901200522602669941955695488850_i128,(-19899769199462681925164456510698710434_i128),157924347260367741941259122344507049719_i128,(-42076147410701278097987907848844179663_i128),(-131267813310086800359566172614449812179_i128)];
RET = _11;
_3 = [_4,_4,_4,_4,_4,_4];
_2 = !_10;
_4 = 104533744306032277346784108668658750421_i128 as isize;
_4 = 69_isize;
match _12 {
0 => bb1,
1 => bb2,
2 => bb3,
27 => bb5,
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
_10 = !_2;
RET = _11;
_8 = (-2756519317548251803_i64) as f32;
_12 = !147_u8;
_9 = [1145204401_i32,(-1174495136_i32),(-2037259148_i32)];
RET = _11;
_1 = [56223918686911323516945171873363772386_i128,112686818572876534553720975227010666687_i128,100351056878784247662806701305758539045_i128,45124392187358849691668557239988909716_i128,(-156771717735402544490519238311286175540_i128)];
_5 = [_4,_4,_4,_4,_4,_4];
_6 = [_4,_4,_4,_4,_4,_4];
_2 = !_10;
_3 = _6;
_1 = [(-59736216540177127476117943118929774844_i128),(-121192217622857804184962928091931670184_i128),(-69232797677401067595760424589139374576_i128),(-49558574082064230433939739414463694127_i128),(-79278566824865077060840772659607264358_i128)];
match _4 {
0 => bb4,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
69 => bb11,
_ => bb10
}
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
match _4 {
0 => bb5,
69 => bb12,
_ => bb10
}
}
bb12 = {
_9 = [1197819921_i32,1013787065_i32,(-2005231099_i32)];
_12 = 131905138720158943741559645726676236968_u128 as u8;
_1 = [(-148546255587525521017626247748692807203_i128),(-62628337893669949159069651527552838444_i128),73488958868964163367894228069702233985_i128,83909065342859358068338336487002754883_i128,(-35848449431362221514527270689441353056_i128)];
RET = _11;
_6 = [_4,_4,_4,_4,_4,_4];
RET = _11;
_11 = RET;
_15.0 = 16056_u16;
_13 = !7246792336548467569_usize;
_13 = !1_usize;
_15 = (37511_u16, 11819920567296277555_u64, 115087341536350510386523215396915828736_i128, _8);
_5 = [_4,_4,_4,_4,_4,_4];
_17.2 = [_4,_4,_4,_4,_4,_4];
_14 = _15.1 > _15.1;
_15.0 = (-675023576_i32) as u16;
RET = _11;
match _15.2 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
115087341536350510386523215396915828736 => bb19,
_ => bb18
}
}
bb13 = {
Return()
}
bb14 = {
_10 = !_2;
RET = _11;
_8 = (-2756519317548251803_i64) as f32;
_12 = !147_u8;
_9 = [1145204401_i32,(-1174495136_i32),(-2037259148_i32)];
RET = _11;
_1 = [56223918686911323516945171873363772386_i128,112686818572876534553720975227010666687_i128,100351056878784247662806701305758539045_i128,45124392187358849691668557239988909716_i128,(-156771717735402544490519238311286175540_i128)];
_5 = [_4,_4,_4,_4,_4,_4];
_6 = [_4,_4,_4,_4,_4,_4];
_2 = !_10;
_3 = _6;
_1 = [(-59736216540177127476117943118929774844_i128),(-121192217622857804184962928091931670184_i128),(-69232797677401067595760424589139374576_i128),(-49558574082064230433939739414463694127_i128),(-79278566824865077060840772659607264358_i128)];
match _4 {
0 => bb4,
1 => bb6,
2 => bb7,
3 => bb8,
4 => bb9,
69 => bb11,
_ => bb10
}
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
Return()
}
bb19 = {
_11 = RET;
_15.0 = (-1739977859_i32) as u16;
_16.2 = [_4,_4,_4,_4,_4,_4];
_18 = 28797_i16 as u32;
_6 = _5;
RET = _11;
Goto(bb20)
}
bb20 = {
Call(_20 = dump_var(9_usize, 11_usize, Move(_11), 5_usize, Move(_5), 3_usize, Move(_3), 6_usize, Move(_6)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_20 = dump_var(9_usize, 12_usize, Move(_12), 14_usize, Move(_14), 4_usize, Move(_4), 21_usize, _21), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn10(mut _1: f32,mut _2: (f32, isize),mut _3: isize,mut _4: isize,mut _5: [isize; 6]) -> isize {
mir! {
type RET = isize;
let _6: [u8; 6];
let _7: (&'static [i128; 5],);
let _8: i8;
let _9: ((f64, &'static [i128; 5]), [isize; 4], ((f32, isize), *const [u32; 4], &'static (f32, isize)));
let _10: Adt57;
let _11: *mut (u16, u64, i128, f32);
let _12: ();
let _13: ();
{
_2 = (_1, _3);
_2.1 = _3;
_3 = -_4;
_5 = [_3,_2.1,_2.1,_2.1,_3,_4];
_2.0 = _1;
_4 = 993373967_u32 as isize;
_1 = -_2.0;
_2.0 = 58_u8 as f32;
_1 = -_2.0;
_1 = -_2.0;
_5 = [_2.1,_2.1,_2.1,_4,_2.1,_2.1];
_2 = (_1, _3);
Goto(bb1)
}
bb1 = {
_2.1 = _3 | _3;
_6 = [111_u8,56_u8,221_u8,24_u8,155_u8,210_u8];
RET = _2.1 | _2.1;
_2.1 = _3;
RET = !_3;
_1 = _2.0 - _2.0;
_5 = [_2.1,_3,_2.1,_2.1,_3,_2.1];
_3 = false as isize;
_8 = (-124_i8);
_2.0 = _1 - _1;
_1 = -_2.0;
RET = _2.1;
_3 = RET;
_2 = (_1, _3);
_9.0.0 = 30752_u16 as f64;
RET = _2.1;
_9.2.0 = (_1, RET);
_1 = _2.0 * _9.2.0.0;
_9.2.0 = _2;
_2.1 = _9.2.0.1 << _3;
_9.1 = [_3,RET,_2.1,_2.1];
_9.2.0 = _2;
_9.2.0 = (_2.0, _2.1);
Goto(bb2)
}
bb2 = {
_9.1 = [_9.2.0.1,_3,_9.2.0.1,_3];
RET = _9.2.0.1;
_9.2.0.1 = _3 - RET;
_2.0 = _1;
_9.0.0 = (-8405656967079745745_i64) as f64;
_9.2.0 = _2;
_10.fld1 = 5_usize & 192166693306349427_usize;
_2.1 = (-3256_i16) as isize;
_10.fld1 = 5_usize - 7656393320086261647_usize;
Goto(bb3)
}
bb3 = {
Call(_12 = dump_var(10_usize, 3_usize, Move(_3), 8_usize, Move(_8), 13_usize, _13, 13_usize, _13), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn11(mut _1: isize,mut _2: (u16, u64, i128, f32)) -> isize {
mir! {
type RET = isize;
let _3: char;
let _4: *mut (u16, u64, i128, f32);
let _5: isize;
let _6: (f32, isize);
let _7: (u8,);
let _8: u32;
let _9: u32;
let _10: isize;
let _11: (&'static [i16; 6], &'static (f32, isize), (u8,));
let _12: isize;
let _13: Adt69;
let _14: bool;
let _15: isize;
let _16: ();
let _17: ();
{
RET = _1 ^ _1;
_3 = '\u{2b4e6}';
RET = !_1;
_2.3 = (-2254744891992034906_i64) as f32;
_4 = core::ptr::addr_of_mut!(_2);
(*_4).3 = 1226572768_u32 as f32;
(*_4).2 = 158734768469677726342256292102327675724_i128 + (-100102594594917468276115161369448869229_i128);
(*_4).3 = RET as f32;
_2.2 = 62013907412215637367410706309060805574_i128 * (-169514965157716346513776442371772704139_i128);
(*_4).2 = (-42695428211711504478748199045126930856_i128) << (*_4).0;
_3 = '\u{a6db4}';
_3 = '\u{cb384}';
RET = _1;
_2.1 = !13243462559396656498_u64;
_5 = 4162987216_u32 as isize;
_2.2 = !(-106110135584450962336545614699507125433_i128);
(*_4).0 = !25404_u16;
_2.0 = 65248_u16;
_2.3 = 577722896_u32 as f32;
RET = _5;
_6 = ((*_4).3, _1);
_2.1 = 6220206837316713812_u64;
(*_4).1 = 12333458551827261365_u64 * 12568179868755676758_u64;
(*_4).0 = (-2239254710226886091_i64) as u16;
_2.2 = (-113013484636323608270809952577395204574_i128);
(*_4).0 = 17214_u16 - 2949_u16;
RET = _1;
match (*_4).2 {
0 => bb1,
1 => bb2,
2 => bb3,
227268882284614855192564654854373006882 => bb5,
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
(*_4).3 = _6.0 + _6.0;
(*_4).1 = 2524417952697651479_u64 + 15607706216905138722_u64;
(*_4).2 = (-21497034925902980231018842100906413356_i128);
(*_4).0 = (*_4).1 as u16;
(*_4).3 = _6.0 - _6.0;
(*_4).1 = (-4184260343166253463_i64) as u64;
_2.3 = -_6.0;
(*_4).0 = !880_u16;
_6.0 = (*_4).3;
_7 = (90_u8,);
_6.0 = -(*_4).3;
_7.0 = 219_u8;
(*_4) = (61977_u16, 15907592491569237396_u64, 161657380563436397219352676717161227841_i128, _6.0);
(*_4).3 = _6.0 - _6.0;
_6.0 = _2.3;
_3 = '\u{c87ae}';
(*_4).2 = 111684321563531701533280067263304451015_i128 * 24927920813152309832725743672490215427_i128;
(*_4).1 = 8887554243285570176_i64 as u64;
_2.0 = 47413_u16;
_9 = 3758963766_u32;
_8 = _7.0 as u32;
(*_4).3 = -_6.0;
_2 = (24655_u16, 362382223949573856_u64, (-31857800283796550935297597450621430402_i128), _6.0);
_2.3 = _6.0 * _6.0;
(*_4) = (11032_u16, 1082806508820559290_u64, 7808958785478755081698246762414595978_i128, _6.0);
RET = -_5;
match (*_4).2 {
0 => bb3,
1 => bb6,
2 => bb7,
7808958785478755081698246762414595978 => bb9,
_ => bb8
}
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
_1 = _6.1 >> _2.2;
_6 = ((*_4).3, _5);
(*_4).3 = -_6.0;
_10 = _6.1 | _5;
RET = _1 & _1;
_9 = _8;
(*_4).0 = _9 as u16;
_4 = core::ptr::addr_of_mut!((*_4));
_7 = (120_u8,);
_12 = RET;
_3 = '\u{b1c47}';
_5 = -_12;
_11.2 = (_7.0,);
(*_4) = (47552_u16, 8502196690591296236_u64, (-151025502083927744366066071761558808844_i128), _6.0);
(*_4).0 = 5864_u16 - 4752_u16;
_2.2 = -(-27999324880265175491280368583641134557_i128);
(*_4).3 = -_6.0;
(*_4).0 = 32770_u16 | 21900_u16;
_11.2.0 = _3 as u8;
_11.2 = (_7.0,);
(*_4).2 = (-151214020199835877105505737649147040346_i128) - 57939155449721577246374475424455583947_i128;
_5 = !_10;
_7 = _11.2;
_11.2 = _7;
(*_4).1 = 10180167799505551195_u64;
_11.1 = &_6;
Goto(bb10)
}
bb10 = {
Call(_16 = dump_var(11_usize, 3_usize, Move(_3), 10_usize, Move(_10), 8_usize, Move(_8), 7_usize, Move(_7)), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: &'static (f32, isize)) -> (f32, isize) {
mir! {
type RET = (f32, isize);
let _2: f64;
let _3: bool;
let _4: u128;
let _5: bool;
let _6: ((f64, &'static [i128; 5]), u128, (f64, &'static [i128; 5]));
let _7: u64;
let _8: ((&'static [i128; 5],),);
let _9: Adt73;
let _10: &'static [i128; 5];
let _11: *const [u32; 4];
let _12: isize;
let _13: usize;
let _14: [u8; 6];
let _15: char;
let _16: char;
let _17: i128;
let _18: [i16; 6];
let _19: [u8; 6];
let _20: usize;
let _21: char;
let _22: [u64; 2];
let _23: *const &'static &'static *const u64;
let _24: usize;
let _25: [i128; 5];
let _26: ();
let _27: ();
{
RET.0 = 11096336576395173920_u64 as f32;
RET.0 = 10150842747939867014_u64 as f32;
RET.0 = 7904613770648189009_u64 as f32;
_1 = &RET;
RET.0 = (-9223372036854775808_isize) as f32;
RET.1 = !(-9223372036854775808_isize);
RET.1 = 9223372036854775807_isize & 84_isize;
RET.0 = 248_u8 as f32;
_1 = &RET;
RET.1 = 59602736257034567707806605265748190849_i128 as isize;
_2 = 127_u8 as f64;
RET.0 = 137711503034522823634544966642524419772_i128 as f32;
Goto(bb1)
}
bb1 = {
RET.0 = RET.1 as f32;
_1 = &RET;
RET.1 = (*_1).0 as isize;
_2 = 1910_i16 as f64;
_3 = !false;
_3 = (*_1).0 == (*_1).0;
_1 = &RET;
Goto(bb2)
}
bb2 = {
RET.0 = (*_1).1 as f32;
RET.1 = 9223372036854775807_isize;
RET.0 = 80319867829948366114134962703445781830_u128 as f32;
_6.1 = 249204123701570213716866044394348196022_u128;
_5 = !_3;
_1 = &RET;
_6.0.0 = _2;
RET.0 = (-6947016983422181507_i64) as f32;
_6.0.0 = -_2;
_6.1 = 312847123806170897748094836206038890996_u128 ^ 301543818064051717437640355877721102708_u128;
_2 = _6.0.0 - _6.0.0;
_7 = !17786836490255035914_u64;
_5 = _3;
_2 = -_6.0.0;
_2 = _6.0.0 * _6.0.0;
_5 = _3;
_3 = _5;
RET.0 = _7 as f32;
_9.fld3.1 = [(-17773655490114351217906619209460918010_i128),89788905605743877285970259526877837234_i128,41544400242745309804193584746940922101_i128,87333985328526120914853482435799465551_i128,(-72488245251218059431967776245067225105_i128)];
RET.0 = RET.1 as f32;
_6.2.1 = &_9.fld3.1;
RET.1 = (-71_i8) as isize;
Goto(bb3)
}
bb3 = {
_6.2.0 = _6.0.0;
_9.fld1 = !_7;
_6.2.0 = _2 * _6.0.0;
_1 = &RET;
RET.0 = 22058_u16 as f32;
_2 = (-65_i8) as f64;
_1 = &RET;
_8.0.0 = &_9.fld3.1;
_9.fld1 = _7;
_12 = (*_1).1 - (*_1).1;
_9.fld3.0 = '\u{1b0b5}';
Goto(bb4)
}
bb4 = {
_6.0.1 = Move(_6.2.1);
_9.fld2 = !(-711809555_i32);
_6.0 = (_6.2.0, Move(_8.0.0));
_6.2.1 = &_9.fld3.1;
_6.0.0 = _6.2.0;
_8.0 = (Move(_6.2.1),);
_5 = _9.fld1 != _9.fld1;
RET.1 = _12;
_5 = !_3;
_6.2.1 = &_9.fld3.1;
_1 = &RET;
_4 = 8053686632717704786_usize as u128;
_9.fld3.0 = '\u{f0ff2}';
_9.fld2 = 445105015_i32 | 1531793444_i32;
RET.1 = _12 << _4;
_1 = &RET;
_8.0 = (Move(_6.2.1),);
_14 = [24_u8,61_u8,254_u8,215_u8,191_u8,80_u8];
_6.2.1 = &_9.fld3.1;
_8.0 = (Move(_6.2.1),);
_13 = 5608043617395718574_usize;
_5 = !_3;
_6.2.1 = &_9.fld3.1;
match _13 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5608043617395718574 => bb8,
_ => bb7
}
}
bb5 = {
_6.2.0 = _6.0.0;
_9.fld1 = !_7;
_6.2.0 = _2 * _6.0.0;
_1 = &RET;
RET.0 = 22058_u16 as f32;
_2 = (-65_i8) as f64;
_1 = &RET;
_8.0.0 = &_9.fld3.1;
_9.fld1 = _7;
_12 = (*_1).1 - (*_1).1;
_9.fld3.0 = '\u{1b0b5}';
Goto(bb4)
}
bb6 = {
RET.0 = (*_1).1 as f32;
RET.1 = 9223372036854775807_isize;
RET.0 = 80319867829948366114134962703445781830_u128 as f32;
_6.1 = 249204123701570213716866044394348196022_u128;
_5 = !_3;
_1 = &RET;
_6.0.0 = _2;
RET.0 = (-6947016983422181507_i64) as f32;
_6.0.0 = -_2;
_6.1 = 312847123806170897748094836206038890996_u128 ^ 301543818064051717437640355877721102708_u128;
_2 = _6.0.0 - _6.0.0;
_7 = !17786836490255035914_u64;
_5 = _3;
_2 = -_6.0.0;
_2 = _6.0.0 * _6.0.0;
_5 = _3;
_3 = _5;
RET.0 = _7 as f32;
_9.fld3.1 = [(-17773655490114351217906619209460918010_i128),89788905605743877285970259526877837234_i128,41544400242745309804193584746940922101_i128,87333985328526120914853482435799465551_i128,(-72488245251218059431967776245067225105_i128)];
RET.0 = RET.1 as f32;
_6.2.1 = &_9.fld3.1;
RET.1 = (-71_i8) as isize;
Goto(bb3)
}
bb7 = {
RET.0 = RET.1 as f32;
_1 = &RET;
RET.1 = (*_1).0 as isize;
_2 = 1910_i16 as f64;
_3 = !false;
_3 = (*_1).0 == (*_1).0;
_1 = &RET;
Goto(bb2)
}
bb8 = {
_6.2.0 = _6.0.0;
_10 = &_9.fld3.1;
_8.0.0 = &(*_10);
_7 = RET.1 as u64;
_15 = _9.fld3.0;
_17 = _6.2.0 as i128;
_6.0 = Move(_6.2);
_16 = _15;
_6.0.1 = &_9.fld3.1;
RET.0 = (-54_i8) as f32;
_3 = !_5;
_1 = &RET;
_6.1 = !_4;
_17 = !(-92838317208542695481479696010630135512_i128);
_15 = _9.fld3.0;
RET.0 = (-86_i8) as f32;
_6.2.1 = Move(_10);
_8.0.0 = Move(_6.0.1);
_9.fld2 = (-667785017_i32);
RET.1 = _12 | _12;
_16 = _15;
RET.1 = -_12;
match _9.fld2 {
0 => bb2,
1 => bb9,
2 => bb10,
340282366920938463463374607431100426439 => bb12,
_ => bb11
}
}
bb9 = {
RET.0 = RET.1 as f32;
_1 = &RET;
RET.1 = (*_1).0 as isize;
_2 = 1910_i16 as f64;
_3 = !false;
_3 = (*_1).0 == (*_1).0;
_1 = &RET;
Goto(bb2)
}
bb10 = {
_6.0.1 = Move(_6.2.1);
_9.fld2 = !(-711809555_i32);
_6.0 = (_6.2.0, Move(_8.0.0));
_6.2.1 = &_9.fld3.1;
_6.0.0 = _6.2.0;
_8.0 = (Move(_6.2.1),);
_5 = _9.fld1 != _9.fld1;
RET.1 = _12;
_5 = !_3;
_6.2.1 = &_9.fld3.1;
_1 = &RET;
_4 = 8053686632717704786_usize as u128;
_9.fld3.0 = '\u{f0ff2}';
_9.fld2 = 445105015_i32 | 1531793444_i32;
RET.1 = _12 << _4;
_1 = &RET;
_8.0 = (Move(_6.2.1),);
_14 = [24_u8,61_u8,254_u8,215_u8,191_u8,80_u8];
_6.2.1 = &_9.fld3.1;
_8.0 = (Move(_6.2.1),);
_13 = 5608043617395718574_usize;
_5 = !_3;
_6.2.1 = &_9.fld3.1;
match _13 {
0 => bb1,
1 => bb2,
2 => bb3,
3 => bb5,
4 => bb6,
5608043617395718574 => bb8,
_ => bb7
}
}
bb11 = {
_6.2.0 = _6.0.0;
_9.fld1 = !_7;
_6.2.0 = _2 * _6.0.0;
_1 = &RET;
RET.0 = 22058_u16 as f32;
_2 = (-65_i8) as f64;
_1 = &RET;
_8.0.0 = &_9.fld3.1;
_9.fld1 = _7;
_12 = (*_1).1 - (*_1).1;
_9.fld3.0 = '\u{1b0b5}';
Goto(bb4)
}
bb12 = {
_6.0.0 = _12 as f64;
_9.fld2 = 961416373_i32 | 1281576124_i32;
_6.2.1 = &_9.fld3.1;
_19 = [115_u8,201_u8,57_u8,67_u8,38_u8,36_u8];
RET.1 = _12;
_8.0.0 = &_9.fld3.1;
_3 = _6.1 > _4;
_6.2.1 = &_9.fld3.1;
_10 = &_9.fld3.1;
_9.fld3.1 = [_17,_17,_17,_17,_17];
_1 = &RET;
_6.1 = _13 as u128;
_6.1 = !_4;
_18 = [(-9702_i16),(-10187_i16),11030_i16,(-7563_i16),(-26939_i16),(-23776_i16)];
_6.0.0 = -_2;
_9.fld3.1 = [_17,_17,_17,_17,_17];
_20 = _13 ^ _13;
_21 = _16;
_17 = (-63730461050396484370983923122077898107_i128);
_9.fld2 = 1763579318_i32 | (-1656606036_i32);
_9.fld3.1 = [_17,_17,_17,_17,_17];
_1 = &(*_1);
_6.0.1 = &_9.fld3.1;
Goto(bb13)
}
bb13 = {
RET.0 = _7 as f32;
_6.0.1 = &_9.fld3.1;
_21 = _16;
_13 = !_20;
_22 = [_7,_7];
_6.2.0 = _17 as f64;
_6.0.1 = &_9.fld3.1;
Goto(bb14)
}
bb14 = {
_9.fld1 = _7;
RET.0 = _9.fld2 as f32;
_6.0.1 = &_9.fld3.1;
_1 = &RET;
_13 = _20;
_9.fld3.0 = _15;
_8.0.0 = &_9.fld3.1;
_20 = _13 - _13;
_6.2 = (_6.0.0, Move(_8.0.0));
_10 = &_9.fld3.1;
RET.1 = (-89_i8) as isize;
RET.0 = 42_u8 as f32;
_6.2 = (_6.0.0, Move(_10));
_3 = _20 <= _13;
_9.fld3.1 = [_17,_17,_17,_17,_17];
_2 = _6.0.0 + _6.0.0;
_8.0.0 = &_9.fld3.1;
_6.0 = (_2, Move(_8.0.0));
_7 = RET.1 as u64;
_21 = _15;
_24 = _13 >> _13;
_6.1 = !_4;
_1 = &RET;
Goto(bb15)
}
bb15 = {
Call(_26 = dump_var(12_usize, 17_usize, Move(_17), 12_usize, Move(_12), 21_usize, Move(_21), 18_usize, Move(_18)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_26 = dump_var(12_usize, 5_usize, Move(_5), 14_usize, Move(_14), 20_usize, Move(_20), 15_usize, Move(_15)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(false), std::hint::black_box('\u{b442e}'), std::hint::black_box((-9223372036854775808_isize)), std::hint::black_box(4521173232147069154_usize), std::hint::black_box(3162677805_u32), std::hint::black_box((-759268210_i32)), std::hint::black_box(7705713721025790039_i64), std::hint::black_box(6901405914895937207_u64));
                
            }
impl PrintFDebug for Adt26{
	unsafe fn printf_debug(&self){unsafe{printf("Adt26::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt26 {
Variant0{
fld0: (bool, i8, i16),

},
Variant1{
fld0: i32,
fld1: usize,

}}
impl PrintFDebug for Adt57{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt57{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt57 {
fld0: i32,
fld1: usize,
fld2: [isize; 6],
fld3: (usize,),
fld4: i16,
}
impl PrintFDebug for Adt59{
	unsafe fn printf_debug(&self){unsafe{printf("Adt59::\0".as_ptr()  as *const c_char)};match self{
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
#[derive(Copy,Clone)]pub enum Adt59 {
Variant0{
fld0: bool,
fld1: [u8; 6],
fld2: isize,
fld3: *mut (u16, u64, i128, f32),
fld4: i128,

},
Variant1{
fld0: u64,
fld1: [i16; 6],
fld2: (u128,),
fld3: u16,
fld4: (u16, u64, i128, f32),
fld5: i32,
fld6: i64,

},
Variant2{
fld0: f64,
fld1: char,
fld2: (usize,),
fld3: *mut (u16, u64, i128, f32),
fld4: i16,
fld5: [i16; 6],
fld6: *const (f32, isize),

},
Variant3{
fld0: Adt57,
fld1: i16,

}}
impl PrintFDebug for Adt69{
	unsafe fn printf_debug(&self){unsafe{printf("Adt69::\0".as_ptr()  as *const c_char)};match self{
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
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
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
},
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
},
		}
unsafe{printf("}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt69 {
Variant0{
fld0: bool,
fld1: *mut [u32; 4],
fld2: (f32, isize),
fld3: f64,
fld4: i16,
fld5: i32,
fld6: Adt26,
fld7: [u64; 2],

},
Variant1{
fld0: (usize,),
fld1: (i128,),
fld2: u16,
fld3: [u8; 6],
fld4: [u32; 4],
fld5: i32,

},
Variant2{
fld0: (u128,),

}}
impl PrintFDebug for Adt73{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt73{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt73 {
fld0: Adt59,
fld1: u64,
fld2: i32,
fld3: (char, [i128; 5]),
fld4: *const Adt57,
}
impl PrintFDebug for Adt75{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt75{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt75 {
fld0: (f32, isize),
fld1: [i16; 6],
fld2: (char, [i128; 5]),
fld3: *const (f32, isize),
}
impl PrintFDebug for Adt78{
	unsafe fn printf_debug(&self){unsafe{printf("Adt78::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt78 {
Variant0{
fld0: *const [u32; 4],

},
Variant1{
fld0: [u8; 3],

}}
impl PrintFDebug for Adt79{
	unsafe fn printf_debug(&self){unsafe{printf("Adt79::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{".as_ptr() as *const c_char)};
		unsafe{printf("fld0:".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",".as_ptr() as *const c_char)};
		unsafe{printf("fld1:".as_ptr() as *const c_char)};
		fld1.printf_debug();
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
#[derive(Copy,Clone)]pub enum Adt79 {
Variant0{
fld0: bool,
fld1: Adt69,

},
Variant1{
fld0: f32,
fld1: char,

},
Variant2{
fld0: *mut (i128,),
fld1: u32,
fld2: [char; 7],
fld3: u16,
fld4: (u16, u64, i128, f32),
fld5: usize,
fld6: [u32; 4],

},
Variant3{
fld0: f64,
fld1: char,
fld2: Adt57,
fld3: usize,
fld4: *mut (u16, u64, i128, f32),
fld5: [char; 7],
fld6: [u8; 6],

}}

